---
id: "0179"
title: "**Selection chooses the producer by lowest `method_rank`, builds one DAG node per activity, and binds `rigor` for `require` rows; a `toolchain` row's input the step leaves undeclared is supplied from the run's row inputs** (`s8d`'s record R8, D21-73 option (C)): step 4 sorts candidates by `(method_rank, activity_id)`, never by the derived `Ord`, and an equal lowest rank still refuses as `RankTie`; step 5's node is one per activity (one per key on a mixed item, un-keyed kinds on the byte-first key's node) carrying every kind it was chosen for; step 2 evaluates `when` over the item's context with `rigor` bound as step 1's; `package_binaries` supplies `package.binaries` and `rk-cli` supplies `sca_scan`'s `audit_db`/`deny_db` at the tools' default locations. Supersedes `0170` decision 9 in part, at *\"(`s8g` writes this repository's set)\"* only"
status: proposed
date: 2026-09-27
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-73 (*"`s8d` fixes items 1, 2 and 3 under a new record written `proposed` (it flips at `s8d`'s close under D21-64 ruling 10): step 4 chooses by lowest `method_rank` (tie broken as the record states, measured, never by the derived `Ord`); step 5's identity is the activity id (one node per activity, all its provided kinds); step 2 binds the run's `rigor` for `require` rows. Each red-first, with mutants."* and *"`s8d` supplies item 5 (the `sca_scan` `audit_db`/`deny_db` defaults and `package`'s `binaries`) through the resolver or a declared registry value, stated in the same record or R2 as fits."*) and D21-64 ruling 10 (this record's flip condition); runbook/phase-1/s8d-step-grain-and-resolver.md §8d.9 (P8's measured table and the F2-D2173 re-measurement) and §8d.10 (the F2-D2173 tests and mutants); references/2026-09-08-acceptance-criteria.md AC-046, AC-047, AC-049 and AC-078; references/adr/0056 (*"The rank rule is unchanged: among admissible producers of a kind, the lowest `method_rank` wins."*), references/adr/0073 (keyed expansion, one producer per toolchain instance), references/adr/0093 decision 1 (`rigor` is a `kernel` facet) and references/adr/0170 decisions 8 and 9 and its evidence bullet on the handlers' input names; crates/rk-kernel/src/select/step2.rs, step4.rs and step5.rs, crates/rk-kernel/src/runtime/step.rs (`ToolRouting`, `run_tool_step`), crates/rk-kernel/src/workflow/memo.rs (`DECLARATION_INPUTS`, `VerbRegistry::row_inputs_for`), crates/rk-cli/src/verbs/run.rs (`tool_routing`, `advisory_db_row_inputs`) and .redkiln/registry.toml's `package_binaries` row, as built 2026-09-27 over `4e0667a`
supersedes: ["0170"]
superseded_by: []
related: ["0024", "0032", "0056", "0073", "0089", "0092", "0093", "0132", "0142", "0170", "0173"]
evidence: []
consumers: []
---

## Context and problem

`s8d` §8d.9 claims that the shipped pack walks 11 of the fix lane's activities and 9 of the story lane's under a
scripted rig, and Checkpoint 8d (6a) floors those counts. Package P8 built the two `lane_walk` tests, measured
`LANE_WALK fix walked=4 refused=6` and `LANE_WALK story walked=3 refused=5`, and traced every refusal. D21-73 lists
five causes. Three are defects in `s7b`'s selection code:

1. **Step 4 picked the alphabetically first producer.** `select::step4::Candidate` derives `Ord` over its fields in
   declaration order, `(activity_id, method_rank)`, and step 4 called `candidates.sort()` and took `candidates[0]`.
   That contradicts the function's own doc (*"The producer with the lowest `method_rank`"*), AC-046 (*"THEN the
   producer with the lowest `method_rank` is selected"*) and ADR-0056 (*"The rank rule is unchanged: among admissible
   producers of a kind, the lowest `method_rank` wins"*). P8's probe read `chosen=Some(("bisect", 30)) beaten=[("differential-debugging", 20), ("root-cause", 10)]`
   for the fix lane's `RootCause`. The tie check was also wrong: it compared each candidate's rank to the FIRST id's,
   so a real tie at the lowest rank went unseen whenever a higher-ranked id sorted first. The AC-046 unit test did not
   see either, because its winner (`verdict-cheap`) also sorts before its loser (`verdict-dear`).
