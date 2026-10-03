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

### Correction (2026-09-11)

Audit PIA-067. F3's "**not** the `ephemeral_1h_input_tokens` / `ephemeral_5m_input_tokens` split" was read
downstream as "F3 measured the fields absent" (design §0.1 row 28; ADR-0078 clause (iv)). What was absent on
2026-09-02 was the **flat** naming from the costs page. The split is on the record as a **nested object**,
`usage.cache_creation.{ephemeral_5m_input_tokens, ephemeral_1h_input_tokens}`, and the runner has parsed it since
E0 (`runner/src/event.rs` `ttl_split()`, :155-165; METHOD.md §E1 :247 already says "present"). Recounted
2026-09-11 over every `spikes/results/*.jsonl` record carrying a `usage` block: **950 of 955** carry the nested
object (the five without are four void records and one P8 driver record with `digest: null`); on all 804 records
where both the total and the object are present, `cache_creation_input_tokens` equals the two tiers' sum
[measured]. What the object holds, which F3 did not say and no later entry recorded: **621 records carry
`ephemeral_1h_input_tokens > 0` and not one carries `ephemeral_5m_input_tokens > 0`** — every cache creation a
`-p` dispatch reported in this programme was on the **1 h tier** (e.g. `e3.jsonl` `b-s-silent`: 30,687 / 23,894 /
20,800 / 21,149 created, all `ephemeral_1h`, `ephemeral_5m` 0; the 329 "both zero" records are warm re-reads that
created nothing) [measured, recount 2026-09-11]. That is per-run `ephemeral_*` evidence for the `-p` 1 h tier, the
thing design §0.1 row 28's gate asked for; the in-session side (arm (a)'s subagents) is not separated by this
field and stays untested. ADR-0078's successor and S-TTL1's scope are the register's to settle; nothing here is an
edit to F3, whose flat-naming observation was correct on the day.

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

## F14 — Headroom: 40 dispatches and ~1.9M tokens stayed below the instrument *[corrected 2026-09-11: ~1.0M — Correction (2026-09-11) below]*

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
  *[corrected 2026-09-11: ~1.0M]* cost at most ~0.01 of the seven-day window — under 1%, and possibly far less.
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

### Correction (2026-09-11)

Audit PIA-077. The heading's and the sizing bullet's token total is a mis-sum; the table above is right. Over
the two batches: cache_read **854,003** (497,398 + 356,605), cache_creation **156,531** (7,822 + 148,709), output
1,704 — **~1.0M tokens (854k cache-read, 157k cache-creation)** [measured, `results/e0-headroom.jsonl` and
`results/e0-headroom-cold.jsonl`, recounted 2026-09-11]. Both batches' `seven_day` readings, from the same records'
`batch_draw` lines: warm batch `first 0.10 → last 0.10` over 20 dispatches; cold batch `first 0.11 → last 0.11` over
its 20 single-dispatch draws (`resolution_limited: true` on every line). The conclusion — neither batch moved
`seven_day` by a tick — is unchanged; the upper bound it licenses is for ~1.0M tokens, not ~1.9M. `METHOD.md` §E0's
struck gate and `HANDOFF.md` restate the 1.9M and carry the same marker.

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

## F17 — The E3 fixture was defeatable: the reviewer restored the fix and reviewed the real code

**Established by:** the first E3 pilot dispatch, 2026-09-02.
**No E3 result is claimed. The rig was wrong and is being rebuilt.**

One item (`issues-135-129-declared-boundary-parse`, 13 ACs), condition
`s-silent` — `src/store/verify.ts` and `test/verify.test.ts` reverted so the
gate stays green while the ACs go unmet. Opus/xhigh, 42 turns, 22,853 output
tokens, 11,231 thinking, 526 seconds. It returned `approved`.

**It was right to.** From its own report, verbatim:

> "the `s01-wt` checkout sitting at that SHA holds a *reverted* spike fixture,
> **which I restored after use**"

and

> "**Mutation proof (the strongest test).** I restored the tree to `d9397179`,
> then reverted **only** `src/store/verify.ts` to `HEAD~1` while keeping the new
> tests. 10 of the 14 new tests went RED"

The reviewer had `Bash` in a git worktree whose object store still contained the
fix commit. It checked the fix out, reviewed the **real** code, and — entirely on
its own initiative — **ran the same mutation experiment S0.1 runs**, reporting
which ACs the reverted file breaks.

### Why this matters more than a broken fixture

1. **The mutation was never durable.** METHOD-E3 assumed the reviewer reviews the
   tree as presented. An agent with `Bash` in a repo that still contains the fix
   can undo the seed in one command. Every `s-silent` record from this rig would
   have measured the reviewer against the *fixed* code while being scored against
   *reverted* ground truth — producing a systematic false "missed the defect"
   across the whole arm, and firing the programme's hardest kill criterion on a
   fixture artefact.
2. **It is evidence the reviewer is strong**, though not the evidence E3 was
   built to collect. Given a suspicious tree it reconstructed the provenance,
   restored it, mutation-tested the file, and cited which ACs each mutation
   broke.
3. **It restored the fixture afterwards** — considerate, and it means the
   worktree state was not left corrupted.

### The fix

The fixture must make the fix **unreachable**, not merely absent from the
working tree. A shallow fetch of the fix commit's **parent** into a fresh repo
gives a single-commit history containing only the pre-fix state, with no object
for the reviewer to restore. Spec and report are read from the frozen corpus and
passed in the prompt, so they are unaffected.

### A second defect found in the same dispatch

The record was marked **void** on "agent card not adopted" — wrongly. The card
*was* adopted: `system/init` reported `tools: ["Read", "Bash"]`, restricted from
the 34 otherwise available. But `effective.agent` is **not populated** by
`system/init` when the agent is set via `--agent`, and that was what the check
read.

The adoption check now compares the **effective tool set** against the card's
declared tools. F7's principle stands — verify adoption from the child's own
view — but the field it reads was the wrong one.

---

## F18 — Half of all human gates outlive the cache TTL

**Established by:** 174–191 measurable gate pauses in the frozen telemetry,
2026-09-02. Zero model cost.

| anchor | n | median | p90 | max | **> 1 h** |
| --- | --- | --- | --- | --- | --- |
| `gate_result` → `verdict` | 174 | 10.5 min | — | — | **45%** |
| `stage_enter` → `verdict` | 191 | 51.6 min | ~9 h | **226 h** | **48%** |

The two anchors bracket the real human latency — the first may include tail
agent work, the second certainly does — and **the figure the design rests on is
stable across both: roughly half of all gates exceed one hour.**

**Why it matters.** F10 puts the prompt-cache TTL at ~1 hour (boundary
45.1–61.6 min). So **the human gate sits precisely on the cache cliff.** About
half of all suspensions resume onto a cold prefix and pay a full re-write.

The distribution is not "humans are slow". It is **fast, or gone**: a median
around ten minutes, a p90 near nine hours, and a longest observed pause of
**9.4 days**.

**Consequences, carried into `METHOD-H.md`:**

- The per-gate cost is bounded and knowable — one cold prefix write, about half
  the time. What is *not* known is its size at review scale; F10 and F16
  measured a ~25k trivial prefix, and a real review carries spec, report and
  diff.
- `--system-prompt` held 1.000 across commits (F16). If it also blunts the
  *time* decay, the human gate becomes near-free — a design conclusion, not a
  detail.
- At a p90 of ~9 hours, **world drift during a gate stops being hypothetical**,
  which makes shipmates L2.2 ("recorded state is a cache; git is truth") a
  requirement of resume rather than good hygiene.

**Limits.** One operator, one project, one tool. A team's distribution would
differ and probably lengthen. And these gates were answered through a live
session — a headless gate may change how fast people answer, plausibly for the
worse, since nothing is waiting in front of them.

---

## F19 — H2/H3 RESULT: suspension is lossless; the world moves, but rarely and predictably

**Established by:** 174 historical gate pauses replayed against the frozen
corpus, 2026-09-02. Zero model cost. Results: `spikes/results/h2-h3.jsonl`.
Registered in `METHOD-H.md`.

### H2 — is a suspension lossless across a process boundary?

**174 of 174 lossless. Zero failures.**

Each pause's reconciled state was serialised, re-hydrated by a **separate
invocation of the binary**, and digested. Every digest matched its in-process
original. Round-tripping inside one process would have proved nothing; the risk
was always that a *fresh* process reconstructs something different, and it does
not.

