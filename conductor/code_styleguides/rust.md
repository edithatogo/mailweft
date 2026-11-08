# Rust code style
- Format with `cargo fmt --all --check` and lint `cargo clippy --workspace --all-targets -- -D warnings`.
- Prefer explicit structs/enums over untyped maps for contracts; serde discriminated unions for event and interpretation states.
- Typed errors and stable diagnostic codes; no `.unwrap()` or `.expect()` on input-derived values.
- Constructors validate time zones, typed IDs, relationship roles, limits and data classification.
- Stream bounded bytes; include unit/property/fuzz tests for parsing and graph operations.
- Keep core public API blocking/deterministic unless async source adapter explicitly requires otherwise.
- No default telemetry, network, panic on malformed MIME, unsafe HTML rendering or plugin-executed attachment contents.
- Clear feature flags, no optional dependency leaking into default build; full Rustdoc on exported contracts.
