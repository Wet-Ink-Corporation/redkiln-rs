# W1 — Vision ledger (redkiln-rs status-quo audit)

Snapshot: commit 36dd8ac (2026-10-05), worktree `audit-wt`, read-only. 517 commits since 2026-09-01. Paths below are relative to the worktree. "RECORDS" means what documents assert; "CODE" means what I confirmed in the tree. I did not run cargo or any test, so nothing here says the code works, only that it is present and what the records say about it.

## 0. Verdict in brief

The vision is a Rust binary that owns the loop, with a deterministic kernel, git-trailer durable state, human gates as marks, and typed agent output. The shape is fully decided and the Phase-1 build surface is largely present: 12 crates, about 237,600 lines of Rust (my sum of `git ls-files` .rs lines), and all four Phase-1 real adapters exist as crates. The Phase-1 exit, meaning Redkiln-RS runs its own fix lane and authors a Phase-2 story, has not been met. Step 8 is recorded as RE-OPENED with "Checkpoint 8 has NOT passed, and M5 is not reached" (runbook/PROGRESS.md:51). Stage 8i is in progress (PROGRESS.md:49), 8j (fixture proof of both lanes) is "Not started" (PROGRESS.md:50), and no Phase 2, 3 or 4 deliverable is built (section 8). Ten of the 39 design areas are still BLOCKED, and the "design final" label is explicitly forbidden (references/2026-09-08-architecture-design.md:25).

## 1. The core bets and why

Sources: references/2026-08-31-rust-inversion-RFC.md (RFC) and references/2026-09-02-rfc-amendment.md (amendment).

**Bet 1, invert control.** Rust owns the loop and spawns `claude -p` per step (RFC:11-13, :25-50). Three findings backed it:
- The cost argument runs against intuition [documented] (RFC:81).
- About two-thirds of the workflow layer is compensation for the Workflow runtime's sandbox, so it deletes rather than ports [measured] (RFC:121).
- About 50% of the command layer is orchestration logic living as prose the model must re-execute each run [measured] (RFC:152).

The RFC's ranked benefits (RFC:225-236): enforced transitions; deleted machinery; per-agent cost attribution; hard budget stops and cancel; prompt corpus separated from the engine; deterministic resume; headless/CI operation; tier policy as typestate; multi-harness.

**Bet 2, port the engine (`src/`, 40k LOC TypeScript) to Rust.** The RFC called this "materially weaker" (RFC:15). It carried an unbounded YAML byte-parity risk R3 and a permanent loss of the "clone and it works" plugin property. The RFC separated the bets and recommended taking Bet 1 and staging Bet 2 behind spikes (RFC:19-21).
- What happened to Bet 2: ADR-0002 chose a clean break instead (references/adr/0002-clean-break-self-hosting-switch.md:35). The new store owns its own serialiser, `redkiln-migrate` converts old content once, and "The YAML byte-parity risk (R3) is dissolved" (ADR-0002, Consequences). That is a third option the RFC did not list.

**Bet 3, implicit in RFC section 7 and design principle 19: keep the plugin surface.** The market scan found no adopted spec-driven tool owns control flow. Standalone binaries get adopted only when they own a durable artifact and also ship a skill/MCP layer (RFC:278-304). The design carries this as PROVISIONAL, on borrowed evidence (design principle 19, architecture-design.md:185). Phase 1 ships no plugin and `rk-plugin-gen` is Phase 2.

**The unoccupied gap.** The RFC claims no harness ships a mid-run human approval gate (RFC:278-304). The amendment says this is "the differentiator" and was NOT TESTED as of 2026-09-02 (amendment:285-291). It was later partly tested: F18/F19/F28/F38 gate findings are cited in the design, but P8 returned no result on its own question (architecture-design.md:25).

