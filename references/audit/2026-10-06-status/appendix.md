# Redkiln-RS status-quo audit: technical appendix

Snapshot: commit `36dd8ac` on `build/phase-0-to-1` (2026-10-05 20:21 -0700). Audit date: 2026-10-05/06.
This appendix supports the one-page status board. Where a survey report and an adversarial verdict disagree, the verdict wins. Refuted claims are not repeated. Anything uncertain is labelled UNVERIFIED or "judgement".

---

## 0. Bottom line

- **Build side: largely done.** Redkiln-RS is a 12-crate Rust workspace with 238,046 lines of Rust, about 137k of them tests. It builds, formats, lints and documents cleanly. Its 1,326 passing tests exercise a real kernel, git-first durable state, three gates, four real adapters and a nine-verb CLI.
- **Proof side: barely started.** No real `claude -p` dispatch has ever gone through `redkiln run` and the lane kernel. Both lanes are proven only under a scripted fake model. The adapter is pinned to Claude CLI 2.1.278, while the installed CLI is frozen at 2.1.289. The least-privilege confinement has never been run live.
- **CI is red locally.** CI has run on the remote branch: it went red, and the fixes were landed there. The local branch is 15 commits behind that remote and is known red. No green run is recorded anywhere.
- **Exit status.** 27 of 30 runbook stages have a recorded pass, 5 of them provisional. 8i is in progress, 8j has not started, and Step 8 (the Phase-1 exit, milestone M5) is re-opened and not passed.
- **Phase-1 progress, judgement:** about 65%. Full-vision progress is about 30%.
- **M5 forecast** (calendar days, observed pace, operator-gated):
  - optimistic about 2026-10-18 to 10-20;
  - likely about 2026-10-23;
  - pessimistic early to mid November.
- **Confirmed defects that sit on the paid-run path and are unscheduled:**
  - Ctrl-C is inert.
  - Acceptance criteria can only be closed by a human answer.
  - `status` conflates items.
  - A run cannot start from a human-provided root input; that arm is not yet built.

---

## 1. Method and isolation

| Aspect | What was done |
| --- | --- |
| Source of truth | Pinned worktree `scratchpad/audit-wt` at `36dd8ac`. Read-only git only (log/show/diff/rev-list/ls-files/grep/merge-base). No fetch, checkout or commit. |
| Live repo | `D:/repos/redkiln-rs` was not read: another session is working there. Two later commits exist there (`ebaf4ca`, `524aadd`, per session git status) and are **outside** this audit. The live tree also had 4 uncommitted modified files at session start. |
| Build/test | Run once in an isolated worktree at `36dd8ac`: fmt, build, clippy, test (`--no-fail-fast`), rk-ports conformance, rustdoc. Re-run with `ANTHROPIC_*`/`CLAUDE_*` scrubbed (`audit/build.txt`, `audit/log-*.txt`). No `pnpm install` was run, because it needs network. |
| Survey wave (W1) | 16 area reports: vision, plan status, 10 code areas, pack/CI/tools, records, traceability, velocity, landing (`audit/w1-*.md`, `audit/w1-results.json`). Every report was read for this synthesis. |
| Adversarial verification | 13 headline claims (C1-C13), each verified independently twice. The verdicts are authoritative here (section 6). |
| Completeness follow-ups | 4 gaps, all from git objects only: CI truth, the acceptance census, a bottom-up effort model, and the 8i/8j coupling (section 7). |
| Not done | No `gh` or network calls, so GitHub Actions conclusions were not read. No cargo-mutants. No `--features corpus` runs. No paid dispatches. |
| Labels | RECORDS means what the runbook, ADRs or decisions say. CODE means what the source or tests show. GIT means the object store. |

---

## 2. Vision recap

Sources: `references/2026-08-31-rust-inversion-RFC.md`, `references/2026-09-02-rfc-amendment.md`, `references/2026-09-08-architecture-design.md`.

**The bets.**
- **Bet 1, invert control.** Rust owns the loop and spawns `claude -p` per step (RFC:11-50). The RFC priced it at about 10k LOC across three workstreams (about 500 new, about 9k ported from TS, about 800 for run record and resume; RFC:263-274). It priced Experiment 4, the fix lane end to end, at 1-2 weeks (RFC:323).
- **Bet 2, port the 40k-LOC TS engine.** The RFC called this "materially weaker" (RFC:15). ADR-0002 replaced it with a clean break plus a one-shot migrator (`redkiln-migrate`).
- **Bet 3, keep a plugin surface.** This is borrowed evidence (design principle 19), and nothing ships in Phase 1.

**What the spikes changed.**
- The quality kill criterion did not fire: detection was 23/23 vs 23/23.
- The cost case rests on a 1.72x in-session premium ($97.39 vs $56.47) from one corpus.
- Reviewer reliability is the least-measured property: a CLEAN item flipped 2 times in 3.
- These spikes cost about US$660 API-equivalent (section 10.4).

**Design shape** (design section 0):
- a deterministic kernel (selection, gates, budgets, retry, replay);
- git trailers plus an append-only event log;
- human gates as durable marks: a `(token, tree_hash, deadline)` record plus exit 0;
- typed agent outputs;
- five sync ports (event store, harness, tracker, docs site, toolchain), with git deliberately not a port;
- 19 principles (design:167-185).

**Phase-1 exit checks** (`runbook/phase-1/s8-exit-criterion.md:15-27`). Check (5) was withdrawn.
- **(1) AC-473:** a recorded fix-lane run on this repo, from intake to a landed checkpoint commit, with the Rust toolchain adapter.
- **(1b) AC-521:** a story-lane run that authors a Phase-2 story.
- **(2):** `doctor --rebuild` reproduces the next action from git alone.
- **(3):** resume across a real `taskkill /T /F` at more than one step.
- **(4):** the determinism ban fails the build.
- **(6):** facet lints on both source classes.
- **(7):** a cost row on every dispatch.

**Phase 1 "may not":**
- declare completion on the fix lane alone;
- finalise `ChangeSummary` Binary before S-BIN;
- call the design final.

**Phases 2-4** (design:1868-1870):
- **Phase 2:** the remaining packs, a parallel DAG plus merge queue, .NET/Python/Unreal adapters, `rk-plugin-gen`, and old-corpus migration (AC-463). Entry needs exit (1) recorded.
- **Phase 3:** GitHub/mdBook adapters, a web view and DORA metrics.
- **Phase 4:** Codex (after S-CODEX), Linear and Astro. Entry needs "Phase 3 stable for a quarter".
- **Inconsistency.** ADR-0094 d2 puts Codex in Phase 2; the design puts it in Phase 4.

**Design status.** The design itself says 26 DECIDED, 3 PROVISIONAL and 10 BLOCKED (architecture-design.md:15). "Design final" is forbidden until P8's arms, S-BIN, S-H1c and S-P7B run (architecture-design.md:25).

---

## 3. Stage and milestone table

### 3.1 Stages (runbook/INDEX.md:59-88; runbook/PROGRESS.md:21-51)

Thirty numbered stages; Step 9 (rollback) is "any" and n/a.

