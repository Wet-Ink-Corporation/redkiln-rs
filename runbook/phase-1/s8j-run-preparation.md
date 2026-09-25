# Step 8j — Fixture proof of both lanes, CI green, and the run preparation (Phase 1, part 8, tenth sub-stage)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8j` | Phase 1 | Checkpoint 8i | Checkpoint 8j | `s8i` | `s8` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this stage exists.** `s8a`…`s8i` build what criterion (1) and (1b) need. Nothing before this stage proves the
parts work **together, as a lane, through the entry point the real runs will use**. D21-40 decision 2 authorises the
real runs *"once the sub-stages' fixture proofs pass, and not before. That includes a FakeHarness end-to-end test of
`verbs::run::run`"*. `s8e` wrote the first such test (D21-42 24c). This stage writes the proofs that the spend gate
turns on: each lane, from `triage` to its landing, through `verbs::run::run`, with the real Rust toolchain adapter
and across real process boundaries.

It also prepares the run so that `s8` has nothing left to decide:

- the two items the runs are pointed at (D21-41 (f); D21-42 18);
- the human roots those items start from (D21-41 (f): *"decided once for both lanes"*);
- the bound on the first real invocation (D21-42 19b);
- the CLI pin and the two live probes `s8h` authored (D21-42 16; D21-41 (g));
- CI green on the pushed branch (D21-42 14a);
- the amendments to `s8`'s own instruments that the re-measurement of D21-40 found, and that the sub-stages make
  necessary (§8j.8).

**The order is fixed, and every paid or published act in it is a gate.** First the fixture proofs (§8j.4). Then,
and only then, the pin move and the probes (§8j.6), which spend. Last, CI on the frozen code (§8j.7), which
publishes. **No block in this file dispatches to a real account, and no block pushes.** Checkpoint 8j reads what
the gated acts recorded; it never repeats them.

🛑 **Every push and every paid dispatch in this stage is an explicit operator go-ahead, given at that moment and for
that act.** A go-ahead for one push does not cover the next push, and a go-ahead for one probe does not cover the
next. D21-42 14a: *"Each push is confirmed with the operator."* D21-40 decision 2: *"If a real run is refused
unexpectedly, the next step is to stop and report, not to spend again."* The same rule governs the probes.

**This stage takes decisions, and says so up front.** The last three stage files that said *"this step takes no
decision"* were wrong by eighteen, eleven and one records (s7i's own correction: *"`s7g` took eighteen … `s7h` eleven"*, and s7i took `ADR-0163`). This stage writes **two records
at `proposed`**, each with the next free id at writing, and neither id is assigned here:

1. **The human-root question** (§8j.2). Decided by D21-41 (f). What is left to the record is the pack TOML spelling
   of a question and how the step-4 gate names one.
2. **The boundary hold** (§8j.3). D21-42 19b decides *what*: *"The first real fix-lane invocation stops at the
   analysis → specification boundary for the operator to review."* No record decides *how*, and no mechanism for it
   exists today.

Each record flips to `accepted` only on the operator's dated ledger line (ADR-0126's route). It also writes ledger
entries in `spikes/DECISIONS-2026-09-21.md`, the file that D21-40…D21-42 extend:

- the story candidate, with its reading of ADR-0011/E7;
- the Codex item's retirement;
- the fix-item reservation;
- the note on the fixture item;
- the (g) flip, which is the operator's.

---

### 8j.1 The items: the story, the Codex item retired, 7i's empty case moved, and the fixture item noted

**(a) The story item — `.redkiln/items/rk-story-harden-spike-pack.toml`.** D21-41 (f): *"A full lane entered at
`triage`. The candidate is to harden `spikes/p9/packs/spike.toml`, a Phase-2 story whose change stays inside
`spikes/p9/`."*

- `id = "rk-story-harden-spike-pack"`, `kind = "story"`, `stage = "triage"`.
- `[facets]`: `cos = "standard"`, `criticality = "low"`, `blast_radius = "low"`, `size = "low"`,
  `external_dep = false`.

The packet (f) option A measured why the facets must be low. With the Codex item's `external_dep = true` and
`blast_radius = "med"`, the selection gained *"five kinds"*: APIContract, ThreatModel, IntegrationReport,
RolloutPlan and FlagRecord. Low facets are the candidate's real profile, and they are not chosen to make the run
cheap.

**What "harden" means — the story's intent, not its spec.** The lane's own `specification` stage authors the
acceptance criteria, and this file must not write them in advance. The intent, stated in the item's header comment
and in the ledger row, is this: *bring `spikes/p9/packs/spike.toml` up to the contract the two shipped packs were
brought to at Step 7e, so that it loads through the kernel's pack loader and could carry a lane — without installing
it.* The measured gaps it starts from, each re-checkable:

- **No lane order.** `grep -c 'stage_order' spikes/p9/packs/spike.toml` gives **0**. `[policy] stage_order` is
  ADR-0153's only ordered stage carrier. The loader reads it as optional, so a spike item would load and then
  could not cross a single boundary.
- **No `stale_policy`.** Both shipped packs record one per cost class, with decision 5's warrant (ADR-0117). The
  spike pack records none.
- **No gate deadline scalar.** Both shipped packs carry `gate_deadline_secs`; the spike pack does not. The value
  belongs to the RT-57 record `s8h` writes (D21-42 19c).
- **Its twelve kinds are all registered and provided.** Every kind its baselines and require rows name is in
  `.redkiln/artifacts.toml` and has at least one provider in `.redkiln/catalog/`. The five `spike-*` /
  `technical-spike` activities are installed. The hardening keeps this true under a mechanical check instead of by
  inspection. That was measured 2026-09-24 over ProblemStatement, CompetitorAnalysis, SpikeCharter, SpikePlan,
  Estimate, ExperimentResults, ReviewVerdict, SpikeReport, DecisionRecord, ClosureRecord, KBAtoms and Retrospective.

🛑 **It is not installed.** ADR-0011: *"Phase 1 ships exactly two process packs: `bug-standard` and `story`"*. E7
rejected *"Install twelve, enable two via `[packs] enabled`"*. The ledger row states the reading on which a landed
edit under `spikes/p9/` ships no pack, as the packet (f) risk list requires: *"Landing a spikes/p9 edit under a story
lane is a reading of ADR-0011/E7, not something they expressly permit, so the D24 row must state it."* **This stage
does not touch `spike.toml`.** Checkpoint 8j asserts its bytes are unchanged since `ca8f387`, because a stage that
pre-did the story would leave the lane nothing to author.

**(b) The Codex item is retired — `.redkiln/items/rk-story-codex-adapter.toml` is deleted, by a dated ledger
entry.** The entry quotes the warrants. ADR-0007: *"the Codex adapter ships in Phase 4 ("reach"), and only after
S-CODEX passes"*. E-0094-1: *"`rk-adapter-codex` is out of Phase 1 under every reading"*. It also cites the record
`s8a` wrote for D21-42 22, which supersedes ADR-0094 decision 2's phase clause toward ADR-0007. The file is deleted
and not marked retired, for two reasons: `ItemRecord` has no retired field, and a story item left in
`.redkiln/items/` is one `redkiln run` away from a lane on a Phase-4 story. The file is named by
`runbook/phase-1/s7i-cost.md` (seven occurrences), by the ledger and by the packet, and by no `.rs` or `.toml`
([measured 2026-09-24]). That is why (c) lands **in the same commit**.

**(c) Checkpoint 7i's empty case moves to `rk-cost-empty-case`.** D21-41 (f): *"Checkpoint 7i's empty case moves to
a dedicated item that nobody dispatches, and its PRECONDITION guard is kept."* A new
`.redkiln/items/rk-cost-empty-case.toml` has `id = "rk-cost-empty-case"`, `kind = "docs"` and `stage = "triage"`.
Its header says only this:

> *dedicated to Checkpoint 7i's empty case; nobody dispatches it; the PRECONDITION guard in Checkpoint 7i is what
> protects that case*

🛑 **It does not say the item "can never run".** The packet (f) measured `derive_lane`'s
`(Some(given), None) => Ok(given)`, which runs a `kind = docs` item whenever `--lane` is passed. The guard is the
protection, and the header must not claim more.

In `runbook/phase-1/s7i-cost.md`, Checkpoint 7i's block has its seven `rk-story-codex-adapter` occurrences replaced
by `rk-cost-empty-case`, under one dated comment above the `for it in` line. The PRECONDITION line keeps its shape
and names the new item: `FAIL (PRECONDITION): rk-cost-empty-case owns $owned readings — …`. The preamble's rule
applies: *"A checkpoint that changed after it passed is provisional"*. So `PROGRESS.md` row 7i is marked
`passed 6907312, re-run pending (s8j re-point)`, and Checkpoint 8j re-runs Checkpoint 7i whole.

**(d) `rk-fix-doctor-test-count` stays the untouched fixture item, with a dated note.** D21-42 18: *"stays the
untouched fixture item, prose defect included, with a dated note"*. Its header opens *"Step 8 criterion (1),
AC-473: the fix-lane self-hosting item"*, and that is no longer true. A dated comment under it says three things:
the item is now the fixture item that Checkpoints 7h and 7i read; criterion (1)'s item is the one §8j.5 reserves;
and neither the item nor `rk-run/rk-fix-doctor-test-count` (at `73c087a`) is to be run, rebased or pruned.
**Only `#` lines change.** Checkpoint 8j asserts that the file's non-comment lines equal those at `ca8f387`, and that
the ref has not moved.

### 8j.2 The human roots: one pack-declared question per human kind, for both lanes

D21-41 (f): *"IntakeRecord and BugReport stay `source = "human"` (D12 #15), supplied through a pack-declared gate
question whose option attaches the kind (ADR-0129 d3/d4); this is decided once for both lanes."* It is on this
stage's path because each lane's first activity needs its root:

- The fix lane's analysis activities `minimal-repro` and `exploitability-analysis` require BugReport.
- Three of the five Spec providers require IntakeRecord, among them the story lane's `user-story-ac` (packet (f),
  measured state (5) and (7)).

**What exists, measured 2026-09-24.**

- `rk_kernel::workflow::question::QuestionRegistry` has the two seams, `with_question` and `bind(gate_kind,
  question_id)`.
- Its module says *"Pack-declared questions are Step 7e's. At 7d the only question that exists in a file is the
  default one … 7e's pack loader will call the same two seams."*
- **No pack loader calls them.** `answer.rs` validates against `QuestionRegistry::default()`, and
  `grep -rn attaches .redkiln/` returns 0 (packet (f) measured state (6)).
