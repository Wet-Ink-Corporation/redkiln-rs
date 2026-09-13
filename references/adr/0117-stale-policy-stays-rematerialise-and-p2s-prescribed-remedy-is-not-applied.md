---
id: "0117"
title: stale_policy stays rematerialise at coarse scope for every cost class, and METHOD-P §2.4 criterion 1's prescribed remedy is NOT applied because the same run contraindicates it
status: accepted
date: 2026-09-13
legacy_id: none
source: spikes/METHOD-P.md §2.2–§2.4 and amendment 12 (P2-B, registered before any draw); spikes/F40-DRAFT.md §1–§4; spikes/DECISIONS-2026-09-11.md #4 (the value) and #16/#19 (the abstention rule and the card as a perturbation candidate); references/2026-09-08-acceptance-criteria.md §1.1 row 2, AC-068, AC-067; references/2026-09-08-success-measures.md SM-021; runbook/00-preamble.md golden constraint 11; runbook/halts.md Halt 12, Halt 13
supersedes: []
superseded_by: []
related: ["0012", "0027", "0067", "0090", "0097", "0105", "0114"]
evidence: ["F40", "PIA-044", "PIA-084"]
consumers: []
---

## Context and problem

P2 measured the `derived_from` reconstruction — *"which upstream changed before this artifact was last
written"* — and METHOD-P §2.4 pre-registered three kill criteria over it, each prescribing a remedy for
`stale_policy` and the `L`-class scope. The perturbation half was scored on 2026-09-10 at corpus pin
`47faec9f`, $0 model cost, labels blind and timestamp-verified [measured, F40].

**Criterion 1 fired, and its own remedy is contraindicated by the same measurement.** That is the whole
of the problem, and it is a shape this programme has a named constraint for.

- Criterion 1 is *"coarse recall < 0.9 **or** precision < 0.7 → fine scope becomes mandatory for
  `L`-class activities and `stale_policy = gate` is the default for them"*. Its **precision half fires**:
  coarse `derived_from` precision is **0.00** [0.00, 0.26] over 11 emitted positives. Its **recall half
  is NOT ANSWERABLE** — **0 of 41** rows are labelled "should re-run", so recall has no denominator.
- The prescribed remedy is *"fine scope mandatory"*, and **the same run measures fine as the worse of
  the two**: specificity coarse **0.73** [0.58, 0.84] against fine **0.55** [0.38, 0.70], and on the
  **33 rows both scopes scored**, coarse marks **5** `Stale` and fine marks **15**, with **10 rows
  flipping `Present` → `Stale` going coarse to fine and none flipping back** [measured, F40 §3].
- Criterion 2 is *"fine scope cannot reach recall ≥ 0.9 either → `stale_policy = gate` for all
  `L`/`XL`"*. It is **NOT ANSWERABLE** on the same missing denominator and **cannot fire either way**.
- Criterion 3 is *"advisory useful fraction < 0.3 → PC3's advisory is demoted to a log line"*. It is
  **UNRUN**: the advisory half (A01–A06) is labelled after the engine in its own file, and
  DECISIONS-2026-09-09 #15's `human_authored` remap must be settled first.

**Why precision is 0.00, stated so it is not read as a reading of the engine.** With no positive labels
every `Stale` the engine emits is a false alarm **by construction**. `precision = 0.00` is a property of
the label set, not of the reconstruction. What this corpus genuinely measures is **specificity** — how
often the engine leaves a correctly-unchanged artifact alone. The cause is mechanical and recorded:
DECISIONS-2026-09-09 #19 added the **card** as a perturbation candidate, and **11 of the 12 cases change
nothing but the `stage`/`status`/`updated`/`gate_open` frontmatter the CLI rewrites on every advance**;
the twelfth (P01) is the only real body perturbation in twenty items [measured, F40 §1]. The labeller
wrote *"only frontmatter properties … no new/updated facts of the system"* against 36 of the 41 rows
before any of this was computed. **The zero-positive label set is the corpus, not the labeller.**

**And the mechanism behind the fine scope's failure is one this programme has met before.** Coarse's
declared map carries `spec.md ← <card>`, so a card write invalidates the spec and nothing else — 11
false alarms from 11 emissions. Fine *also* fires on `implementation-report.md` and `report.md`, which
**the declared map does not connect to the card at all**: the reconstruction picks the card up as a
derivation edge because the card is rewritten so often that it is nearly always the most recent prior
write. **That is F39's failure class at the artifact level — an inference over a trace the instrument
itself writes into** (Halt 10), and it is why "refine the scope" imports noise rather than removing it.

## Decision drivers

