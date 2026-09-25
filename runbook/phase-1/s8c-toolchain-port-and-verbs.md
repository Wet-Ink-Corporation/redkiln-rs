# Step 8c — The Toolchain port returns outcomes, and verbs route through the registry (Phase 1, part 8c)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8c` | Phase 1 | Checkpoint 8b | Checkpoint 8c | `s8b` | `s8d` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** The fix lane's checks cannot be evaluated against the port as it stands, and the
verbs the later stages need have nowhere to go. Each of the following was re-read at `ca8f387`:

- **`Toolchain::test` throws the outcome away.** `crates/rk-ports/src/toolchain.rs` declares
  `fn test(&self, selection: &[String]) -> Result<u32, StepError>;`, and the Rust adapter's body is
  `self.verify_report(selection).map(|r| r.selected_test_count)`. The report is computed and then discarded.
  A red suite is not a result: the adapter's module doc says *"`Ok(selected_test_count)` on a green suite and
  `Err(NonZero { code, stderr_digest })` on a red one (RT-22)"*. The shipped catalog reads the outcome the port
  cannot return. 34 activities read a tool output's `status`, `failed` or `passed`, the comparison
  `status == 'passing'` appears 25 times, and `tdd-red-green-refactor`'s `red-check` *expects* a red suite. D21-42
  records this as the second premise it found false in D21-41.
- **The review re-executor reads `is_err()` as "fails", which makes a cannot-run candidate count as a flip.**
  `crates/rk-cli/src/review_pass.rs` computes `four_cell(candidate.is_err(), reference.is_err())`, and
  `crates/rk-kernel/src/review/reexec.rs` maps `(true, false) => ReExecution::Flipped`. So a candidate arm that
  returns `Err(Transport { Spawn })` (the program cannot start), with a green reference, reads as a **block**.
  AC-344 says the opposite: *"both-fail or cannot-run resolves to `Unverifiable { both_fail | cannot_run }` …
  never to a block"*. **This latent defect is repaired here** as a side effect of 1a, and §8c.9 pins the repair
  with its own test.
- **There is no verb registry.** `run_tool_step` routes through
  `const MAPPED_VERBS: [&str; 4] = ["build", "change_summary", "lint", "test"];`. `.redkiln/` ships no
  `registry.toml`. D21-10 records that absence *"against design §4.5's five-registry row"* as *"a gap and
  deliberately not filled"*. Measured over `.redkiln/catalog/`'s `run =` lines: **113** `tool` steps across **33**
  verbs. The verbs the later stages need — `smoke`, `sca_scan`, `package`, `item_create` and a release tag — all
  refuse by bare name under ADR-0135 clause 2, which is correct behaviour for a verb nobody routes.
- **The 94 `${toolchain.*}` substitutions have no value source, and no step timeout is enforced.** Measured by key:
  `test_timeout` 29, `implement_timeout` 27, `deploy_timeout` 11, `cook_timeout` 5, `build_timeout` 5,
  `scan_timeout` 4, `review_timeout` 4, `ue_test_timeout` 3, `migration_timeout` 3, `repro_timeout` 2,
  `mutation_timeout` 1. Every one of them is a `timeout_secs` value. Of the **298** `timeout_secs` declarations,
  **113** sit on `tool` steps (52 literal, 61 substituted) and **185** on `agent` steps (152 literal, 33
  substituted). ADR-0140 decision 2 makes the 3600 s construction bound *"THE bound on every child a toolchain
  adapter spawns"*, and its decision 4 records the declarations as *"UNREAD"*.
