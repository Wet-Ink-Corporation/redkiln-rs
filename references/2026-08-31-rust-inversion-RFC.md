# RFC — Redkiln in Rust, with inverted control

**Date:** 2026-08-31
**Status:** Evaluation. No decision taken. Nothing here is scheduled.
**Scope evaluated:** the full port — orchestration *and* engine — into one Rust binary that drives `claude` headlessly, per the question asked.
**Method:** teardown of `src/` (40,076 LOC / 100 files), `workflows/` (13,439 LOC / 8 workflows), `commands/` (2,028 lines / 7), `agents/` (324 lines / 4), `test/` (137,873 LOC / 237 files + 22 helpers); Claude Code headless/caching/subagent docs; crates.io + docs.rs + repo evidence for 40+ crates; competitive scan of the orchestrator and spec-driven tool space.
**Evidence discipline:** every claim below is marked **[measured]** (counted in the tree or read off an API), **[documented]** (stated in first-party docs, cited), or **[inferred]** (my reasoning, could be wrong). Unknowns are named as unknowns.

---

## 0. The short answer

The full port is **two bets wearing one coat**, and the evidence separates them cleanly.

**Bet 1 — invert control: Rust owns the loop and spawns `claude -p` per step.** Better supported than I expected. Three findings move it from "interesting" to "probably right": the cost argument runs *opposite* to intuition [documented]; roughly two-thirds of the workflow layer is compensation for the Workflow runtime's sandbox and disappears rather than ports [measured]; and ~50% of the command layer is orchestration logic currently living as prose the model must re-execute correctly on every run [measured].

**Bet 2 — port the engine (`src/`) to Rust too.** Materially weaker. It carries one *unbounded* risk (YAML byte-parity against `eemeli/yaml`, which gates every write in the product), one *permanent* loss (the "clone the repo and it works on any platform" plugin property, which has no substitute for a compiled binary), and ~47k LOC of internal tests to rewrite — for benefits that are real but mostly ergonomic.

**They are separable.** Bet 1 does not require Bet 2: the existing CLI already presents a clean, tested process boundary with meaningful exit codes, and a Rust orchestrator can shell out to it exactly as the commands do today.

**Recommendation: take Bet 1, stage Bet 2 behind two spikes, and do not give up the plugin surface in either case.** The market evidence in §7 is unambiguous that pure standalone binaries with no harness surface do not get adopted, and the two projects in this space that *do* get adopted ship both.

---

## 1. What is actually being proposed

Today the control flow is:

```
human types /redkiln:implement
  └─ Claude Code session  ← the process
       ├─ reads commands/implement.md (prose; owns every state transition, every human gate)
       ├─ Workflow tool → forge-implement.js  (sandboxed JS; no fs, no modules)
       │    └─ agent() → Task subagent  ×N
       └─ Bash → redkiln advance ...       ← deterministic FSM, called BY the agent
```

Inverted:

```
human types redkiln implement <id>
  └─ redkiln (Rust)       ← the process
       ├─ owns phases, gates, retries, budget, worktrees, transitions
       ├─ spawns `claude -p --agents … --model … --cwd <worktree>`  ×N
       └─ engine::advance() in-process (Bet 2), or `redkiln-ts advance` subprocess (Bet 1 only)
```

The load-bearing change is **who holds the loop**, not the language. Everything in §3–§6 follows from that.

---

## 2. Target architecture

Six components. Only two are new.

| Component | Source today | Under inversion |
| --- | --- | --- |
| **Engine** — FSM, gates, telemetry, index, validation | `src/` (40k LOC TS) | Bet 2: Rust modules called in-process. Bet 1: unchanged, invoked as a subprocess. |
| **Orchestrator** — phases, sequencing, retry, reconcile, degraded, budget | `workflows/forge-*.js` (13,439 LOC) + `commands/*.md` (~50% of 2,028 lines) | Rust. This is the port. |
| **Prompt corpus** — every `agent()` prompt body, the two shared rules, the four agent-card bodies, the presentation prose | string literals welded into `forge-*.js`; markdown in `commands/`, `agents/` | External, versioned, digest-pinned files. **This separation is itself a benefit** (§4.5). |
| **Step runner** — spawn, stream, parse, cancel, attribute cost | `agent()`, a Workflow-runtime primitive | Rust, over `claude -p --output-format stream-json`. New. |
| **Run record** — resume state, baseRef, per-step outcomes | git commit trailers + the model's conversation context | git trailers (**keep**) + a durable run record (new). |
| **Human gates** | `AskUserQuestion` inside a live session, 20+ sites | Suspend-and-resume against the existing `gate_open` / `transition_open`/`close` model. New UX. |

### The step runner contract

`agent(prompt, opts)` takes exactly six keys — `{ label, phase, model, effort, agentType, schema }` — and returns the schema-shaped digest or `null` **[measured: `forge-implement.js:2051-2058`]**. The Workflow runtime supplies eight globals total: `agent, parallel, pipeline, phase, log, workflow, args, budget` **[measured: `check-parse.mjs:339`]** — and `workflow` is never used by any forge script. That is the entire host surface to reproduce. In Rust:

