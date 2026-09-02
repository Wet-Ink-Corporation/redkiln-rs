# Pre-registration — Stage 0, E0, E1, E2

**Pre-registration baseline:** commit `363ab2a` on `spike/rust-inversion-layer0`.
**Corpus pin:** redkiln @ `47faec9f` (frozen for the duration).
**Instrument:** `spikes/runner` — `claude` 2.1.258, Windows 11.

Charter §8.2: prompts, arms, metrics and the analysis plan are committed
*before* the first arm runs, and any change afterwards is a **new**
pre-registration reported as such. This document is that registration for the
experiments that come before the E3 gate. E3–E6 are registered separately, after
E0 sizes them.

Everything here reflects the corrections in `PRIOR-ART.md` §9.

---

## 0. Rules that apply to every arm

1. **Isolation is on.** `--strict-mcp-config` by default, and `isolation_ok` is
   read back from the child's own `system/init`. A record with
   `isolation_ok == false` is void, not merely noted (FINDINGS F8).
2. **Environment hygiene is asserted per dispatch**: `ANTHROPIC_API_KEY` absent,
   `--bare` never passed, `claude --version` recorded.
3. **Confounds are recorded, and a change in any of them splits the series**:
   `service_tier`, `speed`, `fast_mode_state`, `init_digest`, `claude_version`,
   and the fixture's `status_digest`.
4. **A run that crossed into usage credits is excluded**, not averaged in —
   `is_using_overage == true` changes the cache TTL and therefore what the
   record means (charter §6.2).
5. **Headroom is reported as a batch delta ÷ n, never per dispatch.**
   `utilization` is quantised to 0.01 (FINDINGS F6); a batch that moved it less
   than two ticks is reported `resolution_limited`, which means "below the
   instrument", not "free".
6. **Distributions, never point estimates.** Single runs prove nothing
   (charter §9.2).

---

## S0.1 — Gate false-positive rate  *(zero model cost)*

**Question.** How often does `redkiln verify --grain story` pass a change that
is known to be bad?

**Why it comes first.** It is the oracle E4 and E5 both rest on. If the gate
passes known-bad patches at a material rate, E4's decision rule is unsound and
its result is uninterpretable.

**This is mutation testing** (PRIOR-ART §5), so it uses that methodology:

- **Mutation source, preferred:** take each of the 23 fix-lane bugs, revert the
  fix, keep its test. The accompanying test *proves non-equivalence*, which
  removes the equivalent-mutant problem by construction.
- **Denominator:** `killed / (total − equivalent)`. Equivalent mutants cannot be
  detected and are not evidence of a weak gate; counting them would inflate the
  FPR and propagate the bias into E4 and E5.
- Any synthetic mutants beyond the 23 get manual equivalence triage on
  survivors. Prefer existing tooling (Stryker for the TS tree) over hand-rolled
  operators.

**Instrument.** `redkiln verify --item <id> --grain story --json` with
`REDKILN_NO_GATE_CACHE=1`. Exit is three-valued — **0 pass / 1 the change is bad
/ 2 could not tell** — and `flaked` is recorded separately, since it is
non-blocking and never memoised.

**Primary metric.** Survival rate: mutants the gate returns 0 for.
**Secondary.** Rate of exit 2 (could not tell), and `flaked` rate.

**Cost note.** No model tokens. But the gate runs the **whole 236-file vitest
suite** every time — there is no affected-area selection — so budget wall clock,
not headroom.

**Reported regardless of outcome.** A low FPR validates the oracle; a high one
is a finding about the current product (charter §14.5) and forces E4's oracle to
be reconsidered before it runs.

---

## S0.2 — Deterministic reconcile replay  *(zero model cost)*

**Question.** Can a deterministic `reconcile(recorded, world) -> (State,
Vec<Drift>)` reproduce the resume decisions the Preflight agent made?

**Why.** RFC §3.4 calls the Preflight agent's git archaeology "the single
highest-value thing to port" because it removes an LLM from the resume decision;
shipmates L2.2 independently reaches the same design. Nothing tests it, and it
is free.

