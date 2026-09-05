# GitHub Platform Research for a Rust Orchestrator (Sept 2026)

## 1. Sub-issues
GA feature (rolled out from private/public preview through 2025 into GA by early 2026). [Adding sub-issues](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/adding-sub-issues) states: **up to 100 sub-issues per parent**, **up to 8 levels of nesting**. Community discussion confirms archived issues still count toward the 100 cap ([#193327](https://github.com/orgs/community/discussions/193327)).

| Surface | Support |
|---|---|
| REST | `/repos/{owner}/{repo}/issues/{issue_number}/sub_issues` (list/add), plus parent lookup, delete, reprioritize. [Docs](https://docs.github.com/en/rest/issues/sub-issues) |
| GraphQL | Query/mutate via `Issue.subIssues`, `Issue.subIssuesSummary` (`total`, `completed`, `percentCompleted`), `Issue.parent`. Required `GraphQL-Features: sub_issues` header during preview; GA docs at [docs.github.com/en/graphql](https://docs.github.com/en/graphql). |
| gh CLI | Native sub-issue/type/dependency management added June 2026 ([changelog](https://github.blog/changelog/2026-06-10-manage-sub-issues-types-and-dependencies-from-github-cli/)); community extension `gh-sub-issue` predates this. |
| Cross-repo | Sub-issues can link across repositories within the same owner/enterprise (parent and child need not share a repo) — UNVERIFIED precise scoping rules; not explicitly limited to same-repo in docs, but org/enterprise install permissions gate visibility. |
| Projects v2 | Sub-issue hierarchy is surfaced in Projects (parent/child indentation in table/board, progress rollup badge on parent using the completed/total sub-issue counts). |
| Webhooks | `issues` webhook payload includes sub-issue events (`sub_issue_added`, `parent_issue_added`, etc., GA per the 2025-03 changelog: [API support for advanced search](https://github.blog/changelog/2025-03-06-github-issues-projects-api-support-for-issues-advanced-search-and-more/)). |

Progress rollup: parent issue shows an automatic completion percentage/checklist based on closed vs. total sub-issues — good fit for Initiative→Project→Story progress but capped at 100 children and 8 levels, so a very large backlog needs sharding (e.g., one Initiative issue per epic, not one root issue for everything).

## 2. Issue types (org-level custom types)
GA. [About the issue type field](https://docs.github.com/en/issues/planning-and-tracking-with-projects/understanding-fields/about-the-issue-type-field) / [Managing issue types in an organization](https://docs.github.com/en/enterprise-cloud@latest/issues/tracking-your-work-with-issues/using-issues/managing-issue-types-in-an-organization).

- Defaults: **Task, Bug, Feature** (editable/disableable).
- Org-wide (not per-repo), single-select per issue, up to **25 types** per org, each with name/description/color.
- REST API support added ([changelog, 2025-03-18](https://github.blog/changelog/2025-03-18-github-issues-projects-rest-api-support-for-issue-types/)); also settable/filterable via GraphQL and search qualifier `type:`.
- Distinct from labels: labels are repo-scoped, multi-select, free-form; issue type is one controlled org-wide taxonomy field, filterable in Projects/search like a first-class field, and shows in the Projects "Type" column automatically. For your Incident/Initiative/Bug/Spike classification, custom issue types are the more natural fit than labels, but the org is capped at 25 types total (shared across all classification needs), so reserve slots carefully if you also want types for Story/Project.

## 3. Issue dependencies ("blocked by")
GA as of Aug 2025 ([changelog](https://github.blog/changelog/2025-08-21-dependencies-on-issues/)), preview discussions [#162396](https://github.com/orgs/community/discussions/162396), [#165749](https://github.com/orgs/community/discussions/165749).

- UI: "Relationships" sidebar section, "Mark as blocked by" / "Mark as blocking."
- Limit: **up to 50 issues per relationship type** per issue.
- REST: `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocked_by` (API version 2026-03-10). [Docs](https://docs.github.com/en/rest/issues/issue-dependencies).
- gh CLI: `--blocked-by`/`--blocking` flags added June 2026.
- This is separate from sub-issues (hierarchy) — dependencies model sequencing/blocking edges, useful for your DAG of Stories with `depends_on`.

## 4. Projects v2
GraphQL-first automation surface; REST support is limited. [Using the API to manage Projects](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-api-to-manage-projects), schema at [docs.github.com/en/graphql/reference/projects](https://docs.github.com/en/graphql/reference/projects).

- Field types: text, number, date, single-select, iteration (`ProjectV2Field`, `ProjectV2SingleSelectField`, `ProjectV2IterationField`).
- Mutations: `updateProjectV2ItemFieldValue`, `addProjectV2ItemById`, `createProjectV2Field`, etc.
- Status updates: `ProjectV2StatusUpdate` object supports project-level (not per-item) status posts (health/summary), separate from item "Status" field.
- Webhooks: `project_v2_item` event now includes field-change diffs (previous/current value) per [2024-06-27 changelog](https://github.blog/changelog/2024-06-27-github-issues-projects-graphql-and-webhook-support-for-project-status-updates-and-more/) — usable for near-real-time sync instead of polling.
- Views: board, table, and **roadmap** (timeline) views; workflows (auto-add to project, auto-set field on close, etc.) configurable in UI, some automatable via Actions marketplace actions (e.g., "Update Project v2 Item Field").
- Sub-issue hierarchy: parent/child indentation renders in table view with rollup progress; roadmap view can show nesting.
- Limits: community reports **~1,200 items** hitting practical ceilings in some contexts ([#9678](https://github.com/orgs/community/discussions/9678)) — treat large Projects as needing pagination/sharding; exact hard cap is UNVERIFIED (GitHub doesn't publish one number, and it has moved over time).

## 5. Copilot coding agent, Agent HQ, third-party agents
- **Assign-to-Copilot model**: assign an Issue (or start from the Agents panel/VS Code); Copilot creates a `copilot/*` branch, runs in an ephemeral GitHub Actions sandbox, pushes commits, opens a **draft PR**, and awaits review. [About cloud agent](https://docs.github.com/en/copilot/concepts/agents/cloud-agent/about-cloud-agent), [Assigning and completing issues](https://github.blog/ai-and-ml/github-copilot/assigning-and-completing-issues-with-coding-agent/).
- Progress reporting: session/activity log on the PR + Agents session view; best practice is to give rich issue context, optionally via `.github/copilot-instructions.md`.
- Custom agents: `.github/agents/*.agent.md` (repo-scoped) plus personal agents, discovered at session start (Copilot CLI). AGENTS.md-style repo instructions supported.
- **Agent HQ**: GitHub's Feb 2026 multi-agent command center — Claude (Anthropic) and Codex (OpenAI) now run as first-class coding agents alongside Copilot, selectable from Issues/PRs/Agents tab/VS Code, **included with existing Copilot Pro+/Enterprise subscription, no extra cost** ([changelog 2026-02-04](https://github.blog/changelog/2026-02-04-claude-and-codex-are-now-available-in-public-preview-on-github/), [GitHub blog](https://github.blog/news-insights/company-news/pick-your-agent-use-claude-and-codex-on-agent-hq/)). Status: **public preview** as of Sept 2026. Google, Cognition, xAI cited as future partners (UNVERIFIED whether shipped by Sept 2026).
- Sentiment (from community/dev-blog coverage): liked — low-friction "just assign the issue," runs in normal CI sandbox so it respects existing lint/test gates, draft-PR-first keeps humans in control. Disliked/friction points reported in community threads: context loss on large repos, agent sometimes needs multiple iterations/re-prompts, cost/quota concerns under Actions minutes, and (per assign-to-Copilot discussions like [#167564](https://github.com/orgs/community/discussions/167564)) automation via workflow-triggered assignment is fiddly.

## 6. GitHub Agentic Workflows (gh-aw)
**Technical preview** as of Feb 2026 ([changelog](https://github.blog/changelog/2026-02-13-github-agentic-workflows-are-now-in-technical-preview/)), repo [github/gh-aw](https://github.com/github/gh-aw), docs at [github.github.com/gh-aw](https://github.github.com/gh-aw/introduction/overview/).

- Workflows are Markdown files (`.github/workflows/*.md`) with YAML frontmatter (triggers, permissions, tools, safe-outputs, engine) + natural-language body.
- `gh aw compile` compiles Markdown into a standard Actions `.lock.yml` that actually runs.
- Default posture: **read-only permissions**; privileged actions (opening issues/PRs/comments) go through **safe-outputs**, a separate, permissioned validation job — the agent itself never gets direct write scope. Strong pattern for a headless-agent-in-worktree design (agent proposes, a gated job executes).
- Pluggable engines: GitHub Copilot, Claude Code, OpenAI Codex, Gemini, Pi.
- Good fit for repo-level automation (triage, PR review, CI-failure investigation) but still preview-stage — expect breaking changes; not yet GA.

## 7. HITL primitives
| Primitive | Notes | Doc |
|---|---|---|
| Environments + required reviewers | Up to **6 reviewers** (users/teams), only one needs to approve; can block self-review; up to **6 deployment protection rules** per environment; supports custom third-party protection rules | [Managing environments](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment), [Reviewing deployments](https://docs.github.com/actions/managing-workflow-runs/reviewing-deployments) |
| Issue comments/reactions | Standard REST/GraphQL + webhook (`issue_comment`, `reaction`) — cheap async human-input channel (e.g., "reply here to answer the agent's question") |
| PR reviews / required reviews | Branch protection: required approving reviews, required status checks, CODEOWNERS |
| Discussions | Threaded Q&A, separate from Issues; good for open-ended design discussion, has GraphQL API |
| Issue forms/templates | YAML issue forms (`.github/ISSUE_TEMPLATE/*.yml`) give structured fields — usable to standardize Incident/Initiative/Bug/Spike intake |
| Task lists | Markdown `- [ ]` checkboxes in issue bodies auto-render as progress bars; **sub-issues are the GA replacement/superset** for the old "task list to issue conversion" flow |
| `gh` CLI | Full issue/PR/sub-issue/dependency/type management now scriptable (June 2026 update) |
| GitHub Apps | Bot identity, fine-grained permissions, short-lived installation tokens — the right identity model for your orchestrator (vs. a PAT) |
| Webhooks vs polling | Webhooks strongly preferred for an orchestrator: lower latency, avoids burning rate limit on polling; GitHub Apps get higher REST rate limits scaled by installation count |
| Rate limits | REST: 5,000/hr (user token) or higher for GitHub Apps (scales with installations); GraphQL: **primary limit ~2,000 points/min** (query cost based), secondary limits include **100 concurrent requests**, **90s CPU/60s wall** (60s of which may be GraphQL), **~80 content-creating requests/min, 500/hr**. [GraphQL rate limits](https://docs.github.com/en/enterprise-cloud@latest/graphql/overview/rate-limits-and-node-limits-for-the-graphql-api), [REST rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api) |

## 8. Markdown rendering
- **Mermaid**: natively rendered in Issues, PRs, Discussions, README/markdown files (not Wikis) via fenced ```mermaid``` blocks — no plugin needed. Some feature gaps (FontAwesome icons, tooltips, sequence-diagram actor popups); wide diagrams need `TD` orientation or subgraphs to avoid horizontal scroll. Known bug: `>` can get HTML-escaped in some comment-edit paths, breaking arrows — test before relying on it for machine-generated status diagrams. [Blog](https://github.blog/developer-skills/github/include-diagrams-markdown-files-mermaid/)
- **Task lists**: `- [ ]`/`- [x]` auto-progress-bar on parent issue/PR; largely superseded in intent by sub-issues but still works standalone.
- **Issue forms**: structured YAML fields (dropdowns, required text, checkboxes) — [syntax docs](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms).
- **SHA-pinned links**: standard GitHub blob URLs (`/blob/<sha>/path`) render inline previews/permalinks; commenting on a line auto-generates a permalink at the current SHA (press `y`).

## 9. Prior art: in-repo backlog ↔ Issues sync
- **git-bug** ([repo](https://github.com/git-bug/git-bug)): stores issues as git objects in the same repo (separate ref/tree, not visible files), bidirectional/incremental "bridges" to GitHub/GitLab/Jira, CLI/TUI/web UI, works fully offline then syncs. Pattern: source of truth lives in git-native objects, not the GitHub-hosted Issue.
- **Beads** ([gastownhall/beads](https://github.com/gastownhall/beads), Steve Yegge): git-tracked `issues.jsonl` as source of truth, SQLite as local read-model cache, background daemon exports DB→JSONL; explicitly built for AI-agent long-horizon memory. Open issue [#4214](https://github.com/gastownhall/beads/issues/4214) documents a real drift bug: `github sync --push-only` never clears per-issue dirty state, so every run re-pushes the *entire* backlog (O(N) API calls) — a concrete cautionary tale about naive dirty-tracking in a bidirectional sync.
- **Backlog.md** ([MrLesk/Backlog.md](https://github.com/MrLesk/Backlog.md)): plain Markdown files as the task store (human- and agent-parseable), with a GitHub Action variant syncing GitHub Issue create/edit/close events to Backlog issues. Known issue: MCP server writing to the main repo instead of an isolated worktree ([#558](https://github.com/MrLesk/Backlog.md/issues/558)) — a warning about tool boundaries when agents and sync daemons share a working tree.
- **Common failure modes across these tools**: (a) **drift** from unidirectional assumptions baked into a bidirectional tool (last-write-wins races between a human editing the Issue and an agent editing the in-repo file); (b) **edit wars** when both sides "own" the same field (e.g., status) without a clear tiebreaker; (c) **ID mapping fragility** — GitHub issue numbers are per-repo and can look mutable across transfers/migrations, so external tools must store the GraphQL **node ID** (stable) rather than the issue number as the durable foreign key; (d) **push amplification** — re-syncing the whole backlog instead of only dirty deltas (Beads' bug above) burns rate limit and creates a flood of no-op webhook/notification noise for humans watching Issues.

## Design implications
- **Authoritative split, not full duplication.** Let `.bklg/` (your richer planning artifacts — specs, briefs, ADR links, agent transcripts) be authoritative for *content and history*; let GitHub Issues/Projects be authoritative for *human-visible status and collaboration state* (comments, approvals, reactions). Sync status/labels/type one direction at a time per field, never let both sides free-edit the same field.
- **Mapping**: Initiative → parent Issue (type=Initiative) with a Project item; Project (feature) → sub-issue of Initiative (type=Feature/Task), also its own Project item; Story → sub-issue of Project, 1:1 with a worktree/branch and (optionally) the Copilot/Agent-HQ draft PR. This fits inside the 100-children/8-depth caps for realistic backlog sizes, but a very large program needs multiple Initiative roots.
- **IDs**: store GraphQL node IDs (not issue numbers) as your durable cross-reference key in `.bklg/` frontmatter; treat issue *numbers* as a display/URL convenience only.
- **Dependencies vs hierarchy**: use native sub-issues for containment (Initiative/Project/Story tree) and native issue-dependencies (`blocked_by`) for your Story DAG edges — don't try to encode DAG order via labels or issue body text.
- **HITL wiring**: use Environments+required-reviewers as the approval gate before a headless agent's changes merge (maps cleanly to "review/approval" stage); use issue comments/reactions for lightweight "answer the agent's question" loops; prefer GitHub App identity + webhooks over PAT + polling for the orchestrator's bot, budgeting GraphQL calls against the ~2,000 pts/min primary limit and the ~80/min content-creation secondary limit (batch mutations, avoid per-item chatty updates).
- **Sync engine discipline**: track dirty-state per field per item (not per whole backlog) to avoid Beads' O(N) resync bug; use `project_v2_item` webhook diffs and `issues`/`sub_issues` webhook events to drive incremental updates instead of periodic full re-pulls; write agent-authored status posts via gh-aw-style **safe-outputs** (a permissioned adjudication step) rather than giving in-worktree agents direct GitHub write tokens.
- **Feature maturity caution**: Agent HQ / Claude & Codex on GitHub and gh-aw are both **preview-stage** as of Sept 2026 — fine to design against, risky to hard-depend on for GA-quality stability; sub-issues, issue types, issue dependencies, and Projects v2 field/status APIs are GA and safe as load-bearing primitives.

Marked UNVERIFIED in this report: exact cross-repo sub-issue scoping rules; a hard numeric ceiling on Projects v2 item count; whether Google/Cognition/xAI agents had actually shipped into Agent HQ by Sept 2026.
