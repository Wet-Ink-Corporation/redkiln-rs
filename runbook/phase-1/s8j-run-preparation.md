# Step 8j — Fixture proof of both lanes, CI green, and the run preparation (Phase 1, part 8, eleventh sub-stage)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8j` | Phase 1 | Checkpoint 8i | Checkpoint 8j | `s8i` | `s8` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45.* The draft at `a1d35ac` was written before those three ledger entries. This
revision changes the following:

- **N2** (§8j.1, §8j.2, §8j.4): the story's authored `Spec` is whatever the story pack's mandatory `specification`
  stage selects, which is `bdd-scenarios` today. It is never `user-story-ac` by name.
- **Slice and project** (§8j.1, §8j.4, §8j.5): both lane items carry `s8e`'s `slice`. `[project] name` is already
  in `.redkiln/config.toml`, and the lane fixture copies it.
- **Question loading** (§8j.2): the loader and `.redkiln/processes/` are `s8d`'s. This stage declares only the two
  human-root questions. RT-59 is cited as settled by R-48, and it leaves this stage's record.
- **D21-44** (§8j.1(c), §8j.1(d), §8j.4, §8j.8, §8j.10):
  - the legacy run worktree moves from `.git/redkiln/worktrees/` to `.redkiln/worktrees/`, with its ref kept, where
    `s8d-ii`'s re-entry refuses it (D21-46 R1 moved W2 out of `s8e`);
  - Checkpoints 7h and 7i are re-pointed at named fixture items;
  - the lane fixture and every recursive walk over `.redkiln/` in `s8` skip `.redkiln/worktrees/`.
- **N28 and the probes** (§8j.6):
  - the pin moves by `s8h` §8h.8's steps 1–3, which are cited here and not restated. That procedure's own
    `live_dispatch_probe` dispatch is taken inside Checkpoint 7c's run;
  - Checkpoint 7c is re-run whole and unamended, from a plain terminal, as the child of the launcher's `wrap`
    mode. That run is also the `live_dispatch_probe` re-run *(split since 2026-09-24 by D21-47: the suite portion
    runs outside `wrap`, and only the live portion runs under it; §8j.6 step 2; and amended only by A12's `-H`
    since the final consistency pass, 2026-09-24)*. *D21-48 (2026-09-24), cross-file pass:* after `s8c`'s
    cargo-nextest install (X15), 7c's `nextest_fallback_recorded_on_report` line still passes, because the test keeps
    its name over a controlled nextest-absent probe (`s8c` §8c.9). Its description *"nextest ABSENT on this box"*
    and `s7c` §7c.22's ARMED presence-arm row are stale, and the run is read against `s8c` §8c.12's note. Neither is
    edited here;
  - the probe names are `s8h`'s;
  - every result is run and recorded by the operator, in `s8j-evidence.txt`.
- **The launcher in `s8`** (§8j.8 A1, A7): all three of `s8`'s `redkiln run` sites go through `tools/live-run.sh run`.
- **Ownership** (§8j.8 A1, A5.2, A8):
  - the quoted-predicate guard is `s8b`'s form, adopted here and not re-specified;
  - `verify_completed`'s `item` and `adapter` members are bound from `s8g`;
  - `s8e`'s anchored rebuild differential joins `s8`'s (2), and `s8`'s walks over `.redkiln/` skip `worktrees/`
    (`s8e` §8e.9; `s8d-ii` §8d-ii.1);
  - AC-114's `curation.decision` lint is this stage's by `s8g`'s assignment, and it is owed with the promotion.
- **The proofs' module** (§8j.4) is `lane_proof`, because `s8e` owns `lane_e2e` and its fixture-toolchain lane.
- **The gates this stage adds** (§8j.2, §8j.3) each take a `[[gate_expiry]]` row in `s8h`'s table.
- **C2 and C6 are excluded** (§8j.5), because `s8f`'s render path calls the locator.
- **`s8g`'s hand-on** (§8j.1(e)): both lane items declare the `tag-release` trial.
- **Checkpoint 8j** (§8j.10):
  - the entry chain is re-run once, through Checkpoint 8i, and the checkpoint says so;
  - every re-run is fenced against a real dispatch;
  - the whole-suite floor is re-derived from Checkpoint 8i's recorded count;
  - the block runs from a plain terminal.

*Revised again 2026-09-24 for D21-46.* This pass changes five things:

- **R4 is confirmed** (§8j.1(e)). The two `tag-release` trials, one per lane item, were a reading this file flagged for
  the operator. D21-46 R4 takes it: *"**Two trials, confirmed.** Each is one run of one activity, which is the
  clause's constraint."* The flag and its one-trial fallback are withdrawn.
- **`s8d-ii` joins the chain** (§8j.10). D21-46 R1 adds a stage, *"`s8d-ii`, "Run worktrees"*, between `s8d` and
  `s8e`. The entry gate stays Checkpoint 8i. Checkpoint 8j's re-run list gains Checkpoint 8d-ii, and this stage is
  now the eleventh sub-stage.
- **The launcher's `answer` mode** (§8j.2, §8j.3, §8j.8 A1). D21-46 gives `s8h`'s launcher an `answer` mode *"so
  s8j's hold answer is not refused for want of a launch record"*. The hold answer and both human-root answers go
  through `tools/live-run.sh answer`, and Checkpoint 8j (8) refuses a bare `redkiln answer` in `s8`.
- **One re-run convention and one no-dispatch guard** (§8j.10 (10)). Under D21-46's `RK_CP_NESTED=1`, one run of
  Checkpoint 8i no longer carries the chain. Checkpoint 8j therefore runs each link once, directly, as a nested
  entry gate, and skips all of it when it is itself run nested. Every re-run has `CLAUDE_CONFIG_DIR` at an empty trap
  directory that must stay empty, and `REDKILN_LAUNCH_RECORD` unset.
- **The dispatch screen** (§8j.10 (10)) refuses only shapes that execute something. `s8h` reported that this file's
  screen matched Checkpoint 8h's lines that list ignored live tests and run clippy with `--features live`. Measured
  over Checkpoint 8h's block as it stands at `c5d19d7`, the drafted screen matched **none** of them: zero hits. It is
  widened here to the launcher's `answer` and `wrap` modes, the `$L` spelling, and a test run with the live feature
  and without `--list`. The widened screen has zero hits on each of the eleven blocks that exist today (8d-ii's does
  not). A first widening also refused `--ignored` without the live feature, and it hit Checkpoint 7h's
  `conformance_every_trailer_key_has_a_producer --ignored` line, which runs no live test. It was narrowed to the live
  feature. The screen carries synthetic controls both ways: six shapes that must match, and six that must not. The
  six that must not are five of Checkpoint 8h's shapes and 7h's non-live ignored run. *Stale, corrected 2026-09-24
  (D21-47; review findings 18 and 19): the screen now carries twelve shapes that must match and nine that must not,
  admits the wrappers' planted bare-word controls by shape and 7f's fixture-route line by a reasoned exemption file, and was
  re-measured over the twenty current blocks (§8j.10 (10)).*