- So `answer --attach IntakeRecord=…` is refused by design. ADR-0129 decision 4: *"An attachment whose `<kind>` no
  declared option of the gate's question admits is **refused at the CLI boundary and nothing is written**"*.

**The work.**

1. The pack loader reads pack-declared questions into the registry through `with_question`. It refuses at load, by
   name, any `attaches` that names a kind absent from `.redkiln/artifacts.toml`.
2. Each shipped pack declares one question whose option attaches the human root its lane's first demand needs:
   - `bug-standard`: BugReport;
   - `story`: IntakeRecord.
3. Selection's step-4 gate is bound through `bind`. That gate is `Unsatisfiable { kind, … }` with
   `NoAdmissibleProducer` on a `source = "human"` kind (AC-054: *"returns `Unsatisfiable { kind, failed_predicates }`
   and opens a gate"*). After the binding, answering it with the attachment writes the root under ADR-0129 decision 4:
   *"accepted and stored as a human-supplied root"*.
4. The re-invocation must then classify that kind as Present. The packet (f) risk list names this as unmeasured:
   *"whether a re-invocation classifies the attached root as Present"*. The fixture proofs in §8j.4 measure it.

**The human-root files the real runs attach are committed now**, so that `s8`'s attach is reproducible:

- `.redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md` states the intent from §8j.1(a) and nothing more;
- `.redkiln/intake/<fix-item>.BugReport.md` is written once §8j.5's pick is made.

`.redkiln/intake/` is a new directory that no reader scans. The record states the path convention, and Checkpoint
8j's whole-suite line proves the new directory disturbs nothing.

**The record** (next free id at writing, `proposed`; ADR-0085's `### Option — <name>` headings per D21-42 24a):

- the pack TOML spelling of a question;
- the gate-kind name the step-4 gate binds under;
- the path convention above;
- **RT-59**, if no upstream stage has settled it. `answer.rs` still says *"RT-59: that `answer` is this row's
  producer is ROUTED, not decided"*, and no sub-stage in the plan names RT-59. If `s8f`'s render-path record already
  decided `answer`'s `artifact.write` role, this record cites it and decides nothing about it.

The record **supersedes nothing**. Carrying out ADR-0129 decision 3's *"Pack-declared questions … are Step 7e's"* at
a later stage is execution order, not a moved Decision sentence.

**Tests, red first.** All are in `rk-kernel` `--test it`:

- `question::pack_declared_question_loads_with_its_attaches` — red today, because nothing reads a question from a pack;
- `question::pack_question_attaching_an_unregistered_kind_refuses_the_load`;
- `pack_validation::shipped_packs_declare_a_question_for_each_human_root_their_lane_demands`.

The behavioural half is §8j.4's `lane_e2e::human_root_is_attached_through_the_pack_question_on_both_lanes`.

**Mutant `M-attach-dropped`.** The loader drops each option's `attaches`. It must turn
`question::pack_declared_question_loads_with_its_attaches` and the `lane_e2e` attach test red.

### 8j.3 The staged first run: the analysis → specification hold (D21-42 19b)

**The mechanism: a human-authored hold on the item, answered like any other gate.**

1. The reserved fix item's record carries `hold_before = "specification"`. The key is spelt that way here; the
   record may re-spell it, and a re-spelling takes a dated correction in this file.
2. When the lane loop's `AdvanceStage { from, to }` would enter the held stage, and no answered gate for that hold
   exists on the item, the kernel **opens a gate instead of advancing**. The gate uses the default question
   (`approve` / `changes-requested`; ADR-0129 decision 3).
3. `run` exits 0. Golden constraint 2, `run` never waits, and ADR-0128's *"a run that STOPS AT A GATE exits 0"* both
   hold unchanged.
4. The operator reads the analysis stage's outputs through `redkiln explain <item>`.
5. The operator answers with `redkiln answer <item> approve --explain-sufficed <yes|no> --toml-opened <yes|no>`
   (ADR-0129 decision 1's grammar). The answer is spent (`gate_answer_spent`, ADR-0123).
6. The next `redkiln run <item>` enters `specification`. That run is a new process, so the first real invocation
   also crosses a process boundary on purpose, under the record `s8e` wrote for D21-41 (a).
7. `changes-requested` does not advance the lane. The rule is D21-40 decision 2's: stop and report.

**Why this and not another route.**

- **A `run` flag.** ADR-0128 *"Forbids a second `run` grammar"*, and an environment variable read by `run` is the
  same flag under another name.
- **A per-item dispatch cap** (D21-42 19a). It stops wherever the count runs out, mid-stage and not at the boundary.
- **A pack-level gate at `specification`.** It would hold every fix-lane item for ever.

The hold is item-level, and it is human-authored in the checkout before the run. It is not something the run writes.
So ADR-0157's forbids bullet, *"Writing an item's advanced stage back to `.redkiln/items/` (AC-518)"*, is untouched:
nothing writes the item record.

**The record** (next free id at writing, `proposed`) covers four things:

- the item-record key;
- the gate cause;
- the rule that an answered hold never re-fires;
- the refusal of a hold that names a stage the item's pack does not order, at load and by name.

If `gate_opened`'s reason set is closed in `rk-types`, the new reason lands in the same change as its schema
snapshot. That is ADR-0116 decision 5's rule applied by analogy: *"The table row and the `rk-types` variant land in
the same change"*. The record supersedes nothing that this file could find. It states that finding and invites
refutation.

**Tests, red first.** All are in `rk-cli` `--test it`, over the §8j.4 fixture:

- `lane_e2e::boundary_hold_stops_the_lane_at_analysis_to_specification`;
- `lane_e2e::boundary_hold_answered_lets_the_next_process_enter_specification`;
- `lane_e2e::boundary_hold_changes_requested_does_not_advance`.

**Mutants.**

- `M-hold-ignored`: the hold is not consulted. It must turn the first test red.
- `M-hold-spent-refires`: an answered hold opens again. It must turn the second test red.

### 8j.4 Both lanes proven end to end under the FakeHarness — the fixture proofs D21-40 decision 2 gates on

**What is proved, and through what.**

- Every proof enters through `rk_cli::verbs::run::run`. The only exception is the binary arm, which enters through
  the built `redkiln` binary. No test calls `replay::resume` or any kernel seam directly.
- The harness is the FakeHarness, injected through the seam `s8e` added for its first end-to-end test (D21-42 24c),
  and extended here where the lanes need more.
- The toolchain is the **real** `rk-adapter-toolchain-rust`. `detect()` chooses it over the run-owned worktree, as
  ADR-0137 decision 2 says. It is never `FixtureToolchain`, and never the `rk-fixture-toolchain` child binary.
  `[adapters] toolchain_programs` is left unset for Rust, so `program` is `cargo` (ADR-0137 decision 7).
- A proof that ran over a fixture toolchain would prove nothing about AC-473's *"the toolchain stage ran through
  `rk-adapter-toolchain-rust` behind the `Toolchain` port"*.

**The fixture repository.** It is materialised per test into a temporary directory as a fresh git repository:

- a one-crate Cargo workspace from `crates/rk-cli/tests/fixtures/lane/`, with a planted defect and no failing test
  committed;
- a **byte copy of the shipped `.redkiln/` tree taken at test time** — packs, catalog, cards, artifacts, facets,
  roles and config. This is not a fixture pack, and not a committed duplicate that could drift. The proof is over the
  registry `s8` will run, so a registry defect fails here and not at spend time;
- the two lane items, with the real items' facets.

Each test builds its crate in the run-owned worktree's own `target/` (ADR-0137 decision 3), so a proof costs real
`cargo` time. That is the price of using the real adapter.

**The scripted harness.**

- Each scripted response is keyed by `(activity@version, step id)` and carries a closed output that validates
  against the `rk-types` `EXPORTED` schema `s8h` produced.
- **An unscripted dispatch fails the test by name, and there is no default response.** So the set of steps that
  dispatched is asserted, not assumed.
- The implementer's scripted response carries the file writes the double performs **inside the run-owned worktree**:
  a failing test at `red`, and the fix at `green`. The Rust adapter then observes a red suite, and after that a green
  one, through `Toolchain::test`'s outcome report (D21-42 1a).

**The ten `lane_e2e` tests.** All are in `crates/rk-cli/tests/it/lane_e2e.rs` and run under `rk-cli` `--test it`.
Each is written red first, and each was watched fail for the named reason before it was made green.

| test | asserts |
| --- | --- |
| `lane_e2e::fix_lane_reaches_a_landed_content_commit_under_fake_harness` | From `triage`, the fix lane reaches a kernel-made content checkpoint commit on `rk-run/<item>` (D21-41 (b)). The commit's tree differs from the fork point under the crate's `src/`. AC-518: the fixture's own checkout is unchanged — `git status --porcelain` is empty, `git diff-index --quiet HEAD` passes, and HEAD has not moved. |
| `lane_e2e::fix_lane_trailers_every_non_forbidden_stage_in_order` | The stage tokens of the `Story:` trailers on `<fork>..rk-run/<item>`, read oldest first through git's own parser (`%(trailers:key=Story,valueonly=true)` with `--reverse`, the last whitespace token of each value), equal `bug-standard`'s `stage_order` filtered to `stages[s] != "forbidden"`. The comparison is set **and** order, and it includes a `stage_empty` stage (D21-42 7). |
| `lane_e2e::fix_lane_verify_completed_names_the_rust_adapter` | At least one `verify_completed` row names the item, and every such row names `rk-adapter-toolchain-rust` (D21-42 10b). |
| `lane_e2e::story_lane_spec_is_landed_and_journalled` | The story lane's Spec is written by `rk-store` under the item's managed artifact directory (D21-42 5b). `git cat-file -e` finds it on `rk-run/<item>`. The item's **last** `artifact.write` row with `kind = "Spec"` carries a `digest` equal to the sha256 of the landed blob. This is ADR-0116 decision 3, *"not tree membership"*. |
| `lane_e2e::story_lane_trailers_every_non_forbidden_stage_in_order` | As the fix-lane twin, over `story`'s order. The whole lane runs, not only up to the Spec, because (1b) asserts *"every story-lane stage"* and a proof that stopped at `specification` would leave `implement`, `verify`, `review` and `release` unproven for the story pack. |
| `lane_e2e::human_root_is_attached_through_the_pack_question_on_both_lanes` | The first invocation stops at the step-4 gate on BugReport or IntakeRecord and exits 0. `answer --attach` through the pack question succeeds. The re-invocation classifies the root as Present. |
| `lane_e2e::gate_then_answer_then_second_process_continues_binary_arm` | **The multi-process arm.** It uses `env!("CARGO_BIN_EXE_redkiln")`, with `[adapters] harness_binary` set to the `rk-fixture-child-cli` child, extended to replay a per-dispatch script. The sequence is: `run`, which stops at the human-root gate; `answer`; `run`, which stops at the §8j.3 hold; `answer approve`; `run`, which reaches the landing. **Five separate processes.** The lane continues across each boundary from the stage-bearing trailer (D21-41 (a)), and the final trailer sequence equals the in-process arm's. |
| `lane_e2e::boundary_hold_stops_the_lane_at_analysis_to_specification` | see §8j.3 |
| `lane_e2e::boundary_hold_answered_lets_the_next_process_enter_specification` | see §8j.3 |
| `lane_e2e::boundary_hold_changes_requested_does_not_advance` | see §8j.3 |

🛑 **A one-hop fixture cannot judge a multi-hop guard.** The lanes cross nine and ten boundaries, and the binary arm
crosses four process boundaries. A green that held on the first hop would prove nothing about the fifth. That is why
the stage lists are compared **whole**, and why the multi-process arm ends at the landing and not at the first
successful re-entry.

**Mutants** (run from Python, never the shell; each must flip the named test red, and a survivor blocks):

| mutant | the change | must redden |
| --- | --- | --- |
| `M-seed-record` | `through_step_seven` seeds the stage from the item record, ignoring the reconciled trailer | `…_second_process_continues_binary_arm`, `…_answered_lets_the_next_process_enter_specification` |
| `M-land-empty` | the landing commits `allow_empty` with nothing staged | `fix_lane_reaches_a_landed_content_commit_under_fake_harness` |
| `M-fixture-toolchain` | the composition root builds `FixtureToolchain` | `fix_lane_verify_completed_names_the_rust_adapter` |
| `M-skip-empty-stage-trailer` | a `stage_empty` stage mints no trailer | both `…_trailers_every_non_forbidden_stage_in_order` |
| `M-spec-digest-pre-render` | `artifact.write`'s digest is taken over the pre-render value, not the written bytes | `story_lane_spec_is_landed_and_journalled` |
| `M-attach-dropped`, `M-hold-ignored`, `M-hold-spent-refires` | see §8j.2 and §8j.3 | as named there |

⚠️ **A fixture per function is not a fixture per branch.** Where a proof turns on a branch — `stage_empty` against
a stage that selected activities, or `approve` against `changes-requested` — each branch has its own assertion. The
mutant table exists to show that each branch is observable.

🛑 **What this section does not claim.**

- It does not claim AC-473 or AC-521. Those are claimed by real runs on this repository, which are `s8`'s.
- It does not claim that the review gate is guarded against false approval. ADR-0101 clause 5 forbids that until
  S-SUBTLE-CB runs.
- It describes cross-process continuation only to the extent the record `s8e` wrote for D21-41 (a) permits. ADR-0157
  decision 5 stands wherever that record did not supersede it.

### 8j.5 The fix item: a shortlist, the operator's pick, a reservation, and a fresh item (D21-42 18)

D21-42 18: *"a shortlist of **at least three behavioural defects**: reproducible by a failing `cargo test`, one crate,
not in machinery the run exercises, and not scheduled for a sub-stage fix. The operator picks one, it is reserved in
the ledger, and a fresh item is minted."*

**The survey method.**

1. **Exclude what the run exercises.** Drop every function reachable from the call graphs of `verbs::run::run`,
   `verbs::answer`, the lane's `rk-kernel` runtime, replay, fsm and selection, all of `rk-git`, every `rk-adapter-*`,
   `rk-containment`, and the verbs `s8`'s blocks read (`explain`'s stage and output paths, `cost`,
   `doctor --rebuild`).
   - The check is **static**: a caller walk up to those entry points.
   - It is also **dynamic**: with the candidate's repro test added, all ten `lane_e2e` tests stay green. That is
     necessary, not sufficient.
   - After `s8f` lands `rk-store`'s render path, re-walk the callers of every `rk-store` candidate. A function the
     render path now calls is excluded.
2. **Exclude what is scheduled.** Drop any candidate whose file or function is named by `s8a`…`s8i`, or listed among
   their owned decisions in the plan. For example, doctor's journal source and `RK-D-HEADROOM-MARGIN` are D21-42 12c,
   and rk-store's `conn.rs` poison unwrap is L3.
3. **Reproduce in a throwaway tree, and never in the crate.** Extract HEAD into `target/` (`git archive`), append the
   repro test to the named module there, and watch it fail for the named reason. 🛑 **The repro test is not committed
   into any crate.** If it were, it would turn CI red, and it would hand the lane's `red` step its answer. It is
   committed as `.redkiln/intake/<fix-item>.repro.rs`, with `.redkiln/intake/<fix-item>.repro.toml` giving its
   `crate`, `module`, `test` and the `expect` substring of its failure. The BugReport the real run attaches points at
   both.
4. **One crate, low blast radius.** The fix touches one crate's `src/`, in about one function.
5. **Present at least three to the operator.** For each, give the file, the quoted code, the contract it breaks
   (quoted from its own doc, an ADR or the register), the red output from step 3, and why it is outside the run.

**Candidates found by reading the code at `ca8f387`.** These are **candidates only**. Each was checked by tracing the
code, and none has yet been run red. Step 3 runs each one before it is presented.

| # | crate · function | the defect (quoted code) | the contract it breaks | repro test (proposed) | caveat |
| --- | --- | --- | --- | --- | --- |
| C1 | `redkiln-migrate` · `legacy::fence::split` | `let tail = src[after + close..].trim_start_matches("\n```");`. `trim_start_matches` strips the pattern **repeatedly**, so a fence that directly follows the closing one loses its opening backticks. | `prose` is *"Everything outside it"* (the `Fenced` doc). Over a yaml fence followed directly by a `diff` fence, `prose` should begin with the second fence's three backticks. It begins with the bare word `diff`. | a new test in `redkiln-migrate` `tests/it`, over `split` | Only `LegacyLedger.prose` carries it, so the impact is low. It is the migrator, which is outside the run. |
| C2 | `rk-store` · `locator::scan` | `let is_fence = trimmed.starts_with("```") \|\| trimmed.starts_with("~~~");` is judged on the **trimmed** line and **before** the HTML-comment branch. So an indented ```` ``` ```` (four or more spaces), or an unmatched ```` ``` ```` inside `<!-- … -->`, opens a fence, and every later heading becomes invisible. | The module's own rule, under AC-150: a heading-looking line *"inside a fenced code block, an indented block, an HTML comment … is content"*. It says so in terms: *"An indented block is code, not a heading."* | `locator::an_indented_fence_marker_is_not_a_fence` and `locator::a_fence_marker_inside_an_html_comment_is_not_a_fence` in `rk-store` `tests/it`. Each resolves `## Target` after the decoy, and each gets `Occurrences { count: 0 }` today. | Today `locator::` has no caller outside its own test file. **Excluded if `s8f`'s render path calls it** (step 1). |
| C3 | `rk-cli` · `verbs::explain::top_level_conjuncts` / `false_conjunct` | `if depth == 0 && src[i..].starts_with(" and ")`. Depth counts only `( [ ) ]`, so the split cuts inside a quoted string, and it cuts `a or b and c` into `a or b` and `c`. | Its own doc: *"A disjunction is **not** decomposed — no single disjunct of a false `or` is "the" reason, so naming one would be inventing a cause."* | a `#[cfg(test)]` unit in `explain.rs` (the function is private). `false_conjunct("kind == 'x and y'", …)` returns `kind == 'x`, not the whole predicate. | `explain` is a read-only verb. `s8` reads its stage output, and this path runs only when explaining a false predicate. |
| C4 | `rk-cli` · `doctor::rules::margin` (`RK-D-MARGIN`) | `let Some(margins) = table(&pack, "margins") else { continue; };` and `margins.get(window).and_then(toml::Value::as_integer)`. A pack with no `[margins]`, with a missing window, or with a non-integer margin passes silently. | AC-377: *"AND `redkiln doctor` reaches the same verdict without dispatching a model call"*. Its Verified-by includes *"a fixture config with no margin that"* must fail. | a doctor fixture under `crates/rk-cli/tests/fixtures/doctor/` with `[margins] five_hour = 10` and no `seven_day`. It should produce an `RK-D-MARGIN` error, and today it produces nothing. | This is not `RK-D-HEADROOM-MARGIN`, which is D21-42 12c's. Re-verify step 2 against `s8f`'s file. |
| C5 | `rk-types` · `schema::lint::walk` | `Value::Object(_) => walk(child, &p, child_inside_union && key != "properties", out)` resets the inside-union flag under every `properties` map. So a union under an object that is itself inside a union is never flagged. | AC-131's instrument is the `NestedUnion` rule, *"A union with a union descendant (AC-131)"*. | `schema_no_oneof::nested_union_through_properties_is_caught` | **Lower confidence.** The `key != "properties"` exclusion is explicit and may be deliberate. Read its history before presenting it. |
| C6 | `rk-store` · `locator::parse_heading` | `text: rest.trim().trim_end_matches('#').trim()` strips a trailing `#` that has no space before it, so `## C#` resolves as `C`. | The locator's ATX reading, in which a closing `#` run needs a preceding space. | `locator::a_trailing_hash_without_a_space_is_heading_text` | This is the same exclusion risk as C2. |

The survey's pass also found three candidates that the method **excludes**:

- `rk-types` `Digest`'s derived `Deserialize` skips `from_hex`'s validation. Its doc says *"The only constructor: 64
  lowercase hex characters"*. But digests are deserialised on the run and replay paths, which fails rule 1.
- `rk-kernel` `toml_subset` admits a table defined twice.
- `catalog::walk` drops a non-string `deviation_ok` entry silently.

The last two sit on the pack and catalog load path the run takes.

**Reservation and mint.**

1. The operator picks one candidate.
2. The pick is reserved by a dated ledger entry. The entry carries one machine-readable line,
   `reserved-fix-item: <id> <crate>:<function>`, and it forbids fixing the defect by hand or in any stage before
   `s8`.
3. A fresh `.redkiln/items/<id>.toml` is minted with an `id` of the form `rk-fix-<slug>` and these fields:
   - `kind = "bug"`, `stage = "triage"`;
   - `hold_before = "specification"` (§8j.3);
   - `[facets]`: `cos = "standard"`, `has_repro = true`, and low `criticality`, `blast_radius` and `size`.
4. The item's BugReport (§8j.2) and its repro pair (step 3) are committed with it.
5. **No `rk-run/<id>` ref exists at this stage's close.** The run ref forks at `s8`'s first `run`, from a HEAD
   descending from the CI-green frozen commit (§8j.7). `s8`'s amended (1) asserts that ancestry, and asserts that the
   code paths are unchanged between the two.

### 8j.6 The CLI pin move and `s8h`'s two live probes — paid, after the fixture proofs, each an operator go-ahead

**Entry condition.** Every `lane_e2e` test and every mutant cycle in §8j.4 is green, and that is recorded. D21-40
decision 2: the spend is authorised *"once the sub-stages' fixture proofs pass, and not before."*

🛑 **Where these run.** They run from a plain terminal, through the committed launcher `s8h` authored (D21-42 17a:
*"A committed launcher script is the only entry to a real run"*), against `~/.claude-personal` (17b). They are not
run from a Claude Code session: such a session exports `CLAUDECODE` and `CLAUDE_CODE_*`, and the launcher refuses
them. That refusal is the design working, not an obstacle to route around (Halt 7). If `s8h`'s launcher offers no
entry for one of the three dispatches below, **STOP and raise it**. Do not run the dispatch bare.

**The order.** The pin must move first. The mismatch policy is `MismatchPolicy::Refuse`, hard-coded in
`real_harness_config`, so any probe dispatched against a CLI other than the pin is refused at the init line. That
happened three times under D17-11's amendment, *"the adapter refused all three dispatches at the init line"*.

1. **Freeze auto-update for the whole real-run window** (D21-42 16), then measure `claude --version`.
2. **If the version equals `TESTED_CLI_VERSION`** (`"2.1.278"` at `ca8f387`), record that no move was needed.
   **Otherwise move the pin in D17-11's form.**
   - Re-tee `spikes/results/instrument-checks/RUNBOOK-instrument.txt`, then `test -s` it. Halt 8 (h) applies:
     `tee` opens its target `O_TRUNC`.
   - Move `pub const TESTED_CLI_VERSION: &str = "…";` in `crates/rk-adapter-claude/src/config.rs`, together with its
     prose and the literal test `config::tested_cli_version_is_the_pin_as_a_config_value`. Run that test green.
   - Record the move in `PROGRESS.md` §1/§3/§6 (golden constraint 8).
3. **Operator go-ahead, then** re-run `live_dispatch_probe` through the launcher. This is one paid haiku dispatch, the
   D17-11 precedent.
4. **Operator go-ahead, then** run `s8h`'s probe 1: reads outside the worktree are denied (D21-42 21d). 🛑 If the CLI
   cannot enforce the deny rules, **the work HALTS** and the gap is reported as a containment finding. D21-42 21d
   takes this in exactly these words.
5. **Operator go-ahead, then** run `s8h`'s probe 2: the tools and permission mode read from the init line (21b).
6. Each pass or fail goes into `runbook/phase-1/s8j-evidence.txt` (format below) as it happens. **A failed or
   unexpectedly refused probe stops the stage.** Nothing is re-spent until the operator has read the report.
7. **The (g) record flips only after both probes pass.** D21-41 (g): *"The record is accepted only after two live
   probes pass, and those probes run after the fixture proofs."* The flip is the operator's dated ledger line. Only
   the frontmatter, the README index cell and the census move.

**The evidence file** is `runbook/phase-1/s8j-evidence.txt`. It is **not under `spikes/results/`**, because
`ci.yml`'s AC-465 job fails on any tracked path there except `RUNBOOK-instrument.txt`. It is space-separated, one
fact per line, and appended in execution order:

```
pin-move <old-version> <new-version|unchanged> <yyyy-mm-dd>
probe live_dispatch_probe <cli-version> pass <yyyy-mm-dd>
probe s8h-probe-1 <cli-version> pass <yyyy-mm-dd>
probe s8h-probe-2 <cli-version> pass <yyyy-mm-dd>
record-g <four-digit id of the record s8h wrote for D21-41 (g)>
ci <github-run-id> <40-hex head sha> <yyyy-mm-dd>
fix-item <reserved id> <40-hex frozen sha>
story-item rk-story-harden-spike-pack <40-hex frozen sha>
```

### 8j.7 CI green on the pushed branch — every job, both OS legs (D21-42 14a)

D21-42 14a: *"`ci.yml` green on the pushed branch, every job, both OS legs, is a Checkpoint 8 condition. Each push
is confirmed with the operator."* The branch has never been pushed (`git branch -r` shows `origin/main` only), and
`ci.yml` has never run. Its own comments say so: *"ARMED, NOT DISCHARGED … this workflow is authored and has never
run."*

**The known missing step.** In the `gates` job, before `test`, add the step below. The warrant is `992d257`'s message,
*"the first fresh run failed 10 rk-adapter-toolchain-node tests on the documented pnpm preflight (node_modules is
git-ignored), and ci.yml has no pnpm step at all"*.

- `pnpm` is installed through `npm` at the version that wrote `pnpm-lock.yaml`: `lockfileVersion: '9.0'`, and pnpm
  10.33.0 as measured on this box by `s7c`.
- No marketplace action is added. `ci.yml`'s own supply-chain stance is *"NO MARKETPLACE ACTION beyond
  `actions/checkout`"*.

```yaml
      # D21-42 14a (s8j §8j.7): the Node toolchain adapter's fixture project needs node_modules before
      # `cargo test --workspace`, or ten rk-adapter-toolchain-node tests fail at tests/it/support.rs's
      # preflight (992d257). pnpm comes from npm at the version that wrote pnpm-lock.yaml -- no
      # marketplace action -- and --frozen-lockfile makes a lockfile drift fail HERE.
      - name: node fixture dependencies (pnpm, frozen lockfile)
        shell: bash
        run: |
          node --version
          npm install --global pnpm@10.33.0
          pnpm --version
          pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project
          test -e crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules/.bin/vitest \
            || { echo "FAIL: no node_modules/.bin/vitest after pnpm install -- the node suite would fail at its preflight"; exit 1; }
```

**Whatever else a first CI run needs is found by running it, and it is not guessed at here.** The loop:

1. 🛑 Get the operator's go-ahead, then `git push -u origin build/phase-0-to-1` (the first time) or `git push`.
2. Read the run with `gh run view <id> --json conclusion,headSha,headBranch,jobs`.
3. For each failing job, diagnose from the log. A code defect is fixed red-first, in its own commit. A test that
   fails on one OS and not the other is a **finding**, never a `#[cfg_attr(…, ignore)]`: AC-121's cross-OS half is
   the reason the matrix has two legs.
4. 🛑 Get the operator's go-ahead, then push again.

The ARMED comments in `ci.yml` get dated, comment-only corrections once their job has run green. The history of the
ARMED reading is kept, not deleted.

**The frozen commit.** The last green run is taken over the commit that closes this stage's work, after §8j.6's pin
move. Its id and head sha go into `s8j-evidence.txt` as `ci …`. The `fix-item` and `story-item` lines carry the same
sha as their frozen sha. The tracker commit that records them changes no code path, and Checkpoint 8j asserts that.

### 8j.8 `s8`'s instrument amendments — the replacement text

This stage owns the plan's list of `s8` amendments, and this section is the deliverable. At this stage's execution
each replacement is applied to `runbook/phase-1/s8-exit-criterion.md` with a dated comment, and Checkpoint 8j proves
it landed. This file does not edit `s8` while it is being authored.

**Bindings.** A few names are minted by earlier sub-stages and are not known at authoring:

- `verify_completed`'s item and adapter members (`s8g`);
- the catalog tests over `.redkiln/` (`s8b`).

Each is written below as a `BIND:` value. Every block that carries one refuses to run while the value is unbound.
When the amendment is applied, each value is replaced with the name the earlier stage's record or file fixed.
Checkpoint 8j (9) asserts that `s8` carries no `BIND:` anywhere, and that every test name `s8` selects exists in
`cargo test --workspace -- --list`.

**A0 — the header row.** Replace `` | `s8` | Phase 1 | Checkpoint 7i | Checkpoint 8 | `s7i` | — | `` with
`` | `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — | ``.

**A1 — criteria (1) and (1b), the whole fenced block.** This one replacement covers six items:

- build first;
- the run's trailers scoped to its own fork, which descends from the frozen sha;
- every lane stage asserted as **set and order** equality (D21-42 7);
- the toolchain identity read from `verify_completed` (D21-42 10b);
- the landed content change;
- (1b)'s digest read from `artifact.write` against the landed blob, with `git cat-file -e` on the run ref (ADR-0116
  decision 3; D21-42 5b). `git ls-files --error-unmatch` is dropped, because it is the tree-membership assertion
  ADR-0116 decision 3 rejects.

⚠️ **The plan says "scoped to commits made by this invocation"**, and that scope is too narrow. §8j.3's hold alone
makes the fix lane at least two invocations, and a set-equality over one invocation's commits would fail a correct
lane. The scope is therefore the run's own fork, `merge-base(HEAD, rk-run/<item>)`. The fork must be ahead of that
point, and must descend from the frozen sha. The fork also rules out the stale-ref shape `s7h` found, *"0 commits
ahead"*.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1; Checkpoint 8j proves it landed). Build first (golden constraint 3).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — criterion (1) NOT RUN (Halt 8)"; exit 1; }
# Names minted by s8g (D21-42 10b), bound when this amendment is applied. An unbound slot refuses.
VC_ITEM_KEY="BIND:s8g-verify_completed-item-member"
VC_ADAPTER_KEY="BIND:s8g-verify_completed-adapter-member"
for v in "$VC_ITEM_KEY" "$VC_ADAPTER_KEY"; do case "$v" in BIND:*) echo "FAIL: unbound slot $v — apply s8j §8j.8's binding before running (1)"; exit 1;; esac; done
grep -qxF 'name = "redkiln-rs"' .redkiln/config.toml || { echo "FAIL: [project] name is not redkiln-rs — the Story: value's <project> would not be what this block compares"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — the frozen sha this run must descend from is unrecorded"; exit 1; }
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
# The expected stage list, derived from the pack and never typed (D21-42 7): stage_order minus forbidden.
order='import sys,tomllib; p=tomllib.load(open(sys.argv[1],"rb"))["policy"]; print(" ".join(s for s in p["stage_order"] if p["stages"][s]!="forbidden"))'
# The lane's stage tokens: git's own trailer parser, oldest first, the LAST token of each Story: value; every value
# must be exactly `<project>/<item> <stage>` (the stage-bearing form of D21-41 (a)).
lane_check() {   # $1 item  $2 pack file — prints nothing on success, the defect otherwise
  exp=$(python -c "$order" "$2"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$exp" ] || { echo "the stage order could not be read from $2 (rc=$rc)"; return 1; }
  git rev-parse --verify --quiet "refs/heads/rk-run/$1" >/dev/null || { echo "no rk-run/$1 ref — no run"; return 1; }
  base=$(git merge-base HEAD "rk-run/$1") || { echo "rk-run/$1 shares no history with HEAD"; return 1; }
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {print $3}' "$ev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $ev"; return 1; }
  git merge-base --is-ancestor "$frozen" "$base" || { echo "rk-run/$1 forked from $base, which does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" "$base" -- crates Cargo.toml Cargo.lock rust-toolchain.toml .redkiln/packs .redkiln/catalog .redkiln/cards || { echo "code changed between the CI-green frozen sha and the run's fork — re-run CI and re-record (s8j §8j.7)"; return 1; }
  ahead=$(git rev-list --count "$base..rk-run/$1") || { echo "rev-list failed"; return 1; }
  [ "$ahead" -ge 1 ] || { echo "rk-run/$1 is $ahead commits ahead of its fork — a stale ref is not a run"; return 1; }
  lg=$(git log --reverse "$base..rk-run/$1" --format='%(trailers:key=Story,valueonly=true)') || { echo "git log over the run failed"; return 1; }
  bad=$(printf '%s\n' "$lg" | awk -v want="redkiln-rs/$1" 'NF && ($1!=want || NF!=2)')
  [ -z "$bad" ] || { echo "Story: values not of the form redkiln-rs/$1 <stage>: $bad"; return 1; }
  got=$(printf '%s\n' "$lg" | awk 'NF {printf "%s%s", sep, $NF; sep=" "}')
  [ "$got" = "$exp" ] || { echo "stages: expected '$exp', the run's trailers say '$got' — if explain shows an open gate, answer it and re-run this block (s8j §8j.3's hold is the first); a gated lane is not failed, and it is not a pass"; return 1; }
}

# (1) — AC-473, the fix lane
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item s8j §8j.5 reserved — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
test -f ".redkiln/items/$FIX_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIX_ITEM.toml does not exist"; exit 1; }
./target/debug/redkiln run "$FIX_ITEM" --lane fix || { echo "FAIL: the fix-lane run exited non-zero"; exit 1; }   # a gate-open exits 0 (C-01 D-01); lane_check distinguishes
out=$(./target/debug/redkiln explain "$FIX_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$FIX_ITEM" .redkiln/packs/bug-standard.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1): $why"; exit 1; }
base=$(git merge-base HEAD "rk-run/$FIX_ITEM") || { echo "FAIL: no merge base"; exit 1; }
git diff --quiet "$base" "rk-run/$FIX_ITEM" -- crates; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL (1): the run landed no content change under crates/ (rc=$rc) — trailers without a landed commit (D21-41 (b))"; exit 1; }
vc='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",("verify_completed",))]; a=sorted({str(x.get(sys.argv[4])) for x in r if x.get(sys.argv[3])==sys.argv[2]}); print(" ".join(a))'
ids=$(python -c "$vc" "$store" "$FIX_ITEM" "$VC_ITEM_KEY" "$VC_ADAPTER_KEY"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the verify_completed probe did not run (rc=$rc)"; exit 1; }
[ "$ids" = "rk-adapter-toolchain-rust" ] || { echo "FAIL (1): the verify stage's toolchain identity is '$ids', not exactly rk-adapter-toolchain-rust (AC-473; D21-42 10b)"; exit 1; }
printf '%s\n' "$out"; git log "$base..rk-run/$FIX_ITEM" --format='%h %s%n%(trailers)'   # → READ: no stage performed by the old redkiln; the landed commit is the fix. The run id is the evidence.

# (1b) — AC-521, the story-lane twin
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM (rk-story-harden-spike-pack)"; exit 1; }
test -f ".redkiln/items/$STORY_ITEM.toml" || { echo "FAIL: .redkiln/items/$STORY_ITEM.toml does not exist"; exit 1; }
./target/debug/redkiln run "$STORY_ITEM" --lane story || { echo "FAIL: the story-lane run exited non-zero"; exit 1; }
out=$(./target/debug/redkiln explain "$STORY_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$STORY_ITEM" .redkiln/packs/story.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1b): $why"; exit 1; }
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the Spec's path under the item's managed artifact directory (D21-42 5b)"; exit 1; }
git cat-file -e "rk-run/$STORY_ITEM:$AUTHORED_STORY" || { echo "FAIL (1b): $AUTHORED_STORY is not on rk-run/$STORY_ITEM"; exit 1; }
git cat-file blob "rk-run/$STORY_ITEM:$AUTHORED_STORY" > "$tdir/spec.blob" || { echo "FAIL: could not read the landed Spec blob"; exit 1; }
landed=$(sha256sum "$tdir/spec.blob") || { echo "FAIL: sha256sum failed"; exit 1; }
landed=${landed%% *}
aw='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? ORDER BY id",("artifact.write",))]; d=[x.get("digest") for x in r if x.get("kind")=="Spec" and x.get("item")==sys.argv[2]]; print(d[-1] if d else "")'
dg=$(python -c "$aw" "$store" "$STORY_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the artifact.write probe did not run (rc=$rc)"; exit 1; }
test -n "$dg" || { echo "FAIL (1b): no artifact.write row with kind Spec for $STORY_ITEM — the authored story is not reachable from the event log (AC-521)"; exit 1; }
[ "$dg" = "$landed" ] || { echo "FAIL (1b): the last Spec artifact.write digest $dg is not the landed blob's $landed (ADR-0116 decisions 3 and 6)"; exit 1; }
printf '%s\n' "$out"   # → READ against AC-521: every story-lane stage (asserted above), the Spec authored by the run. The run id and the story id are the evidence.
```

**A2 — (2)'s operator half: build first, and back up and restore the store around `doctor --rebuild`.** Replace the
three lines from `# The operator path must still work (AC-427), but its STDOUT is not the comparison.` through
`./target/debug/redkiln doctor --rebuild || { … }` with the block below. Either repair would have served: back up
and restore, or run (7) before (2). Back up and restore is taken because it makes the block order-independent. The
restore uses SQLite's own backup API, because a bare `cp` of a WAL-mode database copies an inconsistent pair.

```bash
# The operator path must still work (AC-427), but its STDOUT is not the comparison. AMENDED at s8j (s8j §8j.8 A2):
# built first, and the store `doctor --rebuild` replaces is backed up and restored, so (7) reads the store the run wrote.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (2)'s operator half NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2)'s operator half NOT RUN (Halt 8)"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
STORE_BAK="$store.s8-bak"
bk='import sqlite3,sys; s=sqlite3.connect(sys.argv[1]); d=sqlite3.connect(sys.argv[2]); s.backup(d); d.close(); s.close()'
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate","t1_slice_seal","t2_event")))'
before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$before" ] || { echo "FAIL: could not count the store's rows (rc=$rc)"; exit 1; }
python -c "$bk" "$store" "$STORE_BAK" || { echo "FAIL: the store could not be backed up — doctor --rebuild NOT RUN"; exit 1; }
./target/debug/redkiln doctor --rebuild; drc=$?
python -c "$bk" "$STORE_BAK" "$store" || { echo "FAIL: the store could NOT be restored from $STORE_BAK — restore it by hand before anything reads it"; exit 1; }
after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$after" = "$before" ] || { echo "FAIL: the restored store's row counts ($after) are not the pre-rebuild ones ($before)"; exit 1; }
rm -f "$STORE_BAK"
[ "$drc" -eq 0 ] || { echo "FAIL: doctor --rebuild exited $drc (AC-427)"; exit 1; }
```

**A3 — (3)'s operational arm: build before `test -x`.** Insert these two lines immediately above the line that
opens `test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s operational arm`:

```bash
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A3): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — (3)'s operational arm NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
```

**A4 — (7), the whole fenced block.** D21-42 12a: *"An honest rendering passes, with two assertions: owned readings
== `dispatch_completed` for the item, and no rendering over n = 0. This replaces s8's unreadable AC-394 READ line."*
The `item` member on `dispatch_completed` is D21-42 12b's interim key, which is *"extended to `dispatch_completed`"*.

```bash
# AMENDED at s8j (s8j §8j.8 A4): built first; AC-394 asserted as owned readings == dispatch_completed (D21-42 12a).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (7) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (7) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item whose cost rows are being read"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
own='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); n=lambda k: sum(1 for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",(k,)) if json.loads(p).get("item")==sys.argv[2]); print(n("rate_limit_reading"), n("dispatch_completed"))'
cnt=$(python -c "$own" "$store" "$ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$cnt" ] || { echo "FAIL: the ownership probe did not run (rc=$rc)"; exit 1; }
read -r owned dc <<< "$cnt"
[ "$dc" -ge 1 ] || { echo "FAIL (7): no dispatch_completed row names $ITEM — a reading over n = 0 is not a reading (D21-42 12a)"; exit 1; }
[ "$owned" = "$dc" ] || { echo "FAIL (7): $ITEM owns $owned readings and $dc dispatch_completed rows — AC-394 is one reading per dispatch (ADR-0163 decision 2)"; exit 1; }
out=$(./target/debug/redkiln cost "$ITEM") || { echo "FAIL: redkiln cost exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: redkiln cost printed nothing"; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: a headroom report carrying none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
if printf '%s\n' "$out" | grep -qE ' nothing to attribute |(^|[^0-9])n=0([^0-9]|$)'; then echo "FAIL (7): a rendering over n = 0 for an item that owns $dc dispatches (D21-42 12a)"; printf '%s\n' "$out"; exit 1; fi
printf '%s\n' "$out"   # → READ: no per-run headroom figure (AC-373).
```

**A5 — Checkpoint 8's mechanical block.** Four edits. Each is anchored on a line that exists today, and each anchor
is asserted to exist before it is replaced (golden constraint 7).

*A5.1 — the catalog checks run over `.redkiln/` (D21-42 13a, 13b).* Replace the three lines that open
`out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?` with the lines below. Today's line walks
`spikes/p9/catalog`, because `rigor.rs` holds `const SHIPPED_CATALOG: &str = "spikes/p9/catalog";`.

```bash
CATALOG_TESTS="BIND:s8b-13a-catalog-tests"   # s8b's module-qualified names: rigor monotonicity, AC-527's d2-form lint and the ported lattice, over .redkiln/ with a non-zero walked count (D21-42 13a, 13b)
case "$CATALOG_TESTS" in *BIND:*) echo "FAIL: unbound slot — apply s8j §8j.8's binding"; exit 1;; esac
n_sel=$(printf '%s\n' $CATALOG_TESTS | grep -c .)
out=$(cargo test -p rk-kernel --test it -- $CATALOG_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the .redkiln/ catalog selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
for t in $CATALOG_TESTS; do printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok"; exit 1; }; done
printf '%s\n' "$out" | grep -qE "test result: ok\. $n_sel passed; 0 failed" || { echo "FAIL: the catalog selection did not run exactly $n_sel tests"; exit 1; }
```

*A5.2 — the quoted-predicate guard, widened (D21-41 (h): "widen Checkpoint 8's guard").* Replace the four lines from
the one that opens `raw=$(grep -rnE 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *='`
through its `syn=$(printf 'a.toml:1:expr = …` control with the block below. Its pattern admits either quote on each
member. Add one synthetic control carrying the **shipped
spelling**. The exclusions stay **by named file**, never by directory: `crates/rk-kernel/tests/it/evaluator.rs`,
which is the predicate's own enforcement, and the prescribed must-refuse fixture
`crates/rk-kernel/tests/fixtures/activities/terminal_condition_is_findings_none.toml`. Any further must-refuse
fixture that `s8b` adds joins this list by name, with its header quoted.

```bash
QPAT="findings\\.none\\(evidence in \\[['\"]?Reproduced['\"]?[[:space:]]*,[[:space:]]*['\"]?MutationSurvived['\"]?\\]\\)|quality_shorthand *="
raw=$(grep -rnE "$QPAT" crates/ .redkiln/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the quality-shorthand search could not run (grep rc=$rc)"; exit 1; }
out=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -v '^crates/rk-kernel/tests/it/evaluator.rs:' | grep -v '^crates/rk-kernel/tests/fixtures/activities/terminal_condition_is_findings_none.toml:' | grep -vE '^[[:space:]]*$'); test -z "$out" || { echo "FAIL: ADR-0080 decision 2's predicate, in some quoting, as a terminal condition:"; printf '%s\n' "$out"; exit 1; }
syn=$(printf '%s\n' ".redkiln/catalog/x.toml:1:expr    = \"findings.none(evidence in ['Reproduced', 'MutationSurvived'])\"" | grep -E "$QPAT"); test -n "$syn" || { echo "FAIL: the widened pattern misses the SHIPPED quoted spelling — this ban is vacuous"; exit 1; }
syn=$(printf 'a.toml:1:expr = "findings.none(evidence in [Reproduced, MutationSurvived])"\n' | grep -E "$QPAT"); test -n "$syn" || { echo "FAIL: the widened pattern lost the unquoted spelling"; exit 1; }
```

*A5.3 — the doc screen does not read `.git/`.* Replace `grep -rn 'four questions on twenty items' --include=*.md . ;
rc=$?` with the line below. D21-40 measured this line reading a stale `runbook/` copy under
`.git/redkiln/worktrees/`.

```bash
grep -rn --exclude-dir=.git 'four questions on twenty items' --include=*.md . ; rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: the phrase is gone (rc=$rc) — design §10.5's row may have been deleted"; exit 1; }   # → each hit READ; .git/ excluded (D21-40: it read .git/redkiln/worktrees/)
```

*A5.4 — two new stanzas, appended after the block's last line* (the `--stay` by-path stanza's guard that opens
`printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails`). The first is
ADR-0164's first rule. The second is D21-42 14a's CI condition.

