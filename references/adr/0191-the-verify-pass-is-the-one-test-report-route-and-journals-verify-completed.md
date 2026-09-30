---
id: "0191"
title: "**The verify pass is the one `TestReport` route, and it journals `verify_completed`** (`s8g`'s verify record, R1; D21-41 c as superseded by D21-42 1a; D21-42 10b, 10c; D21-43 N23, N25; D21-46 R5; D21-47; D21-48 X9, X15): an `rk-cli` pass on ADR-0162's shape, selected by the required kind `TestReport` and never by a stage literal, calls `Toolchain::test` once, whole-suite, over a run-owned worktree `setup` has prepared, and writes one keyed `TestReport` through `rk-store`; `TestReport` becomes `source = \"pass\"` under a third `KindSource`; the four displaced catalog providers and `integration-tests` are re-homed whole to `implement` over one minted test-authoring kind, `AuthoredTests`, with the test-design escalation and `story.toml`'s `IntegrationReport` row moved with them; `migration-dry-run` is AC-347's single named residual; `verify_completed`, a tier-2 item-scoped kind, is journalled by the kernel once per completed `Toolchain::test` port call with its `origin` and `stage`; the verify block of `s7d` §7d.18 is routed id by id; the mutation harness of ADR-0104 decision 1 runs in an ephemeral `<item>.mut` tree. Supersedes `0162`'s patch clause in part, for the verify gate's declared AC mutations alone"
status: proposed
date: 2026-09-29
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (row c), D21-42 (rows 1a — *"**Widened to an outcome-bearing report**"*; 10b — *"**Mint `verify_completed`**: report digest, selection with `basis_reason`, counts and adapter identity"*; 10c — *"**Claimed whole**"*), D21-43 (N23 — *"**Re-homed to the implement stage** as test-authoring activities under ADR-0072. Verify only measures."*; N25), D21-46 (R3, R5), D21-47 (the review's findings 4, 8, 14 and cross-file 19; *"Records are located by the id recorded at write time"*), D21-48 (X9, X15); runbook/phase-1/s8g-verify-and-restored-demands.md §8g.2, §8g.4, §8g.5, §8g.8 (R1's row); runbook/phase-1/s8c-toolchain-port-and-verbs.md §8c.5 (the cold measurement, cited)
supersedes: ["0162", "0132", "0189"]
superseded_by: []
related: ["0019", "0072", "0073", "0096", "0104", "0105", "0109", "0116", "0125", "0131", "0140", "0169", "0170", "0171", "0179", "0180", "0182", "0188", "0189"]
evidence: []
consumers: []
---

## Context and problem

