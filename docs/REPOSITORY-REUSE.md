# Repository scope — FOI first, generic converter core

Mailweft's core outcome is **an email-to-OCEL converter written in Rust**. `foi-process` is its first and sole active owned consumer; Mailweft never depends on it. Enron is a canonical verification/example dataset, not a product specialization.

The existing FOI pipeline is unvalidated and is **not a reference implementation, teacher, validation baseline or oracle**. Source inspection may identify reusable patterns or reproduce specific defects; no legacy expected output is inherited. Use independent source adjudication and separately validated fixtures.

Read [PRODUCT-SCOPE](PRODUCT-SCOPE.md), [FOI ownership and ontology feedback](FOI-BOUNDARIES.md), and [robustness evaluation](FOI-SCOPE-AND-ROBUSTNESS.md).

`fyi-cli` owns capture; the `fyi-archive` boundary owns preserved source/manifests and publication (or an explicitly verified successor). Their current documentation conflicts about a handover, so T12.02 must verify it. `foi-o` owns the domain semantics and reviews evidence-linked contribution candidates from validated foi-process results. These are boundaries in the first consumer, not extra generic integration projects.

The earlier repository register is retained as historical provenance only. No CareOps, Kairos, scholarly platform, document platform, generic integration framework or reverse FOI dependency is active scope. Generic provider interfaces and qualified upstream parsing/Rust4PM crates remain eligible.

The previously inspected FOI adapter emits empty attribute vectors. Do not copy that reduced projection into Mailweft's full-fidelity export path. Confirm individual defects independently; do not infer entire-system correctness or failure from one source inspection.
