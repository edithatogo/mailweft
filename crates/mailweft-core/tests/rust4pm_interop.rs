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

#[test]
fn rust4pm_imports_time_qualified_attachment_object_attributes() {
    let raw = b"From: a@example.invalid\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=\"x\"\r\n\r\n--x\r\nContent-Type: text/plain\r\n\r\nHello\r\n--x\r\nContent-Type: text/plain; name=\"report.txt\"\r\nContent-Disposition: attachment; filename=\"report.txt\"\r\nContent-Transfer-Encoding: base64\r\n\r\nSEVMTE8=\r\n--x--\r\n";
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let conversion = convert_eml(raw, "copy", at).unwrap();
    let mut file = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    serde_json::to_writer(&mut file, &conversion.log).unwrap();
    file.flush().unwrap();
    let imported = OCEL::import_from_path(file.path()).unwrap();
    assert_eq!(imported.events.len(), 1);
    assert_eq!(imported.objects.len(), conversion.log.objects.len());
    let source_attachment = conversion
        .log
        .objects
        .iter()
        .find(|o| o.object_type == "mime_attachment")
        .unwrap();
    let loaded_attachment = imported
        .objects
        .iter()
        .find(|o| o.id == source_attachment.id)
        .unwrap();
    assert_eq!(
        loaded_attachment.attributes.len(),
        source_attachment.attributes.len()
    );
    assert_eq!(
        imported.events[0].relationships.len(),
        conversion.log.events[0].relationships.len()
    );
}

#[test]
fn rust4pm_preserves_candidate_reply_object_links() {
    use mailweft_core::{SourceInput, convert_batch};
    let root = b"From: a@example.invalid\r\nMessage-ID: <root@example.invalid>\r\n\r\nRoot";
    let child = b"From: b@example.invalid\r\nMessage-ID: <child@example.invalid>\r\nIn-Reply-To: <root@example.invalid>\r\n\r\nReply";
    let observed = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let sources = [
        SourceInput {
            raw: root,
            source_id: "root",
            observed_at: observed,
        },
        SourceInput {
            raw: child,
            source_id: "child",
            observed_at: observed,
        },
    ];
    let log = convert_batch(&sources).unwrap();
    let expected = log
        .objects
        .iter()
        .filter(|o| {
            o.relationships
                .iter()
                .any(|r| r.qualifier == "reply_target_candidate")
        })
        .count();
    assert_eq!(expected, 1);
    let mut file = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    serde_json::to_writer(&mut file, &log).unwrap();
    file.flush().unwrap();
    let imported = OCEL::import_from_path(file.path()).unwrap();
    assert_eq!(imported.objects.len(), log.objects.len());
    let actual = imported
        .objects
        .iter()
        .filter(|o| {
            o.relationships
                .iter()
                .any(|r| r.qualifier == "reply_target_candidate")
        })
        .count();
    assert_eq!(actual, expected);
}

#[test]
fn rust4pm_xml_roundtrip_preserves_typed_attributes_and_qualified_links() {
    use process_mining::Exportable;
    let raw = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nMessage-ID: <child@example.invalid>\r\nDate: Tue, 10 Nov 2026 07:00:00 +0000\r\n\r\nHello";
    let at = Utc.with_ymd_and_hms(2026, 11, 10, 7, 0, 0).unwrap();
    let log = convert_eml(raw, "synthetic", at).unwrap().log;
    let dir = tempfile::tempdir().unwrap();
    let json = dir.path().join("case.json");
    let xml = dir.path().join("case.xml");
    std::fs::write(&json, serde_json::to_vec(&log).unwrap()).unwrap();
    let original = OCEL::import_from_path(&json).unwrap();
    original.export_to_path(&xml).unwrap();
    let restored = OCEL::import_from_path(&xml).unwrap();
    assert_eq!(restored.object_types, original.object_types);
    assert_eq!(restored.event_types, original.event_types);
    assert_eq!(restored.objects, original.objects);
    assert_eq!(restored.events, original.events);
}

#[cfg(feature = "rust4pm-sqlite-validation")]
#[test]
fn rust4pm_sqlite_roundtrip_preserves_typed_attributes_and_qualified_links() {
    use process_mining::Exportable;
    let raw = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nDate: Tue, 10 Nov 2026 07:00:00 +0000\r\n\r\nHello";
    let at = Utc.with_ymd_and_hms(2026, 11, 10, 7, 0, 0).unwrap();
    let log = convert_eml(raw, "synthetic", at).unwrap().log;
    let dir = tempfile::tempdir().unwrap();
    let json = dir.path().join("case.json");
    let sqlite = dir.path().join("case.sqlite");
    std::fs::write(&json, serde_json::to_vec(&log).unwrap()).unwrap();
    let original = OCEL::import_from_path(&json).unwrap();
    original.export_to_path(&sqlite).unwrap();
    let restored = OCEL::import_from_path(&sqlite).unwrap();
    assert_eq!(restored.object_types, original.object_types);
    assert_eq!(restored.event_types, original.event_types);
    assert_eq!(restored.objects, original.objects);
    assert_eq!(restored.events, original.events);
}
