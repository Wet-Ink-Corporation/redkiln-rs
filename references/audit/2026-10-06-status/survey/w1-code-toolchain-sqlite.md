# W1 audit: toolchain-rust, toolchain-node, sqlite adapters

Snapshot: commit 36dd8ac (worktree `audit-wt`), read-only. No cargo run; all test claims below are from reading tests and the records, not from executing them.

## Verdict

Grade B overall (Rust toolchain B+, SQLite B, Node B-). Completeness against what Phase 1 asks of these three crates is roughly 85%.

- The three crates are real, wired into the composition root, and deeply tested. The remaining gaps are named, dated and mostly owned by later stages, not hidden.
- The real debt is non-functional: heavy duplication across the adapter crates, a huge comment-to-code ratio of dated amendment prose, and a few "inert by decision" behaviours (SQLite `ByRun`, Node verbs).

## 1. Module and public surface map

### rk-adapter-toolchain-rust (src 4,483 lines, tests 5,757, 11 src modules)

- `lib.rs` (840): `Config`, `ConfigError`, `RunnerPolicy`/`Runner`, `RustToolchain`, and `impl Toolchain` with seven methods (`crates/rk-adapter-toolchain-rust/src/lib.rs:623-840`).
  - `detect`, `build`, `test`, `lint`, `classify`, `change_summary`, `run_verb`.
- `report.rs` (812): the `test` body, runner selection, argv builders, affected plan. `libtest_lines.rs` (390) and `nextest_lines.rs` (298): strict, version-pinned output parsers.
- `verbs.rs` (1001): four verb handlers (`smoke`, `sca_scan`, `package`, `setup`; handlers at lines 196, 642, 861, 940), an in-tree CVSS v3 scorer (`cvss3_tenths`, line 382), and CVSS v4 scoring via the `cvss =2.2.0` crate.
- `affected.rs` (265): the pure reverse-dependency-closure function over `cargo metadata`.
- `build.rs` (114): reads cargo's `--message-format=json` stream so a compile error is `Ok(failing)`.
- `classify.rs` (138): a path-and-bytes rule that spawns nothing.
- `registry.rs` (283): the dated capability rows.
- `run.rs` (232): the one `Command` construction site, handed to `rk_containment::run_bounded`.
- `env.rs` (110): child-environment denylist and required names.

### rk-adapter-toolchain-node (src 1,637, tests 1,989)

- `lib.rs` (541): `NodeToolchain` (`tsc --noEmit` build, `vitest` test, `prettier --check` lint).
- `report.rs` (436): vitest JSON-reporter counts. `classify.rs` (105), `env.rs` (120), `registry.rs` (218), `run.rs` (217).

### rk-adapter-sqlite (src 856, tests 1,806)

- `lib.rs` (429): `SqliteStore` implementing the five-method `EventStore` (`append`, `append_tier1`, `query`, `snapshot`, `rebuild`), plus `open`, `open_at`, `pragmas`.
- `identity.rs` (121): root-commit identity. `path.rs` (67): common-dir path. `registry.rs` (239): the dated capability rows.
- It is a thin adapter. SQL, schema, migrations and WAL live in `rk-store` (`rk_store::events`, `tier_one`, `migrate`, `conn`), outside this audit area.

## 2. What Phase 1 requires of this area

Requirements come from the design (`references/2026-09-08-architecture-design.md:246`, `:269`) and ADR-0094 d1: SQLite is the real `EventStore`, and Node and Rust are the two real `Toolchain` adapters. The Rust adapter exists because Phase 1's self-hosting run is on a Cargo workspace.

Stage s8c (`runbook/phase-1/s8c-toolchain-port-and-verbs.md`) raised the port to outcomes (record T = ADR-0169), step bounds (record B = ADR-0171) and a seventh `run_verb` method (record V = ADR-0170). The tracker row (`runbook/PROGRESS.md:42`) says:

- Checkpoint 8c passed at `4ab7c67` and re-ran green after flip commit `e8e4f68`.
- T, V and B are accepted (D21-69, D21-70, D21-71).
- Seven refuting rounds, round 7 dry, mutants M1-M54 killed.
- The checkpoint is provisional, with a whole re-run pending `s8j`.

SQLite additions came in s8e (`append_tier1` per ADR-0181, interim item key per ADR-0182, `rebuild`).

## 3. Implemented versus fake, stubbed, armed or deferred

### Genuinely implemented and exercised (CODE and TESTS agree)