**What the spikes changed (amendment), RECORDS:**
- R2, "effort cannot be pinned per dispatch": DEAD. `--agents` takes a per-agent `effort` key (amendment:44-66).
- R4/R6, the two cache mitigations "conflict": dissolved and inverted (amendment:69).
- The quality kill criterion did not fire. Detection was 23/23 vs 23/23, and every quality metric saturated, so the arms cannot be ranked (amendment:208-283).
- The cost case rests on a 1.72x in-session premium ($97.39 vs $56.47), from one corpus on one machine (amendment:208-283, :325-367).
- The amendment's recommended framing is that quality is "a precondition" rather than "a differentiator", and the "most under-measured thing" is reviewer reliability: a CLEAN item flipped 2 of 3 times (amendment:325-367).
- Bet 2 was untouched by the spike (amendment:293).

**RFC kill criteria, status.**
- Experiment 2 (context fidelity), the criterion that "alone kills it": did not fire.
- Experiment 1 (cache): resolved as 31% cost unmitigated and 0% mitigated (amendment:325-367).
- Experiment 4 (the fix lane end to end, "1-2 weeks", RFC:322): not complete after five weeks. The design was re-scoped far beyond the RFC's effort table, which priced Bet 1 at roughly 10k LOC of new work (RFC:261-276). See sections 4 and 8.

**Architecture proposal (references/2026-09-05-architecture-proposal.md, skimmed).** It is superseded by the design as of 2026-09-08 (proposal:3). It is the earlier statement of the same shape: five ports, two pack axes, typed artifacts, git-first state, three gates, and the same five-phase table (proposal:361-369).

## 2. The design in one paragraph

RECORDS, architecture-design.md:9 (section 0). The design is a clean-break single Rust binary owning the loop, built from:
- a deterministic kernel (selection, gates, budgets, retry, replay) that dispatches one harness process per activity;
- git trailers first, plus an append-only event log, split by one test: "if the database were deleted, would `redkiln run` take a different next action?";
- human gates as durable marks, being a `(token, tree_hash, deadline)` record, a `Gate:` trailer and a process that exits 0;
- every agent output a typed value, with markdown only a rendering;
- five sync-trait ports (event store, harness, tracker, docs site, toolchain), each with one real adapter and one fake, `.await` confined to adapters, and git deliberately not a port.

The design describes itself as "authoritative ... for shape and order" and "not 'design final'" (architecture-design.md:3).

## 3. The 19 principles (design section 2.1, architecture-design.md:167-185)

1. Measure cost, stability and mechanism, never success rates; a saturated, powerless or denominator-less metric is not a result.
2. Rust holds the loop; the model holds no control flow and its output never feeds selection, tiering, budgets or terminal conditions.
3. Determinism is a property of the input too; an LLM-emitted context vector is BLOCKED as a `require` key until identity is at least 0.80 (measured 0.25/0.15/0.10/0.10, F36).
4. Git is truth, the record is a cache, and `reconcile()` runs at every acting entry; nothing writes back from store to git.
5. Nothing waits: gates, long gaps and rate-limit resets are a durable mark plus a later read.
6. Absence of evidence is a block, and the absence is usually silent (F20: a tool allowlist silently voids `--json-schema` at exit 0).
7. Explicit over discovered; every dispatch passes card, prompt, tools, schema, MCP config and mode, and adoption is verified from the child's own report.
8. Typed in, typed out; markdown is a rendering, but a digest carries files, not judgment.
9. One dispatch, one schema per activity; no decomposing sequential work across agents to buy quality.
10. Gate duration is free, frequency is marginally costly, and placement is a value decision.
11. No mitigation generalises past the axis it was measured on (the two cache mitigations point in opposite directions).
12. Neither gate is sufficient alone: a gate may add a finding and may never clear a claim; blocking is computed in Rust from re-executed evidence.
13. A finding that does not block still has a destination (the Advisory sink).
14. An ordinal rigor ladder must be monotone in the quality it promises.
15. A constant-valued policy predicate carries no information and may not terminate anything.
16. Ports exist at named axes only; sync traits, git is not a port, and cancel is a root-owned token backed by a real tree kill.
17. The kernel knows no language and no tracker; ecosystems live in technology packs and collaboration surfaces are projections.
18. Lessons, not code, from the old redkiln (101 CLI test files cross as black-box specs; `reconcile.rs` is the one import); a single trailer axis is insufficient (F12: 20.3% of slices rejected mid-flight).
19. The plugin surface stays and is generated from one source, on borrowed evidence, and none ships in Phase 1.

