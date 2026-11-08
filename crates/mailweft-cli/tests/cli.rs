use std::{fs, process::Command};
fn run(extra: &[&str]) -> std::process::Output {
    let dir = tempfile::tempdir().unwrap();
    let key = dir.path().join("key.bin");
    let mail = dir.path().join("one.eml");
    fs::write(&key, [7u8;32]).unwrap();
    fs::write(&mail, b"From: alice@example.test\r\nTo: bob@example.test\r\n\r\nprivate body").unwrap();
    Command::new(env!("CARGO_BIN_EXE_mailweft")).args(["convert", "--scope", "fictional", "--observed-at", "2026-11-08T00:00:00Z", "--view", "observed", "--key-file"]).arg(key).args(extra).arg(mail).output().unwrap()
}
#[test]
fn cli_converts_only_with_explicit_permission() {
    let out = run(&["--synthetic"]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["ocel"]["events"].as_array().unwrap().len(), 1);
    assert!(!String::from_utf8_lossy(&out.stdout).contains("private body"));
}
#[test]
fn cli_missing_permission_emits_no_log() {
    let out = run(&[]); assert!(!out.status.success()); assert!(out.stdout.is_empty());
}
#[test]
fn cli_can_emit_standalone_ocel() {
    let out = run(&["--synthetic", "--ocel-only"]); assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(value.get("eventTypes").is_some()); assert!(value.get("ocel").is_none());
}