```rust
struct StepSpec { label: Label, phase: PhaseId, model: Tier, effort: Option<Effort>,
                  agent: AgentRef, schema: &'static Schema, prompt: String }
async fn run<T: DeserializeOwned>(&self, spec: StepSpec) -> Result<T, StepError>
```

Concurrency needed is trivial **[measured]**: `forge-implement.js` contains exactly **one** `parallel()`, of width 2, and **zero** `pipeline()`; the slice loop is annotated `*** STRICTLY SEQUENTIAL over slices — DO NOT CONVERT TO pipeline()/parallel() ***` because "concurrent agents mutating the same tree corrupt it (the load-bearing safety property of the whole workflow)" (`:1552-1559`). `forge-fix.js` likewise has one `parallel()` of width 2. `pipeline()` appears only in the plan workflows.

---

## 3. Five findings that change the calculus

### 3.1 The cost argument runs the opposite way from intuition — **[documented]**

The standard objection to subprocess-per-agent is "you re-pay the system prompt every time." The docs say the in-session alternative is worse than it looks:

> "A subagent starts its own conversation with its own system prompt and tool set, separate from the parent's. **Its first request doesn't read the parent's cache, because the two prefixes differ**, and it warms a cache of its own across its turns."
> — code.claude.com/docs/en/prompt-caching

So today's 20–50 Task dispatches per run are already 20–50 *distinct, unshared* cache prefixes. Worse, they sit in the wrong TTL bucket: subagents, workflows and forks get **5 minutes, always**, while the main conversation — *including non-interactive `-p` runs* — gets **1 hour on a Claude subscription within plan usage**.

And `-p` runs in the same directory are documented to share:

> "Sessions you run in parallel in the same directory build matching prefixes and read each other's cache. Sequential sessions share the prefix only when the git status snapshot at startup matches."
> — same page

| Line item | Direction under inversion | Why |
| --- | --- | --- |
| System-prompt size per dispatch | **worse** | `-p` gets the full Claude Code system prompt; a Task subagent gets only its own + env details. `--bare` / `--system-prompt` claws most of it back. |
| Tool-definition re-send | **worse by default** | `-p` loads the full tool set + every MCP server's definitions. `--bare` or explicit `--allowed-tools` closes it. |
| CLAUDE.md re-send | **neutral** | Subagents already load the whole hierarchy; both ride after the system prompt, inside the cacheable prefix. |
| **Cache warmth across dispatches** | **much better** | 20–50 cold, unshared, 5-minute prefixes → one shared prefix at up to 1h TTL. |
| **Orchestrator transcript accumulation** | **much better, and it compounds** | Today every dispatch's prompt and returned summary appends to the orchestrator transcript and is re-sent on every later turn, on a monotonically growing prefix, plus the orchestrator's own reasoning tokens. Out-of-process this is a Rust struct at zero token cost. |
| Per-run process overhead | **worse** | Spawn, MCP handshake, settings load per invocation. Mostly latency. |
| **Fan-out cache races** | **worse unless implemented** | Claude Code's workflow runtime already staggers concurrent identical-prefix agents (`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`, default 5000) so only the first pays the write. Fire N at once and you pay N cold writes. Reimplement the stagger. |
| **Git-status churn** | **worse, and specific to us** | The system prompt captures branch and recent commits, and sequential sessions share a prefix "only when the git status snapshot at startup matches." Redkiln commits at every checkpoint. This could bust the shared prefix on *every* dispatch and destroy the main benefit. |

Two levers exist for exactly this workload **[documented]**:

- `--exclude-dynamic-system-prompt-sections` — "Move per-machine sections from the system prompt (working directory, environment info, memory paths, git-repo flag) into the first user message. **Improves prompt-cache reuse across different users and machines running the same task.** … Use with `-p` for scripted, multi-user workloads."
- `--bare` — skips hooks/skills/commands/subagents/plugins/MCP/auto-memory/CLAUDE.md discovery; "the recommended mode for scripted and SDK calls, **and will become the default for `-p` in a future release**."

**The pivotal unknown:** the flag description lists "git-repo flag", not *branch and recent commits*. Whether `--exclude-dynamic-system-prompt-sections` actually moves the git snapshot out of the prefix is **not stated**, and it is the single fact that decides whether inversion is cheaper or more expensive for a commit-per-checkpoint workflow. §8 Experiment 1 settles it.

