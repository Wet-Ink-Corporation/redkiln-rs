---
id: "0166"
title: "The v0 pack's tag condition was met over the **shipped** pack: `redkiln-pack-v0` stands, and `catalog-b`'s four remaining F42 defects, which were never in a shipped pack, are repaired after the tag. Supersedes ADR-0114 decision 1's sentence that all five F42 defects are repaired before the tag, in part, and folds in Checkpoint 7b's re-pin to the probe's post-repair reading, 279/279."
status: accepted
date: 2026-09-24
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (i) (Checkpoint 7b re-pinned with a record, ADR-0114 d1's residual named as owed) and D21-42 decision 23 (supersede d1's second sentence in part over the shipped pack, folded with the 7b re-pin, plus a dated F42-DRAFT correction, and repair catalog-b's four defects); D21-40's table (Checkpoint 7b red); runbook/phase-1/s8a-records-and-red-checkpoints.md §8a.4; references/adr/0114 decision 1; spikes/F42-DRAFT.md (the five defects, by file); `python spikes/p9/grammar-probe.py` before and after the repair [measured 2026-09-24]
supersedes: ["0114"]
superseded_by: []
related: ["0027", "0048", "0072", "0092", "0103"]
evidence: ["F42"]
consumers: []
---

## Context and problem

ADR-0114 decision 1 sets the v0 pack's tag on two measured conditions: (a) every shipped activity passes P6 lint,
and (b) *"every predicate in the pack and its activities parses to one tree under ADR-0103's grammar (the F42
instrument, run as AC-069/AC-072's lint at Step 7b)"*. It then says: *"The two F42 parse failures and three
`on_false` defects are repaired at Step 7e before the tag; nothing else stands between the v0 pack and its tag."*

F42 found five defects. Only one of them is in a shipped pack: `packs/incident.toml`'s `exists(ProblemRecord)`.
The other four are in `spikes/p9/catalog-b/`, which F42's own table marks *"no — catalog-b is the unshipped
extended file (ADR-0027)"*:

- `blameless-postmortem.toml`'s `applicable_if` carries the bare hyphenated value `security-vuln`;
- `blameless-postmortem.toml` steps `complete` and `reviewed`, and `api-contract-spec.toml` step `policy`, carry a
  bare step name as `on_false`.

Step 7e repaired `packs/incident.toml` and tagged `redkiln-pack-v0` at `dddcdbd`. It did not touch `catalog-b`. So
ADR-0114's sentence is false as written, while the condition it serves, (b) over the shipped pack, was met.

Step 7e's `spikes/p9` edits also dropped eight `when` predicates, so the probe no longer prints F42's 287. Measured
2026-09-24 over a `git archive` of `spikes/p9` at `b8e7aa9`: `predicates parsed 278/279; failed 1/279` and
`on_false forms 96/99 in the closed set`. Checkpoint 7b pins `predicates parsed 285/287; failed 2/287`, and is red.

## Decision drivers

- **The sentence is in `## Decision`**, so ADR-0092 rule 1 makes the correction a supersession in part, not an
  erratum.
- **The tag must not be read as unearned.** Decision 1's condition (b) is about the shipped pack, and the shipped
  pack was clean at the tag.
- **The four defects are real defects, and D21-42 decision 23 repairs them.** Leaving them in place keeps a
  known-red probe and a checkpoint that cannot pass.
- **A pin must say what the instrument prints after the repair.** D21-41 (i) said *"re-pinned to 278/279"*; D21-42
  decision 23, taken later, adds the `catalog-b` repair, which makes the reading 279/279.

## Considered options

### Option — supersede the sentence in part over the shipped pack, and repair `catalog-b` after the tag (taken)

State what the tag's condition was over the shipped pack, keep the tag, repair the four defects now and re-pin
Checkpoint 7b to the clean reading. Taken: it is D21-42 decision 23.

### Option — re-pin Checkpoint 7b to 278/279 and name the residual as owed (D21-41 (i))

Rejected by D21-42 decision 23, which was taken later. It would pin a known defect as the expected reading.

### Option — record the gap as an erratum

Rejected. The sentence is a `## Decision` sentence, and ADR-0092 rule 1 routes a correction that moves one by
supersession.

### Option — move or re-cut the tag after the repair

Rejected. The tag's condition was met over the shipped pack, and `catalog-b` is not in any shipped pack. Moving a
published tag would make the tag, not the sentence, the thing that is wrong.

## Decision

**1. What this record supersedes, in part.** ADR-0114 decision 1's sentence *"The two F42 parse failures and three
`on_false` defects are repaired at Step 7e before the tag; nothing else stands between the v0 pack and its tag."*

**2. What the tag's condition was.** Decision 1's condition (b) is read over the **shipped** pack. It was met by the
one in-scope repair, `packs/incident.toml`'s, at Step 7e, and **`redkiln-pack-v0` stands**.

**3. The other four defects are repaired here, after the tag.** They were never in a shipped pack:

| file | from | to |
| --- | --- | --- |
| `spikes/p9/catalog-b/blameless-postmortem.toml`, `applicable_if` | `kind in [incident, security-vuln]` | `kind in [incident, 'security-vuln']` (ADR-0048's quoting rule) |
| same, step `complete` | `on_false = "draft"` plus a step-level `max = 2` | `on_false = { to = "draft", max = 2 }` |
| same, step `reviewed` | `on_false = "draft"` plus a step-level `max = 1` | `on_false = { to = "draft", max = 1 }` |
| `spikes/p9/catalog-b/api-contract-spec.toml`, step `policy` | `on_false = "assemble"` plus a step-level `max = 2` | `on_false = { to = "assemble", max = 2 }` |

Each `max` is the step's own sibling value, moved into the table; the stray step-level line is removed.
`catalog-b` is unshipped, so ADR-0072's rule for a shipped activity edit (version bump, changelog row, P6 re-run)
does not reach it. The probe's regenerated `spikes/p9/results-s-grammar.json` is committed with the repair.

**4. Checkpoint 7b's re-pin is folded into this record, and it is a re-pin, not a widening.** It asserts
`predicates parsed 279/279; failed 0/279`, `mixed unparenthesised and/or (precedence-dependent) 0/279` and
`on_false forms 99/99 in the closed set`, and the probe's rc is tightened from `-le 1` to `-eq 0`, because a clean
probe exits 0.

**5. What stands.** Decision 1's conditions (a) and (b), decision 2, decision 3 and every forbids bullet of
ADR-0114 stand. ADR-0114 stays `accepted`.

## Consequences

**Positive**

- The F42 instrument reads clean over every file it walks, and Checkpoint 7b can pass on the tree.
- The tag's warrant is stated over the pack it tags.

**Negative**

- `spikes/F42-DRAFT.md`'s **285/287** is now a historical reading. It takes a dated correction beside it, under
  ADR-0092 rule 2.4, and the tests that pin F42's historical measurement keep pinning it.

**What this forbids**

- Forbids reading ADR-0114 decision 1's condition (b) over an unshipped file.
- Forbids moving, deleting or re-cutting `redkiln-pack-v0` on the ground that `catalog-b` was unrepaired at the
  tag.
- Forbids pinning Checkpoint 7b to a reading taken before the `catalog-b` repair, or at an rc other than 0.
- Forbids a bare step name as an `on_false` value, and a bare hyphenated facet value, anywhere under
  `spikes/p9/catalog-b/`.

## Evidence and confidence

- [measured 2026-09-24] `python spikes/p9/grammar-probe.py` over a `git archive` of `spikes/p9` at `b8e7aa9`:
  `predicates parsed 278/279; failed 1/279`, `mixed unparenthesised and/or (precedence-dependent) 0/279`,
  `on_false forms 96/99 in the closed set: {'keyword': 33, 'loop-back': 66}`.
- [measured 2026-09-24] the same probe over a scratch copy carrying the four repairs of decision 3:
  `predicates parsed 279/279; failed 0/279`, `mixed unparenthesised and/or (precedence-dependent) 0/279`,
  `on_false forms 99/99 in the closed set: {'loop-back': 69, 'keyword': 30}`, rc 0.
- [documented] F42-DRAFT's defect table, and its *"no — catalog-b is the unshipped extended file (ADR-0027)"*
  column; ADR-0114 decision 1, quoted above.

Confidence is high. Every number here is the probe's own output, before and after.
