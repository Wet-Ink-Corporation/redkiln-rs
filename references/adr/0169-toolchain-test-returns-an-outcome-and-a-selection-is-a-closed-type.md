---
id: "0169"
title: "**`Toolchain::test` returns an outcome, and a selection is a closed type.** A red suite is `Ok(TestOutcome { failed ≥ 1, status: Failing, … })`; `Err` means only cannot-run, killed or unreadable. `build` returns a `BuildReport` the same way. The per-test counts come from two strict line parsers pinned per version, libtest's (D21-43 N4) and cargo-nextest's (D21-48 X15, installed at `0.9.146`), and the Rust adapter runs nextest where the probe finds it, with a `cargo test --doc` companion because nextest runs no doctests. `\"affected\"` is the touched packages plus their reverse-dependency closure over `cargo metadata --format-version 1`. Supersedes, each in part and quoted, `0162` decision 4 (its reading that `is_err()` is a failing suite) and `0094` decision 1's *\"`test` = `cargo test --workspace`\"* in three respects; carries three of `D21-62`'s four provisional rulings (A, B and D)"
status: proposed
date: 2026-09-26
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 decisions 1a and 4c, D21-43 N4 and N25, D21-48 X15, D21-51 (the `cargo metadata` clause left to this record) and D21-62 (the four stage-open blockers, TAKEN PROVISIONALLY; panel text spikes/afk-panels/D21-62-s8c.md); runbook/phase-1/s8c-toolchain-port-and-verbs.md §8c.1–§8c.3 and §8c.7 (the proposal this record fixes); references/adr/0162 decision 4 and references/adr/0094 decision 1 (the sentences superseded in part); crates/rk-ports/src/toolchain.rs and crates/rk-adapter-toolchain-rust/src/report.rs at `6f70e47` (the signatures and `RUN_ARGS` this record moves); crates/rk-adapter-toolchain-rust/tests/fixtures/libtest/1.98.1/ and tests/fixtures/nextest/0.9.146/ (the goldens, with their `captures.json`)
supersedes: ["0162", "0094"]
superseded_by: []
related: ["0003", "0019", "0093", "0104", "0137", "0140", "0148", "0164", "0165", "0170", "0171"]
evidence: []
consumers: []
---

## Context and problem

Four facts, each re-read at `6f70e47`, make the port unable to carry what the catalog and the review gate read.

- **`test` throws the outcome away.** `crates/rk-ports/src/toolchain.rs` declares
  `fn test(&self, selection: &[String]) -> Result<u32, StepError>;`. The Rust adapter's module doc says it returns
  *"`Ok(selected_test_count)` on a green suite and `Err(NonZero { code, stderr_digest })` on a red one (RT-22)"*. A
  red suite is therefore not a result. Yet the shipped catalog reads the outcome the port cannot return: 34 activities
  read a tool output's `status`, `failed` or `passed`, the comparison `status == 'passing'` appears 25 times, and
  `tdd-red-green-refactor`'s `red-check` *expects* a red suite (D21-42's second false premise).
- **The review re-executor reads `is_err()` as "fails".** `crates/rk-cli/src/review_pass.rs` computes
  `four_cell(candidate.is_err(), reference.is_err())`, so a candidate that cannot start (`Err(Transport { Spawn })`)
  beside a green reference reads as a **flip**, and a flip is a block. AC-344 says the opposite: *"both-fail or
  cannot-run resolves to `Unverifiable { both_fail | cannot_run }` … never to a block"*. ADR-0162 decision 4 is the
  record that licenses the reading: *"`toolchain.test(selection)` is the boolean, because its contract is *"`Ok(count)`
  only on a green suite"* (RT-22) — so `is_err()` **is** "fails""*.
- **A selection is `&[String]`, and no declared selection string can pass through.** ADR-0148 measured why:
  *"`cargo test affected` matches nothing and **exits 0**"*.
- **`build` has no outcome.** `fn build(&self) -> Result<(), StepError>;`, so a red build is `Err`, and
  `build-package`'s `report.status == 'passing'` can never see one.