**Inputs.** 232 `Story:` and 104 `Slice-Verdict:` trailers across all refs at the
pinned SHA, plus the 124 recorded `workflow_run` events.

**Primary metric.** Agreement with the recorded outcome per run, and an
enumeration of every divergence with its cause.

**Decision rule.** Divergences that are the deterministic implementation being
*right* are evidence for the port. Divergences where it is wrong bound how much
of the resume decision can actually be de-LLM'd, and that bound goes in the RFC
amendment.

---

## S0.3 — Corpus extraction  *(zero model cost)*

Normalise the frozen corpus to JSONL. Not an experiment; a dependency.

- Parse the **three** distinct `_review.md` schemas separately (fix-lane 5-dim,
  project 7-dim, initiative none). Section headings are spelled three ways.
- Extract the **15** reviews carrying `## Required Changes` — **not** as an
  answer key. F9 established that all fifteen open with "None"; they are
  extracted only as evidence for that finding.
- Extract the **23** fix-lane triples (spec + report + review + resolving SHA),
  and mark the **22** whose checkpoint SHA resolves. These are the E3/E6 inputs
  under `METHOD-E3.md` and the S0.1 mutation source — one fixture rig serves
  both.
- Extract the 124 `workflow_run` records.
- **Resolve the 923-vs-1,030 dispatch disagreement** between the flat `agents`
  field and the per-phase census, and record which is canonical and why.

---

## E0 — Variance pilot and headroom calibration

**Arms.** One — the baseline reviewer configuration.
**n.** 10 repeats. Published agentic benchmarks commonly use 3–5; 10 is chosen
because our per-item variance is unknown and the batch also has to move a
quantised headroom figure.

**Primary outputs.**

1. **Per-item stability**, not one pooled σ. Each item is classified
   **always-pass / always-fail / flaky** across repeats. Prior work finds
   variance concentrates in a small unstable subset (15 / 1 / 16 of 32 in one
   study), so a pooled σ would describe no item well (PRIOR-ART §6). Later
   experiments are sized against the **flaky subset**, which is where power is
   needed. An item that is always-pass in every arm carries no signal and is
   excluded from agreement counts.
2. **Headroom draw per batch**, as `seven_day` and `five_hour` deltas ÷ n, with
   `resolution_limited` reported.
3. **Does a cache read draw down the subscription window?** The platform docs
   say cache hits are *not* deducted from rate limits; charter §6.1 says they
   draw at ~0.1×. These cannot both be true (PRIOR-ART §8). Compare headroom
   delta across batches with high vs low `cache_read_share` at matched total
   input. **Nothing is sized until this is settled.**
4. Confirmation that `--max-budget-usd` behaves as documented on a subscription.

**Kill criterion.** If the flaky subset is large enough that >20 runs/arm are
needed to detect a 10 pp effect, E4–E6 are unaffordable and we publish mechanism
findings only.

---

## E1 — Cache economics

**H1.** Cache-read share stays ≥ 70% across 10 sequential dispatches with a
commit between each, under at least one mitigation.
**H0.** Share collapses to ~0 — a cold write every dispatch — regardless.

**Arms**, all in one directory, 10 dispatches each, trivial prompt:

| arm | configuration |
| --- | --- |
| a | sequential, no commits — control |
| b | commit between each |
| c | b + `--exclude-dynamic-system-prompt-sections` |
| d | b, but each dispatch in its own git worktree — today's Redkiln topology |
| e | b + `--system-prompt` |

**Arm (e) exists because the RFC's own mitigations conflict.** R6 says pass
`--system-prompt` explicitly and never rely on discovery; but
`--exclude-dynamic-system-prompt-sections` is documented as *ignored with
`--system-prompt`*. If explicit payload costs the cache benefit, that is an
architectural fork and it should be found now. The likely escape — carrying the
role prompt in the **agent card** rather than `--system-prompt` — is what arms
(a)–(d) already do, so the contrast is direct.

**Primary metric.** `cache_read / (cache_read + cache_creation)` — the charter's
definition, kept. `total_input_tokens` is recorded alongside so the other
denominator can be computed later without re-running anything.

