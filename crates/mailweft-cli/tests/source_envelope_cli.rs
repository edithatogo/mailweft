use serde_json::{Value, json};
use std::process::{Command, Output};

const RAW: &[u8] = b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Fixture\r\n\r\nHello\r\n";

fn envelope() -> Value {
    // Fixture SHA-256 independently computed with Python hashlib.
    json!({
        "schema_version": "1.0.0",
        "source_id": "private-custodian/fixture-17",
        "source_sha256": "2ba2c8f860bfc5f2b6d4c13d4cfb0d8eb234239e432880195d9e4579ef87951e",
        "byte_length": RAW.len(),
        "observed_at": "2026-11-09T07:00:00Z",
        "fidelity": "raw_rfc5322",
        "coverage_claim": "unknown",
        "producer": {"name": "fixture-producer", "version": "1.0"},
        "rights_ref": "private-rights-record",
        "scope_ref": "private-authorized-scope",
        "custodian_ref": "private-custodian",
        "locator_ref": "private-archive/item-17",
        "transformation_refs": []
    })
}

fn run(raw: &[u8], env: &Value) -> Output {
    let dir = tempfile::tempdir().unwrap();
    let mail = dir.path().join("source.eml");
    let meta = dir.path().join("envelope.json");
    std::fs::write(&mail, raw).unwrap();
    std::fs::write(&meta, serde_json::to_vec(env).unwrap()).unwrap();
    Command::new(env!("CARGO_BIN_EXE_mailweft"))
        .arg("convert-evidence").arg(mail).arg(meta).output().unwrap()
}

#[test]
fn verified_bytes_have_deterministic_receipt_without_private_locators() {
    let a = run(RAW, &envelope());
    assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stderr));
    let b = run(RAW, &envelope());
    assert_eq!(a.stdout, b.stdout);
    let doc: Value = serde_json::from_slice(&a.stdout).unwrap();
    assert_eq!(doc["receipt"]["schema_version"], "1.0.0");
    assert_eq!(doc["receipt"]["source_sha256"], envelope()["source_sha256"]);
    assert_eq!(doc["receipt"]["coverage_claim"], "unknown");
    assert_eq!(doc["receipt"]["assurance"], "bytes_matched_metadata_caller_asserted");
    assert_eq!(doc["receipt"]["source_copy_id"], doc["conversion"]["source_copy_id"]);
    assert_eq!(doc["receipt"]["observation_event_id"], doc["conversion"]["log"]["events"][0]["id"]);
    assert_eq!(doc["conversion"]["log"]["events"][0]["time"], "2026-11-09T07:00:00Z");
    let output = String::from_utf8(a.stdout).unwrap();
    for secret in ["private-custodian", "private-rights-record", "private-authorized-scope", "private-archive", "alice@example.invalid", "Hello"] {
        assert!(!output.contains(secret));
    }
}

#[test]
fn tampered_source_is_rejected_without_returning_partial_ocel() {
    let raw = RAW.iter().copied().chain(b"tampered".iter().copied()).collect::<Vec<_>>();
    let output = run(&raw, &envelope());
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("SOURCE_LENGTH_MISMATCH"));
    let mut env = envelope();
    env["source_sha256"] = json!("0".repeat(64));
    let output = run(RAW, &env);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("SOURCE_HASH_MISMATCH"));
}

#[test]
fn lower_fidelity_export_is_not_accepted_as_raw_mail() {
    for fidelity in ["rendered_html", "platform_export"] {
        let mut env = envelope();
        env["fidelity"] = json!(fidelity);
        let output = run(RAW, &env);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("SOURCE_FIDELITY_UNSUPPORTED"));
    }
}

#[test]
fn malformed_missing_or_unknown_metadata_fails_closed() {
    for field in ["observed_at", "rights_ref", "scope_ref", "producer"] {
        let mut env = envelope();
        env.as_object_mut().unwrap().remove(field);
        assert!(!run(RAW, &env).status.success());
    }
    let mut env = envelope();
    env["schema_version"] = json!("99.0.0");
    let out = run(RAW, &env);
    assert!(String::from_utf8_lossy(&out.stderr).contains("SOURCE_VERSION_UNSUPPORTED"));
    let mut env = envelope();
    env["scope_ref"] = json!("   ");
    assert!(!run(RAW, &env).status.success());
    let mut env = envelope();
    env["invented_delivery_time"] = json!("2026-11-09T07:00:00Z");
    assert!(!run(RAW, &env).status.success());
}

#[test]
fn normal_input_modes_reject_non_regular_files_without_sensitive_diagnostics() {
    let dir = tempfile::tempdir().unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_mailweft"))
        .arg("inspect").arg(dir.path()).output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("SOURCE_NOT_REGULAR"));
    assert!(!String::from_utf8_lossy(&out.stderr).contains(dir.path().to_str().unwrap()));
}
