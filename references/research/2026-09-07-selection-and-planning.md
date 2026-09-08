# Deterministic activity selection: prior art and a recommended selection model

Date: 2026-09-07. Scope: how a deterministic engine should assemble the per-item activity DAG in redkiln-rs
given (a) an LLM classification of the work item, (b) a catalog of activities with typed inputs/outputs,
(c) artifacts that may already exist, and (d) an LLM allowed to *request* typed deviations but never to
*select*. Constraint inherited from the architecture proposal (`references/2026-09-05-architecture-proposal.md`
§4.2): "Rust holds the loop; the model holds no control flow"; weights per stage pick from an ordered option
list; activities are content-addressed memoised on `hash(activity id, resolved inputs, card digest, KB atoms,
toolchain version)`.

Tags: **[documented]** = claim traceable to the cited URL; **[inferred]** = my synthesis; **[measured]** only
where redkiln spike numbers (F-numbers) are cited.

---

## 1. Build-system / dataflow framing (Make, Bazel, Nix, Dagster)

**Mechanism.** A build rule declares *outputs it produces* and *inputs it needs*; the engine takes a set of
*requested targets*, walks the dependency graph backward, and runs only rules whose outputs are missing or
stale. Freshness is decided by content, not by intent:

- Bazel's action cache is *input-addressed* (`key = digest(action)` where the action digest covers command,
  env, and the digests of every declared input); the CAS is content-addressed (`key = digest(file)`). Hermeticity
  is enforced by sandboxing to *declared* inputs only. [documented] https://bazel.build/remote/caching ,
  https://www.buildbuddy.io/blog/bazels-remote-caching-and-remote-execution-explained/
- Nix store paths are, by default, a hash of the *derivation* (recipe + all inputs), i.e. input-addressed;
  `ca-derivations` switches to output-hash addressing. Anything not captured in the derivation silently breaks
  reproducibility ("the sandbox is a hidden input"). [documented] https://nix.dev/manual/nix/2.34/store/derivation/ ,
  https://www.tweag.io/blog/2021-02-17-derivation-outputs-and-output-paths/ ,
  https://fzakaria.com/2026/07/30/the-nix-sandbox-is-a-hidden-input
- Dagster's software-defined assets invert the task model: you declare *data that should exist* and the
  orchestrator decides when to materialise it. An asset is stale when its most recent materialisation predates
  an upstream's, or its code version changed. [documented] https://dagster.io/blog/software-defined-assets ,
  https://docs.dagster.io/api/dagster/assets
- Dagster **Declarative Automation** composes small boolean operands (`missing`, `in_progress`,
  `any_deps_updated`, `newly_true`, `since`, `all_deps_match`) into named policies: `eager` (run when any dep
  updates, but *not* if any upstream is missing), `on_missing` (fill in when all upstream partitions exist),
  `on_cron` (cron tick as a synchronisation barrier: all deps must update within the window). Asset *checks*
  get the same conditions, so a quality check is itself an automatable node. Freshness policies expose
  `freshness_passed / warned / failed` as operands. [documented]
  https://docs.dagster.io/guides/automate/declarative-automation ,
  https://docs.dagster.io/guides/automate/declarative-automation/customizing-automation-conditions/automation-condition-operands-and-operators ,
  https://docs.dagster.io/guides/automate/declarative-automation/automating-asset-checks ,
  https://github.com/dagster-io/dagster/discussions/32725

**Evidence.** Bazel/Nix have a decade-plus of production correctness resting on exactly two rules: declare all
inputs; key the cache on their digests. Dagster's own migration history (AutoMaterializePolicy → AutomationCondition,
GitHub discussion #23495) shows that a *closed set* of named policies was too rigid and they moved to a small
composable algebra — the same pressure redkiln will feel. [documented] https://github.com/dagster-io/dagster/discussions/23495

