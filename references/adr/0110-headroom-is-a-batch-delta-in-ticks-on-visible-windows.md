---
id: "0110"
title: Headroom is a batch delta over a named dispatch set in integer ticks, throttled on the visible unified windows with a two-tick margin, never synthesised per family and never credited from a cache metric
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-082 (§4.3), PIA-017 (§3.5), PIA-061 (§4.6); references/2026-09-08-architecture-design.md §7.9 :1233-1235; AC-372–AC-381, AC-394, AC-395; spikes/FINDINGS.md F1, F6, F14, F29; SM-111
supersedes: []
superseded_by: []
related: ["0003", "0016", "0078", "0095"]
evidence: ["PIA-082", "PIA-017", "PIA-061", "F1", "F6", "F14", "F29"]
consumers: []
---

## Context and problem

Design §7.9's three headroom rows are DECIDED and cite ADR-0003, which decides the five ports and no-Postgres; AC-372
through AC-381 carry `decisions: ADR-0003` (78 register-wide citations of ADR-0003, most for rules it does not
contain) [measured, PIA-082]. The rules rest on three of the programme's clearest instrument findings:

- `utilization` is **quantised to 0.01** — one tick — and `five_hour` and `seven_day` are independent, not mirrors
  [measured, F6]; a per-draw figure read off a small sample was **7.8× wrong** (F21's 0.077 against F29's 0.0098 mean
  over 60 draws) [measured, F29]; 40 dispatches / ~1.0M tokens moved `seven_day` 0.100 → 0.100 [measured, F14 — the
  E0 gate was struck, not answered].
- `unifiedWindows` carries **no per-model-family sub-limit key**; the binding limit is invisible [measured, F1].
- Whether cache reads draw down the window is **unanswerable at 0.01 resolution** [measured, F6, F14].

Two register defects sit on the same rules: AC-372/AC-376/AC-377 state tick arithmetic in decimal fractions, so a naive
`f64` implementation passes every named vector and mis-flags 80 of 100 one-tick deltas [measured, PIA-017]; and the
batch delta is undefined across a `resetsAt` window reset and under shared-account fan-out, SM-111's n≥10 floor is in
no criterion, and Phase-1 exit criterion (7) cites SM-109 where SM-111 is meant [measured, PIA-061]. This record gives
the rules their home and fixes the arithmetic. Nothing is superseded; ADR-0003 is `related`.

## Decision drivers

- A number the instrument cannot resolve must not be reported as if it could.
- A family value nobody can read is a value nobody can falsify.
- Integer ticks make the comparison exact; decimal fractions make it a floating-point accident.

## Decision

1. **Headroom is a batch delta over an explicitly named set of dispatches, with `n` on it.** No per-run or
   per-dispatch accessor exists (AC-373). The named set is **the account's complete dispatch sequence between the two
   readings**; `redkiln cost <item>` prints a per-item delta only when the item's dispatches were the only traffic, and
   otherwise prints `not attributable` with the interleaving count. A delta is computed only between readings sharing
   one `resetsAt`; a batch that crosses a reset is split per window and a partial window is reported with its own `n`.
   A set with `n < 10` is reported `low_sample` and no delta is quoted (SM-111's floor, now in AC-372/AC-395,
   mirroring AC-382).
2. **`utilization` is parsed from the JSON decimal into an integer tick count** (hundredths, `u16`); "verbatim" means
   the decimal digits, not an `f64`. A delta of `delta_ticks <= 1` carries `resolution_limited: true` (AC-372). The
   throttle refuses at `ticks >= 100 - margin_ticks` (AC-376) and a configuration with `margin_ticks < 2` is refused
   before any dispatch (AC-377). Test vectors at bases 0.09/0.10, 0.10/0.12, 0.20/0.21 and margins 0.07/0.18 are the
   fixtures.
3. **The throttle consults only `unifiedWindows.five_hour` and `unifiedWindows.seven_day`**; no code path reads,
   derives, estimates or synthesises a per-model-family value (AC-375). The refusal is
   `StepError::RateLimited { resets_at }` appended as an event, without spawning (AC-376; ADR-0095's variant).
4. **No cache metric feeds a headroom figure or projection** (AC-381); cache is a latency/cost lever only.
5. **Phase-1 exit criterion (7)** reads: every dispatch event carries a cost row (AC-394); headroom is reported only as
   a batch delta with `n` and `resolution_limited` (AC-372/373/395; SM-111), and `redkiln cost` renders the flag.

## Considered options

### Option — interpolate below the tick
Rejected on F6/F29: a per-draw figure read off a small sample was already 7.8× wrong.

### Option — derive a per-family estimate
Rejected on F1: the binding sub-limit has no key; a synthesised figure is unfalsifiable and SM-112's kill is what
would tell us the proxy failed.

### Option — credit cache reads against the window
Rejected on F6/F14: unanswerable at 0.01 resolution.

## Consequences

**Positive**
- The arithmetic is exact and the criteria test it; the batch delta is defined across resets and fan-out.

**Negative**
- `not attributable` will be the common answer on a shared account; that is the true state.

**What this forbids**
- Forbids a per-run or per-dispatch headroom accessor.
- Forbids a delta across a `resetsAt` boundary, or one quoted at `n < 10`.
- Forbids comparing `utilization` as a floating-point value.
- Forbids any per-family headroom value, and forbids a cache metric entering a headroom figure.

## Evidence and confidence

- [measured, F6] 0.01 quantisation; window independence. [measured, F29] 0.0098 per draw over 60 draws; F21's 0.077
  superseded. [measured, F14] 0.100 → 0.100 over 40 dispatches (~1.0M tokens — F14's ~1.9M is corrected in ERRATA per
  PIA-077). [measured, F1] no family key in `unifiedWindows`.
- [measured, PIA-017] the f64 failure on 80 of 100 one-tick deltas; [measured, PIA-061] the undefined reset case.
- **What this evidence does not license.** It does not license any headroom projection; it does not license quoting
  a per-item delta on a shared account.
