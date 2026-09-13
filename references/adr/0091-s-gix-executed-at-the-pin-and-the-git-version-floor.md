---
id: "0091"
title: S-GIX executed at the pin — gix owns the common-dir read and the shallow-by-SHA review clone, the trailer write path stays on the executable, and the git floor is 2.20
status: accepted
date: 2026-09-13
legacy_id: none
source: runbook/phase-1/s6-store-and-git.md §6.1 (the three arms) and §6.2 constraint 3 (the floor); references/adr/0077-gix-read-path-only-git-write-path-shells-out.md §Decision (the spike table, and the unestablished minimum version); references/adr/0098-s-gix-a-is-a-read-check-with-a-shell-fallback-and-the-review-clone-is-a-retained-gix-write.md decisions 1-3; references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md clauses 3 and 5; references/adr/ERRATA.md E-0077-1; spikes/DECISIONS-2026-09-12.md E10; git v2.49 release notes (/mingw64/share/doc/git-doc/RelNotes/2.49.0.adoc, read 2026-09-13); git-interpret-trailers(1) `ifExists`; gix 0.87.1, gix-object 0.64.1 and gix-actor 0.42.0 resolved source trees (read 2026-09-13)
supersedes: []
superseded_by: []
related: ["0077", "0084", "0098", "0105"]
evidence: ["S-GIX", "PIA-009", "PIA-042", "F12", "F20"]
consumers: []
---

## Context and problem

ADR-0077 split `rk-git` by direction — gix owns the read path, a process-exec wrapper owns the write
path — and registered spike **S-GIX** to decide whether that split could be narrowed. It left two
things open, and named both as open. First, the narrowing itself: *"A **go** on an arm is what may
narrow this record; nothing else may. Narrowing is written as a successor ADR that supersedes this
one in part, not as an edit."* Second, the startup precondition: *"That minimum version is to be
established and written into the assertion when the wrapper is built; it is not asserted here,
because this pass did not verify it"*, restated as a Negative — *"The minimum required `git` version
is not established in this record, and until it is, the startup assertion is incomplete."*

ADR-0098 then restated the arms without running them: it moved arm a to the read path and fixed its
no-go fallback, moved the review clone to the write side as a retained gix operation, discharged arm
b's read half against ERRATA E-0077-1, and registered a third arm for the clone. It said in terms
what it could not do: *"It does not license claiming gix resolves the common dir from a linked
worktree — S-GIX/a is unrun. It does not license claiming the gix clone matches git's — S-GIX/c is
unrun."*

This record is where the arms were run, and it is the second of the seven decisions the runbook
takes. It exists because two of the three outcomes were already decided and only the *measurement*
was missing — and a fallback commissioned on no evidence is the failure ADR-0077's own forbid was
written to stop.

## Decision drivers

- ADR-0077 forbids narrowing on a documentation read; the arms are defined as **executed tests
  against the pin**, so a verdict here must be backed by a run, not by an enumeration of docs.
- A floor asserted at startup has to be a **number**, or the assertion cannot be written at all.
  ADR-0077's startup precondition has been incomplete since 2026-09-08 for exactly this reason.
- The floor must hold on the CI runners, not only on the box that measured it (DECISIONS-2026-09-12
  E10).
- Every arm has a **pre-decided** fallback, which is what makes an unrun arm dangerous rather than
  merely unknown: writing `no-go` without running would silently commission that fallback.
- A no-go must be distinguishable from a not-measurable (golden constraint 10). Arm b is the case
  that tests this, and it is answered below rather than blurred.

## Decision

The three arms ran on 2026-09-13 as `#[ignore]`d tests in `rk-git`, before any worktree code, each
with its own negative controls. The verdicts are:

```text
S-GIX/a: go
S-GIX/b: no-go
S-GIX/c: go
```