**METHOD-H's gate is passed.** The registered decision rule was that a
non-deterministic resume makes the human gate "unsafe at any cost" and would
falsify the RFC's central enforcement claim. It is deterministic.

**What this does and does not license.** It shows the *state carrier* is
faithful — what is written is what comes back. It does **not** show that a
resumed run takes the correct next action, because no FSM was replayed; that
needs the ported engine and remains open.

### H3 — does the world move while a gate is open?

| gate length | n | drifted | rate |
| --- | --- | --- | --- |
| ≤ 1 h | 95 | 3 | **3%** |
| **> 1 h** | 79 | 9 | **11%** |

**A gate that outlives the cache TTL is 3.6× more likely to have the world move
underneath it.** *[RR 3.6, 95% CI [1.0, 12.9], Fisher exact p = 0.039 — marginal at n = 174, one operator; Correction (2026-09-11) below]* The same threshold governs both cost and correctness, which is
a convenient coincidence for design: *short gates are cheap and safe; long gates
are expensive and risky, together.*

Overall drift is **12 of 174 (7%)** — lower than the raw activity suggests.
**1,163 commits landed inside gaps**, but only 12 pauses drifted, because most
commits carry no resume trailers and so cannot change reconciled state. Median
commits-in-gap is **0**; the maximum is 209.

The drifting pauses are strongly bimodal: six under two hours, and six clustered
around **9.4 days** — the same long tail F18 found. Drift is not a steady
background rate; it is what happens when a gate is *abandoned* rather than
merely delayed.

### What it means for the design

- **Resume must re-derive from the world, not trust the record** (shipmates
  L2.4). H2 proves the record is faithful; H3 proves it can be *stale*. A
  faithful-but-stale record is the more dangerous of the two, because it looks
  correct.
- **Drift is rare enough to reconcile eagerly and cheaply.** At 7%, and with a
  median of zero in-gap commits, a full reconcile on every resume costs almost
  nothing and catches the 7% — there is no case for an optimisation that skips
  it.
- **Gate length is the single design lever.** Under an hour: cache-warm and
  drift-free. Over: cold prefix and 3.6× drift risk. That argues for prompting
  humans hard and early rather than for cleverness in the orchestrator.

### Limits

- **The FSM was not replayed.** "Same next action" is stronger than "same state",
  and only the former fully answers METHOD-H's H2.
- **Drift is measured on resume-relevant trailers only.** A commit that changes
  code without touching `Story:` / `Slice-Verdict:` does not register here, yet
  could still invalidate a paused review. This is a **lower bound** on drift.
- One operator, one project; replayed verdicts, not live human behaviour.

---

## Operational — long-lived background jobs are unreliable here

**Established by:** three background runs, 2026-09-02. Cause **not** established.

| job | shape | outcome |
| --- | --- | --- |
| TTL gaps 2,10 | ~13 min, printed a line every ~2 min | **completed** |
| TTL gaps 45,90 | ~55 min, long silent sleeps | **stopped** |
| TTL read | ~35 min, one silent sleep | **stopped** |

**Updated 2026-09-03 — the output-frequency hypothesis is FALSIFIED.** Two more
kills followed: one at ~65 min into a 65-minute sleep, and one that died after
roughly **two minutes** without printing its first line, despite being written
to emit output every minute. Six data points now: ~13 min survived; ~2, ~35,
~55 and ~65 min did not. There is **no duration pattern and no output pattern**.

The cause remains undetermined. The notifications report `killed` with no
reason and the output holds nothing but a `[killed]` marker. The kills do appear
to cluster at conversational turn boundaries, which would suggest the session
reaps background work when it takes a new turn — but that is a third hypothesis,
offered with no more evidence than the two it replaces, and it should not be
recorded as a finding.

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

**Rule adopted, and strengthened:** no spike operation may depend on a
background process surviving *at all*. Anything needing a wait is expressed as a
durable mark plus a later read, so the wait is an interval between two
timestamps rather than a process lifetime.

This has now paid for itself four times. The H1 decay mark at `04:38:06Z`
survived a kill that destroyed the job scheduled to read it, and remains valid
and readable at any later moment — the read simply reports whatever elapsed time
it measures.

---

### Correction (2026-09-11)

Audit PIA-073. "3.6× more likely" is the point relative risk of the H3 table — 9/79 against 3/95 = **RR 3.6** — and
was stated without its interval. **95% CI [1.0, 12.9]; Fisher exact p = 0.039** [measured, recomputed 2026-09-11
from the table above; the audit's figures agree]. The lower bound touches 1.0, so the effect is **marginal at
n = 174, one operator**; the bimodal-drift observation and the design conclusions (reconcile eagerly; gate length is
the lever) do not rest on the ratio's size. Restated the same way wherever 3.6× is quoted (`HANDOFF.md` :36 by
marker; the SM register and the design by their own errata). The S0.2 figure quoted with it in some restatements is
**20.3%** (15/74, the table at F12 above), not 20.2%.

---

## F20 — Two rig defects the E3 pilot could not have revealed, both caught before the set was run

**Established by:** four haiku probes and one zero-cost dispatch failure,
2026-09-03. **No E3 result is claimed.** The handoff's rule — validate one item
before spending on the set — paid for itself twice in one sitting.

### Defect 1 — an agent card's `tools` list silently disables `--json-schema`

E3's primary metric is mechanical only because the digest carries a `files`
array per finding. That digest arrives through `--json-schema`. **No dispatch in
the whole programme had ever used the flag**: E1, E2, E0 and H1 all ran
schemaless, where prose *is* the contract. E3's single pilot dispatch was the
first to demand structure, and it came back `error_kind=schema`, digest `null` —
which was read at the time as one more consequence of the F17 fixture flaw.

It was not. Three haiku probes isolate it:

| configuration | `stop_reason` | `structured_output` |
| --- | --- | --- |
| `--json-schema`, no card | `tool_use` | **present, conforming** |
| `--json-schema` + card with `tools: [Read, Bash]` | `end_turn` | **absent** |
| `--json-schema` + card with `tools: [Read, Bash, StructuredOutput]` | `tool_use` | **present, conforming** |

Structured output is delivered by a **tool call** — the tool is named
`StructuredOutput` and appears in an unrestricted child's 34-tool list. A card's
`tools:` is an **allowlist**, so a card that does not name it removes the only
channel the digest can return on. The flag is then inert: no warning, no error,
exit code 0, `subtype: "success"`.

**What the reviewer does instead is the dangerous part.** It does not refuse. It
writes a JSON block in prose — and in the haiku probe that block did not even
match the schema it had never been shown, inventing `file` / `severity` /
`message` where the schema asked for `issue` / `files`. A rig that scraped JSON
out of prose rather than requiring `structured_output` would have parsed that
happily and scored it.

**Consequence for E3, had this run unfixed:** every one of the 46+ dispatches
would have returned prose, every record would have been void for want of a
digest, and the entire Opus/xhigh budget would have bought nothing. The pilot
already showed this and it was misattributed to F17.

**The fix, and the disclosure it requires.** `StructuredOutput` is added to the
card's declared tools (`e3::CARD_TOOLS`) and to the adoption check, which is now
set **equality** against the declared list rather than the previous size bound.
This is an instrument deviation and is disclosed as one in METHOD-E3 §6: it
grants the reviewer no new reach — it cannot read, run or write anything it
could not before — it is the channel the digest comes back on, and it is applied
identically to every arm. **Made before any E3 data was seen**, which is the only
reason it is legitimate.

**This is a product finding** (charter §14.5, and the fifth to report): a
tool-restricted `--agent` silently voids `--json-schema`, on a success exit,
with no diagnostic. Any caller composing a restricted card with structured
output is affected, and the failure is invisible unless the caller checks for
`structured_output` specifically rather than parsing the text.

### Defect 2 — `build_silent_fixture` had never once completed

The F17 rebuild — a shallow fetch of the fix's **parent**, so the fix is
unreachable rather than merely reverted — was written, unit-tested and committed,
and the handoff records the rig as "built and validated". The first real
invocation failed instantly:

```text
fatal: '//?/D:/repos/redkiln' does not appear to be a git repository
```

`std::fs::canonicalize` returns a Windows extended-length path
(`\\?\D:\repos\redkiln`); the code slash-flipped it for git, producing
`//?/D:/…`, which git parses as a URL with host `?`. Every caller passes a
canonicalised path, so **no code path reached the fetch successfully, ever**. The
unit tests could not catch it: they test scoring, prompts and schema shape, none
of which touch the filesystem.