Count verified: 19 rows at architecture-design.md:167-185. Thirteen restate the proposal and six are new (architecture-design.md:159).

## 4. Phase-1 exit, precisely

**Phase 0 exit** (design section 10.2 row, architecture-design.md:1866), on which Phase 1 entry depends:
- (a) `git status --porcelain` is empty.
- (b) Corpus reads resolve to the pinned SHA via the object store, not a working tree (AC-464).
- (c) Every register row marked "gates Phase 1" has an F-numbered finding or an ADR re-scope.
- (d) The blind panels are filled (both DONE).
- Phase 1 entry needs (a), (b), (d) and the non-S-BIN half of (c) (architecture-design.md:1867).

**Phase 1 scope** (architecture-design.md:1867, ADR-0094 decision 1, ADR-0011):
- Crates `rk-types`, `rk-kernel`, `rk-store`, `rk-git`, `rk-ports`.
- **Four real adapters, each with a fake:** SQLite (EventStore), Claude Code (Harness), Node (Toolchain), Rust (Toolchain). `Tracker` and `DocsSite` are fake-only until Phase 3 (ADR-0094 decision 2).
- Two process packs only: `bug-standard` and `story` (ADR-0011).
- Mark-based human gate with the CLI as the only answer channel.
- Typed artifacts under layout C.
- The hybrid git write path (gix reads, shell writes).
- `redkiln-migrate` v0.

**Exit checks, as runnable checks** (architecture-design.md:1867, restated at runbook/phase-1/s8-exit-criterion.md:15-27). The sequence is (1) (1b) (2) (3) (4) (6) (7). Check (5) was WITHDRAWN on 2026-09-15 (DECISIONS-2026-09-14 D-14), its number retired and nothing renumbered.
1. **(1) AC-473, fix-lane self-hosting.** At least one recorded fix-lane run on `D:/repos/redkiln-rs` itself.
   - The event log must contain every lane stage from intake to a landed commit, with no stage performed by the old redkiln.
   - The toolchain stage must run through `rk-adapter-toolchain-rust` behind the `Toolchain` port.
   - The claim is refused if any stage is missing or the toolchain was anything but the Rust adapter (AC-473 at acceptance-criteria.md:7651-7662).
   - "A landed commit" is read as a kernel-made content checkpoint commit on `rk-run/<item>`. Nothing is integrated in Phase 1, and the operator's merge is the hand-off (AC-473 reading note, acceptance-criteria.md:7663-7672).
2. **(1b) AC-521, story-lane twin.** At least one recorded `story`-lane run authors a Phase-2 story into this repo's backlog.
   - Its event log must contain every story-lane stage.
   - The story artifact's digest must appear in the run's `artifact.write` events (AC-521 at acceptance-criteria.md:7686-7701).
   - Together with AC-473 this is "self-hosting means both lanes" (ADR-0094 decision 3).
3. **(2) Kernel replay.** `redkiln doctor --rebuild` reproduces the stage-level next action from git alone (AC-184, SM-124; AC-161/AC-164 narrowed). The design flags this as "the largest untested assumption" because F19 proved suspension lossless 174/174 but never replayed the FSM (architecture-design.md:1991-1996).
4. **(3) Resume across a real kill.** Deterministic resume across a real `taskkill /T /F` at more than one step (F38 held exactly one step at n=3).
5. **(4) Determinism ban.** The ban list fails the build: `cargo clippy --workspace --all-targets` gated on exit code, plus a must-fail fixture that reintroduces a clock read (AC-001 to AC-004).
6. **(6) Facet lints on both source classes.**
   - `redkiln lint --facet-stability` (AC-071) for `source = "llm"`.
   - `redkiln lint --facet-provenance` (AC-520) for `source = "derived"`.
   - No lexical-scan facet may key a `require` row.
   - Every boolean `require` row must handle `null` explicitly.
