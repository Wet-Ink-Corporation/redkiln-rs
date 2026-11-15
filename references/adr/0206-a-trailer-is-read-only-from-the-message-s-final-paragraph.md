---
id: "0206"
title: "**A trailer is read only from the commit message's final paragraph** (GitHub issue #18; Ryan's ruling A on the decision card of 2026-11-15 15:34Z, Weigh-In wi-3f8ab271bf): `parse_body` keeps `0155` clause 3's three gates (column 0, `\": \"` with the space, case-sensitive key) and adds a fourth, position. Only the lines of the body's last non-blank paragraph are candidates, so a trailer-shaped line quoted in prose earlier in the message is prose, not tier-1 state"
status: proposed
date: 2026-11-15
legacy_id: none
source: GitHub issue #18 (status-quo audit 2026-10-06, appendix §4.2); Ryan's decision card 2026-11-15 15:34Z, option A "Final paragraph only"; .kb/decision/25cfc87890-18-may-the-trailer-parser-be-limited-to-the.md
supersedes: ["0155"]
superseded_by: []
related: ["0105", "0109", "0155", "0188"]
evidence: []
consumers: ["crates/rk-git/src/trailer/grammar.rs", "crates/rk-git/src/reconcile.rs"]
---

*Supersedes `0155` **clause 3 in part**: the three gates stand, and position is added as a fourth. Every other
clause of `0155` is untouched.*

## Context and problem

`rk_git::trailer::grammar::parse_body` treated every column-0 `Key: value` line anywhere in a commit body as a trailer
(`grammar.rs:229-247` before this record). `0155` clause 3 names column 0, the `": "` separator and the key's case as
the only gates. Git's own trailer block is narrower: it is the message's final paragraph (`git interpret-trailers`).
So a body paragraph that quotes `Story: demo/x` or `Gate: …` could mint a phantom signal that `reconcile` counts, or a
spurious `Malformed` drift row. Tier-1 state is derived from this parser, so a quoted line becomes a transition.

Redkiln writes its own trailers only through `git commit --trailer` and `git interpret-trailers`
(`trailer/write.rs`), and both put them in the final paragraph. The exposure is hand-written or pasted commit text on
a ref `reconcile` walks.

## Decision drivers

- A trailer the kernel did not write must not read as tier-1 state (RK-B0013's family: a reading that is not a
  verdict must not count as one).
- `0155` clause 3's warning: *"a skipped line is a trailer that silently does not exist."* Narrowing must not drop
  any trailer redkiln itself writes.
- AC-165: malformed trailer lines are surfaced, not swallowed.

## Considered options

1. **Final paragraph only (chosen).** Parse only the last non-blank paragraph, keeping the existing per-line gates.
2. **Keep the status quo and name the residual.** No code change. A quoted line still mints a phantom signal.
3. **Delegate to `git interpret-trailers --parse`.** Same rule, but one more git spawn per commit read, and git's
   extra heuristics (the 25% trailer-line ratio, configured keys, the `---` divider) would become part of tier-1's
   grammar without a record naming them.

## Decision

**1.** `parse_body` reads only the body's final paragraph: the last run of non-blank lines, after trailing blank
lines are skipped. A line is blank when it is empty or whitespace only. The three gates of `0155` clause 3 apply to
each line of that paragraph exactly as before.

**2.** Within the final paragraph nothing else changes. A malformed known-key line is still surfaced (AC-165), and
an unknown key is still counted. The paragraph is **not** required to be all trailer-shaped. Requiring that would
let one prose line silently delete a real trailer block, which is the failure `0155` clause 3 warns about.

**3.** A trailer-shaped line in any earlier paragraph is prose. It is not a trailer, not `UnknownKey` and not
`Malformed`.

## Consequences

- A body quoting `Story:` or `Gate:` mid-message no longer moves tier-1 state or raises drift.
- Every trailer redkiln writes is still read, because `git commit --trailer` and `git interpret-trailers` append to
  the final paragraph.
- A hand-made commit whose trailers are followed by another paragraph loses them. That matches what git itself
  reports for that commit.

**What this forbids**

- Reading a trailer from any paragraph but the last.
- Dropping a final-paragraph trailer because a neighbouring line in that paragraph is prose.
- Calling `git interpret-trailers --parse` from the read path in place of this rule without a successor record.

## Evidence and confidence

Pinned by `rk-git`'s grammar tests: a trailer-shaped line in an earlier paragraph is ignored, the same line in the
final paragraph is read, trailing blank lines and CRLF line ends do not move the block, and a malformed line in the
final paragraph is still surfaced. The F12 corpus replay (`corpus_f12.rs`) is the check that no measured trailer sat
outside a final paragraph: its pinned counts must hold unchanged. Confidence: high for redkiln-written commits, where
the write path guarantees the position. Legacy or hand-made history is covered only as far as F12 samples it.
