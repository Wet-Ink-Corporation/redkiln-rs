# Process catalog and deterministic activity selection — proposal

Date: 2026-09-07. Status: brainstorm → proposal for spikes. Builds on the architecture proposal (2026-09-05, §4.2 packs/weights, D14) and the workflow-model research (2026-09-03). Supporting research written today: `research/2026-09-07-method-engineering.md`, `research/2026-09-07-selection-and-planning.md`, `research/2026-09-07-competitive-workflows.md`, `research/2026-09-07-activity-catalog-raw.md` (267 sourced activities).

Evidence tags: [documented] = fetched source; [inferred] = reasoning from sources; [measured] = only after a spike runs. Nothing in this note is measured yet.

## 0. Verdict

The idea is viable and has forty years of prior art, but one part of the framing should change.

**Keep:** activities that carry their own applicability (inputs, outputs, conditions, cost), a per-stage rubric that selects them, a deterministic engine that assembles the DAG, and "this initiative already has a competitor analysis, skip it." Every one of these has a direct precedent: OMG Essence activity spaces carry `Input / Entry Criteria / Completion Criteria` as typed alpha-state pairs and the spec's stated purpose is a method that "will return advice on what to do next" [documented]; SME method chunks have a `<situation, intention>` interface [documented]; Provop/C-EPC attach presence conditions to configurable nodes and prove configuration decidable [documented]; V-Modell XT has run questionnaire→process-module tailoring in production for two decades [documented].

**Change:** do not model selection as *weights and calculations that roll up into a score*. Weighted-sum rubrics suffer rank reversal (adding an option changes the ranking of the others) and are hard to explain at a gate [documented, selection §5]. Every deterministic tailoring system that survived narrowed the problem instead: a small banded context vector, boolean threshold rules, a mandatory core, a finite set of tailoring operations, and a logged waiver path [documented, method-engineering §4]. Model it that way.

**Reframe:** the DAG is not "a list of activities per stage"; it is a **build graph over typed artifacts**. The rubric decides which artifacts a work item must possess to leave a stage; activities are *producers* of artifacts; the engine backward-chains from missing artifacts to producers (Make/Bazel/Dagster; HTN ordered methods) [documented, selection §1–2]. "Already has a competitor analysis" is then not a special case — it is a cache hit on an artifact whose inputs have not changed. This also gives the hallucination guarantee for free: the LLM never sees a choice about *what* runs, only a classification form to fill in and a typed deviation request it may raise.

**What is not precedented** (and therefore needs spikes): binding entry/skip criteria to *runtime artifact facts* recomputed per work item, with Essence-style non-monotonic regression when an upstream artifact changes. Every prior system configured once at project start with human-judged checklists [inferred, method-engineering "Direct answers" (a)].

## 1. Prior art in one table

