# Step 7g — The step executor and the Toolchain port: the five closed step kinds, executed (Phase 1, part 5)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7g` | Phase 1 | Checkpoint 7f | Checkpoint 7g | `s7f` | `s7h` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** `s7f` §7f.1 states the finding and arms the gate; this step discharges two thirds of it.
design §4.5 fixes **five closed step kinds** and gives each its own execution — *"`agent` (the only place a model
runs), `tool` (a toolchain verb through the port), `check` (a deterministic predicate in Rust; cannot dispatch),
`gate` (the kernel writes the mark and **exits 0**, resuming into the same step), `emit` (binds outputs to
`provides`)"* — and the runtime implements one of them by dispatching the whole activation to the harness.

**This is a seam, not a design.** The data is already parsed and the surrounding machinery already reasons about
step kinds: `catalog::Step` carries `kind`, `run`, `output_type`, `role`, `when`, `expr`, `for_each`, `retry_max`,
`repeat_max`, `memo_declared`; `workflow::budget` already branches `StepKind::Agent | StepKind::Tool`;
`workflow::memo` already reads `step.kind == StepKind::Tool && step.memo_declared`; `rigor` already branches
`StepKind::Tool | Gate | Emit`; `predicate::eval` is the `check` arm's evaluator and is complete. **Only the
execution point is missing**, which is why `ports.toolchain`, `ports.tracker` and `ports.docs` have zero non-test
call sites.

