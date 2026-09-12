---
id: "0114"
title: The executed-once gate moves to extended→shipped promotion; the v0 pack tags on P6 lint, the F42 parse and a per-activity executed-once ledger the doctor reports
status: accepted
date: 2026-09-11
legacy_id: none
source: references/adr/0027 Decision (the two gating conditions); references/2026-09-08-architecture-design.md §4.6 :613 and §10.5 P6-PACKS (audit PIA-088); spikes/F35-DRAFT.md §1.6 (6 of 102 dead, run9); spikes/F42-DRAFT.md; spikes/DECISIONS-2026-09-09.md open work item (1); Ryan's decision of 2026-09-11 (AskUserQuestion)
supersedes: ["0027"]
superseded_by: []
related: ["0072", "0081", "0103", "0093"]
evidence: ["F35", "F42", "F37", "PIA-088"]
consumers: []
---

## Context and problem

ADR-0027 ships a curated ~95-activity v0 catalog with ~170 rows held in an unshipped `extended` file, and gates the
pack tag on two conditions: "every shipped activity must pass P6 lint (reachable from some context) **and be executed
at least once** before the pack is tagged." The first condition is met and measured — the lattice at run9 reads 0.3 %
unsatisfiable at pack-default with 6 of 102 activities dead [measured, F35 §1.6], and every live predicate but two
parses to one tree under ADR-0103's grammar [measured, F42]. The second cannot be met by the programme that wrote
it: **101 of 102 activities have never been executed** (P7 ran `root-cause` and its members; nothing else has been
dispatched), so the pack is not taggable as shipped, DECISIONS-2026-09-09 work item (1) "tag the pack" has been
open since, and design §4.6 reports every 12-pack figure against an untagged pack (audit PIA-088).

The gate was written against EPF's rot-from-unexecuted-content failure, and it is the right concern aimed at the
wrong moment. Executing 101 activities once each is Phase-1 and Phase-2 work — the kernel that dispatches them
does not exist (runbook Step 7b) — so a literal reading ships the pack untagged through the whole of Phase 1, which
is exactly the state the gate was meant to prevent: content authored, never run, with nothing marking it so. The
proposal's own consequence line saw this — "spike P6 becomes a release gate for the pack" — and P6 has run.

This record supersedes ADR-0027 **in part**: the second gating condition and the forbids line that carries it. The
curated ~95 / unshipped `extended` split, the P6-lint condition, the curation argument and AC-110/AC-114 stand;
ADR-0027 stays `accepted`.

## Decision drivers

- A gate that cannot be met before the system exists is not a gate; it is a permanent exemption with a rule's name.
- Rot is a property of *time since last execution*, which a ledger can carry and a doctor can report; a one-time
  tag cannot.
- Promotion from `extended` to shipped is the moment content enters the surface that rots, and it is the one place
  the executed-once condition can be checked against a run that has actually happened.
- The pack count and the reading are already coupled by SM-027; tagging on the measured conditions closes the
  provenance gap PIA-088 names without changing any figure.

## Decision

**1. The v0 pack tags on the measured conditions.** A shipped pack is taggable when (a) every shipped activity passes
P6 lint — reachable from some context at pack-default, the dead list reported by id (AC-102) — and (b) every
predicate in the pack and its activities parses to one tree under ADR-0103's grammar (the F42 instrument, run as
AC-069/AC-072's lint at Step 7b). The two F42 parse failures and three `on_false` defects are repaired at Step 7e
before the tag; nothing else stands between the v0 pack and its tag.

**2. Each shipped activity carries an executed-once ledger, and the doctor reports it.** The catalog record gains
`executed: { first: Option<{ run_id, item, at }>, last: Option<{ run_id, item, at }> }`, written by the kernel on the
first and every subsequent `activity_committed` for that activity (ADR-0022's activation record). At v0 every
activity but `root-cause`'s P7 members reads `executed: never`. `redkiln doctor` prints the never-executed count and
list per pack, and `redkiln lint` warns — never fails — on an activity whose `last.at` is older than the pack's
declared `rot_after` (default 180 days; a pack-level setting, not a kernel constant). Rot is visible, dated and
per-activity instead of pretended away by a tag.

**3. The executed-once condition moves to promotion.** An activity is promoted from the `extended` file to the
shipped catalog only with a ledger entry: it must have run at least once — under a pack that references it by the
explicit extended-promotion route, in a run whose id the promotion decision (AC-114's `curation.decision`) cites.
AC-110 is unchanged: an enabled pack referencing an unpromoted extended activity is still a load-time configuration
error; the promotion route is a one-run, one-activity trial the operator declares, not a pack reference.

## Considered options

### Option — keep the gate literal
Rejected: the pack ships untagged through Phase 1 with 101 never-run activities and no per-activity mark; the rot
the gate exists for is neither prevented nor visible.

### Option — drop the executed-once condition entirely
Rejected: EPF's failure mode is real and the ledger is cheap; the condition is right, only its moment was wrong.

### Option — tag a Phase-1 slice of ~40 and grow (ADR-0027's rejected phasing option)
Rejected again: the lattice's completeness property (every stage-exit artifact has a producer at every reachable
context) is what P6 measured, and it holds over the ~95, not a slice.

## Consequences

**Positive**
- The v0 pack is taggable now, on two measured conditions, and every 12-pack figure in design §4.6 gains a tagged
  pack behind it.
- Rot becomes a dated, per-activity, doctor-reported fact; the first execution of each activity is on the record.

**Negative**
- One more field per activity and one more doctor row; `rot_after` is a pack setting nobody has sized (default
  stated, unmeasured).
- The extended-promotion route is a small new CLI/config surface (Phase 2, with `rk-plugin-gen`).

**What this forbids**
- Forbids tagging a pack with a shipped activity that fails P6 lint or the F42 parse.
- Forbids promoting an `extended` activity without a ledger entry naming the run.
- Forbids `lint` failing on `executed: never` — the mark is a report, not a block, until rot is measured.
- Forbids describing the v0 pack as "executed" — the tag asserts reachability and parseability, and the ledger says
  what has run.

## Evidence and confidence

- [measured, F35 §1.6, run9] 0.3 % unsatisfiable at pack-default over the 8 pre-registered packs (432/155,520);
  6 of 102 dead; the 12-pack reading 0.2 %.
- [measured, F42] 285/287 predicates parse to one tree; 0 precedence-dependent; the two failures and three
  `on_false` defects are pack defects, one in a shipped pack.
- [measured, F37] P7 executed `root-cause` and its members — the only executions in the catalog's history.
- [documented, ADR-0027] EPF's rot-from-unexecuted-content failure mode; no rate is given for it anywhere in the
  corpus, which is why `rot_after` is a default and not a threshold.
- **What this evidence does not license.** It does not license any claim that the ~95 are the *right* ~95 (AC-114's
  human gate), nor a value for `rot_after`; and it does not change the pack count any reading is taken at (SM-027).