Fixed by stripping the prefix (`e3::git_remote_path`), with a unit test. The
fixture now builds, and `git cat-file -e <fix-sha>` confirms the fix commit is
absent from the fixture's object store, which is what F17 demands.

**Cost of both defects: one failed dispatch that spent nothing, plus four haiku
probes.** Had either reached the set, the loss would have been the whole E3
budget — and Defect 1 would have produced a uniform "no arm returned a digest"
that looks nothing like a kill criterion, while Defect 2 would have aborted the
run outright.

### The instrument moved under the programme

`claude` is now **2.1.259**; every record before today reads 2.1.258. The hygiene
check is a floor (≥ 2.1.251), so nothing blocked, and the version is on every
record. Recorded here because RFC R8 predicts exactly this and because a
cross-session comparison now spans two builds of the CLI.

---

## F21 — E3 arm (b) COMPLETE: 23/23 detection, 2/23 false positives, MCC +0.917

**Established by:** 46 scored dispatches — the full paired set, every item in
both conditions — against `claude` 2.1.259, 2026-09-03, Opus/`xhigh`, corpus pin
`47faec9f`. `spikes/results/e3.jsonl`; audit `spikes/results/audit-e3.py`.
**Spend: $56.47.** An earlier version of this entry reported the interim set
(n=5 / n=2); those numbers are superseded by the full set below.

**This still cannot fire the kill criterion.** METHOD-E3 §9 fires on a
*comparison*: arms (b) and (c) against arm (a). **Arm (a) has not been built.**
One arm measured against ground truth is a prerequisite for the comparison, not
a substitute for it, and nothing here passes or fails the thesis.

### The result

| condition | n | result |
| --- | --- | --- |
| **S-SILENT** | 23 | **strong detection 23/23** · weak 23/23 |
| **CLEAN** | 23 | **false positives 2/23** (8.7%) |
| combined | 46 | **MCC +0.917** (tp=23 fn=0 fp=2 tn=21) |

Every counted record cleared all three checks: `card_adopted` (effective tools
exactly `[Read, Bash, StructuredOutput]`, down from 34), `node_modules_linked`,
and on S-SILENT `fix_unreachable`, verified by `git cat-file -e` failing to find
the fix commit in the fixture's own object store.

**The wrong-reason gap is zero across all 23.** Weak detection equals strong
detection on every item: not one rejection cited a file the revert did not
touch. That is the number that distinguishes this from an always-reject arm,
and it is why the strong criterion was pre-registered as mechanical rather than
judged.

### Stratified, because the corpus holds two defect shapes

| shape | n | detection |
| --- | --- | --- |
| **code** (fix touched `src/`) | 14 | **14/14** |
| **tests-only** (fix touched no `src/` file) | 9 | **9/9** |

9 of the 23 fixes touched no source file at all, so reverting one removes only
tests: the unmet ACs are about coverage *existing*, not about code being wrong.
That is a second defect shape, and METHOD-E3 §12.1 warns against generalising
from one. Detection is total in both, so the pooled figure hides no weak
stratum — but the two are reported apart from here on regardless.

### Against the literature the charter cites

METHOD-E3 §5 cites a reviewer measured at **FPR 0.20% / FNR 87.9%** — it missed
88% of real defects. Arm (b) sits at **FNR 0% / FPR 8.7%**: a completely
different operating point, far to the sensitive end. Whether that trade is the
right one is a product question, not a measurement one, and the two false
positives are where it has to be argued.

### Both false positives are substantive, and both need adjudication

Neither is a lazy or scattershot rejection. Each scored the mechanical
dimensions highly and rejected on one specific, mechanism-level finding.

**1. `fix-advance-write-lock-gate-scope`** (750s, 32 turns). Gate-greenness 3/3
— it independently re-ran the whole merge gate: `tsc --noEmit` clean, `eslint`
clean, `vitest` 95 files / 1927 tests / 0 failures in 308s. Boundary-fidelity
3/3. It rejected on **regression-risk 1/3**, citing two behaviour changes it
says it confirmed by differential probe against the pre-fix code path:
`gate_result.basis` flipping `evaluated` → `memoized` for every real `--apply`
through a memoized gate, and a blocked *entry* gate no longer rendering the
target stage's `produces` artifacts. Both with `file:line`.

**2. `issue-146-147-boundary-window`** (2055s, 71 turns). It judged 17 of 18
criteria genuinely met, killed 8 mutants itself, and scanned all 151 `spec.md`
files in `.bklg` through `declaredBoundary` with pre- and post-fix terminators
to check for verdict changes (zero). It rejected on a **single** finding:
AC-015's ordering criterion is not actually tested, because the assertion's
regex is already satisfied by pre-existing prose in the workflow file and would
pass without the fix. That is a vacuous test — precisely the escape-hatch class
`agents/redkiln-review.md` exists to catch.

**Why this matters for the rate.** CLEAN's ground truth is "this shipped and a
historical review approved it", which is **not** the same as "this is
defect-free". If either finding is real, the reviewer is right and the FPR is
overstated. This is exactly what METHOD-E3 §7 reserves finding fidelity and
Cohen's κ against Ryan's blind annotations for; it cannot be settled
mechanically, and it must not be settled by me, since I would be grading the
instrument I built. **Both are referred, and the co-primary rate is reported as
2/23 pending that adjudication.**

### The effort asymmetry, at full n

| condition | wall clock | turns | cost |
| --- | --- | --- | --- |
| S-SILENT | 97–424s (median ~130s) | 11–19 | $0.45–0.64 |
| CLEAN | 317–2055s (mean 767s) | 28–71 | $1.06–4.89 |

**The reviewer works roughly six times harder when the fix is actually
present.** A rubber stamp and an always-reject arm would both be flat across
conditions; this is the opposite, and it is behavioural evidence that the
verdicts are earned rather than defaulted. The longest CLEAN dispatch was 2055s
and two others exceeded 1900s — all three would have voided under the original
900s ceiling, which was written for the S-SILENT shape before any CLEAN
dispatch had run.

### Sizing — condition-dependent, and the S-SILENT figure alone misleads

| | five_hour per dispatch |
| --- | --- |
| S-SILENT | **+0.03** |
| CLEAN | **~+0.077** |

five_hour binds long before seven_day. Budgeting the paired design off the
S-SILENT figure would have under-costed it by about half. Both are ceilings, not
point estimates: this assistant session shares the account, and the Opus family
sub-limit remains invisible (F1).

### Platform errors are now a named category

Seven dispatches returned API **529 Overloaded** and one API **500** *[corrected 2026-09-11: six and one, seven records — Correction (2026-09-11) below]* — `turns=1`,
$0, `subtype: "success"`, `is_error: true`. All were originally filed as *schema*
faults, which would later have read as the reviewer or the rig failing.
`api_error_status` is on the result, so the distinction costs nothing, and the
void list now says which is which. Every one retried clean. A void that a later
record supersedes is marked as such: it cost a dispatch, not data.

### What remains before E3 can be reported

1. **Arm (a) does not exist**, and it is the entire comparison.
2. **The two false positives are unadjudicated** (§7 κ), and the co-primary rate
   depends on their disposition.
3. **Equivalent-seed triage (§10) has not been applied.** With detection at
   23/23 it cannot lower the detection rate, but the exclusion list is still
   owed.
4. **No item has been run twice** — E0's per-item stability (`--repeat N`) is
   still unmeasured, and a 23/23 with no variance estimate is one sample of a
   stochastic process, not a constant.

---

### Correction (2026-09-11)

Audit PIA-075. "Seven … 529 and one … 500" is eight; `results/e3.jsonl` holds **seven** `is_error: true` records:
**six** `api_error_status: 529` — `fix-doctor-git-config-repair`, `issue-122-doctor-foundation-reachability`
(twice), `issue-130-slice-review-story-gate`, `issue-131-kb-ingest-doctor-scope`,
`issue-72-plan-briefs-readonly-grounding` — and **one** `500` — `issue-82-status-terminal-field-collision`, whose
`void_reason` still reads "no schema-valid digest (schema)" because it was filed before the category existed. All seven are arm `b-s-silent`,
`turns=1`, $0, void, and superseded by a clean retry [measured, recount 2026-09-11]. The fixtures the registers
call "may not exist" (AC-428, SM-127) are these seven records.

