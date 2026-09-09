---
id: "0085"
title: Documentation and traceability conventions for the design phase
status: accepted
date: 2026-09-08
legacy_id: none
source: references/adr/README.md; references/2026-09-07-design-recommendation.md §5; references/2026-09-08-evidence-augmentation.md §3 (decision records), §4 recs 19–20, §5
supersedes: []
superseded_by: []
related: ["0012", "0019", "0075"]
evidence: []
consumers: []
---

## Context and problem

The design phase produces four kinds of durable text — decision records, acceptance criteria, success metrics, and open unknowns — and until now only one of them had a stated convention. The ADR set has a format (MADR-lite, eleven frontmatter keys, six mandatory sections, a required **What this forbids** block, monotonic 4-digit numbering) because 73 records were retrofitted against one. Acceptance criteria have a *type* (`{ id, criterion, gwt: Option<Gwt>, verification: Option<text> }`, design-recommendation §5, recorded as 0075) but no authoring convention: the migration census found 19 of 153 shipped-and-reviewed specs writing criteria with no GIVEN/WHEN/THEN and no verification note at all. Unknowns have a rule (0012: carried as named spikes with a named consumer, never as prose) that was stated for one document's §8 and never generalised. Success metrics have nothing.

Worse, the three identifier families do not point at each other. An ADR's `consumers` key exists precisely to record which criteria a decision feeds and is `[]` on all 73 records, so today a decision cannot be traced forward to what it constrains and a criterion cannot be traced back to the decision that produced it. Without that trace, a superseded decision leaves its consumers silently stale — which is the failure mode the immutability rule was adopted to prevent.

This record fixes the conventions for the design phase as a set, so the four kinds of text share one identifier scheme and one trace.

## Decision drivers

- The ADR format is already settled by 73 records and a normative README; the cost of restating it is near zero and the cost of leaving it undocumented outside that README is a second, divergent convention appearing the first time someone writes a record elsewhere.
- The corpus already demonstrates the failure the convention prevents: 19/153 criteria with no verification note, and `consumers: []` on every ADR.
- EARS single-trigger phrasing constrains a requirement to one measurable trigger–response pair and is closer to machine-checkable than Gherkin or plain "shall"; Gherkin is machine-checkable only once wired to a step-definition harness (evidence-augmentation §3, §4 rec 20). The two suit different criteria, which means the AC type must represent both.
- A criterion that only a human can judge is not made checkable by being written as a boolean; it is made *falsely* checkable, which is worse, because a lint then passes it.
- ADR immutability is this repo's chosen convention, not an inherited standard: MADR permits an accepted record's status to move to `deprecated` in place and permits Consequences and Confirmation to be updated (evidence-augmentation §1 row 11, §4 rec 19). Presenting it as a norm is an overclaim the corpus has already made once.
- Two of the conventions below have **no external norm behind them at all**, and the record must say so rather than borrow authority it does not have (evidence-augmentation §5).

## Considered options

### Option — full MADR

Adopt the MADR 4.0 template unchanged: title, Context and Problem Statement, Decision Drivers, Considered Options, Decision Outcome with Consequences and Confirmation, Pros and Cons of the Options, More Information. Rejected: *Pros and Cons of the Options* duplicates what "one `### Option — <name>` each, with why it lost" already carries, and MADR has no equivalent of the **What this forbids** block, which is the half of a decision that is actually enforceable. Adopting full MADR would mean either losing that block or bolting it on anyway.

### Option — MADR bare/minimal

Use MADR's own documented first cut — title, Context and Problem Statement, Decision Outcome, status/date frontmatter — for lighter decisions (evidence-augmentation §3, decision records). Rejected as the general shape: it drops Considered Options, and the rejected alternatives are exactly what a later reader needs in order not to re-propose them. It is noted here as a legitimate cut for a genuinely minor and reversible decision, but no record in this set uses it and none may use it without this record being superseded.

### Option — Gherkin for every acceptance criterion

Require GIVEN/WHEN/THEN uniformly, dropping the `Option` from `gwt`. Rejected on two grounds: 19 of 153 shipped, reviewed specs do not have it, so the type would reject real artefacts on migration; and Gherkin carries no structural constraint a lint can act on until it is wired to a step-definition harness, so mandating it would buy formatting rather than checkability.

### Option — plain "shall" statements

Write criteria as unstructured shall-statements. Rejected: plain "shall" carries no structural constraint at all, and re-admits precisely the ambiguity classes (vagueness, compound requirements, missing conditions) that a fixed keyword pattern excludes.

### Option — group ADRs into topic folders

Organise records by area rather than a flat monotonic sequence. Rejected: at 100+ records the documented practice is flat `NNNN-slug.md` kept chronological with a generated index, which is what the 73-record set and its legacy-id map already are. Topic folders would make the number non-monotonic and force a hand-maintained index.

### Option — leave unknowns as prose caveats

The status quo 0012 displaced. Rejected for the same reason it was rejected there — a hedge in running text has no consumer, no cost and no gating status, so nothing forces its resolution.

