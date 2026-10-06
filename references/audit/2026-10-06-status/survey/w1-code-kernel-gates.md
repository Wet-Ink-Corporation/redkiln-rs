# W1 audit: `crates/rk-kernel`, the three-gates side

Snapshot: worktree `audit-wt` at commit `36dd8ac` (build/phase-0-to-1, 2026-10-05). Read-only; no cargo run by this auditor.
All paths are relative to the worktree unless marked otherwise. "RECORDS" means runbook, ADR or decision files. "CODE" means source and tests.

## 0. Verdict

**Grade: B (solid, with real wiring gaps that the records mostly admit).** About 72% complete against what Phase 1 requires of this area.

- **What is built.** The kernel's gate logic is well built and thoroughly tested. It has the human gate, the review gate (classifier, policy table, confirm, quarantine, advisory sink, pack rules) and the verify gate (flip predicate, AC ledger, stage-exit refusal, comparator, `verify_completed` record, AC gate).
  - It is written as pure functions with closed enums and exhaustive matches.
  - Production code in the gate modules has zero `unwrap`, `panic` or lint-suppression sites, apart from the one justified `#[expect]` and one lock `expect` noted in §5.
  - The test suite pins each criterion by name, with must-fail controls.
- **Not at the guarantee level the design promises.** Four things are the gaps:
  1. **The real verify harness cannot produce `pinned`.** The harness hard-codes `assertion_flipped: None`, and the CLI passes an empty assertion map to the ledger. The per-AC mutation guarantee therefore reduces, on the production path, to "every AC needs a human `discriminates` answer, or the stage exit is refused". That is fail-closed but is not the designed per-AC pin. The RECORDS say so (AC-334 ARMED, "pinned is unreachable").
  2. **The real re-executor only reproduces `kind = run` findings.** `kind = mutate` returns no record and so can only be advisory.
  3. **Quarantine grant/remove, advisory retirement and several pack and ingest rules have no production caller.** They exist and are tested but are not wired into `redkiln run`.
  4. **The review gate has never been calibrated against false approval.** S-SUBTLE-CB has not run (ADR-0101 clause 5). The review gate wiring runs over fixtures and fakes, and over the S-BIN probe session that is still in progress.
- **Process and structure.** The documentation-to-code ratio is very high. The lint and boundary machinery ships inside the production library crate.

## 1. Module map and public surface

Layout is `src/review.rs` plus `src/review/*` (10 files, 1,222 lines), `src/verify.rs` plus `src/verify/*` (10 files, 1,372 lines), the human gate in `src/workflow/gate.rs` (664 lines) and `src/workflow/fsm.rs` (`GateRecord` at :315, `RunState`), and `src/workflow/question.rs` (311). The kernel-side call sites are in `src/runtime/step.rs`, `src/runtime/advisory.rs` and `src/replay.rs`.

**Human gate**
- `GateRecord` has private fields. The token is `canonical_digest({deadline, kind, reach, tree_hash})` (`fsm.rs:348-359`), so it is a content hash with no clock or RNG.
- `workflow::gate` exposes `reach` (:134), `answer` (:229), `expire` (:475), `drifted`, the two replay checkers, `lint_expiry_policy_row`, and the row constructors `gate_answered_event_with_meta`, `gate_answer_void_event` and `gate_answer_spent_event`.
- `answer` orders its checks as: wrong gate; spent or void (refused as spent); attachment and option validation via the question registry; applied or void by tree-hash comparison; expired; never opened. A mismatching tree hash writes `gate_answered` plus `gate_answer_void` and nothing reconciles it (ADR-0017).

**Review gate**
- `classify::classify(&Repro, &Observation) -> EvidenceClass` (`classify.rs:102`) is one total match over (repro kind × observation) with no wildcard to a blocking class. The class enum is closed at four members.
- `policy::classify(class, weight, Option<&ReExecution>) -> Decision` (`policy.rs:145`) is the only place a class becomes a block. `Block` has private fields and no constructor, and the single construction is `policy.rs:173`. `resolve_review_tier(Option<Rigor>)` takes only the authored rigor (AC-302). A model-supplied tier cannot reach it.
- `reexec` has the four-cell table (`reexec.rs:60`), `mutation_outcome` and the `ReExecutor` port trait.
- `confirm::confirm_with` is the pure pass. It returns records plus rows (`review_block`, `advisory_created`, `review_class_drift`, `quarantine_expired`). `retire_not_re_raised` emits `advisory_retired`.
- The remaining review modules are `ingest`, `quarantine`, `record`, `render`, `regression` and `pack_rules`.

