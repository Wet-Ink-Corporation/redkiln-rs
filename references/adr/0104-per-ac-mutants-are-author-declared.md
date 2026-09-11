---
id: "0104"
title: Per-AC mutants are declared by the AC's author, the flip predicate is AC-334's, the whole-suite comparator bootstraps from the first affected run, and a quarantined survivor still blocks exit
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-049, PIA-051 (§4.5/§4.6); AC-317, AC-326, AC-329, AC-334, AC-337, AC-345, AC-346, AC-357, SM-094, SM-095; spikes/FINDINGS.md F25; spikes/remediation/README.md
supersedes: []
superseded_by: []
related: ["0019", "0029", "0074", "0096"]
evidence: ["PIA-049", "PIA-051", "F25", "F24"]
consumers: []
---

## Context and problem

The verify gate has two predicates for one mutation harness and no derivation rule for either. AC-334 says "the
mutation derived from that AC's THEN clause"; AC-317 says "the AC's implementation step is mutated (removed, and
inverted)" and demands the named test fail under *both*; nothing says how a THEN clause becomes a patch, and F25's
measured case — remove the step → RED, invert the ordering → 36/36 GREEN — is a case where "removed" and "inverted"
disagree, which is the whole finding [measured, PIA-051, PIA-049]. AC-326 reclassifies an `affected` run whose count
reaches "the most recent recorded whole-suite count", and AC-329 compares against `total_test_count` — but on the first
run at a new toolchain version pin no whole-suite count has been recorded, so the comparator is undefined [measured].
And a mutation survivor that a human quarantines (ADR-0074 clause 6) has no stated exit route: AC-346 refuses exit on
an unpinned AC "neither pinned by a flip-verified assertion nor answered at a human gate", and nothing says whether a
quarantine is that answer [measured].

The audit asked for a new record, not an amendment to ADR-0019 (which fixes *that* a per-AC discriminating test is the
norm, not *how* the mutant is obtained). E3's 5-of-23 shipped-with-a-criterion-nothing-pins result and F25's remediation
package (a TDD-evidence table with an explicit **Mutation (post-fix)** column the reviewer re-runs) are the measured
basis.

## Decision drivers

- A derivation the machine performs from prose is a classifier over free text — the thing ADR-0074 removed from the
  review gate; a mutant the author declares is a patch the machine applies.
- One predicate per harness, or two implementers ship two harnesses.
- A comparator with no first value must say where the first value comes from.
- A waiver path at exit is how E3's five unpinned criteria shipped.

## Decision

**1. Mutants are author-declared, per AC.** Each acceptance criterion row in a spec may carry a `mutation:` field —
`{ patch: <unified diff>, suite: <suite id> }` — authored with the AC (the fix and story lanes' spec stage). Where the
toolchain pack declares mutation infeasible, the AC records `mutation: not-feasible(reason)` (AC-337, unchanged). The
harness applies the patch at the candidate SHA, runs the suite, reverts, and records the outcome. It is deterministic
and dispatches nothing. A declared mutation must be a **reverted-fix** patch or carry a discriminating assertion
locator, per ADR-0096's rule; the F25 ordering inversion is the canonical fixture and must yield `mutation-survived`.

**2. AC-334 is the authoritative predicate**: an AC is `pinned` iff its named assertion flips from pass to fail under
its declared mutation; if the suite stays green it is `mutation-survived`. **AC-317 becomes the harness-invocation
criterion** — the harness runs in the fix and story lanes over every AC carrying a `mutation:` field, and an AC whose
mutation survives is refused at report time — and drops "removed, and inverted". SM-094's instrument is AC-334's flip
predicate.

**3. The whole-suite comparator bootstraps.** When no whole-suite count is recorded for the toolchain at the current
version pin, a run claiming `basis: affected` is reclassified `basis: whole-suite` with reason `probe-unverified`, and
**its executed count seeds the comparator**; AC-329's `total_test_count` on an `affected` run is the most recent
recorded whole-suite count at that pin, and the report names the run it came from. A pin bump resets the comparator.

**4. A quarantined survivor still blocks exit.** Quarantine (ADR-0074 clause 6) is a **review-gate** state: it stops a
finding blocking *review* and stops it being re-raised until `until`. It is not an answer to AC-346; an AC whose mutant
survived stays unpinned until a discriminating mutant flips or a human gate (AC-357) answers *that AC* explicitly.
AC-345 and SM-095 say "review-gate only"; AC-346 says quarantine does not satisfy it.

**5. This is decision 6 in the runbook's header table** (after ADR-0090, ADR-0091, the git version, `doctor`'s
window, and ADR-0093/`stale_policy`), because the spec stage's template gains a field on its strength.

## Considered options

### Option — machine-derived mutants from the THEN clause
Keep AC-334's wording and specify an algorithm (invert the comparator, remove the step). Rejected: no measurement
shows such a derivation is deterministic over real ACs, and F25's own case is one where two derivations disagree.

### Option — author-declared mutants; a quarantined survivor may be human-gated at exit
Extend AC-357's GIVEN so a quarantine's `by`/`until` doubles as the AC-346 answer. Rejected by the operator: E3's
evidence (5 of 23 items shipped with a criterion nothing pins) is the case a waiver path produces.

### Option — leave AC-317's two mutations as a stricter superset
Rejected: "removed and inverted" is two mutants for one AC and F25 shows they can disagree; the author declares the one
that discriminates.

## Consequences

**Positive**
- One predicate, one field, zero dispatches; the harness is a patch-apply-and-run loop.
- The comparator has a first value and a stated reset.
- Exit cannot be waived past an unpinned AC by a review-time quarantine.

**Negative**
- Authoring cost moves to the spec stage: every AC wants a patch, or a `not-feasible` reason.
- The bootstrap run is a whole-suite run, once per pin.

**What this forbids**
- Forbids deriving a mutant from prose at run time.
- Forbids a second flip predicate beside AC-334's.
- Forbids reporting `basis: affected` with no recorded whole-suite count at the current pin.
- Forbids treating a quarantine as an answer to AC-346.

## Evidence and confidence

- [measured, F25] the AC-015 case: step removal → RED; ordering inversion → 36/36 GREEN; 5 of 23 fix-lane items
  shipped with a criterion nothing pins.
- [measured, PIA-049, PIA-051] the two predicates, the undefined comparator, the missing exit route.
- [documented, spikes/remediation/README.md] the proposed (not adopted) TDD-evidence table with a re-run mutation
  column — the shape this record adopts at the AC row.
- **What this evidence does not license.** It does not license any rate of `not-feasible` or of author-declared
  mutants that fail to discriminate; the first lane run is the first measurement.