1. **Arm a — gix owns the common-dir read.** `gix::discover()` from a linked worktree returns the
   main repository's common dir at the pin, agreeing with `git rev-parse --git-common-dir`. ADR-0077's
   read-path bullet is therefore **confirmed, not narrowed**, and ADR-0098 decision 1's go branch is
   the one in force: AC-178's startup resolution is a gix read. The wrapper's
   `rev-parse --git-common-dir` fallback is **not** commissioned and does not enter ADR-0105 clause
   5's write set.

   One qualification belongs in the decision and not only in the evidence: gix answers with an
   unresolved traversal (`<main>/.git/worktrees/<name>/../..`) where the executable answers with the
   resolved path. The two name the same directory — confirmed by `realpath` and by inode identity,
   independently of the comparator — so the consumer of this read **must resolve the path before
   comparing it to anything**, and the comparison that AC-178's test performs is between resolved
   paths.

2. **Arm b — the trailer write path stays on the executable, and this record narrows nothing.** The
   arm is a conjunction: a first-class trailer-*writing* API, **and** a byte-identical round trip.
   Its first conjunct is measured false. Every public item naming `trailer` in the resolved source of
   `gix 0.87.1`, `gix-object 0.64.1` and `gix-actor 0.42.0` is read-side; `gix` and `gix-actor` carry
   zero occurrences; the only write-sounding name, `without_trailer()`, returns a `&BStr` — a read
   that excludes the trailer block. The read half stays discharged (`BodyRef::trailers()`,
   ERRATA E-0077-1).

   **This is not a measurement that gix writes trailers incorrectly. It is a measurement that
   gix 0.87.1 offers no trailer-writing API to measure.** The byte half was run anyway, against the
   only route left once the first conjunct fails — appending the trailer text by hand — and its
   result is a property of *that route*, never of any gix API: **3 of 5** of ADR-0105 clause 3's
   named cases are byte-identical to `git interpret-trailers --if-exists=add`; `divider` diverges at
   byte 14 and `crlf` at byte 29. It is recorded as a per-case result and not as a ratio.

   Consequently **ADR-0077's write path stands in full**: every trailer write is `git commit
   --trailer` or `git interpret-trailers`, always with `-c trailer.ifexists=add` / `--if-exists=add`
   (ADR-0105 clause 3). ADR-0077:100's forbid — *"Forbids `gix` for writing a trailer until S-GIX/b
   returns go"* — is unchanged and still in force. **This record supersedes nothing.**

3. **Arm c — the review clone stays a retained gix write.** `prepare_clone_bare` +
   `with_revision(<sha>)` + `with_shallow(DepthAtRemote(1))` produced a bare clone whose HEAD tree
   object id equals that of `git clone --depth 1 --revision <sha>` over the same `file://` source, at
   a SHA that is deliberately not a branch tip. ADR-0098 decision 2's placement is confirmed: the
   clone is a gix write, bare unless the review activity declares it needs a working tree, and it
   remains inside the scope ADR-0084's `cargo audit` job and the Windows CI target cover. The
   wrapper's `git clone --depth 1 --revision` fallback is **not** commissioned.

4. **The git version floor is `git >= 2.20`, asserted at startup.** That is the version
   `interpret-trailers --if-exists=add` requires, and every other operation in ADR-0105 clause 5's
   unconditional write set predates it. The conditional half of DECISIONS-2026-09-12 E10 is resolved
   **downward by arm c's go**: `git clone --revision` was introduced in git 2.49, but on a /c go the
   wrapper never invokes it — gix performs the review clone — so nothing raises the floor above 2.20.
   Had arm c been a no-go the floor would have been `git >= 2.49`.

   The assertion is made at startup and not at first use (ADR-0077), and ADR-0077's Negative — that
   the minimum version is not established — is discharged by this clause.

5. **What is not claimed.** Whether the `ubuntu-latest` and `windows-latest` runner images satisfy
   this floor is **not verified here and is not claimed**. `.github/workflows/ci.yml` has never
   executed — nothing has been pushed — so runner conformance is verifiable rather than verified, and
   it is owed at the first push. E10's second half is therefore open.

## Considered options

### Option — record the result in `spikes/` rather than as an ADR
Rejected. `s6` §6.1 permits it in terms, but the preamble's decision table names an ADR, ADR-0077
requires a successor *record* for any narrowing, and Checkpoint 6 greps `references/adr/0091-*.md`
for the verdict lines. A result with nowhere to be cited from is not a decision.

