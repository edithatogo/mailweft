//! Explicit, local selection adapter. No recursive discovery, network acquisition or crawling.
use mailweft_core::corpus::{MAX_SELECTION_BYTES, MAX_SELECTION_ITEMS, SubmittedSource, reconcile_selection, valid_entry_id};
use mailweft_core::source::{MAX_ENVELOPE_BYTES, MAX_SOURCE_BYTES, SourceError, parse_envelope_json, read_bounded_file};
use serde::Deserialize;
use std::{collections::BTreeSet, path::{Path, PathBuf}};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectionRequest { schema_version: String, entries: Vec<SelectionEntry> }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectionEntry { entry_id: String, source_path: PathBuf, envelope_path: PathBuf }

pub fn run(args: &[String]) -> Result<(String, bool), &'static str> {
    if !(args.len() == 3 || (args.len() == 4 && args[3] == "--allow-partial")) {
        return Err("Usage: mailweft reconcile <selection.json> [--allow-partial]");
    }
    let path = Path::new(&args[2]);
    let bytes = read_bounded_file(path, 1024 * 1024).map_err(|_| "SELECTION_INVALID")?;
    let mut request: SelectionRequest = serde_json::from_slice(&bytes).map_err(|_| "SELECTION_INVALID")?;
    if request.schema_version != "1.0.0" || request.entries.is_empty() || request.entries.len() > MAX_SELECTION_ITEMS {
        return Err("SELECTION_INVALID");
    }
    let mut ids = BTreeSet::new();
    for entry in &request.entries {
        if !valid_entry_id(&entry.entry_id) || !ids.insert(entry.entry_id.clone())
            || entry.source_path.as_os_str().is_empty() || entry.envelope_path.as_os_str().is_empty()
        { return Err("SELECTION_INVALID"); }
    }
    // Sorting before I/O makes resource-limit exclusions independent of request order.
    request.entries.sort_by(|a, b| a.entry_id.cmp(&b.entry_id));
    let base = path.parent().unwrap_or(Path::new("."));
    let mut remaining = MAX_SELECTION_BYTES;
    let mut items = Vec::new();
    for entry in request.entries {
        let result = (|| {
            let meta = read_bounded_file(&base.join(&entry.envelope_path), MAX_ENVELOPE_BYTES)?;
            let envelope = parse_envelope_json(&meta)?;
            if remaining == 0 { return Err(SourceError::SizeLimit); }
            let raw = read_bounded_file(&base.join(&entry.source_path), remaining.min(MAX_SOURCE_BYTES))?;
            remaining -= raw.len();
            Ok((raw, envelope))
        })();
        items.push(SubmittedSource { entry_id: entry.entry_id, source: result });
    }
    let result = reconcile_selection(items, args.len() == 4).map_err(|error| match error {
        mailweft_core::corpus::SelectionError::Invalid => "SELECTION_INVALID",
        mailweft_core::corpus::SelectionError::SizeLimit => "SELECTION_SIZE_LIMIT",
        mailweft_core::corpus::SelectionError::ProjectionConflict => "SELECTION_PROJECTION_CONFLICT",
        mailweft_core::corpus::SelectionError::Serialization => "SELECTION_SERIALIZATION_FAILED",
    })?;
    let partial = result.manifest.rejected != 0;
    let json = serde_json::to_string_pretty(&result).map_err(|_| "OUTPUT_SERIALIZATION_FAILED")?;
    Ok((json, partial))
}
