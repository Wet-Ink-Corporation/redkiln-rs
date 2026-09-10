---
id: "0089"
title: A facet's computed fact must name its provenance; auth becomes a declared surfaces set, and absence of evidence is not false
status: accepted
date: 2026-09-09
legacy_id: none
source: spikes/F39-DRAFT.md; references/2026-09-09-change-classification.md §0, §4.1–§4.3, §5
supersedes: ["0030"]
superseded_by: []
related: ["0012", "0024", "0032", "0050", "0060", "0061", "0067", "0069", "0073", "0079"]
evidence: ["F39", "F36", "F18"]
consumers: []
---

## Context and problem

PC8 rule 2 (ADR-0030) requires that "every facet maps to a computed fact in the context vector — if nothing can
compute it, it cannot be a facet." The rule was written to keep free text out of selection. It does not say what
*kind* of computation counts, and the instrument the programme actually built satisfies it lexically: an
unanchored, case-insensitive regex over file paths **and diff lines**.

P1 arm D scored that instrument against 140 truth cells at corpus pin `47faec9f`, under METHOD-P amendment 9 —
60 cells decided by a deterministic oracle over git metadata, 48 by a unanimous model panel, and 32 adjudicated
by a human, blind [measured, F39]. The result is not a threshold miss:

| facet | precision | 95% CI | positives | verdict under METHOD-P §1.4 |
| --- | --- | --- | --- | --- |
| `ui` | 1.00 | [0.65, 1.00] | 7 | point clears 0.95, **interval does not** |
| `toolchain` | 0.73 | [0.48, 0.89] | 15 | stays LLM-emitted |
| `change` | 0.72 | [0.54, 0.85] | 29 | stays LLM-emitted |
| `binary_change` | 0.60 | [0.23, 0.88] | 5 | stays LLM-emitted |
| `flag_exists` | 0.00 | [0.00, 0.66] | 2 | stays LLM-emitted |
| **`auth`** | **0.00** | **[0.00, 0.26]** | **11** | stays LLM-emitted |
| `data_migration` | — | — | 0 | **not measurable** — no denominator |

`auth` emits eleven positives and every one is false, with an upper bound of 0.26. The rule carries **no signal**
on this corpus, and the collision is with the instrument's own vocabulary: `auth` is a substring of *authored*
(40 hits), `oauth` of *coauthor* (8), and `token` matches a design token (36) [measured, F39 §2]. That is a
finding about the method, not a tuning parameter — adding word boundaries to one alternative removes the
`coauthor` and ARIA-`role` hits and leaves the *authored* and design-`token` hits untouched.

Two further facts force this record rather than a smaller one. **Four of the twenty items changed no code at
all** — every changed file is `.bklg/` bookkeeping — the panel returned `unknown` on all sixteen of their
semantic-facet cells, and the script emitted a confident boolean on every one [measured, F39 §4]. The registry is
the cause: `facets.toml` declares the boolean facets `values = [true, false]`, so a derived boolean that cannot
be computed has nowhere to go but `false`, and AC-079's existing `gate_opened { reason: unknown_facet }` route
cannot fire for it. And `data_migration` emits zero positives across all 20 items and all three sources, so
precision is undefined and a perfect method scores identically — AC-070's degenerate-distribution clause already
covers it, and `external_dep` at 200/200 `false` [measured, F36] was its first named instance.

An independent research pass reached the same conclusion from six axes without reference to any of these numbers
[documented, `references/2026-09-09-change-classification.md` §0, §2, §6a].

This amendment withdraws **one clause of one rule** of PC8 — rule 2's silence on what kind of computation counts,
and the `auth` boolean named in its initial-facets sentence. Rules 1, 3, 4 and 5 stand unchanged, as does rule
2's prohibition itself. This is therefore a **partial** supersession, recorded the way this set records every
partial (cf. 0073 over 0029, 0059 over 0043, 0020 over 0015): the link is carried on both sides at record
granularity, and **ADR-0030 stays at status `accepted`** because only one clause is withdrawn.

## Decision drivers

- **Measured, not argued.** `auth` at 0.00 over 11 positives with an upper bound of 0.26 cannot be recovered by
  tuning; `data_migration` cannot be evaluated at all; no facet clears 0.95 [measured, F39].
- **Rule 2 is the rule that condemns the deriver, and it is also the rule that admitted it.** "A computed fact"
  was satisfied by a lexical scan. The prohibition needs a provenance requirement or it forbids nothing.
- **One boolean cannot carry two thresholds.** The evidence a threat-model activity needs and the evidence a
  session-fixation test needs are different questions; a repo-wide `auth: bool` forces both to inherit whatever
  threshold the regex author happened to pick. ADR-0024 already places admissibility on the activity, so the
  activity can own its own question.