- **`"affected"` has no meaning an adapter can act on.** It appears 19 times, 17 of them on a `toolchain:test`
  step (ADR-0148's census). The Rust adapter's `select_affected` refuses from a registry row whose probe is
  `none`, and `basis` is *"**always `WholeSuite`**"* (`report.rs`).

🛑 **One premise every record in this area repeats is FALSE, and this stage is where it would be relied on.**
`crates/rk-ports/src/toolchain.rs` says *"Checkpoint 7a counts exactly six on this port"*. ADR-0162's drivers say
*"Checkpoint 7a counts exactly six methods on the `Toolchain` port"*. ADR-0135's `## Decision` says *"the count
stays **six**, as Checkpoint 7a asserts"*. D21-10 repeats it, and so do both adapters' crate docs (*"the six
methods Checkpoint 7a counts and no seventh"*) and `review_pass.rs`. **Checkpoint 7a counts no method.**
Re-read at `ca8f387`, its only method-level group is a *"method floor"* requiring the NAMES `fn detect` and `fn
lint` to be present. `s7a` §7a.3 says so in terms: *"The floor pins **names**, never a signature"*, and its
routing list states that `test`'s selection type *"is a routing. **Checkpoint 7a asserts none of them**"*. Its
counts are five `Send + Sync` traits and five `impl … for` lines in `crates/rk-ports/src/`. **So re-running
Checkpoint 7a, which D21-42 1b requires, would pass over a port with six methods, seven or seventy.** The exact
count is therefore this stage's own assertion, as a named test and as a code-line parse (§8c.9 (4)). The
false sentences are corrected with the records (§8c.7).

**This step executes five decisions and takes no free one.** D21-42 1a, 1b, 4a, 4b and 4c (the adapter side) are
taken. What is left is execution order, plus the three records those decisions say must be written (§8c.7). Where
this file had to choose a shape that no record fixes, the shape is written as the record's proposal and named as
such. **Wherever an accepted record binds and this file does not settle the question, the file says so and stops
there. It does not decide.**

**What this step does NOT do**, each with its owner:

| not here | owner |
| --- | --- |
| Resolving a declared `selection` (`"affected"` → the Changeset's paths, `artifact:…`, `${…}`) and lifting ADR-0148's refusal of `toolchain:test` in the executor | `s8d` (D21-42 4c resolver side; ADR-0148 clause 4: *"This is reversed by a resolver, not by a revert"*) |
| Binding a tool step's typed output to `${step.outputs}`, and the run-time half of `GreenAt` (*"and status is passing"*, `catalog.rs`) | `s8d` (the complete reference resolver, D21-41 d) |
| Verb rows for `shallow_clone` and `verdict_record` | `s8d` (the stage plan) |
| The verify pass and `verify_completed` | `s8g` (D21-41 c as superseded by D21-42 1a; D21-42 10b) |
| The producer ACTIVITIES `smoke-test`, `sca-scan`, `build-package`, `tag-release`, and the shipped `smoke` checklist | `s8g` (D21-42 6a, 6b) |
| `item_create`'s rk-store write | `s8f` (the stage plan: *"DebtRegisterEntry's rk-store item write (6a)"*) |
| Any dispatch to a real account | nobody before `s8i` (D21-40 decision 2) — **this stage dispatches nothing** |

---

### 8c.1 The count source is measured first, and a missing one is a halt

1a needs `passed` and `failed` from a red Rust suite, and **the pinned stable toolchain may not provide them in a
form AC-329 admits**. The AC is explicit: *"no field of the report is obtained by parsing prose or markdown"*.
`s7c` measured the options on 1.98.1: *"`libtest --format json` … **does not exist on 1.98.1**"*, `--format junit` is
gated the same way, nextest is ABSENT, and *"`RUSTC_BOOTSTRAP=1` is a workaround Halt 7 forbids"*. The prose summary
line is what the ban is aimed at, and the fixture project carries **two decoys** built to catch a parser of it:
`tests/zz_decoy.rs` prints the summary line and a per-test `... ok` line on stdout, and `build.rs` does the same on
stderr.

**Measure before any port code changes, and record the measurement in record T's evidence (§8c.7).** A candidate
source is admissible only if **all six** of these hold on the pinned toolchain, over the Rust fixture project:

1. It is **stable**. No `-Z`, no nightly, no `RUSTC_BOOTSTRAP`.
2. It is **structured and per-test**, with one record per executed test naming its outcome, and it is read from a
   file or a structured stream, **never from stdout prose**.
3. It **survives both decoys**. Neither the `zz_decoy` stdout lines nor the `build.rs` stderr lines move a count.
4. It survives **`cargo test --workspace` running several test binaries in sequence**. A per-binary log path that
   each binary truncates in turn reports only the last binary, so the source must be keyed per binary, or it must
   come from running each binary itself. In that second case the executables are enumerated from cargo's
   structured `--message-format=json` build output, which is stable.
5. **`passed + failed` equals the `--list`-derived executed count** (`selected_test_count`) on the green fixture
   AND on the red fixture (`red_by_selection`). Doctests are included: `cargo test -- --list` lists them. A
   source that cannot see doctests fails this, and the report does not quietly redefine the count to fit.
6. A `harness = false` target is accounted for, because it has no per-test records. The measurement states what
   the source reports for one, over `zz_decoy`, which exits 0.

The one known candidate is libtest's stable `--logfile <path>`. Hazards (4) and (5) apply to it. Installing
`cargo-nextest` is a second candidate, since ADR-0094 decision 1 already admits *"`cargo nextest run` where
present, recorded on the report"*. It is **an operator decision (a tool install), asked, never assumed**.

🛑 **If no candidate satisfies all six, this stage HALTS (Halt 7).** Do not parse the summary line, do not set
`RUSTC_BOOTSTRAP`, and do not report `failed = 0` for a non-zero exit. Record the finding and stop. A red suite that
reads as `Ok` with `failed = 0` is the exact inversion 1a exists to remove.

The Node adapter needs no measurement. `report.rs` already reads *"`numPassedTests`, `numFailedTests`,
`numPendingTests`, `numTotalTests`"* from vitest's JSON reporter file.

### 8c.2 `Toolchain::test` returns an outcome (D21-42 1a)

**The shape.** Record T fixes it, and this is the proposal:

- **`test` returns a `TestOutcome`**, an `rk-ports` type that wraps the unchanged `rk_types::verify::VerifyReport`
  and adds three fields: `passed: u32`, `failed: u32` and `status: TestStatus`.
  - `VerifyReport` gains no member. AC-329's member list is closed, the store's `validate_report` (ADR-0104
    decision 3) reads it, and `deny_unknown_fields` guards it.
  - `status` is **derived and never stored independently**. It is `Passing` iff `failed == 0 && passed > 0`,
    `Failing` iff `failed > 0`, and `Empty` iff `passed + failed == 0`.
  - `Empty` is AC-332's *"'zero tests executed' is never rendered as green"*. The catalog compares `status` only
    to `'passing'`, so an empty run can never satisfy a check.
  - The serde form spells `status` as `passing` / `failing` / `empty`, which is the vocabulary the 25 catalog
    comparisons already use.
- **Invariant.** `passed + failed == report.selected_test_count`, checked at construction. If it fails, the result
  is `Err(Transport { Decode })` naming both numbers. It is never a corrected count.
- **`Ok` means the suite ran and every executed test reported, green or red.** A red suite is
  `Ok(TestOutcome { failed ≥ 1, status: Failing, … })`.
- **`Err` means exactly three things.** (a) Cannot run: the program cannot start (`Transport { Spawn }`, AC-344), or
  the build fails before any test executes (`NonZero` with no per-test record). (b) Killed: `Timeout { after_secs }`,
  or an OS kill with the `os-killed:` prefix (AC-331). (c) The files around the run are unreadable (`Transport { Io
  | Decode }`). **A non-zero exit that DID produce per-test records is a red suite, not an error.**
- **RT-22 is reversed by record** and **RT-6 is discharged**. The inherent `verify_report(&self, …)` methods on
  `RustToolchain` and `NodeToolchain` are **retired**, so that exactly one path produces a report. The module
  functions `report::verify_report` stay as the port method's body. **R-9 is discharged for the transport**,
  because the report now crosses the port. The verify pass that consumes it is `s8g`'s.
- **The method count does not move under 1a.** 1a changes a signature and 1b adds the seventh method (§8c.4).

**Every caller and every double moves in the same slice.** Measured at `ca8f387`: six `impl … Toolchain for` sites
in `crates/`, plus one fixture PROGRAM. The facts brief placed three of them in `fakes.rs`. Only one is there.

| site | what changes |
| --- | --- |
| `rk-ports/src/fakes.rs` `FixtureToolchain` | `test` returns an outcome. `passed = selection` length, `failed = 0` by default, and a builder scripts `failing(n)`. The report members are non-empty fixture values. The `impl … for` count in `crates/rk-ports/src/` **stays 5** (Checkpoint 7a) |
| `rk-ports/src/conformance.rs` `conform_toolchain` | `let ran = toolchain.test(&[]).expect(…)` reads `outcome.passed` against the row's count, and gains **`validate_report`** (below) |
| `rk-cli/src/verbs/run.rs` `UndetectedToolchain` | every method, `run_verb` included, refuses by name as today |
| `rk-cli/tests/it/review_pass.rs` `ScriptedToolchain` | three modes: green (`Ok`, `failed = 0`), red (`Ok`, `failed ≥ 1`) and cannot-run (`Err`). **Red is no longer `Err`** |
| `rk-kernel/tests/it/runtime.rs` `CountingToolchain` | counts `run_verb` too, and records the `StepBound` it was handed (§8c.6) |
| `rk-cli/src/review_pass.rs` `ToolchainReExecutor::run` | `four_cell(candidate.failed > 0, reference.failed > 0)` when both arms are `Ok`. **If either arm is `Err`, it returns `None`**, which `confirm_with` reads as `Unverifiable { cannot_run }` (AC-344). A red candidate's `observed` string names its counts and can never equal `expected_when_met` |
| `rk-cli/tests/fixtures/bin/rk-fixture-toolchain.rs` (a stub `cargo`, not an impl) | it must emit the §8c.1 source for its one test: failed in the candidate tree, passed in `…review-ref`. **If it does not, the candidate reads as cannot-run, and AC-438's tenth row (`exit/findings-block/`, code 4) silently becomes exit 0**, because that fixture is the ONLY data condition that reaches `four_cell(true, false)` |
| `rk-adapter-toolchain-node/tests/it/cancel.rs`, `…/ac334.rs` (corpus-bound) | their `Err(NonZero)` arms become `failed > 0` reads. `ac334`'s `(false, None)` mapping of `NonZero` is exactly the reading 1a reverses |

**The conformance row gains `validate_report` (AC-329 as a port conformance row).** `rk_ports::conformance::
validate_report(&TestOutcome)` runs inside `conform_toolchain` for the fake row and for both real rows. It checks
`VerifyReport::validate_shape()` (from `rk-types`, which `rk-ports` already depends on), the count invariant, and
that `status` agrees with the counts. **It does not duplicate the store's whole-suite rule**:
`rk_store::artifacts::verify_report::validate_report` stays the boundary validator (ADR-0104 decision 3), and
`rk-ports` cannot depend on `rk-store`. The adapters' existing `satisfies_store_whole_suite_rule` helpers keep
asserting that rule by name.

**AC-329's Verified-by** already reads *"schema conformance test over every adapter's fixture reports"*. This row
is that instrument, so the register takes no amendment.

🛑 **`Ok` stops meaning green on this line, and every reader of it must move.** Before the slice closes, run
`grep -rn '\.test(' crates/*/src crates/*/tests --include=*.rs` and read every hit. Each one must read `failed` or
`status`, and none may read `is_ok()` / `is_err()` / `?` as a verdict. §8c.9 (11) keeps a code-line floor on the
one site whose regression is silent: `review_pass.rs`.

### 8c.3 A selection is a type, and `"affected"` is the package-level affected set (D21-42 4c, adapter side)

`test`'s selection is `&[String]` today. `s7a` §7a.3 calls that a routing (*"`test`'s selection type … is a
routing"*), and ADR-0148 measured why no declared selection string can be passed through (*"`cargo test affected`
matches nothing and **exits 0**"*). Record T replaces the slice with a closed `rk_ports::toolchain::Selection`:

| variant | meaning | Rust adapter | Node adapter |
| --- | --- | --- | --- |
| `WholeSuite { reason: BasisReason }` | the whole suite; the reason travels with the call | `cargo test --workspace`; `basis: WholeSuite`, `basis_reason: Some(reason)` | whole suite, same |
| `Filter(Vec<String>)` | libtest / vitest arguments. What `review_pass::selection_of` already produces from a repro's `--` tail (ADR-0162) | `-- <args>` as today | `-t` / file as today |
| `Affected { changed: Vec<String>, recorded: Option<RecordedWholeSuite> }` | the change set, plus the most recent recorded whole-suite count (`run_id`, `toolchain_version`, `count`) the caller read from the store | below | refused by name: `Unsupported { capability: "affected-selection" }` from its `Probe::None` row (AC-328) |
| `Named(String)` | a selection the adapter DECLARES | a name it does not declare is refused by name | same |

**The Rust `Affected` rules, in order.** Record T fixes them. The reasons come from the closed six (AC-352), and
**no seventh is minted**.

1. The `affected-selection` registry row must carry `Probe::Recorded`. If it does not, the call is refused from
   the row (AC-328, unchanged). §8c.9 records the probe: a dated `Recorded { source }` naming the fixture workspace
   and the test that exercises it. The `verified_on` staleness comparison stays the OWED bridge the registry's own
   doc names.
2. An **empty** change set is refused by name. An affected run over nothing is a caller error. It is not a green,
   and it is not a whole suite.
3. Every changed path maps to the workspace package whose manifest directory is its longest prefix. This is read
   from `cargo metadata --format-version 1`, a structured stable read that ADR-0094 decision 1 already names.
   **A path in no package** — the root `Cargo.toml`, `Cargo.lock`, `rust-toolchain*`, `.cargo/` or any non-crate
   path — forces the whole suite. Record T picks the reason from the six and states why: the proposal is
   `PolicyForced`, with this rule as the adapter's declared policy.
4. The affected set is the touched packages plus their **reverse-dependency closure** over the workspace's
   `resolve.nodes`, **including dev-dependencies**, because a test in `b` that dev-depends on `a` is affected by a
   change to `a`. The set becomes `-p <pkg>` flags in place of `--workspace`. **That is a deviation from ADR-0094
   decision 1's *"`test` = `cargo test --workspace`"*, and record T supersedes that clause in part** (§8c.7).
5. If `recorded` is absent, or its `toolchain_version` differs from this run's `rustc -V`, the adapter runs the
   whole suite with `ProbeUnverified`. `BasisReason`'s own doc: *"AC-326's bootstrap: no whole-suite count recorded
   yet"*.
6. If the `-p` set lists zero tests (`--list`), the adapter escalates to the whole suite with `SelectionEmpty`.
   AC-332: *"it either escalates … or fails, according to the declared policy"*. The declared policy is
   escalate.
7. Otherwise the adapter runs the `-p` set with `basis: Affected`, `basis_reason: None`,
   `total_test_count = recorded.count` and `total_test_count_source: RecordedRun { run_id }`, which is exactly what
   the store's rule requires. **AC-326's `selection_ineffective` reclassification stays the kernel's** and is not
   moved.

**The affected set is a pure function.** It is computed by
`affected_packages(changed, &metadata, policy_version) -> AffectedSet`, with the parsed `cargo metadata` passed
in. The adapter reads the metadata and then calls the function. It never reads the tree, the clock or the
environment inside the computation. AC-347 requires *"a pure function"*, and `s8g`'s AC-348 test
(`selection::ac348_selection_is_byte_identical_over_200_change_sets`) **stops `s8g` if this stage built it
impure**.

**`"e2e"` is refused by name on both adapters at this stage.** The only `"e2e"` selection in the catalog is
`walking-skeleton.toml`'s. No record says what *e2e* means for a Cargo workspace, and inventing a meaning is the
shape ADR-0148 refused. D21-42 4c allows *"declared per adapter or refused"*, and this stage refuses.
⚠️ **If `s8j`'s story candidate selects `walking-skeleton`, the declaration becomes owed there. It is named here so
that the owner is not discovered by a refusal at run time.**

**Fixtures.** Add `crates/rk-adapter-toolchain-rust/tests/fixtures/workspace/`, its own workspace root like
`project/`, holding three packages `a` ← `b` ← `c` (`b` depends on `a`, `c` dev-depends on `b`), a `[[bin]]` target
for §8c.4's `smoke` and `package`, and a committed `Cargo.lock`. Everything is LF under ADR-0164, so no `-text` pin
is needed.

### 8c.4 The verb registry ships, and the port gains one generic method (D21-42 1b)

**`.redkiln/registry.toml` fills design §4.5's verb table.** The row is design §4.5's: *"`registry.toml` (toolchain
verbs and feeds; a verb's `live = true` implies `memo = false` on every step using it)"*.

**One parser, not two.** `rk_kernel::workflow::memo::VerbRegistry` already parses `spikes/p9/registry.toml`'s
`[verb]` shape for AC-018, refuses a row without a boolean `live`, and refuses an empty table. It is **extended**.
Writing a second verb-table reader would give the same file two meanings.

Every row declares:

- `outputs`, the typed output kind;
- `live`;
- `timeout`, a `${toolchain.*}` key;
- `inputs`, the names the call carries;
- `route`, from a **closed** set with an exhaustive match and no wildcard, on ADR-0135 clause 1's own idiom:
  - `port` — the four existing methods;
  - `toolchain` — the new generic method;
  - `store` — an rk-store write;
  - `git` — through the journalled `GitOps` seam.

`s8d`'s record may add a route for `verdict_record`. It does so by record, not by editing the match.

**The rows this stage lands, and who owns each producer:**

| verb | route | typed output | handler built here | producer activity — owner |
| --- | --- | --- | --- | --- |
| `test`, `build`, `lint`, `change_summary` | `port` | `TestReport`, `BuildReport`, `LintReport`, `ChangeSummary` (the spike's own `outputs`) | the existing port methods | — |
| `smoke` | `toolchain` | `SmokeResult` | **yes**, Rust: build, then run each declared checklist row against the built binary | `smoke-test` — `s8g`. `checklist:smoke` resolution is `s8d`'s; the shipped `smoke` checklist is `s8g`'s |
| `sca_scan` | `toolchain` | `ScanReport` | **yes**, Rust | `sca-scan` — `s8g` |
| `package` | `toolchain` | `BuildArtifact` | **yes**, Rust: `cargo build --release --locked --message-format=json`, then the sha256 of each **declared** shipped binary — never a glob, because a release build also links `rk-fixture-toolchain` and the other fixture `[[bin]]`s | `build-package` — `s8g`, which writes this repository's declared set (`redkiln`, `redkiln-migrate`) into the per-toolchain table |
| `item_create` | `store` | `ItemRefs` | **no**. Refused naming the route until `s8f` | `debt-logging` — the write is `s8f`'s |
| `release_tag` | `git` | `ReleaseTag` | **no**. Refused naming the route until `s8g` | `tag-release` (new, via ADR-0114 clause 3's trial) — `s8g`. Local, never pushed (D21-42 6b) |
| `shallow_clone`, `verdict_record` | — | — | not here | `s8d` |
| the other **23** verbs, `release_record` and `mutate` among them | no row | — | refused by **bare** name (ADR-0135 clause 2, unchanged) | Phase 2 or unowned |

**The seventh method.** Record V fixes it, and this is the proposal:
`fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError>;`

- `VerbCall` carries the verb, its resolved inputs as `serde_json::Value` by name, and the `StepBound` (§8c.6).
- `VerbOutput { kind: String, value: serde_json::Value }` is erased at the port. **The kernel types it at the
  boundary**, as `report_at_boundary` already does for `VerifyReport`. A closed decode table maps each declared
  kind to its `rk-types` struct. An output whose `kind` is not the row's `outputs` is refused, naming both kinds. A
  value that does not decode is refused, naming the kind.
- **An adapter implements the verbs it can and refuses every other one by name** (`Unsupported { capability:
  "<verb>" }`). The Node adapter implements none at this stage.
- **`run_verb` is not a patch applier.** ADR-0162's refusal of `ReproKind::Mutate` stands, and no `mutate` row
  lands. Record V says so, because that forbids bullet (*"forbids applying a mutation patch from `rk-cli`"*) is the
  one a generic method would make tempting.

**Typed outputs.** `SmokeResult`, `ScanReport`, `BuildArtifact`, `ItemRefs` and `ReleaseTag` are added as
`rk-types` structs with `#[serde(deny_unknown_fields)]`, no `Default`, and every member required. **They are not
added to `EXPORTED`.** Those schemas are presented to agents, and which kinds an agent emits is `s8h`'s (D21-41 g).
Keep them `oneOf`-free anyway, because Checkpoint 5 greps the snapshot directory for it. What each struct carries:

- **`SmokeResult`**: one row per checklist entry (`id`, the argv run, its `cwd`, the expected exit, the actual
  exit, the stdout predicate and whether it held, `passed`), plus `passed`, `failed` and `status`. The checklist
  item form (`argv`, `cwd`, `expect_exit`, one stdout predicate) is the one `s8g` §8g.6 authors at
  `.redkiln/checklists/smoke.toml`. The handler runs the binaries the call's build produced, never an installed
  `redkiln`. It follows 1a's rule: **a checklist row that fails is `Ok` with
  `failed ≥ 1`**, and `Err` means only that the build or a spawn could not run, or that the run was killed. The
  catalog reads `smoke.status == 'passing'`.
- **`ScanReport`**: `critical`, `high`, `medium`, `low` and **`unrated`**, each required; one entry per advisory
  (id, package, version, severity or none); the tool and its version; and **the advisory database's commit**
  (D21-42 6a). **An advisory with no severity is counted in `unrated`. It is never counted as low or none, and it
  is never dropped.** Whether `sca-scan`'s check `report.critical == 0 and report.high == 0` must also read
  `unrated` is the ACTIVITY's question, and so it is `s8g`'s. ⚠️ As shipped, that check passes with
  `unrated > 0`.
- **`BuildArtifact`**: the profile, one `(name, relative path, sha256)` per **declared** binary, the argv, the
  toolchain version, and the input `sha` together with the project root's `HEAD` read through `rk-git`. The
  digest is `rk_types::digest::sha256`, the plain hash, so that an independent `sha256sum` agrees. **A declared
  binary that the build did not produce is refused, and so is an `sha`/`HEAD` mismatch.** A package built from a
  tree other than the one named is not that sha's artifact. An undeclared set is refused by name.

**The scan's semantics are the ones `s8g` §8g.6 reads, and record V fixes them so that the two files agree.** Both
`cargo audit --json` and `cargo deny check advisories` run. Severity maps each advisory's CVSS base score onto the
CVSS v3 qualitative bands: `critical` ≥ 9.0, `high` 7.0–8.9, `medium` and `low` below. An advisory with no CVSS is
`unrated`. The database commit is read from the report's own database member, and **if the installed tool emits no
such member, the scan refuses by name** rather than record an unknown database. The verb runs **offline, against a
pinned local database** (`--no-fetch` or its equivalent), because a scan whose database moved mid-lane is two
instruments (golden constraint 8). **Measure first**, as §8c.1 did for the count source:

- that both tools emit the database member and a per-advisory CVSS on 0.22.2 and 0.20.2;
- whether scoring a CVSS vector needs a new dependency. If it does, that is a `deny.toml` licence question, and
  record V names the crate.

The tests use a fixture database under `crates/rk-adapter-toolchain-rust/tests/fixtures/advisory-db/`. It holds one
advisory with a severity and one without, and it matches a crate in a fixture lockfile.

**The executor routes through the registry.** In `run_tool_step`, `MAPPED_VERBS` is replaced by a registry lookup.
The table below lists the three existing refusal shapes and the two new ones. **Each refusal must be tellable from
the others in the journal**, because ADR-0148 clause 2 already requires that of the first two:

| case | outcome |
| --- | --- |
| no row | ADR-0135 clause 2's bare-verb refusal, **unchanged** |
| `test` | ADR-0148's sentence, **unchanged** (s8d reverses it) |
| a `port` row | its port method, now given the step's bound |
| a `toolchain` row whose declared inputs the step cannot supply without the resolver | refused **before the port**, naming the verb and the first unresolved input — the resolver is `s8d`'s. On the shipped catalog this is every `smoke` / `sca_scan` / `package` step until `s8d` |
| a `store` / `git` row | refused naming the route and the stage that builds it |

⚠️ **On the shipped catalog, therefore, nothing new walks at `s8c`.** The three verb implementations are proved
over fixtures, through the port, by the adapter's own tests (§8c.9 (7)). The positive control that the executor
reaches `run_verb` is a kernel test over a test-only registry row that declares no inputs. **This is said so
that nobody reads a green Checkpoint 8c as *"smoke runs in the lane"*. It does not, until `s8d` and `s8g`.**

⚠️ **Two things the verb table does not settle, named rather than decided:**

- **`build` has no outcome.** `build-package`'s `built` check reads `report.status == 'passing'` over
  `toolchain:build`, whose port method returns `Result<(), StepError>`. A red build is `Err`, so that check can
  never read a failing build. The step refuses, which fails closed, but its `on_false = "fail"` arm is dead. 1a
  widened `test` only.
- **No row for `[toolchain.feeds]` ships.** `sca-scan`'s `sbom = "toolchain:sbom?"` input has no source, and the
  resolver (`s8d`) will need one.

Both are reported as unassigned (§8c.12).

### 8c.5 The per-toolchain value table (D21-42 4a), and the measurement that is its warrant

`registry.toml` carries a table per toolchain, keyed by the `toolchain` facet's value (`[toolchain.rust]`,
`[toolchain.node]`). Each `${toolchain.<key>}` has **either** a value with its warrant **or** a declared
`unmeasured = "<reason>"`. Every value carries its warrant as structured fields: `secs`; `measured`, the list of
readings; `rule`, how the value follows from the readings; `pin`, the 40-hex commit the readings were taken at;
and `command`. **The loader refuses a value without a warrant, and resolution refuses an `unmeasured` key or an
absent key by name** (§8c.6).

**Keeping the table out of the kernel (AC-487).** The kernel must not name an adapter:
*"`rk-kernel` contains no comparison against an adapter identity"*. So `rk-cli`, which already holds the name
`real_toolchain` chose, selects the table for that adapter's toolchain. It hands the kernel a flat
`ToolchainValues` (key → seconds, or key → `unmeasured`). The kernel never sees `rust`.

**What each key's warrant is, for `[toolchain.rust]`:**

| key | warrant |
| --- | --- |
| `test_timeout` (29) | **measured here** (below): the slowest child of a cold `cargo test --workspace` pass, n = 3 |
| `build_timeout` (5) | **measured here**: a cold `cargo build --workspace`, n = 3 |
| `scan_timeout` (4) | **measured here**: the offline scan over the pinned database, n = 3 |
| `implement_timeout` (27), `review_timeout` (4), `repro_timeout` (2) | these sit on **agent** steps. Warrant each by citing an existing dispatch-duration record at its corpus pin (for example F31's S-TYPED *"median $3.03 / 1,071 s, slowest 4,561 s"*), **or declare it `unmeasured`**. No number is chosen without a record |
| `deploy_timeout` (11), `migration_timeout` (3), `cook_timeout` (5), `ue_test_timeout` (3), `mutation_timeout` (1) | `unmeasured`, each with its reason. No Phase-1 Rust verb deploys, migrates, cooks, runs Unreal automation or mutates |

**The table also admits declaration rows beside the timeouts.** A declaration row is a fact about the toolchain
that is not a duration, so it carries a `source` rather than a `measured` list. Two are expected:

- `package_binaries`, the declared shipped set §8c.4's `package` reads. `s8g` writes this repository's set.
- `s8g`'s `claims` column (AC-349).

Adding a row is a record's act, like every other change to the table.

`[toolchain.node]` is declared, and every key in it is `unmeasured`. The self-hosting run is Rust. A Node value would
be a guess.

**The `rule` is record V's, stated once and applied to every measured key.** It turns readings into a value (for
example, *twice the slowest reading, rounded up to the minute*). Golden constraint 6 applies: *"size the ceiling
against the slow one"*. **Write the rule into the record before the readings are taken, and never choose it after
seeing which value it makes convenient.** A value above the construction bound is legal. §8c.6's `min` makes the
construction bound win, and the record says so rather than clamping the value.

**What "cold" must mean.** It is the condition the adapter meets in a run-owned worktree: a fresh detached
worktree, its own empty `CARGO_TARGET_DIR`, and the Node fixture's `node_modules` installed. The Rust adapter
spawns three children per `test` call, **and the cold compile lands in the FIRST, the `--list` pass, not in the
run**, so each child is timed separately and the warrant is the slowest child. The Rust adapter's `timeout`
bounds each child it spawns (ADR-0140 decision 2), not the call.

🛑 **A fresh run-owned worktree runs this repository's suite RED**, and the measurement surfaced why. It has no
`node_modules`, so 10 Node tests fail at `tests/it/support.rs`' preflight. The measurement installs `node_modules`
so that it times a green suite. **A real `redkiln run` does not install it.** So the first self-hosting verify
would report `failed = 10` for a reason unrelated to the change. This is reported as unassigned (§8c.12).

```bash
cd D:/repos/redkiln-rs
# Run as a script. ONE cold pass; run it THREE times (n = 3) and record every MEASURED line of every pass.
# It adds one detached worktree and REMOVES it on every path; the path set is compared at the end.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
base="$(git rev-parse --git-common-dir)/redkiln/tmp"
test -d "$base" || { echo "FAIL: $base absent — the scratch location ADR-0137 clause 6 names does not exist"; exit 1; }
pin=$(git rev-parse HEAD); rc=$?
[ "$rc" -eq 0 ] && [ -n "$pin" ] || { echo "FAIL: git rev-parse HEAD rc=$rc"; exit 1; }
mw="$base/cold-$(date -u +%Y%m%dT%H%M%SZ)"
out=$(git worktree add --detach "$mw" "$pin" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree add rc=$rc"; printf '%s\n' "$out"; exit 1; }
out=$(pnpm install --frozen-lockfile --dir "$mw/crates/rk-adapter-toolchain-node/tests/fixtures/project" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: pnpm install rc=$rc (network; local-only)"; printf '%s\n' "$out" | tail -10; git worktree remove --force "$mw"; exit 1; }
t0=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-test" cargo test --manifest-path "$mw/Cargo.toml" --workspace -- --list 2>&1); rc=$?; t1=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the cold --list child rc=$rc"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t2=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-test" cargo test --manifest-path "$mw/Cargo.toml" --workspace -- --list --ignored 2>&1); rc=$?; t3=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the --list --ignored child rc=$rc"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t4=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-test" cargo test --manifest-path "$mw/Cargo.toml" --workspace 2>&1); rc=$?; t5=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the run child is RED (rc=$rc) — a red suite's wall time is not the warrant for a green one"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t6=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-build" cargo build --manifest-path "$mw/Cargo.toml" --workspace 2>&1); rc=$?; t7=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the cold build rc=$rc"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t8=$(date +%s); out=$(cargo audit --file "$mw/Cargo.lock" --no-fetch --json 2>&1); rc=$?; t9=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the offline scan rc=$rc — no local advisory database, or advisories on this tree; read it"; printf '%s\n' "$out" | tail -10; git worktree remove --force "$mw"; exit 1; }
echo "MEASURED pin=$pin list_cold=$((t1-t0)) list_ignored=$((t3-t2)) run=$((t5-t4)) build_cold=$((t7-t6)) scan=$((t9-t8))"
out=$(git worktree remove --force "$mw" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the measurement worktree did not remove (rc=$rc) — Halt 6 if a handle holds it"; printf '%s\n' "$out"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: the measurement changed the worktree PATH SET"; exit 1; }
```

⚠️ The scan line is timed with `cargo audit` pending §8c.4's tool measurement. If that measurement picks
`cargo deny`, re-time with it. A warrant timed on a tool the verb does not run is a warrant for the wrong thing.

`s8g`'s D21-42 10c says *"A cold `cargo test --workspace` wall time is measured first"*. This is that measurement,
and `s8g` cites it rather than repeating it.

### 8c.6 A step's `timeout_secs` is the tighter bound (D21-42 4b)

Record B fixes this, and this is the proposal:

- **`catalog::Step` gains `timeout_secs: Option<StepTimeout>`**, which is either `Literal(u64)` or
  `Toolchain(key)`. The loader reads it, where today *"The loader reads no `timeout_secs`"* (ADR-0140's evidence).
  `inputs` is **not** parsed here. ADR-0148's forbids bullet on `inputs` stands until `s8d`.
- The kernel resolves `Toolchain(key)` through the `ToolchainValues` that `rk-cli` supplies (§8c.5). **An absent
  key is refused by name, and so is an `unmeasured` one**, as a `StepError::Unsupported` whose `capability` names
  `${toolchain.<key>}` and says why. **It never falls back to the construction bound.** A silent default is the
  failure ADR-0140 was written about.
- **Every `Toolchain` method a `tool` step reaches takes a `StepBound`**: `build`, `test`, `lint`,
  `change_summary` and `run_verb`.
  - `detect` does not take one: it is adapter selection at construction (ADR-0137 clause 2), not a step.
  - `classify` does not take one: it spawns nothing.
  - `StepBound` is either undeclared or `Secs(n)`, and `n = 0` is refused (RT-5: *"a zero bound is no bound"*).
- **The adapter enforces `min(step, construction)`** on every child it spawns for the call. `Timeout { after_secs }`
  carries **the bound that fired**, so a journal can tell a step's own bound from the construction bound.
  ADR-0140 decision 3's 3600 s default and its optional key stand.
- `ActivitySpec.timeout_secs` is **untouched** (ADR-0140 decision 5, and its forbids bullet against repurposing it).

🛑 **4b as the ledger writes it reaches 113 of the 298 declarations, not all of them.** D21-42 4b says *"the 298
declarations become real"*, and it says the bound is *"passed into the port call"*. The port there is the
`Toolchain` port. **185 declarations sit on `agent` steps.** The Harness port is not the Toolchain port, and a
per-step agent bound built by overloading `ActivitySpec.timeout_secs` is forbidden (ADR-0140 decision 5). The 33
substituted agent-step keys still get **values** here (§8c.5). Where they are **enforced** is not assigned. The
natural home is `s8d`'s per-step agent spec (D21-42 3a), and this file does not assign it (§8c.12).

**`rk-cli`'s two docstrings must be corrected in the same commit.** They say what ADR-0140 decision 2 made them
say: `config.rs` and `run.rs` describe the construction timeout as the only bound. Record B's supersession is what
licenses the change (§8c.7).

### 8c.7 The records this stage writes

Each is **a new record, at the next free id at writing**. Its status is `proposed` until the operator's dated
ledger line flips it. It carries ADR-0164's form, including `### Option — <name>` headings (D21-42 24a) and a
**What this forbids** block. **Moving a `## Decision` sentence or a forbids bullet is a supersession in part,
never an erratum** (ADR-0092 rule 1). An accepted body is never edited.

| label | executes | supersedes IN PART — the sentence, quoted | stands, and is said to stand |
| --- | --- | --- | --- |
| **T** — the outcome and the selection | D21-42 1a, 4c (adapter side); discharges RT-6, reverses RT-22, discharges R-9's transport and RT-26's OWED note against ADR-0094 decision 1 | **ADR-0162 decision 4**: *"`toolchain.test(selection)` is the boolean, because its contract is *"`Ok(count)` only on a green suite"* (RT-22) — so `is_err()` **is** "fails""*. **ADR-0094 decision 1**: *"`test` = `cargo test --workspace`"*, for an `Affected` run's `-p` set only | ADR-0162's four cells, the both-fail cell, the `--` selection rule and the ephemeral reference worktree; ADR-0094's whole-suite form; ADR-0148 whole (`s8d`'s) |
| **V** — the verb registry and the value table | D21-42 1b, 4a; fills D21-10's gap | **ADR-0135 clause 1**: *"The map is closed and holds exactly four entries"*. **Clause 6**: *"The 29 unmapped verbs are recorded as an open gap, owned by no stage in this phase"*, for the five rows landed. The `## Decision` sentence *"It adds no method to the `Toolchain` port: the count stays **six**, as Checkpoint 7a asserts"*. The forbids bullet *"Forbids adding a port method to satisfy a catalog verb"*, **for the one generic method only**. **ADR-0162's forbids bullet** *"Forbids a seventh method on the `Toolchain` port, and forbids applying a mutation patch from `rk-cli`"*: **its seventh-method half only**, likewise. The patch half stands here, and `s8g`'s verify record may move it for the verify gate's declared AC mutations alone | ADR-0135 clause 2 (the bare-name refusal), clause 4 (`change_summary` is never synthesised) and clause 5, and its bullet *"Forbids reading the adapters' `Capability` enum as a verb table, or extending it with catalog verbs"*: **the verb table is `registry.toml`, and the five-token `Capability` enum is untouched**. Checkpoint 7c digests its four byte-equal bodies |
| **B** — the step bound | D21-42 4b; discharges ADR-0140 decision 4's OWED item for **tool** steps | **ADR-0140 decision 1** in part, as the ledger names it. **Measured against the record's text, three further sites must move too, or they forbid what 4b builds. They are for tool steps only:** decision 2's *"It is not a backstop, because there is nothing behind it"*; decision 4's *"recorded as UNREAD"*; and the first two forbids bullets, *"Forbids describing the construction-time `timeout` as a *backstop*"* and *"Forbids any prose, docstring or record asserting that a step's declared `timeout_secs` is applied"*. **D21-42 4b lists decision 1 alone. That gap is reported, not resolved here** (§8c.12). The record states it, and the operator's flip line confirms it | ADR-0140 decision 3 (the 3600 s default), decision 5, and its bullets on lowering the default and on `ActivitySpec`. **ADR-0099 decision 2** (*"the two Toolchain adapters run bounded commands under the same timeout without a token"*): the no-token half stands. Record B **reads** *"under the same timeout"* against the per-call bound and states whether it moves. This file's reading is that it does |

**Errata and dated corrections, in the same work commit:**

- **ADR-0162's drivers**: *"Checkpoint 7a counts exactly six methods on the `Toolchain` port"*. This is measured
  false, and it is not a Decision sentence, so it takes an erratum at the next free `E-0162-<n>` (`E-0162-1` is
  taken). Its decision 4's *"a port Checkpoint 7a counts at six"* rides in record T's supersession of that
  decision. The mutate refusal stands on its remaining reason: *"no port method applies a patch"*.
- **The code sentences that repeat the false count**, corrected in place with the date:
  - `rk-ports/src/toolchain.rs` (*"Checkpoint 7a counts exactly six on this port"*);
  - both adapters' crate docs (*"the six methods Checkpoint 7a counts and no seventh"*) and their *"RT-6: an
    inherent method; the port stays six methods"* lines;
  - `review_pass.rs` and `tests/it/review_pass.rs`.
- **D21-10's** *"the count stays six, as Checkpoint 7a asserts"* takes a dated note under the operator's flip line.
  It does not take a rewrite.
- **`s7a` §7a.3**: a dated note that the `Toolchain` port has seven methods under record V, and that the exact
  count is asserted by `source_parse::toolchain_port_declares_exactly_seven_methods` and not by Checkpoint 7a's
  floor.

### 8c.8 Order of work

Each slice is red first (golden constraint 12). A test is authored and watched fail **for the right reason**
before the code that turns it green, and a filter that selects nothing is not red.

1. **§8c.1's measurement**, then the three records drafted `proposed`, with §8c.5's rule written into record V
   **before** §8c.5's readings.
2. **The port surface and every double, in one slice**: 1a, `Selection`, `StepBound`, `run_verb` and
   `validate_report`. A port change that leaves one double on the old signature does not compile, so this slice
   cannot be split honestly. Mark `PROGRESS.md` §1's Checkpoint 7a row (`rk-ports`) and Checkpoint 5 row
   (`rk-types`) **re-run pending** in this commit, per the maintenance rule. The named tests this stage moves in
   Checkpoints 7b, 7c and 7d's crates are re-proven by name in Checkpoint 8c (6)–(10).
3. **The Rust adapter**: the count source, the red suite, `Affected` over the workspace fixture, the bound, the
   recorded `affected-selection` probe, and the three verbs.
4. **The kernel**: `catalog::Step.timeout_secs`, `ToolchainValues`, `VerbRegistry` extended, `run_tool_step`
   through the registry, and the output decode table.
5. **`rk-cli`**: the value table handed over, `review_pass` reading `failed`, the stub program, and the
   docstrings.
6. **§8c.5's measurement** (n = 3) and `.redkiln/registry.toml` written with its warrants.
7. The mutants (§8c.9), then Checkpoint 8c whole, then the adversarial rounds, which continue **until a round moves
   no code and no guard**.

### 8c.9 The named tests and the mutants

Every name is module-qualified and run with `--test it … --exact`, which is Checkpoint 8c's form. **New** means
authored red first in this stage. **Re-proven** means an existing test that this stage's change reaches and must
still pass for the right reason.

- **`rk-types`**
  - **New**:
    - `verb_outputs::every_verb_output_refuses_unknown_fields_and_has_no_default`
    - `verb_outputs::scan_report_carries_unrated_as_a_required_member`
  - **Re-proven**: `schema_snapshot::schema_snapshot`, `schema_no_oneof::no_oneof_at_any_node`,
    `verify_types::basis_reason_is_the_closed_six`, `verify_types::verify_report_has_no_default`.
- **`rk-ports`** (`--features conformance`)
  - **New**:
    - `fake_behaviour::toolchain_fake_red_suite_is_ok_with_failed_counts`
    - `fake_behaviour::test_outcome_is_passing_only_with_executed_tests_and_no_failures`
    - `fake_behaviour::toolchain_fake_run_verb_refuses_an_undeclared_verb_by_name`
    - `fake_behaviour::step_bound_zero_is_refused`
    - `source_parse::toolchain_port_declares_exactly_seven_methods` — parses `pub trait Toolchain` from CODE lines
      and asserts exactly seven `fn`, naming each one
  - **Re-proven**: `conformance::port_conformance_fake_rows`, now with `validate_report`.
- **`rk-adapter-toolchain-rust`**
  - **New**:
    - `report::red_suite_is_ok_with_failed_counts`
    - `report::a_build_failure_is_err_never_a_red_suite`
    - `report::passed_plus_failed_is_the_executed_count_and_survives_both_decoys`
    - `bound::a_step_bound_tighter_than_construction_fires_first`
    - `bound::the_construction_bound_caps_a_larger_step_bound`
    - `affected::affected_set_is_touched_crates_plus_reverse_dependency_closure`
    - `affected::a_path_in_no_package_runs_the_whole_suite_with_a_reason`
    - `affected::no_recorded_whole_suite_runs_the_whole_suite_probe_unverified`
    - `affected::a_package_set_that_lists_no_test_escalates_selection_empty`
    - `affected::a_named_selection_the_adapter_does_not_declare_is_refused_by_name`
    - `verbs::smoke_runs_each_checklist_row_against_the_built_binary`
    - `verbs::a_failing_smoke_row_is_a_failing_result_not_an_error`
    - `verbs::sca_scan_counts_an_unrated_advisory_as_unrated_never_zero`
    - `verbs::sca_scan_records_the_advisory_database_commit`
    - `verbs::package_digests_only_the_declared_binaries_of_a_locked_release_build`
    - `verbs::an_undeclared_verb_is_refused_by_name`
  - **Re-proven**: `report::ac329_report_conforms_no_prose_parse`, `report::ac325_recorded_argv_is_os_vector`,
    `report::ac331_killed_verify_is_unverifiable`, `report::ac344_missing_binary_is_not_a_candidate_failure`,
    `conformance::port_conformance_real_row_rust`, `registry::ac328_affected_selection_unsupported_without_probe`
    and `registry::ac489_capability_registry_dated`.
  - 🛑 **`ac328_…` KEEPS ITS NAME.** Checkpoint 7c's loop names it for both toolchain crates. Its body is
    re-pointed at a controlled probe-less registry, which still refuses, and the shipped `Recorded` row is proved
    by the `affected::` tests.
- **`rk-adapter-toolchain-node`**
  - **New**:
    - `report::red_suite_is_ok_with_failed_counts`
    - `verbs::an_undeclared_verb_is_refused_by_name`
    - `bound::a_step_bound_tighter_than_construction_fires_first`
  - **Re-proven**: `report::ac329_report_conforms_no_prose_parse`, `report::ac331_killed_verify_is_unverifiable`,
    `report::ac344_missing_binary_is_not_a_candidate_failure`, `conformance::port_conformance_real_row_node`,
    `registry::ac328_affected_selection_unsupported_without_probe` and `cancel::s_cancel_win_vitest_arm`.
  - **Re-proven, corpus-bound (`--features corpus`)**: `ac334::ac334_flip_predicate_real_toolchain`.
- **`rk-kernel`**
  - **New**:
    - `tool_verb_routing::a_verb_with_no_registry_row_is_refused_by_its_bare_name`
    - `tool_verb_routing::a_registry_verb_with_no_inputs_reaches_run_verb`
    - `tool_verb_routing::a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port`
    - `tool_verb_routing::a_row_whose_route_has_no_executor_refuses_naming_the_route`
    - `tool_verb_routing::an_output_of_the_wrong_kind_is_refused_naming_both_kinds`
    - `verb_registry::the_shipped_registry_loads_with_route_outputs_and_live_on_every_row`
    - `verb_registry::a_value_without_a_warrant_is_refused`
    - `verb_registry::every_shipped_toolchain_substitution_resolves_or_refuses_by_name` — walks
      `.redkiln/catalog/` and **asserts the count it walked is non-zero and equals its own census**
    - `step_bound::a_literal_timeout_secs_reaches_the_port_call`
    - `step_bound::a_toolchain_substitution_resolves_through_the_value_table`
    - `step_bound::an_unresolved_toolchain_key_is_refused_by_name`
    - `step_bound::a_step_without_timeout_secs_passes_an_undeclared_bound`
  - **Re-proven**:
    `tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port` (ADR-0148 stands),
    `memo::ac018_live_verb_may_not_memoise` (the extended parser), `verify::ac332_empty_selection_is_never_green`
    and `kernel_denylist::ac487_kernel_denylist` (no adapter name enters the kernel with the value table).
- **`rk-cli`**
  - **New**:
    - `review_pass::a_candidate_that_cannot_run_is_unverifiable_never_a_flip`
    - `adapter_selection::the_chosen_adapters_value_table_reaches_the_kernel_without_its_name`
  - **Re-proven**: `review_pass::four_cells_over_two_arms_and_both_fail_is_not_a_flip`,
    `review_pass::mutate_and_selectionless_repros_produce_no_record`,
    `review_pass::selection_is_the_double_dash_tail_or_nothing`,
    `run::ac438_table_driven_exit_codes_over_fixtures_binary_arm` (the findings-block row still exits 4, through
    the updated stub) and `toolchain::a_tool_step_reaches_the_rust_adapter_and_not_the_fixture` (its `Step`
    literal gains `timeout_secs`).

**The mutants.** Each runs **from a Python driver, never the shell**. A backtick has already swallowed a restore in
this repository, and the next backup captured the mutant. The driver works in five steps:

1. **Anchor.** It asserts its anchor's exact hit count, terminator-agnostic (the tree is mixed per-file).
2. **Record.** It records the file's sha256.
3. **Apply and read.** It applies the mutant, then reads `git diff` before believing either colour.
4. **Run.** It runs the named test by its module-qualified `--exact` name and **requires both `rc ≠ 0` and that
   test's own `FAILED` line**.
5. **Restore.** It restores with `git checkout -- <file>` and re-checks the sha256.

The driver lives in the session scratchpad, whose path is visible to Windows Python. Git Bash's `/tmp` is not.
Parallel agents share one scratchpad, so the driver's filename carries this stage's id.

| id | mutant | must turn RED |
| --- | --- | --- |
| M1 | the Rust adapter maps a red suite back to `Err(NonZero)` (1a reverted) | `report::red_suite_is_ok_with_failed_counts` |
| M2 | `review_pass` reads `four_cell(candidate.is_err(), reference.is_err())` again | `review_pass::a_candidate_that_cannot_run_is_unverifiable_never_a_flip` AND `review_pass::four_cells_over_two_arms_and_both_fail_is_not_a_flip` |
| M3 | `status` is `Passing` whenever `failed == 0`, so an empty run reads as green | `fake_behaviour::test_outcome_is_passing_only_with_executed_tests_and_no_failures` |
| M4 | the adapter ignores `StepBound` | `bound::a_step_bound_tighter_than_construction_fires_first` (Rust) |
| M5 | `max` in place of `min` | `bound::the_construction_bound_caps_a_larger_step_bound` |
| M6 | the affected set omits the reverse-dependency closure | `affected::affected_set_is_touched_crates_plus_reverse_dependency_closure` |
| M7 | an unresolved `${toolchain.*}` key falls back to the construction bound | `step_bound::an_unresolved_toolchain_key_is_refused_by_name` |
| M8 | an unrated advisory is counted as `low` | `verbs::sca_scan_counts_an_unrated_advisory_as_unrated_never_zero` |
| M9 | the kernel skips the output-kind check | `tool_verb_routing::an_output_of_the_wrong_kind_is_refused_naming_both_kinds` |
| M10 | a wildcard arm routes an unknown verb to `run_verb` | `tool_verb_routing::a_verb_with_no_registry_row_is_refused_by_its_bare_name` |
| M11 | the per-test counts are read from stdout lines | `report::passed_plus_failed_is_the_executed_count_and_survives_both_decoys` (the decoys over-count) |

**If a mutant survives, read it before strengthening anything.** A surviving mutant can mean that the property is
unobservable from the seam the test watches. If so, the guard moves; it is not weakened. Record every result, kills
and survivors alike, in the checkpoint output.

### 8c.10 Acceptance set

**Claimed:** AC-018, AC-324, AC-325, AC-326, AC-328, AC-329, AC-331, AC-332, AC-344, AC-352, AC-487 and AC-499.
Each is claimed only for the clause named:

- **AC-018**: the shipped registry declares `live` on every row.
- **AC-326**: the adapter's bootstrap fallback only. The kernel's reclassification is not claimed.
- **AC-328**: Rust recorded, Node refused.
- **AC-329**: as a conformance row.
- **AC-332**: `Empty` is never passing, and the escalation.
- **AC-344**: the adapter half, and the review re-executor's cannot-run cell.
- **AC-352**: no seventh reason.
- **AC-487**: the value table carries no adapter name into the kernel.
- **AC-499**: both real rows and the fake row, re-proven.

**Excluded, each with its reason:**

| id | why it is excluded |
| --- | --- |
| AC-473, AC-521 | criterion (1) and (1b) runs, `s8` |
| AC-485 and the verify block (AC-320, 338, 339, 341, 348, 349, 353, 355, 359) | `s8g` (D21-42 10b, 10c) |
| AC-330 | the second half compares a recorded whole-suite wall to a declared timeout. §8c.5's measurement is that wall's first recording. `RK-D-TIMEOUT` reads the TOML directly and does not resolve a `${toolchain.*}` value, and wiring it is unassigned (§8c.12) |
| AC-326's `selection_ineffective` | the kernel's, and unmoved |
| AC-289's mutate half | ADR-0162's refusal stands |

### 8c.11 Checkpoint 8c

- **The port has exactly seven methods**, `test` returns an outcome, and a red suite is `Ok` with counts. Every
  double compiles, and the `impl … for` count in `crates/rk-ports/src/` is still 5.
- **Both real adapters and the fake pass `validate_report`** inside `conform_toolchain`.
- **A cannot-run candidate is `Unverifiable`, never a flip**, and the findings-block exit row still exits 4.
- **`.redkiln/registry.toml` ships.** Every row declares a route, `outputs` and `live`. Every `${toolchain.*}` in
  the catalog resolves or refuses by name. Every value's `pin` is a commit.
- **The step bound is `min(step, construction)`**, and an unresolved key is refused by name.
- **The three records exist and move what §8c.7 says.** ADR-0135's, ADR-0140's, ADR-0162's and ADR-0094's
  frontmatter carry the new `superseded_by`.
- **The whole suite is green**, fmt and clippy are rc 0, and **Checkpoint 8b and Checkpoint 7a re-run whole and
  pass**.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# s7i's conventions, unchanged. Every cargo reading is `out=$(…); rc=$?`, never a pipe's status: under build-lock
# contention cargo exits 0 having run ZERO tests. Every selection is `--test it`, MODULE-QUALIFIED and `--exact` —
# a bare name under `--exact` runs zero tests and exits 0 — and asserts each test's own `... ok` line AND an exact
# count. Every source grep is a FLOOR over CODE lines beneath a behavioural assertion, never the assertion, and
# every floor has a synthetic control proving it can go red.

# ---- (0) THE WORKTREE PATH SET, read FIRST, and a scratch dir Windows Python can see (not Git Bash's /tmp).
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
scratch="$(git rev-parse --git-common-dir)/redkiln/tmp/cp8c"
rm -rf "$scratch" && mkdir -p "$scratch" || { echo "FAIL: could not create $scratch"; exit 1; }

# ---- (1) BUILD-OR-FAIL (Halt 1). A stale target/ is how this programme reached two false conclusions.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8c NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (2) PREFLIGHTS. A fresh tree fails 10 Node tests at tests/it/support.rs' preflight without node_modules.
test -f crates/rk-adapter-toolchain-rust/tests/fixtures/project/Cargo.toml || { echo "FAIL: the Rust fixture project is absent"; exit 1; }
test -f crates/rk-adapter-toolchain-rust/tests/fixtures/workspace/Cargo.toml || { echo "FAIL: the affected-selection fixture workspace is absent — the closure tests would assert over nothing"; exit 1; }
test -d crates/rk-adapter-toolchain-rust/tests/fixtures/advisory-db || { echo "FAIL: the fixture advisory database is absent — the scan tests would assert over nothing"; exit 1; }
test -e crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules/.bin/vitest || { echo "FAIL: run 'pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project' ONCE (network; local-only)"; exit 1; }
PRED=target/predecessor-f21f55fc
test -d "$PRED/.git" || { echo "FAIL: the AC-334 predecessor checkout is absent at $PRED — s7c's preflight materialises it ONCE"; exit 1; }
d=$(git -C "$PRED" status --porcelain); [ -z "$d" ] || { echo "FAIL: $PRED is dirty — a previous ac334 run left the inversion applied"; exit 1; }

# ---- (3) THE THREE RECORDS, found by WHAT THEY SUPERSEDE — ids are not pre-assigned, so the frontmatter is the
# ---- key. Each is keyed by a PAIR where one exists: `s8a` also writes records superseding ADR-0135 and ADR-0094,
# ---- so a single-id count would be satisfied by a sibling stage's record. The finder prints ids, never paths
# ---- (Windows Python's glob joins with a backslash), and it is proved able to find nothing first.
sup='import re,sys,glob,os; want=set(sys.argv[1:]); ids=[os.path.basename(f)[:4] for f in sorted(glob.glob("references/adr/[0-9]*.md")) if (m:=re.search(r"(?m)^supersedes:\s*\[(.*?)\]",open(f,encoding="utf-8").read())) and want<=set(re.findall(r"\"(\d{4})\"",m.group(1)))]; print(" ".join(ids))'
ctl=$(python -c "$sup" 0001 9999); rc=$?
[ "$rc" -eq 0 ] && [ -z "$ctl" ] || { echo "FAIL: SYNTHETIC CONTROL — the supersedes finder named '$ctl' for a pair no record carries"; exit 1; }
T=$(python -c "$sup" 0162 0094); rc=$?
V=$(python -c "$sup" 0162 0135); rc2=$?
B=$(python -c "$sup" 0140); rc3=$?
[ "$rc" -eq 0 ] && [ "$rc2" -eq 0 ] && [ "$rc3" -eq 0 ] || { echo "FAIL: the supersedes finder did not run (rc=$rc/$rc2/$rc3)"; exit 1; }
for pair in "T:$T" "V:$V" "B:$B"; do
  id=${pair#*:}
  case "$id" in ''|*' '*) echo "FAIL: record ${pair%%:*} must be exactly ONE record, found '$id' (§8c.7: T supersedes 0162+0094, V 0162+0135, B 0140)"; exit 1 ;; esac
done
[ "$T" != "$V" ] || { echo "FAIL: records T and V are one file ($T) — §8c.7 writes them apart"; exit 1; }
for link in "0162:$T" "0094:$T" "0162:$V" "0135:$V" "0140:$B"; do
  old=${link%%:*}; id=${link#*:}
  grep -qE "^superseded_by:.*\"$id\"" references/adr/$old-*.md || { echo "FAIL: ADR-$old's superseded_by does not name $id"; exit 1; }
done
for id in $T $V $B; do
  f=$(ls references/adr/$id-*.md); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$f" ] || { echo "FAIL: no file for record $id"; exit 1; }
  grep -q '^### Option — ' "$f" || { echo "FAIL: $f has no '### Option — ' heading (D21-42 24a: from ADR-0164 on)"; exit 1; }
  grep -qF '**What this forbids**' "$f" || { echo "FAIL: $f carries no What-this-forbids block"; exit 1; }
  grep -qE '^status: (proposed|accepted)$' "$f" || { echo "FAIL: $f is neither proposed nor accepted"; exit 1; }
done

# ---- (4) EXACTLY SEVEN METHODS on `pub trait Toolchain`, parsed from CODE lines — Checkpoint 7a counts none
# ---- (it floors two NAMES), so this is the only count. Synthetic control first: six fns and a commented seventh.
m7='import re,sys; src=open(sys.argv[1],encoding="utf-8").read(); code="\n".join(l for l in src.splitlines() if not l.lstrip().startswith("//")); m=re.search(r"pub trait Toolchain\b[^{]*\{(.*?)\n\}",code,re.S); print(len(re.findall(r"(?m)^\s*fn\s+\w+",m.group(1))) if m else -1)'
printf 'pub trait Toolchain: Send + Sync {\n    fn a(&self);\n    fn b(&self);\n    fn c(&self);\n    // fn commented(&self);\n    fn d(&self);\n    fn e(&self);\n    fn f(&self);\n}\n' > "$scratch/six.rs" || { echo "FAIL: could not write the synthetic control"; exit 1; }
ctl=$(python -c "$m7" "$scratch/six.rs"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = "6" ] || { echo "FAIL: SYNTHETIC CONTROL — the counter read '$ctl' over six fns and a commented seventh; the guard below could not go red"; exit 1; }
got=$(python -c "$m7" crates/rk-ports/src/toolchain.rs); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = "7" ] || { echo "FAIL: pub trait Toolchain declares '$got' methods on code lines, expected exactly 7 (record V)"; exit 1; }

# ---- (5)–(10) THE NAMED SELECTIONS, one per crate. `want` is written beside each list so a shortened list is a
# ---- text change (and so a provisional checkpoint), never a silent narrowing.
run_named() {
  pkg=$1; feat=$2; want=$3; shift 3
  [ "$#" -eq "$want" ] || { echo "FAIL: $pkg: the list names $# tests, the block says $want"; exit 1; }
  out=$(cargo test -p "$pkg" $feat --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $pkg: the named selection rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $pkg: the selection did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
run_named rk-types "" 6 schema_snapshot::schema_snapshot schema_no_oneof::no_oneof_at_any_node \
  verify_types::basis_reason_is_the_closed_six verify_types::verify_report_has_no_default \
  verb_outputs::every_verb_output_refuses_unknown_fields_and_has_no_default verb_outputs::scan_report_carries_unrated_as_a_required_member
run_named rk-ports "--features conformance" 6 conformance::port_conformance_fake_rows \
  fake_behaviour::toolchain_fake_red_suite_is_ok_with_failed_counts fake_behaviour::test_outcome_is_passing_only_with_executed_tests_and_no_failures \
  fake_behaviour::toolchain_fake_run_verb_refuses_an_undeclared_verb_by_name fake_behaviour::step_bound_zero_is_refused \
  source_parse::toolchain_port_declares_exactly_seven_methods
run_named rk-adapter-toolchain-rust "" 23 report::red_suite_is_ok_with_failed_counts report::a_build_failure_is_err_never_a_red_suite \
  report::passed_plus_failed_is_the_executed_count_and_survives_both_decoys report::ac329_report_conforms_no_prose_parse \
  report::ac325_recorded_argv_is_os_vector report::ac331_killed_verify_is_unverifiable report::ac344_missing_binary_is_not_a_candidate_failure \
  conformance::port_conformance_real_row_rust bound::a_step_bound_tighter_than_construction_fires_first bound::the_construction_bound_caps_a_larger_step_bound \
  affected::affected_set_is_touched_crates_plus_reverse_dependency_closure affected::a_path_in_no_package_runs_the_whole_suite_with_a_reason \
  affected::no_recorded_whole_suite_runs_the_whole_suite_probe_unverified affected::a_package_set_that_lists_no_test_escalates_selection_empty \
  affected::a_named_selection_the_adapter_does_not_declare_is_refused_by_name registry::ac328_affected_selection_unsupported_without_probe \
  registry::ac489_capability_registry_dated verbs::smoke_runs_each_checklist_row_against_the_built_binary verbs::a_failing_smoke_row_is_a_failing_result_not_an_error \
  verbs::sca_scan_counts_an_unrated_advisory_as_unrated_never_zero verbs::sca_scan_records_the_advisory_database_commit \
  verbs::package_digests_only_the_declared_binaries_of_a_locked_release_build verbs::an_undeclared_verb_is_refused_by_name
run_named rk-adapter-toolchain-node "" 9 report::red_suite_is_ok_with_failed_counts report::ac329_report_conforms_no_prose_parse \
  report::ac331_killed_verify_is_unverifiable report::ac344_missing_binary_is_not_a_candidate_failure conformance::port_conformance_real_row_node \
  registry::ac328_affected_selection_unsupported_without_probe verbs::an_undeclared_verb_is_refused_by_name \
  bound::a_step_bound_tighter_than_construction_fires_first cancel::s_cancel_win_vitest_arm
run_named rk-adapter-toolchain-node "--features corpus" 1 ac334::ac334_flip_predicate_real_toolchain
run_named rk-kernel "" 16 tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port \
  tool_verb_routing::a_verb_with_no_registry_row_is_refused_by_its_bare_name tool_verb_routing::a_registry_verb_with_no_inputs_reaches_run_verb \
  tool_verb_routing::a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port tool_verb_routing::a_row_whose_route_has_no_executor_refuses_naming_the_route \
  tool_verb_routing::an_output_of_the_wrong_kind_is_refused_naming_both_kinds verb_registry::the_shipped_registry_loads_with_route_outputs_and_live_on_every_row \
  verb_registry::a_value_without_a_warrant_is_refused verb_registry::every_shipped_toolchain_substitution_resolves_or_refuses_by_name \
  step_bound::a_literal_timeout_secs_reaches_the_port_call step_bound::a_toolchain_substitution_resolves_through_the_value_table \
  step_bound::an_unresolved_toolchain_key_is_refused_by_name step_bound::a_step_without_timeout_secs_passes_an_undeclared_bound \
  memo::ac018_live_verb_may_not_memoise verify::ac332_empty_selection_is_never_green kernel_denylist::ac487_kernel_denylist
run_named rk-cli "" 7 review_pass::four_cells_over_two_arms_and_both_fail_is_not_a_flip review_pass::a_candidate_that_cannot_run_is_unverifiable_never_a_flip \
  review_pass::mutate_and_selectionless_repros_produce_no_record review_pass::selection_is_the_double_dash_tail_or_nothing \
  run::ac438_table_driven_exit_codes_over_fixtures_binary_arm toolchain::a_tool_step_reaches_the_rust_adapter_and_not_the_fixture \
  adapter_selection::the_chosen_adapters_value_table_reaches_the_kernel_without_its_name
# The findings-block row's fixture must still expect 4: a table-driven test over a changed `expected` asserts a
# different row. `[:space:]`, as s7h (10a) reads it, because the driver parses it with `.trim().parse()`.
exp=$(tr -d '[:space:]' < crates/rk-cli/tests/fixtures/exit/findings-block/expected)
[ "$exp" = "4" ] || { echo "FAIL: findings-block/expected reads '$exp', not 4 — the stub's red arm no longer reaches four_cell(true, false)"; exit 1; }

# ---- (11) THE SHIPPED REGISTRY AND ITS WARRANTS. The behavioural assertion is (9)'s verb_registry::…; these are
# ---- floors: the file ships, and every warrant's pin is a commit in this repository's object store.
test -f .redkiln/registry.toml || { echo "FAIL: .redkiln/registry.toml is not shipped — design §4.5's verb table is still D21-10's gap"; exit 1; }
pins=$(grep -oE '^[[:space:]]*pin[[:space:]]*=[[:space:]]*"[0-9a-f]{40}"' .redkiln/registry.toml | grep -oE '[0-9a-f]{40}')
[ -n "$pins" ] || { echo "FAIL: no value in .redkiln/registry.toml carries a 40-hex pin — a value without a measured warrant (D21-42 4a)"; exit 1; }
for p in $pins; do
  git cat-file -e "$p^{commit}" || { echo "FAIL: warrant pin $p is not a commit here — the reading cannot be re-taken at its pin"; exit 1; }
done

# ---- (12) CODE-LINE FLOORS, each with a synthetic control. Comment leaders dropped, anchored to grep's prefix.
cl='^([^:]+:)?[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'
printf '140:        let outcome = four_cell(candidate.is_err(), reference.is_err());\n' > "$scratch/ctl-reexec.txt" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(grep -vE "$cl" "$scratch/ctl-reexec.txt" | grep -E 'four_cell\([^)]*is_err\(\)')
[ -n "$ctl" ] || { echo "FAIL: SYNTHETIC CONTROL — the review_pass floor cannot see the M2 shape"; exit 1; }
raw=$(grep -nE 'four_cell\(' crates/rk-cli/src/review_pass.rs); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: no four_cell( call in review_pass.rs (grep rc=$rc) — the floor would read an empty set"; exit 1; }
bad=$(printf '%s\n' "$raw" | grep -vE "$cl" | grep -E 'four_cell\([^)]*is_err\(\)')
[ -z "$bad" ] || { echo "FAIL: review_pass reads is_err() as 'fails' again — a cannot-run candidate is a flip (AC-344):"; printf '%s\n' "$bad"; exit 1; }
raw=$(grep -rnE 'fn verify_report\(&self' crates/rk-adapter-toolchain-rust/src crates/rk-adapter-toolchain-node/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the RT-6 floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$cl" | grep .)
[ -z "$code" ] || { echo "FAIL: an inherent verify_report survives — two paths produce a report (RT-6 is discharged by record T):"; printf '%s\n' "$code"; exit 1; }
raw=$(grep -rnF 'test result:' crates/rk-adapter-toolchain-rust/src crates/rk-adapter-toolchain-node/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the AC-329 floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$cl" | grep .)
[ -z "$code" ] || { echo "FAIL: a toolchain adapter's source names libtest's summary line on a code line (AC-329):"; printf '%s\n' "$code"; exit 1; }
grep -rqE '"--list"' crates/rk-adapter-toolchain-rust/src --include=*.rs || { echo "FAIL: the Rust adapter no longer names \"--list\" — Checkpoint 7c's RT-26 line would go red"; exit 1; }

# ---- (13) THE WHOLE SUITE, --no-fail-fast, rc 0, no FAILED line, and a passed FLOOR: 740 at ca8f387 plus the 40
# ---- tests this stage names as new. A floor, not a pin: s8a and s8b add tests of their own.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .*FAILED|^test result:' | tail -30; exit 1; }
if printf '%s\n' "$out" | grep -qE '^test result: FAILED'; then echo "FAIL: a test binary reported FAILED under rc 0"; exit 1; fi
total=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' | awk '{s+=$1} END {print s+0}')
[ "$total" -ge 780 ] || { echo "FAIL: the whole suite passed $total tests, below the floor of 780"; exit 1; }
out=$(cargo test -p rk-ports --features conformance 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rk-ports under --features conformance rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (14) fmt and clippy, the conformance feature included (ci.yml carries the same step).
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }
out=$(cargo clippy -p rk-ports --all-targets --features conformance 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy over rk-ports --features conformance rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }

# ---- (15) THE ENTRY GATE AND CHECKPOINT 7a, RE-RUN WHOLE from their own files' bytes. The extractor refuses a
# ---- needle that selects zero or two blocks, and that refusal is proved first. No literal triple backtick
# ---- appears here: this file's own fence would close on it.
xb='import re,sys; f=chr(96)*3; t=open(sys.argv[1],encoding="utf-8").read(); bs=[b for b in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in b]; len(bs)==1 or sys.exit("need exactly one bash block holding "+repr(sys.argv[2])+" in "+sys.argv[1]+"; found "+str(len(bs))); open(sys.argv[3],"w",encoding="utf-8",newline="\n").write(re.sub(r"(?m)^> ?","",bs[0]))'
out=$(python -c "$xb" runbook/phase-1/s7a-ports-and-fakes.md 'NO-SUCH-NEEDLE-8C' "$scratch/none.sh" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: SYNTHETIC CONTROL — the extractor accepted a needle that selects no block"; exit 1; }
out=$(python -c "$xb" runbook/phase-1/s8b-catalog-quality.md 'CP8B-END-OF-BLOCK reached' "$scratch/cp8b.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8b's block could not be extracted: $out"; exit 1; }
out=$(bash "$scratch/cp8b.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the entry gate, Checkpoint 8b, re-run WHOLE, exited $rc (set any variable it refuses by name, and re-run)"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP8B-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8b exited 0 without reaching its last line"; exit 1; }
out=$(python -c "$xb" runbook/phase-1/s7a-ports-and-fakes.md 'trybuild_pins_the_reason_each_port_fixture_fails' "$scratch/cp7a.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a's block could not be extracted: $out"; exit 1; }
printf '\necho "CP7A-END-OF-BLOCK reached"\n' >> "$scratch/cp7a.sh" || { echo "FAIL: could not append 7a's end marker"; exit 1; }
out=$(bash "$scratch/cp7a.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a, re-run WHOLE over the seven-method port (D21-42 1b), exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP7A-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7a exited 0 without reaching its last line"; exit 1; }

# ---- (16) THE WORKTREE PATH SET IS UNCHANGED, compared BEFORE any prune; the prune is a dry run and finds nothing.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }
rm -rf "$scratch"

echo "CP8C-END-OF-BLOCK reached"
```

**BEFORE assertions (PIA-035), measured at `ca8f387` with this block's own commands.** (4)'s counter reads `6`
over `crates/rk-ports/src/toolchain.rs`. (12)'s `review_pass` floor finds the code line
`let outcome = four_cell(candidate.is_err(), reference.is_err());`. So both guards are RED on the untouched tree,
as a guard that can pass only after the work must be.

⚠️ **What this block cannot see, said rather than claimed away:**

- **(4)'s parser** drops `//` lines only. A method declared inside a `/* … */` block comment, or produced by a
  macro, is miscounted. The named test `source_parse::toolchain_port_declares_exactly_seven_methods` is what makes
  the pair sound.
- **(11)** proves that a pin is a commit. It does not prove that the reading was taken there. The warrant's
  `measured` list and record V's evidence are that proof, and the operator reads them.
- **(15)** runs Checkpoint 8b's bytes **as they stand at this stage's close**. If `s8b`'s block names a variable
  (as Checkpoint 7i named `FIX_ITEM`), it refuses by name, and the operator sets it. The block does not guess it.

### 8c.12 Stage close, and what is handed on

The stage closes **in two commits** (`00-preamble.md`): first the work and this checkpoint's output, including
§8c.1's and §8c.5's measurements and every mutant result; then the tracker commit that writes that hash into
`PROGRESS.md` §1. The three records stay `proposed` until the operator's dated ledger line. **Re-run Checkpoint 8c
after the flip commit.** A pass is evidence about a specific text.

**Handed on, unassigned — reported here, not decided here:**

1. **Where the 185 agent-step `timeout_secs` are enforced.** D21-42 4b's *"the 298 declarations become real"*
   reaches only the 113 on tool steps through the Toolchain port (§8c.6). The candidate home is `s8d`'s per-step
   agent spec.
2. **Record B's supersession reaches past the ledger's text.** D21-42 4b names ADR-0140 decision 1. Decision 2's
   sentence, decision 4, and the first two forbids bullets must move with it, or they forbid what 4b builds. The
   operator's flip line should say so.
3. **A run-owned worktree runs this repository's suite red.** It has no `node_modules`, so 10 Node tests fail
   (§8c.5). The first self-hosting verify would report failures unrelated to the change. The candidate owners are
   `s8g` and `s8j`.
4. **`build` has no outcome**, so `build-package`'s `report.status == 'passing'` check can never see a red build
   (§8c.4).
5. **`[toolchain.feeds]` is not shipped**, so `sca-scan`'s `toolchain:sbom?` input has no source for `s8d`'s
   resolver.
6. **`"e2e"` is refused.** If `s8j`'s story candidate selects `walking-skeleton`, the declaration becomes owed.
7. **`sca-scan`'s check reads only `critical` and `high`**, so it passes with `unrated > 0`. Whether that is
   right is an activity question, and so `s8g`'s.
8. **`RK-D-TIMEOUT` (AC-330)** reads TOML literals and resolves no `${toolchain.*}` value, and §8c.5's
   measurement is the first recorded whole-suite wall its second half compares against.
9. **AC-018's `live_verb_memo_rule` finally has a shipped registry to read, and nothing wires it over
   `.redkiln/`.** `s8b`, which runs the catalog checks over `.redkiln/` (D21-42 13a), predates the file.
10. **Two records claim the Rust verbs' behaviour.** This file's record V fixes the `smoke`, `sca_scan` and
    `package` handlers, which is why §8c.4 adopts `s8g` §8g.6's semantics. `s8g`'s producers record R4 says it
    *"fixes what the four verbs do on the Rust adapter, through `s8c`'s registry method"*. One of the two must
    cite the other rather than decide again. The proposal is that V fixes the handlers and R4 fixes the
    activities and the checklist.
11. **Sibling files that disagree with this one.** `s8d`'s verify row says of `smoke-test` *"its adapter row is
    `s8g`'s (6a)"*, but the `smoke` row and handler land here. `s8g`'s entry table checks *"the six-method
    count"*, but after record V the port has seven.

---
