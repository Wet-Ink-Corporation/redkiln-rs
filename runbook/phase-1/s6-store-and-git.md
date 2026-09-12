# Step 6 — `rk-store` and `rk-git`, with S-GIX run FIRST (Phase 1, part 2)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s6` | Phase 1 | Checkpoint 5 | Checkpoint 6 (ADR-0091 exists) | `s5` | `s7a` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


> ### 🚦 GATE CHECK
> ```bash
> ls D:/repos/redkiln-rs/references/adr/0090-*.md >/dev/null 2>&1 \
>   && echo "GATE DISCHARGED — proceed" || echo "GATE OPEN — go back to Step 4"
> ```
> **`0090`, not `0089`** — see Step 5's banner for why the token moved.

### 6.1 Run S-GIX before a line of `rk-git` worktree code is written

Two arms, both **$0 model cost**, both a docs.rs read plus a small test against the exact pin `gix = "=0.87.1"`.
The research pass named the trailer arm **"the highest-value single check in the list"**.

| Arm | Question | Consumer |
| --- | --- | --- |
| **S-GIX/a** | **A READ check (ADR-0098):** does gix discovery from a **linked worktree** return the main repository's common dir at the pin — or is gitoxide's own `crate-status.md` (the `GIT_COMMON_DIR`/`GIT_WORK_TREE` interplay is not implemented) still accurate? | the read-side owner of `rev-parse`. **Go:** gix resolves the common dir. **No-go:** `git rev-parse --git-common-dir` through the exec wrapper, once at startup, cached for the process. "Gates Phase 1? Yes for the read-side owner of rev-parse; the shell fallback is decided." |
| **S-GIX/b** | Does the pinned gix expose a first-class trailer-**writing** API, and does a five-line unit test round-trip a trailer **byte-identically** to `git interpret-trailers --in-place`? **The read half is discharged** — `BodyRef::trailers()` exists in gix-object 0.64.1 (ERRATA E-0077-1); the write half is the open question, and enumerating gix 0.87.1's 1,533 items found none [documented, fetched 2026-09-10]. | `rk-git` trailer write path (tier-1 state). The oracle is `git interpret-trailers` under pinned config over a named fixture matrix: an existing trailer block, no trailing newline, a `---` divider, CRLF, a duplicate token (ADR-0105 clause 3). |
| **S-GIX/c** | Does `PrepareFetch::with_revision` + `with_shallow` at the pin produce a checkout whose HEAD tree equals `git clone --depth 1 --revision <sha>`'s? (ADR-0098 decision 3 — the review clone is a **retained gix WRITE op** on the advisory surface, covered by the audit job and the Windows CI target.) | the review-clone path. **No-go:** the wrapper's `git clone --depth 1 --revision`. |

**`[WILL EXIST]`** — as `#[ignore]`d tests in `rk-git` that run on demand:

```bash
cargo build -p rk-git
cargo test -p rk-git -- --ignored s_gix_a_common_dir_from_linked_worktree
cargo test -p rk-git -- --ignored s_gix_b_trailer_roundtrip_matches_interpret_trailers
cargo test -p rk-git -- --ignored s_gix_c_shallow_clone_by_sha_matches_git
```

**Run these before the worktree code, not after.** A **go** on an arm may narrow ADR-0077 and nothing else may, and
the narrowing is written as a **successor record**, never as an edit to 0077 — ADR-0077 is `accepted`, and the
immutability rule binds accepted records absolutely. **This is the second of the seven decisions this runbook
takes** (see the preamble's table); it is an ADR, and **its number is `0091`, reserved** — `0089` was taken by the
facet-provenance amendment, `0090` by Step 4.4, `0092`–`0114` by the 2026-09-11 remediation, and the README's
numbering sentence names `0091` as this step's. **A no-go does not cost nothing — that sentence was false for arm a
(PIA-009).** The two outcomes are written down already: ADR-0098 fixes what each arm's no-go does (arm a → the
wrapper resolves the common dir; arm c → the wrapper clones), so ADR-0091 records the *results* and the git-version
floor, and supersedes 0077 in part only on a **go** that narrows it. Write into ADR-0091 the two verdict lines
Checkpoint 6 greps for, literally `S-GIX/a: go` / `S-GIX/a: no-go` and the same for `/b` and `/c`, one per line.

