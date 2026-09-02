# Shipmates → Redkiln-RS — lessons learned

**Date:** 2026-09-01
**Subject:** `saman-mb/shipmates` @ `main` — crate `0.1.19`, MIT, ~5 weeks old, 13 releases, 195 closed / 86 open issues
**Companions:** `references/2026-08-31-rust-inversion-RFC.md` (the design analysis), `references/2026-08-31-rust-inversion-spike-charter.md` (the measurement programme), `references/competitor-analyses/shipmates/2026-08-31-shipmates-comparative-evaluation.md` (the evaluation this supersedes for build purposes)
**Purpose:** build guidance. What to copy, what to avoid, and what to do differently, grounded in shipmates' code and — more valuably — its mistakes.

---

## 0. Why shipmates is the right prior art

Our first read framed these as opposites: Redkiln a runtime state machine, shipmates a build-time asset compiler. That was accurate then and is wrong now, because **Redkiln-RS is both.** It will hold a canonical corpus of personas and workflows, render and install them into a harness, *and* own the execution loop. Shipmates has shipped the first half properly — seven harness targets, receipt-backed lifecycle, digest-pinned payloads — and has failed, expensively and in public, at the second.

Both halves are useful. The failure more than the success.

---

## 1. The headline: they built the referee, deleted it, and wrote down why

This is the most valuable single artifact in the repository, and it is a validation of the direction we are taking.

**Issue #1** — literally the first issue filed — was *"Epic: Code-enforced FSM for /ship-issue — hard, hook-enforced gates."* Its rationale:

> "Today /ship-issue's 'state machine with gates' is prompt-level… but nothing enforces any of it. There is no runtime engine, counter, or guard. So half of the README's 'tricks that make the loop hold together' are **aspirational, not technically true**: only the parts backed by a real external mechanism (git worktree, gh/CI, separate sub-agent contexts) are sturdy; stage order, the loop cap, and nit-filing are only as reliable as the model on the day."

The design was a `.shipmates/run-<issue>.json` state file, phases `INIT → PLANNED → BUILT → PUSHED → CI_GREEN → ACCEPTED → DELIVERED (+ ESCALATED)`, and `PreToolUse` / `Stop` / `SubagentStart` hooks physically blocking illegal transitions.

**The timeline:**

| Date | Event |
| --- | --- |
| Aug 4 | FSM state model + schema landed — **and reverted the same day** |
| Aug 5–11 | Built anyway and thoroughly: FSM engine, schema v2 with per-stage counters, enforcement gate core, per-harness hook shims, lifecycle-event capability matrix, SubagentStart role gate |
| Aug 6 | **Dropped Zed from supported harnesses** to preserve the architecture — "only support harnesses with a blocking pre-tool hook" |
| Aug 13 | `remove: drop FSM and harness hook subsystem (#258)` — **+897 / −6,928 lines across 77 files** |
| Aug 13 | Same day: `feat: receipt-backed install lifecycle without hooks (#259)` |
| — | Issue #1 closed as **not planned** |

PR #258's stated reason:

> "The FSM and hooks were intended to enforce command phases across seven harnesses, but **lifecycle APIs do not expose a reliable slash-command scope**. The result was cross-harness protocol code, stale-run behavior, hook registration mutations, generated payload maintenance, and **enforcement semantics that differed by harness**."

And the retreat position:

> "Shipmates' core workflows remain in Markdown commands and skills. **Repository CI and branch protection remain the delivery authority.**"

They knew in advance. Issue #1's own *Non-goals / honest limits* section, written before a line was built:

> "Hooks are a referee, **not a conductor**: the model still drives what to do next. **Full determinism (program-drives-model) is the external-orchestrator path.**"

**Three things follow for us.**

**L1.1 — Do not try to enforce workflow state from inside the harness.** A `PreToolUse` event sees a tool call. It has no reliable notion of "we are at stage 3 of this workflow." Nine days and 6,928 lines proved it, and a supported harness was sacrificed to an architecture that was then deleted.

**L1.2 — Our existing hook survives this critique, and it is worth understanding why.** Redkiln's `PreToolUse` deny is a *stateless predicate on the tool input*: does this path match `CANONICAL_ITEM_RE`, and is the edit text frontmatter-shaped? It never asks what stage we are in. Shipmates tried to enforce stage *order*, which requires session-scoped workflow state the hook API cannot provide. **The lesson is not "delete your hook" — it is "a hook can enforce a predicate on its own input, never a position in a sequence."** Keep the deny; do not extend it toward stage enforcement.

