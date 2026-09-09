---
id: "0078"
title: The cache rule is a split rule; both TTL variables are set and verified per run; the scope is machine and directory
status: accepted
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §1 rows 1 and 4, §3 (context and caching), §4 recs 1–2; evidence brief §C.1
supersedes: ["0016"]
superseded_by: []
related: ["0010", "0017"]
evidence: ["F16", "F28", "F13", "F3", "F10", "F18", "S-WT", "S-H1b"]
consumers: []
---

## Context and problem

[0016](0016-static-system-prompt-carries-the-card.md) took three things as one. It chose a static `--system-prompt` to carry the card, it pinned `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` as *the* fix for TTL drift, and it justified pinning the CLI version on the ground that "the cache is org-scoped". The delivery choice stands. The other two clauses do not survive the evidence now in hand, and the way the three were bundled hides a structural fact about the cache that the design has been reasoning around rather than with.

**The mitigations are not one rule.** A static system prompt holds a 1.000 read share *across commits* (F16, 50 dispatches, within-arm range ≤0.001, against 0.689 for commit-each under the default prompt) and *across worktrees* (S-WT: w1 0.807 on the cold write, w2–w10 all 1.000). The same static prompt **forfeits the account-wide subsidy across time**: 29,619–29,680 tokens created at ~66 min at a read share of **0.000** (F28's published correction, S-H1b), where the default prompt costs a flat 18,603 at both 65.2 min and 49.7 h — identical to the token. Holding a prefix across space costs the thing that holds it across time. Treating that as one "cache rule" is what let 0016 read as though the S-WT result also bought durability; the evidence brief records the inference "held across commits implies helps across time" as explicitly falsified (§C.1).

**The TTL is bucketed, not a single value.** First-party documentation now describes two fixed buckets — main conversation (interactive turns, `-p` runs, Agent SDK) and everything else (subagents, workflows, teammates, forks, compaction) — with 1h as the default only for the main bucket, and only on a subscription drawing within plan usage. The second bucket defaults to 5m regardless of billing type, and usage-credit billing silently forces 5m even on an explicit 1h subagent request. Every `forge-*` pipeline this design ships is subagent and workflow traffic, so 0016's single pinned variable does not reach the dominant traffic pattern (evidence-augmentation §1 row 1, severity **high**; §3 context-and-caching).

**The scope sentence is wrong for the path this repo uses.** Isolation is workspace-scoped on the Claude API, Claude Platform on AWS and Microsoft Foundry; org-scoped only on Bedrock and Google Cloud. Claude Code then adds a tighter practical scope on top — "effectively scoped to one machine and directory" — because cwd, platform, shell, OS version and a git branch/status snapshot are embedded in the system prompt layer (evidence-augmentation §1 row 4, §4 rec 2). "Org-scoped" both overstates the subsidy's reach and points at the wrong mitigation: coordinating upgrades across an account is not the risk, and per-worktree cold starts are structural rather than a rig defect.

## Decision drivers

- The two mitigations point in **opposite directions**, and no single sentence can state both. A design that keeps them bundled will keep choosing one and claiming the other's benefit.
- Subagent and workflow dispatches are the dominant traffic this design generates; a TTL pin that does not reach them buys nothing where it is most needed.
- 45–48% of human gates already exceed one hour (F18, n=174/191, median gate→verdict 10.5 min, max 226 h), so which bucket a resume lands in is a first-order cost question, not a tuning detail.
- Setting a second environment variable is free and reversible; blocking it behind a measurement would be a cost with no matching benefit.
- The instrument and the documentation disagree about whether the TTL split is observable at all (F3), so any TTL claim asserted without a per-run check is unfalsifiable on this rig.
- Billing mode changes the answer silently. [0010](0010-claude-adapter-subscription-only-auth.md) fixes the adapter to subscription-only auth, which makes the plan-usage-versus-usage-credits distinction the live confound rather than the API-key one.
- A wrong scope claim licenses a wrong mitigation, which is worse than no claim.

## Considered options

### Option — keep 0016's single rule and single TTL variable
Leave `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` as the whole fix. Rejected: it addresses only the main-conversation bucket, so every subagent and workflow dispatch stays on a 5m default regardless of what the variable says. This is the high-severity finding in the pass, and the traffic it misses is the traffic this design is made of.

### Option — hold the TTL clause at `proposed` until the `ephemeral_*` contradiction is settled
Decide nothing about the TTL until a spike establishes whether the per-run verification fields exist on this instrument. Rejected: setting both variables is free, reversible and strictly dominates setting one, whatever the fields turn out to be. What the contradiction actually blocks is the *claim that the setting took* — so the check is named as a precondition on that claim, not on the setting.

### Option — abandon the static system prompt and take the time subsidy instead
Drop the card from `--system-prompt` to recover the default prompt's flat 18,603 across arbitrary gaps. Rejected here, and deliberately not decided: 0016's delivery clause rests on F16 and S-WT, which this record does not withdraw, and the append route's alternative remains unmeasured pending S-H1c. Naming the split is what makes this a per-workload choice later rather than a global one now.

### Option — restate the scope as "workspace-scoped" and stop there
Adopt only the platform-level correction. Rejected: workspace scope is the correct statement for the API, AWS and Foundry paths, but this repo drives Claude Code, where the machine-plus-directory scope is tighter and is the one that actually predicts behaviour — including two worktrees of the same repo never sharing a prefix.

## Decision

**(i) State the cache mitigation as two rules, never one.** A static `--system-prompt` is a mitigation *across space* — commits and worktrees — and a **forfeiture across time**. The default prompt is a mitigation *across time* and not across space. Any design document, spike pre-registration or activity card that cites one of these results must state which axis it is claiming, and may not carry a benefit from one axis to the other. Specifically: it is forbidden to infer durability across an idle gap from a hit rate measured across commits or worktrees.

**(ii) Set both TTL variables, and verify per run rather than assuming.** The runner environment sets `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and** `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h`. The pinned CLI version must be at or above the version that introduced these settings, and the pin is recorded with the measurement. Every cache measurement records **billing mode** (plan-included versus usage credits) as a declared confound, because usage-credit billing forces 5m even on an explicit 1h subagent request. Any claim that a 1h TTL *took* must be evidenced from the run's own `usage.cache_creation.ephemeral_1h_input_tokens` against `ephemeral_5m_input_tokens`; absent that evidence the run is reported as **TTL-unverified**, not as 1h.

**(iii) Restate the scope, and retire "org-scoped".** The cache is workspace-scoped on the Claude API, Claude Platform on AWS and Microsoft Foundry; org-scoped only on Bedrock and Google Cloud Agent Platform; and, in Claude Code, additionally scoped to **machine + directory + git status**, because cwd, platform, shell, OS version and a git branch/commit snapshot are embedded in the system prompt. The F28/S-H1b subsidy mechanism is real **within** that narrower boundary; its reach was overstated. Per-worktree cold first turns are therefore structural and are budgeted, not treated as a defect. The CLI-version pin survives on its own mechanism — a new version typically changes the system prompt or tool definitions, so the first request after an upgrade rebuilds from the top — not on the retired org-scope claim.

**(iv) Register the contradiction as open, with a named check.** Spike **S-TTL1** is registered as a one-dispatch check: on the pinned CLI version, with both variables set and billing mode recorded, dump the raw `usage` block verbatim and record whether `cache_creation.ephemeral_1h_input_tokens` and `ephemeral_5m_input_tokens` are present. Until S-TTL1 runs, clause (ii)'s verification requirement is a standing obligation that no run can currently discharge, and that state is reported rather than papered over.

**What 0016 keeps.** Its delivery clause — `--system-prompt` = the card body, static, for Phase 1, volatile input in the user message, append route retained as a config option pending S-H1c — is untouched. This is a **partial** supersession: 0016 stays `accepted`, and withdraws only its single-TTL pin and its org-scope justification.

## Consequences

**Positive**
- Subagent and workflow traffic — every `forge-*` pipeline — is no longer left on a 5m default by an unstated bucket rule.
- The two cache mitigations become separately citable, so a future workload choice (static prompt versus default prompt) can be argued on the axis that workload actually lives on.
- Per-run verification converts a TTL assumption into a measured fact or an explicit `TTL-unverified` flag, matching the treatment F6 already forced on utilisation figures with `resolution_limited`.
- Per-worktree cold cache becomes an expected, budgeted line rather than an anomaly to be chased in P8.
- The mechanism behind F13's two-segment prefix gains a candidate explanation, which makes the ~31% reprice of worktree cost intelligible rather than merely observed.

**Negative**
- Two environment variables and a billing-mode field must be threaded through every rig and runner configuration, and a wrong or missing pin is now a reportable condition rather than a silent one.
- Clause (ii) creates an obligation the instrument may not be able to meet: if S-TTL1 confirms the `ephemeral_*` fields are absent, every run is `TTL-unverified` indefinitely and the design carries an unfalsifiable setting.
- Restating the scope narrows the claimed subsidy, so any earlier cost estimate that assumed account-wide warmth is now an overestimate of the subsidy and an underestimate of cold-start cost.
- The split rule makes the static-prompt choice visibly conditional, which will reopen it for any workload dominated by long idle gaps.

**What this forbids**
- Forbids stating a single "cache rule": a citation of F16 or S-WT that does not name the space axis, or of F28/S-H1b that does not name the time axis, is a defect.
- Forbids inferring durability across an idle gap from a hit rate measured across commits or worktrees — this specific inference is falsified, not merely unsupported.
- Forbids setting `CLAUDE_CODE_PROMPT_CACHE_TTL` without also setting `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`.
- Forbids reporting any run as having a 1h TTL without the per-run `ephemeral_1h_input_tokens`/`ephemeral_5m_input_tokens` evidence; the correct report in its absence is `TTL-unverified`.
- Forbids recording a cache measurement without billing mode, and forbids comparing cache figures across runs whose billing mode differs or is unrecorded.
- Forbids describing the cache as "org-scoped" in any redkiln document, and forbids the mitigation that phrasing implies (coordinating CLI upgrades account-wide) as the answer to upgrade invalidation.
- Forbids treating a cold first turn in a fresh worktree as a rig defect, and forbids any budget that assumes two worktrees of one repo share a prefix.
- Forbids asserting that the `ephemeral_*` fields exist on this instrument, or that they do not, until S-TTL1 runs.

## Evidence and confidence

- [measured] **F16**, n=50 dispatches, within-arm range ≤0.001: steady read share — no-commit 1.000 · commit-each 0.689 · +`--exclude-dynamic` 0.846 · +`--system-prompt` **1.000** · worktree-each 0.685. Licenses "a commit costs 31% of the prefix and a static system prompt recovers it". Does **not** license any statement about elapsed time.
- [measured] **S-WT**, n=9 steady-state worktrees: w1 = 0.807 (cold write), w2–w10 = 1.000. Same scope limit; and the 1.000 was obtained with the prompt held static, which is exactly what the machine+directory scope predicts.
- [measured] **F28** with its published correction, and **S-H1b**: default prompt 65.2 min → 18,603 and 49.7 h → **18,603**, identical to the token; `--system-prompt` 66.8 min → share **0.000**, creation 29,680; S-H1b 1.5 min → share 1.000, 66.1 min → share **0.000**, creation 29,619. The correction is the load-bearing part — the flatness was **shared-prefix subsidy, not durability**. n=1 per cell; the 24 h read under the static prompt was **not taken**.
- [measured] **F10**, n=1 per gap: full hit at 45.1 min; 61.6 min → share 0.690 (31% expired). The TTL boundary is **bracketed in (45.1, 61.6) and was never located**. This is date-stamped platform behaviour, so it licenses "approximately one hour on the main bucket at this pin" and nothing about the other bucket.
- [measured] **F18**, n=174 gate→verdict and n=191 stage→verdict, one operator: median 10.5 min, 45% over 1 h; stage median 51.6 min, p90 ~9 h, max 226 h. Establishes that TTL-bucket membership is a live cost question, not a hypothetical.
- [measured] **F13**, 19/19 fresh repos identical: the prefix has two segments, ~17,439 shared plus ~7,827 per-directory.
- [documented, 1P] Bucketed TTL, the layered precedence chain, the 5m default for the non-main bucket regardless of billing, and the usage-credit override — evidence-augmentation §1 row 1 and §3, from `code.claude.com/docs/en/prompt-caching`, severity high, checked 2026-09-08.
- [documented, 1P] Scope: workspace-scoped on the Claude API / AWS / Foundry, org-scoped only on Bedrock and GCP, and "effectively scoped to one machine and directory" in Claude Code with cwd, platform, shell, OS version and git status embedded in the system prompt — evidence-augmentation §1 row 4 and §3, from `platform.claude.com` and `code.claude.com` prompt-caching. Also [1P] the corrected upgrade mechanism and timing (§1 row 7): auto-update applies on next launch, never mid-session, and the real exposure is a **resumed** long session reprocessing uncached — which is precisely [0017](0017-gate-record-is-token-tree-hash-deadline.md)'s human-gate resume path.
- [inferred] The embedded cwd/platform/shell/OS/git-status layer is a **candidate explanation** for F13's measured two-segment prefix (~17,439 shared, ~7,827 per-directory). The shapes agree — a machine-and-directory-varying block sitting inside an otherwise shared prefix — but nothing in this pass measured the correspondence, and no token-level attribution was attempted. Marked inferred, not proven; it is not licensed as the cause.

**The open contradiction, recorded rather than resolved.** The documentation says `usage.cache_creation.ephemeral_1h_input_tokens` and `ephemeral_5m_input_tokens` exist and are the way to verify which TTL took. **F3 measured them absent** from the `usage` block on this instrument — the reason charter E1's TTL-split metric was reported as untestable, and one of the pre-registrations the evidence brief lists as found broken (§E). These cannot both be true of the same instrument at the same pin. This record acts on the documented behaviour for the **setting** — because setting both variables is free and reversible whichever way the fields fall — and refuses to act on it for the **claim**, which is why clause (ii) makes verification a precondition and clause (iv) names S-TTL1 rather than assuming a resolution. A CLI-version skew between F3's pin and the version that introduced the settings is the obvious candidate reconciliation and is [inferred] only; F3's own pin was not re-read in this pass.

**What this record does not license.** It does not establish that a 1h TTL is in force on any run to date; no run has produced the field evidence clause (ii) requires. It does not locate the TTL boundary — F10 brackets it and stops. It does not quantify the forfeited subsidy for the static prompt beyond the two measured creation figures at ~66 min, and the 24 h static-prompt read was never taken, so the static arm's long-gap cost is bounded below by 29,619 and unbounded above. It does not withdraw 0016's delivery choice, and it does not validate the append route: **S-H1c remains unrun**. It does not prove the embedded system-prompt layer causes F13's segmentation. And per the research pass's own discipline, nine agents finding nothing against the bucketed-TTL documentation means "no current source contradicts it", not "confirmed" — the one source here is first-party vendor documentation dated the same day as this record.