---

## F22 — Background jobs die; a detached process does not

**Established by:** roughly a dozen kills and one 12,000-second survivor,
2026-09-03. Supersedes the "no pattern" conclusion of the earlier entry for the
practical case, without explaining it.

The background-kill trap had eight observations and no duration or output
pattern. It now has a **workaround, established by contrast**:

| launch shape | outcome |
| --- | --- |
| harness background job | killed, repeatedly and at any age |
| harness foreground, under the tool limit | **never killed** |
| harness foreground, exceeding the limit → promoted to background | killed |
| **`Start-Process` detached** | **survived 12,000s** through ~14 killed watchers |

`issue-81` was killed three times as a background job and completed in 116s in
the foreground. The entire remaining CLEAN set — 11 items, 3¼ hours, including
three dispatches over 1900s — was finished by one detached `Start-Process`
runner while every watcher observing it was killed underneath it.

**The cause is still not established** and no claim is made about it. What is
established is operational: **anything that must survive belongs outside the
harness's job tree.** A hypothesis that chained commands (`a && b`) were the
trigger was formed and falsified within two dispatches — a single command was
killed on the same item — and is recorded here only so it is not re-formed.

This does not weaken the resume discipline; it vindicates it. Resume is keyed
per `(arm, slug, repeat)`, and across every kill in this session the cost was
the dispatch in flight and never the arm.

---

## F23 — E3 RESULT: the kill criterion does not fire. Detection is tied at ceiling; the arms are statistically indistinguishable

**Established by:** 92 scored dispatches — both arms × both conditions × all 23
items — against `claude` 2.1.259, 2026-09-03/04, Opus/`xhigh`, corpus pin
`47faec9f`. `spikes/results/e3.jsonl`. **Total spend $153.86.**

### The comparison E3 was built to make

| | arm (a) in-session | arm (b) headless |
| --- | --- | --- |
| **S-SILENT detection** | **23/23** | **23/23** |
| wrong-reason gap | 0 | 0 |
| **CLEAN false positives** | **4/23** | **2/23** |
| MCC | +0.839 | +0.917 |
| verdict agreement on CLEAN | 19/23 (83%) | 19/23 (83%) |

### The kill criterion, stated in METHOD-E3 §9, does NOT fire

> "If arms (b) and (c) both show materially lower S-SILENT detection than arm
> (a) — a drop the E0-sized test can detect — at comparable or worse
> false-positive rate, the inversion thesis fails, the RFC recommendation is
> withdrawn, and the programme stops."

Detection is **identical and at ceiling**: 23/23 in both arms, zero discordant
pairs. There is no drop to detect, materially or otherwise. The headless arm's
false-positive rate is *lower*, not worse. **The thesis is not killed.**

### But this is NOT evidence that headless review is better

Two reasons, and both matter more than the headline.

**1. The false-positive difference is not significant.** The design is paired,
so the test is McNemar's on the discordant pairs: 3 items where (a) rejected and
(b) approved, 1 the other way. **Exact two-sided p = 0.625.** Four disagreements
out of 23 is what this design cannot distinguish from chance. Reporting "2/23
beats 4/23" as a finding would be exactly the over-reading the pre-registration
exists to prevent.

**2. Detection saturated, so the primary metric has no discriminating power.**
METHOD-E3 §9 anticipated a *floor* effect — "if every arm, including (a),
detects near zero, the finding is about the seeds or the rubric" — and made it
a reason to report E3 inconclusive rather than fire the criterion. **The mirror
case happened and the same logic applies.** At 23/23 versus 23/23 the primary
metric cannot separate two arms that differ by any amount smaller than "misses
a defect it was pointed at". The seeds are real historical defects with the
whole spec supplied and the fix's file list named; they are, evidently, not
hard. The comparison therefore rests entirely on the co-primary FPR, which is
underpowered at n=23.

**The honest summary: on this corpus, at this difficulty, the two arms are
indistinguishable.** That is a genuine result — it refutes the specific fear
that headless dispatch degrades review quality — but it is not the same as
showing headless is superior, and the write-up must not slide between the two.

### What separates them is cost, not quality

| | arm (a) in-session | arm (b) headless |
| --- | --- | --- |
| total | **$97.39** | **$56.47** |
| S-SILENT median wall | 242s | **128s** |
| CLEAN median wall | 769s | **682s** |
| median turns | 3 (parent) | 15 / 36 |

**The in-session shape cost 1.72× the headless one for the same detection.**
That is the RFC's cost argument, measured rather than assumed, and it is the
first quality-controlled cost comparison in the programme: same card, same
model, same effort, same fixtures, same material, one flipped flag.

The turn counts show why the comparison is fair rather than an artefact: arm
(a)'s *parent* takes ~3 turns because the work happens inside the subagent,
whose 15-odd inspections are recorded separately. The parent is a relay, and
that is what it was built to be.

### The arm (a) validity checks held on every record

| check | result |
| --- | --- |
| `dispatched_card` | **46/46** |
| `parent_inspected` | **0/46** |
| `dispatches > 1` | 0/46 |
| subagent inspections | median 15 |

**Not one parent reviewed the tree for itself.** The confound that would have
made arm (a) a disguised second copy of arm (b) — a parent doing the review and
merely nominally delegating — did not occur once in 46 dispatches. This is
checked, not assumed: a parent's own calls carry `parent_tool_use_id: null`,
the subagent's carry the `Agent` tool-use id.

### The disclosed asymmetry, and what it did

METHOD-E3 §6 records that arm (a)'s digest is authored by the **parent relaying**
the subagent's findings, because a subagent returns prose to its parent, while
arm (b)'s digest is written by the reviewer itself. Relay loss was a live
explanation for any arm (a) shortfall.

**It did not cost arm (a) any detection** — 23/23 with the wrong-reason gap at
zero means every relayed rejection still carried a correct file citation. Relay
therefore cannot explain the FP difference either, which is one more reason to
treat that difference as noise.

### The four disagreements, all on CLEAN

| item | (a) | (b) |
| --- | --- | --- |
| `issue-63-advance-gate-diagnostic-truncated` | REJECT | approve |
| `issue-74-plan-briefs-dead-plan-anchor` | REJECT | approve |
| `issues-135-129-declared-boundary-parse` | REJECT | approve |
| `issue-146-147-boundary-window` | approve | REJECT |

Together with arm (b)'s two, **five distinct shipped-and-approved items were
rejected by at least one arm.** Every one needs the §7 κ adjudication against
Ryan's blind annotation before any false-positive rate can be called final —
CLEAN's ground truth is "this shipped and a review approved it", which is not
"this is defect-free". `fix-advance-write-lock-gate-scope` is the only item both
arms rejected, which makes it the strongest candidate for a real missed defect
rather than reviewer over-strictness.

### Limits, restated because the headline invites over-reading

1. **n=23, one repo, one operator, one domain**, and the FP comparison is
   underpowered at that n.
2. **Detection is at ceiling**, so the primary metric discriminates nothing.
   A harder seed set — novel defects rather than reverted fixes, or specs
   withheld — is what would give this design power.
3. **No item was run twice.** 23/23 is one sample of a stochastic process, and
   E0's per-item stability is still unmeasured.
4. **Arm (c) was not run**, per §8's pre-declared reduction order.
5. **The five rejections are unadjudicated**, and the FPR is provisional until
   they are.

---

## F24 — ADJUDICATION: all six E3 rejections are factually TRUE. Both false-positive rates may be zero

**Established by:** mechanical replication of every claim, 2026-09-04, in the
frozen CLEAN fixtures the reviewers themselves saw. **Zero model cost** — every
check is a mutation plus a local test run.

**What this entry settles and what it does not.** It settles whether the
reviewers' factual claims are *true*. It does **not** settle whether each
warranted `changes-requested`, which is a severity judgement and belongs to
Ryan's κ annotation (METHOD-E3 §7). The model that built the rig must not grade
it.

### Every claim replicated