| # | Stage | First recorded pass | Date | Standing state at 36dd8ac |
|---|---|---|---|---|
| 1 | 0 | bf5086a | 09-11 | passed |
| 2 | 1 | 66276bf | 09-11 | passed |
| 3 | 3 | 9b2c848 | 09-12 | passed; arm L labels DEFERRED by decision |
| 4 | 4 | b39581a | 09-12 | passed |
| 5 | 4a | db6b55c | 09-12 | passed |
| 6 | 2 | bbdaf35 | 09-12 | passed (M0) |
| 7 | 5 | 96ba9c2 | 09-12 | re-run whole, passed at d24d15a (M1) |
| 8 | 6 | ea83eac | 09-13 | re-run passed at 4777941/d5bfc4f (M2) |
| 9 | 7a | 3c8e880 | 09-13 | re-run passed 492e516/abeb578 |
| 10 | 7b | ddc57be | 09-15 | re-run passed at d24d15a |
| 11 | 7b-ii | 164e212 | 09-17 | re-run passed at d24d15a (M3 stamped) |
| 12 | 7c | 2791f1d | 09-18 | **PROVISIONAL**, re-run pending s8j (needs a paid probe) |
| 13 | 7d | 99e209c | 09-21 | re-run passed at d24d15a |
| 14 | 7e | dddcdbd | 09-21 | passed (M4 condition; M4 not stamped) |
| 15 | 7f | 79e0ef4 | 09-21 | **re-run pending** (s8g .redkiln mark) |
| 16 | 7g | 18eb4ae | 09-22 | passed |
| 17 | 7h | 490c755 | 09-23 | passed |
| 18 | 7i | 6907312 | 09-24 | passed |
| 19 | 8a | bf12a1c | 09-25 | **PROVISIONAL**, re-run pending s8j |
| 20 | 8b | 83d1268 | 09-26 | re-run passed at d24d15a |
| 21 | 8c | 4ab7c67 | 09-26 | **PROVISIONAL**, re-run pending s8j |
| 22 | 8d | b183eb9 | 09-27 | **PROVISIONAL**, re-run pending s8j |
| 23 | 8d-ii | 497266b | 09-28 | passed |
| 24 | 8e | 492e516 | 09-28 | passed |
| 25 | 8f | ae38553 | 09-29 | passed |
| 26 | 8g | 4777941 | 10-01 | passed |
| 27 | 8h | d24d15a | 10-02 | passed; Record A (ADR-0197) still proposed, awaiting s8j probes |
| 28 | 8i (S-BIN) | none | open 10-02 (285b659) | **IN PROGRESS**: B0 (4a3d344) and B1 (ee44bf1, $5.561026) done; B2 and B3 (5 Opus each) need signed go-aheads |
| 29 | 8j | none | planned 09-24 | **Not started** (2,816-line stage file) |
| 30 | 8 (exit) | mechanical block only, d6160ff | 09-21 | **RE-OPENED** 09-24 (D21-40); Checkpoint 8 not passed |

**Stage count (verdict C1, confirmed).** 27 of 30 stages have a recorded pass, and 5 of those (7c, 7f, 8a, 8c, 8d) are provisional. Strictly clean passes are 22. One forward-looking note: `s8i-s-bin.md:29` says 8i's Rust edits will make Checkpoints 8a and 5 provisional once they land.

**Plan growth (verdict C8, confirmed).** The plan grew from 14 rows to 30:
- 14 rows at `0e34384` (09-11);
- 15 at `80a40da` (09-13, 7b-ii);
- 19 at `8634de8` (09-21, 7f-7i);
- 30 at `b8e7aa9` (09-24, eleven s8 sub-stages).

Each insertion followed first contact with a real run (D21-3, D21-40).

### 3.2 Milestones (runbook/INDEX.md:95-100)

| Milestone | Meaning | Status |
|---|---|---|
| M0 Phase 0 closed | Checkpoint 2 | reached by record, 2026-09-12 (bbdaf35); not stamped |
| M1 Foundation builds | Checkpoint 5 | reached by record, 2026-09-12; CI not green (section 9) |
| M2 State is durable | Checkpoint 6 | reached by record, 2026-09-13 |
| M3 Kernel decides | Checkpoint 7b-ii | **reached and stamped** 2026-09-17 |
| M4 System runs | Checkpoint 7e | conditions met 2026-09-21; never stamped; 7c provisional |
| — | Checkpoint 8i | open; forecast about 10-09 to 10-11 |
| — | Checkpoint 8j | not started; forecast about 10-15 to 10-19 |
| M5 Phase 1 exits | Checkpoint 8 | **not reached**; forecast about 10-23 (P50) |

---

## 4. Per-area findings

The grades and percentages below are this synthesis's. They start from the survey grades and are adjusted for the verified defects and the follow-up findings. "% complete" is measured against what Phase-1 exit needs from that area.

### 4.1 Typed core and ports (`rk-types`, `rk-ports`). 93%, grade B

**Implemented (CODE):**
- Closed `StepError` (11 variants) with total `spoke()`/`retry_class()` (`crates/rk-types/src/error.rs:65,119-159`).
- 25 exported schemas from one macro table, each with a golden snapshot. A closed-output schema lint applies union ≤16 and optional ≤24 (`schema/lint.rs:19-20`).
- Typed `Findings`, ledger and usage types with no `Default`.
- Five sync `dyn`-safe ports and five public fakes. One shared conformance body per port runs on real rows: SQLite, Claude (via a fixture child), Rust and Node (`crates/rk-ports/src/conformance.rs`).
- Fake-per-port parity is asserted as a number (`tests/it/parity.rs:24`).

**Gaps:**
- The `ChangeSummary::Binary` arm is sealed until S-BIN (`rk-ports/src/toolchain.rs:153`).
- Tracker and DocsSite are fake-only by design (Phase 3). Their types are unit newtypes, so the Tracker idempotency assertion is vacuous (`conformance.rs`, self-admitted).
- `Digest` deserialises without validation, which bypasses `from_hex` (`rk-types/src/digest.rs`).
- `Drift` distinctness is claimed but not tested.
- `Query::ByRun` returns every row (`fakes.rs:129`; `rk-adapter-sqlite/src/lib.rs:383`).
- S-SCHEMA-CAP was never observed against the vendor.

**Quality:**
- Best-disciplined area: zero unsafe, zero `#[allow]`, 3 infallible serde `.expect`s.
- Comments are 33-42% of source lines, much of it dated provenance. There are stale "proposed" labels for accepted ADRs.

### 4.2 Durable state and git (`rk-store`, `rk-git`, `rk-adapter-sqlite`). 85%, grade B

**Implemented:**
- A single git wrapper with a closed 29-op `GitOp` set (`crates/rk-git/src/git_exec.rs:30-128,484-560`).
- A trailer write path with `--if-exists=add` and hook isolation.
- `reconcile()` with `Reconciled<T>`, constructible only through the sanctioned route (trybuild).
- Tree-hash derivation, landing (`landing.rs:690-791`) and the run-worktree lifecycle (2,883 lines, 57 tests).
- A WAL store with a real two-process contention test (`rk-store/tests/it/contention.rs`).
- Identity and migration refusals; the layout-C render/write/read path.

**Gaps:**
- **No producer exists for orphaned-anchor or rewrite events** (verdict C10, confirmed with corrections). `Drift::OrphanedAnchor` and `EventKind::Rewrite` are declared but never constructed, and `GitOp::PatchId` has no caller. Step 6 claimed the range AC-160–199 (`runbook/phase-1/s6-store-and-git.md:478`) and closed without building AC-167/169/170/171/190. No open stage reschedules them.
- The gix read path is only partly built: `reconcile` spawns `git log --all`, and the common dir comes from `git rev-parse`.
- `GitError` has no `Display`, so operators see stderr as byte arrays.
- `identity::read` swallows probe errors (`identity.rs:67-74`), which can bypass the AC-179 refusal.
- Trailers are parsed anywhere in a commit body, not only in the trailer paragraph.
- Two rk-store doctor helpers are dead because rk-cli re-implemented them more weakly.

