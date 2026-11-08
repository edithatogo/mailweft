use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
fn command(key: &Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_mailweft"));
    cmd.args([
        "convert",
        "--scope",
        "fictional",
        "--observed-at",
        "2026-11-08T00:00:00Z",
        "--view",
        "observed",
        "--key-file",
    ])
    .arg(key);
    cmd
}
fn run(extra: &[&str]) -> Output {
    let dir = tempfile::tempdir().unwrap();
    let physical = dir.path().canonicalize().unwrap();
    let key = physical.join("key.bin");
    let mail = physical.join("one.eml");
    fs::write(&key, [7u8; 32]).unwrap();
    fs::write(
        &mail,
        b"From: alice@example.test\r\nTo: bob@example.test\r\n\r\nprivate body",
    )
    .unwrap();
    command(&key).args(extra).arg(mail).output().unwrap()
}
#[test]
fn cli_converts_only_with_explicit_permission() {
    let out = run(&["--synthetic"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["ocel"]["events"].as_array().unwrap().len(), 1);
    assert!(!String::from_utf8_lossy(&out.stdout).contains("private body"));
}
#[test]
fn cli_missing_permission_emits_no_log() {
    let out = run(&[]);
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
}
#[test]
fn cli_can_emit_standalone_ocel() {
    let out = run(&["--synthetic", "--ocel-only"]);
    assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(value.get("eventTypes").is_some());
    assert!(value.get("ocel").is_none());
}
#[test]
fn invalid_later_input_leaves_stdout_empty() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().canonicalize().unwrap();
    let key = p.join("key");
    let a = p.join("a.eml");
    let b = p.join("private-name.eml");
    fs::write(&key, [7u8; 32]).unwrap();
    fs::write(&a, "From: a@example.test\r\n\r\nx").unwrap();
    fs::write(&b, "not a message").unwrap();
    let out = command(&key)
        .arg("--synthetic")
        .arg(a)
        .arg(b)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("private-name"));
}
#[test]
fn parser_diagnostics_do_not_echo_sensitive_arguments() {
    let out = run(&["--private-secret-value"]);
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("private-secret-value"));
}
#[cfg(unix)]
#[test]
fn symlink_input_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().canonicalize().unwrap();
    let key = p.join("key");
    let source = p.join("a.eml");
    let link = p.join("link.eml");
    fs::write(&key, [7u8; 32]).unwrap();
    fs::write(&source, "From: a@example.test\r\n\r\nx").unwrap();
    std::os::unix::fs::symlink(source, &link).unwrap();
    let out = command(&key).arg("--synthetic").arg(link).output().unwrap();
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    assert!(String::from_utf8_lossy(&out.stderr).contains("SYMLINK_NOT_ALLOWED"));
}
