//! Offline CLI. Source capture and archival preservation remain external responsibilities.
use chrono::{DateTime, Utc};
use mailweft_core::source::{
    MAX_ENVELOPE_BYTES, MAX_SOURCE_BYTES, convert_with_envelope, parse_envelope_json,
    read_bounded_file,
};
use mailweft_core::{SourceInput, convert_batch, convert_eml, inspect_body_zones};
use std::{env, io::{self, Write}, path::Path, process::ExitCode};

fn parse_instant(value: &str) -> Result<DateTime<Utc>, &'static str> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| "observed-at must be an RFC3339 instant")
}

fn read_file(path: &str, limit: usize) -> Result<Vec<u8>, &'static str> {
    read_bounded_file(Path::new(path), limit).map_err(|e| e.code())
}

fn run(args: &[String]) -> Result<String, &'static str> {
    match args.get(1).map(String::as_str) {
        Some("convert-evidence") => {
            if args.len() != 4 {
                return Err("Usage: mailweft convert-evidence <raw.eml> <source-envelope.json>");
            }
            let metadata = read_file(&args[3], MAX_ENVELOPE_BYTES)?;
            let envelope = parse_envelope_json(&metadata).map_err(|e| e.code())?;
            let raw = read_file(&args[2], MAX_SOURCE_BYTES)?;
            let result = convert_with_envelope(&raw, &envelope).map_err(|e| e.code())?;
            serde_json::to_string_pretty(&result).map_err(|_| "OUTPUT_SERIALIZATION_FAILED")
        }
        Some("inspect") => {
            if args.len() != 3 {
                return Err("Usage: mailweft inspect <raw.eml>");
            }
            let raw = read_file(&args[2], MAX_SOURCE_BYTES)?;
            let evidence = inspect_body_zones(&raw).map_err(|_| "SOURCE_INSPECTION_FAILED")?;
            serde_json::to_string_pretty(&evidence).map_err(|_| "OUTPUT_SERIALIZATION_FAILED")
        }
        Some("batch") => {
            if args.len() < 4 || args.len() > 1027 {
                return Err("Usage: mailweft batch <observed-at:RFC3339> <file.eml> [file2.eml ...]");
            }
            let at = parse_instant(&args[2])?;
            let mut remaining = 256 * 1024 * 1024usize;
            let mut contents = Vec::new();
            for path in &args[3..] {
                let bytes = read_file(path, MAX_SOURCE_BYTES.min(remaining))?;
                remaining -= bytes.len();
                contents.push(bytes);
            }
            let inputs: Vec<_> = contents.iter().zip(&args[3..]).map(|(bytes, source_id)| {
                SourceInput { raw: bytes, source_id, observed_at: at }
            }).collect();
            let log = convert_batch(&inputs).map_err(|_| "SOURCE_CONVERSION_FAILED")?;
            serde_json::to_string_pretty(&log).map_err(|_| "OUTPUT_SERIALIZATION_FAILED")
        }
        _ => {
            if args.len() < 3 || args.len() > 4 {
                return Err("Usage: mailweft <raw.eml> <observed-at:RFC3339> [source-id]");
            }
            let at = parse_instant(&args[2])?;
            let raw = read_file(&args[1], MAX_SOURCE_BYTES)?;
            let source_id = args.get(3).unwrap_or(&args[1]);
            let conversion = convert_eml(&raw, source_id, at)
                .map_err(|_| "SOURCE_CONVERSION_FAILED")?;
            serde_json::to_string_pretty(&conversion.log).map_err(|_| "OUTPUT_SERIALIZATION_FAILED")
        }
    }
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    match run(&args) {
        Ok(json) => {
            let mut output = io::stdout().lock();
            if output.write_all(json.as_bytes()).and_then(|_| output.write_all(b"\n")).is_err() {
                eprintln!("error: OUTPUT_WRITE_FAILED");
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        Err(code) => {
            eprintln!("error: {code}");
            ExitCode::FAILURE
        }
    }
}
