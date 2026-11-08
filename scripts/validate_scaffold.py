#!/usr/bin/env python3
"""Documentation-only zero-dependency scaffold checker (not a Python runtime)."""
from pathlib import Path
import json
import re
import sys
ROOT=Path(__file__).resolve().parent.parent
errors=[]
required=["README.md","AGENTS.md",".gitmodules","docs/CONTRACT.md","docs/REQUIREMENTS.md","docs/DESIGN.md","docs/DATASETS.md","docs/VALIDATION.md","conductor/index.md","conductor/tracks.md"]
for name in required:
 if not (ROOT/name).exists(): errors.append(f"missing: {name}")
for path in (ROOT/"contracts/v1").glob("*.json"):
 try:
  obj=json.loads(path.read_text())
  assert obj.get("$schema")=="https://json-schema.org/draft/2020-12/schema"
  assert obj.get("additionalProperties") is False
 except Exception as exc:errors.append(f"bad schema {path.name}: {exc}")
for p in (ROOT/"conductor/tracks").glob("*/metadata.json"):
 try:
  meta=json.loads(p.read_text());t=p.parent
  assert meta['tasks']
  assert (t/'plan.md').exists() and (t/'spec.md').exists() and (t/'evidence.md').exists()
  plan=(t/'plan.md').read_text()
  for code in meta['tasks']:
   if not re.search(r'\*\*'+re.escape(code)+r'\s+—',plan):errors.append(f"task missing {code}")
 except Exception as exc:errors.append(f"bad track {p}: {exc}")
if errors:
 print("SCAFFOLD FAILED:",*errors,sep="\n- ")
 sys.exit(1)
print("SCAFFOLD OK: required docs, JSON schemas and track/task references")
