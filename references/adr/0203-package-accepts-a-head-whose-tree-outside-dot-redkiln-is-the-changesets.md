---
id: "0203"
title: "**`package` accepts a `HEAD` whose tree outside `.redkiln/` is the `Changeset` commit's tree** (`s8j`'s package-sha record; the operator's ruling A on the S8J-C4 lane-proof defect D4, 2026-10-18): when the input `sha` is not the project root's `HEAD`, the Rust adapter reads `sha..HEAD` through `rk_git::diff::range_diff` and accepts only when every changed path lies under `.redkiln/`; any other changed path, or a `sha` that does not resolve, is still refused as `sha-head-mismatch`, naming the first such path. `BuildArtifact` still carries both `sha` and `head`. Supersedes ADR-0170 decision 6 in part, at its sentence *\"so is a `sha`/`HEAD` mismatch\"*"
status: proposed
date: 2026-10-18
legacy_id: none
source: the S8J-C4 lane proofs (`crates/rk-cli/tests/it/lane_proof.rs` on `claude/s8j-afk-batch-2-tg23uw-c4`, PR #103), whose lanes' release stage refused `build-package`'s `package` step as `sha-head-mismatch`; the operator's ruling A on that defect (D4), 2026-10-18; references/adr/0170 decision 6 and its forbids, 0077, 0098, 0165 decision 4; spikes/DECISIONS-2026-09-21.md D21-62 C; runbook/phase-1/s8a-records-and-red-checkpoints.md Checkpoint 8a guard (6) (the adapter's `rk_git` allowlist: `diff`, `path`, `head`); crates/rk-adapter-toolchain-rust/src/verbs.rs (`package`), crates/rk-git/src/diff.rs (`range_diff`), .redkiln/catalog/build-package.toml (`sha = "artifact:Changeset.sha"`)
supersedes: ["0170"]
superseded_by: []
related: ["0077", "0098", "0165", "0180"]
evidence: []
consumers: []
---

## Context and problem

**Release always refuses.** `build-package`'s `package` step reads `sha = "artifact:Changeset.sha"`, the commit the
item's change was summarised at, and ADR-0170 decision 6 fixes the Rust handler's check: *"A declared binary the build
did not produce is refused, and so is a `sha`/`HEAD` mismatch"*. The handler compares the input `sha` with the project
root's `HEAD` and refuses any difference (`sha-head-mismatch`). But a run commits its own bookkeeping after the
`Changeset` is bound: stage marks, gate trailers and the managed artifact directory, all under `.redkiln/`. By the
release stage the run tree's `HEAD` is never the `Changeset` commit, so the check refuses every release, although the
product tree being packaged is exactly the reviewed one. The S8J-C4 lane proofs measured that refusal at a
lane's release stage, and a probe that skipped the check carried the lane on.

The check exists for a real reason, which this record keeps: *"a package built from another tree is not that sha's
artifact"*. What the reason needs is the product tree, not the commit id.

## Decision drivers

- **Never package another product tree under a reviewed sha.** Any difference outside the run's own bookkeeping still
  refuses, and the refusal names the path.
- **The run's own bookkeeping is not product.** `.redkiln/` is the pack and the run's managed state; the build reads
  none of it.
- **One git wrapper** (ADR-0077, ADR-0098). The tree comparison is `rk-git`'s gix range read, which the adapter's
  Checkpoint 8a guard (6) allowlist already admits (`rk_git::diff`), so no new route to git is opened.
- **No silent widening.** A `sha` that does not resolve is still a mismatch, not an acceptance.

## Considered options

### Option — Compare the trees outside `.redkiln/` (taken)

Accept `HEAD` when `sha..HEAD` changes only paths under `.redkiln/`. Taken: the operator's ruling A on D4.

### Option — Package at the `Changeset` sha in a scratch checkout

Build from a detached tree at `sha` instead of the run tree. Not taken: it adds a second worktree lifecycle to the
adapter, which spawns no git (guard (6)), and the bytes built would be identical whenever this record's check passes.

### Option — Re-bind `sha` to the run tree's `HEAD` in the catalog

Edit `build-package` to package `HEAD`. Not taken: it drops the link to the reviewed `Changeset`, which is what the
check protects.

## Decision

**1. The tree rule.** When the input `sha` equals the project root's `HEAD`, `package` proceeds as before. When it
does not, the handler reads `<sha>..<HEAD>` through `rk_git::diff::range_diff` over the project root (a gix read that
spawns nothing) and:

- **accepts** when every changed path lies under `.redkiln/` (no path at all, or only bookkeeping), and builds;
- **refuses** as `sha-head-mismatch` when any changed path lies outside `.redkiln/`, naming the first such path in
  path order, the input `sha` and `HEAD`;
- **refuses** as `sha-head-mismatch` when the range cannot be read (a `sha` that does not resolve, or a store fault),
  naming `HEAD` and the read's error.

**2. What `BuildArtifact` carries does not move.** It records the input `sha` and the project root's `head`, as
ADR-0170 decision 6 fixes; after this record they may differ, by bookkeeping only.

**3. The Node adapter** has no `package` handler and no sha check, so nothing moves there.

**4. What is superseded, in part and quoted.** ADR-0170 decision 6's *"and so is a `sha`/`HEAD` mismatch"* is
narrowed to a mismatch of the trees outside `.redkiln/`. The rest of decision 6 (the locked release build, the declared
binary set, the relative paths, the refusals by name) and ADR-0170's forbids stand. ADR-0170 stays `accepted`; its
`superseded_by` gains `0203` in this record's flip commit, not before (ADR-0092).

## Consequences

**Positive**

- A release stage can package the reviewed product tree after the run's bookkeeping commits.
- A product change between the reviewed sha and `HEAD` is still refused, and now by the path that differs.

**Negative**

- **The check reads a diff on every release where `sha` is not `HEAD`.** `range_diff` also reads each changed blob; the
  changed paths are bookkeeping, so the read is small.
- **`.redkiln/` is trusted as non-product by name.** A product whose build reads a file under `.redkiln/` would be
  packaged from a tree that differs there; no shipped project does.

**What this forbids**

- Forbids accepting a `HEAD` whose tree differs from the `sha` commit's at any path outside `.redkiln/`.
- Forbids accepting a `sha` that does not resolve, or a range that cannot be read.
- Forbids a refusal of a product difference that does not name the differing path.
- Forbids reading the trees through anything but `rk-git` (ADR-0077, ADR-0098), and forbids a git spawn in the adapter.

## Evidence and confidence

- [measured 2026-10-18, S8J-C4's lane proofs on PR #103] `build-package`'s `package` step refused as
  `sha-head-mismatch` after the run's `.redkiln/` bookkeeping commits; a probe that skipped the check carried it on.
- [measured 2026-10-18, `s8j` build, package S8J-C4-FIX D4] `rk-adapter-toolchain-rust`'s
  `verbs::package_accepts_a_head_whose_tree_is_the_shas`, `verbs::package_accepts_a_head_that_differs_only_under_dot_redkiln`
  and `verbs::package_refuses_a_product_difference_naming_its_path`, the first two red on the base with
  `sha-head-mismatch`, then green; the existing `package_digests_only_the_declared_binaries_of_a_locked_release_build`
  still refuses an unresolvable `sha` as `sha-head-mismatch` naming `HEAD`.
- **Confidence.** High: the rule is a filter over the one range read the adapter already makes for `change_summary`.