**Verify gate**
- `flip::flip_predicate` is a pure function over a recorded `MutationRun`.
- `ledger::compute_ledger` and `ledger::ac_status` take the criterion, flip outcome, assertion locator, feasibility and human answer. Their signatures contain no suite verdict, finding or quarantine (AC-335, AC-342, AC-345).
- `exit::stage_exit` returns `ExitRefused`, which carries the one `stage_exit_refused` row (ADR-0125).
- `comparator::reclassify` covers AC-326, AC-322 and AC-332. `runner::run_test` is the single `Toolchain::test` call site and journals `verify_completed`. `ac_gate::{open, answers}` mints the `verify.ac-discriminates` gate and reads its answers. `claims` is the AC-349 claim binder. `eligible` is the gate-eligible list as data. `admissibility` is AC-455.
- `verify.rs` also holds `report_at_boundary` and `verify_activity_has_no_agent_step` (AC-347).

**Lint and boundary machinery shipped as `pub mod` in the lib** (`lib.rs:308-403`) includes `module_boundary` (1,164 lines of source-text scanners), `kernel_denylist`, `no_egress` and `adapter_edge`.

## 2. What Phase 1 requires, and what the RECORDS claim

RECORDS: `runbook/phase-1/s7b-ii-gates.md` claims 122 ids. The register's `human-gate` (AC-240..279 plus 524), `review-gate` (AC-280..319 plus 530) and `verify-gate` (AC-320..359) areas have 40 in force each, plus AC-455 and AC-239's list-is-data half. The stage records the following:
- **Discharged by a discriminating test (§7b-ii.14, :813 onward).**
  - Human gate: AC-244, 245, 246, 249, 250, 270, 272, plus 524 built but PROVISIONAL.
  - Review gate: AC-281-283, 285-290, 293-296, 298-307, 310, 313-315, 318, 319, 530, with AC-284 built and not claimed.
  - Verify gate: AC-326, 333, 335, 337 (fixture half), 345, 346 and others.
- **ARMED, with reasons.**
  - AC-317 (Step 8).
  - AC-334 (a real toolchain).
  - AC-295, AC-307 corpus half and AC-354 (a recorded corpus run, Step 8).
  - The doctor halves of AC-240, 241, 248, 251, 256, 260, 264, 268, 277 and others (7d).
  - AC-254, 257, 278 (7c/7d).
  - AC-262, 325, 328, 329 adapter half, 331, 350, 351 (7c).
  - AC-312 (S-SUBTLE-CB).
  - AC-266, 271, 308, 356 (PROVISIONAL on unrun spikes).
  - AC-301's lift (S-CASCADE unrun).
- **Excluded as human-gated and not machine-decidable:** AC-273 (human half), 275, 276, 309, 311, 357, 358. AC-255 and AC-297 are cut.
- **Raised and not settled.** AC-291 and AC-292 were orphaned (later routed to 7d under D18-19). The AC-239 double claim is still noted.

Milestone status: M3 is REACHED (Checkpoint 7b-ii passed `164e212`, 2026-09-17, recorded as "the three gates exist and fail closed" and nothing more) (`runbook/INDEX.md:98`, `runbook/PROGRESS.md:32`).
- The 7b-ii row was re-run whole and passed at `d24d15a` (2026-10-02, inside Checkpoint 8h (13)).
- `git log d24d15a..HEAD -- crates/rk-kernel crates/rk-types crates/rk-store crates/rk-ports` is empty. The kernel has not moved since that pass, so the 7b-ii/8g/8h passes still describe this tree. I did not re-run any checkpoint block.

Later stages that touch this area:
- **7h** (`runbook/phase-1/s7h-lane-completes.md:98-131`) wires the review/verify call site over a fixture corpus (ADR-0162). Its `Slice-Verdict:` axis was delivered by s8e (ADR-0186).
- **8g** (`runbook/phase-1/s8g-verify-and-restored-demands.md`) added the `verify_completed` record (`verify/runner.rs`), the mutation harness, `ac_gate`, `claims` and the criteria-based ledger. Checkpoint 8g passed at `4777941`, was re-run green at `d5bfc4f` and again at `d24d15a`, and is closed.

## 3. S-SUBTLE-CB and ADR-0101 clause 5

