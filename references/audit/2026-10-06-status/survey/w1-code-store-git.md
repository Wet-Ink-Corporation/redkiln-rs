# W1 audit: `crates/rk-store` and `crates/rk-git`

Snapshot: worktree `audit-wt`, commit `36dd8ac` (branch `build/phase-0-to-1`). Read-only; cargo was not run, so no test result below is re-measured. "Record" means a runbook, ADR or tracker statement. "Code" means what I read in the snapshot. Paths are relative to the worktree.

## 0. Verdict

**Overall grade B. rk-git is B+/A- (strong); rk-store is B (solid, slightly more debt). Completeness against what Phase 1 asks of this area: about 85%.**

The tier-1 git store is real, wired into the CLI and kernel, and unusually well tested. The one-wrapper rule, the closed `GitOp` write set, `Reconciled<State>`, the `\\?\` normaliser, the trailer write path, the tree-hash derivation, the landing, run-worktree lifecycle and the store's render/write/read path are all implemented and exercised. The production escape-hatch count is effectively zero.

The shortfalls:
- **The whole orphaned-anchor / rewrite-event subsystem is absent.** AC-167, 169, 170, 171 and 190 have no producer, no test and no stage file that schedules them.
- **The "gix owns the read path" decision is only partly built.** `reconcile` reads history by spawning `git log`, and the common dir is resolved by spawning `git rev-parse`. gix is used only for the range diff, the HEAD read and the review clone.
- **`GitError` has no `Display` or `Error` impl**, so operator-facing errors print `Debug` with stderr as raw byte arrays.
- **A few latent defects** sit at guard boundaries (section 5).
- **Two doctor-facing helpers in rk-store are dead code** because rk-cli re-implemented them, in worse form.
- **Documentation noise** is high: stale module docs and heavy provenance narrative inside source comments.

## 1. Module map and public surface

### rk-git (21 src files, 7,614 lines; tests 8,076 lines in 17 modules, 159 `#[test]`; 3 trybuild/UI fixtures)

| Module | Lines | Surface |
| --- | --- | --- |
| `git_exec` | 560 | The single wrapper. `GitExec::run(GitOp, cwd, args)`. `GitOp` is a closed 29-variant enum (`ALL`, `is_write`). The wrapper supplies the `-c core.hooksPath=<empty>` and `trailer.ifexists=add` prelude and sets `LC_ALL=C` on every child. One `std::process::Command` site (`spawn`, `git_exec.rs:484-560`), under a scoped `#[expect]`. |
| `version` | 104 | `assert_at_startup` mints `GitToolchain`; floor `>= 2.20` (ADR-0091 decision 4). `GitExec::new` requires the token. |
| `path` | 73 | `WorktreePath::new` normalises to drive-letter form and refuses `\\?\UNC\`. The type is the only way to reach an argv. |
| `error` | 105 | `GitError`, 11 variants, closed. |
| `trailer::grammar` and `trailer::write` | 491 and 138 | `parse_body`, `ParsedTrailer`, `Verdict`, `GateState`, `StoryStage`, `UnknownKeyCounts`; `commit_with_trailers`, `interpret_trailers_in_place`. Each exists as a free function and as a `GitOps` method. |
| `reconcile` | 332 | `read_commits` (via `git log --all --reverse`) and `reconcile(&[CommitTrailers]) -> (State, Vec<Drift>)`. The fold covers Story (with stage), Slice-Verdict and Gate (kind/reach/state/history). Lifted from the corpus at `27d97e9b`. |
| `reconciled` | 95 | `Reconciled<T>` with a private field; `Entry {Run, Answer, Doctor}` (`status` deliberately absent); `reconcile_at_entry`. |
| `gitops` | 278 | `GitOps`, a struct and not a trait. `Mode::{Live, Replay}`, `GitJournal`, `VecJournal`, `GitInvocation` (argv digest only, no env field), `writes_performed`. In replay the wrapper is unreachable. |
| `tree` | 246 | The one `tree_hash` derivation: `CheckpointCommit`, `WorkingState`, `WorkingStateLess`. It uses a temporary index under `<common>/redkiln/tmp` and `add -u`, so untracked files are excluded by construction. |
| `landing` | 791 | `stage` and `commit` (ADR-0188): staging set, 17-entry denylist, 2 MiB blob cap, `commit-tree`, re-read of the tree, `update-ref` with the old value, `read-tree` resync. All of it runs through `GitOps`. |
| `worktree` | 2,883 | `authorise_tier1_write` and `OwnedWorktree` (AC-529). Run-tree lifecycle: `ItemId`, `derive`, `prepare_creation`, the refusals (linked segment, case-fold collision, unignored root, past path limit), `enter`/`enter_with_baseline` (reuse on re-entry), `lock_reason`, `release`, `remove_landed`, `roll_back_fresh`, `RkBase`, `check_filters`, `scan_for_pointers`. |
| `worktree_include` | 343 | The `.worktreeinclude` carry, refused whole on any escape. |
| `diff` | 711 | gix-based `range_diff`: per-path change kind, LFS-pointer and binary marks, missing final newline. |
| `clone` | 122 | The gix shallow-by-SHA review clone; bare unless a working tree is declared; `remove_review_clone`. |
| `head` | 71 | gix `head_commit`, opened at the path and never discovered upward. |
| `release` and `mutation` | 199 and 33 | `tag_release` (annotated local tag on the run branch only); `apply` of a mutation patch (verify-pass only). |

### rk-store (21 src files plus 2 SQL migrations, 4,745 lines; tests 5,903 lines in 11 modules, 98 `#[test]`; 3 trybuild fixtures plus a checked-in v1 SQLite fixture)