**Quality:** strong type-level invariants. Zero real `unsafe` (the census hits are identifiers). Heavy provenance comments. `worktree.rs` is a god module.

### 4.3 Kernel selection (`rk-kernel` select/predicate/registry/lattice). 85%, grade B

**Implemented:**
- A pure, deterministic seven-step `select()` (`select.rs:496`), called in production (`rk-cli/src/verbs/run.rs:6044`).
- The ADR-0103 grammar, differential-tested against 386 probe strings.
- Three fail-closed facet lints wired to `redkiln lint`.
- The rigor ladder.
- A Rust port of the P6 lattice, byte-parity tested against the Python. The shipped reading is `pairs=279936 unsat=1296 dead=57/102 ties=0` (`tests/it/lattice.rs:45-47`).

**Gaps:**
- **The rank-tie load lint omits `rust`** (verdict C9, confirmed with corrections). `step4::rank_lint`, which runs at load via `load_lints`, hard-codes node, unreal, node+unreal and dotnet. The rigor-monotonicity sweep has the same domain but runs only in tests; it is not a load-time check. A real tie still refuses at runtime.
- Step 7 (deviation) has no production caller.
- AC-494/495 pack routing has no production caller.
- Three separate selection models exist (`select`, `lattice`, `rigor`) with no differential between them.
- A wrong-winner bug in step 4 survived 14 days (fixed `bfe0324`).
- Catalog headers are lenient: unknown keys are ignored and `min_rigor` silently defaults.
- `excludes` and `deviation_ok` are missing from `inputs_digest`.

**Quality:** clean typed steps. 417 kernel integration tests pass. No property tests on the parser or evaluator. The crate doc opens with claims that are no longer true.

### 4.4 Gates: human, review and verify (`rk-kernel` review/verify/workflow::gate). 72%, grade B-

**Implemented:**
- Human gate: a content-hash token, answer ordering, and void-on-tree-drift.
- Review gate: a total classifier, a single `Block` construction site, and a tier taken only from authored rigor.
- Verify gate: the flip predicate, ledger, stage-exit refusal and `verify_completed`.
- Twelve source-text architecture lints, each with a positive control.
- Run-path call sites at `rk-cli/src/verbs/run.rs:4228-4235,4655-4700`.

**Gaps:**
- **A criterion can be closed only by a human "discriminates" answer** (verdict C4, confirmed). Two separate causes:
  - `verify_pass::mutate` hard-codes `assertion_flipped: None` (`crates/rk-cli/src/verify_pass.rs:667,707`).
  - `verify_step_six` passes `&BTreeMap::new()` as the assertions map (`run.rs:4228-4234`).

  Each block alone stops a `Pinned` status (`verify/flip.rs:64-68`; `verify/ledger.rs:82-92`). An item with no Spec sidecar exits with no answer at all (`run.rs:4089-4091`). The records admit this deliberately: AC-334 is ARMED (`s8g-verify-and-restored-demands.md:778`).
- The real re-executor reproduces only `kind = run`.
- Quarantine grant/remove, advisory retirement, and AC-349 plan-time binding have no run-path caller.
- **The review gate is uncalibrated against false approval** (S-SUBTLE-CB unrun; ADR-0101 cl.5). Nothing may claim it is guarded.

**Quality:** A- structure (private constructors, pure functions). Three silent `unwrap_or` fallbacks. Lint machinery ships as public modules of the production library.

### 4.5 Runtime and lane execution (`rk-kernel` runtime/replay/tier_one). 72%, grade B-

**Implemented:**
- The five step kinds; `check` cannot dispatch by signature (`runtime/step.rs:1084`).
- The reference resolver (ADR-0173).
- Step-grain commits and re-entry.
- A kill-and-resume proof across a real process boundary (`tests/it/cross_process.rs:1274`).
- A git-first tier-1 writer with heal (`tier_one.rs:855,1932`) and landing.
- Per-item envelope, breaker and outer retry are live.
- Both shipped lanes walk under fakes at activity level: fix 14/14, story 13/13 (`tests/it/lane_walk.rs`).

**Gaps:**
- **No human-root arm (follow-up GAP3).** `read_artifact_input` (`runtime/reference.rs:1128-1160`) resolves only from a committed `emit` binding. The first real activities read `artifact:BugReport` (`minimal-repro.toml:23`) and `artifact:IntakeRecord` (`bdd-scenarios.toml:26`), so each real lane's first activity would refuse by name. `lane_walk.rs:665-666` hides this by seeding fake commits. The work is planned as s8j §8j.2 item 5.
- **Item-level `hold_before` is absent.** `ItemRecord` (`run.rs:150-182`) lacks `deny_unknown_fields`, so a hold written today would be silently ignored.
- Budget exhaustion is library-only: AC-024 mark writer, AC-025 gate, and degradation policy (`workflow/budget.rs:873,1006,1050`; `workflow/degrade.rs`). A live run is bounded only by the dispatch cap, token appetite, breaker and per-dispatch timeout.
- 47 public functions have no non-test caller.
- The token charge fails open on a malformed `usage` row (`workflow/envelope.rs:448-450`).
- **Never run live** (verdict C2, section 6).

**Quality:**
- Good seams that fail closed when unbound.
- 35% of non-blank lines are commentary: 264 dated notes, 64 struck passages and 207 stale "proposed" mentions.
- 33 functions are longer than 100 lines.
- There is one 8-event golden log.

### 4.6 Claude adapter and containment (`rk-adapter-claude`, `rk-containment`). 75%, grade B

**Implemented:**
- One argv builder with golden, proptest and trybuild coverage.
- An env denylist.
- Init adoption checks: full model id, `permissionMode`, and `apiKeySource == "none"`.
- Integer headroom ticks and a throttle.
- Classification onto the 11 `StepError`s.
- `dispatch_completed` written on every path.
- Three-way cancel/timeout/drain race.
- Windows Job-Object containment (25 `unsafe` sites, each with a SAFETY comment).
- 149 and 6 tests passing.
- Live record (RECORDS): Checkpoint 7c on 2026-09-18 passed after five stops; re-run 2026-09-21 at 2.1.278.

**Gaps:**
- **Pin drift** (verdict C7, confirmed).
  - The adapter still has `TESTED_CLI_VERSION = "2.1.278"` (`config.rs:124`), `LIVE_WARRANTED = ["2.1.278"]`, and a dormant 2.1.287 row.
  - The installed CLI is frozen at 2.1.289 (DISABLE_AUTOUPDATER, 2026-10-05).
  - The adapter's last real dispatch was 2026-09-21.
  - s8h probes 0/1/2 and AC-215's `live_disallowed_tool_terminates` have never run.
  - S-BIN's 2.1.289 sessions bypassed the adapter (`live-run.sh wrap`), so they do not validate it.
- **POSIX containment defect, fixed only on the remote** (GAP1). `Group::kill` returns before members die, and `pids`/`is_alive` read zombies as alive (HEAD `posix.rs:55-58,72-92,162`). It is fixed in `9959b2f` on `origin/build/phase-0-to-1` only. A residual remains: `Group::is_empty` (`ffi.rs:652-665`) still counts zombies.
- All stream fixtures are authored; no raw CLI stream is in the tree.
- The throttle is process-local, so the first dispatch of each process is unguarded.
- Per-dispatch settings files accumulate.
- Poison `expect`s at `lib.rs:538,764-791` sit outside the catch, contradicting the AC-360 doc.

