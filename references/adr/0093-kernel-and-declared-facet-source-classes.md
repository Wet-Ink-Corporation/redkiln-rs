---
id: "0093"
title: Kernel projections and the declared surfaces map are their own facet source classes; AC-520 gates only what a spike can score
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-001 (§3.1); spikes/p9/facets.toml at pin 47faec9f; spikes/F39-DRAFT.md
supersedes: ["0089"]
superseded_by: []
related: ["0008", "0012", "0030", "0060", "0061", "0067", "0081"]
evidence: ["PIA-001", "F39", "F36", "F35"]
consumers: []
---

## Context and problem

ADR-0089 decision 1 says "no facet may declare `source = "derived"` without a scored record for that deriver", and
AC-520 — the lint Step 7b builds — makes that a load-time failure: a pack does not load while any derived facet lacks a
scored precision record over a pinned corpus. At pin `47faec9f`, `spikes/p9/facets.toml` declares **eleven** facets
`source = "derived"`: `stage`, `rigor`, `toolchain`, `change`, `role`, `cost`, `auth`, `data_migration`, `ui`,
`flag_exists`, `recent_deploy` [measured]. Runbook Checkpoint 4a asserts that the derived count and the `deriver` count
are "both 11".

Four of the eleven are not facts about the item at all. `stage` is the FSM's current state; `rigor` is
`policy.<class>.stages`'s value for that stage; `role` is the card's role; `cost` is the activity's cost class. They are
**projections the kernel computes from its own state**, with no corpus to score them against and no precision to
report — a spike that "scored" `stage` would be measuring whether the FSM knows what state it is in [measured,
PIA-001]. `recent_deploy` is derived from a `toolchain:deploy_log` feed that does not exist at the pin, so ADR-0067's
rule `R-recent-deploy-feed-absent` emits `null` on every item and F39 could not score it (NOT MEASURABLE). And
`surfaces`, the set-valued facet ADR-0089 decision 2 introduces in place of `auth`, is read from a checked-in
`surfaces.toml` — a **declared** fact whose correctness is the repository's, not a deriver's — and S-FACET, the only
spike registered against it, has no `surfaces` label column to score it with [measured, PIA-084].

So, as written, no Phase-1 pack loads: AC-520 demands scored records for five facets no registered spike can produce
[measured]. The audit found no runbook step or spike supplying a passing route, and the register's own residual-risk
bullet (:1287) already warns that the gate's honest effect is "to fail load for the derived facets". The alternatives
are to loosen AC-520 — the collapse AC-071's residual-risk sentence predicts — or to name the classes ADR-0089's
provenance ladder was missing.

This record supersedes ADR-0089 **in part**: decision 1's provenance classes gain two members, and decision 2's
`surfaces` facet is placed in one of them. Decisions 1's deriver rule for genuinely derived facets, decision 2's
withdrawal of `auth`, decision 3's tri-valued booleans and every forbids bullet stand; ADR-0089 stays `accepted`.

## Decision drivers

- AC-520 must stay a fail-closed gate over things a spike can score, or it is not a gate.
- A projection of kernel state is not evidence about the item; scoring it is a category error, and exempting it by
  name inside the lint is a quiet narrowing — the register's :1162 route ("a registry change that moves a
  deterministic projection out of `source = "derived"` altogether, never a quiet narrowing") is the honest one.
- `surfaces` is declared by the repository, not derived from it; its provenance class already exists in ADR-0089's
  ladder (DECLARED) and only its `source` token was missing.
- ADR-0008 scopes S-BIN to "(type design)" of `ChangeSummary`; the runbook's Step 3c reads S-BIN as gating every
  Phase-1 commit, which is the wider reading the design's own Phase-1 Enter/May-not columns contradict.

## Decision

