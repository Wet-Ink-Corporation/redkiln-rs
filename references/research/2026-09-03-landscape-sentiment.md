# redkiln-rs: Landscape + Sentiment Survey (as of 2026-09-03)

Scope: worktree/kanban managers, backlog ledgers, gated/HITL products, multi-harness abstractions, spec-driven frameworks. Claims are linked; inference is marked UNVERIFIED.

---

## 1. Worktree / parallel-agent managers & kanbans

The single most important datapoint since the 2026-08-31 pass is confirmed and dated: **bloop shut down on 10 April 2026**, taking Vibe Kanban's company with it. The post-mortem is explicit and short: "Thousands of software engineers use Vibe Kanban every day... but the vast majority are free users and we couldn't find a business model that we could get excited about" ([shutdown post](https://www.vibekanban.com/blog/shutdown)). Notably it was *not* a traction or tech failure — ~27.6k stars, and it shipped multi-agent support, diff commenting, live preview, click-to-edit and remote access first. Cloud features (kanban issues, comments, projects, orgs) were removed 30 days later; **local workspaces still work**, and the project continues Apache-2.0 community-maintained.

| Tool | Status (Sep 2026) | Shape |
|---|---|---|
| Vibe Kanban | Company dead 2026-04-10; OSS community-maintained, local-only | Kanban + worktrees |
| Conductor (conductor.build) | Alive, proprietary, free, macOS-only | Worktree flow, "least ceremony" |
| Crystal | Reported dead/superseded in prior pass; listed only in comparison content | — |
| Terragon | Dead ("now-dead" cloud VM platform) | — |
| Claude Squad | Alive, tmux-based, per-edit approval | Terminal |
| Superset (superset.sh) | Alive, OSS, any-agent/any-OS, local-first desktop | Worktrees + diff/browser/ports/MCP |
| Emdash | Alive, Apache-2.0, Linux/Windows/SSH, **Linear/Jira integration** | Desktop orchestrator |

