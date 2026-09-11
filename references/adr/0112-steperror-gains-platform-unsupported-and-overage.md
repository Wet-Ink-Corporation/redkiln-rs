---
id: "0112"
title: StepError gains Platform, Unsupported and Overage — the three variants thirteen criteria and design §3.6 already named — making the closed set eleven
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md §0.3 row C5 (recorded 2026-09-11 while absorbing ADR-0095); AC-027, AC-029, AC-218, AC-221, AC-233, AC-234, AC-237, AC-238, AC-328, AC-378, AC-428, AC-494, AC-500; references/2026-09-08-architecture-design.md §3.6 :340, :361; spikes/FINDINGS.md F1 :42-48, F21; spikes/results/e3.jsonl
supersedes: ["0095"]
superseded_by: []
related: ["0003", "0010", "0029", "0099"]
evidence: ["PIA-005", "F1", "F21", "F29", "F41"]
consumers: []
---

## Context and problem

ADR-0095 decision 2 fixed `StepError` as a closed **eight**-variant table — `Transport`, `Schema`, `Timeout`, `Cancelled`,
`NonZero`, `RateLimited`, `NotAdopted`, `NotIsolated` — and forbade any variant not in it. It was written from the
audit's census of step.rs's six plus the two refusal classes PIA-014 and PIA-016 added, and it did not census the
register. The register, absorbing ADR-0095 the same day, found thirteen criteria and design §3.6 naming **three more**
as variants and recorded the gap as contradiction row C5 rather than re-mapping them [measured, register §0.3 C5]:

- **`Platform { status, retry_after }`** — a result stream carrying `is_error: true` or an `api_error_status`, whatever
  the exit code (AC-218; AC-029's backoff; AC-233, AC-237, AC-428). Measured: seven `is_error` frames in `e3.jsonl`
  (six 529, one 500) [measured, F21 as corrected 2026-09-11]; RK-B0013's committed `changes-requested` seal from one
  HTTP 529 is the failure the variant exists to name.
- **`Unsupported(capability)`** — a capability with no dated probe at the current version pin (AC-238, AC-494,
  AC-500; AC-328's `affected-selection`), never a generic error, never a retry, never a pass (design §3.6 P7, :361).
- **`Overage`** — a `rate_limit_event` whose `rate_limit_info.isUsingOverage` is `true`, refused before spawn under
  ADR-0010's subscription-only rule (AC-221, AC-378; AC-234 says it is never retried). The envelope is recorded on
  every `rate_limit_event` payload (`isUsingOverage`, `overageDisabledReason`, `overageStatus`; F1 :42-48) and no
  true-valued instance exists in the 80 recorded rows, so the class is real and its firing is unmeasured.

ADR-0095's forbids bullet — "forbids a `StepError` variant not in the table" — makes those thirteen criteria
unbuildable as written, and mapping them away (`Platform` onto `Transport`/`NonZero`; the two refusals onto
`step_refused`) would lose the retry class AC-029 keys on and the pre-spawn refusal AC-221 keys on. The variants are
right; the table was short. This record supersedes ADR-0095 **in part**: decision 2's table grows by three rows; the
schema rule (decision 1), S-SCHEMA-CAP (decision 3) and every other forbids bullet stand; ADR-0095 stays `accepted`.

## Decision drivers

- A closed set the kernel matches exhaustively must contain every variant a criterion can produce, or the criterion
  is unbuildable — the property AC-229/AC-360 exist for cuts both ways.
- `Platform` is the one class with a measured instance (seven frames) and a distinct retry class; folding it into
  `Transport` would retry a 500 the same way as a spawn failure.
- The two refusals are configuration-time by design (§3.6's "never retried" row) and need their own event arm so
  `doctor` and the record can name them.

## Decision

**`StepError` is this closed set of eleven, and the kernel and every adapter match it exhaustively.** The eight rows
of ADR-0095 decision 2 are unchanged; the three added rows are:

| variant | payload | retry class (AC-029/AC-234) | `spoke()` | event arm (AC-360) |
| --- | --- | --- | --- | --- |
| `Platform` | `{ status: u16, retry_after: Option<u64> }` | retry with backoff (base 2 s, cap 5 min — AC-029's constants), honouring `retry_after`; the exit code is never consulted to overturn the class (AC-218) | true (the stream produced a frame the record must retain) | `step_failed { class: platform, status }` |
| `Unsupported` | `{ capability: String }` | never (configuration); a pack whose only route through a stage needs the capability fails validation with a span at the requiring row (design §3.6 P7) | false | `step_refused { class: unsupported, capability }` |
| `Overage` | `{ reason: Option<String> }` (from `overageDisabledReason` / `overageStatus`) | never; refused **before spawn** (AC-221; ADR-0010) | false | `step_refused { class: overage }` |

`Platform` is distinct from `RateLimited`: a usage-limit result resolves to `RateLimited { resets_at }` (ADR-0095;
AC-523) and waits; a 5xx or `is_error` frame resolves to `Platform` and backs off. `Overage` is distinct from
`RateLimited`: it is a billing state read from the same payload and it refuses. Any further variant is a successor
to this record. The register's row C5 is resolved by this record; the thirteen criteria keep their names.

**S-PORTS is re-run on eleven variants** (`spikes/s-ports/`): the exhaustive-match test carries eleven cases and one
event arm each; F41 gains a dated note.

## Considered options

### Option — map the three onto existing rows
`Platform` → `Transport` or `NonZero`; `Unsupported` and `Overage` → `NotIsolated`/`NotAdopted` or a bare
`step_refused`. Rejected: `Platform`'s retry class differs from `Transport`'s (honour `retry_after`) and from
`NonZero`'s (never); the two refusals have consumers (`doctor` rows, AC-494's never-a-pass clause) that key on the
name; and thirteen criteria would be rewritten to match a table that was written without reading them.

### Option — leave C5 open until Step 5.3
Rejected: Step 5.3 snapshots `StepError`; an open contradiction at the snapshot is two implementers' worth of
divergence (PIA-005's own argument).

### Option — supersede ADR-0095 in full
Rejected: decision 1 (the schema rule) and decision 3 (S-SCHEMA-CAP) are untouched; a full supersession would move a
record that is right in two of its three parts.

## Consequences

**Positive**
- Every `StepError` a criterion names is in the table; the kernel's exhaustive match covers thirteen more criteria.

**Negative**
- Eleven event arms instead of eight; ADR-0105's event-kind table gains three rows.
- `Overage`'s firing is unmeasured (no true-valued instance in 80 rows); its fixture is scripted, not recorded.

**What this forbids**
- Forbids mapping a 5xx / `is_error` result onto `Transport`, `NonZero` or `RateLimited`.
- Forbids retrying `Unsupported` or `Overage`, and forbids spawning a child after an `Overage` read.
- Forbids a twelfth variant without a successor to this record.

## Evidence and confidence

- [measured, register §0.3 C5, 2026-09-11] thirteen criteria and two design rows naming the three variants; ADR-0095's
  table omitting them.
- [measured, F21 as corrected; `spikes/results/e3.jsonl`] seven `is_error` frames — six 529, one 500 — the `Platform`
  instances; RK-B0013 the committed-seal failure.
- [measured, F1 :42-48; PIA-065] `isUsingOverage` / `overageDisabledReason` / `overageStatus` on every
  `rate_limit_event` payload; `false` on all 20 E0 and 60 S-VAR rows.
- [measured, F41 + the 2026-09-11 re-run] the eleven-variant enum compiles and matches exhaustively on 1.98.1.
- **What this evidence does not license.** It does not license any rate for `Platform` or `Overage` under the
  product adapter; the classes are named from the runner's recorded payloads and the first product dispatch is the
  first measurement.
