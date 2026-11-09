//! Non-destructive plaintext zoning; labels and origin scopes are rule hypotheses.
//! Source hashes and offsets are retained, never plaintext or inferred task ownership.
use crate::{Error, MAX_EML_BYTES, digest};
use mail_parser::{MessageParser, PartKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const MAX_TEXT_BYTES: usize = 2 * 1024 * 1024;
const MAX_ZONES: usize = 65_536;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OffsetQuality {
    Exact,
    Derived,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ZoneKind {
    Authored,
    Salutation,
    Quotation,
    ForwardedMarker,
    ForwardedContent,
    InlineHeader,
    Signoff,
    Signature,
    Disclaimer,
    Unknown,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OriginScope {
    NewAuthorContent,
    QuotedPriorContent,
    ForwardedThirdParty,
    GeneratedOrUnknown,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkipReason {
    HtmlUnsupported,
    UnsupportedBodyKind,
    UnverifiablePartRange,
    DecodingUnavailable,
    DecodedSizeLimit,
    ZoneCountLimit,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkippedPart {
    pub part_id: u32,
    pub reason: SkipReason,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceZone {
    pub kind: ZoneKind,
    pub origin_scope: OriginScope,
    pub quote_depth: usize,
    pub forward_depth: usize,
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
    pub skipped_parts: Vec<SkippedPart>,
}

#[derive(Default)]
struct ZoneState {
    seen_authored: bool,
    after_signoff: bool,
    forward_depth: usize,
}

impl ZoneState {
    fn classify(&mut self, line: &str) -> (ZoneKind, OriginScope, usize, usize) {
        let trimmed = line.trim();
        let mut remainder = trimmed;
        let mut quote_depth = 0;
        while let Some(rest) = remainder.strip_prefix('>') {
            quote_depth += 1;
            remainder = rest.trim_start();
        }
        if quote_depth > 0 {
            // A following unprefixed inline answer must not inherit signature state.
            self.after_signoff = false;
            return (
                ZoneKind::Quotation,
                OriginScope::QuotedPriorContent,
                quote_depth,
                self.forward_depth,
            );
        }
        let lower = trimmed.to_ascii_lowercase();
        if lower.contains("forwarded message")
            && (lower.starts_with("begin ") || lower.starts_with('-'))
        {
            self.forward_depth += 1;
            self.after_signoff = false;
            return (
                ZoneKind::ForwardedMarker,
                OriginScope::ForwardedThirdParty,
                0,
                self.forward_depth,
            );
        }
        if self.forward_depth > 0 {
            let inline_header = [
                "from:",
                "sent:",
                "date:",
                "to:",
                "cc:",
                "bcc:",
                "subject:",
                "reply-to:",
            ]
            .iter()
            .any(|prefix| lower.starts_with(prefix));
            let kind = if inline_header {
                ZoneKind::InlineHeader
            } else if trimmed.is_empty() {
                ZoneKind::Unknown
            } else {
                ZoneKind::ForwardedContent
            };
            // Without a trustworthy end delimiter, abstain from promoting any
            // trailing passage back to current-author content.
            return (
                kind,
                OriginScope::ForwardedThirdParty,
                0,
                self.forward_depth,
            );
        }
        if trimmed.is_empty() {
            return (ZoneKind::Unknown, OriginScope::GeneratedOrUnknown, 0, 0);
        }
        // Footer detection precedes signature carry-over.
        if lower.starts_with("this email and any attachments")
            || lower.starts_with("this message is confidential")
        {
            return (ZoneKind::Disclaimer, OriginScope::GeneratedOrUnknown, 0, 0);
        }
        let kind = if trimmed == "--"
            || matches!(
                lower.as_str(),
                "regards,"
                    | "kind regards,"
                    | "best regards,"
                    | "sincerely,"
                    | "many thanks,"
                    | "thanks,"
                    | "yours sincerely,"
            ) {
            self.after_signoff = true;
            ZoneKind::Signoff
        } else if self.after_signoff {
            ZoneKind::Signature
        } else if !self.seen_authored
            && (lower.starts_with("hi ")
                || lower.starts_with("hello ")
                || lower.starts_with("dear "))
            && (trimmed.ends_with(',') || trimmed.ends_with(':'))
        {
            ZoneKind::Salutation
        } else {
            ZoneKind::Authored
        };
        self.seen_authored = true;
        (kind, OriginScope::NewAuthorContent, 0, 0)
    }
}

/// Produce rule-based source annotations, not factual authorship or event assertions.
/// Every accepted text part is fully covered; every skipped candidate part is reported.
pub fn inspect_body_zones(raw: &[u8]) -> Result<BodyEvidence, Error> {
    if raw.is_empty() || raw.len() > MAX_EML_BYTES {
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
    let mut skipped_parts = Vec::new();
    let mut seen = BTreeSet::new();
    for part in parsed
        .html_body()
        .filter(|p| matches!(p.kind(), PartKind::Html))
    {
        if seen.insert(part.id()) {
            skipped_parts.push(SkippedPart {
                part_id: part.id(),
                reason: SkipReason::HtmlUnsupported,
            });
        }
    }
    let skipped_html_parts = skipped_parts.len();
    let mut remaining_zones = MAX_ZONES;
    for part in parsed.text_body() {
        if !seen.insert(part.id()) {
            continue;
        }
        let skip = |reason| SkippedPart {
            part_id: part.id(),
            reason,
        };
        if !matches!(part.kind(), PartKind::Text) {
            skipped_parts.push(skip(SkipReason::UnsupportedBodyKind));
            continue;
        }
        let start = part.offset_body() as usize;
        let end = part.offset_end() as usize;
        if raw.get(start..end) != Some(part.raw_body()) {
            skipped_parts.push(skip(SkipReason::UnverifiablePartRange));
            continue;
        }
        // Measure transfer size before materializing decoded text. Charset
        // expansion is checked again afterwards, within the bounded source.
        if part.decoded_len() > MAX_TEXT_BYTES {
            skipped_parts.push(skip(SkipReason::DecodedSizeLimit));
            continue;
        }
        let Some((text, problems)) = part.text_checked() else {
            skipped_parts.push(skip(SkipReason::DecodingUnavailable));
            continue;
        };
        if text.len() > MAX_TEXT_BYTES {
            skipped_parts.push(skip(SkipReason::DecodedSizeLimit));
            continue;
        }
        let line_count = text.split_inclusive('\n').count();
        if line_count > remaining_zones {
            skipped_parts.push(skip(SkipReason::ZoneCountLimit));
            continue;
        }
        remaining_zones -= line_count;
        let exact = problems.is_empty() && raw.get(start..end) == Some(text.as_bytes());
        let quality = if exact {
            OffsetQuality::Exact
        } else {
            OffsetQuality::Derived
        };
        let mut zones = Vec::with_capacity(line_count);
        let mut pos = 0;
        let mut state = ZoneState::default();
        for line in text.split_inclusive('\n') {
            let end_pos = pos + line.len();
            let (kind, origin_scope, quote_depth, forward_depth) = state.classify(line);
            zones.push(SourceZone {
                kind,
                origin_scope,
                quote_depth,
                forward_depth,
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
            part_id: part.id(),
            raw_start: start,
            raw_end: end,
            raw_sha256: digest(&raw[start..end]),
            decoded_sha256: digest(text.as_bytes()),
            decoded_byte_length: text.len(),
            decoding_problems: !problems.is_empty(),
            zones,
        });
    }
    skipped_parts.sort_by_key(|p| p.part_id);
    Ok(BodyEvidence {
        schema_version: "1.1.0",
        source_sha256: digest(raw),
        parts,
        skipped_html_parts,
        skipped_parts,
    })
}