**How well does "the DAG is a build graph over artifacts; the rubric only decides which artifacts are required"
work?** Very well for the *ordering* and *short-circuit* halves of the problem, and it is already 80% present in
the proposal (typed outputs, content-addressed memo). [inferred] Concretely:

- *Applicability* collapses to `provides`/`requires`: an activity is a rule that materialises typed artifacts
  from typed artifacts. The engine does backward chaining from the **required artifact set** to a DAG; topological
  order is the execution order; parallel antichains fall out for free.
- *Existing artifacts* are handled exactly as Bazel handles a cache hit: an artifact whose digest is present and
  fresh is not rebuilt. "An initiative that already has a competitor analysis" = the `CompetitorAnalysis` node is
  materialised; its producer is pruned unless *stale* (an input it was derived from — e.g. the initiative charter
  — changed digest since).
- The rubric's job shrinks to one question: *which artifacts must exist for this item to advance from this stage?*
  That is a far smaller, more testable surface than "which activities run".

**Steal this for redkiln.**
1. Make the artifact catalog first-class: every typed output has an `ArtifactKind`, a schema, and a
   `staleness` rule (`derived_from` list). Activities are secondary — they are the rules.
2. Adopt Dagster's operand algebra for *freshness* only: `missing(A)`, `stale(A)`, `deps_updated(A)`, `newer_than(A,B)`.
   Do not adopt cron/eager; redkiln is pull-driven per stage.
3. Model *checks* as artifacts too (`ReviewFindings`, `VerifyResult`), so "must be reviewed" is "artifact
   `ReviewFindings@sha` must exist and be fresh w.r.t. `Change@sha`" — this also unifies the F27 policy verdict.
4. Nix's lesson: anything that influences an activity but is not in its cache key will produce silent wrong reuse.
   The card digest, injected KB atoms, toolchain version and *classification vector* must all be inputs.

**Risks.** (a) Human-authored artifacts have no producer rule; they must enter the graph as *source* nodes with a
digest, or the engine cannot reason about staleness. (b) Build systems assume a rule is pure; LLM activities are
not, so "same inputs → same output" holds only for memo reuse, never for correctness. (c) Over-eager staleness
(re-running a competitor analysis because one sentence of the charter changed) needs a *coarse* dependency
declaration (`derived_from: charter.sections.market`, not whole-file) or a human gate on re-materialisation.
[inferred]

---

## 2. Classical AI planning (STRIPS/PDDL, HTN/SHOP2, Datalog/ASP, SAT/Z3)

**STRIPS / PDDL + VAL.** Operators declare preconditions and add/delete effects; a planner searches for an action
sequence from initial state to goal; VAL symbolically simulates a candidate plan checking every precondition and
the final goal. [documented] https://www.emergentmind.com/topics/planbench (VAL as the correctness oracle in
PlanBench). In redkiln terms: `requires` = preconditions, `provides` = add-effects, required artifacts = goal.
Backward chaining (goal regression) from the goal set is exactly the build-graph walk above; classical forward
search is overkill because redkiln has no delete-effects worth speaking of (artifacts are not consumed). [inferred]