**`bug-standard.toml`'s verify stage demands a `TestReport`, and at `ca8f387` no producer a run can reach writes
one.** The catalog carries four `TestReport` providers — measured at `a6349b6` by `grep -lE '^provides *=.*"TestReport"'
.redkiln/catalog/*.toml`: `unit-tests`, `unit-tests-ue`, `equivalence-boundary` and `property-based-testing` — and
`artifacts.toml` reads *"source = "activity" → some catalog activity provides it"*. As shipped, selection would walk
`unit-tests`, whose `gaps` and `fill` steps are `kind = "agent"` under `when = "rigor >= recommended"`, at a
`mandatory` verify stage. That walk breaches AC-347 in terms (*"a verify run that emits any harness dispatch fails the
activity"*).

**The operator took the route in two steps.** D21-41 (c) chose *"an rk-cli verify pass on ADR-0162's shape … selected
by the required kind `TestReport` and never by a stage literal"*. D21-42 1a kept the pass and changed what it calls:
`Toolchain::test` returns an outcome-bearing report, and 1a *"Supersedes D21-41(c)'s 'through the port' register
reading, which is withdrawn as unneeded"*. ADR-0169 (record T) built that return; ADR-0170 (record V) built the
registry-backed seventh method and the per-toolchain values table.

**Three owed items have kept the verify record open since 7b-ii.** The transport (`s7c` RT-6 / R-9: *"the transport
into `rk_kernel::verify` is 7d's composition root — a record is OWED (R-9)"*); D17-4 (AC-321's *"its record is
replayed for audit"* needs a completed verify run's event, and `EventKind` has none); and `s7d` §7d.18's verify
block, raised because *"the verify activity runner is neither an adapter's nor this stage's set"*. D21-42 10b
decides the event and 10c claims the block whole.

**D21-43 N23 re-homes the displaced providers.** It reads *"**Re-homed to the implement stage** as test-authoring
activities under ADR-0072. Verify only measures."* N23 said five; four are measured, because `migration-dry-run`
declares `provides = ["MigrationTestReport"]`, which a `pass` source for `TestReport` does not displace.

**D21-46 R5 settles the two agent-bearing activities N23 left at verify.** It reads *"**`integration-tests`' test-writing steps are re-homed to implement**, as N23. `migration-dry-run` stays a named residual, because no Phase-1 lane reaches it (`data_migration` is NOT MEASURABLE)."*
Its letter names the activity's *"test-writing steps"*; a step cannot sit in a different stage from its activity.
D21-48 X9 took the reading: *"**The whole activity moves to implement**, because stage membership is per activity."*

**Two accepted records bar the mutation harness that AC-338 and AC-339 need.** ADR-0104 decision 1 fixes the harness:
*"The harness applies the patch at the candidate SHA, runs the suite, reverts, and records the outcome. It is
deterministic and dispatches nothing."* ADR-0162's forbids bullet reads *"Forbids a seventh method on the `Toolchain`
port, and forbids applying a mutation patch from `rk-cli`; `ReproKind::Mutate` is refused by name until a record adds
the capability."* ADR-0170 superseded that bullet's seventh-method half only, and says so: *"The patch half stands
here; `s8g`'s verify record may move it for the verify gate's declared AC mutations alone."* The git write the harness
needs (`apply`) is R3's (ADR-0193); this record moves the patch clause.

## Decision drivers

- One `TestReport` route, stated once: two silent routes are the risk the packet named (D21-41 c).
- AC-347: a verify run dispatches nothing, and every verify-stage agent step is named or gone.
- ADR-0162 decision 5: *"The review pass is selected by an activity's declared `provides`, never by a stage name."*
- ADR-0116's route for a new kind: the design §5 row and the `rk-types` variant *"land in the same change"* (decision 5).
- ADR-0140 decision 2: the construction bound *"is not a backstop, because there is nothing behind it"*; the pass is
  whole-suite, so the cold figure is read before the pass is sized.
- Records are located by the id recorded at write time (D21-47).

## Considered options

### Option — a pass-sourced `TestReport`, the displaced providers re-homed over a minted test-authoring kind (taken)

A third `KindSource`, `pass` — *produced by a composition-root pass the record names* — and `[artifact.TestReport]
source = "pass"`. The selector selects no catalog producer for a `pass` kind; stage completeness still requires its
`artifact.write`. The four displaced providers and `integration-tests` move to `stage = "implement"` and list a minted
kind, **`AuthoredTests`**, first in `provides`, with an `[artifact.AuthoredTests] source = "activity"` row. Taken:
it keeps one route, removes every reachable verify-stage agent step, and gives each kind one meaning.

### Option — reuse `TestDesign` as the test-authoring kind

`spikes/p9/schemas.toml`'s `[value.TestDesign]` is a case design, and `equivalence-boundary` already emits it beside
its report, so one kind would carry two meanings. Rejected.

### Option — keep `source = "activity"` and rank an agent-free `whole-suite-test` activity first

Rejected: `bug-standard`'s `[[stage.verify.require]] when = "criticality >= high" … rigor = "mandatory"` escalates the
test-design family to `equivalence-boundary` and `property-based-testing`, both agent-bearing, so the catalog route
re-enters AC-347's breach at the rigor the fix lane is most likely to reach.

### Option — split `integration-tests` into an implement-stage author and an agent-free verify measurement

Rejected: it mints a 103rd shipped activity, moving `s8b`'s pinned count of 102 and the `curation.decision` backfill,
and N23's precedent moves activities whole. X9 decided the whole-activity reading.

## Decision

1. **`TestReport` has exactly one producer on a lane: the verify pass** (`crates/rk-cli/src/verify_pass.rs`, beside
   `review_pass.rs`, on its shape). It runs for a node whose resolved required set contains `TestReport`, read off
   the selection and never off `resolved.stage`; `crates/rk-cli/src/` gains no stage-name literal. `KindSource` gains
   `Pass`, and `.redkiln/artifacts.toml`'s `[artifact.TestReport]` reads `source = "pass"` (keyed by toolchain, as
   ADR-0073 decides: one instance *"per element of the item's `toolchain` set"*).
   *(dated 2026-09-29, s8g build, package S8G-P5-VERIFY-PASS; D21-120, TAKEN PROVISIONALLY under D21-105; an
   in-record note before the flip.)* **Where this decision's completeness clause is enforced.** *"stage completeness
   still requires its `artifact.write`"* (Considered options) is enforced in `rk_kernel::replay::resume`: its
   `AdvanceStage` arm, before `rt.step`, and its `Done` arm when no successor is supplied compute the owed set,
   `selection.pass_sourced()` less every instance with a journalled `artifact.write` for this item at this stage among
   the rows after the stage's latest entry (a row matches on its `kind`, and a keyed instance on its `key` member too;
   never on Classification alone). While the set is non-empty the arm refuses by name (`pass-owed: TestReport{rust}`),
   takes no step and writes no row, and `RecordedRun::pass_owed()` carries the set typed. `rk-cli`'s `run` runs this
   pass for each owed `TestReport` and re-enters the same stage once. `fsm::next_action`, `Runtime::step` and ADR-0152
   decision 2 are unchanged: the arm still breaks.