1. **Golden constraint 11 and Halt 12 are explicit**: a criterion that fires and prescribes a remedy its
   own measurement contraindicates must have **both halves recorded**, the remedy must **not** be
   applied, and the design consequence **needs a decision, not a default**. This record is that decision.
2. **The value is already decided on an independent warrant.** DECISIONS-2026-09-11 #4 set
   `stale_policy = rematerialise`, coarse scope, every cost class, recorded per class with its warrant in
   the pack. That decision does not rest on P2, and Step 8's checkpoint asserts no value was defaulted
   from P2. What was missing was a record of the *relationship* between P2's result and that value.
3. **A contraindicated remedy applied anyway is worse than no remedy.** Fine scope is measurably the
   worse instrument on this corpus, on a paired comparison over the rows both scopes scored. Applying
   the remedy would degrade the behaviour in the name of a criterion that fired on a figure the label
   set determined.
4. **The instrument that could settle it is already registered, before any draw.** METHOD-P amendment 12
   registers **P2-B** with the same §2.4 criteria on a label set that has a positive class. Deciding now
   does not foreclose it.

## Considered options

**A — record the firing, record the contraindication, do not apply the remedy, keep the value.** Closes
golden constraint 11's requirement with a decision rather than a default, and leaves P2-B free to change
it on evidence.

**B — apply the prescribed remedy: fine scope mandatory for `L`, `stale_policy = gate` for those.**
Faithful to the letter of the pre-registration. Rejected on the pre-registration's own measurement: fine
is the worse scope here by a paired comparison, and criterion 1 fired on a precision figure that is a
property of a label set with no positives. This is the option Halt 12 exists to stop.

**C — treat criterion 1 as not fired, on the ground that its precision half is uninterpretable.**
Tidier, and defensible in substance. Rejected because it edits the reading of a pre-registered criterion
after the numbers are in, which is the defect the whole §4.1 discipline exists to prevent. The criterion
**fired**; what is wrong is its remedy, and both facts are recorded rather than one being dissolved.

**D — commission P2-B first and decide afterwards.** The evidentially strongest route, and it remains
available. Rejected as the *Phase-1* route: P2-B needs ≥ 35 blind-labelled "should re-run" rows over real
body perturbations and therefore a second corpus or a later pin, and nothing in Phase 1 waits on it. A
decision is owed now either way, because `stale_policy` ships in the pack at Step 7e.

**E — leave the consequence OPEN with no record.** Rejected: it is the half-absorbed finding golden
constraint 11 names. A document that records the criterion as firing without recording that its own
remedy is contraindicated has absorbed half the finding, and one that records neither is worse.

## Decision

**Option A.** Four clauses.

**1. `stale_policy` stays `rematerialise`, at coarse scope, for every cost class**, as
DECISIONS-2026-09-11 #4 set it, recorded per class with its warrant in the pack at Step 7e. This record
does not change the value; it records that the value survives P2's result and why.

**2. METHOD-P §2.4 criterion 1's prescribed remedy is NOT applied.** Neither *"fine scope becomes
mandatory for `L`-class activities"* nor *"`stale_policy = gate` is the default for them"* takes effect
on this evidence. The reason is recorded with the firing, not in place of it: the same run measures fine
as the worse scope, on the paired comparison over the 33 rows both scopes scored.

**3. Criterion 1 fired on its precision half and its recall half is NOT ANSWERABLE; criterion 2 is NOT
ANSWERABLE and cannot fire either way; criterion 3 is UNRUN.** These are reported by those names and
never as a pass, a failure, `0.00`, `1.00` or `n/a`. The quantity this corpus measures is
**specificity**, and specificity is what is led with.

**4. P2-B is the instrument that may change clauses 1 and 2**, and only a run with a positive class can.
It is registered in METHOD-P amendment 12 before any draw, its consumer is AC-068 / SM-021, and its
construction requirement is real body perturbations classified mechanically **before** labelling, with
labels timestamp-verified and never attested. Until it runs, this record holds.

## Consequences

Step 7e authors `stale_policy = rematerialise` per class with a warrant that now has a record to cite,
and Step 8's checkpoint assertion that no value was defaulted from P2 becomes true in a stronger sense:
the value was decided independently *and* the P2 result that might have overridden it was adjudicated.

`PROVISIONAL(P2)` on AC-068 keeps its meaning as the register states it — not *unspecified*, but *the
criteria that would settle it have no denominator on the registered corpus, and the one that fires
prescribes a contraindicated remedy*. This record is what that sentence now points at.