**HTN / SHOP2.** Instead of goals, SHOP2 takes a partially ordered *task network* and *methods* that decompose a
compound task into subtasks when the method's preconditions hold; methods are tried in order, first applicable
wins, giving an if-then-else reading that is deterministic and easy to explain ("method M fired because its
precondition P held"). Primitive tasks bottom out in operators. [documented] https://ipc02.icaps-conference.org/SHOP.html ,
https://www.cs.umd.edu/~nau/papers/nau2004applications.pdf , https://www.sciencedirect.com/science/article/pii/S0004370215000247
This is the closest structural match to stage → activity: a *stage* is a compound task, its *methods* are the
ordered option list the proposal already describes ("weights select from an ordered option list per stage"),
and an activity is a primitive. HTN's known weakness — the domain author must anticipate every decomposition —
is a *feature* here: it is precisely "the agent cannot hallucinate a strategy". [inferred]

**Datalog / Answer Set Programming.** ASP is used industrially for product configuration; incremental ASP can be
driven to "simulate the behaviour of a deterministic configuration algorithm" while retaining explanations via
preferred minimal conflict sets. [documented] https://arxiv.org/abs/2207.08599 , https://arxiv.org/pdf/2109.08304
Declare constraints (process-mining's declarative language, §6) have a direct ASP encoding. [documented]
https://arxiv.org/pdf/2412.10152 . Stratified Datalog (no negation cycles) has a unique least model — i.e. the
selection is deterministic by construction and the derivation tree *is* the explanation. [inferred, standard result]

**SAT over feature models / Z3.** Feature models reduce naturally to SAT; real-world models are easy for modern
solvers. [documented] https://arxiv.org/pdf/1506.05198 , https://dl.acm.org/doi/10.1145/3634713.3634733 . Strong
for *validating* a catalog (is every classification satisfiable? are two activities mutually exclusive?) but a
SAT solver returns *a* model, not a canonical one, so it should not be the runtime selector unless you add an
explicit objective or fix a tie-break. [inferred]

**Which is simplest with determinism + explainability?** Ranked for redkiln:
1. **HTN-style ordered methods + backward chaining over artifacts** — no solver, no search, a few hundred lines of
   Rust; the trace is a tree of "stage S, method M fired because [predicates], needed artifacts [..], of which
   [..] were present/fresh". This is what to build.
2. **Stratified Datalog** (e.g. `crepe`/`ascent` crates) if the applicability predicates become numerous and
   cross-cutting (toolchain × classification × stage × repo facts) — same determinism, better composability, and
   the rule set stays data. Adopt only when the Rust `match` tables get ugly.
3. **SAT/ASP** only offline: catalog linting in CI (satisfiability per classification, dead activities, cycles).
Full PDDL planners are unnecessary: no delete effects, no search, and the goal is fixed by the rubric. [inferred]

**Steal this.** (a) SHOP2's "first applicable method wins, methods are ordered" as the tie-break law; two
applicable methods at equal rank is a *config error*, matching the proposal's card-selector rule. (b) VAL's idea:
after assembly, *validate* the DAG mechanically — every `requires` satisfied by a `provides` or a present artifact,
no cycles, every required artifact reachable — and record the validation as an event. (c) ASP's "explanation =
minimal conflict set" when no method applies: emit *why not* (which predicate failed) instead of a bare error.

**Risks.** HTN methods drift into an unmaintainable if-else forest if predicates are free-form; keep the predicate
vocabulary closed and typed (see §7). Planning literature's determinism is w.r.t. a *state*; redkiln's state
includes LLM-classified fields, so determinism is conditional on the classification being frozen and logged.

---

## 3. LLM-Modulo, constrained decoding, plan validation

**LLM-Modulo (Kambhampati et al., ICML 2024).** Position: autoregressive LLMs cannot plan or self-verify, but are
"universal approximate knowledge sources"; the productive architecture is *LLM proposes, external model-based
verifier(s) critique*, in a tight bidirectional loop, with the sound verifier holding the guarantee. [documented]
https://proceedings.mlr.press/v235/kambhampati24a.html , https://nyaspubs.onlinelibrary.wiley.com/doi/abs/10.1111/nyas.15125 ,
travel-planning case study https://arxiv.org/pdf/2405.20625 . PlanBench follow-up: o1-preview reached 97.8% on
plain Blocksworld but performance collapsed on obfuscated ("Mystery") variants and long plans, with VAL as the
oracle. [documented] https://www.semanticscholar.org/paper/5329cea2b868ce408163420e6af7e9bd00a1940c ,
https://www.emergentmind.com/topics/planbench . Implication: the model's *knowledge* (this is a data-migration
story; it touches auth) is valuable; its *control flow* is not trustworthy; and even LRM gains evaporate off
distribution. This is the same conclusion as redkiln's F27 (review detection stable, threshold judgement not).

**Constrained decoding / classifier-then-lookup.** JSON-schema `enum` constraints mask logits so the model can only
emit catalog members; constrained generation eliminates format errors. [documented]
https://arxiv.org/html/2501.10868v1 , https://docs.vllm.ai/en/v0.8.2/features/structured_outputs.html . Caveat
studies: format restriction can degrade reasoning quality ("Let Me Speak Freely?") [documented]
https://arxiv.org/pdf/2408.02442 , and grammars are a control-plane attack surface [documented]
https://arxiv.org/pdf/2503.24191 . Pattern for redkiln: let the model reason freely in a scratch field, then emit a
*closed* classification record (schema with enums and bounded integers); the engine does the lookup. This is
Anthropic's own "routing" workflow pattern: classify, then dispatch on predefined code paths, preferring the
simplest solution. [documented] https://www.agentsdecoded.com/p/an-analysis-of-anthropics-guide-to (secondary;
primary is anthropic.com/research/building-effective-agents).

