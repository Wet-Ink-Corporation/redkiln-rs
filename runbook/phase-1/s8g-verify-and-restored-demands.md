# Step 8g — The verify record and the demands restored (Phase 1, part 8, sub-stage g)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8g` | Phase 1 | Checkpoint 8f | Checkpoint 8g | `s8f` | `s8h` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (HEAD `a6349b6`, against `spikes/DECISIONS-2026-09-21.md`'s three entries).
What changed:

- **N23**: the displaced `TestReport` providers are re-homed to the implement stage as test-authoring activities, in
  R1, and the test-design escalation row moves with them. They are **four, measured**, not the five the draft named
  (§8g.2).
- **N24**: AC-139's default is recorded as block, in R2. Both shipped packs ship `verification_required = ["verify"]`
  (§8g.3).
- **Ownership** (D21-43's integration calls): `.redkiln/questions.toml` is dropped. `verify.ac-discriminates` goes
  into `s8d`'s `.redkiln/processes/` registry (§8g.6). AC-133's registry stays here, and `s8h` consumes it (§8g.3).
  R4 cites `s8c`'s record V for the three verb handlers, so the eight handler tests have left this file and its four
  handler mutants are replaced by activity-level ones (§8g.6, §8g.9).
- **The entry table** reads the seven-method count after record V. It gains N25's `setup` verb and widened `build`,
  `s8d`'s loader and `s8d-ii`'s run-worktree root (D21-44; D21-46 R1 moved it out of `s8e`).
- **N25**: the `sca-scan` edit (`unrated == 0`) is `s8b`'s catalog edit, and `setup_projects` is `s8c`'s row. Both
  are cited, not re-specified. The pass's whole suite meets a tree `setup` has prepared, including the Node
  fixture's dependencies, and a test asserts it (§8g.2, §8g.6).
- **Owners named**: the `release_tag` row name follows `s8c`'s registry (§8g.7). AC-114's `curation.decision` lint is
  `s8j`'s (§8g.7). AC-341's CI-green half gets a dated register note (§8g.5).
- **D21-44**: the run-owned worktree is `.redkiln/worktrees/<item>`, and the mutation tree never sits inside it
  (§8g.5).
- **The checkpoint**: the floor is re-derived from Checkpoint 8f's recorded count. The replays are bounded by
  `RK_CP_NESTED`, and none may reach a real `claude`. Checkpoint 8b joins the replays, because the re-homing moves its
  pinned lattice. The cold figure is `s8c`'s, cited, and it is the slowest child (§8g.2, §8g.10).
- **New findings**: `smoke-test` has no check, and R4 decides it (§8g.6). `integration-tests` and `migration-dry-run`
  keep agent steps at verify, and that residual is named (§8g.5).

*Revised again 2026-09-24 for D21-46* (HEAD `c5d19d7`, against that ledger's `## D21-46`). Its R5 and R6 are this
stage's. They are cited as **D21-46 R5** and **D21-46 R6** throughout, so they cannot be confused with this stage's
own records R1–R5 (§8g.8). The last bullet of the list above is superseded by both. What changed:

- **D21-46 R5, first half**: *"`integration-tests`' test-writing steps are re-homed to implement, as N23"*. The
  activity moves to the implement stage whole, as N23 moved the four, over the same test-authoring kind. It takes a
  version bump (2 → 3) and a changelog row, `story.toml`'s `external_dep` → `IntegrationReport` row moves with it,
  and the P6 lattice re-run covers it (§8g.2).
- **D21-46 R5, second half**: `migration-dry-run` is AC-347's **single** named residual. That no Phase-1 lane reaches
  it is verified by quotation, not asserted (§8g.5). The AC-347 lint's residual set is now that one activity, and a
  selection test asserts the unreachability.
- **D21-46 R6**: `smoke-test` gains a `passing` check, `smoke.status == 'passing'` with `on_false = "fail"`, as an
  ADR-0072 edit (1 → 2) with a changelog row. The option framing in §8g.6 is replaced by the decision. Its test is
  renamed `demands::a_failing_smoke_result_fails_the_stage` and proves both arms.
- **Counts**: **sixty-four** new tests (`rk-kernel` 9 → 11), and twenty-three mutants (M20–M23 added, §8g.9).
  Checkpoint 8g (2), (5), (7c) and (7g) read the new shape. *(Superseded 2026-09-24 by the D21-47 fixes below:
  sixty-six new tests and twenty-five mutants.)*
- **The floor**: the drafted `PREV` was **954**, `s8f`'s drafted Checkpoint 8f `FLOOR`, in place of 939. *(Stale.
  The consistency pass below supersedes it: `PREV` is **1063**, and the final consistency pass **1105**.)* It is
  still re-derived at stage open from Checkpoint 8f's recorded count (§8g.10).

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The run worktree is `s8d-ii`'s** (D21-46 R1): its root, the `setup` call and the carry at creation. Every
  sentence that named `s8e` for them now names `s8d-ii`.
- **`verify_completed`'s `item` rides `s8e`'s interim key, beside the payload** (§8g.4). D21-46 R3 fixes N5's key
  *"outside the canonical digest"*, and `s8e`'s record supersedes nothing of ADR-0131 because the key is not in any
  payload. A payload `item` here would be the addressing member ADR-0131's first forbids bullet names. The two names
  `s8j` binds to are unchanged: the `item` key and the `adapter` payload member.
- **(12) adds `REDKILN_LAUNCH_RECORD` unset** to every replay (D21-46's one replay guard), and the trap asserts that it
  starts empty.
- **The floor chain is recomputed once**: Checkpoint 8f's 1063 + this stage's 64 = **1127**, not the drafted 954 + 64,
  and (9) gains the at-least-44-result-lines guard. *(With the two tests the D21-47 fixes add: 1063 + 66 = **1129**.
  Superseded by the final consistency pass: 1105 + 66 = **1171**.)*

*Adversarial-review fixes 2026-09-24 (D21-47)* (the review of this file at `4a8c832`, fixed at HEAD `fc80687`;
`spikes/DECISIONS-2026-09-21.md`'s `## D21-47` is the authority). Each finding id, then its fix:

- **B1** (the no-dispatch guard's `harness_binary` side read comments): (12)'s guard is now a Python scan over code
  lines only. A `redkiln run` line passes only after a code line that writes `harness_binary` at
  `rk-fixture-child-claude`, or as a listed refusal-only line. Three synthetic controls prove it can go red: a
  comment-only name, an armed run, and a wrong exit code.
- **M2** (Checkpoint 7d's two refusal-only run lines): a named, quoted refusal-only list, each line admitted only when
  its next line asserts rc 2 or rc 3. No comment was added to 7d. Measured against 7d's block as it stands: both
  lines are admitted with the list, and both are refused without it.
- **M3** (process-level runs outside the trap): (1a) builds D21-47's wrapper before any cargo test run. The wrapper is
  a fake `claude` first on `PATH` that prints the pin, `REDKILN_LAUNCH_RECORD` unset, and the trap `CLAUDE_CONFIG_DIR`,
  with controls for all three. (5), (9) and (12) run under it, and the trap is asserted empty after each.
- **M4** (`verify_completed` had no origin): `origin` and `stage` members (§8g.4). AC-473's amendment reads only
  `origin = "pass"` rows. `verify::verify_completed_carries_its_origin_and_stage` and mutant M24 were added, and (3)
  asserts `origin` in AC-473's block.
- **M5** (N24's row against agent-authored `Spec`s): §8g.3's warning. R2 names the gap and its owners, `s8h` and
  `s8j`, and §8g.11 reports it upward.
- **M6** (D21-46 R4's two trials): §8g.7 puts the declaration on each lane item and cites R4. The promotion follows
  the trials, and the draft's *"must precede"* sentence is withdrawn. `tag_release::a_lane_run_admits_the_trial_activity_only_on_the_declaring_item`
  and mutant M25 were added.
- **M7** (unexecuted hand-offs): §8g.1 makes Checkpoints 8d, 8d-ii and 8e provisional, unconditionally, and spells
  out the three moves. It states what the low-facet lanes select: implement is unchanged, and `equivalence-boundary`
  is selected nowhere. (12) replays all three.
- **M8** (the mutation tree was unnamed): the tree is `<item>.mut` (§8g.5). `s8d-ii`'s validator gains the suffix by
  dated line (move 3), and (0a) recomputes the worst-case sum. It measured 255 of 259 at `fc80687`.
- **M9** (AC-137 overclaimed): only the type half is claimed. The render half is routed to `s8f`'s `render::`
  renderer over `s8h`'s `Spec` template, and it is reported upward.
- **m10** (clause 5): R3 cites `s8d-ii`'s and `s8f`'s clause-5 records by their recorded ids, and states the
  consolidated write set. (2) asserts the set.
- **m11**: (2) asserts R1 and ADR-0162's `supersedes` and `superseded_by` in both directions.
- **m12**: (7c) reads `<id>@<version>` pairs for all six edited activities and an `IntegrationReport` row. (7d)
  asserts version exactly 3, the moved row's `when` and `rigor`, and that each emit binds the test-authoring kind and
  not `TestReport`.
- **m13**: (3) asserts dated AC-320 and AC-485 notes, and AC-473's dated line.
- **m14**: the whole-activity move is flagged for R1's flip line (§8g.2, R1), and (2) checks the quotation.
- **m15**: the member name is `acceptance_criteria`, which the shipped catalog reads. `s8h`'s `criteria` must follow,
  and this is reported upward.
- **m16**: §8g.11 counts the verify block's seven whole ids correctly. AC-321 sits beside the block.
- **Cross-file 6**: §8g.1, move 1 (smoke-test, `rolling-deploy` → `tag-release`, `equivalence-boundary`,
  `integration-tests`), with Checkpoint 8d provisional.
- **Cross-file 7**: `verify.ac-discriminates`'s expiry row and `MINTED_CLASSES` entry are handed to `s8h` (§8g.6,
  §8g.11).
- **Cross-file 18**: the stale `PREV` 954 line is marked superseded.
- **Cross-file 19**: the ADR-0162 patch-clause supersession is unconditional (§8g.5, §8g.8).
- **D21-47's rules, as they fall here**:
  - Records are located by the id recorded at write time, through (2)'s `R<n>_ID` lines.
  - Clause 5 has one reading.
  - The wrapper covers the replays and this block's own cargo runs.
  - This block has no card or permission precondition, so the `RK_CP_NESTED` rule has no site in it.
  - The optional-`null` rule has no site in this file.
- **Counts**: **sixty-six** new tests (`rk-kernel` 12, `rk-cli` 39), twenty-five mutants, and a floor of 1063 + 66 =
  **1129**. *(Floor superseded by the final consistency pass below: 1105 + 66 = **1171**.)*

*Final consistency pass 2026-09-24* (the eleven `s8` files read against each other after the D21-47 fix round, at
`fc80687` plus that round's edits; D21-47 and the pass's brief the authority):
- *Final consistency pass 2026-09-24:* **The shared no-dispatch wrapper.** The block's inline wrapper, (1a), is
  replaced by the shared helper, sourced at **(0b)** before the build: `runbook/tools/nodispatch.sh`'s `nd_setup`.
  `wrap` becomes `nodispatch`, and `trap_empty` becomes `nd_check`, which also fails on a logged call to the fake. The
  build (1), (5), (9), fmt and clippy (10), and each replay in (12) now run under it, with `nd_check` after each. The
  shell-script fake is gone: a Rust `Command::new("claude")` on Windows resolves only `claude.exe`, so the helper
  compiles its fake. The planted-launch-record control stays, as this block's own assertion. (12)'s static guard's
  variable is renamed `RUN_GUARD`, so it cannot be read as the helper's `nodispatch`.
- *Final consistency pass 2026-09-24:* **Record ids.** The five `R<n>_ID` lines were already literals written at
  writing, refusing while empty. No re-runner supplies an id, and none of (12)'s replays needs one under
  `RK_CP_NESTED=1`.
- *Final consistency pass 2026-09-24:* **The floor.** Checkpoint 8f's expected value is `s8f`'s 1021 + 84 = **1105**,
  so this block's is 1105 + 66 = **1171** (§8g.10, (9)). This stage's 66 agrees with §8g.9's lists and (5).
- *Final consistency pass 2026-09-24:* **`[checklist.smoke]` flips here** (`s8c`'s hand-off, its review finding 7).
  R4's commit replaces the entry's `unauthored` with `path = "checklists/smoke.toml"` in `.redkiln/registry.toml`,
  and (7h) asserts it with a control. Mutant M26 leaves it `unauthored` (§8g.6, §8g.9).
- *Final consistency pass 2026-09-24:* **R3's consolidated write set** includes `s8f`'s third write, `read-tree HEAD`
  on the run-owned worktree's own index, and (2) checks for it (§8g.5).
- *Final consistency pass 2026-09-24:* **Two hand-offs are received.** `s8h` §8h.4 names `Spec`'s member
  `acceptance_criteria` and refuses an authored criterion without a `verification`, and `s8j`'s lane proofs assert
  every AC carries one. §8g.3 and §8g.11 record both as settled.
- *Final consistency pass 2026-09-24:* **The walk-table claims after this stage** are stated: **14 of 14** (fix lane)
  and **12 of 12** (story lane), read from the rows as `s8f` and this stage leave them (§8g.1, move 1).

*D21-48 (2026-09-24)* (the operator's answers to the questions the s8 files left open; `spikes/DECISIONS-2026-09-21.md`'s
`## D21-48` is the authority):
- *D21-48 (2026-09-24):* **X9 decides the whole-activity move.** D21-48 X9: *"**The whole activity moves to implement**, because stage membership is per activity."*
  §8g.2's *"Flagged for R1's flip line"* bullet is replaced by the decision, and the records table's R1 row records
  it rather than flagging it. Checkpoint 8g (2)'s quotation check is re-aimed: the verify record must quote X9's
  *"stage membership is per activity"* as well as D21-46 R5's *"test-writing steps"*, the letter X9 reads, and a
  synthetic control proves the letter alone cannot satisfy it. No test, mutant or count moves: this stage stays at
  **66**. The m14 line above is superseded by this one.
- *D21-48 (2026-09-24):* **The floor follows `s8f`.** `s8f` gains one test under D21-48 X11 (84 → 85), so Checkpoint
  8f's expected value is 1021 + 85 = **1106**, and this block's `PREV` is 1106 and its floor 1106 + 66 = **1172**
  (§8g.10, (9)). Still re-derived at stage open from Checkpoint 8f's recorded count.
- *D21-48 (2026-09-24), floor propagation:* the predecessors now state `s8c` **78**, `s8d` **62**, `s8d-ii` **50** and `s8f` **85**, so Checkpoint
  8f's expected value is 1038 + 85 = **1123**, this block's `PREV` is 1123 and its floor 1123 + 66 = **1189**
  (§8g.10, (9)). The 1106 / 1172 above is superseded, and §8g.10 re-quotes `s8f` §8f.2 item 9 as it now reads.
- *D21-48 (2026-09-24), cross-file pass:* `s8c` hands on that the port now runs nextest (X15) and that whether
  `verify_completed` journals `runner` is this stage's. §8g.4 takes X15's letter: the runner is on the report and
  reaches the row through `report_digest`. No member, test or count moves.

---

**Why this step exists.** The fix lane's verify stage demands three artifacts and its release stage two, and at
`ca8f387` none of the five has a producer a run can reach. `bug-standard.toml` reads *"`baseline = ["TestReport",
"ConfirmationResult", "SmokeResult"]`"* at verify and *"`baseline = ["BuildArtifact", "ReleaseRecord"]`"* at release,
and its verify stage adds *"`when = "'text' in change"`"* → `ScanReport`. D21-41 (c) and (d) narrowed four of those
out as owed; **D21-42 6a reverses the narrowings** and 6b gives a CLI its own release. Underneath the producers sits
the verify record itself, which three owed items have kept open since 7b-ii:

- **The transport.** `s7c` routed it as RT-6 / R-9 — *"the transport into `rk_kernel::verify` is 7d's composition
  root — a record is OWED (R-9)"* — and `s7d` §7d.18 raised the verify block (AC-320, AC-338, AC-339, AC-341, AC-348,
  AC-349, AC-353, AC-355, AC-359) because *"the verify activity runner is neither an adapter's nor this stage's set
  (no §12 or §13 criterion names it); a record routing the block to a stage is OWED"*.
- **D17-3**, OPEN since 2026-09-17: three key spellings no record fixes, and *"`rk_types::AcceptanceCriterion` (AC-137,
  ADR-0104's `mutation:` field), which the tree does NOT carry"*. The kernel reads a stand-in whose own header says
  so: *"a kernel-local shape, decided 2026-09-17 by the operator: the full `rk-types` spec type is the migration's,
  7d"* (`crates/rk-kernel/src/verify/ledger.rs`).
- **D17-4**, OPEN since the same day: AC-321's *"its record is replayed for audit"* needs a completed verify run's
  event, and `EventKind` has none.

**This stage takes records, and says so at the top.** Five are named in §8g.8, each at the next free id **at the
moment it is written** (no number is pre-assigned here: `s8a`–`s8f` write records too, and their order is not
fixed). Every one is committed `proposed` and flips only on the operator's dated ledger line. Three earlier stage
files said *"this step takes no decision"* and were wrong (`s7g`, `s7h`, `s7i`); this one does not repeat the
sentence.

**What it inherits and must find built, measured at its open (Halt 8 if any is absent):**

| from | what | how the open measures it |
| --- | --- | --- |
| `s8c` (D21-42 1a) | `Toolchain::test` returns an outcome-bearing report (the `VerifyReport` plus passed/failed counts); a red suite is `Ok` with counts; `Err` is cannot-run, timeout or killed only | read the trait in `crates/rk-ports/src/toolchain.rs`; the **seven**-method count after `s8c`'s record V (1b's generic method is the seventh) |
| `s8c` (D21-42 1b, 4a; record V) | the registry-backed seventh method, design §4.5's `registry.toml` verb table with its per-toolchain values table, and the Rust `smoke`, `sca_scan` and `package` handlers | the registry file, its Rust rows, and the three handlers' own tests in `s8c`'s Checkpoint 8c |
| `s8c` (D21-43 N25) | a per-toolchain `setup` verb, and `build` widened to an outcome report as 1a widened `test` | a `setup` row in `.redkiln/registry.toml`; `Toolchain::build`'s return type carries the outcome |
| `s8d` (D21-42 3a, 4c, 4d) | step-grain commits and the complete reference resolver: output binding, iteration, check evaluation, emit binding and test-selection resolution; `"affected"` is the package-level affected set, falling back to whole-suite with `basis_reason` | a fixture activity of shape `[tool, check, emit]` walks to an `artifact.write` |
| `s8d` (its R5) | the question loader, used by `run` and `answer`, over `.redkiln/processes/`, holding the lanes' ten gate questions, `confirmation.not-fixed` and `scan.accept-findings` among them | `answer` no longer passes `&QuestionRegistry::default()`; each of the two ids is declared once under `.redkiln/processes/`; whether `s8d`'s loader test pins the total at ten (§8g.6) |
| `s8e` (D21-42 24c) | the first FakeHarness end-to-end test of `verbs::run::run` | `grep -rn "verbs::run::run(" crates/*/tests` is non-empty |
| `s8d-ii` (D21-44; D21-45 W1–W3; D21-46 R1) | run-owned worktrees at `.redkiln/worktrees/<item>`, with `setup` run at creation | no code line under `crates/rk-cli/src` joins the common dir with `redkiln/worktrees` |
| `s8f` (D21-42 5a, 5b) | rk-store's render/write path, one `artifact.write` per write, under the item's managed artifact directory in the run-owned worktree | the one write path's call site |
| `s8b` (D21-42 11a) | the pack changelog at `.redkiln/CHANGELOG.md` | the file exists |
| `s8b` (D21-43 N25) | `sca-scan`'s check requires `unrated == 0` or routes to `scan.accept-findings` | the check's expression names `report.unrated` |
| `s8c` (D21-63, provisional; *dated 2026-09-26, s8c build*) | `sca_scan` refuses `cvss-unscorable` on a MATCHED advisory whose vector is not CVSS v3 (record V decision 5) | at open, `cargo audit --json --no-fetch --file Cargo.lock` against the pinned real database, listing every matched advisory's vector version; any non-`CVSS:3.x` vector is a named, expected halt (*"sca-scan would refuse cvss-unscorable; rule (3') per D21-63"*) before §8g.6 runs. *(dated 2026-09-26, s8c build, CVSS; D21-64 ruling 7.)* (3') is ruled and built at `s8c`: `CVSS:4.0` vectors are scored by the pinned `cvss` crate, so ~~any non-`CVSS:3.x` vector~~ reads *any matched vector that is neither `CVSS:3.0`, `CVSS:3.1` nor a `CVSS:4.0` vector the crate parses*, and the halt's text ~~rule (3') per D21-63~~ becomes *"sca-scan would refuse cvss-unscorable naming `<vector>`"* |

🛑 **If `s8c` or `s8d` delivered a different shape than this table names, this stage stops and reports rather than
re-deriving the shape** — the producers below are thin *because* those stages are thick, and a producer written
against a guessed port is the `s7g` shape (a lint green over code nothing calls).

### 8g.1 What this stage owns, and what it may not decide

It owns **D21-41 (c) as superseded by D21-42 1a, and D21-42 6a, 6b, 10a, 10b and 10c** (the plan's row). From D21-43
it owns **N23** (the displaced `TestReport` providers re-homed) and **N24** (AC-139's default recorded). From D21-46
it owns **R5** (`integration-tests` re-homed, and `migration-dry-run` pinned as AC-347's single residual) and **R6**
(`smoke-test`'s check). It builds **AC-133's judgement-enum registry** once, for every
later stage. D21-43's integration call reads *"AC-133's judgement-enum registry is built once, by **s8g**. s8h
consumes it."*

It does **not** own the following. It cites each owner rather than re-specifying the work:

- **DebtRegisterEntry.** 6a's fifth demand is `s8f`'s rk-store item write.
- **The Rust `smoke`, `sca_scan` and `package` handlers and their semantics.** These are `s8c`'s record V (*"The
  Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4)"*). So are `setup`, its
  `setup_projects` row and the widened `build` (D21-43 N25).
- **`sca-scan`'s `unrated == 0` edit.** N25's catalog edit is `s8b`'s, beside its other catalog edits. This stage's
  producer walk reads the edited activity and does not edit it again.
- **Question loading.** The loader and `.redkiln/processes/` are `s8d`'s R5. The pack-declared human-root questions
  are `s8j`'s.
- **The run-owned worktree.** Its root, `.redkiln/worktrees/<item>`, and the carry at creation are `s8d-ii`'s (D21-44;
  D21-45 W1–W3; D21-46 R1).
- **Card grants, the `Spec` schema and the AC-133 classification.** The card grants, the `Spec` type's schema and the
  per-enum AC-133 classification the operator signs are `s8h`'s (D21-41 g).
- **The story lane, the real trial and AC-114.** The story lane is `s8j`'s (D21-41 f). So are the tag-release real
  trial, AC-114's promotion and its `curation.decision` lint (§8g.7).
- **Any real dispatch.** **No line of this stage dispatches to a real account** (D21-40 decision 2). §8g.10 (12)
  asserts this of every checkpoint it replays.

**What this stage edits, and the checkpoints each edit makes provisional.** Each row is marked `passed <hash>, re-run
pending s8g <crate>` in `PROGRESS.md` §1, in the commit that lands the edit. The precedent is ADR-0124 decision 5:
*"The consequence for Checkpoint 5 is taken, not routed around"*.

| crate or tree | edit | provisional row | re-run where |
| --- | --- | --- | --- |
| `rk-types` | `EventKind::VerifyCompleted`; `AcceptanceCriterion` in `registry::EXPORTED`; the judgement-enum registry | Checkpoint 5 | Checkpoint 8g (12), whole |
| `rk-git` | `GitOp::Tag`, `GitOp::Apply` | Checkpoint 6 | Checkpoint 8g (12), whole |
| `rk-kernel` | the verify runner; `KindSource::Pass`; the ledger reads `rk_types::AcceptanceCriterion` | Checkpoints 7b and 7b-ii | Checkpoint 8g (12), whole |
| `rk-adapter-toolchain-rust` | **tests only**: `selection::ac348_…` and `selection::ac355_…`, both over `s8c`'s affected-set function. No source line changes, because the handlers are record V's | none | Checkpoint 7c's whole re-run, with its live probe, is `s8j`'s (D21-43 N28: *"s8j re-runs it whole, from a plain terminal through the launcher, with the live probes"*) |
| `rk-cli` | `verify_pass.rs`, the `release_tag` git-route handler, the release write, the trial route | Checkpoint 7d | Checkpoint 8g (12), whole |
| `.redkiln/` | the four re-homed activities and the moved escalation row (N23); `integration-tests` re-homed and `story.toml`'s `IntegrationReport` row moved with it (D21-46 R5); `smoke-test`'s check (D21-46 R6, recorded in R4); two `[policy]` rows (N24); the goal amendments; `tag-release` in `catalog-extended/`; `verify.ac-discriminates` in `s8d`'s registry; this repository's `package_binaries` and `claims` rows in `s8c`'s table | Checkpoint 8b. Its shipped-pack lattice *"pins the measured shape"*, and the re-homing moves that shape | Checkpoint 8g (12), whole |
| `rk-kernel` tests, and `s8d`'s walk tables | the two `lane_walk::` tests' expectations and `s8d` §8d.9's rows, by dated line (move 1 below) | Checkpoint 8d | Checkpoint 8g (12), whole |
| `rk-kernel` tests, and `s8e`'s interim-key record | `verify_completed` named item-scoped in the list `fsm::every_item_scoped_kind_carries_the_interim_item_key` reads (move 2 below) | Checkpoint 8e | Checkpoint 8g (12), whole |
| `rk-git`, and `s8d-ii`'s validator rules | the item-id validator refuses a `.mut` suffix (move 3 below) | Checkpoint 8d-ii | Checkpoint 8g (12), whole |

**Checkpoints 8d, 8d-ii and 8e are provisional, unconditionally** (D21-47; the review's cross-file finding 6 and its
finding 7). A line this stage's work commit writes moves each of them, and D21-47's rule is *"A time-bound guard in an
earlier checkpoint is amended by the stage that moves its premise, in the same commit."* Each move below is a dated
line in the earlier stage's file, in that commit:

1. **`s8d` §8d.9's walk tables, and the two `lane_walk` tests' expectations.** `s8d` asks for exactly this: *"Each
   move takes a dated line in this section and moves the two `lane_walk` tests' expectations in the same commit."*
   Both lanes carry low facets and `external_dep = false`. The rows move as follows:
   - **`verify`.** `equivalence-boundary` leaves the row, because `TestReport` is pass-sourced (§8g.2) and the
     selector selects no catalog producer for it. `smoke-test` flips from refusing to **walks**: its checklist exists
     (§8g.6) and its `passing` check reads the result (D21-46 R6). The fix lane then reads `confirmation-test`,
     `smoke-test` and `sca-scan`, all walking. The story lane reads `smoke-test` and `sca-scan`, both walking.
   - **`implement` is unchanged, and the dated line says why.** The test-design escalation moved to
     `[stage.implement]` under *"`when = "criticality >= high"`"*, and both lanes carry `criticality = low`. So no
     re-homed activity is selected there, and `equivalence-boundary` is selected at no stage of either lane. The same
     line corrects `s8d`'s closing sentence, *"the implement rows gain them there"*: on these two lanes they gain
     nothing.
   - **`integration-tests` is in neither table.** Its only demand is `story.toml`'s `external_dep` row, and both lanes
     read `external_dep = false`. The line names it, so that its absence is recorded and not silent.
   - **`release`.** `tag-release` replaces `rolling-deploy`. It is admitted from `.redkiln/catalog-extended/` by the
     lane item's `[trial]` declaration and selected by R5's mechanism (§8g.7). The `lane_walk` fixtures carry that
     declaration, as `s8j`'s lane items do. `build-package` and `tag-release` both **walk**.
   - **The claims lines** are recomputed from the rows as they stand at this stage's open, after `s8f`'s closeout
     flips, and restated by the same dated line. If a test measures a different row, the table is corrected by a
     dated line, and the test is not bent to the table (`s8d`'s rule). *Expected, read from the rows (final
     consistency pass 2026-09-24):* `s8f`'s close leaves **13 of 15** and **11 of 13**, with `smoke-test` and
     `rolling-deploy` refusing in each lane (`s8f` §8f.7). This stage takes `equivalence-boundary` out of both tables,
     flips `smoke-test` to **walks**, and replaces `rolling-deploy` with `tag-release`, which walks. The claims then
     read **14 of 14** (fix lane) and **12 of 12** (story lane). Checkpoint 8d's (6a) floors of 11 and 9 hold.
2. **`s8e`'s interim-key list.** `fsm::every_item_scoped_kind_carries_the_interim_item_key` *"iterates
   `EventKind::ALL` against that list, so a kind added later without a decision fails it"*. `verify_completed` is such
   a kind, so this stage's variant turns the test red until the list names it. R1 names the kind item-scoped, with its
   reason (§8g.4). The list the test reads gains it in the work commit, and `s8e`'s record takes a dated line naming
   R1.
3. **`s8d-ii`'s item-id validator.** The mutation tree is `<item>.mut` (§8g.5). An item id ending `.mut` would collide
   with another item's mutation tree, as one ending `.review-ref` would with its reference tree. `s8d-ii`
   §8d-ii.2's rules gain *"no `.mut` suffix"* by a dated line, and its table-driven test
   `worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused` gains the row.

**The question registry is a fourth move, and it is still measured at stage open.** Checkpoint 8d's (9) asserts that
each of its ten ids is declared exactly once, and an eleventh id leaves that true. If `s8d`'s loader test pins the
registry's total at ten, the question this stage adds moves it, and that test takes a dated edit in the same commit.

**The re-runs are bounded**, because nested re-runs cost hours. (12) re-runs the entry gate, 8f, and the provisional
rows above, each whole, with `RK_CP_NESTED=1` and without `REDKILN_LAUNCH_RECORD` in its environment, under the
trap (D21-46), and with a fake `claude` first on `PATH` that prints the pin (D21-47: *"Every checkpoint replay runs
with a fake `claude` first on PATH that prints the pin"*). The wrapper is the shared helper
`runbook/tools/nodispatch.sh`, whose fake is compiled, so it is the `claude.exe` a Rust `Command::new("claude")`
resolves on Windows (final consistency pass 2026-09-24). The same wrapper covers this block's own process-level
cargo runs, (1), (5), (9) and (10), because *"`CLAUDE_CONFIG_DIR` in this session is the operator's real account"*
(D21-47).
When this block is itself run with `RK_CP_NESTED=1`,
as `s8h` replays it, (12) says that it is skipped and replays nothing. A replayed block that does not honour the
variable still runs its own replays. That costs time, not correctness.

### 8g.2 The verify pass — one TestReport route, called through the widened port

**The route, stated once, because two silent routes is the risk the packet named.** D21-41 (c) chose *"an rk-cli
verify pass on ADR-0162's shape … selected by the required kind `TestReport` and never by a stage literal"*; D21-42
1a kept the pass and changed what it calls — `Toolchain::test` now returns the outcome, so the pass calls **the
port**, and 1a *"Supersedes D21-41(c)'s 'through the port' register reading, which is withdrawn as unneeded"*. So:

- **`TestReport` has exactly ONE producer on a lane: the verify pass.** It runs for a node whose required set
  contains `TestReport` — read off the resolved selection, never off `resolved.stage` (ADR-0162 decision 5: *"The
  review pass is selected by an activity's declared `provides`, never by a stage name"*, and the same record forbids
  *"matching `resolved.stage` against a string literal"*).
- **The catalog's four `TestReport` providers are DISPLACED, not deleted.** `artifacts.toml`'s header says *"source =
  "activity" → some catalog activity provides it"* and the selector's `KindSource::Activity` doc says *"Some catalog
  activity `provides` it"* — so as shipped, selection would walk `unit-tests`, whose `gaps` and `fill` steps are
  `kind = "agent"` under `when = "rigor >= recommended"`, and bug-standard's verify stage is `mandatory`. That walk
  breaches AC-347 in terms (*"a verify run that emits any harness dispatch fails the activity"*). The verify record
  (§8g.8, R1) therefore adds a third `KindSource`, **`pass`** — *produced by a composition-root pass the record
  names* — and sets `[artifact.TestReport] source = "pass"`. The selector selects no catalog producer for a `pass`
  kind; stage completeness still requires its `artifact.write`. **The rejected alternative is named**: keep
  `source = "activity"` and author an agent-free `whole-suite-test` activity ranked first — rejected because
  bug-standard's `[[stage.verify.require]] when = "criticality >= high" … rigor = "mandatory"` escalates the
  test-design family to `equivalence-boundary` and `property-based-testing`, both of which carry agent steps, so the
  catalog route re-enters AC-347's breach at the rigor the fix lane is most likely to reach.
- **ConfirmationResult, SmokeResult and ScanReport are catalog walks** (§8g.6) — `confirmation-test`, `smoke-test`
  and `sca-scan`, all agent-free, walking under `s8d`'s resolver and `s8c`'s port and registry method. Their
  `toolchain:test` calls are port calls like the pass's, and journal the same row (§8g.4).

**The displaced providers are re-homed (D21-43 N23), and there are FOUR of them, measured.** N23 reads *"The five
displaced TestReport activities | **Re-homed to the implement stage** as test-authoring activities under ADR-0072.
Verify only measures."* The draft this revision replaces listed five, and one entry was wrong. At `a6349b6`,
`grep -lE '^provides *=.*"TestReport"' .redkiln/catalog/*.toml` finds **four**: `unit-tests`, `unit-tests-ue`,
`equivalence-boundary` and `property-based-testing`. `migration-dry-run` declares *"`provides     =
["MigrationTestReport"]`"*, and a `pass` source for `TestReport` does not displace that kind. Its one require row was
also dropped at 7e (`story.toml`: *"DROPPED: [[stage.verify.require]] when = 'data_migration', artifacts =
["MigrationTestReport"]"*). R1 (§8g.8) executes N23 over the four:

- **Each activity moves to the implement stage and provides the test-authoring kind.** It takes `stage =
  "implement"`, and in `provides` it replaces `TestReport` with **one test-authoring kind**, listed first. R1 names
  that kind and gives it a `source = "activity"` row in `artifacts.toml`. Its emit binds that kind. Its own
  `toolchain:test` runs stay as step evidence, never as the `TestReport` artifact. R1's options are **(a)** mint a
  kind for the authored tests, or **(b)** reuse `TestDesign`. Option (b)'s value schema (`spikes/p9/schemas.toml`
  `[value.TestDesign]`) is a case design, and `equivalence-boundary` already emits it beside its report, so one kind
  would carry two meanings. Recommended: (a).
- **Each takes a version bump and a row in `.redkiln/CHANGELOG.md`.** Under ADR-0072, an edited activity is a new
  version.
- **The test-design escalation moves with them.** Both packs' verify stage carries *"`when = "criticality >= high"`"*
  → `TestReport` at `rigor = "mandatory"`, under the comment *"criticality raises the rigor of test design"*. That row
  moves to `[stage.implement]` over the new kind in both packs, because a rigor floor on a pass-sourced kind selects
  nothing. `selection::the_rehomed_test_authoring_activities_select_at_implement` asserts the outcome.
- **The P6 lattice is re-run, and Checkpoint 8b becomes provisional.** The lattice, ported to Rust at `s8b` (D21-42
  13b), is re-run with the new source and stage, and its dead count is reported **by id** against AC-102's registered
  20% line. `s8b`'s shipped-pack lattice test *"pins the measured shape"*, so its pin takes a dated edit in this
  stage's work commit, and Checkpoint 8b is re-run whole (§8g.1).
  *(dated 2026-09-26, `s8c` build, D67; `spikes/DECISIONS-2026-09-21.md` D21-67 ruling 6.)* **A named precondition of
  this re-run: the unsat report's five unexercised branches are pinned first.** The ruling reads: *"The five
  unexercised unsat-report branches of the ported P6 lattice: a hand-derived `report-branches` lattice fixture with
  one test and five killed mutants, cross-checked by one quoted `lattice.py` run over the same tree, **carried by
  `s8g`** as a named precondition of its lattice re-run."* The five are `s8b` §8b.10's (refuting round 5):
  `lattice.rs`'s `forbidden_stage` test (rendered `forbidden-stage=`), the `unsat_other_stage` counter (rendered
  `wrong-stage-producer=`), the example's `other_stage` list, the keyed render `format!("{kind}{suffix}")` and the
  transitive entry `format!("{}:transitive@{s}", u.kind)`. None is pinned by a golden byte or a fixture today (34 of
  34 unsat lines in `p9-run.txt` at zero and `[]`, measured 2026-09-25). Before the re-run above:
  - **The fixture.** `crates/rk-kernel/tests/fixtures/lattice/report-branches/` (beside `unsatisfiable/`, sharing
    `common/`), hand-derived so that one lattice run reaches all five: a required kind whose only producer sits at a
    forbidden stage; a producer at another, non-forbidden stage (the wrong-stage count and the `other_stage` list); a
    keyed kind (`{inst}` suffix); and a required kind unsatisfied only because its producer's own requirement is (the
    transitive entry). The expected unsat lines are derived by hand from the fixture and written into the test before
    it is run, never copied from its output.
  - **One test**, `rk-kernel`'s `lattice::the_report_branches_fixture_exercises_every_unsat_branch`, red first (the fixture absent),
    asserting each of the five renders by its exact bytes.
  - **Five mutants**, run from Python (anchor asserted once, tmp + `os.replace`, sha256 restored), each killed by that
    test: RB1 `forbidden_stage` always `false`; RB2 `unsat_other_stage` never incremented; RB3 the example's
    `other_stage` list rendered empty; RB4 the keyed render drops `{suffix}`; RB5 the transitive entry is never pushed.
    A survivor is first investigated for unobservability over the fixture, then the fixture is widened; it is never
    accepted.
  - **One quoted `lattice.py` cross-check** over the same tree: ~~`python spikes/p9/lattice.py --packs <fixture>/packs
    --catalog <fixture>/catalog`~~, its unsat lines quoted in the test's doc and here beside the Rust ones. The run's
    own output must show a non-zero pack and cell count before any line is compared: a `--packs` path that matches
    nothing prints a baseline over zero packs and exits 0.
    *(dated 2026-09-26, `s8c` refuting round 3.)* **The struck command cannot read the fixture.** `lattice.py`'s
    argparse takes `--packs` as a comma list of pack IDS that filters its own packs, *"`pack_paths =
    sorted(glob.glob(os.path.join(HERE, "packs", "*.toml")))`"* then *"`if args.packs: packs = [p for p in packs if
    p.id in args.packs.split(",")]`"*, and it reads `facets.toml` and `artifacts.toml` from `HERE` (*"`HERE =
    os.path.dirname(os.path.abspath(__file__))`"*), never from `--catalog`'s tree. A path in `--packs` selects zero of
    `spikes/p9`'s packs, so the struck run always trips the guard above. `lattice.py` may not be edited: Checkpoint 8b
    (2) runs `git diff --quiet ca8f387 -- … spikes/p9/lattice.py`. **The cross-check uses `s8b` §8b.2 step 2's
    staged-copy method instead:** (i) make a scratch directory under `target/` and name it by a RELATIVE path (`s8b`
    §8b.2: an absolute `/c/…` path reaches Python unconverted); (ii) write `lattice.py` at `P9_PIN` into it, `git show
    "$P9_PIN:spikes/p9/lattice.py" > "$X/lattice.py"` (`P9_PIN` is `crates/rk-kernel/tests/it/p9_pin.rs`'s const,
    `ca8f3874ed9185b419b00b21b33713ba128de217`); (iii) copy the fixture's `catalog/` and `packs/` into it, and
    `common/facets.toml` and `common/artifacts.toml` to its top level, the layout `HERE` expects; (iv) run it there with
    NO `--packs` and no `--catalog` (both then default to the staged tree), `(cd "$X" && PYTHONHASHSEED=0
    PYTHONIOENCODING=utf-8 python lattice.py) > "$X/run.txt"; rc=$?`, and assert `rc` 0. **Guards, before any unsat line
    is compared:** the first line, *"`P6 lattice run; catalog=<n> activities; packs=[…]; pack digests: …`"*, names
    `<n>` > 0 and the fixture's own pack ids, never `packs=[]`; each per-pack line's *"`prereg pairs=`"* count is
    > 0 (the cells); and the digest tag every line opens with, *"`cat:<12> fac+art:<12> pk:<12> |`"*, shows the
    fixture's `fac+art:`, never `spikes/p9`'s `fac+art:592fa138d68d` (`p9-run.txt`'s). Measured 2026-09-26 at `s8c`
    refuting round 3, the method over the existing `unsatisfiable/` fixture: rc 0, *"`packs=['story']`"*, *"`prereg
    pairs=  2592`"* per mode, and the tag `fac+art:1cd93c6cef11` (the shared `common/`'s; it moves only if `common/`
    does, and the new tag is then recorded).
  - **Counts.** The one test joins §8g.9's list and Checkpoint 8g (5)'s `rk-kernel` selection and `want`, and the
    floor chain, by dated edit at this stage's open (floors re-derived from the entry checkpoint's recorded count).

⚠️ **Two things N23 does not settle. The first is named in R1 and reported upward (§8g.11). D21-46 R5 settles the
second:**

- **`unit-tests-ue` authors nothing.** Its steps are two tool steps (`cook`, `automation`), a check and an emit, with
  no agent step. It is re-homed by N23's letter, and the record says so.
- **Two agent-bearing activities stay at verify.** `integration-tests` (`IntegrationReport`, one agent step, reachable
  in the story lane under `external_dep`) and `migration-dry-run` (one agent step, unreachable since 7e) remain at
  verify after N23, so its *"Verify only measures"* did not hold over the shipped catalog. D21-46 R5 reads
  *"**`integration-tests`' test-writing steps are re-homed to implement**, as N23. `migration-dry-run` stays a named
  residual, because no Phase-1 lane reaches it (`data_migration` is NOT MEASURABLE)."* The first half is executed
  below. The second is executed in §8g.5, where AC-347's shipped-catalog lint pins `migration-dry-run` alone.

**`integration-tests` is re-homed as N23 re-homed the four (D21-46 R5).** Its test-writing step is `write`
(*"`kind    = "agent"`"*, *"`role    = "implementer"`"*), and its `green` check loops back to it (*"`on_false = { to =
"write", max = 2 }`"*). A step cannot sit in a different stage from its activity, so R1 moves the activity whole, as
N23 moved the four:

- **`stage = "implement"`, and the test-authoring kind is listed first.** `provides` becomes `[<K>,
  "IntegrationReport"]`, where `<K>` is the one kind R1 names for the four. The emit binds `<K>` beside the
  `IntegrationReport` it already composes. `IntegrationReport` stays, because no other activity provides it and
  `story.toml` requires it.
- **The version moves 2 → 3, with a changelog row.** `.redkiln/CHANGELOG.md` takes `integration-tests@3` in `s8b`'s
  form (`s8b` §8b.5: *"`<id>@<new version>`, the date, and the ledger line"*, quoting the old and new text). The
  ledger line is D21-46 R5.
- **The require row moves with it, for N23's reason.** `story.toml`'s *"`when = "external_dep"`"* → *"`artifacts =
  ["IntegrationReport"]`"* row moves from `[stage.verify]` to `[stage.implement]`. A verify-stage row over a kind
  whose only producer sits at implement selects nothing. `bug-standard.toml` has no such row: measured at
  `c5d19d7`, `grep -rn IntegrationReport .redkiln/packs` finds `story.toml` alone.
- **Verify still measures the integration tests.** The tests that `write` authors are in the workspace, so the verify
  pass's whole suite (below) runs them. Only the authoring leaves verify.
- **The rejected alternative is named.** Splitting the activity into an implement-stage authoring activity and an
  agent-free verify measurement would mint a 103rd shipped activity. That moves `s8b`'s pinned count of 102 and the
  `curation.decision` backfill (§8g.7), and N23's precedent moves activities whole.
- **The lattice re-run above covers it**, and its dead count is reported by id with the four.
  `selection::integration_tests_selects_at_implement_under_external_dep` asserts the outcome: over `story.toml`, with
  `external_dep = true`, `integration-tests` is selected at implement and at no other stage.
- **Decided: the whole activity moves** (D21-48 X9, 2026-09-24; it answers D21-47 review finding 14). D21-46 R5's
  letter re-homes *"`integration-tests`' test-writing steps"*. The operator took the reading: *"**The whole activity moves to implement**, because stage membership is per activity."*
  So R1 moves the whole activity, its `run` and `green` measurement steps with `write`. R1 quotes both D21-46 R5's
  letter and X9's decision, and the split this section names stays rejected. Checkpoint 8g (2) asserts both
  quotations. *(D21-48 (2026-09-24): replaces the flag for R1's flip line.)*

**What the pass does, in order** (`crates/rk-cli/src/verify_pass.rs`, beside `review_pass.rs`, on its shape):

1. **Meet a set-up tree.** The pass runs in the run-owned worktree, which is at `.redkiln/worktrees/<item>` after
   `s8d-ii`'s D21-44 move. The whole suite it requests includes the Node fixture tests. `s8c`'s `setup` verb has run
   there at creation (the carry is `s8d-ii`'s). If it failed, creation refused by name (N25: *"refused by name on
   failure"*), so no pass reaches a tree that was not set up.
   `verify_pass::setup_precedes_the_whole_suite_in_a_fresh_run_worktree` checks this over a counting `Toolchain`
   double on a fresh run. It asserts that `setup` comes before the pass's one `test` call, and that a failed `setup`
   leaves zero `test` calls.
2. Refuse before any port call if the changed-file set or the selection policy is unresolved (AC-320) —
   `StepError`, no gate row, no port call (asserted by a counting `Toolchain` double: zero calls).
3. Call `rk_kernel::verify::runner` — **the kernel makes the port call** and journals `verify_completed` in the same
   function (§8g.4). The selection is AC-328's *"explicit whole-suite policy"*: the pass requests whole-suite, and the
   report's `basis_reason` is what the adapter says under AC-352's closed enum (`probe-unverified` while no
   affected-selection probe is recorded at the pin).
4. Map the outcome: `Ok` with `failed == 0` is green; `Ok` with `failed > 0` is a **failed verify, never a report of
   green**; `Timeout` and an OS kill are `Unverifiable` (AC-331); a spawn failure is `Unverifiable { cannot_run }`
   (AC-344, the note `s7d` §7d.12 owed).
5. Validate through `report_at_boundary` (AC-324), feed the ADR-0104 decision 3 comparator, and write the keyed
   `TestReport` through `s8f`'s rk-store write path — **one `artifact.write`**. The key is the item's `toolchain`
   facet element (ADR-0073: *"one instance per element of the item's `toolchain` set"*), **never the adapter name**.
6. Run the AC ledger and the stage-exit check (AC-346, ADR-0125) over the item's ACs as `rk_types::AcceptanceCriterion`
   values (§8g.3), including the mutation harness for every AC carrying a declared `mutation` (§8g.5, AC-338/AC-339).
7. Append through the store and re-read the journal (ADR-0162 decision 6's idiom).

🛑 **The pass is whole-suite, and a cold whole suite runs against ADR-0140's bound.** ADR-0140 decision 2 reads *"It
is not a backstop, because there is nothing behind it"*. For tool steps, `s8c`'s record B moves that sentence. D21-42
10c says *"A cold `cargo test --workspace` wall time is measured first"*, and `s8c` §8c.5 is that measurement: *"This
is that measurement, and `s8g` cites it rather than repeating it."*

**This stage cites the figure by quotation from `s8c`'s record and names the commit it was measured at. It does not
re-measure.** If `s8c`'s record carries no such figure, this stage stops (Halt 8) rather than measure a second time.

**The figure is the slowest child.** The Rust adapter's timeout bounds *each child it spawns*, not the call (ADR-0140
decision 2, as `s8c` §8c.5 reads it), and §8c.5 finds that *"the cold compile lands in the FIRST, the `--list` pass"*.
So the verify record carries one line of the form `cold cargo test --workspace slowest child: <N> s (source: <s8c's
record id>, at <sha>)`, and Checkpoint 8g (8) reads it.

**A figure at or above 3600 s is a halt.** Every cold verify pass would then be `Unverifiable` by construction (golden
constraint 6: size the ceiling against the slow condition).

### 8g.3 The spec/pack schema record — D17-3's three spellings, and `AcceptanceCriterion` exported (D21-42 10a)

D17-3 lists three spellings that are *"readings, not decisions"*. The schema record (§8g.8, R2) fixes all three **as
they are read today**, so nothing on the tree moves but its warrant:

| spelling | where it is read today | fixed as |
| --- | --- | --- |
| `[policy] verification_required = [<stages>]` | `select/pack.rs`: *"**AC-455's policy row**"* | the process pack's key, unchanged |
| `[mutation] feasible = <bool>, reason = <str>` | `rk_ports::toolchain::MutationFeasibility::declared`; the Rust fixture pack | the key, unchanged — **and its shipped home** (below) |
| `[[ac]] … [ac.mutation] not_feasible = "<reason>"` | `verify/ledger.rs`'s fixture reader | superseded by `AcceptanceCriterion`'s serde form (below) |

**Two absences the record must state, both measured at `ca8f387`:**

- **The shipped `.redkiln/` declares no `[mutation]` table anywhere.** Its two packs are process packs, and AC-337 says
  *"an absent declaration is a configuration error"*. The record gives the declaration a shipped home — the Rust row
  of `s8c`'s per-toolchain values table (D21-42 4a: *"Each value carries its measurement as its warrant"*), with
  `feasible = true` and ADR-0104 decision 1's warrant — so the ledger's feasibility input exists on this tree.
- **Neither shipped pack declares `verification_required`.** Re-measured at `a6349b6`: each pack's `[policy]` holds
  `stages` and `stage_order` only, so AC-455's refusal cannot fire on this tree. What the shipped default *is* belongs
  to **AC-139, a HUMAN GATE**: *"the choice is recorded as a human decision with its rationale, and the shipped
  default is whatever that decision names"*.
  - **The operator took that decision on 2026-09-24.** D21-43 N24 reads *"**Block (fail-closed)**, recorded as the
    human decision AC-139 requires"*. R2 records the decision and its rationale by quotation.
  - **Both shipped packs gain `verification_required = ["verify"]` in their `[policy]` table.** The row names the
    verify stage because that is where the pass runs the ledger and the admissibility check (§8g.2, step 6).
  - **AC-139's own *Verified by* makes a policy row the right form**: *"What IS machine-checked is AC-138: the value
    is a policy-table row, so the human decision is a data change"*. AC-139 takes a dated register note that names
    N24 and R2.
  - **A test proves the row decides.** `verify_pass::ac139_shipped_packs_block_a_missing_verification` loads each
    shipped pack over a fixture spec with `verification: None` and asserts that the stage is refused, naming the AC
    id. It then asserts that the same spec passes under a copy of the pack with the row removed. That is AC-138's
    *"computed from the policy table row, not from a constant in the type"*.

⚠️ **The row refuses every AC whose `verification` is `None`, and nothing yet makes a lane's `Spec` fill it** (D21-47
review, finding 5). The `Spec` on each `s8` lane is agent-authored: `fix-spec` writes it at the fix lane's
specification stage, and `bdd-scenarios` at the story lane's. `AcceptanceCriterion` keeps `verification:
Option<Text>`, because AC-137's THEN accepts a criterion with none, so the type cannot be what supplies it. Under
N24's block default, a lane whose `Spec` leaves one AC's `verification` empty refuses at verify. R2 names the gap and
its two owners, and does not close it here:

- **`s8h`**, whose `Spec` schema and specifying cards are D21-41 (g)'s. Every AC a specifying activity emits carries
  an authored, non-empty `verification`. The card's output contract says what to write, and `s8h`'s schema check
  refuses an authored `Spec` without it. That check is over the authored value; `AcceptanceCriterion` itself is
  embedded unchanged. AC-137 makes `verification` *"an authored field from the first CLI write"*, and this is that.
- **`s8j`**, whose fixture proof walks both lanes. It asserts that every AC of each lane's `Spec` carries a non-empty
  `verification` before verify runs, so that a gap reads as that line and not as a verify refusal.
- **Both files now carry it** (re-read 2026-09-24, final consistency pass). `s8h` §8h.4 refuses an authored `Spec`
  whose criterion lacks a `verification`, by `classify::a_spec_criterion_without_an_authored_verification_is_a_schema_failure`.
  `s8j`'s lane proofs *"assert every AC carries a `verification`"*.

**`rk_types::AcceptanceCriterion`** (AC-137's GIVEN/THEN; ADR-0104 decision 1's field): `id`, `criterion: Text`,
`gwt: Option<Gwt>` (*"`Gwt { given: Text, when: Text, then: Text, and: Vec<Text> }`"*), `verification:
Option<Text>`, and `mutation: Option<MutationDeclaration>`, where `MutationDeclaration` is
`#[serde(tag = "kind")]` over `Declared { patch, suite }` and `NotFeasible { reason }` (ADR-0084's discriminated
form). A prose-only mutation is refused at ingest (ADR-0096 via golden constraint 12). It enters
`registry::EXPORTED` — whose doc says *"Every schema that crosses the closed-output boundary"* — and gains its
`exported__AcceptanceCriterion.snap`, accepted once, deliberately, and reviewed as a schema change.

**The kernel-local type is retired.** `SpecAc` and its TOML reader leave `rk-kernel`; the ledger, the exit check and
`admissibility::admissible` read `&[rk_types::AcceptanceCriterion]`. Two text fixtures carry the old name and move
with it (`crates/rk-kernel/tests/fixtures/lint-controls/ac335_status_reads_the_suite_verdict.rs` and a string in
`tests/it/verify.rs`) — the name occurs **17** times under `crates/` at `ca8f387`, and Checkpoint 8g (4) requires
zero code lines.

🛑 **AC-133's registry does not exist, and this stage builds it rather than registering into nothing.** The brief
asks for the new enum to be *AC-133-registered*. AC-133's THEN is *"the enum appears in the judgement-enum registry
with an explicit judgement-bearing yes/no AND every enum marked yes has a sibling free-text `rationale` field"*.
Measured at `ca8f387`: `grep -rni judgement crates/rk-types/tests crates/rk-types/src/schema` returns **nothing**,
and no rationale-sibling rule exists — yet `s5`'s acceptance set claims AC-133 and `PROGRESS.md` §1 row 5 says
*"Decisions 6 and 7 executed (ADR-0104's `mutation:` field on `AcceptanceCriterion`…)"*, which D17-3 already called
false. The registry is built here to AC-133's *Verified by* (a completeness test over every enum reachable from
`EXPORTED`, the rationale-sibling rule, and a must-fail fixture), `Findings`' enums registered with it; row 5 takes a
**dated correction in the tracker commit**, not an erratum (D15-2's form for an amendable file). A third test,
`judgement_registry::the_registry_names_no_enum_that_is_not_exported`, keeps the registry from outliving its types.

**The registry is built once, here, and `s8h` consumes it** (D21-43's integration call). This stage registers every
enum reachable from `EXPORTED` at its close, `Findings`' and `AcceptanceCriterion`'s included, each with a stated
yes/no. The types `s8h` adds to `EXPORTED` register into the same registry. The per-enum classification the operator
signs is `s8h`'s, because AC-133's own limit reads *"that classification is the human gate AC-156"*. **`s8h` does not
build a second registry.**

⚠️ **A cross-stage constraint on `s8h`.** The pass reads the item's ACs off the committed `Spec` artifact. The `Spec`
type's schema is `s8h`'s (D21-41 g: schemas are Rust types in `EXPORTED`). Until then this stage's fixtures carry a
`Spec` whose one typed member is `acceptance_criteria: Vec<rk_types::AcceptanceCriterion>`, and **`s8h`'s `Spec`
must embed `AcceptanceCriterion` unchanged, under that member name**. It is named here so the second author does not
re-invent it.

**The member is `acceptance_criteria`, not `criteria`** (D21-47 review, finding 15), because the shipped catalog
already reads that name:

- `fix-spec.toml`'s check reads *"`spec.acceptance_criteria.len() > 0`"*;
- `tdd-red-green-refactor.toml` and `mutation-testing.toml` iterate *"`for_each = "artifact:Spec.acceptance_criteria"`"*;
- `spikes/p9/schemas.toml`'s `[value.Spec]` lists `"acceptance_criteria"`.

`s8h`'s sentence named the member `criteria`, and it was reported upward as a cross-file finding. *Settled
2026-09-24 (final consistency pass):* `s8h` §8h.4 now reads *"`Spec`'s criteria member is
`acceptance_criteria: Vec<rk_types::AcceptanceCriterion>`, and nothing in it is redeclared"*.

### 8g.4 `verify_completed` — AC-321's replay event and the identity carrier (D21-42 10b)

The kind is minted by record on ADR-0116's route (its *Option — mint the kind in the runbook* was rejected *"on
mechanism"*), in the verify record (§8g.8, R1), with the design §5 table row and the `rk-types` variant **in the
same change** (ADR-0116 decision 5). Its row:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `verify_completed` | 2 | `report_digest`, `selection`, `basis`, `basis_reason`, `passed`, `failed`, `selected_test_count`, `adapter`, `range: { base, head }`, `paths: [changed paths]`, `toolchain_version`, `origin`, `stage`; and the interim `item` key **beside** the payload, on `s8e` §8e.10's carrier | the kernel, once per **completed** `Toolchain::test` port call, whatever activity or pass made it |

- **10b's four members plus AC-321's three.** D21-42 10b names *"report digest, selection with `basis_reason`, counts
  and adapter identity"*. AC-321's THEN adds *"the range endpoints, the resolved path list and the toolchain version
  are read back from the run event verbatim"* — so `range`, `paths` and `toolchain_version` are carried **as
  members**, not only inside the digest. That is AC-321's text, not a new decision.
- **The runner is on the report, not a member** (*D21-48 (2026-09-24)*, X15). `s8c` now runs `cargo nextest` where
  it is installed, beside a doctest companion, and records the runner on the report (`TestOutcome.runner`, `s8c`
  §8c.2). X15's letter is *"records on the report which runner ran"*, so the row carries it through
  `report_digest` and gains no `runner` member. `passed` and `failed` do not move with the runner, because `s8c`
  §8c.2's invariant (passed + failed equals the `--list` count) holds on both paths. No reading in this stage names
  `cargo test` in a port call's argv.
- **`origin` and `stage`, because the pass is not the kind's only writer** (D21-47 review, finding 4). Every
  completed `Toolchain::test` port call journals the row (*One site*, below), so implement-stage calls journal it too:
  the `toolchain:test` runs of N23's re-homed activities, and `tdd-red-green-refactor`'s `red-verify`. Without a
  member saying which call wrote the row, AC-473's identity reading could be satisfied by an implement row.
  - `origin` is `"pass"` for the verify pass's call, and `"<activity>@<version>/<step>"` for a catalog step's call.
  - `stage` is the node's stage name. It is carried as data, and the kernel never compares it. ADR-0162 decision 5
    bars a stage literal in selection, and this is not selection.
  - AC-473's register amendment reads only the rows whose `origin` is `pass`.
  - `verify::verify_completed_carries_its_origin_and_stage` asserts both arms. A pass call and a `tdd` `red-verify`
    call each journal one row, with their own `origin` and `stage`.
- **`item`, because the store cannot scope a row any other way, carried beside the payload.** When this was drafted
  the store's selector was inert, and ADR-0163 decision 4 carried `item` as a payload member for that reason. D21-43
  N5 has since made an interim key the rule for every item-scoped kind (*"An interim `item` key on every item-scoped
  event kind"*), and D21-46 R3 fixes where it lives: *"**Carried outside the canonical digest**, as ADR-0131 places
  its tag set."* `s8e` §8e.10 carries it beside the event, in the `t2_event.item` column, and `query(ByItem)` honours
  it. So `verify_completed` takes the key on that carrier and not in its payload, which moves no ADR-0131 bullet.
  `s8e`'s `fsm::every_item_scoped_kind_carries_the_interim_item_key` covers the new kind once the list it reads names
  it, which this stage's work commit adds (§8g.1, move 2). `s8j`'s fixture proof reads the item's `verify_completed`
  rows by that key and asserts `adapter` on every one whose `origin` is `pass`. The four names, the `item` key and the
  `adapter`, `origin` and `stage` members, are **fixed here** so `s8j`'s probe binds to them rather than to a guess
  (*restated 2026-09-24, consistency pass; `origin` and `stage` added 2026-09-24, D21-47*).
- **Tier 2**, by ADR-0109 decision 2's deleted-database test: the report is recomputable by re-running the suite, and
  the next action does not move without the row. No trailer.
- **Completed only.** A run that timed out, was killed or could not spawn writes **no** `verify_completed`; its
  refusal travels as the typed `StepError` row D21-42 20 has the kernel append. AC-321 says *"a **completed** verify
  run"*, and a row for a run that did not complete would be a replay of something that did not happen.
- **One site.** The executor's `toolchain:test` arm and the pass both reach the port through the one kernel function
  that appends the row, so a `confirmation-test` walk journals the same kind as the pass.
- **Adapter identity is data the kernel carries and never compares** (AC-487: *"`rk-kernel` contains no comparison
  against an adapter identity — no adapter name string"*). `rk-cli` supplies the name `real_toolchain`'s `detect()`
  loop chose, as an opaque value on the runtime context. argv[0] is not the identity (ADR-0137 clause 7 makes it
  configurable), and `rustc -V` names a compiler, not an adapter.

**The identity carrier moves, and the register moves with it.** AC-473's *Verified by* reads *"an adapter-identity
assertion on the toolchain stage's dispatch record"* — and no dispatch record carries a toolchain. The verify record
carries the dated register amendment D21-42 10b authorises: the assertion reads the item's `verify_completed` rows
whose `origin` is `pass`.
AC-485's PROVISIONAL identity half (`s7d` §7d.18: *"the dispatch-event identity half has no `DispatchRecord` field to
read and the carrier is routed, RT-41"*) is discharged **for the toolchain adapter** by the same row, and a dated note
on AC-485 says so; its harness half stays with RT-41's carrier. **`s8`'s own sentence** *"the verify stage's
toolchain is rk-adapter-toolchain-rust"* is re-pointed at `verify_completed` by `s8j` (the plan's instrument
amendments), not here. **D17-4 closes** in the tracker commit, naming the record.

### 8g.5 `s7d` §7d.18's verify block, claimed whole (D21-42 10c)

Each id has a named instrument, or a named reason in the verify record. **No id is reached only by the crate-wide
floor.**

| id | instrument (module-qualified, `--test it`) | built on | residue, named |
| --- | --- | --- | --- |
| **AC-320** | `verify_pass::ac320_unresolved_inputs_refuse_before_any_port_call` | a counting `Toolchain` double: zero calls, zero gate rows | its *Verified by* names *"zero `process_spawned` events"*, and **no such kind exists** (`grep -c process_spawned crates/rk-types/src/event_kind.rs` → **0** at `ca8f387`). The record carries a dated *Verified by* note substituting the port-call count — no spawn is possible without a port call — rather than minting a kind for one assertion |
| **AC-321** | `verify::ac321_replay_reads_the_path_list_from_verify_completed_alone`; `verify::ac321_replay_property_over_100_synthetic_ranges` | §8g.4's payload; the replay reads no working tree | — |
| **AC-338** | `verify_pass::ac338_restored_mutation_is_unverifiable` | the mutation harness below; a fixture suite that restores the mutated file mid-run (F17's shape); the post-run content hash of the mutated region compared to the applied mutation | — |
| **AC-339** | `verify_pass::ac339_mutation_runs_in_a_disposable_worktree` | an ephemeral worktree named **`<item>.mut`**, created and **removed on every path** (ADR-0162 decision 10's precedent). It sits beside the review pass's `<item>.review-ref` tree under `.redkiln/worktrees/`, the root `s8d-ii`'s D21-44 move gives that tree, and **never inside the run-owned worktree at `.redkiln/worktrees/<item>`**. The name is fixed (D21-47 review, finding 8). Its 4-character suffix fits inside the 11 characters `s8d-ii`'s worst-case sum already reserves for `.review-ref` (255 of 259 at `c5d19d7`). `s8d-ii`'s validator refuses an item id ending `.mut` (§8g.1, move 3), and Checkpoint 8g (0a) recomputes the sum. The test asserts the tree's path, that the run-owned worktree's `tree_hash` is equal before and after, and that `git status --porcelain` is empty | the patch is applied by the kernel through `GitOp::Apply` and never by a `-p` child, so Claude Code's protected-segment rule (D21-45: *"The path is sensitive if ANY segment is one of"* …) does not bear on where the tree sits |
| **AC-341** | `verify_pass::ac341_the_pass_calls_no_tracker_method` (a counting `Tracker` in `Ports`, which `run.rs` already builds as `tracker: Arc::new(rk_ports::fakes::BoardTracker::default())`); the static half stays `verify_reads_no_suite_verdict_rule`'s, inside `ac335_…` | the port-call assertion | **the *"CI is green and verify is `Err`"* integration half cannot be constructed**: the `Tracker` port has `project` and `inbox` and no status surface, and its module says *"Fake-only in Phase 1; GitHub is Phase 3"*. Named in the record as Phase 3's (AC-508), not claimed. **R1 carries a dated register note on AC-341 that says so**, so the half is recorded as Phase 3's rather than dropped silently |
| **AC-348** | `selection::ac348_selection_is_byte_identical_over_200_change_sets` (Rust adapter); `verify_pass::ac348_selection_is_recomputed_from_verify_completed` | `s8c`'s affected-set function over a pinned `cargo metadata` fixture — **it must be a pure function of (change set, metadata, policy version)**; if `s8c` built it impure, stop (AC-347 requires *"a pure function"*) | — |
| **AC-349** | `verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both`; `doctor::ac349_equal_specificity_claim_is_a_doctor_row` | a claims binder over each configured toolchain's declared path claims — a `claims` column the verify record adds to `s8c`'s per-toolchain table; the doctor row `RK-D-TOOLCHAIN-CLAIM-TIE` | — |
| **AC-353** | `verify_pass::ac353_killed_verify_resumes_to_the_same_decision` | a real `redkiln` over a fixture whose `[adapters] toolchain_programs` names `rk-fixture-toolchain` rigged to block; the tree killed through `rk-containment`; re-entry reaches the same decision, `reconcile` ran on entry, resume read the tree and the AC-321 replay did not | — |
| **AC-355** | `selection::ac355_lockfile_change_is_whole_suite_and_one_file_is_affected` (Rust adapter) | two fixtures asserted on basis **and** selected-test count | *"held in CI"* is true once `ci.yml` runs green, which is `s8j`'s (D21-42 14a) |
| **AC-359** | `verify_pass::ac359_drift_voids_the_answer_and_recomputes_the_ledger` | a verify-opened AC-357 gate (question `verify.ac-discriminates`); a commit lands between open and answer; the answer is void, the gate re-opens with the drift attached, and the ledger is recomputed | AC-357 is a HUMAN GATE and is **not claimed**; its machine-checkable part (*"the gate node is created with those three payload fields"*) is exercised inside this test and named so |

**Also discharged by the pass, beside the block**: AC-331 (`verify_pass::ac331_timeout_and_kill_are_unverifiable`),
AC-344's `cannot_run` mapping (`verify_pass::ac344_missing_binary_is_cannot_run`), and AC-347's zero-dispatch half
(`verify_pass::ac347_the_pass_dispatches_nothing`).

**AC-347's lint runs over the shipped catalog** (`verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual`).
It walks every `.redkiln/catalog` activity with `stage = "verify"` and asserts a non-zero count walked, which is D21-42
13a's rule. It then asserts that the set carrying an agent step is **exactly** `{migration-dry-run}`, the single
residual D21-46 R5 names (§8g.2). A new agent step at verify turns the test red. So does `integration-tests` back at
verify, and so does removing the residual without updating the test. The test states the residual. It does not hide
it. **Still ARMED, unchanged**: AC-350 (a composite `Toolchain`, R-8), AC-317 (Step 8).

**Why no Phase-1 lane reaches `migration-dry-run`, by quotation** (D21-46 R5: *"no Phase-1 lane reaches it
(`data_migration` is NOT MEASURABLE)"*). Each line below was read at `c5d19d7`:

- **The facet may key no demand.** `.redkiln/facets.toml`'s `[facet.data_migration]` reads *"`deriver_class =
  "lexical"           # a scan over paths and diff lines — may key no require row (AC-520)`"*. Its `deriver_record`
  comment reads *"NOT MEASURABLE at 47faec9f: 0 positives on 20 items across all three sources"*.
  `.redkiln/corpus-records.toml` says the same: *"`# data_migration -- NOT MEASURABLE: 0 positives on 20 items across
  all three sources [F39 s4]`"*.
- **The one demand for its kind was dropped at 7e.** `story.toml` reads *"DROPPED: [[stage.verify.require]] when =
  'data_migration', artifacts = ["MigrationTestReport"]"*. `grep -rn MigrationTestReport .redkiln/packs` finds that
  comment line and no row.
- **Its one consumer is unreached as well.** `migration-expand-execute` `requires` `MigrationTestReport` and provides
  `MigrationLog`. Both packs dropped `MigrationLog`'s release row at 7e (*"DROPPED: [[stage.release.require]] when =
  'data_migration', artifacts = ["MigrationLog"]"*), and no pack row names either kind.

So `migration-dry-run`'s `applicable_if` (*"`data_migration != null and data_migration`"*) is never consulted,
because selection never looks for a `MigrationTestReport` producer.
`selection::migration_dry_run_is_selected_on_no_shipped_lane` asserts the outcome. It selects both shipped packs over
every lattice cell, with `data_migration = true` forced on the fixture item, and asserts that `migration-dry-run` is
selected in none. The AC-347 lint cannot see reachability, so this test is the one that fails if a later row makes
the residual reachable (M22). A reachable residual is no longer a named exception. It is AC-347's breach.

**The mutation harness AC-338 and AC-339 need is ADR-0104 decision 1's, built here** — *"The harness applies the
patch at the candidate SHA, runs the suite, reverts, and records the outcome. It is deterministic and dispatches
nothing."* Two records bar the obvious build and are moved by record, not worked around:

- ADR-0105 clause 5 enumerates the wrapper's writes and says *"Anything not listed is a successor"*; `git apply` is
  not listed, and `crates/rk-git/src/git_exec.rs` says *"Anything not here is a successor record, and adding a
  variant is that decision made visible"*. **R3 (§8g.8) supersedes clause 5 in part** to add `apply` (and `tag`,
  §8g.7). The patch is applied **only** in the pass's ephemeral worktree, never the run-owned one.
  - **R3 is the third record to supersede clause 5 in part, and it cites the other two.** D21-47 gives clause 5 one
    reading: *"A git WRITE not on its closed list is a supersession in part"*, and *"A READ is not a supersession"*.
    `s8d-ii`'s worktree record adds the writes `worktree lock`, `worktree unlock`, the `RK_BASE` marker write,
    `reset --hard` and `clean -fd`. Its reads are not part of the supersession. `s8f`'s landing record adds
    `commit-tree`, `update-ref` and `read-tree HEAD` on the run-owned worktree's own index (`s8f` §8f.3; the third
    added 2026-09-24 by the final consistency pass).
  - R3 cites each of the two records by the id that stage recorded when it wrote the record, never by a title phrase
    (D21-47: *"Records are located by the id recorded at write time"*).
  - R3 then states the consolidated write set: clause 5's list plus the three records' additions. A reader of ADR-0105
    then finds one enumeration and not three partial ones.
- ADR-0162's forbids bullet includes *"forbids applying a mutation patch from `rk-cli`; `ReproKind::Mutate` is refused
  by name until a record adds the capability"*. `s8c`'s 1b records supersede that bullet's seventh-method half and do
  not reach the patch clause. `s8c`'s record V says so: *"The patch half stands here, and `s8g`'s verify record may
  move it for the verify gate's declared AC mutations alone"*. **R1 supersedes the patch clause in part,
  unconditionally, for the verify gate's declared AC mutations alone** (D21-47 review, cross-file finding 19). The
  review pass's `ReproKind::Mutate` arm stays refused by name, and the record says so. ADR-0162 keeps `status:
  accepted` and gains R1's id in `superseded_by`, and Checkpoint 8g (2) asserts both directions.

### 8g.6 The four demands restored (D21-42 6a)

D21-42 6a reads *"Reversed for five demands … **Supersedes D21-41(c)/(d)'s narrowings.**"* The four demands this stage
owns walk shipped catalog activities under `s8d`'s resolver.

**What the three Rust verbs do is `s8c`'s record V. This stage cites it and does not decide it again.** V fixes the
`smoke`, `sca_scan` and `package` handlers, their typed outputs and the scan's semantics. `s8c` §8c.4 reads *"The
scan's semantics are the ones `s8g` §8g.6 reads, and record V fixes them so that the two files agree."*

**The producers record (§8g.8, R4) fixes what is left**: the activities and this repository's data. That is two
step-list edits, the smoke checklist, and the declaration rows `s8c`'s per-toolchain table admits. Each activity edit
is a version bump with a row in `.redkiln/CHANGELOG.md` (ADR-0072).

**SmokeResult — `smoke-test`.** The activity runs `toolchain:build`, then `toolchain:smoke` with *"`checklist =
"checklist:smoke"`"*. The handler runs *"the binaries the call's build produced, never an installed `redkiln`"*
(record V).

- **The checklist is this stage's**, at `.redkiln/checklists/smoke.toml`. Its id is already on the design's checklist
  registry: `spikes/p9/registry.toml`'s `[checklists]` carries `"smoke"`.
- **The registry entry flips with the file** (final consistency pass 2026-09-24; `s8c`'s hand-off). `s8c` declares
  `[checklist.smoke]` in `.redkiln/registry.toml` as `unauthored`, naming this section as the stage that replaces it
  with `path = "checklists/smoke.toml"`. `s8c` reports the gap: *"No line of `s8g` edits the registry entry, so
  `checklist:smoke` would still refuse by name after `s8g`"*. R4's commit makes that replacement beside the file, so
  `checklist:smoke` resolves. `s8c`'s source-table floor (each entry declares exactly one of `path` and `unauthored`,
  and a path names a non-empty file) holds over the flipped entry, and its census counts declared keys, so Checkpoint
  8c does not move. (7h) asserts the entry, and M26 leaves it `unauthored`.
- **Each item takes the form record V reads**: `argv`, `cwd`, `expect_exit` and one stdout predicate. **No item
  dispatches**: no `run`, no network.
- **The first cut.** Each expectation is measured on the built binary before it is written down:
  - `redkiln --version`;
  - `redkiln-migrate --version`;
  - `redkiln --help`, naming the nine verbs (*"The nine verbs, and no others"*, `cli.rs`);
  - `redkiln -C <scratch> init`, then `status`;
  - `redkiln-migrate` with no arguments, exiting clap's `2` (RT-53).
- **The scratch repository sits outside the run-owned worktree.** Every item's `cwd` is `scratch`, a repository
  **outside `.redkiln/worktrees/<item>`** (D21-44). Inside it, D21-42 9's landing would stage the scratch files.
  Where the handler makes the scratch repository is record V's decision. This stage requires that it be outside the
  run tree, and `demands::smoke_result_is_produced_through_the_registry_verb` asserts that.
- **`smoke` is live.** The spike registry marks it *"`live = true`"*, and the design's rule is that a live verb
  *"implies `memo = false` on every step using it"*.

🛑 **A finding, decided by D21-46 R6: `smoke-test` has no check.** At `c5d19d7` its steps are `build`, `smoke` and
`emit`, and its version is 1. So a `SmokeResult` with `failed ≥ 1` is emitted and the stage completes on it. `s8c`'s
draft reads *"The catalog reads `smoke.status == 'passing'`"*, but no step does. D21-46 R6 reads *"**A check on
`status == 'passing'` with `on_false = "fail"`**, as an ADR-0072 edit in s8g."* R4 records it:

- **The step.** A check step with `id = "passing"` sits between `smoke` and `emit`. It has `kind = "check"`, `inputs =
  { smoke = "${smoke.outputs}" }`, `expr = "smoke.status == 'passing'"` and `on_false = "fail"`.
- **ADR-0072.** The version moves 1 → 2, and `.redkiln/CHANGELOG.md` takes a row `smoke-test@2` in `s8b`'s form: the
  date, the ledger line (D21-46 R6), and the old and new text quoted.
- **The rejected alternative stays named.** Keeping the activity emit-only, so that a later stage reads the status,
  was rejected. No stage-exit rule reads a `SmokeResult`'s status, and a measurement whose verdict nobody reads is
  not a gate.
- **The test asserts the outcome, on both arms.** `demands::a_failing_smoke_result_fails_the_stage` walks
  `smoke-test` twice over a `Toolchain` double. With `status = 'passing'`, it emits one `SmokeResult`
  `artifact.write`. With `failed = 1`, the activity fails, no `SmokeResult` is written, and the verify stage does not
  complete, because its baseline `SmokeResult` demand is unmet. M15 deletes the check and M23 inverts it, and each
  arm kills one of them.

**ConfirmationResult — `confirmation-test`.** `toolchain:test` with *"`selection = "artifact:ReproCase.command"`"* goes
through `selection_of` (ADR-0162) and the resolver. The check reads *"`result.status == 'passing'`"*, and a
non-passing result opens *"`question = "confirmation.not-fixed"`"*. The `ReproCase` comes from `minimal-repro` at
`s8d`'s step grain; it has two agent steps, `attempt` and `reduce`. The fixture proves both arms: at the fix sha a
passing result emits, and a result that does not pass opens the gate and emits nothing.

**ScanReport — `sca-scan`.** The activity runs `toolchain:sca_scan`, and its gate is `scan.accept-findings`.

- **The handler's semantics are record V's.** That covers both tools, the CVSS v3 bands, `unrated`, the advisory
  database's commit (or a refusal by name) and the pinned offline database. `s8c` also takes the first measurement of
  the tools' database member. *(dated 2026-09-26, s8c build; D21-63, provisional.)* It includes the `cvss-unscorable`
  refusal: a MATCHED advisory whose vector the in-tree v3 scorer cannot score (`CVSS:4.0`, malformed, any other
  version) refuses the whole scan by name, naming the vector and its two remedies. That refusal is not `unrated` and
  is not routed to `scan.accept-findings`, so D21-43 N25's pass-or-gate routing holds only while no matched advisory
  carries a non-3.x vector. **Precondition, at this stage's open and before this section runs `sca-scan` over the
  real (non-fixture) advisory database:** run `cargo audit --json --no-fetch --file Cargo.lock` against the pinned
  real database and list every MATCHED advisory's vector version (the `cvss` member's `CVSS:<v>/` prefix, or null).
  If any is not `CVSS:3.x`, stop with a NAMED, expected halt, *"sca-scan would refuse cvss-unscorable; rule (3') per
  D21-63"*, before §8g.6 runs, and D21-63's (3') (the `cvss` =2.2.0 v4 scorer on the same bands, `cvss-unscorable`
  kept for rejected vectors) is ruled then. Measured 2026-09-26 at `5561afa`: `vulnerabilities = []`, so no vector
  reached the scorer then; the reading is taken again at the open, never carried forward.
  *(dated 2026-09-26, s8c build, CVSS; D21-64 ruling 7.)* (3') is no longer ruled here: the operator ruled it, and
  `s8c` built it. A `CVSS:4.0` vector is now scored by the `cvss` =2.2.0 crate's v4 scorer on record V's bands and is
  routed like any other scored advisory, so it no longer halts this stage. **The precondition stays, narrowed:** at
  the open, list every matched advisory's vector; the named, expected halt now fires only for a vector neither
  scorer accepts (malformed, or a version other than 3.0, 3.1 and 4.0), with the text *"sca-scan would refuse
  cvss-unscorable naming `<vector>`"*, and the operator rules that vector (correct it, or rule it `unrated`).
  ~~Record V's evidence flags one open reading this stage may meet: a v4.0 vector carrying threat metrics (`E:`) is
  scored as written (CVSS-BT), not base-only.~~
  *(dated 2026-09-26, `s8c` refuting round 3; `spikes/DECISIONS-2026-09-21.md` D21-67 ruling 7.)* That reading is
  closed: *"CVSS v4.0 vectors are scored base-only (CVSS-B): every non-base metric (threat, environmental,
  supplemental) is stripped before scoring"*, built at `s8c`'s close (`verbs::cvss4_tenths`; record V decision 5,
  *"For a `CVSS:4.0` vector, "the CVSS base score" is explicit: it is CVSS-B"*). An `E:` metric no longer moves a
  band, and the halt above still fires only for a vector neither scorer accepts.
- **The activity edit is `s8b`'s, and this stage cites it.** D21-43 N25 reads *"**`sca-scan` requires `unrated ==
  0`** or goes to `scan.accept-findings`"*. As shipped, the check reads *"`report.critical == 0 and report.high ==
  0`"*, which passes with `unrated > 0` (`s8c`'s draft at `a1d35ac`: *"As shipped, that check passes with `unrated >
  0`"*). The edit is `s8b`'s catalog edit, beside its other ADR-0072 edits.
- **What this stage asserts of it.** The producer walk over the edited activity must open the gate on an unrated
  advisory: `demands::scan_unrated_advisory_opens_scan_accept_findings`. If `s8b`'s edit is absent at Checkpoint 8f's
  close, this stage stops (Halt 8) and does not make the edit itself.

**BuildArtifact — `build-package`.** The steps are `toolchain:build`, the `built` check, then `toolchain:package`.

- **The handler is record V's.** It runs a locked release build and takes the plain sha256 of each **declared**
  binary.
- **The declared set is this stage's data.** `package_binaries`, in the Rust row of `s8c`'s per-toolchain table, is
  `redkiln` and `redkiln-migrate`, with its warrant (AC-441: the migrator is *"a separate binary"*). A release build
  also links `rk-fixture-child-cli`, `rk-fixture-toolchain` and `rk-fixture-child-claude`, and none of them is the
  release.
- **The `built` check is now live.** N25 widens `build` to an outcome report. So the check *"`report.status ==
  'passing'`"*, which `s8c` §8c.4 found dead, now reads a red build.

**This repository's `setup` declaration is `s8c`'s `setup_projects` row** (D21-43 N25; D21-45 W3: *"Dependencies
come through N25's per-toolchain `setup` verb"*). This stage depends on it naming
`crates/rk-adapter-toolchain-node/tests/fixtures/project`. This repository's Rust suite includes the Node adapter's
tests, and a fresh run worktree without that fixture's `node_modules` fails ten of them at preflight (`s8c`'s draft
at `a1d35ac`: *"A fresh run-owned worktree runs this repository's suite RED"*). The verb and the row are `s8c`'s, and
running it at creation is `s8d-ii`'s. This stage asserts that the pass's whole suite meets a tree that has been set up
(§8g.2, step 1), and (7e) reads the row as an entry precondition.

**The gate questions load through `s8d`'s registry, and this stage writes no `.redkiln/questions.toml`.**

- **Two questions are already `s8d`'s.** `confirmation.not-fixed` and `scan.accept-findings` are two of the ten
  questions `s8d` transcribes from `spikes/p9/questions.toml` into `.redkiln/processes/`. Its default reading is
  `.redkiln/processes/questions.toml`, which its R5 fixes.
- **One question is new.** `verify.ac-discriminates` (§8g.5) is declared once under `.redkiln/processes/`, in the
  same form, in the file `s8d`'s R5 fixes. `s8d`'s loader loads it on both `run` and `answer`.
- **A test asserts it.** `verify_pass::verify_ac_discriminates_answers_through_the_loaded_processes_registry` answers
  an AC-359 gate through `answer`'s production path, not through `run_with`'s registry parameter.
- **Its gate class needs an expiry row, and that row is `s8h`'s** (D21-47 review, cross-file finding 7). The kernel
  mints the AC-359 gate the pass opens. No catalog step opens it. So `s8h` §8h.10's table carries a
  `verify.ac-discriminates` row in both packs, and `s8h`'s `MINTED_CLASSES` names it, in the spelling this stage's
  minting code fixes. It is handed to `s8h` by name (§8g.11).
- **Checkpoint 8d is provisional** in any case (§8g.1). If `s8d`'s loader test pins the total, that test also takes a
  dated edit.

### 8g.7 `tag-release` — a CLI's release, entered by ADR-0114's trial (D21-42 6b)

**What ReleaseRecord is.** 6b: *"a release tag on the landed commit, plus the BuildArtifact digest and the rollback
target (the previous release tag). The tag is local and never pushed by the run."* The tag record (§8g.8, R5) fixes:

- **The tag** — annotated, named `rk-release/<item>`, on the landed commit D21-41 (b) defines (*"A kernel-made content
  checkpoint commit on `rk-run/<item>`"*). Re-running for the same item at the same commit is idempotent; at a
  different commit it refuses by name.
- **The rollback target** — the nearest `rk-release/*` tag on an ancestor of the landed commit, read through the
  existing `GitOp::Log` (`--decorate-refs=refs/tags/rk-release/ --simplify-by-decoration`), so no new read op is
  owed. **A first release names its target explicitly as `none`**; it is never absent and never defaulted to the
  landed sha.
- **The write** — a tag is a git write, so it is **`GitOp::Tag`**, R3's second addition to ADR-0105 clause 5, and it
  goes through the one wrapper.
- **The verb row is `s8c`'s, and its name is `release_tag`.** `s8c`'s registry lands the row as `release_tag`, with
  route `git` and typed output `ReleaseTag`, *"Refused naming the route until `s8g`"*. The spike registry's
  `release_record` row (*"`outputs = "ReleaseRecord", live = false`"*) is not the shipped row. This stage builds the
  `git`-route handler. It runs **in the composition root, through `rk-git`**, and never in a toolchain adapter
  shelling git (ADR-0105: *"Forbids a wrapper write not on clause 5's list"*). The activity's emit composes the
  `ReleaseRecord` artifact from the `ReleaseTag` value and the BuildArtifact digest.
- **The target must be on the run-owned branch.** The tag target must be a commit on that branch
  (`authorise_tier1_write`'s ownership pair). Any other target refuses.
- **Never pushed** — `GitOp` has no push variant and R3 adds none; a test asserts the write set.

**The activity** is agent-free: `toolchain:release_tag` (inputs `artifact:BuildArtifact`, `artifact:Changeset.sha`),
a check that the record names a tag and a rollback target, and an emit. **Agent-free is the property that makes its
trial cheap**: it cannot dispatch, so its run needs no spend gate.

**How it enters: ADR-0114 decision 3, read whole.** *"An activity is promoted from the `extended` file to the shipped
catalog only with a ledger entry: it must have run at least once … in a run whose id the promotion decision (AC-114's
`curation.decision`) cites … the promotion route is a one-run, one-activity trial the operator declares, not a pack
reference."* And its Negative consequence: *"The extended-promotion route is a small new CLI/config surface (Phase 2,
with `rk-plugin-gen`)."* So:

- **`tag-release` is authored into `.redkiln/catalog-extended/`**, not `.redkiln/catalog/`. AC-110 is unchanged: an
  enabled pack referencing it, or selection reaching it, is a load-time configuration error, and the extended
  directory contributes **zero** entries to the resolved catalog.
- **The trial declaration is data on an item record, not a CLI flag.** It is `[trial] activity = "tag-release"` on
  **each lane item**, as D21-46 R4 confirms: *"**Two trials, confirmed.** Each is one run of one activity, which is
  the clause's constraint."* The draft's *dedicated trial item* is withdrawn (D21-47 review, finding 6).
  - `run` admits exactly that one extended activity, at the release stage, for exactly the declaring item's one run.
  - A run on an item without the declaration never admits it (AC-110).
  - The AC-400 help snapshot does not move.
  - `s8j`'s two lane items carry the declaration (`s8j` §8j.1(e)).
  - R5 records this as the Phase-1 form of the route ADR-0114 placed in Phase 2, with its limits stated.
- **The ledger entry exists already**: AC-532's executed ledger folds `activity_committed` rows into
  `executed: { first, last }` *"per `activity_id@version`, each stamp holding all three of `{ run_id, item, at }`"*.
- **At this stage the trial runs under the fixture harness only**, in two tests:
  - `tag_release::trial_runs_one_activity_with_zero_dispatches` runs in a temporary repository. It asserts one
    activation, zero `dispatch` and `dispatch_completed` rows, a ReleaseRecord `artifact.write`, and a local tag the
    fixture's bare "remote" never receives.
  - `tag_release::a_lane_run_admits_the_trial_activity_only_on_the_declaring_item` runs a fixture lane over two
    items in one temporary repository, under the fixture harness. The item that declares the trial selects and runs
    `tag-release` once at release. The other item's run admits no extended activity.
- **The trial's REAL run on this repository is DEFERRED past this stage, explicitly.** It needs a landed commit and a
  journalled BuildArtifact for its item, which only a lane run produces. It also creates a tag in the operator's
  repository, which is an operator moment, like a push. The real trials are `s8`'s two lane runs, prepared by `s8j`
  (`s8j` §8j.1(e)).
- **The promotion that follows is AC-114's HUMAN GATE. It follows the trials, and it does not precede them.** The
  trial runs are the lane runs themselves (D21-46 R4), so no promotion can come before them. It cites the fix-lane
  run's id, and it moves `.redkiln/catalog` only after `s8`'s runs (`s8j` §8j.1(e): *"It moves `.redkiln/catalog`
  only after `s8`'s runs, so no fork straddles it"*). Until it lands, a lane selects `tag-release` only on an item
  whose `[trial]` declaration names it. The draft's sentence, that the promotion *"must precede any lane run whose
  release stage is to select `tag-release`"*, is withdrawn (D21-47 review, finding 6).
- **AC-114's residue lint does not exist, and its owner is `s8j`.** The residue reads *"the file carries a non-empty
  `curation.decision` id resolving to a record in the catalog's decision log"*. Re-measured at `a6349b6`, `grep -rln
  curation .redkiln crates` finds nothing. It goes to `s8j` with the promotion it checks, for two reasons:
  - The lint's first non-vacuous input is the promotion decision that `s8j`'s real trial produces.
  - Built here, it would either fail all 102 shipped activities, none of which carries a `curation.decision` id, or
    ship with a grandfather rule that no record takes.

  That backfill is a decision owed upward (§8g.11).

**Why selection would pick it — a free residual, and R5 decides it rather than leaving it.** ReleaseRecord's four
shipped producers (`rolling-deploy`, `blue-green-deploy`, `canary-release`, `flag-progressive-rollout`) all run
`toolchain:deploy`, and no existing facet distinguishes a repository whose release is a tag. R5's options: **(a)** a
`declared` facet (the `surfaces` precedent: *"read from a checked-in declaration map"*) keyed in `tag-release`'s
`applicable_if` — `applicable_if` is not a `require` row, so a declared facet may key it — with `tag-release` first in
the `deploy-pattern` family, and **no shipped deploy activity edited**; **(b)** verb availability as a selection-time
admissibility condition, which is the long-term shape and is rejected **here** because it moves every stage's
selection on the lane `s8` runs. Recommended: (a). Checkpoint 8g asserts the **outcome**
(`tag_release::selection_picks_tag_release_over_the_deploy_patterns`), not the mechanism.

**The release goal is amended, not read.** bug-standard says *"The fix is in production reversibly"* and story says
*"In production, reversibly"*. A run that makes a local tag and pushes nothing puts nothing in production, so a
reading would leave a false sentence in the pack. R5 amends them to **`The fix is tagged for release, reversibly`** and
**`Tagged for release, reversibly`**. The packs carry no `version` key (measured), and ADR-0072 governs activity edits,
so no version moves; the edit is listed in the pack changelog `s8b` opened for D21-42 11a — `s8b` §8b.5 creates
`.redkiln/CHANGELOG.md` and says *"Nothing names the file, so the path is chosen here and said"*. **If that file is
absent at Checkpoint 8f's close, stop**: this stage does not choose a second path. The `tag-release` addition takes
its row there too.

### 8g.8 The records this stage writes

Each is written at the next free id **at writing**, `proposed`, in ADR-0085's form with `### Option — <name>`
headings (D21-42 24a), and a `## Consequences` carrying **What this forbids**. Each takes the slug fixed here. **The id
each record takes is recorded when it is written**, by a dated edit of the five `R<n>_ID` lines in Checkpoint 8g (2),
and the checkpoint locates each record by that id and then checks its slug. D21-47: *"Records are located by the id
recorded at write time"*. The table:

| | slug | decides | supersedes, amends |
| --- | --- | --- | --- |
| R1 | `the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed` | §8g.2, §8g.4, §8g.5: the pass and its composition, `setup` before the whole suite; `KindSource::Pass`; D21-43 N23's re-homing of the four displaced providers to a test-authoring kind at implement, with the escalation row; D21-46 R5's re-homing of `integration-tests` with `story.toml`'s `IntegrationReport` row, and `migration-dry-run` named as AC-347's single residual, its unreachability quoted; `verify_completed`; the verify block's routing, each id's instrument or reason; the cold slowest-child figure cited from `s8c`; the claims column; the mutation harness | **ADR-0162's patch clause in part, unconditionally** (§8g.5; ADR-0162 gains this id in `superseded_by`); R-9 and RT-6 discharged; register: AC-473 *Verified by* (10b), reading only `origin = "pass"` rows; dated notes on AC-320, AC-485, and AC-341 (its CI-green half is Phase 3's); design §5's table row; `verify_completed` named item-scoped for `s8e`'s list; D17-4 closed. It moves `integration-tests` whole, where D21-46 R5's letter names its *"test-writing steps"*, on D21-48 X9's decision *"**The whole activity moves to implement**, because stage membership is per activity."*, quoting both (§8g.2; D21-48 (2026-09-24), which replaces the flag for its flip line) |
| R2 | `the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion` | §8g.3, including AC-139's default: block, fail-closed, recorded as the human decision (D21-43 N24) and shipped as `verification_required = ["verify"]` in both packs; the AC-133 registry, which `s8h` consumes | D17-3 closed; AC-139 dated note; `PROGRESS.md` row 5 corrected in the tracker commit. **It names the `verification` gap** that N24's row opens over agent-authored `Spec`s, and its owners, `s8h` and `s8j` (§8g.3) |
| R3 | `the-rk-git-write-set-gains-tag-and-apply` | `GitOp::Tag`, `GitOp::Apply`, each with its scope; the consolidated clause-5 write set | **ADR-0105 clause 5 in part**; ADR-0105 keeps `status: accepted` and gains this id in `superseded_by`. It cites `s8d-ii`'s worktree record and `s8f`'s landing record, the other two partial supersessions of clause 5, by their recorded ids (§8g.5) |
| R4 | `the-four-verify-and-release-demands-have-producers` | §8g.6: the activities (`smoke-test`'s `passing` check, as D21-46 R6 decided; the producer walks), the smoke checklist's home and shape and its registry entry's flip from `unauthored` to its `path`, and this repository's `package_binaries` row. **The three verb handlers and their semantics are `s8c`'s record V, and `sca-scan`'s `unrated` edit is `s8b`'s; each is cited by id and not decided again** | executes D21-42 6a (which supersedes D21-41 (c)/(d)'s narrowings) |
| R5 | `tag-release-is-the-cli-release-and-enters-by-trial` | §8g.7: ReleaseRecord's shape, the tag name and rollback target, the `release_tag` git-route handler (`s8c`'s row), the extended directory, the item-record trial on each lane item, the promotion after the trials, the selection mechanism, the goal amendment, AC-114's lint named as `s8j`'s | executes D21-42 6b under ADR-0114 decision 3, and D21-46 R4's two trials |

🛑 **A change to a `## Decision` sentence or a forbids bullet is a supersession, never an erratum** (ADR-0092 rule 1).
Two records here supersede in part. R3 moves ADR-0105 clause 5, and R1 moves ADR-0162's patch-clause forbids bullet.
Each supersession is recorded in both directions, and Checkpoint 8g (2) asserts all four links. *(Amended 2026-09-24,
D21-47: R1's disposition was conditional, and is now unconditional.)*

### 8g.9 Red first — the named tests and the mutants

Every test below is authored, run, and **watched fail for its own reason** before the code that makes it pass —
golden constraint 12 — and the red output is quoted in the commit that lands it. **Sixty-six new tests**, all under
`--test it`: `rk-types` 9, `rk-kernel` 12, `rk-git` 4, `rk-adapter-toolchain-rust` 2, `rk-cli` 39. *(Amended
2026-09-24, D21-47: `verify::verify_completed_carries_its_origin_and_stage` and
`tag_release::a_lane_run_admits_the_trial_activity_only_on_the_declaring_item` added.)* Checkpoint 8g
asserts each by its module-qualified name and asserts the exact count per crate; the lists are there and are not
repeated here. The draft's eight handler tests (`registry::`, `sca::`, `package::`, `smoke::` in the Rust adapter)
left this list because the handlers are `s8c`'s record V, and `s8c`'s own `verbs::` tests pin them.
*(dated 2026-09-26, `s8c` build, D67; D21-67 ruling 6.)* One more is owed here, not yet counted in the sixty-six:
`rk-kernel`'s `lattice::the_report_branches_fixture_exercises_every_unsat_branch` and its five mutants RB1–RB5 (§8g.2's
named precondition of the lattice re-run). This stage's open adds it to the count, the table below and Checkpoint 8g
(5) by dated edit.

**The mutants**, each authored, run **from Python and never from the shell** (a backtick once swallowed a restore),
and each must turn its named test red. A survivor blocks the close.

| # | mutant | must turn red |
| --- | --- | --- |
| M1 | the pass records the configured list's first toolchain instead of the one `detect()` chose | `verify_pass::verify_completed_names_the_adapter_detect_chose` |
| M2 | `Ok` with `failed > 0` mapped to green | `verify_pass::red_suite_is_a_failed_verify_never_green` |
| M3 | `Timeout` mapped to a failed verify | `verify_pass::ac331_timeout_and_kill_are_unverifiable` |
| M4 | the pass keyed on `resolved.stage == "verify"` | `verify_pass::selected_by_required_kind_not_stage_name` (its fixture pack names the stage `check`) |
| M5 | `verify_completed` written for a killed run | `verify::no_verify_completed_for_a_run_that_did_not_complete` |
| M6 | `paths` dropped from the payload, recomputed from the tree on replay | `verify::ac321_replay_reads_the_path_list_from_verify_completed_alone` |
| M7 | `sca-scan`'s check restored to its v1 expression, without `report.unrated == 0` (a mutant over `s8b`'s edit, killed by this stage's walk) | `demands::scan_unrated_advisory_opens_scan_accept_findings` |
| M8 | `bug-standard.toml`'s `verification_required` row deleted | `verify_pass::ac139_shipped_packs_block_a_missing_verification` |
| M9 | `package_binaries` gains `rk-fixture-toolchain` | `demands::build_artifact_carries_release_digests` (it asserts the exact declared set) |
| M10 | the rollback target defaulted to the landed sha | `tag_release::rollback_target_is_the_previous_release_tag` |
| M11 | the post-run content hash compare removed | `verify_pass::ac338_restored_mutation_is_unverifiable` |
| M12 | the patch applied in the run-owned worktree | `verify_pass::ac339_mutation_runs_in_a_disposable_worktree` |
| M13 | an equal-specificity tie resolved to the first claim | `verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both` |
| M14 | the affected set built by iterating a `HashMap` | `selection::ac348_selection_is_byte_identical_over_200_change_sets` (its generator permutes input order) |
| M15 | `smoke-test`'s `passing` check deleted | `demands::a_failing_smoke_result_fails_the_stage` |
| M16 | `KindSource::Pass` read as `Activity` by the selector | `selection::a_pass_sourced_kind_selects_no_catalog_producer` |
| M17 | the pass calls `test` on a fresh run worktree without `setup` having run | `verify_pass::setup_precedes_the_whole_suite_in_a_fresh_run_worktree` |
| M18 | `unit-tests` restored to `stage = "verify"` | `selection::the_rehomed_test_authoring_activities_select_at_implement` |
| M19 | an agent step added to `smoke-test` | `verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual` |
| M20 | `integration-tests` restored to `stage = "verify"` | `verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual` |
| M21 | `story.toml`'s `external_dep` → `IntegrationReport` row moved back to `[stage.verify]` | `selection::integration_tests_selects_at_implement_under_external_dep` |
| M22 | a `[[stage.verify.require]]` row `when = "external_dep"`, `artifacts = ["MigrationTestReport"]` added to `story.toml` | `selection::migration_dry_run_is_selected_on_no_shipped_lane` |
| M23 | `smoke-test`'s `passing` expression inverted to `smoke.status != 'passing'` | `demands::a_failing_smoke_result_fails_the_stage` (its passing arm) |
| M24 | `verify_completed`'s `origin` written as `"pass"` for every call | `verify::verify_completed_carries_its_origin_and_stage` (its `red-verify` arm) |
| M25 | the trial admission ignores the item, and admits `tag-release` on any run | `tag_release::a_lane_run_admits_the_trial_activity_only_on_the_declaring_item` |
| M26 | `[checklist.smoke]` left `unauthored` in `.redkiln/registry.toml` (final consistency pass 2026-09-24) | `demands::smoke_result_is_produced_through_the_registry_verb` (its `smoke` step refuses by name on `checklist:smoke`) |

### 8g.10 Checkpoint 8g

**What this checkpoint asserts:**

- **Five records exist, each once, in ADR-0085's form.**
  - Each is located by the id recorded when it was written, and then checked by its slug (D21-47).
  - R3's supersession of ADR-0105 is recorded in both directions, and R3 states the consolidated clause-5 write set.
  - R1's supersession of ADR-0162's patch clause is recorded in both directions.
  - R1 executes N23's re-homing and names the AC-347 residual.
  - R2 records AC-139's fail-closed decision.
- **`verify_completed` is a tier-2 kind** in `rk-types` and in design §5's table. AC-473's *Verified by* names it, on
  a dated line, and reads only its `origin = "pass"` rows. AC-320, AC-341, AC-139 and AC-485 carry their dated notes.
- **The schema.** `AcceptanceCriterion` is exported and snapshotted, AC-133's registry exists and covers it, and
  `SpecAc` is gone.
- **The verify pass is the one TestReport route.** It is reached by `verbs::run::run`, selected by kind, and runs
  whole-suite through the port over a tree `setup` has prepared. Every id of the verify block has its named test.
- **The re-homing and the policy rows.** The four displaced providers are re-homed to implement over one
  test-authoring kind, and the escalation row moved with them. `integration-tests` is re-homed over the same kind,
  and `story.toml`'s `IntegrationReport` row moved with it (D21-46 R5). `migration-dry-run` is AC-347's single
  residual, and no shipped lane selects it. Both packs ship `verification_required = ["verify"]`.
- **The four demands have producers**, and each writes one `artifact.write`. `[checklist.smoke]` names its file by
  `path` (final consistency pass 2026-09-24). The producer walk over `s8b`'s `sca-scan` reads `unrated`. The gate
  questions load from `s8d`'s registry, and there is no `.redkiln/questions.toml`. `smoke-test` fails on a
  `SmokeResult` that is not passing (D21-46 R6).
- **`tag-release`** sits in the extended directory and trials under the fixture harness with zero dispatches. **No
  `rk-release/*` tag appears in this repository** as a side effect of the block.
- **The whole suite, fmt and clippy are green.** Checkpoints 5, 6, 7b, 7b-ii, 7d, 8b, 8d, 8d-ii, 8e and 8f re-run
  whole from their stage files' own bytes. The re-runs are bounded. They and this block's own cargo test runs go
  through D21-47's no-dispatch wrapper, and none of them spawns a real `claude`.

**BEFORE** (PIA-035). Each of these fails on the untouched tree:

- (0a) finds no code line naming the mutation tree `{item}.mut`.
- (2) finds no recorded id: the five `R<n>_ID` lines are empty.
- (3) finds no `verify_completed` row, and no dated note on AC-473, AC-320, AC-341, AC-139 or AC-485.
- (4) finds **17** `SpecAc` sites (re-measured at `a6349b6`).
- (7) finds four `TestReport` providers at `stage = "verify"` and no `verification_required` row.
- (7g) finds `integration-tests` at `stage = "verify"` and version 2, and `smoke-test` at version 1 with no `passing`
  step.
- (7h) finds `[checklist.smoke]` declared `unauthored` (`s8c`'s entry).

**Checkpoint 7c is not re-run here.** Its live probe is `s8j`'s (§8g.1).

**The floor is re-derived at stage open.** (9)'s `PREV` is Checkpoint 8f's recorded passed count. It is written in by
a dated edit before the block's first run and never lowered to make a red run green. The expected value,
**re-derived at stage open from Checkpoint 8f's recorded count**, is PREV = **1123**, `s8f`'s Checkpoint 8f `FLOOR`
(`s8f` §8f.2 item 9: *"That gives 740 + 9 + 25 + 78 + 62 + 50 + 74 = 1038 at Checkpoint 8e, and 1038 + 85 =
**1123** here"*), so (9)'s floor is 1123 + 66 = **1189**. The chain, recomputed once, is ADR-0164's 740 passed at
`992d257`, plus `s8a` 9, `s8b` 25, `s8c` 78, `s8d` 62, `s8d-ii` 50, `s8e` 74 and `s8f` 85, each as its file now states
its new tests. The recorded count governs. *(Final consistency pass 2026-09-24. It supersedes PREV = 1063 and a floor
of 1129, which took the predecessors' counts before their D21-47 fixes. D21-48 (2026-09-24): `s8f`'s X11 test moves
PREV from 1105 to 1106 and the floor from 1171 to 1172. *D21-48 (2026-09-24), floor propagation:* `s8c`'s 78, `s8d`'s
62 and `s8d-ii`'s 50 move PREV from 1106 to 1123 and the floor from 1172 to 1189.)*

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`,
# MODULE-QUALIFIED and `--exact`, and asserts each test's own `... ok` line AND an exact count: a bare name
# under `--exact` runs zero tests and exits 0.

# ---- (0) THE WORKTREE PATH SET AND THE RELEASE-TAG SET, read FIRST, before any cargo run.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
tags_before=$(git tag -l 'rk-release/*'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git tag -l could not run (rc=$rc)"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: run 'pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project' first — ten node tests fail at preflight without it and the whole-suite line would be red for a reason unrelated to this stage"; exit 1; }

# ---- (0a) THE MUTATION TREE'S NAME AND ITS WORST-CASE PATH (§8g.5; D21-47 review, finding 8). The tree is
# ---- `<item>.mut`, beside `<item>.review-ref` under the run-worktree root, so the longer of the two suffixes bounds
# ---- the sum Checkpoint 8d-ii (0b) takes. The name sweep is a floor over CODE lines beneath ac339's path assertion.
raw=$(grep -rnF '{item}.mut' crates --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the mutation-tree name sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
test -n "$code" || { echo "FAIL: no code line under crates/ names the mutation tree '{item}.mut' — the name s8g section 8g.5 fixes is not the one built"; exit 1; }
top=$(git rev-parse --show-toplevel); rc=$?
[ "$rc" -eq 0 ] && [ -n "$top" ] || { echo "FAIL: git rev-parse --show-toplevel failed (rc=$rc)"; exit 1; }
files=$(git ls-files); rc=$?
[ "$rc" -eq 0 ] && [ -n "$files" ] || { echo "FAIL: git ls-files failed or listed nothing (rc=$rc)"; exit 1; }
longest=$(printf '%s\n' "$files" | awk '{ if (length($0) > m) m = length($0) } END { print m + 0 }')
[ "$longest" -gt 0 ] || { echo "FAIL: the longest tracked path read as $longest — the length guard would be about nothing"; exit 1; }
mut_sfx=.mut; ref_sfx=.review-ref
sfx=${#ref_sfx}; [ "${#mut_sfx}" -le "$sfx" ] || sfx=${#mut_sfx}
[ "$sfx" -le 11 ] || { echo "FAIL: the longest sibling-tree suffix is $sfx characters, past the 11 that s8d-ii's worst-case sum reserves"; exit 1; }
prefix="$top/.redkiln/worktrees/"
worst=$(( ${#prefix} + 64 + sfx + 1 + longest ))
lp=$(git config --get core.longpaths); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: git config could not be read (rc=$rc)"; exit 1; }
[ "$worst" -le 259 ] || [ "$lp" = "true" ] || { echo "FAIL: the worst-case mutation-tree path is $worst characters (longest tracked path $longest) and core.longpaths is not true"; exit 1; }
# CONTROL: the same sum over a 200-character path exceeds the limit, so the line above can go red.
ctl=$(( ${#prefix} + 64 + sfx + 1 + 200 ))
[ "$ctl" -gt 259 ] || { echo "FAIL: CONTROL — a 200-character path summed to $ctl, within the limit, so the length guard cannot fail"; exit 1; }

# ---- (0b) THE NO-DISPATCH WRAPPER (D21-47), set up BEFORE any process-level cargo run and used by every one below —
# ---- the build (1), the named selections (5), the whole suite (9), fmt and clippy (10) — and by (12)'s replays. It is
# ---- the shared helper, sourced, never retyped (final consistency pass 2026-09-24): nd_setup compiles a fake `claude`
# ---- (claude.exe on Windows: a Rust Command::new("claude") resolves only claude.exe, so a shell-script fake would
# ---- leave the real binary to answer) that prints the pin for --version and logs and refuses anything else; strips
# ---- every other PATH directory holding a claude; unsets REDKILN_LAUNCH_RECORD; points CLAUDE_CONFIG_DIR at an empty
# ---- trap; and runs its own controls, checking resolution before the fake's first call. In this session
# ---- CLAUDE_CONFIG_DIR is the operator's real account. Inside an enclosing wrapper (this block replayed by s8h) it
# ---- reuses that one. It carries no card or permission precondition, so it is set up whether or not RK_CP_NESTED is.
mkdir -p target/cp8g || { echo "FAIL: could not create target/cp8g"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh could not be sourced from the repo root"; exit 1; }
nd_setup "$PWD/target/cp8g/nd" || exit 1
# This block's own control, beyond the helper's: a planted launch record does not reach a wrapped process.
got=$(REDKILN_LAUNCH_RECORD=planted nodispatch sh -c 'printf %s "${REDKILN_LAUNCH_RECORD-unset}"'); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = unset ] || { echo "FAIL: CONTROL — the wrapper passed REDKILN_LAUNCH_RECORD through ('$got', rc=$rc)"; exit 1; }
nd_check "the wrapper's controls" || exit 1

# ---- (1) BUILD-OR-FAIL, before anything reads target/ (Halt 1), under (0b)'s wrapper.
out=$(nodispatch cargo build --workspace 2>&1); rc=$?
nd_check "(1)'s build" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8g NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE FIVE RECORDS, EACH LOCATED BY THE ID RECORDED WHEN IT WAS WRITTEN (D21-47), THEN CHECKED BY ITS SLUG,
# ---- EACH EXACTLY ONCE AND IN ADR-0085's FORM (D21-42 24a). This file pre-assigns no number: each R<n>_ID line
# ---- below takes a dated edit when its record is written. BEFORE: all five are empty, and the loop fails on the first.
R1_ID=""
R2_ID=""
R3_ID=""
R4_ID=""
R5_ID=""
for spec in "R1@$R1_ID@the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed" "R2@$R2_ID@the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion" "R3@$R3_ID@the-rk-git-write-set-gains-tag-and-apply" "R4@$R4_ID@the-four-verify-and-release-demands-have-producers" "R5@$R5_ID@tag-release-is-the-cli-release-and-enters-by-trial"; do
  r=${spec%%@*}; rest=${spec#*@}; num=${rest%%@*}; slug=${rest#*@}
  case "$num" in [0-9][0-9][0-9][0-9]) ;; *) echo "FAIL: ${r}_ID is '$num', not a four-digit id — record $r's id was not recorded when it was written (s8g section 8g.8)"; exit 1 ;; esac
  f="references/adr/$num-$slug.md"
  test -f "$f" || { echo "FAIL: record $r's recorded id $num does not carry the slug $slug (no $f)"; exit 1; }
  set -- references/adr/[0-9][0-9][0-9][0-9]-"$slug".md
  [ "$#" -eq 1 ] && [ "$1" = "$f" ] || { echo "FAIL: the slug $slug is carried by more than one record, or not by $num alone: $*"; exit 1; }
  grep -qxF "id: \"$num\"" "$f" || { echo "FAIL: $f's frontmatter id is not $num"; exit 1; }
  grep -qxE 'status: (proposed|accepted)' "$f" || { echo "FAIL: $f is neither proposed nor accepted"; exit 1; }
  n=$(grep -c '^### Option — ' "$f")
  [ "$n" -ge 2 ] || { echo "FAIL: $f carries $n '### Option — ' headings; ADR-0085 and D21-42 24a need the considered options in that form"; exit 1; }
  grep -qF '**What this forbids**' "$f" || { echo "FAIL: $f has no What this forbids block"; exit 1; }
done
wf="references/adr/$R3_ID-the-rk-git-write-set-gains-tag-and-apply.md"
vf="references/adr/$R1_ID-the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed.md"
grep -qE '^supersedes: \[.*"0105".*\]$' "$wf" || { echo "FAIL: the write-set record does not list 0105 in supersedes — a new wrapper write without one is ADR-0105's forbids bullet broken"; exit 1; }
grep -qE "^superseded_by: \[.*\"$R3_ID\".*\]$" references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md || { echo "FAIL: ADR-0105's superseded_by does not carry $R3_ID — the supersession is recorded in one direction only"; exit 1; }
# R3 states the consolidated clause-5 write set: its own two writes and the other two records' (D21-47's one reading).
for w in 'commit-tree' 'update-ref' 'read-tree' 'worktree lock' 'worktree unlock' 'reset --hard' 'clean -fd'; do
  grep -qF "$w" "$wf" || { echo "FAIL: the write-set record does not state the consolidated clause-5 write set: '$w' is absent (s8g section 8g.5; D21-47)"; exit 1; }
done
grep -qE '^supersedes: \[.*"0162".*\]$' "$vf" || { echo "FAIL: the verify record does not list 0162 in supersedes — ADR-0162's patch clause is superseded unconditionally (s8g section 8g.5; D21-47)"; exit 1; }
grep -qE "^superseded_by: \[.*\"$R1_ID\".*\]$" references/adr/0162-the-review-call-site-is-built-over-a-fixture-corpus-and-guard-10-becomes-behavioural.md || { echo "FAIL: ADR-0162's superseded_by does not carry $R1_ID — the supersession is recorded in one direction only"; exit 1; }
grep -qF 'applying a mutation patch from `rk-cli`' "$vf" || { echo "FAIL: the verify record does not quote ADR-0162's patch clause — its disposition is stated nowhere (s8g §8g.5)"; exit 1; }
# D21-48 X9 (2026-09-24) decided the whole-activity move: the record quotes the decision AND the letter it reads.
grep -qF 'test-writing steps' "$vf" || { echo "FAIL: the verify record does not quote D21-46 R5's 'test-writing steps' — the letter D21-48 X9 reads is not stated (s8g section 8g.2)"; exit 1; }
X9_NEEDLE='stage membership is per activity'
grep -qF "$X9_NEEDLE" "$vf" || { echo "FAIL: the verify record does not quote D21-48 X9 ('$X9_NEEDLE') — its whole-activity move of integration-tests cites no decision (s8g section 8g.2)"; exit 1; }
# CONTROLS: the ledger's own X9 row satisfies the needle (a mistyped needle would make the guard unsatisfiable), and a
# record quoting only R5's letter, with the old flag, does not.
grep -F '| X9 |' spikes/DECISIONS-2026-09-21.md | grep -qF "$X9_NEEDLE" || { echo "FAIL: CONTROL — D21-48's X9 row does not carry the needle '$X9_NEEDLE', so the guard above cannot be satisfied by a faithful quotation"; exit 1; }
ctl=$(printf '%s\n' "D21-46 R5 re-homes the test-writing steps; flagged for the flip line." | grep -cF "$X9_NEEDLE")
[ "$ctl" -eq 0 ] || { echo "FAIL: CONTROL — R5's letter alone satisfied D21-48 X9's needle"; exit 1; }
grep -qF 'Re-homed to the implement stage' "$vf" || { echo "FAIL: the verify record does not quote D21-43 N23 — the displaced providers' re-homing is executed nowhere (s8g §8g.2)"; exit 1; }
for a in integration-tests migration-dry-run; do
  grep -qF "$a" "$vf" || { echo "FAIL: the verify record does not name $a — D21-46 R5 re-homes the one and names the other as AC-347's residual (s8g §8g.2, §8g.5)"; exit 1; }
done
grep -qF 'no Phase-1 lane reaches it' "$vf" || { echo "FAIL: the verify record does not quote D21-46 R5 — migration-dry-run's residual carries no reason (s8g §8g.5)"; exit 1; }
sf="references/adr/$R2_ID-the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion.md"
grep -qF 'Block (fail-closed)' "$sf" || { echo "FAIL: the schema record does not quote D21-43 N24 — AC-139's human decision is recorded nowhere (s8g §8g.3)"; exit 1; }

# ---- (3) verify_completed: THE DESIGN ROW (ADR-0116 decision 5), then the dated register notes. AC-473's Verified by
# ---- names the kind (D21-42 10b) and reads only its origin = "pass" rows (D21-47 review, finding 4). AC-320's note
# ---- substitutes the port-call count, AC-341's CI-green half is Phase 3's, AC-139 names N24, and AC-485's toolchain
# ---- half reads verify_completed. The marker and a date must sit on ONE line of the AC's own block, so a date
# ---- elsewhere in the block cannot satisfy it.
grep -qE '^\| `verify_completed` \| 2 \|' references/2026-09-08-architecture-design.md || { echo "FAIL: design §5's event table has no tier-2 verify_completed row — the variant and the row land in one change"; exit 1; }
for spec in "473@verify_completed" "320@port-call" "341@Phase 3" "139@N24" "485@verify_completed"; do
  ac=${spec%%@*}; want=${spec#*@}
  blk=$(awk -v s="**AC-$ac** " 'index($0,s)==1{p=1;print;next} p&&/^\*\*AC-[0-9]+\*\* /{exit} p' references/2026-09-08-acceptance-criteria.md)
  test -n "$blk" || { echo "FAIL: AC-$ac's block could not be read from the register"; exit 1; }
  hit=$(printf '%s\n' "$blk" | grep -F "$want")
  test -n "$hit" || { echo "FAIL: AC-$ac's block carries no line naming '$want' (s8g §8g.3, §8g.5)"; exit 1; }
  printf '%s\n' "$hit" | grep -qE '20[0-9]{2}-[0-9]{2}-[0-9]{2}' || { echo "FAIL: AC-$ac's '$want' line is undated — a register amendment carries its date"; exit 1; }
done
blk=$(awk -v s="**AC-473** " 'index($0,s)==1{p=1;print;next} p&&/^\*\*AC-[0-9]+\*\* /{exit} p' references/2026-09-08-acceptance-criteria.md)
test -n "$blk" || { echo "FAIL: AC-473's block could not be read from the register"; exit 1; }
printf '%s\n' "$blk" | grep -qF 'origin' || { echo "FAIL: AC-473's amendment does not restrict its identity reading to origin = pass rows — an implement-stage row could satisfy it (s8g section 8g.4)"; exit 1; }

# ---- (4) THE SCHEMA: AcceptanceCriterion exported, snapshotted and tracked; AC-133's registry; SpecAc GONE.
snap=crates/rk-types/tests/it/snapshots/exported__AcceptanceCriterion.snap
test -f "$snap" || { echo "FAIL: no AcceptanceCriterion snapshot — the type is not in registry::EXPORTED"; exit 1; }
git ls-files --error-unmatch "$snap" >/dev/null 2>&1 || { echo "FAIL: the snapshot exists and is not tracked"; exit 1; }
pending=$(find crates/rk-types/tests/it/snapshots -name '*.snap.new'); rc=$?
[ "$rc" -eq 0 ] && [ -z "$pending" ] || { echo "FAIL: an unaccepted snapshot is pending — a schema change nobody reviewed:"; printf '%s\n' "$pending"; exit 1; }
# CODE LINES ONLY, anchored to grep's `path:LINE:` prefix. BEFORE: 17 sites at a6349b6.
raw=$(grep -rnE '\bSpecAc\b' crates --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the SpecAc sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: the kernel-local SpecAc survives on a code line (D21-42 10a retires it):"; printf '%s\n' "$code"; exit 1; }
# Synthetic control: the same filter KEEPS a code line and DROPS a comment line, or silence above proves nothing.
ctl=$(printf '%s\n%s\n' 'crates/x/src/a.rs:1:pub struct SpecAc {' 'crates/x/src/a.rs:2:/// SpecAc was here' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: the code-line filter's control kept $ctl of 2 lines, expected exactly the code line"; exit 1; }

# ---- (5) THE NAMED TESTS, per crate, each by its own `... ok` line and an EXACT count.
T_TYPES="event_kinds::verify_completed_is_tier_two_and_wired verify_types::verify_completed_payload_refuses_an_absent_member acceptance_criterion::ac137_criterion_without_gwt_or_verification_is_accepted acceptance_criterion::mutation_declaration_round_trips_declared_and_not_feasible acceptance_criterion::a_prose_only_mutation_is_refused_at_ingest acceptance_criterion::acceptance_criterion_is_in_the_exported_registry judgement_registry::ac133_every_exported_enum_is_registered judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails judgement_registry::the_registry_names_no_enum_that_is_not_exported"
T_KERNEL="verify::verify_completed_is_journalled_once_per_completed_test_port_call verify::no_verify_completed_for_a_run_that_did_not_complete verify::ac321_replay_reads_the_path_list_from_verify_completed_alone verify::ac321_replay_property_over_100_synthetic_ranges verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual verify::the_ledger_reads_rk_types_acceptance_criterion verify::verify_completed_carries_its_origin_and_stage selection::a_pass_sourced_kind_selects_no_catalog_producer selection::the_rehomed_test_authoring_activities_select_at_implement selection::integration_tests_selects_at_implement_under_external_dep selection::migration_dry_run_is_selected_on_no_shipped_lane"
T_GIT="gitops::tag_is_on_the_enumerated_write_set gitops::apply_is_on_the_enumerated_write_set gitops::the_write_set_has_no_push gitops::tag_refuses_a_target_off_the_run_owned_branch"
T_RUST="selection::ac348_selection_is_byte_identical_over_200_change_sets selection::ac355_lockfile_change_is_whole_suite_and_one_file_is_affected"
T_CLI="verify_pass::selected_by_required_kind_not_stage_name verify_pass::writes_one_keyed_test_report_with_one_artifact_write verify_pass::verify_completed_names_the_adapter_detect_chose verify_pass::node_detected_fixture_records_the_node_adapter verify_pass::red_suite_is_a_failed_verify_never_green verify_pass::ac331_timeout_and_kill_are_unverifiable verify_pass::ac344_missing_binary_is_cannot_run verify_pass::ac347_the_pass_dispatches_nothing verify_pass::ac320_unresolved_inputs_refuse_before_any_port_call verify_pass::ac338_restored_mutation_is_unverifiable verify_pass::ac339_mutation_runs_in_a_disposable_worktree verify_pass::ac341_the_pass_calls_no_tracker_method verify_pass::ac348_selection_is_recomputed_from_verify_completed verify_pass::ac353_killed_verify_resumes_to_the_same_decision verify_pass::ac359_drift_voids_the_answer_and_recomputes_the_ledger verify_pass::ac485_config_edit_mid_run_does_not_move_the_verify_adapter verify_pass::the_lane_reaches_the_pass_through_verbs_run_run verify_pass::setup_precedes_the_whole_suite_in_a_fresh_run_worktree verify_pass::ac139_shipped_packs_block_a_missing_verification verify_pass::verify_ac_discriminates_answers_through_the_loaded_processes_registry demands::smoke_result_is_produced_through_the_registry_verb demands::a_failing_smoke_result_fails_the_stage demands::confirmation_result_passes_at_the_fix_sha demands::confirmation_not_fixed_opens_its_gate demands::scan_high_finding_opens_scan_accept_findings demands::scan_unrated_advisory_opens_scan_accept_findings demands::scan_clean_emits_without_a_gate demands::build_artifact_carries_release_digests demands::every_restored_demand_writes_one_artifact_write tag_release::release_record_tags_the_landed_commit_locally tag_release::rollback_target_is_the_previous_release_tag tag_release::first_release_names_its_rollback_target_as_none tag_release::release_record_carries_the_build_artifact_digest tag_release::the_tag_is_never_pushed tag_release::trial_runs_one_activity_with_zero_dispatches tag_release::a_lane_run_admits_the_trial_activity_only_on_the_declaring_item tag_release::an_enabled_pack_referencing_the_extended_activity_is_refused tag_release::selection_picks_tag_release_over_the_deploy_patterns doctor::ac349_equal_specificity_claim_is_a_doctor_row"
NEW=0
for spec in "rk-types@9@$T_TYPES" "rk-kernel@12@$T_KERNEL" "rk-git@4@$T_GIT" "rk-adapter-toolchain-rust@2@$T_RUST" "rk-cli@39@$T_CLI"; do
  crate=${spec%%@*}; rest=${spec#*@}; want=${rest%%@*}; names=${rest#*@}
  got=$(printf '%s\n' $names | grep -c .)
  [ "$got" -eq "$want" ] || { echo "FAIL: $crate's list carries $got names, this file says $want — the list and the prose disagree"; exit 1; }
  out=$(nodispatch cargo test -p "$crate" --test it -- $names --exact 2>&1); rc=$?
  nd_check "(5)'s $crate selection" || exit 1
  [ "$rc" -eq 0 ] || { echo "FAIL: $crate's named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in $names; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $crate: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $crate's selection did not run exactly $want tests — a filter that matches nothing is a green"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
  NEW=$((NEW + want))
done
[ "$NEW" -eq 66 ] || { echo "FAIL: $NEW named tests ran, §8g.9 says sixty-six"; exit 1; }

# ---- (6) NO STAGE-NAME LITERAL IN rk-cli/src (ADR-0162 decision 5) — a FLOOR over CODE lines beneath the
# ---- behavioural `selected_by_required_kind_not_stage_name` above, whose fixture names its verify stage `check`.
raw=$(grep -rnE 'stage[[:space:]]*(==|!=)[[:space:]]*"|"[[:space:]]*(==|!=)[[:space:]]*[A-Za-z_.]*stage\b' crates/rk-cli/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the stage-literal sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: resolved.stage compared to a string literal on a CODE line (ADR-0162 decision 5):"; printf '%s\n' "$code"; exit 1; }
ctl=$(printf '%s\n' 'crates/rk-cli/src/verify_pass.rs:9:    if resolved.stage == "verify" {' | grep -E 'stage[[:space:]]*(==|!=)[[:space:]]*"' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: the stage-literal pattern does not catch its own synthetic control"; exit 1; }

# ---- (7) THE SHIPPED FILES THIS STAGE AUTHORS OR EDITS, where the records put them. Each is a FLOOR beneath a
# ---- named behavioural test in (5); none of them is the assertion.
test -f .redkiln/catalog-extended/tag-release.toml || { echo "FAIL: tag-release is not in the extended directory (ADR-0114 decision 3)"; exit 1; }
test ! -e .redkiln/catalog/tag-release.toml || { echo "FAIL: tag-release is in the SHIPPED catalog before a trial ran and a promotion decision cited it (ADR-0114 decision 3; AC-110)"; exit 1; }
test -f .redkiln/checklists/smoke.toml || { echo "FAIL: no smoke checklist — checklist:smoke resolves to nothing"; exit 1; }
# (7a) The questions: s8d's registry, and no second file (D21-43's integration call).
test ! -e .redkiln/questions.toml || { echo "FAIL: .redkiln/questions.toml exists — question loading is s8d's, under .redkiln/processes/ (D21-43)"; exit 1; }
set -- .redkiln/processes/*.toml
[ -f "$1" ] || { echo "FAIL: .redkiln/processes/ holds no .toml — s8d's registry is absent (Halt 8)"; exit 1; }
for q in confirmation.not-fixed scan.accept-findings verify.ac-discriminates; do
  n=$(grep -hxF "[question.\"$q\"]" .redkiln/processes/*.toml | grep -c .)
  [ "$n" -eq 1 ] || { echo "FAIL: question $q is declared $n times under .redkiln/processes/, not once — the gate that opens it could not be answered through the loaded registry"; exit 1; }
done
# (7b) The release goals (R5) and AC-139's policy rows (D21-43 N24; R2).
grep -qxF 'goal = "The fix is tagged for release, reversibly"' .redkiln/packs/bug-standard.toml || { echo "FAIL: bug-standard's release goal is not the amended wording (R5)"; exit 1; }
grep -qxF 'goal = "Tagged for release, reversibly"' .redkiln/packs/story.toml || { echo "FAIL: story's release goal is not the amended wording (R5)"; exit 1; }
for p in bug-standard story; do
  n=$(grep -cE '^goal = ".*[Pp]roduction' ".redkiln/packs/$p.toml")
  [ "$n" -eq 0 ] || { echo "FAIL: $p.toml still promises production, which a run that pushes nothing cannot reach"; exit 1; }
  pol=$(awk '/^\[policy\]$/{p=1;next} /^\[/{p=0} p' ".redkiln/packs/$p.toml")
  test -n "$pol" || { echo "FAIL: $p.toml has no [policy] table to read"; exit 1; }
  printf '%s\n' "$pol" | grep -qxF 'verification_required = ["verify"]' || { echo "FAIL: $p.toml's [policy] does not ship AC-139's fail-closed default (D21-43 N24; R2)"; exit 1; }
done
# (7c) The changelog (s8b's), with a row for every activity and pack edit this stage makes (ADR-0072). Each edited
# activity is read in s8b's <id>@<new version> form, so a bare id cannot be satisfied by a longer one (unit-tests by
# unit-tests-ue). The IntegrationReport row names story.toml's require-row move (D21-46 R5).
test -f .redkiln/CHANGELOG.md || { echo "FAIL: s8b's pack changelog is absent — ADR-0072's row for this stage's pack and catalog edits has nowhere to go"; exit 1; }
for w in tag-release 'tagged for release' verification_required IntegrationReport; do
  grep -qiF "$w" .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md carries no row naming '$w' (ADR-0072; R1, R2, R5; D21-46 R5)"; exit 1; }
done
# (dated 2026-09-26, s8c refuting round 3; D21-67 ruling 1) s8d's class C disposition bumps equivalence-boundary 2->3 and
# writes an `equivalence-boundary@3` changelog row, so this stage's N23 edit takes it 3->4. At s8g's open, amend by dated
# line: this loop's `equivalence-boundary@3` becomes `equivalence-boundary@4`, and (7d)'s exact-version check for
# equivalence-boundary reads 4, not 3. Otherwise s8d's row satisfies this grep before s8g writes anything (a vacuous
# pass). Measure the version on disk at s8g's open first; the note is owed, not yet applied.
# (dated 2026-09-26, s8d build, package P2; receiver note, D21-67 ruling 1) s8d's catalog change LANDED the premise:
# .redkiln/catalog/equivalence-boundary.toml is at version 3 and .redkiln/CHANGELOG.md carries
# `| equivalence-boundary@3 |` (from v2, D21-67 ruling 1, class C: its `policy` severity conjunct and `findings` input
# removed). So this stage's N23 edit is 3->4: at s8g's open this loop's `equivalence-boundary@3` reads
# `equivalence-boundary@4`, and (7d)'s exact-version check reads 4, each by dated line after re-measuring on disk.
for pair in unit-tests@3 unit-tests-ue@3 equivalence-boundary@3 property-based-testing@3 integration-tests@3 smoke-test@2; do
  grep -qF "$pair" .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md carries no row naming $pair in s8b's <id>@<new version> form (ADR-0072; D21-43 N23; D21-46 R5, R6)"; exit 1; }
done
# (7d) N23: the four displaced providers, re-homed to implement over ONE test-authoring kind listed first.
kinds=""
for a in unit-tests unit-tests-ue equivalence-boundary property-based-testing; do
  f=.redkiln/catalog/$a.toml
  test -f "$f" || { echo "FAIL: $f is absent — N23 re-homes the activity, it does not delete it"; exit 1; }
  grep -qxE 'stage[[:space:]]*=[[:space:]]*"implement"([[:space:]]*#.*)?' "$f" || { echo "FAIL: $a is not re-homed to the implement stage (D21-43 N23)"; exit 1; }
  prov=$(grep -E '^provides[[:space:]]*=' "$f")
  test -n "$prov" || { echo "FAIL: $a declares no provides"; exit 1; }
  n=$(printf '%s\n' "$prov" | grep -cF '"TestReport"')
  [ "$n" -eq 0 ] || { echo "FAIL: $a still provides TestReport beside the pass — two routes (s8g §8g.2)"; exit 1; }
  ver=$(sed -n 's/^version[[:space:]]*=[[:space:]]*\([0-9][0-9]*\).*/\1/p' "$f")
  [ -n "$ver" ] && [ "$ver" -eq 3 ] || { echo "FAIL: $a's version is '$ver', not 3 — N23's edit bumps it 2 -> 3 (ADR-0072: an edited activity is a new version)"; exit 1; }
  k=$(printf '%s\n' "$prov" | sed -n 's/^provides[[:space:]]*=[[:space:]]*\["\([A-Za-z0-9]*\)".*/\1/p')
  test -n "$k" || { echo "FAIL: $a's first provided kind could not be read from: $prov"; exit 1; }
  kinds="$kinds $k"
done
n=$(printf '%s\n' $kinds | sort -u | grep -c .)
[ "$n" -eq 1 ] || { echo "FAIL: the four re-homed activities list $n distinct first kinds ($kinds), not one test-authoring kind"; exit 1; }
K=$(printf '%s\n' $kinds | sort -u)
row=$(awk -v h="[artifact.$K]" '$0==h{p=1;next} /^\[/{p=0} p' .redkiln/artifacts.toml)
printf '%s\n' "$row" | grep -qxF 'source = "activity"' || { echo "FAIL: the test-authoring kind $K has no source = \"activity\" row in artifacts.toml"; exit 1; }
# Each re-homed activity's emit binds $K and no longer binds TestReport: its own test runs are step evidence (s8g section 8g.2).
for a in unit-tests unit-tests-ue equivalence-boundary property-based-testing integration-tests; do
  f=.redkiln/catalog/$a.toml
  em=$(awk '/^\[\[step\]\]$/{p=0} /^id[[:space:]]*=[[:space:]]*"emit"/{p=1} p' "$f")
  test -n "$em" || { echo "FAIL: $a's emit step could not be read"; exit 1; }
  n=$(printf '%s\n' "$em" | grep -cF 'TestReport')
  [ "$n" -eq 0 ] || { echo "FAIL: $a's emit still binds TestReport — two routes (s8g section 8g.2)"; exit 1; }
  printf '%s\n' "$em" | grep -qE "^artifacts[[:space:]]*=.*[{,][[:space:]]*$K[[:space:]]*=" || { echo "FAIL: $a's emit does not bind the test-authoring kind $K (s8g section 8g.2)"; exit 1; }
done
for p in bug-standard story; do
  imp=$(awk '/^\[\[?stage\.implement(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' ".redkiln/packs/$p.toml")
  printf '%s\n' "$imp" | grep -qxF "artifacts = [\"$K\"]" || { echo "FAIL: $p.toml's implement stage carries no require row over $K — the test-design escalation did not move with its activities (N23)"; exit 1; }
  # The moved row keeps its when and its rigor: one [[stage.implement.require]] entry over $K, read whole.
  ents=$(awk '/^\[\[stage\.implement\.require\]\]$/{if(e!="")print e; e="@";p=1;next} /^\[/{if(p&&e!="")print e; e="";p=0;next} p{e=e $0 "@"} END{if(p&&e!="")print e}' ".redkiln/packs/$p.toml"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $p.toml's implement require entries could not be read (rc=$rc)"; exit 1; }
  ent=$(printf '%s\n' "$ents" | grep -F "@artifacts = [\"$K\"]@")
  n=$(printf '%s\n' "$ent" | grep -c .)
  [ "$n" -eq 1 ] || { echo "FAIL: $p.toml carries $n [[stage.implement.require]] entries over $K, not one"; exit 1; }
  case "$ent" in *'@when = "criticality >= high"@'*) ;; *) echo "FAIL: $p.toml's implement row over $K is not keyed when = \"criticality >= high\" — the escalation moved without its condition (N23)"; exit 1 ;; esac
  case "$ent" in *'@rigor = "mandatory"@'*) ;; *) echo "FAIL: $p.toml's implement row over $K does not carry rigor = \"mandatory\" — the escalation moved without its floor (N23)"; exit 1 ;; esac
  vrf=$(awk '/^\[\[?stage\.verify(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' ".redkiln/packs/$p.toml")
  test -n "$vrf" || { echo "FAIL: $p.toml's verify stage could not be read"; exit 1; }
  n=$(printf '%s\n' "$vrf" | grep -cxF 'artifacts = ["TestReport"]')
  [ "$n" -eq 0 ] || { echo "FAIL: $p.toml's verify stage still escalates TestReport's rigor — a floor on a pass-sourced kind selects nothing (N23)"; exit 1; }
done
# (7e) ENTRY PRECONDITIONS this stage reads and does not author: s8b's N25 edit to sca-scan, and s8c's setup_projects.
grep -qE '^expr[[:space:]]*=[[:space:]]*".*report\.unrated == 0' .redkiln/catalog/sca-scan.toml || { echo "FAIL: sca-scan's check does not require unrated == 0 — s8b's N25 edit is absent (Halt 8)"; exit 1; }
test -f .redkiln/registry.toml || { echo "FAIL: s8c's registry.toml is absent (Halt 8)"; exit 1; }
grep -qF 'crates/rk-adapter-toolchain-node/tests/fixtures/project' .redkiln/registry.toml || { echo "FAIL: s8c's setup_projects row does not name the Node fixture project — a run worktree's whole suite fails ten tests at preflight (D21-43 N25; s8c §8c.5)"; exit 1; }
# (7f) TestReport is pass-sourced.
grep -qE '^\[artifact\.TestReport\]' .redkiln/artifacts.toml || { echo "FAIL: artifacts.toml has no TestReport row"; exit 1; }
tr_row=$(awk '/^\[artifact\.TestReport\]/{p=1;next} /^\[/{p=0} p' .redkiln/artifacts.toml)
printf '%s\n' "$tr_row" | grep -qxF 'source = "pass"' || { echo "FAIL: TestReport is not pass-sourced — the catalog's providers would still be selected beside the pass (two routes)"; printf '%s\n' "$tr_row"; exit 1; }
# (7g) D21-46 R5: integration-tests re-homed to implement over the SAME test-authoring kind $K, IntegrationReport kept,
# and story.toml's IntegrationReport row moved with it. D21-46 R6: smoke-test's passing check, in step order.
f=.redkiln/catalog/integration-tests.toml
test -f "$f" || { echo "FAIL: $f is absent — D21-46 R5 re-homes the activity, it does not delete it"; exit 1; }
grep -qxE 'stage[[:space:]]*=[[:space:]]*"implement"([[:space:]]*#.*)?' "$f" || { echo "FAIL: integration-tests is not re-homed to the implement stage (D21-46 R5)"; exit 1; }
grep -qE '^version[[:space:]]*=[[:space:]]*3([[:space:]]|$)' "$f" || { echo "FAIL: integration-tests is not at version 3 (ADR-0072; D21-46 R5)"; exit 1; }
prov=$(grep -E '^provides[[:space:]]*=' "$f")
k=$(printf '%s\n' "$prov" | sed -n 's/^provides[[:space:]]*=[[:space:]]*\["\([A-Za-z0-9]*\)".*/\1/p')
[ "$k" = "$K" ] || { echo "FAIL: integration-tests lists '$k' first, not the test-authoring kind $K (D21-46 R5, as N23): $prov"; exit 1; }
printf '%s\n' "$prov" | grep -qF '"IntegrationReport"' || { echo "FAIL: integration-tests no longer provides IntegrationReport — story.toml's external_dep row would have no producer"; exit 1; }
imp=$(awk '/^\[\[?stage\.implement(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' .redkiln/packs/story.toml)
printf '%s\n' "$imp" | grep -qxF 'artifacts = ["IntegrationReport"]' || { echo "FAIL: story.toml's implement stage carries no IntegrationReport row — the require row did not move with its activity (D21-46 R5)"; exit 1; }
vrf=$(awk '/^\[\[?stage\.verify(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' .redkiln/packs/story.toml)
test -n "$vrf" || { echo "FAIL: story.toml's verify stage could not be read"; exit 1; }
n=$(printf '%s\n' "$vrf" | grep -cxF 'artifacts = ["IntegrationReport"]')
[ "$n" -eq 0 ] || { echo "FAIL: story.toml's verify stage still requires IntegrationReport, whose only producer now sits at implement (D21-46 R5)"; exit 1; }
f=.redkiln/catalog/smoke-test.toml
grep -qE '^version[[:space:]]*=[[:space:]]*2([[:space:]]|$)' "$f" || { echo "FAIL: smoke-test is not at version 2 (ADR-0072; D21-46 R6)"; exit 1; }
order=$(sed -n 's/^id[[:space:]]*=[[:space:]]*"\([A-Za-z0-9_-]*\)".*/\1/p' "$f" | tr '\n' ' ')
[ "$order" = "smoke-test build smoke passing emit " ] || { echo "FAIL: smoke-test's ids read '$order', not the activity then build, smoke, passing, emit (D21-46 R6)"; exit 1; }
stp=$(awk '/^\[\[step\]\]$/{p=0} /^id[[:space:]]*=[[:space:]]*"passing"/{p=1} p' "$f")
test -n "$stp" || { echo "FAIL: smoke-test's passing step could not be read"; exit 1; }
printf '%s\n' "$stp" | grep -qE '^kind[[:space:]]*=[[:space:]]*"check"' || { echo "FAIL: smoke-test's passing step is not a check"; exit 1; }
printf '%s\n' "$stp" | grep -qF "\"smoke.status == 'passing'\"" || { echo "FAIL: smoke-test's passing check does not read smoke.status == 'passing' (D21-46 R6)"; exit 1; }
printf '%s\n' "$stp" | grep -qE '^on_false[[:space:]]*=[[:space:]]*"fail"' || { echo "FAIL: smoke-test's passing check does not fail on false (D21-46 R6)"; exit 1; }
# (7h) s8c's hand-off (final consistency pass 2026-09-24): [checklist.smoke] names the file by path and is no longer
# declared unauthored. The probe reads the TOML on stdin (Git Bash's /tmp is invisible to Windows Python); its
# control, an unauthored entry, comes first.
sm='import sys,tomllib; e=tomllib.load(sys.stdin.buffer).get("checklist",{}).get("smoke",{}); print(("path="+str(e["path"]) if "path" in e else "")+("|unauthored" if "unauthored" in e else ""))'
ctl=$(printf '[checklist.smoke]\nunauthored = "s8g"\n' | python -c "$sm"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = '|unauthored' ] || { echo "FAIL: CONTROL — the smoke-entry probe read '$ctl' (rc=$rc) over an unauthored entry, so it could not go red"; exit 1; }
got=$(python -c "$sm" < .redkiln/registry.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = 'path=checklists/smoke.toml' ] || { echo "FAIL: .redkiln/registry.toml's [checklist.smoke] reads '$got' (rc=$rc), not path = \"checklists/smoke.toml\" alone — checklist:smoke would still refuse by name (s8c §8c.12; s8g §8g.6)"; exit 1; }
test -s .redkiln/checklists/smoke.toml || { echo "FAIL: .redkiln/checklists/smoke.toml is empty — the entry would name no content"; exit 1; }

# ---- (8) THE COLD SLOWEST-CHILD FIGURE IS IN THE VERIFY RECORD, cited from s8c, and under ADR-0140's bound.
secs=$(sed -n 's/.*cold cargo test --workspace slowest child: \([0-9][0-9]*\) s.*/\1/p' "$vf" | head -1)
test -n "$secs" || { echo "FAIL: the verify record carries no cold slowest-child line in the form §8g.2 fixes"; exit 1; }
[ "$secs" -lt 3600 ] || { echo "FAIL: a cold child takes $secs s, at or over ADR-0140's 3600 s — every cold verify pass would be Unverifiable (golden constraint 6; halt)"; exit 1; }

# ---- (9) THE WHOLE SUITE, --no-fail-fast, rc 0, every result line green, and a FLOOR: Checkpoint 8f's recorded
# ---- passed count plus this stage's 66. Expected value, re-derived at stage open from Checkpoint 8f's recorded count:
# ---- PREV = 1123 (1038 + 85), so the floor is 1123 + 66 = 1189 (final consistency pass 2026-09-24; D21-48 2026-09-24:
# ---- s8f's X11 test, then the floor propagation of s8c 78, s8d 62 and s8d-ii 50; it was 1106 / 1172). Edit PREV by a dated edit of the next line before the first run. A lower bound, never an equality.
# ---- At least 44 result lines,
# ---- as at 992d257. Under (0b)'s wrapper (D21-47).
PREV=1123
out=$(nodispatch cargo test --workspace --no-fail-fast 2>&1); rc=$?
nd_check "(9)'s whole suite" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: the workspace suite is red (rc=$rc)"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked' | head -20; exit 1; }
results=$(printf '%s\n' "$out" | grep -c '^test result:')
greens=$(printf '%s\n' "$out" | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed')
[ "$results" -ge 44 ] && [ "$results" -eq "$greens" ] || { echo "FAIL: $greens of $results result lines are green, over a floor of 44 result lines"; exit 1; }
passed=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9][0-9]*\) passed.*/\1/p' | awk '{s+=$1} END {print s+0}')
[ "$passed" -ge $((PREV + NEW)) ] || { echo "FAIL: $passed passed, under the floor of $((PREV + NEW)) (Checkpoint 8f's $PREV + this stage's $NEW)"; exit 1; }

# ---- (10) fmt AND clippy, each by its own exit code, under (0b)'s wrapper.
out=$(nodispatch cargo fmt --all --check 2>&1); rc=$?
nd_check "(10)'s fmt" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nodispatch cargo clippy --workspace --all-targets 2>&1); rc=$?
nd_check "(10)'s clippy" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }

