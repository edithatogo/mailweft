use std::process::Command;

#[test]
fn missing_input_is_nonzero() {
    let output = Command::new(env!("CARGO_BIN_EXE_mailweft"))
        .arg("/definitely/not/present.eml")
        .arg("2026-11-09T01:02:03Z")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("panic"));
}

#[test]
fn batch_cli_writes_two_observation_events_as_ocel_json() {
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("a.eml");
    let second = dir.path().join("b.eml");
    let raw = b"From: a@example.invalid\r\nSubject: Sample\r\n\r\nHello";
    std::fs::write(&first, raw).unwrap();
    std::fs::write(&second, raw).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mailweft"))
        .arg("batch")
        .arg("2026-11-09T01:02:03Z")
        .arg(&first)
        .arg(&second)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 2);
    assert_eq!(value["objects"].as_array().unwrap().iter().filter(|x| x["type"] == "email_message").count(), 1);
}
