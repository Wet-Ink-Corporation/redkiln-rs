# S-SEV instruments (spikes/METHOD-SEV.md)

- `make-sheet.py` — writes a blind labelling sheet and its sealed key for `stage1`, `continuation` or `relabel`, from
  F37's `results/p7.jsonl` read through the corpus object store at `CORPUS_PIN`. Seeds come from METHOD-SEV. Prints
  counts and sha256 digests only; refuses an uncommitted method, an existing sheet, and an `--out` inside a repository.
- `score.py` — validates filled sheets against their keys and computes A1, A2, A3 and the report-only comparison;
  `--decide-stage1` prints only whether stage 1 decides; `--selftest` exercises it on synthetic data.
- Both are stdlib-only and make **no dispatch**, no model call and no network call.

Order: commit METHOD-SEV → `make-sheet.py stage1 --out D:/repos/redkiln-sev` → record the key sha256 by a dated
amendment → the operator labels → corpus commit (`results/sev-f37-labels.md`) → `score.py --decide-stage1` (and the
continuation if required) → at least 48 h → `make-sheet.py relabel --labels-commit <sha>` → label → `score.py`.