2. **Step 5 kept two nodes with one id.** It built one `DagNode` per (instance, producer) and deduplicated with the
   derived equality, which compares the instance too. An activity chosen for two required kinds (the shipped
   `tdd-red-green-refactor` provides `Changeset` and `ImplReport`) kept two nodes with one id; `layers` keys its
   in-degree map by id, counted the id once, and reported AC-078's cycle with an empty path.
3. **Step 2 bound no `rigor`.** It evaluated every `require` row's `when` over `item.context()`. `facets.toml`
   declares `rigor` a `kernel` facet (*"a lookup over policy.<class>.stages"*, ADR-0093 decision 1), so the item never
   carries it, and every row keyed on `rigor` refused as `unknown field "rigor"` (the story lane's `implement`, both
   lanes' `closeout`).

The fifth is a supply gap. The shipped `sca_scan` row declares `inputs = ["audit_db", "deny_db"]` and `package`
declares `["sha", "binaries"]`; no catalog step declares `audit_db`, `deny_db` or `binaries`, and the executor refuses
the row before the port naming the first missing input. ADR-0170's evidence states the handlers' input names and
that *"the default locations are the resolver's to supply (`s8d`)"*; its decision 9 admits a `package_binaries`
declaration row and says *"`s8g` writes this repository's set"*. No `s8d` section built either. The fourth cause
(`confirmation-test`'s emit fields, the `lint` port's missing output, `bisect`'s missing registry row) is owed to
later stages by D21-73 and is not this record's.

D21-73 took option (C): `s8d` fixes items 1-3 under a new record written `proposed` and supplies item 5 *"through the
resolver or a declared registry value, stated in the same record or R2 as fits"*. This is that record. R2
(ADR-0173) decides no rule for a `toolchain` row's inputs beyond the step's own resolved `inputs`, so item 5 is
stated here.

## Decision drivers

- **The rank rule is not reopened.** ADR-0056 and AC-046 already state it. The code is brought to the records; no
  record's rank rule moves, and no `method_rank` moves.
- **There is still no tie-break** (ADR-0056's forbids; AC-047). D21-73 asks for the tie to be *"broken as the record
  states, measured, never by the derived `Ord`"*. The record's statement is that an equal lowest rank refuses.
- **AC-049's identity stands.** A mixed item needs one instance of a keyed kind per toolchain, each produced once, so
  one node per activity cannot mean one node for all keys.
- **The kernel reads no environment** (AC-041 for selection; AC-217 for the adapter). A default database location
  under the user's cargo home is computed where the environment is captured, in `rk-cli`.
- **A step's own declared input is the step author's statement.** A supplied default never overrides it.

## Considered options

### Option — Fix the three defects in place, and supply the row inputs through a routing table the composition root fills (taken)

As D21-73 ruled. Each defect is fixed where it lives. Item 5 uses one mechanism, the routing's row inputs, filled
from two sources: a declaration row (`package_binaries`) and a computed default (`sca_scan`'s databases).

### Option — Add `inputs` to the shipped `sca-scan` and `build-package` steps

Declare `audit_db`, `deny_db` and `binaries` on each step. Rejected: it bumps both activities under ADR-0072 and
bakes a machine's cargo-home path, or a literal binary list, into pack content that every repository shares. The
registry row already declares the inputs. What is missing is who supplies them.

### Option — Declaration rows for the two databases in `[toolchain.rust]`

Add `audit_db`/`deny_db` declaration rows beside `setup_projects`. Rejected: ADR-0170 decision 9 closes the
declaration set at three (`setup_projects`, `package_binaries`, `claims`), and Checkpoint 8c (11)'s key map refuses
any other key. A checked-in absolute path is also one machine's path.

### Option — Tie-break an equal lowest rank by id

Rejected: ADR-0056's forbids and AC-047's *"a tie is a pack validation error, never a runtime choice"*.

## Decision

**1. Step 4 chooses the producer with the lowest `method_rank`.** Candidates are sorted by `(method_rank,
activity_id)`, explicitly. The derived `Ord` on `Candidate` is never used to choose. When two or more candidates
share the lowest rank, step 4 returns `LoadError::RankTie` naming them in byte order, as `rank_lint` does at load.
**There is no tie-break.** The `activity_id` second key only orders the `beaten` list (AC-046's *"names the rank of
every candidate it beat"*) deterministically, lowest rank first. It never decides a winner. Measured by
`producer::step4_picks_the_lowest_rank_not_the_first_id` (ids in the reverse of their ranks) and
`producer::a_tie_at_the_lowest_rank_is_refused_whatever_the_ids_sort_to` (a tie at rank 5 while a rank-30 id sorts
first).

**2. Step 5 builds one node per activity, carrying every required kind it was chosen for.** Per activity:
- with no keyed instance, one node, id `activity_id`;
- with keyed instances, one node per key, id `activity_id[key]` (AC-049), each carrying that key's instances. The
  activity's **un-keyed** instances ride the node of its **byte-first key**, so the activity is not dispatched once
  per key and again for no key. On a one-toolchain item this is exactly one node per activity.

`DagNode` gains `instances` (every instance it carries, in byte order of `(kind, key)`); `instance` stays, the first
of them. An edge is drawn from a node for any kind it carries, and a member of `R` is reachable when any node carries
its `(kind, key)`. `dag_assembled` writes `instances` only for a node carrying more than one, so every graph that
could be assembled before this record serialises byte-identically to its recorded form (AC-042's replay compares
bytes). Measured by `dag::one_activity_providing_two_required_kinds_is_one_node`.

**3. Step 2 binds `rigor`.** A `require` row's `when` is evaluated over the item's context with `rigor` bound as the
run's rigor at the stage: step 1's, the pack's `policy` value or the item's logged override. It is bound by
`rigor::with_rigor`, the one place the ordinal is built, **after** the item's facets, so a stale facet named `rigor`
cannot displace it (the executor's order, ADR-0142). Measured by
`selection::a_require_row_keyed_on_rigor_reads_the_stages_rigor`, at the pack's `recommended` and under a
`mandatory` override.

**4. A `toolchain` row's input the step leaves undeclared is supplied from the run's row inputs.**
- `runtime::step::ToolRouting` carries row inputs, verb → input → value. In `run_tool_step`, for a `toolchain`-route
  row only, each declared row input missing from the step's resolved `inputs` is taken from them. **A step that
  declares the input keeps its own value.** An input neither declared nor supplied refuses by name, before the port,
  as before. `port` rows and R5's review routes take no row inputs.
- **Declaration rows** supply row inputs through `workflow::memo::DECLARATION_INPUTS`, which has one entry:
  `package_binaries` → `package.binaries`. `VerbRegistry::row_inputs_for(toolchain)` reads it, and `rk-cli`'s
  `tool_routing` binds it for the chosen adapter's toolchain. `setup_projects` is not mapped, because `setup` runs at
  worktree creation (`s8d-ii`'s) and no catalog step names it. `claims` is not an input.
- **The shipped `[toolchain.rust]` gains `package_binaries = ["redkiln", "redkiln-migrate"]`** with its `source`:
  the workspace's release `[[bin]]` targets, with the six fixture `[[bin]]`s named as excluded (AC-441; `s8g` §8g.6's
  set).
- **`sca_scan`'s databases** are supplied by `rk-cli`'s `run::advisory_db_row_inputs`, for the Rust adapter only:
  `audit_db` = `<CARGO_HOME>/advisory-db` and `deny_db` = `<CARGO_HOME>/advisory-dbs`, each tool's own default
  location. `CARGO_HOME` is read from the captured environment, or taken as `<home>/.cargo` when it is unset, with
  `USERPROFILE` on Windows and `HOME` elsewhere (cargo's own rule). With neither, nothing is bound and the step refuses
  by name. The adapter's own checks stand: it reads each database's `HEAD` before and after, and refuses a `deny_db`
  directory that does not hold exactly one child (ADR-0170's D21-62 C).
- Measured by `tool_verb_routing::a_row_input_the_step_leaves_undeclared_is_supplied_by_the_routing`,
  `verb_registry::the_package_binaries_declaration_supplies_the_package_rows_binaries` and `rk-cli`
  `adapter_selection::the_composition_root_binds_the_row_inputs_no_step_declares`.

**5. What is superseded, in part and quoted.** ADR-0170 decision 9: *"`package_binaries` (`s8g` writes this
repository's set)"*, as to **who writes the row** only. D21-73 rules that `s8d` supplies `package`'s `binaries`, so
this record writes the row. The set is the one `s8g` §8g.6 names. `s8g` cites the row and asserts its exact set; it
changes the set only by record. Every other sentence of decision 9 stands, including its three-row declaration set
and *"Adding a row is a record's act"* (this record is that act). ADR-0170 stays `accepted`, and its `superseded_by`
gains `0179`. ADR-0170's evidence sentence *"the default locations are the resolver's to supply (`s8d`)"* is
**discharged** by clause 4, not superseded.

**6. What this record does not decide.** It does not move a `method_rank`, edit a catalog activity or add a
registry verb. It does not decide D21-73 item 4 (owed to `s8g`/`s8h`). It does not decide the two causes that remain
after the re-measure (§8d.9's F2-D2173 line). One is `tdd-red-green-refactor`'s `"affected"` selection inside
`Changeset`'s own producer, which is R2's. The other is `docs-update`'s optional self-requirement, which puts a
self-edge in step 5's graph. It does not set Checkpoint 8d (6a)'s floors, which D21-73 assigns to a ledger line.

## Consequences

**Positive**

- The fix lane's `analysis` selects `root-cause` (rank 10) and the story's `planning` selects `t-shirt-sizing`, as
  §8d.9 first claimed. The fix lane's `implement` selection assembles, and both lanes' `closeout` and the story's
  `implement` `require` rows evaluate.
- `sca-scan` and `build-package` reach the port on both lanes.
- Re-measured: **`LANE_WALK fix walked=9 refused=6`** and **`LANE_WALK story walked=6 refused=5`** (from 4/6 and 3/5).
- A `dag_assembled` log recorded before this record replays byte-identically.

**Negative**

- Step 4's winner changes wherever the byte order and the rank order disagreed. A selection recorded before this
  record can therefore re-derive to a different producer. That is the defect's correction, not a regression, but a
  replay of such a log now diverges and names the node.
- The fix exposed two causes no earlier run reached (clause 6). The story lane's `implement` now refuses at selection
  on `docs-update`'s self-edge, where it refused on `rigor` before. The refusal names `change-summary[rust]`, a node
  downstream of the loop, because `layers`' path walk starts at the lowest surviving id.
- On a mixed item, an activity's un-keyed kinds ride one key's node. That is a stated rule, not a derived one.
- `package_binaries` is written one stage before the stage that planned it (`s8g`), and `s8g` §8g.6 takes a receiver
  note.
- The databases' default is `rk-cli`'s knowledge of two tools' conventions. A tool that moves its default moves this
  function.

**What this forbids**

- Forbids choosing a producer by the derived `Ord` on `Candidate`, or by any order in which `activity_id` precedes
  `method_rank`.
- Forbids resolving an equal lowest `method_rank` by id, declaration order or file order at runtime. It refuses as
  `RankTie`.
- Forbids two `DagNode`s with one id, and forbids more than one node for one activity on a one-toolchain item.
- Forbids evaluating a `require` row's `when` without `rigor` bound, or with a facet named `rigor` displacing step 1's
  value.
- Forbids a supplied row input overriding an input the step declares, and forbids supplying a row input to a `port`
  row or a review route.
- Forbids `rk-kernel` reading the environment to find a default location. The composition root computes it.
- Forbids reading this record as moving ADR-0170's declaration set, its warrant rules or its adapter checks.

## Evidence and confidence

- [documented, D21-73] the ruling, quoted in `source`, and P8's measured causes it lists.
- [documented, `references/adr/0056`; AC-046; AC-047] the rank rule and the no-tie-break rule this record conforms
  the code to. [documented, `references/adr/0170` decision 9 and its evidence] the sentence superseded in part and the
  sentence discharged.
- [measured 2026-09-27 at `s8d`'s build, package F2-D2173, through the no-dispatch helper] each clause's test was run
  RED for its stated reason before its code (§8d.10's F2-D2173 note quotes each red line), then GREEN. Thirteen
  mutants (MF2a-MF2m), each restored and sha256-checked, were all KILLED.
- [measured 2026-09-27] `lane_walk::` re-measured: fix `walked=9 refused=6`, story `walked=6 refused=5`. A probe
  (not built) with step 5 drawing no self-edge read the story lane `walked=7 refused=7` and left the fix lane
  unchanged.
- [measured 2026-09-27] `lattice::p6_lattice_over_the_shipped_pack` stayed green, so `SHIPPED_UNSAT` (1 296) and
  `SHIPPED_DEAD` (56) did not move. The P6 lattice port chooses producers by its own `(rank, id)` sort
  (`crates/rk-kernel/src/lattice.rs`), not through `select::step4`.
- **What this evidence does not license.** It does not license claiming that either lane walks end to end, or that
  the floors 11 and 9 are met. It does not license any value for the two databases on a machine whose cargo home
  holds no advisory database: the adapter refuses that by name.
