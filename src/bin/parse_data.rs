use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use zstd::stream::Decoder;

#[derive(Deserialize)]
struct Record {
    fen:   String,
    evals: Vec<EvalEntry>,
}

#[derive(Deserialize)]
struct EvalEntry {
    pvs: Vec<Pv>,
}

#[derive(Deserialize)]
struct Pv {
    cp:   Option<i32>,
    mate: Option<i32>,
    line: String,
}

fn main() -> io::Result<()> {
    let file = File::open("lichess_db_eval.jsonl.zst")?;
    let decoder = Decoder::new(file)?;
    let reader = BufReader::new(decoder);

    let mut output = File::create("mate_evals.tsv")?;

    for line in reader.lines() {
        let line_str = line?;
        if let Ok(record) = serde_json::from_str::<Record>(&line_str)
            && let Some(first_eval) = record.evals.first()
            && let Some(first_pv) = first_eval.pvs.first()
            && let Some(mate_val) = first_pv.mate
            && mate_val.abs() < 3
        {
            writeln!(output, "{}\t{}", record.fen, mate_val)?;
        }
    }
    Ok(())
}
