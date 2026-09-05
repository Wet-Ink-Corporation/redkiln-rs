# Redkiln-RS — reimagining brainstorm: positions, challenges, and a proposed shape

**Date:** 2026-09-03
**Builds on:** the RFC (08-31), spike charter (08-31), shipmates lessons (09-01), RFC amendment (09-02)
**Evidence:** `references/research/2026-09-03-*.md` (six research briefs, every claim linked; UNVERIFIED items marked there)
**Status:** brainstorm. Nothing here is a decision. The E3 quality gate and the human-gate spike are still unrun, and this memo should not outrun them.

---

## 0. The short version

Your instincts survive contact with the market, with three corrections and one reframing.

**Survives.** Granular in-repo work breakdown; `.bklg/` as the store; `.kb/` in-repo; a curated `docs/` site; a composable stage/activity process; GitHub as the collaboration surface; a thin internal web view. Every tool that lived through 2025–26 owns a durable in-repo artifact and ships a harness layer; every tool that died owned only a UI or only a session (Vibe Kanban at 27.6k stars, Terragon, Crystal, HumanLayer OSS). Redkiln already owns the artifact.

**Correction 1 — the orchestrating agent must not choose the stages.** An LLM picking which activities to run reintroduces the exact defect the RFC measured: control flow living as prose the model re-executes. Triage (an LLM activity) *classifies*; a deterministic policy table in Rust *selects*; the human *confirms* at the triage gate; the agent may *request* a typed, logged deviation. Temporal's workflow/activity split, applied literally.

**Correction 2 — GitHub is a projection and an input channel, never a second source of truth.** `.bklg/` owns content and the FSM. Issues/sub-issues/Projects render status; comments, reactions, and reviews are *inputs* reconciled on the next run. Never let both sides free-edit the same field. Every tool that tried bidirectional truth has a drift bug to show for it.

