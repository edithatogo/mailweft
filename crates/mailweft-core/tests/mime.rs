use mailweft_core::{Config, EventView, Permission, RawEmail, convert, Error};
fn mime() -> String { concat!("From: a@example.test\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=x\r\n\r\n",
"--x\r\nContent-Type: text/plain\r\n\r\nbody\r\n",
"--x\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=private.bin\r\nContent-Transfer-Encoding: base64\r\n\r\nYWJj\r\n--x--\r\n").into() }
fn cfg() -> Config { Config::new("fixture".into(), [7;32], EventView::Observed) }
fn input(text: String) -> RawEmail { RawEmail { bytes: text.into_bytes(), source_ref: "one".into(), observed_at:"2026-11-08T00:00:00Z".parse().unwrap(), permission:Permission::Synthetic } }
#[test]
fn attachment_metadata_is_bounded_and_content_not_emitted() {
    let r = convert(&[input(mime())], &cfg()).unwrap();
    assert_eq!(r.messages[0].attachments.len(), 1);
    assert_eq!(r.messages[0].attachments[0].size_bytes, 3);
    let text = serde_json::to_string(&r).unwrap();
    assert!(!text.contains("private.bin")); assert!(!text.contains("YWJj"));
    assert!(r.ocel.objects.iter().any(|o| o.object_type == "Attachment"));
}
#[test]
fn mime_part_limit_rejects_instead_of_truncating() {
    let mut c = cfg(); c.max_parts = 1;
    assert_eq!(convert(&[input(mime())], &c).unwrap_err(), Error("MIME_LIMIT_REACHED"));
}
#[test]
fn nested_email_is_not_silently_counted_as_current_authorship() {
    let raw = "From: outer@example.test\r\nContent-Type: message/rfc822\r\n\r\nFrom: inner@example.test\r\n\r\nhello";
    let r = convert(&[input(raw.into())], &cfg()).unwrap();
    assert_eq!(r.messages[0].addresses.len(), 1);
    assert!(r.messages[0].parse_warnings.iter().any(|w| w == "NESTED_MESSAGE_NOT_PROJECTED"));
}
