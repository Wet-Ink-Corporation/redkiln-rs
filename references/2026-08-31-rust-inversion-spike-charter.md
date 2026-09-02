# Spike Charter — Rust Inversion

**Version:** 1.0 (final). Supersedes the 2026-08-31 draft.
**Date:** 2026-09-01
**Companion:** `references/2026-08-31-rust-inversion-RFC.md` — the design analysis this programme tests. This charter stands alone; the RFC adds depth.
**Status:** Ready to schedule. Section 15 records the decisions already taken.

---

## 1. Vision

Redkiln today is a deterministic state machine that lives *inside* a Claude Code session. The human types `/redkiln:implement`, Claude Code is the process, and `redkiln` is a tool that process calls. Every rule about ordering, gating and state — "you own every `redkiln advance`", "never hand-edit frontmatter", "always pass `--stay`" — is prose in a markdown file that a model must read and obey correctly on every single run.

The proposition under test is an **inversion**: Redkiln becomes the process, and Claude Code becomes a subprocess it spawns per step.

Four things follow from that, and all four are why this is worth measuring:

**Enforcement becomes structural.** Today `commands/implement.md` warns that omitting `--stay` walks a project irreversibly past its own review gate, into a state where `validate` and `doctor` both report clean and no CLI verb can move it back. That is a silent, unrecoverable corruption guarded by a paragraph. If the orchestrator owns the loop, the agent is never offered the option — the failure becomes unrepresentable rather than merely discouraged.

**Cost becomes visible and controllable.** We currently record output tokens and nothing else: no input tokens, no cache reads, no wall clock. Our own tier policy concedes that prompt-cache hit rate is a ~12.5× lever it does not model, and the telemetry cannot see it either. Inversion puts every dispatch behind an instrument that reports full usage, and makes budget a hard stop rather than a recorded number.

**Runs become unattended.** An initiative that can only advance when a human is sitting in a terminal session cannot run overnight, on a schedule, or from CI. Owning the loop is the precondition for any of that.

**Portability becomes possible.** Every harness now ships a non-interactive mode. None of them ships a mid-run human approval gate — the universal advice is "make each stage its own invocation", which means something outside must own the stage boundary and the resume. That thing is what Redkiln already is. But it can only drive a second harness if it holds the loop.

**What this is not.** It is not a rewrite for its own sake, and it is not a bet that more orchestration is better. Part of what we are testing is whether the orchestration we have is worth carrying at all.

## 2. The decision this serves

Two decisions, in order:

1. **Should Redkiln invert control?** Gated on whether agent quality survives the move from a warm in-session context to explicit headless payload, and on whether the cost model actually improves.
2. **If so, which orchestration patterns should be carried across?** Gated on whether each pattern beats a deliberately simple baseline.

The second question is not decoration. The published evidence is genuinely hostile to elaborate orchestration:

- **Agentless** — three fixed phases, no agent autonomy at all — scored 32.00% at $0.70 per instance on SWE-bench Lite, against CodeR's multi-agent 28.33% at $3.34. Its single largest ablation was a reproduction-test filter worth **+15 issues for $0.25**.
- **Kim et al.** (260 configurations, 5 architectures, 6 benchmarks, compute-matched) found no architecture dominates, with effects from **+80.8% to −70.0%**, and that "coordination yields diminishing returns once single-agent baselines exceed certain performance."
- A 2026 study of parallel coding agents found **naive fan-out worse than a single agent on quality on both benchmarks**, while 2–4× faster.
- Agent OS v3.0.0 retired its orchestration phases outright: *"frontier models handle this well on their own now."*

There is exactly one structural property with convergent, compute-matched, multi-source support: **verification-gated loops** — grounding on tests and CI rather than model judgement. That is already Redkiln's core. The programme is designed to protect it and interrogate everything around it.

## 3. Why measurement rather than a prototype

A prototype answers *can we build it*; we already know we can. A spike answers *should we*, which requires stated hypotheses and pre-declared kill criteria — otherwise a working prototype becomes the argument by itself.

Three facts make measurement unavoidable:

- **No published benchmark exists** for in-session subagent dispatch versus `claude -p` per agent. We cannot read our way to the answer.
- **Our telemetry cannot see the deciding variable.** 101 recorded runs, 923 dispatches, 42.4M output tokens over 21 days — and no input, cache, or latency data anywhere.
- **Anthropic does not publish the metering function.** Neither the unit of a usage window nor the weighting of input, output and cache-read tokens toward it is documented. Consumption must be derived empirically.

## 4. Non-goals

- **Not building the orchestrator.** The spike stops at a step runner and a plan format.
- **Not porting the engine** (`src/`, ~40k LOC). That decision is downstream and separately gated in the RFC.
- **Not deciding multi-harness.** One harness — `claude`. Portability is a motivation, not an experiment.
- **Not improving Redkiln.** Findings about the current system are written down, not implemented.

## 5. Principles

1. **Isolation over reuse.** Shipping optimises for cost; experiments optimise for control. No `redkiln` CLI, no process packs, no gate engine, no telemetry sink and no `workflow-harness.ts` inside a measured run.
2. **Inherit the corpus, not the machinery.** Data in, code out.
3. **Nothing the experiment writes may touch the tree under test.** Claude Code's system prompt captures branch and recent commits; sequential runs share a cache prefix only when the git snapshot matches. Redkiln commits telemetry during a run — which is precisely how the current system would corrupt a cache measurement.
4. **Arms must be commensurable.** Every arm, including the baseline, is a plan the same runner executes.
5. **Pre-register.** Prompts, arms and the analysis plan are written and committed *before* the arm runs.
6. **Measure variance before sizing anything.**

---

## 6. Platform constraints that shape the design

These come from Anthropic's documentation and materially change what we build. They are stated here because getting any of them wrong invalidates results.

### 6.1 We are rate-limited, not token-billed

Usage runs against a Claude Max subscription: a rolling **5-hour** window, a **7-day** window, and separate **per-model-family sub-limits** for Opus and Sonnet. The session and weekly windows are shared across models; the family sub-limits are not. Documented warning, verbatim: *"A single burst of heavy activity, such as a large workflow fanout, can exhaust the weekly allowance before the session window resets."*

**The Opus sub-limit is the real budget, and it is invisible.** The programmatic headroom read — the statusline JSON `rate_limits` object — exposes `five_hour`, `seven_day` and `spend_limit`, each with `used_percentage` and `resets_at`. It exposes **no Opus or Sonnet key**. The sub-limits are enforced (you get "You've hit your Opus limit") but cannot be read before you hit them. Our review arms run Opus by design, so we self-throttle.

**Anthropic does not publish the metering unit.** It is clearly input-weighted — cached re-reads still draw down the allowance — and cache reads count at roughly a tenth of the input rate. Output weighting is undocumented. The published prompt-count figures still reference Sonnet 4 and Opus 4 and should not be used for planning.

### 6.2 Three traps

**`--bare` does not use the subscription.** It requires `ANTHROPIC_API_KEY` and bills per token. It is otherwise attractive — it skips hooks, skills, commands, subagents, plugins, MCP and CLAUDE.md discovery, shrinking the prefix, and is documented as becoming the `-p` default in a future release. **We do not use it, and the harness asserts `ANTHROPIC_API_KEY` is absent before every run.** A stray key silently converts a rate-limited run into a billed one.

**Crossing into usage credits silently halves cache value.** The one-hour prompt-cache TTL applies *"only on a Claude subscription within your plan's included usage."* Cross into credits and the main conversation drops to five minutes — so the cache hit rate collapses exactly when consumption is already high. Mitigation: set `promptCacheTtl` / `CLAUDE_CODE_PROMPT_CACHE_TTL` to `1h` (requires v2.1.242+), which holds the hour even on credits.

**Worktrees fragment the cache.** Verbatim: *"two sessions in different directories build different prefixes and miss each other's cache. That includes worktrees of the same repository, since each worktree has its own working directory."* And: *"Sessions you run in parallel in the same directory build matching prefixes and read each other's cache."*

This is a direct conflict with Redkiln's architecture, which puts every initiative and every fix in its own worktree. It is simultaneously a **finding about the current system** and a **constraint on the target design**, and E1 tests it explicitly.

