# Release policy — Mailweft (not yet releasable)

**Status: BLOCKED.** Mailweft 0.7.0 is an experimental Rust email-to-OCEL converter, not a production-supported or security-cleared release. The release process does not publish by itself.

## Current hard blocker

The committed Cargo.lock includes optional Rust4PM process_mining 0.6.2, which requires quick-xml ^0.37.4; the resolved quick-xml is 0.37.5. RustSec **RUSTSEC-2026-0194** and **RUSTSEC-2026-0195** are patched in quick-xml >=0.41.0. References: https://github.com/edithatogo/mailweft/issues/4 ; https://rustsec.org/advisories/RUSTSEC-2026-0194.html ; https://rustsec.org/advisories/RUSTSEC-2026-0195.html .

This affects the optional Rust4PM validation dependency graph; there is no established exploitable XML path through the default email-to-JSON CLI. However, absence of a demonstrated reachability route does not clear the dependency audit, qualify XML use, or justify ignoring advisories.

**No advisory suppression, ignore list, continue-on-error, false-green scan, or unconditional release script is permitted.** No production release is authorized until the dependency is remediated (upstream-compatible patched release, reviewed temporary fork with maintenance/retirement plan, or another independently qualified OCEL validator without sacrificing OCEL 2.0 semantics), Cargo.lock regenerated, and all qualification checks rerun.

## Executable gates

The read-only [release-security workflow](../.github/workflows/release-security.yml) is manually runnable and also runs for version tag pushes. It runs a current full Cargo.lock advisory scan, cargo-deny advisories/bans/licences/sources, and the deterministic known-quick-xml blocker check. **The current expected result is a failed security gate.** Failure is a correct release decision, not a reason to disable the checks.

A separate cross-platform [matrix workflow](../.github/workflows/platform-matrix.yml) tests core/CLI and Rust4PM imports across Linux x64, macOS and Windows. A green functional matrix is necessary but never sufficient to approve release.

Required further evidence: complete OCEL 2.0 schema/type/relationship fidelity and independent round-trips; no silent attribute/qualifier losses; source hashing/receipt integrity; malformed/resource-bound adversarial fixtures; platform CI; dependency notices, SBOM and provenance; independently reviewed corpus rights; a versioned changelog; human go/no-go approval and an authorized release destination. Enron/full-corpus, NLP and GraphRAG experiments are **not** prerequisites for an independently qualified core-only release, but core security and interoperability are.

## Conductor status

The canonical task manifest remains authoritative. T00.06 (security CI), T07.07 (supply chain), and release tasks T08.06–T08.08 are not completed by writing workflows or this policy. Evidence from executed successful checks, and explicit residual limitations, must precede changes to their status. Use `python3 scripts/conductor_doctor.py --check`; `--sync` regenerates context only. Gemini extension repository pin is verified; host-level registration is not certified.

Future core-first tasks: remediate the Rust4PM graph; qualify complete OCEL 2.0 round-trip and schema; bound malformed MIME/large-source behavior; use scoped HMAC pseudonyms for sensitive data instead of predictable hashes; qualify safe path traversal and source-enveloped ingestion. The separate draft PR #3 contains an unmerged alternative implementation with HMAC identifiers. It is not presumed compatible or secure without a reviewed migration. No real FYI/Enron data or private correspondence should enter CI.
