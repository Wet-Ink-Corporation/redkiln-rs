---
id: "0196"
title: "**The review pass and the slice entry check key on the review verdict kind, through one predicate, and the seal reads only a review activation's `Findings` row** (`s8g`'s Findings-key record; D21-95 items 1 and 2; D21-105 items 3 and 4; D21-119, TAKEN PROVISIONALLY): `review_pass::is_review_activity` (`provides` first is `ReviewVerdict`, by exact element equality) is the one definition both readers call; `review_pass::findings_in` takes the last committed `Findings`-schema row whose `activity_id_at_version` resolves, in the catalog the invocation loaded, to a review activity at the stage the seal is written under, and refuses an unresolvable id by name; `findings_stage_reached` keys on the same predicate, so the entry check is live on the shipped packs. Supersedes `0186` in part (its `Findings`-first key sentences and its inert-seal gap statement); `0162` decisions 3 and 5 stand whole"
status: accepted
date: 2026-09-29
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-95 (item 1 — *"At entry it walks the stage order from the current stage through `Pack::successor`, skipping forbidden stages, and refuses by name, through the one validator `seal_act` also calls, when any stage reached has a catalog activity for that stage whose `provides` first is `Findings`. No stage-name literal; ADR-0162 stands whole."*; item 2 — *"the shipped review activity and `review_pass`'s schema key must agree (either `code-review` provides `Findings`, or the pass keys on what the shipped review provides), decided by a record there, so a shipped lane reaches the seal"*), D21-105 (items 3 and 4), D21-119 (the ruling, TAKEN PROVISIONALLY); runbook/phase-1/s8g-verify-and-restored-demands.md §8g.1 (the S8E-D2194-ENTRY receiver note and its dated D21-119 notes); references/adr/0162 (decisions 3 and 5), 0186 (decision 2 and its stated gap)
supersedes: ["0186"]
superseded_by: []
related: ["0072", "0101", "0156", "0159", "0160", "0162", "0186"]
evidence: []
consumers: []
---

## Context and problem

**ADR-0186 keys "a review stage" on `Findings`, and nothing shipped provides it.** ADR-0186's decision 2 (as amended by
S8E-D2194-ENTRY) refuses an undeclared slice at the lane's entry *"when a stage it will reach has a catalog activity
whose `provides` first is `Findings`"*, and states the key as *"The key is the one `review_pass` selects its committed
activation by (`review_pass::FINDINGS_SCHEMA`)"*. It then states a gap: *"So on the shipped packs **both the entry check
and the seal are inert** until `s8g` lands the shipped `Findings` producer, or keys `review_pass` on what the shipped
review provides, by a record there"*.

**Measured at `s8g` (package S8G-P1, code read at `178868f`; re-measured at this record's build over the tree at
`dbee9d6`):**
- **0** of the **102** shipped activities under `.redkiln/catalog/` name `Findings` in `provides` (none first, none
  anywhere; `.redkiln/catalog-extended/` adds none).
- The five review-stage verdict producers (`adversarial-review`, `checklist-review`, `code-review`,
  `formal-inspection`, `spike-decision-review`) provide `ReviewVerdict` first. `formal-inspection` provides
  `["ReviewVerdict", "InspectionRecord"]` and the other four provide `["ReviewVerdict"]`.
- Both lanes select `adversarial-review`. `code-review`'s `max_rigor` keeps it out of a mandatory review stage.
- **49** shipped agent steps emit `outputs = "value:Findings"`. **9** of them sit at the review stage and **40** sit
  elsewhere (`fix-spec`/`adversary`, `root-cause`/`adversary`, `equivalence-boundary`/`adversary`, …). Of the review
  stage's nine, six belong to review activities (`formal-inspection` has two, `reader` and `second-reader`). Three do
  not: `security-review` (`SecurityReviewVerdict`), `design-review` (`DesignVerdict`) and
  `production-readiness-review`/`adversary` (`PRRReport`).

**So the gap statement is half false.** The entry check (`findings_stage_reached`) was inert. The seal was **not**
inert: `review_pass::findings_in` took the last committed row whose step schema is `Findings`, from any activation. On a
shipped lane, a non-review adversary row could trigger the review pass and the `Slice-Verdict:` seal, and a later one
could displace the real review verdict. ADR-0162 decision 5 already says *"The pass runs for an activation whose
activity declares the review verdict kind"*, but `findings_in` did not enforce it.

## Decision drivers

- ADR-0162 decision 5: the pass is selected by an activity's declared `provides`, never by a stage name, and runs for
  an activation whose activity declares the review verdict kind.
