use crate::{Config, EventView, ObservedMessage, OCEL};
use process_mining::core::event_data::object_centric::{OCELType, OCELTypeAttribute,
    OCELObject, OCELObjectAttribute, OCELRelationship, OCELAttributeValue, OCELEvent, OCELEventAttribute};
use std::collections::BTreeMap;

fn attribute(name: &str, value: &str) -> OCELEventAttribute {
    OCELEventAttribute { name: name.into(), value: OCELAttributeValue::String(value.into()) }
}
fn kind(name: &str, attributes: &[(&str, &str)]) -> OCELType {
    OCELType { name: name.into(), attributes: attributes.iter().map(|(n,t)| OCELTypeAttribute {
        name: (*n).into(), value_type: (*t).into() }).collect() }
}
fn relationships(m: &ObservedMessage) -> Vec<OCELRelationship> {
    let mut edges = vec![OCELRelationship::new(&m.message_candidate_id, "message")];
    edges.extend(m.addresses.iter().map(|p| OCELRelationship::new(&p.party_id, format!("{}_asserted", p.role))));
    edges.extend(m.attachments.iter().map(|a| OCELRelationship::new(&a.part_id, "contains_mime_part")));
    edges
}

pub(crate) fn project(messages: &[ObservedMessage], c: &Config) -> OCEL {
    let mut objects = BTreeMap::new();
    let mut events = BTreeMap::<String, OCELEvent>::new();
    for m in messages {
        let mut message_links: Vec<_> = m.addresses.iter().map(|p| OCELRelationship::new(&p.party_id, format!("{}_asserted", p.role))).collect();
        message_links.extend(m.attachments.iter().map(|a| OCELRelationship::new(&a.part_id, "contains_mime_part")));
        objects.entry(m.message_candidate_id.clone()).or_insert_with(|| OCELObject {
            id: m.message_candidate_id.clone(), object_type: "Message".into(),
            attributes: vec![OCELObjectAttribute::new("raw_sha256", m.source_sha256.clone(), m.observed_at)],
            relationships: message_links,
        });
        objects.insert(m.source_copy_id.clone(), OCELObject {
            id: m.source_copy_id.clone(), object_type: "SourceCopy".into(),
            attributes: vec![OCELObjectAttribute::new("source_kind", m.source_kind.clone(), m.observed_at)],
            relationships: vec![OCELRelationship::new(&m.message_candidate_id, "copy_of")],
        });
        for party in &m.addresses {
            objects.entry(party.party_id.clone()).or_insert_with(|| OCELObject {
                id: party.party_id.clone(), object_type: "MailboxIdentity".into(),
                attributes: vec![OCELObjectAttribute::new("identity_state", party.identity_state.clone(), m.observed_at)],
                relationships: Vec::new(),
            });
        }
        for a in &m.attachments {
            objects.entry(a.part_id.clone()).or_insert_with(|| OCELObject {
                id: a.part_id.clone(), object_type: "Attachment".into(),
                attributes: vec![OCELObjectAttribute::new("mime_type", a.mime_type.clone(), m.observed_at),
                    OCELObjectAttribute::new("size_bytes", a.size_bytes as i64, m.observed_at)],
                relationships: vec![OCELRelationship::new(&m.message_candidate_id, "attachment_of")],
            });
        }
        if matches!(c.view, EventView::Observed | EventView::Both) {
            let id = c.id("event", &[b"observed", m.source_copy_id.as_bytes()]);
            let mut edges = relationships(m);
            edges.push(OCELRelationship::new(&m.source_copy_id, "archived_copy"));
            events.insert(id.clone(), OCELEvent::new(id, "message_archived_observed", m.observed_at,
                vec![attribute("evidence_state", "observed_source_copy")], edges));
        }
        if matches!(c.view, EventView::HeaderAsserted | EventView::Both) && let Some(time) = m.header_date {
            let id = c.id("event", &[b"header_asserted", m.message_candidate_id.as_bytes()]);
            let event = events.entry(id.clone()).or_insert_with(|| OCELEvent::new(id,
                "message_header_date_asserted", time,
                vec![attribute("evidence_state", "asserted_by_header")], relationships(m)));
            event.relationships.push(OCELRelationship::new(&m.source_copy_id, "evidenced_by_copy"));
        }
    }
    let mut log = OCEL {
        event_types: vec![kind("message_archived_observed", &[("evidence_state", "string")]),
            kind("message_header_date_asserted", &[("evidence_state", "string")])],
        object_types: vec![kind("Message", &[("raw_sha256", "string")]),
            kind("SourceCopy", &[("source_kind", "string")]),
            kind("MailboxIdentity", &[("identity_state", "string")]),
            kind("Attachment", &[("mime_type", "string"), ("size_bytes", "integer")])],
        events: events.into_values().collect(), objects: objects.into_values().collect(),
    };
    log.events.sort_by(|a,b| a.time.cmp(&b.time).then(a.id.cmp(&b.id)));
    for event in &mut log.events { event.relationships.sort_by(|a,b| (&a.object_id, &a.qualifier).cmp(&(&b.object_id, &b.qualifier))); }
    for object in &mut log.objects { object.relationships.sort_by(|a,b| (&a.object_id, &a.qualifier).cmp(&(&b.object_id, &b.qualifier))); }
    log
}
