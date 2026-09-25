# Step 8d-ii — Run worktrees (Phase 1, part 8d-ii)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8d-ii` | Phase 1 | Checkpoint 8d | Checkpoint 8d-ii | `s8d` | `s8e` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§8d-ii.N` in this file means this file's own section; another stage's section is cited as `s8e §8e.4`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Authored 2026-09-24 by splitting s8e (D21-46 R1), at `c5d19d7`.* What came from where:
- `s8e`'s former §8e.10 (D21-44, the relocation) is §8d-ii.1 here. Its former `s8e` §8e.11 (D21-45 W1, W2 and W4, and W3's
  carry) is §8d-ii.2 to §8d-ii.6.
- The records-table row *"the run worktree lives at `.redkiln/worktrees/<item>`"*, its attendant edits, its 30 named
  tests, its eleven mutants, its findings and the checkpoint lines that asserted it (the worst-case path, the ignore
  row, the root, `recreate_run_worktree`, `LC_ALL` and `init`'s marker) moved with it.
- **Two of `s8e`'s Checkpoint 7f amendments moved too** (§8d-ii.7): the run arm in a throwaway clone, and the deleted
  cleanup line. This stage's binary is the one that would break 7f without them.
- **Applied here, and not in `s8e`'s text:** D21-46's correction of D21-44's supersession targets, its correction of
  D21-45's `init` premise, **R8** (the filter-driver check reads the effective config at every scope), its flag on
  the ignore row, the `RK_CP_NESTED` convention and the replay no-dispatch guard.

---

**Why this is a stage and not two sections.** D21-46 R1: *"**Split.** A new stage **`s8d-ii`, "Run worktrees"**, sits
between s8d and s8e (the `7b-ii` precedent). It carries D21-44's relocation and D21-45 W1/W2/W4. Its checkpoint is
s8e's entry gate."* The precedent is `s7b-ii`'s own framing: *"The split is **execution order**, which this runbook
owns; it decides nothing the record does not"*. The same holds here. The worktree record still takes every decision
it took inside `s8e`. `s8e` §8e.4's `worktree_assigned` row records a path, so the path has to be final before `s8e`
opens. `INDEX.md`'s stage table, `PROGRESS.md`'s tracker row and `s8d`'s `next` column move in the same change, and
this file does not make those edits (§8d-ii.9, finding 11).

**Why this step exists.** Measured at `a6349b6` for `s8e`'s revision, and re-read at `c5d19d7` for this file:

- **The implementer cannot edit a run worktree where it lives today.** D21-44 measured it. A `-p` child with
  `--permission-mode acceptEdits --permission-prompts none` left `probe.txt` reading **`original`** under
  `.git/redkiln/worktrees/probe-n1-in`, with *"**Edit AND Write**, both refused"*. The same dispatch under
  `.redkiln/worktrees/probe-n1b` wrote it. D21-44 draws the consequence: *"**the implementer could never edit a file
  in any run-owned worktree as placed today**, and every real run would have failed at implement"*.
- **The run worktree is destroyed at every run start.** `recreate_run_worktree`'s doc reads *"**Re-created, never
  reused** (ADR-0105 clause 6; H-5 / RT-47)"*. Its body runs git with `std::ffi::OsStr::new("--force")` and then
  `let _ = std::fs::remove_dir_all(&path);` on every run. `remove_reference_worktree` does the same to the review
  reference tree. So a resumed run's tree never survives to be resumed, and D21-45 W2 reverses that.
- **Nothing guards the tree's identity.** No item-id validation, no symlink guard, no ownership marker, no lock and no
  re-entry check exist. `GitExec`'s one process-exec site, `std::process::Command::new("git")` in `run_with_env`, sets
  no locale. D21-45 read Claude Code's own versions of each of these, and W1 adopts them all.

**This step takes decisions, and they were already taken.** D21-44, and D21-45 W1, W2 and W4 and W3's carry, as
corrected and extended by D21-46 (its two corrections, R8, and the flag on the ignore row), are all in
`spikes/DECISIONS-2026-09-21.md`. They are written as one record at this stage. It takes **the next free id at
writing**, and this file assigns none. Where the record moves a `## Decision` sentence or a forbids bullet of an
accepted record, it is a supersession in part (ADR-0092 rule 1), never an erratum. It is written `proposed` and flips
only on the operator's dated ledger line.

**What this step owns:**
- **D21-44** whole, including its *"Consequences"*. The ledger says *"owned by `s8e`"*, and D21-46 R1 moves them here.
- D21-45 **W1**, **W2**, **W4** (the worktree half, and `init`'s exclude block) and **W3's carry at creation**.
- D21-46's first correction (the supersession targets), its second correction (`init`'s first write), **R8**, and the
  flagged ignore-row behaviour.

**What it does not own, and who does.**
- **`s8e`:** the `worktree_assigned` row that records the path, `reused` and `discarded` (`s8e` §8e.4). It also owns
  every tier-1 write and the lane's `closed` mark, to which removal is keyed (`s8e` §8e.6). Until `s8e` lands, this
  stage's discard list reaches the operator on stderr, not a journal row (§8d-ii.3).
- **`s8c`:** the `.worktreeinclude` reader and the per-toolchain `setup` verb (D21-45 W3; D21-43 N25). This stage calls
  both at creation and re-specifies neither.
- **`s8h`:** the live probe's edit inside `.redkiln/worktrees/<probe>`, and the protected-segment limit. D21-45 puts it
  this way: *"inside a run worktree, `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/` stay
  protected"*, which `s8h` maps to the typed `permission_denials` failure.
- **`s8a`:** how the `Text` variant classifies an LFS pointer path. §8d-ii.5's filter-driver refusal is what it
  verifies against.
- **`s8b`:** its own catalog-quality walks over `.redkiln/`, which exclude `worktrees/` (D21-44).
- **`s8j`:** re-pointing or retiring the legacy `rk-fix-doctor-test-count` tree (D21-44: *"its re-point is s8j's"*),
  and every amendment to `s8-exit-criterion.md`, including its `grep -r … .redkiln/` lines, which must exclude
  `worktrees/` (§8d-ii.1).

**No stage before `s8i` dispatches to a real account, and this one dispatches nothing.** Every run in it is over a
fixture repository or the fixture child. Checkpoint 8d-ii's replays run under the no-dispatch guard (§8d-ii.11).

**Order of work, and why.**
1. §8d-ii.0's measurements, before any code.
2. The one derivation, the item-id validator and the ignore check (§8d-ii.1, §8d-ii.2), because every other guard
   takes the validated item and the derived path.
3. The lifecycle: create, re-enter, refuse and remove (§8d-ii.3), then the carry (§8d-ii.4).
4. Git hygiene: the effective-config filter check, `init`'s exclude block and `LC_ALL` (§8d-ii.5, §8d-ii.2).
5. The fixture repositories' ignore rows and Checkpoint 7f's two amendments, in the commit that moves the root
   (§8d-ii.1, §8d-ii.7).

Every test of a new behaviour is written **first, red** (§8d-ii.8).

## Records this step writes

The record is **a new record (next free id at writing)**, in the ADR-0085 `### Option — <name>` heading form that
D21-42 24a makes binding from ADR-0164 onward. It may not drop a supersession listed here without a reason written
into the record.

| record | executes | supersedes in part | cites without superseding |
| --- | --- | --- | --- |
| **the run worktree lives at `.redkiln/worktrees/<item>`, is reused, and is guarded** | D21-44; D21-45 W1, W2, W4, and W3's carry; D21-46's two corrections and R8 | **ADR-0137 decision 3's path literal** *"`<common-dir>/redkiln/worktrees/<item>`"*. Its rule, *"**`project_root` is the run-owned worktree**"*, stands. **ADR-0105 clause 6's** *"A worktree is **re-created**, never reused, per run"*, and the *"a reused worktree"* member of its bullet *"Forbids a wrapper write not on clause 5's list, a `\\?\` path handed to git, and a reused worktree."* (W2). **ADR-0105 clause 5's closed list**, *"The wrapper's write set is enumerated"* … *"Anything not listed is a successor"*, for the lock, marker and discard operations §8d-ii.6 names | **ADR-0018, NOT superseded** — it names no worktree path (§8d-ii.1). ADR-0105 clause 8; ADR-0105's rejected option *"reuse worktrees across runs"* (answered in its own terms, §8d-ii.3); ADR-0107 decision 2 (extended to `info/exclude`); ADR-0109 decision 4; AC-518, AC-529 and AC-531 (dated notes); Halt 6 |

**The supersession targets are D21-46's, verified by quotation against `references/adr/` at `c5d19d7`.** D21-46:
*"**D21-44 named the wrong records to supersede.** ADR-0018 and ADR-0105 name no worktree path. The literal
`<common-dir>/redkiln/worktrees/<item>` is **ADR-0137 decision 3's**. What also moves in ADR-0105 is **clause 6** and
its "reused worktree" bullet (D21-45 W2), and **clause 5**'s closed git-op list (the new lock and marker operations).
s8d-ii's placement record supersedes those, not ADR-0018."* Each target, read in its own file:
- `references/adr/0137-the-real-toolchain-adapter-is-built-at-the-composition-root.md`, decision 3: *"**`project_root`
  is the run-owned worktree** — `<common-dir>/redkiln/worktrees/<item>`, the value `run.rs:223` binds as `owned`"*.
- `references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md`, clause 6: *"**6. Worktrees.** One worktree per
  concurrent item (Halt 6). A worktree is **re-created**, never reused, per run"*. Its forbids bullet: *"Forbids a
  wrapper write not on clause 5's list, a `\\?\` path handed to git, and a reused worktree."*
- The same file, clause 5: *"**5. The wrapper's write set is enumerated**: `worktree add`/`remove`; `add`; `commit`"* …
  *"Anything not listed is a successor."* Lock, unlock, reset and clean are not on it.
- `references/adr/0018-rewrite-events-and-common-dir-worktree-ownership.md` places **the store**, not a worktree:
  *"One store per repository lives at `git rev-parse --git-common-dir`/redkiln/"*. Its `## Decision` says the kernel
  *"refuses to write when the current worktree's HEAD is not the run's branch"*. Neither sentence moves.

**Flagged for the operator's flip line, and not decided here** (D21-46: *"The run-worktree root must be git-ignored
before creation proceeds, which makes AC-518's instrument depend on an ignore row. That is a behaviour no earlier line
names (s8d-ii)."*). The record states the refusal (§8d-ii.1) and names it for the flip line. If the operator declines
it, the refusal becomes a warning and `lane_e2e`'s AC-518 run (`s8e` §8e.11) carries the ignore row as a fixture
precondition instead.

**Attendant edits, each dated and each in the commit that makes it true:**
- design §5's `reconcile_started` row, whose recomputation clause names *"`<common-dir>/redkiln/worktrees/<item>/`"*;
- dated register notes on **AC-518**, **AC-529** and **AC-531** (§8d-ii.1, §8d-ii.5);
- dated notes on `s7d` §7d.5's two tables that name the old path;
- the rigged stub's doc in `rk-fixture-toolchain.rs`, which names the candidate arm's directory as
  *"`…/worktrees/<item>`"*. `s8c` names this correction as the path move's;
- the dated amendment to Checkpoint 7f that §8d-ii.7 spells out;
- a ledger entry.

### 8d-ii.0 Stage-open measurements — re-taken at Checkpoint 8d's close, before any code

Each is read-only. A 🛑 line is a STOP: the finding goes to the operator, and the stage does not choose around it.

1. **Where the legacy tree is registered.** At `c5d19d7`, `git worktree list` lists
   `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count` at `73c087a`. `s8a` runs Checkpoint 7f
   whole at its exit, and 7f's block, as `s8a` leaves it, ends with `git worktree remove --force
   .git/redkiln/worktrees/rk-fix-doctor-test-count`. So by this stage's open the registration may already be gone
   (§8d-ii.9, finding 4). Record which. Checkpoint 8d-ii asserts the ref and its mark, never the registration.
2. **The worst-case run-tree path (D21-44's last consequence).** At `c5d19d7` the longest tracked path is 140
   characters, `references/adr/0114-the-executed-once-gate-moves-to-extended-to-shipped-promotion-and-the-v0-pack-tags-on-lint-parse-and-a-visible-ledger.md`.
   `D:/repos/redkiln-rs/.redkiln/worktrees/` is 39 characters. The derived review tree is named
   `<item>.review-ref` (`reference_worktree`'s `format!("{item}.review-ref")`), so the worst case is 39 + 64 + 11 +
   1 + 140 = **255**. Windows' 260-character `MAX_PATH` leaves 259 usable, and `git config --get core.longpaths`
   exits 1 with no value on this box. Under the old root (43 characters) the same sum was **259**, at the limit.
   Checkpoint 8d-ii recomputes the sum from the tree.
3. 🛑 **The effective filter configuration on this box (R8).** At `c5d19d7`, `git config --show-scope --show-origin
   --get-regexp '^filter\.'` prints four `system` rows from `C:/Program Files/Git/etc/gitconfig`:
   `filter.lfs.clean`, `filter.lfs.smudge`, `filter.lfs.process` and `filter.lfs.required true`. That is Git for
   Windows' default, present for **every** repository on this machine. `git check-attr filter` selects no driver for
   any tracked path of this repository, and `--get-regexp '^includeif\.'` exits 1. **Read literally, "refuse when a
   filter driver is declared at any scope" would refuse every run on this box**, this repository and every fixture
   included. §8d-ii.5 takes the reading that keeps W4's own qualifier, *"would yield pointer files"*, and names it for
   the operator. Re-measure. If a tracked path of this repository now selects a filter, STOP: §8d-ii.5's control no
   longer describes this repository.
4. **No code walks `.redkiln/` recursively.** At `c5d19d7`, every `read_dir` under `crates/*/src` that touches
   `.redkiln/` reads one named subdirectory, one level deep: `ctx.redkiln().join("cards")` and `.join("specs")` in
   `doctor/rules.rs`, `redkiln_dir.join("skills")` in `doctor/roles.rs`, `root.join(".redkiln").join("catalog")` in
   `run.rs`'s `catalog_sources`, and `Catalog::from_dir` over a directory it is handed. `init`'s `preexisting_rk_names` is *"Top level only"*. Re-measure. A recursive walk found here takes the
   exclusion and its control in the same commit (§8d-ii.1).
5. **`init` writes no hook, and no file in the git admin area.** D21-46's second correction reads *"`init` writes no
   hook, and `# redkiln-hook` exists only as doctor's recogniser."* At `c5d19d7`, `crates/rk-cli/src/verbs/init.rs`'s
   module doc reads *"It does exactly two things: writes `.redkiln/config.toml` **if and only if the file is
   absent**, and opens the store"*, and *"installs no hook (AC-412)"*. The only `# redkiln-hook` on the tree is
   `doctor`'s recogniser, `const MARKER: &str = "# redkiln-hook";`. So the exclude block is **`init`'s first write
   into a target repository's git admin area** (§8d-ii.5).
6. **Every fixture repository a run creates a worktree in.** List every fixture under `crates/*/tests/fixtures/` that a
   test runs `redkiln run` (or `verbs::run::run`) over. At `c5d19d7` that includes AC-438's exit table,
   `exit/findings-block/` and `cost`'s run transcript repository. Each gains the `.redkiln/worktrees/` ignore in the
   commit that moves the root, or creation refuses and its exit moves to `3` (§8d-ii.1).
7. **The test module names are free.** This stage extends `rk-git` `worktree` and `gitops` and `rk-cli` `init`, and it
   adds `rk-cli` `run_worktree`. If a predecessor created `run_worktree`, the tests go into it. The module-qualified
   names in §8d-ii.8 stay as they are.
8. **The whole-suite floor.** It is re-derived at stage open from Checkpoint 8d's recorded passed count, plus this
   stage's **32** new tests (§8d-ii.8). The expected value at drafting is **864**: `s8d`'s own drafted floor of
   `FLOOR=832`, plus 32. Replace 864 in Checkpoint 8d-ii's `FLOOR=` line with the re-derived value, by a dated edit,
   before the block's first run. It is never lowered to make a red run green.

### 8d-ii.1 The run worktree moves to `.redkiln/worktrees/<item>` (D21-44)

**The decision:** *"Run-owned worktrees move to **`.redkiln/worktrees/<item>`**, git-ignored. This mirrors Claude
Code's own `.claude/worktrees/<name>` convention"*. N1b measured it writable: *"`.redkiln/worktrees/probe-n1b`
(Claude-style, in-repo) | **`edited`** | **none**"*. D21-45 explains why: `checkPathSafetyForAutoEdit` treats a path
as sensitive if *"ANY segment"* is one of its list, and `.git` is on that list while `.redkiln` is not.

**What the record supersedes** is the records table's list, which D21-46 corrected and this file verified by
quotation. ADR-0018 is cited and superseded nowhere. The path literal is ADR-0137 decision 3's, and it moves while the
rule stands. The same path appears in design §5's `reconcile_started` row (*"the worktree and branch rebuild
deterministically from the item id (`<common-dir>/redkiln/worktrees/<item>/` on `rk-run/<item>`)"*) and in `s7d`
§7d.5's tables. Each takes a dated note.

**Every derivation moves to one function.** At `c5d19d7`, `crates/rk-cli/src/verbs/run.rs` derives the old root at
**four** code sites. They are `story_trailer`, `terminal_read`, `reference_worktree` (`format!("{item}.review-ref")`)
and `recreate_run_worktree` (`common_dir.join("redkiln").join("worktrees").join(item)`). There are **six** test code
sites in `crates/rk-cli/tests/it/run.rs` and `answer.rs`. After this stage, `rk_git::worktree` owns the one
derivation. It takes the repository root and a validated item (§8d-ii.2) and returns the path and the branch together.
All four sites, and every test support helper, call it. (`s8e` §8e.3 later deletes `story_trailer` and
`terminal_read`'s commit.) Checkpoint 8d-ii's code-line floor asserts that no code line outside
`crates/rk-git/src/worktree.rs` names a `"worktrees"` directory. The one exception is `doctor`'s
`ctx.common_dir.join("worktrees")`, which is **git's own** administrative directory and does not move.

**The ignore rows are load-bearing, not tidy.** A run tree is now a full checkout inside the checkout's own directory.
Without an ignore, `git status --porcelain` in the user's checkout lists it, and AC-518's instrument
(*"`git status --porcelain` empty, `git diff-index --quiet HEAD` clean, and HEAD unmoved"*) fails on a correct run.
- **This repository:** `.gitignore` already carries `.redkiln/worktrees/`, under the comment *"D21-44 (2026-09-24):
  run-owned worktrees live at .redkiln/worktrees/<item>"*. D21-44 says of that row that it *"is inert until s8e moves
  the root"*. Under D21-46 R1 it is this stage that moves the root.
- **A target repository:** `init` writes the exclude block (§8d-ii.5).
- **Every fixture repository a run creates a worktree in** gains the ignore in the commit that moves the root
  (§8d-ii.0 item 6). That is the same kind of forced edit as `[project] name`, and the one most likely to be missed.
  AC-438's table, `cost`'s run and `exit/findings-block/` are the witnesses: their exits must not move, and
  Checkpoint 8d-ii selects the two binary-arm tests by name.
- **Creation refuses by name** when `git check-ignore` does not report `.redkiln/worktrees/<item>` as ignored
  (`worktree::creation_refuses_when_the_worktree_root_is_not_ignored`). The alternative is a run that dirties the
  user's checkout by existing. **This is the behaviour D21-46 flags for the operator's flip line** (the records
  table).

**The dated register notes.**
- **AC-518**'s THEN reads *"refused unless its target is a run-owned worktree or the store under the resolved common
  dir"*. The note records that the run-owned worktree now sits inside the checkout's directory and is a separate
  checkout. It also records that the instrument's `git status --porcelain` holds through the ignore rows, and that
  `s8e`'s `lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it` is the instrument's run.
- **AC-529** carries no path. Its THEN reads *"the run-owned worktree is resolved **from the store**, never from
  cwd"*. The note records the move and D21-44's *"Ownership stays with the store row, so AC-529 stays
  real wherever the path is"*. It names `s8e` §8e.4's `worktree_assigned` as that row.

**Every recursive walk over `.redkiln/` excludes `worktrees/`, with a control.** A run tree carries a full copy of
`.redkiln/`, so any walker that recurses would read a second catalog, a second set of cards and stale runbook text.
- **In the code this stage owns, the walk count is zero** (§8d-ii.0 item 4). The guard is behavioural, not a promise.
  `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` plants
  `.redkiln/worktrees/planted/.redkiln/{catalog,cards,specs,items}` with **invalid** contents. It asserts that
  `doctor`'s findings, the catalog load and the item resolution are byte-equal with and without the planted tree. Its
  synthetic control is mutant `S8D-II-M9`, which makes `doctor`'s card reader recursive and must turn it RED.
- **Outside this stage's code, each owner applies the exclusion:**
  - `s8-exit-criterion.md`'s recursive greps over `crates/ .redkiln/` belong to `s8j`, which owns every `s8`
    amendment. At `c5d19d7` they include the `require.*\b(criticality|…)\b` search, the `append(_route)? *= *true`
    search and the `findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)` search. So does the mechanical
    block's `grep -rn … .`, which D21-40 found reading `.git/redkiln/worktrees/`. It would now read
    `.redkiln/worktrees/`, so excluding `.git/` alone no longer suffices.
  - `s8b`'s catalog-quality walks are `s8b`'s, and `s8j`'s own greps are `s8j`'s.
- **This stage's own checkpoint** walks nothing under `.redkiln/` recursively.

### 8d-ii.2 W1 — Claude Code's guards, each a named test in `rk-git`'s `worktree` module

**The decision.** W1: *"**Adopt all:** item-id validation as a path and branch component (at most 64 characters, safe
segments, no `.`, `..` or `.git`); symlink guards on `.redkiln`, `.redkiln/worktrees` and `<item>`, plus a post-create
realpath check; an **`RK_BASE`** marker in the worktree's gitdir, so ownership is recoverable from git alone (ADR-0109
d4); `git worktree lock --reason "redkiln run <item> (pid N start T)"` with stale-lock release; a re-entry check (a
separate checkout, registered to this repo); `LC_ALL=C` on every parsed git call; Windows removal never follows a
junction, with a test."*

- **The item id is validated once, as a path component and a branch component, in the derivation §8d-ii.1 names.**
  The rules are:
  - at most 64 characters;
  - the one segment matches `^[a-zA-Z0-9._-]+$`;
  - it is not `.`, `..` or `.git` (case-folded, because Windows is);
  - it has no leading `.`, and no `.lock` suffix (git's ref rules for `rk-run/<item>`);
  - it has no `.review-ref` suffix. That rule was found here: `reference_worktree` names its tree `<item>.review-ref`
    in the same directory, so an item id ending that way would collide with another item's reference tree.

  One table-driven test asserts each row's own refusal reason.
- **Symlink guards.** Creation refuses when `.redkiln`, `.redkiln/worktrees` or `<item>` is a symlink, or on Windows a
  junction. After `git worktree add`, the created tree's canonical path must lie under the canonical
  `.redkiln/worktrees/`, or the tree is refused and the refusal names both paths. Claude Code's reason is the one
  quoted in D21-45: *"A repository-committed symlink at .claude … could redirect worktree creation outside the
  repository"*.
- **The `RK_BASE` marker.** At creation, `RK_BASE` is written into the tree's own gitdir, the one `git rev-parse
  --git-dir` prints from inside it. It holds the fork sha and the item id, so ownership is recoverable from git alone
  (ADR-0109 decision 4). Redkiln never removes a tree without it.
- **The lock.** `git worktree lock --reason "redkiln run <item> (pid N start T)"` is taken at every run start and
  released at the run's end. A lock whose pid `rk_containment::is_alive` reports dead is released. A lock held by a
  live pid refuses the run by name. `is_alive` reads the pid alone, and its own doc names *"the documented ambiguity
  of this API"*. So a reused pid reads as alive and the run refuses. That is fail-closed, and the record says so.
- **The re-entry check.** A registered tree is reused only if it is a separate checkout **of this repository**:
  `git rev-parse --git-common-dir` from inside it equals the repository's, and `git worktree list --porcelain` lists
  its path.
- **`LC_ALL=C` on every parsed git call.** `GitExec`'s child environment sets it at the one process-exec site,
  `run_with_env`'s `std::process::Command::new("git")`, so no parse depends on the operator's locale
  (`gitops::every_git_child_runs_with_lc_all_c`).
- **Windows removal never follows a junction.** `worktree::removal_never_follows_a_junction` puts an ignored junction
  inside a tree, pointing at a sentinel directory outside it. It removes the tree and asserts the sentinel's contents
  survive. On a platform without junctions the arm is a symlink, and the test says which it ran.

### 8d-ii.3 W2 — reuse, never force-remove

**The decision.** W2: *"**Reuse, never force-remove.** Re-entry reuses the registered worktree when its HEAD equals
the `rk-run/<item>` tip. Uncommitted in-flight changes are discarded explicitly and journalled, which is N10's
semantics. A tree without `RK_BASE`, or with commits the run ref lacks, is **refused, never removed**. Removal happens
only when the tree is clean, landed, marked and unlocked. **This replaces `recreate_run_worktree`'s `worktree remove
--force` + `remove_dir_all` at every run start.** It deliberately does not copy Claude's `-B`, which resets an
existing branch and would destroy a resumable run. `--no-track` is adopted."*

This replaces `recreate_run_worktree` whole, and its doc's *"**Re-created, never reused**"* with it.
- **First creation:** `git worktree add --no-track -b rk-run/<item> <path> HEAD`. The fork is the frozen, CI-green
  `HEAD`, not `origin/<default>` (D21-45: *"runs fork from the frozen, CI-green HEAD rather than
  `origin/<default>`"*).
- **The branch exists but no tree is registered:** `git worktree add <path> rk-run/<item>`. That happens after a
  landed removal, or after a deleted tree. **Never `-B`** (`worktree::an_existing_run_branch_is_never_reset`).
- **Re-entry:** the registered tree is reused when it carries `RK_BASE`, passes the re-entry check, and its `HEAD`
  equals the `rk-run/<item>` tip. In-flight changes are read first, path by path, through `GitOp::Status`
  (`--porcelain=v1 -z --untracked-files=all`). They are then discarded explicitly (`reset --hard HEAD`, then
  `clean -fd`), and the lifecycle function **returns** every discarded path in its typed result. That is N10's
  semantics: what was not landed at a committed step is not work. `clean` runs **without `-x`**, so ignored files
  survive re-entry: W3's carried files and the `setup` verb's `node_modules` stay (`worktree::reentry_keeps_ignored_files`).
- **"Journalled" is met at `s8e`, and the interval is stated rather than hidden.** The journal row is `s8e` §8e.4's
  `worktree_assigned { …, reused, discarded }`, and that kind does not exist until `s8e` mints it. Between Checkpoint
  8d-ii and `s8e`'s commit, `run` prints each discarded path on stderr, one line per path. From `s8e` on, the same
  typed result fills `worktree_assigned.discarded`. The rk-git test asserts the returned list, and `s8e`'s tests
  assert the row.
- **Refused, never removed:** a tree with no `RK_BASE`, a tree registered to another repository, and a tree whose
  `HEAD` has commits the run ref lacks. Each refusal names the path and the reason.
- **Removal** happens only when the tree is clean, landed (`HEAD` equals the run-ref tip), marked and unlocked. It uses
  `git worktree remove` **without `--force`**. Anything else is kept, with a reason in Claude Code's form (*"N changed
  file(s) would be lost"*). Redkiln never removes a tree it did not create. This stage builds the removal rule and
  its tests. **The trigger is `s8e`'s**: removal is keyed to the lane's `closed` mark (`s8e` §8e.6), which does not
  exist before it.
- **The review reference tree follows the same rule.** `reference_worktree`'s comment reads *"A leftover from a killed
  run is removed rather than reused"*, and `remove_reference_worktree` forces it. Both change. A leftover reference
  tree is removed only when it is clean, marked and unlocked. D21-41 (g)'s grants make it detached and read-only, so
  it is clean unless something wrote to it. If something did, it is kept and named, and the review refuses by name.
- **Halt 6 is answered, not overridden.** Halt 6 reads *"If a fixture tree will not delete, that is the handle, and
  reusing the tree corrupts the next arm."* Reuse here is of a run's **own** tree, by the same item, only when its
  `HEAD` is the run ref's tip, and only after an explicit discard. A tree that will not delete is kept and named, and
  it is never handed to another item. The record states this. It answers ADR-0105's rejected option *"reuse
  worktrees across runs"* in that option's own terms, *"a fresh worktree per run is the only state the kernel can
  assert"*. The asserted state is now `RK_BASE`, plus `HEAD` equal to the tip, plus an empty status after the discard.
- **Removal is by run state, never by age.** D21-45: *"there is no `cleanupPeriodDays` analogue"*.

### 8d-ii.4 W3's carry, at creation only

**The decision.** W3: *"**Honour the project's existing `.worktreeinclude`**"* and *"**Dependencies come through
N25's per-toolchain `setup` verb.**"* D21-45's ownership line: *"W3 and W4's `init` half go to **s8c** (the `setup`
verb and `.worktreeinclude` reading) and s8e (the carry at creation)"*. Under D21-46 R1 the carry is this stage's.

After `git worktree add` and before the lock is released to the run, creation:
1. calls `s8c`'s `.worktreeinclude` reader (ignored ∩ matched, symlinks skipped, escapes refused) and copies what it
   returns;
2. then runs `s8c`'s per-toolchain `setup` verb (`pnpm install --frozen-lockfile --prefer-offline` for each declared
   Node fixture project).

A `setup` failure is `s8c`'s by-name refusal, and the run surfaces it and stops
(`run_worktree::a_failed_setup_refuses_by_name`). `settings.local.json` is not copied, because children run with
isolated settings (`s8h`). No `node_modules` symlink is made (W3: *"they share mutable state across runs"*). A reused
tree is not re-carried, because its ignored files survived the discard (§8d-ii.3).

### 8d-ii.5 W4 — git hygiene, at every config scope (D21-46 R8)

**The decisions.** W4: *"**Both.** Worktree creation is refused, with a diagnosis, when a repo-local filter driver
(for example LFS via `--local`) or an `includeIf` would yield pointer files. **`redkiln init` writes an idempotent
`# redkiln-runtime` block to `<commondir>/info/exclude`** for target repos, and this repo keeps its tracked
`.gitignore` row."* R8 widens the first half: *"**The effective config at every scope.** Refuse with a diagnosis naming
the scope. BrittonGames' LFS filter is system-scope and would have passed a repo-local check."* `s8i` §8i.0 item 12
measured that: BrittonGames *"carries its LFS filter at **system** scope (`filter.lfs.clean`, `.smudge`, `.process`,
`required true`). `git config --local --get-regexp '^filter\.'` and `'^includeif'` both return rc 1"*.

