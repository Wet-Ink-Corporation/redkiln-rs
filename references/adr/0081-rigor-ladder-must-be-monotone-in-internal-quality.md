---
id: "0081"
title: The rigor ladder must be monotone in internal quality, and the lattice lint must enforce it
status: accepted
date: 2026-09-08
legacy_id: none
source: spikes/F37-DRAFT.md §6 (incidental finding); spikes/F35-DRAFT.md (P6 lattice); references/2026-09-07-activity-graph-schema.md §9c amendment 26
supersedes: []
superseded_by: []
related: ["0056", "0058", "0028", "0036", "0020", "0080"]
evidence: ["F37", "F35", "F34"]
consumers: []
---

## Context and problem

Amendment 26 (ADR 0056) settled how a family of producers escalates with rigor: the rank rule is unchanged — among
admissible producers of a kind, lowest `method_rank` wins — a family may put a `max_rigor` ceiling on its cheaper
members, and a lint rule requires the family to stay **total** over rigor, so that some member remains admissible at
every level. Totality was not a cosmetic addition. F35 measured that ceilings *without* it took unsatisfiability from
11.4% to **30.1%** of enumerated contexts. Ceiling rules in this area have large measured effects.

Totality guarantees that *a* producer exists. It says nothing about *which* one, or about what that producer promises.
P7 found the gap by accident. [F37 §6] The `mandatory` arm was dropped at n = 1 because `root-cause.toml` carries
`max_rigor = "recommended"`, so the activity is inadmissible at `rigor = mandatory` and the engine escalates within the
`localisation` family instead. Running the family through the engine gives:

| `has_repro` | rigor | selected | `adversary` + `policy` present? |
| --- | --- | --- | --- |
| false | `recommended` | `root-cause` (rank 10) | yes |
| false | `mandatory` | `instrumented-trace-analysis` (rank 40, cost L) | yes |
| true | `recommended` | `root-cause` (rank 10) | yes |
| true | **`mandatory`** | **`bisect`** (rank 30) | **NO** |

`bisect`'s steps are `bisect → widen → explain → confirm → signoff → emit` — a gate, but **no adversary step and no
policy step**. Under "lowest admissible wins" it beats `instrumented-trace-analysis` at rank 40. So for an item that has
a reproduction, **escalating from `recommended` to `mandatory` drops the quality block entirely**: the highest rigor
level buys *less* internal quality than the level below it, on exactly the items most likely to be reproducible.

Two things about how this was found matter as much as the finding.

First, it is a defect in the **ceiling design**, not in `root-cause` and not in `bisect`. Each activity is individually
well-formed. The family's rigor ladder is not monotone in internal quality, and neither activity can see that from where
it sits.

Second, **no lint or lattice check found it.** `lattice.py --validate` already checks family totality over rigor, and
F35 reports the validator running the full v0 catalog in 27 s with 0.3% unsatisfiability, 6/102 dead activities, 0 rank
ties and 0 hard cycles. It reported nothing here, because on every one of those axes the family is healthy: a producer
exists at every rigor, ranks are distinct, nothing is dead, nothing cycles. The defect surfaced because someone asked why
the `mandatory` arm was pricing a block on an activity the engine would not select. A property that is only discoverable
by asking the right question is a property the lint should hold instead.

## Decision drivers

- PC6 (ADR 0028) makes an activity's promise about its own output quality the contract the stage consumes. If escalating
  rigor can silently weaken that promise, the contract is not ordered and the rigor enum does not mean what it says.
- The rigor enum (ADR 0020) is ordinal — `forbidden < discouraged < optional < recommended < mandatory`. An ordinal
  scale whose effect on internal quality is non-monotone is a scale that misleads its reader by construction.
- Amendment 26's own measured lesson is that a ceiling rule's failures are **silent**: the 11.4% → 30.1% result was
  unsatisfiability nobody was told about until the lattice enumerated it. The same argument applies to a lost quality
  block, which is quieter still — the graph runs, produces an artifact, and reports success.
- The property is cheap to check. The lattice enumeration this needs is the one `--validate` already performs; the
  evidence a producer promises is already a fixed, closed function of its step kinds (ADR 0036), so the comparison is
  mechanical.
- The failure mode has a live measured instance, at the top of the rigor ladder, on the item class most likely to reach
  it.

## Decision

