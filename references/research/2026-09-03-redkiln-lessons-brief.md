# Redkiln-RS Brief — Data Model, Best Parts, Lessons, Open Questions

## A. Redkiln's current concrete data model

**Hierarchy.** `.bklg/<initiative-slug>/` holds `initiative.md`. Under it, `<project-slug>/project.md` per project; under that, `<project-slug>/<story-slug>/{story.md, spec.md, plan.md, implementation-report.md, report.md, discover.md}` per story. Example fixture: `spikes/fixtures/e3-clean-fix-advance-write-lock-gate-scope/.bklg/support/{initiative.md, fix-advance-write-lock-gate-scope/{project.md, fix-advance-write-lock-gate-scope/{story.md, spec.md, implementation-report.md}}}`. A special `support` initiative (`initiative.md` `slug: support`) holds ad-hoc fix-lane stories outside a planned initiative — the `fix` skill's home.

**Item card frontmatter** (flat, scalar-ish YAML, present on every `.md` card — `id`, `uid`, `type`, `slug`, `title`, `parent`, `initiative`, `project`, `status`, `process`, `stage`, `automation`, `severity`, `blocked_by`, `blocks`, `owner`, `created`, `updated`, `links: {pr, commits, kb}`). Stories/projects add `terminal`, `tier`, `schema`, `process_rev`; stories add `gate_open` (bool — whether a human review gate is currently open on this item). Example, `story.md`: `id: RK-S0263`, `status: in-progress`, `process: story-lite`, `stage: implement`, `gate_open: false`. The **card body is deliberately empty/status-only** for stories — "the full specification lives in `spec.md` beside this file: it is the source of truth." One-write-path rule: each pipeline stage authors exactly one companion artifact.

