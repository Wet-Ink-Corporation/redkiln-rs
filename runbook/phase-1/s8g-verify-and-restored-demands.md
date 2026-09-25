# Step 8g — The verify record and the demands restored (Phase 1, part 8, sub-stage g)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8g` | Phase 1 | Checkpoint 8f | Checkpoint 8g | `s8f` | `s8h` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** The fix lane's verify stage demands three artifacts and its release stage two, and at
`ca8f387` none of the five has a producer a run can reach. `bug-standard.toml` reads *"`baseline = ["TestReport",
"ConfirmationResult", "SmokeResult"]`"* at verify and *"`baseline = ["BuildArtifact", "ReleaseRecord"]`"* at release,
and its verify stage adds *"`when = "'text' in change"`"* → `ScanReport`. D21-41 (c) and (d) narrowed four of those
out as owed; **D21-42 6a reverses the narrowings** and 6b gives a CLI its own release. Underneath the producers sits
the verify record itself, which three owed items have kept open since 7b-ii:

- **The transport.** `s7c` routed it as RT-6 / R-9 — *"the transport into `rk_kernel::verify` is 7d's composition
  root — a record is OWED (R-9)"* — and `s7d` §7d.18 raised the verify block (AC-320, AC-338, AC-339, AC-341, AC-348,
  AC-349, AC-353, AC-355, AC-359) because *"the verify activity runner is neither an adapter's nor this stage's set
  (no §12 or §13 criterion names it); a record routing the block to a stage is OWED"*.
- **D17-3**, OPEN since 2026-09-17: three key spellings no record fixes, and *"`rk_types::AcceptanceCriterion` (AC-137,
  ADR-0104's `mutation:` field), which the tree does NOT carry"*. The kernel reads a stand-in whose own header says
  so: *"a kernel-local shape, decided 2026-09-17 by the operator: the full `rk-types` spec type is the migration's,
  7d"* (`crates/rk-kernel/src/verify/ledger.rs`).
- **D17-4**, OPEN since the same day: AC-321's *"its record is replayed for audit"* needs a completed verify run's
  event, and `EventKind` has none.

**This stage takes records, and says so at the top.** Five are named in §8g.8, each at the next free id **at the
moment it is written** (no number is pre-assigned here: `s8a`–`s8f` write records too, and their order is not
fixed). Every one is committed `proposed` and flips only on the operator's dated ledger line. Three earlier stage
files said *"this step takes no decision"* and were wrong (`s7g`, `s7h`, `s7i`); this one does not repeat the
sentence.

**What it inherits and must find built, measured at its open (Halt 8 if any is absent):**

| from | what | how the open measures it |
| --- | --- | --- |
| `s8c` (D21-42 1a) | `Toolchain::test` returns an outcome-bearing report (the `VerifyReport` plus passed/failed counts); a red suite is `Ok` with counts; `Err` is cannot-run, timeout or killed only | read the trait in `crates/rk-ports/src/toolchain.rs`; the six-method count |
| `s8c` (D21-42 1b, 4a) | the registry-backed seventh method and design §4.5's `registry.toml` verb table, with its per-toolchain values table | the registry file and its Rust rows exist |
| `s8d` (D21-42 3a, 4c, 4d) | step-grain commits and the complete reference resolver: output binding, iteration, check evaluation, emit binding and test-selection resolution; `"affected"` is the package-level affected set, falling back to whole-suite with `basis_reason` | a fixture activity of shape `[tool, check, emit]` walks to an `artifact.write` |
| `s8e` (D21-42 24c) | the first FakeHarness end-to-end test of `verbs::run::run` | `grep -rn "verbs::run::run(" crates/*/tests` is non-empty |
| `s8f` (D21-42 5a, 5b) | rk-store's render/write path, one `artifact.write` per write, under the item's managed artifact directory in the run-owned worktree | the one write path's call site |

🛑 **If `s8c` or `s8d` delivered a different shape than this table names, this stage stops and reports rather than
re-deriving the shape** — the producers below are thin *because* those stages are thick, and a producer written
against a guessed port is the `s7g` shape (a lint green over code nothing calls).

### 8g.1 What this stage owns, and what it may not decide

It owns **D21-41 (c) as superseded by D21-42 1a, and D21-42 6a, 6b, 10a, 10b and 10c** (the plan's row). It does
**not** own DebtRegisterEntry (6a's fifth demand is `s8f`'s rk-store item write), the card grants and the `Spec`
type's schema (D21-41 g, `s8h`), the story lane or the pack-declared intake question (D21-41 f, `s8j`), or any real
dispatch: **no line of this stage dispatches to a real account** (D21-40 decision 2).

**Crates this stage edits, and the checkpoints that edit makes provisional** — each marked `passed <hash>, re-run
pending s8g <crate>` in `PROGRESS.md` §1 in the commit that lands the edit, on the ADR-0124 decision 5 precedent
(*"The consequence for Checkpoint 5 is taken, not routed around"*):

| crate | edit | provisional row | re-run where |
| --- | --- | --- | --- |
| `rk-types` | `EventKind::VerifyCompleted`; `AcceptanceCriterion` in `registry::EXPORTED`; the judgement-enum registry | Checkpoint 5 | Checkpoint 8g (12), whole |
| `rk-git` | `GitOp::Tag`, `GitOp::Apply` | Checkpoint 6 | Checkpoint 8g (12), whole |
| `rk-kernel` | the verify runner; `KindSource::Pass`; the ledger reads `rk_types::AcceptanceCriterion` | Checkpoints 7b and 7b-ii | Checkpoint 8g (12), whole |
| `rk-adapter-toolchain-rust` | the `smoke`, `sca_scan`, `package` rows and their bodies | Checkpoint 7c | **not re-run here**: 7c's block carries the live probe, which runs only at `s8j` after the fixture proofs (D21-40 decision 2). Its non-live lines are asserted by name in (5) below; the whole re-run is `s8j`'s, and the row says so |
| `rk-cli` | `verify_pass.rs`, the release write, the trial route | Checkpoint 7d | Checkpoint 8g (12), whole |

### 8g.2 The verify pass — one TestReport route, called through the widened port

**The route, stated once, because two silent routes is the risk the packet named.** D21-41 (c) chose *"an rk-cli
verify pass on ADR-0162's shape … selected by the required kind `TestReport` and never by a stage literal"*; D21-42
1a kept the pass and changed what it calls — `Toolchain::test` now returns the outcome, so the pass calls **the
port**, and 1a *"Supersedes D21-41(c)'s 'through the port' register reading, which is withdrawn as unneeded"*. So:

- **`TestReport` has exactly ONE producer on a lane: the verify pass.** It runs for a node whose required set
  contains `TestReport` — read off the resolved selection, never off `resolved.stage` (ADR-0162 decision 5: *"The
  review pass is selected by an activity's declared `provides`, never by a stage name"*, and the same record forbids
  *"matching `resolved.stage` against a string literal"*).
- **The catalog's five `TestReport` providers are DISPLACED, not deleted.** `artifacts.toml`'s header says *"source =
  "activity" → some catalog activity provides it"* and the selector's `KindSource::Activity` doc says *"Some catalog
  activity `provides` it"* — so as shipped, selection would walk `unit-tests`, whose `gaps` and `fill` steps are
  `kind = "agent"` under `when = "rigor >= recommended"`, and bug-standard's verify stage is `mandatory`. That walk
  breaches AC-347 in terms (*"a verify run that emits any harness dispatch fails the activity"*). The verify record
  (§8g.8, R1) therefore adds a third `KindSource`, **`pass`** — *produced by a composition-root pass the record
  names* — and sets `[artifact.TestReport] source = "pass"`. The selector selects no catalog producer for a `pass`
  kind; stage completeness still requires its `artifact.write`. **The rejected alternative is named**: keep
  `source = "activity"` and author an agent-free `whole-suite-test` activity ranked first — rejected because
  bug-standard's `[[stage.verify.require]] when = "criticality >= high" … rigor = "mandatory"` escalates the
  test-design family to `equivalence-boundary` and `property-based-testing`, both of which carry agent steps, so the
  catalog route re-enters AC-347's breach at the rigor the fix lane is most likely to reach.
- **ConfirmationResult, SmokeResult and ScanReport are catalog walks** (§8g.6) — `confirmation-test`, `smoke-test`
  and `sca-scan`, all agent-free, walking under `s8d`'s resolver and `s8c`'s port and registry method. Their
  `toolchain:test` calls are port calls like the pass's, and journal the same row (§8g.4).

**The five displaced activities are a consequence, measured rather than assumed.** `unit-tests`, `unit-tests-ue`,
`equivalence-boundary`, `property-based-testing` and `migration-dry-run` declare `provides` `TestReport`. Under a
`pass` source each loses its selection reason at verify, and the P6 lattice (ported to Rust at `s8b`, D21-42 13b) is
re-run with the new source and its dead count reported **by id** against AC-102's registered 20% line. Their
re-homing — test *design* is implement-stage work, and AC-347's lint says *"the verify activity's step kinds contain
no agent step"* — is a catalog decision **this stage does not take**; the record names it as owed.

**What the pass does, in order** (`crates/rk-cli/src/verify_pass.rs`, beside `review_pass.rs`, on its shape):

1. Refuse before any port call if the changed-file set or the selection policy is unresolved (AC-320) —
   `StepError`, no gate row, no port call (asserted by a counting `Toolchain` double: zero calls).
2. Call `rk_kernel::verify::runner` — **the kernel makes the port call** and journals `verify_completed` in the same
   function (§8g.4). The selection is AC-328's *"explicit whole-suite policy"*: the pass requests whole-suite, and the
   report's `basis_reason` is what the adapter says under AC-352's closed enum (`probe-unverified` while no
   affected-selection probe is recorded at the pin).
3. Map the outcome: `Ok` with `failed == 0` is green; `Ok` with `failed > 0` is a **failed verify, never a report of
   green**; `Timeout` and an OS kill are `Unverifiable` (AC-331); a spawn failure is `Unverifiable { cannot_run }`
   (AC-344, the note `s7d` §7d.12 owed).
4. Validate through `report_at_boundary` (AC-324), feed the ADR-0104 decision 3 comparator, and write the keyed
   `TestReport` through `s8f`'s rk-store write path — **one `artifact.write`**. The key is the item's `toolchain`
   facet element (ADR-0073: *"one instance per element of the item's `toolchain` set"*), **never the adapter name**.
5. Run the AC ledger and the stage-exit check (AC-346, ADR-0125) over the item's ACs as `rk_types::AcceptanceCriterion`
   values (§8g.3), including the mutation harness for every AC carrying a declared `mutation` (§8g.5, AC-338/AC-339).
6. Append through the store and re-read the journal (ADR-0162 decision 6's idiom).

🛑 **The pass is whole-suite, and a cold whole suite runs against ADR-0140's bound, which is THE bound** — *"It is
not a backstop, because there is nothing behind it"*. D21-42 10c says *"A cold `cargo test --workspace` wall time is
measured first"*. **If `s8c` recorded that figure, this stage cites it by quotation from `s8c`'s record and names the
commit it was measured at; it does not re-measure.** Otherwise it is measured here, before `verify_pass.rs` is
written: a throwaway worktree with an empty `target/`, `cargo test --workspace --no-fail-fast`, wall time from the
shell, the worktree removed after. Either way the verify record carries one line of the form
`cold cargo test --workspace wall time: <N> s (source: <s8c record | this stage>, at <sha>)`, and Checkpoint 8g (8)
reads it. **A figure at or above 3600 s is a halt**: every cold verify pass would be `Unverifiable` by construction
(golden constraint 6 — size the ceiling against the slow condition).

### 8g.3 The spec/pack schema record — D17-3's three spellings, and `AcceptanceCriterion` exported (D21-42 10a)

D17-3 lists three spellings that are *"readings, not decisions"*. The schema record (§8g.8, R2) fixes all three **as
they are read today**, so nothing on the tree moves but its warrant:

| spelling | where it is read today | fixed as |
| --- | --- | --- |
| `[policy] verification_required = [<stages>]` | `select/pack.rs`: *"**AC-455's policy row**"* | the process pack's key, unchanged |
| `[mutation] feasible = <bool>, reason = <str>` | `rk_ports::toolchain::MutationFeasibility::declared`; the Rust fixture pack | the key, unchanged — **and its shipped home** (below) |
| `[[ac]] … [ac.mutation] not_feasible = "<reason>"` | `verify/ledger.rs`'s fixture reader | superseded by `AcceptanceCriterion`'s serde form (below) |

**Two absences the record must state, both measured at `ca8f387`:**

- **The shipped `.redkiln/` declares no `[mutation]` table anywhere.** Its two packs are process packs, and AC-337 says
  *"an absent declaration is a configuration error"*. The record gives the declaration a shipped home — the Rust row
  of `s8c`'s per-toolchain values table (D21-42 4a: *"Each value carries its measurement as its warrant"*), with
  `feasible = true` and ADR-0104 decision 1's warrant — so the ledger's feasibility input exists on this tree.
- **Neither shipped pack declares `verification_required`.** AC-455's refusal therefore cannot fire on this tree. What
  the shipped default *is* belongs to **AC-139, a HUMAN GATE** (*"the choice is recorded as a human decision with its
  rationale, and the shipped default is whatever that decision names"*). The record fixes the spelling and **names
  the default as owed to the operator**; it does not default it.

**`rk_types::AcceptanceCriterion`** (AC-137's GIVEN/THEN; ADR-0104 decision 1's field): `id`, `criterion: Text`,
`gwt: Option<Gwt>` (*"`Gwt { given: Text, when: Text, then: Text, and: Vec<Text> }`"*), `verification:
Option<Text>`, and `mutation: Option<MutationDeclaration>`, where `MutationDeclaration` is
`#[serde(tag = "kind")]` over `Declared { patch, suite }` and `NotFeasible { reason }` (ADR-0084's discriminated
form). A prose-only mutation is refused at ingest (ADR-0096 via golden constraint 12). It enters
`registry::EXPORTED` — whose doc says *"Every schema that crosses the closed-output boundary"* — and gains its
`exported__AcceptanceCriterion.snap`, accepted once, deliberately, and reviewed as a schema change.

**The kernel-local type is retired.** `SpecAc` and its TOML reader leave `rk-kernel`; the ledger, the exit check and
`admissibility::admissible` read `&[rk_types::AcceptanceCriterion]`. Two text fixtures carry the old name and move
with it (`crates/rk-kernel/tests/fixtures/lint-controls/ac335_status_reads_the_suite_verdict.rs` and a string in
`tests/it/verify.rs`) — the name occurs **17** times under `crates/` at `ca8f387`, and Checkpoint 8g (4) requires
zero code lines.

🛑 **AC-133's registry does not exist, and this stage builds it rather than registering into nothing.** The brief
asks for the new enum to be *AC-133-registered*. AC-133's THEN is *"the enum appears in the judgement-enum registry
with an explicit judgement-bearing yes/no AND every enum marked yes has a sibling free-text `rationale` field"*.
Measured at `ca8f387`: `grep -rni judgement crates/rk-types/tests crates/rk-types/src/schema` returns **nothing**,
and no rationale-sibling rule exists — yet `s5`'s acceptance set claims AC-133 and `PROGRESS.md` §1 row 5 says
*"Decisions 6 and 7 executed (ADR-0104's `mutation:` field on `AcceptanceCriterion`…)"*, which D17-3 already called
false. The registry is built here to AC-133's *Verified by* (a completeness test over every enum reachable from
`EXPORTED`, the rationale-sibling rule, and a must-fail fixture), `Findings`' enums registered with it; row 5 takes a
**dated correction in the tracker commit**, not an erratum (D15-2's form for an amendable file).

⚠️ **A cross-stage constraint on `s8h`.** The pass reads the item's ACs off the committed `Spec` artifact. The `Spec`
type's schema is `s8h`'s (D21-41 g: schemas are Rust types in `EXPORTED`). Until then this stage's fixtures carry a
`Spec` whose one typed member is `criteria: Vec<rk_types::AcceptanceCriterion>`, and **`s8h`'s `Spec` must embed
`AcceptanceCriterion` unchanged** — named here so the second author does not re-invent it.

### 8g.4 `verify_completed` — AC-321's replay event and the identity carrier (D21-42 10b)

The kind is minted by record on ADR-0116's route (its *Option — mint the kind in the runbook* was rejected *"on
mechanism"*), in the verify record (§8g.8, R1), with the design §5 table row and the `rk-types` variant **in the
same change** (ADR-0116 decision 5). Its row:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `verify_completed` | 2 | `item`, `report_digest`, `selection`, `basis`, `basis_reason`, `passed`, `failed`, `selected_test_count`, `adapter`, `range: { base, head }`, `paths: [changed paths]`, `toolchain_version` | the kernel, once per **completed** `Toolchain::test` port call, whatever activity or pass made it |

- **10b's four members plus AC-321's three.** D21-42 10b names *"report digest, selection with `basis_reason`, counts
  and adapter identity"*. AC-321's THEN adds *"the range endpoints, the resolved path list and the toolchain version
  are read back from the run event verbatim"* — so `range`, `paths` and `toolchain_version` are carried **as
  members**, not only inside the digest. That is AC-321's text, not a new decision.
- **`item`, because the store cannot scope a row any other way.** The store's selector is inert and ADR-0163
  decision 4 carries `item` as a payload member for exactly that reason; `s8j`'s fixture proof reads the item's
  `verify_completed` rows by this member and asserts `adapter` on every one. The two member names, `item` and
  `adapter`, are **fixed here** so `s8j`'s probe binds to them rather than to a guess.
- **Tier 2**, by ADR-0109 decision 2's deleted-database test: the report is recomputable by re-running the suite, and
  the next action does not move without the row. No trailer.
- **Completed only.** A run that timed out, was killed or could not spawn writes **no** `verify_completed`; its
  refusal travels as the typed `StepError` row D21-42 20 has the kernel append. AC-321 says *"a **completed** verify
  run"*, and a row for a run that did not complete would be a replay of something that did not happen.
- **One site.** The executor's `toolchain:test` arm and the pass both reach the port through the one kernel function
  that appends the row, so a `confirmation-test` walk journals the same kind as the pass.
- **Adapter identity is data the kernel carries and never compares** (AC-487: *"`rk-kernel` contains no comparison
  against an adapter identity — no adapter name string"*). `rk-cli` supplies the name `real_toolchain`'s `detect()`
  loop chose, as an opaque value on the runtime context. argv[0] is not the identity (ADR-0137 clause 7 makes it
  configurable), and `rustc -V` names a compiler, not an adapter.

**The identity carrier moves, and the register moves with it.** AC-473's *Verified by* reads *"an adapter-identity
assertion on the toolchain stage's dispatch record"* — and no dispatch record carries a toolchain. The verify record
carries the dated register amendment D21-42 10b authorises: the assertion reads the item's `verify_completed` rows.
AC-485's PROVISIONAL identity half (`s7d` §7d.18: *"the dispatch-event identity half has no `DispatchRecord` field to
read and the carrier is routed, RT-41"*) is discharged **for the toolchain adapter** by the same row, and a dated note
on AC-485 says so; its harness half stays with RT-41's carrier. **`s8`'s own sentence** *"the verify stage's
toolchain is rk-adapter-toolchain-rust"* is re-pointed at `verify_completed` by `s8j` (the plan's instrument
amendments), not here. **D17-4 closes** in the tracker commit, naming the record.

### 8g.5 `s7d` §7d.18's verify block, claimed whole (D21-42 10c)

Each id has a named instrument, or a named reason in the verify record. **No id is reached only by the crate-wide
floor.**

| id | instrument (module-qualified, `--test it`) | built on | residue, named |
| --- | --- | --- | --- |
| **AC-320** | `verify_pass::ac320_unresolved_inputs_refuse_before_any_port_call` | a counting `Toolchain` double: zero calls, zero gate rows | its *Verified by* names *"zero `process_spawned` events"*, and **no such kind exists** (`grep -c process_spawned crates/rk-types/src/event_kind.rs` → **0** at `ca8f387`). The record carries a dated *Verified by* note substituting the port-call count — no spawn is possible without a port call — rather than minting a kind for one assertion |
| **AC-321** | `verify::ac321_replay_reads_the_path_list_from_verify_completed_alone`; `verify::ac321_replay_property_over_100_synthetic_ranges` | §8g.4's payload; the replay reads no working tree | — |
| **AC-338** | `verify_pass::ac338_restored_mutation_is_unverifiable` | the mutation harness below; a fixture suite that restores the mutated file mid-run (F17's shape); the post-run content hash of the mutated region compared to the applied mutation | — |
| **AC-339** | `verify_pass::ac339_mutation_runs_in_a_disposable_worktree` | an ephemeral worktree created and **removed on every path** (ADR-0162 decision 10's precedent); the run-owned worktree's `tree_hash` equal before and after; `git status --porcelain` empty | — |
| **AC-341** | `verify_pass::ac341_the_pass_calls_no_tracker_method` (a counting `Tracker` in `Ports`, which `run.rs` already builds as `tracker: Arc::new(rk_ports::fakes::BoardTracker::default())`); the static half stays `verify_reads_no_suite_verdict_rule`'s, inside `ac335_…` | the port-call assertion | **the *"CI is green and verify is `Err`"* integration half cannot be constructed**: the `Tracker` port has `project` and `inbox` and no status surface, and its module says *"Fake-only in Phase 1; GitHub is Phase 3"*. Named in the record as Phase 3's (AC-508), not claimed |
| **AC-348** | `selection::ac348_selection_is_byte_identical_over_200_change_sets` (Rust adapter); `verify_pass::ac348_selection_is_recomputed_from_verify_completed` | `s8c`'s affected-set function over a pinned `cargo metadata` fixture — **it must be a pure function of (change set, metadata, policy version)**; if `s8c` built it impure, stop (AC-347 requires *"a pure function"*) | — |
| **AC-349** | `verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both`; `doctor::ac349_equal_specificity_claim_is_a_doctor_row` | a claims binder over each configured toolchain's declared path claims — a `claims` column the verify record adds to `s8c`'s per-toolchain table; the doctor row `RK-D-TOOLCHAIN-CLAIM-TIE` | — |
| **AC-353** | `verify_pass::ac353_killed_verify_resumes_to_the_same_decision` | a real `redkiln` over a fixture whose `[adapters] toolchain_programs` names `rk-fixture-toolchain` rigged to block; the tree killed through `rk-containment`; re-entry reaches the same decision, `reconcile` ran on entry, resume read the tree and the AC-321 replay did not | — |
| **AC-355** | `selection::ac355_lockfile_change_is_whole_suite_and_one_file_is_affected` (Rust adapter) | two fixtures asserted on basis **and** selected-test count | *"held in CI"* is true once `ci.yml` runs green, which is `s8j`'s (D21-42 14a) |
| **AC-359** | `verify_pass::ac359_drift_voids_the_answer_and_recomputes_the_ledger` | a verify-opened AC-357 gate (question `verify.ac-discriminates`); a commit lands between open and answer; the answer is void, the gate re-opens with the drift attached, and the ledger is recomputed | AC-357 is a HUMAN GATE and is **not claimed**; its machine-checkable part (*"the gate node is created with those three payload fields"*) is exercised inside this test and named so |

**Also discharged by the pass, beside the block**: AC-331 (`verify_pass::ac331_timeout_and_kill_are_unverifiable`),
AC-344's `cannot_run` mapping (`verify_pass::ac344_missing_binary_is_cannot_run`), and AC-347's zero-dispatch half
(`verify_pass::ac347_the_pass_dispatches_nothing`) with its lint run over the **shipped** walking producers
(`verify::ac347_shipped_verify_producers_carry_no_agent_step`, which asserts a non-zero count walked — D21-42 13a's
rule). **Still ARMED, unchanged**: AC-350 (a composite `Toolchain`, R-8), AC-317 (Step 8).

**The mutation harness AC-338 and AC-339 need is ADR-0104 decision 1's, built here** — *"The harness applies the
patch at the candidate SHA, runs the suite, reverts, and records the outcome. It is deterministic and dispatches
nothing."* Two records bar the obvious build and are moved by record, not worked around:

- ADR-0105 clause 5 enumerates the wrapper's writes and says *"Anything not listed is a successor"*; `git apply` is
  not listed, and `crates/rk-git/src/git_exec.rs` says *"Anything not here is a successor record, and adding a
  variant is that decision made visible"*. **R3 (§8g.8) supersedes clause 5 in part** to add `apply` (and `tag`,
  §8g.7). The patch is applied **only** in the pass's ephemeral worktree, never the run-owned one.
- ADR-0162's forbids bullet includes *"forbids applying a mutation patch from `rk-cli`; `ReproKind::Mutate` is refused
  by name until a record adds the capability"*. `s8c`'s 1b record supersedes that bullet's seventh-method half. The
  verify record states the disposition of the patch clause by quotation: **if `s8c`'s record did not reach it, R1
  supersedes it in part for the verify gate's declared AC mutations alone**; the review pass's `ReproKind::Mutate`
  arm stays refused by name either way, and the record says so.

### 8g.6 The four demands restored (D21-42 6a)

D21-42 6a: *"Reversed for five demands … **Supersedes D21-41(c)/(d)'s narrowings.**"* The four this stage owns walk
the **shipped** catalog activities under `s8d`'s resolver; nothing here re-writes an activity's step list, so no
catalog version moves for them (ADR-0072's rule bites only on an edit). The producers record (§8g.8, R4) fixes what
the four verbs do on the Rust adapter, through `s8c`'s registry method:

- **SmokeResult — `smoke-test`** (`toolchain:build`, then `toolchain:smoke` with *"`checklist = "checklist:smoke"`"*).
  The Rust `smoke` row runs **the binaries the build step just produced in the run-owned worktree** — never the
  operator's installed `redkiln` and never the checkout's `target/` — against a declared checklist, in a **scratch
  repository outside the run-owned worktree** (inside it, D21-42 9's landing would stage the scratch files). The
  checklist is authored here, at `.redkiln/checklists/smoke.toml`, its id already on the design's checklist registry
  (`spikes/p9/registry.toml`'s `[checklists]` carries `"smoke"`); each item is `argv`, `cwd` (`scratch`),
  `expect_exit` and one stdout predicate, and **no item dispatches** (no `run`, no network). The first cut, each
  expectation measured on the built binary before it is written down: `redkiln --version`; `redkiln-migrate
  --version`; `redkiln --help` naming the nine verbs (*"The nine verbs, and no others"*, `cli.rs`); `redkiln -C
  <scratch> init` then `status`; `redkiln-migrate` with no arguments exiting clap's `2` (RT-53). The spike registry
  marks `smoke` *"`live = true`"*, and the design's rule is that a live verb *"implies `memo = false` on every step
  using it"*; the record keeps that.
- **ConfirmationResult — `confirmation-test`**. `toolchain:test` with *"`selection = "artifact:ReproCase.command"`"*
  goes through `selection_of` (ADR-0162) and the resolver; the check reads *"`result.status == 'passing'`"*; a
  non-passing result opens *"`question = "confirmation.not-fixed"`"*. The `ReproCase` comes from `minimal-repro` at
  `s8d`'s step grain (it has two agent steps, `attempt` and `reduce`). The fixture proves both arms: passing at the
  fix sha emits; not passing opens the gate and emits nothing.
- **ScanReport — `sca-scan`** (`toolchain:sca_scan`; check *"`report.critical == 0 and report.high == 0`"*; gate
  `scan.accept-findings`). The Rust row runs `cargo audit --json` (0.22.2 on this box) and `cargo deny check
  advisories` (0.20.2; `deny.toml` is committed). **Severity** maps RustSec's CVSS base score onto the report by the
  CVSS v3 qualitative bands — `critical` ≥ 9.0, `high` 7.0–8.9, `medium` and `low` below — and **an advisory with no
  CVSS is counted in `unrated`, never as 0 and never folded into a band** (6a in terms). The **advisory-DB commit** is
  read from the report's own database member; if the installed tool emits none, the scan **refuses by name** rather
  than record an unknown database. The measurement of that member on this box is the stage's first act on the verb.
- **BuildArtifact — `build-package`** (`toolchain:build`, check, `toolchain:package`). The Rust `package` row is
  `cargo build --release --locked` plus the sha256 of each **shipped** binary, through `rk_types::digest::sha256` (the
  plain hash, not `sha256_lp`, so an independent `sha256sum` agrees — ADR-0116 decision 6's *"No second digest
  algorithm"*). **The shipped set is declared, not globbed**: a release build also links `rk-fixture-child-cli`,
  `rk-fixture-toolchain` and `rk-fixture-child-claude` (`[[bin]]` rows in `rk-cli`'s and `rk-adapter-claude`'s
  manifests), and a glob over `target/release/` would digest test fixtures as the release. The set is `redkiln` and
  `redkiln-migrate`, in the Rust row of `s8c`'s per-toolchain table with its warrant (AC-441: the migrator is *"a
  separate binary"*).

**Two registries the gates need.** `answer` validates against `&QuestionRegistry::default()` at `ca8f387`, and
`.redkiln/` has no `questions.toml`. The three questions this stage opens — `confirmation.not-fixed`,
`scan.accept-findings` (both authored in `spikes/p9/questions.toml`) and `verify.ac-discriminates` (new, §8g.5) — are
written into `.redkiln/questions.toml`, design's registry of that name. **Loading it on the binary's `answer` path is
not this stage's**: D21-41 (f)'s pack-declared question is `s8j`'s, after this stage. Until then the fixtures reach
`answer` through `run_with`'s registry parameter — *"The registry is a parameter"* (`verbs/answer.rs`) — and the
cross-stage dependency is reported, not absorbed.

### 8g.7 `tag-release` — a CLI's release, entered by ADR-0114's trial (D21-42 6b)

**What ReleaseRecord is.** 6b: *"a release tag on the landed commit, plus the BuildArtifact digest and the rollback
target (the previous release tag). The tag is local and never pushed by the run."* The tag record (§8g.8, R5) fixes:

- **The tag** — annotated, named `rk-release/<item>`, on the landed commit D21-41 (b) defines (*"A kernel-made content
  checkpoint commit on `rk-run/<item>`"*). Re-running for the same item at the same commit is idempotent; at a
  different commit it refuses by name.
- **The rollback target** — the nearest `rk-release/*` tag on an ancestor of the landed commit, read through the
  existing `GitOp::Log` (`--decorate-refs=refs/tags/rk-release/ --simplify-by-decoration`), so no new read op is
  owed. **A first release names its target explicitly as `none`**; it is never absent and never defaulted to the
  landed sha.
- **The write** — a tag is a git write, so it is **`GitOp::Tag`**, R3's second addition to ADR-0105 clause 5, and it
  goes through the one wrapper. The spike registry's `release_record` verb (*"`outputs = "ReleaseRecord", live =
  false`"*) is served **by the composition root through `rk-git`**, never by a toolchain adapter shelling git
  (ADR-0105's *"Forbids a wrapper write not on clause 5's list"*). The tag target must be a commit on the run-owned
  branch (`authorise_tier1_write`'s ownership pair); anything else refuses.
- **Never pushed** — `GitOp` has no push variant and R3 adds none; a test asserts the write set.

**The activity** is agent-free: `toolchain:release_record` (inputs `artifact:BuildArtifact`, `artifact:Changeset.sha`),
a check that the record names a tag and a rollback target, and an emit. **Agent-free is the property that makes its
trial cheap**: it cannot dispatch, so its run needs no spend gate.

**How it enters: ADR-0114 decision 3, read whole.** *"An activity is promoted from the `extended` file to the shipped
catalog only with a ledger entry: it must have run at least once … in a run whose id the promotion decision (AC-114's
`curation.decision`) cites … the promotion route is a one-run, one-activity trial the operator declares, not a pack
reference."* And its Negative consequence: *"The extended-promotion route is a small new CLI/config surface (Phase 2,
with `rk-plugin-gen`)."* So:

- **`tag-release` is authored into `.redkiln/catalog-extended/`**, not `.redkiln/catalog/`. AC-110 is unchanged: an
  enabled pack referencing it, or selection reaching it, is a load-time configuration error, and the extended
  directory contributes **zero** entries to the resolved catalog.
- **The trial declaration is data on an item record, not a CLI flag** — `[trial] activity = "tag-release"` on a
  dedicated trial item; `run` admits exactly that one extended activity for exactly that item's one run, and the
  AC-400 help snapshot does not move. R5 records this as the Phase-1 form of the route ADR-0114 placed in Phase 2,
  with its limits stated.
- **The ledger entry exists already**: AC-532's executed ledger folds `activity_committed` rows into
  `executed: { first, last }` *"per `activity_id@version`, each stamp holding all three of `{ run_id, item, at }`"*.
- **At this stage the trial runs under the fixture harness only** —
  `tag_release::trial_runs_one_activity_with_zero_dispatches`, in a temporary repository, asserting one activation,
  zero `dispatch` and `dispatch_completed` rows, a ReleaseRecord `artifact.write`, and a local tag the fixture's bare
  "remote" never receives.
- **The trial's REAL run on this repository is DEFERRED to `s8j`, explicitly.** It needs a landed commit and a
  journalled BuildArtifact for its item, which only a lane run produces, and it creates a tag in the operator's
  repository — an operator moment, like a push. **The promotion that follows is AC-114's HUMAN GATE**, and it must
  precede any lane run whose release stage is to select `tag-release`; AC-114's residue lint (*"a non-empty
  `curation.decision` id"*) exists nowhere on the tree (`grep -rn curation .redkiln crates` → nothing at `ca8f387`).
  Both are reported to `s8j`.

**Why selection would pick it — a free residual, and R5 decides it rather than leaving it.** ReleaseRecord's four
shipped producers (`rolling-deploy`, `blue-green-deploy`, `canary-release`, `flag-progressive-rollout`) all run
`toolchain:deploy`, and no existing facet distinguishes a repository whose release is a tag. R5's options: **(a)** a
`declared` facet (the `surfaces` precedent: *"read from a checked-in declaration map"*) keyed in `tag-release`'s
`applicable_if` — `applicable_if` is not a `require` row, so a declared facet may key it — with `tag-release` first in
the `deploy-pattern` family, and **no shipped deploy activity edited**; **(b)** verb availability as a selection-time
admissibility condition, which is the long-term shape and is rejected **here** because it moves every stage's
selection on the lane `s8` runs. Recommended: (a). Checkpoint 8g asserts the **outcome**
(`tag_release::selection_picks_tag_release_over_the_deploy_patterns`), not the mechanism.

**The release goal is amended, not read.** bug-standard says *"The fix is in production reversibly"* and story says
*"In production, reversibly"*. A run that makes a local tag and pushes nothing puts nothing in production, so a
reading would leave a false sentence in the pack. R5 amends them to **`The fix is tagged for release, reversibly`** and
**`Tagged for release, reversibly`**. The packs carry no `version` key (measured), and ADR-0072 governs activity edits,
so no version moves; the edit is listed in the pack changelog `s8b` opened for D21-42 11a — `s8b` §8b.5 creates
`.redkiln/CHANGELOG.md` and says *"Nothing names the file, so the path is chosen here and said"*. **If that file is
absent at Checkpoint 8f's close, stop**: this stage does not choose a second path. The `tag-release` addition takes
its row there too.

### 8g.8 The records this stage writes

Each at the next free id **at writing**, `proposed`, in ADR-0085's form with `### Option — <name>` headings (D21-42
24a), a `## Consequences` carrying **What this forbids**, and a slug fixed here so the checkpoint can find it without a
number:

| | slug | decides | supersedes, amends |
| --- | --- | --- | --- |
| R1 | `the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed` | §8g.2, §8g.4, §8g.5: the pass and its composition; `KindSource::Pass`; `verify_completed`; the verify block's routing, each id's instrument or reason; the cold wall time; the claims column; the mutation harness | ADR-0162's patch clause in part (§8g.5's condition); R-9 and RT-6 discharged; register: AC-473 *Verified by* (10b), AC-320 and AC-485 dated notes; design §5's table row; D17-4 closed |
| R2 | `the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion` | §8g.3 | D17-3 closed; the AC-139 default named as owed; `PROGRESS.md` row 5 corrected in the tracker commit |
| R3 | `the-rk-git-write-set-gains-tag-and-apply` | `GitOp::Tag`, `GitOp::Apply`, each with its scope | **ADR-0105 clause 5 in part**; ADR-0105 keeps `status: accepted` and gains this id in `superseded_by` |
| R4 | `the-four-verify-and-release-demands-have-producers` | §8g.6: the three verb rows, the smoke checklist's home and shape, the CVSS bands and `unrated`, the advisory-DB commit, the shipped-binary set, the questions file | executes D21-42 6a, which already supersedes D21-41 (c)/(d)'s narrowings |
| R5 | `tag-release-is-the-cli-release-and-enters-by-trial` | §8g.7: ReleaseRecord's shape, the tag name and rollback target, the extended directory, the item-record trial, the selection mechanism, the goal amendment | executes D21-42 6b under ADR-0114 decision 3 |

🛑 **A change to a `## Decision` sentence or a forbids bullet is a supersession, never an erratum** (ADR-0092 rule 1).
R3 is the only record here that moves an accepted Decision sentence outright; R1's ADR-0162 disposition is conditional
and stated by quotation.

### 8g.9 Red first — the named tests and the mutants

Every test below is authored, run, and **watched fail for its own reason** before the code that makes it pass —
golden constraint 12 — and the red output is quoted in the commit that lands it. **Sixty-three new tests**, all under
`--test it`: `rk-types` 8, `rk-kernel` 8, `rk-git` 4, `rk-adapter-toolchain-rust` 10, `rk-cli` 33. Checkpoint 8g
asserts each by its module-qualified name and asserts the exact count per crate; the lists are there and are not
repeated here.

**The mutants**, each authored, run **from Python and never from the shell** (a backtick once swallowed a restore),
and each must turn its named test red. A survivor blocks the close.

| # | mutant | must turn red |
| --- | --- | --- |
| M1 | the pass records the configured list's first toolchain instead of the one `detect()` chose | `verify_pass::verify_completed_names_the_adapter_detect_chose` |
| M2 | `Ok` with `failed > 0` mapped to green | `verify_pass::red_suite_is_a_failed_verify_never_green` |
| M3 | `Timeout` mapped to a failed verify | `verify_pass::ac331_timeout_and_kill_are_unverifiable` |
| M4 | the pass keyed on `resolved.stage == "verify"` | `verify_pass::selected_by_required_kind_not_stage_name` (its fixture pack names the stage `check`) |
| M5 | `verify_completed` written for a killed run | `verify::no_verify_completed_for_a_run_that_did_not_complete` |
| M6 | `paths` dropped from the payload, recomputed from the tree on replay | `verify::ac321_replay_reads_the_path_list_from_verify_completed_alone` |
| M7 | an unrated advisory counted as 0 | `sca::unrated_advisory_is_counted_unrated_never_zero` |
| M8 | the high band opened at CVSS 7.1 | `sca::cvss_band_boundaries_map_to_critical_and_high` |
| M9 | the shipped set read by globbing `target/release/` | `package::build_artifact_digests_exactly_the_shipped_bins` |
| M10 | the rollback target defaulted to the landed sha | `tag_release::rollback_target_is_the_previous_release_tag` |
| M11 | the post-run content hash compare removed | `verify_pass::ac338_restored_mutation_is_unverifiable` |
| M12 | the patch applied in the run-owned worktree | `verify_pass::ac339_mutation_runs_in_a_disposable_worktree` |
| M13 | an equal-specificity tie resolved to the first claim | `verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both` |
| M14 | the affected set built by iterating a `HashMap` | `selection::ac348_selection_is_byte_identical_over_200_change_sets` (its generator permutes input order) |
| M15 | a failing smoke item's exit code ignored | `smoke::a_failing_checklist_item_fails_the_smoke_result` |
| M16 | `KindSource::Pass` read as `Activity` by the selector | `selection::a_pass_sourced_kind_selects_no_catalog_producer` |

### 8g.10 Checkpoint 8g

**What this checkpoint asserts:**

- **Five records exist, each once, in ADR-0085's form**, and R3's supersession of ADR-0105 is recorded in both
  directions.
- **`verify_completed` is a tier-2 kind** in `rk-types` and in design §5's table, and AC-473's *Verified by* names it.
- **`AcceptanceCriterion` is exported and snapshotted, AC-133's registry exists and covers it, and `SpecAc` is gone.**
- **The verify pass is the one TestReport route**, reached by `verbs::run::run`, selected by kind, whole-suite
  through the port, and every id of the verify block has its named test.
- **The four demands have producers**, and each writes one `artifact.write`.
- **`tag-release` sits in the extended directory, trials under the fixture harness with zero dispatches, and no
  `rk-release/*` tag appears in this repository** as a side effect of the block.
- **The whole suite, fmt and clippy are green**, and **Checkpoints 5, 6, 7b, 7b-ii, 7d and 8f re-run whole** from
  their stage files' own bytes.

**BEFORE** (PIA-035), each failing on the untouched tree: (2) finds no record; (3) finds no `verify_completed` row;
(4) finds **17** `SpecAc` sites. **Checkpoint 7c is not re-run here** (§8g.1: its live probe is `s8j`'s).

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`,
# MODULE-QUALIFIED and `--exact`, and asserts each test's own `... ok` line AND an exact count: a bare name
# under `--exact` runs zero tests and exits 0.

# ---- (0) THE WORKTREE PATH SET AND THE RELEASE-TAG SET, read FIRST, before any cargo run.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
tags_before=$(git tag -l 'rk-release/*'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git tag -l could not run (rc=$rc)"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: run 'pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project' first — ten node tests fail at preflight without it and the whole-suite line would be red for a reason unrelated to this stage"; exit 1; }

# ---- (1) BUILD-OR-FAIL, before anything reads target/ (Halt 1).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8g NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE FIVE RECORDS, EACH EXACTLY ONCE, EACH IN ADR-0085's FORM (D21-42 24a). Found by SLUG, because
# ---- this file pre-assigns no number. BEFORE: every glob matches nothing on the untouched tree.
for slug in the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion the-rk-git-write-set-gains-tag-and-apply the-four-verify-and-release-demands-have-producers tag-release-is-the-cli-release-and-enters-by-trial; do
  set -- references/adr/[0-9][0-9][0-9][0-9]-"$slug".md
  [ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: no single record carries the slug $slug"; exit 1; }
  num=${1#references/adr/}; num=${num%%-*}
  grep -qxF "id: \"$num\"" "$1" || { echo "FAIL: $1's frontmatter id is not $num"; exit 1; }
  grep -qxE 'status: (proposed|accepted)' "$1" || { echo "FAIL: $1 is neither proposed nor accepted"; exit 1; }
  n=$(grep -c '^### Option — ' "$1")
  [ "$n" -ge 2 ] || { echo "FAIL: $1 carries $n '### Option — ' headings; ADR-0085 and D21-42 24a need the considered options in that form"; exit 1; }
  grep -qF '**What this forbids**' "$1" || { echo "FAIL: $1 has no What this forbids block"; exit 1; }
done
set -- references/adr/[0-9][0-9][0-9][0-9]-the-rk-git-write-set-gains-tag-and-apply.md
wnum=${1#references/adr/}; wnum=${wnum%%-*}
grep -qE '^supersedes: \[.*"0105".*\]$' "$1" || { echo "FAIL: the write-set record does not list 0105 in supersedes — a new wrapper write without one is ADR-0105's forbids bullet broken"; exit 1; }
grep -qE "^superseded_by: \[.*\"$wnum\".*\]$" references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md || { echo "FAIL: ADR-0105's superseded_by does not carry $wnum — the supersession is recorded in one direction only"; exit 1; }
set -- references/adr/[0-9][0-9][0-9][0-9]-the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed.md
vf="$1"
grep -qF 'applying a mutation patch from `rk-cli`' "$vf" || { echo "FAIL: the verify record does not quote ADR-0162's patch clause — its disposition is stated nowhere (s8g §8g.5)"; exit 1; }

# ---- (3) verify_completed: THE DESIGN ROW, AND AC-473's VERIFIED-BY NAMES IT (ADR-0116 decision 5; D21-42 10b).
grep -qE '^\| `verify_completed` \| 2 \|' references/2026-09-08-architecture-design.md || { echo "FAIL: design §5's event table has no tier-2 verify_completed row — the variant and the row land in one change"; exit 1; }
blk=$(awk '/^\*\*AC-473\*\* /{p=1} p&&/^\*\*AC-521\*\*/{exit} p' references/2026-09-08-acceptance-criteria.md)
test -n "$blk" || { echo "FAIL: AC-473's block could not be read from the register"; exit 1; }
printf '%s\n' "$blk" | grep -qF 'verify_completed' || { echo "FAIL: AC-473 still asserts identity on a dispatch record no toolchain writes (D21-42 10b's register amendment is missing)"; exit 1; }

