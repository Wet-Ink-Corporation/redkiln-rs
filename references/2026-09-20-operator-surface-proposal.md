# The operator surface — a terminal HUD, board and gate queue for redkiln-rs

**Status:** PROPOSAL. Nothing here is decided; §10 lists nine decisions and §12.7 four more.
**Date:** 2026-09-20, **amended 2026-09-21** — §12 records the operator's ruling that the read model is a
requirement, and answers whether `happenstance` should build it. **Tree:** `build/phase-0-to-1` @ `1308311`, Step 7d Slice 4.
**House style:** every claim is tagged `[measured]` / `[documented]` / `[inferred]` and cites where it
came from. Claims about this repository cite a path and are checkable by reading it. Claims about the
outside world cite a fetch date. Where a number would be a guess, there is no number.

---

## 0. The proposal in one paragraph

Build a **terminal projection** — one extra binary, `redkiln-view`, beside `redkiln` and
`redkiln-migrate` — that renders six views over the event journal and the on-disk run records, and
whose **only** write is to `exec` `redkiln answer`. `ratatui` is a sound choice for it and the
dependency cost is unusually small. **But the terminal is not the blocker and ratatui is not the
question.** The question is the *read model*, and the read model is not there yet: `Event` is
`{ kind, payload: serde_json::Value }` with no item, no run id and no timestamp
(`crates/rk-types/src/event.rs:61`), the SQLite adapter's `query` ignores its selector and returns the
whole table (`crates/rk-adapter-sqlite/src/lib.rs:322`), and `status` already over-reports on a
multi-item store and says so in its own header (`crates/rk-cli/src/verbs/status.rs`, the RAISED
block). Every board column, every badge, every sparkline in the brainstorm is a *per-item, per-time*
partition of a journal that has neither key. So the recommendation is inverted from the one that was
asked for: **do not build the TUI first — write the view's data contract first, let it be what pins
the typed event payload that is already recorded as OWED, and build the terminal against it.** The
design document has already reserved the slot this would occupy (§8.6's *Web view* row, Phase 3) and
has already recorded the rule the terminal must obey (*"read-mostly, exactly one write: answer a
gate"*).

### 0.1 What this document finds

| # | Finding | Grade |
| --- | --- | --- |
| **OS-1** | A TUI cannot be a `redkiln` subcommand. §8.1: *"Every verb is a read of the world, a decision, and an exit … no verb prompts and no verb reads stdin, so nothing here takes an interactive flag and nothing here is a mode"* (`crates/rk-cli/src/cli.rs:1–17`). A TUI is a mode with a stdin loop. It is a **separate binary**, on `redkiln-migrate`'s precedent (§8.1, AC-441) | `[documented]` |
| **OS-2** | It may write exactly once, and not directly. AC-405: the gate-answer store method has *"exactly one call site, in `rk-cli`'s `answer` command"* (design §8.3). A view that called the store would falsify AC-405. It must compose argv and `exec` the CLI | `[documented]` |
| **OS-3** | Per-**run** detail is buildable **today**: `golden-logs/<run_id>/` carries `run.json` (`run_id`, `stage`, `inputs_digest`, `event_count`, `terminal_state`, `next_action`), a full `inputs/` tree and `events.jsonl`, and `explain` already reads it back (`crates/rk-kernel/src/replay.rs:88–120`, `read_run`) | `[measured]` — read on the tree |
| **OS-4** | Cross-**item** views (board, swimlanes, WIP, any roll-up) are **blocked**, and not by the UI. Events carry no item, no run id, no timestamp; three tier-1 kinds carry no item at all; `status` hands them to every item because *"the store cannot tell and inventing an owner would be worse than over-reporting"* (`verbs/status.rs`, RAISED) | `[measured]` — read on the tree |
| **OS-5** | Any **time-series** view (burn-down, throughput, cost-over-time, cycle time, DORA) is blocked on the same gap plus one more: no event carries a wall-clock instant. `events.jsonl` gives **order**, not duration | `[measured]` |
| **OS-6** | `ratatui` is cheap and clean: `0.30.2` (2026-06-19), **MIT**, MSRV `1.88.0` against this repo's pinned `1.98.1`, and a required graph of three crates (`ratatui-core`, `ratatui-widgets`, `instability`) with every backend optional | `[measured]`, crates.io API, fetched 2026-09-20 |
| **OS-7** | The repo's own landscape research already argued the opposite conclusion — *"Build a **read-mostly run/gate view plus an approve button reachable from a phone**; keep the CLI authoritative"*, and named `gt dashboard` as the one a reviewer *"bounced off, ending up 'in tmux panes anyway'"* (`references/research/2026-09-03-landscape-sentiment.md:104`). **§7 argues that this does not transfer**, and says why | `[documented]` + `[inferred]` |
| **OS-8** | The two surfaces are not rivals. A terminal view serves the operator *at the keyboard, mid-loop*; a web view serves the operator *away from it, on a phone*. They differ in transport and share a read model. Deciding the read model decides both | `[inferred]` |

### 0.2 What it does not find

- It does **not** find that a TUI is a bad idea. It finds that building one before the read model
  exists produces a renderer that computes its own idea of state — which is the one failure mode that
  would be unrecoverable, because a projection that disagrees with the FSM is worse than no projection.
- It does **not** price the build. No line-count or hour estimate appears anywhere below; the spike in
  §9 exists to produce one.
- It takes **no position** on whether the web view should still be Phase 3.

---

## 1. The eight constraints any redkiln surface inherits

These are not preferences. Each is a recorded decision or an acceptance criterion, and each one
eliminates a shape the brainstorm might otherwise reach for.

| # | Constraint | Source | What it forbids |
| --- | --- | --- | --- |
| C1 | Every verb is a read, a decision and an exit; **nothing is a mode** | design §8.1; `cli.rs:1–17` | `redkiln tui`, `redkiln dashboard`, `redkiln status --watch`, any `--interactive` |
| C2 | A projection is **derived, never authoritative**, with **at most one write** | design §8.6 | A view that records state, resolves a finding, moves a stage, or edits an item |
| C3 | The one write is **answering a gate**, and its only call site is `rk-cli`'s `answer` | AC-405; design §8.3 | The view linking `rk-store`'s gate-answer method at all |
| C4 | **Nothing may depend on a process surviving** | F22 `[measured, n≈12 kills]`; design §8.1 | A daemon, a watcher service, a background indexer, a server the view needs running |
| C5 | The orchestrator **never writes to the user's checkout** | AC-518; ADR-0018 | A view cache, a layout file, a `.redkiln-view/` anywhere but under the resolved common dir |
| C6 | Inbound text is **untrusted and typed at the boundary**; nothing parses structure out of prose | design §8.6; ADR-0005/0006 | Scraping the agent's stdout for progress; regexing a commit message for a percentage |
| C7 | The kernel may not read a clock, the environment, or a terminal | design §3.4, §9.3 | Any view logic inside `rk-kernel`; the view owns its own clock and tick |
| C8 | Saturation is declared **at the row**, not discovered in a dashboard; a tracked-not-targeted metric may not be turned into a target by a display | `references/2026-09-08-success-measures.md:10`, `:566` | Rendering a threshold line, a red/green target, or a "health score" over a metric the register tracks without a target |

**C8 is the one most likely to be violated by accident**, and it is the most redkiln-specific thing in
this document. The success-measures register says it in terms: *"Do not set a target on it, do not let
a dashboard turn it into one, and do not count it as a success measure."* A gauge with a red zone
**is** a target. §6.4 turns this into three drawing rules.

**C4 deserves a second reading**, because it is the constraint that most favours the terminal over the
web. A local web view needs a server. A server is a surviving process. The Gas Town field review's
failure list is *"141 orphaned Claude Code processes; mail system broke; background daemons silently
not running"* `[documented, landscape-sentiment:37]`. A `redkiln-view` that is a foreground process in
the operator's own terminal, holding no state, whose death costs nothing, is the shape C4 actually
wants. That is an argument the landscape evidence in §2 does not make, and §7 picks it up.

---

## 2. What the landscape evidence already settled

The repo did this research on 2026-09-03 and re-verified it on 2026-09-08 and 2026-09-10. It is
quoted rather than re-derived.

| Claim | Evidence |
| --- | --- |
| *"Every dead tool had a beautiful UI; every survivor is CLI plus a thin view"* | `references/2026-09-03-reimagining-brainstorm.md:128` `[documented]` |
| Vibe Kanban at 28,040★ — *commercially dead, technically alive*; Roo-Code `archived: true` at 24,306★; Terragon shut down; Crystal rebranded with no commits since 2026-02-26; HumanLayer OSS largely deprecated for a closed rebuild | `references/2026-09-08-evidence-augmentation.md:147` `[documented, 1P GitHub API + vendor posts]` |
| *"Observability is the recurring complaint, not aesthetics"* — *"sometimes 6 PRs merged and I had no idea when I'd slung them"* | `landscape-sentiment.md:104` `[documented]` |
| `gt dashboard` — *"which Gas Town's reviewer bounced off, ending up 'in tmux panes anyway'"* | `landscape-sentiment.md:104` `[documented, single reviewer, n=1]` |
| The recommendation on record: *"Build a **read-mostly run/gate view plus an approve button reachable from a phone**; keep the CLI authoritative"* | `landscape-sentiment.md:104` `[documented]` |
| *"design the **event log first** … and make the web view a projection of it with exactly one write: approve/reject. Keep CLI and FSM authoritative"* | `reimagining-brainstorm.md:128` `[documented]` |
| A standalone local desktop app for individual devs is *"a proven graveyard"* | `landscape-sentiment.md:106` `[documented]` |

**Read carefully, this evidence does not say "do not build a terminal view."** It says four things,
and all four survive into §5 and §7:

1. The UI must not be the product. *(Redkiln's product is the artifact and the log; it already is.)*
2. The complaint it must answer is **observability**, not beauty.
3. A gate must be answerable **from a phone** — which no terminal can do, and which is why §7's
   recommendation keeps the web row rather than replacing it.
4. `gt dashboard`'s failure is `n=1` and **its cause is nameable**: it was a dashboard over a *process
   supervisor* whose processes were silently dead, so the dashboard's picture was false and tmux was
   the only ground truth. Redkiln's architecture inverts that premise exactly — no daemons (C4), a
   durable log, and a `reconcile()` before every decision. **A view over a durable log is a different
   artifact from a view over a process table**, and the one negative data point does not transfer
   without that being said out loud. `[inferred]` — and it is the load-bearing inference in this
   document, so it is flagged rather than buried.

One outside data point, fetched 2026-09-20, points the same way: `agenttrace` is a local-first TUI
that reads *finished* agent session logs (JSON/JSONL) and surfaces token spend, tool-failure patterns,
retry loops and incident timelines — i.e. the terminal observability layer built **over a log**, not
over a process. Its user feedback asked for *more* of that (milestone tracking, idle-gap analysis,
baseline regression comparison), not for a prettier UI `[documented, 1P GitHub discussion, fetched
2026-09-20]`. `n=1`, and it is not evidence that such a tool is *kept*.

---

## 3. The read model — the actual feasibility question

This section is the answer to *"is my idea feasible."* It is feasible; it is **gated on data that does
not exist yet**, and the gap is precisely locatable.

### 3.1 What exists today, measured on the tree

| Source | Shape | Keyed by | Ordered | Timed | Read by |
| --- | --- | --- | --- | --- | --- |
| **Run record** `golden-logs/<run_id>/run.json` | `{ schema, run_id, stage, inputs_digest, event_count, producer, terminal_state, next_action }` | **run_id** (and the item, inside `inputs/`) | — | no | `explain` (`replay::read_run`) |
| **Run inputs** `golden-logs/<run_id>/inputs/` | pack, catalog (one file per activity), artifact registry, `facets.toml`, `context.json`, `artifact-state.json`, `runtime.json` (`tree_hash`, `deadline`, `entered_from`, `successor`, `results_at`), `specs.json` (per DAG node: `activity_id_at_version`, `card_digest`, `tools`, `schema`, `timeout_secs`, `inputs`), optional `deviation.json` | run_id | — | no | `replay` |
| **Run journal** `golden-logs/<run_id>/events.jsonl` | one canonical JSON object per line, LF-terminated | run_id | **yes** | no | `replay` |
| **Event store** `t2_event(id, kind, payload, anchor_sha)` | `Event { kind: EventKind, payload: serde_json::Value }` | **nothing** | by rowid | no | `status`, the kernel fold |
| **Git** | trailers on checkpoints and empty commits; tier-1 kinds land here first | commit | yes | **yes** (commit time) | `reconcile()`, `doctor --rebuild` |

`[measured]` — `crates/rk-kernel/src/replay.rs:88–120`; `crates/rk-types/src/event.rs:61`;
`crates/rk-adapter-sqlite/src/lib.rs:308,322`.

### 3.2 The three gaps, named

**G1 — the payload is untyped, and it is already owed.** `rk-types/src/event.rs` says so itself:
*"design §5's event-kind table gives payload fields in prose, not types … So `payload` is
`serde_json::Value` **provisionally and visibly**. The typed-payload record, per kind, is **OWED**;
its earliest real consumer is the SQLite adapter at 7c."* A view is the *second* consumer, and a
better-shaped one — a renderer needs every field of every kind, so specifying the view specifies the
record. `[documented, on the tree]`

**G2 — no correlation triple.** No event carries `(item, run_id, at)`. The port declares
`Query::{ByItem, BySha, ByRun}` (`crates/rk-ports/src/event_store.rs:32–34`) and the adapter's `query`
is `SELECT kind, payload FROM t2_event ORDER BY id` with no `WHERE`. `status`'s own header states the
consequence: three tier-1 kinds carry no item at all — `stage_transition` is `{from, to}`,
`gate_opened` is `{deadline, kind, reach, token, tree_hash}`, `dag_assembled` carries neither — so
*"a multi-item store therefore over-reports its transitions and gates."* `[measured, on the tree]`

**G3 — no instant.** Nothing in the event carries a wall-clock time, and `RecordedRun` has no field
for one. Order is available; **duration is not**. `[measured]`

### 3.3 What each gap costs a view

| View element | G1 | G2 | G3 | Buildable today? |
| --- | --- | --- | --- | --- |
| Single-item HUD, stage rail, `TerminalState` / `NextAction` / last `Exit` | partly | — | — | **yes**, from the run record |
| Activity-DAG strip with memo hit/miss and committed marks | partly | — | — | **yes**, from `events.jsonl` + `specs.json` |
| Gate card — question, options, attachments, deadline, tree-hash drift | — | — | — | **yes** — `gate_opened` carries `{deadline, kind, reach, token, tree_hash}` and the registry carries the question |
| Live dispatch panel — tokens, cost, headroom gauges | **blocked** | — | partly | needs `dispatch_completed`'s typed `DispatchRecord` (ADR-0126, Step 7c Slice 1) |
| Evidence matrix — AC ledger × `LedgerStatus`, findings, quarantines | — | — | partly | **yes**, from the typed artifacts in `rk-store` |
| **Board / swimlanes / WIP** | — | **blocked** | — | **no** — and a board built on the current journal would *lie*, per `status`'s own RAISED note |
| **Any chart with time on an axis** | — | blocked | **blocked** | **no** |
| Cost roll-up per item / stage / activity | blocked | blocked | — | **no** |

The honest one-line answer to *"is this feasible"*: **the single-item HUD and the gate queue are
feasible on today's tree; the board and every chart are not, and the thing standing in the way is the
event record, not the terminal library.**

---

## 4. The distillation — the status vocabulary is already written

The brainstorm asks what statuses, progresses and badges are needed. They exist, as **closed Rust
enums**, and that is the single most important design fact in this document.

| Domain question | The closed type | Variants | Where |
| --- | --- | --- | --- |
| Where is this item? | `TerminalState` | `NotStarted` · `AwaitingGate{token}` · `AwaitingDispatch{stage}` · `StageComplete{stage}` · `Sealed{verdict}` | `rk-kernel/src/workflow/fsm.rs:444` |
| What happens next? | `NextAction` | `OpenGate` · `AwaitGate` · `Dispatch` · `AdvanceStage{from,to}` · `Halt{reason}` · `Done` | `fsm.rs:552` |
| Why did it stop? | `HaltReason` | `Failed` · `Refused{class}` · `GateExpired{token}` | `fsm.rs:518` |
| How did the process end? | `Exit` | 18 rows, explicit discriminants, snapshot-pinned (AC-438) | `rk-cli/src/exit.rs` |
| Why did a step fail? | `FailureClass` | `Transport` · `Schema` · `Timeout` · `NonZero` · `Platform` | `rk-types/src/event.rs:12` |
| Why was a step refused? | `RefusalClass` | `NotAdopted` · `NotIsolated` · `Unsupported` · `Overage` | `rk-types/src/event.rs:24` |
| Is this AC pinned? | `LedgerStatus` | `Pinned` · `MutationSurvived` · `Unpinned` · `NotFeasible{reason}` · `HumanAnswered{reason}` — with `permits_exit()` | `rk-types/src/artifact/ledger.rs:23` |
| How strong is a finding? | `EvidenceLabel` | `reproduced` · `mutation-survived` · `narrowed-pin` · `advisory` | `rk-types/src/artifact/findings.rs:25` |
| What disagrees with the record? | `Drift` | `StructuredSection` (**blocks**) · `Advisory` (**does not**) · `Template` · `UnparsableTrailer` · `LegacyGateTrailer` · stale-gate-answer | `rk-types/src/drift.rs:21` |
| What happened at all? | `EventKind` | **38** kinds, closed, each with a `Tier` (5 tier-1, 33 tier-2) | `rk-types/src/event_kind.rs:56` |
| What did review conclude? | `ReviewOutcome` | `{reviewed_tree, sha, basis, blocks, sink_records}` + `Quarantine{reason, by, until}` | `rk-types/src/review.rs:79,92` |
| What is the model costing? | `Usage`, `Timings`, `WindowReading{five_hour, seven_day}`, `CostUsd`, `CliVersion`, `DetectedBy` | — | `rk-types/src/usage.rs` |
| What can the human choose? | `Question` — options, and admissible attachment kinds per option | — | `rk-kernel/src/workflow/question.rs:83` |
| Where is it in the process? | stage | `discovery` · `analysis` · `specification` · `planning` · `design` · `implement` · `review` · `verify` · `release` · `operate` · `closeout` — **11** | `spikes/p9/catalog/*.toml` |

### 4.1 The rule this hands the renderer

> **Every badge, colour and glyph in the view is a total function of one closed enum from `rk-types`,
> written as an exhaustive `match` with no wildcard.**

This is not a style note. It is the same device `Exit::name()` and `EventKind::ALL` already use
(`exit.rs`, `event_kind.rs:145`): a new variant **fails to compile** in the renderer rather than
falling into a grey "unknown" chip that nobody notices. It converts the classic dashboard failure —
the display silently drifting behind the model — into a build error. It is also the cheapest
instrument in this document: one `match`, no test.

Its corollary is the crate lint in §8.3: **the view may not compute state.** It may fold events only
by calling `rk_kernel::workflow::fsm`, and it may lay out the values that come back. A second
implementation of "what stage is this in" inside a renderer is how a projection becomes a liar.

---

## 5. Six views, and what each one is *for*

The brainstorm names seven shapes — dashboard, pipeline, kanban, workflow, HUD, backlog, charts. Those
are seven *idioms*, not seven views. Collapsed by the question each answers, there are six views, and
one of them is worth building first by a wide margin.

The wireframes below are drawn at 100×34 because that is the fidelity of the finished product: a
terminal wireframe is not a sketch of the thing, it **is** the thing. Every field is annotated with
its source, and any field that is not available today is marked `‹blocked›` with the gap that blocks
it.

### 5.1 V1 — **The Loop** (the HUD). *"What is it doing, and what does it want from me?"*

The default view. One item. This is the view that answers the observability complaint, and it is the
one that should exist even if nothing else ever does.

```
┌ redkiln ─ RK-412 · verify gate refuses unpinned ACs ────── lane story · pack story@3 · rigor high ┐
│                                                                                                   │
│  discovery  analysis  spec  planning  design ▸ IMPLEMENT ◂ review  verify  release  operate  close│
│  ▰▰▰▰▰▰▰▰▰  ▰▰▰▰▰▰▰▰  ▰▰▰▰  ▰▰▰▰▰▰▰▰  ▰▰▰▰▰▰   ▰▰▰▰▱▱▱▱    ▱▱▱▱▱▱  ▱▱▱▱▱▱   ▱▱▱▱▱▱▱  ▱▱▱▱▱▱▱  ▱▱▱▱│
│                                                        ▲ entered 4 of 7 activities                │
├─────────────────────────────────────────────┬─────────────────────────────────────────────────────┤
│ STATE   AwaitingDispatch { stage: implement}│ DISPATCH  impl-tdd-cycle @ n7        attempt 2 of 3  │
│ NEXT    Dispatch { n7, impl-tdd-cycle@4 }   │ wall 04:12      api 03:58      timeout 15:00        │
│ HALT    —                                   │ in   128,402    out      9,114                      │
│ GATE    none open                           │ cache  read 96,000   creation 18,603   (84% warm)   │
│ LAST    Exit::RateLimited (15) · 2 runs ago │ cost  $2.41 this dispatch                           │
│ DRIFT   1 · Advisory (vouched root) ADVISORY│ 5h window  ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱ 74%                 │
│ REVIEW  2 blocks · 1 quarantined (3d left)  │ 7d window  ▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱ 31%                 │
│ VERIFY  14 ACs · 11 pinned · 2 unpinned ⛔  │            ‹blocked G1: needs DispatchRecord›        │
├─────────────────────────────────────────────┴─────────────────────────────────────────────────────┤
│ DAG · stage implement · 7 nodes                                                                   │
│  n1 read-spec              ✔ committed   memo HIT    ⟵ spec, ac-ledger                            │
│  n2 write-failing-test     ✔ committed   memo MISS   ⟶ test-red                                   │
│  n3 impl-minimal           ✔ committed   memo MISS   ⟶ impl-report                                │
│  n4 self-heal              ✔ committed   memo HIT                                                 │
│  n7 impl-tdd-cycle         ◐ dispatching                                                          │
│  n8 emit-impl-report       ○ waiting on n7                                                        │
│  n9 checkpoint             ○ waiting on n8                                                        │
├───────────────────────────────────────────────────────────────────────────────────────────────────┤
│ JOURNAL ▸ tail                                                                                    │
│  ·· dag_assembled      7 nodes, 0 unsatisfiable                                        [T1]       │
│  ·· activity_committed n4 self-heal                                                    [T2]       │
│  ·· memo_hit           n4  key 7f31…c0                                                 [T2]       │
│  ·· dispatch           n7  idem 9a04…12                                                [T2]       │
│  ·· rate_limit_reading 5h 0.74 · 7d 0.31                                               [T2]       │
└ [b]oard [g]ates [e]vidence [l]edger [j]ournal [x] explain [↵] answer [q]uit ── run 9f3c…a1 ───────┘
```

| Element | Source | Today? |
| --- | --- | --- |
| Item, lane, pack, rigor | `RunInputs` / `inputs/`, `Item` | yes |
| Stage rail, entered marks | `RunState::stages_entered()`, `stage_transition` | yes |
| `STATE` / `NEXT` / `HALT` | `run.json`'s `terminal_state`, `next_action`; `HaltReason` | yes |
| `LAST` | `Exit::name()` + code | yes |
| `DRIFT` / `REVIEW` / `VERIFY` counters | `Drift`, `ReviewOutcome`, `AcLedger` | yes |
| Dispatch panel, both windows | `DispatchRecord` on `dispatch_completed` | **‹G1›** — ADR-0126, Step 7c Slice 1 |
| DAG strip, memo marks | `specs.json` + `memo_hit`/`memo_miss`/`activity_committed` | yes |
| Journal tail, tier chips | `events.jsonl`, `EventKind::tier()` | yes |

**The stage rail is deliberately not a percentage.** Stages are not uniformly sized, activities within
a stage are selected per item, and a "62% complete" would be a number this system cannot defend. The
rail shows *which stages were entered* and *how many activities of the current stage are committed* —
both of which are facts. This is C8 applied to the most tempting widget in the design.

### 5.2 V2 — **The Board** (kanban). *"Where is everything?"* — ‹blocked on G2›

```
┌ redkiln ─ board ─ initiative: rust-inversion ──────────────── 9 items · 2 gates open · 1 halted ──┐
│  SPEC (2)          │ PLANNING (1)   │ IMPLEMENT (3)      │ REVIEW (1)        │ VERIFY (2)        │
│ ┌────────────────┐ │┌──────────────┐│┌──────────────────┐│┌────────────────┐│┌────────────────┐ │
│ │RK-418  story   │ ││RK-402 story  │││RK-412  story     │││RK-390  story   │││RK-377  story   │ │
│ │dispatch record │ ││facet lints   │││verify gate       │││pack loader     │││trailer grammar │ │
│ │ ⧗ gate 04:12:33│ ││ ▸ dispatching│││ ▸ dispatching    │││ ⛔ 2 blocks     │││ ⛔ 2 unpinned  │ │
│ │ ⚑ HUMAN        │ │└──────────────┘││ $18.77           │││ ◈ 1 quarantined│││ ⚑ HUMAN        │ │
│ └────────────────┘ │                │└──────────────────┘│└────────────────┘│└────────────────┘ │
│ ┌────────────────┐ │                │┌──────────────────┐│                  │┌────────────────┐ │
│ │RK-421  fix     │ │                ││RK-408  fix       ││                  ││RK-365  story   │ │
│ │ ✖ HALT Refused ││                 ││ ⏳ rate-limited   ││                  ││ ✔ sealed        │ │
│ │   NotIsolated  │ │                ││   retry 14:02    ││                  ││                │ │
│ └────────────────┘ │                │└──────────────────┘│                  │└────────────────┘ │
│                                                                                                  │
│ legend  ⚑ gate open   ⧗ deadline   ⛔ blocked   ◈ quarantined   ✖ halt   ⏳ retry   ✔ sealed       │
└ [enter] open item  [g] gate queue  [/] filter  [1-9] jump ──────────────────────────────────────┘
```

Every card badge is a closed-enum render per §4.1. **This view cannot be built truthfully today** —
G2. Building it on the current journal would produce a board whose gates and transitions appear on
every item at once, which is what `status` already warns about in its own source. Two honest interim
forms exist and are worth considering in §9: a board built by **scanning `golden-logs/*/run.json`**
(keyed correctly, one row per recorded run, no live state), or a board that shows only the columns it
can key.

The research offers one addition worth recording and **not** adopting yet: Kanban **classes of
service** (Expedite / Fixed-Date / Standard / Intangible), which
`references/2026-09-09-change-classification.md:102` calls *"the right shape for this engine: named
classes each with a pre-agreed handling rule, rather than one risk score and a threshold … this is
what facets-gate-activities already is."* `[documented]`. A class-of-service swimlane is therefore a
*facet render*, not a new concept — but no facet declares it today, so it is an open item, not a view.

### 5.3 V3 — **The Gate Queue** (the weigh-in inbox). *"What needs me?"*

This is the view that pays for the whole exercise, and it is the one that connects to the
decision-review work already in flight. It is buildable today.

```
┌ redkiln ─ gates ─ 3 open ─────────────────────────────── sorted by deadline ─────────────────────┐
│ ▸ RK-412  human · approve-implementation        ⧗ 04:12:33   reach 1   tree 7f31…c0  FRESH       │
│   RK-390  review · disposition                  ⧗ 1d 06:40   reach 2   tree a091…44  ⚠ DRIFTED   │
│   RK-377  human · unpinned-ac-disposition       ⧗ 2d 11:02   reach 1   tree 5cc2…91  FRESH       │
├───────────────────────────────────────────────────────────────────────────────────────────────────┤
│ RK-412 · approve-implementation                                            token 9a04…12 · single-│
│                                                                            use, tree-bound        │
│ QUESTION   Does the implementation at this tree satisfy the story's acceptance criteria?          │
│                                                                                                   │
│  ▸ approve                admits: —                                                               │
│    changes-requested      admits: findings                                                        │
│                                                                                                   │
│ EXPLAIN (from the record — no recomputation, AC-432)                        [space] expand        │
│   selected because  rigor=high ∧ lane=story ∧ affected(crates/rk-kernel/**)                       │
│   dag               7 nodes · 0 unsatisfiable · 2 memo hits                                       │
│   artifacts written impl-report, test-red, ac-ledger                                              │
│   verify            14 ACs · 11 pinned · 2 unpinned (AC-346 would refuse exit) · 1 not-feasible   │
│   review            2 blocks · 1 quarantined until 2026-09-23                                     │
│                                                                                                   │
│ ATTESTATION   --explain-sufficed  ‹y/n›     --toml-opened  ‹y/n›     ← see §5.3.1                 │
│                                                                                                   │
│ [↵] answer   [t] open item toml   [d] diff   [f] findings   [esc] back                            │
└───────────────────────────────────────────────────────────────────────────────────────────────────┘
```

| Element | Source |
| --- | --- |
| Open gates, deadline, reach, tree hash | `RunState::open_gates()`, `GateRecord`, `RunState::reaches(kind)` |
| `FRESH` / `⚠ DRIFTED` | `rk_kernel::workflow::gate::drifted(record, current_tree_hash)` |
| Question text, options, admissible attachments | `QuestionRegistry::question_for(gate_kind)`, `Question::options()`, `Question::attaches(option)` |
| Explain block | `replay::read_run` — the recorded resolution only |
| The answer | `exec redkiln answer <item> <option> [--attach k=p]… --explain-sufficed … --toml-opened …` |

**This is the only write in the entire design**, and it is an `exec` of the CLI, not a store call — C3.

#### 5.3.1 The attestation flags — a genuine design fork, flagged rather than decided

`answer` requires `--explain-sufficed <yes|no>` and `--toml-opened <yes|no>` (ADR-0129 decision 1;
`cli.rs`). They exist to measure whether the explanation was sufficient for the decision. A TUI
**knows** whether the operator expanded the explain pane and whether they opened the item's TOML. It
could prefill both.

It should probably not. These are an **attestation by the operator**, and a UI that fills them in from
telemetry converts a self-report into an observation — which may be better data or may be a different
measurement entirely, and changing what a measure measures without a record is exactly what the
evidence discipline forbids. **Decision D-V6 in §10.** The middle path — the view *offers* the value it
observed as a default, and the operator confirms or overrides it — is a third option and is not
obviously safe either.

### 5.4 V4 — **Evidence** (the verify + review view). *"Why is it refusing to exit?"*

Buildable today; pure tables; the cheapest high-value view after V3.

```
┌ redkiln ─ RK-412 ─ evidence ──────────────────── 14 ACs · exit permitted for 11 · REFUSED ───────┐
│ AC LEDGER                                                                                         │
│  AC      status            permits exit   evidence (file · test · assertion)                      │
│  AC-340  pinned            ✔              kernel/verify/flip.rs · flip_refuses · asserts_refused  │
│  AC-341  pinned            ✔              kernel/verify/flip.rs · flip_pins    · asserts_pinned   │
│  AC-346  unpinned          ✖              —                                    ⛔ names this exit │
│  AC-349  mutation_survived ✖              cli/tests/it/exit.rs · exit_table · code_eq             │
│  AC-352  not_feasible      ✖              reason: toolchain pack declares mutation infeasible     │
│  AC-357  human_answered    ✔              reason: operator D20-1, 2026-09-20                      │
│                                                       … 8 more · [↑↓] scroll · [u] unpinned only  │
│  ── status distribution ───────────────────────────────────────────────────────────────────────── │
│   pinned            ▰▰▰▰▰▰▰▰▰▰▰ 11                                                                │
│   unpinned          ▰▰ 2                                                                          │
│   mutation_survived ▰ 1                                                                           │
│   not_feasible      ▰ 1                                                                           │
│   human_answered    ▰ 1                                                                           │
├───────────────────────────────────────────────────────────────────────────────────────────────────┤
│ FINDINGS · 2 blocks · 1 quarantined                                                               │
│  ⛔ AC-346  reproduced      the verify gate exits without naming the unpinned ACs                  │
│            repro: cargo test -p rk-kernel -- verify::exit::names_unpinned                         │
│            files: crates/rk-kernel/src/verify/exit.rs                                             │
│  ◈ AC-349  mutation-survived  quarantined by rb until 2026-09-23 (3d) — "pending S-MUT rerun"     │
│  ·  AC-352  advisory        naming style in the refusal message                                   │
└ [enter] open finding  [r] repro to clipboard  [q]uarantine detail  [esc] back ────────────────────┘
```

Sources: `AcLedger` / `AcLedgerRow` / `LedgerStatus::permits_exit()` / `AssertionLocator`;
`Findings` / `Finding` / `EvidenceLabel` / `Repro`; `Quarantine{reason, by, until}`;
`stage_exit_refused` names the ACs (ADR-0125, AC-346).

**Note what this view does that a chart cannot.** The bar block is a *distribution over a closed enum*
— five bars, no axis, no target, no threshold. That is the honest form (§6.4). A "verification health
score" over the same data would violate C8 and would also be meaningless, because
`permits_exit()` is a predicate, not a percentage.

### 5.5 V5 — **The Ledger** (cost & health roll-up). ‹blocked on G1+G2+G3›

Cost per item / stage / activity, cache warm ratio, memo hit rate, failure-class distribution,
retry and rate-limit history, drift counts, replay divergences. All of it is a roll-up across events
that are not keyed and not timed, so **none of it is buildable today** beyond the single-run case.

It is also the view most exposed to C8, and §6.4's rules exist mostly for it.

### 5.6 V0 — **The Journal**. *"Show me the raw thing."*

The tmux-pane replacement, and the one the Gas Town reviewer actually wanted. A filtered, colour-coded
tail of `events.jsonl` / `t2_event`, with tier chips, kind filters, a payload pane, and `/` search.
Buildable today, trivially, and it is the view that makes every other view *falsifiable* — the
operator can always drop to the record and check that the pretty panel was telling the truth.

**Build V0 first, in fact.** A projection whose ground truth is one keystroke away is a projection
people trust.

### 5.7 The ranking, stated plainly

| Rank | View | Value | Buildable today | Notes |
| --- | --- | --- | --- | --- |
| 1 | **V3 Gate queue** | highest — it is the HITL surface, and it is the one write | **yes** | connects to the decision-review work already in flight |
| 2 | **V0 Journal** | high — makes the rest trustworthy | **yes** | near-free |
| 3 | **V4 Evidence** | high — makes the verify gate legible | **yes** | pure tables |
| 4 | **V1 Loop HUD** | high, but its best panel is blocked | partly | dispatch panel needs G1 |
| 5 | **V2 Board** | the one that was asked for first | **no** | G2 — would lie today |
| 6 | **V5 Ledger** | high later | **no** | G1+G2+G3 |

The inversion is worth sitting with: **the board is the least buildable and the least valuable of the
six**, and the gate queue — which the brainstorm mentions only as "HITL" — is the most.

---

## 6. Charts, indicators and the four rules that keep them honest

### 6.1 What a terminal can actually draw

`ratatui` ships `Chart` (scatter/line/bar with axes), `BarChart`, `Sparkline`, `Gauge`, `LineGauge`,
`Table`, `List`, `Tabs`, `Canvas` (braille point/line/rectangle drawing, world-map shapes),
`Scrollbar`, `Block`, `Paragraph` with wrapping, and `Clear` for modals `[documented, docs.rs/ratatui
0.30.2, fetched 2026-09-20]`. The `tui-widgets` family adds big-text, popups, scrollview, prompts and
a calendar (`0.7.11`, MIT OR Apache-2.0); `tachyonfx` adds shader-like transitions (`0.25.2`, MIT);
`tui-logger` adds a filtered log pane (`0.18.3`, MIT). Braille density means a sparkline gets ~2× the
horizontal resolution of the cell grid, and `Canvas` gets 2×4 per cell.

That is enough. **Nothing in the brainstorm needs a chart the terminal cannot draw.** The limits that
bite are semantic, not graphical.

### 6.2 The chart inventory, by view

| Chart | Where | Form | Data | Status |
| --- | --- | --- | --- | --- |
| Stage rail | V1 | segmented bar, no % | `stages_entered` | ✔ |
| Activity completion | V1 | `n of m committed`, integers | `activity_committed` | ✔ |
| Rate-limit headroom | V1 | two `Gauge`s, 5h and 7d | `WindowReading{five_hour, seven_day}` | ‹G1› |
| Cache warm ratio | V1 | one `LineGauge` | `cache_read` ÷ (`cache_read`+`cache_creation`) | ‹G1› |
| AC status distribution | V4 | 5-bar `BarChart`, no axis | `LedgerStatus` | ✔ |
| Finding evidence mix | V4 | 4-bar | `EvidenceLabel` | ✔ |
| Failure-class mix | V5 | 5-bar | `FailureClass` | ‹G1› |
| Memo hit rate | V5 | ratio + `Sparkline` | `memo_hit` / `memo_miss` | ‹G2› |
| Cost by stage | V5 | horizontal `BarChart` | `CostUsd` × stage | ‹G1,G2› |
| Cost over time | V5 | `Chart`, time axis | — | ‹G1,G2,G3› |
| Cycle time / throughput / DORA | V5 | `Chart` | — | ‹G3›, and Phase 3 by the phase table |
| Gate-wait duration | V3 | histogram | — | ‹G3›; and SM-075's input, see §6.4 rule 3 |

### 6.3 The one visualisation that is *not* a chart and is worth more than all of them

**The DAG.** The per-item DAG is a build graph over typed artifacts (`redkiln-process-catalog-proposal`,
PC-series), 102 activities in catalog v0 (`spikes/p9/catalog/*.toml`, counted 2026-09-20), with `provides` / `requires` / `applicable_if` /
`method_rank` / `min_rigor`, backward-chained by the engine. A terminal can render it as an indented
dependency strip (V1), as a column layout, or on `Canvas`. Nothing else in the design explains *why
this item is doing these activities* as directly. It is also the cheapest thing to get wrong, because
selection's answer is recorded and must never be recomputed for display — AC-432, warranted by F36
(*recomputing the modal `R(stage, ctx)` returns the same value on only 5/20, 3/20, 2/20 and 2/20
draws* `[measured]`). **The view renders the recorded DAG. It never re-runs selection. The same
architecture lint that protects `explain` protects it** (`verbs/explain.rs`,
`ac432_explain_from_record_only_and_errors_without_resolution`).

### 6.4 The four honesty rules

1. **No percentage without a denominator the system owns.** "62% through implement" is a lie; "4 of 7
   activities committed" is a fact. Applies to the stage rail, the DAG strip, and anything called
   "progress".
2. **No target line, no red zone, no health score on a tracked-not-targeted metric.** C8, and the
   register names the victims: gate-wait duration is *"the input to SM-075 … **do not set a target on
   it, do not let a dashboard turn it into one**"* `[documented, success-measures.md:566]`. Render it
   as a distribution, never as a dial with a red arc.
3. **A saturated metric is drawn as saturated.** The register's method control (SM-118) demotes a
   metric that hits ceiling or floor to a regression tripwire. A view that keeps drawing a full bar as
   if it were an achievement re-promotes it. Mark it.
4. **A single-draw number carries its `n`.** House style. If a panel shows a figure derived from one
   observation, it shows `n=1` beside it, in the same cell.

Rules 2 and 4 are what make this a redkiln dashboard rather than a generic one, and they are the
reason this is worth designing rather than adopting.

---

## 7. Is `ratatui` the right tool? Six options, priced

### 7.1 What `ratatui` costs, measured

| Dimension | Reading | Source |
| --- | --- | --- |
| Version / date | `0.30.2`, 2026-06-19 | crates.io API, fetched 2026-09-20 `[measured]` |
| Licence | **MIT** | ditto — clears `cargo deny check licenses` on the current `deny.toml` shape |
| MSRV | `1.88.0`; repo pins `1.98.1` | ditto — clears |
| Required deps | `ratatui-core`, `ratatui-widgets`, `instability` — **3** | crates.io deps API `[measured]` |
| Backends | `crossterm` / `termion` / `termwiz` / `termina` all **optional**; `ratatui-crossterm 0.1.2` builds on `crossterm` 0.28 **or** 0.29 (MIT), both optional | ditto |
| Pin style required | **exact** — it is 0.x, so ADR-0100 decision 2 applies: `ratatui = "=0.30.2"` with a bump PR that re-runs `cargo audit` and `cargo deny` | ADR-0100 `[documented]` |
| Maturity | 52.5M downloads; no 1.0; `0.30` restructured backends into separate crates, so the 0.x churn is real | crates.io `[measured]` |
| Async | none required — a sync render loop over a sync read model, which suits `rk-ports`' sync-trait rule (ADR-0076) | `[inferred]` |

**Verdict on the library: yes.** It is the smallest, cleanest new dependency this repo has considered
in months, and it sits in a binary crate where `anyhow` is already permitted. The 0.x churn is the
only real cost and ADR-0100 already has the answer for it.

### 7.2 The six options

| | Option | What it is | Serves the terminal moment | Serves the phone moment | New deps | C4 | Blocked by |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **A** | **Do nothing** | `status` + `explain` + `git log` | poorly — the complaint stands | no | 0 | ✔ | — |
| **B** | **`--format json` on `status` / `explain` / a new read verb** | machine-readable reads; the operator brings `jq`, `fx`, `gum`, or their own script | partly — and it makes every other option cheaper | no | 0 | ✔ | G2 for cross-item |
| **C** | **`redkiln-view`, ratatui, read-only + one `exec`** | this proposal | **best** | no | 3–5 | ✔ | G1/G2/G3 per view |
| **D** | **Local web view** (axum + SSE or htmx) | §8.6's reserved row, Phase 3 | worse — context switch | **yes** | many | **✖ needs a server** | same |
| **E** | **Static HTML report, regenerated by `run`** | a file, opened in a browser, no server | partly — stale between runs | via a shared folder, weakly | 1 templating crate — `minijinja` is **planned** for `rk-store` in design §9.2 but is **not on the tree today** | ✔ | same |
| **F** | **Tracker projection** (GitHub adapter, Phase 3) | issues, labels, comment grammar | no | **yes**, natively | 1 adapter | ✔ | Phase 2 exit |

### 7.3 The recommendation

**B now, C next, D and F unchanged at Phase 3.** In one line each:

- **B is a prerequisite and is nearly free.** A `--format json` read is the *data contract* the view
  needs, it is testable with `trycmd` like every other CLI surface, and it makes options C, D, E and F
  all cheaper. It also does something none of the others do: it lets the operator build their own
  view this week, with no Rust. **It does not violate C1** — a format flag is not a mode; the verb is
  still a read, a decision and an exit.
- **C is the right terminal answer, and the argument that it is not is weaker than it looks.** The
  landscape evidence against dashboards is about *products whose UI was the product* and about one
  dashboard over a process supervisor (§2). Redkiln's view would be over a durable log, in the
  operator's own terminal, with no daemon — which is the shape C4 forces and the shape the dead tools
  did not have. The honest residual is that this is `[inferred]` and that `gt dashboard`'s `n=1` is
  the only directly comparable data point either way.
- **D stays where the design put it**, because the phone case is real and recorded (Phase 3's exit
  criterion 1: *"A gate answered from a phone produces the identical `(token, tree_hash, deadline)`
  mark"*) and a terminal cannot serve it. C does not replace D; if C's read model is designed
  properly, C **cheapens** D, because D becomes a second renderer over the same contract.
- **E is the dark-horse.** It costs almost nothing — one templating crate, and `minijinja` is
  already *planned* for `rk-store` in design §9.2 (though not yet on the tree) — it needs no server, it survives the process dying, and it can be committed as an
  artifact. It is strictly worse than C for the live moment and strictly worse than D for the phone,
  but it is the cheapest thing that could possibly answer *"what happened while I was away."*
  It is not recommended over C; it is recommended as C's fallback if the spike in §9 fails.

---

## 8. Architecture, if C is taken

### 8.1 Shape

```
  redkiln-view  (a binary crate — NOT a redkiln verb: C1)
     │
     ├── reads ──▸ rk-kernel::workflow::fsm       fold events → RunState / StageAction /
     │                                            TerminalState / NextAction     (never re-derived)
     ├── reads ──▸ rk-kernel::replay::read_run    golden-logs/<run_id>/          (AC-432's rule)
     ├── reads ──▸ rk-ports::EventStore (read)    via the sqlite adapter, read-only connection
     ├── reads ──▸ rk-store (typed artifacts)     AcLedger, Findings, ReviewOutcome
     ├── reads ──▸ rk-git (read path)             trailers, tree hash for gate freshness
     │
     └── writes ─▸ std::process::Command("redkiln").arg("answer")…       ← THE ONLY WRITE (C3)
```

### 8.2 Five properties it holds by construction

1. **It is disposable.** It holds no state and no cache. `SIGKILL` costs nothing. C4.
2. **It cannot transition anything.** Its only mutation is an `exec` of a verb that reconciles first.
   C2, C3.
3. **It cannot compute state.** Every state value is one the kernel returned. §4.1.
4. **It cannot re-run selection.** It reads the recorded resolution. AC-432, F36.
5. **A new enum variant breaks the build, not the display.** §4.1.

### 8.3 The three lints that make those properties checkable rather than aspirational

| Lint | Shape | Precedent on the tree |
| --- | --- | --- |
| **No selection from the view** | the crate names no `rk_kernel::select` path at any depth; source scan, self-controlled against a crate that legitimately does | `ac432_explain_from_record_only_and_errors_without_resolution` |
| **One write, and it is an `exec`** | the crate's dependency graph contains no `rk-store` write path and no `rk-git` write path; one `Command::new` site, asserted by count | AC-405's single-call-site assertion; AC-484's `cargo metadata` graph assertion |
| **Exhaustive badge rendering** | every badge function is one `match` with no wildcard over a `rk-types` enum | `Exit::name()`, `EventKind::ALL`, `ac438_step_error_mapping_is_exhaustive` |

All three have a working model already in this repo. None needs a new instrument category.

### 8.4 The one thing that must be got right first

**The data contract, not the widgets.** Concretely, the view needs:

| Needs | Which is | Status |
| --- | --- | --- |
| `Event` carries `(item, run_id, at)` | the correlation triple | **G2 + G3** — not designed |
| `payload` is typed per kind | the record already marked **OWED**, earliest consumer the SQLite adapter at 7c | **G1** — owed |
| `Query::{ByItem, BySha, ByRun}` are honoured | the adapter's `WHERE` | **G2** — the port declares them; the adapter ignores them |
| An item index across runs | a scan of `golden-logs/*/run.json`, or a projection table | not designed |
| A stable read-side rendering of `StageAction` | exists | ✔ |

**This is the deliverable to write next**, and writing it as *"what the view needs"* is strictly better
than writing it as *"type the payload table"*, because a renderer that must display every field of
every kind is the forcing function that finds the missing ones. That is the same move the design made
when it let `doctor` force `artifact.write` into existence (*"without which `doctor` cannot tell a
produced-and-never-read artifact from one never produced"*, ADR-0116, `event_kind.rs`).

---

## 9. Phasing, and a spike that can fail

### 9.1 Sequence

| Step | What | Gate |
| --- | --- | --- |
| **0** | This document reviewed; §10's decisions taken | — |
| **1** | **The view data contract** — a reference doc: the correlation triple, the typed payload per kind (folding G1's owed record), the honoured selectors, the item index | it must be writable without a single UI decision; if it is not, it is not a data contract |
| **2** | **Option B** — `--format json` on the read verbs, pinned by `trycmd` snapshots like every other CLI surface | the JSON is the contract from step 1, or it is a second contract and the step failed |
| **3** | **S-VIEW** — the spike below | its kill criteria |
| **4** | `redkiln-view` V0 + V3 + V4 (journal, gates, evidence) — the three buildable today | the three lints in §8.3 pass |
| **5** | V1's dispatch panel, once `dispatch_completed` lands at Step 7c Slice 1 | — |
| **6** | V2 board and V5 ledger, once the correlation triple lands | — |
| **7** | Phase 3's web view and tracker, as a **second renderer over the same contract** | unchanged from the design |

### 9.2 S-VIEW — pre-registered

Because this repo pre-registers, here is the spike written before it runs, with what would kill it.

**Question.** Does a read-only terminal projection over the recorded run reduce the operator's time
to answer *"what is it doing, and what does it need from me"* relative to `status` + `explain` +
`git log`, and does the operator keep using it?

**Arms.** (a) today's CLI; (b) option B's JSON plus the operator's own one-liner; (c) a `ratatui`
prototype rendering V0 + V3 + V4 against a **recorded** corpus of runs — no live loop, no new store
work.

**Pre-registered measures.** Time-to-first-correct-answer on a fixed set of questions over a fixed
corpus; number of drops to the raw record per question; and — the one that actually matters —
**retention: is arm (c) still being opened 14 days after the spike ends, unprompted?**

**Kill criteria, declared now.**
- **K1.** If arm (b) reaches the same answers within the noise of arm (c), **stop at B.** The JSON is
  the win and the TUI is decoration.
- **K2.** If the prototype requires computing any state the kernel does not return, **stop** — the
  design is wrong, not the implementation.
- **K3.** If retention at 14 days is zero, **stop and write it down**, because that is exactly what
  `gt dashboard` did and the second data point is worth more than the tool.
- **K4.** If the prototype cannot be built without a store write or a background process, **stop** —
  it has violated C2 or C4 and the shape is wrong.

**What it may not do.** It may not touch `crates/`, may not add a workspace member outside
`spikes/`, may not write to the store, and its corpus is recorded runs only. `n` is one operator and
one machine, and every result it produces carries that.

**Honest note on the design of this spike:** K3's retention measure over `n=1` operator is the
weakest instrument in it, and a 14-day retention reading from a solo founder who wrote the tool is
close to unfalsifiable. It is kept because it is the *right* question, and it is labelled
`[measured, n=1, self-reported, confounded]` rather than dropped.

---

## 10. Decisions this document asks for

| # | Decision | Options | This document's lean |
| --- | --- | --- | --- |
| **D-V1** | Is a terminal projection in scope at all, ahead of Phase 3's web view? | yes / no / after Phase 1 exit | **yes, after the data contract** |
| **D-V2** | Separate binary, or a verb? | binary / verb | **binary** — C1 leaves no room |
| **D-V3** | Is `--format json` (option B) taken now, independently of the TUI? | yes / no | **yes** — it is the contract, and it is cheap |
| **D-V4** | Does the view write the correlation triple `(item, run_id, at)` into the event record, or does it live with a run-record scan? | triple / scan / both | **triple**, because G3 blocks every chart and nothing else will unblock it |
| **D-V5** | Does the view's need drive the typed payload record (G1), or does 7c's SQLite adapter drive it alone? | view-led / adapter-led | **view-led**, on ADR-0116's `artifact.write` precedent |
| **D-V6** | The attestation flags: prefill from observed behaviour, offer-as-default, or always ask? | prefill / default / ask | **ask** — §5.3.1; a prefill silently changes what SM measures |
| **D-V7** | Board before or after the correlation triple? | before (lying) / after | **after** — a board that over-reports gates is worse than none |
| **D-V8** | `ratatui` pin style and bump policy | exact + bump PR / caret | **exact**, `= "=0.30.2"`, ADR-0100 decision 2 applies unchanged |
| **D-V9** | Does this need an ADR, and which record? | ADR / design §8.6 amendment / neither yet | **a design §8.6 amendment adding a fifth projection row**, then an ADR only if D-V4/D-V5 change a recorded shape. Next free ADR number is **`0131`** (`references/adr/README.md:73`) |

### 10.1 Open items this document carries

| Id | Item | Why it is open |
| --- | --- | --- |
| OSV-1 | Class-of-service swimlanes | the vocabulary is `[documented]` (`change-classification.md:102`) but no facet declares it |
| OSV-2 | Whether V5's cost roll-up may render anything at all under C8 | the success-measures register forbids targets on several of its inputs; which cost figures are *targeted* is not settled |
| OSV-3 | Multi-worktree / merge-queue visualisation | Phase 2 concern (design §7.7, AC-189–AC-198); a swimlane per worktree is the obvious shape and is unspecified |
| OSV-4 | Whether the view may render a live dispatch at all | `run` takes **one** activity and exits; there is no long-running process to attach to, so "live" means "tail the store while a sibling `run` writes it". Whether that is a supported read pattern under the one-writer SQLite contract (ADR-0105 clause 7, `busy_timeout`, R-SQLITE) is **not settled** |
| OSV-5 | `gt dashboard`'s failure cause | attributed here to process-supervision rather than to terminal UIs in general. `[inferred]` from one reviewer's account. Nobody has checked |

**OSV-4 is the sharpest of these** and deserves a line of its own: every wireframe in §5 that shows a
*live* number assumes the view can read the store while `run` is writing it. WAL admits one writer and
readers per connection `[documented, docs.rs/rusqlite 0.40.2]`, so it is probably fine — but "probably
fine" is not this repo's standard, and R-SQLITE is already registered as unmeasured under
self-hosting load.

---

## 11. Sources

**Repo** (read on `build/phase-0-to-1` @ `1308311`, 2026-09-20)
`crates/rk-types/src/{event.rs, event_kind.rs, usage.rs, review.rs, drift.rs, artifact/ledger.rs, artifact/findings.rs}` ·
`crates/rk-kernel/src/{replay.rs, workflow/fsm.rs, workflow/gate.rs, workflow/question.rs}` ·
`crates/rk-cli/src/{cli.rs, exit.rs, store.rs, verbs/status.rs, verbs/explain.rs}` ·
`crates/rk-adapter-sqlite/src/lib.rs` · `crates/rk-ports/src/event_store.rs` ·
`references/2026-09-08-architecture-design.md` §§3.4, 7.7, 8.1, 8.2, 8.3, 8.6, 9.1–9.3, 10.2 ·
`references/2026-09-08-success-measures.md` :10, :566 ·
`references/2026-09-03-reimagining-brainstorm.md` :14, :128 ·
`references/2026-09-09-change-classification.md` :102 ·
`references/2026-09-08-evidence-augmentation.md` :147 ·
`references/research/2026-09-03-landscape-sentiment.md` :33, :35, :37, :102, :104, :106 ·
`references/adr/README.md` :73 · `spikes/p9/catalog/*.toml`

**External** (fetched 2026-09-20)
crates.io API — `ratatui` 0.30.2 / `crossterm` 0.29.0 / `ratatui-crossterm` 0.1.2 / `tui-widgets`
0.7.11 / `tachyonfx` 0.25.2 / `tui-logger` 0.18.3, with licences, MSRVs and dependency lists ·
docs.rs/ratatui 0.30.2 widget index · github.com/luoyuctl/agenttrace discussion #2 (local-first TUI
over agent session logs)

**Not fetched, and therefore not claimed:** no advisory check was run for `ratatui`, `crossterm` or
any `tui-*` crate. The CI `cargo audit` / `cargo deny` job against a committed lock is what would
establish their status, exactly as ADR-0084 says for every other new dependency. Absence of a finding
here is absence of evidence.

---

## 12. Addendum, 2026-09-21 — the read model is a requirement, and `happenstance` builds it

### 12.0 The ruling, and what it changes above

**Operator, 2026-09-21:** *"because redkiln orchestrates the underlying agent sessions and workflows it
is unrealistic to expect that a redkiln user would be able to effectively develop using redkiln via a
CLI which provides no interactive surface or visual feedback … so the read model is going to be a
MUST, regardless of which technology we choose for the UX/UI."*

Taken as settled. §0's *"do not build the TUI first"* stands only as sequencing advice and no longer as
a question: the read model is a **Phase-1/2 work item**, not a prerequisite awaiting a decision, and
§9.1 step 1 is promoted from "write the contract" to "build it." What remains open is **what builds
it** — which is what this section answers.

### 12.1 What `happenstance` is, measured

| Property | Reading | Source |
| --- | --- | --- |
| Shape | Rust workspace, nine crates: `-core`, `happenstance`, `-testkit`, `-sqlite`, `-postgres`, `-neon`, `-cloudflare`, `-ladybug`, `-sync` | `D:\repos\happenstance`, read 2026-09-21 `[measured]` |
| Version / publication | **`0.3.2`, published to crates.io 2026-09-21** | crates.io API, fetched 2026-09-21 `[measured]` |
| Licence | **MIT OR Apache-2.0** across the set | ditto |
| MSRV | **1.97.1**, against redkiln's pinned `1.98.1` | ditto |
| Contract | The [DCB specification](https://dcb.events/specification/), with `[FROZEN]` clauses per release — `EventStore` since `0.2.0`, `ProjectionStore` from `0.3.0` | `README.md`, `spec/SPECIFICATION.md` `[documented]` |
| Instrumentation | `happenstance-testkit`, **116 rules across four families** (event-store, projection, concurrency, model); five adapters have cleared it | `README.md` status table `[documented]` |
| Mileage | **None.** *"'Passes the suite' is the only claim being made … It is not a claim of production mileage: nothing here has run anywhere but a test."* Downloads at 71–169 per crate | `README.md`; crates.io `[measured]` |

### 12.2 The fit — three of four gaps close outright

| §3.2 gap | The primitive that closes it | Verdict |
| --- | --- | --- |
| **G2** — no correlation key | `Tags` (an ordered `key:value` set on every event) + `Query::from_items([QueryItem::new(types, tags)])`, with OR over types and conjunction over tags | **closed, and more generally than the triple would have been** |
| **G3** — no instant | `SequencedEvent.recorded_at: RecordedAt` — millis since epoch, minted **by the store at append**, and deliberately **not `Ord`** (the derive was removed in `0.2.0` so a caller cannot order by it; `SequencePosition` is the ordering key) | **closed, and closed correctly** |
| read-model machinery | `ProjectionStore` (`begin` / `checkpoint` / `commit` / `reset` / `rollback`), `Checkpoint::{NeverRun, Live{through}, Rebuilding{through}}`, `Authority::{Live, Rebuilding}`, and `run_projection(events, models, projection, codec, chunk) -> Progressed{through, applied}` | **closed** |
| **G1** — untyped payload | `Event.data: Bytes` plus the `happenstance` crate's codec layer and typed domain events | **mechanism yes, field set no** — see §12.5 |

Four observations, each of which is why this is a better answer than the bespoke `(item, run_id, at)`
triple §10's **D-V4** proposed:

**Tags subsume the triple and go further.** `Query::{ByItem, BySha, ByRun}` — the three selectors
`rk-ports` declares and the SQLite adapter ignores — become three tag keys among many. Redkiln's
natural set: `item:RK-412`, `run:9f3c…`, `stage:implement`, `lane:story`, `node:n7`, `gate:9a04…`,
`ac:AC-346`, `sha:…`, `worktree:…`, `initiative:…`. §5's board needs *conjunctions* the three
selectors cannot express — "gates open, stage `implement`, initiative X" — and a `QueryItem` is
exactly that. `[measured]` from `crates/happenstance-core/src/{tag,query}.rs`.

**`RecordedAt` fits C7 without an argument.** The kernel may not read a clock; the **store** mints the
instant at `append`, which is an adapter concern. Redkiln gets durations, cycle times and gate-wait
distributions for free without touching the determinism ban list. And the type refuses to be an
ordering key, which is the error a naïve timestamp invites.

**The replay digest is unaffected — checked, not assumed.** `RecordedRun::canonical_events` hashes
`{ "kind": …, "payload": … }` and nothing else (`crates/rk-kernel/src/replay.rs:1073–1078`), so
store-minted `position`, `id` and `recorded_at` never enter it and no replay diverges because a
second run stamped a different millisecond. `[measured]` — this was the integration risk worth
checking first, and it is already closed by redkiln's own canonicaliser.

**`Checkpoint` and `Authority` are C2 expressed in the type system.** A read model that reports
whether its rows are authoritative is precisely *"derived, never authoritative"* made checkable.
`tickets-over-http` shows the operational form: `GET /seats?at=N` answers **`202` with its own
checkpoint until the runner has caught up, `200` after**. That is the exact contract a lagging HUD
needs, and §5's wireframes currently have no answer for it.

**And `run_projection` is one call, not a daemon.** It reads the checkpoint, streams from
`through.next()`, applies in chunks, commits rows and checkpoint together, and returns how far it got.
It is pumped at entry — the same shape as `reconcile_at_entry` — so **C4 holds unchanged**. The
examples drive it under `#[tokio::main(flavor = "current_thread")]`. `[measured]`,
`crates/happenstance/src/runner.rs:514`.

### 12.3 The one thing it must not become: two authorities

This is the whole decision, and it has a right answer.

| | Shape | What it costs |
| --- | --- | --- |
| **Adopt at tier 2** ✅ | `rk-adapter-happenstance` behind the existing `rk-ports::EventStore`, replacing `rk-adapter-sqlite`. **git stays the tier-1 authority.** | Nothing recorded changes. ADR-0109 decision 2 (five kinds to git first), design §5.6 (`reconcile()` git → store, **never the reverse**), AC-184's rebuild differential and `doctor --rebuild` all stand |
| **Invert** ❌ | happenstance becomes the durable log; git becomes a projection of it | Supersedes ADR-0109 decision 2, ADR-0105 clause 8, design §5.6 and AC-184. The present shape is paid for by **F19** (174/174 reconstructed from git losslessly) and **F22** (nothing may depend on a process surviving). Nothing measured says the inversion is better |

**Say the uncomfortable part plainly: DCB's headline feature is the part redkiln needs least.** The
append condition arbitrates concurrent writers whose decisions overlap. Redkiln has **one store per
repository and one writer behind a mutex** (ADR-0105 clause 7), so at tier 2 there is no contention for
it to resolve. What redkiln needs is the *substrate* DCB happens to require in order to express an
append condition at all — tagged events and a query language over them — plus the projection port
built beside it. Adopting the library for the substrate and declining the mechanism is a coherent
position, not a half-measure, and it should be recorded as one so nobody later "completes" the
adoption by moving the authority.

### 12.4 Integration cost, priced

| Item | Reading |
| --- | --- |
| New normal deps entering the shipped graph | `happenstance-core` (→ `bytes`, `futures-core`, `thiserror`, `trait-variant`), `happenstance` (codecs, runner), `happenstance-sqlite` (→ `rusqlite`, `tokio` with `rt`) |
| Licences | `bytes` MIT; `futures-core`, `trait-variant` MIT OR Apache-2.0; happenstance crates MIT OR Apache-2.0 — **all clear** on the current `deny.toml` shape `[measured, crates.io, 2026-09-21]` |
| `rusqlite` collision | happenstance: `"0.40"`, `default-features = false`, `features = ["bundled"]`. redkiln: `= "=0.40.2"`, `bundled`. **Unifies today**, redkiln's exact pin governing `[measured]` |
| MSRV | 1.97.1 ≤ 1.98.1 — clears |
| Pin style | 0.x, so **ADR-0100 decision 2 applies**: `= "=0.3.2"`, with the bump PR re-running `cargo audit` and `cargo deny` |
| Async | Ports are `async fn` in trait via `trait-variant`, in two flavours (`EventStore` / `SendEventStore`, the `!Send` one existing for `wasm32`). This lands in an **adapter**, which is exactly where **ADR-0076** permits `.await`. S-PORTS asserts over `rk-kernel`'s *normal* deps and **AC-484** forbids a `rk-kernel` → `rk-adapter-*` edge, so both hold unchanged |

**Four risks, named:**

1. **Run `cargo deny check licenses` with `projection-store` turned on.** It is **off by default**
   (`happenstance-sqlite`'s `default = ["event-store"]`), and redkiln needs it. This is the `gix`
   lesson verbatim — default features pulled `resolve-http` → reqwest → rustls/aws-lc into the shipped
   graph and cargo-deny refused six licences, *found only by running it with the real feature set*
   (workspace `Cargo.toml`, the dated 2026-09-18 note). `[documented]`
2. **`block_on` under the store mutex.** Redkiln holds `Arc<Mutex<dyn EventStore>>` and its port
   methods are sync; the adapter must block on an **adapter-owned current-thread** runtime, never a
   shared multi-threaded one. Precedent exists — `rk-adapter-claude` is the same shape — but the
   lock-held-across-`block_on` detail is new and is worth one instrument.
3. **Cost paid for nothing.** `no_std`, `wasm32`, the two trait flavours, `Bytes`, replication identity
   (`EventId`, `StoreId`) — redkiln uses none of it. It is noise in the graph and in the docs, not a
   defect, and it is the price of a general library.
4. **Bus factor, inverted — and the honest version of it.** ADR-0084 flagged `gix` for having one
   crates.io owner as *"a bus-factor note, not a single-author development risk."* The same property
   holds here and the owner is the operator, which is mostly an **asset**: the roadmap is reachable.
   The real cost is different and the README states it — *"nothing here has run anywhere but a test"* —
   so redkiln self-hosting would be happenstance's first mileage, and **during Phase-1 bring-up a
   happenstance defect and a redkiln defect present identically.** The mitigation is unusually strong
   and is the reason this is recommendable at all: **`happenstance-testkit` is published**, so
   redkiln's own CI can run all 116 rules against the exact pin it ships. That converts "my own
   library" from a trust question into an instrument, and it is more than any third-party store offers.

### 12.5 What `happenstance` does *not* do for you

1. **The `EventKind` → `EventType` + `Tags` mapping.** Thirty-eight kinds, each owing a type name and a
   tag set. This is G1's owed design task, unchanged in size — but **better shaped**, because *"which
   tags does this kind carry"* is answerable per row from design §5's table, and *"what are this kind's
   payload fields"* was not.
2. **Which tier-1 kinds gain an item tag.** `stage_transition`, `gate_opened` and `dag_assembled`
   carry no item today. A store cannot supply one; that is a redkiln decision and it is the whole of G2.
3. **The projections.** The board, the gate queue and the cost roll-up are each a `Projection` impl —
   an id, a scope, and an `apply`. The library drives them; it does not write them.
4. **Placement.** The projection store lives under `git rev-parse --git-common-dir`/redkiln/, per
   design §5.7 and **AC-518** (C5). Nowhere else.
5. **One connection contract, or two?** `happenstance-sqlite` carries both roles behind *"one driver
   and one connection story"*, while **ADR-0105 clause 7** gives `rk-store` redkiln's connection
   contract — `busy_timeout`, one writer behind a mutex, readers per thread, WAL, and `doctor`'s
   network-filesystem refusal. These must be reconciled **explicitly**; **R-SQLITE** is already
   registered as unmeasured under self-hosting load and this enlarges it rather than answering it.
6. **OSV-4 gets an answer to test.** "Can the view read while `run` writes" becomes: the projection is
   pumped at entry, and a lagging view renders its own `Checkpoint::Live { through }` instead of
   pretending to be current — the `202`-until-caught-up protocol, applied to a terminal.

### 12.6 S-HS — pre-registered

**Question.** Does `happenstance` at tier 2 produce redkiln's read model at lower cost and higher
fidelity than a bespoke `(item, run_id, at)` column set on `t2_event`?

**Arms.** (a) bespoke triple + honoured selectors on `rk-adapter-sqlite`; (b) `rk-adapter-happenstance`
with tags, one `Projection` — the gate queue — and `run_projection` pumped at entry.

**Pre-registered measures.** Kinds mapped to type+tags before the first query is expressible; queries
§5's six views need that each arm can express; `cargo deny check licenses --all-features` result;
`happenstance-testkit` result against redkiln's pin; whether arm (b) can be pumped with **no**
surviving process.

**Kill criteria.** **K1** — if `cargo deny` refuses a licence under `projection-store`, stop until the
feature graph is cut. **K2** — if any part of happenstance must enter `rk-kernel` or `rk-ports`, stop:
ADR-0076 is not negotiable for a dependency. **K3** — if the projection cannot be pumped inline and
wants a daemon, stop: C4. **K4** — if closing G2 requires moving tier-1 authority off git, stop and
record it as closed (§12.3), because that is a different decision wearing this one's clothes.

### 12.7 Decisions — two amended, four added

| # | Decision | Lean |
| --- | --- | --- |
| **D-V4** *(amended 2026-09-21)* | The correlation key is **tags**, not a fixed `(item, run_id, at)` triple — tags subsume it and express the conjunctions §5's board needs. `recorded_at` arrives with them | **tags** |
| **D-V5** *(amended 2026-09-21)* | The typed payload record stays **owed and view-led**. happenstance supplies the codec mechanism and the wire shape; the 38 field sets are still redkiln's design task | unchanged |
| **D-V10** | Adopt `happenstance` at **tier 2**, as `rk-adapter-happenstance` behind the existing port, replacing `rk-adapter-sqlite`? | **yes** |
| **D-V11** | Does `rk-adapter-sqlite` stay as a second adapter, or is it removed? | **keep** until `happenstance-testkit` runs green in redkiln's CI at the shipped pin, then decide |
| **D-V12** | Does redkiln run the 116-rule conformance suite in its own CI against its pin? | **yes** — it is what converts a self-owned dependency into an instrument |
| **D-V13** | Is the tier-1 authority question (git vs the log) re-opened by this? | **no**, and record it closed so a later pass does not "complete" the adoption by moving it |

### 12.8 The answer in one line

**Yes — for the tag/query model, the store-minted `recorded_at`, and the
`ProjectionStore` / `run_projection` pair. Not for DCB's append condition, which arbitrates a
contention redkiln does not have at tier 2, and not for the log's authority, which stays in git.**

### 12.9 Sources added

`D:\repos\happenstance` @ `55288c5` (`main`), read 2026-09-21 — `README.md`; `Cargo.toml`;
`rust-toolchain.toml`; `crates/happenstance-core/src/{store,projection,tag,query,event,append,identity}.rs`;
`crates/happenstance-core/Cargo.toml`; `crates/happenstance-sqlite/Cargo.toml`;
`crates/happenstance/src/runner.rs`; `examples/rebuilding-read-models`, `examples/tickets-over-http` ·
crates.io API for `happenstance{,-core,-sqlite,-testkit}` 0.3.2, `trait-variant` 0.1.3, `bytes` 1.12.1,
`futures-core` 0.3.34, fetched 2026-09-21 · dcb.events/specification (cited by the README; **not
fetched**, and therefore not independently characterised here).
