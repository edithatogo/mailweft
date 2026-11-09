//! Mailweft: an offline, corpus-independent EML-to-OCEL 2.0 converter.
//! Stage 1 records archive observation, not proof of sending, delivery or business action.
mod attachments;
mod batch;
mod header_claims;
mod zones;
pub use batch::{SourceInput, convert_batch};
pub use zones::{
    BodyEvidence, OffsetQuality, SourceZone, TextPartEvidence, ZoneKind, inspect_body_zones,
};

use chrono::{DateTime, Utc};
use mail_parser::{AddressList, MessageParser};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAX_EML_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("source is empty, exceeds 16 MiB, or lacks recognizable RFC 5322 headers")]
    InvalidSource,
    #[error("source identifier must be non-empty")]
    InvalidSourceId,
    #[error("cannot convert an empty input batch")]
    EmptyBatch,
    #[error("conflicting data mapped to the same deterministic identifier")]
    IdentityCollision,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeType {
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OcelType {
    pub name: String,
    pub attributes: Vec<AttributeType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObjectAttribute {
    pub name: String,
    pub time: DateTime<Utc>,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Relationship {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub qualifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Object {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub attributes: Vec<ObjectAttribute>,
    pub relationships: Vec<Relationship>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub time: DateTime<Utc>,
    pub attributes: Vec<Attribute>,
    pub relationships: Vec<Relationship>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ocel {
    #[serde(rename = "objectTypes")]
    pub object_types: Vec<OcelType>,
    #[serde(rename = "eventTypes")]
    pub event_types: Vec<OcelType>,
    pub objects: Vec<Object>,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversion {
    pub schema_version: &'static str,
    pub source_sha256: String,
    pub message_candidate_id: String,
    pub source_copy_id: String,
    pub log: Ocel,
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn identity(namespace: &str, value: &str) -> String {
    digest(format!("mailweft/v1/{namespace}\0{value}").as_bytes())
}

fn push_address_links(
    relationships: &mut Vec<Relationship>,
    objects: &mut BTreeMap<String, Object>,
    role: &str,
    addresses: Option<AddressList<'_>>,
) {
    if let Some(addresses) = addresses {
        for mailbox in addresses.mailboxes() {
            // An address is evidence of a mailbox identifier, not proof of a person.
            if let Some(address) = mailbox.address().filter(|value| !value.trim().is_empty()) {
                let normalized = address.trim().to_ascii_lowercase();
                let object_id = format!("address_{}", identity("address", &normalized));
                objects.entry(object_id.clone()).or_insert_with(|| Object {
                    id: object_id.clone(),
                    object_type: "email_address".to_string(),
                    attributes: Vec::new(),
                    relationships: Vec::new(),
                });
                let relation = Relationship {
                    object_id,
                    qualifier: role.to_string(),
                };
                if !relationships.contains(&relation) {
                    relationships.push(relation);
                }
            }
        }
    }
}

/// Convert a raw email source copy into a standards-shaped OCEL 2.0 JSON model.
///
/// The required `observed_at` comes from an actual supplied acquisition timestamp;
/// Date headers are *not* treated as independent proof of sending/delivery.
/// Distinct source IDs preserve distinct mailbox copies of otherwise identical bytes.
pub fn convert_eml(
    raw: &[u8],
    source_id: &str,
    observed_at: DateTime<Utc>,
) -> Result<Conversion, Error> {
    if raw.is_empty() || raw.len() > MAX_EML_BYTES {
        return Err(Error::InvalidSource);
    }
    if source_id.trim().is_empty() {
        return Err(Error::InvalidSourceId);
    }
    let parsed = MessageParser::new()
        .parse(raw)
        .ok_or(Error::InvalidSource)?;
    if parsed.from().is_none()
        && parsed.to().is_none()
        && parsed.sender().is_none()
        && parsed.subject().is_none()
        && parsed.message_id().is_none()
    {
        return Err(Error::InvalidSource);
    }
    let source_sha256 = digest(raw);
    let message_candidate_id = format!("msg_{}", identity("message-bytes", &source_sha256));
    let source_copy_id = format!(
        "copy_{}",
        identity("source-copy", &format!("{source_id}\0{source_sha256}"))
    );

    let mut objects = BTreeMap::new();
    objects.insert(
        message_candidate_id.clone(),
        Object {
            id: message_candidate_id.clone(),
            object_type: "email_message".into(),
            attributes: Vec::new(),
            relationships: vec![Relationship {
                object_id: source_copy_id.clone(),
                qualifier: "observed_as_copy".into(),
            }],
        },
    );
    objects.insert(
        source_copy_id.clone(),
        Object {
            id: source_copy_id.clone(),
            object_type: "source_copy".into(),
            attributes: Vec::new(),
            relationships: Vec::new(),
        },
    );
    let mut relationships = vec![
        Relationship {
            object_id: message_candidate_id.clone(),
            qualifier: "message".into(),
        },
        Relationship {
            object_id: source_copy_id.clone(),
            qualifier: "source_copy".into(),
        },
    ];
    for (role, addresses) in [
        ("from", parsed.from()),
        ("sender", parsed.sender()),
        ("to", parsed.to()),
        ("cc", parsed.cc()),
        ("bcc", parsed.bcc()),
        ("reply_to", parsed.reply_to()),
    ] {
        push_address_links(&mut relationships, &mut objects, role, addresses);
    }
    let mut event_attributes = vec![Attribute {
        name: "source_sha256".into(),
        value: source_sha256.clone(),
    }];
    header_claims::add_claims(
        &parsed,
        &mut event_attributes,
        &mut relationships,
        &mut objects,
    );
    let attachment_count = attachments::add_attachment_metadata(
        &parsed,
        &message_candidate_id,
        observed_at,
        &mut relationships,
        &mut objects,
    );
    event_attributes.push(Attribute {
        name: "attachment_count".into(),
        value: attachment_count.to_string(),
    });
    event_attributes.sort_by(|a, b| (&a.name, &a.value).cmp(&(&b.name, &b.value)));
    relationships.sort_by(|a, b| (&a.qualifier, &a.object_id).cmp(&(&b.qualifier, &b.object_id)));
    let event_id = format!(
        "event_{}",
        identity(
            "source-observation",
            &format!("{source_copy_id}\\0{}", observed_at.to_rfc3339())
        )
    );

    Ok(Conversion {
        schema_version: "1.2.0",
        source_sha256: source_sha256.clone(),
        message_candidate_id,
        source_copy_id,
        log: Ocel {
            object_types: [
                "claimed_message_identifier",
                "email_address",
                "email_message",
                "mime_attachment",
                "source_copy",
            ]
            .iter()
            .map(|name| OcelType {
                name: (*name).into(),
                attributes: if *name == "mime_attachment" {
                    [
                        "declared_media_type",
                        "filename_sha256",
                        "decoded_byte_length",
                    ]
                    .into_iter()
                    .map(|field| AttributeType {
                        name: field.into(),
                        value_type: "string".into(),
                    })
                    .collect()
                } else {
                    Vec::new()
                },
            })
            .collect(),
            event_types: vec![OcelType {
                name: "source_copy_observed".into(),
                attributes: {
                    let mut types = header_claims::attribute_types();
                    types.push(AttributeType {
                        name: "attachment_count".into(),
                        value_type: "string".into(),
                    });
                    types
                },
            }],
            objects: objects.into_values().collect(),
            events: vec![Event {
                id: event_id,
                event_type: "source_copy_observed".into(),
                time: observed_at,
                attributes: event_attributes,
                relationships,
            }],
        },
    })
}
