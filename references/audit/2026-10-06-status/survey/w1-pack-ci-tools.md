# W1 audit: shipped pack/catalog (.redkiln/), top-level tests/, tools/, CI, deny/clippy policy, CORPUS_PIN

Snapshot: commit `36dd8ac` (build/phase-0-to-1), read from the pinned audit worktree. Read-only. Cargo was not run.
Tags: **[records]** = what the runbook/ADRs/decision ledger claim; **[code]** = what the files/tests in the snapshot show; **[unverified]** = could not be checked from a read-only tree.

## 0. Verdict

The shipped pack is real, loads, and is exercised hard, but only against a scripted fake harness. It is a 102-activity catalog plus one extended activity, two lane packs, a verb registry, a facet registry, 78 artifact kinds, 8 cards, 7 checklists and 4 KB atoms. Under fakes, the fix lane walks 14 of 14 selected activities and the story lane 13 of 13. No recorded run exists of either lane against a real `claude -p` using the shipped cards; that is Step 8j's job, which has not started. CI is thorough on paper (3 jobs, about 25 steps, 2 OS legs) and has never executed: the records say so in terms and no recorded run exists. CI also lacks a step known to be needed (pnpm for the Node adapter tests). Overall grade B: the authored artefacts are careful and mutation-tested, but both the live-agent behaviour and the cross-OS behaviour are unproven, and the exit criterion needs both.

Sub-grades: pack/catalog data B+ (structurally sound, but the cards are one-sentence v0 and most of the catalog is out of Phase-1 scope); tests/ fixtures A; tools/live-run.sh B+; CI authored B+ but proven nothing (C as evidence); deny/clippy policy A-.

## 1. Surface map

### 1.1 `.redkiln/` (136 tracked files)

| Part | Count / size | Citation |
| --- | --- | --- |
| Catalog activities (shipped) | **102** TOML files, 6,495 lines, 571 steps (185 agent, 93 check, 102 emit, 80 gate, 111 tool) | `git ls-files .redkiln/catalog` (102); step-kind counts by grep of `^kind =` |
| Extended (not shipped) | 1: `tag-release`, admitted only by a per-item `[trial] activity = "tag-release"` | `.redkiln/catalog-extended/tag-release.toml:1-30` |
| Packs | `bug-standard` (877 lines), `story` (908 lines) | `.redkiln/packs/*.toml`; `.redkiln/config.toml:10-11` enables both |
| Verb registry | 13 verb rows, 14 checklist/atom sources, 6 feed rows (rust, node) | `.redkiln/registry.toml` (`[verb]` at line 27 onward) |
| Facet registry | 21 facets; 2 declared, 5+1 derived, 4 kernel, 9 human | `.redkiln/facets.toml` (counted by grep) |
| Artifact registry | 78 kinds: 72 `activity`, 5 `human`, 1 `pass` (`TestReport`), 1 `extended` | `.redkiln/artifacts.toml` |
| Cards | 8 JSON files, one per role | `.redkiln/cards/*.json`, bound in `.redkiln/roles.toml` |
| Checklists / KB / questions | 7 checklists (232 lines), 4 KB atoms (23-33 lines each), 11 question definitions | `.redkiln/checklists/`, `.redkiln/kb/`, `.redkiln/processes/questions.toml` |
| Items | 2: `rk-fix-doctor-test-count` (bug), `rk-story-codex-adapter` (story) | `.redkiln/items/` |
| Records in-tree | `CHANGELOG.md` (ADR-0072 per-activity version log), `DISPOSITIONS-2026-09-21.md` | `.redkiln/CHANGELOG.md`, `.redkiln/DISPOSITIONS-2026-09-21.md` |

### 1.2 Top-level `tests/`

Three zero-or-one-file fixture crates, all workspace members (`Cargo.toml` `members = ["crates/*", "tests/fixtures/*"]`). They are must-fail controls, not a test suite:
- `rk-clock-read` is the AC-001..004 control. It holds one `SystemTime::now()` behind a non-default feature so the `clippy.toml` ban can be seen to fire (`tests/fixtures/rk-clock-read/src/lib.rs`).
- `rk-banned-dep` is the AC-005 control. It carries ADR-0086's banned `durable` crate as an optional dependency (`tests/fixtures/rk-banned-dep/Cargo.toml`).
- `rk-adapter-edge-fixture` is the AC-484 control, with zero dependencies on purpose (`tests/fixtures/rk-adapter-edge-fixture/src/lib.rs`).