**Quality:** strong structure. 36-41% comments.

### 4.7 Toolchain adapters (`rk-adapter-toolchain-rust`, `-node`). 85%, grade B

**Implemented:**
- Rust: real `cargo build/test/nextest/clippy` through `run_bounded`; pinned libtest/nextest parsers with goldens; the `change_summary` Text arm; affected selection; four verbs (`smoke`, `sca_scan`, `package`, `setup`) with an in-tree CVSS v3 scorer differential-tested on 92 vectors.
- Node: `tsc`, vitest and prettier.

**Gaps:**
- Node is thin: no verbs, no affected or named selection, no `change_summary`.
- The Rust half of AC-334 is ARMED.
- Registry and run/env plumbing are copied across four crates.
- The parsers refuse any unpinned toolchain version, so every cargo or nextest bump needs a re-measure.
- On POSIX, the test helper for the nextest-absent PATH needs `/usr/bin` (`cb4ddd4`, remote only; a test fix, not a product fix).

**Quality:** consistent fail-closed errors and zero real escape hatches. 28-49% comments.

### 4.8 CLI and migrator (`rk-cli`, `redkiln-migrate`). 75%, grade B-

**Implemented:**
- Nine verbs with real bodies: `init`, `run`, `answer`, `status`, `explain`, `cost`, `doctor` (43 rules, each with a fixture), `doctor --rebuild`, `catalog search`, `lint`.
- A closed exit table.
- The launch gate through `tools/live-run.sh`.
- Migrator v0, 25/25 on a 24-file hand-authored fixture.

**Gaps:**
- **Ctrl-C is inert** (verdict C5, confirmed).
  - `main.rs:34-37` installs the handler, then drops its token.
  - `run` builds a separate `CancelToken::new()` (`verbs/run.rs:398`). The tokens share no state (`rk-ports/src/cancel.rs:23`).
  - The ctrlc handler also replaces default termination, so only a hard kill stops redkiln.
  - UNVERIFIED: whether a Windows child still receives the console Ctrl-C, since it is spawned without `CREATE_NEW_PROCESS_GROUP` (`ffi.rs:260-277`).
- **`status` conflates items** (verdict C6, confirmed with corrections).
  - `crates/rk-cli/src/verbs/status.rs:179-192` takes one global stage from the last `StageTransition`.
  - `roll_up` (196-218) applies that stage to every item.
  - `item_of` (172-177) reads only `payload["item"]` and ignores the `Event::item()` key added in s8e.
- `verbs/run.rs` is 6,481 lines.
- Doc headers contradict the code ("SKELETON", "RULES is empty").
- Tests are not hermetic: 61 rk-cli tests fail when `ANTHROPIC_BASE_URL` is in the environment.
- `anyhow` and `proptest` are declared but unused.
- `status` has 2 tests; `catalog` has 1 (the promised property test is example-based).

**Quality:** excellent escape-hatch discipline (0 unwrap, 2 expect, 3 deliberate panics, 0 `#[allow]`). God module. 31.6% comments.

### 4.9 Pack and catalog (`.redkiln/`). 70%, grade B

**Implemented:**
- 102 activities and two packs (`bug-standard`, `story`).
- 78 artifact kinds, 21 facets, 13 verb rows, 8 cards, 7 checklists, 11 questions.
- The pack loads; ties are 0 and hard cycles are 0.
- Under fakes, both lanes walk using 17 distinct activities plus `tag-release`.

**Gaps:**
- Cards are one-sentence v0 prompts that have never run against a real model. Record A (ADR-0197) is still proposed.
- The committed items are not the exit items:
  - `rk-story-codex-adapter` refuses with `CyclicDag` (`s8g:766`); its replacement `rk-story-harden-spike-pack` does not exist.
  - The real fix item has not been picked (§8j.5).
- 57 of 102 activities are dead under two packs, a scope choice (D21-47 V3).
- 26 activities reference unmapped verbs.
- Smoke covers only `--version` and `--help`.
- The human-root questions and the `[[gate_expiry]]` rows (§8j.2) are absent.

### 4.10 CI, landing and operations. 35%, grade C

**Implemented:**
- `ci.yml` (539 lines): `gates` on ubuntu and windows; `no-egress` under `docker --network none`; `structural` (8 grep assertions with positive controls).
- `deny.toml`, `clippy.toml`, and must-fail fixture crates.
- `tools/live-run.sh` launch gate.

**Gaps (GAP1):**
- CI ran on GitHub Actions from 2026-10-02/03, per commit bodies `d4da70a..f433854` on the remote. It went red on both legs. The fixes exist only on `origin/build/phase-0-to-1` (`7860e66`).
- HEAD has the exact `crates` and `.github` trees the first runs went red on (`git diff d24d15a 36dd8ac -- crates .github` is empty).
- Local `structural` fails AC-465 on 39 files.
- `gates` lacks the pnpm step.
- **No green result for any sha exists in git.**
- Records still say "never pushed / never run" (`PROGRESS.md:559`; `s8j:1177-1180`; `ci.yml:183,304,530`; ADR-0127:178).
- Landing: `main` holds no code. The branch is 416 commits ahead, with no landing step in the runbook.

---

## 5. Defect and gap register (consolidated)

| # | Item | Evidence | On M5 path? | Scheduled? |
|---|---|---|---|---|
| D1 | No human-root arm, so a real lane's first activity refuses | `runtime/reference.rs:1128-1160`; `lane_walk.rs:665-666` | yes, blocks (1)/(1b) | s8j §8j.2 item 5 (worded as conditional; mandatory in fact) |
| D2 | `pinned` unreachable; every Spec AC needs a human answer plus a relaunch | `verify_pass.rs:667,707`; `run.rs:4228-4234` | yes; operator load | no; recorded ARMED |
| D3 | Ctrl-C inert | `main.rs:34-37`; `run.rs:398` | risk on paid runs (s8 uses `taskkill /T /F`, `s8-exit-criterion.md:113`) | no |
| D4 | `status` conflates items | `status.rs:172-218` | no exit check calls it; operator observability | no |
| D5 | POSIX `Group::kill`/zombie defects | HEAD `rk-containment/src/posix.rs`; fix `9959b2f` remote-only | yes, via the CI condition (D21-42 14a) | not planned in records |
| D6 | Adapter pin 2.1.278 vs CLI 2.1.289; confinement never live | `config.rs:124`; `live.rs:1000,1145,1434` | yes | s8j §8j.6 |
| D7 | Rank-tie sweep omits rust | `select/step4.rs:595-639` | low | no |
| D8 | Orphan/rewrite subsystem absent | `rk-types/src/drift.rs:93`; no producer | no (no rebase in the runs) | no; Step 6 closed without it |
| D9 | `hold_before` absent and would fail open | `run.rs:150-182` | yes (s8j staged first run) | s8j §8j.3 |
| D10 | Budget exhaustion and degradation unwired | `workflow/budget.rs`, `degrade.rs` | no | no |
| D11 | rk-cli tests not hermetic against `ANTHROPIC_*` | `tests/it/main.rs:288` | trust in "green" | no |
| D12 | `Digest` deserialisation bypass; `GitError` without `Display`; `identity::read` swallow | section 4 | low | no |

---

## 6. Verification verdicts (authoritative)

