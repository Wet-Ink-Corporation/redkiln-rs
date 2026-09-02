# Spike findings

Running log. Each entry is dated and says how it was established, so a later
reader can tell a measurement from a doc quote from an inference.

Platform observations are date-stamped because they describe behaviour we do
not control and Anthropic may change without notice (charter threat 6).

---

## 2026-09-02 — Layer 0 built and proven

**Established by:** 33 unit tests plus four real dispatches against
`claude` 2.1.258 on Windows 11, fixture `spikes/fixtures/e1`.

The runner spawns `claude -p --output-format stream-json`, streams and
classifies the NDJSON, captures full usage, validates a digest against a step
schema, and journals one record per attempt outside the fixture tree. Verified
working: `probe --repeats 2 --model haiku` returned `outcome=ok` with usage and
headroom on both dispatches.

Deferred from charter §7.1 deliberately, to protect the timebox: fan-out /
stagger / join (only E6 arm (d) needs them) and automatic headroom throttling
(pending F1 below).

---

## F1 — Plan headroom IS readable programmatically, via an undocumented event

**Established by:** direct observation of the stream, 2026-09-02.
**Supersedes:** charter §6.1, and the doc sweep's answer to the same question.

Charter §6.1 states the programmatic headroom read is the statusline
`rate_limits` object, that it exposes `five_hour` / `seven_day` /
`spend_limit`, and that it is unavailable outside a statusline script. A
documentation sweep instead pointed at `result.rate_limit` in the result
message.

**Neither is what v2.1.258 does in `-p`.** `result.rate_limit` is absent
(observed `null` on every dispatch). Instead the stream carries a top-level
event the docs do not mention at all:

```json
{
  "type": "rate_limit_event",
  "rate_limit_info": {
    "isUsingOverage": false,
    "overageDisabledReason": "out_of_credits",
    "overageStatus": "rejected",
    "rateLimitType": "five_hour",
    "resetsAt": 1788370800,
    "status": "allowed",
    "unifiedWindows": {
      "five_hour": { "resetsAt": 1788370800, "utilization": 0.04 },
      "seven_day": { "resetsAt": 1788408000, "utilization": 0.04 }
    }
  }
}
```

**Why this matters.**

1. **E0's primary output becomes directly measurable.** The charter's
   headroom-per-run figure is `seven_day.utilization` delta between consecutive
   dispatches. That is now a field, not an inference.
2. **The charter's §6.2 overage trap becomes detectable rather than assumed.**
   Crossing into usage credits silently drops the prompt-cache TTL from 1h to
   5m, which would corrupt a cache measurement exactly when consumption is
   highest. `isUsingOverage` reports it per dispatch, so an affected run can be
   excluded rather than silently averaged in.
3. **Automatic throttling is now buildable**, which the charter assumed would
   need a statusline hook.

**Caveats.** Undocumented, so it may change without notice (RFC R8) — the
runner retains the raw line on every record so a schema change is recoverable
after the fact instead of silently losing the series. There is still **no Opus
or Sonnet family sub-limit key**, so charter §6.1's central point stands: the
sub-limit that actually binds our Opus-heavy arms remains invisible.

---

## F2 — Hooks DO fire in `-p`

