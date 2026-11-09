#!/usr/bin/env python3
"""One-time reviewed source/context migration for the corpus-mapping branch. Remove before merge."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json
import os
import subprocess
import sys
import tomllib
import conductor_doctor as doctor

ROOT = Path(__file__).resolve().parent.parent
os.chdir(ROOT)

def replace(path, before, after):
    p = ROOT / path
    text = p.read_text()
    if after in text:
        return
    if before in text:
        if text.count(before) != 1: raise RuntimeError(f'ambiguous migration: {path}')
        p.write_text(text.replace(before, after, 1))
    elif after not in text:
        raise RuntimeError(f'migration anchor missing: {path}')


def prepare_code():
    replace('crates/mailweft-core/src/lib.rs', 'mod attachments;', 'pub mod corpus;\npub mod transfer_map;\nmod attachments;')
    # Normalize an accidental non-ASCII identifier in the unpublished source draft.
    path = ROOT / 'crates/mailweft-core/src/corpus.rs'
    path.write_text(path.read_text().replace('oc\u200b\u200bel_sha256', 'ocel_sha256'))
    replace('crates/mailweft-core/src/zones.rs', '    pub zones: Vec<SourceZone>,',
            '    pub zones: Vec<SourceZone>,\n    pub transfer_map: crate::transfer_map::TransferMap,')
    replace('crates/mailweft-core/src/zones.rs', '    let mut remaining_zones = MAX_ZONES;',
            '    let mut remaining_zones = MAX_ZONES;\n    let mut remaining_map_units = 32_768usize;')
    replace('crates/mailweft-core/src/zones.rs', '        parts.push(TextPartEvidence {', '''        let (decoded, transfer_problems) = part.decoded_checked();
        let ambiguous_encoding = part.headers().iter().filter(|header| {
            header.raw_name().eq_ignore_ascii_case("content-transfer-encoding")
        }).count() > 1;
        let transfer_map = crate::transfer_map::verified_map(
            crate::transfer_map::TransferInput {
        raw: part.raw_body(), decoded: decoded.as_ref(), text: text.as_bytes(), base: start,
        encoding: part.content_transfer_encoding().unwrap_or(""),
        problems: !problems.is_empty() || !transfer_problems.is_empty() || ambiguous_encoding,
    }, remaining_map_units,
        );
        remaining_map_units = remaining_map_units.saturating_sub(
            transfer_map.units.len() + transfer_map.source_only_ranges.len()
        );
        parts.push(TextPartEvidence {''')
    replace('crates/mailweft-core/src/zones.rs', '            zones,\n', '            zones,\n            transfer_map,\n')
    replace('crates/mailweft-core/src/zones.rs', 'schema_version: "1.1.0",', 'schema_version: "1.2.0",')
    replace('crates/mailweft-cli/src/main.rs', 'use chrono::{DateTime, Utc};', 'mod reconcile;\nuse chrono::{DateTime, Utc};')
    replace('crates/mailweft-cli/src/main.rs', '    match run(&args) {\n        Ok(json) => {', '''    let result = if args.get(1).is_some_and(|arg| arg == "reconcile") {
        reconcile::run(&args)
    } else {
        run(&args).map(|json| (json, false))
    };
    match result {
        Ok((json, partial)) => {''')
    replace('crates/mailweft-cli/src/main.rs', '            ExitCode::SUCCESS\n', '            ExitCode::from(if partial { 2 } else { 0 })\n')
    replace('crates/mailweft-cli/Cargo.toml', '[dependencies]\n', '[dependencies]\nserde = { version = "1", features = ["derive"] }\n')
    for name in ('mailweft-core', 'mailweft-cli'):
        replace(f'crates/{name}/Cargo.toml', 'version = "0.6.0"', 'version = "0.7.0"')
    # Body evidence is additive; keep original schemas and IDs stable.
    path = ROOT / 'contracts/v1/body-evidence.schema.json'
    body = json.loads(path.read_text())
    body['title'] = 'Mailweft body evidence 1.2.0'
    body['properties']['schema_version'] = {'const': '1.2.0'}
    part = body['$defs']['part']
    if 'transfer_map' not in part['required']: part['required'].append('transfer_map')
    transfer = json.loads((ROOT / 'contracts/v1/transfer-map.schema.json').read_text())
    part['properties']['transfer_map'] = {key: value for key, value in transfer.items() if key not in ('$schema', '$id', 'title')}
    path.write_text(doctor.encode(body))
    path = ROOT / 'contracts/v1/selection-report.schema.json'
    report = json.loads(path.read_text())
    receipt = json.loads((ROOT / 'contracts/v1/source-receipt.schema.json').read_text())
    report['$defs']['receipt'] = {key: value for key, value in receipt.items() if key not in ('$schema', '$id', 'title')}
    path.write_text(doctor.encode(report))


def dependency_inventory():
    obj = json.loads(subprocess.check_output(['cargo', 'metadata', '--all-features', '--format-version', '1'], text=True))
    members = set(obj['workspace_members'])
    names = {d['name'] for p in obj['packages'] if p['id'] in members for d in p['dependencies'] if d.get('source')}
    rows = []
    for p in sorted(obj['packages'], key=lambda p: (p['name'], p['version'])):
        if p['name'] not in names: continue
        if not p.get('license'): raise RuntimeError('direct dependency missing licence metadata')
        files = []
        parent = Path(p['manifest_path']).parent
        for file in sorted(parent.glob('LICEN*')):
            if file.is_file(): files.append({'name':file.name, 'sha256':hashlib.sha256(file.read_bytes()).hexdigest()})
        rows.append({'name':p['name'], 'version':p['version'], 'spdx_declared':p['license'], 'licence_files':files})
    if names != {r['name'] for r in rows}: raise RuntimeError('direct dependency inventory incomplete')
    (ROOT / 'docs/dependency-inventory.json').write_text(doctor.encode({'schema_version':'1.0.0', 'scope':'resolved_direct_dependencies_including_dev_and_optional', 'dependencies':rows}))
    lines = ['# Resolved direct dependency inventory', '', 'Generated from locked all-feature Cargo metadata and available registry source licence files. This is an inventory, not a legal opinion or a transitive vulnerability audit.', '', '| Crate | Exact resolved version | Declared SPDX expression | Source licence files hashed |', '|---|---|---|---|']
    for r in rows: lines.append(f"| {r['name']} | {r['version']} | {r['spdx_declared']} | {len(r['licence_files'])} |")
    lines += ['', 'File SHA-256 receipts: `docs/dependency-inventory.json`. Mail-parser and Rust4PM retain their upstream licences. No upstream implementation is copied into Mailweft; transfer mapping verifies parser-produced bytes and does not replace MIME parsing.', '', 'The CLI now declares serde directly; it was already resolved in the workspace. No new package is added for graph/model/cloud use. Pinning is through Cargo.lock and rust-toolchain.toml. A fresh security/advisory scan remains T00.06 work.', '']
    (ROOT / 'docs/DEPENDENCIES.md').write_text('\n'.join(lines))


def update_manifest(accepted=False):
    path = ROOT / 'conductor/task-manifest.json'
    obj = json.loads(path.read_text())
    obj['schema_version'] = '1.3.0'
    obj.setdefault('reconciled_at', datetime.now(timezone.utc).isoformat())
    obj['execution_rule'] = 'Resume one eligible task; canonical statuses and exact prerequisites drive generated Conductor context. Optional lanes never block the core.'
    by_id = {t['id']:t for t in obj['tasks']}
    baseline = 'https://github.com/edithatogo/mailweft/actions/runs/42590576579'
    gate = 'docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)'
    records = {
      'T00.01': ('completed', ['docs/DEPENDENCIES.md','docs/dependency-inventory.json'], 'Resolved direct dependencies and declared SPDX/source licence file hashes inventoried from locked Cargo metadata.', 'Transitive vulnerabilities and wider candidate evaluation are separate tasks.'),
      'T00.03': ('completed', ['AGENTS.md','.gitignore'], 'Source/capture/FOI boundaries and private-data exclusions are recorded; representative ignore rules are checked.', 'Ignore rules are not a data-loss prevention classifier; deliberately staged material still needs review.'),
      'T00.04': ('completed', ['Cargo.toml','crates/mailweft-core/Cargo.toml','crates/mailweft-cli/Cargo.toml'], 'The two-package Rust workspace exists and builds; no duplicate proposed crate is required.', 'Keep the existing mailweft-core/CLI names; do not recreate the original email-ocel scaffold.'),
      'T00.05': ('completed' if accepted else 'in_progress', ['rust-toolchain.toml','Cargo.lock'], 'Tested Rust 1.99.1 toolchain and dependency resolution are pinned.', 'Future upgrades require an explicit qualification change.'),
      'T00.06': ('in_progress', ['.github/workflows/rust-core.yml','.github/workflows/planning-checks.yml'], 'Read-only synthetic Rust, contracts, planning and Conductor checks run in CI.', 'Dedicated dependency advisory/security scanning and broader fuzz gates remain open.'),
      'T00.07': ('completed' if accepted else 'in_progress', ['scripts/conductor_doctor.py','docs/CONDUCTOR-SETUP.md','GEMINI.md'], 'Pinned gitlink, actual checkout, upstream command manifests and generated project/track context are checked.', 'Host extension registration on the user computer is explicitly unverified, not a repository prerequisite.'),
      'T01.01': ('completed', ['crates/mailweft-core/src/source.rs','crates/mailweft-core/tests/source_integrity.rs'], 'Versioned source envelopes, byte integrity receipts and counted input readers are implemented.', 'Rights/time/coverage remain caller assertions; hostile path races remain out of scope.'),
      'T01.03': ('completed', ['crates/mailweft-core/src/lib.rs','crates/mailweft-core/src/source.rs'], 'Pinned MIME parser is integrated with stable conversion/source diagnostics and negative fixtures.', 'This task does not certify every malformed MIME edge case or full parser fuzzing.'),
      'T01.04': ('in_progress', ['crates/mailweft-core/src/header_claims.rs','crates/mailweft-core/src/lib.rs'], 'From/Sender/To/Cc/Bcc/Reply-To and claimed Date/message identifiers are projected.', 'Resent/envelope evidence, unresolved groups, full address edge cases and comprehensive source spans remain.'),
      'T01.05': ('completed', ['crates/mailweft-core/src/header_claims.rs','crates/mailweft-core/tests/header_claims.rs'], 'Claimed header Date and caller observation time remain distinct; no fabricated delivery event.', 'Independent verified transport timestamps require separate source evidence.'),
      'T01.06': ('completed', ['crates/mailweft-core/src/attachments.rs','crates/mailweft-core/tests/attachment_metadata.rs'], 'Bounded source-derived attachment metadata is emitted without fetching/executing/logging payloads.', 'Content extraction, MIME adversarial corpus validation and content verification are separate work.'),
      'T01.07': ('completed' if accepted else 'in_progress', ['crates/mailweft-core/src/corpus.rs','crates/mailweft-cli/tests/corpus_reconciliation.rs'], 'Each supplied slot has an accepted/rejected outcome, deterministic manifest digest and explicit partial/withheld projection.', 'Completeness covers only the supplied selection; automatic discovery, streaming and archival attestation remain separate.'),
      'T01.08': ('completed' if accepted else 'in_progress', ['crates/mailweft-cli/src/main.rs','crates/mailweft-cli/src/reconcile.rs'], 'Offline CLI covers supplied single/batch emails, source verification and selection reconciliation with non-sensitive diagnostics.', 'The command is reconcile rather than the originally proposed ingest; no acquisition/recursion is implied.'),
      'T02.01': ('completed', ['crates/mailweft-core/src/lib.rs','crates/mailweft-core/tests/conversion.rs'], 'Domain-separated deterministic IDs have source-copy and event tests; raw identifiers are not emitted.', 'Hashes are pseudonyms, not anonymity; keyed/scoped identity is a separate hardening step.'),
      'T02.02': ('completed', ['crates/mailweft-core/src/batch.rs','crates/mailweft-core/tests/batch.rs'], 'Distinct custodians retain source copies while exact-byte message candidates can be shared.', 'Near-duplicate and content-semantic identity resolution remain separate.'),
      'T02.03': ('completed', ['crates/mailweft-core/src/header_claims.rs','crates/mailweft-core/tests/thread_candidates.rs'], 'Missing/colliding Message-ID claims do not overwrite distinct raw messages or force ambiguous parent links.', 'Full legacy-address and broader threading inference are not included.'),
      'T02.04': ('in_progress', ['crates/mailweft-core/tests/rust4pm_interop.rs','crates/mailweft-core/src/lib.rs'], 'Observed participant qualifiers and independent-import checks exist.', 'Complete role-value roundtrip, Bcc disclosure controls and empty/unknown states need stronger qualification.'),
      'T02.05': ('in_progress', ['crates/mailweft-core/src/batch.rs','crates/mailweft-core/tests/thread_candidates.rs'], 'Unique observed In-Reply-To targets are labelled candidate edges; missing and ambiguous targets abstain.', 'Reference-chain reconstruction, cycles, incomplete sources and externally adjudicated accuracy remain.'),
      'T03.01': ('in_progress', ['crates/mailweft-core/Cargo.toml','crates/mailweft-core/tests/rust4pm_interop.rs'], 'Rust4PM 0.6.2 is pinned and independently imports the emitted JSON fixtures.', 'Full supported-format/API compatibility audit remains open.'),
      'T03.03': ('in_progress', ['crates/mailweft-core/src/lib.rs'], 'Stage 1 source-copy, message, address and attachment objects/events are projected.', 'The wider mapping contract and complete typed-value parity are not yet accepted.'),
      'T03.04': ('in_progress', ['crates/mailweft-core/src/lib.rs','crates/mailweft-core/tests/rust4pm_interop.rs'], 'Portable JSON projection imports through Rust4PM without a mandatory Rust4PM runtime dependency.', 'Full OCEL typed-value/export parity and the canonical-representation decision require resolution.'),
      'T03.05': ('in_progress', ['crates/mailweft-core/src/batch.rs','crates/mailweft-core/tests/attachment_metadata.rs'], 'Time-qualified attachment attribute histories are merged and imported.', 'Current attributes are strings; heterogeneous primitive values and full roundtrips remain.'),
      'T03.06': ('in_progress', ['crates/mailweft-core/tests/rust4pm_interop.rs'], 'Observed and candidate E2O/O2O relationships have independent-import regression tests.', 'Complete value/qualifier/multiplicity roundtrip equality remains to be tested.'),
      'T05b.01': ('in_progress', ['crates/mailweft-core/src/zones.rs','contracts/v1/body-evidence.schema.json'], 'Implementation-specific BodyEvidence separates content/origin and exact/derived provenance.', 'The proposed nested email-zone interchange model is not fully implemented.'),
      'T05b.02': ('in_progress', ['crates/mailweft-core/src/zones.rs','crates/mailweft-core/tests/zoning_boundaries.rs'], 'Non-destructive deterministic quote/forward/signature/disclaimer baseline and diagnostics exist.', 'Nested attribution, multilingual coverage and independent classifier evaluation remain.'),
      'T05b.03': ('in_progress', ['crates/mailweft-core/src/transfer_map.rs','crates/mailweft-core/tests/transfer_provenance.rs'], 'Base64/QP transfer-unit maps are admitted only after parser-byte agreement; zone offsets remain conservative.', 'Charset transformation maps, HTML mappings and wider encoding fuzz qualification remain open.'),
    }
    for code, (status, files, summary, remaining) in records.items():
        task = by_id[code]
        task['status'] = status
        task['implementation'] = {'summary':summary, 'remaining':remaining, 'files':files,
            'commands':['cargo test --locked --workspace --all-targets --no-fail-fast', 'cargo fmt --all -- --check', 'python3 scripts/conductor_doctor.py --check'],
            'evidence':[baseline, gate]}
        task['allowed_paths'] = sorted(set(task['allowed_paths'] + files))
        task['action'] = 'Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.' if status != 'completed' else 'Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.'
        task['tests'] = '; '.join(task['implementation']['commands'])
    by_id['T00.04']['title'] = 'Maintain the minimal mailweft-core and CLI Rust workspace'
    by_id['T00.04']['acceptance'] = 'Locked cargo metadata and workspace tests pass for the two existing generic Rust packages; no duplicate proposed crate.'
    by_id['T00.07']['depends_on'] = ['T00.04','T00.05']
    by_id['T00.07']['acceptance'] = 'Conductor doctor validates real pinned checkout, command TOMLs, all track artifacts and synchronized status; no user-host installation claim.'
    by_id['T01.03']['depends_on'] = ['T01.01']
    by_id['T01.05']['depends_on'] = ['T01.03']
    by_id['T01.07']['acceptance'] = 'Supplied equals accepted plus rejected; deterministic manifest and explicit partial projection; no rejected source becomes an event.'
    by_id['T01.08']['title'] = 'Expose supplied-email and selection-reconciliation CLI with non-sensitive diagnostics'
    by_id['T01.08']['acceptance'] = 'Single/batch/convert-evidence/reconcile fictional fixtures run offline with documented exit codes and bounded input.'
    path.write_text(doctor.encode(obj))
    doctor.sync(ROOT)

if '--accept' in sys.argv:
    if os.environ.get('MAILWEFT_QUALIFIED') != '1': raise SystemExit('Acceptance requires the preceding explicit test gate')
    update_manifest(accepted=True)
else:
    prepare_code()
    dependency_inventory()
    update_manifest()