Sources: [runpane desktop managers](https://runpane.com/desktop-agent-managers), [augmentcode 9 open-source orchestrators](https://www.augmentcode.com/tools/open-source-agent-orchestrators), [rustman wiki](https://rustman.org/wiki/conductor-parallel-agents/). Vendor-authored comparison pages (superset.sh, runpane, nimbalyst) dominate this query space — treat rankings as marketing, statuses as roughly reliable. Precise star counts/licences for Conductor and Superset: UNVERIFIED.

**Loved:** worktree isolation is now table stakes and nobody complains about it; diff review + live preview in one pane; zero-ceremony launch.
**Pain:** all of these are *session managers* — they own no durable artifact, so the moment the app dies your state dies (exactly what Vibe Kanban users hit). Every one is single-machine desktop.
**Steal:** (1) diff-commenting/click-to-edit as the review surface, (2) Emdash's tracker-sync posture (an external board is a *view*, not the store), (3) per-item worktree lifecycle with explicit decommission.

---

## 2. Backlog / ledger tools — the durable-artifact cluster

This is where the survivors are, and it validates redkiln's core bet.

**Beads (`bd`, Steve Yegge)** — shipped Oct 2025, git-backed dependency-aware issue graph as *external agent memory*; JSONL in `.beads/` committed to git, Dolt underneath for conflict-free merge, hash IDs (`bd-a1b2`) to dodge merge conflicts, and `bd update --claim` for atomic assignee+status so two parallel agents can't grab one task ([bruton.ai](https://bruton.ai/blog/ai-trends/beads-bd-missing-upgrade-your-ai-coding-agent-needs-2026), [ianbull](https://ianbull.com/posts/beads/)). A [Rust port exists](https://github.com/Dicklesworthstone/beads_rust). The strongest sentiment signal in the whole survey: a reviewer who **abandoned Gas Town after a week kept Beads** — "it is the one thing that I continue to use" ([Tenzin Wangdhen](https://tenzinwangdhen.com/posts/gastown-good-bad-ugly/)).

**Gas Town (Yegge)** — open-sourced 1 Jan 2026, v1.0 April, v1.2.1 June, moved to a `gastownhall` org, ~15.9K stars, hosted cloud version from Kilo GA May 2026 (star count and Kilo GA: single-source, UNVERIFIED). The Feb 2026 field review is the most useful negative evidence available:
- **Good:** one "Mayor" you talk to that tracks N workers; real parallelism via worktrees; agent→mayor **mail** so workers escalate instead of stalling; a **merge queue ("refinery")** serialising PRs; work externalised to beads so no single agent's context is load-bearing.
- **Bad:** 141 orphaned Claude Code processes; mail system broke; background daemons silently not running; "observability is thin"; tmux fluency mandatory.
- **Ugly:** launches agents with `--dangerously-skip-permissions` by default; explicitly optimises throughput over correctness ("most work gets done; some work gets lost"); Peter Steinberger calls it the "ultimate token burner"; Yegge himself calls it a cash guzzler needing multiple $200/mo accounts.
- Verdict quote worth internalising: "It falls apart when the work is heavily human-in-the-loop or ambiguous."

**Backlog.md** — per-task markdown in-repo, zero-config CLI, MCP, web kanban, and **three review checkpoints** with acceptance criteria authored before implementation ([repo](https://github.com/MrLesk/Backlog.md)). Thesis stated plainly: structured tasks with acceptance criteria produce consistent outputs; unstructured prompts don't.

**Symphony (OpenAI)** — released **27 April 2026**, open-source spec/framework (Elixir) that makes **Linear the state machine**: one Codex agent per ticket, poll the board, respawn crashed agents, run until merged ([Help Net Security](https://www.helpnetsecurity.com/2026/04/28/openai-symphony-codex-orchestration-linear/), [tessl](https://tessl.io/blog/openai-open-sources-symphony-a-spec-for-orchestrating-codex-agents)). OpenAI claims 6× merged PRs internally in three weeks — vendor-reported, UNVERIFIED. This is the clearest "issue → agent daemon" in market, and it chose an *external* tracker as system of record.

**Ralph-loop runners** — went viral early 2026 (Huntley's pattern: re-run one prompt against a TODO.md until `ALL_TASKS_COMPLETE`). Criticism is precisely redkiln's opening: "no task orchestration, no dependency tracking, no tool permissions"; runaway loops burn tokens, pollute context, and "leave tasks in ambiguous states that are difficult to audit later" ([LogRocket](https://blog.logrocket.com/ralph-claude-code/), [xr0am](https://xr0am.substack.com/p/what-ralph-wiggum-loops-are-missing)).

**Steal:** (1) atomic claim (`--claim`) + hash IDs — merge-safe concurrency is a *format* decision, not a runtime one; (2) agent→coordinator **mail/escalation** channel so a blocked step raises a human gate instead of stalling; (3) a **merge queue** that serialises worktree integration.

---

## 3. Gated / HITL products

| Product | Mid-run human approval | Note |
|---|---|---|
| HumanLayer / CodeLayer | Verification checkpoints **between phases** (QRSPI: Questions, Research, Design, Structure, Plan, Implement) — sign off research + plan before tokens burn on implementation | Free ≤3 users/200 sessions/mo; **Pro $100/user/mo**; BYO keys ([DevTune](https://devtune.ai/verticals/coding-agent-orchestration/humanlayer/pricing)) |
| Conductor / Claude Squad / Crystal | **Per-edit approval** — human reviews before changes apply | Session-scoped, not stage-scoped |
| Devin | Cloud, autonomous; human out of loop until review | Proprietary; no self-host |
| OpenHands (cloud + OSS) | Every action visible; approve changes before merge | Real bug: severe process leak, ~149 zombie processes on long runs ([review](https://aireviewzones.com/openhands-review/)) |
| Copilot coding agent | PR-shaped review; strongest SSO/audit/policy controls; 4.7M paid seats (Mar 2026); Copilot CLI GA Feb 2026 | Gate = PR review |
| Cursor background/cloud agents | PR-shaped; 35% of Cursor's own merged PRs come from background agents | Vendor-claimed |
| Codex cloud | Bundled into ChatGPT sub, not sold standalone | Symphony is its orchestration layer |
| Jules | No 2026 evidence surfaced — **UNVERIFIED / possibly deprioritised** | |

Sources: [sitepoint](https://www.sitepoint.com/claude-code-vs-cursor-vs-copilot-the-2026-developer-comparison/), [tech-insider](https://tech-insider.org/au/codex-vs-cursor-vs-copilot-2026/), [openhands](https://www.openhands.dev/blog/devin-ai-alternatives).

The market has bifurcated into **per-edit approval** (desktop, high friction, doesn't scale past a few agents) and **PR-as-the-only-gate** (cloud, zero mid-run control, review-at-the-end). Almost nobody does **stage-gated mid-run approval on a durable item** — HumanLayer's phase checkpoints are the closest, and they went closed at $100/user/mo. That gap is redkiln's whitespace.

**Steal:** (1) HumanLayer's "gate *before* the expensive phase" economics framing — approval is a cost-control feature, not just governance; (2) treat the gate as a resumable artifact state, not a blocking modal, so approval can arrive hours later from a phone.

---

## 4. Multi-harness abstractions

The portable unit converged on **AGENTS.md + SKILL.md**. `wshobson/agents` maintains one source-of-truth `plugins/` dir and generates harness-native artifacts for Claude Code, Codex CLI, Cursor, OpenCode, Gemini CLI and Copilot — Codex/Cursor/OpenCode read AGENTS.md directly, Claude Code via a CLAUDE.md symlink, Gemini CLI via `gemini-extension.json` ([AGENTS.md](https://github.com/wshobson/agents/blob/main/AGENTS.md)). ~162 skills are dispatched this way. "Aeon" reportedly dispatches skills to six harnesses behind one Claude-Code-shaped contract (UNVERIFIED).

On the Rust side, `harness-rs` is the notable find: a harness-loop-engine that **earns autonomy in stages — L1 report → L2 assisted (human gates every change) → L3 unattended (allowlisted actions only)** — plus a DAG orchestrator with retry/backoff, run budget, crash-resumable state and dynamic replanning ([repo](https://github.com/liliang-cn/harness-rs)). `dex` is confirmed alive and now described as human-gated planning + multi-reviewer review + dead-end-aware research loops, shipping binaries for **7 CLI backends** — it has broadened well past 21 stars' worth of ambition. `oneharness` did not resurface in search: status UNVERIFIED, possibly stalled.

**Steal:** (1) **graduated autonomy levels (L1/L2/L3)** as a first-class per-stage config — this is the single best idea found and it directly answers "gates are too rigid"; (2) generate harness-native artifacts from one source of truth rather than requiring each harness to speak your protocol; (3) crash-resumable run state + run budget as orchestrator primitives.

---

## 5. Spec-driven frameworks: what's loved, what's hated

**Loved:** acceptance criteria written before code; a spec that survives context compaction; traceability from requirement to PR. Nobody argues against *having* a spec.

**Hated — and the evidence is now heavy:**
- **spec-kit**: a Scott Logic review found "a sea of markdown documents, long agent run-times and unexpected friction" with no qualitative benefit to justify the overhead, and titled around "reinvented waterfall"; 20–40% refinement still needed for missed edge cases and constitution violations. Maintenance is openly questioned — [Discussion #1482 "Is SpecKit *really* maintained?"](https://github.com/github/spec-kit/discussions/1482) and [#1686 High Level Design Concerns](https://github.com/github/spec-kit/discussions/1686); features are migrating into Copilot plan mode. One user: "Abandoned spec-kit... it's all too fluid, including my workflow."
- **BMAD**: ~31,667 tokens per workflow run; large projects reported at $800–2,000+/mo per dev, weeks hitting $3,200 on Opus; issues #446/#563 show documentation-first assumptions break on legacy refactors ([Medium comparison](https://medium.com/@reenbit/bmad-vs-spec-kit-vs-openspec-choosing-your-spec-driven-ai-framework-in-2026-a6996b3ebb8d)) — cost figures are secondary-source, UNVERIFIED.
- **OpenSpec** markets the opposite: "update any artifact anytime, no rigid phase gates."
- **Agent OS v3** (Jan 2026) **retired the v2 implementation and orchestration phases** because plan mode + better models handle scaffolding; what remained is `/shape-spec` — spec authoring, not orchestration ([Discussion #310](https://github.com/buildermethods/agent-os/discussions/310), [migration notes](https://buildermethods.com/agent-os/migration)).
- **Kiro** is now AWS's strategic bet: Amazon Q Developer signups blocked 15 May 2026, IDE plugin support ends 30 Apr 2027, replaced by Kiro ([stack-archive](https://stack-archive.com/blog/amazon-q-developer-eos-kiro-agentic-ide-2026/)).
- Contrarian datapoint for structure: Anthropic reports unguided attempts succeed ~33% of the time, and the delta comes from structure around the tool, not prompt wording (secondary-source, UNVERIFIED).

**Steal:** (1) OpenSpec's *editability* — artifacts amendable at any stage, gates governing transitions not edits; (2) BMAD's adversarial review, minus the token profile; (3) spec-kit's traceability IDs.

---

## 6. What the market says

**(a) In-repo backlog vs GitHub Issues as system of record.** Split, and the split is by *who runs the agents*. Every tool that survived by owning a durable artifact chose **in-repo** (Beads, Backlog.md, Archon, [git-issues on HN](https://news.ycombinator.com/item?id=47973644), which stores YAML-frontmatter markdown in `.issues/` and auto-generates `.agent.md` context). Platform vendors chose **external trackers** — OpenAI's Symphony makes Linear the state machine; Emdash syncs Linear/Jira. In-repo wins on: offline, atomic-with-code, merge-safe, agent-readable with zero API. It loses on: multi-repo initiatives, non-dev stakeholders, notifications. Also note the security drift toward external issues as attack surface — a [public GitHub issue tricking agentic workflows into leaking private repo data](https://thehackernews.com/2026/07/public-github-issue-could-trick-github.html). **Recommendation for redkiln: in-repo as store, external tracker as optional projection — never bidirectional truth.**

**(b) Enforced phase gates.** The loudest voices are anti-gate (OpenSpec's marketing, Agent OS v3 retiring phases, spec-kit's "reinvented waterfall" critique). But read what actually failed: gates that were *ceremony* — mandatory documents produced for every item regardless of size. Gates that are *cheap and consequential* survive and are praised: HumanLayer sells sign-off-before-implementation at $100/seat; harness-rs formalises L1/L2/L3 autonomy; Gas Town's own reviewer concluded it "falls apart when the work is heavily human-in-the-loop." The synthesis: **gate the transition, not the artifact; scale gate weight to item size; make skipping a gate an explicit, logged decision rather than a config fork.** Enforced gates are defensible precisely where the anti-gate crowd is weakest — ambiguous, HITL-heavy, expensive work.

**(c) Web UI for internal devs.** Not optional, and not the product. Every dead tool had a beautiful UI (Vibe Kanban) and every survivor has a CLI-plus-thin-view (Backlog.md's web kanban, Beads' CLI, `gt dashboard` — which Gas Town's reviewer bounced off, ending up "in tmux panes anyway"). Observability is the recurring complaint, not aesthetics: "sometimes 6 PRs merged and I had no idea when I'd slung them." Build a **read-mostly run/gate view plus an approve button reachable from a phone**; keep the CLI authoritative.

**(d) Business/adoption models that survived 2025–26.** The scoreboard: dead-with-great-traction (Vibe Kanban — free users, no model; Terragon; Crystal), alive-closed-and-expensive (HumanLayer $100/user/mo; Conductor free/proprietary/macOS; Devin), alive-because-bundled (Codex in ChatGPT, Copilot at 4.7M seats, Cursor, Kiro replacing Amazon Q), and alive-as-pure-OSS-with-a-personality-or-a-format (Beads, Backlog.md, Gas Town, Emdash, Superset). Two conclusions worth acting on: a standalone **local desktop app for individual devs is a proven graveyard**; the durable positions are **owning a file format others adopt** (Beads, AGENTS.md) or **selling the team/compliance layer** (HumanLayer, Copilot enterprise controls). Kilo's hosted Gas Town is the pattern to watch: OSS engine, someone else monetises the hosting. UNVERIFIED whether it is commercially working.

## Erratum (2026-09-11)

Appended after the pre-implementation audit of 2026-09-10 (PIA-101 and the citation census, `references/audit/2026-09-10/census/grounding-by-file/`); this dossier is a dated record and its rows are not rewritten. One bullet per corrected line: `:NN — was … → is …, source, fetched date`. Every fetch below is dated 2026-09-10 unless stated. The dossiers carried no fetch dates, which is why half of these were already stale at authoring: every research read going forward carries a fetch date.

- :35 — "~15.9K stars" (self-flagged single-source) → **18,006** on 2026-09-10, 13% above the figure, so it was already low when written; v1.0.0 = 2026-04-03, v1.2.1 = 2026-06-06 (latest), the `gastownhall` org move and Kilo GA 2026-05-19 all confirm (census EXT-COMPETIT-018, api.github.com/repos/gastownhall/gastown, fetched 2026-09-10). This is Gas Town; Beads (gastownhall/beads, 27,041) is a different repo — the two figures were never in conflict.
- :74 — "Claude Code, Codex CLI, Cursor, OpenCode, Gemini CLI and Copilot … Gemini CLI via `gemini-extension.json` … ~162 skills … 'Aeon' … six harnesses (UNVERIFIED)" → Gemini CLI was replaced by **Google Antigravity CLI on 2026-08-18** (before this dossier's date) and `gemini-extension.json` is gone; the skill count is **183** (181 at the pre-Antigravity commit — no 2026 revision checked carries ~162); Copilot appears only as a gitignored `.copilot/` output directory; Aeon exists (aeonfun/aeon, 722★) and claims **nine** harnesses, not six (census EXT-COMPETIT-110, github.com/wshobson/agents AGENTS.md, fetched 2026-09-10). The AGENTS.md + SKILL.md convergence point stands.
