//! Reconciliation of an explicitly supplied selection, not proof of complete correspondence.
use crate::{Ocel, SourceInput, convert_batch, digest, identity};
use crate::source::{SourceEnvelope, SourceError, SourceReceipt, convert_with_envelope};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const MAX_SELECTION_ITEMS: usize = 1024;
pub const MAX_SELECTION_BYTES: usize = 256 * 1024 * 1024;

/// A caller-supplied selection slot, including an explicit acquisition/read failure.
/// Private entry IDs and metadata must not be logged. Acquisition stays outside this API.
pub struct SubmittedSource {
    pub entry_id: String,
    pub source: Result<(Vec<u8>, SourceEnvelope), SourceError>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectionOutcome {
    pub entry_sha256: String,
    pub status: String,
    pub diagnostic: Option<String>,
    pub receipt: Option<SourceReceipt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectionManifest {
    pub schema_version: String,
    pub supplied: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub unique_observations: usize,
    pub projection_status: String,
    pub coverage_assurance: String,
    pub oc​​el_sha256: Option<String>,
    pub entries: Vec<SelectionOutcome>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconciledSelection {
    pub manifest: SelectionManifest,
    pub manifest_sha256: String,
    pub log: Option<Ocel>,
}

#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq, Eq)]
pub enum SelectionError {
    #[error("SELECTION_INVALID")]
    Invalid,
    #[error("SELECTION_SIZE_LIMIT")]
    SizeLimit,
    #[error("SELECTION_PROJECTION_CONFLICT")]
    ProjectionConflict,
    #[error("SELECTION_SERIALIZATION_FAILED")]
    Serialization,
}

pub fn valid_entry_id(id: &str) -> bool {
    !id.trim().is_empty() && id.len() <= 4096 && !id.chars().any(char::is_control)
}

/// Every supplied slot has one accepted/rejected outcome. Rejected slots never become events.
/// Partial projection requires explicit opt-in; receipts and outcomes survive withholding.
pub fn reconcile_selection(
    mut items: Vec<SubmittedSource>,
    allow_partial: bool,
) -> Result<ReconciledSelection, SelectionError> {
    if items.is_empty() || items.len() > MAX_SELECTION_ITEMS {
        return Err(SelectionError::Invalid);
    }
    items.sort_by(|a, b| a.entry_id.cmp(&b.entry_id));
    if items.iter().any(|s| !valid_entry_id(&s.entry_id))
        || items.windows(2).any(|w| w[0].entry_id == w[1].entry_id)
    {
        return Err(SelectionError::Invalid);
    }
    let mut total = 0usize;
    for item in &items {
        if let Ok((bytes, _)) = &item.source {
            total = total.checked_add(bytes.len()).ok_or(SelectionError::SizeLimit)?;
        }
    }
    if total > MAX_SELECTION_BYTES { return Err(SelectionError::SizeLimit); }
    let supplied = items.len();
    let mut accepted_data = Vec::new();
    let mut entries = Vec::with_capacity(supplied);
    let mut observation_ids = BTreeSet::new();
    for item in items {
        let entry_sha256 = identity("selection-entry", &item.entry_id);
        let failure = match item.source {
            Err(error) => Some(error),
            Ok((bytes, envelope)) => match convert_with_envelope(&bytes, &envelope) {
                Err(error) => Some(error),
                Ok(result) => {
                    observation_ids.insert(result.receipt.observation_event_id.clone());
                    entries.push(SelectionOutcome {
                        entry_sha256: entry_sha256.clone(), status: "accepted".into(),
                        diagnostic: None, receipt: Some(result.receipt),
                    });
                    accepted_data.push((bytes, envelope));
                    None
                }
            },
        };
        if let Some(error) = failure {
            entries.push(SelectionOutcome {
                entry_sha256, status: "rejected".into(), diagnostic: Some(error.code().into()), receipt: None,
            });
        }
    }
    entries.sort_by(|a, b| a.entry_sha256.cmp(&b.entry_sha256));
    let accepted = accepted_data.len();
    let rejected = supplied - accepted;
    let project = accepted > 0 && (rejected == 0 || allow_partial);
    let log = if project {
        let inputs: Vec<_> = accepted_data.iter().map(|(raw, envelope)| SourceInput {
            raw, source_id: &envelope.source_id, observed_at: envelope.observed_at,
        }).collect();
        Some(convert_batch(&inputs).map_err(|_| SelectionError::ProjectionConflict)?)
    } else { None };
    let ocel_hash = log.as_ref().map(|value| serde_json::to_vec(value).map(|bytes| digest(&bytes)))
        .transpose().map_err(|_| SelectionError::Serialization)?;
    let manifest = SelectionManifest {
        schema_version: "1.0.0".into(), supplied, accepted, rejected,
        unique_observations: observation_ids.len(),
        projection_status: if !project { "withheld" } else if rejected == 0 { "complete_selection" } else { "partial" }.into(),
        coverage_assurance: "supplied_selection_only".into(),
        oc​​el_sha256: ocel_hash, entries,
    };
    let manifest_sha256 = digest(&serde_json::to_vec(&manifest).map_err(|_| SelectionError::Serialization)?);
    Ok(ReconciledSelection { manifest, manifest_sha256, log })
}
