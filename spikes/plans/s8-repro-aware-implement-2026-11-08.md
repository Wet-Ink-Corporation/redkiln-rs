# The fix lane implements against the committed repro, agent selections are refused at the agent step as `Schema`, and a walk-ending failure reaches stderr and the journal

Branch `claude/project-thread-bzyg5t`, base `origin/build/phase-0-to-1` at `7f82d9b`. Plan written 2026-11-08 by the
architect. No Rust is written by this plan. Every `H-16` item is in **Decisions**, and none is taken here except the
operator's ruling (Ryan, card, 2026-11-08 22:34Z: *"Repro-aware tdd plus hardening"*). Next free ids: D21-177,
ADR-0205.

## Goal

When a bug item's repro was placed and reproduced (ADR-0204: the committed repro is the red proof), the implement
stage does not ask an agent to write red tests. It re-proves the committed repro red at the implement baseline, lets
the implementer change the code, and holds the change green on **two** selections: the repro's own command
(`artifact:ReproCase.command`, the placed command, through the one parser) and `affected`. Stories keep
`tdd-red-green-refactor`'s red phase. On the path the operator's run took, four things are hardened:

- The red agent is told what a red step is.
- A bad agent-produced test name is refused **at the agent step** as a retryable `Schema` error, never as a
  walk-ending `Unsupported` at the tool step.
- A step failure that ends the walk is written to stderr with its capability text, and appended to the journal.
- An all-empty red selection loops back instead of ending the walk.

The operator gets the restart procedure for `rk-fix-doctor-margin-2` → `-3`.

## What happened, and what the survey adds (measured on this tree)

The operator's run (item `rk-fix-doctor-margin-2`, frozen `a6d2f76`) went: analysis green (`minimal-repro@2` placed
and confirmed red, `root-cause@6` OK), then the hold, then `fix-spec@2` (4 ACs), then `tdd-red-green-refactor@3`.
`red` for_each:0 committed the **fix** to `crates/rk-cli/src/doctor/rules.rs` (d5de724) with `tests_added: []`.
for_each:1 returned `["doctor test"]`. `red-verify` refused it in `check_selection_token`
(`runtime/reference/selection.rs:137`), then `selection_refusal` (`runtime/step.rs:742`) raised `Unsupported`, then
`StepFailedAfterCommit`, and the run exited 19. The survey adds:

1. **The red agent was never told it was a red step. This is the root cause of the improvisation.** The claude
   adapter's user message is the JSON object of the step's inputs (`rk-adapter-claude/src/lib.rs:309`,
   `user_message`). For `red` those inputs are `{ac, spec, last, item}`. The implementer card's whole prompt is
   *"Make the change the step describes, and report it"* (`.redkiln/cards/implementer.json`). Nothing in either
   names the step, the activity, or the contract "write a failing test; do not change production code". Given an AC
   and a spec, the agent "made the change". The catalog grammar has no step-prompt field. It does admit a string
   literal as an input value (`catalog::Step::inputs`; `reference.rs` row "literals"). So a `task` literal is a
   catalog-only remedy that touches no card and no adapter.
2. **The precedent's verdict on tdd@3 was wrong.** Its table said *"no defect: the implementer writes the test"*. The
   implementer *can* write the test, but nothing tells it to, and the only deterministic net, `red-check`
   (`failed > 0 and passed == 0`), cannot see a for_each element that edits production and names no test while
   another element names a failing test.
3. **An all-empty `tests_added` ends the walk too.** `selection_plan`'s array arm refuses an empty list as
   `Unsupported` (`reference.rs:1601-1609`). This is the same exit 19 by another route, and it was one element away
   on the operator's run (elements 0 and 3 were empty).
4. **A step guard cannot read an artifact.** `guard_holds` (`step.rs:1120`) resolves only `${…}` references
   (`reference.rs` `parse_ref`: a step, a gate, `toolchain.`, `item`). `artifact:` is an input source, not a
   reference. The guard context binds the item's facets and `rigor` (ADR-0142, `replay.rs:1618`).
5. **`has_repro` is `null` on every item that does not declare it, so a `when` on it fails open.** `rk-cli` binds an
   absent facet as `with_facet_null` (`verbs/run.rs:2589`), and the story item declares none. A `when = "not
   has_repro"` on `red` is `Tri::Null` there, which **skips the step** (`guard_holds`: `False` and `Null` both skip).
   Every story would lose its red phase silently. Separately, `has_repro` is a human claim made at triage, not
   evidence that a repro was placed and failed.
6. **Selection already picks by facets within `implement-loop`.** Step 4 takes the lowest admissible `method_rank`
   for each provided kind, with `applicable_if` evaluated over the item's facets (`select/step4.rs` header).
   `bisect` already keys `applicable_if = "has_repro"`. For a bug, today's admissible implement producers are
   `tdd` (20) only: `test-after` (10) tags chore and docs, and `bdd-outside-in` (30) tags story.
7. **A `Schema` failure is already the retryable class at the agent step.** `run_activity_at`'s loop
   (`runtime/activity.rs:1023-1046`) turns a schema mismatch into `StepError::Schema`, writes its `step_failed` row,
   and re-dispatches up to `SCHEMA_REPROMPT_LIMIT = 2` (`workflow/retry.rs:151`). A check placed there before
   `commit_at` gets that path for free. **The re-prompt carries no validation error**: the loop re-sends the same
   `spec`. That is AC-031's *"carrying the validation error"* half, which the code marks unbuilt (`retry.rs:11`).
