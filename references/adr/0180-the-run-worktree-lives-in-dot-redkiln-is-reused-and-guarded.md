---
id: "0180"
title: "**The run-owned worktree lives at `.redkiln/worktrees/<item>` under the main worktree, is reused on re-entry and never force-removed, and is guarded** (`s8d-ii`'s record; D21-44; D21-45 W1, W2, W4 and W3's carry; D21-46's two corrections and R8; D21-47 V1 and its clause-5 rule; D21-48 X1, X2 and X14): one derivation in `rk_git::worktree` from the main worktree and a validated item id; creation refuses by name when the root is not git-ignored, when a filter a path selects at the fork commit would leave pointer files, when a post-checkout scan finds one, and when the worst-case path computed on the actual item passes 259 characters without `core.longpaths`; an `RK_BASE` marker, a pid-and-start-time lock and a re-entry check; re-entry reuses the tree and discards in-flight changes explicitly; removal only when clean, landed, marked and unlocked, never forced; `init` writes a `# redkiln-runtime` block to `<commondir>/info/exclude`. Supersedes in part `0137` decision 3's path literal, `0105` clause 6 and its reused-worktree forbids member (for the run-owned worktree only), `0105` clause 5's closed write list (for the writes lock, unlock, the `RK_BASE` marker, `reset --hard` and `clean -fd`), and `0107` decision 2 (for the first `init` after this record on a repository initialised before it)"
status: proposed
date: 2026-09-27
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-44 (*"Run-owned worktrees move to **`.redkiln/worktrees/<item>`**, git-ignored."*, N1 and N1b measured), D21-45 (W1, W2, W3 and W4, and *"Where redkiln deliberately differs"*), D21-46 (R1, R8, and its two corrections to D21-44 and D21-45), D21-47 (V1, and *"ADR-0105 clause 5 has one reading"*), D21-48 (X1, X2 and X14), D21-49 (the flip-stop this record's checkpoint carries) and D21-64 ruling 10 (this record's flip condition); runbook/phase-1/s8d-ii-run-worktrees.md (the records table, the supersession-target quotations, §8d-ii.0's stage-open measurements re-taken 2026-09-27 at `83cf836`, and §8d-ii.1 to §8d-ii.7); runbook/phase-1/s8c-toolchain-port-and-verbs.md §8c.4 (the `.worktreeinclude` reader) and §8c.12 item 3 (the pnpm junction skeleton a removal leaves); runbook/halts.md Halt 6; references/2026-09-08-acceptance-criteria.md AC-518, AC-522, AC-529 and AC-531; references/adr/0018, 0105 (clauses 5, 6 and 8, its forbids bullet and its rejected option *"reuse worktrees across runs"*), 0107 (decision 2), 0109 (decision 4), 0137 (decision 3) and 0162 (decision 10); crates/rk-cli/src/verbs/run.rs (`recreate_run_worktree`, `reference_worktree`, `remove_reference_worktree`), crates/rk-cli/src/verbs/init.rs (its module doc), crates/rk-git/src/git_exec.rs (the one process-exec site) and crates/rk-containment/src (`is_alive`), as read 2026-09-27 at `83cf836`
supersedes: ["0105", "0107", "0137"]
superseded_by: []
related: ["0018", "0083", "0085", "0092", "0098", "0109", "0162", "0164", "0170"]
evidence: []
consumers: []
---

## Context and problem

**The implementer cannot edit a run worktree where it lives today.** D21-44 measured it on 2026-09-24. A `-p` child
with `--permission-mode acceptEdits --permission-prompts none` left `probe.txt` reading `original` under
`.git/redkiln/worktrees/probe-n1-in`, with *"**Edit AND Write**, both refused: *"this session has no approval surface
for permission prompts"*"*. The same dispatch under `.redkiln/worktrees/probe-n1b` wrote it (*"`.redkiln/worktrees/probe-n1b`
(Claude-style, in-repo) | **`edited`** | **none**"*). D21-44 draws the consequence: *"**the implementer could never
edit a file in any run-owned worktree as placed today**, and every real run would have failed at implement"*. D21-45
explains why: Claude Code's `checkPathSafetyForAutoEdit` treats a path as sensitive if *"ANY segment"* is one of its
list, and `.git` is on that list while `.redkiln` is not.

**The path is an accepted record's literal.** ADR-0137 decision 3 reads *"**`project_root` is the run-owned
worktree** — `<common-dir>/redkiln/worktrees/<item>`, the value `run.rs:223` binds as `owned`"*. D21-46's first
correction says so: *"ADR-0018 and ADR-0105 name no worktree path. The literal `<common-dir>/redkiln/worktrees/<item>`
is **ADR-0137 decision 3's**."* ADR-0018 places the **store**, *"One store per repository lives at `git rev-parse
--git-common-dir`/redkiln/"*, and its Decision's *"refuses to write when the current worktree's HEAD is not the run's
branch"* names no path. Neither sentence moves.

**The run worktree is destroyed at every run start.** `recreate_run_worktree`'s doc reads *"**Re-created, never
reused** (ADR-0105 clause 6; H-5 / RT-47)"*, and its body force-removes the tree and then calls
`std::fs::remove_dir_all` on every run. That is ADR-0105 clause 6 as written: *"**6. Worktrees.** One worktree per
concurrent item (Halt 6). A worktree is **re-created**, never reused, per run"*, with its forbids bullet *"Forbids a
wrapper write not on clause 5's list, a `\\?\` path handed to git, and a reused worktree."* So a resumed run's tree
never survives to be resumed. D21-45 W2 reverses that: *"**Reuse, never force-remove.**"*

**Nothing guards the tree's identity.** There is no item-id validation, no symlink guard, no ownership marker, no
lock and no re-entry check, and the one git exec site sets no locale. D21-45 read Claude Code's own version of each
from its installed code, and W1 adopts them all: *"**Adopt all:** item-id validation as a path and branch component
(at most 64 characters, safe segments, no `.`, `..` or `.git`); symlink guards on `.redkiln`, `.redkiln/worktrees` and
`<item>`, plus a post-create realpath check; an **`RK_BASE`** marker in the worktree's gitdir, so ownership is
recoverable from git alone (ADR-0109 d4); `git worktree lock --reason "redkiln run <item> (pid N start T)"` with
stale-lock release; a re-entry check (a separate checkout, registered to this repo); `LC_ALL=C` on every parsed git
call; Windows removal never follows a junction, with a test."*

**A run tree inside the checkout is a full checkout inside the checkout.** Without an ignore row, `git status
--porcelain` in the user's checkout lists it, and AC-518's instrument (*"`git status --porcelain` empty, `git
diff-index --quiet HEAD` clean, and HEAD unmoved"*) fails on a correct run. D21-46 flagged the behaviour that follows:
*"The run-worktree root must be git-ignored before creation proceeds, which makes AC-518's instrument depend on an
ignore row."* D21-48 X1 took it: *"**Refuse by name.** This answers D21-46's flip-line flag."*

**A checkout can yield pointer files.** D21-45 W4 refuses creation *"when a repo-local filter driver (for example LFS
via `--local`) or an `includeIf` would yield pointer files"*, and D21-46 R8 widens the read to *"**The effective config
at every scope.** Refuse with a diagnosis naming the scope."* Read literally, R8 refuses every run on this box: `git
config --show-scope --show-origin --get-regexp '^filter\.'` prints four `system` rows from `C:/Program
Files/Git/etc/gitconfig` (`filter.lfs.clean`, `.smudge`, `.process`, `.required true`), Git for Windows' default, for
every repository [measured 2026-09-27 at `83cf836`]. D21-47 V1 settles it: *"**Refuse only when pointer files would
result.** That is, when a tracked path at the fork commit selects a filter whose driver is NOT effective at any scope,
or smudge is skipped."* The one case a configuration read cannot see, an effective driver whose command is not
installed, D21-48 X2 closes by observation: *"**A post-checkout scan.** After creation, the paths the filter selects
are scanned for LFS pointer headers; any hit refuses by name and removes the fresh worktree."*

**This is one record, and it takes the decisions already taken.** D21-44, D21-45 W1, W2, W4 and W3's carry, as
corrected and extended by D21-46, refined by D21-47 and answered by D21-48, are the rulings. `s8d-ii`'s records table
names the supersession targets and requires each to be quoted. Where this record moves a `## Decision` sentence or a
forbids bullet of an accepted record, it is a supersession in part (ADR-0092 rule 1: *"A correction that would move
either is a supersession and takes a new record."*), never an erratum.

## Decision drivers

- An implementer dispatched under `--permission-prompts none` must be able to edit the run tree (D21-44, measured).
- A resumed run must find its own tree (D21-45 W2), and a tree redkiln did not make must never be removed by it.
- The user's checkout must read clean with the run tree inside it (AC-518), and nothing may dirty it by existing.
- A run tree must never materialise pointer files that creation could have refused (D21-47 V1, D21-48 X2).
- Every guard must be observable from git and the filesystem alone, so that ownership survives a lost store
  (ADR-0109 decision 4).
- The records this moves must be superseded in part, each quoted, and the ones it only cites must stay whole.

## Considered options

### Option — `.redkiln/worktrees/<item>` under the main worktree, reused on re-entry, guarded (taken)

D21-44's placement, with D21-45 W1's guards, W2's lifecycle, W4's hygiene and W3's carry at creation, as D21-46,
D21-47 and D21-48 correct, refine and answer them. The root is the **main** worktree, so the placement does not
depend on the checkout `run` starts in. Taken, because N1b measured it writable and every other option below fails a
driver.

### Option — keep `<common-dir>/redkiln/worktrees/<item>`

ADR-0137 decision 3's placement, unchanged. Declined on measurement: N1's in-arm left the file `original` with *"**Edit
AND Write**, both refused"*. Every real run would fail at implement. The protected-segment rule is Claude Code's, and
redkiln cannot configure it away under `--permission-prompts none`.

### Option — a sibling directory outside the checkout

A run tree at a path beside the repository, outside it. D21-44: *"The sibling-directory alternative was offered and
declined."* It also puts a run's output where no ignore row of the user's repository reaches it, and where the
repository's own tooling cannot find it.

### Option — keep ADR-0105 clause 6: re-create the run tree on every run

`recreate_run_worktree` as it stands. Declined by D21-45 W2: it force-removes a tree whose in-flight state a resumed
run needs, and *"It deliberately does not copy Claude's `-B`, which resets an existing branch and would destroy a
resumable run."* ADR-0105's own reason for rejecting reuse is answered in decision 9 below, in that option's own
terms.

### Option — refuse creation whenever a filter driver is declared at any scope

D21-46 R8 read literally. Declined by D21-47 V1: it refuses the working case, an installed driver at `system` scope
(BrittonGames' shape, and every repository on this box), and it would admit a selected driver declared nowhere, which
is the case that yields pointer files.

### Option — warn, rather than refuse, when the root is not git-ignored

Creation proceeds and prints a warning. Declined by D21-48 X1, *"**Refuse by name.**"*: a run that dirties the user's
checkout by existing fails AC-518's instrument on a correct run.

## Decision

1. **The run-owned worktree lives at `.redkiln/worktrees/<item>` under the main worktree's directory, on
   `rk-run/<item>`, and it is derived in one place.** The root is the path in the **first** `worktree` record of `git
   worktree list --porcelain`, which git lists as the main worktree, whichever checkout `run` starts in. A repository
   whose first record carries `bare` has no main worktree, and creation refuses it by name. `rk_git::worktree` owns the
   one derivation: it takes the main worktree's directory and a validated item id (decision 3) and returns the path and
   the branch together. Every call site, and every test that plants a tree under the root, calls it. The review
   reference tree is `<item>.review-ref` in the same directory (decision 10).
   - **Supersedes ADR-0137 decision 3 in part: its path literal only**, *"`<common-dir>/redkiln/worktrees/<item>`"*.
     Its rule, *"**`project_root` is the run-owned worktree**"*, stands.
   - **ADR-0018 is cited and not superseded.** It names no worktree path (D21-46's first correction).
   - The one existing tree at the old root, `rk-fix-doctor-test-count` at `73c087a`, stays where it is. D21-44: *"its
     re-point is s8j's"*.

2. **Creation refuses by name when the root is not git-ignored** (D21-48 X1, *"**Refuse by name.** This answers
   D21-46's flip-line flag."*). `git check-ignore`, run in the main worktree, must report `.redkiln/worktrees/<item>` as
   ignored, or creation refuses and names the path. There is no warning branch. This repository carries the row in its
   tracked `.gitignore` (D21-44), a target repository gets it from `init`'s exclude block (decision 14), and every
   fixture repository a run reaches carries it. AC-518's `git status --porcelain` holds through these rows.

3. **The item id is validated once, as a path component and as a branch component**, in the derivation. It is at most
   64 characters; its one segment matches `^[a-zA-Z0-9._-]+$`; it is not `.`, `..` or `.git`, case-folded; it has no
   leading or trailing `.`, no `..` anywhere and no `.lock` suffix (git's ref rules for `rk-run/<item>`); it is not a
   Windows reserved device name, case-folded, with or without an extension (`CON`, `PRN`, `AUX`, `NUL`, `COM1` to
   `COM9`, `LPT1` to `LPT9`); and it has no `.review-ref` suffix, case-folded, which would collide with another item's
   reference tree. An id that case-folds onto a registered run tree or an existing `rk-run/` branch without equalling
   it is refused, because Windows' filesystem folds case and git's refs do not.

4. **Symlink guards and a post-create realpath check.** Creation refuses when `.redkiln`, `.redkiln/worktrees` or
   `<item>` is a symlink or, on Windows, a junction. After `git worktree add`, the created tree's canonical path must
   lie under the canonical `.redkiln/worktrees/`, or the tree is refused and the refusal names both paths. The
   canonical form is used only to compare, and it is never handed to git: git receives the drive-letter form (ADR-0105
   clause 6's *"Paths are passed to git in **drive-letter form**, never `\\?\`"*, which stands).

5. **An `RK_BASE` marker in the tree's own gitdir** (the directory `git rev-parse --git-dir` prints from inside it)
   records the fork sha and the item id at creation, so ownership is recoverable from git alone (ADR-0109 decision 4,
   *"`doctor --rebuild` reproduces the stage-level next action from git alone"*, whose premise it extends to the run
   tree). Redkiln never removes a run tree without it. The review reference tree carries one too.

6. **A lock, and a liveness predicate injected by `rk-cli`.** `git worktree lock --reason "redkiln run <item> (pid N
   start T)"` is taken at every run start and released at the run's end, where T is the process's start time as the OS
   reports it. A lock is **live** when its pid is alive **and** that pid's start time equals T. A lock whose pid is
   dead, or whose pid now names a process with a different start time, is released; a live lock refuses the run by
   name. `rk-cli` builds the predicate from `rk_containment::is_alive` and a new `rk_containment::process_start_time`,
   so `rk-git` gains no dependency on `rk-containment`.
   - **`is_alive` becomes a production call.** It is `#[doc(hidden)]` *"Test support"* at this writing, and this
     record makes it production. Its Windows arm's documented ambiguity is an exit code, not pid reuse: *"a process
     that exits with code 259 (`STILL_ACTIVE`'s value) would read alive, the documented ambiguity of this API"*. Such
     a pid reads alive, and if its start time still equals T the run refuses by name. **That is fail-closed**: the
     ambiguity can keep a stale lock held and refuse a run, never release a live one. Pid reuse is what the
     start-time comparison answers.
   - **`process_start_time` is one more call through the binding layer ADR-0127 decision 5 names**, inside
     `rk-containment`'s one `unsafe` module (`GetProcessTimes` on Windows; `/proc/<pid>/stat` on POSIX, ARMED to CI
     with the rest of that arm). It spawns nothing and creates no containment. ADR-0127 is cited and not superseded:
     its one `allow`, its `unsafe` confinement, its dependency sentence and its `windows-sys` features are unchanged.

7. **The re-entry check.** A registered tree is reused only if it is a separate checkout **of this repository**: `git
   rev-parse --git-common-dir` from inside it equals the repository's, and `git worktree list --porcelain` lists its
   path.

8. **`LC_ALL=C` on every git child**, set at the one process-exec site in `rk-git`'s `git_exec.rs`, so no parse
   depends on the operator's locale.

9. **Reuse, never force-remove** (D21-45 W2). This retires `recreate_run_worktree` and its doc's *"**Re-created, never
   reused**"*.
   - **First creation:** `git worktree add --no-track -b rk-run/<item> <path> HEAD`. The fork is the frozen, CI-green
     `HEAD` (D21-45: *"runs fork from the frozen, CI-green HEAD rather than `origin/<default>`"*); its sha is read once,
     before the add, and `RK_BASE` records it.
   - **The branch exists and no tree is registered:** `git worktree add <path> rk-run/<item>`. Never `-B`.
   - **Re-entry:** the registered tree is reused when it carries `RK_BASE`, passes the re-entry check, and its `HEAD` is
     **symbolic** on `refs/heads/rk-run/<item>` and equals that ref's tip. In-flight changes are read path by path
     (`status --porcelain=v1 -z --untracked-files=all`), then discarded explicitly (`reset --hard HEAD`, then `clean
     -fd`, **without `-x`**, so ignored files such as W3's carried files and `setup`'s `node_modules` survive), and
     every discarded path is **returned** in the lifecycle's typed result. That is N10's semantics. Until `s8e` mints
     `worktree_assigned { …, reused, discarded }`, `run` prints each discarded path on stderr, one line per path; from
     `s8e` on, the same result fills that row. W2's *"journalled"* is met at `s8e`, and the interval is stated here.
   - **Refused, never removed:** a tree with no `RK_BASE`, a tree registered to another repository, a tree whose `HEAD`
     has commits the run ref lacks, and a tree on a **detached** `HEAD`. Each refusal names the path and the reason.
   - **Removal** happens only when the tree is clean, landed, marked and unlocked, by `git worktree remove` **without
     `--force`**. Anything else is kept with a reason in Claude Code's form (*"N changed file(s) would be lost"*).
     Removal never follows a junction: a link inside the tree is removed, never its target. A `node_modules` junction
     skeleton a removal leaves (`s8c` §8c.12 item 3) is therefore a link to be removed, never a path to walk. The one
     removal of a tree that is not landed is decision 13's rollback of the tree the current call has just created,
     and it is unforced too. The trigger for removal is `s8e`'s, keyed to the lane's `closed` mark; removal is by run
     state, never by age (D21-45: *"there is no `cleanupPeriodDays` analogue"*).
   - **Landed**, defined. Phase 1 has no integration ref, so a tree is **landed** when (a) its `HEAD` is symbolic on
     `rk-run/<item>` and equals the tip, (b) its status is empty, and (c) the tip's `Story:` trailer is the lane's
     terminal `closed` mark. Every change the tree holds is then on the ref the operator merges, and the lane has ended.
     When a merge queue exists, the record that brings it adds the integration ref's ancestry to (c).
   - **Supersedes ADR-0105 clause 6 in part, for the run-owned worktree only:** *"A worktree is **re-created**, never
     reused, per run"*, and the *"a reused worktree"* member of its forbids bullet *"Forbids a wrapper write not on
     clause 5's list, a `\\?\` path handed to git, and a reused worktree."* The bullet's other two members stand.
     Clause 6's *"One worktree per concurrent item (Halt 6)"*, its typed removal error and its drive-letter rule stand.
     **`redkiln-migrate`'s worktree keeps clause 6 whole**, as AC-522 verifies it: *"left in a state the next run
     refuses to reuse (re-created, never reused — ADR-0105 clause 6)"*.
   - **Halt 6 is answered, not overridden.** Halt 6 reads *"If a fixture tree will not delete, that is the handle, and
     reusing the tree corrupts the next arm."* Reuse here is of a run's **own** tree, by the same item, only when its
     `HEAD` is symbolic on the run ref at its tip, and only after an explicit discard. A tree that will not delete is
     kept and named, and it is never handed to another item or another arm.
   - **ADR-0105's rejected option *"reuse worktrees across runs"* is answered in its own terms.** It was rejected
     because *"a fresh worktree per run is the only state the kernel can assert"*. The state the kernel asserts at
     re-entry is now `RK_BASE` (the fork and the item), a symbolic `HEAD` equal to the run ref's tip, and an empty
     status after the discard. That state is asserted, not assumed, so the rejection's premise no longer holds for the
     run-owned tree.

10. **The review reference tree keeps ADR-0162 decision 10, which is cited and not superseded.** Decision 10 reads
    that the pass *"**removes its own reference worktree on every path, including every refusal path**"*, and its
    forbids bullet reads *"Forbids leaving the reference worktree registered after the pass returns, on any path
    including every refusal path"*. The reference tree is scratch, so it keeps its forced removal inside
    `remove_reference_worktree`, whose directory fallback does not follow a junction. W2's rule is the run-owned
    tree's. Decision 10's contrast clause, *"unlike the run-owned worktree, which `recreate_run_worktree` recreates and
    never removes"*, names a function this record retires; the clause is a description, and the decision's rule
    stands. Two things change:
    - the reference tree is created at the fork sha the run tree's `RK_BASE` records, not at `HEAD`, because under
      reuse the run branch forked at an earlier `HEAD` and the operator's checkout may have moved since;
    - it gets its own `RK_BASE` at creation. A leftover at `<item>.review-ref` that carries the marker is force-removed
      as scratch, as today; one without it is refused by name and kept.

11. **W3's carry, at creation only** (D21-48 X14: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at
    creation."*). After `git worktree add`, the realpath check, the `RK_BASE` write and decision 13's scan, and before
    the lock is released to the run, creation calls `s8c`'s `.worktreeinclude` reader (ignored ∩ matched, symlinks
    skipped, escapes refused) and copies what it returns, then runs `s8c`'s per-toolchain `setup` verb. A `setup`
    failure is `s8c`'s by-name refusal, and the run surfaces it and stops. `settings.local.json` is not copied, no
    `node_modules` symlink is made, and a reused tree is not re-carried.

12. **The filter check refuses only when pointer files would result** (D21-47 V1, refining D21-46 R8). Before `git
    worktree add`, creation reads: (1) the paths that select a filter **at the fork commit**, by `git check-attr
    --source=<fork> filter` over `git ls-tree -r --name-only <fork>`; (2) the drivers at every scope, by `git config
    --show-scope --show-origin --includes --get-regexp '^filter\.'`, with every `include` followed and every matching
    `includeIf` read; (3) every `includeIf` whose condition matches the **run tree** but not the user's checkout, read
    from the file it names and folded into (2); and (4) `GIT_LFS_SKIP_SMUDGE`, which `rk-cli` reads through its one
    environment read and passes in. A driver is **effective** when a `filter.<name>.process` or `filter.<name>.smudge`
    value resolves at some scope. Creation refuses when (a) a selected driver has no effective value at any scope, the
    diagnosis naming the driver, one selecting path and every scope read; or (b) smudge is skipped, by
    `GIT_LFS_SKIP_SMUDGE` set to a value other than `0` or by a selected driver's resolved `smudge` value carrying
    `--skip`, the diagnosis naming the scope and origin file or the variable (R8's *"Refuse with a diagnosis naming the
    scope"*). A selected driver with an effective value, and a driver no path selects, are admitted **silently**; either
    fact goes to the debug level only. BrittonGames' shape (a tracked `.gitattributes` selecting `lfs`, the driver
    effective at `system` scope) is admitted, as `s8i` §8i.0 item 12 says.

13. **A post-checkout pointer scan completes V1 by observation** (D21-48 X2). It runs only on a tree **this call
    created** (first creation, or a branch with no tree registered), in this order: `git worktree add`, the realpath
    check, the `RK_BASE` write, the scan, and only then the lock, the carry and `setup`. Re-entry does not scan. For
    each path decision 12's read (1) names with a driver, it reads the **checked-out file** in the fresh tree, never the
    blob at the fork commit, and a hit is a file whose first line, with one trailing carriage return dropped, is `version
    https://git-lfs.github.com/spec/v1`. It reads the first line's bytes and no more. A pointer header at a path no
    filter selects is not a hit. Any hit refuses by name (the driver, the number of hits, the first hit's path, and the
    scope and origin file of the driver's effective value) and **removes the fresh tree**, under decision 9's rules:
    `git worktree remove` **without `--force`**, on a tree that carries this call's `RK_BASE` and is not yet locked.
    `rk-run/<item>` is kept where it stands. If git refuses the unforced removal, the tree is kept, never forced, and the
    refusal names it as kept.

14. **`init` writes an idempotent `# redkiln-runtime` block to `<commondir>/info/exclude`**, creating the file if
    git's template did not:

    ```
    # redkiln-runtime
    /.redkiln/worktrees/
    # end redkiln-runtime
    ```

    A second `init` finds the marker and leaves the file byte-identical. Lines the operator wrote are never touched,
    and `.gitignore` is never written (D21-45: *"never to `.gitignore`"*). D21-46's second correction reads *"`init`
    writes no hook, and `# redkiln-hook` exists only as doctor's recogniser. The `info/exclude` runtime block is
    therefore `init`'s first write into a target repo's git admin area, and s8d-ii's record says so."* What is true, and
    what this record says, is narrower: `init` already creates the store at `<git-common-dir>/redkiln/store.sqlite3`
    under the common dir, so the block is **`init`'s first write to a file git itself reads**, the first that changes
    what `git status` reports. Its idempotence is therefore new, not inherited.
    - **Supersedes ADR-0107 decision 2 in part**, *"**`init` is idempotent**: a second `init` on an initialised
      repository changes nothing and exits 0."*, for one case only: the first `init` after this record on a repository
      initialised before it appends the block, and so changes something. From the block's first write on, a second
      `init` changes nothing and exits 0 again. The rest of ADR-0107 stands.
    - **AC-531 does not move.** Its 2026-09-18 amendment scopes the naming lint to *"the working tree, git refs and
      hooks"* and reads that *"the git common dir is not user-visible namespace"*. `info/exclude` sits in the common
      dir and outside `hooks`, so the lint's surfaces do not change. `init` still installs no hook (AC-412).

15. **ADR-0105 clause 5 is superseded in part, for the writes only** (D21-47: *"A git WRITE not on its closed list is a
    supersession in part"*, and *"A READ is not a supersession; s8d-ii drops its reads from its supersession"*).
    Clause 5 reads *"**5. The wrapper's write set is enumerated**: `worktree add`/`remove`; `add`; `commit`"* … and
    *"Anything not listed is a successor."* This record adds to the list, each on the owned tree only: `worktree lock`
    and `worktree unlock`; the discard's `reset --hard HEAD` and `clean -fd`; and the `RK_BASE` marker, which is a file
    `rk-git` writes into the tree's gitdir rather than a git invocation, named because it writes into git's admin area.
    `worktree add` (with `--no-track -b`, with an existing branch, and `--detach` for the reference tree) and `worktree
    remove` are already on the list. **The reads are not a supersession:** `worktree list --porcelain`, `rev-parse
    --git-common-dir`, `status --porcelain=v1 -z`, `config --show-scope --show-origin --includes --get-regexp`,
    `check-attr --source=<fork>`, `ls-tree` and `check-ignore` each get a `GitOp` variant and supersede nothing.
    `init`'s `info/exclude` append is `rk-cli`'s, not a wrapper write; it is decision 14's supersession of ADR-0107,
    not clause 5's. Every git invocation among them runs through `GitOps`, and on replay none runs.

16. **The worst-case path is bounded on the item being created.** The bound is `len(root) + 20 + len(item) + 11 + 1 +`
    the longest path tracked at the fork commit, where `root` is the main worktree's directory, 20 is
    `/.redkiln/worktrees/`, and 11 is `.review-ref`, the longer of the two trees' names. Creation refuses by name when
    the bound exceeds **259** (Windows' 260-character `MAX_PATH` leaves 259 usable) and `core.longpaths` is not `true`,
    rather than check out a partial tree. The bound is computed on the **actual** item, never on the 64-character
    maximum: a rule on the maximum leaves a 23-character root budget, which a throwaway clone under a temporary
    directory could never meet.

17. **Every recursive walk over `.redkiln/` excludes `worktrees/`, with a synthetic control**, because a run tree carries
    a full copy of `.redkiln/`. At this writing no code under `crates/*/src` walks `.redkiln/` recursively: each reader
    takes one named subdirectory one level deep. A walk added later takes the exclusion and its control in the same
    commit.

## Consequences

**Positive**

- An implementer under `--permission-prompts none` can edit the run tree (N1b, measured). Inside it,
  `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/` stay protected (D21-45's standing limit), and `s8h`
  maps an edit there to the typed `permission_denials` failure.
- A resumed run finds its own tree with its ignored files, and its discarded in-flight paths are named rather than
  lost silently.
- Ownership is recoverable from git alone, through `RK_BASE`, and nothing redkiln did not create is ever removed.
- No run tree materialises pointer files that creation could have refused or observed, and a working installed driver
  is admitted without noise.

**Negative**

- The run tree sits inside the user's checkout, so the ignore rows are load-bearing: a repository without one refuses
  creation, and every fixture repository a run reaches must carry the row.
- Between this record and `s8e`, the discard is reported on stderr, not journalled.
- `is_alive`'s exit-code-259 ambiguity can keep a stale lock held; the run then refuses by name rather than proceed.
- Removal has no trigger until `s8e` mints the `closed` mark, so run trees accumulate until then; each is kept with its
  reason.
- `git worktree remove` on Windows can leave a pnpm `node_modules` junction skeleton behind under the removed path
  (`s8c` §8c.12 item 3). Removal never follows it, but the skeleton is not a registration, so `git worktree list`
  cannot see it.
- The worst-case bound holds a four-character margin for a 64-character item on this repository (255 of 259), and a
  target repository with a longer root or deeper paths has less.
- `init`'s first run after this record on a repository initialised before it changes a file, which ADR-0107 decision 2
  forbade until this record.

**What this forbids**

- Forbids deriving a run tree's directory anywhere but `rk_git::worktree`'s one derivation, and forbids deriving the
  root from `--show-toplevel` or from the common dir.
- Forbids creating a run tree under a root git does not report as ignored, and forbids a warning in place of that
  refusal.
- Forbids an item id that is not a safe path and branch component under decision 3's rules.
- Forbids handing git a canonicalised `\\?\` path, and forbids creation through a symlinked or junctioned `.redkiln`,
  `.redkiln/worktrees` or `<item>`.
- Forbids removing a run tree that carries no `RK_BASE`, that is dirty, unlanded or locked, or that redkiln did not
  create, and forbids `worktree remove --force` or `remove_dir_all` on a run-owned tree.
- Forbids `-B` on `rk-run/<item>`, and forbids `clean -x` in the discard.
- Forbids re-entering a tree on a detached `HEAD`, a tree registered to another repository, or a tree whose `HEAD` has
  commits the run ref lacks.
- Forbids releasing a lock whose pid is alive with its recorded start time.
- Forbids refusing creation on a filter driver that is effective and not skipped, and forbids admitting a selected
  driver that is effective at no scope or whose smudge is skipped.
- Forbids reading filter attributes from the working tree rather than the fork commit.
- Forbids the post-checkout scan judging or removing a tree the current call did not create.
- Forbids `init` writing `.gitignore`, touching a line the operator wrote in `info/exclude`, or appending the block
  twice.
- Forbids computing the path bound on anything but the item being created.
- Forbids reading this record as superseding ADR-0018, ADR-0162 decision 10, or ADR-0105 clause 6 for
  `redkiln-migrate`'s worktree.

## Evidence and confidence

- [measured 2026-09-24, D21-44] N1 and N1b: the in-`.git` arm refused Edit and Write; the `.redkiln/worktrees/` arm
  edited; each control succeeded in the same run. Total spend $0.123.
- [measured 2026-09-24, D21-45] Claude Code's own worktree conventions, quoted from the installed CLI's code at
  2.1.281: the name checks, the symlink guards, the `CLAUDE_BASE` marker and lock, the removal rule, the re-entry
  check, `.worktreeinclude`, and the `# claude-code-runtime` exclude block.
- [measured 2026-09-27 at `83cf836`, `s8d-ii` §8d-ii.0] `git worktree list --porcelain` lists
  `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count` at `73c087a82245b342811208655323ae4f49bf879b`,
  whose `Story:` trailer reads `redkiln-rs/rk-fix-doctor-test-count`. The longest tracked path is 140 characters
  (`references/adr/0114-…`), so the bound is **255** for a 64-character item under `D:/repos/redkiln-rs` (39 + 64 + 11
  + 1 + 140), **215** for `rk-fix-doctor-test-count`, and **246** for that item under a `mktemp -d` clone root in
  drive-letter form plus `/c` (50 characters); `git config --get core.longpaths` exits 1. `git check-attr
  --source=HEAD --stdin filter` over every tracked path selects no driver, and `--get-regexp '^includeif\.'` exits 1.
  Every `read_dir` under `crates/*/src` that touches `.redkiln/` reads one named subdirectory (`cards`, `specs`,
  `skills`, `catalog`, `processes`) or a directory it is handed; `init`'s `preexisting_rk_names` is top level only.
  `init.rs`'s module doc reads *"It does exactly two things"* and *"installs no hook (AC-412)"*, and the only `#
  redkiln-hook` in code is `doctor`'s `const MARKER`.
- [documented, `references/adr/0105`, `0107`, `0137`, `0162`, `0018`] each sentence superseded or cited is quoted
  above from its own file, as read 2026-09-27.
- [documented, `crates/rk-containment/src`] `is_alive`'s *"Test support"* status and its Windows arm's 259 note.
- **Confidence.** High for the placement, the lifecycle and the guards: each was measured or read from the shipped
  code of a tool that already runs them. The filter check's reach is as wide as the configuration read's; the scan
  closes the one case it cannot see, and only for the LFS pointer header.
- **What this evidence does not license.** It does not license claiming that a real run has edited a run tree at the
  new root: N1b was a probe, and the first real dispatch is `s8i`'s. It does not license any claim about a filter
  driver whose pointer format is not LFS's. It does not license removing the legacy tree at the old root, which is
  `s8j`'s.
- **Status.** Written `proposed` at `s8d-ii`'s build under the AFK rule. It flips at `s8d-ii`'s close under D21-64
  ruling 10, if the formal Checkpoint 8d-ii passes whole and a refuting round is dry, on its own dated ledger line. The
  `superseded_by` back-links on ADR-0105, ADR-0107 and ADR-0137 are written in the flip commit, not with this record,
  and Checkpoint 8d-ii (9) asserts whichever state this record's status requires.
