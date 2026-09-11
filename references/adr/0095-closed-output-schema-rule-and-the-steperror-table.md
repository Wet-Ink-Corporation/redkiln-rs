---
id: "0095"
title: One closed-output schema rule — no oneOf, anyOf only by transform, union-typed parameters counted per exported schema — and StepError enumerated
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-005 (§3.2), PIA-014 (§3.5), PIA-016; references/adr/0084 :64, :69, :117; platform.claude.com/docs/en/build-with-claude/structured-outputs (fetched 2026-09-10)
supersedes: ["0084"]
superseded_by: []
related: ["0075", "0076", "0083", "0099"]
evidence: ["PIA-005", "PIA-014", "PIA-016", "F20", "F31", "F1"]
consumers: []
---

## Context and problem

Step 5.3 writes `rk-types` and its golden schema snapshot under rules that cannot all hold. ADR-0084's snapshot test
asserts the exported schema "contains **no `oneOf`** anywhere" (:64) and prescribes that "data-carrying enums that must
cross the boundary are written `#[serde(tag = "kind")]` so they render as a single discriminated object" (:69). But in
schemars 1.2.2 `expr_for_internal_tagged_enum` builds one object per variant and joins them with `oneOf` when the
variants are unique (`anyOf` otherwise), so the prescribed shape fails the ADR's own assertion and Checkpoint 5's grep
[measured, schemas generated with schemars =1.2.2; docs.rs fetched 2026-09-10]. AC-131 and design §5.3 forbid only a
union *nested* inside another; AC-157 caps "union branches at any node" while the runbook and ADR-0084 keep "≤ 16
union fields per type"; and Checkpoint 5 greps `oneOf` only, which `#[serde(untagged)]`'s `anyOf` passes while
violating the intent [measured, PIA-005].

The 24/16 caps the corpus calls "this project's own authoring budget, not a vendor constant" are the vendor's
**Schema complexity limits** table — on the page on 2026-09-10 and in a Wayback capture of 2026-09-07, so ADR-0084's
"no limits table was found" (:117) was a truncation artefact — and they are **per request** across all strict tool
schemas plus the JSON output schema: 24 optional parameters, 16 parameters using `anyOf` or type arrays, 20 strict
tools per request, a 180 s compile timeout; `anyOf`/`allOf` are supported and `oneOf` is absent; `additionalProperties`
must be `false` [documented, platform.claude.com fetched 2026-09-10]. A schemars `Option<T>` renders as a type array,
so a type passing AC-132 with 24 optional fields could carry 24 union-typed parameters and breach the vendor's 16
with every corpus lint green — and F20 records that closed-output violations fail open at exit 0, so the breach would
be silent [inferred].

`StepError`, the flagship closed type (AC-229 asserts it closed; AC-360 one event arm per variant), is never enumerated
in a governing document: design :1649 says its shape is "carried from step.rs" (six variants) while §3.8/§7 name the
eleven-variant set of the superseded proposal [measured]. Two adjacent gaps land in the same table: a child that
exhausts the subscription mid-dispatch has no error class and backs off instead of resolving (PIA-014), and tool-set
inequality against the card's allowlist is `NotAdopted` in AC-204 and `NotIsolated` in AC-202/AC-386 (PIA-016).

This record supersedes ADR-0084 **in part**: the golden-snapshot clause's rule set and the :117 evidence line. The
`gix` pin, the `cargo audit` job, the rusqlite/WAL/one-writer clause and the `jsonschema` clause stand; ADR-0084 stays
`accepted`. ADR-0100 supersedes its range clause separately.

## Decision drivers

- One rule, stated identically in AC-131, AC-157, design §5.3 and the runbook, or two implementers ship two lints.
- The vendor does not list `oneOf`; a schema that carries one is outside the documented contract whatever the corpus
  greps for.
- The vendor counts union-typed *parameters per request*; a per-type branch count measures a different quantity, and
  the failure mode on breach is silent (F20).
- An unenumerated `StepError` is matched exhaustively by the kernel (7b) and the adapter (7c) against whichever set
  its author picked.

## Decision

**1. The schema rule.** Every schema `rk-types` exports across the closed-output boundary:

- contains **no `oneOf` at any node**. Data-carrying enums are still written `#[serde(tag = "kind")]`; their derived
  schema is passed through a **post-derive transform** (a `schemars::transform::Transform`) that rewrites a `oneOf`
  whose branches are objects discriminated by `kind` into an equivalent `anyOf`. The transform is part of `rk-types`
  and runs inside the schema export, so the snapshot sees the exported shape.
- contains **no union nested inside a union**: no `anyOf` (or type array) has a descendant `anyOf` or type array
  (AC-131's rule, unchanged).
- sets `additionalProperties: false` at every object level and orders `required` first (unchanged).
- carries **≤ 16 union-typed parameters** — any property whose schema uses `anyOf` or a type array, nullable
  `Option<T>` included — and **≤ 24 optional parameters**, counted **per exported schema**. AC-157 counts the former;
  AC-132 the latter; the 17-parameter and 25-optional fixtures are retained.
- The **precondition under which per-schema equals per-request** — one output schema per dispatch and no strict
  user-defined tools on the adapter's command line — is stated as a GIVEN in AC-132 and AC-157 and enforced by a lint
  on the adapter's argv (ADR-0099's command builder), so adding a strict tool later re-opens the budget explicitly.

Checkpoint 5 greps `grep -rnE 'oneOf|anyOf'` over the snapshot directory and expects `oneOf` → 0; the nesting
check and both counts live in the snapshot test itself. The 24/16 are cited as the vendor's Schema complexity limits
with their per-request scope; the sentence "Do not cite them as a platform constraint" is withdrawn.