### Option — defer the floor to §6.2, where the wrapper is built
Rejected. The preamble places decision 3 here, beside the verdicts, and ADR-0077 has carried an
incomplete startup assertion since 2026-09-08. Arm c's verdict is also what selects between the two
candidate floors, so the number is only determinable at the moment the arms are read.

### Option — write `no-go` for arm b because no API exists, without running anything
Rejected, though it reaches the same verdict. ADR-0077 defines the arms as executed tests against the
pin; a verdict taken from a docs read would be indistinguishable in the record from one taken from a
run, and every no-go commissions a pre-decided fallback. The enumeration was therefore performed
against the resolved source trees on disk, and the byte half was run.

### Option — report arm b as NOT MEASURABLE rather than no-go
Rejected, and the reasoning is recorded because the distinction is one this programme has got wrong
before. A criterion with no denominator is not measurable (golden constraint 10); a **conjunction
with a measured-false conjunct is false**. Arm b's first conjunct was measured, on disk, at the pin.
The verdict is a no-go, and the body says which conjunct failed so that a later reader cannot mistake
it for a byte-level defect in gix.

## Consequences

**Positive**

- The read path has an owner for the one read every startup performs, and it is the one ADR-0077
  assigned. The wrapper gains no new operation from this pass.
- ADR-0105 clause 5's write set is unchanged: neither `rev-parse --git-common-dir` nor
  `clone --depth 1 --revision` is commissioned, so the enumerated list still bounds the wrapper.
- The floor is a number, and the lowest defensible one. Nothing in the write set needs more than
  git 2.20, so the product's requirement on its host is as small as the design permits.
- The trailer fixture matrix and its oracle control now exist as executed tests, and they are the
  instrument §6.2's `trailer_roundtrip_matches_interpret_trailers` inherits.

**Negative**

- Two git implementations remain in one process, so ADR-0105 clause 4's re-read-after-write
  discipline stays load-bearing; arm a's traversal finding is a concrete instance of the two
  disagreeing about spelling.
- The trailer write path keeps a process spawn per trailer commit. Arm b's no-go means that cost is
  not removable at this pin, and revisiting it requires a new arm at a new pin, not a re-reading of
  this one.
- Arm b's byte figure (3 of 5) describes a hand-rolled route this product does not ship. It is
  recorded to show the matrix discriminates, and it must not be cited as a gix property.
- The floor is met by the measuring box by a wide margin (2.55.0 against 2.20), which means this
  record provides no evidence about hosts near the floor. Nothing here has been run on git 2.20.

**What this forbids**

- Forbids asserting any `git` floor other than `2.20` without a successor record, and forbids
  invoking `git clone --revision` from the wrapper at all, since arm c's go means the floor does not
  cover it.
- Forbids citing this record as evidence about any gix version other than `0.87.1`, about any
  operating system other than Windows, or about any clone transport other than `file://`.
- Forbids reading arm b's no-go as "gix cannot write trailers" rather than "gix 0.87.1 exposes no
  trailer-writing API, measured at the pin".
- Forbids treating this record as narrowing ADR-0077: it supersedes nothing, and ADR-0077:100's
  forbid on gix trailer writes remains in force.
- Forbids comparing an unresolved gix common-dir answer to a resolved one without resolving first.
- Forbids claiming the CI runners satisfy the floor until `ci.yml` has actually run.

## Evidence and confidence

- [measured, S-GIX/a, 2026-09-13] `gix::discover()` from a linked worktree and
  `git rev-parse --git-common-dir` resolved to the same directory, and to the main repository's
  `.git` rather than to `.git/worktrees/<name>`. gix answered
  `<main>/.git/worktrees/linked/../..`; `realpath` and inode identity confirm that names the same
  directory as `<main>/.git`, measured independently of the comparator under test.
