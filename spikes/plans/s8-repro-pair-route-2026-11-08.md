# The fix lane runs the committed repro pair (minimal-repro@2), agent selections are refused unless plain, and explain survives a failed step

Branch `claude/project-thread-bzyg5t`, base `origin/build/phase-0-to-1` at `0e569db`. Plan written 2026-11-08 by the
architect. No Rust is written by this plan. Every `H-16` item is in **Decisions** and none is taken here except the
operator's own route ruling (Ryan, 2026-11-08: "the kernel uses the committed repro pair").

## Goal

A real fix-lane run of `rk-fix-doctor-margin` reproduces the defect from the item's committed repro pair
(`.redkiln/intake/<item>.repro.{toml,rs}`), not from agent prose. The pair is placed in the run tree as `.repro.toml`
says and committed on `rk-run/<item>` at the step grain. `confirm` runs exactly that test, and it must be red. The
`ReproCase.command` that verify's `confirmation-test` later re-runs is the placed test's command, so the landed fix
carries a regression test that is green. An agent-produced selection that is not a plain command is refused by name
before the port. `redkiln explain` names a run that stopped at a failed step instead of erroring on a missing
`run.json`. The operator gets a written procedure to restart the item's run on a new frozen sha.

## What happened, and what the survey adds (measured on this tree)

- `minimal-repro@1` `confirm` reads `${attempt.outputs.command}`, which comes from the `investigator` role
  (`.redkiln/cards/investigator.json`: Read, Glob, Grep, StructuredOutput). That role cannot write a test or run a
  command, so its `command` can only be prose or name a test that does not exist. `selection_of`
  (`crates/rk-kernel/src/runtime/reference.rs:1473`) split the prose after ` -- ` into libtest arguments, and
  `--exact\`` failed `--list` in every target: `NonZero`, then `StepFailedAfterCommit`, then exit 14.
- **The fixture proofs passed because the fake did what the real card cannot.** In
  `crates/rk-cli/tests/it/lane_proof.rs:140-149`, the `attempt` response `.writing("crates/rk-lane-fixture/tests/repro.rs", ..)`
  writes a test file. `root-cause@5` `hypothesise` (lines 163-176) also writes `tests/hypothesis.rs`. Both are
  investigator steps. A green fixture proved a route that no real dispatch can take.
- **Which steps feed an agent value into `toolchain:test`.** The source is `crates/rk-kernel/tests/it/lane_walk.rs`'s
  `FIX` and `STORY` rows (the selected activities), plus the catalog and the cards:

  | Lane | Activity | Step's selection | Source step's role | Write tools? | Defect? |
  |---|---|---|---|---|---|
  | fix | `minimal-repro@1` | `${attempt.outputs.command}` | investigator | no | **yes**: the 2026-11-08 failure |
  | fix | `root-cause@5` | `${hypothesise.outputs.discriminating_test}` | investigator | no | **yes**: the next step to fail; its `policy` check `report.failed > 0` cannot hold for an existing, green test |
  | fix | `confirmation-test@2` (verify) | `artifact:ReproCase.command` | investigator (`reduce`) | no | **yes**: the command it re-runs is prose, and the test it names exists only if something committed it on the run branch |
  | fix, story | `tdd-red-green-refactor@3` | `${red.outputs[*].tests_added}` | implementer | yes | no: the implementer writes the test. The new hardening still checks each name |
  | story | `bdd-scenarios`, `t-shirt-sizing`, `docs-update`, `adversarial-review`, `smoke-test`, `sca-scan`, release, closeout | none from an agent | | | no |

  Outside both lanes, `differential-debugging` (`${compare.outputs.discriminating_test}`) and
  `instrumented-trace-analysis` (`${analyse.outputs.discriminating_test}`) have the same investigator defect. `bisect`
  reads `artifact:ReproCase.command`, which this plan fixes at its source. No lane selects any of the three today.
- **The story item meets no read-only defect.** Separately: the real `rk-story-harden-spike-pack` record has no
  `novelty` facet, while the fixture item and `lane_walk`'s `STORY` both set `novelty = "low"`. `story.toml` has a
  `novelty >= high` require row and an `unknown_facet:novelty` expiry row, so the real story run may open an
  `unknown_facet:novelty` gate that the proofs never met. This is not this plan's defect. It is flagged under Risks.

## Constraints and non-goals

- No new dependency, and no new crate edge: `rk-types` gains no `thiserror` and keeps its manual `Error` impls, as
  `acceptance.rs:95` does. No port trait changes: `Toolchain::run_verb` already carries any registry verb (ADR-0170
  decision 2). No new `StepError` variant.
- Cards stay read-only, so `s8h`'s card checks and the paid confinement probes are untouched (operator ruling).
- The kernel stays language-free ("The kernel knows no language", `rk-ports/src/toolchain.rs:383`). `crates/<crate>/tests/it`,
  `--test it` and `--lib` are Rust facts, so they live in `rk-adapter-toolchain-rust`.
- Non-goals: fixing `differential-debugging` and `instrumented-trace-analysis` (a tracked issue; Decision D13);
  `minimal-repro-ue` (Unreal); a retryable, Schema-class refusal at the agent step (Decision D4, option c, as a
  follow-up); the story item's `novelty` facet; checking the pair's `expect` substring inside the run (the confirm's
  `--exact` filter already makes "red" mean "this one test failed").

## Invariants (each is a test below)

1. `minimal-repro@2`'s `confirm` selection is the placed pair's command, never an agent output.
2. The placed test is in `rk-run/<item>`'s tree from `minimal-repro@2`'s `place` checkpoint onward, at
   `crates/<crate>/tests/it/<module>.rs` (`it`) or `crates/<crate>/src/<file>` (`unit`), ending with the pair's
   `.rs` bytes verbatim.
3. Analysis emits `ReproCase` only when `confirm` ran exactly the placed test and it failed (`failed > 0 and passed == 0`),
   or a human answered `repro.cannot-reproduce`.
4. The emitted `ReproCase.command` equals `ReproPlacement.command`, and `selection_of(ReproPlacement.command)` equals
   `[<module>::<test>, "--exact"]`.