| Lineage | What it gives redkiln | What it lacked | Source |
| --- | --- | --- | --- |
| OMG Essence 1.2 / SEMAT | Activity ⟨Input, Entry, Completion⟩ as typed states; "advice on what to do next"; Level-3 conformance = task generation + completeness checking; non-monotonic state regression | Checklists are human-judged; no open runtime; no work-item kinds | [omg.org/spec/Essence/1.2](https://www.omg.org/spec/Essence/1.2/PDF) §7.4, §8.2.3, §9.3.4 |
| SME method chunks (Ralyté; Henderson-Sellers 2014) | `<situation (precondition), intention (postcondition)>` interface; method rationale on every fragment | Similarity-metric selection needing a "method engineer"; no tool | [JUCS 16(3)](https://www.jucs.org/jucs_16_3/situational_method_engineering_state.html) |
| OPEN Process Framework | Deontic matrix per (task, technique): mandatory / recommended / optional / discouraged / forbidden | Same adoption fate | [Firesmith & Henderson-Sellers](https://link.springer.com/article/10.1023/A:1020570027891) |
| SPEM 2.0 / EPF Composer | Method content vs process split; `contributes / replaces / extends` overlay algebra | Documentation nothing executed; content rotted; EPF archived 2022 | method-engineering §1 |
| C-EPC questionnaires (La Rosa) / Provop | Facts → constraints → configuration, decidable, contradiction/cycle checks; presence conditions on nodes; inter-node requires/excludes | Static, design-time; no runtime artifact state | [SoSyM 2009](https://kodu.ut.ee/~dumas/pubs/SoSymVariability.pdf); [Provop](https://www.uni-ulm.de/fileadmin/website_uni_ulm/iui/Ulmer_Informatik_Berichte/2009/UIB_2009-03.pdf) |
| V-Modell XT | Project characteristics → project type → mandatory + selected process modules, tool-generated; four modules never tailored away | German-federal heavyweight | method-engineering §4 |
| Kalus & Kuhrmann 2013; Xu & Ramesh 2007 | 49 tailoring criteria, 20 tailoring actions; tailoring is tacit and best supported by *reusing recorded decisions* | "Consequences remain abstract… interpreted project-per-project" | method-engineering §4 |
| Boehm & Turner 2004 | Five ordinal axes (size, criticality, dynamism, personnel, culture) as a risk-based rubric | Human-plotted | method-engineering §4 |
| Build systems / Dagster | Artifact graph; content-addressed freshness (`missing / stale / deps_updated`); asset checks block downstream | Not process-aware | selection §1 |
| HTN planning (SHOP2) | Ordered methods per task; first applicable wins; ties are errors | — | selection §2 |
| LLM-Modulo (Kambhampati 2024) | Model proposes facts/requests, sound engine selects; plan validation outside the model | — | selection §3 |
| Prifti 2026 AI-SDLC protocol DSL | Policy/mechanism split: structural enforcement removes *process* non-determinism, only bounds *agent* non-determinism | — | [arXiv 2606.20615](https://arxiv.org/html/2606.20615v2) |
| Process mining (Declare, alignments) | Replay the event log against the declared model as the *evaluation* harness | — | selection §6 |

Competitors: nobody combines an item-kind taxonomy, an executable activity catalog, and deterministic selection. Trackers have taxonomy + static state machines (Jira schemes, ADO state categories, GitLab widgets, ServiceNow change models with risk-score routing); AI frameworks have executable activities with either fixed order (spec-kit, Kiro) or LLM choice (BMAD v6 scale levels with human confirm, superpowers' injected "check skills first" mandate, Claude Code Dynamic Workflows writing its own script). Re-entry is naïve everywhere except OpenSpec delta specs and Dynamic Workflows' replay-by-prompt-equality; neither keys on the item. No AI tool ships an incident-specific catalog (Factory retired its Reliability Droid) [documented, competitive §C–D].

## 2. The model

Three layers, mirroring SPEM's content/process split and the proposal's D14 pack axes.

### 2.1 Artifact kinds — the join key

A closed vocabulary of typed artifacts (`serde`/`schemars` types per proposal §4.4), ~110 after collapsing the raw list's ~190 (e.g. `Verdict{kind}`, `TestReport{level}`, `ScanReport{tool}`). Each stored artifact records `derived_from: [(ArtifactKind, digest)]` at materialisation so staleness is decidable. Artifacts may be **human-supplied** (a pasted competitor analysis is a source node with no producer) — that is what makes "already contains" work for things no agent made.

### 2.2 Activity record — catalog content, pack-independent

```toml
[[activity]]
id            = "competitor-analysis"
stage         = "discovery"
role          = "investigator"                        # card resolved by selector (proposal §4.2.1)
provides      = ["CompetitorAnalysis"]
requires      = ["ProblemStatement"]                  # present + fresh before dispatch
derived_from  = ["ProblemStatement"]                  # staleness scope; default = requires
applicable_if = ["kind in [initiative, project]", "market_facing"]   # closed predicate vocabulary
method_rank   = 20                                    # lower wins among providers of the same kind
min_rigor     = "recommended"                         # deontic level at which it becomes eligible
cost_class    = "M"
timeout_secs  = 900
memo          = true
human_source_ok = true
deviation_ok  = ["skip"]                              # what an agent may *request* about it
rationale     = "External-facing work needs a positioning baseline before specification."
```

An activity says what it needs, what it makes, when it is admissible, and how it ranks against alternatives for the same artifact. It does **not** say whether *this* stage on *this* kind requires its artifact — that is policy, and keeping it out of the activity is what lets one catalog serve many packs.

### 2.3 Stage rubric — pack policy

A rubric is a table of boolean rows over the context vector, each contributing required artifact kinds and a rigor level:

```toml
[stage.discovery]
goal      = "The problem is framed and evidenced before a solution is committed to"
baseline  = ["ProblemStatement"]                       # mandatory core, never tailored away
finally   = false

[[stage.discovery.require]]
when = "kind == initiative"
artifacts = ["OutcomeStatement", "OpportunityMap", "DiscoveryReport"]
rigor = "recommended"

[[stage.discovery.require]]
when = "kind in [initiative, project] and market_facing"
artifacts = ["CompetitorAnalysis"]

[[stage.discovery.require]]
when = "novelty >= high"
artifacts = ["AssumptionList", "PrototypeFindings"]
rigor = "mandatory"

[policy.bug-standard]
stages = { triage = "mandatory", analysis = "recommended", implement = "mandatory", verify = "mandatory", review = "mandatory", closeout = "optional" }
```

Rigor replaces the proposal's numeric stage weights with OPF's deontic enum (`forbidden < discouraged < optional < recommended < mandatory`). It is ordinal, never summed: an activity with `min_rigor = recommended` is ineligible when the stage runs at `optional`. The numeric `0.2 / 0.5 / 1.0` weights in proposal §4.2 map onto this enum one-to-one, so nothing already decided is lost; what is gained is that "why was this skipped" is a row, not an arithmetic result.

### 2.4 Context vector — the LLM's whole contribution to selection

Triage emits a closed-schema record: `kind` enum; ordinal axes `criticality`, `novelty`, `size`, `blast_radius`, `dynamism` ∈ {low, med, high, unknown} with a written anchor per level; booleans `market_facing`, `has_ui`, `touches_auth`, `data_migration`, `external_dependency`, `has_repro`; **derived (not LLM-emitted) toolchain facts** `toolchains`, `change_kinds`, `binary_change` computed per item from affected paths (PC7); `evidence: [{field, quote}]`; free `notes` (untrusted, never a predicate input). `unknown` is a legal value that routes to a human at the triage gate rather than to a default. Kalus & Kuhrmann's 49 criteria are the *input* to this classification step, not the vocabulary of the rules [inferred]. Humans confirm at the triage gate (proposal: triage is the one always-run stage) and may override any field with a rationale; the override is stored as a decision-memory row `(context, field, value, why)` and replayed on identical context (Xu & Ramesh).

### 2.5 Selection — a pure function, seven steps

1. **Rigor** per stage from `policy.<class>` or an explicit item-frontmatter override (logged). `forbidden` is a logged skip.
2. **Required set** `R(stage, ctx)` = `baseline` ∪ artifacts of every `require` row whose `when` holds.
3. **Short-circuit** per artifact in `R`: `Present` (exists and every `derived_from` digest matches), `Stale` (digest mismatch → re-materialise, or open a gate showing the diff when `stale_policy = gate` for L-class activities), `Missing`.
4. **Producer** per `Missing`/`Stale` artifact: candidates = activities with the kind in `provides`, `applicable_if` true, `min_rigor ≤ stage rigor`; take lowest `method_rank`; **a tie is a pack validation error, never a runtime choice** (SHOP2). Recurse on the producer's `requires`. No candidate ⇒ `Unsatisfiable(kind, failed_predicates)` ⇒ gate.
5. **Validate** the assembled DAG: acyclic, every `requires` bound, every element of `R` reachable, Σ `cost_class` within the item's appetite, `excludes` respected. Emit `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped: [(kind, Present)] }` to the event log.
6. **Execute** in topological order; antichains parallel where proposal §4.7 allows.
7. **Deviation**: an activity may return `RequestDeviation { target, kind: skip | downgrade | add, claimed_predicate, why }`. The engine re-evaluates `claimed_predicate` against evidence; if it holds, re-run 2–5 and log the delta; otherwise open a gate with the request attached. The agent never edits `R`.

Reproducibility contract: the DAG is a function of `(pack digest, catalog digest, context digest, artifact digests)`. Any run that cannot be replayed to the same DAG has a hidden input, and the memo key of proposal §4.2 is unsound — that is spike P5's kill criterion.

### 2.6 Catalog lint — Essence Level 3 as CI

At pack load: every stage-exit artifact has ≥1 admissible producer for every reachable context; every activity's `provides` is required by some row in some pack (dead content is a warning); no `requires` cycle; `excludes` are symmetric; no two producers tie at equal rank under any context; every `require` row's predicate uses only the closed vocabulary. This is La Rosa's contradiction/cycle check plus Essence's completeness check, and it is a few hundred booleans — trivial in Rust, and it is the mechanism that stopped EPF's content from rotting [inferred].

### 2.7 Two things the LLM still does, and one it must never do

Does: fills the classification form; executes activities; raises typed deviations; proposes catalog additions (mined from successful runs, Think-on-Process style) for a *human* to merge into the catalog. Never: chooses an activity, edits `R`, or interprets a free-text rule. Prifti's framing is the cleanest statement of why: structural enforcement removes process non-determinism (skipped, drifted, reinterpreted steps) and only bounds agent non-determinism inside a node [documented].

## 3. Worked examples

**Bug, standard class, has repro.** ctx = `{kind: bug, has_repro: true, criticality: med}`. Analysis at `recommended`: `R = {ReproCase, RootCause}`. `ReproCase` is Present (the reporter's steps, human-supplied, `human_source_ok`). `RootCause` Missing → producers: `hypothesis-troubleshooting` (rank 10), `bisect` (rank 20, `applicable_if = has_repro and vcs_history`), `instrumented-trace-analysis` (rank 30, `min_rigor = mandatory`). Rank 10 wins; if it returns `RequestDeviation{add: bisect, claimed_predicate: "hypothesis exhausted"}` the engine checks the predicate (a typed `HypothesisLog` with ≥3 rejected entries) and adds it. Implement/verify/review as the proposal's fix lane; closeout at `optional` yields only `debt-logging`.

**Incident.** Stages are not linear per kind: `triage(severity, declare) → operate(mitigate) → analysis(RCA) → implement(hotfix) → release(expedite) → closeout(postmortem, mandatory)`. Operate's mitigation is an ordered method list — `rollback < feature-flag-kill < rate-limit-shed < failover < rolling-restart < hotfix` — chosen by `applicable_if` on facts like `last_deploy_age < 24h` and `flag_exists`. Closeout at `mandatory` requires `Postmortem` and `ProblemRecord`, and `spawn_followup` creates the problem-record initiative (ITIL incident/problem split). Discovery, specification and design are `forbidden` for incidents — a logged constraint, so an agent cannot "helpfully" write a spec during an outage.

**Initiative with an existing competitor analysis.** Ryan pastes `CompetitorAnalysis` into the initiative folder; the store records it with `derived_from: []` (human source). Discovery rubric requires it (`market_facing`); step 3 marks it Present; `competitor-analysis` never dispatches; the gate text reads "CompetitorAnalysis: present (human-supplied 2026-09-07); skipped competitor-analysis". If the `ProblemStatement` is later rewritten, the artifact has no `derived_from` link so it stays Present — correct, because a human vouched for it; an agent-made one would go Stale and re-run or gate.

**Reclassification mid-flight.** A story relabelled `rk:kind/project` re-derives the DAG; every artifact whose producer is still admissible and whose inputs are unchanged stays Present. Reclassification is a label change plus a recompute, not a conversation (BMAD's scope alerts as data).

## 4. Catalog v0 — what to ship

Ship the twelve stages and ~95 activities as the default pack; keep the remaining ~170 raw rows in an `extended` pack. Stage goals, entry conditions and exit artifacts are in `research/2026-09-07-activity-catalog-raw.md` §1; the kind × stage default path in its §4. Per stage below: **ordered methods** (pick one by rank/rigor) then unordered activities. Ids as in the raw catalog.

| Stage | Ordered methods (rank ascending) | Unordered activities in v0 |
| --- | --- | --- |
| triage (always) | `quick-triage` < `structured-triage` < `triage-committee` | `intake-classify`, `duplicate-detect`, `reproducibility-check`, `severity-assess`, `incident-declare`, `vuln-score`, `appetite-set`, `readiness-check` |
| discovery | `problem-framing` < `assumption-mapping` < `opportunity-solution-tree` < `design-sprint` | `stakeholder-map`, `outcome-definition`, `competitor-analysis`, `market-landscape-scan`, `analytics-review`, `user-story-mapping`, `prototype-test`, `rabbit-hole-scan`, `discovery-report` |
| analysis | localisation: `hypothesis-troubleshooting` < `differential-debugging` < `bisect` < `instrumented-trace-analysis`; causal: `five-whys` < `fishbone-analysis` < `fault-tree-analysis` | `minimal-repro`, `change-correlation`, `log-analysis`, `timeline-reconstruction`, `root-cause-confirmation`, `impact-assessment`, `blast-radius-analysis`, `dependency-impact-analysis`, `code-archaeology`, `exploitability-analysis`, `feasibility-study`, `technical-spike`, `tech-debt-assessment`, `analysis-report` |
| specification | `user-story-ac` < `structured-nl-requirements` (EARS) < `bdd-scenarios` < `use-case-spec` | `example-mapping`, `nfr-quality-scenarios`, `slo-definition`, `api-contract-spec`, `data-contract-spec`, `security-requirements`, `abuse-case-spec`, `privacy-requirements`, `accessibility-requirements`, `requirements-tracing`, `definition-of-done`, `spike-question-spec`, `fix-spec` |
| design | description: `c4-context` < `c4-container` < `c4-component`; evaluation: `design-review-lightweight` < `architecture-tradeoff-analysis` | `architecture-options`, `adr-record`, `context-mapping`, `api-design`, `data-model-design`, `data-migration-plan`, `threat-model`, `privacy-by-design`, `ux-flow-design`, `failure-mode-analysis`, `observability-design`, `design-doc`, `walking-skeleton-plan`, `fix-design` |
| planning | sizing: `appetite-check` < `t-shirt-sizing` < `relative-estimation` | `vertical-slicing`, `scope-cutlines`, `dependency-dag`, `risk-register`, `pre-mortem`, `test-strategy`, `rollout-plan`, `spike-timebox-plan`, `migration-sequencing` |
| implement | loop: `test-after` < `tdd-red-green-refactor` < `bdd-outside-in`; large-change safety: `direct-change` < `branch-by-abstraction` < `strangler-fig` | `walking-skeleton`, `parallel-change`, `feature-flag-wrap`, `schema-migration-script`, `instrumentation`, `secure-coding-pass`, `dependency-update`, `docs-update`, `runbook-authoring`, `hotfix`, `self-review`, `implementation-report` |
| verify | design rigor: `smoke-test` < `unit-tests` < `equivalence-boundary` < `property-based-testing` < `mutation-testing` | `integration-tests`, `contract-tests`, `e2e-acceptance-tests`, `regression-suite`, `confirmation-test`, `performance-load-test`, `sast-scan`, `sca-scan`, `secret-scan`, `accessibility-audit`, `migration-dry-run`, `backward-compat-test`, `flaky-test-triage`, `test-completion-report` |
| review | `checklist-review` < `code-review` < `adversarial-review` (findings-only, proposal §4.6) < `formal-inspection` | `design-review`, `security-review`, `privacy-review`, `api-review`, `docs-review`, `production-readiness-review`, `change-authorization`, `spike-decision-review` |
| release | `rolling-deploy` < `blue-green-deploy` < `canary-release` < `flag-progressive-rollout` (+ `dark-launch` orthogonal) | `build-package`, `artifact-signing-provenance`, `sbom-generate`, `versioning`, `release-notes`, `migration-expand-execute`, `migration-contract-execute`, `post-deploy-smoke`, `rollback`, `change-record-close`, `release-record` |
| operate | mitigation: `rollback` < `feature-flag-kill` < `rate-limit-shed` < `failover` < `rolling-restart` < `hotfix` | `slo-monitoring`, `alert-rule-authoring`, `incident-command`, `incident-communication`, `workaround-publish`, `error-budget-review`, `vuln-monitoring`, `adoption-measurement`, `problem-record`, `runbook-update` |
| closeout (finally) | `lightweight-retro` < `structured-retrospective` < `blameless-postmortem` | `action-item-tracking`, `dora-metrics-review`, `outcome-vs-hypothesis`, `knowledge-harvest` (→ `.kb`), `docs-finalization`, `flag-cleanup`, `debt-logging`, `spike-report`, `closure-record` |

Kind-level constraints worth encoding as `forbidden` rows rather than absence: incidents → discovery/spec/design; spikes → release; chores/docs → discovery/analysis/design/planning. Cross-cutting goals (risk, security, docs, metrics — DA "ongoing goals", SSDF) are ordinary activities with `stage = any` and a presence condition, re-enterable from any stage.

Open catalog question: whether `review` is a stage or a gate attribute on every stage (12207 and DA run V&V alongside technical processes). Recommendation: keep it a stage for the item-level verdict, and let stage-specific `*-review` activities live in their own stage with `role = reviewer` — the findings-not-verdicts rule (proposal §4.6) applies to both.

## 5. Risks and failure modes

| Risk | Evidence | Mitigation in this model |
| --- | --- | --- |
| Method bloat; "everything on, then prune" | RUP → OpenUP; Boehm–Turner; Essence's founding motive [documented] | Activities default to `optional`; a row must *positively* require an artifact; mandatory core is small (baseline per stage) |
| Authoring cost > consumption value; content rot | EPF archived 2022; students could not instantiate activity spaces [documented] | Tiny TOML schema; engine is the only consumer; lint in CI fails the build |
| Classification instability makes the "deterministic" DAG effectively random | F27 showed the reviewer's threshold judgement is unstable; the same risk applies to ordinal axes [inferred from measured] | Anchored levels, `unknown` value, human confirmation at triage, spike P1 measures α per axis; unstable axes become human-supplied |
| Staleness scope too coarse → everything re-runs; too fine → stale work accepted | Bazel/Nix over-invalidation vs hidden-input bugs [documented] | `derived_from` coarse by default; `stale_policy = gate` for L-class; spike P2 |
| Human-judged criteria disguised as predicates ("stakeholders in agreement") | Essence checklists [inferred] | Every predicate names an artifact, digest or verdict; judgement-only criteria become a gate node |
| Deviation channel becomes control flow by the back door | Prompt-encoded process drift [documented] | `deviation_ok` is per-activity data; requests are re-evaluated against evidence or gated; spike P4 measures warranted fraction |
| Rubric rows sprawl into a second Jira | Jira scheme sprawl [documented] | Shared stage vocabulary across kinds; kinds vary only in rows fired; lint warns on dead rows |

## 6. Proposed spikes (P-series)

Each is cheap because the rig exists; the corpus is the frozen old `.bklg` (S-MIG's input).

| Spike | Question | Method | Metric | Kill criterion | Gates |
| --- | --- | --- | --- | --- | --- |
| **P1** classification stability | Is the closed-schema context vector stable enough to be a deterministic input? | 30 historical items × 5 runs × 2 models (Sonnet/Opus or Claude/Codex) | Krippendorff's α per axis; % items with identical derived DAG | α < 0.6 on any axis that fires a `require` row, or < 80% identical DAGs → that axis becomes human-supplied or anchors rewritten | catalog v0 rubric shape |
| **P2** short-circuit precision/recall | Does artifact freshness prune rework without skipping needed work? | Replay 20 completed items with artifacts seeded; perturb one upstream artifact per case; compare Present/Stale/Missing against a human "should re-run" label | precision / recall | recall < 0.9 or precision < 0.7 with coarse `derived_from` → `stale_policy = gate` for L-class | store `derived_from` schema |
| **P3** rows vs weighted sum | Do boolean threshold rows match a weighted rubric's selection quality with better explainability and no rank reversal? | Encode the same intent both ways over the P1 corpus; add two activities to the catalog; re-run | agreement with a human panel; items whose selection changed after the addition; time for a reviewer to explain a choice | rows need > 3× the rule count to reach parity → ordinal-per-axis with documented tie-break | §2.3 rubric form |
| **P4** deviation channel value | Do typed `RequestDeviation`s catch real mis-selections without ceding control? | Run P1 corpus with deviations enabled; human labels each request warranted/unwarranted; count auto-accepted vs gated | requests per item; warranted fraction; gate load | warranted < 30% or > 1 request/item → restrict to `skip` or remove | §2.5 step 7 |
| **P5** replay identity + conformance | Is the event log sufficient to re-derive every DAG and to check invariants? | Export runs to XES; assert `dag_assembled` reproduces from logged inputs; check ~10 Declare constraints (e.g. "postmortem always follows incident-close") with pm4py | 100% replay identity; violations attributed to pack vs kernel | any non-reproducible DAG → hidden input → memo key unsound; fix before anything else | Phase 1 kernel |
| **P6** catalog lint coverage | Does the v0 catalog satisfy Level-3-style completeness for every reachable context? | Enumerate the context lattice (~10 kinds × banded axes); run lint | unsatisfiable (stage, ctx) pairs; dead activities; rank ties | > 5% unsatisfiable pairs → catalog gaps before any dispatch | catalog v0 |

P1, P5 and P6 need no model spend beyond P1's ~300 haiku/sonnet classifications; P2–P4 are analysis over existing artifacts plus a small number of dispatches. None gates Phase 1's kernel; P5 should run *inside* Phase 1 because it is the soundness check for memoisation.

### 6.1 Pressure test against PC1–PC8 and the activity-graph schema (2026-09-07, later)

| Spike | Status | Change |
| --- | --- | --- |
| P1 | **refined** | Split the context vector: LLM-emitted facets keep the stability test; derived facets (`toolchains`, `change_kinds`, `touches_auth`, `data_migration`) get a *derivability* test — precision of computing them from paths/diff on the corpus. A facet that derives at ≥0.95 precision leaves the model (PC7/PC8) |
| P2 | **refined** | Add PC3's advisory-drift path: measure advisory noise rate alongside precision/recall. Simulate `derived_from` from the old `.bklg` git history; no store needed |
| P3 | **cut** | PC1 decided the enum on documented grounds; re-open only if P6 shows `require` rows sprawling |
| P4 | **folded into P7** | Add `RequestDeviation` to the output schema of P7's runs and label what comes back |
| P5 | **reclassified** | Needs the kernel's event log and step-level memo keys → Phase 1 acceptance test, not a pre-design spike |
| P6 | **extended** | Enumerate toolchain combinations incl. mixed Node+UE (PC7) and per-rigor well-formedness (schema lint 4–5). Requires the catalog in TOML — P6 *is* the authoring work with P9 |

New spikes:

| Spike | Question | Method | Metric | Kill criterion | Can invalidate |
| --- | --- | --- | --- | --- | --- |
| **P7** encapsulated quality, cost vs catch | Does an activity's internal adversary + policy (PC6) catch more per token than a single dispatch per artifact? | One graph (`root-cause` or story implement) at `optional / recommended / mandatory` on ~10 E3-fixture items; `RequestDeviation` enabled in the schema (P4) | findings caught per token vs F23/F26 single-dispatch baseline; deviation requests per item, warranted fraction | `mandatory` > 3× `recommended` cost with no extra catches → internal default is `recommended`, `mandatory` reserved for `criticality = high`; deviation warranted < 30% → restrict to `skip` | PC6 default rigor; §2.5 step 7 |
| **P8** gate inside a graph | Is resume into a mid-graph `gate` step with partial step outputs lossless, and what does it cost? | ~10 haiku runs on the existing rig; kill between steps and across a gate; compare bound outputs before/after | lost/changed step outputs; resume tokens vs F28's ~18.6k | any lost output → escalation becomes activity-terminal (`Blocked`) only | schema §3 escalation |
| **P9** authoring ergonomics | Is the graph schema cheap enough to author 95 activities without EPF-style rot? | Author 10 representative activities across stages; prototype the lint; count lines, lint catches, repeats of the `adversary + policy` fragment | median TOML lines; lint errors caught per activity; fragment repeat count | median > 150 lines or the same fragment copied > 5× → add `include` fragments (schema open Q1) before the other 85 | PC5; schema open Q1 |

**P9 result (2026-09-07, `spikes/P9-DRAFT.md`) [measured]:** median 56.5 lines/activity (kill > 150) — pass; the `adversary → policy → gate` fragment in 5 of 10 by exact shape, 6 by intent — at threshold, resolved as a `quality` shorthand after P7 rather than includes; lint first run caught 13 real errors (4 design defects) against 34 false alarms from 4 prototype bugs; 11 schema amendments recorded in the schema note §9a. PC5 stands.

**Sequence** (S-series continues undisturbed): P9 → P6 first — they produce the real v0 catalog and a lint prototype, nothing is throwaway; P1 in parallel (cheapest spend); then P7(+P4) and P8 on the existing rig; P2 as corpus analysis whenever. P7, P8 and P9 can each overturn a taken decision, so they gate "design final"; P1 and P2 tune parameters. P5 moves to Phase 1.

## 7. Decisions taken (2026-09-07)

| # | Decision | Choice | Alternatives considered | Grounding |
| --- | --- | --- | --- | --- |
| PC1 | Rigor form | **Deontic enum** `forbidden < discouraged < optional < recommended < mandatory`, ordinal, never summed; activities declare `min_rigor`. Supersedes the numeric `0.2/0.5/1.0` weights in the 2026-09-05 proposal §4.2 | numeric 0–1; enum with numeric sort key; per-activity deontic matrix | OPF deontic matrix; V-Modell XT mandatory modules; rank reversal in weighted sums (selection §5); F27 threshold instability |
| PC2 | Rubric home | **Split**: admissibility (`provides / requires / applicable_if / method_rank / min_rigor`) in the activity; requirement (`baseline` + `require` rows) in the process pack. `redkiln explain` prints both | all-in-activity; all-in-pack | SPEM content/process split; EPF clone-and-own failure; Provop context rules on the base process; D14 |
| PC3 | Human-supplied artifacts | **Vouched root + advisory drift**: `derived_from = []` never blocks or re-runs; when an upstream in the same stage changes, the next gate carries an advisory "human artifact predates X v2" | never stale, silent; human declares inputs at paste; stale → gate | Make source roots; Fiberplane Drift warn-on-change; F18 drift 3.6× past 1 h; F28 gate cost; HELENA effort-decay |
| PC4 | Review shape | **Three levels, encapsulated (amended by PC6).** (a) Activity-internal: an activity is itself a graph responsible for the quality of its output — self-review, adversarial reviewer, retry, HITL escalation all live *inside* it and are invisible to the stage. (b) Stage-exit `checks`: acceptance predicates the stage evaluates over evidence the activity attached — never extra work the stage schedules. (c) A formal `review` stage for the item-level adversarial change review, which no single activity owns | stage only; gate attribute only; stage-visible sibling reviewer nodes (rejected by PC6) | 12207/DA V&V alongside technical processes; Dagster asset checks inside materialisation; proposal §4.6 findings-not-verdicts; F23/F26 headless review |
| PC6 | Quality encapsulation | **Each activity graph owns the quality of its output.** Contract = "materialise artifact X to the declared bar, with evidence"; the stage sees only `Present(X, evidence)`. Nothing about internal review, retries or escalation leaks into stage or workflow definitions. Two kernel constraints: (i) an activity-internal HITL escalation is still a kernel-owned durable `Gate:` mark (proposal §4.6) — the activity *declares* escalation points, the kernel *executes* them; (ii) the memo key hashes every card in the subgraph, so editing an internal reviewer invalidates the artifact | reviewer nodes visible at stage level; stage schedules quality work | Temporal child workflows; Tekton Pipelines-in-Pipelines; Dagster asset checks; Prifti validation tokens as dispatch preconditions |
| PC5 | Catalog size at v0 | **Curated ~95** default pack, full kind × stage coverage; ~170 raw rows stay in an unshipped `extended` file. Every shipped activity must pass P6 lint (reachable from some context) and be executed at least once before the pack is tagged | phase-aligned ~40→95; everything (~267); minimal ~20 + decision memory | EPF rot from unexecuted content; Essence "start minimal"; unused activities cost nothing at runtime under deterministic selection (superpowers' too-many-skills failure is LLM-selection-specific) |

| PC7 | Technology-pack contribution contract | A tech pack may (i) add **facts** to the context vector — `toolchains`, `change_kinds` {text, binary, generated}, `binary_change` — computed **per item from affected paths** via `Toolchain::classify`, never per repo (a story touching Node + UE content needs both change summaries); (ii) contribute **producers** for existing artifact kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `ProfileReport`…) selected by specificity — toolchain match beats generic, equal-specificity ties are a validation error (same rule as §4.2.1 cards); (iii) contribute **artifact kinds** (`CookReport`, `AssetManifest`, `AutomationReport`, `Captures`) and additive `contribute` rows at a declared stage (`[[contribute.verify]] when = "'unreal' in toolchains" artifacts = ["CookReport"]`) — the artifact-model form of D14's slot filler; (iv) override `timeout_secs` / `cost_class` per activity (F21: 2055 s Node reviews; UE budgeted 7200 s). It may **not** add stages, remove baseline artifacts, or edit `require` rows — that remains a process-pack variant (`story-ue`) per D14. Catalog consequence: toolchain-agnostic activities (discovery, spec, design, planning, closeout — most of the ~95) ship once; toolchain-realised ones (implement/verify/review/release producers, `bisect`, `minimal-repro`, `profiling`) ship as a generic text-change producer with `applicable_if = "'text' in change_kinds"` and each tech pack supplies its specialisation | tech pack as a separate catalog per toolchain (duplicates the agnostic 70%); toolchain facts per repo (wrong for mixed repos); tech packs editing rubric rows (violates D14) | Proposal D14 / §4.2.1 selector specificity; S-BIN spike; F21; Cloud Native Buildpacks detect-and-contribute; Tekton Pipelines vs Tasks |

| PC8 | Facets, not tags | Activity classification is a set of **facets**: closed, enumerated vocabularies the engine evaluates against computed facts. `tags = ["kind:bug", "toolchain:unreal", "change:binary"]` is authoring sugar that compiles to a conjunction in `applicable_if`. Five rules: (1) closed vocabulary per facet in `facets.toml`; lint rejects unknown facets/values; a tech pack may extend a facet's *values*, never add a facet; (2) every facet maps to a computed fact in the context vector — if nothing can compute it, it cannot be a facet; (3) facets filter **admissibility only** — selection remains artifact kind → producer → rank; `security` never means "run all security activities"; (4) facets that are human inputs (`kind`, `cos`) share their exact value sets with the tracker labels `rk:kind/*`, `rk:cos/*`, so relabelling an issue is a facet change plus a recompute; (5) free text lives only in an engine-ignored `keywords` field for `redkiln catalog search`. Initial facets: `kind`, `stage`, `toolchain`, `change` {text, binary, generated}, `rigor`, `role`, `cost`, and the boolean risk facets `auth`, `data-migration`, `external-dep`, `ui`, `market-facing` — i.e. the §2.4 context vector plus the activity's placement fields, in one vocabulary | free-form tags (skill-marketplace / Jira-label sprawl; superpowers' description-matching failure); tags as a selection mechanism (reintroduces the LLM-shaped "which activities?" question) | V-Modell XT typed project characteristics; Provop context model with enumerated value ranges; C-EPC facts; competitive §B9, §D lesson 3 |

Consequences for the model above: §2.3's `rigor` field is the enum (PC1); §2.2's activity record gains a `graph` (or `quality`) block describing its internal nodes, escalation points and evidence it attaches, and stages gain `checks = [...]` acceptance predicates only (PC4/PC6); the memo key in §2.5 covers every card in the activity subgraph (PC6); §3's human-supplied example gains the advisory line (PC3); spike P6 becomes a release gate for the pack (PC5). The activity-graph schema (step kinds, escalation, evidence, budgets, memo keys, lint) is in `2026-09-07-activity-graph-schema.md`; rigor-conditional steps (`when = "rigor >= mandatory"`) are where "higher criticality raises the internal bar" lives, not in any pack row.

## 8. Sources

Primary: the four research notes under `references/research/2026-09-07-*` (all URLs are there). Load-bearing externals: [OMG Essence 1.2](https://www.omg.org/spec/Essence/1.2/PDF); [Henderson-Sellers & Ralyté, SME state of the art, JUCS 16(3)](https://www.jucs.org/jucs_16_3/situational_method_engineering_state.html); [La Rosa et al., questionnaire-driven configuration](https://kodu.ut.ee/~dumas/pubs/SoSymVariability.pdf); [Provop](https://www.uni-ulm.de/fileadmin/website_uni_ulm/iui/Ulmer_Informatik_Berichte/2009/UIB_2009-03.pdf); [Kalus & Kuhrmann 2013](https://www.researchgate.net/publication/256471312_Criteria_for_Software_Process_Tailoring_A_Systematic_Review); [Hurtado & Bastarrica MDE tailoring](https://dl.acm.org/doi/10.1145/1987875.1987885); [Boehm & Turner sample chapter](https://ptgmedia.pearsoncmg.com/images/9780321186126/samplepages/0321186125.pdf); [Kambhampati et al., LLM-Modulo (ICML 2024)](https://arxiv.org/abs/2402.01817); [Prifti, AI-SDLC protocol language (2026)](https://arxiv.org/html/2606.20615v2); [Think-on-Process](https://arxiv.org/abs/2409.06568); [Dagster automation conditions](https://docs.dagster.io/guides/automate/declarative-automation); [SHOP2](https://www.cs.umd.edu/projects/shop/); [BMAD v6](https://github.com/bmad-code-org/BMAD-METHOD); [obra/superpowers](https://github.com/obra/superpowers); [Claude Code Dynamic Workflows](https://code.claude.com/docs/en/workflows); SWEBOK v4, ISO/IEC/IEEE 12207:2017 Annex A, ISTQB CTFL v4, NIST SSDF, OWASP threat modeling, Google SRE book ch.12/15/27/32, PagerDuty response docs, DORA capabilities (URLs in the raw catalog).
