# Step 8h — Cards, models and the run environment (Phase 1, part 8, sub-stage h)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8h` | Phase 1 | Checkpoint 8g | Checkpoint 8h | `s8g` | `s8i` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (HEAD `a6349b6`). What changed:
- §8h.0's four questions are closed as D21-43 N17 took them. Q1 reuses `NotAdopted` and `NotIsolated { surface: Env }`,
  and nothing is superseded, so Record A no longer supersedes ADR-0095. Q2 is the four `Usage` counters,
  Q3 is `Exit::Overage` (20), and Q4 is three probe-2 dispatches.
- D21-44 settled §8h.6's `.git` risk: run worktrees move to `.redkiln/worktrees/<item>`, which is `s8d-ii`'s to build
  (D21-46 R1 moved it out of `s8e`).
  Probe 1 now makes its worktree there, and it passes only if an edit inside that worktree succeeds. D21-45's
  protected-segment limit (`<wt>/.git`, `.cargo`, `.claude`, `.husky`) is stated and mapped to the typed
  `permission_denials` failure, with one new test, one mutant and one probe arm.
- AC-133's registry is `s8g`'s. This stage enters its types' enums in it and builds no second registry. Three
  `rk-types` tests are dropped, and `s8g`'s two are selected as regressions.
- The launcher gains a `wrap` mode, so that `s8i`'s corpus runner and `s8j`'s whole Checkpoint 7c can run through
  it. It also refuses `CLAUDE_PID` and `CLAUDE_EFFORT`. The live probes run from a plain terminal.
- §8h.10 carries rows for the gate kinds that earlier sub-stages mint (`s8f`'s `landing` and D21-43 N11's drift
  gate).
- The whole-suite floor is re-derived from Checkpoint 8g's recorded count. Checkpoint 8h re-runs Checkpoint 8g and
  the four older checkpoints its crate edits make provisional. It says where 7c, 7f and 8a are re-run, and why 7e
  cannot be.
- `s8`'s three `redkiln run` sites are `s8j`'s amendment (D21-43). This stage states the constraint and edits
  nothing there.
- New findings for the lead: 10–15 at the end of the file.

*Revised 2026-09-24 for D21-46* (HEAD `c5d19d7`; D21-46 is the authority). What changed:
- **The launcher gains an `answer` mode.** D21-46's integration call: *"s8h's launcher gains an **`answer` mode**, so
  s8j's hold answer is not refused for want of a launch record"*. §8h.7 specifies it: `run`'s environment refusals,
  `run`'s pin comparison, argument refusals of its own, and the same launch record. The launch check's scope on
  `answer` is restated precisely. The draft's *"`answer` where it continues the lane"* named a condition that no code
  has, because `verbs/answer.rs` names no harness at `c5d19d7` [measured: `grep -n -i 'harness'
  crates/rk-cli/src/verbs/answer.rs` prints nothing]. Two `rk-cli` tests and three mutants are added.
- **`wrap`'s guarantee is stated once, which resolves the self-contradiction `s8j` §8j.6 found.** The draft called
  `wrap` the entry for a child that does not dispatch through `rk-adapter-claude`, and then named Checkpoint 7c, whose
  probe does, as a caller. `wrap` now guarantees the refusals, the launch record and before/after version equality,
  and it never compares to the adapter pin. A `wrap` child that dispatches through the adapter carries its own pin
  check, as 7c's block does.
- **§8h.10's table names every gate class the `s8` stages mint, one explicit row each.** That is `s8f`'s two,
  `s8d`'s `unknown_facet:change`, `s8j`'s two human-root question gates, and `s8j`'s hold gate. `MINTED_CLASSES` is
  still re-read at stage open from the code.
- **`RK_CP_NESTED=1` and the replay no-dispatch guard are adopted** in Checkpoint 8h (13), in `s8g`'s form. D21-46:
  *"One re-run convention"* and *"One no-dispatch guard for replays"*.
- The counts move to 41 new tests (was 39), 17 new in `rk-cli` (was 15), and twenty mutants (was seventeen). New
  findings for the lead are 16–19.

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The run-worktree placement and lifecycle are `s8d-ii`'s** (D21-46 R1), and `s8e` keeps the `worktree_assigned`
  row. Every sentence that gave the placement to `s8e` now names `s8d-ii`.
- **§8h.0 item 4's count source** is `s8e` §8e.10's interim `item` key, carried beside `dispatch_completed`'s payload
  under D21-46 R3.
- **§8h.0 item 8's floor** is recomputed once: Checkpoint 8g's 1127 + this stage's 41 = **1168**, not the drafted
  1000 + 41. Its `63 (8g)` is `s8g`'s 64.
- **§8h.10 names `s8j`'s three gate classes by the spellings `s8j` drafts** (`s8j` §8j.2, §8j.3).
- **§8h.14's 7f route** reads 7f as `s8d-ii` §8d-ii.7 left it: the run arm is in a throwaway clone and the cleanup line
  is gone, so the legacy tree is not re-scoped there. The 8a row quotes `s8a`'s guard from its current bytes.
- **Findings 16 and 17 are settled**: `s8i` §8i.5 and `s8j` §8j.6 re-quote §8h.7's current `wrap` text, and `s8j`'s
  screen matches the `answer` mode. The trap in (13) asserts that it starts empty.

*Adversarial-review fixes 2026-09-24 (D21-47)* (HEAD `fc80687`; `review-s8h.md` and the cross items assigned to
this file; D21-47 is the authority). Each finding id and its fix:
- **s8h #1 (BLOCKER)** → Checkpoint 8h (0c) builds D21-47's wrapper once, for the block's own cargo runs and for
  every replay. It has three parts:
  - a fake `claude` first on PATH that prints the pin. It is compiled as `claude.exe`, because a Rust
    `Command::new("claude")` on Windows never resolves a shell script, and 7d's `doctor` probe spawns it that way;
  - `CLAUDE_CONFIG_DIR` at an empty trap, with its control;
  - `REDKILN_LAUNCH_RECORD` removed, with a planted-value control.

  (13) replays under it, and (14) asserts that the trap and the fake's call log are both empty. A dated note in
  §8h.14 says that 7d's Halt 2 line and `RK-D-CLI-VERSION` meet the real binary only at `s8j`, after the pin move.
- **#2** → guard 3 reads two channels: the result's `permission_denials`, and the `permission_denied` stream event
  keyed by `decision_reason_code` (§8h.5). This adds one test and mutant M23. Probe 1 prints `DENIAL_CHANNELS`. A
  new one-haiku probe 0, `live::live_denial_channel_preprobe`, measures the channel before the Sonnet is spent
  (§8h.11; finding 22).
- **#3** → the launcher's own guarantees are now exercised (§8h.7 fixes the record's keys and the refusal line's
  form):
  - (10)'s environment carries an inherited `CLAUDE_CONFIG_DIR` pointing elsewhere and `DISABLE_AUTOUPDATER=0` in
    every control;
  - a non-check `wrap` pair runs in a scratch repository. The control reads every launch-record member, and the
    second run must fail on a moved CLI, naming Halt 2;
  - refusals are read by their anchored `live-run.sh: refused: <NAME>` form;
  - mutants M21 (inherited, not set) and M22 (step 9 deleted).
- **#4** → `codehits` uses `grep -rnH`, and a single-file synthetic control stands beside the directory one. The
  sibling `rk_absent` (`s7c` §7c.21, `s7d`) is reported, not edited (finding 20).
- **#5** → every cargo run of the stage, mutant cycles included, runs under (0c)'s wrapper (§8h.12, §8h.13). The
  launch-check control names its harness by `CARGO_BIN_EXE_rk-fixture-child-cli`. (14) counts journal rows as well
  as store rows, in `s8a`'s `dcount` form, with a seeded control.
- **#6** → all eight `live::` tests open with the launch-record guard, and each ignore reason names
  `tools/live-run.sh probe live::<name>`. `live_guard::every_live_test_opens_with_the_launch_record_guard` asserts
  that the two counts are equal (one new test; mutant M25).
- **#7** → `autoMemoryEnabled: false` joins the child's `--settings` (§8h.6), and probe 2 asserts that the memory
  file set is unchanged. One new test; mutant M24.
- **#8** → probe 2 asserts `plugins == []`.
- **#9** → the closed set is re-cited to `claude --help`: six choices, including `manual`. `default` is added as the
  init's name for `manual` and as D21-41 (g)'s spelling (§8h.2; finding 21).
- **#10** → a denial's path is extracted per tool (`file_path`, `path`, or an absolute `pattern`), and
  `.config/git` is matched as two consecutive segments (§8h.5). One new test.
- **#11** → `AI_AGENT` is refused at step 2, in (0b), and in (10)'s refusal list.
- **#12** → the unused store backup is deleted. (14)'s count covers what it was for.
- **#13** → §8h.10 quotes ADR-0146's actual sentence.
- **#14** → §8h.14's 7e row cites (8) and the `cards::` tests, not `conformance::shipped_pack_loads`.
- **#15** → (8)'s premise reads each minted class from a `&str` constant or constant table, with a control.
  §8h.0 item 7 says what happens to a class built with `format!`.
- **#16** → `wrap` screens its argv (`refused: WRAP_ARGV`) for a `redkiln run` or `answer` and for a live
  `--ignored` cargo run. Three controls in (10); mutant M27.
- **Cross #4** → `severity` is added to `rk_types` `Finding`, typed by the 11c record's vocabulary (§8h.4). Record
  A supersedes ADR-0074 decision 1 in part. This adds two `rk-types` tests, one `rk-kernel` test and mutant M28,
  and Checkpoint 8b joins (13).
- **Cross #5** → `RootCause` carries `discriminating_test`, and the stand-in's row gains it (§8h.4). One test.
- **Cross #7** → `verify.ac-discriminates` has a fail-closed row in both packs and is in `MINTED_CLASSES`
  (§8h.0 item 7, §8h.10).
- **Cross #13** → the shell route names `rk-fixture-child-claude` (D21-47). The `rk-cli` in-crate test binds
  `rk-fixture-child-cli`, which is the same source built for that package, because cargo sets `CARGO_BIN_EXE_*`
  only for a package's own binaries [measured: `crates/rk-cli/Cargo.toml` and `crates/rk-adapter-claude/Cargo.toml`
  build both names from `rk-containment/tests/fixtures/bin/rk-fixture-child.rs`].
- **D21-47's rules:**
  - Checkpoint 7c's re-run at `s8j` is split (§8h.7, §8h.14).
  - Launch-gate tests never inherit a launch record, enforced by a lint
    (`run::every_binary_spawn_withholds_or_supplies_the_launch_record`; mutant M26).
  - This stage amends Checkpoint 8a (11), the card precondition its own cards falsify, in its own work commit
    (§8h.14).
- **Counts:** 50 new tests, up from 41: `rk-adapter-claude` 21, `rk-types` 4, `rk-kernel` 7 and `rk-cli` 18. There
  are twenty-eight mutants, up from twenty. The floor is 1127 + 50 = **1177** *(superseded by the final consistency pass: 1171 + 51 = **1222**)*. New findings for the lead are 20–26.

*Final consistency pass 2026-09-24* (the eleven `s8` files read against each other after the D21-47 fix round; the
pass brief and the lead's integration findings are the authority). One line per edit:
- *Final consistency pass 2026-09-24:* Checkpoint 8h (0c) sources D21-47's shared wrapper, `runbook/tools/nodispatch.sh`
  (`nd_setup`, then `nd_enter`), in place of its inline fake. `nd_check` runs after the block's cargo runs, after each
  replay and at (14); (13) replays as `nodispatch env RK_CP_NESTED=1 bash …`; (11)'s reading uses `ND_REAL_CLAUDE`.
  §8h.12 and §8h.13 run every Red, Green and mutant cycle under the same helper.
- *Final consistency pass 2026-09-24:* (10)'s launcher controls keep shell-script fakes, deliberately. The reason is
  stated there, with a control that their explicit PATH reaches no real `claude`.
- *Final consistency pass 2026-09-24:* the checkpoint's inputs are literals in its INPUTS lines (D21-47: records are
  located by the id recorded at write time). `REC_A`, `REC_B` and `MINTED_CLASSES` refuse while empty; `A_SUPERSEDES`
  and `TOKENS_APPETITE` carry their drafted values. Re-runners supply nothing.
- *Final consistency pass 2026-09-24:* the floor is recomputed once more from the files as they now state their counts:
  BASE_8G = 1171, floor 1171 + 51 = **1222** (§8h.0 item 8).
- *Final consistency pass 2026-09-24:* `Spec`'s member is `acceptance_criteria` (`s8g` §8g.3; D21-47 review finding 15),
  and N24's authored-verification check is specified with one `rk-adapter-claude` test and mutant M29 (§8h.4). That
  makes 51 new tests and twenty-nine mutants.
- *Final consistency pass 2026-09-24:* §8h.14 also amends Checkpoints 8c (15) and 8d (12a), whose card-mode guards this
  stage's cards falsify, and states why 8d-ii's and 8e's are not amended here. Findings 27 and 28 are new.
- *Final consistency pass 2026-09-24:* §8h.7 states that a `wrap` child has the launcher's own stdin, stdout and stderr,
  and where the launch record resolves (`s8i`'s two questions); (10) asserts that stdin reaches the child.
- *Final consistency pass 2026-09-24:* finding 23 is corrected, and findings 10, 20 and 24 carry their current status.

*D21-48 (2026-09-24)* (the operator's X12 on the argv spelling of D21-41 (g)'s read-only mode; D21-48 is the
authority). One line per edit:
- *D21-48 (2026-09-24):* §8h.2's UNMEASURED paragraph on `--permission-mode default` is replaced by X12's decision,
  quoted: the adapter emits the spelling the pinned CLI's `--help` lists; cards and the init comparison keep `default`.
- *D21-48 (2026-09-24):* §8h.2 specifies the mapping: one closed `PERMISSION_MODE_ARGV` table in `command.rs`, keyed by
  the `Config`'s pin, with rows `2.1.278` → `default` (live-warranted, `LIVE_WARRANTED`) and `2.1.281` → `manual`
  (dormant until the pin move), one `cli-help/<version>.permission-mode.txt` capture per help-warranted row, and no
  fallback for a pin without a row.
- *D21-48 (2026-09-24):* §8h.0 item 6 captures the help's `--permission-mode` lines at stage open. §8h.5 guard 2
  compares the init to the card's `default`, never to the argv's spelling.
- *D21-48 (2026-09-24):* §8h.8 step 3 puts the spelling change and the golden argv move in the pin-move commit. Step 4's
  `live_dispatch_probe` is the first live argv to carry the new spelling.
- *D21-48 (2026-09-24):* probe 0 (§8h.11) confirms the spelling and prints `ARGV_MODE_SPELLING`. A refused argv is a HALT
  for a wrong capture, no longer a stop for the lead's decision. Finding 21 is settled.
- *D21-48 (2026-09-24):* three `command::` tests (§8h.12) and mutants M30–M33 (§8h.13). Checkpoint 8h (3) selects 31
  tests in `rk-adapter-claude`, and (9) checks the table's declared form and every help capture against its row, each
  behind a synthetic control.
- *D21-48 (2026-09-24):* new tests 51 → 54 (`rk-adapter-claude` 22 → 25), mutants twenty-nine → thirty-three. The floor
  is BASE_8G + 54, which is 1171 + 54 = **1225** on the recomputed chain (§8h.0 item 8; (12)).
- *D21-48 (2026-09-24), floor propagation:* the predecessors now state `s8c` **78**, `s8d` **62**, `s8d-ii` **50** and `s8f` **85**, so BASE_8G is
  **1189** and the floor 1189 + 54 = **1243** (§8h.0 item 8; (12)). The 1171 + 54 = 1225 above is superseded.
- *D21-48 (2026-09-24):* §8h.14 hands the pin-move edit to `s8j`, and §8h.15 states the table among its conditions.

---

**Why this step exists.** D21-40 decision 1 names four structural reasons that criterion (1) cannot be satisfied.
The last of them is this stage's: *"the shipped registry cannot bind a dispatch (0 of 8 cards carry
`permission_mode`, all 8 grant only `StructuredOutput`, no `.redkiln/schemas/`, 0 `model` rows)"*. Each clause was
re-measured at `ca8f387` on 2026-09-24:

- **The cards.** All eight `.redkiln/cards/*.json` hold exactly the keys `cache_profile`, `description`, `prompt` and
  `tools`, and every `tools` is `["StructuredOutput"]` (read with `json.load`, not grep). None carries
  `permission_mode`, so `AuthoredCard::from_file` refuses every one with `PermissionModeAbsent`. The only check on the
  mode's value is `card.rs`'s `if permission_mode.trim().is_empty() {`, so a misspelt `"acceptEdit"` would bind.
- **The models.** Both packs' eight `[role.*]` tables each carry one key, `family = "claude"`. `RoleExtras.model` is
  `Option<rk_adapter_claude::card::Tier>`, and `Tier` is the closed lifted enum `Haiku | Sonnet | Opus`.
  `--model` receives the tier word. Adoption is read through `init.rs`'s
  `reported == tier || reported.split('-').any(|seg| seg == tier)`, which is a segment match: a reported
  `claude-sonnet-4-6` "belongs" to a requested `sonnet`. `init.rs` says so itself: *"A record settling the reading is
  STILL OWED"*. `DispatchRecord` has no model member.
- **The schemas.** `bind_nodes` reads `.join("schemas")` then `.join(format!("{kind}.json"))`, where
  `let kind = activity.provides.first().cloned().unwrap_or_default();`. That directory does not exist. The binder's
  own module doc says the schema comes *"from `rk_types::registry::EXPORTED` by name"*, and
  `pub const EXPORTED: &[Exporter] = &[("Findings", || {` holds exactly one entry.
- **Principle 7.** Design principle 7 forbids *"inferring from the parent side that a flag took effect — an ignored
  flag looks identical to an honoured one"*. Nothing in `rk-adapter-claude/src` reads `permissionMode`,
  `apiKeySource` or `permission_denials` back from the child. `command.rs` emits no `--setting-sources` and no
  `--settings`, so the child discovers the operator's user settings. Those settings carry `enabledPlugins` and
  `autoMode` [measured 2026-09-24: the key NAMES of `C:\Users\ryanm\.claude-personal\settings.json`; no value was
  read, and there is no `permissions` key].
- **The pin.** `pub const TESTED_CLI_VERSION: &str = "2.1.278";` and `RUNBOOK-instrument.txt` reads
  `2.1.278 (Claude Code)`, but `claude --version` prints `2.1.281 (Claude Code)`. **Halt 2 is live.**
  `~/.local/bin/` holds `claude.exe` (2026-09-23) beside `claude.exe.old.1790192683866` (2026-09-22). That is the
  native updater's rename, so the instrument moved under the tree between sessions.
- **The environment.** No launcher exists. The shell this file was authored from carries `CLAUDECODE` and seven
  `CLAUDE_CODE_*` names (read by name only). `CLAUDE_CONFIG_DIR` is inherited, not set.
- **The envelope and the scalars.** The whole appetite is `appetite = { points = 40 }` in `bug-standard` and
  `appetite = { points = 60 }` in `story`. `appetite.tokens` is read by `select/pack.rs` and set nowhere, and no
  per-item dispatch count exists anywhere. `grep -c 'NO RECORD'` returns 5 in each pack:
  - the `gate_deadline_secs` comment;
  - the banner;
  - the breaker comment `# value; NO RECORD names the number.`;
  - `consecutive_platform_failures = 3   # NO RECORD -- authored at Step 7e (RT-57)`;
  - `expiry_secs = 600                   # NO RECORD -- authored at Step 7e (RT-57)`.
- **The gate expiry.** The packs carry 0 `[[gate_expiry]]` rows. ADR-0146 records the consequence: *"Zero rows, for
  zero of the 45 question classes"*, and a step gate past its deadline makes the next run exit 3.

🛑 **THIS STAGE DISPATCHES NOTHING.** D21-40 decision 2 gates every real dispatch on the fixture proofs, and those
are `s8j`'s. The two live probes of D21-41 (g) are **authored** here (§8h.11) and **run at `s8j`**. The CLI pin
**procedure** is authored here (§8h.8) and **executed at `s8j`**. Record A of §8h.1 is therefore committed
`proposed`, and it is accepted only after both probes pass. Checkpoint 8h asserts that no `dispatch` or
`dispatch_completed` row was appended while it ran.

**What this stage owns.**

| decision | what it builds or writes | section |
| --- | --- | --- |
| D21-41 (g) | the eight cards; three principle-7 guards; schemas as `EXPORTED` Rust types, read by name | §8h.2, §8h.4, §8h.5 |
| D21-42 15a, 15b | `[models]` table; full id to `--model` and the card digest; typed `observed_model`; exact-id refusal | §8h.3 |
| D21-42 16 | the pin-move procedure and the auto-update freeze (executed at `s8j`) | §8h.8 |
| D21-42 17a, 17b | the committed launcher; `CLAUDE_CONFIG_DIR` set to `~/.claude-personal`; the `apiKeySource` check | §8h.7, §8h.5 |
| D21-42 19a, 19c | the per-item dispatch cap and token appetite; the RT-57 scalar record | §8h.9 |
| D21-42 21a | `gate_expiry` drafted fail-closed for every reachable class, signed row by row | §8h.10 |
| D21-42 21b, 21d | explicit minimal `--setting-sources`; the child's confinement settings; probe 1's HALT | §8h.6, §8h.11 |
| D21-42 21c | the doctor card-grant row and ADR-0146's load-time lint | §8h.10 |
| D21-43 N17 | Q1–Q4 as taken: the refusal classes, the token unit, the envelope's exit, probe 2's three dispatches | §8h.0, §8h.5, §8h.9, §8h.11 |
| D21-44; D21-45's standing limit | probe 1 inside `.redkiln/worktrees/<probe>`; the protected-segment limit named by the typed denial failure | §8h.5, §8h.6, §8h.11 |
| D21-46's integration calls | the launcher's `answer` mode; `wrap`'s guarantee; one explicit row per minted gate class; `RK_CP_NESTED` and the replay trap | §8h.7, §8h.10, §8h.15 |
| D21-47's rules and assignments | the no-dispatch wrapper for the stage's cargo runs and every replay; `Finding.severity`; `RootCause.discriminating_test`; `verify.ac-discriminates`'s row; guard 3's second channel and probe 0; the launch-record lint; the Checkpoint 8a (11) amendment | §8h.4, §8h.5, §8h.7, §8h.10, §8h.11, §8h.14, §8h.15 |
| D21-43 N24, as `s8g` §8g.3 hands it on | every authored `Spec` criterion carries a `verification`: the card's output contract and the ingest check (*Final consistency pass 2026-09-24*) | §8h.4 |
| D21-48 X12 | the argv spelling of a card's `default`: `PERMISSION_MODE_ARGV` keyed by the pin, its `--help` captures, and the pin-move edit handed to `s8j` (*D21-48 (2026-09-24)*) | §8h.2, §8h.8, §8h.11 |

