---
id: "0192"
title: "**The spec/pack schema fixes D17-3's three spellings as they are read today, and exports `rk_types::AcceptanceCriterion`** (`s8g`'s schema record, R2; D21-42 10a; D21-43 N24 and its integration call on AC-133): `[policy] verification_required = [<stages>]` and `[mutation] feasible, reason` are fixed unchanged, the mutation declaration gains its shipped home in the Rust row of `0170`'s per-toolchain table, and the fixture reader's `[ac.mutation] not_feasible` is superseded by `AcceptanceCriterion`'s serde form; `AcceptanceCriterion` enters `registry::EXPORTED` and retires the kernel-local `SpecAc`; AC-139's default is block (fail-closed), recorded as the human decision and shipped as `verification_required = [\"verify\"]` in both packs; AC-133's judgement-enum registry is built once, here, and `s8h` registers into it; the `verification` gap N24's row opens over agent-authored `Spec`s is named, with its owners `s8h` and `s8j`. Supersedes nothing"
status: proposed
date: 2026-09-29
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (row 10a — *"**One spec/pack schema record** fixes the three spellings and adds `rk_types::AcceptanceCriterion` (with ADR-0104's mutation field) to `EXPORTED`."*), D21-43 (N24 — *"**Block (fail-closed)**, recorded as the human decision AC-139 requires."*; the integration call *"AC-133's judgement-enum registry is built once, by **s8g**. s8h consumes it."*), D21-47 (review findings 5 and 15); runbook/PROGRESS.md §2 D17-3 (*"**OPEN — readings, not decisions.**"*); runbook/phase-1/s8g-verify-and-restored-demands.md §8g.3, §8g.8 (R2's row)
supersedes: []
superseded_by: []
related: ["0019", "0072", "0074", "0084", "0095", "0096", "0104", "0125", "0170", "0191"]
evidence: []
consumers: []
---

## Context and problem

**D17-3 has been OPEN since 2026-09-17.** The tracker reads: *"The verify gate's three key spellings no record fixes
… **OPEN — readings, not decisions.**"*, and it notes that the schema record is also where
*"`rk_types::AcceptanceCriterion` (AC-137, ADR-0104's `mutation:` field), which the tree does NOT carry"* belongs.
The kernel reads a stand-in whose header says so: *"a kernel-local shape, decided 2026-09-17 by the operator: the full
`rk-types` spec type is the migration's, 7d"* (`crates/rk-kernel/src/verify/ledger.rs`). D21-42 10a decides that one
record fixes all three and exports the type.

**Two absences, both measured at `ca8f387` and re-read at `a6349b6`.** The shipped `.redkiln/` declares no `[mutation]`
table anywhere, and AC-337 says *"an absent declaration is a configuration error"*. Neither shipped pack declares
`verification_required`: each `[policy]` holds `stages` and `stage_order` only, so AC-455's refusal cannot fire on
this tree. What the shipped default is belongs to AC-139, a human gate: *"the choice is recorded as a human decision
with its rationale, and the shipped default is whatever that decision names."*

**The operator took AC-139's decision on 2026-09-24.** D21-43 N24 reads *"**Block (fail-closed)**, recorded as the human decision AC-139 requires."*
AC-139's own *Verified by* makes a policy row the right form: *"What IS machine-checked is AC-138: the value is a
policy-table row, so the human decision is a data change."*

**AC-133's registry does not exist.** Its THEN is *"the enum appears in the judgement-enum registry with an explicit
judgement-bearing yes/no AND every enum marked yes has a sibling free-text `rationale` field"*; `grep -rni judgement
crates/rk-types/tests crates/rk-types/src/schema` returned nothing at `ca8f387`, while `s5`'s acceptance set claims
AC-133. D21-43's integration call puts its build here, once.

## Decision drivers

- Nothing on the tree moves but its warrant: each spelling is fixed as it is read today.
- A human decision is recorded, never claimed by a test (AC-139); its value is a data row (AC-138).
- One registry, built once (D21-43), with a completeness test over every enum reachable from `EXPORTED`.
- `s8h`'s `Spec` embeds the criterion type unchanged (D21-41 g: schemas are Rust types in `EXPORTED`).

## Considered options

### Option — fix the three spellings as read, export `AcceptanceCriterion`, ship N24 as a policy row (taken)

Taken: it moves no working key, gives each reading a warrant, and records the human decision as the data change
AC-139's *Verified by* names.

### Option — keep the kernel-local `SpecAc` until `s8h`'s `Spec` lands

Rejected: D21-42 10a puts the type in `EXPORTED` now, and the verify pass (R1, ADR-0191) reads criteria off a
committed artifact whose schema must cross the closed-output boundary. Two shapes would mean two readers.

### Option — encode AC-139's default as a constant in the type

Rejected: AC-138 requires the value be *"computed from the policy table row, not from a constant in the type"*, and a
constant would turn a human decision into code.

## Decision

1. **The three spellings, fixed as read.**

   | spelling | read today by | fixed as |
   | --- | --- | --- |
   | `[policy] verification_required = [<stages>]` | `select/pack.rs` (*"**AC-455's policy row**"*) | the process pack's key, unchanged |
   | `[mutation] feasible = <bool>, reason = <str>` | `rk_ports::toolchain::MutationFeasibility::declared`; the Rust fixture pack | the key, unchanged, with a shipped home (decision 2) |
   | `[[ac]] … [ac.mutation] not_feasible = "<reason>"` | `verify/ledger.rs`'s fixture reader | superseded by `AcceptanceCriterion`'s serde form (decision 3) |

2. **The mutation declaration's shipped home is the Rust row of ADR-0170's per-toolchain table**, with `feasible =
   true` and ADR-0104 decision 1's warrant (D21-42 4a: *"Each value carries its measurement as its warrant"*), so the
   ledger's feasibility input exists on this tree.
   *(dated 2026-09-29, s8g build, package R-D2122-D2123-8g; ruled D21-123, TAKEN PROVISIONALLY under D21-105; an
   in-record note before the flip.)* **Carried out as written.** The row is `.redkiln/registry.toml`'s
   `[toolchain.rust]` `mutation = { value = { feasible = true }, source = "ADR-0104 decision 1 (…); ADR-0192 decision
   2; D21-42 4a" }`, in the only shape the loader admits for a declaration key (`{value, source}`), with no `reason`;
   `rk_kernel::workflow::memo::DECLARATION_ROWS` admits `mutation` (3 → 4), as ADR-0170 decision 9 allows (*"Adding a
   row is a record's act"*). **The reader** is `rk_cli::verbs::run::mutation_feasibility_at`, called at ADR-0191
   decision 2's step (6): it reads the row of the toolchain the pass's adapter serves (`toolchain_of`, as set-up reads
   `setup_projects`, so the kernel never sees the name), and keeps the pack's `[mutation]` table as a second reader.
   Registry only, pack only, or both and equal: that value; **both and different: refused by name**
   (`Exit::ConfigError`, *"mutation feasibility is declared twice and disagrees"*), with no precedence winner;
   neither: AC-337's refusal. **No Node row:** a declaration key cannot carry `unmeasured`, and a Node value would be
   a guess, so `[toolchain.node]` carries a dated comment instead; the row is owed with the first Node self-hosting
   run, and until then a Node lane with criteria refuses by name. **Stated limit:** the row turns a configuration
   refusal into a harness run; with `assertion_flipped = None`, `pinned` is unreachable (AC-334 ARMED), so the stage
   exit still refuses and mints AC-357's gate. AC-337's doctor rule-table row for the missing declaration stays owed.

3. **`rk_types::AcceptanceCriterion`** (AC-137's GIVEN/THEN; ADR-0104 decision 1's field): `id`, `criterion: Text`,
   `gwt: Option<Gwt>` with `Gwt { given, when, then, and: Vec<Text> }`, `verification: Option<Text>`, and `mutation:
   Option<MutationDeclaration>`, where `MutationDeclaration` is `#[serde(tag = "kind")]` over `Declared { patch,
   suite }` and `NotFeasible { reason }` (ADR-0084's discriminated form). A prose-only mutation is refused at ingest
   (ADR-0096). It enters `registry::EXPORTED` (*"Every schema that crosses the closed-output boundary"*) with its
   `exported__AcceptanceCriterion.snap`, accepted once and reviewed as a schema change.

4. **`SpecAc` is retired.** It and its TOML reader leave `rk-kernel`; the ledger, the exit check and
   `admissibility::admissible` read `&[rk_types::AcceptanceCriterion]`. The two text fixtures carrying the old name
   move with it. No code line under `crates/` names `SpecAc` afterwards.

5. **AC-139's default is block, fail-closed** (D21-43 N24), recorded here as the human decision with the operator's
   rationale by quotation. Both shipped packs gain `verification_required = ["verify"]` in their `[policy]` table:
   the row names the verify stage because that is where the pass runs the ledger and the admissibility check (R1,
   decision 2 step 6). AC-139 takes a dated register note naming N24 and this record. `PROGRESS.md` row 5's claim
   that decisions 6 and 7 were executed takes a dated correction in the tracker commit (D15-2's form), and D17-3
   closes there, naming this record.

6. **The `verification` gap, named with its owners and not closed here** (D21-47 review, finding 5). N24's row
   refuses every criterion whose `verification` is `None`, and each lane's `Spec` is agent-authored (`fix-spec` at
   the fix lane's specification stage, `bdd-scenarios` at the story lane's). `AcceptanceCriterion` keeps
   `verification: Option<Text>`, because AC-137's THEN accepts a criterion with none. The owners: **`s8h`**, whose
   `Spec` schema and specifying cards refuse an authored `Spec` without a non-empty `verification`
   (`classify::a_spec_criterion_without_an_authored_verification_is_a_schema_failure`); and **`s8j`**, whose lane
   proofs assert every criterion carries one before verify runs.

7. **`Spec`'s member is `acceptance_criteria`** (D21-47 review, finding 15), because the shipped catalog already reads
   that name (`fix-spec.toml`'s *"`spec.acceptance_criteria.len() > 0`"*; `for_each =
   "artifact:Spec.acceptance_criteria"` in `tdd-red-green-refactor` and `mutation-testing`). `s8h`'s `Spec` embeds
   `AcceptanceCriterion` unchanged, under that member name, and redeclares nothing in it.

8. **AC-133's judgement-enum registry is built here, once**, to AC-133's *Verified by*: a completeness test over every
   enum reachable from `EXPORTED`, the rationale-sibling rule, and a must-fail fixture; a third test keeps the
   registry from naming an enum that is not exported. Every enum reachable from `EXPORTED` at this stage's close is
   registered with a stated yes/no, `Findings`' and `AcceptanceCriterion`'s included. The per-enum classification
   the operator signs is `s8h`'s (AC-133's limit: *"that classification is the human gate AC-156"*), and `s8h` builds
   no second registry.

## Consequences

**Positive**
- D17-3 closes with every spelling warranted; the criterion type crosses the boundary once, snapshotted.
- AC-139's human decision is a data row both packs ship, and AC-138 is re-provable over it.
- AC-133 has an instrument for the first time, and `s5`'s claim is corrected rather than re-claimed.

**Negative**
- Under the block default, a lane whose agent-authored `Spec` leaves one criterion's `verification` empty refuses at
  verify until `s8h`'s schema check and `s8j`'s proof land.
- AC-137's render half (the empty-slot golden render over `s8f`'s renderer and `s8h`'s template) is claimed by no stage
  yet, and is reported upward.

**What this forbids**
- A second criterion type beside `rk_types::AcceptanceCriterion`, and any code line naming `SpecAc`.
- A prose-only mutation declaration admitted at ingest.
- AC-139's default decided by a constant in a type, or changed without a dated human decision.
- A second judgement-enum registry, or a registry entry for an enum that is not exported.
- Renaming `Spec`'s `acceptance_criteria` member, or redeclaring `AcceptanceCriterion`'s fields inside `Spec`.

## Evidence and confidence

**Measured** (2026-09-29, at `178868f`): `SpecAc` appears on 17 `grep -rnw` lines under `crates/`, 15 of them code
lines by Checkpoint 8g (4)'s filter; neither pack's `[policy]` carries `verification_required`. **Documented:**
D21-43 N24's decision text, AC-139's and AC-138's register text. **Confidence:** high; the decisions are the
operator's, and the record fixes readings rather than choosing new shapes.