**L1.3 — Keep one authority outside your own orchestrator.** When shipmates' enforcement collapsed, what still protected the repo was CI and branch protection — mechanisms living entirely outside the agent system. Redkiln-RS will own the loop, but its own enforcement will have bugs. The deterministic `verify` gates and the repo's CI must remain independently authoritative, not merely things the orchestrator chooses to run.

**L1.4 — The conductor is still unbuilt.** Issue #13, the external-orchestrator epic, remains open with zero of five stories done. Its own re-scope comment records that its founding premise died between filing and scheduling: *"Verified against first-party docs today, that is no longer true. Every CLI target now spawns one natively."* There is no prior art here to copy for the loop itself. Only for everything around it.

---

## 2. Adopt: the epistemics

These are the durable ideas — true whether prose or code runs the loop, and the part of shipmates worth taking wholesale.

**L2.1 — Membership and completion have different provenance.** Their sharpest design idea:

> `STORY_MEMBERSHIP` = the **union** of the sub-issue graph and the body checklist. **Done-state is never read from the graph**: a story counts as done only from a ticked checklist line or a closed issue.

Membership may union several sources; completion trusts exactly one class of verified signal. Disagreements are *reported*, never silently reconciled. For us: an initiative's project set may be derived from several places; "project is done" comes from one gate result and nothing else, and a mismatch is a typed `Drift`, not a fix-up.

**L2.2 — Reconcile the world before acting, unconditionally.** `/ship-epic` Stage 0 rebuilds its "landed" set from **five sources**: ticked checklist lines, its own recorded `SHIPPED_STORIES:`, its own prose log, merged PRs whose bodies contain `Closes #n`, and mis-targeted merges into the wrong base. Sources 3–5 are forensic reconstruction — the orchestrator does not trust its own written state.

Encode as `fn reconcile(recorded: State, world: &Repo) -> (State, Vec<Drift>)` at the top of every run. **Recorded state is a cache; git is truth.** This is already Redkiln's instinct — the Preflight agent's git archaeology — but it should be deterministic Rust, not an LLM read.

**L2.3 — No `--resume` flag.** Verbatim: *"idempotent re-entry must work on a bare `/ship-epic <epic>` whenever this comment exists."* One entry point that reconciles and then proceeds. A resume mode is a second code path that will diverge.

**L2.4 — Re-derive the completion predicate from the world, not the log.** The strongest gate in their corpus is `/migrate`'s sweep: re-grep for the old pattern across the whole repo; it must come back empty bar explicitly-listed exclusions; any straggler returns to the transform stage. Don't ask "did I mark it done" — ask the world whether it is done. Generalise: `fn is_complete(&self, world) -> bool`, re-evaluated, never trusted from a record.

**L2.5 — Bind a gate to a SHA, not a branch.** `/release`: *"wait for CI to go green on that exact commit — the one that will be tagged… A tag on red is never created."* And the guardrail: *"CI at the release SHA is the gate, not 'it worked on main earlier.'"* Type it: `GreenAt(Sha)`, never `GreenOn(Branch)`. Redkiln's `verify` should carry the tree digest it judged, which it partly does via the memo key — make it explicit in the outcome.

**L2.6 — A verdict is not a boolean.** `/pr-review` carries coverage and gaps forward:

> "If a visual specialist could not actually render the change, carry its 'needs a human visual pass' flag into the output rather than implying the visuals were confirmed. The same rule holds for any reviewer whose inspection was partial — an ACCEPT/PASS never reads as covering ground the reviewer said it didn't see."

```rust
struct Verdict { outcome: Outcome, covered: CoverageSet, gaps: Vec<Gap>, gated_out: Vec<(Role, Flag)> }
```

This is the same insight as our `GateBasis` six-way vocabulary (`evaluated | memoized | warned | unverified | declared-absent | none`), arrived at independently. Two projects converging on "a pass is not one thing" is a strong signal.

**L2.7 — Negative coverage is a result, not an omission.** Every reviewer gated out is named *with the flag that gated it*. This is what makes a cost optimisation auditable instead of a silent quality regression. Cheap to implement, and it is the difference between "we skipped the security review" and "we can't tell whether we skipped the security review."

