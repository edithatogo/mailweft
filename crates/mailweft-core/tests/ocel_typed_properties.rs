use chrono::{TimeZone, Utc};
use mailweft_core::{Ocel, SourceInput, convert_batch, convert_eml, ocel_validate::validate_ocel};
use proptest::prelude::*;
use serde_json::json;

fn typed_fixture() -> Ocel {
    serde_json::from_value(json!({
        "objectTypes": [{
            "name": "case",
            "attributes": [
                {"name": "label", "type": "string"},
                {"name": "count", "type": "integer"},
                {"name": "cost", "type": "float"},
                {"name": "verified", "type": "boolean"},
                {"name": "as_of", "type": "time"}
            ]
        }],
        "eventTypes": [{
            "name": "observed",
            "attributes": [
                {"name": "cost", "type": "float"},
                {"name": "verified", "type": "boolean"}
            ]
        }],
        "objects": [{
            "id": "o1", "type": "case",
            "attributes": [
                {"name": "label", "time": "2026-11-10T00:00:00Z", "value": "sample"},
                {"name": "count", "time": "2026-11-10T00:00:00Z", "value": 23},
                {"name": "cost", "time": "2026-11-10T00:00:00Z", "value": 2.5},
                {"name": "verified", "time": "2026-11-10T00:00:00Z", "value": true},
                {"name": "as_of", "time": "2026-11-10T00:00:00Z", "value": "2026-11-10T00:00:00Z"},
                {"name": "cost", "time": "2026-11-10T01:00:00Z", "value": 3.5}
            ],
            "relationships": []
        }],
        "events": [{
            "id": "e1", "type": "observed", "time": "2026-11-10T00:00:00Z",
            "attributes": [{"name": "cost", "value": 2.5}, {"name": "verified", "value": true}],
            "relationships": [{"objectId": "o1", "qualifier": "case"}]
        }]
    }))
    .unwrap()
}

#[test]
fn all_five_native_ocel_attribute_types_survive_json_roundtrip() {
    let log = typed_fixture();
    validate_ocel(&log).unwrap();
    let json = serde_json::to_vec(&log).unwrap();
    let decoded: Ocel = serde_json::from_slice(&json).unwrap();
    assert_eq!(decoded, log);
    assert!(
        serde_json::to_string(&log)
            .unwrap()
            .contains("\"value\":true")
    );
    assert!(
        serde_json::to_string(&log)
            .unwrap()
            .contains("\"value\":23")
    );
}

#[test]
fn type_mismatches_fail_closed() {
    let mut log = typed_fixture();
    log.events[0].attributes[0].value = json!("not a float");
    let violations = validate_ocel(&log).unwrap_err();
    assert!(
        violations
            .iter()
            .any(|v| v.code == "EVENT_ATTRIBUTE_TYPE_MISMATCH")
    );

    let mut log = typed_fixture();
    let obj = &mut log.objects[0];
    obj.attributes
        .iter_mut()
        .find(|a| a.name == "as_of")
        .unwrap()
        .value = json!("not a timestamp");
    assert!(
        validate_ocel(&log)
            .unwrap_err()
            .iter()
            .any(|v| v.code == "OBJECT_ATTRIBUTE_TYPE_MISMATCH")
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn raw_mail_conversion_deterministic_and_well_typed(body in "[a-zA-Z0-9 ]{0,100}") {
        let raw = format!("From: a@example.invalid\r\nSubject: Synthetic\r\n\r\n{body}");
        let at = Utc.with_ymd_and_hms(2026, 11, 10, 0, 0, 0).unwrap();
        let a = convert_eml(raw.as_bytes(), "source", at).unwrap();
        let b = convert_eml(raw.as_bytes(), "source", at).unwrap();
        prop_assert_eq!(&a, &b);
        prop_assert!(validate_ocel(&a.log).is_ok());
    }

    #[test]
    fn order_metamorphism_preserves_ocel(
        a in "[a-z]{1,30}", b in "[a-z]{1,30}"
    ) {
        let raw_a = format!("From: a@example.invalid\r\nSubject: {a}\r\n\r\nFirst");
        let raw_b = format!("From: b@example.invalid\r\nSubject: {b}\r\n\r\nSecond");
        let at = Utc.with_ymd_and_hms(2026, 11, 10, 0, 0, 0).unwrap();
        let sources = [
            SourceInput {raw: raw_a.as_bytes(), source_id: "first", observed_at: at},
            SourceInput {raw: raw_b.as_bytes(), source_id: "second", observed_at: at}
        ];
        let one = convert_batch(&sources).unwrap();
        let two = convert_batch(&[sources[1].clone(), sources[0].clone()]).unwrap();
        prop_assert_eq!(one, two);
    }
}
