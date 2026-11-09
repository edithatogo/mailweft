//! Supplied-source integrity contracts. No acquisition, authorisation or attestation.
use crate::{Conversion, MAX_EML_BYTES, convert_eml, digest, identity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, io::Read, path::Path};

pub const MAX_ENVELOPE_BYTES: usize = 64 * 1024;
pub const MAX_SOURCE_BYTES: usize = MAX_EML_BYTES;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceFidelity {
    RawRfc5322,
    PlatformExport,
    RenderedHtml,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoverageClaim {
    Unknown,
    Partial,
    CompleteSuppliedSelection,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceProducer {
    pub name: String,
    pub version: String,
}

/// Private caller-supplied metadata. References are identifiers, never URLs to fetch.
/// Presence of rights/scope references is not verification of permission.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceEnvelope {
    pub schema_version: String,
    pub source_id: String,
    pub source_sha256: String,
    pub byte_length: u64,
    pub observed_at: DateTime<Utc>,
    pub fidelity: SourceFidelity,
    pub coverage_claim: CoverageClaim,
    pub producer: SourceProducer,
    pub rights_ref: String,
    pub scope_ref: String,
    pub custodian_ref: Option<String>,
    pub locator_ref: Option<String>,
    #[serde(default)]
    pub transformation_refs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceReceipt {
    pub schema_version: String,
    pub source_sha256: String,
    pub byte_length: u64,
    pub observed_at: DateTime<Utc>,
    pub source_copy_id: String,
    pub message_candidate_id: String,
    pub observation_event_id: String,
    pub metadata_sha256: String,
    pub producer_sha256: String,
    pub rights_ref_sha256: String,
    pub scope_ref_sha256: String,
    pub fidelity: SourceFidelity,
    pub coverage_claim: CoverageClaim,
    pub converter_version: String,
    pub parser_version: String,
    pub assurance: String,
}

#[derive(Debug, Serialize)]
pub struct ProvenancedConversion {
    pub conversion: Conversion,
    pub receipt: SourceReceipt,
}

/// Diagnostic codes contain no attacker-controlled filenames, addresses or JSON values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceError {
    InvalidEnvelope,
    UnsupportedVersion,
    UnsupportedFidelity,
    LengthMismatch,
    HashMismatch,
    SizeLimit,
    Empty,
    NotRegular,
    ReadFailed,
    ConversionFailed,
}

impl SourceError {
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidEnvelope => "SOURCE_ENVELOPE_INVALID",
            Self::UnsupportedVersion => "SOURCE_VERSION_UNSUPPORTED",
            Self::UnsupportedFidelity => "SOURCE_FIDELITY_UNSUPPORTED",
            Self::LengthMismatch => "SOURCE_LENGTH_MISMATCH",
            Self::HashMismatch => "SOURCE_HASH_MISMATCH",
            Self::SizeLimit => "SOURCE_SIZE_LIMIT",
            Self::Empty => "SOURCE_EMPTY",
            Self::NotRegular => "SOURCE_NOT_REGULAR",
            Self::ReadFailed => "SOURCE_READ_FAILED",
            Self::ConversionFailed => "SOURCE_CONVERSION_FAILED",
        }
    }
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for SourceError {}

fn valid_ref(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 4096 && !value.chars().any(char::is_control)
}

impl SourceEnvelope {
    /// Validate structure and bounded references; no permission or completeness is inferred.
    pub fn validate(&self) -> Result<(), SourceError> {
        if self.schema_version != "1.0.0" {
            return Err(SourceError::UnsupportedVersion);
        }
        if ![
            &self.source_id,
            &self.producer.name,
            &self.producer.version,
            &self.rights_ref,
            &self.scope_ref,
        ]
        .into_iter()
        .all(|v| valid_ref(v))
            || self.custodian_ref.as_deref().is_some_and(|v| !valid_ref(v))
            || self.locator_ref.as_deref().is_some_and(|v| !valid_ref(v))
            || self.transformation_refs.len() > 64
            || self.transformation_refs.iter().any(|v| !valid_ref(v))
            || self.source_sha256.len() != 64
            || !self
                .source_sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(SourceError::InvalidEnvelope);
        }
        if self.byte_length == 0 || self.byte_length > MAX_SOURCE_BYTES as u64 {
            return Err(SourceError::SizeLimit);
        }
        Ok(())
    }
}

pub fn parse_envelope_json(raw: &[u8]) -> Result<SourceEnvelope, SourceError> {
    if raw.len() > MAX_ENVELOPE_BYTES {
        return Err(SourceError::SizeLimit);
    }
    let envelope: SourceEnvelope =
        serde_json::from_slice(raw).map_err(|_| SourceError::InvalidEnvelope)?;
    envelope.validate()?;
    Ok(envelope)
}

