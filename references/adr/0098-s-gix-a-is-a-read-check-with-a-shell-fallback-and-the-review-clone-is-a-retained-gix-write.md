---
id: "0098"
title: S-GIX/a is a read-path check whose no-go falls back to the wrapper; the review clone is a gix write op retained on purpose
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-009 (§3.3), PIA-042 (§4.3); references/adr/0077 §Decision and spike table; gitoxide crate-status.md (fetched 2026-09-10); docs.rs gix-object 0.64.1
supersedes: ["0077"]
superseded_by: []
related: ["0018", "0084", "0105"]
evidence: ["PIA-009", "PIA-042", "F12"]
consumers: []
---

## Context and problem

ADR-0077 splits `rk-git` by direction: gix owns the read path — including "`rev-parse`, including resolving the
repository common dir that 0018 requires at startup" — and a process-exec wrapper owns the write path. Its spike table
registers S-GIX/a as "can a store rooted at the git common dir be opened and **written** correctly from a linked
worktree" with the answer "**No** — the shell-out fallback is decided. It gates only the *narrowing*."

Three things do not fit. The operation S-GIX/a tests — resolving the common dir from a linked worktree — is on the
**read** path ADR-0077 gives to gix, and gix's own `crate-status.md` says the `GIT_COMMON_DIR`/`GIT_WORK_TREE`
interplay is not implemented; so on a no-go the read path has no owner for the one read every startup performs, and
"a no-go costs nothing" is false for arm a [measured, PIA-009]. Second, "shallow clone by SHA for review checkouts"
is listed under the read path, but a clone is a **write** — a `gix-worktree-state` checkout on the advisory surface
(`gix-worktree`, `gix-worktree-state`, Windows reserved names) the hybrid says it avoids [measured]. Third, ADR-0077's
gitoxide citations point the wrong way: "shallow clone by SHA, confirmed in gitoxide #2309" cites a discussion showing
a panic (the API is `PrepareFetch::with_revision` + `with_shallow`, PR #2824, 2026-08-05), and `crate-status.md` lists
"parse trailers" as an unchecked box although `BodyRef::trailers()` exists in gix-object 0.64.1 — so S-GIX/b's
**read** half is already discharged and its write half remains [documented, fetched 2026-09-10, PIA-042].

This record supersedes ADR-0077 **in part**: the S-GIX/a row, the placement of the review clone, and the two
citations. The hybrid itself, the one-wrapper rule, the write set and the trailer-writing question (S-GIX/b's write
half) stand; ADR-0077 stays `accepted`. The runbook's **ADR-0091** remains Step 6.1's record of the S-GIX *result*
and the minimum `git` version.

## Decision drivers

- A startup read with no owner on one branch of a registered spike is a Phase-1 blocker hiding behind "gates only the
  narrowing".
- A clone writes a working tree; calling it a read does not move it off the advisory surface.
- A citation that points at a panic thread is not evidence for the API it names.

## Decision

**1. S-GIX/a is a READ-path check, restated**: "does gix discovery from a linked worktree return the main repository's
common dir at the pin?" Its consumer is the read-side owner of `rev-parse`. **On a go**, gix resolves the common dir.
**On a no-go**, common-dir resolution moves into the exec wrapper as `git rev-parse --git-common-dir`, invoked once at
startup and cached for the process; nothing else changes. Design §10.5's "Gates Phase 1?" cell for arm a reads "Yes
for the read-side owner of `rev-parse`; the shell fallback is decided". The runbook's Step 6.1 replaces "a no-go costs
nothing" with the two outcomes. Nothing narrows "no gix for `worktree add`/`remove`" — gix exposes no worktree-add API
at the pin to test.

**2. The review clone is a gix WRITE operation, retained deliberately.** `PrepareFetch::with_revision` +
`with_shallow` (gix 0.87.1) gives a typed shallow-by-SHA clone the wrapper cannot express as cheaply; it is kept on
gix with its advisory exposure named: `clone` is added to the operations ADR-0084's `cargo audit` job and the Windows
CI target specifically cover, and the review clone is **bare** where the review needs no working tree (ADR-0105
states which reviews do). ADR-0077's "narrows away from checkout" consequence is qualified: the hybrid narrows away
from *worktree* checkout, not from clone.

**3. S-GIX/b's read half is discharged** — `BodyRef::trailers()` parses trailers at the pin — and its **write** half
stands as registered (a first-class trailer-writing API; none found by enumerating gix 0.87.1's 1,533 items,
gix-object 0.64.1's 142 and gix-actor 0.42.0's 58 [documented, fetched 2026-09-10]). A new arm **S-GIX/c** registers
the clone: "does `PrepareFetch::with_revision` + `with_shallow` at the pin produce a checkout whose HEAD tree equals
`git clone --depth 1 --revision <sha>`'s?" (consumer: the review-clone path; cost: one fixture, minutes).

**4. The two citations are corrected in ERRATA.md** against ADR-0077 (:112 and the crate-status trailer line).

## Considered options

### Option — fold this into ADR-0091 at Step 6.1
Rejected: ADR-0091 records the *outcome* of a spike that has not run; the fallback branch and the clone's placement
are decisions the spike does not depend on, and the runbook needs them before Step 6 starts.

### Option — move the review clone to the wrapper
`git clone --depth 1 --revision`. Rejected narrowly: it works and it would keep gix read-only in the strict sense, but
it gives up the one typed write that has no porcelain to parse, and the advisory exposure is covered by the audit job
either way. Retained as the fallback if S-GIX/c is a no-go.

## Consequences

**Positive**
- Every startup read has an owner on both branches of S-GIX/a; "gates only the narrowing" is true again.
- The clone's exposure is named and audited rather than mislabelled.
- S-GIX/b's scope shrinks to the half that is actually unknown.

**Negative**
- The read path is no longer gix-only in principle: one read may live in the wrapper. That is one call site.
- A third S-GIX arm.

**What this forbids**
- Forbids describing S-GIX/a's no-go as costing nothing, and forbids a startup that has no common-dir owner on either
  branch.
- Forbids listing the review clone under the read path or claiming the hybrid avoids checkout entirely.
- Forbids citing gitoxide discussion #2309 for shallow clone by SHA.

## Evidence and confidence

- [documented, GitoxideLabs crate-status.md, fetched 2026-09-10] the common-dir/work-tree interplay is not
  implemented; "parse trailers" is an unchecked box.
- [documented, docs.rs gix-object 0.64.1] `BodyRef::trailers()` exists.
- [documented, gitoxide PR #2824, 2026-08-05] `PrepareFetch::with_revision`; discussion #2309 shows a panic.
- [measured, PIA-009] the read-path/write-path mismatch and the false "costs nothing".
- **What this evidence does not license.** It does not license claiming gix resolves the common dir from a linked
  worktree — S-GIX/a is unrun. It does not license claiming the gix clone matches git's — S-GIX/c is unrun.