**1. The rigor ladder must be monotone in internal quality.** For every family, and for every context in the enumerated
lattice, the internal-quality evidence promised by the producer the engine selects must be **non-decreasing in rigor**.
Formally, for rigor levels `r1 < r2` in the same context `ctx`:

```
evidence(select(family, ctx, r2))  ⊇  evidence(select(family, ctx, r1))
```

where `evidence(a)` is the set of evidence variants activity `a`'s steps mint, which is fixed and closed per step kind by
ADR 0036 and is therefore computable from the catalog without running anything. Losing an `agent` step's `Findings` — the
adversary — or the `check` step that consumes it is a monotonicity violation whatever else the higher-rigor producer adds.

**2. `lattice --validate` must check block monotonicity, alongside the family-totality check it already performs.** The
two are separate obligations on the same enumeration and both are errors, not warnings:

- *totality* (ADR 0056): some member of the family is admissible at every rigor ≥ the lowest member's `min_rigor`, over
  every context the lowest member's tags cover;
- *monotonicity* (this record): the member the engine actually selects promises evidence that is a superset of what the
  next lower rigor's selection promises, in that same context.

A family that satisfies totality and violates monotonicity is a lint error. `bisect` at
`(has_repro = true, rigor = mandatory)` is the first recorded instance.

**3. This adds to ADR 0056; it withdraws nothing from it.** The rank rule stands unchanged — lowest `method_rank` wins,
and a tie is a pack validation error, never a runtime choice. `max_rigor` ceilings remain legal. Totality remains
required. What changes is that a ceiling now carries a second obligation: the member that covers the space above the
ceiling must not promise less than the member below it. ADR 0056 stays `accepted` and is `related`, not superseded.

**4. Nothing in the catalog is edited by this record.** F37 §6 records that the catalog was not touched on an unattended
run, and that discipline holds. The `localisation` family's violation is a lint finding to be repaired deliberately — by
adding the block to `bisect`, by re-ranking, or by ceiling `bisect` — and the repair is a shipped-activity edit that
bumps the version and enters the changelog per ADR 0072. The lint reports it; a human fixes it.

## Considered options

### Option — leave it to authoring discipline
Document the property in the authoring guide and expect authors to keep their families ordered. Rejected on the strongest
evidence available: this defect got past three authoring passes, a schema amendment specifically about ceilings, and a
validator run over 102 activities. It was found by answering an unrelated question about why an arm was priced against an
unreachable configuration. Discipline did not catch it and there is no reason to think it would catch the next one.

### Option — forbid `max_rigor` ceilings altogether
Remove the mechanism that created the discontinuity. Rejected: ceilings are a real authoring need — cheap methods should
not be asked to run at rigor levels they cannot support — and ADR 0056 already weighed and kept them. Removing them
trades a checkable defect for an authoring problem the corpus has not measured.

### Option — require every family member to carry the quality block
Mandate `adversary → policy` on every producer in a rigor-driven family, making monotonicity true by construction.
Rejected on two grounds. It is the wrong shape: the property that matters is ordering, not universal presence, and a
family whose members all lack the block would be monotone and legal under this record but illegal under this option. And
it collides with ADR 0080, which decided that the `quality` shorthand carries **no default** — a rule that inserts the
block into activities that did not write it is precisely what 0080 forbids.

### Option — check monotonicity at runtime instead of at lint time
Compare the selected producer's promised evidence against the previous rigor's at selection and gate on a decrease.
Rejected: it converts a catalog defect into a per-item runtime gate, which is the exact failure mode ADR 0056 was written
to avoid — a lint-time property discovered late as `Unsatisfiable(kind, failed_predicates)`. The lattice enumeration
already exists and costs 27 s over the whole catalog.

## Consequences

**Positive**
- The rigor enum regains its plain meaning: escalating rigor cannot buy less internal quality than the level below it.
- A silent failure mode — a graph that runs, emits, and reports success while having quietly dropped its adversary —
  becomes a lint error found at pack-load time.
- The check reuses machinery that exists: the same lattice enumeration as the totality check, over an evidence mapping
  that is already fixed and closed by ADR 0036. No new primitive, no new runtime cost.
- The `localisation` family's defect is now a named, reproducible finding with a specified repair path rather than an
  incidental observation buried in a dropped arm.

**Negative**
- Authoring a rigor-driven family gains a third obligation on top of shared `provides`, distinct `method_rank` and
  totality. F34 already recorded the totality rule as one of the harder new lint rules to satisfy, and this compounds it.
