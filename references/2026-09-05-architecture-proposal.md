# Redkiln-RS — architecture proposal

**Date:** 2026-09-05 · **Revision:** 4, 2026-09-07 (rev. 2 after Ryan's review notes; rev. 3 recorded D1–D14; **rev. 4 applies the S-series results** — see `2026-09-07-design-recommendation.md` for the data, the prior art and the reasoning behind every change marked *rev. 4*)
**Status:** Proposal with decisions D1–D19. §8's Phase-1-gating spikes are complete except S-BIN; S-H1c is registered and unrun.
**Inputs:** `spikes/FINDINGS.md` (F1–F28; there is no F9), the RFC (08-31) and its amendment (09-02/04), the spike charter, the brainstorm (09-03) and its six research briefs, the Run 3 conformance scorecard, the shipmates lessons, Claude Code / Codex / Linear / Unreal platform docs as of today.
**Evidence discipline:** every load-bearing claim carries a finding number or a source. Where a claim rests on one draw, one machine or one operator, it says so.

---

## 0. The decision in one paragraph

Build Redkiln-RS as a **clean-break, single Rust binary that owns the loop**: a deterministic workflow kernel (stage selection, gates, budgets, resume) that dispatches one harness process per activity, keeps its durable state in **git trailers plus an append-only event log**, exposes **human gates as durable marks rather than waiting processes**, and treats **every agent output as a typed value** of which markdown is a rendering, never a source. The kernel is hexagonal along five named variation axes — event store, harness, tracker, docs site, toolchain — each with one real adapter and one fake. The old redkiln is neither ported nor phased out; a separate migration tool converts its `.bklg`/`.kb`, and its tests and recorded bugs become specifications. The spike programme answered the kill questions: headless review is not worse (F23/F26), effort and model pin per dispatch (F7/F15), the cache survives commits under `--system-prompt` (F16), suspension is lossless (F19), and a gate costs at most one prefix rewrite to resume — 18.6k under the default prompt, ~30k under a card prompt — regardless of how long it lasts (F28 as corrected, S-H1b). What it did not answer — and what the architecture must absorb — is that the **reviewer's verdict is unstable at the threshold** (F27, S-VAR) while its findings are not (S-TYPED), so the blocking decision moves out of the model into code that re-executes the reviewer's own evidence (*rev. 4*, §4.6).

---

## 1. What the spikes settled

| # | Question the RFC or charter posed | Answer | Evidence |
| --- | --- | --- | --- |
| 1 | Does headless dispatch degrade review quality? (kill criterion) | **No.** 23/23 vs 23/23 detection; 20/20 across two harder tiers; arms indistinguishable (McNemar p=0.625) | F23, F26 |
| 2 | Can effort/model be pinned per dispatch? | **Yes.** `--agents` carries `model`+`effort`; `--agent` adopts; high vs xhigh separate with no overlap | F7, F15 |
| 3 | Does commit-per-checkpoint bust the cache? | **31% unmitigated, 0% under `--system-prompt`.** Prefix is two-segment: ~17.4k shared, ~7.8k per-directory | F13, F16 |
| 4 | What does a worktree cost in cache? | **31%**, same as a commit; not the 100% the docs imply | F16 arm d |
| 5 | Is the `-p` cache TTL 1h? | **~1h** (boundary 45–62 min), date-stamped platform behaviour | F10 |
| 6 | Do in-session subagents get 5m TTL? | **No**, at least inside a headless parent: 0 5m-writes across 56 dispatches. The RFC's asymmetry argument does not hold | amendment §5, findings-for-product A5 |
| 7 | What does headless cost vs in-session for identical detection? | **In-session 1.72×** ($97.39 vs $56.47), same card/model/effort/fixtures | F23 |
| 8 | Is resume across a process boundary deterministic? | **174/174 lossless** | F19 (H2) |
| 9 | Does the world move during a human gate? | **7% overall; 11% for gates >1h vs 3% under** (3.6×) | F19 (H3) |
| 10 | How long are human gates? | **Median 10.5 min (gate→verdict anchor); ~half exceed 1h on both anchors; p90 ~9h and max 9.4 days on the stage→verdict anchor** | F18 |
| 11 | What does resuming a long gate cost? | **At most one prefix rewrite, independent of gate length**: 18,603 under the default prompt at both 65 min and 49.7 h (the shared head survived on an account-wide subsidy, not on its own — F28 correction); **29,619–29,680 (the whole prefix) under a static `--system-prompt`** at 66 min (S-H1b). 0 below the TTL | F28 (H1, corrected 2026-09-05), S-H1b |
| 12 | Is the step digest sufficient context for the next step? | **Yes**: FULL (digest + 10 kB of reviewer prose) and CONTROL both score Jaccard 1.000 vs DIGEST on cited file sets, 5/5 items each; effect +0.000. Caveat: the metric has no variance, so only a large effect was detectable | F28 (H4) |
| 13 | Does the affected-area gate catch regressions? | **9/10 test-reachable mutants killed**; the survivor was caught by the reviewer 4/4 | F11, F26 |
| 14 | Is `Story:`-trailer resume sufficient? | **No — 20.3% of slices passed through a rejected state**; axis 2 (`Slice-Verdict:`) is required | F12 |
| 15 | Is the reviewer reliable? | **Detection stable (0/7 flaky, 21/21 draws). Verdict unstable (2/4 items flip).** | F27 |
| 16 | Does `-p` inherit the operator's environment? | **Yes**: 12 MCP servers, ~280 tools, 56% of cache write. `--strict-mcp-config` removes it | F8 |
| 17 | Does a tool-restricted card + `--json-schema` work? | **Silently no** unless `StructuredOutput` is in the card's tools; exit 0 either way | F20 |
| 18 | Do platform errors surface? | **529/500 return exit 0 / `subtype: success`**; only `is_error`/`api_error_status` reveal it | F21 |
| 19 | Can a reviewer with `Bash` defeat a fixture? | **Yes** — it restored the fix from the object store and reviewed the real code | F17 |
| 20 | Do hooks fire in `-p`? | **Yes**, including user-level ones | F2 |

Three findings from adjudication reshape the product rather than the port: **5 of 23 shipped fix-lane items (22%) carry an AC no test pins** — *rev. 4:* **≥12 of 23 (52%)** after S-TYPED's mechanical adjudication confirmed 14/14 newly raised findings (F24/F25, root cause: Red→Green evidenced per test, an AC is a claim about one assertion); the review consumer in `forge-fix.js` **fails open** on an absent verdict (scorecard GATE-01); and `gate_result.basis` silently changed meaning (GATE-05). A fourth, about method: **five metrics saturated; every discriminating result came from cost, variance or mechanism, never from success rates** (F28).

## 2. What is still open, and how the design hedges it

| Open item | Why it matters | Hedge in this design |
| --- | --- | --- |
| Verdict instability (F27) — n=2–3, arm (b) only | A gate that blocks on different days is a different product | §4.6: verdict computed by policy from typed findings, not returned by the model |
| Interactive-session cache TTL for subagents | Only affects the *size* of the cost win, not its sign | Cost case rests on measured 1.72× and transcript non-accumulation, not on TTL asymmetry |
| H4's metric saturated (CONTROL = 1.000); `plan` prose varied up to 5k chars between arms; `--system-prompt` across *time* unmeasured | Digest sufficiency beyond file selection; whether the 18.6k can go to 0 | Typed digest carries `files[]` + `evidence`; full prose retained in the event log, re-injectable; H1 secondary in §8 |
| Opus family sub-limit invisible (F1) | The binding limit cannot be throttled against | Throttle on `five_hour` with margins ≥2 ticks; per-item token appetite as the real ceiling |
| Whether cache reads draw down the window (F14) | Changes the value of warmth | Unanswerable at instrument resolution; cache treated as a latency/cost lever, not headroom |
| Platform surface unstable (R6: `--bare` will become the `-p` default; R8: no stability policy, undocumented events) | Any flag or event we rely on can move | Harness adapter pins `TESTED_CLI_VERSION`, asserts from `system/init`, retains unknown events, never depends on discovery |
| Quality on a second harness | E3 is Claude-only | Harness port + a Codex/Pi run of the E3 rig before any non-Claude adapter is allowed in a gate (§8) |
| Binary-artifact ecosystems (UE) | No review evidence exists for items whose change is not a text diff | Toolchain port renders a typed change summary; spike in §8 |
| Typed-artifact file layout and human edits | Drives the store format and the whole "no parsing" claim | Decided (D5/D6): prose free, structure gated; layout C with sidecars for machine-only artifacts |

## 3. Principles the data imposes

1. **Rust holds the loop; the model holds no control flow.** Every `advance`, gate, retry and budget is code. The `--stay` class of irreversible corruption becomes unrepresentable (RFC §3.3). Selection is deterministic; the agent *classifies* and may *request* a typed deviation (brainstorm correction 1).
2. **Git is truth; the record is a cache.** `reconcile(recorded, world) -> (state, drift)` runs at the top of every entry (shipmates L2.2/L2.4). H3 says drift is rare (7%) and cheap to catch; there is no case for skipping it.
3. **Nothing waits.** Human gates, long gaps, rate-limit resets and fan-out joins are durable marks plus a later read, never a live process (F22; the mark/read split is recorded as having paid for itself four times in the "Operational" entry, and again for H1's 65-min and 49.7-h reads).
4. **Absence of evidence is a block.** `Result<Findings, StepError>`; an absent digest is `Err`, never `pass` (GATE-01, RK-B0013).
5. **Explicit over discovered.** Every dispatch passes card, system prompt, tools, schema, MCP config, permission mode. Adoption and isolation are verified from the child's own report, not assumed (F7, F8, F17 defect 2).
6. **Typed in, typed out.** Prompts are compiled from typed artifacts; outputs are typed values validated at the boundary; markdown is rendered from types and never parsed back into control flow (scorecard SHAPE-01; the old repo's markdown-shape validators are the cautionary tale).
7. **One dispatch, one schema, per activity.** Sequential work gets a single agent; orchestration is reserved for the parallel project DAG and review panels (Google Research scaling study: sequential −39…−70%, parallel +80.9%).
8. **Gates are weighted, proportionate, placed before expensive phases.** Skipping is `0`, explicit and logged; gate cost is flat and small (F28), so placement is a value decision.
9. **Ports at named variation axes only.** A port exists where a substitute has been named (Codex, Linear, Astro, .NET/UE) or where the fake alone pays for it (EventStore); nothing else is abstracted. Git is not a port.
10. **The kernel knows no language and no tracker.** Ecosystem behaviour lives in technology packs behind the toolchain port; collaboration surfaces are projections behind the tracker port.
11. **Measure cost, stability and mechanism — not success rates** (F28). The product's own telemetry is built on that basis.
12. **Lessons, not code, from the old redkiln.** Its tests port as black-box specifications and its recorded bugs as test cases; its logic does not port. One exception: `reconcile.rs`, rewritten in the spike and validated on the corpus (F12).
13. **The plugin surface stays** and is generated from one source. The two standalone binaries that got adopted (Backlog.md, Archon) own a durable artifact *and* ship a skill/MCP layer (RFC §7; brainstorm §0).

## 4. Target architecture

```
                  ┌──────────────────────────── redkiln (Rust, one binary) ────────────────────────────┐
                  │  kernel: packs → stages → activities · gates · budgets · memo · resume · policy     │
                  │  store : .bklg / .kb typed artifacts, one write path, canonical serialiser          │
                  │  git   : trailers · worktrees · reconcile() · merge queue            (not a port)   │
                  └───┬──────────────┬──────────────┬──────────────┬──────────────┬────────────────────┘
        ports ───►  EventStore     Harness        Tracker        DocsSite       Toolchain
        adapters    sqlite         claude-code    github         mdbook         node · dotnet · python · unreal
                    fake           codex*         linear*        astro* · rustdoc*
                                   fake           fake           fake           fake
                                                  (* = named, not built in Phase 1; no Postgres adapter planned)
```

Authority, most to least trusted, and who writes each:

| Layer | Owner | Writes |
| --- | --- | --- |
| Git (commits, trailers `Story:` / `Slice-Verdict:` / `Gate:`) | kernel | checkpoint commits, gate marks |
| Item store (`.bklg`, `.kb`) | kernel via store | single write path; agents return values, the store renders files |
| Event log (EventStore port) | kernel | every stage/gate/dispatch/cost/drift event; telemetry tier is disposable |
| Projections (Tracker, DocsSite, web) | kernel | derived; inbound events are typed inputs, never authoritative |

### 4.1 The five ports

| Port | Responsibility | Real adapter (Phase 1) | Named alternatives | Fake for tests |
| --- | --- | --- | --- | --- |
| `EventStore` | append event; query by item/SHA/run; snapshot; rebuild | SQLite (WAL, `busy_timeout`, at `<git-common-dir>/redkiln/`) | none planned — the port exists for the fake and for future safety, not for a Postgres adapter (D3) | in-memory Vec |
| `Harness` | `dispatch(ActivitySpec) -> Result<Typed, StepError>`; capability registry with dated probes; cost/usage; cancel | Claude Code (`claude -p`) | Codex (`codex exec --json --output-schema`), Pi | scripted responder |
| `Tracker` | `project(item) -> ExternalRef`; `inbox() -> Vec<TypedInput>`; idempotent writes | GitHub (labels, sub-issues, `blocked_by`, comments) | Linear (GraphQL, signed webhooks), Jira | in-memory board |
| `DocsSite` | `build(DocSet) -> Result`, `check_links`, `drift(changed_sources)` | mdBook (+ `mdbook test`) | Astro/Starlight, rustdoc, DocFX | no-op recorder |
| `Toolchain` | detect; `build`; `test(selection)`; `lint`; `classify(path) -> Text/Binary/Generated`; `change_summary(range) -> ChangeSummary` | Node (vitest/pnpm) | .NET, Python, Unreal, Rust | fixture-driven |

Port rules that keep this from going exponential: one trait per port, no generic parameters leaking into the kernel; adapters selected once at startup from `.redkiln/config.toml`; a fake per port is a hard requirement of adding the port; capability differences are data in the adapter's registry (`{capability, verified_on, probe}`; shipmates L4.3/L5.7), never `if adapter == …` in the kernel. Where an adapter lacks a capability the kernel needs, the activity fails with `StepError::Unsupported(capability)` and the pack can route around it.

**Git is deliberately not a port.** Trailers, worktrees, shallow clones for review, and `reconcile()` are load-bearing and have measured behaviour (F12, F19). No alternative VCS is named.

### 4.2 Kernel — workflow/activity split, two pack axes

Transplant Temporal's split literally: the **workflow** (stage selection, gate evaluation, retry policy, DAG order) is deterministic Rust replayable from the event log; **activities** hold all non-determinism (harness dispatch, toolchain runs, tracker calls) and carry a retry policy, a timeout with a number, and an idempotency key `(run_id, activity_id, attempt)` (shipmates L5.4/L5.6). Content-addressed memoisation keyed on `hash(activity id, resolved inputs, card digest, injected KB atoms, toolchain version)` makes re-entry cheap and makes a stale `.kb` atom invalidate stale work.

Packs are TOML, shallow, validated in Rust with spans, on **two axes**:

- **Process packs** — what runs: stages, activities, gates, weights per classification. Ported as *content* from `.redkiln/processes/*.yaml`, not as code.
- **Technology packs** — how an activity is realised in an ecosystem: verify commands, affected-test selection, build, artifact classification, the shape of the change summary a reviewer receives, timeouts. One per toolchain adapter; a repo may compose several (a Node front-end and a .NET API).

```toml
# .redkiln/config.toml (consumer repo)
[adapters]
event_store = "sqlite"
harness     = "claude-code"
tracker     = "github"
docs        = "mdbook"
toolchains  = ["node", "dotnet"]

# process pack (excerpt)
[stage.review]
goal = "An independent reader confirms every AC is pinned by an assertion that discriminates"
[[stage.review.activity]]
id = "adversarial-review"
run = "agent:redkiln-review"
inputs  = { spec = "artifact:Spec", report = "artifact:ImplReport", change = "artifact:ChangeSummary" }
outputs = { findings = "value:Findings" }          # typed; verdict is NOT an output
timeout_secs = "${toolchain.review_timeout}"       # F21: CLEAN reviews reached 2055s on Node
cache_key = ["spec", "report", "change", "card.digest", "kb.injected"]

[policy.bug-standard]
class_of_service = "standard"
stages = { triage = 0.2, investigation = 0.5, implement = 1.0, verify = 1.0, review = 1.0, closeout = 0.5 }

# technology pack: unreal (excerpt)
[toolchain.unreal]
verify = 'UnrealEditor-Cmd "${project}" -ExecCmds="Automation RunTests ${selection};Quit" -unattended -NullRHI -ReportExportPath=${report_dir}'
binary_globs = ["**/*.uasset", "**/*.umap"]
change_summary = ["text-diff", "asset-manifest", "build-log", "automation-report:json", "captures"]
review_timeout = 7200
```

Weights select from an ordered option list per stage (Disciplined Agile "choose your WoW"); `0` is an explicit, logged skip. Triage is the one always-run stage; Closeout is `finally`. **Deviation is typed**: an activity may return `Blocked { reason, question }` or `RequestDeviation { stage, weight, why }`; both open a gate; neither changes selection without a human.

#### 4.2.1 Roles, cards, skills and slots — how the two pack axes compose

The two axes both want to own "the thing that runs". The resolution, borrowed from every build system that hit the same tension (Nx targets vs executors, Tekton Pipelines vs Tasks, Cloud Native Buildpacks' ordered detect-and-contribute, GitHub reusable workflows vs composite actions), is that **one axis owns structure and the other owns bindings and contributions, through named extension points**.

| Layer | Owns | Never owns |
| --- | --- | --- |
| **Process pack** | stages, activities, gates, weights; **roles** (archetypes: `implementer`, `reviewer`, `investigator`, `spec-author`); tier policy per role; **slots** (named extension points in a stage) | any agent card; any toolchain command |
| **Agent card** | identity, heuristics, return format, capabilities (shipmates L4.5); which role(s) it *fulfils*; an optional *selector* | orchestration; model choice (L4.2, L4.6) |
| **Technology pack** | toolchain adapter config; **skills** (procedural knowledge attached by rule); **specialised cards** fulfilling a role under a selector; **slot fillers** (activities inserted at declared slots) | new stages; edits to process activities |

An activity nominates a **role and a typed contract**, never a card:

```toml
# process pack
[role.implementer]
tier = { model = "sonnet", effort = "high" }        # tier lives on the role; a card cannot downgrade it
[role.reviewer]
tier = { model = "opus", effort = "xhigh" }

[[stage.implement.activity]]
id = "implement-slice"
role = "implementer"
inputs  = { spec = "artifact:Spec", change = "artifact:ChangeSummary" }
outputs = { report = "artifact:ImplReport" }

[stage.verify]
slots = ["pre-test", "post-test"]                   # the only places a technology pack may add work

# technology pack: unreal
[[card]]
name = "ue-implementer"
fulfils = ["implementer"]
selector = { toolchain = "unreal" }                 # beats the generic card when the item's toolchain is unreal

[[skill]]
name = "ue-automation-testing"
attach = { toolchain = "unreal", roles = ["implementer", "reviewer"] }

[[slot_filler]]
slot = "verify.pre-test"
activity = { id = "cook-assets", run = "toolchain:unreal.cook", timeout_secs = 1800 }
```

**The binder** is deterministic Rust, run at plan time and recorded in the event log:

1. For each activity: `role → candidate cards → most specific selector wins` (toolchain match beats generic). Two candidates at equal specificity are a **configuration error**, never a guess.
2. Skills attach by rule — toolchain × role × stage × file-glob (Kiro's conditional-inclusion pattern) — and are compiled into the card's **stable prefix**, so they are in the card digest and therefore in the activity cache key. Each distinct card is one one-time cold cache write (F16).
3. Slot fillers are inserted in declaration order at the named slot; a filler naming an undeclared slot is a configuration error.
4. The resolution — card, skills, fillers, tier — is written to the run record, so "why did this run with that card" is answerable without reading TOML.

**The hard line.** A technology pack may *bind* (cards, skills) and *contribute at slots*; it may never *restructure*. When an ecosystem needs a different shape — a cook/bake stage that is not a pre-test slot — that is a **process-pack variant** (`story-ue`) selected at triage or by config: still content, axes still clean, and the choice visible rather than the product of two packs silently merging.

**`redkiln doctor` rules for packs** (all configuration-time, zero model cost):

| Rule | Failure |
| --- | --- |
| Every role in every enabled process pack resolves to exactly one card per configured toolchain | unresolved or ambiguous role |
| Every card's `fulfils` names a declared role; every skill's `attach.roles` likewise | dangling reference |
| Every slot filler names a declared slot; every declared slot is either filled or documented as optional | undeclared slot / dead slot |
| No role, skill or slot is unused by any enabled pack (L5.11 — no dead schema) | dead definition |
| A card's declared tools include `StructuredOutput` when its role's activities demand a schema (F20) | silent schema void |
| Tier is set on every role; no card or filler sets a model (L4.2) | tier leakage |
| Every filler activity and every card declares a timeout with a number (L5.6) | unbounded activity |

**Ecosystem fit, by example.** Node: vitest `--changed`/affected selection; .NET: `dotnet test --filter` with `.trx` parsed into the typed test report; Python: `pytest -k`/testmon; Unreal: `UnrealEditor-Cmd -ExecCmds="Automation RunTests …" -NullRHI -ReportExportPath` (JSON), Gauntlet for packaged targets, binary assets in LFS and represented to reviewers by manifest + captures, not diff. The frozen-PR-boundary allowlist gains a `binary` class whose evidence is "asset touched + test/capture result", and per-AC mutation evidence (§4.6) is required only where the toolchain declares mutation feasible.

### 4.3 Harness port — the `claude -p` adapter and error handling

Layer 0 exists (`spikes/runner`, ~8.9k lines, ~110 tests) and becomes the Claude Code adapter. Its contract, each line traceable to a finding:

| Rule | Source |
| --- | --- |
| `--output-format stream-json --verbose`; unknown events retained, never dropped | F1, F4 |
| `--strict-mcp-config` (+ explicit `--mcp-config` when needed); verify tool count from `system/init` | F8 |
| `--agents <card-json> --agent <name>`; verify adoption by **set equality** of effective tools | F7, F17, F20 |
| Card `tools` must include `StructuredOutput` whenever `--json-schema` is passed; assert `structured_output` present; unwrap a stray `{"output": …}` envelope (SDK issue #571) | F20 |
| `--system-prompt` = the card body (static); volatile input in the user message (stable prefix / volatile suffix). *Rev. 4:* measured at 1.000 across commits **and** across nine steady-state worktrees after the first write (S-WT: w1 0.807, w2–w10 1.000), at the price of a **full prefix rewrite (~30k review-scale) after the ~1 h TTL** — the static prompt forfeits the account-wide subsidy that keeps the default prompt's head warm (S-H1b, F28 correction). The append route (`--exclude-dynamic-system-prompt-sections` + `--append-system-prompt`) is a config option pending S-H1c (D16). Pin `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and the CLI version: the cache is org-scoped and a CLI upgrade anywhere on the account invalidates the shared tool block | F16, S-WT, S-H1b, L6.4 |
| `--effort` and `model` from the card, never inherited; a missing value is a config error | F15, amendment §1 |
| `--permission-mode` explicit; `--permission-prompts none` (≥2.1.259) so nothing can wait on a prompt | platform docs |
| **Never `--bare`** on a subscription — it disables OAuth and requires an API key | charter §6, platform docs |
| Treat `is_error`/`api_error_status` as `StepError::Platform`, regardless of exit code | F21 |
| Read `rate_limit_event`; refuse to dispatch inside a margin; margins ≥2 ticks; refuse when `isUsingOverage` | F1, F6, charter §6 |
| Timeouts sized to the slow condition; process-group kill on cancel (SIGTERM → 143, no result) | F21, platform docs |
| Pin `TESTED_CLI_VERSION`; record the version on every event | R8, F20 |
| Fan-out: stagger identical-prefix siblings ~5s so the first pays the write | platform docs, R7 |

**Error handling is layered, typed and budgeted.**

```rust
pub enum StepError {
    Transport, Platform { status: u16, retry_after: Option<Duration> }, RateLimited { resets_at: Timestamp },
    Timeout, Cancelled, NonZero(i32), Schema { attempts: u8 }, NotAdopted, NotIsolated, Unsupported(Capability), Overage,
}
```

| Layer | Who | Policy |
| --- | --- | --- |
| Inner | the harness itself (Claude Code emits `system/api_retry` with `attempt`, `max_retries`, `retry_delay_ms`, `error`) | **Observe, record, do not duplicate.** Inner attempts count toward the activity's budget |
| Outer | kernel retry policy per error class | `Transport`/`Platform 5xx`/`Timeout`: exponential backoff with full jitter (base 2s, cap 5 min), ≤ N attempts from the pack. `RateLimited`: **no sleep** — write a `retry_at(resets_at)` mark and exit; the next `run` resumes (principle 3). `Schema`: re-prompt once with the validation error, then fail. `NotAdopted`/`NotIsolated`/`Unsupported`/`Overage`: never retried — configuration, not weather |
| Budgets | three counters, never shared (L5.5) | attempts per activity, tokens per item (appetite), wall-clock per run |
| Degradation | policy, never silent | a model substitution (Opus → Sonnet) is allowed only if the pack says so and is recorded as `Degraded { from, to, why }` in the event and the digest; a review produced by a degraded tier cannot close a gate whose policy demands the tier |
| Circuit breaker | per model family | after K consecutive `Platform` failures, hold dispatch for the family behind a durable mark; other families continue |

Idempotency: every step is its own process (H4 amendment), so re-dispatch is safe; a step that *committed* is memoised by its key and never re-runs. `TransportError` and "the reviewer said no" remain distinct types end-to-end (RK-B0013).

The Codex adapter maps the same contract onto `codex exec --json --output-schema <file> -o`, whose JSONL stream (`thread.*`, `turn.*`, `item.*`, `error`) is classified the same way; capability gaps (effort pinning, per-agent cards, tool allowlists) are registry entries with a date, not assumptions.

### 4.4 Typed artifacts — markdown is a rendering

Every activity output is a Rust type (`serde` + `schemars`-derived JSON Schema). The harness receives the schema; the kernel validates the value at the boundary; the store **renders** markdown from the value with `minijinja` templates and **never parses markdown to recover structure**. Prompts are compiled the same way: `Prompt { stable: StablePrefix, volatile: RuntimeInput }` (L6.4) assembled from typed artifacts, KB atoms selected by typed inclusion rules, and the previous step's typed digest — which H4 shows is sufficient for the next step's file selection.

Design rules for the types: flat where possible, no `oneOf` nesting the model handles badly, every enum closed, `additionalProperties: false`, required-first (Anthropic's caps: ≤24 optional, ≤16 union fields), enum values compared case-insensitively, a free-text `rationale` beside every enum that carries a judgement, and a `schema_version` per type. *Rev. 4, from S-MIG:* the AC type is `{ id, criterion: text, gwt: Option<Gwt>, verification: Option<text> }` — 12% of shipped specs write criteria without GIVEN/WHEN/THEN or a verification note, and the type accepts them while the policy decides what a missing verification blocks; gate results are **emitted by the toolchain and rendered**, never parsed (free-form in 25% of reports and 83% of reviews); each companion (`_integration`, `_grounding`, `_decomposition`, `_ledger`) is a sidecar with its own schema; `plan.md` does not exist (110/110 were the unfilled scaffold); every activity input emits an `artifact.read` event so a stage whose artifact is never read is flagged by `doctor`. A schema change is a migration, and the old redkiln's `template_sig`/`rendered_sig` drift is the failure to avoid.

**Human edits (D5):** humans edit *designated prose sections* of the `.md` directly in an editor or PR; every structured field (ACs, boundary, status, findings) changes only through CLI, web or gate answers and is re-rendered. The one piece of markdown logic that survives is a **section locator** for the opaque prose fields — a heading-anchored slice, not a grammar — and it is tested as such. Three layouts were considered; **C was chosen (D6)**:

| Option | Layout | Human edits | Trade-off |
| --- | --- | --- | --- |
| **A — sidecar** | `spec.json` canonical + `spec.md` rendered (marked generated) | only via CLI/web/gate answers; markdown is read-only | cleanest machine contract; two files per artifact in git review; humans who edit the `.md` are ignored (drift surfaced) |
| **B — embedded** | one `spec.md`: rendered body above, canonical payload in a trailing fenced block with a digest of the rendering | edits to the rendered part detected by digest mismatch → typed drift → re-import via a narrow LLM activity or reject | one file; git diffs show both; import path is a place structure can rot back in |
| **C — split fields** | frontmatter + declared *opaque prose* fields are human-editable; every structured field (ACs, boundary, status, findings) is rendered from types and read-only | humans edit prose sections freely; structured sections regenerate | matches how Hugo/Obsidian/Backlog.md treat frontmatter + body; requires a section grammar, but only for *locating* opaque fields, not parsing their content |

Concretely: human-facing artifacts (`spec.md`, `plan.md`, `story.md`) are typed frontmatter + rendered structured sections (read-only, digest-stamped; an edit inside one is typed drift, surfaced not applied) + opaque prose sections. Machine-only artifacts (`_review`, `_ledger`, digests, run reports) are JSON sidecars rendered to `.md` on demand. Nothing parses structure out of prose.

### 4.5 Durable state — git first, event log second

Two substrates, deliberately redundant, split by tier:

- **Tier 1 (decision-bearing) is written to git first**: stage transitions, gate opened/answered, verdicts, slice seals — as trailers on checkpoint commits or an empty commit when no checkpoint is due. Test: "if the database were deleted, would `redkiln run` take a different next action?" If yes, it may not live only in the database.
- **Tier 2 (telemetry) is EventStore-only** and rebuildable-or-disposable: usage, cost, headroom, wall clock, drift observations, inner retries.

Consistency rules across worktrees and git operations:

1. One store per repository at `git rev-parse --git-common-dir`/redkiln/, shared by every worktree (redkiln's locks already live there; Beads' worktree redirect and v1.0 common-dir placement are the same shape).
2. Every repo-referencing row is keyed by SHA/tree hash, never branch or path (redkiln's verify-cache already keys on `(cmd, base, tree)`). Rebase, reset, amend and force-push produce *orphaned* rows, surfaced as drift, never deleted silently.
3. `reconcile()` at every entry, git → store, never the reverse; store rebuildable from a clean clone (`redkiln doctor --rebuild`), and that rebuild is a CI test.
4. No git hooks feeding the store: git-branchless's `reference-transaction` hook approach cannot be made atomic and forces full-log replays (its issue #1334); the single-write-path rule means the kernel is the only writer of tier-1 commits and writes git then store in one code path.
5. Identity rows: repo identity (root commit + common-dir), schema version, harness versions, boot-id; a store opened against a different repo is refused (Backlog.md #558, shipmates L5.3).
6. Cross-machine sharing, if ever needed, goes through git (append-only notes ref, `cat_sort_uniq`, as Gerrit NoteDb and git-appraise do) — deferred. No shared-database adapter is planned (D3); "one machine per repo, git for sharing" is the operating assumption.

Resume is one idempotent entry point — `redkiln run <item>` — with no `--resume` flag (L2.3). H2's 174/174 covers the state carrier; "same next action" is a replay test in the kernel crate.

### 4.6 Gates: human, verification, review

**Human gate — a mark, not a process.** Opening a gate writes a `Gate:` trailer and a `gate_opened` event, then the process exits 0. Answering is a durable write from any channel — CLI (`redkiln answer <item> approve|changes-requested`; the only channel in Phase 1, D9), a tracker comment grammar (`/redkiln approve`) ingested through the Tracker port's inbox, or the web view's single write (both Phase 3) — all landing in the same `reconcile()`. Resume re-derives from the world before acting; a drifted item re-opens the gate with the drift attached. *Rev. 4:* the gate record is `(token, tree_hash, deadline)` — a single-use token, the tree hash the human saw, and an expiry with a terminal path (fail / escalate / auto-decide, never a hang); an answer whose tree hash no longer matches is **void, not reconciled** (Temporal, Restate, Step Functions, GitHub environments all mint a token; only GitHub pins a SHA; binding to the tree hash is stronger). Placement is a value decision: a resume costs at most one prefix rewrite (18.6k under the default prompt at both 66 min and 49.7 h; ~30k under a card prompt at 66 min, its 24 h read untaken — F28 corrected, S-H1b), so gate *duration* is free and gate *frequency* is marginal. Gates go before every expensive phase, at the integration artifact rather than every unit (L5.10). Autonomy per stage × classification: L1 report / L2 gated / L3 unattended-allowlisted, in the policy table; do not ship a level you will retract. This is the gap the platform's own workflow runtime leaves open ("No mid-run user input … run each stage as its own workflow"; resume within a session only).

**Review gate — reviewer proposes, machine confirms** (*rev. 4*, rebuilt on S-VAR and S-TYPED; the data and prior art are in `2026-09-07-design-recommendation.md` §2). S-VAR: the reviewer's *verdict* flips on 3 of 10 draws for an item with a reproduced regression while its rationale still names the defect in most approving draws (a post-hoc keyword read, not a registered metric; S-TYPED's contradiction secondary is the measured form) — detection is stable, the threshold is not (the pairwise–pointwise paradox, arXiv 2606.13685; the *Automated Software Engineering* 2026 study on rationale/verdict contradictions). S-TYPED: with the verdict removed, detection stays at ceiling (23/23, 5/5) and the reviewer surfaces blocking-grade findings on 12 of 23 shipped, human-approved fixes, **14/14 of the new ones confirmed by mechanical re-execution** — CLEAN's ground truth is wrong for ≥52% of the corpus, and the reviewer's class *labels* wobble at boundaries while its *claims* do not. Therefore:

1. The card returns `Findings { items: [{ ac, claim, evidence (advisory label), rationale, files[], repro { command, observed, expected_when_met } }] }` — **no `pass`**, and every finding is an executable claim. Schema conformance measured 52/52.
2. **The evidence class is computed in Rust from the repro record**, not taken from the model: `reproduced` iff a command ran and `observed ≠ expected_when_met`; `mutation-survived` iff a mutation was applied and the named suite stayed green; `narrowed-pin` iff only the discriminating assertion flips; otherwise `advisory`. The model's label is kept for drift monitoring (arXiv 2608.02677: model reports evidence, code applies the rule).
3. **A finding blocks only after the deterministic gate re-executes it and it flips** — fails on the candidate, passes on the reference; or the mutant survives when re-applied. Both-fail or cannot-run is `Unverifiable` → advisory (Agentless: 213/300 reproductions, 94 flipped; Jin & Chen's verification filter).
4. The policy table is a stage-weight parameter and the only place a class becomes a block: `reproduced`/`mutation-survived` block; `narrowed-pin` blocks under a strict weight (12/23 vs 9/23 on CLEAN); `advisory` never. S-VAR's class ordering (0.70 > 0.37 > 0.10 > 0.00) sets the default order; its levels (n=10, ±30 pp) set nothing.
5. `Quarantined { reason, by, until }` is a first-class finding state: a human override stops a finding blocking and is not re-raised on re-review (GitLab quarantine; Cloudflare "won't fix").
6. Every block and override is an event; `false_block_rate` and per-card `pass^k` are telemetry (Cloudflare's 0.6% break-glass is the only public comparator). False-block is measured against *adjudication*, never against historical approval.
7. S-SUBTLE-shaped negative-control fixtures stay in the corpus as the reviewer's own regression test (a stable approval is consistent with a lenient judge).
8. Per-AC mutation evidence is the product's normal TDD evidence (F25 remediation; the columns exist in 0/156 historical reports), and "reviewed and green" is not evidence that a criterion is pinned (Google's review-start median of one live mutant per changelist, ICSE 2021).

The reviewer keeps `Bash` — F17/F26 show it is what makes the reviewer better than the suite — but in a **disposable shallow clone** at the reviewed SHA, so GATE-02's write-tool rule is satisfied by construction. An absent digest is `Err`; a verdict is `GreenAt(sha)` (L2.5); `basis` semantics are pinned by a test that captures the field (B6). **Cost:** execution-grade review runs +40% to +101% over the prose reviewer depending on condition (+71% on CLEAN) (median $3.03, ~18 min per CLEAN review; Cloudflare's read-only full tier is $1.68, Anthropic's verification-heavy review $15–25), so the review stage weight selects the tier — `0.2` read-only advisory, `1.0` execution-grade — and `bug-standard` is execution-grade because the fix lane is where the 52% lives (D15). No cheap-first cascade until its recall loss is measured.

**Verification gate.** `verify` receives the changed-file set and the toolchain's selection policy; a whole-suite fallback is recorded as `basis: whole-suite` so the point of use says what ran (B3). Deterministic gates and CI remain independently authoritative (L1.3).

### 4.7 Cost and parallelism

Attribution is free — every activity is its own child with `usage`, `total_cost_usd`, headroom delta and version on the event (L6.3). Cache: one static `--system-prompt` per card (one-time cold write, then 1.000 across commits); a worktree costs ~31% unmitigated and should cost ~0 under `--system-prompt` (unrun combination, §8). Budget: per-item appetite fixed at triage; `five_hour` margin as the platform ceiling. Parallelism: strictly sequential within a tree; parallel across project-DAG antichains, one worktree each, serialised at a merge queue; per-story failure isolation quarantines dependents (L6.1/L6.2); fan-out staggered and CPU-bounded. The fix lane stays lean: one agent + tests + policy review.

### 4.8 Projections

- **Tracker (GitHub adapter):** labels under one namespace (`rk:kind/*` human input; `rk:stage/*`, `rk:gate/*` orchestrator-only), sub-issues for containment, `blocked_by` for the story DAG, node IDs in frontmatter, per-field dirty-delta sync, idempotency key on every write, GitHub App + webhooks, agents hold **no** tracker token, inbound text is untrusted. Linear adapter: same port over GraphQL + signed webhooks; Symphony's "Linear as the state machine" is the anti-pattern.
- **DocsSite (mdBook adapter):** `docs/` as a Closeout artifact, Diátaxis-classified typed `DocSet`; agents propose, CI + CODEOWNERS gate; generator-agnostic invariants (build, links, drift-on-source-change) in the kernel, generator checks (`mdbook test`) in the adapter. Astro/Starlight and rustdoc adapters render the same `DocSet`.
- **Web view:** read-mostly projection of the event log with one write (answer a gate); built after CLI status and the tracker projection.
- **`.kb`:** intake → adjudicate → integrate; ADRs immutable (supersede); atoms are typed artifacts injected per activity by declared inclusion rules and included in the activity cache key.

### 4.9 Distribution and the plugin surface

The plugin stays a plain git clone; it contains **generated** `commands/`, `agents/`, `skills/` (and Codex/Pi equivalents from the same source, the wshobson/agents pattern) plus a tiny JS shim that locates or fetches the platform binary from GitHub Releases (`cargo-dist`). The `PreToolUse` frontmatter deny survives as a stateless predicate (L1.2). Every installed artifact is namespaced from day one (L3.1); the generator's output count is the assertion (L5.8).

## 5. Clean break and migration

Redkiln-RS shares no code, file format or runtime with the old redkiln. What crosses over, and how:

| From the old redkiln | Crosses as | Not as |
| --- | --- | --- |
| `.bklg` / `.kb` content | input to **`redkiln-migrate`**, a separate tool that emits the new typed layout in one reviewed commit, with a differential report | a runtime compatibility path |
| Process packs (`.redkiln/processes/*.yaml`) | content for the new TOML process packs | code |
| 101 CLI-behaviour test files | black-box specifications where the behaviour is wanted | ported tests |
| Recorded bugs (lock/id collisions, `--stay`, RK-B0013, `basis` flip, F25's per-AC gap) | test cases in the new crates | fixes carried over |
| `reconcile.rs` | the one code import — rewritten in the spike, validated on 360 commits / 74 slices (F12) | — |
| The frozen corpus @ `47faec9f`, the E3 fixtures, the JSONL results and the spike runner | move to a separate **`redkiln-corpus`** repo (D13), pinned by SHA from redkiln-rs; the evaluation set for §8 and the migration tool's fixtures | a permanent resident of the product repo |

The old redkiln keeps running its own development until Redkiln-RS is **self-hosting** (it runs its own fix lane end to end); that milestone, not a phased cutover, is the switch. The YAML byte-parity risk (R3) is dissolved rather than mitigated: the new store owns its serialiser and the migration commit normalises everything once.

## 6. Crate layout

| Crate | Contents | Origin |
| --- | --- | --- |
| `rk-types` | ids, typed artifacts (`Spec`, `ImplReport`, `Findings`, `ChangeSummary`, `DocSet`, digests), schema derivation, `StepError`, `Drift`, `GateBasis` | new |
| `rk-kernel` | packs (process + technology), stages/activities, policy, memo, gates, budgets, retry, replay | new |
| `rk-store` | `.bklg`/`.kb` typed store, canonical serialiser, markdown rendering, locks, index | new |
| `rk-git` | trailers, `reconcile()`, worktrees, disposable clones, merge queue (`gix`) | `reconcile.rs` + new |
| `rk-ports` | the five traits + fakes | new |
| `rk-adapter-sqlite`, `-claude`, `-github`, `-mdbook`, `-toolchain-{node,dotnet,python,unreal}`; `-codex` after S-CODEX | one crate per adapter; `-claude` from `spikes/runner` (runner.rs, event.rs, step.rs, throttle.rs, journal.rs); subscription-only, asserts no `ANTHROPIC_API_KEY` (D10) | mixed |
| `rk-cli` | `clap` router; one idempotent `run` entry; `answer`, `status`, `doctor` | new |
| `rk-plugin-gen` | emit harness artifacts from packs/cards | new |
| `redkiln-migrate` | separate binary; old `.bklg`/`.kb` → new layout | new |

Dependencies proven in the runner: `tokio`, `clap`, `serde`/`serde_json`, `jsonschema`, `minijinja`, `toml`, `thiserror`, `sha2`. Add `schemars`, `rusqlite` (bundled), `gix`. No durable-execution framework (Temporal/Restate mandate a server); the memo table + replayable log is ~800 LOC.

## 7. Phasing

| Phase | Enter when | Scope | Exit criterion |
| --- | --- | --- | --- |
| **0 — close the programme + spike the new bets** | now | Commit the working tree; extract `redkiln-corpus` (D13); GATE-01 one-dispatch check; Ryan's blind severity annotation (`e3-severity-annotation-BLANK.md`); the §8 spikes marked *gates Phase 1* | RFC amendment closed with F28+; §8 results in `FINDINGS.md` |
| **1 — kernel, store, fix + story lanes on a fresh repo** | Phase 0 | `rk-types`/`rk-kernel`/`rk-store`/`rk-git`/`rk-ports`; SQLite, Claude, Node adapters + fakes; `bug-standard` and `story` process packs (D11); findings-policy review; mark-based gate, CLI answers only (D9); typed artifacts per D5/D6; `ChangeSummary` designed against the UE fixture (D8); `redkiln-migrate` v0 | Runs its own fix lane end to end (self-hosting) and Phase 2's stories are authored under it; `--stay`-class error unrepresentable; deterministic resume across kill; per-step cost recorded; `--system-prompt` + worktree combination measured |
| **2 — packs and parallelism** | Phase 1 self-hosting | Remaining process packs (incident, spike, initiative, project) with weights and fast path; project DAG in parallel worktrees + merge queue; .NET, Python and **Unreal** toolchain adapters (D8); `rk-plugin-gen`; migration of the old corpus | An initiative advanced end to end headlessly; migration differential green on the frozen corpus; a UE item verified through the adapter |
| **3 — surfaces** | Phase 2 stable on one initiative | GitHub adapter + comment grammar; mdBook adapter and docs pipeline; web view; DORA metrics from the event log | A gate answered from a phone; a collaborator with no CLI sees status |
| **4 — reach** | Phase 3 stable for a quarter | Codex adapter (only after S-CODEX passes, D7); Linear and Astro adapters as demanded | — |

## 8. Spikes and research the design now needs

Each is cheap because the rig exists; each names the decision it feeds and whether it gates Phase 1.

| Spike | Question | Feeds | Gates Phase 1? | Cost |
| --- | --- | --- | --- | --- |
| **S-VAR** verdict stability at n=10 | Per-item flip rate on 5 CLEAN items; does evidence class predict stability (F27's hypothesis)? | findings/policy split (D4); policy thresholds | **yes** | ~50 Opus dispatches, ~$60 |
| **S-TYPED** typed vs prose review | Same card, same fixtures: does demanding `Findings` (typed, closed enums) change detection or FP vs the prose digest? | §4.4 types; schema design rules | **yes** | 23 paired Opus dispatches |
| **S-BIN** binary-artifact review | On the real UE project (D8): 5 items whose change is asset manifest + automation report + captures, no diff; can the reviewer verify ACs and catch a seeded regression? | `ChangeSummary` type; Toolchain port | **yes** (type design) | ~10 Opus dispatches + UE automation runs |
| **S-MIG** migration differential | `redkiln-migrate` over the frozen corpus: every item round-trips to typed values under layout C and re-renders; count what cannot be typed | store schema; D6 validation | **yes** | zero model cost |
| **S-H1b** `--system-prompt` across time | Does a static system prompt remove the 18.6k resume cost? | gate/cache strategy | no | 3 haiku reads (plumbed already) |
| **S-WT** `--system-prompt` + worktree | Does the 31% per-worktree cost vanish under a static prompt? | DAG parallelism pricing (Phase 2) | no | 20 haiku dispatches |
| **S-RETRY** error taxonomy under load | Observe `system/api_retry` and `rate_limit_event` across a 5-hour window at the throttle margin; confirm `resets_at` semantics and inner-retry budgets | §4.3 retry policy | no (policy is configurable) | telemetry only, haiku |
| **S-CODEX** harness portability | Run E3 arm (b) through `codex exec --json --output-schema` on the same fixtures (D7); detection, FP, cost, missing capabilities | Harness port viability; registry contents; Phase 4 adapter | no | 46 Codex dispatches |
| **R-LINEAR** research | Linear's relations/sub-issue model, webhook signing, rate limits vs the Tracker port | Tracker port shape (Phase 3) | no | reading |
| **R-DOCS** research | Astro/Starlight and rustdoc as `DocSet` renderers; which invariants are generator-agnostic | DocsSite port (Phase 3) | no | reading |

## 9. Decisions taken (2026-09-05)

| # | Decision | Choice | Alternatives considered |
| --- | --- | --- | --- |
| D1 | Architecture | Inversion (Bet 1) on the §1 evidence | — |
| D2 | Relationship to old redkiln | Clean break; `redkiln-migrate` as a separate tool; self-hosting is the switch | phased cutover; engine adapter |
| D3 | Ports | The five in §4.1, each with a fake; git is not a port; **no Postgres adapter** — the EventStore port exists for testing and future safety only; "one machine per repo, git for sharing" is the operating assumption | Postgres adapter for a fleet |
| D4 | Review gate | Findings, not verdicts; Rust policy computes the block; confirmed pending S-VAR | model returns verdict; more repeats before redesign |
| D5 | Human edits | Prose free, structure gated: designated prose sections editable in an editor/PR; structured fields only via CLI/web/gate; one section locator survives, no grammar | everything via forms; free edit + LLM re-import |
| D6 | Artifact layout | C: typed frontmatter + rendered read-only structured sections + opaque prose for human-facing artifacts; JSON sidecars for machine-only artifacts | A sidecar everywhere; B embedded payload |
| D7 | Second harness | Codex CLI, validation only (S-CODEX); no non-Claude adapter in a gate until it passes E3-shaped checks | Pi; defer |
| D8 | Unreal | A real UE project exists and is the S-BIN fixture in Phase 0; `ChangeSummary` is designed against it; Unreal toolchain adapter ships in Phase 2 | minimal synthetic fixture; paper design |
| D9 | Gate channels (Phase 1) | CLI only; tracker comment grammar and web view in Phase 3 | CLI + GitHub comments |
| D10 | Claude adapter auth | Subscription only, never `--bare`; adapter asserts `ANTHROPIC_API_KEY` absent; CI is a Phase 3+ question | API-key mode option |
| D11 | Phase 1 packs | `bug-standard` + `story`, so Phase 2's own stories are authored under Redkiln-RS | fix lane only; full set |
| D12 | Open-question rule | Remaining unknowns are carried as §8 spikes with a named consumer, not as prose | — |
| D13 | Corpus home | Separate `redkiln-corpus` repo (pinned SHA, fixtures, JSONL, spike runner), depended on by SHA | keep under `spikes/`; archive results only |
| D14 | Pack composition | Process packs own structure (stages, activities, roles, slots, tier per role); technology packs bind (cards, skills) and contribute at slots only; a different shape is a process-pack variant, never a merge (§4.2.1) | tech packs defining activities directly; cards named in activities |
| **D4 (rev. 4)** | Review gate | Rebuilt as *reviewer proposes, machine confirms*: class computed from the repro record; flip re-executed before a block; `Unverifiable`/`Quarantined`; policy per stage weight; block/override telemetry; negative-control fixtures (§4.6) | model-chosen class; block on the model's word; more repeats before redesign |
| **D5/D6 (rev. 4)** | Artifacts | AC type with optional GWT/verification; gates emitted not parsed; sidecar schema per companion; `plan.md` removed; `artifact.read` events (§4.4) | strict AC grammar; parsing gate prose |
| **D15** (2026-09-07) | Review tiering | Review stage weight selects read-only (`0.2`) vs execution-grade (`1.0`); `bug-standard` execution-grade; no cascade until recall loss is measured | execution-grade everywhere; read-only default |
| **D16** (2026-09-07) | Card delivery | Static `--system-prompt` for Phase 1 (1.000 across commits and across w2–w10 of ten worktrees; one ~30k rewrite per post-TTL resume); append route a config option pending S-H1c; TTL and CLI version pinned | append by default (unmeasured); heartbeat keep-alive (quota cost undocumented) |
| **D17** (2026-09-07) | Gate record | `(token, tree_hash, deadline)`; stale answer void; terminal expiry path | item-keyed answer; reconcile stale answers |
| **D18** (2026-09-07) | Rewrite events + worktree ownership | `rewrite(old_sha → new_sha)` events on rebase/amend; kernel resolves the common dir at startup and refuses to write when the worktree's HEAD is not the run's branch | path-keyed rows; trust the cwd |
| **D19** (2026-09-07) | Per-AC discriminating test is the norm | "Reviewed and green" is not evidence of pinning; the reviewer's job on a fix is to produce the discriminating check the author did not | test-green as sufficient |

**Still open after rev. 4:** S-H1c (append route) and S-BIN (binary artifacts) are registered and unrun; the h1b-24h read was not taken (it could only confirm a prefix already fully cold at 66 min); S-VAR levels are n=10 on one repo; CLEAN ground truth is unusable as a false-positive denominator, so false-block is measured against adjudication only.

## 10. Sources

**Repo:** `spikes/FINDINGS.md` F1–F28 (F28 committed 2026-09-05 in `56a4e85`); `spikes/results/h1-gap.jsonl`, `h4.jsonl`; `spikes/METHOD-H.md`; `references/2026-08-31-rust-inversion-RFC.md`; `references/2026-09-02-rfc-amendment.md`; `references/2026-09-03-reimagining-brainstorm.md` and `references/research/*`; `references/conformance-2026-09-04-scorecard.md`; `references/2026-09-04-findings-for-product.md`; `references/2026-09-01-shipmates-lessons-for-redkiln-rs.md`; `spikes/runner/src/*`; `spikes/remediation/`.

**Platform (fetched 2026-09-05):** [Run Claude Code programmatically](https://code.claude.com/docs/en/headless) — `--bare` requires an API key and "will become the default for `-p`"; `--permission-prompts none` (≥2.1.259); SIGTERM exits 143 with no result; `system/api_retry` fields; `-p` runs project hooks and `.mcp.json` without a trust dialog. [Dynamic workflows](https://code.claude.com/docs/en/workflows) — "No mid-run user input … run each stage as its own workflow"; resume within a session only; workflow agents on a 5-minute TTL by default (`subagentPromptCacheTtl: 1h`); prefix stagger 5000 ms; ≤16 concurrent agents. [Structured outputs in the SDK](https://docs.claude.com/en/docs/agent-sdk/structured-outputs); [claude-agent-sdk-python #571](https://github.com/anthropics/claude-agent-sdk-python/issues/571) (`{"output": …}` envelope). [Codex non-interactive mode](https://developers.openai.com/codex/noninteractive) — `codex exec`, `--json` JSONL events, `--output-schema`, `-o`. [Linear webhooks](https://linear.app/developers/webhooks) and [GraphQL](https://linear.app/developers/graphql). [Unreal: run automation tests](https://dev.epicgames.com/documentation/en-us/unreal-engine/run-automation-tests-in-unreal-engine) and [Gauntlet](https://dev.epicgames.com/documentation/en-us/unreal-engine/running-gauntlet-tests-in-unreal-engine) — `-ExecCmds="Automation RunTests …"`, `-NullRHI`, `-ReportExportPath` JSON.

**Precedent for store consistency:** [git-branchless architecture](https://github.com/arxanas/git-branchless/wiki/Architecture) and [#1334](https://github.com/arxanas/git-branchless/issues/1334); [Beads WORKTREES.md](https://github.com/steveyegge/beads/blob/main/docs/WORKTREES.md); Gerrit NoteDb; Jujutsu's operation log.

**Pack composition precedent (§4.2.1):** Nx targets vs executors; Tekton Pipelines vs Tasks; Cloud Native Buildpacks detect/contribute ordering; GitHub reusable workflows vs composite actions; Kiro conditional steering inclusion.

**Literature and prior art:** Google Research scaling study (arXiv:2512.08296) — sequential −39…−70%, parallel +80.9%; Temporal / [Restate](https://restate.dev/what-is-durable-execution) workflow–activity split and journaled side effects; AWS Architecture Blog, "Exponential backoff and jitter"; Disciplined Agile "Choose your WoW"; ISO/IEC/IEEE 12207 tailoring; ITIL 4 change types; Dagster software-defined assets; Tekton `finally`/results; shipmates issue #1 ("a referee, not a conductor").
