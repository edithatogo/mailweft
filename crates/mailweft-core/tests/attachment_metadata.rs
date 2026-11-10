use chrono::{TimeZone, Utc};
use mailweft_core::{SourceInput, convert_batch, convert_eml};

const MIME: &[u8] = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nSubject: Report\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=\"piece\"\r\n\r\n--piece\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nPlease review.\r\n--piece\r\nContent-Type: text/plain; name=\"report.txt\"\r\nContent-Disposition: attachment; filename=\"report.txt\"\r\nContent-Transfer-Encoding: base64\r\n\r\nSEVMTE8=\r\n--piece--\r\n";
fn at() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap()
}

#[test]
fn attachment_has_safe_metadata_and_no_filename_or_body_leak() {
    let c = convert_eml(MIME, "copy", at()).unwrap();
    let attachments: Vec<_> = c
        .log
        .objects
        .iter()
        .filter(|o| o.object_type == "mime_attachment")
        .collect();
    assert_eq!(attachments.len(), 1);
    let attrs = &attachments[0].attributes;
    assert!(
        attrs
            .iter()
            .any(|x| x.name == "declared_media_type" && x.value == "text/plain" && x.time == at())
    );
    assert!(
        attrs
            .iter()
            .any(|x| x.name == "decoded_byte_length" && x.value == serde_json::json!(5))
    );
    let name = attrs.iter().find(|x| x.name == "filename_sha256").unwrap();
    assert_eq!(name.value.as_str().unwrap().len(), 64);
    assert!(
        c.log.events[0]
            .relationships
            .iter()
            .any(|r| r.object_id == attachments[0].id && r.qualifier == "attachment_part")
    );
    assert!(
        c.log
            .objects
            .iter()
            .find(|o| o.id == c.message_candidate_id)
            .unwrap()
            .relationships
            .iter()
            .any(|r| r.object_id == attachments[0].id && r.qualifier == "contains_attachment_part")
    );
    let serialized = serde_json::to_string(&c.log).unwrap();
    assert!(!serialized.contains("report.txt"));
    assert!(!serialized.contains("SEVMTE8="));
    assert!(!serialized.contains("Please review"));
}
#[test]
fn repeated_source_observation_merges_attachment_attribute_history() {
    let next = Utc.with_ymd_and_hms(2026, 11, 10, 7, 0, 0).unwrap();
    let sources = [
        SourceInput {
            raw: MIME,
            source_id: "copy",
            observed_at: at(),
        },
        SourceInput {
            raw: MIME,
            source_id: "copy",
            observed_at: next,
        },
    ];
    let log = convert_batch(&sources).unwrap();
    assert_eq!(log.events.len(), 2);
    let attachment = log
        .objects
        .iter()
        .find(|o| o.object_type == "mime_attachment")
        .unwrap();
    let mut times: Vec<_> = attachment
        .attributes
        .iter()
        .filter(|a| a.name == "declared_media_type")
        .map(|a| a.time)
        .collect();
    times.sort();
    assert_eq!(times, vec![at(), next]);
}
#[test]
fn ordinary_body_without_attachment_does_not_create_fake_part() {
    let raw = b"From: a@example.invalid\r\nSubject: Plain\r\n\r\nHello world";
    let c = convert_eml(raw, "copy", at()).unwrap();
    assert!(
        !c.log
            .objects
            .iter()
            .any(|o| o.object_type == "mime_attachment")
    );
    assert!(
        !c.log.events[0]
            .relationships
            .iter()
            .any(|r| r.qualifier == "attachment_part")
    );
}
