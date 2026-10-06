# W1 audit: `crates/rk-kernel`, the RUNTIME side

Snapshot: worktree `audit-wt` at commit `36dd8ac` (build/phase-0-to-1). Read-only. No cargo was run by me; I read the
sibling build logs at `scratchpad/audit/log-*.txt` (a separate build, same snapshot) for pass/fail evidence and mark
that evidence "log" below. All paths are relative to the worktree.

**Verdict: grade B (solid with debt).** Completeness against what Phase 1 asks of this area: about 80%.

## 0. Scope note: much of the "runtime side" is not in `rk-kernel`

The task names eight sub-areas. Only some of their code lives in `crates/rk-kernel`:

| sub-area | where the code is | in my scope? |
| --- | --- | --- |
| step executor, five kinds (7g) | `rk-kernel/src/runtime/step.rs` (3206 lines) | yes |
| reference resolver (8d) | `rk-kernel/src/runtime/reference.rs` (1590) | yes |
| step-grain commits, loops, `on_false` (8d) | `runtime/step.rs` (the walk), `runtime/activity.rs` (1698) | yes |
| lane completion, successor, `AdvanceStage`, trailers (7h) | per-stage half in `rk-kernel/src/replay.rs` (`resume_routed`, :1932); `Runtime::step` AdvanceStage arm (`runtime.rs:769`). The multi-stage lane loop is `rk-cli/src/verbs/run.rs` | kernel half only |
| tier-1 git-first writer, heal, seal, close (8e) | `runtime/tier_one.rs` (2343) | yes |
| landing checkpoint (8f) | `tier_one.rs` `land_act` (:947) over `rk-git/src/landing.rs` | kernel half only |
| run worktrees (8d-ii) | `rk-git/src/worktree.rs`, `rk-git/src/mutation.rs`; kernel only consumes ownership (`tier_one::Target`, `assigned_ownership` :1889) | consumer only |
| cost / throttle (7i) | `rk-adapter-claude/src/throttle.rs`, `rk-cli/src/verbs/cost.rs`; the kernel only reads `dispatch_completed` rows (`workflow/envelope.rs:443`) | consumer only |
| cards, models, run env (8h) | `rk-adapter-claude`, `rk-cli`, `rk-types`; kernel part is the per-item envelope (`workflow/envelope.rs`, bound at `replay.rs:2051`) | envelope only |

The other sub-areas need other auditors; I do not grade them here.

## 1. Module map and public surface

Runtime boundary (`src/runtime.rs`, 836 lines, 50% comment/doc):

- `Runtime` (`runtime.rs:361`): builder-style (`with_item`, `with_boundary`, `with_breaker`, `with_envelope`,
  `with_read_seam`, `with_write_seam`, `with_tool_routing`). Single acting method `Runtime::step` (`:720`) turns a
  `NextAction` (Dispatch, OpenGate, AwaitGate, AdvanceStage, Halt, Done) into one effect. `RuntimeError` is a hand-written
  Shape-A enum with stable tags (`:211`).
- Children: `activity` (dispatch, commit, retry loop), `advisory`, `ledger` (executed-once), `marks` (retry_at mark),
  `reference` (resolver), `step` (executor), `tier_one` (git-first writer).
- `step.rs` seams: `ToolRouting` (`:205`) carrying `StoreRoute`, `GitRoute`, `ReviewRoutes`, plus `reference.rs`
  `ArtifactReadSeam`, `ArtifactWriteSeam`, `RunBranch`. Each has an `Unbound*` default that refuses by name
  (`step.rs:520-570`), so a composition root that forgets to bind one fails closed.
- Per-stage entry point `replay::resume_routed` (`replay.rs:1932`), `record` (:1834), `replay` (:2879), golden log
  `write_run`/`read_run`.
- `tier_one`: `Boundary` (:1695), `Act`, `Mark {Stage, Closed, Gate, Seal}` (:93), `Content` (entry, step checkpoint,
  landed), `commit_act` (:855), `heal` (:1932), `close_act` (:1260), `seal_act` (:1651), landing refusal gate, drift gate.
- `lib.rs` has 24 `pub mod` and every module is fully public (`lib.rs:308-403`); there is no encapsulation layer. The
  crate is `publish = false`, so this is a style choice, not a breach, but any module can be reached from `rk-cli`.
