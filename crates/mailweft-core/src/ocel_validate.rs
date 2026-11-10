//! Validate structural OCEL 2.0 integrity without silently repairing the log.
//! This is an additional invariant gate, not full normative schema certification.
use crate::Ocel;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OcelViolation {
    pub code: &'static str,
    pub location: String,
}

fn fail(errors: &mut Vec<OcelViolation>, code: &'static str, location: impl Into<String>) {
    errors.push(OcelViolation { code, location: location.into() });
}

pub fn validate_ocel(log: &Ocel) -> Result<(), Vec<OcelViolation>> {
    let mut errors = Vec::new();
    let mut object_types = BTreeMap::new();
    let mut event_types = BTreeMap::new();
    for (label, types, output) in [
        ("objectTypes", &log.object_types, &mut object_types),
        ("eventTypes", &log.event_types, &mut event_types),
    ] {
        for item in types {
            if item.name.is_empty() || output.contains_key(&item.name) {
                fail(&mut errors, "TYPE_DUPLICATE_OR_EMPTY", format!("{label}/{}", item.name));
            }
            let mut names = BTreeSet::new();
            for attribute in &item.attributes {
                if attribute.name.is_empty() || !names.insert(attribute.name.clone()) {
                    fail(&mut errors, "ATTRIBUTE_DEFINITION_DUPLICATE", format!("{label}/{}/{}", item.name, attribute.name));
                }
                if attribute.value_type != "string" {
                    fail(&mut errors, "ATTRIBUTE_TYPE_UNSUPPORTED", format!("{label}/{}/{}", item.name, attribute.name));
                }
            }
            output.insert(item.name.clone(), names);
        }
    }
    let mut ids = BTreeSet::new();
    for object in &log.objects {
        if object.id.is_empty() || !ids.insert(object.id.clone()) {
            fail(&mut errors, "OBJECT_ID_DUPLICATE_OR_EMPTY", object.id.clone());
        }
    }
    for object in &log.objects {
        let allowed = object_types.get(&object.object_type);
        if allowed.is_none() {
            fail(&mut errors, "OBJECT_TYPE_UNDECLARED", object.id.clone());
        }
        let mut seen = BTreeMap::new();
        for attr in &object.attributes {
            if !allowed.is_some_and(|a| a.contains(&attr.name)) {
                fail(&mut errors, "OBJECT_ATTRIBUTE_UNDECLARED", format!("{}/{}", object.id, attr.name));
            }
            let key = (attr.name.clone(), attr.time);
            if let Some(previous) = seen.insert(key, &attr.value) {
                if previous != &attr.value {
                    fail(&mut errors, "OBJECT_ATTRIBUTE_CONFLICT", format!("{}/{}", object.id, attr.name));
                } else {
                    fail(&mut errors, "OBJECT_ATTRIBUTE_DUPLICATE", format!("{}/{}", object.id, attr.name));
                }
            }
        }
        check_links(&mut errors, &ids, &object.id, &object.relationships);
    }
    let mut event_ids = BTreeSet::new();
    for event in &log.events {
        if event.id.is_empty() || !event_ids.insert(event.id.clone()) {
            fail(&mut errors, "EVENT_ID_DUPLICATE_OR_EMPTY", event.id.clone());
        }
        let allowed = event_types.get(&event.event_type);
        if allowed.is_none() {
            fail(&mut errors, "EVENT_TYPE_UNDECLARED", event.id.clone());
        }
        let mut seen = BTreeSet::new();
        for attr in &event.attributes {
            if !allowed.is_some_and(|a| a.contains(&attr.name)) {
                fail(&mut errors, "EVENT_ATTRIBUTE_UNDECLARED", format!("{}/{}", event.id, attr.name));
            }
            if !seen.insert(attr.name.as_str()) {
                fail(&mut errors, "EVENT_ATTRIBUTE_DUPLICATE", format!("{}/{}", event.id, attr.name));
            }
        }
        check_links(&mut errors, &ids, &event.id, &event.relationships);
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn check_links(errors: &mut Vec<OcelViolation>, ids: &BTreeSet<String>, owner: &str, links: &[crate::Relationship]) {
    let mut seen = BTreeSet::new();
    for link in links {
        if !ids.contains(&link.object_id) {
            fail(errors, "RELATIONSHIP_TARGET_MISSING", format!("{owner}/{}", link.object_id));
        }
        if link.qualifier.is_empty() {
            fail(errors, "RELATIONSHIP_QUALIFIER_EMPTY", format!("{owner}/{}", link.object_id));
        }
        if !seen.insert((&link.object_id, &link.qualifier)) {
            fail(errors, "RELATIONSHIP_DUPLICATE", format!("{owner}/{}", link.object_id));
        }
    }
}