**1. `facets.toml`'s `source` field gains two closed values.** The full set is `human | llm | derived | kernel |
declared`.

- **`kernel`** — a projection of the kernel's own state: `stage`, `rigor`, `role`, `cost`. No `deriver`, no scored
  record, no S-FACET arm. AC-072 recognises the token by name; AC-520 does not gate it; a `require` row may key on it
  freely (it already does — every `require` row keys on `stage`).
- **`declared`** — a fact read from a checked-in map the repository owns: `surfaces` (from `surfaces.toml`, per
  ADR-0089 decision 2). It carries provenance `declared` on the value, is unscored, and **may not key a `require`
  row** until S-FACET scores a declared boundary map on a corpus with a labelled `surfaces` column — that half of
  ADR-0089's decision 2 is unchanged.

**2. Six facets remain `source = "derived"` and every one has a record at pin `47faec9f`.** `toolchain` (precision
0.73 [0.48, 0.89], 15 positives), `change` (0.72 [0.54, 0.85], 29), `ui` (1.00 [0.65, 1.00], 7), `flag_exists`
(0.00 [0.00, 0.66], 2) [measured, F39]; `data_migration` — **NOT MEASURABLE** (0 positives on 20 items, all three
sources) [measured, F39 §4]; `recent_deploy` — **NOT MEASURABLE** (`toolchain:deploy_log` absent at the pin;
`deriver = R-recent-deploy-feed-absent`) [measured, F39; ADR-0067]. AC-520 requires "a record that EXISTS, IS PINNED
AND IS READ"; a NOT MEASURABLE record satisfies it and, per ADR-0089, the facet may key no `require` row. The
runbook's Checkpoint 4a therefore asserts **six** derived facets and six records — reconciled by name — not eleven.

**3. S-FACET's gating scope is the pack rows keyed on scored derived facets**, not Phase-1 entry. Its consumer stays
`facets.toml`'s `source`/`deriver` fields; its "Gates Phase 1?" cell reads "the `require` rows keyed on `ui`,
`flag_exists`, `data_migration` and `surfaces`; not Phase-1 entry". Step 3c's exit-(c) table carries an S-FACET row
stating that branch.

**4. S-BIN's gate is scoped to the `ChangeSummary` type, exactly as ADR-0008 words it.** S-BIN blocks the design of
`ChangeSummary` (AC-497, AC-356) and the Toolchain port's `change_summary` method body; it does not block Phase-1
commits, `rk-types`, `rk-store`, `rk-git`, the kernel or the adapters. The design's §10.2 Phase-1 Enter column reads
"(a), (b), (d) and the non-S-BIN half of (c)"; its May-not column already presupposes this.

**5. The shipped catalog's rigor family is repaired by ceiling `bisect`** — recorded here because it is the other
half of the same load-time failure (ADR-0081's lint rejects the catalog as shipped, F37): `bisect.toml` gains
`max_rigor = "recommended"`, so escalation to `mandatory` with `has_repro` selects a full activity and the family is
monotone. This is P7-CEIL's ceiling branch; adding adversary and policy steps to `bisect` remains a shipped-activity
edit under ADR-0072 for a later record. Design §10.5's P7-CEIL row moves to "DECIDED — ceiling; YES for the catalog as
shipped".

## Considered options

### Option — schedule S-FACET in Phase 0 and keep all eleven derived
Rejected: S-FACET cannot score a kernel projection under any design, so an exemption would still be needed; this
only delays the re-class and puts a $0-but-unrunnable spike on Phase-1's entry.

### Option — loosen AC-520 to "derived facets a `require` row keys on"
Rejected: the gate would stop policing the registry and police only usage — the collapse the register's own
residual-risk sentence warns of, and the quiet narrowing :1162 forbids.

### Option — exempt the four projections by name inside AC-520
Rejected: an exemption list inside a lint is the same quiet narrowing with worse discoverability; a `source` class is
visible in the registry, linted by AC-072, and reads as what it is.

### Option — make `surfaces` `human`
Rejected: `surfaces.toml` is authored once per repository and read mechanically; it is not a per-item triage answer,
and calling it `human` would route every item through AC-074's decision memory for a value that never changes.

### Option — add the adversary and policy steps to `bisect` (P7-CEIL's other branch)
Rejected for Phase 1 on cost: a versioned shipped-activity edit plus a re-run of P7's arms, against a one-line ceiling
that makes the shipped catalog pass the lint today at $0. Retained as the Phase-2 repair.

## Consequences

**Positive**
- A Phase-1 pack loads on a path the runbook actually provides, and AC-520 stays fail-closed over exactly the
  facets a spike can score.
- The registry says what each facet is: a projection, a declared map, a measured derivation, a triage answer.
- Checkpoint 4a's count becomes a reconciliation by name rather than a magic number.
- The catalog passes ADR-0081's lint as shipped.

**Negative**
- Five values in `source` where ADR-0030 had three; AC-072's shape list and the P9 lint's R-vocabulary grow.
- `surfaces` is authorable and unscored for the whole of Phase 1 unless S-FACET is re-sized on a corpus with a
  `surfaces` label column (PIA-084's re-registration).
- Ceiling `bisect` means a `mandatory`-rigor item with a repro never gets a bisection activity in Phase 1.

**What this forbids**
- Forbids a `require` row keying on a `declared` facet before S-FACET scores a declared boundary map.
- Forbids a `kernel`-class facet carrying a `deriver` or claiming a precision record.
- Forbids reading a NOT MEASURABLE record as a pass or a fail, and forbids a `require` row keying on
  `data_migration` or `recent_deploy` while their records say NOT MEASURABLE.
- Forbids citing S-BIN as a block on any Phase-1 commit outside `ChangeSummary`'s type design and the Toolchain port's
  `change_summary` body.
- Forbids a `bisect` activity without a `max_rigor` ceiling while it lacks adversary and policy steps.

## Evidence and confidence

- [measured, `spikes/p9/facets.toml` at 47faec9f] eleven `source = "derived"` facets, seven `llm`, two `human`.
- [measured, F39] the four precision records and the `data_migration` NOT MEASURABLE result; intervals Wilson 95%.
- [measured, F39; documented, ADR-0067] `recent_deploy` emitted `null` on every item under `R-recent-deploy-feed-absent`.
- [measured, F37; documented, ADR-0081] the shipped catalog's `bisect` rung is non-monotone; the lattice re-run costs
  27 s [measured, F35].
- [measured, PIA-001] no runbook step or registered spike supplies a passing route for AC-520 as written; Step 3c
  checks exit (c) against S-BIN alone; §10.2's Enter and May-not columns contradict each other.
- **What this evidence does not license.** It does not license claiming any derived facet clears METHOD-P §1.4's
  0.95 bar — none does and none can at n=20 (F39 Limits 1; 73 all-correct positives are needed, PIA-073). It does not
  license keying a `require` row on `surfaces`, `data_migration` or `recent_deploy`. It does not decide source
  asymmetry (ADR-0089's open item) or anything in ADR-0079.