```bash
# ADR-0164: `* text=auto eol=lf` is .gitattributes' FIRST rule (every -text row below it still wins for its paths).
first=$(grep -vE '^[[:space:]]*(#|$)' .gitattributes | head -1)
[ "$first" = '* text=auto eol=lf' ] || { echo "FAIL: .gitattributes' first rule is '$first', not ADR-0164's row"; exit 1; }
if od -c .gitattributes | grep -qF '\r'; then echo "FAIL: .gitattributes carries a CR byte (ADR-0164 decision 4)"; exit 1; fi
# D21-42 14a: ci.yml green on the pushed branch, every job, both OS legs — read from the recorded run, never re-pushed.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} ' runbook/phase-1/s8j-evidence.txt | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in runbook/phase-1/s8j-evidence.txt"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha $ci_sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests .github .gitattributes .redkiln/packs .redkiln/catalog .redkiln/cards || { echo "FAIL: code changed since the green CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }
```

The bullet list gains two bullets:

- ***`ci.yml` is green on the pushed branch, every job, both OS legs*** (D21-42 14a), *read from the recorded run and
  never re-pushed by this checkpoint.*
- ***`.gitattributes` opens with `* text=auto eol=lf`*** (ADR-0164).

Its count sentence *"Those seventeen are not read, they are run."* becomes *"Those nineteen are not read, they are
run."*.

