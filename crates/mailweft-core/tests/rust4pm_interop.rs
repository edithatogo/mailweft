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
    assert_eq!(imported.events[0].relationships.len(), conversion.log.events[0].relationships.len());
}
