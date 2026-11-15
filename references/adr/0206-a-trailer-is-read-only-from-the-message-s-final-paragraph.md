---
id: "0206"
title: "**A trailer is read only from a trailer paragraph** (GitHub issue #18; Ryan's ruling F on the decision card of 2026-11-15 17:05Z, after ruling A failed the F12 corpus replay): `parse_body` keeps `0155` clause 3's three gates (column 0, `\": \"` with the space, case-sensitive key) and adds a fourth, the paragraph. A paragraph is read only when every line in it is a trailer line or an indented continuation, so a trailer-shaped line quoted inside prose is prose, not tier-1 state"
status: proposed
date: 2026-11-15
legacy_id: none
source: GitHub issue #18 (status-quo audit 2026-10-06, appendix §4.2); Ryan's decision cards 2026-11-15 15:34Z (A, final paragraph only) and 17:05Z (F, any all-trailer paragraph); the corpus replay on Ryan's PC 2026-11-15 17:04Z; .kb/decision/25cfc87890-18-may-the-trailer-parser-be-limited-to-the.md
supersedes: ["0155"]
superseded_by: []
related: ["0105", "0109", "0155", "0188"]
evidence: ["F12"]
consumers: ["crates/rk-git/src/trailer/grammar.rs", "crates/rk-git/src/reconcile.rs"]
---

*Supersedes `0155` **clause 3 in part**: the three gates stand, and the paragraph is added as a fourth. Every other
clause of `0155` is untouched. The file name keeps the first ruling's wording; the decision is the second ruling's.*

## Context and problem

`rk_git::trailer::grammar::parse_body` treated every column-0 `Key: value` line anywhere in a commit body as a trailer
(`grammar.rs:229-247` before this record). `0155` clause 3 names column 0, the `": "` separator and the key's case as
the only gates. So a body sentence that quotes `Story: demo/x` or `Gate: …` on a line of its own could mint a phantom
signal that `reconcile` counts, or a spurious `Malformed` drift row. Tier-1 state is derived from this parser, so a
quoted line becomes a transition.

Redkiln writes its own trailers only through `git commit --trailer` and `git interpret-trailers`
(`trailer/write.rs`), which put them in a paragraph of their own. The exposure is hand-written or pasted commit text
on a ref `reconcile` walks.

## Decision drivers

- A trailer the kernel did not write must not read as tier-1 state (RK-B0013's family: a reading that is not a
  verdict must not count as one).
- `0155` clause 3's warning: *"a skipped line is a trailer that silently does not exist."* Narrowing must not drop
  any trailer redkiln writes, nor any trailer in the predecessor history AC-444 replays.
- AC-165: malformed trailer lines are surfaced, not swallowed.

## Considered options

1. **Final paragraph only (git's rule).** Ruled first (A, 15:34Z), then **refuted by measurement**: the F12 replay
   (`corpus_f12.rs`, `--features corpus`) on Ryan's PC rebuilt **4 slices and 2 ever rejected**, against 74 and 15 at
   base `51134da`. The predecessor's `Slice-Verdict:` trailers sit in a paragraph followed by another, so AC-444 fails.
2. **Any all-trailer paragraph (chosen, F).** Read every paragraph whose lines are all trailer lines or indented
   continuations, wherever it sits.
3. **Keep the status quo and name the residual (B).** No code change. A quoted line still mints a phantom signal.
4. **Final paragraph for live reads, whole body for legacy reads (D).** Two rules and a mode parameter to keep in step.

## Decision

**1.** `parse_body` splits the body into paragraphs on blank lines. A line is blank when it is empty or whitespace
only, and CRLF line ends split the same way as LF.

**2.** A paragraph is a **trailer paragraph** when every line in it is either a trailer line under `0155` clause 3's
three gates or an indented continuation (a line starting with whitespace). Every trailer paragraph is read, in order,
wherever it sits in the body. Continuation lines are never read as trailers themselves.

**3.** A paragraph with any other line is prose, and none of its lines is a trailer, `UnknownKey` or `Malformed`.

**4.** Within a trailer paragraph nothing else changes: a malformed known-key line is still surfaced (AC-165), and an
unknown key is still counted.

## Consequences

- A sentence quoting `Story:` or `Gate:` on a line inside a prose paragraph no longer moves tier-1 state or raises
  drift.
- Every trailer redkiln writes is still read, because the write path puts trailers in a paragraph of their own.
- A hand-made commit that mixes prose and trailer lines in one paragraph loses those trailers.
- A trailer-only paragraph quoted on its own (blank lines both sides) still reads as a trailer. That is a residual this
  rule accepts: it is how git-style trailer blocks look, and the predecessor history depends on reading them.

**What this forbids**

- Reading a trailer line from a paragraph that contains a non-trailer, non-continuation line.
- Narrowing to git's final-paragraph rule without a successor record and a passing F12 replay.

## Evidence and confidence

Pinned by `rk-git`'s grammar tests:
- a trailer line inside a prose paragraph is prose;
- a trailer-only paragraph is read wherever it sits (the predecessor's shape);
- an indented continuation keeps a paragraph a trailer paragraph;
- whitespace-only lines and CRLF split paragraphs;
- a malformed line in a trailer paragraph is still surfaced;
- a quoted `Story:` in prose moves no state through `reconcile`.

The F12 corpus replay is the check against legacy history: its pinned counts (74 slices, 15 ever rejected) must hold.
It runs only on a machine with the pinned corpus. **Option 1 failed it (4 and 2). Option 2 is owed a run before
merge.**
