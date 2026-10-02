---
id: "0195"
title: "**`tag-release` is a CLI's release, and it enters by ADR-0114's trial** (`s8g`'s tag record, R5; D21-42 6b; D21-46 R4): `ReleaseRecord` is an annotated local tag `rk-release/<item>` on the landed commit, the `BuildArtifact` digest and the rollback target (the nearest earlier `rk-release/*` tag, or explicitly `none`), made by the `release_tag` git-route handler in the composition root through `rk-git` and never pushed; `tag-release` is authored in `.redkiln/catalog-extended/` and admitted by a `[trial] activity = \"tag-release\"` declaration on each lane item, for that item's one run, the promotion following the trials as AC-114's human gate; selection reaches it through a new `declared` facet, `release_form`, keyed in its `applicable_if`, first in the `deploy-pattern` family, with no shipped deploy activity edited; both packs' release goals are amended to the tag they can reach; AC-114's `curation.decision` lint is `s8j`'s. Supersedes nothing"
status: accepted
date: 2026-09-29
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (row 6b — *"**A new `tag-release` activity.** ReleaseRecord is a release tag on the landed commit, plus the BuildArtifact digest and the rollback target (the previous release tag). The tag is local and never pushed by the run. It enters via ADR-0114 c3's one-run, one-activity trial."*), D21-46 (R4 — *"**Two trials, confirmed.** Each is one run of one activity, which is the clause's constraint."*), D21-47 (review finding 6); references/adr/0114 (decision 3 and its Negative consequence), 0089 (the `declared` facet precedent); runbook/phase-1/s8g-verify-and-restored-demands.md §8g.7, §8g.8 (R5's row)
supersedes: []
superseded_by: []
related: ["0072", "0089", "0105", "0114", "0170", "0179", "0188", "0193", "0194"]
evidence: []
consumers: []
---

## Context and problem

**A CLI has no production to deploy to, and the release stage demands a `ReleaseRecord`.** Its four shipped producers
(`rolling-deploy`, `blue-green-deploy`, `canary-release`, `flag-progressive-rollout`) all run `toolchain:deploy`, and
no existing facet distinguishes a repository whose release is a tag. D21-42 6b decides the shape: *"**A new
`tag-release` activity.** ReleaseRecord is a release tag on the landed commit, plus the BuildArtifact digest and the
rollback target (the previous release tag). The tag is local and never pushed by the run. It enters via ADR-0114 c3's
one-run, one-activity trial."*

**ADR-0114 decision 3, read whole**: *"An activity is promoted from the `extended` file to the shipped catalog only with
a ledger entry: it must have run at least once — under a pack that references it by the explicit extended-promotion
route, in a run whose id the promotion decision (AC-114's `curation.decision`) cites. AC-110 is unchanged: an enabled
pack referencing an unpromoted extended activity is still a load-time configuration error; the promotion route is a
one-run, one-activity trial the operator declares, not a pack reference."* Its Negative consequence places that route
in Phase 2: *"The extended-promotion route is a small new CLI/config surface (Phase 2, with `rk-plugin-gen`)."*

**D21-46 R4 confirms two trials**, one per lane item: *"**Two trials, confirmed.** Each is one run of one activity,
which is the clause's constraint."* The draft's dedicated trial item is withdrawn (D21-47 review, finding 6).

**The packs' release goals promise production.** `bug-standard.toml` reads *"The fix is in production reversibly"* and
`story.toml` *"In production, reversibly"*. A run that makes a local tag and pushes nothing puts nothing in
production.

## Decision drivers

- AC-110 unchanged: the extended directory contributes zero entries to the resolved catalog by default.
- The trial is data on an item record, not a CLI flag; the AC-400 help snapshot does not move.
- Agent-free, so the trial cannot dispatch and needs no spend gate.
- The narrowest selection change that serves the lane: no shipped deploy activity edited, no stage's selection moved.
- A pack sentence the run cannot make true is amended, not read around.

## Considered options

### Option — a `declared` facet keyed in `tag-release`'s `applicable_if`, first in `deploy-pattern` (taken)

The `surfaces` precedent (ADR-0089: a value read from a checked-in declaration map). `applicable_if` is not a
`require` row, so a declared facet may key it. Taken: it edits no shipped deploy activity and moves no other
activity's selection.

### Option — verb availability as a selection-time admissibility condition

The long-term shape: an activity whose verb the toolchain cannot serve is inadmissible. Rejected **here**: it moves
every stage's selection on the lane `s8` runs, which is a wider change than a release producer warrants.

### Option — a dedicated trial item, and the promotion before any lane run selects `tag-release`

Rejected (D21-47 review, finding 6; D21-46 R4): the trials are the lane runs themselves, so no promotion can precede
them, and a dedicated item would be a third run of the activity.

## Decision

1. **`ReleaseRecord`** is composed by `tag-release`'s emit from the `ReleaseTag` value and the `BuildArtifact` digest:
   - **the tag**: annotated, named `rk-release/<item>`, on the landed commit (ADR-0188: a kernel-made content
     checkpoint on `rk-run/<item>`), made through `GitOp::Tag` (R3, ADR-0193); re-running for the same item at the
     same commit is idempotent, and at a different commit it refuses by name; the target must be on the run-owned
     branch;
   - **the rollback target**: the nearest `rk-release/*` tag on an ancestor of the landed commit, read through the
     existing `GitOp::Log` (`--decorate-refs=refs/tags/rk-release/ --simplify-by-decoration`), so no new read op is
     owed; **a first release names its target explicitly as `none`**, never absent and never the landed sha;
   - **never pushed**: `GitOp` has no push variant.

2. **The `release_tag` handler.** `s8c`'s registry row `release_tag` (route `git`, output `ReleaseTag`, *"Refused
   naming the route until `s8g`"*) gets its `git`-route handler in the composition root, through `rk-git`, never in a
   toolchain adapter shelling git. The spike registry's `release_record` row is not the shipped row.

3. **The activity** is agent-free: `toolchain:release_tag` (inputs `artifact:BuildArtifact`,
   `artifact:Changeset.sha`), a check that the record names a tag and a rollback target, and an emit. It is authored
   in **`.redkiln/catalog-extended/tag-release.toml`**, not `.redkiln/catalog/`, with its row in
   `.redkiln/CHANGELOG.md`.

4. **The trial declaration is data on each lane item**: `[trial] activity = "tag-release"`. `run` admits exactly that
   one extended activity, at the release stage, for exactly the declaring item's one run; a run on an item without
   the declaration never admits it, and an enabled pack referencing it is refused at load (AC-110). `s8j`'s two lane
   items carry the declaration. This is the Phase-1 form of the route ADR-0114 placed in Phase 2, and its limits are
   these: one activity, one run per declaring item, no pack reference, no CLI flag.

5. **Selection reaches it through a declared facet** (option a). A new facet, **`release_form`**, `source =
   "declared"`, values `["deploy", "tag"]`, read from a checked-in per-repository declaration (`surfaces`'s form);
   `tag-release` declares `applicable_if = "release_form != null and release_form == 'tag'"`, `family =
   "deploy-pattern"`, and a `method_rank` below `rolling-deploy`'s 10, so ADR-0179's step 4 chooses it first where
   it is applicable and admitted. **No shipped deploy activity is edited.** The facet keys no `require` row. This
   repository declares `release_form = "tag"`.

6. **The release goals are amended**, not read around: `bug-standard.toml`'s release goal becomes `"The fix is tagged
   for release, reversibly"` and `story.toml`'s `"Tagged for release, reversibly"`. The packs carry no `version`
   key and ADR-0072 governs activity edits, so no version moves; the edit is listed in `.redkiln/CHANGELOG.md`
   (`s8b` §8b.5's file).

7. **The promotion follows the trials, and it is AC-114's human gate.** It cites the fix-lane run's id and moves
   `.redkiln/catalog` only after `s8`'s runs (`s8j` §8j.1(e)). Until it lands, a lane selects `tag-release` only on an
   item whose `[trial]` declaration names it. **At this stage the trial runs under the fixture harness only**, in
   temporary repositories; its real run on this repository is `s8j`'s, because it needs a landed commit and a
   journalled `BuildArtifact`, and it creates a tag in the operator's repository, which is an operator moment.

8. **AC-114's residue lint is `s8j`'s.** It reads *"the file carries a non-empty `curation.decision` id resolving to a
   record in the catalog's decision log"*; its first non-vacuous input is the promotion `s8j`'s real trial produces,
   and built here it would fail all 102 shipped activities or ship with an unrecorded grandfather rule. The backfill
   for the 102 is a decision owed upward.

*(dated 2026-09-29, `s8g` build, package S8G-P8-TAG-RELEASE-AND-MOVES; this record `proposed`.)* **Built, red first,
and the four things this record left to the build, fixed here** (the Evidence section's *"the build may refine by dated
note while `proposed`"*):
- **The declaration file (decision 5)** is **`.redkiln/declared.toml`**: one top-level `<facet> = "<value>"` per
  declared, single-valued facet, read through the facet registry (`rk_kernel::registry::Registry::declared_values`),
  which refuses a key that is not a `source = "declared"`, single-valued facet and a value outside its domain. `run`'s
  `build_item` carries the value; a facet the file does not name stays `null`. `surfaces` keeps ADR-0089's own map and
  is not read there. This repository's file declares `release_form = "tag"`.
- **"The BuildArtifact digest" (decision 1)** is carried as the record's `binaries` member, the `BuildArtifact`'s own
  `binaries` list (each released binary's `name`, `path` and `sha256`), composed by the emit from
  `artifact:BuildArtifact.binaries`. ADR-0040's grammar has no digest-of-an-artifact source, and `ReleaseTag` (record
  V's type, `deny_unknown_fields`) has no member for one, so the build's own per-binary digests are the digest carried.
  The `ReleaseRecord` reads `{ tag, target, rollback_target, binaries }`.
- **The activity (decision 3)**: `requires = ["BuildArtifact", "Changeset"]`, `method_rank = 5`, one `tool` step `tag`
  (`toolchain:release_tag`, inputs `build` and `sha`), one `check` step `named` (*"`release.tag != '' and
  release.rollback_target != ''`"*, `on_false = "fail"`), and the composing `emit`. The `git` route is served through a
  kernel seam the composition root binds (`runtime::step::GitRoute`, whose unbound default refuses by name, as the
  `store` route's does) by `rk-cli`'s `release_tag` handler, which reads the rollback target BEFORE the write (the landed
  commit itself excluded; of two release tags on the nearest commit, the first by name) and then calls
  `rk_git::release::tag_release`.
- **The trial admission (decision 4)**: the kernel's `catalog::admit_trial` returns the ONE extended source the item's
  `[trial] activity` names (refusing by name one that names no extended activity, or a shipped one), and `rk-cli`'s
  `catalog_sources` appends it to the shipped catalog, so the run's recorded input set carries exactly the catalog it
  selected over. An enabled pack whose `[activity.<id>]` names an extended activity is refused at `run`'s step (7) with
  exit 3 (AC-110). **"The declaring item's one run" is read as that item's lane**: the admission is bound to the
  declaring item, and a lane runs once, its `closed` mark ending it (`s8e` §8e.6); no separate one-run counter is
  built. **"At the release stage"** holds by the activity's own `stage`, which is the only stage selection reaches it
  at.
- Tests (`rk-cli`, `--test it`): the nine `tag_release::` tests `s8g` §8g.9 names, each red first; mutants M10 and M25
  killed (§8g.9's dated note).

## Consequences

**Positive**
- The release stage has an agent-free producer that a CLI can satisfy, and whose trial cannot dispatch.
- AC-110 holds: the extended directory contributes nothing unless an item declares the trial.

**Negative**
- A new facet with one consumer; verb admissibility, the long-term shape, stays owed.
- The Phase-1 trial route is item-record data, not ADR-0114's Phase-2 CLI/config surface, and must be reconciled with
  it when `rk-plugin-gen` lands.
- The packs no longer promise production; a deploying repository declares `release_form = "deploy"`.

**What this forbids**
- Pushing a release tag, or tagging a commit that is not on `rk-run/<item>`.
- Defaulting the rollback target to the landed sha, or omitting it on a first release.
- Placing `tag-release` in `.redkiln/catalog/` before AC-114's promotion cites a trial run.
- Admitting an extended activity on a run whose item does not declare it, or by a pack reference or CLI flag.
- Keying a `require` row on `release_form`, or editing a shipped deploy activity to make `tag-release` win.
- A release goal that promises production on a pack whose release stage can only tag.

## Evidence and confidence

**Measured** (2026-09-29, at `178868f`): `.redkiln/registry.toml`'s `[verb.release_tag]` reads `route = "git"`,
`outputs = "ReleaseTag"`; the four `deploy-pattern` activities' `applicable_if` rows read *"`blast_radius < med or
rigor < recommended or kind == incident`"*, *"`blast_radius >= high`"*, *"`blast_radius >= med`"* and
*"`flag_exists != null and flag_exists`"*; `rolling-deploy`'s `method_rank` is 10; the release goals read as quoted.
**Confidence:** high on the tag's shape and the trial route (the operator's); medium on the facet's name and its
declaration file, which this record names and the build may refine by dated note while `proposed`.
