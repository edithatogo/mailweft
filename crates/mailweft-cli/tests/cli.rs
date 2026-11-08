use std::process::Command;

#[test]
fn missing_input_is_nonzero() {
    let output = Command::new(env!("CARGO_BIN_EXE_mailweft"))
        .arg("/definitely/not/present.eml")
        .arg("2026-11-09T01:02:03Z")
        .output().unwrap();
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("panic"));
}