**No published benchmark of in-session Task dispatch vs `claude -p`-per-agent exists.** I looked. The nearest first-party number is different-product ("agent teams use approximately 7x more tokens than standard sessions when teammates run in plan mode"), and the best third-party work (Systima's "Subagent Tax", 22 Jul 2026) measures subagents-vs-sequential, n=1 per lane, through a caching proxy the authors themselves flag as "exactly the measurement a gateway is most likely to distort."

One more thing worth carrying, from our own tree — `tiers.mjs:52-60` **[measured]**:

> "Output-token price runs 1 : 3 : 5 across haiku : sonnet : opus … Our own run log shows out-token volume running 15,417 to 2,774,995: a 180x spread … So token VOLUME dominates tier by roughly 36x, and prompt-cache hit rate is a third lever this file does not model at all — cache reads bill at ~0.1x input, so a cold-vs-warm prefix swings up to ~12.5x, itself larger than tier. **The honest ordering is volume, then cache, then tier.**"

Inversion acts directly on volume (no transcript accumulation) and on cache (one shared prefix). It is aimed at the two levers that matter, not the one we currently model.

### 3.2 Two-thirds of the workflow layer is compensation, not logic — **[measured]**

Of 689KB across the eight `forge-*.js`, ~35% is template-literal prompt text; the rest is control flow. But a large share of that control flow exists *only* because the Workflow runtime evaluates scripts in a sandbox with "no filesystem or Node module resolution."

**Disappears entirely under inversion:**

| Thing | LOC | Why it existed |
| --- | --- | --- |
| The inlined-`forge-lib` mechanism + `check-sync.mjs` | 394 × 8 + 87 | `new Function`'s scope reaches only `globalThis`; you cannot `import` a shared lib, so the canonical block is inlined byte-for-byte into eight files and a CI checker enforces it |
| `coerceArgs` / `seedError` / `badEntries` | ~90 + 2 guard blocks/workflow | `args` arrives as an object *or a JSON string* depending on launch path; unparsed, "a workflow silently runs its empty-input fallback … yet still reports green" (`forge-lib.js:307-314`) |
| `_agents` / `agentCensus` / `modelMix` / `phaseCensus` / `ph` / `phaseTimings` / `obs` | ~120 | Reconstructs per-run accounting because "`budget.spent()` is ONE cumulative counter … There is no runtime hook that would give per-agent usage" (`forge-lib.js:34-37`) |
| The `#retry${n}` / `tag` / `:${attempt}` label mutations | 5+ sites, 3 docblocks | The host resume journal keys on `(prompt, opts)`, so a re-dispatch cache-hits and "**replays the PRE-fix verdict — passing the gate on code the fix already changed**" (`forge-implement.js:2068-2071`) |
| `scan-returns.mjs`, the ceiling half of `scan-dispatch.mjs`, `MIN_TOPLEVEL_RETURNS`, `MIN_DECLARED_FIELDS`, `minStaticDispatches` | 462 + floors | Static analysers reconstructing at lint time what a type system gives at compile time |
| The `workflow` global | — | Already unused by every forge script |

**The `meta.returns` seam becomes structural.** Today `always` / `sometimes` is enforced by an AST scan with four assertions and anti-vacuity floors (`MIN_DECLARED_FIELDS = 152`). In Rust:

```rust
enum ImplementOutcome { Halted { .. }, Completed { .. } }   // #[serde(flatten)] common: AlwaysFields
```

Assertion (iii) — "every `always` field is on every return" — becomes the type. The rule it enforces is real and was learned the hard way: "exposing only `designReview` here made the consumer read `undefined` on the one path where the design review definitively failed."

**Per-agent cost becomes free.** `forge-lib.js:34-37` concedes today's limit outright. Under inversion each step is its own child emitting `usage{input, output, cache_read, cache_creation}` and `total_cost_usd` in its `result` message. Caveat **[documented]**: `total_cost_usd` is a client-side estimate from a bundled price table, and `usage` *excludes* subagent tokens — use `modelUsage` for tree accounting.

**Ports faithfully — do not "improve" these:** `safeAgent`, `reconcile`, `degraded`/`degradedSummary`, `chunk`. They are the harness-imperfection absorber, and `reconcile` in particular ("replaces a bare `.filter(Boolean)`" so the manifest reports "N of M produced; K degraded" rather than a quietly shorter array) is the single most reusable thing in the repo.

**Shrinks ~10×:** `strayPaths` — 50 lines of Windows drive-letter/relative-path case analysis plus an "accepted residual false negative," because "the workflow script itself cannot shell out … so this checks the `file` fields a writing agent returns." A Rust parent sets the child's `cwd` absolutely and canonicalizes both sides; the ambiguity evaporates.

**Stays prompt, verbatim:** every `agent()` prompt body, `WORKTREE_RULE`, `REDKILN_FRONTMATTER_RULE`, the four agent-card bodies.

### 3.3 Half the command layer is undeclared code — **[measured]**

`commands/implement.md` is 417 lines. The split, line by line:

- **~55% orchestration logic that becomes code.** Worktree find-or-create-or-enter ladder (`:136-141`); preflight checklist a–f (`:166-218`) — `git status --porcelain` classification, branch/toplevel assertions, `entry_baseline` marker gating, `baseRef` capture; ledger read and next-project selection ("first `pending` project in merge order whose dependency projects are all `done`", `:220-226`); **every** `redkiln advance` sequence in step 6; the `degradedSummary` fatal/warn branch; the `record-run`/`record-links` ordering constraint; the ledger update.
- **~30% prose that stays prose.** The terminal-project heuristic (genuine judgment), the presentation instructions for each gate ("Where a capture shows a real defect, say so plainly rather than relaying the score"), the dirty-tree classification narrative.
- **~15% harness plumbing that vanishes.** The `redkiln workflow-root` / `scriptPath` preamble, the `--obs-file` Windows-command-line-length workaround, `EnterWorktree`/`ExitWorktree` semantics, the "launch fresh, NOT `resumeFromRunId`" instructions, and the `Task({subagent_type:"Explore"})` context-budget dance — a Rust parent reads `_plan.md` and `redkiln status --json` directly at zero token cost.

Aggregate across all 2,028 lines: **~50% code, ~33% prose, ~17% vanishes.**

The sharpest example of why this matters is `--stay`, at `implement.md:380-386`:

> "Without it the approving verdict clears the review gate _and_ walks the project straight on into `closeout`/`status: done` … The overshoot is **IRREVERSIBLE**: there is no OPEN review gate left for `--verdict changes-requested` to answer, and `advance` is forward-only, so no CLI verb moves the project back. `closeout.md` then skips it as already terminal and its human gate never fires, while `validate` and `doctor` both report clean."

That is a silent, unrecoverable state corruption guarded today by a paragraph of prose the model must read and obey on every run. In Rust it is one function that always passes the flag.

Same class: `record-links` must run *before* the advance (`:316-320`); the two-call verdict split is forced by the pack, not chosen (`:89-92`); `baseRef` "stability across re-launches" is currently enforced by the model remembering a value across turns (`:213-218`).

### 3.4 Resume already lives in git — keep it there

There are two resume mechanisms and they are routinely conflated.

The **host journal** keys on `(prompt, opts)` and is why every non-idempotent re-dispatch mutates its label. It goes away.

The **actual** resume is git archaeology, and the commands say so explicitly: "Recovery is cheap and **git is the source of truth**: a blocked or partial project is resumed by **re-launching the workflow fresh**. … No `resumeFromRunId`, no hand-edited cache-buster" (`implement.md:76-82`). Two axes, both read from commit trailers by the Preflight agent: `Story: <proj>/<slug>` for committed stories, `Slice-Verdict: <proj>/<slice> <verdict>` for sealed slices. Axis 2 exists because axis 1 is insufficient — "a slice REJECTED at review looks identical, in commit history, to one that passed … **the state that most needs recovery was precisely the state resume could not see**" (`:1566-1569`).

**Two conclusions.** First, git trailers must stay the durable substrate — they survive a deleted journal, a machine change, a `git worktree remove`, and manual surgery. `MULTI-HARNESS.md:229-239` already argues this against Temporal/DBOS: "In a multi-worker fleet, local disk is a lie." Second, **the Preflight agent's git archaeology is the single highest-value thing to port**: it removes an LLM from the resume decision entirely. `MULTI-HARNESS.md:268-270` flags exactly this — "The #1 resume bug class is a non-deterministic gate predicate, not a flaky model."

And RK-B0013 becomes a type error. Today: "one HTTP 529 produced a COMMITTED `changes-requested` seal whose entire surviving finding was 'review failed to produce a digest'. `!review` meant 'the reviewer said no' and 'the reviewer never spoke' indistinguishably, and the second one got written into git as the first" (`forge-implement.js:2173-2181`). In Rust that is `Result<Verdict, TransportError>`.

### 3.5 The engine port has four hard cores, one unbounded risk, and one permanent loss

The engine is *not* dominated by the modules you'd guess. The dangerous surface is ~4,000 LOC concentrated in five files.

| Area | LOC | Difficulty | The specific thing |
| --- | --- | --- | --- |
| `schema/frontmatter.ts` — YAML byte-parity | 59 | **RESEARCH — unbounded** | See below. Gates every write in the product. |
| `store/lock.ts` + `store/ids.ts` + `git/common-dir.ts` | 1,182 | **HARD** | Boot-id tolerance, `EEXIST\|EPERM\|EBUSY` contention, atomic-rename reaping, lock order, fs-before-git resolution. Wrong ⇒ silent id collision across worktrees. Has been wrong three times. |
| `store/write.ts` | 357 | **HARD** | Body read *inside* the lock ("the lock alone never was" the fix), EOL preservation, ms-resolution timestamps (a same-second tie went red 2 of 9 runs), checkbox-blind authorship digest |
| `process/engine.ts` + `gates.ts` | 3,620 | **HARD** | Gates on *both* sides of a stage; `preflightStage` faithfulness; deferred verdict emission; `--dry-run` always exits 0 *including when previewing a refusal* |
| `store/verify.ts` + `verify-cache.ts` + `transition-record.ts` | 3,266 | **HARD** | 5-way `CheckOutcome`; `flaked` must not memoize; memo key is `(cmd, base, tree)` not `(item, run)`; open/close pairing is **set semantics, never a count**, because `merge=union` can legitimately duplicate a line |
| `schema/` null-vs-absent trichotomy | — | **HARD** | zod distinguishes present-but-null / absent / absent-defaulted; serde collapses all three into `Option<T>`. Get `gate_open` wrong and you "walk every currently-paused item straight past its review" |
| `index/` | 3,354 | MECHANICAL — **straight win** | `rusqlite` bundled deletes `createRequire`, the experimental-warning patch, and the vitest resolution workaround |
| `git/` | 1,039 | MECHANICAL — **straight win** | `gix` in-process removes a *measured* 160-second lock hold: "84% of the cost of a lock measured holding for 160 SECONDS against a 5,000 ms acquisition budget" (`ids.ts:196`) |
| `cli/` router + 21 commands | 4,994 | MECHANICAL | clap gives per-subcommand flag projection natively — the projection bug (`doctor --fix-ids --dry-run` parsed, `dryRun` was never forwarded, `fixIds` rewrote ids) becomes unrepresentable |
| `migrate/` | 4,683 | MECHANICAL | Volume, not risk. `health.ts` is one 3,049-line file. |
| `viz/html.ts` + `cli/serve*` + `decision-*` | ~7,800 | **HARD — and Rust makes it worse** | ~1,900 LOC of CSS/JS as string literals, plus `String(announceOffer)` source-interpolation which has no Rust equivalent |
| `hook.ts` + `emit.ts` | 568 | **TRIVIAL — reason for existing evaporates** | The whole gate architecture exists because Node cold-started a 474 KB bundle on every Write/Edit repo-wide |

**The unbounded risk: YAML byte-parity.** `serializeFrontmatter` is `stringifyYaml(data, {indent: 2, lineWidth: 0})` from `eemeli/yaml`, and the codebase argues for it explicitly: "`lineWidth: 0` disables YAML line folding. … A one-word title edit reflows the block and the diff shows four changed lines instead of one … Both matter for a format whose whole premise is that it is read and reviewed in git."

No Rust emitter produces byte-identical output. Divergences you hit on the first item: `null` vs `~` vs empty scalar; `[]` vs empty block sequence; when a title containing `:` gets quoted; quote-style preference; unicode escaping; trailing-newline handling. Each one turns the first `advance` after the port into a whole-frontmatter diff on **every** item it touches, across `.bklg`, `.kb`, and the pinned `.redkiln/processes/*.yaml`.

The ecosystem is worse than it looks **[measured, 2026-08-31]**:

| Crate | Version / last release | Emission control | Verdict |
| --- | --- | --- | --- |
| `serde_yaml` | 0.9.34+deprecated / **2024-03-25**, repo archived same day | none | dead (still 87.7M downloads/90d) |
| **`serde-saphyr`** | 1.2.0 / 2026-08-30 | **richest in Rust** — `indent_step`, `folded_wrap_chars`, `quote_all`, `empty_as_braces`, `comment_position`, … | **RISKY** — one author, 34 releases in 11 months, `#[non_exhaustive]` options |
| `yaml_serde` | 0.10.7 / 2026-08-18, official YAML org | **none** | FINE for greenfield, useless here |
| `serde_yml` | — | — | **TRAP** — RUSTSEC-2025-0068, segfault in the serializer, no patched version |
| `serde_yaml_ng` / `serde_norway` | 2024 | none | stale |

`serde-saphyr` covers maybe 60% of `eemeli/yaml`'s knobs, and does it by making style a property of your *types* rather than a call-time config. **Budget writing the emitter yourself, plus a corpus differential test against all 1,251 markdown files under `.bklg/`.** This is the one item I cannot bound.

**The permanent loss: distribution.** `.gitattributes` states the current property as policy: "`dist/cli.js` and `dist/index.js` are committed on purpose — the repo IS the plugin, `main`'s tip is the plugin channel … That is deliberate and is not changing." `/plugin marketplace add` is a plain git clone; no build, no install, works wherever Node works.

A Rust binary is 15–25 MB stripped **[inferred from crate-stack analysis; measure with `cargo bloat`]** × 5 platforms × every release. Committing them is ~2.4 GB in `.git` after 20 releases, removable only by a history rewrite that invalidates every clone; GitHub warns above 1 GB. Git LFS is not an option — plugin install is a plain clone, and LFS bandwidth is consumed by every CI clone.

The one workable shape **[inferred]**: keep `bin/redkiln.mjs` and `dist/hook.js` as tiny JS shims that `spawnSync` a platform binary. The hook cost story actually survives — today's expensive part is the dynamic `import()` of a 474 KB bundle; replacing it with a spawn of a Rust binary is strictly faster. But you ship binaries somewhere, and "clone and it works on any platform" is gone for good.

---

## 4. Benefits, ranked by confidence

1. **Enforced transitions.** [high confidence] Today `AGENTS.md` *asks* workflows never to call `advance`, and `commands/*.md` says "You — the command — own **every** `redkiln advance`." That is prose. Under inversion the agent is never offered the option. The `--stay` irreversibility trap and the `record-links`-before-`advance` ordering stop being things a model must remember.
2. **Deleted machinery.** [measured] ~1,000+ LOC of static analysers and their anti-vacuity floors, the forge-lib inlining mechanism, the invocation-contract guards, the label-mutation workarounds — all gone, replaced by types.
3. **Per-agent cost attribution.** [documented] A limitation the code currently apologises for becomes a field.
4. **Hard budget stops and deterministic retry/timeout/cancel.** [high] `BudgetTracker`-style short-circuiting before dispatch; process-group kill on cancel.
5. **Prompt corpus separates from engine.** [high] The ~35% of workflow bytes that are prompt text become versionable, digest-pinnable, repo-overridable data — the BMAD customization gap and the shipmates digest idea landing as a side effect.
6. **Deterministic resume.** [high] Git archaeology moves from an LLM to `git log --grep` + trailer parse.
7. **Headless/CI operation.** [high] An initiative can advance without a human in a terminal.
8. **Tier policy as typestate.** [measured] `Dispatch<Haiku>` has no `effort` field, so the "haiku rejects the parameter" branch becomes unrepresentable; `scan-dispatch.mjs` deleted.
9. **Genuine multi-harness.** [medium] Owning the loop is the precondition; a harness trait is then real work but tractable.
10. **Engine-side wins** (Bet 2 only): `rusqlite` deletes the experimental-API risk; `gix` deletes a measured 160-second lock hold; clap makes the flag-projection bug unrepresentable; the hook's entire raison d'être evaporates.

---

## 5. Risks, ranked

| # | Risk | Severity | Mitigation | Falsifiable test |
| --- | --- | --- | --- | --- |
| R1 | **Context fidelity.** `agent()` today dispatches into a live session; headless every spawn is cold and ambient context becomes explicit payload. Prompts were written against the warm case. | **Critical** | Pass `--agents` JSON explicitly; carry the git-archaeology results as structured input rather than hoping the agent re-derives them | **Exp. 2** |
| R2 | **`effort` has no `--agents` equivalent.** [documented gap] `tiers.mjs:78-84`: an absent effort is inherit-from-session, "so before this column existed every redkiln step ran at whatever depth the invoking session happened to sit at: the cost and quality profile of a run was a property of the CALLER." | **High** | Carry effort out-of-band per invocation; verify `--effort` on the parent applies. Otherwise the exact defect the effort column closed returns. | **Exp. 2b** |
| R3 | **YAML byte-parity** (Bet 2 only) | **High, unbounded** | Write the emitter; differential-test against all 1,251 `.bklg` files; accept a one-time normalising commit if parity proves impossible | **Exp. 3** |
| R4 | **Git-status churn busts the shared prefix.** Redkiln commits at every checkpoint; sequential `-p` runs share a prefix "only when the git status snapshot at startup matches." | **High** | `--exclude-dynamic-system-prompt-sections` — but the docs list "git-repo flag", not branch/commits | **Exp. 1** |
| R5 | **Distribution property lost permanently** (Bet 2 only) | **High** | JS shim + platform binaries via npm optional-deps or cargo-dist; never commit binaries | — (settled, not testable) |
| R6 | **`--bare` becomes the `-p` default.** [documented] It skips agent/command/plugin/MCP/CLAUDE.md discovery. A design relying on `.claude/agents/` discovery in `-p` is building on a surface scheduled to change. | Medium | Pass everything explicitly from day one: `--agents`, `--allowed-tools`, `--system-prompt`. Never rely on discovery. | — |
| R7 | **Fan-out cache races.** The Workflow runtime staggers identical-prefix agents; we'd have to. | Medium | Reimplement the stagger. Cheap — our max fan-out is 2. | — |
| R8 | **CLI surface coupling.** No documented stability policy for headless flags or the `stream-json` schema; breaking changes appear in the changelog post-hoc. | Medium | Adopt `claude-codes`' discipline: pin a `TESTED_CLI_VERSION`, check `claude --version` at startup, warn loudly on divergence | — |
| R9 | **Dependency on 0.x single-author crates.** Every Rust Claude-driving crate is 0.x, single-author, <25 stars; the two best are 1–2 months old; the two feature-richest are 6.5 months stale. | Medium | **Do not depend on one.** Take `claude-codes`' types/protocol layer (Apache-2.0) and own spawn/supervise/cancel yourself. ~500 LOC. | — |
| R10 | **Test rewrite.** [measured] 101 CLI-behaviour files (~60,700 LOC) port as black-box specs; 61 internal unit files (~46,600 LOC) get rewritten; 80 meta files (~30,600 LOC) are untouched or discarded. Plus every help/error string re-pinned. | Medium | `trycmd`/`assert_cmd` + `insta`; the zero-mock injection convention already gives you the seams | — |
| R11 | **The board gets worse.** ~7,800 LOC where Rust is a net negative, including a source-interpolation trick with no Rust equivalent. | Medium | Leave `viz/html.ts` + `serve` in TS behind the shim, or accept an `include_str!`'d JS asset tested under a JS engine | — |
| R12 | **Human gates change shape.** Every vendor's answer to mid-run approval is "make each stage its own invocation." | Medium | The `gate_open` / `transition_open`/`close` model already supports suspend-and-resume; the UX is new, the state model is not | — |
| R13 | **The residual honesty gap survives unimproved.** `relayViolations` — "a green tier suite means the routing is CONSISTENT with the labels, never that the labels are honest … That is not hypothetical, and it has now happened twice." A type system pins `Dispatch<Haiku>` to the gate role; it cannot tell you the *prompt* asks a judgment question. | Low but permanent | Keep `check-prompts.mjs`-class lints over the prompt corpus. They do not port; they persist. | — |
| R14 | **Compile-time / iteration speed.** vitest ~1s → Rust ~15–60s; 101 integration test files means 101 binaries linking the whole stack. `std::env::set_var` is `unsafe` in edition 2024 and racy under the parallel harness — Node tests mutate `process.env` freely. | Low | lld/mold, `debug=0`, consolidate test binaries | — |

---

## 6. Effort

Rough, and deliberately not precise **[inferred from measured LOC]**:

| Workstream | Scope | Note |
| --- | --- | --- |
| Step runner + prompt corpus extraction | ~500 LOC new + move ~244KB of prompt text to files | The genuinely new code is small |
| Orchestrator port (workflows + command logic) | ~9k LOC of logic → Rust; ~1k lines of command prose stays | Two-thirds of the 13,439 LOC is compensation that deletes |
| Run record + deterministic resume | ~800 LOC | Replaces an LLM in the resume decision |
| Engine port (Bet 2) | 40k LOC, of which ~4k is the dangerous core | Plus the YAML spike |
| Test rewrite | ~47k LOC rewritten, ~60k re-pinned | 30k discarded |
| Distribution rework | shim + CI cross-compile matrix | Permanent complexity increase |

Bet 1 alone is a substantial but bounded project. Bet 2 roughly triples it and adds the one item I cannot bound.

---

## 7. Positioning — the evidence says do not abandon the plugin

**[measured, 2026-08-31]** The spec-driven / backlog-as-code space is converging on harness-attached installers that write skills into many harnesses, not on standalone binaries:

| Project | Stars | Owns control flow? |
| --- | --- | --- |
| github/spec-kit | 131,282 | No — prompts only |
| Fission-AI/OpenSpec | 66,255 | No — and markets *"no rigid phase gates"* as its differentiator |
| BMAD-METHOD | ~52,500 | Nearly — but transitions are enforced by the prompt the harness runs |
| claude-task-master | 28,036 | No — and stale since 2026-04-28, commercially absorbed |

~278k combined stars, **none owns control flow**. Meanwhile:

- **Rust + standalone binary + real enforced human gates = exactly one project, `dex`, at 21 stars.** The most rigorous Rust state machine found (`NEEDLE`) has no human gate at all and 18 stars.
- **The middle layer — UI over worktrees, no gates — is commercially dead at any star count.** Vibe Kanban shut down at 27.1k stars ("the vast majority are free users and we couldn't find a business model"); Roo Code archived at 24.3k; Terragon and Crystal both shut down in early 2026; HumanLayer's OSS was deprecated in favour of a $100/user/mo closed product.
- **Gates are commercially validated but only closed-source.** HumanLayer sells a six-phase gated workflow at the highest price point found; Conductor sells cloud sandbox + multiplayer at $50–60/user/mo.
- **The conversion ratio inverts the star story.** Spec Kit converts at 1,177 stars per monthly download; Backlog.md — a compiled binary — at 120:1, ten-fold better. The mega-star repos are bookmarked, not installed.

**Two honest counter-signals against the whole thesis, worth stating plainly.** Agent OS v3.0.0's changelog says: *"Implementation/orchestration phases retired — frontier models handle this well on their own now."* And OpenSpec markets the absence of phase gates as a feature. Two independently well-adopted projects treat enforced phases as a liability. That is evidence, not noise.

**The shape that works** is neither pole: standalone binaries get adoption only when they own a durable artifact the harness cannot hold — and both examples that do (Backlog.md's task ledger, Archon's workflow-runs DB) *also* ship a skill/MCP layer installed into every harness. Redkiln already owns the durable artifact. The conclusion is to add the orchestrator, **not** to trade away the plugin surface for it.

Also worth knowing: the gap a standalone orchestrator fills is real and currently unoccupied. Across every harness's headless mode, **nobody ships a mid-run human approval gate** — Claude Code's workflow docs say "No mid-run user input… For sign-off between stages, run each stage as its own workflow"; Cursor's SDK docs say "There's no human-in-the-loop approval flow in headless SDK runs." Something outside must own the stage boundary and resume. That is precisely what Redkiln is.

And `oneharness` (the only real multi-harness Rust abstraction, 8 harnesses, ~2 months old, 1,338 downloads, 0 stars, bus-factor 1) **stops where Redkiln would start**: no workflow engine, no gates, no retry beyond schema re-prompt.

---

## 8. The experiments that would decide it

In order. Each is days, not weeks, and each can kill the project cheaply.

**Experiment 1 — Cache economics. (½ day)**
Run the same trivial prompt 10× via `claude -p --output-format json` in one directory: (a) back to back; (b) with a `git commit` between each; (c) with a commit between each *plus* `--exclude-dynamic-system-prompt-sections`. Read `usage.cache_read_input_tokens` vs `cache_creation_input_tokens` and the `ephemeral_1h` / `ephemeral_5m` split.
**Kill criterion:** if (b) shows a cold write every time and (c) does not fix it, the shared-prefix benefit is unavailable to a commit-per-checkpoint workflow and the cost case for inversion collapses to "avoids transcript accumulation" alone.

**Experiment 2 — Context fidelity. (1–2 days) — the load-bearing one.**
Take `redkiln-review`, the narrowest agent (one rubric, bounded verdict, `tools: [Read, Bash]`). Run it over 10 already-reviewed stories, both ways: in-session `Task({subagent_type: "redkiln:redkiln-review"})` vs `claude -p --agents '<card as JSON>' --model X --json-schema REVIEW_DIGEST`. Compare verdicts, blocker lists, rubric scores, and cost.
**Kill criterion:** if headless verdicts diverge materially — different blockers, softer scoring, missed AC coverage — no amount of Rust fixes it, and the whole architecture is worth less.
**2b:** in the same run, confirm whether reasoning effort can be pinned per dispatch headlessly. If not, R2 is live and the effort column reverts to being a property of the caller.

**Experiment 3 — YAML parity. (1–2 days, Bet 2 gate.)**
Point `serde-saphyr` (pinned `=1.2.0`) at all 1,251 markdown files under `.bklg/` plus `.kb/` and `.redkiln/processes/`. Round-trip each and diff the bytes.
**Kill criterion:** if the divergence isn't closable with `SerializerOptions` plus a bounded post-pass, the engine port needs a hand-written emitter — reprice Bet 2 accordingly, or don't take it.

**Experiment 4 — The `fix` lane, end to end. (1–2 weeks.)**
Smallest lane: one story, existing `spec.md`, 7 halt paths, one `parallel()` of width 2. Build the step runner, port `forge-fix.js`'s logic and `fix.md`'s transitions, shell out to the existing TS CLI. Run it against real bugs alongside `/redkiln:fix`.
**Success criterion:** same verdicts, same commits, same telemetry, at comparable or lower cost — plus the enforcement wins actually showing up (no `--stay` class of error possible, deterministic resume, per-agent cost recorded).

Only after 4 is green does the engine port become a live question.

---

## 9. What would make me say no

- Experiment 2 shows material verdict divergence. **This alone kills it.**
- Experiment 1 shows the git-churn prefix bust is unavoidable *and* Experiment 4 shows cost materially up.
- Effort cannot be pinned per dispatch (R2) and the quality difference is visible.
- The decision requires giving up `/plugin marketplace add`. §7 says that trade is backwards.

## 10. What I could not establish

- **No published benchmark** of in-session Task dispatch vs `claude -p`-per-agent exists. Anywhere.
- Whether `--exclude-dynamic-system-prompt-sections` removes the git **branch/commit** snapshot from the prefix, or only the working directory. Pivotal; undocumented.
- Whether `--fork-session` (the CLI flag) reuses the parent's prompt cache. The widely-repeated "fork reuses the cache" is documented for the *in-session* `subagent_type: "fork"` agent, not the flag.
- The exact `stream-json` event schema; docs say "each line is a JSON object" and "the last line includes cost" without specifying the shape.
- Whether settings-based hooks fire in `-p`. Documented for the Agent SDK; not stated for CLI print mode. Matters for the `PreToolUse` deny.
- Real binary size for this crate stack. 15 minutes of `cargo build --release && strip` settles it.
- Whether `redkiln-design-review` retains fidelity headlessly — it needs `Bash` + capture + **vision on the resulting PNGs**.

## 11. Sources

**First-party docs:** code.claude.com/docs/en/{prompt-caching, sub-agents, sessions, headless, cli-reference, permission-modes, worktrees, skills, plugins-reference, changelog, costs, model-config}; code.claude.com/docs/en/agent-sdk/{overview, cost-tracking, hooks, permissions, structured-outputs}; platform.claude.com/docs/en/build-with-claude/prompt-caching; docs.claude.com/en/docs/claude-code/sub-agents.

**Redkiln tree @ 0.20.0:** `src/**`, `workflows/**` (esp. `_lib/forge-lib.js`, `_lib/tiers.mjs`, `_lib/check-parse.mjs`, `forge-implement.js`, `forge-fix.js`), `commands/**`, `agents/**`, `hooks/hooks.json`, `.gitattributes`, `references/MULTI-HARNESS.md`, `references/audit/findings/09-workflow-orchestration.md`.

**Ecosystem:** crates.io API (2026-08-31) for serde_yaml / serde-saphyr / yaml_serde / serde_yml / rusqlite / gix / clap / claude-wrapper / claude-codes / oneharness / cargo-dist and ~30 others; RUSTSEC-2025-0068; github.com/dtolnay/serde-yaml (archived 2024-03-25).

**Third-party, with caveats:** systima.ai/blog/subagent-tax (n=1 per lane, measured through a caching proxy the authors flag); anthropic.com/engineering/multi-agent-research-system (different product); github.com/andyrewlee/awesome-agent-orchestrators (~180 projects surveyed).