- The crate also hosts non-runtime "instrument" modules (`lattice.rs` 2855 lines, `module_boundary.rs` 1164,
  `kernel_denylist.rs`, `no_egress.rs`, `adapter_edge.rs`, `quality_block.rs`). They are outside my area but inflate
  the crate: 47,005 source lines, 102,484 including tests.

Size: `src/runtime*` is 10,332 lines; `replay.rs` 3,081; `workflow/fsm.rs` 2,055; `workflow/memo.rs` 2,094.

## 2. What Phase 1 requires of this area (from the records)

- design §4.5 (quoted in `runbook/phase-1/s7g-step-executor.md:11-15`): five closed step kinds, each executed as itself;
  `check` cannot dispatch; `gate` writes a mark and exits 0.
- s7h: the lane advances stage by stage and mints `Story:` / `Slice-Verdict:` trailers so a run reaches a landed
  commit (`s7h-lane-completes.md:12-30`).
- s7i: `redkiln cost` over journalled readings (adapter/CLI work).
- s8d: step-grain commits, the complete `${...}` reference resolver, loops (`s8d-step-grain-and-resolver.md`).
- s8d-ii: run-owned worktrees; s8e: kernel-owned git-first tier-1 writer, lane across processes, `Slice-Verdict:`;
  s8f: landing checkpoint; s8h: cards, pinned models, per-item envelope (`s8h-cards-and-run-environment.md`).
- design §3.6 / §7.5 (`references/2026-09-08-architecture-design.md:333-368`, :1193-1204): three never-shared budgets,
  inner and outer retry, a per-family circuit breaker, degradation policy, and "budget exhaustion yields
  `Err(BudgetExhausted)`, the run writes a durable mark and exits 0".
- Phase-1 exit (`runbook/phase-1/s8-exit-criterion.md`, tracker row `PROGRESS.md:51`): Redkiln-RS runs its own fix lane
  and authors a Phase-2 story. Step 8 is recorded as NOT passed and M5 not reached.

## 3. Compare: built, exercised, deferred, missing

### 3.1 Genuinely implemented and exercised (code + tests)

1. **Five step kinds execute.** `Runtime::run_activity_steps` (`step.rs:1538`) walks the list with a program counter,
   serves committed step activations, and executes the rest. `check` is a free function with no `&self`
   (`step.rs:1084`), so it cannot reach a port. Verbs refuse by name when unrouted (`step.rs:733-775`). Tests:
   `tests/it/step_grain.rs` (10), `gate_step.rs` (7), `tool_verb_routing.rs` (10), `on_false_exec.rs` (21),
   `check_cannot_dispatch.rs` (2), `step_error_row.rs` (4), `second_agent.rs` (2), `dispatch_completed.rs` (3).
2. **Both shipped lanes walk end to end under fakes, over the real pack.** `tests/it/lane_walk.rs` reads `.redkiln/`
   from disk, walks selection and every selected activity's shipped step list with a scripted `FakeHarness` per step, a
   scripted `Toolchain`, and real git. The records' measured result is fix lane 14 of 14 and story lane 13 of 13
   (`s8d-step-grain-and-resolver.md` dated lines; `s8g-verify-and-restored-demands.md`; asserted by the two tests at
   `lane_walk.rs:1580-1586`, `selection_refusals: &[]` at :1485 and :1574). **Caveat:** the `Verdict::Refuses` variant is
   never constructed (`lane_walk.rs:125`; rustc dead-code warning in `log-test.txt`), so the table can no longer detect a
   walk that should refuse.
3. **Reference resolver (ADR-0173).** `runtime/reference.rs` parses ADR-0040's closed source set (step refs, bare
   evidence pointer, `??` coalesce, gate `.answer`/`.attachments`, `${item}`, `${toolchain.k}`, `artifact:`, `kb:`,
   `checklist:`, `toolchain:feed`); an unresolvable reference is a named `RefError`, never a value
   (`reference.rs:1-60`). Tests: `reference_resolver.rs` (17), `forward_ref_lint.rs` (3), `guard_context.rs` (2).
4. **Step-grain commits, loops, re-entry.** `commit_decision` replays only decisions whose route continues the walk;
   re-entry resumes at the first uncommitted step (`step.rs:1590-1625`). Process-boundary proof:
   `cross_process.rs` re-execs the test binary via `current_exe()` (:191-208) and kills it mid-graph
   (`kill_and_resume_n_step` :1274, AC-013 step grain; `ac035_breaker_refusal_survives_a_real_process_death` :523;
   AC-024 mark survives exit).