**Plan validation of LLM output.** 2025 work converts NL plans to Kripke structures + LTL and model-checks them
(F1 96.3% with GPT-5 as formaliser) [documented] https://arxiv.org/abs/2510.03469 ; VeriPlan (CHI 2025) extracts
user rules, verifies with a model checker, and loops with the human [documented]
https://dl.acm.org/doi/10.1145/3706598.3714113 ; AgentLTL enforces procedural compliance on tool-use traces
[documented] https://arxiv.org/pdf/2607.02599 ; Lean4Agent formalises workflow + trajectory [documented]
https://arxiv.org/pdf/2606.06523 . These all *check a plan the LLM made*. redkiln's constraint is stronger and
simpler: the LLM never makes the plan, so validation targets (i) the classification record and (ii) deviation
requests — both small, typed, and checkable against the catalog without a model checker.

**Steal this.** (1) `Classify` is an activity whose output schema is closed (enums + integers + evidence quotes);
the engine treats it as an *input fact*, logged and digested. (2) `RequestDeviation { stage, method_or_activity,
predicate_override, why }` must name a catalog member and a predicate it claims is true; the engine re-evaluates the
predicate against evidence, and only a human can accept an override that the evidence does not support. (3) Keep a
*critic* slot: a second cheap model call that checks the classification against the item text (LLM-Modulo's
"LLM as critic" role) — cheap, and F27 suggests detection tasks are where models are stable.

**Risks.** Constrained decoding can make the model confidently pick the nearest enum; include an explicit
`unknown`/`needs_human` value for every enum and make the engine route `unknown` to a gate rather than a default.
Free-text `why` fields are untrusted input (tracker inbound rule already says so).

---

## 4. Automated workflow generation (AFlow, ADAS, GPTSwarm, FlowMind, AWM, MetaGPT, ChatDev, Dynamic Workflows)

**What they are.** AFlow (ICLR 2025 oral) searches code-represented workflows with MCTS: +5.7% avg over baselines
on six benchmarks; lets smaller models beat GPT-4o at 4.55% of the cost on some tasks. [documented]
https://arxiv.org/abs/2410.10762 , https://github.com/FoundationAgents/AFlow . ADAS / Meta Agent Search has a meta
agent program new agents into an archive; claims transfer across domains/models. [documented]
https://arxiv.org/abs/2408.08435 . Agent Workflow Memory induces reusable sub-routines from past trajectories,
offline or online. [documented] https://arxiv.org/abs/2409.07429 . MetaGPT encodes SOPs as a fixed role pipeline
producing standardised deliverables and reports reduced hallucination vs unstructured chat; ChatDev uses a chat
chain of role dialogues. [documented] https://arxiv.org/pdf/2308.00352 , https://arxiv.org/html/2307.07924v5 .
GPTSwarm treats agents as an optimisable graph. Cost: ChatDev/MetaGPT-style teams can exceed $10 per HumanEval
task; MetaGPT ~126 tokens/LoC vs ChatDev ~249. [documented, secondary] https://chierhu.medium.com/the-engineering-of-multi-agent-collaboration-systems-a-first-principles-technical-report-ca6b3556dcd3