The pinned stable toolchain (`rustc 1.98.1`) has no structured per-test source: `libtest --format json` and
`--format junit` are nightly-only, and `RUSTC_BOOTSTRAP=1` is a workaround Halt 7 forbids (`s7c` §7c.14, measured
2026-09-17). D21-43 N4 took the question: *"A strict parser over libtest's `test <name> ... ok|FAILED` and `test
result:` lines, pinned by a golden fixture per toolchain version."* and *"Unparseable output is Unverifiable, never
green."* D21-48 X15 took the second runner: *"**Install it and support it now.** s8c installs cargo-nextest on this
box and in CI, parses nextest's output beside `cargo test`'s, and records on the report which runner ran. The install
is s8c's act, at stage open."*

At stage open four measurements contradicted the stage file's proposal, and D21-62 ruled on each provisionally under
the AFK rule of 2026-09-26. Three of the rulings are this record's (A, B and D); the fourth (C) is ADR-0170's.

## Decision drivers

- D21-42 1a is taken: *"A red suite is `Ok` with counts; `Err` is only cannot-run/timeout/killed (AC-331/AC-344)."*
  N25 extends it to `build`: *"**`build` is widened to an outcome report** as 1a did for `test`."*
- AC-329: *"no field of the report is obtained by parsing prose or markdown"*. The counts this record adds are not
  report members, and the lines they come from are a versioned line protocol, pinned by a golden, not prose.
- AC-332: *"'zero tests executed' is never rendered as green"*.
- A parser that trusts one version's output on another version is two instruments (golden constraint 8).
- One report path. Two paths that each produce a report (the port method and an inherent `verify_report`) can
  disagree, and nothing would say which one the lane read (RT-6).
- Every reversal below must be one clause of a proposed record and one code branch, because every D21-62 ruling is
  provisional until the operator confirms it.

## Considered options

### Option — Widen `test` and `build` to outcome reports, with a pinned line parser per runner (taken)

What D21-42 1a, N4, N25 and X15 take together, with the shapes §8c.1–§8c.3 proposed and D21-62 corrected. Taken.

### Option — Keep `test` green-only and add a second method for the counts

Rejected. It keeps the `is_err()`-as-fails reading alive at every caller that does not know the second method
exists, it is a second report path (RT-6), and it spends a port method D21-42 1b allots to `run_verb`.

### Option — Read the counts from libtest's summary line alone

Rejected. The summary line is the prose AC-329 bans unless its grammar is pinned, and a summary alone cannot be
checked against anything: s7c's two decoys print a summary on stdout and on stderr. The records inside a block are
what the summary must agree with.

### Option — Keep §8c.1 rule 2 strict and give `rk_cli::verbs::doctor::run` a writer (D21-62 A, dissent)

Not taken, and left open. It moves a crate this record does not otherwise touch, re-captures the self-hosting golden,
and leaves every future in-process verb test able to make every libtest self-hosting run Unverifiable. The panel's
reversibility judge preferred it; the writer seam is compatible with the ruling taken, and is handed on as a hygiene
item (§8c.12).

### Option — On nextest's list failure, fall back to `cargo test` under `Detect` (D21-62 B option 2)

Not taken, and left open. It adds a run-time runner switch that §8c.1's selection rule does not have, and it is the
hardest of the three to remove. It can be layered on this record later, because the `ListFailed` refusal stays its
trigger.

### Option — A fixture `.config/nextest.toml` default filter over `zz_decoy` (D21-62 B option 3)

Not taken, and left open. It changes what runs with no trace in the recorded argv (AC-325), and whether a
default-filter skips listing as the command-line `-E` does was never measured.

### Option — Map `tsc`'s `1` and `2` both to `Ok(failing)` (the §8c.2 proposal)

Rejected by measurement (D21-62 D). On the pinned `tsc 5.9.3`, `1` means a command-line or project error: nothing was
checked. A status that checked nothing is cannot-run under 1a's split, not a red build.

## Decision

**1. `test` returns a `TestOutcome`.** The signature is
`fn test(&self, selection: &Selection, bound: StepBound) -> Result<TestOutcome, StepError>` (`StepBound` is ADR-0171's).
`TestOutcome` is an `rk-ports` type that wraps the unchanged `rk_types::verify::VerifyReport` and adds four fields:

- `passed: u32` and `failed: u32`;
- `status: TestStatus`, **derived and never stored independently**: `Passing` iff `failed == 0 && passed > 0`,
  `Failing` iff `failed > 0`, `Empty` iff `passed + failed == 0`. Its serde form is `passing` / `failing` / `empty`,
  the vocabulary the 25 catalog comparisons already use, so an empty run never satisfies a check (AC-332);
- `runner: TestRunner`, holding `name` (`cargo-test`, `cargo-nextest`, `vitest`, or `fixture` for the fake),
  `version`, `companion_argv` (decision 5) and `foreign_lines` (decision 3). `validate_report` refuses an empty
  `name`.

`VerifyReport` gains **no member**: AC-329's list is closed, the store's `validate_report` (ADR-0104 decision 3) reads
it, and `deny_unknown_fields` guards it. **The invariant** `passed + failed == report.selected_test_count` is checked
at construction. A breach is `Err(Transport { Decode })` naming both numbers, never a corrected count.

**`Ok` means the suite ran and every executed test reported, green or red.** **`Err` means exactly three things:**
(a) cannot run (`Transport { Spawn }`, AC-344, or a build that fails before any test executes: `NonZero` with no
per-test record); (b) killed (`Timeout { after_secs }`, or an OS kill with the `os-killed:` prefix, AC-331);
(c) unreadable (`Transport { Io | Decode }`, including every parser refusal below). **A non-zero exit whose records
name the failures is a red suite, not an error.**

**2. The readers move with the type.** RT-22's routing is **reversed**. RT-6 is **discharged**: the inherent
`verify_report(&self, …)` methods on `RustToolchain` and `NodeToolchain` are retired, and the module functions
`report::verify_report` stay as the port method's body, so exactly one path produces a report. R-9 is discharged for
the transport, because the report now crosses the port. The review re-executor reads
`four_cell(candidate.failed > 0, reference.failed > 0)` when both arms are `Ok`, and **returns `None` when either arm
is `Err`**, which `confirm_with` reads as `Unverifiable { cannot_run }` (AC-344). A red candidate's `observed` string
names its counts, so it can never equal `expected_when_met`.

**3. The libtest parser (D21-43 N4).** `crates/rk-adapter-toolchain-rust/src/libtest_lines.rs` holds one pure
function, `parse(stdout, exit_code, version) -> Result<Tally, ParseRefusal>`, over the run child's stdout **file**. It
never reads stderr, so s7c's `build.rs` decoy cannot reach it.

1. **Pinned per version.** `PINNED` lists the `rustc -V` releases that have a golden: `1.98.1`, the
   `rust-toolchain.toml` channel. An unlisted version is refused by name
   (`Unsupported { capability: "libtest-lines@<version>" }`). A pin bump adds a golden and a list entry in the PR that
   moves the pin.
2. **Block-framed.** A block opens on `^running (\d+) tests?$`. Its record region runs to the first empty line.
   A record is `^test (.+) \.\.\. (ok|FAILED|ignored(, .*)?)$`, and the region must hold exactly as many records as
   the header states. libtest's slow notice, `^test (.+) has been running for over 60 seconds$`, is skipped, and the
   name it carries must be one the block records, or the block refuses. *(D21-62 A, provisional.)* **Any other
   non-empty line in the region is FOREIGN: it is skipped, never counted, and tallied.** The block closes on
   `^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in [0-9.]+s$`.
   Lines between the region and the summary (the `failures:` section, captured output) are skipped. **A
   record-grammar line outside a block is never counted**, which is how the stdout decoy is survived.
3. **Self-consistent.** Each summary's `passed`, `failed` and `ignored` must equal that block's tallied records. A
   header without a summary, or a summary without a header, refuses.
4. **Multi-binary.** The tally is the sum over blocks. Doctests are blocks of the same grammar; their names contain
   spaces (and, on Windows, backslashes), which `(.+)` admits.
5. **Against `--list`.** `passed + failed` equals the `--list`-derived `selected_test_count` (decision 1's invariant),
   on the green fixture and on the red one.
6. **Exit-code agreement.** A non-zero exit whose blocks report `failed == 0` refuses as `UnattributedFailure` (a
   `harness = false` target is the usual cause), never read as green. A zero exit with `failed > 0` refuses too.

`Tally` carries `passed`, `failed`, `ignored`, `blocks`, and *(D21-62 A)* `foreign_lines: u32` with the first foreign
line's 1-based number and text as evidence. The adapter carries the count onto `TestOutcome.runner`. It is a count
from the parser, not a parsed prose value. **Every `ParseRefusal` maps to `Err(Transport { Decode })`**, naming the
refusal and the version.

**This record's reading of N4, flagged for the operator's flip (D21-62 A).** *"Unparseable output is Unverifiable,
never green"* binds the protocol lines the tally is taken from: headers, records, slow notices and summaries. Each must
parse and agree, or the block is Unverifiable. A foreign line contributes to no count. The fail-closed guarantee is
carried by the header count, rule 3, rule 5 and rule 6: a leaked record-shaped line over-counts against the header; a
corrupted record under-counts; a corrupted `FAILED` record is caught by rule 3 and by rule 6; a leaked empty line
closes the region early and refuses. The one balanced case, a leaked `ok` record beside a corrupted `ok` record,
yields the true tally. **This is a deliberate interpretation of N4, not a silent one**, and the operator may reject it.
The self-hosting golden is why it was needed: `this-repository.stdout` carries 39 `RK-D-<rule> ok` lines inside the
rk-cli `it` binary's record region (the first at line 287, *"RK-D-GATE-OUTPUTS ok"*), leaked by
`rk_cli::verbs::doctor::run` writing through `std::io::stdout()` in-process. §8c.1 rule 2 as proposed (*"Any other
non-empty line in the region refuses."*) refused it; with those 39 skipped the tally is 784 passed, 0 failed, 9
ignored over 44 blocks, Checkpoint 8b's reading.

**4. Runner selection, and the nextest parser (D21-48 X15).** ADR-0094 decision 1 admits *"`cargo nextest run` where
present, recorded on the report"*, and this record executes that clause. It does not supersede it.

- **The pin.** `cargo install cargo-nextest --locked` ran at stage open. The installed line is
  `cargo-nextest 0.9.146 (8af696ddc 2026-09-21)`, and `0.9.146` is **the pin**. `.github/workflows/ci.yml`'s `gates`
  job installs the same version, `cargo install cargo-nextest --version 0.9.146 --locked`, in a step immediately
  before its `test` step. A bump moves the pin, the CI line and a new golden in one PR.
- **Selection.** At construction the Rust adapter probes `<program> nextest --version` under the `Config`'s
  `child_env`. It runs nextest when the probe exits 0 with a `cargo-nextest <version>` line, and `cargo test`
  otherwise. `Config` gains a `runner` policy: `Detect` (what `real_toolchain` uses) or `Force(runner)` (a test that
  pins one path's vector). The runner that ran is recorded twice: `VerifyReport.argv` is the run vector as handed to
  the OS (AC-325), so it names `nextest` exactly when nextest ran, and `TestOutcome.runner` names it with its version.
  **Measured, the absent case needs no stub:** the toolchain's own `cargo` under a `child_env` with no
  `cargo-nextest` on `PATH` and an empty `CARGO_HOME` answers `cargo nextest --version` with rc 101, empty stdout
  and *"error: no such command: `nextest`"* on stderr, so `runner::the_probe_falls_back_to_cargo_test_where_nextest_is_absent`
  runs the real `cargo`.
- **The run child.** `cargo nextest run` with `--no-fail-fast --retries 0 --color never --status-level pass
  --final-status-level none`, so each executed test prints exactly one status line. The status lines are on
  **stderr** (stdout is empty without `--nocapture`), and cargo's build output and `build.rs`'s `cargo:warning=`
  lines are on the same stream, so the parser reads stderr and must ignore every line that is not in its grammar.
- **The parser.** `crates/rk-adapter-toolchain-rust/src/nextest_lines.rs` holds one pure function,
  `parse(stream, exit_code, version) -> Result<Tally, ParseRefusal>`, returning the same `Tally`. It **stays strict**:
  D21-62 A's foreign-line skip is the libtest path's only. Its grammar, measured on `0.9.146` (leading spaces
  right-align a 12-column status field):
  - `START` `^\s+Starting (\d+) tests? across (\d+) binar(?:y|ies)(?: \((.+) skipped\))?$`
  - `STATUS` `^\s+(PASS|FAIL) \[\s*[0-9.]+s\] \(\s*(\d+)/(\d+)\) (\S+) (.+)$` (the binary id, then the test name; the
    counter is left-padded to the width of the total)
  - `SLOW` `^\s+SLOW \[>\s*[0-9.]+s\] \(\S+\) (\S+) (.+)$`, skipped, and it must name a test the run records
  - `REFUSE` `^\s+(?:TRY \d+ \S+|RETRY|FLAKY|TIMEOUT|TERMINATING|ABORT|SIGSEGV|SIGABRT|SIGKILL|SIGTERM|LEAK)\b`
  - `SUMMARY` `^\s+Summary \[\s*[0-9.]+s\] (\d+) tests? run: (\d+) passed(?: \((\d+) slow\))?(?:, (\d+) failed)?, (\d+) skipped$`

  Its rules are N4's, carried over: `NEXTEST_PINNED` lists `0.9.146`, and an unlisted version is refused by name
  (`Unsupported { capability: "nextest-lines@<version>" }`); the `Starting` count must equal the `PASS`/`FAIL` records,
  and `Summary`'s run, passed and failed must equal the tally; **a `REFUSE` line refuses**, because a retried test is
  two runs of one test and a timed-out one is a kill, not a failure (AC-331); exit-code agreement is N4's rule 6
  (exit 0 green, 100 test failure).
- *(D21-62 B, provisional.)* **A list failure is a named refusal.** Over a project whose `harness = false` target does
  not speak `--list` (s7c's `zz_decoy`), `0.9.146` exits **104** with the line *"error: creating test list failed"*
  and no status line. That is `ParseRefusal::ListFailed { version, exit: 104 }`, mapped to `Err(Transport { Decode })`
  naming `ListFailed` and `0.9.146`. More generally, **any exit other than 0 or 100 with zero `PASS`/`FAIL` lines
  refuses and is never tallied**. Under `Detect` and under `Force(nextest)` alike nothing falls back. The known
  limitation is stated: on a box where nextest is installed, `Detect` over a repository with a non-listing
  `harness = false` target is Unverifiable, although `cargo test` could run it. That fails closed; it is never a
  false green. Options 2 and 3 above stay open for the operator.
- **Selection arguments** (measured on `0.9.146`, passed after `--`): `--exact`, `--ignored`, `--include-ignored`,
  `--skip <name>` and a bare name filter (`review_pass::selection_of`'s shape) are all accepted with libtest's
  meaning. An argument the installed nextest does not accept refuses by name; it is never dropped. An empty selection
  exits **4** (*"error: no tests to run"*), which the general rule above refuses.

**5. The doctest companion.** `cargo nextest run` runs no doctests. On the nextest path the adapter runs a second
child, `cargo test --workspace --doc --no-fail-fast`, under the same bound. An `Affected` run's `-p` set replaces
`--workspace`, and a `Filter`'s `-- <args>` follows. The child is parsed with `libtest_lines::parse`; the outcome sums
both tallies, `wall_secs` sums both children, and `TestOutcome.runner.companion_argv` records its vector. When the
selected packages have no library target (read from `cargo metadata`), no companion runs and `runner` says so.
**Decision 1's invariant keeps the set whole**: `--list` lists the doctests, so a nextest run whose doctests were
dropped refuses as `Transport { Decode }` rather than reading as a smaller green suite. Measured over this repository:
nextest ran 781 tests (9 skipped) and the companion 3, and 781 + 3 = 784, the libtest whole-suite count.

**6. `--no-fail-fast` on every run child.** Without it cargo stops at the first failing binary, a red suite whose
failure sits in an early binary prints fewer blocks than `--list` counts, and every red suite on a 44-binary
repository reads as cannot-run. `report.rs`'s `RUN_ARGS` (`["test", "--workspace"]` at `6f70e47`) gains the flag, and
the nextest child carries it too.

**7. A selection is a closed type.** `test`'s `&[String]` is replaced by `rk_ports::toolchain::Selection`:

| variant | meaning | Rust adapter | Node adapter |
| --- | --- | --- | --- |
| `WholeSuite { reason: BasisReason }` | the whole suite; the reason travels with the call | `--workspace`; `basis: WholeSuite`, `basis_reason: Some(reason)` | whole suite, same |
| `Filter(Vec<String>)` | runner arguments, what `review_pass::selection_of` produces from a repro's `--` tail (ADR-0162) | `-- <args>` | `-t` / file |
| `Affected { changed: Vec<String>, recorded: Option<RecordedWholeSuite> }` | the change set, and the most recent recorded whole-suite count (`run_id`, `toolchain_version`, `count`) the caller read from the store | the rules below | refused by name, `Unsupported { capability: "affected-selection" }`, from its `Probe::None` row (AC-328) |
| `Named(String)` | a selection the adapter declares | an undeclared name is refused by name | same |

**The Rust `Affected` rules, in order.** The reasons come from AC-352's closed six, and **no seventh is minted**.

1. The `affected-selection` registry row must carry `Probe::Recorded`, or the call is refused from the row (AC-328).
   The row records a dated `Recorded { source }` naming the fixture workspace and the test that exercises it. **Both
   facts are stated:** the adapter produces `basis: Affected` from a `Recorded` row, and nothing compares that row's
   `verified_on` staleness (the OWED bridge the registry's own doc names). AC-328's absent-probe half holds; its
   *"or stale"* half is not claimed.
2. An **empty** change set is refused by name. It is a caller error, not a green and not a whole suite.
3. **This record's own `cargo metadata` read (D21-51).** The adapter reads `cargo metadata --format-version 1`, a
   structured stable read, and maps every changed path to the workspace package whose manifest directory is its
   longest prefix. After ADR-0165 decision 9, ADR-0094 decision 1 names the `cargo metadata` read for the adapter but
   no longer for `classify` or the `Text` arm, so this rule rests on this record, not on that clause. **A path in no
   package** (the root `Cargo.toml`, `Cargo.lock`, `rust-toolchain*`, `.cargo/`, or any non-crate path) forces the
   whole suite with `PolicyForced`. **This record's reading of `PolicyForced`:** its doc in `rk-types` reads *"The
   pack's policy demanded the whole suite regardless"*, which names the PACK's policy. This record reads the reason as
   *a declared policy demanded the whole suite regardless*, and declares this rule as the Rust adapter's own policy,
   because of the six reasons it is the only one whose meaning is "a rule, not a probe or a count, forced the whole
   suite". The doc takes a dated note with this record in the commit that lands the rule. The reading is flagged for
   the operator's flip; rejecting it means minting nothing (AC-352) and choosing another of the six by record.
4. The affected set is the touched packages plus their **reverse-dependency closure** over `resolve.nodes`,
   **dev-dependencies included**. It becomes `-p <pkg>` flags in place of `--workspace`.
5. If `recorded` is absent, or its `toolchain_version` differs from this run's `rustc -V`, the adapter runs the whole
   suite with `ProbeUnverified` (its doc: *"AC-326's bootstrap: no whole-suite count recorded yet"*).
6. If the `-p` set lists zero tests (`--list`), the adapter escalates to the whole suite with `SelectionEmpty`
   (AC-332's declared policy is escalate).
7. Otherwise it runs the `-p` set with `basis: Affected`, `basis_reason: None`, `total_test_count = recorded.count`
   and `total_test_count_source: RecordedRun { run_id }`. AC-326's `selection_ineffective` reclassification stays the
   kernel's.

The affected set is the pure function `affected_packages(changed, &metadata, policy_version) -> AffectedSet` over
metadata read before it is called (AC-347). **`"e2e"` is refused by name on both adapters**: no record says what it
means for a Cargo workspace, and D21-42 4c allows *"declared per adapter or refused"*.

**8. `build` returns a `BuildReport` (D21-43 N25).** The signature is
`fn build(&self, bound: StepBound) -> Result<BuildReport, StepError>`. `BuildReport` is an `rk-types` struct carrying
`argv`, `toolchain_version`, `wall_secs`, `errors: u32` and `status` (`passing` | `failing`).

- **Rust.** `cargo build --workspace --message-format=json`. `status` is the `build-finished` message's `success`,
  and `errors` counts `compiler-message` records at level `error`, so a compile error is
  `Ok(BuildReport { status: failing, errors ≥ 1 })`. A non-zero exit with no `build-finished` message, or a stream
  that disagrees with its exit code, is `Err(Transport { Decode })`. ADR-0094 decision 1's *"`build` = `cargo build
  --workspace`"* **stands**: the flag changes the stream that is read, not what is built.
- *(D21-62 D, provisional.)* **Node**, `<program> tsc --noEmit` with `program` the adapter's `DEFAULT_PROGRAM`
  (`npx.cmd` on Windows, `npx` elsewhere), on the pinned `tsc 5.9.3`: exit `0` is `Ok(passing)`, `errors = 0`; exit
  `2` is `Ok(failing)`, `errors` = the count of `error TS<n>` lines with a floor of 1; exit `1` and any other non-zero
  status are `Err(NonZero)`, cannot-run. **A broken `tsconfig.json` reads as a failing build, never a passing one**:
  `tsc` exits 2 for a tsconfig that is not JSON, an unknown tsconfig option and TS18003 *"No inputs were found"*, and
  this record does not reclassify them by parsing diagnostic codes. That is a known flaw, stated. Exit 1 is a command-line
  or project error (an unknown option, `-p` naming a missing project, a directory with no `tsconfig.json`), where
  nothing was checked. The mapping holds for the `npx` or direct launcher only: DECISIONS-2026-09-09 #34's
  no-pnpm-forwarding ruling is extended from `test` to `build` by citation, and adds no code gate, because through
  pnpm every failure collapses to 1 and fails closed to `Err`.

**9. The conformance row gains `validate_report`.** `rk_ports::conformance::validate_report(&TestOutcome)` runs inside
`conform_toolchain` for the fake row and both real rows. It checks `VerifyReport::validate_shape()`, the count
invariant, and that `status` agrees with the counts. It does not duplicate the store's whole-suite rule, which stays
`rk_store::artifacts::verify_report::validate_report` (ADR-0104 decision 3). AC-329's *Verified by* already reads
*"schema conformance test over every adapter's fixture reports"*, and this row is that instrument.

**10. What this record supersedes, each in part (ADR-0092 rule 1).**

- **ADR-0162 decision 4**, the sentence *"`toolchain.test(selection)` is the boolean, because its contract is
  *"`Ok(count)` only on a green suite"* (RT-22) — so `is_err()` **is** "fails""*. Decision 2 above replaces it. The
  same decision's *"a port Checkpoint 7a counts at six"* rides in this supersession (Checkpoint 7a counts no method;
  E-0162-2). **What stands of ADR-0162:** its four cells, the both-fail cell, the `--` selection rule, the refusal of
  `kind = mutate` on its remaining reason (*"no port method applies a patch"*), and the ephemeral reference worktree.
- **ADR-0094 decision 1**, *"`test` = `cargo test --workspace`"*, **in three respects only**: (i) an `Affected` run
  passes a `-p` set in place of `--workspace` (decision 7); (ii) every run child passes `--no-fail-fast` (decision 6);
  (iii) where nextest runs, a doctest companion child runs beside `cargo nextest run` (decision 5). **What stands of
  ADR-0094 decision 1:** *"(or `cargo nextest run` where present, recorded on the report)"*, which this record
  executes; the whole-suite package set; *"`build` = `cargo build --workspace`"*; and every clause ADR-0165 decision 9
  and ADR-0167 left standing. **ADR-0148 stands whole** (`toolchain:test` refuses in the executor until `s8d`'s
  resolver).

**11. What this record discharges.** RT-6 (decision 2); R-9's transport (decision 2); and RT-26's OWED note against
ADR-0094 decision 1 (`s7c` §7c.22 item (22), *"the Rust adapter's `--list` count route as a dated note against ADR-0094
decision 1 (or a successor record)"*): this record is that successor. `--list` stays the source of
`selected_test_count`, and the per-test results come from the two pinned parsers. RT-22 is reversed (decision 2).

## Consequences

`Ok` stops meaning green, and every reader of `test` or `build` moves with the slice that lands this record. The port
still has six methods under this record; ADR-0170 adds the seventh. The review re-executor stops reading a cannot-run
candidate as a block. The self-hosting run's counts are readable on both runners, and the report says which ran.

**Residuals, named.** An empty selection differs by runner: the libtest path returns `Ok` with `status: Empty`, and
the nextest path refuses (exit 4). Both are never green (AC-332); parity is not decided here. The nextest grammar is
measured on one version; the goldens prove the parser against it only. The POSIX `npx` pass-through of `tsc`'s exit
status (Linux CI) is unmeasured. A foreign libtest line is visible only as a count (decision 3).

**What this forbids**

- Reading `is_ok()`, `is_err()` or `?` on `Toolchain::test` or `Toolchain::build` as a verdict. A verdict is `failed`
  or `status`.
- Obtaining a per-test count from any line outside `libtest_lines::parse` or `nextest_lines::parse`, or naming
  libtest's summary line on a code line outside the libtest parser.
- Parsing a runner version absent from `PINNED` or `NEXTEST_PINNED` on trust.
- Counting a foreign libtest line, or a nextest `TRY`, `RETRY`, `FLAKY`, `TIMEOUT`, `TERMINATING`, `ABORT`, signal or
  `LEAK` line, as a record.
- Falling back from nextest to `cargo test` on a list failure, or on any refusal, without a record naming the fallback
  and recording it on `TestOutcome.runner`.
- Dropping a selection argument the installed nextest does not accept.
- A run child without `--no-fail-fast`, or a nextest run without its doctest companion where the selected packages
  have a library target.
- A second report path: an inherent `verify_report` on either adapter, or a report produced beside the port method.
- Adding a member to `VerifyReport` to carry the outcome, or storing `status` independently of the counts.
- Minting a seventh `BasisReason`, or reading an empty change set as a whole suite or as green.
- Reclassifying `tsc`'s exit 1 as a red build, or routing `build` through a pnpm-forwarded `tsc`.

## Evidence and confidence

Measured 2026-09-26 at stage open (package P1), in the main checkout at `e771730` plus the stage's uncommitted fixture
edits (no crate source changed), every cargo, pnpm and `tsc` run through `runbook/tools/nodispatch.sh`. The raw
streams sat in the session scratchpad, which is not durable, so the load-bearing lines are quoted here.

- **Versions.** `rustc -V` → `rustc 1.98.1 (48a229cea 2026-09-01)`; `cargo -V` → `cargo 1.98.1 (797e8a9bc
  2026-08-05)`; `cargo nextest --version` → **`cargo-nextest 0.9.146 (8af696ddc 2026-09-21)`** (release `0.9.146`,
  commit `8af696ddcce8fff2962d6a5168b6d138b8616a35`, host `x86_64-pc-windows-msvc`), installed by
  `cargo install cargo-nextest --locked` (rc 0, *"Installed package `cargo-nextest v0.9.146`"*); `tsc` → `Version
  5.9.3` (the Node fixture's local bin); pnpm 10.33.0.
- **The libtest goldens** (`crates/rk-adapter-toolchain-rust/tests/fixtures/libtest/1.98.1/`, argv and exit codes in
  its `captures.json`, cwd `D:/repos/redkiln-rs`, each fixture under its own `CARGO_TARGET_DIR`):
  `project-green` (`cargo test --manifest-path …/fixtures/project/Cargo.toml --workspace --no-fail-fast`, rc 0, 2
  passed / 0 failed / 2 ignored over 2 blocks); `project-red` (the same with `-- --ignored red_by_selection`, rc 101,
  0 / 1 / 0); `workspace-green` (`…/fixtures/workspace/Cargo.toml --workspace --no-fail-fast`, rc 0, 4 / 0 / 1 over 6
  blocks); `workspace-red-in-a` (the same with `-- --include-ignored`, rc 101, 4 / 1 / 0 over 6 blocks, the failure in
  `a`'s unit-test binary, the first block); `doctest-a` (`-p a --doc --no-fail-fast`, rc 0, 1 / 0 / 0);
  `this-repository` (`cargo test --workspace --no-fail-fast` at the repository root, rc 0, **784 / 0 / 9 over 44
  blocks**, 170.5 s wall, 39 foreign lines, the first at line 287). All captured LF (0 CR bytes). No "has been running
  for over 60 seconds" notice occurs in the self-hosting golden.
- **The nextest goldens** (`…/fixtures/nextest/0.9.146/`, argv in its `captures.json`):
  `workspace-red-in-a.stderr` (`cargo nextest run --manifest-path …/fixtures/workspace/Cargo.toml --workspace
  --no-fail-fast --retries 0 --color never --status-level pass --final-status-level none -- --include-ignored`, rc
  100, `Starting 4`, 3 passed, 1 failed) and its companion `workspace-red-in-a.doc.stdout` (`cargo test
  --manifest-path …/workspace/Cargo.toml --workspace --doc --no-fail-fast -- --include-ignored`, rc 0, 1 passed over
  2 blocks): 4 passed, 1 failed in total, the libtest golden's 4 / 1. *(D21-62 B.)* Owed beside them, from P1's raw
  captures: `project-green.list-failed.stderr` and `project-red.list-failed.stderr` (the unfiltered vector over the
  fixture project, rc 104, *"error: creating test list failed"* then *"for `rk-toolchain-rust-fixture::zz_decoy`, line
  "test result: ok. 99 passed; 0 failed" did not end with the string ": test" or ": benchmark""*), and
  `project-green.filtered.stderr` / `project-red.filtered.stderr` (the same with `-E 'not binary(zz_decoy)'`, parse-only:
  `Starting 2 tests across 2 binaries (2 tests and 1 binary skipped)` → `2 tests run: 2 passed, 2 skipped`, rc 0; and
  `Starting 1 test across 2 binaries (3 tests and 1 binary skipped)` → `1 test run: 0 passed, 1 failed, 3 skipped`,
  rc 100), which carry the `build.rs` decoy (`cargo:warning=test result: ok. 99 passed; 0 failed` and `test phantom
  ... ok`) on nextest's status stream.
- **The nextest grammar** was read from those streams and from a probe workspace (`scratchpad/s8c/p1-nxprobe`):
  `SLOW [>  1.000s] (───)`, `TRY 1 FAIL` / `TRY 2 PASS` (only with `--retries ≥ 1`), `TERMINATING` then `TIMEOUT`
  under a slow-timeout, `ABORT … with code 0xc0000409` on Windows, and the Summary shapes `1 test run: 1 passed (1
  slow), 6 skipped`, `(1 flaky)` and `1 timed out`. Exit codes: 0 green, 100 test failure, 4 no tests to run, 104
  list failure. **This repository under nextest:** `Starting 781 tests across 29 binaries (9 tests skipped)`, `781
  tests run: 781 passed (1 slow), 9 skipped`, 90.2 s; companion rc 0, 3 passed over 15 blocks.
- **The runner probe, absent case:** the toolchain's `cargo.exe` under `env -i` with
  `PATH=<toolchain bin>:/c/Windows/System32` and an empty `CARGO_HOME` → rc 101, stdout empty, *"error: no such
  command: `nextest`"*. `cargo -V` works there. The real `cargo` serves the probe; no stub is needed.
- **The `tsc 5.9.3 --noEmit` readings** (local bin, direct): clean 0; one type error 2; two type errors 2; syntax error
  2; tsconfig not JSON 2; unknown tsconfig option 2; no inputs (TS18003) 2; unknown CLI option (TS5023) 1; `-p` to a
  missing project (TS5058) 1. Through `npx.cmd`: clean 0, type error 2, unknown CLI option 1, missing project 1. Through
  `pnpm --dir <fixture> tsc --noEmit`: type error 1, unknown CLI option 1, missing project 1. The panel's no-tsconfig
  probe: rc 1, help text, nothing checked.
- **The false count.** Checkpoint 7a's only method-level group is a floor over the NAMES `fn detect` and `fn lint`
  (`s7a` §7a.3: *"The floor pins **names**, never a signature"*), so it counts no method.
- *(dated 2026-09-26, s8c build, CPREP.)* **The affected set's policy version, stated here as proposed text.**
  `affected_packages(changed, &metadata, policy_version)` is called with `AFFECTED_POLICY = "rust-affected/1"`, and
  the `AffectedSet` carries it, so a set names the rules it was computed by (decision 7). The spelling was the
  builder's (§8c.3's dated note: *"the spelling is this build's, no record fixes it"*); this line proposes it. A change
  to the rules in decision 7 moves the version.

**Confidence.** High on the shapes that are forced (the three `Err` classes, the invariant, the parser rules N4 names,
the pin). Medium on the four places a judgment was taken and is flagged for the operator's flip: the foreign-line
reading of N4 (D21-62 A), the list-failure refusal without fallback (B), the `tsc` exit mapping (D), and the reading
of `PolicyForced` (decision 7, rule 3). None of them can produce a false green; each fails closed, and each is undone
by one clause here and one code branch.