5. **Tier-1 git-first writer (8e).** `commit_act` authorises the target, commits through the journalled `GitOps` seam,
   re-reads HEAD and tree, then appends rows anchored to that sha (`tier_one.rs:1-40`). A failed commit leaves zero
   store rows; `heal` re-derives rows from trailer marks reachable from `rk-run/<item>` (:1932); replay performs zero
   git writes. Tests use a real repository through `GitExec` (`tier_one.rs` 18 tests, `step_landing.rs` 2). One test
   asserts the writer is called only from the boundary (`tier_one.rs:960`).
6. **Landing (8f), kernel half.** Landed content commits via `commit-tree` + `update-ref`, refusal gate
   (`LANDING_REFUSED_KIND`, `refused_landing_act`), drift gate, entry/step/landed `Content` arms. Tested by
   `step_landing.rs` and `tier_one.rs`; the rk-git primitive is another auditor's.
7. **Stage advance, successor, trailers (7h/8e), kernel half.** `resume_routed` takes exactly one stage boundary and
   breaks (`replay.rs:2105-2160`; ADR-0152), enters a stage once per `(item, stage)`, holds an advance while a
   pass-sourced `artifact.write` is owed, and reports `lane_end`. Tests: `lane_order.rs` (12), `resume.rs`,
   `cross_process.rs`.
8. **Per-item envelope (8h) is live.** `ItemEnvelope::admit` runs before every dispatch (`runtime/activity.rs:636-638`)
   and reads the store, not memory; the pack must declare both `appetite.dispatches` and `appetite.tokens`
   (`replay.rs:2051`; `workflow/envelope.rs:476`). Per-family breaker bound from the pack (`BreakerBinding::of`
   `runtime.rs:424`), outer retry live (`activity.rs:1061`), `retry_at` mark written on RateLimited
   (`activity.rs:1067`).
9. **Determinism and I/O discipline hold.** No clock: time arrives as `JournaledNow`. `workflow` carries
   `#![deny(clippy::disallowed_methods/types)]` with committed positive-control features (`Cargo.toml` features
   `workflow-clock-reintroduce` etc.). All store access goes through one `Arc<Mutex<dyn EventStore>>`.

Test evidence (log): `rk-kernel` lib 12 passed; integration `it` 417 passed, 0 failed, 5 ignored (child-process
entrypoints and the fixture writer, by design). `cargo fmt --check`, build and `cargo clippy --workspace --all-targets`
all rc=0 on this snapshot (`audit/build.txt`). Other crates in that log had failures (below, section 6).

### 3.2 Library-only, unwired, ARMED or deferred (code exists, no live call site)

I counted non-test, non-self call sites across `crates/*/src` for every `pub fn` of 8+ characters in `runtime/`,
`workflow/` and `replay.rs`; 47 have zero. The ones that matter:

| item | location | what is missing | records |
| --- | --- | --- | --- |
| AC-024 wall-clock expiry mark writer `wall_clock_expiry_mark` | `workflow/budget.rs:1050` | zero non-test callers (only `tests/it/budget.rs`, `cross_process.rs`) | `s8h:372` records "No producer" explicitly |
| AC-025 exhaustion gate `evaluate_gate`, `read_budget_extension_marks`, `BudgetLedger` fold | `workflow/budget.rs:873,1006,417` | zero non-test callers; no `BudgetExhausted` can occur on a live run. The only live budget enforcement is the per-item envelope above and a per-dispatch wall timeout passed from `rk-cli/src/verbs/run.rs:6112` | design §3.6 row "Budget exhaustion" says it must exist |
| per-pass `attempts_total` charge `charge_pass`, `charge_observed_inner_attempts`, `required_attempts_total` | `workflow/budget.rs:523,553,577` | not called by the runtime; `note_charge` (`activity.rs:360`) records only dispatch counts | ADR-0122; stage files call it ARMED |
| inner-retry accounting `decide_after_inner_retries`, backoff `schedule_for` | `workflow/retry.rs:548,815` | rule pinned by tests, wiring "OWED" (comment at :540-548; `s8d` D-entry "inner attempts ARMED") | design §3.6 rows "Inner retry", "Outer retry" |
| degradation policy `degrade::substitute`, `degraded_event`, `gate_closes` | `workflow/degrade.rs:285,320,388` | zero callers in `rk-kernel/src/runtime`, `replay.rs` or `rk-cli/src` | AC-036..038, design §3.6 |
| executed-once ledger readers, advisory derivation | `runtime/ledger.rs`, `runtime/advisory.rs` | have callers in the replay/record path; `recorded_advisories`/`derive_advisories` have none outside tests | ADR-0114, AC-525 |
| mid-walk `deviate` route | `step.rs:2214,2242` | refuses by name; `RequestDeviation` mid-walk is "OWED" | D21-48 X7 |
| a second runnable `agent` step with no per-step specs | `step.rs:1651` fallback | refuses by name; per-step specs now exist (`Activation.step_specs`) so only the pre-R1 fallback refuses | ADR-0143 |