**What they found (the part that matters).**
- Hand-designed SOPs (MetaGPT) beat free chat on *stability* — the wins come from fixed roles and typed
  deliverables, not from agent count. [documented] https://arxiv.org/pdf/2308.00352
- Generated workflows beat hand-designed ones only on *benchmark-shaped* tasks with a scorer to optimise against;
  the search itself costs many rollouts. [documented] AFlow above.
- "The Illusion of Multi-Agent Advantage" (June 2026): automatically generated MAS vs single-agent CoT-SC —
  MAS advantage claims rest on benchmarks that reward isolated reasoning; automated frameworks incur ~10x the cost
  for negligible gains. [documented] https://arxiv.org/abs/2606.13003
- Google Research scaling study cited in the proposal: sequential multi-agent −39…−70%, parallel +80.9% — i.e.
  orchestration pays only for genuinely parallel work. [documented via proposal §4 principle 7]

**Claude Code Dynamic Workflows (announced 28 May 2026, GA by June).** Claude writes JavaScript orchestration
scripts that fan out tens-to-hundreds of subagents, with "results checked before they're folded in", adversarial
refutation agents, iteration until convergence, checkpointed progress, and an admin kill-switch; Anthropic warns it
consumes "substantially more tokens". Showcase: the Bun Zig→Rust port (~750k lines of Rust, 99.8% of tests, 11
days, "two reviewers on each file", "not yet in production"). [documented]
https://claude.com/blog/introducing-dynamic-workflows-in-claude-code , https://www.infoq.com/news/2026/06/dynamic-workflows-claude-code/
Read as a competitor: it is *emergent* orchestration — the model chooses the plan and the topology per run. Its
sweet spot is embarrassingly parallel, test-suite-oracled work (migrations, audits). It has no notion of a
process catalog, stage gates, per-item appetite, or reproducible selection; two runs of the same prompt may produce
different scripts. [inferred from the announcement's own description] Its strengths redkiln should *not* try to
replicate: massive fan-out. Its gaps are redkiln's thesis: deterministic, auditable selection with human gates as
durable marks.

**Steal this.** (a) MetaGPT's "typed deliverable per role" is already redkiln's typed-output rule; keep it. (b)
AWM's *induced sub-routine* idea is safe if the induced thing is a *proposal for a new catalog entry* reviewed by a
human, never a live method. (c) Dynamic Workflows' "independent attempts + refuter" is a legitimate *activity
shape* for the verify/review stage (a parallel review panel) — put it in the catalog as an activity with a
parallelism parameter, selected by the engine when criticality is high, rather than letting the model decide to
spawn it. (d) Its checkpoint/resume claim is table stakes; redkiln already has lossless suspension (F19).

**Risks.** Chasing generated workflows would re-import the non-determinism the architecture just removed. The
honest competitive risk is that for large mechanical migrations, Dynamic Workflows will simply be faster; redkiln
should position on *traceable process for work that must be governed*, and could later add a "dynamic fan-out"
technology-pack activity that shells out to it.

---

## 5. Rubric / scoring approaches

**Weighted multi-criteria (WSM/AHP).** AHP suffers *rank reversal*: adding/removing an alternative can reorder the
rest; the same pathology appears in SAW and TOPSIS; multiplicative AHP / weighted geometric mean are proposed
fixes. [documented] https://link.springer.com/article/10.1007/s10479-023-05278-6 ,
https://en.wikipedia.org/wiki/Rank_reversals_in_decision-making . For redkiln this means: if the rubric produces
a *ranking* of activities from summed weights, adding an activity to the catalog can silently change what gets
selected for unrelated items. [inferred]

**Boehm–Turner polar chart.** Five axes (size, criticality, personnel, dynamism, culture) rated per project; the
closer to the centre, the more agile; the further out, the more plan-driven; explicitly a *risk-assessment* of
each axis followed by a mixed method. [documented] https://www.fritz.tips/wp-content/uploads/2016/09/BeohmAndTurner_UsingRiskToBalanceAgileAndPlan-DrivenMethods.pdf ,
https://www.cs3240.org/modules/process/the-polar-chart/ . The important design feature: each axis is *ordinal and
independently interpretable*; no axis is summed with another. That maps cleanly to a classification record with
a handful of ordinal fields, each of which gates specific activities.

**Risk-based test selection / Definition of Ready.** Risk-based selection multiplies likelihood × impact per
area to prioritise coverage; DoR is a boolean checklist that a story must satisfy before it is taken into a
sprint, explicitly "a working agreement, not a strict set of rules". [documented]
https://www.boost.co.nz/blog/2022/07/definition-of-ready-agile-user-stories/ ,
https://www.agileambition.com/Essays/Definition-Of-Ready . DoR is a *required-artifact set by another name*:
"AC written, dependencies known, sized" = artifacts must exist. [inferred]

**Numeric weights vs boolean rules — what the evidence supports.**
- Numeric summed scores are compact but opaque and unstable under catalog change (rank reversal); they answer
  "how much" when the question is usually "whether". [documented above]
- Ordinal *thresholds* per axis ("criticality ≥ high ⇒ ReviewPanel required") keep the numbers but avoid
  summation, so each selection has a one-line explanation. Boehm–Turner is exactly this. [inferred]
- Boolean rules over typed predicates are the most explainable and the most testable (each rule is a unit test);
  their failure mode is combinatorial growth, mitigated by ordering (HTN) or stratification (Datalog). [inferred]
- The proposal's `stages = { triage = 0.2, investigation = 0.5, ... }` weights are best read as *ordinal
  option-list indices* ("0 = skip, 0.5 = light, 1.0 = full"), not as additive scores. Keep them ordinal; never sum.

**Steal this.** Classification = a Boehm–Turner-style vector of ~5–7 *ordinal* axes plus a few booleans
(`has_ui`, `touches_auth`, `data_migration`, `external_dependency`). Rules are of the form
`axis >= threshold  →  artifact required`. A "rubric" is then a table, versioned in the process pack, and every
selection cites the rows that fired.

**Risks.** Ordinal axes need calibration anchors (what is "criticality: high"?) or the classifier drifts across
models; fix by giving each level a written anchor in the pack and testing classification stability across
models/temperatures (spike E1 below).

---

## 6. Process mining / conformance checking as the *evaluation* method

**Mechanism.** Given a process model and an event log, conformance checking replays each trace against the model
and reports deviations (missing/extra/misordered activities) with diagnostics. Token replay was the original
technique; *alignments* (optimal match between trace and model) are now standard because replay needs heuristics
for invisible/duplicate transitions. [documented] https://wires.onlinelibrary.wiley.com/doi/abs/10.1002/widm.1045 ,
https://arxiv.org/pdf/2007.14237 , https://arxiv.org/pdf/1910.09767 . For *declarative* models (Declare:
constraints like `response(a,b)`, `precedence`, `not-coexistence`), conformance means checking each trace against
each constraint; everything not forbidden is allowed; multi-perspective variants add data conditions. [documented]
https://arxiv.org/html/1503.04957 , https://link.springer.com/chapter/10.1007/978-3-642-32885-5_6 , ASP encoding
https://arxiv.org/pdf/2412.10152

