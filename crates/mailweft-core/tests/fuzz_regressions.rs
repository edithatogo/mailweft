//! Synthetic corpus and reproducible, bounded mutation regression.
#[path = "support/fuzz_contract.rs"]
mod contract;

const SEEDS: &[&[u8]] = &[
    b"", b"\xff", b"Subject: synthetic\r\n\r\nbody",
    b"From: alpha@synthetic.invalid\r\nTo: beta@synthetic.invalid\r\nSubject: fictional\r\n\r\nSynthetic fixture only.",
    b"Subject: synthetic multipart\r\nContent-Type: multipart/mixed; boundary=synthetic\r\n\r\n--synthetic\r\nContent-Type: text/plain\r\nContent-Transfer-Encoding: base64\r\n\r\nU3ludGhldGlj\r\n--synthetic--\r\n",
    b"Subject: synthetic attachment\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"../synthetic.bin\"\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n=00=FF=GZ",
];

#[test]
fn synthetic_corpus_satisfies_boundary_invariants() {
    for seed in SEEDS {
        contract::check(seed);
    }
}

#[test]
fn bounded_mutations_satisfy_boundary_invariants() {
    let mut count = 0;
    for seed in SEEDS {
        let stride = (seed.len() / 32).max(1);
        for position in (0..=seed.len()).step_by(stride) {
            contract::check(&seed[..position]);
            count += 1;
            if position < seed.len() {
                for mask in [1_u8, 0x80, 0xff] {
                    let mut candidate = seed.to_vec();
                    candidate[position] ^= mask;
                    contract::check(&candidate);
                    count += 1;
                }
            }
        }
        for suffix in [b"\0".as_slice(), b"\xff", b"\n", b"{}"] {
            let mut candidate = seed.to_vec();
            candidate.extend_from_slice(suffix);
            contract::check(&candidate);
            count += 1;
        }
    }
    assert!(count > 100, "mutation corpus must execute");
}
