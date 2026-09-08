# Activity-graph schema — PC6 realised

Date: 2026-09-07. Status: proposal. Companion to `2026-09-07-process-catalog-proposal.md` (PC1–PC8) and the architecture proposal (2026-09-05, §4.2–4.6). Evidence tags as before; nothing here is measured.

## 0. What this schema must satisfy

| Constraint | Source |
| --- | --- |
| An activity is a graph that owns the quality of its output; the stage sees only `Present(artifact, evidence)` | PC6 |
| Every non-deterministic step is its own process with an idempotency key `(run_id, activity_id, step_id, attempt)` and a timeout with a number | proposal §4.2, L5.4/L5.6, H4 |
| Human input is a kernel-owned durable `Gate:` mark, never a waiting process, even when raised from inside an activity | proposal §4.6, PC6(i), F22 |
| Memo key covers every card in the subgraph | PC6(ii), F16 |
| Reviews return findings; verdicts are computed by Rust policy | proposal §4.6, F27 |
| Deviation is typed; the model never edits the stage's required set | proposal §4.2, catalog §2.5 step 7 |
| "Should this run?" (`when`) and "how many copies?" (`for_each`) are distinct constructs | workflow-model §2 |
| TOML stays shallow; edges are string references, not nested tables | workflow-model §2 (toml-lang #781/#1052) |
| Variability is annotative (one graph with conditions), not compositional (one graph per rigor) | method-engineering §5 (process-line SLRs) |
| Three budgets, never shared: attempts per step, tokens per item, wall-clock per run | proposal §4.3, L5.5 |

## 1. Concepts

**Activity** — a named graph that materialises one or more artifact kinds with attached evidence. Its public surface is exactly: `provides`, `requires`, `applicable_if`/facets, `method_rank`, `min_rigor`, `cost_class`, `evidence` (what it promises to attach), and three typed exits (`Materialized`, `Blocked`, `RequestDeviation`). Everything else is private.

**Step** — a node in the activity graph. Five kinds, closed:

| kind | Runs | Non-deterministic? | Memoised | Notes |
| --- | --- | --- | --- | --- |
| `agent` | one harness dispatch of a **role** (card resolved by selector, §4.2.1) with a typed output schema | yes | by step key | the only place a model runs |
| `tool` | a toolchain command via the Toolchain port (`test(selection)`, `build`, `change_summary`, tech-pack verbs) | yes (environment) | by step key | output is a typed report; never free text |
| `check` | a deterministic predicate over typed values, evaluated in Rust | no | n/a | branches the graph; cannot dispatch |
| `gate` | a human question; the kernel writes the mark and **exits 0** | n/a | n/a | resumes into the same step with the answer as a typed input |
| `emit` | binds step outputs to the activity's `provides` artifacts and evidence list | no | n/a | exactly one per artifact kind provided; the activity's terminal node |

**Edge** — implicit. A step that references `${other.outputs.x}` in any input depends on `other` (Tekton results semantics). No separate edge table.

**Loop** — a bounded `repeat` on a step: `{ until = <predicate over this step's outputs>, max = N }`. `max` is mandatory. A loop that exhausts `max` takes the step's `on_exhausted` exit (`escalate | fail | continue`). Unbounded loops are a lint error.

**Fan-out** — `for_each = "${step.outputs.list}"` on a step runs one copy per element with `${item}` bound; a `for_each` step's outputs are a list. Cardinality is runtime-computed but the *shape* is declared. Siblings are staggered per §4.3.

**Variant by rigor** — steps carry `when = "rigor >= recommended"` style predicates over the stage rigor the activity was selected at (PC1). One graph, annotated; lint proves the graph is well-formed at every rigor level ≥ `min_rigor`.

## 2. Evidence — the only thing that crosses the boundary besides the artifact

```rust
pub enum Evidence {
    SelfCheck     { step: StepId, checklist: ChecklistId, passed: u16, failed: u16 },
    Findings      { step: StepId, role: Role, items: Vec<Finding> },            // proposal §4.6 shape; no verdict
    ToolReport    { step: StepId, kind: ReportKind, digest: Digest, summary: ReportSummary },  // TestReport, ScanReport, AutomationReport…
    GreenAt       { step: StepId, sha: Sha, selection: TestSelection },          // L2.5
    Mutation      { step: StepId, per_ac: Vec<(AcId, MutationOutcome)> },
    HumanAnswer   { gate: GateId, question: QuestionId, answer: Answer, by: Actor, at: Timestamp },
    Degraded      { from: Tier, to: Tier, why: String },                         // proposal §4.3
    Attempts      { step: StepId, used: u8, max: u8 },
}
```

Rules: closed enum; every variant names the step that produced it; free text appears only inside `Finding.claim`/`why` and is never a predicate input. Stage-exit `checks` (PC4b) are predicates over this list — e.g. `checks = ["evidence.any(GreenAt)", "evidence.none(Findings.items[evidence in {Reproduced, MutationSurvived}])"]` — and the stage may not schedule anything to obtain missing evidence; a failing check is `Blocked` back to the activity's owner (the pack author), surfaced at the gate.

The verdict policy of §4.6 becomes a `check` step *inside* the review-bearing activities and a stage-exit check at the review stage; both are the same Rust function over `Findings`.

## 3. Escalation and the three exits

A step may declare `escalate = { on = <predicate>, question = <QuestionId>, options = [..] }` or a loop may `on_exhausted = "escalate"`. The kernel then:

1. Writes `Gate: <activity>/<step> <question>` as a trailer + `gate_opened` event, attaching the step's typed outputs so far; exits 0.
2. On answer (CLI/comment grammar/web, all via `reconcile()`), resumes **into the same step** with `${gate.answer}` bound; the step's `when`/`repeat` are re-evaluated. Drift since the mark re-opens the gate with the drift attached (§4.6).
3. Records `HumanAnswer` evidence.

Escalation is therefore internal to the activity in *content* (the stage definition never mentions it) and kernel-owned in *mechanism*. Its cost is the F28 flat ~18.6k resume; the pack author pays it deliberately by declaring the escalation point, and `rigor` gates whether the point is live (`when = "rigor >= mandatory"`).

The three exits an activity can take toward the stage, all typed:

| Exit | Meaning | Stage behaviour |
| --- | --- | --- |
| `Materialized { artifacts, evidence }` | the `emit` step ran | mark artifacts Present; run stage checks |
| `Blocked { reason, question }` | the graph cannot proceed and no declared escalation fits | open a gate at the stage |
| `RequestDeviation { target, kind, claimed_predicate, why }` | the activity believes the *selection* was wrong | catalog §2.5 step 7 — re-evaluate predicate against evidence, else gate |

Nothing else leaks. In particular a step failure with retries exhausted is `Blocked`, not an error the stage interprets.

## 4. Budgets, retry, timeout

Every `agent`/`tool` step declares `timeout_secs` (a number; `${toolchain.*}` substitution allowed per PC7) and `retry = { max = N }` selecting the kernel's per-error-class policy (§4.3) — a step never defines its own backoff. The activity declares `budget = { attempts_total, tokens, wall_clock_secs }`; steps draw from it; exhaustion is `Blocked { reason: BudgetExhausted }`. Inner harness retries (`system/api_retry`) count.

## 5. Memoisation and resume

- **Step key** = `hash(activity id@version, variant-relevant rigor, step id, resolved input digests, card digest (agent) or toolchain version + command (tool), injected KB atoms)`. A completed step is never re-run on resume.
- **Activity key** = `hash(activity id@version, rigor, requires digests, all card digests reachable in the graph at that rigor, KB atoms, toolchain versions)` — PC6(ii). It is what the stage's short-circuit consults; a change to any internal card invalidates the artifact.
- Resume after a kill or a gate re-enters the graph at the first step whose key has no committed result (H4/F19 lossless).

## 6. Events (observability without leakage)

`activity_started`, `step_dispatched`, `step_completed { key, cost, usage, version }`, `step_retry { class, attempt }`, `gate_opened { activity, step }`, `gate_answered`, `activity_materialized { artifacts, evidence_digest }`, `activity_blocked`, `deviation_requested`. Internal steps are fully visible in the event log and web projection; they are invisible to *stage definitions*. Process-mining replay (spike P5) works at both granularities.

## 7. TOML shape

```toml
# catalog/analysis/root-cause.toml
[activity]
id            = "root-cause"
version       = 3
stage         = "analysis"
provides      = ["RootCause"]
requires      = ["ReproCase"]
derived_from  = ["ReproCase"]
tags          = ["kind:bug", "kind:incident", "change:text"]    # PC8: (bug OR incident) AND text — same facet disjoins (§9b.20)
method_rank   = 10
min_rigor     = "optional"
cost_class    = "M"
evidence      = ["Findings", "GreenAt", "HumanAnswer?"]         # promise; '?' = only at some rigor
budget        = { attempts_total = 8, tokens = 400_000, wall_clock_secs = 5400 }
keywords      = ["debugging", "sre troubleshooting"]            # engine-ignored (PC8 rule 5)

[[step]]
id      = "hypothesise"
kind    = "agent"
role    = "investigator"
inputs  = { repro = "artifact:ReproCase", telemetry = "toolchain:logs?" }
outputs = "value:HypothesisLog"          # schema-backed; closed enums
timeout_secs = 900
retry   = { max = 2 }
repeat  = { until = "${hypothesise.outputs.status} == 'confirmed'", max = 3 }
on_exhausted = "escalate"
escalate = { question = "root-cause.no-hypothesis" }          # options live in questions.toml (§9a.8)

[[step]]
id      = "confirm"
kind    = "tool"
run     = "toolchain:test"
inputs  = { selection = "${hypothesise.outputs.discriminating_test}" }
outputs = "value:TestReport"
timeout_secs = "${toolchain.test_timeout}"
retry   = { max = 1 }

[[step]]
id      = "adversary"
kind    = "agent"
role    = "reviewer"
when    = "rigor >= recommended"
inputs  = { log = "${hypothesise.outputs}", report = "${confirm.outputs}" }
outputs = "value:Findings"
timeout_secs = 1200
retry   = { max = 1 }

[[step]]
id      = "policy"
kind    = "check"
inputs  = { findings = "${adversary.outputs?}" }             # '?' = absent when adversary skipped
expr    = "findings.none(evidence in {Reproduced, MutationSurvived})"
on_false = { to = "hypothesise", max = 2 }                    # loop back, bounded (§9b.14); hypothesise reads ${adversary.outputs?} so the loop learns (§9b.13)

[[step]]
id      = "signoff"
kind    = "gate"
when    = "rigor >= mandatory"
question = "root-cause.accept"
inputs  = { summary = "${hypothesise.outputs.summary}", findings = "${adversary.outputs}" }

[[step]]
id      = "emit"
kind    = "emit"
artifacts = { RootCause = "${hypothesise.outputs.root_cause}" }
evidence  = ["${confirm}", "${adversary?}", "${signoff?}"]
```

The stage rubric that selects this activity (`[[stage.analysis.require]] when = "kind in [bug, incident]" artifacts = ["RootCause"]`) says nothing about hypotheses, adversaries or sign-off. Raising the stage rigor from `optional` to `mandatory` activates `adversary` and `signoff` without touching the pack.

A fan-out example, from the project lane:

```toml
[[step]]
id       = "implement-slice"
kind     = "agent"
role     = "implementer"
for_each = "${plan.outputs.slices}"           # list from a typed Plan; cardinality runtime, shape declared
inputs   = { slice = "${item}", spec = "artifact:Spec" }
outputs  = "value:ImplReport"
timeout_secs = "${toolchain.implement_timeout}"
retry    = { max = 2 }
```

## 8. Rust types (sketch)

```rust
pub struct ActivityDef {
    pub id: ActivityId, pub version: u32, pub stage: StageId,
    pub provides: Vec<ArtifactKind>, pub requires: Vec<ArtifactKind>, pub derived_from: Vec<ArtifactKind>,
    pub applicable_if: Predicate,            // compiled from tags + applicable_if (PC8)
    pub method_rank: u16, pub min_rigor: Rigor, pub cost_class: CostClass,
    pub evidence: Vec<EvidencePromise>, pub budget: Budget,
    pub steps: Vec<StepDef>,                 // order irrelevant; edges from refs
}
pub enum StepKind {
    Agent { role: Role, outputs: SchemaRef },
    Tool  { run: ToolchainVerb, outputs: SchemaRef },
    Check { expr: Predicate, on_false: OnFalse },
    Gate  { question: QuestionId, options: Vec<OptionId> },
    Emit  { artifacts: BTreeMap<ArtifactKind, Ref>, evidence: Vec<Ref> },
}
pub struct StepDef {
    pub id: StepId, pub kind: StepKind, pub when: Option<Predicate>,
    pub inputs: BTreeMap<String, Ref>, pub for_each: Option<Ref>,
    pub timeout_secs: Option<Timeout>,       // required for Agent/Tool; lint enforces
    pub retry: Option<Retry>, pub repeat: Option<Repeat>, pub on_exhausted: Option<OnExhausted>,
    pub escalate: Option<Escalation>,
}
pub enum ActivityExit { Materialized { artifacts: Vec<ArtifactRef>, evidence: Vec<Evidence> },
                        Blocked { reason: BlockReason, question: Option<QuestionId> },
                        RequestDeviation { target: ArtifactKind, kind: DeviationKind, claimed: Predicate, why: String } }
```

`Predicate` is the same closed-vocabulary expression type used by `applicable_if` and `require.when` (PC8), extended with `${step.outputs.*}` field access and `rigor`. No user-defined functions.

## 9. Lint (catalog load; CI)

1. Graph acyclic except through declared `repeat` / `on_false` back-edges, each with a finite `max`.
2. Every `agent`/`tool` step has a numeric `timeout_secs` after substitution (L5.6).
3. Exactly one `emit` per provided artifact kind; every `provides` bound; every `${ref}` resolves; `?` marks every ref to a conditional step.
4. For every rigor level ≥ `min_rigor`: the sub-graph with `when` evaluated is well-formed and reaches `emit`.
5. The `evidence` promise is satisfiable at every rigor level, and every stage-exit `check` in every pack that can select this activity is decidable over that promise (context lattice enumeration — spike P6 extended).
6. Every `escalate`/`gate` names a `QuestionId` in the pack's question registry with a closed option set.
7. `for_each` sources are list-typed; a `for_each` step's outputs are only consumed as lists.
8. Budget ≥ Σ (step `retry.max` × 1) attempts; otherwise the budget is unreachable and the author is warned.
9. No predicate references a fact outside the facet registry (PC8 rule 1).

## 9a. Amendments from P9 (2026-09-07, `spikes/F32-DRAFT.md` §4)

Authoring ten activities against §1–§9 produced eleven amendments; none reverse a section above, all tighten it:

1. `on_false ∈ { <earlier-declared step>, continue, fail }` — loop-back only. Forward branching is `when` on the downstream step. Lint rejects forward targets.
2. Absent `?` inputs are `null`; quantifiers over `null` are vacuously true; `null == null`; any other operation on `null` is a lint error.
3. Predicate grammar (normative): roots = step inputs, facets, `rigor`, `item`; `.field` access; `len()`; `none/all/any(<field> in […])` with the element's fields in scope; `== != < <= > >= in`; `and or not`; string/number/bool/`null` literals. No user functions.
4. Coalesce in refs: `${a.outputs?} ?? ${b.outputs}`.
5. `for_each` sources: `${step.outputs.<list-field>}` or `artifact:Kind.<list-field>`; fan-out outputs are projected with `${step.outputs[*].field}`.
6. Evidence minting is normative: `tool` mints `ToolReport` (+ `GreenAt` when `run = toolchain:test` and status passing; + `Mutation` when the output is a `MutationReport`); `agent` mints `Findings` or `SelfCheck` by output type; `gate` mints `HumanAnswer`; `check` and `emit` mint nothing.
7. Lint rule 8 becomes: `budget.attempts_total ≥ number of agent/tool steps on one pass`.
8. `escalate = { question = … }` only; options live in the question registry.
9. (Deferred) `after = "<step>"` implying the same `when`, if rigor-conditional chains recur at scale.
10. Input sources (normative): `artifact:Kind[.field]`, `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]`, `${…}`, literals; lint checks `kb:`/`checklist:` against registries.
11. Alternative outcomes (e.g. a rolled-back release) are expressed by an unconditional final `tool`/`check` step that materialises the artifact from whichever branch ran; the exit set stays at three.

Shorthands to add **after P7**: `quality = { adversary = <role>, policy = <policy-id>, gate = <rigor> }` expanding to the `adversary → policy → gate` block (present by intent in 6 of the 10 P9 activities); and `on_exhausted = "escalate"` implying the gate step. Open Q1 below resolves to "shorthands, not includes".

## 9b. Amendments from P9b (2026-09-07, `spikes/F33-DRAFT.md`) — second author

A second author writing three activities against §1–§9a produced 27 ambiguities (20 not covered above) and 13 proposed amendments. P9's claim that the amendment count would fall on the next batch is **falsified at n=3**; the lint's 13 first-run catches in P9 were of things the schema's author already knew (step ids, `?` marks, rigor reachability), and it passed five newcomer guesses untouched. Adjudication:

12. **Feedback refs** (was lint folklore, now normative): a ref to a later-declared step is legal iff that step or one after it has `on_false`/`repeat` targeting the referencing step; it is `null` on the first pass. Goes in §1 "Edge" and lint rule 1.
13. **Loops must be able to learn** — a §5 design defect, not an ambiguity. The iteration counter enters the step key for every step inside a `repeat`/`on_false` cycle; `tool` steps that read live state (telemetry, flags, clocks) declare `memo = false`. Lint: a loop-back target with neither a feedback ref nor `memo = false` is an error. This applied to P9's `hypothesise` and P9b's `observe` alike.
14. **`on_false = { to = "<earlier step>", max = N }`**; the scalar form is an error (rule 1 demanded a finite `max` with nowhere to put it).
15. **Gate output is `answer`, only.** Downstream reads `${<gate-step>.answer}`; `.outputs` on a gate is a lint error; §8 `Gate` gains `→ answer: OptionId`. Until the post-P7 shorthand lands, the documented pattern for `on_exhausted = "escalate"` is an explicit `gate` step with `when` over the looped step's status.
16. **Deviation is authorable**: `on_false.to = "deviate"` with `deviate = { kind = skip|downgrade|add, target = <ArtifactKind>, claimed = <predicate> }`; the kernel handles it per catalog §2.5 step 7. Previously `RequestDeviation` was an exit no step could produce.
17. **Spawn — accepted in a different home.** Not on the activity (PC2: policy lives in the pack). The activity provides `ProblemRecord`; the pack declares `[[stage.closeout.spawn]] when = "exists(ProblemRecord)" kind = "initiative"`, which replaces the proposal's `spawn_followup`.
18. **Grammar**: parentheses allowed; facet values are bare when `[a-z_][a-z0-9_]*`, quoted otherwise (`'security-vuln'`); `facets.toml` gains `ordered = true` on ordinal facets so `>=` is checkable.
19. **Quantifier bodies take full comparisons**: `all(owner != null and due != null)`, not membership only.
20. **Tags: same facet disjoins, distinct facets conjoin** (`kind:bug, kind:incident` = bug or incident). Fixes the §7 example, which was unsatisfiable as a conjunction.
21. **Provided artifacts are unconditional at every rigor ≥ `min_rigor`** (rule 3 text; lint already enforced).
22. **Rule 3 resolves paths**, not just step ids, against the step's output schema (`answer` for gates).
23. **Rule 8: a `for_each` step counts once.**
24. **`artifact:Kind?` is legal; alternation in `requires` is not.**

Registry consequences: `operator` joins the role facet; `Finding` fields (`ac`, `claim`, `evidence`, `severity`, `files`, `repro`) are registered; a toolchain-verb registry and a checklist registry are added and lint-checked (P9b §2 items 4–5). Decided pending P8: a gate re-reached via loop-back is a new mark and a new `HumanAnswer`.

25. **Self-reference means previous iteration.** Inside a loop, `${<this-step>.outputs?}` in a step's own inputs binds the previous iteration's output (`null` on the first pass). It satisfies §9b.13's feedback requirement for steps whose only learning input is their own prior attempt (P9's `dag` step under `acyclic.on_false`).

