# Shipmates (saman-mb/shipmates) — Status Check, Sept 2026

Repo: https://github.com/saman-mb/shipmates | Site: https://saman-mb.github.io/shipmates/ | Docs: https://saman-mb.github.io/shipmates/docs/

## 1. Current status (2026-09-03)

- **Latest release: v0.1.19**, published 2026-09-01T20:51Z. Before it: v0.1.18 (2026-09-01T16:54Z), v0.1.17 (2026-09-01T16:44Z) — three releases in under six hours. (Source: GitHub Releases API.)
- **Cadence**: rapid, small patch releases (0.1.7 -> 0.1.19 across ~Aug 31–Sep 1, 2026 per CHANGELOG.md: https://raw.githubusercontent.com/saman-mb/shipmates/main/CHANGELOG.md), consistent with a single active maintainer. `Cargo.toml` on `main` still reads `0.1.12` (https://raw.githubusercontent.com/saman-mb/shipmates/main/Cargo.toml) — behind the tags; UNVERIFIED whether that's a real gap or a crawl-timing artifact.
- **Stars: 4. Forks: 0. Watchers: 0. Open issues: 67** (repo page). Contributor count did not resolve via automated fetch — UNVERIFIED, but the commit/release pattern strongly suggests a solo project.
- **Activity, last 60 days**: heavy — the whole visible CHANGELOG (0.1.7–0.1.19) sits inside roughly the last two weeks, issue numbers in the #300s-#390s, matching the brief's #150–#399 pointer.
- **Referee pivot: confirmed.** Issue **#1**, "Epic: Code-enforced FSM for /ship-issue — hard, hook-enforced gates" (opened 2026-07-24, closed 2026-07-29, `state_reason: not_planned`) is the referee epic: a `.shipmates/run-<issue>.json` state file, a `shipmates` verb CLI owning every state-changing git/gh op, and Claude Code hooks (`PreToolUse` deny-lists, `Stop` liveness, `SubagentStart/Stop` verdict stamping) meant to physically block illegal transitions. Its own text concedes the limits: hooks are "a referee, not a container" (Bash can still shell out) and "a referee, not a conductor" (the model still decides what's next) — full determinism was deferred to an "external-orchestrator path." Closed not_planned, not merged. (https://api.github.com/repos/saman-mb/shipmates/issues/1)
- **No successor orchestrator/runtime shipped.** The project pivoted instead toward: (a) portability across harnesses (opencode/Codex/Cursor/Gemini/Copilot/Windsurf/Zed exporters), (b) an epic-level workflow, **`/ship-epic`**, looping `/ship-issue` over an epic's stories in dependency order with pause/resume gates, and (c) GitHub-native tracking (sub-issues) instead of a bespoke engine. Real architectural shift — "enforce the FSM in the harness" to "track state in GitHub, let the model drive, review boards are the real gate" — but not a new orchestrator/daemon; state still lives in git/GitHub artifacts.

## 2. Community sentiment

**No discoverable Reddit, Hacker News, X, or blog coverage.** Multiple WebSearch queries ("saman-mb shipmates," "shipmates referee hooks," "shipmates rust claude code") returned only unrelated hits. At 4 stars / 0 forks this reads as pre-traction. UNVERIFIED / not found: any third-party praise or complaints. The only visible signal is the maintainer's own tracker (67 open issues), which is self-filed (issue #1 author association: OWNER) — no evidence of outside contributors filing issues.

## 3. GitHub Issues integration, human-in-the-loop, docs

- **`/plan-epics`**: turns a brief into GitHub epics + linked, labelled stories, filed in parallel by per-epic `product-manager` subagents; has a dry-run preview.
- **`/ship-epic`**: per CHANGELOG #388 (v0.1.17), attaches each story to its epic as a real **GitHub sub-issue** (the checklist stays only as progress copy); story membership = union of sub-issue graph + checklist, backed by new `shipmates-gh` ops (`issue.sub_issue_add/list/remove`).
- **`/ship-issue`**: 8-stage flow (Plan -> Design specs -> Isolate in worktree -> Build -> Self-check/CI gate -> Acceptance board -> Remediate -> Deliver); stops at the PR by default, `MERGE_MODE=auto` opts into hands-off merge.
- **Human-in-the-loop**: PR-gated by default. Read-only commands (`/pr-review`, `/harden`'s report mode) never touch the tree. `IS_SECURITY_SENSITIVE` stories force manual merge even under `/ship-epic`. Bounded retry loops escalate to the human rather than spinning forever.
- **Web UI/dashboard**: none — CLI + agent-file generator plus a static Jekyll docs/marketing site.
- **Docs site**: https://saman-mb.github.io/shipmates/docs/ — install, harness matrix, troubleshooting, architecture, per-agent/per-command pages.

## 4. `docs/COST.md` summary

Not a $/token model — a prompt-discipline doc. Six principles: (1) amortize fixed overhead, bundle only genuinely cohesive work; (2) value-gate reviewer "seats" — add a specialist only when their judgment can change the outcome, name gated-out seats explicitly; (3) route by difficulty — cheap model/effort for mechanical work, stronger reasoning for planning/architecture/security/acceptance; (4) cache-friendly prompts — stable instructions first, volatile runtime input last; (5) return decisions not transcripts — status/verdict + criterion findings only; (6) avoid paid repetition — re-run a check only when new information invalidates it. Also defines **Complexity-Based Tiered Execution** (Simple/Medium/High) scaling subagent count per story, and a carry-forward-ACCEPT rule on retry.

## 5. ship-epic -> ship-issue decomposition model

Tracked **in GitHub, not in-repo files**: `/plan-epics` creates an epic issue plus one linked, labelled sub-issue per story. `/ship-epic` loops `/ship-issue` over stories in dependency order, pausing on gates/failures with resumable state, then — once every checklist story lands — convenes a second, epic-scope **integration acceptance board** (PO + principal-engineer + conditional architect) against the combined epic PR head, judging cross-story coherence and epic-level value, not just re-approving already-approved units.

## 6. Best parts to steal / avoid

**Steal:** (1) Named, gated reviewer seats with explicit gate conditions and mandatory "why skipped" reporting. (2) Carry-forward ACCEPT on retry — only re-run a reviewer if the fix delta could invalidate their prior verdict. (3) Sub-issue-first epic tracking (native GitHub relationship, checklist as redundant view only) to avoid tracker/plan-file drift. (4) Honest capability labeling — a public harness matrix distinguishing runtime-verified vs. builds-but-unverified. (5) A second-order epic integration board that judges the combined artifact, not just each already-approved unit.

**Avoid:** (1) The referee/FSM detour itself — a hook enforcement layer the team's own design doc admitted couldn't deliver real isolation or determinism, abandoned after ~5 days; a caution against chasing "hard gates" before validating payoff. (2) Very fine-grained same-day patch releases (three in six hours) — fine for solo velocity, noisy as a maturity signal for adopters. (3) No visible external-contributor process yet (0 forks, no outside issues found) — nothing here proves the model holds up with multiple contributors. (4) Cargo.toml lagging release tags — a small hygiene gap worth avoiding.

UNVERIFIED throughout: exact contributor count, any third-party sentiment (none found), whether the Cargo.toml/tag mismatch is real or a crawl artifact.
