---
id: "0092"
title: Errata to a frozen record live in ERRATA.md; the four authority documents have one precedence order; a cut id may be re-issued under a hazard table
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md (PIA-036, PIA-039, and the erratum remediations of PIA-019, 023, 042, 065, 067, 074, 083, 086, 087, 092, 093, 095, 102, 104); references/adr/README.md §Open items
supersedes: ["0085"]
superseded_by: []
related: ["0012", "0084", "0089"]
evidence: ["PIA-036", "PIA-039", "PIA-037", "PIA-029"]
consumers: []
---

## Context and problem

ADR-0085 makes accepted records immutable ("supersede, never edit") and gives every acceptance criterion an owning
instrument, but it leaves three things unsaid that the pre-implementation audit of 2026-09-10 measured the cost of.

**There is no way to correct a frozen record's evidence line.** The audit found wrong evidence lines — a misnamed paper,
a mis-cited arXiv id, a fixed-version number attached to the wrong crate, a "no first-party source" that has one, a
"no external norm" that has two — on eleven accepted records (0012, 0019, 0074, 0075, 0077, 0080, 0081, 0082, 0084,
0085, 0086, 0088, 0089) whose *Decisions* are not moved by the correction [measured, PIA-023/042/074/079/083/086/087/
092/093/095]. The only precedent for correcting a frozen record is one dated note under README §Open items (the 0067
correction), which does not scale to twenty-odd corrections and gives a reader of the record no signal that a
correction exists. Writing a six-section successor for each would add a dozen records to the Supersessions table that
withdraw nothing.

**No rule ranks the design, the AC register, the SM register and the runbook.** The design says "the register decides"
(:3), the register says the design is its source, the RUNBOOK says "where the two disagree the design governs" (:4),
and the only ownership statement points at thirteen `references/areas/ac-*.md` files that were never committed
[measured, PIA-039, PIA-029]. Step 5 would build rk-types against two renderings of the same criterion (AC-511 is
"unresolved" in five design lines the register settles; the design says `schema_version` where AC-111/AC-117 say
`version`) with no rule for which wins [measured, PIA-036].

**The register re-issued six cut ids against ADR-0085 rule 7.** AC-185, AC-199, AC-323, AC-428, AC-511 and AC-512 were
cut and then re-issued to different criteria, each with an inline hazard note and a §0.4 table, because renumbering
"is the one change that breaks every citation already written" [measured, register §0.4:108-125]. Rule 7 says "ids are
never reused within their scope". The register's practice is sound and the rule is one sentence too strict.

This record supersedes ADR-0085 **in part**: it adds a tenth convention (errata), an eleventh (precedence) and
narrows rule 7. Rules 1–6, 8 and 9 stand; ADR-0085 stays `accepted`.

## Decision drivers

- A correction that a reader cannot find from the record it corrects is not a correction; a successor that moves no
  decision is noise in the supersession graph. The convention has to be findable from the record and cheap.
