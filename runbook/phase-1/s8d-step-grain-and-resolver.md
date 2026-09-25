# Step 8d — Step-grain commits and the complete reference resolver (Phase 1, part 8d)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8d` | Phase 1 | Checkpoint 8c | Checkpoint 8d | `s8c` | `s8d-ii` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45.* The draft (`a1d35ac`) was written before three ledger entries. What changed:
- **N2.** The story lane's `Spec` is whatever the mandatory specification stage selects, which is `bdd-scenarios`
  today. `user-story-ac`'s premise and its `spec.ambiguity` entry are gone (§8d.1 S4, §8d.6, §8d.9).
- **N3.** `change` is `null` at triage and routes to AC-079's `unknown_facet` gate, which the operator answers. S1
  is no longer a STOP (§8d.1, §8d.6, §8d.7, §8d.9; one new test).
- **N13.** `red-verify`'s selection comes from `s8b`'s catalog edit to `${red.outputs[*].tests_added}`. The resolver
  supports `[*]` and adds no implicit flatten. The value of `[*]` over a list-valued field is a new finding, named
  for R2 (§8d.1 S2, §8d.3). *D21-48 (2026-09-24): settled by X5, concatenation in element order.*
- **N14.** `kb:`, `checklist:` and `toolchain:<feed>` resolve through `s8c`'s declared tables, and an undeclared key
  refuses by name. The keys the lanes reach are enumerated (§8d.1 S3, §8d.3; one new test).
- **N15.** The commit key is separate. ADR-0097's memo key is untouched, and Checkpoint 6 does not re-run (§8d.1 S5,
  §8d.2; one new test).
- **N16.** Each per-step spec carries its agent step's `timeout_secs`, resolved through 4a's table and bounded by the
  activity budget. R1 names the ADR-0140 sentence this moves (§8d.2; one new test).
- **N12.** The resolver emits `artifact.read` for every resolved `artifact:` input, through `s8f`'s rk-store read
  seam, which lands after this stage (§8d.3; one new test).
- **N25.** `build-package`'s `built` check reads `s8c`'s widened `build` outcome (§8d.3, §8d.9; one new test).
- **`selection_of`'s route** out of `rk-cli` is stated (§8d.3), with a code-line floor in Checkpoint 8d (10).
- **Ownership.** The question loader and `.redkiln/processes/` are this stage's. `s8j` owns the pack-declared
  human-root questions, and `s8g` uses this loader (§8d.6).
- **The walk table is re-claimed.** 12 of 15 fix-lane and 10 of 13 story-lane activities walk (§8d.9).
- **The `bind_nodes` schema half** gains a mutant (M18), and six more mutants cover the new tests (§8d.10).
- **D21-44/D21-45.** No run-worktree path appears in this file, and no walk here recurses over `.redkiln/` as a whole
  (§8d.11). Checkpoint 8d re-runs only its entry gate and the one older checkpoint its edits make provisional. It
  asserts that no re-run can reach a real dispatch (§8d.11, §8d.12). The floor is re-derived (§8d.12 (11)).

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The next stage is `s8d-ii`** (D21-46 R1: *"A new stage **`s8d-ii`, "Run worktrees"**, sits between s8d and
  s8e"*). This checkpoint is `s8d-ii`'s entry gate.
- **The walk table is re-claimed at 11 of 15 and 9 of 13.** At Checkpoint 8d, `toolchain:backlog` is declared
  `unprovided` (`s8c` §8c.4) and `closure-record`'s `audit` reads it as a mandatory input, so `closure-record` refuses
  by name in both lanes. D21-46 R7 gives the producer to `s8f`, whose close flips both rows back to **walks** by dated
  line (`s8f` §8f.7, §8f.10). The verify rows' re-homing at `s8g` (D21-43 N23; D21-46 R5) is noted under the tables.
- **Checkpoint 8d (13) adopts D21-46's re-run convention and replay guard**: it skips its whole re-runs of 8c and 7g
  under `RK_CP_NESTED=1`, and otherwise launches both with `RK_CP_NESTED=1`, `REDKILN_LAUNCH_RECORD` unset and
  `CLAUDE_CONFIG_DIR` at an empty trap. *(D21-47: it also re-runs 7a, and re-runs 5 when `crates/rk-types` moved. See
  below.)*
- **The floor chain is recomputed once**: Checkpoint 8c's 833 + this stage's 52 = **885**, not the drafted 832.
  *Superseded 2026-09-24 (D21-47): 833 + 58 = **891**. See the fix note below.* *Superseded again by the final
  consistency pass 2026-09-24: 843 + 58 = **901**.*
- **Citations.** The resolver's optional-`null` rule cited ADR-0103 decision 4 here. *Reverted 2026-09-24 (D21-47):*
  it cites ADR-0032's own Decision, *"Absent `?` inputs are `null`"*, which still stands. ADR-0103 decision 4 is
  predicate semantics. The sentence *"an unresolved key is refused by name"* is D21-42 4a's, quoted as the ledger's.

*Adversarial-review fixes 2026-09-24 (D21-47)* (the review of `4a8c832`; D21-47's decisions and fix-round rules are
the authority). The ids are the review's numbering (B = BLOCKER, M = MAJOR, m = MINOR). Each finding and its fix:
- **B1 (feedback refs and `??`).** §8d.3's table gains a feedback-ref row (ADR-0042: the later step's value from the
  previous frame, `null` on the first pass) and a coalesce row (ADR-0034). Measured over `.redkiln/catalog/*.toml` at
  `fc80687`: **81** forward references in `inputs`, every one optional (`?`), 15 of them to a gate's answer; and
  **3** shipped `??` uses, all in `emit` steps. R2 cites ADR-0034, ADR-0042 and ADR-0043. Two new tests, and
  mutants M24 and M25. **Finding, named for R2:** 64 of the 81 are licensed by ADR-0042's letter (*"that step or one
  after it has `on_false`/`repeat` targeting the referencing step"*) and 3 by the repeat body (M4). The other **14**
  are licensed by neither. Two of them are on the lanes, and both are escalate-gate answers:
  `minimal-repro`'s `attempt` reading `${triage-gate.answer?}` and `tdd`'s `green` reading `${green-gate.answer?}`.
  §8d.3 resolves them by the escalate re-entry. R2 names all 14 against ADR-0042's lint rule for the operator.
  *D21-48 (2026-09-24): X8 took them.* 10 are licensed by the escalate re-entry, and the other 4 are removed from
  the catalog here (§8d.3).
