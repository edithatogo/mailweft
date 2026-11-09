//! Verified transfer-unit provenance over the maintained MIME parser's decoded bytes.
//! Not a replacement MIME/charset decoder. Never claims one-to-one character offsets.
use serde::{Deserialize, Serialize};

const MAX_UNITS: usize = 32_768;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RawRange {
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TransferUnit {
    pub decoded_start: usize,
    pub decoded_end: usize,
    pub raw_start: usize,
    pub raw_end: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TransferMap {
    pub status: String,
    pub reason: Option<String>,
    pub decoded_byte_length: usize,
    pub units: Vec<TransferUnit>,
    pub source_only_ranges: Vec<RawRange>,
}

fn unavailable(length: usize, reason: &str) -> TransferMap {
    TransferMap {
        status: "unavailable".into(),
        reason: Some(reason.into()),
        decoded_byte_length: length,
        units: Vec::new(),
        source_only_ranges: Vec::new(),
    }
}
fn value(byte: u8) -> Option<u32> {
    match byte {
        b'A'..=b'Z' => Some((byte - b'A') as u32),
        b'a'..=b'z' => Some((byte - b'a' + 26) as u32),
        b'0'..=b'9' => Some((byte - b'0' + 52) as u32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}
fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
fn record(
    map: &mut TransferMap,
    decoded_start: usize,
    decoded_end: usize,
    start: usize,
    end: usize,
    base: usize,
) {
    map.units.push(TransferUnit {
        decoded_start,
        decoded_end,
        raw_start: base + start,
        raw_end: base + end,
    });
}

/// All units are admitted only if their reconstruction equals parser-produced bytes.
/// A charset transform, decode warning or bounded-layout failure returns no units.
pub struct TransferInput<'a> {
    pub raw: &'a [u8],
    pub decoded: &'a [u8],
    pub text: &'a [u8],
    pub base: usize,
    pub encoding: &'a str,
    pub problems: bool,
}

pub fn verified_map(input: TransferInput<'_>, budget: usize) -> TransferMap {
    let TransferInput {
        raw,
        decoded,
        text,
        base,
        encoding,
        problems,
    } = input;
    let fail = |reason| unavailable(text.len(), reason);
    if problems {
        return fail("decoding_problem");
    }
    if decoded != text {
        return fail("charset_transformation");
    }
    if base.checked_add(raw.len()).is_none() {
        return fail("offset_overflow");
    }
    let budget = budget.min(MAX_UNITS);
    if budget == 0 {
        return fail("unit_limit");
    }
    let mut map = TransferMap {
        status: "verified_transfer".into(),
        reason: None,
        decoded_byte_length: text.len(),
        units: Vec::new(),
        source_only_ranges: Vec::new(),
    };
    let encoding = encoding.trim().to_ascii_lowercase();
    if matches!(encoding.as_str(), "" | "7bit" | "8bit" | "binary") {
        if raw != decoded {
            return fail("layout_mismatch");
        }
        map.status = "identity".into();
        if !raw.is_empty() {
            record(&mut map, 0, raw.len(), 0, raw.len(), base);
        }
        return map;
    }
    let mut rebuilt = Vec::with_capacity(decoded.len());
    if encoding == "base64" {
        let mut tokens = Vec::new();
        let mut pos = 0;
        while pos < raw.len() {
            if matches!(raw[pos], b' ' | b'\t' | b'\r' | b'\n') {
                let start = pos;
                while pos < raw.len() && matches!(raw[pos], b' ' | b'\t' | b'\r' | b'\n') {
                    pos += 1;
                }
                map.source_only_ranges.push(RawRange {
                    start: base + start,
                    end: base + pos,
                });
            } else {
                if raw[pos] != b'=' && value(raw[pos]).is_none() {
                    return fail("layout_mismatch");
                }
                tokens.push((pos, raw[pos]));
                pos += 1;
            }
            if tokens.len().div_ceil(4) + map.source_only_ranges.len() > budget {
                return fail("unit_limit");
            }
        }
        if !tokens.len().is_multiple_of(4) {
            return fail("layout_mismatch");
        }
        let groups = tokens.len() / 4;
        for (index, q) in tokens.as_chunks::<4>().0.iter().enumerate() {
            let Some(a) = value(q[0].1) else {
                return fail("layout_mismatch");
            };
            let Some(b) = value(q[1].1) else {
                return fail("layout_mismatch");
            };
            let padding = if q[2].1 == b'=' && q[3].1 == b'=' {
                2
            } else if q[3].1 == b'=' {
                1
            } else {
                0
            };
            if padding > 0 && index + 1 != groups {
                return fail("layout_mismatch");
            }
            let c = if padding == 2 {
                0
            } else {
                match value(q[2].1) {
                    Some(v) => v,
                    None => return fail("layout_mismatch"),
                }
            };
            let d = if padding > 0 {
                0
            } else {
                match value(q[3].1) {
                    Some(v) => v,
                    None => return fail("layout_mismatch"),
                }
            };
            if (padding == 2 && b & 15 != 0) || (padding == 1 && c & 3 != 0) {
                return fail("noncanonical_padding");
            }
            let combined = (a << 18) | (b << 12) | (c << 6) | d;
            let bytes = [
                (combined >> 16) as u8,
                (combined >> 8) as u8,
                combined as u8,
            ];
            let start = rebuilt.len();
            rebuilt.extend_from_slice(&bytes[..3 - padding]);
            record(&mut map, start, rebuilt.len(), q[0].0, q[3].0 + 1, base);
        }
    } else if encoding == "quoted-printable" {
        let mut pos = 0;
        while pos < raw.len() {
            if map.units.len() + map.source_only_ranges.len() >= budget {
                return fail("unit_limit");
            }
            let start = pos;
            if raw[pos] != b'=' {
                while pos < raw.len() && raw[pos] != b'=' {
                    pos += 1;
                }
                let decoded_start = rebuilt.len();
                rebuilt.extend_from_slice(&raw[start..pos]);
                record(&mut map, decoded_start, rebuilt.len(), start, pos, base);
            } else if raw.get(pos + 1..pos + 3) == Some(b"\r\n") {
                pos += 3;
                map.source_only_ranges.push(RawRange {
                    start: base + start,
                    end: base + pos,
                });
            } else if raw.get(pos + 1) == Some(&b'\n') {
                pos += 2;
                map.source_only_ranges.push(RawRange {
                    start: base + start,
                    end: base + pos,
                });
            } else {
                let Some(&hi) = raw.get(pos + 1) else {
                    return fail("layout_mismatch");
                };
                let Some(&lo) = raw.get(pos + 2) else {
                    return fail("layout_mismatch");
                };
                let (Some(hi), Some(lo)) = (hex(hi), hex(lo)) else {
                    return fail("layout_mismatch");
                };
                let decoded_start = rebuilt.len();
                rebuilt.push(hi * 16 + lo);
                pos += 3;
                record(&mut map, decoded_start, rebuilt.len(), start, pos, base);
            }
        }
    } else {
        return fail("unsupported_encoding");
    }
    if rebuilt != decoded {
        return fail("parser_disagreement");
    }
    map
}