8. **The adapter-side alternative re-opens paid steps.** The claude adapter's `extract_digest`
   (`classify.rs:574-614`) has a `Spec` ingest check of exactly this kind. But `crates/rk-adapter-claude` is inside
   D21-174 item 3's D10 guard (`git diff --quiet 1810fdd… HEAD -- crates/rk-adapter-claude crates/rk-containment
   .redkiln/cards tools/live-run.sh`), and so is `.redkiln/cards`. A change in either re-opens §8j.6's paid steps.
   The design below touches neither.
9. **Observability.** `run`'s `StepFailedAfterCommit` arm (`verbs/run.rs:3137-3147`) prints a generic sentence and
   not `class`. The `NotCommitted` arm just above it does print the class. A non-agent step failure appends **no**
   journal row: `Walk::halt` (`step.rs:3056`) only folds the error into the report. Only a dispatch failure has a
   row, `error_row` (`activity.rs:1153`). Caution: a `step_refused` row is a **recorded halt** to the fold
   (`workflow/fsm.rs` table, `HaltReason::Refused`), while `step_failed` changes no state.
10. **`root-cause@6` and `confirmation-test@2` stay correct.** `minimal-repro@2`'s composing emit sets
    `ReproCase.command = ${place.outputs.command}`. `root-cause@6` confirms on it and composes `discriminating_test`
    from it. `confirmation-test@2`'s `before` step selects `artifact:ReproCase.command` at `artifact:Changeset.sha`,
    so verify re-runs the placed repro after the fix, and the placed test lands with the fix (ADR-0204 decision 5).
    Nothing here changes either. `affected` (`HEAD..rk-run/<item>`) also contains the placed test file, so the
    repro runs inside `affected` too.
11. **The fixture proofs passed because the fake did what the real card would not.** `lane_proof.rs:176-182`
    scripts `tdd@3` `red` as a model implementer: it writes `tests/red.rs` and names `RED_NAME`. The real agent,
    given no contract, wrote the fix and returned prose names. The rig keys a response on
    `(activity@version, step, visit)`, and the visit counts **committed** rows (`rig.rs:582-640`). So a re-prompt
    gets the same scripted answer, and today a fixture cannot model "bad first, good on retry".
12. **The AC ledger reads each criterion's `verification`** (`verify/ledger.rs`, `verify/ac_gate.rs`). If the
    implement loop writes no test that a criterion's verification names, the verify pass may open
    `verify.ac-discriminates` gates. Under tdd, red wrote one test per AC. A repro-driven loop must still ask the
    implementer for those tests (see `green`'s `task` below).
13. **The Spec did not cover the whole BugReport.** The BugReport for `-2` names three defect cases: `[margins]`
    absent, a window missing, a non-integer value. The repro (`s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error`)
    covers only the missing window, and the Spec's AC-1..4 cover only the absent key. With the repro as the red
    proof, a green run could land a fix for one case of three (Decision D13).

## Constraints and non-goals

- No new dependency, no new crate edge, no port trait change, no new `StepError` variant and no new `EventKind`
  (Decision D7 keeps that true).
- **Nothing under the D10 guard changes**: not `crates/rk-adapter-claude`, not `crates/rk-containment`, not
  `.redkiln/cards`, not `tools/live-run.sh`. D21-174's ruling that §8j.6's paid steps stand therefore stays
  available (Decision D12).
- The kernel stays language-free. "Is this path a test?" is a toolchain fact, and nothing here asks it (Decision D4).
- Non-goals:
  - A deterministic "red touched only tests" check (Decision D4 option ii, a tracked issue).
  - Changing `confirmation-test`, `minimal-repro` or `root-cause`.
  - Making `ImplReport.status` an enum.
  - `differential-debugging` and `instrumented-trace-analysis` (ADR-0204's owed issue): the feed check below covers
    their selection-feeding fields too, and their read-only-role defect is unchanged.

## Invariants (each is a test below)

1. A `kind = "bug"` item with `has_repro = true` selects `repro-green-refactor@1` for `Changeset` and `ImplReport`.
   An item with `has_repro = false`, and every story, selects `tdd-red-green-refactor@4`. `has_repro = null` on a
   bug opens the `unknown_facet:has_repro` abstention gate and never chooses silently.
2. `repro-green-refactor@1` dispatches no agent before `red-proof` has run `artifact:ReproCase.command` through
   `parse_selection`. If that run is not red (`failed == 0 or passed > 0`), the `repro.cannot-reproduce` gate opens
   before any agent runs.
3. `repro-green-refactor@1` emits only after `repro-verify` (`artifact:ReproCase.command`) and `green-verify`
   (`affected`) both report `passing` on the same pass, or after the `tdd.green-exhausted` gate was answered.
4. An agent step whose output feeds a `toolchain:test` selection in its own activity has that output checked before
   commit. A string feed must pass `parse_selection`. Each element of a list feed must pass `check_selection_token`.
   A failure is `StepError::Schema` with the field, the value and the refusal variant in `last_error`. It writes
   one `step_failed {class: "schema"}` row and is re-dispatched under AC-031's limit. Nothing is committed for the
   refused answer.
5. The check applies to fresh dispatch results only. A value served from a committed row, on re-entry or replay, is
   never re-checked (the `Spec` ingest check's rule, `classify.rs:597-600`).
6. A re-prompt after a `Schema` failure carries the refusal as the input `refused` (Decision D6). No catalog step may
   declare an input named `refused`.
7. `tdd@4`'s `red-verify` never receives an empty list. `red-named` (`names.len() > 0`) loops back to `red`, at most
   two times.
8. A walk that ends on a step failure writes the failing class, with its capability text, to stderr. It appends
   exactly one error row for a non-agent step's failure (Decision D7) and none extra for an agent dispatch's failure,
   which already has its row.

## Design

### 1. Repro-aware implement: a separate `implement-loop` member (Decision D1, option C)

A new activity, `.redkiln/catalog/repro-green-refactor.toml`, version 1:

```toml
[activity]
id           = "repro-green-refactor"
version      = 1   # added 2026-11-08 (ADR-0205, proposed; D21-177; .redkiln/CHANGELOG.md)
stage        = "implement"
provides     = ["Changeset", "ImplReport"]
requires     = ["Spec", "ReproCase"]
derived_from = ["Spec", "ReproCase"]
tags         = ["kind:bug", "change:text"]
applicable_if = "has_repro"          # Decision D2
method_rank  = 15   # implement loop family: test-after(10) < repro-green-refactor(15) < tdd(20) < bdd-outside-in(30)
family       = "implement-loop"
min_rigor    = "optional"
cost_class   = "L"
evidence     = ["GreenAt", "ToolReport", "HumanAnswer?"]
budget       = { attempts_total = 12, tokens = 800_000, wall_clock_secs = 10800 }
deviation_ok = []                     # no lower implement-loop member tags a bug
keywords     = ["repro-driven fix", "make it pass", "regression test first"]
rationale    = "Agans rule 9 and ADR-0204: the committed repro is the red proof; re-proved red at the implement baseline, the change is done when it and the affected suite are green."
```

Steps, in order:

1. `red-proof`: `tool`, `toolchain:test`, `inputs = { selection = "artifact:ReproCase.command" }`,
   `outputs = "value:TestReport"`, `timeout_secs = "${toolchain.test_timeout}"`, `retry = { max = 1 }`.
2. `red-holds`: `check`, `inputs = { report = "${red-proof.outputs}" }`,
   `expr = "report.failed > 0 and report.passed == 0"`, `on_false = "continue"`.
3. `not-red`: `gate`, `question = "repro.cannot-reproduce"`,
   `when = "${red-proof.outputs.failed} == 0 or ${red-proof.outputs.passed} > 0"`,
   `inputs = { report = "${red-proof.outputs}" }`. This is the `confirmation-test` and `root-cause@6` check-then-gate
   pattern, with an existing question that both packs already carry (Decision D3).
4. `green`: `agent`, role `implementer`,
   `inputs = { task = "<literal, below>", spec = "artifact:Spec", repro = "artifact:ReproCase", red = "${red-proof.outputs}", last = "${green-verify.outputs?}", repro_last = "${repro-verify.outputs?}", hint = "${green-gate.answer?}" }`,
   `outputs = "value:ImplReport"`, `timeout_secs = "${toolchain.implement_timeout}"`, `retry = { max = 2 }`,
   `repeat = { until = "${green-verify.outputs.status} == 'passing' and ${repro-verify.outputs.status} == 'passing'", max = 3 }`,
   `on_exhausted = "escalate"`, `escalate = { question = "tdd.green-exhausted" }`.
5. `repro-verify`: `tool`, `toolchain:test`, `selection = "artifact:ReproCase.command"`, the same bound as step 1.
6. `green-verify`: `tool`, `toolchain:test`, `selection = "affected"`.
7. `green-gate`: `gate`, `question = "tdd.green-exhausted"`,
   `when = "${green-verify.outputs.status} != 'passing' or ${repro-verify.outputs.status} != 'passing'"`,
   `inputs = { report = "${green-verify.outputs}", repro = "${repro-verify.outputs}", impl = "${green.outputs}" }`.
8. `refactor`, `refactor-verify` and `still-green`: as tdd's, with one addition. A `refactor-repro` tool step
   (`artifact:ReproCase.command`, `when = "rigor >= recommended"`) runs before `still-green`, whose expression becomes
   `report.status == 'passing' and repro.status == 'passing'`. `refactor` also gets a `task` literal.
9. `summary`: `toolchain:change_summary`, `range = "baseline..HEAD"`.
10. `emit`: `artifacts = { Changeset = "${summary.outputs}", ImplReport = "${green.outputs}" }`,
    `evidence = ["${red-proof}", "${not-red?}", "${repro-verify}", "${green-verify}", "${refactor-verify?}", "${refactor-repro?}", "${green-gate?}"]`.

`green`'s `task` literal (this is the agent's whole description of the step; the implementer must keep the text
under 600 characters):

> "Fix the defect so the committed repro (`repro.command`) passes. It fails now (`red`). Do not edit or delete the
> repro test. For each criterion in `spec`, add or extend a test named by its `verification`, and list every test
> you added in `tests_added` as a bare test path with no spaces (for example `doctor::margin_absent_window_is_an_error`)."

Why each choice:

- **Rank 15** puts the new activity between `test-after` and `tdd` without an equal-rank pair. `rank_lint`
  (`select/step4.rs:534`) enumerates the contexts and must stay green.
- **`ReproCase` is a mandatory `requires`.** Every bug that passed analysis has one: `minimal-repro@2` refuses at
  `place` without a pair (ADR-0204 decision 1).
- **Re-proving red costs one test run.** The first run of `affected` builds the workspace's test targets, and the
  measured cold `--list` was up to 99 s against `test_timeout = 360`. It makes the red proof local to the activity,
  and it covers the `proceed-without-repro` answer at minimal-repro's gate, where `ReproCase` is emitted although
  nothing failed.
- **`tdd` is untouched by the repro path**, so the story lane's (1b) behaviour is changed only by the hardening
  (section 3).

Why not the other two options (detail in D1):

- **A, a `when` on `has_repro` inside tdd@4.** It fails open on `null` (survey 5), and it needs `green`'s
  `${red.outputs}` → `${red.outputs?}`, emit evidence `${red-verify?}`, and an `artifact:ReproCase?` selection that
  is `null` on a story and would have to be guarded. That makes two loops in one activity, and a `null` guard skips
  silently.
- **B, an optional `artifact:ReproCase?` input read by a guard.** It needs the guard grammar to resolve `artifact:`
  inside `${…}`: an ADR-0173 grammar change, plus every guard-lint that reads it.

### 2. Hardening (b): the agent-step feed check (Decision D5, option i)

**Placement.** `rk-kernel::runtime::reference::selection` (the one parser's module, 153 lines; ARC-06 room), and the
re-prompt loop in `runtime/activity.rs`. Dependencies are unchanged.

```rust
// crates/rk-kernel/src/runtime/reference/selection.rs (add)

