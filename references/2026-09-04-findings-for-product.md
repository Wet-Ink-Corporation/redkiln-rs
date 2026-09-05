# Findings for product change — charter §14.5

**Date:** 2026-09-04. **Programme:** Redkiln-RS inversion spike.
**Instrument:** `claude` 2.1.258 → 2.1.259, Windows 11. **Corpus:** redkiln @
`47faec9f` (frozen).

Everything here was found while measuring something else. Each entry says how it
was established and what it would cost to act on. Evidence lives in
`spikes/FINDINGS.md` under the cited F-number; nothing is asserted here that is
not measured there.

Two audiences, separated because the actions are unrelated:
**Part A** is Claude Code platform behaviour. **Part B** is redkiln itself.

---

# Part A — Claude Code

## A1. `--json-schema` is silently inert under a tool-restricted `--agent` — **highest severity here**

**F20.** An agent card's `tools:` is an **allowlist**, and structured output is
delivered by a tool named `StructuredOutput`. A card that does not name it
removes the only channel the digest can return on, so `--json-schema` does
nothing.

**There is no signal that this happened.** Exit code 0, `subtype: "success"`,
`is_error: false`, no warning on stdout or stderr. The model answers in prose
instead — and in the isolating probe, the prose JSON did not even match the
schema it had never been shown, inventing `file`/`severity`/`message` where the
schema asked for `issue`/`files`.

| configuration | `stop_reason` | `structured_output` |
| --- | --- | --- |
| `--json-schema`, no card | `tool_use` | present, conforming |
| `--json-schema` + card `tools: [Read, Bash]` | `end_turn` | **absent** |
| `--json-schema` + card `tools: [Read, Bash, StructuredOutput]` | `tool_use` | present, conforming |

**Who this hits:** anyone composing a restricted agent card with structured
output — a natural combination for a verification or scoring agent. The failure
is invisible unless the caller checks for `structured_output` specifically
rather than parsing the text, which most callers will not do.

**Suggested fix, cheapest first:** warn when `--json-schema` is passed and the
resolved tool set excludes `StructuredOutput`; or implicitly allow
`StructuredOutput` whenever a schema is demanded; or document the coupling.

**Cost to us:** the entire E3 pilot dispatch, and it was initially
misattributed to a different defect.

## A2. `--allowed-tools` does not restrict the main turn

**F23 (arm a construction).** A `-p` dispatch given
`--allowed-tools Task,StructuredOutput` still reported **all 34 tools** at
`system/init`. Restriction was only achievable through an agent card's `tools:`.

**Consequence for us:** E3 arm (a) could not *prevent* its parent from reviewing
the tree itself; compliance had to be verified after the fact from
`parent_tool_use_id`. That worked — 0/46 parents inspected — but the check
should not have been necessary.

**Ask:** either make `--allowed-tools` restrict the main turn's available set,
or document that it does not and say what it does do.

## A3. Plan headroom is readable only through an undocumented event, and the limit that binds is invisible

**F1.** `result.rate_limit` was `null` on every dispatch. The only readable
headroom is a top-level `rate_limit_event` the docs do not mention, carrying
`unifiedWindows.five_hour` / `.seven_day` utilisation.

**Two asks.** Document the event, or expose the same data on the result. And
more importantly: **there is no Opus/Sonnet family sub-limit key.** Our
Opus-heavy runs are governed by a limit we cannot read, so a long batch cannot
throttle against the constraint that actually stops it — only against two
windows that do not bind.

Measured draws, for calibration: an Opus/`xhigh` review dispatch costs ~**0.03**
of `five_hour` when short (~130s) and ~**0.077** when long (~700s+).

## A4. A platform error returns a successful exit

**F21/F26.** Eight dispatches returned API **529 Overloaded** or **500**, with
`num_turns: 1`, `$0.00`, `subtype: "success"`, exit code 0 — and
`is_error: true` plus `api_error_status: 529` on the result object.

A caller checking exit status sees success. A caller checking `subtype` sees
success. Only `api_error_status` / `is_error` reveal it. We initially filed all
eight as *schema* faults — i.e. as the model failing — before noticing.

**Ask:** a non-zero exit, or a distinct `subtype`, for a dispatch that produced
no model output because the platform refused.

## A5. In-session subagents are not on a 5-minute cache TTL

Public guidance we relied on says subagents get a 5-minute prompt-cache TTL
"always". Measured across **56 dispatches** in which a headless parent
dispatches an agent card through `Task`:

| shape | 5m cache creation | 1h cache creation |
| --- | --- | --- |
| subagent inside a headless parent | **0** | 1,878,032 |
| direct headless | 0 | 2,476,973 |

**Limit:** the parent here is itself a `-p` dispatch, not a live interactive
session, so this does not settle the interactive case. But the blanket
"always 5 minutes" is not what this shape does.

## A6. Smaller platform notes

- **`--agents` without `--agent` is inert for the main turn** (F7) — it defines
  the card but nothing adopts it. It *does* register the card for `Task`
  dispatch, which is a useful and undocumented distinction.
- **Hooks fire in `-p`** (F2), which one documentation path implied they did not.
- **Cache usage fields are documented under two different names** (F3) —
  `ephemeral_cache_*` in the SDK reference, `ephemeral_1h/5m_*` in the costs
  page; the TTL split actually arrives nested under `usage.cache_creation`.
