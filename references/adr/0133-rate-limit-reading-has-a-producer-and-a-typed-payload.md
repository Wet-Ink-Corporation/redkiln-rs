---
id: "0133"
title: "`rate_limit_reading` gains the producer and the typed payload it has never had — the Harness adapter appends one per reading through the `&mut dyn EventStore` it already holds, carrying `windows` (the tick pair `doctor` already reads), `resets_at` **per window**, and the binding-window discriminator; the durable kind keeps the wire name `rate_limit_reading` and the platform's stream-line type `rate_limit_event` is not renamed; no `EventKind` variant is minted; and **RT-19 is settled** — `total_cost_usd` is `CostUsd(String)`, the decimal digits verbatim, with what a reader may and may not do with them fixed here rather than by the first call site"
status: proposed
date: 2026-09-21
legacy_id: none
source: references/adr/0110 decision 1 (*"A delta is computed only between readings sharing one `resetsAt`; a batch that crosses a reset is split per window"*) and decision 3 (*"The throttle consults only `unifiedWindows.five_hour` and `unifiedWindows.seven_day`"*) and decision 5 (Phase-1 exit criterion (7)); references/adr/0126 decision 6 (*"The typed per-kind payload record (`s7a` §7a.5: 'The typed-payload record, per kind, off a design §5 table that must first be given types, is OWED')"* — the deferral this record discharges); references/2026-09-08-architecture-design.md §5.4's event-kind table row *"`rate_limit_event` (as read) / headroom reading | 2 | `utilization` in integer ticks, `resetsAt`, window | the adapter's stream reader (AC-380)"*; references/2026-09-08-acceptance-criteria.md AC-380 (the sole headroom source), AC-372/AC-373/AC-395 (the batch delta, `n`, `resolution_limited`), AC-220 (`doctor`'s headroom margin); `crates/rk-types/src/event_kind.rs` — `RateLimitReading` is already one of the 38 variants and already carries the wire name `rate_limit_reading`; the operator's decision of 2026-09-21, recorded as `spikes/DECISIONS-2026-09-21.md` D21-5
supersedes: []
superseded_by: []
related: ["0095", "0099", "0110", "0126"]
evidence: ["F6", "F31"]
consumers: []
---

## Context and problem

**ADR-0110 defines headroom as a batch delta over readings sharing one `resetsAt`, and nothing journals a
`resetsAt`.** The kind exists and the producer does not. Measured 2026-09-21:

- `EventKind::RateLimitReading` is one of the enum's 38 variants and `wire_name()` already returns
  `rate_limit_reading`. Its **only three references in the workspace** are kernel exhaustive-match no-op arms —
  `workflow/breaker.rs`, `workflow/fsm.rs`, `workflow/memo.rs` — each classifying it as tier 2. **Nothing constructs
  an `Event` of this kind and nothing appends one.**
- The row that *is* written on every dispatch, `DispatchRecord`, carries `headroom_before` / `headroom_after` as
  `WindowReading { five_hour, seven_day }` — **utilisation ticks and nothing else**. No `resetsAt`, no window
  discriminator. Of the three payload members design §5.4's row names — *"`utilization` in integer ticks, `resetsAt`,
  window"* — the recorded shape carries **one**.
- **The value is already in hand and is discarded.** `rk-adapter-claude`'s `headroom::read_rate_limit_event` returns a
  `RateLimitReading` carrying `resets_at` and, under `Shape::Unified`, the per-window timestamps reachable through its
  `resets_at(window)` accessor; `ClaudeHarness::last_reading()` exposes it. The gap is a missing **producer**, not a
  missing measurement.

**The one consumer reads a shape nothing writes.** `doctor`'s `RK-D-HEADROOM-MARGIN` (AC-220's doctor half) does
`of_kind(ctx, "rate_limit_reading").last().and_then(|r| r.payload.get("windows")?.get("five_hour")?.as_i64())`. That
payload shape exists **only in three hand-authored fixture lines**. In production the filter matches nothing, the rule
returns `Vec::new()`, and a warning that can never fire reads exactly like a warning that never needed to —
**a filter that matches nothing is a green**.

**This is a deferral coming due, not a discovery.** ADR-0126 decision 6 routed *"the typed per-kind payload record …
off a design §5 table that must first be given types"* forward by name, and `WindowReading`'s own doc says
*"`resets_at` per window is the typed-payload record's (ADR-0126 decision 6)"*. This is that record.

**Phase-1 exit criterion (7) is not computable without it.** ADR-0110 decision 5 makes criterion (7) *"headroom is
reported only as a batch delta with `n` and `resolution_limited`"*, and decision 1 makes a delta legal only between
readings **sharing one `resetsAt`**. With no `resetsAt` in the event stream the grouping cannot be formed at all.

## Decision drivers

- Criterion (7) is one of the two Phase-1 exit criteria that **failed, measured**, at Step 8. The batch-delta rule is
  uncomputable from the stream as recorded.
- F6's quantisation finding is *why* the `resetsAt` grouping is necessary: `utilization` quantises to 0.01, per-run
  cost is not measurable, and a delta over "readings in one run" would be a different measure wearing the same name.
  Amending ADR-0110 toward what happens to be recorded would silently redefine the measure.