7. **(7) Cost.** Every dispatch event carries a cost row (AC-394). Headroom is reported only as a batch delta with `n` and `resolution_limited` (AC-372/373/395, SM-111).

**What Phase 1 "may not" do** (architecture-design.md:1867):
- Key a `require` row on an LLM-emitted facet.
- Key a row on `auth`, `ui`, `flag_exists` or `data_migration` as computed by the lexical deriver, before S-FACET scores it.
- Finalise the `ChangeSummary` type before S-BIN.
- Enable the card append route before S-H1c.
- Set a quality-shorthand default.
- Report any run as 1h TTL without per-run `ephemeral_*` evidence.
- Let gix own the write path.
- Declare Phase 1 complete on the fix lane alone.
- Describe the design as final.

**Discrepancy to flag.** The 2026-09-05 proposal's Phase-1 exit (proposal:365) also listed "`--stay`-class error unrepresentable" and "`--system-prompt` + worktree combination measured". Both were re-homed: (5) was folded into AC-294 by D-14, and the cache measurement is carried by S-WT and F30 (architecture-design.md principle 11).

## 5. Phase 2, 3, 4 roadmap (architecture-design.md:1868-1870)

| Phase | Entry condition | Delivers | Exit checks | May not |
| --- | --- | --- | --- | --- |
| **2, packs and parallelism** | Phase 1 exit (1) recorded, "self-hosting is a log fact, not a claim" | Remaining process packs (incident, spike, initiative, project) with weights and fast path; project DAG in parallel worktrees plus a merge queue; .NET, Python and **Unreal** toolchain adapters (ADR-0008); `rk-plugin-gen`; migration of the old corpus | (1) AC-463 migration differential green (AC-451 section accounting, AC-461 zero unrecorded manual interventions, AC-462 every sub-80% slot carries an Accepted ADR id); (2) an initiative advanced end to end headlessly; (3) a UE item verified through the toolchain adapter, **conditional on S-BIN**; (4) per-worktree cache cost measured (SM-053) | Ship the plugin surface as evidenced, since all distribution evidence is borrowed |
| **3, surfaces** | Phase 2 stable on one initiative | GitHub adapter and comment grammar; mdBook adapter and docs pipeline; web view; DORA metrics from the event log | (1) a gate answered from a phone yields the identical `(token, tree_hash, deadline)` mark and re-validates staleness at the read site; (2) a collaborator with no CLI sees status; (3) `doctor --rebuild` drops and rebuilds all three projections from git plus the log | Let a projection become authoritative |
| **4, reach** | Phase 3 stable for a quarter | Codex adapter, only after S-CODEX passes; Linear and Astro adapters as demanded | None registered (AC-239 is PROVISIONAL for that reason) | Put a non-Claude adapter in a gate before it passes E3-shaped checks |

**Inconsistency (RECORDS vs RECORDS).** ADR-0094 decision 2 says the Codex adapter "is Phase 2 and stays behind the CI decision ADR-0010 defers" (references/adr/0094-phase-1-ships-a-rust-toolchain-adapter-and-self-hosting-means-both-lanes.md, Decision 2). Design 10.2 puts Codex in Phase 4, and design 9.1 says `rk-adapter-codex` "is not in Phase 1" (architecture-design.md:1728, :1870). The repo's exit-run story item `.redkiln/items/rk-story-codex-adapter.toml` is a Phase-2 Codex story that cites ADR-0094. The Phase-2 versus Phase-4 placement of the Codex adapter should be reconciled. S-CODEX is registered and unrun (METHOD-S-CODEX.md; architecture-design.md:429).

