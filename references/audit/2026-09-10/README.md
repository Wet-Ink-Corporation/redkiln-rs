# The 2026-09-10 pre-implementation audit — instrument and working set

**What this is.** The machine-readable side of `references/2026-09-10-pre-implementation-audit.md`, preserved from the
session that produced it (it lived in a temporary scratchpad and would otherwise have been lost). It is the **work list**
for the remediation pass of 2026-09-11 and the **validation instrument** for the runbook that pass produces.

**Status.** Frozen record. Nothing here is edited to reflect remediation; `MASTER.json` says what the audit found at
HEAD `009a4ec`, and `runbook/PROGRESS.md` says what was done about it.

| File | What it holds |
| --- | --- |
| `MASTER.json` | The 110 concerns (`PIA-001`…`PIA-110`) with `priority`, `build_stage`, `classification`, `owner`, `anchors`, `statement`, `remediation` (verbatim from the audit), `member_ids`. **The per-concern work order.** |
| `INTERNAL-MERGED.json` | The 338 refuted survivors of the internal audit (34 finders × 34 refuters), with truth/materiality verdicts. |
| `GROUNDING-MERGED.json` | The 860 external claims re-verified live on 2026-09-10: `verdict`, `url`, `fetched`, `corpus_says`, `source_says`, `impact`, `load_bearing`, `anchors`. `rows` is the list; `load_bearing_nonconfirmed` the 43 that matter most. Drives the citation pass without re-fetching. |
| `critic-coverage.json`, `critic-method.json` | The two critics' limits (audit §8): what was never read, what never ran. |
| `coverage.json` | Per-finder coverage self-reports (capped at 1,500 characters each — see §8 C-05). |
| `reconcile.json` | The census reproducibility record: 9 of 9 scripts reproduced byte-for-byte, 43 of 44 cross-checks passing. |
| `scripts/` | The nine census scripts and the four reconciliation steps. Output paths were re-pointed from the scratchpad to `census/` here (gitignored, regenerable); nothing else changed. |

## Regenerating the census

Each `census_*.py` is READ-ONLY over the repo and writes exactly one artefact under `census/`. They take no arguments;
`REPO` is hard-coded to `D:/repos/redkiln-rs`. From the repo root:

```bash
for s in census_ids census_adr census_anchors census_constants census_known census_slices census_external census_deps census_docmap; do
  python references/audit/2026-09-10/scripts/$s.py
done
python references/audit/2026-09-10/scripts/reconcile_step2_rerun.py   # re-runs and diffs byte-for-byte
python references/audit/2026-09-10/scripts/reconcile_step3_crosscheck.py
```

`census_docmap.py` takes about three minutes; the rest seconds. The four used as the remediation's validators are
`census_ids` (id namespaces: dangling/orphan/duplicate), `census_adr` (frontmatter, supersession reciprocity, README
claims), `census_anchors` (section references and file paths) and `census_constants` (restated numbers: AGREE / VARIANT).

**Assertions that now fail by construction.** `census_anchors.py` asserts `corpus_size_194`; the corpus has grown since the
pin (this folder, `runbook/`, the new ADRs), so that assertion fails on any post-remediation tree. Read the counts, not
that flag. `census_deps.py` also reads `D:/repos/redkiln` (the frozen predecessor) and will report its HEAD.

## Baseline on the untouched tree (2026-09-11, before remediation, branch `remediation/pia-2026-09-11`)

Recorded in `runbook/PROGRESS.md` §Validation alongside the post-remediation run.