- [measured, S-GIX/b1, 2026-09-13] Over the resolved source trees of `gix 0.87.1`, `gix-object 0.64.1`
  and `gix-actor 0.42.0`: `gix` **0** occurrences of `trailer`, `gix-actor` **0**, `gix-object` **141**,
  all in `src/commit/message/{body,mod}.rs`. Every public item naming `trailer` is read-side
  (`Trailers`, `TrailerRef`, `BodyRef::trailers()`, `message_trailers()`, the attribution helpers);
  `without_trailer()` returns `&BStr`. The read surface was additionally exercised at runtime, so its
  presence is a positive control on the absence of a write counterpart.
- [measured, S-GIX/b2, 2026-09-13] Over ADR-0105 clause 3's five named cases against
  `git interpret-trailers --in-place --if-exists=add`: `existing_block`, `no_trailing_newline` and
  `duplicate_token` byte-identical; `divider` first differs at byte 14; `crlf` at byte 29. **3 of 5.**
- [measured, S-GIX/b control, 2026-09-13] `git interpret-trailers` **without** `--if-exists=add`
  left one `Gate: pass` where the input already carried one; **with** the flag it produced two. The
  oracle is therefore sensitive to the flag whose omission ADR-0105 clause 3 exists to prevent, which
  is what makes the duplicate-token case an instrument rather than a formality.
- [measured, S-GIX/c, 2026-09-13] `prepare_clone_bare` + `with_revision` + `with_shallow(DepthAtRemote(1))`
  and `git clone --depth 1 --revision <sha>` over the same `file://` source produced HEAD trees both
  equal to `53819b7e0cdf98c12e7dee641f57f9a13c30a1f2`, at a SHA one commit behind the branch tip. The
  git-side clone was asserted genuinely shallow (a non-empty `shallow` file) before comparison, and a
  control confirms a full clone is reported as not shallow.
- [documented, /mingw64/share/doc/git-doc/RelNotes/2.49.0.adoc, read 2026-09-13] *"'git clone' learned
  to make a shallow clone for a single commit that is not necessarily be at the tip of any branch."*
  This is the `--revision` option, and the version that would have been the floor on a /c no-go.
- [documented, git-interpret-trailers(1)] `ifExists` defaults to `addIfDifferentNeighbor`, which is
  why every call carries `--if-exists=add` (via ADR-0105 clause 3).
- [measured, PIA-009 / PIA-042] the read-path/write-path mismatch ADR-0098 corrected, and the
  discharged read half of arm b.
- **Instrument.** `git version 2.55.0.windows.3` (`/mingw64/bin/git`); `rustc 1.98.1 (48a229cea
  2026-09-01)`; `gix = "=0.87.1"` resolving `gix-object 0.64.1`, `gix-actor 0.42.0`, `gix-pack 0.74.2`,
  `gix-protocol 0.65.1`; `cargo-deny 0.20.2`; `cargo-audit 0.22.2`; Claude Code `2.1.269`;
  MINGW64_NT-10.0-26200. Workspace graph 41 → 187 packages at this step. Every `git` invocation in the
  arms ran with `GIT_CONFIG_NOSYSTEM=1`, an empty `GIT_CONFIG_GLOBAL`, a fixed identity and fixed
  dates, and `-c core.autocrlf=false -c core.hooksPath=<empty> -c commit.gpgsign=false
  -c init.defaultBranch=main` — this box has `core.autocrlf=true` set globally and no `.gitattributes`,
  so without that isolation arm b's CRLF and duplicate-token cases would have measured the operator's
  configuration rather than git's behaviour.
- **What this evidence does not license.** It does not license any claim about the `git` version on
  the CI runners: `ci.yml` has never executed. It does not license extending any arm's result to
  Linux or macOS — all three ran on Windows only — nor to any gix other than `0.87.1`, nor arm c to
  any transport other than `file://` or to repositories with submodules, LFS or clean/smudge filters.
  It does not license reading arm b's byte figure as a property of a gix API, because arm b found
  none. It does not license any claim about behaviour at or near git 2.20, which was not exercised.
  It does not license treating arm c's single-fixture tree equality as evidence about packfile bytes,
  `.git` metadata or the shallow boundary, none of which were compared.
