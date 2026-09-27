---
id: "0171"
title: "**A `tool` step's declared `timeout_secs` is the tighter bound.** `catalog::Step` gains `timeout_secs` (a literal or a `${toolchain.<key>}`), the kernel resolves it through ADR-0170's value table after the registry lookup and the route and input checks, an absent or `unmeasured` key refuses by name and never falls back, and every `Toolchain` method a `tool` step reaches takes a `StepBound` that the adapter enforces as `min(step, construction)` on every child it spawns. Supersedes, each in part and quoted and each for `tool` steps (D21-48 X13), `0140` decision 1's withdrawal, decision 2's sentence, decision 4 and its first two forbids bullets, and `0099` decision 2's *\"under the same timeout\"*. `0140` decision 5 stands here; `s8d`'s R1 supersedes it in part for per-step specs"
status: accepted
date: 2026-09-26
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 decision 4b (*"`min(step, construction)` is passed into the port call. Supersedes ADR-0140 c1 in part, and the 298 declarations become real."*), D21-43 N16 and D21-48 X13 (the supersession's breadth); *(dated 2026-09-26, s8c refuting round 5 minor)* D21-64 ruling 9 (this record's flip condition, *"Records T, V and B (ADR-0169, 0170, 0171) flip at `s8c`'s close if the formal Checkpoint 8c passes whole and a refuting round finds no blocker or major"*, the condition ADR-0169 and ADR-0170 cite); runbook/phase-1/s8c-toolchain-port-and-verbs.md §8c.6 and §8c.7 (the proposal this record fixes); references/adr/0140 and references/adr/0099 decision 2 (the sentences superseded in part); references/adr/0137 clause 5 (the clause ADR-0140 decision 1 withdrew); crates/rk-kernel/src/catalog.rs, crates/rk-cli/src/config.rs and crates/rk-cli/src/verbs/run.rs at `6f70e47`
supersedes: ["0140", "0099"]
superseded_by: ["0174"]
related: ["0137", "0139", "0148", "0169", "0170"]
evidence: []
consumers: []
---

## Context and problem

ADR-0140 recorded, correctly at the time, that nothing reads a step's `timeout_secs`: `catalog::Step` carried no
timeout field, the loader documented *"The loader reads no `timeout_secs`"*, and the shipped catalog's **298**
declarations (204 literal, 94 `${toolchain.<key>}` substitutions) reached no executor. So it made the adapters'
construction-time `timeout` *"THE bound on every child a toolchain adapter spawns"*, and recorded the declarations as
*"UNREAD"*, a gap it opened and left OWED.

D21-42 4b closed that gap for the `Toolchain` port: *"**Yes, as the tighter bound**: `min(step, construction)` is
passed into the port call. Supersedes ADR-0140 c1 in part, and the 298 declarations become real."* D21-48 X13 took the
supersession's breadth, because 4b named decision 1 alone and five sites in two records say the opposite of 4b:
*"**Record B supersedes, each in part and each quoted:** ADR-0140 decision 1, decision 2's sentence, decision 4, the
first two forbids bullets, and ADR-0099 decision 2's *"under the same timeout"*."*

**Through this port 4b reaches 113 of the 298 declarations**: the ones on `tool` steps (52 literal, 61 substituted).
The other **185** sit on `agent` steps, whose bound belongs to the `Harness`, not the `Toolchain` port. D21-43 N16
settles them: *"**Carried by each per-step spec** (3a), resolved through 4a's table and bounded by the activity budget.
Owner: s8d."*

## Decision drivers

- A silent default is the failure ADR-0140 was written about. A key that does not resolve must refuse, not fall back
  to the construction bound.
- A per-call bound that is not the construction bound is **not** *"the same timeout"*. ADR-0099 decision 2's sentence
  is false once a step's bound reaches the port, so it moves by record, not by prose.
- The construction bound must still win when a step declares more: a declared value may exceed it (ADR-0170 decision
  10 does not clamp), and a bound that loosened the construction cap would widen every child's window silently.
- A journal must be able to tell which bound fired.
- ADR-0140 decision 5 forbids building a per-step bound by overloading `ActivitySpec.timeout_secs`; this record builds
  one without it.

## Considered options

### Option — Carry the step's bound into each port call as `min(step, construction)` (taken)

D21-42 4b's route, with the shape §8c.6 proposed. Taken.

### Option — Enforce the step bound in the kernel, around the port call

Rejected. The kernel holds no child process: it cannot kill what the adapter spawned, and a wall-clock guard around a
blocking call returns while the child keeps running. The adapter already enforces the construction bound on every
child, and the tighter bound belongs at the same site.

### Option — Replace the construction bound with the step bound

