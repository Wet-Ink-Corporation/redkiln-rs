# Runbook — Redkiln-RS from here to the end of Phase 1 · index

**What this is.** The operator-facing build runbook, one file per stage, so a build session loads the stage it is
executing and not the whole. It is the execution order for `references/2026-09-08-architecture-design.md` §0.1 and
§10.2, absorbing the pre-implementation audit of 2026-09-10 (110 concerns, all dispositioned in `PROGRESS.md` §4)
and the twenty-one decision records that pass produced (ADR-0092–0112; `spikes/DECISIONS-2026-09-11.md`). It decides
nothing the record does not: precedence is **ADR-0092**'s — accepted ADRs > the AC register > the SM register > the
spike files > this runbook — and where a stage file disagrees with any of those, the stage file is wrong.

**How to use it.** Read `00-preamble.md` once (the golden constraints, the tool preflight, the legend) and `halts.md`
once (they apply at every stage). Then open the stage file `PROGRESS.md` §1 says is next, run it top to bottom, run
its checkpoint, record the result in `PROGRESS.md` in the same commit, and move on. Never skip a checkpoint; a
checkpoint you could not run is a failed checkpoint (Halt 8), and a `[WILL EXIST]` line is deferred to the stage it
names, not passed.

## Execution order

`0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7a → 7b → 7c → 7d → 7e → 8` — Step 2 runs **sixth** (its ORDER banner says why:
it deletes the directory Step 3b writes into and Step 4 reads from). Step 9 is rollback and may run at any point.

