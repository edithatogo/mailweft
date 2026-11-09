#![cfg(feature = "rust4pm-validation")]
use chrono::{TimeZone, Utc};
use mailweft_core::convert_eml;
use process_mining::{Importable, OCEL};
use std::io::Write;

#[test]
fn rust4pm_imports_exported_ocel_with_roles_and_objects() {
    let raw = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nCc: c@example.invalid\r\nDate: Mon, 09 Nov 2026 11:02:03 +1000\r\nSubject: Action\r\n\r\nPlease review.\r\n";
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 1, 2, 3).unwrap();
    let conversion = convert_eml(raw, "fixture/copy", at).unwrap();
    let mut file = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    serde_json::to_writer(&mut file, &conversion.log).unwrap();
    file.flush().unwrap();

    // Engine-independent compatibility acceptance, not our own serde roundtrip.
    let imported = OCEL::import_from_path(file.path()).unwrap();
    assert_eq!(imported.events.len(), 1);
    assert_eq!(imported.objects.len(), conversion.log.objects.len());
    assert_eq!(
        imported.events[0].relationships.len(),
        conversion.log.events[0].relationships.len()
    );
}

#[test]
fn rust4pm_imports_two_copy_events_with_merged_message_relationships() {
    use mailweft_core::{SourceInput, convert_batch};
    let raw = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nSubject: Sample\r\n\r\nhello";
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 1, 2, 3).unwrap();
    let sources = [
        SourceInput {
            raw,
            source_id: "custodian/a",
            observed_at: at,
        },
        SourceInput {
            raw,
            source_id: "custodian/b",
            observed_at: at,
        },
    ];
    let log = convert_batch(&sources).unwrap();
    let mut file = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    serde_json::to_writer(&mut file, &log).unwrap();
    file.flush().unwrap();
    let loaded = OCEL::import_from_path(file.path()).unwrap();
    assert_eq!(loaded.events.len(), 2);
    assert_eq!(loaded.objects.len(), log.objects.len());
}

#[test]
fn rust4pm_import_preserves_claimed_header_attributes_and_reference_qualifiers() {
    let raw = b"From: a@example.invalid\r\nDate: Sun, 08 Nov 2026 20:30:00 -0500\r\nMessage-ID: <child@example.invalid>\r\nIn-Reply-To: <parent@example.invalid>\r\nReferences: <root@example.invalid> <parent@example.invalid>\r\n\r\nBody";
    let observed = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let conversion = convert_eml(raw, "held/1", observed).unwrap();
    let mut file = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    serde_json::to_writer(&mut file, &conversion.log).unwrap();
    file.flush().unwrap();
    let imported = OCEL::import_from_path(file.path()).unwrap();
    assert_eq!(imported.events.len(), 1);
    assert_eq!(imported.objects.len(), conversion.log.objects.len());
    assert_eq!(
        imported.events[0].relationships.len(),
        conversion.log.events[0].relationships.len()
    );
    assert_eq!(
        imported.events[0].attributes.len(),
        conversion.log.events[0].attributes.len()
    );
}