**Reported in the results table:** cache hit rate beside cost, per published
practice, so a reader can tell whether a difference came from price, volume or
reuse (PRIOR-ART §3).

**Kill criterion.** If (c) fails to restore sharing, the cache benefit is
unavailable to a commit-per-checkpoint workflow. Inversion's cost case reduces
to "avoids transcript accumulation" alone — still real, but the headline is
withdrawn and restated before proceeding.

**Expected secondary finding.** Arm (d) prices what per-worktree isolation costs
us *today*. That is a finding about the current product regardless of the Rust
decision, and it bears on shipmates L6.1 (parallel DAG execution buys capability
but fragments the cache).

### E1-TTL — behavioural cache TTL  *(runs first)*

**Promoted to the first experiment after E0** (PRIOR-ART §9.4).

**Why.** RFC §3.1's headline is that `-p` gets a 1h TTL while in-session
subagents are forced to 5m. The platform docs say the default is **5 minutes**,
1h only on explicit request. A well-evidenced but unconfirmed report claims
Claude Code's effective TTL silently moved 1h→5m in March 2026. And
`usage.cache_creation` reads `{ephemeral_1h: 0, ephemeral_5m: 0}` on every
dispatch observed — present but uninformative. **Behaviour is the only route.**

**Design.** For each gap: warm a prefix, wait, read it back. Each gap gets its
**own** warm dispatch, because a cache read refreshes the entry and a cumulative
series would measure a prefix being kept alive rather than one decaying.

**Gaps.** 2 (control, both TTLs alive), **10 (decisive — 5m dead, 1h alive)**,
then 45 and 90 to bound the upper end if 10 comes back warm.

**Reading.** Warm read at 10 min ⇒ TTL exceeds 5m and the RFC's premise
survives. Cold read at 10 min ⇒ the asymmetry does not exist for us and the
cache half of the cost argument is parity, not advantage.

**Caveat, stated in advance.** Cache-read share is noisy (FINDINGS F5: 0.58–1.00
across identical dispatches). A single gap proves nothing; repeats are required
before either reading is asserted.

---

## E2 — Effort pinning

**Largely dissolved.** `--agents` accepts a per-agent `effort` key and `--agent`
makes the main `-p` turn adopt the card (FINDINGS F7), so per-dispatch effort
pinning exists by construction and RFC R2 is retired.

**What remains is a confirmation**, folded into E0's batch: does
`usage.output_tokens_details.thinking_tokens` separate across
`--effort ∈ {low, high, xhigh}` on a fixed prompt? n = 5 per cell.

**Instrument note.** `output_tokens_details` lives under `usage`, not at the
result root. A root-only read returns null on every dispatch and would read as
"effort changes nothing" — which is the conclusion this experiment exists to
test. The lookup checks both, with a test for each.

**Outcome.** Not a kill — a constraint. If thinking tokens do not separate,
record it as a permanent architectural limitation and re-price the tier policy.

---

## E3 / E6 — superseded

The charter's blocker-recall metric is dead (FINDINGS F9: all fifteen
`## Required Changes` sections open with "None"). E3 and E6 are re-registered in
**`METHOD-E3.md`** as seeded-defect detection against clean controls, using real
historical defects re-created by reverting real historical fixes.

## Judging (applies to E3–E6, registered here because it is a method choice)

Deterministic outcomes are preferred wherever they exist.

Where quality needs judging: Ryan annotates a ≥30-item calibration set, blind,
with arm labels stripped and order randomised. An LLM judge is calibrated
against it.

**Report Cohen's κ as the headline, not raw agreement.** Raw agreement inflates
badly on imbalanced labels, and our corpus is extreme: 215/217 verdicts pass,
1,249/1,249 ledger rows satisfied, 35/36 reviews scored 3. A judge that always
says "pass" would score ~99% agreement at κ ≈ 0. Bands: κ > 0.80 strong,
0.60–0.80 substantial, < 0.60 means the rubric needs work. Use Krippendorff's α
for E6's 3-reviewer panel arm. Close calls are recorded as ties, never resolved
in our favour.