## 6. DECIDED vs BLOCKED in design section 0.1 (architecture-design.md:13-58)

Tally as stated: "Twenty-six DECIDED, three PROVISIONAL, ten BLOCKED" of 39 (architecture-design.md:15). I recounted the table rows (:19-57): 10 BLOCKED, 3 PROVISIONAL and 26 DECIDED, so the header holds.

**BLOCKED (10), with what unblocks each:**
1. Selection engine, the LLM-sourced context-vector source (ADR-0079, superseded by ADR-0090: human-sourced for Phase 1). Unblock: AC-071's five clauses, with identity at least 0.80. Gates Phase 1: yes, as a prohibition on `require` rows.
2. Selection engine, the derived-facet source (ADR-0089). Unblock: S-FACET. Gates Phase 1: yes, as a prohibition on rows keyed on `auth`, `ui`, `flag_exists` or `data_migration`.
3. Selection step 3, the `stale_policy` setting. The Phase-1 value is `rematerialise`, coarse scope. Unblock: P2-B, a re-run on a corpus with real perturbations.
4. `ChangeSummary` type design. Unblock: S-BIN on the real Unreal project. Gates Phase 1 only for that type and the `change_summary` body. **Since narrowed:** ADR-0165 (2026-09-24) makes it an enum with the `Text` arm finalised now and S-BIN blocking only the `Binary` arm (architecture-design.md:22, :1866).
5. Card-delivery append route. Unblock: S-H1c. Phase 1 default: static `--system-prompt`.
6. Quality shorthand default. Unblock: S-P7B, a changed criterion rather than a larger n (ADR-0111).
7. "Design final" as a whole. Unblock: P8's 65-minute and 24-hour arms and three controls, S-BIN, S-H1c and S-P7B. "It gates the label, not the build" (architecture-design.md:25).
8. Git write path, the common-dir interplay. A hybrid is adopted. Unblock: S-GIX/a.
9. Git trailer writing. Shells out meanwhile. Unblock: S-GIX/b's write half.
10. Cache-TTL verification. The rule is decided. Unblock: S-TTL1 (narrowed by ADR-0113). Constraint: no run may be reported as 1h without per-run `ephemeral_*` evidence.

**PROVISIONAL (3):**
- Mid-graph gate placement and resume cost (P8-B).
- Review-gate calibration against false approval (S-SUBTLE-CB).
- Distribution and the plugin surface.

**DECIDED (26)**, the spine:
- Architecture shape, the kernel, memo and budgets.
- No durable-execution framework.
- Five sync ports, with git not a port.
- The tier-1/tier-2 split and the second trailer axis.
- `reconcile()` at every entry.
- The human gate as a durable mark, and its record.
- The review gate (reviewer proposes, machine confirms).
- Neither gate is sufficient alone.
- The lenient policy default and the Advisory sink.
- Per-AC verification.
- Structured output as a configuration-time check.
- Isolation, per-dispatch effort and model, the split cache rule, cost and headroom telemetry, the throttle.
- Packs on two axes, catalog v0 and the lattice.
- Typed artifacts and layout C.
- Clean break and `redkiln-migrate`.
- Rust workspace and lint policy.
- Phase-1 scope.
- E3 false-positive rates.

**Post-design movement (RECORDS):**
- S-GIX was executed (ADR-0091, ADR-0098). S-GIX/a returned no-go, so the shell fallback ships (runbook/PROGRESS.md:71, :108). The git floor is >= 2.20, since S-GIX/c returned go.
- ADR-0090 settled the blocking gate.
- S-BIN is IN PROGRESS at stage 8i (PROGRESS.md:49).
- I found no record in the sources I read that S-FACET, S-H1c, S-P7B, S-CODEX or P8-B have run. Treat them as unrun, but see section 9.