5. `parse_selection` refuses any command that holds a newline, a control character or shell syntax
   (`` ` $ ; | & < > ( ) { } " ' \ ``), or that has no ` -- ` filter or an empty one. `selection_of(c) ==
   parse_selection(c).ok()` for every `c`.
6. A missing, unreadable or invalid pair refuses `place` by name (the path and the reason), before any `toolchain:test`
   call, and nothing is written to the run tree.
7. `place` is idempotent. If the target already ends with the pair's bytes, it returns the same placement and writes
   nothing. Any other prior `fn <test>(` in the target is refused.
8. `explain` over an item whose latest run stopped before `write_run` exits 0. It names that run and its failure row,
   then renders the newest earlier run that has a record, if there is one.

## Design

### 1. Where the placement lives (ARC-02): a toolchain verb, fed by a composition-root row input

- **`rk-cli` (composition root)** reads the pair from the **run tree** (`run_tree_of(resolved).path`). That tree is the
  fork commit plus run commits, so its `.redkiln/intake/` bytes are the committed, frozen ones (CODE_PATHS includes
  `.redkiln`). `rk-cli` parses the TOML with its existing `toml` dependency into `rk_types::repro_pair::ReproPairSpec`,
  then runs `TryFrom` into `ReproPair`. It binds the result as the row input `repro_place.pair` through the existing
  `ToolRouting::with_row_input` (`verbs/run.rs:969`, the `advisory_db_row_inputs` site). It always binds the input:
  the value is an envelope (`ReproPairInput`), so an absent or invalid pair refuses at `place`, by name, and never at
  load for every item and stage.
- **`rk-adapter-toolchain-rust`** gains the verb `repro_place` in a new module, `src/repro.rs` (`verbs.rs` is already
  1041 lines; ARC-06). It maps the validated pair to a target path under `config().project_root()`, which is the
  run-owned worktree (ADR-0137 decision 2). It checks the target, appends the bytes, and returns
  `VerbOutput { kind: "ReproPlacement", .. }`. `verbs::run_verb` gains one arm. The Node adapter needs no change: its
  `run_verb` already refuses an unknown verb by name.
- **`rk-kernel`** gains one decode-table arm, `"ReproPlacement"` (`runtime/step.rs:1078`'s match). It also gains the
  hardened parser (section 4). Commit and serve-on-re-entry need no new code: the registry row is
  `side_effecting = true`, so ADR-0188 decision 2 makes `place` a step-grain act that commits to `rk-run/<item>`.
- **`rk-types`** owns the shapes: `ReproPairSpec` (the DTO), `ReproPair` (validated), `ReproPairInput` (the envelope)
  and `ReproPlacement` (the output, in `verb_output.rs`).
- Dependency direction is unchanged: rk-cli → rk-kernel and the adapters → rk-ports → rk-types.

Alternatives (Decision D1): a human-attached `ReproPair` root through a gate (ADR-0200/0202); a kernel seam that
writes files (breaks "kernel knows no language"); placing the test at run entry (ADR-0188's entry landing). Each is
weighed in Decision D1.

### 2. Types and signatures (ownership decided)

`crates/rk-types/src/repro_pair.rs` (new; re-exported from `lib.rs`):

```rust
/// `.redkiln/intake/<item>.repro.toml` as written (s8j §8j.5 step 3). DTO: serde only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReproPairSpec {
    pub kind: String,           // "it" | "unit"
    #[serde(rename = "crate")]
    pub krate: String,
    pub module: String,
    pub test: String,
    pub expect: String,
    #[serde(default)]
    pub file: Option<String>,   // `unit` only, relative to crates/<crate>/src/
}

/// Where the repro goes (API-02: a closed set, not a string plus an optional file).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReproTarget {
    /// Appended to `crates/<crate>/tests/it/<module>.rs`; `module` is one identifier.
    It,
    /// Appended to `crates/<crate>/src/<file>`; `module` is the appended module's full path.
    Unit { file: SrcRelPath },
}

/// A validated pair (private fields; constructed only through `TryFrom`, API-03).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproPair { krate: CrateName, module: RustPath, test: Ident, expect: String,
                       target: ReproTarget, source: String, source_digest: Digest, spec_digest: Digest }

impl ReproPair {
    /// Takes ownership of `source` (the pair stores it). Spec by value (stored).
    pub fn new(spec: ReproPairSpec, source: String, spec_bytes: &[u8]) -> Result<Self, ReproPairError>;
    pub fn krate(&self) -> &str; pub fn module(&self) -> &str; pub fn test(&self) -> &str;
    pub fn expect(&self) -> &str; pub fn target(&self) -> &ReproTarget; pub fn source(&self) -> &str;
    pub fn source_digest(&self) -> &Digest; pub fn spec_digest(&self) -> &Digest;
}

/// What `rk-cli` binds as `repro_place.pair` (an envelope, so absence refuses at `place`, by name).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReproPairInput {
    Present(ReproPair),
    Absent { looked_for: Vec<String> },          // repo-relative paths
    Invalid { path: String, why: String },       // a read, parse or validation failure, rendered
}

/// Why a pair is refused (ERR-01..04). Manual Display + Error, as `acceptance.rs` does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproPairError {
    UnknownKind { found: String },
    UnitNeedsFile,
    ItTakesNoFile { file: String },
    BadCrate { found: String },                  // not [a-z0-9_-]+, or empty
    BadModule { found: String },                 // `it`: one identifier; `unit`: `ident(::ident)*`
    BadTest { found: String },                   // not an identifier
    BadFile { found: String },                   // absolute, `..`, backslash, not `.rs`
    EmptyExpect,
    SourceTooLarge { bytes: usize, cap: usize },
    SourceLacksTest { test: String, found: usize }, // `fn <test>(` must occur exactly once
}

