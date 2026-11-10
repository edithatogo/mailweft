//! Machine-readable capability/loss ledger for the Stage 1 OCEL projection.
//! Reports representational limitations without asserting that source content
//! was actually lost, or silently converting unsupported OCEL value types.
use crate::Ocel;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct LossEntry {
    pub code: &'static str,
    pub severity: &'static str,
    pub scope: &'static str,
    pub explanation: &'static str,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct OcelCapabilityReport {
    pub schema_version: &'static str,
    pub ocel_version: &'static str,
    pub projection: &'static str,
    pub object_count: usize,
    pub event_count: usize,
    pub validation_passed: bool,
    pub validation_codes: Vec<&'static str>,
    pub loss_entries: Vec<LossEntry>,
}

pub fn capability_report(log: &Ocel) -> OcelCapabilityReport {
    let errors = crate::ocel_validate::validate_ocel(log).err().unwrap_or_default();
    let mut validation_codes: Vec<_> = errors.into_iter().map(|e| e.code).collect();
    validation_codes.sort_unstable();
    validation_codes.dedup();
    OcelCapabilityReport {
        schema_version: "1.0.0",
        ocel_version: "2.0",
        projection: "stage1_source_observation",
        object_count: log.objects.len(),
        event_count: log.events.len(),
        validation_passed: validation_codes.is_empty(),
        validation_codes,
        loss_entries: vec![
            LossEntry {
                code: "SOURCE_CONTENT_EXTERNAL",
                severity: "information",
                scope: "source",
                explanation: "Body, original headers and attachment bytes remain in the external source; the OCEL projection is not a source archive.",
            },
            LossEntry {
                code: "CLAIMED_TIMELINE_NOT_OBSERVED",
                severity: "information",
                scope: "time",
                explanation: "Claimed RFC Date is an attribute, not independent evidence of sending or delivery.",
            },
            LossEntry {
                code: "STRING_ATTRIBUTE_PROFILE",
                severity: "limitation",
                scope: "attributes",
                explanation: "The current Rust OCEL representation emits string values; native integer, float, boolean and temporal attribute value parity is not supported.",
            },
            LossEntry {
                code: "FORMAT_ROUNDTRIP_UNQUALIFIED",
                severity: "limitation",
                scope: "interop",
                explanation: "OCEL 2.0 JSON is qualified for import and structural checks; XML and SQLite equivalence and full independent export roundtrips are unqualified.",
            },
            LossEntry {
                code: "ACTOR_AND_CASE_INFERENCE_EXCLUDED",
                severity: "information",
                scope: "semantics",
                explanation: "Mailbox roles, claimed references and candidate replies do not establish people, case identity, delivery or business actions.",
            },
        ],
    }
}