- D21-95 item 2's second route: *"the pass keys on what the shipped review provides"*.
- The stage note's *"they are one constant and must not diverge"*: the step schema (`Findings`) and the provided kind
  (`ReviewVerdict`) are measured to differ, so what is shared is one predicate, not one string.
- Fail closed on both halves, and move the fewest record words. No catalog `@version`, CHANGELOG, `artifacts.toml`,
  lattice or Checkpoint 8b pin moves.

## Considered options

### Option — key both readers on the review verdict kind through one predicate, scoping the seal (taken)

This is D21-95 item 2's second route (panel scores 8.5 / 8 / 8.5). It is code-only, it needs no journal change (the
committed row already carries `activity_id_at_version`), and it makes ADR-0162 decision 5 true of the code.

### Option — `code-review` (or `adversarial-review`) provides `Findings` (D21-95's first route)

Rejected. `code-review` is kept out of the mandatory review stage by `max_rigor`, so the literal route never reaches
the seal. Retargeting it to `adversarial-review` means ADR-0072 version bumps, CHANGELOG rows, a lattice re-run and
pin moves. It would also still mis-seal on a non-review adversary row unless this record's scoping were taken as well.

### Option — one constant: `FINDINGS_SCHEMA` becomes `ReviewVerdict`

Rejected: no committed step row carries the schema `ReviewVerdict`, so every seal would become permanently inert. That
is a vacuous green.

### Option — defer to a later stage

Rejected: it leaves a live mis-seal and wrong-stage hazard on shipped fix lanes through `s8h`–`s8j`.

## Decision

1. **One predicate.** `rk_cli::review_pass::REVIEW_VERDICT_KIND = "ReviewVerdict"` sits beside the unchanged
   `FINDINGS_SCHEMA = "Findings"`. `review_pass::is_review_activity(a)` holds when `a.provides.first()` equals that kind
   by exact element equality: never a substring, `contains`, `starts_with` or `ends_with`. The shipped
   `security-review`'s `SecurityReviewVerdict` is the near miss. Both readers below call this one function.

2. **The seal side.** A candidate row is an `activity_committed` row whose `schema` is `Findings` and whose
   `activity_id_at_version` resolves, in the catalog this invocation loaded, to an activity that is a review activity
   **and** whose `stage` equals the stage the seal is written under (D21-93: the seal names its stage). The catalog is
   the same `catalog_sources` composition `base_inputs` hands the kernel, and it is loaded only when a
   `Findings`-schema row exists. The last candidate is the verdict row (the re-dispatch rule), so `formal-inspection`'s
   verdict is its `second-reader` row. A `Findings` row read before the candidate whose id resolves to nothing, or
   which carries no id, is **refused by name** (`PassSkipped::Unresolved`, exit 3). It is never skipped, because a skip
   would read as "no review". A run with no candidate behaves as a run with no `Findings` row did. The seal's verdict
   mapping (`review_pass::seal_verdict`) is unchanged.

3. **The entry side.** `findings_stage_reached` finds, at each reached stage, an activity with `a.stage == stage &&
   review_pass::is_review_activity(a)`. The walk, the `forbidden` skip, the bound, `tier_one::validate_slice` and the
   refusal's exit are unchanged, and no stage-name literal is added. On the shipped packs the check is now **live**:
   a fix-lane or story-lane item whose walk reaches the review stage is refused at entry unless it declares a `slice`.

4. **Two commits, in this order.** The first commit is the seal scoping, which only narrows and fails closed. It
   carries an interim entry key, `provides` names `Findings` anywhere, which keeps the entry exactly as live as before
   on every catalog while the fixture review activity declares `ReviewVerdict` first and `Findings` second. The second
   commit makes the entry live on this record's predicate. Each commit reverts alone.

