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
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 2);
    assert_eq!(
        value["objects"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x["type"] == "email_message")
            .count(),
        1
    );
}

#[test]
fn inspect_returns_source_mapped_zones_without_body_text() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("source.eml");
    std::fs::write(
        &file,
        b"From: source@example.invalid\r\nSubject: Check\r\n\r\nHi Bob,\r\nPlease review.\r\nKind regards,\r\nAlex\r\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mailweft"))
        .arg("inspect")
        .arg(&file)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["parts"].as_array().unwrap().len(), 1);
    assert!(value["parts"][0]["zones"].as_array().unwrap().len() >= 3);
    let published = String::from_utf8_lossy(&output.stdout);
    assert!(!published.contains("source@example.invalid"));
    assert!(!published.contains("Please review."));
    assert!(!published.contains("Hi Bob,"));
}