### 6.3 Headless has no safety net

In `-p` there is no wait-and-continue and no auto-retry on a plan limit — Claude Code blocks further requests until the reset time. There is **no documented exit code or structured error subtype** for usage-limit exhaustion; `resets_at` from `rate_limits` is the practical retry-after. Transient throttles do retry, emitting `system/api_retry` events with `attempt`, `max_retries`, `retry_delay_ms` on `--output-format stream-json` — the harness consumes these.

### 6.4 Version floor

Pin **`claude` ≥ v2.1.251** and record the exact version on every run. Rationale: subagent caps need v2.1.219+, `promptCacheTtl` needs v2.1.242+, and the `rate_limits.spend_limit` and `prompt_cache` statusline objects need v2.1.251+. The `prompt_cache` object (`warm`, `hit_ratio`, `ttl`, `expires_at`, miss counts) is our cleanest cache instrument — main conversation only, subagents excluded.

---

## 7. What gets built — Layer 0

Four pieces, in Rust. **One week, hard stop.** A runner that is not working at the end of week one is itself a finding about the approach.

### 7.1 The step runner (~400 LOC)

| Requirement | Acceptance |
| --- | --- |
| Spawn `claude -p --output-format stream-json` with `--model`, `--agents <json>`, `--cwd`, `--json-schema`, `--allowed-tools`, `--permission-mode` per step | Nothing relies on `.claude/` discovery — everything passed explicitly |
| Assert environment hygiene before every run | `ANTHROPIC_API_KEY` absent; `--bare` never passed; `claude --version` ≥ floor and recorded |
| Parse the NDJSON event stream | Every event typed or explicitly captured; unknown events logged, never dropped |
| Capture usage | `input_tokens`, `output_tokens`, `cache_read_input_tokens`, `cache_creation_input_tokens`, the `ephemeral_1h` / `ephemeral_5m` split, `total_cost_usd`, wall clock |
| Capture headroom | `rate_limits.five_hour` and `.seven_day` `used_percentage` + `resets_at` before and after every step |
| Validate the digest | `serde` against the step schema |
| Distinguish "said no" from "never spoke" | `Result<Digest, StepError>` with `StepError::{Transport, Schema, Timeout, Cancelled, NonZero, UsageLimit}`. This is RK-B0013 as a type. |
| Cancel and timeout | Process-group kill, terminal settlement before return |
| Throttle | Gate dispatch on `used_percentage` against a configured margin; `--max-budget-usd` per step as a secondary ceiling |
| Journal | Append-only, out-of-tree, one line per attempt: full spec, usage, headroom delta, outcome |

**Dependency decision.** No Claude-driving crate is depended upon — every candidate is 0.x, single-author, under 25 stars, and the two most featureful are months stale. Take `claude-codes`' *types* layer (Apache-2.0) if its protocol structs are sound; own spawn, supervise and cancel ourselves. Adopt its version discipline regardless.

