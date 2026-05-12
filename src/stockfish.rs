use crate::ChessPosition;
use std::sync::{Mutex, OnceLock};
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
};
use tempfile::TempPath;
const STOCKFISH_BYTES: &[u8] = include_bytes!("../bin/stockfish");

pub struct Stockfish {
    process: Child,
    reader:  BufReader<std::process::ChildStdout>,
    _temp:   TempPath,
}

impl Default for Stockfish {
    fn default() -> Self {
        Self::new()
    }
}

impl Stockfish {
    pub fn new() -> Self {
        let mut temp_exe = tempfile::Builder::new().suffix(".exe").tempfile().expect("Failed to create temp file");
        let path = temp_exe.path().to_path_buf();

        temp_exe.write_all(STOCKFISH_BYTES).expect("Failed to write bytes");
        temp_exe.as_file().sync_all().expect("Failed to sync file");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = temp_exe.as_file().metadata().unwrap().permissions();
            perms.set_mode(0o755);
            temp_exe.as_file().set_permissions(perms).unwrap();
        }

        let temp_path = temp_exe.into_temp_path();

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

        let mut process = child.expect("Failed to start Stockfish after 100 retries");

        let stdout = process.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        Self { process, reader, _temp: temp_path }
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

    pub fn get_eval(&mut self, position: &ChessPosition) -> f32 {
        let stdin = self.process.stdin.as_mut().unwrap();

        writeln!(stdin, "position fen {}", position.to_fen()).unwrap();
        writeln!(stdin, "eval").unwrap();

        let mut line = String::new();
        let mut evaluation = 0.0;

        loop {
            line.clear();
            self.reader.read_line(&mut line).unwrap();

            if line.starts_with("Final evaluation") {
                // Split the line by whitespace: ["Final", "evaluation", "+0.36", "(white", "side)"]
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    evaluation = parts[2].parse::<f32>().unwrap_or(0.0);
                }
                break;
            }
        }
        evaluation * 100.0
    }
}

pub static STOCKFISH: OnceLock<Mutex<Stockfish>> = OnceLock::new();
