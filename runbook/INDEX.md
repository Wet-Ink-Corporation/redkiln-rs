# Runbook — Redkiln-RS from here to the end of Phase 1 · index

**What this is.** The operator-facing build runbook, one file per stage, so a build session loads the stage it is
executing and not the whole. It is the execution order for `references/2026-09-08-architecture-design.md` §0.1 and
§10.2, absorbing the pre-implementation audit of 2026-09-10 (110 concerns, all dispositioned in `PROGRESS.md` §4)
and the twenty-three decision records that pass produced (ADR-0092–0114; `spikes/DECISIONS-2026-09-11.md`). It decides
nothing the record does not: precedence is **ADR-0092**'s — accepted ADRs > the AC register > the SM register > the
spike files > this runbook — and where a stage file disagrees with any of those, the stage file is wrong.

**How to use it.** Read `00-preamble.md` once (the golden constraints, the tool preflight, the legend) and `halts.md`
once (they apply at every stage). Then open the stage file `PROGRESS.md` §1 says is next, run it top to bottom, run
its checkpoint, record the result in `PROGRESS.md` in the same commit, and move on. Never skip a checkpoint; a
checkpoint you could not run is a failed checkpoint (Halt 8), and a `[WILL EXIST]` line is deferred to the stage it
names, not passed.

**What redkiln-rs is, and why it is a rewrite — read this once if you have not read the RFC.** Redkiln is
backlog-as-code: the work items, the process that moves them and the evidence that they are done all live in the
repository as files, and an agent harness executes that process against them. The predecessor is a Node/TypeScript
Claude Code plugin at `D:/repos/redkiln`, frozen at `47faec9f` and bundled into the corpus at Step 2. The RFC of
2026-08-31 separated two bets and recommended only the first: **invert control** — Rust owns the loop and spawns
`claude -p` per step — on the finding that roughly two-thirds of the workflow layer is compensation for the Workflow
runtime's sandbox and *disappears* rather than ports, and that ~50% of the command layer is orchestration logic
living as prose the model must re-execute correctly on every run [measured, RFC §0]. The design of 2026-09-08 took
the stronger clean-break form (ADR-0002): **one Rust binary that owns the loop**, with a **deterministic kernel** —
selection, gates, budgets, retry, replay — that holds no non-determinism of its own and dispatches one harness
process per activity; durable state as git trailers first and an append-only event log second; human gates as
durable marks, never a process that waits; and every agent output a typed value of which markdown is a *rendering*
and never a source (design §0). The kernel is hexagonal along **five named ports** — event store, harness, tracker,
docs site, toolchain (ADR-0003) — each a sync trait with a fake, every `.await` confined to an adapter.

**Why that shape matters to you while you build it.** The point of the inversion is that a decision becomes a thing
you can write a test about. Every constraint in `00-preamble.md`, every halt in `halts.md` and every checkpoint in
these stages exists because a decision that could not be replayed or falsified had already cost this programme
something measurable. When a stage file tells you to assert rather than read, that is the same bet, one level down.

**Where the vision lives**, when a stage does not say enough: `references/2026-08-31-rust-inversion-RFC.md` and its
amendment `references/2026-09-02-rfc-amendment.md` (why at all), `references/2026-09-05-architecture-proposal.md`
(the shape at revision 4), and the binding `references/2026-09-08-architecture-design.md` (§0 in one paragraph,
§0.1 decided-vs-blocked, §2 the principles the data imposes). **Phase 1 ends when Redkiln-RS runs its own fix lane
and authors a Phase-2 story** — both lanes, four adapters (ADR-0011, ADR-0094) — which is what self-hosting means
here.

## Execution order

