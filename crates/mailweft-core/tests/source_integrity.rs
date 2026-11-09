use mailweft_core::{convert_eml, source::{SourceEnvelope, convert_with_envelope, parse_envelope_json}};

const RAW: &[u8] = b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Fixture\r\n\r\nHello\r\n";
const ENVELOPE: &[u8] = include_bytes!("../../../fixtures/synthetic/contracts/source-envelope.json");

#[test]
fn evidence_bundle_preserves_the_existing_ocel_projection() {
    let envelope = parse_envelope_json(ENVELOPE).unwrap();
    let baseline = convert_eml(RAW, &envelope.source_id, envelope.observed_at).unwrap();
    let result = convert_with_envelope(RAW, &envelope).unwrap();
    assert_eq!(result.conversion, baseline);
}

#[test]
fn metadata_revisions_change_receipt_not_observed_events() {
    let mut envelope = parse_envelope_json(ENVELOPE).unwrap();
    let first = convert_with_envelope(RAW, &envelope).unwrap();
    envelope.rights_ref = "revised-scope-record".into();
    let second = convert_with_envelope(RAW, &envelope).unwrap();
    assert_eq!(first.conversion, second.conversion);
    assert_ne!(first.receipt.metadata_sha256, second.receipt.metadata_sha256);
    assert_ne!(first.receipt.rights_ref_sha256, second.receipt.rights_ref_sha256);
}

#[test]
fn json_field_order_does_not_change_semantic_metadata_fingerprint() {
    let original = parse_envelope_json(ENVELOPE).unwrap();
    let value: serde_json::Value = serde_json::from_slice(ENVELOPE).unwrap();
    let reordered: SourceEnvelope = serde_json::from_value(value).unwrap();
    assert_eq!(convert_with_envelope(RAW, &original).unwrap().receipt,
        convert_with_envelope(RAW, &reordered).unwrap().receipt);
}

#[test]
fn duplicate_json_fields_and_unknown_producer_keys_are_rejected() {
    let text = std::str::from_utf8(ENVELOPE).unwrap();
    let duplicate = text.replacen("{", "{\"schema_version\":\"1.0.0\",", 1);
    assert!(parse_envelope_json(duplicate.as_bytes()).is_err());
    let mut value: serde_json::Value = serde_json::from_slice(ENVELOPE).unwrap();
    value["producer"]["unexpected"] = serde_json::json!("no");
    assert!(parse_envelope_json(&serde_json::to_vec(&value).unwrap()).is_err());
}

#[test]
fn absent_timestamp_is_never_replaced_with_current_time() {
    let mut value: serde_json::Value = serde_json::from_slice(ENVELOPE).unwrap();
    value.as_object_mut().unwrap().remove("observed_at");
    assert!(parse_envelope_json(&serde_json::to_vec(&value).unwrap()).is_err());
}