- **Absence of evidence is a distinct outcome from evidence of absence**, and conflating them is how a silent
  `false` looks like a correct answer. The ordinals already have the third value; the booleans do not.
- **The affordable instrument the measured literature supports is a path-glob boundary map plus git metadata**
  [documented, `2026-09-09-change-classification.md` §2A, §4.5]. CodeQL publishes **no precision or recall as a
  classifier of anything** and costs $30/committer/month; plain Semgrep's measured numbers on adjacent tasks run
  from 0.57% to 23.8% recall; each relocates the vocabulary problem into a different DSL rather than solving it.
- **A per-item human gate is affordable only if it is per item.** F18 prices a gate at a median **10.5 min** with
  45% exceeding an hour [measured, n=174, one operator]. On this corpus the sixteen abstaining cells cluster on
  **four items**, so the gate is four questions, not sixteen [measured, F39 §4], and AC-074's decision-memory row
  replays the answer rather than re-asking it.

## Decision

Three parts. Each is a registry and predicate-grammar change inside PC8, not a break with it.

**1. A facet's computed fact must name its evidence source and its provenance class.** `facets.toml`'s `source`
field gains a companion **`deriver`** field naming the instrument, and no facet may declare `source = "derived"`
without a scored record for that deriver. The provenance classes, in precedence order, are **DECLARED** (a
checked-in map of path globs to closed surface names, plus a required directory convention for migrations),
**DERIVED** (git's own metadata, and parser-anchored call and import sites — never a substring over prose),
**PROPOSED** (a model reads the assembled diff and the card; its output is a *proposal* that cannot key a
`require` row until a human confirms it) and **ASKED** (a triage question answered once and replayed by AC-074).
A higher class overrides a lower one for the same fact, and the override is logged.

**A facet whose only computation is a lexical scan of free text does not satisfy rule 2 for `require`-row
purposes.** Rule 2's prohibition is unchanged in force and strengthened in content: the computed fact becomes a
declared boundary intersection or a git-metadata read, not a regex over prose.

**2. The `auth` boolean is withdrawn and replaced by a set-valued `surfaces` facet.** Closed vocabulary, values
sourced from a checked-in `surfaces.toml` mapping path globs to closed surface names, extensible in *values* by a
technology pack — which ADR-0030 rule 1 already permits. Set-valued facets are precedented: `toolchain` and
`change` are already `set = true` [documented, `facets.toml`], and ADR-0050's "same facet disjoins, distinct
facets conjoin" gives the right semantics for `surfaces` ∋ {auth, payments}. A fact is the **intersection of the
changeset with a declared boundary** — a structural question, not a lexical one. A path covered by no glob
contributes nothing; it does **not** imply `false`.

Each activity's `applicable_if` owns its own evidence question, which ADR-0024 already places on the activity and
ADR-0033's predicate grammar already expresses:

```
# broad: any declared security-relevant surface was touched
applicable_if = 'any(s in surfaces, s in ["auth", "payments", "secrets"])'

# narrow: a declared auth entry point was touched, and we know it structurally
applicable_if = 'any(s in surfaces, s == "auth") and surfaces.provenance != "proposed"'
```

Provenance is carried on the value, so a `require` row may demand a minimum: a cheap activity may run on a
DECLARED fact; an expensive or destructive one may demand `provenance in ["declared", "asked"]` and refuse to
fire on an unconfirmed proposal. Without this, every activity shares one trust level and the weakest evidence
sets the bar for all of them.

**3. Boolean facets become tri-valued at the context-vector level, and absence of evidence never coerces to
`false`.** `null` means "no evidence" and is not `false`. The grammar already supports it: ADR-0032 makes absent
inputs `null` and ADR-0069 defines `==`/`!=` against `null` while ordering against it is a lint error. A
`require` row whose predicate cannot be evaluated over a `null` yields `Unsatisfiable(kind, failed_predicates)`
and escalates to the existing human gate. **AC-079's `gate_opened { reason: unknown_facet }` route is extended to
the boolean facets**, which today it cannot reach because their declared domain is `[true, false]`. ADR-0067's
`recent_deploy` is the live precedent: it already emits `null` with rule `R-recent-deploy-feed-absent` because
the `toolchain:deploy_log` feed does not exist at this pin [measured]. The pattern is generalised, not invented.

Abstention is reported separately from error, always. A `null` cell is never counted as a false negative.

**What this record does not decide.** Two things, named so they are not read into it:

- **Source asymmetry is a separate, untaken decision.** `commits` beats `docs` on every facet (`toolchain` 1.00
  vs 0.50; `change` 0.80 vs 0.50; `binary_change` finds nothing at all from `docs`) and `union` is *worse than
  `commits` alone* on `toolchain` and `change`, because unioning a precise source with an imprecise one imports
  the imprecision [measured, F39 §3]. Which source the engine trusts is a decision this record does not take; it
  is registered as an arm of **S-FACET** (ADR-0012), whose named consumer is `facets.toml`'s `source`/`deriver`
  fields.
- **ADR-0079 is untouched and stays `proposed`.** Arm D measures whether a *derived* context vector can drive
  selection; 0079 asks whether an *LLM-emitted* one can. F39 narrows the options without deciding among them, and
  0079 forbids being amended into a decision.

## Considered options

### Option — anchor the existing regex and score it again
Add word boundaries and drop diff lines, keeping paths only. This is S-FACET arm 2 and it is worth running as a
control, but it is not a candidate for the decision: it removes the `coauthor` (8 hits) and ARIA-`role` (2) hits
and leaves *authored* (40) and design-`token` (36) untouched [measured, F39 §2]. Rejected as the answer,
retained as the arm that closes the argument.

### Option — add more keywords
Rejected. Every measured precision/recall curve on this task family moves precision and recall in opposite
directions, so adding keywords to fix false negatives predictably reintroduces false positives [documented, peer,
`2026-09-09-change-classification.md` §2, §4.5].

### Option — a confidence score on the existing rule
Rejected. More categories reduce agreement rather than increase it — binary kappa 0.60 against 4-category 0.20
[documented, peer]. Abstain-or-answer, never "how auth-related, 1–5".

### Option — buy a static-analysis platform (CodeQL, Semgrep, a call graph)
Rejected. CodeQL costs $30/committer/month plus a maintained database and publishes **no precision or recall as a
classifier of anything**; plain Semgrep's measured numbers on adjacent tasks run 0.57%–23.8% recall; a call graph
is priced and built for continuous whole-dependency triage and no source was found using one for first-party
domain tagging. Each relocates the auth-vocabulary problem into a different DSL [documented, 1P/vendor/preprint,
`2026-09-09-change-classification.md` §4.5].

### Option — train a classifier on the corpus
Rejected. The nearest measured analogue needed 2,715 hand-labelled commits to reach 80% precision / 43% recall
[documented, peer]. Twenty items is three orders short.

### Option — do nothing until S-FACET reports
Rejected. The registry is currently emitting confident booleans over empty diffs, and AC-071 gates
`source = "llm"` only — so the semantic facets are `source = "derived"`, therefore outside the block, therefore
**ungated while measurably wrong**. The abstention half of this record is buildable, small, and independent of
which deriver eventually wins; withholding it leaves the defect live for the sake of tidiness.

## Consequences

**Positive**
- Rule 2 becomes enforceable. "A computed fact" was satisfiable by any regex; "a computed fact naming its
  evidence source and provenance class, with a scored record for its deriver" is checkable at configuration time.
- The four empty-diff items stop producing sixteen confident booleans and start producing four questions.
- A `require` row can demand a trust level, so an expensive activity no longer inherits the weakest evidence in
  the vector.
- `surfaces.toml` is a repo-authored artefact, so a team that disagrees with a boundary can fix it by editing a
  glob rather than by arguing with a regex it does not own — and where the repo's directory shape does not follow
  its domains, the boundary map is the forcing function rather than a silent failure.
- ADR-0067's abstention pattern is generalised rather than duplicated, so there is one mechanism, not two.

**Negative**
- **`surfaces.toml` has an unmeasured maintenance cost.** Hours to seed per repository is the easy half; ongoing
  drift is the real cost and **no source in any brief has published it in units comparable to the regex**
  [documented]. It must be audited as a reported output of S-FACET, not assumed.
- **The gate load rises, and by an amount this record cannot state.** Four questions on twenty items is a
  measurement of an oracle-construction exercise, not a forecast for a shipped triage gate, and must not be
  quoted as one. SM-024 has no baseline; S-FACET is what gives it one.
- **Tri-valued booleans widen the predicate surface.** Every `require` row over a boolean facet must now handle
  `null` explicitly, and a row that does not is a lint error rather than a silent `false` — which is the point,
  but it is authoring work that did not exist yesterday.
- **A set-valued `surfaces` facet is a third registry shape**, alongside boolean and ordinal, and ADR-0073 keeps
  those in separate registries. The shape is precedented by `toolchain` and `change`, so no rule breaks, but the
  registry loader must recognise it explicitly rather than by omission.
- **This record removes a facet from the vector without supplying its replacement's numbers.** `surfaces` is
  unscored until S-FACET runs. Until then it may be authored and linted, and it may not key a `require` row on
  the strength of this record alone.

**What this forbids**
- It forbids a facet declaring `source = "derived"` without a named `deriver` and a scored precision record for
  that deriver — the gap that let the semantic facets run ungated while measurably wrong.
- It forbids a lexical scan of free text — file paths and diff lines included — from satisfying PC8 rule 2 for
  `require`-row purposes. Free text was already forbidden as a predicate input by rule 5; this closes the route
  by which it was reaching one through a regex.
- It forbids a derived boolean facet from resolving an absence of evidence to `false`. A fact that cannot be
  computed is `null`, and `null` routes to the triage gate.
- It forbids a repo-wide `auth` boolean keying any `require` row or `applicable_if`, and forbids any activity
  inheriting a security threshold set by whoever authored a pattern.
- It forbids a model proposal keying a `require` row before a human confirms it, and forbids substituting
  unanimous N-seat model agreement for that confirmation — including as a later efficiency proposal. Small-panel
  unanimity is a weaker correctness signal than its arithmetic implies and it fails in the correlated direction
  [documented, preprint, arXiv:2605.29800].
- It forbids treating a `null` as a false negative in any scoring of a facet deriver. A silent `false` and an
  honest `null` are different outcomes and may never be summed.
- It forbids reading this record as deciding the source-asymmetry question or anything in ADR-0079.

## Evidence and confidence

- [measured, F39] Every precision figure, interval and count above is over 140 truth cells at corpus pin
  `47faec9f`, scored by `spikes/p1/score-arm-d-panel.py` at $0 model cost. Intervals are Wilson 95%.
- [measured, F39 §5] Truth-set provenance: 60 cells from a deterministic oracle over git metadata
  (`spikes/p1/oracle-mechanical.py`, independently implemented), 48 from unanimous model panels, 32 from human
  adjudication — Ryan, blind, with the panel's and the script's positions revealed only after he answered.
  **15 of those 32 differ from the script.** Reported as **panel-adjudicated**, never as human-panel.
- [measured, F36] `external_dep` collapsed to a constant, 200/200 `false`, over 400 dispatches, 0 void, $21.37 —
  the first named instance of AC-070's degenerate-distribution clause.
- [measured, F18] The human-gate price: gate→verdict median 10.5 min, n=174, one operator, one machine.
- [documented] The declared-boundary recommendation, the rejected platform options and their published numbers
  are `references/2026-09-09-change-classification.md` §2, §4 and §6a, which carries a live URL per source and
  reached this conclusion from six research axes independently of F39.
- [documented] ADR-0067's `recent_deploy` emitting `null` under `R-recent-deploy-feed-absent` is the live
  precedent this record generalises; the same ADR already recorded "a stale or absent deploy log silently yields
  `recent_deploy = false`" as a known negative consequence.

**Limits, stated rather than discounted.**

1. **No facet has enough positives to clear 0.95 with confidence, even in principle.** `ui` is 7/7 and its lower
   bound is 0.65; roughly 60 positives are needed for a lower bound above 0.95. **METHOD-P §1.4's threshold is
   not reachable at n=20 for any facet, however good the method** — the same class of defect as P1's α rule (F36)
   and P7's criterion 1 (F37). This record therefore rests on `auth` carrying *no signal*, which 11/11 and an
   upper bound of 0.26 does establish, and not on any facet having failed to clear a bar nothing could clear.
2. **39 of the 140 cells rest on unanimous 3-seat model agreement with no human.** A 9-judge panel was measured
   to supply ~2.18 effective independent votes, with accuracy 8–22 points short of independent-vote predictions
   [documented, preprint, arXiv:2605.29800]. The 32 contested cells carry a human; these 39 do not.
3. **The oracle is independent on `toolchain` and on `binary_change`'s Unreal shortcut, and not on the rest.**
   For `change`, git's marker is the ground truth for the binary component, so oracle and script agree there by
   construction.
4. **One repo, one operator, one pin.** Every number is over `47faec9f`, and the colliding vocabulary — a
   spec-driven backlog tool whose documents are *authored* by *coauthors* in an initiative about design *tokens*
   — is this corpus's. A different corpus would collide differently; the claim is that any corpus collides with
   something, and that claim is [inferred], not measured.
5. **`surfaces` is unscored.** This record decides the shape and forbids the old one; S-FACET decides whether the
   new one reaches 0.95, and its kill criterion is a legitimate result either way — if a declared boundary map
   does not clear the bar at an abstention rate the triage gate can absorb, those facets route to ASKED with
   their gate load carried openly by SM-024.