| ID | Claim (short) | Status | Corrected statement used in this report |
|---|---|---|---|
| C1 | 27/30 stages passed; 5 provisional; 8i in progress, 8j not started, Step 8 re-opened; M5 not reached | **Confirmed** | As stated (INDEX.md:59-88; PROGRESS.md:21-51). |
| C2 | No real `claude -p` dispatch has ever gone through the rk-kernel/rk-cli lane | **Confirmed with corrections** | True: no real dispatch through `redkiln run`. Three corrections. (1) A real-binary `redkiln run --lane fix` in s7h (`59be987`) crossed triage → analysis and minted the first real `Story:` trailer, but dispatched nothing. (2) No story lane has been walked through `redkiln run` even with fakes: `lane_e2e.rs` walks only `pack-min`'s fixture lane; the shipped lanes are walked at kernel level in `lane_walk.rs`. (3) Real `claude -p` calls happened outside the lane: the 7c probes through the adapter (`2791f1d`) and S-BIN B1 through `wrap` (`ee44bf1`). |
| C3 | The remote ref has about 15 commits HEAD lacks, including CI and rk-containment fixes, while records say CI never ran | **Confirmed** | `rev-list --left-right --count HEAD...origin/build/phase-0-to-1` = 40 15; merge-base `d24d15a`. The branch **was pushed** (reflog "update by push", 2026-10-02). The rival claim "never pushed" is wrong. |
| C4 | On the real path, ACs close only by a human "discriminates" answer | **Confirmed** | As stated. Caveat: no Spec sidecar means an empty ledger and an exit with no answer. It is a deliberate, recorded build choice. |
| C5 | The Ctrl-C token is orphaned, so interrupt is inert | **Confirmed** | As stated. The child's own behaviour on Windows Ctrl-C is UNVERIFIED. |
| C6 | `status` derives one global stage and applies it to every item | **Confirmed with corrections** | Path is `crates/rk-cli/src/verbs/status.rs:179-218`. Events do carry an interim `item` key since s8e (`Event::item()`), but `status` ignores it. Item-less rows (transitions, gates) fold into every item. |
| C7 | Adapter pinned to 2.1.278 (last dispatch 09-21); CLI is 2.1.289; s8h probes and AC-215 live test never run | **Confirmed** | As stated. The 2.1.289 sessions on 10-05/06 were S-BIN's, not the adapter's. A dormant 2.1.287 row exists. |
| C8 | Plan grew 14 → 30; about 70% docs-only commits last week; s8i added zero Rust | **Confirmed** | 68 to 69 of 98 commits (69-70%) in the 7 days to HEAD. "Zero Rust" holds for this repo only; s8i wrote Rust in private corpus copies (`5736b64`). |
| C9 | Rank-tie and rigor sweeps hard-code a toolchain domain without rust | **Confirmed with corrections** | Only the rank-tie lint runs at load. The rigor-monotonicity sweep is test-only. `python` is also omitted. `lattice.rs` adds rust. |
| C10 | Orphan/rewrite subsystem has no producer and no stage before exit | **Confirmed with corrections** | No producer. Step 6 formally claimed AC-160–199 and closed without building these. No open stage reschedules them. |
| C11 | About 73% of about 475 Phase-1 ACs are evidenced; about 87 never cited; nothing reconciles | **Confirmed with corrections** | 526 in force. The uncited count is method-dependent (about 80-105). 73% is auditor judgement; the resample gives 65% strict (section 8). Reconciliation is partial and per stage, not register-wide or standing. |
| C12 | Paid spend about US$660; S-BIN $5.56 of a $90 cap | **Confirmed** | The total is a computed sum and a floor: E0's Opus repeats have no recorded cost. API-equivalent, on a subscription. |
| C13 | RFC priced Bet 1 at about 10k LOC and Exp-4 at 1-2 weeks; the tree is about 238k Rust plus about 130k md after 5 weeks | **Confirmed with corrections** | The RFC gives no single total; about 10.3k includes a 9k port. About 137k of the Rust lines are tests. The tree includes engine work that Bet 1 assumed TS would cover (D2 clean break). |

**Survey statements superseded by the verdicts and follow-ups:**
- "The branch has never been pushed" (w1-plan-status §1) is **wrong**.
- "ci.yml has never executed; POSIX arms never compiled" (w1-pack-ci-tools §4.2, w1-code-claude-adapter §3) is **wrong**. It ran and was red, and the POSIX defect was found by CI.
- "cb4ddd4 is a product fix" (the GAP1 brief) is **wrong**. It is a test-helper fix; `9959b2f` is the only product change among the remote fixes.

---

## 7. Completeness follow-ups (summary)

**GAP1, CI truth.** The remote ran CI from 2026-10-02/03 and was red on every recorded run. Remote fixes, all classified:
- **Product:** `9959b2f` (rk-containment POSIX).
- **Test/fixture:** `cb4ddd4`, `aab766c`, `f433854`, and part of `d4da70a` and `ef747da`.
- **CI config:** `d4da70a` and `ef747da` (pnpm step, AC-465 allowlist (e)-(h), Windows `RUNNER_TEMP`).

Under remote `ci.yml` the AC-465 filter clears all 39 local files (0 remaining). No green result is recorded for any sha. The Windows result after `ef747da` is unknown, as are `no-egress`, `temper`, `audit` and `deny`.

Framing: **cross-OS known defective locally, fixed remotely, remote green not established.**

The remote also adds a `temper` lint ratchet (`rust.yml`; 6,606 findings accepted as baseline debt, "H-12"). It also adds 59 warn-level lints in `Cargo.toml`. No runbook record mentions any of these 15 commits.

**GAP2, acceptance census.**
- In-scope Phase-1 universe: 493 ids, made up of 455 binding, 19 provisional and 19 human-gated.

| Disposition of the 455 binding ids | Count |
|---|---|
| Discharged or claimed, no live residual | 373 (some are range-swept "claimed, not individually pinned") |
| ARMED/OWED, scheduled before M5 | 15 (10 CI-job ACs; 473 and 521; 215 and 224 by mechanism; 265) |
| Waived or narrowed by decision | 17 |
| **ARMED/OWED, unscheduled** | **50** |

- **The 50 unscheduled, by group:**
  - 28 owed register amendments or owner records;
  - 6 corpus replay;
  - 4 live/spike;
  - 3 POSIX/CI;
  - 9 unwired code or fixtures.
- **Provisional ids:** 14 have no scheduled spike.
- **What M5 checks:** Checkpoint 8 names only 28 register ids, or 47 with §8j.9-8j.10. `s8-exit-criterion.md` has no acceptance set.
- **M5 will not certify register-level discharge.**

**GAP3, bottom-up effort.**
- s8j is more than run preparation. It specifies new product code (human-root arm, `hold_before`, `lane_proof.rs` with 10 proofs, a fixture crate, a mutant driver), a pin move across 62 files, and about 740 lines of instrument amendments.
- Recomputed s8j: 7.5 d central (5-11), against velocity's 2-4 d.
- Operator actions to M5: 11-13 sittings, about 14-20 paid launch go-aheads, 13-20 gate answers, 8-12 push/probe/signature acts, and 20 rulings.
- No recovery procedure is documented for a failed first real run. Each repair means a full re-freeze.

