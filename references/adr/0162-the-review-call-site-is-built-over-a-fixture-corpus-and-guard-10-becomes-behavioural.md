---
id: "0162"
title: "**The `review`/`verify` call site is built, over a fixture corpus, and `Exit::FindingsBlock` gets the producer its own declaration has promised since 7b-ii.** `0156` decision clause 3's sentence *\"No fixture pack is authored\"* is **superseded in part**: that sentence was written against `0160`'s three SLICE-IDENTITY absences, and `0160` clause 5 says in terms that *\"§7h.3's call site is separable … a review call site could exist without sealing a slice\"* — so the withdrawal reached further than its own reason, and `0156`'s `## Consequences`, its decision drivers and two of its forbids bullets all still assume the fixture pack (E-0156-1). Four things are decided that nobody had written down: the finding set is read **from the journal**, off `activity_committed.result`, which is `Typed.value` **verbatim** (AC-009) — so **no kernel seam and no new port method are owed**; the re-executor is **`Toolchain`-backed over AC-288's four cells** and refuses `kind = mutate` by name, because no port method applies a patch and Checkpoint 7a counts **exactly six** methods on that port; the review pass is selected by an activity's declared `provides`, **never by a stage-name literal**, because `resolved.stage` is compared to no literal anywhere in `rk-cli/src` and this record does not start; and Checkpoint 7h guard (10) becomes **behavioural** — a binary run exiting 4 — because as authored it is a `grep` over `crates/rk-cli/src/` that **a doc comment would satisfy**"
status: accepted
date: 2026-09-23
legacy_id: none
source: references/adr/0156 (decision clause 3's *"No fixture pack is authored"*; `## Consequences`'s *"Both axes get their first producer … `Exit::FindingsBlock` gains the producer"*; the `Giving the fixture pack's review activity a `check` or `gate` step` forbids bullet; the decision drivers *"a call site can be exercised without 61 schema files"* and *"A fixture pack is the tree's own established instrument"*); references/adr/ERRATA.md (E-0156-1); references/adr/0160 (clause 5, the separability sentence; clause 4, the no-unreachable-producer bar); references/adr/0101 (clause 5); crates/rk-cli/src/exit.rs (`FindingsBlock = 4` and its *"its producer is the kernel's, built at 7b-ii"*); crates/rk-kernel/src/runtime/activity.rs (`activity_committed_event`'s `"result": typed.value` and its *"the dispatch's `Typed.value` **verbatim** (AC-009)"*); crates/rk-types/src/artifact/findings.rs (`Findings::ingest` — *"The one door a review payload comes through"*); crates/rk-kernel/src/review/confirm.rs (`confirm_with`; `Confirmed::block_set`); crates/rk-kernel/src/review/policy.rs (`classify`; `Block`'s private members; `resolve_review_tier`); crates/rk-kernel/src/review/reexec.rs (the `ReExecutor` trait; `four_cell`; `mutation_outcome`); crates/rk-ports/src/toolchain.rs (six methods; *"Checkpoint 7a counts exactly six on this port"*); crates/rk-adapter-toolchain-rust/src/lib.rs (`fn test` — *"RT-22: `Ok(count)` only on a green suite"*); crates/rk-cli/src/verbs/run.rs (`resume_and_read`'s tail, the `expiry_pass` append-and-re-read idiom, `recreate_run_worktree`); crates/rk-cli/tests/fixtures/pack-min/ (no `[stage.review]` block; two catalog activities, neither a review); runbook/phase-1/s7h-lane-completes.md (§7h.3; §7h.4 guard (10))
supersedes: ["0156"]
superseded_by: []
related: ["0074", "0096", "0101", "0124", "0147", "0156", "0160"]
evidence: []
consumers: []
---

## Context and problem

Checkpoint 7h runs whole and exits `rc=1` at its **last** guard, and only there. Guard (10) is:

```bash
prod=$(grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ --include=*.rs | grep -v 'exit.rs' | grep -c .)
[ "$prod" -ge 1 ] || { echo "OWED (ADR-0156): Exit::FindingsBlock still has no producer…"; exit 1; }
```

Measured at `f25440e` on 2026-09-23: that grep returns **0**, and `Exit::FindingsBlock = 4` occurs
only in `crates/rk-cli/src/exit.rs` — its declaration, its `name()` arm and `Exit::ALL`. The row's
own doc comment has promised a producer since 7b-ii: *"The review gate's block verdict surfaced by
`run`. The value exists so the row is distinct; its producer is the kernel's, built at 7b-ii."*

`s7h` §7h.3 characterises the remaining work as wiring — *"`rk_kernel::review` (10 files) and
`rk_kernel::verify` (6 files) are built, tested, and have no caller on the run path … They are not
written here; they are **wired**."* That is true of the verdict machine and **not** true of four
things the wiring needs, none of which any record names. Measured at `f25440e`:

- `grep -rn "rk_kernel::\(review\|verify\)" crates/*/src/` returns **zero** across the workspace.
- `impl ReExecutor` occurs at exactly **three** sites, all under `tests/`; **none in any `src/`**.
- `resolved.stage` is compared to **no string literal** anywhere in `crates/rk-cli/src/`.
- `crates/rk-cli/tests/fixtures/pack-min/` declares `review = "mandatory"` under `[policy] stages`
  but carries **no `[stage.review]` block** and exactly two catalog activities,
  `emit-report.toml` and `second-activity.toml`, neither of them a review.

And the corpus question is blocked by a record that disagrees with itself. `0156` decision clause 3
says *"No fixture pack is authored"*; its `## Consequences` says *"Both axes get their first
producer … and `Exit::FindingsBlock` gains the producer its own declaration has been promising since
7b-ii"*; and one of its forbids bullets binds *"the fixture pack's review activity"* — a thing
clause 3 says does not exist. E-0156-1 records that contradiction. Clause 3 cannot be read as
binding a later stage until it is settled, and it is settled here rather than worked around.

## Decision drivers

- **`0160` clause 5 already separated the two tasks**: *"§7h.3's call site is separable and is also
  not delivered … a review call site could exist without sealing a slice, and sealing a slice needs
  an identity a call site would not supply."* Clause 3's withdrawal was reasoned from `0160`'s
  slice-identity absences, and those absences do not reach `Exit::FindingsBlock`.
- **`0160` clause 4 sets the bar this record must clear**: no producer *"whose row no run path
  appends"*, and no discharge by *"an occurrence that no run reaches"*. A ten-line scan of
  `recorded.events` for `EventKind::ReviewBlock` would turn guard (10) green and clear no bar.
- **Guard (10) as authored cannot tell the two apart.** Its grep matches any line of any
  `crates/rk-cli/src/**.rs` other than `exit.rs` — a doc comment naming the variant satisfies it.
  This programme has paid for that shape twice: both `s7g` source lints went green while
  `execute_step` had no caller.
- **The tier-1 layer is the standing warning.** `0158` measured three declared tables with zero
  writers. A producer that exists in source and never runs is the same defect one layer up.
- **`Block`'s members are private and it has no public constructor** (AC-294), so a caller cannot
  synthesise a blocking verdict; it can only read one back out of a `Confirmed`. The type system
  already makes an honest call site the only possible one.
- **Checkpoint 7a counts exactly six methods on the `Toolchain` port.** A seventh — an
  `apply_patch` — would re-open it for a fixture, which is the cost `s7d` refused for `Pack`.
- **`0101` clause 5 binds hardest at the stage that wires the gate**, and S-SUBTLE-CB is UNRUN.

## Considered options

### Option — Scan the journal for a `review_block` row

Ten lines in `resume_and_read`: scan `recorded.events` for `EventKind::ReviewBlock` and return
`Exit::FindingsBlock` when the scan is non-empty. Turns guard (10) green today. **Rejected**: no run
path appends that row, because `review::confirm` has no caller, so the arm is unreachable code and
the guard's grep is satisfied by its own declaration. This is exactly `0160` clause 4's prohibition
and the `s7g` source-lint shape. It would also make Checkpoint 7h report a producer the tree does
not have, which is the failure `0158` names one layer down.

### Option — Reach the shipped review stage first

Author `.redkiln/schemas/` (61 distinct `provides` kinds catalog-wide), populate
`.redkiln/processes/`, map `toolchain:verdict_record`, move an item to `stage = "review"` and close
`${…}` step-output resolution, then wire the call site against shipped content. **Rejected**, and
not on preference: `0156`'s forbids bullet bars *"Authoring `.redkiln/schemas/`, populating
`.redkiln/processes/`, or mapping `toolchain:verdict_record` under this stage's scope, or closing
step-output resolution here"*, and `0156` rejected the same shape as its own option 1 for absorbing
another stage's OWED work. The route stays recorded and unowned.

### Option — A fixture corpus, with the call site built for real (chosen)

Author a review activity in `pack-min` of shape `agent` + `emit`, ingest its finding set from the
journal, re-execute through the `Toolchain` port, call `review::confirm::confirm_with`, append its
rows, and return `Exit::FindingsBlock` when `Confirmed::block_set()` is non-empty. This is the route
`0156`'s own decision drivers describe — *"a call site can be exercised without 61 schema files"*,
*"A fixture pack is the tree's own established instrument"* — and that its `## Consequences` and two
forbids bullets already assume. What it does **not** prove is stated in `## Consequences` below.

## Decision

**1. `0156` decision clause 3's sentence *"No fixture pack is authored"* is superseded in part.** A
fixture pack review activity **is** authored, here, for the `Exit::FindingsBlock` call site alone.
Every other sentence of `0156` stands, including the whole of clause 2, the six blockers of clause
4, and clause 6. `0156` keeps `status: accepted`; only its `superseded_by` moves.

**2. The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that.** Checkpoint 7h
guards (6) and (7) require the ARMED lint to stay ARMED and to name `Slice-Verdict` as the **sole**
missing producer; a slice seal would turn both RED. `0160` stands whole: `<slice>` is undeclared,
no `slice_sealed` producer is written, and the `#[ignore]` stays.

**3. The finding set is read from the JOURNAL, and no kernel seam is owed.** The review activity's
`agent` step commits its structured output, and `activity_committed`'s payload carries
`"result": typed.value` — *"the dispatch's `Typed.value` **verbatim** (AC-009)"* — beside
`"schema"`. The call site reads `recorded.events`, which is a public field of `RecordedRun`, selects
the `ActivityCommitted` row by its `schema`, and passes `result` to `Findings::ingest`, *"the one
door a review payload comes through"*. **No new port method, no widened kernel return type, and no
change to `dispatch_through`'s `Result<Vec<Event>, ReplayError>`.** AC-282's *"no `structured_output`
is a `StepError::Schema`, never an empty set"* is already enforced upstream, on the dispatch path
that the `schema-no-structured-output` exit fixture pins at code 11; this record does not re-do it
and does not read prose (AC-318).

**4. The re-executor is `Toolchain`-backed, over AC-288's four cells, and refuses `kind = mutate` by
name.** For `ReproKind::Run` the selection is the tokens of `Repro.command` after a `--` separator;
the candidate arm runs in the run-owned worktree and the reference arm at the merge base, in an
**ephemeral worktree the pass creates and REMOVES** (decision 10), and
`toolchain.test(selection)` is the boolean, because its contract is *"`Ok(count)` only on a green
suite"* (RT-22) — so `is_err()` **is** "fails". The outcome is `reexec::four_cell(candidate_fails,
reference_fails)` and nothing else. **A repro whose command carries no `--` selection, and every
`ReproKind::Mutate` repro, yields `None`** — which `confirm_with` reads as
`Unverifiable::CannotRun` and which AC-287 forbids from blocking. The mutate arm is refused **by
name, in the record**, because applying a patch would need a seventh method on a port Checkpoint 7a
counts at six, and an empty selection is refused because `ac332_empty_selection_is_never_green`.

**5. The review pass is selected by an activity's declared `provides`, never by a stage name.**
`resolved.stage` is compared to no string literal anywhere in `crates/rk-cli/src/`, and this record
does not introduce the first one: a stage-name match would bind the wiring to one pack's spelling of
one stage. The pass runs for an activation whose activity declares the review verdict kind.

**6. `Confirmed.events` are appended through the store, and the journal is re-read**, by the
`expiry_pass` idiom already in `resume_and_read` — *"a pass that wrote nowhere would be invisible to
it, and reading it back is what makes the append observable rather than assumed"*. An append failure
is `Exit::ConfigError` with the row named, never a silent drop. Without this, `review_block` never
reaches the journal that §7d.4 names as row 4's identity, and `RK-D-SINK-COUNT` and
`RK-D-REVIEW-REEXEC` would have nothing to read.

**7. Guard (10) becomes BEHAVIOURAL, and the amendment is recorded here.** As authored it is a grep
over `crates/rk-cli/src/` that a doc comment satisfies. It is re-stated as a **binary run over the
fixture that exits 4**, with the exit code captured as `out=$(…); rc=$?` and never through a pipe.
The source grep is kept **beneath** it as a floor, scoped to code lines, so a deleted call site is
named at its own line rather than only by a changed exit. A checkpoint edited after it has run is
provisional; Checkpoint 7h has run, and this amendment is why its next run is the one that counts.

**8. The lane loop gains its first automated test.** `grep -rln "verbs::run::run(" crates/*/tests`
returns nothing today, and `0161` records that gap as what let two of its three defects through. The
call site added here sits on that same path and is not delivered without one.

**9. `0101` clause 5 stands.** S-SUBTLE-CB is UNRUN. Nothing in this record, the code it authorises,
its tests, its fixtures or its commit messages states or implies anything about the gate's
calibration; the sweep's scope is `crates/` and its pass condition is silence.

**10. The reference worktree is EPHEMERAL, and this is a constraint the checkpoint imposes rather
than a tidiness preference.** Checkpoint 7h guard (9) compares the worktree **path set** before and
after a `git worktree prune` and fails on any difference — the path-set form `0156` clause 5(d)
required precisely because *"counts cannot tell a leaked registration from a replaced one"*. A
review pass that left its reference registration behind would turn guard (9) RED, trading one
failing guard for another and leaving the checkpoint no better off. So the pass **removes its own
reference worktree on every path, including every refusal path**, and the run's net worktree path
set is unchanged. This is deliberately **unlike** the run-owned worktree, which
`recreate_run_worktree` recreates and never removes: that one is the run's output and is read after
the run; the reference tree is scratch, is read only inside the pass, and nothing outside it may
depend on its existence. A removal that fails is reported and does not fail the run, because a
leaked scratch tree is an operator problem and not a verdict.

## Consequences

`Exit::FindingsBlock` gains a producer a real `redkiln run` reaches, and Checkpoint 7h's last guard
can be satisfied by behaviour rather than by a substring. The `review` and `verify` modules — 16
files, built and tested since 7b-ii with zero `src/` callers — acquire their first one. The lane
loop acquires its first automated test, which is a debt `0161` named rather than a bonus.

The cost is that the call site is exercised over a **fixture corpus**, and the shipped pack's review
stage remains unreachable for the blockers `0156` clause 4 enumerates. This record asserts nothing
about the shipped catalog: `0 of 102` shipped activities completing a committing walk stands
unchanged, Phase-1 criterion (1) is not closed, and a fixture proves the **wiring**, not the
catalog. It also asserts nothing about whether a verdict is *correct* — only that a block verdict
reaches an exit row and a journal row.

**What this forbids**

- Forbids discharging guard (10) with an `Exit::FindingsBlock` occurrence no run path reaches — a
  doc comment, a `const`, a match arm on an unreachable branch, or a scan for a row nothing appends
  (`0160` clause 4's shape, one layer up).
- Forbids any claim that the shipped pack's review stage is reachable, that a shipped review
  activity would reach a verdict, or that `0 of 102` has moved.
- Forbids any claim about the review gate's calibration, in any document, record, commit message,
  fixture or test name (`0101` clause 5; S-SUBTLE-CB is UNRUN).
- Forbids writing a `slice_sealed` producer, declaring a `<slice>`, or removing
  `conformance_every_trailer_key_has_a_producer`'s `#[ignore]` under this record's scope.
- Forbids a seventh method on the `Toolchain` port, and forbids applying a mutation patch from
  `rk-cli`; `ReproKind::Mutate` is refused by name until a record adds the capability.
- Forbids blocking on a finding whose re-execution is absent or `Unverifiable` (AC-287), and forbids
  treating an empty selection as a green suite.
- Forbids matching `resolved.stage` against a string literal to find the review stage.
- Forbids leaving the reference worktree registered after the pass returns, on any path including
  every refusal path, and forbids anything outside the pass reading it (decision 10; guard (9)).
- Forbids authoring `.redkiln/schemas/`, populating `.redkiln/processes/`, mapping
  `toolchain:verdict_record`, or closing `${…}` step-output resolution under this record's scope —
  `0156`'s bullet, carried forward unchanged.
- Forbids giving the fixture review activity a `check` or `gate` step — `0156`'s bullet, which this
  record makes binding rather than vacuous by authoring the pack it presupposes.
- Forbids inventing an exit row for `verify::exit::stage_exit`'s `ExitRefused`, which has none.

## Evidence and confidence

**Every absence above was reproduced at its own line on 2026-09-23 at `f25440e`, tree clean**, not
inferred from a symptom. `grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ --include=*.rs | grep -v
'exit.rs' | grep -c .` → **0**. `grep -rn "rk_kernel::\(review\|verify\)" crates/*/src/` → **0**.
`grep -rn "impl.*ReExecutor for" crates/*/src/ --include=*.rs | grep -c .` → **0**, against three
sites under `tests/`. `ls crates/rk-cli/tests/fixtures/pack-min/catalog/` → `emit-report.toml`,
`second-activity.toml`. The baseline the work starts from is measured, not carried: `cargo build
--workspace` rc=0; suites `rk-cli` **110**, `rk-kernel` **238** (5 ignored), `rk-adapter-claude`
**102**, `redkiln-migrate` **25**, each read from cargo's own exit code with a non-zero passed count
asserted; `cargo fmt --all --check` rc=0; `cargo clippy --workspace --all-targets` **0 errors**, 154
warnings; Checkpoint 7h rc=1 **at guard (10) and nowhere else**.

**The contradiction this record resolves is textual and checkable**, not a reading: `0156`'s
`## Decision` clause 3 and its `## Consequences` are quoted against each other in E-0156-1, and
`0156`'s own **title** already carries the amendment — *"clauses 2 and 3 prescribed a fixture pack
and were amended"* — which is why `## Consequences` is the half that was not carried forward.

**What is NOT asserted.** That the chosen selection rule (`--`-separated tokens of
`Repro.command`) is the right general rule — it is the narrowest rule that makes the four-cell
computable from what a `Repro` carries, and a repro it cannot serve is refused rather than guessed.
That the reference arm at the merge base is the right baseline for every lane — it is right for the
fix lane this fixture exercises, and a story lane may need another, which is a residual and is named
here rather than discovered later. That `kind = mutate` is unimportant — it is the half of AC-289
this record does not deliver, and `mutation_outcome` keeps its tests and its caller-less state until
a port capability exists to feed it.