**Workflow-crate decision.** No Rust durable-execution framework fits, and the reason is structural rather than incidental: Temporal, Restate and Obelisk all sell *deterministic replay of arbitrary user code*, which is why each mandates a server — arbitrating the journal **is** the product. We do not need replay. Our steps are coarse, side-effecting, minutes-long subprocesses. We need a memoisation table keyed by step identity, which is a much smaller problem. (`temporal-sdk-core` on crates.io is a 2021 alpha placeholder; the live SDK is `temporalio-*`. The `dbos` crate is 12 lines of Rust with 39 downloads. `graph-flow` has exactly the right shape — a pluggable `SessionStorage` trait, one-step-then-persist runner, native suspend/resume — but hard-depends on `sqlx`+postgres and is one person's side project. Read its `storage.rs` and `runner.rs`, then write our own.) Optionally adopt `statig` or `rust-fsm` for the per-step lifecycle so illegal transitions are a compile error.

### 7.2 The plan format

A run is a list of steps. **Arms are files, not code.**

```toml
[[step]]
name       = "review"
occurrence = 1
model      = "opus"
effort     = "xhigh"
agent      = "reviewer"          # → --agents JSON
prompt     = "prompts/review.md"
schema     = "schemas/review-digest.json"
fanout     = 1                   # >1 = N parallel instances
stagger_ms = 5000                # cache-prefix stagger; 0 disables
join       = "none"              # none | merge-step | union
gate       = "none"              # none | tests | schema
```

Fan-out, sequencing and joins are data, so orchestration shapes can be tested before any orchestrator exists. Concurrency needed is small — the real `forge-implement.js` contains exactly one `parallel()`, of width 2.

**The stagger is a first-class parameter** because Claude Code's own workflow runtime holds all but the first agent in a fan-out until the first response begins, so the rest read the cached prefix rather than each paying a cold write. Fire N at once and we pay N cold writes. Making it a parameter lets E7 separate a pattern's quality effect from its cache effect.

### 7.3 The fixture rig

Per run: `git init`, seed from the corpus, a real test command, reset to a known commit between runs. **No `.redkiln/`.** The gate is "run the test command, read the exit code" — about twenty lines.

Reset is verified, not assumed: assert a clean tree and expected HEAD before each arm, or the run is void.

**Fixtures live in one directory per experiment, not one per run** — see §6.2. Per-run worktrees would fragment the prompt cache and confound every cost measurement.

### 7.4 The results store

Append-only JSONL **outside every fixture repo**. One record per step attempt, one per run: arm, run id, seed, step name/occurrence/attempt, model, effort, all usage fields, headroom before/after, wall clock, outcome, gate result, digest hash, `claude` version.

---

## 8. The corpus — inputs only

Scope is the **fix and review lanes**. The relevant inputs:

| Asset | Count | Use |
| --- | --- | --- |
| `_review.md` with resolving checkpoint SHAs | **18 of 38** | Review inputs — `git show <sha>` gives the exact diff each rubric judged |
| Incidents + fix-lane bugs | ~20 | TDD-implementation inputs, each with a spec and a known-good historical fix |
| `_ledger.md` rows | 1,249 across 110 files, each with `evidence` + `mount_point` + `verifying_test` | Requirement → implementation → test triples; the verifying tests are our outcome oracle |
| `implementation-report.md` | 156, all with `## TDD Evidence` / `## Changes` / `## Gates` | Implementation ground truth |
| `workflow_run` events | 101 runs, 923 dispatches, 42.4M output tokens | Historical cost baseline — **output tokens only** |

**Deferred, not discarded:** the 153 `spec.md` (1,737 ACs) and the 15 storymap/decomposition targets. Planning-artifact and fan-out-decomposition experiments come back if the gates in §11 pass and headroom allows.

### 8.1 The constraint that shapes every metric

**The labels are censored at the gate.** Verdicts run **215 pass / 2 fail**. `_review.md` `overall` is **35 of 36 scored 3**. `_ledger.md` is **1,249 of 1,249 `satisfied: true`**. This is structural — the process cannot advance an item until it passes, so failures are absent from the artifacts by construction.

**Therefore rubric score is never an outcome variable.** The negative signal lives in exactly three places: `run.result` (**57% of 101 runs halted, across 14 distinct halt classes**), `gate_result` failures with their diagnostics, and prose (`## Required Changes`, `run.blockedAt.summary`). Every metric below is built on halt class, gate diagnostics, prose, or deterministic test outcomes.

### 8.2 Pre-registration instead of a holdout

With 18 review items and ~20 bugs, a 60/40 split would leave 7 held-out reviews — too few to support any conclusion. Spending scarce items on a holdout costs more than it buys.

**Instead: pre-register.** Before any arm runs, commit the prompts, the arm definitions, the primary and secondary metrics, and the analysis plan, and record the commit SHA in the results file. Any change after the first run is a new pre-registration and is reported as such. This is the honest discipline at this n; a holdout is not.

---

## 9. Method

### 9.1 Power

From KTH's work on randomness in agentic evals (two-sample, 80% power, p<0.05, σ≈1.5%):

| Effect to detect | runs per arm |
| --- | --- |
| 1 pp | 36 |
| 2 pp | 9 |
| 5 pp | 2 |
| 10 pp | 1 |

**Those σ values are theirs, not ours.** E0 measures ours before anything else is sized.

### 9.2 Determinism is not available

Temperature 0 is not deterministic: 1,000 completions at temp 0 from one prompt produced **80 unique outputs**, because batch size varies with server load and changes kernel reduction order. Single runs prove nothing. Use paired designs on identical inputs; report distributions, never point estimates.

### 9.3 Judging

Deterministic outcomes are preferred wherever available — tests pass, gate green, AC row satisfiable by its recorded verifying test.

Where quality genuinely needs judging, the arrangement is **human annotation plus a calibrated LLM judge**:

1. Ryan annotates a calibration set of ≥30 items — for each historical `## Required Changes` finding, did this arm surface it? Blind: arm labels stripped, presentation order randomised.
2. An LLM judge is calibrated against those annotations and its agreement rate computed.
3. The judge scales to the remaining items. **Its agreement rate is reported alongside every result it produces.**

The known ceiling is stated in every write-up: the best fully-automatic LLM-judge configuration measured 62.5% agreement / κ=0.37, still ~9pp and 0.17κ below a single average human annotator. Where a call is close, it is recorded as a tie rather than resolved in our favour.

### 9.4 Confounds controlled explicitly

Model and CLI version (pinned, recorded per run; the baseline arm re-runs if either changes mid-experiment), git snapshot state, cache warmth (recorded, never assumed), fixture directory identity, and time of day.

**Model tier is never a free variable.** Each experiment pins the tier used in production for that step and holds it constant across arms, unless tier is the stated variable.

---

## 10. Experiments

### E0 — Variance pilot
**Question.** What is σ for our metrics on our tasks, and what does one run cost in plan headroom?
**Arms.** One — the baseline reviewer configuration.
**Inputs.** 10 of the 18 SHA-resolving reviews.
**Procedure.** Run the identical arm 10×. Compute σ for blocker recall, false-positive rate, output tokens, input tokens, cache-read share, wall clock — **and mean `seven_day.used_percentage` delta per run.**
**Output.** A sizing table for every experiment below, and the first real answer to "what does this cost me."
**Kill criterion.** If σ requires >20 runs/arm to detect a 10pp effect, E4/E6/E7 are unaffordable and we run mechanism only.
**Blocks.** Everything.

---

### E1 — Cache economics
**Question.** Do sequential `claude -p` runs share a prompt-cache prefix when the workflow commits between them, and does either available mitigation restore sharing if not?
**Why it decides something.** The cost case for inversion rests on this. Docs state `-p` runs in one directory read each other's cache at up to 1h TTL, while in-session subagents get their own unshared prefix at a forced 5m — but also that sequential sessions share *"only when the git status snapshot at startup matches."* Redkiln commits at every checkpoint.
**H1.** Cache-read share stays ≥70% across 10 sequential runs with a commit between each, under at least one mitigation.
**H0.** Share collapses to ~0 — a cold write every run — regardless.
**Arms.** (a) sequential, no commits, one directory; (b) sequential, commit between each; (c) as (b) plus `--exclude-dynamic-system-prompt-sections`; (d) **as (b) but each run in its own git worktree** — the current Redkiln topology.
**Metric.** `cache_read / (cache_read + cache_creation)`; the 1h vs 5m split; `seven_day.used_percentage` delta per run.
**n.** 10 per arm. Mechanism check, not a quality comparison.
**Kill criterion.** If (c) fails to restore sharing, the cache benefit is unavailable to a commit-per-checkpoint workflow. Inversion's cost case reduces to "avoids transcript accumulation" alone — still real, but the headline argument is withdrawn and restated before proceeding.
**Expected secondary finding.** Arm (d) should quantify what per-worktree isolation costs us *today*. That is a finding about the current product regardless of what we decide about Rust.
**Cost.** Trivial — a one-line prompt suffices. Half a day.

---

### E2 — Effort pinning
**Question.** Can reasoning effort be set per dispatch headlessly?
**Why.** `--agents` JSON has no `effort` field. `tiers.mjs`: an absent effort is inherit-from-session, *"so before this column existed every redkiln step ran at whatever depth the invoking session happened to sit at: the cost and quality profile of a run was a property of the CALLER, not of redkiln."* If effort cannot be pinned, that defect returns.
**Arms.** `--effort` on the parent × {absent, low, high, xhigh}, on a task whose output length is effort-sensitive.
**Metric.** Output-token and reasoning-token distribution by requested effort. If they do not separate, effort is not being applied.
**n.** 5 per cell.
**Outcome.** Not a kill — a **constraint**. If effort cannot be pinned per dispatch, record it as a permanent architectural limitation and re-price the tier policy.
**Cost.** Trivial. Half a day.

---

### E3 — Context fidelity ← **GATE**
**Question.** Does an adversarial reviewer produce the same verdicts headless as in-session?
**Why.** `agent()` today dispatches into a live session and inherits ambient context — the transcript, the agent cards, `model: inherit`, a git snapshot from parent session start. Headless, every spawn is cold and all of it becomes explicit payload. Our prompts were written against the warm case. **If verdicts degrade, no amount of Rust fixes it.**
**Arms.** (a) in-session `Task({subagent_type: "redkiln:redkiln-review"})`; (b) `claude -p --agents '<card as JSON>' --model opus --effort xhigh --json-schema REVIEW_DIGEST`; (c) as (b) plus explicitly-passed context — repo conventions, the spec, the AC list.
**Inputs.** All 18 SHA-resolving reviews. `git show <sha>` gives the exact diff.
**Primary metric.** **Blocker recall** — of the findings in the historical `## Required Changes`, what fraction does each arm surface?
**Co-primary.** **False-positive rate.** A reviewer tuned toward zero false positives measured FPR 0.20% / FNR 87.9% in the literature — it rejected 88% of real defects. Reporting recall alone is meaningless.
**Secondary.** Verdict agreement, cost, headroom draw, wall clock.
**n.** From E0; provisionally 3 repeats × 18 items × 3 arms.
**Kill criterion.** **If (b) and (c) both show materially lower blocker recall than (a) — a drop the E0-sized test can detect — the inversion thesis fails, the RFC recommendation is withdrawn, and the programme stops.**
**Note.** This arm set is Opus-heavy and the Opus sub-limit is not programmatically visible. Spread across days; watch `seven_day` and stop on the family limit rather than planning around it.

---

### E4 — The null arm ← **GATE**
**Question.** Does a single agent with one pass and a test filter match multi-phase orchestration on the fix lane?
**Why.** See §2. This is how we find out whether we are defending real value, and it is the cheapest way to find out.
**Arms.** (a) the transcribed current `forge-fix.js` topology — preflight → red → green → review → gate → commit; (b) single agent, one pass, TDD instruction, tests as the only gate; (c) as (b) plus sample-3-and-select-by-tests.
**Inputs.** The ~20 real incidents and fix-lane bugs, each with a spec and a known-good historical fix.
**Primary metric.** Tests green **and** the AC ledger rows satisfiable, verified by running the historical `verifying_test` for each row.
**Secondary.** Cost, headroom draw, wall clock, halt class.
**n.** From E0.
**Decision rule.** If (b) or (c) matches (a) within the detectable margin at materially lower cost, **the orchestration worth porting is smaller than the one we have.** That lowers the RFC's effort estimate and raises the bar for every remaining pattern. Run this before defending any orchestration structure.
**Baseline construction.** Arm (a) is hand-transcribed from the static text of `forge-fix.js` — `meta.phases`, its literal dispatch sites, its prompt bodies — into the plan format. It is not driven by the existing harness. Both arms must be plans the same runner executes or they are not comparable.

---

### E5 — Fix-loop depth
**Question.** Is one fix round better than three?
**Why.** Reflexion-style self-correction depends entirely on test quality — the gain runs from 91% down to *below baseline* as test false-positive rate moves between 1.4% and 16.3%. Huang et al. found intrinsic self-correction without an oracle drops accuracy on every model and benchmark tested. Self-Debugging attributes 54–100% of its gain to test execution rather than reflection.
**Arms.** (a) 3 fix rounds — current; (b) 1 fix round; (c) 1 fix round plus 2 fresh samples selected by tests, at equal total compute to (a).
**Inputs.** As E4.
**Primary metric.** Tests green at equal compute.
**Secondary — and required.** **Our test suite's false-positive rate.** Inject N known-bad patches and count how many the affected-area gate passes. This number determines whether any fix loop can work, and **we currently do not know it.** It is cheap, deterministic, costs no model tokens, and should be measured first.
**Decision rule.** If (c) ≥ (a) at equal compute, bounded-retry-with-fresh-samples replaces the fix loop in the ported design.

---

### E6 — Reviewer configuration
**Question.** Does a fresh reviewer beat self-review, and does a panel beat one reviewer?
**Why.** Cross-context review measured 28.6% F1 versus self-review-with-full-history 24.6% (paired t-test, N=30, p=0.008, d=0.52) — so "reviewers can't grade their own homework" is earned. But absolute F1 was 28.6% *for the winner*, which suggests our reviewer is weaker than we assume.
**Arms.** (a) fresh reviewer, no author context — current; (b) author self-reviews with full history; (c) fresh reviewer given the author's generation prompt; (d) panel of 3 fresh reviewers, union of blockers.
**Inputs.** The 18 SHA-resolving reviews, as E3.
**Primary metric.** Blocker recall; FPR as co-primary.
**Decision rule.** Confirms or overturns a rule currently enforced by prose. Arm (d) prices whether a panel is worth 3× the cost.
**Bonus.** Arm (d) is a fan-out-of-3 with a union join — so it doubles as a cheap first read on whether fan-out/fan-in earns its cost, without funding the full decomposition experiment.
**Runs after E3**, on the same rig and inputs.

---

## 11. Sequencing and gates

```
Week 1        Layer 0 — runner, plan format, fixture rig, results store   [hard stop]
Week 2        E0 variance pilot → sizing + headroom-per-run table
              E1 cache · E2 effort        (cheap, mechanism, run in parallel)
Week 3–4      E3 context fidelity                              ← GATE
Week 5        E4 null arm                                      ← GATE
Week 6        E5 fix depth  (test-suite FPR measured first, no model cost)
Week 7        E6 reviewer configuration
Week 8        Transfer check + write-up
```

**Gate after E3.** Fail ⇒ stop, write up, withdraw the RFC recommendation.
**Gate after E4.** If the null arm wins, re-scope: the question stops being "port this orchestration" and becomes "how much orchestration is worth keeping at all."

**The transfer check (week 8).** Every pattern that wins in a clean fixture must be shown to still win inside a real Redkiln repo, with real gates, locks, worktrees and commits. A result proven only in the lab is how a finding fails to ship — and given §6.2, the worktree topology alone may change the answer. This is a validation step, not an arm.

---

## 12. Budget, in the units that actually bind

**Estimated volume.** Roughly **460 runs / ~38M output tokens** across E0–E6, dominated by E3 (~160 review-scale runs at ≈68k output each) and E4/E5 (~180 fix-scale runs at ≈127k each).

For scale: the historical ledger records **101 runs / 42.4M output tokens over 21 days** of normal Redkiln work. So the whole programme is **comparable in output volume to three weeks of normal use, spread across eight weeks** — very roughly a third of a normal week's Redkiln draw, added on top of normal use, each week.

**Treat that as an order of magnitude, not a forecast.** Anthropic publishes neither the metering unit nor the input/output/cache weighting, our historical ledger has no input-token data, and input volume likely dominates. E0 replaces this estimate with a measured `seven_day.used_percentage` delta per run in week 2, and every later experiment is sized against that number rather than this one.

**Management, not prediction:**

- The runner reads `rate_limits` before and after every step and **refuses to dispatch** below a configured margin.
- Every experiment declares a headroom ceiling; hitting it pauses the experiment rather than the week.
- Opus-heavy arms (E3, E6) are spread across days, because the Opus family sub-limit is enforced but not readable.
- `--max-budget-usd` is set per step as a secondary ceiling. On a subscription it functions as a notional list-price proxy rather than a real spend cap, but it is the only documented per-invocation limit that counts subagent work. **Verify this empirically in E0** — it is inference, not documented.
- **Usage credits stay disabled** unless deliberately enabled, because crossing into them silently drops the cache TTL from 1h to 5m and compounds consumption. If they are enabled, `promptCacheTtl=1h` is set.

**Stop conditions, pre-declared:**

- E3 fails ⇒ stop, write up, withdraw the recommendation.
- E0 shows σ requiring >20 runs/arm for a 10pp effect ⇒ drop E4–E6, ship mechanism findings only.
- The `claude` CLI ships a breaking change to the headless surface mid-programme ⇒ pause, re-baseline, record as evidence for the version-coupling risk.
- Layer 0 slips past one week ⇒ escalate rather than absorb.
- Weekly headroom consumed by the programme exceeds the declared share two weeks running ⇒ pause and re-plan.

**The failure mode to guard against.** A Rust runner plus a plan format is roughly 60% of a working orchestrator. The temptation will be to keep building and let the spike quietly become the product without anyone taking a decision. The gates are the countermeasure, and they only work if they are honoured when inconvenient.

---

## 13. Threats to validity

Stated up front so the write-up cannot quietly omit them.

1. **Single repo, single operator, single domain.** Everything is measured on a TypeScript CLI built by one person. Transfer to other codebases is unevidenced.
2. **The corpus was produced by the incumbent system.** "Matches the historical artifact" rewards mimicry, not quality. This is why E3 and E6 use blocker recall against prose rather than similarity to the historical review.
3. **No negative labels.** §8.1. Every metric rests on halt class, gate diagnostics, prose, or deterministic test outcomes.
4. **Model and CLI drift.** An update mid-programme invalidates cross-week comparison. Pin, record, re-baseline.
5. **We are judging our own system.** Blind, randomised order, ties recorded as ties, LLM-judge agreement rate reported alongside every judged result.
6. **E1 measures platform behaviour we do not control** and Anthropic may change without notice. Date-stamp the result.
7. **The Opus sub-limit is invisible**, so headroom management is partly blind. Expect at least one experiment to be interrupted by it.
8. **n is small.** 18 reviews and ~20 bugs. Pre-registration mitigates fishing; it does not manufacture power. Only large effects will be detectable, and we say so rather than reporting a near-miss as a trend.

---

## 14. Deliverables

1. The runner, plan format, fixture rig and results store, in a `spikes/` tree — **not** a separate repo, **not** merged into product code.
2. One results file per experiment: pre-registered hypothesis and its commit SHA, arms, n, raw records, distributions, and the decision the criterion produced.
3. A dated scorecard extending `references/conformance/` — a third column alongside the 2026-08-08 and 2026-08-09 scorecards, using the same PASS/PARTIAL/FAIL discipline and the same rule that a PASS without a cited evidence path is a defect in the scorecard.
4. An amendment to the RFC recording which claims survived, which were falsified, and which remain untested.
5. A findings note for anything discovered about the current system that warrants a product change — written down, not implemented. **Two are already expected**: the worktree/cache interaction (§6.2) and the unmeasured test-suite false-positive rate (E5).

---

## 15. Decisions taken

| Question | Decision |
| --- | --- |
| **Intent** | All four drivers are in scope: enforcement, cost, unattended operation, multi-harness reach. They pull in slightly different directions — multi-harness argues for a thin portable runner, enforcement for a rich engine — and §11's gates are ordered so the shared prerequisites are tested first. |
| **Budget** | Rate-limited on Claude Max, not token-billed. Managed by headroom polling and per-experiment ceilings rather than a dollar cap. Estimate in §12; replaced by measurement in week 2. |
| **Judging** | Ryan annotates a ≥30-item calibration set; an LLM judge is calibrated against it and scales, with its agreement rate reported on every result it produces. |
| **Scope** | Fix and review lanes only. Planning-artifact and fan-out-decomposition experiments deferred; E6 arm (d) gives a cheap first read on fan-out. |
| **Corpus split** | No holdout — pre-registration instead. n is too small to spend items on a holdout (§8.2). |
| **Model tiers** | Pinned per experiment to the tier used in production for that step, constant across arms. Tier is never a free variable. |
| **`redkiln-design-review`** | Out of scope. It needs Bash, screenshot capture and vision on the resulting PNGs; its headless fidelity is untested and rigging it would likely cost a week. Recorded as an unmeasured risk. |