# ---- (4) THE SCHEMA: AcceptanceCriterion exported, snapshotted and tracked; AC-133's registry; SpecAc GONE.
snap=crates/rk-types/tests/it/snapshots/exported__AcceptanceCriterion.snap
test -f "$snap" || { echo "FAIL: no AcceptanceCriterion snapshot — the type is not in registry::EXPORTED"; exit 1; }
git ls-files --error-unmatch "$snap" >/dev/null 2>&1 || { echo "FAIL: the snapshot exists and is not tracked"; exit 1; }
pending=$(find crates/rk-types/tests/it/snapshots -name '*.snap.new'); rc=$?
[ "$rc" -eq 0 ] && [ -z "$pending" ] || { echo "FAIL: an unaccepted snapshot is pending — a schema change nobody reviewed:"; printf '%s\n' "$pending"; exit 1; }
# CODE LINES ONLY, anchored to grep's `path:LINE:` prefix. BEFORE: 17 sites at ca8f387.
raw=$(grep -rnE '\bSpecAc\b' crates --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the SpecAc sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: the kernel-local SpecAc survives on a code line (D21-42 10a retires it):"; printf '%s\n' "$code"; exit 1; }
# Synthetic control: the same filter KEEPS a code line and DROPS a comment line, or silence above proves nothing.
ctl=$(printf '%s\n%s\n' 'crates/x/src/a.rs:1:pub struct SpecAc {' 'crates/x/src/a.rs:2:/// SpecAc was here' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: the code-line filter's control kept $ctl of 2 lines, expected exactly the code line"; exit 1; }

# ---- (5) THE NAMED TESTS, per crate, each by its own `... ok` line and an EXACT count.
T_TYPES="event_kinds::verify_completed_is_tier_two_and_wired verify_types::verify_completed_payload_refuses_an_absent_member acceptance_criterion::ac137_criterion_without_gwt_or_verification_is_accepted acceptance_criterion::mutation_declaration_round_trips_declared_and_not_feasible acceptance_criterion::a_prose_only_mutation_is_refused_at_ingest acceptance_criterion::acceptance_criterion_is_in_the_exported_registry judgement_registry::ac133_every_exported_enum_is_registered judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails"
T_KERNEL="verify::verify_completed_is_journalled_once_per_completed_test_port_call verify::no_verify_completed_for_a_run_that_did_not_complete verify::ac321_replay_reads_the_path_list_from_verify_completed_alone verify::ac321_replay_property_over_100_synthetic_ranges verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both verify::ac347_shipped_verify_producers_carry_no_agent_step verify::the_ledger_reads_rk_types_acceptance_criterion selection::a_pass_sourced_kind_selects_no_catalog_producer"
T_GIT="gitops::tag_is_on_the_enumerated_write_set gitops::apply_is_on_the_enumerated_write_set gitops::the_write_set_has_no_push gitops::tag_refuses_a_target_off_the_run_owned_branch"
T_RUST="registry::smoke_package_and_sca_scan_route_through_the_registry_method sca::cvss_band_boundaries_map_to_critical_and_high sca::unrated_advisory_is_counted_unrated_never_zero sca::advisory_db_commit_is_recorded_or_the_scan_refuses package::build_artifact_digests_exactly_the_shipped_bins package::binary_digest_equals_an_independent_sha256 smoke::a_failing_checklist_item_fails_the_smoke_result smoke::smoke_runs_in_a_scratch_repository_outside_the_worktree selection::ac348_selection_is_byte_identical_over_200_change_sets selection::ac355_lockfile_change_is_whole_suite_and_one_file_is_affected"
T_CLI="verify_pass::selected_by_required_kind_not_stage_name verify_pass::writes_one_keyed_test_report_with_one_artifact_write verify_pass::verify_completed_names_the_adapter_detect_chose verify_pass::node_detected_fixture_records_the_node_adapter verify_pass::red_suite_is_a_failed_verify_never_green verify_pass::ac331_timeout_and_kill_are_unverifiable verify_pass::ac344_missing_binary_is_cannot_run verify_pass::ac347_the_pass_dispatches_nothing verify_pass::ac320_unresolved_inputs_refuse_before_any_port_call verify_pass::ac338_restored_mutation_is_unverifiable verify_pass::ac339_mutation_runs_in_a_disposable_worktree verify_pass::ac341_the_pass_calls_no_tracker_method verify_pass::ac348_selection_is_recomputed_from_verify_completed verify_pass::ac353_killed_verify_resumes_to_the_same_decision verify_pass::ac359_drift_voids_the_answer_and_recomputes_the_ledger verify_pass::ac485_config_edit_mid_run_does_not_move_the_verify_adapter verify_pass::the_lane_reaches_the_pass_through_verbs_run_run demands::smoke_result_is_produced_through_the_registry_verb demands::confirmation_result_passes_at_the_fix_sha demands::confirmation_not_fixed_opens_its_gate demands::scan_high_finding_opens_scan_accept_findings demands::scan_clean_emits_without_a_gate demands::build_artifact_carries_release_digests demands::every_restored_demand_writes_one_artifact_write tag_release::release_record_tags_the_landed_commit_locally tag_release::rollback_target_is_the_previous_release_tag tag_release::first_release_names_its_rollback_target_as_none tag_release::release_record_carries_the_build_artifact_digest tag_release::the_tag_is_never_pushed tag_release::trial_runs_one_activity_with_zero_dispatches tag_release::an_enabled_pack_referencing_the_extended_activity_is_refused tag_release::selection_picks_tag_release_over_the_deploy_patterns doctor::ac349_equal_specificity_claim_is_a_doctor_row"
NEW=0
for spec in "rk-types@8@$T_TYPES" "rk-kernel@8@$T_KERNEL" "rk-git@4@$T_GIT" "rk-adapter-toolchain-rust@10@$T_RUST" "rk-cli@33@$T_CLI"; do
  crate=${spec%%@*}; rest=${spec#*@}; want=${rest%%@*}; names=${rest#*@}
  got=$(printf '%s\n' $names | grep -c .)
  [ "$got" -eq "$want" ] || { echo "FAIL: $crate's list carries $got names, this file says $want — the list and the prose disagree"; exit 1; }
  out=$(cargo test -p "$crate" --test it -- $names --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $crate's named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in $names; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $crate: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $crate's selection did not run exactly $want tests — a filter that matches nothing is a green"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
  NEW=$((NEW + want))
done
[ "$NEW" -eq 63 ] || { echo "FAIL: $NEW named tests ran, §8g.9 says sixty-three"; exit 1; }

# ---- (6) NO STAGE-NAME LITERAL IN rk-cli/src (ADR-0162 decision 5) — a FLOOR over CODE lines beneath the
# ---- behavioural `selected_by_required_kind_not_stage_name` above, whose fixture names its verify stage `check`.
raw=$(grep -rnE 'stage[[:space:]]*(==|!=)[[:space:]]*"|"[[:space:]]*(==|!=)[[:space:]]*[A-Za-z_.]*stage\b' crates/rk-cli/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the stage-literal sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: resolved.stage compared to a string literal on a CODE line (ADR-0162 decision 5):"; printf '%s\n' "$code"; exit 1; }
ctl=$(printf '%s\n' 'crates/rk-cli/src/verify_pass.rs:9:    if resolved.stage == "verify" {' | grep -E 'stage[[:space:]]*(==|!=)[[:space:]]*"' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: the stage-literal pattern does not catch its own synthetic control"; exit 1; }

# ---- (7) THE SHIPPED FILES THIS STAGE AUTHORS, where the records put them.
test -f .redkiln/catalog-extended/tag-release.toml || { echo "FAIL: tag-release is not in the extended directory (ADR-0114 decision 3)"; exit 1; }
test ! -e .redkiln/catalog/tag-release.toml || { echo "FAIL: tag-release is in the SHIPPED catalog before a trial ran and a promotion decision cited it (ADR-0114 decision 3; AC-110)"; exit 1; }
test -f .redkiln/checklists/smoke.toml || { echo "FAIL: no smoke checklist — checklist:smoke resolves to nothing"; exit 1; }
test -f .redkiln/questions.toml || { echo "FAIL: no .redkiln/questions.toml"; exit 1; }
for q in confirmation.not-fixed scan.accept-findings verify.ac-discriminates; do
  grep -qxF "[question.\"$q\"]" .redkiln/questions.toml || { echo "FAIL: question $q is not declared — the gate that opens it could never be answered"; exit 1; }
done
grep -qxF 'goal = "The fix is tagged for release, reversibly"' .redkiln/packs/bug-standard.toml || { echo "FAIL: bug-standard's release goal is not the amended wording (R5)"; exit 1; }
grep -qxF 'goal = "Tagged for release, reversibly"' .redkiln/packs/story.toml || { echo "FAIL: story's release goal is not the amended wording (R5)"; exit 1; }
for p in bug-standard story; do
  n=$(grep -cE '^goal = ".*[Pp]roduction' ".redkiln/packs/$p.toml")
  [ "$n" -eq 0 ] || { echo "FAIL: $p.toml still promises production, which a run that pushes nothing cannot reach"; exit 1; }
done
test -f .redkiln/CHANGELOG.md || { echo "FAIL: s8b's pack changelog is absent — ADR-0072's row for this stage's pack and catalog edits has nowhere to go"; exit 1; }
for w in tag-release 'tagged for release'; do
  grep -qiF "$w" .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md carries no row naming '$w' (ADR-0072; R5)"; exit 1; }
done
grep -qE '^\[artifact\.TestReport\]' .redkiln/artifacts.toml || { echo "FAIL: artifacts.toml has no TestReport row"; exit 1; }
tr_row=$(awk '/^\[artifact\.TestReport\]/{p=1;next} /^\[/{p=0} p' .redkiln/artifacts.toml)
printf '%s\n' "$tr_row" | grep -qxF 'source = "pass"' || { echo "FAIL: TestReport is not pass-sourced — the catalog's five providers would still be selected beside the pass (two routes)"; printf '%s\n' "$tr_row"; exit 1; }

# ---- (8) THE COLD WALL TIME IS IN THE VERIFY RECORD, and under ADR-0140's bound (D21-42 10c).
secs=$(sed -n 's/.*cold cargo test --workspace wall time: \([0-9][0-9]*\) s.*/\1/p' "$vf" | head -1)
test -n "$secs" || { echo "FAIL: the verify record carries no cold wall-time line in the form §8g.2 fixes"; exit 1; }
[ "$secs" -lt 3600 ] || { echo "FAIL: a cold whole suite takes $secs s, at or over ADR-0140's 3600 s — every cold verify pass would be Unverifiable (golden constraint 6; halt)"; exit 1; }

# ---- (9) THE WHOLE SUITE, --no-fail-fast, rc 0, every result line green, and a FLOOR: 740 at ca8f387 plus this
# ---- stage's 63. The floor is a lower bound — s8a–s8f add tests too — never an equality.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the workspace suite is red (rc=$rc)"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked' | head -20; exit 1; }
results=$(printf '%s\n' "$out" | grep -c '^test result:')
greens=$(printf '%s\n' "$out" | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed')
[ "$results" -ge 1 ] && [ "$results" -eq "$greens" ] || { echo "FAIL: $greens of $results result lines are green"; exit 1; }
passed=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9][0-9]*\) passed.*/\1/p' | awk '{s+=$1} END {print s+0}')
[ "$passed" -ge $((740 + NEW)) ] || { echo "FAIL: $passed passed, under the floor of $((740 + NEW))"; exit 1; }

# ---- (10) fmt AND clippy, each by its own exit code.
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }

# ---- (11) NO RELEASE TAG WAS MADE IN THIS REPOSITORY BY THE BLOCK — the tag tests run in temporary repositories.
tags_after=$(git tag -l 'rk-release/*'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git tag -l could not run after the suite (rc=$rc)"; exit 1; }
[ "$tags_before" = "$tags_after" ] || { echo "FAIL: the block created or removed an rk-release tag in the operator's repository:"; diff <(printf '%s\n' "$tags_before") <(printf '%s\n' "$tags_after"); exit 1; }

# ---- (12) THE PROVISIONAL CHECKPOINTS, RE-RUN WHOLE FROM THEIR STAGE FILES' OWN BYTES (§8g.1): 5 (rk-types), 6
# ---- (rk-git), 7b and 7b-ii (rk-kernel), 7d (rk-cli), and 8f, the entry gate. Extracted, never retyped. A block with no end
# ---- marker gets one appended, so rc=0 can be told apart from a block that stopped early. The extractor writes
# ---- under target/ — Git Bash's /tmp is invisible to Windows Python. Store handling around `doctor --rebuild`
# ---- is each block's own (the s8 amendment), and nothing in THIS block reads the store after this point.
mkdir -p target/cp8g || { echo "FAIL: could not create target/cp8g"; exit 1; }
EXTRACT='import sys
F = chr(96) * 3
src, mode, key, out = sys.argv[1:5]
lines = open(src, encoding="utf-8").read().split("\n")
blocks = []
i = 0
while i < len(lines):
    if lines[i] == F + "bash":
        j = i + 1
        while j < len(lines) and lines[j] != F:
            j += 1
        blocks.append((i, lines[i + 1:j]))
        i = j + 1
    else:
        i += 1
if mode == "marker":
    hit = [b for (s, b) in blocks if any(key in l for l in b)]
else:
    at = [n for n, l in enumerate(lines) if key in l]
    hit = [b for (s, b) in blocks if at and s > at[0]][:1]
if len(hit) != 1:
    print("EXTRACT-FAIL", len(hit))
    sys.exit(3)
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(hit[0]) + "\n")
print("EXTRACTED", len(hit[0]))'
for spec in "cp5@runbook/phase-1/s5-workspace-ci-rk-types.md@anchor@**Checkpoint 5.** All of:" "cp6@runbook/phase-1/s6-store-and-git.md@anchor@**Checkpoint 6.** All of:" "cp7b@runbook/phase-1/s7b-kernel.md@anchor@### 7b.14 Checkpoint 7b" "cp7bii@runbook/phase-1/s7b-ii-gates.md@anchor@### 7b-ii.13 Checkpoint 7b-ii" "cp7d@runbook/phase-1/s7d-cli-and-migrate.md@marker@CP7D-END-OF-BLOCK reached" "cp8f@runbook/phase-1/s8f-landing-and-rendering.md@marker@CP8F-END-OF-BLOCK reached"; do
  name=${spec%%@*}; rest=${spec#*@}; file=${rest%%@*}; rest=${rest#*@}; mode=${rest%%@*}; key=${rest#*@}
  test -f "$file" || { echo "FAIL: $file does not exist — Checkpoint $name cannot be re-run (Halt 8)"; exit 1; }
  ex=$(python -c "$EXTRACT" "$file" "$mode" "$key" "target/cp8g/$name.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: could not extract exactly one block for $name: $ex"; exit 1; }
  printf '\necho "CP8G-REPLAY-%s-END reached"\n' "$name" >> "target/cp8g/$name.sh" || { echo "FAIL: could not append $name's marker"; exit 1; }
  out=$(bash "target/cp8g/$name.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $name re-run whole exited $rc — its row stays provisional"; printf '%s\n' "$out" | grep -E 'FAIL' | head -10; printf '%s\n' "$out" | tail -15; exit 1; }
  printf '%s\n' "$out" | grep -qxF "CP8G-REPLAY-$name-END reached" || { echo "FAIL: Checkpoint $name exited 0 without reaching its last line"; exit 1; }
done

# ---- (13) THE WORKTREE PATH SET IS UNCHANGED, compared BEFORE any prune; the prune is a dry run and finds nothing.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET (the pass's ephemeral trees must be removed on every path — ADR-0162 decision 10):"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }

echo "CP8G-END-OF-BLOCK reached"
```

⚠️ **What this block cannot see, said rather than claimed away.** (6) is a floor over code lines: a stage literal
assembled by a `const`, a `matches!` over a pattern, or a helper whose name hides the comparison passes it, which is
why the behavioural test with a renamed stage is the assertion. (9)'s floor is a lower bound and cannot tell a
deleted older test from a new one; the named lists are what pin this stage's. (12) runs the 8f block with the
environment the operator gives this block; if 8f needs a variable, its own guard names it and this block fails,
which is the right direction.

### 8g.11 Acceptance set

**Claimed here, each by the named test in §8g.10 (5):** AC-320 (with its dated *Verified by* note), AC-321, AC-338,
AC-339, AC-348, AC-349, AC-353, AC-359 — **seven of the verify block whole and AC-355 on its fixtures** (its *"held
in CI"* clause is `s8j`'s 14a); **AC-341** on the port-call half, its static half cross-listed to 7b-ii; AC-331,
AC-344's `cannot_run` mapping, AC-347's zero-dispatch half and its shipped-producer lint; AC-137; AC-133 (its
instrument, built here for the first time — `s5`'s claim is corrected, not re-claimed); AC-485's toolchain identity
half.

**Named with a reason, not claimed:** AC-341's CI-green integration half (the `Tracker` port has no status surface;
Phase 3, AC-508); AC-357 (HUMAN GATE — its machine part is exercised inside `ac359_…`); AC-114 (HUMAN GATE — the
promotion is `s8j`'s); AC-139 (HUMAN GATE — the verification default is the operator's); AC-350 and AC-317 (ARMED as
before); AC-473 and AC-521 (Step 8's release checks — this stage moves AC-473's *Verified by* and claims neither).

**Cross-listed:** AC-328's whole-suite policy (the adapter side is `s8c`'s); AC-455 (discharged at 7b-ii, re-pointed
here to `rk_types::AcceptanceCriterion` with its two policy rows unchanged); AC-110 (unchanged; asserted by
`tag_release::an_enabled_pack_referencing_the_extended_activity_is_refused`).

**Owed and reported upward, because no sub-stage owns them:** the re-homing of the five displaced test-design
activities (§8g.2); the AC-139 verification default (§8g.3); the real trial run, the AC-114 promotion and its
`curation.decision` lint, and loading `.redkiln/questions.toml` on `answer`'s path (all `s8j`, §8g.6, §8g.7);
`s8h`'s `Spec` embedding `AcceptanceCriterion` unchanged (§8g.3); Checkpoint 7c's whole re-run (`s8j`, §8g.1).
