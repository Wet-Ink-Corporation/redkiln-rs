---
id: "0025"
title: Human-supplied artifacts are vouched roots with advisory drift
status: accepted
date: 2026-09-07
legacy_id: PC3
source: references/2026-09-07-process-catalog-proposal.md §7
supersedes: []
superseded_by: []
related: []
evidence: ["F18", "F28", "Make source roots", "Fiberplane Drift warn-on-change", "HELENA effort-decay"]
consumers: []
---

## Context and problem

An artifact a human pastes in (a competitor analysis, a design decision) has no producer and so no natural staleness check under the selection model's `derived_from` digest mechanism (§2.1, §2.5 step 3). The proposal needed a rule for what happens when an upstream artifact in the same stage later changes underneath a human-supplied one, without either silently going stale or re-triggering work a human already vouched for. PC3's grounding column cites F18 ("drift 3.6× past 1 h"), F28 ("gate cost") and HELENA effort-decay.

## Decision drivers

- F18, cited by the row as "drift 3.6× past 1 h". In its own home (the human-gate duration study) F18 measured repository drift 3.6× higher past a one-hour window while a gate is open; the row does not restate it as a measurement of human-supplied-artifact staleness.
- F28, cited by the row only as "gate cost". The row gives no figure and does not say what was priced; elsewhere F28 is the per-resume prefix-rewrite cost of a gate (~18.6k tokens), not a measurement of gating on upstream change.
- HELENA effort-decay, named in the grounding column with no elaboration in the source row.
- The selection model's `derived_from` staleness mechanism (§2.1) has no producer to re-run for a human-supplied node, so "re-materialise on staleness" (the default path in §2.5 step 3) does not apply here.

## Considered options

### Option — never stale, silent
Treat a human-supplied artifact (`derived_from = []`) as permanently valid once pasted, with no signal when its upstream context changes. Rejected: this lets drift accumulate invisibly. The source row does not state a per-alternative reason; F18's cited "drift 3.6× past 1 h" is the grounding nearest to this failure mode, but the row does not attribute that measurement to this option.

### Option — human declares inputs at paste
Require the human to enumerate the artifact's dependencies explicitly at the moment of pasting, and gate on those. The source row does not say why it lost; what the chosen mechanism does differently is keep the burden off the human at paste time, since `derived_from = []` requires no declaration.

### Option — stale → gate
Treat a human-supplied artifact as normal `Stale` output the moment its upstream changes, opening a hard gate (blocking) as for machine-produced artifacts. The source row does not state a reason; the two groundings pointing away from it are F28 ("gate cost", no figure given) and HELENA effort-decay. The chosen decision's "never blocks or re-runs" is the direct negation of this option.

## Decision

**Vouched root + advisory drift**: a human-supplied artifact is recorded with `derived_from = []`, and this **never blocks or re-runs**. When an upstream artifact in the same stage subsequently changes, the *next* gate the item reaches carries an advisory note — "human artifact predates X v2" — rather than opening a gate of its own or silently doing nothing.

## Consequences

**Positive**
- Drift becomes visible at the point a human is already looking (the next gate), rather than invisible (silent) or costly on every change (hard gate).
- The human who supplied the artifact is not asked to pre-declare its dependencies at paste time.

**Negative**
- The advisory can be missed or dismissed like any other advisory note; nothing in this decision forces a human to act on it.
- Drift between the upstream change and the next gate is still unflagged in real time — the advisory is deferred, not immediate.

**What this forbids**
- Blocking or re-running work on a human-supplied artifact purely because `derived_from` is empty or because an upstream changed.
- Silently doing nothing when an upstream a human artifact depended on changes — the advisory is mandatory, even though it does not block.

## Evidence and confidence

- [measured] F18: "drift 3.6× past 1 h" is all the source row says. The underlying finding measured repository drift while human gates are open; n, methodology and its bearing on human-*supplied* artifacts are not given here.
- [documented] F28 is named only as "gate cost". No figure appears in the row, and the row does not say what gating scenario was priced — this ADR states no number.
- [documented] HELENA effort-decay is a named grounding with no elaboration in the source. Any account of *why* repeated gating degrades human review is this reader's inference, not a claim the row makes.
- [documented] Fiberplane's "Drift" warn-on-change feature and Make's source-roots concept are cited as prior-art precedent for the advisory-not-block shape.
- [unmeasured] The advisory's actual noise rate is untested. §6.1 refines spike **P2** to "add PC3's advisory-drift path: measure advisory noise rate alongside precision/recall" — P2 is unrun, so nothing here licenses a claim that the advisory will be neither ignored nor spammy.
