# Pinned Conductor command compatibility — final review

This review resumes the existing `feat/corpus-mapping-conductor` branch rather than recreating work already committed. The prior checkpoint `4ef2bf7160320f26728515c46868e09fc22ef61d` passed its Rust and planning checks, but source review found an integration defect those tests did not cover.

The pinned `commands/conductor/implement.toml` requires `## [ ] Track: ...` headings in sections separated by `---`, with a track-folder link. The custom generated registry used Markdown list entries. These looked readable but did not satisfy the pinned command's documented parsing protocol. The renderer and checked-in registry now use the upstream heading form; an independent grammar test checks every track rather than only comparing the renderer with itself.

The doctor now also refuses a completed task whose exact prerequisites are not completed. Rendering `--sync` never changes canonical statuses or converts partial work to acceptance. This prevents an internally synchronized but logically invalid status report.

RED was executed at commit `38a3ff868de56276a76923e248c8102db6ff32e0`, planning run `42626883968`, job `129192829247`: two behavioral tests failed (missing heading grammar and missing dependency-acceptance guard). The current accepted task dependency check itself passed. The dependency-test fixture has since been made complete so no incidental missing-artifact diagnostics obscure its result.

The original corpus-reconciliation and transfer-map RED/GREEN evidence is in `VERIFICATION-0.7.md`. Repository setup is checked from the actual immutable gitlink, clean checked-out upstream source, extension manifest, six command TOMLs, project context and all track artifacts. This is not a claim that Gemini CLI was installed or linked on the user's computer.

No upstream submodule code is modified and no auto-update is enabled. The previous source-envelope work remains recorded separately in `conductor/remaining-priorities-20261109.md`; it is a historical ledger rather than the current task scheduler.
