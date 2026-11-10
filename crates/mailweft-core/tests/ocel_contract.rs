use chrono::{TimeZone, Utc};
use mailweft_core::{Ocel, convert_eml, ocel_validate::validate_ocel};

const MIME: &[u8] = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nMessage-ID: <a@example.invalid>\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=\"part\"\r\n\r\n--part\r\nContent-Type: text/plain\r\n\r\nHello\r\n--part\r\nContent-Type: text/plain; name=\"report.txt\"\r\nContent-Disposition: attachment; filename=\"report.txt\"\r\nContent-Transfer-Encoding: base64\r\n\r\nSEVMTE8=\r\n--part--\r\n";
fn sample() -> Ocel {
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    convert_eml(MIME, "custodian", at).unwrap().log
}
fn codes(log: &Ocel) -> Vec<&'static str> {
    validate_ocel(log)
        .unwrap_err()
        .into_iter()
        .map(|x| x.code)
        .collect()
}
#[test]
fn real_generated_ocel_validates_without_mutation() {
    let log = sample();
    validate_ocel(&log).unwrap();
    let encoded = serde_json::to_vec(&log).unwrap();
    let roundtrip: Ocel = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(roundtrip, log);
    validate_ocel(&roundtrip).unwrap();
}
#[test]
fn dangling_object_reference_is_detected() {
    let mut log = sample();
    log.events[0].relationships[0].object_id = "missing".into();
    assert!(codes(&log).contains(&"RELATIONSHIP_TARGET_MISSING"));
}
#[test]
fn missing_type_declaration_is_detected() {
    let mut log = sample();
    log.object_types.retain(|t| t.name != "mime_attachment");
    assert!(codes(&log).contains(&"OBJECT_TYPE_UNDECLARED"));
}
#[test]
fn missing_attribute_declaration_is_detected() {
    let mut log = sample();
    let ty = log
        .object_types
        .iter_mut()
        .find(|t| t.name == "mime_attachment")
        .unwrap();
    ty.attributes.clear();
    assert!(codes(&log).contains(&"OBJECT_ATTRIBUTE_UNDECLARED"));
}
#[test]
fn duplicate_qualified_relationship_is_detected() {
    let mut log = sample();
    let rel = log.events[0].relationships[0].clone();
    log.events[0].relationships.push(rel);
    assert!(codes(&log).contains(&"RELATIONSHIP_DUPLICATE"));
}
#[test]
fn conflicting_object_attribute_history_is_detected() {
    let mut log = sample();
    let attachment = log
        .objects
        .iter_mut()
        .find(|o| o.object_type == "mime_attachment")
        .unwrap();
    let mut conflicting = attachment.attributes[0].clone();
    conflicting.value = "different".into();
    attachment.attributes.push(conflicting);
    assert!(codes(&log).contains(&"OBJECT_ATTRIBUTE_CONFLICT"));
}
#[test]
fn mismatched_declared_type_is_not_silently_coerced() {
    let mut log = sample();
    let typ = log
        .object_types
        .iter_mut()
        .find(|t| t.name == "mime_attachment")
        .unwrap();
    typ.attributes[0].value_type = "integer".into();
    assert!(codes(&log).contains(&"OBJECT_ATTRIBUTE_TYPE_MISMATCH"));
}