Rejected. A step that declares nothing, or declares more than the construction bound, would then run under a bound no
record measured. ADR-0140 decision 3's 3600 s default and its optional key stand, re-grounded, and remain the cap.

### Option — Supersede ADR-0140 decision 1 alone, as D21-42 4b listed it

Rejected by X13. Decisions 2 and 4 and the first two forbids bullets would then stand beside this record and forbid
what it does.

## Decision

**1. `catalog::Step` gains `timeout_secs: Option<StepTimeout>`**, which is `Literal(u64)` or `Toolchain(key)`. The
loader reads it. `inputs` is **not** parsed here, and ADR-0148's forbids bullet on `inputs` stands until `s8d`.

**2. The kernel resolves `Toolchain(key)` through the `ToolchainValues` `rk-cli` supplies (ADR-0170 decision 9)**,
after the registry lookup and the route and input checks (ADR-0170 decision 3's order). **An absent key is refused by
name, and so is an `unmeasured` one**, as `StepError::Unsupported` whose `capability` names `${toolchain.<key>}` and
says why. **It never falls back to the construction bound.**

**3. Every `Toolchain` method a `tool` step reaches takes a `StepBound`**: `build`, `test`, `lint`, `change_summary`
and `run_verb`. `detect` does not (it is adapter selection at construction, ADR-0137 clause 2, not a step), and
`classify` does not (it spawns nothing). `StepBound` is `Undeclared` or `Secs(n)`, and `n = 0` is refused (RT-5: *"a
zero bound is no bound"*).

**4. The adapter enforces `min(step, construction)` on every child it spawns for the call.** `Timeout { after_secs }`
carries **the bound that fired**, so a journal can tell a step's own bound from the construction bound. An
`Undeclared` bound leaves the construction bound alone. ADR-0140 decision 3's 3600 s default and its optional
`[adapters] toolchain_timeout_secs` key stand.

**5. The agent steps are not this record's.** `ActivitySpec.timeout_secs` is untouched. This record gives the 33
substituted agent-step keys their resolution through ADR-0170's table; **enforcing** the 185 agent-step declarations
is `s8d`'s (N16).

**6. `rk-cli`'s two docstrings are corrected in the commit that lands this.** `config.rs` and `run.rs` describe the
construction timeout as the only bound, as ADR-0140 decision 2 made them say; this record's supersession licenses the
change.

**7. What this record supersedes, each in part (ADR-0092 rule 1), and each for `tool` steps only.**

- **ADR-0140 decision 1**, its withdrawal of ADR-0137 clause 5's *"while a step's own `timeout_secs` remains the
  kernel's"*, which it says *"is **withdrawn as false**"*. For a `tool` step that clause is true again under decisions
  1–4. The withdrawal stands for `agent` steps until `s8d`'s per-step specs carry their bound, and the rest of decision
  1 (the withdrawal of *"The backstop may not be smaller than the largest step it could serve …"* as reasoning) stands:
  the construction bound caps a step's bound, it does not have to exceed it.
- **ADR-0140 decision 2's sentence** *"It is not a backstop, because there is nothing behind it"*. A resolved step
  bound now sits inside it. The rest of decision 2, that the construction bound applies to every child and that
  lowering it shortens every child's window, stands.
- **ADR-0140 decision 4**, *"The catalog's `timeout_secs` declarations are recorded as UNREAD"*. The loader reads them,
  and its OWED item is **discharged for tool steps** (D21-42 4b). The 94 substitutions' value source is ADR-0170's
  table (D21-42 4a).
- **ADR-0140's first two forbids bullets**, *"Forbids describing the construction-time `timeout` as a *backstop*, a
  *ceiling behind* another bound, or an *outer fence*"* and *"Forbids any prose, docstring or record asserting that a
  step's declared `timeout_secs` is applied, enforced or honoured by the kernel or by a toolchain adapter"*. For a
  `tool` step whose declared bound resolves, the construction bound is the cap behind it, and the adapter enforces the
  declared bound.
- **ADR-0099 decision 2**, *"the two Toolchain adapters run bounded commands under the same timeout"*, as to *"under
  the same timeout"*: a per-call step bound is not the same timeout. Its no-token half, *"without a token"*, stands:
  only the Harness port carries cancellation in Phase 1.

**What stands.** ADR-0140 decision 3 (the 3600 s default and its optional key); **decision 5**, *"`ActivitySpec.timeout_secs`
is untouched … A future per-step bound may not be built by overloading it"*, which stands **here** and which `s8d`'s
R1 supersedes in part for per-step specs (*"A future per-step bound may not be built by overloading it"*, and its
forbids bullet *"forbids repurposing it as one"*), so that the two records read as one; its bullets against conflating
`ActivitySpec.timeout_secs` with a step timeout or repurposing it, against lowering the default below the pack's
largest declared literal without a record, and against resolving a `${toolchain.<key>}` without a record naming the
table (which ADR-0170 now names). ADR-0099 decision 2's no-token half stands, and so does the rest of ADR-0099 as
ADR-0115, ADR-0127 and ADR-0165 left it.

## Consequences

The 113 `tool`-step declarations become real bounds, each either a literal, a resolved key, or a refusal by name. A
`tool` step that declares 120 s is cut off at 120 s, and the journal says the step's bound fired. An `unmeasured` key
refuses at `run` before the port, and is an `Error` at `doctor` (ADR-0170 decision 12). The 185 agent-step
declarations have values but no enforcement until `s8d`.

**What this forbids**

- Falling back to the construction bound, or to any default, for an absent or `unmeasured` `${toolchain.*}` key.
- Resolving a step's bound before the registry lookup and the route and input checks.
- Enforcing `max(step, construction)`, or any step bound that loosens the construction bound.
- A zero step bound.
- A `Timeout` that does not carry the bound that fired.
- Enforcing an agent step's `timeout_secs` through `ActivitySpec.timeout_secs`, or through this record's `StepBound`.
- Describing the construction bound as the only bound on a `tool` step whose declared bound resolves, or describing a
  step's declared bound as unread for such a step.
- Passing a `StepBound` to `detect` or `classify`.

## Evidence and confidence

- [measured at `a6349b6`, §8c.6] `.redkiln/catalog/`: **298** `timeout_secs` declarations, **113** on `tool` steps (52
  literal, 61 substituted) and **185** on `agent` steps (152 literal, 33 substituted). All 204 `timeout_secs` in
  `.redkiln/packs/` are integer literals; all 94 substitutions sit on catalog steps.
- [documented] ADR-0140 decisions 1–5 and its forbids bullets, and ADR-0099 decision 2, quoted above from their
  accepted bodies at `6f70e47`.
- [documented] `crates/rk-kernel/src/catalog.rs`'s loader doc at `6f70e47`: the loader reads no `timeout_secs`, which is
  what decision 1 changes.
- No measurement bears on the bound's value: the values are ADR-0170's, each with its warrant.
- *(dated 2026-09-26, s8c build, CPREP.)* **The build's readings, stated here as proposed text** (first written in
  `runbook/phase-1/s8c-toolchain-port-and-verbs.md` §8c.6's dated notes):
  - **The step side.** The loader reads `timeout_secs` through `StepTimeout::from_value`: a non-negative integer, or
    a string of the exact form `${toolchain.<[a-z0-9_]+>}`; anything else is refused as a `Shape` naming the step,
    never dropped. `runtime::step::resolve_bound` makes the `StepBound`: no declaration is `Undeclared`, a literal
    goes through `StepBound::secs` (zero refuses naming RT-5), and a key resolves through `ToolchainValues`.
  - **A table-less key refuses too.** `ToolchainValues::unselected(reason)` is the case where no table can be
    selected; under it every key refuses as `Unsupported` naming `${toolchain.<key>}` and the reason, as an absent or
    `unmeasured` key does (decision 2). It never falls back.
  - **The order.** `run_tool_step` runs the registry lookup, ADR-0148's `test` sentence, the route match and its
    input check, and only then `resolve_bound`. A runtime bound to no registry refuses every verb by its bare name,
    so it never reaches the bound (record V's evidence).
    `tool_verb_routing::a_verb_with_no_row_refuses_by_its_bare_name_before_its_bound_is_resolved` pins the order.
  - **Enforcement, decision 4.** `Config::bounded(StepBound)` returns the call's `Config` with `timeout = min(step,
    construction)` (an `Undeclared` bound leaves it alone) on both adapters, and every child the call spawns runs
    under it: `build`, `test` and its `--list`, run, doctest and metadata children, `lint`, and each verb handler's.
    Every `Timeout { after_secs }` the call answers carries that value. `change_summary` spawns nothing.
  - **Decision 6, done.** `crates/rk-cli/src/config.rs` (`Adapters::toolchain_timeout_secs`) and
    `crates/rk-cli/src/verbs/run.rs` (`DEFAULT_TOOLCHAIN_TIMEOUT_SECS`) each carry a dated correction citing this
    record (proposed). The `s7g` paragraphs are left as written beneath it, and they stand for `agent` steps until
    `s8d`.

**Confidence.** High. The shape is forced by D21-42 4b and the breadth by D21-48 X13; the only proposal here is the
`StepBound` type and the `Timeout` payload, both of which the tests in §8c.9 pin (`bound::…`, `step_bound::…`).