**Why it fits redkiln.** redkiln already has the two inputs: an append-only event log (every stage/gate/dispatch
event) and a declared model (the process pack). Two distinct questions can be answered offline:
1. *Did the engine do what the pack says?* Replay the `dispatched(activity)` events against the DAG the engine
   claimed to assemble — a deviation here is a **kernel bug**. This is a replay-determinism test, cheap and
   exact, and should be a CI check (same spirit as Temporal replay tests).
2. *Does the pack say sensible things?* Express process-level invariants as Declare constraints over the log —
   `precedence(Verify, Review)`, `response(RequestDeviation, GateOpened)`, `not-coexistence(Skip(Review),
   criticality=high)`, `existence(Closeout)` — and check every run. A violation here is a **pack or rubric defect**.
   Because Declare is declarative, the invariants stay stable while the option lists churn.
Alignments additionally give you *where* a run diverged and the cheapest repair, which is the right diagnostic
when an LLM-classified item took a path a human disagrees with. [inferred]

**Steal this.** Emit events in an XES/OCEL-compatible shape (case id = item id, activity = activity id, timestamp,
resource = card/model, attributes = classification vector, artifacts consumed/produced) so `pm4py` can be used
directly in a spike without building tooling. Keep a `constraints.toml` of Declare rules next to each process pack.

