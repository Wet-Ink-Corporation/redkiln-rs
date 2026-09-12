---
id: "0109"
title: reconcile() runs at every entry and nothing skips it; state is two tiers split by the deleted-database test; trailers carry three axes
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-082 (§4.3); references/2026-09-08-architecture-design.md §5.4 :675-683, §5.6 :702-712; AC-160–AC-165, AC-172–AC-176, AC-183; spikes/FINDINGS.md F12, F19; references/2026-09-08-evidence-brief.md §C.3
supersedes: []
superseded_by: []
related: ["0013", "0017", "0018", "0097", "0105", "0107"]
evidence: ["PIA-082", "F12", "F19", "F18"]
consumers: []
---

## Context and problem

Three of the design's most-cited rules carry `decisions: ADR-0018` on AC-160–AC-165, AC-172–AC-175 and AC-183, and
ADR-0018 contains none of them: it decides rewrite events and the wrong-worktree refusal [measured, PIA-082]. The
rules themselves are the spine of the durable-state design and each rests on a measurement:

- **`reconcile()` at every entry, git → store, never the reverse** (§5.6). Drift is 12/174 = 7% overall and 9/79 = 11%
  on gaps over an hour against 3/95 = 3% at or under [measured, F19, one operator, one repository]; 45–48% of gates
  exceed an hour [measured, F18]. Brief §C.3 forces reconcile on every entry and forbids an optimisation that skips it.
- **Two tiers split by one question** (§5.4): if the database were deleted, would `redkiln run` take a different next
  action? Tier 1 is git first — trailers on a checkpoint commit, or an empty commit carrying the trailer; tier 2 is
  the event store, rebuildable or disposable.
- **Three trailer axes** (§5.4 :684): `Story:` alone loses the rejection history — 15/74 = 20.3% of slices passed
  through a rejected state and 21.6% were sealed more than once over 360 trailer commits [measured, F12] — so
  `Slice-Verdict:` is required, and `Gate:` is written when a mark is opened.

Without a record, an implementer adding a reconcile cache or a "unchanged since" flag violates no accepted ADR. This
record gives the three rules their home; ADR-0018 stays as it is and is `related`. The tier-1 write path's mechanics
are ADR-0105's.

## Decision drivers

- Every measured drift figure is a figure about what happens between entries; the only defence is to look at every
  entry.
- The deleted-database test is the one sentence that decides where a field lives; a field with no answer is a compile
  error, not a default (AC-160).

## Decision

1. **Every CLI entry that can cause an action obtains decision-driving state only as the output of `reconcile()` for
   that invocation.** The planner accepts only a `Reconciled<State>` value (AC-172: the compile failure is the
   instrument); exactly one `reconcile_started` event per invocation. **No cache, TTL, "unchanged since" flag,
   configuration or CLI flag skips it** (AC-173). Reconcile mutates nothing in git (AC-174) and is idempotent (AC-175).
   A gate answer bound to a tree hash that no longer matches is void and the gate re-opens with the drift attached
   (AC-176; ADR-0017 at the boundary).
2. **State is two tiers, split by the deleted-database test.** Tier 1 (stage transitions, gate opened/answered,
   verdicts, slice seals) is written to git first — a trailer on a checkpoint commit or an empty commit carrying the
   trailer — and a failed git write leaves zero store rows (AC-160–AC-163). Tier 2 (usage, cost, headroom, wall clock,
   drift observations, inner retries, the step memo per ADR-0097) is event-store only and rebuildable or disposable
   (AC-164; AC-365–AC-370; AC-397–AC-398). A state field with no tier annotation is a compile error.
3. **Trailers carry three axes**: `Story:`, `Slice-Verdict:`, `Gate:`. Resume from `Story:` alone is forbidden; a
   trailer that does not parse against the declared grammar (ADR-0105 clause 3's table) is surfaced as drift, never
   silently skipped (AC-183).
4. **`doctor --rebuild` reproduces the stage-level next action from git alone** after tier 2 is dropped (AC-161/AC-164
   as narrowed by ADR-0097); its differential is over the tier-1 projection ADR-0105 clause 9 defines.

## Considered options

### Option — reconcile on a TTL
The optimisation brief §C.3 forbids. Rejected: drift at gaps under an hour is 3%, not zero, and F18's gate durations
are exactly the gaps a TTL would skip.

### Option — one trailer axis with the verdict inferred from the story's history
Rejected on F12: the rejection history is not recoverable from `Story:` trailers; 20.3% of slices would resume wrong.

### Option — keep citing ADR-0018
Rejected: the rules are not there.

## Consequences

**Positive**
- The reconcile rule, the tier split and the trailer axes have a record the forbids machinery protects.

**Negative**
- `reconcile()`'s own cost is unmeasured anywhere in the brief (design §5.6 says so); every entry pays it.

**What this forbids**
- Forbids any code path that reaches the planner without a `Reconciled<State>` produced in that invocation.
- Forbids a cache, TTL, flag or configuration that skips `reconcile()`.
- Forbids a tier-1 transition recorded only in the store, and forbids a store row for a transition whose trailer
  commit failed.
- Forbids resuming from a single trailer axis.
- Forbids a state field without a tier annotation.

## Evidence and confidence

- [measured, F19] 12/174 = 7% drift; 9/79 vs 3/95 by gap; Wilson intervals overlap (design §5.6's honest reading).
- [measured, F18] 45–48% of gates exceed an hour; maximum 9.4 days.
- [measured, F12] 360 trailer commits / 74 slices; 15/74 = 20.3% ever-rejected; 21.6% sealed more than once — F12's
  registered primary was uncomputable at n=1 and the re-scoping is disclosed; computed across all refs of the frozen
  clone (ADR-0100's bundle is the substrate).
- [documented, brief §C.3] reconcile at every entry, no skip.
- **What this evidence does not license.** It does not license claiming reconcile catches all drift — SM-049's
  detection rule is what makes its target falsifiable; "none was missed in 174" is not a claim the brief makes. It
  does not license any figure for reconcile's cost.
