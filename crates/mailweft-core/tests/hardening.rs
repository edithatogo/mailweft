use mailweft_core::{Config, Error, EventView, Permission, RawEmail, convert};
use std::collections::BTreeSet;

fn config() -> Config { Config::new("fixture".into(), [7;32], EventView::Both) }
fn input(raw: &str) -> RawEmail { RawEmail { bytes: raw.as_bytes().to_vec(), source_ref:"fixture/one".into(), observed_at:"2026-11-08T00:00:00Z".parse().unwrap(), permission:Permission::Synthetic } }

#[test]
fn missing_timezone_is_not_fabricated_utc() {
    let r = convert(&[input("Date: Mon, 02 Nov 2026 09:30:00\r\nFrom: a@example.test\r\n\r\nx")], &config()).unwrap();
    assert!(r.messages[0].header_date.is_none());
    assert_eq!(r.ocel.events.len(), 1);
}
#[test]
fn unknown_timezone_offset_is_not_an_exact_instant() {
    let r = convert(&[input("Date: Mon, 02 Nov 2026 09:30:00 -0000\r\nFrom: a@example.test\r\n\r\nx")], &config()).unwrap();
    assert!(r.messages[0].header_date.is_none());
}
#[test]
fn conflicting_offset_representations_of_one_source_fail_closed() {
    let a = input("From: a@example.test\r\n\r\nx");
    let mut b = input("From: a@example.test\r\n\r\nx");
    b.observed_at = "2026-11-08T10:00:00+10:00".parse().unwrap();
    assert_eq!(convert(&[a,b], &config()).unwrap_err(), Error("SOURCE_CONFLICT"));
}
#[test]
fn quoted_headers_do_not_become_current_recipients() {
    let r = convert(&[input("From: outer@example.test\r\nTo: one@example.test\r\n\r\nFrom: other@example.test\r\nTo: two@example.test")], &config()).unwrap();
    assert_eq!(r.messages[0].addresses.len(), 2);
}
#[test]
fn html_remote_resources_are_not_fetched_or_emitted() {
    let r = convert(&[input("From: a@example.test\r\nContent-Type: text/html\r\n\r\n<img src='https://not-accessed.invalid/private'>")], &config()).unwrap();
    assert!(!serde_json::to_string(&r).unwrap().contains("not-accessed"));
}
#[test]
fn unsupported_group_and_thread_semantics_mark_partial_coverage() {
    let r = convert(&[input("To: Team: a@example.test, b@example.test;\r\nReferences: <earlier@example.test>\r\n\r\nx")], &config()).unwrap();
    assert_eq!(r.receipt.completeness, "partial");
    assert!(r.messages[0].thread_evidence.is_empty());
}
#[test]
fn every_emitted_relation_resolves_to_a_declared_object() {
    let r = convert(&[input("From: a@example.test\r\nTo: b@example.test\r\n\r\nx")], &config()).unwrap();
    let ids: BTreeSet<_> = r.ocel.objects.iter().map(|o| o.id.as_str()).collect();
    assert_eq!(ids.len(), r.ocel.objects.len());
    for e in &r.ocel.events { for link in &e.relationships { assert!(ids.contains(link.object_id.as_str())); } }
    for o in &r.ocel.objects { for link in &o.relationships { assert!(ids.contains(link.object_id.as_str())); } }
}
#[test]
fn zero_limits_and_empty_inputs_are_rejected() {
    assert_eq!(convert(&[], &config()).unwrap_err(), Error("EMPTY_INPUT"));
    let mut c = config(); c.max_parts = 0;
    assert_eq!(convert(&[input("From: a@example.test\r\n\r\nx")], &c).unwrap_err(), Error("INVALID_LIMIT"));
}
#[test]
fn deterministic_malformed_byte_cases_do_not_panic() {
    for seed in 0..256u32 {
        let mut state = seed;
        let bytes: Vec<u8> = (0..128).map(|_| { state = state.wrapping_mul(1664525).wrapping_add(1013904223); (state >> 24) as u8 }).collect();
        let mut raw = input(""); raw.bytes = bytes;
        let _ = convert(&[raw], &config());
    }
}