~~**This step takes no decision.** Every arm's behaviour is design §4.5's sentence for that kind.~~
**This step takes SIX decisions, and the sentence above was false when written** *(corrected 2026-09-22 at this
stage's open, before any executor code was written)*. Every arm's *behaviour* is indeed design §4.5's
sentence for that kind — that much stands. What design §4.5 does not settle, and what this stage
cannot be built without settling, is:

| # | what was owed | record |
| --- | --- | --- |
| 1 | the port call-site lint asserted a property Phase 1 is DESIGNED never to satisfy | **ADR-0134** |
| 2 | no verb → port-method table exists anywhere; `resolver.rs` says a record is OWED | **ADR-0135** |
| 3 | 54 of the pack's 121 step guards carry a `${…}` reference nothing resolves | **ADR-0136** |
| 4 | the real toolchain adapter is never constructed on the run path | **ADR-0137** |
| 5 | `ActivitySpec` carries no step list and the runtime may not load the catalog | **ADR-0138** |
| 6 | clause 8 of ADR-0137, corrected by its own first execution | **ADR-0139** (supersedes 0137 in part) |
| 7 | clause 5 of ADR-0137 — a per-step timeout nothing applies | **ADR-0140** (supersedes 0137 in part, a second clause) |
| 8 | clause 4 of ADR-0138 — an if-then whose consequent was declined | **ADR-0141** (supersedes 0138 in part) |

The first six are recorded **before** the code they authorise; **0140** and **0141** are the two supersessions `s7g`'s own errata named as OWED, taken at the repair pass. All eight are flipped on their own dated lines **D21-9 … D21-16**.
Four were settled by the operator; **0138** and **0139** were engineering calls taken inside the approved plan,
and their dated lines say so rather than presenting them as operator choices they were not.

### 7g.1 The executor walks the steps, and the kind chooses the arm

`Runtime` gains the step walk. The activation remains the unit of *selection* — the DAG's node is an activity, and
that is correct (selection picks activities; execution walks their steps) — but the unit of *execution* becomes the
step, in declaration order, with `when` consulted per step.

The five arms, each pinned to its own sentence:

| kind | arm | the port it reaches |
| --- | --- | --- |
| `agent` | the existing dispatch, unchanged | `Harness` |
| `tool` | `run = "toolchain:<verb>"` resolved against the capability registry | **`Toolchain`** |
| `check` | `expr` through `predicate::eval`; **cannot dispatch** | none |
| `gate` | the kernel writes the mark and exits 0, resuming into the same step | none |
| `emit` | binds outputs to the activity's `provides` | none |

🛑 **AC-013's idempotence is per ACTIVATION and must stay so.** `is_committed(item, activity_id_at_version)` is the
FSM's own predicate and the re-entry path reads it; a step walk that re-ran committed activations would break
`kill_and_resume_n_step` and criterion (3). The walk happens **inside** an activation that is not yet committed.

🛑 **`check` cannot dispatch, and that is asserted, not assumed.** design §4.5's parenthesis is a prohibition; the
arm takes no `Ports` at all, so the compiler carries it.

⚠️ **And the ARM THAT ROUTES TO IT is asserted too, since 2026-09-22.** `PROGRESS.md` row 37's finding
(13): the lint pinned `check_verdict`'s receiverless signature and nothing else, while a dispatch would be
written in `execute_step`'s `StepKind::Check` arm — which is inside `impl Runtime`, has `&self`, and sits
past the column-0 `}` that ended the body scan. Adding `self.run_tool_step(step)` there COMPILES, reaches
`ports.toolchain` from every `check` step, and left the lint GREEN. The lint now scans that arm as a joined,
whitespace-stripped region — its first repair banned `"self."` per line and was defeated by `rustfmt`
splitting `match self` from `.run_tool_step(step)`.

🛑 **THE `gate` ROW OF THAT TABLE WAS A STATEMENT OF INTENT UNTIL 2026-09-22, AND IS NOW A FACT.**
Finding (4), repaired by **ADR-0145** (D21-21). The arm `break`ed and wrote nothing, and the comment beside
it named `Runtime::step`'s `OpenGate` arm as the writer — which is reached only from `selection.gates()`,
never from a catalog `[[step]] kind = "gate"`. Measured: **55** of the pack's activities had a gate silently
skipped after their first agent step, and **15** reached one before dispatching and were handed the `emit`
refusal, *a message about a mechanism that has nothing to do with gates*. The arm now mints through
`workflow::gate::reach` and appends `runtime::gate_opened_event` — the same token-bearing row, so the fold,
the `AwaitGate` action and `terminal_read`'s exit 0 all compose unchanged. `catalog::Step` parses `question`
for the first time, as the gate's KIND. **This is the first repair in this stage that makes something
executable rather than honestly refused**, and it still does not make any of the 102 activities complete.

⚠️ **Finding (3), repaired by ADR-0144** (D21-20): a step failing AFTER a committed dispatch had its error
discarded, so the run reported a success having executed nothing past the dispatch — ADR-0135's by-name
refusal voided at the last step, for a re-measured **29 of 102** activities that reach a step which cannot
succeed on this tree at all. The commit stands and the failure now travels on its own field.

### 7g.2 The Toolchain port's call site, and what AC-473 needs from it

`rk-adapter-toolchain-rust` implements the `Toolchain` port — `detect`, `build`, `test`, `lint`, `classify`,
`change_summary` — and **is never constructed on the run path**: `run.rs` wires `FixtureToolchain::default()`.
AC-473 requires the self-hosting run's *"toolchain stage [to have] run through `rk-adapter-toolchain-rust` behind
the `Toolchain` port (ADR-0094 decision 1)"*, so until a `tool` step reaches the port that clause has no mechanism
and criterion (1) cannot be satisfied however well the lane otherwise runs.

The shipped catalog's `tool` steps, **re-measured 2026-09-22** — **113** across **33** verbs, the largest being
`toolchain:test` (**28**), `toolchain:change_summary` (11), `toolchain:verdict_record` (8), `toolchain:deploy` (6)
and `toolchain:metrics` (6). *(This line read **"12 verbs"** until 2026-09-22. The **113** was right and the 12
was not: measured by `grep -hoE '^run += *"toolchain:[a-z_]+"'` over `.redkiln/catalog/*.toml`, deduplicated,
there are **33** distinct verbs across 62 of the 102 activity files. The figure matters because it is the
denominator of ADR-0135's gap — only **four** of the 33 name a `Toolchain` port method (`test` 28,
`change_summary` 11, `build` 4, `lint` 4; **47** steps), so **29 verbs / 66 steps** have no port method at all
and are refused by name. At 12 the gap reads as a rounding error; at 33 it is most of the catalog.)*
The adapter selection is `config.toml`'s `[adapters] toolchains`, which already lists
both adapters; which one serves an instance is ADR-0073's keyed expansion and is not re-decided here.

**`change_summary` stays refused.** Both adapters return `Err(registry::refusal(...))` from a dated registry row,
and ADR-0093 decision 4 keeps `ChangeSummary` an opaque non-finalised newtype until S-BIN reports. A `tool` step
naming `toolchain:change_summary` therefore reaches the port and receives `Unsupported` — which is the correct
behaviour and must not be "fixed" into a synthesised value. Checkpoint 5's RED is untouched by this step.

