# Cross-track implementation checkpoint — synthetic evaluation and OCEL loss accounting

Date: 2026-11-10. This is **additive evidence**, not an override of the canonical task manifest. Conductor status is not changed until the full task acceptance contract is satisfied.

## Implemented slices

**Track 03 / T03.08 (partial):** Rust `ocel_capabilities::capability_report` produces a deterministic machine-readable loss/capability ledger for a supplied OCEL graph. It includes structural validation codes, counts, source-content exclusion, claimed-time semantics, string-only attribute limitations, unqualified XML/SQLite roundtrips and absence of actor/work-item inference. It does not claim normative certification or actual source data loss. Tested against valid and intentionally invalid generated OCEL; the original log is never changed.

**Track 04 / T04.01 (implementation candidate):** `scripts/synthetic_mime.py` deterministically generates four original fictional RFC 5322 messages and five controlled perturbations, with SHA-256 hashes, byte lengths, construction truth and a reproducible manifest. All addresses use `example.invalid`. The generated dataset is optional verification input, not Mailweft's purpose or a production dependency. Unit tests check repeatability, byte integrity and fictional-data boundaries.

**Track 11 / T11.02 (partial):** The perturbations vary claimed date, missing reply parent, nested quote depth, recipient and duplicate Message-ID claim. `scripts/validate_synthetic_runtime.py` runs them against the compiled Rust CLI, asserting candidate-link abstention and observation-time invariance. This is a *controlled counterfactual smoke test*, not calibrated error-impact measurement or a process-mining distortion study.

**Track 02 (supporting evidence):** The synthetic runtime verifies that missing and ambiguous claimed identifiers do not become verified reply edges, and that quote-depth changes do not create new observed messages.

**Track 08 (documentation):** Consumer and release boundaries remain unchanged. The machine-readable report is an additive library API; no new format is declared OCEL 2.0 itself, and no version bump is claimed. FOI remains the first downstream consumer; it must not use legacy unvalidated outputs as truth.

## Explicitly unfinished

- T03.08 full cross-format loss matrix and independent JSON/XML/SQLite roundtrips.
- T04.01 full parameterised generator and coverage-based test family, beyond this bounded fictional baseline; rights-gated real corpora T04.02–08.
- T11.02 full semantic perturbation coverage, calibration, downstream process distortion and statistical power/uncertainty; T11.01 and other prerequisites remain open.
- T02 full identity resolution, case threading and external-source adjudication.
- All source rights, GraphRAG, NLP, FOI consumer and publication/release gates.

## Verification requirements

`python3 -m unittest discover -s scripts -p 'test_*.py'`
`cargo test --locked --workspace --all-targets`
`cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop`
`cargo build --locked -p mailweft-cli && python3 scripts/validate_synthetic_runtime.py`
`cargo fmt --all -- --check`
`cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
`python3 scripts/conductor_doctor.py --check`

Record exact final commit CI links in the PR. No private or real email data is processed. The Rust4PM quick-xml advisory remains a release blocker.