- **The prompt cache has two segments and ~69% is directory-independent**
  (F13) — a worktree costs ~31% of the prefix, not 100%. Useful for anyone
  reasoning about worktree-per-task designs.
- **`--system-prompt` preserves the cache across commits** (F16): steady-state
  cache-read share 1.000 versus 0.689 unmitigated, because it removes the
  volatile git-status section rather than relocating it.

## A7. Harness: background processes are killed unpredictably

**F22.** Across this programme, in one harness:

| launch shape | outcome |
| --- | --- |
| harness background job | killed, repeatedly, at any age |
| foreground under the tool limit | never killed |
| foreground exceeding the limit → auto-promoted to background | killed |
| detached `Start-Process` | survived 12,000s through ~14 killed watchers |

Cause not established. One item was killed three times as a background job and
completed in 116s in the foreground. This made long-running measurement
unreliable until work was moved outside the harness's job tree, which is not a
workaround most users would find.

---

# Part B — redkiln

## B1. Red→Green evidence is collected per TEST, but an AC is a claim about ONE ASSERTION — **the highest-value item here**

**F25.** `forge-fix` mandates a failing test per AC; the implementation report
records it; the reviewer checks it. All three are honest, and all three are
satisfied by a test that does not pin its AC — **because a test is red if any of
its assertions fails, and an AC is a claim about one of them.**

A new capability's test is red before the fix because the capability is absent.
That redness is free. Demonstrated on `issue-146-147` AC-015:

| probe | result |
| --- | --- |
| remove the instruction entirely (pre-fix state) | **RED** — on the "does it exist" assertion |
| keep it, invert the ordering the AC is about | **36/36 GREEN** |

The report recorded *"RED pre-fix, same static proof (the ordering words did not
exist) → GREEN"* — literally true, and it conflates the two assertions.

**Measured consequence: 5 of 23 fix-lane items (22%) shipped with a criterion
nothing pins**, each verified by mutation (F24).

**A remediation package is written and staged** at `spikes/remediation/`, not
applied because redkiln is this programme's frozen corpus. It adds
Assertion / Mutation / Test-run / Red? columns to the TDD Evidence table, makes
the reviewer re-run one mutation row rather than read it, and validates against
the real failure: AC-015's row comes out `GREEN`, a failing value, so it cannot
be completed. Cost is one scoped test file per AC (~30s), not the full suite
(~300s). **Unexamined:** `template_sig`/`rendered_sig` drift on existing items.

## B2. The review gate detects reliably but BLOCKS unreliably

**F27.** Repeated dispatches of the same reviewer on the same fixture:

| | flaky items |
| --- | --- |
| detection (is the criterion met?) | **0 / 7** — 21/21 draws |
| verdict (does this block?) | **2 / 4** |

`issue-146-147` returned `REJECT / approve / REJECT`; `issue-63` returned
`approve / REJECT`. **A gate whose purpose is to stop work advancing passes
different work on different days.** Detection is not the problem — the threshold
judgement is.

The one stable-reject item was the one with a live behavioural regression; both
flaky items rested on test-quality findings. **Defect strength may predict
verdict stability** (hypothesis, n=4). If it holds, the actionable form is to
separate "blocking defect" from "quality deduction" in the card so the marginal
class stops being a coin flip — the definitional half of that is in the staged
package.

## B3. There is no affected-area test selection

`verify.affected_gate` is `pnpm typecheck && pnpm lint && pnpm test` with no
`{{changed}}` placeholder, so **every gate run is the whole 236-file suite.**
This dominates wall clock everywhere it appears — reviewers that ran the gate
took 300s+ inside dispatches that otherwise take 130s. A one-line fix in
`issue-63`'s area runs the same suite as a workflow rewrite.

## B4. The AC ledger join is empty

All 110 `_ledger.md` files are under `_archive/`; none exists in the `support/`
fix lane. Charter E4's oracle — "AC ledger rows satisfiable" — is therefore not
computable, and the experiment had to be redesigned. If the ledger is meant to
be a durable per-AC record, the fix lane does not produce one.

## B5. The affected-area gate kills 90% of mutants — and the survivor is instructive

**S0.1/F11.** Reverting each fix's code while keeping its tests, the declared
gate killed 9 of 10 test-reachable mutants. **The one survivor
(`issue-75-item-frontmatter-strict-drift`, `src/schema/common.ts`) was caught by
the agentic reviewer in all four of four attempts**, twice without being told
which file to look at. That is a concrete case where review caught what the
suite could not, and it is the strongest single argument in this programme for
keeping an adversarial review gate at all.

## B6. `gateResults()` drops the field that would have caught a regression

`test/advance-lock-scope.test.ts` reduces each `gate_result` event to
`{stage, pass}` and never captures `basis`. The fix under review changed
`basis` from `evaluated` to `memoized` for every real `--apply` — reproduced
end-to-end — and the suite structurally could not see it. The fix's own tests
pin the "reusing the verified result" *warning* but not the recorded basis it
implies.

---

## What we would act on first

1. **A1** — silent `--json-schema` failure. Cheap to fix, invisible to the
   caller, and it destroys structured-output pipelines without a symptom.
2. **B1** — per-AC mutation evidence. Package written and validated; 22% of
   shipped fix-lane items are affected.
3. **A4** — platform errors returning success. Cheap, and it currently causes
   callers to blame the model.
4. **B2** — verdict instability. Needs measurement before design; the honest
   next step is more repeats, not a redesign.