Honest reading: the budget/degradation policy layer of design §3.6 is built and unit-pinned but is not what stops a
real run. What stops a real run today is the dispatch cap, the token appetite, the breaker and outer retry. Whether
Phase 1 exit needs the rest is a judgement the records leave to later; I mark it "unverified".

### 3.3 Missing for Phase-1 exit (records, not code)

- **No real lane run has been driven through this kernel.** Step 8 is RE-OPENED (`PROGRESS.md:51`); Step 8j "Fixture
  proof of both lanes, CI green, and the run preparation" is "PLANNED, Not started" (`PROGRESS.md:50`); Step 8i (S-BIN) is
  IN PROGRESS (`PROGRESS.md:49`). Everything above is proven under fakes, real git and real process boundaries, but
  never with a live `claude -p` dispatch through `run_activity_steps`. (The adapter alone has a live probe, Checkpoint
  7c, `PROGRESS.md:33`.) The shipped store held zero `dispatch_completed` rows when 7i opened (`s7i-cost.md:25-35`).
- **ADR-0197 (cards, models, run environment) is the one `proposed` ADR** (193 accepted, 1 proposed, 4 superseded,
  counted from `references/adr/*.md`); it flips only after s8j's probes (`PROGRESS.md:305`).
- Several earlier checkpoints are recorded "provisional, re-run pending s8j" (8a, 8c, 8d, 8d-ii, 8e, 8f, 7c), so the
  kernel-side checkpoint statuses for 8d/8e/8f are provisional until s8j (`PROGRESS.md:33,42-46`).

### 3.4 Honest read of the "ARMED" ledger

The code and tests are more complete than the older prose suggests, because the prose is stale in places (section 4).
The tracker rows show checkpoints re-run whole and passing at `d24d15a` for 5, 7b, 7b-ii, 7d, 8b and 8g; I did not
re-run any checkpoint block.

## 4. Code quality

**Structure.** Good. The workflow/activity split is real: `workflow` is pure and lint-fenced, `runtime` is the only place
that appends, dispatches or locks, and `runtime.rs:1-60` argues why. `check` cannot dispatch by signature. Unbound seams
refuse by name. The dispatch path orders refusals deliberately (ADR-0144/0145 comments at `replay.rs:1695-1730`). The
FSM is a pure fold; golden-log replay serves rows from the journal rather than dispatching.

**Error design.** Typed, hand-written enums with stable tags (house "Shape A"), `#[non_exhaustive]`-style closed sets,
`StepError` at 11 variants, refusals name the step and reference. No `anyhow`. A real strength.

**Escape hatches in non-test code (spot-checked, not the raw census).** The census says `unwrap/expect=58`, `panics=8`,
`allow/expect-attr=22`. After excluding doc comments, strings and `cfg(test)` modules (my own scan of `src/`), real
sites are:
- 16 `.expect(LOCK_HALTS)` on mutex acquisition (`runtime.rs:493,528,651`; `runtime/activity.rs:911`;
  `runtime/advisory.rs:284`; 11 in `runtime/tier_one.rs`). Deliberate: ADR-0118 makes a poisoned store lock a halt and
  `rk-cli`'s panic hook matches the constant.
- 4 invariant `.expect("...")`: `replay.rs:1207`, `predicate/parser.rs:252,333,371`; 1 `unreachable!` `predicate/eval.rs:414`.
- 1 real `panic!` in a public builder: `select/item.rs:240` (`with_facet_ordinal` panics on a member outside the
  declared domain; its 13 other references are all in tests and fixtures, none in `src` beyond the definition).
- 11 `#[expect(...)]` attributes, each with a written reason (cast precision, `too_many_arguments`,
  `significant_drop_tightening`, `too_many_lines`). Zero `#[allow]`, zero `unsafe` (workspace `unsafe_code = "deny"`),
  zero TODO/FIXME.