| item | arm | claim | replication |
| --- | --- | --- | --- |
| `issue-146-147-boundary-window` | b | AC-015's ordering test is vacuous | **CONFIRMED** — 36/36 pass with the ordering inverted |
| `issue-63-advance-gate-diagnostic-truncated` | a | AC-001 untested under its own precondition | **CONFIRMED** — 18/18 pass with `detail` truncated |
| `issues-135-129-declared-boundary-parse` | a | the `Path`-header guard is unpinned | **CONFIRMED** — 63/63 pass with the guard disabled |
| `issue-74-plan-briefs-dead-plan-anchor` | a | the strictness claim is unproven | **CONFIRMED** — 28/28 pass with `=== true` relaxed |
| `fix-advance-write-lock-gate-scope` | a + b | `basis` flips `evaluated`→`memoized` | **CONFIRMED end-to-end** |
| `fix-advance-write-lock-gate-scope` | b | a refused entry gate stops rendering `produces` | **CONFIRMED end-to-end** |

**Not one rejection was a hallucination.** Six for six.

### The two behavioural claims, reproduced by running the real thing

A probe test drove a genuine `advance --apply` through a passing memoized verify
gate and read the recorded event. The only thing varied was the fix's own
pre-flight line.

| | recorded `gate_result.basis` |
| --- | --- |
| with the fix (pre-flight present) | **`memoized`** |
| pre-flight removed (pre-fix) | **`evaluated`** |

And with a *failing* gate, the item directory after the throw:

| | contents |
| --- | --- |
| with the fix | `discover.md, plan.md, spec.md, story.md` |
| pre-fix | the same **plus `implementation-report.md`** |

Both are real, undocumented behaviour changes shipped by an approved fix. Two
arms found the first one independently.

### Why the reviewers could see this and the suite could not

Four of the six are **mutation survivors**: the reviewer mutated the code and
watched the suite stay green. That is S0.1's methodology, applied unprompted, by
an agent that was only asked to review. It is the same behaviour F17 recorded
when a reviewer mutation-tested a fixture on its own initiative.

The suite cannot see the `basis` flip for a structural reason worth recording:
`gateResults()` in `test/advance-lock-scope.test.ts` reduces each event to
`{stage, pass}` and **never captures `basis`**. The fix's own tests pin the
"reusing the verified result" *warning* but not the recorded basis it implies.

### What it does to E3's numbers

If every finding is accepted as blocking, both arms' false-positive rates go to
**0/23**, MCC to **+1.000** for both, and the arms become identical on every
measured axis — leaving the 1.72× cost difference as the sole discriminator.
**That is the upper bound, not the result.** The lower bound is the reported
4/23 and 2/23. The truth is in between and is Ryan's call, because four of the
six are "the test is vacuous" rather than "the code is broken", and a reviewer
may legitimately treat those as advisory rather than blocking.

**The consequence for CLEAN's ground truth is the same either way, and it is the
bigger finding:** five shipped-and-approved items carry defects their own tests
do not pin. METHOD-E3 §5 defines CLEAN's ground truth as `approved` because the
item shipped and a historical review passed it. **That definition is now known
to be wrong for at least five of 23 items (22%).** Any false-positive rate
measured against it is an upper bound on the true rate, for both arms.

### Limits of this adjudication

1. **Severity is not adjudicated** — only factual truth.
2. `issue-74`'s **first** issue (the schema-less `safeAgent` dispatch) was not
   replicated; only its second (the strictness mutation) was. The item's
   rejection stands on the replicated one.
3. The replications use the reviewers' own cited mutations. A mutation is
   evidence a test does not pin a behaviour; it is not evidence the behaviour is
   wrong.
4. **Two of my own replication attempts were invalid before they were right.**
   The first `issue-146-147` run reported a failure that was an artefact of my
   edit converting LF→CRLF and breaking the extraction regex; the first
   `issue-63` mutation broke the TypeScript transform through bad escaping.
   Both were caught by reading the failure rather than the summary line. This is
   the same line-ending class as F20's `\\?\` path bug and is now the third
   occurrence in this programme.

---

## F25 — ROOT CAUSE: Red→Green is evidenced at TEST granularity, but an AC is a claim about ONE ASSERTION

**Established by:** the corpus's own review and report artifacts plus targeted
replication, 2026-09-04. Zero model cost.
**Question:** why did a process that mandates a failing test per AC ship five
items whose ACs no test pins?

### What it is not

**Not a detection-technique gap.** 11 of 23 historical reviews used mutation
testing. Cross-tabulated against the items E3 rejected: 3 of 12 non-mutating
reviews, 2 of 11 mutating ones. The rates are the same and the hypothesis is
dead.

**Not uniformly a miss, either.** `issue-63`'s historical reviewer **found the
exact defect**, wrote it down —

> "the fixture uses 2 required items with only ONE left unticked, while the AC's
> GIVEN specifies '2+ required items unticked' … This narrows the AC-001 pin
> without invalidating it"

— docked `ac-coverage` to 2/3, and approved anyway. The other four scored
`ac-coverage 3/3` and `test-integrity 3/3`. So one case is a graded-rubric
judgement and four are genuine misses; no single story covers both.

### The mechanism, for the four genuine misses

`forge-fix` mandates **Red → Green: a failing test per AC**. The
`implementation-report.md` records it in a TDD Evidence table, and the reviewer
checks that table. Both are honest. Both are satisfied by a test that does not
pin its AC.

**Because a test is red if ANY of its assertions fails, and an AC is a claim
about ONE of them.**

`issue-146-147` AC-015 is the clean demonstration. Its test carries two
assertions:

```js
expect(flat).toMatch(/redkiln record-links/);        // does the instruction exist?
expect(flat).toMatch(/after[^.]{0,40}commit/);       // is it ordered after the commit?
```

| probe | result |
| --- | --- |
| remove step 7 entirely (the pre-fix state) | **RED** — on assertion 1 |
| keep step 7, invert the ordering to "FIRST, before the commit" | **36/36 GREEN** |

So the test genuinely was red before the fix, and the report says so:

> "AC-015 … **RED pre-fix, same static proof (the ordering words did not exist)
> → GREEN**"

That sentence is literally true and quietly conflates the two assertions: the
redness came from the instruction being *absent*, not from the ordering being
*wrong*. **A new feature's test is trivially red pre-fix because the feature
does not exist.** Red→Green is therefore satisfied for free, and the AC's actual
claim is never exercised.

This generalises to the other three, all of which pin behaviour their fix newly
introduced — a `Path`-header guard, a `=== true` strictness check, an ordering
instruction. Each is red pre-fix because the feature is absent; each survives
mutation of the specific claim.

### The corpus already knows the answer, and applies it inconsistently

The same `issue-146-147` report marks a *different* criterion:

> "AC-017 … RED pre-fix → GREEN, and **mutation-proven**"

The author mutation-proved AC-017 and not AC-015. Nothing in the process
requires it per AC, so it is applied where someone thought of it. **Mutation
against the POST-fix tree is the only evidence that an assertion discriminates;
redness against the PRE-fix tree is not, and the two are recorded in the same
column as if they were.**

### Remediation, cheapest first

1. **Make the Red claim assertion-granular.** The TDD Evidence table should name
   *which assertion* went red and why. AC-015's row could not have been written:
   its ordering assertion never failed. This is a template change plus a
   reviewer check, and it costs nothing to run.
2. **Require post-fix mutation per AC, not per story.** One line per AC: the
   mutation applied and the test that went red. The corpus already does this for
   some ACs, so it is a codification rather than an invention.
3. **Give the review card the operational form of its own question.** It already
   asks "is there a test that would fail if the behaviour regressed?" — the
   answer is only knowable by mutating. E3's arms did this unprompted on five
   items; the card should require it.
4. **Bind the rubric to the verdict** for the `issue-63` class: today a reviewer
   can dock `ac-coverage` to 2/3 for an AC whose pin is narrower than the AC and
   still return `approved`. Either that deduction should block, or the card's
   "untested → changes-requested" clause should say that a narrowed pin is not
   "untested". At present the card contains a hard rule and a graded instrument
   that quietly absorbs findings the hard rule would block on.

### The finding that complicates all of the above

**A CLEAN rejection did not replicate.** `issue-146-147` arm (b), same fixture,
same prompt, same model and effort:

| repeat | verdict |
| --- | --- |
| 1 | `changes-requested` — found the vacuous AC-015 test |
| 2 | **`approved`** |

The reviewer that catches this catches it *sometimes*. That means the historical
review and E3's arms may be **two draws from one stochastic process rather than
two different competences** — "the historical reviewer missed it" and "E3's arm
found it" are then the same reviewer on different days. It also means every rate
in F23 and the adjudication is a single draw, and the remediation above is worth
more than any per-arm number, because it converts a probabilistic catch into a
required procedure.