**Risks.** Conformance checking evaluates *behaviour against a model*, not *whether the model is good*; it will not
tell you an activity was pointless. Pair it with outcome metrics (review verdicts, rework, gate durations).
Alignment computation is expensive for large models; redkiln DAGs are tiny, so this is not a concern.

---

## 7. Recommended selection model

**Framing.** The DAG is a build graph over typed artifacts. The rubric decides *which artifacts are required* for
the item to leave the stage; HTN-style ordered methods decide *which producer* materialises each required artifact
when more than one could; content-addressed freshness decides *what is skipped*. The LLM contributes facts
(classification, evidence) and requests (deviations), both as closed schemas the engine validates.

**Activity record — fields it must declare** (process pack TOML; all values are data, no code):

```toml
[[activity]]
id            = "competitor-analysis"
stage         = "investigation"
role          = "investigator"                 # card resolved by selector, per proposal §4.3
provides      = ["CompetitorAnalysis"]         # ArtifactKinds (schema-backed)
requires      = ["Charter"]                    # must be present + fresh before dispatch
derived_from  = ["Charter.market"]             # staleness scope (coarse pointer); default = all of requires
applicable_if = ["kind == initiative", "market_facing == true"]   # closed predicate vocabulary
method_rank   = 10                             # lower wins among activities providing the same kind
weight_min    = 0.5                            # min stage weight at which this option is eligible
cost_class    = "M"                            # S/M/L/XL for appetite accounting
timeout_secs  = 900
memo          = true                           # participates in content-addressed reuse
human_source_ok = true                         # artifact may be supplied by a human instead (source node)
deviation_ok  = ["skip", "downgrade"]          # what an LLM may *request* about this activity
```

**Classification record (LLM output, closed schema):** `kind` enum; ordinal axes `criticality`, `dynamism`,
`size`, `novelty`, `blast_radius` each `{low, med, high, unknown}` with a written anchor per level; booleans
`has_ui`, `touches_auth`, `data_migration`, `external_dependency`; `evidence: [{field, quote}]`; free `notes`
(untrusted, never a predicate input). Digested and logged; it is an *input* to every memo key.

**How the rubric composes (evaluation order, all deterministic):**
1. *Stage weight* from the pack's per-classification table (`kind × criticality → weight`), or an explicit
   override in item frontmatter (logged). `0` = logged skip.
2. *Required artifact set* `R(stage, classification)` = union of rows of the requirement table whose predicates hold
   (`axis >= threshold → ArtifactKind`), plus the stage's baseline set. Rules are boolean over the closed predicate
   vocabulary; no summation.
3. *Short-circuit* — for each `A ∈ R`: if an artifact of kind A exists **and** its recorded `derived_from`
   digests match current digests → `Present`; if digests differ → `Stale` (re-materialise, unless `stale_policy =
   gate`, which opens a human gate showing the diff); else `Missing`.
4. *Producer selection* — for each `Missing`/`Stale` A: candidates = activities with `A ∈ provides`, `applicable_if`
   true, `weight_min <= weight`; pick lowest `method_rank`; ties are a **pack validation error**, not a runtime
   choice. Recurse on the producer's `requires` (backward chaining). No candidate ⇒ `Unsatisfiable(A, failed_predicates)`
   ⇒ gate.