- The check is defined over the lattice enumeration, so it inherits that enumeration's cost profile and its blind spots:
  a context the lattice does not enumerate is a context the check does not cover.
- The `localisation` family is in violation as of this record, so the v0 catalog does not currently pass its own new
  lint. The repair is deliberate work that has not been done.
- Defining internal quality as the promised evidence set is a deliberately mechanical proxy. A producer could satisfy
  monotonicity by minting the right evidence variants while doing shallower work; the check is structural, not semantic,
  and no measurement here says how often that gap would matter.

**What this forbids**
- Forbids a family in which escalating rigor selects a producer promising a strict subset of the evidence promised at a
  lower rigor, in the same context.
- Forbids `lattice --validate` passing a catalog on family totality alone; totality and block monotonicity are separate
  obligations and both must be checked.
- Forbids treating a non-monotone family as a warning, a note, or an authoring preference. It is an error.
- Forbids repairing this class of defect by inserting a quality block into an activity that did not declare one — ADR
  0080 forbids implied defaults, so the repair is an explicit authored edit under ADR 0072's version-bump rule.
- Forbids citing amendment 26's totality rule as sufficient evidence that a family's rigor ladder is well-formed.

## Evidence and confidence

- [measured] F37 §6: the `localisation` family run through the engine yields `root-cause` (rank 10) at `recommended` for
  both values of `has_repro`, `instrumented-trace-analysis` (rank 40, cost L) at `(false, mandatory)`, and `bisect`
  (rank 30) at `(true, mandatory)`. `root-cause.toml` carries `max_rigor = "recommended"` and `lattice.py:137` makes it
  inadmissible above that. `bisect`'s step list is `bisect → widen → explain → confirm → signoff → emit` — no adversary,
  no policy. This is an engine run over the committed catalog, not an inference about it.
- [measured] F35: family totality is worth a large amount. Unsatisfiability was 11.4% with the totality rule in force and
  **30.1%** with ceilings present but ungoverned by it. The v0 lattice validator reports 0.3% unsatisfiability post-
  decision, 6/102 dead activities, 0 rank ties, 0 hard cycles, 27 s runtime. ADR 0056 records that the source does not
  name the condition under which the 11.4% figure was taken; that caveat is carried here unchanged.
- [documented] The evidence-minting mapping this record's monotonicity test is defined over is normative and closed per
  step kind (ADR 0036): `tool` mints `ToolReport` (plus `GreenAt`, plus `Mutation`), `agent` mints `Findings` or
  `SelfCheck`, `gate` mints `HumanAnswer`, and `check` and `emit` mint nothing. The comparison is therefore mechanical
  and needs no new metadata.
- [inferred] "This is a candidate defect in amendment 26's ceiling design, not in `root-cause`" is F37 §6's own reading
  and is adopted here. It is a judgement about where the fault lies, not a measurement.
- **How this was found, and why that is the argument.** The finding is incidental to P7's dropped `mandatory` arm. No
  lint rule, no lattice check and no authoring pass surfaced it; a question did. That is recorded as evidence about the
  *check's* necessity, not as evidence about the frequency of the defect.
- **What this evidence does NOT license.** It does not license any claim about how many families are non-monotone: **one
  instance, in one family, on one catalog.** No sweep has been run, and the `localisation` family is one of several in
  the v0 catalog. It does not license a claim that the new check would have caught anything else, or that it is
  sufficient — monotonicity over promised evidence is a structural proxy for internal quality, and nothing here measures
  the gap between promising evidence and producing good work. It does not license quoting the `mandatory` arm's cost
  (F37 reports 3.66× against `recommended`; the decisions log reports 4.4×, unreconciled) as anything but n = 1 on a
  configuration the engine does not select. And it does not license any inference about the quality block's *value* —
  that question is ADR 0080's, and 0080 records that P7 could not resolve it.
- **Corpus versus external evidence.** The 2026-09-08 research pass has nothing on-axis: evidence-augmentation §5 records
  that no external source frames activity selection as a research question at all, and names that as a positive gap — no
  comparable published system exists to check this framing against. The nearest external material is the Dagster RFC
  cited at §4 rec 22, which documents a closed policy enum that "could not compose" being replaced by composable
  primitives; it is a warning about the rigor enum's long-term shape, not a check on this rule. This decision therefore
  rests on the corpus's own measurement, and the absence of external contradiction is not treated as support.
