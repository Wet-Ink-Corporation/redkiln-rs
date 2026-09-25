# Step 8d — Step-grain commits and the complete reference resolver (Phase 1, part 8d)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8d` | Phase 1 | Checkpoint 8c | Checkpoint 8d | `s8c` | `s8e` |

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
  for R2 (§8d.1 S2, §8d.3).
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
flips only on the operator's dated ledger line.

| # | record | supersedes / discharges |
| --- | --- | --- |
| R1 | **Step-grain commits** (3a, 3b): per-step specs on `RecordedSpec`, the commit key, re-entry at the first uncommitted step, the per-step memo key | supersedes in part **ADR-0143** decision 2 (narrowed: a runnable agent step with **no** per-step spec still refuses by name) and decision 4 in full; **ADR-0150** decision 1 in full; and **ADR-0138**, because ADR-0143's own forbids bullet reads *"Forbids widening AC-013's idempotence grain without superseding ADR-0138"* — the record names which ADR-0138 sentence it moves (its grain sentence sits under *"What this record does NOT decide"*, not in `## Decision`) and says so rather than inventing a clause. Discharges **ADR-0143 clause 5** and **ADR-0150 clause 4**. A dated AC-013 register note: *"no committed step re-executes"* is read at step-activation grain. The commit key is **separate**: *"A separate commit key. ADR-0097's memo key is untouched."* (D21-43 N15). So ADR-0097 is not superseded, and Checkpoint 6 does not re-run. Each per-step spec carries its agent step's `timeout_secs` (N16, §8d.2). That is a per-step bound on the spec the Harness receives, so R1 also supersedes **ADR-0140** in part, for per-step specs only: decision 5's *"A future per-step bound may not be built by overloading it"*, and its forbids bullet *"forbids repurposing it as one"* |
| R2 | **The complete reference resolver**: every input-source form, output binding, prior-step names, ADR-0055's previous iteration, `for_each` and `repeat.until`, `check` evaluation, `emit` binding, and the resolver side of test selection | discharges **ADR-0136 clause 4** and **ADR-0142 clause 4**; supersedes **ADR-0148**'s refusal of `toolchain:test` (clause 4: *"This is reversed by a resolver, not by a revert"*) — `s8c` leaves it standing (its routing table: *"`test` \| ADR-0148's sentence, **unchanged** (s8d reverses it)"*) — while ADR-0148's forbids, that no declared selection is passed through as a test name or as an empty selection, stand. Executes ADR-0035, ADR-0040, ADR-0045, ADR-0055 and ADR-0103 unchanged; ADR-0136's forbids all stand. R2 also emits `artifact.read` for every resolved `artifact:` input (N12). It resolves `kb:`, `checklist:` and `toolchain:<feed>` through `s8c`'s declared tables (N14). It states the value of ADR-0035's `[*].field` over a list-valued field (§8d.3), and it fixes the one home of `selection_of` (§8d.3) |
| R3 | **`on_false` as iteration** (4d) | executes ADR-0031, ADR-0044 and ADR-0103 clause 6's closed set, and ADR-0144's `StepFailedAfterCommit`; names ADR-0046's `deviate` form OWED (0 shipped uses — see §8d.4) |
| R4 | **Failure telemetry and the breaker's call site** (20) | the kernel becomes the writer of the `step_failed` / `step_cancelled` / `step_refused` / `rate_limited` row; design §5's row names the producer as *"the adapter, via the dispatch outcome"*, which takes a dated amendment under this record; `crates/rk-kernel/src/runtime.rs`'s table row *"AC-033's breaker mark \| **none exists**"* is corrected in the same change |
| R5 | **Two verb-registry rows and the question registry**: `shallow_clone` → the retained review clone, `verdict_record` → `review::confirm`; `.redkiln/processes/` populated and loaded | `shallow_clone` takes `s8c`'s closed `git` route, whose first handler this stage builds; `verdict_record` takes a route R5 adds to that closed set **by record**, as `s8c` provides — *"`s8d`'s record may add a route for `verdict_record`. It does so by record, not by editing the match."*; lifts nothing ADR-0156 or ADR-0162 forbid, because both scope their bar — *"under this stage's scope"*, *"under this record's scope"*; executes ADR-0129 decision 3's *"Pack-declared questions … are Step 7e's, loaded into the same type"*, whose loader was never built. The loader and `.redkiln/processes/` belong to this stage alone (D21-43: *"The loader and `.redkiln/processes/` are **s8d**'s."*). `s8j` authors the human-root questions into that directory, and `s8g` loads through this loader |
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
`budget.wall_clock_secs`. The binder resolves a substitution through `s8c`'s 4a table (*"an unresolved key is refused
by name"*). It sets the per-step spec's `timeout_secs` to `min(step, budget.wall_clock_secs)`. No literal exceeds its
activity's budget today (0 of 152), so the bound bites only on a substituted value or a later edit. The test pins
both arms. ADR-0140 decision 5 says of `ActivitySpec.timeout_secs`: *"A future per-step bound may not be built by
overloading it."* This is exactly such a bound, carried on the spec the Harness receives. So R1 supersedes that
sentence and the forbids bullet *"forbids repurposing it as one"* in part, for **per-step** specs. An activation's
own spec keeps the per-activity value that decision 5 describes.