/// The cap on a pair's `.rs` and `.toml` read (H-13). It is 64 KiB: the shipped `.repro.rs` is about 2 KiB, so the cap
/// leaves a 30x margin and keeps an untrusted file from becoming an unbounded prompt input.
pub const REPRO_PAIR_MAX_BYTES: usize = 64 * 1024;
```

`CrateName`, `RustPath`, `Ident` and `SrcRelPath` are private newtypes in the same module (API-02). They are not
re-exported. Their accessors hand out `&str`.

`crates/rk-types/src/verb_output.rs` (add):

```rust
/// `repro_place`'s output: what was appended where, and the one command that runs it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReproPlacement {
    /// The repo-relative file the repro was appended to.
    pub path: String,
    /// `<module>::<test>`, the test's full path inside its target.
    pub test: String,
    /// The plain command; its ` -- ` tail is the selection (`selection_of`).
    pub command: String,
    /// The pair's expected failure substring.
    pub expect: String,
    /// sha256 of the appended bytes, and of the `.repro.toml`, so the journal names what ran.
    pub source_sha256: String,
    pub spec_sha256: String,
    /// `true` when this call wrote; `false` when the target already ended with the bytes (re-entry).
    pub appended: bool,
}
```

Command spelling, fixed by the adapter. `it` gives `cargo test -p <crate> --test it -- <module>::<test> --exact`.
`unit` gives `cargo test -p <crate> --lib -- <module>::<test> --exact`. The port's `Filter` still runs
`cargo test --workspace --no-fail-fast -- <module>::<test> --exact` (`report.rs:636`). `--exact` on a full path
matches one test in the whole workspace, so the scope the display names and the scope that runs select the same test.
Decision D11 covers the gap between the two.

`crates/rk-adapter-toolchain-rust/src/repro.rs` (new):

```rust
/// `repro_place` (registry row `toolchain`, side_effecting). Reads `inputs["pair"]`.
pub(crate) fn repro_place(tc: &RustToolchain, inputs: &BTreeMap<String, serde_json::Value>)
    -> Result<VerbOutput, StepError>;
/// The pure mapping, unit-tested without a filesystem.
pub(crate) fn target_of(pair: &ReproPair) -> (PathBuf /*repo-relative*/, String /*command*/);
```

Order of work: decode the envelope. `Absent` and `Invalid` give `Unsupported { capability: "repro_place: .." }`,
naming the paths and the reason. Then compute the target. The target must exist as a regular file, and its
canonical path must stay under the canonical project root (no symlink escape). Read the target, capped at the
adapter's 2 MiB landing bound (ADR-0188 decision 5's constant, re-read, not copied). If the target ends with
`"\n" + source`, return `appended: false`. If it contains `fn <test>(` anywhere else, refuse with `Unsupported`.
Otherwise append `"\n" + source`, adding a trailing newline when the source lacks one, through
`OpenOptions::append`. Each write failure is `Transport { Io }` (backoff).

Precondition refusals are `Unsupported` (`RetryClass::Never`), not the module's `refuse` helper
(`Transport { Decode }`, which retries with backoff), because a missing pair does not improve when repeated.

`crates/rk-cli/src/repro_pair.rs` (new):

```rust
/// The pair for `item` in the run tree at `tree`, as the envelope `repro_place.pair` carries. Never fails:
/// every failure is an `Invalid` or `Absent` value, refused later by `place`, by name.
pub fn read(tree: &Path, item: &str) -> ReproPairInput;
```

It reads both files with `File::take(REPRO_PAIR_MAX_BYTES + 1)`, so it can tell "too large" from "exactly the cap".
It decodes them as UTF-8, runs `toml::from_str::<ReproPairSpec>`, then `ReproPair::new`. The binding site is
`verbs/run.rs` beside `advisory_db_row_inputs`. It binds only when the chosen adapter is the Rust one; under the
Node adapter the verb is refused by its name anyway.

### 3. The catalog: `minimal-repro@2` and `root-cause@6` (bumped per ADR-0072, `.redkiln/CHANGELOG.md`)

`minimal-repro@2`, in this step order:

1. `place`: `kind = "tool"`, `run = "toolchain:repro_place"`, `outputs = "value:ReproPlacement"`, `retry = { max = 0 }`.
2. `confirm`: `toolchain:test`, `inputs = { selection = "${place.outputs.command}" }`. The string goes through the
   one parser (ADR-0173).
3. `red`: `kind = "check"`, `inputs = { report = "${confirm.outputs}" }`, `expr = "report.failed > 0 and report.passed == 0"`,
   `on_false = "continue"`. This is `confirmation-test`'s check-then-gate pattern.
4. `attempt`: agent, investigator,
   `inputs = { report = "artifact:BugReport", placed = "${place.outputs}", run = "${confirm.outputs}" }`,
   `outputs = "value:ReproAttempt"`, `retry = { max = 2 }`. **There is no `repeat` and no `escalate`.** Asking the
   agent again cannot change a deterministic run, which answers "status reproduced with no runnable repro must not
   satisfy the until": that `until` is gone, replaced by `red` and the gate below.
5. `triage-gate`: `question = "repro.cannot-reproduce"`,
   `when = "${confirm.outputs.failed} == 0 or ${attempt.outputs.status} != 'reproduced'"`.
6. `reduce`: as in `@1`, with `placed = "${place.outputs}"` added to its inputs.
7. `emit`: a composing emit (§9c.34, the form `confirmation-test.toml:45` uses):
   `ReproCase = { status = "${reduce.outputs.status}", steps = "${reduce.outputs.steps}", command = "${place.outputs.command}", expected = "${reduce.outputs.expected}", observed = "${reduce.outputs.observed}", confidence = "${reduce.outputs.confidence}", rationale = "${reduce.outputs.rationale}" }`,
   `evidence = ["${place}", "${confirm}", "${triage-gate?}"]`.

**What the attempt agent is told.** The harness renders a dispatch's inputs as the user message
(`rk-adapter-claude/src/lib.rs:309`'s `user_message`). The agent therefore receives the BugReport, the placement
(path, test, command, expect) and the confirm's `TestReport`. It reads the placed test with Read, judges whether the
red run is the reported defect, and reports `status` and `observed`. It is not asked to produce a runnable command.
`ReproAttempt.command` keeps its shape (no schema migration). Its doc comment changes from "`command` is the
selection a later step re-runs" to "`command`: the placed repro's command, copied from the `placed` input; no step
runs this value". The doc comment is the JSON-Schema `description` the model sees, so the snapshot
`exported__ReproAttempt.snap` changes text only. That is Decision D9.

`root-cause@6` (Decision D6, option A): `confirm`'s selection becomes `artifact:ReproCase.command`, the placed and
trusted command. `policy`'s `report.failed > 0` holds on the red repro. `emit` becomes a composing emit that
overrides `RootCause.discriminating_test` with `artifact:ReproCase.command`, and takes the other members from
`${hypothesise.outputs.root_cause.*}`. The hypothesis itself is tested by `adversary` (reviewer, `rigor >= recommended`)
and, at verify, by the fix turning the repro green.

`.redkiln/registry.toml` gains one row, in a dated comment block that cites ADR-0204:

```toml
repro_place    = { outputs = "ReproPlacement", live = false, timeout = "build_timeout", inputs = ["pair"], route = "toolchain", side_effecting = true }
```

`build_timeout` is an existing measured key. The verb spawns nothing, so the key only bounds the step.

### 4. Hardening: one parser, in `rk-kernel::runtime::reference`

```rust
/// Why a command is not a test selection (refused by name, before the port).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionRefusal {
    Multiline,
    Control { at: usize },
    ShellSyntax { found: char, at: usize },
    NoFilter,          // no " -- "
    EmptyFilter,
    BadToken { token: String },   // an array element with whitespace or shell syntax
}
impl fmt::Display for SelectionRefusal { .. }   impl std::error::Error for SelectionRefusal {}

