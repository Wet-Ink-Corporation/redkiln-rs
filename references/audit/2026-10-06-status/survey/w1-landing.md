# W1 - Landing risk audit (redkiln-rs, snapshot 36dd8ac)

Auditor scope: read-only, pinned snapshot worktree only. Evidence is git object-store reads (log, diff, rev-list, cat-file, ls-tree, grep) and file reads inside the snapshot. Nothing was run (no cargo). Claims from RECORDS (runbook, ADRs, commit messages) are labelled as such; claims from CODE or the object store are labelled as such.

## 1. Verdict

- **Mechanically the branch lands cleanly on main** (main has not moved; no conflicts; fast-forwardable; no secrets; no run noise; tracked size goes DOWN by about 13 MB).
- **Programmatically it is not ready to land**: Phase 1 has not exited (8i in progress, 8j not started, Step 8 re-opened), and the runbook defines CI green on the pushed branch as a Checkpoint 8 condition (spikes/DECISIONS-2026-09-21.md:1688, D21-42 14a).
- **The real risk is NOT main. It is divergence from the branch's own remote.** `origin/build/phase-0-to-1` is at `7860e66` and carries 15 commits (64 files, +8,709/-19) that the local branch does not have, and that no runbook record mentions. They include product-code fixes (rk-containment POSIX kill, rk-adapter-claude, rk-cli, rk-git, rk-adapter-toolchain-rust), CI fixes, and a "temper" lint-ratchet adoption. Local HEAD's own `ci.yml` would fail its AC-465 structural step on 39 tracked files until those fixes are merged in.
- completeness_pct (readiness to land) = 45.

## 2. What is on the branch

Merge-base with main: `353ea01` (main HEAD = merge-base; `git rev-list --count HEAD..main` = 0). Branch: 416 commits, 2026-09-11 to 2026-10-05 (daily 1-40 commits), 1,890 files changed, +363,228/-37,874 (`git diff --shortstat main...HEAD`). Status letters: 1,708 A, 118 D, 60 M, 4 R.

Main today holds no code at all: root tree is `.gitignore LICENSE RUNBOOK.md references runbook spikes` and `Cargo.toml` is absent from main. This branch is therefore the first landing of the product itself.

By category (non-binary numstat, `main...HEAD`):

| category | files | lines added | lines deleted | note |
| --- | --- | --- | --- | --- |
| crate src + manifests | 260 | 101,717 | 0 | 12 crates (redkiln-migrate, rk-adapter-claude, rk-adapter-sqlite, rk-adapter-toolchain-node, rk-adapter-toolchain-rust, rk-cli, rk-containment, rk-git, rk-kernel, rk-ports, rk-store, rk-types) |
| crate tests (.rs) | 296 | 136,892 | 0 | more test code than source |
| fixtures (crate tests/fixtures, tests/fixtures) | 829 | 23,160 | 0 | 59 .jsonl + 56 .ndjson at HEAD, all small |
| tests/ui snapshots | 48 | 1,695 | 0 | |
| `.redkiln/` product catalog | 136 | 9,979 | 0 | the product's own cards/checklists/packs/kb, tracked since Step 7e |
| references/adr | 102 | 20,236 | 74 | ADR set goes 115 -> 200 files at HEAD (`git ls-files references/adr` = 200 vs main 115) |
| references other | 5 | 2,189 | 225 | design, AC register, SM register edits |
| runbook | 41 | 47,837 | 732 | stage files 190-490 KB each; PROGRESS.md is 601 KB |
| spikes other | 40 | 14,586 | 114 | DECISIONS-2026-09-21.md (593 KB), METHOD-S.md, S8 packet |
| spikes/results + runner + corpus | 122 | 12 | 36,726 | pure removal: 97 results files, 19 runner files, 1 corpus pin |
| .github | 1 | 580 | 0 | one workflow, ci.yml (39,831 bytes) |
| root and tools | 9 | 4,345 | 3 | Cargo.toml/lock, clippy, deny, rust-toolchain, CORPUS_PIN, tools/live-run.sh |