- **B2 (= cross #11; 7a was never re-run).** Checkpoint 8d (13) re-runs 7a whole itself. It re-runs 5 when
  `crates/rk-types` differs from the entry commit (§8d.11).
- **M3 (= cross #3).** (12a) moves inside (13)'s non-nested branch. It also asserts that every `rk-cli` fixture config
  naming a `harness_binary` names a fixture child, with `rk-fixture-child-claude` in `pack-min`. `s8h` amends it
  (§8d.11).
- **M4 (`repeat.until` over a later step).** §8d.3 defines the repeat body and names the schema finding for R2. One
  new test, and M26. *D21-48 (2026-09-24): X6 took the reading.*
- **M5 (the journal row).** R1 names the row. `activity_committed` gains `step_id` and `iteration_path`, and
  completion is the `emit` step's row. R1 supersedes ADR-0150's forbids bullets 1 and 4 in part. One new test, and
  M27.
- **M6 (`artifact.read`).** The resolver calls a named, non-emitting stub. `s8f`'s rk-store seam is the only emitter
  (D21-47). The test and M19 are restated.
- **M7 (the review clone).** R5 routes `shallow_clone` on a distinct `review_clone` route added by record, not the
  `GitOps` route. It states how the clone is journalled and what working tree `confirm` needs (§8d.6).
- **M8 (non-idempotent tool steps).** R1 supersedes ADR-0150 decision 2 in part. A `side_effecting` registry column
  makes such a step commit and be served on re-entry. Two new tests, M28 and M30.
- **M9 (the walk-table floor).** The `lane_walk` tests print `LANE_WALK` lines, and (6a) floors them at 11 and 9.
- **M10 (the timeout test's crate).** The test moves to `rk-cli`'s `transcription::`, beside `bind_nodes`.
- **m11.** The resolver's home is named: `runtime::reference`, never AC-494's `src/resolver.rs`. The `null_out_refs`
  floor now reads all of `crates/rk-kernel/src` except `rigor.rs`, with a live control.
- **m12.** An undeclared key refuses even with `?`. Mutant M29.
- **m13.** The `deviate` refusal is flagged as a narrowing of D21-42 4d, for the operator's flip line (R3).
  *D21-48 (2026-09-24): X7 took the narrowing, and R3 states it.*
- **m14.** The attempt charge per iteration frame is stated (§8d.3).
- **m15.** The 7g amendment re-lists the modules that call `run_activity_steps` (§8d.11).
- **m16.** The stale "processes rename" is removed (§8d.10).
- **m17.** The AC-013 note becomes a dated in-place register amendment (R1).
- **The D21-47 rules.** The ADR-0032 citation is restored. The walk table states the `severity` dependency (`s8h`) and
  V4's reorder. The replay wrapper covers every cargo run and every replay in the block: a fake `claude` printing the
  pin, `REDKILN_LAUNCH_RECORD` unset, and a trap `CLAUDE_CONFIG_DIR` (§8d.12 (0a)). Records are located by the id
  recorded at write time.
- **Counts.** 59 new test names: 46 in `rk-kernel` and 13 in `rk-cli`, which is **58 net** of the one removal. The
  floor is 833 + 58 = **891**. *Superseded by the final consistency pass: 843 + 58 = **901**.*

*Final consistency pass 2026-09-24* (the eleven `s8` files read against each other after the D21-47 fix round;
`runbook/tools/nodispatch.sh` and `runbook/tools/fake-claude.rs` land with this pass):
- **The shared no-dispatch wrapper.** Checkpoint 8d (0a) no longer builds its own extensionless-script fake `claude`
  and trap directory. It sources `runbook/tools/nodispatch.sh` and runs `nd_setup` and `nd_enter`: a COMPILED fake
  that a Rust `Command::new("claude")` on Windows resolves, every other `PATH` directory holding a `claude` removed,
  `REDKILN_LAUNCH_RECORD` unset, and the trap `CLAUDE_CONFIG_DIR`. `nd_check` follows each cargo group, each replay in
  (13) and the criterion (3) run in (14), and closes the block at (15). The replays run as
  `nodispatch env RK_CP_NESTED=1 bash …`. §8d.11's *"extensionless script"* residual is replaced.
- **The entry sha is a literal.** The `RK_CP8D_ENTRY_SHA` input is gone. The commit at which Checkpoint 8c passed is
  written into (0)'s `CP8D_ENTRY=` line by a dated edit at stage open, and the block refuses while the line is empty
  (D21-47: a block carries its own record ids and shas, and no re-runner supplies one). A re-runner such as `s8j` sets
  nothing for it (§8d.11).
- **The floor chain, recomputed once, final.** Checkpoint 8c's expected count is 740 + `s8a`'s 9 + `s8b`'s **25** +
  `s8c`'s **69** = **843**, not 833, so this stage's floor is 843 + 58 = **901** ((11)). §8d.10 still gives 58 net new
  tests. Every earlier 832, 885 and 891 in these notes is history.
- **The walk table** states how `verdict` binds `${record.outputs}` after D21-47 V4 (the review row). Its closing
  paragraph names every later flip: `s8f`'s `debt-logging` rows beside its `closure-record` rows (13 of 15 and 11 of
  13), and `s8g`'s verify and release moves (§8d.9).
- **`s8f`'s stage-open STOP** is cited as `s8f` §8f.2 item 10, not §8f.1 (§8d.3).

*D21-48 (2026-09-24)* (the operator's answers to the questions the `s8` files left open; D21-48 is the authority, and
each decision is cited by quotation where it lands):
- *D21-48 (2026-09-24):* **X5 settles `[*]` over a list-valued field:** *"Concatenation in element order. Nothing is
  flattened implicitly anywhere else."* §8d.3's `[*]` row and its paragraph state the value as taken, R2 records it,
  and the flip-line fallback is gone.
- *D21-48 (2026-09-24):* **X6 settles the repeat body.** §8d.3's `repeat.until` paragraph states it as taken: one
  `repeat` frame per pass, and `until` evaluated after the body's last step. R2 names it as a schema finding, and the
  flip-line fallback is gone.
- *D21-48 (2026-09-24):* **X7 takes the `deviate` refusal:** *"Refused by name, naming ADR-0046's route OWED. R3
  states the narrowing of D21-42 4d."* R3's row and §8d.4's row state the narrowing as taken, not flagged.
- *D21-48 (2026-09-24):* **X8: R2 supersedes ADR-0042's Decision in part.** R2's row quotes the Decision sentence
  and the widened licence, and §8d.3 (*Which forward refs are licensed*) specifies the widened lint. The 14 refs the
  D21-47 fix round found licensed by neither were re-measured at `fc80687` by a read-only Python pass over
  `.redkiln/catalog`. **10** are licensed by the escalate re-entry, and **4** stay unlicensed. All four are
  approval-gate answers read by a drafting step that no cycle re-enters after the gate. This stage removes each one by
  an ADR-0072 edit: four activities are bumped, and each takes a changelog row. `vertical-slicing`'s bump moves the
  premise of Checkpoint 8b (3) and of `quality_block::adr0072_d2_edits_are_versioned_and_changelogged`, so this
  stage amends both in the same commit (D21-47's time-bound rule) and re-runs Checkpoint 8b whole (§8d.11). It adds
  four new tests and mutants M31–M35. Checkpoint 8d gains (4b), (6b) and (6c), and 8b joins (13).
- *D21-48 (2026-09-24):* **A citation.** §8d.3's feedback-ref row cited schema §4 for *"resumes **into the same
  step**"*. That sentence is in schema §3 (*"Escalation and the three exits"*), and the row now cites §3.
- *D21-48 (2026-09-24):* **Counts.** There are 63 new test names: 50 in `rk-kernel` and 13 in `rk-cli`. That is
  **62 net** of the one removal. The floor, re-derived at stage open from Checkpoint 8c's recorded count, is
  843 + 62 = **905** (§8d.10, (11)).
- *D21-48 (2026-09-24), floor propagation:* `s8c`'s D21-48 work (X4, X13, X15) moves its new tests from 69 to **78**, so Checkpoint 8c's expected
  count is 774 + 78 = **852**, and this stage's floor is 852 + 62 = **914** ((11)). The 843 + 62 = 905 above is
  superseded. Still re-derived at stage open from Checkpoint 8c's recorded count.

---

**Why this step exists.** The step executor `s7g` built walks the five closed step kinds and completes nothing:
*"**0 of 102** shipped activities complete a committing walk"* (ADR-0147 clause 3). Two records name why, and both
say the work is owed and unowned. ADR-0136 clause 4: *"The reference resolver is OWED … **It is not `s7g`'s**, and no
stage in this runbook currently claims it."* ADR-0150 clause 4: *"The repair is an idempotency key with a STEP
component, and no record currently routes it … Until a record does, no partial mechanism is built."* The ledger
adds the constraint that shapes this whole stage — D21-11: *"It does **not** license a partial resolver — a lookup
for `${x.answer}` but not `${x.outputs.y}` — and it does not make the resolver anyone's"*.

This stage takes both. It executes, in the order the ledger took them:

- **D21-41 (d), the hybrid** — *"Build the complete reference resolver (D21-11): output binding, iteration, check
  evaluation, emit binding and test-selection resolution. Route `shallow_clone`/`verdict_record` through
  verb-registry rows, populate `.redkiln/processes/`, and wire the `change` deriver."* Its cut list is **reversed**
  by D21-42 6a (*"Supersedes D21-41(c)/(d)'s narrowings"*); those producers are `s8f`'s and `s8g`'s, not this stage's.
- **D21-42 3a** — *"Step grain in the runtime. Per-step specs are carried on `RecordedSpec` (ADR-0138's warrant),
  each agent step commits under its own key, and re-entry resumes at the first uncommitted step."* It *"Supersedes
  D21-41(d)'s "or not at all""*, because D21-42 measured that branch dead: *"The mandatory implement stage's only
  Changeset producer (`tdd-red-green-refactor`) and (f)'s `user-story-ac` both need a second runnable agent step."*
  D21-43 N2 has since replaced (f)'s `user-story-ac` with whatever the story lane's mandatory specification stage
  selects. Today that is `bdd-scenarios`, which has four agent steps, so the measurement stands.
- **D21-42 3b** — *"`(item, activity@version, step_id, iteration path)`. The iteration path is the stack of for_each
  index, repeat count and loop-back count."*
- **D21-42 4c, the resolver side** — *"`artifact:` and `${…}` forms go through `selection_of` and the resolver"*.
  The adapter side (`"affected"`, `"e2e"`) is `s8c`'s.
- **D21-42 4d** — *"`on_false` is honoured as iteration. A `{to,max}` loop-back runs under a new iteration path,
  keyword forms follow the schema, and exhaustion or no `on_false` refuses as `StepFailedAfterCommit` (ADR-0144).
  Committed steps stand."*
- **D21-42 20** — *"The kernel appends `StepError::event()` for every dispatch, so fake and real harnesses behave
  alike, and AC-033's breaker counts those rows across processes."*

**What this stage consumes from Checkpoint 8c, read at open and cited, not assumed.** The outcome-bearing
`Toolchain::test` report (D21-42 1a — `tdd-red-green-refactor`'s `red-check` reads `report.failed > 0 and
report.passed == 0`, which today's port cannot answer); the registry-backed generic port method and its verb table
(1b); the per-toolchain `${toolchain.*}` table (4a); the `min(step, construction)` timeout (4b); and the adapter side
of the selection mapping (4c). D21-43 adds two more. The first is the declared `[checklist.*]`, `[feed.*]` and `[kb.*]`
tables in the same registry (N14: *"Undeclared keys are refused by name, and content is authored only for the keys
the lanes reach."*). The second is `build` *"widened to an outcome report as 1a did for `test`"* (N25).
**`selection_of` lives in `rk-cli`.** It is `crates/rk-cli/src/review_pass.rs`'s
`pub fn selection_of(command: &str) -> Option<Vec<String>>` at `a6349b6`, and `rk-kernel` cannot depend on `rk-cli`.
§8d.3 states the route. If Checkpoint 8c's records moved any of these, the sentence in this file that relies on it
takes a dated correction at open, before code.

### 8d.1 What this stage takes, and what it must measure before it writes code

**Records.** Each is written `proposed` in the form ADR-0164 uses (`### Option — ` headings, D21-42 24a), with the
next free id **at writing** — several stages write records in the same window, so no number is assigned here — and
flips only on the operator's dated ledger line. Each record's id is recorded at write time, by a dated line in this
table, and every later reference finds the record by that id. It is never found by title phrase or by an *"exactly
one superseding X"* count (D21-47). Two records can supersede the same ADR: R1 and `s8c`'s record B both supersede
ADR-0140 in part.

| # | record | supersedes / discharges |
| --- | --- | --- |
| R1 | **Step-grain commits** (3a, 3b): per-step specs on `RecordedSpec`, the commit key, re-entry at the first uncommitted step, the per-step memo key | supersedes in part **ADR-0143** decision 2 (narrowed: a runnable agent step with **no** per-step spec still refuses by name) and decision 4 in full; **ADR-0150** decision 1 in full; and **ADR-0138**, because ADR-0143's own forbids bullet reads *"Forbids widening AC-013's idempotence grain without superseding ADR-0138"* — the record names which ADR-0138 sentence it moves (its grain sentence sits under *"What this record does NOT decide"*, not in `## Decision`) and says so rather than inventing a clause. Discharges **ADR-0143 clause 5** and **ADR-0150 clause 4**. A dated **in-place AC-013 register amendment**, not a reading note (m17). D21-42 3a says it *"Supersedes in part … AC-013's per-activation grain"*, so AC-013's *"no committed step re-executes"* is amended in place to read at step-activation grain. It uses D15-2's form, a dated in-place amendment and not an erratum, and it lands with R1's flip. The commit key is **separate**: *"A separate commit key. ADR-0097's memo key is untouched."* (D21-43 N15). So ADR-0097 is not superseded, and Checkpoint 6 does not re-run. Each per-step spec carries its agent step's `timeout_secs` (N16, §8d.2). That is a per-step bound on the spec the Harness receives, so R1 also supersedes **ADR-0140** in part, for per-step specs only: decision 5's *"A future per-step bound may not be built by overloading it"*, and its forbids bullet *"forbids repurposing it as one"*. **The journal row (M5).** Each agent step's commit is one `activity_committed` row, appended when that step dispatches, as today. The row gains two payload members, `step_id` and `iteration_path`, and `is_committed` folds the commit key from them. No `EventKind` is minted. The activation's **completion** is one more `activity_committed` row, whose `step_id` is the activation's `emit` step, appended when the emit binding commits. A zero-agent activation writes only that row. So R1 also supersedes **ADR-0150's forbids** in part. Bullet 1 is *"Moving `activity_committed` to the end of the walk"*: the emit row is an added row at the walk's end, and no agent-step row moves. Bullet 4 is *"Restating "the walk happens inside an uncommitted activation""*: an activation with no emit row is *incomplete*, and its agent-step rows stand committed. **Side-effecting tool steps (M8).** R1 supersedes **ADR-0150 decision 2** in part. Its residual closes by re-execution only for a tool step whose verb is idempotent. This stage adds a `side_effecting` column to every row of `s8c`'s `.redkiln/registry.toml`, and a row without a value refuses the registry at load. A tool step whose verb is `side_effecting = true` commits under the commit key like an agent step. On re-entry it is served and never re-executed. Marked `true` here: `shallow_clone` and `verdict_record` (R5), `package` (`s8c`'s record V), and `item_create` and `release_tag`, whose rows exist and whose handlers `s8f` and `s8g` build. The rule, the column and the marks are this stage's |
| R2 | **The complete reference resolver**: every input-source form, output binding, prior-step names, ADR-0055's previous iteration, `for_each` and `repeat.until`, `check` evaluation, `emit` binding, and the resolver side of test selection | discharges **ADR-0136 clause 4** and **ADR-0142 clause 4**; supersedes **ADR-0148**'s refusal of `toolchain:test` (clause 4: *"This is reversed by a resolver, not by a revert"*) — `s8c` leaves it standing (its routing table: *"`test` \| ADR-0148's sentence, **unchanged** (s8d reverses it)"*) — while ADR-0148's forbids, that no declared selection is passed through as a test name or as an empty selection, stand. Executes these unchanged: ADR-0032 (*"Absent `?` inputs are `null`"*), ADR-0034 (the `??` coalesce), ADR-0035, ADR-0040, ADR-0042's second half (feedback refs, *"`null` on the first pass"*), ADR-0043 (the iteration counter in the step key, which the iteration path carries), ADR-0045, ADR-0055 and ADR-0103. ADR-0136's forbids all stand. **R2 supersedes ADR-0042's Decision in part** (*D21-48 (2026-09-24):* X8). The Decision reads *"A ref to a later-declared step is legal iff that step or one after it has `on_false`/`repeat` targeting the referencing step; it is `null` on the first pass."* X8 widens its licence: *"an escalate gate whose re-entry resumes into the referencing step also licenses the ref. The lint widens to match. Any ref still unlicensed is a lint error, fixed in the catalog in s8d."* The *"`null` on the first pass"* half stands. ADR-0042's first forbids bullet is the Decision's converse: *"Forbids a ref to a later-declared step where no step at or after the target has an `on_false`/`repeat` back-edge pointing at the referencing step"*. So the record names that bullet as moved in the same part, rather than leave a forbids bullet that the lint contradicts. The Decision says *"This goes into §1 "Edge" and lint rule 1"* of the schema. The record names both sites as read with the widening, and the schema text under `references/` is not edited. The widened lint, the census and the catalog fix are §8d.3's. R2 names a schema finding: `repeat.until` over a later-declared step, read as D21-48 X6 took it (§8d.3). R2 routes every resolved `artifact:` input through one call to a named read seam (N12). The only emitter of `artifact.read` is `s8f`'s rk-store seam (D21-47), and at this stage the call reaches a non-emitting stub (§8d.3). R2 fixes the resolver's home as `runtime::reference`, not `src/resolver.rs` (m11). It resolves `kb:`, `checklist:` and `toolchain:<feed>` through `s8c`'s declared tables (N14). It records the value of ADR-0035's `[*].field` over a list-valued field as D21-48 X5 took it, concatenation in element order (§8d.3), and it fixes the one home of `selection_of` (§8d.3) |
| R3 | **`on_false` as iteration** (4d) | executes ADR-0031, ADR-0044 and ADR-0103 clause 6's closed set, and ADR-0144's `StepFailedAfterCommit`; names ADR-0046's `deviate` form OWED (0 shipped uses — see §8d.4). Refusing it **narrows** D21-42 4d's *"keyword forms follow the schema"*, and D21-48 X7 took the narrowing (*D21-48 (2026-09-24)*): *"Refused by name, naming ADR-0046's route OWED. R3 states the narrowing of D21-42 4d."* R3 states it as taken (m13). R3 states the attempt charge per iteration frame (§8d.3, m14) |
| R4 | **Failure telemetry and the breaker's call site** (20) | the kernel becomes the writer of the `step_failed` / `step_cancelled` / `step_refused` / `rate_limited` row; design §5's row names the producer as *"the adapter, via the dispatch outcome"*, which takes a dated amendment under this record; `crates/rk-kernel/src/runtime.rs`'s table row *"AC-033's breaker mark \| **none exists**"* is corrected in the same change |
| R5 | **Two verb-registry rows and the question registry**: `shallow_clone` → the retained review clone, `verdict_record` → `review::confirm`; `.redkiln/processes/` populated and loaded | `shallow_clone` does **not** take `s8c`'s `git` route, which runs *"through the journalled `GitOps` seam"*: `prepare_review_clone` is a gix write outside `GitOps` (ADR-0098 decision 2). R5 adds a distinct `review_clone` route to the closed set **by record** (M7). `verdict_record` takes a second route added the same way, as `s8c` provides: *"`s8d`'s record may add a route for `verdict_record`. It does so by record, not by editing the match."* R5 states that the clone is journalled as a side-effecting tool step's commit (R1), not as a `git_op` row. It also states that the clone is not an ADR-0105 clause-5 wrapper write, because clause 5 names the review clone and lists its removal (§8d.6); lifts nothing ADR-0156 or ADR-0162 forbid, because both scope their bar — *"under this stage's scope"*, *"under this record's scope"*; executes ADR-0129 decision 3's *"Pack-declared questions … are Step 7e's, loaded into the same type"*, whose loader was never built. The loader and `.redkiln/processes/` belong to this stage alone (D21-43: *"The loader and `.redkiln/processes/` are **s8d**'s."*). `s8j` authors the human-root questions into that directory, and `s8g` loads through this loader |
| R6 | **The `change` deriver's call site in `build_item`** (ADR-0093 decision 2, AC-520) | carries D21-43 N3's route: a `null` `change` at triage opens AC-079's `unknown_facet` gate, which the operator answers (§8d.7). It takes a dated AC-079 reading note and no supersession. Its scored record already exists — *"`change` (0.72 [0.54, 0.85], 29)"* at pin `47faec9f` (ADR-0093 decision 2) |

**The five questions the draft raised, each now taken by D21-43** (2026-09-24). The draft stopped on S1 and S2 and
reported S3–S5. The ledger took all five, so none is a STOP. Each premise below was measured by the draft and is kept.
Each ruling is quoted.

- **S1 → N3: `change` at triage.** `facets.toml` says `derived` means *"computed over the changeset by a named
  `deriver`"*. The deriver it names, `R-change-extension-and-numstat-marker`, reads *"git metadata only: extension
  table + --numstat's binary marker"*. A fix or story item at `triage` has **no changeset yet**, because `rk-run/<item>`
  has nothing on it. So the deriver abstains (`null`). `tags_match` gives an unbound axis no tag (*"An axis the item
  does not bind cannot satisfy a constraint on it"*), so `minimal-repro` and `root-cause` would drop out at `analysis`
  in silence. Both are tagged `change:text`. **Taken:** *"`null` routes to AC-079's `unknown_facet` gate at triage,
  and the operator supplies it."* §8d.7 builds the route, and §8d.6 carries its question. The deriver still never
  defaults.
- **S2 → N13: `tdd-red-green-refactor`'s `red-verify` selection.** The draft read `inputs = { selection =
  "${red.outputs}" }` and the catalog's comment *"list of ImplReport; tool flattens tests_added (schema gap: no
  flatten)"*. **Taken:** *"A catalog edit to the existing `[*].tests_added` projection (ADR-0035), under ADR-0072."*
  The edit is `s8b`'s, beside that stage's other catalog edits, and Checkpoint 8b precedes this stage. So this stage
  reads `${red.outputs[*].tests_added}` from the catalog at open. The resolver supports the `[*]` projection and
  **adds no implicit flatten** (§8d.3). If Checkpoint 8b closed without the edit, `red-verify` refuses by name and
  §8d.9 takes a dated line.
- **S3 → N14: `kb:`, `checklist:` and `toolchain:<feed>` sources.** ADR-0040's grammar is *"`artifact:Kind[.field]`,
  `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]`, `${…}`, and literals"*. At `ca8f387` no `.kb/`, no checklist
  registry and no feed provider existed. **Taken:** *"Declared `[checklist.*]`, `[feed.*]` and `[kb.*]` tables in
  s8c's registry. Undeclared keys are refused by name, and content is authored only for the keys the lanes reach."*
  The tables and their content are `s8c`'s. The `smoke` checklist's content is `s8g`'s (`s8c`'s verb table: *"the
  shipped `smoke` checklist is `s8g`'s"*). This stage resolves through the tables and authors no content. The keys the
  two lanes reach were measured over the selected activities' `inputs` at `a6349b6`. That is the set `s8c`'s
  enumeration must cover, and the set this stage re-reads at open:

  | form | keys (reading activity) |
  | --- | --- |
  | `kb:` | `spec-conventions` (`fix-spec`, `bdd-scenarios`), `refactoring-catalog` (`tdd-red-green-refactor`), `retro-format` (`lightweight-retro`), `estimation-history` (`t-shirt-sizing`) |
  | `checklist:` | `fix-scope` (`fix-spec`), `smoke` (`smoke-test`, `rolling-deploy`), `closure` (`closure-record`), `retro` (`lightweight-retro`), `gherkin-quality` (`bdd-scenarios`), `risk-register` (`t-shirt-sizing`), `docs-diataxis` (`docs-update`) |
  | `toolchain:<feed>` | `backlog` (`closure-record`); optional `sbom?` (`sca-scan`) and `logs?` (`root-cause`) |

  That is four atoms, seven checklists and three feeds. Any key in the table that Checkpoint 8c's enumeration did not
  author turns its §8d.9 row by a dated line before code.
- **S4 → N2: the story lane's `Spec` producer.** `story.toml` has `specification = "mandatory"`, and `user-story-ac`
  declares `max_rigor = "recommended"`. `select::step1::admits` is `max_rigor.is_none_or(|ceiling| stage_rigor <=
  ceiling)`, so `user-story-ac` is **never admitted** there and `bdd-scenarios` (`min_rigor = "recommended"`, no
  ceiling) is. **Taken:** *"The Spec the story lane's mandatory specification stage selects"*, which is
  `bdd-scenarios` today. No catalog edit. It *"Supersedes D21-41 (f) and D21-42 11b's naming of `user-story-ac`"*.
  §8d.9's story row reads `bdd-scenarios`. `user-story-ac`'s `spec.ambiguity` leaves §8d.6's list, because no lane
  selects the activity that declares it.
- **S5 → N15: which key 3b names.** ADR-0097 forbids *"a fourth part on the idempotency key"*, and `rk-store`'s own
  table says *"`(item, activity_id@version, attempt)` — **three parts, never a fourth**"*. **Taken:** *"A separate
  commit key. ADR-0097's memo key is untouched."* So `IdempotencyKey` stays three parts, and `attempt` stays strictly
  increasing across the activation. `RunState::is_committed` gains the step component through the commit key.
  Checkpoint 6 does not re-run. ADR-0097 decision 2's component 7 is *"the loop iteration counter (0 outside a
  loop)"*. It is fed as that record writes it, from the innermost frame of the iteration path, and it is 0 outside a
  loop. **Residual, named and not claimed:** two nested paths that share an innermost counter share component 7, so
  the memo key alone cannot tell them apart. The memo is consulted by nothing on the run path (§8d.2), so no run reads
  such a collision today. Whoever wires the memo inherits the residual. Here, the commit key is what separates
  iterations.

### 8d.2 Step grain — per-step specs, the commit key, and re-entry (3a, 3b)

**Per-step specs are bound where the catalog activity is held, and completed where the values exist.** `bind_nodes`
binds one spec per **agent step**: that step's `role` → its card, that step's `outputs = "value:<Kind>"` → its
schema, and that step's `timeout_secs` (N16, below). Today it takes the schema from the activity instead —
*"`let kind = activity.provides.first()`"* — which is wrong for every agent step whose output is not `provides[0]`
(`tdd`'s `red` emits `ImplReport`; its `provides[0]` is `Changeset`). Run-time inputs (`${red.outputs}`,
`artifact:Spec`) are not known at bind time, so the **kernel** appends each step's resolved inputs to that step's
spec at dispatch; the binder's reserved keys (`worktree`, `item`) are never overwritten. The Claude adapter already
builds its user message from `spec.inputs` at dispatch (`fn user_message`), so nothing adapter-side moves.
`ActivitySpec` is **not** edited (ADR-0138's forbids; ADR-0099's five fields): R1 carries a list of them keyed by
step id on `RecordedSpec`, *"beside"* the activation's, which is the warrant ADR-0138 decision 2 names.

**Each per-step spec carries its agent step's bound.** D21-43 N16: *"Carried by each per-step spec (3a), resolved
through 4a's table and bounded by the activity budget. Owner: s8d."* Measured over `.redkiln/catalog/*.toml` at
`a6349b6`: **185** of the 298 `timeout_secs` declarations sit on `agent` steps, and every one of the 185 agent steps
declares one. **152** are literals and **33** are `${toolchain.*}` substitutions. All 102 activities declare
`budget.wall_clock_secs`. The binder resolves a substitution through `s8c`'s 4a table (D21-42 4a: *"an unresolved key is
refused by name"*). It sets the per-step spec's `timeout_secs` to `min(step, budget.wall_clock_secs)`. No literal exceeds its
activity's budget today (0 of 152), so the bound bites only on a substituted value or a later edit. The test pins
both arms. It lives in `rk-cli`'s `transcription::`, beside `bind_nodes`, the binder that computes the bound (M10). ADR-0140 decision 5 says of `ActivitySpec.timeout_secs`: *"A future per-step bound may not be built by
overloading it."* This is exactly such a bound, carried on the spec the Harness receives. So R1 supersedes that
sentence and the forbids bullet *"forbids repurposing it as one"* in part, for **per-step** specs. An activation's
own spec keeps the per-activity value that decision 5 describes.

**The commit key is `(item, activity@version, step_id, iteration path)`**, and every agent step activation commits
under its own. The iteration path is a stack of frames, outermost first — a `for_each` index, a `repeat` count, a
loop-back count — and is empty outside any loop. Two iterations of one step never share a key (AC-017's *"the loop
iteration counter is part of the step key"*, carried to the commit).

**Re-entry resumes at the first uncommitted step.** The walk runs in declaration order; a step activation whose
commit is in the journal is **served** from it, never re-dispatched (AC-013); the first step with no commit
executes. A `check` step and an **idempotent** `tool` step do not commit (3a names agent steps), so on re-entry they
re-execute, and a later `${…}` that reads one binds the re-executed value. That is how ADR-0150's named residual —
*"39 mapped `tool` steps across 23 activities"* lost to a crash after the commit — closes for idempotent verbs: by
re-execution. A tool step whose verb row declares `side_effecting = true` is the exception (M8). Re-executing it would
repeat a write: a second review clone, a second verdict record, a second item or a second tag. So it commits under the
commit key like an agent step and is served on re-entry. R1 states both halves and supersedes ADR-0150 decision 2 in
part (§8d.1).

**An activation's completion is its `emit`**, and `fsm::next_action`'s plan walk moves past a node only when the
emit binding has committed — not when its first agent step has. **The row, named (R1; M5):** each agent step's
commit is an `activity_committed` row carrying `step_id` and `iteration_path`, appended when **it** dispatches. The
completion is one more `activity_committed` row, whose `step_id` is the `emit` step's. No agent-step row moves to the
end of the walk; the emit row is added there. R1 supersedes ADR-0150's forbids bullets 1 and 4 in part to say so.
`step_grain::the_plan_walk_moves_past_a_node_only_once_its_emit_row_is_journalled` pins it.

**The memo key is per step.** `workflow::memo::StepContext` is assembled for each agent step with **that step's**
card digest (component 5) and schema id (component 8) — never the node's. ADR-0097's tuple itself is untouched
(N15; §8d.1 S5). What this does **not** claim: the memo is
still consulted by nothing on the run path (`grep` finds no `src/` caller of `memo::consult`), and this stage does
not wire it.

**Criterion (3)'s asserted half is re-proven at step grain.** `cross_process::kill_and_resume_n_step` is rewritten
over **one** activation with N ≥ 3 agent steps: the child commits two steps and dies; a different process re-enters;
steps 1–2 are served with byte-identical digests and step 3 dispatches once. The activation-grain body it replaces is
kept, renamed `cross_process::kill_and_resume_n_activation`, so the property it asserted stays asserted. `explain`
prints one `output_digest` per committed **step activation** (ADR-0128 decision 4's field), which is what s8's
criterion (3) operational arm diffs. A first divergence across the kill is **Halt 4**, not a retry.

### 8d.3 The complete reference resolver (D21-11)

**One mechanism with one record, or absent** (ADR-0136's forbids: *"The resolver is one mechanism with one record,
or it is absent."*). R2's resolver supplies, for every step it reaches:

| form | resolves to | absent / unresolvable |
| --- | --- | --- |
| `${<step>.outputs[.path]}`, `${<step>}`, `.len()` | the prior step's bound output in this activation, at the **current** iteration path | a mandatory ref to a step that has not run refuses by name (step id + reference); `${…?}` is `null` (ADR-0032's Decision: *"Absent `?` inputs are `null`"*) |
| `${<this-step>.outputs?}` inside a loop | the **previous** iteration's output, `null` on the first pass (ADR-0055) | — |
| `${<later-step>.outputs?}`, `${<later-gate>.answer?}`: a **feedback ref** (ADR-0042) | the value the later step bound in the **previous** frame of the cycle that re-enters this step. That cycle is a loop-back, a `repeat` pass or an escalate gate's re-entry (schema §3: *"resumes **into the same step** with `${gate.answer}` bound"*; the section was cited as §4 until D21-48's pass). It is `null` on the first pass (ADR-0042: *"it is `null` on the first pass"*) | it never binds the **current** frame's value. All 81 shipped forward refs in `inputs` carry `?` (measured at `fc80687`; 77 after this stage's four removals). A forward ref without `?` refuses by name at load, and so does one that no licence covers, `?` or not (D21-48 X8; *Which forward refs are licensed*, below) |
| `${a?} ?? ${b}` (ADR-0034) | the left operand's value unless it is `null`, otherwise the right operand's | the right operand resolves like any other ref. There are 3 shipped uses, all in `emit` steps (`appetite-check`, `branch-by-abstraction`, `unit-tests`) |
| `${<gate>.answer}`, `${<gate>.attachments.<Kind>}` | the `gate_answered` row's option / admitted attachment (ADR-0045; ADR-0129 decision 4) | `.outputs` on a gate refuses by name (ADR-0045) |
| `${item}` | the current `for_each` element (schema: *"runs one copy per element with `${item}` bound"*) | outside a `for_each`, refuses by name |
| `${step.outputs[*].field}` | ADR-0035's projection over a fan-out's list, one value per element in element order. Over a list-valued field, the value is the concatenation of the elements' lists in element order (D21-48 X5, below) | any other projection refuses by name. A bare `${step.outputs}` of a fan-out is never flattened into a test list (N13) |
| `${toolchain.<key>}` | `s8c`'s 4a table (D21-42 4a: *"an unresolved key is refused by name"*) | refused by name |
| `artifact:Kind[.field]` | the committed `emit` binding of the activation that produced `Kind` for this item, read from the journal (tier 2), through **one** read-seam call per resolved input (N12, below). `s8f` renders the bytes, and its seam emits `artifact.read`. This stage reads the binding | mandatory: refuses by name; `?`: `null`. Neither calls the seam |
| `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]` | the key's row in `s8c`'s `[kb.*]`, `[checklist.*]` or `[feed.*]` table (N14). An atom or checklist resolves to its content with its content digest (ADR-0097 component 6 for `kb:`). A feed resolves by calling the producing verb its row names, through `s8c`'s registry method | an undeclared key refuses by name **even with `?`** (`s8c`'s tables; m12). A mandatory declared key with no content also refuses, including a mandatory `unprovided` feed. `?` makes only a **declared** key with no content `null` (ADR-0032's Decision) |
| literals | themselves | — |

**The resolver's home (m11).** The resolver is `crates/rk-kernel/src/runtime/reference.rs` (`runtime::reference`). It
is **not** `crates/rk-kernel/src/resolver.rs`, which is AC-494's capability resolver (*"a missing capability is
`StepError::Unsupported`"*) and stays as it is. Checkpoint 8d (10)'s `null_out_refs` floor reads all of
`crates/rk-kernel/src` except `rigor.rs`, where the static lint's `null_out_refs` is correct.

🛑 **Unchanged, and each is asserted:** `null_out_refs` is never called on the execution path; `UnresolvedRef` is
never `Tri::Null`, `Tri::False` or a skip; a refusal names the step id **and** the reference (ADR-0136's forbids).

**Iteration.** `for_each = "${step.outputs.<list>}"` or `"artifact:Kind.<list>"` (ADR-0035) runs one activation per
element, each under its own path frame; AC-022's charge stays one. `repeat = { until, max }` re-runs the step while
`until` is not `Tri::True`, each pass a new frame; at `max` it takes `on_exhausted`. The shipped catalog spells
`on_exhausted = "escalate"` as schema item 15's documented pattern — *"an explicit `gate` step with `when` over the
looped step's status"* — so exhaustion proceeds to that gate step; an `escalate` with no such gate step refuses by
name.

**`repeat.until` over a later step (M4).** The schema defines `until` as *"<predicate over this step's outputs>"*
(its §1, Loop). But three shipped `until`s read a later-declared step: `tdd-red-green-refactor`'s `green`
(`${green-verify.outputs.status}`), `bdd-outside-in`'s `inward` and `implement-ue`'s `edit` (measured at `fc80687`).
If such an `until` is evaluated right after the repeating step, it reads a step that has not run. It is then
unresolvable and refuses by name. **D21-48 X6 took the reading** (*D21-48 (2026-09-24)*). The **repeat body** *"runs
from the repeating step through the last step `until` references"*, *"in declaration order, one `repeat` frame per
pass, and `until` is evaluated after the body's last step. The record names it as a schema finding."* So each pass
runs the whole body under a new `repeat` frame. An `until` over only the step's own outputs has a one-step body, as
the schema says. R2 records the reading and names the schema finding.
`reference_resolver::a_repeat_until_over_a_later_step_runs_the_body_through_that_step_each_pass` pins it over `tdd`'s
shape. The repeat body is also one of the three licences for a forward ref (*Which forward refs are licensed*, below).

**The attempt charge per frame (m14).** A `for_each` counts once, whatever its element count (AC-022, ADR-0053). Each
loop-back frame and each `repeat` frame is a pass for AC-021's per-pass count. Every agent or tool dispatch in a frame
charges that frame's pass. Each inner harness retry that the stream records charges the pass that dispatched it
(ADR-0122 decision 1; AC-026). A step served from its commit on re-entry charges nothing. Exhaustion is `Blocked {
reason: BudgetExhausted }`, which reaches AC-024 and AC-025 unchanged (ADR-0122). ADR-0037 is a load-time lint on the
declared field and does not move. R3 states this, and
`on_false_exec::a_loop_back_re_enters_its_target_under_a_new_iteration_path` asserts the per-frame charge.

**The `[*]` projection over a list-valued field (N13).** At `a6349b6`, `tdd-red-green-refactor`'s `red-verify` reads
`inputs = { selection = "${red.outputs}" }`. N13's edit to `${red.outputs[*].tests_added}` is `s8b`'s (§8d.1 S2).
The resolver adds **no** implicit flatten. A bare `${red.outputs}` still resolves to a list of `ImplReport` values,
and a selection built from it refuses by name. **Finding, named for R2:** ADR-0035 fixes the syntax
(*"`${step.outputs[*].field}` as the only projection"*) but not the value when `field` is itself a list, and
`tests_added` is a list (`spikes/p9/schemas.toml`: `lists  = ["files", "tests_added"]`). All three shipped uses
project `tests_added` into a `selection`: `bdd-outside-in`, `technical-spike`, and `tdd` after `s8b`'s edit. A list
of lists is not a selection. **D21-48 X5 settled the value** (*D21-48 (2026-09-24)*): *"Concatenation in element
order. Nothing is flattened implicitly anywhere else."* So `${red.outputs[*].tests_added}` is each element's
`tests_added`, element by element, with each list kept in its own order. This projection is the only place where a
fan-out's lists are joined, and a bare `${red.outputs}` is still never flattened. That is also the reading N13's
choice presupposes. R2 records it, and the three activities need no dated line.
`reference_resolver::for_each_fans_out_over_a_list_and_star_projects_its_outputs` asserts the concatenation over
two elements whose `tests_added` lists have different lengths.

**Which forward refs are licensed (D21-48 X8).** ADR-0042's Decision: *"A ref to a later-declared step is legal iff
that step or one after it has `on_false`/`repeat` targeting the referencing step; it is `null` on the first pass."*
D21-48 X8: *"an escalate gate whose re-entry resumes into the referencing step also licenses the ref. The lint widens
to match. Any ref still unlicensed is a lint error, fixed in the catalog in s8d."* So the load-time lint in
`rk_kernel::catalog` admits a ref from step R to a later-declared step L only under one of three licences. It
refuses any other ref by name, giving the activity id, the step id and the reference, whether or not the ref carries
`?`:
1. **The letter (ADR-0042).** L, or a step after it, has `on_false = { to = R, … }` or a `repeat` targeting R. A
   `repeat` targets only its own step, so for a later L the letter is met by `on_false`, and a `repeat` on R is
   licence 2.
2. **The repeat body (X6).** R declares `repeat`, and L lies within R's repeat body, which runs through the last step
   that R's `until` references.
3. **The escalate re-entry (X8).** R escalates, through `escalate = { question = Q }`, directly or with
   `on_exhausted = "escalate"`. And L, or a step after it, is a `gate` step whose `question` is Q. That gate is R's
   escalate gate in schema §9b item 15's pattern (*"an explicit `gate` step with `when` over the looped step's
   status"*), and answering it *"resumes **into the same step**"* (schema §3). A gate step with another question
   licenses nothing.

A licensed ref binds the previous frame's value, and it is `null` on the first pass (the feedback-ref row above; the
second half of ADR-0042's Decision stands). An escalate re-entry is a new frame of the referencing step, so a later
**tool** step's output is bound across it as a gate's answer is.
`reference_resolver::an_escalate_re_entry_binds_a_later_steps_previous_frame_value_and_is_null_first` pins that
over `minimal-repro-ue`'s shape.

**The census.** It was measured 2026-09-24 at `fc80687` by a read-only Python pass over `.redkiln/catalog/*.toml`,
which counted every `${<step>…}` in a step's `inputs` whose step is declared later in the same activity. There are
**81** forward refs. All 81 are optional, and 15 of them are gate answers. **64** are licensed by the letter, **3**
by the repeat body, **10** by the escalate re-entry, and **4** by none. The D21-47 fix round's 14 are the last two
groups. A Python census is a proxy for the Rust one, so
`forward_ref_lint::every_shipped_forward_ref_is_licensed_and_the_census_is_printed` re-measures it from inside the
test binary, and Checkpoint 8d (6b) asserts that reading.

| activity | step | reference | licence |
| --- | --- | --- | --- |
| `bdd-outside-in` | `inward` | `${green-gate.answer?}` | escalate re-entry |
| `bisect` | `bisect` | `${widen.answer?}` | escalate re-entry |
| `implement-ue` | `edit` | `${stuck.answer?}` | escalate re-entry |
| `incident-command` | `scribe` | `${observe.outputs?}` | escalate re-entry (the gate `end`, question `incident.end`, is declared after `observe`) |
| `incident-command` | `scribe` | `${end.answer?}` | escalate re-entry |
| `minimal-repro` | `attempt` | `${triage-gate.answer?}` | escalate re-entry (fix lane) |
| `minimal-repro-ue` | `attempt` | `${triage-gate.answer?}` | escalate re-entry |
| `minimal-repro-ue` | `attempt` | `${cook.outputs?}` | escalate re-entry (`triage-gate` is declared after `cook`) |
| `minimal-repro-ue` | `attempt` | `${run.outputs?}` | escalate re-entry (`triage-gate` is declared after `run`) |
| `tdd-red-green-refactor` | `green` | `${green-gate.answer?}` | escalate re-entry (both lanes) |
| `adr-record` | `draft` | `${accept.answer?}` | **none** |
| `spike-question-spec` | `draft` | `${accept.answer?}` | **none** |
| `structured-nl-requirements` | `draft` | `${conflicts.answer?}` | **none** |
| `vertical-slicing` | `slice` | `${approve.answer?}` | **none** |

**The four unlicensed refs, and their catalog fix.** Each one is an approval gate's answer, read by the drafting
step. In each activity, the only back-edge into the drafting step is a `check`'s `on_false` declared **before** the
gate: `complete` in `adr-record` and `spike-question-spec`, and `policy` in `structured-nl-requirements` and
`vertical-slicing`. None of the four drafting steps declares `escalate`. No cycle re-enters the drafting step after
its gate, so under ADR-0042 the value is `null` on every pass, and the input carries nothing. The fix removes the
input and adds no back-edge. A gate that sends a draft back would be a new escalation, which is a pack author's
decision and not a coherence edit. Each removal is an ADR-0072 edit: *"Edits to shipped activities made for catalog
coherence bump `version` and are listed in the pack changelog."*

| activity | step | removed from `inputs` | version |
| --- | --- | --- | --- |
| `adr-record` | `draft` | `answer = "${accept.answer?}"` | 1 → 2 |
| `spike-question-spec` | `draft` | `answer = "${accept.answer?}"` | 1 → 2 |
| `structured-nl-requirements` | `draft` | `resolution = "${conflicts.answer?}"` | 2 → 3 |
| `vertical-slicing` | `slice` | `answer = "${approve.answer?}"` | 2 → 3 (after `s8b`'s 1 → 2) |

- **Versions.** They were measured at `fc80687` as 1, 1, 2 and 1, and `s8b` §8b.5 moves `vertical-slicing` to 2. If
  Checkpoint 8b closed at another version, the row takes a dated line at open.
- **The changelog.** `s8b`'s `.redkiln/CHANGELOG.md` takes one row for each, carrying `<id>@<new version>`, the date
  and *"D21-48 X8"*, and quoting the old and new `inputs` line.
- **What the removal changes.** It changes no value that a step binds, because the removed key was always `null`. It
  does change the step's input bytes and so the user message the adapter builds, which is why the version moves.
- **The lattice is re-run** over `.redkiln/` with `s8b`'s `lattice::p6_lattice_over_the_shipped_pack`, which is
  ADR-0072's reachability test. `inputs` are not lattice axes, so no figure should move. If any figure moves, it is
  recorded with the edit.
- **Checkpoint 8b's premise moves.** Checkpoint 8b (3) pins `vertical-slicing:2` with an exact-version grep, and
  `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` pins the same triple against the live file.
  D21-47's rule is *"A time-bound guard in an earlier checkpoint is amended by the stage that moves its premise, in
  the same commit"*. So this stage's catalog commit amends both, each by a dated line. The version is read as **at
  least** the triple's new value, and the changelog row `<id>@<new>` is still required. `s8b`'s mutant M14 (which
  moves `user-story-ac` from 3 to 2) still dies, because 2 is below 3. Checkpoint 8b is then re-run whole inside
  Checkpoint 8d (§8d.11).
- **Red first.** The lint lands before the catalog edit. `conformance::shipped_pack_loads` and the census test are
  watched fail on exactly these four refs, and then the edit turns them green.

**`check` evaluation.** A `check` step's `expr` is evaluated against a `Context` built from **its own `inputs`
table** — `red-check`'s `report` is `${red-verify.outputs}` — through the same `predicate::evaluate`, never a second
evaluator (ADR-0136 decision 1). `Tri::True` continues; `Tri::False` goes to §8d.4; `Tri::Null` keeps ADR-0103/AC-079;
an `EvalError` refuses by name and is **never** read as false. `build-package`'s `built` check,
`report.status == 'passing'` over `${build.outputs}`, reads `s8c`'s widened `build` report (N25). A red build is
therefore a report whose status fails the check, and `on_false = "fail"` refuses by name. Before N25 that arm was
dead, because a red build was an `Err` and never a report.

**`emit` binding.** The `emit` step binds each key of its `artifacts` table — a reference, or a composing table as
`confirmation-test`'s `ConfirmationResult` is — to the provided kind, and its `evidence` list to the evidence refs;
the binding is what commits the activation. Measured over `.redkiln/catalog/*.toml` at `ca8f387`: **102 of 102**
activities carry exactly one `emit`, it is the **last** step, and its `artifacts` keys equal `provides`. The walk's
arm today is *"`StepVerdict::Emitted => break,`"*, which drops any later step silently; §8d.8 makes a step after an
`emit` a refusal by name at load.

**`artifact.read` (N12).** Every `artifact:` input the resolver resolves makes exactly one call to the read seam, and
that seam emits the tier-2 `artifact.read` row. This is the input half of ADR-0116's pair, whose quotation of design §5's row reads *"kind, item id,
`schema_version`, the tree hash served · every activity input"*. The resolver's `artifact:` arm is the only **caller**
of the seam. `s8f`'s rk-store seam is the only **emitter** (D21-47: *"`artifact.read` has exactly one emitter:
rk-store's read seam (s8f)"*). An optional input that resolves `null` makes no call, and neither does a refused one. **Ordering, named:** the seam
is `s8f`'s. D21-43 N12: *"`artifact.read` is emitted by s8d's resolver through s8f's rk-store read seam."* Checkpoint
8f comes after this stage, so this stage builds the call site as one kernel function,
`runtime::reference::read_artifact_input`. It serves the journalled binding and calls a trait object,
`ArtifactReadSeam`. At this stage the object's one production value is a named, **non-emitting** stub,
`UnwiredReadSeam`, which appends nothing. `rk-kernel` cannot depend on `rk-store`, so `s8f` re-points the composition
root's value to its rk-store seam, and that seam emits the row. The kernel function itself never appends a row, at
this stage or later. That is the shape `s8f`'s stage-open STOP expects (`s8f` §8f.2 item 10).
`reference_resolver::every_resolved_artifact_input_calls_the_read_seam_once_and_the_stub_emits_no_row` counts the calls
through a counting double and asserts that the stub appends no row. `doctor`'s `unread` rule groups `artifact.read` rows by `stage` and `kind`
(`crates/rk-cli/src/doctor/rules.rs`: `str_field("stage")`). Design §5's row does not list `stage`, so R2 names it
among the row's members. AC-159's doctor rule stays `s8f`'s.

**Test selection, the resolver side (4c).** `s8c` hands this stage *"`"affected"` → the Changeset's paths"*: the
resolver supplies the item's committed `Changeset` paths to `s8c`'s package-level affected-set mapping. A `${…}`
selection and an `artifact:ReproCase.command` selection are resolved here, then parsed into a test list by **one** `selection_of`. A second parser is not written for the kernel.
**The route, since `rk-kernel` cannot depend on `rk-cli`:** `selection_of` is a pure function over a string, so it
**moves** into `rk-kernel`, beside the resolver's selection mapping. `rk-cli`'s `review_pass` re-exports it with
`pub use`, so `rk_cli::review_pass::selection_of` keeps its path and its tests keep their import. `rk-cli` already
depends on `rk-kernel` (its manifest: `rk-kernel = { path = "../rk-kernel" }`), and the reverse edge is never added.
Checkpoint 8d (10) floors exactly one `fn selection_of(` code line under `crates/*/src`. If Checkpoint 8c already
moved the function, for example beside `s8c`'s `Selection` in `rk-ports`, that home stands by a dated line at open,
and the floor still holds. An unresolvable selection refuses by name and never reaches the port as an
empty selection (ADR-0148's reason stands: *"`cargo test affected`, which matches **no test and exits 0**"*).

**The change-summary range, OWED here** *(added 2026-09-25 at `s8a`, D21-51)*. ADR-0165 decision 4, as amended before acceptance, makes a range exactly `<base>..<head>`, and decision 6 refuses a single revision by name: under step-grain commits a changeset spans several commits, so `R^..R` would answer a `Text` omitting the earlier ones. `.redkiln/catalog/change-summary.toml` and `.redkiln/catalog/change-summary-ue.toml` bind `range = "artifact:Changeset.sha"`, a single sha, so once the resolver hands that value to the port it reaches the named refusal. **The catalog edit that binds a two-dot range** (for example `baseline..<sha>`) **is this stage's**, because this stage owns the resolver that binds it. It lands under ADR-0072: a version bump on each edited activity, a changelog row and a lattice re-run.

### 8d.4 `on_false` as iteration (4d)

Measured over `.redkiln/catalog/*.toml` at `ca8f387`: **94** `on_false` values — `{ to = …, max = N }` **66**,
`"continue"` **15**, `"fail"` **13**, `{ to = "deviate", … }` **0** — all inside ADR-0103 clause 6's closed set as
`predicate::classify_on_false` reads it.

| form | on a `check` that does not hold |
| --- | --- |
| `{ to, max }` | re-enter `to` (an earlier step, ADR-0031) under a **new loop-back frame**; steps from `to` onward run again; committed step activations of the earlier frame **stand** |
| after `max` re-entries | refuse as `ReplayError::StepFailedAfterCommit` when a step committed, `refused` otherwise (ADR-0144) |
| `"continue"` | proceed to the next step |
| `"fail"` | refuse by name, as the exhausted case |
| absent | refuse by name, as the exhausted case |
| `{ to = "deviate", … }` | refuse by name, naming ADR-0046's route OWED — 0 shipped uses, and R3 records it rather than building a mid-walk `step7` amendment no activity asks for. Refusing it **narrows** D21-42 4d's *"keyword forms follow the schema"*, and D21-48 X7 took the narrowing: *"Refused by name, naming ADR-0046's route OWED."* R3 states it (m13; *D21-48 (2026-09-24)*) |

The exit code stays the class's — ADR-0144 decision 6, *"No row of §7d.4's exit table is added, moved or
renumbered"*, whose table is s7d §7d.4. The class R3 gives an exhausted loop is named in R3; `StepError::Unsupported` with *"`on_false`
has no executor"* stops being true the day this lands and is not reused.

### 8d.5 Failure telemetry and the breaker (20)

**The kernel appends the row, once, for every dispatch whose outcome is an error**, from `report.outcome()` through
`StepError::event()` — the total map `rk-types` already carries (*"The one event arm this variant maps to
(AC-360)"*). Today `rk-ports`' `FakeHarness` writes that row itself (`fn event_for`) and `ClaudeHarness` writes none,
which is why the StepError class reached the journal under the fake only. After this stage the fake writes **no**
error row of its own, so both harnesses produce exactly one. `rate_limited` is already appended by the kernel on the
AC-030 path; R4 maps `StepError::RateLimited` onto that row rather than writing a second.

**The row carries what the breaker folds.** `BreakerState::from_events` refuses a family-scoped row without `class`
and `at` — *"`family` is present but `at` is absent or not an integer"* — so the row carries `family` (the role's
pack family, as the breaker key already is) and `at` (the journaled instant). R4 states these as members added to
ADR-0095 decision 2's event arm, and whether that moves a `## Decision` sentence.

**The breaker gets its call site.** `from_events` has no `src/` caller at `ca8f387`. Before an agent dispatch the
kernel folds the store's family-scoped rows with `Pack::require_breaker_threshold()` and refuses a dispatch whose
family is open (AC-033), per family (AC-034), from the store and never from memory (AC-035). D21-42 20's *"(durable
mark)"* is read as: **the journalled rows are the mark**, and the fold's *"INSERT OR REFRESH"* mark is derived from
them — no new `EventKind`. If R4 finds AC-033's THEN — *"a durable breaker mark for the family is written"* — needs a
row of its own, the kind is minted by ADR-0120's route and the record says so. `K` is the pack's
`consecutive_platform_failures = 3`, which still carries *"NO RECORD"*; its record is `s8h`'s (D21-42 19c).

### 8d.6 Two verb rows, and the question registry

**The rows.** `adversarial-review` runs `toolchain:shallow_clone` (`clone`) and `toolchain:verdict_record`
(`record`). Both are routed by rows in `.redkiln/registry.toml`, the verb table Checkpoint 8c ships — not by new port methods
— onto what exists: `shallow_clone` on a `review_clone` route R5 adds by record, to
`rk_git::clone::prepare_review_clone` (ADR-0098's retained gix clone), and `verdict_record` on a second route R5 adds
by record, to `rk_kernel::review::confirm` (ADR-0162, which classifies findings by re-execution). `rk-git` is a concrete crate and not a port (`rk-kernel`'s
manifest: *"`rk-ports` declares five ports and no sixth, and its `git_port_is_not_a_port` fixture is what refuses
the sixth"*), and `rk-kernel` opens no files, so R5 names which side performs the clone and hands the kernel its path.

**The clone is not on the `git` route, and how it is journalled (M7).** `s8c`'s `git` route runs *"through the
journalled `GitOps` seam"*, and `s8e`'s N7 `git_op` row journals `GitOps` invocations. `prepare_review_clone` calls
gix's `prepare_clone_bare` or `prepare_clone` directly, so it writes no `git_op` row, and a replay would clone again.
So R5 adds a distinct route. Both verbs are `side_effecting = true` (R1), so each commits under the commit key.
`clone`'s commit holds the destination path, the sha and the working-tree choice. On re-entry or replay it is served,
and no clone is made. The clone is not an ADR-0105 clause-5 wrapper write, and no supersession is taken. The gix write
is ADR-0098 decision 2's retained one, and clause 5 already names the review clone and lists its removal. D21-47's
clause-5 rule covers the wrapper's writes. **The working tree.** `confirm` re-executes each finding's test, so it needs
a checked-out tree at the sha, and the row passes `WorkingTree::Declared`. But `prepare_review_clone` calls
`fetch_only` in **both** arms (read at `fc80687`), so today even the `Declared` arm checks out no tree. R5 decides how
that arm checks out, for example gix's checkout after the fetch, inside ADR-0098 decision 2's retained write.
`verb_rows::shallow_clone_routes_to_the_retained_review_clone` asserts a checked-out file at the sha, as well as AC-191's
one-commit object set.

**The question registry.** Production `answer` passes `&QuestionRegistry::default()`, so every gate today is
answerable only as the default question (`approve`, `changes-requested`), and `question.rs` still says
*"Pack-declared questions are Step 7e's"*. `.redkiln/processes/` is empty (ADR-0156 blocker 2). This stage adds a
TOML loader into the existing `QuestionRegistry::with_question` and `bind` seams, used by both `run` (a gate step's
kind must be declared, or the load refuses by name) and `answer`. The file home reconciles D21-41 (d)'s
`.redkiln/processes/` with design §4.5's *"`questions.toml` (question ids, closed option sets, `attaches`)"*: the
default reading is `.redkiln/processes/questions.toml`, and R5 fixes it.

**This loader is the only one** (D21-43's integration call). `s8j` authors the pack-declared human-root questions for
`IntakeRecord` and `BugReport` into `.redkiln/processes/`. `s8g` drops its own `.redkiln/questions.toml` and loads
through this loader. Neither stage writes a second loader. The loader reads `.redkiln/processes/*.toml` and does not
recurse, so D21-44's `.redkiln/worktrees/` is never under it.

**The entries are transcribed, not invented, except one.** Nine entries are every gate and `escalate` question the
two lanes' selected activities declare. They were enumerated from `.redkiln/catalog/*.toml` at `a6349b6` and found,
text and options verbatim, in `spikes/p9/questions.toml`. The tenth is N3's: the question the operator answers the
`unknown_facet:change` gate with (§8d.7).

| question | declared by (lane) |
| --- | --- |
| `repro.cannot-reproduce` | `minimal-repro` (fix) |
| `root-cause.no-hypothesis`, `root-cause.accept` | `root-cause` (fix) |
| `draft.accept` | `fix-spec` (fix), `bdd-scenarios` (story) |
| `tdd.green-exhausted` | `tdd-red-green-refactor` (both) |
| `confirmation.not-fixed` | `confirmation-test` (fix) |
| `scan.accept-findings` | `sca-scan` (both) |
| `review.blocking` | `adversarial-review` (both) |
| `closure.signoff` | `closure-record` (both) |
| `facet.change` | N3's triage gate (both), bound to the gate kind `unknown_facet:change`. Its options are transcribed from `facets.toml`'s `values = ["text", "binary", "generated"]`, in that order. Its text is the one entry not taken from `spikes/p9` |

`spec.ambiguity` is not an entry. The only activity that declares it is `user-story-ac`, and no lane selects that
activity (N2; §8d.1 S4). The intake and bug-report questions that supply `IntakeRecord` and `BugReport` (D21-41 (f))
are `s8j`'s. They are not authored here, and they load through this loader. Every other `unknown_facet:*` gate keeps
the default question (`question.rs`: *"The default binding is the absence of one"*).

### 8d.7 The `change` deriver in `build_item`, and `change` at triage (N3)

`build_item` today *"nulls every non-kernel facet the record omits"* and accepts a `[facets]` value for any
non-kernel facet, derived ones included — the packet's own measurement. R6 wires
`R-change-extension-and-numstat-marker` into it, ported from `spikes/p1/derive-facets.py`'s `derive` (text: a path
that is neither generated nor binary; binary: `--numstat`'s `-\t-` marker or the `unreal` table; generated: the
generated table) and run through `rk-git`. Its input range is the item's changeset on `rk-run/<item>`. At `triage`
nothing is on that ref, so the range is empty and the deriver **abstains** (`null`). Because `facets.toml` declares
`source = "derived"`, a record-supplied `change` is refused by name rather than read as a human value on a derived
facet — R6 states that reading of ADR-0093 decision 1's source classes. An empty range abstains, never `[]` and never
`"text"`.

**`null` at triage routes to AC-079's gate.** D21-43 N3: *"`null` routes to AC-079's `unknown_facet` gate at triage,
and the operator supplies it."* Without the route the null is silent. `change` is first read by `tags` at step 4, and
`tags_match` never opens a gate. The rows that would open AC-079's gate through step 2 come too late. They are
`'text' in change` in both packs' `[[stage.verify.require]]`, and the same expression in `change-summary`'s and
`equivalence-boundary`'s `applicable_if`. All are read at `implement` or `verify`, after `analysis` has already
dropped `minimal-repro` and `root-cause`. So R6 opens `gate_opened { reason: unknown_facet, axis: "change" }` at
triage, once (AC-079: *"counted once per read facet"*), before any stage's selection reads `change`. Its question is
§8d.6's `facet.change`. The answered option binds `change` for the item as a one-element set. The binding is
journalled as the gate's `gate_answered` row and folded from it. It is never written back to the item record, where a
`change` value stays refused. A later step-2 read of an answered axis opens no second gate. R6 states when the deriver
re-runs once a changeset exists, and how a derived value that disagrees with the answer is reported. The walk test
binds only the answer. A dated AC-079 reading note says that the triage gate opens for a derived facet whose first
reader is a tag.

### 8d.8 Four executor defects this work touches, each verified at `ca8f387`

1. **A step after `emit` is silently dropped.** The arm is *"`StepVerdict::Emitted => break,`"*, `catalog.rs`
   enforces no position for `emit`, and `walk_semantics::an_emit_step_ends_the_walk_and_a_later_tool_step_never_runs`
   asserts the drop as intended. 0 shipped activities are affected (measured above), so the repair is a load-time
   refusal by name, and that test is replaced by
   `emit_binding::a_step_after_emit_is_refused_by_name_and_never_silently_dropped`.
2. **ADR-0143's refusal reports a committed activation as not committed.** The second-agent arm returns
   *"`return Ok(ActivityReport::refused(now, cause));`"* even when the first dispatch committed, and `refused` builds
   an opened report, for which *"`self.dispatches == 0 && self.commit.is_none()`"* makes `skipped()` true and
   `committed()` false. `dispatch_through` then answers `NotCommitted` for an activation whose row is in the journal —
   ADR-0144's named lie. `second_agent::a_second_runnable_agent_step_refuses_and_the_commit_survives` checks the
   refusal **string** for `"COMMITTED"` and never the report. The repair is ADR-0144's shape:
   `done.with_step_failure(cause)`.
3. **The `check` arm does NOT swallow `Ok(false)` or `Err` at `ca8f387`** — the brief's wording is stale. `s7g`'s
   finding (21) repaired it and `check_cannot_dispatch`'s C4 pins it: `Ok(false)` fails the walk with
   *"did not hold, and `on_false` has no executor"*, and `Err(e)` fails it with `e`. What this stage must keep is
   the **distinction**: §8d.4 routes `Ok(false)` and nothing else, and an `Err` is never read as false
   (`check_eval::an_unresolvable_check_input_refuses_and_is_never_read_as_false`).
4. **ADR-0141 clause 4's guard is built, and the production path still skips nodes.** `rk-cli`'s
   `transcription::the_production_path_transcribes_a_step_list_and_never_supplies_none` and
   `transcription::the_spec_a_real_run_binds_carries_its_steps` exist. But `bind_nodes` does *"`find_map(|s|
   s.role.clone())`"* and `continue`s when no step nominates a role, so a node with **no agent step** is never bound,
   and the run's supplier then refuses it: *"`{node}` was first seen inside the loop and step (8) bound no spec for
   it"*. Six of the fix lane's fifteen selected activities have no agent step (`change-summary`, `confirmation-test`,
   `smoke-test`, `sca-scan`, `build-package`, `rolling-deploy`). The guard's own residual — *"one fixture binding ONE
   node"* — is exactly the branch it cannot see. R1 states how a zero-agent activation's binding is carried, as an
   explicit absence of agent specs and never a synthesised card (ADR-0143's forbids). The same function takes every
   node's schema from *"`activity.provides.first()`"*. That is the second half of the defect, and §8d.2 repairs it
   per step (M18).

### 8d.9 The walk table — a claim, measured by a test

**What "walks" means here.** The executor reaches the activity's `emit` and commits it, over the **shipped**
`.redkiln/` pack and catalog, with a scripted `FakeHarness` returning happy-path typed values per step, a scripted
`Toolchain` over the verbs Checkpoint 8c routes, specs supplied by the test (the shipped cards and schemas are
`s8h`'s), and each stage's `requires` seeded from a fixture artifact so one stage's refusal does not mask the next
stage's verdict. A happy-path gate is answered with its question's first option. That includes N3's
`unknown_facet:change` gate at triage, which is answered `text`. It is **not** a real run, and it says nothing about
`redkiln run`, which is `s8e`'s. The rows below assume two things from earlier stages. One is Checkpoint 8b's
`red-verify` edit (N13). The other is that Checkpoint 8c's enumeration covers every key in §8d.1 S3's table, less
the `smoke` checklist's content (N14). If either differs at open, the table takes a dated line before code.

The two tests — `lane_walk::the_fix_lane_walk_table_holds_for_a_rust_text_bug` and
`lane_walk::the_story_lane_walk_table_holds_for_a_spike_toml_story` — assert the selected set per stage and each
activity's verdict **exactly**. If a test measures a different row, **the table is corrected by a dated line; the
test is not bent to the table.** The selection column below is a reading of `select::step1::admits`, method ranks
and tags (plus the packet's hand application), not a run. **Each test also prints one line (M9):**
`LANE_WALK <lane> walked=<n> refused=<m>`. Checkpoint 8d (6a) floors `n` at 11 for the fix lane and 9 for the story
lane. A later stage that flips a row to **walks** raises `n` and needs no edit there. Lowering a floor needs the
operator's dated ledger line first.

**The fix lane** — `bug-standard`, item `kind = "bug"`, `cos = "standard"`, `has_repro = true`,
`criticality`/`blast_radius`/`size` = `low`, and `change = ["text"]` from the triage gate's answer (N3):

| stage | rigor | selected (claim) | after this stage | first stop, and whose |
| --- | --- | --- | --- | --- |
| `triage` | mandatory | none — no `[stage.triage]` (ADR-0156) | the `unknown_facet:change` gate opens and is answered `text` (N3) | `stage_empty` is `s8e`'s (D21-42 7) |
| `analysis` | recommended | `minimal-repro`, `root-cause` | both **walk** | `root-cause`'s `policy` is the form `s8b` leaves (D21-41 h); its `telemetry = "toolchain:logs?"` is an optional feed (N14) |
| `specification` | optional | `fix-spec` | **walks** | `draft` reads `kb:spec-conventions` and `self-check` reads `checklist:fix-scope`, both through `s8c`'s tables (N14) |
| `design`, `planning` | optional | none (every `require` row keyed on a `high`/`med` facet) | — | — |
| `implement` | mandatory | `tdd-red-green-refactor`, `change-summary` | both **walk** | `tdd`'s `red-verify` reads `s8b`'s `${red.outputs[*].tests_added}` (N13), and `refactor` reads `kb:refactoring-catalog` (N14). `change-summary` needs `ChangeSummary`'s `Text` variant constructible, which is `s8a`'s (D21-42 2) |
| `verify` | mandatory | `equivalence-boundary`, `confirmation-test`, `smoke-test`, `sca-scan` | `equivalence-boundary`, `confirmation-test` and `sca-scan` **walk** (`sca-scan`'s `toolchain:sbom?` is an optional feed); `smoke-test` refuses | `smoke-test`: its `smoke` step's `checklist:smoke` has no content until `s8g` (*"the shipped `smoke` checklist is `s8g`'s"*, `s8c`). `sca-scan`'s `clean` is `s8b`'s `unrated == 0` form (N25) |
| `review` | mandatory | `adversarial-review` (`code-review` is not admitted: `max_rigor = "recommended"`) | **walks** | `verdict` is `s8b`'s re-execution form (D21-42 11a). `s8b` moves it **after** `record` (D21-47 V4: `verdict` reads the outcome `record` produces), so the walk runs `record` before `verdict`. `verdict`'s input `${record.outputs}` is then a prior-step reference in this activation (§8d.3's first row), bound to `record`'s committed `verdict_record` output. Before V4 it read a later step without `?`, which refuses at load. If `s8b`'s spelling of that binding differs at open (`s8b` §8b.5 leaves the spelling to the commit that keeps `shipped_pack_loads` green), this row takes a dated line (*final consistency pass 2026-09-24*). `clone` and `record` route by §8d.6's rows, and both are side-effecting (R1) |
| `release` | recommended | `build-package`, `rolling-deploy` | `build-package` **walks**; `rolling-deploy` refuses | `build-package`: `built` reads `s8c`'s widened `build` outcome (N25), and `package`'s handler is `s8c`'s record V. `rolling-deploy`: `deploy` has no registry row and is refused by bare name; it is replaced by `s8g`'s `tag-release` (6b) |
| `closeout` | recommended | `debt-logging`, `closure-record`, `lightweight-retro` | `lightweight-retro` **walks**; `debt-logging` and `closure-record` refuse | `debt-logging`: `item_create` — `s8f` (6a). `closure-record`: `audit` reads `toolchain:backlog` as a mandatory input, and the feed is `unprovided` until `s8f` builds its producer (D21-46 R7; `s8f` §8f.7), so it refuses by name; it also reads `checklist:closure`. `lightweight-retro` reads `kb:retro-format` and `checklist:retro` (N14) |

**Claimed: 11 of 15 walk, and 4 refuse by name** (`smoke-test`, `rolling-deploy`, `debt-logging`, `closure-record`).
Each refusal is owned above: `s8g`, `s8g`, `s8f` and `s8f`. *Re-claimed 2026-09-24 (consistency pass; D21-46 R7):*
the draft's 12 of 15 counted `closure-record` as walking over an `unprovided` mandatory feed, which is the silent
`null` N14 forbids. `s8f`'s close flips its row back to **walks** by dated line (`s8f` §8f.10).

**The story lane** — `story`, item `kind = "story"`, entered at `triage`, `cos = "standard"`, low facets,
`external_dep = false`, `novelty = low`, and `change = ["text"]` from the triage gate's answer (N3), for a story whose
change touches `spikes/p9/packs/spike.toml`:

| stage | rigor | selected (claim) | after this stage | first stop, and whose |
| --- | --- | --- | --- | --- |
| `triage`, `analysis` | mandatory, optional | none (no stage section) | at `triage`, the `unknown_facet:change` gate, answered `text` (N3) | — |
| `specification` | mandatory | `bdd-scenarios` (N2: the mandatory stage's selection, since `user-story-ac`'s ceiling keeps it out) | **walks** | `draft` reads `kb:spec-conventions`, and `self-check` reads `checklist:gherkin-quality` (N14). Its `IntakeRecord` is seeded here; in a real lane it comes from `s8j`'s human-root question |
| `design` | recommended | none (`require` rows keyed on `external_dep`, `size`, `novelty`) | — | — |
| `planning` | optional | `t-shirt-sizing` (`applicable_if = "size < high"`) | **walks** | `size` reads `kb:estimation-history` (N14) |
| `implement` | mandatory | `tdd-red-green-refactor`, `change-summary`, `docs-update` (`Docs` is required at `rigor >= recommended`) — whether `docs-update` (rank 5) or `tdd` provides `Changeset` is the test's to measure | all three **walk** | as the fix lane; `docs-update` reads `checklist:docs-diataxis` (N14) |
| `verify` | mandatory | `equivalence-boundary`, `smoke-test`, `sca-scan` | as the fix lane | as the fix lane |
| `review` | mandatory | `adversarial-review` | **walks** | — |
| `release` | mandatory | `build-package`, `rolling-deploy` | as the fix lane | as the fix lane |
| `operate` | optional | none (`SLOReport` keyed on `blast_radius >= high`) | — | — |
| `closeout` | optional | `closure-record`, `debt-logging` | both refuse | as the fix lane: `debt-logging` on `item_create`, `closure-record` on the `unprovided` `toolchain:backlog` feed, both `s8f`'s |

**Claimed: 9 of 13 walk** (`bdd-scenarios`, `t-shirt-sizing`, `tdd-red-green-refactor`, `change-summary`,
`docs-update`, `equivalence-boundary`, `sca-scan`, `adversarial-review`, `build-package`), and **4 refuse by name**,
the same four as the fix lane with the same owners (*re-claimed 2026-09-24, consistency pass; D21-46 R7*). Neither
lane walks end to end after this stage, and the tables say why by owner. That is the result, not a failure of the
stage.

**The `severity` dependency (D21-47; cross #4).** 39 catalog `expr`s, one in each of 39 activities, read
`findings.none(severity in ['blocking'])`. Three sit on the lanes' walk, each in a `policy` check step: `fix-spec`'s,
`bdd-scenarios`' and `equivalence-boundary`'s (measured at `fc80687`). `rk-types`' `Finding` has no `severity` field until `s8h` adds
it, typed by `s8b`'s 11c vocabulary. Until then the evaluator's `none` over an **empty** findings list is true without
reading `severity`. Over a non-empty list it is `UnknownField`. So these three rows **walk** here only because the
scripted harness returns an empty findings list, and the `lane_walk` fixtures say so in a comment. A walk with a
non-empty findings list at those steps refuses until `s8h`, and proving that walk is `s8h`'s.

**These tables are Checkpoint 8d's, and two later stages move rows by dated line.** `s8f`'s close flips both
`closure-record` rows to **walks** once its `backlog` producer lands (D21-46 R7). *(Final consistency pass 2026-09-24:
the same dated line flips both `debt-logging` rows once `s8f`'s `item_create` handler lands (D21-47, cross #6; `s8f`
§8f.7, §8f.10), so the claims then read 13 of 15 and 11 of 13.)* `s8g` re-homes `equivalence-boundary`
from `verify` to the `implement` stage (D21-43 N23), with `integration-tests` beside it (D21-46 R5), so the verify
rows above read as they stand at this stage and the implement rows gain them there. Each move takes a dated line in
this section and moves the two `lane_walk` tests' expectations in the same commit. *(Final consistency pass
2026-09-24: `s8g` §8g.1's move 1 measures that on these two lanes the implement rows gain nothing, because the
re-homed escalation is keyed on `criticality >= high`. The same move flips `smoke-test` to **walks** and replaces
`rolling-deploy` with `tag-release`, and its dated line here says so.)*

### 8d.10 Tests, red first, and the mutants that must die

Every test below is authored, run, and **watched fail for the right reason** before the code it pins
(golden constraint 12). Every name is module-qualified in its crate's `--test it` binary.

**`rk-kernel`:**

- `step_grain::` — `each_agent_step_dispatches_under_its_own_spec_and_commits_under_its_own_key` (it also asserts that
  each commit is one `activity_committed` row carrying `step_id` and `iteration_path`, R1),
  `a_runnable_agent_step_with_no_per_step_spec_refuses_and_the_report_stays_committed` (defect 2),
  `re_entry_resumes_at_the_first_uncommitted_step_and_re_dispatches_no_committed_one`,
  `two_iterations_of_one_step_never_share_a_commit_key`,
  `the_memo_key_of_an_agent_step_carries_that_steps_own_card_digest`,
  `an_activation_with_no_agent_step_walks_to_its_emit_and_completes`,
  `the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts` (N15),
  `the_plan_walk_moves_past_a_node_only_once_its_emit_row_is_journalled` (R1's completion row, M5),
  `a_side_effecting_tool_step_commits_and_is_served_on_re_entry_never_re_executed` (R1, M8).
- `cross_process::kill_and_resume_n_step` (rewritten, §8d.2) and `cross_process::kill_and_resume_n_activation`
  (the preserved body).
- `reference_resolver::` — `every_input_source_form_resolves_or_refuses_by_name` (ADR-0040's closed set, each
  form, both outcomes: the completeness guard D21-11 asks for),
  `a_prior_step_output_binds_by_step_name_and_field_path`,
  `an_optional_reference_is_null_and_a_mandatory_one_refuses_when_its_step_has_not_run`,
  `a_self_reference_in_a_loop_binds_the_previous_iteration_and_is_null_first`,
  `a_feedback_ref_binds_the_previous_loop_back_frame_and_is_null_first` (ADR-0042, over `tdd`'s `red` and
  `red-verify`, and over `minimal-repro`'s escalate-gate answer, B1),
  `a_coalesce_takes_its_left_operand_unless_it_is_null` (ADR-0034, B1),
  `a_repeat_until_over_a_later_step_runs_the_body_through_that_step_each_pass` (over `tdd`'s `green` and
  `green-verify`, M4),
  `for_each_fans_out_over_a_list_and_star_projects_its_outputs` (it also asserts the value of `[*]` over a
  list-valued field that R2 states, and that a bare fan-out `${…}` is never flattened, N13),
  `repeat_until_reruns_until_the_predicate_holds_and_takes_on_exhausted_at_max`,
  `a_gate_step_binds_answer_and_never_outputs`,
  `every_shipped_step_reference_is_in_the_resolver_grammar` (over `.redkiln/catalog`, asserting a non-zero count
  walked — D21-42 13a's rule),
  `null_out_refs_has_no_execution_path_caller` (a code-lines-only lint with a live control that a planted call is
  seen),
  `every_resolved_artifact_input_calls_the_read_seam_once_and_the_stub_emits_no_row` (N12: one seam call per resolved
  input, none for a `null` optional or a refusal, and no `artifact.read` row from this stage's stub, D21-47),
  `kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name` (N14; an
  undeclared key with `?` refuses too, m12),
  `an_escalate_re_entry_binds_a_later_steps_previous_frame_value_and_is_null_first` (D21-48 X8, over
  `minimal-repro-ue`'s `attempt` reading `${cook.outputs?}` and `${run.outputs?}`: `null` on the first pass, and the
  previous frame's values after `triage-gate` is answered and the step re-entered) [red: the escalate re-entry opens
  no frame, so both refs stay `null` on the resumed pass].
- `forward_ref_lint::` (D21-48 X8; §8d.3) —
  `a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name`
  (fixture catalogs under `crates/rk-kernel/tests/fixtures/forward_refs/`. One per licence loads. Three refuse at
  load by name, naming the activity, the step and the ref: an approval-gate answer read after the only loop, which is
  `adr-record`'s shape; an escalating step whose later gate carries another question; and a ref past the end of a
  repeat body) [red: the load has no licence check, so the three unlicensed fixtures load],
  `every_shipped_forward_ref_is_licensed_and_the_census_is_printed` (over `.redkiln/catalog`. It prints
  `FORWARD_REFS total=<n> letter=<a> repeat=<b> escalate=<c> unlicensed=<d>` once, from inside the test binary, and
  asserts `n > 0`, `n = a + b + c + d` and `d = 0`, over a non-zero count of activities walked — D21-42 13a's rule)
  [red: before the catalog edit `d` is 4],
  `the_four_unlicensed_refs_are_removed_versioned_and_changelogged` (it pins the four `(id, old, new)` triples of
  §8d.3's table, that each named input key is gone from its step, and one changelog row per triple) [red: the four
  inputs are present, the versions have not moved, and no row exists].
- `verb_registry::every_row_declares_side_effecting_and_the_writing_verbs_are_marked` (R1, M8).
- `check_eval::` — `a_check_binds_its_inputs_table_and_decides_true_false_and_refuses`,
  `an_unresolvable_check_input_refuses_and_is_never_read_as_false`,
  `a_red_build_report_fails_the_built_check_by_name` (N25).
- `emit_binding::` — `emit_binds_every_provided_kind_including_a_composing_table`,
  `a_step_after_emit_is_refused_by_name_and_never_silently_dropped`,
  `every_shipped_activity_has_one_last_emit_whose_artifacts_equal_provides`.
- `test_selection::` — `a_step_output_selection_resolves_and_reaches_the_port_as_named_tests`,
  `an_artifact_command_selection_resolves_through_selection_of`,
  `an_unresolvable_selection_refuses_and_never_runs_the_whole_suite`.
- `on_false_exec::` — `a_loop_back_re_enters_its_target_under_a_new_iteration_path`,
  `loop_back_exhaustion_refuses_as_step_failed_after_commit_and_committed_steps_stand`,
  `continue_proceeds_and_fail_refuses`, `a_false_check_with_no_on_false_refuses_as_step_failed_after_commit`,
  `a_deviate_on_false_refuses_by_name`.
- `step_error_row::` — `the_kernel_appends_exactly_one_error_row_per_failed_dispatch`,
  `the_fake_harness_writes_no_error_row_of_its_own`, `a_rate_limited_dispatch_yields_one_rate_limited_row_not_two`,
  `the_error_row_carries_family_and_at_so_the_breaker_can_fold_it`.
- `breaker::the_executor_refuses_a_dispatch_whose_family_the_kernels_rows_opened`; and
  `cross_process::ac035_breaker_refusal_survives_a_real_process_death`, which must now pass over rows the **kernel**
  wrote.
- `lane_walk::the_fix_lane_walk_table_holds_for_a_rust_text_bug`,
  `lane_walk::the_story_lane_walk_table_holds_for_a_spike_toml_story`.
- **Renamed, because this stage makes the old name false:** `guard_context::a_check_expr_reading_a_step_output_still_refuses_with_the_full_context`
  becomes `guard_context::a_check_expr_reading_a_step_output_resolves_it_and_refuses_only_an_unresolvable_one`;
  `walk_semantics::an_emit_step_ends_the_walk_and_a_later_tool_step_never_runs` is replaced by defect 1's test.
  `second_agent::a_second_runnable_agent_step_refuses_and_the_commit_survives` keeps its name — it is still true of a
  runnable agent step with no per-step spec — and gains the `committed()` / `skipped()` assertions it never had.

**`rk-cli`:** `transcription::every_selected_node_is_bound_including_one_with_no_agent_step` (defect 4),
`transcription::an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget`
(N16; moved from `rk-kernel`, because `bind_nodes` computes the bound, M10),
`transcription::each_agent_step_carries_its_own_card_and_schema_in_the_run_record`,
`processes::the_lanes_nine_gate_questions_and_the_change_question_load_from_redkiln_processes`,
`processes::the_answer_verb_validates_against_the_loaded_registry_not_the_default`,
`processes::a_gate_kind_the_registry_does_not_declare_refuses_by_name`,
`verb_rows::shallow_clone_routes_to_the_retained_review_clone`, `verb_rows::verdict_record_routes_to_review_confirm`,
`change_deriver::build_item_derives_change_over_the_recorded_range`,
`change_deriver::a_record_supplied_value_for_a_derived_facet_is_refused`,
`change_deriver::an_empty_range_abstains_and_never_defaults`,
`change_deriver::a_null_change_at_triage_opens_the_unknown_facet_gate_and_its_answer_binds_the_facet` (N3),
`explain::explain_prints_one_output_digest_per_committed_step_activation`.

**New test names: 59.** 46 are in `rk-kernel`, one of them replacing defect 1's old test, which is removed. 13 are
in `rk-cli`. That is **58 net** of the removal. The `guard_context` rename moves nothing. The 2026-09-24 revision added
six of the drafted 53: five in `rk-kernel` and one in `rk-cli`. *The D21-47 fix round (2026-09-24)* added six more to
`rk-kernel`: the feedback ref, the coalesce, the repeat body, the completion row, the side-effecting step and the
registry column. It also moved the timeout test from `rk-kernel` to `rk-cli`. Hence 41 − 1 + 6 = 46 and 12 + 1 = 13.
*D21-48 (2026-09-24):* X8 adds four to `rk-kernel`: the three `forward_ref_lint` tests and the escalate re-entry's
resolver test. **New test names are now 63**: 50 in `rk-kernel` (46 + 4) and 13 in `rk-cli`, which is **62 net** of
the removal. Checkpoint 8d (11)'s floor rises by the four.

**Mutants** — each authored, run from **Python** in a throwaway export that is checked out and then overlaid with
the working tree's bytes (a clone under `core.autocrlf=true` is not byte-faithful), each diffed and read, and each
must turn the named test red for the stated reason. A survivor blocks the close.

| id | mutation | must be killed by |
| --- | --- | --- |
| M1 | `is_committed` loses its step component (the commit key reverts to the activation) | `step_grain::re_entry_resumes_at_the_first_uncommitted_step_and_re_dispatches_no_committed_one`, `cross_process::kill_and_resume_n_step` |
| M2 | re-entry ignores served commits and re-dispatches from the first agent step | `cross_process::kill_and_resume_n_step` (dispatch count) |
| M3 | the iteration path is dropped from the commit key | `step_grain::two_iterations_of_one_step_never_share_a_commit_key` |
| M4 | the second-agent refusal restored to `ActivityReport::refused(now, cause)` | `step_grain::a_runnable_agent_step_with_no_per_step_spec_refuses_and_the_report_stays_committed` |
| M5 | `UnresolvedRef` mapped to `Tri::Null` (ADR-0136's near-miss) | `check_eval::an_unresolvable_check_input_refuses_and_is_never_read_as_false` |
| M6 | a self-reference binds the **current** iteration | `reference_resolver::a_self_reference_in_a_loop_binds_the_previous_iteration_and_is_null_first` |
| M7 | a loop-back ignores `max` | `on_false_exec::loop_back_exhaustion_refuses_as_step_failed_after_commit_and_committed_steps_stand` — bounded, and the bound proven to fire |
| M8 | exhaustion returns `Continue` | the same |
| M9 | `StepVerdict::Emitted => break` restored and the load refusal removed | `emit_binding::a_step_after_emit_is_refused_by_name_and_never_silently_dropped` |
| M10 | the kernel's error-row append deleted | `step_error_row::the_kernel_appends_exactly_one_error_row_per_failed_dispatch`, `breaker::the_executor_refuses_a_dispatch_whose_family_the_kernels_rows_opened` |
| M11 | the fake keeps its own append (two rows) | `step_error_row::the_fake_harness_writes_no_error_row_of_its_own` |
| M12 | `bind_nodes`' role-gated `continue` restored | `transcription::every_selected_node_is_bound_including_one_with_no_agent_step` |
| M13 | the step memo key takes the node spec's card digest | `step_grain::the_memo_key_of_an_agent_step_carries_that_steps_own_card_digest` |
| M14 | an unresolvable selection passed through as an empty selection | `test_selection::an_unresolvable_selection_refuses_and_never_runs_the_whole_suite` |
| M15 | a check's `Err` routed to the `on_false` arm | `check_eval::an_unresolvable_check_input_refuses_and_is_never_read_as_false` |
| M16 | the answer verb falls back to `QuestionRegistry::default()` | `processes::the_answer_verb_validates_against_the_loaded_registry_not_the_default` |
| M17 | `build_item` reads a record-supplied `change` | `change_deriver::a_record_supplied_value_for_a_derived_facet_is_refused` |
| M18 | `bind_nodes` takes each node's schema from `provides.first()` again | `transcription::each_agent_step_carries_its_own_card_and_schema_in_the_run_record` |
| M19 | the resolver's `artifact:` arm calls the read seam zero times or twice, or the stub appends an `artifact.read` row | `reference_resolver::every_resolved_artifact_input_calls_the_read_seam_once_and_the_stub_emits_no_row` |
| M20 | a per-step bound drops `min(…, budget.wall_clock_secs)` | `transcription::an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget` (`rk-cli`) |
| M21 | an undeclared `kb:` key resolves to empty content instead of refusing | `reference_resolver::kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name` |
| M22 | a `null` `change` at triage opens no gate, so the deriver's null is kept in silence | `change_deriver::a_null_change_at_triage_opens_the_unknown_facet_gate_and_its_answer_binds_the_facet` |
| M23 | `built` reads the `build` call's `is_ok()` instead of the report's status | `check_eval::a_red_build_report_fails_the_built_check_by_name` |
| M24 | a feedback ref binds the **current** frame's value of the later step, so it is `null` on every pass | `reference_resolver::a_feedback_ref_binds_the_previous_loop_back_frame_and_is_null_first` |
| M25 | `??` always takes its right operand | `reference_resolver::a_coalesce_takes_its_left_operand_unless_it_is_null` |
| M26 | the repeat body ends at the repeating step, so `until` is evaluated before `green-verify` has run | `reference_resolver::a_repeat_until_over_a_later_step_runs_the_body_through_that_step_each_pass` |
| M27 | the plan walk moves past a node on its first agent step's row | `step_grain::the_plan_walk_moves_past_a_node_only_once_its_emit_row_is_journalled` |
| M28 | a `side_effecting` tool step re-executes on re-entry | `step_grain::a_side_effecting_tool_step_commits_and_is_served_on_re_entry_never_re_executed` |
| M29 | an undeclared `toolchain:<feed>?` key resolves `null` instead of refusing | `reference_resolver::kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name` |
| M30 | a registry row with no `side_effecting` value loads as `false` | `verb_registry::every_row_declares_side_effecting_and_the_writing_verbs_are_marked` |
| M31 | the licence check drops the escalate arm, so only ADR-0042's letter and the repeat body license (D21-48 X8) | `forward_ref_lint::a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name` (the escalate fixture refuses), `forward_ref_lint::every_shipped_forward_ref_is_licensed_and_the_census_is_printed` (`unlicensed` becomes 10) |
| M32 | the escalate arm admits any later `gate` step, whatever its question | `forward_ref_lint::a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name` (the other-question fixture loads) |
| M33 | the licence check is skipped at load, so every `?` forward ref loads | `forward_ref_lint::a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name` (all three unlicensed fixtures load) |
| M34 | `.redkiln/catalog/vertical-slicing.toml`: `slice`'s `answer = "${approve.answer?}"` restored, version left at 3 | `forward_ref_lint::every_shipped_forward_ref_is_licensed_and_the_census_is_printed` (`unlicensed=1`), `forward_ref_lint::the_four_unlicensed_refs_are_removed_versioned_and_changelogged` |
| M35 | an escalate re-entry binds `null` for a later tool step's output on the resumed pass | `reference_resolver::an_escalate_re_entry_binds_a_later_steps_previous_frame_value_and_is_null_first` |

### 8d.11 What this stage makes provisional, and what it does not claim

**Checkpoints it touches.** Checkpoint 8c names two tests whose premise is the resolver's absence —
`tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port` (*"ADR-0148 stands"*) and
`tool_verb_routing::a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port`. Each keeps its name
only if its fixture is narrowed to a case that is still true (a selection or an input the resolver refuses by
name); otherwise it is renamed and Checkpoint 8c takes a dated amendment naming the new test. Either way Checkpoint
8c is re-run whole inside Checkpoint 8d **after** that edit. Checkpoint 7g's guards (7a) and (8) name three tests
this stage renames, replaces or narrows (the two in §8d.10 and the first `tool_verb_routing` test above); the
stage amends Checkpoint 7g by a dated line naming the new tests (ADR-0092's chain: the accepted record is the
authority, so the amendment is execution order) and re-runs it whole inside Checkpoint 8d. The amendment also
**re-lists the modules that call `run_activity_steps` by name** in 7g's (7a) filter and in its failure message,
*"these five modules are every test module that calls run_activity_steps by name"* (m15). The list is recounted off
disk at this stage's close with `grep -rln 'run_activity_steps' crates/*/tests`, never incremented. At `fc80687` that
grep names five `rk-kernel` modules (`second_agent`, `post_commit_failure`, `gate_step`, `check_cannot_dispatch` and
`walk_semantics`) and `rk-cli`'s `transcription`. Every new module of this stage that calls the function joins the
list. `rk-ports/src/fakes.rs`
changes, so **Checkpoint 7a** is made provisional and its row is marked per the legend's *"passed `<hash>`, re-run
pending"* rule until it passes. *Corrected 2026-09-24 (D21-47 review B2).* This paragraph said that 7a's re-run
was carried by the re-run of Checkpoint 8c, whose (15) re-runs 7a. It is not. Checkpoint 8d launches 8c with
`RK_CP_NESTED=1`, and 8c's (15) is skipped under it. So Checkpoint 8d (13) re-runs 7a whole itself. **Checkpoint 5**
re-runs when `crates/rk-types` differs from the commit this stage was entered at. R4's payload members, or R1's
`activity_committed` members, may land there. (13) decides with `git diff --quiet <entry-sha> -- crates/rk-types`. The
entry sha is the commit at which Checkpoint 8c passed. It is written into (0)'s `CP8D_ENTRY=` line as a literal by a
dated edit at stage open, and no re-runner supplies it (D21-47; the *final consistency pass 2026-09-24* replaced the
`RK_CP8D_ENTRY_SHA` input). **Checkpoint 6 does not
re-run.** N15 keeps `IdempotencyKey` three parts, and
`step_grain::the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts` pins that. s8's criterion (3)
asserted half is re-run inside Checkpoint 8d. *D21-48 (2026-09-24):* **Checkpoint 8b** is amended and re-run whole.
Its (3) and `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` pin `vertical-slicing` at version 2, and
X8's catalog fix moves it to 3 (§8d.3). The amendment lands in the catalog commit, and until the re-run passes, 8b's
row is marked *"passed `<hash>`, re-run pending"*.

**Nested re-runs cost hours, so Checkpoint 8d re-runs only these.** It re-runs its entry gate (8c), the older checkpoint its
renames make provisional (7g), and the one its `fakes.rs` edit makes provisional (7a). *D21-48 (2026-09-24):* it
also re-runs Checkpoint 8b, whose (3) the `vertical-slicing` bump amends (X8). It re-runs Checkpoint 5 only
when `crates/rk-types` moved. It also re-runs s8's criterion (3) asserted half, which this stage re-proves. It does not
re-run 8a or any other checkpoint (*D21-48 (2026-09-24):* 8b left this list). What 8c's own edits made provisional is 8c's block's business. **D21-46's one
re-run convention applies**: each is launched with `RK_CP_NESTED=1`, so 8c skips its own whole re-runs of 8b, 7a and 5.
That is why 7a is re-run here, and why 8b's re-run here is launched by this block itself. When this block is itself run with `RK_CP_NESTED=1`, as `s8d-ii`'s entry gate, it
re-runs none of them. The criterion (3) half is a named selection, not a whole checkpoint, and it runs either way.

**No re-run here can reach a real dispatch.** Checkpoint 7g's block runs `rk-cli`'s process-level tests, and those
spawn `redkiln run`. At this stage's time the shipped cards carry no `permission_mode`: 0 of the 8 files under
`.redkiln/cards/` at `a6349b6`. `lint_card` refuses a blank one with `ConfigError::PermissionModeAbsent`, so the
Claude adapter refuses every shipped card before it builds a command. Checkpoint 8d asserts that precondition before
its re-runs, inside (13)'s non-nested branch (§8d.12 (12a); M3). D21-47 puts every card or permission precondition
there, so that `s8j`'s nested replay of this block does not trip on `s8h`'s modes. (12a) also asserts that every
`rk-cli` fixture config naming a `harness_binary` names a fixture child. In `pack-min` that is
`rk-fixture-child-claude`, the canonical fixture harness (D21-47). That is what keeps the process tests off a real
`claude` once modes exist. **`s8h` amends (12a)** by dated line in the commit that gives the cards their modes, and it
lists 8d as provisional (D21-47's time-bound-guard rule). The card-mode half then becomes the assertion that the
fixture-child half still holds. Every replay and every process-level cargo run in the block also runs under the D21-47
wrapper, `runbook/tools/nodispatch.sh` (§8d.12 (0a)): a compiled fake `claude` first on `PATH` that prints the pin,
every other `PATH` directory holding a `claude` removed, `REDKILN_LAUNCH_RECORD` unset, and `CLAUDE_CONFIG_DIR` at an
empty trap. `nd_check` reads the trap empty and the fake uncalled after each cargo group and each replay, and at (15).
**Residual, named:** the fake is compiled, so a Rust `Command::new("claude")` on Windows resolves it. *(Final
consistency pass 2026-09-24: this replaces the extensionless-script residual.)* A process that builds its own `PATH`,
or names a real binary by its absolute path, passes it by, and for such a spawn the trap is the proof.

**D21-44 and D21-45 touch nothing here.** No run-worktree path appears in this file. The loader reads
`.redkiln/processes/*.toml`, and the catalog tests read `.redkiln/catalog/*.toml`. Neither recurses over `.redkiln/`
as a whole, so D21-44's `worktrees/` exclusion has no site in this stage. The worktree path-set guards in Checkpoint
8d compare `git worktree list` before and after, wherever the root is.

**What this stage does not claim.** No shipped activity completes against a real harness, and nothing here dispatches
to a real account (D21-40 decision 2). Cards, schemas and models are `s8h`'s; the tier-1 writer and the lane across
processes are `s8e`'s; rendering and landing are `s8f`'s; the verify record and the restored producers are
`s8g`'s. The memo is still consulted by nothing. Criterion (1) is not satisfied, and §8d.9 says by owner why.

### 8d.12 Checkpoint 8d

- **Each agent step dispatches under its own spec and commits under its own key**, as one `activity_committed` row
  carrying the step and the iteration path; a runnable agent step with no per-step spec refuses by name **and the
  report stays committed** (defect 2). Each per-step spec carries its bound, never above the activity budget, and the
  idempotency key keeps three parts. The plan walk moves past a node only on its emit row, and a side-effecting tool
  step commits and is served on re-entry.
- **Re-entry resumes at the first uncommitted step** and re-dispatches no committed one, across a real process
  death — `kill_and_resume_n_step` at step grain, and criterion (3)'s asserted half re-run from s8's own bytes.
- **The reference resolver is complete**: every ADR-0040 form resolves or refuses by name; the shipped catalog's
  references are all in its grammar; `null_out_refs` has no execution-path caller. `kb:`, `checklist:` and feed
  keys resolve through `s8c`'s tables, and an undeclared key refuses even with `?`. Feedback refs bind the previous
  frame, `??` coalesces, and a `repeat.until` over a later step runs its body. Every resolved `artifact:` input makes
  one read-seam call, and this stage emits no `artifact.read` itself. `selection_of` has one body.
- **Every shipped forward ref is licensed** (*D21-48 (2026-09-24)*, X8): by ADR-0042's letter, by a repeat body or
  by an escalate re-entry. Zero are unlicensed, over a non-zero census read from inside the test binary. The four
  unlicensed refs are removed under ADR-0072, versioned and changelogged, and the lattice re-runs.
- **`check`, `emit` and test selection** decide from resolved values; an unresolvable input is never read as false;
  a step after `emit` is refused, not dropped.
- **`on_false` iterates**, bounded, and exhaustion refuses as `StepFailedAfterCommit` with committed steps standing.
- **One error row per failed dispatch**, written by the kernel under either harness, carrying what the breaker folds;
  the breaker refuses from the store.
- **The two verb rows route**, the nine lane questions and N3's `facet.change` load, `answer` validates against
  them, a `null` `change` at triage opens AC-079's gate and its answer binds the facet, and the `change` deriver is
  the only source of `change`.
- **The walk table holds** — both lanes, exactly, above a walked floor of 11 and 9.
- **Nothing regressed**: the executor guards `s7g` built stay green. Checkpoints 8c, 8b, 7g and 7a re-run whole
  (8b because this stage amends its (3); *D21-48 (2026-09-24)*), and 5 re-runs when `crates/rk-types` moved. Each runs as `nodispatch env RK_CP_NESTED=1 bash …` under the shared D21-47
  wrapper (`runbook/tools/nodispatch.sh`: a compiled fake `claude` printing the pin, `REDKILN_LAUNCH_RECORD` unset,
  `CLAUDE_CONFIG_DIR` at an empty trap), and none runs when this block is under `RK_CP_NESTED=1`. The whole suite passes above its floor, and `fmt` and `clippy` are clean. No re-run can
  reach a real dispatch.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Conventions, as Checkpoint 7i's: every cargo reading is `out=$(…); rc=$?`, never a pipe's status (under build-lock
# contention cargo exits 0 having run ZERO tests); every named selection is `--test it`, MODULE-QUALIFIED and
# `--exact`, and asserts each test's own `... ok` line AND an exact count; the whole suite runs `--no-fail-fast`
# above a passed-count FLOOR; every grep over source is a floor over CODE lines beneath a behavioural test, never the
# assertion; and every guard that could only ever go green is given a synthetic control that proves it can go red.

# ---- (0) THE WORKTREE PATH SET, read FIRST, and a precondition that a prune would change nothing: Checkpoint 7g,
# ---- re-run below, prunes, and a stale registration present now would otherwise read as this block's leak.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
stale0=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale0" ] || { echo "FAIL (PRECONDITION): stale worktree registrations exist before this block — prune them and record it first:"; printf '%s\n' "$stale0"; exit 1; }
tmpd=$(mktemp -d)
test -d "$tmpd" || { echo "FAIL: mktemp -d made no directory"; exit 1; }
# The commit at which Checkpoint 8c passed, written into the next line as a LITERAL by a dated edit at stage open
# (D21-47: a block carries its own record ids and shas, and no re-runner supplies one). Empty means not yet written.
CP8D_ENTRY=
printf '%s\n' "$CP8D_ENTRY" | grep -qxE '[0-9a-f]{40}' || { echo "FAIL: CP8D_ENTRY (the commit at which Checkpoint 8c passed) is not recorded in this block — write it by dated edit at stage open (§8d.11)"; exit 1; }

# ---- (0a) THE NO-DISPATCH WRAPPER (D21-47), runbook/tools/nodispatch.sh, set ONCE for every cargo run and every
# ---- replay below: a COMPILED fake `claude` first on PATH that prints the pin (a Rust Command::new("claude") on
# ---- Windows resolves only claude.exe, so a script fake would be passed by), every other PATH directory holding a
# ---- claude removed, REDKILN_LAUNCH_RECORD unset, and CLAUDE_CONFIG_DIR at an empty trap. In this session the
# ---- inherited CLAUDE_CONFIG_DIR is the operator's real account. nd_setup runs the controls, and reuses an enclosing
# ---- wrapper when this block is itself a replay (ND_ACTIVE=1). nd_check reads the trap empty and the fake's call log
# ---- absent after each cargo group and each replay, and at (15).
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh (run from the repo root)"; exit 1; }
nd_setup "$PWD/target/cp8d/nd" || exit 1
nd_enter || exit 1

# ---- (1) BUILD-OR-FAIL. `target/` is gitignored; the rk-cli selections spawn the binary.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8d NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists — every spawning test below would be about nothing"; exit 1; }
nd_check "(1) build" || exit 1

# ---- (2) THE SELECTION RUNNER. `want` is the NUMBER OF NAMES: under `--exact` a name selects at most one test, so
# ---- an exact count equal to the names given, plus each name's own `... ok`, means every name ran and passed.
run_named() {
  pkg=$1; label=$2; shift 2
  want=$#
  [ "$want" -gt 0 ] || { echo "FAIL: selection '$label' names no test"; exit 1; }
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: selection '$label' rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: '$label': $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: '$label' did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
  echo "ok: $label ($want)"
}
# SYNTHETIC CONTROL: the exact-count form must reject a count that is not the one named.
if printf '%s\n' 'test result: ok. 1 passed; 0 failed; 0 ignored' | grep -qE "test result: ok\. 2 passed; 0 failed"; then echo "FAIL: synthetic control — the exact-count grep admits 1 for 2"; exit 1; fi
if printf '%s\n' 'test result: ok. 12 passed; 0 failed; 0 ignored' | grep -qE "test result: ok\. 2 passed; 0 failed"; then echo "FAIL: synthetic control — the exact-count grep admits 12 for 2"; exit 1; fi

# ---- (3) BULLETS 1-2: STEP GRAIN, RE-ENTRY ACROSS A REAL KILL, AND DEFECT 2.
run_named rk-kernel "step grain" \
  step_grain::each_agent_step_dispatches_under_its_own_spec_and_commits_under_its_own_key \
  step_grain::a_runnable_agent_step_with_no_per_step_spec_refuses_and_the_report_stays_committed \
  step_grain::re_entry_resumes_at_the_first_uncommitted_step_and_re_dispatches_no_committed_one \
  step_grain::two_iterations_of_one_step_never_share_a_commit_key \
  step_grain::the_memo_key_of_an_agent_step_carries_that_steps_own_card_digest \
  step_grain::an_activation_with_no_agent_step_walks_to_its_emit_and_completes \
  step_grain::the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts \
  step_grain::the_plan_walk_moves_past_a_node_only_once_its_emit_row_is_journalled \
  step_grain::a_side_effecting_tool_step_commits_and_is_served_on_re_entry_never_re_executed \
  verb_registry::every_row_declares_side_effecting_and_the_writing_verbs_are_marked \
  cross_process::kill_and_resume_n_step \
  cross_process::kill_and_resume_n_activation \
  second_agent::a_second_runnable_agent_step_refuses_and_the_commit_survives \
  second_agent::a_guarded_off_second_agent_step_refuses_nothing

# ---- (4) BULLETS 3-4: THE RESOLVER, CHECK, EMIT AND TEST SELECTION.
run_named rk-kernel "resolver" \
  reference_resolver::every_input_source_form_resolves_or_refuses_by_name \
  reference_resolver::a_prior_step_output_binds_by_step_name_and_field_path \
  reference_resolver::an_optional_reference_is_null_and_a_mandatory_one_refuses_when_its_step_has_not_run \
  reference_resolver::a_self_reference_in_a_loop_binds_the_previous_iteration_and_is_null_first \
  reference_resolver::a_feedback_ref_binds_the_previous_loop_back_frame_and_is_null_first \
  reference_resolver::a_coalesce_takes_its_left_operand_unless_it_is_null \
  reference_resolver::a_repeat_until_over_a_later_step_runs_the_body_through_that_step_each_pass \
  reference_resolver::for_each_fans_out_over_a_list_and_star_projects_its_outputs \
  reference_resolver::repeat_until_reruns_until_the_predicate_holds_and_takes_on_exhausted_at_max \
  reference_resolver::a_gate_step_binds_answer_and_never_outputs \
  reference_resolver::every_shipped_step_reference_is_in_the_resolver_grammar \
  reference_resolver::null_out_refs_has_no_execution_path_caller \
  reference_resolver::every_resolved_artifact_input_calls_the_read_seam_once_and_the_stub_emits_no_row \
  reference_resolver::kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name \
  check_eval::a_check_binds_its_inputs_table_and_decides_true_false_and_refuses \
  check_eval::an_unresolvable_check_input_refuses_and_is_never_read_as_false \
  check_eval::a_red_build_report_fails_the_built_check_by_name \
  emit_binding::emit_binds_every_provided_kind_including_a_composing_table \
  emit_binding::a_step_after_emit_is_refused_by_name_and_never_silently_dropped \
  emit_binding::every_shipped_activity_has_one_last_emit_whose_artifacts_equal_provides \
  test_selection::a_step_output_selection_resolves_and_reaches_the_port_as_named_tests \
  test_selection::an_artifact_command_selection_resolves_through_selection_of \
  test_selection::an_unresolvable_selection_refuses_and_never_runs_the_whole_suite \
  guard_context::a_rigor_guard_refuses_on_an_empty_context_and_decides_on_the_selections_one \
  guard_context::a_check_expr_reading_a_step_output_resolves_it_and_refuses_only_an_unresolvable_one

# ---- (4b) THE FORWARD-REF LICENCE (D21-48 X8; §8d.3): the widened lint, the census, the four catalog removals under
# ---- ADR-0072 and the escalate re-entry's binding. Beside them are the pre-existing tests the edit must keep green:
# ---- the shipped pack loads, the lattice re-runs over .redkiln/ (ADR-0072's reachability test), and s8b's triple
# ---- test, amended in this stage's catalog commit.
run_named rk-kernel "forward-ref licence" \
  forward_ref_lint::a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name \
  forward_ref_lint::every_shipped_forward_ref_is_licensed_and_the_census_is_printed \
  forward_ref_lint::the_four_unlicensed_refs_are_removed_versioned_and_changelogged \
  reference_resolver::an_escalate_re_entry_binds_a_later_steps_previous_frame_value_and_is_null_first \
  quality_block::adr0072_d2_edits_are_versioned_and_changelogged \
  lattice::p6_lattice_over_the_shipped_pack \
  conformance::shipped_pack_loads

# ---- (5) BULLETS 5-6: ON_FALSE AS ITERATION, THE ERROR ROW AND THE BREAKER.
run_named rk-kernel "on_false, telemetry, breaker" \
  on_false_exec::a_loop_back_re_enters_its_target_under_a_new_iteration_path \
  on_false_exec::loop_back_exhaustion_refuses_as_step_failed_after_commit_and_committed_steps_stand \
  on_false_exec::continue_proceeds_and_fail_refuses \
  on_false_exec::a_false_check_with_no_on_false_refuses_as_step_failed_after_commit \
  on_false_exec::a_deviate_on_false_refuses_by_name \
  step_error_row::the_kernel_appends_exactly_one_error_row_per_failed_dispatch \
  step_error_row::the_fake_harness_writes_no_error_row_of_its_own \
  step_error_row::a_rate_limited_dispatch_yields_one_rate_limited_row_not_two \
  step_error_row::the_error_row_carries_family_and_at_so_the_breaker_can_fold_it \
  breaker::the_executor_refuses_a_dispatch_whose_family_the_kernels_rows_opened \
  breaker::breaker_ac033_ac035 \
  breaker::ac034_breaker_never_trips_globally \
  cross_process::ac035_breaker_refusal_survives_a_real_process_death

# ---- (6) BULLET 8: THE WALK TABLE, both lanes, exactly (§8d.9). A test that measures another row corrects the
# ---- TABLE by a dated line; it is never bent to the table.
run_named rk-kernel "walk table" \
  lane_walk::the_fix_lane_walk_table_holds_for_a_rust_text_bug \
  lane_walk::the_story_lane_walk_table_holds_for_a_spike_toml_story

# ---- (6a) THE WALKED FLOOR (M9). Each lane_walk test prints `LANE_WALK <lane> walked=<n> refused=<m>` once. `n` must
# ---- be at least this stage's claim: 11 (fix) and 9 (story). A later stage's flip to "walks" raises n and needs no
# ---- edit here; LOWERING a floor needs the operator's dated ledger line first. `grep -o` reads the line even where
# ---- libtest glues it to the `test ... ` prefix.
walked_of() { printf '%s\n' "$1" | grep -oE "LANE_WALK $2 walked=[0-9]+ refused=[0-9]+" | sed -nE 's/^.* walked=([0-9]+) .*$/\1/p'; }
[ "$(walked_of 'test lane_walk::x ... LANE_WALK fix walked=7 refused=8' fix)" = 7 ] || { echo "FAIL: synthetic control — walked_of does not read a glued LANE_WALK line"; exit 1; }
[ -z "$(walked_of 'LANE_WALK story walked=7 refused=8' fix)" ] || { echo "FAIL: synthetic control — walked_of reads another lane's line"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- lane_walk::the_fix_lane_walk_table_holds_for_a_rust_text_bug lane_walk::the_story_lane_walk_table_holds_for_a_spike_toml_story --exact --nocapture --test-threads=1 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the walk-table tests, re-run to read their LANE_WALK lines, rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
for pair in fix:11 story:9; do
  lane=${pair%%:*}; floor=${pair##*:}
  k=$(printf '%s\n' "$out" | grep -oE "LANE_WALK $lane walked=[0-9]+ refused=[0-9]+" | grep -c .)
  [ "$k" -eq 1 ] || { echo "FAIL: the $lane lane printed $k LANE_WALK lines, not exactly one"; exit 1; }
  w=$(walked_of "$out" "$lane")
  [ "$w" -ge "$floor" ] || { echo "FAIL: the $lane lane walked $w activities, below this stage's floor $floor — correct the table by a dated line and take the operator's ledger line before lowering the floor"; exit 1; }
  echo "ok: $lane lane walked $w (floor $floor)"
done

# ---- (6b) ZERO UNLICENSED FORWARD REFS (D21-48 X8), read from INSIDE the test binary, because a Python census is a
# ---- proxy. The census test prints `FORWARD_REFS total=<n> letter=<a> repeat=<b> escalate=<c> unlicensed=<d>` once.
# ---- n must be non-zero (a census over nothing is Halt 8), the parts must sum to n, d must be 0, and c must be
# ---- non-zero, since the lanes' own minimal-repro and tdd refs ride the escalate licence.
FR_RE='FORWARD_REFS total=[0-9]+ letter=[0-9]+ repeat=[0-9]+ escalate=[0-9]+ unlicensed=[0-9]+'
fr_field() { printf '%s\n' "$1" | grep -oE "$FR_RE" | sed -nE "s/^.* $2=([0-9]+).*\$/\1/p"; }
fr_ok() {
  k=$(printf '%s\n' "$1" | grep -oE "$FR_RE" | grep -c .)
  [ "$k" -eq 1 ] || return 1
  t=$(fr_field "$1" total); a=$(fr_field "$1" letter); b=$(fr_field "$1" repeat); c=$(fr_field "$1" escalate); d=$(fr_field "$1" unlicensed)
  [ -n "$t" ] && [ -n "$d" ] && [ "$t" -gt 0 ] && [ "$t" -eq $((a + b + c + d)) ] && [ "$d" -eq 0 ] && [ "$c" -gt 0 ]
}
# SYNTHETIC CONTROLS: a glued, licensed census passes; four unlicensed, a census over nothing, parts that do not sum,
# no escalate licence, and two census lines each fail.
fr_ok 'test forward_ref_lint::x ... FORWARD_REFS total=77 letter=64 repeat=3 escalate=10 unlicensed=0' || { echo "FAIL: synthetic control — fr_ok refuses a glued, licensed census line"; exit 1; }
if fr_ok 'FORWARD_REFS total=81 letter=64 repeat=3 escalate=10 unlicensed=4'; then echo "FAIL: synthetic control — fr_ok admits a census with four unlicensed refs"; exit 1; fi
if fr_ok 'FORWARD_REFS total=0 letter=0 repeat=0 escalate=0 unlicensed=0'; then echo "FAIL: synthetic control — fr_ok admits a census over nothing"; exit 1; fi
if fr_ok 'FORWARD_REFS total=80 letter=64 repeat=3 escalate=10 unlicensed=0'; then echo "FAIL: synthetic control — fr_ok admits a census whose parts do not sum"; exit 1; fi
if fr_ok 'FORWARD_REFS total=67 letter=64 repeat=3 escalate=0 unlicensed=0'; then echo "FAIL: synthetic control — fr_ok admits a census with no escalate licence"; exit 1; fi
if fr_ok "$(printf 'FORWARD_REFS total=77 letter=64 repeat=3 escalate=10 unlicensed=0\nFORWARD_REFS total=77 letter=64 repeat=3 escalate=10 unlicensed=0')"; then echo "FAIL: synthetic control — fr_ok admits two census lines"; exit 1; fi
out=$(cargo test -p rk-kernel --test it -- forward_ref_lint::every_shipped_forward_ref_is_licensed_and_the_census_is_printed --exact --nocapture --test-threads=1 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the forward-ref census test, re-run to read its FORWARD_REFS line, rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
fr_ok "$out" || { echo "FAIL: the census did not print exactly one FORWARD_REFS line with total > 0, parts summing to total, escalate > 0 and unlicensed=0 (D21-48 X8):"; printf '%s\n' "$out" | grep -F 'FORWARD_REFS'; exit 1; }
echo "ok: forward refs $(fr_field "$out" total), unlicensed 0 (escalate-licensed $(fr_field "$out" escalate))"

# ---- (6c) THE FOUR REMOVALS ON DISK (D21-48 X8; §8d.3), a floor beneath (4b)'s tests: each removed reference is on no
# ---- non-comment line of its activity, each activity is at its new version, and the changelog names it. This is a
# ---- time-bound guard: a later stage that bumps one of these four amends this line in its own commit (D21-47).
cf_hit() { grep -nF "$2" "$1" | grep -vE '^[0-9]+:[[:space:]]*#' | grep .; }
printf 'inputs = { answer = "${approve.answer?}" }\n# answer = "${approve.answer?}"\n' > "$tmpd/fr.toml"
[ "$(cf_hit "$tmpd/fr.toml" '${approve.answer?}' | grep -c .)" -eq 1 ] || { echo "FAIL: synthetic control — the removal floor does not see exactly the one planted code line"; exit 1; }
test -f .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md is absent — s8b's ADR-0072 changelog (Halt 8)"; exit 1; }
for row in 'adr-record|2|${accept.answer?}' 'spike-question-spec|2|${accept.answer?}' 'structured-nl-requirements|3|${conflicts.answer?}' 'vertical-slicing|3|${approve.answer?}'; do
  id=${row%%|*}; rest=${row#*|}; v=${rest%%|*}; ref=${rest#*|}; f=".redkiln/catalog/$id.toml"
  test -f "$f" || { echo "FAIL: $f is absent (Halt 8)"; exit 1; }
  [ -z "$(cf_hit "$f" "$ref")" ] || { echo "FAIL: $id still reads $ref on a code line — an unlicensed forward ref (D21-48 X8)"; exit 1; }
  grep -qE "^version[[:space:]]*=[[:space:]]*${v}([[:space:]]|$)" "$f" || { echo "FAIL: $id is not at version $v — a coherence edit bumps version (ADR-0072)"; exit 1; }
  grep -qF "$id@$v" .redkiln/CHANGELOG.md || { echo "FAIL: no changelog row names $id@$v (ADR-0072)"; exit 1; }
done
echo "ok: the four unlicensed forward refs are removed, versioned and changelogged"
nd_check "(3)-(6c) kernel selections" || exit 1

# ---- (7) BULLET 9, KERNEL HALF: the executor guards `s7g` built stay green, and criterion (2) with them.
run_named rk-kernel "s7g executor guards" \
  post_commit_failure::a_step_failing_after_a_committed_dispatch_refuses_by_name_and_keeps_the_commit \
  gate_step::a_gate_step_after_a_dispatch_writes_the_runtime_mark_and_keeps_the_commit \
  gate_step::a_gate_step_before_any_dispatch_opens_the_gate_and_does_not_refuse \
  gate_step::a_guarded_off_gate_step_opens_nothing \
  gate_step::a_gate_step_without_a_question_is_refused_by_name \
  gate_step::an_answered_gate_step_is_walked_past_and_not_re_opened \
  gate_step::a_voided_gate_step_re_opens_at_the_next_reach \
  gate_step::the_walk_continues_past_a_skipped_step_to_the_gate_behind_it \
  check_cannot_dispatch::a_check_step_never_reaches_the_toolchain_port \
  check_cannot_dispatch::a_walk_of_check_steps_moves_the_toolchain_counter_exactly_once_and_that_once_is_the_tool_step \
  walk_semantics::an_empty_transcribed_step_list_is_refused_by_name_and_none_is_not_the_empty_case \
  projection::rebuild_differential

# ---- (8) BULLET 7 AND DEFECT 4: THE COMPOSITION ROOT — binding, questions, verb rows, the deriver, explain.
run_named rk-cli "composition root" \
  transcription::the_production_path_transcribes_a_step_list_and_never_supplies_none \
  transcription::the_spec_a_real_run_binds_carries_its_steps \
  transcription::every_selected_node_is_bound_including_one_with_no_agent_step \
  transcription::an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget \
  transcription::each_agent_step_carries_its_own_card_and_schema_in_the_run_record \
  processes::the_lanes_nine_gate_questions_and_the_change_question_load_from_redkiln_processes \
  processes::the_answer_verb_validates_against_the_loaded_registry_not_the_default \
  processes::a_gate_kind_the_registry_does_not_declare_refuses_by_name \
  verb_rows::shallow_clone_routes_to_the_retained_review_clone \
  verb_rows::verdict_record_routes_to_review_confirm \
  change_deriver::build_item_derives_change_over_the_recorded_range \
  change_deriver::a_record_supplied_value_for_a_derived_facet_is_refused \
  change_deriver::an_empty_range_abstains_and_never_defaults \
  change_deriver::a_null_change_at_triage_opens_the_unknown_facet_gate_and_its_answer_binds_the_facet \
  explain::explain_prints_one_output_digest_per_committed_step_activation \
  exit::adr0144_post_commit_step_failure_exits_the_class_row_not_three
nd_check "(7)-(8) selections" || exit 1

# ---- (9) THE QUESTION REGISTRY'S CONTENT — a floor beneath (8)'s loader test: each of §8d.6's ten ids (nine lane
# ---- questions and N3's facet.change) declared exactly once across `.redkiln/processes/`. spec.ambiguity is not
# ---- among them (N2). If R5 fixes another file home, this line takes a dated amendment.
ls .redkiln/processes/*.toml > "$tmpd/qfiles" 2>&1; rc=$?
[ "$rc" -eq 0 ] && [ -s "$tmpd/qfiles" ] || { echo "FAIL: .redkiln/processes/ holds no .toml — the lanes' gate questions have no registry (ADR-0156 blocker 2)"; exit 1; }
for q in closure.signoff confirmation.not-fixed draft.accept facet.change repro.cannot-reproduce review.blocking root-cause.accept root-cause.no-hypothesis scan.accept-findings tdd.green-exhausted; do
  n=$(grep -hxF "[question.\"$q\"]" .redkiln/processes/*.toml | grep -c .)
  [ "$n" -eq 1 ] || { echo "FAIL: question $q is declared $n times under .redkiln/processes/, not once"; exit 1; }
done

# ---- (10) CODE-LINE FLOORS beneath the behavioural tests. `codeonly` drops `//`, `/*` and doc-continuation `* `
# ---- leaders after the `path:LINE:` prefix grep prints; a string literal is a CODE line and FAILS a floor, which is
# ---- the safe direction. It cannot see a renamed identifier or one assembled by a macro — said, not claimed away.
codeonly() { printf '%s\n' "$1" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .; }
ctl=$(codeonly "$(printf 'a.rs:1:    // null_out_refs(x)\na.rs:2:    let y = null_out_refs(x);')")
[ "$(printf '%s\n' "$ctl" | grep -c .)" -eq 1 ] || { echo "FAIL: synthetic control — codeonly did not keep exactly the one code line"; exit 1; }
printf '%s\n' "$ctl" | grep -qF 'let y = null_out_refs' || { echo "FAIL: synthetic control — codeonly dropped the code line instead of the comment"; exit 1; }
# The null_out_refs floor reads ALL of crates/rk-kernel/src except rigor.rs, whose static lint's null_out_refs is
# correct (m11), so a call from the resolver's home, runtime/reference.rs, or from anywhere else is seen. LIVE
# CONTROL: the same reading over rigor.rs must keep its code lines, or an empty result below proves nothing.
test -f crates/rk-kernel/src/runtime/reference.rs && test -f crates/rk-kernel/src/rigor.rs || { echo "FAIL: the resolver's home (runtime/reference.rs) or rigor.rs is not where this floor reads (Halt 8)"; exit 1; }
grep -qF 'AC-494' crates/rk-kernel/src/resolver.rs || { echo "FAIL: src/resolver.rs no longer reads as AC-494's capability resolver — the reference resolver's home is runtime/reference.rs (m11)"; exit 1; }
live=$(codeonly "$(grep -nF 'null_out_refs' crates/rk-kernel/src/rigor.rs | sed 's|^|crates/rk-kernel/src/rigor.rs:|')")
[ -n "$live" ] || { echo "FAIL: live control — the floor's reading keeps no null_out_refs code line even in rigor.rs"; exit 1; }
raw=$(grep -rnF --exclude=rigor.rs 'null_out_refs' crates/rk-kernel/src); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the null_out_refs floor could not run (grep rc=$rc)"; exit 1; }
hit=$(codeonly "$raw")
[ -z "$hit" ] || { echo "FAIL: null_out_refs on an execution-path CODE line (ADR-0136's forbids):"; printf '%s\n' "$hit"; exit 1; }
raw=$(grep -rnE 'Emitted => break|has no executor' crates/rk-kernel/src/runtime); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the executor floor could not run (grep rc=$rc)"; exit 1; }
hit=$(codeonly "$raw")
[ -z "$hit" ] || { echo "FAIL: the silent emit drop or the stale 'on_false has no executor' refusal is still on a CODE line (defect 1; 4d):"; printf '%s\n' "$hit"; exit 1; }
# ONE selection parser (§8d.3): exactly one CODE-line definition of `fn selection_of(` under crates/*/src. A
# `pub use` re-export is not a definition and is not counted. SYNTHETIC CONTROL first: a doc line and a code line
# must count as one.
ctl=$(codeonly "$(printf 'a.rs:1:/// fn selection_of(x)\nb.rs:2:pub fn selection_of(command: &str) -> Option<Vec<String>> {')")
[ "$(printf '%s\n' "$ctl" | grep -cE 'fn selection_of[[:space:]]*\(')" -eq 1 ] || { echo "FAIL: synthetic control — the selection_of floor did not count exactly the one code definition"; exit 1; }
raw=$(grep -rnE 'fn selection_of[[:space:]]*\(' crates/*/src); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the selection_of floor could not run (grep rc=$rc)"; exit 1; }
hit=$(codeonly "$raw")
n=$(printf '%s\n' "$hit" | grep -c 'fn selection_of')
[ "$n" -eq 1 ] || { echo "FAIL: $n CODE-line definitions of fn selection_of under crates/*/src, not exactly one — the kernel wrote a second parser, or the function is gone (§8d.3):"; printf '%s\n' "$hit"; exit 1; }

# ---- (11) BULLET 9: THE WHOLE SUITE, `--no-fail-fast`, rc 0, zero failed, above a FLOOR. Expected value, re-derived
# ---- at stage open from Checkpoint 8c's recorded count: 852 + 62 = 914 (852 = 740 at `992d257`, ADR-0164's evidence,
# ---- + s8a 9 + s8b 25 + s8c 78; 62 is §8d.10's net new tests after D21-48 X8's four). D21-48 (2026-09-24): this
# ---- supersedes 843 + 62 = 905 (s8c was 69 before D21-48 X15's nine), which superseded 843 + 58 = 901, and the
# ---- final consistency pass's figure superseded the 833 + 58 = 891 before it.
# ---- 852 is re-read from Checkpoint 8c's recorded count at open, not assumed. The
# ---- derivation is recorded. If 8c recorded more, the floor rises with it. It is never lowered to make a red run
# ---- green. At least 44 result lines, as at 992d257.
FLOOR=914
tally() { printf '%s\n' "$1" | awk -v k="$2" '/^test result: /{for(i=1;i<NF;i++) if($(i+1)==k";" || $(i+1)==k) s+=$i} END{print s+0}'; }
[ "$(tally 'test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out' passed)" -eq 3 ] || { echo "FAIL: synthetic control — tally does not read a passed count"; exit 1; }
[ "$(tally 'test result: FAILED. 3 passed; 2 failed; 0 ignored' failed)" -eq 2 ] || { echo "FAIL: synthetic control — tally does not read a failed count"; exit 1; }
[ "$(tally 'running 0 tests' passed)" -lt "$FLOOR" ] || { echo "FAIL: synthetic control — the floor admits a run of zero tests"; exit 1; }
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the whole suite rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked at' | head -20; exit 1; }
passed=$(tally "$out" passed); failed=$(tally "$out" failed)
results=$(printf '%s\n' "$out" | grep -c '^test result: ')
[ "$failed" -eq 0 ] || { echo "FAIL: $failed test(s) failed across the suite"; exit 1; }
[ "$passed" -ge "$FLOOR" ] || { echo "FAIL: the whole suite passed $passed, below the floor $FLOOR — a test was lost"; exit 1; }
[ "$results" -ge 44 ] || { echo "FAIL: only $results result lines (44 at 992d257) — a test target did not run"; exit 1; }
echo "ok: whole suite $passed passed, 0 failed, over $results results"
nd_check "(11) whole suite" || exit 1

# ---- (12) fmt and clippy, gated on exit codes (never `-D warnings` on the command line — ADR-0083; PIA-030).
out=$(cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }
nd_check "(12) fmt and clippy" || exit 1

# ---- (12a) is now INSIDE (13)'s non-nested branch (D21-47: every card or permission precondition sits there, so
# ---- s8j's nested replay of this block does not trip on s8h's modes). s8h amends it by dated line (§8d.11).

# ---- (13) THE ENTRY CHECKPOINT, 8b, THE EXECUTOR CHECKPOINT AND 7a, RE-RUN WHOLE FROM THEIR OWN BYTES, and 5 when
# ---- crates/rk-types moved (§8d.11: nested re-runs cost hours, so only these). 8c is the entry gate; 8b is amended by
# ---- this stage's vertical-slicing bump (D21-48 X8, 2026-09-24); 7g is provisional on this stage's renames; 7a on its
# ---- fakes.rs edit. 7a does NOT ride inside 8c's block here: 8c runs with
# ---- RK_CP_NESTED=1 and skips its own (15) (D21-47 review B2). The fence string is built from octal escapes so this
# ---- block carries no literal fence. D21-46: SKIPPED under RK_CP_NESTED=1 (this block is s8d-ii's entry gate);
# ---- otherwise each runs as `nodispatch env RK_CP_NESTED=1 bash …` under (0a)'s wrapper, and nd_check follows it.
fence=$(printf '\140\140\140')
test -n "$fence" || { echo "FAIL: the fence string is empty — extract_block would match every line"; exit 1; }
extract_block() {
  awk -v f="$fence" -v m="$2" '$0 == f "bash" {inb=1; buf=""; next} inb && $0 == f {inb=0; if (index(buf, m)) {printf "%s", buf; n++}; next} inb {buf = buf $0 "\n"} END {exit (n == 1) ? 0 : 1}' "$1"
}
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8D (13) SKIPPED: RK_CP_NESTED=1 — this block is another checkpoint's entry gate; its re-runs of 8c, 8b, 7g, 7a and 5 ran at Checkpoint 8d's own close"
else
  # ---- (12a) PRECONDITION: NO RE-RUN BELOW CAN REACH A REAL DISPATCH (D21-43; §8d.11). 7g's re-run spawns
  # ---- `redkiln run` through rk-cli's process tests. Before s8h no shipped card carries `permission_mode`, and
  # ---- `lint_card` refuses a blank one. If a card carries one, s8h has landed and has not amended this line by its
  # ---- dated edit (D21-47's time-bound-guard rule): stop.
  ncards=0
  for c in .redkiln/cards/*.json; do [ -f "$c" ] && ncards=$((ncards + 1)); done
  [ "$ncards" -gt 0 ] || { echo "FAIL: no card files under .redkiln/cards — the precondition would hold over nothing (Halt 8)"; exit 1; }
  # SYNTHETIC CONTROL: the probe must see a planted mode.
  printf '{"permission_mode": "default"}\n' > "$tmpd/card.json"
  grep -qF '"permission_mode"' "$tmpd/card.json" || { echo "FAIL: synthetic control — the card-mode probe cannot see a planted permission_mode"; exit 1; }
  moded=$(grep -lF '"permission_mode"' .redkiln/cards/*.json); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: the card-mode precondition could not run (grep rc=$rc)"; exit 1; }
  [ -z "$moded" ] || { echo "FAIL (PRECONDITION): a shipped card carries permission_mode, so s8h has landed without its dated amendment of (12a):"; printf '%s\n' "$moded"; exit 1; }
  # (12a), second half: every rk-cli fixture config that names a harness_binary names a FIXTURE CHILD, and pack-min's
  # names the canonical one, rk-fixture-child-claude (D21-47). SYNTHETIC CONTROL first: a planted `claude` is seen.
  hb_bad() { grep -rhE '^[[:space:]]*harness_binary[[:space:]]*=' "$@" | grep -vE '^[[:space:]]*harness_binary[[:space:]]*=[[:space:]]*"rk-fixture-child[A-Za-z0-9_-]*"[[:space:]]*(#.*)?$'; }
  printf 'harness_binary = "claude"\n' > "$tmpd/hb.toml"
  [ -n "$(hb_bad "$tmpd/hb.toml")" ] || { echo "FAIL: synthetic control — the harness_binary probe admits a real claude"; exit 1; }
  hbs=$(grep -rlE '^[[:space:]]*harness_binary[[:space:]]*=' crates/rk-cli/tests/fixtures); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$hbs" ] || { echo "FAIL: no rk-cli fixture config names a harness_binary (rc=$rc) — the process tests would spawn the default claude"; exit 1; }
  grep -qxF 'harness_binary = "rk-fixture-child-claude"' crates/rk-cli/tests/fixtures/pack-min/config.toml || { echo "FAIL: pack-min's config does not name the canonical fixture harness rk-fixture-child-claude (D21-47)"; exit 1; }
  bad=$(hb_bad crates/rk-cli/tests/fixtures)
  [ -z "$bad" ] || { echo "FAIL: an rk-cli fixture config names a harness_binary that is not a fixture child:"; printf '%s\n' "$bad"; exit 1; }

  for spec in "runbook/phase-1/s8c-toolchain-port-and-verbs.md|CP8C|CP8C-END-OF-BLOCK reached" \
              "runbook/phase-1/s8b-catalog-quality.md|CP8B|CP8B-END-OF-BLOCK reached" \
              "runbook/phase-1/s7g-step-executor.md|CP7G|CP7G-END-OF-BLOCK reached" \
              "runbook/phase-1/s7a-ports-and-fakes.md|CP7A|trybuild_pins_the_reason_each_port_fixture_fails"; do
    file=${spec%%|*}; rest=${spec#*|}; tag=${rest%%|*}; needle=${rest#*|}
    test -f "$file" || { echo "FAIL: $file absent — Checkpoint $tag cannot be re-run (Halt 8)"; exit 1; }
    extract_block "$file" "$needle" > "$tmpd/$tag.sh"; rc=$?
    [ "$rc" -eq 0 ] && [ -s "$tmpd/$tag.sh" ] || { echo "FAIL: exactly one $tag block was not found in $file"; exit 1; }
    # 7a's block carries no end marker of its own: one is appended, as s8c's (15) does.
    grep -qxF "echo \"$tag-END-OF-BLOCK reached\"" "$tmpd/$tag.sh" || printf '\necho "%s-END-OF-BLOCK reached"\n' "$tag" >> "$tmpd/$tag.sh" || { echo "FAIL: could not append $tag's end marker"; exit 1; }
    out=$(nodispatch env RK_CP_NESTED=1 bash "$tmpd/$tag.sh" 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $tag, re-run whole, rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
    printf '%s\n' "$out" | grep -qxF "$tag-END-OF-BLOCK reached" || { echo "FAIL: Checkpoint $tag exited 0 without reaching its end marker"; exit 1; }
    nd_check "Checkpoint $tag replay" || exit 1
    echo "ok: Checkpoint $tag re-run whole"
  done

  # CHECKPOINT 5, ONLY IF crates/rk-types MOVED since the entry commit (§8d.11). CP8D_ENTRY, recorded at (0), is the
  # commit at which Checkpoint 8c passed. SYNTHETIC CONTROL first: the diff must see a change against the root commit,
  # which carries no crates/rk-types at all.
  entry=$CP8D_ENTRY
  git cat-file -e "$entry^{commit}" 2>/dev/null || { echo "FAIL: CP8D_ENTRY=$entry names no commit"; exit 1; }
  git merge-base --is-ancestor "$entry" HEAD || { echo "FAIL: HEAD does not descend from CP8D_ENTRY=$entry"; exit 1; }
  root=$(git rev-list --max-parents=0 HEAD | tail -1)
  git diff --quiet "$root" -- crates/rk-types; rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: synthetic control — git diff --quiet over crates/rk-types against the root commit returned $rc, not 1"; exit 1; }
  git diff --quiet "$entry" -- crates/rk-types; rc=$?
  if [ "$rc" -eq 1 ]; then
    extract_block runbook/phase-1/s5-workspace-ci-rk-types.md 'FAIL: deny.toml absent' > "$tmpd/CP5.sh"; rc=$?
    [ "$rc" -eq 0 ] && [ -s "$tmpd/CP5.sh" ] || { echo "FAIL: exactly one Checkpoint 5 block was not found"; exit 1; }
    printf '\necho "CP5-END-OF-BLOCK reached"\n' >> "$tmpd/CP5.sh" || { echo "FAIL: could not append 5's end marker"; exit 1; }
    out=$(nodispatch env RK_CP_NESTED=1 bash "$tmpd/CP5.sh" 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 5, re-run whole over the moved rk-types, rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
    printf '%s\n' "$out" | grep -qxF 'CP5-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 5 exited 0 without reaching its end marker"; exit 1; }
    nd_check "Checkpoint 5 replay" || exit 1
    echo "ok: Checkpoint 5 re-run whole (crates/rk-types moved since $entry)"
  elif [ "$rc" -eq 0 ]; then
    echo "ok: crates/rk-types unchanged since $entry — Checkpoint 5 is not re-run (§8d.11)"
  else
    echo "FAIL: git diff over crates/rk-types could not run (rc=$rc)"; exit 1
  fi
fi
# SYNTHETIC CONTROL: a file with no marked block must make extract_block fail, or the loop above proves nothing.
printf 'no fenced block here\n' > "$tmpd/none.md"
if extract_block "$tmpd/none.md" 'CP8C-END-OF-BLOCK reached' > "$tmpd/none.sh"; then echo "FAIL: synthetic control — extract_block succeeded on a file with no block"; exit 1; fi

# ---- (14) s8's CRITERION (3), ITS ASSERTED HALF, RUN FROM s8's OWN BYTES — the lines from its first fence under
# ---- the (3) heading through its `echo "(3) asserted…"` line. The operational arm is s8's and is not run here.
s8=runbook/phase-1/s8-exit-criterion.md
test -f "$s8" || { echo "FAIL: $s8 absent (Halt 8)"; exit 1; }
awk -v f="$fence" '/^\*\*\(3\) Deterministic resume across a real kill/ {hd=1; next} hd && $0 == f "bash" {inb=1; next} inb {print} inb && index($0, "echo \"(3) asserted") {exit}' "$s8" > "$tmpd/c3.sh"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: awk could not read $s8 (rc=$rc)"; exit 1; }
grep -qF 'kill_and_resume_n_step' "$tmpd/c3.sh" && grep -qF 'echo "(3) asserted' "$tmpd/c3.sh" || { echo "FAIL: criterion (3)'s asserted half was not extracted from $s8"; exit 1; }
out=$(bash "$tmpd/c3.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: criterion (3)'s asserted half, re-run from s8's bytes, rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qF '(3) asserted: AC-013 N-step kill-and-resume' || { echo "FAIL: criterion (3)'s asserted half did not reach its own echo"; exit 1; }
nd_check "(14) criterion (3)" || exit 1

# ---- (15) THE TRAP IS EMPTY AND THE FAKE UNCALLED (D21-47), THE WORKTREE PATH SET IS UNCHANGED, and nothing is
# ---- left for a prune to find.
nd_check "(15) end" || exit 1
rm -rf "$tmpd"
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations after this block:"; printf '%s\n' "$stale"; exit 1; }

echo "CP8D-END-OF-BLOCK reached"
```

---