**The filter-driver refusal reads the effective configuration, not the local file.**
- **What it reads.** Before `git worktree add`, creation reads `git config --show-scope --show-origin --includes
  --get-regexp '^filter\.'` over the resolved configuration: `system`, `global`, `local`, `worktree` and
  `command` scopes, with every `include` and every `includeIf` whose condition matches followed. It also reads
  `--get-regexp '^includeif\.'` at every scope, so that an `includeIf` whose condition would match the **run tree**
  (a `gitdir:` pattern that covers `.redkiln/worktrees/`) but not the user's checkout is read from the file it
  names. Every read goes through `GitOps`, with `LC_ALL=C`.
- **When it refuses, and the reading this file takes.** W4 qualifies the refusal by effect: *"would yield pointer
  files"*. A driver yields pointer files only for a path whose `filter` attribute selects it. On this box a `system`
  scope `filter.lfs` is declared for every repository by Git for Windows (§8d-ii.0 item 3). So the refusal fires when
  **an effective filter driver, at any scope, is selected by the `filter` attribute of at least one tracked path**
  (`git check-attr filter` over the tree creation will check out), or **when a matching `includeIf` contributes a
  `filter.*` key**. Its diagnosis names each driver, the scope that declares it, the origin file, and one path that
  selects it. A driver that is declared and selected by no path is written on stderr as a diagnosis line and does
  not refuse. **This reading is flagged for the operator's flip line.** It keeps W4's qualifier and R8's scope. The
  literal alternative, refusing any declared driver at any scope, would refuse every run on this machine.