| order | stage | file | what it delivers | entry gate | exit checkpoint | decisions it executes |
| --- | --- | --- | --- | --- | --- | --- |
| — | preamble | [`00-preamble.md`](00-preamble.md) | the golden constraints, the seven-decision table, the tool preflight, the legend | — | — | — |
| — | halts | [`halts.md`](halts.md) | the thirteen halt conditions, Halt 8's `[WILL EXIST]` carve-out | — | — | — |
| 1 | Step 0 | [`phase-0/s0-preconditions.md`](phase-0/s0-preconditions.md) | tree state re-derived, toolchain 1.98.1 pinned by intent, instrument recorded to a file, branch cut | preflight | Checkpoint 0 | the pin (DECISIONS #7) |
| 2 | Step 1 | [`phase-0/s1-commit-spike-output.md`](phase-0/s1-commit-spike-output.md) | run noise ignored, instrument file committed, tree clean — Phase-0 exit (a) | Checkpoint 0 | Checkpoint 1 | — |
| 3 | Step 3 | [`phase-0/s3-review-consumer-and-labels.md`](phase-0/s3-review-consumer-and-labels.md) | the D-FIXCONSUMER dispatch; the label files (P2 discharged, arm L outstanding); exit (c) with both rows and exit (d) stated plainly | Checkpoint 1 | Checkpoints 3a, 3b, 3c | arm L deferred (DECISIONS #6); S-BIN/S-FACET scope (ADR-0093) |
| 4 | Step 4 | [`phase-0/s4-blocking-gate.md`](phase-0/s4-blocking-gate.md) | the scorers run, the pre-registered rule applied, **ADR-0090** written and committed — the gate token | Checkpoint 3c | Checkpoint 4 | decision 1 (ADR-0090; the booleans clause, DECISIONS #2) |
| 5 | Step 4a | [`phase-0/s4a-apply-adr-0089.md`](phase-0/s4a-apply-adr-0089.md) | the registry after ADR-0089/0090/0093: 6 derived · 4 kernel · 1 declared · 9 human · 0 llm; booleans tri-valued; AC-520 fails closed | Checkpoint 4 | Checkpoint 4a | none (execution of three records) |
| 6 | Step 2 | [`phase-0/s2-corpus-extraction.md`](phase-0/s2-corpus-extraction.md) | `redkiln-corpus` at a pinned SHA with the predecessor bundled (118 refs); `CORPUS_PIN`; AC-465 enforceable — Phase-0 exit (b) | Checkpoint 4a | Checkpoint 2 | the substrate (ADR-0100 / D-N) |
| 7 | Step 5 | [`phase-1/s5-workspace-ci-rk-types.md`](phase-1/s5-workspace-ci-rk-types.md) | workspace, five-gate CI, `rk-types` with the golden snapshot under ADR-0095's rule, `StepError`, the typed repro, the companion kinds | 🚦 gate + Checkpoint 2 | Checkpoint 5 | decisions 6 and 7 (ADR-0104; DECISIONS #3) |
| 8 | Step 6 | [`phase-1/s6-store-and-git.md`](phase-1/s6-store-and-git.md) | S-GIX/a, /b, /c run first; **ADR-0091**; the hybrid under ADR-0098/0105; `rk-store` under ADR-0097/0109 | Checkpoint 5 | Checkpoint 6 | decisions 2 and 3 (ADR-0091) |
| 9 | Step 7a | [`phase-1/s7a-ports-and-fakes.md`](phase-1/s7a-ports-and-fakes.md) | `rk-ports` to ADR-0099's shape (F41 is the reference), the fakes, the parity assertion | Checkpoint 6 | Checkpoint 7a | — |
| 10 | Step 7b | [`phase-1/s7b-kernel.md`](phase-1/s7b-kernel.md) | `rk-kernel`: the selection engine's semantics, both facet lints, the rigor-monotonicity lint, the grammar to ADR-0103 (F42), 7b-ii the three gates | 🛑 STOP banner | Checkpoint 7b | — |
| 11 | Step 7c | [`phase-1/s7c-adapters.md`](phase-1/s7c-adapters.md) | four real adapters; the Claude adapter's containment, cancel, denylist, tick arithmetic, per-profile argv | Checkpoint 7b | Checkpoint 7c | DECISIONS #16, #19 |
| 12 | Step 7d | [`phase-1/s7d-cli-and-migrate.md`](phase-1/s7d-cli-and-migrate.md) | `rk-cli` (one idempotent `run`, `answer`, `doctor`, `explain`, `init`); `redkiln-migrate` v0 as defined | Checkpoint 7c | Checkpoint 7d | decision 4 (`doctor`'s `N`) |
| 13 | Step 7e | [`phase-1/s7e-pack-authoring.md`](phase-1/s7e-pack-authoring.md) | `.redkiln/`, the derived registry, row dispositions, `stale_policy` with its warrant, the lattice re-baselined | Checkpoint 7d | Checkpoint 7e | decision 5 (DECISIONS #4) |
| 14 | Step 8 | [`phase-1/s8-exit-criterion.md`](phase-1/s8-exit-criterion.md) | the seven exit checks — (1) in both lanes, (6) on both lints — and the standing prohibitions | Checkpoint 7e | Checkpoint 8 | — |
| any | Step 9 | [`s9-rollback.md`](s9-rollback.md) | clean, total rollback; the salvage first | a decision to abandon | Checkpoint 9 | — |

## Milestones

| milestone | reached when | what it means |
| --- | --- | --- |
| **M0 — Phase 0 closed** | Checkpoint 2 passed (after 0, 1, 3, 4, 4a) | the programme's records are committed, the gate is discharged by a record, the registry is honest, the corpus is pinned |
| **M1 — the foundation builds** | Checkpoint 5 passed | a workspace with CI and the typed core, every schema inside the vendor's contract |
| **M2 — state is durable** | Checkpoint 6 passed | git is the tier-1 store and the store's contract is rules, not lists |
| **M3 — the kernel decides** | Checkpoint 7b passed | selection, gates and lints exist and fail closed |
| **M4 — the system runs** | Checkpoint 7e passed (after 7c, 7d) | four adapters, a CLI, a migrator and a loadable pack set |
| **M5 — Phase 1 exits** | Checkpoint 8 passed | Redkiln-RS ran its own fix lane and authored a Phase-2 story |

## Shared conventions (stated once here, restated in each file's header)

- **Shell:** every block is POSIX, run in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.
- **`§`:** a bare `§N.M` refers to the file it appears in; another stage's section is `sN §N.M`; other documents
  are prefixed (`design §10.5`, `METHOD-P §2.4`, `AC §12.10`).
- **Tags:** `[WILL EXIST]` (deferred, re-run at the named stage), `[measured]`, `[documented]`, `[inferred]`; the four
  fault names — saturated, powerless, not-measurable, not-answerable — are used and never blurred.
- **Checkpoints:** `command   # → expected result`; BEFORE assertions beside AFTER ones where the audit found a line
  that was green before the step ran (PIA-035).
- **Acceptance sets:** an explicit range minus named exclusions, each with its reason.
- **Records:** decisions live in `references/adr/` (never edited; superseded) and `spikes/DECISIONS-2026-09-11.md`;
  corrections to frozen ADR evidence lines in `references/adr/ERRATA.md`; open questions as design §10.5 rows.

## Where things went

`RUNBOOK.md` at the repository root is a stub pointing here. The 2026-09-10 audit's `RUNBOOK.md:NNNN` anchors
resolve at git pin `009a4ec` (`git show 009a4ec:RUNBOOK.md`); `PROGRESS.md` §4 maps every PIA concern to the stage
file and record where its remediation landed.

## Validation of this runbook (the "validated" in its name)

Run from the repository root after any change to `runbook/`:

```bash
# 1. the corpus's own validators (the audit's instrument, kept in-tree)
for s in census_ids census_adr census_anchors census_constants; do python references/audit/2026-09-10/scripts/$s.py | tail -2; done
# 2. no stale claim survives in a live document (each hit must be a quoted historical figure beside its correction)
git grep -nE 'Step 7b pack authoring|both are 11|Do not cite them as a platform constraint|redkiln run --resume <|nursery = "warn"$|1\.97\.1 or later|thirteen area files|rk-adapter-node\b' -- runbook references/2026-09-08-*.md references/adr/README.md
# 3. every § reference in runbook/ resolves to a heading in the file it names
python - <<'PY'
import re,glob
files={f.replace('\\\\','/'):open(f,encoding='utf-8').read() for f in glob.glob('runbook/**/*.md',recursive=True)}
bad=0
for f,s in files.items():
    heads=set(re.findall(r'^#+ .*?(\\d+[a-z]?(?:\\.\\d+)?)',s,re.M))
    for m in re.finditer(r'(?<![A-Za-z-])§(\\d+[a-z]?\\.\\d+)',s):
        pre=s[max(0,m.start()-14):m.start()]
        if re.search(r'(design|METHOD\\S*|README|brief|AC|SM|s\\d+[a-z]?)\\s*$',pre): continue
        if m.group(1) not in heads and not any(h.startswith(m.group(1)) for h in heads): bad+=1; print(f, m.group(0))
print('unresolved bare § refs:', bad)
PY
# 4. the checkpoint dry-run: every checkpoint line whose target exists today, executed, with its output recorded in PROGRESS.md §6
```
