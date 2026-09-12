---
id: "0096"
title: A finding's repro is typed — run or mutate, with the mutation and suite named — and a surviving mutant blocks only when it discriminates
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-007 (§3.2), PIA-056 (§4.1), PIA-097 (§4.4); references/adr/0074 clauses 1 and 2; spikes/PRIOR-ART.md §5; spikes/runner/src/e3.rs :175, :254-256
supersedes: ["0074"]
superseded_by: []
related: ["0004", "0019", "0080", "0082", "0104"]
evidence: ["PIA-007", "PIA-056", "PIA-097", "F31", "F29", "F25"]
consumers: []
---

## Context and problem

ADR-0074 clause 1 fixes the finding shape as `{ ac, claim, evidence, rationale, files[], repro { command, observed,
expected_when_met } }` and says "every finding is an executable claim". Clause 2 computes the evidence class from that
record: `mutation-survived` "iff a mutation was applied and the named suite stayed green". AC-289 re-applies "the
named mutant" at the reviewed SHA and re-runs "the named suite"; AC-305's identity tuple carries a "mutant id". The
shape has no field for any of them [measured, PIA-007]. In the measured S-TYPED payloads the `command` field is
described as "the exact command **or code change** you ran" (e3.rs:175); roughly 24 of 29 `mutation-survived` commands
describe the mutation in prose and 111 of 176 blocking-grade `command` values do not begin with an executable
[measured, refuter recount]. Under AC-290 those become `cannot_run` → `Unverifiable` → advisory: the gate fails open on
exactly the class ADR-0074 built it to block. Step 5.3 snapshots `rk-types::Findings` before the classifier exists, so
the shape has to be right before the snapshot.

Clause 2 has a second hole: an **equivalent mutant** — one that changes the program's text and not its behaviour —
survives re-execution by construction, so `mutation-survived` can block on a no-op the machine cannot filter
[measured, PIA-056; documented, PRIOR-ART §5's equivalence rule]. And the block counts, sink seeds and typed-arm
scoring F31 reports were produced by `policy_verdict` calling `policy.blocks(evidence)` on the **model-emitted label**
(e3.rs:254-256), the routing clause 2 rejects; the Rust-computed class is unmeasured [measured, PIA-097].

This record supersedes ADR-0074 **in part**: clause 1's repro shape and clause 2's `mutation-survived` definition.
Clauses 3–8, the policy table, `Quarantined`, the negative-control rule and every forbids bullet stand; ADR-0074 stays
`accepted`.

## Decision drivers

- A gate that cannot run what it is asked to re-execute does not fail closed; it classifies everything it cannot
  parse as advisory.
- Two implementers reading AC-289 against clause 1 build different things: one extends the schema, the other executes
  `command` verbatim. The snapshot fixes one of them.
- An equivalent mutant is indistinguishable from a real one by re-execution alone; the discriminating signal has to
  be in the record.
- The 52/52 conformance figure (F31) was measured against the old shape and must be re-tagged, not silently kept.

## Decision

**1. `Findings.repro` is typed.**

```
repro {
  kind: run | mutate,
  command: String,                    # a shell string; MUST begin with an executable token
  mutation: Option<{ patch: String, suite: String }>,   # REQUIRED when kind = mutate; the unified diff and the suite id
  expected: { exit: i32 } | { assertion: Locator },     # machine-readable; beside the prose
  observed: String,
  expected_when_met: String,
}
```

`kind = run` is a reproduction: the command runs against candidate and reference and the class is `reproduced` iff
`observed ≠ expected_when_met` (clause 2, unchanged). `kind = mutate` is a mutation: the gate applies `mutation.patch`
at the reviewed SHA, runs `mutation.suite`, and reverts. A finding whose `command` does not begin with an executable
token, or whose `kind = mutate` lacks `mutation`, is **rejected at ingest** as malformed — it is not `Unverifiable`, it
is not a finding (AC-281 gains the negative fixture). The classifier (the pure function over the repro record) and
`policy::classify` (the policy-table lookup) are **two functions**; AC-283 names `claim` and `rationale` as the only
free-text fields and neither is an input to either.

**2. `mutation-survived` requires a discriminating mutant.** The class is computed iff `kind = mutate`, the suite stayed
green after the patch, **and** one of: (a) the patch is a **reverted fix** — it removes the change under review and the
named test stays green, which is the E3 measurement of a vacuous test (F25); or (b) the finding carries, in
`expected.assertion`, an assertion that **distinguishes mutant from original** and the gate observes it flip. A
surviving mutant with neither collapses to `narrowed-pin` (the assertion exists but did not flip) or `advisory` (no
assertion). An equivalent mutant therefore cannot block.

**3. F31's block counts are re-tagged.** Design §0.1 rows 42/44, §6.4.1 clause 2 and §6.4.2 say "measured under the
model-label policy; the Rust-computed class is unmeasured". **S-CLASS-REPLAY** is registered ($0: run the clause-2
decision table over the 253 recorded S-TYPED findings — 118/29/29/77 by label — and report per-class agreement with the
model label; consumers AC-284, SM-091); AC-284 is PROVISIONAL until it runs. The 52/52 conformance figure is cited as
conformance to the **pre-0096** shape.

