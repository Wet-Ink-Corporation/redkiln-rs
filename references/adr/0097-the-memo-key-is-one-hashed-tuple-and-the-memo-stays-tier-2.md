---
id: "0097"
title: The step memo key is SHA-256 over one length-prefixed ordered tuple, the memo stays tier 2, and 0021's per-step memo = false clause is withdrawn with 0043's
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-008 (§3.3); references/adr/0021; references/adr/0059; references/adr/0086; references/2026-09-08-architecture-design.md §3.5 :324-325, §5.4
supersedes: ["0021"]
superseded_by: []
related: ["0022", "0043", "0059", "0086", "0105"]
evidence: ["PIA-008", "F19", "F38"]
consumers: []
---

## Context and problem

The memo record is specified three ways. ADR-0086 puts the memo table in SQLite — tier 2, rebuildable — while AC-161
and AC-164 require `doctor --rebuild` to reproduce "the same next action from git alone, after dropping tier-2
entirely" and AC-187 requires memo replay across a store loss; a tier-2 memo cannot satisfy either at step grain
[measured, PIA-008]. The key is described with five components in one list and seven in another, with no hash
framing, no canonicalisation of `resolved inputs`, and no statement of which `toolchain version` string enters it;
the idempotency key is three-part everywhere but the schema. And ADR-0021's rule that "`tool` steps that read live
state declare `memo = false`" — the second sentence of amendment 13, which ADR-0059 withdrew in favour of the verb's
`live = true` registry flag — is still demanded by ADR-0021's forbids block, AC-018 and design §3.5 [measured].

This record supersedes ADR-0021 **in part**: the per-step `memo = false` sentence is withdrawn (the same clause ADR-0059
withdrew from ADR-0043, now withdrawn from its D-series home too), and the key formula is pinned. ADR-0021's
per-step-key and loop-iteration-counter rules stand; ADR-0021 stays `accepted`. ADR-0086 is untouched: the memo stays
tier 2 and the criteria narrow.

## Decision drivers

- Two records demanding two mechanisms for liveness (a per-step flag and a per-verb flag) is one mechanism too many,
  and the register's AC-018 is currently testing the withdrawn one.
- AC-161's "same next action from git alone" is what the FSM decides at stage grain; the memo avoids recomputing a
  step, it does not decide anything the trailers do not already record.
- A key with no hash framing lets two implementers hash the same tuple to two values.

## Decision

**1. The memo stays tier 2, and the criteria say what tier 2 can promise.** AC-161 and AC-164 narrow to the
**stage-level next action**: `doctor --rebuild` from git alone reproduces the FSM's next stage transition and the gate
records the trailers carry; step outputs and memo hits are recomputed, not restored. AC-187 is scoped to a run whose
EventStore survives; a run whose store is lost replays from its trailer commits at stage grain. Design §5.4's tier table
gains the row "step memo — tier 2 — rebuildable by re-execution, never by rebuild"; AC-160's fixture drops the memo from
its rebuild assertion.

**2. The step memo key is `SHA-256` over a length-prefixed, ordered tuple of exactly these components**, in this order,
each encoded as `u32-length || bytes`:

1. `activity_id@version` (ADR-0072's version);
2. the rigor variant selected;
3. the step id within the activity graph;
4. the digest of each resolved step input, in declaration order, each input canonicalised by `rk-store`'s canonical
   serialiser (sorted keys, fixed float format, LF);
5. the card digest, **or** for a `tool` step the Toolchain adapter's detected version string plus the command;
6. the digests of the resolved KB atoms, sorted;
7. the loop iteration counter (0 outside a loop — ADR-0021's rule, unchanged);
8. the output type's `schema_version`.

AC-014 and AC-019 enumerate this list with one property case per component; `memo_hit` journals exactly it. The
**activity-level key** (AC-016) is a separately derived value — SHA-256 over the activity's reachable card digests
(ADR-0022) — and design §3.5 says the two are different formulas (:325 corrected to agree with :324). The
**idempotency key** is `(item, activity_id@version, attempt)`, three-part, in the schema as everywhere else; `attempt`
increments and `activity_committed` (AC-012) is emitted per step activation under ADR-0022.

**3. Liveness is the verb's `live = true` flag and nothing else.** A step declaring `memo = false` is a pack
validation error (AC-018 rewritten: GIVEN a `tool` step whose verb's registry entry carries `live = true`, WHEN the
step is memoised or declares `memo = true`, THEN pack validation fails; ADR-0059 on its decisions line). ADR-0021's
forbids bullet on the per-step declaration is withdrawn; README's "two homes" note lists 0021/0043 beside 0023/0020.

## Considered options

### Option — committed step results become tier 1
Render every committed step result into the store's typed artifacts and checkpoint it (one commit per committed step,
or a per-run sidecar at gate/exit), the SQLite memo an index rebuildable from git. AC-161 would then hold at step
grain. Rejected: it supersedes ADR-0086, puts a write on the tier-1 path per step (the path ADR-0018 and ADR-0105 keep
narrow), and buys step-grain rebuild for a property — "same next action" — that the FSM decides at stage grain.

### Option — keep both liveness mechanisms
Rejected: one is the other's withdrawn predecessor.

### Option — a plain concatenation key
Rejected: without length prefixes two different tuples can hash equal; without canonicalisation two serialisations of
one input hash different.

## Consequences

**Positive**
- One key formula, testable component by component; one liveness mechanism; criteria that promise what the tier
  provides.
- `doctor --rebuild`'s contract is stated at the grain the trailers actually carry.

**Negative**
- A run whose store is lost recomputes every step on resume; the cost is bounded by F38's measured resume (one
  step, n=3) and is unmeasured for many steps — Phase-1 exit criterion (3) is where it is looked for.
- The idempotency key's three parts mean a re-run of the same attempt is refused, which is what AC-013 wants and
  what `--resume` is refused for.

**What this forbids**
- Forbids a `memo = false` declaration on a step, and forbids a `live = true` verb's step being memoised.
- Forbids hashing the memo key over any tuple but the one above, or without length prefixes and canonicalisation.
- Forbids a criterion requiring `doctor --rebuild` to restore a step output or a memo hit from git.
- Forbids a fourth part on the idempotency key.

## Evidence and confidence

- [measured, PIA-008] the five- and seven-component lists, the three-part idempotency key everywhere but the schema,
  and AC-018/design §3.5's demand for the withdrawn clause.
- [measured, F19] 174/174 lossless suspensions at activity grain; the FSM was never replayed — the reason AC-161 is a
  stage-grain claim.
- [measured, F38] lossless mid-graph resume at one step, n=3, inside the TTL; the multi-step case is unmeasured.
- [documented, ADR-0059] the `live = true` flag "implies `memo = false` on every step using that verb".
- **What this evidence does not license.** It does not license claiming resume cost at many steps; it does not
  license claiming any key collision rate — the framing is design, not measurement.