| Module | Lines | Surface |
| --- | --- | --- |
| `conn` | 212 | `Writer` (one `Mutex<Connection>`, `transaction`), `Reader`, `StoreError`, `BUSY_TIMEOUT_MS = 5000`, WAL check, `looks_like_network_filesystem`. |
| `identity` and `migrate` | 146 and 72 | A root-commit/common-dir/schema stamp. `establish` reads the stamp, refuses a foreign or other-version store, applies migrations tracked by `PRAGMA user_version`, and stamps a fresh store, all in one transaction. |
| `schema_lint` | 177 | AC-166 declared-key-domain lint over the `.sql`/`.keys` pairs, run as a test. |
| `tier` | 108 | `tiered_state!`: a field with no tier is a compile error. |
| `tier_one` and `events` | 405 and 98 | The only SQL against `t1_*` tables. Append-only inserts for stage transition, gate transition and slice seal; `stored_rows`; `rebuild`; `t2_event` insert and selector. |
| `canon` and `memo` | 131 and 144 | The canonical serialiser (sorted keys, LF, floats refused). The 8-component `StepKeyComponents` memo key, `activity_key`, `IdempotencyKey`. |
| `locator` | 252 | The only module that reads markdown; returns byte ranges. |
| `render` | 1,512 | Layout-C renderer, `ArtifactWriter` (`write`, `write_keyed`, `restore`, replay mode), `reconcile_entry`, `move_to_prose`, `ArtifactReader`, the `Reconciled` and `Drift` records. |
| `vouched` | 255 | AC-127 vouched-root store and advisory pass. |
| `item_write` and `backlog` | 354 and 213 | The store-route `item_create` write (a hand-rolled closed-subset TOML emitter) and the `toolchain:backlog` producer, which reads exact paths named by store rows. |
| `retention` | 76 | Re-exports `Retained` and `DispatchRecord` from rk-types; `looks_like_credential`. |
| `artifacts/{finding_identity,sink,ledger,verify_report}` | about 500 | AC-305 finding identity, AC-310/530 sink accounting folds, AC-333 ledger completeness, AC-329 whole-suite record. |

**Dependency discipline holds.** rk-store depends only on rk-types, serde and rusqlite, and has no edge to rk-git. The render path receives git reads as injected closures (`render.rs:504-508`). rk-git depends on rk-types and gix. Neither carries `anyhow` or tokio (`crates/*/Cargo.toml`).

