---
id: "0029"
title: Technology packs add facts, producers and artifact kinds, never rubric rows
status: accepted
date: 2026-09-07
legacy_id: PC7
source: references/2026-09-07-process-catalog-proposal.md §7 PC7
supersedes: []
superseded_by: ["0073"]
related: ["0014", "0022"]
evidence: ["Proposal D14 / §4.2.1 selector specificity", "S-BIN spike", "F21", "Cloud Native Buildpacks detect-and-contribute", "Tekton Pipelines vs Tasks"]
consumers: []
---

## Context and problem

D14 established that process packs own structure and technology packs bind and contribute at slots only, never merging into or redefining process-pack structure. PC7 is the artifact-model elaboration of that D14 slot-filler idea for a catalog that must serve toolchain-agnostic activities (discovery, spec, design, planning, closeout) alongside toolchain-realised ones (implement/verify/review/release producers, `bisect`, `minimal-repro`, `profiling`). A story touching both Node and UE content needs both toolchains' change summaries, which rules out computing toolchain facts once per repo. The row settles exactly what a technology pack may and may not do.

## Decision drivers

- D14's existing rule that technology packs bind and contribute at slots, never own structure — PC7 must not violate it.
- Mixed-toolchain items (e.g. Node + UE content in one story) require per-item, not per-repo, toolchain classification, or a story would silently get only one toolchain's artifacts.
- The §4.2.1 selector-specificity rule (toolchain match beats generic; equal-specificity ties are a validation error) needs to extend cleanly to producer contribution, not just card selection.
- F21's measured timeout spread (2055 s for Node reviews vs. a 7200 s budget for UE) means per-activity timeout/cost overrides need to be a legitimate pack lever, not a catalog-wide constant.
- Wanting the toolchain-agnostic ~70% of the catalog (per the row's own estimate) to ship once, with tech packs supplying only the toolchain-specific specialisation.

## Decision

A tech pack may:

(i) add **facts** to the context vector — `toolchains`, `change_kinds` {text, binary, generated}, `binary_change` — computed **per item from affected paths** via `Toolchain::classify`, never per repo, because "a story touching Node + UE content needs both change summaries";

(ii) contribute **producers** for existing artifact kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `ProfileReport`…) selected by specificity — toolchain match beats generic, equal-specificity ties are a validation error (same rule as §4.2.1 cards);

(iii) contribute **artifact kinds** (`CookReport`, `AssetManifest`, `AutomationReport`, `Captures`) and additive `contribute` rows at a declared stage (e.g. `[[contribute.verify]] when = "'unreal' in toolchains" artifacts = ["CookReport"]`) — "the artifact-model form of D14's slot filler";

(iv) override `timeout_secs` / `cost_class` per activity (grounded in F21's measured 2055 s Node reviews vs. UE's 7200 s budget).

A tech pack may **not** add stages, remove baseline artifacts, or edit `require` rows — "that remains a process-pack variant (`story-ue`) per D14."

Catalog consequence: toolchain-agnostic activities ship once; toolchain-realised ones ship as a generic text-change producer with `applicable_if = "'text' in change_kinds"`, and each tech pack supplies its specialisation.

## Considered options

### Option — tech pack as a separate catalog per toolchain
Give each toolchain its own full catalog rather than a contribution layer over a shared one. Rejected: this "duplicates the agnostic 70%" — the toolchain-agnostic activities (discovery, spec, design, planning, closeout) would have to be re-authored per toolchain for no benefit.

### Option — toolchain facts per repo
Compute `toolchains`/`change_kinds`/`binary_change` once per repository rather than per item. Rejected as "wrong for mixed repos" — a story touching both Node and UE content would be classified under only one toolchain, silently losing the other's change summary.

### Option — tech packs editing rubric rows
Allow a technology pack to edit a process pack's `require` rows directly. Rejected: this "violates D14," which reserves structural/rubric ownership to process packs.

## Consequences

**Positive**
- The ~70% toolchain-agnostic slice of the catalog ships once and is reused across every tech pack, rather than being duplicated per toolchain.
- Mixed-toolchain items get correct per-item classification and both toolchains' artifacts, rather than one toolchain silently winning.
- The specificity tie-break (borrowed unchanged from §4.2.1's card-selection rule) gives producer contribution the same predictable, error-on-ambiguity behaviour as existing card selection, rather than a second bespoke mechanism.
- Per-activity timeout/cost overrides let a tech pack reflect real measured cost differences (F21) without forcing every toolchain onto one budget.

**Negative**
- Every technology pack must implement `Toolchain::classify` correctly per item; a wrong per-path classification silently misclassifies which producers apply, and this is now the load-bearing mechanism for mixed-toolchain items.
- Equal-specificity ties between two tech-pack producers are a validation error, not a runtime fallback — a pack author must catch this at pack-load time rather than relying on any implicit resolution.

**What this forbids**
- It forbids a technology pack from adding stages.
- It forbids a technology pack from removing baseline artifacts.
- It forbids a technology pack from editing `require` rows — any structural or rubric variation that looks like this must instead be authored as a process-pack variant (e.g. `story-ue`), per D14.
- It forbids computing toolchain facts per repo rather than per item.

**Partial withdrawal recorded here (forward reference):** activity-graph-schema amendment 43 (ADR 0073, outside this cohort's number range) withdraws clause (ii)'s "specificity beats generic" tie-break for toolchain-keyed artifact kinds (`keyed_by = "toolchain"` in `artifacts.toml`: `ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`). Under the withdrawn rule, a required keyed kind for a mixed-toolchain item silently received only the more specific toolchain's producer output (F35 §1.3: "a mixed Node+UE item received the UE change summary only, silently"). The amendment instead expands a required keyed kind to one instance per element of the item's `toolchain` set, with one producer per instance, and a generic producer's `'text' in change` becomes an instance-level predicate rather than a global tie-break loser. This ADR (0029) stays "accepted" — the withdrawal is scoped to the tie-break clause for toolchain-keyed kinds only; clauses (i), (iii) and (iv), and the specificity rule as applied to non-keyed artifact kinds, are unaffected.

## Evidence and confidence

- [documented] Proposal D14 / §4.2.1 selector specificity is cited as the mechanism PC7 extends; no new measurement is attached to the extension itself.
- [documented] The S-BIN spike is cited in the row's grounding column for binary-change handling, but it is **unrun**: the ledger's status note records S-BIN (binary artifacts) as "registered and unrun", and D8 treats the real UE project as the S-BIN fixture against which `ChangeSummary` is designed. So clause (i)'s `binary_change` fact rests on a design fixture, not on a spike result; nothing here is licensed as a measurement of binary-change classification.
- [measured] F21 gives n as specific durations: 2055 s for Node reviews, 7200 s as the UE budget — cited to justify per-activity timeout/cost overrides as a real, not hypothetical, need.
- [documented] Cloud Native Buildpacks' detect-and-contribute pattern and Tekton Pipelines vs. Tasks are cited as prior art for the pack/catalog split; neither is a measurement on this system.
- [measured] The mixed-toolchain silent-loss failure mode under the old tie-break rule is confirmed by F35 §1.3 ("a mixed Node+UE item received the UE change summary only, silently") — this is the evidence behind amendment 43's partial withdrawal noted above, not evidence for PC7's original rule, which was written before that failure was observed.