**L2.8 — Distinguish "paused" from "crew-complete."** They shipped `/ship-epic` pausing forever on owner-only remainders (DNS, registrar, deploy console) and had to add a **crew-complete terminal state** in 0.1.8: work the crew can never do is not a blocker, it is a handoff. An orchestrator with only one paused state holds runs open on work it cannot perform. Two terminal states, and only one prints a resume command.

**L2.9 — Fix the rubric before you see the evidence.** `/spike` sets its judging criteria in Stage 0, before any prototype exists. Prevents post-hoc rationalisation. Applies directly to `redkiln-review`: the rubric is an input to the judge, never chosen by it.

**L2.10 — Reproduce-before-fix as a typed proof.** `/fix-bug` Stage 0 is a hard gate: a failing test must exist *and fail for the real reason*, and non-reproduction is **terminal, not a retry** — *"never 'fix' an unconfirmed bug."* Encode as a transition precondition: `Fix` requires `RedGreenProof { failing_at: Sha, passing_at: Sha }`.

**L2.11 — A discovery output is a contract.** `/migrate`'s census: *"This census is the definition of 'done': every item on it must end migrated or explicitly excluded."* Every entry terminates as `Migrated` or `Excluded { reason }`. Nothing silently drops.

**L2.12 — Irreversibility is opt-in, by type.** Every mode default in their corpus is the reversible one: `MERGE_MODE=manual`, `PUBLISH_MODE=manual`, `MODE=report`, `RUN_TESTS=no` for fork PRs. And a trust boundary that deserves copying: running a fork's test suite executes untrusted code, so `isCrossRepository` from the API gates whether the sandbox runs at all.

**L2.13 — Flags gate seats, and one flag can carry different policy per context.** `IS_SECURITY_SENSITIVE` has three behaviours: in `/pr-review` it seats a security reviewer (you don't own the branch, so remediation isn't available); in standalone `/ship-issue` it seats nobody but forces manual merge; in epic-delegated `/ship-issue` it does neither, because the parent PR is the gate. One flag, three policies, selected by execution context — `fn board(flags, context) -> Vec<Role>`, table-driven.

---

## 3. Adopt: the packaging and lifecycle

**This is where all of shipmates' pain lives**, by every measure: most fixes, most open issues, an entire release (0.1.16) spent repairing the previous one, its own `doctor` subcommand, its own e2e job, its own epic. It is also the half Redkiln has never had to solve, because `/plugin marketplace add` is a git clone. The moment Redkiln-RS ships a binary, all of it becomes ours.

**L3.1 — Namespace everything on day one.** They shipped 20+ generically-named skills into a shared user-level directory — `polish`, `refactor`, `release`, `migrate`, `document`, `spike`, and a tool literally called **`gh`** — and had to prefix them all in 0.1.15. Cost: one release, three regressions in the next, six open follow-ups. **Free at t=0; ~9 issues at t=30d.**

**L3.2 — Receipts need a third ownership state.** Not just own/refuse. From #386 — *"doctor --fix cannot claim an unmanaged flagship (report-bug left stale on 0.1.15 upgrade)"*:

> A Shipmates file at a payload path **that no receipt claims is adopted** — backed up, rewritten from the payload and claimed. A file Shipmates does **not** own is refused instead, naming `--force`; third-party skills beside ours are untouched.

"Ours, but unclaimed" arises from a prior version, an interrupted install, or a rename. Fail closed on it and you strand users on stale content forever. Design **adopt** as a first-class operation.

**L3.3 — Renames need a checked-in migration table.** The sharpest paragraph in their repo, from #373:

> "A rename that only changes `name:` and hopes install is enough will leave two skills on disk… **The product break is /polish disappearing. The CLI break is a tree that still has polish and shipmates-polish. The second one is worse.**"

Their design: a committed old→new table, run on both `install` and `doctor --fix`, **before** the new payload is written, ordered so the new path is valid on disk before the old is removed, rewriting every receipt that listed the old path *including sibling targets sharing a directory*. Plus: **no live stubs** — a stub skill is still a live name in the picker, which is the collision you were trying to kill.

**L3.4 — The embedded payload is authoritative.** #385 — *"Released install silently uses cwd commands/ over the embedded payload."* Their source-resolution order put the current directory ahead of the embed, so a user standing in a stale clone silently got that clone's prompts. Fixed to: the compiled-in payload wins; a filesystem override requires `--from-cwd` / an env var; a stale checkout gets **one loud warning**. Related and still open: #394 (precedence still unsettled), #231 (`build.rs` embedded `__pycache__/*.pyc` and broke `cargo build`).