**Correction 3 — gates must be weighted and proportionate, not uniform.** The anti-gate evidence (Agent OS v3 retiring orchestration phases, spec-kit's "reinvented waterfall", the 41-spec-file sprawl case) is evidence against *ceremony*, not against gates. Gates that are cheap and consequential are praised and sold ($100/seat). Gate the transition, not the artifact; scale gate weight to item size; make skipping explicit and logged.

**Reframing.** The market gap is unchanged and now sharper: nobody ships **stage-gated, mid-run, resumable human approval on a durable work item**. Per-edit approval (desktop) doesn't scale; PR-as-only-gate (cloud) has no mid-run control. Anthropic's Dynamic Workflows (May 2026) generate orchestration *emergently* at runtime — the direct competitor — and redkiln's defensible difference is *declared, reproducible, auditable* structure with a human at the boundaries.

---

## 1. Your points, one by one

### 1.1 Granularity of work — keep, and fix visibility by projection, not by moving the store

Agree. The initiative → project → story hierarchy with one-write-path companions (`spec.md` with frozen PR boundary and anchors, `_ledger.md` AC→impl→test triples, `_review.md` structured verdicts) is the most rigorous item model in the space. Archon has no hierarchy at all and can't answer "what's our sprint status"; Backlog.md and Beads are flat; Shipmates now tracks in GitHub only.

The visibility problem is real but is a *rendering* problem. Solve it with two projections of the same FSM: GitHub (for collaborators) and the web view (for internal devs), both fed by one event log. Don't relocate the data to fix the view.

### 1.2 GitHub integration via classification + sub-issues — yes, with an authoritative split

The platform now supports this properly. Sub-issues are GA (≤100 children per parent, ≤8 levels, automatic completion rollup, renders in Projects). Issue dependencies ("blocked by") are GA (≤50 per relation) — use these for the story DAG instead of encoding order in labels. `gh` CLI gained native sub-issue/type/dependency management in June 2026.

**Classification via labels, not issue types.** Issue types are org-scoped (unavailable on personal repos), capped at 25 per org, and single-select. Labels are repo-scoped (created idempotently by `redkiln init`), multi-valued (one label per axis: `rk:kind/incident`, `rk:cos/expedite`, `rk:stage/spec`, `rk:gate/open`), and have a decade-stable API surface with `labeled`/`unlabeled` webhooks. Shipmates, Backlog.md, and Symphony all key off labels. Rules: namespace under one prefix the orchestrator owns outright (shipmates L3.1 — retrofitting namespacing cost a release plus nine issues); swap old-for-new in a single `setLabelsForLabelable` mutation; `kind/*` is a legitimate human input (relabeling is a reclassification request Triage reconciles), while `stage/*` and `gate/*` are orchestrator-only projections — a human edit there is typed drift, surfaced not applied. If the repo lives in an org with issue types, mirror `kind/*` into the type field for Projects filtering, but never read it back.

Proposed mapping:

| `.bklg/` item | GitHub | Notes |
| --- | --- | --- |
| Initiative | Issue (`rk:kind/initiative`) + Project item | Root of the sub-issue tree |
| Project | Sub-issue of the initiative | Rollup shows story completion |
| Story / bug / spike | Sub-issue of the project; `blocked_by` edges | 1:1 with worktree + branch + draft PR |
| Incident | Issue (`rk:kind/incident`) → sub-issue stories; spawns a follow-on Problem/Initiative | ITIL incident/problem split |
| Stage + `gate_open` | `rk:stage/*` + `rk:gate/*` labels; mirrored to Project `Status` field | Written by the orchestrator only |
| Spec / plan / review bodies | **Not** in the issue. SHA-pinned link to the file in `.bklg/` | Issue body = rendered summary |

Rules that the prior art earned the hard way:

- **Store GraphQL node IDs in item frontmatter**, treat issue numbers as display-only.
- **Dirty-delta sync, per field.** Beads' `github sync --push-only` re-pushed the whole backlog every run (O(N) API calls) because dirty state was never cleared.
- **Idempotency key on every external write** (shipmates L5.4) or resume double-posts comments and PRs.
- **GitHub App identity + webhooks**, not PAT + polling. Budget against ~2,000 GraphQL points/min and ~80 content-creating requests/min; batch mutations.
- **Agents in worktrees hold no GitHub write token.** The orchestrator applies writes — the gh-aw "safe-outputs" pattern.
- **Inbound issue text is untrusted data.** A public issue has already been used to trick agentic workflows into exfiltrating private repo data (July 2026). Triage reads it, emits structured classification, and a human confirms before the orchestrator spends money. That confirmation is a natural, cheap first gate.

Shipmates arrived at "sub-issue-first epic tracking" after its referee collapsed; Symphony (OpenAI, April 2026) made Linear the state machine. Both chose an external tracker as *store*; that is the choice to avoid — it's the right surface and the wrong substrate.

### 1.3 Keep `.bklg/` in-repo — yes, strongly

Issue bodies are lossy, unversioned relative to code, not SHA-bound, and not diffable with the change they describe. A `spec.md` whose frozen PR boundary and anchors table point at `file:line` only makes sense next to the code at the same commit. In-repo wins on offline, atomic-with-code, merge-safe, agent-readable with zero API. It loses on multi-repo initiatives, non-dev stakeholders, and notifications — which is exactly what the GitHub projection supplies.

One addition from Beads: **hash-suffixed IDs and atomic claim** (`update --claim` = assignee + status in one write) are what make parallel workers merge-safe. Redkiln's repo-wide locks in `<git-common-dir>/redkiln/` already cover the claim; consider whether `RK-S0263`-style sequential IDs survive two machines allocating concurrently.

### 1.4 Keep `.kb/` in-repo — yes; Archon is the cautionary tale

Archon's 2026-04 rewrite dropped its knowledge base entirely; the pinned explainer issue exists because users asked where it went. Keep the intake → adjudicate → integrate contract, ADR immutability (supersede, never edit), and closeout harvesting. Kiro's conditional steering (include a doc always / on file-glob match / on demand) is a good pattern for how `.kb` atoms get injected into activity context without loading everything.

### 1.5 A curated `docs/` site — yes, gated like code

Recommendation: **mdBook** (Rust-native, one `book.toml`, `mdbook test` compiles code blocks — an agent-checkable invariant), organised by **Diátaxis** (tutorial / how-to / reference / explanation — a deterministic bucket per document, and mode-mixing is precisely the unsupervised-agent failure). Docs are a declared **artifact of the Release/Closeout stage**, but unlike `.kb/`, agents only *propose*: CI runs `mdbook build` + `mdbook test` + lychee + Vale, and CODEOWNERS on `docs/` forces human review. DeepWiki's confidently-wrong generated docs are the reason — fluent unverified prose is worse than none because it's trusted by default. Fiberplane's Drift (fail the build when source changes without its linked doc) is the model for drift detection.

### 1.6 Composable stage/activity workflow in TOML — yes, with the determinism split and weights

This is the load-bearing design, and the prior art is unusually good:

- **Disciplined Agile's "Choose your WoW"** is a near-exact structural match: process goals → decision points → option tables with a bolded default, ordered where options dominate (review rigor: self-check < adversarial agent < human) and unordered where they don't (investigation techniques). Stage ≈ goal, "which activities" ≈ decision point, activity ≈ option, classification × complexity × size ≈ context factors. Transplant it.
- **ISO 12207** supplies the lineage: a process catalogue plus first-class *tailoring* to size, risk, and regulation. That is a definition of redkiln.
- **Temporal** supplies the architecture *and* the words: workflows are deterministic and replayable from an event log; activities hold all non-determinism and carry retry policy, timeout, heartbeat, and an idempotency key. Stage selection = workflow (Rust). Agent dispatch = activity.
- **Tekton/Argo/Dagger** supply the activity contract: typed inputs/outputs; small *values* separate from *artifacts* (by pointer); dependency edges *inferred* from output references (`${investigation.reproduce.outputs.repro}`) rather than a separate DAG; content-addressed caching keyed on `hash(activity id, resolved inputs, prompt version)`; `when` (should this run?) kept distinct from fan-out (how many?); a `finally` stage that always runs (Closeout/Retro).
- **Dagster** supplies the artifact-first framing: activities *materialise named typed assets*; an artifact no downstream activity declares as an input is dead and gets pruned. That is the antidote to spec sprawl.
- **ITIL/Cynefin/Kanban/Shape Up** supply the classification vocabulary: standard / normal / emergency change → class of service (expedite preempts and breaks WIP); Cynefin's domain diagnosis is what Triage does (clear → categorise; complicated → analyse; complex → probe with a Spike; chaotic → stabilise, i.e. Incident); *appetite* (budget fixed before scope) is the right shape for a per-item token budget.

Two design commitments that fall out:

1. **Stage weights, not booleans.** RUP's effort curves say investment is continuous. `stages = { specification = 0.2 }` means the cheapest option in the ordered list; `1.0` means the full set. Skipping is `0`, explicit and logged.
2. **A default fast path.** ITIL's *standard change*: pre-authorised, repeatable, minimal ceremony. Most bugs go here. This is the answer to Agent OS v3 and OpenSpec — not fewer gates, but a proportionate default.

TOML is fine *if kept shallow*. Array-of-tables for graph-shaped data is a known pain (toml-lang #781/#1052; `just` rejected TOML for this reason). Use flat tables plus Tekton-style reference strings; validate the schema in Rust with good error spans; isolate the graph layer so it can leave TOML later. The existing `.redkiln/processes/*.yaml` packs (story, bug, incident, initiative, project, project-lite, story-lite) are the migration source and already encode most of the stage table.

Sketch (see `research/2026-09-03-workflow-model.md` §5 for the fuller version):

```toml
[stage.investigation]
goal = "Establish what is actually happening and why"
default_when = "classification in ['incident','bug']"

[[stage.investigation.activity]]
id = "reproduce"
run = "agent:investigator"          # model/effort tier resolved from the card
inputs  = { report = "required:text", telemetry = "optional:artifact" }
outputs = { repro = "artifact:md", confirmed = "value:bool" }
retries = 2
autonomy = "L2"                     # L1 report / L2 gated / L3 unattended-allowlisted

[policy.incident]
class_of_service = "expedite"
stages = { triage = 1.0, investigation = 1.0, implement = 1.0, verify = 1.0, release = 1.0, closeout = 1.0 }
spawn_followup = { classification = "initiative", reason = "problem-record" }

[policy.bug-standard]               # the fast path; ITIL standard change
stages = { triage = 0.2, investigation = 0.5, implement = 1.0, verify = 1.0 }
```

**Where I'd push back on "activities as mini-workflows".** The literature (Google Research, arXiv:2512.08296 — note: the 09-02 amendment's "correction" of the charter to 180 configurations / 4 benchmarks matches the paper's v1 and the Google blog, but the paper's v3 revision expanded to 260 / 6, so the charter's figures were not wrong; the effect sizes are unchanged across versions) says multi-agent orchestration *degrades* sequential tasks by 39–70% and helps parallelisable ones by ~80%. Most activities should be *one dispatch with a schema* — a single agent, a rubric fixed before evidence, a structured verdict. Save orchestration for the project DAG (parallel stories across worktrees, which the amendment repriced at ~31% cache cost unmitigated and ~0% under `--system-prompt`) and for review panels. The fix lane should get *leaner*, not richer.

### 1.7 HITL through GitHub — yes for the channel; the verdict needs a grammar

Comments + reactions for lightweight Q&A; PR reviews for code; Environments + required reviewers for the release gate. But a *stage* gate (spec approved? plan approved?) needs a structured verdict, and comments are free text. Two options that can coexist because both land in the same `reconcile()`: a tiny comment grammar (`/redkiln approve`, `/redkiln changes-requested: …`) parsed by the orchestrator, and a button in the web view. Either way the gate is a **durable mark, not a waiting process** — F22 showed background jobs don't survive, and F18/F19 showed ~48% of gates outlive the 1h cache TTL with drift 3.6× higher past an hour. Reconcile-on-resume is a correctness requirement. Approval must be reachable from a phone hours later.

Borrow two things: HumanLayer's economics framing — *gate before the expensive phase* is a cost-control feature, not just governance — and shipmates L5.10: the human gate belongs at the **integration artifact** (the combined PR head of a project), not at every unit. Shipmates' second-order "epic integration board" that judges cross-story coherence rather than re-approving units is worth stealing outright.

### 1.8 Web view — yes, thin, read-mostly, never the product

Every dead tool had a beautiful UI; every survivor is CLI plus a thin view (Backlog.md's kanban, Beads' CLI, Gas Town's `gt dashboard` that its reviewer bounced off). The recurring complaint is **observability**, not aesthetics — "6 PRs merged and I had no idea when I'd slung them"; 141 orphaned processes; daemons silently not running. So: design the **event log first** (Temporal-style history: stage_enter/exit, gate_result, verdict, dispatch, cost, drift), persist it in the SQLite index you already have, and make the web view a projection of it with exactly one write: approve/reject. Keep CLI and FSM authoritative. The hosted Pi-harness phase is plausible but is a phase-3 question; Kilo hosting Gas Town is the pattern (OSS engine, someone else monetises hosting), and it's unproven.

---

## 2. Things you didn't list that I'd add

**Graduated autonomy per stage (L1 report / L2 gated / L3 unattended with allowlisted actions).** From harness-rs; the single best idea in the survey. It converts "gates are rigid" into a per-classification × per-stage dial, and your existing `automation: HITL` field is the seed. Do not ship an autonomy level you'll walk back — shipmates' `epic merge auto` lived one day.

**Escalation as a first-class outcome.** Gas Town's agent→mayor mail and Shipmates' "stuck → ping chat → resume" both exist because a blocked step must *raise a gate*, not stall or improvise. Make `Blocked(reason, question)` a typed activity outcome that opens a gate and posts the question to the issue.

**Merge queue for the project DAG.** Parallel worktrees need serialised integration (Gas Town's refinery). Pair with per-story failure isolation — quarantine dependents, continue independent branches — which nobody in the space has built.

**Named, value-gated reviewer seats with mandatory "why skipped".** Shipmates' `COST.md` discipline: every reviewer seat is either convened or named as gated-out with a reason. Cheap, auditable, and it's how the review panel scales with stage weight.

**Harness as an activity-runner adapter, capability claims dated and probed.** Generate harness-native artifacts (AGENTS.md / SKILL.md / cards) from one source of truth (the wshobson/agents pattern); the activity contract stays harness-agnostic; per-activity harness selection is a policy field. Shipmates L5.7: capability claims decay in weeks — every one needs a date and a probe. Multi-harness is untested in the spike programme; design the seam, ship Claude only.

**Reclassification is cheap and mid-flight.** Cynefin's Disorder domain: the wrong classification poisons everything downstream. Triage is the one always-run stage; log classification changes as a metric.

**DORA as the measurement layer.** Lead time per stage, change-failure rate at Release, MTTR for Incidents, and rework. DORA 2025's finding that AI *amplifies* both throughput and instability is the argument for keeping verification-gated loops as the spine.

---

## 3. What I'd challenge hardest

1. **"The orchestrating agent could choose which activities."** No — see §0. Classification is an LLM output; selection is a deterministic table; deviation is a typed request the human sees.
2. **Composability can become ceremony.** Measure artifacts that are never read downstream and delete the activities that produce them. The 41-spec-files case is what "great defaults" become without pruning.
3. **Don't design against preview APIs.** Agent HQ (Claude/Codex inside GitHub) and gh-aw are technical preview; sub-issues, types, dependencies, Projects v2 are GA. Build on the GA set.
4. **The thesis is still unproven.** Nothing in this memo bears on E3 (review-quality divergence, the kill criterion) or the human gate (no spike exists, and it's the differentiator). A human-gate spike — suspend, mutate the world, resume, measure drift and UX — should precede any UI work.
5. **Sequential IDs and single-machine assumptions** will bite the moment two orchestrators or two developers allocate concurrently. Decide now.
6. **Web view before event log** is the wrong order. Event log, CLI status, GitHub projection, *then* the view.

---

## 4. A phased shape

| Phase | Gate to enter | Scope |
| --- | --- | --- |
| **0 — finish the spikes** | — | E3 arms (a)/(b); a human-gate spike (suspend → world changes → resume → reconcile); binary-size check; `--max-budget-usd` verification |
| **1 — orchestrator, Claude only** | E3 doesn't fire the kill criterion | Rust loop; step runner; event log in SQLite; `reconcile()` with typed drift; TOML process packs ported from `.redkiln/processes/*.yaml` with weights + fast path; one-way GitHub projection (issues/sub-issues/status) + comment/reaction ingestion; autonomy levels; escalation outcome |
| **2 — surfaces** | Phase 1 runs a real initiative end-to-end | Thin web view (read + approve); `docs/` as a Closeout artifact under mdBook/Diátaxis with CI gates; project-DAG parallelism across worktrees + merge queue; integration board |
| **3 — reach** | Phase 2 stable for a quarter | Second harness behind the adapter seam; hosted variant if there's a reason to |

---

## 5. Open questions this memo doesn't settle

- Which side wins when a human drags a Project board card to a status the FSM disagrees with? (Proposal: it's drift, typed and surfaced, never silently applied.)
- Do stage weights live in the policy table only, or can a triage verdict override them per item within bounds?
- Does the activity cache key include the `.kb` atoms injected into context? (It must, or KB updates never invalidate stale work.)
- Is the `support` initiative the right home for incidents, or does Incident become a peer root type?
- What is the minimum comment grammar that a non-developer collaborator will actually use?
