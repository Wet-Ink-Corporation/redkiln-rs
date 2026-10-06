# W1 audit: `crates/rk-cli` and `crates/redkiln-migrate` (snapshot 36dd8ac, branch build/phase-0-to-1)

Auditor scope: the `redkiln` binary (verbs `run`, `answer`, `status`, `explain`, `cost`, `doctor` / `doctor --rebuild`,
`catalog search`, `init`, `lint`; `release` is NOT a verb, see section 1) and `redkiln-migrate` v0. Read-only, from the pinned worktree
only. No cargo run by me. Where I quote a test/build result I am quoting the pre-existing background build logs
(`scratchpad/audit/log-*.txt`), not a run of my own; they are marked "log".

Legend: RECORDS = what runbook / PROGRESS / ADRs claim. CODE = what the source shows.

---

## 0. Verdict (plain English)

All nine `redkiln` verbs and the `redkiln-migrate` v0 binary exist as real code with real bodies (no stub, no
`todo!`/`unimplemented!`), backed by roughly 330 integration tests plus 10 `--help` snapshot files for `rk-cli` and 25 for the migrator.
An operator can drive the *mechanics* of a lane through the CLI (`init` -> hand-written item TOML -> `run` -> gate opens, exit 0 ->
`answer` -> `run` again -> `status`/`explain`/`cost`/`doctor`), but that has only ever been proven against a `FakeHarness` or the
fixture child process; **no real `claude -p` lane has been driven end to end through `redkiln run`** (the two self-hosting item files exist at
`.redkiln/items/`, the real runs are owed to 8j/Step 8, RECORDS: PROGRESS row 8). A real run is also not a bare `redkiln run`: it is refused
unless entered through the POSIX launcher `tools/live-run.sh` from a plain Git Bash terminal (`verbs/mod.rs:225`). Engineering quality is
disciplined (typed exit table with no wildcard, `#[expect]` with reasons rather than `#[allow]`, `Result`-returning verbs, a
poisoned-lock halt path) but carries notable debt: a 6,481-line `verbs/run.rs`, many doc-comment headers that now describe code that no
longer exists, a signal path that is wired in tests but **not connected in the shipped binary** (section 4, finding F1), `status` computing one global stage for all items
(F2), and a test suite that is not hermetic against ambient `ANTHROPIC_*` env (61 of 329 `rk-cli` tests red in the audit shell, F3).
The migrator is small, tight and fully green (25/25 in the log) but proven only on a 24-file hand-authored fixture; the real-corpus differential is Phase 2 by design.

Grade: rk-cli **B-/C+ (reported as B)**, redkiln-migrate **B+**. Combined completeness against Phase-1's requirement for this area ~80%.

---

## 1. Module / public-surface map

### rk-cli (src 17,917 lines incl. 5,667 comment lines; tests/it 35,999 lines; 329 `#[test]` run in the log)
Library `rk_cli` (everything tests link) + `[[bin]] redkiln` (`src/main.rs`, 68 lines, the only `std::process::exit` sites: `main.rs:52`, `main.rs:68`) + two test-only fixture bins (`rk-fixture-child-cli`, `rk-fixture-toolchain`, `Cargo.toml`).