**Established by:** `system/hook_started` and `system/hook_response` events
observed twice per dispatch, 2026-09-02.
**Closes:** RFC §10 ("Whether settings-based hooks fire in `-p`. Documented for
the Agent SDK; not stated for CLI print mode. Matters for the `PreToolUse`
deny.")

They fire. Two consequences:

- The `PreToolUse` deny survives inversion, which matters because shipmates
  L1.2 identifies it as the one enforcement mechanism that is correctly scoped
  (a stateless predicate on tool input, never a position in a sequence).
- **It is also a confound for E1.** Hooks fired on a bare fixture repo with no
  `.redkiln/`, so they came from user-level settings and are contributing to
  the prefix. E1 arms must hold hook configuration constant and record it, or a
  settings change mid-experiment silently moves the cache baseline.

---

## F3 — The usage block uses the plain cache naming, and exposes no TTL split

**Established by:** observed `result.usage`, 2026-09-02.

Observed: `input_tokens`, `output_tokens`, `cache_read_input_tokens`,
`cache_creation_input_tokens`. **Not** observed: the `ephemeral_cache_*` naming
from the SDK reference, and **not** the `ephemeral_1h_input_tokens` /
`ephemeral_5m_input_tokens` split from the costs page.

**This is a problem for E1 as chartered.** E1's metric is stated as
"`cache_read / (cache_read + cache_creation)`; **the 1h vs 5m split**". The
ratio is available; the TTL split is not, at least not here. Since the whole
cost argument for inversion rests on `-p` getting a 1h TTL where in-session
subagents are forced to 5m, E1 needs another way to establish the TTL — or the
claim has to be reported as untested rather than assumed.

Also observed: `total_cost_usd` is populated on a subscription run (~$0.021–
0.027 for a trivial haiku dispatch). It is a client-side estimate from a
bundled price table, so it is a comparison unit, not a bill.

---

## F4 — Other undocumented events on the `-p` stream

**Established by:** event census, 2026-09-02.

`system/thinking_tokens` (4–6 per dispatch) and `system/hook_started` /
`system/hook_response` appear in no documentation consulted. None break
anything — the runner types what it knows and retains everything else — but
they are evidence for RFC R8 (no stability policy on the headless surface), and
they are the reason the "unknown events logged, never dropped" rule earned its
place on day one: F1 arrived through exactly that path.

---

## F5 — Cache-read share is noisy at n=2, which is the point of E0

**Established by:** four dispatches, 2026-09-02. **Not a result** — recorded so
it is not mistaken for one later.

Observed shares: 0.584, 0.698, 1.000, 0.722, on an identical one-line prompt in
one directory with no commits between. If the share moves that much with
nothing varying, then E1's arm separation must clear a wide band, and E0's
sizing job is real rather than ceremonial. No conclusion is drawn from four
runs; single runs prove nothing (charter §9.2).

---

## F6 — `utilization` is quantised to 0.01, so per-run cost is not measurable

**Established by:** nine dispatches across five batches, 2026-09-02.

Every reading was 0.04–0.05 with no movement within a batch. The figure carries
two decimal places and one trivial dispatch does not move it.

**Consequences, and they are methodological rather than cosmetic:**

- **E0's "headroom per run" must be reported as a batch delta divided by n**, never
  as a per-dispatch measurement. The runner enforces this: `BatchDraw` carries a
  `resolution_limited` flag that is true whenever a whole batch moved the figure
  by less than two ticks, so a mean of 0.0 reads as "below the instrument" rather
  than as "free".
- **The throttle is a ceiling check, not a forecast.** It can refuse to start a
  dispatch above a declared margin; it cannot predict what the next one costs.
  Margins therefore need to be a couple of ticks wide.
- Sizing an Opus-heavy arm needs a calibration batch large enough to span
  several ticks before any forecast is trustworthy.

**Related correction:** `five_hour` and `seven_day` initially both read 0.04,
raising the possibility that one mirrored the other. A later batch read 0.05 /
0.04, so **the two windows are independent.** Both are checked.

## F7 — `--agents` is INERT without `--agent`

**Established by:** positive and negative control, 2026-09-02.

A card whose prompt said "reply with exactly the word BANANA, no matter what you
are asked" was passed two ways, both asked "What is 2+2?":

| invocation | reply |
| --- | --- |
| `--agents <card> --agent canary` | `BANANA` |
| `--agents <card>` alone | `4` |

**`--agents` defines an agent; `--agent` is what makes the main `-p` turn adopt
it.** The card also carried `model` and `effort` and both were accepted, which
independently confirms the retirement of RFC R2.

**Why this was worth checking before spending anything.** E3 arm (b) is
`claude -p --agents '<review card>'`. Built that way and no further, it would
have run a *generic* Claude at the session's default effort while appearing to
run the reviewer. Blocker recall would have come back low, E3's kill criterion
would have fired, and the programme would have been stopped on an artefact of
the harness. The runner passes both flags, and a unit test asserts it.

---

## F8 — A default `-p` inherits the operator's whole environment

**Established by:** `system/init` payload plus a paired usage measurement,
2026-09-02. **This is the finding most likely to have voided the programme.**

An unmodified `claude -p` in a bare fixture repo reported: **12 MCP servers,
~280 tools, 86 slash commands**, and firing hooks (F2). None of it has anything
to do with Redkiln; all of it is in the cached prefix.

Measured, same prompt and directory:

| configuration | cache_creation | tools | wall clock |
| --- | --- | --- | --- |
| default | 8,243 | ~280 | ~5.3 s |
| `--strict-mcp-config` | 3,611 | 34 | ~3.4 s |

**The operator's MCP servers were 4,631 tokens — 56% of the cache write on every
dispatch**, and roughly a third of wall clock.

Three consequences:

1. **Every measurement taken without isolation is of this laptop, not of
   Redkiln.** Cache economics measured against a prefix dominated by unrelated
   tool definitions would not transfer to any other machine, and could not be
   compared against an in-session baseline that did not carry them.
2. **The runner now isolates by default** (`--strict-mcp-config`), and does not
   take the flag on trust: `isolation_ok` is computed from the child's own
   `system/init`, because an ignored flag is indistinguishable from an honoured
   one in the results. Opt out explicitly with `--inherit-mcp`.
3. **It independently confirms RFC §3.1**, which predicted "tool-definition
   re-send — worse by default: `-p` loads the full tool set + every MCP server's
   definitions". That is now a number rather than a prediction.

**Related measurement:** `time_to_first_event_ms` ≈ 1,480 ms of a ~3,400 ms
trivial dispatch — spawn, settings load and handshake are ~40% of a short step.
RFC §3.1 lists per-run process overhead as a cost of inversion and calls it
"mostly latency"; this is the first number behind that claim, and it will matter
much less on a step that runs for minutes.

---

## F10 — The prompt cache TTL is ~1 hour. First decay observed between 45 and 62 minutes

**Established by:** E1-TTL probe, `claude` 2.1.258, haiku, isolated
(`--strict-mcp-config`), one directory, no commits, nothing else dispatched in
the fixture. Results: `spikes/results/e1-ttl.jsonl`. Pre-registration `45ecf29`;
design committed in `363ab2a` before the first dispatch.

| measured gap | cache_read | cache_creation | share |
| --- | --- | --- | --- |
| 2.1 min | 25,261 | 0 | 1.000 |
| 10.1 min | 25,261 | 0 | 1.000 |
| 45.1 min | 25,261 | 0 | 1.000 |
| **61.6 min** | **17,439** | **7,822** | **0.690** |

**The boundary sits between 45.1 and 61.6 minutes — consistent with a 1-hour
TTL, and flatly inconsistent with the platform-documented 5-minute default.**

At 61.6 minutes, 7,822 of 25,261 prefix tokens (31%) had expired and were
re-created; 17,439 were still served from cache.

### What this settles

**The RFC's premise holds.** RFC §3.1's cost case rests on `claude -p` getting a
1-hour cache TTL. It does. The pessimistic reading in PRIOR-ART §8 — that the
platform's 5m default applies, or that Claude Code silently moved to 5m in March
2026 (claude-code#46829) and the 1h-vs-5m asymmetry no longer exists — is
**refuted for this configuration on this date**. A 5-minute TTL is incompatible
with a full hit and zero re-creation at 45.1 minutes.

**The instrument is vindicated over the field.** `usage.cache_creation` reports
`{ephemeral_1h: 0, ephemeral_5m: 0}` on every dispatch, so the platform's own TTL
field could not have answered this either way. Only behaviour could.

### Observation worth carrying: the prefix is segmented and ages unevenly

Expiry was **partial, not total**. A ~17.4k-token segment survived past 62
minutes while a ~7.8k-token segment expired between 45 and 62. The surviving
figure (17,439) matches the `cache_read` seen in the very first smoke dispatch
hours earlier, which suggests a long-lived base segment (system prompt and tool
definitions) sitting under a shorter-lived tail.

If that reading is right, cache decay for our workload is **gradual rather than
cliff-edged**, and the practical cost of a long gap is smaller than a binary
hit/miss model implies. Stated as an observation, not a conclusion: it rests on
one decay measurement and a matching token count.

### What is still not established

- **n = 1 per gap.** Four gaps, one observation each. The zero-creation results
  are cleaner than a ratio — there is no partial state to be noisy about — but
  61.6 min is a single observation of the decay itself, and F5 records
  cache-read share ranging 0.58–1.00 across identical dispatches.
- **The boundary is bracketed, not located.** It lies in (45.1, 61.6). Nothing
  here distinguishes 50 from 60 minutes.
- **This measures survival, not creation.** `cache_creation` was 0 on every warm
  dispatch, so the series never observed a cold→warm transition.
- **The in-session subagent side of the asymmetry is unmeasured.** The RFC's
  claim that subagents get "5 minutes, always" is taken from documentation. E3
  arm (a) is the first opportunity to observe it.
- **Haiku only, one prompt, one directory, one day.** Whether TTL varies by tier,
  prefix size, or account state is untested.
- **Date-stamped.** This is platform behaviour we do not control, and #46829
  documents it changing silently before (charter threat 6).

## F11 — S0.1 RESULT: the gate kills 9 of 10 test-reachable mutants

**Established by:** mutation testing over all 23 fix-lane items, 2026-09-02.
Instrument: revert each fix's production code, keep its tests, run the declared
`affected_gate` (`pnpm typecheck && pnpm lint && pnpm test`) with
`REDKILN_NO_GATE_CACHE=1`. Results: `spikes/results/s01-gate-fpr.jsonl`.
Pre-registration `45ecf29`.

| | n |
| --- | --- |
| items in manifest | 23 |
| excluded — fix touches no production code | 8 |
| **mutants run** | **15** |
| killed at `typecheck` | 5 |
| killed at `lint` | 0 |
| **killed at `test`** | **9** |
| **SURVIVED** | **1** |

**Two figures, and the second is the one that matters.**

- *Headline*, all mutants run: **93.3% killed**.
- **Test-reachable (n=10): 90.0% killed, 10.0% survived.**

The five `typecheck` kills are excluded from the second figure deliberately.
Each died in 3–8 seconds, having never reached the suite: reverting production
code while keeping its test can leave a tree that does not compile, which no
real defect would. Counting those as kills flatters the suite by measuring the
compiler. **The test-reachable figure is the oracle number E4 and E5 rest on.**

### The survivor

`issue-75-item-frontmatter-strict-drift` — reverting `src/schema/common.ts`
passed the **entire** gate: all three stages ran, exit 0, 272 seconds. The
retained test did not catch its own bug's reintroduction.

That is one confirmed case of the gate waving through a known-bad change, and it
is exactly the class E4's decision rule depends on not being common.

### What this means for E4 and E5

**The oracle is usable.** At ~10% survival among test-reachable mutants, "tests
green" is a meaningful signal, and E4's decision rule stands. It is not a perfect
oracle and must not be reported as one: roughly one silent regression in ten
would pass unnoticed, so an E4 arm scoring within ~10 points of another has not
been separated by this instrument.

### Limits, stated plainly

- **n = 15 run, 10 test-reachable.** Small, and a single survivor drives the
  headline rate. The confidence interval on 1/10 is wide.
- **8 of 23 items were excluded** because their fixes touch no production code
  (mostly the `plan-*` lane, which fixes workflow scripts under `workflows/`,
  a path `classify()` treats as `Other`). That is a third of the corpus outside
  this measurement, and the excluded lane may behave differently.
- **Regression-shaped defects only.** Reverting a fix re-creates the bug that
  fix addressed. Novel defects in unfamiliar code are not measured.
- **One commit's suite per item.** Each mutant runs against the suite as it
  existed at that fix's checkpoint (92 files / 1,881 tests at the August commit
  measured), not today's 236-file suite. The gate's power has changed over time
  and this averages across that.
- **Wall clock:** ~5–16 minutes per test-reachable mutant, median 323 s.

---

## F12 — S0.2 RESULT: 1 slice in 5 passed through a rejected state. Axis 2 earns its place

**Established by:** deterministic reconcile over all resume trailers across all
refs at the pinned corpus SHA, 2026-09-02. Zero model cost.
Results: `spikes/results/s02-reconcile.jsonl`.

### Re-scoped, and disclosed

METHOD.md registered S0.2's primary metric as "agreement with the recorded
outcome per run" over the 124 `workflow_run` events. **Only 1 of 101 runs
records `storiesDoneAtStop` / `slicesSealedAtStop`.** At n=1 that metric is not
computable — the same class of problem as F9. What *is* computable, and is
closer to the claim the RFC actually makes, is measured instead. No data had
been seen when the metric changed.

### What the reconcile reconstructed

| | |
| --- | --- |
| commits carrying resume trailers | 360 |
| projects reconstructed | 23 |
| stories committed (`project/slug`) | 172 |
| bare story ids (fix lane, `RK-S####`) | 31 |
| `Fix:` slugs | 28 |
| slices sealed | 74 |

Deterministic, in-process, no LLM. RFC §3.4 calls this "the single
highest-value thing to port"; it ports.

### The axis-2 claim, measured

RFC §3.4 argues `Story:` trailers alone are insufficient because "a slice
REJECTED at review looks identical, in commit history, to one that passed …
**the state that most needs recovery was precisely the state resume could not
see**."

| | n | of 74 |
| --- | --- | --- |
| latest verdict `approved` | 73 | 98.6% |
| latest verdict rejected | 1 | 1.4% |
| sealed more than once | 16 | 21.6% |
| rejected, then later approved | 14 | 18.9% |
| **ever rejected** | **15** | **20.3%** |

**The final-snapshot figure (1.4%) badly understates the risk, and the
mid-flight figure (20.3%) is the honest one.** Resume does not run at the end of
history — it runs when a workflow is re-entered, which is exactly when some
slice is sitting rejected. **One slice in five passed through a state where a
`Story:`-trailer-only resume would have skipped work that still needed doing.**

That is a quantified vindication of a design decision the RFC defended only with
an anecdote. Axis 2 is not belt-and-braces; it covers a fifth of the slices.

### Drift found

| kind | n |
| --- | --- |
| `duplicate-story-trailer` | 29 |
| `slice-left-rejected` | 1 |

**The 29 duplicates are ambiguous and are not claimed as defects.** A story
committed, rejected, and re-committed after rework would produce exactly this
shape, and 14 slices are known to have gone through that cycle. Distinguishing
"legitimate rework" from "the same story committed twice by accident" needs the
commit ordering cross-referenced against slice verdicts, which is a further
question rather than a result. Recorded so it is not mistaken for either.

### Limits

- **The reconcile was not validated against the agent's decisions**, because
  those decisions were almost never recorded (n=1). What is shown is that a
  deterministic implementation *can* reconstruct the state from git, and what
  that state contains — not that it agrees with what the Preflight agent
  concluded on any given run.
- Trailer parsing is column-anchored and case-sensitive, matching
  `src/store/verify.ts`. A trailer written differently would be invisible here
  and to the product alike.
- `latest verdict wins` is the rule applied. It matches the intent of sealing
  but is not independently verified against engine behaviour.

---

## F13 — The prompt cache has TWO segments, and 69% of it is shared across directories

**Established by:** E0 headroom calibration, 40 haiku dispatches, 2026-09-02.
20 in one warm directory; 20 in twenty **brand-new** git repos.
Results: `e0-headroom.jsonl`, `e0-headroom-cold.jsonl`.

A fresh directory is **not** a cold cache. Every one of 19 never-before-used
directories returned an identical split:

| | cache_read | cache_creation | share |
| --- | --- | --- | --- |
| fresh directories (n=19) | **17,439** | ~7,827 | **0.690** |
| a directory dispatched once before | 25,264 | 0 | 1.000 |

**The prefix is two segments:**

- **~17,439 tokens — shared across directories.** Served from cache in a repo
  that has never been dispatched in. Almost certainly the system prompt and tool
  definitions.
- **~7,827 tokens — per-directory.** Created fresh in each new directory.

**This corroborates F10 from an independent direction.** The 61.6-minute TTL read
returned *exactly* 17,439 read / 7,822 created — the same split. So the partial
expiry seen there was the per-directory segment ageing out while the shared base
survived. Two experiments, two methods, the same two numbers.

### What it changes

Charter §6.2 quotes the documentation: *"two sessions in different directories
build different prefixes and miss each other's cache. That includes worktrees of
the same repository"* — and calls this "a direct conflict with Redkiln's
architecture", which puts every initiative and every fix in its own worktree.

**Measured, that conflict is roughly a third the size claimed.** Two directories
miss ~31% of the prefix, not 100%. Worktree fragmentation is real and worth
pricing, but it is not the wholesale cache loss the docs imply — which matters
for the target design and directly for shipmates L6.1 (executing the project DAG
in parallel worktrees, "the largest single capability gain available").

**Caveat, and it could matter:** the 20 fixtures were near-identical — same
one-file structure, same commit shape, differing only in path and one line of
README text. Directories whose content differs substantially may share less.
This bounds cross-directory sharing from above for *similar* trees; it does not
establish it for dissimilar ones.

---

## F14 — Headroom: 40 dispatches and ~1.9M tokens stayed below the instrument

**Established by:** the same 40 dispatches.

| batch | cache_read | cache_creation | output | 7d utilisation |
| --- | --- | --- | --- | --- |
| warm (one directory) | 497,398 | 7,822 | 887 | 0.100 → 0.100 |
| cold (20 fresh dirs) | 356,605 | 148,709 | 817 | 0.110 → 0.110 |

**The cache-cost question is NOT settled, and I am not going to claim it is.**
METHOD.md §E0 asks whether cache reads draw down the subscription window —
platform docs say cache hits are not deducted; charter §6.1 says they draw at
~0.1×. Neither batch moved `seven_day` by a single tick, so **both readings
remain consistent with the data**: 500k cache-read tokens are either free or
merely small.

What the batches *do* establish:

- **An upper bound for sizing.** 40 haiku dispatches processing ~1.9M tokens
  cost at most ~0.01 of the seven-day window — under 1%, and possibly far less.
  Haiku mechanism arms are effectively free at this scale.
- **`five_hour` and `seven_day` move independently** (0.200/0.100, later
  0.210/0.110), confirming they are not mirrors of one another.

**A confound that limits every headroom measurement in this programme:** the
assistant driving these experiments runs on the *same account and the same
windows*. Any observed delta includes that traffic. Headroom-per-dispatch
therefore cannot be cleanly attributed while the session is active, and the
figures above should be read as upper bounds on the probe's own draw, not as
measurements of it.

---

## F15 — E2 CLOSED: effort IS applied per dispatch. RFC R2 is dead

**Established by:** 15 sonnet dispatches, one fixed prompt, one directory, only
`--effort` varying, 2026-09-02. Results: `spikes/results/e2-effort.jsonl`.

| effort | thinking tokens (n=5) | mean | output (mean, range) |
| --- | --- | --- | --- |
| low | 0, 0, 0, 0, 0 | 0.0 | 121 (113–128) |
| high | 0, 0, 24, 0, 0 | 4.8 | 130 (109–169) |
| **xhigh** | **201, 163, 246, 214, 281** | **221.0** | **360 (299–413)** |

**`high` and `xhigh` separate completely** — max high 24 < min xhigh 163, no
overlap, with output tokens nearly tripling. Reasoning effort is unambiguously
applied per `claude -p` invocation.

**RFC R2 — "`effort` has no `--agents` equivalent", severity High — is
falsified.** Combined with F7 (`--agents` carries `model` and `effort`, and
`--agent` makes the main turn adopt the card), per-dispatch effort pinning is
available by construction. The defect the tier policy's effort column closed —
`tiers.mjs`: "the cost and quality profile of a run was a property of the
CALLER, not of redkiln" — cannot return through this route.

**E2 is closed.** It was already reduced from a half-day experiment to a
confirmation; this is the confirmation.

### The nuance worth keeping

**`low` and `high` did not separate on this prompt** — both essentially zero
thinking, overlapping output ranges. Either the task is too easy to provoke
reasoning at `high`, or the two levels behave alike for sonnet here. So this
does **not** demonstrate a monotonic low < high < xhigh gradient; it
demonstrates that the mechanism works and that the *production-relevant*
distinction separates.

That distinction is the one that matters: `tiers.mjs` pins `verify:` at
sonnet/high and `review:` at opus/xhigh. high-vs-xhigh is exactly the contrast
the tier policy depends on, and it is the one that separated.

**Limits.** One prompt, one tier, n=5 per cell, one day. Whether the gradient
holds for opus, for harder tasks, or between `low` and `medium` is untested and
was not asked.

---

## F16 — E1 COMPLETE: commits cost 31% of the prefix, and `--system-prompt` costs nothing

**Established by:** 50 haiku dispatches across five arms, one directory per arm,
identical prompt, 2026-09-02. Results: `spikes/results/e1-arms.jsonl`.
Steady state excludes each arm's first dispatch, which pays the initial write.

| arm | configuration | steady share | cache_read | created |
| --- | --- | --- | --- | --- |
| **a** | no commits — control | **1.000** | 25,258 | 0 |
| **b** | commit between each | **0.689** | 17,439 | 7,853 |
| **c** | b + `--exclude-dynamic-system-prompt-sections` | **0.846** | 21,234 | 3,869 |
| **e** | b + `--system-prompt` | **1.000** | 18,752 | 0 |
| **d** | one worktree per dispatch | **0.685** | 17,439 | 8,012 |

Variance within each arm is negligible — the widest range is 0.001.

### H1 vs H0

**H1 holds, and comfortably.** The registered hypothesis was "cache-read share
stays ≥ 70% across 10 sequential runs with a commit between each, under at
least one mitigation." Arm c reaches 0.846 and arm e reaches **1.000**.

**H0 — "share collapses to ~0, a cold write every run" — is falsified even
unmitigated.** Arm b holds 0.689 with no mitigation at all: a commit busts the
~7.8k per-directory segment while the ~17.4k shared base survives untouched
(F13). Commit-per-checkpoint costs **31% of the prefix, not 100%**.

### The result that inverts my own prediction

I recorded earlier (PRIOR-ART §9, C3) that RFC R6 and R4 conflict: R6 says pass
`--system-prompt` explicitly, but that flag makes
`--exclude-dynamic-system-prompt-sections` a no-op, so the explicit-payload
discipline was expected to **cost** cache reuse. **The opposite is true.**

`--system-prompt` **replaces** the default system prompt — the thing carrying the
volatile git snapshot — with static text. There is then nothing volatile left to
bust, and the cache stays perfect across commits. It does not relocate the
churn; it deletes it.

**The two RFC mitigations do not conflict. R6's discipline is the better cache
strategy, and it is free.** `--exclude-dynamic-system-prompt-sections` recovers
about half the loss (0.690 → 0.846); `--system-prompt` recovers all of it.

Note arm e's *first* dispatch scored 0.000 — a new system prompt is a new
prefix, so there is a one-time cold write. That cost is paid once per distinct
system prompt, not once per dispatch.

### RFC §10's pivotal unknown, resolved

RFC §10 could not establish "whether `--exclude-dynamic-system-prompt-sections`
removes the git **branch/commit** snapshot from the prefix, or only the working
directory", and called it "the single fact that decides whether inversion is
cheaper or more expensive for a commit-per-checkpoint workflow."

**Answer: partially.** It recovers 3,984 of the 7,853 tokens a commit
invalidates — roughly half. The git snapshot is therefore split across the
sections the flag moves and sections it does not.

### Charter §6.2's worktree conflict, priced

Arm d — one worktree per dispatch, today's Redkiln topology — steady-states at
**0.685**, statistically indistinguishable from arm b's 0.689. So a worktree
costs the same as a commit: the per-directory segment, ~31%.

Charter §6.2 calls worktree fragmentation "a direct conflict with Redkiln's
architecture". Priced, it is a 31% prefix re-write per worktree, and the same
mitigation applies — under `--system-prompt` it should vanish, though that
combination was not run.

### Limits

- Haiku, one trivial prompt, one machine, one day. A larger prompt changes the
  ratio between the shared base and the per-directory segment.
- Arm d used ten worktrees of one repo with no commits between; it isolates the
  directory effect rather than reproducing the full commit+worktree topology.
- `--system-prompt` was a one-line static prompt. A real agent card is larger,
  which changes the size of the one-time cold write but not its one-time nature.
- Cost in headroom was unmeasurable throughout (F14); this is a cache-share
  result, not a spend result.

---

## Operational — long-lived background jobs are unreliable here

**Established by:** three background runs, 2026-09-02. Cause **not** established.

| job | shape | outcome |
| --- | --- | --- |
| TTL gaps 2,10 | ~13 min, printed a line every ~2 min | **completed** |
| TTL gaps 45,90 | ~55 min, long silent sleeps | **stopped** |
| TTL read | ~35 min, one silent sleep | **stopped** |

Not a fixed cap — 55 and 35 minutes both died, 13 minutes lived. The only
pattern visible is that the survivor emitted output periodically and both
casualties were silent for long stretches. **That is a hypothesis, not a
finding**; the notifications reported `killed` with no cause and there is
nothing in the output but a `[killed]` marker. It may equally have been the
operator.

**Why it matters beyond this experiment.** E3 as registered is ~198 Opus
dispatches and E4/E5 run the full 236-file vitest suite per gate. Both are
long-running by construction. If a job that runs for tens of minutes can vanish
without explanation, then **no measurement may depend on a single process
surviving to the end of it.**

Two mitigations, both already in place:

1. **Resume is on by default.** A killed run costs the dispatches in flight, not
   the arm; re-running into the same results file skips what already succeeded.
2. **TTL measurement was restructured** into `ttl-mark` / `ttl-read`, so a gap is
   the interval between two timestamps rather than one process's lifetime. This
   paid off immediately: the second kill landed mid-wait and cost nothing,
   because the 17:49:11Z mark stayed valid and the read simply reports whatever
   elapsed time it actually measures.

**Rule adopted:** any spike operation expected to exceed ~10 minutes is either
resumable or split into short invocations. Nothing is written that assumes a
long process completes.

---

## Open — the pre-registration SHA is currently meaningless

Every results header records `preregistration_sha` from repo HEAD. Nothing has
been committed yet, so it currently points at the initial commit, which contains
no prompts, arms or analysis plan. Charter §8.2 requires the pre-registration to
be committed *before the first arm runs*, and the recorded SHA to identify it.

**Until the spike tree is committed, no run is validly pre-registered.** The
smoke and pressure-test runs so far are instrument checks, not arms, so this has
cost nothing — but it must be closed before E0.

## Corrections to the reference documents

Carried here so the eventual RFC amendment (charter §14.4) has a single source.

| Ref | Claim | Status |
| --- | --- | --- |
| RFC R2 | "`effort` has no `--agents` equivalent" — severity High | **Falsified.** `--agents` accepts `model`, `effort`, `tools`, `permissionMode` per agent; `--agent <name>` makes the main `-p` turn adopt the card. The defect the effort column closed cannot return this way. |
| RFC §10 | "Whether settings-based hooks fire in `-p`" — unknown | **Closed.** They fire (F2). |
| RFC §10 | The exact `stream-json` schema — unknown | **Partly closed.** Result-message usage and cost observed (F3); several undocumented events found (F1, F4). |
| Charter §6.1 | Headroom readable only via a statusline script | **Superseded** by F1, except the Opus/Sonnet sub-limit, which remains invisible as stated — and it is the limit that binds E3/E6. |
| Charter §12 | Runner "refuses to dispatch below a configured margin" | **Built** (`throttle.rs`), and now real rather than aspirational. Bounded by F6: a ceiling check, not a forecast. |
| Charter §6.4 | Pin `claude` ≥ 2.1.251 | Installed 2.1.258. Recorded on every record. |
| RFC R4 / R6 | Mitigations conflict | **Unresolved and structural.** `--exclude-dynamic-system-prompt-sections` is "ignored with `--system-prompt`", so the explicit-payload discipline disables the cache mitigation. Carrying the role prompt in the agent card instead may sidestep it — E1 arm (e) tests this. |
| Charter E1 | Metric includes the 1h vs 5m split | **At risk** — the split was not observed in `usage` (F3). |

## Corrections about the corpus

| Ref | Claim | Reality |
| --- | --- | --- |
| Charter §8 | `_review.md` "18 of 38" resolve | 38 of 38 resolve, but only **14** carry `## Required Changes` — that is E3's real n |
| Charter §8 | "~20 incidents + fix-lane bugs, each with a spec and a known-good fix" | **23** fix-lane bugs qualify; the 10 incidents do not (no specs, no commits, 9 of 10 never left triage) |
| Charter E4 | Oracle is "AC ledger rows satisfiable" | **Not computable.** All 110 `_ledger.md` are under `_archive/`; none in the `support/` fix lane. Empty join — oracle must change |
| Charter §8.1 | Negative signal is scarce | True, but understated in one direction: `gate_result` runs **187 pass / 53 fail**, a usable 22% negative class the charter does not mention |
| — | Affected-area test selection | **Does not exist.** `affected_gate` is `pnpm typecheck && pnpm lint && pnpm test` with no `{{changed}}` placeholder, so every gate run is the whole 236-file vitest suite. Dominates E4/E5 wall clock |