## Decision

**One format, one identifier scheme, one trace, for the whole design phase.**

1. **ADR format is MADR-lite, as `references/adr/README.md` states it, and that README is normative.** Eleven frontmatter keys in the fixed order `id · title · status · date · legacy_id · source · supersedes · superseded_by · related · evidence · consumers`; six mandatory sections (`Context and problem`, `Decision drivers`, `Considered options`, `Decision`, `Consequences`, `Evidence and confidence`), none omissible, with `Decision` permitted before `Considered options`; a required `**What this forbids**` block inside Consequences. Each considered option is one `### Option — <name>` heading and must state why it lost. Every claim in `Evidence and confidence` carries a `[measured]` / `[documented]` / `[inferred]` tag, gives its n, and the section states what the evidence does **not** license.

2. **Numbering is flat, monotonic and permanent.** `NNNN-<kebab-slug>.md`, zero-padded to four digits, assigned once, never reused — including for records later rejected or superseded. The index is generated from frontmatter, never hand-maintained.

3. **Accepted ADRs are immutable — supersede, never edit — and this is redkiln-rs's own convention.** Both sides carry the link: the successor's `supersedes` and the predecessor's `superseded_by`; a one-sided link is a defect. Full supersession moves the predecessor to `superseded`; partial supersession leaves it `accepted` and both bodies record the scope. **This rule is stated as adopted, not inherited.** It matches the `adr-tools -s` / Nygard lineage and this repo's own `validate --kb` accepted-decision-immutability check; MADR itself permits in-place status transitions. Any document presenting it as an industry standard is to be corrected.

4. **Every acceptance criterion names its own verifying instrument.** An AC is written as `AC-###` plus a criterion, plus the named instrument that decides it — a test id or path, a lint rule, a command with an expected exit status, a metric with a threshold, or a named human gate node. "Reviewed and green" is not an instrument (0019). A criterion with no named instrument is incomplete and does not count as authored.

5. **Phrasing follows the instrument, and the AC type carries both shapes.** Where the instrument is a lint or a gate that must **parse** the criterion, use EARS single-trigger phrasing ("While ⟨precondition⟩, the ⟨system⟩ shall ⟨response⟩") — one trigger, one response, no compound clauses. Where the instrument is a runnable feature file that exists or is planned, use GIVEN/WHEN/THEN. Because both are legitimate and neither is universal, `gwt` stays `Option<Gwt>` on the AC type (0075) and `verification` stays optional on the *type*; it is **policy**, not the type, that decides whether a missing instrument blocks a stage.

6. **A criterion only a human can judge is routed to a human gate node, never written as a boolean.** If no machine instrument can decide it, the criterion names a human gate node as its instrument and the graph reaches that gate. Writing such a criterion as a pass/fail assertion is forbidden: it produces a check that a lint will pass and that means nothing, which is strictly worse than an honest gate.

7. **Three identifier families, one scheme.** `AC-###` for an acceptance criterion, scoped and unique within its owning artefact. `SM-###` for a success metric — the measurable statement, with its instrument and threshold, that a set of criteria is meant to move. `ADR-NNNN` for a decision record, unique across the repository. Ids are never reused within their scope.

8. **The trace is bidirectional and both halves are required.** A criterion or metric names the `ADR-NNNN` it derives from; the ADR names it in `consumers`. A one-directional trace is a defect on the same footing as a one-sided supersession link. `consumers` is `[]` across the retrofitted set pending the backfill pass, and that emptiness is a known, tracked debt — it is not the convention.

9. **An unknown is discharged as a named spike with a named consumer, never as prose.** 0012's rule is generalised from one document's §8 to the whole design phase: every open question is a row carrying a spike name, the question, the consumer it feeds, whether it gates the phase, and a cost. A record that depends on an unrun spike carries status `proposed` and names that spike. A hedge in running text is not a tracked unknown.

## Consequences

**Positive**
- One format for every decision, so a record written outside `references/adr/` cannot quietly diverge, and the projection into `.kb/decisions/` at self-hosting inherits an already-conforming shape.
- Naming the instrument makes an AC's checkability visible at authoring time rather than at implementation time, when the census shows it is discovered as an absence (19/153).
- Routing human-judgement criteria to a gate node rather than a boolean removes a whole class of check that passes without meaning.
- The bidirectional trace makes a supersession's blast radius computable: superseding a record lists exactly which criteria and metrics need re-reading.
- Immutability stated as a chosen convention is defensible under audit; stated as an inherited standard it is falsifiable in one fetch of the MADR template.

**Negative**
- Two phrasing grammars (EARS and Gherkin) means two lint paths, and the criterion that selects between them — "does a runnable feature file exist or is one planned" — is a judgement, not a predicate.
- Naming an instrument on every criterion raises authoring cost and will, on some criteria, force an honest "human gate" where a plausible-looking boolean would have been cheaper and would have passed.
- The bidirectional trace is only as good as its enforcement, and no enforcement exists yet: `consumers` is `[]` on all 73 retrofitted records, so the convention currently describes an intent, not a state.
- Two of these conventions have no external validation, so they carry only this repo's own reasoning and will be judged on whether they work here.

