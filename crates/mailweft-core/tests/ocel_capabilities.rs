use chrono::{TimeZone, Utc};
use mailweft_core::{convert_eml, ocel_capabilities::capability_report};

#[test]
fn capability_ledger_is_stable_and_does_not_change_source_ocel() {
    let raw = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nSubject: Evidence\r\n\r\nSynthetic text";
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let log = convert_eml(raw, "synthetic/1", at).unwrap().log;
    let before = serde_json::to_vec(&log).unwrap();
    let report = capability_report(&log);
    assert!(report.validation_passed);
    assert!(report.validation_codes.is_empty());
    assert_eq!(report.event_count, 1);
    assert!(
        report
            .loss_entries
            .iter()
            .any(|e| e.code == "STRING_ATTRIBUTE_PROFILE")
    );
    assert!(
        report
            .loss_entries
            .iter()
            .any(|e| e.code == "FORMAT_ROUNDTRIP_UNQUALIFIED")
    );
    assert_eq!(serde_json::to_vec(&log).unwrap(), before);
    assert_eq!(capability_report(&log), report);
}

#[test]
fn invalid_ocel_is_not_presented_as_conformant() {
    let raw = b"From: a@example.invalid\r\nSubject: Example\r\n\r\nText";
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let mut log = convert_eml(raw, "synthetic/1", at).unwrap().log;
    log.events[0].relationships[0].object_id = "absent".into();
    let report = capability_report(&log);
    assert!(!report.validation_passed);
    assert!(
        report
            .validation_codes
            .contains(&"RELATIONSHIP_TARGET_MISSING")
    );
}
