//! Offline CLI. Source capture and archival preservation remain external responsibilities.
mod reconcile;

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand, error::ErrorKind};
use mailweft_core::source::{
    MAX_ENVELOPE_BYTES, MAX_SOURCE_BYTES, convert_with_envelope, parse_envelope_json,
    read_bounded_file,
};
use mailweft_core::{SourceInput, convert_batch, convert_eml, inspect_body_zones};
use std::{
    env,
    io::{self, Write},
    path::Path,
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    name = "mailweft",
    version,
    about = "Offline, bounded EML-to-OCEL 2.0 evidence conversion"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Convert an EML and a separately verified source envelope.
    ConvertEvidence { raw: String, envelope: String },
    /// Inspect source-mapped email zones without disclosing body text.
    Inspect { raw: String },
    /// Convert a bounded batch of EML files into one event log.
    Batch {
        observed_at: String,
        #[arg(required = true, num_args = 1..=1024)]
        files: Vec<String>,
    },
    /// Reconcile a declared local selection against source envelopes.
    Reconcile {
        selection: String,
        #[arg(long)]
        allow_partial: bool,
    },
    // Preserve the original positional mode: <raw.eml> <observed-at> [source-id].
    #[command(external_subcommand)]
    Legacy(Vec<String>),
}

fn parse_instant(value: &str) -> Result<DateTime<Utc>, &'static str> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| "observed-at must be an RFC3339 instant")
}

fn read_file(path: &str, limit: usize) -> Result<Vec<u8>, &'static str> {
    read_bounded_file(Path::new(path), limit).map_err(|e| e.code())
}

fn run(command: CliCommand) -> Result<(String, bool), &'static str> {
    match command {
        CliCommand::ConvertEvidence { raw, envelope } => {
            let metadata = read_file(&envelope, MAX_ENVELOPE_BYTES)?;
            let envelope = parse_envelope_json(&metadata).map_err(|e| e.code())?;
            let raw = read_file(&raw, MAX_SOURCE_BYTES)?;
            let result = convert_with_envelope(&raw, &envelope).map_err(|e| e.code())?;
            let json = serde_json::to_string_pretty(&result)
                .map_err(|_| "OUTPUT_SERIALIZATION_FAILED")?;
            Ok((json, false))
        }
        CliCommand::Inspect { raw } => {
            let raw = read_file(&raw, MAX_SOURCE_BYTES)?;
            let evidence = inspect_body_zones(&raw).map_err(|_| "SOURCE_INSPECTION_FAILED")?;
            let json = serde_json::to_string_pretty(&evidence)
                .map_err(|_| "OUTPUT_SERIALIZATION_FAILED")?;
            Ok((json, false))
        }
        CliCommand::Batch { observed_at, files } => {
            if files.is_empty() || files.len() > 1024 {
                return Err("Usage: mailweft batch <observed-at:RFC3339> <file.eml> [file2.eml ...]");
            }
            let at = parse_instant(&observed_at)?;
            let mut remaining = 256 * 1024 * 1024usize;
            let mut contents = Vec::with_capacity(files.len());
            for path in &files {
                let bytes = read_file(path, MAX_SOURCE_BYTES.min(remaining))?;
                remaining -= bytes.len();
                contents.push(bytes);
            }
            let inputs: Vec<_> = contents
                .iter()
                .zip(&files)
                .map(|(bytes, source_id)| SourceInput {
                    raw: bytes,
                    source_id,
                    observed_at: at,
                })
                .collect();
            let log = convert_batch(&inputs).map_err(|_| "SOURCE_CONVERSION_FAILED")?;
            let json = serde_json::to_string_pretty(&log)
                .map_err(|_| "OUTPUT_SERIALIZATION_FAILED")?;
            Ok((json, false))
        }
        CliCommand::Reconcile {
            selection,
            allow_partial,
        } => {
            // Preserve existing reconciliation contract/exit codes and source gates.
            let mut args = vec![String::from("mailweft"), String::from("reconcile"), selection];
            if allow_partial {
                args.push(String::from("--allow-partial"));
            }
            reconcile::run(&args)
        }
        CliCommand::Legacy(arguments) => {
            if arguments.len() < 2 || arguments.len() > 3 {
                return Err("Usage: mailweft <raw.eml> <observed-at:RFC3339> [source-id]");
            }
            let at = parse_instant(&arguments[1])?;
            let raw = read_file(&arguments[0], MAX_SOURCE_BYTES)?;
            let source_id = arguments.get(2).unwrap_or(&arguments[0]);
            let conversion =
                convert_eml(&raw, source_id, at).map_err(|_| "SOURCE_CONVERSION_FAILED")?;
            let json = serde_json::to_string_pretty(&conversion.log)
                .map_err(|_| "OUTPUT_SERIALIZATION_FAILED")?;
            Ok((json, false))
        }
    }
}

fn usage_error(args: &[String]) -> &'static str {
    match args.get(1).map(String::as_str) {
        Some("convert-evidence") => "Usage: mailweft convert-evidence <raw.eml> <source-envelope.json>",
        Some("inspect") => "Usage: mailweft inspect <raw.eml>",
        Some("batch") => "Usage: mailweft batch <observed-at:RFC3339> <file.eml> [file2.eml ...]",
        Some("reconcile") => "Usage: mailweft reconcile <selection.json> [--allow-partial]",
        _ => "Usage: mailweft <raw.eml> <observed-at:RFC3339> [source-id]",
    }
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    let command = match Cli::try_parse_from(&args) {
        Ok(Cli { command: Some(command) }) => command,
        Ok(Cli { command: None }) => {
            eprintln!("error: {}", usage_error(&args));
            return ExitCode::FAILURE;
        }
        Err(error) => {
            if matches!(error.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
                return if error.print().is_ok() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                };
            }
            eprintln!("error: {}", usage_error(&args));
            return ExitCode::FAILURE;
        }
    };
    match run(command) {
        Ok((json, partial)) => {
            let mut output = io::stdout().lock();
            if output
                .write_all(json.as_bytes())
                .and_then(|_| output.write_all(b"\n"))
                .is_err()
            {
                eprintln!("error: OUTPUT_WRITE_FAILED");
                return ExitCode::FAILURE;
            }
            ExitCode::from(if partial { 2 } else { 0 })
        }
        Err(code) => {
            eprintln!("error: {code}");
            ExitCode::FAILURE
        }
    }
}
