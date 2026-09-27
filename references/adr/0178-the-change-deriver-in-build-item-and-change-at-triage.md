---
id: "0178"
title: "**The `change` deriver in `build_item`, and `change` at triage** (`s8d`'s record R6): `R-change-extension-and-numstat-marker`, ported verbatim from `spikes/p1/derive-facets.py`, runs at step (7) over the item's changeset `HEAD..rk-run/<item>` through `rk-git`; an empty range abstains (`null`), never `[]` or `text`; a record-supplied value for a `source = \"derived\"` facet is refused by name. A `change` still `null` at the lane's first stage opens AC-079's `gate_opened { reason: unknown_facet, axis: \"change\" }` once, answered with `facet.change`, and the answer binds `change` as a one-element set, folded from `gate_answered` and never written to the record; a later derived value binds over it and a disagreement is reported. Carries D21-43 N3 and a dated AC-079 reading note. Supersedes nothing"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R6's row), §8d.7, §8d.10 (the four `change_deriver::` names; mutants M17 and M22); spikes/DECISIONS-2026-09-21.md D21-41 (d) (*"wire the `change` deriver"*), D21-43 N3 (*"`null` routes to AC-079's `unknown_facet` gate at triage, and the operator supplies it."*), D21-64 ruling 10 (this record's flip condition); references/2026-09-08-acceptance-criteria.md AC-079 (and its dated reading note, added with this record), AC-520; references/adr/0089, 0093 (decisions 1 and 2), 0103 (decision 5), 0105, 0177 (`facet.change`); spikes/p1/derive-facets.py (`derive`, `RULES`); .redkiln/facets.toml (`[facet.change]`); crates/rk-cli/src/change_deriver.rs, crates/rk-cli/src/verbs/run.rs (`through_step_seven`, `build_item`, `bind_answered_facets`, `open_derived_facet_gates`) and crates/rk-kernel/src/workflow/gate.rs (`derived_facet_gate`, `answered_facet`) at `s8d`'s build (package P7)
supersedes: []
superseded_by: []
related: ["0089", "0093", "0103", "0105", "0177"]
evidence: []
consumers: []
---

## Context and problem

`.redkiln/facets.toml` declares `change` a derived facet:

```toml
[facet.change]
values = ["text", "binary", "generated"]   # set-valued per item
source = "derived"
deriver = "R-change-extension-and-numstat-marker"   # git metadata only: extension table + --numstat's binary marker
```

and ADR-0093 decision 2 records its score: *"`change` (0.72 [0.54, 0.85], 29)"* at pin `47faec9f`. Nothing ran the
deriver. `rk-cli`'s `build_item` *"nulls every non-kernel facet the record omits"* and read a `[facets]` value for any
non-kernel facet, derived ones included, so a record could hand the lane a `change` no deriver computed.

The facet matters early. `minimal-repro` and `root-cause` are tagged `change:text`, and `tags_match` gives an unbound
axis no tag (*"An axis the item does not bind cannot satisfy a constraint on it"*), so both drop out of `analysis` in
silence while `change` is `null`. A fix or story item at `triage` has no changeset, so the deriver can only abstain
there. D21-43 N3 took the route: *"`null` routes to AC-079's `unknown_facet` gate at triage, and the operator supplies
it."* Without it the null is silent: `change` is first read by a tag at step 4, which opens no gate, and the `require`
rows and `applicable_if` expressions that read it (`'text' in change`) are read at `implement` or `verify`, after
`analysis` has already dropped the two activities.

## Decision drivers

- **The absence of evidence never resolves to a value** (ADR-0089; AC-079). An empty range is an abstention.
- **A derived facet's source is its deriver** (ADR-0093 decision 1's classes). A record is not a deriver.
- **The gate is AC-079's, counted once per read facet**, and the answer is the operator's.
- **Nothing is written back to the record.** The record is the item's author's; the journal is the run's.

## Considered options

### Option — Run the deriver at step (7); open the abstention gate at the lane's first stage; bind the answer from the journal (taken)

`s8d` §8d.7 as specified. The composition root runs the deriver where it builds the item, and the kernel decides the
gate row and reads the answer back.

### Option — Default `change` to `text` when the range is empty

Rejected: *"An empty range abstains, never `[]` and never `"text"`"* (§8d.7), and ADR-0089's rule.

### Option — Let a record supply `change`

Rejected: it reads a human value on a derived facet, which ADR-0093 decision 1's classes do not admit, and it would
bypass the scored deriver ADR-0093 decision 2 records.

### Option — Open the gate through the selection's step 2

Rejected for this record: step 2 opens a gate only for a facet a firing row reads, and `change`'s first reader is a
tag at step 4. Widening step 2 to tags would move every selection's `gates()` and its canonical bytes; the root's
first-stage gate reaches the same AC-079 row without moving selection.

## Decision

**1. The deriver runs at step (7), for every facet it derives.** `through_step_seven` runs
`rk_cli::change_deriver::derive` for each facet `facets.toml` declares `source = "derived"` with `deriver =
"R-change-extension-and-numstat-marker"`, over the item's changeset: `HEAD..refs/heads/rk-run/<item>`, read through
`rk-git`'s wrapper (`GitOp::RevParse` for the ref, `GitOp::Log` with `--numstat --no-renames -z --format=`; both reads
on ADR-0105's list). The rule is `spikes/p1/derive-facets.py`'s `derive`, verbatim: `text` if some path is neither
generated nor marked binary; `binary` if `--numstat` marks some path `-\t-` or a path matches the `unreal` table;
`generated` if a path matches the generated table. The two tables are hand-translated from the Python's regular
expressions (the crate carries no regex engine) and pinned by a test. A value is bound as a set. `answer` runs step
(7) too, so it derives as well.

