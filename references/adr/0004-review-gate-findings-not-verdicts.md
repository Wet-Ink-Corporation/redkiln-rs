---
id: "0004"
title: Review gate returns findings, not a verdict
status: superseded
date: 2026-09-05
legacy_id: D4
source: references/2026-09-05-architecture-proposal.md §9 (choice grounded in §4.6)
supersedes: []
superseded_by: ["0074"]
related: ["0001", "0019"]
evidence: ["S-VAR"]
consumers: []
---

## Context and problem

The D1 architectural bet already commits to moving control-flow decisions out of the model and into Rust (§0, D1). Applied to the review gate specifically, the row records the decision as "Findings, not verdicts; Rust policy computes the block" and marks it "confirmed pending S-VAR" — i.e. taken ahead of the spike that would confirm it, on the general principle rather than on review-specific measurement at the time of the row. §4.6 as it stands in the source document describes the fuller, rebuilt mechanism (reviewer proposes, machine confirms) that followed once S-VAR and S-TYPED actually ran; this ADR records only the original D4 choice, not that rebuild.

## Decision drivers

- The inversion architecture's general principle that the model may classify or propose but must never itself hold a control-flow decision (D1; §3 principle 1, "Rust holds the loop; the model holds no control flow … the agent *classifies* and may *request* a typed deviation").
- A general expectation — not quantified in this row — that a reviewer's verdict is a less stable signal than the underlying findings it is based on, motivating computing the block in Rust policy rather than accepting the model's own pass/fail word.
- The row explicitly flags the decision as taken ahead of confirmation ("confirmed pending S-VAR"), i.e. a deliberate bet made before the supporting spike ran.

## Considered options

### Option — model returns a verdict

The reviewer would emit a pass/fail (or approve/changes-requested) judgement directly, and the gate would act on that word. Rejected because it re-introduces a model-held control-flow decision, the shape D1 already rules against.

### Option — more repeats before redesign

Keep the existing verdict-returning review mechanism and drive down its instability by sampling more repeats (e.g. majority vote across draws) rather than changing what the reviewer emits. Rejected in favour of redesigning the contract itself (findings, not a verdict) rather than trying to average out an unstable signal.

## Decision

The review gate returns **findings, not a verdict**: the reviewer's output is treated as data (claims plus evidence), and a Rust policy component — not the model — computes whether the change is blocked. This was decided and recorded as D4 ahead of the S-VAR spike that would confirm it ("confirmed pending S-VAR").

## Consequences

**Positive**
- The blocking decision becomes an auditable, testable piece of Rust code rather than an opaque model judgement, consistent with D1's "Rust holds the loop" principle.
- Findings survive as structured, inspectable data regardless of whether any single verdict would have been trusted.

**Negative**
- At the time of this row the decision was not yet confirmed by measurement; taking it "pending S-VAR" means the corpus commits to the shape of the mechanism (findings-based) before knowing how unstable the verdict signal actually was, or what a findings-based policy would need to look like in detail.

**What this forbids**
- The reviewer's own verdict, if one were still emitted, may not be the thing the gate acts on; only a Rust-side policy computation may decide the block.
- The design may not fall back to "more repeats of the same verdict-returning mechanism" as a fix for instability — the row records that alternative as considered and rejected in favour of the findings-based redesign.

## Evidence and confidence

- [documented] The row itself is the only evidence for this ADR's scope: it states the decision and names S-VAR as the spike that would confirm it, but does not report S-VAR's results here — this ADR is provisional on S-VAR as of the date recorded (2026-09-05).
- [inferred] The general instability-of-verdicts rationale is inferred from the surrounding architecture principle (D1, §3 principle 1) and from the fact the row itself frames the decision as pending confirmation; no verdict-flip rate is cited in this row.
- This ADR does not license claiming the findings-based mechanism was validated as of this decision — validation (S-VAR, S-TYPED) came later and produced the rev. 4 rebuild described in §4.6, which is out of scope here (see Consequences).
- [documented] Superseding note: the source carries a separate **D4 (rev. 4)** row rebuilding the review gate as "reviewer proposes, machine confirms" (class computed from the repro record, flip re-executed before a block, `Unverifiable`/`Quarantined`, policy per stage weight, block/override telemetry, negative-control fixtures; §4.6). That row is now recorded as [0074](0074-review-gate-reviewer-proposes-machine-confirms.md), which supersedes this record **in full**; this record's status is `superseded` and it covers the original D4 choice only. Read 0074 for what is now in force.