- **BrittonGames is the case R8 names**, and it refuses under this reading, because its tracked `.gitattributes` selects
  `lfs` for its binary assets. The test `worktree::an_effective_filter_driver_at_system_scope_refuses_creation_naming_the_scope`
  reproduces that shape with `GIT_CONFIG_SYSTEM` pointed at a fixture file, so no test reads the machine's real
  system configuration. The `global` arm uses `GIT_CONFIG_GLOBAL` in the same way.
- **The tests.** One per scope that declares a selected driver (`local`, `global`, `system`), one `includeIf` arm, and
  two controls. The first control is a driver declared at `system` scope and selected by no path, which **creates**.
  That is this box's own shape. The second is a repository with neither a driver nor an `includeIf`, which creates.
  **This refusal is what `s8a`'s `Text` variant verifies an LFS pointer path against**, so a run tree never
  materialises pointers through a filter it could not see.

**`init` writes the `# redkiln-runtime` block — its first write into the git admin area.** D21-46's second
correction: *"The `info/exclude` runtime block is therefore `init`'s first write into a target repo's git admin area,
and s8d-ii's record says so."* The record says so in those terms. Its idempotence is **new, not inherited**, because
`init` had no earlier admin-area write to be idempotent about. `init` appends to `<commondir>/info/exclude`, creating
the file if git's template did not:

