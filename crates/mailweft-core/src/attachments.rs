//! Safe metadata-only MIME attachment projection.
//!
//! The parser owns MIME interpretation; this module never decodes, executes,
//! renders, follows external links or writes attachment payloads.
use crate::{Object, ObjectAttribute, Relationship, identity};
use chrono::{DateTime, Utc};
use mail_parser::Message;
use std::collections::BTreeMap;

fn add_attribute(attrs: &mut Vec<ObjectAttribute>, name: &str, value: String, at: DateTime<Utc>) {
    attrs.push(ObjectAttribute {
        name: name.to_owned(),
        time: at,
        value,
    });
}

/// Returns the number of MIME parts classified as attachments by the parser.
/// These are observed *parts of supplied source bytes*, not proven delivered documents.
/// Declared MIME types are not content verification, and filenames stay hashed.
pub(crate) fn add_attachment_metadata(
    message: &Message<'_>,
    message_candidate_id: &str,
    observed_at: DateTime<Utc>,
    event_links: &mut Vec<Relationship>,
    objects: &mut BTreeMap<String, Object>,
) -> usize {
    let mut count = 0usize;
    for part in message.attachments() {
        let part_id = part.id();
        let attachment_id = format!(
            "attachment_{}",
            identity(
                "mime-attachment-part",
                &format!("{message_candidate_id}\\0{part_id}")
            )
        );
        let mut attrs = Vec::new();
        if let Some(content_type) = part.content_type() {
            if let Some(subtype) = content_type.subtype() {
                let declared = format!("{}/{}", content_type.ctype(), subtype).to_ascii_lowercase();
                add_attribute(&mut attrs, "declared_media_type", declared, observed_at);
            }
        }
        if let Some(filename) = part.attachment_name().filter(|name| !name.is_empty()) {
            let digest = identity("mime-filename", filename);
            add_attribute(&mut attrs, "filename_sha256", digest, observed_at);
        }
        add_attribute(
            &mut attrs,
            "decoded_byte_length",
            part.decoded_len().to_string(),
            observed_at,
        );
        attrs.sort_by(|a, b| (&a.name, &a.time, &a.value).cmp(&(&b.name, &b.time, &b.value)));
        objects.insert(
            attachment_id.clone(),
            Object {
                id: attachment_id.clone(),
                object_type: "mime_attachment".into(),
                attributes: attrs,
                relationships: Vec::new(),
            },
        );
        if let Some(email) = objects.get_mut(message_candidate_id) {
            email.relationships.push(Relationship {
                object_id: attachment_id.clone(),
                qualifier: "contains_attachment_part".into(),
            });
        }
        event_links.push(Relationship {
            object_id: attachment_id,
            qualifier: "attachment_part".into(),
        });
        count += 1;
    }
    count
}