### 7g.3 The `.redkiln/` fills the run demands, authored when it demands them

The first invocations after §7g.1 will refuse in turn on inputs Step 7e did not author, each measured 2026-09-21 at
Step 8: `.redkiln/schemas/<Kind>.json` is absent (read per role-bearing node); **all 8** cards lack
`permission_mode`; both packs' `[role.*]` tables carry `family` and no `model`/`effort`; and **20 of 102** catalog
activities carry no `role =` on any code line, of which `change-summary` is in `bug-standard`'s `[stage.implement]`
baseline and `confirmation-test` / `smoke-test` are the verify stage's.

**Authored by selection, never speculatively.** A fill is added when a run demands it and the refusal names it —
twenty roles invented ahead of a run would be twenty unmeasured decisions, and `s7e`'s dispositions are the
precedent for recording each one's warrant.

**AND AT THIS STAGE'S CLOSE, SELECTION HAS DEMANDED NONE — so none is authored, and that measurement is the
result.** `redkiln run rk-fix-doctor-test-count` was executed on this tree at `s7g`'s close, against the real
composition root with the real toolchain adapter wired. It reached **`dag_assembled`** — the positive witness,
emitted by selection step 5 and reachable only after the catalog, the artifact registry and the facet registry
have all loaded, so the shipped pack now loads **on the run path** and not only inside `shipped_pack_loads`. The
run then exited 0 having dispatched nothing, and the record says exactly why: the assembled DAG carries
**`"nodes": []`** and **`"rows_fired": 0`**, with two `forbidden_stage` skips (`discovery`, `operate`).
`inputs/specs.json` is `{}` because there was nothing to bind, not because binding failed.

**So not one of §7g.3's four gaps was reached.** No `.redkiln/schemas/<Kind>.json` was read, no card's
`permission_mode` was consulted, no `[role.*]` `model`/`effort` was wanted, and no activity's missing `role =`
refused anything — because no activation was selected. Authoring them now would be precisely the twenty
unmeasured decisions the paragraph above forbids, so this stage authors **zero** fills.

🛑 **What this does NOT say.** It does not say the four gaps are closed or absent — all four remain
measured and open, exactly as recorded above. It says only that the mechanism §7g.3 prescribes (*a fill is
added when a run demands it and the refusal names it*) has not yet fired, and that the reason is upstream: an
item that selects no activation at `triage`. **That is pack authoring's — `s7e`'s and Step 8's** — and Step 8
already records criterion (1) NOT SATISFIED on pack-authoring grounds. It is carried forward, not absorbed here.

### 7g.4 Checkpoint 7g

- **Two of the three ARMED conformance lints are GREEN**, their `#[ignore]` removed in the commit that made each
  pass: every `StepKind` has an executor arm, and `ports.toolchain` has a non-test call site.
- **A `tool` step reaches `rk-adapter-toolchain-rust`** through the `Toolchain` port, asserted by the adapter being
  observed rather than by the step being selected.
- **`check` cannot dispatch** — asserted structurally at `check_verdict` **and at `execute_step`'s
  `StepKind::Check` arm**, which is where a dispatch would actually be written (finding (13)).
- **AC-013's re-entry still holds**: `kill_and_resume_n_step` and criterion (3)'s asserted half stay green.
- **Criterion (2) stays green** — the step walk must not move the tier-1 projection.
- **The step walk is ENTERED, and by the PRODUCTION route** — `run_activity_steps` is reached both
  directly and through `replay::resume`. Added 2026-09-22: finding (12) measured that a `panic!` on its
  first statement left all six guards below GREEN, while **12** tests across `rk-kernel` and `rk-cli` failed.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell -- every guard below ends in `exit 1`.
