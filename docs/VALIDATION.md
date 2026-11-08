# Validation strategy — explicit reference populations and metrics

## Levels
1. **Parser**: adjudicated synthetic raw EML; headers, body, charset, MIME structure, attachments, nested messages, envelope errors and malformed inputs.
2. **Threading**: constructed truth graph for References/In-Reply-To, missing headers, subject collisions, quoted-forward chains; measure link precision/recall and false-positive rate.
3. **OCEL**: test all OCEL 2.0 elements (event/object types/attributes, time-varying values, E2O/O2O qualifiers, multiplicity and timestamps) across native Rust import/export; independent reader checks when possible.
4. **Corpus**: compare raw-first outputs with processed Enron only after confirming equivalent source subset and matching coverage. Run structural benchmarks across non-paired datasets without suggesting event-level agreement.
5. **Semantic**: gold annotations of request/action/actor/deadline/negation/temporality/decision communication and explicitly unresolvable cases; adjudication blind to model output. Report precision, recall, F1, calibration, abstention and contradictions.
6. **GraphRAG**: optional and only if graph-only/vector-only baselines underperform. Measure grounded evidence retrieval and hallucination rate, cost, privacy and time; no graph-retrieval step can create observed facts.

## Pass/fail
- Core fixtures: required data fully preserved on valid samples, no invalid OCEL accepted, duplicates explicit, deterministic replay. Hard fail on unauthorized reads, uncontrolled egress, unbounded nesting, leaked raw email content, or silent loss of OCEL qualifiers/history.
- Cross-engine output: explicit equality comparisons of IDs/types/qualifiers/time-varying attributes, and format-aware differences documented in a `loss_ledger`.
- Performance: record wall-clock seconds, peak RSS, events/second, source size, crate versions, OS/CPU, deterministic seed. Set baselines empirically rather than guessing throughput.
- Report all excluded/failed messages and denominator. Missing/mismatched corpus scope is NOT zero events.
- Benchmark sets: tiny invented synthetic fixture; 10k/100k appropriately sampled raw corpus; full corpus only after rights and resource qualification.
- CI runs synthetic tests only. Enron/full-corpus tests are external reproducible jobs with no raw data uploaded to GitHub Actions.

## 0.2 benchmark and boundary refinements
See [FRONTIER-EVALUATION.md](FRONTIER-EVALUATION.md) for metamorphic cases, independent component-level splits, downstream process-error metrics, calibration and contemporary comparators. Preserve cross-population matchability gates. Distinguish a library feature being implemented, executed, externally benchmarked and scientifically superior.
