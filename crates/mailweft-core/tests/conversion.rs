use mailweft_core::{Config, Error, EventView, Permission, RawEmail, convert, OCEL};
use std::collections::BTreeSet;

fn config(view: EventView) -> Config { Config::new("test-scope".into(), [7; 32], view) }
fn input(raw: &str, source: &str) -> RawEmail {
    RawEmail { bytes: raw.as_bytes().to_vec(), source_ref: source.into(),
        observed_at: "2026-11-08T10:00:00+10:00".parse().unwrap(), permission: Permission::Synthetic }
}
fn mail() -> String {
    concat!("From: Alice <alice@example.test>\r\nSender: Relay <relay@example.test>\r\n",
        "To: Bob <bob@example.test>\r\nCc: Carol <carol@example.test>\r\n",
        "Bcc: Dana <dana@example.test>\r\nReply-To: desk@example.test\r\n",
        "Date: Mon, 02 Nov 2026 09:30:00 +1000\r\nMessage-ID: <one@example.test>\r\n",
        "Subject: PRIVATE_SUBJECT\r\n\r\nPRIVATE_BODY\r\n").into()
}
#[test]
fn header_roles_are_preserved_not_task_assignments() {
    let result = convert(&[input(&mail(), "box/a")], &config(EventView::Both)).unwrap();
    let roles: BTreeSet<_> = result.messages[0].addresses.iter().map(|p| p.role.as_str()).collect();
    assert_eq!(roles, BTreeSet::from(["from", "sender", "to", "cc", "bcc", "reply_to"]));
    assert!(result.ocel.events.iter().all(|e| !e.event_type.contains("assigned")));
    assert!(result.messages[0].source_received_at.is_none());
}
#[test]
fn dates_have_separate_evidence_states_and_offsets() {
    let result = convert(&[input(&mail(), "a")], &config(EventView::Both)).unwrap();
    assert_eq!(result.ocel.events.len(), 2);
    let observed = result.ocel.events.iter().find(|e| e.event_type == "message_archived_observed").unwrap();
    let asserted = result.ocel.events.iter().find(|e| e.event_type == "message_header_date_asserted").unwrap();
    assert_eq!(observed.time.to_rfc3339(), "2026-11-08T10:00:00+10:00");
    assert_eq!(asserted.time.to_rfc3339(), "2026-11-02T09:30:00+10:00");
    assert!(asserted.attributes.iter().any(|a| a.name == "evidence_state"));
}
#[test]
fn absent_bcc_does_not_mean_no_blind_recipients() {
    let raw = mail().replace("Bcc: Dana <dana@example.test>\r\n", "");
    let result = convert(&[input(&raw, "a")], &config(EventView::Observed)).unwrap();
    assert!(result.messages[0].addresses.iter().all(|p| p.role != "bcc"));
    assert!(result.messages[0].parse_warnings.iter().any(|s| s == "BCC_VISIBILITY_UNKNOWN"));
}
#[test]
fn missing_date_never_uses_ingestion_as_sent_time() {
    let raw = "From: a@example.test\r\n\r\nHello";
    let result = convert(&[input(raw, "a")], &config(EventView::HeaderAsserted)).unwrap();
    assert!(result.ocel.events.is_empty());
    assert!(result.messages[0].header_date.is_none());
}
#[test]
fn malformed_date_is_unknown() {
    let result = convert(&[input("Date: not-a-date\r\nFrom: a@example.test\r\n\r\nx", "a")], &config(EventView::Both)).unwrap();
    assert_eq!(result.ocel.events.len(), 1);
    assert!(result.messages[0].parse_warnings.iter().any(|s| s == "HEADER_DATE_INVALID_OR_AMBIGUOUS"));
}
#[test]
fn duplicated_singleton_date_is_not_silently_last_wins() {
    let raw = format!("Date: Tue, 03 Nov 2026 09:30:00 +1000\r\n{}", mail());
    let result = convert(&[input(&raw, "a")], &config(EventView::Both)).unwrap();
    assert!(result.messages[0].header_date.is_none());
    assert_eq!(result.ocel.events.len(), 1);
}
#[test]
fn source_copies_do_not_double_count_claimed_communication() {
    let result = convert(&[input(&mail(), "a"), input(&mail(), "b")], &config(EventView::Both)).unwrap();
    assert_eq!(result.messages.len(), 2);
    assert_ne!(result.messages[0].source_copy_id, result.messages[1].source_copy_id);
    assert_eq!(result.messages[0].message_candidate_id, result.messages[1].message_candidate_id);
    assert_eq!(result.ocel.events.len(), 3);
}
#[test]
fn message_id_collision_does_not_merge_different_bytes() {
    let altered = mail().replace("PRIVATE_BODY", "Different content");
    let result = convert(&[input(&mail(), "a"), input(&altered, "b")], &config(EventView::HeaderAsserted)).unwrap();
    assert_ne!(result.messages[0].message_candidate_id, result.messages[1].message_candidate_id);
    assert_eq!(result.ocel.events.len(), 2);
}
#[test]
fn replay_and_input_permutation_are_identical() {
    let a = convert(&[input(&mail(), "a"), input(&mail(), "b")], &config(EventView::Both)).unwrap();
    let b = convert(&[input(&mail(), "b"), input(&mail(), "a")], &config(EventView::Both)).unwrap();
    assert_eq!(serde_json::to_vec(&a).unwrap(), serde_json::to_vec(&b).unwrap());
}
#[test]
fn replayed_source_copy_is_counted_not_emitted_twice() {
    let result = convert(&[input(&mail(), "a"), input(&mail(), "a")], &config(EventView::Both)).unwrap();
    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.receipt.input_count, 2);
    assert_eq!(result.receipt.parsed_count, 2);
    assert_eq!(result.receipt.duplicate_copy_count, 1);
}
#[test]
fn changed_source_ref_content_fails_closed() {
    let changed = mail().replace("PRIVATE_BODY", "Changed");
    assert_eq!(convert(&[input(&mail(), "a"), input(&changed, "a")], &config(EventView::Both)).unwrap_err(), Error("SOURCE_CONFLICT"));
}
#[test]
fn raw_content_identity_and_secret_are_not_serialized() {
    let result = convert(&[input(&mail(), "/private/alice/mail")], &config(EventView::Both)).unwrap();
    let text = serde_json::to_string(&result).unwrap();
    for sensitive in ["alice@example.test", "Alice", "PRIVATE_SUBJECT", "PRIVATE_BODY", "/private/alice/mail"] { assert!(!text.contains(sensitive), "unexpected source content"); }
}
#[test]
fn scope_separates_party_ids() {
    let a = convert(&[input(&mail(), "a")], &config(EventView::Observed)).unwrap();
    let mut other = config(EventView::Observed); other.scope = "other".into();
    let b = convert(&[input(&mail(), "a")], &other).unwrap();
    assert_ne!(a.messages[0].addresses[0].party_id, b.messages[0].addresses[0].party_id);
}
#[test]
fn address_domain_case_is_normalized_but_local_case_preserved() {
    let result = convert(&[input("To: User@EXAMPLE.TEST, User@example.test, user@example.test\r\n\r\nx", "a")], &config(EventView::Observed)).unwrap();
    let a = &result.messages[0].addresses;
    assert_eq!(a.len(), 3);
    assert_eq!(a[0].party_id, a[1].party_id);
    assert_ne!(a[0].party_id, a[2].party_id);
}
#[test]
fn repeated_recipient_headers_keep_all_roles() {
    let result = convert(&[input("To: a@example.test\r\nTo: b@example.test\r\nCc: a@example.test\r\n\r\nx", "a")], &config(EventView::Observed)).unwrap();
    assert_eq!(result.messages[0].addresses.iter().filter(|a| a.role == "to").count(), 2);
    assert_eq!(result.messages[0].addresses.iter().filter(|a| a.role == "cc").count(), 1);
}
#[test]
fn resource_and_permission_failures_are_nonsensitive() {
    let mut c = config(EventView::Both); c.max_message_bytes = 4;
    assert_eq!(convert(&[input(&mail(), "a")], &c).unwrap_err(), Error("MESSAGE_TOO_LARGE"));
    let mut i = input(&mail(), "a"); i.permission = Permission::Unknown;
    assert_eq!(convert(&[i], &config(EventView::Observed)).unwrap_err(), Error("SOURCE_NOT_AUTHORISED"));
    assert_eq!(convert(&[input("not a message", "a")], &config(EventView::Observed)).unwrap_err(), Error("INVALID_MESSAGE"));
}
#[test]
fn empty_scope_is_rejected() {
    let mut c = config(EventView::Observed); c.scope.clear();
    assert_eq!(convert(&[input(&mail(), "a")], &c).unwrap_err(), Error("INVALID_SCOPE"));
}
#[test]
fn native_rust4pm_round_trip_preserves_whole_output() {
    let result = convert(&[input(&mail(), "a")], &config(EventView::Both)).unwrap();
    let json = serde_json::to_vec(&result.ocel).unwrap();
    let restored: OCEL = serde_json::from_slice(&json).unwrap();
    assert_eq!(restored, result.ocel);
    assert!(restored.objects.iter().any(|o| !o.relationships.is_empty()));
    assert!(restored.objects.iter().any(|o| !o.attributes.is_empty()));
    assert!(restored.events.iter().any(|e| !e.attributes.is_empty()));
}
#[test]
fn hundred_fictional_messages_need_no_enron_or_model() {
    let inputs: Vec<_> = (0..100).map(|i| input(&format!("From: person{i}@example.test\r\nMessage-ID: <{i}@example.test>\r\n\r\nfictional"), &format!("source/{i}"))).collect();
    let result = convert(&inputs, &config(EventView::Observed)).unwrap();
    assert_eq!(result.messages.len(), 100);
    assert_eq!(result.ocel.events.len(), 100);
    assert_eq!(result.receipt.input_count, 100);
    assert_eq!(result.receipt.rejected_count, 0);
    assert_eq!(result.receipt.completeness, "complete_for_admitted_scope");
}