| module | role | lines |
| --- | --- | --- |
| `cli.rs` | clap tree, 11-entry resume-name DENYLIST (`cli.rs`), `Command` = Run, Answer, Status, Explain, Cost, Doctor, Init, Catalog, Lint | 290 |
| `exit.rs` | closed `Exit` enum (18 values, 0/2/3/4/5/10-20/70/130), `From<&StepError>` exhaustive with no wildcard, `Halt` | 182 |
| `verbs/mod.rs` | `dispatch`, the `Startup` seam (reconciler, config reader, adapter resolver, harness factory), `launch_check` | 328 |
| `verbs/run.rs` | the lane loop, steps (1)-(10), toolchain selection, store route, checklist bridge, verify/review/landing orchestration, drift gates, `bind_nodes` | **6,481** |
| `verbs/answer.rs` | token resolution, option validation, the single `gate::answer` call site, attachments | 922 |
| `verbs/explain.rs` / `cost.rs` / `status.rs` | read verbs over the recorded run / store | 844 / 895 / 229 |
| `verbs/doctor.rs`, `doctor/{mod,rules,rebuild,probe,roles}.rs` | rule table (`RULES`, **43 ids**, 43 fixture dirs under `tests/fixtures/doctor/RK-D-*`), rebuild, version probe | 255 + 3,891 |
| `verbs/init.rs`, `catalog.rs`, `lint.rs` | scaffold, keyword search, facet lints | 256 / ~120 / 207 |
| `review_pass.rs`, `verify_pass.rs`, `release_tag.rs`, `change_deriver.rs`, `processes.rs` | later-stage composition-root passes (review, verify, `release_tag` git route, `change` facet deriver, question loader) | 412 / 773 / 175 / 238 / 324 |
| `adapters.rs`, `config.rs`, `env.rs`, `clock.rs`, `repo.rs`, `store.rs`, `signal.rs`, `no_dispatch.rs` | root plumbing: the four reals, closed `ConfigFile`, the one env read, the one clock read, repo discovery, the one lock acquisition, ctrlc handler, panicking harness for `doctor` | 280 / 226 / ... |

Public surface a reader/test sees: `rk_cli::{code_for, render_halt, is_lock_halt}` plus every module `pub`.

**`release` is not a CLI verb.** The brief lists "release"; in the code it is the `release_tag` git-route handler in the composition root
(`release_tag.rs:1-30`) invoked by the `tag-release` activity (ADR-0195/R5), not a `redkiln release` subcommand (not in `cli.rs` `Command`). `doctor --probe`
and `doctor --fix` (design section 8.1 rows) are also absent (RECORDS marks them "[inferred]"/CONDITIONAL, not Phase 1).

### redkiln-migrate (src 1,605 lines; tests 2,404; 25 tests)
`src/main.rs` (90): `redkiln-migrate --target <path> [--dry-run]` and `--version` only; exits 0/1/2/3 (RT-53, documented, not snapshot-pinned).
`lib.rs` -> `legacy/{config_yaml,fence,frontmatter,ledger,review}` (the only `yaml_serde` users), `lift.rs` (406), `report.rs` (185), `run_contract.rs` (360), schema `schema/report.schema.json`.
Dependency closure deliberately excludes `rk-ports`/`rk-kernel`/`rk-adapter-*` (`Cargo.toml`; asserted by `ac466_ac441_migrate_depends_on_no_harness_adapter_or_ports`).

---

## 2. What Phase 1 requires of this area (RECORDS)

- Design section 8.1 verb table: `run <item> [--lane fix|story]` (one idempotent entry, no resume flag), `answer` (the only gate channel), `status` (read-only, no reconcile), `explain` (from the record only), `doctor`/`doctor --rebuild`, `catalog search`, `init` (idempotent), `redkiln-migrate` separate binary (design `references/2026-09-08-architecture-design.md` section 8.1, 8.2-8.4). `cost` was added by ADR-0163 (9th verb); `lint` by D18-15.
- s7d (`runbook/phase-1/s7d-cli-and-migrate.md`, 3,964 lines) and its Checkpoint 7d; "migrate v0" is defined in 7d.11 as: binary exists, structural assertions (AC-441/466/474), report schema (AC-448/468), run contract (AC-447/450/522), dry run over a small fixture. Corpus-bound differential (AC-463) is Phase 2.
- Exit criterion (`s8-exit-criterion.md`): "Redkiln-RS runs its own fix lane end to end, **and** one story-lane run authors a Phase-2 story" (criterion 1), plus replay/kill-resume (2,3). 8j is "fixture proof of both lanes, CI green, and the run preparation".
- RECORDS status: PROGRESS row 7d "re-run WHOLE and passed at `d24d15a`" (2026-10-02); Step 8 "RE-OPENED ... Checkpoint 8 has NOT passed"; 8i IN PROGRESS; 8j PLANNED, not started.