This is a Rust-specific trap and we will hit it, because Redkiln-RS will embed its prompt corpus exactly this way.

**L3.5 — Bound every integrity scan to the paths you claim.** 0.1.16: *"the unmanaged-file scan is bounded to the payload's own subtrees, skips symlinks outright, and never walks `node_modules`."* The bug title says it: *"install --harness opencode aborts on node_modules/.bin symlinks (v0.1.15 upgrade stuck)."* A safety scan that walks the whole tree eventually meets a symlink farm and becomes a denial of service.

**L3.6 — Multi-target operations must tolerate partial failure.** Same release: `--harness all` now *"continues past a failed harness, prints a per-harness summary and exits non-zero."* Fail-fast on target 1 of 7 is not a safety property.

**L3.7 — Name the crash-debris convention with the write path.** 0.1.13 taught `doctor --fix` to restore *"from a sibling `{name}.bak-<secs>-<pid>-<n>` left by an interrupted install."* They had to teach the repair tool to recognise its own installer's debris after the fact. Design the naming when you design the write, and make repair a first-class consumer of it. (Redkiln already half-does this — `doctor`'s `STRAY_TEMP_RE` matches the `.tmp` suffix deliberately, *"so an interrupted write leaves a file the product can tell you about rather than a mystery."* Carry that forward.)