---

## F26 — E3-H RESULT: 20/20 across two harder tiers. The prediction was falsified twice, and the line is closed

**Established by:** 20 scored dispatches — 5 seeds × 2 arms × 2 difficulty tiers
— 2026-09-04, Opus/`xhigh`, corpus pin `47faec9f`. `spikes/results/e3h.jsonl`.
**Spend $36.18.** Pre-registered in `METHOD-E3H.md` (tier 1) and its §9
amendment (tier 2), each written before the dispatches it governs.

### Every cell is at ceiling

| tier | arm (a) in-session | arm (b) headless |
| --- | --- | --- |
| **S-SILENT** (E3, n=23) | 23/23 | 23/23 |
| **S-SUBTLE sighted** (n=5) | 5/5 | 5/5 |
| **S-SUBTLE blind** (n=5) | 5/5 | 5/5 |

20/20 records cleared every check. Arm (a): `dispatched_card` 10/10,
**`parent_inspected` 0/10** — no parent reviewed for itself in either tier.

### Two levers, two falsified predictions

**Tier 1 — subtler defect.** The fix fully present, one line wrong, every seed
verified to leave the affected suite green. Predicted: "detection will fall
below 23/23 — probably well below". **It did not move.**

**Tier 2 — withhold the checklist and the pointer.** No AC enumeration, no fix
file list; the reviewer must locate the code itself in a 236-test-file repo,
against seeds chosen because the suite does not react to them. Predicted, with
stated lower confidence after being wrong once: detection falls. **It did not
move.**

Both predictions were recorded before their data. That is the only reason this
entry is a result rather than a rationalisation.

### What DID move: cost, and it is the interesting number

| | median wall |
| --- | --- |
| sighted | 497s |
| blind | 585s (**+18%**) |

The median understates it. Per seed, arm (b) sighted → blind:

| seed | change |
| --- | --- |
| `issues-135-129` | 358s → 1201s (**+236%**) |
| `issue-75` | 158s → 353s (+123%) |
| `issue-63` | 599s → 974s (+63%) |
| `issue-146-147` | 1359s → 1395s (+3%) |
| `issue-74` | 614s → 230s (−62%) |

**Removing the pointer made the reviewer search — sometimes three times as hard
— and it still found the line every time.** Localisation was never the binding
constraint. That is the substantive finding of this tier, and it is more
informative than the detection rate it was designed to move.

### The single most striking record

`issue-75` is the one seed not of my construction: S0.1 found it as **the lone
mutant redkiln's entire affected-area gate failed to kill**. All four arm/tier
cells caught it — including blind, where nothing pointed at
`src/schema/common.ts`. **The reviewer found what the test suite could not, four
times out of four, twice without being told where to look.**

### The pre-registered conclusion, applied rather than avoided

METHOD-E3H §9 committed in advance:

> if detection does not fall under tier 2 either, the honest conclusion is that
> this corpus cannot produce a discriminating review task by prompt manipulation
> alone, and the programme should stop trying.

**That condition is met. The difficulty-escalation line is closed.** A tier 3 is
not proposed. Charter §12 names the failure mode this avoids — building
instrument instead of reporting findings — and inventing a fourth lever after
two falsified predictions would be exactly that.

### What this does and does not license

**Does:** the reviewer detects seeded defects at ceiling across every
construction this corpus supports — absent capability, subtle wrong line, and
subtle wrong line with no pointer — in **both** dispatch shapes. The RFC's
fear that headless dispatch degrades review quality has now survived three
independent attempts to make it visible.

**Does not:** establish that the arms are equal in general. Every tier saturated,
so none of them could rank the arms even in principle. E3's conclusion is
unchanged and is the ceiling of what the programme can claim — **quality does not
discriminate; only cost does** (1.72×).

**And it does not establish that the reviewer is reliable**, which is the live
threat: F25 records a CLEAN verdict flipping across three identical runs (2 of
3). Every cell above is a single draw. A 20/20 built from single draws of a
process known to vary is consistent with a per-item catch rate well below 1.0,
and nothing here bounds it.

### Threats, restated

1. **n = 5 per cell.** Ceiling results at this n are compatible with a true rate
   anywhere above roughly 0.55.
2. **Four of five seeds are ones I chose because reviewers had already found
   them** in E3's CLEAN condition — circular by construction. Seed 5 is not
   mine, and it is the one carrying the weight.
3. **Repetition.** Both arms had seen all five items in E3. Affects both arms
   equally; inflates the absolute rate, not the comparison.
4. **"Blind" withholds the checklist and the pointer, not the criteria** —
   `spec.md` still states them in prose. Stated in §9 before running, and worth
   repeating because the label overclaims.

---

## F27 — E0 VARIANCE: detection is stable, the verdict is not, and half of E3's arm disagreements were within-arm noise

**Established by:** 33 repeat dispatches, 2026-09-04, Opus/`xhigh`, corpus pin
`47faec9f`. `spikes/results/e0-stability.jsonl`, held in a **separate file** so
repeats can never inflate E3's n. Pre-registered in `METHOD.md` (E0), amended
that day to n=3 breadth-over-depth, disclosed before the data.

### The result splits cleanly along one line

| what varies | n | flaky items |
| --- | --- | --- |
| **detection** (`s-silent`, strong detection) | 7 items × 3 draws | **0 / 7** |
| **verdict** (`clean`, approve vs reject) | 4 items × 2–3 draws | **2 / 4** |

**Detection did not vary once in 21 draws** — exact one-sided 95% lower bound
**86.7%** on the per-draw detection rate. Every item classified *always-detect*.

**The verdict varied on half the items measured.**

| item | draws |
| --- | --- |
| `issue-146-147-boundary-window` | REJECT / approve / REJECT |
| `issue-63-advance-gate-diagnostic-truncated` | approve / **REJECT** |
| `fix-advance-write-lock-gate-scope` | REJECT / REJECT |
| `issue-62-new-stale-pinned-template` | approve / approve |

### The finding that reaches back into E3

E3 recorded **four CLEAN disagreements between the arms**, and they were the
entire basis for arm (a)'s 4/23 versus arm (b)'s 2/23 false-positive rates. Two
of those four are now demonstrated to be **within-arm variance, not an arm
difference**:

| item | E3 read as an arm difference | now known |
| --- | --- | --- |
| `issue-63` | a=REJECT, b=approve | **arm (b) flips approve→REJECT on its own** |
| `issue-146-147` | a=approve, b=REJECT | **arm (b) flips REJECT→approve→REJECT** |
| `issue-74` | a=REJECT, b=approve | stability unmeasured |
| `issues-135-129` | a=REJECT, b=approve | stability unmeasured |

**Arm (b), re-run on `issue-63`, produces arm (a)'s E3 verdict.** The "difference
between the arms" on that item is a difference between two draws of the same
arm. McNemar already said the FP gap was not separable from chance (p = 0.625);
this shows *mechanically why*.

**Consequence:** neither arm's false-positive rate is a property of the arm. E3
(F23) and the adjudication (F24) both report per-arm FP numbers — 1/23 and 0/23
after adjudication — and **those numbers should now be read as single draws of a
process that flips on half the items where it was checked.** The conclusion they
support ("quality does not discriminate the arms") is *strengthened*, because
the residual difference has been shown to be noise. The numbers themselves are
weakened.

### Why the split makes sense

The two measurements are not the same kind of judgement.

- **Detection** asks: is this criterion met by the code? On S-SILENT the answer
  is grossly no — the capability is absent. That is a robust signal and it
  reproduces.
- **The verdict** asks: does this finding rise to blocking? On CLEAN the
  candidate findings are *marginal by construction* — vacuous tests, narrowed
  pins, an undocumented side effect. That is a judgement at the threshold, and
  thresholds are where a stochastic process visibly wobbles.

The pattern within CLEAN supports this. The stable-reject item,
`fix-advance-write-lock-gate-scope`, is the one with a **live behavioural
regression** reproduced end-to-end (F24). The two flaky items rest on
**test-quality** findings. **Strength of the underlying defect appears to predict
verdict stability** — offered as a hypothesis on n=4, not a result.

### What this means for the programme

1. **The 23/23 headline survives, and is now the best-supported number in the
   spike.** Detection is stable across every item measured.
2. **Every false-positive rate is provisional**, in both arms and in the
   adjudication.
