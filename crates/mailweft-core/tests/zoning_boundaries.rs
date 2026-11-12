use mailweft_core::inspect_body_zones;
use serde_json::Value;

fn inspect(body: &str) -> Value {
    let raw = format!("From: a@example.invalid\r\nSubject: Fixture\r\n\r\n{body}");
    serde_json::to_value(inspect_body_zones(raw.as_bytes()).unwrap()).unwrap()
}

#[test]
fn forwarded_history_never_becomes_new_authored_content() {
    let doc = inspect(
        "My introduction\n---------- Forwarded message ---------\nFrom: earlier@example.invalid\nHi Jane,\nPlease approve.\nKind regards,\nEarlier Author\n",
    );
    let zones = doc["parts"][0]["zones"].as_array().unwrap();
    for zone in &zones[2..] {
        assert_ne!(zone["kind"], "authored");
        assert_eq!(zone["origin_scope"], "forwarded_third_party");
    }
    assert_eq!(zones[2]["kind"], "inline_header");
}

#[test]
fn quote_depth_is_preserved_and_inline_response_is_not_quoted() {
    let doc = inspect("> First quote\n>> Nested quote\nMy new inline reply\n");
    let zones = doc["parts"][0]["zones"].as_array().unwrap();
    assert_eq!(zones[0]["quote_depth"], 1);
    assert_eq!(zones[1]["quote_depth"], 2);
    assert_eq!(zones[1]["origin_scope"], "quoted_prior_content");
    assert_eq!(zones[2]["kind"], "authored");
    assert_eq!(zones[2]["quote_depth"], 0);
}

#[test]
fn disclaimer_after_signature_is_not_signature_identity() {
    let doc = inspect(
        "Hello Jane,\nMain text\nKind regards,\nAuthor\nThis message is confidential and intended only for the addressee.\n",
    );
    let zones = doc["parts"][0]["zones"].as_array().unwrap();
    assert_eq!(zones.last().unwrap()["kind"], "disclaimer");
}

#[test]
fn skipped_large_body_is_reported_rather_than_silently_lost() {
    let doc = inspect(&"x".repeat(2 * 1024 * 1024 + 1));
    assert!(doc["parts"].as_array().unwrap().is_empty());
    assert_eq!(doc["skipped_parts"][0]["reason"], "decoded_size_limit");
}
