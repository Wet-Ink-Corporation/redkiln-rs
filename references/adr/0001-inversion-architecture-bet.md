---
id: "0001"
title: Adopt the inversion architecture (Bet 1) on the spike evidence
status: accepted
date: 2026-09-05
legacy_id: D1
source: references/2026-09-05-architecture-proposal.md §9 (choice grounded in §1, §3, §4)
supersedes: []
superseded_by: []
related: ["0002", "0003"]
evidence: []
consumers: []
---

## Context and problem

The spike programme (§1) closed the RFC's kill questions on the old redkiln design: headless dispatch does not degrade review quality (F23/F26), effort and model pin per dispatch (F7/F15), the prompt cache survives commits under `--system-prompt` (F13/F16), suspension across a process boundary is lossless (F19), and a human gate costs at most one prefix rewrite to resume regardless of its length (F28 as corrected, S-H1b). Against that evidence, the D1 row records the choice as "Inversion (Bet 1) on the §1 evidence" and leaves its alternatives column empty (`—`). "Bet 1" is the RFC's label (`references/2026-08-31-rust-inversion-RFC.md` §0), where it names inverting control — "Rust owns the loop and spawns `claude -p` per step". The RFC's Bet 2 (porting the old engine's `src/` to Rust) is not a competing architecture: the RFC states the two "are separable" and recommends taking Bet 1 while staging Bet 2 behind spikes. So the proposal's D1 row records a bet taken, not a choice between two scored options.

## Decision drivers

- The spike results in §1 (headless-quality parity, deterministic resume, flat gate cost, cache survival) removed the technical objections to a kernel that owns the loop and dispatches subprocesses.
- §3's principles impose determinism on control flow ("Rust holds the loop; the model holds no control flow") and treat human gates as durable marks rather than waiting processes — the behaviour the inversion shape is built to provide.
- §0 states the reviewer's verdict is unstable at the threshold (F27, S-VAR) while its findings are not (S-TYPED), pushing the blocking decision out of the model and into code — a requirement the inversion architecture is built to satisfy.

## Considered options

### Option — do nothing (keep the old redkiln's architecture)

The D1 row's alternatives column is empty (`—`) and no rejected architecture is named there. Reconstructed baseline: continuing on the old redkiln would mean the model continues to hold control flow, human gates continue as waiting processes, and the reviewer's verdict continues to be trusted directly — three positions the §1 spike results and the §3 principles argue against. The source does not record this option as having been weighed; it is reconstructed here only because the row leaves the column empty, and it is marked as such rather than presented as a rejected candidate.

### Not an alternative — the RFC's "Bet 2" (port the old engine to Rust)

Recorded for completeness because the "Bet 1" label implies a second bet. The RFC treats Bet 2 as separable from Bet 1 and stages it behind spikes rather than rejecting it in favour of Bet 1; D2 (clean break) — not this row — is where the old engine's fate is settled. It therefore contributes no losing reason to this decision.

## Decision

Build Redkiln-RS as "a clean-break, single Rust binary that owns the loop": a deterministic workflow kernel (stage selection, gates, budgets, resume) that dispatches one harness process per activity, keeps durable state in git trailers plus an append-only event log, exposes human gates as durable marks rather than waiting processes, and treats every agent output as a typed value of which markdown is a rendering, never a source (§0). This is recorded as D1, "Architecture: Inversion (Bet 1) on the §1 evidence."

## Consequences

**Positive**
- Every `advance`, gate, retry and budget becomes code, making the `--stay` class of corruption unrepresentable (§3, principle 1).
- Human gates become durable marks plus a later read, which the source records as having "paid for itself four times" operationally, and again for H1's 65-minute and 49.7-hour reads (§3, principle 3; the reads are F28's, corrected 2026-09-05 — S-H1b is the separate static-`--system-prompt` read at 66 min).
- The reviewer's verdict instability (F27) is absorbed by moving the blocking decision into policy over typed findings rather than trusting the model's word (§0, §4.6).

**Negative**
- The source does not quantify a cost of the inversion bet itself (no rejected alternative is scored); the decision rests on the accumulated spike evidence in §1 rather than a head-to-head comparison.

**What this forbids**
- The model may not hold control flow: selection is deterministic in Rust, and the agent may only classify or request a typed deviation, never change control flow directly (§3, principle 1).
- Human gates may not be implemented as live waiting processes (§3, principle 3).
- A reviewer's own verdict may not be treated as the block/pass signal; the block must be computed by policy from typed findings (§0, foreshadowing D4's rev. 4 rebuild).

## Evidence and confidence

- [measured] Headless dispatch parity: 23/23 vs 23/23 detection, 20/20 across two harder tiers, McNemar p=0.625 (F23, F26).
- [measured] Deterministic resume: 174/174 lossless across a process boundary (F19, H2).
- [measured] Gate resume cost is flat regardless of gate length: 18,603 tokens at both 65 minutes and 49.7 hours under the default prompt (F28 (H1), corrected 2026-09-05; §1 row 11). The companion static-`--system-prompt` read is S-H1b's 29,619–29,680 at 66 min, and its 24 h read was not taken.
- [measured] Reviewer verdict flips on 2/4 items while detection is stable (0/7 flaky draws) — the specific instability that drives the findings/policy split (F27).
- [documented] "Bet 1" is defined in the RFC (`2026-08-31-rust-inversion-RFC.md` §0) as inverting control — Rust owns the loop and spawns `claude -p` per step — and the RFC records Bet 2 (the engine port) as separable and staged, not as the losing side of this choice.
- This decision does not, on its own evidence, license a claim that the inversion architecture outperforms any specific rejected alternative — no such alternative is scored in the read material.
