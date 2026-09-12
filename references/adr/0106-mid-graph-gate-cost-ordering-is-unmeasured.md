---
id: "0106"
title: A mid-graph gate resumes into the same step; whether it costs less than a stage gate is unmeasured, and placement is not derived from a cost model
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-080 (§4.4), PIA-073; references/adr/0087 clause 3, forbids, evidence :88; spikes/F38-DRAFT.md §2; spikes/runner/src/main.rs :3288, :3349; spikes/METHOD-P.md §5.4
supersedes: ["0087"]
superseded_by: []
related: ["0016", "0017", "0078", "0086"]
evidence: ["PIA-080", "F38", "F28", "F19", "F22"]
consumers: []
---

## Context and problem

ADR-0087 decides that a human gate suspends the activity graph and resumes into the same step across a real process
boundary — and its title, clause 3 and one forbids bullet add "and costs less than a stage gate … measured, it is
cheaper". The measurement is F38 §2: resume output tokens 8,408 / 10,314 / 14,579 = 0.45× / 0.55× / 0.78× of F28's
~18.6k. The numerators are the resumed dispatch's **`output_tokens`** (main.rs:3288); the denominator is F28's
**`cache_creation_input_tokens`** (main.rs:3349 hard-codes 18600.0) — unlike counters [measured, PIA-080]. On the
same field, the resumed dispatches created 84,960 / 49,468 / 46,961 cache tokens = **4.57× / 2.66× / 2.52×** of
18,603, and METHOD-P §5.4's ">1.5× kills" criterion would have fired on all three (subject to the cross-fixture caveat:
F28's gate and F38's resume are different fixtures) [measured, F38 result files]. The design's own n=3 rule (ADR-0080)
forbids deciding on three cells whose intervals overlap, and SM-002/SM-052/SM-073 name one instrument and quote the
other. The "3.66× vs 4.4×" contradiction behind R-P7RATIO is two denominators — $9.857/$2.222 (issue-77 `recommended`
r1, one run) vs $9.857/$2.693 (the arm mean, n=30) — and is reconciled by naming them.

The suspend-and-resume shape stands on F19 (174/174 lossless at activity grain) and F38 §1 (3/3 lossless across a
real kill), not on cost. This record supersedes ADR-0087 **in part**: the title's cost clause, clause 3's "Measured, it
is cheaper", the matching forbids bullet, and the :88 evidence line. Clauses 1–2, the fresh-process rule, "what P8 did
not settle" and every other forbids bullet stand; ADR-0087 stays `accepted`.

## Decision drivers

- A ratio of unlike counters is not a measurement; a decision resting on it must be re-founded on what was measured.
- The cost question is real and open; the honest state is UNMEASURED with the right instrument named for P8-B.
- Placement should follow what a gate is for — a human answer at the point the graph needs it — and the record should
  say that placement is not to be derived from a cost model in either direction.

## Decision

1. **Clause 3 is restated:** gate placement is a value decision. A mid-graph gate is placed where the graph needs the
   answer; it is neither to be avoided nor preferred on cost grounds, because **the cost ordering between a mid-graph
   gate and a stage gate is UNMEASURED.** The forbids bullet now reads "forbids deriving gate placement from a cost
   model" in either direction.
2. **The comparator unit is fixed for P8-B.** Resume cost is `cache_creation_input_tokens` on the first post-resume
   dispatch (summed across the turns of that dispatch), reported beside `total_cost_usd` — SM-073's unit. METHOD-P §5.4
   gains a dated note fixing the token type; F38 gains a dated correction restating §2 on that field beside the output
   tokens; SM-002 :48-49 and SM-052/SM-073 re-baseline to **no baseline**; design :735 and AC-266's provisional note
   state the class and the corrected numbers. The three "buys nothing" sites read "F28 bounds what a keep-warm could
   buy at one prefix rewrite (18.6k–29.7k) per gate; F22 forbids the process anyway" — F28 ran no keep-alive arm.
3. **R-P7RATIO is reconciled by denominator identification** (4.4× = issue-77 r1 single run; 3.66× = the
   `recommended` arm mean, n=30) and marked done; design §0.1 row 24, SM-067 and SM-110 carry the two figures with
   their denominators; the `emit.corpus_sha` re-keying remains.
4. **Nothing else in ADR-0087 moves.** F38's losslessness (n=3, one step, inside the TTL) is the evidence the shape
   rests on; the multi-step case is Phase-1 exit criterion (3).

## Considered options

### Option — restate SM-002's instrument as output tokens
Keep F38's numerators and change F28's comparator. Rejected: output tokens do not price a resume — the cost is the
rebuilt prefix — so the 1.5× criterion would measure the wrong thing.

### Option — file the correction as an erratum
Rejected: "costs less" is in the title, a Decision clause and a forbids bullet; ADR-0092 puts that above the erratum
line.

### Option — declare a stage gate cheaper on the corrected figures
Rejected: three cells, cross-fixture, intervals overlapping — the n=3 rule ADR-0080 wrote for exactly this.

## Consequences

**Positive**
- The gate's shape rests on what was measured; the cost question has an instrument and a unit.

**Negative**
- SM-002, SM-052 and SM-073 lose their baselines until P8-B runs on the right field.

**What this forbids**
- Forbids claiming a mid-graph gate costs less, or more, than a stage gate before P8-B reports on
  `cache_creation_input_tokens` and `total_cost_usd`.
- Forbids dividing `output_tokens` by `cache_creation_input_tokens`, or quoting 0.45×/0.55×/0.78× as a cost ratio.
- Forbids quoting 3.66× or 4.4× without its denominator.

## Evidence and confidence

- [measured, F38 result files; main.rs:3288, :3349] the two counters and the hard-coded denominator; the
  cache-creation figures 84,960 / 49,468 / 46,961.
- [measured, F28] 18,603 `cache_creation_input_tokens` per gate, flat across 65.2 min and 49.7 h; no keep-alive arm.
- [measured, F19; F38 §1] the losslessness this record keeps.
- [documented, ADR-0080] the n=3 rule.
- **What this evidence does not license.** It does not license a cost ordering in either direction.