**GAP4, 8i/8j coupling.**
- The 8i → 8j entry gate is a positional sequencing choice that no record justifies. ADR-0093 d4 and ADR-0165 say S-BIN does not block Phase-1 work.
- 8j consumes one S-BIN output, the pinned `cli_version`, and that value is already known (2.1.289).
- 8j §8j.1-8j.5, the CI reconcile and §8j.8 can overlap B2/B3.
- The hard ordering constraint: 8i's Rust package (`rk-ports`, `rk-store`, `rk-cli`, possibly `rk-kernel`) must land before §8j.6 step 1b's proof re-recording.
- Saving: about 1-2 d by GAP4's own estimate. It could be larger given GAP3's bigger s8j, but the single operator caps it (judgement).

---

## 8. Build and test

| Step | Result |
|---|---|
| `cargo fmt --all --check` | rc 0 |
| `cargo build --workspace --all-targets` | rc 0 (90 s) |
| `cargo clippy --workspace --all-targets` | rc 0, 315 warnings, mostly nursery/pedantic doc-length (67 of 78 distinct rk-kernel warnings are `too_long_first_doc_paragraph`) |
| `cargo test --workspace --no-fail-fast` | rc 101: **1,326 passed / 78 failed / 9 ignored** |
| rk-ports conformance feature | rc 0 |
| `cargo doc --workspace --no-deps` | rc 0 |

**Failure triage (`audit/build.txt`, `audit/log-test-cleanenv.txt`):**
- 61 failures: the audit shell inherited `ANTHROPIC_BASE_URL`/`CLAUDE_*`, which the product deliberately refuses (ADR-0010).
- 14 failures in rk-adapter-toolchain-node, plus 2 more node-fixture failures: the fixture `node_modules` was not installed (`pnpm install`).
- 1 failure in rk-cli: it needed the same node fixture.
- Re-run with the environment scrubbed: rk-cli 328/1, toolchain-rust 71/1. Both remaining failures are node-fixture preflights.
- **Net: no product-logic failure on Windows.**
- The 9 ignored tests are paid live probes and corpus-bound tests.

**Caveats:**
- Windows only. Linux is known red at this tree (GAP1).
- The rk-cli suite is not hermetic against provider environment variables (D11).
- `--features corpus` and the ignored live tests were not run.
- The recorded whole-suite figure at `d24d15a` was 1,404 passed / 0 failed / 9 ignored (PROGRESS row 8h). The kernel and product crates have not changed since (`git log d24d15a..HEAD -- crates` is empty).

**Escape-hatch census, qualified by the area auditors (`audit/escape-hatches.txt`):**
- Real production `unwrap()`: 0 across the workspace.
- Production `expect`: mostly the deliberate ADR-0118 poisoned-lock halts (about 40 sites), plus a handful of invariant serde expects.
- Real `unsafe`: only in `rk-containment/src/ffi.rs`, 25 sites each with a SAFETY comment.
- `#[allow]`: one (`ffi.rs:61`). Elsewhere `#[expect(..., reason)]` is used.
- `todo!`: 0.
- The workspace does **not** deny `unwrap_used`/`expect_used`/`panic`; the discipline is convention.

**Size:**
- 584 `.rs` files, 238,046 lines, of which about 137k are under `tests/`.
- 434 `.md` files, 130,759 lines.
- About 1,450 `#[test]` functions, plus about 25 trybuild compile-fail cases.

---

## 9. Traceability

- **AC register** (`references/2026-09-08-acceptance-criteria.md:41-43`): 526 in force, made up of 480 binding, 20 provisional, 20 human-gated and 6 conditional.
- **SM register:** 135 measures. 58 have no baseline and 48 cannot discriminate (`success-measures.md:1422`). 14 SM ids appear in code. "Success" is not yet measurable, which is expected before production.
- **Evidence mechanisms:**
  - 343 `fn acNNN_*` tests covering about 234 ids;
  - behaviour-named tests with the id in comments;
  - trybuild cases;
  - per-stage checkpoint scripts that assert named tests.

  There is **no register-wide reconciliation tool, CI step or standing report**. SM-100 (per-AC pinning coverage) awaits first product measurement.
- **Evidence rate** (GAP2 resample, n=55 pooled, seed 20261006):
  - strict FULL: 37/55 = 67% (95% CI 54-78%);
  - stratified: 65% (54-77%);
  - partials at half: 77% (68-87%).

  That implies about 320 of 474 machine-testable ids (range 256-371). Seven of the 55 (13%) had a gap that no ARMED/OWED line admits: AC-084, 116, 122, 168, 363, 446, 471. That extrapolates to about 30-110 silent gaps among the 455 binding ids. The earlier "about 73%" estimate (n=15) sits inside this interval but is not confirmed.
- **Weakest area by id:** packs/catalog, AC-080-118. AC-084 (multi-pack slot filling) looks unbuilt.
- **Orphaned ids:** AC-291 and AC-292 are orphaned. AC-043/044/045/056 register amendments are owed since 7b (`s7b-kernel.md:2035-2039`).
- **Implication:** M5 will certify that the six exit checks pass and the 8j chain re-runs. It will not certify that the Phase-1 register is discharged. This should be stated in the M5 record (decision O-5, section 13).

---

## 10. Records and process health

### 10.1 Records
- **ADRs:** 198 records: 193 accepted, 1 proposed (ADR-0197, Record A, cards/models/run environment), 4 superseded. 61 records carry a partial-supersede edge.
- **ERRATA.md:** 51 entries. Category 4 shows Phase-1 implementation ADRs needing correction once the code met them.
- **Decisions:**
  - 184 id-labelled decisions plus about 100 earlier numbered ones.
  - `spikes/DECISIONS-2026-09-21.md` alone is 6,865 lines and holds D21-1 to D21-151.
  - D21-141 to D21-151 are all S-BIN instrument hygiene.
- **Provisional AFK rulings without recorded operator confirmation:** 20 (D21-106 to D21-128 except 107 and 116-118, plus D21-137). Several ADR flips rest on "operator has not objected".
- **Stale tracker rows** say open for items that are closed: L2, L3, D17-4, D21-143, O-12 to O-16. The 8i row lags git.
- **Stale CI and push statements** in records and source: `PROGRESS.md:28,559,621`; `s8j:1177-1180`; `ci.yml:183,304,530`; ADR-0127:178; `rk-containment/src/lib.rs:38`.
- **Audit residue never dispositioned:** M-05/14, M-08, M-03, M-04, M-07, M-09, M-10, C-02, C-03, C-07, C-08/M-13. All are marked "not run" (PROGRESS §5).

### 10.2 Velocity (git, 517 commits 2026-09-01 to 10-05)

| Week | Commits | Rust added | md-only commits | Stages closed |
|---|---|---|---|---|
| W1 09-01..07 | 76 | 7.5k | 33% | 0 (spikes) |
| W2 09-08..14 | 75 | 59.1k | 52% | 10 |
| W3 09-15..21 | 133 | 71.7k | 42% | 6 |
| W4 09-22..28 | 141 | 81.6k | 51% | 9 |
| W5 09-29..10-05 | 92 | 37.6k | 70% | 3 |

- Close-to-close time for stages 7g to 8h: median 17.1 h, mean 21.5 h, maximum 62.5 h (8g).
- D21 rulings by week: 8, then 101, then 42.
- s8i has been open 74 h or more and has added 0 lines of Rust to this repo.

### 10.3 Process judgement
The programme is unusually honest about itself. It records its own false passes (D21-40 un-passed Checkpoint 8), keeps positive controls, and runs mutation-style refuting rounds. Those rounds find real defects: 36 defects in 7i's green work (`5853aa8`), and the step-4 wrong-winner bug.

