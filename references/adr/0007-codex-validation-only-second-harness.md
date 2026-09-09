---
id: "0007"
title: Codex CLI is a second harness for validation only, not a gate-eligible adapter
status: accepted
date: 2026-09-05
legacy_id: D7
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: []
related: ["0003"]
evidence: ["S-CODEX"]
consumers: []
---

## Context and problem

The E3 review-quality rig had only ever been run against Claude Code, so the Harness port's abstraction was unproven against a second implementation — "Quality on a second harness" is listed among the architecture's own risks (§2), since "E3 is Claude-only". (The source does not say which §8 spikes constitute E3; it names only "E3 arm (b)" and "the E3 fixtures".) The design needed a concrete plan for a non-Claude harness without letting an unvalidated adapter into a review gate.

## Decision drivers

- The Harness port (§4.1) exists at a "named variation axis" only where a substitute has actually been named; Codex is named.
- A gate must not accept output from a harness whose detection/false-positive behaviour has not been measured against the same fixtures as Claude Code.
- `codex exec --json --output-schema` exposes a JSONL stream (`thread.*`, `turn.*`, `item.*`, `error`) that can be mapped onto the same typed contract as the Claude adapter (§4.3), so the mapping is specifiable now even though the adapter itself is not scheduled until Phase 4.

## Considered options

### Option — Pi
Considered as an alternative second-harness target. Not chosen; the source names it only as a rejected alternative without further detail — no rationale beyond "Codex CLI" being the choice is recorded.

### Option — defer
Deferring the second-harness question entirely until Phase 4 planning began in earnest. Rejected in favour of registering the validation work now: S-CODEX is a live §8 spike (46 Codex dispatches, feeding "Harness port viability; registry contents; Phase 4 adapter"), so the port's portability is measured in the current programme even though the adapter itself is not scheduled until Phase 4. The source records no further rationale for this rejection.

## Decision

"Codex CLI, validation only (S-CODEX); no non-Claude adapter in a gate until it passes E3-shaped checks." The Codex adapter maps the same Harness contract onto `codex exec --json --output-schema <file> -o`; its JSONL stream is classified by the same error taxonomy as the Claude adapter. Capability gaps relative to Claude Code (effort pinning, per-agent cards, tool allowlists) are recorded as dated registry entries, not assumed away. Per the phasing table (§7), the Codex adapter ships in Phase 4 ("reach"), and only after S-CODEX passes.

## Consequences

**Positive**
- The Harness port's abstraction is exercised by a second, materially different CLI (JSONL shape, capability set) before Phase 4 planning locks in on it.
- Capability gaps are tracked explicitly (dated registry entries) rather than silently assumed compatible.

**Negative**
- A second harness is exercised on the same fixtures (S-CODEX, costed at 46 Codex dispatches) before it is known whether it can be trusted to gate anything, i.e. spend that may not convert into a shippable Phase 4 adapter.

**What this forbids**
- No non-Claude adapter — Codex included — may be the harness behind a review or verification gate until it has passed E3-shaped checks (S-CODEX). An unvalidated harness producing a `Findings`/verdict that closes a gate is exactly what this decision rules out.

## Evidence and confidence

- [documented] The Codex adapter's wire mapping (`codex exec --json --output-schema`, `thread.*`/`turn.*`/`item.*`/`error` event classification) is specified in §4.3 as a direct extension of the Claude adapter's typed-error contract.
- [documented] S-CODEX is costed at 46 Codex dispatches and is explicitly marked "Gates Phase 1? no" in the §8 spike table — it feeds "Harness port viability; registry contents; Phase 4 adapter," not the Phase 1 exit criterion.
- The source gives no result for S-CODEX as of this ledger snapshot (2026-09-05/rev. 4); this ADR records the decision to build and validate the adapter, not a claim that S-CODEX has run or passed. The decision to admit Codex into a gate remains conditional on that spike, which is not yet reported.
- [documented] The crate table (§6) lists `-codex` "after S-CODEX", and the Phase 4 row reads "Codex adapter (only after S-CODEX passes, D7)" — so the shipped adapter follows the spike; what exists before it is the §4.3 mapping and the S-CODEX run itself.
- [inferred] "validation only" is read here as: Codex is exercised through the E3 fixtures but is never behind a gate until S-CODEX passes. No separate row states this mechanism beyond the quoted decision text, and the source names no criterion for what "passes" means.
