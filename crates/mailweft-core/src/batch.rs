//! Deterministic multi-message OCEL assembly. Do not turn copy counts into message counts.
use crate::{Error, Event, Object, Ocel, Relationship, convert_eml};
use chrono::{DateTime, Utc};
use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};

#[derive(Clone, Debug)]
pub struct SourceInput<'a> {
    pub raw: &'a [u8],
    pub source_id: &'a str,
    pub observed_at: DateTime<Utc>,
}

fn sort_relations(links: &mut Vec<Relationship>) {
    links.sort_by(|a, b| (&a.qualifier, &a.object_id).cmp(&(&b.qualifier, &b.object_id)));
    links.dedup();
}

/// Combine source copies into a single OCEL, preserving both source-specific
/// observations and object-to-object copy relations. Exact repeated observations
/// are idempotent; probable duplicates are not silently merged.
pub fn convert_batch(inputs: &[SourceInput<'_>]) -> Result<Ocel, Error> {
    if inputs.is_empty() {
        return Err(Error::EmptyBatch);
    }

    let mut objects: BTreeMap<String, Object> = BTreeMap::new();
    let mut events: BTreeMap<String, Event> = BTreeMap::new();
    let mut object_types = Vec::new();
    let mut event_types = Vec::new();

    for source in inputs {
        let mut log = convert_eml(source.raw, source.source_id, source.observed_at)?.log;
        if object_types.is_empty() {
            object_types = log.object_types.clone();
            event_types = log.event_types.clone();
        }
        for object in log.objects.drain(..) {
            match objects.entry(object.id.clone()) {
                Entry::Vacant(v) => {
                    v.insert(object);
                }
                Entry::Occupied(mut o) => {
                    let previous = o.get_mut();
                    if previous.object_type != object.object_type {
                        return Err(Error::IdentityCollision);
                    }
                    // Merge time-varying OCEL object attribute history from
                    // repeated, independently observed source copies.
                    for attr in object.attributes {
                        if previous.attributes.iter().any(|existing| {
                            existing.name == attr.name
                                && existing.time == attr.time
                                && existing.value != attr.value
                        }) {
                            return Err(Error::IdentityCollision);
                        }
                        if !previous.attributes.contains(&attr) {
                            previous.attributes.push(attr);
                        }
                    }
                    previous
                        .attributes
                        .sort_by(|a, b| (&a.name, &a.time).cmp(&(&b.name, &b.time)));
                    previous.relationships.extend(object.relationships);
                    sort_relations(&mut previous.relationships);
                }
            }
        }
        for event in log.events {
            match events.entry(event.id.clone()) {
                Entry::Vacant(v) => {
                    v.insert(event);
                }
                Entry::Occupied(o) => {
                    if o.get() != &event {
                        return Err(Error::IdentityCollision);
                    }
                }
            }
        }
    }

    // Header identifiers are claims. Link only an observed, uniquely matching
    // logical message, and call the O2O edge a *candidate*, not a verified reply.
    // A copied mailbox observation does not multiply thread evidence.
    // A candidate O2O edge cannot establish authorship or successful delivery.
    let mut announced_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut requested_parents: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for event in events.values() {
        let message = event
            .relationships
            .iter()
            .find(|r| r.qualifier == "message")
            .map(|r| r.object_id.clone());
        if let Some(message) = message {
            for relationship in &event.relationships {
                match relationship.qualifier.as_str() {
                    "claimed_message_id" => {
                        announced_ids
                            .entry(relationship.object_id.clone())
                            .or_default()
                            .insert(message.clone());
                    }
                    "claimed_in_reply_to" => {
                        requested_parents
                            .entry(message.clone())
                            .or_default()
                            .insert(relationship.object_id.clone());
                    }
                    _ => {}
                }
            }
        }
    }
    for (child_id, references) in requested_parents {
        if references.len() != 1 {
            continue;
        }
        let Some(reference) = references.iter().next() else {
            continue;
        };
        let Some(parents) = announced_ids.get(reference) else {
            continue;
        };
        if parents.len() != 1 {
            continue;
        }
        let Some(parent_id) = parents.iter().next() else {
            continue;
        };
        if &child_id == parent_id {
            continue;
        }
        if let Some(child) = objects.get_mut(&child_id) {
            child.relationships.push(Relationship {
                object_id: parent_id.clone(),
                qualifier: "reply_target_candidate".into(),
            });
            sort_relations(&mut child.relationships);
        }
    }

    Ok(Ocel {
        object_types,
        event_types,
        objects: objects.into_values().collect(),
        events: events.into_values().collect(),
    })
}
