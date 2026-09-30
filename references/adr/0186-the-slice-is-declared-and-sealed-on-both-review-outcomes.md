---
id: "0186"
title: "**The slice is declared on the item record, and `Slice-Verdict:` is sealed on both review outcomes** (`s8e`'s record; D21-41 e, first half; D21-93; D21-94; D21-95): `ItemRecord` gains an optional `slice`; ~~the kernel's `tier_one::seal_act` refuses at the mint, by name,~~ *(amended 2026-09-28, `s8e` build, packages S8E-D2194-ENTRY and R1FIX-8e; D21-94, D21-95)* the lane refuses at its ENTRY, before any dispatch and by name, through the kernel's `tier_one::validate_slice`, when a stage it will reach has a catalog activity whose `provides` first is `Findings`, and `tier_one::seal_act` keeps the same refusal at the mint as the backstop, for a `<slice>` that is undeclared, the item id, the project, a stage of the pack's `stage_order`, or carries whitespace (the run id cannot be declared); the review pass seals `approved` when nothing blocks and `changes-requested` before `Exit::FindingsBlock` returns, both through the kernel's tier-1 boundary; ADR-0160 decision 6's *\"at the review stage's boundary\"* is read as *\"at the review stage's exit outcome\"*; the seal's row names the stage it was made in and `terminal()` reads `Sealed` only while the fold is in that stage (D21-93); the ARMED trailer lint is un-ignored and re-pointed, and Checkpoints 7h (6)/(7), 8a (7) and 7f's literal are amended in the same commit. Supersedes nothing"
status: accepted
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (row e — *"Slice-Verdict: a declared `slice` key on the item record, refused by name when absent, never equal to the item id, one per fix-lane item, and sealed on both review outcomes including the `FindingsBlock` path."* *(dated 2026-09-28, s8e build, package R2FIX-8e: the ledger's exact row e text; this line quoted a paraphrase that no file carries)*) and D21-93 (*"The fold records the stage a seal was made in, and `terminal()` returns `Sealed` only until a later `stage_transition`"*), D21-94 (*"The refusal moves to the lane's entry, before any dispatch"*) and D21-95 (item 1, the entry check read from declared `provides`; item 2, the stated gap); runbook/phase-1/s8e-tier-one-and-the-lane.md (the records table's fourth row, §8e.0 item 8, §8e.8, §8e.12, §8e.13); references/adr/0159, 0160, 0161, 0162; crates/rk-cli/src/verbs/run.rs, crates/rk-kernel/src/runtime/tier_one.rs, crates/rk-kernel/src/workflow/fsm.rs, crates/rk-git/src/trailer/grammar.rs and crates/rk-kernel/tests/it/conformance.rs, as built 2026-09-28 on `d80609b`
supersedes: []
superseded_by: ["0196"]
related: ["0101", "0109", "0159", "0160", "0161", "0162", "0181", "0183"]
evidence: []
consumers: []
---

## Context and problem

**`Slice-Verdict:` has no producer, and ADR-0160 named the route rather than taking it.** Its decision 6: *"declare
the slice on the item record (or decide that a fix lane has exactly one, and say so), then a `slice_sealed` producer at
the review stage's boundary, then the trailer through the same `commit_with_trailers` path `Story:` now uses."* Its
first forbids bullet: *"Emitting a `Slice-Verdict:` trailer whose `<slice>` is the item id, the project, the stage, the
run id, or any value not declared as a slice."* D21-41 e took the route: the slice is declared on the item record, one
per fix-lane item, refused by name when it equals the item id, and sealed on both review outcomes.

Three facts of the tree shape how.

- **The rejecting verdict crosses no boundary.** ADR-0161 decision 2: *"The lane loop returns a non-`Exit::Success`
  status immediately, **before** reading `advanced`."* A `changes-requested` review ends the lane with exit 4, so a seal
  placed at a stage boundary could only ever record `approved`.
- **`terminal()` put a seal first, and nothing cleared it.** S8E-P0 measured it in the test binary (§8e.0 item 8): the
  journal `review -> slice_sealed{approved} -> release -> gate_opened` folded to `awaiting_gate = Some(..)` and
  `terminal = sealed`. `run.json`'s `terminal_state` is folded over the whole prior journal, so every run after review
  would have recorded `sealed`, a run halted at a release gate included. The operator ruled on it (D21-93).
- **Every tier-1 act now goes through one kernel function** (ADR-0181 decision 7; `s8e` §8e.3), and the trailer
  renderer is `rk-git`'s round-trip-checked `ParsedTrailer::to_trailer`. So `TrailerKey::` no longer appears on any code
  line of `rk-cli/src/` or `rk-kernel/src/`, and the ARMED lint's old spelling would fail at its own anchor with all
  three axes produced.

## Decision drivers

- ADR-0160's first bullet, **in full**: five values a `<slice>` may never be.
- ADR-0159 decision 2's siting: *"The refusal is at the mint, not at startup: a run that mints no trailer is not affected
  by the key's absence, and failing such a run would refuse work that has nothing to do with this."* *(dated
  2026-09-28, `s8e` build, package R3FIX-8e: quoted through to the sentence's end; it ended at ~~"absence."~~, a full
  stop where ADR-0159 has a comma, with no ellipsis.)*
- AC-162 and ADR-0181: the seal is a tier-1 act, git first, rows anchored to its commit.
- ADR-0162 decision 2 (*"The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that"*), scoped to that
  record's work; D21-47's time-bound rule: *"A time-bound guard in an earlier checkpoint is amended by the stage that
  moves its premise, in the same commit."*
- ADR-0109 decision 3: resume reads all three axes; the seal must stay readable as history.
- ADR-0101 clause 5: S-SUBTLE-CB is UNRUN.

## Considered options

### Option — seal at the review stage's exit outcome, refuse at the mint ~~(taken)~~ (the seal's siting chosen; the refusal's siting moved to entry, the next option but one)

`review_pass`, which ADR-0162 wired without a stage literal, is the one place both verdicts exist: `blocked.is_empty()`
and `Exit::FindingsBlock`. The seal is minted there, through the kernel's boundary, and every refusal is taken before any
git call. Chosen: it seals both verdicts, which D21-41 e requires, and moves no sentence of ADR-0160 (decision 3 below).

*(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's records minors)* **Only the seal's siting of this option stands as taken.** Its refusal siting is superseded in part
by D21-94: the refusal is taken at the lane's entry (the option *"refuse an undeclared slice at entry"*, the one
heading that reads taken) and the mint keeps it only as the backstop. The heading's *(taken)* is struck so the
two headings cannot both read taken.

### Option — seal at the review stage's boundary (the advance)

ADR-0160 decision 6's literal words. Rejected: a `changes-requested` review takes no advance (ADR-0161 decision 2), so
only `approved` could ever be sealed, and F12's 20.3% of slices passing through a rejected state — the reason the axis
exists — would be invisible.

### Option — refuse an undeclared slice at entry, before the lane runs (taken, D21-94; the mint kept as the backstop)

It would stop a fix lane without a slice before it dispatches anything. ~~Rejected for now: ADR-0159 decision 2 sites
the `<project>` refusal at the mint for the stated reason that a run minting no trailer must not be refused, and the
same holds here — a lane that never reaches its review mints no seal. The cost is stated in decision 2.~~

*(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; `spikes/DECISIONS-2026-09-21.md` D21-94 and D21-95.)*
**Taken, beside the mint's refusal, which stays as the backstop.** D21-94: *"**The refusal moves to the lane's entry,
before any dispatch**, for a lane that will reach a review stage (the fix lane always does): an undeclared slice there
is a certain failure, and it must not be paid for. The mint keeps its refusal as the backstop."* Three readings:
- **A divergence from ADR-0159 decision 2's siting, for this key only.** ADR-0159 decision 2 (*"The refusal is at the
  mint, not at startup: a run that mints no trailer is not affected by the key's absence"*) is cited, not superseded:
  its reason is kept by scoping the entry refusal to a lane that WILL mint a seal, so a lane that reaches no sealing
  stage is still not refused for a key it never needs. The `<project>` key stays at the mint unchanged: an absent
  `[project] name` is ADR-0159's `NoProject`, refused by `seal_act` and never at entry.
- **"Will reach a review stage" is read from declared `provides`, never from a stage name** (D21-95 item 1: *"At entry
  it walks the stage order from the current stage through `Pack::successor`, skipping forbidden stages, and refuses by
  name, through the one validator `seal_act` also calls, when any stage reached has a catalog activity for that stage
  whose `provides` first is `Findings`. No stage-name literal; ADR-0162 stands whole."*). The key is the one
  `review_pass` selects its committed activation by (`review_pass::FINDINGS_SCHEMA`), so ADR-0162's forbids bullet
  (*"matching `resolved.stage` against a string literal to find the review stage"*) is honoured. D21-94's parenthetical
  *"(the fix lane always does)"* is read, per D21-95, as the operator's expectation and not as the test.
- **One validator, two sitings.** The refusals of decision 2 are one kernel function, `tier_one::validate_slice`, which
  `seal_act` and the entry check both call, so the entry and the mint cannot disagree.

### Option — derive the slice from the item id when none is declared

Rejected by ADR-0160's first bullet in terms, and by D21-41 e. ~~Mutant S8E-M8 builds exactly this and
`slice_verdict::an_absent_slice_is_refused_by_name` is red on it.~~

*(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's records minors)* Since the entry refusal, the binary tests meet the entry check first, so the default is two mutants
by siting (`s8e` §8e.12, measured): **S8E-M8a**, the default in `seal_act` alone, leaves
`slice_verdict::an_absent_slice_is_refused_by_name` GREEN and is red on
`slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop`; **S8E-M8b**, the default in the shared
`validate_slice`, is red on both, because the entry then names the item-id rule.

## Decision

**1. The slice is declared.** `ItemRecord` gains an optional `slice` (`#[serde(default)]`), optional in the type so the
refusal names the item, as `stage` is. `Resolved` carries it verbatim. **Cardinality:** a fix-lane item declares
exactly one slice (D21-41 e: *"one per fix-lane item"*); the story item `s8j` mints declares one too, and this record
says the same of it.

**2. The refusals are at the mint and by name.** `rk_kernel::runtime::tier_one::seal_act(declared, item, project,
stage_order, stage, verdict)` returns `TierOneError::SliceRefused { item, slice, reason }` for each value ADR-0160
names, and every refusal is taken before any git call:
- an absent `slice`, or an empty one (*"any value not declared as a slice"*) — `SliceRefusal::Absent`;
- a `slice` equal to the item id (D21-41 e) — `ItemId`;
- a `slice` equal to `[project] name` — `Project` (an absent project is ADR-0159's `NoProject`);
- a `slice` equal to any stage of the pack's `stage_order` — `Stage(name)`;
- a `slice` carrying whitespace (the value is *"exactly two whitespace-separated tokens"*) — `Whitespace`.

**The run id cannot be declared.** It is `format!("{}-{}", stage, &digest[..16])`, computed from inputs the item
record cannot know and carrying a digest, so a declared slice never equals it; no refusal is written for a value that
cannot occur, and this sentence is the record of why. ~~**The cost of the mint siting, stated:** a fix lane with no
slice walks to its review stage, dispatches its review, and only then refuses (exit 3, nothing committed). ADR-0159's
siting applies unchanged: a lane that never reaches its review is not refused for a key it never needs.~~

*(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* **The refusal is taken at the lane's
ENTRY, before any dispatch, and the mint keeps it as the backstop.** The refusals above are one function,
`rk_kernel::runtime::tier_one::validate_slice(declared, item, project, stage_order)`, and `seal_act` calls it first.
`rk-cli`'s `run`, after the closed-item read and before the store, the run tree, any commit or any harness, walks the
stage order from the current stage through `Pack::successor` (a `forbidden` current stage is skipped too; the walk is
bounded by the order's length), and when a stage reached has a catalog activity for that stage whose `provides` first
is `Findings`, it calls the same validator and refuses by name: exit 3, AC-172's one `reconcile_started` row, nothing
committed, zero dispatches. A lane that reaches no such stage is not refused at entry (ADR-0159's reason, kept). An
absent `[project] name` is not an entry refusal: `validate_slice` skips only the project comparison when the project is
absent, and `seal_act` then refuses `NoProject` at the mint as before. One consequence of the shared order, stated: a
`slice` equal to a stage, with no `[project] name`, is now refused as `Stage(..)` rather than `NoProject`; both are
refusals by name, before any git call. If the pack or the catalog does not load, the entry check decides nothing and
the path that loads them refuses by name; the mint is the backstop there too.

**The stated gap (D21-95 item 2).** Measured by a `tomllib` census over `.redkiln/catalog/*.toml`: **0** shipped
activities declare `provides` first `Findings`; the shipped `code-review` provides `["ReviewVerdict"]`. So on the
shipped packs **both the entry check and the seal are inert** until `s8g` lands the shipped `Findings` producer, or
keys `review_pass` on what the shipped review provides, by a record there; `s8e` closes on its fixtures, and a dated
receiver note in `s8g`'s file owes it.

**3. The seal sits on the review stage's exit outcome.** `review_pass`'s outcome is the producer: `blocked.is_empty()`
seals `approved`, and the blocking path seals `changes-requested` **before** it returns `Exit::FindingsBlock`. Both go
through the kernel's tier-1 boundary (ADR-0181 decision 7): the `Slice-Verdict: <project>/<slice> <verdict>` commit
first, then `t1_slice_seal` and the anchored `slice_sealed` row. A refused seal is exit 3, named. **Reading:** ADR-0160
decision 6's *"at the review stage's boundary"* is read as *"at the review stage's exit outcome"*. It moves no sentence:
decision 6 names a ROUTE (*"The route out is named"*) and its forbids bullets name no site, and the boundary could not
carry the rejecting verdict ADR-0160's own evidence (F12's 20.3%) exists for. If the adversarial pass finds that it
does move a sentence, this record takes it as a supersession in part of decision 6 alone.

**4. The seal is scoped to the stage it was made in (D21-93).** The `slice_sealed` payload is `{ project, slice, stage,
verdict }`, `stage` the review stage whose outcome is sealed. The fold records it (`RunState::sealed_stage`), and
`terminal()` returns `Sealed` only while the fold's stage is that stage; after a later `stage_transition` it falls
through to gate, dispatch or complete. `sealed()` keeps the verdict as history, so resume still reads all three axes
(ADR-0109 decision 3, cited, not superseded). `sealed_stage` is not serialised, so no canonical byte moves; no committed
golden log carries `slice_sealed` (S8E-P0). **Stated residual:** a row re-derived from a `Slice-Verdict:` trailer — by
the heal or the rebuild — carries no `stage`, because the trailer holds none, and the fold scopes it to the stage it
folds in. An `approved` seal is committed after the kernel's advance out of review (the review pass runs after
`resume`), so a HEALED or REBUILT approved seal of a lane that went on reads `Sealed` in the next stage until that
stage's own transition. The live row, which carries `stage`, does not.

**5. The unbound seam seals nothing.** `run_with` — the in-process machinery seam, which binds no tier-1 boundary
(ADR-0185's stated residual) — runs the review pass and mints no seal, as it mints no `closed` mark. The verb `run`
always binds one.

**6. The ARMED lint is discharged, re-pointed and narrowed, in the commit whose real fixture run reaches the seal.**
ADR-0162's precedent that a real `redkiln run` over a fixture corpus reaches a producer: `slice_verdict::
approved_is_sealed_on_the_advance_path` (a new `slice/approved/` corpus, exit 0) and `slice_verdict::
changes_requested_is_sealed_on_the_findings_block_path` (`exit/findings-block/`, exit 4) run the binary and read the mark
off `rk-run/<item>` with git's own parser. The `#[ignore]` line is deleted (it is source, not a record, so no erratum is
owed). **The lint is re-pointed**: a producer is a call of the boundary's act constructor for its axis — `Mark::Stage(` or
`close_act(` for `Story`, `Act::gate(` or `Act::gate_opened(` for `Gate`, `seal_act(` for `Slice-Verdict` — named on a
code line outside `runtime/tier_one.rs`, which DEFINES the constructors and whose definitions are the lint's anchor.
**Its producer scope drops `crates/rk-cli/src/doctor/`**, the READER direction, on the same reason the scope never held
`rk-git/src/trailer/`. Mutant S8E-M45 deletes the seal while `doctor/rebuild.rs` names `TrailerKey::SliceVerdict` (and,
a second variant, `seal_act(`), and the lint stays red on both.

**7. ADR-0162 decision 2, re-stated rather than superseded.** It was scoped to that record's work — *"`0160` stands
whole: `<slice>` is undeclared, no `slice_sealed` producer is written, and the `#[ignore]` stays"* — and it is true of
the tree `0162` recorded. This record's work is the discharge, and it re-states the three checkpoint guards that pinned
the armed state, each amended by dated line **in this commit**:
- **Checkpoint 7h guard (6)**: `conformance::conformance_every_trailer_key_has_a_producer --exact`, its `... ok` line and
  `1 passed; 0 failed; 0 ignored`;
- **Checkpoint 7h guard (7)**: the `ARMED: Slice-Verdict has 0 producers` string absent from `conformance.rs`, grep's `rc`
  exactly 1 (and the block's Slice-Verdict bullet dated ✅; guard (10a)'s FAIL message loses its two executing backticks);
- **Checkpoint 8a guard (7)**: the lint's `... ok` line, `1 passed; 0 failed; 0 ignored`, the ARMED string absent.

Checkpoint 7f's literal moves too: all three lints `... ok`, `3 passed; 0 failed; 0 ignored`. 7h, 8a and 7f are
`re-run pending` until Checkpoint 8e re-runs each whole.

**8. 🛑 The S-SUBTLE-CB non-claim.** A seal records a verdict. It says nothing about the verdict's calibration: no
document claims the review gate is guarded against false approval (ADR-0101 clause 5; S-SUBTLE-CB is UNRUN).

**9. Supersessions, and what is cited without superseding.** **Nothing is superseded.** ADR-0160's first bullet is
honoured in full (decision 2); ADR-0160 decision 6 is read (decision 3), with the fallback stated; ADR-0162 decision 2
is re-stated (decision 7); ADR-0161 decisions 1–2 and ADR-0159 decision 2 are cited; ADR-0109 decision 3 is honoured
(decision 4).

## Consequences

**Positive**
- The second trailer axis has a producer a real run reaches, on both verdicts, git-first.
- The run record's terminal state stops reading `sealed` for a lane that went on past review.
- The ARMED lint runs on every commit, and it counts call sites of the boundary, not a spelling a reader can supply.

**Negative**
- ~~A fix lane without a slice fails late, at its review (decision 2).~~ *(dated 2026-09-28, `s8e` build, package
  S8E-D2194-ENTRY; D21-94.)* A lane that will reach a `Findings` stage without a slice fails at entry, before any
  dispatch; one whose `Findings` producer is not declared first in `provides` still fails only at the mint.
- *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-95 item 2.)* On the shipped packs the entry check and
  the seal are both inert: 0 shipped activities provide `Findings` first, so no shipped lane reaches either until `s8g`.
- A healed or rebuilt approved seal is scoped less exactly than a live one (decision 4's residual).
- Three earlier checkpoints are provisional until Checkpoint 8e re-runs them.

**What this forbids**
- A `Slice-Verdict:` whose `<slice>` is undeclared, the item id, the project, a stage of the pack's `stage_order`, or a
  value carrying whitespace; a default slice of any kind.
- A seal committed outside the kernel's tier-1 boundary, or a `slice_sealed` row written before its commit.
- Sealing only one of the two review outcomes.
- *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* Dispatching anything on a lane that
  will reach a stage whose catalog activity provides `Findings` first while its slice is undeclared; a second slice
  validator beside `tier_one::validate_slice`; deciding "will reach a review stage" by a stage name; and refusing at
  entry a lane that reaches no such stage.
- `terminal()` reading `Sealed` for a fold that has left the stage the seal was made in.
- Counting a construction under `crates/rk-cli/src/doctor/` or inside `runtime/tier_one.rs` as a producer.
- Claiming the review gate is guarded against false approval (ADR-0101 clause 5).

## Evidence and confidence

**Measured** (2026-09-28, `s8e` build, package S8E-P7, on `d80609b` plus the stage's uncommitted work): the four
`slice_verdict::` tests red first (no seal; exit 0 where 3 was required), then green; the lint red first at its anchor
(no `seal_act`) and then on `["Slice-Verdict"]` alone, then green; the D21-93 fold test red first (`Sealed { verdict:
"approved" }` at a release gate), then green; mutants S8E-M7, M8, M45 (two variants) and M50 killed for their stated
reasons. **Not measured, and named:** the shipped tree's review stage stays unreachable (ADR-0156), so no shipped item
has been sealed. This record is `proposed` and flips only on the operator's dated ledger line.

*(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* **Measured**: `lane_e2e::
an_undeclared_slice_is_refused_at_entry_before_any_dispatch` red first (*"the lane dispatched 2 time(s) before
refusing … left: 2, right: 0"*, the mint's refusal after `implement` and `review`), then green: exit 3, zero dispatches,
zero harness builds, no run tree, no commit, no tier-1 row, and the binary's stderr names the undeclared slice and the
entry siting. Its control `lane_e2e::a_lane_that_reaches_no_findings_stage_is_not_refused_for_an_undeclared_slice`
(no `Findings` activity installed, no `slice`) dispatches once and exits 0. `slice_verdict::
the_mint_refuses_an_undeclared_slice_as_the_backstop` pins the mint and the validator to the same refusal for every
named value. Mutants **S8E-M53** (the entry check dropped), **S8E-M54** (the walk reads the current stage only) and
**S8E-M55** (the validator's call dropped from `seal_act`) are each killed for the stated reason;
`slice_verdict::an_absent_slice_is_refused_by_name` stays green, and green under S8E-M53, because the mint's backstop
refuses the same absence. **Not measured, and named:** no shipped lane reaches the check (D21-95's census, 0 of the
shipped activities).

*(dated 2026-09-29, `s8g` build, package R-D2119-8g; D21-119, TAKEN PROVISIONALLY under D21-105 items 3 and 4.)*
**Reading note: this record is superseded in part by ADR-0196 (`proposed`).** This note does not edit the
Decision above. Every sentence ADR-0196 moves still reads as written here until ADR-0196 flips on its own dated ledger
line. ADR-0196 moves three things:
- **The key.** *"whose `provides` first is `Findings`"* (decision 2 as amended, the title's parenthetical, and the
  Consequences and forbids bullets that say *"provides `Findings` first"*) and *"The key is the one `review_pass`
  selects its committed activation by (`review_pass::FINDINGS_SCHEMA`)"*. They are read as *a review activity*:
  `review_pass::is_review_activity`, `provides` first `ReviewVerdict` by exact element equality. The entry check and
  the seal both call that one predicate.
- **The gap statement.** *"So on the shipped packs **both the entry check and the seal are inert** until `s8g` …"* is
  withdrawn. It was measured false on its seal half: `review_pass::findings_in` took the last `Findings`-schema row
  from any activation, and 40 of the 49 shipped `value:Findings` steps sit outside the review stage.
- **What stays.** The validator, the two sitings, the mint's backstop, the seal on both outcomes and the stage scoping
  of `terminal()` all stand.
ADR-0162's decisions 3 and 5 stand whole, and decision 5 is now enforced.
