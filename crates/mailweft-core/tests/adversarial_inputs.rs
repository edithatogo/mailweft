//! Deterministic malformed-input smoke tests. This is NOT coverage-guided fuzzing.
use chrono::{TimeZone, Utc};
use mailweft_core::{convert_eml, inspect_body_zones};
use std::panic::{AssertUnwindSafe, catch_unwind};

const RAW: &[u8] = b"From: a@example.invalid\r\nTo: b@example.invalid\r\nSubject: Example\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=\"part\"\r\n\r\n--part\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello there.\r\n--part\r\nContent-Type: text/plain\r\nContent-Disposition: attachment; filename=\"a.txt\"\r\nContent-Transfer-Encoding: base64\r\n\r\nSGVsbG8=\r\n--part--\r\n";

fn next(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^ (state << 17)
}

fn mutated(seed: u64) -> Vec<u8> {
    let mut data = RAW.to_vec();
    let mut state = seed.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(1);
    for _ in 0..1 + (seed % 9) {
        state = next(state);
        let index = (state as usize) % (data.len() + 1);
        state = next(state);
        match state % 4 {
            0 if index < data.len() => data[index] = (state >> 8) as u8,
            1 if data.len() > 1 && index < data.len() => {
                data.remove(index);
            }
            2 if data.len() < 1024 => data.insert(index, (state >> 16) as u8),
            _ => {
                if index < data.len() {
                    data[index] = 0;
                }
            }
        }
    }
    data
}

#[test]
fn synthetic_mime_mutations_never_panic_or_emit_oversized_source() {
    let at = Utc.with_ymd_and_hms(2026, 11, 9, 7, 0, 0).unwrap();
    for seed in 0..256 {
        let candidate = mutated(seed);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let converted = convert_eml(&candidate, "fictional/adversarial", at);
            let zoning = inspect_body_zones(&candidate);
            if let Ok(log) = converted {
                assert_eq!(log.source_sha256.len(), 64);
                assert_eq!(log.log.events.len(), 1);
            }
            if let Ok(evidence) = zoning {
                assert_eq!(evidence.source_sha256.len(), 64);
                assert!(
                    evidence
                        .parts
                        .iter()
                        .all(|part| part.raw_start <= part.raw_end
                            && part.raw_end <= candidate.len())
                );
                assert!(
                    evidence
                        .parts
                        .iter()
                        .flat_map(|part| &part.zones)
                        .all(|zone| zone.decoded_start <= zone.decoded_end)
                );
            }
        }));
        assert!(
            result.is_ok(),
            "deterministic malformed MIME seed {seed} caused panic"
        );
    }
}