**S-SUBTLE-CB has not run.** Evidence:
- `spikes/DECISIONS-2026-09-21.md:235` states "S-SUBTLE-CB ... has not" run.
- `spikes/DECISIONS-2026-09-21.md:1461` states it is "UNRUN".
- `crates/rk-kernel/src/review.rs:29` and `crates/rk-cli/src/review_pass.rs:22` carry the same disclaimer.
- The only artefacts are two authored, undispatched fixture files in `crates/rk-adapter-claude/tests/fixtures/framing/` (README: "AUTHORED at Step 7c and UNRUN").

The kernel header discipline is sound. A Checkpoint 7b-ii guard greps the crate for the forbidden claim, and the guard fired on the header's own denial and the header was reworded (`runbook/PROGRESS.md:32`, history text). No kernel code or doc claims the review gate is guarded against false approval.

**Consequence for any status report:** the review gate may be described as existing, wired and fail-closed on re-execution flip. It may not be described as calibrated or protected against false approval. S-BIN (Step 8i, in progress) asks a different question, whether the reviewer can verify a non-text-diff item (`runbook/phase-1/s8i-s-bin.md:1,221`). S-BIN counts are not S-SUBTLE-CB evidence either (ADR-0101 clause 5 also bars treating S-VAR or S-TYPED false-positive rates as calibration).

## 4. Genuinely implemented and exercised (CODE)