## Considered options

### Option — keep the shape; execute `command` verbatim
Rejected: the measurement says most recorded commands are prose; the gate would fail open on the class it exists to
block, and AC-289/AC-305 would be narrowed to "whatever string was recorded".

### Option — typed shape; measure the equivalent-mutant false-block rate instead of designing it out
Register a spike with the policy table as consumer. Rejected as the primary route: the equivalence rule is
well-established (PRIOR-ART §5) and the reverted-fix mutant is the E3-measured discriminator; a measurement would
price a hole the record can close. Retained as a note: if a later corpus shows discriminating assertions are rare, that
is a finding for a successor.

### Option — a free-form `mutation` string
Rejected: a string the gate cannot apply is the current defect with a new field name.

## Consequences

**Positive**
- AC-289 and AC-305 have the fields they name; the gate can re-execute what it blocks on.
- A malformed finding is rejected where it enters, not routed to advisory.
- `mutation-survived` cannot fire on a no-op.

**Negative**
- The 52/52 S-TYPED conformance figure describes the old shape; the new shape has no conformance measurement until a
  card emits it (S-CLASS-REPLAY does not measure emission).
- Reviewers must supply a patch and a suite id; the card's schema grows and the Step 5.3 snapshot changes on first
  export — a reviewed schema change, as the runbook already treats it.

**What this forbids**
- Forbids a `repro.command` that is not a shell command beginning with an executable token, and forbids a
  `kind = mutate` record without `mutation`.
- Forbids computing `mutation-survived` without a reverted-fix patch or an observed assertion flip.
- Forbids feeding `claim` or `rationale` to the classifier or the policy table.
- Forbids citing F31's 12/23 or 9/23 block counts, or its 0/23 false positives, as evidence for the Rust classifier
  until S-CLASS-REPLAY reports.

## Evidence and confidence

- [measured, S-TYPED payloads, refuter recount] 24/29 `mutation-survived` commands are prose; 111/176 blocking-grade
  commands do not begin with an executable.
- [measured, F25] the reverted-fix discriminator: `issue-146-147` AC-015's test stays 36/36 green with the ordering
  inverted — the vacuous-pin case this record turns into a mutant definition.
- [measured, F31] 52/52 schema conformance, 147/147 `repro.command` filled — against the pre-0096 shape.
- [measured, e3.rs:254-256] `policy_verdict` classifies on the model's label.
- [documented, PRIOR-ART §5] the mutation-equivalence rule.
- **What this evidence does not license.** It does not license claiming the typed shape is emitted correctly by any
  card — unmeasured. It does not license a false-block rate for `mutation-survived` under the new definition.