- The one existing consumer must keep working. A payload that drops `windows.five_hour` would turn `doctor`'s silent
  green into a silent red-shaped green — the rule would still return `Vec::new()`, for a new reason.
- `rk-types`, `rk-ports`, `rk-store`, `rk-git` and the four adapters are not edited by the stage that builds this
  (`s7d` §7d.15's floor, carried forward), so a decision that required minting an `EventKind` variant would be
  foreclosed. It does not: the variant already exists.

## Considered options

1. **Journal the reading with a typed payload (taken).** Discharges ADR-0126 decision 6 for this kind, satisfies
   ADR-0110 decision 1's grouping, and keeps `doctor`'s existing read working unchanged.
2. **Amend ADR-0110 to define headroom over what `DispatchRecord` already carries.** Rejected: it redefines the
   measure to fit the recording. F6 is the reason the `resetsAt` grouping exists.
3. **Rename the durable kind to design §5.4's `rate_limit_event`.** Rejected, and the reason is concrete rather than
   conservative: **`rate_limit_event` is already a live, distinct identifier** — it is the Claude CLI stream-line
   `type` value, classified by the adapter-local `rk_adapter_claude::event::EventKind::RateLimitEvent` whose
   `census_key()` returns the literal string `"rate_limit_event"`. The design's name is taken by a different concept
   in a different enum, and collapsing them would make the census and the journal indistinguishable by name.
4. **Derive `Serialize` on `RateLimitReading` and serialise it whole.** Rejected: it is an adapter type and the
   journal's payload is a durable wire shape. ADR-0126 decision 2's principle — *a field of an `rk-types` struct has
   an `rk-types` type* — applies to the payload as well, and the adapter struct carries members the journal has no
   business pinning (`rate_limit_type`, `overage_status`).

## Decision

1. **The durable kind keeps the wire name `rate_limit_reading` and no `EventKind` variant is minted.** Unlike
   ADR-0123, ADR-0124, ADR-0125 and ADR-0126, this record has **no arity clause**: `EventKind::ALL` stays `[Self; 38]`,
   `RateLimitReading` is already in it, and `wire_name()` already returns `rate_limit_reading`. **Design §5.4's
   `rate_limit_event` spelling is the platform's stream-line type and is not renamed**; the design row is corrected to
   name the durable kind `rate_limit_reading` and to note the two are different things. The three kernel match arms
   stand as written — tier 2, no-op.

2. **The producer is the Harness adapter, once per reading, through the `&mut dyn EventStore` it already receives.**
   This is ADR-0126 decision 1's route for `dispatch_completed`, reused rather than reinvented: the adapter already
   holds the store for the duration of a dispatch and already appends on every termination path. A reading is appended
   when the adapter's stream reader parses a `rate_limit_event` line (AC-380's sole headroom source), **whether or not
   the dispatch it accompanies succeeds** — a reading taken before a refusal is still a reading of the account.

3. **The payload, named against design §5.4's three members and against its one existing consumer:**

   - **`windows`** — `{ five_hour: <u16 ticks>, seven_day: <u16 ticks> }`. This is design §5.4's *"`utilization` in
     integer ticks"* and is **byte-compatible with the shape `doctor`'s `RK-D-HEADROOM-MARGIN` already reads**. The
     key name and nesting are fixed by that existing reader and are not re-chosen here.
   - **`resets_at`** — `{ five_hour: <RFC 3339 | null>, seven_day: <RFC 3339 | null> }`, **per window**, read through
     the adapter's `resets_at(window)` accessor so a `Shape::Flat` reading falls back to the top-level value exactly
     as the accessor already defines. This is design §5.4's *"`resetsAt`"* and is the member ADR-0110 decision 1's
     grouping is computed over. **Absent, not zero** — a window with no timestamp is `null` and a delta across it is
     not formed.
   - **`rate_limit_type`** — the platform's own view of the binding window, verbatim or `null`. This is design §5.4's
     *"window"* member: the discriminator saying which window is binding, which neither `WindowReading` nor the tick
     pair carries.

   `utilization` stays **integer ticks, never an `f64`**, per ADR-0110 decision 2. The payload carries no delta: a
   delta is a batch computation over readings (ADR-0110 decision 1) and is `redkiln cost`'s, never a row's.

4. **The event is tier 2, and the tier is argued rather than assumed.** ADR-0109 decision 2 requires that dropping the
   event store not change `redkiln run`'s next action. A headroom reading changes no next action: the throttle's
   refusal is `StepError::RateLimited { resets_at }` appended as its own event (ADR-0110 decision 3, ADR-0095's
   variant), and that refusal — not the reading — is what gates a dispatch. Readings are rebuildable by re-reading
   retained stream bytes and disposable in the sense AC-164 requires.