## 7. Registers (headers and counts)

**AC register** (references/2026-09-08-acceptance-criteria.md):
- Header count (as of the 2026-09-15 amendment): "526 criteria in force ... 480 binding, 20 provisional, 20 human-gated, 6 conditional" (acceptance-criteria.md:41-42).
- I counted 526 unique `**AC-nnn**` ids, maximum AC-532, so the count agrees with the ids. The criteria sit across 13 areas (section 0.1 table).
- Caveat: the register carries reading notes and dated amendments up to 2026-10-02 (AC-473, :7663-7672), but I did not re-derive the binding/provisional split after 2026-09-15. The 480/20/20/6 split is unverified for the current tree.
- The header says the register is "Binding on the Phase 1 build runbook; it decides nothing new."

**SM register** (references/2026-09-08-success-measures.md):
- 135 measure definitions (my count of `**SM-nnn` headings), SM-001..SM-154 plus SM-157 (SM:3; AC register :41-42).
- Stale note: the SM header's "Companion" line still reads "AC-001..AC-519 plus AC-520" (SM:5), though the AC register has since grown to AC-532.
- Doctrine: five metrics saturated, and every discriminating result came from cost, stability or mechanism (SM:10-14).
- SM-002, SM-052 and SM-073 have "no baseline" under ADR-0106 (SM:3).
- Many measures read "first measurement establishes it" (SM:6).

**ADRs:** the README says 198 records (references/adr/README.md:3). The 200 files on disk are those 198 plus README.md and ERRATA.md, so there is no discrepancy.

## 8. Vision goals evidenced vs not started (RECORDS plus what I confirmed in CODE)

**Evidenced.**
- *The loop is Rust and the shape matches.* CODE: crates `rk-types`, `rk-kernel` (about 102k lines), `rk-ports`, `rk-store`, `rk-git`, `rk-cli`, `rk-containment`, the four adapters, and `redkiln-migrate`.
- *Five ports as sync traits, async confined to the Claude adapter.*
  - CODE: `pub trait EventStore | Harness | Tracker | DocsSite | Toolchain: Send + Sync` at `crates/rk-ports/src/event_store.rs:184`, `harness.rs:34`, `tracker.rs:37`, `docs_site.rs:19`, `toolchain.rs:392`.
  - CODE: `git grep 'async fn' crates/rk-ports/src` finds only a doc comment (`lib.rs:28`).
  - CODE: tokio appears only in `crates/rk-adapter-claude/Cargo.toml:35`; `rk-kernel/Cargo.toml:170` records "No async runtime" (AC-528, ADR-0076).
- *Determinism ban as a workspace lint.* CODE: `clippy.toml` bans `SystemTime::now`, `Instant::now`, `rand::random`, `thread::sleep` and `env::set_var` (AC-001 to AC-004; `clippy.toml:1-30`).
- *Phase 0 closed.* RECORDS: M0 reached at Checkpoint 2 (`bbdaf35`), and the blocking gate discharged by ADR-0090 (PROGRESS.md:21-26).
- *M3, "the kernel decides".* RECORDS: reached 2026-09-17 at Checkpoints 7b (`ddc57be`) and 7b-ii (`164e212`) (runbook/INDEX.md, Milestones table). INDEX also defines M4 (four adapters, a CLI, a migrator and a loadable pack set); I did not see an explicit "reached" annotation on M4, but 7c, 7d and 7e are recorded closed or re-run green (PROGRESS.md:33-35).
- *Four Phase-1 real adapters.* CODE: crates `rk-adapter-sqlite`, `rk-adapter-claude`, `rk-adapter-toolchain-node` and `rk-adapter-toolchain-rust`.
- *Catalog v0 and packs.* CODE: `.redkiln/catalog/` holds 102 activity files, and `.redkiln/` has cards, checklists and packs.
- *Clean break and migrator.* CODE: `crates/redkiln-migrate`, about 4,000 lines.
- *Sub-stages 8a to 8h recorded passed or re-run green, with provisional marks.* RECORDS: PROGRESS.md:40-48. Several carry "provisional, re-run pending `s8j`" marks (8a, 8c, 8d), so these are not fully closed.
- *Phase-1 exit checks (2), (4) and (6) on both lints.* RECORDS: measured passing on 2026-09-24 at `4c282d7`, with (3)'s asserted half also passing (PROGRESS.md:51). That was before the eleven sub-stages were inserted, so I treat it as stale until s8j and Step 8 re-run.

