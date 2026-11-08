use crate::identity::sha256;
use crate::{Config, Conversion, Error, Permission, RawEmail, Receipt};
use std::collections::BTreeMap;

/// All-or-nothing conversion of a bounded batch. No partial log is returned on error.
/// Source observation timestamps must be stable caller metadata, not filesystem mtimes.
pub fn convert(inputs: &[RawEmail], c: &Config) -> Result<Conversion, Error> {
    if c.scope.trim().is_empty() || c.scope.len() > 256 {
        return Err(Error("INVALID_SCOPE"));
    }
    if c.max_message_bytes == 0
        || c.max_total_bytes == 0
        || c.max_messages == 0
        || c.max_parts == 0
        || c.max_depth == 0
    {
        return Err(Error("INVALID_LIMIT"));
    }
    if inputs.is_empty() {
        return Err(Error("EMPTY_INPUT"));
    }
    if inputs.len() > c.max_messages {
        return Err(Error("TOO_MANY_MESSAGES"));
    }
    let total = inputs
        .iter()
        .try_fold(0usize, |sum, i| sum.checked_add(i.bytes.len()))
        .ok_or(Error("BATCH_TOO_LARGE"))?;
    if total > c.max_total_bytes {
        return Err(Error("BATCH_TOO_LARGE"));
    }
    let mut copies = BTreeMap::new();
    let mut refs = BTreeMap::new();
    let mut duplicates = 0;
    for input in inputs {
        let message = crate::parser::parse(input, c)?;
        let fingerprint = (
            message.source_sha256.clone(),
            message.observed_at,
            message.source_kind.clone(),
        );
        let ref_id = c.id("source", &[input.source_ref.as_bytes()]);
        if let Some(previous) = refs.insert(ref_id, fingerprint.clone())
            && previous != fingerprint
        {
            return Err(Error("SOURCE_CONFLICT"));
        }
        if copies
            .insert(message.source_copy_id.clone(), message)
            .is_some()
        {
            duplicates += 1;
        }
    }
    let mut messages: Vec<_> = copies.into_values().collect();
    messages.sort_by(|a, b| {
        a.observed_at
            .cmp(&b.observed_at)
            .then(a.source_copy_id.cmp(&b.source_copy_id))
    });
    let ocel = crate::projection::project(&messages, c);
    let config_value = serde_json::json!({"version": "mailweft-projection-v1", "scope_id": c.id("scope", &[]),
        "key_id": c.id("key", &[]), "view": c.view, "max_message_bytes": c.max_message_bytes,
        "max_total_bytes": c.max_total_bytes, "max_messages": c.max_messages,
        "max_parts": c.max_parts, "max_depth": c.max_depth});
    let config_sha256 =
        sha256(&serde_json::to_vec(&config_value).map_err(|_| Error("SERIALIZATION_FAILED"))?);
    let result_hash = sha256(
        &serde_json::to_vec(&(&ocel, &messages)).map_err(|_| Error("SERIALIZATION_FAILED"))?,
    );
    let rights = if inputs.iter().all(|i| i.permission == Permission::Synthetic) {
        "synthetic"
    } else {
        "authorised"
    };
    let lossy = messages.iter().any(|m| {
        m.parse_warnings.iter().any(|w| {
            matches!(
                w.as_str(),
                "ADDRESS_GROUP_STRUCTURE_NOT_PROJECTED"
                    | "ADDRESS_HEADER_UNRESOLVED"
                    | "THREAD_RESOLUTION_NOT_IMPLEMENTED"
                    | "RESENT_HEADERS_NOT_PROJECTED"
                    | "NESTED_MESSAGE_NOT_PROJECTED"
            )
        })
    });
    let receipt = Receipt {
        schema_version: "1.0.0".into(), run_id: c.id("run", &[config_sha256.as_bytes(), result_hash.as_bytes(), &(inputs.len() as u64).to_be_bytes()]),
        source_id: c.id("scope", &[]), source_version: "caller-supplied-raw-v1".into(),
        source_rights_status: rights.into(), input_count: inputs.len(), parsed_count: inputs.len(),
        rejected_count: 0, duplicate_copy_count: duplicates, output_count: messages.len(),
        completeness: if lossy { "partial" } else { "complete_for_admitted_scope" }.into(),
        config_sha256, producer_version: env!("CARGO_PKG_VERSION").into(), result_hash,
        notes: vec!["All-or-nothing batch; output_count counts unique source copies.".into(),
            "Completeness covers supplied input, not the original mailbox or real-world workflow.".into(),
            "Header dates and participants are assertions, not delivery, person or responsibility verification.".into(),
            "Pseudonymous local output is not anonymous or automatically safe for publication.".into()],
    };
    Ok(Conversion {
        ocel,
        messages,
        receipt,
    })
}
