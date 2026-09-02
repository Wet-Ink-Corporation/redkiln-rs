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

## F10 — The prompt cache survives a 10-minute idle gap. TTL exceeds 5 minutes

**Established by:** E1-TTL probe, gaps 2 and 10 minutes, `claude` 2.1.258, haiku,
isolated (`--strict-mcp-config`), one directory, no commits, 2026-09-02.
Results: `spikes/results/e1-ttl.jsonl`. Pre-registration `45ecf29`; probe design
committed in `363ab2a` before it ran.

| gap | measured idle | cache_read | cache_creation | share |
| --- | --- | --- | --- | --- |
| 2 min | 2.1 min | 25,261 | 0 | 1.000 |
| 10 min | 10.1 min | 25,261 | 0 | 1.000 |
| **45 min** | **45.1 min** | **25,261** | **0** | **1.000** |
| 90 min | — | — | — | **not run** |

Every read was a **full hit with zero re-creation**, including after a measured
**45.1-minute idle gap** (17:04:03 → 17:49:07).

The 90-minute pair did not complete: its warm dispatch fired at 17:49:11 and the
run was stopped during the sleep, so no read was taken. One orphaned `ttl-90m`
warm record sits in the journal; resume keys on the *read*, so re-running the
90-minute gap redoes the pair correctly rather than skipping it.

**What this settles.** The effective prompt-cache TTL for `claude -p` on this
subscription **is at least 45 minutes** — nine times the platform's documented
5-minute default, and consistent with a 1-hour TTL. The pessimistic reading in PRIOR-ART §8 —
that the platform default 5m applies and the RFC's 1h-vs-5m asymmetry does not
exist — is **not supported**. The cache half of the inversion cost argument
survives this test.

**What it does not settle, stated plainly:**

- **n = 1 per gap.** F5 records cache-read share ranging 0.58–1.00 across
  identical dispatches. A single observation per gap is not a distribution.
  The zero-creation result is cleaner than a ratio (there is no partial state to
  be noisy about), but it is still one observation.
- **The prefix never went cold.** `cache_creation` was 0 on *every* dispatch
  including the first "warm" one, so this series never observed a cold→warm
  transition. It measures survival, not creation.
- **The upper bound is untested.** Survival at 45 minutes is consistent with a
  1-hour TTL but does not establish one. Only the 90-minute read separates "~1h,
  as the RFC assumes" from "longer than either the docs or the RFC suggest", and
  it was not taken.
- **`ttl_split` remains `[0, 0]`** on every dispatch, so the platform's own TTL
  field still cannot corroborate this. The finding rests entirely on behaviour.
- This says nothing about the *in-session subagent* side of the asymmetry, which
  the RFC takes from documentation and which we have not measured.

**Method note.** Each gap gets its own warm dispatch precisely because a cache
read refreshes the entry; probing one warmed prefix at increasing gaps would
measure a prefix being kept alive rather than one decaying undisturbed.

---

## F9 — E3's answer key is empty. `## Required Changes` contains no blockers

**Established by:** reading all 15 `## Required Changes` sections at the pinned
SHA, 2026-09-02. **Plan-breaking, same class as C1.**

E3's primary metric is *blocker recall*: "of the findings in the historical
`## Required Changes`, what fraction does each arm surface?"

**All fifteen sections open with "None".** Every one. Verbatim first lines
include "None. The verdict is `approved` and nothing in this review blocks the
project", "None blocking this verdict. The four items below are **carried
forward**", and "None. The verdict is `approved`."

The numbered items that follow are **carry-forwards to other projects** — work
for a downstream item, each written as a future check — not blockers against the
diff under review. Six of the fifteen have no list at all.

So the answer key E3 was going to be scored against does not exist. This is
charter §8.1's censoring problem, but sharper than §8.1 states: it names
`## Required Changes` as one of the three places negative signal survives, and
in fact that section exists precisely to record its *absence*.

**Consequence.** Blocker recall cannot be computed from this corpus. E3 needs a
different primary metric before it runs, and E6 — which shares the metric and the
inputs — inherits the same problem.

**Recommended replacement: seed the defects.** Rather than mining an answer key
that isn't there, construct one: inject known defects into real reviewed diffs
and measure detection per arm. The defect classes are not arbitrary — the
`redkiln-review` card enumerates exactly what it claims to hunt, so seed those:

- a double / no-op / injected stub standing in for a real contract
- a fixture-pinned assertion that pre-sets the answer a substrate should compute
- a `test.fixme` / skipped / flag-gated-off scenario
- an unmounted deliverable — built but never wired into the render path

This is the same methodology as S0.1 (mutation testing), applied to a reviewer
instead of a gate, and it has the property the historical corpus cannot offer:
**known ground truth**. It also tests the rubric the card actually claims,
rather than agreement with a past reviewer whose output was censored at the gate.

**Cost of finding this now:** nothing. **Cost of finding it after E3:**
roughly 40 Opus-heavy review dispatches, and a recall number that would have
been computed against an empty key and read as "the headless reviewer surfaced
nothing" — which would have fired the programme's hardest kill criterion on an
artefact of the corpus.

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