---

## 3. What is implemented vs fake-only / ARMED / deferred / missing

### Verb-by-verb (CODE, with test evidence)

| verb | state | evidence |
| --- | --- | --- |
| `init` | **complete** (writes `config.toml` only if absent, opens/identity-stamps store, appends info/exclude block once) | `verbs/init.rs`; `tests/it/init.rs` 7 tests; `tests/cmd/init.trycmd` |
| `catalog search` | **complete** (keywords only, absent catalog = empty exit 0) | `verbs/catalog.rs`; one example-based test `tests/it/catalog.rs:45` (RECORDS D18-15 said "property test"; it is not one, and `proptest` in `[dev-dependencies]` is unused, grep 0 hits) |
| `lint --facet-stability/--facet-provenance` | **complete as a thin wrapper** over kernel rules | `verbs/lint.rs`; 2 tests |
| `doctor` | **complete**: 43 rules, each with a rejecting fixture, zero-dispatch by construction (rules receive `Ctx`, no harness) | `doctor/rules.rs:75` `RULES`; 47 tests in `tests/it/doctor.rs` + journal/drift/artifact siblings |
| `doctor --rebuild` | **complete**, re-opens store identity after rebuild, prints `unresolvable-from-git-alone` | `verbs/mod.rs` `doctor_with_rebuild_reopen`; `doctor/rebuild.rs` (423); differential test in `lane_e2e`/`doctor` |
| `status` | **partial / thin**: prints `item<TAB>transition<TAB>gates=N` plus cost-draw lines; stage is global (F2) | `verbs/status.rs:183-229`; only 2 tests, both about headroom lines (`tests/it/status.rs`) |
| `explain` | **complete** (renders `RecordedRun` only, scans assert no `rk_kernel::select` path) | `verbs/explain.rs`; 8 tests + 3 insta snapshots |
| `cost` | **complete** (ADR-0110 batch delta or the reason none is quoted) | `verbs/cost.rs`; 11 tests + 24 fixture dirs under `tests/fixtures/cost/` |
| `answer` | **complete** (token resolved from open gate, option validated against registry, attachments admitted/refused, void-after-commit) | `verbs/answer.rs`; 12 tests (9 red in audit shell only for env reason, F3) |
| `run` | **complete as code, never proven on a real model**. Steps 1-10, run-owned worktree, carry/setup, stage advance, gates, expiry pass, verify pass, review pass, landing, close/removal, rate-limit mark. | `verbs/run.rs`; 15+20+35+26+... tests across `run`, `lane_e2e`, `landing`, `verify_pass`, `review_pass`; all lane tests use `FakeHarness::scripted` except a 4-case process-boundary set over `rk-fixture-child-claude` (`lane_e2e.rs:1-30`) |