Rust at HEAD: 238,046 lines (`git ls-files '*.rs' | cat | wc -l`), 248 src files, 336 test files. Note this matches the 238k figure in the brief.

## 3. Large or noisy files - should anything not land?

Findings (CODE / object store):

1. **The branch removes the noise rather than adding it.** Main tracks 582 files / 46,245,713 bytes; HEAD tracks 2,172 files / 33,502,477 bytes (`git ls-tree -r -l`). The difference is `spikes/results/*` (raw spike JSONL: p1-arm-l-v2.jsonl 5.9 MB, p1-arm-l.jsonl 5.5 MB, p7.jsonl 4.6 MB, e3.jsonl 4.0 MB, s-typed.jsonl 3.4 MB, s-var.jsonl 2.6 MB, ...) and `spikes/runner/*`, which move to the separate `redkiln-corpus` repo at `CORPUS_PIN` (ADR-0013; `.gitignore` diff, `CORPUS_PIN:4-5`). `.gitignore` now ignores `spikes/corpus/`, `spikes/runner/`, `spikes/results/*`, `.redkiln/telemetry/`, `.redkiln/worktrees/`, `spikes/*.log`.
2. **No single new blob exceeds 1 MB**; HEAD's largest file is `references/audit/2026-09-10/GROUNDING-MERGED.json` (1.44 MB), already on main. Largest new code files: `crates/rk-cli/src/verbs/run.rs` 302 KB (6,481 lines), `crates/rk-cli/tests/it/landing.rs` 167 KB, `doctor.rs` 162 KB, `crates/rk-kernel/src/runtime/step.rs` 156 KB. Large, but source, not noise.
3. **History weight, not tree weight.** The 416 commits introduce 4,345 new blobs totalling 357 MB uncompressed (`rev-list --objects main..HEAD | cat-file --batch-check`). It is dominated by repeated rewrites of a few markdown files: runbook/PROGRESS.md 231 revisions = 96.7 MB; spikes/DECISIONS-2026-09-21.md 123 revisions = 43.8 MB; the architecture design 37 revisions = 28.8 MB; the AC register 24 revisions = 19.1 MB; references/adr/README.md 86 revisions = 13.4 MB. Delta compression handles this; it is a clone-size footnote, not a blocker. Compressed pack size was not measured (unverified).
4. **Tracked-but-ignored leftovers (trivial):** `spikes/s-typed.log`, `s-typed-oldschema.log`, `s-var.log`, `s-var-2400.log` (33 KB total) are tracked on main and at HEAD but match the new `spikes/*.log` ignore row (`git ls-files -ci --exclude-standard`). Candidate `git rm --cached` housekeeping.
5. **Telemetry and worktrees**: `.redkiln/telemetry/` and `.redkiln/worktrees/` are ignored and nothing under them is tracked (`git ls-files .redkiln` filtered: 0 rows). Good.
6. **Secrets**: grep for sk-ant, AKIA, ghp_, PEM headers over HEAD hits only deliberately fake test strings (`crates/rk-adapter-claude/tests/it/config.rs:37`, `crates/rk-cli/tests/fixtures/doctor/RK-D-CREDENTIAL-RETAINED/events.jsonl:1`, `crates/rk-store/tests/it/store.rs:519-521`). No real credential found. The two `*.keys` files are rk-store migration key lists, not secrets.
7. **Machine-specific paths** (`D:/repos`, `C:/Users/ryanm`): present in `CORPUS_PIN:4` (`CORPUS_REPO=D:/repos/redkiln-corpus`), `.redkiln/corpus-records.toml`, `crates/rk-adapter-claude/tests/it/live.rs`, `crates/rk-cli/src/verbs/run.rs`, `crates/rk-git/src/{error,path}.rs` and some fixtures. The corpus-bound suite is local-only by design (89 `feature = "corpus"` sites, 8 of them in rk-kernel; `.github/workflows/ci.yml` header comment, decision D-N), so hosted CI cannot exercise it. Not a landing blocker, but anyone cloning main cannot run the corpus-bound suite without `D:/repos/redkiln-corpus` at the pin (unverified: whether the corpus repo itself has a durable remote).
8. **Ignored tests**: 59 `#[ignore` lines under crates, mostly live-dispatch, guard and cross-process tests (live.rs 11, live_guard.rs 9, cross_process.rs 6, conformance.rs 6+2). Some hits are doc comments. These are by design not run by `cargo test --workspace`.

