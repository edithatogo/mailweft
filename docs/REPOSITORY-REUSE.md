# Repository scope and ownership — FOI only

**Superseding user decision, 8 November 2026:** `foi-process` is the only active owned-repository integration. The earlier broad repository review is preserved in Git history and the historical register; it is not an adoption backlog.

The new library is an intended dependency of `foi-process`, never the reverse. It remains a generic Rust raw-email -> OCEL 2.0/evidence library. FOI/FYI vocabulary, request/authority identity, platform state interpretation, policy/deadline rules and publication stay in the consumer.

See [the complete source-fidelity, migration and robustness protocol](FOI-SCOPE-AND-ROBUSTNESS.md).

## Confirmed source finding, not a whole-system verdict

The read-only inspected `foi-process/src/rust4pm.rs` (blob `7665c02ae363507b22f659a14cebc372e1353e14`) deliberately emits empty attribute vectors. The normalizer `src/normalize.rs` (blob `0eda8dd516754248b25615ac28dc8a86ff96a2cd`) maps supplied activity labels and allowlisted attributes. Reproduce bounded defects before replacing either path; no fresh FOI test run has been performed here.

## Integration order

1. Reproduce defects and freeze permitted source/output fixtures.
2. Distinguish raw RFC messages from platform exports, web renditions and attachments.
3. Qualify complete OCEL/evidence mappings and explicit unsupported/field-loss states.
4. Shadow old/new processing with independent adjudication; old output is not the oracle.
5. Prepare migration, feature flag, replay and rollback in the consumer repository.
6. Promote only after actual consumer tests and permissions pass. No remote change is implied by a packet.

The track retains its existing ID `12_ecosystem-reuse` to preserve task references, but its scope is exclusively FOI integration. All other owned-repository integrations are out of scope. Upstream third-party parsing and Rust4PM dependencies are not affected by this restriction.