`0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7a → 7b → 7b-ii → 7c → 7d → 7e → 7f → 7g → 7h → 7i → 8` — Step 2 runs **sixth** (its ORDER banner says
why: it deletes the directory Step 3b writes into and Step 4 reads from). Step 9 is rollback and may run at any point.
**Step 7b-ii became its own stage file on 2026-09-13**: the three gates were 120 criteria in force carried inside
`s7b` as one paragraph with six checkpoint assertions, and they now have their own acceptance set and their own
**Checkpoint 7b-ii**. Step 7b's exit checkpoint is unchanged; Step 7c's entry gate moved from Checkpoint 7b to
Checkpoint 7b-ii in the same change, or the execution order stops resolving.

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
| 9 | Step 7a | [`phase-1/s7a-ports-and-fakes.md`](phase-1/s7a-ports-and-fakes.md) | `rk-ports` to ADR-0099's shape (F41 is the reference), the fakes, the parity assertion | Checkpoint 6 | Checkpoint 7a | **ADR-0118** — a poisoned store lock halts the run, closing the residual ADR-0115:135-138 named and assigned to this stage *(this column read `—` until 2026-09-13; the residual was there all along and no file said the stage met it)* |
| 10 | Step 7b | [`phase-1/s7b-kernel.md`](phase-1/s7b-kernel.md) | `rk-kernel`: the crate skeleton and the determinism gate, the grammar to ADR-0103 (F42), the registry loader and both facet lints, the rigor-monotonicity lint, the seven-step selection function, the runtime, the golden logs and the single `CORPUS_PIN` move | **Checkpoint 7a + 🛑 STOP banner** | Checkpoint 7b | **ADR-0119** — AC-045's key is a fourth, selection-level key derived in `rk-kernel::select` (written 2026-09-13 for this stage; `00-preamble.md`:47-49) *(the same 00-preamble decision-table drift `s7a` §7a.1 records for ADR-0118 applies here and is owed)* |
| 11 | Step 7b-ii | [`phase-1/s7b-ii-gates.md`](phase-1/s7b-ii-gates.md) | the three gates, all `rk-kernel`'s: the human-gate record and its one write-tree derivation, the review classifier and policy table, re-execution, quarantine, the advisory sink, replay determinism, the verify harness and the AC evidence ledger | Checkpoint 7b | Checkpoint 7b-ii | decision 6 (**ADR-0104**, author-declared per-AC mutants — `00-preamble.md`:36 names 7b-ii as one of its two sites) |
| 12 | Step 7c | [`phase-1/s7c-adapters.md`](phase-1/s7c-adapters.md) | four real adapters; the Claude adapter's containment, cancel, denylist, tick arithmetic, per-profile argv | **Checkpoint 7b-ii** | Checkpoint 7c | DECISIONS #16, #19 |
| 13 | Step 7d | [`phase-1/s7d-cli-and-migrate.md`](phase-1/s7d-cli-and-migrate.md) | `rk-cli` (one idempotent `run`, `answer`, `status`, `explain`, `doctor` / `doctor --rebuild`, `catalog search`, `init`, `lint` — DECISIONS-2026-09-18 D18-15); `redkiln-migrate` v0 as defined | Checkpoint 7c | Checkpoint 7d | decision 4 (`doctor`'s `N`) |
| 14 | Step 7e | [`phase-1/s7e-pack-authoring.md`](phase-1/s7e-pack-authoring.md) | `.redkiln/`, the derived registry, row dispositions, `stale_policy` with its warrant, the lattice re-baselined | Checkpoint 7d | Checkpoint 7e | decision 5 (DECISIONS #4) |
| 15 | Step 7f | [`phase-1/s7f-conformance-and-records.md`](phase-1/s7f-conformance-and-records.md) | the conformance gate over the design's closed sets (three lints, ARMED) and the records it needs: **ADR-0073 superseded in part by ADR-0132** so `ImplReport` is the sixth toolchain-keyed kind, the `rate_limit_reading` producer named, RT-19's cost unit settled | Checkpoint 7e | Checkpoint 7f | decision: `ImplReport` keyed (ADR-0132, superseding ADR-0073 in part, 2026-09-21) |
| 16 | Step 7g | [`phase-1/s7g-step-executor.md`](phase-1/s7g-step-executor.md) | the step executor — design §4.5's **five closed step kinds** each executed as such, and the `Toolchain` port's first call site; the `.redkiln/` fills the run demands, authored by selection | Checkpoint 7f | Checkpoint 7g | **ADR-0134** (the port lint's scope), **ADR-0135** (the `tool` verb routing), **ADR-0136** (an unresolvable step guard), **ADR-0137** (the real toolchain adapter) and **ADR-0138** (how a step list reaches the runtime) — plus **ADR-0139**, **ADR-0140** and **ADR-0141**, the three that supersede 0137 clause 8, 0137 clause 5 and 0138 clause 4 in part after their own first execution. — plus **ADR-0142** (the executor evaluates a guard against the selection's own context), **ADR-0143** (a second runnable `agent` step refuses), **ADR-0144** (a step failing AFTER a committed dispatch refuses by name) and **ADR-0145** (a catalog `gate` step opens the selection's own gate). **Twelve**, flipped **D21-9 … D21-21**. *(This cell read “none (execution of design §4.5)” until 2026-09-22; the stage file said the same and both were wrong — see `s7g`'s own corrected paragraph.)* |
| 17 | Step 7h | [`phase-1/s7h-lane-completes.md`](phase-1/s7h-lane-completes.md) | the lane completes: `ctx.successor` and `AdvanceStage`, the **two trailer axes** minted where design §5's table puts their producers, and the review/verify modules' call site | Checkpoint 7g | Checkpoint 7h | none (execution of design §5 and ADR-0105 d3); **ADR-0101 clause 5 binds** |
| 18 | Step 7i | [`phase-1/s7i-cost.md`](phase-1/s7i-cost.md) | `redkiln cost` — the rate-limit reading journalled so ADR-0110's `resetsAt` grouping key exists, and the three renderings over `throttle::attribute` | Checkpoint 7h | Checkpoint 7i | none (execution of `s7f` §7f.3 and §7f.4) |
| 19 | Step 8 | [`phase-1/s8-exit-criterion.md`](phase-1/s8-exit-criterion.md) | the **six** exit checks — (1) in both lanes, (6) on both lints — and the standing prohibitions *(this read "the seven exit checks" until 2026-09-15: design §10.2's **(5) is WITHDRAWN**, its number **retired** and **nothing renumbered**, so the sequence is (1) (2) (3) (4) (6) (7) — `DECISIONS-2026-09-14 D-14`, and `s8`:15-19 carries the same count)* | Checkpoint 7i *(this read "Checkpoint 7e" until 2026-09-22: the four stages `7f`–`7i` were inserted before Step 8 with a correct `7e → 7f → 7g → 7h → 7i` chain and Step 8’s own entry gate was never re-pointed off `7e`, so this cell contradicted the execution-order line in this same file at :45, the identical line in `00-preamble.md`:296, and `s8`’s own header table, all three of which read `7i`)* | Checkpoint 8 | — |
| any | Step 9 | [`s9-rollback.md`](s9-rollback.md) | clean, total rollback; the salvage first | a decision to abandon | Checkpoint 9 | — |

## Milestones

| milestone | reached when | what it means |
| --- | --- | --- |
| **M0 — Phase 0 closed** | Checkpoint 2 passed (after 0, 1, 3, 4, 4a) | the programme's records are committed, the gate is discharged by a record, the registry is honest, the corpus is pinned |
| **M1 — the foundation builds** | Checkpoint 5 passed | a workspace with CI and the typed core, every schema inside the vendor's contract |
| **M2 — state is durable** | Checkpoint 6 passed | git is the tier-1 store and the store's contract is rules, not lists |
| **M3 — the kernel decides** — **REACHED 2026-09-17** (Checkpoint 7b `ddc57be`; Checkpoint 7b-ii `164e212`) | **Checkpoint 7b-ii passed** (after Checkpoint 7b) | selection, gates and lints exist and fail closed — **and the milestone needs both checkpoints, which is why it names the later one**: Checkpoint 7b attests the selection engine, the grammar, the three registry/facet lints and the golden-log replay; Checkpoint 7b-ii attests the human, review and verify gates. *(This row read "Checkpoint 7b passed" until 2026-09-13, when the gates were split into `s7b-ii`; the row's meaning is unchanged and the gate that carries it moved.)* **ADR-0101 clause 5 bounds what this row may be read as**: until S-SUBTLE-CB runs, no document may claim the review gate is guarded against false approval |
| **M4 — the system runs** | Checkpoint 7e passed (after 7c, 7d) | four adapters, a CLI, a migrator and a loadable pack set |
| **M5 — Phase 1 exits** | Checkpoint 8 passed | Redkiln-RS ran its own fix lane and authored a Phase-2 story |

## Where the human is needed

Every entry gate in the table above names a checkpoint or a banner. **None of them names a person**, and three
obligations are outstanding that no stage gate will stop you on. They are tracked in `PROGRESS.md` §1 and §2, not
here; this table exists so an operator planning a session knows where it will block on someone rather than something.

| Owed | Since | What is owed | Where it is recorded | What proceeds without it |
| --- | --- | --- | --- | --- |
| **D-FIXCONSUMER countersignature** | Step 3a, 2026-09-12 | The runtime probe is `[WILL EXIST]` and could not run, so the Workflow runtime's behaviour is **UNVERIFIED**. A forced deferral, not an adjudicated one: countersign it, or schedule the dispatch. | `DECISIONS-2026-09-09 #39`; `PROGRESS.md` §1 | Steps 4–7e. **Step 8 check (1)** consumes the fix lane and is where it bites. |
| **Arm L labels** | Step 3b, by decision | Arm L is outstanding and is labelled later, at a new corpus pin (DECISIONS-2026-09-11 #6). It reports **DEFERRED** through the three-state form, never a pass. | `DECISIONS-2026-09-11 #6`; `s2` Checkpoint 2; `s3` §3b | Everything. It is a decision, not a blocker. |
| **M-06 Part 2** | before Step 5 | The 30 seeded P2 operator labels. C-01 and C-02 each got a "Before this step, not after" paragraph at their consumer stage; this one is **tracker-enforced only**. | `PROGRESS.md` §2 / §5 | Step 5 does not gate on it. Read the tracker before starting Phase 1. |

**And the decisions the runbook may not take for you.** `00-preamble.md`'s table names seven this runbook executes;
the ones still *open* are listed there with their record. A stage that meets an open decision **stops** — it does
not choose a default. `doctor`'s window `N` (Step 7d) and the `git` version floor (Step 6.2) are the two that will
halt a build session outright if they are still open when you reach them.

Twenty AC rows across the Phase-1 stages are **human-gated** and are excluded from the stages' acceptance sets by
name and reason; they are not in this table because they gate the criterion, not the build.

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

Run from the repository root after any change to `runbook/`. **Checks 3 and 4 exit non-zero on failure and are
the only two that do** — check 1 pipes each validator to `tail` (read its output), and check 2 is an inverted
grep whose pass condition is no hit, so read both rather than trusting their status. A check that only prints
is the Halt 8 shape this very block carried until 2026-09-12, when check 3's regexes were found
double-escaped — matching 0 headings and 0 references, and so printing `0` on every tree. The two checkers now
live in `runbook/tools/` rather than in a `python - <<'PY'` heredoc, because escaping a regex through a
heredoc is what broke it; each carries a corpus-wide self-test that fails if its patterns stop matching.

```bash
# 1. the corpus's own validators (the audit's instrument, kept in-tree)
for s in census_ids census_adr census_anchors census_constants; do python references/audit/2026-09-10/scripts/$s.py | tail -2; done
# 2. no stale claim survives in a live document (each hit must be a quoted historical figure beside its correction)
git grep -nE 'Step 7b pack authoring|both are 11|Do not cite them as a platform constraint|redkiln run --resume <|nursery = "warn"$|1\.97\.1 or later|thirteen area files|rk-adapter-node\b' -- runbook references/2026-09-08-*.md references/adr/README.md
# 2b. strings the 2026-09-12 review pass retired. Scope is `-- runbook` ONLY, minus this file: ADR-0089:248's
#     "roughly 60" is frozen body covered by ERRATA E-0089-2, and the audit, the design's §12 "was" column and
#     the proposals carry several of these as sanctioned historical quotes. This file is excluded because it
#     CONTAINS the patterns and would otherwise always match itself — the same shape as a guard that cannot fail.
#     Expected: exactly ONE hit, PROGRESS.md §6's 2026-09-12 row, which quotes each retired string beside its
#     correction. A second hit is drift; read it against the rule in check 2.
git grep -nE 'roughly \*\*60 positives\*\*|MCC \+0\.978|Today that is \*\*2\.1\.266|clippy -D warnings|^resolver = "2"|--stability-record <path>|the line reads `FILL-ME`|\(AC-104, SM-035\)|0\.00 over 11 positives' -- runbook ':!runbook/INDEX.md'
# 3. every bare section reference resolves to a heading in its own file, or names its document
python runbook/tools/check-refs.py    # exit 0; prints the ref and heading counts it actually matched
# 4. every checkpoint line can fail (Halt 8 shapes (a)-(h)), with the justified exceptions enumerated
python runbook/tools/check-checkpoints.py    # exit 0; prints flagged-vs-allowed
# 5. the checkpoint dry-run: every checkpoint line whose target exists today, executed, with its output
#    recorded in PROGRESS.md §6. Still operator-driven — it has no single command, because "whose target
#    exists today" moves with the tree. Record which lines you ran and which you could not, by stage.
```