#
# 🛑 HARDENED 2026-09-22, BEFORE ITS FIRST RUN, AND THE REASONS ARE RECORDED RATHER THAN SMOOTHED.
# As authored this block asserted THREE of the five bullets above: bullet 2 (a `tool` step reaches
# `rk-adapter-toolchain-rust`, the ADAPTER observed) and bullet 3 (`check` cannot dispatch) had no
# line at all. It also carried Checkpoint 7f's two defects -- no build step over a gitignored
# `target/`, and guards that captured `rc` and then tested only `rc` -- and only ONE of its four
# guards grepped a positive passed count, so the other three were satisfied by a filter matching
# NOTHING. Amending a checkpoint that has never been run is legitimate; doing it silently is not.
#
# 🛑 HARDENED A SECOND TIME, 2026-09-22, AND THIS BLOCK HAD RUN GREEN IN BETWEEN -- so the
# warrant above does NOT cover this amendment and is not stretched to. An adversarial pass measured
# that the five bullets were certified by six filters none of which entered the step walk (finding
# (12)) and that the `check` prohibition was asserted at the function but not at the arm that routes
# to it (finding (13)). Guard (7) is new; guards (5) and (6) gained `--test it`, without which their
# pass-count grep is satisfied by any target in the package; bullets 3 and 6 moved with them.
wt_before=$(git worktree list | grep -c .)

# 🛑 BUILD-OR-FAIL. `target/` is gitignored, 0 tracked files live under it, and Checkpoint 7f
# reached its end marker with rc=0 and NO PRODUCT BINARY on a fresh clone, because its only cargo
# lines were two `cargo test`. A checkpoint that asserts on built behaviour builds first.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the workspace does not build (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
bin=""
for cand in target/debug/redkiln target/debug/redkiln.exe; do [ -f "$cand" ] && bin="$cand"; done
[ -n "$bin" ] || { echo "FAIL: the build succeeded and no redkiln binary exists -- every assertion below would be about nothing"; exit 1; }