**A6 — (6): build before the binary runs.** This one is not on the plan's list. It is added because (6)'s two lint
lines invoke `./target/debug/redkiln` with no build and no presence guard, which is the shape golden constraint 3
exists for. Insert these lines above the line that opens `./target/debug/redkiln lint --facet-stability`. The two
lint lines themselves stay bare reads of an exit code, as `s8` explains.

```bash
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A6): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (6) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (6) NOT RUN (Halt 8)"; exit 1; }
```

### 8j.9 Acceptance set

**Claimed.**

- **AC-410**: the positive control through a pack-declared question, over both human roots.
- **AC-054**: the step-4 gate is answerable, and a re-invocation reads the root as Present.
- **AC-518**: the user's checkout is unchanged across a full lane run. This is its Verified-by, *"an integration test
  asserting the user checkout is unchanged across a full lane run"*, in `lane_e2e`.
- **AC-226**: after the pin move, the configured `Refuse` outcome is still the asserted one.
- The first run of every `ci.yml` job that `ci.yml` marks *"ARMED, NOT DISCHARGED"*:
  - **AC-498**, the no-egress job;
  - **AC-465**;
  - **AC-362**;
  - **AC-441**, **AC-442** and **AC-466**;
  - **AC-481**, **AC-483** and **AC-528**;
  - **AC-529**.

