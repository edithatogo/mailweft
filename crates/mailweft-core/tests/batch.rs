use chrono::{TimeZone, Utc};
use mailweft_core::{SourceInput, convert_batch};

const RAW: &[u8] = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nSubject: test\r\n\r\nhello";
fn at() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 9, 0, 0, 0).unwrap()
}

#[test]
fn two_custodian_copies_preserve_both_events_but_one_message() {
    let inputs = [
        SourceInput {raw: RAW, source_id: "custodian/a", observed_at: at()},
        SourceInput {raw: RAW, source_id: "custodian/b", observed_at: at()},
    ];
    let log = convert_batch(&inputs).unwrap();
    assert_eq!(log.events.len(), 2);
    assert_eq!(log.objects.iter().filter(|x| x.object_type == "email_message").count(), 1);
    assert_eq!(log.objects.iter().filter(|x| x.object_type == "source_copy").count(), 2);
    let message = log.objects.iter().find(|x| x.object_type == "email_message").unwrap();
    assert_eq!(message.relationships.iter().filter(|x| x.qualifier == "observed_as_copy").count(), 2);
}

#[test]
fn input_order_does_not_change_canonical_log() {
    let first = SourceInput {raw: RAW, source_id: "a", observed_at: at()};
    let second = SourceInput {raw: RAW, source_id: "b", observed_at: at()};
    let a = convert_batch(&[first.clone(), second.clone()]).unwrap();
    let b = convert_batch(&[second, first]).unwrap();
    assert_eq!(serde_json::to_vec(&a).unwrap(), serde_json::to_vec(&b).unwrap());
}

#[test]
fn duplicate_exact_observation_is_not_double_counted() {
    let source = SourceInput {raw: RAW, source_id: "a", observed_at: at()};
    let log = convert_batch(&[source.clone(), source]).unwrap();
    assert_eq!(log.events.len(), 1);
}

#[test]
fn empty_batch_is_rejected() {
    assert!(convert_batch(&[]).is_err());
}
