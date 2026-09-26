# Step 8c — The Toolchain port returns outcomes, and verbs route through the registry (Phase 1, part 8c)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8c` | Phase 1 | Checkpoint 8b | Checkpoint 8c | `s8b` | `s8d` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (`spikes/DECISIONS-2026-09-21.md`). The draft at `a1d35ac` predates all
three entries. What changed:

- **N4.** §8c.1 no longer measures for a count source and halts if it finds none. It builds and pins a strict
  libtest-line parser, with a golden fixture per toolchain version, and unparseable output is Unverifiable. Checkpoint
  8c (12)'s AC-329 floor and mutant M11 are re-cut to match. Mutants M12–M20 are added: three for the parser, and
  one each for the new behaviours below.
- **N25.**
  - `build` is widened to an outcome report (§8c.2).
  - A per-toolchain `setup` verb lands (§8c.4), with its `setup_timeout` warrant (§8c.5).
  - `sca-scan`'s `unrated` check is `s8b`'s catalog edit, and it is cited here, not re-decided.
- **N14.** `registry.toml` gains declared `[checklist.*]`, `[kb.*]` and `[feed.*]` tables, and an undeclared key is
  refused by name. The 14 keys the two lanes reach are enumerated. Their content is authored here, except
  `checklist:smoke`, which is `s8g`'s (§8c.4).
- **D21-45 W3.** This stage builds the `.worktreeinclude` reader, and `s8d-ii` calls it at creation (§8c.4). D21-46 R1
  names D21-44 and W1/W2/W4, not W3. That W3's carry at creation moves with the creation code was the lead's reading
  of R1. *D21-48 (2026-09-24):* X14 confirms it: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at creation."*
- **N16.** The 185 agent-step bounds are `s8d`'s (§8c.6).
- **`RK-D-TIMEOUT`** resolves `${toolchain.*}` through 4a's table (§8c.6).
- **`s8b`'s hand-ons.** The `smoke` row is `live = true`, and AC-018 is re-proven over the shipped catalog and
  registry (§8c.4).
- **Ownership.**
  - Record V owns the Rust verb handlers, and `s8g`'s R4 cites it.
  - The 7a "exactly six" erratum stands. After record V the port has seven methods, and `s8g` corrects its own entry
    table.
- **Cross-cutting.**
  - The whole-suite floor is re-derived at open from Checkpoint 8b's recorded count.
  - Only the entry gate and the checkpoints this stage's own edits make provisional (7a, 5) are re-run whole.
    Checkpoint 7c's whole re-run is `s8j`'s (N28).
  - A no-dispatch guard precedes the 8b re-run.
  - The run-worktree root is `.redkiln/worktrees/<item>` (D21-44) wherever this file speaks of a run-owned worktree.
- **Settled items.** Eight items the draft handed on as unassigned are settled, and §8c.12 says by what. One new
  finding is handed on in their place: the `backlog` feed has no producer. A second sibling-file note is added: the
  review stub's doc names the old worktree path.

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The run-worktree caller is `s8d-ii`, not `s8e`** (D21-46 R1). Every sentence here that named `s8e` as the stage
  that moves the root, calls `setup` and does the `.worktreeinclude` carry at creation, or sets `LC_ALL=C`, now names
  `s8d-ii`.
- **The optional-`null` rule cited ADR-0103 decision 4.** *Reverted 2026-09-24 (D21-47):* the rule cites ADR-0032's
  own Decision, *"Absent `?` inputs are `null`"*, which still stands. ADR-0103 decision 4 is predicate semantics.
- **The `backlog` feed has an owner** (D21-46 R7): `s8f` builds its producer and flips `[feed.*.backlog]`. §8c.4's row
  and §8c.12's handed-on item 3 say so, and the item is marked settled.
- **Checkpoint 8c (15) adopts D21-46's re-run convention and replay guard**: it skips its whole re-runs under
  `RK_CP_NESTED=1`, and otherwise launches 8b, 7a and 5 with `RK_CP_NESTED=1`, `REDKILN_LAUNCH_RECORD` unset and
  `CLAUDE_CONFIG_DIR` at an empty trap.
- **The floor chain is recomputed once**: Checkpoint 8b's 770 + this stage's 63 = **833**, not the drafted 831, and
  (13) gains the at-least-44-result-lines guard. *Moved 2026-09-24 (D21-47):* the fix round adds six new tests, so
  the chain reads 770 + 69 = **839** (§8c.9, (13)). *Superseded by the final consistency pass 2026-09-24:
  774 + 69 = **843**.*
- **Sibling notes settled**: `s8g`'s entry table reads the seven-method count, and the stub's path correction is
  `s8d-ii`'s.

*Adversarial-review fixes 2026-09-24 (D21-47)* (review of this file at `4a8c832`, fixed at `fc80687`; the ids are
the review's). **New tests: 63 → 69.**
- **#1** (BLOCKER): the run child passes `--no-fail-fast` (§8c.1 rule 4), and record T names it against ADR-0094
  decision 1's argv. Added a red golden with its failure in the first binary, the test
  `report::a_red_test_in_an_early_binary_still_tallies_every_binary`, and mutant M21.
- **#2** (BLOCKER): `verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map`, (11)'s tomllib
  key-map floor with its control, and mutant M22.
- **#3 and cross #1** (BLOCKER): (3) locates T, V and B by the id recorded at write time (`REC_T`, `REC_V`, `REC_B`,
  written by dated edit), and checks that each supersedes list contains its set. Record B says ADR-0140 decision 5
  is superseded in part by `s8d`'s R1 for per-step specs (§8c.7).
- **#4**: a sixth golden (this repository's own whole-suite stdout), the slow-test notice in the grammar as a skipped
  line, two tests, and mutant M25.
- **#5**: ADR-0099 decision 2's *"under the same timeout"* moves into record B's supersessions, and (3) checks
  0099's link.
- **#6**: design §3.3's `Toolchain` row takes a dated note under T and V (§8c.7).
- **#7**: record V places the smoke scratch directory (§8c.4). **Not fixable here:** `s8g` never flips
  `[checklist.smoke]`. That is reported under §8c.12's sibling files. *Settled (final consistency pass 2026-09-24, lead):* `s8g` §8g.6's R4 commit sets `path = "checklists/smoke.toml"`, and Checkpoint 8g (7h) asserts it with a control and mutant M26.
- **#8**: the three agent-step keys are lane-reached, so they must be cited, never `unmeasured`. There is a stop at
  stage open if no record exists (§8c.5; §8c.12 item 3).
- **#9**: `nextest_fallback_recorded_on_report`, both crates' `concurrent_verify_reports_share_no_output_file`,
  `d34_vitest_invoked_directly_never_pnpm_forwarded`, and Node's `ac325_recorded_argv_is_os_vector` (not in the
  review; it too calls `verify_report`) are re-proven. (5)–(10)'s `want` counts are raised to match.
- **#10**: `RK-D-TIMEOUT` stays `Error`, so an `unmeasured` key is an `Error` finding (AC-330). The consequence is
  stated in §8c.6.
- **#11**: AC-018 is claimed for its literal only. **#12**: the optional-`null` citation is ADR-0032's Decision
  again. **#13**: "R1 moved the carry" is labelled the lead's reading. **#14**: the reader uses the
  `:(exclude).redkiln/worktrees` pathspec, with a planted-sibling test and M24. **#15**: the `cancel.rs` and
  `ac334.rs` row is corrected. **#16**: the `smoke.status` sentence now cites R6 and `s8g`. **#17**: ADR-0135
  option 3 takes an erratum, its forbids sentence moves with V, and both `report.rs` docs are corrected. **#18**:
  per-tool database path and commit, and (15) records both around Checkpoint 5's fetching replay. **#19**: pins
  are read with tomllib. **#20**: `PolicyForced`'s doc and AC-328's stale half are both stated, and the stale half
  is excluded. **#21**: the verb lookup runs before bound resolution, with a test and M23. **#22**: a short
  measurement path with its length printed, and pnpm's `downloaded` count recorded.
- **Cross #18**: the stale "770 + 63 = 833" lines now read 839, and §8c.12 item 3 is moved to the settled table.
  *(843 since the final consistency pass 2026-09-24.)*
- **D21-47 rules**: (0b) sets the wrapper for the whole block and its cargo runs (a fake `claude` printing the pin,
  `REDKILN_LAUNCH_RECORD` unset, and the trap with its control), and (16) reads the trap empty. §8c.5's measurement
  runs under the same wrapper. The card and permission precondition stays inside the non-nested branch. The
  fixture harness is named `rk-fixture-child-claude`. `LsFiles` and `CheckIgnore` stand as reads under clause 5.

*Final consistency pass 2026-09-24* (the eleven `s8` files read against each other after the D21-47 fix round;
`runbook/tools/nodispatch.sh` and `runbook/tools/fake-claude.rs` land with this pass):
- **The shared no-dispatch wrapper.** §8c.5's measurement and Checkpoint 8c (0b) no longer build their own
  shell-script fake `claude` and trap directory. Each sources `runbook/tools/nodispatch.sh` and runs `nd_setup` and
  `nd_enter`: a COMPILED fake that a Rust `Command::new("claude")` on Windows resolves, every other `PATH` directory
  holding a `claude` removed, `REDKILN_LAUNCH_RECORD` unset, and the trap `CLAUDE_CONFIG_DIR`. `nd_check` follows each
  cargo group and each replay in (15), and closes the block at (16). (15)'s replays run as
  `nodispatch env RK_CP_NESTED=1 bash …`. §8c.11's *"is a `sh` script"* residual is replaced.
- **Record ids.** (3) already carries `REC_T`, `REC_V` and `REC_B` as literals written by dated edit, and it refuses
  while one reads `unrecorded`. (15)'s replay of Checkpoint 8b supplies no id, because each block carries its own
  (D21-47). §8c.11's (15) residual and (15)'s FAIL text now say so.
- **The floor chain, recomputed once, final.** Checkpoint 8b's expected count is 740 + `s8a`'s 9 + `s8b`'s **25** =
  **774**, not 770, so this stage's expected floor is 774 + 69 = **843** (§8c.11, (13)). §8c.9's own lists still give
  69 new tests. Every earlier 831, 833 and 839 in these notes is history.
- **`s8d`'s `side_effecting` column** is noted under §8c.4's row list. `s8d`'s R1 adds it to every row this stage
  lands (its M8).
- **`s8g`'s `[checklist.smoke]` flip** was still absent from `s8g`'s bytes at this pass. §8c.12's sibling note
  records it, with the final pass's assignment to `s8g` §8g.6. *Settled (final consistency pass 2026-09-24, lead):* `s8g` §8g.6's R4 commit sets `path = "checklists/smoke.toml"`, and Checkpoint 8g (7h) asserts it with a control and mutant M26.