The cost is weight:
- a 50k-line runbook and 32k lines of s8 stage files;
- 30-50% of source lines are dated provenance;
- checkpoints assert by grepping prose, which makes the prose load-bearing;
- every touch to a crate triggers whole-checkpoint re-runs.

Scope grew 114% in 24 days. The growth was triggered by reality, not by planning, which is the right reason, but nothing caps it. The operator is now the constraint: paid steps cannot run AFK (D21-105). The records also show **drift**: the remote CI campaign of 10-03 never entered any record.

### 10.4 Spend
- **Total:** about US$660 API-equivalent (`total_cost_usd`, on a subscription). This is a computed floor. Per campaign:
  - E3 $153.86, E3-H $36.18 and H1/H4 $18.85;
  - S-VAR $176.96 and S-TYPED $112.25;
  - P1-L $21.37, P7 $131.62 and P8 $3.01;
  - small probes, about $0.15;
  - S-BIN $5.561026.
- **S-BIN:** $5.56 spent of a $90 cap (raised from $30 by D21-140). The 8i spend table warns that B3, retries and B4 "may not fit" at their planning bounds.

---

## 11. Landing

- `main` (`353ea01`, 09-11) holds no code; `Cargo.toml` is absent there. The branch is 416 commits ahead, with 1,890 files changed, +363,228/-37,874. It fast-forwards with no conflicts and contains no secrets. The tracked tree shrinks by about 13 MB because raw spike data moved to `redkiln-corpus` at `CORPUS_PIN`.
- The runbook has **no landing step**. The only precedent is a `--no-ff` merge (`66d0ced`). PROGRESS cites 189 or more branch-unique hashes, so a squash or rebase would orphan the evidence chain.
- **The real risk is divergence from the branch's own remote:**
  - The remote has 15 commits (64 files, +8,709/-19); local has 40 docs-only commits.
  - Zero file overlap, so the merge is textually clean.
  - Product code changed on the remote after CP8h passed at `d24d15a`, so the checkpoint chain does not cover the integrated tree.
- The 40 local-only commits and the corpus repo on `D:` are single-machine risk. Whether the corpus repo has a remote is UNVERIFIED. The S-BIN evidence lives in the corpus repo.
- Recommended order:
  1. merge the remote now (one session);
  2. re-run the checkpoints and get CI green;
  3. land once at M5 with `--no-ff`;
  4. untrack the four stale `spikes/*.log` files.

---

## 12. Remaining-effort model and forecast

### 12.1 Remaining work (calendar time at observed pace)

| Item | Effort | Waits on |
|---|---|---|
| 8i: D21-149 driver change, zero-spend dry run, re-signed B1; B2 and B3 (10 Opus sessions in the UE editor); audit, finding, `Binary` record and 8i Rust package; Checkpoint 8i | 3-5 d (central 4) | operator: signed go-ahead per batch; attended UE capture sessions; $84 headroom |
| Merge remote (15 commits), CI shake-out to green on both legs, record run ids | 1-2 d | operator: per-push go-ahead (D21-42 14a); single-session coordination |
| §8j.1 items: fixture item, the real story item, worktree move | 0.5 d | agent; operator runs `git worktree move` |
| §8j.2 human roots: questions, `bind`, `[[gate_expiry]]`, resolver/reader root arm, partial supersede of ADR-0173 | 1-2 d | agent; operator signs three gate-class rows and the record |
| §8j.3 `hold_before` (with `deny_unknown_fields`) | 0.5-1 d | agent |
| §8j.4 `lane_proof.rs` (10 proofs), fixture crate, per-dispatch fixture child, 10-mutant driver | 1.5-3 d | agent; high first-contact risk |
| §8j.5 fix-item shortlist and pick | 0.25-0.5 d | operator pick |
| §8j.6 pin move to 2.1.289 (62 files), `--help` capture, 7c split re-run, probes 0/1/2 | 1-1.5 d | operator: 4 paid go-aheads from a plain terminal |
| §8j.8 s8 instrument amendments A0-A12 (about 740 lines); Checkpoint 8j block plus dry run plus 20-checkpoint chain | 2-3 d | operator runs the chain (a working day or more) |
| Step 8: fix-lane and story-lane real runs, kill/resume, replay, cost; Checkpoint 8 | 2.5-4 d clean | operator at every launch (about 16 or more launches, 13-20 answers) |
| Ratify 20 provisional rulings; sign Record A | 0.5 d | operator |
| Recommended additions: wire the Ctrl-C token, fix `status` item keying, add rust to rank_lint, scrub env in tests | 0.5-1 d | agent (not yet in any stage) |

### 12.2 Velocity-based forecast for M5 (from 2026-10-06)

**Model.** Calendar days. Legs: 8i remainder 4 d (3-5), s8j 7.5 d (5-11, GAP3), Step 8 3 d (2.5-4). That gives 14.5 d serial and clean.

**Assumptions:**
1. The W5 pace holds, about 13 commits per day. AFK throughput does not return for the paid steps (D21-105).
2. About one operator sitting per day, with 11-13 sittings needed. This puts a floor of about 11-13 days under any scenario.
3. No new stage is inserted except in the "insertion" scenario.
4. The CLI stays frozen at 2.1.289.
5. CI turns green within 1-2 push cycles after the remote merge.
6. Overlapping 8j's build half with 8i, if the operator approves, saves about 2 d (1-3).
7. Weekend work continues, as in the commit history.

**Scenarios.** Weights are carried from w1-velocity and are judgement.

| Scenario | Weight | Days | Date |
|---|---|---|---|
| Clean, overlapped | 30% | 12-14.5 | 2026-10-18 to 10-20 |
| One repair and re-freeze cycle (+2-4 d) | 45% | 16-18.5 | 2026-10-22 to 10-24 |
| Structural insertion after first real contact (+5-20 d; precedents D21-3 at about 3 d, D21-40 at about 11 d) | 25% | 20-35 | 2026-10-26 to 11-10 |

**Result:**
- P50 about **2026-10-23**.
- 80% band 10-19 to 11-03.
- P90 early to mid November (11-03 to 11-10).
- If s8j runs operator-attended rather than at W4 AFK pace, add 3-5 d.

**Comparison with earlier estimates.**
- w1-velocity: P50 10-21, P90 11-08.
- GAP3 recompute: P50 10-23/24, P90 about 11-03.
- The median barely moves because velocity's weights already padded for repairs. The clean case was understated by about 8 days.

---

## 13. Decisions owed (operator)

1. **O-1:** Re-point s8j's entry gate from Checkpoint 8i to Checkpoint 8h, so that §8j.1-8j.5, the CI reconcile and §8j.8 run alongside S-BIN B2/B3. Conditions:
   - 8j's chain still re-runs 8i;
   - §8j.6 step 1b and all paid steps stay behind 8i and its Rust package;
   - each push needs its own go-ahead;
   - the pin target is 2.1.289.

   Edit `INDEX.md:87`, the s8j header, and add a ledger entry.
2. **O-2:** Merge `origin/build/phase-0-to-1` (and optionally `origin/chore/temper-1.3.0`) into the local branch. Decide whether to accept the temper lint baseline of 6,606 findings as debt, given ADR-0102's no-`-D warnings` gate.
3. **O-3:** Confirm or reverse the 20 provisional AFK rulings: D21-106 to D21-128 (except 107 and 116-118) and D21-137. Several accepted ADRs rest on silence.
4. **O-4:** Sign Record A (ADR-0197): decision 7's enum table, then the flip after probes 0/1/2.
5. **O-5:** Decide what M5 certifies. Either:
   - require a disposition (discharged, scheduled, waived or deferred) for each of the about 50 unscheduled binding ACs and the 14 provisional ACs with no spike; or
   - record explicitly that M5 certifies the six exit checks only.