/// THE one selection parser (ADR-0173; s8d §8d.3): the plain tokens after ` -- `.
pub fn parse_selection(command: &str) -> Result<Vec<String>, SelectionRefusal>;
/// Unchanged signature and meaning for its callers (review_pass's `Unverifiable { cannot_run }`).
pub fn selection_of(command: &str) -> Option<Vec<String>> { parse_selection(command).ok() }
/// One name from a `${…}` list (tests_added): non-empty, no whitespace, no shell syntax.
pub fn check_selection_token(token: &str) -> Result<(), SelectionRefusal>;
```

`selection_plan`'s string arm calls `parse_selection`, and its `RefError.why` names the variant. The 2026-11-08 prose
refuses as `ShellSyntax { found: '(' .. }`. The array arm calls `check_selection_token` on each element (Decision
D5). Both refusals already leave as `selection_refusal`, then `StepError::Unsupported`, before the bound and the port
(`step.rs:756`). The kernel stays language-free: the check rejects shell syntax, not non-cargo heads, so a
`pnpm test -- name` still parses. `ReproAttempt` gets no schema newtype (Decision D4).

Where the checks sit: the shell-syntax refusal is in `selection_of`'s parser, because every agent-fed and repro-fed
selection passes through it (both lanes, `bisect`, `confirmation-test`, `review_pass`). The "no runnable repro"
rule is in the catalog (`red` plus the gate), because only the catalog knows that `confirm` is the reproduction.
The schema keeps its shape (Decision D4).

### 5. `explain` after a run stopped at a failed step (Decision D7)

In `crates/rk-cli/src/verbs/explain.rs`, `run_with` changes. `recorded_run_id` becomes `recorded_run_ids`, which
returns the item's distinct `activity_committed` run ids, newest first; the selector stays unchanged. For the newest
id, `symlink_metadata(<runs>/<id>/run.json)` is checked. On `NotFound`, `explain` writes:

```text
run: <id> (no run record: the run stopped at a failed step before its record was written)
stopped: <kind> <payload's class/capability/step members, canonical JSON>   # the item's last step_failed | step_refused row after <id>'s last activity_committed, if any
```

It then renders the newest earlier id whose `run.json` exists, under a `last recorded run:` line, through the
unchanged `render`. Then it writes the landing block and exits `Exit::Success`. Any other read error stays exit 3,
as today. `render(&RecordedRun)` is untouched, so AC-432's byte-identity holds. `run` still writes the record only on
success (`verbs/run.rs` around line 3125). A partial `run.json` would be a format migration, and the `RecordedRun` it
needs does not exist on the error path.

### 6. Committing on the run branch (question 2)

Yes. `place` is the step-grain act `rk step <item> minimal-repro@2 place <iteration-path>` (ADR-0188 decisions 2 and
3), so the test is on `rk-run/<item>` before `confirm` runs. Implement's `affected` selections (package `rk-cli`) and
verify's whole-suite pass and `confirmation-test` therefore all run it, and the landed fix carries it green. This does
not break Checkpoint 8j (4)'s rule, which reads the **checkout's** `HEAD` (the integration branch) and still requires
the test to be absent there until the operator merges the run ref. After that merge the item's (4) premise is retired
by a dated note (Records). **Step 8 (1) check needs an amendment.** Its content test
`git diff --quiet "$base" "rk-run/$FIX_ITEM" -- crates` would now pass on the placed test alone, so it must exclude
the placed path, read from `.repro.toml` the way Checkpoint 8j (4) reads it.

### 7. Concurrency and resources

There are no new threads or tasks. Every read is capped (`REPRO_PAIR_MAX_BYTES`, plus the 2 MiB target cap). The
append is one `write_all` under the step's bound. The kernel's step-grain act holds the store guard across the
commit, unchanged.

## Test list (red first, in this order)

rk-kernel, `tests/it/test_selection.rs`:
- [x] 1. `parse_selection_reads_the_plain_tail_after_the_separator`: `"cargo test -p rk-cli --test it -- doctor::x --exact"` gives `["doctor::x","--exact"]`.
- [x] 2. `parse_selection_refuses_the_2026_11_08_prose_by_name`: the verbatim `command` from the failed run gives `ShellSyntax`.
- [x] 3. `parse_selection_refuses_a_backtick_a_dollar_and_a_semicolon`, each naming `found` and `at`.
- [x] 4. `parse_selection_refuses_a_newline_as_multiline` and `.._a_control_byte`.
- [x] 5. `parse_selection_refuses_no_filter_and_an_empty_filter` (the existing `selection_of` cases, now typed).
- [x] 6. Property (proptest is already a dev-dependency here): `selection_of(c) == parse_selection(c).ok()`, and any `c` holding a shell-syntax char is `Err`.
- [x] 7. `selection_plan_refuses_a_prose_string_naming_the_refusal`: the `RefError.why` contains `ShellSyntax`, and the runtime test shows no port call (FakeToolchain call count 0).
- [x] 8. `selection_plan_refuses_a_list_element_with_whitespace_naming_it` (D5).

rk-types, `tests/it/repro_pair.rs` and `verb_outputs.rs`:
- [x] 9. `the_shipped_pair_shape_validates`: the JSON equivalent of `rk-fix-doctor-margin.repro.toml` plus a source holding `fn <test>(` once gives `Ok`.
- [x] 10. One test per `ReproPairError` variant (ten), each asserting the variant and its context fields.
- [x] 11. `ReproPlacement` round-trips, rejects an unknown field, and its new schema snapshot `exported__ReproPlacement.snap` exists. Its row joins `verb_outputs.rs`'s table, which is a list extension and moves no existing expectation.

rk-adapter-toolchain-rust, `tests/it/repro.rs`, over a temp project root:
- [x] 12. `an_it_pair_is_appended_to_its_module_file_and_named_by_command`: file bytes, `path`, `test`, `command`, `appended = true`.
- [x] 13. `a_unit_pair_is_appended_to_its_src_file_under_lib`.
- [x] 14. `a_second_place_is_idempotent`: `appended = false`, and the file is byte-identical.
- [x] 15. `a_prior_different_fn_of_the_test_name_is_refused`: `Unsupported` naming the path.
- [x] 16. `a_missing_target_is_refused_naming_it`.
- [x] 17. `an_absent_pair_is_refused_naming_the_intake_paths`, and `an_invalid_pair_is_refused_with_its_reason`. Nothing is written in either case.
- [x] 18. `a_target_symlinked_outside_the_root_is_refused` (POSIX; on Windows a junction, where the platform allows).

rk-kernel, existing suites with expectation changes (Decision D9):
- [x] 19. `verb_registry::SHIPPED_ROWS` goes from 13 to 14 rows (`("repro_place","toolchain","ReproPlacement",false)`), and `WRITING_VERBS` from 5 to 6.
- [x] 20. `tool_verb_routing`: `repro_place` with no row input bound refuses before the port, naming the verb and `pair`.
- [x] 21. `lane_walk`: `LaneToolchain` answers `repro_place`. `minimal-repro` and `root-cause` still `Walks`.
- [x] 22. `forward_ref_lint`: activities stay at 102 and `escalate > 0` still holds through `tdd-red-green-refactor`. The doc sentence naming `minimal-repro` is corrected.

rk-cli:
- [x] 23. `repro_pair::read` returns `Present` for both files, `Absent` when the `.toml` is missing, `Invalid` for malformed TOML (naming the path and the toml error), and `Invalid` for an over-cap `.rs`.
- [x] 24. `processes::the_reserved_fix_items_shipped_pair_validates`: the real `.redkiln/intake/rk-fix-doctor-margin.repro.{toml,rs}` gives `Present`, with target `crates/rk-cli/tests/it/doctor.rs`, and that file exists. A premise test for s8.
- [x] 25. `catalog::no_test_selection_reads_a_read_only_roles_output`: for every activity and every `toolchain:test` step whose selection reads `${<step>.outputs..}`, the source step's role card has `Write` or `Edit`. The scope follows Decision D13, with two tracked known violators.
- [x] 26. `explain::a_run_that_stopped_at_a_failed_step_is_named_not_an_error`: the store holds an `activity_committed` row (run X) and a `step_refused` row, with no `runs/X`. Exit 0, a `run: X (no run record` line, and the `stopped:` line.
- [x] 27. `explain::the_newest_recorded_run_is_rendered_after_a_stopped_one`: run Y is recorded and run X is newer and stopped. Y's render follows.
- [x] 28. `lane_proof::fix_lane_reproduces_through_the_committed_pair_not_agent_prose`. The rig writes `rk-lane-fix.repro.{toml,rs}` (`kind = "it"`, crate `rk-lane-fixture`, module `words`, test `repro_empty_text_has_no_words`). The fixture gains `crates/rk-lane-fixture/tests/it/{main.rs,words.rs}.in`, with one passing test in `words`. The `attempt` and `reduce` responses carry the 2026-11-08 prose as `command`, with **no `.writing`**. `hypothesise` writes no file and its `discriminating_test` is prose. Assertions: (a) an `rk step rk-lane-fix minimal-repro@2 place` commit exists on `rk-run`, and `tests/it/words.rs` there ends with the repro bytes; (b) minimal-repro's confirm `TestReport` has `failed == 1, passed == 0`; (c) the committed `ReproCase.command` equals the placement's command; (d) verify's `ConfirmationResult` is `passing`; (e) the landed tree keeps the repro test.
- [x] 29. `lane_proof::fix_lane_without_a_repro_pair_refuses_at_place_by_name`: no pair in the fixture. The run exits on the refusal, its stderr names `.redkiln/intake/rk-lane-fix.repro.toml`, there is no `TestReport` for minimal-repro, and `words.rs` is unchanged on `rk-run`.
- [x] 30. The existing proofs move to the new activity keys (`minimal-repro@2`, `root-cause@6`), and the old `.writing` of `tests/repro.rs` and `tests/hypothesis.rs` is removed. All 15 existing R_TESTS stay green (Decision D9: a script rewrite).

Mutants (s8j convention, `runbook/tools/s8j-mutants.py`, Python, sha256-verified restores; Decision D9):
- `M-repro-pair-ignored`: `minimal-repro.toml`'s confirm selection goes back to `${attempt.outputs.command}`. Must turn test 28 red, because the prose is refused.
- `M-repro-place-noop`: `repro.rs`'s append is skipped but the placement is still returned. Must turn test 28 red, because the confirm is `Empty` and the gate opens.
- `M-selection-shell-accepted`: `parse_selection`'s shell-syntax arm is removed. Must turn tests 2 and 3 red. This mutant runs in `rk-kernel --test it`. If the driver supports only `rk-cli`, it is recorded as a kernel mutant outside S8J_MUTANTS (Decision D9).

## Records and runbook

- **ADR-0204** (proposed): "The fix lane reproduces from the committed repro pair, placed on the run branch by the
  `repro_place` toolchain verb; agent-produced selections are refused unless plain." It decides:
  1. The route (section 1).
  2. The envelope and its refusals.
  3. The registry row, on the existing `toolchain` route. ADR-0170 decision 1 provides for a later record to add a
     row, so nothing is superseded.
  4. `minimal-repro@2` and `root-cause@6`.
  5. The commit on `rk-run/<item>` at the step grain.
  6. `parse_selection` as the one parser (ADR-0173 stands; its parser refuses more).
  7. `explain`'s stopped-run block.

  It **supersedes in part** ADR-0200 decision 4's bullet *"Nothing reads the directory"* and the bullet's
  *"which Checkpoint 8j (4) reads and no run reads"*, for the repro pair only. Human roots still reach a run only
  through `--attach`. It answers ADR-0200's rejected option *"read the root from a committed file at `run`"*: the
  pair is not an artifact kind and not a root, it is read from the frozen commit's bytes, and its digests are
  journalled on `ReproPlacement`. `superseded_by` on 0200 gains 0204 at the flip commit only (ADR-0092).
- **D21-174** (`spikes/DECISIONS-2026-09-21.md`): Ryan's route ruling of 2026-11-08, quoted. It also records each
  D1-D14 outcome once he decides it, and the restart ruling (D8/D10).
- `.redkiln/CHANGELOG.md`: the `minimal-repro` 1→2 and `root-cause` 5→6 bumps, citing ADR-0204.
- **Dated amendments:**
  - `runbook/phase-1/s8-exit-criterion.md`: (1) check's content test excludes the placed repro path. A note under
    (1) points to the restart procedure below.
  - `runbook/phase-1/s8j-run-preparation.md`:
    - §8j.5 step 3: the repro is never committed on the integration branch before s8; the lane commits it on
      `rk-run/<item>` at `minimal-repro@2`'s `place`.
    - Checkpoint 8j (4): a dated note that (4) is a pre-launch gate, retired for an item once its run ref is merged.
    - §8j.4: the two new proofs and the mutants table.
    - §8j.6 step 1b: `R_TESTS` goes from 15 to 17, the count line becomes `17 passed`, and `S8J_MUTANTS` grows per D9.
  - `runbook/tools/s8j-mutants.py` and `test_s8j_mutants.py`: the new anchors.
  - `runbook/PROGRESS.md`: a dated line.
  - The comments in `.redkiln/intake/rk-fix-doctor-margin.repro.{toml,rs}` ("never committed into the crate").
  - The charters `crates/rk-adapter-toolchain-rust/AGENTS.md` (the verb) and `crates/rk-types/AGENTS.md` (the module) (PROC-11).
- **Evidence lines.** The operator appends these. The implementer never hand-edits `s8j-evidence.txt`.

## Restarting the item's run (question 5): the operator's procedure, no new code (Decision D8)

What the product offers, and why it is not enough:
- **Run-tree reuse** (ADR-0180 decision 9) re-enters the old tree.
- **On an existing `rk-run/<item>` branch**, `RK_BASE` records `merge-base(new HEAD, old tip)`, which is `0e569db`.
  Step 8's `lane_check` then fails, because the fork does not descend from the new frozen sha.
- **Product removal** happens only when the tree is landed (`closed` mark).
- **`doctor --rebuild`** rebuilds **tier-1 only** from `rk-run/*` marks into a new file renamed over the store
  (ADR-0105 clause 7; `doctor/rebuild.rs`). It reconciles from git before touching the store, so it runs whatever
  the store holds. **Every `t2_event` row of every item is dropped.**

Procedure. Rehearse it first on a scratch clone with a copy of the store, the way step 1a was run:
1. Check that no `redkiln` process is live and that `git worktree list --porcelain` shows the tree unlocked. A lock
   left by a dead pid is released with `git worktree unlock` only after checking the pid.
2. Read what the store holds:
   `SELECT item, kind, COUNT(*) FROM t2_event GROUP BY 1,2`, plus the `t1_*` tables. **Stop and report if any item
   other than `rk-fix-doctor-margin` has t2 rows worth keeping**, because the rebuild drops them.
3. Archive: `git bundle create <outside-repo>/rk-run-rk-fix-doctor-margin-2026-11-08.bundle rk-run/rk-fix-doctor-margin`.
   Copy `store.sqlite3`, `-wal` and `-shm`, and `<common>/redkiln/runs/analysis-*`, outside the repository. Do not
   use a tag: a ref left reachable keeps the old `Story:` marks visible to reconcile and to the rebuild.
4. `git worktree remove .redkiln/worktrees/rk-fix-doctor-margin`, unforced. If git refuses over untracked files, list
   them and stop.
5. `git branch -D rk-run/rk-fix-doctor-margin`.
6. `./target/debug/redkiln doctor --rebuild`, over the **new** frozen build. Expect no rows for the item.
7. Move the stale `<common>/redkiln/runs/analysis-f9b24b27126beb68` aside.
8. Launch with (1) launch. Triage, the `human_root:BugReport` attach and the hold are answered again.

**Do stale t2 rows break Step 8?** Only if they were left in place:
- (1)'s probe requires `observed_model` on **every** item `dispatch_completed` row. A stale row from a real dispatch
  most likely carries one, so it would pass.
- (7) requires item readings to equal `dispatch_completed` rows. That holds only if the failed run wrote one reading
  per dispatch.
- Neither can be verified from here, because the store is on the operator's box.
- They also count against the per-item envelope (ADR-0198 decision 4).
- The stale **tier-1** rows anchored to the deleted ref would leave the store disagreeing with git at entry.

The procedure clears them all, so the question becomes moot. The query in step 2 is the measurement.

**Before the next paid launch:**
1. Push and get CI green on the new sha, giving `ci <run> <sha> <date>`.
2. Run §8j.6 step 1b, as amended, at that sha, giving `fixture-proofs <sha> pass <date>` and one `mutant` line each.
3. Append `fix-item rk-fix-doctor-margin <sha>` and `story-item rk-story-harden-spike-pack <sha>` in an evidence
   commit that changes no CODE_PATHS.
4. Re-run Checkpoint 8j (4) at the new HEAD, which is free: the repro is still red at HEAD and still absent from the
   crate.
5. Settle Decision D10: whether the paid steps of §8j.6 re-open.
6. Run the restart procedure.
7. Run (1) launch. Its prespend reads `ci`, `fix-item`, `fixture-proofs` and `record-rt57`.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| D1 | How the pair reaches the run | **A** `repro_place` toolchain verb plus an rk-cli row input envelope; **B** a human-attached `ReproPair` root through a gate (ADR-0200/0202): two files do not fit one `--attach`, and a mandatory human kind would block every bug without a pair; **C** a kernel seam that writes files: breaks "kernel knows no language"; **D** rk-cli places it at run entry (ADR-0188's entry landing): implicit, from triage on, with no typed output in the journal | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D2 | Commit the placed test on `rk-run/<item>` | **A** yes, at `place`'s step grain (`side_effecting = true`); **B** a throwaway tree only: verify's `confirmation-test` then selects nothing and the landed fix has no regression test | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D3 | A bug with no pair | **A** `place` refuses by name (fail closed); **B** fall back to `@1`'s agent command, which cannot run under read-only cards | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D4 | Where "not a plain command" is refused | **a** the one parser (`parse_selection`), plus the catalog's `red` check and gate; **b** also a schema newtype with a `pattern` on `ReproAttempt.command` and co.: a migration, with decode risk for historical rows under `deny_unknown_fields`; **c** also validate selection-feeding fields at the agent step as a `Schema` error (retryable), as a follow-up issue | a now, c as an issue | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D5 | Validate `${…}` list elements (`tests_added`) too | **A** yes: a bad name refuses the run instead of looping on an `Empty` red check; **B** strings only, lists as today | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D6 | `root-cause` under read-only investigators | **A** `@6` confirms on `artifact:ReproCase.command` and composes `discriminating_test` from it; **D** drop `confirm` and `policy` (no executable evidence); **leave**: the restarted run fails at `root-cause` | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D7 | `explain` after a stopped run | **a** exit 0, a `stopped:` block, then the newest recorded run; **b** exit 5 (`NoRecord`) with a message; **c** exit 3 as now, with a clearer message | a | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D8 | Restarting the run | **A** the operator procedure above (git, archive, `doctor --rebuild`), no code; **B** a new verb (`doctor --discard-run <item>`): CLI surface plus a record | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D9 | Test-expectation and fixture changes | `SHIPPED_ROWS` 13→14 and `WRITING_VERBS` 5→6; the `ReproAttempt` schema snapshot description; the `fix_script` rewrite (new keys, `.writing` removed); two fixture files in `lane/`; `R_TESTS` 15→17; three new mutants (which of them join `S8J_MUTANTS`) | approve all; put `M-repro-pair-ignored` and `M-repro-place-noop` in `S8J_MUTANTS`, and `M-selection-shell-accepted` beside it if the driver can run `rk-kernel` | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D10 | Paid steps after the code change | **A** follow §8j.7 to the letter: re-open §8j.6's paid steps (the 7c split and three probes); **B** a dated ruling that they stand, guarded by `git diff --quiet <old frozen> <new> -- crates/rk-adapter-claude crates/rk-containment .redkiln/cards tools/live-run.sh` | B, recorded in D21-174 | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D11 | The displayed scope against the scope that runs | **A** accept: the `Filter` runs workspace-wide with `--exact <path>`, which selects the same single test; **B** a new `Selection::Scoped { package, target, args }`: a port API change | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D12 | New public API | `rk_kernel::runtime::reference::{parse_selection, check_selection_token, SelectionRefusal}`; `rk_types::repro_pair::*`; `rk_types::verb_output::ReproPlacement`; the registry row; the catalog versions | approve (additive; `selection_of` unchanged) | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D13 | Scope of the read-only-role lint (test 25) | **A** the whole catalog, with `differential-debugging` and `instrumented-trace-analysis` as named known violators carrying a GitHub issue reference; **B** the two lanes' selected activities only | A | Ryan, 2026-11-08 06:12Z | as recommended (D21-174) |
| D14 | The story item has no `novelty` facet (an observation) | add `novelty` to `.redkiln/items/rk-story-harden-spike-pack.toml` before its run, or let the `unknown_facet:novelty` gate ask | outside this plan: the operator decides | open | outside this plan |

## Risks

- **The run tree may not exist when rk-cli builds `ToolRouting`.** Verify the order in `verbs/run.rs` before
  test 28. If it does not exist, bind after the tree's creation, or read from the fork commit through
  `rk-git` (a blob read through `GitOp`, a new variant and therefore H-16).
- **nextest and `--exact`.** The real box runs nextest (`Runner` selection, `NEXTEST_ARGS`). The plan assumes
  nextest accepts `-- <path> --exact` and that the doctest companion
  `cargo test --doc -- <path> --exact` selects zero doctests without failing. This is unverified. Test 28 runs on
  the fixture's runner only, so add one adapter test over a nextest-present fixture, or measure on the box.
- **The placed repro compiles only in context.** The shipped `.repro.rs` uses `RULES`, `materialise` and `ctx_for`
  from `doctor.rs`. Checkpoint 8j (4) proved it compiles red at HEAD. A change to `doctor.rs` between now and the
  frozen sha would make the run fail at `confirm` with `NonZero` (the `--list` build), so re-run (4) on the new HEAD.
- **Wall time.** The confirm's `Filter` builds every workspace test target in the run's own target directory
  (measured `list_cold` up to 99 s, under `test_timeout = 360`). Risk is low, but `root-cause@6` repeats it.
- **Adding `tests/it` to the lane fixture** may move counts that existing proofs or `verify_completed` assertions
  read. Test 30 shows it.
- **The story run** may stop at an `unknown_facet:novelty` gate that the fixture never met (D14).

## Size (lower bound)

About 14 Rust files and 1,600 lines: roughly 650 production and 950 test.

| Area | Files | Lines |
|---|---|---|
| rk-types | `repro_pair.rs`, `verb_output.rs` | about 230 |
| rk-adapter-toolchain-rust | `repro.rs`, `verbs.rs` arm | about 180 |
| rk-kernel | `reference.rs`, `step.rs` | about 110 |
| rk-cli | `repro_pair.rs`, `verbs/run.rs`, `verbs/explain.rs` | about 150 |
| Tests | across 10 files, plus 2 fixture files and 2 snapshots | about 950 |

About 12 non-Rust files: `registry.toml`, `minimal-repro.toml`, `root-cause.toml`, `CHANGELOG.md`, two intake
comments, ADR-0204, D21-174, two runbook files, the mutant driver and its test, and two charters.

## Progress

- 2026-11-08: plan written. Nothing implemented. Next: the human decides D1-D14, then test 1.

---

*(dated 2026-11-08)* **Approved.** Ryan approved every recommended answer to D1-D13 on the thread's decision card
(06:12Z), including D10 option B (a dated ruling, recorded in D21-174, that s8j's paid 7c split and three live probes
stand, guarded by an empty `git diff` over `crates/rk-adapter-claude crates/rk-containment .redkiln/cards
tools/live-run.sh`). D14 (the story item's `novelty` facet) is outside this plan and stays open.
- 2026-11-08 (implementer): tests 1-8. Red: `cargo test -p rk-kernel --test it parse_selection` failed to compile,
  `E0432 unresolved imports rk_kernel::runtime::reference::{SelectionRefusal, check_selection_token, parse_selection}`.
  Green: 16 passed in `test_selection`; `rk-kernel` whole 456 passed / 5 ignored. The parser moved into
  `runtime/reference/selection.rs` (ARC-06: `reference.rs` is 1648 lines), re-exported at its old path. The verbatim
  2026-11-08 command is not in the repository (it is in the operator's store), so test 2 keeps its failing features
  (a `(` before the separator and a backtick glued to `--exact`) and says so.
- 2026-11-08 (implementer): tests 9-11. Red: `cargo test -p rk-types --test it repro` failed to compile,
  `E0432 unresolved import rk_types::repro_pair` and `rk_types::verb_output::ReproPlacement`. Green: rk-types 97 passed.
  Judgement calls: `ReproPair` (de)serialises through a private wire form with `try_from`, so a decoded pair is
  re-validated (API-03); `CrateName`/`RustPath`/`Ident` are not newtypes because the fields are private and checked in
  the one constructor; `SrcRelPath` is a pub newtype (private field) because it sits in the pub `ReproTarget::Unit`.
  The accessor is `expected()`, not `expect()`, so no call site reads as `.expect()`. `ReproTarget` derives no
  `JsonSchema` (nothing exports it). The snapshot is `verb_output__ReproPlacement.snap`, not `exported__…`, because
  verb outputs are asserted not in `EXPORTED` and `exported__` is the agent-presented set.
- 2026-11-08 (implementer): tests 12-20. Red (12-18): `cargo test -p rk-adapter-toolchain-rust --test it repro::`, 9
  failed, each refused by the bare name `Unsupported { capability: "repro_place" }`. Red (20 and the decode arm):
  `repro_place_with_no_pair_bound_refuses_before_the_port_naming_it` and the decode-table test failed at "the kind
  `ReproPlacement` has no entry in the kernel's closed decode table". Green: repro 9 + 3 unit; kernel routing and
  registry 18. Test 19's row was written before its count moved, so 19's red was not separately recorded (a D9
  expectation change). The handler lives in `src/verbs/repro.rs` (a child of `verbs`) rather than `src/repro.rs`, so
  `pub(super)` satisfies both `unreachable_pub` and `redundant_pub_crate`. The 11 other adapter failures are
  environmental and unchanged: no cargo-nextest, no cargo-audit, the pnpm fixture not installed.
- 2026-11-08 (implementer): tests 21-22 and the catalog (minimal-repro@2, root-cause@6, CHANGELOG). lane_walk walks
  fix 14/0 and story 13/0; forward_ref_lint's census 74/61/3/10 -> 73/61/3/9. **Three rk-kernel tests went red on the
  catalog change and are NOT re-pinned (an H-16 decision the plan did not settle):**
  `quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies` (root-cause@6's policy loop-back to `hypothesise`
  reads `${confirm.outputs}`, and confirm no longer reads anything hypothesise produces),
  `quality_block::ac527_empty_set_terminal_condition` (check steps 93 -> 94: the new `red` check) and
  `review::ac303_confirm_step_survives_escalation` ((4, 16) -> (4, 15): root-cause's policy no longer consumes the
  adversary's findings through confirm). Reported to the operator with options.
- 2026-11-08 (implementer): tests 23-27. Red: compile errors on `rk_cli::repro_pair` and on `explain`'s stopped block
  (exit 3 "no run record"); test 25 red on the @1 catalog (minimal-repro/confirm and root-cause/confirm beside the two
  known violators). Green: rk-cli repro_pair 10, processes, catalog, explain.
- 2026-11-08 (implementer): tests 28-30. The rig commits `rk-lane-fix.repro.{toml,rs}`; the lane fixture gains
  `tests/it/{main,words}.rs.in`; `fix_script` moves to @2/@6 with prose and no investigator `.writing`. Red: 29 failed
  first on "no journalled refusal names .redkiln/intake/rk-lane-fix.repro.toml" (a toolchain refusal journals no
  failure row, and `run` printed only "did not commit"); fixed by `run` naming the class on stderr (ADR-0204 decision
  7) and moving 29 to the binary arm so stderr is observable. Green: the 15 R_TESTS plus 28 and 29, 17 passed (in this
  container with local, uncommitted shims for cargo-audit and cargo-deny, which are not installed here, and with
  pnpm/node linked into a PATH directory without claude, because nodispatch drops /opt/node22/bin for holding claude).
- 2026-11-08 (implementer): mutants. `M-repro-pair-ignored` moved from confirm's selection to the emit's `command`
  (at @2 `attempt` reads `${confirm.outputs}`, so the plan's site is a dependency cycle the loader refuses, a kill for
  the wrong reason). Killed: `M-repro-pair-ignored`, `M-repro-place-noop` (driver, `--only`), and
  `M-selection-shell-accepted` (`--kernel`). The driver can run rk-kernel, but step 1b's table test holds every
  `S8J_MUTANTS` entry to `R_TESTS`, so the kernel mutant sits in `KERNEL_MUTANTS`, outside `S8J_MUTANTS` (D9's
  fallback). Records: ADR-0204 (proposed), D21-174, the s8 and s8j amendments, PROGRESS row 8, charters, intake
  comments.
- 2026-11-08 (implementer): D21-174 item 4, commit bce6dbd. Ryan's ruling (08:11Z): root-cause@6 opens a gate, no
  loop-back. The `on_false` grammar has no gate form, so policy continues and a gate step `not-reproduced` (question
  `repro.cannot-reproduce`, when `confirm.outputs.failed == 0`) opens on its negation; hypothesise drops
  `findings = ${adversary.outputs?}`. The three tests left red above were re-pinned by dated lines, as approved: d2
  loop rows 5 -> 4 (the crafted must-fail row restores the old loop-back in its copy), ac527 93 -> 94, ac303 (4, 16) ->
  (4, 15), and forward_ref_lint's back-edge list loses root-cause (census 73/61/3/9 -> 72/59/3/10).
- 2026-11-08 (implementer): independent-review findings F1-F8. F1 `check_selection_token` refuses a `-`-led token
  (red: `--include-ignored` accepted); F2 the stopped `run:` line reads "the run did not write its record"; F3 over-cap
  and directory targets pinned; F4 `run_subjects` expects its git log; F6 `run_verb`'s doc; F7 `writeln!` for the
  class line, `stopped_only` returns `Exit`; F8 a target holding the block anywhere is already placed (red: refused as
  a duplicate `fn repro_red(`).