```
# redkiln-runtime
/.redkiln/worktrees/
# end redkiln-runtime
```

- **Idempotent.** A second `init` finds the marker and leaves the file byte-identical. That extends ADR-0107 decision 2,
  *"a second `init` on an initialised repository changes nothing and exits 0"*, to this file.
- **Lines the operator wrote are never touched**, and `.gitignore` is never written (D21-45: *"never to
  `.gitignore`"*). This repository keeps its tracked `.gitignore` row as well.
- **AC-531 is unchanged.** Its 2026-09-18 amendment scopes the lint to *"the working tree, git refs and hooks"*, and
  it says that *"the git common dir is not user-visible namespace"*. `info/exclude` sits in the common dir and outside
  `hooks`, so `CreatedNames` does not change. The dated AC-531 note records the new write and why the lint does not
  see it.
- **The module doc's *"It does exactly two things"* becomes three**, by a dated edit in the same commit, and its
  *"installs no hook (AC-412)"* stays true.

### 8d-ii.6 The `GitOp` variants this takes, and the worst-case path

**The variants, named because *"adding a variant is that decision made visible"*** (`git_exec.rs`). ADR-0105 clause 5
enumerates the wrapper's write set and says *"Anything not listed is a successor"*. So the worktree record supersedes
clause 5 in part, for:
- the writes: `worktree lock`, `worktree unlock`, the `RK_BASE` marker write into the tree's gitdir, and `reset --hard
  HEAD` and `clean -fd`, the last two on the owned tree only;
- the reads: `worktree list --porcelain`, `config --show-scope --get-regexp` (at every scope, R8), `check-attr filter`
  and `check-ignore`.

Every one of them runs through `GitOps`. `s8e`'s N7 `git_op` row journals them once it exists, and on replay none of
them runs.

**The worst-case path (§8d-ii.0 item 2).** The record states the bound: `len(root) + 20 + 64 + 11 + 1 +` the longest
tracked path. It measured **255** of **259** at `c5d19d7`, with no `core.longpaths`. It also states the rule: creation
refuses by name when the computed worst case exceeds 259 and `core.longpaths` is not `true`, rather than check out a
partial tree (`worktree::creation_refuses_a_path_past_the_platform_limit_by_name`).

### 8d-ii.7 Checkpoint 7f's run arm and cleanup line (moved from `s8e`)

**Why these two amendments are this stage's.** As `s8a` leaves it, Checkpoint 7f runs `./target/debug/redkiln run
rk-fix-doctor-test-count --lane fix` in **this** repository. It ends with `git worktree remove --force
.git/redkiln/worktrees/rk-fix-doctor-test-count >/dev/null 2>&1 || true`. Under this stage's binary, that run either
refuses at W2's re-entry check (if the legacy registration still holds the branch) or creates
`.redkiln/worktrees/rk-fix-doctor-test-count` on the existing branch. The cleanup line then removes nothing, because
it names the old root and swallows its own failure. W2 never removes the new tree. So a second, stale full checkout
stays registered inside the product repository, which is the hazard 7f's own comment describes: *"it makes every
later repo-root grep double-count against a stale tree"*. Two dated amendments, in the commit that moves the root:
1. **The run arm moves to a throwaway clone.** The arm runs in a `git clone` of `HEAD` under 7f's own scratch
   directory, and it reads that clone's `.git/redkiln/runs`. The clone's `.gitignore` carries `.redkiln/worktrees/`,
   so creation passes the ignore check. ADR-0164's `* text=auto eol=lf` row is what makes a fresh checkout test the
   same bytes (`992d257`: *"a new checkout under autocrlf=true tests the same bytes"*). The clone's `rk-run/` refs,
   worktrees and store are its own, so nothing in this repository moves.
2. **The cleanup line is deleted.** It force-removes the legacy tree D21-44 keeps for `s8j` (*"stays where it is"*),
   and W2 forbids a forced removal.

`s8e`'s third 7f edit, the literal that expects the discharged `Slice-Verdict` lint, stays `s8e`'s (`s8e` §8e.13),
because the lint is discharged there. 7f's row in `PROGRESS.md` §1 becomes `passed …, re-run pending` in this
stage's commit, and Checkpoint 8d-ii re-runs 7f whole.

**The arm still cannot dispatch here.** 0 shipped cards carry `permission_mode`, and `card::lint_card` refuses a blank
one at bind. Checkpoint 8d-ii asserts that before it re-runs 7f, and refuses to re-run 7f once a card carries one.

### 8d-ii.8 Named tests, red first, and the mutants

**Every test of a NEW behaviour is written before the code it covers and watched RED for the reason in its row.** A
test first seen green is not evidence (PIA-035). **Three rows are pins over behaviour that no code violates today:**
- `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree`;
- `worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse`;
- `worktree::a_repo_with_no_filter_or_include_if_creates`.

They are green at entry by design. Each is shown able to fail by the mutant named for it: `S8D-II-M9` for the planted
tree, and `S8D-II-M5`'s inverse (refuse on any declared driver, which is R8's literal reading) for both controls.
**Two further rows are existing tests, re-selected as witnesses** that the fixture repositories' ignore rows move no
exit: `run::ac438_table_driven_exit_codes_over_fixtures_binary_arm` and
`cost::cost_after_a_real_run_renders_low_sample_through_the_binary`.

Each test is selected in Checkpoint 8d-ii by its module-qualified name with `--exact`. A bare function name under
`--exact` runs zero tests and exits 0.

| crate | test | red at entry because |
| --- | --- | --- |
| `rk-git` | `worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused` | no validator |
| `rk-git` | `worktree::a_symlinked_redkiln_or_worktrees_directory_is_refused` | no symlink guard |
| `rk-git` | `worktree::a_created_worktree_whose_realpath_leaves_the_root_is_refused` | no post-create check |
| `rk-git` | `worktree::creation_refuses_when_the_worktree_root_is_not_ignored` | no ignore check |
| `rk-git` | `worktree::creation_refuses_a_path_past_the_platform_limit_by_name` | no length rule |
| `rk-git` | `worktree::creation_writes_rk_base_into_the_worktree_gitdir` | no marker |
| `rk-git` | `worktree::creation_locks_the_worktree_with_the_pid_and_start_reason` | no lock |
| `rk-git` | `worktree::a_lock_held_by_a_dead_pid_is_released` | no lock |
| `rk-git` | `worktree::a_lock_held_by_a_live_pid_refuses_by_name` | no lock |
| `rk-git` | `worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip` | re-created by force |
| `rk-git` | `worktree::reentry_discards_in_flight_changes_and_reports_each_path` | removed silently |
| `rk-git` | `worktree::reentry_keeps_ignored_files` | the tree is deleted |
| `rk-git` | `worktree::a_tree_without_rk_base_is_refused_and_never_removed` (renamed from `an_existing_worktree_path_is_refused_not_reused`, whose refusal is kept) | no marker check |
| `rk-git` | `worktree::a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed` | force-removed |
| `rk-git` | `worktree::a_tree_registered_to_another_repository_is_refused` | no re-entry check |
| `rk-git` | `worktree::an_existing_run_branch_is_never_reset` | no reuse path to observe it |
| `rk-git` | `worktree::a_dirty_tree_is_kept_with_its_reason` | no removal rule |
| `rk-git` | `worktree::a_locked_tree_is_not_removed` | no lock |
| `rk-git` | `worktree::a_clean_landed_marked_unlocked_tree_is_removed` | no removal rule |
| `rk-git` | `worktree::removal_never_follows_a_junction` | no guarded removal |
| `rk-git` | `worktree::an_effective_filter_driver_at_local_scope_refuses_creation_naming_the_scope` | no config read |
| `rk-git` | `worktree::an_effective_filter_driver_at_global_scope_refuses_creation_naming_the_scope` | no config read |
| `rk-git` | `worktree::an_effective_filter_driver_at_system_scope_refuses_creation_naming_the_scope` | no config read; a `--local` read would also pass it |
| `rk-git` | `worktree::an_include_if_that_contributes_a_filter_refuses_creation_naming_the_included_file` | no config read |
| `rk-git` | `worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse` | green — a control; this box's own shape |
| `rk-git` | `worktree::a_repo_with_no_filter_or_include_if_creates` | green — the control; no refusal to observe |
| `rk-git` | `gitops::every_git_child_runs_with_lc_all_c` | no `LC_ALL` |
| `rk-cli` | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` | the common-dir root |
| `rk-cli` | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` | green — the pin; `S8D-II-M9` shows it can fail |
| `rk-cli` | `run_worktree::creation_carries_worktreeinclude_files_and_runs_setup` | no carry |
| `rk-cli` | `run_worktree::a_failed_setup_refuses_by_name` | no `setup` call |
| `rk-cli` | `init::init_writes_the_redkiln_runtime_exclude_block_once` | `init` writes no exclude |
| `rk-cli` | `init::init_leaves_the_operators_exclude_lines_untouched` | `init` writes no exclude |
| `rk-cli` | `run::ac438_table_driven_exit_codes_over_fixtures_binary_arm` | green — an existing witness; RED if a fixture repository lacks the ignore row |
| `rk-cli` | `cost::cost_after_a_real_run_renders_low_sample_through_the_binary` | green — an existing witness; RED if `cost`'s run repository lacks the ignore row |

**The count.** 35 names: `rk-git` 27, `rk-cli` 8. Two are existing witnesses and one is a rename, so **32 are new**,
and the suite gains **32** passed (§8d-ii.0 item 8).

`rk-git`'s `worktree::control_a_worktree_can_be_created_and_removed` is **re-pointed**, not deleted, because its
removal now needs a clean, marked, unlocked tree. The six `rk-cli` test sites that derive the old root (§8d-ii.1) are
re-pointed through the one derivation.

**The mutants.** Each mutant is applied and restored **from Python, never the shell**: a backtick once swallowed a
restore, and the next backup captured the mutant. Each diff is read. Each must turn the named test RED **for the
named reason**, printed and read, not merely "the suite went red". The names carry an `S8D-II-` prefix. These were
`s8e`'s S8E-M19 and S8E-M21 to S8E-M24, S8E-M30 to S8E-M33, and the rk-git half of S8E-M20. `s8e` keeps the
`lane_e2e` half of S8E-M20.

| mutant | change | must redden | was |
| --- | --- | --- | --- |
| S8D-II-M1 | the run-tree root derived under the common dir again | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` | S8E-M19 |
| S8D-II-M2 | re-entry force-removes and re-creates (`recreate_run_worktree` restored) | `worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip` | S8E-M20 (rk-git half) |
| S8D-II-M3 | a tree without `RK_BASE` is removed rather than refused | `worktree::a_tree_without_rk_base_is_refused_and_never_removed` | S8E-M21 |
| S8D-II-M4 | the discard returns no paths | `worktree::reentry_discards_in_flight_changes_and_reports_each_path` | S8E-M22 |
| S8D-II-M5 | the filter read narrowed to `--local` | `worktree::an_effective_filter_driver_at_system_scope_refuses_creation_naming_the_scope`, `..._at_global_scope_...`. Its inverse, refusing any declared driver, reddens both controls | S8E-M23, widened by R8 |
| S8D-II-M6 | `init` appends the block on every run | `init::init_writes_the_redkiln_runtime_exclude_block_once` | S8E-M24 |
| S8D-II-M7 | `LC_ALL` not set on the child | `gitops::every_git_child_runs_with_lc_all_c` | S8E-M30 |
| S8D-II-M8 | a dead pid's lock is never released | `worktree::a_lock_held_by_a_dead_pid_is_released` | S8E-M31 |
| S8D-II-M9 | `doctor`'s card reader made recursive | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` | S8E-M32 |
| S8D-II-M10 | `clean -fdx` in the discard | `worktree::reentry_keeps_ignored_files` | S8E-M33 |
| S8D-II-M11 | the ignore check skipped | `worktree::creation_refuses_when_the_worktree_root_is_not_ignored` | new |

A mutant that survives is not waved through. It is either an **unobservable property**, which moves to a source lint
with its reason written down, or a **missing fixture branch**. Both are recorded by the mutant's name.

### 8d-ii.9 What this file found

Findings carried from `s8e`'s revision (its former findings 10–13 and 15–17), with their disposition now, then this
split's own:

1. **D21-45's premise that `init` already writes the hooks marker (RT-51) is false.** *Settled by D21-46's second
   correction.* The record says the exclude block is `init`'s first write into a target repository's git admin area
   (§8d-ii.5).
2. **D21-44's supersession targets are not where it says.** *Settled by D21-46's first correction.* The records table
   carries ADR-0137 decision 3 and ADR-0105 clauses 5 and 6, each verified by quotation, and ADR-0018 is cited only.
3. **W1, W2 and W4 need `GitOp` variants outside ADR-0105 clause 5's list.** *Made explicit* (§8d-ii.6). R8 adds two
   reads to that list, `config --show-scope` and `check-attr`.
4. 🛑 **Checkpoint 7f's cleanup line removes the legacy tree before `s8j` can re-point it.** `s8a` runs 7f whole at its
   exit, before this stage, and 7f's last lines remove the registration. This stage deletes the line (§8d-ii.7), but
   `s8a`'s run comes first. Owner: `s8a` (or the lead), before `s8a` runs 7f. `s8j` then finds either outcome, and
   §8d-ii.0 item 1 records which.
5. **AC-518's instrument now depends on an ignore row.** *Flagged by D21-46 for the operator's flip line*, and stated
   as a refusal in §8d-ii.1.
6. **The review reference tree shares the item namespace.** `<item>.review-ref` sits beside `<item>`, so the validator
   refuses an item id with that suffix (§8d-ii.2).
7. **The worst-case path is 255 of 259 at `c5d19d7`, and it was 259 under the old root.** The margin is four characters
   on this repository, and a target repository with a longer root or deeper paths has less.
8. 🛑 **R8, read literally, refuses every run on this box** (§8d-ii.0 item 3). Git for Windows declares `filter.lfs.*`
   at `system` scope for every repository. §8d-ii.5 keeps W4's *"would yield pointer files"* qualifier: a declared
   driver refuses only when a tracked path selects it. That reading is the operator's to confirm on the flip line.
   Without it, Checkpoint 7f's clone, every fixture repository and every real run would refuse at creation.
9. **`s8e`'s quotation of ADR-0107 decision 2 did not match the record.** `s8e`'s former §8e.11 quoted *"A second
   `init` leaves this file untouched"*. ADR-0107 decision 2 reads *"**`init` is idempotent**: a second `init` on an
   initialised repository changes nothing and exits 0"*. Its quotation of AC-531 (*"separately and unlinted"*) is also
   about the store path, not the common dir as a whole. §8d-ii.5 quotes both as they stand.
10. **The discard is journalled one stage late.** W2's *"discarded explicitly and journalled"* has no journal row until
    `s8e` mints `worktree_assigned`. This file states the interval and reports on stderr in the meantime (§8d-ii.3).
11. **Files this split leaves stale, which this stage does not edit** (D21-46 R1's integration is the lead's):
    - `INDEX.md`'s stage table and `PROGRESS.md`'s tracker gain the `s8d-ii` row;
    - `s8d`'s header `next` becomes `s8d-ii`;
    - `s8c`'s ownership row and its `setup` row name `s8e` as the caller at creation, and its *"from `s8e` on"* names
      the root move's stage. Both now read `s8d-ii`;
    - `s8j` cites `s8e` §8e.10 for the `.redkiln/` walk exclusion and `s8e` §8e.15 for the 7f amendment. The first is
      now §8d-ii.1, and the second is §8d-ii.7 here plus `s8e` §8e.13.

### 8d-ii.10 Acceptance set

**Claimed: none as met at this stage.** This stage builds the placement and lifecycle that AC-518 and AC-529 run over,
and adds the dated notes. AC-518's instrument is a lane run (`s8e`'s `lane_e2e`), and AC-529's is `s8e`'s
`worktree_assigned` suite. Both are claimed at `s8e` (`s8e` §8e.14).

