//! Synthetic malformed mail must preserve source identity and valid OCEL links.
use chrono::{DateTime, Utc};
use mailweft_core::ocel_validate::validate_ocel;
use mailweft_core::source::parse_envelope_json;
use mailweft_core::{convert_eml, inspect_body_zones};
use sha2::{Digest, Sha256};

pub fn check(data: &[u8]) {
    if data.len() > 65_536 {
        return;
    }
    let observed: DateTime<Utc> = "2026-01-01T00:00:00Z"
        .parse()
        .expect("fixed synthetic instant");
    let first = convert_eml(data, "synthetic:fuzz:one", observed);
    let repeated = convert_eml(data, "synthetic:fuzz:one", observed);
    assert_eq!(
        first.as_ref().map_err(|e| e.to_string()),
        repeated.as_ref().map_err(|e| e.to_string())
    );
    if let Ok(conversion) = first {
        assert_eq!(
            conversion.source_sha256,
            format!("{:x}", Sha256::digest(data))
        );
        assert!(
            validate_ocel(&conversion.log).is_ok(),
            "successful conversion must be structurally valid OCEL"
        );
        let other = convert_eml(data, "synthetic:fuzz:two", observed)
            .expect("source identity does not change parsing");
        assert_eq!(other.source_sha256, conversion.source_sha256);
        assert_eq!(other.message_candidate_id, conversion.message_candidate_id);
        assert_ne!(other.source_copy_id, conversion.source_copy_id);
        let json = serde_json::to_vec(&conversion.log).expect("OCEL serializes");
        let decoded: mailweft_core::Ocel = serde_json::from_slice(&json).expect("OCEL round trip");
        assert_eq!(decoded, conversion.log);
    }
    let _ = inspect_body_zones(data);
    let _ = parse_envelope_json(data);
}
