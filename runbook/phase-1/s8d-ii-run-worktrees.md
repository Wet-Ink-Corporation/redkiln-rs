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

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The legacy registration survives `s8a`.** `s8a` guard (11) restores the tree 7f's whole run removes and asserts
  its ref (§8d-ii.0 item 1), so finding 4 is settled by `s8a`, not left to the lead.
- **Checkpoint 8d now honours `RK_CP_NESTED=1`**, so (7a)'s replay of it skips 8d's own re-runs of 8c and 7g. The
  trap asserts that it starts empty, in `s8g`'s form.
- **The floor chain is recomputed once**: Checkpoint 8d's 885 + this stage's 32 = **917**, not the drafted 864.
- **Finding 11's sibling edits are made** in `s8c`, `s8d` and `s8j`. `INDEX.md`'s and `PROGRESS.md`'s rows stay the
  lead's.

*Adversarial-review fixes 2026-09-24 (D21-47)* (the review of this file at `4a8c832`, and the cross-file critic's item
10; D21-47 and its V1 the authority; every fix checked against the tree at `fc80687`). Finding id → fix:
- **B1** (the checkpoint asserted none of the stage's records) → Checkpoint 8d-ii guard (9). It locates the record by
  `REC_WT`, the id recorded at writing, never by title. It asserts the supersedes line (0105, 0107, 0137; not 0018 or
  0162), the quoted sentences, the back-links in the state the record's status requires, the index row, the ledger
  entry, the register notes on AC-518, AC-522, AC-529 and AC-531 read from each criterion's own block, design §5's
  and §6's row notes, the `s7d` notes and the fixture doc. Each reader has a control.
- **M2** (the length rule assumed a 64-character item) → the bound is computed on the actual item (§8d-ii.0 item 2,
  §8d-ii.6), with a new control test. 7f's clone moves outside the repository to `mktemp -d` (§8d-ii.7), and (0b)
  checks both sums.
- **M3 and V1** (the filter refusal was inverted) → §8d-ii.5 rewritten to V1: refuse only when pointer files would
  result, with attributes read by `check-attr --source=<fork>`. The six filter tests become ten, and (0c) checks V1's
  condition on this repository. M5 is re-aimed, and M12 to M15 and M21 are added.
- **M4** (it contradicted `s8i` §8i.0 item 12 and §8i.8) → BrittonGames is admitted, as `s8i` says (§8d-ii.5).
- **M5** (ADR-0162 decision 10 would move) → the reference tree keeps its forced removal, and decision 10 is cited, not
  superseded (§8d-ii.3). (3)'s force floor admits a force only inside `remove_reference_worktree`.
- **M6** (the root was checkout-dependent) → the root is the main worktree, read from `git worktree list --porcelain`,
  and a bare repository is refused (§8d-ii.1). Two tests; (0a) checks it here.
- **M7** ("landed" was tautological) → landed is defined against the `closed` mark, since Phase 1 has no integration
  ref, and re-entry requires a symbolic `HEAD` (§8d-ii.3). Two tests, including the detached-`HEAD` fixture.
- **M8** (reuse broke the reference tree's merge-base premise) → the reference tree is created at `RK_BASE`'s fork sha
  (§8d-ii.3). One test.
- **M9** (the design §6 row, AC-522 and ADR-0107 decision 2 were missing) → all three are in the records table. ADR-0107
  decision 2 is superseded in part, and design §6 and AC-522 take dated notes. Clause 6's supersession is scoped to
  the run worktree, so `redkiln-migrate`'s keeps it.
- **M10** (it built unconfirmed readings behind its own STOP) → V1 answers the filter reading. The ignore refusal
  becomes a stage-open 🛑 (§8d-ii.0 item 9), asked before any code. *D21-48 (2026-09-24): X1 took it ("Refuse by
  name"), so item 9 is no longer a STOP.*
- **Cross 10 / D21-47's clause-5 rule** → the reads are dropped from the clause-5 supersession, which now lists writes
  only (the records table, §8d-ii.6).
- **m11** (`init` already writes under the common dir) → reworded: `info/exclude` is `init`'s first write to a file
  git reads (§8d-ii.0 item 5, §8d-ii.5).
- **m12** (the 7f replay guards were weak) → (7b) reads code lines only and refuses any `worktree remove` or `prune`.
  It then judges the replay by behaviour: the path set, the run-record count and the dispatch rows (§8d-ii.7).
- **m13** (the floors were bypassable) → the root floor scans `src` and `tests` for every spelling, the force floor
  covers `-f` and `rk-git`'s worktree module, `LC_ALL` must be the call form, and (0c) takes no pipe status.
- **m14** (validator gaps) → `a..b`, a trailing `.`, device names and the case-fold collision are added. The symlink
  test uses a junction, and `\\?\` paths never reach git (§8d-ii.2).
- **m15** (`is_alive` misquoted) → the 259 exit-code ambiguity is quoted, the start time is compared, and the
  predicate is injected, so there is no new `rk-git` edge (§8d-ii.2). One `rk-containment` test is added.
- **m16** (code-built test repositories) → listed in §8d-ii.0 item 6.
- **m17** (the diagnosis line printed on every run) → debug level only (§8d-ii.5).
- **m18** (the `node_modules` precondition was a proxy) → (0) runs an `--offline` install of the lockfile.
- **D21-47's wrapper** → every cargo run and every replay goes through `wrap` (Checkpoint 8d-ii (00)). The fake's
  limit is new finding 16.
- **The count** goes from 32 to **46** new tests (49 names), and the floor from 917 to **931** (§8d-ii.0 item 8).

*Final consistency pass 2026-09-24* (the pass brief after D21-47's fix round; each edit dated on its own line):
- *Final consistency pass 2026-09-24:* the floor is re-derived once on the final chain: 740 + 9 (8a) + 25 (8b) + 69
  (8c) + 58 (8d) = 901 at Checkpoint 8d, plus this stage's 46 = **947** (§8d-ii.0 item 8, Checkpoint 8d-ii (4)). The
  917 and 931 above are superseded.
- *Final consistency pass 2026-09-24:* Checkpoint 8d-ii sources the shared no-dispatch wrapper,
  `runbook/tools/nodispatch.sh`, in place of its inline shell-script fake. `nd_setup` builds it, every cargo run and
  replay runs under `nodispatch`, and `nd_check` follows each. The stage's own dispatch counts and path-set guards
  stay (§8d-ii.11, (00), (7), (8)).
- *Final consistency pass 2026-09-24:* finding 16 is settled by that helper, whose fake is a compiled `claude.exe`
  (§8d-ii.9).
- *Final consistency pass 2026-09-24:* `REC_WT` is a literal in the block, written by dated edit in the commit that
  writes the record. The block refuses while `REC_WT=____` remains, and no re-runner supplies it (the attendant edits,
  Checkpoint 8d-ii).
- *Final consistency pass 2026-09-24:* 7f's amended arm names its clone `clone="$c7f/c"`, the variable `s8j` §8j.8 A10
  takes at application, and (7b) admits a runs read that names `$clone` (§8d-ii.7).

*D21-48 (2026-09-24)* (the operator's answers to the questions the `s8` files left; D21-48 the authority; this file
executes X1, X2 and X14; each edit dated on its own line):
- *D21-48 (2026-09-24):* **X1**, the ignore refusal is taken: *"**Refuse by name.** This answers D21-46's flip-line flag."* The stage-open 🛑 (§8d-ii.0
  item 9) is removed, and so is the records table's "if the operator declines" branch (the `creation_warns_…` rename,
  `S8D-II-M11`'s re-aim and `lane_e2e`'s fixture precondition). The record states the refusal (§8d-ii.1). Finding 5
  is settled.
- *D21-48 (2026-09-24):* **X2**, finding 15 is settled by a post-checkout scan: *"After creation, the paths the filter
  selects are scanned for LFS pointer headers; any hit refuses by name and removes the fresh worktree."* §8d-ii.5
  specifies it, §8d-ii.3 names its rollback as the one removal of a not-yet-landed tree (the fresh tree this call
  created, unforced), and §8d-ii.4 orders it before the lock and the carry. Four new `rk-git` tests (§8d-ii.8), mutants
  `S8D-II-M22` to `S8D-II-M26`, Checkpoint 8d-ii (0d) with two synthetic controls, a (3) code-line floor, and two
  (9) quotations.
- *D21-48 (2026-09-24):* **X14**, W3's owner is confirmed: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at creation."* (§8d-ii.4, and "What it
  does not own").
- *D21-48 (2026-09-24):* the count goes from 46 to **50** new tests (53 names), and the floor from 947 to **951**
  (§8d-ii.0 item 8, §8d-ii.8, Checkpoint 8d-ii (4)). `s8e`'s floor chain starts from 947 and is not edited here.
- *D21-48 (2026-09-24), floor propagation:* `s8c` now states **78** new tests and `s8d` **62** net, so Checkpoint 8d's expected count is 740 + 9 + 25
  + 78 + 62 = **914**, and this stage's floor is 914 + 50 = **964** (§8d-ii.0 item 8, Checkpoint 8d-ii (4)). The
  901 + 50 = 951 above is superseded. `s8e`'s chain is propagated in its own file.

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
corrected and extended by D21-46 (its two corrections, R8, and the flag on the ignore row), and as D21-47 refines them
(V1, R8's refusal semantics, and its clause-5 rule), and as D21-48 answers what they left open (X1, the ignore
refusal; X2, the post-checkout pointer scan; X14, W3's owner), are all in `spikes/DECISIONS-2026-09-21.md`. They are written as one record at this stage. It takes **the next free id at
writing**, and this file assigns none. Where the record moves a `## Decision` sentence or a forbids bullet of an
accepted record, it is a supersession in part (ADR-0092 rule 1), never an erratum. It is written `proposed` and flips
only on the operator's dated ledger line.

**What this step owns:**
- **D21-44** whole, including its *"Consequences"*. The ledger says *"owned by `s8e`"*, and D21-46 R1 moves them here.
- D21-45 **W1**, **W2**, **W4** (the worktree half, and `init`'s exclude block) and **W3's carry at creation**.
- D21-46's first correction (the supersession targets), its second correction (`init`'s first write), **R8**, and the
  flagged ignore-row behaviour.
- D21-47 **V1** (the filter refusal fires only when pointer files would result) and its clause-5 rule as it applies
  here (the writes supersede, the reads do not).
- D21-48 **X1** (the ignore refusal is taken), **X2** (the post-checkout pointer scan, which completes V1 by
  observation) and **X14** (the carry at creation is this stage's call of `s8c`'s reader).

**What it does not own, and who does.**
- **`s8e`:** the `worktree_assigned` row that records the path, `reused` and `discarded` (`s8e` §8e.4). It also owns
  every tier-1 write and the lane's `closed` mark, to which removal is keyed (`s8e` §8e.6). Until `s8e` lands, this
  stage's discard list reaches the operator on stderr, not a journal row (§8d-ii.3).
- **`s8c`:** the `.worktreeinclude` reader and the per-toolchain `setup` verb (D21-45 W3; D21-43 N25). This stage calls
  both at creation and re-specifies neither. D21-48 X14: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at creation."*
- **`s8h`:** the live probe's edit inside `.redkiln/worktrees/<probe>`, and the protected-segment limit. D21-45 puts it
  this way: *"inside a run worktree, `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/` stay
  protected"*, which `s8h` maps to the typed `permission_denials` failure.
- **`s8a`:** how the `Text` variant classifies an LFS pointer path. §8d-ii.5's filter-driver refusal and its
  post-checkout pointer scan (D21-48 X2) are what it verifies against.
- **`s8b`:** its own catalog-quality walks over `.redkiln/`, which exclude `worktrees/` (D21-44).
- **`s8j`:** re-pointing or retiring the legacy `rk-fix-doctor-test-count` tree (D21-44: *"its re-point is s8j's"*),
  and every amendment to `s8-exit-criterion.md`, including its `grep -r … .redkiln/` lines, which must exclude
  `worktrees/` (§8d-ii.1).

**No stage before `s8i` dispatches to a real account, and this one dispatches nothing.** Every run in it is over a
fixture repository or the fixture child. Checkpoint 8d-ii's replays and its own process-level cargo runs go through
the shared no-dispatch wrapper, `runbook/tools/nodispatch.sh` (D21-47; §8d-ii.11). That is a compiled fake `claude`
first on `PATH` that prints the pin for `--version` and logs and refuses anything else, every other `PATH` directory
holding a `claude` removed, `REDKILN_LAUNCH_RECORD` unset, and `CLAUDE_CONFIG_DIR` at an empty trap directory.

**Order of work, and why.**
1. §8d-ii.0's measurements, before any code. The ignore refusal is answered (D21-48 X1), so nothing waits on the
   operator.
2. The one derivation, the item-id validator and the ignore check (§8d-ii.1, §8d-ii.2), because every other guard
   takes the validated item and the derived path.
3. The lifecycle: create, re-enter, refuse and remove (§8d-ii.3), then the carry (§8d-ii.4).
4. Git hygiene: the effective-config filter check, the post-checkout pointer scan (D21-48 X2), `init`'s exclude
   block and `LC_ALL` (§8d-ii.5, §8d-ii.2).
5. The fixture repositories' ignore rows and Checkpoint 7f's two amendments, in the commit that moves the root
   (§8d-ii.1, §8d-ii.7).

Every test of a new behaviour is written **first, red** (§8d-ii.8).

## Records this step writes

The record is **a new record (next free id at writing)**, in the ADR-0085 `### Option — <name>` heading form that
D21-42 24a makes binding from ADR-0164 onward. It may not drop a supersession listed here without a reason written
into the record.

| record | executes | supersedes in part | cites without superseding |
| --- | --- | --- | --- |
| **the run worktree lives at `.redkiln/worktrees/<item>` under the main worktree, is reused, and is guarded** | D21-44; D21-45 W1, W2, W4, and W3's carry; D21-46's two corrections and R8; D21-47 V1 and its clause-5 rule; D21-48 X1, X2 and X14 | **ADR-0137 decision 3's path literal** *"`<common-dir>/redkiln/worktrees/<item>`"*. Its rule, *"**`project_root` is the run-owned worktree**"*, stands. **ADR-0105 clause 6's** *"A worktree is **re-created**, never reused, per run"*, and the *"a reused worktree"* member of its bullet *"Forbids a wrapper write not on clause 5's list, a `\\?\` path handed to git, and a reused worktree."* (W2), **for the run-owned worktree only**: `redkiln-migrate`'s worktree keeps clause 6 whole (AC-522's *"re-created, never reused — ADR-0105 clause 6"*). **ADR-0105 clause 5's closed list**, *"The wrapper's write set is enumerated"* … *"Anything not listed is a successor"*, for the **writes** §8d-ii.6 names (lock, unlock, the `RK_BASE` marker, and the discard's `reset --hard` and `clean -fd`). The reads are not a supersession (D21-47). **ADR-0107 decision 2**, *"a second `init` on an initialised repository changes nothing and exits 0"*, in part: the first `init` after this stage on a repository initialised before it appends the `# redkiln-runtime` block (§8d-ii.5) | **ADR-0018, NOT superseded** — it names no worktree path (§8d-ii.1). **ADR-0162 decision 10, NOT superseded** — the review reference tree stays scratch and is removed on every path; its contrast clause names `recreate_run_worktree`, which this record retires, and the record says so (§8d-ii.3). ADR-0105 clause 8; ADR-0105's rejected option *"reuse worktrees across runs"* (answered in its own terms, §8d-ii.3); ADR-0109 decision 4; AC-518, AC-522, AC-529 and AC-531 (dated notes); design §5's `reconcile_started` row and §6's GitOps row (dated notes); Halt 6 |

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

**Three more targets, found by the adversarial review (D21-47) and verified by quotation at `fc80687`:**
- `references/adr/0107-one-idempotent-run-entry-and-no-resume-flag.md`, decision 2: *"**`init` is idempotent**: a
  second `init` on an initialised repository changes nothing and exits 0."* A repository initialised before this
  stage has no `# redkiln-runtime` block, so its next `init` writes one. That `init` changes something, so the record
  supersedes the sentence in part rather than argue it away. From the block's first write on, a second `init`
  changes nothing again.
- ADR-0105 clause 6 is superseded **for the run worktree only.** `references/2026-09-08-acceptance-criteria.md`'s
  AC-522 verifies `redkiln-migrate`'s worktree as *"left in a state the next run refuses to reuse (re-created, never
  reused — ADR-0105 clause 6)"*. That worktree keeps clause 6, and AC-522 takes a dated note saying so.
- Design §6's GitOps row reads *"`git worktree add` / `git worktree remove` (re-created per run, never reused; …)"*.
  It takes a dated note beside §5's `reconcile_started` note.

ADR-0162 decision 10 is **cited, not superseded**: *"the pass **removes its own reference worktree on every path,
including every refusal path**"* stands (§8d-ii.3). D21-47's clause-5 rule also takes the reads out of this record's
clause-5 supersession: *"A READ is not a supersession; s8d-ii drops its reads from its supersession"*.

**The ignore refusal is taken (D21-48 X1), and the record states it (§8d-ii.1).** D21-46 flagged it: *"The
run-worktree root must be git-ignored before creation proceeds, which makes AC-518's instrument depend on an ignore
row. That is a behaviour no earlier line names (s8d-ii)."* The adversarial review (D21-47) made it a stage-open STOP,
because the draft built the refusal before the answer. The operator answered on 2026-09-24, D21-48 X1: *"**Refuse by name.** This answers D21-46's flip-line flag."*
So creation refuses by name when the root is not ignored, and there is no "warn" branch:
`worktree::creation_refuses_when_the_worktree_root_is_not_ignored` and `S8D-II-M11` stand as written, and nothing
moves to `lane_e2e` (`s8e` §8e.11). The filter reading needs no question either (D21-47 V1), and the one case V1's
configuration read cannot see is closed by D21-48 X2's post-checkout scan (§8d-ii.5).

**Attendant edits, each dated and each in the commit that makes it true:**
- design §5's `reconcile_started` row, whose recomputation clause names *"`<common-dir>/redkiln/worktrees/<item>/`"*;
- design §6's GitOps row, whose write set reads *"re-created per run, never reused"*;
- dated register notes on **AC-518**, **AC-522**, **AC-529** and **AC-531** (§8d-ii.1, §8d-ii.5, and the targets
  above), each inside its own criterion's block;
- dated notes on `s7d` §7d.5's two tables that name the old path;
- the rigged stub's doc in `rk-fixture-toolchain.rs`, which names the candidate arm's directory as
  *"`…/worktrees/<item>`"*. `s8c` names this correction as the path move's. The corrected doc names
  `.redkiln/worktrees/<item>`;
- the dated amendments to Checkpoint 7f that §8d-ii.7 spells out;
- the record's row in `references/adr/README.md`;
- a ledger entry that names the record as `ADR-NNNN`, **with the id it took at writing**. The same commit writes that
  id into Checkpoint 8d-ii's `REC_WT=____` line as a literal, by a dated edit, and the block locates every record by
  it (D21-47). The block refuses while the placeholder remains, and no re-runner supplies the id (final consistency
  pass 2026-09-24);
- the back-links (`superseded_by`) on ADR-0105, ADR-0107 and ADR-0137 are **not** written with the record. They move in
  the flip commit, as `s8h` Checkpoint 8h holds its own (*"the link moves in the flip commit"*), and Checkpoint 8d-ii
  asserts whichever state the record's status requires.

### 8d-ii.0 Stage-open measurements — re-taken at Checkpoint 8d's close, before any code

Each is read-only. A 🛑 line is a STOP: the finding goes to the operator, and the stage does not choose around it.
*D21-48 (2026-09-24):* no item below is a 🛑 any more; X1 took item 9's question.

1. **Where the legacy tree is registered.** At `c5d19d7`, `git worktree list` lists
   `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count` at `73c087a`. `s8a` runs Checkpoint 7f
   whole at its exit, and 7f's block, as `s8a` leaves it, ends with `git worktree remove --force
   .git/redkiln/worktrees/rk-fix-doctor-test-count`. `s8a` guard (11) restores the registration after that run and
   asserts the ref unmoved (`s8a` finding (8)), so at this stage's open the registration should still be present
   (§8d-ii.9, finding 4). Record which. Checkpoint 8d-ii asserts the ref and its mark, never the registration.
2. **The worst-case run-tree path (D21-44's last consequence).** At `c5d19d7` the longest tracked path is 140
   characters, `references/adr/0114-the-executed-once-gate-moves-to-extended-to-shipped-promotion-and-the-v0-pack-tags-on-lint-parse-and-a-visible-ledger.md`,
   and it still is at `fc80687` [measured]. The root is the **main** worktree's directory (§8d-ii.1), and
   `D:/repos/redkiln-rs/.redkiln/worktrees/` is 39 characters. The derived review tree is named `<item>.review-ref`
   (`reference_worktree`'s `format!("{item}.review-ref")`), so the bound for an item is 39 + len(item) + 11 + 1 + 140.
   For the longest item the validator admits (64) that is **255**; for Checkpoint 7f's fixture item
   `rk-fix-doctor-test-count` (24) it is **215**. Windows' 260-character `MAX_PATH` leaves 259 usable, and `git config
   --get core.longpaths` exits 1 with no value on this box. Under the old root (43 characters) the 64-character sum
   was **259**, at the limit. **The rule is computed on the actual item** (§8d-ii.6). A rule on the 64-character
   maximum leaves a 23-character root budget, which Checkpoint 7f's clone could never meet (the adversarial review,
   D21-47). Checkpoint 8d-ii recomputes both sums from the tree, and the clone's own (§8d-ii.7).
3. **The effective filter configuration on this box (R8, as D21-47 V1 reads it).** At `c5d19d7`, `git config
   --show-scope --show-origin --get-regexp '^filter\.'` prints four `system` rows from `C:/Program
   Files/Git/etc/gitconfig`: `filter.lfs.clean`, `filter.lfs.smudge`, `filter.lfs.process` and `filter.lfs.required
   true`. They are unchanged at `fc80687` [measured]. That is Git for Windows' default, present for **every**
   repository on this machine. `git check-attr filter` selects no driver for any tracked path of this repository,
   and `--get-regexp '^includeif\.'` exits 1. Read literally, *"refuse when a filter driver is declared at any
   scope"* would refuse every run on this box. **The operator settled the reading on 2026-09-24 (D21-47 V1):**
   *"**Refuse only when pointer files would result.** That is, when a tracked path at the fork commit selects a filter
   whose driver is NOT effective at any scope, or smudge is skipped."* So this item is no longer a STOP.
   Re-measure. A tracked path of this repository that selects a filter is admitted when its driver is effective, and
   Checkpoint 8d-ii's (0c) checks that for every path selected at `HEAD`.
4. **No code walks `.redkiln/` recursively.** At `c5d19d7`, every `read_dir` under `crates/*/src` that touches
   `.redkiln/` reads one named subdirectory, one level deep: `ctx.redkiln().join("cards")` and `.join("specs")` in
   `doctor/rules.rs`, `redkiln_dir.join("skills")` in `doctor/roles.rs`, `root.join(".redkiln").join("catalog")` in
   `run.rs`'s `catalog_sources`, and `Catalog::from_dir` over a directory it is handed. `init`'s `preexisting_rk_names` is *"Top level only"*. Re-measure. A recursive walk found here takes the
   exclusion and its control in the same commit (§8d-ii.1).
5. **`init` writes no hook, and no file git itself reads.** D21-46's second correction reads *"`init` writes no
   hook, and `# redkiln-hook` exists only as doctor's recogniser."* At `c5d19d7`, `crates/rk-cli/src/verbs/init.rs`'s
   module doc reads *"It does exactly two things: writes `.redkiln/config.toml` **if and only if the file is
   absent**, and opens the store"*, and *"installs no hook (AC-412)"*. The only `# redkiln-hook` on the tree is
   `doctor`'s recogniser, `const MARKER: &str = "# redkiln-hook";`. So the exclude block is **`init`'s first write
   to a file git reads** (§8d-ii.5). It is not `init`'s first write under the common dir: opening the store already
   creates `<git-common-dir>/redkiln/store.sqlite3` (the adversarial review, D21-47).
6. **Every fixture repository a run creates a worktree in.** List every fixture under `crates/*/tests/fixtures/` that a
   test runs `redkiln run` (or `verbs::run::run`) over. At `c5d19d7` that includes AC-438's exit table,
   `exit/findings-block/` and `cost`'s run transcript repository. Each gains the `.redkiln/worktrees/` ignore in the
   commit that moves the root, or creation refuses and its exit moves to `3` (§8d-ii.1).
   **Repositories built in code take the same row** (the adversarial review, D21-47). At `fc80687` the `rk-cli`
   integration tests build repositories with:
   - `temp_repo` (`tests/it/main.rs`) and `pack_min_repo` (`support.rs`), the shared helpers;
   - `repo_with_open_gate` (`answer.rs`), `synthetic_trailer_repo` (`doctor.rs`), and `repo_seeded_from` and
     `repo_with_record` (`explain.rs`);
   - the `adr0128_*` tests' own repositories (`run.rs`, `surface.rs`).

   The files whose tests invoke `run` are `config.rs`, `cost.rs`, `exit.rs`, `review_pass.rs`, `root.rs`, `run.rs` and
   `surface.rs` [measured by grep at `fc80687`]. `answer.rs` and `transcription.rs` build their repositories through
   the shared helpers. The row goes into the two shared helpers as the `/.redkiln/worktrees/` line `init` writes to
   `info/exclude`, and into every other builder a `run` reaches, in the commit that moves the root.
   `worktree::creation_refuses_when_the_worktree_root_is_not_ignored` builds its own repository without the row.
   Re-measure the list. A test that fails with the ignore refusal's name after the move is a builder this list
   missed.
7. **The test module names are free.** This stage extends `rk-git` `worktree` and `gitops` and `rk-cli` `init`, and it
   adds `rk-cli` `run_worktree`. If a predecessor created `run_worktree`, the tests go into it. The module-qualified
   names in §8d-ii.8 stay as they are.
8. **The whole-suite floor.** The expected value is **re-derived at stage open from Checkpoint 8d's recorded
   count**, plus this stage's **50** new tests (§8d-ii.8). Computed once, on the final chain: 740 (ADR-0164's
   evidence, `992d257`) + 9 (8a) + 25 (8b) + 78 (8c) + 62 (8d) = 914 at Checkpoint 8d, so 914 + 50 = **964**. Replace
   964 in Checkpoint 8d-ii's `FLOOR=` line with Checkpoint 8d's recorded count plus 50, by a dated edit, before the
   block's first run. It is never lowered to make a red run green. *(It read 32 and 917 before the adversarial-review
   fixes of 2026-09-24, and 885 + 46 = 931 before the final consistency pass of 2026-09-24. *D21-48 (2026-09-24):* it read 46
   and 947 before X2's four tests. *D21-48 (2026-09-24), floor propagation:* it read 901 + 50 = 951 before `s8c`'s
   78 and `s8d`'s 62.)*
9. **The ignore refusal is no longer a STOP** (D21-48 X1, 2026-09-24). The draft asked it here, before any code,
   because D21-46 had flagged it for the flip line and the ledger carried no answer. D21-48 X1 answers it:
   *"**Refuse by name.** This answers D21-46's flip-line flag."* Re-measure only that the ledger line stands: `spikes/DECISIONS-2026-09-21.md`'s
   D21-48 table carries the X1 row. §8d-ii.1 builds the refusal.

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
sites in `crates/rk-cli/tests/it/run.rs` and `answer.rs` (still four and six at `fc80687` [measured]). After this
stage, `rk_git::worktree` owns the one derivation. It takes the **main worktree's** directory and a validated item
(§8d-ii.2) and returns the path and the branch together. All four sites, every test support helper, and every later
test that plants a tree under the root call it. (`s8e` §8e.3 later deletes `story_trailer` and `terminal_read`'s
commit.) Checkpoint 8d-ii's code-line floor asserts that no code line under `crates/*/src` or `crates/*/tests`,
outside `rk-git`'s worktree module and its own test module, derives a `worktrees` directory in any spelling:
`join("worktrees")`, `join(".redkiln/worktrees")`, or a joined literal. The exceptions are reads of **git's own**
administrative directory, which does not move: `doctor`'s `ctx.common_dir.join("worktrees")`, and the two tests that
read it (`doctor.rs`'s `common.join("worktrees")` and `s_gix_a.rs`'s `".git/worktrees/linked"`). The draft's floor
read `crates/*/src` only and the literal `"worktrees"` only, so `root.join(".redkiln/worktrees")` passed it (the
adversarial review, D21-47).

**The root is the main worktree, whichever checkout `run` starts in** (the adversarial review, D21-47).
`--show-toplevel` names the checkout a command runs in, so a root read from it depends on where `run` starts. A run
started from a linked worktree would put its tree under that checkout. This box has one: `D:/repos/redkiln-rs.swt`,
on `spike/s-h1b-s-wt` [measured at `fc80687`]. A run started inside a run tree would nest a run tree inside it. The
derivation reads the first `worktree` record of `git worktree list --porcelain`, and git lists the main worktree
first. A repository whose first record carries `bare` has no main worktree, and creation refuses it by name
(`worktree::a_bare_repository_is_refused_by_name`). The ignore check, the length rule and `init`'s exclude block read
the same root. `worktree::the_root_is_the_main_worktree_when_run_from_a_linked_worktree` pins it.

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
- **Creation refuses by name** when `git check-ignore`, run in the main worktree, does not report
  `.redkiln/worktrees/<item>` as ignored (`worktree::creation_refuses_when_the_worktree_root_is_not_ignored`). The
  alternative is a run that dirties the user's checkout by existing. **This is the behaviour D21-46 flags, and D21-48 X1
  takes it:** *"**Refuse by name.** This answers D21-46's flip-line flag."* The record states the refusal.

**The dated register notes.**
- **AC-518**'s THEN reads *"refused unless its target is a run-owned worktree or the store under the resolved common
  dir"*. The note records that the run-owned worktree now sits inside the checkout's directory and is a separate
  checkout. It also records that the instrument's `git status --porcelain` holds through the ignore rows, and that
  `s8e`'s `lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it` is the instrument's run.
- **AC-529** carries no path. Its THEN reads *"the run-owned worktree is resolved **from the store**, never from
  cwd"*. The note records the move and D21-44's *"Ownership stays with the store row, so AC-529 stays
  real wherever the path is"*. It names `s8e` §8e.4's `worktree_assigned` as that row.
- **AC-522** keeps its instrument. Its note records that ADR-0105 clause 6's *"re-created, never reused"* still
  governs `redkiln-migrate`'s worktree, and that the record supersedes clause 6 for the run-owned worktree only.

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
  - it has no leading `.`, no trailing `.`, no `..` anywhere (`a..b`), and no `.lock` suffix (git's ref rules for
    `rk-run/<item>`);
  - it is not a Windows reserved device name, case-folded, with or without an extension: `CON`, `PRN`, `AUX`, `NUL`,
    `COM1` to `COM9` and `LPT1` to `LPT9` (so `nul.txt` and `com1` are refused too);
  - it has no `.review-ref` suffix, case-folded. That rule was found here: `reference_worktree` names its tree
    `<item>.review-ref` in the same directory, so an item id ending that way would collide with another item's
    reference tree.

  One table-driven test asserts each row's own refusal reason. The adversarial review (D21-47) added `a..b`, the
  trailing `.` and the device names. It also added one rule the table cannot hold, because it reads the registry: an
  id that case-folds onto a registered run tree or an existing `rk-run/` branch without equalling it is refused
  (`worktree::an_item_id_that_case_folds_onto_a_registered_tree_is_refused`). Windows' filesystem folds case and
  git's refs do not, so `Fix-1` and `fix-1` would share a directory and not a branch.
- **Symlink guards.** Creation refuses when `.redkiln`, `.redkiln/worktrees` or `<item>` is a symlink, or on Windows a
  junction. After `git worktree add`, the created tree's canonical path must lie under the canonical
  `.redkiln/worktrees/`, or the tree is refused and the refusal names both paths. Claude Code's reason is the one
  quoted in D21-45: *"A repository-committed symlink at .claude … could redirect worktree creation outside the
  repository"*. The test builds a **junction** on Windows, which needs no Developer Mode, and a symlink elsewhere, and
  it says which it ran. `std::fs::canonicalize` returns a `\\?\` path on Windows. The canonical form is used only to
  compare, and it is **never handed to git**: ADR-0105's forbids bullet bars *"a `\\?\` path handed to git"*, so git
  receives the drive-letter form.
- **The `RK_BASE` marker.** At creation, `RK_BASE` is written into the tree's own gitdir, the one `git rev-parse
  --git-dir` prints from inside it. It holds the fork sha and the item id, so ownership is recoverable from git alone
  (ADR-0109 decision 4). Redkiln never removes a run tree without it. It is a file that `rk-git` writes, not a git
  invocation, so it has no `GitOp` variant. It is still a write into git's admin area, and D21-47 names the marker
  among the writes that supersede clause 5 in part (§8d-ii.6). The review reference tree carries one too
  (§8d-ii.3).
- **The lock.** `git worktree lock --reason "redkiln run <item> (pid N start T)"` is taken at every run start and
  released at the run's end. T is the process's start time as the OS reports it. A lock is **live** when its pid is
  alive **and** that pid's start time equals T. A lock whose pid is dead, or whose pid now names a process with a
  different start time, is released (`worktree::a_lock_held_by_a_dead_pid_is_released`,
  `worktree::a_lock_whose_pid_now_names_another_process_is_released`). A live lock refuses the run by name.
  - **The liveness predicate is injected by `rk-cli`, so `rk-git` gains no edge to `rk-containment`.** At `fc80687`
    `rk-git`'s `[dependencies]` are `gix` and `rk-types`, and `rk-cli` already depends on `rk-containment`.
  - `rk-cli` builds the predicate from `rk_containment::is_alive` and a new `rk_containment::process_start_time`. The
    new function reads Windows' `GetProcessTimes` creation time, and `/proc/<pid>/stat` on Linux; the POSIX arm stays
    ARMED to CI, as that crate's POSIX arm is. It gets its own test,
    `containment::process_start_time_reads_a_live_child_and_nothing_for_an_exited_one`.
  - `is_alive` is `#[doc(hidden)]` *"Test support"* at `fc80687`, and this makes it a production call. The record
    says so.
  - Its documented ambiguity is **an exit code**, not pid reuse: *"a process that exits with code 259
    (`STILL_ACTIVE`'s value) would read alive, the documented ambiguity of this API"*. Such a pid reads alive. If its
    start time still equals T, the run refuses by name. That is fail-closed, and the record says so. Pid reuse is
    what the start-time comparison answers.
- **The re-entry check.** A registered tree is reused only if it is a separate checkout **of this repository**:
  `git rev-parse --git-common-dir` from inside it equals the repository's, and `git worktree list --porcelain` lists
  its path.
- **`LC_ALL=C` on every parsed git call.** `GitExec`'s child environment sets it at the one process-exec site,
  `run_with_env`'s `std::process::Command::new("git")`, in the call form `.env("LC_ALL", "C")`, so no parse depends
  on the operator's locale (`gitops::every_git_child_runs_with_lc_all_c`).
  *(dated 2026-09-26, s8c build, P6.)* The literal is now in `git_exec.rs`'s private `spawn`, which `run_with_env`
  calls with no stdin and `GitExec::run_with_stdin` calls with one (s8c §8c.4, the `.worktreeinclude` reader's
  `check-ignore --stdin`). It is still the one process-exec site, and the `.env("LC_ALL", "C")` belongs on its
  `cmd` builder there. The floor's grep reads the whole file, so it is unaffected.
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
  `origin/<default>`"*). The fork sha is read once, before the add, and it is what `RK_BASE` records.
- **The branch exists but no tree is registered:** `git worktree add <path> rk-run/<item>`. That happens after a
  landed removal, or after a deleted tree. **Never `-B`** (`worktree::an_existing_run_branch_is_never_reset`).
- **Re-entry:** the registered tree is reused when it carries `RK_BASE`, passes the re-entry check, and its `HEAD` is
  **symbolic**, on `refs/heads/rk-run/<item>`, and equals that ref's tip. In-flight changes are read first, path by
  path, through `GitOp::Status` (`--porcelain=v1 -z --untracked-files=all`). They are then discarded explicitly
  (`reset --hard HEAD`, then `clean -fd`), and the lifecycle function **returns** every discarded path in its typed
  result. That is N10's semantics: what was not landed at a committed step is not work. `clean` runs **without
  `-x`**, so ignored files survive re-entry: W3's carried files and the `setup` verb's `node_modules` stay
  (`worktree::reentry_keeps_ignored_files`).
- **"Journalled" is met at `s8e`, and the interval is stated rather than hidden.** The journal row is `s8e` §8e.4's
  `worktree_assigned { …, reused, discarded }`, and that kind does not exist until `s8e` mints it. Between Checkpoint
  8d-ii and `s8e`'s commit, `run` prints each discarded path on stderr, one line per path. From `s8e` on, the same
  typed result fills `worktree_assigned.discarded`. The rk-git test asserts the returned list, and `s8e`'s tests
  assert the row.
- **Refused, never removed:** a tree with no `RK_BASE`, a tree registered to another repository, a tree whose `HEAD`
  has commits the run ref lacks, and a tree on a **detached** `HEAD`
  (`worktree::a_tree_on_a_detached_head_is_refused_at_reentry_and_kept`). A detached `HEAD` can equal the tip and
  still not be on the run ref, so an equality test alone would pass it. Each refusal names the path and the reason.
- **Removal** happens only when the tree is clean, landed, marked and unlocked. It uses `git worktree remove`
  **without `--force`**. Anything else is kept, with a reason in Claude Code's form (*"N changed file(s) would be
  lost"*). Redkiln never removes a run tree it did not create. The one removal of a tree that is not landed is D21-48 X2's
  rollback of the tree the current call has just created, before anything runs in it, and it is unforced too
  (§8d-ii.5). This stage builds the removal rule and its tests. **The
  trigger is `s8e`'s**: removal is keyed to the lane's `closed` mark (`s8e` §8e.6), which does not exist before it.
- **What "landed" means here.** The draft read it as *"`HEAD` equals the run-ref tip"*. Re-entry already requires
  that, so as a removal condition it was tautological (the adversarial review, D21-47). In Phase 1 nothing is
  integrated: `s8f` §8f.4 reads *"**Nothing is integrated.**"*, so no integration ref exists to be an ancestor of. A
  tree is **landed** when:
  1. its `HEAD` is symbolic on `rk-run/<item>` and equals the tip;
  2. its status is empty;
  3. the tip's `Story:` trailer is the lane's terminal `closed` mark (`s8e` §8e.6).

  So every change the tree holds is on the ref the operator merges, and the lane has ended. Before `s8e` mints
  `closed`, condition 3 reads a `Story:` trailer whose stage value is `closed`, which the rk-git fixture writes
  directly. When Phase 2's merge queue exists, condition 3 gains the integration ref's ancestry, by the record that
  brings the queue. `worktree::an_unlanded_tree_is_kept_with_its_reason` builds a clean, marked, unlocked tree whose
  tip carries no `closed` mark, and asserts that it is kept.
- **The review reference tree keeps ADR-0162 decision 10, and it moves to the fork sha.** Decision 10 reads *"the pass
  **removes its own reference worktree on every path, including every refusal path**"*. Its forbids bullet reads
  *"Forbids leaving the reference worktree registered after the pass returns, on any path including every refusal
  path"*. W2's rule is for the run-owned worktree, which is the run's output. The reference tree is scratch, so it
  keeps its forced removal: `remove_reference_worktree`'s `worktree remove --force`, and its `remove_dir_all`
  fallback, which does not follow a junction. The draft extended W2's rule to the reference tree. That would have
  kept a dirty reference tree registered, which moves decision 10 with no supersession. The adversarial review found
  it (D21-47). Two things do change:
  - **It is created at `RK_BASE`'s fork sha, not at `HEAD`.** `reference_worktree`'s doc argues *"[`recreate_run_worktree`]
    creates `rk-run/<item>` **at `HEAD`**, so the base of `HEAD` and that branch IS `HEAD`"*. Under reuse that
    premise is false. The run branch forked at an earlier `HEAD`, and the operator's checkout may have moved since.
    The fork sha in the run tree's `RK_BASE` is the base, read without a `merge-base`, which clause 5 still does not
    carry. `run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves` moves `HEAD`
    between two invocations and asserts that the reference tree's `HEAD` is the recorded fork sha. The doc is
    rewritten in the same commit.
  - **A leftover is removed only if redkiln made it.** The reference tree gets its own `RK_BASE` at creation. A
    leftover at `<item>.review-ref` from a killed run that carries the marker is force-removed as scratch, as today.
    One without it is refused by name and kept
    (`run_worktree::a_leftover_reference_tree_without_rk_base_is_refused_and_kept`).

  Decision 10's contrast clause, *"unlike the run-owned worktree, which `recreate_run_worktree` recreates and never
  removes"*, describes a function this record retires. The record cites decision 10, says so, and supersedes none of
  it, because the decision's rule stands.
- **Halt 6 is answered, not overridden.** Halt 6 reads *"If a fixture tree will not delete, that is the handle, and
  reusing the tree corrupts the next arm."* Reuse here is of a run's **own** tree, by the same item, only when its
  `HEAD` is on the run ref at its tip, and only after an explicit discard. A tree that will not delete is kept and
  named, and it is never handed to another item. The record states this. It answers ADR-0105's rejected option
  *"reuse worktrees across runs"* in that option's own terms, *"a fresh worktree per run is the only state the kernel
  can assert"*. The asserted state is now `RK_BASE`, plus a symbolic `HEAD` equal to the tip, plus an empty status
  after the discard.
- **Removal is by run state, never by age.** D21-45: *"there is no `cleanupPeriodDays` analogue"*.

### 8d-ii.4 W3's carry, at creation only

**The decision.** W3: *"**Honour the project's existing `.worktreeinclude`**"* and *"**Dependencies come through
N25's per-toolchain `setup` verb.**"* D21-45's ownership line: *"W3 and W4's `init` half go to **s8c** (the `setup`
verb and `.worktreeinclude` reading) and s8e (the carry at creation)"*. Under D21-46 R1 the carry is this stage's.
D21-48 X14 confirms it: *"**Confirmed:** s8c builds the reader, and s8d-ii calls it at creation."*

After `git worktree add`, the realpath check, the `RK_BASE` write and D21-48 X2's post-checkout pointer scan
(§8d-ii.5), and before the lock is released to the run, creation:
1. calls `s8c`'s `.worktreeinclude` reader (ignored ∩ matched, symlinks skipped, escapes refused) and copies what it
   returns;
2. then runs `s8c`'s per-toolchain `setup` verb (`pnpm install --frozen-lockfile --prefer-offline` for each declared
   Node fixture project).

A `setup` failure is `s8c`'s by-name refusal, and the run surfaces it and stops
(`run_worktree::a_failed_setup_refuses_by_name`). `settings.local.json` is not copied, because children run with
isolated settings (`s8h`). No `node_modules` symlink is made (W3: *"they share mutable state across runs"*). A reused
tree is not re-carried, because its ignored files survived the discard (§8d-ii.3).

### 8d-ii.5 W4 — git hygiene, at every config scope (D21-46 R8, as D21-47 V1 reads it)

**The decisions.** W4: *"**Both.** Worktree creation is refused, with a diagnosis, when a repo-local filter driver
(for example LFS via `--local`) or an `includeIf` would yield pointer files. **`redkiln init` writes an idempotent
`# redkiln-runtime` block to `<commondir>/info/exclude`** for target repos, and this repo keeps its tracked
`.gitignore` row."* R8 widens the first half: *"**The effective config at every scope.** Refuse with a diagnosis naming
the scope. BrittonGames' LFS filter is system-scope and would have passed a repo-local check."* D21-47 V1 settles what
the refusal refuses: *"**Refuse only when pointer files would result.** That is, when a tracked path at the fork
commit selects a filter whose driver is NOT effective at any scope, or smudge is skipped. Attributes are read from the
fork commit. An installed, working driver (BrittonGames' system-scope LFS) is admitted silently. This **refines D21-46
R8**, whose intent was D21-45 W4's *"would yield pointer files"*."* `s8i` §8i.0 item 12 measured BrittonGames: it
*"carries its LFS filter at **system** scope (`filter.lfs.clean`, `.smudge`, `.process`, `required true`). `git config
--local --get-regexp '^filter\.'` and `'^includeif'` both return rc 1"*.

**The draft had the refusal inverted** (the adversarial review, D21-47). It refused a selected driver that was
declared, which is the working case, and it admitted a selected driver that was declared nowhere, which is the case
that yields pointer files. V1 turns it the right way round.

**What it reads, before `git worktree add`.** Every git read goes through `GitOps`, with `LC_ALL=C`. They are reads,
so they supersede nothing (D21-47's clause-5 rule).
1. **The selecting paths, at the fork commit.** `git check-attr --source=<fork> filter` over every path `git ls-tree
   -r --name-only <fork>` lists. The attributes are the fork commit's `.gitattributes`, not the working tree's, which
   is what a run tree checks out. Git 2.55 on this box takes `--source`: `git check-attr --source=HEAD filter --
   Cargo.toml` printed `Cargo.toml: filter: unspecified`, rc 0 [measured at `fc80687`].
2. **The drivers, at every scope.** `git config --show-scope --show-origin --includes --get-regexp '^filter\.'` over
   the resolved configuration: `system`, `global`, `local`, `worktree` and `command` scopes, with every `include`
   followed and every `includeIf` whose condition matches.
3. **The run tree's own `includeIf`s.** `--get-regexp '^includeif\.'` at every scope, so that an `includeIf` whose
   condition matches the **run tree** (a `gitdir:` pattern that covers `.redkiln/worktrees/`) but not the user's
   checkout is read from the file it names and folded into (2).
4. **`GIT_LFS_SKIP_SMUDGE`**, which `rk-cli` reads through `env.rs`, the one environment read (ADR-0083), and passes
   in.

**When it refuses.** For each driver a selecting path names, the driver is **effective** when a
`filter.<name>.process` or `filter.<name>.smudge` value resolves at some scope. Creation refuses when:
- **(a)** a selected driver has no effective value at any scope. Git then checks out the committed content, which
  for LFS is the pointer. The diagnosis names the driver, one path at the fork commit that selects it, and every
  scope it read;
- **(b)** smudge is skipped: `GIT_LFS_SKIP_SMUDGE` is set to a value other than `0`, or a selected driver's resolved
  `smudge` value carries `--skip`. The diagnosis names the driver, one selecting path, and the scope and origin file
  that skip it, or the environment variable. That is R8's *"Refuse with a diagnosis naming the scope"*.

**When it admits, silently.** A selected driver with an effective value is admitted, and so is a driver declared and
selected by no path. Neither prints at the default level; either fact goes to the **debug** level only. The draft
printed a diagnosis line for a declared, unselected driver on every run, and on this box that line would differ per
machine (the adversarial review, D21-47).

**BrittonGames is admitted, as `s8i` says.** `s8i` §8i.0 item 12 reads *"D21-45 W4 would not refuse BrittonGames"*, and
§8i.8 repeats it. The draft said the opposite and contradicted `s8i`; V1 settles it on `s8i`'s side. BrittonGames'
tracked `.gitattributes` selects `lfs`, and its `filter.lfs.*` is effective at `system` scope.
`worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently` reproduces that shape with
`GIT_CONFIG_SYSTEM` pointed at a fixture file and a portable fixture driver, so no test reads the machine's real system
configuration or needs `git-lfs` installed. The `global` arms use `GIT_CONFIG_GLOBAL` in the same way.

**The case a configuration read cannot see is closed by observation (D21-48 X2; finding 15).** An effective driver
whose command is not installed, and that is not `required = true`, makes git keep the committed content, which yields
pointers. V1 admits *"an installed, working driver"*, and the read above sees only that a driver is effective. The
operator took it on 2026-09-24, D21-48 X2: *"**A post-checkout scan.** After creation, the paths the filter selects are scanned for LFS pointer headers;
any hit refuses by name and removes the fresh worktree."* D21-48 says the
scan completes V1's *"refuse only when pointer files would result"* by observation.

**The post-checkout pointer scan.** It runs on a tree **this call created**, on first creation or when the branch
exists with no tree registered (§8d-ii.3), in this order: `git worktree add`, the realpath check, the `RK_BASE`
write, the scan, and only then the lock, the carry and `setup` (§8d-ii.4). Re-entry does not scan. A reused tree was
scanned when it was created, and the scan never judges a tree this call did not create.
- **What it reads.** The path set is the one read (1) already took: every path that `git check-attr
  --source=<fork> filter` names with a driver, at the fork commit `RK_BASE` records. For each path it reads the
  **checked-out file in the fresh tree**, never the blob at the fork commit. The committed bytes of a working LFS path
  are a pointer by design; the question is what the checkout left. A selected path the tree does not hold as a
  regular file is skipped, at the debug level.
- **What a hit is.** A file whose first line, with one trailing carriage return dropped, is the Git LFS pointer
  header `version https://git-lfs.github.com/spec/v1`. The scan reads the first line's bytes and no more, so it never
  reads a large smudged file whole. A pointer header at a path no filter selects is not a hit: a document that quotes
  the spec is not a pointer file.
- **The refusal is by name.** It names the driver, the number of hits, the first hit's path, and the scope and origin
  file of the driver's effective value (read (2)), so the operator sees which installed-looking driver did not run.
- **The rollback removes the fresh tree, under W2's rules.** Redkiln removes only a tree it created, and this is the
  tree this call created: it carries the `RK_BASE` this call wrote, with this call's fork sha, and it is unlocked,
  because the lock is not yet taken. The removal is `git worktree remove` **without `--force`**, which is on clause 5's
  list, so it supersedes nothing. The branch `rk-run/<item>` is kept where it stands: at the fork sha on a first
  creation, at its tip otherwise. Deleting it would be a write not on clause 5's list, and the next creation takes the
  "branch exists, no tree registered" path. If git refuses the unforced removal, the tree is **kept**, never forced,
  and the refusal names its path as kept. Nothing ran in the tree, so no work is lost either way.

**The tests.** Refusing:
- `a_selected_filter_with_no_driver_at_any_scope_refuses_naming_the_path`;
- `a_smudge_skipped_at_local_scope_refuses_naming_the_scope`, and the same at `global` and `system` scope;
- `git_lfs_skip_smudge_in_the_environment_refuses_by_name`;
- `an_include_if_that_skips_the_smudge_for_the_run_tree_refuses_naming_the_included_file`;
- `the_filter_attribute_is_read_from_the_fork_commit`. Its fork commit selects a driver that no scope declares, and
  the working tree's `.gitattributes` has been edited to select nothing;
- `a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree` (D21-48 X2). It
  builds finding 15's shape: the fork commit's `.gitattributes` selects a fixture driver for `*.bin`, a committed
  `a.bin` holds pointer bytes, and the driver is effective at fixture `system` scope with a `smudge` that names a
  command that does not exist and no `required`. It first asserts that the configuration read admits the driver and
  that `git worktree add` exits 0, so the scan refuses and not the read. It then asserts the refusal's name, its
  driver and path, that `.redkiln/worktrees/<item>` is absent and unregistered, and that `rk-run/<item>` stands at
  the fork sha.

Kept, never removed: `the_pointer_scan_never_removes_a_tree_this_call_did_not_create` (D21-48 X2). A registered,
marked tree whose selected path holds pointer bytes is re-entered. It is reused and kept, and nothing is scanned or
removed.

Admitting: the BrittonGames shape; a driver declared at `system` scope and selected by no path, which is this box's
own shape; a repository with neither a driver nor an `includeIf`; and, for the scan (D21-48 X2), a committed pointer
that a working fixture driver smudges (`a_committed_pointer_that_the_driver_smudges_creates_silently`) and a pointer
header at a path no filter selects (`a_pointer_header_at_a_path_no_filter_selects_does_not_refuse`). **This refusal is what `s8a`'s `Text` variant
verifies an LFS pointer path against**, so a run tree never materialises pointers that creation could have predicted.

**`init` writes the `# redkiln-runtime` block — its first write to a file git itself reads.** D21-46's second
correction: *"The `info/exclude` runtime block is therefore `init`'s first write into a target repo's git admin area,
and s8d-ii's record says so."* That wording is not literally true, and the record says what is. `init` already writes
under the common dir: it creates the store at `<git-common-dir>/redkiln/store.sqlite3`, which AC-531's 2026-09-18
amendment names. What is new is a write to a file **git reads**. `info/exclude` changes what `git status` reports. The
adversarial review found the gap (D21-47). The block's idempotence is **new, not inherited**, because `init` had no
earlier write to a git-owned file to be idempotent about. `init` appends to `<commondir>/info/exclude`, creating the
file if git's template did not:

```
# redkiln-runtime
/.redkiln/worktrees/
# end redkiln-runtime
```

- **Idempotent once written, and a supersession for the one case where it is not.** A second `init` finds the marker
  and leaves the file byte-identical. That holds ADR-0107 decision 2, *"a second `init` on an initialised repository
  changes nothing and exits 0"*, for this file once the block exists. The **first** `init` after this stage on a
  repository initialised before it does change something: it appends the block. So the record supersedes decision 2
  in part for that case (the records table). `init::init_writes_the_redkiln_runtime_exclude_block_once` starts from a
  repository in the pre-stage initialised shape (`.redkiln/config.toml` present, no block), so it pins exactly that
  case: one append, then byte-identical.
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
enumerates the wrapper's write set and says *"Anything not listed is a successor"*. D21-47 gives the clause one
reading: *"A git WRITE not on its closed list is a supersession in part"*, and *"A READ is not a supersession; s8d-ii
drops its reads from its supersession"*. So the worktree record supersedes clause 5 in part for **the writes only**:
- `worktree lock` and `worktree unlock`;
- the discard's `reset --hard HEAD` and `clean -fd`, on the owned tree only;
- the `RK_BASE` marker. It is a file `rk-git` writes into the tree's gitdir, not a git invocation, so it has no
  `GitOp` variant. It is named because it writes into git's admin area and D21-47 names it among the writes.

`worktree add` (with `--no-track -b`, with an existing branch, and `--detach` for the reference tree) and `worktree
remove` are already on clause 5's list. The reads this takes are `worktree list --porcelain`, `rev-parse
--git-common-dir`, `status --porcelain=v1 -z`, `config --show-scope --show-origin --includes --get-regexp` at every
scope, `check-attr --source=<fork>`, `ls-tree` and `check-ignore`. Each gets a `GitOp` variant, and none supersedes
anything. D21-48 X2's post-checkout scan adds no git read: it reuses the `check-attr` path set and reads files in the
fresh tree. Its rollback is `worktree remove` without `--force`, already on clause 5's list. `init`'s `info/exclude` append is not a wrapper write either: `rk-cli` makes it, and it is ADR-0107 decision
2's supersession, not clause 5's (§8d-ii.5).

Every git invocation among them runs through `GitOps`. `s8e`'s N7 `git_op` row journals them once it exists, and on
replay none of them runs.

**The worst-case path (§8d-ii.0 item 2).** The record states the bound for an item: `len(root) + 20 + len(item) + 11 +
1 +` the longest path tracked at the fork commit, where the root is the main worktree's directory. It measured
**255** of **259** at `c5d19d7` for a 64-character item, and **215** for `rk-fix-doctor-test-count`. The rule:
creation refuses by name when that bound, **computed on the item being created**, exceeds 259 and `core.longpaths` is
not `true`, rather than check out a partial tree (`worktree::creation_refuses_a_path_past_the_platform_limit_by_name`).
The draft computed it on a 64-character item whatever the item was. That left a 23-character root budget, which
Checkpoint 7f's clone could never meet (the adversarial review, D21-47). `worktree::the_length_rule_reads_the_actual_item_length`
is the control: a short item under a root too long for a 64-character one is admitted.

### 8d-ii.7 Checkpoint 7f's run arm and cleanup lines (moved from `s8e`)

**Why these amendments are this stage's.** As `s8a` leaves it, Checkpoint 7f runs `./target/debug/redkiln run
rk-fix-doctor-test-count --lane fix` in **this** repository. It ends with `git worktree remove --force
.git/redkiln/worktrees/rk-fix-doctor-test-count >/dev/null 2>&1 || true` and `git worktree prune`. Under this stage's
binary, that run either refuses at W2's re-entry check (if the legacy registration still holds the branch) or creates
`.redkiln/worktrees/rk-fix-doctor-test-count` on the existing branch. The cleanup line then removes nothing, because
it names the old root and swallows its own failure. W2 never removes the new tree. So a second, stale full checkout
stays registered inside the product repository, which is the hazard 7f's own comment describes: *"it makes every
later repo-root grep double-count against a stale tree"*. Three dated amendments, in the commit that moves the root:
1. **The run arm moves to a throwaway clone outside this repository, on a short path.** The amended arm:
   - makes `c7f=$(mktemp -d)` and converts it with `cygpath -m`, so the Windows binary and Python read the same path;
   - sets `clone="$c7f/c"` and clones `HEAD` into `"$clone"` with `git clone --quiet --no-local`;
   - runs `init` there, then `run rk-fix-doctor-test-count --lane fix` with the clone as its working directory, the
     binary named by its absolute path;
   - reads **that clone's** `.git/redkiln/runs`, by a path that names `$clone` (or `c7f`);
   - ends with `rm -rf "$c7f"`.

   The clone is outside this repository because under it the length rule would refuse. The adversarial review
   (D21-47) computed a 23-character root budget for a 64-character item, and a clone under `.git/cp8dii-scratch/` is
   far past that. On the actual item, 24 characters, the budget is 63. This box's `mktemp -d` path plus `/c` is 50
   characters (`C:/Users/ryanm/AppData/Local/Temp/tmp.AnSOKq8tPg/c` [measured at `fc80687`]), so the bound is 246.
   Checkpoint 8d-ii's (0b) recomputes it on every run. `init` writes the `# redkiln-runtime` block into the clone's
   `info/exclude`, and the clone's tracked `.gitignore` carries `.redkiln/worktrees/` too, so creation passes the
   ignore check either way. ADR-0164's `* text=auto eol=lf` row makes a fresh checkout test the same bytes
   (`992d257`: *"a new checkout under autocrlf=true tests the same bytes"*). The clone's `rk-run/` refs, worktrees and
   store are its own, so nothing in this repository moves. Its registrations live in the clone's own `.git`, and
   `rm -rf` takes them with it.
2. **The cleanup line is deleted.** It force-removes the legacy tree D21-44 keeps for `s8j` (*"stays where it is"*),
   and W2 forbids a forced removal.
3. **The `git worktree prune` line is deleted too.** It runs in this repository, and a prune removes the registration
   of any tree whose directory is missing. If the legacy tree's directory ever went, that would be the registration
   D21-44 keeps for `s8j`. The adversarial review (D21-47) found that the draft's guard let this line survive.

`s8e`'s third 7f edit, the literal that expects the discharged `Slice-Verdict` lint, stays `s8e`'s (`s8e` §8e.13),
because the lint is discharged there. `s8j`'s amendment of the same run arm (the fixture child and the launch record)
is `s8j`'s (`s8j` §8j.8), and it builds on this clone, which it takes as `$clone` (final consistency pass
2026-09-24). 7f's row in `PROGRESS.md` §1 becomes `passed …, re-run pending`
in this stage's commit, and Checkpoint 8d-ii re-runs 7f whole.

**How Checkpoint 8d-ii judges the replay.** The text guards read 7f's **code** lines only, with comments stripped, so
a comment naming a clone satisfies nothing. They require the `git clone … --no-local` and the `mktemp -d`. They refuse
any `worktree remove` or `worktree prune`, in any spelling (`-f` and `--force` alike), and any read of
`.git/redkiln/runs` that does not name the clone. Then the replay is judged by behaviour, as `s8a` guard (11) judges its
whole runs: this repository's worktree path set and run-record count are unchanged across it, and so are the
`dispatch_completed` rows in its store and journals, `.redkiln/worktrees/` and the fixture item's ref (Checkpoint
8d-ii (7b) and (8)).

**The arm still cannot dispatch here.** 0 shipped cards carry `permission_mode`, and `card::lint_card` refuses a blank
one at bind. Checkpoint 8d-ii asserts that before it re-runs 7f, and refuses to re-run 7f once a card carries one.

### 8d-ii.8 Named tests, red first, and the mutants

**Every test of a NEW behaviour is written before the code it covers and watched RED for the reason in its row.** A
test first seen green is not evidence (PIA-035). **Seven rows are pins over behaviour that no code violates today:**
- `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree`;
- `worktree::the_length_rule_reads_the_actual_item_length`;
- `worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently`;
- `worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse`;
- `worktree::a_repo_with_no_filter_or_include_if_creates`;
- `worktree::a_pointer_header_at_a_path_no_filter_selects_does_not_refuse` (D21-48 X2);
- `worktree::a_committed_pointer_that_the_driver_smudges_creates_silently` (D21-48 X2).

They are green at entry by design. Each is shown able to fail by the mutant named for it: `S8D-II-M9` for the planted
tree, `S8D-II-M18` for the length rule, `S8D-II-M12` and `S8D-II-M13` for the BrittonGames shape, `S8D-II-M13` for the
declared-but-unselected driver, `S8D-II-M14` for the empty case, `S8D-II-M23` for the unselected pointer header,
and `S8D-II-M24` for the smudged pointer. Every filter test isolates the machine's own
configuration (`GIT_CONFIG_SYSTEM` and `GIT_CONFIG_GLOBAL` pointed at fixture files), so no test
reads this box's `system`-scope `filter.lfs` and no result differs per machine.
**Two further rows are existing tests, re-selected as witnesses** that the fixture repositories' ignore rows move no
exit: `run::ac438_table_driven_exit_codes_over_fixtures_binary_arm` and
`cost::cost_after_a_real_run_renders_low_sample_through_the_binary`.

Each test is selected in Checkpoint 8d-ii by its module-qualified name with `--exact`. A bare function name under
`--exact` runs zero tests and exits 0.

| crate | test | red at entry because |
| --- | --- | --- |
| `rk-git` | `worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused` (rows include `a..b`, a trailing `.`, `CON`, `nul.txt`, `com1` and a `.Review-Ref` suffix) | no validator |
| `rk-git` | `worktree::an_item_id_that_case_folds_onto_a_registered_tree_is_refused` | no validator |
| `rk-git` | `worktree::the_root_is_the_main_worktree_when_run_from_a_linked_worktree` | the root is derived from the common dir |
| `rk-git` | `worktree::a_bare_repository_is_refused_by_name` | no root rule |
| `rk-git` | `worktree::a_symlinked_redkiln_or_worktrees_directory_is_refused` (a junction on Windows, a symlink elsewhere; it says which) | no symlink guard |
| `rk-git` | `worktree::a_created_worktree_whose_realpath_leaves_the_root_is_refused` | no post-create check |
| `rk-git` | `worktree::creation_refuses_when_the_worktree_root_is_not_ignored` | no ignore check |
| `rk-git` | `worktree::creation_refuses_a_path_past_the_platform_limit_by_name` | no length rule |
| `rk-git` | `worktree::the_length_rule_reads_the_actual_item_length` | green — a control; `S8D-II-M18` shows it can fail |
| `rk-git` | `worktree::creation_writes_rk_base_into_the_worktree_gitdir` | no marker |
| `rk-git` | `worktree::creation_locks_the_worktree_with_the_pid_and_start_reason` | no lock |
| `rk-git` | `worktree::a_lock_held_by_a_dead_pid_is_released` | no lock |
| `rk-git` | `worktree::a_lock_whose_pid_now_names_another_process_is_released` | no lock |
| `rk-git` | `worktree::a_lock_held_by_a_live_pid_refuses_by_name` | no lock |
| `rk-git` | `worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip` | re-created by force |
| `rk-git` | `worktree::reentry_discards_in_flight_changes_and_reports_each_path` | removed silently |
| `rk-git` | `worktree::reentry_keeps_ignored_files` | the tree is deleted |
| `rk-git` | `worktree::a_tree_without_rk_base_is_refused_and_never_removed` (renamed from `an_existing_worktree_path_is_refused_not_reused`, whose refusal is kept) | no marker check |
| `rk-git` | `worktree::a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed` | force-removed |
| `rk-git` | `worktree::a_tree_registered_to_another_repository_is_refused` | no re-entry check |
| `rk-git` | `worktree::a_tree_on_a_detached_head_is_refused_at_reentry_and_kept` | no re-entry check |
| `rk-git` | `worktree::an_existing_run_branch_is_never_reset` | no reuse path to observe it |
| `rk-git` | `worktree::a_dirty_tree_is_kept_with_its_reason` | no removal rule |
| `rk-git` | `worktree::an_unlanded_tree_is_kept_with_its_reason` | no removal rule |
| `rk-git` | `worktree::a_locked_tree_is_not_removed` | no lock |
| `rk-git` | `worktree::a_clean_landed_marked_unlocked_tree_is_removed` | no removal rule |
| `rk-git` | `worktree::removal_never_follows_a_junction` | no guarded removal |
| `rk-git` | `worktree::a_selected_filter_with_no_driver_at_any_scope_refuses_naming_the_path` | no config read |
| `rk-git` | `worktree::a_smudge_skipped_at_local_scope_refuses_naming_the_scope` | no config read |
| `rk-git` | `worktree::a_smudge_skipped_at_global_scope_refuses_naming_the_scope` | no config read |
| `rk-git` | `worktree::a_smudge_skipped_at_system_scope_refuses_naming_the_scope` | no config read; a `--local` read would also pass it |
| `rk-git` | `worktree::git_lfs_skip_smudge_in_the_environment_refuses_by_name` | no environment read |
| `rk-git` | `worktree::an_include_if_that_skips_the_smudge_for_the_run_tree_refuses_naming_the_included_file` | no config read |
| `rk-git` | `worktree::the_filter_attribute_is_read_from_the_fork_commit` | no attribute read |
| `rk-git` | `worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently` | green — a control; BrittonGames' shape (V1) |
| `rk-git` | `worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse` | green — a control; this box's own shape |
| `rk-git` | `worktree::a_repo_with_no_filter_or_include_if_creates` | green — the control; no refusal to observe |
| `rk-git` | `worktree::a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree` (D21-48 X2; finding 15's shape) | no post-checkout scan: the configuration read admits the driver, and the tree keeps its pointer files |
| `rk-git` | `worktree::the_pointer_scan_never_removes_a_tree_this_call_did_not_create` (D21-48 X2) | re-created by force; no reuse to observe |
| `rk-git` | `worktree::a_pointer_header_at_a_path_no_filter_selects_does_not_refuse` (D21-48 X2) | green — a control; `S8D-II-M23` shows it can fail |
| `rk-git` | `worktree::a_committed_pointer_that_the_driver_smudges_creates_silently` (D21-48 X2) | green — a control; `S8D-II-M24` shows it can fail |
| `rk-git` | `gitops::every_git_child_runs_with_lc_all_c` | no `LC_ALL` |
| `rk-containment` | `containment::process_start_time_reads_a_live_child_and_nothing_for_an_exited_one` | no start-time read |
| `rk-cli` | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` | the common-dir root |
| `rk-cli` | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` | green — the pin; `S8D-II-M9` shows it can fail |
| `rk-cli` | `run_worktree::creation_carries_worktreeinclude_files_and_runs_setup` | no carry |
| `rk-cli` | `run_worktree::a_failed_setup_refuses_by_name` | no `setup` call |
| `rk-cli` | `run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves` | the reference tree is created at `HEAD` |
| `rk-cli` | `run_worktree::a_leftover_reference_tree_without_rk_base_is_refused_and_kept` | a leftover is force-removed |
| `rk-cli` | `init::init_writes_the_redkiln_runtime_exclude_block_once` (from a repository in the pre-stage initialised shape) | `init` writes no exclude |
| `rk-cli` | `init::init_leaves_the_operators_exclude_lines_untouched` | `init` writes no exclude |
| `rk-cli` | `run::ac438_table_driven_exit_codes_over_fixtures_binary_arm` | green — an existing witness; RED if a fixture repository lacks the ignore row |
| `rk-cli` | `cost::cost_after_a_real_run_renders_low_sample_through_the_binary` | green — an existing witness; RED if `cost`'s run repository lacks the ignore row |

**The count.** 53 names: `rk-git` 42, `rk-containment` 1, `rk-cli` 10. Two are existing witnesses and one is a
rename, so **50 are new**, and the suite gains **50** passed (§8d-ii.0 item 8). *(It was 35 names and 32 new before
the adversarial-review fixes of 2026-09-24. The fixes added fourteen rows and replaced four filter rows with eight.)*
*(D21-48 (2026-09-24): it was 49 names and 46 new before D21-48 X2 added four `rk-git` rows.)*

`rk-git`'s `worktree::control_a_worktree_can_be_created_and_removed` is **re-pointed**, not deleted, because its
removal now needs a clean, landed, marked, unlocked tree. The six `rk-cli` test sites that derive the old root
(§8d-ii.1) are re-pointed through the one derivation.

**The mutants.** Each mutant is applied and restored **from Python, never the shell**: a backtick once swallowed a
restore, and the next backup captured the mutant. Each diff is read. Each must turn the named test RED **for the
named reason**, printed and read, not merely "the suite went red". The names carry an `S8D-II-` prefix. M1 to M11
were `s8e`'s S8E-M19 and S8E-M21 to S8E-M24, S8E-M30 to S8E-M33, and the rk-git half of S8E-M20. `s8e` keeps the
`lane_e2e` half of S8E-M20. M12 to M21 are the adversarial review's (D21-47), and M22 to M26 are D21-48 X2's.

| mutant | change | must redden | was |
| --- | --- | --- | --- |
| S8D-II-M1 | the run-tree root derived under the common dir again | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` | S8E-M19 |
| S8D-II-M2 | re-entry force-removes and re-creates (`recreate_run_worktree` restored) | `worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip` | S8E-M20 (rk-git half) |
| S8D-II-M3 | a tree without `RK_BASE` is removed rather than refused | `worktree::a_tree_without_rk_base_is_refused_and_never_removed` | S8E-M21 |
| S8D-II-M4 | the discard returns no paths | `worktree::reentry_discards_in_flight_changes_and_reports_each_path` | S8E-M22 |
| S8D-II-M5 | the filter read narrowed to `--local` | `worktree::a_smudge_skipped_at_system_scope_refuses_naming_the_scope`, `..._at_global_scope_...`, and `worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently` (the `system` driver is no longer seen, so the path reads as selecting no effective driver) | S8E-M23, widened by R8 |
| S8D-II-M6 | `init` appends the block on every run | `init::init_writes_the_redkiln_runtime_exclude_block_once` | S8E-M24 |
| S8D-II-M7 | `LC_ALL` not set on the child | `gitops::every_git_child_runs_with_lc_all_c` | S8E-M30 |
| S8D-II-M8 | a dead pid's lock is never released | `worktree::a_lock_held_by_a_dead_pid_is_released` | S8E-M31 |
| S8D-II-M9 | `doctor`'s card reader made recursive | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` | S8E-M32 |
| S8D-II-M10 | `clean -fdx` in the discard | `worktree::reentry_keeps_ignored_files` | S8E-M33 |
| S8D-II-M11 | the ignore check skipped | `worktree::creation_refuses_when_the_worktree_root_is_not_ignored` | new |
| S8D-II-M12 | refuse whenever a tracked path selects a filter, effective driver or not (the inverted reading V1 corrected) | `worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently` | new (V1) |
| S8D-II-M13 | refuse on any declared driver, selected or not (R8's literal reading) | `worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse`, `worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently` | new (R8) |
| S8D-II-M14 | refuse when the read finds no driver and no `includeIf` (the empty case inverted) | `worktree::a_repo_with_no_filter_or_include_if_creates` | new |
| S8D-II-M15 | attributes read from the working tree, not `--source=<fork>` | `worktree::the_filter_attribute_is_read_from_the_fork_commit` | new (V1) |
| S8D-II-M16 | "landed" read as `HEAD` equal to the run-ref tip | `worktree::an_unlanded_tree_is_kept_with_its_reason` | new |
| S8D-II-M17 | the root derived from `--show-toplevel` | `worktree::the_root_is_the_main_worktree_when_run_from_a_linked_worktree` | new |
| S8D-II-M18 | the length rule computed on a 64-character item | `worktree::the_length_rule_reads_the_actual_item_length` | new |
| S8D-II-M19 | the reference tree created at `HEAD` | `run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves` | new |
| S8D-II-M20 | the lock's start time ignored (pid alone) | `worktree::a_lock_whose_pid_now_names_another_process_is_released` | new |
| S8D-II-M21 | `GIT_LFS_SKIP_SMUDGE` not read | `worktree::git_lfs_skip_smudge_in_the_environment_refuses_by_name` | new |
| S8D-II-M22 | the post-checkout scan skipped | `worktree::a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree` | new (D21-48 X2) |
| S8D-II-M23 | the scan reads every tracked path, not only the paths a filter selects at the fork commit | `worktree::a_pointer_header_at_a_path_no_filter_selects_does_not_refuse` | new (D21-48 X2) |
| S8D-II-M24 | the scan reads the blob at the fork commit (a prediction) rather than the checked-out file | `worktree::a_committed_pointer_that_the_driver_smudges_creates_silently` | new (D21-48 X2) |
| S8D-II-M25 | the scan also runs at re-entry, and its rollback removes the reused tree | `worktree::the_pointer_scan_never_removes_a_tree_this_call_did_not_create` | new (D21-48 X2) |
| S8D-II-M26 | the refusal returns without removing the fresh tree | `worktree::a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree` (its absence assertion) | new (D21-48 X2) |

A mutant that survives is not waved through. It is either an **unobservable property**, which moves to a source lint
with its reason written down, or a **missing fixture branch**. Both are recorded by the mutant's name.

### 8d-ii.9 What this file found

Findings carried from `s8e`'s revision (its former findings 10–13 and 15–17), with their disposition now, then this
split's own:

1. **D21-45's premise that `init` already writes the hooks marker (RT-51) is false.** *Settled by D21-46's second
   correction.* The record says the exclude block is `init`'s first write to a file git reads (§8d-ii.5). D21-46's
   *"first write into a target repo's git admin area"* is refined there: the store already lives under the common
   dir (the adversarial review, D21-47).
2. **D21-44's supersession targets are not where it says.** *Settled by D21-46's first correction.* The records table
   carries ADR-0137 decision 3 and ADR-0105 clauses 5 and 6, each verified by quotation, and ADR-0018 is cited only.
3. **W1, W2 and W4 need `GitOp` variants outside ADR-0105 clause 5's list.** *Made explicit* (§8d-ii.6). R8 adds two
   reads, `config --show-scope` and `check-attr`. *Corrected by D21-47's clause-5 rule:* only the writes supersede
   clause 5 in part (lock, unlock, the marker, reset and clean). The reads get variants and supersede nothing.
4. **Checkpoint 7f's cleanup line removes the legacy tree before `s8j` can re-point it.** `s8a` runs 7f whole at its
   exit, before this stage, and 7f's last lines remove the registration. This stage deletes the line (§8d-ii.7), but
   `s8a`'s run comes first. *Settled by `s8a` (consistency pass 2026-09-24):* `s8a` guard (11) records the
   registration before its whole runs and restores it afterwards, asserting the ref unmoved (`s8a` finding (8)). From
   this stage on the amended 7f never touches the legacy tree. §8d-ii.0 item 1 still records what it finds.
5. **AC-518's instrument now depends on an ignore row.** *Flagged by D21-46*, stated as a refusal in §8d-ii.1, and
   made a stage-open question by D21-47. *Settled by D21-48 X1 (2026-09-24):* *"**Refuse by name.** This answers D21-46's flip-line flag."*
6. **The review reference tree shares the item namespace.** `<item>.review-ref` sits beside `<item>`, so the validator
   refuses an item id with that suffix (§8d-ii.2).
7. **The worst-case path is 255 of 259 at `c5d19d7` for a 64-character item, and it was 259 under the old root.** The
   margin is four characters on this repository, and a target repository with a longer root or deeper paths has
   less. *Refined by the adversarial review (D21-47):* the rule reads the actual item, so the margin for a 24-character
   item is 44.
8. **R8, read literally, refuses every run on this box** (§8d-ii.0 item 3). Git for Windows declares `filter.lfs.*`
   at `system` scope for every repository. *Settled by D21-47 V1*, and not the way the draft read it. The draft
   refused a declared driver when a tracked path selected it, which is the working case. V1 refuses only when a
   selected driver is **not** effective, or its smudge is skipped (§8d-ii.5).
9. **`s8e`'s quotation of ADR-0107 decision 2 did not match the record.** `s8e`'s former §8e.11 quoted *"A second
   `init` leaves this file untouched"*. ADR-0107 decision 2 reads *"**`init` is idempotent**: a second `init` on an
   initialised repository changes nothing and exits 0"*. Its quotation of AC-531 (*"separately and unlinted"*) is also
   about the store path, not the common dir as a whole. §8d-ii.5 quotes both as they stand.
10. **The discard is journalled one stage late.** W2's *"discarded explicitly and journalled"* has no journal row until
    `s8e` mints `worktree_assigned`. This file states the interval and reports on stderr in the meantime (§8d-ii.3).
11. **Files this split leaves stale, which this stage does not edit** (D21-46 R1's integration is the lead's). *The
    three `s8` sibling edits below were made in the consistency pass 2026-09-24; the `INDEX.md` and `PROGRESS.md` rows
    remain the lead's:*
    - `INDEX.md`'s stage table and `PROGRESS.md`'s tracker gain the `s8d-ii` row;
    - `s8d`'s header `next` becomes `s8d-ii`;
    - `s8c`'s ownership row and its `setup` row named `s8e` as the caller at creation, and its "from `s8e` on" named
      the root move's stage. Both now read `s8d-ii`;
    - `s8j` cites `s8e` §8e.10 for the `.redkiln/` walk exclusion and `s8e` §8e.15 for the 7f amendment. The first is
      now §8d-ii.1, and the second is §8d-ii.7 here plus `s8e` §8e.13.

The adversarial review of 2026-09-24 (D21-47) found the following:

12. **The filter refusal was inverted.** It refused the working case: a selected driver effective at `system` scope,
    which is BrittonGames' shape. That contradicted `s8i` §8i.0 item 12. *Settled by D21-47 V1* (§8d-ii.5).
13. **The review reference tree is not a run tree.** Extending W2's rule to it moved ADR-0162 decision 10 with no
    supersession. Reuse also made false its doc's premise that the run branch forks at the current `HEAD`. *Fixed
    here:* decision 10 stands, and the tree is created at `RK_BASE`'s fork sha (§8d-ii.3).
14. **"Landed" had no content.** As *"`HEAD` equals the run-ref tip"* it repeated re-entry's own condition. Phase 1
    has no integration ref, so landed is read against the lane's `closed` mark and a symbolic `HEAD` (§8d-ii.3). When
    the merge queue arrives, the record that brings it widens the definition.
15. **One pointer-file case passes the refusal.** An effective driver whose command is not installed, and that is not
    `required = true`, yields pointers, and a configuration read cannot see that. V1 admits *"an installed, working
    driver"*. The refusal checks that the driver is effective, not that it is installed. *Settled by D21-48 X2
    (2026-09-24):* *"**A post-checkout scan.** After creation, the paths the filter selects are scanned for LFS
    pointer headers; any hit refuses by name and removes the fresh worktree."* Every selected path is scanned, not
    one (§8d-ii.5).
16. **D21-47's fake `claude` shadows a shell-resolved `claude` only.** On Windows, `std::process::Command::new("claude")`
    resolves `claude.exe` through `PATH` and passes a script named `claude` by. The real CLI on this box is
    `C:/Users/ryanm/.local/bin/claude.exe` [measured at `fc80687`]. So a native spawn from a test or a replayed binary
    would reach the real CLI past the fake. What catches that case is the trap `CLAUDE_CONFIG_DIR` and the
    `dispatch_completed` counts, which Checkpoint 8d-ii both asserts. The trap has its own stated limit: a child
    launched with `CLAUDE_CONFIG_DIR` set explicitly escapes it. The lead's call is whether the wrapper should also
    place a `claude.exe` fake. *Settled in the final consistency pass 2026-09-24:* the shared wrapper,
    `runbook/tools/nodispatch.sh`, compiles its fake from `runbook/tools/fake-claude.rs` to `claude.exe` and removes
    every other `PATH` directory holding a `claude`, `claude.exe` or `claude.cmd`. A native spawn therefore finds only
    the fake, which logs the call and exits 97. Checkpoint 8d-ii sources it.

### 8d-ii.10 Acceptance set

**Claimed: none as met at this stage.** This stage builds the placement and lifecycle that AC-518 and AC-529 run over,
and adds the dated notes. AC-518's instrument is a lane run (`s8e`'s `lane_e2e`), and AC-529's is `s8e`'s
`worktree_assigned` suite. Both are claimed at `s8e` (`s8e` §8e.14).

**Contributed to, and named so that no document claims them early:**
- **AC-518**: the ignore rows and the refusal that make its `git status --porcelain` hold with the tree inside the
  checkout.
- **AC-529**: the one derivation, which `s8e` §8e.4's row replaces as the source of ownership.
- **AC-531**: unchanged by the `info/exclude` write, with a dated note saying why (§8d-ii.5).
- **AC-522**: unchanged. `redkiln-migrate`'s worktree keeps ADR-0105 clause 6, with a dated note saying so (§8d-ii.1).

**Excluded:** AC-412 (`init` still installs no hook) and every tier-1 criterion (`s8e`'s).

### 8d-ii.11 Checkpoint 8d-ii

- **The run tree lives at `.redkiln/worktrees/<item>` under the main worktree, derived in one place.** No code line
  under `crates/*/src` or `crates/*/tests` outside `rk-git`'s worktree module and its own tests derives a `worktrees`
  directory in any spelling, apart from reads of git's own admin directory. *(D21-44; the adversarial review, D21-47)*
- **The root is ignored here, creation refuses when it is not, and the worst-case path is within the limit**, for
  this repository and for Checkpoint 7f's clone, computed on the actual item. *(D21-44; D21-46's flag, taken by D21-48 X1)*
- **W1's guards hold, each by a named test.** *(W1)*
- **Reuse, never force-remove.** `recreate_run_worktree` is gone. The only forced removal left in `run.rs` is ADR-0162
  decision 10's reference-tree removal, inside `remove_reference_worktree`, and `rk-git`'s worktree module forces
  nothing. *(W2; ADR-0162 decision 10)*
- **The filter check refuses only when pointer files would result, reading attributes from the fork commit, and a
  post-checkout scan refuses by name and removes the fresh tree when a selected path holds a pointer anyway** ((0d),
  (2), (3)). `init` writes its block once. *(W4; R8; D21-47 V1; D21-48 X2)*
- **The fixture repositories' exits did not move.** *(§8d-ii.1)*
- **The whole suite, fmt and clippy are green.**
- **The records are written**: the worktree record, located by the id it took at writing (`REC_WT`), with its
  supersessions and quotations; the back-links in the state its status requires; the register notes on AC-518,
  AC-522, AC-529 and AC-531; design §5's and §6's row notes; the `s7d` notes; the fixture doc; the index row; and the
  ledger entry. *(the adversarial review's blocker, D21-47)*
- **Checkpoint 8d (the entry gate) re-runs whole, and so does 7f, the one checkpoint this stage's edits make
  provisional (§8d-ii.7).** Nothing older is re-run. 7h and 7i are not amended here, and their witnesses are the two
  existing tests selected in (2). The block skips both nested re-runs when it is itself run with `RK_CP_NESTED=1`, the
  D21-46 convention: *"running as another checkpoint's entry gate: skip your own nested whole re-runs"*. It sets the
  same variable for the checkpoints it calls. `s8d`'s block honours the variable (consistency pass 2026-09-24), so it
  skips its own re-runs of 8c and 7g when this block replays it. The card precondition sits inside the non-nested
  branch (D21-47).
- **No process this block starts can reach a real `claude` unobserved (D21-47).** The block sources the shared
  wrapper, `runbook/tools/nodispatch.sh`, and every cargo run and every replay runs under `nodispatch`: a compiled
  fake `claude` first on `PATH` that prints the pin for `--version` and logs and refuses anything else, every other
  `PATH` directory holding a `claude` removed, `REDKILN_LAUNCH_RECORD` unset, and `CLAUDE_CONFIG_DIR` at an empty trap
  directory. `nd_check` passes after the cargo runs and after each replay: the trap is still empty and the fake logged
  no call. `dispatch_completed` rows in this repository's store and journals do not move. *(final consistency pass
  2026-09-24)*
- **`rk-run/rk-fix-doctor-test-count` and its legacy mark are unchanged**, and so are `.redkiln/worktrees/`, the
  worktree path set and this repository's run-record count, so no lane ran against the product repository.

**BEFORE (PIA-035): on the entry tree these lines FAIL:**
- (2)'s selections, because the new tests do not exist;
- (3)'s root floor, because of `run.rs`'s four derivations and the six test sites;
- (3)'s `recreate_run_worktree` and force-removal floors, because `recreate_run_worktree` carries a `"--force"` and a
  `remove_dir_all` outside `remove_reference_worktree`;
- (3)'s `LC_ALL`, `RK_BASE`, `--show-scope`, `--source`, LFS pointer-header (D21-48 X2) and `# redkiln-runtime`
  floors, because none exists;
- (7b)'s 7f guards, because 7f still runs in this repository and still carries the cleanup and prune lines;
- the block's `REC_WT` line and (9)'s records, because `REC_WT` is still the placeholder `____`.

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
# THE RECORD ID, a LITERAL (D21-47: records are located by the id recorded at write time, never by a title phrase or a
# supersession count). The commit that writes the worktree record writes its four-digit id into the line below by a
# dated edit. Re-runners supply nothing; while the placeholder remains, the block refuses.
REC_WT=____
case "$REC_WT" in
  [0-9][0-9][0-9][0-9]) ;;
  *) echo "FAIL: REC_WT is '$REC_WT', not a four-digit record id — write the id the worktree record took into this block by a dated edit"; exit 1 ;;
esac

# ---- (00) THE NO-DISPATCH WRAPPER (D21-47), before any cargo run: the shared helper, runbook/tools/nodispatch.sh.
# ---- nd_setup compiles a fake claude.exe first on PATH (it prints the pin for --version, and logs and exits 97 on
# ---- anything else), removes every other PATH directory holding a claude, unsets REDKILN_LAUNCH_RECORD, points
# ---- CLAUDE_CONFIG_DIR at an empty trap, and runs its own controls. Every cargo run below and every replay in (7)
# ---- runs under `nodispatch`, and nd_check (the trap still empty, no fake call logged) follows each. Inside an
# ---- enclosing wrapper (a replay of this block) nd_setup verifies and reuses that one.
. runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh could not be sourced"; exit 1; }
nd_setup "$PWD/target/s8d-ii/nd" || exit 1

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
common=$(git rev-parse --path-format=absolute --git-common-dir); rc=$?
[ "$rc" -eq 0 ] && [ -n "$common" ] || { echo "FAIL: cannot resolve the git common dir (rc=$rc)"; exit 1; }
store="$common/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store — the dispatch count would read nothing"; exit 1; }
runs_before=$(find "$common/redkiln/runs" -mindepth 1 -maxdepth 1 2>/dev/null | grep -c .)
# NO-DISPATCH, MEASURED: dispatch_completed rows in the store AND in every run journal (s8a guard (11)'s form).
cat > "$scratch/dcount.py" <<'PY'
import glob, os, sqlite3, sys
db, runs = sys.argv[1], sys.argv[2]
c = sqlite3.connect("file:" + db.replace(os.sep, "/") + "?mode=ro", uri=True)
s = c.execute("select count(*) from t2_event where kind = 'dispatch_completed'").fetchone()[0]
j = 0
for f in glob.glob(os.path.join(runs, "*", "events.jsonl")):
    j += sum(1 for line in open(f, encoding="utf-8") if '"kind":"dispatch_completed"' in line)
print(s, j)
PY
d_before=$(python "$scratch/dcount.py" "$store" "$common/redkiln/runs"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$d_before" ] || { echo "FAIL: cannot count dispatch_completed before this block's runs (rc=$rc)"; exit 1; }
# CONTROL: the counter reads one seeded row in a control store and one in a control journal.
mkdir -p "$scratch/dctl/runs/r1" || { echo "FAIL: cannot make the dispatch-count control"; exit 1; }
cat > "$scratch/dseed.py" <<'PY'
import sqlite3, sys
c = sqlite3.connect(sys.argv[1])
c.execute("create table t2_event (id integer primary key, kind text, payload text)")
c.execute("insert into t2_event (kind, payload) values ('dispatch_completed', '{}')")
c.commit()
PY
python "$scratch/dseed.py" "$scratch/dctl/store.sqlite3" || { echo "FAIL: cannot seed the dispatch-count control store"; exit 1; }
printf '{"kind":"dispatch_completed","payload":{}}\n' > "$scratch/dctl/runs/r1/events.jsonl" || { echo "FAIL: cannot seed the control journal"; exit 1; }
[ "$(python "$scratch/dcount.py" "$scratch/dctl/store.sqlite3" "$scratch/dctl/runs")" = "1 1" ] || { echo "FAIL: CONTROL — the dispatch counter did not read 1 1, so the no-dispatch guard is blind"; exit 1; }
# The whole suite's node fixture (s8c's precondition), and a WARM pnpm store for this stage's setup test, which runs
# `pnpm install --frozen-lockfile --prefer-offline` in a fresh run tree. node_modules alone is a proxy (D21-47): an
# --offline install of the same lockfile into a scratch copy must succeed.
nf=crates/rk-adapter-toolchain-node/tests/fixtures/project
test -d "$nf/node_modules" || { echo "FAIL: the node fixture has no node_modules — run pnpm install --frozen-lockfile --prefer-offline --dir $nf first"; exit 1; }
mkdir -p "$scratch/pnpm-warm" && cp "$nf/package.json" "$nf/pnpm-lock.yaml" "$scratch/pnpm-warm/" || { echo "FAIL: the warm-store probe could not be staged"; exit 1; }
out=$(pnpm install --frozen-lockfile --offline --dir "$scratch/pnpm-warm" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an --offline install of the node fixture's lockfile failed (rc=$rc) — the pnpm store is cold; run the install above once with network"; printf '%s\n' "$out" | tail -5; exit 1; }
rm -rf "$scratch/pnpm-warm" || { echo "FAIL: the warm-store probe could not be removed"; exit 1; }

# ---- (0a) THE ROOT IS THE MAIN WORKTREE (section 8d-ii.1), AND IT IS IGNORED HERE (D21-44), or every run tree reads as
# ---- untracked (AC-518). git lists the main worktree first; a first record carrying `bare` has no main worktree.
wtl=$(git worktree list --porcelain); rc=$?
[ "$rc" -eq 0 ] && [ -n "$wtl" ] || { echo "FAIL: git worktree list --porcelain failed (rc=$rc)"; exit 1; }
main=$(printf '%s\n' "$wtl" | sed -n '1s/^worktree //p')
test -n "$main" || { echo "FAIL: the first worktree record names no path"; exit 1; }
[ "$(printf '%s\n' "$wtl" | sed -n '2p')" != "bare" ] || { echo "FAIL: this repository is bare — it has no main worktree to root run trees in"; exit 1; }
top=$(git rev-parse --show-toplevel); rc=$?
[ "$rc" -eq 0 ] && [ "$top" = "$main" ] || { echo "FAIL: this block runs in '$top', not the main worktree '$main' (rc=$rc)"; exit 1; }
# CONTROL: the reading takes the FIRST record when a linked worktree follows it.
ctl=$(printf '%s\n' 'worktree D:/m' 'HEAD 0' 'branch refs/heads/x' '' 'worktree D:/m.swt' | sed -n '1s/^worktree //p')
[ "$ctl" = "D:/m" ] || { echo "FAIL: CONTROL — the main-worktree reading took '$ctl'"; exit 1; }
git -C "$main" check-ignore -q .redkiln/worktrees/cp8dii-probe; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: .redkiln/worktrees/ is not ignored in the main worktree (check-ignore rc=$rc) — D21-44's .gitignore row is missing"; exit 1; }
# CONTROL: a tracked file is NOT ignored, so the line above can go red.
git -C "$main" check-ignore -q Cargo.toml; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: CONTROL — check-ignore called a tracked file ignored (rc=$rc), so the ignore guard cannot fail"; exit 1; }

# ---- (0b) THE WORST-CASE RUN-TREE PATH (section 8d-ii.0 item 2), computed on an item's ACTUAL length: root +
# ---- /.redkiln/worktrees/ + the item + .review-ref + / + the longest tracked path, against the 259 MAX_PATH leaves.
# ---- Two cases: the longest item the validator admits (64) under this repository, and Checkpoint 7f's fixture item
# ---- under its throwaway clone's root (section 8d-ii.7), which is `mktemp -d` in drive-letter form plus /c.
files=$(git ls-files); rc=$?
[ "$rc" -eq 0 ] && [ -n "$files" ] || { echo "FAIL: git ls-files failed or listed nothing (rc=$rc)"; exit 1; }
longest=$(printf '%s\n' "$files" | awk '{ if (length($0) > m) m = length($0) } END { print m + 0 }')
[ "$longest" -gt 0 ] || { echo "FAIL: the longest tracked path read as $longest — the length guard would be about nothing"; exit 1; }
lp=$(git config --get core.longpaths); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: git config could not be read (rc=$rc)"; exit 1; }
prefix="$main/.redkiln/worktrees/"
worst=$(( ${#prefix} + 64 + 11 + 1 + longest ))
[ "$worst" -le 259 ] || [ "$lp" = "true" ] || { echo "FAIL: the worst-case run-tree path here is $worst characters (longest tracked path $longest) and core.longpaths is not true"; exit 1; }
item7f=rk-fix-doctor-test-count
t=$(mktemp -d); rc=$?
[ "$rc" -eq 0 ] && [ -d "$t" ] || { echo "FAIL: mktemp -d failed (rc=$rc)"; exit 1; }
croot="$(cygpath -m "$t")/c"
rmdir "$t" || { echo "FAIL: the mktemp probe directory could not be removed"; exit 1; }
cworst=$(( ${#croot} + 20 + ${#item7f} + 11 + 1 + longest ))
[ "$cworst" -le 259 ] || [ "$lp" = "true" ] || { echo "FAIL: Checkpoint 7f's clone root ($croot, ${#croot} characters) gives a $cworst-character worst case for $item7f — its run would refuse at creation"; exit 1; }
# CONTROL: the same sum over a 200-character path exceeds the limit, so the lines above can go red.
ctl=$(( ${#prefix} + 64 + 11 + 1 + 200 ))
[ "$ctl" -gt 259 ] || { echo "FAIL: CONTROL — a 200-character path summed to $ctl, within the limit, so the length guard cannot fail"; exit 1; }
echo "worst-case run-tree path: $worst here (64-character item), $cworst in 7f's clone ($item7f); longest tracked path $longest; limit 259 without core.longpaths"

# ---- (0c) V1's CONDITION ON THIS REPOSITORY (section 8d-ii.0 item 3; D21-47 V1): every filter a path selects AT HEAD
# ---- (attributes read from the commit, not the working tree) has an effective driver, and smudge is not skipped.
# ---- No pipe carries a status: the path list is captured, written to a file, and fed to check-attr on stdin.
lsf=$(git ls-tree -r --name-only HEAD); rc=$?
[ "$rc" -eq 0 ] && [ -n "$lsf" ] || { echo "FAIL: git ls-tree could not list HEAD (rc=$rc)"; exit 1; }
printf '%s\n' "$lsf" > "$scratch/paths.txt" || { echo "FAIL: the path list could not be written"; exit 1; }
attrs=$(git check-attr --source=HEAD --stdin filter < "$scratch/paths.txt"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$attrs" ] || { echo "FAIL: git check-attr --source=HEAD filter could not read the tracked paths (rc=$rc)"; exit 1; }
drivers=$(printf '%s\n' "$attrs" | grep -vE ': filter: (unspecified|unset)$' | sed -E 's/^.*: filter: //' | sort -u)
effective() {  # $1 = a driver name; rc 0 when a process or smudge value resolves at some scope and smudge is not skipped
  p=$(git config --includes --get "filter.$1.process"); prc=$?
  s=$(git config --includes --get "filter.$1.smudge"); src=$?
  [ "$prc" -eq 0 ] || [ "$src" -eq 0 ] || return 1
  case "$s" in *--skip*) return 1 ;; esac
  case "${GIT_LFS_SKIP_SMUDGE:-0}" in 0) ;; *) return 1 ;; esac
  return 0
}
bad=""
for d in $drivers; do effective "$d" || bad="$bad $d"; done
[ -z "$bad" ] || { echo "FAIL (STOP): a filter selected at HEAD has no effective driver here, or its smudge is skipped:$bad — a run tree of this repository would hold pointer files; take it to the operator"; exit 1; }
# CONTROLS: a driver no scope declares is not effective; one declared with a smudge at command scope is; a skipping
# smudge is not.
effective s8dii-no-such-driver && { echo "FAIL: CONTROL — an undeclared driver read as effective"; exit 1; }
GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=filter.s8dii-ctl.smudge GIT_CONFIG_VALUE_0=cat effective s8dii-ctl || { echo "FAIL: CONTROL — a driver with a smudge value read as not effective"; exit 1; }
GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=filter.s8dii-ctl.smudge GIT_CONFIG_VALUE_0='git-lfs smudge --skip -- %f' effective s8dii-ctl && { echo "FAIL: CONTROL — a skipping smudge read as effective"; exit 1; }
echo "filters selected at HEAD: $(printf '%s\n' "$drivers" | grep -c .) (every one effective)"

# ---- (0d) D21-48 X2's POST-CHECKOUT POINTER SCAN (section 8d-ii.5), observed on this checkout, with two synthetic
# ---- controls that prove the reader can go red. ptr_scan prints each path a filter selects AT HEAD whose CHECKED-OUT
# ---- bytes begin with the Git LFS pointer header. The rollback is the named tests' in (2). No pipe carries a status.
LFS_HDR='version https://git-lfs.github.com/spec/v1'
ptr_scan() {  # $1 = a checkout; stdout = the hits, one path per line; rc 2 when a read fails
  pl=$(git -C "$1" -c core.quotePath=false ls-tree -r --name-only HEAD) || return 2
  printf '%s\n' "$pl" > "$scratch/ps-paths.txt" || return 2
  pa=$(git -C "$1" -c core.quotePath=false check-attr --source=HEAD --stdin filter < "$scratch/ps-paths.txt") || return 2
  printf '%s\n' "$pa" | grep -vE ': filter: (unspecified|unset)$' | sed -E 's/: filter: [^:]*$//' | while IFS= read -r p; do
    [ -n "$p" ] && [ -f "$1/$p" ] || continue
    first=""
    IFS= read -r first < "$1/$p"
    first=${first%$'\r'}
    [ "$first" = "$LFS_HDR" ] && printf '%s\n' "$p"
  done
  return 0
}
hits=$(ptr_scan "$main"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the pointer scan could not read this checkout (rc=$rc)"; exit 1; }
[ -z "$hits" ] || { echo "FAIL (STOP): a path a filter selects at HEAD holds a Git LFS pointer in this checkout — a run tree of this repository would refuse at creation (D21-48 X2); take it to the operator:"; printf '%s\n' "$hits"; exit 1; }
# CONTROLS, in a throwaway repository outside this one: a committed pointer at a SELECTED path is flagged, the same
# bytes at an UNSELECTED path are not (S8D-II-M23's shape), and once the checked-out file is smudged nothing is flagged,
# so the scan reads the checkout and not the committed blob (S8D-II-M24's shape).
t=$(mktemp -d); rc=$?
[ "$rc" -eq 0 ] && [ -d "$t" ] || { echo "FAIL: mktemp -d failed for the pointer-scan control (rc=$rc)"; exit 1; }
pr="$(cygpath -m "$t")/r"
out=$(git init -q "$pr" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the pointer-scan control repository could not be made (rc=$rc): $out"; exit 1; }
printf '%s\n' '*.bin filter=s8dii-ctl' > "$pr/.gitattributes" || { echo "FAIL: the control's .gitattributes could not be written"; exit 1; }
printf '%s\n' "$LFS_HDR" 'oid sha256:0000000000000000000000000000000000000000000000000000000000000000' 'size 1' > "$pr/a.bin" || { echo "FAIL: the control's pointer file could not be written"; exit 1; }
cp "$pr/a.bin" "$pr/spec-quote.txt" || { echo "FAIL: the control's unselected pointer text could not be written"; exit 1; }
out=$(git -C "$pr" -c core.autocrlf=false add -A 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the control's add failed (rc=$rc): $out"; exit 1; }
out=$(git -C "$pr" -c core.autocrlf=false -c user.name=cp8dii -c user.email=cp8dii@invalid commit -q -m scratch 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the control's commit failed (rc=$rc): $out"; exit 1; }
ctl=$(ptr_scan "$pr"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = "a.bin" ] || { echo "FAIL: CONTROL — the scan over a selected pointer and an unselected pointer text printed '$ctl' (rc=$rc), not exactly a.bin"; exit 1; }
printf 'smudged content\n' > "$pr/a.bin" || { echo "FAIL: the control's smudged file could not be written"; exit 1; }
ctl=$(ptr_scan "$pr"); rc=$?
[ "$rc" -eq 0 ] && [ -z "$ctl" ] || { echo "FAIL: CONTROL — the scan flagged '$ctl' after the checked-out file was smudged (rc=$rc), so it reads the committed blob, not the checkout"; exit 1; }
rm -rf "$t" || { echo "FAIL: the pointer-scan control directory could not be removed"; exit 1; }
echo "pointer scan: 0 pointer files at selected paths in this checkout; the controls flagged the selected pointer and neither the unselected text nor the smudged file"

# ---- (1) BUILD-OR-FAIL, before any binary is read (Halt 1).
out=$(nodispatch cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8d-ii NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }
nd_check "build" || exit 1

# ---- (2) THE NAMED TESTS, per crate, each selected exactly. `sel_tests <package> <count> <names…>`.
sel_tests() {
  pkg=$1; want=$2; shift 2
  [ "$#" -eq "$want" ] || { echo "FAIL: the $pkg list names $# tests and its count says $want — the list and the assertion disagree"; exit 1; }
  out=$(nodispatch cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted, ignored, or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed; 0 ignored" || { echo "FAIL: the $pkg selection did not run exactly $want tests with none ignored"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
# CONTROL: a filter that matches nothing EXITS 0. This is why sel_tests asserts each ok line and an exact count.
out=$(nodispatch cargo test -p rk-git --test it -- worktree::s8dii_control_no_such_test --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the zero-match control did not run (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed' || { echo "FAIL: CONTROL — a zero-match filter did not report 0 passed, so this block cannot tell it from a real run"; exit 1; }

sel_tests rk-git 42 worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused worktree::an_item_id_that_case_folds_onto_a_registered_tree_is_refused worktree::the_root_is_the_main_worktree_when_run_from_a_linked_worktree worktree::a_bare_repository_is_refused_by_name worktree::a_symlinked_redkiln_or_worktrees_directory_is_refused worktree::a_created_worktree_whose_realpath_leaves_the_root_is_refused worktree::creation_refuses_when_the_worktree_root_is_not_ignored worktree::creation_refuses_a_path_past_the_platform_limit_by_name worktree::the_length_rule_reads_the_actual_item_length worktree::creation_writes_rk_base_into_the_worktree_gitdir worktree::creation_locks_the_worktree_with_the_pid_and_start_reason worktree::a_lock_held_by_a_dead_pid_is_released worktree::a_lock_whose_pid_now_names_another_process_is_released worktree::a_lock_held_by_a_live_pid_refuses_by_name worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip worktree::reentry_discards_in_flight_changes_and_reports_each_path worktree::reentry_keeps_ignored_files worktree::a_tree_without_rk_base_is_refused_and_never_removed worktree::a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed worktree::a_tree_registered_to_another_repository_is_refused worktree::a_tree_on_a_detached_head_is_refused_at_reentry_and_kept worktree::an_existing_run_branch_is_never_reset worktree::a_dirty_tree_is_kept_with_its_reason worktree::an_unlanded_tree_is_kept_with_its_reason worktree::a_locked_tree_is_not_removed worktree::a_clean_landed_marked_unlocked_tree_is_removed worktree::removal_never_follows_a_junction worktree::a_selected_filter_with_no_driver_at_any_scope_refuses_naming_the_path worktree::a_smudge_skipped_at_local_scope_refuses_naming_the_scope worktree::a_smudge_skipped_at_global_scope_refuses_naming_the_scope worktree::a_smudge_skipped_at_system_scope_refuses_naming_the_scope worktree::git_lfs_skip_smudge_in_the_environment_refuses_by_name worktree::an_include_if_that_skips_the_smudge_for_the_run_tree_refuses_naming_the_included_file worktree::the_filter_attribute_is_read_from_the_fork_commit worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse worktree::a_repo_with_no_filter_or_include_if_creates worktree::a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree worktree::the_pointer_scan_never_removes_a_tree_this_call_did_not_create worktree::a_pointer_header_at_a_path_no_filter_selects_does_not_refuse worktree::a_committed_pointer_that_the_driver_smudges_creates_silently gitops::every_git_child_runs_with_lc_all_c
sel_tests rk-containment 1 containment::process_start_time_reads_a_live_child_and_nothing_for_an_exited_one
sel_tests rk-cli 10 run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees run_worktree::no_redkiln_reader_sees_a_planted_run_worktree run_worktree::creation_carries_worktreeinclude_files_and_runs_setup run_worktree::a_failed_setup_refuses_by_name run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves run_worktree::a_leftover_reference_tree_without_rk_base_is_refused_and_kept init::init_writes_the_redkiln_runtime_exclude_block_once init::init_leaves_the_operators_exclude_lines_untouched run::ac438_table_driven_exit_codes_over_fixtures_binary_arm cost::cost_after_a_real_run_renders_low_sample_through_the_binary
nd_check "named selections" || exit 1

# ---- (3) CODE-LINE FLOORS beneath section 8d-ii.8's behavioural tests — never the assertion.
test -f crates/rk-git/src/worktree.rs && test -f crates/rk-cli/src/verbs/run.rs && test -f crates/rk-git/src/git_exec.rs && test -f crates/rk-cli/src/verbs/init.rs || { echo "FAIL: a source file these floors read is absent (Halt 8)"; exit 1; }
# One derivation of the root. No code line under crates/*/src OR crates/*/tests derives a worktrees directory in any
# spelling — join("worktrees"), join(".redkiln/worktrees"), a joined literal — outside rk-git's worktree module and its
# own tests. Exempt: reads of git's OWN admin directory, <common-dir>/worktrees, which does not move.
ROOT_RE='"worktrees"|join\("[^"]*worktrees|"[^"]*redkiln[^"]*worktrees[^"]*"'
GIT_ADMIN='common(_dir)?(\([^)]*\))?\.join\("worktrees"\)|"\.git/worktrees/'
OWN='^crates/rk-git/(src|tests/it)/worktree\.rs:'
raw=$(grep -rnE "$ROOT_RE" crates/*/src crates/*/tests --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the root search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -vE "$OWN" | grep -vE "$GIT_ADMIN" | grep .)
[ -z "$code" ] || { echo "FAIL: a code line outside rk-git's worktree module derives a worktrees directory — the root is derived in one place (D21-44):"; printf '%s\n' "$code"; exit 1; }
own=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep '^crates/rk-git/src/worktree\.rs:')
[ -n "$own" ] || { echo "FAIL: rk-git's worktree module names no worktrees directory on a code line — the one derivation does not exist"; exit 1; }
# CONTROL: a run.rs derivation and a test's .redkiln/worktrees literal are KEPT; the doctor admin read, a comment and
# the worktree module's own line are DROPPED.
ctl=$(printf '%s\n' 'crates/rk-cli/src/verbs/run.rs:9:    let p = common.join("redkiln").join("worktrees").join(item);' 'crates/rk-cli/tests/it/run.rs:5:    let t = root.join(".redkiln/worktrees").join("x");' 'crates/rk-cli/src/doctor/rules.rs:12:    let Ok(e) = std::fs::read_dir(ctx.common_dir.join("worktrees")) else {' 'crates/rk-cli/src/verbs/run.rs:10:    // the old root joined "worktrees" under the common dir' 'crates/rk-git/src/worktree.rs:3:    root.join(".redkiln").join("worktrees")' | grep -E "$ROOT_RE" | grep -vE "$CODE_ONLY" | grep -vE "$OWN" | grep -vE "$GIT_ADMIN" | grep -c .)
[ "$ctl" -eq 2 ] || { echo "FAIL: CONTROL — the root filter kept $ctl of two derivations, one admin read, one comment and one own-module line"; exit 1; }
# W2: recreate_run_worktree is gone.
raw=$(grep -rn 'recreate_run_worktree' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the recreate search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: recreate_run_worktree is still on a code line — D21-45 W2 replaces it:"; printf '%s\n' "$code"; exit 1; }
# W2 and ADR-0162 decision 10: the only forced removal in run.rs ("--force", "-f" or remove_dir_all) lies inside
# remove_reference_worktree, the scratch reference tree's; rk-git's worktree module forces nothing.
rr=$(awk '/^fn remove_reference_worktree\(/ { s = NR } s && NR > s && /^}/ { print s, NR; exit }' crates/rk-cli/src/verbs/run.rs)
read -r rs re <<< "$rr"
[ -n "$rs" ] && [ -n "$re" ] || { echo "FAIL: fn remove_reference_worktree was not found in run.rs — ADR-0162 decision 10's removal has no home"; exit 1; }
raw=$(grep -nE '"--force"|"-f"|remove_dir_all' crates/rk-cli/src/verbs/run.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the run.rs force-removal search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
outside=$(printf '%s\n' "$code" | awk -F: -v s="$rs" -v e="$re" '$1 != "" && ($1 + 0 < s + 0 || $1 + 0 > e + 0)')
[ -z "$outside" ] || { echo "FAIL: run.rs forces a removal outside remove_reference_worktree — W2 is never force-remove:"; printf '%s\n' "$outside"; exit 1; }
# CONTROL: with the function at lines 10-20, a hit on line 5 is KEPT and a hit on line 15 is DROPPED.
ctl=$(printf '%s\n' '5:    "--force",' '15:    "--force",' | awk -F: -v s=10 -v e=20 '$1 != "" && ($1 + 0 < s + 0 || $1 + 0 > e + 0)' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the function-range filter kept $ctl of one outside hit and one inside hit"; exit 1; }
raw=$(grep -nE '"--force"|"-f"|remove_dir_all' crates/rk-git/src/worktree.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the worktree-module force search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: rk-git's worktree module forces a removal on a code line — W2 is never force-remove:"; printf '%s\n' "$code"; exit 1; }
# W1: LC_ALL=C set on the child at the one git exec site — the call form, not a mention.
raw=$(grep -nE '"LC_ALL"[[:space:]]*,[[:space:]]*"C"' crates/rk-git/src/git_exec.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the LC_ALL search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in git_exec.rs sets LC_ALL to C — W1's locale pin is absent"; exit 1; }
# CONTROL: the call form is kept; an attribute or string that merely names LC_ALL is not.
ctl=$(printf '%s\n' '40:        cmd.env("LC_ALL", "C");' '41:    #[expect(clippy::x, reason = "LC_ALL is set below")]' | grep -E '"LC_ALL"[[:space:]]*,[[:space:]]*"C"' | grep -vE "$LINE_ONLY" | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the LC_ALL form kept $ctl of one call and one reason string"; exit 1; }
raw=$(grep -nF 'RK_BASE' crates/rk-git/src/worktree.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the RK_BASE search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in rk-git's worktree module names RK_BASE — W1's ownership marker is absent"; exit 1; }
# R8 and V1: the config read carries the scope, the attribute read names its source commit, and nothing narrows the
# config read to --local.
raw=$(grep -nF -e '--show-scope' -e '--source' -e '"--local"' crates/rk-git/src/worktree.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the filter-read search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
printf '%s\n' "$code" | grep -qF -- '--show-scope' || { echo "FAIL: no code line in rk-git's worktree module reads the config with --show-scope — R8's scope-naming diagnosis has no source"; exit 1; }
printf '%s\n' "$code" | grep -qF -- '--source' || { echo "FAIL: no code line in rk-git's worktree module reads attributes with --source — V1 reads them from the fork commit"; exit 1; }
local_hits=$(printf '%s\n' "$code" | grep -F '"--local"')
[ -z "$local_hits" ] || { echo "FAIL: the worktree module narrows a config read to --local on a code line — R8 reads every scope:"; printf '%s\n' "$local_hits"; exit 1; }
# CONTROL: the line filter KEEPS a --local code line and DROPS a comment naming it.
ctl=$(printf '%s\n' '12:        .arg("--local")' '13:    // not "--local" (R8)' | grep -vE "$LINE_ONLY" | grep -F '"--local"' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the --local filter kept $ctl of one code line and one comment"; exit 1; }
# D21-48 X2: the post-checkout scan's pointer header is on a code line of rk-git's worktree module (a floor beneath
# the named tests, never the assertion).
raw=$(grep -nF 'https://git-lfs.github.com/spec/v1' crates/rk-git/src/worktree.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the pointer-header search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in rk-git's worktree module carries the Git LFS pointer header — D21-48 X2's post-checkout scan is absent"; exit 1; }
# W4: init's exclude marker is on a code line.
raw=$(grep -nF '# redkiln-runtime' crates/rk-cli/src/verbs/init.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the exclude-marker search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: init.rs carries no # redkiln-runtime marker on a code line — W4's exclude block is absent"; exit 1; }

# ---- (4) THE WHOLE SUITE, no fail-fast, with a floor. Expected value, re-derived at stage open from Checkpoint 8d's
# ---- recorded count plus 50 (section 8d-ii.0 item 8): 914 + 50 = 964 on the final chain (D21-48 X2, 2026-09-24:
# ---- 46 -> 50; D21-48 floor propagation, 2026-09-24: was 901 + 50 = 951, before s8c 78 and s8d 62). Replace it by a dated edit before the first run.
FLOOR=964
tally() { printf '%s\n' "$1" | awk '/^test result:/ { p += $4; f += $6; i += $8; n += 1 } END { print p+0, f+0, i+0, n+0 }'; }
# CONTROL: a run of zero tests tallies below the floor, and a failed count is read.
read -r cp cf ci cn <<< "$(tally 'test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out')"
[ "$cp" -lt "$FLOOR" ] || { echo "FAIL: CONTROL — a zero-test run tallied $cp passed, not below the floor $FLOOR"; exit 1; }
read -r cp cf ci cn <<< "$(tally 'test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out')"
[ "$cf" -eq 2 ] || { echo "FAIL: CONTROL — tally read $cf failed from a line carrying 2"; exit 1; }
out=$(nodispatch cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
read -r passed failed ignored results <<< "$(tally "$out")"
[ "$failed" -eq 0 ] && [ "$passed" -ge "$FLOOR" ] && [ "$results" -ge 44 ] || { echo "FAIL: the suite read passed=$passed failed=$failed ignored=$ignored over $results results — the floor is $FLOOR passed over at least 44 results"; exit 1; }
echo "whole suite: passed=$passed failed=$failed ignored=$ignored results=$results (floor $FLOOR) — record these in PROGRESS.md"
nd_check "whole suite" || exit 1

# ---- (5) fmt and clippy, gated on exit codes (never `-D warnings` on the command line — ADR-0083).
out=$(nodispatch cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nodispatch cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }
nd_check "fmt and clippy" || exit 1

# ---- (6) THE WORKTREE PATH SET, compared BEFORE the re-run blocks, and no stale registration a nested prune could
# ---- remove. (8) compares the set again after the re-runs.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

# ---- (7) RE-RUN FROM THEIR OWN BYTES: the entry gate (8d) and the one checkpoint this stage's edits make provisional
# ---- (7f, amended in section 8d-ii.7). Nothing older. SKIPPED WHOLE under RK_CP_NESTED=1 (D21-46: running as another
# ---- checkpoint's entry gate), and set for every block this one calls. Every replay runs under `nodispatch` (00),
# ---- and nd_check follows it.
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
  # PRECONDITION, inside the non-nested branch (D21-47): no replay below can reach a real dispatch. Before s8h no
  # shipped card carries permission_mode.
  ncards=0
  for c in .redkiln/cards/*.json; do [ -f "$c" ] && ncards=$((ncards + 1)); done
  [ "$ncards" -gt 0 ] || { echo "FAIL: no card files under .redkiln/cards — the precondition would hold over nothing (Halt 8)"; exit 1; }
  moded=$(grep -lF '"permission_mode"' .redkiln/cards/*.json); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: the card-mode precondition could not run (grep rc=$rc)"; exit 1; }
  [ -z "$moded" ] || { echo "FAIL (PRECONDITION): a shipped card carries permission_mode, so s8h has landed — re-run 7f only through s8h's fixture-child route, not from this block:"; printf '%s\n' "$moded"; exit 1; }
  # SYNTHETIC CONTROL: the probe sees a planted mode.
  printf '{"permission_mode": "default"}\n' > "$scratch/card.json" || { echo "FAIL: the card control could not be written"; exit 1; }
  grep -qF '"permission_mode"' "$scratch/card.json" || { echo "FAIL: CONTROL — the card-mode probe cannot see a planted permission_mode"; exit 1; }
  replay() {
    out=$(nodispatch env RK_CP_NESTED=1 "$@" 2>&1); rc=$?
  }

  # (7a) CHECKPOINT 8d, the entry gate, whole. s8d's block honours RK_CP_NESTED=1 and skips its own re-runs of 8c and 7g.
  ext runbook/phase-1/s8d-step-grain-and-resolver.md 'echo "CP8D-END-OF-BLOCK reached"' "$scratch/cp8d.sh"
  replay bash "$scratch/cp8d.sh"
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8d, re-run whole, exited $rc — this stage's entry gate no longer holds"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8D-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8d exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 8d replay" || exit 1

  # (7b) CHECKPOINT 7f whole, AS AMENDED (section 8d-ii.7). Its CODE lines (comments stripped, so a comment naming a
  # clone satisfies nothing) clone into a mktemp directory outside this repository, and remove or prune no worktree
  # of this repository in any spelling. Then the replay is judged by what it did not change here.
  ext runbook/phase-1/s7f-conformance-and-records.md 'echo "CP7F-END-OF-BLOCK reached"' "$scratch/cp7f.sh"
  code7f=$(grep -vE '^[[:space:]]*#' "$scratch/cp7f.sh")
  test -n "$code7f" || { echo "FAIL: Checkpoint 7f's block has no code lines"; exit 1; }
  printf '%s\n' "$code7f" | grep -qE 'git clone .*--no-local' || { echo "FAIL: no code line in 7f clones this repository (git clone --no-local) — its run arm would create a run tree in the product repository"; exit 1; }
  printf '%s\n' "$code7f" | grep -qF 'mktemp -d' || { echo "FAIL: no code line in 7f makes its clone's directory with mktemp -d — under this repository the length rule refuses it (section 8d-ii.7)"; exit 1; }
  wt7f=$(printf '%s\n' "$code7f" | grep -nE 'worktree[[:space:]]+(remove|prune)')
  [ -z "$wt7f" ] || { echo "FAIL: Checkpoint 7f still removes or prunes a worktree of this repository — D21-44 keeps the legacy tree for s8j:"; printf '%s\n' "$wt7f"; exit 1; }
  runs7f=$(printf '%s\n' "$code7f" | grep -F '.git/redkiln/runs' | grep -vE 'c7f|[$][{]?clone')
  [ -z "$runs7f" ] || { echo "FAIL: Checkpoint 7f reads this repository's .git/redkiln/runs, not its clone's:"; printf '%s\n' "$runs7f"; exit 1; }
  # CONTROL: the unamended 7f's cleanup and prune lines are caught, whether -f or --force, and a comment is not.
  ctl=$(printf '%s\n' 'git worktree remove --force .git/redkiln/worktrees/x >/dev/null 2>&1 || true' 'git worktree remove -f x' 'git worktree prune' '# git worktree remove is gone' | grep -vE '^[[:space:]]*#' | grep -cE 'worktree[[:space:]]+(remove|prune)')
  [ "$ctl" -eq 3 ] || { echo "FAIL: CONTROL — the 7f removal filter caught $ctl of three removal lines and one comment"; exit 1; }
  # CONTROL: a runs read naming this repository is KEPT; one naming the clone ($clone or c7f) is DROPPED.
  ctl=$(printf '%s\n' 'ls .git/redkiln/runs' 'ls "$clone/.git/redkiln/runs"' 'ls "$c7f/c/.git/redkiln/runs"' | grep -F '.git/redkiln/runs' | grep -vE 'c7f|[$][{]?clone' | grep -c .)
  [ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the 7f runs-read filter kept $ctl of one local read and two clone reads"; exit 1; }
  wt7_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
  runs7_before=$(find "$common/redkiln/runs" -mindepth 1 -maxdepth 1 2>/dev/null | grep -c .)
  replay bash "$scratch/cp7f.sh"
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7f, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7F-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7f exited 0 without reaching its end marker"; exit 1; }
  wt7_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
  [ "$wt7_before" = "$wt7_after" ] || { echo "FAIL: Checkpoint 7f's replay changed this repository's worktree PATH SET:"; diff <(printf '%s\n' "$wt7_before") <(printf '%s\n' "$wt7_after"); exit 1; }
  runs7_after=$(find "$common/redkiln/runs" -mindepth 1 -maxdepth 1 2>/dev/null | grep -c .)
  [ "$runs7_after" -eq "$runs7_before" ] || { echo "FAIL: Checkpoint 7f's replay opened a run record in THIS repository ($runs7_before -> $runs7_after) — its run arm did not run in the clone"; exit 1; }
  nd_check "Checkpoint 7f replay" || exit 1
fi

# ---- (8) AFTER: nothing this block ran touched the product repository or reached a real claude.
ref_after=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ "$ref_after" = "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count moved during this block ($ref_before -> $ref_after)"; exit 1; }
rw_after=$(ls -A .redkiln/worktrees 2>/dev/null)
[ "$rw_before" = "$rw_after" ] || { echo "FAIL: .redkiln/worktrees changed during this block ('$rw_before' -> '$rw_after') — a run created a tree in the product repository"; exit 1; }
wt_end=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_end" ] || { echo "FAIL: the worktree PATH SET changed across the re-runs:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_end"); exit 1; }
runs_after=$(find "$common/redkiln/runs" -mindepth 1 -maxdepth 1 2>/dev/null | grep -c .)
[ "$runs_after" -eq "$runs_before" ] || { echo "FAIL: this repository's run-record count moved ($runs_before -> $runs_after) — a lane ran against the product repository"; exit 1; }
d_after=$(python "$scratch/dcount.py" "$store" "$common/redkiln/runs"); rc=$?
[ "$rc" -eq 0 ] && [ "$d_after" = "$d_before" ] || { echo "FAIL: dispatch_completed rows (store, journals) moved from '$d_before' to '$d_after' (rc=$rc) — something dispatched"; exit 1; }
# The trap is still empty and the fake logged no call (the helper's controls proved both readings at nd_setup).
nd_check "end" || exit 1

# ---- (9) THE RECORDS THIS STAGE WRITES, located by REC_WT (the adversarial review's blocker, D21-47).
fr=$(ls references/adr/"$REC_WT"-*.md 2>/dev/null)
[ "$(printf '%s\n' "$fr" | grep -c .)" -eq 1 ] || { echo "FAIL: expected exactly one record file for REC_WT=$REC_WT, found: $fr"; exit 1; }
st=$(sed -nE 's/^status: (proposed|accepted)$/\1/p' "$fr")
[ -n "$st" ] || { echo "FAIL: $fr carries neither status: proposed nor status: accepted"; exit 1; }
[ "$(grep -c '^### Option — ' "$fr")" -ge 2 ] || { echo "FAIL: $fr lacks the ADR-0085 option headings (D21-42 24a)"; exit 1; }
grep -qF 'What this forbids' "$fr" || { echo "FAIL: $fr carries no forbids block"; exit 1; }
sup=$(grep -m1 '^supersedes:' "$fr"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: $fr has no supersedes line"; exit 1; }
for s in 0105 0107 0137; do
  printf '%s\n' "$sup" | grep -qF "\"$s\"" || { echo "FAIL: $fr does not declare that it supersedes $s in part (section 8d-ii's records table)"; exit 1; }
done
for s in 0018 0162; do
  if printf '%s\n' "$sup" | grep -qF "\"$s\""; then echo "FAIL: $fr supersedes $s, which it cites only (sections 8d-ii.1 and 8d-ii.3)"; exit 1; fi
done
# The moved sentences are quoted, V1's reading is stated, and the migrate worktree's scope is named.
for q in '<common-dir>/redkiln/worktrees/<item>' 'never reused' 'Anything not listed is a successor' 'changes nothing and exits 0' 'pointer files would result' 'AC-522' 'removes its own reference worktree' 'Refuse by name' 'post-checkout'; do
  grep -qF -- "$q" "$fr" || { echo "FAIL: $fr does not quote or name '$q'"; exit 1; }
done
# Back-links move in the flip commit: none while the record is proposed, all three once it is accepted.
for s in 0105 0107 0137; do
  pf=$(ls references/adr/"$s"-*.md 2>/dev/null)
  [ "$(printf '%s\n' "$pf" | grep -c .)" -eq 1 ] || { echo "FAIL: expected one file for ADR-$s"; exit 1; }
  bl=$(grep -m1 '^superseded_by:' "$pf"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: ADR-$s has no superseded_by line"; exit 1; }
  if [ "$st" = "proposed" ]; then
    if printf '%s\n' "$bl" | grep -qF "\"$REC_WT\""; then echo "FAIL: ADR-$s is back-linked to a PROPOSED record — the link moves in the flip commit"; exit 1; fi
  else
    printf '%s\n' "$bl" | grep -qF "\"$REC_WT\"" || { echo "FAIL: $fr is accepted and ADR-$s carries no superseded_by link to it"; exit 1; }
  fi
done
grep -qE "^\| \[$REC_WT\]\($REC_WT-" references/adr/README.md || { echo "FAIL: references/adr/README.md has no index row for $REC_WT"; exit 1; }
ledger=$(grep -lF "ADR-$REC_WT" spikes/DECISIONS-*.md); rc=$?
[ "$rc" -le 1 ] && [ -n "$ledger" ] || { echo "FAIL: no ledger entry names ADR-$REC_WT (rc=$rc)"; exit 1; }
# The register notes: each criterion's OWN block (its heading to the next criterion, rule or heading) names the record.
cat > "$scratch/acnote.py" <<'PY'
import sys
reg, needle = sys.argv[1], sys.argv[2]
lines = open(reg, encoding="utf-8").read().split(chr(10))
missing = []
for ac in sys.argv[3:]:
    head = "**" + ac + "**"
    starts = [i for i, l in enumerate(lines) if l.startswith(head)]
    if len(starts) != 1:
        missing.append("%s (%d headings)" % (ac, len(starts)))
        continue
    i = starts[0] + 1
    block = [lines[starts[0]]]
    while i < len(lines) and not lines[i].startswith("**AC-") and lines[i].strip() != "---" and not lines[i].startswith("#"):
        block.append(lines[i])
        i += 1
    if needle not in chr(10).join(block):
        missing.append(ac)
if missing:
    print("MISSING: " + ", ".join(missing))
    sys.exit(1)
print("NOTES-OK")
PY
out=$(python "$scratch/acnote.py" references/2026-09-08-acceptance-criteria.md "ADR-$REC_WT" AC-518 AC-522 AC-529 AC-531 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the register's dated notes are incomplete: $out"; exit 1; }
# CONTROL: a criterion whose block lacks the needle is reported, though the needle sits in the NEXT criterion's block.
printf '%s\n' '**AC-900** x' 'THEN y' '' '**AC-901** z' 'note ADR-9999' '---' > "$scratch/reg-control.md" || { echo "FAIL: the register control could not be written"; exit 1; }
out=$(python "$scratch/acnote.py" "$scratch/reg-control.md" "ADR-9999" AC-900 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: CONTROL — the note reader accepted a block without the needle (rc=$rc): $out"; exit 1; }
out=$(python "$scratch/acnote.py" "$scratch/reg-control.md" "ADR-9999" AC-901 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — the note reader refused a block that carries the needle (rc=$rc): $out"; exit 1; }
# Design section 5's reconcile_started row and section 6's GitOps write-set row each carry the dated note.
D=references/2026-09-08-architecture-design.md
row=$(grep -F '| `reconcile_started` |' "$D"); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$row" | grep -c .)" -eq 1 ] || { echo "FAIL: design's reconcile_started row is not exactly one line (rc=$rc)"; exit 1; }
printf '%s\n' "$row" | grep -qF "ADR-$REC_WT" && printf '%s\n' "$row" | grep -qF '.redkiln/worktrees/' || { echo "FAIL: design's reconcile_started row carries no dated note naming ADR-$REC_WT and .redkiln/worktrees/"; exit 1; }
row=$(grep -F 'The write set, enumerated' "$D"); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$row" | grep -c .)" -eq 1 ] || { echo "FAIL: design's GitOps write-set row is not exactly one line (rc=$rc)"; exit 1; }
printf '%s\n' "$row" | grep -qF "ADR-$REC_WT" || { echo "FAIL: design's GitOps row ('re-created per run, never reused') carries no dated note naming ADR-$REC_WT"; exit 1; }
grep -qF "ADR-$REC_WT" runbook/phase-1/s7d-*.md || { echo "FAIL: s7d section 7d.5's tables carry no dated note naming ADR-$REC_WT"; exit 1; }
grep -qF '.redkiln/worktrees/<item>' crates/rk-cli/tests/fixtures/bin/rk-fixture-toolchain.rs || { echo "FAIL: the rigged stub's doc still names the old candidate-arm directory"; exit 1; }
rm -rf "$scratch" || { echo "FAIL: the scratch directory $scratch could not be removed"; exit 1; }

echo "CP8D-II-END-OF-BLOCK reached"
```

---