**Contributed to, and named so that no document claims them early:**
- **AC-518**: the ignore rows and the refusal that make its `git status --porcelain` hold with the tree inside the
  checkout.
- **AC-529**: the one derivation, which `s8e` §8e.4's row replaces as the source of ownership.
- **AC-531**: unchanged by the `info/exclude` write, with a dated note saying why (§8d-ii.5).

**Excluded:** AC-412 (`init` still installs no hook) and every tier-1 criterion (`s8e`'s).

### 8d-ii.11 Checkpoint 8d-ii

- **The run tree lives at `.redkiln/worktrees/<item>`, derived in one place.** No code line outside `rk-git`'s
  worktree module names a `worktrees` directory, apart from `doctor`'s read of git's own admin directory. *(D21-44)*
- **The root is ignored here, creation refuses when it is not, and the worst-case path is within the limit.**
  *(D21-44; D21-46's flag)*
- **W1's guards hold, each by a named test.** *(W1)*
- **Reuse, never force-remove.** `recreate_run_worktree` is gone, and no code line in `run.rs` forces a removal.
  *(W2)*
- **The filter check reads every scope, and it refuses by the scope's name.** `init` writes its block once.
  *(W4; R8)*
- **The fixture repositories' exits did not move.** *(§8d-ii.1)*
- **The whole suite, fmt and clippy are green.**
- **Checkpoint 8d (the entry gate) re-runs whole, and so does 7f, the one checkpoint this stage's edits make
  provisional (§8d-ii.7).** Nothing older is re-run. 7h and 7i are not amended here, and their witnesses are the two
  existing tests selected in (2). The block skips both nested re-runs when it is itself run with `RK_CP_NESTED=1`, the
  D21-46 convention: *"running as another checkpoint's entry gate: skip your own nested whole re-runs"*. It sets the
  same variable for the checkpoints it calls. `s8d`'s block does not read the variable, so it still runs its own
  nested re-runs: correct, only slower.
- **No replay can reach a real `claude`.** Every replay runs with `CLAUDE_CONFIG_DIR` at an empty trap directory that
  must still be empty afterwards, and with `REDKILN_LAUNCH_RECORD` unset (D21-46's replay guard).
- **`rk-run/rk-fix-doctor-test-count` and its legacy mark are unchanged**, and `.redkiln/worktrees/` is unchanged, so
  no lane ran against the product repository.

**BEFORE (PIA-035): on the entry tree these lines FAIL:**
- (2)'s selections, because the new tests do not exist;
- (3)'s root, `recreate_run_worktree` and force-removal floors, because of `run.rs`'s four derivations and its
  `"--force"`;
- (3)'s `LC_ALL`, `RK_BASE`, `--show-scope` and `# redkiln-runtime` floors, because none exists;
- (7b)'s 7f guards, because 7f still runs in this repository and still carries the cleanup line.

A block that passed on `c5d19d7` would be asserting nothing.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a SCRIPT (save it and `bash` it). Every guard ends in `exit 1`, and the helper functions below
# exit the script, which only holds outside a subshell.
#
# Conventions, each one paid for on this tree: every cargo reading is `out=$(…); rc=$?` and never a pipe's status;
# every selection is `--test it`, MODULE-QUALIFIED and `--exact`, asserting each test's own `... ok` line AND an exact
# count, because a bare name under `--exact` runs zero tests and exits 0; source greps read CODE lines only, anchored
# to grep's `path:LINE:` prefix; every synthetic control proves a guard can go RED. Scratch files live under .git/
# (Windows Python cannot see Git Bash's /tmp). Python helpers are written without a single backslash.
scratch=.git/cp8dii-scratch
rm -rf "$scratch" && mkdir -p "$scratch" || { echo "FAIL: the scratch directory $scratch could not be made"; exit 1; }
CODE_ONLY='^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'
LINE_ONLY='^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'

# ---- (0) BEFORE READINGS, taken before any cargo run and before any prune.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
rw_before=$(ls -A .redkiln/worktrees 2>/dev/null)
ref_before=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is absent — Checkpoints 7h and 7i read it"; exit 1; }
case "$ref_before" in
  73c087a*) ;;
  *) echo "FAIL: rk-run/rk-fix-doctor-test-count is at $ref_before, not 73c087a — something ran the untouched fixture item (D21-42 18)"; exit 1 ;;
esac
legacy=$(git log -1 --format='%(trailers:key=Story,valueonly=true)' "$ref_before"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git could not read the legacy mark (rc=$rc)"; exit 1; }
legacy=$(printf '%s\n' "$legacy" | grep .)
[ "$legacy" = "redkiln-rs/rk-fix-doctor-test-count" ] || { echo "FAIL: the one real Story: mark reads '$legacy', not the stage-less legacy value"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: the node fixture has no node_modules — run pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project first"; exit 1; }

# ---- (0a) THE RUN-TREE ROOT IS IGNORED IN THIS REPOSITORY (D21-44), or every run tree reads as untracked (AC-518).
git check-ignore -q .redkiln/worktrees/cp8dii-probe; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: .redkiln/worktrees/ is not ignored here (check-ignore rc=$rc) — D21-44's .gitignore row is missing"; exit 1; }
# CONTROL: a tracked file is NOT ignored, so the line above can go red.
git check-ignore -q Cargo.toml; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: CONTROL — check-ignore called a tracked file ignored (rc=$rc), so the ignore guard cannot fail"; exit 1; }

# ---- (0b) THE WORST-CASE RUN-TREE PATH (section 8d-ii.0 item 2): root + /.redkiln/worktrees/ + a 64-character item +
# ---- .review-ref + / + the longest tracked path, against the 259 characters MAX_PATH leaves.
top=$(git rev-parse --show-toplevel); rc=$?
[ "$rc" -eq 0 ] && [ -n "$top" ] || { echo "FAIL: git rev-parse --show-toplevel failed (rc=$rc)"; exit 1; }
files=$(git ls-files); rc=$?
[ "$rc" -eq 0 ] && [ -n "$files" ] || { echo "FAIL: git ls-files failed or listed nothing (rc=$rc)"; exit 1; }
longest=$(printf '%s\n' "$files" | awk '{ if (length($0) > m) m = length($0) } END { print m + 0 }')
[ "$longest" -gt 0 ] || { echo "FAIL: the longest tracked path read as $longest — the length guard would be about nothing"; exit 1; }
prefix="$top/.redkiln/worktrees/"
worst=$(( ${#prefix} + 64 + 11 + 1 + longest ))
lp=$(git config --get core.longpaths); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: git config could not be read (rc=$rc)"; exit 1; }
[ "$worst" -le 259 ] || [ "$lp" = "true" ] || { echo "FAIL: the worst-case run-tree path is $worst characters (longest tracked path $longest) and core.longpaths is not true — the record's length rule must refuse this at creation"; exit 1; }
# CONTROL: the same sum over a 200-character path exceeds the limit, so the line above can go red.
ctl=$(( ${#prefix} + 64 + 11 + 1 + 200 ))
[ "$ctl" -gt 259 ] || { echo "FAIL: CONTROL — a 200-character path summed to $ctl, within the limit, so the length guard cannot fail"; exit 1; }
echo "worst-case run-tree path: $worst characters (longest tracked path $longest; the limit is 259 without core.longpaths)"

# ---- (0c) R8's PRECONDITION (section 8d-ii.0 item 3): no tracked path of THIS repository selects a filter driver, so the
# ---- control test's shape (a declared, unselected driver) is still this repository's own. A STOP, not a guess.
# (Not `-z`: a NUL does not survive `$(…)`, so the line form is read and its last field is the value.)
attrs=$(git ls-files | git check-attr --stdin filter); rc=$?
[ "$rc" -eq 0 ] && [ -n "$attrs" ] || { echo "FAIL: git check-attr filter could not read the tracked paths (rc=$rc)"; exit 1; }
nsel=$(printf '%s\n' "$attrs" | grep -vE ': filter: (unspecified|unset)$' | grep -c .)
[ "$nsel" -eq 0 ] || { echo "FAIL (STOP): $nsel tracked path(s) here select a filter driver — section 8d-ii.5's control no longer describes this repository; take it to the operator"; exit 1; }
# CONTROL: the same count over one selected line and one unspecified line is 1, so the line above can go red.
csel=$(printf '%s\n' 'a.bin: filter: lfs' 'b.rs: filter: unspecified' | grep -vE ': filter: (unspecified|unset)$' | grep -c .)
[ "$csel" -eq 1 ] || { echo "FAIL: CONTROL — one selected and one unspecified line counted $csel, not 1"; exit 1; }
scopes=$(git config --show-scope --get-regexp '^filter\.'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the effective filter configuration could not be read (rc=$rc)"; exit 1; }
echo "effective filter drivers here (declared, selected by no path): $(printf '%s\n' "$scopes" | grep -c .)"

# ---- (1) BUILD-OR-FAIL, before any binary is read (Halt 1).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8d-ii NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE NAMED TESTS, per crate, each selected exactly. `sel_tests <package> <count> <names…>`.
sel_tests() {
  pkg=$1; want=$2; shift 2
  [ "$#" -eq "$want" ] || { echo "FAIL: the $pkg list names $# tests and its count says $want — the list and the assertion disagree"; exit 1; }
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted, ignored, or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed; 0 ignored" || { echo "FAIL: the $pkg selection did not run exactly $want tests with none ignored"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
# CONTROL: a filter that matches nothing EXITS 0. This is why sel_tests asserts each ok line and an exact count.
out=$(cargo test -p rk-git --test it -- worktree::s8dii_control_no_such_test --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the zero-match control did not run (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed' || { echo "FAIL: CONTROL — a zero-match filter did not report 0 passed, so this block cannot tell it from a real run"; exit 1; }

sel_tests rk-git 27 worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused worktree::a_symlinked_redkiln_or_worktrees_directory_is_refused worktree::a_created_worktree_whose_realpath_leaves_the_root_is_refused worktree::creation_refuses_when_the_worktree_root_is_not_ignored worktree::creation_refuses_a_path_past_the_platform_limit_by_name worktree::creation_writes_rk_base_into_the_worktree_gitdir worktree::creation_locks_the_worktree_with_the_pid_and_start_reason worktree::a_lock_held_by_a_dead_pid_is_released worktree::a_lock_held_by_a_live_pid_refuses_by_name worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip worktree::reentry_discards_in_flight_changes_and_reports_each_path worktree::reentry_keeps_ignored_files worktree::a_tree_without_rk_base_is_refused_and_never_removed worktree::a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed worktree::a_tree_registered_to_another_repository_is_refused worktree::an_existing_run_branch_is_never_reset worktree::a_dirty_tree_is_kept_with_its_reason worktree::a_locked_tree_is_not_removed worktree::a_clean_landed_marked_unlocked_tree_is_removed worktree::removal_never_follows_a_junction worktree::an_effective_filter_driver_at_local_scope_refuses_creation_naming_the_scope worktree::an_effective_filter_driver_at_global_scope_refuses_creation_naming_the_scope worktree::an_effective_filter_driver_at_system_scope_refuses_creation_naming_the_scope worktree::an_include_if_that_contributes_a_filter_refuses_creation_naming_the_included_file worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse worktree::a_repo_with_no_filter_or_include_if_creates gitops::every_git_child_runs_with_lc_all_c
sel_tests rk-cli 8 run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees run_worktree::no_redkiln_reader_sees_a_planted_run_worktree run_worktree::creation_carries_worktreeinclude_files_and_runs_setup run_worktree::a_failed_setup_refuses_by_name init::init_writes_the_redkiln_runtime_exclude_block_once init::init_leaves_the_operators_exclude_lines_untouched run::ac438_table_driven_exit_codes_over_fixtures_binary_arm cost::cost_after_a_real_run_renders_low_sample_through_the_binary

# ---- (3) CODE-LINE FLOORS beneath section 8d-ii.8's behavioural tests — never the assertion.
test -f crates/rk-git/src/worktree.rs && test -f crates/rk-cli/src/verbs/run.rs && test -f crates/rk-git/src/git_exec.rs && test -f crates/rk-cli/src/verbs/init.rs || { echo "FAIL: a source file these floors read is absent (Halt 8)"; exit 1; }
# One derivation of the root: no code line outside rk-git's worktree module names a "worktrees" directory, except
# doctor's read of git's OWN administrative directory, <common-dir>/worktrees, which does not move.
raw=$(grep -rnF '"worktrees"' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the root search could not run (grep rc=$rc)"; exit 1; }
DOCTOR_ADMIN='^crates/rk-cli/src/doctor/rules\.rs:[0-9]+:.*common_dir\.join\("worktrees"\)'
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-git/src/worktree\.rs:' | grep -vE "$DOCTOR_ADMIN" | grep .)
[ -z "$code" ] || { echo "FAIL: a code line outside rk-git's worktree module derives a worktrees directory — the root is derived in one place (D21-44):"; printf '%s\n' "$code"; exit 1; }
own=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep '^crates/rk-git/src/worktree\.rs:')
[ -n "$own" ] || { echo "FAIL: rk-git's worktree module names no worktrees directory on a code line — the one derivation does not exist"; exit 1; }
# CONTROL: a run.rs code line is KEPT; the doctor admin-dir line and a comment are DROPPED.
ctl=$(printf '%s\n' 'crates/rk-cli/src/verbs/run.rs:9:    let p = common.join("redkiln").join("worktrees").join(item);' 'crates/rk-cli/src/doctor/rules.rs:12:    let Ok(e) = std::fs::read_dir(ctx.common_dir.join("worktrees")) else {' 'crates/rk-cli/src/verbs/run.rs:10:    // the old root joined "worktrees" under the common dir' | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-git/src/worktree\.rs:' | grep -vE "$DOCTOR_ADMIN" | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the root filter kept $ctl of one derivation, one admin-dir read and one comment"; exit 1; }
# W2: recreate_run_worktree is gone, and run.rs forces no removal (the run tree and the reference tree alike).
raw=$(grep -rn 'recreate_run_worktree' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the recreate search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: recreate_run_worktree is still on a code line — D21-45 W2 replaces it:"; printf '%s\n' "$code"; exit 1; }
raw=$(grep -nE '"--force"|remove_dir_all' crates/rk-cli/src/verbs/run.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the force-removal search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: run.rs still forces a removal on a code line — W2 is never force-remove:"; printf '%s\n' "$code"; exit 1; }
# W1: LC_ALL=C at the one git exec site, and the RK_BASE marker in the worktree module.
raw=$(grep -nF 'LC_ALL' crates/rk-git/src/git_exec.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the LC_ALL search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in git_exec.rs sets LC_ALL — W1's locale pin is absent"; exit 1; }
raw=$(grep -nF 'RK_BASE' crates/rk-git/src/worktree.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the RK_BASE search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in rk-git's worktree module names RK_BASE — W1's ownership marker is absent"; exit 1; }
# R8: the filter read carries the scope, and no code line narrows it to --local.
raw=$(grep -nF -e '--show-scope' -e '"--local"' crates/rk-git/src/worktree.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the filter-scope search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
printf '%s\n' "$code" | grep -qF -- '--show-scope' || { echo "FAIL: no code line in rk-git's worktree module reads the config with --show-scope — R8's scope-naming diagnosis has no source"; exit 1; }
local_hits=$(printf '%s\n' "$code" | grep -F '"--local"')
[ -z "$local_hits" ] || { echo "FAIL: the worktree module narrows a config read to --local on a code line — R8 reads every scope:"; printf '%s\n' "$local_hits"; exit 1; }
# CONTROL: the line filter KEEPS a --local code line and DROPS a comment naming it.
ctl=$(printf '%s\n' '12:        .arg("--local")' '13:    // not "--local" (R8)' | grep -vE "$LINE_ONLY" | grep -F '"--local"' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the --local filter kept $ctl of one code line and one comment"; exit 1; }
# W4: init's exclude marker is on a code line.
raw=$(grep -nF '# redkiln-runtime' crates/rk-cli/src/verbs/init.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the exclude-marker search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: init.rs carries no # redkiln-runtime marker on a code line — W4's exclude block is absent"; exit 1; }

# ---- (4) THE WHOLE SUITE, no fail-fast, with a floor. FLOOR is re-derived at stage open (section 8d-ii.0 item 8):
# ---- Checkpoint 8d's recorded passed count + this stage's 32. 864 is the drafted expectation; replace it by a dated edit.
FLOOR=864
tally() { printf '%s\n' "$1" | awk '/^test result:/ { p += $4; f += $6; i += $8; n += 1 } END { print p+0, f+0, i+0, n+0 }'; }
# CONTROL: a run of zero tests tallies below the floor, and a failed count is read.
read -r cp cf ci cn <<< "$(tally 'test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out')"
[ "$cp" -lt "$FLOOR" ] || { echo "FAIL: CONTROL — a zero-test run tallied $cp passed, not below the floor $FLOOR"; exit 1; }
read -r cp cf ci cn <<< "$(tally 'test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out')"
[ "$cf" -eq 2 ] || { echo "FAIL: CONTROL — tally read $cf failed from a line carrying 2"; exit 1; }
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
read -r passed failed ignored results <<< "$(tally "$out")"
[ "$failed" -eq 0 ] && [ "$passed" -ge "$FLOOR" ] && [ "$results" -ge 44 ] || { echo "FAIL: the suite read passed=$passed failed=$failed ignored=$ignored over $results results — the floor is $FLOOR passed over at least 44 results"; exit 1; }
echo "whole suite: passed=$passed failed=$failed ignored=$ignored results=$results (floor $FLOOR) — record these in PROGRESS.md"

# ---- (5) fmt and clippy, gated on exit codes (never `-D warnings` on the command line — ADR-0083).
out=$(cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }

# ---- (6) THE WORKTREE PATH SET, compared BEFORE the re-run blocks, because a nested block may prune.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

# ---- (7) RE-RUN FROM THEIR OWN BYTES: the entry gate (8d) and the one checkpoint this stage's edits make provisional
# ---- (7f, amended in section 8d-ii.7). Nothing older. SKIPPED WHOLE under RK_CP_NESTED=1 (D21-46: running as another
# ---- checkpoint's entry gate), and set for every block this one calls. Every replay runs under the no-dispatch
# ---- guard: CLAUDE_CONFIG_DIR at an empty trap directory that must stay empty, and REDKILN_LAUNCH_RECORD unset.
# ---- The extractor takes the ONE bash block of a file that contains a needle and refuses on zero or two. Its fence
# ---- string is built from chr(96), so no triple backtick appears inside this block.
cat > "$scratch/extract.py" <<'PY'
import sys
src, needle, dst = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(src, encoding="utf-8").read()
fence = chr(96) * 3
opener = fence + "bash" + chr(10)
blocks, pos = [], 0
while True:
    a = text.find(opener, pos)
    if a < 0:
        break
    b = text.find(chr(10) + fence, a + len(opener))
    if b < 0:
        break
    blocks.append(text[a + len(opener):b + 1])
    pos = b + 4
hits = [blk for blk in blocks if needle in blk]
if len(hits) != 1:
    print("EXTRACT-FAIL: %d of %d bash blocks in %s contain the needle" % (len(hits), len(blocks), src))
    sys.exit(1)
open(dst, "w", encoding="utf-8", newline=chr(10)).write(hits[0])
print("EXTRACT-OK " + dst)
PY
test -s "$scratch/extract.py" || { echo "FAIL: the extractor was not written"; exit 1; }
# CONTROL: the extractor REFUSES a needle no block carries, so a renamed checkpoint cannot be skipped silently.
out=$(python "$scratch/extract.py" runbook/phase-1/s8d-step-grain-and-resolver.md 'S8DII-CONTROL-NO-SUCH-NEEDLE' "$scratch/none.sh" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: CONTROL — the extractor accepted a needle no block contains"; exit 1; }
ext() {
  test -f "$1" || { echo "FAIL: $1 does not exist — the checkpoint it holds cannot be re-run (Halt 8)"; exit 1; }
  out=$(python "$scratch/extract.py" "$1" "$2" "$3" 2>&1); rc=$?
  [ "$rc" -eq 0 ] && [ -s "$3" ] || { echo "FAIL: the block in $1 could not be extracted whole (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
}

if [ "${RK_CP_NESTED:-0}" = "1" ]; then
  echo "CP8D-II: nested re-runs skipped — RK_CP_NESTED=1 was set by the checkpoint re-running this one as its entry gate"
else
  # PRECONDITION: no replay below can reach a real dispatch. Before s8h no shipped card carries permission_mode.
  ncards=0
  for c in .redkiln/cards/*.json; do [ -f "$c" ] && ncards=$((ncards + 1)); done
  [ "$ncards" -gt 0 ] || { echo "FAIL: no card files under .redkiln/cards — the precondition would hold over nothing (Halt 8)"; exit 1; }
  moded=$(grep -lF '"permission_mode"' .redkiln/cards/*.json); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: the card-mode precondition could not run (grep rc=$rc)"; exit 1; }
  [ -z "$moded" ] || { echo "FAIL (PRECONDITION): a shipped card carries permission_mode, so s8h has landed — re-run 7f only through s8h's fixture-child route, not from this block:"; printf '%s\n' "$moded"; exit 1; }
  # SYNTHETIC CONTROL: the probe sees a planted mode.
  printf '{"permission_mode": "default"}\n' > "$scratch/card.json" || { echo "FAIL: the card control could not be written"; exit 1; }
  grep -qF '"permission_mode"' "$scratch/card.json" || { echo "FAIL: CONTROL — the card-mode probe cannot see a planted permission_mode"; exit 1; }
  # THE TRAP: an empty CLAUDE_CONFIG_DIR every replay inherits. A real claude writes into it; nothing else does.
  trap_dir="$(pwd)/$scratch/claude-config-trap"
  mkdir -p "$trap_dir" || { echo "FAIL: could not create the CLAUDE_CONFIG_DIR trap"; exit 1; }
  replay() {
    out=$(env -u REDKILN_LAUNCH_RECORD RK_CP_NESTED=1 CLAUDE_CONFIG_DIR="$trap_dir" "$@" 2>&1); rc=$?
  }

  # (7a) CHECKPOINT 8d, the entry gate, whole. s8d's block does not read RK_CP_NESTED, so it re-runs 8c and 7g itself.
  ext runbook/phase-1/s8d-step-grain-and-resolver.md 'echo "CP8D-END-OF-BLOCK reached"' "$scratch/cp8d.sh"
  replay bash "$scratch/cp8d.sh"
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8d, re-run whole, exited $rc — this stage's entry gate no longer holds"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8D-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8d exited 0 without reaching its end marker"; exit 1; }

  # (7b) CHECKPOINT 7f whole, AS AMENDED (section 8d-ii.7): its run arm is in a throwaway clone, and its cleanup line
  # is gone. Both are asserted on the extracted bytes before the block runs.
  ext runbook/phase-1/s7f-conformance-and-records.md 'echo "CP7F-END-OF-BLOCK reached"' "$scratch/cp7f.sh"
  grep -qF 'git clone' "$scratch/cp7f.sh" || { echo "FAIL: Checkpoint 7f's run arm is not re-pointed to a throwaway clone — run here, it would create a run tree in the product repository"; exit 1; }
  hits=$(grep -nF 'worktree remove --force .git/redkiln/worktrees/' "$scratch/cp7f.sh"); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: Checkpoint 7f still force-removes the legacy run tree (grep rc=$rc) — D21-44 keeps it for s8j:"; printf '%s\n' "$hits"; exit 1; }
  replay bash "$scratch/cp7f.sh"
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7f, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7F-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7f exited 0 without reaching its end marker"; exit 1; }

  # THE TRAP IS STILL EMPTY — no replay spawned a real claude. CONTROL first: the emptiness read sees a planted file.
  mkdir -p "$scratch/trap-control" && : > "$scratch/trap-control/planted" || { echo "FAIL: the trap control could not be planted"; exit 1; }
  ctl=$(ls -A "$scratch/trap-control")
  [ -n "$ctl" ] || { echo "FAIL: CONTROL — the emptiness read did not see a planted file"; exit 1; }
  left=$(ls -A "$trap_dir")
  [ -z "$left" ] || { echo "FAIL: something wrote into the trap CLAUDE_CONFIG_DIR during the replays — a real claude was spawned:"; printf '%s\n' "$left" | head -10; exit 1; }
fi

# ---- (8) AFTER: the fixture item's ref and mark are untouched, and no run tree appeared in this repository.
ref_after=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ "$ref_after" = "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count moved during this block ($ref_before -> $ref_after)"; exit 1; }
rw_after=$(ls -A .redkiln/worktrees 2>/dev/null)
[ "$rw_before" = "$rw_after" ] || { echo "FAIL: .redkiln/worktrees changed during this block ('$rw_before' -> '$rw_after') — a run created a tree in the product repository"; exit 1; }
rm -rf "$scratch" || { echo "FAIL: the scratch directory $scratch could not be removed"; exit 1; }

echo "CP8D-II-END-OF-BLOCK reached"
```

---
