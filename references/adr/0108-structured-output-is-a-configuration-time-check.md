---
id: "0108"
title: StructuredOutput is a configuration-time check — a schema-carrying dispatch whose tool set lacks it is refused before spawn, and nothing parses a schema out of prose
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-082 (§4.3); references/2026-09-08-architecture-design.md §0.1 :46, §1.2 :82, §3.6 :364; AC-095, AC-205, AC-417, AC-504; spikes/FINDINGS.md F20
supersedes: []
superseded_by: []
related: ["0014", "0016", "0095", "0099"]
evidence: ["PIA-082", "F20"]
consumers: []
---

## Context and problem

Design §0.1 row 46 marks "structured output is a configuration-time check" DECIDED and cites ADR-0016; AC-095, AC-205,
AC-417 and AC-504 carry `decisions: ADR-0016`. ADR-0016 decides that `--system-prompt` carries the card body and how
the cache TTL is pinned; it does not contain the configuration-time rule [measured, PIA-082]. The rule's evidence is
F20, the programme's most consequential harness measurement: an agent card's `tools` allowlist **silently disables
`--json-schema`** — `structured_output` absent from the payload, **exit 0**, `subtype: success` — and adding
`StructuredOutput` to the allowlist fixes it [measured, F20]. Halt 3 and golden constraint 4 exist because of it.
This record gives the rule its home. Nothing is superseded.

## Decision drivers

- The failure is silent and successful-looking; the only place to catch it is before the process exists.
- A rule cited from a record that does not contain it is unprotected by the forbids machinery.

## Decision

1. **A dispatch that carries a JSON schema and whose effective tool set does not include `StructuredOutput` is
   refused as a configuration error before any process is spawned** (AC-205), and `doctor` reports the same card at
   configuration time at zero model cost (AC-095, AC-417).
2. **The effective tool set is the card's declared list and nothing else** (ADR-0099 clause 5's §3.6.1 sentence); a
   card with no `tools:` key is a configuration error, not an "inherit everything" default.
3. **Nothing parses a schema, a verdict or a finding out of prose.** A dispatch that was expected to return a
   structured payload and returned none is `StepError::Schema`, never a pass (design §1 principle 6; Halt 3).
4. **The check is on the argv the adapter assembles**, which is also where ADR-0095's per-request budget precondition
   is linted; the two lints share the command builder.

## Considered options

### Option — retry without the allowlist when the payload is absent
Rejected: it turns a configuration error into a behaviour change at run time, and F20's mechanism is that the
absence looks like success — a retry would have nothing to key on except the absence.

### Option — keep citing ADR-0016
Rejected: the rule is not there.

## Consequences

**Positive**
- The single most expensive silent failure the programme measured is caught at zero model cost.

**Negative**
- A card author must list `StructuredOutput` explicitly; the doctor row is the affordance.

**What this forbids**
- Forbids spawning a schema-carrying dispatch whose tool set lacks `StructuredOutput`.
- Forbids reading exit 0 or `subtype: success` as evidence that a structured payload was produced.
- Forbids a default tool set for a card that declares none.

## Evidence and confidence

- [measured, F20] the mechanism, on the pinned CLI; the three E3 defects that survived "built and validated" include
  this one.
- [measured, PIA-082] four criteria and three design rows citing a record without the rule.
- **What this evidence does not license.** It does not license any claim that `StructuredOutput` guarantees a
  schema-conformant payload — S-TYPED's 52/52 (F31) measured conformance under the pre-0096 shape, and ADR-0095's
  S-SCHEMA-CAP measures the vendor's rejection shape.
