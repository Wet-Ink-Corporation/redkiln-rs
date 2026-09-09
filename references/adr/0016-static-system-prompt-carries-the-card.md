---
id: "0016"
title: Static system prompt carries the card for Phase 1
status: accepted
date: 2026-09-07
legacy_id: D16
source: references/2026-09-05-architecture-proposal.md §4.3
supersedes: []
superseded_by: ["0078"]
related: []
evidence: ["F16", "S-WT", "S-H1b", "L6.4"]
consumers: []
---

## Context and problem

S-WT measured prompt-cache behaviour for a static `--system-prompt` carrying the card body: 1.000 hit rate across commits and across nine steady-state worktrees (w2–w10) after the first write, against 0.807 on the first worktree (w1, the cold write). The price of this stability is a full prefix rewrite (~30k tokens, review-scale) after the cache's ~1 h TTL expires, which forfeits the account-wide subsidy that otherwise keeps the *default* system prompt's head warm (S-H1b, with an F28 correction to the earlier reading). Phase 1 needed a concrete delivery mechanism for the card into the harness's system prompt before the alternative append route could be measured.

## Decision drivers

- S-WT's measured 1.000 cache hit rate across steady-state worktrees is a strong result for the static approach, once past the cold first write.
- The static prompt's ~30k-token rewrite after TTL expiry is a known, bounded cost, whereas the append route's benefit is unmeasured pending S-H1c.
- The cache is org-scoped: a CLI upgrade anywhere on the account invalidates the shared tool block, so the CLI version and TTL must be pinned regardless of which route is chosen.

## Considered options

### Option — append by default (unmeasured)
Use `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt` as the default route rather than a static system prompt. Rejected for Phase 1: no measurement exists yet (pending spike S-H1c); adopting it as default would be a bet without S-WT's evidence behind it.

### Option — heartbeat keep-alive (quota cost undocumented)
Keep the cache warm across the TTL boundary with a heartbeat request. Rejected: the quota cost of doing so is undocumented, so the trade against the ~30k rewrite cost cannot be evaluated.

## Decision

"`--system-prompt` = the card body (static)" for Phase 1; volatile input goes in the user message (stable prefix / volatile suffix). Pin `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and the tested CLI version, because the cache is org-scoped and a CLI upgrade anywhere on the account invalidates the shared tool block. The append route (`--exclude-dynamic-system-prompt-sections` + `--append-system-prompt`) is retained as a config option, pending S-H1c.

## Consequences

**Positive**
- Measured 1.000 cache hit rate across nine steady-state worktrees (w2–w10) and across commits, once the first (cold) write has occurred.

**Negative**
- Pays a full prefix rewrite (~30k tokens, review-scale) once per TTL expiry (~1 h) on resume.
- Forfeits the account-wide subsidy that keeps the default (non-card) system prompt's head warm, a cost the static approach does not get back.

**What this forbids**
- Forbids making the append route the Phase 1 default before S-H1c measures it.
- Forbids leaving the cache TTL or the CLI version unpinned, since the cache is org-scoped and shared across the whole account's tool block.
- Forbids a heartbeat keep-alive strategy while its quota cost is undocumented.

## Evidence and confidence

- [measured] S-WT: cache hit rate w1 = 0.807 (cold), w2–w10 = 1.000 (steady state), across nine worktrees.
- [documented] Post-TTL rewrite cost cited as "~30k (review-scale)"; this is the stated cost of the chosen static-prompt approach, not a measurement of the append alternative.
- [documented] S-H1b, with an F28 correction, establishes that the static approach forfeits the account-wide subsidy keeping the default prompt's head warm — the source does not quantify that forfeited subsidy in tokens or dollars here.
- This record does not license treating the append route as validated: S-H1c has not run, and the config option exists precisely because that measurement is still pending.
- [documented] Scope note: [0078](0078-cache-rule-is-split-both-ttls-set-scope-corrected.md) supersedes this record **partially** — this record stays `accepted` and carries `superseded_by: ["0078"]`. Two clauses are withdrawn: the single `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` pin as *the* TTL fix (0078 requires `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` as well, and per-run field evidence before any run may be reported as 1h), and the "the cache is org-scoped" justification for pinning the CLI version (the scope is narrower, and the mitigation that phrasing implies is withdrawn with it). The delivery clause above — `--system-prompt` carries the card body, static, for Phase 1, volatile input in the user message, append route retained as a config option pending S-H1c — is untouched and stands.