# ---- (11) NO RELEASE TAG WAS MADE IN THIS REPOSITORY BY THE BLOCK — the tag tests run in temporary repositories.
tags_after=$(git tag -l 'rk-release/*'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git tag -l could not run after the suite (rc=$rc)"; exit 1; }
[ "$tags_before" = "$tags_after" ] || { echo "FAIL: the block created or removed an rk-release tag in the operator's repository:"; diff <(printf '%s\n' "$tags_before") <(printf '%s\n' "$tags_after"); exit 1; }

# ---- (12) THE ENTRY GATE AND THE PROVISIONAL CHECKPOINTS, RE-RUN WHOLE FROM THEIR STAGE FILES' OWN BYTES (§8g.1):
# ---- 5 (rk-types), 6 (rk-git), 7b and 7b-ii (rk-kernel), 7d (rk-cli), 8b (.redkiln/), 8d (the walk tables), 8d-ii
# ---- (the item-id validator), 8e (the interim-key list), and 8f, the entry gate.
# ---- Extracted, never retyped. A block with no end marker gets one appended, so rc=0 can be told apart from a block
# ---- that stopped early. The extractor writes under target/ — Git Bash's /tmp is invisible to Windows Python. Store
# ---- handling around `doctor --rebuild` is each block's own (the s8 amendment), and nothing below reads the store.
# ---- BOUNDED: every replay runs with RK_CP_NESTED=1, and when THIS block runs under RK_CP_NESTED=1 it replays
# ---- nothing. NO DISPATCH (D21-46; D21-47): every replay runs under (0b)'s wrapper, and nd_check follows each one
# ---- (the trap still empty, the fake uncalled). Before a replay runs, every `redkiln run` on a CODE line of its block must either follow a code line
# ---- that writes `harness_binary` at the fixture child rk-fixture-child-claude, or be a listed refusal-only
# ---- invocation whose NEXT line asserts its refusal exit code. A comment line counts for neither. The refusal-only
# ---- list is quoted whole below, and it holds Checkpoint 7d's two lines (rc 2 from the parser, rc 3 from config).
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8G (12) SKIPPED: this block is itself a replay (RK_CP_NESTED=1); its re-runs ran at Checkpoint 8g's own close"
else
  printf '%s\n' \
    'out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln run --resume </dev/null 2>&1); rc=$?' '[ "$rc" -eq 2 ]' \
    'out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln run some-item </dev/null 2>&1); rc=$?' '[ "$rc" -eq 3 ]' \
    > target/cp8g/refusal-only.txt || { echo "FAIL: could not write the refusal-only list"; exit 1; }
  RUN_GUARD='import re, sys
src, allow = sys.argv[1], sys.argv[2]
run = re.compile(r"(\$RK|\$BIN|redkiln(\.exe)?)\"?[ \t]+run([ \t]|$)")
armer = re.compile(r"harness_binary[ \t]*=.*rk-fixture-child-claude")
al = open(allow, encoding="utf-8").read().split("\n")
refusal = {}
for i in range(0, len(al) - 1, 2):
    if al[i].strip():
        refusal[al[i].strip()] = al[i + 1].strip()
lines = open(src, encoding="utf-8").read().split("\n")
armed = False
bad = []
for n, l in enumerate(lines):
    s = l.strip()
    if not s or s.startswith("#"):
        continue
    if armer.search(s):
        armed = True
    if run.search(s):
        nxt = lines[n + 1].strip() if n + 1 < len(lines) else ""
        if s in refusal and nxt.startswith(refusal[s]):
            continue
        if not armed:
            bad.append("%d: %s" % (n + 1, s))
for b in bad:
    print("UNGUARDED", b)
print("RUN-LINES-CHECKED")
sys.exit(4 if bad else 0)'
  # SYNTHETIC CONTROLS: a harness_binary named only in a comment arms nothing; a code line that writes it arms the run
  # after it; a listed refusal-only line whose next line asserts another exit code is not admitted.
  printf '%s\n' '# harness_binary = "target/debug/rk-fixture-child-claude"' '"$RK" run x' > target/cp8g/ctl-comment.sh || { echo "FAIL: could not write a control"; exit 1; }
  printf '%s\n' 'harness_binary = "target/debug/rk-fixture-child-claude"' '"$RK" run x' > target/cp8g/ctl-armed.sh || { echo "FAIL: could not write a control"; exit 1; }
  printf '%s\n' 'out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln run some-item </dev/null 2>&1); rc=$?' '[ "$rc" -eq 0 ]' > target/cp8g/ctl-wrongrc.sh || { echo "FAIL: could not write a control"; exit 1; }
  for c in comment:4 armed:0 wrongrc:4; do
    out=$(python -c "$RUN_GUARD" "target/cp8g/ctl-${c%%:*}.sh" target/cp8g/refusal-only.txt 2>&1); rc=$?
    [ "$rc" -eq "${c#*:}" ] || { echo "FAIL: CONTROL — the no-dispatch guard exited $rc on its ${c%%:*} control, expected ${c#*:}"; printf '%s\n' "$out"; exit 1; }
  done
  EXTRACT='import sys
F = chr(96) * 3
src, mode, key, out = sys.argv[1:5]
lines = open(src, encoding="utf-8").read().split("\n")
blocks = []
i = 0
while i < len(lines):
    if lines[i] == F + "bash":
        j = i + 1
        while j < len(lines) and lines[j] != F:
            j += 1
        blocks.append((i, lines[i + 1:j]))
        i = j + 1
    else:
        i += 1
if mode == "marker":
    hit = [b for (s, b) in blocks if any(key in l for l in b)]
else:
    at = [n for n, l in enumerate(lines) if key in l]
    hit = [b for (s, b) in blocks if at and s > at[0]][:1]
if len(hit) != 1:
    print("EXTRACT-FAIL", len(hit))
    sys.exit(3)
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(hit[0]) + "\n")
print("EXTRACTED", len(hit[0]))'
  for spec in "cp5@runbook/phase-1/s5-workspace-ci-rk-types.md@anchor@**Checkpoint 5.** All of:" "cp6@runbook/phase-1/s6-store-and-git.md@anchor@**Checkpoint 6.** All of:" "cp7b@runbook/phase-1/s7b-kernel.md@anchor@### 7b.14 Checkpoint 7b" "cp7bii@runbook/phase-1/s7b-ii-gates.md@anchor@### 7b-ii.13 Checkpoint 7b-ii" "cp7d@runbook/phase-1/s7d-cli-and-migrate.md@marker@CP7D-END-OF-BLOCK reached" "cp8b@runbook/phase-1/s8b-catalog-quality.md@marker@CP8B-END-OF-BLOCK reached" "cp8d@runbook/phase-1/s8d-step-grain-and-resolver.md@marker@CP8D-END-OF-BLOCK reached" "cp8dii@runbook/phase-1/s8d-ii-run-worktrees.md@marker@CP8D-II-END-OF-BLOCK reached" "cp8e@runbook/phase-1/s8e-tier-one-and-the-lane.md@marker@CP8E-END-OF-BLOCK reached" "cp8f@runbook/phase-1/s8f-landing-and-rendering.md@marker@CP8F-END-OF-BLOCK reached"; do
    name=${spec%%@*}; rest=${spec#*@}; file=${rest%%@*}; rest=${rest#*@}; mode=${rest%%@*}; key=${rest#*@}
    test -f "$file" || { echo "FAIL: $file does not exist — Checkpoint $name cannot be re-run (Halt 8)"; exit 1; }
    ex=$(python -c "$EXTRACT" "$file" "$mode" "$key" "target/cp8g/$name.sh" 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: could not extract exactly one block for $name: $ex"; exit 1; }
    out=$(python -c "$RUN_GUARD" "target/cp8g/$name.sh" target/cp8g/refusal-only.txt 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $name invokes redkiln run with no fixture harness_binary written before it, on a line that is not a listed refusal-only invocation — its replay could dispatch to a real account:"; printf '%s\n' "$out"; exit 1; }
    printf '%s\n' "$out" | grep -qxF 'RUN-LINES-CHECKED' || { echo "FAIL: the no-dispatch guard did not finish over $name"; printf '%s\n' "$out"; exit 1; }
    printf '\necho "CP8G-REPLAY-%s-END reached"\n' "$name" >> "target/cp8g/$name.sh" || { echo "FAIL: could not append $name's marker"; exit 1; }
    out=$(nodispatch env RK_CP_NESTED=1 bash "target/cp8g/$name.sh" 2>&1); rc=$?
    nd_check "Checkpoint $name's replay" || exit 1
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $name re-run whole exited $rc — its row stays provisional"; printf '%s\n' "$out" | grep -E 'FAIL' | head -10; printf '%s\n' "$out" | tail -15; exit 1; }
    printf '%s\n' "$out" | grep -qxF "CP8G-REPLAY-$name-END reached" || { echo "FAIL: Checkpoint $name exited 0 without reaching its last line"; exit 1; }
  done
fi
nd_check "the end of the wrapped runs" || exit 1

# ---- (13) THE WORKTREE PATH SET IS UNCHANGED, compared BEFORE any prune; the prune is a dry run and finds nothing.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET (the pass's ephemeral trees must be removed on every path — ADR-0162 decision 10):"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }

echo "CP8G-END-OF-BLOCK reached"
```

⚠️ **What this block cannot see, stated rather than claimed away.**

- **(6) is a floor over code lines.** A stage literal assembled by a `const`, a `matches!` over a pattern, or a helper
  whose name hides the comparison passes it. That is why the behavioural test with a renamed stage is the assertion.
- **(7) reads the catalog and packs as text.** A kind listed second, a require row written as an inline table, or a
  `provides` spread over two lines reads wrong. Each of those lines is a floor beneath a named test in (5), and a
  shape it cannot read fails, which is the safe direction.
- **(9)'s floor is a lower bound.** It cannot tell a deleted older test from a new one. The named lists are what pin
  this stage's tests.
- **(12) replays with the environment the operator gives this block, plus what the wrapper sets and unsets.** It sets
  `RK_CP_NESTED`, `CLAUDE_CONFIG_DIR` and the fake first on `PATH`, strips every other `PATH` directory holding a
  `claude`, and unsets `REDKILN_LAUNCH_RECORD`. If a
  replayed checkpoint needs a variable, its own guard names it and this block fails, which is the right direction.
- **(12)'s trap has limits.** It catches a real `claude` that inherits `CLAUDE_CONFIG_DIR`. A child launched with that
  variable set explicitly escapes it. From `s8h` on, `s8h`'s launch-record check closes that path: a real-harness run
  without a launch record refuses before it spawns anything (`s8h`'s launcher; the exact sentence is `s8h`'s to fix).
- **The fake `claude` is compiled, not a script** (final consistency pass 2026-09-24). A Windows process that
  resolves `claude.exe` by itself skips an extensionless script on `PATH`, which is why the draft's shell-script fake
  was replaced: the helper's fake is a `claude.exe`, first on `PATH`, and every other `PATH` directory holding a
  `claude` is stripped. A process that resolves `claude` by an absolute path escapes both, and the trap and the unset
  launch record are what cover it. `nd_setup` checks resolution before it runs the fake, so the block never runs a
  real `claude`, not even for `--version`.
- **(12)'s static guard reads code lines only.** A run assembled from a variable passes it. So does a run after a
  `harness_binary` line written into a config the run does not read, because one armed line arms every later run in
  the block. The refusal-only list admits a line only when the line after it asserts that exact exit code.
- **(0a)'s name sweep is a floor.** A tree name assembled another way passes it. `ac339_…`'s assertion on the tree's
  path is the assertion.
- **`setup` is not exercised end to end here.** It runs for real on a fresh run worktree only in `s8j`'s fixture
  proof of the lane, where a whole verify runs inside a real run worktree. Here, (5)'s `setup_precedes_…` asserts the
  order over a double, and (7e) asserts the declaration.

### 8g.11 Acceptance set

**Claimed here, each by the named test in §8g.10 (5):**

- **The verify block, seven of its nine ids whole**: AC-320 (with its dated *Verified by* note), AC-338, AC-339,
  AC-348, AC-349, AC-353 and AC-359. AC-355 is claimed on its fixtures (its *"held in CI"* clause is `s8j`'s 14a), and
  AC-341 on its port-call half (below). *(Corrected 2026-09-24, D21-47 review, finding 16: the list named eight ids
  as seven, and AC-321 is not in `s7d` §7d.18's block.)*
- **AC-321, beside the block**: D17-4's replay event, by `verify::ac321_…`'s two tests.
- **AC-341** on the port-call half. Its static half is cross-listed to 7b-ii.
- **Beside the block**: AC-331; AC-344's `cannot_run` mapping; AC-347's zero-dispatch half and its shipped-catalog
  lint, with the lint's single residual, `migration-dry-run`, named and its unreachability asserted (D21-46 R5).
- **AC-137's type half**: a criterion with no `gwt` and no `verification` is accepted and stored
  (`acceptance_criterion::ac137_criterion_without_gwt_or_verification_is_accepted`). Its render and authoring halves
  are not claimed (below).
- **AC-133.** Its instrument is built here for the first time. `s5`'s claim is corrected, not re-claimed, and `s8h`
  registers into the same registry.
- **AC-485's toolchain identity half.**

**Named with a reason, not claimed:**

| id | reason |
| --- | --- |
| AC-341's CI-green integration half | the `Tracker` port has no status surface. It is Phase 3's (AC-508), recorded by a dated register note |
| AC-357 | HUMAN GATE. Its machine part is exercised inside `ac359_…` |
| AC-114 | HUMAN GATE. The promotion and its `curation.decision` lint are `s8j`'s |
| AC-139 | HUMAN GATE, **decided 2026-09-24** by D21-43 N24 (*"Block (fail-closed)"*). R2 records it, and both packs ship it as a `[policy]` row. A human decision is recorded, never claimed by a test |
| AC-350 and AC-317 | ARMED, as before |
| AC-137's render and authoring halves | The rendered `spec.md` that *"shows the criterion with its GWT and verification slots visibly empty, not fabricated"*, and its golden render fixture, go through `s8f`'s rk-store renderer (`render::`) over the `Spec` template, which is `s8h`'s. *"authored fields from the first CLI write"* is `s8h`'s card (§8g.3). Reported upward (D21-47 review, finding 9) |
| AC-473 and AC-521 | Step 8's release checks. This stage moves AC-473's *Verified by* and claims neither |
| AC-347 over `migration-dry-run` | the single named residual (D21-46 R5): one agent step at verify, on an activity no Phase-1 lane reaches (§8g.5). A row that makes it reachable turns `selection::migration_dry_run_is_selected_on_no_shipped_lane` red |

**Cross-listed:**

- AC-328's whole-suite policy. The adapter side is `s8c`'s.
- AC-455. It was discharged at 7b-ii, and is re-pointed here to `rk_types::AcceptanceCriterion` with its two policy
  rows unchanged.
- AC-138, re-proven over the shipped rows by `ac139_shipped_packs_block_a_missing_verification`'s row-removed arm.
- AC-110, unchanged. `tag_release::an_enabled_pack_referencing_the_extended_activity_is_refused` asserts it.

**Owed and reported upward, because no sub-stage owns them:**

- **The `curation.decision` backfill for the 102 shipped activities.** None of them carries a decision id, and
  AC-114's lint (`s8j`'s) will meet them on its first run (§8g.7).
- **N23's *"Verify only measures"* holds over every reachable verify activity, and not over the shipped catalog.**
  D21-46 R5 re-homed `integration-tests` and left `migration-dry-run` as the named residual, unreachable while
  `data_migration` is NOT MEASURABLE (§8g.5). Making that facet measurable, or adding any row that requires
  `MigrationTestReport`, reopens the question for whichever stage does it.
- **`unit-tests-ue` authors nothing.** It is re-homed as a test-authoring activity by N23's letter (§8g.2).
- **`s8h`'s `Spec` must embed `AcceptanceCriterion` unchanged, as `acceptance_criteria`** (§8g.3). *Settled
  2026-09-24 (final consistency pass):* `s8h` §8h.4 now names the member `acceptance_criteria`.
- **The `verification` gap under N24's block default** (§8g.3). A lane whose agent-authored `Spec` leaves one AC's
  `verification` empty refuses at verify. `s8h`'s schema and cards, and `s8j`'s fixture proof, are named as its
  owners. *Settled 2026-09-24 (final consistency pass):* both files now carry it (§8g.3).
- **AC-137's render half** (the empty-slot golden render over `s8f`'s renderer and `s8h`'s `Spec` template), which no
  stage claims yet.

**Handed to `s8j`, by name:**

- the tag-release real trial, AC-114's promotion and its lint (§8g.7);
- Checkpoint 7c's whole re-run (D21-43 N28; §8g.1);
- `verify_completed`'s `item` key and its `adapter`, `origin` and `stage` member names, for its fixture proof, which
  reads identity only from `origin = "pass"` rows (§8g.4);
- the assertion that every AC of each lane's `Spec` carries a `verification` (§8g.3).

**Handed to `s8h`, by name:**

- a `verify.ac-discriminates` row in `s8h` §8h.10's gate-expiry table for both packs, and the class in `MINTED_CLASSES`
  (§8g.6; the review's cross-file finding 7);
- `Spec`'s member name, `acceptance_criteria`, and a required, authored `verification` on every AC a specifying
  activity emits (§8g.3).
