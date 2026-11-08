//! Thin, offline command-line adapter over the Mailweft Rust core.
use chrono::{DateTime, Utc};
use mailweft_core::{SourceInput, convert_batch, convert_eml};
use std::{env, fs, process::ExitCode};

fn parse_instant(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn print_ocel(log: &mailweft_core::Ocel) -> ExitCode {
    match serde_json::to_string_pretty(log) {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(_) => {
            eprintln!("error: cannot serialize OCEL");
            ExitCode::FAILURE
        }
    }
}

fn batch(args: &[String]) -> ExitCode {
    if args.len() < 4 || args.len() > 1027 {
        eprintln!("Usage: mailweft batch <observed-at:RFC3339> <file.eml> [file2.eml ...]");
        return ExitCode::FAILURE;
    }
    let Some(at) = parse_instant(&args[2]) else {
        eprintln!("error: observed-at must be an RFC3339 instant");
        return ExitCode::FAILURE;
    };
    // The batch CLI deliberately bounds inputs and total bytes; a streaming
    // adapter is a separate implementation track for larger corpora.
    let mut total: u64 = 0;
    let mut contents = Vec::new();
    for path in &args[3..] {
        let Ok(meta) = fs::metadata(path) else {
            eprintln!("error: cannot read source metadata");
            return ExitCode::FAILURE;
        };
        total = total.saturating_add(meta.len());
        if total > 256 * 1024 * 1024 || meta.len() > 16 * 1024 * 1024 {
            eprintln!("error: batch exceeds its input resource limit");
            return ExitCode::FAILURE;
        }
        match fs::read(path) {
            Ok(bytes) => contents.push(bytes),
            Err(_) => {
                eprintln!("error: cannot read source file");
                return ExitCode::FAILURE;
            }
        }
    }
    let inputs: Vec<_> = contents.iter()
        .zip(args[3..].iter())
        .map(|(bytes, source_id)| SourceInput {
            raw: bytes.as_slice(),
            source_id: source_id.as_str(),
            observed_at: at,
        })
        .collect();
    match convert_batch(&inputs) {
        Ok(log) => print_ocel(&log),
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.get(1).is_some_and(|arg| arg == "batch") {
        return batch(&args);
    }
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: mailweft <raw.eml> <observed-at:RFC3339> [source-id]");
        return ExitCode::FAILURE;
    }
    let Some(observed_at) = parse_instant(&args[2]) else {
        eprintln!("error: observed-at must be an RFC3339 instant");
        return ExitCode::FAILURE;
    };
    let raw = match fs::read(&args[1]) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("error: cannot read source file");
            return ExitCode::FAILURE;
        }
    };
    let source_id = args.get(3).map(String::as_str).unwrap_or(args[1].as_str());
    match convert_eml(&raw, source_id, observed_at) {
        Ok(result) => print_ocel(&result.log),
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