**The surface is wired into production, not orphaned.** Callers outside the two crates:
- `rk-cli/src/verbs/run.rs` uses `item_write`, `reconcile_entry`, `move_to_prose`, `vouched_pass`, `backlog`, `ArtifactWriter` and `ArtifactReader`, plus `roll_back_fresh`, `remove_landed`, `enter_with_baseline` and `worktree_include`.
- `rk-kernel/src/runtime/tier_one.rs` uses `landing`, `reconcile` and `GitOps`.
- `rk-cli/src/review_pass.rs` uses `prepare_review_clone`; `release_tag.rs` uses `tag_release`; `verify_pass.rs` uses `mutation::apply`.
- `rk-adapter-toolchain-rust` uses `diff` and `head_commit`; `rk-cli/src/doctor/rebuild.rs` uses `tier_one::rebuild`.

## 2. What Phase 1 requires of this area (records)

- `runbook/phase-1/s6-store-and-git.md` §6.1-§6.3: the S-GIX arms; the hybrid read/write split; six constraints (one wrapper; stderr never parsed; startup version assert; paths in drive-letter form and re-read after write; the journalled `GitOps` struct; worktrees re-created per run); and `rk-store`'s tier split, three trailer axes, `reconcile()` at every entry, and ADR-0105 clause 7's connection contract. The red-first set is the S-GIX arms, `trailer_roundtrip_matches_interpret_trailers`, `reconcile_runs_at_every_entry` (as a trybuild fixture) and `git_paths_are_drive_letter_form`.
- Later stages widened the area, and their stage files carry the specifics:
  - `s8c`: `.worktreeinclude` and `head`.
  - `s8d-ii`: run worktrees (ADR-0180).
  - `s8e`: tier-1 writes git-first (ADR-0181/0183/0184/0185), the `Slice-Verdict` axis, migration 0002.
  - `s8f`: landing (ADR-0188), and the render/write/read path (ADR-0189).
  - `s8g`: `GitOp::Tag` and `GitOp::Apply` (ADR-0193), and the keyed write (ADR-0191).
- The tracker's PROGRESS row for Step 6 records that Checkpoint 6 was re-run whole and passed at `4777941` and again at `d5bfc4f` (2026-10-02). Record claim, not re-measured here. Later changes to these crates were two lines in `crates/rk-store/tests/it/store.rs` (commit `55fda45`).
- ADR-0077, 0091, 0097, 0098, 0105, 0109, 0115, 0116, 0118, 0180-0185, 0188, 0189, 0191 and 0193 are all `accepted` at this snapshot (checked from their frontmatter).
- The merge queue is explicitly out of Phase 1: `runbook/phase-1/s8f-landing-and-rendering.md:632-640` (`GitOp::Integrate` keeps no caller; AC-196 and AC-197 route to Phase 2).

## 3. Genuinely implemented and exercised (code plus tests)