**Excluded, each for its own reason.**

- **AC-473** and **AC-521** are claimed by `s8`'s real runs. The fixture proofs are the spend gate D21-40 decision 2
  names, not the claim.
- **AC-160…AC-163** belong to `s8e`. ADR-0158's forbids bullet, *"Claiming AC-160, AC-161, AC-162 or AC-163 is met,
  on any axis"*, stands wherever `s8e`'s record did not lift it.
- **AC-184** is `s8`'s (2).
- **AC-372, AC-373, AC-394 and AC-395** are `s8`'s (7).
- **AC-276** belongs to `s8h`.
- **AC-121's cross-OS half.** The two legs make it checkable, but no instrument named here asserts it, so it is not
  claimed.

### 8j.10 Checkpoint 8j

- **Both lanes are proven end to end under the FakeHarness**, through `verbs::run::run` with the real Rust adapter,
  across real process boundaries: ten named tests, plus three question tests.
- **The items are as §8j.1 and §8j.5 say.**
  - The story item exists at `triage` with low facets.
  - The Codex item is gone.
  - The empty-case item exists, and Checkpoint 7i names it.
  - The fixture item's non-comment lines and ref are untouched.
  - The reserved fix item exists with its hold, its BugReport and its repro pair. It has no run ref.
  - The reserved defect **is still red** at HEAD in a throwaway tree.