*D21-48 (2026-09-24)* (`spikes/DECISIONS-2026-09-21.md` D21-48: the operator's answers to the sixteen questions the
`s8` files left; this file's are X4, X13, X14 and X15). **New tests: 69 → 78.**
- *D21-48 (2026-09-24), lead:* **`test_timeout` is measured on the selected runner** (§8c.5), not on `cargo test`
  alone. **The install moves Checkpoint 7c's premise**, so this stage's work commit amends it by D21-47's
  time-bound rule. `s7c-adapters.md`'s `nextest_fallback_recorded_on_report 'nextest ABSENT on this box'` line, its
  §7c.22 ARMED presence-arm row and the RT-26 text each take a dated note: nextest is installed from `s8c` on, the
  fallback test runs under a controlled nextest-absent probe (§8c.9), and the presence arm is live. 7c is listed as
  provisional in this stage's tracker row, and `s8j` re-runs it whole.
- *D21-48 (2026-09-24):* **X4.** `implement_timeout` and `repro_timeout` are 4,800 s each, cited to F31/S-TYPED's
  slowest agent dispatch (4,561 s) as a borrowed class. §8c.5's row replaces its stage-open stop with the decision,
  and a new paragraph names the re-derivation rule and the duration records it reads.
  `verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map` asserts both values, (11) gains a
  borrowed-class floor with its control, and mutant M31 is added. §8c.12's handed-on item 3 moves to the settled
  table.
- *D21-48 (2026-09-24):* **X13.** Record B supersedes, each in part and each quoted, ADR-0140 decision 1, decision 2's
  sentence, decision 4, the first two forbids bullets and ADR-0099 decision 2's *"under the same timeout"*. §8c.7's
  B row quotes all five sites and drops *"That gap is reported, not resolved here"*. §8c.12's handed-on item 1 moves
  to the settled table.
- *D21-48 (2026-09-24):* **X14.** The `.worktreeinclude` owner is confirmed: this stage builds the reader, and
  `s8d-ii` calls it at creation. The flip-line hedge is removed from the W3 note above and from the not-here table.
- *D21-48 (2026-09-24):* **X15.** cargo-nextest is installed and supported here. §8c.1 replaces *"an operator decision
  (a tool install), asked, never assumed"* with the install at stage open, the CI install step, a nextest line parser
  beside N4's, its goldens, runner selection per ADR-0094 decision 1 and the doctest companion child. `TestOutcome`
  gains `runner` (§8c.2). Record T executes X15 and supersedes ADR-0094 decision 1 in a third respect (§8c.7). §8c.8
  gains step 0. §8c.9 gains nine Rust-adapter tests and mutants M26–M30, and `nextest_fallback_recorded_on_report`
  keeps its name over a controlled nextest-absent probe. Checkpoint 8c (2) gains the installed-version, golden and
  `ci.yml` preflights, (3) reads the pin in record T, (7)'s `want` goes 39 → 48, and (12) gains the
  doctest-companion floor, each with a synthetic control.
- *D21-48 (2026-09-24):* **The floor.** Re-derived at stage open from Checkpoint 8b's recorded count:
  774 + 78 = **852** (§8c.11, (13)). Every later stage's floor moves by nine with it. That is reported to the later
  files under §8c.12's sibling notes, not edited here.
- *(dated 2026-09-25, `s8b` refuting round 4)* **The 774 is stale.** It took `s8b`'s count as 25 on 740 + `s8a`'s 9.
  `s8a` closed at a recorded 755, and D21-57 added four tests to `s8b` (29 new, floor **784**). This chain, here and in
  (13)'s comment, takes Checkpoint 8b's recorded close figure, not 774; the 852 is not recomputed here and is
  re-derived at stage open, as the line above says.

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
- **`build` has no outcome, and no checklist, atom or feed has a source.** *(Added at this revision, re-read at
  `a6349b6`.)* The port declares `fn build(&self) -> Result<(), StepError>;`, so a red build is `Err` and
  `build-package`'s `report.status == 'passing'` can never see one. Over the step `inputs` of the 102 files in
  `.redkiln/catalog/` there are **50** distinct `checklist:` / `kb:` / `toolchain:<feed>` keys (27, 12 and 11), and
  nothing declares or holds any of them. D21-43 N25 and N14 take both.

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

**This step executes taken decisions and takes no free one:**

- D21-42 1a, 1b, 4a, 4b and 4c (the adapter side);
- D21-43 N4 (the parser), N14 (the source tables) and N25's `setup` verb and `build` outcome;
- D21-45 W3's reader.

What is left is execution order, plus the three records those decisions say must be written (§8c.7). Where
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
| The producer ACTIVITIES `smoke-test`, `sca-scan`, `build-package`, `tag-release`, and the shipped `smoke` checklist | `s8g` (D21-42 6a, 6b). Its R4 cites record V for the handlers (D21-43: *"The Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4)."*) |
| `sca-scan`'s check reading `unrated` | `s8b` (D21-43 N25: *"**`sca-scan` requires `unrated == 0`** or goes to `scan.accept-findings`"*, a catalog edit) |
| Enforcing the 185 agent-step `timeout_secs` | `s8d` (D21-43 N16: *"**Carried by each per-step spec** (3a), resolved through 4a's table and bounded by the activity budget. Owner: s8d."*) |
| Resolving `checklist:` / `kb:` / `toolchain:<feed>` inputs through the tables this stage declares | `s8d` (N14, its S3) |
| The run-worktree root `.redkiln/worktrees/<item>`, calling `setup` and the `.worktreeinclude` carry at creation, the filter-driver refusal and `redkiln init`'s exclude block | `s8d-ii` (D21-46 R1 moved D21-44 and W1/W2/W4 out of `s8e`: *"It carries D21-44's relocation and D21-45 W1/W2/W4"*. R1 does not name W3, and D21-48 X14 settles the owner: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at creation."*; D21-44; D21-45 W1, W2, W4, and W3's carry: *"W3 and W4's `init` half go to **s8c** (the `setup` verb and `.worktreeinclude` reading) and s8e (the carry at creation). `redkiln init`'s exclude block is `s8e`'s too"*) |
| `item_create`'s rk-store write | `s8f` §8f.7 (D21-42 6a: *"DebtRegisterEntry is an rk-store item write with provenance; human-sourced facets are null and go to `unknown_facet`."*; re-quoted by the final consistency pass 2026-09-24) |
| Any dispatch to a real account | nobody before `s8i` (D21-40 decision 2) — **this stage dispatches nothing** |

---

### 8c.1 The per-test counts come from a strict libtest-line parser, pinned per toolchain version (D21-43 N4)

1a needs `passed` and `failed` from a red Rust suite, and the pinned stable toolchain has no structured per-test
source. `s7c` measured the options on 1.98.1:

- *"`libtest --format json` … **does not exist on 1.98.1**"*;
- `--format junit` is gated the same way;
- nextest is ABSENT (at `s7c`; D21-48 X15 installs it at this stage's open, below);
- *"`RUSTC_BOOTSTRAP=1` is a workaround Halt 7 forbids"*.

This section's draft measured candidates and halted if none qualified. **D21-43 N4 took the question instead**: *"A
strict parser over libtest's `test <name> ... ok|FAILED` and `test result:` lines, pinned by a golden fixture per
toolchain version."* and *"Unparseable output is Unverifiable, never green."* So this section builds and pins the
parser. It does not halt on a missing source.

**AC-329 and the parser.** AC-329 says *"no field of the report is obtained by parsing prose or markdown"*. Three
things keep the parser inside that sentence:

- The parser's `passed` and `failed` are `TestOutcome` fields, not `VerifyReport` members (§8c.2), and every member
  AC-329 lists is produced as it is today.
- The lines it reads are a line protocol, pinned per version by a golden. They are not prose.
- Record T states this reading of N4 against AC-329, and the register takes a dated note (§8c.7).

The fixture project's two decoys still apply. `tests/zz_decoy.rs` prints the summary line and a per-test `... ok`
line on stdout, and `build.rs` prints the same two lines on stderr.

**The parser's contract.** Record T fixes it, and this is the proposal. `crates/rk-adapter-toolchain-rust/src/
libtest_lines.rs` holds one pure function, `parse(stdout, exit_code, version) -> Result<Tally, ParseRefusal>`,
over the run child's stdout FILE. It never reads stderr, so the `build.rs` decoy cannot reach it.

1. **Pinned per version.** `PINNED` lists the release numbers of `rustc -V` for which a golden exists: at this
   stage `1.98.1`, the `rust-toolchain.toml` channel. A version it does not list is refused by name
   (`Unsupported { capability: "libtest-lines@<version>" }`), never parsed on trust. So a pin bump adds a golden and
   a list entry in the same PR that moves the pin.
2. **Block-framed.** A block opens on `^running (\d+) tests?$`. Its RECORD region is the lines up to the first
   empty line. Every line in the region must match `^test (.+) \.\.\. (ok|FAILED|ignored(, .*)?)$`, and there must
   be exactly as many as the header states. **One line is skipped, not refused**: libtest's own slow-test notice,
   `^test (.+) has been running for over 60 seconds$`, which it prints inside the region for a test that runs past
   60 s. The name it carries must be one the same block records. If it is not, the block refuses. The self-hosting
   suite has tests that run that long, so without this line the parser would refuse on every run of this repository.
   Any other non-empty line in the region refuses. The block closes on the
   full summary grammar:
   `^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in [0-9.]+s$`.
   The lines between the region and the summary are skipped: libtest's `failures:` section and captured test output.
   **A record-grammar line outside a block is never counted.** That is how the stdout decoy is survived:
   `zz_decoy` prints no `running` header.
3. **Self-consistent.** Each summary's `passed`, `failed` and `ignored` must equal that block's tallied records. A
   disagreement refuses. So does a header without a summary, or a summary without a header.
4. **Multi-binary.** `cargo test --workspace` runs its binaries in sequence, and each prints its own block on the one
   stdout. The tally is the sum over blocks. Doctests are blocks of the same grammar. Their names contain spaces,
   which `(.+)` admits. **The run child passes `--no-fail-fast`.** Without it, cargo stops at the first binary that
   fails. A red suite whose failure sits in an early binary then prints fewer blocks than `--list` counts, rule 5
   refuses, and every red suite on this 44-binary repository reads as cannot-run. At `fc80687` `report.rs` declares
   `const RUN_ARGS: [&str; 2] = ["test", "--workspace"];`, and the flag is added there. Record T names it as a
   deviation from ADR-0094 decision 1's argv (§8c.7).
5. **Against `--list`.** `passed + failed` equals the `--list`-derived `selected_test_count`, which is §8c.2's
   invariant, on the green fixture AND the red one (`red_by_selection`). The report does not redefine the count to
   fit.
6. **Exit-code agreement.** Two mismatches refuse:
   - A non-zero exit whose blocks report `failed == 0` means something failed that no record names. A
     `harness = false` target is the usual cause, since it has no block and only its exit code speaks. This refuses
     as `UnattributedFailure`, and it is never read as green.
   - A zero exit with `failed > 0` refuses too.

   `zz_decoy` exits 0, so it moves no count, and the golden records that.

**A refusal is Unverifiable.** The adapter maps every `ParseRefusal` to `Err(Transport { Decode })`, naming the
refusal and the version. This is §8c.2's error class (c):

- the review re-executor reads it as `cannot_run` (AC-344);
- `s8g`'s verify pass reads it as Unverifiable;
- it is never `Ok` with `failed = 0`.

**The golden fixture.** `crates/rk-adapter-toolchain-rust/tests/fixtures/libtest/1.98.1/` holds six captured
run-child stdout files, each with its expected tally beside it:

- the fixture project green, with `zz_decoy` included;
- the same project red under `red_by_selection`;
- §8c.3's workspace, for several binaries;
- the same workspace red, with its failing test in package `a`'s unit-test binary. That is the first binary cargo
  runs, so every later block must still be tallied;
- one doctest block;
- **this repository's own** `cargo test --workspace --no-fail-fast` stdout at the stage's pin, so that the parser is
  proved over the output the self-hosting run meets. If the parser refuses it, the stage stops. In that case the
  parser is wrong, not the golden.

They are captured once, on the pinned toolchain, and committed LF under ADR-0164. **They are parsed by the tests,
never compared byte for byte with a fresh run**, because `finished in` varies. The capture command and its
`rustc -V` go into record T's evidence.

**Nextest is installed and supported (D21-48 X15; *D21-48 (2026-09-24)*).** ADR-0094 decision 1 admits *"`cargo
nextest run` where present, recorded on the report"*. The draft left the install to the operator, and X15 takes it:
*"**Install it and support it now.** s8c installs cargo-nextest on this box and in CI, parses nextest's output beside
`cargo test`'s, and records on the report which runner ran. The install is s8c's act, at stage open."* So nextest
needs no further question. Record T fixes the shape, and this is the proposal:

- **The install, at stage open.** `cargo install cargo-nextest --locked` on this box, before any code (§8c.8 step 0).
  The installed `cargo nextest --version` line is recorded in record T's evidence, and that version is **the pin**.
  CI installs the same version. A later bump moves the pin, the CI line and a new golden in one PR, as a `rustc` bump
  moves `PINNED`.
- **The CI step.** `.github/workflows/ci.yml`'s `gates` job gains a step, `install cargo-nextest`, that runs
  `cargo install cargo-nextest --version <the pin> --locked`. It sits **immediately before the `test` step**, because
  the adapter's nextest-path tests run inside that step's `cargo test --workspace` and spawn `cargo nextest run`
  against the fixtures. The existing `install audit tooling` step comes after `test`, which is too late. The gate's
  own command stays `cargo test --workspace`: the runner the adapter selects is the adapter's, and CI's gate does not
  change runner. The `no egress` job runs `rk-kernel` only and needs no install. Checkpoint 8c (2) asserts the step's
  version and its order.
- **Runner selection** (ADR-0094 decision 1). At construction, the Rust adapter probes `<program> nextest --version`
  under the `Config`'s `child_env`. It runs nextest when the probe exits 0 with a `cargo-nextest <version>` line, and
  `cargo test` otherwise. `Config` gains a `runner` policy: `Detect`, which `real_toolchain` uses, or
  `Force(runner)`, for a test that pins one path's vector. The runner that ran is recorded twice. `VerifyReport.argv`
  is the run vector as handed to the OS (AC-325), so it names `nextest` exactly when nextest ran. `TestOutcome.runner`
  (§8c.2) names the runner and its version. `VerifyReport` gains no member.
- **The nextest parser.** `crates/rk-adapter-toolchain-rust/src/nextest_lines.rs` holds one pure function,
  `parse(stream, exit_code, version) -> Result<Tally, ParseRefusal>`, beside `libtest_lines::parse`. It returns the
  same `Tally`, so `test` returns the same `TestOutcome` whichever runner ran. Its rules are N4's, carried over:
  - **Pinned per version.** `NEXTEST_PINNED` lists the `cargo nextest --version` releases that have a golden. An
    unlisted version is refused by name (`Unsupported { capability: "nextest-lines@<version>" }`), never parsed on
    trust.
  - **Framed and self-consistent.** Nextest runs with `--no-fail-fast`, `--retries 0`, `--color never`,
    `--status-level pass` and `--final-status-level none`, so each executed test prints exactly one status line.
    The run's `Starting` header states a count, and there must be exactly that many `PASS` or `FAIL` records. The
    `Summary` line's run, passed and failed counts must equal the tallied records. A disagreement refuses.
  - **No retry and no kill is counted.** A `TRY`, `RETRY`, `FLAKY`, `TIMEOUT` or signal line refuses. A retried test
    is two runs of one test, and a timed-out one is a kill, not a failure (AC-331). A `SLOW` notice is skipped, as
    N4 skips libtest's, and it must name a test the run records.
  - **Exit-code agreement**, as N4's rule 6.
  - **The grammar is measured, not remembered.** The exact regexes, and which stream carries the status lines, are
    read from the installed version's captured output at stage open and written into record T before the parser is.
    The selection arguments are measured the same way: `--exact`, `--ignored`, `--include-ignored`, `--skip` and the
    name filters `review_pass::selection_of` produces. An argument the installed nextest does not accept refuses by
    name. It is never dropped.
- **The doctest companion.** `cargo nextest run` runs no doctests, and `cargo test --workspace` runs them as blocks of
  this section's grammar. So on the nextest path the adapter runs a second child, `cargo test --workspace --doc
  --no-fail-fast`, under the same bound. An `Affected` run's `-p` set replaces `--workspace`, and a `Filter`'s
  `-- <args>` follows. The child is parsed with `libtest_lines::parse`. The outcome sums both tallies, and
  `wall_secs` sums both children. When the selected packages have no library target (read from `cargo metadata`), no
  companion runs, and `TestOutcome.runner` says so. **§8c.2's invariant is what keeps the test set whole.**
  `passed + failed` must equal the `--list`-derived `selected_test_count`, and `--list` lists the doctests. A nextest
  run whose doctests were dropped therefore refuses as `Transport { Decode }`. It never reads as a smaller green
  suite.
- **The goldens.** `crates/rk-adapter-toolchain-rust/tests/fixtures/nextest/<version>/` holds, for the installed
  version, three captured status streams, each with its expected tally beside it: the fixture project green (with
  `zz_decoy`), the same project red under `red_by_selection`, and §8c.3's workspace red in package `a`. Package `a`
  carries one doctest, so the workspace golden also proves the companion. The goldens are committed LF under
  ADR-0164 and **parsed, never compared byte for byte**, like the libtest ones. The capture commands and the version
  go into record T's evidence.

The Node adapter needs no parser. `report.rs` already reads *"`numPassedTests`, `numFailedTests`,
`numPendingTests`, `numTotalTests`"* from vitest's JSON reporter file.

### 8c.2 `Toolchain::test` returns an outcome (D21-42 1a)

**The shape.** Record T fixes it, and this is the proposal:

- **`test` returns a `TestOutcome`**, an `rk-ports` type that wraps the unchanged `rk_types::verify::VerifyReport`
  and adds four fields: `passed: u32`, `failed: u32`, `status: TestStatus` and `runner: TestRunner`.
  - `VerifyReport` gains no member. AC-329's member list is closed, the store's `validate_report` (ADR-0104
    decision 3) reads it, and `deny_unknown_fields` guards it.
  - `status` is **derived and never stored independently**. It is `Passing` iff `failed == 0 && passed > 0`,
    `Failing` iff `failed > 0`, and `Empty` iff `passed + failed == 0`.
  - `Empty` is AC-332's *"'zero tests executed' is never rendered as green"*. The catalog compares `status` only
    to `'passing'`, so an empty run can never satisfy a check.
  - The serde form spells `status` as `passing` / `failing` / `empty`, which is the vocabulary the 25 catalog
    comparisons already use.
  - *D21-48 (2026-09-24):* `runner` records which runner ran (X15: *"records on the report which runner ran"*). It
    holds the runner's `name` (`cargo-test`, `cargo-nextest`, `vitest`, or `fixture` for the fake), its `version`,
    and `companion_argv`, the doctest child's vector when the runner runs no doctests itself (§8c.1).
    `validate_report` refuses an empty `name`.
- **Invariant.** `passed + failed == report.selected_test_count`, checked at construction. If it fails, the result
  is `Err(Transport { Decode })` naming both numbers. It is never a corrected count.
- **`Ok` means the suite ran and every executed test reported, green or red.** A red suite is
  `Ok(TestOutcome { failed ≥ 1, status: Failing, … })`.
- **`Err` means exactly three things:**
  - (a) **Cannot run.** The program cannot start (`Transport { Spawn }`, AC-344), or the build fails before any
    test executes (`NonZero` with no per-test record).
  - (b) **Killed.** `Timeout { after_secs }`, or an OS kill with the `os-killed:` prefix (AC-331).
  - (c) **Unreadable.** The files around the run cannot be read, or the run child's stdout does not parse under
    §8c.1's contract (`Transport { Io | Decode }`).

  **A non-zero exit whose blocks name the failures is a red suite, not an error.**
- **RT-22 is reversed by record** and **RT-6 is discharged**. The inherent `verify_report(&self, …)` methods on
  `RustToolchain` and `NodeToolchain` are **retired**, so that exactly one path produces a report. The module
  functions `report::verify_report` stay as the port method's body. **R-9 is discharged for the transport**,
  because the report now crosses the port. The verify pass that consumes it is `s8g`'s.
- **The method count does not move under 1a.** 1a changes a signature and 1b adds the seventh method (§8c.4).

**`build` returns an outcome too (D21-43 N25: *"**`build` is widened to an outcome report** as 1a did for
`test`"*).** Record T fixes it, and this is the proposal. The slice is the same one, because the signature moves
every double.

- **The signature.** `fn build(&self, bound: StepBound) -> Result<BuildReport, StepError>`. `BuildReport` is an
  `rk-types` struct carrying:
  - `argv`, `toolchain_version`, `wall_secs`;
  - `errors: u32`;
  - `status`, which is `passing` or `failing`.

  It is the `outputs` kind that `spikes/p9/registry.toml` already names for `build`.
- **The Rust adapter reads cargo's structured stream.** It runs `cargo build --workspace --message-format=json`.
  - `status` is the `build-finished` message's `success`.
  - `errors` counts the `compiler-message` records at level `error`.
  - So a compile error is `Ok(BuildReport { status: failing, errors ≥ 1 })`.

  A non-zero exit with no `build-finished` message is `Err(Transport { Decode })`, and so is a stream that disagrees
  with its exit code. Spawn, timeout and kill are `Err` as for `test`. No accepted record fixes `build`'s return
  type: searching `references/adr/` for `Result<(), StepError>` and `fn build(` finds 0 hits at `a6349b6`. ADR-0094
  decision 1's *"`build` = `cargo build --workspace`"* stands, because the flag changes the stream that is read, not
  what is built. Record T says so.
- **The Node adapter** maps `tsc --noEmit`'s own exit statuses:
  - `1` and `2` (diagnostics present) are `Ok(failing)`;
  - any other non-zero status is `Err(NonZero)`.

  **Measure that mapping on the pinned `tsc` before relying on it**, and record the reading in record T's evidence.
- **Downstream.** `build-package`'s `built` check (`report.status == 'passing'`) can now see a red build. The
  resolver side of that check is `s8d`'s.

**Every caller and every double moves in the same slice.** Measured at `ca8f387`: six `impl … Toolchain for` sites
in `crates/`, plus one fixture PROGRAM. The facts brief placed three of them in `fakes.rs`. Only one is there.

| site | what changes |
| --- | --- |
| `rk-ports/src/fakes.rs` `FixtureToolchain` | `test` returns an outcome. `passed = selection` length, `failed = 0` by default, and a builder scripts `failing(n)`. `build` returns `passing` by default, and a builder scripts `build_failing()`. The report members are non-empty fixture values, and `runner` names `fixture` (*D21-48 (2026-09-24)*). The `impl … for` count in `crates/rk-ports/src/` **stays 5** (Checkpoint 7a) |
| `rk-ports/src/conformance.rs` `conform_toolchain` | `let ran = toolchain.test(&[]).expect(…)` reads `outcome.passed` against the row's count, and gains **`validate_report`** (below) |
| `rk-cli/src/verbs/run.rs` `UndetectedToolchain` | every method, `run_verb` included, refuses by name as today |
| `rk-cli/tests/it/review_pass.rs` `ScriptedToolchain` | three modes: green (`Ok`, `failed = 0`), red (`Ok`, `failed ≥ 1`) and cannot-run (`Err`). **Red is no longer `Err`** |
| `rk-kernel/tests/it/runtime.rs` `CountingToolchain` | counts `run_verb` too, and records the `StepBound` it was handed (§8c.6) |
| `rk-cli/src/review_pass.rs` `ToolchainReExecutor::run` | `four_cell(candidate.failed > 0, reference.failed > 0)` when both arms are `Ok`. **If either arm is `Err`, it returns `None`**, which `confirm_with` reads as `Unverifiable { cannot_run }` (AC-344). A red candidate's `observed` string names its counts and can never equal `expected_when_met` |
| `rk-cli/tests/fixtures/bin/rk-fixture-toolchain.rs` (a stub `cargo`, not an impl) | it must print one whole §8c.1 block for its one test: `FAILED` in the candidate tree, `ok` in `…review-ref`, with a full-grammar summary each time. Its version is the REAL `rustc -V` (the stub's own doc: *"`rustc -V` is spawned separately by `toolchain_version` and is the REAL `rustc`"*), so it is `PINNED`. **If it does not, the candidate reads as cannot-run, and AC-438's tenth row (`exit/findings-block/`, code 4) silently becomes exit 0**, because that fixture is the ONLY data condition that reaches `four_cell(true, false)` |
| `rk-adapter-toolchain-node/tests/it/cancel.rs`, `…/ac334.rs` (corpus-bound) | `cancel.rs` calls `tc.test(…)` and reads a `Timeout`, which stays `Err`. It has no `NonZero` arm, and it moves for the new signature only. `ac334.rs` holds the one `NonZero` arm, `Err(rk_types::error::StepError::NonZero { .. }) => (false, None)`, and that arm becomes a `failed > 0` read: the mapping is exactly the reading 1a reverses. `ac334.rs` also calls the retired inherent `verify_report` twice, and those calls move to the port's `test` [measured at `fc80687`] |
| the adapters' `tests/it/report.rs` and `tests/it/registry.rs` | every `tc.verify_report(…)` call moves to the port's `test`. Measured at `fc80687`, the calls sit in `ac329_report_conforms_no_prose_parse`, `ac325_recorded_argv_is_os_vector` and `concurrent_verify_reports_share_no_output_file` (both crates), `nextest_fallback_recorded_on_report` (Rust), `d34_vitest_invoked_directly_never_pnpm_forwarded` (Node), and both `ac328_affected_selection_unsupported_without_probe`. Checkpoint 8c (5)–(10) re-proves each one by name |

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
`grep -rnE '\.(test|build)\(' crates/*/src crates/*/tests --include=*.rs` and read every hit:

- each `test` hit must read `failed` or `status`;
- each `build` hit must read `status`;
- no hit may read `is_ok()`, `is_err()` or `?` as a verdict. §8c.9 (11) keeps a code-line floor on the
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
   doc names. **Both facts are stated.** The adapter produces `basis: Affected` from a `Recorded` row, and nothing
   compares that row's staleness. AC-328's last clause reads *"no `basis: affected` record can be produced … while the
   probe entry is absent or stale"*. The absent half holds here. The stale half is not claimed (§8c.10).
2. An **empty** change set is refused by name. An affected run over nothing is a caller error. It is not a green,
   and it is not a whole suite.
3. Every changed path maps to the workspace package whose manifest directory is its longest prefix. This is read
   from `cargo metadata --format-version 1`, a structured stable read that ADR-0094 decision 1 already names.
   *(Noted 2026-09-25 at `s8a`, D21-51: after ADR-0165 decision 9, ADR-0094 decision 1 still names the
   `cargo metadata` read for the adapter, but no longer for `classify` or the `Text` arm, so it no longer
   carries this rule alone. Record T's Decision must state the `cargo metadata --format-version 1` read itself.)*
   **A path in no package** — the root `Cargo.toml`, `Cargo.lock`, `rust-toolchain*`, `.cargo/` or any non-crate
   path — forces the whole suite. Record T picks the reason from the six and states why: the proposal is
   `PolicyForced`, with this rule as the adapter's declared policy. `PolicyForced`'s own doc in `rk-types` reads
   *"The pack's policy demanded the whole suite regardless"*. That names the PACK's policy, not an adapter's. Record T
   states its reading against that doc, and the doc takes a dated note with the record. This file does not choose
   further.
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
for §8c.4's `smoke` and `package`, and a committed `Cargo.lock`. The same workspace supplies §8c.1's multi-binary
golden. *D21-48 (2026-09-24):* package `a`'s library carries one doctest, for the nextest path's doctest companion
and its golden (§8c.1; X15). Everything is LF under ADR-0164, so no `-text` pin is needed.

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

*Final consistency pass 2026-09-24:* `s8d`'s record R1 adds a sixth column, `side_effecting`, to every row, and a row
without a value refuses the registry at load (`s8d` §8d.1 R1; its M8). `s8d` writes the column into the rows this
stage lands, in its own work commit, and marks `package`, `item_create` and `release_tag` `true`. This stage lands the
rows without it.

**The rows this stage lands, and who owns each producer:**

| verb | route | typed output | handler built here | producer activity — owner |
| --- | --- | --- | --- | --- |
| `test`, `build`, `lint`, `change_summary` | `port` | `TestReport`, `BuildReport`, `LintReport`, `ChangeSummary` (the spike's own `outputs`) | the existing port methods; `test` and `build` now return outcomes (§8c.2) | — |
| `smoke` | `toolchain` | `SmokeResult` | **yes**, Rust: build, then run each declared checklist row against the built binary | `smoke-test` — `s8g`. `checklist:smoke` resolution is `s8d`'s; the shipped `smoke` checklist is `s8g`'s |
| `sca_scan` | `toolchain` | `ScanReport` | **yes**, Rust | `sca-scan` — `s8g` |
| `package` | `toolchain` | `BuildArtifact` | **yes**, Rust: `cargo build --release --locked --message-format=json`, then the sha256 of each **declared** shipped binary — never a glob, because a release build also links `rk-fixture-toolchain` and the other fixture `[[bin]]`s | `build-package` — `s8g`, which writes this repository's declared set (`redkiln`, `redkiln-migrate`) into the per-toolchain table |
| `setup` | `toolchain` | `SetupResult` | **yes**, Rust: the install over the declared project set (below) | none. No catalog step runs it. `s8d-ii` calls it at run-worktree creation (D21-45 W3; D21-46 R1) |
| `item_create` | `store` | `ItemRefs` | **no**. Refused naming the route until `s8f` | `debt-logging` — the write is `s8f`'s |
| `release_tag` | `git` | `ReleaseTag` | **no**. Refused naming the route until `s8g` | `tag-release` (new, via ADR-0114 clause 3's trial) — `s8g`. Local, never pushed (D21-42 6b) |
| `shallow_clone`, `verdict_record` | — | — | not here | `s8d` |
| the other **23** verbs, `release_record` and `mutate` among them | no row | — | refused by **bare** name (ADR-0135 clause 2, unchanged) | Phase 2 or unowned |

**Liveness, as `s8b` hands it on.** `s8b` §8b.5 deletes the per-step `memo = false` keys from 19 steps over twelve
verbs, and it hands the liveness to this table: *"A registry row for any of them without `live = true` would make
memoisable a step that its author marked otherwise. That is `s8c`'s to settle"*. Of the twelve, only `smoke` gets a
row here, and it is **`live = true`**. The other eleven have no row, so they refuse by bare name and nothing about
them is memoised. `s8b` also hands on AC-018's re-proof over the shipped registry (*"The re-proof against the
shipped registry is `s8c`'s"*). That re-proof is `verb_registry::ac018_holds_over_the_shipped_catalog_and_registry`
(§8c.9).

**Record V owns the handlers, and `s8g`'s R4 owns the activities and the checklist and cites V.** This is D21-43's
integration call: *"The Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4)."* So §8c.4
fixes what `smoke`, `sca_scan` and `package` do, and `s8g` does not decide it again.

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

**Typed outputs.** `BuildReport` (§8c.2), `SmokeResult`, `ScanReport`, `BuildArtifact`, `SetupResult`, `ItemRefs` and
`ReleaseTag` are added as
`rk-types` structs with `#[serde(deny_unknown_fields)]`, no `Default`, and every member required. **They are not
added to `EXPORTED`.** Those schemas are presented to agents, and which kinds an agent emits is `s8h`'s (D21-41 g).
Keep them `oneOf`-free anyway, because Checkpoint 5 greps the snapshot directory for it. What each struct carries:

- **`SmokeResult`**: one row per checklist entry (`id`, the argv run, its `cwd`, the expected exit, the actual
  exit, the stdout predicate and whether it held, `passed`), plus `passed`, `failed` and `status`. The checklist
  item form (`argv`, `cwd`, `expect_exit`, one stdout predicate) is the one `s8g` §8g.6 authors at
  `.redkiln/checklists/smoke.toml`. The handler runs the binaries the call's build produced, never an installed
  `redkiln`. It follows 1a's rule: **a checklist row that fails is `Ok` with
  `failed ≥ 1`**, and `Err` means only that the build or a spawn could not run, or that the run was killed. At
  `fc80687` **no check reads `smoke-test`'s result**: its steps are `build`, `smoke` and `emit`. D21-46 R6 adds
  *"a check on `status == 'passing'` with `on_false = "fail"`"* as `s8g`'s ADR-0072 edit (§8g.6, R4). Today only
  `rolling-deploy`'s `healthy` step reads `smoke.status == 'passing'`, with `on_false = "continue"`.
  - **The scratch directory.** `s8g` §8g.6 requires every item's `cwd` to be a scratch location outside the
    run-owned worktree, and it leaves the placement to record V. This is the proposal. The handler makes one empty
    directory per call, under ADR-0137 clause 6's scratch location (`<git-common-dir>/redkiln/tmp/smoke-<unique>`).
    That is outside every `.redkiln/worktrees/<item>` and outside the project's checkout. It resolves each item's
    `cwd = "scratch"` to that directory, and it removes the directory on every path: success, failure and kill. A
    removal that fails is `Err` naming the path, never a silent leak. The handler runs no git itself, so ADR-0105
    clause 5's write set is untouched. If an item needs the directory to be a repository, making it one is that
    checklist row's own argv, which is `s8g`'s.
- **`ScanReport`**: `critical`, `high`, `medium`, `low` and **`unrated`**, each required; one entry per advisory
  (id, package, version, severity or none); and, **per tool**, the tool's name and version, its advisory database's
  local path and **that database's commit** (D21-42 6a). `cargo audit` and `cargo deny` keep separate databases, so
  the report carries both. **An advisory with no severity is counted in `unrated`. It is never counted as low or none, and it
  is never dropped.** D21-43 N25 settled the activity's side: *"**`sca-scan` requires `unrated == 0`** or goes to
  `scan.accept-findings`"*. That catalog edit is `s8b`'s, and this required member is what it reads.
- **`BuildArtifact`**: the profile, one `(name, relative path, sha256)` per **declared** binary, the argv, the
  toolchain version, and the input `sha` together with the project root's `HEAD` read through `rk-git`. The
  digest is `rk_types::digest::sha256`, the plain hash, so that an independent `sha256sum` agrees. **A declared
  binary that the build did not produce is refused, and so is an `sha`/`HEAD` mismatch.** A package built from a
  tree other than the one named is not that sha's artifact. An undeclared set is refused by name.
  *(Noted 2026-09-25, s8a fix round 15: `s8a`'s Checkpoint 8a guard (6) lets the Rust adapter's code lines name
  `rk_git` only as `rk_git::diff` and `rk_git::path`, and this HEAD read is in neither, so that allowlist is a pin
  this stage amends. The commit that lands the `package` handler's HEAD read either amends guard (6)'s allowlist
  by a dated note in `s8a`, or places the read where the allowlist already admits it, and records the choice here.
  `s8e`, `s8i` and `s8j` re-run Checkpoint 8a whole, so leaving the pin unamended turns them red.)*
- **`SetupResult`**: one row per declared project (its path, the argv run, and the sha256 of the lockfile it
  installed from), plus the toolchain it ran for. It has no `status`, because a failed install is never `Ok`
  (below).

**The scan's semantics are the ones `s8g` §8g.6 reads, and record V fixes them so that the two files agree.** Both
`cargo audit --json` and `cargo deny check advisories` run. Severity maps each advisory's CVSS base score onto the
CVSS v3 qualitative bands: `critical` ≥ 9.0, `high` 7.0–8.9, `medium` and `low` below. An advisory with no CVSS is
`unrated`. The database commit is read from the report's own database member, and **if the installed tool emits no
such member, the scan refuses by name** rather than record an unknown database. The verb runs **offline, against a
pinned local database** (`--no-fetch` or its equivalent), because a scan whose database moved mid-lane is two
instruments (golden constraint 8). **The two databases are two pins.** On this box on 2026-09-24, `cargo audit` reads
`~/.cargo/advisory-db` and `cargo deny` reads `~/.cargo/advisory-dbs/advisory-db-<hash>`. Each is its own git
checkout, and both were at `ef8244d2`. **Checkpoint 5's replay in (15) fetches both.** Its lines are
`cargo audit --deny unmaintained` and `cargo deny check bans advisories licenses`, and neither runs offline. So a
database can move at (15). The per-tool commit in the report makes a move visible, and (15) prints both commits
before and after the replay. **Measure first, before the handler is written:**

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

**The order is fixed.** The registry lookup runs first, then ADR-0148's `test` refusal, then the route and input
checks, and **only then** is the step's bound resolved (§8c.6). So `rolling-deploy`'s `deploy` step refuses by its
bare verb name, as ADR-0135 clause 2 requires, and never by its key. That step's verb has no row, and its
`timeout_secs` is the `unmeasured` `${toolchain.deploy_timeout}`.
`tool_verb_routing::a_verb_with_no_row_refuses_by_its_bare_name_before_its_bound_is_resolved` pins the order.

⚠️ **On the shipped catalog, therefore, nothing new walks at `s8c`.** The three verb implementations are proved
over fixtures, through the port, by the adapter's own tests (§8c.9 (7)). The positive control that the executor
reaches `run_verb` is a kernel test over a test-only registry row that declares no inputs. **This is said so
that nobody reads a green Checkpoint 8c as *"smoke runs in the lane"*. It does not, until `s8d` and `s8g`.**

**The two gaps the draft named here are closed by D21-43.**

- `build` has an outcome (N25; §8c.2), so `build-package`'s `on_false = "fail"` arm is live.
- Feeds have a declared table (N14; below). `sca-scan`'s `sbom = "toolchain:sbom?"` resolves through
  `[feed.rust.sbom]`, which is declared `unprovided`, so the optional input is `null` (ADR-0032's Decision:
  *"Absent `?` inputs are `null`"*).

**The `setup` verb (D21-43 N25; D21-45 W3).** N25 reads: *"A per-toolchain **`setup` verb** at worktree creation
(`pnpm install --frozen-lockfile --prefer-offline`, refused by name on failure)"*. W3 reads: *"**Dependencies come
through N25's per-toolchain `setup` verb.** No `node_modules` symlinks"*. Record V fixes the verb, and this is the
proposal:

- **The row.** `setup` takes the `toolchain` route, outputs `SetupResult`, is `live = true`, and its timeout is
  `setup_timeout` (§8c.5). `s8d-ii` calls it through `run_verb` once per run worktree, at creation, with the resolved
  bound.
- **What it installs.** The project set is a declaration row, `setup_projects`, in the per-toolchain table (§8c.5).
  In this repository `[toolchain.rust]` lists one project, `crates/rk-adapter-toolchain-node/tests/fixtures/project`,
  which holds the only tracked `pnpm-lock.yaml` (`git ls-files`, re-measured at `a6349b6`). For each declared
  project, in order, the Rust handler runs `pnpm install --frozen-lockfile --prefer-offline --dir <project>` inside
  the worktree. `pnpm` is resolved through `resolved.toolchain_programs`, as `cargo` is.
- **Failure is refused by name.** A non-zero install is `Err`, naming `setup`, the project and the exit code. It is
  never `Ok` with a failed row, because a worktree with half its dependencies is no place for any later verb to run.
  An empty declared set is `Ok` with no projects.
- **The Node adapter** refuses `setup` by name at this stage, as it refuses every verb, and `[toolchain.node]` is all
  `unmeasured`. An install over a Node project's own root is owed with the first Node self-hosting run. It is not
  built here.

**The source tables: `[checklist.*]`, `[kb.*]` and `[feed.*]` (D21-43 N14).** N14 reads: *"**Declared
`[checklist.*]`, `[feed.*]` and `[kb.*]` tables in s8c's registry.** Undeclared keys are refused by name, and content
is authored only for the keys the lanes reach."* ADR-0040's grammar names the three forms, and design §4.5 puts
*"toolchain verbs and feeds"* in `registry.toml`. Record V fixes the shape, and this is the proposal.

*The entries.*

- **`[checklist.<id>]` and `[kb.<atom>]`** each declare exactly one of two things:
  - `path`, relative to `.redkiln/`: `checklists/<id>.toml` or `kb/<atom>.md`;
  - `unauthored = "<reason, naming the owner>"`.

  A checklist is a TOML file of `[[item]]` rows (`id`, `text`), plus a `source` that names what it was authored
  from. The `smoke` checklist's form (`argv`, `cwd`, `expect_exit`, one stdout predicate) is `s8g` §8g.6's, and only
  the `smoke` handler reads it.
- **`[feed.<toolchain>.<name>]`** declares exactly one of `verb = "<a registry verb>"` (the producing verb, whose
  output is the feed's value) or `unprovided = "<reason>"`.

*Resolution.* The resolver is `s8d`'s (N14; its S3). It reads these rules:

- An **undeclared** key is refused by name, even when the input is optional (`?`). An undeclared key is a typo or
  an unowned demand, never an absence.
- An `unauthored` or `unprovided` key is refused by name when the input is mandatory, and is `null` when it is
  optional (ADR-0032's Decision: *"Absent `?` inputs are `null`"*).

*Loading.* The loader refuses at load, naming the entry, in three cases:

- a `path` that does not exist or holds no item;
- a feed `verb` that is not a registry row;
- an entry that declares both forms, or neither.

The loader reads declared paths only. It does not walk `.redkiln/` recursively, so D21-44's exclusion of
`worktrees/` has no site in the loader. The `.worktreeinclude` reader below is this stage's one site for it.

**Which keys, measured** over the step `inputs` of the 102 files in `.redkiln/catalog/` at `a6349b6`. There are
**50** distinct keys:

- **27** `checklist:`;
- **12** `kb:`;
- **11** `toolchain:<feed>` names, in 12 spellings, because `vcs_history` occurs both bare and as
  `vcs_history?`.

The two lanes reach **14** of them. The lanes are read from `s8d`'s drafted walk tables, which list the fix lane's
and the story lane's selected activities. `rolling-deploy` is included until `s8g` replaces it.

| form | key | reached by (activity/step) | declared as |
| --- | --- | --- | --- |
| checklist | `closure` | `closure-record/audit` | `path`, authored here |
| checklist | `docs-diataxis` | `docs-update/write`, `docs-update/self-check` | `path`, authored here |
| checklist | `fix-scope` | `fix-spec/self-check` | `path`, authored here |
| checklist | `gherkin-quality` | `bdd-scenarios/self-check` | `path`, authored here |
| checklist | `retro` | `lightweight-retro/self-check` | `path`, authored here |
| checklist | `risk-register` | `t-shirt-sizing/self-check` | `path`, authored here |
| checklist | `smoke` | `smoke-test/smoke`, `rolling-deploy/smoke` | `unauthored`, naming `s8g` §8g.6, which replaces it with a `path` |
| kb | `spec-conventions` | `fix-spec/draft`, `bdd-scenarios/draft`, `bdd-scenarios/to-spec` | `path`, authored here |
| kb | `refactoring-catalog` | `tdd-red-green-refactor/refactor` | `path`, authored here |
| kb | `estimation-history` | `t-shirt-sizing/size` | `path`, authored here |
| kb | `retro-format` | `lightweight-retro/note` | `path`, authored here |
| feed | `backlog` | `closure-record/audit` (mandatory) | `unprovided` on both toolchains at this stage. Its only honest producer is a `store`-route read of the item tree, which `s8f` builds under D21-46 R7 and which flips both rows to `verb = "backlog"` (§8c.12) |
| feed | `logs?` | `root-cause/hypothesise` (optional) | `unprovided`, so `null` |
| feed | `sbom?` | `sca-scan/scan` (optional) | `unprovided`, so `null`. No SBOM tool is installed, and no record names one |

The other **36** keys are **not declared**, and each refuses by name wherever an activity reaches it. Content for
an activity that no lane selects would be a guess with no reader. Every authored file's `source` names its
provenance: the process-catalog proposal's passage for that activity, or the ADR it executes. Nothing is authored
from memory. `[feed.node.*]` declares the same three keys, each `unprovided`.

⚠️ **The enumeration rests on `s8d`'s lane-walk tests, and they run after this stage.** Three later inputs can move
this table, each by a dated line:

- `lane_walk::…` measuring a different selection;
- `s8g`'s `tag-release`;
- N23's five re-homed test-authoring activities.

The census test (§8c.9) counts the table, so a move cannot be silent.

**The `.worktreeinclude` reader (D21-45 W3).** W3 reads: *"**Honour the project's existing `.worktreeinclude`**,
the same file with the same semantics (ignored ∩ matched, symlinks skipped), for files like `.env`."* This stage
builds the reader. `s8d-ii` does the copy, at creation (D21-46 R1). The reader is
`rk_git::worktree_include::carry_set(exec, repo_root) -> Result<CarrySet, GitError>`, in a new
`crates/rk-git/src/worktree_include.rs`.

- **An absent file carries nothing.** Claude Code carries *"nothing by default"* (D21-45), so an absent
  `.worktreeinclude` gives the empty set, not an error. This repository has none (re-measured at `a6349b6`), so its
  carry is empty until one is committed.
- **Matched ∩ ignored, by git's own matcher.** Two reads go through `GitExec`, each with `-z`:
  1. `git ls-files -z --others --ignored --exclude-from=.worktreeinclude -- . ':(exclude).redkiln/worktrees'`
     lists the untracked paths the file matches. The exclude pathspec keeps another run's worktree, under the root
     (D21-44), from ever being walked or carried. `target/` is still walked. That costs time, and it carries only
     what the file matches.
  2. `git check-ignore -z --stdin --no-index`, over that list, keeps the ones the standard rules ignore. Its exit 1
     means *none*, not failure.

  A tracked file is never carried, because the checkout already holds it. **No second gitignore matcher is
  written.** Two matchers would give one pattern file two meanings. `GitOp` gains two read variants, `LsFiles` and
  `CheckIgnore`, whose `is_write()` is `false`, so ADR-0105 clause 5's write set is untouched.
- **Symlinks are skipped.** A path whose `symlink_metadata` is a symlink, or on Windows a junction or other reparse
  point, is skipped and NAMED in `CarrySet.skipped`. It is never followed.
- **Escapes are refused.** A listed path that is absolute, carries a prefix or a `..` component, or canonicalises
  outside the canonical repo root refuses the whole carry by name. The refusal is whole, not per path, because an
  escaping entry means the listing is not what the reader thinks it is.
- **A pure core.** The selection is `select_carry(matched, ignored, kinds) -> Result<CarrySet, CarryRefusal>` over
  lists already read. So the symlink and escape cases are tested without creating an OS symlink, which on Windows
  needs a privilege this box may not grant. The OS-level case is not asserted here, and §8c.11 says so.
- **`LC_ALL=C`** on these calls is W1's rule, which is `s8d-ii`'s. The reader goes through `GitExec` and inherits
  whatever `s8d-ii` sets there.

### 8c.5 The per-toolchain value table (D21-42 4a), and the measurement that is its warrant

`registry.toml` carries a table per toolchain, keyed by the `toolchain` facet's value (`[toolchain.rust]`,
`[toolchain.node]`). Each `${toolchain.<key>}` has **either** a value with its warrant **or** a declared
`unmeasured = "<reason>"`. Every value carries its warrant as structured fields: `secs`; `measured`, the list of
readings; `rule`, how the value follows from the readings; `pin`, the 40-hex commit of this repository the readings
were taken or read at; and exactly one of `command`, for a value measured here, or `source`, for a value cited from an
existing record (the agent-step keys, below). **The loader refuses a value without a warrant, and resolution refuses an `unmeasured` key or an
absent key by name** (§8c.6).

**Keeping the table out of the kernel (AC-487).** The kernel must not name an adapter:
*"`rk-kernel` contains no comparison against an adapter identity"*. So `rk-cli`, which already holds the name
`real_toolchain` chose, selects the table for that adapter's toolchain. It hands the kernel a flat
`ToolchainValues` (key → seconds, or key → `unmeasured`). The kernel never sees `rust`.

**What each key's warrant is, for `[toolchain.rust]`:**

| key | warrant |
| --- | --- |
| `test_timeout` (29) | **measured here** (below): the slowest child of a cold `test` call **on the runner the port selects** — after X15's install, `cargo nextest run` with its `--doc` companion, each child timed — n = 3 (*D21-48 (2026-09-24), lead:* a warrant measured on `cargo test` alone would bound a stand-in for what runs) |
| `build_timeout` (5) | **measured here**: a cold `cargo build --workspace`, n = 3 |
| `scan_timeout` (4) | **measured here**: the offline scan over the pinned database, n = 3 |
| `setup_timeout` (no catalog site; the `setup` row's own) | **measured here**: `setup`'s install over `setup_projects` in the fresh worktree, n = 3 |
| `implement_timeout` (27), `review_timeout` (4), `repro_timeout` (2) | these sit on **agent** steps, and the lanes reach all three [measured over `.redkiln/catalog/` at `fc80687`]. `implement_timeout` is on `tdd-red-green-refactor`'s `red`, `green` and `refactor`, `equivalence-boundary/write` and `docs-update/write`. `repro_timeout` is on `minimal-repro/attempt`, and `review_timeout` on `adversarial-review/review`. **So none of them may be `unmeasured`.** An `unmeasured` key refuses by name, and `s8d`'s walk would stop at the first of them. Each is **cited**: a dispatch-duration record for the key's activity class, with `source` naming it. AC-223 sets the floor: *"at least the recorded slow-condition figure for that class (its recorded maximum, not its median)"*. `review_timeout`'s candidate is AC-223's own evidence line, F31/S-TYPED *"CLEAN review median 1,071 s, slowest 4,561 s"*. **For `implement_timeout` and `repro_timeout` this file found no such record** (it searched `references/2026-09-08-evidence-brief.md` and AC-223's evidence line). *D21-48 (2026-09-24):* the operator took the value, X4: *"**4,800 s each, cited to F31/S-TYPED's slowest agent dispatch (4,561 s) as a borrowed class.** The record says so."* So each carries `secs = 4800`, `measured = [4561]`, a `source` naming F31/S-TYPED and the words *borrowed class*, and a `rule` naming the re-derivation below. This stage takes no free decision |
| `deploy_timeout` (11), `migration_timeout` (3), `cook_timeout` (5), `ue_test_timeout` (3), `mutation_timeout` (1) | `unmeasured`, each with its reason. No Phase-1 Rust verb deploys, migrates, cooks, runs Unreal automation or mutates |

**The borrowed class is re-derived once its own class is recorded (D21-48 X4; *D21-48 (2026-09-24)*).** X4 reads:
*"Every fixture-proof and real dispatch records its duration, and the record names the rule that re-derives each key
from its own class's recorded maximum once one exists."* Record V names the rule, and this is the proposal:

- **The record it reads.** A key's own class is the activity class its steps sit on: the implement class for
  `implement_timeout`, and `minimal-repro/attempt` for `repro_timeout`. The class's dispatch-duration record is the
  set of recorded wall durations of that class's dispatches, each with its run and its pin.
- **The rule.** Once such a record exists, the key is re-derived from that class's recorded maximum, by the same
  `rule` record V states for the measured keys (below), and never below AC-223's floor, *"its recorded maximum, not
  its median"*. The re-derivation is a dated record edit that replaces the `source` and drops *borrowed class*.
- **Until then 4,800 s stands.** It is never re-derived from a median, or from a class other than the key's own.
- **This stage dispatches nothing, so it records no duration.** The stages that run the fixture proofs and the real
  dispatches record each one's duration. §8c.12 reports that obligation to them.

**The table also admits declaration rows beside the timeouts.** A declaration row is a fact about the toolchain
that is not a duration, so it carries a `source` rather than a `measured` list. Three are expected:

- `setup_projects`, the project set §8c.4's `setup` installs. This stage writes it, with the `git ls-files`
  reading as its `source`.
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

**What "cold" must mean.** It is the condition the adapter meets in a run-owned worktree, which lives at
`.redkiln/worktrees/<item>` from `s8d-ii` on (D21-44; D21-46 R1). That condition has three parts:

- a fresh detached worktree;
- its own empty `CARGO_TARGET_DIR`;
- the Node fixture's `node_modules`, installed by the `setup` verb's own command, with the local pnpm store as it
  stands, which is what `--prefer-offline` meets. **`--prefer-offline` is not offline.** It uses the store where it
  can and fetches on a miss. So the setup reading records pnpm's `downloaded` count. The warrant labels a reading
  that fetched as a network reading, and it is not mixed silently with local ones.

The measurement's worktree sits in ADR-0137 clause 6's scratch location, not under `.redkiln/worktrees/`. Nothing
is dispatched into it, and the placement changes no timing. Its directory name is kept short (`c<HHMMSS>`), and the
script prints the path's length. A Windows path near 260 characters fails deep in `target/` for a reason that is not
timing (`core.longpaths` is unset on this box). Record V's evidence records this length beside the length of
`.redkiln/worktrees/<item>` for the lane's item. The Rust adapter
spawns three children per `test` call, **and the cold compile lands in the FIRST, the `--list` pass, not in the
run**, so each child is timed separately and the warrant is the slowest child. The Rust adapter's `timeout`
bounds each child it spawns (ADR-0140 decision 2), not the call. *D21-48 (2026-09-24), lead:* the measurement runs
after §8c.8 step 0's nextest install, through the port's own runner selection, so the children timed are the ones a
run spawns: the `--list` pass, the nextest run and the `cargo test --doc` companion. Record V's evidence names the
runner and its pinned version beside each child's times.

**A fresh worktree without `node_modules` runs this repository's suite red.** 10 Node tests fail at
`tests/it/support.rs`' preflight. The draft handed that on as unassigned. D21-43 N25 and D21-45 W3 settle it:

- the `setup` verb built here installs the dependencies;
- `s8d-ii` calls it at creation;
- `s8g` asserts that the verify pass's whole suite in a run worktree has its Node fixture dependencies.

The measurement below runs the same command, so it times the suite a run will meet.

```bash
cd D:/repos/redkiln-rs
# Run as a script. ONE cold pass; run it THREE times (n = 3) and record every MEASURED line of every pass.
# It adds one detached worktree and REMOVES it on every path; the path set is compared at the end.
# D21-47's wrapper, runbook/tools/nodispatch.sh, for every cargo, pnpm and scan child below: a COMPILED fake `claude`
# first on PATH that prints the pin, every other PATH directory holding a claude removed, REDKILN_LAUNCH_RECORD unset,
# and CLAUDE_CONFIG_DIR at an empty trap (this session's is the operator's real account). nd_setup runs the controls.
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh (run from the repo root)"; exit 1; }
nd_setup "$PWD/target/s8c-measure/nd" || exit 1
nd_enter || exit 1
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
base="$(git rev-parse --git-common-dir)/redkiln/tmp"
test -d "$base" || { echo "FAIL: $base absent — the scratch location ADR-0137 clause 6 names does not exist"; exit 1; }
pin=$(git rev-parse HEAD); rc=$?
[ "$rc" -eq 0 ] && [ -n "$pin" ] || { echo "FAIL: git rev-parse HEAD rc=$rc"; exit 1; }
mw="$base/c$(date -u +%H%M%S)"
out=$(git worktree add --detach "$mw" "$pin" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree add rc=$rc"; printf '%s\n' "$out"; exit 1; }
ts0=$(date +%s); out=$(pnpm install --frozen-lockfile --prefer-offline --dir "$mw/crates/rk-adapter-toolchain-node/tests/fixtures/project" 2>&1); rc=$?; ts1=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the setup verb's install rc=$rc (network on a cold pnpm store; local-only)"; printf '%s\n' "$out" | tail -10; git worktree remove --force "$mw"; exit 1; }
dl=$(printf '%s\n' "$out" | sed -n 's/.*downloaded \([0-9][0-9]*\).*/\1/p' | tail -1)
t0=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-test" cargo test --manifest-path "$mw/Cargo.toml" --workspace -- --list 2>&1); rc=$?; t1=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the cold --list child rc=$rc"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t2=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-test" cargo test --manifest-path "$mw/Cargo.toml" --workspace -- --list --ignored 2>&1); rc=$?; t3=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the --list --ignored child rc=$rc"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t4=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-test" cargo test --manifest-path "$mw/Cargo.toml" --workspace --no-fail-fast 2>&1); rc=$?; t5=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the run child is RED (rc=$rc) — a red suite's wall time is not the warrant for a green one"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t6=$(date +%s); out=$(CARGO_TARGET_DIR="$mw/target-build" cargo build --manifest-path "$mw/Cargo.toml" --workspace 2>&1); rc=$?; t7=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the cold build rc=$rc"; printf '%s\n' "$out" | tail -20; git worktree remove --force "$mw"; exit 1; }
t8=$(date +%s); out=$(cargo audit --file "$mw/Cargo.lock" --no-fetch --json 2>&1); rc=$?; t9=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the offline audit scan rc=$rc — no local advisory database, or advisories on this tree; read it"; printf '%s\n' "$out" | tail -10; git worktree remove --force "$mw"; exit 1; }
t10=$(date +%s); out=$(cargo deny --manifest-path "$mw/Cargo.toml" check --disable-fetch advisories 2>&1); rc=$?; t11=$(date +%s)
[ "$rc" -eq 0 ] || { echo "FAIL: the offline deny scan rc=$rc — no local advisory database, or advisories on this tree; read it"; printf '%s\n' "$out" | tail -10; git worktree remove --force "$mw"; exit 1; }
echo "MEASURED pin=$pin path_len=${#mw} setup=$((ts1-ts0)) setup_downloaded=${dl:-none-reported} list_cold=$((t1-t0)) list_ignored=$((t3-t2)) run=$((t5-t4)) build_cold=$((t7-t6)) scan=$((t9-t8+t11-t10)) (audit=$((t9-t8)) deny=$((t11-t10)))"
out=$(git worktree remove --force "$mw" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the measurement worktree did not remove (rc=$rc) — Halt 6 if a handle holds it"; printf '%s\n' "$out"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: the measurement changed the worktree PATH SET"; exit 1; }
nd_check "the measurement pass" || exit 1
```

⚠️ The verb runs both `cargo audit` and `cargo deny` (§8c.4), so the scan reading times both, and `scan` is their sum.
The flags are the proposal's (`--no-fetch`, `--disable-fetch`). If §8c.4's tool measurement changes either command,
take the reading again with the command the verb runs. A warrant timed on a command the verb does not run is a
warrant for the wrong thing.

`s8g`'s D21-42 10c says *"A cold `cargo test --workspace` wall time is measured first"*. This is that measurement,
and `s8g` cites it rather than repeating it.

### 8c.6 A step's `timeout_secs` is the tighter bound (D21-42 4b)

Record B fixes this, and this is the proposal:

- **`catalog::Step` gains `timeout_secs: Option<StepTimeout>`**, which is either `Literal(u64)` or
  `Toolchain(key)`. The loader reads it, where today *"The loader reads no `timeout_secs`"* (ADR-0140's evidence).
  `inputs` is **not** parsed here. ADR-0148's forbids bullet on `inputs` stands until `s8d`.
- The kernel resolves `Toolchain(key)` through the `ToolchainValues` that `rk-cli` supplies (§8c.5). It does so
  after the registry lookup and the route and input checks (§8c.4's order). **An absent
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
  `s8d`'s R1 supersedes decision 5 in part for per-step specs, and record B says so (§8c.7).

**Through this port, 4b reaches 113 of the 298 declarations.** D21-42 4b says *"the 298 declarations become
real"*, and that the bound is *"passed into the port call"*. The port there is the `Toolchain` port. **185
declarations sit on `agent` steps.** The Harness port is not the Toolchain port, and a per-step agent bound built by
overloading `ActivitySpec.timeout_secs` is forbidden (ADR-0140 decision 5). D21-43 N16 settles the other 185:
*"**Carried by each per-step spec** (3a), resolved through 4a's table and bounded by the activity budget. Owner:
s8d."* This stage gives the 33 substituted agent-step keys their **values** (§8c.5), and it gives the
`ToolchainValues` resolution that `s8d` calls. **Enforcing** them is `s8d`'s.

**`rk-cli`'s two docstrings must be corrected in the same commit.** They say what ADR-0140 decision 2 made them
say: `config.rs` and `run.rs` describe the construction timeout as the only bound. Record B's supersession is what
licenses the change (§8c.7).

**`RK-D-TIMEOUT` resolves `${toolchain.*}` through the same table (D21-42 4a).** Today the row reads
`[activity.<id>] timeout_secs` as a TOML value, and a string is reported *"which is not an integer (NonNumeric)"*
(`crates/rk-cli/src/doctor/rules.rs`). No shipped pack declares a substituted activity timeout: all 204 `timeout_secs`
in `.redkiln/packs/` are integer literals, and all 94 substitutions sit on catalog steps (both re-measured at
`a6349b6`). So a row that knows the table must read the steps too. Record V fixes the row, and this is the proposal:

- **A third arm.** A value of the exact form `${toolchain.<key>}`, in a pack activity table or on a catalog step, is
  resolved through the `ToolchainValues` that `run` would hand the kernel. The table is selected by the same
  detection (§8c.5), so doctor and run cannot disagree about a key.
  - A resolved key is numeric, and the row is silent.
  - An **absent** key is an `Error` finding naming `${toolchain.<key>}` (`Unresolved`).
  - An **`unmeasured`** key is an `Error` finding naming its declared reason. The rule table gives each rule one
    severity, and `RK-D-TIMEOUT` is `Severity::Error` in `rules.rs`. AC-330 reads *"doctor fails if no numeric
    `timeout_secs` is declared (an inherited default is a failure)"*, and an `unmeasured` key declares no number. So
    on this repository, `doctor` reports one `Error` for each of the 23 catalog steps whose key is `unmeasured`
    (`deploy`, `migration`, `cook`, `ue_test` and `mutation`). That is the true state of those activities. No `s8`
    checkpoint asserts a clean plain `doctor` over the shipped tree: at `fc80687` their one `doctor` run is
    `doctor --rebuild`, which returns before the rule table.
  - If no adapter detects, one `Error` finding says so, and no key is guessed.
- **What does not move.**
  - AC-419's two-case fixture still yields exactly two findings (`doctor::ac419_missing_or_non_numeric_timeout_rejected`,
    re-proven).
  - AC-330's p95 half stays unclaimed (§8c.10). §8c.5's n = 3 readings are a wall's first recording, not a p95.

### 8c.7 The records this stage writes

Each is **a new record, at the next free id at writing**. Its status is `proposed` until the operator's dated
ledger line flips it. It carries ADR-0164's form, including `### Option — <name>` headings (D21-42 24a) and a
**What this forbids** block. **Moving a `## Decision` sentence or a forbids bullet is a supersession in part,
never an erratum** (ADR-0092 rule 1). An accepted body is never edited.

| label | executes | supersedes IN PART — the sentence, quoted | stands, and is said to stand |
| --- | --- | --- | --- |
| **T** — the outcome and the selection | D21-42 1a, 4c (adapter side); D21-43 N4 (§8c.1's parser) and N25's `build` outcome (§8c.2); D21-48 X15 (nextest installed and supported, §8c.1); discharges RT-6, reverses RT-22, discharges R-9's transport and RT-26's OWED note against ADR-0094 decision 1 | **ADR-0162 decision 4**: *"`toolchain.test(selection)` is the boolean, because its contract is *"`Ok(count)` only on a green suite"* (RT-22) — so `is_err()` **is** "fails""*. **ADR-0094 decision 1**: *"`test` = `cargo test --workspace`"*, in three respects only: an `Affected` run's `-p` set, `--no-fail-fast` on every run child (§8c.1 rule 4), and, where nextest runs, the doctest companion child beside `cargo nextest run`, which runs no doctests (§8c.1; *D21-48 (2026-09-24)*, X15). Record T's evidence names the installed nextest version, which is the pin | ADR-0094 decision 1's *"(or `cargo nextest run` where present, recorded on the report)"*, which T executes and does not supersede (D21-48 X15); ADR-0162's four cells, the both-fail cell, the `--` selection rule and the ephemeral reference worktree; ADR-0094's whole-suite package set, and its *"`build` = `cargo build --workspace`"* (`--message-format=json` changes the stream that is read, not what is built); ADR-0148 whole (`s8d`'s) |
| **V** — the verb registry and the value table | D21-42 1b, 4a; D21-43 N14 (the source tables) and N25's `setup` verb; `RK-D-TIMEOUT`'s resolution (§8c.6); fills D21-10's gap. It owns the Rust verb handlers, and `s8g`'s R4 cites it | **ADR-0135 clause 1**: *"The map is closed and holds exactly four entries"*. **Clause 6**: *"The 29 unmapped verbs are recorded as an open gap, owned by no stage in this phase"*, for the five rows landed. The `## Decision` sentence *"It adds no method to the `Toolchain` port: the count stays **six**, as Checkpoint 7a asserts"*. The forbids bullet *"Forbids adding a port method to satisfy a catalog verb"*, **for the one generic method only**, together with that bullet's second sentence, *"The port's six are ADR-0003's and ADR-0094's, and Checkpoint 7a counts them"*: the count moves, and *"Checkpoint 7a counts them"* is measured false. **ADR-0162's forbids bullet** *"Forbids a seventh method on the `Toolchain` port, and forbids applying a mutation patch from `rk-cli`"*: **its seventh-method half only**, likewise. The patch half stands here, and `s8g`'s verify record may move it for the verify gate's declared AC mutations alone | ADR-0135 clause 2 (the bare-name refusal), clause 4 (`change_summary` is never synthesised) and clause 5 *(noted 2026-09-25, s8a fix round 2, ADR-0165: clauses 4 and 5 stand only as ADR-0165 left them. Decision 4 stands for the Node adapter, and for the Rust adapter's refusal of an empty or unresolvable range and of a range carrying a `Binary` or LFS pointer path. Decision 5 stands without its Checkpoint 5 grep sentences. Record V's what-stands cell cites ADR-0165 for both)* *(noted 2026-09-25, s8a fix round 8: ADR-0165 decision 9 was narrowed after round 2, and the restatement above is superseded by it. At round 5 decision 5's bold lead, *"**The kernel names `ChangeSummary` in no type position.**"*, and its clause *"without the type ever appearing in a signature or a binding in `rk-kernel`"* moved with the fourth forbids bullet, in part and only as far as a test double's impl signature under `tests/`. At round 6 decision 4's *"Both real adapters return … from a row dated `2026-09-18` …"* was superseded in part for the Rust adapter as a whole, and stands for the Node adapter only. Record V states what stands from ADR-0165 decision 9, not from this cell's round-2 wording)*, and its bullet *"Forbids reading the adapters' `Capability` enum as a verb table, or extending it with catalog verbs"*: **the verb table is `registry.toml`, and the five-token `Capability` enum is untouched**. Checkpoint 7c digests its four byte-equal bodies |
| **B** — the step bound | D21-42 4b; discharges ADR-0140 decision 4's OWED item for **tool** steps | *D21-48 (2026-09-24):* X13 took the breadth: *"**Record B supersedes, each in part and each quoted:** ADR-0140 decision 1, decision 2's sentence, decision 4, the first two forbids bullets, and ADR-0099 decision 2's *"under the same timeout"*."* Each moves for tool steps. **ADR-0140 decision 1**: its withdrawal of ADR-0137 clause 5's *"while a step's own `timeout_secs` remains the kernel's"*, which it says *"is **withdrawn as false**"*. For a tool step that clause is true again under 4b. **Decision 2's sentence**: *"It is not a backstop, because there is nothing behind it"*. **Decision 4**: *"The catalog's `timeout_secs` declarations are recorded as UNREAD"*. **The first two forbids bullets**: *"Forbids describing the construction-time `timeout` as a *backstop*, a *ceiling behind* another bound, or an *outer fence*"* and *"Forbids any prose, docstring or record asserting that a step's declared `timeout_secs` is applied, enforced or honoured by the kernel or by a toolchain adapter"*. **ADR-0099 decision 2**: *"the two Toolchain adapters run bounded commands under the same timeout"*, because a per-call step bound is not the same timeout. D21-42 4b listed decision 1 alone, and X13 settles the other sites | ADR-0140 decision 3 (the 3600 s default), decision 5, and its bullets on lowering the default and on `ActivitySpec`. Decision 5 stands **here**. `s8d`'s R1 supersedes it in part for per-step specs (*"A future per-step bound may not be built by overloading it"*, and its forbids bullet *"forbids repurposing it as one"*), and record B says so, so that the two records read as one. ADR-0099 decision 2's no-token half (*"without a token"*) stands |

**Errata and dated corrections, in the same work commit:**

- **ADR-0162's drivers**: *"Checkpoint 7a counts exactly six methods on the `Toolchain` port"*. This is measured
  false, and it is not a Decision sentence, so it takes an erratum at the next free `E-0162-<n>` (`E-0162-1` is
  taken). Its decision 4's *"a port Checkpoint 7a counts at six"* rides in record T's supersession of that
  decision. The mutate refusal stands on its remaining reason: *"no port method applies a patch"*.
- **ADR-0135's option 3**: *"a port whose method count Checkpoint 7a asserts at six"*. An option's rejection
  rationale is not a Decision sentence, so it takes an erratum at the next free `E-0135-<n>`. None is taken at
  `fc80687`. That bullet's forbids sentence moves with record V (above).
- **The code sentences that repeat the false count**, corrected in place with the date:
  - `rk-ports/src/toolchain.rs` (*"Checkpoint 7a counts exactly six on this port"*);
  - both adapters' crate docs (*"the six methods Checkpoint 7a counts and no seventh"*) and their *"RT-6: an
    inherent method; the port stays six methods"* lines;
  - both adapters' `report.rs` module docs (*"RT-6: the port stays six methods"*);
  - `review_pass.rs` and `tests/it/review_pass.rs`.
- **D21-10's** *"the count stays six, as Checkpoint 7a asserts"* takes a dated note under the operator's flip line.
  It does not take a rewrite.
- **`s7a` §7a.3**: a dated note that the `Toolchain` port has seven methods under record V, and that the exact
  count is asserted by `source_parse::toolchain_port_declares_exactly_seven_methods` and not by Checkpoint 7a's
  floor.
- **Design §3.3's `Toolchain` row** is the method-set authority: *"detect; `build`; `test(selection)`; `lint`;
  `classify(path) -> Text/Binary/Generated`; `change_summary(range) -> ChangeSummary`"*. It takes a dated note under
  the flip lines of records T and V. Under T, `test` takes a closed `Selection` and returns a `TestOutcome`, and
  `build` returns a `BuildReport`. Under B, every step-reached method takes a `StepBound`. Under V, a seventh method,
  `run_verb`, routes the registry's `toolchain` rows.
- **`BasisReason::PolicyForced`'s doc** (`rk-types`) takes record T's dated note, if T uses it for the adapter's
  rule (§8c.3 rule 3).
- **The record ids.** When each of T, V and B is written, its id is written into Checkpoint 8c (3)'s `REC_T`,
  `REC_V` or `REC_B` line by a dated edit, in the same work commit. The checkpoint locates each record by that id and
  by nothing else (D21-47).
- **AC-329**: a dated register note under the operator's flip line. It records that D21-43 N4's parser reads
  libtest's line protocol for `TestOutcome`'s `passed` and `failed`, which are not report members, and that
  *"no field of the report is obtained by parsing prose or markdown"* still holds for every member AC-329 lists.
  The note reads the criterion. It does not amend it.

The 7a "exactly six" erratum stands as drafted. `s8g`'s entry table, which the draft found checking the six-method
count, now reads the seven-method count after record V (*corrected in `s8g`'s own file; consistency pass
2026-09-24*). It is not made here.

### 8c.8 Order of work

Each slice is red first (golden constraint 12). A test is authored and watched fail **for the right reason**
before the code that turns it green, and a filter that selects nothing is not red.

0. **At stage open, install cargo-nextest (D21-48 X15; *D21-48 (2026-09-24)*).** Run `cargo install cargo-nextest
   --locked`, then read `cargo nextest --version` and record its line in record T's evidence. That version is the
   pin. The install is this stage's act and needs no further question: X15 reads *"The install is s8c's act, at stage
   open."* Capture the nextest goldens (§8c.1) with the libtest ones in step 1. Add the `ci.yml` install step, at this
   pin and before the `test` step, in the work commit.
1. **§8c.1's goldens**, captured on the pinned toolchain: they are the parser's pins. Then the three records,
   drafted `proposed`, with §8c.5's rule written into record V **before** §8c.5's readings.
2. **The port surface and every double, in one slice.** It carries 1a, `build`'s outcome (N25), `Selection`,
   `StepBound`, `run_verb` and `validate_report`. A port change that leaves one double on the old signature does not
   compile, so this slice cannot be split honestly. Three tracker rows move in this commit, per the maintenance rule:
   - `PROGRESS.md` §1's Checkpoint 7a row (`rk-ports`) is marked **re-run pending**;
   - its Checkpoint 5 row (`rk-types`) is marked **re-run pending**;
   - its Checkpoint 7c row (both toolchain crates) is marked **provisional**, and the whole re-run is `s8j`'s (D21-43
     N28: *"**s8j re-runs it whole**, from a plain terminal through the launcher, with the live probes."*).

   The named tests this stage moves in Checkpoints 7b, 7c and 7d's crates are re-proven by name in Checkpoint 8c
   (5)–(10).
3. **The Rust adapter**:
   - the parser over its goldens, and the red suite;
   - the runner probe, the nextest parser over its goldens, and the doctest companion (§8c.1; D21-48 X15);
   - `build`'s outcome;
   - `Affected` over the workspace fixture, the bound, and the recorded `affected-selection` probe;
   - the four verbs: `smoke`, `sca_scan`, `package` and `setup`.

   Then the Node adapter's `build` outcome, after its `tsc` exit-status measurement.
4. **`rk-git`**: the `.worktreeinclude` reader, and its two read `GitOp` variants.
5. **The kernel**:
   - `catalog::Step.timeout_secs` and `ToolchainValues`;
   - `VerbRegistry`, extended with the source tables;
   - `run_tool_step` through the registry, and the output decode table.
6. **`rk-cli`**:
   - the value table handed over;
   - `review_pass` reading `failed`;
   - the stub program;
   - `RK-D-TIMEOUT`'s third arm;
   - the docstrings.
7. **§8c.5's measurement** (n = 3). Then `.redkiln/registry.toml`, with its warrants and source tables, and the ten
   content files §8c.4 enumerates: six checklists and four atoms.
8. **The close.** The mutants (§8c.9), then Checkpoint 8c whole, then the adversarial rounds. The rounds continue
   **until one moves no code and no guard**.

### 8c.9 The named tests and the mutants

Every name is module-qualified and run with `--test it … --exact`, which is Checkpoint 8c's form. **New** means
authored red first in this stage. **Re-proven** means an existing test that this stage's change reaches and must
still pass for the right reason. **78** tests are new: 2 in `rk-types`, 6 in `rk-ports`, 39 in the Rust adapter, 4
in the Node adapter, 5 in `rk-git`, 19 in `rk-kernel` and 3 in `rk-cli`. That is the revision's 63, plus six added by
the D21-47 fix round, plus nine added by D21-48 X15 (*D21-48 (2026-09-24)*).

- **`rk-types`**
  - **New**:
    - `verb_outputs::every_verb_output_refuses_unknown_fields_and_has_no_default`
    - `verb_outputs::scan_report_carries_unrated_as_a_required_member`
  - **Re-proven**: `schema_snapshot::schema_snapshot`, `schema_no_oneof::no_oneof_at_any_node`,
    `verify_types::basis_reason_is_the_closed_six`, `verify_types::verify_report_has_no_default`.
  - The first new test walks all seven verb-output structs, `BuildReport` and `SetupResult` included.
- **`rk-ports`** (`--features conformance`)
  - **New**:
    - `fake_behaviour::toolchain_fake_red_suite_is_ok_with_failed_counts`
    - `fake_behaviour::test_outcome_is_passing_only_with_executed_tests_and_no_failures`
    - `fake_behaviour::toolchain_fake_red_build_is_ok_with_status_failing`
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
    - `report::a_red_test_in_an_early_binary_still_tallies_every_binary`. It runs over the red workspace golden's
      selection. It asserts that every later binary's block is tallied, and that `passed + failed` equals the
      `--list` count.
    - `libtest_lines::the_pinned_goldens_parse_to_their_recorded_counts`. It also asserts that
      `rust-toolchain.toml`'s channel is in `PINNED`.
    - `libtest_lines::a_record_line_outside_a_block_is_never_counted`
    - `libtest_lines::a_block_whose_record_count_differs_from_its_header_is_unverifiable`
    - `libtest_lines::a_summary_that_disagrees_with_its_records_is_unverifiable`
    - `libtest_lines::a_nonzero_exit_with_no_failed_record_is_unverifiable_never_green`
    - `libtest_lines::an_unpinned_toolchain_version_is_refused_by_name`
    - `libtest_lines::this_repositorys_whole_suite_stdout_parses_to_its_recorded_tally`, over the sixth golden
    - `libtest_lines::a_slow_test_notice_inside_a_block_is_skipped_never_refused`. A notice naming a test that the
      block never records still refuses.
    - `build::a_compile_error_is_ok_with_status_failing`
    - `build::a_build_stream_without_build_finished_is_err`
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
    - `verbs::setup_installs_each_declared_node_project_offline`
    - `verbs::a_failing_setup_is_refused_naming_the_project`
    - `verbs::an_empty_setup_set_is_ok_with_no_projects`
    - `verbs::an_undeclared_verb_is_refused_by_name`
    - *D21-48 (2026-09-24), X15:* the nine below. Each is red first, for the reason in brackets.
    - `nextest_lines::the_pinned_nextest_goldens_parse_to_their_recorded_counts` [red: no `nextest_lines` module and
      no golden under `tests/fixtures/nextest/`]. It also asserts that the installed `cargo nextest --version` is in
      `NEXTEST_PINNED`.
    - `nextest_lines::an_unpinned_nextest_version_is_refused_by_name` [red: no `NEXTEST_PINNED`, so nothing refuses]
    - `nextest_lines::a_summary_that_disagrees_with_its_status_lines_is_unverifiable` [red: no parser; later, M28's
      shape, a parser that trusts `Summary`]
    - `nextest_lines::a_nonzero_exit_with_no_fail_line_is_unverifiable_never_green` [red: no parser]
    - `nextest_lines::a_retry_or_timeout_line_is_unverifiable_never_counted` [red: no parser; later, M30's shape]
    - `runner::the_probe_finds_the_installed_nextest_and_names_its_version` [red: the adapter has no probe and always
      runs `cargo test`]
    - `runner::the_probe_falls_back_to_cargo_test_where_nextest_is_absent` [red: no probe]. Its `child_env` holds no
      `cargo-nextest` on `PATH` or under `CARGO_HOME`. If the toolchain's `cargo` cannot run under such an
      environment on this box, the test runs the probe over a stub `program` that answers as cargo does for an
      unknown subcommand, captured at stage open, and record T says which.
    - `report::the_nextest_path_records_its_runner_and_tallies_a_red_suite_as_ok` [red: no nextest path]. Under
      `Force(nextest)` over the fixture project red, `argv` names `nextest`, `runner.name` is `cargo-nextest` with
      the pinned version, and the outcome is `Ok` with `failed ≥ 1`.
    - `report::the_nextest_path_runs_doctests_beside_it_so_the_count_matches_list` [red: no companion child; later,
      M26's shape, which drops it and refuses on the count]. Over §8c.3's workspace, the companion's argv carries
      `--doc`, the doctest is tallied, and `passed + failed` equals the `--list` count.
  - **Re-proven**: `report::ac329_report_conforms_no_prose_parse`, `report::ac325_recorded_argv_is_os_vector`,
    `report::ac331_killed_verify_is_unverifiable`, `report::ac344_missing_binary_is_not_a_candidate_failure`,
    `conformance::port_conformance_real_row_rust`, `registry::ac328_affected_selection_unsupported_without_probe`,
    `registry::ac489_capability_registry_dated`, `report::concurrent_verify_reports_share_no_output_file` and
    `report::nextest_fallback_recorded_on_report` (each of the last two called the retired `verify_report`).
  - 🛑 **`ac328_…` KEEPS ITS NAME.** Checkpoint 7c's loop names it for both toolchain crates. Its body is
    re-pointed at a controlled probe-less registry, which still refuses, and the shipped `Recorded` row is proved
    by the `affected::` tests.
  - 🛑 **`nextest_fallback_recorded_on_report` KEEPS ITS NAME** (*D21-48 (2026-09-24)*). Its doc says nextest is
    absent on this box, and the stage-open install makes that false. Its body is re-pointed at the probe's absent
    case, a `child_env` without `cargo-nextest` as in the fallback test above. It asserts the recorded
    `cargo test --workspace` vector and `runner.name = cargo-test`. `report::ac329_report_conforms_no_prose_parse`
    and `report::ac325_recorded_argv_is_os_vector` pin the `cargo test` vector, so they construct with
    `Force(cargo-test)`. Checkpoint 7c names the first of these, and its description is reported under §8c.12's
    sibling notes.
- **`rk-adapter-toolchain-node`**
  - **New**:
    - `report::red_suite_is_ok_with_failed_counts`
    - `verbs::an_undeclared_verb_is_refused_by_name`
    - `bound::a_step_bound_tighter_than_construction_fires_first`
    - `build::a_type_error_is_ok_with_status_failing`
  - **Re-proven**: `report::ac329_report_conforms_no_prose_parse`, `report::ac331_killed_verify_is_unverifiable`,
    `report::ac344_missing_binary_is_not_a_candidate_failure`, `conformance::port_conformance_real_row_node`,
    `registry::ac328_affected_selection_unsupported_without_probe`, `cancel::s_cancel_win_vitest_arm`,
    `report::ac325_recorded_argv_is_os_vector`, `report::concurrent_verify_reports_share_no_output_file` and
    `report::d34_vitest_invoked_directly_never_pnpm_forwarded` (each of the last three called the retired
    `verify_report`).
  - **Re-proven, corpus-bound (`--features corpus`)**: `ac334::ac334_flip_predicate_real_toolchain`.
- **`rk-git`**
  - **New**:
    - `worktree_include::carries_only_files_both_ignored_and_matched`. It runs over a temp repository, so the git
      reads are real.
    - `worktree_include::an_absent_file_carries_nothing`
    - `worktree_include::a_symlink_is_skipped_and_named`. It runs over `select_carry`, the pure core.
    - `worktree_include::a_path_that_escapes_the_root_is_refused_by_name`. It runs over `select_carry`, the pure core.
    - `worktree_include::a_file_under_a_run_worktree_is_never_carried`. It plants a matched, ignored file under
      `.redkiln/worktrees/x/` in a temp repository, beside one at the root that is carried.
- **`rk-kernel`**
  - **New**:
    - `tool_verb_routing::a_verb_with_no_registry_row_is_refused_by_its_bare_name`
    - `tool_verb_routing::a_registry_verb_with_no_inputs_reaches_run_verb`
    - `tool_verb_routing::a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port`
    - `tool_verb_routing::a_row_whose_route_has_no_executor_refuses_naming_the_route`
    - `tool_verb_routing::an_output_of_the_wrong_kind_is_refused_naming_both_kinds`
    - `verb_registry::the_shipped_registry_loads_with_route_outputs_and_live_on_every_row`
    - `verb_registry::a_value_without_a_warrant_is_refused`
    - `verb_registry::ac018_holds_over_the_shipped_catalog_and_registry`. It runs `live_verb_memo_rule` over
      `.redkiln/catalog/` and `.redkiln/registry.toml`, asserts `Ok` and a non-zero walked-step count, and asserts
      `smoke`'s row is `live = true` (`s8b`'s hand-on).
    - `verb_registry::every_shipped_toolchain_substitution_resolves_or_refuses_by_name` — walks
      `.redkiln/catalog/` and **asserts the count it walked is non-zero and equals its own census**
    - `verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map`. `[toolchain.rust]`'s timeout
      keys are exactly §8c.5's map. `test_timeout`, `build_timeout`, `scan_timeout` and `setup_timeout` each carry
      `secs`, `measured`, `rule`, `pin` and `command`. `implement_timeout`, `review_timeout` and `repro_timeout` each
      carry `secs`, `measured`, `rule`, `pin` and `source`. The five `deploy`, `migration`, `cook`, `ue_test` and
      `mutation` keys are each `unmeasured`. No other timeout key exists. *D21-48 (2026-09-24), X4:* `implement_timeout` and `repro_timeout` each carry
      `secs = 4800`, `measured = [4561]`, a `source` naming F31/S-TYPED as a borrowed class and a `rule` naming the
      re-derivation (§8c.5); M31 kills it. On its own, "resolves or refuses by name"
      passes over an empty table, and this test does not.
    - `tool_verb_routing::a_verb_with_no_row_refuses_by_its_bare_name_before_its_bound_is_resolved`. Its step has an
      unrowed verb and an `unmeasured` `${toolchain.*}` bound, and the refusal names the verb.
    - `step_bound::a_literal_timeout_secs_reaches_the_port_call`
    - `step_bound::a_toolchain_substitution_resolves_through_the_value_table`
    - `step_bound::an_unresolved_toolchain_key_is_refused_by_name`
    - `step_bound::a_step_without_timeout_secs_passes_an_undeclared_bound`
    - `source_registry::the_catalog_source_census_is_fifty_with_fourteen_declared`. It walks `.redkiln/catalog/`,
      and **asserts the count it walked is non-zero and equals its own census**.
    - `source_registry::an_undeclared_checklist_kb_or_feed_key_is_refused_by_name`
    - `source_registry::an_unprovided_feed_is_null_when_optional_and_refused_when_mandatory`
    - `source_registry::a_malformed_entry_is_refused_at_load_naming_it`: a missing path, both forms, neither form,
      or a feed verb with no row
  - **Re-proven**:
    `tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port` (ADR-0148 stands),
    `memo::ac018_live_verb_may_not_memoise` (the extended parser), `verify::ac332_empty_selection_is_never_green`
    and `kernel_denylist::ac487_kernel_denylist` (no adapter name enters the kernel with the value table).
- **`rk-cli`**
  - **New**:
    - `review_pass::a_candidate_that_cannot_run_is_unverifiable_never_a_flip`
    - `adapter_selection::the_chosen_adapters_value_table_reaches_the_kernel_without_its_name`
    - `doctor::rk_d_timeout_resolves_a_toolchain_substitution_through_the_value_table`. Its fixture covers a
      resolved key, an `unmeasured` key and an absent key. The resolved key is silent, and each of the other two is
      one `Error` finding.
  - **Re-proven**: `review_pass::four_cells_over_two_arms_and_both_fail_is_not_a_flip`,
    `review_pass::mutate_and_selectionless_repros_produce_no_record`,
    `review_pass::selection_is_the_double_dash_tail_or_nothing`,
    `run::ac438_table_driven_exit_codes_over_fixtures_binary_arm` (the findings-block row still exits 4, through
    the updated stub) and `toolchain::a_tool_step_reaches_the_rust_adapter_and_not_the_fixture` (its `Step`
    literal gains `timeout_secs`), and `doctor::ac419_missing_or_non_numeric_timeout_rejected` (still exactly two
    findings).

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
| M11 | the parser's block framing is removed, so a record-grammar line anywhere on stdout counts | `libtest_lines::a_record_line_outside_a_block_is_never_counted` AND `report::passed_plus_failed_is_the_executed_count_and_survives_both_decoys` (the stdout decoy's phantom over-counts) |
| M12 | the summary line's counts are trusted in place of the tallied records | `libtest_lines::a_summary_that_disagrees_with_its_records_is_unverifiable` |
| M13 | a version absent from `PINNED` is parsed anyway | `libtest_lines::an_unpinned_toolchain_version_is_refused_by_name` |
| M14 | a non-zero exit with `failed == 0` is read as green | `libtest_lines::a_nonzero_exit_with_no_failed_record_is_unverifiable_never_green` |
| M15 | the Rust `build` maps a compile error back to `Err(NonZero)` | `build::a_compile_error_is_ok_with_status_failing` |
| M16 | the `setup` handler returns `Ok` over a failed install | `verbs::a_failing_setup_is_refused_naming_the_project` |
| M17 | the `.worktreeinclude` reader drops the ignored intersection, so a matched tracked-or-unignored file is carried | `worktree_include::carries_only_files_both_ignored_and_matched` |
| M18 | an undeclared optional source key resolves to `null` | `source_registry::an_undeclared_checklist_kb_or_feed_key_is_refused_by_name` |
| M19 | `RK-D-TIMEOUT` reads an absent `${toolchain.*}` key as numeric | `doctor::rk_d_timeout_resolves_a_toolchain_substitution_through_the_value_table` |
| M20 | the shipped `smoke` row is `live = false` | `verb_registry::ac018_holds_over_the_shipped_catalog_and_registry` |
| M21 | `RUN_ARGS` drops `--no-fail-fast` | `report::a_red_test_in_an_early_binary_still_tallies_every_binary` |
| M22 | `test_timeout` is deleted from the shipped `[toolchain.rust]` | `verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map` |
| M23 | the step's bound is resolved before the registry lookup | `tool_verb_routing::a_verb_with_no_row_refuses_by_its_bare_name_before_its_bound_is_resolved` |
| M24 | the reader drops the `:(exclude).redkiln/worktrees` pathspec | `worktree_include::a_file_under_a_run_worktree_is_never_carried` |
| M25 | the parser refuses libtest's slow-test notice | `libtest_lines::a_slow_test_notice_inside_a_block_is_skipped_never_refused` AND `libtest_lines::this_repositorys_whole_suite_stdout_parses_to_its_recorded_tally`, if the sixth golden holds a notice. Record which of the two it killed |
| M26 | the nextest path drops the doctest companion child (*D21-48 (2026-09-24)*, X15) | `report::the_nextest_path_runs_doctests_beside_it_so_the_count_matches_list` (the count invariant refuses) |
| M27 | the runner probe always answers `cargo test` | `runner::the_probe_finds_the_installed_nextest_and_names_its_version` |
| M28 | the nextest parser trusts the `Summary` line's counts in place of the tallied records | `nextest_lines::a_summary_that_disagrees_with_its_status_lines_is_unverifiable` |
| M29 | a nextest version absent from `NEXTEST_PINNED` is parsed anyway | `nextest_lines::an_unpinned_nextest_version_is_refused_by_name` |
| M30 | a `TRY` or `TIMEOUT` line is counted as a record | `nextest_lines::a_retry_or_timeout_line_is_unverifiable_never_counted` |
| M31 | the shipped `implement_timeout` is `secs = 3600` (*D21-48 (2026-09-24)*, X4 undone) | `verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map` |

**If a mutant survives, read it before strengthening anything.** A surviving mutant can mean that the property is
unobservable from the seam the test watches. If so, the guard moves; it is not weakened. Record every result, kills
and survivors alike, in the checkpoint output.

### 8c.10 Acceptance set

**Claimed:** AC-018, AC-324, AC-325, AC-326, AC-328, AC-329, AC-330, AC-331, AC-332, AC-344, AC-352, AC-487 and
AC-499.
Each is claimed only for the clause named:

- **AC-018**: **its literal only.** The shipped registry declares `live` on every row, `smoke`'s is `true`, and the
  rule holds over the shipped catalog and registry (the re-proof `s8b` hands on). After `s8b` §8b.5, no shipped step
  declares `memo`. So the rule would hold over the shipped catalog whatever the registry said, and the re-proof is
  insensitive to the registry. The explicit `live = true` assertion is what kills M20.
- **AC-326**: the adapter's bootstrap fallback only. The kernel's reclassification is not claimed.
- **AC-328**: its absent-probe half: Rust recorded, Node refused. Its stale half is excluded (below).
- **AC-329**: as a conformance row, and the parser's refusal of unparseable output (§8c.1). The dated register note
  is in §8c.7.
- **AC-330**: the declaration half only, as it reads a `${toolchain.*}` substitution through 4a's table (§8c.6).
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
| AC-330's p95 half | it compares a recorded whole-suite p95 wall with a declared timeout. §8c.5's n = 3 readings are that wall's first recording, not a p95 |
| AC-326's `selection_ineffective` | the kernel's, and unmoved |
| AC-328's stale half (*"or stale"*) | the `verified_on` staleness comparison is OWED (§8c.3 rule 1). So the Rust adapter can produce `basis: Affected` from a `Recorded` row whose staleness nothing checks |
| AC-289's mutate half | ADR-0162's refusal stands |

### 8c.11 Checkpoint 8c

- **The port has exactly seven methods.** `test` and `build` return outcomes, and a red suite or a red build is
  `Ok`. Every double compiles, and the `impl … for` count in `crates/rk-ports/src/` is still 5.
- **The counts come from the pinned parser.** A golden exists for the pinned channel. Unparseable output and an
  unpinned version are Unverifiable. Libtest's summary line is named on a code line only in the parser's module.
- **nextest is installed, pinned and supported** (D21-48 X15). The installed version has a golden, it is the version
  `ci.yml` installs before its `test` step, and record T's evidence names it. Both runners' paths are proved, the
  report records which ran, and the doctest companion is named on a code line.
- **Both real adapters and the fake pass `validate_report`** inside `conform_toolchain`.
- **A cannot-run candidate is `Unverifiable`, never a flip**, and the findings-block exit row still exits 4.
- **`.redkiln/registry.toml` ships.** Every row declares a route, `outputs` and `live`. Every `${toolchain.*}` in
  the catalog resolves or refuses by name. `[toolchain.rust]`'s timeout keys are exactly §8c.5's map: four measured,
  three cited and five `unmeasured`. Every value's `pin` is a commit, and every pin is read with `tomllib`.
  `implement_timeout` and `repro_timeout` are 4,800 s, cited to F31/S-TYPED as a borrowed class (D21-48 X4).
- **The source tables load.** They declare the 14 lane-reached keys, and every `path` names a non-empty file.
  Every feed declares one form. An undeclared key is refused by name.
- **`setup` installs the declared projects or refuses by name.** The `.worktreeinclude` reader carries only
  ignored ∩ matched paths, skips symlinks and refuses escapes.
- **The step bound is `min(step, construction)`.** An unresolved key is refused by name, and `RK-D-TIMEOUT`
  resolves a substitution through the same table.
- **The three records exist and move what §8c.7 says.** Each is located by the id recorded at write time
  (D21-47). ADR-0135's, ADR-0140's, ADR-0162's, ADR-0094's and ADR-0099's frontmatter carry the new
  `superseded_by`.
- **The whole suite is green** above the floor re-derived from Checkpoint 8b's recorded count (774 + 78 = 852: the
  final consistency pass's 843, plus D21-48 X15's nine), over at least 44 result lines, and fmt and clippy are rc 0.
- **Checkpoint 8b (the entry gate), Checkpoint 7a and Checkpoint 5 re-run whole and pass.** 7a and 5 are the two
  checkpoints this stage's own edits make provisional. No run here can reach a real dispatch. Each runs as
  `nodispatch env RK_CP_NESTED=1 bash …` under the shared wrapper (D21-46; D21-47), and under `RK_CP_NESTED=1` this
  block re-runs none of them.
- **The whole block runs under D21-47's wrapper**, `runbook/tools/nodispatch.sh`, set at (0b), and so do its own
  cargo runs: a compiled fake `claude` first on PATH that prints the pin, every other PATH directory holding a `claude`
  removed, `REDKILN_LAUNCH_RECORD` unset, and `CLAUDE_CONFIG_DIR` at an empty trap. `nd_setup`'s controls prove that
  the fake resolves and the trap starts empty. `nd_check` reads the trap empty and the fake uncalled after each cargo
  group and each replay, and at (16).

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

# ---- (0b) D21-47's WRAPPER, runbook/tools/nodispatch.sh, for this block's own cargo runs and every replay in (15). A
# ---- COMPILED fake `claude` stands first on PATH and prints the pin (a Rust Command::new("claude") on Windows resolves
# ---- only claude.exe, so a script fake would be passed by); every other PATH directory holding a claude is removed;
# ---- REDKILN_LAUNCH_RECORD is unset; CLAUDE_CONFIG_DIR is an empty trap. This session's CLAUDE_CONFIG_DIR is the
# ---- operator's real account. nd_setup runs the controls, and reuses an enclosing wrapper when this block is itself a
# ---- replay (ND_ACTIVE=1). nd_check reads the trap empty and the fake's call log absent after each cargo group, after
# ---- each replay, and at (16).
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh (run from the repo root)"; exit 1; }
nd_setup "$PWD/target/cp8c/nd" || exit 1
nd_enter || exit 1

# ---- (1) BUILD-OR-FAIL (Halt 1). A stale target/ is how this programme reached two false conclusions.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8c NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "(1) build" || exit 1

# ---- (2) PREFLIGHTS. A fresh tree fails 10 Node tests at tests/it/support.rs' preflight without node_modules.
test -f crates/rk-adapter-toolchain-rust/tests/fixtures/project/Cargo.toml || { echo "FAIL: the Rust fixture project is absent"; exit 1; }
test -f crates/rk-adapter-toolchain-rust/tests/fixtures/workspace/Cargo.toml || { echo "FAIL: the affected-selection fixture workspace is absent — the closure tests would assert over nothing"; exit 1; }
test -d crates/rk-adapter-toolchain-rust/tests/fixtures/advisory-db || { echo "FAIL: the fixture advisory database is absent — the scan tests would assert over nothing"; exit 1; }
chan=$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' rust-toolchain.toml); rc=$?
[ "$rc" -eq 0 ] && [ -n "$chan" ] || { echo "FAIL: rust-toolchain.toml names no channel (rc=$rc) — the libtest golden cannot be located"; exit 1; }
test -d "crates/rk-adapter-toolchain-rust/tests/fixtures/libtest/$chan" || { echo "FAIL: no libtest golden for the pinned $chan — the parser refuses every run on it by name (D21-43 N4)"; exit 1; }
# D21-48 X15: cargo-nextest was installed at stage open. The installed version is the pin: it has a golden, and it is
# the version ci.yml's gates job installs, in a step BEFORE its test step (the adapter's nextest-path tests run there).
nv_out=$(cargo nextest --version 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo nextest --version rc=$rc — the stage-open install did not happen (D21-48 X15)"; printf '%s\n' "$nv_out" | head -5; exit 1; }
nv=$(printf '%s\n' "$nv_out" | sed -n 's/^cargo-nextest \([0-9][0-9.]*\).*/\1/p' | head -1)
[ -n "$nv" ] || { echo "FAIL: cargo nextest --version printed no 'cargo-nextest <version>' line: $nv_out"; exit 1; }
test -d "crates/rk-adapter-toolchain-rust/tests/fixtures/nextest/$nv" || { echo "FAIL: no nextest golden for the installed $nv — the parser refuses every nextest run by name"; exit 1; }
ci='import re,sys; L=open(sys.argv[1],encoding="utf-8").read().split("\n"); code=[(i,l) for i,l in enumerate(L) if not l.lstrip().startswith("#")]; ins=[(i,m.group(1)) for i,l in code for m in [re.search(r"cargo install cargo-nextest --version (\S+) --locked",l)] if m]; tst=[i for i,l in code if re.fullmatch(r"\s*- name: test\s*",l)]; ok=len(ins)==1 and len(tst)==1 and ins[0][0]<tst[0]; print(ins[0][1] if ok else "BAD installs="+str(len(ins))+" tests="+str(len(tst)))'
printf '      - name: test\n        run: cargo test --workspace\n      - name: install cargo-nextest\n        run: cargo install cargo-nextest --version 9.9.9 --locked\n' > "$scratch/ctl-ci-late.yml" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(python -c "$ci" "$scratch/ctl-ci-late.yml"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = "BAD installs=1 tests=1" ] || { echo "FAIL: SYNTHETIC CONTROL — the ci.yml reader passed an install AFTER the test step ('$ctl', rc=$rc)"; exit 1; }
printf '      - name: install cargo-nextest\n        run: cargo install cargo-nextest --version 9.9.9 --locked\n      # cargo install cargo-nextest --version 1.1.1 --locked\n      - name: test\n' > "$scratch/ctl-ci-ok.yml" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(python -c "$ci" "$scratch/ctl-ci-ok.yml"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = "9.9.9" ] || { echo "FAIL: SYNTHETIC CONTROL — the ci.yml reader read '$ctl' over an ordered install and a commented decoy (rc=$rc)"; exit 1; }
civ=$(python -c "$ci" .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] && [ "$civ" = "$nv" ] || { echo "FAIL: ci.yml installs cargo-nextest '$civ', not the installed pin $nv, or not before its test step (rc=$rc; D21-48 X15)"; exit 1; }
test -e crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules/.bin/vitest || { echo "FAIL: run 'pnpm install --frozen-lockfile --prefer-offline --dir crates/rk-adapter-toolchain-node/tests/fixtures/project' ONCE, the setup verb's own command (network on a cold pnpm store; local-only)"; exit 1; }
PRED=target/predecessor-f21f55fc
test -d "$PRED/.git" || { echo "FAIL: the AC-334 predecessor checkout is absent at $PRED — s7c's preflight materialises it ONCE"; exit 1; }
d=$(git -C "$PRED" status --porcelain); [ -z "$d" ] || { echo "FAIL: $PRED is dirty — a previous ac334 run left the inversion applied"; exit 1; }

# ---- (3) THE THREE RECORDS, located by the id RECORDED AT WRITE TIME (D21-47), never by what they supersede. Two
# ---- records legitimately supersede one ADR: record B and s8d's R1 both touch ADR-0140, and Checkpoint 8d replays
# ---- this block. Each id is written into its line below by a dated edit when its record is written (§8c.7). Each
# ---- record's supersedes list must then CONTAIN its §8c.7 set, and each superseded ADR's superseded_by must name it.
REC_T=unrecorded
REC_V=unrecorded
REC_B=unrecorded
has='import re,sys; t=open(sys.argv[1],encoding="utf-8").read(); m=re.search(r"(?m)^supersedes:\s*\[(.*?)\]",t); got=set(re.findall(r"\"(\d{4})\"",m.group(1))) if m else set(); print(" ".join(sorted(set(sys.argv[2:])-got)))'
printf '%s\n' '---' 'supersedes: ["0140"]' '---' > "$scratch/ctl-adr.md" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(python -c "$has" "$scratch/ctl-adr.md" 0140 0099); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = "0099" ] || { echo "FAIL: SYNTHETIC CONTROL — the supersedes reader reported '$ctl' missing, not 0099 (rc=$rc)"; exit 1; }
for rec in "T:$REC_T:0162 0094" "V:$REC_V:0162 0135" "B:$REC_B:0140 0099"; do
  lab=${rec%%:*}; rest=${rec#*:}; id=${rest%%:*}; want=${rest#*:}
  case "$id" in [0-9][0-9][0-9][0-9]) ;; *) echo "FAIL: record $lab's id reads '$id' — write it into (3) by a dated edit when the record is written (D21-47)"; exit 1 ;; esac
  set -- references/adr/$id-*.md
  [ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: record $lab ($id) is not exactly one file under references/adr/"; exit 1; }
  f=$1
  miss=$(python -c "$has" "$f" $want); rc=$?
  [ "$rc" -eq 0 ] && [ -z "$miss" ] || { echo "FAIL: record $lab ($f) does not supersede ADR $miss (§8c.7; rc=$rc)"; exit 1; }
  for old in $want; do
    grep -qE "^superseded_by:.*\"$id\"" references/adr/$old-*.md || { echo "FAIL: ADR-$old's superseded_by does not name record $lab ($id)"; exit 1; }
  done
  grep -q '^### Option — ' "$f" || { echo "FAIL: $f has no '### Option — ' heading (D21-42 24a: from ADR-0164 on)"; exit 1; }
  grep -qF '**What this forbids**' "$f" || { echo "FAIL: $f carries no What-this-forbids block"; exit 1; }
  grep -qE '^status: (proposed|accepted)$' "$f" || { echo "FAIL: $f is neither proposed nor accepted"; exit 1; }
done
[ "$REC_T" != "$REC_V" ] && [ "$REC_T" != "$REC_B" ] && [ "$REC_V" != "$REC_B" ] || { echo "FAIL: two of records T, V and B share an id — §8c.7 writes three records"; exit 1; }
# D21-48 X15: record T's evidence names the installed nextest pin read at (2). The control proves the match can fail.
printf 'evidence: cargo-nextest 0.0.0-control\n' > "$scratch/ctl-rect.md" || { echo "FAIL: could not write the control"; exit 1; }
if grep -qF "cargo-nextest $nv" "$scratch/ctl-rect.md"; then echo "FAIL: SYNTHETIC CONTROL — the pin match passed a record naming another version"; exit 1; fi
grep -qF "cargo-nextest $nv" references/adr/$REC_T-*.md || { echo "FAIL: record T ($REC_T) does not name the installed pin 'cargo-nextest $nv' in its evidence (D21-48 X15)"; exit 1; }

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
run_named rk-ports "--features conformance" 7 conformance::port_conformance_fake_rows \
  fake_behaviour::toolchain_fake_red_suite_is_ok_with_failed_counts fake_behaviour::test_outcome_is_passing_only_with_executed_tests_and_no_failures \
  fake_behaviour::toolchain_fake_red_build_is_ok_with_status_failing \
  fake_behaviour::toolchain_fake_run_verb_refuses_an_undeclared_verb_by_name fake_behaviour::step_bound_zero_is_refused \
  source_parse::toolchain_port_declares_exactly_seven_methods
run_named rk-adapter-toolchain-rust "" 48 report::red_suite_is_ok_with_failed_counts report::a_build_failure_is_err_never_a_red_suite \
  report::a_red_test_in_an_early_binary_still_tallies_every_binary report::concurrent_verify_reports_share_no_output_file report::nextest_fallback_recorded_on_report \
  libtest_lines::this_repositorys_whole_suite_stdout_parses_to_its_recorded_tally libtest_lines::a_slow_test_notice_inside_a_block_is_skipped_never_refused \
  report::passed_plus_failed_is_the_executed_count_and_survives_both_decoys report::ac329_report_conforms_no_prose_parse \
  report::ac325_recorded_argv_is_os_vector report::ac331_killed_verify_is_unverifiable report::ac344_missing_binary_is_not_a_candidate_failure \
  conformance::port_conformance_real_row_rust bound::a_step_bound_tighter_than_construction_fires_first bound::the_construction_bound_caps_a_larger_step_bound \
  affected::affected_set_is_touched_crates_plus_reverse_dependency_closure affected::a_path_in_no_package_runs_the_whole_suite_with_a_reason \
  affected::no_recorded_whole_suite_runs_the_whole_suite_probe_unverified affected::a_package_set_that_lists_no_test_escalates_selection_empty \
  affected::a_named_selection_the_adapter_does_not_declare_is_refused_by_name registry::ac328_affected_selection_unsupported_without_probe \
  registry::ac489_capability_registry_dated verbs::smoke_runs_each_checklist_row_against_the_built_binary verbs::a_failing_smoke_row_is_a_failing_result_not_an_error \
  verbs::sca_scan_counts_an_unrated_advisory_as_unrated_never_zero verbs::sca_scan_records_the_advisory_database_commit \
  verbs::package_digests_only_the_declared_binaries_of_a_locked_release_build verbs::an_undeclared_verb_is_refused_by_name \
  libtest_lines::the_pinned_goldens_parse_to_their_recorded_counts libtest_lines::a_record_line_outside_a_block_is_never_counted \
  libtest_lines::a_block_whose_record_count_differs_from_its_header_is_unverifiable libtest_lines::a_summary_that_disagrees_with_its_records_is_unverifiable \
  libtest_lines::a_nonzero_exit_with_no_failed_record_is_unverifiable_never_green libtest_lines::an_unpinned_toolchain_version_is_refused_by_name \
  build::a_compile_error_is_ok_with_status_failing build::a_build_stream_without_build_finished_is_err \
  verbs::setup_installs_each_declared_node_project_offline verbs::a_failing_setup_is_refused_naming_the_project verbs::an_empty_setup_set_is_ok_with_no_projects \
  nextest_lines::the_pinned_nextest_goldens_parse_to_their_recorded_counts nextest_lines::an_unpinned_nextest_version_is_refused_by_name \
  nextest_lines::a_summary_that_disagrees_with_its_status_lines_is_unverifiable nextest_lines::a_nonzero_exit_with_no_fail_line_is_unverifiable_never_green \
  nextest_lines::a_retry_or_timeout_line_is_unverifiable_never_counted runner::the_probe_finds_the_installed_nextest_and_names_its_version \
  runner::the_probe_falls_back_to_cargo_test_where_nextest_is_absent report::the_nextest_path_records_its_runner_and_tallies_a_red_suite_as_ok \
  report::the_nextest_path_runs_doctests_beside_it_so_the_count_matches_list
run_named rk-adapter-toolchain-node "" 13 report::red_suite_is_ok_with_failed_counts report::ac329_report_conforms_no_prose_parse \
  report::ac325_recorded_argv_is_os_vector report::concurrent_verify_reports_share_no_output_file report::d34_vitest_invoked_directly_never_pnpm_forwarded \
  report::ac331_killed_verify_is_unverifiable report::ac344_missing_binary_is_not_a_candidate_failure conformance::port_conformance_real_row_node \
  registry::ac328_affected_selection_unsupported_without_probe verbs::an_undeclared_verb_is_refused_by_name \
  bound::a_step_bound_tighter_than_construction_fires_first cancel::s_cancel_win_vitest_arm build::a_type_error_is_ok_with_status_failing
run_named rk-adapter-toolchain-node "--features corpus" 1 ac334::ac334_flip_predicate_real_toolchain
run_named rk-git "" 5 worktree_include::carries_only_files_both_ignored_and_matched worktree_include::an_absent_file_carries_nothing \
  worktree_include::a_file_under_a_run_worktree_is_never_carried \
  worktree_include::a_symlink_is_skipped_and_named worktree_include::a_path_that_escapes_the_root_is_refused_by_name
run_named rk-kernel "" 23 tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port \
  verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map tool_verb_routing::a_verb_with_no_row_refuses_by_its_bare_name_before_its_bound_is_resolved \
  tool_verb_routing::a_verb_with_no_registry_row_is_refused_by_its_bare_name tool_verb_routing::a_registry_verb_with_no_inputs_reaches_run_verb \
  tool_verb_routing::a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port tool_verb_routing::a_row_whose_route_has_no_executor_refuses_naming_the_route \
  tool_verb_routing::an_output_of_the_wrong_kind_is_refused_naming_both_kinds verb_registry::the_shipped_registry_loads_with_route_outputs_and_live_on_every_row \
  verb_registry::a_value_without_a_warrant_is_refused verb_registry::every_shipped_toolchain_substitution_resolves_or_refuses_by_name \
  verb_registry::ac018_holds_over_the_shipped_catalog_and_registry \
  step_bound::a_literal_timeout_secs_reaches_the_port_call step_bound::a_toolchain_substitution_resolves_through_the_value_table \
  step_bound::an_unresolved_toolchain_key_is_refused_by_name step_bound::a_step_without_timeout_secs_passes_an_undeclared_bound \
  memo::ac018_live_verb_may_not_memoise verify::ac332_empty_selection_is_never_green kernel_denylist::ac487_kernel_denylist \
  source_registry::the_catalog_source_census_is_fifty_with_fourteen_declared source_registry::an_undeclared_checklist_kb_or_feed_key_is_refused_by_name \
  source_registry::an_unprovided_feed_is_null_when_optional_and_refused_when_mandatory source_registry::a_malformed_entry_is_refused_at_load_naming_it
run_named rk-cli "" 9 review_pass::four_cells_over_two_arms_and_both_fail_is_not_a_flip review_pass::a_candidate_that_cannot_run_is_unverifiable_never_a_flip \
  review_pass::mutate_and_selectionless_repros_produce_no_record review_pass::selection_is_the_double_dash_tail_or_nothing \
  run::ac438_table_driven_exit_codes_over_fixtures_binary_arm toolchain::a_tool_step_reaches_the_rust_adapter_and_not_the_fixture \
  adapter_selection::the_chosen_adapters_value_table_reaches_the_kernel_without_its_name \
  doctor::ac419_missing_or_non_numeric_timeout_rejected doctor::rk_d_timeout_resolves_a_toolchain_substitution_through_the_value_table
nd_check "(5)-(10) named selections" || exit 1
# The findings-block row's fixture must still expect 4: a table-driven test over a changed `expected` asserts a
# different row. `[:space:]`, as s7h (10a) reads it, because the driver parses it with `.trim().parse()`.
exp=$(tr -d '[:space:]' < crates/rk-cli/tests/fixtures/exit/findings-block/expected)
[ "$exp" = "4" ] || { echo "FAIL: findings-block/expected reads '$exp', not 4 — the stub's red arm no longer reaches four_cell(true, false)"; exit 1; }

# ---- (11) THE SHIPPED REGISTRY AND ITS WARRANTS. The behavioural assertions are (9)'s verb_registry::… tests; these
# ---- are floors. The file ships. [toolchain.rust]'s timeout keys are exactly §8c.5's map: four measured (command),
# ---- three cited (source) and five unmeasured, with no other timeout key. Declaration rows are admitted from their
# ---- named set only. Every warrant's pin is a commit in this repository's object store. Everything is read with
# ---- tomllib, because a grep misses the inline-table spelling. The control comes first.
test -f .redkiln/registry.toml || { echo "FAIL: .redkiln/registry.toml is not shipped — design §4.5's verb table is still D21-10's gap"; exit 1; }
km='import sys,tomllib,re; t=tomllib.load(open(sys.argv[1],"rb")).get("toolchain",{}).get("rust",{}); M={"test_timeout","build_timeout","scan_timeout","setup_timeout"}; C={"implement_timeout","review_timeout","repro_timeout"}; U={"deploy_timeout","migration_timeout","cook_timeout","ue_test_timeout","mutation_timeout"}; D={"setup_projects","package_binaries","claims"}; w=lambda v,last: isinstance(v,dict) and set(v)=={"secs","measured","rule","pin",last} and type(v["secs"]) is int and v["secs"]>0 and isinstance(v["measured"],list) and len(v["measured"])>0 and re.fullmatch("[0-9a-f]{40}",str(v["pin"])) is not None; bad=[k for k in sorted(M) if not w(t.get(k),"command")]+[k for k in sorted(C) if not w(t.get(k),"source")]+[k for k in sorted(U) if not (isinstance(t.get(k),dict) and set(t[k])=={"unmeasured"})]+[k for k in sorted(t) if k not in M|C|U|D]+([] if "setup_projects" in t else ["setup_projects"]); print(","+",".join(bad)+","); print(" ".join(sorted({v["pin"] for v in t.values() if isinstance(v,dict) and "pin" in v})))'
a40=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
printf '[toolchain.rust]\ntest_timeout = { secs = 60, measured = [30], rule = "r", pin = "%s", command = "c" }\nbuild_timeout = { unmeasured = "x" }\n' "$a40" > "$scratch/ctl-values.toml" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(python -c "$km" "$scratch/ctl-values.toml"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the key-map floor did not run over its control (rc=$rc)"; exit 1; }
c1=$(printf '%s\n' "$ctl" | sed -n 1p); c2=$(printf '%s\n' "$ctl" | sed -n 2p)
case "$c1" in *,build_timeout,*) ;; *) echo "FAIL: SYNTHETIC CONTROL — the key-map floor passed an unmeasured build_timeout ('$c1')"; exit 1 ;; esac
case "$c1" in *,test_timeout,*) echo "FAIL: SYNTHETIC CONTROL — the key-map floor could not read an inline-table warrant ('$c1')"; exit 1 ;; esac
[ "$c2" = "$a40" ] || { echo "FAIL: SYNTHETIC CONTROL — the pin reader read '$c2' from an inline table"; exit 1; }
got=$(python -c "$km" .redkiln/registry.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the key-map floor did not run over .redkiln/registry.toml (rc=$rc)"; exit 1; }
bad=$(printf '%s\n' "$got" | sed -n 1p); pins=$(printf '%s\n' "$got" | sed -n 2p)
[ "$bad" = ",," ] || { echo "FAIL: [toolchain.rust] departs from §8c.5's key map at: $bad"; exit 1; }
[ -n "$pins" ] || { echo "FAIL: no value in [toolchain.rust] carries a pin — a value without a warrant (D21-42 4a)"; exit 1; }
for p in $pins; do
  git cat-file -e "$p^{commit}" || { echo "FAIL: warrant pin $p is not a commit here — the reading cannot be re-taken at its pin"; exit 1; }
done
# D21-48 X4: implement_timeout and repro_timeout are 4,800 s each, cited to F31/S-TYPED's slowest dispatch (4,561 s) as a
# borrowed class, with a rule naming the re-derivation. The control comes first.
bx='import sys,tomllib; t=tomllib.load(open(sys.argv[1],"rb")).get("toolchain",{}).get("rust",{}); ok=lambda v: isinstance(v,dict) and v.get("secs")==4800 and v.get("measured")==[4561] and "F31" in str(v.get("source","")) and "borrowed class" in str(v.get("source","")) and "re-deriv" in str(v.get("rule","")); print(","+",".join(k for k in ("implement_timeout","repro_timeout") if not ok(t.get(k)))+",")'
printf '[toolchain.rust]\nimplement_timeout = { secs = 4800, measured = [4561], rule = "re-derived from the class maximum", pin = "%s", source = "F31/S-TYPED, borrowed class" }\nrepro_timeout = { secs = 3600, measured = [4561], rule = "re-derived from the class maximum", pin = "%s", source = "F31/S-TYPED, borrowed class" }\n' "$a40" "$a40" > "$scratch/ctl-borrowed.toml" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(python -c "$bx" "$scratch/ctl-borrowed.toml"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = ",repro_timeout," ] || { echo "FAIL: SYNTHETIC CONTROL — the borrowed-class floor read '$ctl' over a 3600 s repro_timeout (rc=$rc)"; exit 1; }
got=$(python -c "$bx" .redkiln/registry.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = ",," ] || { echo "FAIL: [toolchain.rust] departs from D21-48 X4 at '$got' (rc=$rc): 4800 s, measured [4561], an F31 borrowed-class source and a re-derivation rule"; exit 1; }
# The source tables (D21-43 N14). Each checklist/atom entry declares exactly one of `path` / `unauthored`, and a path
# names a NON-EMPTY file under .redkiln/. Each feed entry declares exactly one of `verb` / `unprovided`. The expected
# shape is 7 checklists + 4 atoms = 11, and 3 feeds x 2 toolchains = 6 (§8c.4). The control comes first.
st='import sys,tomllib,os; r=tomllib.load(open(sys.argv[1],"rb")); d=os.path.dirname(sys.argv[1]); es=[(f+"."+k,v) for f in ("checklist","kb") for k,v in r.get(f,{}).items()]; fs=[("feed."+c+"."+k,v) for c,tab in r.get("feed",{}).items() for k,v in tab.items()]; bad=[n for n,v in es if ("path" in v)==("unauthored" in v) or ("path" in v and not (os.path.isfile(os.path.join(d,v["path"])) and os.path.getsize(os.path.join(d,v["path"]))>0))]+[n for n,v in fs if ("verb" in v)==("unprovided" in v)]; print(len(es),len(fs),"["+" ".join(bad)+"]")'
printf '[checklist.ghost]\npath = "checklists/ghost.toml"\n[feed.rust.x]\nverb = "a"\nunprovided = "b"\n' > "$scratch/ctl-registry.toml" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(python -c "$st" "$scratch/ctl-registry.toml"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = "1 1 [checklist.ghost feed.rust.x]" ] || { echo "FAIL: SYNTHETIC CONTROL — the source-table floor read '$ctl' over a missing path and a two-form feed; it could not go red"; exit 1; }
got=$(python -c "$st" .redkiln/registry.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = "11 6 []" ] || { echo "FAIL: the source tables read '$got' (rc=$rc), expected '11 6 []' — a missing or empty content file, a two-form entry, or a moved enumeration without its dated line"; exit 1; }

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
# libtest's summary line is named on a code line ONLY in the one pinned parser (D21-43 N4). Anywhere else it is a
# second, unpinned reading (AC-329). Both halves of the filter are proved on synthetic lines first.
home='^crates/rk-adapter-toolchain-rust/src/libtest_lines\.rs:'
printf 'crates/rk-adapter-toolchain-rust/src/report.rs:9:    if l.starts_with("test result:") {\ncrates/rk-adapter-toolchain-rust/src/libtest_lines.rs:40:    const SUMMARY: &str = "test result: ";\n' > "$scratch/ctl-summary.txt" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(grep -vE "$cl" "$scratch/ctl-summary.txt" | grep -vE "$home")
[ "$ctl" = 'crates/rk-adapter-toolchain-rust/src/report.rs:9:    if l.starts_with("test result:") {' ] || { echo "FAIL: SYNTHETIC CONTROL — the outside-the-parser filter read '$ctl'"; exit 1; }
raw=$(grep -rnF 'test result:' crates/rk-adapter-toolchain-rust/src crates/rk-adapter-toolchain-node/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the AC-329 floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$cl" | grep -vE "$home" | grep .)
[ -z "$code" ] || { echo "FAIL: libtest's summary line is named on a code line outside the pinned parser (D21-43 N4; AC-329):"; printf '%s\n' "$code"; exit 1; }
inparser=$(printf '%s\n' "$raw" | grep -vE "$cl" | grep -E "$home")
[ -n "$inparser" ] || { echo "FAIL: the pinned parser names no summary line on a code line — the counts are not read where N4 says"; exit 1; }
grep -rqE '"--list"' crates/rk-adapter-toolchain-rust/src --include=*.rs || { echo "FAIL: the Rust adapter no longer names \"--list\" — Checkpoint 7c's RT-26 line would go red"; exit 1; }
# D21-48 X15: the nextest path's doctest companion. `cargo nextest run` runs no doctests, so "--doc" must be named on a
# CODE line of the Rust adapter, or the doctests drop out of a nextest run. Both halves of the filter are proved first.
printf 'crates/rk-adapter-toolchain-rust/src/report.rs:9:    // "--doc",\ncrates/rk-adapter-toolchain-rust/src/report.rs:10:    const DOC: &str = "--doc";\n' > "$scratch/ctl-doc.txt" || { echo "FAIL: could not write the control"; exit 1; }
ctl=$(grep -vE "$cl" "$scratch/ctl-doc.txt" | grep -F '"--doc"')
[ "$ctl" = 'crates/rk-adapter-toolchain-rust/src/report.rs:10:    const DOC: &str = "--doc";' ] || { echo "FAIL: SYNTHETIC CONTROL — the doc-companion filter read '$ctl'"; exit 1; }
raw=$(grep -rnF '"--doc"' crates/rk-adapter-toolchain-rust/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the doc-companion floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$cl" | grep .)
[ -n "$code" ] || { echo "FAIL: the Rust adapter names no \"--doc\" on a code line — a nextest run would drop the doctests (D21-48 X15)"; exit 1; }

# ---- (13) THE WHOLE SUITE, --no-fail-fast, rc 0, no FAILED line, and a passed FLOOR. Expected value, re-derived at
# ---- stage open from Checkpoint 8b's recorded count: 774 + 78 = 852 (774 = 740 at 992d257, ADR-0164's evidence, + s8a's
# ---- 9 + s8b's 25; 78 is §8c.9's new tests, 63 + the D21-47 fix round's 6 + D21-48 X15's 9). D21-48 (2026-09-24): this
# ---- supersedes the final consistency pass's 774 + 69 = 843, which superseded 770 + 69 = 839 (s8b's count taken as 21). Write the re-derived value
# ---- here, by a dated edit, before the first run. A floor, not a pin, and never lowered to make a red run green. At
# ---- least 44 result lines, as at 992d257.
# ---- This gate runs `cargo test` itself, so the runner the adapter selects (D21-48 X15) does not move it.
FLOOR=852
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .*FAILED|^test result:' | tail -30; exit 1; }
if printf '%s\n' "$out" | grep -qE '^test result: FAILED'; then echo "FAIL: a test binary reported FAILED under rc 0"; exit 1; fi
nres=$(printf '%s\n' "$out" | grep -c '^test result: ')
[ "$nres" -ge 44 ] || { echo "FAIL: $nres test result lines, below the 44 of 992d257 — a target stopped being built or run"; exit 1; }
total=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' | awk '{s+=$1} END {print s+0}')
[ "$total" -ge "$FLOOR" ] || { echo "FAIL: the whole suite passed $total tests, below the floor of $FLOOR"; exit 1; }
out=$(cargo test -p rk-ports --features conformance 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rk-ports under --features conformance rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "(13) whole suite" || exit 1

# ---- (14) fmt and clippy, the conformance feature included (ci.yml carries the same step).
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }
out=$(cargo clippy -p rk-ports --all-targets --features conformance 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy over rk-ports --features conformance rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }
nd_check "(14) fmt and clippy" || exit 1

# ---- (15) THE ENTRY GATE, AND THE OLDER CHECKPOINTS THIS STAGE'S OWN EDITS MAKE PROVISIONAL, RE-RUN WHOLE from their
# ---- own files' bytes: 8b (the entry gate), 7a (rk-ports: the seventh method, D21-42 1b) and 5 (rk-types: the new
# ---- structs). Nothing older is re-run from here. 8b runs with RK_CP_NESTED=1, so it skips its own re-run of 8a, and
# ---- 7c's whole re-run is s8j's (D21-43 N28). The extractor refuses a needle that selects zero or two blocks, and that
# ---- refusal is proved first. No literal triple backtick appears here: this file's own fence would close on it.
# ---- D21-46: SKIPPED under RK_CP_NESTED=1 (this block is another checkpoint's entry gate); otherwise every replay runs
# ---- as `nodispatch env RK_CP_NESTED=1 bash …` under (0b)'s wrapper, and nd_check follows it (D21-47).
xb='import re,sys; f=chr(96)*3; t=open(sys.argv[1],encoding="utf-8").read(); bs=[b for b in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in b]; len(bs)==1 or sys.exit("need exactly one bash block holding "+repr(sys.argv[2])+" in "+sys.argv[1]+"; found "+str(len(bs))); open(sys.argv[3],"w",encoding="utf-8",newline="\n").write(re.sub(r"(?m)^> ?","",bs[0]))'
out=$(python -c "$xb" runbook/phase-1/s7a-ports-and-fakes.md 'NO-SUCH-NEEDLE-8C' "$scratch/none.sh" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: SYNTHETIC CONTROL — the extractor accepted a needle that selects no block"; exit 1; }
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8C (15) SKIPPED: RK_CP_NESTED=1 — this block is another checkpoint's entry gate; its re-runs ran at Checkpoint 8c's own close"
else
# The wrapper is (0b)'s: nd_check follows each replay and closes the block at (16). The card and permission precondition
# below sits inside this non-nested branch (D21-47).
# NO ACCIDENTAL DISPATCH. If 8b's chain still reaches Checkpoint 7f through 8a, 7f runs `redkiln run` on the fix item.
# Before s8h no shipped card carries permission_mode, so lint_card refuses before any spawn. That is proved here, with
# a control. From s8h on it is false: a re-run from then points [adapters] harness_binary at
# target/debug/rk-fixture-child-claude, the canonical fixture harness (D21-47).
printf '{"permission_mode": "acceptEdits"}\n' > "$scratch/ctl-card.json" || { echo "FAIL: could not write the control card"; exit 1; }
ctl=$(grep -l 'permission_mode' "$scratch/ctl-card.json"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ctl" ] || { echo "FAIL: SYNTHETIC CONTROL — the card guard cannot see permission_mode (rc=$rc)"; exit 1; }
set -- .redkiln/cards/*.json
[ "$#" -eq 8 ] && [ -f "$1" ] || { echo "FAIL: expected 8 shipped cards, found $#"; exit 1; }
pm=$(grep -l 'permission_mode' "$@"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: a card carries permission_mode (rc=$rc: $pm) — a whole re-run reaching redkiln run could dispatch for real; stop (D21-40 decision 2)"; exit 1; }
out=$(python -c "$xb" runbook/phase-1/s8b-catalog-quality.md 'CP8B-END-OF-BLOCK reached' "$scratch/cp8b.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8b's block could not be extracted: $out"; exit 1; }
out=$(nodispatch env RK_CP_NESTED=1 bash "$scratch/cp8b.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the entry gate, Checkpoint 8b, re-run WHOLE, exited $rc (read its FAIL line: record ids are literals in its own block, and none is supplied from here)"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP8B-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8b exited 0 without reaching its last line"; exit 1; }
nd_check "Checkpoint 8b replay" || exit 1
out=$(python -c "$xb" runbook/phase-1/s7a-ports-and-fakes.md 'trybuild_pins_the_reason_each_port_fixture_fails' "$scratch/cp7a.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a's block could not be extracted: $out"; exit 1; }
printf '\necho "CP7A-END-OF-BLOCK reached"\n' >> "$scratch/cp7a.sh" || { echo "FAIL: could not append 7a's end marker"; exit 1; }
out=$(nodispatch env RK_CP_NESTED=1 bash "$scratch/cp7a.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a, re-run WHOLE over the seven-method port (D21-42 1b), exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP7A-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7a exited 0 without reaching its last line"; exit 1; }
nd_check "Checkpoint 7a replay" || exit 1
# Checkpoint 5's cargo audit and cargo deny lines FETCH their advisory databases. Record both databases' commits
# around the replay, so that a moved database is a recorded fact and not a silent second instrument (§8c.4).
dbs() { for d in "$HOME/.cargo/advisory-db" "$HOME"/.cargo/advisory-dbs/*/; do [ -d "$d" ] || continue; h=$(git -C "$d" rev-parse HEAD 2>/dev/null); printf '%s=%s ' "$d" "${h:-none}"; done; }
db_before=$(dbs)
out=$(python -c "$xb" runbook/phase-1/s5-workspace-ci-rk-types.md 'FAIL: deny.toml absent' "$scratch/cp5.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 5's block could not be extracted: $out"; exit 1; }
printf '\necho "CP5-END-OF-BLOCK reached"\n' >> "$scratch/cp5.sh" || { echo "FAIL: could not append 5's end marker"; exit 1; }
out=$(nodispatch env RK_CP_NESTED=1 bash "$scratch/cp5.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 5, re-run WHOLE over the new rk-types structs, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP5-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 5 exited 0 without reaching its last line"; exit 1; }
nd_check "Checkpoint 5 replay" || exit 1
echo "ADVISORY-DBS before CP5: $db_before"
echo "ADVISORY-DBS after CP5:  $(dbs)"
fi

# ---- (16) THE TRAP IS EMPTY AND THE FAKE UNCALLED (D21-47), and THE WORKTREE PATH SET IS UNCHANGED, compared
# ---- BEFORE any prune; the prune is a dry run and finds nothing.
nd_check "(16) end" || exit 1
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }
rm -rf "$scratch"

echo "CP8C-END-OF-BLOCK reached"
```

**BEFORE assertions (PIA-035), measured at `ca8f387` with this block's own commands and re-read at `a6349b6`.**
On the untouched tree these guards are RED, as a guard that can pass only after the work must be:

- (4)'s counter reads `6` over `crates/rk-ports/src/toolchain.rs`.
- (12)'s `review_pass` floor finds the code line
  `let outcome = four_cell(candidate.is_err(), reference.is_err());`.
- (12)'s parser-home floor finds no `test result:` code line: neither adapter's `src/` names one, and
  `libtest_lines.rs` does not exist.
- (2)'s golden preflight and (11)'s registry lines fail, because neither the golden directory nor
  `.redkiln/registry.toml` exists.
- (3) fails: `REC_T` reads `unrecorded` until record T is written and its id is recorded.
- (2)'s nextest preflight fails before the stage-open install: `cargo nextest` is absent on this box (`s7c`, measured
  2026-09-17), and `ci.yml` installs no nextest. *(D21-48 (2026-09-24).)*

⚠️ **What this block cannot see, said rather than claimed away:**

- **(4)'s parser** drops `//` lines only. A method declared inside a `/* … */` block comment, or produced by a
  macro, is miscounted. The named test `source_parse::toolchain_port_declares_exactly_seven_methods` is what makes
  the pair sound.
- **The fake `claude` is compiled, and it is still not a proof on its own.** *(Final consistency pass 2026-09-24:
  this replaces "the fake is a `sh` script".)* `runbook/tools/nodispatch.sh` builds it as `claude.exe`, so a Rust
  `Command::new("claude")` resolves it, and it removes every other `PATH` directory holding a `claude`. A process that
  builds its own `PATH`, or names a real binary by its absolute path, passes both by. The trap `CLAUDE_CONFIG_DIR`,
  read empty by `nd_check`, is what would catch such a spawn, and only if the spawn inherits the environment.
- **(11)** proves that a pin is a commit. It does not prove that the reading was taken there. The warrant's
  `measured` list and record V's evidence are that proof, and the operator reads them.
- **(15)** runs Checkpoint 8b's bytes **as they stand at this stage's close**. If `s8b`'s block names a variable
  (as Checkpoint 7i named `FIX_ITEM`), it refuses by name, and the operator sets it. The block does not guess it.
  Record ids and commit shas are never such a variable. Under D21-47 each block carries its own as literals, written
  by dated edit, so no re-run supplies one (final consistency pass 2026-09-24).
- **The libtest goldens** prove the parser against one version's captured output. They do not prove it against a
  test that writes to an inherited stdout mid-block. Such a test refuses (§8c.1, rule 2), which is fail-closed. It
  is not proved absent.
- **(2) compares `ci.yml`'s nextest pin with this box's install** (*D21-48 (2026-09-24)*). It does not see what CI
  actually installed. CI's log of its `install cargo-nextest` step is that evidence, read at the first CI run after
  this stage. The nextest goldens prove the parser against the one pinned version only, as the libtest ones do.
- **The `.worktreeinclude` reader's OS-level symlink case** is not exercised. The pure core carries the symlink and
  escape tests (§8c.4). Creating a symlink on this Windows box needs a privilege it may not grant.
- **(11)'s `11 6 []`** is the shape of the enumeration, not its truth. That the 14 keys are the ones the lanes reach
  rests on `s8d`'s lane-walk tests, which run later (§8c.4).

### 8c.12 Stage close, and what is handed on

The stage closes **in two commits** (`00-preamble.md`):

1. the work and this checkpoint's output, including §8c.1's goldens, §8c.5's measurement and every mutant result;
2. the tracker commit that writes that hash into `PROGRESS.md` §1, with Checkpoint 7c's row left provisional for
   `s8j` (§8c.8).

The three records stay `proposed` until the operator's dated ledger line. **Re-run Checkpoint 8c after the flip
commit.** A pass is evidence about a specific text.

**Settled since the draft, each by the entry cited. These are no longer handed on:**

| draft item | settled by | where it now lives |
| --- | --- | --- |
| where the 185 agent-step `timeout_secs` are enforced | D21-43 N16 | `s8d` (§8c.6) |
| a run-owned worktree runs this repository's suite red, with no `node_modules` | D21-43 N25; D21-45 W3 | the `setup` verb here; `s8d-ii` calls it at creation; `s8g` asserts the Node deps (§8c.5) |
| `build` has no outcome | D21-43 N25 | here (§8c.2) |
| `[toolchain.feeds]` is not shipped | D21-43 N14 | the `[feed.*]` table here (§8c.4) |
| `sca-scan`'s check passes with `unrated > 0` | D21-43 N25 | `s8b`'s catalog edit |
| `RK-D-TIMEOUT` resolves no `${toolchain.*}` value | the D21-43 revision brief's assignment | here (§8c.6) |
| two records claim the Rust verbs' behaviour | D21-43's integration call | V owns the handlers, and `s8g`'s R4 the activities, citing V (§8c.4) |
| AC-018's `live_verb_memo_rule` has a shipped registry and nothing runs it over `.redkiln/` | D21-43 N26 (`s8b` repairs the 14 activities and hands on the re-proof) | here: `verb_registry::ac018_holds_over_the_shipped_catalog_and_registry` (§8c.4) |
| `toolchain:backlog` has no producer | D21-46 R7: *"s8f builds the `toolchain:backlog` producer (a store-route read of items)"* | `s8f` §8f.7 builds it and flips `[feed.rust.backlog]` and `[feed.node.backlog]` to `verb = "backlog"`. Between Checkpoint 8c and Checkpoint 8f, `closure-record` refuses by name at `audit`, and `s8d` §8d.9's walk table counts it among the refusals (§8c.4) |
| record B's supersession reaches past the ledger's text (the revision's handed-on item 1) | D21-48 X13: *"**Record B supersedes, each in part and each quoted:** ADR-0140 decision 1, decision 2's sentence, decision 4, the first two forbids bullets, and ADR-0099 decision 2's *"under the same timeout"*."* | record B (§8c.7) |
| `implement_timeout` and `repro_timeout` have no dispatch-duration record (the revision's handed-on item 3) | D21-48 X4: *"**4,800 s each, cited to F31/S-TYPED's slowest agent dispatch (4,561 s) as a borrowed class.**"* | record V and `[toolchain.rust]`, with the re-derivation rule (§8c.5) |
| installing cargo-nextest was an operator question (§8c.1) | D21-48 X15: *"**Install it and support it now.**"* | here, at stage open, and in `ci.yml` (§8c.1, §8c.8) |
| the owner of D21-45 W3's `.worktreeinclude` carry | D21-48 X14: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at creation."* | the reader here (§8c.4); the call at creation is `s8d-ii`'s |

**Handed on, unassigned. Each is reported here, not decided here:**

1. **`"e2e"` is refused.** If `s8j`'s story candidate selects `walking-skeleton`, the declaration becomes owed there.

*D21-48 (2026-09-24):* the revision's handed-on items 1 and 3, record B's breadth and the two agent-step keys, are settled by X13
and X4. They now sit in the table above.

The draft's third item, `toolchain:backlog`'s producer, is settled by D21-46 R7 and now sits in the table above.

**Sibling files that disagree with this one:**

- **`s8g`'s entry table** checked the six-method count in the draft. After record V the port has seven. *Settled:*
  `s8g`'s revised entry table reads *"the **seven**-method count after `s8c`'s record V"*.
- **`s8g` §8g.6 does not flip `[checklist.smoke]`.** This stage declares it `unauthored`, naming `s8g` §8g.6 as the
  stage that replaces it with `path = "checklists/smoke.toml"`. At `fc80687` `s8g` authors the checklist file, and
  its (7) checks that the file exists. No line of `s8g` edits the registry entry, so `checklist:smoke` would still
  refuse by name after `s8g`. The flip, and a checkpoint line asserting it, belong in `s8g` §8g.6 and its
  checkpoint. This file cannot make them, and it reports the gap.
  *Final consistency pass 2026-09-24:* the gap was still open in `s8g`'s bytes when this file's pass closed. The
  final pass assigns the flip, and its checkpoint line, to `s8g` §8g.6. Until `s8g`'s dated line lands,
  `checklist:smoke` refuses by name, and `s8d` §8d.9's `smoke-test` row refuses with it. *Settled (final consistency pass 2026-09-24, lead):* `s8g` §8g.6's R4 commit sets `path = "checklists/smoke.toml"`, and Checkpoint 8g (7h) asserts it with a control and mutant M26.
- **The rigged stub's doc** (`rk-fixture-toolchain.rs`) names the candidate arm's directory as
  *"`…/worktrees/<item>`"*. Under D21-44 the run worktree moves to `.redkiln/worktrees/<item>`. The stub decides by
  the `.review-ref` suffix, so its behaviour does not move. The doc sentence is corrected with the path move, which
  is `s8d-ii`'s (D21-46 R1; `s8d-ii`'s attendant edits list it).
- *D21-48 (2026-09-24):* **`s7c`'s Checkpoint 7c line for `nextest_fallback_recorded_on_report`** describes it as
  *"nextest ABSENT on this box (measured 2026-09-17)"*, and `s7c` §7c.22 holds the presence arm ARMED. After this
  stage's install both are stale. The test keeps its name over a controlled absent probe, and the presence arm is
  proved by `runner::the_probe_finds_the_installed_nextest_and_names_its_version`. Checkpoint 7c's whole re-run is
  `s8j`'s, and `s8j` reads the line's description against this.
- *D21-48 (2026-09-24):* **The verify pass and the lanes call `Toolchain::test`, and on this box it now runs nextest**
  plus the doctest companion. `s8g`'s verify pass and its `verify_completed` row, `s8d`'s `red-check` read and
  `s8j`'s runs select tests through the port, so their argv readings name `nextest`. Their counts do not move,
  because §8c.2's invariant holds on both paths. Whether `verify_completed` journals `runner` is `s8g`'s.
- *D21-48 (2026-09-24):* **X4's duration records** are owed by the stages that run the fixture proofs and the real
  dispatches. Each records its wall duration by activity class, and record V's rule re-derives `implement_timeout`
  and `repro_timeout` from them (§8c.5).
- *D21-48 (2026-09-24):* **The floor chain moves by nine.** Checkpoint 8c's expected count is 852, not 843, so every
  later stage's floor re-derived from it moves by nine.

---