**The commit key is `(item, activity@version, step_id, iteration path)`**, and every agent step activation commits
under its own. The iteration path is a stack of frames, outermost first — a `for_each` index, a `repeat` count, a
loop-back count — and is empty outside any loop. Two iterations of one step never share a key (AC-017's *"the loop
iteration counter is part of the step key"*, carried to the commit).

**Re-entry resumes at the first uncommitted step.** The walk runs in declaration order; a step activation whose
commit is in the journal is **served** from it, never re-dispatched (AC-013); the first step with no commit
executes. A `tool` or `check` step does not commit (3a names agent steps), so on re-entry it re-executes, and a later
`${…}` that reads it binds the re-executed value. That is how ADR-0150's named residual — *"39 mapped `tool` steps
across 23 activities"* lost to a crash after the commit — closes: by re-execution, not by a tool-step commit. R1
states it in those words.

**An activation's completion is its `emit`**, and `fsm::next_action`'s plan walk moves past a node only when the
emit binding has committed — not when its first agent step has. Moving `activity_committed` to the end of the walk
stays forbidden (ADR-0150's forbids): each agent step commits when **it** dispatches.

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
| `${<step>.outputs[.path]}`, `${<step>}`, `.len()` | the prior step's bound output in this activation, at the **current** iteration path | a mandatory ref to a step that has not run refuses by name (step id + reference); `${…?}` is `null` (ADR-0032) |
| `${<this-step>.outputs?}` inside a loop | the **previous** iteration's output, `null` on the first pass (ADR-0055) | — |
| `${<gate>.answer}`, `${<gate>.attachments.<Kind>}` | the `gate_answered` row's option / admitted attachment (ADR-0045; ADR-0129 decision 4) | `.outputs` on a gate refuses by name (ADR-0045) |
| `${item}` | the current `for_each` element (schema: *"runs one copy per element with `${item}` bound"*) | outside a `for_each`, refuses by name |
| `${step.outputs[*].field}` | ADR-0035's projection over a fan-out's list, one value per element in element order. Over a list-valued field, R2 states the value (below) | any other projection refuses by name. A bare `${step.outputs}` of a fan-out is never flattened into a test list (N13) |
| `${toolchain.<key>}` | `s8c`'s 4a table (*"an unresolved key is refused by name"*) | refused by name |
| `artifact:Kind[.field]` | the committed `emit` binding of the activation that produced `Kind` for this item, read from the journal (tier 2), with **one** `artifact.read` row per resolved input (N12, below). `s8f` renders the bytes; this stage reads the binding | mandatory: refuses by name; `?`: `null`. Neither emits a read |
| `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]` | the key's row in `s8c`'s `[kb.*]`, `[checklist.*]` or `[feed.*]` table (N14). An atom or checklist resolves to its content with its content digest (ADR-0097 component 6 for `kb:`). A feed resolves by calling the producing verb its row names, through `s8c`'s registry method | an undeclared key refuses by name, and so does a declared key with no content; `?` is `null` (ADR-0032) |
| literals | themselves | — |

🛑 **Unchanged, and each is asserted:** `null_out_refs` is never called on the execution path; `UnresolvedRef` is
never `Tri::Null`, `Tri::False` or a skip; a refusal names the step id **and** the reference (ADR-0136's forbids).

**Iteration.** `for_each = "${step.outputs.<list>}"` or `"artifact:Kind.<list>"` (ADR-0035) runs one activation per
element, each under its own path frame; AC-022's charge stays one. `repeat = { until, max }` re-runs the step while
`until` is not `Tri::True`, each pass a new frame; at `max` it takes `on_exhausted`. The shipped catalog spells
`on_exhausted = "escalate"` as schema item 15's documented pattern — *"an explicit `gate` step with `when` over the
looped step's status"* — so exhaustion proceeds to that gate step; an `escalate` with no such gate step refuses by
name.

**The `[*]` projection over a list-valued field (N13).** At `a6349b6`, `tdd-red-green-refactor`'s `red-verify` reads
`inputs = { selection = "${red.outputs}" }`. N13's edit to `${red.outputs[*].tests_added}` is `s8b`'s (§8d.1 S2).
The resolver adds **no** implicit flatten. A bare `${red.outputs}` still resolves to a list of `ImplReport` values,
and a selection built from it refuses by name. **Finding, named for R2:** ADR-0035 fixes the syntax
(*"`${step.outputs[*].field}` as the only projection"*) but not the value when `field` is itself a list, and
`tests_added` is a list (`spikes/p9/schemas.toml`: `lists  = ["files", "tests_added"]`). All three shipped uses
project `tests_added` into a `selection`: `bdd-outside-in`, `technical-spike`, and `tdd` after `s8b`'s edit. A list
of lists is not a selection. So R2 states the value. This stage reads it as the concatenation in element order,
because the projection is the one declared point where a fan-out's lists meet. That is also the reading N13's choice
presupposes. If the operator's flip line reads it otherwise, the three activities take a dated line.

**`check` evaluation.** A `check` step's `expr` is evaluated against a `Context` built from **its own `inputs`
table** — `red-check`'s `report` is `${red-verify.outputs}` — through the same `predicate::evaluate`, never a second
evaluator (ADR-0136 decision 1). `Tri::True` continues; `Tri::False` goes to §8d.4; `Tri::Null` keeps ADR-0032/AC-079;
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

**`artifact.read` (N12).** Every `artifact:` input the resolver resolves emits exactly one tier-2 `artifact.read`
row. This is the input half of ADR-0116's pair, whose quotation of design §5's row reads *"kind, item id,
`schema_version`, the tree hash served · every activity input"*. The resolver's `artifact:` arm is the only emitter.
An optional input that resolves `null` emits nothing, and neither does a refused one. **Ordering, named:** the seam
is `s8f`'s. D21-43 N12: *"`artifact.read` is emitted by s8d's resolver through s8f's rk-store read seam."* Checkpoint
8f comes after this stage, so this stage builds the call site as one kernel function the resolver calls. That
function serves the journalled binding and emits the row. `s8f` puts its rk-store read behind the same function and
adds no second emitter. `doctor`'s `unread` rule groups `artifact.read` rows by `stage` and `kind`
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
| `{ to = "deviate", … }` | refuse by name, naming ADR-0046's route OWED — 0 shipped uses, and R3 records it rather than building a mid-walk `step7` amendment no activity asks for |

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
— onto what exists: `shallow_clone` on the closed `git` route to `rk_git::clone::prepare_review_clone` (ADR-0098's
retained gix clone), and `verdict_record` on a route R5 adds by record to `rk_kernel::review::confirm` (ADR-0162,
which classifies findings by re-execution). `rk-git` is a concrete crate and not a port (`rk-kernel`'s
manifest: *"`rk-ports` declares five ports and no sixth, and its `git_port_is_not_a_port` fixture is what refuses
the sixth"*), and `rk-kernel` opens no files, so R5 names which side performs the clone and hands the kernel its path.

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
and tags (plus the packet's hand application), not a run.

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
| `review` | mandatory | `adversarial-review` (`code-review` is not admitted: `max_rigor = "recommended"`) | **walks** | `verdict` is `s8b`'s re-execution form (D21-42 11a); `clone` and `record` route by §8d.6's rows |
| `release` | recommended | `build-package`, `rolling-deploy` | `build-package` **walks**; `rolling-deploy` refuses | `build-package`: `built` reads `s8c`'s widened `build` outcome (N25), and `package`'s handler is `s8c`'s record V. `rolling-deploy`: `deploy` has no registry row and is refused by bare name; it is replaced by `s8g`'s `tag-release` (6b) |
| `closeout` | recommended | `debt-logging`, `closure-record`, `lightweight-retro` | `closure-record` and `lightweight-retro` **walk**; `debt-logging` refuses | `debt-logging`: `item_create` — `s8f` (6a). `closure-record` reads `toolchain:backlog` and `checklist:closure`, and `lightweight-retro` reads `kb:retro-format` and `checklist:retro` (N14) |

**Claimed: 12 of 15 walk, and 3 refuse by name** (`smoke-test`, `rolling-deploy`, `debt-logging`). Each refusal is
owned above: `s8g`, `s8g` and `s8f`.

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
| `closeout` | optional | `closure-record`, `debt-logging` | `closure-record` **walks**; `debt-logging` refuses | as the fix lane |

**Claimed: 10 of 13 walk** (`bdd-scenarios`, `t-shirt-sizing`, `tdd-red-green-refactor`, `change-summary`,
`docs-update`, `equivalence-boundary`, `sca-scan`, `adversarial-review`, `build-package`, `closure-record`), and **3
refuse by name**, the same three as the fix lane with the same owners. Neither lane walks end to end after this
stage, and the tables say why by owner. That is the result, not a failure of the stage.

### 8d.10 Tests, red first, and the mutants that must die

Every test below is authored, run, and **watched fail for the right reason** before the code it pins
(golden constraint 12). Every name is module-qualified in its crate's `--test it` binary.

**`rk-kernel`:**

- `step_grain::` — `each_agent_step_dispatches_under_its_own_spec_and_commits_under_its_own_key`,
  `a_runnable_agent_step_with_no_per_step_spec_refuses_and_the_report_stays_committed` (defect 2),
  `re_entry_resumes_at_the_first_uncommitted_step_and_re_dispatches_no_committed_one`,
  `two_iterations_of_one_step_never_share_a_commit_key`,
  `the_memo_key_of_an_agent_step_carries_that_steps_own_card_digest`,
  `an_activation_with_no_agent_step_walks_to_its_emit_and_completes`,
  `an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget` (N16),
  `the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts` (N15).
- `cross_process::kill_and_resume_n_step` (rewritten, §8d.2) and `cross_process::kill_and_resume_n_activation`
  (the preserved body).
- `reference_resolver::` — `every_input_source_form_resolves_or_refuses_by_name` (ADR-0040's closed set, each
  form, both outcomes: the completeness guard D21-11 asks for),
  `a_prior_step_output_binds_by_step_name_and_field_path`,
  `an_optional_reference_is_null_and_a_mandatory_one_refuses_when_its_step_has_not_run`,
  `a_self_reference_in_a_loop_binds_the_previous_iteration_and_is_null_first`,
  `for_each_fans_out_over_a_list_and_star_projects_its_outputs` (it also asserts the value of `[*]` over a
  list-valued field that R2 states, and that a bare fan-out `${…}` is never flattened, N13),
  `repeat_until_reruns_until_the_predicate_holds_and_takes_on_exhausted_at_max`,
  `a_gate_step_binds_answer_and_never_outputs`,
  `every_shipped_step_reference_is_in_the_resolver_grammar` (over `.redkiln/catalog`, asserting a non-zero count
  walked — D21-42 13a's rule),
  `null_out_refs_has_no_execution_path_caller` (a code-lines-only lint with a live control that a planted call is
  seen),
  `every_resolved_artifact_input_emits_exactly_one_artifact_read` (N12: one row per resolved input, none for a
  `null` optional or a refusal),
  `kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name` (N14).
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

**New test names: 53.** 41 are in `rk-kernel`, one of them replacing defect 1's old test, which is removed. 12 are
in `rk-cli`. That is **52 net** of the removal. The `guard_context` and `processes` renames move nothing. The
2026-09-24 revision added six of the 53: five in `rk-kernel` and one in `rk-cli`.

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
| M19 | the resolver's `artifact:` arm emits no `artifact.read`, or emits two | `reference_resolver::every_resolved_artifact_input_emits_exactly_one_artifact_read` |
| M20 | a per-step bound drops `min(…, budget.wall_clock_secs)` | `step_grain::an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget` |
| M21 | an undeclared `kb:` key resolves to empty content instead of refusing | `reference_resolver::kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name` |
| M22 | a `null` `change` at triage opens no gate, so the deriver's null is kept in silence | `change_deriver::a_null_change_at_triage_opens_the_unknown_facet_gate_and_its_answer_binds_the_facet` |
| M23 | `built` reads the `build` call's `is_ok()` instead of the report's status | `check_eval::a_red_build_report_fails_the_built_check_by_name` |

### 8d.11 What this stage makes provisional, and what it does not claim

**Checkpoints it touches.** Checkpoint 8c names two tests whose premise is the resolver's absence —
`tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port` (*"ADR-0148 stands"*) and
`tool_verb_routing::a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port`. Each keeps its name
only if its fixture is narrowed to a case that is still true (a selection or an input the resolver refuses by
name); otherwise it is renamed and Checkpoint 8c takes a dated amendment naming the new test. Either way Checkpoint
8c is re-run whole inside Checkpoint 8d **after** that edit. Checkpoint 7g's guards (7a) and (8) name three tests
this stage renames, replaces or narrows (the two in §8d.10 and the first `tool_verb_routing` test above); the
stage amends Checkpoint 7g by a dated line naming the new tests (ADR-0092's chain: the accepted record is the
authority, so the amendment is execution order) and re-runs it whole inside Checkpoint 8d. `rk-ports/src/fakes.rs`
changes, so **Checkpoint 7a** is made provisional and its row is marked per the legend's *"passed `<hash>`, re-run
pending"* rule until it passes. Its whole re-run is carried by this stage's re-run of Checkpoint 8c, whose block
re-runs 7a whole (`s8c`'s (15)). If the revised Checkpoint 8c no longer does, Checkpoint 8d adds 7a to its own loop
by a dated amendment. Checkpoint 5 re-runs if R4's payload members land in `rk-types`. **Checkpoint 6 does not
re-run.** N15 keeps `IdempotencyKey` three parts, and
`step_grain::the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts` pins that. s8's criterion (3)
asserted half is re-run inside Checkpoint 8d.

**Nested re-runs cost hours, so Checkpoint 8d re-runs only these.** It re-runs its entry gate (8c) and the one older
checkpoint its own renames make provisional (7g). It also re-runs s8's criterion (3) asserted half, which this stage
re-proves. It does not re-run 8a, 8b or any other checkpoint. What 8c's own edits made provisional is 8c's block's
business.

**No re-run here can reach a real dispatch.** Checkpoint 7g's block runs `rk-cli`'s process-level tests, and those
spawn `redkiln run`. At this stage's time the shipped cards carry no `permission_mode`: 0 of the 8 files under
`.redkiln/cards/` at `a6349b6`. `lint_card` refuses a blank one with `ConfigError::PermissionModeAbsent`, so the
Claude adapter refuses every shipped card before it builds a command. Checkpoint 8d asserts that precondition before
its re-runs (§8d.12 (12a)). Once `s8h` gives the cards their modes, a later whole re-run of 7g must point
`[adapters] harness_binary` at the fixture child, or go through a fixture config. It must also assert that no real
`claude` was spawned (D21-43's integration call).

**D21-44 and D21-45 touch nothing here.** No run-worktree path appears in this file. The loader reads
`.redkiln/processes/*.toml`, and the catalog tests read `.redkiln/catalog/*.toml`. Neither recurses over `.redkiln/`
as a whole, so D21-44's `worktrees/` exclusion has no site in this stage. The worktree path-set guards in Checkpoint
8d compare `git worktree list` before and after, wherever the root is.

**What this stage does not claim.** No shipped activity completes against a real harness, and nothing here dispatches
to a real account (D21-40 decision 2). Cards, schemas and models are `s8h`'s; the tier-1 writer and the lane across
processes are `s8e`'s; rendering and landing are `s8f`'s; the verify record and the restored producers are
`s8g`'s. The memo is still consulted by nothing. Criterion (1) is not satisfied, and §8d.9 says by owner why.

### 8d.12 Checkpoint 8d

- **Each agent step dispatches under its own spec and commits under its own key**; a runnable agent step with no
  per-step spec refuses by name **and the report stays committed** (defect 2). Each per-step spec carries its
  bound, never above the activity budget, and the idempotency key keeps three parts.
- **Re-entry resumes at the first uncommitted step** and re-dispatches no committed one, across a real process
  death — `kill_and_resume_n_step` at step grain, and criterion (3)'s asserted half re-run from s8's own bytes.
- **The reference resolver is complete**: every ADR-0040 form resolves or refuses by name; the shipped catalog's
  references are all in its grammar; `null_out_refs` has no execution-path caller. `kb:`, `checklist:` and feed
  keys resolve through `s8c`'s tables, every resolved `artifact:` input emits one `artifact.read`, and
  `selection_of` has one body.
- **`check`, `emit` and test selection** decide from resolved values; an unresolvable input is never read as false;
  a step after `emit` is refused, not dropped.
- **`on_false` iterates**, bounded, and exhaustion refuses as `StepFailedAfterCommit` with committed steps standing.
- **One error row per failed dispatch**, written by the kernel under either harness, carrying what the breaker folds;
  the breaker refuses from the store.
- **The two verb rows route**, the nine lane questions and N3's `facet.change` load, `answer` validates against
  them, a `null` `change` at triage opens AC-079's gate and its answer binds the facet, and the `change` deriver is
  the only source of `change`.
- **The walk table holds** — both lanes, exactly.
- **Nothing regressed**: the executor guards `s7g` built stay green, Checkpoints 8c and 7g re-run whole, the whole
  suite passes above its floor, and `fmt` and `clippy` are clean. No re-run can reach a real dispatch.

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

# ---- (1) BUILD-OR-FAIL. `target/` is gitignored; the rk-cli selections spawn the binary.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8d NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists — every spawning test below would be about nothing"; exit 1; }

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
  step_grain::an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget \
  step_grain::the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts \
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
  reference_resolver::for_each_fans_out_over_a_list_and_star_projects_its_outputs \
  reference_resolver::repeat_until_reruns_until_the_predicate_holds_and_takes_on_exhausted_at_max \
  reference_resolver::a_gate_step_binds_answer_and_never_outputs \
  reference_resolver::every_shipped_step_reference_is_in_the_resolver_grammar \
  reference_resolver::null_out_refs_has_no_execution_path_caller \
  reference_resolver::every_resolved_artifact_input_emits_exactly_one_artifact_read \
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
test -d crates/rk-kernel/src/runtime && test -f crates/rk-kernel/src/replay.rs || { echo "FAIL: the execution-path sources are not where this floor reads (Halt 8)"; exit 1; }
raw=$(grep -rnF 'null_out_refs' crates/rk-kernel/src/runtime crates/rk-kernel/src/runtime.rs crates/rk-kernel/src/replay.rs); rc=$?
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

# ---- (11) BULLET 9: THE WHOLE SUITE, `--no-fail-fast`, rc 0, zero failed, above a FLOOR. RE-DERIVED at stage open
# ---- from Checkpoint 8c's recorded whole-suite passed count + 52 (§8d.10's net new tests), and the derivation is
# ---- recorded. The expected value is 832: Checkpoint 8c's drafted floor of 780 (740 at `992d257`, 0 failed, 9
# ---- ignored, 44 results, plus its 40) + 52. If 8c recorded more, the floor rises with it. It is never lowered to
# ---- make a red run green.
FLOOR=832
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

# ---- (12) fmt and clippy, gated on exit codes (never `-D warnings` on the command line — ADR-0083; PIA-030).
out=$(cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }

# ---- (12a) PRECONDITION: NO RE-RUN BELOW CAN REACH A REAL DISPATCH (D21-43; §8d.11). 7g's re-run spawns
# ---- `redkiln run` through rk-cli's process tests. Before s8h no shipped card carries `permission_mode`, and
# ---- `lint_card` refuses a blank one. If a card carries one, s8h has landed: the 7g re-run must then go through the
# ---- fixture child (`[adapters] harness_binary`) with a no-real-claude assertion, and this block is amended first.
ncards=0
for c in .redkiln/cards/*.json; do [ -f "$c" ] && ncards=$((ncards + 1)); done
[ "$ncards" -gt 0 ] || { echo "FAIL: no card files under .redkiln/cards — the precondition would hold over nothing (Halt 8)"; exit 1; }
moded=$(grep -lF '"permission_mode"' .redkiln/cards/*.json); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the card-mode precondition could not run (grep rc=$rc)"; exit 1; }
[ -z "$moded" ] || { echo "FAIL (PRECONDITION): a shipped card carries permission_mode, so s8h has landed. Point the 7g re-run at the fixture child and assert no real claude spawn before running this block:"; printf '%s\n' "$moded"; exit 1; }
# SYNTHETIC CONTROL: the probe must see a planted mode.
printf '{"permission_mode": "default"}\n' > "$tmpd/card.json"
grep -qF '"permission_mode"' "$tmpd/card.json" || { echo "FAIL: synthetic control — the card-mode probe cannot see a planted permission_mode"; exit 1; }

# ---- (13) THE ENTRY CHECKPOINT AND THE EXECUTOR CHECKPOINT, RE-RUN WHOLE FROM THEIR OWN BYTES — and ONLY these two
# ---- (§8d.11: nested re-runs cost hours). 8c is the entry gate; 7g is provisional on this stage's renames. 7a rides
# ---- inside 8c's own block. The fence string is built from octal escapes so this block carries no literal fence.
fence=$(printf '\140\140\140')
test -n "$fence" || { echo "FAIL: the fence string is empty — extract_block would match every line"; exit 1; }
extract_block() {
  awk -v f="$fence" -v m="$2" '$0 == f "bash" {inb=1; buf=""; next} inb && $0 == f {inb=0; if (index(buf, m)) {printf "%s", buf; n++}; next} inb {buf = buf $0 "\n"} END {exit (n == 1) ? 0 : 1}' "$1"
}
for pair in "runbook/phase-1/s8c-toolchain-port-and-verbs.md:CP8C" "runbook/phase-1/s7g-step-executor.md:CP7G"; do
  file=${pair%%:*}; tag=${pair##*:}
  test -f "$file" || { echo "FAIL: $file absent — Checkpoint $tag cannot be re-run (Halt 8)"; exit 1; }
  extract_block "$file" "$tag-END-OF-BLOCK reached" > "$tmpd/$tag.sh"; rc=$?
  [ "$rc" -eq 0 ] && [ -s "$tmpd/$tag.sh" ] || { echo "FAIL: exactly one $tag block was not found in $file"; exit 1; }
  out=$(bash "$tmpd/$tag.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $tag, re-run whole, rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  printf '%s\n' "$out" | grep -qxF "$tag-END-OF-BLOCK reached" || { echo "FAIL: Checkpoint $tag exited 0 without reaching its end marker"; exit 1; }
  echo "ok: Checkpoint $tag re-run whole"
done
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

# ---- (15) THE WORKTREE PATH SET IS UNCHANGED, and nothing is left for a prune to find.
rm -rf "$tmpd"
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations after this block:"; printf '%s\n' "$stale"; exit 1; }

echo "CP8D-END-OF-BLOCK reached"
```

---
