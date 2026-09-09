---
id: "0043"
title: Loops must be able to learn — iteration enters the memo key
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-13
source: references/2026-09-07-activity-graph-schema.md §9b amendment 13
supersedes: []
superseded_by: ["0059"]
related: ["0021", "0031", "0042"]
evidence: ["P9b A18 (cited in D21, architecture proposal §9)"]
consumers: []
---

## Context and problem

§5's step key is `hash(activity id@version, variant-relevant rigor, step id, resolved input digests, card digest (agent) or toolchain version + command (tool), injected KB atoms)` — it does not include an iteration counter. A step inside a `repeat`/`on_false` cycle that reads the same resolved inputs on a second pass therefore memo-hits and is never re-run, even when what it reads (telemetry, flags, clocks) has changed in the world since the first pass. The source calls this "a §5 design defect, not an ambiguity" — explicitly set apart from the 27 ambiguities the same batch raised — and names it as present in both P9's own `hypothesise` step and P9b's `observe` step.

## Decision drivers

- A loop whose body cannot observe anything new on each pass is not a loop that can learn; it can only ever re-derive the same memoised result.
- The defect was found in the schema author's own P9 catalog (`hypothesise`), not only in a newcomer's activity — it is a structural gap in §5, not a misunderstanding of it.
- The general architecture decision (D21) had already flagged the same failure mode at the proposal level ("iteration-blind key (rejected: loops cannot learn)"); this amendment is where the schema itself closes it.

## Considered options

### Option — iteration-blind key (status quo)
Leave the step key as §5 defines it, with no iteration counter and no live-state declaration. Rejected: per D21's own framing of the same choice, an iteration-blind key means loops cannot learn — a step that reads live state re-runs never, past the first pass, regardless of what changed in the environment.

## Decision

"The iteration counter enters the step key for every step inside a `repeat`/`on_false` cycle; `tool` steps that read live state (telemetry, flags, clocks) declare `memo = false`. Lint: a loop-back target with neither a feedback ref nor `memo = false` is an error." This applies to P9's `hypothesise` and P9b's `observe` alike.

## Consequences

**Positive**
- Closes the memo no-op failure mode: a loop body that reads live state, or that is targeted by a feedback ref (amendment 12), is now guaranteed to actually re-evaluate on each pass rather than silently short-circuiting.
- The lint rule gives authors a hard check instead of relying on them to notice the gap themselves, as the schema's own author did not on the first pass.

**Negative**
- The per-step `memo = false` declaration this decision introduces places the burden of noticing "this verb reads live state" on every author of every step calling that verb, individually — a duplication risk later addressed by amendment 29.

**What this forbids**
- Forbids a loop-back target (a step named by `on_false`/`repeat`) from having neither a feedback ref (amendment 12) nor a `memo = false` declaration — lint rejects such a graph outright, since it cannot learn under either mechanism.

**Partial withdrawal**
- Amendment 29 (ADR 0059) supersedes this decision's second sentence only: the per-step `memo = false` declaration on a `tool` step is replaced by a `live = true` flag on the verb's own registry entry, which now implies `memo = false` on every step using that verb. This decision's first sentence (iteration counter enters the key) and its lint rule remain in force unchanged.

## Evidence and confidence

- [documented] The source explicitly distinguishes this from the batch's ambiguities, calling it "a §5 design defect," and names concrete instances in both authors' catalogs (`hypothesise`, `observe`).
- [documented] The only identifier the corpus attaches to this defect is D21's parenthetical "(P9b A18)" in the architecture proposal §9, describing the same failure mode ("a telemetry or hypothesis loop over unchanged inputs is a memo no-op"). It is an ambiguity id from the P9b batch, not a finding id, and carries no measurement.
- [inferred] No count of how many activities besides these two named instances exhibited the defect is given; the claim is limited to what the source states about these two steps, not a broader measured rate across the ten-plus-three activities authored to date.
