# redkiln-rs: composable SDLC workflow — prior art and a proposed contract

## 1. Process canon: what to steal

**Disciplined Agile "Choose your WoW" is the closest structural match and should be the primary transplant.** DA models process as **process goals** ("Explore Scope", "Address Risk"), each decomposing into **decision points** ("intents you must at least consider"), each with a list of **options**. Option lists are either *ordered* (ranked by effectiveness — picking one tells you what to try next as improvement) or *unordered* (no dominance; often pick several), and each decision point carries a **bolded default**. Stated rationale: make tailoring explicit and trade-offs visible ([How to Read Process Goal Diagrams](https://www.pmi.org/disciplined-agile/ip-architecture/how-to-read-process-goal-diagrams), [DA Browser](https://www.pmi.org/disciplined-agile/da-browser)). Direct mapping: **Stage ≈ process goal; "which activities?" ≈ decision point; Activity ≈ option; classification × complexity × size ≈ DA's context factors.** Encode ordered/unordered literally — review rigor is ordered (self-check < adversarial agent < human), investigation techniques are not.

**ISO/IEC/IEEE 12207 is the standards-lineage justification for the whole design**: a catalogue of life-cycle processes (Agreement / Organizational Project-Enabling / Technical Management / Technical groups) plus first-class **tailoring** — the standard explicitly requires no specific life-cycle model and expects organizations to select and adapt processes to project size, risk and regulation ([ISO 12207](https://en.wikipedia.org/wiki/ISO/IEC_12207), [iso.org/standard/63712](https://www.iso.org/standard/63712.html)). That is nearly a definition of redkiln.

**ITIL 4 gives the risk-tier vocabulary for whole-stage skipping.** Change enablement defines **standard** (pre-authorized, repeatable), **normal** (assessed + authorized) and **emergency** (expedited, fixes an active incident) change types; the **service value chain** (Plan, Improve, Engage, Design & Transition, Obtain/Build, Deliver & Support) is explicitly non-linear, composed into per-demand-type value streams ([change types](https://blog.invgate.com/what-are-the-itil-change-categories), [SVC](https://www.novelvista.com/blogs/it-service-management/itil4-service-value-chain-explained)). Also borrow the **incident vs problem** split: an Incident restores service, the Problem record is a *separate* item — which is exactly why Incident skips Discovery/Specification and spawns a follow-on Initiative.

**Cynefin supplies the epistemic "why".** Clear → sense-categorize-respond (best practice; Bug/routine); Complicated → sense-analyze-respond (expert good practice; Feature); Complex → probe-sense-respond (Spike/Discovery *as safe-to-fail probes*, not upfront spec); Chaotic → act-sense-respond (Incident: stabilize first). Disorder's first move is *diagnose which domain you are in* — that is exactly redkiln's Triage stage ([Cynefin](https://conversational-leadership.net/cynefin-framework/)).

**Kanban** contributes an orthogonal axis: **classes of service** — Expedite (WIP 1, breaks limits), Fixed Date, Standard, Intangible — plus WIP limits and **explicit policies** (per-column entry/exit criteria) ([classes of service](https://businessmap.io/kanban-resources/getting-started/kanban-classes-of-service)). Classification chooses *which* stages; class of service chooses *scheduling*. Keep them separate. **Shape Up** contributes **appetite**: a time/risk budget fixed *before* scoping, not an estimate after it ([Betting Table](https://basecamp.com/shapeup/2.2-chapter-08)) — the right shape for redkiln's budget field.

**RUP/OpenUP** contributes the disciplines × phases matrix with per-phase **effort-distribution curves** (~10/25/55/10 across Inception/Elaboration/Construction/Transition) — model investment as *continuous weights per stage per classification*, not binary include/exclude. OpenUP's **practice** (an independently adoptable, composable capsule of process) is essentially redkiln's Activity ([RUP phases](https://www.geeksforgeeks.org/rup-and-its-phases/)).

**DORA** is the measurement layer: emit lead time per stage, change failure rate at Release, MTTR for Incidents, plus the 2025 fifth metric, rework ([dora.dev](https://dora.dev/guides/dora-metrics-four-keys/)). The 2025 State of DevOps finding is load-bearing here: AI is an **amplifier** — it raises throughput *and* instability/rework unless foundations are strong ([summary](https://www.faros.ai/blog/key-takeaways-from-the-dora-report-2025)). **Team Topologies**' interaction modes (Collaboration / X-as-a-Service / Facilitating) usefully type *handoffs* between activities.

## 2. Workflow engine models: the activity contract

The convergent best-practice contract, synthesized:

1. **Typed inputs/outputs as schema** — name, type, required/optional, default. Windmill derives JSON Schema from the function signature; Dagger's typed function signature *is* the contract; GitHub Actions reusable workflows declare typed `inputs`/`secrets` under `workflow_call` ([GH docs](https://docs.github.com/en/actions/how-tos/reuse-automations/reuse-workflows), [Dagger types](https://docs.dagger.io/getting-started/types/)).
2. **Split small values from artifacts.** Argo separates `parameters` (inline) from `artifacts` (external blobs by pointer, with a repository-resolution hierarchy) ([Argo fields](https://argo-workflows.readthedocs.io/en/latest/fields/)); Tekton mirrors this with `results` vs `workspaces` ([Tekton](https://tekton.dev/docs/pipelines/pipelines/)); CWL's `secondaryFiles` handles multi-file artifacts ([CWL](https://www.commonwl.org/v1.2/CommandLineTool.html)).
3. **Dependency edges inferred from output references.** In Tekton, writing `$(tasks.investigate.results.root_cause)` in a downstream param *creates* the ordering edge — no separate DAG. Best ergonomics win available, and it dodges TOML's nesting problem (below).
4. **Content-addressed caching.** Dagger caches by hash of inputs, position-independent ([function caching](https://docs.dagger.io/extending/function-caching/)); Nextflow's `-resume` hashes inputs + files + script ([cache and resume](https://www.nextflow.io/docs/latest/cache-and-resume.html)); Argo `memoize` uses an explicit key. Two independent engines converged: cache key = hash(activity identity + resolved inputs + prompt/logic version). For agent work this is the main cost lever.
5. **Temporal's Workflow/Activity split is both the naming precedent and the architecture.** Workflows are **deterministic and replayable**; Activities hold all non-determinism and carry `RetryPolicy`, timeouts and heartbeats, with idempotency keyed on stable (run ID + activity ID) rather than wall-clock ([Activity definition](https://docs.temporal.io/activity-definition), [retry policies](https://docs.temporal.io/encyclopedia/retry-policies)). Adopt literally: stage selection deterministic and replayable from an event log; agent invocations are Activities.
6. **Artifact-first framing (Dagster).** A software-defined **asset** is a named, typed, durable object; the graph is over *assets*, not tasks, and `@asset_check` runs inside the materialization event and can block downstream ([SDA](https://dagster.io/blog/software-defined-assets), [asset checks](https://docs.dagster.io/guides/test/asset-checks)). "Activity materializes named artifacts" gives provenance and audit for free.

**Conditional/dynamic selection.** Every engine keeps two distinct primitives: a declarative boolean **gate** (Tekton `when`, Argo/GHA `if`/`when`, CWL 1.2 step `when`) and a separate **fan-out** (Tekton `matrix`, Argo `withItems`/`withParam` — the latter consuming a runtime-computed JSON array, GHA `fromJSON` matrices, Dagster partitions). Keep "should this run?" and "how many copies?" as different constructs. Tekton `finally` is the precedent for always-run Closeout/Retro.

**TOML.** Real precedent: Cargo, [mise tasks](https://mise.jdx.dev/tasks/toml-tasks.html), [pixi tasks](https://pixi.prefix.dev/latest/workspace/advanced_tasks/), pyproject. But array-of-tables is a known pain point for graph-shaped data — unresolved spec issues [toml-lang/toml#781](https://github.com/toml-lang/toml/issues/781) and [#1052](https://github.com/toml-lang/toml/issues/1052), plus practitioner complaints ([HN](https://news.ycombinator.com/item?id=29221695)); `just` chose a bespoke DSL over TOML because script bodies don't fit tables. **Mitigation: keep every table shallow and express the graph with Tekton-style string references.** Deep `[[stage.activity.inputs]]` nesting will not stay hand-editable.

## 3. AI SDLC tools: ceremony vs freedom

- **Anthropic Dynamic Workflows** (Claude Code, 28 May 2026, research preview): Claude *generates the orchestration script at runtime* rather than following a fixed sequence — decomposes, fans out to parallel subagents, cross-verifies, iterates to convergence; progress is checkpointed so interrupted runs resume. Triggered explicitly or by an `ultracode` setting letting Claude decide when a workflow is warranted. Anthropic warns it "can consume substantially more tokens than a typical session" ([blog](https://claude.com/blog/introducing-dynamic-workflows-in-claude-code), [InfoQ](https://www.infoq.com/news/2026/06/dynamic-workflows-claude-code/)). **redkiln's most direct competitor and its most useful validation** — but that structure is *emergent*; redkiln's differentiator is *declared, auditable, reproducible* structure with the same dynamism.
- **spec-kit** (GitHub): `constitution.md` (non-negotiable principles) → `/specify` → `/plan` → `/tasks`; four in-repo markdown artifacts, strict sequential phase gates ([repo](https://github.com/github/spec-kit)).
- **OpenSpec** markets the opposite: "fluid not rigid — no phase gates, work on any artifact at any time; iterative not waterfall; minimal ceremony" ([repo](https://github.com/Fission-AI/OpenSpec)). Comparisons land on spec-kit for greenfield, OpenSpec for changing what exists ([comparison](https://ypyl.github.io/programming/2026/06/03/openspec-vs-spec-kit-sdd.html)).
- **Agent OS v3 is the strongest negative evidence.** The v2 implementation and orchestration phases were **retired**: "Today's frontier models handle spec implementation well on their own — this is the recommended approach in 2026+." v3 refocused on *standards* injection ([v3 discussion](https://github.com/buildermethods/agent-os/discussions/310)).
- **Kiro** (AWS): `requirements.md` in **EARS** ("WHEN [condition] THE SYSTEM SHALL [behavior]" — precise enough to derive acceptance tests), `design.md`, `tasks.md`, plus `.kiro/steering/*.md` included always / conditionally on file globs / manually ([Kiro](https://kiro.dev/docs/specs/feature-specs/)). **Conditional steering inclusion is a good pattern for redkiln's context injection.**
- **BMAD**: YAML workflows with one entry-point file declaring metadata and execution model; agents as versioned, diffable artifacts; v6 uses `SKILL.md` with a four-layer TOML resolver (base → team → user → skill) ([custom workflows](https://deepwiki.com/bmad-code-org/BMAD-METHOD/6.4-creating-custom-workflows)).
- **Jules / Amazon Q / Amp** converge on plan-surface-then-approve rather than declared stages ([Jules API](https://developers.google.com/jules/api)).

**Evidence against ceremony (strongest available):** "double documentation overhead when every phase is treated as mandatory ceremony instead of a tool you reach for when the task warrants it"; running every phase gate on a one-line CSS fix; a documented case of **41 spec files across 9 repos, four describing the same service differently, and nine AGENTS.md variants** ([hidden costs](https://hiddedesmet.com/the-hidden-costs-of-spec-driven-development), [what it breaks](https://dev.to/mudassirworks/spec-driven-development-what-it-fixes-and-breaks-1co3)). That literature's conclusion is *exactly redkiln's thesis*: governance needs a proportionate fast path. **Evidence for structure:** spec-kit's persistence argument (every interaction references the same constitution, so output stays consistent), Kiro's EARS→test derivability, and DORA 2025's amplifier finding.

## 4. Docs-as-code with agents

**[mdBook](https://rust-lang.github.io/mdBook/)** is the right choice: pure Rust, one `book.toml`, plain markdown, no JS toolchain, canonical in the ecosystem (The Rust Book, The Cargo Book), and `mdbook test` compiles code blocks — an agent-checkable invariant. Docusaurus/Starlight/Mintlify add versioning and polish at real ceremony cost.

**[Diátaxis](https://diataxis.fr/)** gives agents a deterministic bucket per document: tutorial / how-to / reference / explanation. Its load-bearing claim is that **mixing modes degrades all of them** — precisely the mistake an unsupervised agent makes. Adopted by Django, Canonical, Cloudflare.

**Auto-generated docs are the cautionary tale.** DeepWiki builds repo wikis from static analysis + LLM summarization but has produced flatly wrong claims (asserting Buck as LibreOffice's build system) ([accuracy concerns](https://biggo.com/news/202508270142_DeepWiki_Accuracy_Concerns), [OpenWrt thread](https://forum.openwrt.org/t/important-consideration-deepwiki-ai-generated-docs-may-contain-inaccuracies/249471)). Fluent-but-unverified prose is worse than no docs, because it is trusted by default. Contrast [Context7](https://github.com/upstash/context7), which *retrieves* version-pinned official docs rather than generating.

**Drift detection in CI:** [lychee](https://lychee.cli.rs/) link-checking, [Vale](https://vale.sh/) prose linting, rustdoc doctests + `mdbook test`, CODEOWNERS on `docs/`, and [Fiberplane's Drift](https://fiberplane.com/blog/drift-documentation-linter/), which tracks provenance links between prose and source and fails the build when code changes without a doc update — the clearest existing "docs must change when the API changes" implementation.

**Release automation:** [git-cliff](https://git-cliff.org/) (Rust-native, Conventional Commits → CHANGELOG) plus [release-plz](https://release-plz.dev/) (diffs local crates against the registry, uses `cargo-semver-checks`) — better Rust fits than release-please/changesets.

**ADRs:** [Nygard's original](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) format or MADR; lifecycle proposed → accepted → (rejected | deprecated | superseded). **Accepted ADRs are immutable — supersede, never edit.** For `.kb/` atoms, Matuschak's [evergreen notes](https://notes.andymatuschak.org/Evergreen_notes) (atomic, concept-oriented, linked bottom-up rather than foldered) is the right model.

**Recommended structure and gating:**

```
docs/         # PUBLIC. mdBook. src/{tutorials,how-to,reference,explanation}
.kb/          # INTERNAL. concepts/<slug>.md, decisions/<NNNN>-<slug>.md (MADR),
              #   playbooks/<slug>.md, _intake/ (unadjudicated), _map.md
```

`docs/` is a **stage output gated like code**: agents draft during Closeout/Release, but CI must pass `mdbook build` + `mdbook test` + lychee + Vale, and CODEOWNERS forces human review (Diátaxis mode-mixing is the predictable agent failure). `.kb/` is **agent-autonomous within a contract**: writes go through an adjudicated intake (extract claims → prefer merge/amend over new atom → integrate) enforcing frontmatter schema; the only human gate is ADR status transitions.

## 5. Proposed stage/activity contract (TOML sketch)

Shallow tables + Tekton-style reference strings, so TOML's nesting weakness never bites.

```toml
[stage.investigation]
goal = "Establish what is actually happening and why"   # DA process goal
default_when = "classification in ['incident','bug']"    # gate, not fan-out
always_run = false

[[stage.investigation.activity]]
id  = "reproduce"
run = "agent:investigator"
inputs  = { report = "required:text", telemetry = "optional:artifact" }
outputs = { repro = "artifact:md", confirmed = "value:bool" }
retries = 2
cache_key = "hash(inputs, prompt_version)"        # Dagger/Nextflow style

[[stage.investigation.activity]]
id  = "root-cause"
inputs  = { repro = "required:artifact = ${investigation.reproduce.outputs.repro}" }
outputs = { causes = "artifact:md", adr_candidate = "optional:artifact" }
when    = "${investigation.reproduce.outputs.confirmed} == true"

# Selection policy: classification x size -> stage weights (RUP effort curve)
[policy.incident]      # ITIL emergency change / Cynefin chaotic
class_of_service = "expedite"                     # Kanban: WIP 1, preempts
stages = { triage = 1.0, investigation = 1.0, implement = 1.0, verify = 1.0,
           release = 1.0, closeout = 1.0 }
skip   = ["discovery", "specification", "design", "planning"]
spawn_followup = { classification = "initiative", reason = "problem-record" }

[policy.feature]
class_of_service = "standard"
stages = { triage = 0.2, specification = 0.8, design = 0.6, planning = 0.5,
           implement = 1.0, review = 1.0, verify = 1.0, release = 0.5 }
appetite_hours = 40                               # Shape Up: budget before scope
design.skip_if = "project.surface == 'api-only'"

[policy.spike]                                    # Cynefin complex: probe
stages = { discovery = 1.0, closeout = 1.0 }
outputs_only = true                               # produces knowledge, not code
```

Stage weights (0–1) drive *investment*, not just inclusion — weight 0.2 means one cheap activity, 1.0 means the full ordered option set. Every stage declares explicit entry/exit policies (Kanban), and Closeout runs `finally`-style regardless of outcome.

## 6. Risks of this design, with mitigations

| Risk | Evidence | Mitigation |
|---|---|---|
| **Ceremony bloat** — every stage becomes mandatory | 41 spec files / 9 conflicting AGENTS.md; Agent OS v3 *retired* its orchestration phases | Weights not gates; ship a `fast` policy (ITIL standard change) as the default for small items; measure and delete stages whose artifacts are never read downstream |
| **Token burn** | Anthropic warns Dynamic Workflows "consume substantially more tokens"; DORA 2025 AI-as-amplifier | Content-addressed caching (Dagger/Nextflow) as a first-class feature, not an afterthought; per-stage token budget derived from `appetite_hours`; `when` gates evaluated *before* spawning an agent |
| **Agent ignores the selection policy** | LLM orchestrators drift from declared plans | Enforce selection in the **Rust** orchestrator, not in a prompt (Temporal's determinism split): stage selection is deterministic and replayable; the agent only executes an Activity it was handed |
| **Artifact sprawl / conflicting specs** | The 41-file case; DeepWiki accuracy failures | Dagster-style named typed assets with lineage — an artifact nobody declares as an input is dead and gets pruned; one canonical artifact per (item, stage, name); accepted ADRs immutable |
| **Docs slop reaching users** | DeepWiki's wrong build-system claims | `docs/` gated by `mdbook test` + lychee + Vale + CODEOWNERS human review; agents may only *propose* public docs |
| **TOML unusable at depth** | toml-lang #781/#1052; `just` rejected TOML | Shallow tables + reference strings; validate the schema in Rust with good error spans; if the graph layer outgrows TOML, that seam is already isolated |
| **Wrong classification poisons everything** | Cynefin's Disorder domain | Triage is the one always-run stage; make reclassification cheap and mid-flight (Jules-style steerability), and log classification changes as a DORA-adjacent metric |
