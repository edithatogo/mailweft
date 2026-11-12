# Local Git and GitHub bootstrap

The delivered `.git.bundle` is a complete, self-contained **local Git repository**, including a true Git submodule reference. It is not a GitHub-hosted repository. The ZIP is for reading/inspection and does **not** preserve Git history or the submodule gitlink.

## Clone locally from the bundle

```sh
git clone email-ocel.git.bundle email-ocel
cd email-ocel
# Requires network access to GitHub:
git submodule update --init --recursive
python3 scripts/validate_scaffold.py
```

The submodule should resolve to the verified `conductor-v0.4.1` commit:

```sh
git ls-tree HEAD .agents/plugins/conductor
# 160000 commit 7a5c560a4fdf5297be58594cc37527eb12790272 ...
```

## Create an independent remote repository when authorised

A GitHub repository has **not** been created. After checking that `edithatogo/email-ocel` is the intended owner/name and installing/authenticating GitHub CLI, either create an empty repository through GitHub UI then change the origin URL, or use:

```sh
# Only run after confirming desired public/private visibility and owner:
gh repo create edithatogo/email-ocel --private --source=. --remote=github --push
```

Do not push any data directories or actual email content. Project source remains Apache-2.0; third-party datasets have independent rights. Do not fork/translate `jamtho/enron-emails` source unless licence rights have been established.

## Enable Conductor in Gemini CLI

The submodule preserves the source dependency reproducibly. Plugin activation is separate from a Git submodule checkout:

```sh
gemini extensions install https://github.com/gemini-cli-extensions/conductor --ref conductor-v0.4.1
```

The repository already contains the project's `conductor/` product context and tracks; **do not rerun setup in a way that overwrites them**. Use the first unchecked task, T00.01, with `conductor/agent-task-template.md` and `conductor/task-manifest.json`.