5. **ADR-0186 is superseded in part, and only in these sentences:** *"whose `provides` first is `Findings`"* (decision
   2 as amended, its title's parenthetical, and the matching Consequences and forbids bullets that say *"provides
   `Findings` first"*), which now read *"that is a review activity (`review_pass::is_review_activity`)"*; *"The key is
   the one `review_pass` selects its committed activation by (`review_pass::FINDINGS_SCHEMA`)"*, which now reads *"the
   key is the one predicate `review_pass` selects its committed activation by (`review_pass::is_review_activity`)"*;
   and the gap statement *"So on the shipped packs **both the entry check and the seal are inert** until `s8g` …"*,
   which is withdrawn because its seal half was false. ADR-0186 keeps `status: accepted` and gains this record's id in
   `superseded_by`. Everything else in it stands: the validator, the two sitings, the mint's backstop and the stage
   scoping of `terminal()`.

6. **ADR-0162 is not moved.** Decisions 3 (*"selects the `ActivityCommitted` row by its `schema`"*) and 5 stand whole.
   Decision 3's schema selection is now narrowed to review activations, and decision 5 (*"The pass runs for an
   activation whose activity declares the review verdict kind"*) is now enforced. D21-95 item 1's *"provides first is
   `Findings`"* is read, by D21-119, as *"provides first is the review verdict kind"*.

## Consequences

**Positive**
- A shipped non-review adversary can no longer seal a slice or displace a review verdict, and a verdict is sealed only
  under the stage its review activation belongs to.
- The entry check reaches the shipped review stage, so an undeclared slice is refused before any dispatch.
- One definition of "review activity", read by both sitings.

**Negative**
- Every shipped lane that reaches review now needs a declared `slice`. This repository's two self-hosting items
  (`.redkiln/items/rk-fix-doctor-test-count.toml` and `rk-story-codex-adapter.toml`) declare none, so a real `run` of
  either is refused at entry until one is declared. That is an operator moment before `s8j`'s live runs.
- The entry check fires when **any** activity at a reached stage is a review activity, even one the lane would not
  select. It fails closed and matches D21-95 item 1's *"has a catalog activity for that stage"*. A selection-aware
  entry is a later record's choice (D21-119's residual).
- A journal whose latest `Findings` row names a catalog version no longer shipped (an `@version` bump between two runs
  of one item) is refused by name at the review pass until the item is re-run from a stage before the review. This is
  the fail-closed side of *"never skipped"*.
- `pack-min`'s fixture review activity declares `["ReviewVerdict", "Findings"]`.

**What this forbids**
- Selecting the review pass's verdict row by step schema alone, or from an activation that is not a review activity at
  the sealing stage.
- A second definition of "review activity" beside `review_pass::is_review_activity`, or either reader not calling it.
- Matching the review verdict kind by substring, `contains`, `starts_with` or `ends_with`.
- Skipping a `Findings` row whose `activity_id_at_version` does not resolve.
- Deciding "will reach a review stage" by a stage name (ADR-0162 decision 5, kept).
- Moving `FINDINGS_SCHEMA` to `ReviewVerdict`.

## Evidence and confidence

**Measured** (2026-09-29, `s8g` build, package R-D2119-8g, over `dbee9d6` plus this package's uncommitted work; every
cargo run went through the no-dispatch helper). Eight tests were red first, then green:
- `review_pass::adversarial_review_seals_approved_at_its_own_stage` (b1) and
  `review_pass::adversarial_review_seals_changes_requested_at_its_own_stage` (b2) were red at the stage clause
  (*"left: Ok(Findings { items: [] }) right: Err(NoFindingsRow)"*, read at `release`).
- `review_pass::a_fix_spec_adversary_findings_row_does_not_seal` (c) was red (*"left: Ok(Findings { … AC-601 … })"*).
- `review_pass::a_later_non_review_findings_row_does_not_displace_the_review_verdict` (d) was red (*"left: [\"AC-999\"]
  right: [\"AC-601\"]"*).
- `review_pass::security_review_and_design_review_do_not_seal` (e) was red.
- `review_pass::an_unresolvable_activity_id_at_version_is_refused_by_name` (f) was red (*"right: Err(Unresolved {
  activity_id_at_version: Some(\"adversarial-review@999\") })"*).
- `review_pass::a_shipped_lane_reaching_review_without_a_slice_is_refused_at_entry` (a) was red (*"the shipped fix
  lane reaches no review stage at entry: the check is inert on the shipped packs"*).
- `review_pass::formal_inspections_second_reader_row_is_the_verdict` (g) was green on arrival by construction; its red
  is mutant M6.

Mutants M1 (the schema-only match), M2 (the entry reverted to `Findings` first), M3 and M3b (the predicate dropped from
the seal reader, then from the entry reader), M4 and M4b (`ends_with`, `contains`), M5 (the unresolvable id skipped),
M6 (the first row taken) and M7 (the stage clause dropped) were each killed, in a private worktree. `rk-cli` read
297/0 after the first commit. The whole workspace read 1321 passed / 0 failed / 9 ignored over 44 result lines after
the second. **Not measured, and named:** no real `redkiln run` of a shipped item has reached the review stage; the
seal on a shipped lane is read through the shipped catalog and the seal's own mapping, not through a sealed commit.
**Confidence:** high on the selection rule, medium on the entry's operator cost (the residual above). This record is
`proposed` and flips only on its own dated ledger line (D21-105 item 1).