**What this forbids**
- Forbids editing an accepted ADR; forbids a one-sided supersession link; forbids reusing a retired number.
- Forbids omitting any of the six sections, the **What this forbids** block, or a `[measured]`/`[documented]`/`[inferred]` tag on an evidence claim.
- Forbids presenting ADR immutability as an inherited industry standard rather than as this repository's chosen convention.
- Forbids an acceptance criterion with no named verifying instrument, and forbids "reviewed and green" as that instrument.
- Forbids writing a criterion only a human can judge as a boolean assertion or a lint-checkable predicate; it must name a human gate node.
- Forbids mandating GIVEN/WHEN/THEN on every criterion, and forbids using it where a lint must parse the criterion instead of a harness running it.
- Forbids a one-directional trace between an `AC-###`/`SM-###` and the `ADR-NNNN` it derives from.
- Forbids recording an open question as a prose hedge; forbids a record standing at `accepted` on the strength of a spike that has not run.
- Forbids citing an external norm as authority for rules 4, 6 or 7 — see Evidence.

## Evidence and confidence

- [documented] MADR 4.0's frontmatter keys (`status`, `date`, `decision-makers`, `consulted`, `informed`) and full section set were re-fetched and match the template exactly; MADR's own bare/minimal cut keeps title, Context and Problem Statement, Decision Outcome and status/date only. The supersession convention as a bidirectional pointer, stamped by `adr-tools -s`, is confirmed. Flat `NNNN-slug.md` with a generated index is community consensus at 100+ records, with no single authoritative source. [evidence-augmentation §2 and §3, decision records; [1P] github.com/adr/madr, [Pr] adr-tools / Log4brains]
- [documented] Immutability is **not** an inherited standard. MADR permits an accepted decision's status to move to `deprecated` in place and permits Consequences/Confirmation to be updated over time. It is the adr-tools/Nygard convention and this repo's own `validate --kb` rule. [evidence-augmentation §1 row 11 and §4 rec 19; [1P] github.com/adr/madr]
- [documented] EARS provenance is Mavin, Wilkinson, Harwood & Novak, IEEE RE'09 pp.317–322. Its fixed keyword patterns constrain a requirement to one measurable trigger–response pair, addressing the vagueness / compound-requirement / missing-condition classes; Gherkin is machine-checkable only once wired to a step-definition harness; plain "shall" carries no structural constraint. [peer, corroborated by practitioner explainers; evidence-augmentation §3 and §4 rec 20]
- [measured] The census behind the AC type: 19 of 153 shipped and reviewed specs write criteria with no GIVEN/WHEN/THEN or no verification note (S-MIG, design-recommendation §5). n=153, one corpus, one operator. This is what makes both fields optional on the type and pushes the requirement into policy.
- **[documented] — the honest provenance of rules 4 and 6, stated plainly.** The external research pass looked for and did not find external support for either. On rule 4: "Is there a documented norm that an acceptance criterion should name its own verifying instrument (test ID, lint rule, metric)? — Nothing first-party or peer-reviewed found; only informal practitioner blog content… **No, currently** — this looks like an emerging pattern. The AC-### + `depends_on`/`tracesTo` convention may be **ahead of** any external norm rather than confirming one. State it as original process design." On the adjacent question behind rule 6: "Is there a convention for phrasing an AC for a deliberately undecided or blocked item so it does not read as an omission? — Nothing found. **No** — a genuine gap in the external literature. Any redkiln practice here is original." [evidence-augmentation §5] **Both rules are original process design taken on this repository's own reasoning. Neither may be presented as adopted practice, and the absence of a contradicting source is not validation** — the pass states its own discipline as "nine agents finding nothing against a claim means 'no current source contradicts it', not 'confirmed'."
- [inferred] The `SM-###` family and the bidirectional `AC`/`SM` ↔ `ADR` trace are this record's own construction. `consumers` exists on all 73 ADRs as the forward half of that trace and is `[]` on every one of them, so the trace has been designed and never populated. No measurement supports the scheme; it is reasoned from the requirement that a supersession's blast radius be computable.
- [documented] The unknowns-as-spikes rule restates and generalises D12 (0012), whose own source row cites no finding, spike or measurement.
- **What this evidence does not license.** It does not license claiming that any of these conventions improves outcomes: none has been measured, and no spike is registered against them. It does not license claiming external validation for the named-instrument rule or the human-gate-routing rule. It does not license treating the EARS/Gherkin split as settled for redkiln — it is a recommendation carried from one peer-reviewed provenance and practitioner corroboration, applied here by judgement, and the shape of `gwt` on the AC type is decided in 0075, not here. It does not license treating the trace as operational: it is a convention with zero enforcement and zero populated instances as of this date.
