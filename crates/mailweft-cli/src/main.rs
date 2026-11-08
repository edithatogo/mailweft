//! Thin, offline command-line adapter over the Mailweft Rust core.
use chrono::{DateTime, Utc};
use mailweft_core::convert_eml;
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: mailweft <raw.eml> <observed-at:RFC3339> [source-id]");
        return ExitCode::FAILURE;
    }
    let observed_at = match DateTime::parse_from_rfc3339(&args[2]) {
        Ok(v) => v.with_timezone(&Utc),
        Err(_) => {
            eprintln!("error: observed-at must be an RFC3339 instant");
            return ExitCode::FAILURE;
        }
    };
    let raw = match fs::read(&args[1]) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("error: cannot read source file");
            return ExitCode::FAILURE;
        }
    };
    let source_id = args.get(3).map(String::as_str).unwrap_or(args[1].as_str());
    let result = match convert_eml(&raw, source_id, observed_at) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    match serde_json::to_string_pretty(&result.log) {
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