5. **RT-19 is settled here, and this record is its named owner.** RT-19's register row
   (`spikes/DECISIONS-2026-09-17.md`) gives its owner as *"the typed-payload record (ADR-0126 decision 6) names the
   cost unit and any arithmetic form"*, and D21-5 settles it *"in the same pass, because the renderer is what forces
   the unit question"*. The unit is **`Option<CostUsd>`** — `rk_types::usage::CostUsd(String)`, **the decimal digits
   verbatim as the CLI printed them**, taken from the raw result bytes by the same scanner AC-374 uses for ticks,
   **never an `f64` and never a fixed scale**. `CostUsd` has **no `impl` block at all** on this tree — no `Display`,
   no `FromStr`, no arithmetic — and that is a property to preserve, not an omission to fill.

   **What a reader may do with the digits:** compare two values for byte equality; render them verbatim; sum them
   **as decimals** with no intermediate binary floating-point step, carrying every digit either operand carried.

   **What a reader may not do:** parse them to `f64` (or any binary float) at any point, including transiently inside
   a sum or a comparison; round, truncate or re-scale them to a fixed number of decimal places; or infer a currency
   minor unit from them. The withdrawn `CostMicros`/`u64` form is the worked example of why: the recorded F20 value
   `2.337147` is the runner's `f64` re-serialisation, so **the CLI's true digit count is UNCONFIRMED at the pin** and
   a six-decimal scale would truncate or round a longer value silently. `DispatchRecord` derives `Eq` — an
   `Option<f64>` does not compile under it — and `rk-store`'s canonical serialiser refuses floats and accepts
   strings, so the type system and the serialiser both already enforce this; clause 5 records **why**, so that a
   future caller reaching for `.0.parse::<f64>()` is refused by a record and not merely by a missing impl.

**What this record does NOT decide.** It does not define `redkiln cost`'s rendering, its `n`, its `low_sample` floor
or its `resolution_limited` flag — those are AC-372/AC-373/AC-395 and ADR-0110's, and `s7i` builds them. It mints no
`EventKind` variant and edits no `rk-types` struct.

## Consequences

**What this forbids**

- Forbids computing a headroom delta across readings that do not share a `resetsAt` for the window being measured
  (ADR-0110 decision 1, unchanged and now computable).
- Forbids a payload that drops `windows.five_hour` / `windows.seven_day` or renames them. The existing `doctor` reader
  is a consumer of this shape and a rename would silently restore the never-fires state this record exists to end.
- Forbids coercing an absent `resets_at` to a zero, an epoch or "now". Absent is `null`, and a delta over it is not
  formed.
- Forbids reading a `rate_limit_reading` row as a per-dispatch or per-run cost attribution. AC-373 has no such
  accessor and F6 is why.
- Forbids renaming the adapter-local `rate_limit_event` stream-line type to match the durable kind, or vice versa.
- Forbids parsing `CostUsd`'s digits to a binary float at any point, and forbids re-scaling them to a fixed number of
  decimal places (clause 5). `CostUsd` gaining a `Display`, a `FromStr` or an arithmetic impl that routes through
  `f64` is the defect this bullet names in advance.

**What follows**

- `doctor`'s `RK-D-HEADROOM-MARGIN` fires on real data for the first time. Its current green is vacuous and its
  becoming non-vacuous is a change in what the tree knows, not a regression.
- `conformance_every_trailer_key_has_a_producer`'s sibling question — a design §5 row with a producer named and none
  built — is closed for this kind. It remains open for the rows §7f.1's trailer lint covers, which is `s7h`'s.
- Phase-1 exit criterion (7) becomes computable. It is **not thereby met**: `redkiln cost` must still render the
  delta, `n` and `resolution_limited`, which is `s7i`'s.

## Evidence and confidence

- [measured 2026-09-21] `grep -rn 'EventKind::RateLimitReading' crates/` → exactly 3 hits, all kernel no-op match
  arms (`workflow/breaker.rs`, `workflow/fsm.rs`, `workflow/memo.rs`). No construction, no append.
- [measured 2026-09-21] `RateLimitReading` derives `Debug, Clone, PartialEq, Eq` **only** — no `Serialize` — and
  carries six fields: `shape`, `status`, `resets_at`, `rate_limit_type`, `is_using_overage`, `overage_status`. The
  per-window timestamps are fields of the `Shape::Unified` variant, not of the struct, and are reached through the
  `resets_at(window)` accessor.
- [measured 2026-09-21] `EventKind` has **38** variants, `RateLimitReading` among them, and carries no serde
  attributes at all — the wire name comes from a hand-written `wire_name()` with 38 exhaustive arms.
- [measured 2026-09-21] `crates/rk-cli/src/doctor/rules.rs` is the only consumer, reading
  `payload["windows"]["five_hour"]`; that shape appears in three fixture lines and nowhere else.
- [documented] F6's 0.01 `utilization` quantisation is ADR-0110's own evidence for the batch rule and is not
  re-measured here.
- **Confidence: high on the gap, and the limit is stated.** That the producer is absent is a measured absence over a
  named grep, which is the strongest form available for an absence claim. What is **not** measured here is that the
  payload above is sufficient for AC-372's `resolution_limited` arithmetic on real traffic — no run on this tree has
  yet produced two readings sharing a `resetsAt`, because none has produced one. That first measurement is `s7i`'s
  and this record does not anticipate its result.
