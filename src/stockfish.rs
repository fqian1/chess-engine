use crate::ChessPosition;
use std::sync::{Mutex, OnceLock};
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
};
use tempfile::NamedTempFile;
const STOCKFISH_BYTES: &[u8] = include_bytes!("../bin/stockfish");

pub struct Stockfish {
    process: Child,
    reader:  BufReader<std::process::ChildStdout>,
    _temp:   NamedTempFile,
}

impl Default for Stockfish {
    fn default() -> Self {
        Self::new()
    }
}

impl Stockfish {
    pub fn new() -> Self {
        let temp_exe = tempfile::Builder::new().suffix(".exe").tempfile().expect("Failed to create temp file");

        let path = temp_exe.path().to_path_buf();

        {
            let mut file = std::fs::File::create(&path).expect("Failed to create file");
            file.write_all(STOCKFISH_BYTES).expect("Failed to write bytes");

            file.sync_all().expect("Failed to sync file");

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let mut perms = file.metadata().unwrap().permissions();
                perms.set_mode(0o755);
                file.set_permissions(perms).unwrap();
            }
            drop(file);
        }

        let mut child = None;

        for attempt in 0..100 {
            match Command::new(&path).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
                Ok(c) => {
                    child = Some(c);
                    break;
                }
                Err(e) if e.raw_os_error() == Some(26) => {
                    std::thread::sleep(std::time::Duration::from_millis(1 << attempt));
                    continue;
                }
                Err(e) => panic!("Permanent failure starting Stockfish: {}", e),
            }
        }

        let mut process = child.expect("Failed to start Stockfish after 10 retries");

        let stdout = process.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        Self { process, reader, _temp: temp_exe }
    }

    pub fn with_global<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Stockfish) -> R,
    {
        let mut engine = Self::global().lock().expect("Lock poisoned");
        f(&mut engine)
    }

    fn global() -> &'static Mutex<Self> {
        STOCKFISH.get_or_init(|| Mutex::new(Self::new()))
    }

    pub fn get_eval(&mut self, position: &ChessPosition) -> i32 {
        let stdin = self.process.stdin.as_mut().unwrap();

        writeln!(stdin, "position fen {}", position.to_fen()).unwrap();
        writeln!(stdin, "go depth 12").unwrap();

        let mut line = String::new();
        let mut cp = 0;
        loop {
            line.clear();
            self.reader.read_line(&mut line).unwrap();

            if let Some(pos) = line.find("score cp ") {
                let part = &line[pos + 9..];
                cp = part.split_whitespace().next().unwrap_or("0").parse().unwrap_or(0);
            }

            if line.starts_with("bestmove") {
                break;
            }
        }
        cp
    }
}

pub static STOCKFISH: OnceLock<Mutex<Stockfish>> = OnceLock::new();
