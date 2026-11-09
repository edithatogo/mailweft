# Message/custodian identity, duplicates and conversation inference — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [x] Task: **T02.01 — Implement domain-separated stable identity types**

  - Lane: `core`; dependencies: T01.08.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/src/identity.rs`, `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/tests/conversion.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Same bytes+config => same ID; no raw identity in output. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.
  - Implemented: Domain-separated deterministic IDs have source-copy and event tests; raw identifiers are not emitted.
  - Remaining: Hashes are pseudonyms, not anonymity; keyed/scoped identity is a separate hardening step.

- [x] Task: **T02.02 — Differentiate source copies and logical message candidates**

  - Lane: `core`; dependencies: T02.01.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/src/batch.rs`, `crates/mailweft-core/src/copies.rs`, `crates/mailweft-core/tests/batch.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Two custodians retain separate copies. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.
  - Implemented: Distinct custodians retain source copies while exact-byte message candidates can be shared.
  - Remaining: Near-duplicate and content-semantic identity resolution remain separate.

- [x] Task: **T02.03 — Handle missing and colliding Message-ID values**

  - Lane: `core`; dependencies: T02.02.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/src/dedup.rs`, `crates/mailweft-core/src/header_claims.rs`, `crates/mailweft-core/tests/thread_candidates.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: No silent data loss or overwrite. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.
  - Implemented: Missing/colliding Message-ID claims do not overwrite distinct raw messages or force ambiguous parent links.
  - Remaining: Full legacy-address and broader threading inference are not included.

- [~] Task: **T02.04 — Implement explicit To/Cc/Bcc role assertions**

  - Lane: `core`; dependencies: T02.03.
  - Requirements: R04, R05, R06, R19, R27, R33, R41.
  - Allowed paths: `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/src/parties.rs`, `crates/mailweft-core/tests/rust4pm_interop.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Roles round-trip without duplicates introduced. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.
  - Implemented: Observed participant qualifiers and independent-import checks exist.
  - Remaining: Complete role-value roundtrip, Bcc disclosure controls and empty/unknown states need stronger qualification.

- [~] Task: **T02.05 — Implement RFC References and In-Reply-To thread graph**

  - Lane: `core`; dependencies: T02.04.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/src/batch.rs`, `crates/mailweft-core/src/threads.rs`, `crates/mailweft-core/tests/thread_candidates.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Truth graph fixtures pass. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.
  - Implemented: Unique observed In-Reply-To targets are labelled candidate edges; missing and ambiguous targets abstain.
  - Remaining: Reference-chain reconstruction, cycles, incomplete sources and externally adjudicated accuracy remain.

- [ ] Task: **T02.06 — Add subject/quote fallback as inferred links only**

  - Lane: `core`; dependencies: T02.05.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/src/heuristics.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Ambiguous collisions abstain, links uncertain. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.

- [ ] Task: **T02.07 — Implement legacy Exchange/X500/IMCEA adapter**

  - Lane: `core`; dependencies: T02.06.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/src/legacy.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Retain unresolved identity assertions, no false merges. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.

- [ ] Task: **T02.08 — Add thread graph cycle/conflict and duplication tests**

  - Lane: `core`; dependencies: T02.07.
  - Requirements: R04, R05, R06, R19.
  - Allowed paths: `crates/mailweft-core/tests/threading.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No cycles passed as confirmed thread. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/02_identity/evidence.md`.
