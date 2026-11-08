//! Local-only CLI. Source bodies, paths and keys are never diagnostics.
use clap::{Args, Parser, Subcommand, ValueEnum};
use mailweft_core::{Config, Error, EventView, Permission, RawEmail, convert};
use std::{fs::{self, File}, io::{self, Read, Write}, path::{Component, Path, PathBuf}};

#[derive(Parser)]
#[command(name = "mailweft", version, about = "Convert local raw email into evidence-linked OCEL 2.0")]
struct Cli { #[command(subcommand)] command: Command }
#[derive(Subcommand)]
enum Command { Convert(ConvertArgs) }
#[derive(Clone, Copy, ValueEnum)]
enum View { Observed, HeaderAsserted, Both }
#[derive(Args)]
struct ConvertArgs {
    /// Opaque dataset/scope identifier, not a person's name.
    #[arg(long)] scope: String,
    /// Explicit source observation timestamp (RFC 3339), not a sent/delivery date.
    #[arg(long)] observed_at: String,
    /// Chronology to emit; mixing clocks requires an explicit selection.
    #[arg(long, value_enum)] view: View,
    /// Private file containing exactly 32 random bytes. Do not commit it.
    #[arg(long)] key_file: PathBuf,
    /// Declare that all supplied messages are fictional test fixtures.
    #[arg(long, conflicts_with = "authorised_local")] synthetic: bool,
    /// Declare independently obtained permission for local processing.
    #[arg(long, conflicts_with = "synthetic")] authorised_local: bool,
    /// Emit only native OCEL JSON rather than OCEL plus evidence receipt.
    #[arg(long)] ocel_only: bool,
    /// Individual regular EML files; directory ingestion is a later increment.
    #[arg(required = true, num_args = 1..)] inputs: Vec<PathBuf>,
}

fn read_bounded(path: &Path, max: usize) -> Result<Vec<u8>, Error> {
    let mut prefix = PathBuf::new();
    for component in path.components() {
        if matches!(component, Component::ParentDir) { return Err(Error("PATH_TRAVERSAL")); }
        prefix.push(component.as_os_str());
        if matches!(component, Component::Normal(_)) && fs::symlink_metadata(&prefix)
            .map_err(|_| Error("INPUT_UNREADABLE"))?.file_type().is_symlink() {
            return Err(Error("SYMLINK_NOT_ALLOWED"));
        }
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| Error("INPUT_UNREADABLE"))?;
    if !metadata.is_file() { return Err(Error("REGULAR_FILE_REQUIRED")); }
    if metadata.len() > max as u64 { return Err(Error("INPUT_TOO_LARGE")); }
    let file = File::open(path).map_err(|_| Error("INPUT_UNREADABLE"))?;
    if !file.metadata().map_err(|_| Error("INPUT_UNREADABLE"))?.is_file() {
        return Err(Error("REGULAR_FILE_REQUIRED"));
    }
    let mut bytes = Vec::new();
    file.take((max as u64).saturating_add(1)).read_to_end(&mut bytes)
        .map_err(|_| Error("INPUT_UNREADABLE"))?;
    if bytes.len() > max { return Err(Error("INPUT_TOO_LARGE")); }
    Ok(bytes)
}

fn execute(args: ConvertArgs) -> Result<(), Error> {
    let permission = match (args.synthetic, args.authorised_local) {
        (true, false) => Permission::Synthetic,
        (false, true) => Permission::AuthorisedLocal,
        _ => return Err(Error("SOURCE_NOT_AUTHORISED")),
    };
    let observed_at = chrono::DateTime::parse_from_rfc3339(&args.observed_at)
        .map_err(|_| Error("INVALID_OBSERVED_AT"))?;
    let key: [u8; 32] = read_bounded(&args.key_file, 32)?.try_into()
        .map_err(|_| Error("INVALID_KEY_LENGTH"))?;
    let view = match args.view { View::Observed => EventView::Observed,
        View::HeaderAsserted => EventView::HeaderAsserted, View::Both => EventView::Both };
    let config = Config::new(args.scope, key, view);
    if args.inputs.len() > config.max_messages { return Err(Error("TOO_MANY_MESSAGES")); }
    let mut inputs = Vec::new();
    let mut total = 0usize;
    for path in args.inputs {
        let bytes = read_bounded(&path, config.max_message_bytes)?;
        total = total.checked_add(bytes.len()).ok_or(Error("BATCH_TOO_LARGE"))?;
        if total > config.max_total_bytes { return Err(Error("BATCH_TOO_LARGE")); }
        let canonical = fs::canonicalize(path).map_err(|_| Error("INPUT_UNREADABLE"))?;
        let source_ref = canonical.to_str().ok_or(Error("PATH_ENCODING_UNSUPPORTED"))?.to_owned();
        inputs.push(RawEmail { bytes, source_ref, observed_at, permission });
    }
    let result = convert(&inputs, &config)?;
    let mut encoded = if args.ocel_only { serde_json::to_vec(&result.ocel) }
        else { serde_json::to_vec(&result) }.map_err(|_| Error("SERIALIZATION_FAILED"))?;
    encoded.push(b'\n');
    io::stdout().lock().write_all(&encoded).map_err(|_| Error("OUTPUT_WRITE_FAILED"))
}

fn main() -> std::process::ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            if matches!(error.kind(), clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion) {
                let _ = error.print();
                return std::process::ExitCode::SUCCESS;
            }
            eprintln!("ARGUMENT_ERROR: use --help; supplied arguments are not echoed");
            return std::process::ExitCode::from(2);
        }
    };
    match cli.command {
        Command::Convert(args) => match execute(args) {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(error) => { eprintln!("{}", error); std::process::ExitCode::from(2) }
        }
    }
}