**2. `StepError` is this closed set, and the kernel and every adapter match it exhaustively.**

| variant | payload | retry class (AC-029/AC-234) | `spoke()` | event arm (AC-360) |
| --- | --- | --- | --- | --- |
| `Transport` | `{ kind: spawn \| io \| decode, detail }` | retry with backoff | false | `step_failed { class: transport }` |
| `Schema` | `{ attempts, last_error }` | retry up to `attempts`, then fail | true | `step_failed { class: schema }` |
| `Timeout` | `{ after_secs }` | retry once, then escalate | true | `step_failed { class: timeout }` |
| `Cancelled` | `{ by: signal \| timeout \| operator }` | never | true | `step_cancelled` |
| `NonZero` | `{ code, stderr_digest }` | never | true | `step_failed { class: non_zero }` |
| `RateLimited` | `{ resets_at: Option<Timestamp> }` | wait until `resets_at`, else the pack delay; never backoff | true | `rate_limited` |
| `NotAdopted` | `{ expected_agent, observed }` | never (configuration) | false | `step_refused { class: not_adopted }` |
| `NotIsolated` | `{ surface: tools \| mcp_servers \| env, diff_digest }` | never (configuration) | false | `step_refused { class: not_isolated }` |

`spoke()` is true when the child produced output the record must retain (the raw-retention pattern from step.rs is
carried; the variant set is redesigned — design :1649 is corrected to say so). `RateLimited` is detected from the
structured `subtype` first and from text markers only as a disclosed fallback recorded on the event, with `resets_at`
taken from the last `rate_limit_event` in the same stream (new criterion AC-523). **Tool-set inequality against the
card's allowlist is `NotIsolated`**, in AC-204 as in AC-202/AC-386 and design §7.4; `NotAdopted` is reserved for the
wrong agent answering. Any further variant is a successor to this record.

**3. S-SCHEMA-CAP is registered** (consumer: the `rk-types` snapshot test, AC-132/AC-157 and AC-129's fail-open guard):
submit a 17-union-parameter schema and a 25-optional schema to the pinned CLI and record the observed failure shape,
because F20 says the failure is silent and nothing in the corpus has observed the vendor's rejection.

## Considered options

### Option — flatten every data-carrying enum to a struct with a unit `kind` plus optional payload fields
Keeps "no `oneOf`" literally. Rejected: it spends the ≤ 24 optional budget on payload fields that are mutually
exclusive by construction, and it turns `StepError` into a struct the kernel cannot match exhaustively — the property
AC-229 exists for.

### Option — adopt AC-131's nesting rule only and allow a top-level `oneOf`
Matches schemars' default output. Rejected: the vendor's page lists `anyOf`/`allOf` and not `oneOf`; a schema outside
the documented contract fails open (F20) and S-SCHEMA-CAP would have to pass before it was safe.

### Option — count branches at a node, as AC-157 had it
Rejected: the vendor counts union-typed parameters, and a nullable `Option<T>` is one under that count and zero under a
branch count — the divergence that could breach 16 with every lint green.

### Option — keep `StepError` at step.rs's six
Rejected: it has no class for the measured exhaustion-on-result (F1's `rate_limit_event` stream) and no refusal
variants for the configuration checks AC-202/AC-204 require.

## Consequences

**Positive**
- One rule, four homes, one grep, one test; a tagged enum is legal and its exported shape is inside the vendor's
  documented contract.
- `StepError` has a table the kernel, the adapter and AC-360's event arms are written against.
- The per-schema/per-request precondition is a GIVEN and a lint, not an assumption.

**Negative**
- A transform in the export path is code that can drift from schemars' output between versions; the golden snapshot
  is what catches it (schemars' own docs say the generated structure may change between versions).
- Two refusal variants and one wait variant add three event kinds to the vocabulary (ADR-0105's table carries them).
- The budget is per exported schema, which is stricter than per request only while the precondition holds.

**What this forbids**
- Forbids a `oneOf` node in any exported schema, and forbids exporting a schema that has not passed the transform.
- Forbids counting union branches per node as the AC-157 quantity.
- Forbids a `StepError` variant not in the table, and forbids an adapter mapping a usage-limit result to backoff.
- Forbids classifying tool-set inequality as anything but `NotIsolated`.
- Forbids describing 24/16 as the project's own budget; they are the vendor's limits with the stated scope.

## Evidence and confidence

- [measured] schemars 1.2.2 renders `#[serde(tag = "kind")]` as `oneOf` when variants are unique (`schema_exprs.rs`
  :347-374, :484); generated locally against the pinned version.
- [documented, platform.claude.com/docs/en/build-with-claude/structured-outputs, fetched 2026-09-10; Wayback
  2026-09-07] the Schema complexity limits table: 24 optional / 16 union-typed parameters per request, 20 strict tools,
  180 s compile; `anyOf`/`allOf` supported, `oneOf` absent; `additionalProperties: false` required.
- [measured, F20] closed-output violations fail open at exit 0 with `subtype: success`.
- [measured, F31] 52/52 payload conformance to the S-TYPED schema — conformance of *payloads*, not of the exported
  schema, and under the pre-0096 repro shape.
- [measured, F1] the `rate_limit_event` stream shape on the pinned CLI; `SDKRateLimitEvent` is now typed in both
  Agent SDK references with `seven_day_opus`/`seven_day_sonnet` keys [documented, fetched 2026-09-10, PIA-069].
- [measured, PIA-016] AC-204 vs AC-202/AC-386/design §7.4 on the class of tool-set inequality.
- **What this evidence does not license.** It does not license claiming the vendor accepts the transformed `anyOf`
  shape for tagged enums — S-SCHEMA-CAP is what observes that. It does not license any retry-class value being
  measured; the classes are design, and AC-029's backoff parameters are unchanged.
