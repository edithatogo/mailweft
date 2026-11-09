# Evidence — Message/custodian identity, duplicates and conversation inference

Status: **PARTIAL BASELINE IMPLEMENTED**, not full track acceptance.

Current Rust `convert_batch` preserves exact-byte source copy identities and prevents multiple observed-copy events from being treated as multiple independent logical messages. The `feat/thread-candidates` branch adds an unverified `reply_target_candidate` object-to-object edge **only** for a uniquely matched `In-Reply-To` to an observed logical message's claimed `Message-ID`. Tests verify missing targets, duplicate custody, conflicting/ambiguous targets, References-only, and deterministic input order.

Tests:
`cargo test --locked -p mailweft-core --test thread_candidates`,
`cargo test --locked --workspace --all-targets`,
`cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop`,
`cargo fmt --all -- --check`,
`cargo clippy --locked --workspace --all-targets -- -D warnings`.

Add exact final GitHub Actions run links once accepted; do not claim these tests run from their presence alone.

This does **not** complete the full identity/thread task series: absent/forged/duplicated Message-ID, aliases, malformed MIME, external message references, missing source copies, subject heuristics, quoted correspondence, multiple concurrent work items, temporal consistency, actor confirmation and accuracy benchmarks all require further tests or human adjudication. The inferred O2O link is labelled a **candidate**, not a verified reply. No real mail or FOI data was processed.
