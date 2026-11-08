# Mailweft naming and contract identity

The owner selected `edithatogo/mailweft` on 8 November 2026. Product and planned Rust package names use Mailweft. The earlier working name `email-ocel` remains in Git history and existing version-1 schema `$id` URIs. Those URIs are stable identifiers, not claims that an HTTP endpoint exists. Do not silently rewrite serialized identifiers or previous evidence hashes because the repository was renamed.

No published Rust crate or runtime API exists yet. T00.04 creates the actual workspace; planned `crates/mailweft` and `crates/mailweft-semantics` names do not establish registry availability. The current schema files and their bytes are preserved. New contracts use a versioned Mailweft namespace only after an explicit migration decision.

The Conductor gitlink is pinned to `7a5c560a4fdf5297be58594cc37527eb12790272`. A release tag is not a branch: the erroneous historical `branch = conductor-v0.4.1` setting has been removed. `git submodule update --init --recursive` checks out the recorded commit; no automatic remote tracking or plugin execution is configured.

## Implemented package layout

The native checkpoint uses `crates/mailweft-core` and `crates/mailweft-cli`; logical components are internal modules. Earlier single-directory paths are superseded by this tested two-package layout. Stable schema IDs are unchanged. See docs/CORE-CHECKPOINT.md for actual status and unresolved security gates.