3. **The reliability question §7 of the amendment names as decision-critical is
   now partly answered, and the answer is uncomfortable:** a review gate that
   detects reliably but *blocks* unreliably will pass different work on
   different days. For a gate whose purpose is to stop work advancing, verdict
   stability is the property that matters, and it is the one that varies.
4. **This is a finding about the reviewer, not about inversion.** It affects
   both arms identically and is inherited by any deployment shape.

### Limits

1. **n=3 at best, n=2 on two items.** A "stable" classification here means "did
   not flip in 2–3 draws" and is compatible with a real flip rate up to ~30%
   *[corrected 2026-09-11: up to ~56% (n = 3) / ~66% (n = 2), Wilson 95% — Correction (2026-09-11) below]*.
2. **Only arm (b) was repeated.** Arm (a)'s stability is unmeasured; there is no
   reason to expect it differs, and that is an assumption, not a finding.
3. **Two of the four E3 disagreements remain unexplained** — `issue-74` and
   `issues-135-129` have no repeat data, so the within-arm-noise account covers
   half the disagreements, not all of them.
4. **`s-subtle` stability is unmeasured.** F26's 20/20 is still 20 single draws.

---

### Correction (2026-09-11)

Audit PIA-073. Limits 1's "compatible with a real flip rate up to ~30%" was not computed. A classification that did
not flip in n draws is compatible, at the registered two-sided Wilson 95% interval, with a true flip rate up to
**~56% at n = 3** (0/3 → [0.000, 0.561]) and **~66% at n = 2** (0/2 → [0.000, 0.658]) [measured, recomputed
2026-09-11; the audit's figures agree]. "Stable" at this n is therefore an even weaker claim than the original
sentence allowed, which is the direction the finding already argued (nothing inferred from a single draw; S-VAR at
n = 10). `METHOD-S.md` §1.1 restates the ~30% and carries a dated amendment; ADR-0074 and ADR-0101 (ex-0088) carry
the correction by erratum.

---

## F28 — H1/H4 RESULT: a human gate costs a bounded 18.6k tokens however long it lasts, and the digest carries everything the next step uses

**Established by:** 3 TTL reads (haiku) and 15 step-2 dispatches (Opus/`xhigh`),
2026-09-04/05, corpus pin `47faec9f`. `results/h1-gap.jsonl`, `results/h4.jsonl`.
**Spend: $18.85.** Pre-registered in `METHOD-H.md` §3 and §6, the latter amended
before any H4 dispatch *[corrected 2026-09-11: amended after three arm (a) draws, disclosed — Correction (2026-09-11) below]*.

RFC §7 calls the mid-run human approval gate the **unoccupied market gap** — the
capability that would justify building Redkiln-RS at all. These are the first
measurements of what it costs.

---

### H1 — the cost of a suspension is a step, not a curve

A review-scale prefix (~36k tokens: spec, report and diff for a real item),
warmed and read back at three gaps:

| gap | cache-read share | cache creation on resume |
| --- | --- | --- |
| 0.9 min | **1.000** | **0** |
| 65.2 min | 0.484 | **18,603** |
| 2,982 min (**49.7 h**) | 0.484 | **18,603** |

**The 65-minute and the 49.7-hour reads are identical** — but see the correction below: the mechanism is not what this paragraph originally claimed. The prefix's two
segments (F13) behave completely differently across time: a volatile ~18.6k
expires somewhere under an hour — consistent with the ~1h TTL of F10 — and the
remaining ~17.5k **survives two days untouched**.

So the cost of a human gate is not proportional to its length. It is **zero
below the TTL and a flat 18.6k above it**, and a gate lasting two days costs
exactly what a gate lasting sixty-six minutes costs.

**METHOD-H §3's decision rule fires cleanly:** "a per-gate cost under ~25k tokens
is noise; batch gates freely." 18,603 is under it, at a gap beyond the observed
p90. **Gate placement is not a cost decision.** F18 established that ~half of all
gates outlive the cache TTL; this establishes that it does not matter that they
do.

*Not measured: the `--system-prompt` secondary. The flag is now plumbed through
`ttl-mark`/`ttl-read`, and F16's result across commits makes it plausible that it
removes even the 18.6k — but that is untested and must not be assumed.*

### H4 — the digest carries everything the next step uses

METHOD-H §6 as amended. **The registered comparison could not have worked:**
every step under inversion is its own `claude -p` process, so a step "after a
suspension" and one run "contiguously" receive exactly the same thing. Running it
would have returned "no divergence" for a reason unrelated to the gate.

What was run instead tests §6's stated intent — is the structured digest a
sufficient carrier? A real recorded step-1 review is replayed into a second step
under three arms:

| arm | step 2 receives | mean Jaccard vs DIGEST |
| --- | --- | --- |
| **DIGEST** | the structured digest alone | — (reference) |
| **FULL** | the same digest **+ the reviewer's full prose** | **1.000** |
| **CONTROL** | the digest again | **1.000** |

**Effect: +0.000.** The handover nearly doubled — 11,560 → 21,958 prompt bytes —
and the second step named **exactly the same files, on all five items**.

Step 1 was replayed from `results/e3.jsonl` rather than re-run, so step-1
variance is removed from the comparison entirely, and the CONTROL arm exists
because F27 showed verdicts flipping across identical runs: without a same-batch
baseline, a null result would be unattributable.

**Conclusion for the design.** For the decision the next step actually makes —
which files to change — the digest is sufficient, and the reviewer's prose is
dead weight in the handover. RFC R1 (context fidelity at the gate boundary) is
**not** a constraint here, and the gate is free in context as well as nearly free
in cache.

### Correction and completion, 2026-09-05 — the step was real, but I had the mechanism wrong, and `--system-prompt` makes it worse

**What was wrong.** The entry above read the three H1 gaps as pure time decay.
The fixture sits **inside the redkiln-rs working tree** and this programme was
committing continuously, and the default system prompt carries a git snapshot
(F16) — so a commit inside the gap invalidates the prefix regardless of elapsed
time. Commits fell inside exactly the gaps that showed re-creation:

| gap | commits inside | creation |
| --- | --- | --- |
| 0.9 min | 0 | 0 |
| 65.2 min | 1 | 18,603 |
| 49.7 h | many | 18,603 |

Both readings fitted. **The original entry asserted one of them without the
evidence to separate it from the other.**

**The discriminating run.** Two marks in the same fixture, both read at ~66
minutes. `--system-prompt` removes the git snapshot entirely, so it cannot be
invalidated by repository activity — whatever it loses, it lost to time.

| arm | gap | share | creation | prefix |
| --- | --- | --- | --- | --- |
| default system prompt | 66.6 min | 0.484 | 18,603 | ~36,160 |
| **`--system-prompt`** | 66.8 min | **0.000** | **29,680** | ~29,680 |

**Time decay is real.** The `--system-prompt` prefix went *fully* cold in 66
minutes with no repository activity able to touch it, which is exactly F10's ~1h
TTL observed on a review-scale payload for the first time.

**And the mechanism behind the "step" was not durability — it was subsidy.** The
default-prompt prefix retained 48% across both 65 minutes and 49.7 hours because
that portion is the **shared default system prompt**, which F13 measured as ~69%
directory-independent, and which this programme's own constant unrelated
dispatching kept perpetually warm. The surviving ~17.5k was never surviving *on
its own*. In a quiet system it would decay too.

**So `--system-prompt` does not blunt time decay — it forfeits the shared-prefix
subsidy and costs more.** F16 measured it holding 1.000 across *commits*, and
that still stands; the inference that it would therefore help across *time* is
now falsified. The two mitigations point in opposite directions:

| threat | best mitigation |
| --- | --- |
| commits during a gate | **`--system-prompt`** — removes the volatile snapshot |
| elapsed time during a gate | **the default prompt** — rides the shared segment other work keeps warm |

**What survives of the practical conclusion.** A resume costs between 18.6k and
29.7k tokens on a ~30–36k review-scale prefix — at worst a full re-write, and
bounded by prefix size rather than by gate duration. METHOD-H §3's ~25k
"batch gates freely" threshold sits inside that range rather than clearly above
it, so the honest statement is weaker than the original entry's: **a gate costs
at most one prefix re-write, which is cheap in absolute terms and does not grow
with how long the human takes.** Gate *duration* is still not a cost decision.
Gate *frequency* now marginally is.

**A second instrument flaw, recorded because it defeated one arm.** A companion
`nocommit-65m` arm was meant to isolate elapsed time under the default prompt by
forbidding repository writes during the window. It is **confounded and its
result is not used**: the runner writes its own journal into
`spikes/results/` — inside the repository it is snapshotting — so the mark's own
output file changed `git status` before the read. Measuring a repository from
inside it requires the journal to live outside it, and `--out` makes that a
one-flag fix for any re-run.


**The `nocommit` arm is CLOSED UNMEASURED after two attempts, and both failures
are worth recording.**

*Attempt 1* was defeated by the instrument: the runner wrote its journal into
`spikes/results/` — inside the repository it was snapshotting — so the mark's
own output file changed `git status` before the read.

*Attempt 2* fixed that (`--out` to a path outside the repo, with the tree's
`git status` digest captured at mark time and verified after). The mark was
confirmed to leave the tree byte-identical. It was then defeated by **concurrent
work landing in the repository 22 minutes into the window** — not by the
instrument.

Its read is therefore a **third replication of the with-activity condition**, and
the determinism is the interesting part:

| gap | share | creation |
| --- | --- | --- |
| 65.2 min | 0.484 | 18,603 |
| 66.6 min | 0.484 | 18,603 |
| 67.3 min | 0.484 | 18,603 |

Three independent marks, three reads, **identical to the token**. Whatever the
prefix loses across an hour, it loses exactly and reproducibly.

**Not pursued further, for two reasons.** The mechanism question is already
answered by the `--system-prompt` arm — time decay is real, and the retained 48%
is the shared-segment subsidy — so a clean `nocommit` read would confirm the
account rather than change it. And two failures suggest the condition is not
holdable in a live repository, which makes it the *less* operationally relevant
of the two: a production human gate sits in a repo where other work continues,
which is the condition already measured three times.

### What the two together say about the differentiator

The human gate was the largest untested assumption in the thesis and the RFC's
claimed moat. It is now measured on both axes that could have made it expensive:

- **Cache:** bounded at 18.6k, flat, under the batching threshold.
- **Context:** the digest carries what is used; adding 10kB of prose changes
  nothing.

**Neither is a reason not to build it.** That is a genuine result and it lands on
the side of the RFC's own §7 argument — with the caveat below.

### The caveat, and it is the same one as everywhere else

**CONTROL scored 1.000, meaning the metric has no variance at all.** File
selection given a digest is deterministic, so the instrument could not have
detected a small effect — only a large one. "FULL equals CONTROL" is therefore
"adding prose did not perturb a deterministic output", which is weaker than
"prose never matters".

And the `plan` prose *does* vary between arms — by up to 5k characters, including
between DIGEST and its own CONTROL — so there is real variation in the second
step's output that the file-set metric does not capture. Whether any of it is
consequential is unmeasured.

**This is the fifth saturated metric in the programme** — S-SILENT detection,
S-SUBTLE detection, blind detection, and now H4 file agreement all pinned at
ceiling, alongside E0's detection stability. The pattern is now strong enough to
state as a finding about the method rather than about any one experiment: **the
tasks this corpus supports are ones the model does not fail at, so metrics built
on task success cannot discriminate.** Every discriminating result the programme
has produced came from *cost*, *stability*, or *mechanism* — never from success
rates.

---

### Correction (2026-09-11)

Audit PIA-089. "§6 … amended before any H4 dispatch" is wrong against git. `results/h4.jsonl` lines 1–4 are four
arm (a) (`a-contiguous`) H4 records stamped 2026-09-03T04:37Z, committed in **`7a796ce`** (2026-09-02 21:39 −0700);
the §6 amendment headed "before any H4 dispatch" is commit **`3eb93d5`** (2026-09-04 23:23 −0700) [measured,
`git log`, `git show 7a796ce --stat`]. So §6 was **amended after three arm (a) draws** (four records; lines 5–6 of
the file are the batch's two `batch_draw` headroom lines), and the amendment's heading overstated its timing. The
four records contribute to **no** figure in this finding: every H4 number above is from the 15 DIGEST / FULL /
CONTROL dispatches of 2026-09-05 (lines 7–21), and the arm (a) records were the registered comparison the amendment
declared vacuous. METHOD-H §6 carries the dated correction and states whether the draws were seen; the design's
"replaced before data" (:117) reads "amended after three arm (a) draws, disclosed".

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

## Continued in (2026-09-11)

This file stops at F28. Every later F-number is a dated draft under `spikes/` and is indexed here so that
architecture-design §10.2(c)'s "in FINDINGS.md" resolves by pointer (audit PIA-102). Drafts are records in their own
right; none is folded into this file.

| F | draft | spike | headline |
| --- | --- | --- | --- |
| F29 | `spikes/F29-DRAFT.md` | S-VAR | verdict stability at n = 10: no defect class is stable-reject |
| F30 | `spikes/F30-DRAFT.md` | S-H1b / S-WT | `--system-prompt` makes worktree fan-out free; the `h1b-24h` read is still not taken (drafted 2026-09-06, promoted 2026-09-11) |
| F31 | `spikes/F31-DRAFT.md` | S-TYPED | the typed contract costs no detection; blocks 9/23 on CLEAN with zero verified false positives |
| F32 | `spikes/F32-DRAFT.md` | P9 | authoring ergonomics, first author |
| F33 | `spikes/F33-DRAFT.md` | P9b | second-author check |
| F34 | `spikes/F34-DRAFT.md` | P6 authoring | third author, 90 activities |
| F35 | `spikes/F35-DRAFT.md` | P6 | catalog completeness over the lattice: 23.5% → 11.4% → 0.3%; dead 6/102 after run9 |
| F36 | `spikes/F36-DRAFT.md` | P1 arm L | the LLM-emitted context vector is not stable enough to drive selection |
| F37 | `spikes/F37-DRAFT.md` | P7 | the adversary → policy block costs 2.64×; criterion 1 fires while powerless |
| F38 | `spikes/F38-DRAFT.md` | P8 (partial) | resume is lossless across a real kill; cost ordering UNMEASURED |
| F39 | `spikes/F39-DRAFT.md` | P1 arm D | no derived facet clears the 0.95 bar; `auth` 0/12 |
| F40 | `spikes/F40-DRAFT.md` | P2 | the corpus holds no staleness to detect; recall has no denominator |
| F41 | `spikes/F41-DRAFT.md` | S-PORTS | ADR-0099's port shape compiles on the spike and pinned toolchains (rustc 1.97.1, 1.98.1): five sync `Arc<dyn Port + Send + Sync>` traits, a blocking `dispatch`, a root-owned `CancelToken`; 5/5 tests, 2/2 `compile_fail` doctests, clippy clean (row added 2026-10-02, s8i) |
| F42 | `spikes/F42-DRAFT.md` | S-GRAMMAR | under ADR-0103's grammar 285 of 287 live predicates parse to one tree and zero depend on precedence; the literal-versus-field rule is load-bearing on 191 sites; the two failures and three `on_false` values are pack lint errors, one in a shipped pack (row added 2026-10-02, s8i) *(dated 2026-10-03, s8i, R1FIX-8i: historical at pin `47faec9f`; the draft's s8a correction (ADR-0166) reads 279/279 now, the shipped pack clean at the `redkiln-pack-v0` tag)* |
| F3x (unnumbered) | `spikes/results/s-mig-report.md` | S-MIG | 96.0% of typed slots lift; the losses name three type decisions |

A later F-number is added here the day its draft is written.

### Correction (2026-09-11) — a reference-document citation

Grounding row EXT-CLAUDECO-021 (`references/audit/2026-09-10/census/grounding-by-file/spikes__FINDINGS.md.json`),
fetched 2026-09-10 from code.claude.com/docs/en/sub-agents and the Claude Code CHANGELOG. The charter's rationale
for the `claude ≥ 2.1.251` floor this file restates (F20's "The instrument moved" note; the RFC/charter corrections
table) says the subagent cap needs **2.1.219+**; the cap shipped in **2.1.217** ("Added a cap on concurrently-running
subagents (default 20, override with `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`)"); 2.1.219 is the nested-subagent
`parent_tool_use_id` forwarding. `promptCacheTtl` at 2.1.242 and `spend_limit` / `prompt_cache` at 2.1.251 are
confirmed. The floor conclusion (≥ 2.1.251) is unaffected [documented, fetched 2026-09-10]. This file never carried
the 2.1.219 figure; the row is recorded here because the file is the one that restates the floor.