5. *Validate* the assembled DAG (VAL-style): acyclic, every `requires` bound, every `R` reachable, total
   `cost_class` within appetite. Emit `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped: [(A, Present)] }`.
6. *Execute* in topological order; antichains may run in parallel where the proposal's parallel policy allows.
7. *Deviation loop* — an activity may return `RequestDeviation { target, kind: skip|downgrade|add, claimed_predicate, why }`;
   the engine re-evaluates `claimed_predicate` against evidence; if it holds, re-run steps 2–5 and log the delta;
   if not, open a gate with the request attached. The LLM never edits R directly.

**Explainability contract.** Every selection is reproducible from `(pack digest, classification digest, artifact
digests)` and explained by the list of rule rows fired plus the method chosen per artifact. This is the text a
human sees at a gate and the input to the conformance check.

**Experiments / spikes** (each: hypothesis, method, metric, kill criterion; tagged as measured only after run)

- **E1 — Classification stability.** *H:* the closed-schema classification vector is stable enough to be a
  deterministic input. *Method:* 30 historical items from the old `.bklg` (mix of kinds) × 5 runs × 2 models
  (Sonnet/Opus, or Claude/Codex), temperature default; compare vectors. *Metric:* per-axis agreement (Krippendorff's
  α) and % of items whose *selected DAG* is identical across runs. *Kill:* α < 0.6 on any axis that gates a
  required artifact, or < 80% identical DAGs — then that axis must become human-supplied or the anchors rewritten.
- **E2 — Short-circuit correctness.** *H:* artifact-level freshness prunes rework without skipping needed work.
  *Method:* replay 20 completed initiatives/projects with their artifacts pre-seeded; vary one upstream artifact per
  case; record what the engine marks Present/Stale/Missing. *Metric:* precision/recall against a human-labelled
  "should re-run" set. *Kill:* recall < 0.9 (misses needed rework) or precision < 0.7 (redoes too much) with coarse
  `derived_from`; if finer scoping cannot fix it, fall back to `stale_policy = gate` for L-class activities.
- **E3 — Rubric form: boolean-threshold rows vs weighted sum.** *H:* threshold rules give equal selection quality
  with better explainability and no rank reversal. *Method:* encode the same intent both ways; run both over the
  E1 corpus; then add two new activities to the catalog and re-run. *Metric:* selection agreement with a human
  panel; count of items whose selection changed after the catalog addition; time for a reviewer to state why an
  activity was chosen (think-aloud). *Kill:* if threshold rules need > 3× the rows of the weighted form to reach the
  same agreement, adopt ordinal-per-axis with a documented tie-break instead.
- **E4 — Deviation channel value.** *H:* typed `RequestDeviation` catches real mis-selections without giving the
  model control. *Method:* run the E1 corpus with deviation enabled; classify every request as
  warranted/unwarranted by a human; measure how many warranted ones the engine auto-accepts (predicate held) vs gate.
  *Metric:* requests per item, warranted fraction, gate load. *Kill:* warranted fraction < 30% or > 1 request per
  item on average — then restrict `deviation_ok` to `skip` only or remove the channel.
- **E5 — Replay + conformance in CI.** *H:* the event log is sufficient to re-derive every DAG and to check Declare
  invariants. *Method:* export runs to XES; assert `dag_assembled` reproduces from inputs; check ~10 Declare
  constraints with `pm4py`. *Metric:* 100% replay identity; number of constraint violations traced to pack vs
  kernel. *Kill:* any non-reproducible DAG means an unlogged input exists (the Nix "hidden input" failure) and the
  memo key is unsound — fix before anything else.

**Non-goals confirmed by this survey.** No PDDL planner, no SAT at runtime, no MCTS over workflows, no
model-generated orchestration scripts. Offline SAT/ASP for catalog linting and process-mining for evaluation are
the only solver-class tools worth adopting, and both stay outside the run loop.
