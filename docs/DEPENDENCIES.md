# Initial native dependency qualification

Scope: T00.01/T00.04/T00.05. Inspected 2026-11-08. Cargo resolution/build evidence is required before completion.

| Direct dependency | Exact candidate | Upstream licence | Purpose |
|---|---|---|---|
| mail-parser | 1.0.1 | Apache-2.0 OR MIT | RFC/MIME parsing with explicit size/depth/part limits |
| process_mining (Rust4PM) | 0.6.2 | MIT OR Apache-2.0 | Authoritative complete OCEL structures, not slim projections |
| serde / serde_json | 1.0.228 / 1.0.145 | MIT OR Apache-2.0 | Typed interchange |
| chrono | 0.4.42 | MIT OR Apache-2.0 | Fixed-offset timestamps; no implicit system clock |
| sha2 / hmac | 0.10.9 / 0.12.1 | MIT OR Apache-2.0 | Raw hashes and keyed, scope-separated pseudonymous identifiers |
| clap | 4.5.51 | MIT OR Apache-2.0 | Thin CLI |
| tempfile (tests only) | 3.23.0 | MIT OR Apache-2.0 | Isolated fictional file tests |

Rust 1.99.1 is pinned after checking the official rust-lang/rust release. mail-parser 1.0.1 declares Rust >=1.98. Exact direct pins do not replace the full Cargo.lock or a transitive licence/advisory audit. The first CI checkpoint resolves and locks the graph on an isolated feature branch; no registry package is published.

mail-threading is deferred until its actual graph semantics and dependency value are qualified. No duplicate parser, OCEL representation, model runtime, graph database, Python application or FOI dependency is introduced. XML/SQLite and external comparator features remain separately qualified.

Sources: https://github.com/stalwartlabs/mail-parser/blob/main/Cargo.toml ; https://github.com/aarkue/rust4pm/blob/main/process_mining/Cargo.toml ; https://github.com/rust-lang/rust/releases/tag/1.99.1 . Full dependency licence inventory and security scans remain T00.06 until actually executed.