1. **The wrapper and its closed set.** `GitOp` is the argv gate (`git_exec.rs:30-110`); `GitExec::new` demands a `GitToolchain` token (`git_exec.rs:14-16`). Tests: `version.rs` (7), `gitops.rs` (9, including `the_write_set_has_no_push`, `every_git_child_runs_with_lc_all_c`, `gitops_is_a_struct_not_a_port_trait`). Workspace-wide, no `Command::new("git")` exists outside rk-git (grep: other `Command` sites are the Claude runner, migrate tests, and lint-control fixtures).
2. **Trailer write path.** Both the free functions and the `GitOps` methods route through the wrapper with `--if-exists=add`; the HEAD is re-read after the commit (`trailer/write.rs:55-79`). `tests/it/trailers.rs` (16) includes the discriminating duplicate-token case, hook-isolation, and a measured control that git drops the duplicate without the flag.
3. **Trailer grammar.** Three axes plus the s8e widening; legacy and malformed values are surfaced as typed drift (`grammar.rs`; `tests/it/grammar.rs`, 13 tests; `rk_git_emits_no_legacy_keys`).
4. **`reconcile`.** The fold is deterministic and typed. `reconcile_is_idempotent_and_adds_no_drift_on_a_second_run` and `reconcile_mutates_nothing_in_git` pin it. `Reconciled<T>` is constructible only through the sanctioned route (`reconciled.rs:22-95`) and backed by a trybuild must-fail (`tests/ui/reconciled_cannot_be_constructed.rs`). An F12 corpus test sits behind `--features corpus` (`corpus_f12.rs:19`); the record says it passed at Checkpoint 6, and I could not re-run it.
5. **Path normaliser.** It refuses `\\?\UNC\` instead of mangling it (`path.rs:42-62`). `tests/it/paths.rs` has an end-to-end wrapper test (`wrapper_accepts_a_canonicalised_path_end_to_end`).
6. **Tree hash and landing.** The temporary index lives under the common dir. A single write-tree call site is asserted by test (`there_is_exactly_one_write_tree_call_site`). The landing re-reads the new commit's tree and moves the ref with the old value (`landing.rs:690-791`); tests cover denylist, size limit, replay with zero writes, and hook isolation (`tests/it/landing.rs`, 695 lines).
7. **Run worktrees.** Ownership is a parameter and never cwd. Refusals are typed (`WorktreeRefusal`, `KeepReason`, `Removal`). `tests/it/worktree.rs` is 2,965 lines and 57 tests, with Windows and non-Windows arms (`worktree.rs:873-880, 1984-1991`).
8. **Review clone and diff.** The review clone is a gix shallow clone and is bare unless a working tree is declared (`clone.rs:38-90`). Range diff has binary/LFS/newline tests (`tests/it/diff.rs`, 940 lines).
9. **S-GIX arms.** The arms are `#[ignore]` by design, assert their verdicts against ADR-0091, and run inside Checkpoint 6 per the record.
10. **Store connection contract.** The WAL and busy-timeout settings are asserted. AC-181 is a genuine two-process contention test, not a thread near-oracle (`tests/it/contention.rs:1-63, 86`). Poison halts rather than recovers: the L3 violation in the tracker (`.unwrap_or_else(PoisonError::into_inner)`) has been repaired (`conn.rs:150`, `.expect(LOCK_HALTS)`, ADR-0118), with `writer_poison_is_a_halt_and_never_recovered`.
11. **Identity and migration.** Foreign root commit refused with the file unchanged; older and newer schema refused; a checked-in v1 store is refused with its bytes unchanged (`tests/it/store.rs`, 917 lines and 28 tests; `tests/fixtures/store-v1.sqlite3`).
12. **Tier machinery.** `tiered_state!` makes an undeclared tier a compile error (`tests/ui/tier_missing.rs`); a test asserts no DELETE against a `t1_*` table (AC-168); `schema_lint` passes and has failing controls.
13. **Render, write, read path.** Layout C, one `artifact.write` per write, replay performs zero writes, structured drift is surfaced then restored, prose edits are accepted, `move-to-prose` reads from the causing commit, reads halt on digest mismatch, and `ac122_sibling_write.rs` is a trybuild fixture. `tests/it/render.rs` is 2,498 lines and 26 tests.
14. **Canonical serialiser and memo.** The key is a typed 8-field struct; floats are refused; three distinct key types share no code (`canon.rs`, `memo.rs`; `canon_and_memo.rs`, 13 tests).

Test posture overall: integration tests carry about 1.06x (rk-git) and 1.24x (rk-store) the source line count. Nearly every test group has a named "control" test that proves the instrument can fail. The tests are mostly real-git or real-SQLite integration tests; there are almost no unit tests inside `src/`.

## 4. Fake-only, stubbed, deferred, armed, or missing

