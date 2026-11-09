use mailweft_core::inspect_body_zones;
use serde_json::Value;

fn inspect(encoding: &str, charset: &str, body: &str) -> (Vec<u8>, Value) {
    let raw = format!("From: example@example.invalid\r\nContent-Type: text/plain; charset={charset}\r\nContent-Transfer-Encoding: {encoding}\r\n\r\n{body}").into_bytes();
    let doc = serde_json::to_value(inspect_body_zones(&raw).unwrap()).unwrap();
    (raw, doc)
}

#[test]
fn base64_maps_decoded_units_to_original_quartets_not_fabricated_char_offsets() {
    let (raw, doc) = inspect("base64", "utf-8", "SGVs\r\nbG8K");
    let part = &doc["parts"][0];
    let map = &part["transfer_map"];
    assert_eq!(map["status"], "verified_transfer");
    assert_eq!(map["units"].as_array().unwrap().len(), 2);
    let first = &map["units"][0];
    assert_eq!(first["decoded_start"], 0);
    assert_eq!(first["decoded_end"], 3);
    let s = first["raw_start"].as_u64().unwrap() as usize;
    let e = first["raw_end"].as_u64().unwrap() as usize;
    assert_eq!(&raw[s..e], b"SGVs");
    assert_eq!(part["zones"][0]["offset_quality"], "derived");
    assert!(part["zones"][0]["raw_start"].is_null());
}

#[test]
fn quoted_printable_preserves_escapes_and_source_only_soft_breaks() {
    let (raw, doc) = inspect("quoted-printable", "utf-8", "A=3DB=\r\nC");
    let map = &doc["parts"][0]["transfer_map"];
    assert_eq!(map["status"], "verified_transfer");
    assert_eq!(map["decoded_byte_length"], 4);
    let ranges = map["source_only_ranges"].as_array().unwrap();
    assert_eq!(ranges.len(), 1);
    let s = ranges[0]["start"].as_u64().unwrap() as usize;
    let e = ranges[0]["end"].as_u64().unwrap() as usize;
    assert_eq!(&raw[s..e], b"=\r\n");
    assert!(map["units"].as_array().unwrap().iter().any(|u| {
        let s = u["raw_start"].as_u64().unwrap() as usize;
        let e = u["raw_end"].as_u64().unwrap() as usize;
        &raw[s..e] == b"=3D" && u["decoded_start"] == 1 && u["decoded_end"] == 2
    }));
}

#[test]
fn charset_transcoding_and_malformed_encoding_abstain_from_transfer_precision() {
    let (_, latin) = inspect("quoted-printable", "iso-8859-1", "=E9");
    assert_eq!(latin["parts"][0]["transfer_map"]["status"], "unavailable");
    assert_eq!(latin["parts"][0]["transfer_map"]["reason"], "charset_transformation");
    let (_, bad) = inspect("base64", "utf-8", "!SGk=");
    assert_eq!(bad["parts"][0]["transfer_map"]["status"], "unavailable");
    assert!(bad["parts"][0]["transfer_map"]["units"].as_array().unwrap().is_empty());
}

#[test]
fn unicode_plaintext_identity_map_remains_byte_based() {
    let (raw, doc) = inspect("8bit", "utf-8", "Hello é\r\n");
    let part = &doc["parts"][0];
    assert_eq!(part["transfer_map"]["status"], "identity");
    assert_eq!(part["decoded_byte_length"], 10);
    let unit = &part["transfer_map"]["units"][0];
    assert_eq!(&raw[unit["raw_start"].as_u64().unwrap() as usize..unit["raw_end"].as_u64().unwrap() as usize], "Hello é\r\n".as_bytes());
}