/// One output field of an agent step that a `toolchain:test` step of the same activity selects with
/// (`${<step>.outputs.<path>}` or `${<step>.outputs[*].<field>}`), derived from the catalog at walk start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionFeed {
    /// The path inside ONE dispatch's value (for `[*]`, the element's field).
    path: Vec<Seg>,
    /// The `toolchain:test` step that reads it, for the refusal's text.
    reader: String,
}

impl SelectionFeed {
    #[must_use] pub fn path(&self) -> &[Seg];
    #[must_use] pub fn reader(&self) -> &str;
}

/// Every feed of every agent step in `steps`, keyed by agent step id. Borrows the steps (OWN-01) and returns
/// owned values the walk stores for the activation (OWN-02). A plain `${s.outputs.p}` on a `for_each` agent step
/// reads the element list, not one dispatch, so it yields no per-dispatch feed (the tool step's own refusal stays
/// the net for it).
#[must_use]
pub fn selection_feeds(steps: &[Step]) -> BTreeMap<String, Vec<SelectionFeed>>;

/// Why a dispatched value fails a feed (refused at the agent step as `Schema`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedRefusal {
    /// The path is absent from the value, or is neither a string nor a list of strings.
    NotASelection { path: String, reader: String },
    /// A string feed that `parse_selection` refuses.
    Command { path: String, reader: String, value: String, why: SelectionRefusal },
    /// One element of a list feed that `check_selection_token` refuses.
    Token { path: String, reader: String, index: usize, why: SelectionRefusal },
}
impl fmt::Display for FeedRefusal { .. }   // starts with "the field `<path>` feeds `<reader>`'s selection and …"
impl std::error::Error for FeedRefusal {}

