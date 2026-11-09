# Resolved direct dependency inventory

Generated from locked all-feature Cargo metadata and available registry source licence files. This is an inventory, not a legal opinion or a transitive vulnerability audit.

| Crate | Exact resolved version | Declared SPDX expression | Source licence files hashed |
|---|---|---|---|
| chrono | 0.4.45 | MIT OR Apache-2.0 | 1 |
| mail-parser | 1.0.1 | Apache-2.0 OR MIT | 0 |
| process_mining | 0.6.2 | MIT OR Apache-2.0 | 0 |
| serde | 1.0.229 | MIT OR Apache-2.0 | 2 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | 2 |
| sha2 | 0.10.9 | MIT OR Apache-2.0 | 2 |
| tempfile | 3.27.0 | MIT OR Apache-2.0 | 2 |
| thiserror | 2.0.21 | MIT OR Apache-2.0 | 2 |

File SHA-256 receipts: `docs/dependency-inventory.json`. Mail-parser and Rust4PM retain their upstream licences. No upstream implementation is copied into Mailweft; transfer mapping verifies parser-produced bytes and does not replace MIME parsing.

The CLI now declares serde directly; it was already resolved in the workspace. No new package is added for graph/model/cloud use. Pinning is through Cargo.lock and rust-toolchain.toml. A fresh security/advisory scan remains T00.06 work.