### 6.2 Build to the hybrid, whatever S-GIX says

| Direction | Owner | Operations |
| --- | --- | --- |
| **Read** | `gix`, exact pin `= "=0.87.1"` | `rev-parse` (including resolving the common dir AC-178 needs at startup — **on S-GIX/a's go**; on its no-go this one read moves to the wrapper, ADR-0098), repository and worktree status, trailer **parsing** (`BodyRef::trailers()`, discharged) |
| **Write** | a **process-exec wrapper** invoking the `git` executable — **and gix for one retained write, the shallow-by-SHA review clone** (`PrepareFetch::with_revision` + `with_shallow`; ADR-0098) | the enumerated set of ADR-0105 clause 5: `git worktree add` / `remove`; `git add`; `git commit` (incl. `--allow-empty`, `--trailer`); `git interpret-trailers` — always with `-c trailer.ifexists=add` / `--if-exists=add`, because the default `addIfDifferentNeighbor` silently drops a repeated identical `Gate:` or `Slice-Verdict:` trailer; `git write-tree` on a temporary index (the one `tree_hash` derivation, ADR-0105 clause 4); `patch-id`; fast-forward/merge on the integration ref; clone removal; `rev-parse --git-common-dir` on arm a's no-go. Every commit runs with `-c core.hooksPath=<empty dir>` (ADR-0105 clause 2). Anything not listed is a successor record. |

Six constraints that come with the hybrid and are easy to lose:

1. **All** shelling out to git goes through **one wrapper type inside `rk-git`**. No `std::process::Command` for
   git exists anywhere else in the workspace.
2. Git's stderr is diagnostic payload and is **never parsed for control flow**.
3. The wrapper asserts **at startup, not at first use**, that `git` is on `PATH` at or above a minimum version —
   and **that minimum is not yet established**, so the assertion is incomplete until you establish it. **This is
   the third of the seven decisions this runbook takes.** Pick a version, record it in the same successor record
   6.1 produces (ADR-0091), do not leave the assertion as a stub. (`git --version` on this box, 2026-09-08: the MSYS
   git at `/mingw64/bin/git`. Whatever you pick, the floor must be met by the CI runners too, not only by this
   machine; `--if-exists=add` on `interpret-trailers` needs git ≥ 2.20.)
4. Two git implementations in one process can disagree — about config resolution and about Windows path handling.
   Anything read by gix and then written by the executable is **re-read after the write, never assumed**. Paths are
   handed to git in **drive-letter form, never `\\?\`** — `git fetch` failed on the extended-length form in the
   spike runner (F20/2); the runner's `git_remote_path` normaliser is ported under this constraint with a Windows
   unit test (ADR-0105 clause 6).
5. **Git operations on the activity side go through the journalled `GitOps` seam** (ADR-0105 clause 1): a struct in
   `rk-git` that records every invocation and its result to the EventStore, so that on replay the kernel is served
   the journal and `rk-git` performs **zero writes** (AC-006/AC-008's "and zero rk-git writes").
6. **Worktrees are re-created per run, never reused**; a failed `worktree remove` (the Windows handle case, Halt 6)
   is a typed `rk-git` error and a `doctor` orphans row, with S-CANCEL-WIN's fixture as the regression. The review
   clone is **bare** unless the review activity declares it needs a working tree.

**The source the lift comes from (PIA-036, PIA-078).** `reconcile.rs` is the spike runner's **re-implementation**
(`spikes/runner/src/reconcile.rs`, 330d46c, 2026-09-02) of the old redkiln's `src/store/verify.ts` (the frozen tree at
`47faec9f`); it was **built and run in S0.2 over 360 commits / 74 slices (F12) and exercised, not validated against
an oracle**. After Step 2 it lives at `$CORPUS_REPO/runner/src/reconcile.rs` at the SHA in `CORPUS_PIN` — lift from
that commit, not a working tree, and name the SHA in the module header (the same rule as 7c's five files).

### 6.3 `rk-store`

One store per repository, at the common dir. Tier-1 / tier-2 split by the test *if the database were deleted, would
`redkiln run` take a different next action?* (ADR-0109). Trailers on **three** axes — `Story:`, `Slice-Verdict:`,
`Gate:` — because `Story:` alone is insufficient: **20.3% of slices (15/74) passed through a rejected state** over
360 trailer commits, and sealed-more-than-once was 21.6%, so a `Slice-Verdict:` axis is required; the trailer grammar
(key, value ABNF, column anchoring, the verdict vocabulary, the unknown-key rule, the legacy `Gate:` drift rule) is
design §5.4's table and AC-444's drift counts are pinned to it. Markdown is a **rendering**; nothing parses
structure out of prose. `reconcile()` runs at **every** entry — drift is 7% overall (9/79 = 11% on gaps over 1 h
against 3/95 = 3% at or under; the two Wilson intervals overlap, so **3.6× is directional, not separated** — PIA-107)
while 45–48% of gates exceed 1 h — and no optimisation may skip it.

**What the store promises, and no more (ADR-0097; decision D-H).** The step memo is **tier 2** — the SQLite memo
table ADR-0086 builds here — and is rebuildable by re-execution, never by rebuild from git; `doctor --rebuild`
reproduces the **stage-level** next action from git alone (AC-161/AC-164 as narrowed), and AC-187's memo replay is
scoped to a run whose EventStore survives. The memo key is SHA-256 over ADR-0097's eight-component length-prefixed
tuple; liveness is the verb's `live = true` flag and nothing else. **rk-store's contract as rules (ADR-0105 clause
7):** `busy_timeout` **5000 ms** (rusqlite's default; the R-SQLITE read confirms); every write transaction shorter
than it — a rebuild writes a separate file and renames; **one writer connection per process behind a mutex, no
queue** (`rusqlite::Connection` is `Send` and `!Sync`); readers one per thread, also with `busy_timeout`; `doctor`
refuses a common dir on a network filesystem (WAL does not work over one); the event-kind table (kind · tier ·
payload · producing step) is design §5's with `rk-types` as owning crate; retention is argv/env **digest-only**,
stdout/stderr verbatim up to a size cap, child environment **never**, with a `doctor` row for credential-shaped
values. AC-184's **tier-1 projection** is the tier-1 rows whose anchor is reachable from the clone's refs; orphan
rows and rewrite events sit outside the rebuild differential (ADR-0105 clause 9). A tier-1 write resolves the
run-owned worktree **from the store**, never from cwd (the new AC §6.4 criterion).

**Spikes owed by this step (PIA-047):** **R-SQLITE** — the `busy_timeout` / one-writer read against docs.rs rusqlite
0.40.2 and sqlite.org, $0, minutes; run it or record its deferral in the checkpoint commit.

**Checkpoint 6.** All of:

```bash
cd D:/repos/redkiln-rs
cargo build --workspace && cargo clippy --workspace --all-targets && cargo test --workspace   # exit 0 each
out=$(cargo test --workspace --features corpus 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --features corpus exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: --features corpus selected 0 tests — the bundle was never read"; exit 1; }   # exit 0 on this box AND ≥ 1 test actually ran: AC-440's fixture and the F12 replay read the bundle
test -f Cargo.toml || { echo "FAIL: Cargo.toml absent — the workspace table does not exist yet"; exit 1; }
grep -c 'gix = "=0.87.1"' Cargo.toml                                       # → 1. EXACT pin, in the WORKSPACE table (ADR-0084)
test -f crates/rk-git/Cargo.toml || { echo "FAIL: crates/rk-git/Cargo.toml absent"; exit 1; }
grep -c 'gix.workspace = true' crates/rk-git/Cargo.toml                    # → 1. the member inherits it (PIA-030: the old grep false-failed on this form)
test -s crates/rk-git/src/git_exec.rs || { echo "FAIL: crates/rk-git/src/git_exec.rs absent — the type-ban grep is vacuous"; exit 1; }
test "$(grep -rnE 'std::process::Command|tokio::process::Command' crates/ --include=*.rs \
  | grep -v 'crates/rk-git/src/git_exec.rs' | wc -l)" = 0 \
  || { echo "FAIL: process::Command outside the one wrapper"; exit 1; }   # → 0: one wrapper, one file — and clippy.toml's disallowed-types is the same rule at build time
test -s crates/rk-git/src/git_exec.rs || { echo "FAIL: crates/rk-git/src/git_exec.rs absent — the CI grep is vacuous"; exit 1; }
test "$(grep -rn 'Command::new("git")' crates/ --include=*.rs | grep -v 'crates/rk-git/src/git_exec.rs' | wc -l)" = 0 \
  || { echo "FAIL: Command::new(\"git\") outside the wrapper"; exit 1; }   # → 0 (the CI grep, PIA-035)
test -s crates/rk-git/src/git_exec.rs || { echo "FAIL: crates/rk-git/src/git_exec.rs absent"; exit 1; }
grep -c 'hooksPath' crates/rk-git/src/git_exec.rs                          # → ≥ 1: hooks isolation on every commit (ADR-0105 clause 2)
test -s crates/rk-git/src/git_exec.rs || { echo "FAIL: crates/rk-git/src/git_exec.rs absent"; exit 1; }
grep -c 'ifexists=add\|if-exists=add' crates/rk-git/src/git_exec.rs          # → ≥ 1: repeated identical trailers are appended, not dropped
cargo audit --deny unmaintained                                            # exit 0; gix-* clean at the pin; clone is on gix and in scope
test "$(ls references/adr/0091-*.md 2>/dev/null | wc -l)" = 1 || { echo "FAIL: expected exactly one references/adr/0091-*.md — the S-GIX successor record"; exit 1; }   # → 1: the S-GIX successor record
ls references/adr/0091-*.md >/dev/null 2>&1 || { echo "FAIL: ADR-0091 absent — no S-GIX record to grep"; exit 1; }
test "$(grep -cE '^S-GIX/[abc]: (go|no-go)$' references/adr/0091-*.md)" = 3 \
  || { echo "FAIL: ADR-0091 does not carry exactly three S-GIX verdict lines"; exit 1; }   # → 3: one explicit verdict line per arm (PIA-035)
ls references/adr/0091-*.md >/dev/null 2>&1 || { echo "FAIL: ADR-0091 absent — no S-GIX record to grep"; exit 1; }
grep -cE 'git (>=|version) [0-9]+\.[0-9]+' references/adr/0091-*.md         # → 1: the git-version floor is a number in the record
out=$(cargo test -p rk-git -- --ignored s_gix_ 2>&1); rc=$?
test "$(printf '%s\n' "$out" | grep -cE '^test .*s_gix_[abc]_')" = 3 \
  || { echo "FAIL: the s_gix_ filter did not select exactly the three arms"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -E '^test .*s_gix_[abc]_'
if [ "$rc" -eq 0 ]; then echo "S-GIX: all three arms GO"; else echo "S-GIX: NO-GO on at least one arm (rc=$rc) — each arm above must equal its S-GIX/x line in ADR-0091"; fi   # → the three arms ran; their verdicts match the record
```

The verdict and floor lines make "a **written S-GIX result**" checkable instead of aspirational, and the
`hooksPath`/`ifexists` greps make two of ADR-0105's rules a build-time fact rather than a sentence. "We built the hybrid so we
didn't need to run it" is not a result; the spike is what licenses the *narrowing*, and an unrun spike leaves
ADR-0077 exactly where it is. If you record the result in `spikes/` rather than an ADR, point those two lines at
that file instead — but point them somewhere, and run them.

**Acceptance set:** git-as-truth **AC-160–AC-199** (notably **AC-178** common-dir resolution at startup, **AC-184**
`doctor --rebuild` from git alone with its 40-hex corpus-pin assertion, **AC-187** resume); store and typed-artifact
**AC-120–AC-159**; **AC-464** corpus reads through the object store, **AC-465** residency. **AC-013, AC-187,
AC-266, AC-267, AC-271, AC-401 are PROVISIONAL on P8** — multi-step resume is unmeasured; F38 held exactly one step
at n=3, and its 65-min arm, 24-h arm and all three controls are unrun. Build them; do not claim them.

---