/// Check one dispatched value against its step's feeds. An empty list passes (an element may name no test;
/// `tdd@4`'s `red-named` judges the aggregate).
///
/// # Errors
/// The first [`FeedRefusal`], in feed order then element order.
pub fn check_feeds(value: &serde_json::Value, feeds: &[SelectionFeed]) -> Result<(), FeedRefusal>;
```

Re-exported at `rk_kernel::runtime::reference::{SelectionFeed, FeedRefusal, selection_feeds, check_feeds}`, beside
`parse_selection`. `Seg` is already public there.

**Threading.** `StepAt` (`activity.rs:98`) gains `pub feeds: &'a [SelectionFeed]`. The walk computes
`selection_feeds(steps)` once per activation (in `run_activity_steps`, stored on `WalkEnv`) and passes the agent
step's slice. Every other construction site passes `&[]`: the non-agent sites in `step.rs`, plus
`tests/it/step_landing.rs` and `tests/it/on_false_exec.rs` (a mechanical test edit, not an expectation change;
Decision D8).

**The check** sits in `run_activity_at`'s loop, inside the `typed.schema == spec.schema` arm and before `commit_at`.
The order of operations:

1. Run `check_feeds(typed.value(), at.feeds)`.
2. On `Err(why)`: increment `report.schema_failures` and set
   `StepError::Schema { attempts: report.schema_failures, last_error: why.to_string() }`.
3. Fall through to the existing error-row, `decide` and re-dispatch code, unchanged. AC-031's counter therefore
   counts it as one schema failure.

Served steps never reach this code: `already` returns early (`activity.rs:960`), which satisfies Invariant 5.
`BOUNDARY_ANCHORS` and `boundary_prose_extraction_rule` stay green, because the check reads a `Value` and parses no
text out of prose.

**The re-prompt carries the refusal (Decision D6, option A).** When `decide` returns `Reprompt` for a `Schema`
error, the next `dispatch_once_at` gets a spec built as `spec.clone()` plus `("refused", Value::String(last_error))`.
The clone is justified: a per-attempt owned spec, because the adapter receives `&ActivitySpec` and the base spec is
borrowed from the walk. A catalog lint (`rk-kernel` `tests/it/selection.rs`) asserts that no shipped step declares
an input named `refused`. The memo context is built from the walk's own spec (`StepContext::for_agent_step(&own)`),
not the re-prompt's, so memo keys do not move. The rig's `key_of` must filter `refused`, as it filters `item` and
`worktree` (`rig.rs:600`).

**Why the kernel, not the schema or the adapter** (D5):

