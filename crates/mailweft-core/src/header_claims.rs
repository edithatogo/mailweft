//! Email header *claims*, distinct from independently observed source acquisition.
//! Identifiers are represented as non-reversible, deterministic opaque objects.
use crate::{Attribute, AttributeType, Object, Relationship, identity};
use chrono::{TimeZone, Utc};
use mail_parser::Message;
use std::collections::BTreeMap;

fn status(attributes: &mut Vec<Attribute>, name: &str, value: &str) {
    attributes.push(Attribute {
        name: name.to_owned(),
        value: value.to_owned(),
    });
}

/// Preserve a claimed message identifier only if its parsed form is plausible.
/// This does not prove that the referenced email exists or was sent.
/// The identifier is case-preserving, and its SHA-256 pseudonym is not anonymity.
fn canonical_id(value: &str) -> Option<&str> {
    let id = value.trim().trim_start_matches('<').trim_end_matches('>');
    if id.is_empty()
        || id.len() > 998
        || !id.contains('@')
        || id
            .bytes()
            .any(|b| b.is_ascii_whitespace() || b == b'<' || b == b'>' || b == 0 || !b.is_ascii())
    {
        None
    } else {
        Some(id)
    }
}

fn add_id_object(
    id: &str,
    qualifier: &str,
    relationships: &mut Vec<Relationship>,
    objects: &mut BTreeMap<String, Object>,
) {
    let object_id = format!("claimed_id_{}", identity("claimed-rfc-message-id", id));
    objects.entry(object_id.clone()).or_insert_with(|| Object {
        id: object_id.clone(),
        object_type: "claimed_message_identifier".into(),
        attributes: Vec::new(),
        relationships: Vec::new(),
    });
    let link = Relationship {
        object_id,
        qualifier: qualifier.to_owned(),
    };
    if !relationships.contains(&link) {
        relationships.push(link);
    }
}

fn count_headers(message: &Message<'_>, name: &str) -> usize {
    message
        .headers()
        .iter()
        .filter(|field| field.raw_name().eq_ignore_ascii_case(name))
        .count()
}

fn claim_id_list(
    values: Option<mail_parser::TextList<'_>>,
    name: &str,
    qualifier: &str,
    count: usize,
    attrs: &mut Vec<Attribute>,
    relations: &mut Vec<Relationship>,
    objects: &mut BTreeMap<String, Object>,
) {
    let key = format!("{name}_claim_status");
    if count == 0 {
        status(attrs, &key, "absent");
        return;
    }
    if count > 1 {
        status(attrs, &key, "ambiguous");
        return;
    }
    let Some(values) = values else {
        status(attrs, &key, "malformed");
        return;
    };
    let parsed: Vec<_> = values.iter().map(canonical_id).collect();
    if parsed.is_empty() || parsed.iter().any(|value| value.is_none()) || parsed.len() > 1000 {
        status(attrs, &key, "malformed");
        return;
    }
    for item in parsed.into_iter().flatten() {
        add_id_object(item, qualifier, relations, objects);
    }
    status(attrs, &key, "parsed");
}

pub(crate) fn add_claims(
    message: &Message<'_>,
    attrs: &mut Vec<Attribute>,
    relations: &mut Vec<Relationship>,
    objects: &mut BTreeMap<String, Object>,
) {
    // Date is an assertion by the source, not the OCEL event time. Reject
    // ambiguous duplicates even if a parser can select the last one.
    match count_headers(message, "date") {
        0 => status(attrs, "date_claim_status", "absent"),
        1 => {
            let claimed = message
                .date()
                .filter(|date| date.is_valid())
                .and_then(|date| Utc.timestamp_opt(date.to_timestamp(), 0).single());
            if let Some(claimed) = claimed {
                status(attrs, "date_claim_status", "parsed");
                status(attrs, "claimed_date_utc", &claimed.to_rfc3339());
            } else {
                status(attrs, "date_claim_status", "malformed");
            }
        }
        _ => status(attrs, "date_claim_status", "ambiguous"),
    }

    match count_headers(message, "message-id") {
        0 => status(attrs, "message_id_claim_status", "absent"),
        1 => {
            if let Some(id) = message.message_id().and_then(canonical_id) {
                add_id_object(id, "claimed_message_id", relations, objects);
                status(attrs, "message_id_claim_status", "parsed");
            } else {
                status(attrs, "message_id_claim_status", "malformed");
            }
        }
        _ => status(attrs, "message_id_claim_status", "ambiguous"),
    }

    claim_id_list(
        message.in_reply_to(),
        "in_reply_to",
        "claimed_in_reply_to",
        count_headers(message, "in-reply-to"),
        attrs,
        relations,
        objects,
    );
    claim_id_list(
        message.references(),
        "references",
        "claimed_reference",
        count_headers(message, "references"),
        attrs,
        relations,
        objects,
    );
}

pub(crate) fn attribute_types() -> Vec<AttributeType> {
    [
        "source_sha256",
        "date_claim_status",
        "claimed_date_utc",
        "message_id_claim_status",
        "in_reply_to_claim_status",
        "references_claim_status",
    ]
    .into_iter()
    .map(|name| AttributeType {
        name: name.into(),
        value_type: "string".into(),
    })
    .collect()
}