Recommendation on content: nothing needs to be excluded from the landing except the four tracked `.log` files if you want the tree to match `.gitignore`. Do NOT rewrite history to slim the PROGRESS.md churn: see section 5.

## 4. Does the runbook intend a single merge, or earlier landings?

RECORDS: **the runbook has no landing step at all.**
- `runbook/phase-0/s0-preconditions.md:121-123`: "Cut a branch. Nothing below runs on `main`." (section 0.4).
- `runbook/phase-1/s8-exit-criterion.md` (six runnable checks) and `runbook/INDEX.md:100` (M5 "Phase 1 exits" = Checkpoint 8 passed) contain no merge, PR or push-to-main line. grep for merge/main/PR/pull request across INDEX, 00-preamble, halts, s8-exit-criterion, s8j returns nothing that schedules a merge to main.
- `runbook/s9-rollback.md:25-27` treats the branch as one unit that is either salvaged (cherry-pick onto main) or deleted: "the 2026-09-11 remediation ... are on main already and are not at risk."
- The only precedent is the earlier remediation branch: "merged to `main` as `66d0ced` (--no-ff, 2026-09-11) and pushed; branch deleted" (`runbook/PROGRESS.md:20`).
- Pushing is governed by the operator-gate rule: `spikes/DECISIONS-2026-09-21.md:1688` (D21-42 14a): "`ci.yml` green on the pushed branch, every job, both OS legs, is a Checkpoint 8 condition. Each push is confirmed with the operator." `runbook/phase-1/s8j-run-preparation.md:1215` still reads "`git push -u origin build/phase-0-to-1` (the first time)".
- Earlier-landing candidates are therefore never planned; the de facto intent is a single merge after Checkpoint 8 / M5. The runbook is silent on how, and on the post-merge state of the records that cite branch commit hashes (section 5).

CODE/object store contradicts the runbook on one point: the branch HAS been pushed, and has a live remote line (section 6).

## 5. Why the merge shape matters (hash citations)

`runbook/PROGRESS.md` and INDEX cite 223 distinct backticked 7-10 hex commit ids; 189 of the 7-hex ones in PROGRESS.md are commits unique to this branch (computed by intersecting `rev-list --abbrev-commit main..HEAD` with the backticked tokens). The checkpoint ledger ("Checkpoint 7c passed 2791f1d", "CP8h passed d24d15a", etc.) is an evidence chain. A squash or rebase-merge would orphan those ids. Only a merge commit (as with 66d0ced) or a fast-forward preserves them.

## 6. Conflict risk