**2. A record-supplied value for a `source = "derived"` facet is refused by name** — R6's reading of ADR-0093 decision
1's source classes: a derived facet is computed over the changeset, never read from the record. The refusal names the
facet and its deriver, exits `3` after the reconcile row, and applies to all six derived facets; no shipped item record
supplies one (measured: `.redkiln/items/*.toml` and `pack-min`'s items carry none).

**3. An empty range abstains.** No `rk-run/<item>` ref, or a ref with no commit past `HEAD`, is `None`, and the facet
stays `with_facet_null`: never `[]`, never `text`. A range that exists and cannot be read is refused by name, not
read as an abstention.

**4. `null` at the lane's first stage opens AC-079's gate, once.** Before `resume`, at the stage that is the pack's
declared `stage_order`'s first (`triage` in both shipped packs), for each wired derived facet still `null`, the kernel
(`workflow::gate::derived_facet_gate`) mints `gate_opened { kind: "unknown_facet:<axis>", reason: "unknown_facet",
axis, tree_hash, deadline, reach, token }` when the fold holds no reach of the kind, or its latest record is void
(AC-252's re-open); an open, answered or expired record opens nothing. The root appends it, and the FSM's first rule
awaits it (ADR-0022), so the run exits `0` at the gate before any selection reads the facet. The kind is the
selection's own spelling, so `answer` resolves its question by kind: `facet.change` (ADR-0177 decision 9). A pack with
no declared order has no first stage and opens nothing here; an item that enters the lane past its first stage with a
`null` `change` is AC-079's ordinary case, gated only where a firing row reads it.

**5. The answer binds `change` as a one-element set.** Before each stage's inputs are built, the latest
`unknown_facet:<axis>` gate answered and not void (`workflow::gate::answered_facet`) binds the still-`null` facet to
`{answer}`, folded from the `gate_answered` row. It is never written to the item record, where a `change` value stays
refused (decision 2), and an answered axis opens no second gate.

**6. The deriver re-runs at every invocation, and a derived value binds over the answer.** Once `rk-run/<item>`
carries commits, step (7) derives a value and it binds: the facet's source is its deriver. A derived value that is not
exactly `{answer}` is reported on stderr, naming both; no row is written for it.

**7. A dated AC-079 reading note.** AC-079's GIVEN reads *"that facet is read by a row that fires"*; the note records
that, for a derived facet whose first reader is a tag, the triage gate opens at the lane's first stage (decision 4),
counted once per facet, and is otherwise AC-079's gate in every respect. It is a reading, not an amendment.

## Consequences

**Positive**

- `change` has its declared source: the scored deriver, or the operator's answer while no changeset exists, and never
  the record or a default.
- A fix item at `triage` no longer loses `minimal-repro` and `root-cause` in silence: the gate asks, and the answer binds
  `change` for every later stage.

**Negative**

- **The other five derived facets have no wired deriver.** `toolchain`, `ui`, `flag_exists`, `data_migration` and
  `recent_deploy` stay `null` unless a later record wires theirs; decision 2 now also refuses a record-supplied value for
  them.
- **The ported tables are hand-translated.** They are pinned by one test's cases, not by a differential run against
  the Python over a corpus.
- **Only the lane's first stage opens the gate.** An item entering at a later stage relies on AC-079's step-2 gate.
- **The binary marker is git's numstat heuristic** (a NUL in the leading bytes), as the Python's was; `rk-git`'s
  `range_diff` binary mark (which also marks non-UTF-8 text) is not used.

**What this forbids**

- Forbids reading a derived facet's value from the item record.
- Forbids binding `[]`, `text` or any default for an empty range.
- Forbids writing the triage answer back to the item record, and forbids a second `unknown_facet` gate for an axis
  whose latest gate is open, answered or expired.
- Forbids an answered value overriding a value the deriver computed without the disagreement being reported.

## Evidence and confidence

- [documented] D21-43 N3, ADR-0093 decisions 1 and 2, AC-079 and `s8d` §8d.7, each quoted above or in `source`.
- [measured 2026-09-26 at `s8d`'s build, through the no-dispatch helper] each test first shown RED, then GREEN:
  - `change_deriver::build_item_derives_change_over_the_recorded_range` — RED *"left: None, right: Some(["binary",
    "generated", "text"])"*;
  - `change_deriver::a_record_supplied_value_for_a_derived_facet_is_refused` — RED *"a record-supplied `change` must be
    refused, never read as the facet's value (M17)"*;
  - `change_deriver::an_empty_range_abstains_and_never_defaults` — RED at its CONTROL, *"a non-empty changeset derives
    a value"*;
  - `change_deriver::a_null_change_at_triage_opens_the_unknown_facet_gate_and_its_answer_binds_the_facet` — RED: no
    gate opened, the lane advanced out of `triage` and refused on its `Story:` trailer.
- [measured, same session] mutant M17 (the refusal skipped) is KILLED by the record-supplied test; M22 (no gate opened)
  by the triage test, at its first assertion (the run did not stop at a gate). Each ran from a Python driver, the file
  restored and its sha256 verified.
- **What this evidence does not license.** No real item's changeset was derived; the fixtures are synthetic
  repositories. This record flips at `s8d`'s close under D21-64 ruling 10, on its own dated ledger line, only if the
  formal Checkpoint 8d passes whole and a refuting round finds no blocker or major.