**Missing (record requires it, code does not have it)**
- **Orphaned anchors and rewrite events (AC-167, 169, 170, 171, 190; SM-050).**
  - The design specifies `rewrite(old_sha -> new_sha, mechanism)` events and orphan reporting (`references/2026-09-08-architecture-design.md:745-756`; AC-169 at `references/2026-09-08-acceptance-criteria.md:3593`).
  - The code has only the declared shapes: `EventKind::Rewrite` is matched in three kernel `match` arms (`rk-kernel/src/workflow/fsm.rs:1730`, `memo.rs:753`, `breaker.rs:635`) and `Drift::OrphanedAnchor` exists in `rk-types/src/drift.rs:93`. Nothing emits either.
  - `GitOp::PatchId` is an enum variant with zero callers.
  - AC-169, 170 and 190 have zero mentions in any crate, and no runbook stage file mentions them (grep over `runbook/`, `spikes/`, ADR 0105/0109/0181). I could not find a disposition that defers them. This is not the same as the merge-queue deferral, which is explicit.
  - The tier-1 rows do store an anchor (`tier_one.rs` `Anchor`), so the data is there; the detector is not.
- **AC-136 (read of an older `schema_version` artifact).** No mention in code. Unverified whether covered elsewhere.

**Deferred by record (acceptable)**
- Merge queue and `GitOp::Integrate`, which has no caller (AC-196, AC-197; `s8f-landing-and-rendering.md:632-640`). The `rk-git/src/lib.rs` header still advertises "the merge queue".
- `Reconciled`/gate machinery for orphans (AC-171) is not built, same cause as above.

**Armed or ignored tests**
- The three S-GIX arms are `#[ignore]` by design (`s_gix_a.rs:71`, `s_gix_b.rs:103`, `s_gix_c.rs:87`) and run inside the checkpoint. The corpus F12 test needs `--features corpus` (`corpus_f12.rs:19`) and a box holding the pinned corpus.
- Cross-platform behaviour: the records at the S6 close said CI-runner conformance (`ci.yml` unexecuted), the arms on Linux/macOS, and arm c on any transport but `file://` were not measured. I found no later record that clears this; **unverified** (see section 7). The code has `cfg(unix)`/`cfg(not(windows))` branches (`worktree.rs:880, 1991`, `worktree_include.rs:247`, `tests/it/trailers.rs:167, 214`, `tests/it/worktree.rs:604`) that a Windows-only run never executes.

