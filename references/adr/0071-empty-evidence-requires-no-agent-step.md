---
id: "0071"
title: Empty evidence is legal only for activities with no agent step
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-41
source: references/2026-09-07-activity-graph-schema.md §9c amendment 41
supersedes: []
superseded_by: []
related: ["0036"]
evidence: []
consumers: []
---

## Context and problem

§9a amendment 6 normatively fixed which step kinds mint which evidence variants: `tool` mints `ToolReport` (plus
`GreenAt`/`Mutation` under conditions), `agent` mints `Findings` or `SelfCheck`, `gate` mints `HumanAnswer`, and `check`
and `emit` mint nothing. Because `check` and `emit` mint no evidence, an activity built only from `check`/`emit` steps
(no `agent`, `tool`, or `gate` step) can legitimately have an empty `evidence` promise. The P6 authoring batch produced
an amendment stating the legality condition as a rule. The source row states the rule only; the reading below of why
an unconditional allowance would be wrong is drawn from §9a.6 and §2, not from the row.

## Decision drivers

- §9a.6's minting table already implies that an activity with only `check`/`emit` steps produces no evidence; the P6
  batch needed a lint-checkable rule to distinguish that legitimate case from an activity that promised no evidence but
  also ran an `agent` step (whose `Findings`/`SelfCheck` output would otherwise go unrecorded as a promise).
- Evidence promises are load-bearing elsewhere: stage-exit `checks` (§2) are predicates over the evidence list, and an
  activity that runs an agent but declares no evidence would make its own findings unreachable to those checks.

## Considered options

The source row records no alternatives column and no rejected option; the option below is reconstructed as the status
quo the amendment displaced, and its stated reason is drawn from §9a.6 and §2, not from amendment 41 itself.

### Option — do nothing (allow empty evidence unconditionally)
Lost: allowing any activity, including one with an `agent` step, to declare empty `evidence` would let
an activity silently discard the findings/self-check output its own agent step mints, defeating the "reviews return
findings" model (proposal §4.6) and the evidence-based stage-exit checks in §2.

### Option — empty evidence legal only with no `agent` step (chosen)
Adopted verbatim from the amendment. The source states the rule for the `agent` kind alone; it says nothing about
`tool` or `gate` steps, whose minted evidence (`ToolReport`, `HumanAnswer`) is not brought under the same condition
here.

## Decision

Empty `evidence` is legal only for activities with no `agent` step. An activity containing at least one `agent` step
must declare a non-empty `evidence` promise (consistent with §9a.6's minting rule that `agent` steps mint `Findings` or
`SelfCheck`).

## Consequences

**Positive**
- Closes a gap where an activity could run an `agent` step and declare no evidence promise, making its findings
  unreachable to stage-exit checks.
- Gives lint a simple, checkable rule tied directly to the step-kind evidence-minting table already normative in §9a.6.

**Negative**
- Activities built purely from `check`/`emit` steps must remember to leave `evidence` empty deliberately rather than
  copying a non-empty template from an agent-bearing activity, or lint will presumably flag the mismatch (the source
  does not specify lint behaviour for the reverse case — non-empty evidence on a no-agent activity).

**What this forbids**
- Forbids any activity with at least one `agent` step from declaring an empty `evidence` list.

## Evidence and confidence

- [documented] The rule is stated directly in §9c amendment 41: "Empty `evidence` is legal only for activities with no
  `agent` step."
- [inferred] The rationale connecting this to §9a.6's minting table and to stage-exit checks over evidence (§2) is
  drawn from those sections' own normative text, not stated explicitly alongside amendment 41 itself.
- No finding id or measurement is cited for this amendment; the `evidence` field is left empty accordingly.
- This licenses only the stated legality condition on `agent` steps. It does not license any claim about how many
  catalog activities carry empty `evidence`, about the reverse case (non-empty `evidence` on an activity with no
  `agent` step), or about `tool`/`gate` steps — the source addresses none of these.
