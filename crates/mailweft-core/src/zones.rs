//! Optional source-anchored, non-destructive plaintext zoning.
//!
//! Derived annotations never alter the authoritative OCEL observation. Values
//! retain hashes and offsets, not plaintext or untrusted message instructions.
use crate::{Error, digest};
use mail_parser::{MessageParser, PartKind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OffsetQuality {
    /// Each decoded span matches the same bytes in the original EML source.
    Exact,
    /// Transfer/charset decoding changed bytes: a raw part is known but
    /// there is no verified span-level correspondence.
    Derived,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ZoneKind {
    Authored,
    Salutation,
    Quotation,
    ForwardedMarker,
    Signoff,
    Signature,
    Disclaimer,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceZone {
    pub kind: ZoneKind,
    pub decoded_start: usize,
    pub decoded_end: usize,
    pub raw_start: Option<usize>,
    pub raw_end: Option<usize>,
    pub offset_quality: OffsetQuality,
    pub sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextPartEvidence {
    pub part_id: u32,
    pub raw_start: usize,
    pub raw_end: usize,
    pub raw_sha256: String,
    pub decoded_sha256: String,
    pub decoded_byte_length: usize,
    pub decoding_problems: bool,
    pub zones: Vec<SourceZone>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BodyEvidence {
    pub schema_version: &'static str,
    pub source_sha256: String,
    pub parts: Vec<TextPartEvidence>,
    pub skipped_html_parts: usize,
}

fn classify(line: &str, first_authored: bool, after_signoff: bool) -> ZoneKind {
    let trimmed = line.trim();
    let lower = trimmed.to_ascii_lowercase();
    if trimmed.is_empty() {
        return ZoneKind::Unknown;
    }
    if trimmed.starts_with('>') {
        return ZoneKind::Quotation;
    }
    if lower.contains("forwarded message")
        && (lower.starts_with("begin ") || lower.starts_with('-'))
    {
        return ZoneKind::ForwardedMarker;
    }
    if trimmed == "-- " || trimmed == "--" {
        return ZoneKind::Signoff;
    }
    if after_signoff {
        return ZoneKind::Signature;
    }
    if first_authored
        && (lower.starts_with("hi ") || lower.starts_with("hello ") || lower.starts_with("dear "))
        && (trimmed.ends_with(',') || trimmed.ends_with(':'))
    {
        return ZoneKind::Salutation;
    }
    if matches!(
        lower.as_str(),
        "regards,"
            | "kind regards,"
            | "best regards,"
            | "sincerely,"
            | "many thanks,"
            | "thanks,"
            | "yours sincerely,"
    ) {
        return ZoneKind::Signoff;
    }
    if lower.starts_with("this email and any attachments")
        || lower.starts_with("this message is confidential")
    {
        return ZoneKind::Disclaimer;
    }
    ZoneKind::Authored
}

/// Build a private, body-free sidecar from raw RFC5322 input. No HTML conversion
/// or remote content retrieval is attempted; HTML parts are explicitly skipped.
/// Unsupported character/transfer decoding yields derived offsets, never exact
/// offsets fabricated from Unicode-text lengths.
pub fn inspect_body_zones(raw: &[u8]) -> Result<BodyEvidence, Error> {
    if raw.is_empty() || raw.len() > 16 * 1024 * 1024 {
        return Err(Error::InvalidSource);
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
    let mut parts = Vec::new();
    let skipped_html_parts = parsed.html_body().len();
    for part in parsed.text_body() {
        // The parser may expose an HTML-only fallback in its text-body list.
        // Do not claim that converted HTML corresponds to original plain text.
        if !matches!(part.kind(), PartKind::Text) {
            continue;
        }
        let start = part.offset_body() as usize;
        let end = part.offset_end() as usize;
        if start > end || end > raw.len() {
            continue;
        }
        let Some((text, problems)) = part.text_checked() else {
            continue;
        };
        // Avoid retaining large untrusted expanded texts in memory beyond
        // a bounded part. Source messages themselves are already bounded.
        if text.len() > 2 * 1024 * 1024 {
            continue;
        }
        let exact = problems.is_empty() && part.raw_body() == text.as_bytes();
        let quality = if exact {
            OffsetQuality::Exact
        } else {
            OffsetQuality::Derived
        };
        let mut zones = Vec::new();
        let mut pos = 0usize;
        let mut first_authored = true;
        let mut after_signoff = false;
        for line in text.split_inclusive('\n') {
            let end_pos = pos + line.len();
            let kind = classify(line, first_authored, after_signoff);
            if !line.trim().is_empty() && kind != ZoneKind::Unknown {
                first_authored = false;
            }
            if kind == ZoneKind::Signoff {
                after_signoff = true;
            }
            zones.push(SourceZone {
                kind,
                decoded_start: pos,
                decoded_end: end_pos,
                raw_start: exact.then_some(start + pos),
                raw_end: exact.then_some(start + end_pos),
                offset_quality: quality,
                sha256: digest(line.as_bytes()),
            });
            pos = end_pos;
        }
        parts.push(TextPartEvidence {
            part_id: u32::from(part.id()),
            raw_start: start,
            raw_end: end,
            raw_sha256: digest(&raw[start..end]),
            decoded_sha256: digest(text.as_bytes()),
            decoded_byte_length: text.len(),
            decoding_problems: !problems.is_empty(),
            zones,
        });
    }
    Ok(BodyEvidence {
        schema_version: "1.0.0",
        source_sha256: digest(raw),
        parts,
        skipped_html_parts,
    })
}