# (1) BULLET 1a: every `StepKind` has an executor arm.
out=$(cargo test -p rk-kernel --test it -- conformance_every_step_kind_has_an_executor 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the step-kind executor lint is red or absent (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched -- the lint was deleted rather than discharged"; exit 1; }

# (2) BULLET 1b: `ports.toolchain` has a non-test call site -- ADR-0134's equality form.
out=$(cargo test -p rk-kernel --test it -- conformance_every_port_has_a_call_site 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the port call-site lint is red (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched the port lint -- a filter that matches nothing exits 0"; exit 1; }

# (3) BULLET 2, WHICH HAD NO LINE: a `tool` step reaches `rk-adapter-toolchain-rust` through the
# port, asserted by the ADAPTER BEING OBSERVED. The two lints above are SOURCE lints: they pass on a
# `ports.toolchain` spelling existing and neither requires an adapter to have run. The test carries
# its own positive control -- the same step through the same walk with the fixture in the port
# SUCCEEDS, and with the real adapter over an absolute nonexistent program it FAILS.
out=$(cargo test -p rk-cli --test it -- a_tool_step_reaches_the_rust_adapter_and_not_the_fixture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: no tool step reaches the real toolchain adapter (rc=$rc) -- AC-473's clause has no mechanism"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched the adapter-observed assertion"; exit 1; }

# (4) BULLET 3, WHICH HAD NO LINE: `check` cannot dispatch -- asserted structurally over CODE LINES
# with comments stripped and string literals blanked, two-sided (no receiver AND no port named).
out=$(cargo test -p rk-kernel --test it -- the_check_arm_has_no_receiver_and_so_cannot_reach_a_port 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the check arm can reach a port (rc=$rc) -- design's parenthesis is a prohibition"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched the cannot-dispatch assertion"; exit 1; }

# (5) BULLET 4: AC-013's re-entry still holds -- criterion (3)'s asserted half.
out=$(cargo test -p rk-kernel --test it -- kill_and_resume_n_step 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: AC-013's re-entry broke under the step walk (rc=$rc) -- criterion (3) regressed"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched kill_and_resume_n_step -- the AC-013 guard asserted nothing"; exit 1; }

# (6) BULLET 5: the tier-1 projection did not move -- criterion (2).
out=$(cargo test -p rk-kernel --test it -- rebuild_differential 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the tier-1 projection moved under the step walk (rc=$rc) -- criterion (2) regressed"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched rebuild_differential -- criterion (2)'s guard asserted nothing"; exit 1; }

# (7) BULLET 6, ADDED 2026-09-22 AFTER THIS BLOCK HAD ALREADY RUN GREEN -- AND THAT IS THE FINDING.
# Measured in a throwaway `git clone --local` at `2ce5576`: a `panic!` on `run_activity_steps`'
# FIRST statement leaves guards (1)-(6) ALL GREEN. (1), (2) and (4) are SOURCE lints that execute
# no runtime; (3) calls `Runtime::execute_step` directly; (5) calls `Runtime::run_activity`
# directly; and (6) constructs no `Runtime` at all, so bullet 5 would pass identically if
# `runtime/step.rs` were DELETED. Measured on the same mutant: 14 tests across `rk-kernel` and
# `rk-cli` fail, and the six filters select around every one of them.
#
# ⚠️ THAT COUNT IS 12 AND THE FIRST READING SAID 14 -- the two extra were RED FOR A REASON
# THAT WAS NOT THE MUTANT. `git clone --local` checks out under `core.autocrlf=true`, which
# rewrote 121 of 143 `rk-kernel` fixture files LF->CRLF, moving a recorded `run_id` digest
# (`config::adr0083_...`) and breaking a fixture text anchor (`rigor::rigor_monotonicity`).
# Both FAIL in that clone at pristine HEAD with no mutant applied, and both PASS on this tree.
# `PROGRESS.md` row 37's "twelve" was right; the correction is recorded rather than smoothed,
# because a mutation cycle whose export is not byte-faithful reports kills it did not make.
#
# TWO LINES BECAUSE THERE ARE TWO ROUTES, not one assertion written twice. The first names every test
# module that calls `run_activity_steps` BY NAME; `ac432` reaches it through the composition root's
# `replay::resume`, which is its ONE production call site. A kernel-only line would be satisfied by a
# walk no production path enters, which is this stage's own defect one level up.
#
# ⚠️ CORRECTED THE SAME DAY, AND THE CORRECTION IS THE POINT. This comment read "`second_agent::` are
# the only two tests in the workspace that call `run_activity_steps` BY NAME" and the line filtered on
# `second_agent::` alone. That was TRUE when written and was made FALSE ninety minutes later BY THIS
# SAME PASS, which added `post_commit_failure::` (2 call sites, ADR-0144) and `gate_step::` (4,
# ADR-0145): the filter then selected 2 of 8. A guard whose scope is a fact about the tree goes stale
# the moment the tree moves, and the three filters are listed BY NAME so a fourth module is a visible
# omission rather than a silent one. Measured: `grep -rn 'run_activity_steps(' crates/ --include=*.rs`
# outside `src/` returns 8 lines over 3 modules.
#
# The greps are PER-TEST (`^test ... ... ok$`), not the `test result: ok. N passed` form the guards
# above use: with no `--test it` that form is satisfied by ANY target reporting ANY pass.
out=$(cargo test -p rk-kernel --test it -- second_agent:: post_commit_failure:: gate_step:: 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the step walk is not entered, or refuses wrongly (rc=$rc) -- these three modules are every test that calls run_activity_steps by name"; printf '%s\n' "$out" | tail -20; exit 1; }
for t in a_second_runnable_agent_step_refuses_and_the_commit_survives \
         a_step_failing_after_a_committed_dispatch_refuses_by_name_and_keeps_the_commit \
         a_gate_step_after_a_dispatch_writes_the_runtime_mark_and_keeps_the_commit; do
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: $t did not run and pass -- a filter that matches nothing exits 0, and one module dropping out would otherwise be absorbed by the other two"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
out=$(cargo test -p rk-cli --test it -- explain::ac432_explain_over_replayed_log_is_byte_identical 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the PRODUCTION route into the walk is red (rc=$rc) -- replay::resume no longer reaches run_activity_steps, or reaches it and breaks"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\bac432_explain_over_replayed_log_is_byte_identical\b.* \.\.\. ok$' || { echo "FAIL: ac432 did not run and pass -- the composition-root route into the walk is asserted by nothing"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# 🛑 DISARM THE HAZARD THIS BLOCK ARMS. `rk-cli`'s process-level tests spawn `redkiln run`, which
# creates run-owned worktrees. A block that leaves registrations behind double-counts every later
# repo-root grep against a stale second checkout -- measured at `s7f`, `TrailerKey::` returned 38
# from the repo root against 19 from `crates/`.
git worktree prune
wt_after=$(git worktree list | grep -c .)
[ "$wt_after" -le "$wt_before" ] || { echo "FAIL: worktree registrations grew $wt_before -> $wt_after and the prune did not clear them"; git worktree list; exit 1; }
echo "CP7G-END-OF-BLOCK reached"
```

---
