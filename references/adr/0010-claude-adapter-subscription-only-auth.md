---
id: "0010"
title: The Claude adapter authenticates by subscription only, never --bare, and asserts no API key
status: accepted
date: 2026-09-05
legacy_id: D10
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: []
related: ["0003"]
evidence: []
consumers: []
---

## Context and problem

The Claude Code CLI supports a `--bare` mode that disables OAuth and requires an API key instead of the subscription session; the platform docs and the spike charter (§6), the two sources the row cites, treat this as a real, selectable mode, not an edge case. The Harness port's Claude adapter (`rk-adapter-claude`) needs one fixed authentication story rather than silently accepting whichever credential happens to be present, since a config error here changes billing and account behaviour, not just connectivity.

## Decision drivers

- `--bare` "disables OAuth and requires an API key" (§4.3, charter §6, platform docs) — it is a different billing and auth surface from the subscription path the rest of the harness contract (rate-limit events, `isUsingOverage`, prompt-cache TTL) is written against.
- The crate plan (§6) commits `rk-adapter-claude` to be "subscription-only, asserts no `ANTHROPIC_API_KEY`" as a concrete, testable adapter behaviour, not just a policy statement.
- CI execution of the Claude adapter is left open by the row itself — "CI is a Phase 3+ question" — so the auth choice is taken without first settling how the adapter runs where no subscription session exists.

## Considered options

### Option — API-key mode option
Supporting `--bare`/API-key authentication as a configurable alternative to the subscription path. Rejected: the rest of §4.3's contract (rate-limit event handling, overage refusal, cache-TTL pinning) is designed around the subscription session's behaviour, and mixing in an API-key mode would require a second, divergent auth and quota story the design does not build for.

## Decision

The Claude adapter is "subscription only, never `--bare`; adapter asserts `ANTHROPIC_API_KEY` absent." Concretely: the adapter never passes `--bare`, and it asserts that `ANTHROPIC_API_KEY` is not set in the environment it dispatches from — an absence check, not merely a preference. "CI is a Phase 3+ question": no decision is taken here about how (or whether) the Claude adapter runs under CI; that is explicitly left open past Phase 1.

## Consequences

**Positive**
- The adapter's rate-limit and overage handling (`rate_limit_event`, `isUsingOverage`, margin-based dispatch refusal, §4.3) can be built against one known auth mode instead of two.
- The assertion against `ANTHROPIC_API_KEY` gives an explicit, checkable failure instead of the adapter silently behaving differently depending on which credential happens to be present.

**Negative**
- No path exists yet for running the Claude adapter in CI or any environment where only an API key is available — that gap is acknowledged and deferred, not solved.

**What this forbids**
- The adapter must never invoke `--bare`.
- The adapter must refuse to proceed (assert, not warn) if `ANTHROPIC_API_KEY` is present in its environment — an API-key-mode fallback is not an allowed degradation path.
- Decisions about CI execution of the Claude adapter may not be made under this ADR; it is explicitly punted to Phase 3 or later.

## Evidence and confidence

- [documented] "Never `--bare` on a subscription — it disables OAuth and requires an API key" is sourced to "charter §6, platform docs" in the §4.3 harness-contract table.
- [documented] The crate table (§6) restates the same behaviour for `rk-adapter-claude`: "subscription-only, asserts no `ANTHROPIC_API_KEY` (D10)."
- No spike or measurement backs this row — it is a policy/config decision, not one the source claims to have tested; no finding id is cited and none is invented here.