- Two implementers reading two renderings of one criterion ship two things (PIA-036's `version`/`schema_version`,
  PIA-039's AC-511). The build cannot start on "the design governs" and "the register decides" both being true.
- The register has already chosen re-issue-with-a-hazard-table over renumbering six times and the audit found the
  practice correct; a rule the record's own maintainers have to break is the wrong rule.
- ADR-0012's discharge-by-id grep and the README's supersession machinery must resolve on the first try; a
  correction hidden in Open items prose defeats both.

## Decision

**1. Errata to an accepted record are recorded in `references/adr/ERRATA.md`, never in the record.** One section per
record, ordered by record number, each entry dated and carrying: the frozen sentence quoted verbatim with its line at
the pin it was read at; what is true, with the source and its fetch date and an evidence tag; what the correction does
**not** change; and the audit or finding id that raised it. The record itself is not touched — not its body, not its
frontmatter. `references/adr/README.md` carries one pointer to ERRATA.md under Open items and a per-record marker
in the legacy-id map's status column (`accepted (errata)`), so a reader of the index knows to look.

**The line between an erratum and a supersession is the Decision.** An erratum corrects an *evidence line*, a
*citation*, a *number* or a *characterisation* in Context, Drivers, Options or Evidence, and leaves every sentence of
`## Decision` and every bullet of `**What this forbids**` standing as written. A correction that would move either is
a supersession and takes a new record. A correction that removes the *only* evidence a Decision rests on is also a
supersession — the Decision now rests on nothing, and that is a change to the Decision.

**2. Precedence among the authority documents is fixed, in this order.**

1. **Accepted ADRs govern every decision** and change only by supersession (rule 3, unchanged). A design row, a
   criterion or a runbook step that contradicts an accepted ADR is the defect; the ADR is not.
2. **The AC register (`references/2026-09-08-acceptance-criteria.md`) is the in-tree owner of each criterion** — its
   GIVEN/WHEN/THEN, its `*Verified by:*` instrument, its state label (binding / PROVISIONAL / CONDITIONAL / HUMAN
   GATE / retired) and its id. Where the design renders a criterion differently, the design is amended to match;
   the register's THEN is never edited to match the design. The thirteen `references/areas/ac-*.md` files that
   authored the register are history, not owners; every sentence that says "the area file owns the wording" is to
   read "the register owns the wording".
3. **The SM register (`references/2026-09-08-success-measures.md`) owns targets, baselines, kills and sampling.** A
   measure that restates an AC's executable precondition takes the AC's number for that half.
4. **Spike files govern the brief** (`spikes/METHOD*.md`, `spikes/F##-DRAFT.md`, `spikes/FINDINGS.md` over
   `references/2026-09-08-evidence-brief.md` and the design's restatements): a number is quoted from the spike file
   that computed it, and a restatement that disagrees with its spike file is corrected toward the spike file.
5. **The runbook (`runbook/`) defers to the design for phase order and to the AC register for every threshold and
   instrument.** It decides execution order and nothing else; where it must decide, it says so in its header table
   and names the record.

**3. Rule 7 is narrowed: a cut id may be re-issued under a hazard table.** An id that was allocated and then cut may
be re-issued to a different criterion **only** when (a) the cut criterion had no citation outside the register at the
time of the cut, (b) both the cut site and the re-issue site carry an inline note naming the other, and (c) the
register's §0.4 table lists the id with both meanings. An id with an external citation is retired, never reused.
The six existing re-issues (AC-185, AC-199, AC-323, AC-428, AC-511, AC-512) are recorded as conforming to this rule;
they are not debt.

## Considered options

### Option — one successor record per corrected ADR
House-rule-pure: twelve partial supersessions whose Decision sections read "unchanged; the evidence line is
corrected". Rejected: it adds twelve rows to the Supersessions table that withdraw nothing, forces every future
reader to open two records to learn that one number moved, and — the decisive point — makes "superseded in part"
mean two different things in the same table.

### Option — grow README §Open items
The 0067 precedent, applied literally. Rejected: ~25 bullets under an index heading, no per-record locality, and
nothing on the record or the legacy-id map tells a reader a correction exists.

### Option — permit editing evidence lines in place
MADR itself allows Consequences and Confirmation to be "updated over time". Rejected: ADR-0085 rule 3 adopted
immutability deliberately so that "a record says what it said when it was cited", and the audit's anchors (603 of them
at `file:line`) are exactly the citations that in-place edits would silently orphan.

### Option — "the design governs"
The RUNBOOK's existing sentence. Rejected: the design is 2,052 lines of prose restating 514 criteria; the register
is the thing with ids, instruments and state labels, and it is the thing the checkpoints grep. The design governs
*shape and order*; that is what clause 2.5 gives it.

### Option — keep rule 7 as written and renumber the six
Rejected by the register's own argument: renumbering breaks every citation already written, including the audit's.

## Consequences

**Positive**
- Twenty-odd evidence corrections land in one file a reader can find from the record's index row, and the
  Supersessions table keeps meaning one thing.
- The `version`/`schema_version` and AC-511 class of divergence has a resolution rule an implementer can apply
  without asking.
- Six re-issued ids stop being a latent rule violation.

**Negative**
- ERRATA.md is a second place to look. The mitigation is the marker on the README row; a record without the marker
  has no errata, and that has to stay true.
- The precedence order makes the register the busiest file in the corpus (6,567 lines). That is already the case.
- Clause 2.2 means a design row can be wrong for as long as nobody amends it toward the register; the audit's
  mirror diff (§8 M-04) is the instrument that finds those, and it ran over four keyword classes only.

**What this forbids**
- Forbids editing any line of an accepted record to correct it, including its evidence lines; the correction goes to
  ERRATA.md or to a successor.
- Forbids an erratum that touches a `## Decision` sentence or a `**What this forbids**` bullet, or that removes the
  only evidence a Decision rests on — those are supersessions.
- Forbids resolving a design/register disagreement by editing the register's THEN to match the design.
- Forbids naming `references/areas/ac-*.md` as the owner of any wording.
- Forbids re-issuing a cut id without the three conditions of clause 3, and forbids reusing an id that has an
  external citation.
- Forbids the runbook deciding a threshold, an instrument or a phase boundary; it may only name the record that does.

## Evidence and confidence

- [measured, PIA-023/042/074/079/083/086/087/092/093/095/102/104] The evidence-line defects on accepted records that
  this convention exists to record; each is entered in ERRATA.md with its source and fetch date.
- [measured, PIA-036, PIA-039] The five design lines that call AC-511 "unresolved" against a register that settles
  it; `schema_version` vs `version` at design :534/:1333 against AC-111/AC-117/ADR-0072; thirteen area files named as
  owners at design :349/:1968 and register §16, none in the tree.
- [measured, register §0.4:108-125] The six re-issued ids with their hazard notes, and the register's stated reason
  for not renumbering.
- [documented] README.md:195 — the 0067 correction "lives here because the record is frozen" — the precedent this
  record generalises.
- [documented] MADR permits in-place updates of Consequences/Confirmation (0085's own evidence line); this record
  keeps 0085's stricter choice and adds the outlet that choice lacked.
- **What this evidence does not license.** It does not license claiming any of the three conventions is externally
  normed; all three are this repository's own, adopted for the reasons above. It does not license re-tagging any
  corrected number `[measured]` on the strength of the erratum alone — an erratum carries the tag its source earns.