**L3.8 — Test the upgrade path from the previous published version, end to end, before shipping.** They added a CLI e2e job at 0.1.5, after **five blind releases** — and every escape since (#384, #386) has been an upgrade-path escape found by the owner upgrading, not by a test. The upgrade path is simultaneously the least-tested and most-broken surface in this class of product.

---

## 4. Adopt: persona and corpus authoring

Redkiln-RS needs agent cards both as installed files and as inline `--agents` JSON at dispatch. Shipmates' schema is a good starting point.

**L4.1 — The schema is small: three required, five optional, all flat scalars.**

Required: `name` (also a filesystem path — validate as a security boundary; their test is `name: ../escape`), `description` (one line, doubling as the *delegation trigger*: "Use for…", "Use before opening a PR and again against the pushed PR head"), `capabilities` (a fixed semantic set: `read, edit, bash, web, agent`).

Optional: `read-scopes` / `web-scopes` (narrow a capability below its default fan-out), `tool-order` (explicit override, used once), `effort` (`low|medium|high` — the cross-harness intersection).

**L4.2 — Never put a model in the canonical artifact.** #205, verbatim:

> "**Portability** — harnesses offer different model sets. `model: opus` is meaningless on Codex, Antigravity, or opencode… **Availability** — even on one harness, which models a given user can access varies by plan/time. A baked model may name something the user can't run."

Model is a spawn-time decision; effort is a static per-role property emitted only where the target's format carries the key. **That split is exactly the seam our inline dispatch needs.** Generalise it: no fast-moving external identity belongs in a canonical artifact.

**L4.3 — Capabilities are semantic; the adapter maps them; unknown values fail the build.** Their `capability_registry.json` has a good two-level shape: a flat `scopes` leaf map (canonical scope token → that harness's tool name), a `tools` capability→scope fan-out, a `permission_model` (`allowlist` / `permission-map` / `none`), and dated `notes` carrying the evidence. Unknown scope → `bail!`. Unknown capability → `bail!`. Multiple argument names → `bail!`. **Nothing degrades quietly.**

**L4.4 — Least privilege is the adapter's job, and the mechanism differs per target.** opencode defaults to `"*": "allow"`, so its adapter emits `"*": deny` *first* and specific allows after — that harness resolves last-match-wins, so the ordering is the mechanism. Claude Code, by contrast, **inherits every tool when `tools:` is absent**, which makes an empty mapping fail *open*. Their Copilot adapter refuses that case with an error; their Claude Code adapter does not. Ours must.

**L4.5 — The persona body template.** ~2k characters, five parts, no markdown headings: a marker for the shared preamble; one identity sentence *with a negative boundary* ("you review structure — not line-by-line style, and not 'does it run'"); five to seven bolded heuristic bullets; an explicit `Return format:` paragraph naming literal field tokens; a closing verdict enum plus capability boundary ("You verify; you do not edit code" / "You do NOT commit, push, or open pull requests — the orchestrator owns git").

**L4.6 — Personas are leaves; the orchestration graph lives above them.** No `depends_on`, no `delegates_to`. Roles stay independently installable and the wiring is in the command layer. Correct, and it is what lets the same corpus serve both install-time rendering and dispatch-time injection.

**L4.7 — Record gaps rather than faking keys.** Codex has no per-agent tool allowlist; Copilot and Windsurf have no effort key; the shared-tree harnesses cannot hide a skill from manual invocation. Every one is written down in the adapter *and* in the registry `notes`, with a verification date. This discipline is why their honesty claims are credible.

**L4.8 — Watch for the inline-dialect trap.** `{{agents-glob}}` renders to `.claude/agents/*.md`, and personas instruct the model to resolve roles by looking there. Under `--agents` there are no files on disk. We need a fifth dialect whose agents-glob describes "the agents passed on this invocation," or our inline cards will tell the model to look for files that do not exist.

---

## 5. Avoid: traps with evidence

**L5.1 — Never append to a user-owned file.** They shipped contributor steering merged into the harness's `AGENTS.md`, filed the reversal immediately, and shipped a migration to undo it — all within 72 hours. Appending is not idempotent, not uninstallable, and forces you to write an un-doer. **Own a file or don't touch it.**

**L5.2 — A policy is scoped to a caller, not to a flag.** 0.1.18: a security flag on one story deadlocked the whole epic loop, because a rule written for the standalone command was inherited by the delegated one. When A delegates to B, B's safety defaults are wrong by construction. Any per-item policy needs an explicit "who is the gate for this run" parameter. Note they then needed 0.1.19 to fix the *documentation* of the contract, because the prose still described it from the wrong side.

**L5.3 — A repair's trigger is the absence of the end state, never a proxy for it.** GitHub refuses a PR when head == base, so 0.1.18 added an empty kickoff commit when *branch tips were equal*. Wrong condition — and it committed into the user's own checkout. 0.1.19 fixed both halves: trigger on *no PR exists*, and pin the write to a dedicated worktree. **The orchestrator never writes to the user's checkout. Ever.**

**L5.4 — Every external side effect needs an idempotency key.** Issue #267's thread contains the same "Ship Issue — Done" report posted three times by their own orchestrator. If a run can resume, every comment, PR, label and issue it creates needs a key.

**L5.5 — One counter cannot bound three budgets.** Their `MAX_FIX_ROUNDS = 3` simultaneously bounds rebase/sync conflicts, CI red-fix rounds, and review-rejection rounds. A run that burned two sync conflicts gets one CI fix attempt. Three budgets, three exhaustion behaviours.

**L5.6 — A bound with no number is not a bound.** `/release` Stage 3 says "bounded" and names no value. `/spike` declares `TIME_BOX` as a config key with no value and no enforcement — *"keep each spike minimal"*. And **there is not a single timeout in the entire corpus**: every CI gate is "poll until done", so a permanently-pending check hangs forever. Prose cannot hold a clock. We can.

**L5.7 — Harness capability research has a shelf life of weeks.** #204's eight-harness research table was invalidated by its own follow-up comment. #13's founding premise died between filing and scheduling. Gemini was renamed to Antigravity; Windsurf became Devin Desktop (#168 — *"is .windsurf/ still right?"*). Put every external-world assumption behind a **dated, testable** capability registry, and treat a capability claim with no probe behind it as a bug.

**L5.8 — Every derived artifact is a permanent tax.** Roughly **one commit in fourteen** exists only to re-sync payload digests, generated pages, demo GIFs, posters, or hard-coded file counts. Worse is the class CI cannot see: a tool shipped with correct syntax everywhere and *silently dropped another tool's card from the homepage*, and CI passed — *"the gap is convention + parity, not syntax."* Ask of each generated artifact: can a check prove this matches its source, **including counts, ordering and parity with siblings**? If not, don't generate it. Where you must, make the *count* the assertion.

**L5.9 — The prose that steers your agents is an artifact, and it drifts.** #112, still open: *"nothing validates README/CONTRIBUTING prose against the skills — three defects shipped through that gap in one PR."* Concretely: their `AGENTS.md` and `CONTRIBUTING.md` still document a neutral dialect (`agent-files/*.md`, `Harness-Session`, `{{argument}}`) that `render.rs` does not implement and whose absence its tests actively assert. **We already do better here** — `check-prompts.mjs` refuses a prompt citing a nonexistent path, agent, stage or tool grant. That checker does not port to Rust and does not become unnecessary. Keep it.

**L5.10 — Don't ship an autonomy level you will have to take away.** They shipped `epic merge auto` and removed it one day later. The human gate belongs at the *integration* artifact, not at each unit.

**L5.11 — Don't ship dead schema.** `writes:` is declared on all thirteen personas and read by zero adapters. If a field has no consumer, it is documentation pretending to be a contract.

---

## 6. Go further than they did

**L6.1 — Execute the DAG. They flatten it.** `/ship-epic` computes a real dependency graph over stories, runs a topological sort, and then uses the result **solely to produce a linear order** — independent branches ship one after another. That is not a considered choice; prose cannot schedule a wavefront. Their own `/spike` and `/migrate` demonstrate the safe concurrency primitive — *one worktree per independent branch of work, disjoint file ownership, join at a verified merge point*.

We already compute an initiative → project → story DAG. A code orchestrator can execute its antichains in parallel and serialise only at merge. **This is the largest single capability gain available, and it is unavailable to any prose orchestrator.** Caveat from the charter: worktrees fragment the prompt cache, so parallel width has a measurable cost — E1 and E7 should price it.

**L6.2 — Add per-story failure isolation. They have none.** `MAX_FIX_ROUNDS` exhaustion on one unit pauses the whole epic; the unit is not skipped, deferred, or marked failed-and-continue. With a DAG you can mark a story `Failed`, quarantine its dependents, and continue every independent branch, delivering a partial initiative with an explicit failure manifest. Defensible for prose (a paused epic is legible, a half-failed one isn't); strictly worse for us.

**L6.3 — Instrument the cost doctrine from commit one.** `docs/COST.md` is a genuinely good six-principle doctrine, spliced into fourteen commands, with tiered execution built on top — and **issue #187, "Per-run cost report — make token/agent spend visible", is open**, as is its parent epic #179. They have no idea whether any of it saved a token. We are about to make the same mistake for the same reason (our telemetry records output tokens only). Owning the spawn makes per-seat-per-run accounting nearly free. Do it first, not last.

**L6.4 — Make the cache contract a type, not an authoring rule.** COST.md states it well:

> "Put stable instructions and role context first. Put invocation arguments, issue bodies, diffs, and other volatile material in one runtime-input section at the bottom… do not weave volatile issue text, arguments, diffs, or generated output through this prefix."

That is prompt-cache architecture expressed as prose discipline. In Rust it is `struct Prompt { stable: StablePrefix, volatile: RuntimeInput }` with one assembly order and no way to interleave.

**L6.5 — Per-persona digests, not whole-payload only.** Their drift detection is whole-payload: a one-word change to `architect.md` fails the digest for every target and forces a seven-file update commit. Loud and safe, but it cannot distinguish "this agent card changed" from "the renderer changed" — and if we also inject personas inline at dispatch, we need that distinction.

**L6.6 — Close the crew-complete gate gap.** Their `/ship-epic` skips the integration board when an epic terminates via crew-complete, so a captain-facing PR can reach a human without ever receiving the cross-story review. Full closure and crew-complete should share the integration gate; only the closure *action* should differ.

---

## 7. What the tempo tells us

Thirteen releases, all dated 31 Aug – 1 Sep. Six on one day, seven the next. Auto-release fires on a version bump to main, and a bump is mandatory in the same PR when work is release-affecting — so **release cadence is an artifact of merge cadence, not a decision.**

Read against content, that burst is one architectural change (#373's namespacing) plus its own blast radius: 0.1.16, 0.1.18 and 0.1.19 exist mostly to repair 0.1.15, 0.1.17 and 0.1.18. Fix-to-feature runs about **0.9 : 1** across ~140 commits. And every escape was found by the owner dogfooding, not by a test.

**Copy:** the write-it-down-before-you-build-it discipline. Issues #373, #325 and #205 are better specs than most funded projects produce, and they are why the corrections are *deep* — identity, work-graph representation — rather than cosmetic. Copy also the acceptance-board-nits-as-issues valve, which is a legitimate tempo mechanism (~25 of their open issues are deferred board nits).

**Don't copy:** shipping a rename to users before the upgrade path has an end-to-end test, and auto-publishing every merged fix as a version.

---

## 8. Day-one checklist for Redkiln-RS

Ordered so that the expensive-to-retrofit items come first.

**Identity and packaging**
- [ ] Namespace every installed artifact from the first commit (L3.1)
- [ ] Receipts with per-file SHA-256 and **three** ownership states: own / adopt / refuse (L3.2)
- [ ] A checked-in rename table, applied before the new payload is written, new-then-delete, updating sibling receipts. No live stubs (L3.3)
- [ ] Embedded payload authoritative; filesystem override explicit and loud; filter the `build.rs` embed (L3.4)
- [ ] Crash-debris naming designed with the write path; repair is a consumer of it (L3.7)
- [ ] Integrity scans bounded to claimed paths; skip symlinks (L3.5)
- [ ] Multi-target ops continue past failure, summarise, exit non-zero (L3.6)
- [ ] E2E test of the upgrade path from the previous published version, gating release (L3.8)

**Corpus and dispatch**
- [ ] Persona schema: three required, five optional, flat scalars; `name` validated as a path (L4.1)
- [ ] No model in canonical frontmatter; effort only where the target carries it (L4.2)
- [ ] Semantic capabilities; adapter maps; unknown value → build failure (L4.3)
- [ ] Empty tool mapping is an **error**, never an empty allowlist (L4.4)
- [ ] An `INLINE` dialect for `--agents` dispatch (L4.8)
- [ ] Per-persona digests alongside whole-payload digests (L6.5)
- [ ] `Prompt { stable, volatile }` as a type (L6.4)

**Loop correctness**
- [ ] `reconcile(recorded, world) -> (State, Vec<Drift>)` at the top of every run; no `--resume` flag (L2.2, L2.3)
- [ ] Membership may union; completion trusts one signal class; drift is typed (L2.1)
- [ ] Completion predicates re-derived from the world (L2.4)
- [ ] Gates bound to a SHA (L2.5)
- [ ] `Verdict { outcome, covered, gaps, gated_out }` — never a bool (L2.6, L2.7)
- [ ] `enum PauseReason` and `enum TerminalState`, with crew-complete distinct from paused (L2.8)
- [ ] Separate budgets for sync / gate / review retries, each with a number and a timeout (L5.5, L5.6)
- [ ] Idempotency keys on every external side effect (L5.4)
- [ ] Per-story failure isolation; quarantine dependents; continue independent branches (L6.2)
- [ ] Execute DAG antichains in parallel, serialise at merge (L6.1)
- [ ] The orchestrator never writes to the user's checkout (L5.3)

**Discipline**
- [ ] Per-seat, per-run token accounting from the first dispatch (L6.3)
- [ ] Dated capability registry with a probe behind every claim (L5.7)
- [ ] Keep `check-prompts.mjs`-class lints over the prompt corpus — they do not port and do not become unnecessary (L5.9)
- [ ] Keep CI and the deterministic gates as authority independent of the orchestrator (L1.3)
- [ ] Keep the `PreToolUse` deny as a stateless predicate; never extend it toward stage enforcement (L1.2)

---

## 9. Sources

`saman-mb/shipmates` @ `main`: `AGENTS.md`, `CONTRIBUTING.md`, `docs/COST.md`, `crew/*.md` (13), `commands/{ship-issue, ship-epic, pr-review, shipmates-fix-bug, shipmates-release, shipmates-spike, shipmates-migrate}.md`, `steering/shipmates.md`, `toolbox/shipmates-{scrub,pixelart}/`, `src/{main, catalog, manifest, embedded, digest, doctor}.rs`, `src/adapters/*`, `src/installer/*`, `build.rs`, `tools/{manifest, capability_registry, harness_matrix}.json`, `tools/e2e_cli.sh`, `.github/workflows/*`, `CHANGELOG.md` (0.1.7–0.1.19), issues #1, #13, #112, #150, #152, #154, #164, #168, #187, #189, #203, #204, #205, #231, #258, #259, #267, #273, #295, #298, #308, #315, #320, #325, #351, #352, #373, #379–#399, and ~140 commit subjects.

Redkiln @ `0.20.0`: `src/hook.ts`, `src/process/gates.ts`, `src/migrate/health.ts`, `workflows/_lib/{forge-lib.js, tiers.mjs, check-prompts.mjs}`, `references/MULTI-HARNESS.md`.
