use chrono::{TimeZone, Utc};
use mailweft_core::{OffsetQuality, ZoneKind, convert_eml, inspect_body_zones};

const RAW: &[u8] = b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Review\r\n\r\nHi Bob,\r\nPlease review the paper.\r\n> Earlier message\r\nKind regards,\r\nAlice\r\n";

#[test]
fn classify_complete_plaintext_body_without_dropping_lines() {
    let evidence = inspect_body_zones(RAW).unwrap();
    assert_eq!(evidence.source_sha256.len(), 64);
    assert_eq!(evidence.parts.len(), 1);
    let zones = &evidence.parts[0].zones;
    assert!(zones.iter().any(|z| z.kind == ZoneKind::Salutation));
    assert!(zones.iter().any(|z| z.kind == ZoneKind::Authored));
    assert!(zones.iter().any(|z| z.kind == ZoneKind::Quotation));
    assert!(zones.iter().any(|z| z.kind == ZoneKind::Signoff));
    assert!(zones.iter().any(|z| z.kind == ZoneKind::Signature));
    assert!(
        zones
            .iter()
            .all(|z| z.offset_quality == OffsetQuality::Exact)
    );
    assert_eq!(zones.first().unwrap().decoded_start, 0);
    assert_eq!(
        zones.last().unwrap().decoded_end,
        evidence.parts[0].decoded_byte_length
    );
    assert!(
        zones
            .windows(2)
            .all(|w| w[0].decoded_end == w[1].decoded_start)
    );
    let raw_body = &RAW[evidence.parts[0].raw_start..evidence.parts[0].raw_end];
    for z in zones {
        let start = z.raw_start.expect("exact");
        let end = z.raw_end.expect("exact");
        assert_eq!(&RAW[start..end], &raw_body[z.decoded_start..z.decoded_end]);
        assert_eq!(z.sha256.len(), 64);
    }
    let public = serde_json::to_string(&evidence).unwrap();
    for secret in [
        "alice@example.invalid",
        "Please review",
        "Earlier message",
        "Hi Bob",
        "Kind regards",
    ] {
        assert!(!public.contains(secret));
    }
}

#[test]
fn transfer_decoding_never_fabricates_exact_source_offsets() {
    let raw = b"From: a@example.invalid\r\nMIME-Version: 1.0\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\nSGkgQm9iLAo+IFF1b3RlZAo=";
    let e = inspect_body_zones(raw).unwrap();
    assert_eq!(e.parts.len(), 1);
    assert!(
        e.parts[0]
            .zones
            .iter()
            .all(|z| z.offset_quality == OffsetQuality::Derived
                && z.raw_start.is_none()
                && z.raw_end.is_none())
    );
}

#[test]
fn opaque_html_is_not_misrepresented_as_plaintext() {
    let raw = b"From: a@example.invalid\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<p>Hello <b>Bob</b></p>";
    let e = inspect_body_zones(raw).unwrap();
    assert!(e.parts.is_empty());
    assert_eq!(e.skipped_html_parts, 1);
}

#[test]
fn no_plaintext_body_yields_no_zones() {
    let raw = b"From: a@example.invalid\r\nSubject: hi\r\n\r\n";
    let e = inspect_body_zones(raw).unwrap();
    assert!(e.parts.iter().all(|p| p.zones.is_empty()));
}

#[test]
fn source_inspection_does_not_mutate_authoritative_ocel() {
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let before = convert_eml(RAW, "copy", at).unwrap();
    let _ = inspect_body_zones(RAW).unwrap();
    let after = convert_eml(RAW, "copy", at).unwrap();
    assert_eq!(before, after);
}

#[test]
fn parser_declares_all_constructed_attachment_objects() {
    let raw = b"From: a@example.invalid\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=\"m\"\r\n\r\n--m\r\nContent-Type: text/plain\r\n\r\nhello\r\n--m\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"doc.bin\"\r\nContent-Transfer-Encoding: base64\r\n\r\nQUJD\r\n--m--\r\n";
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    let c = convert_eml(raw, "copy", at).unwrap();
    for obj in &c.log.objects {
        let ty = c
            .log
            .object_types
            .iter()
            .find(|t| t.name == obj.object_type)
            .unwrap_or_else(|| panic!("undeclared object type: {}", obj.object_type));
        for attr in &obj.attributes {
            assert!(ty.attributes.iter().any(|t| t.name == attr.name));
        }
    }
}

#[test]
fn unstructured_bytes_are_not_mislabelled_as_email_evidence() {
    assert!(inspect_body_zones(b"not an RFC 5322 email").is_err());
}
