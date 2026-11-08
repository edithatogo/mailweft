# FOI-only consumer qualification and migration specification

**Goal:** prepare a verified, reversible integration into `foi-process`, without importing that application into the core.
**Architecture:** consumer -> generic email library -> upstream crates. Raw MIME and FYI web/export evidence remain distinguishable.
**Requirements:** R07, R20, R21, R25, R32, R35, R38, R43, R45, R47.
**Design:** `docs/FOI-SCOPE-AND-ROBUSTNESS.md`, `docs/REPOSITORY-REUSE.md`, `docs/CONTRACT.md`.
**Acceptance:** each packet has its checker and honest execution statuses. Packet completion does not establish live runtime qualification or legal correctness. No other owned repository is active scope; historical metadata is not an implementation instruction.