All three are consumed by `crates/rk-kernel/tests/it/adapter_edge.rs`, and `rk-kernel`'s `Cargo.toml` declares the optional edges. The real test suite lives in `crates/*/tests`. Counting `#[test]` by grep gives roughly 1,450 test functions across 12 crates; `rk-kernel` has 448 and `rk-cli` 331.

### 1.3 `tools/`

Only `tools/live-run.sh` (370 lines, POSIX sh; `bash -n` is clean). It is "the only entry to a real run" (header, lines 1-26). It has modes `run | probe | wrap | answer | --check`, and a 9-step launch gate:
- It refuses a Claude Code session environment and reads the env denylist out of `crates/rk-adapter-claude/src/env_denylist.rs`.
- It forces `CLAUDE_CONFIG_DIR` and `DISABLE_AUTOUPDATER=1`.
- It compares the CLI version and sha256 to the pin before and after, and writes a launch record under the git-common-dir.

It was added in a single commit (`55fda45`, s8h interim 3). `--check` spawns no child, so its refusals are testable without spending money. `runbook/tools/` (6 files, out of this area) holds the checkpoint tooling.

### 1.4 Policy files

- `deny.toml` (144 lines): bans (ADR-0086's durable-execution names plus HTTP-client names), `unmaintained = "all"`, `yanked = "deny"`, a measured license allow-list with per-crate justification, `all-features = false` with the reason stated (`deny.toml:4-14`). There is no `[sources]` table (see risks).
- `clippy.toml` (35 lines): `disallowed-methods` (clock, rand, uuid, sleep, `set_var`/`remove_var`) and `disallowed-types` (`HashMap`/`HashSet`, `std::process::Command`, `tokio::process::Command`).
- `Cargo.toml` `[workspace.lints]`: `unsafe_code = "deny"`, `clippy::all = deny`, four extra denies, `nursery = warn`.
- `CORPUS_PIN`: corpus repo and SHA `61f590e5…`, predecessor SHA `47faec9f…`, a bundle name, 118 refs, an off-disk backup bundle on `C:`. `CORPUS_REPO` and `REDKILN_REPO` are `D:/` machine-local paths (by design; the bundle is the portable form).

## 2. What Phase 1 requires of this area

- **ADR-0011:** Phase 1 ships exactly two packs, `bug-standard` and `story`. The incident, spike, initiative and project packs are explicitly Phase 2 (`references/adr/0011*.md`, Decision and Consequences). Phase 1 may not be declared complete by running any other pack.
- **Exit criterion** (`runbook/INDEX.md`, "Where the vision lives"): Redkiln-RS runs its own fix lane and authors a Phase-2 story, in both lanes.
- **Step 8j** (`runbook/phase-1/s8j-run-preparation.md`): fixture proof of both lanes, CI green on the pushed branch, and run preparation. §8j.7 states D21-42 14a: "`ci.yml` green on the pushed branch, every job, both OS legs, is a Checkpoint 8 condition."
- **Catalog quality instruments** (s8b): AC-101..104, P6 lattice, ties and hard cycles zero, family totality.

## 3. Is the pack loadable and complete enough?

### 3.1 Loadable: yes [code], with strong test coverage

- `rk-kernel/tests/it/lattice.rs` loads the real `.redkiln/` (not a copy) via `Sources::under(...)` and asserts `packs == 2`, `catalog_len == 102`, ties 0, hard cycles 0, a pinned reading (`SHIPPED_PAIRS 279_936`, `SHIPPED_UNSAT 1_296`, `SHIPPED_DEAD 57`), and AC-104 family totality (`lattice.rs:523-626`, constants at `lattice.rs:45-47`).
- `rk-kernel/tests/it/lane_walk.rs` runs a real `select` plus a real `Runtime::run_activity_steps` over the shipped pack, catalog, registries and questions, with a scripted `FakeHarness`/`Toolchain`. The measured table (`lane_walk.rs:1395-1399`, `runbook/phase-1/s8g…md:386`, `s8j…md:554`) is **fix `walked=14 refused=0`** and **story `walked=13 refused=0`**. The test header says in terms: "It is not a real run and says nothing about `redkiln run`" (`lane_walk.rs`, last paragraph of the module doc).
- CI itself runs the lattice and validate tests by exact name with a positive control that the filter matched (`ci.yml` step "P6 lattice and validate over the shipped pack", about lines 114-127).
- **[records]** `redkiln doctor` has 42 rules including pack checks; Checkpoint 7e, 8b and 8h were re-run whole at `d24d15a` (2026-10-02) per `PROGRESS.md` rows 41 and 8h.

### 3.2 What each lane actually selects [code, `lane_walk.rs:1410-1579`]

- **Fix lane** (low-criticality Rust bug, 14 activities):
  - analysis: `minimal-repro`, `root-cause`
  - specification: `fix-spec`
  - implement: `tdd-red-green-refactor`, `change-summary`
  - verify: `confirmation-test`, `smoke-test`, `sca-scan`
  - review: `adversarial-review`
  - release: `build-package`, `tag-release` (extended, by trial)
  - closeout: `debt-logging`, `lightweight-retro`, `closure-record`
- **Story lane** (low-facet story, 13 activities):
  - specification: `bdd-scenarios`
  - planning: `t-shirt-sizing`
  - implement: `docs-update`, `tdd-red-green-refactor`, `change-summary`
  - verify: `smoke-test`, `sca-scan`
  - review: `adversarial-review`
  - release: `build-package`, `tag-release`
  - closeout: `debt-logging`, `lightweight-retro`, `closure-record`

Only 17 distinct shipped activities (plus `tag-release`) are walked by the two lanes. That is about 17% of the catalog.

### 3.3 Activity counts and Phase-2 deferral

| Cut | Count | Basis |
| --- | --- | --- |
| Shipped activities | 102 | `.redkiln/catalog/` |
| Tagged `kind:bug` and/or `kind:story` (the only kinds Phase 1's packs apply to) | 60 (13 bug-only, 14 story-only, 33 both) | grep of `tags =` lines |
| Tagged for neither (only incident/initiative/project/spike/security-vuln/tech-debt/chore/docs): Phase-2-deferred by ADR-0011 | **42** | same grep; ids listed in 3.3a below |
| Unreachable from the two installed packs at any rigor ("dead") | **57 of 102 (55.9%)** | `lattice.rs:36-47`; `s8b…md:701-710` |
| Unsatisfiable (kind has no admissible producer) in the lattice | 1,296 of 279,936 pairs (0.46%); five kinds: Postmortem, MutationSummary, ThreatModel, IntegrationReport, APIContract | `s8g…md:764` |
| Activities containing a tool verb with no `registry.toml` row (refuse by bare name if reached) | 26 | grep of `run = "toolchain:<verb>"` against the 13 `[verb]` rows; includes tagged-bug/story ones such as `bisect`, `mutation-testing`, `property-based-testing`, `rollback`, `rolling-deploy`, `canary-release`, `migration-*` |
| Agent steps whose output kind has an exported schema | 146 of 185 (79%); the 39 untyped steps sit in 30 activities, none of which the two lanes walk | `crates/rk-types/src/registry.rs:21-60` (24 exported kinds) against `outputs = "value:<Kind>"` |
| Question definitions | 11 defined vs 45 distinct referenced; every question reached by the 17 lane activities is defined | `.redkiln/processes/questions.toml`; grep |

**3.3a The 42 activities tagged for neither lane:** appetite-check, architecture-tradeoff-analysis, assumption-mapping, branch-by-abstraction, c4-container, c4-context, checklist-review, competitor-analysis, dependency-update, design-review, design-sprint, discovery-report, exploitability-analysis, failover, fault-tree-analysis, feature-flag-kill, formal-inspection, hotfix, incident-command, knowledge-harvest, opportunity-solution-tree, outcome-definition, problem-framing, production-readiness-review, prototype-test, rate-limit-shed, risk-register, rollback-mitigation, rolling-restart, spike-decision-review, spike-question-spec, spike-report, spike-timebox-plan, strangler-fig, structured-nl-requirements, tech-debt-assessment, technical-spike, test-after, timeline-reconstruction, use-case-spec, vertical-slicing, walking-skeleton.

The records handle the 55.9% dead figure as a deliberate Phase-1 scope effect. D21-47 V3 (`spikes/DECISIONS-2026-09-21.md:1928`) says "the installed pair is reported with its count and does not fire the kill line; its unreachability is Phase-1 scope by design (ADR-0011, E7)". Over the eight pre-registered packs the figure is 14/102 (13.7%), inside the 20% line (`DECISIONS-2026-09-21.md:2563-2568`). This is a fair reading, but the 57 includes some `kind:bug`/`kind:story`-tagged activities that are dead under the two installed packs for rigor-ceiling reasons (for example `unit-tests`, `lattice.rs:36-44`).

### 3.4 Completeness gaps that matter for the exit

1. **No real-agent evidence for the shipped cards.** Cards are one-sentence v0 prompts. For example the reviewer card reads "Give an independent verdict on the work under review. Emit the artifacts the activity declares, as StructuredOutput." (`.redkiln/cards/reviewer.json`), and every card still says "Step 7e's v0 card". Record A (ADR-0197, cards/models/run environment) "awaits s8j's probes" (`PROGRESS.md` row 8h). The only recorded live dispatches are the 7c probe and the s8i S-BIN capture sessions; none exercises the shipped pack. Whether these prompts produce schema-valid output over a full lane is **[unverified]** and is the main execution risk.
2. **The two items in `.redkiln/items/` are not the exit items.**
   - `rk-fix-doctor-test-count` is about a prose count in `crates/rk-cli/tests/it/doctor.rs`. s8j re-points it as the *fixture* item and reserves a different real fix item at §8j.5, which is not picked yet (`s8j…md:411-443`, `:611`).
   - `rk-story-codex-adapter` sets `external_dep = true`. s8g measured that this makes story's `design` selection refuse `Step5(CyclicDag { adr-record → design-doc → adr-record })` (`s8g…md:766`, still true on the optional-edge reading). s8j §8j.1(b) therefore retires it and replaces it with `rk-story-harden-spike-pack` at low facets (`s8j…md:343-349`, `:398`), which does not exist yet.
   - The prose in `doctor.rs` still says "thirty-eight" (`doctor.rs:188`) while the file now carries 47 `#[test]`s, so the item's own premise has drifted ("thirty-nine" in the item header).
3. **Triage has zero catalog activities** and `bug-standard` has no `[stage.triage]` block (ADR-0156 title; `bug-standard.toml:20-23` marks triage mandatory). Triage is covered by the kernel's `unknown_facet:change` gate. This is a documented design decision, not a defect, but it means "triage" is not exercised by any catalog activity.
4. **Smoke is shallow.** `checklist:smoke` has 12 rows that are `--version` and `--help` checks only. The `init`/`status` rows are owed to a ruling (D21-125; `.redkiln/checklists/smoke.toml:9-20`).
5. **Estimation KB is empty by admission:** "No estimate has been recorded" (`.redkiln/kb/estimation-history.md`). It is honest, not fabricated.
6. **Node toolchain feed values are `unmeasured`** (`.redkiln/registry.toml:166-171`). Fine for a Rust self-hosting run, but the pack declares node/unreal toolchains it cannot run.
7. **Most records the pack depends on are flagged `proposed` in file comments, though the ledger says several were flipped to accepted** (for example ADR-0191, 0195 and 0198 flips, `PROGRESS.md` D21-135/139). The in-file comments were not all updated, so a reader of `registry.toml` sees "proposed" for records that are accepted. This is stale prose, not wrong behaviour.

## 4. CI: what is enforced, and is there evidence it is green?

### 4.1 What `ci.yml` enforces (539 lines, 16 commits of history, triggers `push` and `pull_request`)

**Job `gates` (ubuntu-latest and windows-latest, fail-fast off).** Steps, in order:
1. toolchain show
2. `cargo fmt --all --check`
3. `cargo clippy --workspace --all-targets` (gate is the exit code; `RUSTFLAGS -D warnings` deliberately not set so `nursery` stays warn)
4. install cargo-audit 0.22.2, cargo-deny 0.20.2 and cargo-nextest 0.9.146, all `--locked`, from source
5. `cargo test --workspace`
6. rk-ports conformance bodies (feature `conformance`), with a positive control that the named test ran and passed
7. P6 lattice and validate over the shipped pack, each by exact name with a positive control
8. `cargo audit --deny unmaintained`
9. `cargo deny check bans advisories licenses`

**Job `no-egress` (ubuntu).** It checks the Docker image tag equals the `rust-toolchain.toml` pin (`rust:1.98.1`). It warms caches with the network on, then runs `cargo test --offline -p rk-kernel` and the `pack_validation::` suite under `docker --network none` (AC-498), with a guard that the filter selects at least one test.

**Job `structural` (ubuntu).** Roughly eight grep-style assertions:
- corpus residency (AC-465)
- usage fields not laundered (AC-362)
- anyhow confined to the binary crates
- the migrator reachable only as its own binary (AC-441/466)
- legacy formats read only inside the migrator (AC-442)
- git shells out through one wrapper only (ADR-0105)
- the run worktree not resolved from cwd (AC-529)
- the `rk-ports` port inventory (AC-481/483/528)

Design quality is good. The positive-control habit ("a filter that matches nothing exits 0") is applied consistently, comment filtering is handled, and `fetch-depth: 0` is set where history is read. The must-fail fixture crates mean the lint gates are shown able to fail.

### 4.2 Evidence CI is green on this branch: **none**

- `PROGRESS.md:559` (Step 5, 2026-09-13): "`ci.yml` is AUTHORED and has never run. Nothing is pushed."
- `s8j-run-preparation.md:1177-1180`: "The branch has never been pushed (`git branch -r` shows `origin/main` only), and `ci.yml` has never run." That stage is **Not started** (`PROGRESS.md` row 8j).
- Three `ARMED, NOT DISCHARGED` banners remain in the workflow itself (`ci.yml:183`, `:304`, `:530`). The AC-362 banner is stale because `rk-adapter-claude` now exists, and the line 540 step name still says "armed, never yet run".
- ADR-0127:178 states the POSIX containment arm is "ARMED to a CI run that has never executed".
- No recorded run (URL, run id, `gh run view`) exists in the snapshot's runbook, spikes or ADRs. s8j's own check shape expects one ("read from the recorded run", `s8j…md:1727`) and references `gh run view` (`:2497`), which confirms none yet exists.
- The local substitute is real but old: Checkpoint 5 (fmt/clippy/test/audit/deny, Windows only) was re-run whole and passed at `d24d15a` on 2026-10-02 inside Checkpoint 8h (13) (`PROGRESS.md` row 5). HEAD `36dd8ac` is 40 commits later. A whole-suite figure of 1,404 passed / 0 failed / 9 ignored over 44 binaries is recorded at `d24d15a` (row 8h). **[unverified]** whether the suite is green at `36dd8ac`.

### 4.3 Known and likely first-run failures

1. **Missing pnpm step (known).** `ci.yml` has no pnpm install, so about 10 `rk-adapter-toolchain-node` tests fail at their preflight. The fix is drafted in s8j §8j.7 (`s8j…md:1181-1203`) but not applied; `git grep -c pnpm` on `ci.yml` returns nothing. This is documented, not hidden.
2. **POSIX arm never compiled or run.** `rk-containment`, `env.rs` in both toolchain adapters, and `libc` use `#[cfg(unix)]` arms flagged as armed (`crates/rk-containment/src/ffi.rs`, `posix.rs`, `lib.rs`). All development and every recorded checkpoint ran on Windows. The ubuntu leg will be the first compile of these. This is the likeliest source of a red first run **[inferred]**.
3. **Machine-local `D:/` literals in test inputs** (`rk-adapter-claude/tests/it/{classify,command,probe_denials,support}.rs`, `rk-adapter-toolchain-rust/tests/it/{affected,selection,verbs}.rs`). They look like synthetic strings, and whether they behave on Linux is **[unverified]**.
4. **First-run unknowns** the stage file itself concedes: "Whatever else a first CI run needs is found by running it" (`s8j…md:1209`). Docker image `rust:1.98.1` availability and the 218-package offline fetch are **[unverified]**.
5. **Cost and flakiness.** Every `gates` run does three `cargo install` source builds (audit, deny, nextest) with no cache, twice (two OS legs). This is slow but not wrong.
6. cargo-nextest is installed and its version is pinned, but the gate's own command is `cargo test --workspace`. This is intentional (the Rust adapter's nextest-path tests spawn it), and the comment says so (`ci.yml:72-77`).

## 5. Code and config quality

- **Escape hatches:** the area has essentially no non-test Rust. The three fixture `lib.rs` files contain no `unwrap/expect/panic/allow/unsafe` (`git grep` over `tests/`: zero). `unsafe_code = "deny"` is workspace-wide.
- **Data quality of the catalog is high.** Comment density in `catalog/*.toml` is 180 of 6,495 lines (2.8%); `artifacts.toml` 21/341. Every activity carries the same schema (id, version, stage, provides, requires, tags, rank, family, rigor band, cost class, budget, rationale). The `CHANGELOG.md` records each ADR-0072 edit with old and new text. `lattice.rs` pins the shipped reading so drift fails the build.
- **Comment-to-code noise is high in the pack and registry:** `packs/*.toml` 359 of 1,785 lines (20%), `registry.toml` 83/184 (45%), `facets.toml` 79/194. The comments are dated provenance notes, consistent with the programme's house style. The cost is readability: `registry.toml`'s 40-line header cites six decision ids before the first row.
- **Prose rot:** some in-file comments still say "proposed" for records now accepted (see 3.4 item 7), and the `s8d` text elsewhere in the repo quotes retired counts. Low severity but pervasive.
- **deny.toml:** `"MIT-0"` appears twice in the `allow` list (`deny.toml`, lines near 94 and 123); harmless. No `[sources]` table, so unknown registries and git sources stay at cargo-deny's default (warn). Advisory handling and licence reasoning are exemplary.
- **Test depth:**
  - The pack is covered by integration tests (`lattice`, `lane_walk`, `pack_validation`, `quality_block`, `verb_registry`, `selection`) over the real files, plus golden parity against the p9 Python lattice.
  - Must-fail fixtures exist for each lint.
  - Mutation-style refuting rounds are recorded (for example "mutant M29" in `PROGRESS.md` row 8c).
  - What is missing is any non-fake end-to-end layer.
- **`live-run.sh`:** careful (anchored extraction of the denylist, no-clobber launch record, version re-check). It is a guard against accident, "not a security boundary" by its own statement (header). Its refusal behaviour is testable via `--check`. It is Windows/Git Bash-oriented (`uname -s` case at line 152).

## 6. Risks

1. **Exit-criterion risk (high):** no real-agent lane run has happened; the cards are one-sentence v0. A first live run will likely surface prompt and schema-conformance problems the fakes cannot.
2. **CI-never-run risk (high):** D21-42 14a makes CI green on both OS legs a Checkpoint 8 condition. The workflow has never executed, a known step (pnpm) is missing, and the POSIX arm has never compiled. Expect at least one red iteration; the record's "found by running it" loop (`s8j…md:1209-1215`) already assumes this.
3. **Stale local-green evidence (medium):** the last recorded whole-suite pass is `d24d15a`, 40 commits before the snapshot.
4. **Scope-fit of the catalog (medium):** 57 of 102 activities are unreachable from the installed packs and 26 reference unmapped verbs. They are Phase-2 inventory, not Phase-1 deliverables; maintaining their lattice pins costs effort at every pack edit (SHIPPED_DEAD was re-pinned 56 to 57 by dated edit).
5. **Item readiness (medium):** the real fix item and the real story item (`rk-story-harden-spike-pack`) are described in s8j but not created; the one committed story item would hit a known `CyclicDag` refusal.
6. **Windows-only development (medium):** `CORPUS_PIN`, `live-run.sh`, and the checkpoint tooling assume Git Bash on one box; CI's ubuntu leg is the first portability test.
7. **Single point of truth drift (low):** ADR-status comments inside `.redkiln/` files lag the ledger.

## 7. Unverified (could not be checked read-only)

- Whether the suite and the five local gates are green at `36dd8ac` (last recorded: `d24d15a`).
- Whether `ci.yml` parses and runs on GitHub (never executed); whether the ubuntu leg compiles the POSIX arms.
- Behaviour of the shipped cards under a real `claude -p` over a full lane.
- That `redkiln` as a built binary runs the shipped pack end to end under fakes. The kernel-level walk is shown; Checkpoint 7h's `FIX_ITEM` run is recorded as passing at the cited commits, but the checkpoint blocks were not re-executed here.
- The `[models]` ids (`claude-haiku-4-5-20251001`, `claude-sonnet-5`, `claude-opus-5-5`) resolve in the operator's account; the harness adoption check (RT-27) is the control, not run here.
- Test counts are grep-derived (`#[test]` attributes), not from a `cargo test` run.