1. **Test volume and breadth.** Gate-side integration tests (`tests/it/gates.rs`, `review.rs`, `review_confirm.rs`, `verify.rs`, `advisory.rs`, `ledger.rs`, `gate_step.rs`, `question.rs`) hold 16, 20, 12, 27, 2, 1, 7 and 3 tests. They total about 7,860 lines of tests against about 3,970 lines of gate source.
   - The audit's own default `cargo test --workspace` log (`scratchpad/audit/log-test.txt`, supplementary, not the repo tree) shows `rk-kernel` lib 12 passed and integration `it` 417 passed, 0 failed, 5 ignored. The log also shows gates 16 ok, review 19 ok (+1 child), review_confirm 12 ok, verify 27 ok, gate_step 7 ok, advisory 2 ok, question 3 ok and the trybuild harness ok.
   - Workspace build and clippy were rc=0 in that log. The workspace log has failures in an adapter crate (`rk-adapter-toolchain-node`, other auditors' scope). Those failures are not rk-kernel.
2. **Named test per criterion.** 87 distinct gate-range ids (AC-240..359, 524, 530) have a `fn ac<N>_…` somewhere in the workspace (census by grep). The ids with none are, by the records, the human-gated, doctor-only, spike-gated or adapter-gated ones.
3. **Compile-fail instruments.** The trybuild cases `tests/ui/ac244_gate_record_missing_a_field.rs` and `tests/ui/ac294_block_constructor.rs` each carry a committed `.stderr` snapshot. Other trybuild cases cover AC-007, AC-010 and AC-020, among others. They test the unconstructibility claims by actually failing to compile.
4. **Cross-process determinism.** `review::ac286_classifier_is_deterministic` re-executes the binary's own `#[ignore]`d child entry point in a fresh process and compares digests. The 5 `ignored` tests in `rk-kernel` are child entry points, not skipped tests (grep of `#[ignore` reasons).
5. **Source-text architecture rules with positive controls** (`src/module_boundary.rs`) include:
   - `policy_reads_no_model_label_rule` (AC-285), `classifier_purity_rule` (AC-286), `review_path_no_prose_json_rule` (AC-318), `review_tier_call_graph_rule` (AC-302), `block_construction_sites_rule` (AC-294).
   - `block_path_appends_its_own_event_rule` (AC-307), `void_consumer_rule` (AC-250), `gate_path_purity_rule` (AC-242), `stage_exit_refused_single_site_rule` (ADR-0125), `verify_reads_no_suite_verdict_rule` (AC-335/341/342).
   - Each has a fixture in `tests/fixtures/lint-controls/` or `tests/fixtures/gates/`.
6. **Design properties confirmed in code, not only in prose.**
   - `confirm_with` appends `review_block` from one match arm (`confirm.rs`, `Decision::Block` arm). Every non-block becomes `advisory_created` with a persisted reason.
   - `policy::classify` ordering is weight, then re-execution outcome (absent or unverifiable always advises), then class, then flip.
   - AC-288's both-fail cell is `Unverifiable::BothFail`, not a block (`reexec.rs:60-66`).
   - The model label is compared for drift in `confirm.rs`, outside `policy`, and never read by policy (`record.rs` header; lint).
   - Human gate: spent, void and wrong-gate refusals; reach counter in the token digest (AC-524); expiry at a journaled `now`.
7. **Real call sites on the run path (7h and 8g).**
   - `rk-cli/src/verbs/run.rs:4655-4700` resolves the tier from authored rigor, builds the re-executor, calls `review_pass::confirm`, appends rows and re-reads the journal before deciding the exit.
   - `runtime/step.rs:771-890` implements `verdict_record` via `review::confirm_with`.
   - `run.rs:4228-4235` computes the ledger and `stage_exit` and writes `stage_exit_refused`.
   - `verify/runner.rs:137` is the single port-call site that journals `verify_completed`. `verify_pass::mutate` runs a real ephemeral-worktree mutation (`rk-cli/src/verify_pass.rs:556-720`) with AC-338 post-suite region re-hash.
   - These are exercised in `rk-cli` tests (`crates/rk-cli/tests/it/review_pass.rs` exists). I did not read them.

## 5. Stubbed, fake-only, ARMED or not wired (CODE vs RECORDS)

| item | what CODE shows | RECORDS |
| --- | --- | --- |
| **`pinned` unreachable in production** | `verify_pass.rs:667,707` set `assertion_flipped: None` on every real mutation run. `run.rs:4228-4234` calls `compute_ledger(&criteria, &runs, &BTreeMap::new() /*assertions*/, …)`. `ledger.rs:83,88` require `assertion.is_some()` for `Pinned` and `MutationSurvived`. No production code builds `FlipOutcome::Pinned` (grep: only tests). Net: ledger statuses reachable in a real run are `unpinned`, `not_feasible`, `human_answered`. The designed pin is not reachable. | Admitted: `verify_pass.rs` header ("`pinned` is not reachable through this harness"), `s8g` §8g.10 close-out ("`pinned` is unreachable, AC-334 ARMED"), `s7b-ii` §7b-ii.14. AC-334 ARMED. |
| **Real review re-executor reproduces only `kind = run`** | `review_pass.rs:220` returns `None` for `Mutate` repros; `:233` returns `None` for empty runs; `only_assertion_flipped` is always false (`:266`), so `narrowed-pin` can never arise on the real path; `mutate` findings are advisory at run time. | Doc comments in `review_pass.rs` say a patch-applying port method does not exist (ADR-0162). AC-289 workflow-only fixture ARMED (S0.1-SCOPE unrun). |
| **Quarantine grant/remove not wired** | `quarantine::grant`/`remove` have zero callers outside tests (grep). `review_pass.rs:285-287` and `runtime/step.rs:842` hard-code an empty quarantine map ("nothing persists them yet"). No CLI verb mentions quarantine; only a `doctor` rule reads `quarantine_granted` rows (`doctor/rules.rs:1809`). | Built and claimed at kernel level (AC-304, 306, 307). No record claims run-path wiring. |
| **Advisory retirement not wired** | `retire_not_re_raised` (`confirm.rs:351`) has no non-test caller. The sink is therefore append-only in a real run (AC-530's retirement half runs only in tests). | AC-530 kernel half claimed. `doctor` half routed to 7d (D18-19). |
| **Unwired helpers** | No production caller for `RegressionRecord` (AC-314), `GateEligible` (AC-239), `fix_lane_review_is_strict` (AC-300), `cascade_is_refused` (AC-301), `verify_activity_has_no_agent_step` (AC-347 helper; AC-347 is enforced by a shipped-catalog test), `review::ingest::findings_from_result_event` (AC-282; production enforcement is the adapter's `rk-adapter-claude/src/classify.rs:576-579`, so the kernel version is a parallel, test-only copy). | Records classify these as instruments tested at the stage. None says they are wired. AC-300 holds on the shipped pack (`review = "mandatory"`, `.redkiln/packs/bug-standard.toml:19`; `ac300_fix_lane_is_strict` reads the shipped packs). |
| **No-sidecar means exit permitted** | `verify_pass.rs:741`: "No sidecar is no criteria: the ledger is then empty and permits the exit". A lane with no `Spec` artifact passes the verify exit. | Stated as "a build choice, stated and reversible" in the doc comment. |
| **Corpus-bound replay not in a default run** | `tests/it/replay.rs` items are `#[cfg(feature = "corpus")]` and need a corpus repo at `CORPUS_PIN`. The default test log does not include them (replay shows 1 test). Golden-log replay, AC-307's corpus replay half, AC-295 and AC-354 need a recorded corpus run. | AC-295, 307 (replay half), 354 ARMED to Step 8 (`s7b-ii` §7b-ii.14). The 7b and 7b-ii checkpoint blocks run `--features corpus` on a box with the corpus. |
| **AC-349 plan-time binding** | `claims::bind` runs only in `doctor` (`rk-cli/src/doctor/rules.rs:2432`), not in `run`'s plan. | `s8g` residual list ("No sub-stage owns wiring the binder into the plan"). |
| **Kernel-local recorded dispatches are fake** | The golden-log replay's N dispatches are a scripted fake harness (`src/replay.rs` header). | Stated in `replay.rs` ("N being served is N fake results ... 7c's"). |
| **Gate request for non-diff review (S-BIN)** | The `gate_request::` tests described for the human-gate route in s8i are not on disk (`grep gate_request` over `crates/` returns nothing). | `s8i-s-bin.md:165-176` says they are built after S-BIN reports. |

## 6. Code quality (CODE)

**Structure: A-.**
- The invariants are enforced by types and module privacy instead of discipline: private `Block` and `GateRecord`, a closed `EvidenceClass`, `permits_exit` as a three-line `matches!`, and a ledger signature that omits forbidden inputs.
- Functions are pure with injected `now`, so replay is trivially deterministic. The determinism ban (`clippy.toml`) is enforced with module-level `#![deny(clippy::disallowed_methods/disallowed_types)]` on `workflow` (`workflow.rs:218-219`) and positive-control features (`workflow-clock-reintroduce` etc., `Cargo.toml`).

**Error design: B+.**
- Typed enums with `Display` and `Error` throughout (`GateError`, `QuarantineError`, `ReviewPackError`, `ExitRefused`, `BoundaryError`, `ReviewIngestError`).
- Two soft spots:
  - Several seams take `&mut dyn FnMut(Event) -> Result<(), String>` (`verify/runner.rs:137`), which loses error structure.
  - `workflow::gate::answer` has 8 parameters and an `#[expect(clippy::too_many_arguments)]` whose reason cites a runbook section (`gate.rs:225-229`). A request struct would be cleaner.

**Escape hatches in non-test gate code (spot-checked, qualifies the census).**
- The census counts `unwrap/expect` 58, `panic` 8, allow/expect-attr 22 for the whole `rk-kernel` (47,005 src lines), which includes doc comments and test modules.
- In the gate modules (`review/*`, `verify/*`, `workflow/gate.rs`, `question.rs`, `idem.rs`, `runtime/advisory.rs`, `runtime/ledger.rs`) a code-line grep finds only two:
  1. `workflow/gate.rs:225` `#[expect(too_many_arguments)]` (justified).
  2. `runtime/advisory.rs:284` `pending.lock().expect(LOCK_HALTS)`, a deliberate lock-poison halt (ADR-0118), documented with `# Panics`.
- Elsewhere in `src` the non-test hits cluster in `predicate/parser.rs` (17), `runtime/tier_one.rs` (11), `select.rs` (9), `select/step5.rs` (6), `registry.rs` (6) (grep, not qualified to test vs production; outside this audit's area).
- `replay.rs:1207` has one `expect("json! built an object literal")`, which is an infallible case.
- **No workspace lint forbids `unwrap_used` or `panic`.** `Cargo.toml [workspace.lints]` carries `unsafe_code = "deny"` and an enumerated clippy subset only. The gate code's cleanliness is by convention and review, not machine-enforced.

**Silent fallbacks (non-panicking, but they swallow errors).**
- `confirm.rs:175` `serde_json::to_value(finding).unwrap_or(Value::Null)`: an unserialisable finding would persist `null` in the sink row.
- `runner.rs:104`: if the outcome fails to serialise, the report digest is computed over the error text.
- `ac_gate.rs:133`: `unwrap_or_else(|| "unrecorded")` for a missing verdict.
- All three are practically unreachable, but each is a place where an error becomes a value.

**Dead code and unwired code:** see §5 "Unwired helpers". `GateEligible`, `RegressionRecord` and `review::render` are instrument-only in production.

**Test depth.**
- Mostly integration tests with fakes, plus real-process and real-git tests for AC-245 (a tempfile git repo) and AC-286.
- `proptest` appears for AC-286, AC-296 and others, and trybuild for the type claims.
- Unit tests inside `src` are few (12 in lib) because the stage deliberately keeps tests in `tests/it`.
- Replay "k=10" tests (AC-313) run a pure function over scripted recorded input, which is correct for a pure function but is not a corpus run. The corpus replay is ARMED to Step 8.
- The suite pins the shape of behaviour well (red-first, mutants named in commit messages). I did not run mutation testing.

**Comment-to-code noise: heavy.**
- `src/lib.rs`'s header is about 300 lines of dated change-log narrative. Gate modules carry about 30% comment lines (review 358 of 1,222; verify 385 of 1,372; gate+question 321 of 975), and the whole crate 15,339 of 47,005 (33%).
- Many comments are provenance ("dated 2026-09-29, s8g build, package …"), strikethrough corrections and ADR cross-references. Much of the content is valuable, but it duplicates the runbook and is hard to maintain next to the code.
- Clippy's default run reports 315 warnings workspace-wide, with several `too_long_first_doc_paragraph` hits in `lib.rs` and `module_boundary.rs` (`scratchpad/audit/log-clippy.txt`, supplementary). They are warnings, not failures, since only the exit code gates.

**Structural debt.** Test instruments (source scanners that read the crate's own source tree with `std::fs`, plus the denylist and dependency-graph rules) are public modules of the production kernel library (`lib.rs:308-403`). They enlarge the public API and add I/O-capable code to a crate whose design says it does no I/O. They exist because the checkpoints call them as library functions. The textual scanners also depend on source formatting, so they are brittle even with positive controls.

## 7. Risks

1. **Overstated verify guarantee.** If a summary says "per-AC mutation verification is live", that is false today. Only the human `discriminates` answer can close an AC at the run path. The cost is that every spec AC with a verification requirement goes to a human gate. That is safe but is an operator-load and throughput issue for the Phase-1 exit lane.
2. **Review blocking is `run`-only in production.** Mutation-style findings never block a real run, and `narrowed-pin` never appears, so the Strict-weight policy rows for those classes are unreached at run time.
3. **Review gate calibration unknown** (S-SUBTLE-CB unrun) and AC-284's decision table is PROVISIONAL on S-CLASS-REPLAY, which is also unrun.
4. **Unwired lifecycle features** (quarantine, advisory retirement) mean the advisory sink only grows in a real run. SM-089's disposition goal (every sink record reaches a terminal disposition) cannot be met on the run path.
5. **Checkpoint passes are over the fixture corpus and fakes.** The kernel's gates have not been through a real end-to-end Phase-1 run. The first real review-side evidence, S-BIN (8i), is mid-flight (B1 complete, `ee44bf1`). The Phase-1 exit (Step 8) is the first end-to-end test of the gate chain.
6. **Process load.** The stage files for these gates run to about 3,800 lines (`s7b-ii` 1,037, `s7h` 328, `s8g` 2,395). Maintenance cost is high, and many checkpoint guards are assertions about text. This is a risk to whoever must keep them current.
7. **Minor design note (reasoning, not measured).** The human-gate token digest does not include the item id (`fsm.rs:354-359`). The fold is per item journal, so collisions are harmless within the current design.

## 8. Unverified (not read or not run)

- I did not run cargo, any checkpoint block, the `--features corpus` replay tests, or mutation testing. Test pass counts come from the orchestrator's separate log, not from my own execution.
- I read module headers, the core logic files and selected tests. I did not read every test body in `tests/it/gates.rs`, `verify.rs`, `review.rs` or the `rk-cli` `review_pass`/`verify_pass` tests, so I cannot confirm each named test discriminates as its doc says. Spot-checked: `ac313_replay_k10`, the `Block` privacy and lint structure.
- The per-AC status of all 122 claimed ids was not recomputed. Counts of discharged, armed and excluded ids are the stage file's, plus my grep census (87 ids with a named test fn).
- Whether `rk-cli` tests drive the review pass and the verify pass end to end with the fake `claude` is inferred from file names and headers, not read.
- The ADR statuses (0123-0125, 0162, 0177, 0186, 0190-0196) are taken from PROGRESS and stage text, not from the ADR files themselves.
- Whether the live S-BIN probe or capture sessions exercised `review::confirm` (as opposed to a standalone reviewer prompt) is not determined from this tree.
- A real model-driven review or verify run on this tree: none is recorded in what I read. The decision log says the shipped tree is "0 of 102" activities completing a committing walk as of 2026-09-21, and I did not re-measure it (`spikes/DECISIONS-2026-09-21.md:1458-1461`).
