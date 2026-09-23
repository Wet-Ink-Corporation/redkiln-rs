---
id: "0156"
title: "The two axes are asserted over **two different corpora, for a measured reason**: `Story:` by a REAL `redkiln run` on the shipped tree — reachable because **`triage` has ZERO catalog activities** and `bug-standard` declares no `[stage.triage]` block, so the plan is empty, `bind_nodes` reads no schema and the first boundary is crossed with nothing in the way — and `Slice-Verdict:`, the `review`/`verify` call site and `Exit::FindingsBlock` over a **purpose-built fixture pack** whose review activity is `agent` + `emit` only. The shipped pack's review stage is unreachable for **six independent, measured absences**, one of which (`${…}` step-output resolution) the tracker records as OWED to another stage. And Checkpoint 7h's `grep -q '^Slice-Verdict:'` guard is **already GREEN on the untouched tree** — satisfied by wrapped PROSE in commit `ba46ed9` over a stale `rk-run/` branch that is 0 commits ahead of the build branch — so it is re-stated against git's own trailer parser and scoped to the run's own commits before it is ever run"
status: proposed
date: 2026-09-22
legacy_id: none
source: .redkiln/catalog/ (102 activities; the per-stage census, `triage` = 0); .redkiln/packs/bug-standard.toml (no `[stage.triage]` block; `[stage.review]` at 63-76); .redkiln/catalog/code-review.toml (24-58, the five steps) and adversarial-review.toml; crates/rk-cli/src/verbs/run.rs (`bind_nodes`'s schema read at 1735-1739; the stage read at 550; `terminal_read` at 1257-1331); crates/rk-kernel/src/runtime/step.rs (`MAPPED_VERBS` at 190, the refusal at 310-315); crates/rk-kernel/src/replay.rs (1760-1763, ADR-0142's measurement — 75 of 121 guards evaluable, the 94 `check` exprs untouched); crates/rk-kernel/src/review.rs and src/verify.rs (built, tested, zero `src/` callers); crates/rk-cli/src/exit.rs (44-46, `FindingsBlock = 4` and its *"its producer is the kernel's, built at 7b-ii"*); runbook/phase-1/s7h-lane-completes.md (§7h.4's block); references/adr/0101 (clause 5); references/adr/0147 (the instrument/executor split criterion)
supersedes: []
superseded_by: []
related: ["0101", "0136", "0142", "0147", "0148", "0152", "0155", "0157", "0158"]
evidence: []
consumers: []
---

## Context and problem

Checkpoint 7h asserts, in one block, that *"**Both trailer axes are present** on that run's branch"* and
that *"**A lane runs intake → landed commit**"*. Re-deriving what a run can actually do on this tree at
`fc45eeb` splits that bullet in two, and the split is not a matter of taste.

**The advance half is reachable today, for a reason nobody had written down.** A census of
`.redkiln/catalog/*.toml` by declared stage gives, over 102 activities:

| triage | discovery | analysis | specification | design | planning | implement | verify | review | release | operate | closeout |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **0** | 8 | 13 | 8 | 11 | 7 | 14 | 10 | 9 | 7 | 7 | 8 |

**`triage` has zero activities**, and `bug-standard.toml` carries **no `[stage.triage]` block at all** —
no `baseline`, no `require` rows, therefore no gate. The shipped fix item
(`.redkiln/items/rk-fix-doctor-test-count.toml`) is at `stage = "triage"`. So its selection is empty,
`bind_nodes` walks zero nodes and reads zero schemas, `next_action` finds nothing to dispatch and no gate
to open, and the plan is exhausted on the first iteration. **That is also the mechanical explanation for
§7h's own premise that a landed run leaves zero commits** — the premise was measured but its cause was
not, and the cause is what makes `0152`'s advance demonstrable end to end.

**The review half is not reachable, for SIX independent absences.** Each measured, and ordered as a lane
meets them.

⚠️ **This record was first drafted at FIVE, and blocker (0) was found by RUNNING the binary.** It is the
one a lane hits first — `bind_nodes` reads a role's **card** before it reads the kind's **schema** — so
the schema absence below is second, not first, and had never actually been reached by anything.

0. **Every shipped card lacks `permission_mode`.** Measured: **8 of 8** files under `.redkiln/cards/`
   (`architect`, `implementer`, `investigator`, `operator`, `planner`, `releaser`, `reviewer`,
   `spec-author`) carry no such key, and the binder refuses by name — *"card investigator: no
   `permission_mode` — `--permission-mode` takes an explicit value from the card and there is no
   implicit default (AC-214)"*. This is the exit **3** a real `redkiln run` now stops at, one stage
   after the boundary it takes, and it blocks **every** agent dispatch over the shipped pack, not only
   the review stage's.
1. **`.redkiln/schemas/` does not exist** — **not yet reached by any run**, because (0) fires first.
   `bind_nodes` reads
   `.redkiln/schemas/<provides[0]>.json` (`run.rs:1735-1739`) and returns `Exit::ConfigError` if it
   cannot. `ls .redkiln/` lists eleven entries and `schemas` is not among them. The catalog would need
   **61** distinct kinds; the review stage alone needs five — `ChangeRecord`, `DesignVerdict`, `PRRReport`,
   `ReviewVerdict`, `SecurityReviewVerdict`.
2. **`.redkiln/processes/` is empty** — zero entries — so the `question = "review.blocking"` that
   `code-review` and `adversarial-review` both declare resolves to nothing.
3. **`toolchain:verdict_record` is not a mapped verb.** `MAPPED_VERBS` is
   `["build", "change_summary", "lint", "test"]` (`runtime/step.rs:190`) and every other verb is refused
   by name (`:310-315`), so each review activity's `record` step yields `StepError::Unsupported` → exit
   **19**. `adversarial-review`'s `toolchain:shallow_clone` is unmapped too.
4. **No item is at `stage = "review"`**, and the stage is read verbatim from the item record
   (`run.rs:550`: *"the stage is read once, from the record, and is not defaulted"*).
5. **`${review.outputs}` does not resolve.** ADR-0142's own measurement (`replay.rs:1760-1763`) is that
   75 of 121 step guards became evaluable and that it *"repairs none of the 94 `check` exprs, which read
   step outputs"*. `code-review`'s `verdict` step is one of those 94. Step-output resolution is recorded
   as **OWED**, and not to this stage.

Blockers 0–3 and 5 are absences of things `7h` was not scoped to author; 5 in particular is another
stage's recorded debt, and taking it would be closing OWED work silently. Blocker 0 is the one a lane
actually stops at, and it is authored registry content (eight card files), not code.

**Separately, the checkpoint cannot currently fail.** With `FIX_ITEM=rk-fix-doctor-test-count`:

- `refs/heads/rk-run/rk-fix-doctor-test-count` **already exists** and
  `git rev-list --count build/phase-0-to-1..rk-run/rk-fix-doctor-test-count` is **0** — a stale leftover
  pointing into mainline history. So the block's `|| FAIL: no rk-run/$FIX_ITEM ref` guard passes without a
  run.
- `grep -q '^Slice-Verdict:'` **passes**, on one line inside commit `ba46ed9`'s body (*"H2/H3 RESULT:
  suspension is lossless…"*): the wrapped prose `Slice-Verdict: does not register here yet could still
  invalidate a`. Git does not agree it is a trailer — `%(trailers:key=Slice-Verdict)` is empty for every
  commit on that branch.

`00-preamble.md`'s PIA-035 rule requires a checkpoint to carry a BEFORE assertion that **fails on the
untouched tree**. That guard cannot.

## Decision drivers

- `0147` already fixed this stage-boundary criterion: a finding that is the *instrument* is carried, and a
  finding that is the *thing under test* is repaired. An absence of authored registry content is neither —
  it is scope, and scope is decided, not discovered at the checkpoint.
- A guard satisfied by prose is worse than a missing guard, because it reports evidence it does not have.
- The `review`/`verify` modules are built and tested with **zero** `src/` callers; what 7h owes them is a
  call site, and a call site can be exercised without 61 schema files.
- `Exit::FindingsBlock = 4` needs no table change: the row is already in `Exit::ALL` and in the committed
  snapshot. Only a producer is missing.
- `0101` clause 5 binds hardest exactly here, at the stage that wires the gate.
- A fixture pack is the tree's own established instrument for a run that cannot be driven off shipped
  content: `crates/rk-cli/tests/fixtures/` already holds `pack-min` and nine exit-code fixtures, and every
  binary test in `rk-cli` runs over them.

## Considered options

**1. Assert both axes on the shipped tree.** Requires authoring the missing schemas, populating
`.redkiln/processes/`, mapping `verdict_record`, moving an item to `review`, and closing step-output
resolution. It is the strongest end state and it absorbs another stage's OWED work into a stage whose
declared scope (§7h.1–§7h.3) names none of it. Rejected as scope, with the route recorded.

**2. Close at the scope boundary and record Checkpoint 7h as partially unsatisfiable.** Honest, and it
leaves `s7i`'s stated premise (*"the first blocker is cleared by `s7h`"*) unmet and the `Slice-Verdict:`
axis with no producer at all — so the ARMED lint could not be discharged either. Rejected.

**3. Split the corpora (chosen).** `Story:` on the shipped tree, where it genuinely works;
`Slice-Verdict:` plus the review call site plus `Exit::FindingsBlock` over a fixture pack built for it.
Both axes get a real producer and a real assertion; nothing is claimed of the shipped pack that is false.

## Decision

**1.** The `Story:` axis is asserted by a **real `redkiln run`** on the shipped tree, over
`rk-fix-doctor-test-count`, crossing the `triage → analysis` boundary that `0153`'s declared order
supplies.

**2.** The `Slice-Verdict:` axis, the `review`/`verify` call site and `Exit::FindingsBlock` are asserted
over a **fixture pack** whose review-stage activity declares **`agent` + `emit` steps only**: no `check`
step (so no `${…}` reference, so blocker 5 is not in the path) and no `gate` step (so no `question`, and
carried finding (22)'s missing `gate_expiry` class is not in the path either).

**3.** The wiring under test is the post-dispatch half: the agent result →
`review::ingest::findings_from_result_event` → `review::confirm::confirm_with` → append
`Confirmed::events` → the `Slice-Verdict:` trailer → `Exit::FindingsBlock` from `block_set()`. `review/`
returns `Vec<Event>` and appends nothing itself, so the caller owns the append and the exit mapping.

**4. The six blockers are recorded by number** as the reason the shipped review stage is unreachable, and
**`0 of 102` shipped activities completing a committing walk stands unchanged** by this stage.

**5.** Checkpoint 7h's block is re-stated **before it is run**, under PIA-035:
   (a) trailers are read with `git log --format='%(trailers:key=<K>,valueonly=true)'`, git's own parser,
   not `grep '^<K>:'`;
   (b) the log is scoped to the run's own commits (`<base>..rk-run/$FIX_ITEM`) and the branch is asserted
   **ahead** of the base, so a stale ref proves nothing;
   (c) the values are asserted to **parse** against design §5.4's grammar, not merely to appear;
   (d) the worktree guard compares **path sets, not counts**, reading BEFORE any prune;
   (e) `cargo` exit codes are captured directly (`out=$(…); rc=$?`), never through a pipe.

**6.** No document, record, commit message or amendment this stage produces states or implies that the
review gate is guarded against false approval. `0101` clause 5 stands; S-SUBTLE-CB is UNRUN.

## Consequences

Both axes get their first producer, the ARMED lint
`conformance_every_trailer_key_has_a_producer` is discharged with a behavioural assertion behind it rather
than by a substring, and `Exit::FindingsBlock` gains the producer its own declaration has been promising
since 7b-ii. `s7i`'s first blocker — *"zero `dispatch_completed` events because no run completes"* — is
cleared for the fixture corpus and **not** for the shipped catalog, which is stated rather than implied.

The cost is that the shipped pack's review stage remains unreached, so Phase-1 criterion (1) is not closed
by this stage. The six blockers are now enumerated with the files and line numbers that produce them,
which is the difference between a blocked criterion and an unexplained one.

**What this forbids**

- Any claim that the shipped pack completes a committing walk, or that `0 of 102` has moved.
- Any claim that the review gate is guarded against false approval (`0101` clause 5).
- Asserting either axis with `grep '^Story:'` / `grep '^Slice-Verdict:'` over reachable history.
- Treating the pre-existing `rk-run/rk-fix-doctor-test-count` ref as evidence of a run.
- Giving the fixture pack's review activity a `check` or `gate` step, which would route it through a
  refusal this record explicitly stepped around rather than through the wiring under test.
- Authoring `.redkiln/schemas/`, populating `.redkiln/processes/`, or mapping `toolchain:verdict_record`
  under this stage's scope, or closing step-output resolution here.
- Inventing an exit row for `verify::exit::stage_exit`'s `ExitRefused`, which has none.

## Evidence and confidence

**The stage census is a count off disk**, parsing `stage = "…"` from all 102 `.redkiln/catalog/*.toml`;
it sums to 102 over 11 stages with `triage` at 0. `grep -n '^\[stage\.' .redkiln/packs/bug-standard.toml`
returns nine headers, beginning at `[stage.analysis]` — there is no `[stage.triage]`.

**Each blocker was reproduced at its own line, not inferred from a symptom.**
`ls -d .redkiln/schemas` → *No such file or directory*; `ls -A .redkiln/processes | wc -l` → **0**;
`MAPPED_VERBS` is the four-element array quoted above; both shipped items' `stage =` values are `triage`
and `specification`; and ADR-0142's own recorded measurement names the 94 `check` exprs it does not
repair. The 61 figure is the count of distinct first-`provides` kinds across the catalog.

**The checkpoint's false green was executed, not predicted.** With `FIX_ITEM` set, `git log
"rk-run/$FIX_ITEM"` returns rc=0; `grep -c '^Story:'` returns **0**; `grep -n '^Slice-Verdict:'` returns
one hit, and `git log --format='%h %(trailers:key=Slice-Verdict,valueonly=true)'` returns no value for any
commit — so the line is prose and the two readings disagree. Across all refs the census is **0** real
trailers for all three keys and **1** prose line.

**Confidence, and what is NOT asserted.** A fixture pack proves the wiring, not the catalog: this record
does **not** assert that any shipped review activity would reach a verdict, and it does not assert that
the `agent` + `emit` shape is how a real review activity should be authored — it is the shape that
isolates the call site from four absences this stage does not own. It asserts nothing about whether the
review gate's verdict is *correct*, only that a verdict reaches a trailer and an exit row. And the
`triage`-is-empty finding is a statement about the shipped catalog at `fc45eeb`; an activity added at
`triage` later would put schema resolution back in the advance path, which is why the assertion is a
`stage_transition` and a trailer rather than *"the run does nothing"*.
