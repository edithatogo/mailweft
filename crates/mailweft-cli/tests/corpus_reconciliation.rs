use serde_json::{Value, json};
use std::{
    path::Path,
    process::{Command, Output},
};

const RAW: &[u8] =
    b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Fixture\r\n\r\nHello\r\n";
const ENVELOPE: &[u8] =
    include_bytes!("../../../fixtures/synthetic/contracts/source-envelope.json");

fn fixture(root: &Path) -> Value {
    std::fs::write(root.join("source.eml"), RAW).unwrap();
    std::fs::write(root.join("valid.json"), ENVELOPE).unwrap();
    let mut bad: Value = serde_json::from_slice(ENVELOPE).unwrap();
    bad["source_sha256"] = json!("0".repeat(64));
    std::fs::write(root.join("bad.json"), serde_json::to_vec(&bad).unwrap()).unwrap();
    json!({"schema_version":"1.0.0", "entries":[
        {"entry_id":"private-one", "source_path":root.join("source.eml"), "envelope_path":root.join("valid.json")},
        {"entry_id":"private-two", "source_path":root.join("source.eml"), "envelope_path":root.join("bad.json")},
        {"entry_id":"private-three", "source_path":root.join("missing.eml"), "envelope_path":root.join("valid.json")}
    ]})
}

fn run(root: &Path, request: &Value, partial: bool) -> Output {
    let path = root.join("selection.json");
    std::fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_mailweft"));
    cmd.arg("reconcile").arg(path);
    if partial {
        cmd.arg("--allow-partial");
    }
    cmd.output().unwrap()
}

#[test]
fn every_selected_item_has_an_outcome_and_partial_projection_is_explicit() {
    let dir = tempfile::tempdir().unwrap();
    let request = fixture(dir.path());
    let strict = run(dir.path(), &request, false);
    assert_eq!(strict.status.code(), Some(2));
    let doc: Value = serde_json::from_slice(&strict.stdout).unwrap();
    assert_eq!(doc["manifest"]["supplied"], 3);
    assert_eq!(doc["manifest"]["accepted"], 1);
    assert_eq!(doc["manifest"]["rejected"], 2);
    assert_eq!(doc["manifest"]["entries"].as_array().unwrap().len(), 3);
    assert_eq!(doc["manifest"]["projection_status"], "withheld");
    assert!(doc["log"].is_null());
    let partial = run(dir.path(), &request, true);
    assert_eq!(partial.status.code(), Some(2));
    let doc: Value = serde_json::from_slice(&partial.stdout).unwrap();
    assert_eq!(doc["manifest"]["projection_status"], "partial");
    assert_eq!(doc["log"]["events"].as_array().unwrap().len(), 1);
    assert_eq!(doc["manifest_sha256"].as_str().unwrap().len(), 64);
    let output = String::from_utf8_lossy(&partial.stdout);
    for private in [
        "private-one",
        "missing.eml",
        "source.eml",
        "alice@example.invalid",
        "Hello",
    ] {
        assert!(!output.contains(private));
    }
    let outcomes = doc["manifest"]["entries"].as_array().unwrap();
    assert!(
        outcomes
            .iter()
            .any(|e| e["diagnostic"] == "SOURCE_HASH_MISMATCH")
    );
    assert!(
        outcomes
            .iter()
            .any(|e| e["diagnostic"] == "SOURCE_READ_FAILED")
    );
}

#[test]
fn reordering_selection_does_not_change_manifest_or_ocel() {
    let dir = tempfile::tempdir().unwrap();
    let mut request = fixture(dir.path());
    let first = run(dir.path(), &request, true);
    request["entries"].as_array_mut().unwrap().reverse();
    let second = run(dir.path(), &request, true);
    assert_eq!(first.status.code(), Some(2));
    assert_eq!(first.stdout, second.stdout);
}

#[test]
fn repeated_observation_is_accepted_but_not_counted_as_two_events() {
    let dir = tempfile::tempdir().unwrap();
    let mut request = fixture(dir.path());
    let one = request["entries"][0].clone();
    let mut two = one.clone();
    two["entry_id"] = json!("other-selection-slot");
    request["entries"] = json!([one, two]);
    let output = run(dir.path(), &request, false);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(doc["manifest"]["accepted"], 2);
    assert_eq!(doc["manifest"]["unique_observations"], 1);
    assert_eq!(doc["manifest"]["projection_status"], "complete_selection");
    assert_eq!(doc["log"]["events"].as_array().unwrap().len(), 1);
    assert_eq!(
        doc["manifest"]["coverage_assurance"],
        "supplied_selection_only"
    );
}

#[test]
fn empty_or_duplicate_selection_keys_are_invalid_not_complete_corpora() {
    let dir = tempfile::tempdir().unwrap();
    let mut request = fixture(dir.path());
    let one = request["entries"][0].clone();
    request["entries"] = json!([one.clone(), one]);
    let duplicate = run(dir.path(), &request, false);
    assert_eq!(duplicate.status.code(), Some(1));
    assert!(duplicate.stdout.is_empty());
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("SELECTION_INVALID"));
    request["entries"] = json!([]);
    assert_eq!(run(dir.path(), &request, false).status.code(), Some(1));
}