6. **O-6:** Accept that verify-stage ACs close only by human answer for the exit runs (C4), with one answer plus one relaunch per AC, or fund wiring `assertion_flipped` and the assertion map.
7. **O-7:** Add the Ctrl-C (C5) and `status` (C6) fixes to s8j before any paid lane run.
8. **O-8:** Pick the real fix item from the §8j.5 shortlist (at least 3 behavioural defects).
9. **O-9:** Define a recovery and re-freeze procedure for a failed first real run. None exists: Checkpoint 8j is one-shot and refuses an existing `rk-run/<item>`.
10. **O-10:** Add a landing step to the runbook: a `--no-ff` merge to `main` at M5.
11. **O-11:** Reconcile the Codex adapter's phase placement (ADR-0094 d2 says Phase 2; the design says Phase 4).

---

## 14. Risks

| Risk | Severity | Action |
|---|---|---|
| First real lane contact triggers another structural insertion. Both earlier insertions came from first contact (09-21, 09-24), and two real-path gaps are already known (D1 human roots, D9 hold). | High | Move a single real dispatch through `redkiln run` on the fixture item to right after the pin move in 8j, before the freeze. Time-box the repairs. |
| CI red locally; fixes stranded on remote; no green recorded; POSIX containment defect | High | Merge the remote now, get CI green on both legs, record run ids, and fix the residual `Group::is_empty`. |
| Adapter at 2.1.289 and least-privilege confinement unproven live; ADR-0197 still proposed; denial channel "UNMEASURED as a live frame" | High | Run probes 0/1/2 as the first paid 8j step. Expect at least one surprise. |
| Operator bandwidth: 11-13 sittings, about 30-40 approvals and answers; every Spec AC needs an answer plus a relaunch | Medium-High | Batch sittings, overlap 8j with 8i (O-1), keep N_f and N_s small in the chosen items, and consider wiring the pin path. |
| Ctrl-C inert during paid runs | Medium | About half a day of fixes: pass the root token into `dispatch`/`run`, and add a process-level test. |
| M5 read as "Phase 1 complete" while about 50 binding ACs are unscheduled and the review gate is uncalibrated | Medium | Publish a disposition table and an M5 scope statement (O-5). Keep the S-SUBTLE-CB disclaimer. |
| Single-machine risk: 40 unpushed commits; corpus repo on `D:`, remote unverified; S-BIN evidence in the corpus | Medium | Push the integrated branch. Confirm the corpus remote and that `CORPUS_PIN` resolves there. |
| Doc rot and process weight: stale headers contradict code; 70% docs-only commits | Medium | After M5, sweep comments and headers, split `run.rs` and `worktree.rs`, and cap stage-file size for Phase 2. |
| `status` wrong with two items in one store | Medium | Key it on `Event::item()` and add a two-item test. |
| Silent AC gaps (about 13% of sampled ids) | Medium | Build a register-to-test reconciliation report (SM-100), even a grep-based one, in CI. |
| Rank-tie lint blind to rust; orphan and rewrite events undetected; budget exhaustion unwired | Low-Medium | Put these on the post-M5 hardening list. Avoid rebasing `rk-run/*` refs until the detector exists. |
| Non-hermetic tests produce a false red on any machine with provider env set | Low-Medium | Scrub `ANTHROPIC_*`/`CLAUDE_*` in the spawned-binary test helpers. |

---

## 15. Recommendations (prioritised)

1. **Reconcile with the remote and get CI green now.**
   - Merge `origin/build/phase-0-to-1` into the local branch in one session.
   - Push with a go-ahead and iterate until both legs, `no-egress`, `structural` and `temper` are green.
   - Record the run ids, and correct every "never pushed / never run" statement.
   - This removes single-machine risk, brings the POSIX containment fix, and completes most of §8j.7.
2. **Unserialise 8j from 8i (O-1).** Start the 8j build half now, while the operator runs S-BIN B2/B3: the human-root arm, `hold_before` with `deny_unknown_fields`, `lane_proof`, the shortlist and the s8 amendments. Keep the pin move and paid probes behind Checkpoint 8i. Expected saving is about 1-3 days.
3. **Fix the confirmed defects before any paid lane run.** Add to s8j: wire the Ctrl-C token (C5); key `status` on `Event::item()` (C6); add `rust`/`python` to `rank_lint` (C9); and make rk-cli tests scrub provider env. Together these are about a day of work.
4. **Front-load the riskiest unknown.** Make the pin move plus probes 0/1/2 the first paid 8j step. Then put one real `redkiln run` dispatch on the fixture item before the 20-checkpoint freeze, so a structural surprise lands in 8j rather than at Step 8. Write the failed-first-run recovery procedure (O-9).
5. **Make M5 mean something precise (O-5).** Publish an AC disposition table for the about 50 unscheduled binding ids and the 14 unspiked provisionals. State in the M5 record that verify-stage ACs close by human answer, not mutation pin, and that the review gate is uncalibrated.
6. **Clear the ratification backlog in one sitting.** That covers the 20 provisional rulings, Record A's table, the temper baseline, and Codex placement (O-2 to O-4, O-11).
7. **After M5, run a short hardening pass before Phase 2:**
   - comment and header sweep; split `run.rs` and `worktree.rs`; give `GitError` a `Display`;
   - fix the `Digest` deserialisation;
   - build the orphan/rewrite detector and wire budget exhaustion;
   - add a standing AC-to-test report.

   Also put a size cap on stage files, so Phase 2 does not inherit Phase 1's process weight.

---

## 16. Roadmap after Phase 1 (judgement; low confidence)

| Phase | Delivers | Effort at observed pace |
|---|---|---|
| Post-M5 hardening | defect and debt list (section 15, item 7), about 50 ARMED ACs, doc sweep, landing to `main` | 1-2 weeks |
| Phase 2: packs and parallelism | incident/spike/initiative/project packs; parallel worktrees plus merge queue; .NET, Python and Unreal adapters (Unreal conditional on S-BIN); `rk-plugin-gen`; old-corpus migration (AC-463) | 6-10 weeks. Phase 1's plan doubled once real runs began; expect similar. |
| Phase 3: surfaces | GitHub and mdBook adapters, phone-answerable gates, web view, DORA metrics | 4-8 weeks, after Phase 2 is stable on one initiative |
| Phase 4: reach | Codex (after S-CODEX passes), Linear, Astro | 3-6 weeks of build, but entry requires Phase 3 "stable for a quarter", so it is calendar-bound |

---

## 17. Unverified and limits

- GitHub Actions conclusions for `7860e66`/`f571243`, and any green run. No `gh` access.
- Whether the remote-only fixes compile and pass the full local suite once merged.
- Windows child behaviour on console Ctrl-C (C5).
- The exact Spec AC counts (N_f, N_s) and the number of conditional gates in the real runs. These drive operator load.
- Commits `ebaf4ca` and `524aadd`, and the uncommitted live-tree edits, were not read. The 8i state may be ahead of what is reported here.
- Corpus repo durability and remote. The S-BIN evidence lives there.
- Mutation-kill claims (M-numbers) and corpus-feature tests were not re-run.
- Every percentage, grade and date in this report is judgement anchored to the cited evidence, not a measurement.