- The 17 hits in `predicate/parser.rs` are a method named `expect`, not the panicking method.
Workspace lints do NOT deny `unwrap_used`/`expect_used`/`panic` (`Cargo.toml:96-113`), so the "no escape hatch" rule is
by convention and review, not enforced by the compiler.

**One fail-open spot in a money guard.** `ItemEnvelope::charged_tokens` silently skips a `dispatch_completed` row whose
`usage` fails to deserialize (`workflow/envelope.rs:448-450`, `let Ok(usage) = ... else { continue }`), so a malformed
row charges zero tokens. The code comment says the dispatch cap bounds this; it is a conscious choice but it is
fail-open.

**Test depth.** Strong on integration, thin on unit. All of `src/` has only 12 unit tests (429 `cfg(test)` lines); the
rest are 417 integration tests in one binary (`tests/it/main.rs`, per ADR-0083), many of them 1,000-2,000-line files
with heavy fixtures. Includes: 15 `trybuild` compile-fail cases (`tests/ui`, each with a `.stderr` snapshot), a re-exec process harness, real-git tests,
proptest in `review.rs` only, 77 in-test references to hand-run mutants, and **one** committed golden log of **8
events** (`tests/fixtures/logs/implement-bf1d8cec3b22d288`, toy `incident` pack). Replay determinism is therefore
asserted over a very small recorded run plus the fake-driven walks; AC-006/AC-008 coverage is real but narrow.
Source-text lints (e.g. `boundary_prose_extraction_rule`, `activity.rs:1678`; `module_boundary.rs`) are tests that grep
production source: effective but brittle to refactors.

**Dead and unwired code.** Beyond section 3.2, `is_bound` (`runtime.rs:477`), `execute_step_output` (`step.rs:2695`) and
`question_ids` (`workflow/question.rs:222`) have no references anywhere, test or src. A handful of pure accessors
(`expired_gates`, `committed_rows`) have only test callers.

**Comment-to-code noise (the biggest quality debt).** In `rk-kernel/src` (outside `cfg(test)`): 28,815 code lines vs
13,260 doc-comment lines and 2,000 `//` lines, so 35% of non-blank lines are commentary. `runtime.rs` is 50%,
`workflow/budget.rs` 58%, `workflow/retry.rs` 57%, `workflow/breaker.rs` 56%. Evidence of rot:
- 264 inline "dated 20xx / corrected 20xx" notes and 64 struck-through (`~~`) passages recorded inside source comments,
  1,314 `ADR-` references, 160 stop-sign markers. History belongs in git and the ADRs.
- **207 mentions of "proposed" in `src/`** while 193 of 200 ADRs are accepted (e.g. `runtime/step.rs:535-541` cites
  "ADR-0195 decision 2, proposed" and ADR-0195 is accepted).
- Headers contradict the code: `runtime.rs:131-146` says the FSM dispatch key is unversioned and "a record is OWED"
  (a test still pins that for the bare `Runtime::step(Dispatch)` arm, but the real walk goes through `dispatch_through`);
  `runtime.rs` "What is deliberately NOT here" says "No `pub trait` of any shape" but `runtime/` now declares six
  (`ArtifactReadSeam`, `ArtifactWriteSeam`, `RunBranch`, `ReviewRoutes`, `StoreRoute`, `GitRoute`;
  `grep "pub trait" src/`); `step.rs:41` and `:106` still say per-step specs and step-output resolution are OWED though
  s8d built both. `lib.rs:1-150` is an eight-paragraph changelog that begins "Steps 5, 6, 7 are not built and not
  stubbed".
- Clippy (log): of 78 distinct warnings in `rk-kernel/src`, **67 are "first doc comment paragraph is too long"**
  (`tier_one.rs` 15, `module_boundary.rs` 12). rc is 0 because the lint is warn-level.

**Function size.** 65 of 1,186 functions exceed 60 code lines; 33 exceed 100 (my count; brace-matching may inflate
two). Largest real ones: `run_tool_step` 254 (`step.rs:1206`), `fsm::apply` 239 (`workflow/fsm.rs:1395`), `land_act`
219 (`tier_one.rs:947`), `read_run` 219 (`replay.rs:2481`), `resume_routed` 198 (:1932), `one_step` 165
(`step.rs:1750`), `run_activity_steps` 144. They are linear and heavily commented but hard to review.