- **`spike.toml` is untouched** since `ca8f387`.
- **The pin, the instrument record and both probe results agree**, and the (g) record is `accepted` only with both
  probes recorded as passing.
- **CI is green on the recorded run**: every job, both legs, and the code unchanged since.
- **`s8`'s amendments landed**: no `BIND:`, no `test -x` before a build, and every selected test exists.
- **Checkpoint 7i and Checkpoints 8a–8i re-run whole**, each reaching its end marker.
- **The whole suite, fmt and clippy are green, and the three runbook checkers pass.**
- **This block dispatched nothing and changed no store row.**

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# 🛑 THIS BLOCK DISPATCHES NOTHING AND PUSHES NOTHING. CI status and probe results are READ from the run ids and lines
# recorded in runbook/phase-1/s8j-evidence.txt. Every cargo reading is `out=$(…); rc=$?`; every selection is `--test it`,
# module-qualified and `--exact`, with each test's own `... ok` line and an exact count asserted.

# ---- (0) READ FIRST: the worktree PATH SET and the store's row counts. Neither may move.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate","t1_slice_seal","t2_event")), c.execute("SELECT COUNT(*) FROM t2_event WHERE kind IN (?,?)",("dispatch_completed","rate_limit_reading")).fetchone()[0])'
rows_before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$rows_before" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }
tdir=target/s8j-cp8j; rm -rf "$tdir"; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir (a git-bash /tmp path is invisible to Windows Python, so scratch lives under target/)"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — nothing this stage's gated acts recorded can be read"; exit 1; }
if od -c "$ev" | grep -qF '\r'; then echo "FAIL: $ev carries a CR byte — a field read from it would carry one too"; exit 1; fi