**Companion artifacts per stage** (from `.redkiln/processes/story.yaml`): `discover.md` → `spec.md` (requires AC-### enumerated criteria, `requires_ac: true`) → `plan.md` → `implementation-report.md` → `report.md`, then `closeout` (`harvest_kb: true`, produces nothing). `spec.md`'s structure (seen in `fix-advance-write-lock-gate-scope/spec.md`): one-line summary, Context pack (file:line citations), Repro, Behavior, Frozen PR boundary (a file allowlist table — the fix may touch only these paths), Acceptance Criteria (`AC-###`, Given/When/Then + a `_Verification:_` clause naming the exact test), Error Conditions (`EC-###`), Non-Functional (`NF-###`), Merge gate, Anchors table (path / why load-bearing / when to open / which ACs it serves — progressive disclosure), Out of Scope. Other companion types referenced across the tree: `_review.md`, `_plan.md` / `_storymap.md` / `_decomposition.md`, `_ledger.md` (AC-row → implementation → verifying-test triples), `_integration.md`, `_retrospective.md`, `_design.md`/`_design-review.md`, `_intake-brief.md`, `closure.md`.

**Stage FSM** (`.redkiln/processes/story.yaml`, applies to `[story, bug, spike]`): lifecycle statuses `[pending, specced, ready, in-progress, in-review, done, blocked]`. Stages, in order, each with `status_on_enter`, `produces`, `template`, and a `gate`:

| stage | status_on_enter | produces | gate kind |
| --- | --- | --- | --- |
| discover | pending | discover.md | `checklist` (`gates/discover`) |
| spec | specced | spec.md | `review`, approver human, verdicts `[approved, changes-requested]` |
| plan | ready | plan.md | none declared |
| implement | in-progress | implementation-report.md | `command` — `{{redkiln}} verify --item {{id}} --grain story`, `blocking: true` |
| report | in-review | report.md | `review`, human, `[approved, changes-requested]` |
| closeout | done | (none) | `harvest_kb: true` |

One declared backward transition: `on: changes-requested, from: report, to: implement`. Telemetry emits `[stage_enter, stage_exit, gate_result, verdict]`. Other process packs exist for `bug`, `incident`, `initiative`, `project`, `project-lite`, `story-lite` (`.redkiln/processes/*.yaml`).

**`.redkiln/config.yaml` shape**: `version`, `prefix` (`RK`), `default_automation` (`HITL`), `active_branch_window_days`, `support_initiative: support`, `visualization: {default, html_out}`, `processes_dir`, `telemetry_dir`, `index_path` (sqlite), `auto_stage_telemetry`, a `verify:` block (`affected_gate`, `reachability_static`, `e2e` — shell commands, heavily commented with the historical reason each was wrong before), and a `design:` block (`capture` command with `{{out}}`/`{{surfaces}}` templating, `viewports`, `themes`, `require_design_review: true` — a red design review halts the project rather than merely degrading the verdict).

**`.kb/` layers** (seen at the fixture root): `.kb/concepts/`, `.kb/decisions/` (ADRs — "accepted" ones are immutable, only superseded), `.kb/playbooks/`, `.kb/maps/`, `.kb/open-questions/`, `.kb/_governance/integration-waves/`, `.kb/_templates/`, and an intake queue `.kb/_intake/` (consumed by `redkiln:kb-ingest`). Atoms are authored with a `KbFrontmatter` schema (per skill descriptions) and linked outbound-only.

**IDs**: `RK-I####` (initiative), `RK-P####` (project), `RK-S####` (story/bug/spike, shared prefix). Repo-wide id/write locks live at `<git-common-dir>/redkiln/`, i.e. **shared across every worktree of the same repo**, not per-worktree (the fixture's own spec.md is literally about that).

## B. "Best parts" — capabilities that must survive

| Capability | Why it must survive | Source |
| --- | --- | --- |
| Deterministic FSM/gate engine, invoked from outside the model | `redkiln advance` as a real external mechanism vs. shipmates' deleted prose-only FSM | shipmates §1 (L1.1–L1.3) |
| Stateless `PreToolUse` deny (predicate on tool input only, never on sequence position) | Survives shipmates' collapse because it never tried to encode stage order | shipmates §1, L1.2; day-one checklist |
| CI / deterministic gates as an authority independent of the orchestrator | shipmates' own enforcement collapsed; only CI+branch protection still protected the repo | shipmates L1.3 |
| Membership-may-union, completion-trusts-one-signal-class, drift typed not silently fixed | Matches `GateBasis` six-way vocabulary already converged on independently | shipmates L2.1, L2.6/2.7 |
| `reconcile(recorded, world) -> (State, Drift)` at top of every run; no `--resume` flag | One entry point; recorded state is a cache, git is truth | shipmates L2.2–L2.4; RFC §3.4; FINDINGS F19/H2-H3 |
| Gate bound to a SHA (`GreenAt(Sha)`), not a branch | avoids "worked on main earlier" laundering | shipmates L2.5 |
| Verdict as a structured type (`outcome, covered, gaps, gated_out`), never a bool | Matches Redkiln's own `_review.md`/gate-basis instinct | shipmates L2.6–2.7 |
| Two terminal states (paused vs. crew-complete) | prevents holding runs open on unrunnable-by-crew work | shipmates L2.8 |
| Rubric fixed before evidence seen (redkiln-review's rubric as input, not chosen by judge) | prevents post-hoc rationalization | shipmates L2.9 |
| Reproduce-before-fix as a typed precondition (`RedGreenProof`) | non-reproduction is terminal, not retried | shipmates L2.10 |
| Git commit trailers (`Story:`, `Slice-Verdict:`) as the durable resume substrate | survives deleted journals, machine changes, worktree removal; "git is truth" | RFC §3.4; shipmates L2.2; FINDINGS F12, F19 |
| Preflight agent's git archaeology, ported as deterministic Rust (not an LLM read) | removes an LLM from the resume decision entirely — "the #1 resume bug class is a non-deterministic gate predicate, not a flaky model" | RFC §3.4 |
| One-write-path rule: single source of truth per stage's artifact, story cards status-only | seen directly in the fixture's `story.md` vs `spec.md` split | fixture; redkiln:conventions |
| Frozen PR boundary + Anchors table in spec.md (progressive disclosure, explicit file allowlist) | keeps implementers in-scope, keeps context loading cheap | fixture spec.md |
| `--stay` semantics / irreversible-transition guarding, made structural not prose | commands/implement.md's `--stay` trap becomes unrepresentable once the orchestrator owns the loop | RFC §3.3, §4.1 |
| Verification-gated loops (tests/CI grounding, not model judgement) | the one orchestration pattern with convergent, compute-matched, multi-source literature support | spike charter §2 |
| `check-prompts.mjs`-class lints over the prompt corpus | does not port to Rust, does not become unnecessary | shipmates L5.9; day-one checklist |
| KB harvesting into durable atoms at closeout, ADR immutability (supersede, never mutate an accepted decision) | seen in kb-ingest skill description and `.kb/decisions/` layer | fixture; skills |

## C. Hard-won lessons and traps constraining the redesign

**From shipmates (build/packaging class):**
- Namespace every installed artifact from day one — retrofitting cost shipmates one release + 9 follow-up issues (L3.1).
- Receipts need a third state: own / **adopt** / refuse — "ours but unclaimed" must not be refused-forever (L3.2).
- Renames need a checked-in old→new migration table, applied new-then-delete, before the payload writes, rewriting sibling receipts too; no live stubs (L3.3).
- Embedded payload must be authoritative over a stray cwd checkout; loud warning on override (L3.4) — a Rust-specific trap since Redkiln-RS embeds its prompt corpus the same way.
- Integrity scans must be bounded to claimed paths and skip symlinks, or they become a DoS on `node_modules` (L3.5).
- Multi-target ops must continue past a single failure and summarize, never fail-fast on item 1 of N (L3.6).
- Never append to a user-owned file — own it or don't touch it; appending isn't idempotent or uninstallable (L5.1).
- A policy/flag's scope must be explicit to a caller, never inherited by whoever delegates to it (L5.2).
- A repair's trigger must be the literal absence of the end state, never a proxy for it, and must never write into the user's own checkout — only a dedicated worktree (L5.3).
- Every external side effect needs an idempotency key, or resume re-posts duplicate comments/PRs (L5.4).
- One counter cannot bound three unrelated retry budgets — separate budgets, separate exhaustion behaviors, each with a real number and a timeout ("a bound with no number is not a bound") (L5.5, L5.6).
- Harness capability claims decay in weeks; every one needs a date and a probe behind it (L5.7).
- Every derived/generated artifact is a permanent tax unless a check can prove it matches its source including counts/ordering/parity (L5.8).
- Don't ship an autonomy level you'll have to walk back (`epic merge auto` shipped and died in a day) — the human gate belongs at the integration artifact, not each unit (L5.10).
- Don't ship schema no adapter reads (`writes:` on all 13 personas, consumed by zero) (L5.11).

**From FINDINGS (measurement class):**
- **F17 — referee deletion / fixture defeat.** The E3 pilot reviewer had `Bash` in a worktree whose git object store still contained the "reverted" fix commit; it reconstructed provenance, restored the fix, mutation-tested it, and correctly approved — but this meant the fixture measured nothing about defect-detection. Fix: make the seeded defect state *unreachable* (shallow-fetch the fix's parent into a fresh repo), not merely absent from the working tree. Lesson for the FSM redesign: any adversarial/review gate given `Bash` in a real git worktree can always "cheat" toward ground truth by walking the object store — this is a property worth preserving for real reviews, but it means test fixtures for reviewer fidelity must deny that escape hatch deliberately.
- **F20, Defect 1 — a card's `tools:` allowlist silently voids `--json-schema`.** Structured output is delivered via a `StructuredOutput` tool call; a restricted `tools:` list that omits it makes the schema flag inert with *no error, no warning, exit 0* — the reviewer just writes JSON-shaped prose instead, which can silently not even match the schema. Any restricted card composed with structured output is affected and invisible unless the caller checks for `structured_output` specifically.
- **F20, Defect 2 — Windows extended-length paths break naive git shelling.** `canonicalize()` on Windows returns `\\?\D:\...`; slash-flipping it for git produces a URL-shaped `//?/D:/...` that git parses with host `?` — the fixture-build path never once succeeded until the prefix was stripped. A concrete Windows/Rust git-interop trap.
- **F8 — a default `-p` inherits the operator's whole environment** (12 MCP servers, ~280 tools, 86 slash commands measured) — 56% of cache-write tokens and ~a third of wall clock were unrelated tool definitions. `--strict-mcp-config` is required and must be *verified from the child's own `system/init`*, not trusted from the flag.
- **F7 — `--agents` is inert without `--agent`.** Passing a card alone runs a generic session at default effort while silently *appearing* to run the target persona — this would have produced a false-negative kill on E3 had it not been caught first.
- **F1/F6/F14 — headroom/rate-limit instrumentation is coarse.** `utilization` is quantized to 0.01; per-dispatch cost is below the instrument's resolution; only batch deltas are measurable. No Opus/Sonnet sub-limit key exists at all — the binding limit for Opus-heavy arms is invisible.
- **F3 — no 1h/5m cache-TTL split is exposed** in `usage`; F10 established the ~1h TTL only by direct behavioral observation (a decay test), not from any documented field — "the instrument is vindicated over the field."
- **F10/F13/F16 — cache topology is two segments**, not the all-or-nothing model the docs imply: a ~17.4k shared base (system prompt + tools) and a ~7.8k per-directory segment. A commit or a new worktree busts only the per-directory 31%, not the whole prefix — worktree fragmentation is real but "roughly a third the size claimed," and `--system-prompt` (replacing the volatile git-snapshot-carrying default prompt with static text) recovers the loss to 1.000 share entirely.
- **N1 / sequencing lesson — bash-defeats-fixture generalization.** The recurring shape across F17 and F20 is: *validate one item before spending on the whole set.* Both rig defects (F17, F20) were caught by single/small probes before the real experiment ran; had either reached the full E3 set, the entire Opus/xhigh budget would have bought nothing (F20 explicitly quantifies this).
- **Sequential vs. parallel / DAG execution.** shipmates computes a real dependency DAG in `/ship-epic` then flattens it to a linear order — "prose cannot schedule a wavefront" (shipmates L6.1). F16 prices the cost of doing better: parallel worktrees only cost ~31% of the prefix, not 100%, so DAG-antichain parallelism is cheaper than the RFC's own worst-case assumption, making it "the largest single capability gain available" per shipmates L6.1/RFC. But sequential slice execution within a project must stay strictly sequential — `forge-implement.js` is explicitly annotated "concurrent agents mutating the same tree corrupt it" (RFC §2), and per-story failure isolation (quarantine dependents, continue independent branches) is something shipmates never built (shipmates L6.2).
- **F18/F19/H2-H3 — human gates and cache/drift correlate at the same boundary.** ~48% of gates outlive the ~1h cache TTL; drift rate is 3.6× higher (11% vs 3%) once a gate exceeds an hour. Suspension itself is provably lossless (174/174 round-tripped through a fresh process), but the record can be faithful *and stale* — reconcile-on-resume is a correctness requirement, not an optimization, and at 7% overall drift with median 0 in-gap commits, reconciling eagerly on every resume costs almost nothing.
- **F11 — the merge gate's own false-negative rate is ~10%** (1/10 test-reachable mutants survived), so "tests green" is a meaningful but imperfect oracle; two arms within ~10 points of each other haven't been separated by this instrument.
- **F22 — background jobs are unreliable under the harness's own job tree** (killed at any duration/output pattern); only a detached `Start-Process` survived 12,000s through repeated watcher kills. Operational rule: nothing that must survive may depend on a background process surviving at all — express waits as a durable mark + later read, never as one process's lifetime.
- **F2 — hooks do fire in `-p`**, closing an open RFC question and validating that the stateless `PreToolUse` deny survives inversion unchanged.
- **F21 — the review arm's actual measured operating point** (FNR 0%, FPR 8.7%, MCC +0.917 across 46 dispatches) is far more sensitive than the literature's cited reviewer (FPR 0.2%/FNR 87.9%) — but this is a mid-programme result, not yet compared against an in-session baseline arm (a), so no thesis claim can be made from it yet.

## D. Explicit open questions the references leave unresolved

- **RFC §10 / FINDINGS**, still open even after F1–F22: **whether `--fork-session` reuses the parent's prompt cache** — undocumented, distinct from the in-session `subagent_type: "fork"` behavior that *is* documented.
- **Real binary size** for the eventual Rust crate stack — RFC calls this a 15-minute `cargo build --release && strip` task never actually run.
- **Whether `redkiln-design-review` retains fidelity headlessly** — needs `Bash` + screenshot capture + vision on the resulting PNGs; explicitly out of scope for the whole spike programme (charter §15), "recorded as an unmeasured risk."
- **YAML byte-parity (RFC R3 / Experiment 3)** — not run in the FINDINGS log at all; still an "unbounded" research risk gating Bet 2 (engine port), independent of Bet 1 (orchestrator inversion).
- **E3's actual gating comparison is still incomplete**: arm (a) — the in-session baseline — "does not exist" as of F21; nothing in FINDINGS yet fires the pre-registered kill criterion either way.
- **The two E3 false positives are unadjudicated** (F21) — Cohen's κ against blind human annotation per METHOD-E3 §7 has not been run; the co-primary FPR of 2/23 is provisional pending that adjudication.
- **E3 stability/variance is unmeasured** — no item has been run twice; "a 23/23 with no variance estimate is one sample of a stochastic process, not a constant" (F21).
- **Whether `--max-budget-usd` functions as a real per-invocation ceiling on a subscription** — charter §12 flags this as inference requiring empirical verification in E0; FINDINGS never confirms it.
- **The cause of background-job kills (F22/Operational entry) is explicitly not established** — only a workaround (detached process) is confirmed; three competing hypotheses were formed and two falsified, with the true mechanism still unknown.
- **Equivalent-seed triage (METHOD-E3 §10)** for the E3 corpus is "still owed" per F21 — not applied even though detection is currently 23/23.
- **Whether cache reads draw down the rate-limit window at all** (F14) — both "free" and "small but nonzero" readings remain consistent with the data; the confound of the *measuring* session sharing the same account/window is stated as unresolved and limits every headroom figure in the programme.
- **Whether `--system-prompt` also blunts *time*-based cache decay** (not just commit-based busting) — F18 flags this as a live design conclusion, not yet tested.
- **RFC R4/R6 mitigation conflict is called "unresolved and structural"** in the corrections table even after F16 answered the *cache-share* half — `--exclude-dynamic-system-prompt-sections` is still documented as a no-op under `--system-prompt`, so which discipline the shipped design should standardize on is not fully closed.
- **Charter's own pre-registration commit was, per FINDINGS' final "Open" entry, meaningless as of that log** — no arm was validly pre-registered until the spike tree itself was committed; whether that was subsequently fixed is not recorded in FINDINGS as read.
- **Multi-harness is explicitly out of scope for the whole spike** (charter §15, non-goal in §4) — one harness (`claude`) only; the multi-harness inline-dialect concerns raised in shipmates L4.8 are therefore untested against real headless dispatch.