**Seam design.** `Runtime` accumulates optional bindings through ~12 `with_*` builders and several `Slot` wrappers with
hand-written `Debug`/`PartialEq` (`step.rs:574-700`) just to carry `dyn` seams. It works and fails closed, but a
`RuntimeBuilder` returning a fully bound runtime would remove the "forgot to bind" class entirely.

## 5. Risks

1. **Never run live.** The kernel's lane walk, tier-1 landing and heal are proven under fakes and real git, but no real
   `claude -p` has run through `run_activity_steps`. The first live run will exercise untested interactions (adapter
   output shapes, real token usage into the envelope, real rate-limit rows). Highest-impact risk for the exit criterion.
2. **Budget/degradation policy is unwired.** A real run today is bounded only by dispatch cap, token appetite, breaker,
   per-dispatch wall timeout and outer retry. AC-024/025 (exhaustion gate and durable mark, exit 0), inner-retry
   accounting and degradation recording are library-only. If Phase 1 claims design §3.6 delivered, the claim outruns the
   wiring.
3. **Envelope read-then-dispatch is not one critical section** (`activity.rs:636-640`: `admit` over a journal read, then a
   separate `append` and dispatch). With a single lane process per item this is benign; two concurrent processes on one
   item could both pass the cap. I did not check whether `run`'s lane lock prevents it (unverified).
4. **Fail-open token charge on a malformed `usage` row** (section 4).
5. **Doc rot.** Stale "OWED/proposed/not built" claims will mislead the next maintainer or the next agent; the
   program's own records assert that prose-versus-tree drift has repeatedly caused false claims. Worth a dedicated
   comment-diet pass after Phase 1 closes.
6. **One tiny golden log** limits regression power of replay determinism across schema moves.
7. **Public-everything crate and source-grep lints** make refactors expensive; a rename can break a "lint" test.
8. **Kernel-local `pub trait` seams** (six in `runtime/`) sit outside the "five ports and no sixth" check, which parses
   `rk-ports` only (`runtime.rs` header). Defensible (ADR-0003 says git is not a port) but the lint cannot see them.

## 6. Unverified

- I did not run `cargo`; test and clippy results are from the sibling build logs (`audit/log-test.txt`,
  `log-clippy.txt`) for the same snapshot. In that log `rk-cli --test it` had 63 failures, 55 of them
  `ANTHROPIC_BASE_URL ... is refused` (an environment variable in the audit shell tripping the kernel-adjacent launch
  guard, ADR-0010), and `rk-adapter-toolchain-node` 14 (missing `pnpm install`). I believe these are environmental
  but did not confirm; the rk-cli auditor should.
- Whether the lane lock prevents concurrent envelope races (risk 3).
- Whether design §3.6's budget-exhaustion and degradation rows are required for Phase-1 exit versus Phase 2; the records
  call them ARMED without a dated exit decision I could find.
- I did not re-read design §3-§10 in full; design citations are to the §3.6/§7.5 tables and to the stage files'
  quotations of §4.5.
- Checkpoint pass claims in `PROGRESS.md` (e.g. "passed at d24d15a, 1404 tests") were not re-run.
- The "14 of 14 / 13 of 13" lane counts come from the stage records plus the two test files' empty refusal lists; I
  did not execute the tests to read the `LANE_WALK` lines myself (the log run shows both lane tests passing within the
  417).
- Function-size counts came from a brace-matching script and may over-count where `'{'` char literals occur.

## 7. Key numbers

- `rk-kernel`: 47,005 source lines (28,815 code, 13,260 doc, 2,000 line comments, 2,579 blank), 55,479 test lines,
  12 unit + 417 integration tests passing (log), 5 ignored by design, 15 trybuild cases, 1 golden log (8 events).
- Runtime area source: `runtime/` + `runtime.rs` 10,332 lines; test files for it about 15,000 lines.
- Non-test escape hatches: 16 `expect(LOCK_HALTS)`, 4 invariant `expect`, 1 `unreachable!`, 1 `panic!`, 11 reasoned
  `#[expect]`, 0 `#[allow]`, 0 `unsafe`, 0 TODO.
- Unwired public policy functions: 47 pub fns with no non-test caller, including `wall_clock_expiry_mark`,
  `evaluate_gate`, `degrade::substitute`.
- ADRs: 193 accepted, 1 proposed (0197), 4 superseded of 198 counted.