The `derived_from` reconstruction itself is untouched. Nothing here says the coarse map is good; it says
the fine map is measurably worse on this corpus and that neither has been evaluated for detection at
all, because the corpus carries no positives to detect.

**What this forbids**

- **The prescribed remedy may not be applied on this evidence** — not fine scope for `L`-class, and not
  `stale_policy = gate` for `L`/`XL` — and it may not be reintroduced as a default, a convention or a
  "safer" fallback. A successor to this record is the only route.
- **`precision = 0.00` may not be cited as a reading of the reconstruction.** It is a property of a
  label set with **0 of 41** positives. Any document quoting it carries that fact in the same breath,
  and leads with **specificity** (coarse 0.73 [0.58, 0.84], fine 0.55 [0.38, 0.70]).
- **Criterion 2 may not be reported as passed or failed**, and criterion 1's recall half may not be
  reported as `0.00` or `1.00`. Both are **NOT ANSWERABLE**; `data_migration`'s **NOT MEASURABLE** in
  F39 is the same *shape* and **their numbers may never be mixed**.
- **Criterion 3 may not be reported as run**, and the advisory half may not be labelled until
  DECISIONS-2026-09-09 #15's `human_authored` remap is settled in writing.
- **A fine-scope abstention may not be counted as a false negative.** A reconstructed scope of
  `unknown` is an abstention with its own rate (8/41 = 19.5% [0.10, 0.34]) and is never summed into
  misses (DECISIONS-2026-09-09 #16).
- **P2's result may not be read as touching the selection path.** P2 gates `stale_policy`; ADR-0090
  decides the context vector. Neither discharges nor tightens the other.
- **A re-score of the frozen corpus may not be offered as settling this.** Only a corpus with a
  positive class can, and P2-B names what that requires.

## Evidence and confidence

**The firing, and the contraindication, from one run.** Coarse `derived_from` precision **0.00**
[0.00, 0.26] over 11 emitted positives; recall **NOT COMPUTABLE** (`tp + fn = 0`). Specificity coarse
**0.73** [0.58, 0.84] over 41 of 41 rows scored; fine **0.55** [0.38, 0.70] over 33 of 41, with 8
abstentions (19.5% [0.10, 0.34]). The two specificity intervals **overlap**, so on the aggregate the
difference is **not separated at this n** — which is why the paired comparison is the evidence: on the
**33 rows both scopes scored**, coarse marks 5 `Stale`, fine marks 15, the two agree on 23 of 33, and
**all 10 disagreements are `Present` → `Stale` going coarse to fine, none the other way**, named
individually as P02/P06/P07/P09/P11 × {`implementation-report.md`, `report.md`}. [measured, F40 §2–§3;
re-run 2026-09-12 and 2026-09-13, output byte-identical]

**Why the label set has no positives, mechanically and checkably.** 11 of the 12 perturbation cases
change nothing but YAML frontmatter the CLI rewrites on every stage advance; the 11 frontmatter-only
cases **are** the 11 card (`story.md`) perturbations that DECISIONS-2026-09-09 #19 added. The twelfth,
P01 on `discover.md`, changes one body line. [measured, F40 §1]

**The blindness claim this rests on is machine-checkable and held.** `labelled_at`
**2026-09-10T15:58:08Z** precedes the engine's earliest record at **16:01:12Z**, and `score-p2.py`
compares every record's `labels_at` against the label file's own stamp, so a re-stamped label file
cannot be quietly scored against an older engine run. Two of the five documented refusals did not fire
on this run — **a guard that does not fire is evidence only that its precondition held**, which is the
blindness claim itself and says nothing about the verdicts. [measured, F40]

**What would settle it, sized.** For recall ≥ 0.9 to be *demonstrated* (lower bound ≥ 0.90) the label
set needs **≥ 35 blind-labelled "should re-run" rows with every one recalled** (35/35 → 0.901); **53** if
one is missed, **69** if two. Fewer than 35 positives and the criterion is registered as **powerless
before the run and is not run**. The frozen corpus at `47faec9f` supplied **one** such case in twenty
items, so P2-B needs a second corpus or a later pin of the same project. Model cost **$0**; the cost is
case construction and labelling time. [measured, METHOD-P amendment 12, recomputed 2026-09-11]

**Confidence: high on the decision, on a corpus that could not decide the criteria.** That pairing is
the honest one. What is established is that fine scope is worse *here* and that the label set carries no
positives; both are measured and neither is in doubt. What is **not** established is whether either
scope detects real staleness at all — no measurement in this programme has tested that, because the
registered corpus contains almost nothing to detect. This record decides what ships in Phase 1. It does
not claim the reconstruction is good, and P2-B is the only thing that would.
