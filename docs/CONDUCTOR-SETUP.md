# Conductor setup and integrity

## Repository setup

Mailweft uses the **Gemini Conductor v0.4.1** source at immutable commit `7a5c560a4fdf5297be58594cc37527eb12790272` as a true Git submodule at `.agents/plugins/conductor`. This pin is retained; no unreviewed upstream upgrade is introduced.

The pinned setup protocol audits project artifacts, including each track's `spec.md`, `plan.md`, `metadata.json` and `index.md`. The existing project lacked track indexes and used a private metadata/status shape. The reconciliation adds upstream `track_id`, `type`, `description`, `created_at`/`updated_at`, `new/in_progress/completed` status and the registry/index links while retaining the original task IDs and requirements.

`conductor/task-manifest.json` is the canonical state. `python3 scripts/conductor_doctor.py --sync` regenerates task plans, per-track indexes/metadata/evidence summaries, the registry, setup summary and execution order. It **does not modify the manifest or mark tasks complete**. Review acceptance evidence before changing status. `[~]` means partial/in progress, not completion. Historical evidence remains in the Git history and implementation checkpoint.

The doctor checks drift, required artifacts, evidence file references, the gitlink, checked-out revision, clean upstream source, extension manifest and six TOML command definitions. It is run in read-only CI alongside task DAG and contract tests. A passing doctor verifies repository setup, not runtime feature correctness or a model's adherence to instructions.

## Fresh checkout

```sh
git clone --recurse-submodules https://github.com/edithatogo/mailweft.git
cd mailweft
python3 scripts/conductor_doctor.py --check
python3 scripts/conductor_doctor.py --status
```

For an existing clone with an uninitialized submodule:

```sh
git submodule update --init --recursive -- .agents/plugins/conductor
python3 scripts/conductor_doctor.py --check
```

Do not rerun the interactive setup command over the prepared context. The project `setup_state.json` is explicitly a generated summary, not a claim that upstream's interactive setup or a host extension installer was executed.

## Optional Gemini CLI registration on the user's development host

The pinned revision contains `gemini-extension.json`; on a host with Gemini CLI installed:

```sh
gemini extensions link ./.agents/plugins/conductor
gemini extensions list
```

Restart the CLI session and verify `/conductor:status` can resolve the project and track indexes. Linking is a host-level operation; this session does not claim to have installed or registered the extension on the user's computer. The repository doctor does not silently perform it.

Official reference: https://geminicli.com/docs/extensions/reference/
Pinned setup/new-track protocols: https://github.com/gemini-cli-extensions/conductor/tree/7a5c560a4fdf5297be58594cc37527eb12790272/commands/conductor

## Scope and limitations

Existing broad tasks may remain partial even though a working implementation slice exists. Do not infer complete requirements from test counts alone. Optional research/model/graph tracks cannot enter core release prerequisites. No source-data access or authorization is granted by Conductor setup.