2. **The pass, in order.** (1) It meets a set-up tree: it runs in the run-owned worktree at `.redkiln/worktrees/<item>`
   (ADR-0180), where `setup` ran at creation (D21-43 N25: *"refused by name on failure"*), so `setup` precedes the
   pass's one `test` call and a failed `setup` leaves zero `test` calls. (2) It refuses before any port call if the
   changed-file set or the selection policy is unresolved (AC-320): a `StepError`, no gate row, zero port calls.
   (3) It calls `rk_kernel::verify::runner`, which makes the one `Toolchain::test` call, whole-suite (AC-328's
   *"explicit whole-suite policy"*), and journals `verify_completed` in the same function (decision 8). (4) It maps
   the outcome: `Ok` with `failed == 0` is green; `Ok` with `failed > 0` is a failed verify, never a report of green;
   a timeout or an OS kill is `Unverifiable` (AC-331); a spawn failure is `Unverifiable { cannot_run }` (AC-344).
   (5) It validates through `report_at_boundary` (AC-324), feeds ADR-0104 decision 3's comparator, and writes the
   keyed `TestReport` through `rk-store`'s write path (ADR-0189): one `artifact.write`, keyed by the item's
   `toolchain` facet element, never the adapter name. (6) It runs the AC ledger and the stage-exit check (AC-346,
   ADR-0125) over the item's criteria as `rk_types::AcceptanceCriterion` values (R2, ADR-0192), including the
   mutation harness for every criterion with a declared `mutation` (decision 11). (7) It appends through the store and
   re-reads the journal (ADR-0162 decision 6's idiom).
   *(dated 2026-09-29, s8g build, package S8G-P5-VERIFY-PASS; TAKEN PROVISIONALLY and returned for a ruling.)* **The
   keyed write's carrier.** ADR-0189 (accepted) decision 8 lists the `artifact.write` row's members and has no key,
   and its decision 3 keeps one sidecar per `(item, kind)`; D21-120 matches a keyed instance on its key element. The
   build adds one member, `key`, to the row `rk-store`'s `ArtifactWriter::write_keyed` appends, and only that writer
   writes it; the sidecar stays `<Kind>.json`, so an `artifact:TestReport` reader still finds it. An un-keyed instance
   (every shipped lane today: `rk-cli` sets no item `toolchain` set, so a keyed kind expands un-keyed) is written by the
   unchanged `write`, with no `key` member. Step (4)'s failed or unverifiable verify writes no `TestReport`: the
   pass-owed guard then keeps holding the stage, and `run` exits by name.
   *(dated 2026-09-29, s8g build, package R-D2122-D2123-8g; ruled D21-122, TAKEN PROVISIONALLY under D21-105; an
   in-record amendment before the flip.)* **The carrier is kept, and this record supersedes ADR-0189 in part, for
   decision 8's member list only** (an additive `key` member on the keyed writer's row): ADR-0189's decision 8 lists
   the row *"with these members"* and names no key, so `"0189"` enters this record's `supersedes`. ADR-0189 decision
   3 (*"One file per `(item, kind)`"*) does not move, and ADR-0189 is not edited now: its back-link (*"superseded in
   part by 0191, decision 8's member list"*) is written at this record's flip, and only if the operator has not
   objected in the ledger. **A second key is refused by name.** `write_keyed` refuses, before any write, when its
   stage run already holds a keyed `artifact.write` of the same kind with a different `key`:
   `RenderError::KeyedSidecarCollision { kind, prior_key, key }` (*"keyed-sidecar-collision: TestReport{node} after
   TestReport{rust}"*), with no sidecar, no rendering and no row. The pass maps it to `Refused::Write`, so a
   two-toolchain item holds its stage by name and D21-120's guard keeps owing the second key; a later write with
   the same key still replaces the sidecar. `served`'s key-blind replay match is unchanged, because at most one key
   per kind per stage run can now exist. **Residual, left to a later record:** per-key sidecars (D21-122's option
   2), to be written if a multi-toolchain lane ever ships (it would supersede ADR-0189 decision 3). No shipped lane
   reaches this code: `rk-cli` sets no item `toolchain` set.

3. **The four displaced providers are re-homed (D21-43 N23).** `unit-tests`, `unit-tests-ue`, `equivalence-boundary`
   and `property-based-testing` each take `stage = "implement"` and list `AuthoredTests` first in `provides`, in
   place of `TestReport`; each emit binds `AuthoredTests` and no longer binds `TestReport`, and each activity's own
   `toolchain:test` runs are step evidence, never the `TestReport` artifact. Each is a new version under ADR-0072
   with a row in `.redkiln/CHANGELOG.md`: `unit-tests@3`, `unit-tests-ue@3`, `property-based-testing@3`, and
   `equivalence-boundary@4` (`s8d` moved it to 3 under D21-67 ruling 1; measured on disk at `178868f`).
   `unit-tests-ue` authors nothing (two tool steps, a check and an emit): it is re-homed by N23's letter, and this
   record says so rather than hiding it.

4. **`AuthoredTests` is minted.** `.redkiln/artifacts.toml` gains `[artifact.AuthoredTests]` with `source =
   "activity"` and `schema = "AuthoredTests"`; ~~it is not `keyed_by`~~. It names the tests an implement-stage activity
   authored; it is not a verdict.
   *(dated 2026-09-29, s8g build, package S8G-P5-VERIFY-PASS; D21-121, TAKEN PROVISIONALLY under D21-105; an in-record
   amendment before the flip.)* **`AuthoredTests` is `keyed_by = "toolchain"`, on ADR-0073's mechanism**, and the struck
   clause *"it is not `keyed_by`"* is withdrawn. Unkeyed, AC-047's load-time rank lint refuses every shipped selection
   (`Load(RankTie { kind: "AuthoredTests", … activities: ["unit-tests", "unit-tests-ue"] })`): both are `method_rank`
   20, and while they provided the keyed `TestReport` ADR-0073's per-instance expansion kept them from contending.
   ADR-0132's own finding is the reason: *"The tie is a shadow of a kind that *is* keyed."* **This record therefore
   supersedes ADR-0132 in part, at one site, its `## Decision` enumeration**: *"The toolchain-keyed set is **six**"*
   and its pack half's *"moves 5 → 6 keyed kinds"*. With `AuthoredTests` the set is **seven** and the pack moves
   **6 → 7**. Every other clause of ADR-0132 and of ADR-0073 stands verbatim, the `method_rank` forbids bullet
   included; `spikes/p9/artifacts.toml` is not edited (ADR-0132). ADR-0132's `superseded_by` gains `0191` at this
   record's flip, not before.

5. **The test-design escalation moves with them.** Both packs' verify-stage row *"`when = "criticality >= high"`"* →
   `TestReport` at `rigor = "mandatory"` moves to `[stage.implement]` over `AuthoredTests`, keeping its `when` and its
   `rigor`, because a rigor floor on a pass-sourced kind selects nothing. On the two `s8` lanes (`criticality = low`)
   no re-homed activity is selected at implement, and `equivalence-boundary` is selected at no stage.

6. **`integration-tests` is re-homed whole (D21-46 R5; D21-48 X9).** D21-46 R5's letter re-homes its *"test-writing
   steps"*; X9 reads it: *"**The whole activity moves to implement**, because stage membership is per activity."* So its
   `write` step moves with its `run` and `green` measurement steps: `stage = "implement"`, `provides =
   ["AuthoredTests", "IntegrationReport"]`, the emit binding `AuthoredTests` beside `IntegrationReport`, version 2 →
   3 with an `integration-tests@3` changelog row. `story.toml`'s *"`when = "external_dep"`"* → *"`artifacts =
   ["IntegrationReport"]`"* row moves from `[stage.verify]` to `[stage.implement]`; `bug-standard.toml` has no such
   row. Verify still measures the integration tests: the pass's whole suite runs them.

7. **`migration-dry-run` is AC-347's single named residual.** It keeps one agent step at verify. D21-46 R5: *"no
   Phase-1 lane reaches it"*, verified by quotation and not asserted: `.redkiln/facets.toml`'s `data_migration` is
   `deriver_class = "lexical"` (*"may key no require row (AC-520)"*, *"NOT MEASURABLE at 47faec9f: 0 positives on 20
   items across all three sources"*); `story.toml` reads *"DROPPED: [[stage.verify.require]] when = 'data_migration',
   artifacts = ["MigrationTestReport"]"*; and its one consumer, `migration-expand-execute`, feeds a `MigrationLog`
   whose release row both packs dropped at 7e. AC-347's shipped-catalog lint pins the verify-stage agent-bearing set
   as exactly `{migration-dry-run}`, and a selection test asserts that no shipped lane selects it. A row that makes
   it reachable is AC-347's breach, not a named exception.

8. **`verify_completed` is minted, tier 2, on ADR-0116's route**, with the design §5 row and the `rk-types` variant in
   the same change (ADR-0116 decision 5: *"The table row and the `rk-types` variant land in the same change."*). Its
   payload: `report_digest`, `selection`, `basis`, `basis_reason`, `passed`, `failed`, `selected_test_count`,
   `adapter`, `range: { base, head }`, `paths`, `toolchain_version`, `origin`, `stage`. D21-42 10b's four members
   plus AC-321's three (*"the range endpoints, the resolved path list and the toolchain version are read back from
   the run event verbatim"*). The runner rides on the report and reaches the row through `report_digest`, with no
   `runner` member (D21-48 X15: *"records on the report which runner ran"*).
   - **One site.** The kernel appends the row once per **completed** `Toolchain::test` port call, whatever activity or
     pass made it; the executor's `toolchain:test` arm and the pass reach the port through that one function. A
     timed-out, killed or unspawnable call writes no `verify_completed`; its refusal is the `StepError` row.
   - **`origin`** is `"pass"` for the pass's call and `"<activity>@<version>/<step>"` for a catalog step's call;
     **`stage`** is the node's stage name, carried as data and never compared (this is not selection).
   - **`adapter`** is the name `real_toolchain`'s `detect()` loop chose, supplied by `rk-cli` as an opaque value; the
     kernel never compares it (AC-487).
   - **It is item-scoped.** Its interim `item` key rides beside the event on ADR-0182's carrier (the `t2_event.item`
     column; D21-46 R3: *"**Carried outside the canonical digest**"*), never in its payload, so no ADR-0131 bullet
     moves. `fsm::every_item_scoped_kind_carries_the_interim_item_key`'s carried list names it in the work commit, and
     `s8e`'s file takes a dated line naming this record. `EventKind::ALL` moves 41 → 42, and Checkpoint 7c's size guard
     moves in the same commit (D21-47).
   - **Tier 2** by ADR-0109 decision 2's deleted-database test: the report is recomputable by re-running the suite.
     No trailer.

9. **The register moves with the carrier.** AC-473's *Verified by* (*"an adapter-identity assertion on the toolchain
   stage's dispatch record"*) is amended, by dated note, to read the item's `verify_completed` rows whose `origin` is
   `"pass"` (D21-42 10b). AC-485's toolchain identity half reads the same rows; its harness half stays with RT-41's
   carrier. AC-320's *Verified by* (*"zero `process_spawned` events"*; no such kind exists) is read, by dated note,
   as a zero port-call count: no spawn is possible without a port call. AC-341's *"CI is green and verify is `Err`"*
   integration half is Phase 3's (AC-508): the `Tracker` port is *"Fake-only in Phase 1; GitHub is Phase 3"* and has
   no status surface. D17-4 closes in the tracker commit, naming this record. R-9 and RT-6 are discharged.

10. **The verify block of `s7d` §7d.18 is routed id by id** (D21-42 10c), each by its named instrument in `s8g` §8g.5:
    AC-320, AC-338, AC-339, AC-348, AC-349, AC-353 and AC-359 whole; AC-355 on its fixtures (its *"held in CI"*
    clause is `s8j`'s); AC-341 on its port-call half; AC-321 beside the block. AC-357 is a human gate and is not
    claimed; its machine part is exercised inside AC-359's test. AC-349's claims binder reads a `claims` column this
    record adds to ADR-0170's per-toolchain table, and its doctor row is `RK-D-TOOLCHAIN-CLAIM-TIE`.

11. **The mutation harness is ADR-0104 decision 1's, built here, and ADR-0162's patch clause is superseded in part,
    unconditionally, for the verify gate's declared AC mutations alone** (D21-47 review, cross-file finding 19). The
    clause is ADR-0162's forbids bullet, quoted whole: *"Forbids a seventh method on the `Toolchain` port, and forbids applying a mutation patch from `rk-cli`; `ReproKind::Mutate` is refused by name until a record adds the capability."*
    - The pass applies a criterion's declared `mutation` patch through `GitOp::Apply` (R3, ADR-0193), only in an
      ephemeral worktree named **`<item>.mut`**, beside the review pass's `<item>.review-ref` under
      `.redkiln/worktrees/`, **never inside the run-owned worktree**; the tree is created and removed on every path
      (ADR-0162 decision 10's precedent). The kernel applies the patch; no `-p` child does.
    - The name's 4-character suffix fits the 11 characters ADR-0180's worst-case path sum already reserves for
      `.review-ref` (255 of 259 at `178868f`), and `s8d-ii`'s item-id validator refuses an id ending `.mut`.
    - The review pass's `ReproKind::Mutate` arm stays refused by name. ADR-0162 keeps `status: accepted` and gains
      this record's id in `superseded_by`.
    - *(dated 2026-09-29, s8g build, package S8G-P6-MUTATION-RESUME-DRIFT; proposed, not flipped.)* **Built, and
      what the build chose where this decision is silent, each reversible.** (a) The harness is
      `rk_cli::verify_pass::mutate`: it creates `<item>.mut` detached at the run-owned tree's `HEAD` (the candidate
      SHA), applies the declared patch through `rk_git::mutation::apply` (the one `GitOp::Apply` site, R3), runs the
      declared `suite` through `rk_kernel::verify::runner::run_test` over a `Toolchain` rooted in that tree (its row's
      `origin` is `mutation/<ac>`, never `pass`), re-reads every path the patch touches and compares that region's
      content hash with the applied one (AC-338: a restore mid-run is `still_present = false`, so `Unverifiable`),
      and removes the tree on every path. The sentence above, *"The kernel applies the patch; no `-p` child does"*,
      is met in its contrast (no `-p` child applies anything) but not in its subject: the call is made from `rk-cli`'s
      pass through `rk-git`'s journalled seam, which is the capability this decision moves ADR-0162's patch clause
      for. Named for the flip, not edited. (b) The port reports counts, not which assertion failed, so the harness
      records `assertion_flipped = None`: a red suite is `RedElsewhere` and `pinned` is unreachable through it (AC-334
      stays ARMED, `rk_kernel::verify::flip`'s own note). (c) Decision 2's step (6) runs BEFORE step (5)'s write:
      a refused stage exit writes no `TestReport`, so D21-120's guard keeps holding the stage and the next entry
      re-runs the pass and the ledger. (d) The criteria are read off the committed `Spec` sidecar in the run-owned
      tree (`acceptance_criteria`, each through `AcceptanceCriterion::ingest`); an absent sidecar is no criteria.
      (e) The feasibility input is the pack's `[mutation]` table (AC-337: absent is a configuration error, raised only
      when the item has criteria); ADR-0192 decision 2's shipped home (the Rust row of ADR-0170's table) ~~is not yet
      written, so a shipped lane whose `Spec` carries criteria refuses at verify by name until it is~~.
      *(dated 2026-09-29, s8g build, package R-D2122-D2123-8g; ruled D21-123, TAKEN PROVISIONALLY under D21-105.)*
      The shipped home is written; the pack table remains a second reader and a disagreement is a configuration
      error. The row is `[toolchain.rust] mutation = { value = { feasible = true }, source = "…" }`, read at step 6
      through the adapter the pass ran under (`toolchain_of`); `[toolchain.node]` has no row, so a Node lane with
      criteria still refuses by name. The row turns the configuration refusal into a harness run whose stage exit
      still refuses (`pinned` is unreachable, (b)), never into a pass. (f) A refused
      exit appends `stage_exit_refused` (keyed to the item) and the kernel mints AC-357's gate,
      `verify.ac-discriminates` (`rk_kernel::verify::ac_gate`), for the first unpinned criterion, carrying `ac`,
      `criterion`, `assertion` and `suite_result`; after a void answer it re-opens one reach on with `drift` (the
      void row's two tree hashes and token) attached (AC-359). Only an applied `discriminates` answer naming its
      answerer is a ledger answer; its `at` is the instant of the run that reads it, because the `gate_answered` row
      carries no time.

12. **The pass is whole-suite, and it is sized against the cold figure `s8c` measured, cited and not re-measured.**
    D21-42 10c: *"A cold `cargo test --workspace` wall time is measured first"*; `s8c` §8c.5: *"This is that
    measurement, and `s8g` cites it rather than repeating it."* ADR-0140 decision 2 bounds each child the adapter
    spawns, and the figure is the slowest child (with X15's nextest runner, the `run` child; the cold compile lands
    in the `--list` child, 99 s at its slowest). Re-measured at the tree holding `s8c`'s work, n = 3, ADR-0170's
    evidence table reads `run` 152, 152 and 161 s:
    cold cargo test --workspace slowest child: 161 s (source: ADR-0170, at 2f3f6ab257c4b21c1bd16bcd4781f97475f958f5)
    It is under 3600 s, so a cold verify pass is not `Unverifiable` by construction (golden constraint 6).

## Consequences

**Positive**
- One `TestReport` route, reached by kind; the verify stage walks no agent step on any reachable lane.
- AC-321's replay and AC-473/AC-485's identity reading have a carrier the kernel writes at one site.
- AC-338 and AC-339 have a harness that dispatches nothing and never touches the run-owned tree.

**Negative**
- `migration-dry-run` still carries an agent step at verify; N23's *"Verify only measures"* holds over every reachable
  verify activity, not over the shipped catalog. Making `data_migration` measurable, or adding any row that requires
  `MigrationTestReport`, reopens it for whichever stage does it.
- `unit-tests-ue` is re-homed as a test-authoring activity and authors nothing.
- `EventKind::ALL` moves 41 → 42, which moves Checkpoint 7c's size guard and `s8e`'s item-scoped list.
- Checkpoint 8b's shipped-pack lattice pin and `s8d`'s walk tables move; Checkpoints 5, 6, 7b, 7b-ii, 7d, 8b, 8d,
  8d-ii and 8e are provisional until Checkpoint 8g re-runs them whole.

**What this forbids**
- A second `TestReport` producer on a lane: a catalog activity providing `TestReport`, or an emit binding it.
- Selecting the pass, or any verify behaviour, by comparing `resolved.stage` with a string literal.
- A `verify_completed` row for a call that did not complete, or written anywhere but the kernel's one port-call site.
- Carrying the interim `item` inside `verify_completed`'s payload, or comparing its `adapter` in `rk-kernel`.
- Reading AC-473's adapter identity from a `verify_completed` row whose `origin` is not `"pass"`.
- Applying a mutation patch in the run-owned worktree, from a `-p` child, or for anything but a criterion's declared
  `mutation` at the verify gate; lifting `ReproKind::Mutate`'s refusal in the review pass under this record.
- A new agent-bearing activity at `stage = "verify"`, or `integration-tests` back at verify.
- Re-measuring the cold figure in `s8g`, or sizing the pass against a figure at or above 3600 s.

## Evidence and confidence

**Measured** (2026-09-29, at `178868f`, `s8g`'s stage open): `EventKind::ALL` is `[Self; 41]`; the four providers
provide `TestReport` at `stage = "verify"` (versions 2, 2, 3, 2); `integration-tests` is at verify, version 2,
providing `["IntegrationReport"]`; `migration-dry-run` is at verify, version 4; `crates/rk-cli/src/review_pass.rs`
exists and `verify_pass.rs` does not; the worst-case mutation-tree sum is 39 + 64 + 11 + 1 + 140 = 255, with
`core.longpaths` unset. **Cited, not re-measured:** the cold figure, from ADR-0170's evidence (*"`test_timeout` = 60
× ⌈2 × 161 / 60⌉ = **360** (the slowest child, pass 3's nextest run"*), at `2f3f6ab`. **Confidence:** high on the
route and the carrier, which are the operator's decisions executed; medium on `AuthoredTests`'s schema, which this
record names and the build fills.