# ---- (1) BUILD-OR-FAIL.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8j NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE FIXTURE PROOFS (§8j.4) and THE QUESTION TESTS (§8j.2).
test -d crates/rk-cli/tests/fixtures/lane || { echo "FAIL: the lane fixture is absent — the proofs would materialise nothing"; exit 1; }
test -f crates/rk-cli/tests/it/lane_e2e.rs || { echo "FAIL: crates/rk-cli/tests/it/lane_e2e.rs is absent"; exit 1; }
L_TESTS="lane_e2e::fix_lane_reaches_a_landed_content_commit_under_fake_harness lane_e2e::fix_lane_trailers_every_non_forbidden_stage_in_order lane_e2e::fix_lane_verify_completed_names_the_rust_adapter lane_e2e::story_lane_spec_is_landed_and_journalled lane_e2e::story_lane_trailers_every_non_forbidden_stage_in_order lane_e2e::human_root_is_attached_through_the_pack_question_on_both_lanes lane_e2e::gate_then_answer_then_second_process_continues_binary_arm lane_e2e::boundary_hold_stops_the_lane_at_analysis_to_specification lane_e2e::boundary_hold_answered_lets_the_next_process_enter_specification lane_e2e::boundary_hold_changes_requested_does_not_advance"
out=$(cargo test -p rk-cli --test it -- $L_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the lane proofs did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
for t in $L_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 10 passed; 0 failed' || { echo "FAIL: the lane selection did not run exactly 10 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
K_TESTS="question::pack_declared_question_loads_with_its_attaches question::pack_question_attaching_an_unregistered_kind_refuses_the_load pack_validation::shipped_packs_declare_a_question_for_each_human_root_their_lane_demands"
out=$(cargo test -p rk-kernel --test it -- $K_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the question selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $K_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok"; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 3 passed; 0 failed' || { echo "FAIL: the question selection did not run exactly 3 tests"; exit 1; }
# The proofs use the REAL Rust adapter: no lane test may name the fixture toolchain child or FixtureToolchain on a CODE
# line — not even to assert its absence, which is asserted behaviourally by the verify_completed identity test. A FLOOR.
raw=$(grep -nE 'rk-fixture-toolchain|FixtureToolchain' crates/rk-cli/tests/it/lane_e2e.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the fixture-toolchain floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: a lane proof names a fixture toolchain on a code line — AC-473's adapter identity would be proved over a double:"; printf '%s\n' "$code"; exit 1; }
syn=$(printf '12:    let t = FixtureToolchain::default();\n' | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -E 'rk-fixture-toolchain|FixtureToolchain'); test -n "$syn" || { echo "FAIL: the comment filter swallowed a code line — the floor is vacuous"; exit 1; }

# ---- (3) THE ITEMS (§8j.1, §8j.5).
itm='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); f=d.get("facets",{}); print(d.get("id"), d.get("kind"), d.get("stage"), d.get("hold_before","-"), f.get("cos","-"), f.get("criticality","-"), f.get("blast_radius","-"), f.get("size","-"), str(f.get("external_dep","-")).lower(), str(f.get("has_repro","-")).lower())'
test ! -e .redkiln/items/rk-story-codex-adapter.toml || { echo "FAIL: the Codex item is not retired (ADR-0007; D21-41 (f))"; exit 1; }
s=$(python -c "$itm" .redkiln/items/rk-story-harden-spike-pack.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the story item did not parse (rc=$rc)"; exit 1; }
[ "$s" = "rk-story-harden-spike-pack story triage - standard low low low false -" ] || { echo "FAIL: the story item is '$s', not a story at triage with low facets and external_dep = false"; exit 1; }
e=$(python -c "$itm" .redkiln/items/rk-cost-empty-case.toml); rc=$?
[ "$rc" -eq 0 ] && [ "${e%% *}" = "rk-cost-empty-case" ] || { echo "FAIL: the empty-case item is absent or mis-named ('$e', rc=$rc)"; exit 1; }
fx_line=$(grep -E '^fix-item rk-fix-[a-z0-9-]+ [0-9a-f]{40}$' "$ev" | tail -1)
test -n "$fx_line" || { echo "FAIL: no fix-item line in $ev — the reservation is unrecorded"; exit 1; }
read -r _ fx fx_frozen <<< "$fx_line"
grep -qE "^reserved-fix-item: $fx [a-z0-9_-]+:[A-Za-z0-9_:]+\$" spikes/DECISIONS-2026-09-21.md || { echo "FAIL: $fx is not reserved in the ledger (D21-42 18)"; exit 1; }
f=$(python -c "$itm" ".redkiln/items/$fx.toml"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: .redkiln/items/$fx.toml did not parse (rc=$rc)"; exit 1; }
[ "$f" = "$fx bug triage specification standard low low low - true" ] || { echo "FAIL: the reserved item is '$f', not a bug at triage holding before specification with low facets and has_repro"; exit 1; }
if git rev-parse --verify --quiet "refs/heads/rk-run/$fx" >/dev/null; then echo "FAIL: rk-run/$fx already exists — the fresh item's run must fork from the frozen HEAD at s8's first run"; exit 1; fi
test -s ".redkiln/intake/$fx.BugReport.md" || { echo "FAIL: no BugReport for $fx"; exit 1; }
test -s .redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md || { echo "FAIL: no IntakeRecord for the story item"; exit 1; }
old=$(git show ca8f387:.redkiln/items/rk-fix-doctor-test-count.toml | grep -vE '^[[:space:]]*(#|$)')
new=$(grep -vE '^[[:space:]]*(#|$)' .redkiln/items/rk-fix-doctor-test-count.toml)
[ -n "$old" ] && [ "$old" = "$new" ] || { echo "FAIL: rk-fix-doctor-test-count's non-comment lines changed — it is the UNTOUCHED fixture item (D21-42 18)"; exit 1; }
r_now=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); r_pin=$(git rev-parse --verify --quiet "73c087a^{commit}")
[ -n "$r_now" ] && [ -n "$r_pin" ] && [ "$r_now" = "$r_pin" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is '$r_now', not 73c087a ('$r_pin') — Checkpoints 7h and 7i read it; two empty reads never compare equal here"; exit 1; }
# The story is the lane's to author: this stage did not touch spike.toml.
test -f spikes/p9/packs/spike.toml || { echo "FAIL: spikes/p9/packs/spike.toml is gone"; exit 1; }
git diff --quiet ca8f387 HEAD -- spikes/p9/packs/spike.toml || { echo "FAIL: spike.toml changed since ca8f387 — the story lane would have nothing to author"; exit 1; }

# ---- (4) THE RESERVED DEFECT IS STILL RED AT HEAD — in a throwaway tree, never in the crate.
rmeta=".redkiln/intake/$fx.repro.toml"; rsrc=".redkiln/intake/$fx.repro.rs"
test -s "$rmeta" && test -s "$rsrc" || { echo "FAIL: $fx's repro pair is absent"; exit 1; }
rp='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); print(d["crate"]); print(d["module"]); print(d["test"]); print(d["expect"])'
meta=$(python -c "$rp" "$rmeta"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: $rmeta did not parse (rc=$rc)"; exit 1; }
{ read -r rcrate; read -r rmod; read -r rtest; read -r rexpect; } <<< "$meta"
[ -n "$rcrate" ] && [ -n "$rmod" ] && [ -n "$rtest" ] && [ -n "$rexpect" ] || { echo "FAIL: $rmeta lacks crate/module/test/expect"; exit 1; }
test -f "crates/$rcrate/tests/it/$rmod.rs" || { echo "FAIL: crates/$rcrate/tests/it/$rmod.rs does not exist"; exit 1; }
n=$(grep -cE "fn $rtest\(" "crates/$rcrate/tests/it/$rmod.rs")
[ "$n" = "0" ] || { echo "FAIL: the repro test is committed INTO the crate — it would turn CI red and hand the lane's red step its answer"; exit 1; }
git archive --format=tar -o "$tdir/head.tar" HEAD || { echo "FAIL: git archive HEAD failed"; exit 1; }
mkdir -p "$tdir/repro" && tar -xf "$tdir/head.tar" -C "$tdir/repro" || { echo "FAIL: could not extract HEAD"; exit 1; }
cat "$rsrc" >> "$tdir/repro/crates/$rcrate/tests/it/$rmod.rs" || { echo "FAIL: could not append the repro"; exit 1; }
out=$(cd "$tdir/repro" && CARGO_TARGET_DIR=../../s8j-repro-target cargo test -p "$rcrate" --test it -- "$rmod::$rtest" --exact 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the reserved defect's repro PASSES at HEAD — the defect was fixed before the run (D21-42 18)"; exit 1; }
printf '%s\n' "$out" | grep -qxF "test $rmod::$rtest ... FAILED" || { echo "FAIL: the repro did not run and fail by name — red for another reason is not the defect"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qF "$rexpect" || { echo "FAIL: the repro failed without its expected message '$rexpect' — a red for the wrong reason"; exit 1; }

# ---- (5) CHECKPOINT 7i'S EMPTY CASE POINTS AT THE DEDICATED ITEM, AND KEEPS ITS PRECONDITION GUARD.
x7i='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
b=[x for x in re.findall(f+"bash\n(.*?)"+f,t,re.S) if "CP7I-END-OF-BLOCK reached" in x]
code="\n".join(l for l in (b[-1].split("\n") if b else []) if not l.strip().startswith("#"))
print(code.count("rk-story-codex-adapter"), code.count("rk-cost-empty-case"), code.count("FAIL (PRECONDITION): rk-cost-empty-case owns"))'
c7=$(python -c "$x7i" runbook/phase-1/s7i-cost.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7i could not be read (rc=$rc)"; exit 1; }
read -r c_old c_new c_pre <<< "$c7"
[ "$c_old" = "0" ] && [ "$c_new" -ge 6 ] && [ "$c_pre" = "1" ] || { echo "FAIL: Checkpoint 7i's code lines name the Codex item $c_old times, the empty-case item $c_new times, and its PRECONDITION guard $c_pre times"; exit 1; }

# ---- (6) THE PIN, THE INSTRUMENT RECORD AND THE PROBES AGREE; (g) IS ACCEPTED ONLY WITH BOTH PROBES PASSED.
inst=$(tr -d '\r\n' < spikes/results/instrument-checks/RUNBOOK-instrument.txt)
ver=${inst%% *}
pin=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9]+\.[0-9]+\.[0-9]+)";$/\1/p' crates/rk-adapter-claude/src/config.rs)
[ -n "$ver" ] && [ "$ver" = "$pin" ] || { echo "FAIL: the instrument record says '$ver' and TESTED_CLI_VERSION says '$pin' (Halt 2; D21-42 16)"; exit 1; }
grep -qxF "    assert_eq!(TESTED_CLI_VERSION, \"$pin\");" crates/rk-adapter-claude/tests/it/config.rs || { echo "FAIL: the literal pin test does not carry $pin"; exit 1; }
out=$(cargo test -p rk-adapter-claude --test it -- config::tested_cli_version_is_the_pin_as_a_config_value --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the pin test did not pass exactly once (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'test config::tested_cli_version_is_the_pin_as_a_config_value ... ok' || { echo "FAIL: the pin test did not report ok by name"; exit 1; }
pm=$(grep -E '^pin-move [0-9.]+ ([0-9.]+|unchanged) [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$pm" || { echo "FAIL: no pin-move line in $ev"; exit 1; }
read -r _ pm_old pm_new _ <<< "$pm"
[ "$pm_new" = "$pin" ] || { [ "$pm_new" = "unchanged" ] && [ "$pm_old" = "$pin" ]; } || { echo "FAIL: the recorded pin move ($pm_old -> $pm_new) does not end at the pin $pin"; exit 1; }
for p in live_dispatch_probe s8h-probe-1 s8h-probe-2; do
  grep -qxE "probe $p $pin pass [0-9]{4}-[0-9]{2}-[0-9]{2}" "$ev" || { echo "FAIL: no passing '$p' recorded at the pin $pin — the (g) record may not flip (D21-41 (g))"; exit 1; }
done
rg_id=$(sed -nE 's/^record-g ([0-9]{4})$/\1/p' "$ev" | tail -1)
test -n "$rg_id" || { echo "FAIL: no record-g line names the (g) record"; exit 1; }
rg_file=$(ls references/adr/"$rg_id"-*.md 2>/dev/null)
test -n "$rg_file" && test -f "$rg_file" || { echo "FAIL: ADR-$rg_id's file does not exist"; exit 1; }
grep -qxF 'status: accepted' "$rg_file" || { echo "FAIL: ADR-$rg_id is not accepted — both probes are recorded as passing, so the operator's dated flip is what is missing"; exit 1; }

# ---- (7) CI: GREEN ON THE RECORDED RUN, EVERY JOB, BOTH LEGS; THE CODE UNCHANGED SINCE; THE pnpm STEP PRESENT.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in $ev"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
[ "$fx_frozen" = "$ci_sha" ] || { echo "FAIL: the fix item's frozen sha $fx_frozen is not the CI-green sha $ci_sha"; exit 1; }
grep -qxF "story-item rk-story-harden-spike-pack $ci_sha" "$ev" || { echo "FAIL: the story item's frozen sha is not the CI-green sha"; exit 1; }
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests .github .gitattributes .redkiln || { echo "FAIL: code or registry changed since CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
cy=$(grep -nE '^[[:space:]]+pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project$' .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ci.yml has no pnpm install step on a code line (D21-42 14a)"; exit 1; }
ct=$(grep -nE '^[[:space:]]+run: cargo test --workspace$' .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ci.yml's test gate line was not found — the ordering check would compare nothing"; exit 1; }
[ "${cy%%:*}" -lt "${ct%%:*}" ] || { echo "FAIL: the pnpm step comes after cargo test — the node suite would still fail at its preflight"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
# SYNTHETIC CONTROL first: a run missing the windows leg must be refused, or the checker is vacuous.
v=$(python -c "$cichk" "$ci_sha" <<< "{\"conclusion\":\"success\",\"headSha\":\"$ci_sha\",\"headBranch\":\"build/phase-0-to-1\",\"jobs\":[{\"name\":\"gates (ubuntu-latest)\",\"conclusion\":\"success\"},{\"name\":\"no egress (AC-498)\",\"conclusion\":\"success\"},{\"name\":\"structural assertions\",\"conclusion\":\"success\"}]}"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the CI checker accepted a run with no windows leg (rc=$rc: $v) — it is vacuous"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }

# ---- (8) s8's AMENDMENTS LANDED (§8j.8): header, no BIND:, no test -x before a build, the named instruments present.
s8chk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]
if "| `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — |" not in t: bad.append("header row")
if "BIND:" in t: bad.append("an unbound BIND: slot")
blocks=re.findall(f+"bash\n(.*?)"+f,t,re.S)
code="\n".join(l for b in blocks for l in b.split("\n") if not l.strip().startswith("#"))
if "git ls-files --error-unmatch" in code: bad.append("(1b) still asserts tree membership")
for need in ("--exclude-dir=.git","%(trailers:key=Story,valueonly=true)","stage_order","verify_completed","artifact.write","git cat-file -e","dispatch_completed","gh run view","* text=auto eol=lf","STORE_BAK","]?Reproduced","]?MutationSurvived"):
    if need not in code: bad.append("absent from code lines: "+need)
if "Those nineteen are not read, they are run." not in t: bad.append("the bullet count sentence")
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$s8chk" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: s8's amendments did not land: $v"; exit 1; }
# Golden constraint 3 in every s8 block: no `test -x ./target/debug/redkiln` before a guarded `cargo build --workspace`.
# Its OWN checker, so its synthetic control can fail for this reason and no other.
txchk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]
blocks=re.findall(f+"bash\n(.*?)"+f,t,re.S)
for i,b in enumerate(blocks):
    built=False
    for ln in b.split("\n"):
        s=ln.strip()
        if s.startswith("#"): continue
        if "cargo build --workspace" in s and "rc=$?" in s: built=True
        if "test -x ./target/debug/redkiln" in s and not built: bad.append("block %d: test -x before a build" % i)
print("; ".join(bad) if bad else "ok %d blocks" % len(blocks))
sys.exit(1 if bad or not blocks else 0)'
v=$(python -c "$txchk" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an s8 block runs the binary before building it: $v"; exit 1; }
# SYNTHETIC CONTROLS: one block that runs the binary unbuilt must be refused; the same block built first must pass.
bt=$(printf '%s' '`' '`' '`')
printf '%s\n' "$bt""bash" 'test -x ./target/debug/redkiln || exit 1' "$bt" > "$tdir/s8-syn-red.md" || { echo "FAIL: could not write the red control"; exit 1; }
printf '%s\n' "$bt""bash" 'out=$(cargo build --workspace 2>&1); rc=$?' 'test -x ./target/debug/redkiln || exit 1' "$bt" > "$tdir/s8-syn-green.md" || { echo "FAIL: could not write the green control"; exit 1; }
v=$(python -c "$txchk" "$tdir/s8-syn-red.md"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the build-first checker accepted a test -x with no build before it (rc=$rc: $v) — it is vacuous"; exit 1; }
v=$(python -c "$txchk" "$tdir/s8-syn-green.md"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the build-first checker refused a correctly ordered block (rc=$rc: $v) — it cannot pass"; exit 1; }
# Every test name s8 selects through a *_TESTS variable exists.
nm='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
for v in re.findall(r"^[A-Z_]*TESTS=\"([^\"]*)\"",t,re.M):
    for n in v.split(): print(n)'
names=$(python -c "$nm" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$names" ] || { echo "FAIL: s8 selects no test through a *_TESTS variable (rc=$rc) — A5.1 did not land"; exit 1; }
lst=$(cargo test --workspace -- --list 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --list rc=$rc"; exit 1; }
for n in $names; do printf '%s\n' "$lst" | grep -qxF "$n: test" || { echo "FAIL: s8 selects '$n', which no test binary lists"; exit 1; }; done

# ---- (9) THE THREE RUNBOOK CHECKERS, from the repo root: pass, and zero flagged.
out=$(python runbook/tools/check-checkpoints.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE 'flagged 0;' || { echo "FAIL: check-checkpoints rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }
out=$(python runbook/tools/check-refs.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE '^unresolved bare section refs: 0 ' || { echo "FAIL: check-refs rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }
out=$(python runbook/tools/check-bash-syntax.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE ' 0 failed$' || { echo "FAIL: check-bash-syntax rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }

# ---- (10) RE-RUN, WHOLE: Checkpoint 7i (edited by §8j.1(c), so provisional) and Checkpoints 8a…8i, each to its marker.
# ⚠️ COST, stated rather than hidden: each sub-stage block re-runs its own entry checkpoint, so the chain nests and 8i
# alone re-runs 8h…8a. Running each one separately repeats that work many times over. That is the price of reading each
# block's end marker ourselves instead of trusting 8i's nested read of it. Budget hours, not minutes.
xblk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
hits=[b for b in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in b]
if not hits: sys.exit(3)
open(sys.argv[3],"w",encoding="utf-8",newline="\n").write(hits[-1])'
for pair in s7i-cost:CP7I s8a-records-and-red-checkpoints:CP8A s8b-catalog-quality:CP8B s8c-toolchain-port-and-verbs:CP8C s8d-step-grain-and-resolver:CP8D s8e-tier-one-and-the-lane:CP8E s8f-landing-and-rendering:CP8F s8g-verify-and-restored-demands:CP8G s8h-cards-and-run-environment:CP8H s8i-s-bin:CP8I; do
  sf="runbook/phase-1/${pair%%:*}.md"; mk="${pair##*:}-END-OF-BLOCK reached"; blk="$tdir/${pair%%:*}.sh"
  test -f "$sf" || { echo "FAIL: $sf does not exist — its checkpoint cannot be re-run (Halt 8)"; exit 1; }
  python -c "$xblk" "$sf" "echo \"$mk\"" "$blk"; rc=$?
  [ "$rc" -eq 0 ] && test -s "$blk" || { echo "FAIL: no block in $sf echoes '$mk' (rc=$rc)"; exit 1; }
  hit=$(grep -nE -- '--features live|-- --ignored|git push' "$blk" | grep -vE '^[0-9]+:[[:space:]]*#')
  [ -z "$hit" ] || { echo "FAIL: $sf's checkpoint would dispatch or push on a code line — this block runs nothing that does:"; printf '%s\n' "$hit"; exit 1; }
  out=$(FIX_ITEM=rk-fix-doctor-test-count bash "$blk" 2>&1); rc=$?
  last=$(printf '%s\n' "$out" | grep . | tail -1)
  [ "$rc" -eq 0 ] && [ "$last" = "$mk" ] || { echo "FAIL: $sf's checkpoint rc=$rc, last line '$last'"; printf '%s\n' "$out" | tail -25; exit 1; }
done

# ---- (11) THE WHOLE SUITE, NO FAIL-FAST, WITH A FLOOR; fmt; clippy.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|test result:' | tail -30; exit 1; }
nres=$(printf '%s\n' "$out" | grep -cE '^test result: ')
nbad=$(printf '%s\n' "$out" | grep -E '^test result: ' | grep -vc ' 0 failed;')
tot=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="passed;") p+=$(i-1)} END{print p+0}')
[ "$nres" -ge 44 ] && [ "$nbad" = "0" ] && [ "$tot" -ge 753 ] || { echo "FAIL: $nres results, $nbad with failures, $tot passed — the floor is 44 results, 0 failures and 753 passed (740 at ca8f387 plus this stage's 13; s8a…s8i add more, and a floor is not a count)"; exit 1; }
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red (rc=$rc; never -D warnings on the CLI — ADR-0083)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (12) NOTHING DISPATCHED, NOTHING WRITTEN: the store's row counts and the worktree PATH SET are unchanged.
rows_after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$rows_after" = "$rows_before" ] || { echo "FAIL: the store moved during this block ($rows_before -> $rows_after) — a re-run block wrote to it, or something dispatched"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

echo "CP8J-END-OF-BLOCK reached"
```

**Stage close.** The stage closes in two commits, as the preamble requires. The first holds the work: items, intake
files, records at `proposed`, the lane fixture and tests, `ci.yml`, the pin move, the `s7i` re-point, the `s8`
amendments, and the Checkpoint 8j output. The second is the tracker: `PROGRESS.md` §1 with the first commit's hash,
the `runbook/phase-1/s8j-evidence.txt` lines that name that commit, the ledger rows, and `INDEX.md`'s execution order gaining `8j`.
Adversarial refuting rounds run until a round moves no code and no guard.

---