**Lint v2 status (2026-09-07, later):** `spikes/p9/lint.py` now implements §9 + §9a + §9b (rule 3 path resolution, `registry.toml` for toolchain verbs/feeds/timeouts, `checklist:` and `kb:`; `on_false` table form with forward-target rejection; loop-learning; quoted-value grammar; `ordered` facets; `[*]` projection; new rule 8; unknown-key rejection). v1 is frozen as `lint-v1.py` so `F32`/`F33` result files replay. Against the ten P9 activities v2 raised 25 errors on first run (`results-v2-catalog-run1.txt`) — 12 `on_false` form, 8 gate/check reference shape, 3 loop-learning, 3 registry — all fixed in the catalog (`results-v2-catalog-run2.txt`: 0 errors). Against P9b's three files it raises 15 (`results-v2-catalog-b.txt`), including all five guesses v1 let through; `catalog-b/` is left as the second author's record.

## 9c. Amendments from the P6 authoring batch (2026-09-08, `spikes/F34-DRAFT.md` §4; decided in `spikes/p9/DECISIONS-2026-09-08.md`)

A third author writing 90 activities and running the P6 lattice (`F35-DRAFT.md`) surfaced 30 ambiguities, 25 not covered
above; unlike §9a/§9b these are mostly catalog-level (selection, families, roots) rather than step-level. Adjudicated:

26. **Rank semantics.** Among admissible producers of a kind, the lowest `method_rank` wins (unchanged, §2.5 step 4;
    proposal §3 example). A family escalates with rigor through `max_rigor` on its lower members: an activity is
    inadmissible above its ceiling. Lint: for every rigor ≥ the lowest member's `min_rigor`, some member is admissible in
    every context the lowest member's tags cover ("family total over rigor"). Applies to the rigor-driven families
    (spec form, test design, review, learning); fact-driven families (mitigation, deploy pattern) keep `applicable_if`.
    Measured: ceilings without the totality rule took unsatisfiability from 11.4% to 30.1% (F35 scratch run).
27. **Producer scope and roots.** Candidates for a stage's artifact are activities whose `stage` equals the requiring
    stage. A producer's unconditional `requires` resolve from any stage the pack does not forbid, or from an artifact
    kind registered `source = "human"` in `artifacts.toml` (new registry; also the §2.1 kind vocabulary). Lint: every
    unconditional `requires` kind is produced by some activity or is human-sourced. `forbidden` therefore means what
    proposal §4 says — nothing in that stage runs, even via `requires`.
28. **Families are declared**: `family = "<name>"` on the activity; lint checks shared `provides`, distinct ranks, and
    rule 26's totality.
29. **Registries are one table per value** (`[verb.bisect] outputs = "BisectReport" live = true timeout = "test_timeout"`),
    so additions carry a date and verbs carry a type. `live = true` implies `memo = false` on every step using the verb
    (supersedes the per-step declaration in §9b.13's second sentence).
30. **Boolean facets are `applicable_if`-only**; a tag on a boolean facet is a lint error.
31. **Names**: `facets.toml`'s `toolchain` and `change` are normative; proposal §2.4 / PC7 read accordingly.
32. **Shared question ids are legal**; a mark is identified by (activity, step, question).
33. **Gate attachments.** A gate's output is `answer` plus `attachments: [ArtifactRef]`. A question option may declare
    `attaches = <Kind>`; the kernel refuses to close the mark without it; the attachment is stored as a human-supplied
    root (PC3, `derived_from = []`) and is readable as `${<gate>.attachments.<Kind>}`. Free-text answer fields remain
    forbidden (PC8 rule 5).
34. **Tool steps are typed by their verb** (rule 29): a `tool` step's `outputs` must equal its verb's `outputs` or be
    omitted. `toolchain:record` is removed. `emit` may compose an artifact from several refs —
    `Kind = { field = "${ref}", … }` — with fields checked against the kind's schema; §9a.11's "final tool step" pattern
    is superseded by composition for pure-composition cases.
35. **Lint R4b**: an `on_false.to` target must be active at every rigor at which the check is active.
36. **`rigor` is a legal root in `applicable_if`** (the stage rigor at selection); lint warns where `max_rigor` would
    express the same thing.
37. **`recent_deploy`** joins the facet registry as a derived boolean (from `toolchain:deploy_log`; anchor: a deploy
    touching the item's surface within 24 h). Registry owner's edit, not the catalog's; P1 arm D measures it.
38. **A `gate` inside a `for_each` is a lint error** (revisit after P8 prices a mark).
39. **Null**: `==` and `!=` are defined against `null`; ordering operators on `null` are lint errors (refines §9a.2).
40. **Quantifier bodies admit `len()` and nested calls**; the lint's regex becomes a parser.
41. **Empty `evidence`** is legal only for activities with no `agent` step.
42. **Edits to shipped activities** for catalog coherence bump `version` and are listed in the pack changelog; the P6
    lattice is the test that they did not break reachability.
43. **Toolchain-keyed kinds** (`keyed_by = "toolchain"` in `artifacts.toml`: `ChangeSummary`, `TestReport`,
    `ReproCase`, `Changeset`, `ProfileReport`). A required keyed kind expands to one instance per element of the item's
    `toolchain` set; one producer per instance; PC7(ii)'s "specificity beats generic" tie-break is withdrawn — a
    generic producer's `'text' in change` becomes an instance-level predicate. Measured: under the old rule a mixed
    Node+UE item received the UE change summary only, silently (F35 §1.3).

Rejected: boolean tags (30); per-element gate marks (38, pending P8); `wants_facet` (37 — the fact was derivable, so it
became a facet); free-text gate answers (33).

Registry consequences: `artifacts.toml` (kinds, `source`, `keyed_by`); `registry.toml` reshaped per 29; `facets.toml`
+`recent_deploy`; `questions.toml` options may carry `attaches`. Lint v3 implements 26–28, 30, 33–36, 38–41; v2 is frozen as
`lint-v2.py` so F34's result files replay.

## 10. Open questions

1. **Cross-activity reuse of sub-graphs.** A `red-green` loop or an `adversary + policy` pair will recur. Options: copy (simple; drift), `include = "fragment:adversarial-check"` with parameters (SPEM overlay algebra, `contributes` only), or a `role`-level default graph. Recommend starting with copy and measuring duplication before adding fragments.
2. **Question registry ownership.** Escalation questions need a stable id, text and option set for the comment grammar and web view. Proposed: pack-level `questions.toml`; tech packs may add questions, not edit.
3. **Evidence retention.** Full `Findings` per item are large; store the digest in the artifact's frontmatter and the body in the JSON sidecar (layout C).
4. **Whether `check` may call the Toolchain port** (e.g. "did the affected-test selection change?"). Proposed no — keep `check` pure over values; put port reads in `tool` steps so they are memoised and attributable.