- **The floor's drafted base** (§8j.10 (11)) is `s8i`'s drafted floor, 780 + 6 + N, and no longer 780. N is not
  known at authoring. *Stale, corrected 2026-09-24 (D21-47; cross-file #18): the consistency pass below recomputed
  it to 1187 + N, and the D21-47 fixes make it Checkpoint 8i's count, read from `PROGRESS.md` row 8i, plus fifteen.* *Superseded
  2026-09-24 (final consistency pass): plus fourteen, 1242 + N.*

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **Cross-references after the split.** The `.redkiln/` walk rule is `s8d-ii` §8d-ii.1's, 7f's cleanup-line deletion is
  `s8d-ii` §8d-ii.7's, W2's re-entry refusal is `s8d-ii`'s, and `s8e`'s 7h/7f section is §8e.13. `s8h`'s launch check
  on `answer` and its `wrap` guarantee are re-quoted from §8h.7's current bytes, and `s8g`'s binding sentence from
  §8g.4's.
- **`verify_completed`'s and `dispatch_completed`'s `item` is read from `t2_event.item`**, the column `s8e` §8e.10
  carries it in beside the payload under D21-46 R3 (A1, A4, and the `lane_proof` row). `rate_limit_reading.item`
  stays a payload member, as ADR-0163 put it. The store probes read `t1_gate_transition`, because `s8e`'s `0002`
  migration drops `t1_gate` (D21-46 R2).
- **The last gate before spend** (lead decision in the consistency pass). Checkpoint 8j's chain also re-runs whole
  every older checkpoint an `s8` stage made provisional: 5, 6, 7a, 7b, 7b-ii, 7d, 7f and 7g (7h and 7i were already
  in it; 7c is §8j.6's live run, N28; 7e cannot be re-run whole). Each runs nested, under the trap and the screen.
  Budget: twenty whole checkpoint runs, most carrying a whole suite.
- **The pin rule is carried here**: the adapter pin `TESTED_CLI_VERSION` equals S-BIN's pinned `cli_version`
  (§8j.6; Checkpoint 8j (6)).
- **The three gate classes are spelt** (`human_root:BugReport`, `human_root:IntakeRecord`, `hold:specification`, as
  drafts the records may re-spell), and their fail-closed `[[gate_expiry]]` rows land in both packs; `s8h` §8h.10's
  table names them. Checkpoint 8j (3) asserts the rows.
- **The probe session runs from a plain terminal only** (§8j.6): the `! python …` alternative is withdrawn, because
  `s8h` §8h.7's launcher refuses a session's `!` line at step 2.
- **The floor chain is recomputed once**: Checkpoint 8i's 1168 + 6 + N, plus this stage's 13 = 1187 + N. *Corrected
  2026-09-24 (D21-47): this stage now adds fifteen tests, so 1189 + N, and (11) reads 8i's count from its row.* *Superseded 2026-09-24
  (final consistency pass): the chain recomputed once gives 8i 1228 + N, and this stage adds fourteen, so 1242 + N.*

*Adversarial-review fixes 2026-09-24 (D21-47).* Twelve refuters reviewed the eleven `s8` files at `4a8c832`. This file
drew four BLOCKERs, seven MAJORs and seventeen numbered MINORs, which name eighteen items and are keyed here 12–29 by
position. It also drew six cross-file items. D21-47 is the authority: its five operator decisions (V1–V5) and the
fix-round rules. Each finding and its fix:

- **1 (BLOCKER), and cross-file #9: A1 spent before its gate checks, and it ran both lanes under one go-ahead.**
  Fixed (§8j.8 A1). `s8`'s (1) block becomes four blocks: (1) launch, (1) check, (1b) launch and (1b) check. Every
  launch block runs `prespend` before its launcher line, and that function carries D21-47's four checks and five
  more. The go-ahead is `S8_GO=<item>`, given per launch as a command prefix.
- **2 (BLOCKER): Checkpoint 8j (8) was satisfied by A5.5's `WALKED` line, and A5.2 described a line `s8b` does not
  have.** Fixed (§8j.8 A5.2; Checkpoint 8j (8)). The replacement is Checkpoint 8b section (4)'s code, copied verbatim.
  (8) reads `s8b`'s `EXCL=`, `D2=` and `two_walk() {` lines from `s8b`'s file at check time, and asserts them by fixed
  string. `s8b`'s plant-control walk is exempted by its exact bytes and nothing else.
- **3 (BLOCKER): 7c's whole run under `wrap` exposed its suite to a real launch record and the real config.** Fixed
  (§8j.6 step 2), by D21-47's split. The suite portion runs outside `wrap`, under the wrapper. A zero-spend
  `--check wrap` dry run follows, with a counting fake `claude`. Only the live portion runs under `wrap`, on its own
  go-ahead. The launch-gate `Env` rule is enforced by the new lint
  `root::every_binary_spawn_in_the_suite_sets_the_launch_record_explicitly` (§8j.4).
- **4 (BLOCKER), V2: the story moves `pk:` and reddens the p9-pinned tests at its own verify.** Fixed, by record and by
  proof (§8j.1(a); Checkpoint 8j (3b)). D21-47 V2 has `s8b` read every historical p9 measurement at its pinned
  commit, so the story stands. (3b) makes a byte change to `spike.toml` in a `--shared` clone and runs the three
  crates that read `spikes/p9`, under the wrapper. The residual, a **semantic** break that a non-pinned test reads,
  is the story lane's own verify's to catch, and it is stated.
- **5 (MAJOR): Checkpoint 8j (10) could not pass.** Fixed, in four parts:
  - 8a's `ARMED` stanza is `s8e`'s amendment (cross-file #2). (10) now refuses by name while it is missing.
  - The 7f amendment is specified as §8j.8 A10, with the exemption file.
  - 8a (11)'s card guard is re-scoped by A11.
  - The stage opens with a neutralise-and-re-run of the whole chain (§8j.10).
- **6 (MAJOR): the stage close was circular.** Fixed (§8j.10's stage close). There are six ordered steps: work, pin
  move, evidence, CI and the frozen record, Checkpoint 8j over a clean tree, then the tracker. The checkpoint output
  moves to the tracker commit.
- **7 (MAJOR): probe spend was not gated mechanically on the fixture proofs.** Fixed (§8j.6 step 1b; Checkpoint 8j
  (6) and (7)). A pre-probe block writes `fixture-proofs` and eleven `mutant` lines (ten since the final consistency pass). It runs over the moved pin, so
  the proofs are re-run after the move. (6) requires every paid run to be dated on or after the proofs, and its launch
  record to carry `RC=0` and a HEAD with the proofs' code. (7) ties the proofs' code to CI's.
- **8 (MAJOR): A5.3 walked `target/`.** Fixed. It is now `git grep` over tracked files, with an untracked plant under
  `target/` and the design row as its two controls.
- **9 (MAJOR): (3)'s arm had an unbound `ITEM`, and it passed as a no-op over a closed item.** Fixed (§8j.8 A3). The
  arm moves into two blocks of its own, bound to `$FIX_ITEM`. It refuses a closed lane. It kills the launcher's tree
  after the first committed step, with a bounded poll, and each of its launches is preceded by `prespend`.
- **10 (MAJOR): AC-114's promotion and lint had no receiver in `s8`.** Fixed (§8j.8 A9; §8j.1(e)). A named step comes
  after Checkpoint 8, with its own push and CI.
- **11 (MAJOR): the repro pair could not express C3 or C4.** Fixed (§8j.5 step 3; Checkpoint 8j (4)). The pair gains
  `kind = "it" | "unit"`. C3 is a `unit` repro, and C4 is an `it` repro that writes its own pack.
- **12:** `lane_check` and `prespend` diff all of `.redkiln` (items and intake included), plus the CI path list.
  Fixed.
- **13:** the fix ref must not merge before Checkpoint 8. Stated (§8j.5 step 5), and refused by `prespend` and
  `lane_check`. Fixed.
- **14:** A1 refuses a shipped `harness_binary`, and requires `observed_model` on every `dispatch_completed` row of
  the item. Fixed.
- **15:** A2 compares the rebuilt tier-1 counts before the restore. Fixed.
- **16:** A2's replacement keeps A8's anchor line byte-identical, so the order of application is free. Fixed.
- **17:** A6 inserts into both of (6)'s blocks, and (8)'s build-first checker reads every binary use. Fixed.
- **18:** the screen now refuses `--features rk-adapter-claude/live`, `"live"`, `=foo,live`, `export` and
  `declare -x` of the launch record, and a path-valued launch record even beside a `printf` control. Fixed
  (Checkpoint 8j (10)). Probed over the twenty blocks of the working tree mid-fix-round, it had four kinds of hit:
  - the wrappers' planted controls that `s8a`, `s8e`, `s8f` and `s8g` are adding (`REDKILN_LAUNCH_RECORD=planted`
    reaching `sh -c 'printf …'`);
  - `s8d-ii`'s planted control (`=s8dii-control` reaching `sh -c 'echo …'`);
  - `s8h`'s two sandboxed `wrap` runs;
  - a quoted list of argv strings in `s8h`.

  The first two are admitted by shape: a bare-word value names no launch-record file. The last two go through the
  reasoned exemption file, if `s8h` keeps them as landed.
- **19:** the header's screen note. Corrected by a dated line. Fixed.
- **20:** the attach grammar is `--attach <KIND>=<PATH>`, with both sufficiency flags, as `cli.rs` fixes it (§8j.2).
  Fixed.
- **21:** *"a run that STOPS AT A GATE exits 0"* is `s8`'s comment, not ADR-0128. It is re-attributed to AC-403 and
  C-01 D-01 (§8j.3). Fixed.
- **22:** (11)'s literal must appear in `PROGRESS.md` row 8i, and the floor is +15. Fixed.
- **23:** the transcript tripwire is scoped to this repository's project directories, and `claude --version` in the
  replays reaches the fake, whose log (12) reads (cross-file #12). Fixed.
- **24:** Checkpoint 8j is one-shot. Stated (§8j.10, and the block's header). Fixed.
- **25:** `lane_proof` copies every tracked `.redkiln/` file, `registry.toml` included, and asserts it (§8j.4). Fixed.
- **26:** the hold record's fourth clause has a test,
  `run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load`, and a mutant (§8j.3). Fixed.
- **27:** `INDEX.md` row 19's entry gate becomes Checkpoint 8j (A0), asserted in (8). Fixed.
- **28:** the identity check reads only the verify stage's own pass rows (`origin`, after `s8g`'s review finding 4),
  in A1 and in `lane_proof`. Fixed.
- **29:** A5.1 copies `s8b`'s `K_TESTS` and its three walked-count readings. Fixed.
- **Cross-file #8:** the 7f amendment is in §8j.8 A10. It uses the fixture child. The launch record is supplied from a
  file on the one line `s8j-screen-exempt.txt` admits byte for byte. The spawn markers are asserted against the
  run's dispatch events. Fixed.
- **Cross-file #13:** the fixture child is `rk-fixture-child-claude`, reached by `support::fixture_child()` (§8j.4).
  Fixed.
- **Cross-file #14:** `CATALOG_TESTS`'s `BIND:` slot is withdrawn for `s8b`'s twenty-six `K_TESTS` (A5.1). Fixed.
- **Cross-file #15:** `artifact.write`'s item is read from the `t2_event.item` column (A1; §8j.4). Fixed.
- **Cross-file #18:** the header's `780 + 6 + N` line is corrected by a dated line. Fixed.
- **D21-47's 7c split:** §8j.6 step 2. Fixed.

**The stage's count of new tests is fifteen**, two more than the drafted thirteen:
`run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load` (finding 26) and
`root::every_binary_spawn_in_the_suite_sets_the_launch_record_explicitly` (D21-47's lint). So Checkpoint 8j's floor is
Checkpoint 8i's recorded count plus fifteen, 1189 + N on the drafted chain. The mutants are eleven, two more
(`M-hold-unordered-accepted` and `M-launch-env-inherited`). *Superseded 2026-09-24 (final consistency pass): fourteen
new tests and ten mutants, because the launch-record lint and its mutant are `s8h`'s (§8j.4). The floor is Checkpoint
8i's recorded count plus fourteen, 1242 + N on the chain recomputed once (Checkpoint 8j (11)).*

**Found in this pass, and not fixed here** *(all three closed by the final consistency pass, 2026-09-24, below)*:
- A Rust `Command::new("claude")` on Windows resolves `claude.exe`, which D21-47's shell-script fake does not shadow.
  For Rust-spawned `claude` the unset launch record and the trap remain the guards. This is stated in Checkpoint 8j
  (0).
- The wrapper controls other stages are adding plant a bare-word `REDKILN_LAUNCH_RECORD` value. The screen admits
  that shape only when the command is `sh -c 'printf …'` or `sh -c 'echo …'`. A differently shaped control is
  refused, and it needs a reasoned exemption entry or a reshaped line. `s8h`'s in-progress sandbox `wrap` runs are
  refused today, and they need exemption entries at application.
- A10 and A11 name lines that other stages own and are still editing. At application, their anchors are taken from
  those files as landed.

*Final consistency pass 2026-09-24* (after the D21-47 fix round; the eleven files read against each other as they stand):
- *Final consistency pass 2026-09-24:* §8j.6 steps 1b and 2 and Checkpoint 8j source `runbook/tools/nodispatch.sh`.
- *Final consistency pass 2026-09-24:* the three shell-script fakes are gone; the compiled fake shadows Rust spawns too.
- *Final consistency pass 2026-09-24:* `nd_check` runs after each cargo run and each replay, and replaces (12)'s reads.
- *Final consistency pass 2026-09-24:* the launch-record lint is `s8h`'s; this stage's draft lint and mutant withdrawn.
- *Final consistency pass 2026-09-24:* new tests 15 → 14, mutants 11 → 10; floor = CP8i's count + 14, 1242 + N.
- *Final consistency pass 2026-09-24:* (11)'s chain is recomputed once: 8h 1222, 8i 1228 + N, lower bound 1228.
- *Final consistency pass 2026-09-24:* this stage's two record ids are literals in Checkpoint 8j (3), not inputs.
- *Final consistency pass 2026-09-24:* the run note asks for no record-id input; 8d runs nested, needing no entry sha.
- *Final consistency pass 2026-09-24:* probe 0 is §8j.6 step 2b, in the spend table, the evidence file and (6).
- *Final consistency pass 2026-09-24:* A10's anchors are `s8d-ii` §8d-ii.7's `clone="$c7f/c"` arm, quoted.
- *Final consistency pass 2026-09-24:* A11 builds on `s8h`'s own 8a (11) amendment and adds the launch-check proof.
- *Final consistency pass 2026-09-24 (lead):* A11 is reduced to a citation, because `s8h` §8h.14 already takes the
  launch-check proof into its 8a amendment, so the proof is made once. A10 gains its two dependent amendments, to
  8d-ii's and 8e (10d)'s card-mode stops (`s8h` finding 27). A0 was made at insertion. (8) finds INDEX's Step 8 row
  by its stage cell, not by number.
- *Final consistency pass 2026-09-24:* A10 lists `s8h`'s three screen hits as exemption entries, re-measured.
- *Final consistency pass 2026-09-24:* A12 adds `-H` to `s7c`/`s7d` `rk_absent`; (10) and step 2 assert it.
- *Final consistency pass 2026-09-24:* both lanes' proofs assert every AC carries a `verification` (`s8g` §8g.3).
- *Final consistency pass 2026-09-24:* `s8b`'s goldens line, `s8g`'s names line and `s8h`'s guard line re-quoted.
- *Final consistency pass 2026-09-24:* §8j.7 carries `s8b`'s first-CI measurement (git in the image, depth 0).
- *Final consistency pass 2026-09-24:* Checkpoint 8j (0) also refuses `AI_AGENT`, as `s8h` §8h.7 step 2 does.
- *Final consistency pass 2026-09-24:* A2's quotation of A8 is A8's own words.
- *Final consistency pass 2026-09-24:* the three classes join 8h's `MINTED_CLASSES` literal by this stage's dated edit.

*D21-48 (2026-09-24)* (the operator's X12 and X15; D21-48 is the authority). One line per edit:
- *D21-48 (2026-09-24):* §8j.6 step 1's pin-move commit carries X12's argv spelling change: `s8h` §8h.2's table is cut
  to the pin's one row, `LIVE_WARRANTED` is emptied, and the golden argv fixtures and snapshots move once.
- *D21-48 (2026-09-24):* step 2b's probe 0 confirms the spelling. A refused argv is a HALT for a wrong capture, no longer
  a stop for the lead's decision. Step 2's `live_dispatch_probe` is the first live argv to carry the spelling.
- *D21-48 (2026-09-24):* Checkpoint 8j (6) asserts the table's one row at the pin, the empty exemption, the pin's `--help`
  capture listing the row's spelling, and both golden argv fixtures carrying it, each reader behind a synthetic control.
- *D21-48 (2026-09-24):* §8j.7 notes that `ci.yml`'s cargo-nextest install is `s8c`'s (X15), and this stage adds none.
- *D21-48 (2026-09-24):* this stage's new tests stay fourteen and its mutants ten. X12's tests are `s8h`'s, so (11)'s chain
  moves by `s8h`'s three: 8h 1225, 8i lower bound 1231, and this block's floor 1245 + N.
- *D21-48 (2026-09-24), floor propagation:* the predecessors now state `s8c` **78**, `s8d` **62**, `s8d-ii` **50**, `s8f` **85** and `s8h` **54**,
  and `s8i` adds 6 + N + G (G = 3 on its human-gate route, 0 on its reviewer route, D21-48 X16). So (11)'s chain is
  8h **1243**, 8i lower bound **1249**, and this block's floor **1263 + N + G**. The line above is superseded.

---

**Why this stage exists.** `s8a`…`s8i`, with `s8d-ii` (D21-46 R1), build what criterion (1) and (1b) need. Nothing before this stage proves the
parts work **together, as a lane, through the entry point the real runs will use**. D21-40 decision 2 authorises the
real runs *"once the sub-stages' fixture proofs pass, and not before. That includes a FakeHarness end-to-end test of
`verbs::run::run`"*. `s8e` wrote the first such test (D21-42 24c). This stage writes the proofs that the spend gate
turns on: each lane, from `triage` to its landing, through `verbs::run::run`, with the real Rust toolchain adapter
and across real process boundaries.

It also prepares the run so that `s8` has nothing left to decide:

- the two items the runs are pointed at (D21-41 (f); D21-42 18);
- the human roots those items start from (D21-41 (f): *"decided once for both lanes"*);
- the bound on the first real invocation (D21-42 19b);
- the CLI pin, Checkpoint 7c's whole re-run and the two live probes `s8h` authored (D21-42 16; D21-43 N28;
  D21-41 (g));
- the legacy run worktree's re-point, and Checkpoints 7h and 7i re-pointed at named fixture items (D21-44);
- CI green on the pushed branch (D21-42 14a);
- the amendments to `s8`'s own instruments that the re-measurement of D21-40 found, and that the sub-stages make
  necessary (§8j.8).

**The order is fixed, and every paid or published act in it is a gate.** First the fixture proofs (§8j.4). Then
the pin move, which spends nothing, and the proofs' recorded re-run over it (§8j.6 step 1b; D21-47). Then, and only
then, Checkpoint 7c's split re-run and the probes (§8j.6), which spend. Last, CI on the
frozen code (§8j.7), which publishes. **No block in this file dispatches to a real account, and no block pushes.** Checkpoint 8j reads what
the gated acts recorded; it never repeats them.

🛑 **Every push and every paid dispatch in this stage is an explicit operator go-ahead, given at that moment and for
that act.** A go-ahead for one push does not cover the next push, and a go-ahead for one probe does not cover the
next. D21-42 14a: *"Each push is confirmed with the operator."* D21-40 decision 2: *"If a real run is refused
unexpectedly, the next step is to stop and report, not to spend again."* The same rule governs the probes.

**This stage takes decisions, and says so up front.** The last three stage files that said *"this step takes no
decision"* were wrong by eighteen, eleven and one records (s7i's own correction: *"`s7g` took eighteen … `s7h` eleven"*, and s7i took `ADR-0163`). This stage writes **two records
at `proposed`**, each with the next free id at writing, and neither id is assigned here:

1. **The human-root questions** (§8j.2). Decided by D21-41 (f). What is left to the record is the two questions'
   ids and options, in the spelling `s8d`'s loader reads, and the name the step-4 gate binds under.
2. **The boundary hold** (§8j.3). D21-42 19b decides *what*: *"The first real fix-lane invocation stops at the
   analysis → specification boundary for the operator to review."* No record decides *how*, and no mechanism for it
   exists today.

Each id is written into Checkpoint 8j (3)'s `S8J_Q_ADR=` or `S8J_HOLD_ADR=` line by a dated edit when its record is
written, and (3) refuses while either is empty *(final consistency pass 2026-09-24; D21-47: records are located by
the id recorded at write time)*. Each record flips to `accepted` only on the operator's dated ledger line (ADR-0126's
route). It also writes ledger
entries in `spikes/DECISIONS-2026-09-21.md`, the file that D21-40…D21-42 extend:

- the story candidate, with its reading of ADR-0011/E7;
- the Codex item's retirement;
- the fix-item reservation;
- the note on the fixture item;
- the re-point of the legacy checkout (D21-44);
- the `tag-release` trial declarations, citing D21-46 R4, which confirms two trials (§8j.1(e));
- the (g) flip, which is the operator's.

---

### 8j.1 The items: the story, the Codex item retired, 7i's empty case moved, the fixture item noted and its checkout re-pointed

**(a) The story item — `.redkiln/items/rk-story-harden-spike-pack.toml`.** D21-41 (f): *"A full lane entered at
`triage`. The candidate is to harden `spikes/p9/packs/spike.toml`, a Phase-2 story whose change stays inside
`spikes/p9/`."*

- `id = "rk-story-harden-spike-pack"`, `kind = "story"`, `stage = "triage"`.
- `slice = "harden-spike-pack"`. This is the declared slice from `s8e` §8e.8, and the story item has exactly one.
  The value is not the item id, not the project name, and not a stage name, and it has no whitespace. Those are the
  refusals `s8e` §8e.8 puts at the mint.
- `[facets]`: `cos = "standard"`, `criticality = "low"`, `blast_radius = "low"`, `size = "low"`,
  `external_dep = false`.
- `[trial] activity = "tag-release"`. See (e).

The `<project>` component of its trailers is `.redkiln/config.toml`'s `[project] name = "redkiln-rs"`, which is
already present at `a6349b6` [measured]. This stage adds nothing to the config.

The packet (f) option A measured why the facets must be low. With the Codex item's `external_dep = true` and
`blast_radius = "med"`, the selection gained *"five kinds"*: APIContract, ThreatModel, IntegrationReport,
RolloutPlan and FlagRecord. Low facets are the candidate's real profile. They are not chosen to make the run
cheap.

**What the lane authors is the `Spec` its mandatory `specification` stage selects.** D21-43 N2 takes *"The Spec the
story lane's mandatory specification stage selects"*, which is `bdd-scenarios` today, and supersedes D21-41 (f)'s
naming of `user-story-ac`. The premise it corrects is quoted in the same entry: *"`user-story-ac` has
`max_rigor = "recommended"`, so it is never admitted at the story pack's **mandatory** specification stage"*. No
line of this file names the producing activity as a literal. The proofs read it from the run (§8j.4).

**What "harden" means: the story's intent, not its spec.** The lane's own `specification` stage authors the
acceptance criteria, and this file must not write them in advance. The intent is stated in the item's header
comment and in the ledger row: *bring `spikes/p9/packs/spike.toml` up to the contract the two shipped packs were
brought to at Step 7e, so that it loads through the kernel's pack loader and could carry a lane — without installing
it.* It starts from these measured gaps, and each can be checked again:

- **No lane order.** `grep -c 'stage_order' spikes/p9/packs/spike.toml` gives **0**. `[policy] stage_order` is
  ADR-0153's only ordered stage carrier. The loader reads it as optional, so a spike item would load and then could
  not cross a single boundary.
- **No `stale_policy`.** Both shipped packs record one per cost class, with decision 5's warrant (ADR-0117). The
  spike pack records none.
- **No gate deadline scalar.** Both shipped packs carry `gate_deadline_secs`, and the spike pack does not. The value
  belongs to the RT-57 record that `s8h` writes (D21-42 19c).
- **Its twelve kinds are all registered and provided.** Every kind named by its baselines and require rows is in
  `.redkiln/artifacts.toml` and has at least one provider in `.redkiln/catalog/`. The five `spike-*` /
  `technical-spike` activities are installed. The hardening keeps this true under a mechanical check instead of by
  inspection. This was measured on 2026-09-24 over ProblemStatement, CompetitorAnalysis, SpikeCharter, SpikePlan,
  Estimate, ExperimentResults, ReviewVerdict, SpikeReport, DecisionRecord, ClosureRecord, KBAtoms and
  Retrospective.

🛑 **It is not installed.** ADR-0011 says *"Phase 1 ships exactly two process packs: `bug-standard` and `story`"*,
and E7 rejected *"Install twelve, enable two via `[packs] enabled`"*. The packet (f) risk list requires the ledger
row to state the reading under which a landed edit under `spikes/p9/` ships no pack: *"Landing a spikes/p9 edit
under a story lane is a reading of ADR-0011/E7, not something they expressly permit, so the D24 row must state it."*
**This stage does not touch `spike.toml`.** Checkpoint 8j asserts that its bytes are unchanged since `ca8f387`. A
stage that did the story's work in advance would leave the lane nothing to author.

⚠️ **The lattice goldens do not move, here or at the story's merge.** This stage hardens nothing. The edit is the
story lane's, and it lands on `rk-run/rk-story-harden-spike-pack` at `s8`. `s8b`'s draft had the goldens regenerate
in the commit that merges that ref. *Re-quoted 2026-09-24 (final consistency pass):* under D21-47 V2, `s8b` now
records *"When the story lane hardens `spikes/p9/packs/spike.toml` in the working tree (D21-41 f, `s8j`), no tag
moves and no golden is regenerated, and the lane's whole-suite verify stays green."*

**The story does not break the suite at its own verify (D21-47 V2)** *(added 2026-09-24; review finding 4).* The
review found that the story's edit moves `pk:`. The p9 parity goldens and the p9-pinned rigor, memo and AC-527 tests
would then go red at the story lane's own verify, which runs the whole suite. The lane's implementer has no shell to
repair them, and `lane_proof` cannot see the breakage. D21-47 V2: *"Every test pinning a historical `spikes/p9`
measurement reads the p9 tree at its pinned commit via git, never the working tree. … The story candidate (D21-41 f)
stands."* `s8b` makes that true, and it names the pinned sha and the helper. Under it, the story's edit moves none of
those tests, and the goldens need no regeneration (`s8b`, quoted above). Checkpoint 8j (3b) is the proof. It makes a byte change
to `spike.toml` in a `--shared` clone and runs the three crates that read `spikes/p9` there, under D21-47's wrapper.
A **semantic** change that a non-pinned test reads remains the story lane's own verify's to catch, which is the lane
working as designed.

**(b) The Codex item is retired: `.redkiln/items/rk-story-codex-adapter.toml` is deleted, by a dated ledger
entry.** The entry quotes the warrants:

- ADR-0007: *"the Codex adapter ships in Phase 4 ("reach"), and only after S-CODEX passes"*;
- E-0094-1: *"`rk-adapter-codex` is out of Phase 1 under every reading"*;
- the record `s8a` wrote for D21-42 22, which supersedes ADR-0094 decision 2's phase clause toward ADR-0007.

The file is deleted rather than marked retired, for two reasons. `ItemRecord` has no retired field. A story item
left in `.redkiln/items/` is also one `redkiln run` away from a lane on a Phase-4 story. The item is named by
`runbook/phase-1/s7i-cost.md` (seven occurrences), by the ledger and by the packet. Apart from its own file, no
`.rs` or `.toml` names it [measured at `a6349b6`]. So (c) lands **in the same commit**.

**(c) Checkpoint 7i's empty case moves to `rk-cost-empty-case`, and Checkpoints 7h and 7i stop reading
`$FIX_ITEM`.** D21-41 (f): *"Checkpoint 7i's empty case moves to a dedicated item that nobody dispatches, and its
PRECONDITION guard is kept."* A new `.redkiln/items/rk-cost-empty-case.toml` has `id = "rk-cost-empty-case"`,
`kind = "docs"` and `stage = "triage"`. Its header says only this:

> *dedicated to Checkpoint 7i's empty case; nobody dispatches it; the PRECONDITION guard in Checkpoint 7i is what
> protects that case*

🛑 **It does not say the item "can never run".** The packet (f) measured `derive_lane`'s
`(Some(given), None) => Ok(given)`, which runs a `kind = docs` item whenever `--lane` is passed. The guard is the
protection, and the header must not claim more.

**Both checkpoints are re-pointed at named fixture items, in one dated edit each.** `s8e` records the need: *"`s8j`
re-points 7h and 7i when it mints the fresh item"*. The reason is the environment. `s8` sets `FIX_ITEM` to the item
§8j.5 reserves (§8j.8 A1). A Checkpoint 7h or 7i run in that shell would read the reserved item's run instead of the
fixture it was built over. Both checkpoints stop reading the variable:

- **Checkpoint 7i** (`runbook/phase-1/s7i-cost.md`):
  - its seven `rk-story-codex-adapter` occurrences become `rk-cost-empty-case`;
  - every `$FIX_ITEM` becomes `$FIXTURE_ITEM`;
  - the line that opens `test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM (rk-fix-doctor-test-count at this pin)`
    becomes `FIXTURE_ITEM=rk-fix-doctor-test-count`;
  - one dated comment is added above that line.

  The PRECONDITION line keeps its shape and names the new item:
  `FAIL (PRECONDITION): rk-cost-empty-case owns $owned readings — …`.
- **Checkpoint 7h** (`runbook/phase-1/s7h-lane-completes.md`):
  - every `$FIX_ITEM` becomes `$FIXTURE_ITEM`;
  - the line that opens `test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item the lane ran` becomes two
    lines: `FIXTURE_ITEM=rk-fix-doctor-test-count`, and a guard that `.redkiln/items/$FIXTURE_ITEM.toml` exists;
  - one dated comment is added above them.

  Its ref reads are unchanged. `rk-run/rk-fix-doctor-test-count` carries exactly one `Story:` trailer,
  `redkiln-rs/rk-fix-doctor-test-count`, on `73c087a` [measured at `a6349b6`], and that is what its (2) and (3)
  count and parse.

The preamble's rule applies: *"A checkpoint that changed after it passed is provisional"*. So `PROGRESS.md` rows 7h
and 7i are marked `passed …, re-run pending (s8j re-point)`, and Checkpoint 8j re-runs both whole.

**(d) `rk-fix-doctor-test-count` stays the untouched fixture item, with a dated note, and its checkout is re-pointed.**
D21-42 18: *"stays the untouched fixture item, prose defect included, with a dated note"*. Its header opens *"Step 8
criterion (1), AC-473: the fix-lane self-hosting item"*, which is no longer true. A dated comment under it says
three things:

- the item is now the fixture item that Checkpoints 7h and 7i read;
- criterion (1)'s item is the one §8j.5 reserves;
- neither the item nor `rk-run/rk-fix-doctor-test-count` (at `73c087a`) is to be run, rebased or pruned.

**Only `#` lines change.** Checkpoint 8j asserts that the file's non-comment lines equal those at `ca8f387`, and that
the ref has not moved.

**The checkout under `.git` is re-pointed to `.redkiln/worktrees/` (D21-44).** D21-44: *"The one existing run-owned
worktree, `rk-fix-doctor-test-count` at `73c087a`, stays where it is. It is the undispatched fixture item (D21-42
18), and its re-point is s8j's."* The checkout is moved to the new root, and it is not removed. There are three
reasons:

- **At the new root it guards the item.** It carries no `RK_BASE` marker, because it predates D21-45 W1. So
  `s8d-ii`'s re-entry refuses any run of the item by name. D21-45 W2: *"A tree without `RK_BASE`, or with commits the run ref
  lacks, is **refused, never removed**"*. With the registration gone, `s8e` §8e.13 records the opposite outcome:
  *"If it is gone …, the run creates `.redkiln/worktrees/rk-fix-doctor-test-count` on the existing branch and mints
  stage-bearing marks"*, which breaks 7h.
- **Nothing that reads the fixture depends on the checkout's path.** Checkpoint 7h reads the ref and
  `.git/redkiln/runs/`. Checkpoint 7i reads the store. The ref stays at `73c087a`.
- **It removes the last registration under `.git/redkiln/worktrees/`.** That is the placement D21-44 measured
  unwritable, and the one in which D21-40 found the doc screen reading a stale `runbook/` copy. At the new root, every
  walk over `.redkiln/` skips it (§8j.8 A5.3, A5.5; `s8d-ii` §8d-ii.1), and `.gitignore`'s `.redkiln/worktrees/` row keeps
  it out of `git status` [measured at `a6349b6`].

The re-point is `git worktree move .git/redkiln/worktrees/rk-fix-doctor-test-count
.redkiln/worktrees/rk-fix-doctor-test-count`. The operator runs it after five reads:

1. `git -C .git/redkiln/worktrees/rk-fix-doctor-test-count status --porcelain` is empty.
2. Its HEAD equals `rk-run/rk-fix-doctor-test-count`'s tip, `73c087a`.
3. `git worktree list --porcelain` shows no `locked` line for it.
4. `git check-ignore -q .redkiln/worktrees/rk-fix-doctor-test-count` succeeds.
5. Neither `.redkiln` nor `.redkiln/worktrees` is a symlink. That is W1's guard, applied by hand.

Reads 1–3 held at `a6349b6` [measured]: the tree was clean, HEAD was `73c087a82245…`, and it was not locked. If any
read differs, STOP. A dirty or advanced tree is not the fixture D21-42 18 describes. The worst-case path under the new
root is 204 characters: 64 for the prefix, plus 140 for ADR-0114's file name, which is the tree's longest path at
`73c087a` [measured]. That is under the 260-character limit. The ref and `.git/redkiln/runs/` are not touched.

This interacts with two other stages:

- Checkpoint 7f's old cleanup line is `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count`.
  It is deleted by `s8d-ii`'s 7f amendment (`s8d-ii` §8d-ii.7), which also moved 7f's run arm into a throwaway clone.
  That file's finding 4 notes that `s8a` runs 7f first, and that `s8a`'s block restores the registration afterwards.
  So reads 1–3 are taken at this stage, not trusted from the measurement at `a6349b6`. From `s8d-ii` on, no stage's
  run of 7f touches either location.
- `s8a`'s Checkpoint 8a restores the old registration only if it was present when its block began. After the
  re-point it was not, so a chain re-run leaves the moved tree alone.

**(e) Both lane items declare `tag-release`'s trial (handed on by `s8g`).** `s8g` §8g.7 defers *"The trial's REAL
run on this repository"* to this stage, because *"It needs a landed commit and a journalled BuildArtifact for its
item, which only a lane run produces"*. It also requires that AC-114's promotion *"must precede any lane run whose
release stage is to select `tag-release`"*. On this repository only `s8`'s runs land a commit. Each of those runs
forks with `.redkiln/catalog` unchanged since the CI-green frozen sha (§8j.8 A1's `lane_check`), so a promotion
cannot sit between them. This file therefore takes the following route:

- The reserved fix item (§8j.5) and the story item each carry `s8g`'s item-record trial declaration,
  `[trial] activity = "tag-release"`, in the spelling `s8g`'s record fixes.
- Each lane's release stage then admits `tag-release` from `.redkiln/catalog-extended/` for that item's one run.
  That run is the trial's real run. There are two trials, and each is one run of one activity.
- The §8j.4 proofs carry the same declarations, so the fixture lanes walk `release` through `tag-release` before any
  real run does.
- **AC-114's promotion is not an act of this stage.** It follows `s8`'s fix-lane run and cites that run's id. It
  moves `.redkiln/catalog` only after `s8`'s runs, so no fork straddles it. *Its receiver in `s8` is named (added
  2026-09-24, D21-47; review finding 10).* §8j.8 A9 makes it a step **after Checkpoint 8**. The promotion and its
  lint edit `.redkiln/catalog` and `crates/`, and any such edit between the CI-green frozen sha and Checkpoint 8
  fails A5.4's CI line and every pre-spend check.

**Two trials, confirmed by D21-46 R4.** ADR-0114 decision 3 speaks of *"a one-run, one-activity trial the operator
declares"*. The draft read that as admitting one declaration per lane item, and flagged the reading for the operator.
D21-46 R4 takes it: *"**Two trials, confirmed.** Each is one run of one activity, which is the clause's constraint."*
The ledger row cites R4 and states no reading of its own. The draft's fallback is withdrawn. It dropped the story
item's declaration and moved the promotion ahead of the story run.

**AC-114's residue lint is this stage's, and it is owed with the promotion.** `s8g` §8g.7 assigns it here: *"AC-114's
residue lint does not exist, and its owner is `s8j`."* It gives two reasons:

- *"The lint's first non-vacuous input is the promotion decision that `s8j`'s real trial produces."*
- Built earlier, it *"would either fail all 102 shipped activities, none of which carries a `curation.decision` id,
  or ship with a grandfather rule that no record takes"*. `s8g` reports that backfill as *"a decision owed upward"*.

On this repository the real trials are `s8`'s runs and not this stage's, as shown above. So this stage takes the
ownership but builds the lint in the promotion's commit, after Checkpoint 8 passes (§8j.8 A9). It builds the lint only once the
operator has taken the backfill decision. The ledger row names the lint, its owner and both preconditions. Until
then no line of this file claims AC-114.

### 8j.2 The human roots: one question per human kind, declared once for both lanes

D21-41 (f): *"IntakeRecord and BugReport stay `source = "human"` (D12 #15), supplied through a pack-declared gate
question whose option attaches the kind (ADR-0129 d3/d4); this is decided once for both lanes."* It is on this
stage's path because each lane's first activity needs its root:

- The fix lane's analysis activities `minimal-repro` and `exploitability-analysis` both carry
  `requires = ["BugReport"]` [measured].
- Three of the five Spec providers require IntakeRecord: `bdd-scenarios`, `use-case-spec` and `user-story-ac`
  [measured: their `requires` rows]. `bdd-scenarios` is the one the story pack's mandatory specification stage
  selects today (D21-43 N2).

**Who owns what (D21-43's integration calls).** *"The loader and `.redkiln/processes/` are **s8d**'s. The
pack-declared human-root questions are **s8j**'s. s8g's `.redkiln/questions.toml` is dropped in favour of s8d's
loader."*

- `s8d` §8d.6 builds the loader: *"This stage adds a TOML loader into the existing `QuestionRegistry::with_question`
  and `bind` seams, used by both `run` … and `answer`"*. Its file home is `.redkiln/processes/questions.toml` by
  default, and its record fixes that. It also defers this stage's part: *"`s8j` authors the pack-declared human-root
  questions for `IntakeRecord` and `BugReport` into `.redkiln/processes/`."* The loader *"reads
  `.redkiln/processes/*.toml` and does not recurse"*, so a run worktree under `.redkiln/worktrees/` is never under it.
- This stage builds no loader and no second registry file. It adds entries to the one file `s8d`'s record fixes,
  in that file's spelling.

**The work.**

1. **Two questions**, one per human kind the lanes demand. Each has one option whose `attaches` names its kind:
   - BugReport, for the fix lane's analysis stage;
   - IntakeRecord, for the story lane's specification stage.

   The ids are `root.bug-report` and `root.intake-record`, and the record may re-spell them by a dated correction
   here. Each question's text states what the attachment is for, and nothing about the item.
2. **The binding.** Selection's step-4 gate is `Unsatisfiable { kind, … }` with `NoAdmissibleProducer` on a
   `source = "human"` kind (AC-054: *"returns `Unsatisfiable { kind, failed_predicates }` and opens a gate"*). For
   each of the two kinds that gate is bound to its question through `s8d`'s `bind` route. After the binding,
   answering the gate with the attachment writes the root under ADR-0129 decision 4: *"accepted and stored as a
   human-supplied root"*.
3. **Present on re-invocation.** The packet (f) risk list names this as unmeasured: *"whether a re-invocation
   classifies the attached root as Present"*. The proofs in §8j.4 measure it.
4. **Each new gate class has an expiry row.** `s8h` refuses a run at load when a reachable gate class has no
   `[[gate_expiry]]` row (its `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load`). So each of the
   two questions' gate classes gets a fail-closed row in `s8h`'s table, which the operator signs row by row like the
   others (D21-42 21a; AC-276). This stage drafts the rows and does not sign them. The classes are spelt here, as
   drafts the record may re-spell by a dated correction: **`human_root:BugReport`** and **`human_root:IntakeRecord`**
   (the step-4 gate's reason, then the human kind, on `unknown_facet:<axis>`'s pattern). Their rows land in **both**
   packs, `action = "fail"`, in the change that mints the classes, and `s8h` §8h.10's table names them. *Revised
   2026-09-24 (final consistency pass):* Checkpoint 8h's `MINTED_CLASSES` is a literal now, and `s8h` §8h.10 says this
   stage *"adds their spellings to Checkpoint 8h's `MINTED_CLASSES` line by a dated edit in the change that mints
   them"*. Both spellings are added so, in this stage's work commit. Each is a `&str` constant on a code line, because
   8h's premise reads it there (`s8h` §8h.0 item 7: *"Each minted spelling must be a `&str` constant, or a member of a
   constant `&[&str]` table, on a code line under `crates/*/src`"*). Checkpoint 8j (3) asserts one row per class per
   pack.

**The human-root files the real runs attach are committed now**, so that `s8`'s attach can be reproduced:

- `.redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md` states the intent from §8j.1(a) and nothing more;
- `.redkiln/intake/<fix-item>.BugReport.md` is written once §8j.5's pick is made.

**In `s8`, each root is attached through the launcher's `answer` mode**, as `tools/live-run.sh answer <item>
<option> --attach <Kind>=<file> --explain-sufficed <yes|no> --toml-opened <yes|no>`, with the option §8j.2's
record fixes. *(Corrected 2026-09-24, D21-47; review finding 20: this read `--attach <file>`. The grammar is the one
`crates/rk-cli/src/cli.rs` documents for `AnswerArgs`, `[--attach <KIND>=<PATH>]...`, and both sufficiency flags
are required, ADR-0129 decision 1.)* The arguments after the mode are `redkiln answer`'s
own (ADR-0129 decision 1), passed through unchanged. D21-46 adds the mode to `s8h`'s launcher. `s8h` §8h.7 makes
the binary's `answer` refuse without a launch record: *"Two binary composition roots refuse at entry with
`Exit::ConfigError` unless the captured `Env` carries `REDKILN_LAUNCH_RECORD` naming an existing file"*, and the second
of them is *"`answer::run`, the binary's `answer` (D21-46)"*. A bare `redkiln answer` from a plain terminal would
therefore be refused.

`.redkiln/intake/` is a new directory that no reader scans. The record states the path convention, and Checkpoint
8j's whole-suite line proves the new directory disturbs nothing.

**The record** (next free id at writing, `proposed`, with ADR-0085's `### Option — <name>` headings per D21-42 24a)
decides three things:

- the two question ids and their options, in the spelling `s8d`'s record fixes;
- the gate-kind name the step-4 gate binds under;
- the path convention above.

**RT-59 is not this record's.** It was settled by R-48 on 2026-09-20, by a dated note on design §5's `artifact.write`
row: *"every activity output, **and `redkiln answer` for an admitted attachment**"*. `PROGRESS.md`'s hygiene recount
also says so: *"**RT-59** by its dated `artifact.write` producing-step note (R-48)"*. The record cites that note and
decides nothing about it. `answer.rs` still carries *"RT-59: that `answer` is this row's producer is ROUTED, not
decided"*. That comment is stale, and it is corrected by a dated comment-only edit in this stage's work commit.

The record **supersedes nothing**. Carrying out ADR-0129 decision 3's *"Pack-declared questions … are Step 7e's"* at
a later stage is execution order, not a moved Decision sentence.

**Tests, red first.** All are in `rk-cli` `--test it`, beside `s8d`'s `processes::` tests:

- `processes::the_two_human_root_questions_load_and_each_attaches_its_kind`. This is red today, because neither
  question exists.
- `processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question`. It checks both directions: every
  `source = "human"` kind that a selected activity of either lane requires without a `?` is attached by exactly one
  question, and every question's `attaches` names a `source = "human"` kind in `.redkiln/artifacts.toml`.
- `processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question`.

The behavioural half is §8j.4's `lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes`.

**Mutant `M-attach-dropped`.** The BugReport question's option loses its `attaches`, which is a content mutation and
not a loader mutation. It must turn `processes::the_two_human_root_questions_load_and_each_attaches_its_kind` and the
`lane_proof` attach test red.

### 8j.3 The staged first run: the analysis → specification hold (D21-42 19b)

**The mechanism: a human-authored hold on the item, answered like any other gate.**

1. The reserved fix item's record carries `hold_before = "specification"`. The key is spelt that way here; the
   record may re-spell it, and a re-spelling takes a dated correction in this file.
2. When the lane loop's `AdvanceStage { from, to }` would enter the held stage, and no answered gate for that hold
   exists on the item, the kernel **opens a gate instead of advancing**. The gate uses the default question
   (`approve` / `changes-requested`; ADR-0129 decision 3).
3. `run` exits 0. Golden constraint 2, `run` never waits, and AC-403's *"the process exits 0 without waiting on any
   channel, prompt, socket or sleep"* (C-01 D-01) both hold unchanged. *(Corrected 2026-09-24, D21-47; review finding
   21: this attributed "a run that STOPS AT A GATE exits 0" to ADR-0128. That phrase is `s8`'s own comment, and its
   warrant is C-01 D-01 and AC-403.)*
4. The operator reads the analysis stage's outputs through `redkiln explain <item>`.
5. The operator answers through the launcher's `answer` mode: `tools/live-run.sh answer <item> approve
   --explain-sufficed <yes|no> --toml-opened <yes|no>`. The arguments after the mode are `redkiln answer`'s, in
   ADR-0129 decision 1's grammar. D21-46 adds the mode to `s8h`'s launcher *"so s8j's hold answer is not refused for
   want of a launch record"*. The answer is spent (`gate_answer_spent`, ADR-0123).
6. The next `redkiln run <item>` enters `specification`. In `s8` that run goes through `tools/live-run.sh run`
   (§8j.8 A1). It is a new process, so the first real invocation also crosses a process boundary on purpose, under
   the record `s8e` wrote for D21-41 (a).
7. `changes-requested` does not advance the lane. The rule is D21-40 decision 2's: stop and report.

**Why this and not another route.**

- **A `run` flag.** ADR-0128 *"Forbids a second `run` grammar"*, and an environment variable read by `run` is the
  same flag under another name.
- **A per-item dispatch cap** (D21-42 19a). It stops wherever the count runs out, mid-stage and not at the boundary.
- **A pack-level gate at `specification`.** It would hold every fix-lane item for ever.

The hold is item-level, and it is human-authored in the checkout before the run. It is not something the run writes.
So ADR-0157's forbids bullet, *"Writing an item's advanced stage back to `.redkiln/items/` (AC-518)"*, is untouched:
nothing writes the item record.

**The hold's gate is a gate like any other.** Its transitions follow `s8e`'s git-first gate lifecycle. D21-43 N6:
*"Every gate transition (opened, answered, expired, voided) is a git-first tier-1 event"*, and the hold changes
nothing about that. Its gate class takes a fail-closed `[[gate_expiry]]` row in `s8h`'s table, as §8j.2's two do.
Without the row, `s8h`'s load lint refuses every run of the item. The class is spelt **`hold:specification`**, the
held stage behind the cause, as a draft the record may re-spell. Its row lands in both packs, because a hold is
item-level and either pack's item can carry one. Its spelling is a `&str` constant on a code line and joins Checkpoint
8h's `MINTED_CLASSES` literal by the same dated edit as §8j.2's two (final consistency pass 2026-09-24).

**The record** (next free id at writing, `proposed`) covers four things:

- the item-record key;
- the gate cause;
- the rule that an answered hold never re-fires;
- the refusal of a hold that names a stage the item's pack does not order, at load and by name.

If `gate_opened`'s reason set is closed in `rk-types`, the new reason lands in the same change as its schema
snapshot. That is ADR-0116 decision 5's rule applied by analogy: *"The table row and the `rk-types` variant land in
the same change"*. The record supersedes nothing that this file could find. It states that finding and invites
refutation.

**Tests, red first.** All are in `rk-cli` `--test it`, over the §8j.4 fixture:

- `lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification`;
- `lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification`;
- `lane_proof::boundary_hold_changes_requested_does_not_advance`;
- `run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load` *(added 2026-09-24, D21-47; review finding
  26)*. It tests the record's fourth clause: an item whose `hold_before` names a stage its pack's `stage_order` does
  not carry is refused at load, by name. It is red today, because nothing reads the key.

**Mutants.**

- `M-hold-ignored`: the hold is not consulted. It must turn the first test red.
- `M-hold-spent-refires`: an answered hold opens again. It must turn the second test red.
- `M-hold-unordered-accepted`: the load check on the held stage is skipped. It must turn the `run::` test red.

### 8j.4 Both lanes proven end to end under the FakeHarness — the fixture proofs D21-40 decision 2 gates on

**What is proved, and through what.**

- Every proof enters through `rk_cli::verbs::run::run`. The only exception is the binary arm, which enters through
  the built `redkiln` binary. No test calls `replay::resume` or any kernel seam directly.
- The harness is the FakeHarness. It is injected through the seam `s8e` added for its first end-to-end test (D21-42
  24c), and extended here where the lanes need more.
- The toolchain is the **real** `rk-adapter-toolchain-rust`. `detect()` chooses it over the run-owned worktree, as
  ADR-0137 decision 2 says. It is never `FixtureToolchain`, and never the `rk-fixture-toolchain` child binary.
  `[adapters] toolchain_programs` is left unset for Rust, so `program` is `cargo` (ADR-0137 decision 7).
- A proof run over a fixture toolchain would prove nothing about AC-473's *"the toolchain stage ran through
  `rk-adapter-toolchain-rust` behind the `Toolchain` port"*.

**Why a module of its own.** `s8e` owns `crates/rk-cli/tests/it/lane_e2e.rs`. Its tests run a lane variant of
`pack-min`, which is a fixture lane, and they are not bound to the real adapter. The proofs here are in
`crates/rk-cli/tests/it/lane_proof.rs`, module `lane_proof`, for two reasons:

- the fixture-toolchain floor in Checkpoint 8j can read one file that holds only proofs over the real adapter;
- `s8e`'s tests are not made to carry a constraint they were not written under.

**The fixture repository.** It is materialised per test into a temporary directory, as a fresh git repository:

- a one-crate Cargo workspace from `crates/rk-cli/tests/fixtures/lane/`, with a planted defect and no failing test
  committed;
- a **byte copy of the shipped `.redkiln/` tree, taken at test time**: every tracked file under it. That is packs,
  catalog, `catalog-extended`, cards, processes, `artifacts.toml`, `facets.toml`, `roles.toml`, `registry.toml`
  (`s8c`'s verb table), `corpus-records.toml` and `config.toml`. The support code asserts that `registry.toml`
  arrived *(review finding 25)*, because A1's frozen-path check covers it and a proof without it would prove another
  registry. It is not a fixture pack, and not a committed duplicate that could
  drift. The proof runs over the registry `s8` will run, so a registry defect fails here and not at spend time. The
  copy includes `config.toml`'s `[project] name`, which `s8e` requires of *"every fixture repository a lane runs
  in"*;
- 🛑 **the copy skips `.redkiln/worktrees/`** (D21-44: *"a run worktree is a full repository copy"*). After the copy,
  the support code asserts that no directory named `worktrees` exists under the destination `.redkiln/`. A copy
  taken after `s8`'s runs would otherwise nest whole repositories into every proof;
- the repository is initialised through the product's own `init` path. Its `<commondir>/info/exclude` therefore
  carries the `# redkiln-runtime` block that `s8d-ii` writes (D21-45 W4; D21-46 R1). AC-518's porcelain assertion below then
  measures the product's own exclusion of `.redkiln/worktrees/`, not a `.gitignore` written for the test;
- the two lane items, carrying the real items' facets, `slice`, `hold_before` (fix only) and `[trial]` declaration
  (§8j.1(e)).

Each test builds its crate in the run-owned worktree's own `target/` (ADR-0137 decision 3), so a proof costs real
`cargo` time. That is the price of the real adapter. The run-owned worktree is at
`<fixture>/.redkiln/worktrees/<item>` (D21-44). The first proof asserts that it lies there, and that no path under
the fixture's `.git` is a registered worktree.

**The scripted harness.**

- Each scripted response is keyed by `(activity@version, step id, iteration path)`, which is D21-42 3b's step-grain
  key without its item. An `on_false` loop-back re-dispatches under a new iteration path (D21-42 4d) and gets its
  own response.
- Each response carries a closed output that validates against the `rk-types` `EXPORTED` schema `s8h` produced.
- **An unscripted dispatch fails the test by name, and there is no default response.** So the set of steps that
  dispatched is asserted, not assumed.
- The implementer's scripted response carries the file writes the double performs **inside the run-owned
  worktree**: a failing test at `red`, and the fix at `green`. The Rust adapter then observes a red suite, and after
  that a green one, through `Toolchain::test`'s outcome report (D21-42 1a).
- **Every gate the lane opens is answered with its question's first option**, as `s8d`'s walk does. That includes
  AC-079's `unknown_facet` gate for `change` at triage (D21-43 N3: *"`null` routes to AC-079's `unknown_facet` gate at
  triage, and the operator supplies it"*). The proofs assert the set of gates opened. They do not assume it.

**The ten `lane_proof` tests.** All are in `crates/rk-cli/tests/it/lane_proof.rs` and run under `rk-cli`
`--test it`. Each is written red first, and each was watched fail for the named reason before it was made green.

| test | asserts |
| --- | --- |
| `lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness` | From `triage`, the fix lane reaches a kernel-made content checkpoint commit on `rk-run/<item>` (D21-41 (b)). The commit's tree differs from the fork point under the crate's `src/`. The run-owned worktree is `<fixture>/.redkiln/worktrees/<item>` (D21-44). AC-518: the fixture's own checkout is unchanged. `git status --porcelain` is empty, `git diff-index --quiet HEAD` passes, and HEAD has not moved. Before the verify stage's first step, every AC of the fix lane's `Spec` carries a non-empty `verification` (`s8g` §8g.3's hand-on under D21-43 N24's block default; *added 2026-09-24, final consistency pass*), so a gap reads as this line and not as a verify refusal. |
| `lane_proof::fix_lane_trailers_every_non_forbidden_stage_in_order` | Git's own parser reads the stage tokens of the `Story:` trailers on `<fork>..rk-run/<item>`, oldest first (`%(trailers:key=Story,valueonly=true)` with `--reverse`), taking the last whitespace token of each value. Those tokens equal `bug-standard`'s `stage_order` filtered to `stages[s] != "forbidden"`, followed by the terminal `closed` token of D21-43 N9's *"`Story: <project>/<item> closed` mark"*. The comparison is set **and** order, and it includes a `stage_empty` stage (D21-42 7). |
| `lane_proof::fix_lane_verify_completed_names_the_rust_adapter` | At least one `verify_completed` row is scoped to the item by its interim `item` key, which `s8e` §8e.10 carries beside the payload (D21-46 R3), **and is the verify stage's own pass**, by the member `s8g` adds for its review finding 4 (`origin = "pass"`, as `s8g` §8g.4 fixes it; implement-stage test calls journal the kind too). Every such pass row's `adapter` payload member is `rk-adapter-toolchain-rust`. The names are `s8g`'s (D21-42 10b; `s8g` §8g.4). *(Scoped 2026-09-24, D21-47; review finding 28.)* |
| `lane_proof::story_lane_spec_is_landed_and_journalled` | The story lane's `Spec` is written by `rk-store` under the item's managed artifact directory (D21-42 5b). `git cat-file -e` finds it on `rk-run/<item>`. The item's **last** `artifact.write` row with `kind = "Spec"`, scoped by the `t2_event.item` column (`s8e` §8e.10; cross-file #15), carries a `digest` equal to the sha256 of the landed blob. That is ADR-0116 decision 3, *"not tree membership"*. The producing activity is read from the run's own record of what the specification stage selected, and never typed. It is `bdd-scenarios` at authoring (D21-43 N2), and the test does not name it. Every AC in the `Spec`'s `acceptance_criteria` carries a non-empty `verification`, read before the verify stage's first step (`s8g` §8g.3; D21-43 N24; *added 2026-09-24, final consistency pass*). |
| `lane_proof::story_lane_trailers_every_non_forbidden_stage_in_order` | As the fix-lane twin, over `story`'s order. The whole lane runs, not only up to the Spec. (1b) asserts *"every story-lane stage"*, and a proof that stopped at `specification` would leave `implement`, `verify`, `review` and `release` unproven for the story pack. |
| `lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes` | The first invocation stops at the step-4 gate on BugReport or IntakeRecord and exits 0. `answer --attach` through §8j.2's question succeeds. The re-invocation classifies the root as Present. |
| `lane_proof::gate_then_answer_then_second_process_continues_binary_arm` | **The multi-process arm.** It uses `env!("CARGO_BIN_EXE_redkiln")`, with `[adapters] harness_binary` set to `rk-fixture-child-claude`, D21-47's canonical fixture harness, located by `support::fixture_child()` as the sibling of `redkiln` (cross-file #13), and extended to replay a per-dispatch script. It also carries a fixture launch record, passed explicitly through `s8h`'s one spawn helper and never inherited (`s8h`'s lint, below; *re-pointed 2026-09-24, final consistency pass*), because `s8h` §8h.7 requires one of *"Binary-level tests that drive the real harness at the fixture child"*. Each `run` stops at a gate, the gate is answered, and the next `run` is a new process. The gates come in the order the lane reaches them, and they include triage's `unknown_facet` gate for `change`, the human-root gate and the §8j.3 hold. The last `run` reaches the landing. **That is at least five separate processes.** The lane continues across each boundary from the stage-bearing trailer (D21-41 (a)). The final trailer sequence equals the in-process arm's. |
| `lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification` | see §8j.3 |
| `lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification` | see §8j.3 |
| `lane_proof::boundary_hold_changes_requested_does_not_advance` | see §8j.3 |

🛑 **A one-hop fixture cannot judge a multi-hop guard.** The lanes cross nine and ten boundaries, and the binary arm
crosses at least four process boundaries. A green that held on the first hop would prove nothing about the fifth.
That is why the stage lists are compared **whole**, and why the multi-process arm ends at the landing and not at the
first successful re-entry.

**Mutants.** Each is run from Python, never the shell. Each must flip the named test red, and a survivor blocks.

| mutant | the change | must redden |
| --- | --- | --- |
| `M-seed-record` | `through_step_seven` seeds the stage from the item record, ignoring the reconciled trailer | `…_second_process_continues_binary_arm`, `…_answered_lets_the_next_process_enter_specification` |
| `M-land-empty` | the landing commits `allow_empty` with nothing staged | `fix_lane_reaches_a_landed_content_commit_under_fake_harness` |
| `M-fixture-toolchain` | the composition root builds `FixtureToolchain` | `fix_lane_verify_completed_names_the_rust_adapter` |
| `M-skip-empty-stage-trailer` | a `stage_empty` stage mints no trailer | both `…_trailers_every_non_forbidden_stage_in_order` |
| `M-spec-digest-pre-render` | `artifact.write`'s digest is taken over the pre-render value, not the written bytes | `story_lane_spec_is_landed_and_journalled` |
| `M-worktree-under-git` | the run-worktree root is derived under the common dir again | `fix_lane_reaches_a_landed_content_commit_under_fake_harness` |
| `M-attach-dropped`, `M-hold-ignored`, `M-hold-spent-refires` | see §8j.2 and §8j.3 | as named there |
| `M-hold-unordered-accepted` | see §8j.3 | `run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load` |

*`M-launch-env-inherited` is withdrawn (2026-09-24, final consistency pass):* its lint is `s8h`'s, whose mutant M26
covers the same deletion (below). The mutants are ten.

⚠️ **A fixture per function is not a fixture per branch.** Some proofs turn on a branch: `stage_empty` against a
stage that selected activities, or `approve` against `changes-requested`. Each branch then has its own assertion.
The mutant table exists to show that each branch can be observed.

**The launch-record lint is `s8h`'s (D21-47; review finding 3).** D21-47: *"Launch-gate tests build their `Env`
explicitly, enforced by a lint."* *Re-owned 2026-09-24 (final consistency pass).* The draft built a lint of its own,
`root::every_binary_spawn_in_the_suite_sets_the_launch_record_explicitly`, while `s8h` builds the same rule one stage
earlier: *"every `Command` over the `redkiln` binary in `rk-cli`'s `tests/it` is built by one helper. The helper calls
`.env_remove("REDKILN_LAUNCH_RECORD")`, and it sets the variable only for an explicit fixture record."* Its lint is
`run::every_binary_spawn_withholds_or_supplies_the_launch_record`, with mutant M26 (`s8h` §8h.7, §8h.13), and `s8h`
folds the spawn sites into that helper. Two lints over one rule would drift, so this stage's draft lint and its mutant
`M-launch-env-inherited` are withdrawn, and this stage adds no calls. The binary arm spawns through `s8h`'s helper.
§8j.6 step 1b and Checkpoint 8j (2) select `s8h`'s lint beside this stage's fourteen tests, because 7c's split
(§8j.6 step 2) relies on it.

🛑 **What this section does not claim.**

- It does not claim AC-473 or AC-521. Those are claimed by real runs on this repository, which are `s8`'s.
- It does not claim that the review gate is guarded against false approval. ADR-0101 clause 5 forbids that until
  S-SUBTLE-CB runs.
- It describes cross-process continuation only as far as the record `s8e` wrote for D21-41 (a) permits. ADR-0157
  decision 5 stands wherever that record did not supersede it.

### 8j.5 The fix item: a shortlist, the operator's pick, a reservation, and a fresh item (D21-42 18)

D21-42 18: *"a shortlist of **at least three behavioural defects**: reproducible by a failing `cargo test`, one crate,
not in machinery the run exercises, and not scheduled for a sub-stage fix. The operator picks one, it is reserved in
the ledger, and a fresh item is minted."*

**The survey method.**

1. **Exclude what the run exercises.** Drop every function reachable from the call graphs of `verbs::run::run`,
   `verbs::answer`, the lane's `rk-kernel` runtime, replay, fsm and selection, all of `rk-git`, every `rk-adapter-*`,
   `rk-containment`, `rk-store`'s render and read path (`s8f`), and the verbs `s8`'s blocks read (`explain`'s stage
   and output paths, `cost`, `doctor --rebuild`).
   - The check is **static**: a caller walk up to those entry points.
   - It is also **dynamic**: with the candidate's repro test added, all ten `lane_proof` tests stay green. That is
     necessary, not sufficient.
   - `s8f`'s render path puts `rk_store::locator` on the run path. At every entry, the drift pass in `s8f` §8f.5
     *"locates the section by anchor (the locator's byte range)"*. So C2 and C6 are **excluded** under this rule.
     Once `s8f` has landed, the callers of every remaining `rk-store` candidate are walked again.
2. **Exclude what is scheduled.** Drop any candidate whose file or function is named by `s8a`…`s8i` (`s8d-ii` included), or listed among
   their owned decisions in the plan. For example, doctor's journal source and `RK-D-HEADROOM-MARGIN` are D21-42 12c,
   and rk-store's `conn.rs` poison unwrap is L3.
3. **Reproduce in a throwaway tree, and never in the crate.** Extract HEAD into `target/` (`git archive`), append the
   repro to its file there, and watch it fail for the named reason. 🛑 **The repro test is not committed
   into any crate.** If it were, it would turn CI red, and it would hand the lane's `red` step its answer. It is
   committed as `.redkiln/intake/<fix-item>.repro.rs`, with `.redkiln/intake/<fix-item>.repro.toml` giving its
   `kind`, `crate`, `module`, `test` and the `expect` substring of its failure. The BugReport the real run attaches
   points at both. *Kinds added 2026-09-24 (D21-47; review finding 11):*
   - `kind = "it"` appends the test to `crates/<crate>/tests/it/<module>.rs` and selects `<module>::<test>` under
     `--test it`.
   - `kind = "unit"` names a `file` under `crates/<crate>/src/`. The repro is a whole
     `#[cfg(test)] mod <name> { use super::*; … }` appended to that file, `module` is that module's full path (for
     example `verbs::explain::s8j_repro`), and the selection is `--lib`. A child module sees its parent's private
     items, so a private function such as C3's is reachable.
   - A repro that needs fixture files writes them into a temporary directory itself, as C4's does. The pair carries
     no directory.
4. **One crate, low blast radius.** The fix touches one crate's `src/`, in about one function.
5. **Present at least three to the operator.** For each, give the file, the quoted code, the contract it breaks
   (quoted from its own doc, an ADR or the register), the red output from step 3, and why it is outside the run.

**Candidates found by reading the code at `ca8f387`.** These are **candidates only**. Each was checked by tracing the
code, and none has yet been run red. Step 3 runs each one before it is presented.

| # | crate · function | the defect (quoted code) | the contract it breaks | repro test (proposed) | caveat |
| --- | --- | --- | --- | --- | --- |
| C1 | `redkiln-migrate` · `legacy::fence::split` | `let tail = src[after + close..].trim_start_matches("\n```");`. `trim_start_matches` strips the pattern **repeatedly**, so a fence that directly follows the closing one loses its opening backticks. | `prose` is *"Everything outside it"* (the `Fenced` doc). Over a yaml fence followed directly by a `diff` fence, `prose` should begin with the second fence's three backticks. It begins with the bare word `diff`. | a new test in `redkiln-migrate` `tests/it`, over `split` | Only `LegacyLedger.prose` carries it, so the impact is low. It is the migrator, which is outside the run. |
| C2 | `rk-store` · `locator::scan` | `let is_fence = trimmed.starts_with("```") \|\| trimmed.starts_with("~~~");` is judged on the **trimmed** line and **before** the HTML-comment branch. So an indented ```` ``` ```` (four or more spaces), or an unmatched ```` ``` ```` inside `<!-- … -->`, opens a fence, and every later heading becomes invisible. | The module's own rule, under AC-150: a heading-looking line *"inside a fenced code block, an indented block, an HTML comment … is content"*. It says so in terms: *"An indented block is code, not a heading."* | `locator::an_indented_fence_marker_is_not_a_fence` and `locator::a_fence_marker_inside_an_html_comment_is_not_a_fence` in `rk-store` `tests/it`. Each resolves `## Target` after the decoy, and each gets `Occurrences { count: 0 }` today. | **EXCLUDED under step 1.** At `a6349b6`, `locator::` has no caller outside its own test file, but `s8f` §8f.5's render path puts it on the run path. |
| C3 | `rk-cli` · `verbs::explain::top_level_conjuncts` / `false_conjunct` | `if depth == 0 && src[i..].starts_with(" and ")`. Depth counts only `( [ ) ]`, so the split cuts inside a quoted string, and it cuts `a or b and c` into `a or b` and `c`. | Its own doc: *"A disjunction is **not** decomposed — no single disjunct of a false `or` is "the" reason, so naming one would be inventing a cause."* | `kind = "unit"`: a `#[cfg(test)] mod` appended to `src/verbs/explain.rs` (the function is private). `false_conjunct("kind == 'x and y'", …)` returns `kind == 'x`, not the whole predicate. | `explain` is a read-only verb. `s8` reads its stage output, and this path runs only when explaining a false predicate. |
| C4 | `rk-cli` · `doctor::rules::margin` (`RK-D-MARGIN`) | `let Some(margins) = table(&pack, "margins") else { continue; };` and `margins.get(window).and_then(toml::Value::as_integer)`. A pack with no `[margins]`, with a missing window, or with a non-integer margin passes silently. | AC-377: *"AND `redkiln doctor` reaches the same verdict without dispatching a model call"*. Its Verified-by includes *"a fixture config with no margin that"* must fail. | `kind = "it"`: a test in `rk-cli`'s `tests/it/doctor.rs` that writes its own pack, with `[margins] five_hour = 10` and no `seven_day`, into a temporary repository (the repro pair carries no fixture directory). It should produce an `RK-D-MARGIN` error, and today it produces nothing. | This is not `RK-D-HEADROOM-MARGIN`, which is D21-42 12c's. Re-verify step 2 against `s8f`'s file. |
| C5 | `rk-types` · `schema::lint::walk` | `Value::Object(_) => walk(child, &p, child_inside_union && key != "properties", out)` resets the inside-union flag under every `properties` map. So a union under an object that is itself inside a union is never flagged. | AC-131's instrument is the `NestedUnion` rule, *"A union with a union descendant (AC-131)"*. | `schema_no_oneof::nested_union_through_properties_is_caught` | **Lower confidence.** The `key != "properties"` exclusion is explicit and may be deliberate. Read its history before presenting it. |
| C6 | `rk-store` · `locator::parse_heading` | `text: rest.trim().trim_end_matches('#').trim()` strips a trailing `#` that has no space before it, so `## C#` resolves as `C`. | The locator's ATX reading, in which a closing `#` run needs a preceding space. | `locator::a_trailing_hash_without_a_space_is_heading_text` | **EXCLUDED under step 1**, as C2 is. |

The survey's pass also found three candidates that the method **excludes**:

- `rk-types` `Digest`'s derived `Deserialize` skips `from_hex`'s validation. Its doc says *"The only constructor: 64
  lowercase hex characters"*. But digests are deserialised on the run and replay paths, which fails rule 1.
- `rk-kernel` `toml_subset` admits a table defined twice.
- `catalog::walk` drops a non-string `deviation_ok` entry silently.

The last two sit on the pack and catalog load path the run takes.

**What is presented.** Four candidates remain: C1, C3, C4 and C5. That meets the floor of three. C5 carries its
lower-confidence caveat, and C4 carries its re-check against `s8f`'s file.

**Reservation and mint.**

1. The operator picks one candidate.
2. The pick is reserved by a dated ledger entry. The entry carries one machine-readable line,
   `reserved-fix-item: <id> <crate>:<function>`, and it forbids fixing the defect by hand or in any stage before
   `s8`.
3. A fresh `.redkiln/items/<id>.toml` is minted with an `id` of the form `rk-fix-<slug>` and these fields:
   - `kind = "bug"`, `stage = "triage"`;
   - `hold_before = "specification"` (§8j.3);
   - `slice = "<slug>"`: the id without its `rk-fix-` prefix, so never the id itself (`s8e` §8e.8);
   - `[trial] activity = "tag-release"` (§8j.1(e));
   - `[facets]`: `cos = "standard"`, `has_repro = true`, and low `criticality`, `blast_radius` and `size`.
4. The item's BugReport (§8j.2) and its repro pair (step 3) are committed with it.
5. **Neither `rk-run/<id>` nor `.redkiln/worktrees/<id>` exists at this stage's close.** The run ref forks at `s8`'s
   first `run`, from a HEAD descending from the CI-green frozen commit (§8j.7). `s8`'s amended (1) asserts that
   ancestry, and it asserts that the code paths are unchanged between the two. The run ref is not merged into the
   checkout before Checkpoint 8 passes. Every pre-spend check and (1)'s check block refuse a merged ref (§8j.8 A1;
   review finding 13).

### 8j.6 The probe session: the pin move, Checkpoint 7c split, and `s8h`'s two live probes — paid, after the fixture proofs, each an operator go-ahead

**Entry condition.** Every `lane_proof` test, the §8j.2 and §8j.3 companions, `s8h`'s launch-record lint, and every
mutant cycle in §8j.2–§8j.4 are green. That is **recorded mechanically** before any paid step: step 1b's block
writes the `fixture-proofs` line and one `mutant` line per mutant, over a clean commit that already carries the
moved pin (D21-47; review finding 7). D21-40 decision 2 authorises the spend *"once the sub-stages' fixture proofs
pass, and not before."* RT-57's record, `s8h`'s Record B, is `accepted` before any step here. Its id goes on the
evidence file's `record-rt57` line, because D21-47 names it among the pre-spend checks and Checkpoint 8j (6) reads
it.

🛑 **Who runs it, and where.** The operator runs every step below in one sitting, the probe session. They run from a
plain terminal, never from a Claude Code session or a shell an agent spawned. The operator's own `! python …` line is
not an alternative here: `s8h` §8h.7 says *"a `!` line that invokes it from a session is refused at step 2"*.
Every dispatch goes through the committed launcher `s8h` authored, `tools/live-run.sh` (D21-42 17a: *"A committed
launcher script is the only entry to a real run"*), against `~/.claude-personal` (17b). A Claude Code session
exports `CLAUDECODE`, `CLAUDE_CODE_*`, `CLAUDE_PID` and `CLAUDE_EFFORT`, and the launcher refuses each of them
(`s8h` §8h.7, step 2). That refusal is the design working, not an
obstacle to route around (Halt 7). D21-44 records the same wall from the other side: *"the session's permission layer
refused to spawn a `claude` child from inside Claude Code"*. **The operator records each result** in
`runbook/phase-1/s8j-evidence.txt` (format below) as it happens. No agent writes a `pass` line. If `s8h`'s launcher
offers no entry for a dispatch below, **STOP and raise it**. Do not run the dispatch bare.

**The order, and what each step spends.**

1. **The freeze and the pin move, by `s8h` §8h.8's steps 1–3, which are not restated here.** `s8h` authored the
   procedure and says it is *"executed at `s8j`"*. Its step 4 is the `live_dispatch_probe` dispatch, and step 2
   below takes that dispatch as part of Checkpoint 7c's live portion, so the probe is not dispatched twice. Its step 5,
   *"Only then probe 0, and after it probes 1 and 2"*, is steps 2b, 3 and 4 below (probe 0 added 2026-09-24, final
   consistency pass). Steps 1–3 cover five things (the fifth added by D21-48, 2026-09-24):
   - the operator's `DISABLE_AUTOUPDATER` setting;
   - the instrument file re-written by `printf`, never by `tee`;
   - the constant, its prose, its literal test and every `claude_code_version` fixture, moved together;
   - `PERMISSION_PROMPTS_FLOOR` asserted below the pin;
   - **the argv spelling change, in the same pin-move commit** (*D21-48 (2026-09-24):* X12). X12 reads: *"The adapter
     emits the spelling the PINNED CLI's `--help` lists, fixed at s8j's pin move (`manual` on 2.1.281). Cards keep
     `default`, and the init comparison reads `default`. The golden argv snapshots move once, at the pin move."*
     So the commit makes three changes:
     - `s8h` §8h.2's `PERMISSION_MODE_ARGV` loses its `2.1.278` row, and `LIVE_WARRANTED` loses that row's entry. One
       row stays, keyed by the new pin, with its `--help` capture under
       `crates/rk-adapter-claude/tests/fixtures/cli-help/`;
     - the golden argv fixtures and their `insta` snapshots move in their `--permission-mode` element only;
     - `command::argv_permission_mode_reads_the_row_of_the_configs_pin`'s `2.1.278` arm becomes a refusal arm.

     If the pin moves to a version that has no capture yet, the capture is taken here by `s8h` §8h.0 item 6's rule,
     before the constant moves. A capture that lists neither `manual` nor `default` stops the move and is reported.

   If `claude --version` already equals `TESTED_CLI_VERSION` (`"2.1.278"` at `a6349b6` [measured]), the move is
   recorded as unneeded. **The adapter pin `TESTED_CLI_VERSION` equals S-BIN's pinned `cli_version`**: `s8i` §8i.5
   hands this on (*"The adapter pin `TESTED_CLI_VERSION` equals S-BIN's pinned `cli_version`."*), because D21-43 N22
   froze the CLI from S-BIN on. If the installed version differs from S-BIN's pin, the freeze failed, and that is
   Halt 2 to report, not a pin to move. Checkpoint 8j (6) asserts the equality. The mismatch policy stays
   `MismatchPolicy::Refuse`, hard-coded in `real_harness_config`
   (`crates/rk-cli/src/verbs/run.rs` [measured]). That is why the pin moves before any dispatch. Under D17-11's
   amendment, *"the adapter refused all three dispatches at the init line"*.
   **Spend: none.**
1b. **The recorded fixture proofs, after the pin move and before any paid step** *(added 2026-09-24, D21-47; review
   finding 7).* The draft's entry condition was a sentence, and nothing mechanical stood between it and the first
   dispatch. The block below writes the gate's evidence. It runs over a clean commit that carries the moved pin, so
   the proofs are re-run after the pin move. It runs the fifteen tests Checkpoint 8j (2) selects, under D21-47's
   wrapper, and reads the Python mutant driver's results file. Every mutant in §8j.2–§8j.4 must be `killed` at this
   HEAD. Only then does it append `fixture-proofs <HEAD> pass <date>` and one `mutant <id> killed <date>` line per
   mutant. The pre-spend checks (§8j.8 A1) and Checkpoint 8j (6) and (7) read those lines. They require the proofs'
   code to be the code CI ran, and every paid run's launch record to carry it. **Spend: none.**

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# s8j §8j.6 step 1b: THE SPEND GATE'S EVIDENCE (D21-40 decision 2; D21-47). Appends `fixture-proofs` and one `mutant`
# line per mutant to the evidence file, and nothing else. It dispatches nothing.
ev=runbook/phase-1/s8j-evidence.txt
test -f "$ev" || { echo "FAIL: $ev is absent"; exit 1; }
st=$(git status --porcelain -- . ':!runbook/phase-1/s8j-evidence.txt'); rc=$?
[ "$rc" -eq 0 ] && [ -z "$st" ] || { echo "FAIL: the tree is not clean apart from the evidence file (rc=$rc) — the sha recorded would not be what ran:"; printf '%s\n' "$st"; exit 1; }
head=$(git rev-parse HEAD) || { echo "FAIL: git rev-parse HEAD failed"; exit 1; }
grep -qE '^pin-move ' "$ev" || { echo "FAIL: no pin-move line — step 1 comes first, so the proofs run over the moved pin"; exit 1; }
tdir=target/s8j-proofs; rm -rf "$tdir"; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
# D21-47's wrapper, the shared helper (final consistency pass 2026-09-24): the compiled fake claude first on PATH, the
# trap config, no launch record. The item variables s8 sets are unset too, so no proof reads a real item.
. runbook/tools/nodispatch.sh && nd_setup "$PWD/$tdir/nd" || exit 1
wrapped() { nodispatch env -u FIX_ITEM -u STORY_ITEM -u ITEM "$@"; }
out=$(wrapped cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
R_TESTS="lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness lane_proof::fix_lane_trailers_every_non_forbidden_stage_in_order lane_proof::fix_lane_verify_completed_names_the_rust_adapter lane_proof::story_lane_spec_is_landed_and_journalled lane_proof::story_lane_trailers_every_non_forbidden_stage_in_order lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes lane_proof::gate_then_answer_then_second_process_continues_binary_arm lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification lane_proof::boundary_hold_changes_requested_does_not_advance processes::the_two_human_root_questions_load_and_each_attaches_its_kind processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load run::every_binary_spawn_withholds_or_supplies_the_launch_record"
out=$(wrapped cargo test -p rk-cli --test it -- $R_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the fixture proofs did not pass (rc=$rc) — no spend (D21-40 decision 2)"; printf '%s\n' "$out" | tail -30; exit 1; }
for t in $R_TESTS; do printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok"; exit 1; }; done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 15 passed; 0 failed' || { echo "FAIL: the proofs did not run exactly 15 tests"; exit 1; }
nd_check "the fixture proofs" || { echo "FAIL: a claude process ran under the proofs — no spend (D21-40 decision 2)"; exit 1; }
# The mutant driver (Python, never the shell; sha256-verified restores) writes <id><TAB>killed|SURVIVED<TAB><HEAD sha>.
res=target/s8j-mutants/results.tsv
test -s "$res" || { echo "FAIL: no mutant results at $res — run the §8j.2–§8j.4 mutant driver at this HEAD first"; exit 1; }
S8J_MUTANTS="M-seed-record M-land-empty M-fixture-toolchain M-skip-empty-stage-trailer M-spec-digest-pre-render M-worktree-under-git M-attach-dropped M-hold-ignored M-hold-spent-refires M-hold-unordered-accepted"
for m in $S8J_MUTANTS; do
  r=$(awk -F '\t' -v m="$m" -v h="$head" '$1==m && $3==h {s=$2} END {print s}' "$res")
  [ "$r" = killed ] || { echo "FAIL: mutant $m is '${r:-unrun}' at $head — a survivor blocks, and a result at another HEAD is not this code's"; exit 1; }
done
d=$(date +%F)
{ printf 'fixture-proofs %s pass %s\n' "$head" "$d"; for m in $S8J_MUTANTS; do printf 'mutant %s killed %s\n' "$m" "$d"; done; } >> "$ev" || { echo "FAIL: could not append to $ev"; exit 1; }
echo "fixture-proofs $head recorded, with $(printf '%s\n' $S8J_MUTANTS | grep -c .) mutants killed"
```

2. **Operator go-ahead, then Checkpoint 7c, split (D21-43 N28; D21-47).** `s8a` makes 7c provisional and says *"its
   whole re-run is `s8j`'s (D21-43 N28)"*. D21-43 N28: *"**s8j re-runs it whole**, from a plain terminal through the
   launcher, with the live probes."* *Rewritten 2026-09-24 for D21-47 (review finding 3):* the draft ran all of 7c's
   block as one `wrap` child. The launcher exports a real `REDKILN_LAUNCH_RECORD` and the operator's real
   `CLAUDE_CONFIG_DIR` to that child, so 7c's own `cargo test --workspace` ran every launch-gate test with the gate
   open, and any test that spawned the real harness could have reached the real CLI on the operator's account.
   D21-47: *"Checkpoint 7c's whole re-run at s8j is split. Its suite portion runs OUTSIDE `wrap`: no launch record, the
   trap config and the fake `claude`. Only its live-probe portion runs under `wrap`. Launch-gate tests build their
   `Env` explicitly, enforced by a lint."* The lint is `s8h`'s `run::every_binary_spawn_withholds_or_supplies_the_launch_record`
   (§8j.4). The whole of 7c still runs, and nothing in `s7c-adapters.md` is edited except A12's `-H` (§8j.8), which
   moves no reading:
   - **The split.** The block is extracted verbatim from `runbook/phase-1/s7c-adapters.md`, as the last fenced
     block that echoes `CP7C-END-OF-BLOCK reached`, into a file under `target/`. It is cut at its line that opens
     `# ---- THE FIFTEENTH LINE, as statements.` The **suite portion** is everything above that line, with a sentinel
     appended. The **live portion** is the block's `cd` line followed by everything from that line through the end
     marker. The live portion defines its own `pin`, `out` and paths, and it reads nothing the suite portion set.
   - **The suite portion, then a zero-spend dry run, with no go-ahead.** The first block below runs the suite portion
     under D21-47's wrapper, `runbook/tools/nodispatch.sh`. It then runs `tools/live-run.sh --check wrap -- bash <live
     portion>` under the same wrapper, so the helper's compiled fake `claude` is the one the launcher resolves. That
     dry run performs every refusal and capture `wrap` makes, spawns nothing and writes no launch record (`s8h`
     §8h.7). The fake's call log must stay absent (it logs every call but `--version`), and the trap must stay empty:
     `nd_check`. The block appends `checkpoint-7c-suite <HEAD> pass <date>`.
   - **The live portion, with the operator's go-ahead.** Part 2 below is one launcher command, given as prose because
     no block of this file dispatches:
     `tools/live-run.sh wrap -- bash <live portion>`. The launcher sets `CLAUDE_CONFIG_DIR` and
     `DISABLE_AUTOUPDATER`, writes the launch record, and exports `REDKILN_LAUNCH_RECORD` to the child (steps 4, 5
     and 8). The portion's own bare line,
     `cargo test -p rk-adapter-claude --features live -- --ignored --nocapture live_dispatch_probe`, therefore passes
     the probe's first statement: *"Every `live::` test asserts the same variable as its first statement"* (`s8h`
     §8h.7; re-quoted 2026-09-24, final consistency pass). Under `wrap`, step 6 records the CLI version and does not compare it. The portion compares it itself,
     against the pin, as Halt 2's guard. It passes only if it reaches its end marker, and the launcher's rc is the
     child's.

   **This one live run is also the `live_dispatch_probe` re-run** that D21-42 16 asks for after a pin move, so the
   probe is not dispatched a second time. **Spend: one haiku dispatch.** `PROGRESS.md` row 7c is updated only from
   these two blocks together. *D21-48 (2026-09-24):* `live_dispatch_probe`'s card, `probe.json`, says `default`, so
   this is the first live argv to carry the pinned spelling. A parser refusal of it is `s8h` §8h.11's wrong-capture
   HALT, as in step 2b.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# s8j §8j.6 step 2, part 1: 7c's SUITE PORTION outside wrap, then a zero-spend `--check wrap` dry run (D21-47). No spend.
tdir=target/s8j-7c; rm -rf "$tdir"; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
xs='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
b=[x for x in re.findall(f+"bash\n(.*?)"+f,t,re.S) if "echo \"CP7C-END-OF-BLOCK reached\"" in x]
if not b: sys.exit("no 7c block")
L=b[-1].split("\n")
cut=[i for i,l in enumerate(L) if l.startswith("# ---- THE FIFTEENTH LINE, as statements.")]
if len(cut)!=1: sys.exit("the cut line occurs %d times" % len(cut))
k=cut[0]
open(sys.argv[2],"w",encoding="utf-8",newline="\n").write("\n".join(L[:k])+"\necho \"CP7C-SUITE-PORTION reached\"\n")
open(sys.argv[3],"w",encoding="utf-8",newline="\n").write(L[0]+"\n"+"\n".join(L[k:]).rstrip("\n")+"\n")'
python -c "$xs" runbook/phase-1/s7c-adapters.md "$tdir/suite.sh" "$tdir/live.sh" || { echo "FAIL: 7c's block could not be split at its FIFTEENTH LINE"; exit 1; }
tail -1 "$tdir/live.sh" | grep -qxF 'echo "CP7C-END-OF-BLOCK reached"' || { echo "FAIL: the live portion does not end at 7c's end marker"; exit 1; }
# A12 (s8j §8j.8) landed: 7c's rk_absent reads with -H, and the old spelling is gone.
grep -qF 'raw=$(grep -rnHE "$1" $scope --include=*.rs); rc=$?' "$tdir/suite.sh" && ! grep -qF 'raw=$(grep -rnE "$1" $scope' "$tdir/suite.sh" || { echo "FAIL: 7c's rk_absent does not read with -H — A12 did not land (s8j §8j.8)"; exit 1; }
# D21-47's wrapper, the shared helper (final consistency pass 2026-09-24): 7c's suite portion reaches only the compiled
# fake claude, with the trap config and no launch record.
. runbook/tools/nodispatch.sh && nd_setup "$PWD/$tdir/nd" || exit 1
out=$(nodispatch bash "$tdir/suite.sh" 2>&1); rc=$?
last=$(printf '%s\n' "$out" | grep . | tail -1)
[ "$rc" -eq 0 ] && [ "$last" = "CP7C-SUITE-PORTION reached" ] || { echo "FAIL: 7c's suite portion rc=$rc, last line '$last'"; printf '%s\n' "$out" | tail -25; exit 1; }
out=$(nodispatch bash tools/live-run.sh --check wrap -- bash "$tdir/live.sh" 2>&1); rc=$?   # the launcher sets its own CLAUDE_CONFIG_DIR (s8h §8h.7 step 4); its `claude --version` meets the fake
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qxF 'LAUNCH_CHECK=ok' && printf '%s\n' "$out" | grep -qxF 'MODE=wrap' || { echo "FAIL: the zero-spend --check wrap dry run did not pass (rc=$rc) — this terminal cannot launch the live portion"; printf '%s\n' "$out" | tail -10; exit 1; }
nd_check "7c's suite portion and the dry run" || { echo "FAIL: a claude process ran during the suite portion or the dry run"; exit 1; }
printf 'checkpoint-7c-suite %s pass %s\n' "$(git rev-parse HEAD)" "$(date +%F)" >> "$ev" || { echo "FAIL: could not append to $ev"; exit 1; }
echo "7c's suite portion passed and the dry run spawned nothing; the live portion is $tdir/live.sh — operator go-ahead next"
```

**Part 2, the live portion, on the operator's go-ahead**, from a plain terminal and in no block of this file, because
no block here dispatches: `tools/live-run.sh wrap -- bash target/s8j-7c/live.sh`. It is ONE haiku dispatch. The
operator appends `checkpoint-7c <cli-version> pass <date> <launch-record basename>` only if the portion reaches
`CP7C-END-OF-BLOCK reached` and the launcher exits 0. Otherwise it stops and reports, and spends nothing again
(D21-40 decision 2).

2b. **Operator go-ahead, then `tools/live-run.sh probe live::live_denial_channel_preprobe`** *(scheduled 2026-09-24,
   final consistency pass; D21-47)*. This is `s8h` §8h.11's probe 0, which `s8h` hands on: *"Probe 0, run before probe
   1 in its own block on its own go-ahead (one haiku dispatch)"*. It measures guard 3's denial channel at haiku price
   before probe 1 spends a Sonnet. It also **confirms** the argv spelling that step 1's pin move fixed (*D21-48
   (2026-09-24):* X12, *"Probe 0 confirms rather than decides"*; this read *"and whether the argv accepts
   `--permission-mode default`"*). Its pass condition and its two HALTs are `s8h` §8h.11's. A denial seen only as an
   errored `tool_result` HALTS the work before probe 1, and so does a refused argv. The refusal is no longer a stop for
   the lead's decision (*"a refused argv stops it before probe 1 for the lead's decision on the spelling"*, before
   D21-48). It means the `--help` capture, the table row or the installed binary is wrong. The repair is a fresh
   capture and a red-first row correction in its own commit, which re-opens step 1b like any code change. If its retained frame differs from
   guard 3's drafted channel (b), `s8h` §8h.5's correction is a code change. It is made red-first in its own commit,
   and, as a CI repair does (§8j.7), it re-opens step 1b and every paid step whose launch record must carry the frozen
   code, each on a fresh go-ahead. **Spend: one haiku dispatch.**
3. **Operator go-ahead, then `tools/live-run.sh probe live::live_confinement_and_denial_probe`.** This is `s8h`'s
   probe 1 (D21-42 21d), and its pass condition is `s8h` §8h.11's. That condition includes the edit *inside* a
   worktree under `.redkiln/worktrees/`, which D21-44's N1b arm measured writable. 🛑 If the CLI cannot enforce the
   deny rules, **the work HALTS** and the gap is reported as a containment finding. D21-42 21d takes this in exactly
   these words.
   **Spend: one Sonnet dispatch.**
4. **Operator go-ahead, then `tools/live-run.sh probe live::live_init_equality_probe`.** This is `s8h`'s probe 2
   (21b): the tools, mode, model and auth are read from the init line.
   **Spend: three dispatches, one per pinned model id** (D21-43 N17 Q4).
5. **A failed or unexpectedly refused step stops the stage.** Nothing is re-spent until the operator has read the
   report. That is D21-40 decision 2's rule: *"stop and report, not to spend again"*.
6. **The (g) record flips only after both probes pass.** D21-41 (g): *"The record is accepted only after two live
   probes pass, and those probes run after the fixture proofs."* That record is `s8h`'s Record A. The flip is the
   operator's dated ledger line. Only the frontmatter, the README index cell and the census move.

**The spend table** *(probe 0 added 2026-09-24, final consistency pass)*. The session spends six dispatches in all:

| step | launch | model | dispatches |
| --- | --- | --- | --- |
| 2 | Checkpoint 7c's live portion, through `wrap`, which is also the `live_dispatch_probe` re-run | haiku | 1 |
| 2b | probe 0, `live::live_denial_channel_preprobe`, through `probe` | haiku | 1 |
| 3 | probe 1, `live::live_confinement_and_denial_probe`, through `probe` | Sonnet | 1 |
| 4 | probe 2, `live::live_init_equality_probe`, through `probe` | one per pinned model id | 3 |

Each launch leaves its launch record under
`<git-common-dir>/redkiln/launches/` (`s8h` §8h.7, step 8). Its basename goes on the evidence line, so Checkpoint 8j
can find it.

**The evidence file** is `runbook/phase-1/s8j-evidence.txt`. It is **not under `spikes/results/`**, because
`ci.yml`'s AC-465 job fails on any tracked path there except `RUNBOOK-instrument.txt`. It is space-separated, holds
one fact per line, and is appended in execution order. *Amended 2026-09-24 (D21-47; review finding 7):* the
`record-rt57`, `fixture-proofs`, `mutant` and `checkpoint-7c-suite` lines are new. The pre-spend checks and Checkpoint
8j (6) read them. *Amended 2026-09-24 (final consistency pass):* probe 0's `probe` line is new, and (6) reads it.

```
record-rt57 <four-digit id of s8h's Record B, accepted>
pin-move <old-version> <new-version|unchanged> <yyyy-mm-dd>
fixture-proofs <40-hex HEAD sha> pass <yyyy-mm-dd>
mutant <id> killed <yyyy-mm-dd>            (one line per mutant of §8j.2–§8j.4, ten in all)
checkpoint-7c-suite <40-hex HEAD sha> pass <yyyy-mm-dd>
checkpoint-7c <cli-version> pass <yyyy-mm-dd> <launch-record basename>
probe live::live_denial_channel_preprobe <cli-version> pass <yyyy-mm-dd> <launch-record basename>
probe live::live_confinement_and_denial_probe <cli-version> pass <yyyy-mm-dd> <launch-record basename>
probe live::live_init_equality_probe <cli-version> pass <yyyy-mm-dd> <launch-record basename>
record-g <four-digit id of the record s8h wrote for D21-41 (g)>
legacy-worktree-repointed rk-fix-doctor-test-count 73c087a <yyyy-mm-dd>
ci <github-run-id> <40-hex head sha> <yyyy-mm-dd>
fix-item <reserved id> <40-hex frozen sha>
story-item rk-story-harden-spike-pack <40-hex frozen sha>
```

### 8j.7 CI green on the pushed branch — every job, both OS legs (D21-42 14a)

D21-42 14a: *"`ci.yml` green on the pushed branch, every job, both OS legs, is a Checkpoint 8 condition. Each push
is confirmed with the operator."* The branch has never been pushed (`git branch -r` shows `origin/main` only), and
`ci.yml` has never run. Its own comments say so: *"ARMED, NOT DISCHARGED … this workflow is authored and has never
run."*

**The known missing step.** In the `gates` job, before `test`, add the step below. The warrant is `992d257`'s message,
*"the first fresh run failed 10 rk-adapter-toolchain-node tests on the documented pnpm preflight (node_modules is
git-ignored), and ci.yml has no pnpm step at all"*.

- `pnpm` is installed through `npm` at the version that wrote `pnpm-lock.yaml`: `lockfileVersion: '9.0'`, and pnpm
  10.33.0 as measured on this box by `s7c`.
- No marketplace action is added. `ci.yml`'s own supply-chain stance is *"NO MARKETPLACE ACTION beyond
  `actions/checkout`"*.
- *D21-48 (2026-09-24):* `ci.yml`'s cargo-nextest install step is `s8c`'s. X15: *"s8c installs cargo-nextest on this box
  and in CI … The install is s8c's act, at stage open."* This stage adds no nextest step and does not duplicate or move
  that one. If `s8c` rewrites the test gate line, Checkpoint 8j (7)'s `run: cargo test --workspace` anchor is re-read
  from `ci.yml` as `s8c` lands it, by a dated edit. *D21-48 (2026-09-24), cross-file pass:* `s8c` §8c.1 settles it:
  *"The gate's own command stays `cargo test --workspace`"*, and the install step sits immediately before `test`.
  So (7)'s anchor and its pnpm-before-test order hold unedited.

```yaml
      # D21-42 14a (s8j §8j.7): the Node toolchain adapter's fixture project needs node_modules before
      # `cargo test --workspace`, or ten rk-adapter-toolchain-node tests fail at tests/it/support.rs's
      # preflight (992d257). pnpm comes from npm at the version that wrote pnpm-lock.yaml -- no
      # marketplace action -- and --frozen-lockfile makes a lockfile drift fail HERE.
      - name: node fixture dependencies (pnpm, frozen lockfile)
        shell: bash
        run: |
          node --version
          npm install --global pnpm@10.33.0
          pnpm --version
          pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project
          test -e crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules/.bin/vitest \
            || { echo "FAIL: no node_modules/.bin/vitest after pnpm install -- the node suite would fail at its preflight"; exit 1; }
```

**Whatever else a first CI run needs is found by running it, and it is not guessed at here.** The loop:

1. 🛑 Get the operator's go-ahead, then `git push -u origin build/phase-0-to-1` (the first time) or `git push`.
2. Read the run with `gh run view <id> --json conclusion,headSha,headBranch,jobs`.
3. For each failing job, diagnose from the log. A code defect is fixed red-first, in its own commit. A test that
   fails on one OS and not the other is a **finding**, never a `#[cfg_attr(…, ignore)]`: AC-121's cross-OS half is
   the reason the matrix has two legs.
4. 🛑 Get the operator's go-ahead, then push again.

The ARMED comments in `ci.yml` get dated, comment-only corrections once their job has run green. The history of the
ARMED reading is kept, not deleted.

**What the first run measures for `s8b`** *(added 2026-09-24, final consistency pass)*. `s8b` gives the `gates` and
no-egress checkouts `fetch-depth: 0` for V2's pinned p9 reads, and leaves the measurement here: *"Pushing is `s8j`'s,
so whether the pinned `rust:` image carries `git` is first measured there"*. Its helper passes `-c safe.directory=*`
so that the container job can read a checkout its user does not own. The first run answers both: the `p9_pin::` tests
pass in both jobs, or the helper's refusal names the missing binary or the ownership refusal. Either failure is a
finding for the loop above, never a skipped job, and Checkpoint 8j (7)'s every-job-green read covers it.

**The frozen commit.** The last green run is taken over the evidence commit, after §8j.6's pin move and the probe
session (the stage close's step 4). Its id and head sha go into `s8j-evidence.txt` as `ci …`. The `fix-item` and
`story-item` lines carry the same sha as their frozen sha. The commit that records them changes no code path, and
Checkpoint 8j asserts that. *Amended 2026-09-24 (D21-47):* the fixture proofs' sha must carry the same code as the
frozen sha, and every paid run's launch record must name a HEAD carrying it. A CI repair that moves a code path
therefore re-opens §8j.6 step 1b and the paid steps. Checkpoint 8j (6) and (7) and every pre-spend check refuse
otherwise.

### 8j.8 `s8`'s instrument amendments — the replacement text

This stage owns the plan's list of `s8` amendments, and this section is the deliverable. It also takes one
amendment the plan does not list, which `s8h` §8h.7 hands on: every `redkiln run` in `s8` goes through
`tools/live-run.sh run` (A1, A7). By D21-46, every gate answer the operator gives during `s8` also goes through
the launcher, as `tools/live-run.sh answer` (§8j.2, §8j.3). Once that lands, `s8` runs from a plain terminal, because
the launcher refuses a Claude-spawned shell. At this stage's execution each replacement is applied to
`runbook/phase-1/s8-exit-criterion.md` with a dated comment, and Checkpoint 8j proves it landed. This file does not edit `s8` while it is being authored.

**Bindings.** `verify_completed`'s names are bound here, from `s8g`: *"The four names, the `item` key and the
`adapter`, `origin` and `stage` members, are **fixed here** so `s8j`'s probe binds to them rather than to a guess"*
(re-quoted 2026-09-24, final consistency pass). The `item` key is read
from `t2_event.item`, where `s8e` §8e.10 carries it beside the payload (D21-46 R3), and `adapter` from the payload. The catalog tests over `.redkiln/`
that `s8b` names were unknown at the draft, which carried a `BIND:` slot for them. *Bound 2026-09-24 (D21-47;
cross-file #14):* Checkpoint 8b section (5)'s `K_TESTS` fixes twenty-six names, and A5.1 copies that line verbatim
from `s8b`'s file as landed. Checkpoint 8j (8) asserts that `s8` carries no `BIND:` anywhere, that the copy is
verbatim, and that every test name `s8` selects exists in `cargo test --workspace -- --list`.

**A0 — the header row.** *Made at insertion 2026-09-24 by the lead (D21-40), not at this stage:* `s8`'s header row
reads `` | `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — | `` with a dated note, and `INDEX.md`'s Step 8 row
(renumbered 19 → 30 when the eleven sub-stages took rows 19–29) opens its entry-gate cell with `Checkpoint 8j`, with a
dated note. The draft replaced both here; the chain would not have resolved between insertion and this stage, so they
moved with the insertion. A0 is now an assertion only: Checkpoint 8j (8) reads both.

**A1 — criteria (1) and (1b): the whole fenced block becomes four blocks.** *Rewritten 2026-09-24 (D21-47; review
findings 1, 12, 13, 14, 28; cross-file #9, #15).* The draft ran the fix-lane and the story-lane runs in one block, with
one go-ahead for both, and its `lane_check` gate ran **after** the paid dispatch. D21-47: *"Every real run gets its
own block and its own go-ahead. In s8, the fix-lane and story-lane runs are separate blocks. Each launcher line is
preceded by mechanical pre-spend checks: the frozen sha is the CI-green sha, HEAD descends from it with the lane paths
unchanged, the fixture-proofs evidence line exists, and RT-57 is `accepted`."* So `s8`'s (1) block is replaced by four
blocks, in this order in the file: **(1) launch**, **(1) check**, **(1b) launch**, **(1b) check**. A launch block
spends; a check block launches nothing and may be re-run freely. Together they cover:

- build first;
- each run line through `tools/live-run.sh run` (`s8h` §8h.7; D21-42 17a), in a block of its own, after the
  pre-spend checks below, under a per-run go-ahead that names the item;
- the gate answer that a gated lane's FAIL message asks for, through `tools/live-run.sh answer` (D21-46), followed by
  a new launch with a new go-ahead;
- the run's trailers scoped to its own fork, which descends from the frozen sha;
- every lane stage asserted as **set and order** equality (D21-42 7), ending in D21-43 N9's terminal `closed` mark;
- the toolchain identity read from `verify_completed`, over the verify stage's own pass only (D21-42 10b; review
  finding 28, after `s8g`'s review finding 4);
- every `dispatch_completed` row of the item carrying a non-empty `observed_model` (`s8h`'s member), so a real model
  answered and no fixture child did;
- the landed content change, and a run ref not yet merged into the checkout (review finding 13);
- (1b)'s digest read from `artifact.write` against the landed blob, with `git cat-file -e` on the run ref (ADR-0116
  decision 3; D21-42 5b). `git ls-files --error-unmatch` is dropped, because it is the tree-membership assertion
  ADR-0116 decision 3 rejects.

**The pre-spend checks** are one function, `prespend`, carried verbatim in every `s8` block that launches a real run:
(1) launch, (1b) launch and (3)'s two launch blocks (A3). Before the launcher line it refuses, naming the defect:

- a go-ahead that does not name this item. The go-ahead is `S8_GO=<item>`, given as the block's command prefix
  (`S8_GO=rk-fix-… bash <block file>`), never exported, so it covers one launch;
- an evidence file with a CR byte, or no frozen sha for the item;
- a frozen sha that is not the recorded CI-green sha;
- a HEAD that does not descend from the frozen sha, or any change since it under `crates`, the Cargo files,
  `clippy.toml`, `deny.toml`, `tests`, `tools`, `.github`, `.gitattributes` or `.redkiln` (so `.redkiln/items` and
  `.redkiln/intake` too; review finding 12);
- no `fixture-proofs` line over the frozen code (§8j.6 step 1b; D21-40 decision 2);
- RT-57's record (`s8h`'s Record B, whose id the evidence file's `record-rt57` line names) not `accepted`;
- a `harness_binary` in the shipped `.redkiln/config.toml` (review finding 14): a real run reaches the real CLI;
- a run ref already merged into HEAD (review finding 13);
- a recorded CI run that `gh run view` does not report green on the frozen sha.

⚠️ **The plan says "scoped to commits made by this invocation"**, and that scope is too narrow. §8j.3's hold alone
makes the fix lane at least two invocations, and a set-equality over one invocation's commits would fail a correct
lane. The scope is therefore the run's own fork, `merge-base(HEAD, rk-run/<item>)`. The fork must be ahead of that
point, and must descend from the frozen sha. The fork also rules out the stale-ref shape `s7h` found, *"0 commits
ahead"*.

**The order of acts in `s8` for the fix item.** (1) launch → the run stops at §8j.3's hold and exits 0 → the hold is
answered through `tools/live-run.sh answer` → (3)'s kill block launches the continuation and kills it after its first
committed step (A3) → (3)'s resume block re-invokes it → any further gate is answered and (1) launch is run again,
each with its own go-ahead → (1) check. The story item has no hold: (1b) launch → answers and relaunches as its gates
require → (1b) check.

**The names bound from `s8g` and `s8e`.** `verify_completed`'s `item` is the interim key `s8e` §8e.10 carries in the
`t2_event.item` column (D21-46 R3), and its `adapter` a payload member (`s8g` §8g.4). The member that tells the verify
stage's pass from an implement-stage test call is the one `s8g` adds for its review finding 4. `s8g` §8g.4 fixes it:
the proof *"asserts `adapter` on every one whose `origin` is `pass`"*, which is what `VC_ORIGIN_KEY`/`VC_PASS` carry
below (settled 2026-09-24, final consistency pass). `artifact.write`
is item-scoped under `s8e` §8e.10's enumeration, so its item is read from the same column, not from the payload
(cross-file #15); a dated correction here if that record lists it otherwise.

The launch blocks share this function, verbatim:

```bash
# D21-47's pre-spend checks (s8j §8j.8 A1), carried verbatim by every s8 block that launches a real run. It prints the
# defect and returns 1, or prints nothing. The go-ahead is per run: S8_GO=<item> as THIS block's command prefix.
prespend() {   # $1 = the item this block is about to launch
  pev=runbook/phase-1/s8j-evidence.txt
  CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
  [ "${S8_GO:-}" = "$1" ] || { echo "no go-ahead for $1: run this block as S8_GO=$1 bash <block>, one launch per go-ahead (D21-47)"; return 1; }
  test -s "$pev" || { echo "$pev is absent or empty"; return 1; }
  if od -c "$pev" | grep -qF '\r'; then echo "$pev carries a CR byte"; return 1; fi
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$pev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $pev"; return 1; }
  ci=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  read -r _ ci_run ci_sha _ <<< "$ci"
  [ -n "$ci_sha" ] && [ "$frozen" = "$ci_sha" ] || { echo "$1's frozen sha $frozen is not the CI-green sha '$ci_sha'"; return 1; }
  git merge-base --is-ancestor "$frozen" HEAD || { echo "HEAD does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" HEAD -- $CODE_PATHS || { echo "code, registry, items or intake changed since the frozen sha: re-run CI and re-record (s8j §8j.7)"; return 1; }
  fp=$(grep -E '^fixture-proofs [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  fp_sha=$(printf '%s\n' "$fp" | awk '{print $2}')
  [ -n "$fp_sha" ] && git diff --quiet "$fp_sha" "$frozen" -- $CODE_PATHS || { echo "no fixture-proofs line over the frozen code: the spend gate (D21-40 decision 2; s8j §8j.6 step 1b)"; return 1; }
  rt=$(sed -nE 's/^record-rt57 ([0-9]{4})$/\1/p' "$pev" | tail -1)
  rtf=$(ls references/adr/"$rt"-*.md 2>/dev/null)
  [ -n "$rt" ] && [ -f "$rtf" ] && grep -qxF 'status: accepted' "$rtf" || { echo "RT-57's record (ADR-${rt:-none}, s8h's Record B) is not accepted (D21-42 19c)"; return 1; }
  if grep -qE '^[[:space:]]*harness_binary[[:space:]]*=' .redkiln/config.toml; then echo ".redkiln/config.toml names a harness_binary: a real run must reach the real CLI, never a fixture child"; return 1; fi
  if git merge-base --is-ancestor "rk-run/$1" HEAD 2>/dev/null; then echo "rk-run/$1 is already merged into HEAD: the run ref does not merge before Checkpoint 8"; return 1; fi
  j=$(gh run view "$ci_run" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha 2>&1) || { echo "gh run view $ci_run failed: $j"; return 1; }
  ok=$(printf '%s\n' "$j" | python -c 'import json,sys; d=json.load(sys.stdin); print("ok" if d.get("conclusion")=="success" and d.get("headSha")==sys.argv[1] else "no")' "$frozen")
  [ "$ok" = ok ] || { echo "CI run $ci_run is not green on $frozen"; return 1; }
}
```

**(1) launch.** The function above is pasted where the comment says so.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1) LAUNCH. ONE real run of the fix item, its own block and its own go-ahead
# (D21-47). Run from a plain terminal as `S8_GO=$FIX_ITEM bash <this block>`: s8h's launcher refuses a
# Claude-spawned shell. A gate-open exits 0 (C-01 D-01; AC-403); (1)'s check block tells a gated lane from a landed one.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1) NOT LAUNCHED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — criterion (1) NOT LAUNCHED (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent — s8h's launcher is the only entry to a real run (D21-42 17a)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item s8j §8j.5 reserved — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
test -f ".redkiln/items/$FIX_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIX_ITEM.toml does not exist"; exit 1; }
# <prespend() from s8j §8j.8 A1, verbatim>
why=$(prespend "$FIX_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1) NOT LAUNCHED: $why"; exit 1; }
tools/live-run.sh run "$FIX_ITEM" --lane fix || { echo "FAIL: the fix-lane run exited non-zero: stop and report, do not spend again (D21-40 decision 2)"; exit 1; }
echo "(1) launched once. Read explain; answer an open gate through tools/live-run.sh answer; then (3)'s blocks, or (1) launch again with a new go-ahead, and last (1)'s check block"
```

**(1) check.** It launches nothing.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1)'s CHECK — AC-473. It launches nothing and may be re-run freely.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1) NOT CHECKED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build (Halt 8)"; exit 1; }
# verify_completed's names (s8g §8g.4; D21-42 10b): the interim `item` key in the t2_event.item column (s8e §8e.10;
# D21-46 R3), the `adapter` payload member, and the member that marks the verify stage's own pass (s8g's review 4).
VC_ITEM_KEY=item
VC_ADAPTER_KEY=adapter
VC_ORIGIN_KEY=origin
VC_PASS=pass
grep -qxF 'name = "redkiln-rs"' .redkiln/config.toml || { echo "FAIL: [project] name is not redkiln-rs — the Story: value's <project> would not be what this block compares"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — the frozen sha this run must descend from is unrecorded"; exit 1; }
CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
# The expected stage list, derived from the pack and never typed (D21-42 7): stage_order minus forbidden.
order='import sys,tomllib; p=tomllib.load(open(sys.argv[1],"rb"))["policy"]; print(" ".join(s for s in p["stage_order"] if p["stages"][s]!="forbidden"))'
# The lane's stage tokens: git's own trailer parser, oldest first, the LAST token of each Story: value; every value
# must be exactly `<project>/<item> <stage>` (the stage-bearing form of D21-41 (a)), and the last is N9's `closed`.
lane_check() {   # $1 item  $2 pack file — prints nothing on success, the defect otherwise
  exp=$(python -c "$order" "$2"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$exp" ] || { echo "the stage order could not be read from $2 (rc=$rc)"; return 1; }
  exp="$exp closed"   # D21-43 N9: a complete lane ends in the terminal 'Story: <project>/<item> closed' mark (s8e)
  git rev-parse --verify --quiet "refs/heads/rk-run/$1" >/dev/null || { echo "no rk-run/$1 ref — no run"; return 1; }
  if git merge-base --is-ancestor "rk-run/$1" HEAD; then echo "rk-run/$1 is merged into HEAD — it does not merge before Checkpoint 8"; return 1; fi
  base=$(git merge-base HEAD "rk-run/$1") || { echo "rk-run/$1 shares no history with HEAD"; return 1; }
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$ev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $ev"; return 1; }
  git merge-base --is-ancestor "$frozen" "$base" || { echo "rk-run/$1 forked from $base, which does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" "$base" -- $CODE_PATHS || { echo "code, registry, items or intake changed between the CI-green frozen sha and the run's fork — re-run CI and re-record (s8j §8j.7)"; return 1; }
  ahead=$(git rev-list --count "$base..rk-run/$1") || { echo "rev-list failed"; return 1; }
  [ "$ahead" -ge 1 ] || { echo "rk-run/$1 is $ahead commits ahead of its fork — a stale ref is not a run"; return 1; }
  lg=$(git log --reverse "$base..rk-run/$1" --format='%(trailers:key=Story,valueonly=true)') || { echo "git log over the run failed"; return 1; }
  bad=$(printf '%s\n' "$lg" | awk -v want="redkiln-rs/$1" 'NF && ($1!=want || NF!=2)')
  [ -z "$bad" ] || { echo "Story: values not of the form redkiln-rs/$1 <stage>: $bad"; return 1; }
  got=$(printf '%s\n' "$lg" | awk 'NF {printf "%s%s", sep, $NF; sep=" "}')
  [ "$got" = "$exp" ] || { echo "stages: expected '$exp', the run's trailers say '$got' — if explain shows an open gate, answer it through the launcher's answer mode (tools/live-run.sh answer; D21-46) and launch again with a new go-ahead (s8j §8j.3's hold is the first); a gated lane is not failed, and it is not a pass"; return 1; }
}
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item s8j §8j.5 reserved"; exit 1; }
test -f ".redkiln/items/$FIX_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIX_ITEM.toml does not exist"; exit 1; }
out=$(./target/debug/redkiln explain "$FIX_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$FIX_ITEM" .redkiln/packs/bug-standard.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1): $why"; exit 1; }
base=$(git merge-base HEAD "rk-run/$FIX_ITEM") || { echo "FAIL: no merge base"; exit 1; }
git diff --quiet "$base" "rk-run/$FIX_ITEM" -- crates; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL (1): the run landed no content change under crates/ (rc=$rc) — trailers without a landed commit (D21-41 (b))"; exit 1; }
vc='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? AND "+sys.argv[3]+"=?",("verify_completed",sys.argv[2]))]; a=sorted({str(x.get(sys.argv[4])) for x in r if x.get(sys.argv[5])==sys.argv[6]}); print(" ".join(a))'
ids=$(python -c "$vc" "$store" "$FIX_ITEM" "$VC_ITEM_KEY" "$VC_ADAPTER_KEY" "$VC_ORIGIN_KEY" "$VC_PASS"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the verify_completed probe did not run (rc=$rc)"; exit 1; }
[ "$ids" = "rk-adapter-toolchain-rust" ] || { echo "FAIL (1): the verify stage's own passes name '$ids', not exactly rk-adapter-toolchain-rust (AC-473; D21-42 10b) — implement-stage calls are not the verify stage"; exit 1; }
om='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? AND item=?",("dispatch_completed",sys.argv[2]))]; print(len(r), sum(1 for x in r if str(x.get("observed_model") or "").strip()))'
cnt=$(python -c "$om" "$store" "$FIX_ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$cnt" ] || { echo "FAIL: the observed_model probe did not run (rc=$rc)"; exit 1; }
read -r n_dc n_om <<< "$cnt"
[ "$n_dc" -ge 1 ] && [ "$n_om" = "$n_dc" ] || { echo "FAIL (1): $n_om of $n_dc dispatch_completed rows for $FIX_ITEM carry an observed_model — a real run's every dispatch names the model that answered (s8h)"; exit 1; }
printf '%s\n' "$out"; git log "$base..rk-run/$FIX_ITEM" --format='%h %s%n%(trailers)'   # → READ: no stage performed by the old redkiln; the landed commit is the fix. The run id is the evidence.
```

**(1b) launch.** The function above is pasted where the comment says so.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1b) LAUNCH. ONE real run of the story item, its own block and its own go-ahead
# (D21-47): `S8_GO=$STORY_ITEM bash <this block>`, from a plain terminal.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1b) NOT LAUNCHED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — criterion (1b) NOT LAUNCHED (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent — s8h's launcher is the only entry to a real run (D21-42 17a)"; exit 1; }
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM (rk-story-harden-spike-pack)"; exit 1; }
test -f ".redkiln/items/$STORY_ITEM.toml" || { echo "FAIL: .redkiln/items/$STORY_ITEM.toml does not exist"; exit 1; }
# <prespend() from s8j §8j.8 A1, verbatim>
why=$(prespend "$STORY_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1b) NOT LAUNCHED: $why"; exit 1; }
tools/live-run.sh run "$STORY_ITEM" --lane story || { echo "FAIL: the story-lane run exited non-zero: stop and report, do not spend again (D21-40 decision 2)"; exit 1; }
echo "(1b) launched once. Answer an open gate through tools/live-run.sh answer and launch again with a new go-ahead; last, (1b)'s check block"
```

**(1b) check.** It launches nothing. `lane_check`, `order`, `store`, `ev`, `CODE_PATHS` and `tdir` are defined as in (1)'s check
block, whose lines from `grep -qxF 'name = "redkiln-rs"'` through the closing `}` of `lane_check` are pasted where the
comment says so.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1b)'s CHECK — AC-521. It launches nothing and may be re-run freely.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1b) NOT CHECKED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build (Halt 8)"; exit 1; }
# <(1)'s check block, from its `grep -qxF 'name = "redkiln-rs"'` line through lane_check's closing brace, verbatim>
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM (rk-story-harden-spike-pack)"; exit 1; }
test -f ".redkiln/items/$STORY_ITEM.toml" || { echo "FAIL: .redkiln/items/$STORY_ITEM.toml does not exist"; exit 1; }
out=$(./target/debug/redkiln explain "$STORY_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$STORY_ITEM" .redkiln/packs/story.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1b): $why"; exit 1; }
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the Spec's path under the item's managed artifact directory (D21-42 5b)"; exit 1; }
git cat-file -e "rk-run/$STORY_ITEM:$AUTHORED_STORY" || { echo "FAIL (1b): $AUTHORED_STORY is not on rk-run/$STORY_ITEM"; exit 1; }
git cat-file blob "rk-run/$STORY_ITEM:$AUTHORED_STORY" > "$tdir/spec.blob" || { echo "FAIL: could not read the landed Spec blob"; exit 1; }
landed=$(sha256sum < "$tdir/spec.blob") || { echo "FAIL: sha256sum failed"; exit 1; }
landed=${landed%% *}
# artifact.write is item-scoped (s8e §8e.10): its item is the t2_event.item column, never a payload member (cross-file #15).
aw='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? AND item=? ORDER BY id",("artifact.write",sys.argv[2]))]; d=[x.get("digest") for x in r if x.get("kind")=="Spec"]; print(d[-1] if d else "")'
dg=$(python -c "$aw" "$store" "$STORY_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the artifact.write probe did not run (rc=$rc)"; exit 1; }
test -n "$dg" || { echo "FAIL (1b): no artifact.write row with kind Spec for $STORY_ITEM — the authored story is not reachable from the event log (AC-521)"; exit 1; }
[ "$dg" = "$landed" ] || { echo "FAIL (1b): the last Spec artifact.write digest $dg is not the landed blob's $landed (ADR-0116 decisions 3 and 6)"; exit 1; }
printf '%s\n' "$out"   # → READ against AC-521: every story-lane stage (asserted above), the Spec authored by the run. The run id and the story id are the evidence.
```

**A2 — (2)'s operator half: build first, back up and restore the store around `doctor --rebuild`, and compare the
rebuilt tier-1 counts.** Replace the three lines from `# The operator path must still work (AC-427), but its STDOUT is
not the comparison.` through `./target/debug/redkiln doctor --rebuild || { … }` with the block below. Either repair
would have served: back up and restore, or run (7) before (2). Back up and restore is taken because it makes the block
order-independent. The restore uses SQLite's own backup API, because a bare `cp` of a WAL-mode database copies an
inconsistent pair. *Amended 2026-09-24 (D21-47; review findings 15 and 16):* the rebuilt store's three tier-1 counts
are read **before** the restore and must equal the pre-rebuild ones, which is AC-184's differential over the real
store (ADR-0105 clause 9); a restore-only block measured nothing about the rebuild. The replacement's first line is
the anchor line itself, byte for byte, so A8's *"immediately above the line"* (re-quoted 2026-09-24, final consistency pass) holds whichever of A2 and A8 is applied
first.

```bash
# The operator path must still work (AC-427), but its STDOUT is not the comparison.
# AMENDED at s8j (s8j §8j.8 A2): built first; the store `doctor --rebuild` replaces is backed up and restored, so (7)
# reads the store the run wrote; and the REBUILT store's tier-1 counts equal the pre-rebuild ones (AC-184; ADR-0105
# clause 9), read before the restore. The line above is A8's anchor and stays byte-identical.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (2)'s operator half NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2)'s operator half NOT RUN (Halt 8)"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
STORE_BAK="$store.s8-bak"
bk='import sqlite3,sys; s=sqlite3.connect(sys.argv[1]); d=sqlite3.connect(sys.argv[2]); s.backup(d); d.close(); s.close()'
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate_transition","t1_slice_seal","t2_event")))'   # t1_gate_transition: s8e's 0002 migration (D21-46 R2)
before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$before" ] || { echo "FAIL: could not count the store's rows (rc=$rc)"; exit 1; }
python -c "$bk" "$store" "$STORE_BAK" || { echo "FAIL: the store could not be backed up — doctor --rebuild NOT RUN"; exit 1; }
./target/debug/redkiln doctor --rebuild; drc=$?
rebuilt=$(python -c "$rows" "$store"); rrc=$?
python -c "$bk" "$STORE_BAK" "$store" || { echo "FAIL: the store could NOT be restored from $STORE_BAK — restore it by hand before anything reads it"; exit 1; }
after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$after" = "$before" ] || { echo "FAIL: the restored store's row counts ($after) are not the pre-rebuild ones ($before)"; exit 1; }
rm -f "$STORE_BAK"
[ "$drc" -eq 0 ] || { echo "FAIL: doctor --rebuild exited $drc (AC-427)"; exit 1; }
[ "$rrc" -eq 0 ] && [ -n "$rebuilt" ] || { echo "FAIL: the rebuilt store's rows could not be counted (rc=$rrc)"; exit 1; }
[ "${rebuilt% *}" = "${before% *}" ] || { echo "FAIL (2): the rebuilt tier-1 counts ($rebuilt) are not the pre-rebuild ones ($before) — the projection does not rebuild from the refs (AC-184; ADR-0105 clause 9)"; exit 1; }
```

**A3 — (3)'s operational arm leaves (3)'s block, bound to the fix item and a real kill.** *Rewritten 2026-09-24
(D21-47; review finding 9).* The draft only inserted a build above the arm's `test -x`. The arm then read `ITEM` from
whatever the shell held, and over a closed item its re-invocation, its two `explain` captures and its READ diff all
pass having killed nothing. The arm holds two real runs, the killed one and the re-invocation, and D21-47 gives each
its own block and go-ahead. So:

1. In `s8`'s (3) block, the lines from the one opening `# --- the operational arm, in order.` through the block's
   last line before its closing fence are deleted. The asserted half, the kernel test, stays.
2. The two blocks below are inserted after that block's closing fence: **(3) kill** and **(3) resume**, each opening
   with A1's `prespend` function, verbatim.

The item is the fix item (`ITEM="$FIX_ITEM"`), and the arm runs on the lane's **continuation**, after §8j.3's hold is
answered, so the killed process has stages left to be killed inside (A1's order of acts). The kill block refuses a
lane whose last `Story:` token is already `closed`, starts the run through the launcher in the background, polls
(bounded, never a fixed sleep) until the run ref moves past its pre-run tip, takes the PRE-KILL capture while the
process lives, and kills the launcher's tree with `taskkill //T //F` (Git Bash's spelling of `/T /F`). A run that
exits first, or commits nothing within the bound, is reported and not retried: D21-40 decision 2.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A3): (3)'s OPERATIONAL ARM, part 1 — ONE real run of the fix item, killed mid-graph. Its
# own block and go-ahead (D21-47): `S8_GO=$FIX_ITEM bash <this block>`, from a plain terminal, after (1)'s hold is
# answered. The PRE-KILL capture is taken HERE, after the run has committed a step and before the kill.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — (3)'s operational arm NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s operational arm NOT RUN (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent (D21-42 17a)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM — (3) is bound to the fix item"; exit 1; }
ITEM="$FIX_ITEM"
# <prespend() from s8j §8j.8 A1, verbatim>
why=$(prespend "$ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (3) NOT LAUNCHED: $why"; exit 1; }
git rev-parse --verify --quiet "refs/heads/rk-run/$ITEM" >/dev/null || { echo "FAIL: no rk-run/$ITEM — run (1)'s launch block and answer the hold first; (3) kills the continuation"; exit 1; }
lg=$(git log -1 --format='%(trailers:key=Story,valueonly=true)' "rk-run/$ITEM") || { echo "FAIL: git log over rk-run/$ITEM failed"; exit 1; }
last=$(printf '%s\n' "$lg" | awk 'NF {t=$NF} END {print t}')
[ -n "$last" ] && [ "$last" != closed ] || { echo "FAIL: rk-run/$ITEM's last stage token is '$last' — a kill over a closed or unread lane is a no-op pass"; exit 1; }
pre_tip=$(git rev-parse "rk-run/$ITEM") || { echo "FAIL: cannot read rk-run/$ITEM"; exit 1; }
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
rm -f "$tdir/s8-3-pre.txt"
tools/live-run.sh run "$ITEM" --lane fix > "$tdir/s8-3-killed.out" 2>&1 &
bg=$!
# A BOUNDED POLL: until the run ref moves past pre_tip (the process committed a step), the process exits, or 900 s pass.
n=0
while [ "$(git rev-parse "rk-run/$ITEM")" = "$pre_tip" ] && kill -0 "$bg" 2>/dev/null && [ "$n" -lt 900 ]; do sleep 1; n=$((n + 1)); done
kill -0 "$bg" 2>/dev/null || { echo "FAIL: the run exited after ${n}s, before it could be killed — no kill mid-graph happened; read $tdir/s8-3-killed.out, stop and report (D21-40 decision 2)"; exit 1; }
wp=$(cat "/proc/$bg/winpid") || { echo "FAIL: no Windows PID for the launcher (pid $bg) — stop it by hand"; exit 1; }
[ "$(git rev-parse "rk-run/$ITEM")" != "$pre_tip" ] || { taskkill //T //F //PID "$wp" >/dev/null 2>&1; echo "FAIL: no step committed within ${n}s (the bound fired) — the run was killed before any step; stop and report"; exit 1; }
pre=$(./target/debug/redkiln explain "$ITEM") || { taskkill //T //F //PID "$wp" >/dev/null 2>&1; echo "FAIL: pre-kill explain exited non-zero (the run was killed)"; exit 1; }
taskkill //T //F //PID "$wp" || { echo "FAIL: taskkill on the launcher's tree ($wp) failed — the run may still be live; stop it by hand"; exit 1; }
wait "$bg"
test -n "$pre" || { echo "FAIL: empty pre-kill capture — two empty outputs compare equal"; exit 1; }
printf '%s\n' "$pre" > "$tdir/s8-3-pre.txt" || { echo "FAIL: cannot write the pre-kill capture"; exit 1; }
echo "(3) killed mid-graph after ${n}s at $(git rev-parse --short "rk-run/$ITEM"); run (3)'s resume block with its own go-ahead"
```

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A3, A7): (3)'s OPERATIONAL ARM, part 2 — re-invoke the SAME idempotent entry through the
# launcher, its own go-ahead: `S8_GO=$FIX_ITEM bash <this block>`. There is no --resume flag and no resume verb
# (ADR-0107; AC-013/AC-186/AC-264/AC-400). That --resume is refused is itself AC-400's check, at s7d's checkpoint.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — (3)'s resume NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s resume NOT RUN (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent (D21-42 17a)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM — (3) is bound to the fix item"; exit 1; }
ITEM="$FIX_ITEM"
tdir=target/s8-cp
test -s "$tdir/s8-3-pre.txt" || { echo "FAIL: no pre-kill capture at $tdir/s8-3-pre.txt — run (3)'s kill block first"; exit 1; }
# <prespend() from s8j §8j.8 A1, verbatim>
why=$(prespend "$ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (3) NOT RE-INVOKED: $why"; exit 1; }
tools/live-run.sh run "$ITEM" --lane fix || { echo "FAIL: the re-invocation exited non-zero"; exit 1; }   # AMENDED at s8j (s8j §8j.8 A7): through s8h's launcher, the only entry to a real run (D21-42 17a). A re-invocation that reaches a gate exits 0 (C-01 D-01), so this guard passes on a gated run too; the digest diff below is the assertion.
post=$(./target/debug/redkiln explain "$ITEM") || { echo "FAIL: post-resume explain exited non-zero"; exit 1; }
test -n "$post" || { echo "FAIL: empty post-resume capture"; exit 1; }
diff "$tdir/s8-3-pre.txt" <(printf '%s\n' "$post")   # → READ this. Whole-output equality is NOT asserted: the post-resume output carries the resume's own appended events. Each committed step's output_digest (ADR-0128 decision 4) must be unchanged; AC-265 (the gate answer surviving the kill) is exercised here.
```

**A4 — (7), the whole fenced block.** D21-42 12a: *"An honest rendering passes, with two assertions: owned readings
== `dispatch_completed` for the item, and no rendering over n = 0. This replaces s8's unreadable AC-394 READ line."*
`dispatch_completed`'s `item` is N5's interim key, carried beside the payload in `t2_event.item` (`s8e` §8e.10; D21-46
R3). `rate_limit_reading.item` stays in its payload, where ADR-0163 put it. The probe reads each where it lives.

```bash
# AMENDED at s8j (s8j §8j.8 A4): built first; AC-394 asserted as owned readings == dispatch_completed (D21-42 12a).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (7) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (7) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item whose cost rows are being read"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
own='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); rd=sum(1 for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",("rate_limit_reading",)) if json.loads(p).get("item")==sys.argv[2]); dc=c.execute("SELECT COUNT(*) FROM t2_event WHERE kind=? AND item=?",("dispatch_completed",sys.argv[2])).fetchone()[0]; print(rd, dc)'   # readings: payload item (ADR-0163); dispatches: the interim key's column (s8e; D21-46 R3)
cnt=$(python -c "$own" "$store" "$ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$cnt" ] || { echo "FAIL: the ownership probe did not run (rc=$rc)"; exit 1; }
read -r owned dc <<< "$cnt"
[ "$dc" -ge 1 ] || { echo "FAIL (7): no dispatch_completed row names $ITEM — a reading over n = 0 is not a reading (D21-42 12a)"; exit 1; }
[ "$owned" = "$dc" ] || { echo "FAIL (7): $ITEM owns $owned readings and $dc dispatch_completed rows — AC-394 is one reading per dispatch (ADR-0163 decision 2)"; exit 1; }
out=$(./target/debug/redkiln cost "$ITEM") || { echo "FAIL: redkiln cost exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: redkiln cost printed nothing"; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: a headroom report carrying none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
if printf '%s\n' "$out" | grep -qE ' nothing to attribute |(^|[^0-9])n=0([^0-9]|$)'; then echo "FAIL (7): a rendering over n = 0 for an item that owns $dc dispatches (D21-42 12a)"; printf '%s\n' "$out"; exit 1; fi
printf '%s\n' "$out"   # → READ: no per-run headroom figure (AC-373).
```

**A5 — Checkpoint 8's mechanical block.** Five edits. Each is anchored on a line that exists today, and each anchor
is asserted to exist before it is replaced (golden constraint 7).

*A5.1 — the catalog checks run over `.redkiln/` (D21-42 13a, 13b).* Replace the three lines that open
`out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?`. Today's line walks `spikes/p9/catalog`, because
`rigor.rs` holds `const SHIPPED_CATALOG: &str = "spikes/p9/catalog";`. *Rewritten 2026-09-24 (D21-47; cross-file
#14; review finding 29).* The draft carried a `BIND:` slot because `s8b`'s names were unknown. They are now fixed:
Checkpoint 8b section (5)'s `K_TESTS` names twenty-six, asserted by `[ "$nk" -eq 26 ]`. A green selection says nothing
about **what** each instrument walked, so the walked-count readings come too. The replacement is copied verbatim from
`s8b`'s file as landed, never retyped here, so the two cannot drift:

- Checkpoint 8b section (5): every code line from the one opening `K_TESTS=` through its
  `test result: ok\. 26 passed; 0 failed` guard;
- Checkpoint 8b section (6): the three runs and the guards that read what each walked, printed from inside the test
  binary. These are `lattice::p6_lattice_over_the_shipped_pack` with its `^lattice-shipped: packs=` reading,
  `rigor::rigor_monotonicity` with its `^rigor_monotonicity: catalog=\.redkiln/catalog` guard, and
  `quality_block::ac527_empty_set_terminal_condition` with its `^ac527: catalog=\.redkiln/catalog` guard.

Checkpoint 8j (8) reads `s8b`'s file at check time and asserts that its `K_TESTS=` line and the three walked-count
lines are present verbatim on `s8`'s code lines, and that every name `s8` selects exists in `cargo test --workspace --
--list`.

*A5.2 — the quoted-predicate guard is `s8b`'s form (D21-41 (h): "widen Checkpoint 8's guard").* `s8b` owns the form:
*"`s8j` widens Checkpoint 8's quality-shorthand block to it. This file does not edit `s8`."* Checkpoint 8b's section
(4) is that form, and it runs there first. This file does not restate it.

At application, the four lines of `s8`'s block that begin with the line opening
`raw=$(grep -rnE 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *='` and end with
its `syn=$(printf 'a.toml:1:expr = …` control are replaced. *Rewritten 2026-09-24 (D21-47; review finding 2).* The
draft described a copied `grep -rnE … crates/ .redkiln/` line that `s8b`'s form does not have. `s8b` walks `crates/`
whole in one call and `.redkiln/` with `--exclude-dir=worktrees` in another, through its `two_walk` function, because
`--exclude-dir` matches a base name at any depth and `crates/` holds fixture directories named `worktrees`. The
replacement is **every code line of Checkpoint 8b section (4), copied verbatim** from `s8b`'s file as landed, from
the `EXCL=` line through the `syn2` guard. That includes:

- the `EXCL` list, with each path's stated reason in its comment;
- the `find` guard against a second directory named `worktrees` inside `.redkiln/`;
- `D2=` and `CAL=`, `two_walk()`, `d2_walk()` and `cal_walk()`;
- the planted control, with its deliberately **unexcluded** `seen=$(grep -rlF …)` walk, which proves the plant is
  findable;
- both synthetic controls.

Nothing in the copy changes: `two_walk` already skips `worktrees/` on its `.redkiln/` call, so A5.5 does not touch
it. Any further must-refuse fixture that `s8b` adds is already in its `EXCL`. Checkpoint 8j (8) reads `s8b`'s file at
check time and asserts that its `EXCL=`, `D2=` and `two_walk() {` lines are present verbatim on `s8`'s code lines,
that the unquoted-only pattern is gone, and that every recursive walk over `.redkiln/` on `s8`'s code lines carries
`--exclude-dir=worktrees` except `s8b`'s plant-control line, which is exempted by its exact bytes and nothing else.

*A5.3 — the doc screen reads tracked files only.* Replace the line that opens
`grep -rn 'four questions on twenty items' --include=*.md . ; rc=$?` with the lines below. D21-40 measured this line
reading a stale `runbook/` copy under `.git/redkiln/worktrees/`. D21-44 moves run worktrees to `.redkiln/worktrees/`,
where the same line would read the same stale copy again. *Rewritten 2026-09-24 (D21-47; review finding 8).* The
draft's `--exclude-dir=.git --exclude-dir=worktrees` still walked `target/`, which holds stale `references/` and
`runbook/` copies (Checkpoint 8j (4)'s `git archive` extraction, `s8a`'s scratch trees). `git grep` reads the index's
tracked files and nothing else: no `.git/`, no `target/`, no run worktree, no untracked copy. A planted untracked copy
under `target/` is the control that it stays so, and the design document's own row is the control that it reads.

```bash
# AMENDED at s8j (s8j §8j.8 A5.3; D21-40; D21-44): tracked files only — git grep never reads .git/, target/, a run
# worktree or any other untracked copy. The plant proves the last; the design row proves it reads.
ctl=target/s8-a53; rm -rf "$ctl"; mkdir -p "$ctl" || { echo "FAIL: cannot create $ctl"; exit 1; }
printf '%s\n' 'four questions on twenty items' > "$ctl/stale-copy.md" || { echo "FAIL: cannot write the untracked plant"; exit 1; }
hits=$(git grep -n 'four questions on twenty items' -- '*.md'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the phrase is gone (rc=$rc) — design §10.5's row may have been deleted"; exit 1; }
printf '%s\n' "$hits" | grep -q '^references/2026-09-08-architecture-design\.md:' || { echo "FAIL: the design row is not among the hits — the screen is not reading the tracked tree"; exit 1; }
if printf '%s\n' "$hits" | grep -qF "$ctl/"; then echo "FAIL: the screen read the untracked plant under target/"; exit 1; fi
rm -rf "$ctl"
printf '%s\n' "$hits"   # → each hit READ: an oracle-construction measurement, never a forecast
```

*A5.4 — two new stanzas, appended after the block's last line* (the `--stay` by-path stanza's guard that opens
`printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails`). The first is
ADR-0164's first rule. The second is D21-42 14a's CI condition.

```bash
# ADR-0164: `* text=auto eol=lf` is .gitattributes' FIRST rule (every -text row below it still wins for its paths).
first=$(grep -vE '^[[:space:]]*(#|$)' .gitattributes | head -1)
[ "$first" = '* text=auto eol=lf' ] || { echo "FAIL: .gitattributes' first rule is '$first', not ADR-0164's row"; exit 1; }
if od -c .gitattributes | grep -qF '\r'; then echo "FAIL: .gitattributes carries a CR byte (ADR-0164 decision 4)"; exit 1; fi
# D21-42 14a: ci.yml green on the pushed branch, every job, both OS legs — read from the recorded run, never re-pushed.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} ' runbook/phase-1/s8j-evidence.txt | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in runbook/phase-1/s8j-evidence.txt"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha $ci_sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln || { echo "FAIL: code changed since the green CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }
```

The bullet list gains two bullets:

- ***`ci.yml` is green on the pushed branch, every job, both OS legs*** (D21-42 14a), *read from the recorded run and
  never re-pushed by this checkpoint.*
- ***`.gitattributes` opens with `* text=auto eol=lf`*** (ADR-0164).

Its count sentence *"Those seventeen are not read, they are run."* becomes *"Those nineteen are not read, they are
run."*.

*A5.5 — no recursive walk over `.redkiln/` enters a run worktree (D21-44).* D21-44 gave the rule to `s8e`, and
D21-46 R1 moved it to `s8d-ii`: *"Every recursive walk over `.redkiln/` as a whole excludes `worktrees/`, each with a
synthetic control, because a run worktree is a full repository copy. This includes Checkpoint 8's `grep -r …
.redkiln/` lines"*. `s8d-ii` §8d-ii.1 hands `s8`'s share to this stage: *"`s8-exit-criterion.md`'s recursive greps
over `crates/ .redkiln/` belong to `s8j`, which owns every `s8` amendment"*. Two walks in the block are affected *(three in the draft; corrected 2026-09-24, D21-47: A5.2's copy is no longer
one of them, because `s8b`'s `two_walk` already skips `worktrees/` on its `.redkiln/` call)*:

- the line that opens `raw=$(grep -rnE 'require.*\b(criticality|`;
- the line that opens `grep -rnE 'append(_route)? *= *true'`;

Each gains `--exclude-dir=worktrees` after `-rnE`. Insert the stanza below immediately above the first of them.
`--exclude-dir` matches a base name, so it also skips the four tracked fixture files that lie under directories named
`worktrees` in `crates/*/tests/fixtures/` [measured at `a6349b6`: none carries any of the three walked patterns]. The
stanza's first guard keeps that true, and its control proves the skip drops a run-worktree copy and keeps a pack.

```bash
# AMENDED at s8j (s8j §8j.8 A5.5; D21-44, whose walk rule is s8d-ii's): the two walks over .redkiln/ below skip every
# directory named worktrees — a run worktree under .redkiln/worktrees/<item> is a full repository copy.
WALKED='require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b|append(_route)? *= *true|findings\.none\(evidence in \[|quality_shorthand *='
lost=$(git ls-files | grep -E '(^|/)worktrees/' | while IFS= read -r wf; do grep -lE "$WALKED" "$wf"; done)
test -z "$lost" || { echo "FAIL: a tracked file under a directory named worktrees carries a walked pattern — the skip would hide it:"; printf '%s\n' "$lost"; exit 1; }
ctl=target/s8-a55; rm -rf "$ctl"; mkdir -p "$ctl/.redkiln/worktrees/x" "$ctl/.redkiln/packs" || { echo "FAIL: cannot create $ctl"; exit 1; }
printf 'append = true\n' > "$ctl/.redkiln/worktrees/x/a.toml" && printf 'append = true\n' > "$ctl/.redkiln/packs/b.toml" || { echo "FAIL: cannot write the worktrees-skip controls"; exit 1; }
hits=$(grep -rlE --exclude-dir=worktrees 'append(_route)? *= *true' "$ctl/.redkiln" --include=*.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$hits" = "$ctl/.redkiln/packs/b.toml" ] || { echo "FAIL: the worktrees skip is wrong (rc=$rc) — it must drop the run-worktree copy and keep the pack: $hits"; exit 1; }
rm -rf "$ctl"
```

**A6 — (6): build before the binary runs, in both of (6)'s blocks.** This one is not on the plan's list. It is added
because (6)'s two lint lines invoke `./target/debug/redkiln` with no build and no presence guard, which is the shape
golden constraint 3 exists for. *Amended 2026-09-24 (D21-47; review finding 17):* the draft named one insertion point,
but the two lint lines sit in **two** fenced blocks, (6) and (6, second lint). The lines below are inserted twice:
above the line that opens `./target/debug/redkiln lint --facet-stability`, and above the line that opens
`./target/debug/redkiln lint --facet-provenance`. The two lint lines themselves stay bare reads of an exit code, as
`s8` explains. Checkpoint 8j (8)'s build-first checker reads **every** use of `./target/debug/redkiln` on a code line,
not only `test -x`, so a block that runs the binary before a build fails it.

```bash
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A6): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (6) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (6) NOT RUN (Halt 8)"; exit 1; }
```

**A7 — (3)'s third run site goes through the launcher.** This amendment comes from `s8h` §8h.7, not from the plan:
*"`s8-exit-criterion.md` invokes `./target/debug/redkiln run` at three sites. After this stage, each one refuses
without a launch record, so each must go through `tools/live-run.sh run`."* A1 carries (1)'s and (1b)'s run sites.
*Folded into A3, 2026-09-24 (D21-47):* the third site, the line that opened `./target/debug/redkiln run "$ITEM" --lane
fix || { echo "FAIL: the re-invocation exited non-zero"`, is now (3)'s resume block's launcher line, which carries
this amendment's comment. The comment that opened `# Now kill the running process mid-graph` is gone with the rest of
the arm: the kill is (3)'s kill block's, on the launcher's own PID, and `/T` takes its `redkiln` child with it. `s8`
then has **four** launcher `run` lines: (1) launch, (1b) launch, (3) kill and (3) resume.

**A8 — (2) gains the anchored rebuild differential (`s8e` §8e.9).** This one is not on the plan's list either. `s8e`
builds `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`, because the kernel test cannot measure ADR-0105
clause 9's route. `s8e` hands its selection on: *"Adding the anchored test to `s8` (2)'s own block is `s8j`'s
instrument amendment, and it is listed there rather than made here."* Insert the lines below immediately above the
line `# The operator path must still work (AC-427), but its STDOUT is not the comparison.`, which is A2's anchor. The
kernel selections above them stay as they are.

```bash
# AMENDED at s8j (s8j §8j.8 A8; s8e §8e.9): the ROUTE of ADR-0105 clause 9's tier-1 projection — rows whose anchor is
# reachable from the refs, the store deleted, doctor --rebuild, canonical equality — which the kernel test cannot see.
ANCHOR_TESTS="doctor::ac184_rebuild_differential_over_anchored_tier_one_rows"
out=$(cargo test -p rk-cli --test it -- $ANCHOR_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the anchored rebuild differential did not pass (rc=$rc) — AC-184's route is unmeasured"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qxF "test $ANCHOR_TESTS ... ok" || { echo "FAIL: $ANCHOR_TESTS did not report ok by name"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the anchored differential selection did not run exactly one test"; exit 1; }
```

**A9 — the step after Checkpoint 8: AC-114's promotion and its lint get a receiver in `s8`.** *Added 2026-09-24
(D21-47; review finding 10).* §8j.1(e) puts AC-114's promotion after `s8`'s fix-lane run, and the `curation.decision`
lint in the promotion's commit, but no `s8` line named the step. Both edit code paths (`.redkiln/catalog` and
`crates/`). Done between the CI-green frozen sha and Checkpoint 8, they would fail A5.4's *"code changed since the
green CI run"* line and every later pre-spend check. So the step is ordered **after** Checkpoint 8. This paragraph is
appended to `s8` after Checkpoint 8's mechanical block. It is prose, not a checkpoint line:

> **After Checkpoint 8 passes: AC-114's promotion (`s8j` §8j.1(e)).** `tag-release` is promoted out of
> `.redkiln/catalog-extended/`, citing the fix-lane run's id. The promotion's commit also lands the
> `curation.decision` lint, which is `s8j`'s by `s8g` §8g.7's assignment, and it lands only once the operator has taken
> the backfill decision for the 102 shipped activities. That commit is pushed on the operator's go-ahead, and CI is
> made green on it. Checkpoint 8's recorded pass stands for the commit it ran over and is not re-run over this one.

**A10 — Checkpoint 7f's run arm reaches the fixture child, never a real CLI** *(added 2026-09-24; D21-47; cross-file
#8; `s8h` §8h.14).* This one is not an `s8` amendment. It edits `runbook/phase-1/s7f-conformance-and-records.md`, and
it is here because Checkpoint 8j (10) re-runs 7f and cannot pass without it. From `s8h` on, 7f's run arm, which has
been in its throwaway clone since `s8d-ii` §8d-ii.7, has no launch record. It therefore refuses at the launch check
*"before `reconcile_started` is appended"* (`s8h` §8h.7), opens no run record, and its `dag_assembled` guard goes red.
Given a real launch record, it would reach the real CLI once the cards bind. The amendment is applied at this stage's
open, by a dated comment, and it does four things inside the clone and nothing outside it:

1. It writes `harness_binary` into the **clone's** `.redkiln/config.toml` `[adapters]` table, pointing at the product
   repository's `target/debug/rk-fixture-child-claude`, and reads the value back with `tomllib`. D21-47: *"The
   canonical fixture harness is `rk-fixture-child-claude` … A shell replay points `[adapters] harness_binary` at
   `target/debug/rk-fixture-child-claude`."* The shipped config is never edited.
2. It writes a fixture launch record **file** under 7f's scratch directory. The file's path reaches the run only
   through the one exempt line below.
3. It sets `RK_FIXTURE_MODE=exit:0`, so the child exits at once and the `hold` default cannot hang the arm. It sets
   `RK_FIXTURE_MARKER_DIR` to an empty directory.
4. It asserts the spawn markers. The number of `<pid>.marker` files must equal the number of `"kind":"dispatch"`
   lines in the clone's newest `events.jsonl`, and each marker must carry `mode=exit`. Zero markers with zero
   dispatches is consistent: the lane stopped at a gate before it dispatched. A dispatch with no marker is a spawn
   of something other than the fixture child, and it fails.

*Anchors re-taken 2026-09-24 from `s8d-ii` §8d-ii.7 (final consistency pass).* The amended arm *"makes
`c7f=$(mktemp -d)` and converts it with `cygpath -m`"* and *"sets `clone="$c7f/c"` and clones `HEAD` into `"$clone"`
with `git clone --quiet --no-local`"*, so the lines below name the clone `$clone`, the variable `s8d-ii` says this
amendment takes. It then *"runs `init` there, then `run
rk-fix-doctor-test-count --lane fix` with the clone as its working directory, the binary named by its absolute
path"*. The lines are inserted after that `init` and immediately above that `run` line. `${newest}` is the name 7f's
guard gives the newest run directory (`s7f`: `grep -q 'dag_assembled' "${newest}events.jsonl"`), which `s8d-ii`'s
arm reads from the clone's `.git/redkiln/runs` *"by a path that names `$clone` (or `c7f`)"*:

```bash
# AMENDED at s8j (s8j §8j.8 A10; s8h §8h.14; D21-47): the clone's run reaches the fixture child, never a real CLI. The
# launch record is a FILE, supplied on the one line s8j-screen-exempt.txt admits; the spawn markers are counted below.
s8j_root=D:/repos/redkiln-rs   # the product repository, whatever directory the clone arm has cd-ed into
s8j_fx="$s8j_root/target/debug/rk-fixture-child-claude.exe"; [ -f "$s8j_fx" ] || s8j_fx="$s8j_root/target/debug/rk-fixture-child-claude"
test -f "$s8j_fx" || { echo "FAIL: the fixture child is not built — cargo build --workspace first (D21-47)"; exit 1; }
cfg='import sys; p=sys.argv[1]; t=open(p,encoding="utf-8").read(); assert t.count("[adapters]\n")==1, "one [adapters] table"; open(p,"w",encoding="utf-8",newline="\n").write(t.replace("[adapters]\n","[adapters]\nharness_binary = \""+sys.argv[2]+"\"\n",1))'
python -c "$cfg" "$clone/.redkiln/config.toml" "$s8j_fx" || { echo "FAIL: could not point the clone's harness_binary at the fixture child"; exit 1; }
got=$(python -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1],"rb"))["adapters"].get("harness_binary",""))' "$clone/.redkiln/config.toml") || { echo "FAIL: the clone's config does not parse after the edit"; exit 1; }
[ "$got" = "$s8j_fx" ] || { echo "FAIL: the clone's harness_binary reads '$got', not the fixture child"; exit 1; }
s8j_mk="$s8j_root/target/s7f-markers"; rm -rf "$s8j_mk"; mkdir -p "$s8j_mk" || { echo "FAIL: cannot create $s8j_mk"; exit 1; }
s8j_lr="$s8j_root/target/s7f-fixture-launch-record.txt"
printf 'MODE=fixture\nNOTE=Checkpoint 7f fixture route (s8j A10); no real CLI is reachable from this run\n' > "$s8j_lr" || { echo "FAIL: cannot write the fixture launch record"; exit 1; }
```

The clone arm's `redkiln run` line gains the prefix `env REDKILN_LAUNCH_RECORD="$s8j_lr" RK_FIXTURE_MODE=exit:0
RK_FIXTURE_MARKER_DIR="$s8j_mk"` before the binary. The resulting line, byte for byte and with its leading
whitespace stripped, is written as the single entry for stem `s7f-conformance-and-records` in
`runbook/phase-1/s8j-screen-exempt.txt`. That file holds one entry per line, `<stem><TAB><code line><TAB><reason>`.
It is committed in this stage's work commit. Apart from a planted bare-word control, it is the only way a code line
that supplies a launch record or runs the launcher passes Checkpoint 8j (10)'s screen. Every entry must state its
reason and still match a code line of its block. *Measured 2026-09-24 over the working tree, while the fix round
was in progress:* `s8h`'s block was gaining two `tools/live-run.sh wrap` runs against a sandbox repository
(children `sh -c 'printf …'` and `cp`) and a quoted list of refused argv strings. The screen refuses all three.
Each is admitted here, with its reason, only if `s8h` keeps it as landed.

*Re-measured 2026-09-24 (final consistency pass), by running (10)'s own `screen` over the twenty current blocks with an
empty exemption list:* every block except Checkpoint 8h's is clean, and 8h's has exactly those three hits. So the
exemption file carries three more entries, each under stem `s8h-cards-and-run-environment`, each line taken byte for
byte from `s8h`'s (10) as landed, each with its reason. Each is named here by a substring that only it carries,
because `s8h`'s bytes were still moving during this pass:

- the line that carries `tools/live-run.sh wrap -- sh -c 'printf "CHILD_CFG=`. Reason: `s8h`
  (10)'s non-check `wrap` control, run in its scratch repository `$sr` under `env -i` with only fake `claude`s on
  `PATH`; the child prints what it inherited, and the launch record lands in the scratch repository, never here.
- the line that carries `tools/live-run.sh wrap -- cp "$sw/claude.moved"`. Reason: `s8h` (10)'s
  step-9 control (mutant M22), in the same scratch repository; the child copies one fake `claude` over another.
- the line that carries `for argv in "./target/debug/redkiln run rk-x"`. Reason: `s8h` (10)'s `WRAP_ARGV` refusals;
  the strings are argv handed to `--check wrap`, which spawns nothing.

(10)'s stale-entry check refuses an entry that no longer matches a code line of 8h's block, so a reshaped `s8h` line
re-opens its entry rather than passing silently. With those three entries in an exemption file, the screen found
zero hits on all twenty blocks [measured 2026-09-24, final consistency pass]. The following lines are inserted after 7f's `dag_assembled` guard:

```bash
s8j_nm=$(find "$s8j_mk" -name '*.marker' | grep -c .)
s8j_nd=$(grep -c '"kind":"dispatch"' "${newest}events.jsonl")
[ "$s8j_nm" -eq "$s8j_nd" ] || { echo "FAIL: $s8j_nm fixture-child spawn markers for $s8j_nd dispatch events — a dispatch reached something other than the fixture child (D21-47)"; exit 1; }
if [ "$s8j_nm" -gt 0 ]; then s8j_bad=$(grep -L '^mode=exit$' "$s8j_mk"/*.marker); [ -z "$s8j_bad" ] || { echo "FAIL: a spawn marker without mode=exit: $s8j_bad"; exit 1; }; fi
```

**A10's two dependent amendments: the card-mode stops in Checkpoints 8d-ii and 8e (10d)** *(added 2026-09-24,
final consistency pass (lead); `s8h` §8h.14 table and finding 27).* Each of those stops refuses once a shipped card
carries `permission_mode`, because each re-runs Checkpoint 7f whole and 7f's run arm could reach `redkiln run`. A10
moves that premise: after A10, 7f's arm reaches only the fixture child. So A10's commit amends both, by D21-47's
time-bound rule, and lists 8d-ii and 8e as provisional (Checkpoint 8j (10) re-runs both). In `s8d-ii`'s non-nested
precondition, the `moded=$(grep -lF '"permission_mode"' .redkiln/cards/*.json); rc=$?` line, its `[ "$rc" -le 1 ]`
guard and the `[ -z "$moded" ]` stop, and in `s8e`'s (10d) the `raw=$(grep -l 'permission_mode' .redkiln/cards/*.json); rc=$?`
line and its `[ "$rc" -eq 1 ]` stop, are each replaced by one dated comment and the reading below, over the block's
own extracted 7f copy (`$scratch/cp7f.sh` in both; in `s8d-ii` the extraction moves above the reading). The
card-count lines stay. `s8d-ii`'s planted-mode control is re-pointed: it plants a copy of 7f without the marker and
requires the reading to refuse it.

```bash
  # AMENDED at s8j (s8j §8j.8 A10; D21-47 time-bound rule): 7f's run arm reaches the fixture child, never a real CLI,
  # so a shipped card's permission_mode no longer bears on this replay. The premise is A10's marker in 7f's CODE.
  n=$(grep -vE '^[[:space:]]*#' "$scratch/cp7f.sh" | grep -cF 'rk-fixture-child-claude')
  [ "$n" -ge 1 ] || { echo "FAIL: Checkpoint 7f's code does not route its run arm to rk-fixture-child-claude (s8j A10 has not landed) — its run could reach a real dispatch; stop"; exit 1; }
```

**A11 — Checkpoint 8a (11)'s card guard: `s8h` owns it** *(added 2026-09-24; D21-47; review finding 5. Reduced
2026-09-24 by the final consistency pass (lead) to a citation, so the edit is made once).* The draft replaced 8a
(11)'s card guard here, and a later draft inserted a launch-check proof after `s8h`'s assertion. `s8h` §8h.14 now
makes both edits in its own work commit, by D21-47's time-bound rule: the all-cards assertion and, *"The same second
assertion joins 8a's amendment"*, the by-name run of
`run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn`. This stage makes no edit to `s8a`.
Checkpoint 8j re-runs 8a with `RK_CP_NESTED=1`, so 8a's non-nested branch is skipped there.

**A12 — `rk_absent` reads with `-H` in Checkpoints 7c and 7d** *(added 2026-09-24, final consistency pass; `s8h`
finding 20)*. This is not an `s8` amendment either. It edits `runbook/phase-1/s7c-adapters.md` and
`runbook/phase-1/s7d-cli-and-migrate.md`, whose checkpoints this stage re-runs (§8j.6 step 2; Checkpoint 8j (10)).
`s8h` found the defect and left the fix to them: *"`s7c` §7c.21's and `s7d`'s `rk_absent` run `grep -rnE "$1" $scope`
with no `-H`, so a single-file scope prints no file name and the comment filter cannot drop a `//` line."* In each
file, the line inside `rk_absent() {` that reads `raw=$(grep -rnE "$1" $scope --include=*.rs); rc=$?` becomes
`raw=$(grep -rnHE "$1" $scope --include=*.rs); rc=$?`. One dated `#` line goes immediately above `rk_absent() {`,
never inside it, so each file's stated count of helper-definition lines still holds. No reading moves today: every
one of the 42 bans in the two blocks (23 and 19) names a directory or several operands, for which `grep -r` already
prints file names [measured 2026-09-24 over the bans' scope arguments]. The fix removes the false red that a
single-file scope would give. It lands in the work commit, before §8j.6 step 1b, so 7c's split run and (10)'s 7d
re-run both run it. §8j.6 step 2's block asserts it over 7c's suite portion, and Checkpoint 8j (10) over 7d's block.

### 8j.9 Acceptance set

**Claimed.**

- **AC-410**: the positive control through a pack-declared question, over both human roots.
- **AC-054**: the step-4 gate is answerable, and a re-invocation reads the root as Present.
- **AC-518**: the user's checkout is unchanged across a full lane run. This is its Verified-by, *"an integration test
  asserting the user checkout is unchanged across a full lane run"*, in `lane_proof`.
- **AC-226**: after the pin move, the configured `Refuse` outcome is still the asserted one.
- The first run of every `ci.yml` job that `ci.yml` marks *"ARMED, NOT DISCHARGED"*:
  - **AC-498**, the no-egress job;
  - **AC-465**;
  - **AC-362**;
  - **AC-441**, **AC-442** and **AC-466**;
  - **AC-481**, **AC-483** and **AC-528**;
  - **AC-529**.

**Excluded, each for its own reason.**

- **AC-473** and **AC-521** are claimed by `s8`'s real runs. The fixture proofs are the spend gate D21-40 decision 2
  names, not the claim.
- **AC-160…AC-163** belong to `s8e`. ADR-0158's forbids bullet, *"Claiming AC-160, AC-161, AC-162 or AC-163 is met,
  on any axis"*, stands wherever `s8e`'s record did not lift it.
- **AC-184** is `s8`'s (2).
- **AC-372, AC-373, AC-394 and AC-395** are `s8`'s (7).
- **AC-276** belongs to `s8h`.
- **AC-114**'s promotion follows `s8`'s fix-lane run (§8j.1(e)). This stage does not claim it, and it does not
  build the promotion's `curation.decision` lint.
- **AC-121's cross-OS half.** The two legs make it checkable, but no instrument named here asserts it, so it is not
  claimed.

### 8j.10 Checkpoint 8j

- **Both lanes are proven end to end under the FakeHarness**, through `verbs::run::run` with the real Rust adapter,
  across real process boundaries. That is ten named `lane_proof` tests, three human-root question tests and the
  hold's load-refusal test (§8j.3): fourteen, selected with `s8h`'s launch-record lint (§8j.4) as fifteen. Each lane's
  `Spec` carries a `verification` on every AC before verify runs.
- **The items are as §8j.1 and §8j.5 say.**
  - The story item exists at `triage`, with low facets, its slice and its trial declaration.
  - The Codex item is gone.
  - The empty-case item exists, and Checkpoint 7i names it.
  - The fixture item's non-comment lines and ref are untouched. Its checkout is re-pointed to `.redkiln/worktrees/`,
    clean and without an `RK_BASE` marker, so `s8d-ii`'s re-entry refuses it.
  - Both packs carry one fail-closed `[[gate_expiry]]` row for each of this stage's three gate classes.
  - The reserved fix item exists with its hold, slice, trial declaration, BugReport and repro pair. It has no run
    ref.
  - The reserved defect **is still red** at HEAD, in a throwaway tree, as an integration or a unit repro.
- **`spike.toml` is untouched** since `ca8f387`, and a byte change to it in a clone moves no test of the kernel or the
  two adapters that read `spikes/p9` (D21-47 V2): the story's own verify will not go red on a historical pin.
- **Checkpoints 7h and 7i read named fixture items**, and neither reads `$FIX_ITEM`.
- **The pin, the instrument record, Checkpoint 7c's split run, probe 0 and both probe results agree**, and the pin equals
  S-BIN's pinned `cli_version`. The fixture proofs and every mutant were recorded over the code CI ran, before any
  paid step. Each paid run has its launch record, with rc 0 and a HEAD carrying that code. The (g) record is
  `accepted` only with both probes recorded as passing. *D21-48 (2026-09-24):* the argv spelling table holds one row,
  keyed by the pin. The pin's `--help` capture lists that row's spelling, and the golden argv carries it (X12).
- **CI is green on the recorded run**: every job, both legs, and no code change since.
- **`s8`'s amendments landed**:
  - no `BIND:`;
  - no use of the binary before a build;
  - no bare `redkiln run` or `redkiln answer`, since each goes through the launcher;
  - four launcher runs, each in a block that runs the pre-spend checks first;
  - `s8b`'s guard form and its catalog selection copied verbatim;
  - every selected test exists;
  - AC-114's post-Checkpoint-8 step named, and `INDEX.md` row 19's entry gate is Checkpoint 8j.
- **The entry chain re-runs once, link by link, and so does every older checkpoint an `s8` stage made
  provisional.** The links are Checkpoints 8a, 8b, 8c, 8d, 8d-ii, 8e, 8f, 8g, 8h and 8i. The older set, read from
  each `s8` file's "made provisional" statements, is 5, 6, 7a, 7b, 7b-ii, 7d, 7f, 7g, 7h and 7i: `s8a` (5, 7a, 7b,
  7f), `s8c` (5, 7a), `s8d` (7g), `s8d-ii` (7f), `s8e` (7a, 7f, 7h, 7i), `s8f` (7i), `s8g` (5, 6, 7b, 7b-ii, 7d),
  `s8h` (5, 7b, 7b-ii, 7d, 7f), `s8i` (5), and this stage (7h, 7i). 7c is §8j.6's live split run (D21-43 N28;
  D21-47), and 7e cannot be re-run whole (its block commits and tags). Each runs whole with `RK_CP_NESTED=1`
  (D21-46), and each reaches its end marker. Each is fenced against a real dispatch by D21-47's wrapper, the shared
  `runbook/tools/nodispatch.sh`: an unset launch record, a compiled fake `claude` first on `PATH` that prints the pin,
  every other `PATH` directory holding a `claude` removed, and an empty `CLAUDE_CONFIG_DIR` trap, with `nd_check`
  after each replay. The screen refuses every executing shape except a planted bare-word control and the lines the
  exemption file lists, each with its reason: 7f's A10 line and `s8h`'s three. 7d's `rk_absent` reads with `-H`
  (A12). When Checkpoint 8j is itself run with
  `RK_CP_NESTED=1`, it re-runs none of them.
- **The whole suite, fmt and clippy are green, and the three runbook checkers pass.**
- **This block dispatched nothing, and it changed no store row, no `rk-run/*` ref and no launch record.** The fake
  `claude` was asked for nothing but `--version`.

**Run it from a Git Bash that Claude Code did not spawn.** Checkpoints 8h and 8i refuse a Claude-spawned shell, and
this block re-runs them. *Revised 2026-09-24 (final consistency pass):* record ids and commit shas need no input. Each
block carries its own as literals written at record time and refuses while one is unwritten (D21-47), 8a's, 8i's and
this block's (3) included. No chain block takes an environment input at this pass [measured 2026-09-24: a grep for
`FAIL: set` over the twenty blocks' files finds only 7h's and 7i's `FIX_ITEM`, which §8j.1(c) replaces]. Checkpoint
8h's `MINTED_CLASSES` literal carries this stage's three gate classes, added by this stage's dated edit (§8j.2). Checkpoint 8d
carries its entry sha as the literal `CP8D_ENTRY=` (`s8d` (0)), so this block supplies nothing. The shared helper
needs `rustc` on `PATH` for its fake `claude`, which cargo's toolchain provides. Budget hours: (10) re-runs twenty checkpoints
whole, most of them with a whole `cargo test --workspace`. At one cold-suite wall time each (`s8c` §8c.5 measures the
run child), that is about twenty whole-suite runs plus each block's named selections, a working day or more. Run no
Claude session in this repository while it runs: (12)'s transcript tripwire reads this repository's project
directories.

🛑 **Checkpoint 8j is one-shot** *(stated 2026-09-24, D21-47; review finding 24).* It passes once, over the
frozen-record commit (§8j.10's stage close), before `s8`'s first launch. It is never re-run after that launch: (3)
refuses an existing `rk-run/<fix item>`, which the first launch creates, and (12) refuses a moved store. `s8` reads
the pass recorded in `PROGRESS.md` row 8j. A re-run after the first launch fails at (3) by design.

**At stage open, the chain is run once whole with its failures neutralised** *(added 2026-09-24, D21-47; review
finding 5).* A checkpoint that has never been run whole accumulates assertions that cannot be satisfied. Checkpoint
7d failed four times at its first whole run, and none of those failures belonged to the current slice. So before any
work, (10)'s twenty blocks are extracted and run once each under the same wrapper, with `RK_CP_NESTED=1` as (10) runs
them. Every failing line is recorded and
then neutralised in a scratch copy, never in the stage file, and the run is repeated until each block reaches its end.
The list of failures is this stage's opening blocker list. The known entries are these. 8a's guard (7) expects the
trailer lint `ARMED`, and `s8e` owes that amendment (cross-file #2); (10) refuses by name while it is missing. 7f's
arm refuses at the launch check without A10. 8a (11)'s card guard would fire on 8a's own non-nested run, and `s8h`
re-scopes it in its own work commit (`s8h` §8h.14; A11 cites it).

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# 🛑 THIS BLOCK DISPATCHES NOTHING AND PUSHES NOTHING. CI status and probe results are READ from the run ids and lines
# recorded in runbook/phase-1/s8j-evidence.txt. Every cargo reading is `out=$(…); rc=$?`; every selection is `--test it`,
# module-qualified and `--exact`, with each test's own `... ok` line and an exact count asserted.
# 🛑 ONE-SHOT (s8j §8j.10): it passes once, before s8's first launch; after that launch (3) fails by design.

# ---- (0) THIS SHELL, THEN READ FIRST: every quantity a dispatch, a run or a launch would move. None may move.
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_|CLAUDE_PID=|CLAUDE_EFFORT=|AI_AGENT=)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — Checkpoints 8h and 8i, which (10) re-runs, refuse a Claude-spawned shell"; exit 1; }
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
gcd=$(git rev-parse --git-common-dir) || { echo "FAIL: git rev-parse --git-common-dir failed"; exit 1; }
store="$gcd/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
# After s8e's 0002 migration (D21-46 R2) the gate table is t1_gate_transition; t1_gate no longer exists.
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate_transition","t1_slice_seal","t2_event")), c.execute("SELECT COUNT(*) FROM t2_event WHERE kind IN (?,?,?)",("dispatch","dispatch_completed","rate_limit_reading")).fetchone()[0])'
rows_before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$rows_before" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }
refs_before=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
[ "$rc" -eq 0 ] && [ -n "$refs_before" ] || { echo "FAIL: no rk-run/* ref could be read (rc=$rc) — rk-run/rk-fix-doctor-test-count at least must exist"; exit 1; }
launches="$gcd/redkiln/launches"
nl_before=$(find "$launches" -type f 2>/dev/null | sort)
# The transcript tripwire: a real `claude -p` persists a session .jsonl under its config dir's projects/. It is a
# DETECTOR beside the prevention, not the prevention, and it reads THIS REPOSITORY's project directories only: a
# concurrent session elsewhere is not this block's business (review finding 23). The trap below is the primary reading.
CFG_DIRS=""
for d in "$HOME/.claude-personal/projects" "$HOME/.claude/projects"; do [ -d "$d" ] && CFG_DIRS="$CFG_DIRS $d"; done
test -n "$CFG_DIRS" || { echo "FAIL: no Claude projects directory under \$HOME — the transcript tripwire has no baseline"; exit 1; }
sess_before=$(find $CFG_DIRS -path '*redkiln-rs*' -type f -name '*.jsonl' 2>/dev/null | sort)   # unquoted on purpose: a word per directory
tdir=target/s8j-cp8j; rm -rf "$tdir"; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir (a git-bash /tmp path is invisible to Windows Python, so scratch lives under target/)"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — nothing this stage's gated acts recorded can be read"; exit 1; }
if od -c "$ev" | grep -qF '\r'; then echo "FAIL: $ev carries a CR byte — a field read from it would carry one too"; exit 1; fi
CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
# D21-47'S WRAPPER, the shared helper runbook/tools/nodispatch.sh (final consistency pass 2026-09-24), for this
# block's own process-level cargo runs and for every replay in (10): REDKILN_LAUNCH_RECORD unset; a COMPILED fake
# `claude` first on PATH that prints the adapter pin for --version and logs and refuses anything else, so it shadows a
# Rust `Command::new("claude")` as well as a shell's; every other PATH directory holding a claude removed; and
# CLAUDE_CONFIG_DIR at an empty trap. nd_check (the trap empty, no fake call logged) follows each cargo run and each
# replay, and closes (12). Run nested (ND_ACTIVE=1), nd_setup verifies and reuses the enclosing wrapper.
. runbook/tools/nodispatch.sh && nd_setup "$PWD/$tdir/nd" || exit 1
wrapped() { nodispatch env -u FIX_ITEM -u STORY_ITEM -u ITEM "$@"; }

# ---- (1) BUILD-OR-FAIL.
out=$(wrapped cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8j NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE FIXTURE PROOFS (§8j.4), THE HUMAN-ROOT QUESTION TESTS (§8j.2), THE HOLD'S LOAD REFUSAL (§8j.3) AND THE
# ---- s8h's LAUNCH-RECORD LINT (§8j.4): one rk-cli selection of fifteen, fourteen of them this stage's, under the wrapper.
test -d crates/rk-cli/tests/fixtures/lane || { echo "FAIL: the lane fixture is absent — the proofs would materialise nothing"; exit 1; }
test -f crates/rk-cli/tests/it/lane_proof.rs || { echo "FAIL: crates/rk-cli/tests/it/lane_proof.rs is absent"; exit 1; }
R_TESTS="lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness lane_proof::fix_lane_trailers_every_non_forbidden_stage_in_order lane_proof::fix_lane_verify_completed_names_the_rust_adapter lane_proof::story_lane_spec_is_landed_and_journalled lane_proof::story_lane_trailers_every_non_forbidden_stage_in_order lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes lane_proof::gate_then_answer_then_second_process_continues_binary_arm lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification lane_proof::boundary_hold_changes_requested_does_not_advance processes::the_two_human_root_questions_load_and_each_attaches_its_kind processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load run::every_binary_spawn_withholds_or_supplies_the_launch_record"
out=$(wrapped cargo test -p rk-cli --test it -- $R_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the lane proofs and their companions did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
for t in $R_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 15 passed; 0 failed' || { echo "FAIL: the selection did not run exactly 15 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
nd_check "(2)" || exit 1
# The proofs use the REAL Rust adapter: no proof names the fixture toolchain child or FixtureToolchain on a CODE line —
# not even to assert its absence, which the verify_completed identity test asserts behaviourally. A FLOOR. The file is
# lane_proof.rs alone: s8e's lane_e2e.rs runs pack-min's fixture lane and is not bound by this.
raw=$(grep -nE 'rk-fixture-toolchain|FixtureToolchain' crates/rk-cli/tests/it/lane_proof.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the fixture-toolchain floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: a lane proof names a fixture toolchain on a code line — AC-473's adapter identity would be proved over a double:"; printf '%s\n' "$code"; exit 1; }
syn=$(printf '12:    let t = FixtureToolchain::default();\n' | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -E 'rk-fixture-toolchain|FixtureToolchain'); test -n "$syn" || { echo "FAIL: the comment filter swallowed a code line — the floor is vacuous"; exit 1; }

# ---- (3) THE ITEMS (§8j.1, §8j.5), THE SLICES, AND THE RE-POINTED CHECKOUT.
itm='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); f=d.get("facets",{}); t=d.get("trial",{}); print(d.get("id"), d.get("kind"), d.get("stage"), d.get("hold_before","-"), d.get("slice","-"), f.get("cos","-"), f.get("criticality","-"), f.get("blast_radius","-"), f.get("size","-"), str(f.get("external_dep","-")).lower(), str(f.get("has_repro","-")).lower(), t.get("activity","-"))'
test ! -e .redkiln/items/rk-story-codex-adapter.toml || { echo "FAIL: the Codex item is not retired (ADR-0007; D21-41 (f))"; exit 1; }
s=$(python -c "$itm" .redkiln/items/rk-story-harden-spike-pack.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the story item did not parse (rc=$rc)"; exit 1; }
[ "$s" = "rk-story-harden-spike-pack story triage - harden-spike-pack standard low low low false - tag-release" ] || { echo "FAIL: the story item is '$s', not a story at triage with its slice, low facets, external_dep = false and the tag-release trial"; exit 1; }
e=$(python -c "$itm" .redkiln/items/rk-cost-empty-case.toml); rc=$?
[ "$rc" -eq 0 ] && [ "${e%% *}" = "rk-cost-empty-case" ] || { echo "FAIL: the empty-case item is absent or mis-named ('$e', rc=$rc)"; exit 1; }
fx_line=$(grep -E '^fix-item rk-fix-[a-z0-9-]+ [0-9a-f]{40}$' "$ev" | tail -1)
test -n "$fx_line" || { echo "FAIL: no fix-item line in $ev — the reservation is unrecorded"; exit 1; }
read -r _ fx fx_frozen <<< "$fx_line"
grep -qE "^reserved-fix-item: $fx [a-z0-9_-]+:[A-Za-z0-9_:]+\$" spikes/DECISIONS-2026-09-21.md || { echo "FAIL: $fx is not reserved in the ledger (D21-42 18)"; exit 1; }
f=$(python -c "$itm" ".redkiln/items/$fx.toml"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: .redkiln/items/$fx.toml did not parse (rc=$rc)"; exit 1; }
[ "$f" = "$fx bug triage specification ${fx#rk-fix-} standard low low low - true tag-release" ] || { echo "FAIL: the reserved item is '$f', not a bug at triage holding before specification, with slice ${fx#rk-fix-}, low facets, has_repro and the tag-release trial"; exit 1; }
# s8e §8e.8's refusals, read here rather than trusted to the mint: a slice that is the id, the project or a stage.
slc='import sys,tomllib
d=tomllib.load(open(sys.argv[1],"rb")); p=tomllib.load(open(sys.argv[2],"rb"))["policy"]; s=d.get("slice")
bad=[]
if not isinstance(s,str) or not s: bad.append("no slice")
elif any(c.isspace() for c in s): bad.append("whitespace in the slice")
elif s in (d.get("id"), sys.argv[3]) or s in p["stage_order"]: bad.append("the slice "+s+" is the item id, the project or a stage")
print("; ".join(bad) if bad else "ok"); sys.exit(1 if bad else 0)'
for pair in "rk-story-harden-spike-pack:story" "$fx:bug-standard"; do
  v=$(python -c "$slc" ".redkiln/items/${pair%%:*}.toml" ".redkiln/packs/${pair##*:}.toml" redkiln-rs); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: ${pair%%:*}'s slice: $v (s8e §8e.8)"; exit 1; }
done
printf 'id = "rk-fix-x"\nkind = "bug"\nstage = "triage"\nslice = "rk-fix-x"\n' > "$tdir/slice-red.toml" || { echo "FAIL: could not write the slice control"; exit 1; }
v=$(python -c "$slc" "$tdir/slice-red.toml" .redkiln/packs/bug-standard.toml redkiln-rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the slice check accepted a slice equal to the item id (rc=$rc: $v) — it is vacuous"; exit 1; }
if git rev-parse --verify --quiet "refs/heads/rk-run/$fx" >/dev/null; then echo "FAIL: rk-run/$fx already exists — the fresh item's run must fork from the frozen HEAD at s8's first run (and Checkpoint 8j is one-shot: it is not re-run after that run)"; exit 1; fi
test ! -e ".redkiln/worktrees/$fx" || { echo "FAIL: .redkiln/worktrees/$fx exists — nothing may have run the reserved item"; exit 1; }
test -s ".redkiln/intake/$fx.BugReport.md" || { echo "FAIL: no BugReport for $fx"; exit 1; }
test -s .redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md || { echo "FAIL: no IntakeRecord for the story item"; exit 1; }
old=$(git show ca8f387:.redkiln/items/rk-fix-doctor-test-count.toml | grep -vE '^[[:space:]]*(#|$)')
new=$(grep -vE '^[[:space:]]*(#|$)' .redkiln/items/rk-fix-doctor-test-count.toml)
[ -n "$old" ] && [ "$old" = "$new" ] || { echo "FAIL: rk-fix-doctor-test-count's non-comment lines changed — it is the UNTOUCHED fixture item (D21-42 18)"; exit 1; }
r_now=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); r_pin=$(git rev-parse --verify --quiet "73c087a^{commit}")
[ -n "$r_now" ] && [ -n "$r_pin" ] && [ "$r_now" = "$r_pin" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is '$r_now', not 73c087a ('$r_pin') — Checkpoints 7h and 7i read it; two empty reads never compare equal here"; exit 1; }
# D21-44: the legacy checkout is re-pointed from .git/redkiln/worktrees/ to .redkiln/worktrees/, its ref kept, and it
# carries no RK_BASE marker, so s8d-ii's re-entry refuses any run of the item (D21-45 W2; D21-46 R1; §8j.1(d)).
wtl=$(git worktree list --porcelain); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree list failed (rc=$rc)"; exit 1; }
if printf '%s\n' "$wtl" | grep -qE '^worktree .*/\.git/redkiln/worktrees/'; then echo "FAIL: a worktree is still registered under .git/redkiln/worktrees/ — the legacy placement was not re-pointed (s8j §8j.1(d); D21-44)"; exit 1; fi
top=$(git rev-parse --show-toplevel) || { echo "FAIL: git rev-parse --show-toplevel failed"; exit 1; }   # D:/… — the form `git worktree list` prints; Git Bash's pwd prints /d/…
lwt="$top/.redkiln/worktrees/rk-fix-doctor-test-count"
printf '%s\n' "$wtl" | grep -qxF "worktree $lwt" || { echo "FAIL: the legacy checkout is not registered at $lwt"; exit 1; }
lh=$(git -C "$lwt" rev-parse HEAD); rc=$?
[ "$rc" -eq 0 ] && [ "$lh" = "$r_pin" ] || { echo "FAIL: the re-pointed checkout is at '$lh', not 73c087a (rc=$rc)"; exit 1; }
lst=$(git -C "$lwt" status --porcelain); rc=$?
[ "$rc" -eq 0 ] && [ -z "$lst" ] || { echo "FAIL: the re-pointed checkout is not clean (rc=$rc)"; exit 1; }
lgd=$(git -C "$lwt" rev-parse --absolute-git-dir); rc=$?
[ "$rc" -eq 0 ] && [ -d "$lgd" ] || { echo "FAIL: the re-pointed checkout's gitdir could not be read (rc=$rc)"; exit 1; }
test ! -e "$lgd/RK_BASE" || { echo "FAIL: the legacy checkout carries an RK_BASE marker — s8d-ii's re-entry would ADOPT it instead of refusing it"; exit 1; }
git check-ignore -q .redkiln/worktrees/rk-fix-doctor-test-count || { echo "FAIL: .redkiln/worktrees/ is not ignored — the re-pointed checkout dirties git status (AC-518)"; exit 1; }
test ! -e "$gcd/redkiln/worktrees/rk-fix-doctor-test-count" || { echo "FAIL: the old checkout directory is still on disk under .git"; exit 1; }
grep -qE '^legacy-worktree-repointed rk-fix-doctor-test-count 73c087a [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" || { echo "FAIL: the re-point is not recorded in $ev"; exit 1; }
# §8j.2's and §8j.3's three gate classes each carry ONE fail-closed [[gate_expiry]] row in BOTH packs (s8h §8h.10).
# The spellings are drafts the records may re-spell: edit S8J_GATE_CLASSES by a dated line when they do.
S8J_GATE_CLASSES="human_root:BugReport human_root:IntakeRecord hold:specification"
ge='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); rows=d.get("gate_expiry",[]); print(" ".join(c+"="+str(sum(1 for r in rows if r.get("class")==c and r.get("action")=="fail")) for c in sys.argv[2:]))'
want=$(for c in $S8J_GATE_CLASSES; do printf '%s=1 ' "$c"; done); want=${want% }
for p in bug-standard story; do
  got=$(python -c "$ge" ".redkiln/packs/$p.toml" $S8J_GATE_CLASSES); rc=$?
  [ "$rc" -eq 0 ] && [ "$got" = "$want" ] || { echo "FAIL: $p.toml's gate rows for this stage's classes read '$got' (rc=$rc), expected '$want' — s8h's load lint would refuse every run that reaches one"; exit 1; }
done
printf '[[gate_expiry]]\nclass = "a"\naction = "fail"\n' > "$tdir/ge-ctl.toml" || { echo "FAIL: could not write the gate-row control"; exit 1; }
v=$(python -c "$ge" "$tdir/ge-ctl.toml" a b); rc=$?
[ "$rc" -eq 0 ] && [ "$v" = "a=1 b=0" ] || { echo "FAIL: the gate-row reader read '$v' (rc=$rc) over one row for a and none for b — it is vacuous"; exit 1; }
# This stage's two records (§8j.2, §8j.3), located by the ids written HERE by dated edit when each is written (D21-47;
# the final consistency pass's convention). A re-runner supplies no id; an unwritten one refuses.
S8J_Q_ADR=
S8J_HOLD_ADR=
for r in "$S8J_Q_ADR" "$S8J_HOLD_ADR"; do
  printf '%s\n' "$r" | grep -qxE '0[0-9]{3}' || { echo "FAIL: a record id of this stage (S8J_Q_ADR, S8J_HOLD_ADR; §8j.2, §8j.3) is not written into (3): '$r'"; exit 1; }
  set -- references/adr/"$r"-*.md
  [ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: expected exactly one references/adr/$r-*.md for the id written into (3)"; exit 1; }
  grep -qxE 'status: (proposed|accepted)' "$1" || { echo "FAIL: ADR-$r is neither proposed nor accepted"; exit 1; }
done
# The story is the lane's to author: this stage did not touch spike.toml.
test -f spikes/p9/packs/spike.toml || { echo "FAIL: spikes/p9/packs/spike.toml is gone"; exit 1; }
git diff --quiet ca8f387 HEAD -- spikes/p9/packs/spike.toml || { echo "FAIL: spike.toml changed since ca8f387 — the story lane would have nothing to author"; exit 1; }

# ---- (3b) THE STORY DOES NOT BREAK THE SUITE AT ITS OWN VERIFY (D21-47 V2): every test pinning a historical spikes/p9
# ---- measurement reads the p9 tree at its pinned commit (s8b), so a byte change to spike.toml in a CLONE — the class of
# ---- edit the story lane lands — reddens no test of the three crates that read spikes/p9. Never in this checkout.
# ---- The clone's worst path is 44 + 140 = 184 characters (ADR-0114's file name), under 260 [measured 2026-09-24;
# ---- the same clone under the session scratch directory failed checkout with "Filename too long"].
p9c="$tdir/p9clone"
out=$(git clone -q --shared . "$p9c" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: could not clone HEAD into $p9c (rc=$rc): $out"; exit 1; }
[ "$(git -C "$p9c" rev-parse HEAD)" = "$(git rev-parse HEAD)" ] || { echo "FAIL: the clone is not at HEAD"; exit 1; }
printf '\n# s8j Checkpoint 8j (3b): a byte perturbation in a throwaway clone, never committed\n' >> "$p9c/spikes/p9/packs/spike.toml" || { echo "FAIL: could not perturb the clone's spike.toml"; exit 1; }
git -C "$p9c" diff --quiet -- spikes/p9/packs/spike.toml; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the perturbation did not change spike.toml's bytes in the clone (rc=$rc) — the control is vacuous"; exit 1; }
p9t="$PWD/target/s8j-repro-target"
out=$(cd "$p9c" && wrapped env CARGO_TARGET_DIR="$p9t" cargo test -p rk-kernel -p rk-adapter-sqlite -p rk-adapter-toolchain-rust --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (V2): a test reddened on a byte change to spike.toml — it reads the p9 WORKING tree, not its pinned commit (D21-47 V2; s8b), and the story lane's verify would go red on it:"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|test result:' | tail -20; exit 1; }
nbad=$(printf '%s\n' "$out" | grep -E '^test result: ' | grep -vc ' 0 failed;')
[ "$nbad" = "0" ] || { echo "FAIL (V2): $nbad test binaries report failures over the perturbed clone"; exit 1; }
nd_check "(3b)" || exit 1

# ---- (4) THE RESERVED DEFECT IS STILL RED AT HEAD — in a throwaway tree, never in the crate. Two kinds (review
# ---- finding 11): `it`, appended to tests/it/<module>.rs; `unit`, a `#[cfg(test)] mod` appended to src/<file>, for a
# ---- private function a child module can see.
rmeta=".redkiln/intake/$fx.repro.toml"; rsrc=".redkiln/intake/$fx.repro.rs"
test -s "$rmeta" && test -s "$rsrc" || { echo "FAIL: $fx's repro pair is absent"; exit 1; }
rp='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); print(d.get("kind","")); print(d["crate"]); print(d["module"]); print(d["test"]); print(d["expect"]); print(d.get("file","-"))'
meta=$(python -c "$rp" "$rmeta"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: $rmeta did not parse (rc=$rc)"; exit 1; }
{ read -r rkind; read -r rcrate; read -r rmod; read -r rtest; read -r rexpect; read -r rfile; } <<< "$meta"
[ -n "$rcrate" ] && [ -n "$rmod" ] && [ -n "$rtest" ] && [ -n "$rexpect" ] || { echo "FAIL: $rmeta lacks crate/module/test/expect"; exit 1; }
case "$rkind" in
  it) tf="crates/$rcrate/tests/it/$rmod.rs"; tsel="--test it" ;;
  unit) tf="crates/$rcrate/src/$rfile"; tsel="--lib" ;;
  *) echo "FAIL: $rmeta's kind is '$rkind', not it or unit (s8j §8j.5 step 3)"; exit 1 ;;
esac
test -f "$tf" || { echo "FAIL: $tf does not exist — the repro has nowhere to be appended"; exit 1; }
n=$(grep -cE "fn $rtest\(" "$tf")
[ "$n" = "0" ] || { echo "FAIL: the repro test is committed INTO the crate ($tf) — it would turn CI red and hand the lane's red step its answer"; exit 1; }
git archive --format=tar -o "$tdir/head.tar" HEAD || { echo "FAIL: git archive HEAD failed"; exit 1; }
mkdir -p "$tdir/repro" && tar -xf "$tdir/head.tar" -C "$tdir/repro" || { echo "FAIL: could not extract HEAD"; exit 1; }
cat "$rsrc" >> "$tdir/repro/$tf" || { echo "FAIL: could not append the repro"; exit 1; }
out=$(cd "$tdir/repro" && wrapped env CARGO_TARGET_DIR="$p9t" cargo test -p "$rcrate" $tsel -- "$rmod::$rtest" --exact 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the reserved defect's repro PASSES at HEAD — the defect was fixed before the run (D21-42 18)"; exit 1; }
printf '%s\n' "$out" | grep -qxF "test $rmod::$rtest ... FAILED" || { echo "FAIL: the repro did not run and fail by name — red for another reason is not the defect"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qF "$rexpect" || { echo "FAIL: the repro failed without its expected message '$rexpect' — a red for the wrong reason"; exit 1; }
nd_check "(4)" || exit 1

# ---- (5) CHECKPOINTS 7i AND 7h ARE RE-POINTED (§8j.1(c)): code lines name the fixture items, never $FIX_ITEM.
x7='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
b=[x for x in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in x]
code="\n".join(l for l in (b[-1].split("\n") if b else []) if not l.strip().startswith("#"))
print(len(b), code.count("FIX_ITEM"), sum(1 for l in code.split("\n") if l.strip()=="FIXTURE_ITEM=rk-fix-doctor-test-count"), code.count("rk-story-codex-adapter"), code.count("rk-cost-empty-case"), code.count("FAIL (PRECONDITION): rk-cost-empty-case owns"))'
c7=$(python -c "$x7" runbook/phase-1/s7i-cost.md 'CP7I-END-OF-BLOCK reached'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7i could not be read (rc=$rc)"; exit 1; }
read -r nb c_fix c_asg c_old c_new c_pre <<< "$c7"
[ "$nb" -ge 1 ] && [ "$c_fix" = "0" ] && [ "$c_asg" = "1" ] && [ "$c_old" = "0" ] && [ "$c_new" -ge 7 ] && [ "$c_pre" = "1" ] || { echo "FAIL: Checkpoint 7i's code lines: $c_fix FIX_ITEM, $c_asg fixture assignments, $c_old Codex-item names, $c_new empty-case names, $c_pre PRECONDITION guards (blocks: $nb)"; exit 1; }
c7=$(python -c "$x7" runbook/phase-1/s7h-lane-completes.md 'CP7H-END-OF-BLOCK reached'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7h could not be read (rc=$rc)"; exit 1; }
read -r nb c_fix c_asg _ _ _ <<< "$c7"
[ "$nb" -ge 1 ] && [ "$c_fix" = "0" ] && [ "$c_asg" = "1" ] || { echo "FAIL: Checkpoint 7h's code lines: $c_fix FIX_ITEM and $c_asg fixture assignments (blocks: $nb) — it still reads the environment"; exit 1; }
# SYNTHETIC CONTROL: a block that still reads $FIX_ITEM must count it.
bt=$(printf '%s' '`' '`' '`')
printf '%s\n' "$bt""bash" 'test -n "$FIX_ITEM" || exit 1' 'echo "CP7X-END-OF-BLOCK reached"' "$bt" > "$tdir/x7-red.md" || { echo "FAIL: could not write the re-point control"; exit 1; }
c7=$(python -c "$x7" "$tdir/x7-red.md" 'CP7X-END-OF-BLOCK reached'); rc=$?
read -r _ c_fix _ _ _ _ <<< "$c7"
[ "$rc" -eq 0 ] && [ "$c_fix" = "1" ] || { echo "FAIL: the re-point reader did not count a \$FIX_ITEM read (rc=$rc: $c7) — it is vacuous"; exit 1; }

# ---- (6) THE PIN, THE INSTRUMENT RECORD, THE SPEND GATE, 7c's SPLIT RUN AND THE PROBES AGREE; (g) IS ACCEPTED ONLY
# ---- WITH BOTH PASSED.
inst=$(tr -d '\r\n' < spikes/results/instrument-checks/RUNBOOK-instrument.txt)
ver=${inst%% *}
pin=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9]+\.[0-9]+\.[0-9]+)";$/\1/p' crates/rk-adapter-claude/src/config.rs)
[ -n "$ver" ] && [ "$ver" = "$pin" ] || { echo "FAIL: the instrument record says '$ver' and TESTED_CLI_VERSION says '$pin' (Halt 2; D21-42 16)"; exit 1; }
# The adapter pin equals S-BIN's pinned cli_version (s8i §8i.5; D21-43 N22 froze the CLI from S-BIN on).
sbin=$(grep -lE '^## F[0-9]+ — S-BIN' spikes/F*-DRAFT.md); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$sbin" | grep -c .)" -eq 1 ] || { echo "FAIL: not exactly one S-BIN finding draft (rc=$rc) — the pin has no S-BIN version to equal"; exit 1; }
sbin_pins() { sed -n '/^<!-- s-bin-pins$/,/^-->$/p' "$1" | sed -n 's/^cli_version=//p'; }
sbin_cli=$(sbin_pins "$sbin")
[ -n "$sbin_cli" ] && [ "$sbin_cli" = "$pin" ] || { echo "FAIL: TESTED_CLI_VERSION is '$pin' and S-BIN pinned cli_version '$sbin_cli' — the freeze failed between S-BIN and the pin move (Halt 2)"; exit 1; }
printf '%s\n' '<!-- s-bin-pins' 'cli_version=9.9.9' '-->' > "$tdir/sbin-ctl.md" || { echo "FAIL: could not write the S-BIN pins control"; exit 1; }
[ "$(sbin_pins "$tdir/sbin-ctl.md")" = "9.9.9" ] || { echo "FAIL: the S-BIN pins reader did not read a planted cli_version — it is vacuous"; exit 1; }
grep -qxF "    assert_eq!(TESTED_CLI_VERSION, \"$pin\");" crates/rk-adapter-claude/tests/it/config.rs || { echo "FAIL: the literal pin test does not carry $pin"; exit 1; }
out=$(wrapped cargo test -p rk-adapter-claude --test it -- config::tested_cli_version_is_the_pin_as_a_config_value --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the pin test did not pass exactly once (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'test config::tested_cli_version_is_the_pin_as_a_config_value ... ok' || { echo "FAIL: the pin test did not report ok by name"; exit 1; }
# D21-48 X12 (2026-09-24): the pin-move commit carried the argv spelling change (§8j.6 step 1; s8h §8h.2). The table holds
# exactly one row, keyed by the pin; LIVE_WARRANTED is empty; the pin's --help capture lists the row's spelling; both
# golden argv fixtures carry it on --permission-mode, while their --agents JSON keeps the card's `default`. Each
# reader is proved on a synthetic input first.
spell_rows() {  # spell_rows <command.rs>: "row <version> <spelling>" per table row, "live <version>" per exemption; code lines only
  python - "$1" <<'PY'
import re, sys
sys.stdout.reconfigure(newline="\n")  # LF on Windows: a CR would ride into the tr/grep -x reads below
code = "".join(l for l in open(sys.argv[1], encoding="utf-8") if not l.lstrip().startswith("//"))
m = re.search(r'^pub\(crate\) const PERMISSION_MODE_ARGV: &\[\(&str, &str\)\] = &\[\n(.*?)^\];', code, re.S | re.M)
w = re.search(r'^pub\(crate\) const LIVE_WARRANTED: &\[&str\] = &\[(.*?)\];', code, re.S | re.M)
if not m or not w:
    sys.exit(1)
for v, s in re.findall(r'^\s*\("([0-9.]+)", "([A-Za-z]+)"\),\s*$', m.group(1), re.M):
    print("row", v, s)
for v in re.findall(r'"([0-9.]+)"', w.group(1)):
    print("live", v)
PY
}
hchoices() {  # hchoices <file>: the --permission-mode choices of a captured `claude --help`, one per line; rc 1 if none
  python - "$1" <<'PY'
import re, sys
sys.stdout.reconfigure(newline="\n")  # LF on Windows: a CR would ride into the tr/grep -x reads below
t = open(sys.argv[1], encoding="utf-8").read()
m = re.search(r'--permission-mode\b[^(]*\(choices: ([^)]*)\)', t)
if not m:
    sys.exit(1)
for c in re.findall(r'"([^"]+)"', m.group(1)):
    print(c)
PY
}
gargv() {  # gargv <argv.json>: the --permission-mode value, then each --agents entry's permissionMode
  python - "$1" <<'PY'
import json, sys
sys.stdout.reconfigure(newline="\n")  # LF on Windows: a CR would ride into the tr/grep -x reads below
a = json.load(open(sys.argv[1], encoding="utf-8"))
print(a[a.index("--permission-mode") + 1])
for v in json.loads(a[a.index("--agents") + 1]).values():
    print(v.get("permissionMode"))
PY
}
printf '%s\n' 'pub(crate) const PERMISSION_MODE_ARGV: &[(&str, &str)] = &[' '    ("1.0.0", "default"),' '//    ("1.0.1", "manual"),' '    ("1.0.2", "manual"),' '];' 'pub(crate) const LIVE_WARRANTED: &[&str] = &["1.0.0"];' > "$tdir/spell-ctl.rs" || { echo "FAIL: could not write the spelling-table control"; exit 1; }
[ "$(spell_rows "$tdir/spell-ctl.rs" | tr '\n' ';')" = "row 1.0.0 default;row 1.0.2 manual;live 1.0.0;" ] || { echo "FAIL: CONTROL — the spelling-table reader misread a synthetic table (a // row must be dropped, the live entry kept)"; exit 1; }
printf '  --permission-mode <mode>  Permission mode (choices: "acceptEdits", "default")\n' > "$tdir/help-ctl.txt" || { echo "FAIL: could not write the help control"; exit 1; }
[ "$(hchoices "$tdir/help-ctl.txt" | tr '\n' ';')" = "acceptEdits;default;" ] || { echo "FAIL: CONTROL — hchoices misread a synthetic --help capture"; exit 1; }
printf '%s\n' '["--agents", "{\"x\":{\"permissionMode\":\"default\"}}", "--permission-mode", "manual"]' > "$tdir/argv-ctl.json" || { echo "FAIL: could not write the argv control"; exit 1; }
[ "$(gargv "$tdir/argv-ctl.json" | tr '\n' ';')" = "manual;default;" ] || { echo "FAIL: CONTROL — gargv misread a synthetic argv"; exit 1; }
sr=$(spell_rows crates/rk-adapter-claude/src/command.rs); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: command.rs declares no PERMISSION_MODE_ARGV / LIVE_WARRANTED in the form s8h §8h.2 states (D21-48 X12)"; exit 1; }
[ "$(printf '%s\n' "$sr" | grep -c '^row ')" -eq 1 ] && printf '%s\n' "$sr" | grep -qE "^row $pin (manual|default)\$" || { echo "FAIL: PERMISSION_MODE_ARGV does not hold exactly one row keyed by the pin $pin — the pin-move commit did not carry X12's change: $sr"; exit 1; }
if printf '%s\n' "$sr" | grep -q '^live '; then echo "FAIL: LIVE_WARRANTED is not empty after the pin move: $sr"; exit 1; fi
spell=$(printf '%s\n' "$sr" | sed -n 's/^row [0-9.]* //p')
hf="crates/rk-adapter-claude/tests/fixtures/cli-help/$pin.permission-mode.txt"
test -s "$hf" || { echo "FAIL: no --help capture for the pin at $hf (s8h §8h.2)"; exit 1; }
hchoices "$hf" | grep -qxF "$spell" || { echo "FAIL: $hf's --permission-mode choices do not list the row's spelling '$spell' (X12)"; exit 1; }
for g in golden-default golden-static; do
  gv=$(gargv "crates/rk-adapter-claude/tests/fixtures/argv/$g.argv.json"); rc=$?
  [ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$gv" | tr '\n' ';')" = "$spell;default;" ] || { echo "FAIL: $g.argv.json reads '$(printf '%s' "$gv" | tr '\n' ' ')' (rc=$rc); X12 wants --permission-mode $spell and the --agents JSON at default"; exit 1; }
done
nd_check "(6)" || exit 1
pm=$(grep -E '^pin-move [0-9.]+ ([0-9.]+|unchanged) [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$pm" || { echo "FAIL: no pin-move line in $ev"; exit 1; }
read -r _ pm_old pm_new _ <<< "$pm"
[ "$pm_new" = "$pin" ] || { [ "$pm_new" = "unchanged" ] && [ "$pm_old" = "$pin" ]; } || { echo "FAIL: the recorded pin move ($pm_old -> $pm_new) does not end at the pin $pin"; exit 1; }
# THE SPEND GATE (D21-40 decision 2; review finding 7): the fixture proofs were recorded over a commit whose code
# carries the pin, every mutant was recorded killed, and RT-57's record is accepted. (7) ties the proofs' code to CI's.
fp=$(grep -E '^fixture-proofs [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$fp" || { echo "FAIL: no fixture-proofs line in $ev — no paid step had a recorded gate (s8j §8j.6 step 1b)"; exit 1; }
read -r _ fp_sha _ fp_date <<< "$fp"
git cat-file -e "$fp_sha^{commit}" || { echo "FAIL: the fixture-proofs sha $fp_sha is not a commit here"; exit 1; }
git merge-base --is-ancestor "$fp_sha" HEAD || { echo "FAIL: the fixture-proofs sha is not an ancestor of HEAD"; exit 1; }
fpin=$(git show "$fp_sha:crates/rk-adapter-claude/src/config.rs" | sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9]+\.[0-9]+\.[0-9]+)";$/\1/p')
[ "$fpin" = "$pin" ] || { echo "FAIL: the fixture proofs ran at pin '$fpin', not $pin — they were not re-run after the pin move"; exit 1; }
S8J_MUTANTS="M-seed-record M-land-empty M-fixture-toolchain M-skip-empty-stage-trailer M-spec-digest-pre-render M-worktree-under-git M-attach-dropped M-hold-ignored M-hold-spent-refires M-hold-unordered-accepted"
for m in $S8J_MUTANTS; do grep -qE "^mutant $m killed [0-9]{4}-[0-9]{2}-[0-9]{2}\$" "$ev" || { echo "FAIL: no 'mutant $m killed' line in $ev — a survivor blocks (s8j §8j.4)"; exit 1; }; done
rt=$(sed -nE 's/^record-rt57 ([0-9]{4})$/\1/p' "$ev" | tail -1)
rtf=$(ls references/adr/"$rt"-*.md 2>/dev/null)
[ -n "$rt" ] && [ -f "$rtf" ] && grep -qxF 'status: accepted' "$rtf" || { echo "FAIL: RT-57's record (ADR-${rt:-none}, s8h's Record B) is not accepted (D21-42 19c)"; exit 1; }
# 7c's suite portion ran outside wrap over the proofs' code (D21-47's 7c split).
cs=$(grep -E '^checkpoint-7c-suite [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$cs" || { echo "FAIL: no checkpoint-7c-suite line — 7c's suite portion is unrecorded (D21-47)"; exit 1; }
read -r _ cs_sha _ _ <<< "$cs"
git diff --quiet "$fp_sha" "$cs_sha" -- $CODE_PATHS || { echo "FAIL: 7c's suite portion ran over other code than the fixture proofs"; exit 1; }
# Each paid run is recorded at the pin, after the proofs, with its launch record on disk, rc 0, and a HEAD whose code
# is the proofs' code. The record's name carries its mode (<UTC>-<mode>.txt, s8h §8h.7 step 8); RC= and HEAD= are its
# KEY=value members — a dated correction here if s8h spells them otherwise.
for pm2 in "checkpoint-7c:wrap" "probe live::live_denial_channel_preprobe:probe" "probe live::live_confinement_and_denial_probe:probe" "probe live::live_init_equality_probe:probe"; do   # probe 0 added 2026-09-24 (final consistency pass; s8h §8h.11)
  p=${pm2%:*}; md=${pm2##*:}
  ln=$(grep -E "^$p $pin pass [0-9]{4}-[0-9]{2}-[0-9]{2} [^ ]+\$" "$ev" | tail -1)
  test -n "$ln" || { echo "FAIL: no passing '$p' recorded at the pin $pin — the (g) record may not flip (D21-41 (g); D21-43 N28)"; exit 1; }
  lr=${ln##* }; ld=${ln% *}; ld=${ld##* }
  [ "${ld//-/}" -ge "${fp_date//-/}" ] || { echo "FAIL: '$p' ran on $ld, before the fixture proofs were recorded on $fp_date (D21-40 decision 2)"; exit 1; }
  case "$lr" in *-"$md".txt) : ;; *) echo "FAIL: '$p' names launch record $lr, which is not a $md launch"; exit 1;; esac
  test -s "$launches/$lr" || { echo "FAIL: '$p' names launch record $lr, which is not under $launches — it did not go through the launcher (D21-42 17a)"; exit 1; }
  grep -qxE 'RC=0' "$launches/$lr" || { echo "FAIL: launch record $lr does not carry RC=0"; exit 1; }
  lh=$(sed -nE 's/^HEAD=([0-9a-f]{40})$/\1/p' "$launches/$lr" | tail -1)
  [ -n "$lh" ] && git merge-base --is-ancestor "$fp_sha" "$lh" && git diff --quiet "$fp_sha" "$lh" -- $CODE_PATHS || { echo "FAIL: launch record $lr's HEAD '$lh' does not carry the fixture proofs' code"; exit 1; }
done
rg_id=$(sed -nE 's/^record-g ([0-9]{4})$/\1/p' "$ev" | tail -1)
test -n "$rg_id" || { echo "FAIL: no record-g line names the (g) record"; exit 1; }
rg_file=$(ls references/adr/"$rg_id"-*.md 2>/dev/null)
test -n "$rg_file" && test -f "$rg_file" || { echo "FAIL: ADR-$rg_id's file does not exist"; exit 1; }
grep -qxF 'status: accepted' "$rg_file" || { echo "FAIL: ADR-$rg_id is not accepted — both probes are recorded as passing, so the operator's dated flip is what is missing"; exit 1; }

# ---- (7) CI: GREEN ON THE RECORDED RUN, EVERY JOB, BOTH LEGS; THE CODE UNCHANGED SINCE; THE pnpm STEP PRESENT.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in $ev"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
[ "$fx_frozen" = "$ci_sha" ] || { echo "FAIL: the fix item's frozen sha $fx_frozen is not the CI-green sha $ci_sha"; exit 1; }
grep -qxF "story-item rk-story-harden-spike-pack $ci_sha" "$ev" || { echo "FAIL: the story item's frozen sha is not the CI-green sha"; exit 1; }
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- $CODE_PATHS || { echo "FAIL: code or registry changed since CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
git diff --quiet "$fp_sha" "$ci_sha" -- $CODE_PATHS || { echo "FAIL: the fixture proofs ran over other code than CI run $run_id — re-run §8j.6 step 1b over the frozen code"; exit 1; }
cy=$(grep -nE '^[[:space:]]+pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project$' .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ci.yml has no pnpm install step on a code line (D21-42 14a)"; exit 1; }
ct=$(grep -nE '^[[:space:]]+run: cargo test --workspace$' .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ci.yml's test gate line was not found — the ordering check would compare nothing"; exit 1; }
[ "${cy%%:*}" -lt "${ct%%:*}" ] || { echo "FAIL: the pnpm step comes after cargo test — the node suite would still fail at its preflight"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
# SYNTHETIC CONTROL first: a run missing the windows leg must be refused, or the checker is vacuous.
v=$(python -c "$cichk" "$ci_sha" <<< "{\"conclusion\":\"success\",\"headSha\":\"$ci_sha\",\"headBranch\":\"build/phase-0-to-1\",\"jobs\":[{\"name\":\"gates (ubuntu-latest)\",\"conclusion\":\"success\"},{\"name\":\"no egress (AC-498)\",\"conclusion\":\"success\"},{\"name\":\"structural assertions\",\"conclusion\":\"success\"}]}"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the CI checker accepted a run with no windows leg (rc=$rc: $v) — it is vacuous"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }

# ---- (8) s8's AMENDMENTS LANDED (§8j.8): header, no BIND:, s8b's form and selection verbatim, walks, launches.
s8chk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
b8=open(sys.argv[2],encoding="utf-8").read()
f=chr(96)*3
def code_lines(text, marker=None):
    bl=re.findall(f+"bash\n(.*?)"+f,text,re.S)
    if marker is not None: bl=[b for b in bl if marker in b]
    return [l.strip() for b in bl for l in b.split("\n") if l.strip() and not l.strip().startswith("#")]
bad=[]
if "| `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — |" not in t: bad.append("header row")
if "BIND:" in t: bad.append("an unbound BIND: slot")
code=code_lines(t); cs=set(code); jc="\n".join(code)
b8c=code_lines(b8,"CP8B-END-OF-BLOCK reached")
keys=("EXCL=","D2=","two_walk() {","K_TESTS=","^lattice-shipped: packs=","^rigor_monotonicity: catalog=","^ac527: catalog=")
for k in keys:
    ls=[l for l in b8c if l.startswith(k) or (k.startswith("^") and k in l)]
    if not ls: bad.append("s8b carries no "+k+" line to copy")
    for l in ls:
        if l not in cs: bad.append("s8b line not copied verbatim (A5.1/A5.2): "+l[:70])
plant=[l for l in b8c if l.startswith("seen=$(grep -rlF")]
if len(plant)!=1: bad.append("s8b has %d plant-control walks, expected 1" % len(plant))
if "git ls-files --error-unmatch" in jc: bad.append("(1b) still asserts tree membership")
if "findings\\.none\\(evidence in \\[Reproduced, MutationSurvived\\]\\)" in jc: bad.append("the unquoted-only d2 pattern survives (A5.2)")
for l in code:
    if l in plant: continue
    if re.search(r"grep\s+-r\S*\s", l) and re.search(r"\s\.redkiln/?(\s|\)|;|$)", l) and "--exclude-dir=worktrees" not in l: bad.append("a walk over .redkiln/ without --exclude-dir=worktrees (A5.5): "+l[:60])
if re.search(r"grep\s+-r\S*\s[^\n]*four questions on twenty items", jc): bad.append("the doc screen still walks the tree (A5.3)")
for need in ("git grep -n","%(trailers:key=Story,valueonly=true)","stage_order","verify_completed","VC_ORIGIN_KEY","observed_model","artifact.write","git cat-file -e","dispatch_completed","gh run view","* text=auto eol=lf","STORE_BAK","${rebuilt% *}","tools/live-run.sh run","ac184_rebuild_differential_over_anchored_tier_one_rows","taskkill //T //F","ITEM=\"$FIX_ITEM\""):
    if need not in jc: bad.append("absent from code lines: "+need)
if "Those nineteen are not read, they are run." not in t: bad.append("the bullet count sentence")
if "After Checkpoint 8 passes: AC-114" not in t: bad.append("A9: the post-Checkpoint-8 step is unnamed")
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$s8chk" runbook/phase-1/s8-exit-criterion.md runbook/phase-1/s8b-catalog-quality.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: s8's amendments did not land: $v"; exit 1; }
# Golden constraint 3 in every s8 block: no use of ./target/debug/redkiln on a code line before a guarded build in the
# same block (review finding 17: `test -x` was the only shape read, and (6)'s second block has none).
txchk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]
blocks=re.findall(f+"bash\n(.*?)"+f,t,re.S)
for i,b in enumerate(blocks):
    built=False
    for ln in b.split("\n"):
        s=ln.strip()
        if s.startswith("#"): continue
        if "cargo build --workspace" in s and "rc=$?" in s: built=True
        if "./target/debug/redkiln" in s and not built: bad.append("block %d: the binary before a build: %s" % (i, s[:60]))
print("; ".join(bad) if bad else "ok %d blocks" % len(blocks))
sys.exit(1 if bad or not blocks else 0)'
v=$(python -c "$txchk" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an s8 block runs the binary before building it: $v"; exit 1; }
# D21-42 17a and D21-47 in every s8 block: no bare `redkiln run` or `redkiln answer` on a code line; at least four
# launcher runs (A1, A3); and every launcher run follows, in its own block, the prespend definition and a call of it.
rnchk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]; n=0
for i,b in enumerate(re.findall(f+"bash\n(.*?)"+f,t,re.S)):
    defd=False; called=False
    for ln in b.split("\n"):
        s=ln.strip()
        if s.startswith("#"): continue
        if s.startswith("prespend() {"): defd=True
        if s.startswith("why=$(prespend \""): called=True
        if re.search(r"(target/debug/redkiln(\.exe)?|\$\{?RK\}?)\"?\s+(run|answer)(\s|$)", s): bad.append("block %d: a bare run or answer: %s" % (i, s[:80]))
        if s.startswith("tools/live-run.sh run "):
            n+=1
            if not (defd and called): bad.append("block %d: a launcher run with no pre-spend checks before it (D21-47): %s" % (i, s[:60]))
if n < int(sys.argv[2]): bad.append("%d launcher run lines, expected at least %s" % (n, sys.argv[2]))
print("; ".join(bad) if bad else "ok %d launcher runs" % n)
sys.exit(1 if bad else 0)'
v=$(python -c "$rnchk" runbook/phase-1/s8-exit-criterion.md 4); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an s8 run site does not go through tools/live-run.sh after the pre-spend checks: $v"; exit 1; }
# INDEX.md's Step 8 row and s8's header open from Checkpoint 8j (A0, made at insertion; review finding 27). The row is
# found by its stage cell, not its number: the insertion renumbered it 19 -> 30.
r19=$(grep -E '^\| [0-9]+ \| Step 8 \|' runbook/INDEX.md)
[ "$(printf '%s\n' "$r19" | grep -c .)" -eq 1 ] || { echo "FAIL: INDEX.md has not exactly one Step 8 row"; exit 1; }
printf '%s\n' "$r19" | grep -qF '| Checkpoint 8j' || { echo "FAIL: INDEX.md's Step 8 row's entry gate is not Checkpoint 8j (A0)"; exit 1; }
grep -qxF '| `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — |' runbook/phase-1/s8-exit-criterion.md || { echo "FAIL: s8's header row does not open from Checkpoint 8j (A0)"; exit 1; }
# SYNTHETIC CONTROLS: a build-first violation, an unskipped walk, a bare run, a bare answer and an unchecked launch
# must each be refused; the correct forms must pass.
bt=$(printf '%s' '`' '`' '`')
printf '%s\n' "$bt""bash" 'test -x ./target/debug/redkiln || exit 1' "$bt" > "$tdir/s8-syn-red.md" || { echo "FAIL: could not write the red control"; exit 1; }
printf '%s\n' "$bt""bash" 'out=$(cargo build --workspace 2>&1); rc=$?' 'test -x ./target/debug/redkiln || exit 1' "$bt" > "$tdir/s8-syn-green.md" || { echo "FAIL: could not write the green control"; exit 1; }
v=$(python -c "$txchk" "$tdir/s8-syn-red.md"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the build-first checker accepted a test -x with no build before it (rc=$rc: $v) — it is vacuous"; exit 1; }
v=$(python -c "$txchk" "$tdir/s8-syn-green.md"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the build-first checker refused a correctly ordered block (rc=$rc: $v) — it cannot pass"; exit 1; }
# The run and answer controls take the verb from a variable, so no code line of this block spells a launch.
m=run; a=answer
printf '%s\n' "$bt""bash" "./target/debug/redkiln $m \"\$X\" --lane fix || exit 1" "$bt" > "$tdir/run-red.md" || { echo "FAIL: could not write the bare-run control"; exit 1; }
printf '%s\n' "$bt""bash" "./target/debug/redkiln $a \"\$X\" approve || exit 1" "$bt" > "$tdir/answer-red.md" || { echo "FAIL: could not write the bare-answer control"; exit 1; }
printf '%s\n' "$bt""bash" "tools/live-run.sh $m \"\$X\" --lane fix || exit 1" "$bt" > "$tdir/run-unchecked.md" || { echo "FAIL: could not write the unchecked-launch control"; exit 1; }
printf '%s\n' "$bt""bash" 'prespend() {' '}' 'why=$(prespend "$X"); rc=$?' "tools/live-run.sh $m \"\$X\" --lane fix || exit 1" "$bt" > "$tdir/run-green.md" || { echo "FAIL: could not write the launcher-run control"; exit 1; }
printf '%s\n' "$bt""bash" 'raw=$(grep -rnE x crates/ .redkiln/); rc=$?' "$bt" > "$tdir/walk-red.md" || { echo "FAIL: could not write the walk control"; exit 1; }
v=$(python -c "$s8chk" "$tdir/walk-red.md" runbook/phase-1/s8b-catalog-quality.md); rc=$?
[ "$rc" -eq 1 ] && printf '%s\n' "$v" | grep -qF 'without --exclude-dir=worktrees' || { echo "FAIL: the amendment checker accepted a walk over .redkiln/ with no worktrees skip (rc=$rc) — the A5.5 check is vacuous"; exit 1; }
v=$(python -c "$rnchk" "$tdir/run-red.md" 0); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the run-site checker accepted a bare run of the binary (rc=$rc: $v) — it is vacuous"; exit 1; }
v=$(python -c "$rnchk" "$tdir/answer-red.md" 0); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the run-site checker accepted a bare redkiln answer (rc=$rc: $v) — the D21-46 half is vacuous"; exit 1; }
v=$(python -c "$rnchk" "$tdir/run-unchecked.md" 1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the run-site checker accepted a launch with no pre-spend checks (rc=$rc: $v) — the D21-47 half is vacuous"; exit 1; }
v=$(python -c "$rnchk" "$tdir/run-green.md" 1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the run-site checker refused a checked launcher run (rc=$rc: $v) — it cannot pass"; exit 1; }
# Every test name s8 selects through a *_TESTS variable exists.
nm='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
for v in re.findall(r"^[A-Z_]*TESTS=\"([^\"]*)\"",t,re.M):
    for n in v.split(): print(n)'
names=$(python -c "$nm" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$names" ] || { echo "FAIL: s8 selects no test through a *_TESTS variable (rc=$rc) — A5.1 did not land"; exit 1; }
lst=$(wrapped cargo test --workspace -- --list 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --list rc=$rc"; exit 1; }
for n in $names; do printf '%s\n' "$lst" | grep -qxF "$n: test" || { echo "FAIL: s8 selects '$n', which no test binary lists"; exit 1; }; done
nd_check "(8)" || exit 1

# ---- (9) THE THREE RUNBOOK CHECKERS, from the repo root: pass, and zero flagged.
out=$(python runbook/tools/check-checkpoints.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE 'flagged 0;' || { echo "FAIL: check-checkpoints rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }
out=$(python runbook/tools/check-refs.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE '^unresolved bare section refs: 0 ' || { echo "FAIL: check-refs rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }
out=$(python runbook/tools/check-bash-syntax.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE ' 0 failed$' || { echo "FAIL: check-bash-syntax rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }

# ---- (10) RE-RUN, WHOLE: THE ENTRY CHAIN ONCE, LINK BY LINK, AND EVERY OLDER CHECKPOINT AN s8 STAGE MADE PROVISIONAL.
# D21-46's ONE RE-RUN CONVENTION: RK_CP_NESTED=1 means "running as another checkpoint's entry gate: skip your own
# nested whole re-runs". One run of 8i therefore no longer carries the chain, and this block runs each link ONCE,
# directly, with RK_CP_NESTED=1: 8a, 8b, 8c, 8d, 8d-ii (D21-46 R1), 8e, 8f, 8g, 8h and 8i, the entry gate. 7h and 7i run
# directly as well, because §8j.1(c) edits both. THE LAST GATE BEFORE SPEND (consistency pass 2026-09-24, lead
# decision): every older checkpoint an s8 stage made provisional runs whole here too, nested and under the same guards:
# 5, 6, 7a, 7b, 7b-ii, 7d, 7f and 7g. 7c is §8j.6's live split run (D21-43 N28; D21-47); 7e cannot be re-run whole.
# 7f runs by the fixture route §8j.8 A10 writes into it: harness_binary at rk-fixture-child-claude in its clone, a
# fixture launch record from a file, its spawn markers counted. When THIS block is itself run with RK_CP_NESTED=1 (as
# s8's entry gate), it re-runs nothing.
# NO ACCIDENTAL DISPATCH (D21-47's wrapper for replays):
#   · PREVENTION. Each block runs with REDKILN_LAUNCH_RECORD, FIX_ITEM, STORY_ITEM and ITEM unset, so s8h's gate
#     refuses a real-harness `run` or `answer` "before `reconcile_started` is appended and before any worktree
#     operation" (s8h §8h.7); with CLAUDE_CONFIG_DIR at the empty trap; and with the compiled fake `claude` first on
#     PATH (runbook/tools/nodispatch.sh), every other PATH directory holding a claude removed.
#   · SCREEN. Before anything runs, each block's code lines are refused if they carry a shape that EXECUTES what the
#     prevention exists for: the launcher in a running mode (run, probe, answer, wrap), a launch record supplied or
#     exported, a push, or a test run with the live feature in any spelling (`live`, `"live"`, `foo,live`,
#     `rk-adapter-claude/live`, `--all-features`), without which no live probe compiles. Not refused: listing ignored
#     tests (`--list`), linting the live feature, the launcher's `--check` modes, a probe's name in a string, an ignored
#     test run WITHOUT the live feature, an `env -u` of the launch record, and a wrapper's PLANTED control, whose
#     bare-word value (`planted`: no path, so it names no launch record) reaches only
#     `sh -c 'printf …'` or `sh -c 'echo …'`. Every other admitted line is listed, byte for byte and with its reason,
#     in runbook/phase-1/s8j-screen-exempt.txt: 7f's fixture-route line (A10) always, and any line a chain block runs
#     against a sandbox the stage that owns it describes (Checkpoint 8h's three, listed in A10). The screen is a FLOOR: a block that assembles another at run
#     time is screened by its own parent, and the trap and (12) cover it.
#   · DETECTION. The trap and the fake's log below, and (12)'s store, ref, launch-record and transcript readings.
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8J (10) SKIPPED: this block is itself an entry-gate re-run (RK_CP_NESTED=1); the chain ran at Checkpoint 8j's own close"
else
  xblk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
hits=[b for b in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in b]
if not hits: sys.exit(3)
open(sys.argv[3],"w",encoding="utf-8",newline="\n").write(hits[-1])'
  RUN_RE='(tools/live-run\.sh|\$L|\$\{L\})"?[[:space:]]+(run|probe|answer|wrap)([[:space:]]|$)|REDKILN_LAUNCH_RECORD[=][^[:space:];)]|(export|declare[[:space:]]+-x)[[:space:]]+([A-Za-z_][A-Za-z0-9_]*[[:space:]]+)*REDKILN_LAUNCH_RECORD([[:space:]=;]|$)|git[[:space:]]+push([[:space:]]|$)'
  TESTRUN_RE="cargo[[:space:]]+test[^;|]*((--features|-F)[[:space:]=]+[\"']?([A-Za-z0-9_-]+/)?([A-Za-z0-9_/-]+,)*live([^A-Za-z0-9_-]|\$)|--all-features)"
  EXEMPT=runbook/phase-1/s8j-screen-exempt.txt
  test -f "$EXEMPT" || { echo "FAIL: $EXEMPT is absent — 7f's fixture-route line (s8j §8j.8 A10) cannot pass the screen"; exit 1; }
  if od -c "$EXEMPT" | grep -qF '\r'; then echo "FAIL: $EXEMPT carries a CR byte — no exempt line would ever match"; exit 1; fi
  screen() {   # $1 = block file, $2 = its stem: prints each refused line, nothing when the block is clean
    exf="$tdir/exempt-$2.txt"
    awk -F '\t' -v s="$2" '$1==s && $3!="" {print $2}' "$EXEMPT" > "$exf" || { echo "the exemption list could not be read"; return 0; }
    c=$(grep -vE '^[[:space:]]*#' "$1" | sed -E 's/^[[:space:]]+//' | grep -vxF -f "$exf" | sed -E "s/REDKILN_LAUNCH_RECORD=[A-Za-z0-9_-]+([;[:space:]].* sh -c '(printf|echo) )/PLANTED_CONTROL\1/")
    printf '%s\n' "$c" | grep -nE "$RUN_RE"; printf '%s\n' "$c" | grep -nE "$TESTRUN_RE" | grep -v -- '--list'
  }
  # SYNTHETIC CONTROLS, both ways. Each is assembled by printf, so no code line of this block spells what it screens.
  for s in "$(printf 'bash "$L" %s live::x' probe)" "$(printf 'tools/live-run.sh %s "$ITEM" approve' answer)" "$(printf 'tools/live-run.sh %s -- bash x.sh' wrap)" "$(printf 'REDKILN_%s=/x/y.txt cargo test' LAUNCH_RECORD)" "$(printf 'git %s origin HEAD' push)" "$(printf 'cargo test -p x --features %s --test it -- %s live::x' live --ignored)" "$(printf 'cargo test -p x --features rk-adapter-claude/%s -- live::x' live)" "$(printf 'cargo test -p x --features "%s" -- live::x' live)" "$(printf 'cargo test -p x --features=foo,%s -- live::x' live)" "$(printf 'export REDKILN_%s' LAUNCH_RECORD)" "$(printf 'declare -x REDKILN_%s=/x' LAUNCH_RECORD)" "$(printf 'REDKILN_%s=planted cargo test -p x' LAUNCH_RECORD)" "$(printf "REDKILN_%s=/x/r.txt w sh -c 'printf x'" LAUNCH_RECORD)"; do
    printf '%s\n' "$s" > "$tdir/screen-ctl.sh" || { echo "FAIL: could not write a screen control"; exit 1; }
    [ -n "$(screen "$tdir/screen-ctl.sh" screen-ctl)" ] || { echo "FAIL: the dispatch screen does not refuse '$s' — it is vacuous for that shape"; exit 1; }
  done
  # Checkpoint 8h's own shapes list, compile, lint, check or name, and run nothing; 7h's runs a non-live ignored test;
  # the wrappers' env -u and planted controls execute nothing but printf. Each must pass.
  for s in "$(printf 'out=$(cargo test -p x --features %s --test it -- --list %s 2>&1); rc=$?' live --ignored)" "$(printf 'out=$(cargo clippy -p x --all-targets --features %s 2>&1); rc=$?' live)" "$(printf 'out=$("${clean[@]}" bash "$L" --check %s -- true 2>&1); rc=$?' wrap)" "$(printf '[ -z "${REDKILN_%s:-}" ] || exit 1' LAUNCH_RECORD)" "$(printf 'n=$(grep -c %s x.rs)' live_dispatch_probe)" "$(printf 'out=$(cargo test -p x --test it -- y %s 2>&1); rc=$?' --ignored)" "$(printf 'out=$(env -u REDKILN_%s bash x.sh 2>&1); rc=$?' LAUNCH_RECORD)" "$(printf 'out=$(cargo test -p x --features corpus -- y 2>&1); rc=$?')" "$(printf "got=\$(REDKILN_%s=planted w sh -c 'printf %%s \"\${REDKILN_LAUNCH_RECORD-unset}\"'); rc=\$?" LAUNCH_RECORD)" "$(printf "got=\$(REDKILN_%s=s8dii-control wrap sh -c 'echo \"\${REDKILN_LAUNCH_RECORD-unset}\"')" LAUNCH_RECORD)"; do
    printf '%s\n' "$s" > "$tdir/screen-ctl.sh" || { echo "FAIL: could not write a screen control"; exit 1; }
    [ -z "$(screen "$tdir/screen-ctl.sh" screen-ctl)" ] || { echo "FAIL: the dispatch screen refuses '$s', which runs nothing — a checkpoint that carries it could never be re-run"; exit 1; }
  done
  # The exemption list: every entry is <stem><TAB><line><TAB><reason>, reasoned; exactly one is 7f's, of A10's shape;
  # it is admitted verbatim and refused with one byte changed. After extraction, every entry must still match a code
  # line of its stem's block, so no stale entry lingers.
  nun=$(awk -F '\t' 'NF && (NF < 3 || $3 == "")' "$EXEMPT" | grep -c .)
  [ "$nun" -eq 0 ] || { echo "FAIL: $nun entries of $EXEMPT carry no reason — an exemption is a claim, and states why"; exit 1; }
  n7f=$(awk -F '\t' '$1=="s7f-conformance-and-records"' "$EXEMPT" | grep -c .)
  [ "$n7f" -eq 1 ] || { echo "FAIL: $EXEMPT holds $n7f entries for 7f, not the one line A10 writes"; exit 1; }
  exl=$(awk -F '\t' '$1=="s7f-conformance-and-records" {print $2}' "$EXEMPT")
  printf '%s\n' "$exl" | grep -qF 'REDKILN_LAUNCH_RECORD="$s8j_lr"' && printf '%s\n' "$exl" | grep -qF 'RK_FIXTURE_MARKER_DIR="$s8j_mk"' || { echo "FAIL: the exempt line is not A10's fixture-route line: $exl"; exit 1; }
  printf '%s\n' "$exl" > "$tdir/screen-ctl.sh" || { echo "FAIL: could not write the exemption control"; exit 1; }
  [ -z "$(screen "$tdir/screen-ctl.sh" s7f-conformance-and-records)" ] || { echo "FAIL: the screen refuses the exempt 7f line itself"; exit 1; }
  printf '%s\n' "${exl/\$s8j_lr/\$other_lr}" > "$tdir/screen-ctl.sh" || { echo "FAIL: could not write the exemption variant"; exit 1; }
  [ -n "$(screen "$tdir/screen-ctl.sh" s7f-conformance-and-records)" ] || { echo "FAIL: the screen admits a variant of the exempt line — the exemption is not byte-exact"; exit 1; }
  # A premise the chain needs from s8e (cross-file #2): 8a's guard (7) no longer expects the Slice-Verdict lint ARMED.
  a8=$(python -c "$xblk" runbook/phase-1/s8a-records-and-red-checkpoints.md 'echo "CP8A-END-OF-BLOCK reached"' "$tdir/a8-premise.sh"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8a's block could not be extracted (rc=$rc) $a8"; exit 1; }
  a8c=$(grep -vE '^[[:space:]]*#' "$tdir/a8-premise.sh")
  if printf '%s\n' "$a8c" | grep -qF 'ignored, ARMED: Slice-Verdict has 0 producers'; then echo "FAIL: Checkpoint 8a guard (7) still expects the trailer lint ARMED — s8e's dated amendment (cross-file #2) has not landed, and 8a's re-run below would fail"; exit 1; fi
  d2=$(ls runbook/phase-1/s8d-ii-*.md 2>/dev/null)
  [ "$(printf '%s\n' "$d2" | grep -c .)" = "1" ] || { echo "FAIL: not exactly one runbook/phase-1/s8d-ii-*.md ('$d2') — Checkpoint 8d-ii cannot be re-run (D21-46 R1; Halt 8)"; exit 1; }
  d2=$(basename "$d2" .md)
  CHAIN="s8a-records-and-red-checkpoints:CP8A s8b-catalog-quality:CP8B s8c-toolchain-port-and-verbs:CP8C s8d-step-grain-and-resolver:CP8D $d2:CP8D-II s8e-tier-one-and-the-lane:CP8E s8f-landing-and-rendering:CP8F s8g-verify-and-restored-demands:CP8G s8h-cards-and-run-environment:CP8H s8i-s-bin:CP8I s7h-lane-completes:CP7H s7i-cost:CP7I s7d-cli-and-migrate:CP7D s7f-conformance-and-records:CP7F s7g-step-executor:CP7G"
  # Every block is extracted and screened before ANY of them runs.
  for pair in $CHAIN; do
    sf="runbook/phase-1/${pair%%:*}.md"; mk="${pair##*:}-END-OF-BLOCK reached"; blk="$tdir/${pair%%:*}.sh"
    test -f "$sf" || { echo "FAIL: $sf does not exist — its checkpoint cannot be re-run (Halt 8)"; exit 1; }
    python -c "$xblk" "$sf" "echo \"$mk\"" "$blk"; rc=$?
    [ "$rc" -eq 0 ] && test -s "$blk" || { echo "FAIL: no block in $sf echoes '$mk' (rc=$rc)"; exit 1; }
    hit=$(screen "$blk" "${pair%%:*}")
    [ -z "$hit" ] || { echo "FAIL: $sf's checkpoint would launch, supply a launch record, push or run a live test on a code line — this block runs nothing that does:"; printf '%s\n' "$hit"; exit 1; }
  done
  stale=$(awk -F '\t' 'NF {print $1}' "$EXEMPT" | sort -u | while IFS= read -r st; do cst=$(grep -vE '^[[:space:]]*#' "$tdir/$st.sh" 2>/dev/null | sed -E 's/^[[:space:]]+//'); awk -F '\t' -v s="$st" '$1==s {print $2}' "$EXEMPT" | while IFS= read -r el; do printf '%s\n' "$cst" | grep -qxF "$el" || printf '%s: %s\n' "$st" "$el"; done; done)
  [ -z "$stale" ] || { echo "FAIL: exemption entries that match no code line of their block — stale, or their stem is not in the chain:"; printf '%s\n' "$stale"; exit 1; }
  c7f=$(grep -vE '^[[:space:]]*#' "$tdir/s7f-conformance-and-records.sh" | sed -E 's/^[[:space:]]+//')
  printf '%s\n' "$c7f" | grep -qxF "$exl" || { echo "FAIL: Checkpoint 7f's block does not carry the exempt line — A10 did not land, or the exemption is stale"; exit 1; }
  printf '%s\n' "$c7f" | grep -qF 'harness_binary' || { echo "FAIL: Checkpoint 7f's block writes no harness_binary into its clone — A10 did not land"; exit 1; }
  printf '%s\n' "$c7f" | grep -qF '.marker' || { echo "FAIL: Checkpoint 7f's block counts no spawn markers — A10 did not land"; exit 1; }
  # A12 (§8j.8; final consistency pass 2026-09-24): 7d's rk_absent reads with -H, and the old spelling is gone.
  grep -qF 'raw=$(grep -rnHE "$1" $scope --include=*.rs); rc=$?' "$tdir/s7d-cli-and-migrate.sh" && ! grep -qF 'raw=$(grep -rnE "$1" $scope' "$tdir/s7d-cli-and-migrate.sh" || { echo "FAIL: Checkpoint 7d's rk_absent does not read with -H — A12 did not land"; exit 1; }
  # The older checkpoints whose blocks carry no end marker: extracted by a needle unique to the block, then given a
  # sentinel, so "rc 0 and the sentinel last" means the block ran to its end. stem|needle|label.
  n_old=0
  for spec in "s5-workspace-ci-rk-types|FAIL: deny.toml absent|CP5" "s6-store-and-git|the five CI gates, each guarded|CP6" "s7a-ports-and-fakes|FAIL: cargo test -p rk-ports exited|CP7A" "s7b-kernel|spikes/p9/grammar-probe.py absent|CP7B" "s7b-ii-gates|BEFORE: no gate fixtures yet|CP7B-II"; do
    IFS='|' read -r stem needle label <<< "$spec"
    sf="runbook/phase-1/$stem.md"; blk="$tdir/$stem.sh"
    test -f "$sf" || { echo "FAIL: $sf does not exist — its checkpoint cannot be re-run (Halt 8)"; exit 1; }
    python -c "$xblk" "$sf" "$needle" "$blk"; rc=$?
    [ "$rc" -eq 0 ] && test -s "$blk" || { echo "FAIL: no block in $sf carries '$needle' (rc=$rc)"; exit 1; }
    printf '\necho "%s-END-OF-BLOCK reached"\n' "$label" >> "$blk" || { echo "FAIL: could not append $label's sentinel"; exit 1; }
    hit=$(screen "$blk" "$stem")
    [ -z "$hit" ] || { echo "FAIL: $sf's checkpoint would launch, supply a launch record, push or run a live test on a code line:"; printf '%s\n' "$hit"; exit 1; }
    n_old=$((n_old + 1))
  done
  [ "$n_old" -eq 5 ] || { echo "FAIL: $n_old of the five marker-less older checkpoints were extracted"; exit 1; }
  for pair in $CHAIN; do
    sf="runbook/phase-1/${pair%%:*}.md"; mk="${pair##*:}-END-OF-BLOCK reached"; blk="$tdir/${pair%%:*}.sh"
    out=$(wrapped env RK_CP_NESTED=1 bash "$blk" 2>&1); rc=$?
    last=$(printf '%s\n' "$out" | grep . | tail -1)
    [ "$rc" -eq 0 ] && [ "$last" = "$mk" ] || { echo "FAIL: $sf's checkpoint rc=$rc, last line '$last'"; printf '%s\n' "$out" | tail -25; exit 1; }
    nd_check "${pair%%:*}" || exit 1
  done
  for spec in "s5-workspace-ci-rk-types|CP5" "s6-store-and-git|CP6" "s7a-ports-and-fakes|CP7A" "s7b-kernel|CP7B" "s7b-ii-gates|CP7B-II"; do
    IFS='|' read -r stem label <<< "$spec"
    blk="$tdir/$stem.sh"; mk="$label-END-OF-BLOCK reached"
    out=$(wrapped env RK_CP_NESTED=1 bash "$blk" 2>&1); rc=$?
    last=$(printf '%s\n' "$out" | grep . | tail -1)
    [ "$rc" -eq 0 ] && [ "$last" = "$mk" ] || { echo "FAIL: $stem's checkpoint rc=$rc, last line '$last'"; printf '%s\n' "$out" | tail -25; exit 1; }
    nd_check "$stem" || exit 1
  done
fi

# ---- (11) THE WHOLE SUITE, NO FAIL-FAST, WITH A FLOOR; fmt; clippy.
# Expected value, re-derived at stage open from Checkpoint 8i's recorded count, plus this stage's 14 new tests (D21-46:
# each file re-derives from its entry checkpoint's recorded count). The chain, recomputed once (final consistency pass
# 2026-09-24): 740 at 992d257 + 9 (8a) + 25 (8b) + 78 (8c) + 62 (8d) + 50 (8d-ii) + 74 (8e) + 85 (8f) + 66 (8g)
# + 54 (8h) = 1243; 8i adds 6 + N + G (N is the Binary field count s8i's record fixes; G is 3 on s8i's human-gate
# route and 0 on its reviewer route, D21-48 X16), so 1249 is a lower bound and this block's floor is 1263 + N + G.
# (D21-48 2026-09-24: 8h was 51, so 1222 / 1228 / 1242; X12 adds three s8h tests. D21-48 floor propagation,
# 2026-09-24: was 1225 / 1231 / 1245 + N, before s8c 78, s8d 62, s8d-ii 50 and s8f 85.) Edit the literal, by a dated line, to the count PROGRESS.md row 8i records: the guard
# below refuses a literal that row does not carry (review finding 22).
CP8I_PASSED=1249
r8i=$(grep -E '^\| *(Step )?8i\b' runbook/PROGRESS.md)
test -n "$r8i" || { echo "FAIL: PROGRESS.md has no row 8i — Checkpoint 8i's recorded count cannot be read"; exit 1; }
printf '%s\n' "$r8i" | grep -qwF "$CP8I_PASSED" || { echo "FAIL: CP8I_PASSED=$CP8I_PASSED does not appear in PROGRESS.md row 8i — edit the literal to the count that row records"; exit 1; }
[ "$CP8I_PASSED" -ge 1249 ] || { echo "FAIL: CP8I_PASSED=$CP8I_PASSED is below the authoring lower bound 1243 + 6"; exit 1; }
floor=$((CP8I_PASSED + 14))
out=$(wrapped cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|test result:' | tail -30; exit 1; }
nres=$(printf '%s\n' "$out" | grep -cE '^test result: ')
nbad=$(printf '%s\n' "$out" | grep -E '^test result: ' | grep -vc ' 0 failed;')
tot=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="passed;") p+=$(i-1)} END{print p+0}')
[ "$nres" -ge 44 ] && [ "$nbad" = "0" ] && [ "$tot" -ge "$floor" ] || { echo "FAIL: $nres results, $nbad with failures, $tot passed — the floor is 44 results, 0 failures and $floor passed (Checkpoint 8i's $CP8I_PASSED plus this stage's 14; a floor is not a count)"; exit 1; }
nd_check "(11) suite" || exit 1
out=$(wrapped cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(wrapped cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red (rc=$rc; never -D warnings on the CLI — ADR-0083)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (12) NOTHING DISPATCHED, NOTHING RUN, NOTHING LAUNCHED, NOTHING WRITTEN.
rows_after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$rows_after" = "$rows_before" ] || { echo "FAIL: the store moved during this block ($rows_before -> $rows_after) — a re-run block wrote to it, or something dispatched"; exit 1; }
refs_after=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
[ "$rc" -eq 0 ] && [ "$refs_after" = "$refs_before" ] || { echo "FAIL: an rk-run/* ref moved or appeared during this block — a lane ran:"; diff <(printf '%s\n' "$refs_before") <(printf '%s\n' "$refs_after"); exit 1; }
nl_after=$(find "$launches" -type f 2>/dev/null | sort)
[ "$nl_after" = "$nl_before" ] || { echo "FAIL: a launch record appeared during this block — something went through tools/live-run.sh"; exit 1; }
nd_check "(12)" || { echo "FAIL: a claude process wrote into the trap, or the fake claude was asked for more than --version — something tried to dispatch"; exit 1; }
sess_after=$(find $CFG_DIRS -path '*redkiln-rs*' -type f -name '*.jsonl' 2>/dev/null | sort)   # unquoted on purpose, as in (0)
new=$(comm -13 <(printf '%s\n' "$sess_before") <(printf '%s\n' "$sess_after"))
[ -z "$new" ] || { echo "FAIL: a new session transcript for this repository appeared while this block ran — something dispatched (or a Claude session ran in this repository; re-run without one):"; printf '%s\n' "$new"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

echo "CP8J-END-OF-BLOCK reached"
```

**Stage close** *(re-sequenced 2026-09-24, D21-47; review finding 6).* The draft put the Checkpoint 8j output in the
work commit, and the `ci`/`fix-item` lines in a tracker commit that named the work commit. But the checkpoint reads
those lines, and it re-runs Checkpoint 7h, whose porcelain guard admits only `PROGRESS.md`. So the checkpoint can
neither run before the evidence is committed nor be part of the commit it reads. The preamble's two-commit pattern is
kept at both ends, work first and tracker last. The commits between them carry only what the gated acts recorded, and
none changes a code path:

1. **The work commit:**
   - the items and intake files, including the repro pair;
   - the records at `proposed`;
   - the lane fixture and the fourteen tests (the launch-record lint and its spawn helper are `s8h`'s);
   - `ci.yml` (the pnpm step; the cargo-nextest install is `s8c`'s, *D21-48 (2026-09-24)*, X15);
   - the `s7i` and `s7h` re-points;
   - the `answer.rs` comment correction;
   - `s8`'s amendments A1–A9 (A0 was made at insertion);
   - the 7f amendment A10 and `runbook/phase-1/s8j-screen-exempt.txt`, A10's two dependent amendments (8d-ii, 8e), and A12's `-H` in `s7c`
     and `s7d`;
   - the three gate classes' constants, their rows in both packs, and their spellings in Checkpoint 8h's
     `MINTED_CLASSES` literal;
   - the evidence file opened with its `record-rt57` line.
2. **The pin-move commit** (§8j.6 step 1). This is the constant, its prose, its literal test and fixtures, and the
   instrument file. The `pin-move` line is appended here. *D21-48 (2026-09-24):* it also carries X12's spelling
   change: `s8h` §8h.2's table cut to the pin's one row, `LIVE_WARRANTED` emptied, the pin's `--help` capture, and the
   golden argv fixtures and snapshots moved in their `--permission-mode` element.
3. The probe session appends its lines as it goes: `fixture-proofs`, `mutant`, `checkpoint-7c-suite`,
   `checkpoint-7c`, the three `probe` lines (probe 0's first) and `record-g`. The operator re-points the legacy checkout and appends
   `legacy-worktree-repointed`. **The evidence commit** takes those lines and the (g) flip. It changes no code path,
   and neither does the re-point, because `.redkiln/worktrees/` is ignored.
4. **CI** (§8j.7) runs over the pushed evidence commit, on the operator's go-ahead. A repair is made red-first, in its
   own commit. It moves a code path, so step 1b and every paid step whose launch record must carry the frozen code are
   run again (the pre-spend checks and Checkpoint 8j (6) and (7) refuse otherwise). The last green run's head is the
   frozen sha. **The frozen-record commit** appends `ci`, `fix-item` and `story-item`, each naming that sha. It
   changes no code path, so the frozen sha stays the CI-green one.
5. **Checkpoint 8j** runs over the frozen-record commit, from a plain terminal, on a clean tree. Its output is saved
   under `target/`. It is one-shot (§8j.10).
6. **The tracker commit:**
   - `PROGRESS.md` §1, with the work and frozen hashes, and rows 7c, 7d, 7h, 7i and 8j;
   - the Checkpoint 8j output;
   - the ledger rows;
   - `INDEX.md`'s execution order: nothing — the whole `8a`–`8j` chain entered it at insertion (A0).

Adversarial refuting rounds run until a round moves no code and no guard.

---
