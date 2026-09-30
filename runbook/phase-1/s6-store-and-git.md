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
> ( ls D:/repos/redkiln-rs/references/adr/0090-*.md >/dev/null 2>&1 \
>     || { echo "GATE OPEN — no ADR-0090. Go back to Step 4; no work in this stage may start."; exit 1; }
>   echo "GATE DISCHARGED — proceed" )   # a subshell: a pasted FAIL must not kill an interactive shell
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

**Run these before the worktree code, not after.** A **go** on an arm is what may narrow ADR-0077 *in ADR-0091*,
and the narrowing is written as a **successor record**, never as an edit to 0077 — ADR-0077 is `accepted`, and the
immutability rule binds accepted records absolutely. *(Corrected 2026-09-13: this read "a **go** on an arm may
narrow ADR-0077 **and nothing else may**", which is false as history. **ADR-0098** narrowed 0077 on 2026-09-11 on a
documentation read, with no arm having run — legitimately, because an accepted ADR may supersede another under
ADR-0092 clause 2.1. The sentence is about what licenses *this* record's narrowing, not about 0077's history.)*
**This is the second of the seven decisions this runbook takes** (see the preamble's table); it is an ADR, and
**its number is `0091`** — `0089` was taken by the facet-provenance amendment, `0090` **by Step 4.4 on
2026-09-12**, `0092`–`0114` by the 2026-09-11 remediation, `0115`–`0116` by the 2026-09-12 alignment review and
`0117` by the `stale_policy` adjudication on 2026-09-13; `0091` was the last reserved number. **A no-go does not
cost nothing — that sentence was false for arm a (PIA-009).** The two outcomes are written down already: ADR-0098
fixes what each arm's no-go does (arm a → the wrapper resolves the common dir; arm c → the wrapper clones), so
ADR-0091 records the *results* and the git-version floor, and supersedes 0077 in part only on a **go** that narrows
it. Write into ADR-0091 the three verdict lines Checkpoint 6 greps for, literally `S-GIX/a: go` / `S-GIX/a: no-go`
and the same for `/b` and `/c`, one per line.

> **EXECUTED 2026-09-13 — `7669fa3`.** The arms ran and **ADR-0091 is accepted**: `S-GIX/a: go`, `S-GIX/b: no-go`,
> `S-GIX/c: go`. It **supersedes nothing** — arm b was the only arm whose go could have withdrawn a sentence of
> ADR-0077, so 0077's write path stands in full and its frontmatter was not touched. The floor is **`git >= 2.20`**
> (ADR-0091 decision 4): the conditional half of E10 resolved *downward*, because `clone --revision` (git 2.49) is
> reached only on a /c no-go and /c is a go. §6.2 below therefore builds the **go** branch of arms a and c: gix owns
> the common-dir read, gix performs the review clone, and neither `rev-parse --git-common-dir` nor
> `clone --depth 1 --revision` enters the wrapper's write set. Arm b's no-go leaves the trailer write path exactly
> where ADR-0077 put it. The three arms remain as `#[ignore]`d tests and now **assert their own verdict against the
> record**, so re-running them is a cross-check rather than a re-measurement.

### 6.2 Build to the hybrid, whatever S-GIX says

| Direction | Owner | Operations |
| --- | --- | --- |
| **Read** | `gix`, exact pin `= "=0.87.1"` | `rev-parse` (including resolving the common dir AC-178 needs at startup — **on S-GIX/a's go**; on its no-go this one read moves to the wrapper, ADR-0098), repository and worktree status, trailer **parsing** (`BodyRef::trailers()`, discharged) |
| **Write** | a **process-exec wrapper** invoking the `git` executable — **and gix for one retained write, the shallow-by-SHA review clone** (`PrepareFetch::with_revision` + `with_shallow`; ADR-0098) | the enumerated set of ADR-0105 clause 5: `git worktree add` / `remove`; `git add`; `git commit` (incl. `--allow-empty`, `--trailer`); `git interpret-trailers` — always with `-c trailer.ifexists=add` / `--if-exists=add`, because the default `addIfDifferentNeighbor` silently drops a repeated identical `Gate:` or `Slice-Verdict:` trailer; `git write-tree` on a temporary index (the one `tree_hash` derivation, ADR-0105 clause 4); `patch-id`; fast-forward/merge on the integration ref; clone removal; `rev-parse --git-common-dir` on arm a's no-go. Every commit runs with `-c core.hooksPath=<empty dir>` (ADR-0105 clause 2). Anything not listed is a successor record. |

Six constraints that come with the hybrid and are easy to lose:

1. **All** shelling out to git goes through **one wrapper type inside `rk-git`**. No `std::process::Command` for
   git exists anywhere else in the workspace.
2. Git's stderr is diagnostic payload and is **never parsed for control flow**.
3. The wrapper asserts **at startup, not at first use**, that `git` is on `PATH` at or above a minimum version.
   **The floor is `2.20` unconditionally, and it is conditional above that** (DECISIONS-2026-09-12 E10): 2.20 is
   what `interpret-trailers --if-exists=add` needs, and every other op in clause 5's *unconditional* write set is
   older than that. The one op that is newer is **`git clone --depth 1 --revision <sha>`**, and that is reached
   **only on an S-GIX/c no-go** — if gix performs the review clone, the wrapper never invokes it. So ADR-0091,
   which §6.1 writes immediately after the arms run, records the final floor: it stays `2.20` on a /c **go**, and
   rises to the version that introduced `clone --revision` on a /c **no-go** (confirm that version against the
   git release notes when you write the record; `git version 2.55.0.windows.3` on this box lists
   `--[no-]revision <rev>` in `git clone -h` [measured, 2026-09-12]). **This is the third of the seven decisions
   this runbook takes**, and it is taken here rather than left a stub. (`git --version` on this box, 2026-09-08: the MSYS
   git at `/mingw64/bin/git`. Whatever you pick, the floor must be met by the CI runners too, not only by this
   machine; `--if-exists=add` on `interpret-trailers` needs git ≥ 2.20.)
4. Two git implementations in one process can disagree — about config resolution and about Windows path handling.
   Anything read by gix and then written by the executable is **re-read after the write, never assumed**. Paths are
   handed to git in **drive-letter form, never `\\?\`** — `git fetch` failed on the extended-length form in the
   spike runner (F20/2); the runner's `git_remote_path` normaliser is ported under this constraint with a Windows
   unit test (ADR-0105 clause 6).
5. **Git operations on the activity side go through the journalled `GitOps` seam** (ADR-0105 clause 1): a struct in
   `rk-git` that records every invocation and its result to the EventStore, so that on replay the kernel is served
   the journal and `rk-git` performs **zero writes** (AC-006/AC-008's "and zero rk-git writes"). It is a **struct
   and never a trait**, so AC-483's port-inventory denylist stands and Checkpoint 7a's count of five port traits is
   unaffected. **ADR-0115 governs the handle it journals through** *(added 2026-09-13; this step was written before
   0115 existed and did not cite it)*: the EventStore port is held as `Arc<Mutex<dyn EventStore>>`, because
   `append` takes `&mut self` and an `Arc<dyn T>` yields only `&T`. The lock is acquired by the composition root at
   the call boundary and released when the call returns; **no port method acquires it, and no lock is held across a
   dispatch of another port**. Lock poisoning is an open item 0115 names and does not close — it must not be
   silently recovered with `unwrap_or_else(|e| e.into_inner())`.
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
7 — one of its **eleven**, not nine):** `busy_timeout` **5000 ms** (rusqlite's default, a **documented** figure;
see the R-SQLITE note below for what is and is not measured); every write transaction shorter
than it — a rebuild writes a separate file and renames; **one writer connection per process behind a mutex, no
queue** (`rusqlite::Connection` is `Send` and `!Sync`); readers one per thread, also with `busy_timeout`; `doctor`
refuses a common dir on a network filesystem (WAL does not work over one); the event-kind table (kind · tier ·
payload · producing step) is design §5's with `rk-types` as owning crate — **and ADR-0116 supersedes ADR-0105
clause 7 in part**: the table gains **`artifact.write`** (tier 2, emitted by every activity output, payload
`kind, item id, schema_version, artifact content digest`), the `EventKind` enum is **closed**, a kind in the design
table and not in the enum (or the reverse) is schema drift, and the digest is the artifact's content digest
computed by the **same** function the typed-artifact digests use — no second digest algorithm enters the system; retention is argv/env **digest-only**,
stdout/stderr verbatim up to a size cap, child environment **never**, with a `doctor` row for credential-shaped
values. AC-184's **tier-1 projection** is the tier-1 rows whose anchor is reachable from the clone's refs; orphan
rows and rewrite events sit outside the rebuild differential (ADR-0105 clause 9). A tier-1 write resolves the
run-owned worktree **from the store**, never from cwd (the new AC §6.4 criterion).

**Spikes owed by this step (PIA-047):** **R-SQLITE**, and it splits in two — *(clarified 2026-09-13: this step
previously said the read "confirms" the 5000 ms figure four paragraphs above while listing the whole spike as owed
here, which cannot both be true)*. Its **documentary** half is **discharged and cited**: rusqlite 0.40.2's
`busy_timeout` default and `Connection: Send + !Sync`, and sqlite.org's WAL single-writer and network-filesystem
rules, are `[documented]` evidence lines on ADR-0105 itself. Its **empirical** half is **not** discharged, and it is
**AC-181's literal instrument** — the two-process contention test. ADR-0105's own evidence limits say so: *"It does
not license claiming … that 5000 ms suffices under the self-hosting run's load — R-SQLITE and Step 8 (3) are the
measurements."* So: cite the documentary half, build AC-181's test, and if it cannot be run, record the deferral in
the checkpoint commit and report the store's concurrency contract as **documented-only** — never as measured.

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence:
every one of its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins.
Author each named test **before** the code it describes, run it, and watch it fail for the right reason —
the absent behaviour, not a compile error or a filter that matched nothing.

- The three **S-GIX arms are themselves the red step** and run before a line of worktree code (§6.1) — that
  ordering is already test-first, and it is why a no-go costs a wrapper call rather than a rewrite.
- `trailer_roundtrip_matches_interpret_trailers` — over ADR-0105 clause 3's **named fixture matrix**: an existing
  trailer block, no trailing newline, a `---` divider, CRLF, and a duplicate token. Red on the duplicate-token case
  specifically: git's default `addIfDifferentNeighbor` silently drops a repeated identical `Gate:` trailer, so a
  wrapper that omits `--if-exists=add` must fail this test. That case is the discriminating one; the other four
  pass either way.
- `reconcile_runs_at_every_entry` — AC-172's `Reconciled<State>` is the instrument: a state that reaches the kernel
  without passing through it must not be **constructible**. Red as a `trybuild` must-fail fixture, not a runtime
  assertion, because this one is type-level.
- `git_paths_are_drive_letter_form` — the `\\?\` extended-length path that broke `git fetch` (F20/2). Red by
  handing the normaliser an extended-length path. Note golden constraint 4's warning: this defect was **not**
  reachable from a unit test when it happened, so the unit test pins the normaliser, and the wrapper still needs
  one real end-to-end invocation before Checkpoint 6 is claimed.

**Checkpoint 6.** All of:

**Hardened 2026-09-13, before its first run** — the preamble's rule is that hardening lands *before* the pass,
or the recorded pass attests the old text. The audit found **one line that failed on this tree for a reason
unrelated to its rule** and **three that could not fail at all**; each is marked below with what it was.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }

# ---- the five CI gates, each guarded. `cargo build && clippy && test` on one line exited non-zero
# ---- without stopping the block: nothing consumed the status (the guard was absent, not weak). And
# ---- `fmt` and `deny` were MISSING from this checkpoint entirely, which is how CI goes red after a
# ---- green Checkpoint 6 — the licence set is re-derived when the lock moves, and it moved here.
cargo build --workspace  || { echo "FAIL: cargo build --workspace"; exit 1; }
cargo fmt --all --check  || { echo "FAIL: cargo fmt --all --check"; exit 1; }
cargo clippy --workspace --all-targets || { echo "FAIL: cargo clippy --workspace --all-targets"; exit 1; }
out=$(cargo test --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }   # gix-* clean at the pin; the clone is on gix and in scope
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses"; exit 1; }

# ---- the corpus precondition, BEFORE any corpus read (AC-464). A dirty or unpinned checkout is
# ---- refused rather than read.
test -f CORPUS_PIN || { echo "FAIL: CORPUS_PIN absent"; exit 1; }
CORPUS_REPO=$(grep '^CORPUS_REPO=' CORPUS_PIN | cut -d= -f2-)
CORPUS_SHA=$(grep '^CORPUS_SHA=' CORPUS_PIN | cut -d= -f2-)
test -d "$CORPUS_REPO" || { echo "FAIL: corpus absent at $CORPUS_REPO"; exit 1; }
echo "$CORPUS_SHA" | grep -qE '^[0-9a-f]{40}$' || { echo "FAIL: CORPUS_SHA is not an unabbreviated 40-hex SHA"; exit 1; }
h=$(git -C "$CORPUS_REPO" rev-parse HEAD); test "$h" = "$CORPUS_SHA" || { echo "FAIL: corpus HEAD $h != CORPUS_SHA $CORPUS_SHA"; exit 1; }
d=$(git -C "$CORPUS_REPO" status --porcelain); [ -z "$d" ] || { echo "FAIL: the corpus checkout is dirty; AC-464 refuses before any read"; printf '%s\n' "$d"; exit 1; }

# ---- the corpus suite. The old form grepped for `test result: ok. N passed`, which matches ANY
# ---- binary in a workspace run — it attested that the feature existed, not that the bundle was
# ---- read. THIS CANNOT FAIL as written. Replaced by the NAMED tests plus a differential proving
# ---- the feature is what gates them.
out=$(cargo test --workspace --features corpus 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --features corpus exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(printf '%s\n' "$out" | grep -cE '^test corpus_f12::.* \.\.\. ok$')
test "${n:-0}" -ge 3 || { echo "FAIL: ${n:-<unreadable>} corpus_f12 tests ran, expected >= 3 — the bundle was never read"; exit 1; }
k=$(cargo test --workspace -- --list 2>/dev/null | grep -c 'corpus_f12' | tr -d '[:space:]')
test "${k:-1}" = 0 || { echo "FAIL: the corpus tests are selected WITHOUT --features corpus (got ${k}); the feature is not what gates them"; exit 1; }

# ---- the pins. `grep -c 'gix = "=0.87.1"'` returned **0** on this tree: Cargo.toml:32 is
# ---- column-aligned (`gix` + 8 spaces), so the line FAILED FOR A REASON UNRELATED TO ITS RULE.
# ---- The guard's scope is "exactly one exact pin in the workspace table"; alignment is incidental,
# ---- so the regex is hardened and the manifest left alone (golden constraint 9).
test -f Cargo.toml || { echo "FAIL: Cargo.toml absent — the workspace table does not exist yet"; exit 1; }
n=$(grep -cE '^gix[[:space:]]*= "=0\.87\.1"' Cargo.toml); test "${n:-0}" -eq 1 || { echo "FAIL: ${n:-<unreadable>} exact gix pins in the workspace table, expected 1 (ADR-0084)"; exit 1; }
# `.` was an unescaped wildcard, and a COMMENT quoting this line counts: a first draft of
# rk-git/Cargo.toml named it twice in its header and the assertion read 3. Anchored and escaped.
test -f crates/rk-git/Cargo.toml || { echo "FAIL: crates/rk-git/Cargo.toml absent"; exit 1; }
n=$(grep -cE '^gix\.workspace = true$' crates/rk-git/Cargo.toml); test "${n:-0}" -eq 1 || { echo "FAIL: rk-git does not inherit the workspace gix pin exactly once (got ${n:-<unreadable>})"; exit 1; }

# ---- one wrapper. Scoped to PRODUCTION code and comment lines excluded (narrowed to its warrant
# ---- 2026-09-13; see §6.2's note — ADR-0077:70 bans process::Command FOR GIT, and the wider form
# ---- blocked AC-181's two-process instrument).
test -s crates/rk-git/src/git_exec.rs || { echo "FAIL: crates/rk-git/src/git_exec.rs absent — the type-ban grep is vacuous"; exit 1; }
# ~~test "$(git ls-files 'crates/*/src/*.rs' 'crates/*/src/**/*.rs' \~~
# ~~  | xargs grep -nE 'std::process::Command|tokio::process::Command' \~~
# ~~  | grep -vE '^crates/rk-git/src/git_exec\.rs:' \~~
# ~~  | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | wc -l | tr -d '[:space:]')" = 0 \~~
# ~~  || { echo "FAIL: process::Command in production code outside the one wrapper"; exit 1; }~~
# (dated 2026-09-30, s8g build, package D21128-8g; D21-128 (B), TAKEN PROVISIONALLY under D21-105 and on the confirm
# list as the answer to the s8d close's P7 flag; D21-47's time-bound rule: "A time-bound guard in an earlier
# checkpoint is amended by the stage that moves its premise, in the same commit.") The struck `= 0` has read 14 since
# 7c and 7d, and every hit is a NON-git spawn. s7c and s7d RAISED this line for narrowing to its stated warrant
# (ADR-0077, git) or an anchored filter. The regex and both exclusions are kept. The count is now compared FILE BY
# FILE, with strict equality, against the anchored admission table below. Each row names its pinned count, its raising
# stage and its reason, ADR-0127: rk-containment owns spawning, and its accepted API takes a std::process::Command.
# A new site in an unlisted file fails. An extra site in a listed file fails through the pin. A removed site fails
# through the equality, so no admission can go stale. The PER-SITE instrument is clippy.toml's disallowed-types row
# (std::process::Command, tokio::process::Command) under Cargo.toml's [workspace.lints.clippy] all = deny: every site
# answers it with its own #[expect(clippy::disallowed_types, reason = ...)], as ci.yml has said since 7c. A record
# that moves ADR-0127's API amends this table in the same commit. Reversal (D21-128's): restore the struck line.
cp6_prod_census() {
  git ls-files 'crates/*/src/*.rs' 'crates/*/src/**/*.rs' \
  | xargs grep -nE 'std::process::Command|tokio::process::Command' \
  | grep -vE '^crates/rk-git/src/git_exec\.rs:' \
  | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | cut -d: -f1 | sort | uniq -c | awk '{ print $2" "$1 }' | sort
}
# path | pinned count | raising stage | reason
cp6_prod_table=$(cat <<'CP6_ADMIT'
crates/rk-adapter-claude/src/runner.rs           2  7c  ADR-0127
crates/rk-adapter-toolchain-node/src/run.rs      2  7c  ADR-0127
crates/rk-adapter-toolchain-rust/src/run.rs      2  7c  ADR-0127
crates/rk-cli/src/doctor/probe.rs                1  7d  ADR-0127
crates/rk-containment/src/bounded.rs             2  7c  ADR-0127
crates/rk-containment/src/ffi.rs                 2  7c  ADR-0127
crates/rk-containment/src/posix.rs               1  7c  ADR-0127
crates/rk-containment/src/windows.rs             2  7c  ADR-0127
CP6_ADMIT
)
bad=$(printf '%s\n' "$cp6_prod_table" | awk 'NF && !(NF == 4 && $2 ~ /^[1-9][0-9]*$/ && $3 ~ /^7[cd]$/ && $4 == "ADR-0127")')
[ -z "$bad" ] || { echo "FAIL: an admission row without a pinned count, a raising stage and ADR-0127 (D21-128 (B)):"; printf '%s\n' "$bad"; exit 1; }
cp6_prod_admit=$(printf '%s\n' "$cp6_prod_table" | awk 'NF { print $1" "$2 }' | sort)
test "$(printf '%s\n' "$cp6_prod_admit" | awk '{ s += $2 } END { print s + 0 }')" = 14 \
  || { echo "FAIL: the admission table no longer sums to D21-128's fourteen sites"; exit 1; }
cp6_prod_got=$(cp6_prod_census)
[ "$cp6_prod_got" = "$cp6_prod_admit" ] || { echo "FAIL: process::Command in production code does not equal the pinned ADR-0127 admissions (D21-128 (B)); census (<) against admitted (>):"; diff <(printf '%s\n' "$cp6_prod_got") <(printf '%s\n' "$cp6_prod_admit"); exit 1; }
# PLANTED CONTROLS: the comparison, fed three altered censuses (a spawn in an unlisted file, one more in an admitted
# file, one admitted site removed), refuses each.
for c in unlisted extra removed; do
  case "$c" in
    unlisted) alt=$( { printf '%s\n' "$cp6_prod_got"; echo 'crates/rk-cli/src/cp6_planted.rs 1'; } | sort) ;;
    extra) alt=$(printf '%s\n' "$cp6_prod_got" | awk '$1 == "crates/rk-adapter-claude/src/runner.rs" { $2 = $2 + 1 } 1') ;;
    removed) alt=$(printf '%s\n' "$cp6_prod_got" | awk '$1 == "crates/rk-containment/src/windows.rs" { $2 = $2 - 1 } 1') ;;
  esac
  [ "$alt" != "$cp6_prod_admit" ] || { echo "FAIL: PLANTED CONTROL ($c): an altered census still equals the admission table"; exit 1; }
done
# The git rule stays workspace-wide, tests included — that one IS ADR-0077:70's own sentence.
# ~~test "$(git ls-files '*.rs' | xargs grep -n 'Command::new("git")' | grep -vE '^crates/rk-git/src/git_exec\.rs:' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | wc -l | tr -d '[:space:]')" = 0 \~~
# ~~  || { echo "FAIL: Command::new(\"git\") outside the wrapper"; exit 1; }~~
# (dated 2026-09-30, s8g build, package D21128-8g; D21-128 (A), TAKEN PROVISIONALLY under D21-105; D21-47's
# time-bound rule, quoted above.) The struck line read 21 at bb22b51 and 19 at 66c9c6c: 16 test helpers
# (redkiln-migrate and rk-cli tests/it) and the 3 lint-control fixtures. The 16 now go through
# rk_git::git_exec::run (rk-cli's support::git_run, and redkiln-migrate's git_ok), so ADR-0077 is kept whole and no
# record moves. The line gains ONLY the anchored exclusion ci.yml has carried since s7c RT-13, held in one variable.
# The companion line pins what that variable excludes, through the SAME variable: exactly the three lint-control
# instruments, by file, with strict equality. So a widened exclusion that hides a fourth file fails there.
cp6_lc_re='^crates/[^/]+/tests/fixtures/lint-controls/'
test "$(git ls-files '*.rs' | xargs grep -n 'Command::new("git")' | grep -vE '^crates/rk-git/src/git_exec\.rs:' | grep -vE "$cp6_lc_re" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | wc -l | tr -d '[:space:]')" = 0 \
  || { echo "FAIL: Command::new(\"git\") outside the wrapper"; exit 1; }
cp6_lc_got=$(git ls-files '*.rs' | xargs grep -n 'Command::new("git")' | grep -E "$cp6_lc_re" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | cut -d: -f1 | sort | uniq -c | awk '{ print $2" "$1 }' | sort)
cp6_lc_want=$(printf '%s\n' \
  'crates/rk-adapter-claude/tests/fixtures/lint-controls/adr0077_git_command_control.rs 1' \
  'crates/rk-cli/tests/fixtures/lint-controls/adr0077_git_command_control.rs 1' \
  'crates/rk-cli/tests/fixtures/lint-controls/adr0102_allow_attribute_control.rs 1' | sort)
[ "$cp6_lc_got" = "$cp6_lc_want" ] || { echo "FAIL: the lint-controls exclusion hides something other than the three pinned instruments (D21-128 A4); got (<) against pinned (>):"; diff <(printf '%s\n' "$cp6_lc_got") <(printf '%s\n' "$cp6_lc_want"); exit 1; }
# PLANTED CONTROL: two synthetic hits, one in a tests/it file and one under tests/fixtures/ OUTSIDE lint-controls/,
# survive the git line's filters (so the exclusion is not wider than its anchor), and the lint-controls hit does not.
n=$(printf '%s\n' \
  'crates/rk-cli/tests/it/cp6_planted.rs:1:    let _ = std::process::Command::new("git");' \
  'crates/rk-cli/tests/fixtures/cp6_planted/x.rs:1:    let _ = std::process::Command::new("git");' \
  'crates/rk-cli/tests/fixtures/lint-controls/cp6_planted.rs:1:    let _ = std::process::Command::new("git");' \
  | grep -vE '^crates/rk-git/src/git_exec\.rs:' | grep -vE "$cp6_lc_re" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | wc -l | tr -d '[:space:]')
test "$n" = 2 || { echo "FAIL: PLANTED CONTROL: the git line's filters kept $n of the two non-instrument planted hits, expected 2"; exit 1; }
# THE POSITIVE HALF. Both bans above pass on a workspace with no git exec at all — Halt 8 shape (g),
# a pass condition that is the absence of output. THIS CANNOT FAIL without it.
n=$(grep -c 'Command::new("git")' crates/rk-git/src/git_exec.rs); test "${n:-0}" -ge 1 || { echo "FAIL: the wrapper does not invoke git; the two bans above assert nothing"; exit 1; }

# ---- ADR-0105 clauses 2 and 3. The greps are satisfied by a COMMENT, so each is paired with the
# ---- behavioural test that actually discriminates it. THE GREPS ALONE CANNOT FAIL meaningfully.
n=$(grep -c 'hooksPath' crates/rk-git/src/git_exec.rs); test "${n:-0}" -ge 1 || { echo "FAIL: git_exec.rs never sets core.hooksPath — ADR-0105 clause 2's hooks isolation is absent"; exit 1; }
out=$(cargo test -p rk-git --test it -- trailers::a_message_rewriting_hook_has_no_effect 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the hooks-isolation fault injection (AC-162) did not pass"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(grep -c 'ifexists=add\|if-exists=add' crates/rk-git/src/git_exec.rs); test "${n:-0}" -ge 1 || { echo "FAIL: git_exec.rs never passes --if-exists=add — a repeated identical Gate:/Slice-Verdict: trailer is silently dropped (ADR-0105 clause 3)"; exit 1; }
out=$(cargo test -p rk-git --test it -- trailers::trailer_roundtrip_matches_interpret_trailers 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trailer round-trip over clause 3's five-case matrix did not pass"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- ADR-0091: the record, its verdicts, and the floor.
test "$(ls references/adr/0091-*.md 2>/dev/null | wc -l | tr -d '[:space:]')" = 1 || { echo "FAIL: expected exactly one references/adr/0091-*.md — the S-GIX successor record"; exit 1; }
test "$(grep -cE '^S-GIX/[abc]: (go|no-go)$' references/adr/0091-*.md)" = 3 \
  || { echo "FAIL: ADR-0091 does not carry exactly three S-GIX verdict lines"; exit 1; }
n=$(grep -cE 'git (>=|version) [0-9]+\.[0-9]+' references/adr/0091-*.md); test "${n:-0}" -ge 1 || { echo "FAIL: ADR-0091 states no git-version floor as a number — decision 3 of the preamble's table is undischarged"; exit 1; }
# That grep also matches the record's MEASUREMENT prose ("git version 2.55.0.windows.3 on this box"),
# so decision 3 could read as discharged by a sentence that is not the decision. This test compares
# the number the WRAPPER asserts on against the number the RECORD decides.
out=$(cargo test -p rk-git --test it -- version::the_floor_in_the_code_equals_the_floor_in_the_record 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: version::GIT_FLOOR and ADR-0091's floor have drifted"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- the three arms. The old block echoed on BOTH branches and exited 0 either way, while its
# ---- comment claimed "their verdicts match the record" — nothing compared them. Halt 8 shape (f),
# ---- and THIS CANNOT FAIL. Closed at the source: each arm now reads its own S-GIX/<x> line out of
# ---- ADR-0091 and asserts the behaviour it measured equals it, so rc == 0 MEANS agreement.
out=$(cargo test -p rk-git -- --ignored s_gix_ 2>&1); rc=$?
test "$(printf '%s\n' "$out" | grep -cE '^test .*s_gix_[abc]_')" = 3 \
  || { echo "FAIL: the s_gix_ filter did not select exactly the three arms"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -E '^test .*s_gix_[abc]_'
[ "$rc" -eq 0 ] || { echo "FAIL: an S-GIX arm DISAGREES with the verdict ADR-0091 records"; printf '%s\n' "$out" | tail -30; exit 1; }
echo "S-GIX: all three arms agree with ADR-0091"

# ---- the criteria this step claims that a grep cannot see.
for t in \
  "paths::git_paths_are_drive_letter_form" \
  "reconciled::reconcile_runs_at_every_entry" \
  "worktree::the_resolution_reads_the_store_row_and_not_the_current_directory" \
  "tree::untracked_files_are_excluded_from_the_working_state_tree" \
  "gitops::replay_serves_the_journal_and_performs_zero_writes" ; do
  out=$(cargo test -p rk-git --test it -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: rk-git $t"; printf '%s\n' "$out" | tail -20; exit 1; }
done
for t in \
  "tier_ui::a_state_field_without_a_tier_is_a_compile_error" \
  "contention::two_processes_writing_concurrently_both_land_with_no_lock_errors" \
  "locator::a_duplicate_genuine_heading_is_an_error_and_never_a_first_match" ; do
  out=$(cargo test -p rk-store --test it -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: rk-store $t"; printf '%s\n' "$out" | tail -20; exit 1; }
done

# ---- AC-529's structural half: no production path resolves the run-owned worktree from cwd.
test "$(git ls-files 'crates/*/src/*.rs' 'crates/*/src/**/*.rs' | xargs grep -n 'env::current_dir' \
  | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | wc -l | tr -d '[:space:]')" = 0 \
  || { echo "FAIL: a crate resolves the current directory (AC-529, ADR-0105 clause 8)"; exit 1; }

# ---- PIA-036: the lifted module names the corpus SHA it was lifted from. The lift SHA is a fact about the
# ---- lift and does not move when CORPUS_PIN does (re-scoped 2026-09-17, PROGRESS section 2 D17-2: the line
# ---- read "$CORPUS_SHA", the LIVE pin, and could not pass after 7b.11's single move at ddc57be). Four
# ---- statements: the header names a 40-hex SHA (its first); that SHA is one CORPUS_PIN has carried, in its
# ---- committed history; the lifted file exists in the corpus at that SHA; and the blob the header names is
# ---- that file's blob there.
lift=$(grep -oE '\b[0-9a-f]{40}\b' crates/rk-git/src/reconcile.rs | head -1); test -n "$lift" || { echo "FAIL: reconcile.rs's header names no 40-hex corpus SHA (PIA-036)"; exit 1; }
git log --format= -p -- CORPUS_PIN | grep -qE "^\+CORPUS_SHA=$lift\$" || { echo "FAIL: reconcile.rs names lift SHA $lift, which CORPUS_PIN has never carried (PIA-036)"; exit 1; }
lift_blob=$(git -C "$CORPUS_REPO" rev-parse --verify -q "$lift:runner/src/reconcile.rs") || { echo "FAIL: runner/src/reconcile.rs is not in the corpus at lift SHA $lift"; exit 1; }
grep -q "$lift_blob" crates/rk-git/src/reconcile.rs || { echo "FAIL: reconcile.rs's header does not name the lifted file's blob $lift_blob at $lift"; exit 1; }

# ---- the CI rule Step 5's table assigned to Checkpoint 6. Nothing asserted it had ever landed.
test -f .github/workflows/ci.yml || { echo "FAIL: ci.yml absent"; exit 1; }
grep -q 'Command::new("git")' .github/workflows/ci.yml || { echo "FAIL: ci.yml carries no Command::new(\"git\") structural rule (Step 5's table assigned it here)"; exit 1; }
n=$(grep -c '^      - name:' .github/workflows/ci.yml); test "${n:-0}" -ge 11 || { echo "FAIL: ci.yml has ${n:-<unreadable>} named steps, expected >= 11 (five gates + the structural job's five)"; exit 1; }
```

> **Two Checkpoint-6 lines were narrowed to their warrant on 2026-09-13, at s6 §6.3's run.**
>
> **(a) The `process::Command` ban.** It read `grep -rnE 'std::process::Command|tokio::process::Command'
> crates/`, which bans **every** subprocess spawn anywhere under `crates/` — including `tests/`. Its warrant
> is narrower. ADR-0077:70 says: *"All shelling out goes through one wrapper type inside `rk-git`. **No
> `std::process::Command` for git** exists anywhere else in the workspace."* ADR-0105 clause 5 enumerates the
> **git** write set. Neither forbids a non-git child process.
>
> The wider form was not merely over-strict, it **blocked a criterion**: **AC-181**'s GIVEN is *"two redkiln
> processes running in two worktrees of the same repository, of two different items"* and its Verified-by is
> *"a **two-process** integration test"* — which is also **R-SQLITE**'s empirical half. A ban wider than its
> record would have forced that criterion to be reported as not-dischargeable for a reason no record states.
>
> So the line is now scoped to **production code** (`crates/*/src/**`), comment lines excluded, and the
> `Command::new("git")` line below it stays **workspace-wide including tests** — that one *is* the record's
> own sentence. The git rule is not weakened in any respect; what changes is that a non-git spawn in a test
> is no longer caught by a rule about git. The AC-181 test carries an `#[expect(clippy::disallowed_types)]`
> naming the criterion, so the exemption is a declaration and not a silence.
>
> **(b) The `wc -l` comparisons** gained `| tr -d '[:space:]'`: `wc -l` emits leading whitespace on some
> builds, and `test " 0" = 0` is false.
>
> The same narrowing is mirrored in `.github/workflows/ci.yml`'s structural job, which carries the rule
> between checkpoint runs. **Recorded as a decision, not a tidy-up** — the operator was asked before the line
> moved, and the alternative offered was to leave AC-181 undischarged.
>
> **(c)** *(dated 2026-09-30, `s8g` build, package D21128-8g; D21-128, TAKEN PROVISIONALLY under D21-105 and on the
> confirm list as the answer to the `s8d` close's P7 flag: "flagged for the operator and not decided".)* **Both
> Command-site lines are amended in place under D21-47's time-bound rule, and the block above carries the struck
> text beside the new.** The census, by this block's own pipelines:
>
> - **The git line** read **21** at `bb22b51` and **19** at `66c9c6c`: 16 test helpers and the 3 lint-control
>   fixtures. The 16 sites (redkiln-migrate `tests/it/main.rs` x2; rk-cli `tests/it/` `answer`, `init`, `lane_e2e`
>   x2, `main` x4, `root`, `run_worktree` x2, `slice_verdict`, `support` x2) now route through
>   `rk_git::git_exec::run`, on one support helper per test binary. Their `#[expect(clippy::disallowed_types)]`
>   attributes are gone with the type. The line gains only the anchored exclusion `ci.yml` has carried since s7c
>   RT-13, and a companion line pins exactly the 3 instruments it excludes. The line reads **0**. **ADR-0077 is kept
>   whole, and no record moves.**
> - **The production line** read **14** at `178868f`, at `bb22b51` and at `66c9c6c`. All 14 are non-git spawns in 8
>   files, and all 14 are admitted and pinned, with **0** unadmitted. The warrant is threefold: ADR-0077 is git-only;
>   ADR-0127 makes rk-containment the owner of spawning, and its accepted API takes a `std::process::Command`; and s7c
>   and s7d RAISED this narrowing. The per-site instrument is clippy's `disallowed-types` deny, as `ci.yml` has said
>   since 7c.
>
> **Controls, measured in a private worktree** (`target/wt-d21128`, the routed tree applied over `66c9c6c`, then
> removed). Each turns its line red (rc 1) and goes green again (rc 0) after restore. The baseline is rc 0.
>
> 1. (i) a new git spawn in a new `tests/it` file: rc 1 on the git line.
> 2. (ii) a new git spawn appended to the routed `support.rs`: rc 1 on the git line.
> 3. (iii) the exclusion widened to `tests/fixtures/`, with a planted `tests/fixtures/cp6_ctl_iii/x.rs`: the git line
>    passes, and the pin fails with rc 1.
> 4. (iv) a new `std::process::Command` in an unlisted `rk-cli/src` file: rc 1 on the production line.
> 5. (v) a second one appended to `runner.rs`: rc 1 on the production line.
> 6. (vi) one `windows.rs` site removed: rc 1 on the production line.
>
> Before the routing, the amended lines over the unrouted `66c9c6c` tree read 16 and FAILED (rc 1), which is the red.
> The whole block: three formal runs through the no-dispatch wrapper at 4/4, extracted from this file's bytes (no line of it runs a redkiln binary or mutates git, so none was skipped), each exited 1 at a whole-suite cargo test line on ONE test, the 5 s-bound `run::ac403_gate_open_writes_trailer_and_event_and_exits_zero_without_waiting` (6.2 s, 6.4 s and 9.4 s, each 309 passed / 1 failed), with the operator's IDE running; re-run alone it passes (1 passed, 1.57 s). A DIAGNOSTIC copy skipping only that test in the two whole-suite runs exited 0 and reached its end marker, S-GIX line printed, so every other line, both Command-site lines included, passes. NOT a formal pass: Checkpoint 8g (12)'s replay owns that.

The verdict and floor lines make "a **written S-GIX result**" checkable instead of aspirational, and the
`hooksPath`/`ifexists` greps make two of ADR-0105's rules a build-time fact rather than a sentence. "We built the hybrid so we
didn't need to run it" is not a result; the spike is what licenses the *narrowing*, and an unrun spike leaves
ADR-0077 exactly where it is. If you record the result in `spikes/` rather than an ADR, point those two lines at
that file instead — but point them somewhere, and run them.

**Acceptance set:** **AC-529** first — allocated outside this area's exhausted range by the 2026-09-11 amendment
and claimed by no stage until 2026-09-12: a tier-1 write resolves the run-owned worktree **from the store**, never
from cwd, and the kernel refuses to write when that worktree's HEAD is not the run's branch (ADR-0018 as ADR-0105
clause 8 narrows it — "current worktree" means the worktree the store owns for the run). **No spike measured a
foreign-worktree write**, so this is built and tested, not measured. Then git-as-truth **AC-160–AC-199** (notably **AC-178** common-dir resolution at startup, **AC-184**
`doctor --rebuild` from git alone with its 40-hex corpus-pin assertion, **AC-187** resume); store and typed-artifact
**AC-120–AC-159**; **AC-464** corpus reads through the object store, **AC-465** residency. **AC-013, AC-187,
AC-266, AC-267, AC-271, AC-401 are PROVISIONAL on P8** — multi-step resume is unmeasured; F38 held exactly one step
at n=3, and its 65-min arm, 24-h arm and all three controls are unrun. Build them; do not claim them.

---