### Fake-only / ARMED / deferred (RECORDS and CODE agree unless stated)
- **No real-`claude` lane run.** `ClaudeHarnessFactory::builds_the_real_harness()` is true only in the shipped path (`verbs/mod.rs:88-110`); no test builds the real `ClaudeHarness` over a real `claude` for a lane. `lane_proof` (s8j's fixture proof module) does not exist in `tests/it/` (file list). RECORDS agree (8j "Not started").
- **Operator entry is gated.** `run` and `answer` over the real harness refuse without `REDKILN_LAUNCH_RECORD` (`verbs/mod.rs:225-246`, `env.rs:26`) and the record is written only by `tools/live-run.sh` (370-line POSIX sh, refuses Claude-session env and Windows non-Git-Bash). Intentional (s8h 8h.7) but it means "drive a lane through the CLI" = "through a shell script wrapper".
- **OS signal delivery: ARMED, never run** (RECORDS s7d 7d.10/7d.18; `signal.rs:24`). In addition, see F1: in the shipped binary the handler's token is not connected to the run.
- AC-366 CI rebuild job, AC-398 drift report, AC-278: ARMED per s7d 7d.12/7d.18 (RECORDS; not re-verified).
- Items are hand-authored TOML under `.redkiln/items/` (`ItemRecord`, `verbs/run.rs:149`); there is no `redkiln item add/create` verb. Two real self-hosting items exist (`.redkiln/items/rk-fix-doctor-test-count.toml`, `rk-story-codex-adapter.toml`).
- Composite two-toolchain adapter not built (s7d table row 245; RECORDS) - the Rust adapter fills `Ports.toolchain`.

### Migrate v0
- Implements exactly 7d.11's definition: dirty-target refusal (AC-522), re-created never reused worktree, one commit + report in the same commit (AC-447/448), no-op second run (AC-450), idempotence across kill, byte-identical output across TZ and read order (AC-449), six typed kinds + eight opaque (ADR-0075), loss markers, no `ChangeSummary` values (AC-475), no dual-write flag/env key (AC-474). 25/25 pass in the log (`log-test.txt`, `running 25 tests ... ok. 25 passed`).
- `ac463_corpus_differential_runner_named` exists only as `#[cfg(feature="corpus")] #[ignore]` (`tests/it/lift.rs:249-254`) - corpus differential is Phase 2 (RECORDS). The fixture `tests/fixtures/legacy-mini` is 24 hand-authored files (`PROVENANCE.md`), so fidelity to real old-Redkiln corpora is **unproven** by any test here.

---

## 4. Findings (CODE-verified unless marked)

**F1 (medium, correctness-in-shipped-binary): the Ctrl-C/SIGTERM token is orphaned.**
`main.rs:34-37` does `Ok(token) => drop(token)`; `verbs::dispatch(&cli, &env)` takes no token (`main.rs:64`, `verbs/mod.rs` `dispatch`); `run` builds its own `CancelToken::new()` (`verbs/run.rs:398`) and hands that to the harness and the step-boundary checks (`run.rs:1307-1341, 2735-2763`). `signal.rs:44-50` installs a `ctrlc` handler (which replaces the default Ctrl-C behaviour) that cancels a token no reader holds. Consequence by reading: a real Ctrl-C never produces `Cancelled{by:Signal}` (exit 13) or `SignalOutsideDispatch` (130); the in-process tests (`root.rs:132`, `root.rs:1340ff`) create their own token and fire it, so they cannot see the gap. `main.rs`'s own comment says the readers "are Slice 3's"; Slice 3 never connected them. Not executed by me (no cargo, no signal delivery), so confidence is high on the plumbing, unverified on runtime effect. The kill/resume exit criterion uses `taskkill /T /F` (design 8.1), so Phase-1's gate does not depend on this path, but an operator pressing Ctrl-C on a live run is affected.

**F2 (medium, probable functional defect): `status` conflates stages across items.**
`verbs/status.rs:196-197`: `roll_up` takes `recorded_stage(events)` (the last `StageTransition` in the *whole* journal, `status.rs:183`) and passes it to `fsm::stage_action(&item, &stage, ...)` for **every** item. `StageTransition` payloads carry only `from`/`to` with no `item` (`rk-kernel/src/runtime.rs:821-826`, `runtime/tier_one.rs:2230-2262`), so `item_of` (`status.rs`) files them under the shared set that is merged into every item's slice. With the two Phase-1 lane items (fix + story) in one store, `status` would fold both under whichever stage transitioned last. The kernel's own `two_items_in_one_store_do_not_fold_together` (`lane_e2e.rs:1297`) covers `run`, not `status`. Also: `status` output is terse (`transition`, `gates=N`; no stage name, no "awaiting gate/closed" words) and swallows write errors (`writeln!(...).unwrap_or(())`, `status.rs:145,159`). Unverified by execution.

**F3 (medium, test hygiene): the suite is not hermetic against ambient provider env.**
In the pre-existing audit test log, `it` ran 329 tests: 266 passed, **63 failed**; 61 of the 63 stderr dumps carry "the environment carries `ANTHROPIC_BASE_URL`, which is refused" (the product's own denylist, `adapters.rs`/`env.rs`, working as designed against the audit shell's inherited Claude-Code env); the other two (`checklist_bridge` rust-smoke detection and `checklist_bridge:190` DenylistedEnv) I could not individually classify and list as unverified. Spawned-binary tests scrub only `REDKILN_LAUNCH_RECORD` (`tests/it/main.rs:288`); none scrub `ANTHROPIC_*`/`CLAUDE_CODE_*`. s7d records that the *checkpoint block* refuses such a shell, but `cargo test --workspace` does not, so any developer or CI box with `ANTHROPIC_BASE_URL`/`ANTHROPIC_FOUNDRY_API_KEY` set (RECORDS: the latter is persisted on the operator's box, s7c 7c.21) gets a large false-red. Treat the 63 as environmental, not product regressions; the RECORDS claim (Checkpoint 7d passed whole at d24d15a) is not contradicted, only not reproduced by me.

**F4 (medium, maintainability): `verbs/run.rs` is a 6,481-line god module** holding the lane loop (`run_lane` ~550 lines, `verbs/run.rs:598-1144`), `through_step_seven` (~390), `resume_and_read` (~330), `verify_step_six` (~340), the checklist bridge, toolchain choice, store route and pack-extras parsing. Six `#[expect(clippy::too_many_arguments)]` and two `too_many_lines` waivers are the symptom (`run.rs:588,593,1227,1284,1365,2716,3595,4575`; each carries a reason). The newer passes (`review_pass.rs`, `verify_pass.rs`, `release_tag.rs`) were split out, which shows the direction; the remaining mass has not been.

**F5 (medium, documentation integrity): doc headers contradict the code they sit on.** Examples: `lib.rs:1-33` says "What is here is Slices 1 and 2 ... `doctor` is a SKELETON ... `RULES` is empty" (RULES has 43 rows, `rules.rs:75`); `doctor/mod.rs:1-45` ("WHAT IS BUILT HERE IS THE SKELETON, AND THE SKELETON CLAIMS NO TEST"); `verbs/answer.rs:1-30` ("THIS FILE IS A SKELETON ... `run_with` therefore refuses without doing anything", while the file is 922 lines with a full verb); `verbs/run.rs:296-314` ("STEP (8)'S ADAPTER CONSTRUCTION IS CYCLE 6'S, AND THE STUB BELOW IS THE DECISION ... does **not** build the real `Harness`", while the function below builds the real one via `startup.harness`); `tests/it/main.rs:1-60` still lists "Slices 1 to 3". Comments are 31.6% of `rk-cli/src` lines (5,667/17,917; `run.rs` 28%, 136 "dated 20.." annotations in `run.rs` alone) and carry struck-through history (`~~...~~`), so a reader cannot tell live from superseded text. RECORDS own the corrective rule ("a module is declared when its file exists") but the headers were not swept after slices landed.

**F6 (low): dead/vestigial seams.** `let _ = harness;` (`cost.rs:175`), `let _ = &registry;` (`explain.rs:398`), `let _ = ctx;` (`explain.rs:660`), unused `Ports` constructed and discarded in `verbs/doctor.rs` `evaluate_and_render` (`let _ports = ...`), `status::run_with` takes reconciler/exec/worktree and ignores them (deliberate, to keep AC-402 falsifiable; documented). `anyhow` is declared in `rk-cli/Cargo.toml` but grep finds no use in `src/` or `tests/`. `proptest` dev-dep unused. One clippy "unfulfilled lint expectation" (`verbs/explain.rs:101`, log) and two never-used test helpers (`support.rs:162,347`, log).

**F7 (low): swallowed errors on best-effort paths.** `let _ = store.append(event)` in `refuse_after_reconcile` (`run.rs:2181`; documented: the operator's real refusal is not replaced by a store error), `let _ = writeln!(...)` in `doctor.rs`/`init.rs`, `.unwrap_or_default()` in `migrate/src/lift.rs:301,305` on `serde_json::to_value` (a failed serialisation would silently become `null` before the schema check; low risk, the schema validation that follows would catch it).

### Escape-hatch census, qualified (NON-TEST code; census file said rk-cli: unwrap/expect 4, panics 11, allow/expect-attr 16, unsafe 1; migrate: unwrap/expect 1)
Re-grepped `src/` for non-comment hits:
- `unsafe`: **0** real hits (the census "1" is the word "unsafe item id" in a doc comment, `run.rs:4801`). `unsafe_code = "deny"` workspace-wide.
- `.unwrap()`: **0**. `.expect(`: **2**: `rk-cli/src/config.rs:201` (`serde_json::to_value(&every).expect("ConfigFile serialises")`, in the config-surface walker, which only builds a key list for the AC-474 snapshot) and `redkiln-migrate/src/run_contract.rs:210` (`worktree.expect("a non-dry run opened its worktree")`, guarded by control flow just above; a typestate would remove it).
- `panic!`: **3 real sites**, all deliberate and documented: `no_dispatch.rs:78` (doctor harness; AC-414 instrument), `verbs/cost.rs:148` (AC-395 instrument), `store.rs:78` (poisoned lock re-raised with `LOCK_HALTS` payload so `main.rs`'s hook maps it to exit 70; ADR-0118). Rest of census "11" are doc/test lines.
- `todo!/unimplemented!/FIXME/TODO`: **0**.
- `#[expect(...)]` (never `#[allow]`): 16 real, all with `reason = "..."`: 1 `disallowed_methods` (the one wall-clock read, `clock.rs:25`), 1 `disallowed_types` (the doctor probe `Command`, `doctor/probe.rs:82`), 9 `too_many_arguments`, 3 `too_many_lines`/similar. **No `#[allow]` anywhere.** `std::process::exit`: exactly 2 in rk-cli (`main.rs:52,68`) and 3 in migrate's `main.rs` (49,53,57), as the stage says.
- Clippy (log, rc=0; workspace policy is `all=deny` + enumerated pedantic + `nursery=warn`, not `-D warnings`, by ADR-0102): rk-cli lib 29 and tests 55 warnings, nearly all nursery (`pub(crate)` in private module 32, redundant clone 14, `first doc comment paragraph too long` 25, `const fn` 4), 0 errors; redkiln-migrate: none attributable in the log extract.

Net: escape-hatch discipline is genuinely good, better than most Rust codebases of this size.

---

## 5. Code-quality judgement

- **Structure / seams (good):** composition root with every effect behind a trait seam (`EntryReconciler`, `ConfigReader`, `AdapterResolver`, `HarnessFactory`, `VersionProbe`); one env read, one clock read, one lock acquisition function; library/bin split so tests link the same surface.
- **Error design (good):** closed `Exit` table with exhaustive `From<&StepError>` (`exit.rs`), typed `ConfigError/Refusal/RepoError/RebuildError/ProbeError/ProcessesError`, `Halt` as a value for the poisoned lock, verbs return `Result<Exit, Halt>` so they are testable in process. Some inner seams return `Result<_, String>` (`HarnessFactory::build`), 36 occurrences of string-mapped errors, acceptable at the edge.
- **Test depth (strong but skewed):** ~2:1 test:source LOC. Integration-only by ADR-0083 (0 unit tests in `src/`); 10 trycmd `--help` snapshots, 5 insta snapshots in `tests/it/snapshots/` (exit table, config surface, three explain), 24+ cost golden dirs, 43 doctor fixture dirs. Tests are adversarial ("PREMISE:/CONTROL:" assertions, synthetic positive controls, source-scan lints). Gaps: `status` 2 tests, `lint` 2, `catalog` 1 (not the promised property test), no test of the main-to-run signal plumbing, no multi-item `status` test, zero real-`claude` lane test. 0 `#[ignore]` except 2 in `landing.rs`; feature `corpus` empty for rk-cli.
- **Dead code / noise:** see F5-F7; comment-to-code ratio is very high and much of it is process narrative that belongs in the runbook.

---

## 6. Risks

1. First live lane may expose what only fakes have exercised: the real `ClaudeHarness` + `run` loop + launcher has never run together (RECORDS PROGRESS row 8; no `lane_proof`).
2. F1 (orphan signal token) - Ctrl-C handler installed but inert; an operator interrupt behaves unpredictably (child may die from the shared console event, run records a failure instead of `Cancelled`).
3. F2 - `status` is the operator's main observability verb and is likely wrong with >1 item in a store, exactly the exit-criterion shape (fix + story in one repo).
4. F3 - false-red suite on any box with provider env set; erodes trust in "green"; the s7d block's refusal guards the checkpoint only.
5. F4/F5 - maintenance cost: a 6.5k-line module and stale headers make it easy for the next stage to build on a false description (this repo already logged such drift repeatedly in its own corrections).
6. Windows/Git-Bash coupling of the live path (launcher refuses other shells; CI POSIX arms of signal/AC-224 are ARMED, not run).
7. Migrate trusted on a hand-authored 24-file fixture; real-corpus fidelity is deferred to Phase 2 and `ac463_` is `#[ignore]`.

---

## 7. Unverified (not shown by anything I read or ran)

- That `status` really prints one global stage with two items in one store (code read only; not executed).
- That a real Ctrl-C is inert in the shipped binary (code read only).
- The cause of the two non-`ANTHROPIC_BASE_URL` failures among the 63 (`checklist_bridge::the_real_rust_smoke_gets_past_input_parsing_through_real_toolchain` asserts a toolchain detects the Cargo fixture and got `Some(false)`; likely a missing-toolchain/env effect on the audit box).
- Whether `rk-cli` tests are green in a clean plain terminal (RECORDS claim yes at `d24d15a`; this snapshot is `36dd8ac`, and the log is from the audit shell).
- Quality of `doctor`'s 43 rules beyond fixture-per-rule (AC-415 join is claimed by RECORDS; I counted 43 ids = 43 fixture dirs, did not run the join).
- Wall-clock cost/time of `doctor` (SM-122 target <= 60 s) - not measured.
- RECORDS claims of ARMED rows (AC-366, AC-398, AC-278, POSIX signal arms) - not re-verified.
- Rest of `redkiln-migrate` correctness beyond its tests and a skim of `lift.rs`/`main.rs`/`run_contract.rs`.

## 8. Key citations

- Verb dispatch: `crates/rk-cli/src/verbs/mod.rs` (`dispatch`), launch gate `verbs/mod.rs:225`, `tools/live-run.sh:1-30`.
- Exit table: `crates/rk-cli/src/exit.rs`; snapshot `tests/it/snapshots/it__exit__ac438_exit_table_snapshot.snap`.
- Signal gap: `main.rs:34-37`, `signal.rs:44-50`, `verbs/run.rs:398,1307-1341,2735-2763`, `tests/it/root.rs:132`.
- Status: `verbs/status.rs:145,159,183-229`; StageTransition payload: `rk-kernel/src/runtime.rs:821-826`.
- Doctor: `doctor/rules.rs:75`, fixtures `tests/fixtures/doctor/RK-D-*` (43).
- Stale headers: `lib.rs:1-33`, `doctor/mod.rs:1-45`, `verbs/answer.rs:1-30`, `verbs/run.rs:296-314`.
- Migrate: `crates/redkiln-migrate/src/{main,lift,run_contract,report}.rs`, `tests/it/lift.rs:249-254`, test log (25 passed).
- RECORDS: `runbook/PROGRESS.md` rows 7d, 8, 8i, 8j; `runbook/phase-1/s7d-cli-and-migrate.md` 7d.10, 7d.11, 7d.18; `references/2026-09-08-architecture-design.md` 8.1-8.4.
