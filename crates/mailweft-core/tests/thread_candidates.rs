use chrono::{TimeZone, Utc};
use mailweft_core::{SourceInput, convert_batch};

const ROOT: &[u8] =
    b"From: a@example.invalid\r\nMessage-ID: <root@example.invalid>\r\nSubject: Base\r\n\r\nBase";
const CHILD: &[u8] = b"From: b@example.invalid\r\nMessage-ID: <child@example.invalid>\r\nIn-Reply-To: <root@example.invalid>\r\nReferences: <root@example.invalid>\r\nSubject: Re: Base\r\n\r\nReply";
fn at() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap()
}
fn source<'a>(raw: &'a [u8], id: &'a str) -> SourceInput<'a> {
    SourceInput {
        raw,
        source_id: id,
        observed_at: at(),
    }
}
#[test]
fn unique_observed_parent_has_only_a_candidate_reply_edge() {
    let root_id = mailweft_core::convert_eml(ROOT, "root", at())
        .unwrap()
        .message_candidate_id;
    let child_id = mailweft_core::convert_eml(CHILD, "child", at())
        .unwrap()
        .message_candidate_id;
    let log = convert_batch(&[source(ROOT, "root"), source(CHILD, "child")]).unwrap();
    assert_eq!(log.events.len(), 2);
    let child = log.objects.iter().find(|o| o.id == child_id).unwrap();
    assert!(
        child
            .relationships
            .iter()
            .any(|r| r.qualifier == "reply_target_candidate" && r.object_id == root_id)
    );
    assert!(
        !child
            .relationships
            .iter()
            .any(|r| r.qualifier == "verified_reply_target")
    );
    let root = log.objects.iter().find(|o| o.id == root_id).unwrap();
    assert!(
        !root
            .relationships
            .iter()
            .any(|r| r.qualifier == "reply_target_candidate")
    );
}

#[test]
fn missing_parent_is_not_fabricated() {
    let log = convert_batch(&[source(CHILD, "child")]).unwrap();
    assert!(!log.objects.iter().any(|o| {
        o.relationships
            .iter()
            .any(|r| r.qualifier == "reply_target_candidate")
    }));
    assert_eq!(
        log.objects
            .iter()
            .filter(|o| o.object_type == "email_message")
            .count(),
        1
    );
}

#[test]
fn duplicated_parent_source_copies_do_not_duplicate_thread_edges() {
    let log = convert_batch(&[source(ROOT, "a"), source(ROOT, "b"), source(CHILD, "c")]).unwrap();
    let child_id = mailweft_core::convert_eml(CHILD, "child", at())
        .unwrap()
        .message_candidate_id;
    let child = log.objects.iter().find(|o| o.id == child_id).unwrap();
    assert_eq!(
        child
            .relationships
            .iter()
            .filter(|r| r.qualifier == "reply_target_candidate")
            .count(),
        1
    );
}

#[test]
fn ambiguous_claimed_message_id_prevents_inference() {
    let other_root = b"From: different@example.invalid\r\nMessage-ID: <root@example.invalid>\r\nSubject: Different\r\n\r\nDifferent";
    let log = convert_batch(&[
        source(ROOT, "a"),
        source(other_root, "b"),
        source(CHILD, "c"),
    ])
    .unwrap();
    assert_eq!(
        log.objects
            .iter()
            .filter(|o| o.object_type == "email_message")
            .count(),
        3
    );
    assert!(!log.objects.iter().any(|o| {
        o.relationships
            .iter()
            .any(|r| r.qualifier == "reply_target_candidate")
    }));
}

#[test]
fn references_only_does_not_claim_a_direct_parent() {
    let only_references = b"From: b@example.invalid\r\nMessage-ID: <child@example.invalid>\r\nReferences: <root@example.invalid>\r\n\r\nRef only";
    let log = convert_batch(&[source(ROOT, "a"), source(only_references, "b")]).unwrap();
    assert!(!log.objects.iter().any(|o| {
        o.relationships
            .iter()
            .any(|r| r.qualifier == "reply_target_candidate")
    }));
}

#[test]
fn thread_candidates_are_independent_of_input_order() {
    let a = convert_batch(&[source(ROOT, "a"), source(CHILD, "b")]).unwrap();
    let b = convert_batch(&[source(CHILD, "b"), source(ROOT, "a")]).unwrap();
    assert_eq!(
        serde_json::to_vec(&a).unwrap(),
        serde_json::to_vec(&b).unwrap()
    );
}
