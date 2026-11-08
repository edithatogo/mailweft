# Message/custodian identity, duplicates and conversation inference — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T02.01 — Implement domain-separated stable identity types**

- Lane: `core`; dependencies: T01.08.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/src/identity.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Same bytes+config => same ID; no raw identity in output. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.02 — Differentiate source copies and logical message candidates**

- Lane: `core`; dependencies: T02.01.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/src/copies.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Two custodians retain separate copies. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.03 — Handle missing and colliding Message-ID values**

- Lane: `core`; dependencies: T02.02.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/src/dedup.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No silent data loss or overwrite. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.04 — Implement explicit To/Cc/Bcc role assertions**

- Lane: `core`; dependencies: T02.03.
- Requirements: R04, R05, R06, R19, R27, R33, R41.
- Allowed paths: `crates/mailweft-core/src/parties.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Roles round-trip without duplicates introduced. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.05 — Implement RFC References and In-Reply-To thread graph**

- Lane: `core`; dependencies: T02.04.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/src/threads.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Truth graph fixtures pass. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.06 — Add subject/quote fallback as inferred links only**

- Lane: `core`; dependencies: T02.05.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/src/heuristics.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Ambiguous collisions abstain, links uncertain. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.07 — Implement legacy Exchange/X500/IMCEA adapter**

- Lane: `core`; dependencies: T02.06.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/src/legacy.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Retain unresolved identity assertions, no false merges. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.

## [ ] **T02.08 — Add thread graph cycle/conflict and duplication tests**

- Lane: `core`; dependencies: T02.07.
- Requirements: R04, R05, R06, R19.
- Allowed paths: `crates/mailweft-core/tests/threading.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No cycles passed as confirmed thread. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/02_identity/evidence.md`.