**Rust adapter**
- Real `cargo build`, `cargo test` or `cargo nextest run` (chosen by a construction-time probe), and `cargo clippy` through `run_bounded` (`lib.rs:605-690`, `report.rs:732`).
- `min(step, construction)` bound with a `Timeout` that names the bound (`lib.rs:348`; tests `tests/it/bound.rs`, 2 tests).
- Count and tally from pinned parsers with six goldens (libtest 1.98.1 and nextest 0.9.146, including a 1,053-line golden of this repository's own stdout). Unpinned versions and disagreeing summaries are refused by name (`tests/it/libtest_lines.rs`, `nextest_lines.rs`).
- `change_summary` `Text` arm via `rk_git::diff::range_diff`. Binary and LFS-pointer paths are refused through the dated registry row (`lib.rs:728-837`; 5 tests in `tests/it/change_summary.rs`).
- Affected selection: a pure function plus a real `cargo metadata` read, with a workspace fixture a <- b <- c (`affected.rs`; `tests/it/affected.rs`, 5 tests).
- Verbs run real `cargo audit`, `cargo deny`, `cargo build --release --locked` and `pnpm install` against fixture databases and workspaces (`tests/it/verbs.rs`, 18 tests).
  - Refusals by name: `database-moved-mid-scan`, `cvss-unscorable`, `deny-db-path-line`, `package-binary-outside-target`, among others.
  - The in-tree CVSS v3 scorer is pinned against `cvss::v3` over 92 real RustSec vectors (`tests/fixtures/cvss/v3-vectors.txt`).
- CI installs cargo-audit 0.22.2, cargo-deny 0.20.2 and cargo-nextest 0.9.146 so these tests run on both ubuntu and windows (`.github/workflows/ci.yml:28,67-79`).

**Node adapter**
- Real `tsc --noEmit` build with exit-code mapping, real vitest run with JSON-reporter counts, `prettier --check` lint, an S-CANCEL-WIN vitest arm (`tests/it/cancel.rs`), an AC-325 argv-wrapper fixture, and a conformance row.

**SQLite adapter**
- One store per common dir, append-only with correction by append (AC-364), file recreation (AC-365), typed append errors (AC-368), identity-mismatch refusal (AC-369), common-dir location (AC-370), gate replay over the real store (AC-243), WAL and busy-timeout checks, and an `append_tier1` single-transaction test.
- `rebuild` is differentially checked against `rk-cli`'s doctor in the registry row's recorded probe. `tests/it/conformance.rs` runs the shared conformance suite including the tier-one arm.

### Deferred, armed or refused (all named in the records)

- **Binary arm of `change_summary` is sealed until S-BIN (s8i).**
  - The registry row is `probe: none` with reason ADR-0165 (`registry.rs:160-162`). That is correct for now, and s8i (B1, in progress elsewhere) is the gate.
- **Node is deliberately thin.**
  - `change_summary` is refused from the registry (`lib.rs` Node, `change_summary`).
  - `Selection::Affected` is refused, and `select_affected` always returns the registry refusal. The probe is owed as S-SEL-NODE.
  - `Selection::Named` is refused.
  - `run_verb` refuses every verb by bare name, `setup` included (`lib.rs` Node, `run_verb`; test `tests/it/verbs.rs`).
  - `total_test_count` is the run's own executed count (`OwnExecutedCount`). With a `Filter` selection, a shrunken run cannot be detected by this adapter. The records say AC-326's `selection_ineffective` compare is the kernel's job (`report.rs` Node doc, lines 26-36).
- **AC-334 Rust half is ARMED.**
  - No Rust ordering-inversion fixture exists. The `corpus` feature is declared empty so `--features corpus` exits 0 (`rk-adapter-toolchain-rust/Cargo.toml`, comment above `corpus = []`). The Node half runs behind `--features corpus` (`tests/it/ac334.rs`).
- **SQLite `ByRun` is inert.**
  - It reads every row (`lib.rs:383`), per ADR-0182 decision 5 (accepted). Callers use it as the "whole store" read: `rk-cli` `status.rs:131`, `cost.rs:186`, `doctor/mod.rs:529`.
  - It works, but the name misleads and the real run filter does not exist. ADR-0131's tag-set selector is the eventual home.
- **R-23 is OPEN** (`runbook/PROGRESS.md:179`).
  - Identity compares root commit and schema version only. It takes the first line of `git log --max-parents=0` (`identity.rs`, `root_commit`), so a repository with several root commits can answer differently per branch. No moved-repository policy and no harness-version column exist.
- **Whole re-run of Checkpoints 8c and 7c is pending `s8j`.** They are provisional by the project's own convention, and part of the 7c re-run needs a paid probe.
- **Unverified here.** Whether the full 877-test companion suite is green on the current head. I did not execute anything, and the last recorded green is `4ab7c67` / `e8e4f68`, with later commits having touched `rk-adapter-toolchain-rust` (`5d590c3`, 2026-09-29).

## 4. Code quality

### Structure and error design (strong)

- Modules are small and single-purpose. The exception is `verbs.rs`, 1,001 lines, in which `sca_scan` alone is about 200 lines (`verbs.rs:642-845`). It should be split (database checks, tool runs, merge and banding).
- The error model is consistent and fail-closed:
  - `thiserror` everywhere, no `anyhow`.
  - Typed `StepError` with a closed `TransportKind`.
  - A killed run is never a failing test.
  - A missing program is `Transport::Spawn`, never `NonZero`.
  - Refusals name the capability or verb.
  - A parse disagreement is a decode failure, never a guessed verdict.
- The registry design is careful. A row that claims a probe for an unimplemented method, or a missing row, is a decode failure and never defaults to `Unsupported`.
- SQLite flattens every `rk-store` failure into `TransportKind::Io` via `e.to_string()` (`lib.rs:341-386`). Callers cannot tell lock contention, a corrupt file or a decode error apart, and the original error type is lost.

### Escape hatches in NON-TEST code (census qualified)

The census figures for these three crates are doc-comment hits, not code:
- `unwrap`/`expect`/`panic`/`todo`: 0 in all three.
- `unsafe`: 1 each in the Rust and Node crates, but the only hits are prose lines saying "no `unsafe`" (`lib.rs:126`, `lib.rs:79`). The workspace denies `unsafe_code` (`Cargo.toml:97`). Real count is 0.
- `allow`/`expect` attributes: the real sites are six `#[expect(..., reason = "...")]`, each with a stated reason.
  - Rust: `run.rs:172` and `:180` (two `Instant::now` sites bounding `wall_secs`), and `run.rs:209` (the single `std::process::Command` site).
  - Node: `run.rs:160`, `:168`, `:196`.
  - SQLite: zero real sites. The lone census hit is a doc line.
- No `todo!`, `unimplemented!` or dead stubs found. `filter_map(Result::ok)` in `sca_scan` silently drops unreadable directory entries (`verbs.rs`, deny db-path listing). That is minor, and the single-child check still fires.

### Duplication and dead weight

- `enum Capability`, `Probe`, `Row`, `refusal` and `answer` are copied across four adapter crates (rust, node, sqlite and `rk-adapter-claude`).
  - The SQLite and Rust copies share variants that cannot apply (for SQLite: `Dispatch`, `ChangeSummary`, `AffectedSelection`).
- Rust and Node `run.rs` differ by about 31 lines out of about 150 code lines, and `env.rs` is near-identical.
- Each copy must be amended in lockstep when the dated-row rules change. That is a real maintenance cost, apparently chosen to keep each adapter self-contained.
- Rust `classify.rs` and Node `classify.rs` apply one shared rule per ADR-0029 (i), also copied.
- The in-tree CVSS v3 scorer duplicates the `cvss` crate's v3 scorer. It is justified by the differential test, but it is extra code to keep correct.

### Comment-to-code noise (high)

- Comment lines (`//`, `///`, `//!`) in src:
  - Rust: about 1,241 of 4,483 (28%).
  - Node: about 599 of 1,637 (37%).
  - SQLite: about 418 of 856 (49%).
- `rk-adapter-toolchain-rust/src/lib.rs` is 327 comment lines of 840. Its module doc accumulates dated amendments, struck-through (`~~...~~`) sentences and "corrected 2026-09-26" overlays. A reader has to reconcile five layers of history to learn that `change_summary` now reads the range (`lib.rs:1-143`).
- `rk-adapter-sqlite/src/lib.rs` carries the same pattern, including a struck-through sentence and the stale paragraph that the dated note supersedes (`lib.rs:46-75`). The Node `report.rs` doc still says a red suite is `Err(NonZero)` (RT-22, line 37), which record T reversed. The Rust crate's equivalent was amended, but Node's was not re-dated.
- This is accurate history but poor present-tense documentation. It is the main quality drag in the area.

### Test depth

- Rust tests: about 72 `#[test]` functions in 14 files, plus table-driven cases (the census says 81 test fns).
- Node tests: 19 (census); SQLite: 16 (census; about 17 by `#[test]` count).
- Test-to-source line ratio is 1.3 (Rust), 1.2 (Node) and 2.1 (SQLite).
- Almost everything is integration against real processes (`cargo`, `vitest`, `tsc`, `cargo audit`/`deny`, git) plus golden-fixture parser tests. There are no `cfg(test)` unit modules in src (grep: none). Pure logic (affected closure, CVSS, parsers, registry decode) is exercised through the public API, so it is tested, but through slow, environment-heavy harnesses.
- Environment dependence: tests need cargo-audit, cargo-deny, cargo-nextest, pnpm and a vitest install. Missing tools fail loudly. Node `support::preflight` refuses to run without `node_modules/.bin/vitest` (`tests/it/main.rs` doc). CI provisions these, but a bare developer box cannot run the suite.
- Parsers are pinned to a single toolchain version (libtest 1.98.1, nextest 0.9.146) with one golden directory each. This is by design and fail-closed, but every cargo or nextest upgrade needs a re-measure and a new golden.
- The mutation-testing claims (M1-M54 killed, a leaked mutant caught in round 5) are record claims. I could not verify them without running the harness.

## 5. Risks

1. **Toolchain-version brittleness.** Parsers refuse unpinned libtest and nextest versions by name. A routine toolchain bump fails the Rust adapter closed until goldens are re-measured. The pin is in `rust-toolchain.toml` and CI, so it is controlled, but it is a recurring chore and a self-hosting-run hazard if the run box drifts.
2. **Binary arm unproven.** The one design question these crates cannot answer is what `change_summary` returns for non-text changes. It is gated by S-BIN (s8i), now running. Whatever it finds will need a new arm, a new registry row and probably new goldens in the Rust adapter (and Node, if Node ever needs it).
3. **Node adapter would not survive a real Node self-host.** Node has no affected selection, no verbs, no `setup`, and cannot detect a shrunken filtered run. This is fine while Phase 1 self-hosts on Rust, but Node is weaker than "real adapter" suggests. It is a second adapter proving the port is multi-implementation, not a second usable lane.
4. **Environment-measured Windows assumptions.** `REQUIRED_CHILD_ENV` (`SystemDrive`, `TMP`/`TEMP`/`USERPROFILE`) was measured on one box, and the Windows `.cmd` shims and drive-letter paths are special-cased. CI runs a ubuntu-and-windows matrix, which mitigates this.
5. **SQLite semantics limits.** `ByRun` returns everything, so any caller assuming a run filter gets the whole journal. Root-commit identity is branch-dependent (R-23), and an operator who moves or re-clones the repo gets a refusal rather than a managed migration. Error flattening to `Io` hides store failure classes from the kernel.
6. **Duplication drift.** Four copies of the registry module and two of the run and env plumbing can drift apart. Today a comment-driven discipline ("amended in lockstep") holds them together.
7. **Provisional checkpoints.** 8c and 7c are provisional until `s8j`'s whole re-run. Until then, "green" for these crates rests on `4ab7c67` / `e8e4f68` plus per-slice suites.

## 6. Evidence index

- Tracker rows: `runbook/PROGRESS.md:33` (7c), `:42` (8c), `:179` (R-23), `:229-238` (D21-62..D21-71).
- Design: `references/2026-09-08-architecture-design.md:246` (four real adapters), `:269` (Toolchain port row).
- ADRs (all `status: accepted`): 0094, 0127, 0165, 0181, 0182 (`references/adr/`).
- Wiring: `crates/rk-cli/src/verbs/run.rs:390` (SQLite open), `:5535` (RustToolchain), `:5564` (NodeToolchain).
- CI: `.github/workflows/ci.yml:28` (os matrix), `:67-79` (audit tooling and nextest installs).
- Commit history for these crates: 19 commits from `a52c0af` (SQLite slice 3) and `729a8ab` (toolchains slice 6) through `5d590c3` (s8g, 2026-09-29).

## 7. Unverified

- No test was executed (cargo forbidden).
- Current-head greenness of the three crates' test suites.
- The 877/0/9 companion-suite figure and M1-M54 mutant-kill claims (record claims only).
- Whether the Node `project/` fixture install and `pnpm` are present on the target run box.
- Behaviour of `cargo audit`/`cargo deny` at versions other than 0.22.2 / 0.20.2.
- Cross-process SQLite contention. The tests live in `rk-store` and `rk-kernel`, not read here.
