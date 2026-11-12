use chrono::{TimeZone, Utc};
use mailweft_core::convert_eml;

fn observed() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap()
}
const RAW: &[u8] = b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nDate: Sun, 08 Nov 2026 20:30:00 -0500\r\nMessage-ID: <a1@example.invalid>\r\nIn-Reply-To: <root@example.invalid>\r\nReferences: <first@example.invalid> <root@example.invalid>\r\nSubject: Re: Review\r\n\r\nI will review.\r\n";

#[test]
fn claimed_date_is_not_the_observed_event_time() {
    let result = convert_eml(RAW, "custodian/1", observed()).unwrap();
    assert_eq!(result.log.events[0].time, observed());
    let attrs = &result.log.events[0].attributes;
    assert!(
        attrs
            .iter()
            .any(|a| a.name == "claimed_date_utc" && a.value == "2026-11-09T01:30:00+00:00")
    );
    assert!(
        attrs
            .iter()
            .any(|a| a.name == "date_claim_status" && a.value == "parsed")
    );
}

#[test]
fn reference_claims_are_linked_without_observing_nonexistent_messages() {
    let result = convert_eml(RAW, "custodian/1", observed()).unwrap();
    let log = &result.log;
    let msg_ids: Vec<_> = log
        .objects
        .iter()
        .filter(|o| o.object_type == "claimed_message_identifier")
        .collect();
    assert_eq!(msg_ids.len(), 3);
    let names: Vec<_> = log.events[0]
        .relationships
        .iter()
        .map(|r| r.qualifier.as_str())
        .collect();
    for qualifier in [
        "claimed_message_id",
        "claimed_in_reply_to",
        "claimed_reference",
    ] {
        assert!(names.contains(&qualifier), "missing {qualifier}");
    }
    assert_eq!(
        names.iter().filter(|q| **q == "claimed_reference").count(),
        2
    );
    let public = serde_json::to_string(&log).unwrap();
    assert!(!public.contains("root@example.invalid"));
    assert!(!public.contains("first@example.invalid"));
    assert!(!public.contains("a1@example.invalid"));
    assert!(
        log.objects
            .iter()
            .all(|o| o.object_type != "email_message" || o.id == result.message_candidate_id)
    );
}

#[test]
fn missing_claim_headers_remain_absent_not_synthetic() {
    let raw = b"From: a@example.invalid\r\nSubject: hi\r\n\r\nHello";
    let result = convert_eml(raw, "copy", observed()).unwrap();
    assert!(
        !result.log.events[0]
            .attributes
            .iter()
            .any(|a| a.name == "claimed_date_utc")
    );
    assert!(
        result.log.events[0]
            .attributes
            .iter()
            .any(|a| a.name == "date_claim_status" && a.value == "absent")
    );
    assert!(
        !result.log.events[0]
            .relationships
            .iter()
            .any(|r| r.qualifier.starts_with("claimed_"))
    );
}

#[test]
fn malformed_or_duplicate_date_never_creates_an_authoritative_timestamp() {
    for raw in [
        b"From: a@example.invalid\r\nDate: not-a-date\r\n\r\nHello".as_slice(),
        b"From: a@example.invalid\r\nDate: Mon, 09 Nov 2026 00:00:00 +0000\r\nDate: Tue, 10 Nov 2026 00:00:00 +0000\r\n\r\nHello".as_slice(),
    ] {
        let result = convert_eml(raw, "copy", observed()).unwrap();
        assert_eq!(result.log.events[0].time, observed());
        assert!(!result.log.events[0].attributes.iter().any(|a| a.name == "claimed_date_utc"));
        assert!(result.log.events[0].attributes.iter().any(|a| a.name == "date_claim_status" && a.value != "parsed"));
    }
}

#[test]
fn same_message_id_is_a_claim_not_deduplicated_source() {
    let a =
        b"From: a@example.invalid\r\nMessage-ID: <same@example.invalid>\r\nSubject: one\r\n\r\nOne";
    let b =
        b"From: a@example.invalid\r\nMessage-ID: <same@example.invalid>\r\nSubject: two\r\n\r\nTwo";
    let first = convert_eml(a, "a", observed()).unwrap();
    let second = convert_eml(b, "b", observed()).unwrap();
    assert_ne!(first.message_candidate_id, second.message_candidate_id);
    let f = first.log.events[0]
        .relationships
        .iter()
        .find(|r| r.qualifier == "claimed_message_id")
        .unwrap();
    let s = second.log.events[0]
        .relationships
        .iter()
        .find(|r| r.qualifier == "claimed_message_id")
        .unwrap();
    assert_eq!(f.object_id, s.object_id);
}

#[test]
fn malformed_message_id_does_not_establish_linkage() {
    let raw = b"From: a@example.invalid\r\nMessage-ID: definitely-not-a-valid-id\r\n\r\nHi";
    let result = convert_eml(raw, "copy", observed()).unwrap();
    assert!(
        !result.log.events[0]
            .relationships
            .iter()
            .any(|r| r.qualifier == "claimed_message_id")
    );
}
