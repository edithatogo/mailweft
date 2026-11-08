use chrono::{TimeZone, Utc};
use mailweft_core::convert_eml;

const RAW: &[u8] = b"From: Alice <alice@example.invalid>\r\nTo: Bob <bob@example.invalid>\r\nCc: Claire <claire@example.invalid>\r\nBcc: Hidden <hidden@example.invalid>\r\nMessage-ID: <m1@example.invalid>\r\nSubject: Review\r\n\r\nPlease review.\r\n";

fn observed() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 9, 1, 2, 3).unwrap()
}

#[test]
fn emits_valid_role_qualified_ocel_shape_without_leaking_addresses() {
    let result = convert_eml(RAW, "synthetic-mailbox/1", observed()).unwrap();
    let doc = serde_json::to_value(&result.log).unwrap();
    assert!(doc.get("objectTypes").unwrap().as_array().unwrap().len() >= 2);
    assert_eq!(doc["events"].as_array().unwrap().len(), 1);
    let event = &doc["events"][0];
    assert_eq!(event["type"], "source_copy_observed");
    assert_eq!(event["time"], "2026-11-09T01:02:03Z");
    let roles: Vec<_> = event["relationships"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|x| x["qualifier"].as_str())
        .collect();
    for role in ["from", "to", "cc", "bcc", "source_copy", "message"] {
        assert!(roles.contains(&role), "missing {role}");
    }
    let output = serde_json::to_string(&result).unwrap();
    assert!(!output.contains("alice@example.invalid"));
    assert!(!output.contains("hidden@example.invalid"));
    assert!(!output.contains("synthetic-mailbox/1"));
    assert!(result.source_sha256.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn preserves_distinct_source_copies_without_claiming_message_delivery() {
    let first = convert_eml(RAW, "mailbox/1", observed()).unwrap();
    let second = convert_eml(RAW, "mailbox/2", observed()).unwrap();
    assert_eq!(first.message_candidate_id, second.message_candidate_id);
    assert_ne!(first.source_copy_id, second.source_copy_id);
    assert_ne!(first.log.events[0].id, second.log.events[0].id);
    assert_eq!(first.log.events[0].event_type, "source_copy_observed");
}

#[test]
fn conversion_is_deterministic_for_explicit_observation_time() {
    let a = convert_eml(RAW, "mailbox/1", observed()).unwrap();
    let b = convert_eml(RAW, "mailbox/1", observed()).unwrap();
    assert_eq!(
        serde_json::to_vec(&a).unwrap(),
        serde_json::to_vec(&b).unwrap()
    );
}

#[test]
fn rejects_empty_or_oversized_input() {
    assert!(convert_eml(&[], "mailbox/1", observed()).is_err());
    assert!(convert_eml(&vec![b'X'; 16 * 1024 * 1024 + 1], "mailbox/1", observed()).is_err());
}

#[test]
fn missing_bcc_is_not_fabricated() {
    let raw = b"From: a@example.invalid\nTo: b@example.invalid\n\nhello";
    let result = convert_eml(raw, "copy", observed()).unwrap();
    let roles: Vec<_> = result.log.events[0]
        .relationships
        .iter()
        .map(|r| r.qualifier.as_str())
        .collect();
    assert!(!roles.contains(&"bcc"));
}

#[test]
fn malformed_header_fails_closed_instead_of_synthesizing_recipients() {
    assert!(convert_eml(b"hello no header block", "copy", observed()).is_err());
}

#[test]
fn repeated_acquisition_at_new_time_is_a_distinct_observation() {
    let first = convert_eml(RAW, "mailbox/1", observed()).unwrap();
    let later = convert_eml(
        RAW,
        "mailbox/1",
        Utc.with_ymd_and_hms(2026, 11, 10, 1, 2, 3).unwrap(),
    )
    .unwrap();
    assert_eq!(first.source_copy_id, later.source_copy_id);
    assert_ne!(first.log.events[0].id, later.log.events[0].id);
}