/// Verify actual supplied bytes before projecting them. Metadata remains caller-asserted.
pub fn convert_with_envelope(
    raw: &[u8],
    envelope: &SourceEnvelope,
) -> Result<ProvenancedConversion, SourceError> {
    envelope.validate()?;
    if envelope.fidelity != SourceFidelity::RawRfc5322 {
        return Err(SourceError::UnsupportedFidelity);
    }
    if raw.len() as u64 != envelope.byte_length {
        return Err(SourceError::LengthMismatch);
    }
    if digest(raw) != envelope.source_sha256 {
        return Err(SourceError::HashMismatch);
    }
    let conversion = convert_eml(raw, &envelope.source_id, envelope.observed_at)
        .map_err(|_| SourceError::ConversionFailed)?;
    // Serde struct field order and UTC datetime normalization define this version's
    // semantic metadata fingerprint. This is not a generic JSON canonicalizer.
    let metadata = serde_json::to_vec(envelope).map_err(|_| SourceError::InvalidEnvelope)?;
    let producer =
        serde_json::to_vec(&envelope.producer).map_err(|_| SourceError::InvalidEnvelope)?;
    let receipt = SourceReceipt {
        schema_version: "1.0.0".into(),
        source_sha256: envelope.source_sha256.clone(),
        byte_length: envelope.byte_length,
        observed_at: envelope.observed_at,
        source_copy_id: conversion.source_copy_id.clone(),
        message_candidate_id: conversion.message_candidate_id.clone(),
        observation_event_id: conversion.log.events[0].id.clone(),
        metadata_sha256: digest(&metadata),
        producer_sha256: digest(&producer),
        rights_ref_sha256: identity("source-rights-ref", &envelope.rights_ref),
        scope_ref_sha256: identity("source-scope-ref", &envelope.scope_ref),
        fidelity: envelope.fidelity,
        coverage_claim: envelope.coverage_claim,
        converter_version: env!("CARGO_PKG_VERSION").into(),
        parser_version: "mail-parser/1.0.1".into(),
        assurance: "bytes_matched_metadata_caller_asserted".into(),
    };
    Ok(ProvenancedConversion {
        conversion,
        receipt,
    })
}

/// Read at most limit+1 bytes, including when input length metadata is absent or stale.
/// No partial bytes are returned for oversized or failed reads.
pub fn read_bounded(reader: impl Read, limit: usize) -> Result<Vec<u8>, SourceError> {
    if limit == 0 || limit > MAX_SOURCE_BYTES {
        return Err(SourceError::SizeLimit);
    }
    let mut out = Vec::with_capacity(limit.min(8192));
    reader
        .take(limit as u64 + 1)
        .read_to_end(&mut out)
        .map_err(|_| SourceError::ReadFailed)?;
    if out.len() > limit {
        return Err(SourceError::SizeLimit);
    }
    if out.is_empty() {
        return Err(SourceError::Empty);
    }
    Ok(out)
}

/// Explicit local files only; rejects visible symlinks and non-regular entries.
/// This is not a filesystem sandbox or a guarantee against hostile directory races.
pub fn read_bounded_file(path: &Path, limit: usize) -> Result<Vec<u8>, SourceError> {
    let before = fs::symlink_metadata(path).map_err(|_| SourceError::ReadFailed)?;
    if !before.file_type().is_file() {
        return Err(SourceError::NotRegular);
    }
    if before.len() > limit as u64 {
        return Err(SourceError::SizeLimit);
    }
    let file = fs::File::open(path).map_err(|_| SourceError::ReadFailed)?;
    let opened = file.metadata().map_err(|_| SourceError::ReadFailed)?;
    if !opened.is_file() {
        return Err(SourceError::NotRegular);
    }
    read_bounded(file, limit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, io, rc::Rc};

    struct InfiniteReader(Rc<Cell<usize>>);
    impl Read for InfiniteReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf.fill(0);
            self.0.set(self.0.get() + buf.len());
            Ok(buf.len())
        }
    }

    #[test]
    fn growing_input_never_reads_beyond_limit_plus_one() {
        let count = Rc::new(Cell::new(0));
        let result = read_bounded(InfiniteReader(count.clone()), 1024);
        assert_eq!(result, Err(SourceError::SizeLimit));
        assert_eq!(count.get(), 1025);
    }

    #[test]
    fn exact_limit_succeeds_and_empty_or_oversize_fails() {
        assert_eq!(read_bounded(&b"abcd"[..], 4).unwrap(), b"abcd");
        assert_eq!(read_bounded(&b"abcde"[..], 4), Err(SourceError::SizeLimit));
        assert_eq!(read_bounded(&b""[..], 4), Err(SourceError::Empty));
        assert_eq!(read_bounded(&b"a"[..], 0), Err(SourceError::SizeLimit));
    }

    #[test]
    fn read_error_never_returns_partial_content() {
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("private-path-must-not-escape"))
            }
        }
        assert_eq!(read_bounded(Broken, 16), Err(SourceError::ReadFailed));
    }
}