**Not yet evidenced.**
- *The exit itself.* No recorded real fix-lane run and no recorded real story-lane run is landed under Redkiln-RS.
  - PROGRESS.md:51 says Checkpoint 8 has NOT passed and M5 is not reached; 8j is "Not started" (PROGRESS.md:50).
  - The first real run against the product pack failed to load (cycle lint, then a rank tie; PROGRESS.md:51).
  - Stages 8e to 8h then built the tier-1 trailers, landing checkpoint and run worktrees that the failed run exposed as missing (PROGRESS.md:45-48, :51). All of these are unproven by a real run.
- *The cost premise at product level.* The 1.72x in-session premium is one corpus and one machine.
- *The human-gate mechanism as a product.* Suspension is lossless at 174/174 and mid-graph resume holds at n=3, but P8's cost arms and controls are unrun (architecture-design.md:25, :29).
- *A real run of the S-BIN reviewer on no-diff items.* In progress; B1 was just completed (recent commits `ee44bf1`, `36dd8ac`).
- *Phase 2:* remaining packs (incident, spike, initiative, project), the parallel DAG and merge queue (`GitOp::Integrate` has no caller per the AC-473 reading note), .NET, Python and Unreal adapters, `rk-plugin-gen`, and old-corpus migration. No such crate or file exists; `git ls-files` for plugin-gen, merge-queue, github, mdbook, codex, linear and astro found only the CI workflow and the exit-run story item.
- *Phase 3:* GitHub and mdBook adapters, web view and DORA metrics. None built. `Tracker` and `DocsSite` have fakes only.
- *Phase 4:* the Codex, Linear and Astro adapters. None built.

## 9. Unverified

- Whether any crate builds or passes its tests at 36dd8ac. I did not run cargo.
- The current AC provisional and binding counts after 2026-09-15 (section 7).
- Whether S-FACET, S-H1c, S-P7B, S-CODEX, P8-B and S-SUBTLE-CB have since run. My targeted greps found no result, and the exit file at runbook/phase-1/s8-exit-criterion.md:324 says S-BIN, S-H1c and P8's unrun arms still gate "design final".
- The state of 8i beyond the tracker row (PROGRESS.md:49). The task brief says B1 just completed.
- The M4 "reached" annotation. INDEX.md defines M4 but the Milestones row I read does not mark it reached.
- Per-stage claims such as "re-run WHOLE and passed at d24d15a". I report them as RECORDS only.
- The scale of rework implied by the many "provisional, re-run pending" marks. I cannot tell which are real regressions and which are tracker discipline.

## 10. Completeness judgement (Phases 1 to 4)

My estimate is about 35% of the full vision. This is a judgement from records and file inventory, not a measurement.
- Phase 1 is about 80% built by surface area: all Phase-1 crates exist and sub-stages 8a to 8h are recorded passed or re-run green. Its exit evidence is not yet delivered, because the six checks have not passed on the current tree and the hardest check (real self-hosted runs in both lanes) has never succeeded.
- Phase 1 carries roughly half of the programme's total effort (kernel, gates, store, adapters, CLI, migrator).
- Phases 2, 3 and 4 are essentially 0% built; they are specified only.
- Weighting: about half of the total effort times about 80% of that half gives about 40%, discounted to about 35% for the unproven exit and the unrun blocking spikes.
