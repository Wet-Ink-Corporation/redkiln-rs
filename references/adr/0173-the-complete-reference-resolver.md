---
id: "0173"
title: "**The complete reference resolver** (`s8d`'s record R2): one mechanism, `runtime::reference`, resolves every step-input source form ADR-0040 closes — prior-step output binding by step name and field path, ADR-0055's previous iteration, feedback refs, the `??` coalesce, gate answers, `${item}`, the `[*]` projection (concatenation in element order), `${toolchain.*}`, `artifact:` through one read-seam call, and `kb:`/`checklist:`/`toolchain:<feed>` through `s8c`'s declared tables — plus `for_each` and `repeat.until` iteration (the repeat body runs through the last step `until` references), `check` evaluation, `emit` binding and the resolver side of test selection. Discharges `0136` clause 4 and `0142` clause 4. Supersedes, each in part and quoted: `0042`'s Decision and first forbids bullet (an escalate gate whose re-entry resumes into the referencing step also licenses a forward ref, D21-48 X8), and `0148` decisions 1, 3 and 4 (`toolchain:test` is reversed by this resolver; `catalog::Step` carries `inputs`)"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R2's row), §8d.3 (the resolver's table, iteration, the `[*]` projection, *Which forward refs are licensed*, the census, the four removals, `check` evaluation, `emit` binding, `artifact.read`, test selection) and §8d.10; spikes/DECISIONS-2026-09-21.md D21-11 (*"It does **not** license a partial resolver"*), D21-41 (d), D21-42 4a and 4c, D21-43 N12, N13 and N14, D21-47 (the read seam's one emitter), D21-48 X5, X6 and X8, D21-57 (AC-303's rule moves onto this record's carrier), D21-60 (D59b), D21-64 ruling 10 (this record's flip condition), D21-74 ruling 2 (decision 9's dated amendment, 9a, 2026-09-27); references/adr/0042 (Decision and first forbids bullet), 0136 clause 4 and forbids, 0142 clause 4, 0148 decisions 1, 3 and 4 and forbids; crates/rk-kernel/src/catalog.rs (`Step::inputs`, `forward_ref_lint`), crates/rk-kernel/src/select/lints.rs (`load_lints`) and crates/rk-kernel/src/rigor.rs (`confirm_step_monotonicity_rule`) at `s8d`'s build
supersedes: ["0042", "0148"]
superseded_by: []
related: ["0032", "0034", "0035", "0040", "0043", "0045", "0055", "0072", "0103", "0136", "0142", "0172"]
evidence: []
consumers: []
---

## Context and problem

The step executor `s7g` built walks the five closed step kinds and completes nothing: *"**0 of 102** shipped
activities complete a committing walk"* (ADR-0147 clause 3). ADR-0136 clause 4 names why: *"The reference resolver
is OWED and is named here rather than left implicit. What it must supply: a `Context` populated from the outputs of
prior steps in the same activation, under a name resolution that agrees with the grammar ADR-0103 fixes and with
ADR-0055's rule that a self-reference inside a loop binds the **previous** iteration."* ADR-0142 clause 4 leaves the
`check` arm refusing for the same reason: *"Binding them requires the reference resolver ADR-0136 clause 4 records
as **OWED**, and this record does not build it."* ADR-0148 made `toolchain:test` refuse until a selection resolver
exists, and its decision 4 says how that ends: *"This is reversed by a resolver, not by a revert."*

The ledger sets the shape. D21-11: *"It does **not** license a partial resolver — a lookup for `${x.answer}` but not
`${x.outputs.y}` — and it does not make the resolver anyone's"*. D21-41 (d) gives it to `s8d`: *"Build the complete
reference resolver (D21-11): output binding, iteration, check evaluation, emit binding and test-selection
resolution."* ADR-0136's forbids: *"The resolver is one mechanism with one record, or it is absent."* This is that
record.

Two findings the stage measured need a ruling, and D21-48 took both:

- **Forward refs.** ADR-0042's Decision is *"A ref to a later-declared step is legal iff that step or one after it
  has `on_false`/`repeat` targeting the referencing step; it is `null` on the first pass."* Nothing in `crates/`
  enforced it (D21-60: `s8b`'s control mutant D59b, which restored an unlicensed back-ref, survived the whole
  workspace). Measured over `.redkiln/catalog` at `s8d`'s stage open (a read-only proxy, and then from inside the
  test binary): **80** forward refs in `inputs`, all optional, 15 of them gate answers; **63** licensed by the
  letter, **3** by a repeat body, **10** by neither but by an escalate gate whose answer resumes into the
  referencing step (both lanes' `minimal-repro` and `tdd-red-green-refactor` among them), and **4** by nothing.
  D21-48 X8: *"R2's record supersedes ADR-0042's Decision in part: an escalate gate whose re-entry resumes into the
  referencing step also licenses the ref. The lint widens to match. Any ref still unlicensed is a lint error, fixed
  in the catalog in s8d."*
- **`repeat.until` over a later step, and `[*]` over a list.** The schema defines `until` as *"<predicate over this
  step's outputs>"*, but three shipped `until`s read a later-declared step. ADR-0035 fixes the `[*]` syntax but not
  its value over a list-valued field, and `tests_added` is one. D21-48 X6 and X5 took both (Decision 4 and 3).

`catalog::Step` did not carry `inputs`. ADR-0148 decision 3 kept it off: *"`catalog::Step` is not widened to parse
`inputs`. Parsing it would put a value in the type that nothing may act on"*; its fourth forbids bullet forbids
*"Widening `catalog::Step` to parse `inputs` without a record routing what may then read it."* D21-57 routes AC-303's
confirm rule onto this record's carrier when it lands.

## Decision drivers

- **One mechanism or none** (ADR-0136's forbids). A resolver that binds some forms and not others is the partial
  resolver D21-11 refuses.
- **An unresolvable reference is never a value.** `UnresolvedRef` is never `Tri::Null`, `Tri::False` or a skip, and a
  refusal names the step id and the reference (ADR-0136's forbids, standing).
- **A forward ref must be decidable at load**, as ADR-0042's own Positive says (*"The rule is lint-checkable"*), and
  the widened licence must keep the lanes' escalate-gate refs, which carry each lane's human hint back into the step
  that stalled.
- **An absent optional value is a null field, never the literal** (ADR-0172 clause 3): `${a?}` over an absent `a`
  binds a `null` value and never rewrites the expression into the literal `null`.

## Considered options

### Option — The complete resolver, one record, with the widened licence (taken)

Every ADR-0040 form resolves or refuses by name through `runtime::reference`, the iteration and binding readings
D21-48 took are stated, and the forward-ref lint admits the three licences. Taken as D21-11, D21-41 (d) and D21-48
X5, X6 and X8 direct.

### Option — ADR-0042's letter alone, and edit the catalog to fit

Enforce the Decision as written and remove or re-loop every ref it does not license. Rejected by D21-48 X8 and
D21-60: it would refuse the ten escalate-gate refs, among them the two the lanes depend on (`minimal-repro`'s
`attempt` reading `${triage-gate.answer?}` and `tdd-red-green-refactor`'s `green` reading `${green-gate.answer?}`),
and turning each escalation into a loop-back would be ten pack-author decisions, not coherence edits.

### Option — A partial resolver now, the rest later

Bind `${<step>.outputs…}` for `check` evaluation only, and leave iteration, `artifact:` and selection for later.
Rejected by D21-11 in terms (*"It does **not** license a partial resolver"*).

### Option — Carry `inputs` on a side structure, not on `catalog::Step`

Read `inputs` from each activity's raw TOML wherever needed, as AC-303's rule did through `toml_subset`. Rejected:
it keeps two parses of one key, and D21-57 already ruled that the rule moves onto this record's carrier.

## Decision

**1. The resolver is one mechanism**, `crates/rk-kernel/src/runtime/reference.rs` (`runtime::reference`), and not
`crates/rk-kernel/src/resolver.rs`, which is AC-494's capability resolver and stays as it is (`s8d` §8d.3, m11). For
every step it reaches it resolves each `inputs` value, by form:

| form | resolves to | absent or unresolvable |
| --- | --- | --- |
| `${<step>.outputs[.path]}`, `${<step>}`, `.len()` | the prior step's bound output in this activation, at the current iteration path. *(dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting round 2's bare-`${<step>}` finding, option (b)): this holds for `${<step>.outputs[.path]}` and `.len()`. A **bare** `${<step>}` is an evidence pointer `{"step": <id>, "kind": <step kind>}`, not the step's output, resolved once the step has run in this activation: it is an `emit`'s evidence reference, and ADR-0036 quotes design §2's rule that every `Evidence` variant *"names the step that produced it"*, and its Decision fixes the variant by step kind (*"a `tool` step mints `ToolReport` … an `agent` step mints `Findings` or `SelfCheck` … a `gate` step mints `HumanAnswer`"*). The pointer is what the completion row's `evidence` member carries (decision 7).* *(dated 2026-09-27, `s8d` build, package R6FIX-8d; refuting round 6's spec-conformance major.)* **"Once the step has run in this activation" means in ANY frame of the activation, not only the current one**: a loop-back, `repeat` pass or escalate re-entry opens a new frame that does not un-run a step for the bare pointer, which reads an activation-wide set no frame clears. `${<step>.outputs[.path]}`, `.len()` and `.answer` keep their frame readings. An escalate gate answered before its re-entry, and skipped by its own `when` in the resumed frame, is therefore evidence on the completion row (a `gate` step mints `HumanAnswer`, ADR-0036). | a mandatory ref to a step that has not run refuses by name (step id and reference); `${…?}` is a `null` value (ADR-0032: *"Absent `?` inputs are `null`"*) |
| `${<this-step>.outputs?}` inside a loop | the previous iteration's output (ADR-0055) | `null` on the first pass |
| a licensed forward ref (decision 2) | the value the later step bound in the **previous** frame of the cycle that re-enters this step | `null` on the first pass (ADR-0042's second half, standing); never the current frame's value |
| `${a?} ?? ${b}` (ADR-0034) | the left operand's value unless it is `null`, otherwise the right operand's | the right operand resolves like any other ref |
| `${<gate>.answer}`, `${<gate>.attachments.<Kind>}` | the `gate_answered` row's option or admitted attachment (ADR-0045) | `.outputs` on a gate refuses by name |
| `${item}` | the current `for_each` element | outside a `for_each`, refuses by name |
| `${step.outputs[*].field}` | ADR-0035's projection, one value per element in element order | any other projection refuses by name |
| `${toolchain.<key>}` | `s8c`'s per-toolchain table (D21-42 4a) | refused by name |
| `artifact:Kind[.field]` | the committed `emit` binding of the activation that produced `Kind` for this item, read from the journal, through **one** read-seam call per resolved input | mandatory: refuses by name; `?`: `null`; neither calls the seam |
| `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]` | the key's row in `s8c`'s `[kb.*]`, `[checklist.*]` or `[feed.*]` table (D21-43 N14) | an undeclared key refuses by name even with `?`; a mandatory declared key with no content refuses; `?` makes only a declared key with no content `null` |
| literals | themselves | — |

**2. The forward-ref licence (D21-48 X8).** A ref from step R to a later-declared step L is admitted at load only
when it carries `?` and one of three licences covers it; every other ref is refused by name, giving the activity,
the step and the reference, whether or not it carries `?`:

1. **The letter (ADR-0042).** L, or a step after it, has `on_false = { to = R, … }`.
2. **The repeat body (decision 4).** R declares `repeat`, and L lies within R's repeat body.
3. **The escalate re-entry (X8).** R declares `escalate = { question = Q }` (directly or with `on_exhausted =
   "escalate"`), and L, or a step after it, is a `gate` step whose `question` is Q. A gate with another question
   licenses nothing.

The lint is `rk_kernel::catalog::forward_ref_lint`, run at load by `select::load_lints`, the refusing load rules
every shipped path into selection runs before step 1. It is not run by the strict parser, because the frozen p9
controls load `spikes/p9/catalog` at `P9_PIN`, which carries the four refs the catalog fix removes, and a frozen
control must stay loadable. A licensed ref binds as decision 1's forward-ref row says. An escalate re-entry is a new
frame of the referencing step, so a later **tool** step's output is bound across it as a gate's answer is. The four
refs no licence covered (`adr-record`'s and `spike-question-spec`'s `${accept.answer?}`,
`structured-nl-requirements`' `${conflicts.answer?}`, `vertical-slicing`'s `${approve.answer?}`) are removed from
the catalog under ADR-0072, each with a version bump and a changelog row (`s8d` §8d.3).

**3. The `[*]` projection over a list-valued field is concatenation in element order** (D21-48 X5: *"Concatenation
in element order. Nothing is flattened implicitly anywhere else."*). This projection is the only place a fan-out's
lists are joined; a bare `${step.outputs}` of a fan-out is never flattened into a selection.

**4. The repeat body** (D21-48 X6): *"The repeat body runs from the repeating step through the last step `until`
references, in declaration order, one `repeat` frame per pass, and `until` is evaluated after the body's last
step."* An `until` over only the step's own outputs has a one-step body. **Schema finding, named:** the schema's
`until` (*"<predicate over this step's outputs>"*) does not describe the three shipped `until`s that read a later
step; the schema text under `references/` is not edited.

**5. Iteration.** `for_each` runs one activation per element, each under its own iteration-path frame, and charges
the pass once (AC-022). `repeat = { until, max }` re-runs its body while `until` is not `Tri::True`, each pass a new
frame; at `max` it takes `on_exhausted`, and the shipped `on_exhausted = "escalate"` proceeds to the escalate gate
step (schema item 15's pattern); an `escalate` with no such gate step refuses by name.

**6. `check` evaluation.** A `check` step's `expr` is evaluated against a `Context` built from its own `inputs`
table, through the one `predicate::evaluate` (ADR-0136 decision 1). `Tri::True` continues; `Tri::False` goes to the
step's `on_false`; `Tri::Null` keeps ADR-0103 and AC-079; an `EvalError` refuses by name and is never read as false.

**7. `emit` binding.** The `emit` step binds each key of its `artifacts` table to the provided kind and its
`evidence` list to the evidence refs; that binding is what commits the activation. A step after an `emit` is refused
by name at load, never dropped.

**8. `artifact.read` has one caller and one emitter.** Every resolved `artifact:` input makes exactly one call to
`runtime::reference::read_artifact_input`, which serves the journalled binding and calls an `ArtifactReadSeam`. The
seam's one production value at `s8d` is a named, non-emitting stub; `s8f`'s rk-store seam is the only emitter of
`artifact.read` (D21-47). The row's members, besides design §5's, include `stage`, which `doctor`'s `unread` rule
groups by.

**9. Test selection, the resolver side (D21-42 4c).** `"affected"` resolves to the item's committed `Changeset`
paths, handed to `s8c`'s package-level mapping; a `${…}` or `artifact:ReproCase.command` selection is resolved and
then parsed into a test list by **one** `selection_of`, which moves into `rk-kernel` and is re-exported from
`rk_cli::review_pass` with `pub use`. An unresolvable selection refuses by name and never reaches the port as an
empty selection.

*(Amended 2026-09-27, `s8d` build, package F3-D2174, on `spikes/DECISIONS-2026-09-21.md` D21-74 ruling 2: *"R2
(ADR-0173, `proposed`) decision 9 is amended: inside the producer of `Changeset`, `"affected"` resolves over the run
branch's diff (R6's `HEAD..rk-run/<item>` range, read through `rk_git`), and elsewhere as before, from the committed
`Changeset`."* This record is still `proposed`.)* **9a. Inside the producer of `Changeset`, `"affected"` is the run
branch's diff.**
- *The producer* is an activation one of whose `emit` steps binds `Changeset` (`Scope::emits`). Every shipped
  activity's `emit` binds exactly its `provides`
  (`emit_binding::every_shipped_activity_has_one_last_emit_whose_artifacts_equal_provides`), so this is the
  activation of an activity that provides `Changeset`. It is not only the one step 4 chose for the kind. On the story
  lane `docs-update` wins `Changeset`, and `tdd-red-green-refactor` still emits a `Changeset` binding, so it reads the
  branch too.
- Inside it, `"affected"` is the paths `HEAD..refs/heads/rk-run/<item>` changes, in first-seen order. A `Changeset`
  that an earlier activation committed is not read there, because it is not this activation's. Outside it,
  `"affected"` is the committed `Changeset`'s `files`, unchanged.
- **The read is the composition root's.** `rk-kernel` opens no files, so the kernel declares a seam,
  `runtime::reference::RunBranch`, bound with `ToolRouting::with_run_branch` and read through `Env::run_branch`.
  `rk-cli` binds `change_deriver::RunBranchDiff`, which reads R6's range through `change_deriver::read_range`. That
  is R6's one read, now shared by the `change` deriver and this selection, through `rk-git`'s `GitOp::RevParse` and
  `GitOp::Log`. It makes no read-seam call, because no `artifact:` input is resolved.
- Each of these refuses by name, never reaches the port and is never an empty selection: no reader bound (*"no
  run-branch reader is bound"*), an empty range (*"carries no change"*), and an unreadable range (the reader's error).
- Measured by `test_selection::inside_the_changeset_producer_affected_resolves_over_the_run_branch_diff`, `rk-cli`
  `change_deriver::the_run_branch_reader_reads_the_items_changeset_paths_through_rk_git`, and the two `lane_walk`
  tests over the rig's git fixture. **The call site in `verbs::run::run` is not observable at `s8d`**: mutant MF3j,
  which removes it, survived all 140 `rk-cli` tests. It is owed to `s8e` §8e.11's first `run()` tests by a receiver
  note.

**10. The carrier.** `catalog::Step` carries `inputs` (`Step::inputs`, each value a source string or a scalar
literal; an array, a table or a float is refused at load), and the three keys the licence reads: `on_false`'s `to`,
`repeat`'s `until` and `escalate`'s `question`. What may read `inputs` is routed here: the licence lint (decision
2), AC-303's confirm rule, which D21-57 moves onto this carrier (`rigor::confirm_step_monotonicity_rule` now reads
`Step::inputs` and no raw TOML), and the resolver (decision 1).

**11. What is superseded, each in part and quoted.**
- **ADR-0042's Decision**, *"A ref to a later-declared step is legal iff that step or one after it has
  `on_false`/`repeat` targeting the referencing step"*, as to the licence: the escalate re-entry (decision 2) also
  licenses a ref, and a ref must carry `?`. Its second half, *"it is `null` on the first pass"*, stands. Its first
  forbids bullet, the Decision's converse (*"Forbids a ref to a later-declared step where no step at or after the
  target has an `on_false`/`repeat` back-edge pointing at the referencing step"*), is moved in the same part, so no
  forbids bullet contradicts the lint. The Decision says *"This goes into §1 "Edge" and lint rule 1"*; both sites are
  read with the widening, and the schema text is not edited. ADR-0042 stays `accepted`, and its `superseded_by`
  gains `0173`.
- **ADR-0148 decision 1** (*"`toolchain:test` refuses"*) and **decision 4** (*"This is reversed by a resolver, not
  by a revert"*): the resolver reverses the refusal for a selection it resolves (decision 9). **Decision 3**
  (*"`catalog::Step` is not widened to parse `inputs`"*): the carrier widens it (decision 10), and this record is the
  record that routes what may read it, which ADR-0148's fourth forbids bullet asks for. ADR-0148's other forbids
  stand: no empty selection from the executor, and no declared selection string passed through as a test selector.
  ADR-0148 stays `accepted`, and its `superseded_by` gains `0173`.

**12. What is discharged, not superseded.** ADR-0136 clause 4 and ADR-0142 clause 4, each of which names this
resolver as owed. ADR-0136's forbids all stand.

## Consequences

**Positive**

- A `check` step decides from resolved values; a `toolchain:test` step reaches the port with a real selection; and
  a lane activity can complete a committing walk once its other producers exist.
- A forward ref is decided at load. The census is read from inside the test binary
  (`forward_ref_lint::every_shipped_forward_ref_is_licensed_and_the_census_is_printed`) and zero are unlicensed.
- AC-303's confirm rule reads the same parsed `inputs` the lint and the resolver read, not a second parse.

**Negative**

- A third licence makes the forward-ref rule harder to state than ADR-0042's one sentence, and the schema text
  still reads the narrower rule until someone edits it.
- The run record's per-step transcription now carries each step's `inputs`, `on_false` target, `repeat.until` and
  `escalate` question where declared, so a recorded `specs.json` is larger than before for a step that declares them.
- An escalate-gate ref reads a previous frame; a reader must know which cycle made the frame.

**What this forbids**

- Forbids a partial resolver: a form of decision 1's table that neither resolves nor refuses by name.
- Forbids reading an `UnresolvedRef` as `Tri::Null`, `Tri::False` or a skip, and forbids a refusal that does not
  name the step id and the reference.
- Forbids admitting a forward ref at load that no licence of decision 2 covers, or that carries no `?`, and forbids a
  gate step with another question licensing an escalate re-entry.
- Forbids binding a licensed forward ref to the current frame's value.
- Forbids flattening a fan-out's output anywhere but the `[*]` projection.
- Forbids a second `artifact.read` emitter, and forbids the kernel function appending the row.
- Forbids a second selection parser, and forbids an unresolvable selection reaching the port as an empty selection.
- *(Dated 2026-09-27, F3-D2174; decision 9a.)* Forbids reading `"affected"` inside `Changeset`'s producer from a
  committed `Changeset`, forbids `rk-kernel` spawning `git` or reading the repository for it, and forbids a second
  reader of the item's changeset range beside R6's.
- Forbids binding an absent `${…?}` by rewriting the expression into the literal `null` (ADR-0172 clause 3).

## Evidence and confidence

- [documented] D21-11, D21-41 (d), D21-43 N12–N14, D21-47, D21-48 X5, X6 and X8, D21-57 and D21-60, each quoted in
  `source` or above; ADR-0042, 0136, 0142 and 0148, quoted in Decision clause 11 and Context.
- [measured 2026-09-26 at `s8d`'s build, from inside the test binary] before the catalog fix,
  `FORWARD_REFS total=80 letter=63 repeat=3 escalate=10 unlicensed=4`, the four being exactly §8d.3's table, and
  `conformance::shipped_pack_loads` refused the shipped pack through `load_lints` naming those four; after it,
  `FORWARD_REFS total=76 letter=63 repeat=3 escalate=10 unlicensed=0`. The fixture test loads one fixture per
  licence and refuses five (an approval answer after the only loop, an escalate to another question, a ref past a
  repeat body, D59b's shape, and a licensed ref without `?`), each by activity, step and reference.
- [measured 2026-09-26 at `s8d`'s build] AC-303's confirm rule over the carrier: every `review::ac303_*` test green,
  zero violations over `.redkiln/catalog`.
- *(dated 2026-09-26, `s8d` build, package P4.)* [measured, through the no-dispatch helper] Decisions 1 and 6–9 are
  built in `crates/rk-kernel/src/runtime/reference.rs` and the walk in `runtime/step.rs`, with 19 tests
  (`reference_resolver::` ×9 of this record's forms, `check_eval::` ×3, `emit_binding::` ×3, `test_selection::` ×3 and
  the renamed `guard_context::` test), each red against the code with its block removed, and mutants M5, M9, M14,
  M15, M19 (a–c), M21, M23, M25 and M29 killed (`s8d` §8d.10). Readings taken beside this record's text, each stated in
  `s8d` §8d.3: a `check`'s `Context` is the selection's context with its own `inputs` bound OVER it (decision 6 says
  *"its own `inputs` table"*; the facet context is kept so a `check` over `rigor` still decides), and a `null` input
  binds `null` at each field path its `expr` reads; `Tri::False` refuses by name until R3 routes `on_false`; the
  completion row carries the `emit` binding as two members, `artifacts` and `evidence`, and `artifact:` reads the
  latest completion row binding the kind; an `agent` step's resolved inputs join its spec at dispatch and never
  overwrite a binder key; a tool step's input refusal follows ADR-0170 decision 3's registry lookup; `s8c`'s interim
  verdicts (`build_verdict`, the `SmokeResult` arm) give way to the catalog's `check` only where a later `check`
  reads the step (N25), and stand elsewhere. The iteration forms of decision 1 (`for_each`, `repeat`, a loop-back's
  previous frame) have their resolver half only: the scope's `bind_previous` / `set_element` entries, which the loop
  executor's package binds.
- *(dated 2026-09-26, `s8d` build, package P5.)* [measured, through the no-dispatch helper] **Decisions 4 and 5, and
  decision 1's iteration rows, are built** — the loop executor in `runtime/step.rs`'s walk, with `s8d`'s record R3
  (`0175`, `proposed`). The P4 bullet's *"have their resolver half only"* is superseded by this line. Six
  `reference_resolver::` tests (the self-reference, the feedback ref, the repeat body over a later step, `for_each`
  with the `[*]` projection over lists of different lengths, `repeat` and its three `on_exhausted` exits, and the
  escalate re-entry) were each first shown RED on the pre-P5 walk, then GREEN; mutants M6, M24, M26 and M35 are
  killed (`s8d` §8d.10). Readings taken beside this record's text, each stated in `s8d` §8d.3: `for_each` runs one
  copy of the STEP per element (decision 5's *"one activation per element"* is read as the schema's *"one copy per
  element"*: one step activation, each under its own `for_each:<i>` frame, `i` 0-based), and the step's output is
  the list of the copies' outputs in element order; a `for_each` source that resolves to an empty list, or to
  anything but a list, and a `for_each` on a step that is neither `agent` nor `tool`, each refuse by name; a
  `repeat` pass is `repeat:<n>` (`n` from 1), `until` is evaluated with every body step's output at the CURRENT pass,
  and an `until` that is `null` does not hold (the step re-runs while `until` is not `Tri::True`); a `repeat` with
  no `until` refuses by name, and an absent `on_exhausted` refuses as `fail` does; exhaustion with `escalate`
  proceeds IN DECLARATION ORDER from the body's end, so the steps between the body and the gate run before it (the
  reading D21-48 X8's escalate licence presupposes for `minimal-repro-ue`'s `cook` and `run`); a later gate's
  `.answer`, like a later step's `.outputs`, reads the previous frame and is `null` first; and the repeat body is ONE
  function, `catalog::repeat_body_end`, read by the licence lint and the walk alike.
- **What this evidence does not license.** At the time this record is written the forward-ref licence, the carrier
  and AC-303's move are built; decisions 1 and 3–9 (the resolver, iteration, `check` and `emit`, the read seam and
  test selection) are this record's decisions for `s8d`'s later packages to build, and each is claimed only by the
  test §8d.10 names for it. This record flips at `s8d`'s close under D21-64 ruling 10, on its own dated ledger line,
  only if the formal Checkpoint 8d passes whole and a refuting round finds no blocker or major.
