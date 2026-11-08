# Implementation interface register (planned, not compiled)

Core task interfaces remain in their owning track. New optional modules live in `email-ocel-semantics` created by T05.02, not extra publishable microcrates. Each task owns its named function, associated new enums/structs and exactly named tests. Existing types must be imported, not redefined. If source signatures differ after an accepted predecessor, stop for a contract-level reconciliation instead of silently editing neighboring interfaces.

## Shared type ownership

- `ObservedMessage`, `MimeMetadata`: T01.04/T01.06, core.
- `AnalysisView`, `ParticipantAssertion`, decoded source map: T05b.01–T05b.06, semantics; use field names from existing zone/intext schemas.
- `ExtractionClaim`, `EvidenceReference`: T06.01; no raw text in exported evidence references.
- `TaxonomyProfile`, `MappingDecision`, `AttributedCommunicationAct`: T09.01/T09.03; the act record mirrors `communication-act.schema.json`.
- `RoleCandidates`, `WorkItemLinkCandidate`, `MessageKindCandidate`: respective T09.04/T09.07/T09.02 owners. Each has explicit candidate/unknown states; no forced human identity.
- `InteractionLink`, `InteractionContext`, `CommitmentCandidate`, `TransitionDecision`: T09.05/T09.06; links mirror `interaction-link.schema.json`.
- `InferenceReceipt`, `EvidenceContext`, `ValidationDecision`: T10.01; mirror receipt schema and resolve IDs against protected evidence, not embedded text.
- `ProjectionPolicy`, `AdmissionDecision`: T10.02; policy specifies required checks and scope. `ProjectionResult`/`OcelProjection` reuse T06.08/T03.03 outputs through a small wrapper, not a second OCEL model.
- `CopyOrQuoteLink`, `DependenceIndex`: T10.03; link origins carry source identity and evidence basis.
- `TemporalMention`, `TemporalContext`, `TemporalResolution`: T10.04; preserve candidate intervals, timezone/locale and unknown status.
- `InterpretationDelta`, `InterpretationState`, `DeltaOutcome`: T10.05; semantic validation beyond JSON Schema is mandatory.
- `SnapshotBoundary`, `InterpretationSnapshot`, `ReplayError`: T10.06; no revision mutation of source observations.
- `AccessScope`, `SubjectId`, `DerivationIndex`, `PurgePlan`: T10.07; purge is a plan, not unauthorized destructive execution.
- `WorkBudget`, `RecomputePlan`: T10.08; explicit pending/partial result on exhausted budget.
- Research types `ExpectedDeltaPair`, `PerturbationProfile`, `ScoredPrediction`, `GoldSet`, `EvalPolicy`, `CalibrationReport`, `ExtractionRequest`, `ProviderSet`, `InferenceBudget`, `ExtractionOutcome`, `ComparisonPolicy`, `ProcessDistortionReport`: defined by their single T11 owner and not exposed by the core public API.

Exact struct fields for schema-backed types are those in the named schema. Other types use only fields required by the two named oracle tests and the design; no speculative generic type hierarchy. Update this register and task-owned schema for any externally visible change.