- A `schemars` pattern on `ImplReport.tests_added` changes the exported schema the CLI is handed (the snapshot and a
  CLI re-measure, D21-167's territory). It also duplicates `check_selection_token` in a second spelling.
- An adapter ingest check sits inside the D10 guard.
- The kernel check is harness-independent, so the fakes exercise it. That is the honesty the fixture constraint
  asks for.
- It discharges ADR-0204's owed issue "D4, option c" for every selection-feeding field, driven by the catalog rather
  than a hand list.

### 3. Hardening (a) and (d) in `tdd-red-green-refactor@4` (Decision D4, options i + aggregate check)

- `red` gains `task` = *"Red step of test-driven development for ONE acceptance criterion (`ac`). Write a test that
  fails because the behaviour is missing. Do not change production code. List the test in `tests_added` as a bare
  test path with no spaces. If the criterion already holds, write no test and say so in `summary`."* `green` and
  `refactor` gain short `task` literals of their own.
- A new `check` step `red-named`, between `red` and `red-verify`: `inputs = { names = "${red.outputs[*].tests_added}" }`,
  `expr = "names.len() > 0"`, `on_false = { to = "red", max = 2 }`. It closes survey 3. The loop varies what it
  reads, so the d2 lint's rule holds.
- `red-verify`, `red-check`, green, refactor and emit are otherwise unchanged. Under section 2, a bad name is now
  refused at `red` itself, so `red-verify`'s token refusal becomes a belt.

What still is not enforced: an element that edits production while another names a failing test. `red-check`
catches a fix that makes the named tests pass. It does not catch a fix plus an unrelated failing test. The
deterministic check is option ii, filed as an issue (D4).

### 4. Hardening (c): the failure reaches stderr and the journal (Decision D7, option J1)

- **stderr.** `verbs/run.rs`'s `StepFailedAfterCommit` arm writes, through `writeln!(io::stderr().lock(), ..)` (H-06;
  replacing that arm's `eprintln!`), the existing sentence plus `redkiln: the step's class: {class:?}`. That is the
  `NotCommitted` arm's spelling, and `Debug` carries the `capability` text. If the write fails, the arm leaves
  through the exit code, as the `NotCommitted` arm does.
- **Journal.** In `run_activity_steps`, when the walk ends on `Halt::Failed(err)` or `Halt::Stop(err)` for a step
  whose kind is not `agent`, the walk appends `error_row(&err, None, now)` with two added payload members,
  `step_id` and `activity_id_at_version`. The fold reads only `class`, so the extra members move nothing. An agent
  step's failure already has its row from the dispatch loop, so none is added (Invariant 8). Under J1 the existing
  one mapping applies (`StepError::event()`, AC-360): `Unsupported` becomes `step_refused`, a recorded halt, and
  every other class becomes `step_failed`, which changes no state. `explain`'s `stopped:` line (ADR-0204 decision
  7) then names the refusal for a tool-step failure too.

### 5. Ownership and errors, summarised

| Item | Ownership | Error |
|---|---|---|
| `selection_feeds(&[Step])` | borrows; returns an owned map stored per activation | none (pure) |
| `check_feeds(&Value, &[SelectionFeed])` | borrows both | `FeedRefusal` (manual `Display` + `Error`, like `SelectionRefusal`), mapped to `StepError::Schema` at one site |
| `StepAt::feeds` | `&'a [SelectionFeed]`, borrowed from `WalkEnv` for the call | none |
| re-prompt spec | one owned clone per re-prompt (stated) | none |
| walk failure row | `error_row` by reference | the existing `RuntimeError` on append failure |

No new threads, tasks, locks or I/O. The check is O(feeds × elements) over a value already in memory.

## Test list (red first, in this order)

rk-kernel, `tests/it/test_selection.rs`:
- [ ] 1. `selection_feeds_of_tdd_red_is_the_tests_added_field`: the shipped `tdd@4` gives `{red: [path tests_added, reader red-verify]}` and nothing for `green` or `refactor`.
- [ ] 2. `selection_feeds_of_a_string_ref_is_its_path`: a crafted activity whose `confirm` reads `${attempt.outputs.command}` gives `{attempt: [command]}`.
- [ ] 3. `selection_feeds_skips_a_plain_ref_to_a_for_each_step`.
- [ ] 4. `check_feeds_refuses_a_spaced_token_naming_field_index_and_reader`: `{"tests_added":["ok","doctor test"]}` gives `Token { index: 1, why: BadToken }`.
- [ ] 5. `check_feeds_refuses_prose_naming_the_variant`, `.._a_missing_or_non_string_field`, and `check_feeds_passes_an_empty_list_and_plain_tokens`.
- [ ] 6. Property (proptest): for every string `s`, `check_feeds({"f":[s]})` is `Ok` iff `check_selection_token(s)` is `Ok`.

rk-kernel, `tests/it/` (activity loop; the scripted-harness suites):
- [ ] 7. `a_bad_feed_value_is_a_schema_failure_and_is_redispatched`: answers `[bad, good]`. One `step_failed {class:"schema"}` row, two dispatches, the good value committed.
- [ ] 8. `a_bad_feed_value_twice_refuses_as_schema_and_commits_nothing`: `Schema { attempts: 2 }`, no `activity_committed` for the step.
- [ ] 9. `the_reprompt_carries_the_refusal_as_refused` (D6): the second spec's inputs hold `refused` naming the token.
- [ ] 10. `a_committed_bad_value_is_served_not_rechecked`: re-entry over a journal whose committed `red` row holds `"doctor test"` serves it and dispatches nothing (Invariant 5).
- [ ] 11. `a_tool_step_failure_after_a_commit_appends_one_error_row`: a refused verb after an agent commit gives exactly one `step_refused` carrying `step_id`, `activity_id_at_version` and `capability`. An agent dispatch failure adds no second row.

rk-kernel, catalog and selection:
- [ ] 12. `selection::repro_green_refactor_is_chosen_for_a_bug_with_a_repro`: `has_repro = true` chooses `repro-green-refactor`, `false` chooses `tdd-red-green-refactor`, a story chooses `tdd`, and `null` gives an abstention (Invariant 1).
- [ ] 13. `selection::no_shipped_step_declares_the_reserved_input_refused`.
- [ ] 14. `on_false_exec::tdd_red_named_loops_back_on_an_all_empty_red`: two elements `[]`, then a named test, gives one loop-back and no `Unsupported`.
- [ ] 15. `lane_walk`: the `FIX` implement row becomes `("repro-green-refactor", Walks), ("change-summary", Walks)`. `LaneToolchain` answers `red-proof` red and the verifies green. `STORY` is unchanged at 13/0 (Decision D9).

rk-cli:
- [ ] 16. `run::a_walk_ending_refusal_names_its_capability_on_stderr`: a pack-min pack whose tool step names an unrouted verb after an agent commit. stderr holds `the step's class: Unsupported { capability: "` and the verb, and the store holds the `step_refused` row. Red before: the generic sentence only, and no row.
- [ ] 17. Rig: `Response::then_on_retry(Response)`, a per-key answer sequence indexed by the dispatch ordinal within a visit, so a re-prompt can answer differently (survey 11). Its own unit test is in `lane_proof/rig.rs`'s `#[cfg(test)]`.
- [ ] 18. **`lane_proof::fix_lane_with_an_improvising_implementer_lands_through_the_repro`, the reproduction of THIS failure.**
  - The script models the real card. `tdd-red-green-refactor@3` (also `@4`) `red` visit 0 writes `FIXED_LIB` into `src/lib.rs` and answers `tests_added = ["word count test"]`, `status = "implemented"`, as the operator's for_each:0 and :1 did. `repro-green-refactor@1` `green` writes `FIXED_LIB` and the AC's named test, `tests_added = [RED_NAME]`.
  - Red before the fix: the run exits 19 (`Unsupported`, `doctor`-shaped token), and `src/lib.rs` is committed by a `red` step.
  - Green after: the lane lands. Implement's `red-proof` report has `failed == 1, passed == 0`, and `repro-verify` and verify's `ConfirmationResult` are `passing`. No `tdd-red-green-refactor` key is visited. No `verify.ac-discriminates` gate opens (survey 12).
- [ ] 19. **`lane_proof::story_lane_red_with_a_bad_test_name_is_reprompted_as_schema`.**
  - `tdd@4` `red` visit 0 attempt 0 writes `STORY_RED_TEST` and answers `["story red test"]`. Attempt 1 answers `[STORY_RED_NAME]`.
  - Red before: exit 19.
  - Green after: the lane lands. There is one `step_failed {class:"schema"}` row for `red`, and the second dispatch's inputs carry `refused`.
- [ ] 20. The existing proofs move to the new keys: `fix_script` keys `repro-green-refactor@1` for implement, and `story_script` keys `tdd-red-green-refactor@4`. All 17 current `R_TESTS` stay green.

Mutants (`runbook/tools/s8j-mutants.py`, sha256-verified restores; Decision D9):
- `M-repro-aware-unselected`: `repro-green-refactor.toml` `method_rank = 15` → `25`. Must turn 18 red, because `tdd` is chosen and the improvising red is refused.
- `M-feed-check-off`: `check_feeds` returns `Ok(())` first. Must turn 19 red (exit 19 at `red-verify`) and 7 red.
- `M-step-class-silent`: the new stderr `writeln!` is deleted. Must turn 16 red.

## Records and runbook

- **ADR-0205** (proposed, Decision D10): "A bug with a placed repro implements against it; agent outputs that feed a
  test selection are checked at the agent step; a walk-ending failure reaches stderr and the journal." It records
  D1-D7. It answers ADR-0204's owed issue "D4, option c": the operator files that issue closed as done, or does not
  file it. It amends nothing in ADR-0204 or ADR-0173 (the parser is unchanged; it is called earlier).
- **D21-177**: Ryan's 22:34Z ruling, quoted; each Decision's outcome once taken; the (3) ruling (D11) and the D13
  repro-scope ruling.
- `.redkiln/CHANGELOG.md`: rows for `repro-green-refactor@1` (added) and `tdd-red-green-refactor@4` (the `task`
  literals and `red-named`), each with old and new text. Both packs gain
  `[activity.repro-green-refactor] timeout_secs = 10800`, as every activity has a row there.
- Dated amendments:
  - `runbook/phase-1/s8j-run-preparation.md` §8j.4 (the three new proofs and the mutants table); §8j.6 step 1b
    (`R_TESTS` 17 → 19 for proofs 18 and 19; test 16 is rk-cli `run::`, so 20 if it joins; `S8J_MUTANTS` gains
    `M-repro-aware-unselected` and `M-step-class-silent`, with `M-feed-check-off` beside them under `--kernel` if
    test 7 is its killer).
  - `runbook/phase-1/s8-exit-criterion.md` (the restart note below).
  - `runbook/PROGRESS.md` row 8.
  - The charters `crates/rk-kernel/AGENTS.md` (the feed check's home) and `crates/rk-cli/AGENTS.md` (the stderr
    line).
- Evidence lines are the operator's. The implementer never edits `s8j-evidence.txt`.
- Owed issue (D4 option ii), for the operator to file: "A deterministic check that `tdd`'s `red` changed only test
  paths (`toolchain:test_scope`, both adapters)."

## Restarting the item's run: `rk-fix-doctor-margin-2` → `-3` (D21-163's route; no new code)

`rk-run/rk-fix-doctor-margin-2` carries **an unreviewed fix commit, d5de724**, made by a `red` step. It is never
merged, cherry-picked or used as a reference for the lane's fix, because the `reserved-fix-item:` entry forbids a
fix by hand or outside the lane. It stays only as archived evidence. The steps follow
`runbook/phase-1/s8-exit-criterion.md`, "After a failed first real run of the fix item", as approved by D21-163 and
taken by D21-174 item 5. Each step is the operator's, with its own go-ahead.

1. **Stop and record.** Add a dated line to `PROGRESS.md` row 8 with:
   - the launch record's basename;
   - the failing line (`tdd-red-green-refactor@3` `red-verify`, rc 19, `Unsupported` from `check_selection_token`
     on `"doctor test"`);
   - `git rev-parse rk-run/rk-fix-doctor-margin-2`;
   - a note that d5de724 is a red-step fix and is not reviewed.

   The ref is not merged.
2. **Archive the ref.** Use the `b` suffix, because `s8-failed-2026-11-08/` already holds `-1`'s archive:
   - `git update-ref refs/archive/s8-failed-2026-11-08b/rk-fix-doctor-margin-2 rk-run/rk-fix-doctor-margin-2`
   - `tar -cf <archive dir>/worktree.tar -C .redkiln/worktrees rk-fix-doctor-margin-2`
   - `git worktree remove --force .redkiln/worktrees/rk-fix-doctor-margin-2`
   - `git branch -D rk-run/rk-fix-doctor-margin-2`
   - `git worktree prune --dry-run -v` must print nothing (Checkpoint 8j (12)).
3. **Archive the store, and keep it live.** Copy it with SQLite's backup API to
   `$(git rev-parse --git-common-dir)/redkiln/archive/2026-11-08b-rk-fix-doctor-margin-2/store.sqlite3`. Write a
   `counts.txt` of Checkpoint 8j (0)'s `rows` reading over both copies. The two readings must agree. The live store
   is not deleted, truncated or edited.
4. **Mint `rk-fix-doctor-margin-3`** (s8j §8j.5 steps 1-5):
   - `.redkiln/items/rk-fix-doctor-margin-3.toml`, a copy of `-2`'s with the dated header updated and
     `has_repro = true` kept (Invariant 1 needs it).
   - Its BugReport and repro pair, copied, **or the repro widened per Decision D13**.
   - The operator's ledger line `reserved-fix-item: rk-fix-doctor-margin-3 rk-cli:doctor::rules::margin`, in his
     voice.
   - One dated `#` line on `-2`'s item file naming the archive ref and directory.
5. **Re-freeze.** This plan's code changes `CODE_PATHS`, and so does the mint. In order:
   - CI green on the pushed head, giving a new `ci` line.
   - **§8j.6 step 1b as amended**: 19 or 20 proofs, the mutants. This gives `fixture-proofs <sha> pass <date>` and
     one `mutant <id> killed <date>` line each, for the old twelve plus the new ones.
   - `fix-item rk-fix-doctor-margin-3 <40-hex sha>`, appended last so `tail -1` reads it, and the `story-item` line,
     in an evidence commit that changes no `CODE_PATHS`.
   - **D10's guard** at the new sha:
     `git diff --quiet 1810fddc5c96a58bb6fc069cc7bcb0ff04003820 HEAD -- crates/rk-adapter-claude crates/rk-containment .redkiln/cards tools/live-run.sh`
     must exit 0, or §8j.7 applies to the letter (Decision D12).
6. **Re-check Checkpoint 8j (0), (1), (3), (4) and (12)** against `-3`, from the block as it stands, under
   `nodispatch`, from a plain terminal:
   - (0): the readings.
   - (1): build-or-fail.
   - (3): the new `fix-item` line and reservation, and the absence of `rk-run/rk-fix-doctor-margin-3` and of its
     worktree.
   - (4): the reserved defect is still red at HEAD in a throwaway tree. If D13 widens the repro, (4) proves the
     widened test red.
   - (12): nothing dispatched or written.

   Record the pass in row 8j by a dated line naming the archive ref.
7. **Preflight, then the paid launches.** Run `prespend()` (it reads `ci`, `fix-item`, `fixture-proofs` and
   `record-rt57`) and the launcher's own preflight. Then:
   - (1) launch of `-3` under `S8_GO=rk-fix-doctor-margin-3`, and answer the hold through `tools/live-run.sh answer`.
   - (3) kill and resume, per Decision D11.
   - Any further gate, then (1) launch again, then (1) check.
   - The story item's (1b) launch is separate and unchanged, except that it now runs `tdd@4`.

**What the operator should look for on `-3`'s run:**

- Implement selects `repro-green-refactor@1`, and `explain` shows `red-proof` red.
- `repro-verify` and `green-verify` are passing on one pass.
- The verify stage opens no `verify.ac-discriminates` gate.
- A refused agent answer shows as a `step_failed {class:"schema"}` row and a second dispatch, never as an exit.
- At the hold before specification, the operator cannot edit the Spec, because it is written after the hold. D13
  is therefore the place to settle the BugReport's other two cases.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| D1 | How implement becomes repro-aware | **A** `tdd@4` with `when` on the facet `has_repro` on red, red-verify and red-check: `null` (every story) skips red silently (survey 5); `green`/emit must go optional; a `null` `artifact:ReproCase?` selection must be guarded. **B** `tdd@4` reads `artifact:ReproCase?` and guards on it: guards cannot read artifacts (survey 4), so it needs an ADR-0173 grammar change. **C** a separate `implement-loop` member, `repro-green-refactor@1`, rank 15, `applicable_if = "has_repro"`, chosen by step 4 for bugs. `tdd` keeps its shape for stories | **C**. It departs from the ruling's literal "tdd@4" for the repro path; tdd still goes to @4 for the hardening | open | |
| D2 | `repro-green-refactor`'s admission | **A** `applicable_if = "has_repro"`: explicit, the same as `bisect`; `null` opens the abstention gate. **B** no `applicable_if`: every bug (all have a `ReproCase` after `minimal-repro@2`); a bug whose analysis was skipped by override then gates on the missing `ReproCase` | A | open | |
| D3 | Re-prove red at implement entry | **A** yes: `red-proof`, `red-holds`, and the `not-red` gate on `repro.cannot-reproduce` (one test run; covers `proceed-without-repro`). **B** trust analysis's red and go straight to `green` | A | open | |
| D4 | Red must not edit production (hardening a) | **i** a `task` literal telling the step its contract (catalog only; not enforced; fixes the root cause in survey 1). **ii** a deterministic check: a new `toolchain:test_scope` verb in **both** toolchain adapters plus a `check` over red's paths (a registry row, a `VerbOutput` and a Node rule; about 400 lines). **iii** a per-step landing allowlist in the kernel (catalog grammar, `rk-git` landing and an ADR-0188 amendment: heavy). Plus, in every case, `red-named` for the empty aggregate | **i + `red-named` now; ii as a filed issue; iii rejected** | open | |
| D5 | Where a bad agent name is refused (hardening b) | **i** a kernel check before commit, driven by the catalog's selection feeds, as `Schema` (re-prompted). **ii** a `schemars` pattern on `ImplReport.tests_added` (schema snapshot, CLI re-measure, a second spelling of the token rule). **iii** an adapter ingest check like `Spec`'s: inside the D10 guard, re-opens the paid steps | i | open | |
| D6 | The re-prompt carries the refusal | **A** every `Schema` re-prompt adds input `refused = last_error` (AC-031's letter; a reserved input name, linted). **B** only the kernel feed refusals. **C** a plain re-dispatch, as today: the same inputs can produce the same answer | A | open | |
| D7 | Journal a walk-ending step failure (hardening c) | **J1** `error_row` with `step_id` and `activity_id_at_version` for a non-agent step: `Unsupported` becomes `step_refused`, which halts the item (ADR-0095; the restart route mints a new item anyway), other classes become `step_failed`. **J2** a new tier-2 `EventKind` the fold ignores: closed-set change, its own record. **J3** stderr only | J1 | open | |
| D8 | Public API | `StepAt::feeds` (a new pub field: struct literals in two test files gain `feeds: &[]`); `rk_kernel::runtime::reference::{SelectionFeed, FeedRefusal, selection_feeds, check_feeds}` | approve (additive; one mechanical test edit) | open | |
| D9 | Test-expectation changes | `lane_walk` `FIX` implement row (tdd → repro-green-refactor); counts that move with a 103rd activity and two new check steps (`forward_ref_lint` activities 102 → 103 and its census, `quality_block::ac527` check steps 94 → 96, `review::ac303` confirm-step counts, `pack_validation` per-activity counts), re-pinned by dated lines as measured; `fix_script`/`story_script` re-keyed; `R_TESTS` 17 → 19 (or 20 with test 16); three mutants | approve all as measured; `M-repro-aware-unselected` and `M-step-class-silent` in `S8J_MUTANTS`, `M-feed-check-off` under `--kernel` beside it if its killer is a kernel test | open | |
| D10 | The record | **A** ADR-0205 (proposed). **B** amend ADR-0204 (proposed), whose subject is analysis | A | open | |
| D11 | s8 criterion (3) kill/resume after the restart | **A** re-run (3) kill and resume on `-3`. This plan changes `run_activity_at`'s dispatch-to-commit path (the feed check, the re-prompt spec) and the walk's failure append, which is the code AC-013's "in-flight dispatch redone, no pre-kill step re-ran" exercises, and (3) is bound to `$FIX_ITEM` (s8j A3). **B** (3) stands from `-2` (gate answer survived, `gate_resumed`, in-flight dispatch redone, nothing re-ran) by a dated ruling: no guard can hold, because `activity.rs` changes | **A** (one extra dispatch at most; the kernel arm `kill_and_resume_n_step` re-runs free) | open | |
| D12 | §8j.6's paid steps after this change | **A** §8j.7 to the letter. **B** D21-174 item 3's ruling stands, guarded by the same `git diff --quiet 1810fdd… HEAD -- crates/rk-adapter-claude crates/rk-containment .redkiln/cards tools/live-run.sh`, which this design keeps empty | B, re-measured at the new sha | open | |
| D13 | The BugReport's two uncovered cases (`[margins]` absent, non-integer) | **A** widen `-3`'s repro test to assert all three cases (still one `fn`, so `repro_place`'s `fn <test>(` rule holds); re-prove 8j (4) red. **B** narrow `-3`'s BugReport to the missing window, and mint the other two as a separate item. **C** copy as is: the lane may land a one-of-three fix green | A | open | |
| D14 | Catalog versions | `repro-green-refactor@1` (new), `tdd-red-green-refactor` 3 → 4, both packs' `[activity.repro-green-refactor]` rows, CHANGELOG rows (ADR-0072) | approve | open | |

## Risks

- **The `task` literal may meet a lint** that expects every agent input to be a source (the s8h catalog checks,
  `forward_ref_lint`). Fallback: a `kb:` entry (`kb:tdd-red-contract`, `kb:repro-green-contract`), files under
  `.redkiln/kb/` with registry rows. The text then lives in a versioned file. Check this before test 1.
- **The AC ledger.** If the fix-spec's verifications name tests that `green` does not write under the names given,
  verify opens `verify.ac-discriminates` gates (survey 12). Test 18 shows the fixture path. The real path is
  measured on `-3`.
- **AC-105 and AC-303 (rigor and family monotonicity rules) see a new family member.** It is admitted at every
  rigor, so it should compare against itself only. `review::ac303` and `rigor` rules may still move. If they do,
  they are re-pinned under D9, never loosened.
- **Re-prompt inputs and the harness.** Adding `refused` changes the re-prompt's prompt digest. The claude adapter
  serialises inputs generically, so it needs no change. Nothing keys idempotency on inputs. Verify this against
  `dispatch_once_at` before test 9.
- **`step_refused` halts the item.** Under J1, a tool-step `Unsupported` leaves the item halted for good. That is
  consistent with ADR-0095 and with D21-163's mint-a-new-item route, but an operator re-launching the same item
  after a catalog fix will meet the halt. `explain` names it.
- **Wall time.** Implement now runs the repro up to three times (`red-proof`, `repro-verify`, `refactor-repro`)
  besides `affected`. Each run is a `Filter` over the workspace with `--exact`. Its cost is dominated by the first
  build.
- **The rig change (test 17)** must not change any existing proof's visits. A key with one response behaves as
  today.
- **Replay of the operator's `-2` store.** Its committed `red` rows hold `"doctor test"`. Invariant 5 and test 10
  keep a replay or `explain` over it unchanged.

## Size (lower bound)

About 12 Rust files and 1,150 lines: roughly 350 production and 800 test.

| Area | Files | Lines |
|---|---|---|
| rk-kernel | `runtime/reference/selection.rs` (feeds), `runtime/activity.rs` (`StepAt`, check, re-prompt), `runtime/step.rs` (feeds per walk, failure row), `runtime/reference.rs` (re-exports) | about 300 |
| rk-cli | `verbs/run.rs` (stderr arm) | about 20 |
| Tests | `test_selection.rs`, an activity-loop suite, `selection.rs`, `on_false_exec.rs`, `lane_walk.rs`, `step_landing.rs`, `run.rs`, `lane_proof.rs`, `lane_proof/rig.rs` | about 800 |

About 12 non-Rust files:

- `repro-green-refactor.toml`, `tdd-red-green-refactor.toml`, two packs and `CHANGELOG.md`;
- ADR-0205 and D21-177;
- the s8j and s8 runbook amendments and `PROGRESS.md`;
- the mutant driver and its test;
- two charters.

## Progress

- 2026-11-08: plan written. Nothing implemented. Next: the operator decides D1-D14, then test 1.