**Partially built against the record**
- **gix read path.** S-GIX/a returned *go* (ADR-0091) and the runbook says gix owns the common-dir read. In code the common dir is resolved by spawning `git rev-parse --git-common-dir` (`landing.rs:397-398`, `worktree.rs:1301-1305`), and the CLI resolves it through `rk_adapter_sqlite::path::common_dir`, which joins git's relative answer onto the worktree (`rk-cli/src/repo.rs:12-17`). gix reads the common dir only in the `s_gix_a` test (`tests/it/s_gix_a.rs:106`). Also, `reconcile::read_commits` spawns `git log --all --reverse` and parses with the crate's own grammar; neither gix `BodyRef::trailers()` nor gix history walking is used (`reconcile.rs:290-332`). gix appears only in `diff.rs`, `head.rs` and `clone.rs`. The `GitOp` doc comments treat these reads as intended, and `GitOp` includes them as reads, but I found no successor record that formally narrows ADR-0077's "gix owns the read path". The ERRATA entry (`references/adr/ERRATA.md:89`) corrects evidence lines and does not narrow it.
- **Review-clone removal is not wired.** `rk_git::clone::remove_review_clone` has zero production callers. `rk-cli/src/review_pass.rs:365` uses `std::fs::remove_dir_all` directly, and only on a stale leftover. Clones are otherwise kept by sha. Inside `remove_review_clone`, the "journalled" `GitOp::CloneRemove` call is `rev-parse --git-dir` with its result ignored (`clone.rs:113`), a ceremonial op that records a git invocation without doing the removal.
- **Render coverage is one kind.** `TEMPLATES` holds a single human-facing template, `Spec` (`render.rs:87-92`). That follows ADR-0006/0075 per the dated note, and every other kind is machine-only.
- **Dead helpers in rk-store.** `retention::looks_like_credential` (11 prefixes, entropy rule, tested) and `conn::looks_like_network_filesystem` have no production callers. `rk-cli/src/doctor/rules.rs:1304-1311` and `:1334-1342` re-implement both with weaker rules: `credential_retained` checks only `sk-ant-` and `Bearer `, and the network-fs check treats `/Volumes/` (every macOS external disk) as a network mount. `artifacts::ledger::validate_ledger` (AC-333's persisted-ledger completeness check) and `file_digest` also have no non-test caller; `rk-kernel/src/verify/ledger.rs` does its own computation. Whether the kernel enforces AC-333 completeness another way is **unverified**.

## 5. Code quality

**Structure.** Good. One concern per module; the deliberate sub-seams (`GitOps`, `Reconciled`, `OwnedWorktree`, `ManagedDir`'s private write, `WorktreePath`, the `tiered_state!` macro) turn review conventions into compile errors. Closed enums and exhaustive matches are used throughout. Weak spots:
- `worktree.rs` (2,883 lines) is a god-module covering ids, path hazards, locks, re-entry, rollback, filter checks and pointer scans. It should be split into submodules. `render.rs` (1,512) is large but cohesive. The longest real functions are about 140 lines (`landing::stage`, `render::serve`); the larger "functions" my counter found were `Display`/`name` match tables.
- Duplicated logic: `commit_with_trailers` and `interpret_trailers_in_place` each exist twice, once as a free function and once on `GitOps` (`trailer/write.rs:23-138`).
- Two unrelated types are named `Reconciled`: `rk_git::reconciled::Reconciled<T>` and `rk_store::render::Reconciled` (`render.rs:920`). A confusion hazard.

**Error design.** Mixed.
- `GitError` implements neither `Display` nor `std::error::Error` (no `impl ... for GitError` anywhere in the workspace). The CLI prints its `Debug` form: `rk-cli/src/verbs/doctor.rs:115`, `rk-cli/src/repo.rs:47-52` and `doctor/rebuild.rs:143` say so. `NonZeroExit { stderr: Vec<u8> }` therefore renders stderr as a byte-integer list. The records know and defer the repair ("edit a closed crate", `repo.rs:47-52`).
- `StoreError`, `CanonError` and `LocatorError` implement `Display` but not `std::error::Error`.
- ADR-0102 decision 6 puts `thiserror` in every library crate (`Cargo.toml` rows for 4 adapter crates carry it), but rk-git and rk-store hand-write everything and do not use it.
- `StoreError::Sqlite(String)` flattens the rusqlite error, so callers cannot distinguish `SQLITE_BUSY` from corruption.
- Positive: `GitError::is_precondition` keeps the "operator said no vs tool never spoke" distinction (the RK-B0013 shape) explicit, and no code branches on git stderr text.

**Escape hatches in non-test code.** Census says rk-git `panics=2, unsafe=7, allow/expect-attr=1` and rk-store `unwrap/expect=1, allow/expect-attr=1`. Qualified by spot-check:
- `unsafe=7` is a false positive: the hits are the identifiers `UnsafeCharacter` and `UnsafeItemId` in `worktree.rs:279, 416`, plus doc text. **There is no `unsafe` code in either crate**, and the workspace sets `unsafe_code = "deny"`.
- `panics=2` are two `unreachable!` inside `unwrap_or_else`: `clone.rs:44` (`NonZeroU32::new(1)`) and `gitops.rs:273` (a hex digest built by construction). Both are provably safe and avoidable with a const or a typed constructor.
- `expect=1` is `conn.rs:150`, `.expect(rk_types::LOCK_HALTS)` on a poisoned writer mutex. It is deliberate and halts the run by ADR-0118. It is documented under `# Panics`.
- The two `#[expect(clippy::disallowed_types/methods)]` attributes (`git_exec.rs:484`, `conn.rs:142`) are scoped to the single item and carry reasons.
- There is no `todo!`, `unimplemented!`, `dbg!` or `#[allow]` in src.
- Nothing mechanically enforces "no unwrap/expect": the workspace lints (`Cargo.toml` lints table) do not enable `clippy::unwrap_used` or `expect_used`. The cleanliness is discipline, not a gate.

**Silent-coercion spots (not counted by the census).**
- `identity::read` (`identity.rs:67-74`): `.unwrap_or(false)` swallows any error from the `sqlite_master` probe. A busy or corrupt probe reads as "no identity", and `migrate::establish` then treats the store as fresh and skips the AC-179 foreign-repo check (`migrate.rs:58-67`). The later `INSERT OR IGNORE` will not overwrite the stamp, but the guard is bypassed on that path. Narrow, but it sits on a safety-critical refusal.
- `reconcile::read_commits` (`reconcile.rs:308-332`): `String::from_utf8_lossy`, `ct.parse().unwrap_or(0)`, and a silent `continue` on a malformed record chunk. A bad timestamp becomes epoch 0, and a chunk that does not split into four fields is dropped, both without drift.
- `Writer::transaction` (`conn.rs:146-163`): a transaction that outruns `busy_timeout` is committed first and the error is returned afterwards, so the caller sees a failure for a write that landed (retry would duplicate).
- `ArtifactWriter` read helper (`render.rs:479-481`) turns any I/O error into "absent"; `vouched.rs:206` then reads an unreadable root as zero bytes and records it as drift rather than as an error.
- `looks_like_network_filesystem` recognises UNC only and says so (`conn.rs:200-212`).

**Trailer parsing semantics.** `parse_body` treats every column-0 `key: value` line anywhere in a commit body as a trailer candidate (`grammar.rs:229-247`), not only the final trailer paragraph that git and `interpret-trailers` define. A prose line such as `Gate: ...` or `Story: x/y` quoted at column 0 in the middle of a long body (this repo's commit bodies are long) would register as a state transition. The test `only_column_zero_and_exact_case_registers_as_a_trailer` pins indentation and case, not the paragraph rule. The record's grammar is "column 0, case-sensitive", so this matches the record; whether it is the intended semantics is **unverified**.

**Performance.** `reconcile()` reads `git log --all` for the entire history on every `run`, `answer` and `doctor` (`reconcile.rs:290-332`), by design (no skip is allowed). The cost is O(history) per invocation. At 517 commits it is cheap; there is no measurement or budget for a large repo.

**Test depth.**
- Strong: integration over real git and SQLite; trybuild for the type-level criteria (`Reconciled`, tier, sibling write); a real two-process contention test; a checked-in v1 store fixture; a corpus-bound reconcile test; negative "control" tests throughout.
- Gaps: no property tests or fuzzing (`proptest` is a workspace dependency, but neither crate uses it) for `parse_body`, `canon`, `locator`, the TOML emitter or the worktree-path hazards; no insta/golden snapshots for the renderer beyond byte-identity and structural assertions; very few in-module unit tests. Of the 81 ACs labelled to `rk-store` or `rk-git` in the register, 58 are cited somewhere in `crates/`. The 23 with no citation are AC-136, 140, 141, 142, 145, 153, 169, 170, 177, 182, 190, 192, 194, 195, 197, 291, 292, 398, 399, 440, 456, 459 and 460. Several are implemented under another AC or crate (for example 192-195, 291, 292 and 182 relate to the review clone and hooks, which are built and tested). Absence of a citation is not proof of absence; the genuinely missing cluster is 169/170/190 plus AC-167/171.

**Comment-to-code noise.**
- rk-git src: 1,831 comment lines against 5,375 code lines (about 25%). rk-store src: 1,238 against 3,327 (about 27%). Test files are similar.
- The noise is mostly provenance narrative, not explanation:
  - 90 "(dated 20xx ..., package ...)" annotations.
  - `~~struck-through~~` superseded text inside `///` doc comments (for example `render.rs` `write_keyed`, `reconcile.rs` `State::gates`, `worktree.rs:17-25`).
  - Whole paragraphs recounting checkpoint-grep mechanics inside `Cargo.toml` (`crates/rk-git/Cargo.toml:21-34`).
- **Stale docs:**
  - `crates/rk-git/src/lib.rs:1-20` still says "At s6 section 6.1 this crate is a spike harness and nothing more" and lists the worktree lifecycle, `reconcile()` and `GitOps` as "deliberately ABSENT". The crate is 7.6k lines and has all of them.
  - About 66 lines containing "proposed" in src comments (25 files; some may be ordinary prose) refer to ADRs (0180-0185, 0188, 0189, 0191, 0193) that are now `accepted`.
  - `git_exec.rs` module docs describe the write set as ADR-0105 clause 5's, while the enum has grown to 29 ops by supersession in part.
- Process smell: Checkpoint 6 asserts by grepping source and comments (for example the `Cargo.toml` inheritance-line count, a lift-SHA grep in `reconcile.rs`), which is why prose in these files is load-bearing and hard to tidy.

## 6. Risks

1. **Rewrite and orphan handling is unbuilt and unscheduled.** Rebase, amend or force-push of a run ref orphans tier-1 anchors with no detector, no `rewrite` event and no human gate (AC-171). This matters most at self-hosting, where an operator hand-merging `rk-run/<item>` is the documented hand-off (`s8f-landing-and-rendering.md:638`). SM-050 has no instrument.
2. **Operator-facing error quality.** A failed git call surfaces as a `Debug` dump with stderr as a byte-integer array. This will hurt triage during the Step 8 self-hosting run.
3. **The `identity::read` swallow** can bypass the AC-179 foreign-store refusal on a probe error.
4. **Read-path divergence from the accepted design.** ADR-0077/0091's gix read path and "gix owns common-dir resolution" are not what ships. gix is a heavy pinned dependency (`=0.87.1`, `blocking-network-client`) used for three narrow reads, while `reconcile`, `rev-parse` and `worktree list` spawn git. This is not a correctness defect, but the records overstate gix's role and the dependency cost is real. No formal narrowing record exists.
5. **Trailer-anywhere parsing** can promote quoted prose to state.
6. **Cross-platform exposure.** Runner conformance and the Linux/macOS arms are unproven by any record I found; doctor's network-fs rule and `LC_ALL=C`/path handling have only been run on Windows.
7. **Dead duplicate helpers** (credential detector, network-fs detector) mean the tested rk-store implementations are not what doctor runs.
8. **`reconcile` is O(history)** per entry with no budget.
9. **Review clones accumulate** by sha with no production cleanup.
10. **Schema bumps force a rebuild.** There is no in-place migration by design (AC-180), so every schema change requires `doctor --rebuild`; acceptable for Phase 1, a cost later.

## 7. Key facts and unverified items

Key facts (with citations):
- rk-git: 7,614 src lines, 8,076 test lines, 159 tests; rk-store: 4,745 src lines, 5,903 test lines, 98 tests (`escape-hatches.txt` census; line counts from `git ls-files` plus `wc -l`).
- `GitOp::ALL` is 29 (`git_exec.rs:128`); `Integrate` and `PatchId` (lines 42, 44) have zero non-test callers.
- The single process-exec site for git is `git_exec.rs:484-560`; no other `Command::new("git")` exists in production code.
- Real two-process AC-181 test: `rk-store/tests/it/contention.rs`.
- ADR-0091: `S-GIX/a: go`, `/b: no-go`, `/c: go`, floor `git >= 2.20` (`runbook/phase-1/s6-store-and-git.md:59-66`).
- 81 ACs are labelled `rk-store`/`rk-git` in the register; 58 cited in code; 23 uncited (list above).

Unverified:
- No test, clippy or build result was re-run (cargo prohibited). All "passed" statements are the tracker's, not mine.
- Whether CI (`.github/workflows/ci.yml`) has ever executed on a hosted runner, and so whether the Linux/macOS branches pass.
- Whether the corpus-bound F12 test and the three S-GIX arms pass on the current tree (they need `--features corpus` or `--ignored`).
- Whether any record formally narrows ADR-0077's gix read path.
- Whether AC-333's persisted-ledger completeness is enforced elsewhere than `validate_ledger`.
- Whether AC-136, 140-142, 145, 153, 398, 399, 440, 456, 459, 460 are satisfied under other crates (`rk-kernel`, `redkiln-migrate`) or are still owed.
- Whether the "trailer anywhere in the body" behaviour is the intended grammar.
- Performance of `reconcile()` on large histories.