- **vs main: nil.** main == merge-base (`353ea01`); `origin/main` (local remote-tracking ref) is also `353ea01`. A fast-forward is possible. Caveat: the remote-tracking ref may be stale; whether the real origin/main has since moved is UNVERIFIED (no fetch allowed).
- **vs the branch's own remote: real divergence, but textually clean.** Merge-base of HEAD and `origin/build/phase-0-to-1` is `d24d15a` ("s8h: refuting round 3 dry", CP8h passed). Local is 40 commits ahead (10 files, all docs: PROGRESS.md, s8i-s-bin.md, DECISIONS-2026-09-21.md, METHOD-S.md, ADR README etc.; +3,942/-80). Remote is 15 commits ahead (64 files). **File overlap between the two sides: zero** (`comm -12` of the two name lists is empty). So a merge should apply without textual conflict; a rebase would also work but would rewrite 40 commits whose ids are cited.
- A third ref, `origin/chore/temper-1.3.0` (`f571243`, 2026-10-05), is one commit on top of 7860e66 (vendored temper CLI bump).
- Remote-only content (all dated 2026-10-03, authored through Claude sessions, merged by PR #1/#2/#3 by Ryan Britton):
  - CI-driven product fixes: `d4da70a` (first Linux and node runs: POSIX-honest dispatch tests, pnpm fixture step, AC-465 allowlist (e)-(h)), `cb4ddd4` (nextest-absent PATH keeps /usr/bin:/bin), `ef747da` (Windows 8.3 temp paths, rk-git fixture identity), `9959b2f` (rk-containment POSIX `Group::kill` now waits for members, zombies read dead; `crates/rk-containment/src/posix.rs`), `aab766c` (rk-cli ac248 deadline wait), `f433854` (rk-cli temp_repo maintenance off).
  - temper adoption: `Cargo.toml` +64 lines (59 named lints at warn), `clippy.toml`, `rustfmt.toml`, `AGENTS.md` and `CLAUDE.md` for the workspace and 12 crates, `.claude/temper/*`, `.githooks/pre-commit` (opt-in via `core.hooksPath`), `.github/workflows/rust.yml` (job `temper`), and a recorded lint baseline (commit message: 12,147 findings tightened to 6,606, "accepted legacy debt, a human decision (H-12)").
- **None of this is in any runbook, ADR or decision file** (grep for the commit ids and for "temper"/"affectionate-cray" in runbook/, spikes/DECISIONS-*.md, references/adr/ returns only unrelated hits). So the records do not know that:
  - CI has already run on the remote and produced real red results (CODE-side evidence is those commit messages); the `ARMED, NOT DISCHARGED` comments in ci.yml (3 sites) and PROGRESS.md "ci.yml ... never run - nothing is pushed" (PROGRESS.md:28, :559, :621) are stale.
  - Product code changed after the last checkpoint pass: CP8h passed at `d24d15a`, which predates every remote-only fix. The checkpoint ledger's "re-run whole" evidence does not cover the integrated tree.

## 7. CI evidence

- Workflow: one file, `.github/workflows/ci.yml`, three jobs at remote head: `gates` (ubuntu-latest + windows-latest matrix: fmt, clippy, audit tooling, nextest pin, test, rk-ports conformance, P6 lattice, audit, deny), `no-egress (AC-498)`, `structural assertions` (corpus residency AC-465 and six more). Remote head adds a long-form-temp Windows step and a pnpm fixture step (ci.yml:79, :94 at `7860e66`); local HEAD lacks both. Plus `rust.yml` (temper ratchet) on remote only.
- RECORD evidence of green: none in the snapshot. The phase-0/phase-1 checkpoints were run locally on the operator's Git Bash under `runbook/tools/nodispatch.sh` (PROGRESS.md:48). The runbook itself says hosted CI is s8j's job (s8j 1215 loop; Checkpoint 8j (7) "every-job-green read").
- Object-store evidence of red-then-fixed: `d4da70a`, `ef747da`, `9959b2f`, `f433854` messages describe failing ubuntu-latest and windows-latest runs (pull_request and push runs of the same commit differing: f433854). The last remote-side verification claimed in the messages is local ("rk-cli 325/325; doctor:: 47/47 three runs"); **whether `7860e66` or `f571243` is green on both legs is UNVERIFIED** (no `gh` access in this audit).
- Structural gate on local HEAD would fail: running the AC-465 `git ls-files ... | grep -v` pipeline from the local ci.yml against HEAD's index yields **39** untracked-by-allowlist `.jsonl` paths (the synthetic cost/doctor/unread/pack-min fixtures). That is exactly what remote commit `d4da70a` added allowlist entries (e)-(h) for. Evidence that local HEAD is not CI-green as it stands.
- Flake class: two of the remote fixes are race fixes (git auto-maintenance, whole-second clock) - the suite has been flaky across OS and should be run repeatedly after integration.

## 8. State of the in-flight work (not landable yet)

- Runbook position (RECORD): Step 8i in progress, B1 complete (`ee44bf1`, $5.561026 spent), item (f) owed-item commit `36dd8ac` is HEAD. 8j "PLANNED ... Not started" (PROGRESS.md:50). Step 8 "RE-OPENED ... Checkpoint 8 has NOT passed, and M5 is not reached" (PROGRESS.md:51).
- The snapshot is also not the live tree: the active session has 4 uncommitted modified files (PROGRESS.md, s8i-s-bin.md, DECISIONS-2026-09-21.md, METHOD-S.md per the task's git status). Anything landing must wait for that session's commits.
- 8i's S-BIN driver and results live in the corpus repo, not in this branch (`.gitignore` ignores spikes/runner and spikes/results); landing this branch does not land the S-BIN evidence. That evidence's durability depends on `D:/repos/redkiln-corpus` (UNVERIFIED remote).

## 9. Recommended landing approach

1. **Do not land now. Land once, at M5 (Checkpoint 8 passed), as a `--no-ff` merge commit** like `66d0ced`. Preserves the 189+ cited hashes and the checkpoint evidence chain; no squash, no rebase, no history rewrite. Add the missing runbook line saying so (the plan is currently silent).
2. **First reconcile the branch with its own remote** (before any more stage work if possible): merge `origin/build/phase-0-to-1` (and optionally `origin/chore/temper-1.3.0`) into the local branch with a merge commit. Zero file overlap makes it textually clean; then re-run the full checkpoint block (CP5, 7b, 7b-ii, 7d, 8b-8h) because product code (rk-containment, rk-adapter-claude, rk-git, rk-cli) changed after CP8h passed at `d24d15a`. Coordinate with the active s8i session so only one session merges, and have it record the 15 remote commits in PROGRESS.md and fix the stale "ci.yml never run" and "first push" statements (ci.yml ARMED comments, s8j:1215).
3. **Get CI actually green on the integrated head, both OS legs plus no-egress, structural and temper jobs, repeated** (the flake class above), under the operator's per-push confirmation (D21-42 14a), and freeze that sha as 8j's evidence commit. Decide explicitly whether the temper lint baseline (6,606 findings accepted as debt) is acceptable to ship on main, since it now sits beside ADR-0102's "no -D warnings" gate.
4. **Pre-merge hygiene commit:** `git rm --cached` the four tracked `spikes/*.log`; confirm the corpus repo has a durable remote and that CORPUS_PIN (`61f590e`) resolves there, because this landing deletes 45 MB of spike data from main's tree (history keeps it); consider moving `CORPUS_REPO=D:/repos/redkiln-corpus` to a per-machine override so a fresh clone is not tied to one drive path.
5. **Keep main pristine until then, but consider an interim non-mainline safety net** (the remote branch already is one; make the integrated branch the pushed branch, so the 40 local-only commits stop being single-machine risk). If earlier landing is ever wanted, the clean cut is "everything except `runbook/` and `spikes/` stage churn" - not recommended, since the runbook's citations point at branch hashes.

## 10. Unverified (could not be established from the snapshot)

- Whether the real `origin/main` has moved past `353ea01` (remote-tracking ref only).
- Whether CI is green on `7860e66`/`f571243` (no hosted-run data in the object store).
- Compressed pack size of the 416-commit delta.
- Durability/remote of `D:/repos/redkiln-corpus`.
- Whether `core.hooksPath` is set to `.githooks` on the operator's machine (the temper pre-commit hook runs node and clippy on every commit if so).
- Content of the 4 uncommitted files in the live tree.
- Whether the remote-only commits' code merges also compile and pass the full local suite (not run; no cargo allowed).
- Another auditor's build logs sit in the sibling `audit/` directory (`build.txt`, `log-clippy.txt` etc.); deliberately not read, per the hard rule that only the snapshot tree may be read.