**Owned elsewhere, cited rather than re-specified here.**
- **The run-worktree placement and its lifecycle belong to `s8d-ii`** (D21-46 R1: *"It carries D21-44's relocation and
  D21-45 W1/W2/W4"*), and the `worktree_assigned` row stays `s8e`'s. That covers the move to `.redkiln/worktrees/<item>`,
  its record, the `worktree_assigned` row, item-id validation, the symlink guards, `RK_BASE`, the lock, re-entry,
  and the filter-driver refusal. D21-44 wrote *"Consequences, owned by `s8e`"*, and D21-45 *"W1, W2 and W4's worktree
  half are **s8e**'s"*, before D21-46 R1 split them into `s8d-ii`.
- **The per-toolchain `setup` verb and `.worktreeinclude` reading belong to `s8c`** (D21-43 N25; D21-45 W3).
- **AC-133's judgement-enum registry belongs to `s8g`.** D21-43: *"AC-133's judgement-enum registry is built once,
  by **s8g**. s8h consumes it."*
- **Routing `s8`'s three `redkiln run` sites through the launcher belongs to `s8j`.** D21-43: *"s8's three
  `redkiln run` sites go through s8h's launcher (s8j amendment)."*
- **The pin move, both probes' execution and Checkpoint 7c's whole re-run belong to `s8j`.** See the plan's `s8j`
  row, and D21-43 N28: *"**s8j re-runs it whole**, from a plain terminal through the launcher, with the live
  probes."*

### 8h.0 Stage open: re-measure, and the four questions D21-43 N17 took

**The entry gate.** Checkpoint 8g must be recorded as passed in `PROGRESS.md` §1 with a hash, and its block is re-run
whole by Checkpoint 8h (§8h.15 (13)). Before any Red, re-measure the following against the tree `s8g` left, and
write each reading into the stage-open commit:

1. `cargo test -p <crate> --test it -- --list` for the four crates named in §8h.12. Every regression name that
   §8h.12 reuses must still exist. A rename by `s8a`–`s8g` is corrected here with a dated line, never silently.
2. **How `s8d`'s per-step binding names a step's output kind.** §8h.4 reads the schema by the **bound step's**
   output kind, the `value:<Kind>` of its `outputs`, never by `provides[0]`. If `s8d` left `bind_nodes` per node,
   the per-node `provides[0]` read is replaced by the step's kind here, and the gap is reported.
3. **Which lane kinds already exist as `rk-types` types.** At `ca8f387`, 0 of §8h.4's 24 kinds other than
   `Findings` exist as a type. `s8f`/`s8g` may have added `DebtRegisterEntry` (D21-42 6a), `Spec` or
   `AcceptanceCriterion` (D21-42 10a). A type that exists is **registered, never redeclared**. `s8g` exports
   `AcceptanceCriterion` and fixes a constraint on this stage's `Spec` (`s8g` §8g.3): *"`s8h`'s `Spec` must embed
   `AcceptanceCriterion` unchanged"*.
4. **The count source for §8h.9.** Does the kernel's pre-dispatch `dispatch` row name the item? If it does not, the
   count reads `dispatch_completed` by its interim `item` key, which `s8e` §8e.10 carries beside the payload
   (D21-43 N5; D21-46 R3), through `query(ByItem)`. Either way the source is the store, never a per-process counter.
5. **Whether `budget_extension` has gained a producer.** At `ca8f387` `wall_clock_expiry_mark`'s write has *"no call
   site anywhere in `src/`"* (`budget.rs`). If an earlier sub-stage wired one, that class joins §8h.10's table.
6. `claude --version`, and the size and bytes of `RUNBOOK-instrument.txt`. Halt 2 is recorded, not worked around,
   and D21-42 16 moves the pin once, at `s8j`.
   *D21-48 (2026-09-24):* in the same step, write the lines of `claude --help` that document `--permission-mode`,
   through their `(choices: …)` list, to `crates/rk-adapter-claude/tests/fixtures/cli-help/<version>.permission-mode.txt`
   by `printf`, never `tee`, where `<version>` is the `claude --version` just read (§8h.2's argv spelling table; X12).
   `--help` dispatches nothing. The spelling of the table's help-warranted row is read from this capture, and the
   choices are recorded in the stage-open commit.
7. **The gate classes `s8d`–`s8g` mint** (§8h.10's table lists every `s8` class, one row each). `s8f` mints two,
   drafted as `landing.refused-path` (`s8f` §8f.4) and `render.structured-drift` (`s8f` §8f.5). `s8f`'s landing and
   render records fix the spellings. *(dated 2026-09-29, `s8f` close: the records are ADR-0188 and ADR-0189, accepted on D21-117 and D21-118; the code spells the classes `landing.refused-path` (`tier_one::LANDING_REFUSED_KIND`) and `render.structured-drift` (`fsm::DRIFT_GATE_KIND`), and until this stage lands neither class reaches an expiry (D21-116))* `s8d`'s R6 opens `gate_opened { reason: unknown_facet, axis: "change" }` at
   triage, and the gate is answered with the `facet.change` question (`s8d` §8d.6, §8d.7), so its class is
   `unknown_facet:change`. Read each spelling from the code the minting stage landed, never from this file.
   - Write `MINTED_CLASSES` into Checkpoint 8h's INPUTS line, by a dated edit in the stage-open commit, as the
     kernel-minted classes only. At this stage that is `s8f`'s two plus `s8g`'s `verify.ac-discriminates` (D21-47 cross
     #7). That is the AC-357 gate `s8g`'s verify pass opens (`s8g` §8g.5). It is declared under `.redkiln/processes/`
     (`s8g` §8g.6) and is not one of the 45 catalog questions. *(dated 2026-10-02, `s8g` close: the class is spelled by the constant `rk_kernel::verify::ac_gate::AC_GATE_KIND` (`"verify.ac-discriminates"`) on a code line under `crates/*/src`, under ADR-0191 decision 10, accepted on D21-131, and it is declared once in `.redkiln/processes/questions.toml`; `s8g` wrote no expiry row for it (D21-129))*
   - Each minted spelling must be a `&str` constant, or a member of a constant `&[&str]` table, on a code line
     under `crates/*/src`, because (8)'s premise reads it there (s8h review #15). A class that the minting stage
     builds with `format!` and no constant is a stage-open gap. This stage adds the constant beside the mint site,
     points the mint at it, and reports the gap.
   - Confirm that R6's gate carries `unknown_facet:change`. That class is an axis row, not a minted one, and (8)
     asserts it by name.
   - A class that a later stage mints brings its own row in the change that mints it, because §8h.10's load-time
     lint refuses a row whose class nothing mints. §8h.10 names `s8j`'s three.
8. **The whole-suite floor.** Read Checkpoint 8g's recorded passed count from `PROGRESS.md` §1 and write it over
   `BASE_8G` in Checkpoint 8h (12) by a dated edit. The expected value, **re-derived at stage open from Checkpoint
   8g's recorded count**, is ~~BASE_8G = **1189**, so the floor is 1189 + 54 = **1243**~~ *(dated 2026-10-02, s8h build)* BASE_8G = **1337**, Checkpoint 8g's RECORDED count (D21-129), so the floor is 1337 + 54 = **1391**; the drafted chain below is superseded by the recorded count, as the paragraph's last sentence requires. The arithmetic, once:
   - ADR-0164's baseline at `992d257`: 740 passed / 0 failed / 9 ignored over at least 44 results;
   - plus each sub-stage's new tests as its file states them: 9 (8a) + 25 (8b) + 78 (8c) + 62 (8d) + 50 (8d-ii) +
     74 (8e) + 85 (8f) + 66 (8g) = 449, which gives 1189;
   - plus this stage's 54 (§8h.12), which gives 1243. *D21-48 (2026-09-24):* 51 → 54, for X12's three `command::`
     tests (§8h.2), so 1222 → 1225. *D21-48 (2026-09-24), floor propagation:* `s8c` 69 → 78, `s8d` 58 → 62,
     `s8d-ii` 46 → 50 and `s8f` 84 → 85 move BASE_8G 1171 → 1189 and the floor 1225 → 1243.

   *Final consistency pass 2026-09-24:* this supersedes the D21-46 recomputation's BASE_8G = 1127 and floor 1127 + 50 =
   1177, which rested on sibling drafts since revised (9 + 21 + 63 + 52 + 32 + 67 + 79 + 64 = 387 over 8a..8g).

   Those drafts are being revised too, so the recorded count governs. A floor is never lowered to make a red run
   green.
9. **The run-worktree root.** Confirm that `s8d-ii` moved every derivation of the run-owned worktree to
   `.redkiln/worktrees/<item>` (D21-44), and name `recreate_run_worktree`'s successor. §8h.6's deny set and probe 1
   derive from the spec's worktree path and assume nothing more.
10. **AC-133's test names in `s8g`.** In the draft at `a6349b6`, Checkpoint 8g selects
    `judgement_registry::ac133_every_exported_enum_is_registered` and
    `judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails`. Checkpoint 8h (4) selects both.
    A rename is corrected there by a dated line.
11. **Every binary-level `answer` spawn** (D21-46). List the tests that run the `redkiln` binary's `answer` verb. That
    includes `rk-cli`'s shared helper in `tests/it/answer.rs` [measured at `c5d19d7`: `pub(crate) fn spawn_answer(`],
    `s8e`'s `lane_e2e` binary arm, and any other. §8h.7's launch check on `answer` refuses each of them unless it
    carries a fixture launch record. The record is supplied in the shared helper, so every such test exercises the
    check and none bypasses it. The list goes into the stage-open commit.
12. **The 11c record's severity vocabulary, and `RootCause`'s stand-in** (D21-47 cross #4, #5). Locate `s8b`
    §8b.6's record by the id recorded when it was written, never by its title. Read the vocabulary's spellings from
    it, and confirm that its paragraph opening `**The severity field's source and owner.**` names this stage.
    Confirm that `spikes/p9/schemas.toml`'s `[value.RootCause]` row still lacks `discriminating_test`
    [measured at `fc80687`: `fields = ["cause", "contributing_factors", "confidence", "summary"]`].
13. **The permission-mode choices.** Re-read `claude --help`'s `--permission-mode` choices, which is a spawn and
    not a dispatch. §8h.2's closed set is cited to that list, and a change to it is corrected there by a dated line.

*(dated 2026-10-02, s8h build)* **Stage-open readings, package P1-records, at `2770bda` (HEAD and a clean tree confirmed; `PROGRESS.md` §1
records Checkpoint 8g passed whole at `4777941` and re-run green at `d5bfc4f`, 1337 passed / 0 failed / 9 ignored over
44, D21-129).** Each item above, as read. The store was backed up first to `target/store-backups/s8h-open` (sha256
`09020ee9b3deb4405173cd4b71bf858a4f7a36ac23b3e762c9ced1c36038fad8`, equal to the live store's).

- **Item 1.** `cargo test -p <crate> --test it -- --list`, each through D21-47's wrapper (the AFK `nd.sh`): `rk-adapter-claude`
  112, `rk-types` 77, `rk-kernel` 415, `rk-cli` 310 tests listed. Every regression name §8h.12 and Checkpoint 8h (3)–(6)
  reuse is listed, so no line in (3)–(6) is corrected: `command::golden_argv_per_profile`,
  `command::ac214_permission_mode_explicit`, `command::permission_prompts_none_on_every_argv`,
  `isolation::ac204_adoption_equal_subset_superset_wrong_agent` (beside `isolation::ac204_adoption_at_2_1_277_is_read_from_agents_and_model`),
  `isolation::ac202_isolation_from_system_init_not_argv` (beside `isolation::ac202_absent_mcp_key_is_the_empty_set`),
  `config::tested_cli_version_is_the_pin_as_a_config_value`, `schema_snapshot::schema_snapshot`,
  `schema_no_oneof::no_oneof_at_any_node`, `schema_caps::union_and_optional_caps`,
  `dispatch_record::ac360_dispatch_record_carries_the_usage_fields`, the three `judgement_registry::` names (item 10),
  `envelope::ac029_n_is_declared_never_defaulted`, `conformance::shipped_pack_loads`,
  `doctor::ac415_every_rule_id_has_a_rejecting_fixture`, `exit::ac438_exit_table_snapshot` and
  `exit::ac438_step_error_mapping_is_exhaustive` (of five `exit::ac438_` names), and `surface::ac400_help_snapshot_every_verb`.
- **Item 2.** `s8d` left the binding **per agent step**, not per node: `bind_nodes` (`crates/rk-cli/src/verbs/run.rs`) walks each
  activity's `kind = "agent"` steps and passes each step's own `step.output_type` (its `value:<Kind>`) to `bind_one`; its
  comment reads *"never `activity.provides.first()`"*. No per-node gap. The remaining gap is the read itself: `bind_one`
  still reads `.join("schemas")` then `.join(format!("{kind}.json"))`, which §8h.4 replaces by the `EXPORTED` lookup.
- **Item 3.** Of §8h.4's 24 kinds, **one** exists as an `rk-types` type: `Findings` (`rk_types::artifact::findings`).
  `DebtRegisterEntry`, `Spec`, `RootCause` and the other 21 do not (`grep -rnE "pub (struct|enum) <Kind>\b"
  crates/rk-types/src` → 0 each). `AcceptanceCriterion` is exported (`registry::EXPORTED`'s second entry, ADR-0192) and is
  embedded by `Spec`, never redeclared. So 23 types are new.
- **Item 4.** The kernel's pre-dispatch `dispatch` row **names the item twice**: its payload carries `item`
  (`rk_kernel::runtime::activity::dispatch_event`: `{activity_id_at_version, attempt, item, step_id}`), and
  `EventKind::Dispatch` is `ItemKey::Carried`, so `Runtime::append` keys it with the interim `item` (ADR-0182).
  `dispatch_completed` is `Carried` too. The count source is therefore the store's `dispatch` rows through
  `query(ByItem)`, counting only rows keyed to the item (`ByItem` also returns unkeyed rows in the interim, ADR-0182
  decision 4), and the token sum reads `dispatch_completed` the same way. Record B (ADR-0198) decision 4 states it.
- **Item 5.** No producer. `wall_clock_expiry_mark(` has no call site under `crates/*/src` (the definition at
  `crates/rk-kernel/src/workflow/budget.rs` only; its callers are `rk-kernel`'s tests `budget.rs` and
  `cross_process.rs`), and `budget_extension` appears only as `BudgetExtensionMark::KIND` and its reader. It does not
  join §8h.10's table.
- **Items 6 and 13 (D21-48).** The real `claude --version` → `2.1.287 (Claude Code)` (rc 0; `~/.local/bin/claude.exe`,
  247,736,480 bytes, dated 2026-10-01 11:02). `spikes/results/instrument-checks/RUNBOOK-instrument.txt` is 22 bytes,
  `od -c` → `2 . 1 . 2 7 8 ␠ ( C l a u d e ␠ C o d e ) \n`. **Halt 2 is LIVE and recorded, not worked around**: the pin is
  `2.1.278`, and the installed CLI has moved again since authoring (2.1.281 → 2.1.287); D21-42 16 moves the pin once, at
  `s8j`. `claude --help`'s `--permission-mode` lines through `(choices: …)` were written by `printf` (never `tee`) to
  `crates/rk-adapter-claude/tests/fixtures/cli-help/2.1.287.permission-mode.txt` (282 bytes, 4 lines, LF, 0 CR,
  byte-identical to the help's lines 150–153). **The choices are `"acceptEdits", "auto", "bypassPermissions", "manual",
  "dontAsk", "plan"`**, the same six as 2.1.281. They list `manual`, so X12's premise holds, no STOP, and by §8h.2's rule
  the help-warranted row is keyed by the installed version: `("2.1.287", "manual")`, not the drafted `2.1.281` (§8h.2's
  dated note). Both reads were spawns, not dispatches.
- **Item 7.** Each minted spelling is a `&str` constant on a code line under `crates/*/src`, and no `format!` gap exists:
  `pub const LANDING_REFUSED_KIND: &str = "landing.refused-path";` (`crates/rk-kernel/src/runtime/tier_one.rs`),
  `pub const DRIFT_GATE_KIND: &str = "render.structured-drift";` (`crates/rk-kernel/src/workflow/fsm.rs`), and
  `pub const AC_GATE_KIND: &str = "verify.ac-discriminates";` (`crates/rk-kernel/src/verify/ac_gate.rs`). R6's gate
  carries `unknown_facet:change`: `workflow/gate.rs` builds `format!("unknown_facet:{axis}")`, an axis row and not a
  minted class, and `.redkiln/processes/questions.toml`'s `[question."facet.change"]` reads `binds =
  ["unknown_facet:change"]`. `MINTED_CLASSES` is written into Checkpoint 8h's INPUTS by dated edit.
- **Item 8.** BASE_8G is Checkpoint 8g's RECORDED count, **1337** (D21-129), so the floor is 1337 + 54 = **1391**,
  written over (12)'s `BASE_8G` by dated edit. The drafted 1189 / 1243 are struck in item 8.
- **Item 9.** Confirmed: `rk_git::worktree`'s one derivation roots every run tree at `run_tree_root(main)` →
  `main.join(".redkiln").join("worktrees")` (`<item>`, `<item>.review-ref`, `<item>.mut`). `recreate_run_worktree` is
  retired (`s8d-ii` P6; ADR-0180 decision 9), and its successor is **`rk_git::worktree::enter`**: created at
  `.redkiln/worktrees/<item>`, re-entered and reused, or refused and kept.
- **Item 10.** Three `judgement_registry::` names exist: `ac133_every_exported_enum_is_registered`,
  `ac133_a_judgement_enum_without_a_rationale_sibling_fails` (both selected by (4)) and
  `the_registry_names_no_enum_that_is_not_exported`. No rename.
- **Item 11.** Binary-level `answer` spawns: the shared helper `pub(crate) fn spawn_answer(` (`crates/rk-cli/tests/it/answer.rs`),
  called from `answer.rs`, `change_deriver.rs`, `landing.rs`, `lane_e2e.rs`, `processes.rs`, `run.rs`, `verify_pass.rs`
  and `worktree_assigned.rs`. `s8e`'s `lane_e2e` binary arm is `spawn_run_replaying` (a `run` spawn). **Binary spawn
  sites over the `redkiln` binary in `rk-cli`'s `tests/it`: 21 code lines** (13 at `fc80687`): 20 of the form
  `Command::new(crate::redkiln())` (`redkiln()` in `main.rs`) in `adapter_selection.rs`, `answer.rs`, `config.rs`,
  `cost.rs` (3), `doctor.rs` (3), `doctor_journal.rs`, `explain.rs`, `lane_e2e.rs`, `main.rs`, `review_pass.rs`, `root.rs`,
  `run.rs` (2), `run_worktree.rs`, `slice_verdict.rs` and `verify_pass.rs`, and one `Command::new(env!("CARGO_BIN_EXE_redkiln"))`
  in `surface.rs`. The other `Command::new` sites (`support.rs`'s `taskkill`/`kill`/`tasklist` and its fixture child) do
  not spawn `redkiln`.
- *(dated 2026-10-02, s8h build, package P6-cli-launch-gate)* **Item 11, folded.** The 21 code sites listed above now construct no `Command`: each calls
  `crate::redkiln_command(crate::Launch::…)` (§8h.7's dated note names each arm), and
  `run::every_binary_spawn_withholds_or_supplies_the_launch_record` reads one site, `main.rs`'s helper
  (`THE_ONE_REDKILN_COMMAND=main.rs:<line>`). The three in-process `answer::run` callers are listed in §8h.7's note.
- **Item 12.** The 11c record is **ADR-0168** (recorded id; accepted on D21-65). Its decision 1: *"The severity vocabulary
  is `blocking | advisory`."* Its paragraph `**The severity field's source and owner.**` reads *"The owner is **`s8h`**"*.
  `spikes/p9/schemas.toml`'s `[value.RootCause]` still reads `fields = ["cause", "contributing_factors", "confidence",
  "summary"]`, without `discriminating_test`. `grep -rn "severity in" .redkiln/catalog` → **0** (D21-67 ruling 1 holds).
- **D21-119 … D21-136.** What binds this stage is D21-129's owed-forward list: the `verify.ac-discriminates` row in both
  packs and the class in `MINTED_CLASSES`; `Spec`'s `acceptance_criteria` embedding `AcceptanceCriterion` unchanged, with
  a required authored `verification` on every criterion a specifying activity emits; this stage's enums entered in
  AC-133's registry. D21-126 leaves `LintReport` without a type, outside `EXPORTED`; D21-132 accepts ADR-0192. **D21-128
  stays OPEN for the operator and is not re-decided here.** Checkpoint 7d's manifest loop now expects THREE `rk-cli`
  `[[bin]]` targets (amended 2026-10-02, `wi-b846d9`); a `[[bin]]` this stage adds needs the same dated amendment.
- **Line endings, by bytes** (`open(f,'rb').read().count(b'\r')`): this file 0 CR / 2321 LF; `s8a` 0 / 1927; `s8c` 0 / 3348;
  `s8d` 0 / 3476; `s8d-ii` 0 / 2835; `bug-standard.toml` 0 / 547; `story.toml` 0 / 568; each of the eight cards 0 / 8;
  `spikes/p9/schemas.toml` 0 / 416; `references/adr/README.md` 0 / 417; `runbook/00-preamble.md` 0 / 320. All LF.
- **`FLIP_PENDING`.** No line of Checkpoint 8h asserts `status: accepted` for this stage's records: (2) asserts Record A
  `status: proposed` and admits `proposed|accepted` for Record B. So the formal run passes with both `proposed`, and no
  `FLIP_PENDING` form is added.

**Four questions were owed before Red. D21-43 N17 took all four on 2026-09-24**, and each is stated here as taken.
Each one changes a test's expected value.

- **Q1: the typed class of the four new refusals.** Taken: *"reuse `NotAdopted` and
  `NotIsolated { surface: Env }`; nothing superseded."* ADR-0112 closes `StepError` at eleven and *"Forbids a
  twelfth variant without a successor to this record"*. ADR-0095 decision 2 fixes `NotIsolated`'s payload as
  `{ surface: tools \| mcp_servers \| env, diff_digest }`. The mapping:
  - **(i)** An init-reported model unequal to the requested id → `NotAdopted { expected_agent, observed }`. This is
    the RT-27 reading: `init.rs`'s no-`agent`-key arm already refuses a model outside the bound tier as
    `NotAdopted`.
  - **(ii)** `apiKeySource` absent or not `"none"` → `NotIsolated { surface: Env, diff_digest }`. The digest is taken
    over the reported value's one-member set (empty when the key is absent) against `{"none"}`.
    `IsolationSurface::Env` has no producer at `a6349b6` [measured: `grep -rn "IsolationSurface::Env" crates/*/src`
    returns nothing].
  - **(iii)** An init-reported `permissionMode` that is unequal or absent → `NotIsolated { surface: Env, .. }`. The
    digest is over `{reported}` against `{the card's mode}`.
  - **(iv)** A non-empty `permission_denials` → `NotIsolated { surface: Env, .. }`. The digest is over the sorted
    denied `(tool_name, path)` entries against the empty set.

  The draft recommended two new `IsolationSurface` members for (iii) and (iv), which would have superseded ADR-0095
  decision 2's payload cell in part. N17's *"nothing superseded"* rules that out. So (iii) and (iv) take `Env`,
  the surface of the child's run environment, and Record A states that reading. The variant count stays eleven,
  and the retry class is `NotIsolated`'s `Never`. `exit::ac438_step_error_mapping_is_exhaustive` and
  `exit::ac438_exit_table_snapshot` do not move.
- **Q2: the unit `appetite.tokens` counts.** Taken: *"`appetite.tokens` is the sum of `Usage`'s four counters,
  about 24M per pass at first."* The four are `input_tokens`, `output_tokens`, `cache_creation_input_tokens` and
  `cache_read_input_tokens`, carried verbatim. Cache reads count. This does not re-open F14's question of whether
  they draw the window; the envelope only needs a sum that every row can compute without a price model. §8h.9 gives
  the first value.
- **Q3: how an envelope refusal leaves the CLI.** Taken: *"`Exit::Overage` (20)."* Its doc already reads *"A budget
  or billing state that does not improve by being repeated"*, so the exit snapshot does not move and the
  register's exit table takes no amendment. The draft's kernel class, one `EnvelopeError` variant per counter,
  stands as this file's construction. Finding 12 names the existing `StepError::Overage` that the implementer must
  weigh first.
- **Q4: probe 2's dispatch count.** Taken: *"probe 2 is three dispatches, one per model id."* §8h.11 authors it that
  way. A single dispatch would verify only one model id.

### 8h.1 The records this stage writes

Both records are new. Each takes **the next free id at writing** (no number is pre-assigned; `00-preamble.md` names
`0165` as the next free at this file's authoring, and other sub-stages allocate in writing order). Both use ADR-0164's
form — frontmatter, `## Context and problem`, `## Decision drivers`, `## Considered options` with `### Option — <name>`
headings (D21-42 24a), `## Decision`, `## Consequences` with a **What this forbids** block, `## Evidence and
confidence`. Checkpoint 8h reads the two ids from its `REC_A` and `REC_B` lines, written into the block by a dated edit
on the day each record is written (D21-47: *"Records are located by the id recorded at write time"*). *(dated 2026-10-02, s8h build)* Record A is **ADR-0197** (`references/adr/0197-cards-models-and-the-run-environment-are-least-privilege-pinned-and-launched.md`) and Record B is **ADR-0198** (`references/adr/0198-rt-57-pack-scalars-the-per-item-envelope-and-gate-expiry-are-recorded-fail-closed.md`), the ids free at writing in that order, both written `status: proposed` by package P1-records; `REC_A=0197` and `REC_B=0198` are in Checkpoint 8h's INPUTS. Record A's AC-133 table carries placeholder rows that the build completes by dated line before the operator signs (its decision 7).

**Record A — cards, models and the run environment** (`proposed`; **accepted only after both §8h.11 probes pass at
`s8j`**, on the operator's dated ledger line, D21-41 (g)). It decides, as `## Decision` sentences:
the grant table of §8h.2; the closed permission-mode set and the shipped pair; the `[models]` shape and the three
ids (§8h.3); **RT-27 settled** — adoption requires exact equality of the reported and requested model id, which
retires the segment match; the schema route (§8h.4) and the AC-133 judgement classification of each enum on the
types this stage adds, entered in `s8g`'s registry and **signed by the operator per enum** (AC-133's own limit:
*"that classification is the human gate AC-156"*); the three guards, N17's Q1 mapping and D21-45's protected-segment
limit named as a known limit (§8h.5); the setting-sources and confinement settings (§8h.6); the launcher and the
launch record as the only entry to a real run (§8h.7); the freeze (§8h.8). **It supersedes in part** ADR-0126
decision 1 — its payload table gains `observed_model` (a member added to a `## Decision` table is a supersession,
ADR-0092 r1) — and ADR-0074 decision 1, whose finding shape *"`{ ac, claim, evidence (advisory label), rationale,
files[], repro … }`"* gains `severity` under the same rule (§8h.4; D21-47 cross #4). Nothing else of the error set
is superseded: N17 took Q1 as reuse, so ADR-0095 decision 2 and ADR-0112 stand. It supersedes **nothing** of ADR-0084 (the schemas are Rust types in `EXPORTED`, which is what its golden-snapshot clause binds: *"Every type that crosses the closed-output boundary has
its generated `schemars` output snapshotted with `insta`"*) and nothing of ADR-0156/0162, whose bars on
`.redkiln/schemas/` are stage-scoped and are honoured anyway (no such directory is created). It records that
**landing is not decided here** (D21-41 (b), `s8e`/`s8f`) and that the probes' spend is D21-40 decision 2's. It
also records that the run-worktree placement is not decided here either: that is D21-44's, written as `s8d-ii`'s record,
and Record A only cites it.

**Record B — RT-57's pack scalars, the per-item envelope, and gate expiry** (`proposed` at writing; the operator
signs **each value and each gate row**; it flips to `accepted` on the operator's dated ledger line **at this stage's
close, before `s8i`**, because D21-42 19c says *"written and signed before any spend"* and `s8i` spends). It carries
§8h.9's value table with a warrant per value, §8h.10's per-class table (AC-276: *"the choice is made and signed by a
human, per class"*), and fixes the key spellings RT-57 routed to *"7e's pack schema record"*. It supersedes nothing.

### 8h.2 The cards — least privilege, exactly

| card | `tools` | `permission_mode` | reached by (§8h.4's walk) |
| --- | --- | --- | --- |
| `implementer` | `Read`, `Glob`, `Grep`, `Edit`, `Write`, `StructuredOutput` | `acceptEdits` | both lanes |
| `investigator` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `reviewer` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `architect` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `spec-author` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `planner` | `StructuredOutput` | `default` | both |
| `releaser` | `StructuredOutput` | `default` | both |
| `operator` | `StructuredOutput` | `default` | **neither** — `operate` is `forbidden` in both packs |

No card gets `Bash`, a `Web*` tool or an `mcp__*` tool. Build and test stay on the `Toolchain` port. The card
record stays closed (`deny_unknown_fields`), and `model`/`effort` stay a tier leak (AC-118, AC-418). The `cwd` is the
run-owned worktree, already carried on the spec (RT-21's `WORKTREE_KEY`, `runner.rs`'s
`.current_dir(&prepared.worktree)`). Probe 1 reads it back from `system/init`'s `cwd`. After `s8d-ii` that path is
`.redkiln/worktrees/<item>` (D21-44), which is not under `.git`.

**The closed permission-mode set.** `lint_card` refuses a mode outside the CLI's own set. That set is cited to
`claude --help`, which is the CLI's argv parser, and not to an SDK enum: *"(choices: "acceptEdits", "auto",
"bypassPermissions", "manual", "dontAsk", "plan")"* [measured 2026-09-24, `claude --help` on 2.1.281].
- *Corrected 2026-09-24 (D21-47, s8h review #9):* the earlier revision cited a `["default",…]` array found by string
  search. That array is the SDK's enum, not the argv choices.
- The binary maps `manual` to `default` [measured by string search: the literal `"manual"?"default"`]. So `default`
  is the name the init reports for that mode, and it is the name D21-41 (g) uses (*"read under `default`"*).
- The admitted set is therefore the help's six plus `default`, seven in all. A misspelling is refused, and a valid
  mode is not.
- *D21-48 (2026-09-24):* **the argv spelling is decided, and it is the pinned CLI's.** This bullet read *"Whether the
  2.1.281 parser still accepts `--permission-mode default` on the argv is UNMEASURED"*, and it left the spelling to
  probe 0 (finding 21). D21-48 X12 takes it: *"The adapter emits the spelling the PINNED CLI's `--help` lists, fixed
  at s8j's pin move (`manual` on 2.1.281). Cards keep `default`, and the init comparison reads `default`. The golden
  argv snapshots move once, at the pin move. Probe 0 confirms rather than decides."* The table below does it.

**Only the shipped pair `default` / `acceptEdits` is admitted in a shipped card**,
and that is the doctor row's job (§8h.10), not the adapter's. A test pack may legitimately use another mode.

**The argv spelling table (D21-48 X12).** The card says `default`, and the argv says what the pinned CLI's `--help`
lists.
- **One closed table in `command.rs`**, declared in exactly this form, because Checkpoint 8h (9) and `s8j`'s
  Checkpoint 8j (6) read it: `pub(crate) const PERMISSION_MODE_ARGV: &[(&str, &str)] = &[`, then one row per line as
  `    ("<CLI version>", "<argv spelling of a card's default>"),`, then `];`. Beside it,
  `pub(crate) const LIVE_WARRANTED: &[&str] = &[…];` names the rows whose warrant is a live acceptance rather than a
  help capture.
- **The lookup is keyed by the `Config`'s pin**, `tested_cli_version`, which is D17-11's `Config` value and which
  `TESTED_CLI_VERSION` fills in `real_harness_config`. It never reads the constant directly, so a test can build a
  `Config` at another pin. `assemble` puts the row's spelling on the `--permission-mode` element when the card's mode
  is `default`, and the card's own spelling for every other mode. The 2.1.281 help lists `acceptEdits` as spelled.
- **A pin with no row is refused** by `assemble`, before any spawn, with a typed error that names the version. There
  is no fallback to the card's spelling, so a pin move that forgets its row fails red and never re-spells silently.
- **Only the argv element moves.** These all keep `default`: the card file, `AgentCard.permission_mode`, the
  `--agents` JSON's `permissionMode`, the card digest (and so the memo key), `lint_card`'s closed set, the doctor
  row's shipped pair, and guard 2's init comparison (§8h.5). The binary maps `manual` to `default` (above), so the init
  reports `default`, and the comparison reads `default` against the card's `default`. X12 names the argv. The
  `--agents` JSON is not the argv parser's surface, and probes 0 and 2 read its effect from the init.
- **The rows at this stage are two:**
  - `("2.1.278", "default")`, the pin this stage builds against. Its warrant is a live acceptance, not a help
    capture. The 2026-09-21 `live_dispatch_probe` at 2.1.278 passed with
    `tests/fixtures/cards/catalog/probe.json`'s `"permission_mode": "default"` on its argv (`PROGRESS.md`, the Claude
    Code instrument row). No 2.1.278 `--help` was captured, and that binary is no longer the installed one. So this
    row is the single entry of `LIVE_WARRANTED`, and the help test below exempts it by that literal.
  - `("2.1.281", "manual")`, warranted by the help capture below. It is dormant here: nothing at this stage emits
    `manual`. *(dated 2026-10-02, s8h build)* At stage open the installed CLI read `2.1.287`, so by the rule below this row is
    `("2.1.287", "manual")` and its capture is `cli-help/2.1.287.permission-mode.txt` (§8h.0's readings, items 6
    and 13); no `2.1.281` capture exists. Record A (ADR-0197) decision 3 states the two rows. This stage's own golden argv changes (`--setting-sources`, `--settings`) leave the `--permission-mode`
    element at `default`.
- **The help capture is a fixture**, `crates/rk-adapter-claude/tests/fixtures/cli-help/<version>.permission-mode.txt`.
  It holds the lines of `claude --help` that document `--permission-mode`, through the `(choices: …)` list, written
  at stage open (§8h.0 item 6). At 2026-09-24 the list is the one quoted above. If the installed CLI is not 2.1.281 at
  stage open, the help-warranted row and its capture are keyed by the installed version, and the row's spelling is
  read from that capture:
  - `manual`, if it lists `manual`;
  - `default`, if it lists `default` and not `manual`;
  - if it lists neither, the stage stops and reports, because X12's premise moved.

  Outside `LIVE_WARRANTED`, rows and help captures are one to one.
- **At `s8j`'s pin move** (`s8j` §8j.6 step 1), in the pin-move commit and nowhere earlier:
  - the `2.1.278` row is deleted and `LIVE_WARRANTED` becomes `&[]`. The table then holds exactly one row, keyed by
    the new pin and warranted by its help capture. If the pin moves to a version other than the help-warranted
    row's, that row and its capture are replaced by a fresh capture at the new pin, by the rule above;
  - the golden argv fixtures whose card says `default` (`tests/fixtures/argv/golden-default.argv.json` and
    `golden-static.argv.json`) and the `insta` snapshots `it__command__golden_argv_default.snap` and
    `it__command__golden_argv_static.snap` move `default` → the row's spelling in the `--permission-mode` element, and
    in no other element. That is X12's *"The golden argv snapshots move once, at the pin move"*;
  - `command::argv_permission_mode_reads_the_row_of_the_configs_pin`'s `2.1.278` arm becomes a refusal arm (§8h.12).
- **`command::ac214_permission_mode_explicit`'s value assertion** reads *"the value is the card's"* today. This stage
  amends it, by a dated line in the test, to compare against the table's spelling of the card's mode at the fixture
  `Config`'s pin. At 2.1.278 the compared value does not change, so the amendment moves no reading here, and the pin
  move does not touch it again.
- Record A states the table as a construction under D21-41 (g), and cites X12.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **The eight shipped cards are this table.** Each
`.redkiln/cards/<role>.json` carries exactly the table's `tools` (as a set, no duplicate) and its `permission_mode`
(`acceptEdits` for `implementer`, `default` for the other seven), no `model` or `effort`, and loads through
`AuthoredCard::from_file` (the closed-set lint included). The `spec-author` prompt now reads *"Write the enumerated,
testable acceptance criteria; every criterion carries an authored, non-empty verification stating how the criterion is
shown to hold. …"* (D21-43 N24; §8h.4). `cards::shipped_cards_are_the_least_privilege_table` reads them with
`serde_json`; `RK-D-CARD-GRANT` (§8h.10) is the doctor half. All eight files stay LF.

### 8h.3 Models — the `[models]` table, full ids, and `observed_model` (D21-42 15a, 15b)

Each installed pack gains one table, so both packs stay independently complete, as their timeout rows already are:

```text
[models.haiku]    id = "claude-haiku-4-5-20251001"   accepts_effort = false
[models.sonnet]   id = "claude-sonnet-5"             accepts_effort = true
[models.opus]     id = "claude-opus-5-5"             accepts_effort = true
```

(Shown flattened. Each row is a TOML table with the two keys. Record A fixes the spellings.)
**`Tier` becomes the open set of `[models]` row keys.** `RoleExtras.model` becomes the row key (a string), and the
closed `card::Tier` enum leaves the binding path. `Tier::accepts_effort`'s lifted rule (*"haiku rejects the
parameter"*) becomes the row's `accepts_effort`, and its warrant is carried beside the haiku row.
`EffortUntraceable` / `EffortRefusedByTier` keep their meaning, keyed on the row. The `Effort` enum stays closed.
A role naming a key absent from `[models]` is refused at bind, naming the role and the key. The three ids appear as
string literals in the installed binary [measured 2026-09-24 by string search: `"claude-sonnet-5"` 54,
`"claude-opus-5-5"` 43, `"claude-haiku-4-5-20251001"` 15]. The 2.1.278 init reported `model` as the full
`claude-haiku-4-5-20251001` when the argv carried `haiku` (`PROGRESS.md`'s Claude Code instrument row, the 2026-09-21
probe). **Whether the child
reports exactly the requested id when `--model` carries a full id is UNMEASURED**, and probe 2 measures it.

**The role rows**, per D21-41 (g) as kept by D21-42 15b:

| role | `model` | `effort` |
| --- | --- | --- |
| `implementer`, `investigator` | `sonnet` | `high` |
| `spec-author` | `sonnet` | `medium` |
| `reviewer` | `opus` | `high` |
| `architect` | `opus` | `medium` |
| `planner`, `operator`, `releaser` | `haiku` | *(none)* |

`family = "claude"` stays. The breaker's partition key is D-7's, and it does not move.

**The full id travels everywhere the tier word did.** It goes into `--model`, into `AgentCard.model`, and so into
`to_agents_json()` and `AgentCard::digest()`. That digest is `ActivitySpec.card_digest`, so it enters the per-step
memo key D21-42 3a names. **Adoption becomes exact equality**: the init's `model` must equal the requested id, and
`model_belongs_to_tier` is deleted rather than left beside it. **`DispatchRecord` gains a typed `observed_model`.** It
is `Some` on every path that read a `system/init` carrying `model`, and `None` on every path that terminated before
one was read. It is never the requested id copied in. That would be Halt 11's shape, the same reasoning that gives
`cli_version` its `Observed`/`Pinned` arms. This is Record A's supersession of ADR-0126 decision 1 in part.

*(dated 2026-10-02, s8h build, package P3-adapter-argv-models)* Built: `card::ModelRow` (`id`, `accepts_effort`, `deny_unknown_fields`) and
`card::Models`; `card::Tier` deleted; `binder::Role.model` and `RoleExtras.model` are row keys; `BindRequest.models`
is the pack's table (`PackExtras.models`, `#[serde(default)]`); a key the table lacks is `ConfigError::ModelKeyAbsent`
naming the card, the role and the key; the bound card carries the row's full id. The eight `pack-min` fixture packs
gained Record A's three rows. The SHIPPED packs' `[models]` tables and role rows are not written by this package
(`cards::` is rk-cli's package); until they are, a shipped-pack bind refuses at `ModelUntraceable`, as it did at
entry (their `[role.*]` rows carry `family` alone). `model_belongs_to_tier` (adoption) is the isolation package's
(M3); with the full id on the card, its `reported == tier` arm is exact equality already.

*(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* **Adoption is exact equality, and `observed_model` is read.** `init::decide` compares the reported
`model` to the bound card's full id on BOTH agent readings (an `agent` key equal to the card, or the 2.1.277
`agents` list carrying it); an absent `model` is `NotAdopted`, and so is a different spelling of the same model
(`claude-haiku-4-5` for `claude-haiku-4-5-20251001`). `model_belongs_to_tier` is deleted (Checkpoint 8h (9):
0 code hits). The drain records the init's `model` before any decision, so `DispatchRecord.observed_model` is
`Some(reported)` on every path that read an init carrying it (a refused one too) and `None` otherwise (pre-spawn,
no init, an init without `model`); mutant check I3 (the requested id copied in) is killed. Every scripted init in
`rk-adapter-claude`'s `tests/fixtures/streams/` now reports the full id (twelve files, README dated), and the seven
`rk-cli` `*frames.jsonl` transcripts' init lines report `claude-sonnet-5` and `acceptEdits` (the `pack-min`
implementer card's row and mode; JSONL carries no comment, so recorded here): `cost/run-frames.jsonl`,
`exit/{findings-block,platform-is-error,schema-invalid-structured-output,schema-no-structured-output}/frames.jsonl`,
`pack-min/lane/implement-frames.jsonl`, `slice/approved/frames.jsonl`.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **The shipped packs' `[models]` tables and role rows are
written.** Both installed packs carry `[models.haiku]` (`claude-haiku-4-5-20251001`, `accepts_effort = false`),
`[models.sonnet]` (`claude-sonnet-5`, `true`) and `[models.opus]` (`claude-opus-5-5`, `true`), and the eight `[role.*]`
rows above carry `model` (a row KEY) and `effort` as this section's table states, `family = "claude"` kept; the three
haiku roles carry no effort. `cards::the_models_table_pins_the_three_full_ids` and
`cards::shipped_role_rows_resolve_through_the_models_table` read them through `rk-cli`'s own `PackExtras`. A
shipped-pack bind therefore no longer refuses at `ModelUntraceable` (P3's note above).

### 8h.4 Schemas — Rust types in `EXPORTED`, read by name (D21-41 (g); ADR-0084, ADR-0095, AC-133)

**The kinds the two lanes reach.** These are enumerated mechanically, with the rule stated so it reproduces:
- For each installed pack (`bug-standard` → the fix lane, `story` → the story lane), take every stage whose rigor
  is not `forbidden`. Collect its `baseline` and every `require` row's `artifacts`.
- Close that set over `requires` through every catalog activity tagged `kind:bug` or `kind:story` respectively
  that `provides` a member.
- Collect the `value:<Kind>` of every `kind = "agent"` step of those activities.

[measured 2026-09-24 at `ca8f387`: fix lane 18 kinds and story lane 17 over 38 candidate activities each. The union
is **24**. Every agent-step `outputs` in the set has the `value:` form.] This is a **candidate universe, not a
selection run.** Facets unknown at run time can route either way, so a selection-run set would miss kinds that a
different facet reading reaches. Every kind in the universe gets a type, and the binder refuses any kind outside
`EXPORTED`, so the direction is fail-closed.

*(dated 2026-09-27, `s8d` build, package F2-D2173; receiver note, `spikes/DECISIONS-2026-09-21.md` D21-73 item 4.)* Two of
`s8d`'s owed items touch this section's types, and `s8g` §8g.1's receiver note owns both. (a) `Toolchain::lint` returns `()`,
although the shipped `lint` row declares `outputs = "LintReport"`, so `bdd-scenarios`' and `docs-update`'s `policy` steps
refuse on an unbound `lint` output. If `s8g` gives `lint` a typed output, `LintReport` is a `tool`-step kind and not an
agent-step `value:` kind, so the rule above does not enumerate it; it joins `EXPORTED` only if a card or a binder reads it
by name. (b) `confirmation-test`'s `emit` composes `ConfirmationResult` from `${before.outputs.selection}` and `.sha`, which
record T's `TestOutcome` does not carry. Whatever `s8g` settles, this section's `ConfirmationResult` type must match it. Nothing
here moves until `s8g` lands. Selection itself changed under `s8d`'s R8 (ADR-0179, `proposed` *(dated 2026-09-27, `s8d` close: accepted on D21-86)*: lowest rank, one node per
activity, `rigor` bound). This section's universe is a closure over `requires`, not a selection run, so R8 does not move
the 24.

*(dated 2026-09-29, `s8g` build, package R-D2124-D2127-8g; receiver note, `spikes/DECISIONS-2026-09-21.md` D21-126,
TAKEN PROVISIONALLY under D21-105.)* **(a) is settled with no typed `lint` output.** `s8g` retired the two checks by
ADR-0072 edits (`bdd-scenarios@5`, `docs-update@5`: the `toolchain:lint` step and the `policy` conjunct that read it
deleted) rather than widening the port, so `LintReport` stays a registry-declared kind with no `rk-types` type, it
does not join `EXPORTED`, and nothing here moves for it. (b) was settled by `confirmation-test@2`'s catalog edit
(`s8g` §8g.1's S8G-P7-DEMANDS disposition): `ConfirmationResult`'s members are unchanged.

| kind | lanes | role(s) | kind | lanes | role(s) |
| --- | --- | --- | --- | --- | --- |
| `ADR` | story | architect | `ImplReport` | both | implementer, investigator |
| `APIContract` | story | spec-author | `Postmortem` | fix | investigator |
| `ArchitectureDescription` | story | architect | `ReproAttempt` | fix | investigator |
| `CausalChain` | fix | investigator | `ReproCase` | fix | investigator |
| `ChangeRecord` | both | releaser | `Retrospective` | both | implementer, investigator |
| `DebtRegisterEntry` | both | implementer | `RolloutPlan` | both | releaser |
| `DesignDoc` | story | architect | `RootCause` | fix | investigator |
| `Docs` | story | implementer | `SelfCheck` | both | all but operator |
| `Estimate` | both | planner | `Spec` | both | spec-author |
| `Findings` | both | reviewer | `TestDesign` | both | implementer |
| `FixDesign` | fix | architect | `ThreatModel` | story | architect |
| `GherkinScenarios` | both | spec-author | `HypothesisLog` | fix | investigator |

**Field shapes.** `spikes/p9/schemas.toml` (*"Stand-in for schemars types: value schema name -> fields; list fields
flagged"*) carries a `[value.<Kind>]` row for **all 24** [measured]. Its field names and list flags are the
transcription source. The Rust types are the implementer's, under ADR-0095 decision 1's rule: no `oneOf`, no nested
union, `additionalProperties: false`, `required` first, ≤ 16 union-typed and ≤ 24 optional parameters per exported
schema. Each type is registered in `EXPORTED` under its kind name, and the golden snapshot (`schema_snapshot`) takes
it (ADR-0084). An `Option` is not a default: a field the stand-in lists is required unless Record A names why not.

**AC-133's registry is `s8g`'s, and this stage enters its types' enums in it.** D21-43's integration call: *"AC-133's
judgement-enum registry is built once, by **s8g**. s8h consumes it."* `s8g` builds that registry to AC-133's
*Verified by* (`s8g` §8g.3): a completeness test over every enum reachable from `EXPORTED`, the rationale-sibling
rule, and a must-fail fixture. AC-133 wants *"the enum appears in the judgement-enum registry with an explicit
judgement-bearing yes/no AND every enum marked yes has a sibling free-text `rationale` field"*. *(dated 2026-10-02, `s8g` close: the registry is built, under ADR-0192 decisions 3 and 8, accepted on D21-132; its tests are `judgement_registry::ac133_every_exported_enum_is_registered`, `judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails` and `judgement_registry::the_registry_names_no_enum_that_is_not_exported`, and `rk_types::AcceptanceCriterion` is exported for `Spec` to embed unchanged as `acceptance_criteria`)*

Every type this stage adds to `EXPORTED` brings its enums with it, so `s8g`'s completeness test goes red until each
enum is entered with a yes/no and every "yes" has its `rationale` sibling. That red is this stage's instrument. No
second registry and no second completeness test are written. The yes/no **values** are the operator's (AC-156),
signed in Record A. The tests check that entries exist, never that the values are right.

**`Spec` embeds `AcceptanceCriterion` unchanged.** `s8g` §8g.3 sets the constraint: *"`s8h`'s `Spec` must embed
`AcceptanceCriterion` unchanged, under that member name"*. `Spec`'s criteria member is `acceptance_criteria:
Vec<rk_types::AcceptanceCriterion>`, and nothing in it is redeclared. That is the name the shipped catalog already reads
(`fix-spec.toml`'s check, and the `for_each` of `tdd-red-green-refactor.toml` and `mutation-testing.toml`) and the
stand-in's `[value.Spec]` lists [measured: `spikes/p9/schemas.toml` lists `"acceptance_criteria"`]. *Final consistency
pass 2026-09-24:* the earlier text named the member `criteria`, which `s8g` §8g.3 reported (D21-47 review, finding 15).

**Every authored criterion carries a `verification`** (D21-43 N24, as `s8g` §8g.3 hands it on; final consistency pass
2026-09-24). Both packs ship `verification_required = ["verify"]` (`s8g`), so an AC whose `verification` is `None`
refuses at verify. `AcceptanceCriterion` keeps `verification: Option<Text>`, because AC-137 accepts a criterion with
none, so the type cannot supply it. The authoring does:
- **The `spec-author` card's output contract** says that every criterion carries an authored, non-empty
  `verification`, stating how the criterion is shown to hold. The card's `prompt` says so.
- **The ingest check.** When an agent step's structured output is of kind `Spec`, the adapter's structured-output
  validation refuses it after the schema validation if any criterion's `verification` is absent or blank. The class
  is `StepError::Schema`, the one a structured output that fails its contract already takes, so ADR-0112's closed set
  does not move. The check reads the authored value only. A `Spec` read back from the store is not re-checked, so
  `s8g`'s `verify_pass::ac139_shipped_packs_block_a_missing_verification` fixture still reaches the policy row.
- **The test** is `classify::a_spec_criterion_without_an_authored_verification_is_a_schema_failure` (§8h.12), and
  mutant M29 skips the check (§8h.13). `s8j`'s fixture proof asserts the same over each lane's `Spec` before verify,
  which is `s8g` §8g.3's second owner.
  *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* Built: `classify::extract_digest` refuses a `Spec`-kind (`classify::SPEC_KIND`, the
  `EXPORTED` key the binder gives `value:Spec`) structured output AFTER the schema validation when a criterion's
  `verification` is absent, `null`, not a string or blank, naming the criterion, as `StepError::Schema`. Red, green
  and M29 in §8h.12 and §8h.13. The `spec-author` card's prompt sentence is the cards package's.

**`Finding` gains `severity`** (D21-47 cross #4: *"ADD `severity` to rk-types `Finding`, typed by s8b's 11c
vocabulary, with tests"*). The facts:
- 39 shipped `expr` rows read `severity`.
- The stand-in lists it: `fields = ["ac", "claim", "evidence", "severity", "files", "repro"]`.
- `rk_types::artifact::findings::Finding` has no such member at `fc80687` [measured: its members are `ac`, `claim`,
  `evidence`, `rationale`, `repro` and `files`].
- `s8b` §8b.6's 11c record names this stage as the field's owner.

The member is built as follows:
- **Its type** is a closed enum whose spellings are that record's. They are read from the record, located by its
  recorded id (§8h.0 item 12), and never from this file. In the branch where A1 passes, that is
  `blocking | advisory`.
- **It is required**, not an `Option`.
- **The reviewer card's structured output fills it**, because `Findings` is that card's output kind and the schema is
  the contract.
- **Its enum is entered in `s8g`'s AC-133 registry** like every other; its `rationale` sibling already exists.
  `schema_snapshot` moves.
- **It supersedes in part.** The member is added to ADR-0074 decision 1's finding shape, so Record A supersedes that
  decision in part (§8h.1).

Once the member lands, a `severity` row over a non-empty findings list reads the typed member instead of failing with
`UnknownField`. Over an empty list the row stays true without reading it, which is `s8b`'s corrected 39-row
statement.
*(dated 2026-09-26, `s8c` build, D67; `spikes/DECISIONS-2026-09-21.md` D21-67 ruling 1.)* **The 39 rows are
disposed of in `s8d`, before this stage types `severity`.** D21-67 ruling 1 answers D21-57: *"a severity-only row
without re-execution does not conform"* to ADR-0080 d2, and *"`s8d` carries the edits (39 version bumps and 39
`.redkiln/CHANGELOG.md` rows under ADR-0072), before `s8h` types `severity` and makes them live"*: six review-family
`verdict` rows rewritten to `adversarial-review@4`'s `record.blocking.len() == 0` form, `migration-dry-run`'s
`policy` step deleted, and 32 looping `policy` conjuncts' severity conjunct removed under ADR-0080 decision 4. So at
this stage's open no shipped `expr` reads `severity`, and *"39 shipped `expr` rows read `severity`"* above is
history; re-measure it at open (`grep -rn "severity in" .redkiln/catalog`, expected 0). **The ruling re-points this
stage's severity test to a fixture:** *"`s8h`'s severity test re-points to a fixture"*.
`quality_block::a_severity_row_reads_the_typed_member_over_a_non_empty_list` (§8h.12) reads a fixture activity
carrying `findings.none(severity in [...])` (`rk-kernel`'s `tests/fixtures/activities/`, beside
`terminal_condition_is_severity_only.toml`), never a shipped row. Its name and its place in the counts do not move.
*(dated 2026-10-02, s8h build, package P5-kernel-envelope)* Built: the fixture is `crates/rk-kernel/tests/fixtures/activities/terminal_condition_reads_typed_severity.toml`,
whose `policy` check binds `findings = "${adversary.outputs.items?}"` (the LIST: `Findings` serialises as `{ items: [...] }`
and a quantifier over the record is `NotAList`) and reads `findings.none(severity in ['blocking'])`. The test ingests the
finding through `rk_types::artifact::findings::Findings::ingest`, serialises the TYPED value, binds it through the kernel's
own `runtime::reference::check_context` and evaluates it with the one `predicate::evaluate`: one `blocking` finding is
`false`, one `advisory` finding is `true`, and the empty list (the control) is `true`. P2 landed `Finding.severity` before
this test was written, so its red was taken from an instrument (I6 in §8h.13: the member kept off the wire), which reads
*"a severity row over one blocking finding must read the typed member and be false, got Err(UnknownField("severity"))"* —
the stated reason.

**`RootCause` carries `discriminating_test`** (D21-47 cross #5). `s8b` §8b.5's (C) rows bind
`${compare.outputs.discriminating_test}` and `${analyse.outputs.discriminating_test}`. Both steps output `RootCause`,
and its stand-in lacks the field.
- `RootCause` is transcribed with the field as a required string. It is the test selection that `confirm` re-runs.
- The stand-in's `[value.RootCause]` row gains it, with a dated comment. The edit to `spikes/p9/schemas.toml` is safe
  under D21-47 V2, because every test that pins a historical p9 measurement reads the p9 tree at its pinned commit.
- Until the field lands, the resolver refuses those bindings by name (ADR-0136). That is `s8b`'s stated interim.
  *(dated 2026-09-25, s8b refuting round 3: `s8b` §8b.5 corrects that interim. No resolver exists before `s8d`, and
  ADR-0136 governs only a `when` guard's `${…}`; before `s8d` the runtime's tool arm refuses `confirm` under ADR-0148,
  and from `s8d` until this field lands `s8d`'s resolver has no field to bind.)* *(dated 2026-09-25, s8b refuting
  round 4: before `s8d` the walk never reaches `confirm`. Read from `runtime/step.rs`, not run, it stops at
  `differential-debugging`'s `stuck` (its `when` reads an unresolved `${…}`, ADR-0136) and at
  `instrumented-trace-analysis`'s `collect` (unmapped verb `trace_collect`, ADR-0135). The tool arm is what would
  refuse `confirm` under ADR-0148 were it reached; `s8b` §8b.5 carries the same note.)* *(dated 2026-09-25, s8b
  refuting round 5: each walk reaches that step only if its first agent dispatch, `compare` or `instrument`, commits;
  otherwise it ends there. `confirm` is unreached on every path.)*

**The binder reads by name.** `bind_nodes` (or `s8d`'s per-step successor) resolves the schema as
`rk_types::registry::EXPORTED` entry named by the bound step's output kind. A kind that is absent is a bind-time
refusal naming the kind and the step (`Exit::ConfigError`), before any spawn. `.redkiln/schemas/` is not created,
and the file read is deleted.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **Built.** `bind_one` (`crates/rk-cli/src/verbs/run.rs`, reached
from `bind_nodes` once per agent step with the step's own `value:<Kind>`, §8h.0 item 2) takes the schema from
`rk_types::registry::EXPORTED` by that kind name (`exported_schema(kind)`); the `.join("schemas")` /
`.join(format!("{kind}.json"))` read is deleted (Checkpoint 8h (9)'s floor reads 0 code hits, its filter proved on a
synthetic pair first). A kind `EXPORTED` lacks refuses with `Exit::ConfigError` naming the step, the node and the
kind, before any spawn. Fixture consequences, each by dated line: `pack-min/schemas/` (`Findings.json`,
`ImplReport.json`) is DELETED and `support::pack_min_repo` creates no `.redkiln/schemas/`; `root.rs`'s direct `Binder`
test takes `EXPORTED`'s `ImplReport`; `landing.rs` writes no schema files; and, because the real adapter validates
a structured output against the bound schema, `pack-min/lane/implement-frames.jsonl`'s result frame now carries
`EXPORTED`'s `ImplReport` shape, `{"status":"implemented","files":[],"summary":"the lane fixture's implement stage",
"tests_added":[]}` (was `{"item":"lane-1","summary":…}`; JSONL carries no comment, so recorded here). The two
`{"item":7}` frames (`cost/run-frames.jsonl`, `exit/schema-invalid-structured-output/frames.jsonl`) still fail the
schema, which is what they exist for.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **The universe rule, made to reproduce.** The 2026-09-24 reading
(18 / 17 / 24) reproduces at this tree only when the closure follows REQUIRED inputs: a `requires` entry spelt
`X?` is optional and does not extend the closure. Following the optional entries too reaches 19 / 19 / 26
(`DataMigrationPlan` through `design-doc`'s and `rollout-plan`'s `DataMigrationPlan?`, `UXDesign` through
`design-doc`'s `UXDesign?`). `schemas::every_agent_output_kind_the_two_lanes_reach_is_exported` states the rule in
its helper and asserts 18, 17 and 24 as premises. The candidate activities now number 45 (`kind:bug`) and 47
(`kind:story`), not 38, and the 24 does not move.

*(dated 2026-10-02, s8h build, package P2-rk-types; Record A = ADR-0197 decisions 6 and 7, `proposed`.)* **The type half
is built.** `rk_types::registry::EXPORTED` now registers 25 entries: `Findings` and `AcceptanceCriterion` (`s8g`'s,
registered once, never redeclared) and the 23 new kinds of `crates/rk-types/src/artifact/lane.rs`, each under its kind
name (`ADR` and `APIContract` carry those schema titles over the Rust names `Adr` and `ApiContract`). `EXPORTED` and
`EXPORTED_NAMED` are now generated from ONE table by a `macro_rules!` in `registry.rs`, so a type cannot enter one list
alone; `judgement_registry::ac133_every_exported_enum_is_registered` still asserts the pair. Every stand-in field is
required (no type declares an optional member). The typing rule is stated in `lane.rs`'s module comment: a list is a
`Vec`; a scalar is a closed enum only when a shipped `expr`/`when`/`until` compares it to a literal or a catalog row,
checklist or record names its complete vocabulary, a `bool`/count when a shipped expression reads it as one, a
`String` when it is a machine token (a command, a test selection, a path, an AC id), and free `Text` otherwise. Eleven
enums enter `s8g`'s registry (five drafted `yes`, each with a required `rationale` sibling on its object, which the
stand-in did not list: `RootCause`, `CausalChain`, `ReproCase`, `Estimate`, `ReproAttempt`, `HypothesisLog`, `Docs`);
Record A decision 7's table is completed by dated cells (four candidates struck to `Text`). `Spec.acceptance_criteria` is
`Vec<rk_types::AcceptanceCriterion>`, unchanged; `GherkinScenarios.acceptance_criteria` embeds the same type, since the
stand-in lists the member there too. `HypothesisLog.root_cause` is a `RootCause`, because `root-cause`'s emit writes
`RootCause = "${hypothesise.outputs.root_cause}"`. `Finding.severity` is required and typed `FindingSeverity`
(`blocking | advisory`, ADR-0168 decision 1) — named so because `rk_types::verb_output::Severity` is the CVSS band and
the registry keys enums by schema name. `RootCause.discriminating_test` is a required `String`, and
`spikes/p9/schemas.toml`'s `[value.RootCause]` gains it with a dated comment. `schema_snapshot` moved: `exported__Findings`
gained `severity`, and 23 new `exported__<Kind>.snap` were reviewed (no `oneOf`; `Spec` and `GherkinScenarios` carry the
one `anyOf` of `AcceptanceCriterion.mutation`) and accepted. **Fixtures amended for the required member**, each by a dated
`//` line where the format admits one: the `Finding` constructors and JSON payloads in `rk-types`' `repro_ingest`,
`review_gate`; `rk-kernel`'s `retry`, `review`, `review_confirm`, `verify` (the `verdict`-key refusal fixture gains
`severity` so its refusal stays the key's); `rk-store`'s `artifacts`; `rk-cli`'s `review_pass`, `verb_rows`; and two
JSON fixtures with no comment syntax, recorded here: `crates/rk-cli/tests/fixtures/exit/findings-block/frames.jsonl`
(the review frame's finding gains `"severity":"blocking"`) and `crates/rk-cli/tests/fixtures/pack-min/schemas/Findings.json`
(the file `bind_one` still reads until this section's binder change deletes the read: `severity` added to `required` and
`properties`). `crates/rk-kernel/tests/fixtures/findings/ac310_f31_narrowed_pin_seed.json` is historical F31 data, not
a `Finding` payload, and is untouched. `DispatchRecord`/`RecordTail` gain `observed_model: Option<rk_types::usage::ObservedModel>`
(§8h.3's type half): every literal across `rk-kernel`, `rk-store`, `rk-ports` and both `rk-adapter-claude` sites sets
`None`, and the adapter's init reading is left for its own red-first test (§8h.12's
`dispatch::observed_model_is_recorded_on_every_path_that_read_init`).

### 8h.5 The three principle-7 guards, and the auth check (D21-41 (g); D21-42 15a, 17a)

All of these read the child's own report, never the argv (AC-202's rule, the `init|isolation` module filter).

1. **The closed-set lint** (§8h.2). It runs in `lint_card`, so it applies at load and again at bind.
2. **Init equality.** `init::decide` gains three comparisons beside the existing tool-set and MCP checks:
   - `permissionMode` must equal the card's mode, and an absent key is refused, not treated as equal. The card's
     mode is `default`, never the argv's spelling of it (*D21-48 (2026-09-24):* X12, *"the init comparison reads
     `default`"*; §8h.2's table).
   - `model` must equal the requested full id (§8h.3).
   - `apiKeySource` must equal `"none"`, and an absent key is refused.

   The last is ADR-0010 checked in-band: *"The adapter must refuse to proceed (assert, not warn)"*. The field is
   named by the 2.1.277 raw init (D17-11's amendment: *"AC-511's auth-mode field is **`apiKeySource`**, read as
   `"none"` on a subscription session"*), and the installed 2.1.281 init builder still emits `apiKeySource`,
   `permissionMode`, `model`, `tools` and `cwd` [measured by string search of the binary's `subtype:"init"` object].
   The classes are N17's Q1 (§8h.0). A model mismatch is `NotAdopted`, and a mode or `apiKeySource` mismatch is
   `NotIsolated { surface: Env }`.
3. **`permission_denials` → a typed failure.** The result frame carries `permission_denials: [{ tool_name,
   tool_use_id, tool_input }]` [measured: the binary's result schema; documented in `headless` as *"the final result
   message lists them in `permission_denials`"*]. A non-empty list fails the dispatch as
   `NotIsolated { surface: Env }` (Q1), whatever the exit code. It **outranks a valid `structured_output`**: a step
   whose child was denied an out-of-worktree edit has not done what it reported. It is placed in `classify.rs`'s
   order after the platform arms and before the `structured_output` validation. An empty list is not a failure.

   **The result list is not the only channel, so it is not the only detector** (D21-47; s8h review #2).
   - In 2.1.281 the result's `permission_denials` is filled by the `canUseTool` wrapper. The binary's own schema text
     says that a path-scoped deny-rule refusal resolves before that wrapper and is *"not covered here, and neither
     is the MCP --permission-prompt-tool surface"* [measured 2026-09-24 by string search of the 2.1.281 binary; the
     literals `permission_denied`, `decision_reason_code` and `outside_reads_blocked` are present too].
   - §8h.6's deny rules and block setting produce exactly that kind of refusal.

   So guard 3 reads two channels and fails on either:
   - (a) a non-empty result `permission_denials`;
   - (b) any stream frame of the `permission_denied` system event, keyed by its `decision_reason_code` (for example
     `outside_reads_blocked`).

   The frame shape of (b) is the reviewer's reading of the binary, and it is **UNMEASURED as a live frame**:
   - The unit test's fixture drafts it, and §8h.11's probe 0 retains the real frame.
   - If the retained frame differs, `s8j` corrects the fixture and the arm by a dated line before probe 1. The arm is
     never widened to match text.
   - The digest is over the union of both channels' `(tool_name, path)` entries, de-duplicated by `tool_use_id`.
   - An errored `tool_result` that names a denial is **not** a third detector. Its text is the model-facing message,
     not a typed channel. Probe 1 prints it under `DENIAL_CHANNELS`, so a denial carried only there is seen.

   **A denial's path is extracted per tool** (s8h review #10). The entries carry `tool_input`, not a path:
   - `file_path` for `Read`, `Edit` and `Write`;
   - `path` for `Glob` and `Grep`, which is the search root, and the working directory when the key is absent;
   - `pattern` for `Glob`, only when it is absolute.

   A denial with no extractable path enters the digest as `(tool_name, "")` and never carries the note below.

**The standing limit, named (D21-45).** Claude Code's `checkPathSafetyForAutoEdit` treats a path as sensitive if ANY
case-folded segment is one of `[".git",".vscode",".idea",".claude",".husky",".cargo",".devcontainer",".yarn",".mvn"]`
or the path `.config/git` [D21-45, quoted from the 2.1.281 code]. That path is **two consecutive segments**,
`.config` then `git`, and it is matched as a pair, never as one segment name (s8h review #10). D21-45 draws the consequence for a run worktree:
*"inside a run worktree, `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/` stay protected. An
implementer edit there is denied, and s8h's typed `permission_denials` failure names it as a known limit, not a
defect."*

- **The class does not change.** Such a denial reaches guard 3 as a non-empty `permission_denials` and fails as
  `NotIsolated { surface: Env }`, like any other denial. Nothing is retried.
- **The failure carries a typed note.** When a denied path resolves inside the spec's worktree, and one of its
  segments below the worktree root is on that list, classification carries a typed note beside the error:
  `ProtectedSegmentDenial { segment, tool_name }`. The consecutive pair `.config`, `git` qualifies the same way, and
  its `segment` is `.config/git`. The adapter prints the note as a warning naming D21-45's known limit,
  the way `record::mismatch_warning` names a version mismatch. The class and the digest stay the same.
- **The segment list is one `const`** in `rk-adapter-claude`, copied from D21-45 with that citation. A CLI release
  that changes the list is Halt 2's concern, through the pin, and not this guard's.

*(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* **Built as written.** Guard 2 (`init.rs`): after the tool set, `permissionMode` must equal the
card's mode (the card's `default`, never the argv's `manual`: an init reporting `default` under a `Config` pinned
`2.1.287`, whose argv carries `manual`, is adopted) and `apiKeySource` must equal `init::API_KEY_SOURCE_NONE`;
absent keys refuse; each is `NotIsolated { surface: Env }` with N17 Q1's digests. Guard 3 (`classify.rs` step 5b,
after the platform arms 3-5 and before the exit code, so it holds whatever the exit code and outranks step 10's
`structured_output`): channel (a) `denials_from_result`, channel (b) every `system` / `classify::PERMISSION_DENIED_SUBTYPE`
line the drain collects (`denial_from_event`, keyed by `type`/`subtype`, its `decision_reason_code` kept), the union
de-duplicated by `tool_use_id` (`denial_digest`). Paths per tool (`denial_path`): `file_path` for Read/Edit/Write;
Glob's `pattern` when absolute, else its `path`; Grep's `path`; the worktree when the key is absent; `""` for any
other tool. The list is `classify::PROTECTED_SEGMENTS` (D21-45, quoted with its citation) plus
`PROTECTED_SEGMENT_PAIR` (`.config` then `git`, matched as a pair); segments are lexically normalised and
case-folded, a relative path resolves against the worktree, and only segments BELOW the worktree root count. The note
`ProtectedSegmentDenial { segment, tool_name }` rides `Classified.notes` / `Run.notes`; `dispatch` prints
`ProtectedSegmentDenial::warning()` (naming D21-45's known limit) to stderr and keeps it on
`ClaudeHarness::denial_warnings()`. The channel-(b) frame is the DRAFT this section describes, in
`tests/fixtures/streams/permission-denied-outside-read.scripted.ndjson`, unmeasured live until probe 0.

### 8h.6 Settings isolation and confinement (D21-42 21b, 21d)

**`--setting-sources ""`, always.** The 2.1.281 help reads *"Comma-separated list of setting sources to load (user,
project, local)."*, and the parser begins `if(e==="")return[];` [measured by string search]. The empty value loads
none of the three file sources. Policy settings and `--settings` flag settings are separate sources and still apply.
The element joins the golden argv in a fixed position that Record A states. `golden_argv_per_profile`'s snapshots
move, and the cap measure counts the quoted empty element. The docs site does not document the flag [a docs-reading
pass, 2026-09-24], so its effect is asserted from the init line (tools and mode equal; probe 2) and never inferred.

**The child's confinement settings.** ~~They go in one `--settings <json>` element, assembled per dispatch from the
spec's worktree.~~ *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* They go in one `--settings <path>` element. The path names a read-only file
whose bytes are the JSON. It is written per dispatch, with create_new and a unique name, into a caller-supplied
directory outside the worktree (rk-cli: ~~`<common_dir>/redkiln/runs/<run_id>/`~~ `<common_dir>/redkiln/child-settings/<run_id>/`,
the dated note at this section's end). Its sha256 and path are recorded in the dispatch's launch evidence, and it is
retained with the run. The file's location is fenced three ways: it is a denied sibling (this section's *"the main
checkout's own files are denied siblings. That includes its `.git` common directory"*), it sits under a protected
`.git` segment (D21-45: *"The path is sensitive if ANY segment is one of"* a list that begins `".git"`), and *"The child has no
`Bash`"*. [documented, `code.claude.com/docs/en/headless.md`, via the same pass: `--settings` *"can set any key
your user settings file can set"*.] It carries three things:

- **`permissions.blockReadsOutsideWorkingDirectories: true`** — [documented, `permissions.md`: *"Set
  `permissions.blockReadsOutsideWorkingDirectories` to make the file tools refuse the paths it fences in every
  permission mode"*; present in the 2.1.281 binary].
- **`permissions.deny` rules for `Read`, `Edit` and `Write` covering everything outside the worktree.** They are
  written as the **complement along the worktree's ancestor chain**: at each ancestor level, deny every sibling
  entry not on the path to the worktree, plus `//<drive>/**` for every drive letter other than the worktree's, plus
  `~/**`. **A carve-out is not used.** [documented, `permissions.md`: *"Claude Code reads a `!` pattern relative to
  the current directory even when `/`, `~/`, or `//` follows the `!`, so the pattern can't reach a rule anchored with
  one of those prefixes"*; and *"An allow rule can't carve an exception out of a deny rule."*] Paths take the
  documented Windows form (`//d/repos/...`). A sibling created after assembly is outside the set, and that
  residual is stated, not claimed away. `Read` rules govern `Glob`/`Grep` on a documented *"best-effort"* basis only,
  which is why the block setting is the primary read fence and the deny rules are the second.
- **`autoMemoryEnabled: false`** (D21-47; s8h review #7). The key is in the 2.1.281 binary [measured by string
  search: `autoMemoryEnabled`]. Auto memory reads and writes under the configuration directory on the child's own
  initiative, which is a channel the deny rules were not written for. Its effect is asserted by probe 2 from the
  memory file set (§8h.11), never inferred from the argv.

**The `.git` risk this section once carried is measured and settled.** D21-44 ran D21-43 N1's probe from the
operator's own shell:
- an edit inside `.git/redkiln/worktrees/probe-n1-in` was refused, *"**Edit AND Write**, both refused"*;
- an edit inside `.redkiln/worktrees/probe-n1b` succeeded;
- both outside-the-repository controls succeeded.

Run-owned worktrees therefore move to `.redkiln/worktrees/<item>`, which is git-ignored (`.gitignore` carries
`.redkiln/worktrees/` at `a6349b6` [measured: `git check-ignore -v`]). The move, its record and the lifecycle
around it are `s8d-ii`'s. This stage assumes only that the spec's worktree path is under that root. Probe 1
re-measures the placement with the shipped card and settings (§8h.11). The standing in-worktree limit is handled by
§8h.5's named note. The one legacy tree, `rk-fix-doctor-test-count` at `73c087a`, stays under `.git` until `s8j`
re-points it (D21-44), and nothing here dispatches into it.

**The deny chain under the new root.** With the worktree at `D:/repos/redkiln-rs/.redkiln/worktrees/<item>`, the
chain's levels are:
- `D:/`
- `D:/repos`
- the repository root
- `.redkiln`
- `worktrees`

So the main checkout's own files are denied siblings. That includes its `.git` common directory, which a run
worktree's `.git` file points into. The child has no `Bash` and runs no git, so it has no need to read there. Every
other run worktree is a denied sibling too.

**Nothing is carried into the worktree for the child's settings.** D21-45 records the same choice from the other
side: `settings.local.json` is not copied, *"because children run with isolated settings (s8h)"*.

*(dated 2026-10-02, s8h build, package P3-adapter-argv-models)* **BLOCKED in part: the `--settings` element is built and tested, and not
yet wired into a production assembly.** `command::ChildSettings` builds the element from a `ConfinementChain`
(`ConfinementChain::read` lists each ancestor level from the filesystem) and `assemble` emits it in Record A's
position when the `CommandSpec` carries one; `--setting-sources ""` is unconditional. But the complement's size is
the sum of every ancestor level's entry count, and the element rides the argv, under RT-1's 32,767-unit cap
[measured 2026-10-02 by `P3-measure-settings.py`, the rule restated]: a run worktree at
`D:/repos/redkiln-rs/.redkiln/worktrees/bug-1` is 115 deny paths, 345 rules, about 11,989 units (37% of the cap
before the schema and the card); under `%TEMP%` (4,660 entries on this machine), where every adapter and `rk-cli`
test's worktree or scratch repository lives, it is 4,999 paths and about 1,068,619 units; under `target/tmp` it is
1,104 paths and about 243,048 units. Wiring `ChildSettings::for_worktree` into `Binder::bind` and `dispatch` would
refuse every such bind at the cap. Neither this section nor Record A decision 9 (*"one `--settings <json>`
element"*) foresaw the cap, so the delivery is returned for a ruling; until then `Binder::bind` and `dispatch` pass
`settings: None`, by dated comment at each site. **RULED provisionally 2026-10-02 (panel, AFK rule): option (1), file
delivery; see the D-entry** (D21-137). The measured figures above (11,989 / 1,068,619 / 243,048 units) are the reason.

*(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* **Built.** `Binder::bind` and `dispatch` carry `settings: Some(SettingsArg { settings, path })`, and
the dated interim comments are gone. `Config::with_settings_dir` carries the directory (an input: the adapter never runs
git to find it and has no default).
- **Bind writes no file.** It builds `ChildSettings::for_worktree` (fail closed), checks the directory
  (`command::check_settings_dir`), and assembles with `command::settings_stand_in(dir)`, the directory joined with a name
  of the dispatch form at its fixed maximum length. `assemble` checks the settings against the CLI's 2 MiB file limit
  and the argv against RT-1's cap. Dispatch rebuilds the settings, re-checks the directory, assembles with the real
  path (both checks again), then writes the file LAST before the spawn.
- **The file.** `child-settings-<pid:010>-<seq:020>-<sha256 prefix:16>.json`, written with `create_new`, synced, then
  read-only; `DispatchRecord.settings_file { path, sha256 }` (rk-types, on the `dispatch_completed` row; `null` on a
  pre-spawn row).
- **The typed refusals, none falling back to `settings: None`:** `SettingsDirUnset` (none supplied);
  `SettingsDirMissing`; `SettingsDirUnderWorktree` (the worktree itself or anything under it, compared with `\\?\`
  stripped, `\` read as `/`, ASCII case-folded on Windows); `SettingsPathRefused` (not UTF-8, verbatim, relative, or
  carrying `.`/`..`); `SettingsTooLarge` (over 2,097,152 bytes, the CLI's `Kp`, found by string search, not executed);
  `SettingsWrite` (an existing name under `create_new`, or any I/O failure). At dispatch each is a pre-spawn
  `Transport { Spawn }` row, nothing spawned.
- **The red-first tests (D21-137 (a)-(g)), all in `rk-adapter-claude`:** (a)
  `command::a_five_thousand_sibling_chain_assembles_under_the_cap`; (b)
  `dispatch::the_settings_file_is_the_json_and_its_sha256_is_recorded`; (c)
  `command::a_settings_dir_under_the_worktree_is_refused`; (d)
  `dispatch::two_dispatches_write_two_distinct_settings_files`; (e)
  `command::settings_over_two_mib_are_refused_before_spawn`; (f) `command::a_missing_settings_dir_is_refused`
  (including a dispatch whose directory was removed after bind); (g)
  `binder::binds_over_temp_and_target_tmp_worktrees_succeed_and_write_nothing`; and two for the remaining refusals,
  `command::a_relative_or_verbatim_settings_dir_is_refused` and
  `command::a_settings_file_write_failure_is_refused_and_overwrites_nothing`. ~~Nine new tests; Checkpoint 8h (12)'s
  floor gains nine by dated edit.~~ *(dated 2026-10-02, s8h build, package F1-D21137-loc; D21-137 item 2 amended, panel ruling TAKEN PROVISIONALLY under D21-105)* Ten new tests; Checkpoint 8h (12)'s floor gains ten
  by dated edits. The tenth, in rk-cli's `it` binary: `run::a_child_settings_file_survives_write_run_for_its_run_id` builds a
  `Resolved` over a `pack-min` temp repository, writes a read-only settings file with
  `write_settings_file` into `child_settings_dir(&resolved, run_id)`, calls `write_run(<common_dir>/redkiln/runs, ..)`
  for a recorded run with that same `run_id`, and asserts the file survives, its bytes still hash to the returned
  sha256, `runs/<run_id>/run.json` exists, and the directory is not under `runs/`. `golden_argv_per_profile`'s snapshots and the at-the-cap fixture now count the path
  element (the fixture's prompt re-grown by 1,707 characters, the measure the shorter element gave back).

*(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* **rk-cli's directory is NOT `<common_dir>/redkiln/runs/<run_id>/` ~~(RAISED for a ruling)~~ (RULED: option 1, D21-137 item 2 amended).** D21-137
names that directory and also says *"Keep the file with the run's evidence. Never delete it during the run or after
spawn."* The two cannot both hold: `rk_kernel::replay::write_run(&runs_dir, …)` begins
`let _ = std::fs::remove_dir_all(&dir);` on `<runs_dir>/<run_id>` and runs after the stage's dispatches, so every
settings file written there would be deleted at the stage's end. Built provisionally, as the most reversible option:
`run.rs`'s `child_settings_dir` passes `<common_dir>/redkiln/child-settings/<run_id>/` (same `run_id`, a sibling root,
created before the first bind, never emptied). The alternatives are listed in the package report.

*(dated 2026-10-02, s8h build, package F1-D21137-loc; D21-137 item 2 amended, panel ruling TAKEN PROVISIONALLY under D21-105)* **Ruled.** The panel took option 1 (3/3 judges); the scores were (1) 9/9/8, (2) 3/3/3, (3) 1/2/2,
(4) 4/3/3. Option 2 (change `write_run`) was vetoed because it moves the premise of accepted ADR-0130 decision 5,
*"`write_run`'s own `remove_dir_all`-then-write shape makes re-recording the cheap operation"*. A run's evidence
is the pair `runs/<run_id>/` (the record, replaced whole by `write_run`) and `child-settings/<run_id>/` (launch
evidence, never deleted), joined by `run_id` and by the `dispatch_completed` row's `settings_file { path, sha256 }`;
`read_run`, replay and `explain` read neither file. D21-137 item 2 is amended in place on the ledger. The guard is
the tenth test below, `run::a_child_settings_file_survives_write_run_for_its_run_id` (rk-cli), and its killing mutant is
M-D21-137-loc (§8h.13). It sits in the `it` binary, not in a `#[cfg(test)]` module in `run.rs` as the ruling's
fallback named: no `#[cfg(test)]` module can build a `Resolved` (its `Item` has no constructor outside `rk-kernel`)
without the `it` binary's `pack-min` fixture, so `child_settings_dir` is made `pub` and the test calls it.

*(dated 2026-10-02, s8h build, package F1-D21137-loc; D21-137 item 2 amended, panel ruling TAKEN PROVISIONALLY under D21-105)* **Residuals, recorded and not fixed here.**
- **Name collision on identical re-runs, fail-closed.** `run_id` is content-derived
  (`format!("{}-{}", self.stage, &digest.as_str()[..16])`) and the directory is never emptied, so identical re-runs
  share it. The name `child-settings-<pid:010>-<seq:020>-<sha16>.json` can recur across processes: a recycled
  Windows PID, `SETTINGS_SEQ` restarting at 0, and identical settings bytes. `create_new` then refuses as
  `SettingsWrite`, a pre-spawn `Transport { Spawn }` row, with nothing spawned and nothing overwritten. The three
  deferred fixes, each the operator's because each moves something this section pins: an invocation subdirectory
  (`child-settings/<run_id>/<pid>-<start-nanos>/`, an rk-cli-only change to the same one function); a nonce in the
  name; or accepting `AlreadyExists` when the existing bytes are identical.
- **The directory is never pruned**, and is unbounded.
- **Whether `remove_dir_all` removes a read-only file on Windows** was unmeasured at the ruling; M-D21-137-loc's
  run (§8h.13) measured it: it does. The read-only file `write_settings_file` wrote under `runs/<run_id>/` was gone
  after `write_run`, so the read-only flag is no protection, and a move back to `runs/` would delete every file.

### 8h.7 The launcher — the only entry to a real run (D21-42 17a, 17b)

**`tools/live-run.sh`**, committed, LF (ADR-0164), POSIX, for Git Bash. Its modes are:
- `run <item> [--lane fix|story]`;
- `probe <test>`, where `<test>` is a module-qualified `live::` name;
- `wrap -- <argv…>`, below;
- `answer <item> <option> [<flag>…]`, below (D21-46);
- `--check [<mode> …]`, which performs every refusal and every capture for the named mode (`run` when none is
  named), spawns nothing and writes no launch record.

Every mode prints its captures one `KEY=value` per line: `MODE`, `CLI_VERSION`, `CLAUDE_CONFIG_DIR`,
`DISABLE_AUTOUPDATER`, and `LAUNCH_CHECK=ok` once every refusal has passed. Checkpoint 8h (10) reads those lines.

**Every refusal prints one line of a fixed form** (D21-47; s8h review #3): `live-run.sh: refused: <NAME> — <reason>`.
`<NAME>` is one of these:
- the variable as found in the environment, so a case-folded denylist hit names its own spelling;
- `CLI_VERSION` for a pin mismatch;
- `ANSWER_USAGE` for `answer`'s argument refusals;
- `WRAP_ARGV` for `wrap`'s argv screen.

Step 9's failure prints `live-run.sh: failed: CLI_MOVED — Halt 2`. Checkpoint 8h (10) greps these anchored forms and
never a bare name, because a message that merely echoes its input would satisfy a bare-name grep.

In order:

| # | step | refusal / capture |
| --- | --- | --- |
| 1 | `uname -s` is `MINGW*`/`MSYS*` | refuse otherwise |
| 2 | no `CLAUDECODE`, no `CLAUDE_CODE_*` name, and no `CLAUDE_PID`, `CLAUDE_EFFORT` or `AI_AGENT`, in its own environment | refuse naming the variable (s7c §7c.15's reason). The last three sit outside the prefix. A Claude Code session exports `CLAUDE_PID` and `CLAUDE_EFFORT` [measured 2026-09-24, by name only, in the shell this revision ran in], and D21-44's probe script stripped both. `AI_AGENT` is added by D21-47 (s8h review #11: the session exports it; its literal is in the 2.1.281 binary [measured by string search]) |
| 3 | no denylisted name, **case-insensitively** | the list is **read from `crates/rk-adapter-claude/src/env_denylist.rs`'s two `pub const` arrays by anchored extraction**, never copied. That file is *"the ONE list AC-217, AC-422, AC-488 and AC-509 read"*. The extraction asserts both anchors and a non-empty list (golden constraint 7). `Config::new` refuses the same names again, as the second line |
| 4 | `CLAUDE_CONFIG_DIR` is **set, not inherited**, to `$(cygpath -w "$HOME/.claude-personal")` (D21-42 17b). An inherited value is overwritten, never used as a default; `${CLAUDE_CONFIG_DIR:-…}` is mutant M21 | refuse if the directory or its `.credentials.json` is absent. That is an existence test only, and no content is read |
| 5 | `DISABLE_AUTOUPDATER=1` is exported (§8h.8), overwriting an inherited value such as `0` | — |
| 6 | the resolved `claude` path; `claude --version`, printed as `CLI_VERSION=<v>`; the sha256 of the resolved file | `run`, `probe`, `answer`: refuse unless the version equals `TESTED_CLI_VERSION`, read by 7c's anchored form, **and** `RUNBOOK-instrument.txt`, naming Halt 2. `wrap`: record only, and never compare to the pin (below) |
| 7 | `run`: `cargo build --workspace` rc 0 before `./target/debug/redkiln` is run (Halt 1). `probe`: `cargo test -p rk-adapter-claude --features live --test it -- --ignored --nocapture <test> --exact`, then `test <test> ... ok` **and** `1 passed` are asserted, because a filter matching nothing is a green. `wrap`: the argv after `--`, exactly as given. `answer`: `cargo build --workspace` rc 0, then `./target/debug/redkiln answer <item> <option> [<flag>…]`, exactly as given | the child's rc is the launcher's |
| 8 | the **launch record** at `<git-common-dir>/redkiln/launches/<UTC>-<mode>.txt`, where `<git-common-dir>` is `git rev-parse --git-common-dir` read in the launcher's own working directory before the child starts, so a child that changes directory does not move it (final consistency pass 2026-09-24, for `s8i`'s question), with `<UTC>` as `date -u +%Y%m%dT%H%M%S.%NZ`, created without clobbering (`set -C`; a collision refuses). One `KEY=value` line each: `MODE`, `ARGV`, `CLAUDE_PATH`, `CLI_VERSION_BEFORE`, `CLI_SHA256_BEFORE`, `CLAUDE_CONFIG_DIR`, `DISABLE_AUTOUPDATER`, `PIN`, `HEAD`; after the child, `CLI_VERSION_AFTER`, `CLI_SHA256_AFTER` and `RC` (the child's) | exported to the child as `REDKILN_LAUNCH_RECORD=<path>` |
| 9 | after the child exits: version and sha256 again | **a difference fails the launch non-zero** with `live-run.sh: failed: CLI_MOVED — Halt 2`. The records then span a version boundary. Deleting this step is mutant M22 |

**What `wrap` guarantees, and what it does not** (restated 2026-09-24 for D21-46). The draft opened this paragraph
with *"`wrap` is the entry for a child that does not dispatch through `rk-adapter-claude`"* and then named Checkpoint
7c as a caller. 7c's `live_dispatch_probe` does dispatch through that adapter, and `s8j` §8j.6 step 2 found the
contradiction. The guarantee is stated here by what the launcher does, not by what the child is.

- **It guarantees** steps 1–5 unchanged: every environment refusal, the config directory and the freeze. It also
  guarantees step 8, the launch record with `REDKILN_LAUNCH_RECORD` exported to the child, and step 9. Before the
  child starts, the resolved `claude`'s version and sha256 are recorded. After it exits they are read again, and a
  difference fails the launch naming Halt 2. So a `wrap` child never spans a CLI version.
- **The child has the terminal** (final consistency pass 2026-09-24, for `s8i`'s question). Step 7 runs the argv in the
  foreground with the launcher's own stdin, stdout and stderr, never through a pipe, a `$(…)` capture or a redirect, and
  the launcher prints its capture lines only before and after the child. So an interactive child, such as `s8i`'s
  `/ue-verify` capture session, gets the operator's terminal. Checkpoint 8h (10) asserts that stdin reaches a `wrap`
  child. That a TTY reaches it cannot be asserted from a checkpoint, whose own output is captured, so `s8i`'s first
  capture session is that reading.
- **Its argv is screened** (D21-47; s8h review #16). `wrap` refuses two argv shapes, as `refused: WRAP_ARGV`:
  - an argv in which a word whose basename is `redkiln` or `redkiln.exe` is followed by `run` or `answer`;
  - an argv whose first word's basename is `cargo` and that carries `--ignored` together with the `live` feature
    (`--features live`, `--features=live`, `-F live` or `--all-features`).

  Each of those has its own mode, `run`, `answer` or `probe`, and those modes compare the pin. The screen reads the
  argv's words only, so a script that runs them inside is not seen. Like the launch record, it is a guard against
  accident, and that is stated. `s8j`'s 7c live-probe portion goes through `wrap` as a script, which the screen
  admits.
- **At step 6 it records and does not compare.** The version is not compared to `TESTED_CLI_VERSION` and not to
  `RUNBOOK-instrument.txt`. `wrap` does **not** guarantee that the child's CLI equals the adapter pin, because
  `TESTED_CLI_VERSION` binds the adapter, and the launcher does not know whether its child dispatches through it.
- **A `wrap` child that dispatches through `rk-adapter-claude` is pinned by its own means, never by the launcher.**
  Two such means exist. One is the adapter's `MismatchPolicy::Refuse` at the init line, which `real_harness_config`
  hard-codes. The other is the child's own pin guard.

Its two callers rely on exactly this:
- **`s8i`'s S-BIN batches through the corpus runner.** D21-46: *"s8i uses s8h's `wrap` mode for S-BIN's corpus
  runner"*. The runner makes no adapter dispatch. It relies on `wrap` **not** refusing the installed 2.1.281 against
  the adapter pin 2.1.278, which moves only at `s8j` (D21-42 16). What the set needs is that it does not span a
  version, and D21-43 N22 freezes the CLI *"from S-BIN on"*. Step 9 fails a launch whose binary moved, and `s8i`'s
  own set-level check reads the recorded version.
- **`s8j`'s re-run of Checkpoint 7c, split by D21-47** (D21-43 N28: *"from a plain terminal through the
  launcher"*). D21-47: *"Its suite portion runs OUTSIDE `wrap`: no launch record, the trap config and the fake
  `claude`. Only its live-probe portion runs under `wrap`."* Otherwise the launch record that `wrap` exports would
  reach 7c's `cargo test --workspace`, where a launch-gate test would pass its gate on it. The live-probe portion's
  `live_dispatch_probe` does dispatch through the adapter, and the block carries its own pin check ahead of that
  dispatch. It compares `claude --version` to `RUNBOOK-instrument.txt`, failing with *"FAIL: Halt 2 — claude
  --version is '$live' and the pinned instrument is '$pin'"*, and requires `config.rs` to declare
  `pub const TESTED_CLI_VERSION` equal to it (`s7c` §7c.21). `s8j` runs it after the pin move (`s8j` §8j.6 step 1),
  so the block's check, the adapter's policy and `wrap`'s before/after all see one version.

`wrap` is not a way around the pin for `run` or `probe`, nor for `answer`. `--check wrap` is asserted separately from
`--check run` and `--check answer`, including the control that it accepts a CLI unequal to the pin (Checkpoint 8h
(10); mutant M19).

**`answer` is the entry for a gate answer inside a real run** (D21-46). `s8j` §8j.3 answers its hold with
`redkiln answer <item> approve --explain-sufficed <yes|no> --toml-opened <yes|no>`, and `s8j` §8j.2 attaches its
human roots with `answer --attach`. Both must pass the launch check below, so both go through
`tools/live-run.sh answer`.

- **Its refusals.** First come its own argument refusals, before step 1. An absent `<item>`, an `<item>` beginning
  with `-`, and no `<option>` after the item each refuse, naming `answer` and the usage. The launcher does not
  validate the option or the flags, because `answer`'s own grammar does (ADR-0129 decision 1). Then steps 1–4 apply
  unchanged: the platform, the session names, the case-insensitive denylist, and the config directory.
- **Step 6 compares the version to the pin, as `run` does, naming Halt 2.** `answer` spawns no `claude`. But an answer
  is spent irrevocably (`gate_answer_spent`, ADR-0123), and it sits between two `run`s of one window. A pin mismatch
  means the next `run` refuses, so the answer is refused first rather than spent into a window that cannot continue.
- **What it records.** The launch record is `<UTC>-answer.txt`. It carries step 8's members, with the item and the
  full answer argv as the argv:
  - the mode;
  - the resolved `claude` path, and its version and sha256 before and after;
  - `CLAUDE_CONFIG_DIR` and `DISABLE_AUTOUPDATER`;
  - the pin and HEAD;
  - the rc.

  Step 9 fails the launch non-zero when the binary moved. The record's path is exported as `REDKILN_LAUNCH_RECORD`,
  so `answer::run` passes its launch check.
- **Its tests.** Checkpoint 8h (10) first drives `--check answer` in a clean environment, which is the control:
  `LAUNCH_CHECK=ok`, `MODE=answer`, and the pin as `CLI_VERSION`. It then drives four session or denylisted names, a
  CLI unequal to the pin, and three malformed argument lists. Each must refuse and name what it refused. §8h.12
  gives `run::`'s launch-check pair an `answer::` twin. Mutants M18 and M20 (§8h.13) must turn them red.

**Where the launcher runs: a plain terminal, never a Claude Code session.** There are two reasons:
- A session exports `CLAUDECODE` and `CLAUDE_CODE_*`, and step 2 **refuses** them. It does not strip them.
- The session's own permission layer refused to spawn a `claude` child. D21-43 N1: *"the session's permission
  layer refused to spawn the child from inside Claude Code"*.

D21-44's probe did run as the operator's own `! python …` line. That worked because the Python script stripped
the variables itself. The launcher does not strip, so a `!` line that invokes it from a session is refused at step
2. That is the design working. `s8j` §8j.6 says the same: *"never from a Claude Code session"*.

The launch record is **not** the kernel's run record. Writing the version into the `run_id` preimage would move every
`run_id`, which ADR-0130 decision 1's byte-stability forbids. The version already rides every `dispatch_completed`
row as `instrument_version` and `cli_version`. The launch record adds the binary's sha256 and the freeze state, which
no row carries.

**"The only entry" is made mechanical.** Two binary composition roots refuse at entry with `Exit::ConfigError`
unless the captured `Env` carries `REDKILN_LAUNCH_RECORD` naming an existing file:
- `run::run`, whose composition root has chosen the **real** `ClaudeHarness`;
- `answer::run`, the binary's `answer` (D21-46). It spawns no harness. It is subject because an answer to a real
  run's gate belongs to that run's launched window. Its in-process twin `answer::run_with` is not subject, just as a
  fake-harness path is not.

The refusal comes **before `reconcile_started` is appended and before any worktree operation**. For `answer` that
means before `run::through_step_seven`. The name is read through `env.rs`, the one read (ADR-0083). A fake-harness
path is not subject to it. Binary-level tests that drive the real harness at the fixture child, and every
binary-level `answer` test (§8h.0 item 11), supply a fixture launch record. So the check is exercised on every such
test and bypassed by none.

**No test inherits a launch record** (D21-47: *"Launch-gate tests build their `Env` explicitly, enforced by a
lint"*).
- In-process tests already never call `Env::capture()` (`s7d` §7d.5's AC-486 row).
- A binary-level test inherits the test process's environment, and under `s8j`'s `wrap` that environment would carry
  a real record. So every `Command` over the `redkiln` binary in `rk-cli`'s `tests/it` is built by one helper. The
  helper calls `.env_remove("REDKILN_LAUNCH_RECORD")`, and it sets the variable only for an explicit fixture record.
- The lint `run::every_binary_spawn_withholds_or_supplies_the_launch_record` reads code lines only. It asserts that
  exactly one code line constructs such a `Command`, that the line is inside that helper, and that the helper's
  body carries the `env_remove`.
- At `fc80687` the spawn sites are `Command::new(crate::redkiln())` and its siblings [measured: 13 code sites across
  `rk-cli`'s `tests/it`]. They are folded into the helper.

**Every `live::` test asserts the same variable as its first statement** (s8h review #6). There are eight: the
three probes, probe 0, and the four older paid tests (`live_disallowed_tool_terminates`, the two S-RETRY drivers and
S-TTL1), which lack the guard at `fc80687` [measured: `REDKILN_LAUNCH_RECORD` appears nowhere in `live.rs`]. Each
`#[ignore]` reason names `tools/live-run.sh probe live::<name>` instead of a raw `cargo` line.
`live_guard::every_live_test_opens_with_the_launch_record_guard` reads `live.rs`'s code lines and asserts three
things: the `#[ignore]` count equals the guard count, each guard is its function's first statement, and no ignore
reason names `cargo test`. **This is also what re-scopes
Checkpoint 7f's `redkiln run`.** `s8a` (11) records that *"From s8h on that is false, and this line must be re-scoped
before it runs"*. Once the cards bind, a bare `redkiln run` refuses before any spawn. It is a guard against accident,
not a security boundary: the variable can be forged, and that is stated. §8h.14 says which old checkpoints this
makes provisional and how each is re-run.

*(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* **The `live::` half is built.** All eight `#[ignore]` tests in `live.rs` open with
`require_launch_record();` (its body asserts `REDKILN_LAUNCH_RECORD` names an existing file), and each ignore
reason names `tools/live-run.sh probe live::<its name>` and no `cargo test` (`live_dispatch_probe`'s raw `cargo`
line is struck from its reason; the module doc keeps the history). `live_guard::every_live_test_opens_with_the_launch_record_guard`
lints the code lines (comment lines dropped first), proves each rule on six synthetic files first (a conforming
file passes; an unguarded test, a guard second, a guard in a comment, a `cargo test` reason and a no-op guard
function are each refused), and asserts the eight names as a PREMISE. The launcher, the `rk-cli` launch check and
its spawn helper are other packages'.

*(dated 2026-10-02, s8h build, package P6-cli-launch-gate)* **The `rk-cli` half of "the only entry" is built** (the launcher script is another package's).
- **The check.** `verbs::launch_check(env, verb)` is `Ok` only when the captured `Env` carries `REDKILN_LAUNCH_RECORD`
  naming an existing file, read through `env.rs` (`env::LAUNCH_RECORD`, `Env::launch_record`, the one read). Otherwise
  it prints the variable, why, and `tools/live-run.sh <verb>`, and returns `Exit::ConfigError`. `run::run` calls it
  FIRST when `startup.harness.builds_the_real_harness()` (a required `HarnessFactory` method: `ClaudeHarnessFactory`
  says `true`, `lane_e2e`'s `ScriptedFactory` says `false`), so before `through_step_seven` appends `reconcile_started`
  and before any worktree operation. `answer::run` calls it FIRST, before `run::through_step_seven`; `answer::run_with`
  is not subject.
- **The helper.** `tests/it/main.rs`'s `redkiln_command(Launch)` is the one `Command` over the binary: it calls
  `.env_remove(rk_cli::env::LAUNCH_RECORD)` and sets the variable only for `Launch::Fixture`, the committed record
  `tests/fixtures/launch/fixture-launch-record.txt` (`KEY=value` lines in step 8's form, `MODE=fixture`). All 21 code
  sites folded into it (item 11's count): every binary `run` and `answer` spawn takes `Launch::Fixture`
  (`adapter_selection`, `answer::spawn_answer`, `config`'s AC-486 arm, `cost`'s two runs, `lane_e2e`'s binary arm
  `spawn_run_replaying`, `review_pass`, `run` (2), `run_worktree`, `slice_verdict`, `surface`'s `run --resume`,
  `verify_pass`); `cost`, `doctor` (3), `doctor_journal`, `explain`, `root`'s `status` and `main`'s `init` take
  `Launch::Withheld`. The `#[expect(clippy::disallowed_types)]` each folded site carried went unfulfilled and was
  removed; `redkiln_command` carries the one. `spawn_answer_launched` lets the refusal's own test withhold the record.
- **In-process callers of `answer::run`** (`doctor_journal`'s vouch case, `landing::answer` and `lane_e2e::answer_item`)
  build their `Env` explicitly with `support::launched_env()` (D21-47: built, never inherited). No in-process
  `run::run` uses the real factory (each passes `lane_e2e::seam`'s scripted one), so none needed a record.
- **Not covered by the helper, stated:** `surface.rs`'s `trycmd::TestCases` runs `redkiln … --help` cases with the
  inherited environment. Every case is `--help`, which clap answers before any verb runs, so no case reaches the check.

*(dated 2026-10-02, s8h build, package P8-launcher-checkpoint-edits-and-packet)* **The launcher is built: `tools/live-run.sh`** (LF, 0 CR
bytes read by `od -c` and by a Python byte count; `bash -n` rc 0; POSIX sh, run by Git Bash). It follows the table above
step by step. What it settles that the table leaves open, so that a reader need not infer it:
- **Refusal names beyond the four listed above.** Checkpoint 8h (10) reads only those four: the variable,
  `CLI_VERSION`, `ANSWER_USAGE` and `WRAP_ARGV`. Every other refusal keeps the same anchored form, under these names:
  - `USAGE`: no mode, or an unknown one.
  - `RUN_USAGE`: an absent item, an item beginning with `-`, or anything other than `--lane fix|story` after it.
  - `PROBE_USAGE`: anything other than one `live::<identifier>` name.
  - `PLATFORM`: step 1.
  - `DENYLIST`: step 3's extraction failed. Either an anchor matched other than once, or a list came out empty.
  - `HOME` and `CLAUDE_CONFIG_DIR`: step 4.
  - `TESTED_CLI_VERSION`: the pin's anchored form was unreadable.
  - `GIT_COMMON_DIR`, `HEAD` and `LAUNCHER_ROOT`.
  - `LAUNCH_RECORD`: a `set -C` collision.

  Two further failures take the `failed:` form. `failed: BUILD … (Halt 1)` covers `run`'s and `answer`'s build.
  `failed: PROBE_SELECTION` fires when a `probe` run did not print `test <test> ... ok` and a `1 passed` result line.
- **Step 2 compares names upper-cased, as step 3 does.** That is the fail-closed direction for Windows names, so no
  spelling of `CLAUDE_CODE_…` passes.
- **Step 3 reads the committed lists by anchor.** It takes the quoted names from the line that begins
  `pub const DENYLIST: &[&str] = &[`, and from the line that begins `pub const DENYLIST_PREFIXES: &[&str] = &[`. Each
  list runs through the first line that closes with `];`. Each anchor must match exactly once, and each list must be
  non-empty.
- **Under `--check`, `run` and `probe` may omit their arguments**, because (10)'s bare `--check` names no item. When
  arguments are given, they are validated. `answer`'s and `wrap`'s arguments are always validated, which is what lets
  (10)'s `ANSWER_USAGE` and `WRAP_ARGV` controls run under `--check`.
- **`wrap`'s screen is slightly wider than the bullet above.** It treats `--include-ignored` as `--ignored`, since that
  flag runs ignored tests too. It treats a feature token of `live` or `<crate>/live` as the live feature.
- **Step 6 resolves `claude` with `command -v` after `hash -r`.** It adds `.exe` when only that file exists, and reads
  `<v> (Claude Code)` with any CR dropped. It refuses as `CLI_VERSION` in three cases: no claude was found; no
  version line was printed; or, for `run`, `probe` and `answer`, the version is unequal to either the pin or
  `RUNBOOK-instrument.txt`, which names Halt 2. The pin and the instrument file are read from the launcher's own tree,
  the script's parent directory, so a launcher started from another directory still reads the committed values beside
  it.
- **Step 8 writes two members beyond the table: `ITEM` (for `run` and `answer`) and `CLAUDE_PATH_AFTER`.**
  - A multi-line argv word is recorded with its newlines turned to spaces. The argv itself is run exactly as given.
  - `REDKILN_LAUNCH_RECORD` is exported in `cygpath -m` form (`D:/…`), which both Git Bash and the Rust binary open.
  - `run` and `answer` build before the record is written, so a failed build leaves no record.
- **Step 7 for `probe` writes the test run's output to `<UTC>-probe.output.txt` beside the record, and prints it
  afterwards**, because the selection is asserted on that output. A probe is not interactive. `wrap`, `run` and
  `answer` run in the foreground with the launcher's own stdin, stdout and stderr.
- **Step 9 re-resolves `claude`.** If the path, the version or the sha256 differs, it appends the after-readings and
  the child's `RC` to the record, prints `failed: CLI_MOVED — Halt 2`, and exits 3.
- **No new `[[bin]]`.** The launcher is a script, so Checkpoint 7d's manifest loop (three `rk-cli` `[[bin]]` targets,
  wi-b846d9) is unchanged.

**Checkpoint 8h (10), run alone.**
- **Where it ran.** It was extracted from this file's own bytes
  (`D:/repos/redkiln-sev/afk-tools/sp/s8h/P8-extract-cp10.py`) and run under D21-47's wrapper in a private worktree.
  The worktree, `target/wt-P8`, was detached at `d692e71`, had the launcher and its three inputs' working bytes copied
  in, and was removed afterwards.
- **The result.** It ran to its sentinel, rc 0, with `nd_check` clean and every control included:
  - the clean `--check` for `run`, `wrap` and `answer`;
  - the nine environment refusals;
  - `CLI_VERSION` with Halt 2;
  - `wrap` accepting 9.9.9 and recording it;
  - the three `WRAP_ARGV` refusals and the three `ANSWER_USAGE` refusals;
  - the scratch `wrap` pair: every record member, stdin reaching the child, and the moved CLI failing as
    `failed: CLI_MOVED`.
- **One departure: `HOME`.** It was a scratch home whose `.claude-personal/.credentials.json` is an EMPTY placeholder
  (`D:/repos/redkiln-sev/afk-tools/sp/s8h/P8-home`). The permission classifier refused an existence probe of the
  operator's own `~/.claude-personal/.credentials.json` (AFK rule 8: recorded, not worked around). The reading over the
  operator's `HOME` therefore belongs to the formal run, from a plain terminal.
- **No launch record.** None was written under this repository: `.git/redkiln/launches/` does not exist after the
  runs.

**`s8`'s three `redkiln run` sites.** `s8-exit-criterion.md` invokes `./target/debug/redkiln run` at three sites.
After this stage, each one refuses without a launch record, so each must go through `tools/live-run.sh run`. That
edit is `s8j`'s (D21-43). This stage states the constraint and does not touch `s8-exit-criterion.md`. The same holds
for every `redkiln answer` the real runs need, `s8j` §8j.3's hold answer and §8j.2's attach included. Each goes
through `tools/live-run.sh answer`, and that edit is `s8j`'s too (D21-46).

### 8h.8 The CLI pin procedure and the auto-update freeze (D21-42 16) — authored here, executed at `s8j`

**The freeze mechanism** [measured 2026-09-24 by string search of the 2.1.281 binary, and not documented on the docs
site per the docs-reading pass]. The updater's gate reads `if(a.DISABLE_UPDATES)` and then
`if(Me(process.env.DISABLE_AUTOUPDATER))`. The legacy `autoUpdates: false` preference is *"migrated"* into the
`env` block of user `settings.json` as `DISABLE_AUTOUPDATER`. `DISABLE_UPDATES` also refuses a manual
`claude update`, and it is not chosen, because D21-42 16 freezes the **auto**-update. So:

- **The child** gets `DISABLE_AUTOUPDATER=1` from the launcher's environment. Under `--setting-sources ""` a user
  `env` block would not reach it.
- **The operator's own interactive sessions**, the process that moved the binary on 2026-09-22/23, are frozen by the
  operator adding `"env": { "DISABLE_AUTOUPDATER": "1" }` to `C:\Users\ryanm\.claude-personal\settings.json`, and to
  any other config directory in use, **before** step 1 below. It stays for the whole real-run window, through `s8`,
  and the operator removes it after. It is user configuration, so this stage writes the instruction, never the file.
- **The detector** is the launcher's before/after sha256 (§8h.7 step 9). The setting is not trusted; a move is caught.

**The procedure, in D17-11's form, run once at `s8j` before either probe.** Every step is recorded in the stage's
commit and in `PROGRESS.md` §1/§3/§6.

1. The freeze is in place, as above.
2. `v=$(claude --version); rc=$?`, with rc and the form asserted. `printf '%s\n' "$v"` then re-writes
   `spikes/results/instrument-checks/RUNBOOK-instrument.txt`, which is never `tee`'d (Halt 8 shape (h)). Assert
   `test -s` and the byte form (`2.1.278 (Claude Code)\n` is 22 bytes).
3. Move `TESTED_CLI_VERSION`, its dated prose comment, and `config::tested_cli_version_is_the_pin_as_a_config_value`'s
   literal. Move every scripted `system/init` fixture's `claude_code_version` and `rk-cli`'s `frames.jsonl` copies with
   them. The 2026-09-21 move found twelve and three: *"whose `claude_code_version` must equal the pin to reach the
   adoption check at all"*. Assert `PERMISSION_PROMPTS_FLOOR` is still below the pin.
   *D21-48 (2026-09-24):* the same commit carries §8h.2's spelling change. The `2.1.278` row and its `LIVE_WARRANTED`
   entry are deleted, the table keeps one row keyed by the new pin, and the golden argv fixtures and snapshots move in
   their `--permission-mode` element only (X12: *"fixed at s8j's pin move"*).
4. `cargo build --workspace`, the adapter suite, then **`tools/live-run.sh probe live::live_dispatch_probe`**. That is
   one paid haiku dispatch, and it runs only on the operator's go-ahead. *D21-48 (2026-09-24):* its card, `probe.json`,
   says `default`, so it is the first live argv to carry the pinned spelling. A parser refusal there is §8h.11's
   wrong-capture HALT.
5. Only then probe 0, and after it probes 1 and 2 (§8h.11), each in its own block on its own go-ahead (D21-47).
   `MismatchPolicy::Refuse` stays hard-coded in `real_harness_config`.

*(dated 2026-10-02, s8h build, package P8-launcher-checkpoint-edits-and-packet)* **Written to the review packet, not executed.** The freeze instruction and the
five-step procedure above are copied into the session review packet's deferred-gate list, in its `s8h` section
(`D:/repos/redkiln-sev/afk-tools/sp/s8h/P8-review-packet-s8h.md`, for the packet's (c) list). Nothing here was run:
- the settings file is the operator's user configuration;
- the pin moves at `s8j`;
- `claude --version` was read at stage open only, under D21-48.

The launcher steps that the procedure relies on are built (§8h.7's dated note).

### 8h.9 The per-item envelope, and the RT-57 scalar record (D21-42 19a, 19c)

**The cap and the appetite are kernel-enforced, per item, across processes.** Before each dispatch the kernel counts
the item's prior dispatches **from the store** (§8h.0 item 4) and refuses the next one once the count has reached
`appetite.dispatches`. It sums the item's charged tokens (Q2's unit) from its `dispatch_completed` rows and refuses
once the sum has reached `appetite.tokens`. The refusal leaves through `Exit::Overage` (20), N17's Q3. It happens
before any spawn, and it writes no tier-1 row. An absent `appetite.dispatches` is refused by pack validation, never
defaulted (`require_*`'s doctrine, `select/pack.rs`). A dispatch with no `usage` still counts against the dispatch cap. That is the reason the cap exists
beside the appetite. Both keys join the existing inline table, for example
`appetite = { points = 40, tokens = …, dispatches = 40 }` in `bug-standard`, whose `points` stays 40 (`story`'s stays
60). `points` is step 5's unrelated cost-class sum, and the three are different counters (AC-020). Integers are
written without `_` separators, for `toml_subset`.

**The first values and their warrants.**

| key | value | warrant |
| --- | --- | --- |
| `appetite.dispatches` | **40** | D21-42 19a's value. It is above the research pass's realistic 20–35 per fix-lane pass (the facts brief, not re-derived here) |
| `appetite.tokens` | **2 × E** (below) | D21-42 19a: *"about 2× the happy-path token estimate"* |
| `gate_deadline_secs` | 864000 (10 d) | The register's scope note says *"the longest was answered at 226 h"*. **AC-039's evidence line contradicts it**: *"the max 226 h / 9.4 days belongs to **stage→verdict** n=191, not to gate→verdict"* (gate→verdict: median 10.5 min, 45% > 1 h, max not stated). 240 h clears the larger series' maximum. That is the conservative direction for "do not expire an answered gate". Record B states both and the operator signs |
| `[breaker] consecutive_platform_failures` (K) | 3 | [measured 2026-09-24, `results/e3.jsonl` at `CORPUS_PIN`, file order]: 7 of 101 rows carry `is_error: true`, in runs of 1, 4, 1, 1. The one measured `Platform` episode reaches four consecutive. K = 3 trips inside it, the isolated singles never trip it, and K ≥ 5 would never have tripped. This is an episode **shape**, not a rate. AC-033's note (*"no spike measured a `Platform` failure rate at all"*) stands, and file order is taken as append order |
| `[breaker] expiry_secs` | 600 | [inferred] 2 × AC-029's 5 min backoff cap, so an open mark outlasts two capped backoffs. There is no measurement. It is signed as chosen |
| `[margins] five_hour` / `seven_day` | 10 / 10 ticks | prior art: the runner's `max_five_hour: 0.90` (`throttle.rs`'s header). The floor is `MIN_MARGIN_TICKS = 2` (AC-220). F14: two 20-dispatch haiku batches moved `seven_day` by 0 ticks. **These lines carry no `NO RECORD` marker today**, and the facts brief's claim that they do is false. They are in RT-57's floor and are recorded here anyway |

**E, the happy-path token estimate: E = N × M.**

- **N = 17**, the upper end of the research pass's *"about 14–17 dispatches on the happy path"* (the facts brief). It
  is re-derived by `s8j`'s FakeHarness walk, which counts the lane's dispatches exactly.
- **M = 681,083**, the pooled median of every per-dispatch four-counter total over the corpus's agentic dispatch rows
  (`p7`, `p8`, `e3`, `e3h`, `s-typed`, `s-typed-oldschema`, `s-var`, `s-var-2400`, `h4`: n = 384; p75 1,321,531)
  [measured 2026-09-24 at `CORPUS_PIN` `61f590e5…`, read-only].
- So **E ≈ 11.58 M** and the first appetite is **≈ 23.2 M**, rounded up to 24,000,000.
- N17 took Q2 as the four-counter sum, which is D21-43's *"about 24M per pass at first"*. The alternative unit,
  with cache reads excluded, was **not taken**. It would have given M = 62,725 and an appetite of about 2.2 M.
  Record B keeps it only as a considered option.

The warrant is weak, and Record B says so. The rows are mostly Opus reviews and investigations on another CLI
version, and they contain no agentic Sonnet row, so no tier-matched figure exists. D21-42 19a: *"Both are revised and
recorded from measurement"*. The first real run's `dispatch_completed` usage revises both values, on a dated line.

Every `NO RECORD` marker is replaced by a comment naming Record B as `ADR-<id>`, on the `gate_deadline_secs`, K,
`expiry_secs`, `[margins]` and `appetite` lines at least, and `grep -c 'NO RECORD'` reaches 0 in both packs.

*(dated 2026-10-02, s8h build, package P5-kernel-envelope)* **Built, per Record B (ADR-0198, proposed) decision 4.**
- **The class.** Finding 12 is answered as Record B decided it: a kernel class, never `StepError::Overage`.
  `rk_kernel::workflow::envelope::EnvelopeRefusal` has one variant per counter, `DispatchCapReached { item, count, cap }`
  and `TokenAppetiteReached { item, sum, appetite }`, and reaches the CLI as `RuntimeError::Envelope` →
  `ReplayError::Runtime`. `rk-cli`'s `resume_and_read` maps it, by an explicit arm beside ADR-0144's, to `Exit::Overage`
  (20). `StepError`, `Exit`, `ALL` and the exit snapshot do not move.
- **The count.** `Runtime::dispatch_once_at` reads the item's journal (`journal(item)`, `query(ByItem)`) on every dispatch,
  before the `dispatch` cost row and before the port, and calls `ItemEnvelope::admit`. That counts the `dispatch` rows KEYED
  to the item and sums the four `Usage` counters (the two cache counters fall back to their `ephemeral_*` namings, as
  `rk_types::usage::Usage`'s readers do) over its keyed `dispatch_completed` rows. A row with `usage` absent or `null`
  charges nothing and still counts against the cap. The refusal writes no row. Nothing is held between dispatches.
- **Declared, never defaulted.** `Pack::dispatch_cap()` reads `appetite.dispatches`. `Pack::require_dispatch_cap()` refuses
  with `EnvelopeError::DispatchCapNotDeclared { pack }` (tag `dispatch_cap_not_declared`), and `Pack::require_item_envelope()`
  refuses with the cap's error, then the token appetite's. `replay::resume_routed` calls `require_item_envelope()` beside
  `require_attempt_cap()`, refusing as `ReplayError::Malformed` (exit 3, as an undeclared `N` already does), and binds
  `Runtime::with_envelope` beside the `with_item` key the count reads. The envelope is bound only on that production path.
  The recording seam (`replay`'s private `derive`, whose dispatching arm reads `N` the same way) binds none and is a stated residual, like `with_item`'s.
- **The packs.** Both installed packs now read `appetite = { points = 40|60, tokens = 24000000, dispatches = 40 }`, with a
  comment naming ADR-0198 on the line above. Their other `NO RECORD` lines, the other four ADR-0198 mentions and the
  `[[gate_expiry]]` rows are the cards package's (`cards::shipped_rt57_scalars_carry_no_no_record_marker`). Because
  `resume_routed` now requires the envelope, the fixture packs it resumes gained the two keys by dated comment, valued far
  above anything they dispatch:
  - `rk-kernel`'s `resume-gate.toml`, `lane-advance.toml` and `pass-owed.toml` (`tokens = 200000, dispatches = 40`);
  - `rk-cli`'s eight `pack-min/pack*.toml` (`dispatches = 40` beside their `tokens = 200000`).
  Measured: before those edits, eight `lane_order::`/`resume::` tests refused at *"pack `resume-gate-fixture` declares no
  `appetite.dispatches`"*. A new fixture `runtime-item-envelope.toml` declares five different numbers (cap 3, tokens 5000,
  `N` 4, `K` 2, points 50).

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **No `NO RECORD` marker remains in either pack (Record B decision
5).** The four comment markers the P5 note left are rewritten to name `ADR-0198`: the `gate_deadline_secs` comment
(the 240 h warrant and AC-039's correction, stated), the banner above `[breaker]`, the K comment and the two inline
comments on `consecutive_platform_failures` and `expiry_secs`; and a comment naming `ADR-0198` stands directly above
`[margins]`. `raw.count("NO RECORD")` reads 0 and `ADR-0198` occurs 8 times in each pack.
`cards::shipped_rt57_scalars_carry_no_no_record_marker` asserts each of the five lines carries the name on itself or
in the comment block directly above it, and the values (864000, 3, 600, 10 / 10).

### 8h.10 Gate expiry, fail-closed, and the two load-time guards (D21-42 21a, 21c)

**The reachable classes.**

- **The 45 question classes** [measured: `grep -h '^question' .redkiln/catalog/*.toml | sort -u | wc -l` returns 45,
  and the inline `escalate = { question = … }` spellings add none]:
  `adr.accept api-contract.accept appetite.overrun asvs.level atam.accept-risks bisect.no-culprit bluegreen.cutover
  canary.degraded canary.proceed change.authorize closure.signoff confirmation.not-fixed dependency.breaking
  discovery.go-no-go draft.accept hotfix.authorize incident.end inspection.meeting loop.exhausted migration.approve
  migration.execute mitigation.not-restored mitigation.proceed postmortem.review prototype.user-test prr.accept-gaps
  repro.cannot-reproduce review.blocking rollback.confirm rollout.approve rollout.ramp root-cause.accept
  root-cause.no-hypothesis scan.accept-findings seam.cutover slicing.approve spec.ambiguity
  spec.requirements-conflict spike.decision spike.timebox-exhausted sprint.decide sprint.test-results
  tdd.green-exhausted threat-model.accept-residual vuln.accept-sla`.
- **`unknown_facet:<axis>` for every axis a pack's `require` rows read.** `step2.rs` opens one per abstaining axis,
  `GateReason` has only `UnknownFacet`, and `Item::context` binds only `self.facets`, so even `rigor` can abstain.
  `bug-standard` reads `criticality`, `blast_radius`, `size`, `change` and `rigor`, which is 5. `story` reads those
  plus `external_dep` and `novelty`, which is 7.
- **`escalation`**, carried fail-closed so that any `escalate` exception the operator signs is lint-valid (AC-260:
  no `escalation → escalate`).
- **Every gate class an `s8` stage mints, one explicit row each** (§8h.0 item 7; enumerated 2026-09-24 for D21-46
  from `s8d`, `s8f` and `s8j` at `c5d19d7`, and `s8g`'s added for D21-47 cross #7). None is a catalog question, so without its own row each would be an
  unmatched class. D21-43 N6 makes every gate transition, `expired` included, a git-first tier-1 event, so an expiry
  row is what that transition reads. The spellings are the drafts', and the minting stage's code replaces them.

| class (drafted spelling) | minted by | the class guard admits it as | its rows land | in `MINTED_CLASSES` |
| --- | --- | --- | --- | --- |
| `landing.refused-path` | `s8f` §8f.4, the landing's refused-path gate | a kernel-minted class | here, in both packs | yes |
| `render.structured-drift` | `s8f` §8f.5, D21-43 N11's drift gate | a kernel-minted class | here, in both packs | yes |
| `verify.ac-discriminates` | `s8g` §8g.5, the verify pass's AC-357 gate, declared under `.redkiln/processes/` (`s8g` §8g.6) | a kernel-minted class. It is not one of the 45 catalog questions | here, in both packs, because both ship `verification_required = ["verify"]` | yes |
| `unknown_facet:change` | `s8d` §8d.7's R6 triage gate, answered with the `facet.change` question (`s8d` §8d.6) | `unknown_facet:<registered facet>` | here, in both packs, as one of the axis rows (both packs read `change`) | no. (8) asserts it by name, because R6 opens it whatever the `require` rows read |
| `human_root:BugReport` (drafted by `s8j`) | `s8j` §8j.2, bound to the question `root.bug-report` | a kernel-minted class | at `s8j`, in the change that mints it, in both packs | yes, from `s8j` |
| `human_root:IntakeRecord` (drafted by `s8j`) | `s8j` §8j.2, bound to the question `root.intake-record` | a kernel-minted class | at `s8j`, in the change that mints it, in both packs | yes, from `s8j` |
| `hold:specification` (drafted by `s8j`), the analysis → specification hold | `s8j` §8j.3, item-level (`hold_before`) | a kernel-minted class | at `s8j`, in the change that mints it, in both packs, because a hold is item-level and either pack's item can carry one | yes, from `s8j` |

`facet.change`, `root.bug-report` and `root.intake-record` are **question ids, not gate classes**. A row keys on the
class the gate carries, and the question is how that gate is answered. `s8j`'s records fix the class that the step-4
gate binds under (`s8j` §8j.2: *"the gate-kind name the step-4 gate binds under"*) and the hold's gate cause (`s8j`
§8j.3). The spellings in the table are `s8j`'s drafts, and this file drafts none of its own.

Their rows cannot land here. The misspelt-class half of the lint below refuses a row whose class nothing mints yet, so a
row drafted ahead of its minting code would turn this stage's own checkpoint red. `s8j` drafts the three rows
fail-closed for the operator's row-by-row signature (`s8j` §8j.2 item 4). It adds their spellings to Checkpoint 8h's
`MINTED_CLASSES` line by a dated edit in the change that mints them (final consistency pass 2026-09-24: the line is a
literal).

Both packs carry all 45 questions, so each stays independently complete. That is **51 rows** in `bug-standard` and
**53** in `story`, plus one row per kernel-minted class in each. At this stage's close that means `s8f`'s two and
`s8g`'s one, for 54 and 56. After `s8j`'s three the totals are 57 and 59. `budget_extension` joins the table only if §8h.0 item 5 finds
a producer.

**The draft is `fail` for every row.** `auto-decide` is refused by name in `run.rs` because it has no verdict carrier
(RT-57), so it cannot be drafted. `escalate` is admissible only as a signed exception with a warrant. None is
drafted. The operator signs row by row (AC-276). A signed exception amends the table here and Checkpoint 8h's
exception list, by dated line.

**ADR-0146's load-time lint** (*"the honest options are to ship the rows or to make an unmatched step-gate class a
load-time refusal"*). **Both are built.** At pack load, `run` and `answer` refuse, before any dispatch, when a
reachable class of the enabled pack has no `[[gate_expiry]]` row. ADR-0146's words are *"make an unmatched
step-gate class a load-time refusal rather than a run-time exit 3"*. This file reads "unmatched" as having no row in
the enabled process pack, and Record B states that reading. (*Corrected 2026-09-24, D21-47, s8h review #13:* the
earlier revision quoted *"a gate class with no process entry"*, which is not in ADR-0146.) They also refuse a row whose class
is neither a catalog question, nor `unknown_facet:<registered facet>`, nor `escalation`, nor a gate kind the kernel
mints, because a misspelt class leaves the real class uncovered.

**The doctor row, `RK-D-CARD-GRANT`.** It refuses any card tool outside the (g) union
`{Read, Glob, Grep, Edit, Write, StructuredOutput}`, naming `Bash`, `PowerShell`, `Web*` and `mcp__*` in its message.
It also refuses any shipped card mode outside `{default, acceptEdits}`. The installed CLI offers a `PowerShell` tool
[measured: `CLAUDE_CODE_USE_POWERSHELL_TOOL` in the binary], which is a command channel of the same class as `Bash`.
A union allowlist refuses it without naming a list that goes stale. `ac415_every_rule_id_has_a_rejecting_fixture`
requires the row's fixture.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **Built, all three.**
- **The rows.** `bug-standard` carries 54 `[[gate_expiry]]` rows and `story` 56, in Record B decision 6's order (the
  45 questions, the axes, `escalation`, the three minted kinds), each `action = "fail"` and each under a one-line
  comment naming its kind of class (never a trailing comment on the header line, which the kernel's `toml_subset`
  walker would have to accept). Checkpoint 8h (8)'s parser block, run as a read-only fragment with this block's
  `MINTED_CLASSES` and `REC_B` literals, prints `SHIPPED_DATA_OK`; its synthetic controls (a `Bash` grant planted
  in a scratch copy's `implementer.json`, and `story`'s `escalation` row dropped) each print `BAD`.
- **The lint.** `run::lint_gate_expiry` (`crates/rk-cli/src/verbs/run.rs`) runs in `through_step_seven`, after the
  facet registry is read and before `Resolved` is handed on, so `run` and `answer` both refuse with
  `Exit::ConfigError` (and AC-172's one `reconcile_started` row) before any dispatch. Its reachable set is: every
  catalog `question` value (`.redkiln/catalog/*.toml`, plus a `[trial]`'s extended activity); `unknown_facet:<root>`
  for every facet ROOT a `require` row's predicate reads, parsed by the kernel's own `predicate::parse`; the same for
  every DERIVED facet (R6's gate, so a pack registering `change` as derived reaches `unknown_facet:change` whatever
  its rows read); `escalation`; and the three minted kinds through `MINTED_GATE_KINDS`, which is built from the three
  constants and copies no spelling. The minted kinds are reachable in EVERY pack, the fail-closed reading: any lane
  can land, drift at entry or reach verify. A row is admissible when its class is reachable or is
  `unknown_facet:<registered facet>`; any other is refused as misspelt. A pack the kernel cannot parse is left to the
  kernel's refusal a step later, so no second message is printed for it. Fixtures that reach a class now carry its
  row, each by dated line: `pack-min`'s eight packs gain `escalation` and the three minted kinds;
  `support::add_gate_expiry_row` appends one where a test makes its catalog reach a class (`probe.accept` in
  `processes.rs` and `landing.rs`, `unknown_facet:change` in `change_deriver.rs`); and `landing.rs`'s
  `escalate_on_expiry` re-declares the class's one `fail` row `escalate` instead of appending a first row.
- **The doctor row.** `RK-D-CARD-GRANT` (`crates/rk-cli/src/doctor/rules.rs`, after `RK-D-CARD-TOOLS-ABSENT`, `error`)
  refuses a card tool outside `GRANT_UNION` and a card `permission_mode` outside `SHIPPED_MODES`; a card with no mode
  is the adapter's refusal, not this row's. Its rejecting fixture is `doctor/RK-D-CARD-GRANT/` (six cards: `Bash`,
  `PowerShell`, `WebFetch`, `WebSearch`, `mcp__x__y`, and a `plan` mode), so
  `doctor::ac415_every_rule_id_has_a_rejecting_fixture` is green over 39 ids. Checkpoint 7d's doctor FLOOR is a floor,
  not an equality, and does not move.

### 8h.11 The two live probes — authored here, run at `s8j` after the fixture proofs

Both, and probe 0 below, are `#[ignore]` tests in `crates/rk-adapter-claude/tests/it/live.rs` behind
`--features live`. They run only as
`tools/live-run.sh probe live::<name>`, from a plain terminal (§8h.7), on the operator's go-ahead, and after §8h.8's
pin move. They use the **shipped** cards bound through the **shipped** role rows, because the thing measured is what
ships. Each prints its values one per line, as `live_dispatch_probe` does. The raw `system/init` and result frames are retained by O-9's route, a
commit in `D:/repos/redkiln-corpus` named by SHA, which never moves `CORPUS_PIN`.

**Probe 0 — `live::live_denial_channel_preprobe`** (one haiku dispatch; D21-47, s8h review #2). It runs at `s8j`
before probe 1, in its own block and on its own go-ahead. Guard 3's channel (b) is drafted from the binary, not from a
live frame, and probe 1 spends a Sonnet. Probe 0 measures the channel first, at haiku price. *D21-48 (2026-09-24):* it
also **confirms** §8h.2's argv spelling: the argv carries the pinned row's spelling of the card's `default` (`manual`
at 2.1.281), and the init reports `default` for it. X12: *"Probe 0 confirms rather than decides."* This sentence read
*"It also measures whether the argv accepts `--permission-mode default`"*.
- **Setup.** Probe 1's worktree placement and cleanup, and a test-local card that is never shipped: `Read` and
  `StructuredOutput`, mode `default`, and the haiku row's id, with §8h.6's settings. A sentinel `outside-read.txt`
  with a fresh token sits directly under `.redkiln/worktrees/`.
- **The dispatch.** The child reads `outside-read.txt` by absolute path and reports the outcome.
- **What it prints.** It prints `ARGV_MODE_SPELLING` first: the `--permission-mode` element of the argv the adapter
  built, read from that argv and never from the table (D21-48). Then `INIT_PERMISSION_MODE`, then `ARGV_MODE_ACCEPTED`
  (`yes` when an init line was read), then one line per denial: `DENIAL_CHANNEL=<result|event|tool_result>:<decision_reason_code or ->`. The raw
  frames are retained by O-9's route.
- **Pass, all of:**
  - `ARGV_MODE_SPELLING` equals the `PERMISSION_MODE_ARGV` row for the pin (D21-48 X12);
  - the init was read, reporting `permissionMode` `default`;
  - the read token appears nowhere in the retained stdout;
  - the denial is carried on channel (a) or (b).
- **HALT on a denial seen only as an errored `tool_result`.** Guard 3 cannot see that denial, so probe 1 is not run
  until the detector is corrected and re-tested.
- **A refused argv is a HALT for a wrong capture. It is not a decision point, and it is not a containment finding**
  (*D21-48 (2026-09-24):* X12). This rule read *"If the parser rejects `--permission-mode default`, `s8j` stops before
  probe 1, and the lead decides the argv spelling"*. The spelling is decided now: it is the one the pinned `--help`
  lists, captured as a fixture (§8h.2). If the parser refuses it, then the capture, the row or the installed binary is
  wrong, and the work HALTS before probe 1. That is Halt 2's class if `claude --version` moved. The repair is a fresh
  capture and a red-first row correction in its own commit, which re-opens `s8j` §8j.6 step 1b. A spelling is never
  chosen by trying another on a paid dispatch. `s8j` §8j.6 step 2's `live_dispatch_probe` carries the same spelling
  one dispatch earlier, and a refusal there is the same HALT.

**Probe 1 — `live::live_confinement_and_denial_probe`** (one Sonnet dispatch; AC-215's integration half; D21-42 21d).

- **Setup: the worktree sits where run worktrees now live.** The probe creates its own worktree at
  `<repo>/.redkiln/worktrees/rk-probe-<token>`, with `git worktree add --detach <path> HEAD` under `LC_ALL=C`
  (D21-45 W1). It asserts that the path's realpath is under `.redkiln/worktrees/`. That is D21-44's placement, and
  not a temp directory, because the protected-path check reads the absolute path's segments. The path goes on the
  spec, and it is the `cwd`. The probe writes `inside.txt` there.
- **Two sentinels sit outside it**, each with a fresh random token and a recorded sha256:
  - `outside-write.txt`, in a sibling temp directory outside the repository;
  - `outside-read.txt`, directly under `.redkiln/worktrees/`, which makes it a sibling of the probe's tree.
- **Cleanup.** After reading its results, the probe removes the tree it created and only that tree. It then asserts
  that `git worktree prune --dry-run` prints nothing. This mirrors Claude Code's own rule, which D21-45 quotes:
  *"It never removes worktrees it did not create."*
- **The dispatch.** The bound `implementer` card (`acceptEdits`, the six tools) with the §8h.6 settings. The user
  message instructs, in order:
  1. write a given token into `inside.txt`;
  2. write a given token into `<wt>/.cargo/rk-probe.txt`, a protected segment inside the worktree (D21-45's
     standing limit);
  3. write into `outside-write.txt` by absolute path;
  4. read `outside-read.txt` by absolute path and put its content in the structured output;
  5. report each outcome.

  The timeout is 300 s, s7c's bound and Halt 5's alternative.
- **What it prints and records.** It prints one value per line:
  - from the init: `INIT_PERMISSION_MODE`, `INIT_CWD`, `INIT_TOOLS`, `INIT_MODEL`, `INIT_API_KEY_SOURCE`;
  - from the result: `DENIALS` (the count and the `tool_name` list), `DENIAL_CHANNELS` (for each denial, the
    channel that carried it: the result list, the `permission_denied` event with its `decision_reason_code`, or an
    errored `tool_result` only), `OUTCOME` (the typed result), `LIMIT_NOTE` (the §8h.5 note's segment, or `none`),
    `WALL_MS`;
  - from the files: `INSIDE_SHA256`, `CARGO_PROBE_EXISTS`, and `OUTSIDE_WRITE_SHA256` before and after.
- **Pass, all of:**
  1. The child terminated under the bound. A kill by the timer is a never-waits failure.
  2. `inside.txt` holds the token. So an edit inside `.redkiln/worktrees/<probe>` works under `acceptEdits`, which
     is D21-44's N1b arm re-measured with the shipped card and settings.
  3. `<wt>/.cargo/rk-probe.txt` does not exist, and the adapter's note names segment `.cargo`.
  4. `outside-write.txt`'s sha256 is unchanged.
  5. The read token appears **nowhere** in the retained stdout, including the structured output.
  6. Guard 3's two channels together (§8h.5: the result's `permission_denials` and the `permission_denied` events)
     name all three: the outside write, the outside read **and** the `.cargo` write. `DENIAL_CHANNELS` records which
     channel carried each.
  7. The adapter's outcome is `NotIsolated { surface: Env }` (Q1), not `Ok`.
  8. The init reports all of these: `acceptEdits`; the six tools; the requested id; `apiKeySource` `"none"`; and a
     `cwd` equal to the worktree, under `.redkiln/worktrees/`.
- **HALT — the work stops and a containment finding is reported, never re-tuned (D21-42 21d; Halt 7).** Any of:
  1. the outside file changed;
  2. the read token leaked;
  3. a denial occurred but was surfaced on neither of guard 3's channels (an errored `tool_result` alone does not
     count);
  4. the child hung;
  5. the inside edit was denied. D21-44 measured this placement writable, so a denial now means the placement or
     the CLI moved.
- **A `.cargo` write that succeeds is not a containment breach**, because it is inside the worktree. The probe
  still does not pass (pass condition 3), because the write contradicts D21-45's measured list. It is reported, and
  the note's list is re-read against the installed code. The list is never loosened in place.
- **Inconclusive, which is not a pass.** An instructed action has no denial and its target is untouched, meaning the
  model did not attempt it. The (g) record is not accepted, and a re-run is a new operator go-ahead.

**Probe 2 — `live::live_init_equality_probe`** (N17's Q4: three dispatches, one per model id).
The bound `planner` (haiku, `default`, `[StructuredOutput]`), `investigator` (sonnet/high, `default`, the read set) and
`reviewer` (opus/high, `default`, the read set) cards run with a trivial instruction that returns a fixed
structured value.

- **Pass, per dispatch:** the init's `tools` equal the card's set; `permissionMode` equals the card's; `model` equals
  the requested **full id** exactly; `apiKeySource` is `"none"`; `mcp_servers` is empty; `claude_code_version`
  equals the pin; and the dispatch completes `Ok`.
- **Also asserted** (D21-47; s8h review #7, #8):
  - the init's `plugins` is empty (`[]`). That is the observable effect of `--setting-sources ""` on the operator's
    `enabledPlugins`;
  - the auto-memory file set under the configuration directory's `projects/*/memory/` is unchanged across the three
    dispatches, read by names, sizes and mtimes and never by content. That is the observable effect of
    `autoMemoryEnabled: false`.
- **Recorded, not asserted:** the `skills` and `slash_commands` counts. Built-in ones remain under any source
  setting.
- **A reported model that is a different spelling of the same model** (say, a dated suffix) **is a failure.** Record A
  then needs a decision on the mapping. The equality is never loosened in place (Halt 7).

**What the (g) record's acceptance cites:** all three probes' printed values, the corpus commit SHA and the frames'
sha256, and the launch records' paths and sha256.

*(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* **The three probes are AUTHORED, compiled under `--features live`, listed as ignored, and NOT RUN**
(D21-105 item 2: *"Every other live or paid step (the live probes of `s8h` and `s8j` …) is prepared to its go-ahead
line and deferred to the review packet"*). `cargo test -p rk-adapter-claude --features live --test it -- --list
--ignored` lists `live::live_denial_channel_preprobe`, `live::live_confinement_and_denial_probe` and
`live::live_init_equality_probe` (listing only). Each opens with the launch-record guard, binds the SHIPPED cards
through the SHIPPED `[role.*]` rows and `[models]` table of `.redkiln/packs/bug-standard.toml` (read by a flat-table
reader in `live.rs`: the crate takes no `toml` edge), prints its values one per line, and writes its raw frames and
retained stdout under the printed `PROBE_DIR` for O-9's route. Probes 0 and 1 read their premises BEFORE spending:
`ClaudeHarness::argv_for` (the dispatch's own assembly site, spawning nothing) must carry `--setting-sources ""`
and §8h.6's `--settings` element for their worktree, and probe 0's `ARGV_MODE_SPELLING` must equal the pin's
`PERMISSION_MODE_ARGV` row. Their worktree is `<repo>/.redkiln/worktrees/rk-probe-<token>` (`git worktree add
--detach … HEAD` under `LC_ALL=C`, realpath asserted under `.redkiln/worktrees/`), removed by the probe (only the
tree it created) with `git worktree prune --dry-run` asserted empty.

**Deferred gate — the go-ahead, for the review packet.** Preconditions, each a STOP if unmet: (a) `tools/live-run.sh`
exists with its `probe` mode (the launcher package); (b) `s8j` §8j.6 step 1's pin move is done (§8h.8); (c) §8h.6's
`--settings` element is wired into `Binder::bind`/`dispatch` — ~~BLOCKED at this build on RT-1's cap (§8h.6's dated
note), so probes 0 and 1 refuse at their premise, costing nothing, until it is ruled~~ *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* wired by
file delivery per the 2026-10-02 ruling; (d) the shipped packs carry
Record A's `[models]` table and role rows (the `cards::` package). From a plain terminal, never a Claude Code session
(§8h.7), in this order, each on its own line of the operator's go-ahead:

```text
tools/live-run.sh probe live::live_denial_channel_preprobe       # one haiku dispatch; HALT rules as above
tools/live-run.sh probe live::live_confinement_and_denial_probe  # one Sonnet dispatch; only after probe 0 passes
tools/live-run.sh probe live::live_init_equality_probe           # three dispatches: haiku, sonnet, opus
```

The launcher runs `cargo test -p rk-adapter-claude --features live --test it -- --ignored --nocapture <test> --exact`
and asserts `test <test> ... ok` and `1 passed` (§8h.7 step 7). Fixtures: the shipped cards
`.redkiln/cards/{implementer,planner,investigator,reviewer}.json`, probe 0's test-local card in `live.rs`, the
sentinels each probe creates (`outside-read.txt` directly under `.redkiln/worktrees/`, `outside-write.txt` in a temp
directory outside the repository). The go-ahead line the operator writes, dated, in the ledger: *"GO-AHEAD: run `s8h`
§8h.11 probe <0|1|2> as authored at <commit>; this spend is outside D21-105 item 2 (S-BIN only) and is authorised by
this line, not by the AFK session."*

*(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* **The probes under file delivery.** Unmeasured until `s8j` (D21-105 item 2); each is owed to the
probe named.
- **The pre-dispatch premise** (probes 0 and 1, `settings_premise`): the `--settings` element of the argv
  `ClaudeHarness::argv_for` assembles is an absolute path outside the worktree, in the live settings directory
  `<repo>/.git/redkiln/child-settings/live-probes/` (the stand-in name: `argv_for` writes nothing), and the settings fit
  the 2 MiB limit. **After each dispatch** (`settings_delivered`, probes 0, 1, 2 and `live_dispatch_probe`): the
  record's `settings_file` names a file that exists outside the worktree, whose bytes hash to the recorded sha256;
  `SETTINGS_FILE` and `SETTINGS_FILE_SHA256` are printed.
- **Probe 2** asserts that the file-delivered settings took effect: the init's tools and mode (its existing checks) and
  the auto-memory file set (unchanged). It now reads them as evidence that the file was honoured.
- **Probe 1 gains an arm (built):** step 5 tells the child to Read the settings file itself, its path predicted before
  the bind from the name's three parts (`settings_file_name(pid, next_settings_seq(), …)`) and checked against the
  record after the dispatch (a miss is INCONCLUSIVE, printed as `SETTINGS_READ_AIMED_AT_THE_DELIVERED_FILE`). The read
  must be denied: a `blockReadsOutsideWorkingDirectories` key in the retained stream is a HALT (containment finding),
  and guard 3's channels must now name four denials, not three.
- **Owed arms, authored as text here and NOT built:** a negative arm with a missing or unparsable `--settings` path,
  where the CLI must fail closed (non-zero exit or an error), never drop the flag silently; and a size arm near
  2 MiB. Each needs an argv the adapter refuses to build, so each is a `wrap` child at `s8j`, written there.
- **Exploratory only, never a fence of record without a new record:** option (3)'s `[!...]` negated-class patterns
  (only after probe 1 measures `[!...]` matching and Windows case-folding against a sibling sentinel) and 2.1.287's
  `--restricted` flag.

*(dated 2026-10-02, s8h build, package P8-launcher-checkpoint-edits-and-packet)* **Precondition (a) is met, and nothing was run.** `tools/live-run.sh` exists with its
`probe` mode (§8h.7's dated note). Precondition (d) is met by the cards package. Preconditions (b) and (c) are as dated
above: (b) is `s8j`'s pin move, and (c) is wired. The three go-ahead lines above, `live_dispatch_probe`'s
(§8h.8 step 4), the fixtures and the go-ahead sentence citing D21-105 item 2 are copied verbatim into the review
packet's `s8h` section (`D:/repos/redkiln-sev/afk-tools/sp/s8h/P8-review-packet-s8h.md`), together with §8h.8's freeze instruction. No probe ran: D21-105 item 2
authorises no spend outside S-BIN.

### 8h.12 Red first — the named tests

Each test is authored and **run red for the named reason** before its code, per golden constraint 12. Names are
module-qualified. The new modules, `rk-cli`'s `cards` and `schemas`, are added to that crate's `tests/it/main.rs`
[measured at `a6349b6`: neither exists]. **54 new tests in all** (*D21-48 (2026-09-24):* 51 → 54, for X12's three
`command::` tests, §8h.2). D21-46's `answer` mode added two to the draft's 39,
and D21-47's fix round added nine to that 41: four in `rk-adapter-claude`, three in `rk-types`, one in `rk-kernel` and
one in `rk-cli`. *Final consistency pass 2026-09-24:* N24's ingest check adds one in `rk-adapter-claude` (§8h.4), so 50
becomes 51.

**Every cargo run of this stage runs under D21-47's shared wrapper, `runbook/tools/nodispatch.sh`**, which Checkpoint
8h (0c) also sources (s8h review #5). That covers each Red and each Green run, and every mutant cycle (§8h.13). Once
per shell, `. runbook/tools/nodispatch.sh && nd_setup "$PWD/target/s8h/nd" || exit 1`; then each run is
`nodispatch cargo …`, and `nd_check <label>` follows it. That gives:
- the compiled fake `claude` first on PATH, and every other PATH directory holding a `claude` dropped;
- `CLAUDE_CONFIG_DIR` at an empty trap, checked empty afterwards;
- `REDKILN_LAUNCH_RECORD` removed.

From this stage on the cards bind, and `harness_binary` defaults to `claude`. A binary test that carried a launch
record and no fixture `harness_binary` would otherwise dispatch on the terminal's real account.

**`rk-adapter-claude` (25 new; 22 before D21-48).** Red reasons and N17's classes are in brackets.
- `card::permission_mode_outside_the_cli_closed_set_is_refused` [`"acceptEdit"` binds today]
- `card::every_cli_permission_mode_is_admitted_by_the_closed_set` [the control against an over-narrow set]
- `isolation::permission_mode_reported_unequal_to_the_card_is_refused` [`NotIsolated { surface: Env }`]
- `isolation::permission_mode_absent_from_init_is_refused` [the same class]
- `isolation::model_reported_unequal_to_the_requested_id_is_refused` [requested `claude-sonnet-5`, reported
  `claude-sonnet-4-6`: the segment match adopts it today; the class is `NotAdopted`]
- `isolation::model_equal_to_the_requested_id_proceeds`
- `isolation::api_key_source_other_than_none_is_refused` [`NotIsolated { surface: Env }`]
- `isolation::api_key_source_absent_is_refused` [the same class]
- `classify::permission_denials_non_empty_is_a_typed_failure` [`NotIsolated { surface: Env }`]
- `classify::permission_denials_empty_is_not_a_failure`
- `classify::permission_denials_outrank_a_valid_structured_output`
- `classify::a_denial_under_a_protected_worktree_segment_is_named_the_known_limit` [a `<wt>/.cargo/x` denial has
  the same class plus a `ProtectedSegmentDenial` note naming `.cargo`. Control: a denial outside the worktree
  carries no note, and neither does a path whose only `.cargo` segment is above the worktree root]
- `classify::a_permission_denied_stream_event_is_a_typed_failure` [D21-47, s8h #2: an empty result list beside one
  `permission_denied` event (drafted frame, `decision_reason_code` `outside_reads_blocked`) is
  `NotIsolated { surface: Env }`; today it is `Ok`]
- `classify::denial_paths_are_extracted_per_tool` [s8h #10: `file_path` for `Read`/`Edit`/`Write`, `path` for
  `Glob`/`Grep`, an absolute `pattern` for `Glob`, `""` otherwise; `<wt>/.config/git/x` carries the note, and
  `<wt>/.config/x` and `<wt>/git/x` do not]
- `command::setting_sources_is_explicit_and_empty`
- `command::child_settings_carry_the_worktree_confinement` [the deny set is the ancestor-chain complement, no `!`,
  plus the block setting]
- `command::child_settings_disable_auto_memory` [s8h #7: the `--settings` element carries `autoMemoryEnabled: false`]
- `command::permission_mode_spelling_rows_are_listed_by_their_help_fixtures` [D21-48 X12: there is no
  `PERMISSION_MODE_ARGV` and no `tests/fixtures/cli-help/` at `fc80687`. Every row outside `LIVE_WARRANTED` has
  `cli-help/<version>.permission-mode.txt`, and that capture's parsed `(choices: …)` list holds the row's spelling
  and `acceptEdits`. Premises: a row exists for the fixture `Config`'s pin; at least one row is checked, so the test
  cannot pass over zero rows; `LIVE_WARRANTED` equals `["2.1.278"]` here and `[]` after `s8j`'s pin move. From then
  on the test reads exactly *"the pinned row's spelling of `default` is in the pinned `--help` list"*. Control: the
  same parser over a synthetic capture whose choices lack the row's spelling reports it absent]
- `command::argv_permission_mode_reads_the_row_of_the_configs_pin` [D21-48 X12: today the argv carries the card's
  mode verbatim. A `default` card assembled under a `Config` pinned `2.1.281` carries `--permission-mode manual`, and
  under `2.1.278` it carries `default`. In both, the `--agents` JSON's `permissionMode` is `default`, the card digest
  is equal, and guard 2 adopts an init reporting `default`. An `acceptEdits` card carries `acceptEdits` under both.
  The pin-move commit turns the `2.1.278` arm into a refusal arm when it deletes that row]
- `command::a_pin_without_a_spelling_row_is_refused` [D21-48 X12: today a `Config` pinned `9.9.9` assembles an argv.
  `assemble` refuses it with a typed error naming `9.9.9`, before any spawn. Control: the fixture `Config`'s own pin
  assembles]
- `binder::the_full_model_id_reaches_argv_and_the_card_digest` [two ids for one row key move the digest]
- `binder::a_tier_absent_from_the_models_table_is_refused`
- `dispatch::observed_model_is_recorded_on_every_path_that_read_init` [`None` before init, never the requested id]
- `live_guard::every_live_test_opens_with_the_launch_record_guard` [s8h #6, a code-line lint over `live.rs`: at
  `fc80687` five `#[ignore]` tests carry no guard. The module `live_guard` is new, is outside the `live` feature,
  and is added to `tests/it/main.rs`]
- `classify::a_spec_criterion_without_an_authored_verification_is_a_schema_failure` [N24 (§8h.4); final consistency
  pass 2026-09-24: a `Spec` structured output with one criterion whose `verification` is absent, and one whose
  `verification` is `""`, each fails as `StepError::Schema`; today both are `Ok`. Control: the same `Spec` with a
  non-empty `verification` on every criterion is `Ok`]
  *(dated 2026-10-02, s8h build, package P3-adapter-argv-models)* Ten of these ran red first through D21-47's wrapper (`nd.sh`), each for its
  stated reason, then green: `card::permission_mode_outside_the_cli_closed_set_is_refused` *"`acceptEdit` must be
  refused at load (s8h §8h.2), got Ok(AuthoredCard { name: "probe", … })"*; `command::setting_sources_is_explicit_and_empty`
  *"--setting-sources occurs exactly once in ["-p", …]"* (left 0, right 1); `command::child_settings_carry_the_worktree_confinement`
  *"the read fence is set"* (left `Null`, right `Bool(true)`) and `command::child_settings_disable_auto_memory`
  *"autoMemoryEnabled is present and false"* (left `Null`, right `Bool(false)`), both with the element pushed over a `{}`
  scaffold; `command::argv_permission_mode_reads_the_row_of_the_configs_pin` *"2.1.287: a `default` card's
  --permission-mode is the row's spelling"* (left `"default"`, right `"manual"`); `command::a_pin_without_a_spelling_row_is_refused`
  *"a pin with no spelling row must be refused naming 9.9.9 (X12), got Ok(["-p", …])"*;
  `binder::the_full_model_id_reaches_argv_and_the_card_digest` *"the bound card carries the row's full id"* (left
  `"sonnet"`, right `"claude-sonnet-5"`); `binder::a_tier_absent_from_the_models_table_is_refused` *"a role naming a key
  absent from [models] must be refused at bind (s8h §8h.3), got Ok(ActivitySpec { … })"*. The two control tests,
  `card::every_cli_permission_mode_is_admitted_by_the_closed_set` and
  `command::permission_mode_spelling_rows_are_listed_by_their_help_fixtures`, were green over the declared table and
  set before any enforcing code (a control reads a declaration), so their reds were taken against the failure each
  exists for, in the private worktree (§8h.13's driver): the set narrowed to the shipped pair, *"`auto` is a CLI
  choice (2.1.287 help) and must be admitted"*; the `2.1.287` row removed, *"PREMISE: at least one help-warranted row
  was checked"*. Green: the ten `... ok`, `test result: ok. 10 passed; 0 failed`; `rk-adapter-claude --test it` whole
  122 passed / 0 failed (112 before + 10). The help-warranted arm is keyed `2.1.287`, not `2.1.281` (§8h.2's dated
  note). `argv_permission_mode_reads_the_row_of_the_configs_pin`'s clause *"guard 2 adopts an init reporting
  `default`"* is not asserted by this package: no init permission-mode guard exists in its tree; it is the isolation
  package's (`isolation::permission_mode_reported_unequal_to_the_card_is_refused`). Regressions moved by dated line:
  `command::ac214_permission_mode_explicit` (the table's spelling at the fixture pin, §8h.2), the AC-216 matrix
  (`KNOWN_FLAGS` gains `--setting-sources`/`--settings`; the mode axis is the closed set, since a free spelling is now
  refused before an argv exists), `command::argv_cap_refuses_before_spawn` (`at-the-cap.card.json`'s prompt
  re-trimmed by 1,825 characters, the measure the new elements add), the two `binder::` full-id readings, and the
  golden fixtures and both `insta` snapshots, which moved by the four new elements only (`--permission-mode` stays
  `default` at 2.1.278).
  *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* **Fifteen ran red first through D21-47's wrapper (`nd.sh`), each for its stated reason, then
  green.** Red, at runtime over a scaffold (the new `Evidence`/`Classified`/`Run` members and the classify helpers
  present as stubs, no behaviour): the six `isolation::` guards *"… must be NotIsolated { surface: env } … got Ok(…)"*
  (mode unequal, mode absent, `apiKeySource` other, `apiKeySource` absent) and *"requested claude-sonnet-5, reported
  claude-sonnet-4-6: adoption is EXACT equality … expected NotAdopted, got Ok(…)"*;
  `classify::permission_denials_non_empty_is_a_typed_failure` *"exit Some(0): a non-empty permission_denials must be
  NotIsolated … got Err(Schema { … no structured_output … })"*; `…outrank_a_valid_structured_output` *"… never Ok … got
  Ok(…)"*; `…a_denial_under_a_protected_worktree_segment_is_named_the_known_limit` (left `None`, right the digest);
  `…a_permission_denied_stream_event_is_a_typed_failure` *"… got Ok(…)"*; `…denial_paths_are_extracted_per_tool` (left
  `""`, right `"/a/r.txt"`); `…a_spec_criterion_without_an_authored_verification_is_a_schema_failure` *"absent: … must
  be StepError::Schema at ingest … got Ok(…)"*; `dispatch::observed_model_is_recorded_on_every_path_that_read_init`
  (left `None`, right `Some(ObservedModel("claude-haiku-4-5-20251001"))`); `live_guard::…` *"5 #[ignore] tests but 0 guard
  statements"*, *"live_dispatch_probe: its #[ignore] reason names `cargo test`"*. The two controls,
  `isolation::model_equal_to_the_requested_id_proceeds` and `classify::permission_denials_empty_is_not_a_failure`, were
  green before any enforcing code (a control), so their reds were taken against the failure each exists for in the
  private worktree (§8h.13: I1, I2). Green: the fifteen `... ok`, `test result: ok. 15 passed; 0 failed`;
  `rk-adapter-claude --test it` whole 137 passed / 0 failed (122 + 15); Checkpoint 8h (3)'s 31-name selection `test
  result: ok. 31 passed; 0 failed`; (7)'s listing names the three probes; (9)'s floors `permission_denials` 1,
  `apiKeySource` 1, `model_belongs_to_tier` 0, each filter proved on its synthetic pair; `rk-cli --test it` 310 passed /
  0 failed after the seven frame fixtures moved (§8h.3's note). Regressions: `classify::ac229_…` went red once on two
  `_ =>` arms in the new helpers (rewritten as an if-chain and a `filter`), and `isolation::ac204_adoption_at_2_1_277_…`
  carries a dated doc line; the `exit::ac438_*` snapshots did not move (no `StepError` variant was added).

**`rk-types` (4 new).**
- `dispatch_record::observed_model_is_a_typed_member_absent_before_init`
- `findings::severity_is_a_required_member_typed_by_the_11c_vocabulary` [cross #4: `Finding` has no `severity`
  today; a finding without one is refused]
- `findings::a_severity_outside_the_11c_vocabulary_is_refused` [a spelling outside the record's set fails to
  deserialize]
- `root_cause::discriminating_test_is_a_required_member` [cross #5: `RootCause`'s schema requires the member as a
  string]
  *(dated 2026-10-02, s8h build, package P2-rk-types)* Red, each through D21-47's wrapper (`nd.sh`): the two
  `findings::` and the `root_cause::` test ran red at runtime before any type existed — *"the record's spelling
  `blocking` must be accepted on a finding, got Err(Malformed { detail: "unknown field `severity`, …" })"*, the
  out-of-vocabulary test's control *"control: the record's spelling `blocking` is accepted"*, and *"`RootCause` is not
  registered in registry::EXPORTED"*; then `dispatch_record::` ran red by compile, `error[E0609]: no field
  `observed_model` on type `DispatchRecord`` and `error[E0560]: struct `RecordTail` has no field named
  `observed_model``. Green: all four `... ok`, and Checkpoint 8h (4)'s ten-name selection `test result: ok. 10 passed;
  0 failed`. The `findings::` tests read the vocabulary from ADR-0168 by its id (`0168-*.md`, decision 1's sentence),
  never from this file.

AC-133's registry and its completeness tests are `s8g`'s, and this stage adds none. Its new types' enums are entered
in that registry, and `s8g`'s two `judgement_registry::` tests (§8h.0 item 10) are selected in Checkpoint 8h (4) as
regressions. They go red until the entries exist.

**`rk-kernel` (7 new).**
- `envelope::dispatch_cap_is_declared_never_defaulted`
- `envelope::the_dispatch_at_the_cap_proceeds`
- `envelope::the_dispatch_past_the_cap_is_refused_with_its_class`
- `envelope::the_dispatch_count_survives_a_second_process` [two runtimes over one store; a one-hop fixture cannot
  judge a multi-hop guard]
- `envelope::token_appetite_past_the_appetite_refuses_the_next_dispatch`
- `envelope::a_dispatch_with_no_usage_still_counts_against_the_cap`
- `quality_block::a_severity_row_reads_the_typed_member_over_a_non_empty_list` [cross #4: a
  `findings.none(severity in [...])` row over one finding reads the typed member; today it is `UnknownField`.
  Control: over an empty list the row is true]
  *(dated 2026-10-02, s8h build, package P5-kernel-envelope)* Red, each through D21-47's wrapper (`nd.sh`). First, with the new API
  absent, the binary failed to compile (19 errors, among them `unresolved imports ... DispatchCap, ... EnvelopeRefusal, ...
  ItemEnvelope`, `no method named with_envelope`, `no variant named DispatchCapNotDeclared`). Then, with the types and
  accessors present and no check in `dispatch_once_at`, the runtime reds were each the dispatch past a limit reaching the port:
  - `the_dispatch_past_the_cap_…`, `a_dispatch_with_no_usage_…` and `the_dispatch_count_survives_…` read `left: Ok(true)`,
    `right: Err(Envelope(DispatchCapReached { item: "INC-1", count: 3, cap: DispatchCap(3) }))`;
  - `token_appetite_past_the_appetite_…` read `left: Ok(true)`, `right: Err(Envelope(TokenAppetiteReached { item: "INC-1",
    sum: Tokens(5000), appetite: Tokens(5000) }))`;
  - `dispatch_cap_is_declared_never_defaulted` read `left: None`, `right: Some(DispatchCap(3))` with the parse stubbed, then
    `left: None`, `right: Some(DispatchCap(40))` on `bug-standard.toml` before the installed packs carried the key.
  `the_dispatch_at_the_cap_proceeds` is the control: red by compile only, and its discrimination is instrument I1 (§8h.13).
  The quality_block red is I6 (§8h.4's note). Green: Checkpoint 8h (5)'s nine-name selection `test result: ok. 9 passed;
  0 failed`. `rk-kernel` (whole package) 417 passed / 0 failed / 5 ignored in `it`, plus 12 in its lib.

**`rk-cli` (18 new).**
- `cards::shipped_cards_are_the_least_privilege_table`
- `cards::shipped_role_rows_resolve_through_the_models_table`
- `cards::the_models_table_pins_the_three_full_ids`
- `cards::shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class`
- `cards::shipped_rt57_scalars_carry_no_no_record_marker`
- `schemas::every_agent_output_kind_the_two_lanes_reach_is_exported` [it asserts the universe equals §8h.4's 24 as a
  PREMISE, so a catalog change that reaches a new kind fails loudly]
- `schemas::bind_reads_the_schema_from_exported_by_name`
- `schemas::a_kind_absent_from_exported_refuses_at_bind`
- `doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set` [fixtures `Bash`, `PowerShell`,
  `WebFetch`, `WebSearch`, `mcp__x__y`; control: the shipped cards are clean]
- `doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair`
- `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load` [a counting fake sees 0 dispatches]
- `run::a_gate_expiry_row_naming_no_reachable_class_is_refused`
- `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` [the store's row count and the
  worktree set are unchanged]
- `run::a_real_harness_run_with_a_launch_record_passes_the_launch_check` [the control. Its fixture config names
  `env!("CARGO_BIN_EXE_rk-fixture-child-cli")` as `harness_binary` by absolute path, and it asserts that the spawned
  harness is that fixture child and that the trap configuration directory stays empty (s8h #5)]
- `run::every_binary_spawn_withholds_or_supplies_the_launch_record` [D21-47's lint (§8h.7): exactly one code line
  under `tests/it` constructs a `Command` over the `redkiln` binary, inside the helper that calls
  `.env_remove("REDKILN_LAUNCH_RECORD")`. At `fc80687` 13 sites construct one]
- `answer::a_binary_answer_without_a_launch_record_refuses_before_any_row` [D21-46: the binary's `answer`, driven
  through the shared spawn helper with its fixture record withheld. The store's row count is unchanged and the gate
  is still open]
- `answer::a_binary_answer_with_a_launch_record_passes_the_launch_check` [the control: the same gate is answered]
- `exit::an_envelope_refusal_leaves_through_overage` [N17's Q3: a dispatch refused at the cap exits 20, and
  `exit::ac438_exit_table_snapshot` does not move]
  *(dated 2026-10-02, s8h build, package P5-kernel-envelope)* Over `pack-min` (`appetite.dispatches = 40`), the counting store holds 40
  keyed `dispatch` rows of `bug-1` written by an earlier process. Red, with the explicit `Envelope` arm removed (the catch-all
  then maps the refusal to row 3): `left: Ok(ConfigError)`, `right: Ok(Overage)`. The verb's own stderr was *"redkiln: the
  runtime boundary refused a dispatch: the per-item envelope refused the dispatch: item `bug-1` has made 40 dispatches …"*.
  Green with the arm: `ok`, the harness reached 0 times and no `dispatch` row appended. `exit::ac438_exit_table_snapshot`
  and `exit::ac438_step_error_mapping_is_exhaustive` are `ok` and unmoved. `rk-cli` (whole package) 311 passed / 0 failed.

**Live (authored, compiled, listed, never run here).** `live::live_confinement_and_denial_probe` and
`live::live_init_equality_probe`, and probe 0's `live::live_denial_channel_preprobe`, each
`#[ignore = "PAID; run only as: tools/live-run.sh probe live::<its name>"]`.

*(dated 2026-10-02, s8h build, package P6-cli-launch-gate)* **The launch tests, red then green, through D21-47's wrapper.**
- `run::every_binary_spawn_withholds_or_supplies_the_launch_record`: red *"the real tree: 21 code lines construct a
  `Command` over the `redkiln` binary; exactly one may, inside the helper"* (its six synthetic controls passed first);
  green after the helper and the fold.
- `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` (in process, `Startup::real()`, an `Env`
  built explicitly): red before the check, `left: Transport`, `right: ConfigError` (the run reconciled and
  dispatched). A second arm, added with the check, hands a record path that names no file.
- `run::a_real_harness_run_with_a_launch_record_passes_the_launch_check` (`harness_binary` rewritten to
  `env!("CARGO_BIN_EXE_rk-fixture-child-cli")`, `CLAUDE_CONFIG_DIR` at an empty trap): red against a check that refused
  every invocation, *"a real-harness run with a launch record spawns the fixture child: left: 0 right: 1"*.
- `answer::a_binary_answer_without_a_launch_record_refuses_before_any_row`: red before the check, `left: Some(0)`,
  `right: Some(3)`.
- `answer::a_binary_answer_with_a_launch_record_passes_the_launch_check`: red against an `answer`-only refusing check,
  `left: Some(3)`, `right: Some(0)`.

All five green together, then `rk-cli`'s suite whole (`lane_e2e`'s binary arm and every binary `answer` test
included).

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **The twelve `cards::`, `schemas::`, `doctor::` and load-lint
`run::` tests, red then green, through D21-47's wrapper (`nd.sh`).** Red, all twelve in one run before any data or
code (`test result: FAILED. 1 passed; 11 failed`):
- `cards::shipped_cards_are_the_least_privilege_table` *"architect: tools ["StructuredOutput"], want {…}"*,
  *"permission_mode None, want "default""*, *"does not load through AuthoredCard::from_file: … no
  `permission_mode`"*;
- `cards::the_models_table_pins_the_three_full_ids` `left: {}`; `cards::shipped_role_rows_resolve_through_the_models_table`
  *"architect: model None, want "opus""*; `cards::shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class`
  *"bug-standard: 0 rows; missing ["adr.accept", …]"*; `cards::shipped_rt57_scalars_carry_no_no_record_marker`
  *"bug-standard still carries a `NO RECORD` marker"* (`left: 5`);
- `doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set` and `…_a_mode_outside_the_shipped_pair`
  *"RK-D-CARD-GRANT is not in RULES — no rule to run over RK-D-CARD-GRANT"*;
- `schemas::bind_reads_the_schema_from_exported_by_name` *"ImplReport: the binder read the decoy file (§8h.4)"*;
  `schemas::a_kind_absent_from_exported_refuses_at_bind` (after its anchor was corrected to the step's own line)
  *"the run spawned the harness over a kind absent from EXPORTED"*, `left: 1`, `right: 0`;
- `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load` and
  `run::a_gate_expiry_row_naming_no_reachable_class_is_refused` `left: Success`, `right: ConfigError`.

`schemas::every_agent_output_kind_the_two_lanes_reach_is_exported` was green at its first run: P2 had registered the
23 types before this test existed. Its red is instrument I5 (§8h.13): `Spec` removed from `EXPORTED` in the private
worktree. Green: the twelve `... ok`, `test result: ok. 12 passed; 0 failed`; Checkpoint 8h (6)'s 22-name selection
`test result: ok. 22 passed; 0 failed`, each `test <name> ... ok` line present; `rk-cli` whole 328 passed / 0 failed
(316 + 12).

### 8h.13 Mutants — named, each run from Python in a throwaway worktree, restore verified

Each mutant is applied by a Python script with an anchor assertion (golden constraint 7) in a
`git worktree add --detach` copy at the stage's HEAD. Its named test is run and must go **red**, then the copy is
removed, and `git diff --quiet` in the main tree is asserted. A survivor blocks. Every cycle runs under D21-47's
shared wrapper, `runbook/tools/nodispatch.sh` (s8h review #5). After `nd_setup "$PWD/target/s8h-mutants/nd"`, the
driver is started once per mutant as `nodispatch python <script> <id>`, so every cargo run it spawns inherits:
- the compiled fake `claude` first on PATH, with every other `claude` directory dropped;
- `CLAUDE_CONFIG_DIR` at an empty trap;
- `REDKILN_LAUNCH_RECORD` removed.

`nd_check <id>` follows each cycle, so the trap and the fake's call log are asserted empty after it.

| id | mutation | must turn red |
| --- | --- | --- |
| M1 | the closed-set check replaced by the old non-empty test | `card::permission_mode_outside_the_cli_closed_set_is_refused` |
| M2 | the init mode comparison deleted | `isolation::permission_mode_reported_unequal_to_the_card_is_refused` |
| M3 | model equality relaxed to the pre-stage segment match over the row key | `isolation::model_reported_unequal_to_the_requested_id_is_refused` |
| M4 | the `apiKeySource` comparison deleted | `isolation::api_key_source_other_than_none_is_refused` |
| M5 | `permission_denials` ignored | `classify::permission_denials_non_empty_is_a_typed_failure` |
| M6 | the denial arm moved after the `structured_output` validation | `classify::permission_denials_outrank_a_valid_structured_output` |
| M7 | the `--setting-sources` push deleted | `command::setting_sources_is_explicit_and_empty` |
| M8 | `AgentCard.model` given the row key instead of the id | `binder::the_full_model_id_reaches_argv_and_the_card_digest` |
| M9 | the cap comparison off by one (`>` for `>=`) | one of `envelope::the_dispatch_at_the_cap_proceeds` / `…past_the_cap…` |
| M10 | the dispatch count taken from an in-process counter | `envelope::the_dispatch_count_survives_a_second_process` |
| M11 | the gate-expiry load lint skipped | `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load` |
| M12 | the grant row admits the `mcp__` prefix | `doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set` |
| M13 | `bind_nodes` reads `.redkiln/schemas/<kind>.json` again | `schemas::bind_reads_the_schema_from_exported_by_name` |
| M14 | the launch-record check deleted | `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` |
| M15 | the launcher's `CLAUDE_CODE_` refusal deleted, in a copy of the script | Checkpoint 8h (10)'s `CLAUDE_CODE_ENTRYPOINT` control |
| M16 | the `ProtectedSegmentDenial` note never attached | `classify::a_denial_under_a_protected_worktree_segment_is_named_the_known_limit` |
| M17 | the envelope refusal mapped to `Exit::ConfigError` | `exit::an_envelope_refusal_leaves_through_overage` |
| M18 | `answer::run`'s launch-record check deleted | `answer::a_binary_answer_without_a_launch_record_refuses_before_any_row` |
| M19 | `wrap` given `run`'s pin comparison at step 6, in a copy of the script | Checkpoint 8h (10)'s `--check wrap` control over a CLI unequal to the pin |
| M20 | `answer`'s pin comparison deleted at step 6, in a copy of the script | Checkpoint 8h (10)'s `--check answer` Halt 2 control |
| M21 | step 4 made `${CLAUDE_CONFIG_DIR:-…}` (inherited, not set), in a copy of the script | Checkpoint 8h (10)'s `CLAUDE_CONFIG_DIR=$cfg` lines under an inherited value, and the scratch `wrap` control's `CHILD_CFG` |
| M22 | step 9's before/after comparison deleted, in a copy of the script | Checkpoint 8h (10)'s moved-CLI `wrap` run, which must exit non-zero with `failed: CLI_MOVED` |
| M23 | the `permission_denied` event arm deleted | `classify::a_permission_denied_stream_event_is_a_typed_failure` |
| M24 | `autoMemoryEnabled` dropped from the `--settings` element | `command::child_settings_disable_auto_memory` |
| M25 | one older live test's guard statement deleted | `live_guard::every_live_test_opens_with_the_launch_record_guard` |
| M26 | the spawn helper's `env_remove` deleted | `run::every_binary_spawn_withholds_or_supplies_the_launch_record` |
| M27 | `wrap`'s argv screen deleted, in a copy of the script | Checkpoint 8h (10)'s three `WRAP_ARGV` controls |
| M28 | `Finding.severity` made an `Option` | `findings::severity_is_a_required_member_typed_by_the_11c_vocabulary` |
| M29 | the `Spec` authored-verification check skipped (final consistency pass 2026-09-24) | `classify::a_spec_criterion_without_an_authored_verification_is_a_schema_failure` |
| M30 | the spelling lookup bypassed, so `--permission-mode` carries the card's mode verbatim (D21-48 X12) | `command::argv_permission_mode_reads_the_row_of_the_configs_pin` (its `2.1.281` arm) |
| M31 | a pin with no row falls back to the card's spelling (D21-48 X12) | `command::a_pin_without_a_spelling_row_is_refused` |
| M32 | the lookup keyed by the constant `TESTED_CLI_VERSION` instead of the `Config`'s pin (D21-48 X12) | `command::argv_permission_mode_reads_the_row_of_the_configs_pin` (its `2.1.281` arm) |
| M33 | the help-capture check accepts a row's spelling without finding it among the parsed choices (D21-48 X12) | `command::permission_mode_spelling_rows_are_listed_by_their_help_fixtures` (its synthetic control) |

*(dated 2026-10-02, s8h build, package P2-rk-types)* **M28 KILLED**, from the Python driver
`D:/repos/redkiln-sev/afk-tools/sp/s8h/P2-rk-types-mutants.py` in a private worktree (`target/wt-P2rkt`, HEAD plus this
build's working diff, removed afterwards; the main checkout's `git diff HEAD` hash equal before and after), through
D21-47's wrapper: the unmutated control `... ok` (1 passed), then *"a finding without `severity` must be refused at
ingest, got Ok(… severity: None …)"*, rc 101, the file's sha256 restored and verified. The same driver ran four
instrument checks for this package's other guards, each with its green control first, all KILLED: `RootCause.discriminating_test`
made an `Option` (`root_cause::discriminating_test_is_a_required_member`); `observed_model` given
`skip_serializing_if` (`dispatch_record::observed_model_is_a_typed_member_absent_before_init`); `FindingSeverity`'s
registry row renamed away (`judgement_registry::ac133_every_exported_enum_is_registered`, AC-133's completeness red,
the instrument §8h.4 names); and `RootCause`'s `rationale` deleted beside the judgement-bearing `Confidence`
(`judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails`).

*(dated 2026-10-02, s8h build, package P3-adapter-argv-models)* **M1, M7, M8, M24, M30, M31, M32 and M33 KILLED**, from the Python driver
`D:/repos/redkiln-sev/afk-tools/sp/s8h/P3-adapter-argv-models-mutants.py` in a private worktree (`target/wt-P3`,
HEAD plus this build's working diff, removed afterwards; the main checkout's `git diff HEAD` sha256 equal before and
after), through D21-47's wrapper, each after an unmutated control run of all eight named tests (`... ok`), each
rc 101 with exactly its named test `FAILED`, and each file's sha256 restored and verified: M1 *"`acceptEdit` must be
refused at load"*; M7 *"--setting-sources occurs exactly once"* (left 0); M8 *"the bound card carries the row's
full id"* (left `"sonnet"`); M24 *"autoMemoryEnabled is present and false (golden-static)"* (left `Null`); M30 and
M32 *"2.1.287: a `default` card's --permission-mode is the row's spelling"* (left `"default"`, right `"manual"`);
M31 *"a pin with no spelling row must be refused naming 9.9.9 (X12), got Ok(…)"*; M33 *"CONTROL: a spelling absent
from the parsed choices must be reported absent"*. The same driver ran two instrument checks for the control tests
(both KILLED, quoted in §8h.12's note) and three negative controls for Checkpoint 8h (9)'s READ-ONLY fragment (the
block's own lines, the five guards other packages own dropped), each passing unmutated and firing mutated: the
`--setting-sources` push commented out, *"command.rs pushes no --setting-sources on a code line"*; the table made
`pub const`, *"does not declare PERMISSION_MODE_ARGV in its one stated form"*; the `2.1.287` row removed, *"the
--help capture for 2.1.287 has no row in PERMISSION_MODE_ARGV"*. `rustfmt` folds the table's rows onto one line, which
the declared-form guard refuses, so the table carries `#[rustfmt::skip]`.

*(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes)* **M2, M3, M4, M5, M6, M16, M23, M25 and M29 KILLED**, from the Python driver
`D:/repos/redkiln-sev/afk-tools/sp/s8h/P4-adapter-guards-and-probes-mutants.py` in a private worktree (`target/wt-P4`:
HEAD, the main checkout's tracked working diff, and its untracked crate files copied; removed afterwards; the main
checkout's state sha256 — `git diff HEAD --binary` plus every untracked crate file — equal before and after), through
D21-47's wrapper, after an unmutated control run of all thirteen named tests (`... ok`), each rc 101 with its named
test `FAILED` and each file's sha256 restored and verified: M2 *"an init reporting permissionMode acceptEdits … got
Ok"*; M3 (the segment match over the row key, `sonnet` a segment of `claude-sonnet-4-6`) *"… expected NotAdopted, got
Ok"*; M4 (the `apiKeySource` row deleted); M5 *"… got Err(Schema …)"*; M6 (the `structured_output` validation taken
before the denial arm) *"… never Ok … got Ok"*; M16 (left `[]`, right the `.cargo` note); M23 *"… got Ok"*; M25 (the
S-RETRY throttle driver's guard deleted) *"every live:: test must open with the launch-record guard"*; M29 *"absent: …
got Ok"*. Six instrument checks, all KILLED: I1 (model compared to the card name) and I2 (an empty list counted as a
denial), the two controls' reds; I3 (the requested id copied into `observed_model`); I4 (Glob's absolute pattern
ignored); I5 (`.config` matched as one segment) *".config alone is not protected"*; I6 (the mode compared to the argv's
`manual` spelling) — the digest moves.

*(dated 2026-10-02, s8h build, package P5-kernel-envelope)* **M9, M10 and M17 KILLED**, from the Python driver
`D:/repos/redkiln-sev/afk-tools/sp/s8h/P5-kernel-envelope-mutants.py`. It ran in a private worktree (`target/wt-P5`: HEAD,
the main checkout's tracked working diff and its untracked crate files, plus a `cargo build --workspace --bins` so
`rk-cli`'s fixture child exists), which was removed afterwards. The main checkout's state sha256 was equal before and after.
Every run went through D21-47's wrapper and came after an unmutated control of every named test (`... ok`). Each mutant
exited rc 101 with exactly its named test `FAILED`, and each file's sha256 was restored and verified.
- M9 (`count >= cap` → `count > cap`) killed `envelope::the_dispatch_past_the_cap_is_refused_with_its_class`: `left: Ok(true)`,
  `right: Err(Envelope(DispatchCapReached { … count: 3 … }))`.
- M10 (the count taken from a per-runtime `AtomicU32` on `Runtime` instead of the store) killed
  `envelope::the_dispatch_count_survives_a_second_process` the same way.
- M17 (the arm returns `Exit::ConfigError`) killed `exit::an_envelope_refusal_leaves_through_overage`: `left: Ok(ConfigError)`,
  `right: Ok(Overage)`.

Six instrument checks were all KILLED:
- I1 (the cap one low) — *"dispatch 3 of a cap of 3 must proceed"*, the at-the-cap control's red;
- I2 (an unkeyed row counted by its payload `item`) — the keyed-only control;
- I3 (cache reads left out of the sum) — the token test;
- I4 (the cap counting `dispatch_completed` rows that carry `usage`) — the usage-less test;
- I5 (`resume_routed` no longer binding the envelope) — the exit test read `Ok(Transport)`, so the production wiring is
  guarded;
- I6 (`Finding.severity` kept off the wire) — *"… got Err(UnknownField("severity"))"*.

A process-static counter, rather than a per-runtime one, would not be killed by the second-process test, which runs both
runtimes in one test process. Production builds one `Runtime` per stage of each `run` process, and M10 is the counter held
on it.

*(dated 2026-10-02, s8h build, package P6-cli-launch-gate)* **M14, M18 and M26 KILLED, and D21-137's guards each
killed**, from the Python driver `D:/repos/redkiln-sev/afk-tools/sp/s8h/P6-cli-launch-gate-mutants.py` (and its D6
re-run, `…-mutants-d6.py`). Each ran in a private worktree (`target/wt-P6`: HEAD, the main checkout's tracked working
diff and its one untracked fixture, `cargo build --workspace` so `rk-cli`'s fixture child exists), removed afterwards;
the main checkout's `git diff HEAD` sha256 was equal before and after. Every cargo run went through D21-47's wrapper,
after an unmutated control of every named test, and each file's sha256 was restored and verified.
- M14 (`run::run`'s launch check deleted) killed `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn`:
  `left: Transport`, `right: ConfigError`. Two siblings were killed by the same test: `ClaudeHarnessFactory`'s
  `builds_the_real_harness` made `false`, and the existence check reduced to an emptiness check (*"a launch record
  naming no existing file refuses too"*).
- M18 (`answer::run`'s check deleted) killed `answer::a_binary_answer_without_a_launch_record_refuses_before_any_row`:
  `left: Some(0)`, `right: Some(3)`.
- M26 (the helper's `env_remove` deleted) killed `run::every_binary_spawn_withholds_or_supplies_the_launch_record`:
  *"the real tree: the helper's body does not `.env_remove` the launch record"*.
- D21-137 (§8h.6), ~~seventeen~~ eighteen *(dated 2026-10-02, s8h build, package F1-D21137-loc: M-D21-137-loc,
  below)* mutants, all killed: the JSON inlined again (D1: (a) and (g)); the recorded sha256 not
  the bytes' (D2: (b) and the write test); the under-worktree comparison always false (D3), without case-folding
  (D4) or without the `\\?\` strip (D5), each by (c); `create_new` made `create` + `truncate` (D6: it SURVIVED the
  first run, because the read-only flag refused the overwrite on its own; the write test then gained a writable file
  at the name and the re-run killed it, *"a writable file at the name is refused, never truncated"*); one name for
  every dispatch (D7: (d)); no size cap (D8: (e)); an unset directory falling back (D9), no existence check (D10) and
  no re-check at dispatch (D11), each by (f); the verbatim (D12), relative (D13) and `..` (D17) refusals dropped, each
  by the relative-or-verbatim test; no read-only flag (D14: (b) and the write test); bind writing the file (D15: (g),
  *"bind writes nothing … left: 1 right: 0"*); the record's `settings_file` dropped (D16: (b)).
- *(dated 2026-10-02, s8h build, package F1-D21137-loc; D21-137 item 2 amended, panel ruling TAKEN PROVISIONALLY under D21-105)* **M-D21-137-loc KILLED**, from `D:/repos/redkiln-sev/afk-tools/sp/s8h/F1-D21137-loc-mutants.py`
  in a private worktree (`target/wt-F1loc`: HEAD plus the main checkout's tracked working diff), removed
  afterwards; the main checkout's `run.rs` sha256 was equal before and after, the file's sha256 restored and
  verified, every cargo run through D21-47's wrapper. `child_settings_dir`'s `.join("child-settings")` retargeted
  to `.join("runs")` turned `run::a_child_settings_file_survives_write_run_for_its_run_id` red: *"the settings file C:/Users/ryanm/AppData/Local/Temp/.tmp6bIsHW/.git\redkiln\runs\implement-4807c4db9441017d\child-settings-0000035544-00000000000000000000-4bc6981cac98ba34.json was deleted by write_run over its run_id (D21-137 item 3: never delete it during the run or after spawn)"*. The same
  run is the guard's red-first reading. An unmutated control before it and the restored tree after it each read
  1 passed, after `cargo build --workspace` in the worktree. A first run without that build failed both arms,
  the unmutated one included, so its kill reading was discarded and the run repeated with the control.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **M11, M12 and M13 KILLED**, from the Python driver
`D:/repos/redkiln-sev/afk-tools/sp/s8h/P7-cli-binding-data-and-amendments-mutants.py`, in a private worktree
(`target/wt-P7`: HEAD, the main checkout's tracked working diff and its untracked files, `cargo build --workspace`),
removed afterwards; the main checkout's state sha256 (`git diff HEAD --binary` plus every untracked file) was equal
before and after. Every cargo run went through D21-47's wrapper, after an unmutated control of all seven named tests
(`... ok`, 7 passed); each mutant exited rc 101 with its named test `FAILED`, and each file's sha256 was restored and
verified.
- M11 (the lint's refusal skipped, `if false && !refusals.is_empty()`) killed
  `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load`: `left: Success`, `right: ConfigError`.
- M12 (the grant row admits the `mcp__` prefix) killed
  `doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set`: *"RK-D-CARD-GRANT did not name
  `mcp__x__y` on `cards/mcp.json`"*.
- M13 (`bind_one` reads `.redkiln/schemas/<kind>.json` again, falling back to `EXPORTED`) killed
  `schemas::bind_reads_the_schema_from_exported_by_name`: *"ImplReport: the binder read the decoy file (§8h.4)"*.

Five instrument checks, all KILLED: I1 (the Unmatched half alone disabled) and I2 (the misspelt-row half alone
disabled), each by its own `run::` test; I3 (the mode half of `RK-D-CARD-GRANT` disabled) by
`doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair`; I4 (an absent kind bound to a stand-in schema) by
`schemas::a_kind_absent_from_exported_refuses_at_bind` (*"the run spawned the harness"*); I5 (`Spec` removed from
`EXPORTED`) by `schemas::every_agent_output_kind_the_two_lanes_reach_is_exported` (*"1 lane-reached kind(s) have no
EXPORTED type … ["Spec"]"*), that test's red.

*(dated 2026-10-02, s8h build, package P8-launcher-checkpoint-edits-and-packet)* **M15, M19, M20, M21, M22 and M27 KILLED**, from the Python driver
`D:/repos/redkiln-sev/afk-tools/sp/s8h/P8-launcher-mutants.py`.
- **How each mutant was run.** Each was applied to a COPY of the script: the private worktree `target/wt-P8`'s
  `tools/live-run.sh`, whose sha256 was asserted equal to the main checkout's before the run. Each was restored and
  its sha256 verified after it ran. The main checkout's launcher was unchanged at the end, sha256 `329a7db5…`.
- **The control.** Every mutant ran Checkpoint 8h (10), extracted alone, under D21-47's wrapper. The unmutated
  baseline ran first: rc 0, and the sentinel was reached. Each mutant's run then exited rc 1, and its named control
  was the first `FAIL`:
  - **M15** (step 2's `CLAUDE_CODE_*` arm deleted): *"FAIL: the launcher accepted CLAUDE_CODE_ENTRYPOINT=cli"*. That
    name is not on the denylist, so step 3 does not mask the mutant. `CLAUDE_CODE_USE_BEDROCK` would have been
    masked.
  - **M19** (`wrap` given the pin comparison): *"FAIL: --check wrap refused a CLI unequal to the adapter pin …"*.
  - **M20** (`answer`'s comparison deleted): *"FAIL: --check answer accepted a CLI unequal to the pin …"*.
  - **M21** (`${CLAUDE_CONFIG_DIR:-…}`): *"FAIL: --check did not print 'CLAUDE_CONFIG_DIR=…'"*. A per-arm reading in
    a scratch repository also showed the `wrap` child receiving `CHILD_CFG=D:/rk-inherited-elsewhere`. Unmutated,
    the child receives the scratch home's directory.
  - **M22** (step 9's comparison made `if false`): *"FAIL: a wrap whose child moved the CLI exited 0 — step 9 did
    not compare (M22)"*.
  - **M27** (the screen's call deleted): *"FAIL: --check wrap accepted './target/debug/redkiln run rk-x' …"*. A
    per-arm reading found all three `WRAP_ARGV` argvs ACCEPTED under the mutant, and each REFUSED unmutated.
- **Records and side effects.** No launch record was written under this repository. The scratch repositories were
  removed.

### 8h.14 What this stage may not do, what it makes provisional, and what it hands on

- It may not dispatch, run a `live::` test, move the pin, or edit the operator's user settings.
- It may not create `.redkiln/schemas/`, copy the denylist, or build a second AC-133 registry.
- It may not loosen an equality to make a probe pass.
- It may not move the run-worktree root, or touch the legacy `rk-fix-doctor-test-count` tree. Both are `s8d-ii`'s and
  `s8j`'s (D21-44).

**The checkpoints this stage makes provisional, and where each is re-run.** The rule is to re-run the entry gate
whole, plus the older checkpoints this stage's own edits make provisional. It does not re-run the whole chain.

| checkpoint | why provisional | re-run where |
| --- | --- | --- |
| 8g | the entry gate | Checkpoint 8h (13), whole |
| 5 | `rk-types`: `observed_model`, the new `EXPORTED` types | Checkpoint 8h (13), whole |
| 7b, 7b-ii | `rk-kernel`: the per-item envelope | Checkpoint 8h (13), whole |
| 7d | `rk-cli`: the launch check, `RK-D-CARD-GRANT`, the load lint, the envelope's exit | Checkpoint 8h (13), whole, under (0c)'s wrapper. *Dated 2026-09-24 (D21-47, s8h review #1):* 7d's Halt 2 line and `doctor`'s `RK-D-CLI-VERSION` meet the fake `claude`, which prints the pin, so here they assert the fake. Both meet the real binary only at `s8j`, after the pin move (§8h.8) |
| 8b | `rk-types`: `Finding` gains `severity`, which `s8b`'s severity-reading rows and fixtures meet (D21-47 cross #4) | Checkpoint 8h (13), whole |
| 7c | `rk-adapter-claude`: argv, init equality, classification | **not here.** Its block carries the live probe. `s8j` re-runs it split (D21-47). The suite portion runs outside `wrap`, under the no-launch-record, trap and fake-`claude` wrapper. Only the live-probe portion runs under `wrap`, from a plain terminal (D21-43 N28). Its non-live regressions are selected by name in (3) |
| 7e | `.redkiln/cards/` and both packs | **not re-runnable whole**: its block ends in `git add .redkiln spikes/p9 && git commit`. Its subject is asserted by (8)'s parser read of the cards and both packs, by the `cards::` tests in (6), and by the load lint's tests. `conformance::shipped_pack_loads` loads the catalog only, so it is not cited for this subject (s8h review #14) |
| 7f | its `redkiln run` arm now refuses at the launch check. Before that check exists, the same arm would reach a real dispatch once the cards bind | `s8j`'s entry-chain run, as described below |
| 8a | its (11) guard, *"a card carries permission_mode (rc=$rc) — 7i's or 7f's run could reach a REAL dispatch; stop"*, fires on the eight cards this stage writes | **amended here**, in this stage's work commit (D21-47's time-bound rule; below). `s8j` re-runs it whole |
| 8c | its (15) guard, the same `pm=$(grep -l 'permission_mode' "$@")` shape, fires on the same cards (final consistency pass 2026-09-24) | **amended here**, in the same commit and the same way. `s8j` re-runs it whole |
| 8d | its (12a) precondition, *"a shipped card carries permission_mode, so s8h has landed without its dated amendment of (12a)"*, fires on the same cards. `s8d` §8d.11 asks for this: *"**`s8h` amends (12a)** by dated line in the commit that gives the cards their modes, and it lists 8d as provisional"* (final consistency pass 2026-09-24) | **amended here**, card-mode half only; its fixture-child half stands. `s8j` re-runs it whole |
| 8d-ii, 8e | 8d-ii's non-nested precondition and 8e's (10d) stop once a card carries a mode, because each re-runs Checkpoint 7f directly and 7f's run arm reaches `redkiln run` (final consistency pass 2026-09-24) | **not amended here.** Their stop is right from this stage until `s8j`'s A10 re-points 7f's arm at the fixture child with a fixture launch record. A10 moves their premise, so A10's commit amends them under the same rule (finding 27); `s8j` §8j.8 specifies both edits under *"A10's two dependent amendments"* |

**How a later stage re-runs an old checkpoint that executes `redkiln run`.** Of 7f, 7g, 7h and 7i's run arm, only
7f's block executes `redkiln run` at `a6349b6` [measured by a grep over the four files' bash lines]. Every re-run of
such a block after this stage has to do four things:
- point `[adapters] harness_binary` at `target/debug/rk-fixture-child-claude` through a fixture config, never the
  shipped `.redkiln/config.toml` left edited (D21-47: *"The canonical fixture harness is `rk-fixture-child-claude`"*);
- run under D21-47's wrapper: the fake `claude` first on PATH, the trap `CLAUDE_CONFIG_DIR`, and no launch record
  except the fixture one this route supplies;
- supply a fixture `REDKILN_LAUNCH_RECORD`;
- back up the store and restore it;
- assert that no real `claude` was spawned, for example by showing no new session transcript under either
  configuration directory.

7f's block no longer touches the legacy tree. `s8d-ii` §8d-ii.7 moved its run arm into a throwaway clone and deleted
its `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count` line, so the five things above apply
inside that clone. All of this is `s8j`'s. This stage states it and runs none of it.

**This stage amends Checkpoint 8a (11) in its own work commit** (D21-47: *"A time-bound guard in an earlier
checkpoint is amended by the stage that moves its premise, in the same commit"*). The eight cards this stage writes
carry `permission_mode`, so 8a's `pm=$(grep -l 'permission_mode' "$@"); rc=$?` and its `[ "$rc" -eq 1 ]` guard would
stop every later replay of 8a. Those bytes are quoted from `s8a` at `fc80687`; if `s8a`'s own fix round moves them,
its bytes at this stage's HEAD govern, and the intent below does not change. The amendment is a dated comment line
plus two edits in `s8a` Checkpoint 8a (11):
- the card precondition (the `set --` count, the `grep -l` and its guard) stays inside the `RK_CP_NESTED` non-nested
  branch, where `s8a`'s final text already has it (final consistency pass 2026-09-24). D21-47: *"The `RK_CP_NESTED` skip wraps every card or permission precondition."*
- the `grep -l` line and its guard are replaced by an assertion that all eight cards carry `permission_mode`, with a
  dated comment. From this stage on, 8a's no-dispatch premise is the launch check (§8h.7), which 8a's existing
  `env -u` control and trap already feed, and not the cards' lack of a mode.

8a is listed as provisional in this stage's tracker row, and `s8j` re-runs it whole.

*(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments)* **Done, in this stage's work change.** `s8a` Checkpoint 8a (11),
`s8c` Checkpoint 8c (15) and `s8d` Checkpoint 8d (12a) each carry a dated comment and, in place of the card-mode guard
(whose replaced lines are quoted in the comment): an assertion that all eight shipped cards carry `permission_mode`
(`grep -l '"permission_mode"'` lists exactly eight), and
`run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` run by name through `nodispatch`, asserting
its `... ok` line and `1 passed; 0 failed`, followed by `nd_check`. Each stays inside its block's non-nested branch;
8a's gains a synthetic control (a card without a mode is not listed), 8c's keeps its planted-mode control, and 8d's
fixture-child half stands. The three amended fragments, extracted from their own bytes and run read-only under the
shared wrapper, printed `AMENDED 8a OK`, `AMENDED 8c OK`, `AMENDED 8d OK`; the all-eight assertion refuses a
scratch copy with one card's mode removed (7 listed). `s8a`'s two prose statements of the old premise carry dated
strikes. 8d-ii's and 8e's stops are NOT amended (the table above; `s8j`'s A10): both sit in non-nested branches, so
Checkpoint 8h (13)'s nested replays do not reach them.

*Final consistency pass 2026-09-24:* **the same amendment reaches Checkpoints 8c (15) and 8d (12a)**, in the same
commit, because the same cards falsify the same premise there (the table above). In each, the card-mode guard is
replaced by two assertions, and both stay inside the block's non-nested branch, where each already sits:
- all eight shipped cards carry `permission_mode`;
- `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` (§8h.12), run by name under the block's
  wrapper, reports its `... ok` line. From this stage on, that test is the no-dispatch premise the replays rest on,
  because each replay runs with `REDKILN_LAUNCH_RECORD` removed.

The same second assertion joins 8a's amendment. `s8j` §8j.8 A11 drafts that reading for 8a, and it is taken into
this amendment rather than applied twice (finding 28). 8d's (12a) keeps its fixture-child half, which is still what
keeps its process tests off a real `claude`. 8c and 8d are listed as provisional in this stage's tracker row, and
`s8j` re-runs both whole.

**If `s8b` pins the non-empty `UnknownField` behaviour** that cross #4 corrects (a `severity` row over a non-empty
findings list), in a test or a checkpoint line, this stage's work commit amends that line by the same rule, because
adding `Finding.severity` moves its premise.

**What it hands on:**
- **To `s8j`:**
  - `s8`'s three `redkiln run` sites go through `tools/live-run.sh run` (§8h.7; D21-43).
  - The 7f / 8a route above.
  - The pin move and both probes, run from a plain terminal (`s8j` §8j.6).
  - *D21-48 (2026-09-24):* §8h.2's spelling edit, in the pin-move commit. The `2.1.278` row and its `LIVE_WARRANTED`
    entry are deleted, and the golden argv fixtures and snapshots move in their `--permission-mode` element (X12).
  - Checkpoint 7c's re-run, split as D21-47 takes it: the suite portion outside `wrap`, and only the live-probe
    portion under `wrap`. `wrap` does not compare to the pin, and 7c's block does (§8h.7).
  - Probe 0, run before probe 1 in its own block on its own go-ahead (one haiku dispatch), and the correction of
    guard 3's channel (b) from its retained frame if the frame differs from the draft (§8h.5, §8h.11).
  - Checkpoint 8a's whole re-run after this stage's amendment of its (11). *Final consistency pass 2026-09-24:* also
    8c's and 8d's after their amendments, and, in A10's commit, the amendment of 8d-ii's and 8e's card-mode stops
    (finding 27).
  - `s8j`'s answers go through `tools/live-run.sh answer`, both §8j.3 step 5's hold answer and §8j.2's attach
    (D21-46). The mode exists here, and the edit to those lines is `s8j`'s.
  - The rows for its three gate classes, landed in the change that mints them (§8h.10). Their spellings are added to
    Checkpoint 8h's `MINTED_CLASSES` line by a dated edit in that change (final consistency pass 2026-09-24: the line is
    a literal).
- **To `s8i`:**
  - Record B must be accepted before S-BIN spends (D21-42 19c).
  - S-BIN launches through `wrap` (§8h.7). The adapter pin does not bind the corpus runner, and D21-43 N22 freezes
    the CLI from S-BIN on. **This withdraws the draft's finding 3**, which said an `s8i` dispatch would refuse at
    the init line. That holds only for a dispatch through `rk-adapter-claude`, and S-BIN makes none.
- **To `s8i` and `s8j`: their dispatch scans refuse this checkpoint as written** (finding 10).

### 8h.15 Checkpoint 8h

- The eight cards are §8h.2's table. The closed-set lint, the init equality (mode, model, auth) and the denial
  mapping each fail when absent, each through its own named test.
- `[models]` pins the three ids, the role rows resolve through it, and `observed_model` is recorded.
- Every one of the 24 lane-reached kinds is an `EXPORTED` Rust type, read by name. Their enums are entered in
  `s8g`'s AC-133 registry, and its two completeness tests pass.
- `--setting-sources ""` and the confinement settings are on every argv.
- *D21-48 (2026-09-24):* the argv's spelling of a card's `default` comes from one closed table keyed by the pin.
  Every help-warranted row has its `--help` capture, and a pin with no row refuses (X12). At this stage's pin the argv
  still says `default`.
- The launcher refuses what it must and captures what it must, in each of `run`, `wrap` and `answer`. Only `wrap`
  skips the pin comparison. A real-harness run, or a binary `answer`, without a launch record refuses before any row.
- The per-item cap and appetite refuse across processes.
- An authored `Spec` whose criterion lacks a `verification` is refused at ingest (N24; §8h.4).
- No `NO RECORD` marker remains. Both packs carry fail-closed gate rows for every reachable class, including the
  minted gate kinds. An unmatched class refuses at load, and `RK-D-CARD-GRANT` refuses a grant outside the union.
- A protected-segment denial inside the worktree keeps its class and carries the named note (D21-45).
- Record A is `proposed`. Record B is written, and the operator's signatures are a stage-close gate (AC-276:
  *"routed to a human gate; not machine-decidable"*).
- The two probes and probe 0 compile and are ignored. **No dispatch happened.**
- Checkpoint 8g passes whole, and so do Checkpoints 5, 7b, 7b-ii, 7d and 8b, which this stage's crate edits make
  provisional (§8h.14). Checkpoint 7c's shell guards pass.
- The launcher sets `CLAUDE_CONFIG_DIR` and `DISABLE_AUTOUPDATER` rather than inheriting them. A non-check `wrap`
  writes a launch record with every member, and fails on a CLI that moved under it. `wrap` refuses a `run`,
  `answer` or live `--ignored` argv.
- No `REDKILN_LAUNCH_RECORD` is in the environment, so nothing re-run here can pass the launch check.
- The whole suite is at or above Checkpoint 8g's recorded count plus 54 (*D21-48 (2026-09-24):* was 51), with 0 failed
  over at least 44 results.
- The block's own cargo runs and every replay run under D21-47's shared wrapper, `runbook/tools/nodispatch.sh`,
  sourced at (0c): the compiled fake `claude` first on PATH, printing the pin, and every other `claude` directory
  dropped; `CLAUDE_CONFIG_DIR` at an empty trap; and `REDKILN_LAUNCH_RECORD` removed. `nd_check` passes after the
  cargo runs, after each replay and at (14). Under `RK_CP_NESTED=1`, (13) replays nothing.
- The record ids and inputs are literals in the block, and it refuses while an id is not yet written.

**Run it from a plain terminal: a Git Bash that Claude Code did not spawn** (7c's guard, repeated here). It takes no
input from the environment. Its INPUTS lines carry literals, each written by a dated edit (D21-47: *"Records are
located by the id recorded at write time"*):
- `REC_A` and `REC_B`, written on the day each record is written. The block refuses while either is empty;
- `A_SUPERSEDES`, the ids Record A supersedes: `0126 0074`. N17 took Q1 as reuse, and `severity` moves ADR-0074
  decision 1's finding shape (D21-47 cross #4);
- `TOKENS_APPETITE`, Record B's value, 24000000 as drafted. The operator's signed value replaces it by dated edit;
- `MINTED_CLASSES`, written at stage open from §8h.0 item 7. The block refuses while it is empty. A later stage
  that mints a class adds it by dated edit in the change that mints it (`s8j`'s three, §8h.10).

Each re-run checkpoint carries its own ids the same way, so a re-runner supplies nothing. *Final consistency pass
2026-09-24:* this replaces the environment inputs that the earlier revisions demanded. Leave `REDKILN_LAUNCH_RECORD`
unset. `BASE_8G` in (12) is edited at stage open (§8h.0 item 8). Budget hours: (13) runs six checkpoints whole, unless
`RK_CP_NESTED=1` is set, in which case it runs none (D21-46). The block needs `rustc` on PATH for the shared wrapper's
compiled fake `claude`, which cargo's toolchain provides.

*(dated 2026-10-02, s8h build, package P8-launcher-checkpoint-edits-and-packet)* **The literal inputs, confirmed, and no FLIP_PENDING form.**
- **The inputs are confirmed to have landed as dated literals.** These are:
  - `REC_A=0197` and `REC_B=0198`;
  - `MINTED_CLASSES="landing.refused-path render.structured-drift verify.ac-discriminates"`;
  - `A_SUPERSEDES` and `TOKENS_APPETITE`, as drafted;
  - in (12), `BASE_8G=1337` with `FLOOR` at 1337 + 54 + 9. *(dated 2026-10-02, s8h build, package F1-D21137-loc)*
    Now 1337 + 54 + 9 + 1: D21-137 item 2's survival guard (§8h.6).
- **(2) needs no FLIP_PENDING form.** It asserts `status: proposed` for Record A and `status: (proposed|accepted)` for
  Record B. No line of the block asserts `status: accepted` for this stage's own records, so the FLIP_PENDING form
  (Checkpoint 8f (2), dated 2026-09-29) is not needed and is not added.
- **(10) passed in isolation** (§8h.7's dated note).
- **The replayed checkpoints' guard lines.** The read-only, context-free single-line guards of the checkpoints that
  (13) replays (8g, 5, 7b, 7b-ii, 7d and 8b) were run one by one on the main checkout, from
  `D:/repos/redkiln-sev/afk-tools/sp/s8h/P8-readonly-guards.py`. That script's synthetic red control fired first.
  - 65 lines ran green and 0 ran red.
  - 314 lines that read a variable set elsewhere in their block were not run alone. They run in the formal replays.
- **No new `[[bin]]` was added**, so Checkpoint 7d's manifest loop needs no amendment.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`. It DISPATCHES NOTHING: it never
# passes `--ignored` to a test run without `--list`, never runs tools/live-run.sh outside `--check`, and (14) asserts
# the store gained no dispatch row. Every cargo reading is `out=$(…); rc=$?`; every selection is `--test it`,
# module-qualified and `--exact`, asserting each `test <name> ... ok` line AND an exact count.

# ---- (0) THIS BOX, THEN CHECKPOINT 7c's THREE SHELL GUARDS FROM s7c §7c.21 (the re-run the entry asks for) — verbatim
# ---- except that their messages' section refs are qualified `s7c §…`, as a bare ref would name this file's section.
case "$(uname -s)" in MINGW*|MSYS*) : ;; *) echo "FAIL: this block runs on the Windows box; the POSIX arm is ARMED to CI and is not this block's evidence"; exit 1;; esac
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — a Claude-spawned shell puts CLAUDECODE/CLAUDE_CODE_* into the probe's captured map (s7c §7c.15)"; exit 1; }
[ -z "${ANTHROPIC_FOUNDRY_API_KEY:-}" ] || { echo "FAIL: ANTHROPIC_FOUNDRY_API_KEY is set in this shell — it is on the committed denylist (s7c §7c.6) and Config::new will refuse; run the block under 'env -u ANTHROPIC_FOUNDRY_API_KEY bash' (never set_var)"; exit 1; }
env | grep -qE '^(GITHUB_TOKEN|GH_TOKEN|GH_ENTERPRISE_TOKEN)=' && { echo "FAIL: a tracker credential (GITHUB_TOKEN / GH_TOKEN / GH_ENTERPRISE_TOKEN — RT-25, AC-509) is set in this shell — it is on the committed denylist and Config::new will refuse; unset it outside the process"; exit 1; }
# ---- (0b) THIS STAGE'S ADDITION: the three session names outside the CLAUDE_CODE_ prefix (§8h.7 step 2).
env | grep -qE '^(CLAUDE_PID|CLAUDE_EFFORT|AI_AGENT)=' && { echo "FAIL: CLAUDE_PID, CLAUDE_EFFORT or AI_AGENT is set — a Claude Code session exported it; run this block from a plain terminal (§8h.7)"; exit 1; }

# ---- INPUTS, RECORDED AT WRITE TIME (D21-47: a record is located by the id recorded when it was written, never by a
# ---- title phrase or an "exactly one" count). REC_A and REC_B are written into these lines by a dated edit on the day
# ---- each record is written; MINTED_CLASSES by a dated edit at stage open (§8h.0 item 7), and again by any later
# ---- stage in the change that mints a class. A_SUPERSEDES and TOKENS_APPETITE carry their drafted values; the
# ---- operator's signed appetite replaces the second by dated edit. Re-runners supply nothing. Empty means not yet
# ---- written, and the block stops here.
# *(dated 2026-10-02, s8h build, package P1-records)* REC_A and REC_B: ADR-0197 and ADR-0198, the ids recorded when
# the records were written. MINTED_CLASSES: the three kernel-minted classes read from their &str constants at stage
# open (s8h §8h.0 item 7): tier_one::LANDING_REFUSED_KIND, fsm::DRIFT_GATE_KIND, verify::ac_gate::AC_GATE_KIND.
REC_A=0197
REC_B=0198
A_SUPERSEDES="0126 0074"
TOKENS_APPETITE=24000000
MINTED_CLASSES="landing.refused-path render.structured-drift verify.ac-discriminates"
printf '%s\n' "$REC_A" | grep -qxE '0[0-9]{3}' || { echo "FAIL: REC_A (Record A's four-digit id) is not recorded in this block — write it by dated edit the day the record is written"; exit 1; }
printf '%s\n' "$REC_B" | grep -qxE '0[0-9]{3}' || { echo "FAIL: REC_B (Record B's four-digit id) is not recorded in this block — write it by dated edit the day the record is written"; exit 1; }
[ "$REC_A" != "$REC_B" ] || { echo "FAIL: REC_A and REC_B record the same id ($REC_A)"; exit 1; }
printf '%s\n' "$TOKENS_APPETITE" | grep -qxE '[1-9][0-9]*' || { echo "FAIL: TOKENS_APPETITE is not a positive integer: '$TOKENS_APPETITE'"; exit 1; }
test -n "$MINTED_CLASSES" || { echo "FAIL: MINTED_CLASSES is not recorded in this block — write the kernel-minted gate classes read at stage open, space-separated, by dated edit (§8h.0 item 7; §8h.10's table)"; exit 1; }
# No launch record may be in reach: every real-harness `run` in this block and in the checkpoints (13) re-runs then
# refuses before any spawn (§8h.7), and (14) reads the store for the dispatch rows that would say otherwise.
[ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is set in this shell — unset it; nothing this block runs may pass the launch check"; exit 1; }

# ---- (0c) D21-47'S NO-DISPATCH WRAPPER, the shared helper runbook/tools/nodispatch.sh, for this block's own cargo runs
# ---- AND every replay in (13). nd_setup compiles the fake `claude` (a claude.exe: a Rust `Command::new("claude")` on
# ---- Windows resolves `claude.exe` from PATH and never a shell script, and 7d's `doctor` probe spawns it that way,
# ---- s8h review #1), puts it first on PATH, drops every other PATH directory holding a claude, points
# ---- CLAUDE_CONFIG_DIR at an empty trap, and proves each part by a control. nd_enter exports it into this shell (the
# ---- global form). Under an enclosing wrapper, when a later checkpoint replays this block, nd_setup reuses that
# ---- wrapper. ND_REAL_CLAUDE is the claude resolved before the wrapper, and only (11)'s live reading uses it.
test -f runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh is absent — D21-47's shared wrapper"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh"; exit 1; }
nd_setup "$PWD/target/s8h-whole/nd" || exit 1
nd_enter || exit 1
[ -n "${ND_REAL_CLAUDE:-}" ] || { echo "FAIL: no claude was on PATH before the wrapper — (11)'s live reading has no binary to read"; exit 1; }
pin=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9.]+)";$/\1/p' crates/rk-adapter-claude/src/config.rs)
test -n "$pin" || { echo "FAIL: the pin's anchored form did not match"; exit 1; }
wrapdir="$PWD/target/s8h-whole"
mkdir -p "$wrapdir" || { echo "FAIL: could not create $wrapdir"; exit 1; }

wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store"; exit 1; }
launches="$(git rev-parse --git-common-dir)/redkiln/launches"
# NO-DISPATCH, MEASURED in s8a's `dcount` form (s8h review #5): dispatch and dispatch_completed rows in the store AND
# in every run journal, before and after, with a seeded control so a blind counter cannot read as "unchanged".
dcount() {  # $1 = store path; prints "<store rows> <journal rows>"
  python - "$1" "$(dirname "$1")/runs" <<'PY'
import glob, os, sqlite3, sys
db, runs = sys.argv[1], sys.argv[2]
c = sqlite3.connect("file:" + db.replace(os.sep, "/") + "?mode=ro", uri=True)
s = c.execute("select count(*) from t2_event where kind in ('dispatch', 'dispatch_completed')").fetchone()[0]
j = 0
for f in glob.glob(os.path.join(runs, "*", "events.jsonl")):
    j += sum(1 for line in open(f, encoding="utf-8") if '"kind":"dispatch_completed"' in line or '"kind":"dispatch"' in line)
print(s, j)
PY
}
mkdir -p "$wrapdir/dctl/runs/r1" && rm -f "$wrapdir/dctl/store.sqlite3" || { echo "FAIL: cannot make the dispatch-count control"; exit 1; }
python - "$wrapdir/dctl/store.sqlite3" <<'PY' || { echo "FAIL: cannot seed the dispatch-count control store"; exit 1; }
import sqlite3, sys
c = sqlite3.connect(sys.argv[1])
c.execute("create table t2_event (id integer primary key, kind text, payload text, anchor_sha text)")
c.execute("insert into t2_event (kind, payload) values ('dispatch', '{}')")
c.commit()
PY
printf '{"kind":"dispatch_completed","payload":{}}\n' > "$wrapdir/dctl/runs/r1/events.jsonl" || { echo "FAIL: cannot seed the control journal"; exit 1; }
[ "$(dcount "$wrapdir/dctl/store.sqlite3")" = "1 1" ] || { echo "FAIL: CONTROL — the dispatch counter did not read 1 1; the no-dispatch guard is blind"; exit 1; }
disp_before=$(dcount "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$disp_before" ] || { echo "FAIL: cannot count dispatch rows before the run (rc=$rc)"; exit 1; }
nl_before=$(find "$launches" -type f 2>/dev/null | wc -l)

# ---- (1) BUILD-OR-FAIL, FIRST (Halt 1). Nothing below may read a binary this did not just build.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8h NOT RUN"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }
nd_check "(1) build" || exit 1

# ---- (2) THE TWO RECORDS: one file each, the ADR-0164 form, Record A proposed and not yet back-linked.
fa=$(ls references/adr/"$REC_A"-*.md 2>/dev/null); fb=$(ls references/adr/"$REC_B"-*.md 2>/dev/null)
[ "$(printf '%s\n' "$fa" | grep -c .)" -eq 1 ] && [ "$(printf '%s\n' "$fb" | grep -c .)" -eq 1 ] || { echo "FAIL: expected exactly one file per record id"; exit 1; }
grep -qxF 'status: proposed' "$fa" || { echo "FAIL: Record A must be proposed until both s8j probes pass (D21-41 (g))"; exit 1; }
grep -qxE 'status: (proposed|accepted)' "$fb" || { echo "FAIL: Record B carries no valid status"; exit 1; }
for f in "$fa" "$fb"; do
  [ "$(grep -c '^### Option — ' "$f")" -ge 2 ] || { echo "FAIL: $f lacks ADR-0085 option headings (D21-42 24a)"; exit 1; }
  grep -qF 'What this forbids' "$f" || { echo "FAIL: $f carries no forbids block"; exit 1; }
  grep -qxF '## Evidence and confidence' "$f" || { echo "FAIL: $f carries no evidence section"; exit 1; }
done
sup=$(grep -m1 '^supersedes:' "$fa"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Record A has no supersedes line"; exit 1; }
for s in $A_SUPERSEDES; do printf '%s\n' "$sup" | grep -qF "\"$s\"" || { echo "FAIL: Record A does not declare it supersedes $s"; exit 1; }; done
for s in $A_SUPERSEDES; do
  sf=$(ls references/adr/"$s"-*.md 2>/dev/null)
  [ "$(printf '%s\n' "$sf" | grep -c .)" -eq 1 ] || { echo "FAIL: expected exactly one file for superseded ADR-$s"; exit 1; }
  if grep -m1 '^superseded_by:' "$sf" | grep -qF "\"$REC_A\""; then echo "FAIL: ADR-$s is back-linked to a PROPOSED record — the link moves in the flip commit (ADR-0092 cl.2.1)"; exit 1; fi
done

# ---- (3)-(6) THE NAMED TESTS, per crate: each `... ok` and an exact count (new + the regression names §8h.12 keeps).
sel() {  # sel <package> <expected count> <tests...>
  pkg=$1; want=$2; shift 2
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $pkg selection rc=$rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted or not selected"; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $pkg did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
sel rk-adapter-claude 31 command::permission_mode_spelling_rows_are_listed_by_their_help_fixtures command::argv_permission_mode_reads_the_row_of_the_configs_pin command::a_pin_without_a_spelling_row_is_refused classify::a_spec_criterion_without_an_authored_verification_is_a_schema_failure classify::a_permission_denied_stream_event_is_a_typed_failure classify::denial_paths_are_extracted_per_tool command::child_settings_disable_auto_memory live_guard::every_live_test_opens_with_the_launch_record_guard card::permission_mode_outside_the_cli_closed_set_is_refused card::every_cli_permission_mode_is_admitted_by_the_closed_set isolation::permission_mode_reported_unequal_to_the_card_is_refused isolation::permission_mode_absent_from_init_is_refused isolation::model_reported_unequal_to_the_requested_id_is_refused isolation::model_equal_to_the_requested_id_proceeds isolation::api_key_source_other_than_none_is_refused isolation::api_key_source_absent_is_refused classify::permission_denials_non_empty_is_a_typed_failure classify::permission_denials_empty_is_not_a_failure classify::permission_denials_outrank_a_valid_structured_output classify::a_denial_under_a_protected_worktree_segment_is_named_the_known_limit command::setting_sources_is_explicit_and_empty command::child_settings_carry_the_worktree_confinement binder::the_full_model_id_reaches_argv_and_the_card_digest binder::a_tier_absent_from_the_models_table_is_refused dispatch::observed_model_is_recorded_on_every_path_that_read_init command::golden_argv_per_profile command::ac214_permission_mode_explicit command::permission_prompts_none_on_every_argv isolation::ac204_adoption_equal_subset_superset_wrong_agent isolation::ac202_isolation_from_system_init_not_argv config::tested_cli_version_is_the_pin_as_a_config_value
sel rk-types 10 findings::severity_is_a_required_member_typed_by_the_11c_vocabulary findings::a_severity_outside_the_11c_vocabulary_is_refused root_cause::discriminating_test_is_a_required_member dispatch_record::observed_model_is_a_typed_member_absent_before_init schema_snapshot::schema_snapshot schema_no_oneof::no_oneof_at_any_node schema_caps::union_and_optional_caps dispatch_record::ac360_dispatch_record_carries_the_usage_fields judgement_registry::ac133_every_exported_enum_is_registered judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails
sel rk-kernel 9 quality_block::a_severity_row_reads_the_typed_member_over_a_non_empty_list envelope::dispatch_cap_is_declared_never_defaulted envelope::the_dispatch_at_the_cap_proceeds envelope::the_dispatch_past_the_cap_is_refused_with_its_class envelope::the_dispatch_count_survives_a_second_process envelope::token_appetite_past_the_appetite_refuses_the_next_dispatch envelope::a_dispatch_with_no_usage_still_counts_against_the_cap envelope::ac029_n_is_declared_never_defaulted conformance::shipped_pack_loads
sel rk-cli 22 run::every_binary_spawn_withholds_or_supplies_the_launch_record cards::shipped_cards_are_the_least_privilege_table cards::shipped_role_rows_resolve_through_the_models_table cards::the_models_table_pins_the_three_full_ids cards::shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class cards::shipped_rt57_scalars_carry_no_no_record_marker schemas::every_agent_output_kind_the_two_lanes_reach_is_exported schemas::bind_reads_the_schema_from_exported_by_name schemas::a_kind_absent_from_exported_refuses_at_bind doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load run::a_gate_expiry_row_naming_no_reachable_class_is_refused run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn run::a_real_harness_run_with_a_launch_record_passes_the_launch_check answer::a_binary_answer_without_a_launch_record_refuses_before_any_row answer::a_binary_answer_with_a_launch_record_passes_the_launch_check exit::an_envelope_refusal_leaves_through_overage doctor::ac415_every_rule_id_has_a_rejecting_fixture exit::ac438_exit_table_snapshot exit::ac438_step_error_mapping_is_exhaustive surface::ac400_help_snapshot_every_verb
# *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* The nine D21-137 tests, an exact count (> 0: a filter that matches nothing is a green).
sel rk-adapter-claude 9 command::a_five_thousand_sibling_chain_assembles_under_the_cap dispatch::the_settings_file_is_the_json_and_its_sha256_is_recorded command::a_settings_dir_under_the_worktree_is_refused dispatch::two_dispatches_write_two_distinct_settings_files command::settings_over_two_mib_are_refused_before_spawn command::a_missing_settings_dir_is_refused binder::binds_over_temp_and_target_tmp_worktrees_succeed_and_write_nothing command::a_relative_or_verbatim_settings_dir_is_refused command::a_settings_file_write_failure_is_refused_and_overwrites_nothing
# *(dated 2026-10-02, s8h build, package F1-D21137-loc; D21-137 item 2 amended, panel ruling TAKEN PROVISIONALLY under D21-105)* The tenth D21-137 test, rk-cli's: item 2's survival guard, an exact count.
sel rk-cli 1 run::a_child_settings_file_survives_write_run_for_its_run_id
nd_check "(3)-(6) named tests" || exit 1

# ---- (7) THE TWO PROBES EXIST, COMPILE UNDER `live`, AND ARE IGNORED — LISTED, NEVER RUN (`--list` runs nothing).
# ---- Finding 10: s8i's and s8j's dispatch scans match this line's feature flag; the scans are theirs to scope.
out=$(cargo test -p rk-adapter-claude --features live --test it -- --list --ignored 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the live feature does not compile (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
for t in live::live_denial_channel_preprobe live::live_confinement_and_denial_probe live::live_init_equality_probe; do
  printf '%s\n' "$out" | grep -qxF "$t: test" || { echo "FAIL: $t is not an IGNORED test under --features live — it would run, or it does not exist"; exit 1; }
done
nd_check "(7) live listing" || exit 1

# ---- (8) THE SHIPPED DATA, read by parsers (json / tomllib), never grepped: cards, [models], role rows, gate rows,
# ---- the envelope keys, and no NO RECORD marker. The expected gate-class set is DERIVED from the catalog and the
# ---- packs' own `when` rows and compared to §8h.10's counts as a PREMISE. EXCEPTIONS is empty until the operator
# ---- signs one; a signed exception amends this list by dated line. The minted gate kinds are an input, and each must
# ---- sit as a string literal on a code line under crates/*/src. Every `.redkiln/` read is one level deep, so no run
# ---- worktree under `.redkiln/worktrees/` is ever walked (D21-44).
out=$(python - "$TOKENS_APPETITE" "$REC_B" "$MINTED_CLASSES" <<'PY'
import glob, json, os, re, sys, tomllib
bad = []
want_cards = {
    "implementer": ({"Read", "Glob", "Grep", "Edit", "Write", "StructuredOutput"}, "acceptEdits"),
    "investigator": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "reviewer": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "architect": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "spec-author": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "planner": ({"StructuredOutput"}, "default"),
    "releaser": ({"StructuredOutput"}, "default"),
    "operator": ({"StructuredOutput"}, "default"),
}
names = sorted(os.path.basename(p)[:-5] for p in glob.glob(".redkiln/cards/*.json"))
if names != sorted(want_cards):
    bad.append("card set %r" % names)
for n, (tools, mode) in want_cards.items():
    d = json.load(open(".redkiln/cards/%s.json" % n, encoding="utf-8"))
    t = d.get("tools") or []
    if set(t) != tools or len(t) != len(set(t)):
        bad.append("%s tools %r" % (n, t))
    if d.get("permission_mode") != mode:
        bad.append("%s mode %r" % (n, d.get("permission_mode")))
    if "model" in d or "effort" in d:
        bad.append("%s carries a tier (AC-118)" % n)
models = {"haiku": ("claude-haiku-4-5-20251001", False), "sonnet": ("claude-sonnet-5", True), "opus": ("claude-opus-5-5", True)}
roles = {"implementer": ("sonnet", "high"), "investigator": ("sonnet", "high"), "spec-author": ("sonnet", "medium"),
         "reviewer": ("opus", "high"), "architect": ("opus", "medium"), "planner": ("haiku", None),
         "operator": ("haiku", None), "releaser": ("haiku", None)}
questions = set()
def walk(v):
    if isinstance(v, dict):
        for k, x in v.items():
            if k == "question" and isinstance(x, str):
                questions.add(x)
            walk(x)
    elif isinstance(v, list):
        for x in v:
            walk(x)
for f in glob.glob(".redkiln/catalog/*.toml"):
    walk(tomllib.load(open(f, "rb")))
if len(questions) != 45:
    bad.append("PREMISE: %d question classes, expected 45" % len(questions))
minted = set(sys.argv[3].split())
if len(minted) < 3:
    bad.append("PREMISE: MINTED_CLASSES names %d classes; s8f mints two and s8g one (§8h.10's table)" % len(minted))
for c in sorted(minted):
    if c in questions or c.startswith("unknown_facet:") or c == "escalation":
        bad.append("PREMISE: MINTED_CLASSES lists %r, which already has a row of another kind; it would be counted twice" % c)
# The minted spellings are read from a &str constant or a constant &[&str] table on a code line (§8h.0 item 7), so a
# class built by format! cannot pass as a literal that merely appears somewhere (s8h review #15).
one = re.compile(r"^(pub(\([a-z]+\))? )?(const|static) [A-Z0-9_]+: &('static )?str = \"([^\"]+)\";$")
table = re.compile(r"^(pub(\([a-z]+\))? )?(const|static) [A-Z0-9_]+: &('static )?\[&('static )?str\] = &\[")
for src, want in (('pub const K: &str = "x.y";', True), ('let k = format!("x.{}", y);', False)):
    if bool(one.match(src)) != want:
        bad.append("PREMISE: the gate-kind constant reader's control misread %r" % src)
consts, nfiles = set(), 0
for f in glob.glob("crates/*/src/**/*.rs", recursive=True):
    nfiles += 1
    inside = False
    for ln in open(f, encoding="utf-8"):
        s = ln.strip()
        if s.startswith("//"):
            continue
        m = one.match(s)
        if m:
            consts.add(m.group(5))
        if table.match(s):
            inside = True
        if inside:
            consts.update(re.findall(r"\"([^\"]+)\"", s))
            if "];" in s:
                inside = False
if nfiles == 0:
    bad.append("PREMISE: no source file read under crates/*/src")
for c in sorted(minted):
    if c not in consts:
        bad.append("PREMISE: minted gate kind %r is spelt by no &str constant or constant table under crates/*/src (§8h.0 item 7)" % c)
EXCEPTIONS = {}
words = {"and", "or", "not", "in", "high", "med", "low", "mandatory", "recommended", "optional", "forbidden"}
expect_axes = {"bug-standard": 5, "story": 7}
expect_rows = {"bug-standard": 51 + len(minted), "story": 53 + len(minted)}
expect_points = {"bug-standard": 40, "story": 60}
for pack in ("bug-standard", "story"):
    raw = open(".redkiln/packs/%s.toml" % pack, encoding="utf-8").read()
    if raw.count("NO RECORD") != 0:
        bad.append("%s still carries %d NO RECORD markers" % (pack, raw.count("NO RECORD")))
    if raw.count("ADR-" + sys.argv[2]) < 5:
        bad.append("%s names Record B fewer than five times (gate deadline, K, expiry, margins, appetite)" % pack)
    d = tomllib.loads(raw)
    m = d.get("models", {})
    if set(m) != set(models):
        bad.append("%s [models] keys %r" % (pack, sorted(m)))
    for k, (mid, eff) in models.items():
        row = m.get(k, {})
        if row.get("id") != mid or row.get("accepts_effort") is not eff:
            bad.append("%s [models.%s] %r" % (pack, k, row))
    for r, (mk, ef) in roles.items():
        row = d.get("role", {}).get(r, {})
        if row.get("model") != mk or row.get("effort") != ef or row.get("family") != "claude":
            bad.append("%s [role.%s] %r" % (pack, r, row))
    app = d.get("pack", {}).get("appetite", {})
    if app.get("dispatches") != 40 or app.get("tokens") != int(sys.argv[1]) or app.get("points") != expect_points[pack]:
        bad.append("%s appetite %r" % (pack, app))
    axes = set()
    for st in d.get("stage", {}).values():
        for rq in st.get("require", []):
            text = re.sub(r"'[^']*'", " ", rq.get("when", ""))
            axes.update(w for w in re.findall(r"[a-z_]+", text) if w not in words)
    if len(axes) != expect_axes[pack]:
        bad.append("PREMISE: %s reads %d axes %r" % (pack, len(axes), sorted(axes)))
    if "change" not in axes:
        bad.append("PREMISE: %s's require rows no longer read change; s8d's R6 still opens unknown_facet:change at triage, so the row count below is wrong" % pack)
    want = set(questions).union({"unknown_facet:" + a for a in axes}, {"unknown_facet:change", "escalation"}, minted)
    rows = d.get("gate_expiry", [])
    got = [r.get("class") for r in rows]
    if "unknown_facet:change" not in got:
        bad.append("%s carries no unknown_facet:change row (s8d's R6 triage gate, §8h.10)" % pack)
    if len(got) != len(set(got)) or set(got) != want or len(got) != expect_rows[pack]:
        bad.append("%s gate rows: %d rows, missing %r, extra %r" % (pack, len(got), sorted(want.difference(got)), sorted(set(got).difference(want))))
    for r in rows:
        if r.get("action") != EXCEPTIONS.get(r.get("class"), "fail"):
            bad.append("%s gate %s action %r is not the signed value" % (pack, r.get("class"), r.get("action")))
if os.path.exists(".redkiln/schemas"):
    bad.append(".redkiln/schemas exists")
print("SHIPPED_DATA_OK" if not bad else "BAD: " + " ;; ".join(bad))
PY
); rc=$?
[ "$rc" -eq 0 ] && [ "$out" = "SHIPPED_DATA_OK" ] || { echo "FAIL: the shipped cards/packs are not §8h.2/§8h.3/§8h.9/§8h.10's (rc=$rc)"; printf '%s\n' "$out" | head -20; exit 1; }

# ---- (9) CODE-LINE FLOORS beneath the named tests — never the assertion. Comment lines are dropped AFTER the search
# ---- status is asserted, and each filter is first proved on a synthetic pair: a token on a code line must be kept, the
# ---- same token on a `//` line must be dropped (a guard satisfied by its own prose is the failure this exists for).
codehits() {  # codehits <regex> <path...>; -H, so a single-file operand still prints the file name the filter needs
  pat=$1; shift
  raw=$(grep -rnHE --include=*.rs -e "$pat" "$@"); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: grep could not run over $* (rc=$rc)"; exit 1; }
  printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -c .
}
ctl=$(mktemp -d)
printf 'let a = "--setting-sources";\n// "--setting-sources" in prose\n' > "$ctl/c.rs"
[ "$(codehits '--setting-sources' "$ctl")" -eq 1 ] || { echo "FAIL: the code-line filter is broken — the synthetic pair did not give exactly one code hit"; exit 1; }
[ "$(codehits '--setting-sources' "$ctl/c.rs")" -eq 1 ] || { echo "FAIL: the code-line filter is broken on a SINGLE-FILE operand — a // line survived because no file name was printed (s8h review #4)"; exit 1; }
[ "$(codehits '--setting-sources' crates/rk-adapter-claude/src/command.rs)" -ge 1 ] || { echo "FAIL: command.rs pushes no --setting-sources on a code line"; exit 1; }
[ "$(codehits 'permission_denials' crates/rk-adapter-claude/src)" -ge 1 ] || { echo "FAIL: no adapter code line reads permission_denials"; exit 1; }
[ "$(codehits 'apiKeySource' crates/rk-adapter-claude/src/init.rs)" -ge 1 ] || { echo "FAIL: init.rs reads no apiKeySource on a code line"; exit 1; }
[ "$(codehits 'REDKILN_LAUNCH_RECORD' crates/rk-cli/src)" -ge 1 ] || { echo "FAIL: rk-cli names no REDKILN_LAUNCH_RECORD on a code line"; exit 1; }
[ "$(codehits 'model_belongs_to_tier' crates/rk-adapter-claude/src crates/rk-cli/src)" -eq 0 ] || { echo "FAIL: the segment-match adoption survives on a code line (RT-27 is settled by equality)"; exit 1; }
[ "$(codehits 'join\("schemas"\)' crates/rk-cli/src)" -eq 0 ] || { echo "FAIL: rk-cli still reads a schemas directory (§8h.4)"; exit 1; }
# *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* No `settings: None` on the non-test bind and dispatch paths, and the
# file delivery present at both (code lines only: a `settings: None` in a comment must neither fail nor pass this).
printf 'settings: None,\n// settings: None,\n' > "$ctl/s.rs"
[ "$(codehits 'settings: None' "$ctl/s.rs")" -eq 1 ] || { echo "FAIL: CONTROL — the code-line filter did not give exactly one hit on a synthetic settings: None pair"; exit 1; }
[ "$(codehits 'settings: None' crates/rk-adapter-claude/src/binder.rs crates/rk-adapter-claude/src/lib.rs)" -eq 0 ] || { echo "FAIL: a non-test bind or dispatch path passes settings: None — the child would run without its settings (§8h.6; D21-137)"; exit 1; }
[ "$(codehits 'settings: Some\(SettingsArg \{' crates/rk-adapter-claude/src/binder.rs)" -eq 1 ] || { echo "FAIL: Binder::bind does not assemble with the settings file (§8h.6; D21-137)"; exit 1; }
[ "$(codehits 'settings: Some\(settings\)' crates/rk-adapter-claude/src/lib.rs)" -eq 1 ] && [ "$(codehits 'write_settings_file\(&settings_path, &settings\)' crates/rk-adapter-claude/src/lib.rs)" -eq 1 ] || { echo "FAIL: dispatch does not assemble with, and write, the settings file (§8h.6; D21-137)"; exit 1; }
# D21-48 X12 (2026-09-24): the argv spelling table is declared on a code line in its one stated form, and every --help
# capture parses, lists acceptEdits and a spelling of the card's `default`, and has its row (§8h.2). Each reader is
# proved on a synthetic input first.
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
printf '  --permission-mode <mode>  Permission mode (choices: "acceptEdits", "default")\n' > "$ctl/help.txt"
hc=$(hchoices "$ctl/help.txt"); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$hc" | grep -qxF default && ! printf '%s\n' "$hc" | grep -qxF manual || { echo "FAIL: CONTROL — hchoices misread a synthetic capture listing acceptEdits and default (rc=$rc)"; exit 1; }
printf '  --model <model>  Model for the current session\n' > "$ctl/nohelp.txt"
if hchoices "$ctl/nohelp.txt" >/dev/null; then echo "FAIL: CONTROL — hchoices read choices from a capture with no --permission-mode line"; exit 1; fi
printf '    ("9.9.9", "manual"),\n//    ("9.9.9", "manual"),\n' > "$ctl/t.rs"
[ "$(codehits '^[[:space:]]*\("9\.9\.9", "(manual|default)"\),$' "$ctl/t.rs")" -eq 1 ] || { echo "FAIL: CONTROL — the row reader did not give exactly one code hit on a synthetic row and its // twin"; exit 1; }
[ "$(codehits '^pub\(crate\) const PERMISSION_MODE_ARGV: &\[\(&str, &str\)\] = &\[$' crates/rk-adapter-claude/src/command.rs)" -eq 1 ] || { echo "FAIL: command.rs does not declare PERMISSION_MODE_ARGV in its one stated form on a code line (§8h.2; D21-48 X12)"; exit 1; }
nh=0
for hf in crates/rk-adapter-claude/tests/fixtures/cli-help/*.permission-mode.txt; do
  test -s "$hf" || continue
  nh=$((nh + 1)); hv=$(basename "$hf" .permission-mode.txt)
  hc=$(hchoices "$hf"); rc=$?
  [ "$rc" -eq 0 ] && printf '%s\n' "$hc" | grep -qxF acceptEdits || { echo "FAIL: $hf lists no acceptEdits among its --permission-mode choices (rc=$rc)"; exit 1; }
  printf '%s\n' "$hc" | grep -qxE 'manual|default' || { echo "FAIL: $hf lists neither manual nor default — X12's premise moved; stop and report (§8h.2)"; exit 1; }
  hvre=$(printf '%s' "$hv" | sed 's/\./\\./g')
  [ "$(codehits "^[[:space:]]*\(\"$hvre\", \"(manual|default)\"\),\$" crates/rk-adapter-claude/src/command.rs)" -eq 1 ] || { echo "FAIL: the --help capture for $hv has no row in PERMISSION_MODE_ARGV (§8h.2: rows and captures are one to one)"; exit 1; }
done
[ "$nh" -ge 1 ] || { echo "FAIL: no --help capture under crates/rk-adapter-claude/tests/fixtures/cli-help/ — the spelling table rests on nothing (§8h.0 item 6)"; exit 1; }
rm -rf "$ctl"

# ---- (10) THE LAUNCHER, BEHAVIOURALLY: `--check` in this repository, and ONE non-check `wrap` pair in a scratch
# ---- repository (never this one: (14) counts this repository's launch records). A fake `claude` stands first on
# ---- PATH, so nothing real is spawned. The clean environment is `env -i` plus what Git Bash needs, PLUS an inherited
# ---- CLAUDE_CONFIG_DIR pointing elsewhere and DISABLE_AUTOUPDATER=0, so every control proves both are SET, not
# ---- inherited (s8h review #3). Every refusal is read by its anchored `live-run.sh: refused: <NAME>` line (§8h.7).
L=tools/live-run.sh
test -f "$L" || { echo "FAIL: $L is absent"; exit 1; }
if od -c "$L" | grep -qF '\r'; then echo "FAIL: $L carries a CR byte (ADR-0164)"; exit 1; fi
out=$(bash -n "$L" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: $L does not parse"; printf '%s\n' "$out"; exit 1; }
# The launcher's controls use SHELL-SCRIPT fakes, deliberately, the one exception to D21-47's compiled fake: the
# launcher is a bash script and resolves `claude` through bash, which runs a script; the controls need a second version
# and a binary that moves (9.9.9, claude.moved), which the pin-printing compiled fake cannot give; and each control's
# PATH is explicit (a scratch fake directory, /usr/bin and /mingw64/bin), so no Windows child of the launcher can find
# a real claude.exe. That premise is asserted first.
for d in /usr/bin /mingw64/bin; do
  for c in claude claude.exe claude.cmd; do
    [ ! -e "$d/$c" ] || { echo "FAIL: $d/$c exists — the launcher controls' explicit PATH would reach a real claude"; exit 1; }
  done
done
fake=$(mktemp -d); bad_fake=$(mktemp -d)
printf '#!/bin/sh\necho "%s (Claude Code)"\n' "$pin" > "$fake/claude"
printf '#!/bin/sh\necho "9.9.9 (Claude Code)"\n' > "$bad_fake/claude"
chmod +x "$fake/claude" "$bad_fake/claude"
base=(env -i "HOME=$HOME" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "CLAUDE_CONFIG_DIR=D:/rk-inherited-elsewhere" "DISABLE_AUTOUPDATER=0")
clean=("${base[@]}" "PATH=$fake:/usr/bin:/mingw64/bin")
badv=("${base[@]}" "PATH=$bad_fake:/usr/bin:/mingw64/bin")
refused() {  # refused <output> <NAME>: the anchored refusal line is present, and LAUNCH_CHECK=ok is not
  printf '%s\n' "$1" | grep -qF "live-run.sh: refused: $2" || { echo "FAIL: no anchored 'live-run.sh: refused: $2' line"; printf '%s\n' "$1"; exit 1; }
  if printf '%s\n' "$1" | grep -qxF 'LAUNCH_CHECK=ok'; then echo "FAIL: the refusal of $2 also printed LAUNCH_CHECK=ok"; exit 1; fi
}
out=$("${clean[@]}" bash "$L" --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the launcher refused a clean environment (rc=$rc)"; printf '%s\n' "$out" | tail -10; exit 1; }
cfg=$(cygpath -w "$HOME/.claude-personal")
for line in "LAUNCH_CHECK=ok" "DISABLE_AUTOUPDATER=1" "CLAUDE_CONFIG_DIR=$cfg" "CLI_VERSION=$pin"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
for v in CLAUDECODE=1 CLAUDE_CODE_ENTRYPOINT=cli CLAUDE_PID=1 CLAUDE_EFFORT=high AI_AGENT=x ANTHROPIC_API_KEY=x anthropic_base_url=x GH_TOKEN=x CLAUDE_CODE_USE_BEDROCK=1; do
  out=$("${clean[@]}" "$v" bash "$L" --check 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: the launcher accepted $v"; exit 1; }
  refused "$out" "${v%%=*}"
done
out=$("${badv[@]}" bash "$L" --check 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the launcher accepted a CLI version unequal to the pin"; exit 1; }
refused "$out" CLI_VERSION
printf '%s\n' "$out" | grep -qF 'Halt 2' || { echo "FAIL: the version refusal does not name Halt 2"; printf '%s\n' "$out"; exit 1; }
# `wrap` (§8h.7): the same refusals, a recorded version, and NO comparison to the adapter pin.
out=$("${clean[@]}" bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: --check wrap refused a clean environment (rc=$rc)"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "LAUNCH_CHECK=ok" "MODE=wrap" "DISABLE_AUTOUPDATER=1" "CLAUDE_CONFIG_DIR=$cfg" "CLI_VERSION=$pin"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check wrap did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
out=$("${clean[@]}" CLAUDECODE=1 bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: --check wrap accepted CLAUDECODE — wrap skips the pin, never the environment refusals"; exit 1; }
refused "$out" CLAUDECODE
out=$("${badv[@]}" bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: --check wrap refused a CLI unequal to the adapter pin — wrap records the version, it does not compare (§8h.7)"; printf '%s\n' "$out" | tail -10; exit 1; }
printf '%s\n' "$out" | grep -qxF "CLI_VERSION=9.9.9" || { echo "FAIL: --check wrap did not record the version it found"; printf '%s\n' "$out"; exit 1; }
# wrap's argv screen (s8h review #16): these have their own modes, which compare the pin. Nothing here runs: --check.
for argv in "./target/debug/redkiln run rk-x" "./target/debug/redkiln.exe answer rk-x approve" "cargo test -p rk-adapter-claude --features live --test it -- --ignored"; do
  out=$("${clean[@]}" bash "$L" --check wrap -- $argv 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: --check wrap accepted '$argv' — it has its own mode (§8h.7)"; exit 1; }
  refused "$out" WRAP_ARGV
done
# `answer` (§8h.7, D21-46): run's refusals, run's pin comparison, and its own argument refusals. The first call is the
# control: the same launcher, the same clean environment and a well-formed argv must pass.
out=$("${clean[@]}" bash "$L" --check answer rk-fix-check-only approve 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: --check answer refused a clean environment (rc=$rc)"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "LAUNCH_CHECK=ok" "MODE=answer" "DISABLE_AUTOUPDATER=1" "CLAUDE_CONFIG_DIR=$cfg" "CLI_VERSION=$pin"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check answer did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
for v in CLAUDECODE=1 CLAUDE_CODE_ENTRYPOINT=cli CLAUDE_PID=1 AI_AGENT=x ANTHROPIC_API_KEY=x; do
  out=$("${clean[@]}" "$v" bash "$L" --check answer rk-fix-check-only approve 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: --check answer accepted $v — answer takes run's environment refusals"; exit 1; }
  refused "$out" "${v%%=*}"
done
out=$("${badv[@]}" bash "$L" --check answer rk-fix-check-only approve 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: --check answer accepted a CLI unequal to the pin — an answer is spent irrevocably, so it is pinned as run is (§8h.7)"; exit 1; }
refused "$out" CLI_VERSION
printf '%s\n' "$out" | grep -qF 'Halt 2' || { echo "FAIL: --check answer's version refusal does not name Halt 2"; printf '%s\n' "$out"; exit 1; }
for argv in "" "-x approve" "rk-fix-check-only"; do
  out=$("${clean[@]}" bash "$L" --check answer $argv 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: --check answer accepted the argument list '$argv' — an item and an option are required"; exit 1; }
  refused "$out" ANSWER_USAGE
done
# The non-check `wrap` pair (s8h review #3), in a scratch repository built from this tree's working bytes, never a
# clone (autocrlf smudges a clone). HOME is a scratch home whose config directory holds an empty .credentials.json, so
# no child sees the operator's. Run 1 is the control: its child moves nothing, prints what it inherited, and reads
# one line from stdin, which must be the one piped to the launcher (§8h.7: the child has the launcher's own stdin).
# In run 2 the child moves the fake CLI, and step 9 must fail naming Halt 2 (M22). Both records must carry every
# member.
sr=$(mktemp -d); sh_home=$(mktemp -d); sw=$(mktemp -d)
out=$(git init -q -b main "$sr" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: could not init the scratch repository (rc=$rc)"; exit 1; }
mkdir -p "$sr/tools" "$sr/crates/rk-adapter-claude/src" "$sr/spikes/results/instrument-checks" "$sh_home/.claude-personal" || { echo "FAIL: could not lay out the scratch repository"; exit 1; }
cp "$L" "$sr/tools/" && cp crates/rk-adapter-claude/src/env_denylist.rs crates/rk-adapter-claude/src/config.rs "$sr/crates/rk-adapter-claude/src/" && cp spikes/results/instrument-checks/RUNBOOK-instrument.txt "$sr/spikes/results/instrument-checks/" || { echo "FAIL: could not copy the launcher's inputs into the scratch repository"; exit 1; }
: > "$sh_home/.claude-personal/.credentials.json" || { echo "FAIL: could not write the scratch home's credentials placeholder"; exit 1; }
out=$(git -C "$sr" -c core.autocrlf=false add -A 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: scratch git add rc=$rc"; exit 1; }
out=$(git -C "$sr" -c core.autocrlf=false -c user.name=cp8h -c user.email=cp8h@invalid commit -q -m scratch 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: scratch git commit rc=$rc"; printf '%s\n' "$out"; exit 1; }
sr_head=$(git -C "$sr" rev-parse HEAD); rc=$?
[ "$rc" -eq 0 ] && [ -n "$sr_head" ] || { echo "FAIL: the scratch repository has no HEAD (rc=$rc)"; exit 1; }
printf '#!/bin/sh\necho "%s (Claude Code)"\n' "$pin" > "$sw/claude" && printf '#!/bin/sh\necho "9.9.9 (Claude Code)"\n' > "$sw/claude.moved" && chmod +x "$sw/claude" "$sw/claude.moved" || { echo "FAIL: could not write the scratch fakes"; exit 1; }
scl=(env -i "HOME=$sh_home" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "CLAUDE_CONFIG_DIR=D:/rk-inherited-elsewhere" "DISABLE_AUTOUPDATER=0" "PATH=$sw:/usr/bin:/mingw64/bin")
want_cfg=$(cygpath -w "$sh_home/.claude-personal")
recchk() {  # recchk <record> <same|moved>: every step-8 member present, and each value the one this run must record
  python - "$1" "$2" "$want_cfg" "$pin" "$sr_head" <<'PY'
import sys
path, how, cfg, pin, head = sys.argv[1:6]
kv = {}
for ln in open(path, encoding="utf-8").read().splitlines():
    k, sep, v = ln.partition("=")
    if sep:
        kv[k] = v
keys = ["MODE", "ARGV", "CLAUDE_PATH", "CLI_VERSION_BEFORE", "CLI_SHA256_BEFORE", "CLAUDE_CONFIG_DIR",
        "DISABLE_AUTOUPDATER", "PIN", "HEAD", "CLI_VERSION_AFTER", "CLI_SHA256_AFTER", "RC"]
bad = ["%s absent or empty" % k for k in keys if not kv.get(k)]
for k, want in (("MODE", "wrap"), ("CLAUDE_CONFIG_DIR", cfg), ("DISABLE_AUTOUPDATER", "1"), ("PIN", pin),
                ("HEAD", head), ("CLI_VERSION_BEFORE", pin), ("RC", "0")):
    if kv.get(k) != want:
        bad.append("%s=%r, expected %r" % (k, kv.get(k), want))
same = kv.get("CLI_VERSION_AFTER") == kv.get("CLI_VERSION_BEFORE") and kv.get("CLI_SHA256_AFTER") == kv.get("CLI_SHA256_BEFORE")
if how == "same" and not same:
    bad.append("the before and after readings differ in a run that moved nothing")
if how == "moved" and (kv.get("CLI_VERSION_AFTER") != "9.9.9" or kv.get("CLI_SHA256_AFTER") == kv.get("CLI_SHA256_BEFORE")):
    bad.append("the move is not recorded in the after readings")
print("RECORD_OK" if not bad else "BAD: " + " ;; ".join(bad))
PY
}
out=$(cd "$sr" && printf 'rk-stdin-8h\n' | "${scl[@]}" bash tools/live-run.sh wrap -- sh -c 'printf "CHILD_CFG=%s\nCHILD_DAU=%s\n" "$CLAUDE_CONFIG_DIR" "$DISABLE_AUTOUPDATER"; read -r l; printf "CHILD_STDIN=%s\n" "$l"; test -f "$REDKILN_LAUNCH_RECORD"' 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — a non-check wrap whose child moves nothing failed (rc=$rc); the child's test -f also fails when no launch record was exported"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "CHILD_CFG=$want_cfg" "CHILD_DAU=1"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: the wrap child did not see '$line' — the launcher let an inherited value through (§8h.7 steps 4-5; M21)"; printf '%s\n' "$out"; exit 1; }
done
printf '%s\n' "$out" | grep -qxF "CHILD_STDIN=rk-stdin-8h" || { echo "FAIL: the wrap child did not read the launcher's stdin — an interactive child would not get the terminal (§8h.7)"; printf '%s\n' "$out"; exit 1; }
rec1=$(find "$sr/.git/redkiln/launches" -type f -name '*-wrap.txt'); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$rec1" | grep -c .)" -eq 1 ] || { echo "FAIL: the control wrap wrote no single launch record (rc=$rc): '$rec1'"; exit 1; }
[ "$(recchk "$rec1" same)" = RECORD_OK ] || { echo "FAIL: the control wrap's launch record is incomplete or wrong: $(recchk "$rec1" same)"; exit 1; }
out=$(cd "$sr" && "${scl[@]}" bash tools/live-run.sh wrap -- cp "$sw/claude.moved" "$sw/claude" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: a wrap whose child moved the CLI exited 0 — step 9 did not compare (M22)"; exit 1; }
printf '%s\n' "$out" | grep -qF 'live-run.sh: failed: CLI_MOVED' || { echo "FAIL: the moved-CLI failure is not the anchored 'failed: CLI_MOVED' line"; printf '%s\n' "$out"; exit 1; }
printf '%s\n' "$out" | grep -qF 'Halt 2' || { echo "FAIL: the moved-CLI failure does not name Halt 2"; printf '%s\n' "$out"; exit 1; }
all=$(find "$sr/.git/redkiln/launches" -type f -name '*-wrap.txt'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: could not list the scratch launch records (rc=$rc)"; exit 1; }
rec2=$(printf '%s\n' "$all" | grep -vxF "$rec1")
[ "$(printf '%s\n' "$rec2" | grep -c .)" -eq 1 ] || { echo "FAIL: the moved-CLI wrap wrote no single second record: '$rec2'"; exit 1; }
[ "$(recchk "$rec2" moved)" = RECORD_OK ] || { echo "FAIL: the moved-CLI wrap's launch record is incomplete or wrong: $(recchk "$rec2" moved)"; exit 1; }
rm -rf "$fake" "$bad_fake" "$sr" "$sh_home" "$sw"

# ---- (11) THE PIN, READ NOT MOVED: the constant equals the instrument file (7c's anchored form). The live binary is a
# ---- READING: at authoring it is 2.1.281 against 2.1.278 — Halt 2 LIVE, recorded, moved once at s8j (§8h.8).
inst=$(sed -nE 's/^([0-9.]+) \(Claude Code\)$/\1/p' spikes/results/instrument-checks/RUNBOOK-instrument.txt)
[ -n "$inst" ] && [ "$inst" = "$pin" ] || { echo "FAIL: TESTED_CLI_VERSION ($pin) and RUNBOOK-instrument.txt ($inst) disagree — a half-taken pin move"; exit 1; }
# The READING uses $ND_REAL_CLAUDE, the binary resolved before (0c)'s wrapper, never the fake, with its own empty
# configuration directory and not the replay trap. Whether `--version` writes there is measured and printed, not
# failed: it is a spawn, not a dispatch (D-11), and keeping it out of the trap keeps (14)'s emptiness a statement about
# replays (cross #12).
vr="$wrapdir/version-reading-config"
rm -rf "$vr" && mkdir -p "$vr" || { echo "FAIL: could not create the version reading's configuration directory"; exit 1; }
live_v=$(CLAUDE_CONFIG_DIR="$(cygpath -w "$vr")" DISABLE_AUTOUPDATER=1 "$ND_REAL_CLAUDE" --version 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: claude --version rc=$rc"; exit 1; }
if [ "$live_v" != "$pin (Claude Code)" ]; then echo "READING: Halt 2 LIVE — installed '$live_v' vs pin $pin; recorded, moved at s8j (D21-42 16), not here"; fi
vw=$(find "$vr" -mindepth 1 | wc -l); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: could not read the version reading's configuration directory (rc=$rc)"; exit 1; }
echo "READING: the real claude --version left $vw entries in an empty CLAUDE_CONFIG_DIR (cross #12)"

# ---- (12) FMT, CLIPPY (the workspace, and the live-gated code the workspace line does not compile), THE WHOLE SUITE.
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo clippy -p rk-adapter-claude --all-targets --features live 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy over the live probes rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the whole suite rc=$rc (a fresh checkout needs pnpm install for the node fixture first)"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
nd_check "(12) fmt, clippy and the whole suite" || exit 1
# The floor is Checkpoint 8g's recorded passed count plus this stage's 54 new tests (§8h.12). Expected value, re-derived
# at stage open from Checkpoint 8g's recorded count (§8h.0 item 8): BASE_8G = 1189, floor 1189 + 54 = 1243. Edit the
# next line by a dated edit before the first run. (Final consistency pass 2026-09-24: was 1127 + 50. D21-48
# (2026-09-24): was 1171 + 51; X12 adds three command:: tests. D21-48 floor propagation (2026-09-24): was
# 1171 + 54, before s8c 78, s8d 62, s8d-ii 50 and s8f 85.)
# *(dated 2026-10-02, s8h build, package P1-records)* BASE_8G 1189 -> 1337, Checkpoint 8g's RECORDED count (D21-129:
# 1337 passed / 0 failed / 9 ignored over 44 at 4777941), so FLOOR = 1337 + 54 = 1391.
# *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY under D21-105)* FLOOR 1391 -> 1400: D21-137 adds nine tests beyond §8h.12's 54 (§8h.6's dated
# note), so FLOOR = 1337 + 54 + 9. The floor is raised, never lowered.
# *(dated 2026-10-02, s8h build, package F1-D21137-loc; D21-137 item 2 amended, panel ruling TAKEN PROVISIONALLY under D21-105)* FLOOR 1400 -> 1401 (D21-137 item 2's survival guard,
# run::a_child_settings_file_survives_write_run_for_its_run_id), so FLOOR = 1337 + 54 + 9 + 1.
BASE_8G=1337
FLOOR=$((BASE_8G + 54 + 9 + 1))
tally() { printf '%s\n' "$1" | awk -v k="$2" '/^test result: /{for(i=1;i<NF;i++) if($(i+1)==k";" || $(i+1)==k) s+=$i} END{print s+0}'; }
[ "$(tally 'test result: ok. 3 passed; 0 failed; 9 ignored; 0 measured; 0 filtered out' passed)" -eq 3 ] || { echo "FAIL: synthetic control — tally does not read a passed count"; exit 1; }
[ "$(tally 'test result: FAILED. 3 passed; 2 failed; 0 ignored' failed)" -eq 2 ] || { echo "FAIL: synthetic control — tally does not read a failed count"; exit 1; }
[ "$(tally 'running 0 tests' passed)" -lt "$FLOOR" ] || { echo "FAIL: synthetic control — the floor admits a run of zero tests"; exit 1; }
passed=$(tally "$out" passed); failed=$(tally "$out" failed)
nres=$(printf '%s\n' "$out" | grep -c '^test result: ')
[ "$failed" -eq 0 ] && [ "$nres" -ge 44 ] && [ "$passed" -ge "$FLOOR" ] || { echo "FAIL: the whole suite read passed=$passed failed=$failed over $nres results — the floor is $FLOOR passed, 0 failed, at least 44 results"; exit 1; }

# ---- (13) RE-RUN WHOLE, FROM THEIR OWN BYTES: Checkpoint 8g (the entry gate) and the five older checkpoints
# ---- this stage's crate edits make provisional (§8h.14's table): 5, 7b, 7b-ii, 7d, 8b. 7c, 7e, 7f and 8a are NOT
# ---- re-run here; the table says where. Each extracted block gets a sentinel, so "rc 0 and the sentinel" means it
# ---- ran to its end. REDKILN_LAUNCH_RECORD is unset (INPUTS), so a real-harness `run` in any of them refuses first.
# ---- BOUNDED (D21-46's one re-run convention): every replay runs with RK_CP_NESTED=1, and when THIS block runs under
# ---- RK_CP_NESTED=1, as a later checkpoint's entry gate, it replays nothing. NO DISPATCH (D21-46's one guard for
# ---- replays, s8g's form, and D21-47's shared wrapper sourced at (0c)): every replay runs as `nodispatch env
# ---- RK_CP_NESTED=1 bash …`, with REDKILN_LAUNCH_RECORD removed, the compiled fake `claude` first on PATH, and
# ---- CLAUDE_CONFIG_DIR at (0c)'s trap; nd_check follows each replay and (14). 7d's Halt 2 line and RK-D-CLI-VERSION
# ---- therefore read the fake, which prints the pin; they meet the real binary at s8j, after the pin move (§8h.14).
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8H (13) SKIPPED: this block is itself a replay (RK_CP_NESTED=1); its re-runs ran at Checkpoint 8h's own close"
else
lr=REDKILN_LAUNCH_RECORD
got=$(export "$lr=planted"; nodispatch sh -c 'printf "%s" "${REDKILN_LAUNCH_RECORD-unset}"'); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = unset ] || { echo "FAIL: CONTROL — nodispatch left a planted $lr in the child (got '$got', rc=$rc); a replay would inherit a launch record"; exit 1; }
extract() {  # extract <file> <anchor|marker> <key> <out>
  python - "$1" "$2" "$3" "$4" <<'PY'
import sys
path, mode, key, out = sys.argv[1:5]
fence = chr(96) * 3
lines = open(path, encoding="utf-8").read().split("\n")
blocks, cur, start = [], None, 0
for i, ln in enumerate(lines):
    if cur is None and ln == fence + "bash":
        cur, start = [], i
    elif cur is not None and ln == fence:
        blocks.append((start, cur))
        cur = None
    elif cur is not None:
        cur.append(ln)
if mode == "marker":
    hit = [b for (s, b) in blocks if ('echo "%s"' % key) in b]
else:
    at = [i for i, ln in enumerate(lines) if ln.startswith(key)]
    hit = [b for (s, b) in blocks if len(at) == 1 and s > at[0]][:1]
if len(hit) != 1:
    sys.exit("FAIL: %d bash blocks in %s match %r, expected exactly 1" % (len(hit), path, key))
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(hit[0]) + "\necho CP8H-WHOLE-RUN-SENTINEL\n")
PY
}
for spec in "8g|runbook/phase-1/s8g-verify-and-restored-demands.md|marker|CP8G-END-OF-BLOCK reached" "5|runbook/phase-1/s5-workspace-ci-rk-types.md|anchor|**Checkpoint 5.** All of:" "7b|runbook/phase-1/s7b-kernel.md|anchor|### 7b.14 Checkpoint 7b" "7b-ii|runbook/phase-1/s7b-ii-gates.md|anchor|### 7b-ii.13 Checkpoint 7b-ii" "7d|runbook/phase-1/s7d-cli-and-migrate.md|marker|CP7D-END-OF-BLOCK reached" "8b|runbook/phase-1/s8b-catalog-quality.md|marker|CP8B-END-OF-BLOCK reached"; do
  IFS='|' read -r name file mode key <<< "$spec"
  test -f "$file" || { echo "FAIL: $file is absent — Checkpoint $name cannot be re-run (Halt 8)"; exit 1; }
  extract "$file" "$mode" "$key" "target/s8h-whole/$name.sh" || { echo "FAIL: could not extract Checkpoint $name"; exit 1; }
  out=$(nodispatch env RK_CP_NESTED=1 bash "target/s8h-whole/$name.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $name run whole rc=$rc — its row stays provisional"; printf '%s\n' "$out" | tail -25; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8H-WHOLE-RUN-SENTINEL' || { echo "FAIL: Checkpoint $name did not reach its last line"; exit 1; }
  nd_check "replay of Checkpoint $name" || exit 1
done
fi

# ---- (14) NOTHING DISPATCHED, NO CLAUDE SPAWNED, NO LAUNCH RECORD WRITTEN, THE WORKTREE PATH SET UNCHANGED (compared
# ---- before any prune). The trap and the fake's call log cover this whole block, not the replays alone.
disp_after=$(dcount "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$disp_after" ] || { echo "FAIL: cannot count dispatch rows after the run (rc=$rc)"; exit 1; }
[ "$disp_after" = "$disp_before" ] || { echo "FAIL: dispatch rows moved '$disp_before' -> '$disp_after' (store, journals) — this block, or a checkpoint (13) re-ran, DISPATCHED"; exit 1; }
nd_check "(14), the whole block" || exit 1
nl_after=$(find "$launches" -type f 2>/dev/null | wc -l)
[ "$nl_after" -eq "$nl_before" ] || { echo "FAIL: a launch record was written — --check must write none"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

echo "CP8H-END-OF-BLOCK reached"
```

**Closing.** The stage closes in two commits: first the work and its checkpoint output, then the tracker.

- **Before the second commit, three gates are passed, and none can be decided by a machine.**
  - The operator's dated ledger line signs Record B **value by value** and **gate row by row** (AC-276). It also
    signs Record A's AC-133 judgement table **enum by enum** (AC-156).
  - Record B flips to `accepted` in the tracker commit (ADR-0092 cl.2.1): the frontmatter, the README index cell
    and the census move, and the body does not.
  - Record A stays `proposed`. `PROGRESS.md` §1 names it as awaiting `s8j`'s two probes.
- **The §8h.13 mutants' outcomes** go into the checkpoint output: twenty-nine, each red, each restore verified.
- **Adversarial refuting rounds** run over the diff until a round moves no code and no guard.

**Findings for the lead, recorded rather than absorbed.** Items 1–9 are the draft's, each with its status after
D21-43/44/45. Items 10–15 are new in that revision, items 16–19 are new in the D21-46 revision, items 20–26 are
new in the D21-47 fix round, and items 27–28 are new in the final consistency pass.

1. **Settled by D21-44.** The draft's blocker was *"the run-owned worktree lives under `.git`"*. D21-44 measured it:
   an edit inside `.git/redkiln/worktrees/…` was refused. The run worktree moves to `.redkiln/worktrees/<item>`,
   which is `s8d-ii`'s (D21-46 R1). Probe 1 re-measures the placement at `s8j` (§8h.11), and D21-45's in-worktree limit is named
   (§8h.5).
2. **`!` negation cannot carve out of an absolute rule**, as documented. So D21-42 21d's deny rules cannot be written
   as "deny all, except the worktree". §8h.6 writes them as the ancestor-chain complement, now over the new root.
   That is a construction choice, and Record A states it.
3. **Withdrawn.** S-BIN does not dispatch through `rk-adapter-claude`, and it launches through `wrap` (§8h.7,
   §8h.14).
4. **Assigned.** `s8`'s three `redkiln run` sites are `s8j`'s amendment (D21-43).
5. **Owned by `s8g`.** AC-133's registry is built there, and `s8g` carries `s5`'s dated correction. This stage only
   enters its enums.
6. **The register contradicts itself on the 226 h figure.** The scope note, *"the longest was answered at 226 h"*,
   sits against AC-039's *"belongs to stage→verdict … not to gate→verdict"*. The pack's `gate_deadline_secs` warrant
   quotes the first. Record B states both.
7. **Taken by D21-43 N17.** Q1–Q4 are no longer free residuals, and nothing is superseded by Q1.
8. **Brief facts that are false or imprecise, kept corrected:**
   - The margin ticks carry no `NO RECORD` marker.
   - `model_belongs_to_tier` is a `-`-segment match, not a prefix match.
   - `story`'s appetite is `points = 60`, not 40 [re-measured at `a6349b6`].
   - The packet's *"one paid dispatch each"* for the two probes cannot verify three model ids with one dispatch.
     N17 took Q4 as three.
9. **`DISABLE_AUTOUPDATER`, `DISABLE_UPDATES` and `--setting-sources` are undocumented on the docs site.** They are
   measured from the installed binary's strings. `blockReadsOutsideWorkingDirectories`, `--permission-prompts none`,
   the `!`-scope rule and the protected-path list are documented. D21-45 also quotes the protected-path list from
   the 2.1.281 code. Record A quotes each against a fresh fetch.
10. **`s8i`'s and `s8j`'s dispatch scans refuse Checkpoint 8h as written** [measured at `a6349b6`]. `s8i` (11)'s
    `DISPATCH_RE` matches both `--features live` and `--ignored`, and `s8j` (10) matches `--features live`, on any
    non-comment line. `s8i`'s revision, which is in progress, may already have narrowed its own scan. Checkpoint
    8h carries `--features live` twice:
    - (7), a `-- --list --ignored` listing that runs nothing;
    - (12), `cargo clippy` over the live-gated code.

    Neither dispatches. *Added 2026-09-24 (D21-47):* a third site is (10)'s `wrap` argv-screen control, which names
    `cargo test … --features live … -- --ignored` as a string that `--check wrap` must refuse. It runs nothing.
    Respelling any of them to dodge the pattern would satisfy a guard by its text, so they stay as they are. Each scan has to admit a `--list` line and a `cargo clippy` line, with a synthetic control. That is `s8i`'s
    and `s8j`'s work, and it is not resolved here.
    *Final consistency pass 2026-09-24:* `s8i` (11) now admits the `--list`, `clippy` and `--check` lines by shape, and
    (10)'s two sandboxed `wrap` runs and its argv-screen list by two counted, reasoned exemptions. `s8j` admits them
    through its exemption file. (10)'s first sandboxed run changed in this pass (it now pipes a stdin line), so an
    exemption keyed on its bytes moves with it.
11. **Checkpoint 8a (11) and Checkpoint 7f become provisional at this stage.** The first stops on *"a card carries
    permission_mode"*. The second's `redkiln run` refuses at the launch check. Its force-removing cleanup line is gone
    since `s8d-ii` §8d-ii.7, which also moved its run arm into a throwaway clone. §8h.14 gives the route, and running
    it is `s8j`'s.
12. **`StepError::Overage { reason }` already exists.** Its retry comment reads *"`Overage` is refused BEFORE spawn
    (AC-221, ADR-0010)"*, and it maps to `Exit::Overage`. N17 fixed only the exit. Before building a parallel
    kernel `EnvelopeError`, the implementer has to decide whether the envelope refusal is this variant with a
    `reason` naming the counter. If it is, Record B says so. If it is not, Record B says why not.
13. **A Claude Code session exports two names outside `CLAUDE_CODE_*`.** They are `CLAUDE_PID` and `CLAUDE_EFFORT`
    [measured by name]. 7c's guard and the committed denylist miss both, and D21-44's probe script stripped both.
    The launcher and (0b) now refuse them. Whether 7c's guard should too is `s8j`'s question, since `s8j` re-runs
    Checkpoint 7c.
14. **Minted gate kinds need rows before they can be reached.** `s8f`'s `landing` gate and N11's drift gate are
    covered here (§8h.10). Any gate kind a later stage mints, including `s8j`'s pack-declared human-root questions,
    brings its own `[[gate_expiry]]` row, or the load lint refuses the pack. *Updated 2026-09-24 for D21-46:* §8h.10
    now lists all six `s8` classes by row, and says why `s8j`'s three land at `s8j`.
15. **The drafted floor chain rests on sibling drafts that are being revised too** (§8h.0 item 8). Only Checkpoint
    8g's recorded count may replace `BASE_8G`.
16. **Two sibling files quote the `wrap` text this revision replaced.** `s8i` §8i.5 quotes *"Step 6 records and does
    not compare, because `TESTED_CLI_VERSION` binds the adapter, and neither caller is the adapter"* [read at
    `c5d19d7`]. `s8j` §8j.6 step 2 quotes *"`wrap` is the entry for a child that does not dispatch through
    `rk-adapter-claude`"*. Neither sentence is in this file any more. Both callers' substance still holds: `s8i` needs
    no pin comparison, and `s8j` needs 7c's own pin check. Each quotation is re-pointed at §8h.7's restated guarantee
    by its own stage's revision. The row-6 cell *"`wrap`: record only, and never compare to the pin (below)"* and the
    sentence *"`wrap` is not a way around the pin for `run` or `probe`"* are unchanged, so those quotations still
    resolve. *Settled in the consistency pass 2026-09-24:* both quotations are re-pointed at §8h.7's restated text.
17. **`s8j` §8j.3 step 5 and §8j.2's attach still spell a bare `redkiln answer`.** After this stage, a bare binary
    `answer` refuses without a launch record (§8h.7), so each goes through `tools/live-run.sh answer`. That edit is
    `s8j`'s. `s8j`'s dispatch screen `SCREEN='tools/live-run\.sh[[:space:]]+(run|probe)|…'` does not match the
    `answer` mode, and `answer` spawns no `claude`. Whether the screen should see it is `s8j`'s question. *Settled:*
    `s8j`'s revised screen matches the launcher's `answer` and `wrap` modes, and §8j.2 and §8j.3 route every answer
    through `tools/live-run.sh answer`.
18. **The launch check on `answer` reaches every binary-level `answer` test** (§8h.0 item 11). That includes the
    spawn helper in `rk-cli`'s `tests/it/answer.rs` and `s8e`'s `lane_e2e` binary arm. Each is given a fixture
    launch record in its shared helper, not an exemption. If a checkpoint between `s8e` and `s8g` spawns `answer` from
    a bash line rather than through a test, it refuses at its next whole run. No such line exists at `c5d19d7`
    [measured: a scan of every fenced `bash` block in `runbook/phase-1/*.md` for `redkiln`, `$RK` or `$BIN` followed
    by `answer` on a non-comment line finds none].
19. **`s8j`'s three gate classes have no fixed spelling anywhere yet.** `s8j` §8j.2 leaves *"the gate-kind name the
    step-4 gate binds under"* to its record, and §8j.3 leaves *"the gate cause"*. `GateReason` has one variant,
    `UnknownFacet`, at `c5d19d7` [measured: `crates/rk-kernel/src/select/step2.rs`]. §8h.10 lists them by what they
    are and leaves their rows to `s8j`, so that the misspelt-class lint cannot refuse a row drafted before its code.
20. **The sibling `rk_absent` shares `codehits`' old shape** (s8h review #4). `s7c` §7c.21's and `s7d`'s `rk_absent`
    run `grep -rnE "$1" $scope` with no `-H`, so a single-file scope prints no file name and the comment filter
    cannot drop a `//` line. There the failure direction is a false red, because it is an absence check, not a
    vacuous green. The `-H` fix is theirs to take, and this file edits neither. *Final consistency pass 2026-09-24:* the
    lead assigns the dated `-H` amendment to `s8j`, the stage that re-runs 7c and 7d.
21. **Settled by D21-48 X12** (*D21-48 (2026-09-24)*). It read *"Whether `--permission-mode default` is accepted on
    the 2.1.281 argv is unmeasured"*, and *"a decision is owed on whether `command.rs` emits `manual` for a card's
    `default`"*. The operator took it: *"The adapter emits the spelling the PINNED CLI's `--help` lists, fixed at s8j's
    pin move (`manual` on 2.1.281). Cards keep `default`, and the init comparison reads `default`. The golden argv
    snapshots move once, at the pin move. Probe 0 confirms rather than decides."* §8h.2 specifies the table, §8h.8
    step 3 the pin-move edit, and §8h.11 probe 0's confirmation and its HALT.
22. **`s8j` receives probe 0** (§8h.11). It is one haiku dispatch, run before probe 1 in its own block with its own
    go-ahead (D21-47's rule for real runs). Its retained frame fixes guard 3's channel (b), whose shape is drafted
    here from the binary's strings. `s8j` §8j.6's spend table gains the line.
23. **Record A supersedes ADR-0074 decision 1 in part** (cross #4). `s8b`'s 11c record supersedes nothing of ADR-0074,
    which stands, because it names the owner and does not add the field. (*Final consistency pass 2026-09-24:* corrected
    from *"supersedes nothing"*: in the branch where A1 passes and A2 or A3 fails, `s8b` §8b.6's record does supersede
    ADR-0080 d2 bullet 1 in part, which is not this field.) The field is added here, so the supersession is here.
    `A_SUPERSEDES` becomes `0126 0074`.
24. **D21-47's fake `claude` must be an executable on Windows.** The rule's words, *"a fake `claude` first on PATH
    that prints the pin"*, are met by a shell script for bash lines only. A Rust `Command::new("claude")` resolves
    `claude.exe` from PATH, so 7d's `doctor` probe would still reach the real 2.1.281 binary and fire
    `RK-D-CLI-VERSION` under Halt 2. Any file whose replay wrapper uses a script leaves 7d's `doctor` line red until
    `s8j`. (0c) compiles one with `rustc`, and the other files' wrappers are theirs. *Final consistency pass
    2026-09-24:* the compiled fake is now one shared helper, `runbook/tools/nodispatch.sh` with
    `runbook/tools/fake-claude.rs`, which (0c) sources and every `s8` file adopts.
25. **Launch-record names are unique to the nanosecond, and a collision refuses** (§8h.7 step 8). (10)'s
    non-check pair writes two records within one second, which the draft's `<UTC>` could not tell apart. The
    `-<mode>.txt` suffix is unchanged, so `s8i`'s and `s8j`'s `*-answer.txt` / `*-wrap.txt` readings still match.
26. **This stage now edits `s8a`'s Checkpoint 8a (11) at execution** (§8h.14), under D21-47's time-bound rule. The
    quoted bytes are `fc80687`'s, and `s8a`'s own fix round may move them. The amendment's intent governs: the card
    precondition stays inside the `RK_CP_NESTED` non-nested branch (already there at the final pass), and it asserts
    that the cards carry a mode.
27. **Five earlier checkpoints carry a card-mode precondition that this stage's cards falsify** (final consistency
    pass 2026-09-24). They are 8a (11), 8c (15), 8d (12a), 8d-ii's non-nested precondition and 8e (10d). This stage
    amends the first three in its work commit (§8h.14), because their replays reach 7f only through blocks that now
    run nested, or not at all. 8d-ii and 8e re-run Checkpoint 7f directly, and 7f's run arm refuses at the launch
    check until `s8j`'s A10 re-points it at the fixture child. Their stop is right until then, and A10's commit amends
    them under the same time-bound rule. That is `s8j`'s edit.
28. **`s8j` §8j.8 A11 drafts the 8a (11) re-scope as `s8j`'s own amendment**, citing a hand-off from this file,
    *"`s8j`, with that guard re-scoped"*, that is no longer in it. D21-47's time-bound rule puts the amendment here,
    in the commit that gives the cards their modes. §8h.14 takes A11's launch-check reading into it, so it is applied
    once. A11 should become a citation of this amendment, and `s8j` re-runs 8a whole. That is `s8j`'s edit.

---
