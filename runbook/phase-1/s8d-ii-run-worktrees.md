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
- **Checkpoint 8d now honours `RK_CP_NESTED=1`**, so (7a)'s replay of it skips 8d's own re-runs of ~~8c and 7g~~ 8c,
  8b, 7g, 7a and 5 *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* (Checkpoint 8d's (13) reads *"its re-runs of 8c, 8b, 7g, 7a and 5 ran at
  Checkpoint 8d's own close"*; this named two of the five). The
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
only on the operator's dated ledger line. *(dated 2026-09-28, `s8d-ii` close: ADR-0180 flipped `accepted` on D21-91, on its own dated line under D21-64 ruling 10.)*

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
  asserts whichever state the record's status requires. *(dated 2026-09-28, `s8d-ii` close: they moved in ADR-0180's flip commit, on D21-91; `0180` is appended to each `superseded_by`.)*

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
   count**, plus this stage's ~~**50**~~ **61** new tests (§8d-ii.8) *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* (53 at package D2188, and 61 once
   round 1's fixes added eight: `rk-git` six, `rk-cli` two; this read **50** after D2188 moved the floor, a stale
   instruction round 1 found). Computed once, on the final chain: 740 (ADR-0164's
   evidence, `992d257`) + 9 (8a) + 25 (8b) + 78 (8c) + 62 (8d) = 914 at Checkpoint 8d, so 914 + 50 = ~~**964**~~ **1021** *(dated 2026-09-27, s8d-ii build, package P1-RECORD:
   re-derived from Checkpoint 8d's RECORDED count, 971 passed / 0 failed over 44 at `b183eb9` (D21-78); its
   post-flip re-run after `0b470ee` passed again (rc 0, 0 FAIL), and this stage's open baseline read 971, so 971 + 50 =
   1021; the 914 chain above was the planned figure)* *(dated 2026-09-27, s8d-ii build, package D2188.)* **1024**: 971 + 53, D21-88 ruling 2 adding three
   tests (§8d-ii.8's count). *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* **1032**: 971 + 61, round 1's fixes adding eight tests (§8d-ii.8's count). *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)* **1034**: 971 + 63,
   round 2's fixes adding two tests (§8d-ii.8's count).
   Replace ~~964~~ in Checkpoint 8d-ii's `FLOOR=` line with Checkpoint 8d's recorded count plus ~~50~~ ~~61~~ 63, by a dated edit, before the
   block's first run. It is never lowered to make a red run green. *(It read 32 and 917 before the adversarial-review
   fixes of 2026-09-24, and 885 + 46 = 931 before the final consistency pass of 2026-09-24. *D21-48 (2026-09-24):* it read 46
   and 947 before X2's four tests. *D21-48 (2026-09-24), floor propagation:* it read 901 + 50 = 951 before `s8c`'s
   78 and `s8d`'s 62.)*
9. **The ignore refusal is no longer a STOP** (D21-48 X1, 2026-09-24). The draft asked it here, before any code,
   because D21-46 had flagged it for the flip line and the ledger carried no answer. D21-48 X1 answers it:
   *"**Refuse by name.** This answers D21-46's flip-line flag."* Re-measure only that the ledger line stands: `spikes/DECISIONS-2026-09-21.md`'s
   D21-48 table carries the X1 row. §8d-ii.1 builds the refusal.

*(dated 2026-09-27, s8d-ii build, package P1-RECORD.)* **The stage-open readings, re-taken at `83cf836` (the tracker
commit after `s8d`'s flip commit `0b470ee`), read-only, before any code.** Next free ADR `0180` (`ls references/adr` ended at
`0179`) and next ledger id `D21-87` (the last heading was `## D21-86`) at open.
1. **Registered.** `git worktree list --porcelain` lists `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count`
   at `73c087a82245b342811208655323ae4f49bf879b` on `refs/heads/rk-run/rk-fix-doctor-test-count`; the ref is `73c087a…`
   and its `Story:` trailer reads `redkiln-rs/rk-fix-doctor-test-count`. Neither the tree nor the ref was touched. The
   linked tree `D:/repos/redkiln-rs.swt` (`spike/s-h1b-s-wt`) is still registered.
2. **140**, the same file (`references/adr/0114-…`). Bounds: 39 + 64 + 11 + 1 + 140 = **255** here; 39 + 24 + 11 + 1 + 140
   = **215** for `rk-fix-doctor-test-count`; a `mktemp -d` root in `cygpath -m` form plus `/c` measured 50 characters
   (`C:/Users/ryanm/AppData/Local/Temp/tmp.6KlMDxskDr/c`), so 7f's clone bound is 50 + 20 + 24 + 11 + 1 + 140 = **246**.
   `git config --get core.longpaths` exits 1. The worktree record's own file name (81 characters with its directory)
   does not move the longest path.
3. **Unchanged.** Four `system` rows from `C:/Program Files/Git/etc/gitconfig` (`filter.lfs.clean`, `.smudge`,
   `.process`, `.required true`); `git check-attr --source=HEAD --stdin filter` over `ls-tree -r --name-only HEAD`
   selects no driver (0 lines other than `unspecified`/`unset`); `--get-regexp '^includeif\.'` exits 1. V1's condition
   holds.
4. **No recursive walk over `.redkiln/`.** The `read_dir` sites that touch `.redkiln/` read one named subdirectory one
   level deep: `cards` and `specs` (`doctor/rules.rs`), `skills` (`doctor/roles.rs`), `catalog` (`run.rs`'s
   `catalog_sources`), and, new since `c5d19d7`, `processes` (`rk-cli/src/processes.rs`, *"Not recursive, and `*.toml`
   only"*). `Catalog::from_dir` and `load_diagnostic_dir` read the directory they are handed; `init`'s
   `preexisting_rk_names` reads the root's top level. The two recursive walks under `crates/*/src`,
   `kernel_denylist::read_rust_sources` and `module_boundary`'s use of it, walk `crates/rk-kernel/src`, not `.redkiln/`.
5. **Holds.** `init.rs`'s module doc reads *"It does exactly two things"* and *"installs no hook (AC-412)"*; the only
   `# redkiln-hook` on a code line is `doctor/rules.rs`'s `const MARKER: &str = "# redkiln-hook";` (the two other hits
   are a doc comment and a message string in `tests/it/doctor.rs`).
6. **The list, re-measured (P6 consumes it).** Fixture repositories a run reaches: AC-438's exit table,
   `crates/rk-cli/tests/fixtures/exit/{cancelled-timeout, findings-block, gate-open, no-pack, no-record,
   platform-is-error, resume-flag, schema-invalid-structured-output, schema-no-structured-output,
   transport-missing-binary}`; `cost`'s run (`cost::cost_after_a_real_run_renders_low_sample_through_the_binary`) runs over
   `pack_min_repo(PackVariant::Base)`, not a fixture directory. Code builders: `temp_repo` (`tests/it/main.rs`),
   `pack_min_repo` (`support.rs`), `repo_with_open_gate` (`answer.rs`), `synthetic_trailer_repo` (`doctor.rs`),
   `repo_seeded_from` and `repo_with_record` (`explain.rs`), the `adr0128_*` tests' own (`run.rs`, `surface.rs`), and,
   added since `fc80687`, `change_repo` (`change_deriver.rs`), `gate_repo_with_questions` (`processes.rs`) and
   `source_repo` (`verb_rows.rs`, its own `git init`). The files whose tests invoke `run` (`verbs::run::run_with` or the
   binary's `run`) are `adapter_selection.rs` (new since `fc80687`), `config.rs`, `cost.rs`, `exit.rs`, `root.rs`,
   `run.rs` and `surface.rs`; `review_pass.rs`, on the planning list, names `"run"` once and calls neither form, and
   `answer.rs`, `change_deriver.rs`, `story_axis.rs`, `support.rs` and `transcription.rs` call other `verbs::run`
   functions (`story_value`, `through_step_seven`, `tool_routing`) without creating a tree. A test that fails
   with the ignore refusal's name after the move is still a builder this list missed.
7. **Free and present as planned.** `crates/rk-cli/tests/it/run_worktree.rs` does not exist (new);
   `crates/rk-git/tests/it/worktree.rs`, `gitops.rs`, `crates/rk-cli/tests/it/init.rs` and
   `crates/rk-containment/tests/it/containment.rs` exist.
8. **1021** (971 + 50), written into (4)'s `FLOOR=` line and item 8 above by dated edit. *(dated 2026-09-27, s8d-ii build, package D2188.)* Now **1024**
   (971 + 53) under D21-88 ruling 2's three tests. *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)* Now **1034** (971 + 63): round 1's eight tests
   (package R1FIX-8dii, which moved the floor to 1032 and not this line) and round 2's two.
9. **Stands.** D21-48's table carries *"| X1 | s8d-ii: creation when `.redkiln/worktrees/` is not git-ignored |
   **Refuse by name.** This answers D21-46's flip-line flag. |"*.

**Baseline** (SO-11), in a private `git worktree add --detach target/wt-p1rec HEAD` at `83cf836`, after `pnpm install
--frozen-lockfile --prefer-offline` of the node fixture, through the no-dispatch helper at `CARGO_BUILD_JOBS=4
RUST_TEST_THREADS=4`: `cargo test --workspace --no-fail-fast` rc 0, **971 passed / 0 failed / 10 ignored over 44 result
lines**, the count Checkpoint 8d recorded. Removing that tree with `git worktree remove --force` left s8c §8c.12 item 3's shape again: a `node_modules` junction skeleton, 114 links and 0 files, under `target/wt-p1rec`, invisible to `git worktree list`; it was removed with a walk that does not follow junctions.

**A finding for the package that moves the root (not settled here).** (3)'s root floor also reads two lines `s8c`
added after this file was planned: `crates/rk-git/src/worktree_include.rs`'s
`pub const RUN_WORKTREES_EXCLUDE: &str = ":(exclude).redkiln/worktrees";` (the `.worktreeinclude` reader's pathspec, D21-44)
and `crates/rk-git/tests/it/worktree_include.rs`'s planted `.redkiln/worktrees/x/.env`. Neither is exempt under `OWN`
(`worktree\.rs:` only), so the floor stays red on them after every run-tree derivation moves. Whichever way the root-move
package meets them (route the pathspec through the one derivation, or widen `OWN` by a dated edit with a control), it
is that package's to choose and record.
*(dated 2026-09-27, s8d-ii build, package P6-RKCLI-ROOT-MOVE.)* **Settled by routing the pathspec through the one
derivation's module**, not by widening `OWN`: `rk_git::worktree::RUN_ROOT_EXCLUDE_PATHSPEC` holds the literal beside
`derive` (with a `const` assertion tying it to `RUN_ROOT_IGNORE_ROW`), `worktree_include::RUN_WORKTREES_EXCLUDE` names
it, and the test plants its tree at `derive(&r, "x").path`. The root floor reads 0 lines outside `OWN` (§8d-ii.8's P6
note).

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
  - *(dated 2026-09-29, `s8g` build, package S8G-P3-KERNEL-GIT; `s8g` §8g.1 move 3; R1 = ADR-0191 decision 11,
    proposed.)* it has no `.mut` suffix, case-folded. `s8g`'s verify pass names its ephemeral mutation tree
    `<item>.mut` beside the run tree and the reference tree (`s8g` §8g.5), so an item id ending that way would collide
    with another item's mutation tree. `rk_git::worktree::derive` gives the tree as `RunTree::mutation` and the
    validator refuses it as `ItemIdRefusal::MutSuffix` (`mut-suffix`); the table-driven test
    `worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused` gains the rows `x.mut` and `x.MUT`,
    and the controls `x.muts` and `mut`. The 4-character suffix fits inside the 11 characters the worst-case path sum
    reserves for `.review-ref` (a `const` assertion in `worktree.rs` holds `MUT_SUFFIX.len() <= REVIEW_REF_SUFFIX.len()`),
    so §8d-ii.6's bound and Checkpoint 8d-ii (0b) are unchanged; Checkpoint 8g (0a) recomputes the sum (255 of 259,
    measured 2026-09-29).

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
  *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* **D21-89, by the operator on refuting round 1's findings:** *"On the existing-branch path: (1)
  the filter read, the pointer scan's path set and the length rule read the **commit being checked out** (the run
  branch's tip, read before the add); (2) `RK_BASE` records **`merge-base(start HEAD, run-branch tip)`** — the point
  the branch forked. First creation is unchanged (fork = start's HEAD)."* The path is routine after D21-88 ruling
  2's rollback, which keeps the branch. Round 1's probe created a tree holding a pointer file here with no refusal,
  and `RK_BASE` recorded start's moved `HEAD` as the fork (§8d-ii.9 findings 24 and 25).
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
  lost"*). Redkiln never removes a run tree it did not create. ~~The one~~ *One* removal of a tree that is not landed is D21-48 X2's
  rollback of the tree the current call has just created, before anything runs in it, and it is unforced too
  (§8d-ii.5). *(dated 2026-09-27, s8d-ii build, package D2188.)* The other is D21-88 ruling 2's rollback of a fresh tree whose carry or `setup` failed
  (§8d-ii.4), unforced and through the same pre-unlink as the landed removal. This stage builds the removal rule and its tests. **The
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
    The fork sha in the run tree's `RK_BASE` is the base, read without a `merge-base` *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii: at the
    reference tree's creation; a run tree created on an existing branch records a merge-base in `RK_BASE` at ITS
    creation, D21-89 ruling 2, a read that supersedes nothing)*~~, which clause 5 still does not
    carry~~ *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii; refuting round 2: the `GitOp` set now carries
    `MergeBase`, D21-89 ruling 2's read, which rk-git's `create` runs only when a run tree is created on an existing
    branch; the reference tree itself still reads its base from `RK_BASE` and runs no `merge-base`. On that arm the
    fork is the merge-base, not the tip the run tree checks out, so the length rule and V1's filter read also run over
    it, §8d-ii.6 and §8d-ii.5)*. `run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves` moves `HEAD`
    between two invocations and asserts that the reference tree's `HEAD` is the recorded fork sha. The doc is
    rewritten in the same commit.
  - **A leftover is removed only if redkiln made it.** The reference tree gets its own `RK_BASE` at creation. A
    leftover at `<item>.review-ref` from a killed run that carries the marker is force-removed as scratch, as today.
    One without it is refused by name and kept
    (`run_worktree::a_leftover_reference_tree_without_rk_base_is_refused_and_kept`).

  Decision 10's contrast clause, *"**unlike** the run-owned worktree, which `recreate_run_worktree` recreates and never
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

*(dated 2026-09-27, s8d-ii build, package D2188.)* **A fresh tree whose carry or `setup` fails is rolled back before the run refuses** (D21-88 ruling 2:
*"when a FRESH tree's carry or `setup` fails, the tree is removed (unforced, through the same pre-unlink path) before
the run refuses by name, as D21-48 X2's pointer scan already does, so the next run creates it anew and re-carries. A
reused tree is still not re-carried."*). Without it, finding 18: the failed tree stayed registered, and the next run
re-entered it as reused and skipped both the carry and `setup`. `rk_git::worktree::roll_back_fresh` touches only the
tree this run has just created (not `Reused`, registered at the derived path, its `RK_BASE` naming the item and this
creation's fork, and holding this run's own lock); it releases the lock, unlinks every untracked link without
following it, and runs git's unforced `worktree remove`. The branch is kept, so the next run checks it out anew on
the existing branch and re-carries. If git refuses (an untracked, unignored file a failed `setup` left) or a residue
remains, the tree is kept and named on stderr, never forced; the next run then reuses it without a carry, which
ADR-0180's consequences state. `run` prints the rollback's outcome and exits with the carry's or `setup`'s own
refusal. Tests: `run_worktree::a_fresh_tree_whose_setup_failed_is_rolled_back_and_the_next_run_recarries` (the first
run refuses and leaves no registered tree; the second re-creates and re-carries, and `setup` installs; a third reuses
and does not re-carry, the control), `run_worktree::a_failed_setup_refuses_by_name` (its last assertion reversed:
the tree is no longer registered), and `rk-git`'s
`worktree::a_fresh_tree_whose_carry_failed_is_rolled_back_with_its_links_unlinked` and
`worktree::a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back` (§8d-ii.8).

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
   is what a run tree checks out. *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* On the existing-branch arm the commit read is the run branch's tip, which
   is what that tree checks out (D21-89 ruling 1); `<fork>` in this section means the commit checked out.
   *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii; refuting round 2.)* On that arm the review's reference tree
   checks out a second commit, `RK_BASE`'s fork, the merge-base (D21-89 ruling 2; `rk-cli`'s `reference_worktree`), so
   read (1) runs over the merge-base too, before the add, and a refusal (a) or (b) there names that commit
   (`worktree::an_existing_run_branch_whose_merge_base_selects_an_ineffective_driver_refuses`). Every V1 refusal now
   names the commit it read. Two residuals, recorded rather than built: the drivers are resolved as the **run** tree
   resolves them (read (3) matches the run tree's gitdir and branch, not `<item>.review-ref`'s detached gitdir), and
   D21-48 X2's post-checkout scan covers the run tree only, never the reference tree. Git 2.55 on
   this box takes `--source`: `git check-attr --source=HEAD filter --
   Cargo.toml` printed `Cargo.toml: filter: unspecified`, rc 0 [measured at `fc80687`].
2. **The drivers, at every scope.** `git config --show-scope --show-origin --includes --get-regexp '^filter\.'` over
   the resolved configuration: `system`, `global`, `local`, `worktree` and `command` scopes, with every `include`
   followed and every `includeIf` whose condition matches.
3. **The run tree's own `includeIf`s.** `--get-regexp '^includeif\.'` at every scope, so that an `includeIf` whose
   condition matches the **run tree** (~~a `gitdir:` pattern that covers `.redkiln/worktrees/`~~) but not the user's
   checkout is read from the file it names and folded into (2). *(dated 2026-09-27, s8d-ii build, P4: measured on git
   2.55.0.windows.3, a `gitdir:` pattern covering `.redkiln/worktrees/` does **not** match a run tree. Git matches
   `gitdir:` against the tree's gitdir, `<common-dir>/worktrees/<item>`, so `gitdir:**/.git/worktrees/**` and
   `onbranch:rk-run/**` are the run-tree conditions. The read matches against those, as §8d-ii.8's P4 note records.)*
   *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* The `<item>` in `<common-dir>/worktrees/<item>` is the admin directory git WILL assign: git names it
   after the tree path's basename and appends a number on a collision (round 1 measured `item`, then `item1`), and
   this repository's legacy tree already holds `worktrees/rk-fix-doctor-test-count`. The read predicts the suffix as
   git's `add_worktree` does, before the add; a directory created between that read and the add is the one case it
   misses, and D21-48 X2's scan is the backstop (§8d-ii.9 finding 27).
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
  --source=<fork> filter` names with a driver, at the fork commit `RK_BASE` records *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii: at the commit
  the tree checks out, which on the existing-branch arm is the tip and not the merge-base `RK_BASE` records; D21-89
  ruling 1)*. For each path it reads the
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
anything. *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* D21-89 ruling 2 adds one more read, `merge-base <start HEAD> <tip>` (`GitOp::MergeBase`), for
the fork a tree created on an existing branch records; it supersedes nothing, and s8a's GitOp pin admits it (its name
matches none of `diff|range|show|revlist|catfile`). D21-48 X2's post-checkout scan adds no git read: it reuses the `check-attr` path set and reads files in the
fresh tree. Its rollback is `worktree remove` without `--force`, already on clause 5's list. `init`'s `info/exclude` append is not a wrapper write either: `rk-cli` makes it, and it is ADR-0107 decision
2's supersession, not clause 5's (§8d-ii.5).

Every git invocation among them runs through `GitOps`. `s8e`'s N7 `git_op` row journals them once it exists, and on
replay none of them runs.

**The worst-case path (§8d-ii.0 item 2).** The record states the bound for an item: `len(root) + 20 + len(item) + 11 +
1 +` the longest path tracked at the fork commit, where the root is the main worktree's directory. It measured
**255** of **259** at `c5d19d7` for a 64-character item, and **215** for `rk-fix-doctor-test-count`. The rule:
creation refuses by name when that bound, **computed on the item being created** *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* (and on the commit the
tree checks out: the tip, when the branch exists; D21-89 ruling 1), exceeds 259 and `core.longpaths` is
not `true`, rather than check out a partial tree (`worktree::creation_refuses_a_path_past_the_platform_limit_by_name`).
*(dated 2026-09-28, s8d-ii build, package R2FIX-8dii; refuting round 2.)* The `.review-ref` term bounds the reference
tree's path, and on the existing-branch arm that tree checks out the merge-base, not the tip. So on that arm the
longest-path term is **the longer of the longest path at the tip and at the merge-base**, and the refusal names the
commit that tracks it (`worktree::an_existing_run_branch_whose_merge_base_is_past_the_path_limit_refuses`). On a first
creation both trees check out start's `HEAD` and the bound is unchanged. Taking the maximum is conservative for the run
tree, whose own path is 11 shorter than the bound.
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

*(dated 2026-09-27, s8d-ii build, package P6-RKCLI-ROOT-MOVE.)* **The three amendments are made**, each dated in
`s7f` §7f.5 (a prose note above the block, and dated comment lines in it): the arm makes `c7f=$(mktemp -d)` in
`cygpath -m` form, `clone="$c7f/c"` by `git clone --quiet --no-local . "$clone"`, runs `init` and `run
rk-fix-doctor-test-count --lane fix` in the clone by the binary's absolute path (`rk_bin`), reads `"$clone/.git/redkiln/runs"`,
and ends with `rm -rf "$c7f"`; the forced-removal and prune lines are deleted, and no comment keeps the forced-removal
line's text (s8e's (10d) greps the whole block for it). `PROGRESS.md`'s 7f row reads *"checkpoint passed `bf12a1c`, re-run
pending"* with a dated note. (7b)'s text guards pass over the amended block (§8d-ii.8's P6 note); 7f was not replayed
here. One consequence for the replay: this repository's `registry.toml` declares `setup_projects` for
`[toolchain.rust]`, so the clone's run installs the Node fixture with `pnpm` at creation (finding 20).

*(dated 2026-09-28, s8d-ii build, package R2FIX-8dii; refuting round 2.)* **Four amendments, not three.** Refuting
round 1 made a fourth, dated in `s7f`'s block (package R1FIX-8dii, §8d-ii.9 finding 27(f)): the catalog-LOAD failure
line removes the throwaway directory (`rm -rf "$c7f"`) before it exits, as every other post-clone failure line already
did. It moves no pass or fail state, and (7b)'s text guards do not read it. `s7f`'s prose note and `PROGRESS.md`'s 7f
row carry the same count by dated note.

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

*(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* **Two further pins** are round 1's `rk-cli` rows for the values `rk-cli` hands `rk-git`,
`run_worktree::a_run_tree_locked_by_a_live_process_refuses_the_next_run_by_name` and
`run_worktree::git_lfs_skip_smudge_in_runs_environment_refuses_creation_by_name`: the wiring existed (P6), so they
were green at their writing, and `R1FIX-PROBE-NONE` and `R1FIX-LFS-NONE` show each can fail.

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
| `rk-cli` | `run_worktree::a_fresh_tree_whose_setup_failed_is_rolled_back_and_the_next_run_recarries` (D21-88 ruling 2; dated 2026-09-27, s8d-ii build, package D2188) | the failed fresh tree stays registered, and the next run reuses it with no carry and no `setup` |
| `rk-git` | `worktree::a_fresh_tree_whose_carry_failed_is_rolled_back_with_its_links_unlinked` (D21-88 rulings 1 and 2; package D2188) | no rollback: `roll_back_fresh` kept the tree |
| `rk-git` | `worktree::a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back` (D21-88 ruling 2; package D2188) | no rollback to observe; `S8D-II-M29` and `S8D-II-M30` show it can fail |
| `rk-git` | `worktree::an_existing_run_branch_whose_tip_selects_an_ineffective_driver_refuses` (D21-89 ruling 1; dated 2026-09-28, package R1FIX-8dii) | the read runs over start's `HEAD`, which selects nothing: the tree is created on the branch holding a pointer file |
| `rk-git` | `worktree::an_existing_run_branch_whose_tip_leaves_a_pointer_is_scanned_refused_and_rolled_back` (D21-89 ruling 1; R1FIX-8dii) | the scan's path set is start's `HEAD`'s: the pointer at the tip is never read |
| `rk-git` | `worktree::an_existing_run_branch_whose_tip_is_past_the_path_limit_refuses` (D21-89 ruling 1; R1FIX-8dii) | the length rule reads start's `HEAD`: a 260 worst case at the tip is admitted |
| `rk-git` | `worktree::a_tree_created_on_an_existing_run_branch_records_the_merge_base_as_its_fork` (D21-89 ruling 2; R1FIX-8dii) | `RK_BASE` records start's moved `HEAD` |
| `rk-git` | `worktree::a_reentry_whose_discard_is_incomplete_releases_its_own_lock_and_keeps_the_tree` (round-1 minor; R1FIX-8dii) | the refusal returns with the run's own lock still held |
| `rk-git` | `worktree::an_include_if_is_matched_against_the_suffixed_gitdir_git_assigns` (round-1 minor; R1FIX-8dii) | read (3) assumes `worktrees/<item>`: it misses the run tree's suffixed gitdir and folds another tree's include |
| `rk-git` | `worktree::an_existing_run_branch_whose_merge_base_is_past_the_path_limit_refuses` (round-2 minor; dated 2026-09-28, package R2FIX-8dii) | the length rule reads the tip only: a 260 worst case at the merge-base, which the reference tree checks out, is admitted |
| `rk-git` | `worktree::an_existing_run_branch_whose_merge_base_selects_an_ineffective_driver_refuses` (round-2 minor; R2FIX-8dii) | V1's read runs over the tip only: a merge-base that selects an undeclared driver is admitted |
| `rk-cli` | `run_worktree::a_run_tree_locked_by_a_live_process_refuses_the_next_run_by_name` (round-1 minor; R1FIX-8dii) | green — a pin; `R1FIX-PROBE-NONE` shows it can fail |
| `rk-cli` | `run_worktree::git_lfs_skip_smudge_in_runs_environment_refuses_creation_by_name` (round-1 minor; R1FIX-8dii) | green — a pin; `R1FIX-LFS-NONE` shows it can fail |
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
*(dated 2026-09-27, s8d-ii build, package D2188.)* **D21-88 ruling 2 adds three rows: 56 names, `rk-git` 44, `rk-containment` 1, `rk-cli` 11, and 53 new**
~~(`a_failed_setup_refuses_by_name` is amended, not new)~~ *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* (the renamed
`a_tree_without_rk_base_is_refused_and_never_removed` is not new in the pass count; `a_failed_setup_refuses_by_name`
was amended by D2188 and is new to the stage: round 1 measured `git diff 83cf836` at 53 added `#[test]` lines and
none removed, 56 names less 2 witnesses less 1 rename), so the suite gains **53** passed over Checkpoint 8d's
recorded 971: `FLOOR=1024`.
*(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* **Round 1's fixes add eight rows, all new: 64 names, `rk-git` 50, `rk-containment` 1, `rk-cli` 13, and
61 new**, so the suite gains **61** passed over Checkpoint 8d's recorded 971: `FLOOR=1032`.
*(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)* **Round 2's fixes add two rows, both new: 66 names, `rk-git` 52, `rk-containment` 1, `rk-cli` 13, and 63
new**, so the suite gains **63** passed over Checkpoint 8d's recorded 971: `FLOOR=1034`. The merge-base row's fixture
is strengthened, not added (`run_branch_ahead`, two commits a side).

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
| S8D-II-M27 | `run` skips the rollback after a failed carry or `setup` (dated 2026-09-27, package D2188) | `run_worktree::a_fresh_tree_whose_setup_failed_is_rolled_back_and_the_next_run_recarries`, `run_worktree::a_failed_setup_refuses_by_name` | new (D21-88) |
| S8D-II-M28 | the rollback skips the pre-unlink | `worktree::a_fresh_tree_whose_carry_failed_is_rolled_back_with_its_links_unlinked` | new (D21-88) |
| S8D-II-M29 | the rollback's `Reused` guard dropped | `worktree::a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back` | new (D21-88) |
| S8D-II-M30 | the rollback's own-lock guard dropped | `worktree::a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back` | new (D21-88) |
| R1FIX-PB-SUFFIX | `path_bound` drops `.review-ref` + 1 (dated 2026-09-28, package R1FIX-8dii) | `worktree::creation_refuses_a_path_past_the_platform_limit_by_name` | new (round 1; SURVIVED the fixture sized from `path_bound`) |
| R1FIX-PB-PLUS1 | `path_bound` drops the `+ 1` | `worktree::creation_refuses_a_path_past_the_platform_limit_by_name` | new (round 1; SURVIVED likewise) |
| R1FIX-D2189-HEAD | the existing-branch arm checks start's `HEAD`, not the tip | the three D21-89 ruling-1 rows | new (D21-89) |
| R1FIX-D2189-BASE | `RK_BASE` on the existing-branch arm records start's `HEAD` | `worktree::a_tree_created_on_an_existing_run_branch_records_the_merge_base_as_its_fork` | new (D21-89) |
| R1FIX-DISCARD-LOCK | the incomplete-discard refusal keeps the run's own lock | `worktree::a_reentry_whose_discard_is_incomplete_releases_its_own_lock_and_keeps_the_tree` | new (round 1) |
| R1FIX-GITDIR-SUFFIX | read (3) matches `gitdir:` against `worktrees/<item>` | `worktree::an_include_if_is_matched_against_the_suffixed_gitdir_git_assigns` (both directions, `-OTHER`) | new (round 1) |
| R1FIX-PROBE-NONE | `run`'s liveness probe reads nothing (every lock stale) | `run_worktree::a_run_tree_locked_by_a_live_process_refuses_the_next_run_by_name` | new (round 1) |
| R1FIX-LFS-NONE | `run` hands `rk-git` no `GIT_LFS_SKIP_SMUDGE` | `run_worktree::git_lfs_skip_smudge_in_runs_environment_refuses_creation_by_name` | new (round 1) |
| R2FIX-MB-TIPPARENT | `RK_BASE` on the existing-branch arm records `tip^` (dated 2026-09-28, package R2FIX-8dii) | `worktree::a_tree_created_on_an_existing_run_branch_records_the_merge_base_as_its_fork` | new (round 2; SURVIVED the one-commit-a-side fixture) |
| R2FIX-MB-HEADPARENT | `RK_BASE` on the existing-branch arm records `HEAD^` | `worktree::a_tree_created_on_an_existing_run_branch_records_the_merge_base_as_its_fork` | new (round 2; SURVIVED likewise) |
| R2FIX-LEN-TIPONLY | the length rule reads the tip only, not the merge-base | `worktree::an_existing_run_branch_whose_merge_base_is_past_the_path_limit_refuses` | new (round 2) |
| R2FIX-FILTER-TIPONLY | V1's read skips the merge-base | `worktree::an_existing_run_branch_whose_merge_base_selects_an_ineffective_driver_refuses` | new (round 2) |

A mutant that survives is not waved through. It is either an **unobservable property**, which moves to a source lint
with its reason written down, or a **missing fixture branch**. Both are recorded by the mutant's name.

*(dated 2026-09-27, s8d-ii build, package P2-RKGIT-IDENTITY.)* **§8d-ii.1's derivation, §8d-ii.2's validator and guards,
§8d-ii.6's read variants and length rule, and the ignore refusal are built in `rk_git::worktree`; ten of the table's
rows are green, each seen red first.** What exists for the later packages to call: `ItemId::parse` (the validator;
`ItemIdRefusal` names each rule), `derive(main, &item)` returning `RunTree { item, path, reference, branch }` (the ONE
derivation: `<main>/.redkiln/worktrees/<item>`, `<item>.review-ref` beside it, `rk-run/<item>`) *(dated 2026-09-29,
`s8g` build, package S8G-P3-KERNEL-GIT: `RunTree` gains `mutation`, `<item>.mut` beside the reference tree, and
`ItemIdRefusal` gains `MutSuffix`, `s8g` §8g.1 move 3)*, `main_worktree`
(the first `worktree list --porcelain` record; `bare` refused by name), `registered_worktrees`,
`refuse_linked_segments`, `refuse_case_fold_collision`, `refuse_unignored_root`, `path_bound` and `refuse_past_limit`,
`verify_created` (compares canonical paths only; nothing canonical reaches git), and `prepare_creation(exec, start,
item, fork)`, which runs the guards in that order and returns the derivation. Every refusal is
`GitError::WorktreeRefused { refusal: WorktreeRefusal }`, whose `Display` names the path or id and the rule. `GitOp`
gains three reads, `WorktreeList`, `LsTree` and `Config` (`RevParse` and `CheckIgnore` already existed and carry
`rev-parse --git-common-dir` and the ignore read); none is a write, and `s8a`'s GitOp pin passes (added since
`S8A_BASE`: `CheckIgnore Config LsFiles LsTree WorktreeList`, none named for a range read). `git_exec.rs`'s private
`spawn` sets `cmd.env("LC_ALL", "C")` **after** the caller's `env`, so no caller can undo it. Nothing calls
`prepare_creation` or `derive` from `rk-cli` yet: `run.rs`'s four derivations and the six test sites move in the
root-move package, and (3)'s root floor still reads the same 12 lines it read at entry.
- **RED on the entry-shaped stub** (the API present, the entry behaviour inside it), each for its row's reason:
  `an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused` *"24 of 24 rows were not refused for their own
  reason (no validator?)"*; `an_item_id_that_case_folds_onto_a_registered_tree_is_refused` *"fix-1 beside a registered
  Fix-1: ADMITTED"*; `the_root_is_the_main_worktree_when_run_from_a_linked_worktree` *"read from the linked worktree,
  the root is `…/rootmain/.git`, not the main worktree"* (the common-dir root); `a_bare_repository_is_refused_by_name`
  *"the main worktree of a bare repository: ADMITTED"*; `a_symlinked_redkiln_or_worktrees_directory_is_refused` *"a
  junction at .redkiln: ADMITTED"*; `a_created_worktree_whose_realpath_leaves_the_root_is_refused` *"a tree resolving
  outside the root: ADMITTED (())"*; `creation_refuses_when_the_worktree_root_is_not_ignored` *"creation under an
  unignored root: ADMITTED"*; `creation_refuses_a_path_past_the_platform_limit_by_name` *"a 260-character worst case:
  ADMITTED"*; `gitops::every_git_child_runs_with_lc_all_c` *"5 of 5 routes ran git without LC_ALL=C (W1)"* (the child
  saw `LC_ALL=[]`). `the_length_rule_reads_the_actual_item_length` was **green** on the stub, as its row requires.
- **GREEN:** the ten selected module-qualified with `--exact`, *"test result: ok. 10 passed; 0 failed; 0 ignored; 0
  measured; 78 filtered out"*. The symlink test built a **junction** at each of `.redkiln`, `.redkiln/worktrees` and
  `.redkiln/worktrees/item-1` (it prints which), and the refusal reads *"is a symlink or junction"*.
- **Mutants**, from `p2rkgit_mutants.py` (anchor asserted once, tmp + `os.replace`, restored in `finally`, sha256
  verified), each killed by its named test for its named reason: `S8D-II-M7` (the `LC_ALL` line removed) *"5 of 5
  routes ran git without LC_ALL=C"*; `S8D-II-M11` (the ignore check skipped) *"creation under an unignored root:
  ADMITTED"*; `S8D-II-M17` (the root read by `--show-toplevel`) *"read from the linked worktree, the root is
  `…/rootmain.swt`"*; `S8D-II-M18` (the bound on `ITEM_ID_MAX`) *"a 1-character item was refused on a 64-character
  bound: … bound: 260"*. Each reading: rc 101, *"0 passed; 1 failed"*.
- **The package brief said "12 tests"; its own list names ten**, the ten above. The other rows whose subject is W1
  (`RK_BASE`, the lock, re-entry, `removal_never_follows_a_junction`) are the lifecycle package's (§8d-ii.3).
- **Design choices a later package may revisit, recorded rather than hidden:** the length rule's lengths are UTF-16
  code units (what `MAX_PATH` counts) and the longest path is read from `ls-tree -r -z --name-only <fork>`;
  `core.longpaths` is read with `config --type=bool --get`, exit 1 meaning *not set*; the case-fold rule reads
  branches with `rev-parse --symbolic-full-name --branches=rk-run/*` and strips a `.review-ref` suffix from registered
  names; `.lock` and `.review-ref` are case-folded (a Windows loose ref is a file); the linked-segment check reports
  *"symlink or junction"* on Windows, because `is_symlink` is true for both there. The tests pin `core.excludesFile`
  and `core.longpaths` locally, because the production reads run in the inherited environment and a box whose
  system config sets `core.longpaths=true` would otherwise admit the refusal fixture.

*(dated 2026-09-27, s8d-ii build, package P3-RKGIT-LIFECYCLE.)* **§8d-ii.2's marker, lock, re-entry check and
junction rule, §8d-ii.3's lifecycle and §8d-ii.6's four writes are built in `rk_git::worktree`, and the start-time
half of the liveness predicate in `rk-containment`; the table's eighteen lifecycle rows and its `rk-containment` row
are green, each seen red first.** What exists for the later packages to call:
- `enter(exec, start, item, &LockOwner { pid, start }, &probe)` returns `Entered { tree, fork, created, discarded }`,
  with `created` one of `Creation::{New, OnExistingBranch, Reused}` and `discarded` every discarded path. `release(…)`
  unlocks only the run's own lock. `remove_landed(exec, start, item)` returns `Removal::Removed` or
  `Removal::Kept { path, reason }`; `KeepReason`'s `Dirty` displays Claude Code's *"N changed file(s) would be
  lost"*. `read_rk_base`, `write_rk_base` and `lock_reason` are public.
- **The predicate is the `ProcessProbe` trait** (`start_time(pid) -> Option<u64>`, implemented for any `Fn(u32) ->
  Option<u64>`). A lock is live only when it returns exactly the recorded start. `rk-git`'s manifest is unchanged: it
  has no edge to `rk-containment`. `rk-cli` has not yet built the predicate from `is_alive` and `process_start_time`;
  that is the wiring package's.
- New refusals (each keeps the tree): `NoRkBase`, `RkBaseMismatch`, `ForeignRepository`, `CommitsRunRefLacks`,
  `DetachedHead`, `OffRunRef`, `LockHeld`, `ForeignLock` and `DiscardIncomplete`.
- `GitOp` gains the four writes, `WorktreeLock`, `WorktreeUnlock`, `ResetHard` and `Clean`, each with `is_write`
  true. `s8a`'s GitOp pin passes and its three controls fire. It lists as added since `S8A_BASE`: `CheckIgnore Clean
  Config LsFiles LsTree ResetHard WorktreeList WorktreeLock WorktreeUnlock`.
- `rk_containment::process_start_time(pid) -> Option<u64>` is public and not `#[doc(hidden)]`: it is a production
  read. Its Windows arm reads `GetProcessTimes` through `ffi::win`, and returns nothing unless `GetExitCodeProcess`
  reads `STILL_ACTIVE`. Its POSIX arm reads `/proc/<pid>/stat` field 22, returns nothing for a zombie, and is ARMED to
  CI. `is_alive`'s `#[doc(hidden)]` is left as it stands until the `rk-cli` call that makes it production.

**RED, on an entry-shaped stub** (the API present; the entry behaviour inside it: an existing path refused as
ADR-0105 clause 6 re-created it, no marker, no lock, no discard, removal by a bare `git worktree remove`). Each test
failed for its row's reason:

| test | the red line |
| --- | --- |
| `creation_writes_rk_base_into_the_worktree_gitdir` | *"no RK_BASE in the tree's gitdir `…/base/.git/worktrees/item-1` … — W1's ownership marker is absent"* |
| `creation_locks_the_worktree_with_the_pid_and_start_reason` | *"the created tree is not locked with the pid and start reason (no lock?)"*, `left: None` |
| `a_lock_held_by_a_dead_pid_is_released` | *"a dead pid's lock was not released: WorktreeRemoveFailed { … }"* (clause 6's refusal) |
| `a_lock_whose_pid_now_names_another_process_is_released` | as the dead-pid row |
| `a_lock_held_by_a_live_pid_refuses_by_name` | *"refused, but not by a WorktreeRefused: WorktreeRemoveFailed"* |
| `reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip` | *"marker: … NotFound"* (no marker, then no reuse) |
| `reentry_discards_in_flight_changes_and_reports_each_path` | *"reused: WorktreeRemoveFailed"* (re-created per run: no reuse, no discard) |
| `reentry_keeps_ignored_files` | as the discard row |
| `a_tree_without_rk_base_is_refused_and_never_removed` | *"refused, but not by a WorktreeRefused"* (no marker check) |
| `a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed` | as the no-`RK_BASE` row |
| `a_tree_registered_to_another_repository_is_refused` | as the no-`RK_BASE` row |
| `a_tree_on_a_detached_head_is_refused_at_reentry_and_kept` | as the no-`RK_BASE` row |
| `an_existing_run_branch_is_never_reset` | *"no reuse path for an existing run branch: NonZeroExit { op: "worktree add", … }"* |
| `a_dirty_tree_is_kept_with_its_reason` | *"removal decision: WorktreeRemoveFailed"* (git's own refusal; no removal rule) |
| `an_unlanded_tree_is_kept_with_its_reason` | *"an UNLANDED tree (its tip carries no closed mark) was removed"* |
| `a_locked_tree_is_not_removed` | *"a LOCKED tree was removed"* |
| `removal_never_follows_a_junction` | *"a skeleton of the removed tree is left on disk (s8c §8c.12 item 3)"* |
| `a_clean_landed_marked_unlocked_tree_is_removed` | *"a landed tree was kept: Kept { …, reason: Unmarked }"* |
| `containment::process_start_time_reads_a_live_child_and_nothing_for_an_exited_one` | *"process_start_time read nothing for the LIVE child … (no start-time read?)"* |

The positive removal row passed on the bare-removal stub, as a positive control must. So it was read red on the
stub's other entry form, the one where the run tree is never removed (`recreate_run_worktree` recreates and never
removes). That reading is the one in the table.

**GREEN:** `rk-git` 19 selected module-qualified with `--exact`, the table's 18 plus the re-pointed
`control_a_worktree_can_be_created_and_removed`: *"test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 86
filtered out"*. `rk-containment`: *"test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out"*,
reading *"MEASURED process_start_time: child 33280 -> Some(134350387165104729), again Some(134350387165104729)"*. The
zero-match control reads rc 0 and *"0 passed"*.

The renamed test keeps its refusal: `worktree::add`'s clause-6 refusal of an existing path still holds for
`redkiln-migrate`'s worktree (AC-522), and is asserted at the renamed test's end.

**Mutants**, from `p3lifecycle_mutants.py` (anchor asserted once, tmp + `os.replace`, restored in `finally`, sha256
`58fa5a242694…` verified after each). Each read rc 101, *"0 passed; 1 failed"*, and was killed by its named test for
its named reason:

| mutant | killed with |
| --- | --- |
| `S8D-II-M2` (rk-git half: a registered tree force-removed and re-created) | *"the registered tree was re-created, not reused: OnExistingBranch"* |
| `S8D-II-M3` (the unmarked tree removed) | *"after the refused re-entry: the tree … was removed from disk (refused trees are never removed)"* |
| `S8D-II-M4` | *"the discard did not report each path"* |
| `S8D-II-M8` | *"a dead pid's lock was not released: … LockHeld { …, pid: 111, start: 111000 }"* |
| `S8D-II-M10` (`clean -fdx`) | *"an ignored carried file did not survive re-entry"* |
| `S8D-II-M16` (the `closed`-mark condition dropped) | *"an UNLANDED tree (its tip carries no closed mark) was removed"* |
| `S8D-II-M20` (pid alone) | *"a lock whose pid now names another process was not released: … LockHeld"* |

Two synthetic controls on the junction test, from `p3lifecycle_junction_controls.py`, went red:
- **J1**, the link followed before it is removed: *"the removal FOLLOWED the junction and deleted the sentinel's
  contents outside the tree"*.
- **J2**, no link unlinked before git's removal: *"removal decision: WorktreeRemoveFailed { … }"*, whose stderr bytes
  read *"git reported the removal done and a residue of the tree is still on disk"*. That is the product's own
  residue check, which fires before the test's skeleton assertion.

**`s8c` §8c.12 item 3, measured** (a temporary probe test, never committed; the driver restored `main.rs` to sha256
`eb599193…` and deleted the probe). A fixture repository carried the Node fixture's `package.json` and
`pnpm-lock.yaml`, with `node_modules/` ignored. A run tree entered through `enter` then had `pnpm install
--frozen-lockfile --offline` run in it, which gave 114 links, 1084 files and 273 directories under `node_modules`.
The lane was closed, the lock released, and the status was empty.
- **git's own unforced `git worktree remove`:** exit 0 and deregistered, but **114 dangling junctions, 0 files and 60
  directories left on disk**. That is s8c's skeleton, reproduced without `--force`.
- **`remove_landed`,** which unlinks every untracked link first and never walks through one: `Removed`, nothing on
  disk, deregistered.

The raw removal leaves dangling junctions, so under the package brief this is reported as a blocker (§8d-ii.9). The
built behaviour is ADR-0180 decision 9's: *"a link inside the tree is removed, never its target"*.

*An earlier reading in this session is retracted, because it was a proxy.* A PowerShell probe appeared to show git
following a junction out of the tree and deleting a sentinel's file. Its `Set-Content` wrote a file named `precious`
into the working directory instead of the sentinel, so the sentinel was empty from the start. The stray file was
deleted. Git Bash probes with a populated sentinel, and the Rust fixture, show git **not** following the junction,
only leaving it behind.

**Guards, read-only lines.**
- **Checkpoint 8d-ii (3):**
  - the force floor over `worktree.rs` reads 0 code-line hits, and a planted `"--force"` line is kept;
  - `RK_BASE` is on a code line (`pub const RK_BASE`);
  - no `"--local"`;
  - the `LC_ALL` call form is present;
  - the root floor reads the same 12 lines outside OWN that it read at entry (`run.rs` 4, `rk-cli` tests 6,
    `worktree_include` 2), and none is added.
- **Checkpoint 7c's rk-containment lines** (`p3lifecycle_cp7c.sh`) pass:
  - exactly one `#![allow(unsafe_code)]`, commented, in `ffi.rs`;
  - no `unsafe` token outside it, with a planted control kept;
  - no `#[allow(`, no tokio token, and no `rk-*` or tokio row in the manifest;
  - the five `windows-sys` features unchanged (`GetProcessTimes` and `FILETIME` need none new);
  - `is_alive` and `terminate_pid` still `#[doc(hidden)]`, and `process_start_time` not hidden.

  **The admission:** `ffi.rs`'s `unsafe` code-line sites go from 23 to 25 (the new function's `GetExitCodeProcess`
  and `GetProcessTimes` calls). `s7c` §7c.20 item 2's site list (*"spawn, assign, resume, kill, enumerate,
  `terminate_pid`, `is_alive`"*) gains `process_start_time`. `ffi.rs`'s site table gains its row, and ADR-0180
  decision 6 now says the call goes through the binding layer ADR-0127 decision 5 names, citing ADR-0127 without
  superseding it. `s7c`'s text is not edited.
- **Checkpoint 6:** `Command::new("git")` is on git_exec.rs code lines (3 hits) and on 13 lines outside it, unchanged
  from HEAD. The workspace `process::Command` grep still fails as it did at entry. The rk-containment files it names
  hold the same count as at HEAD (`ffi.rs` 2, `posix.rs` 1, `windows.rs` 2), so none of the hits is this package's.
- **Other checks:** (9)'s record lines pass (`P1REC-(9)-RECORD-LINES-OK`). check-adr-options (17 records, 0
  flagged), check-refs (0), check-bash-syntax (0) and check-checkpoints (0) all pass.

**Suite:** fmt passes, and clippy on `rk-git` and `rk-containment` (all targets) exits 0, with no unfulfilled
expectation and no warning on this package's lines. The whole suite through the helper reads rc 0, **999 passed / 0
failed / 10 ignored over 44 result lines**: 981 plus 18 net new (19 names, one of them the rename). The legacy ref is
at `73c087a…` with its mark and registration, the worktree path set, the store's sha256 (`e83d295e…`) and the
absent `.redkiln/worktrees` are all unchanged.

**Design choices a later package or the operator may revisit, recorded rather than hidden:**
- **Re-entry order.** Re-entry checks `RK_BASE` first, then the repository, then commits the run ref lacks, a
  detached `HEAD` and another ref. It **takes the lock before the discard**, so a live run's in-flight work is never
  discarded under its lock (`a_lock_held_by_a_live_pid_refuses_by_name` asserts the file survives).
- ~~**The fork on an existing branch.**~~ *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* *Superseded by D21-89 ruling 2 (`RK_BASE` records
  `merge-base(start HEAD, run-branch tip)`); the struck choice follows.* ~~When the branch exists and no tree is registered, the fork `RK_BASE` records is
  `start`'s `HEAD` at that moment: the old tree's `RK_BASE` went with its gitdir, and no `merge-base` is read. The
  review reference tree's base (§8d-ii.3) is therefore that `HEAD` for such a tree, the pre-stage behaviour. §8d-ii.3
  does not settle this, and it is named here for the reference-tree package.~~
- **Removal order.** Removal checks: registered (`KeepReason::NotRegistered`, added), marked, unlocked, symbolic on the
  run ref, clean (the dirty count is status entries), and the `closed` mark (`<project>/<item> closed`, exactly two
  tokens).
- **Links before git's removal.** Before git's removal it unlinks every **untracked** symlink or junction with
  `remove_dir` or `remove_file`, never `remove_dir_all`, and leaves a tracked link to git. If git refuses, or a residue
  is left, the result is `GitError::WorktreeRemoveFailed` (Halt 6's typed error), never a retry and never a force.
- **The fork read.** The creation fork is read as `rev-parse --verify HEAD^{commit}` in `start`, and `git worktree
  add` receives the sha, not `HEAD`, so `RK_BASE` and the branch cannot disagree.

*(dated 2026-09-27, s8d-ii build, package P4-RKGIT-FILTER-SCAN.)* **§8d-ii.5's filter refusal (D21-47 V1) and D21-48
X2's post-checkout pointer scan are built in `rk_git::worktree`, with §8d-ii.6's `check-attr` read. The table's
fourteen filter and scan rows are green.** The package brief says "13 tests", but its own list names fourteen: nine
refusing rows and five controls. With P2's ten and P3's eighteen they make the 42 that (2)'s `sel_tests rk-git 42`
names.

**What exists for the later packages to call:**
- `enter(exec, start, item, owner, probe, lfs_skip_smudge)` gains its last parameter. It is the value of
  `GIT_LFS_SKIP_SMUDGE`, which `rk-cli` reads through `env.rs` and passes in (P6); this crate reads no environment.
  - Every existing call site passes `None`: P3's twenty test sites, rewritten mechanically.
  - `Entered` gains `filter_notes`, the admission facts for the **debug** level only. It is empty on re-entry.
- `check_filters(exec, main, &item, fork, lfs_skip_smudge) -> FilterCheck { selected, notes }` makes the reads and
  takes the refusal. Each `SelectedDriver` has `driver`, `paths` and `effective`.
- `scan_for_pointers(tree, &check)` returns `(path, driver)` for each hit.
- `LFS_POINTER_HEADER` holds `version https://git-lfs.github.com/spec/v1` on a code line.
- There are three new refusals, and each `Display` names the driver, a selecting path, and the scope and origin:
  - `FilterDriverNotEffective { driver, path, scopes }`;
  - `SmudgeSkipped { driver, path, scope, origin }`, where the environment arm reads scope `environment` and origin
    `GIT_LFS_SKIP_SMUDGE=<v>`;
  - `PointerFilesAfterCheckout(Box<PointerHits>)`, boxed so that `GitError` stays under clippy's `result_large_err`.
- `GitOp::CheckAttr` is a read, and s8a's GitOp pin passes. The variants added since `S8A_BASE` now read `CheckAttr
  CheckIgnore Clean Config LsFiles LsTree ResetHard WorktreeList WorktreeLock WorktreeUnlock`, and none is named for
  a range read.
- `GitExec::with_env(key, value)` sets extra environment on every child before `LC_ALL`, so it cannot undo `LC_ALL`.
  - It is empty in production. The filter tests use it to point `GIT_CONFIG_SYSTEM` and `GIT_CONFIG_GLOBAL` at
    fixture files and to set `GIT_CONFIG_NOSYSTEM=0`.
  - `run_labelled` was folded into `run`, and `spawn` stays the one `Command::new("git")` site.

**The order in `create`:**
1. the fork;
2. `prepare_creation`;
3. `check_filters`, which refuses before the add;
4. `git worktree add`;
5. `verify_created`;
6. `RK_BASE`;
7. the scan;
8. the lock.

**The reads:**
1. `ls-tree -r -z --name-only <fork>`, then `check-attr -z --source=<fork> --stdin filter` over those paths, fed on
   stdin. The values `unspecified`, `unset` and a bare `set` name no driver.
2. `config -z --show-scope --show-origin --includes --get-regexp '^(filter|includeif)\.'` in the main worktree. This
   is a single read, so a folded file's values sit in git's order.
3. Each `includeif.<condition>.path` whose condition matches the run tree is read with `config -z --show-origin
   --includes --file <f> --get-regexp '^filter\.'`. Its values are folded in at the include's position, carrying the
   includer's scope and the included file as origin.

**When a driver is effective, and when creation refuses:**
- A driver is effective when a non-empty `process` value resolves, or else a non-empty `smudge` value. The resolved
  value is the last one read.
- The refusals are checked in this order:
  - (a), for each selected driver;
  - `GIT_LFS_SKIP_SMUDGE` set to anything but `0`. An empty value reads as unset, as Checkpoint (0c)'s
    `${GIT_LFS_SKIP_SMUDGE:-0}` does;
  - a resolved `smudge` carrying a `--skip` token.
- (b) is taken literally: the environment arm applies to any selected driver, not only to one named `lfs`.

**The scan:**
- It reads every path that (1) found selecting a driver, in the fresh tree, and skips anything that is not a regular
  file.
- It reads at most the header's length plus 2 bytes, and a hit is a first line that equals the header once one CR is
  dropped.
- On a hit, `git worktree remove` runs without `--force`. The result is `removed: true` only when git exits 0 and
  nothing is left on disk. Otherwise the tree is kept, with git's reason.
- The branch is never touched.
- If a selected file cannot be read, the scan takes the same unforced removal and then returns
  `SegmentUnreadable`, so a fresh tree it could not judge is never locked and kept.

**A finding about the spec's own example, measured and settled here (git 2.55.0.windows.3; probe under
`scratchpad/s8d-ii/p4filter-probe1`).**
- **The measurement:**
  - Read (3)'s example, *"a `gitdir:` pattern that covers `.redkiln/worktrees/`"*, does **not** match a run tree. Git
    matches `gitdir:` against the tree's **gitdir**, `<common-dir>/worktrees/<item>`, not against its checkout path.
  - Inside `.redkiln/worktrees/item`:
    - `includeIf "gitdir:**/.redkiln/worktrees/**"` was **not** followed;
    - `includeIf "gitdir:**/.git/worktrees/**"` was followed;
    - `includeIf "onbranch:rk-run/**"` was followed.
  - `rev-parse --git-dir` printed `…/main/.git/worktrees/item`.
- **What the code does:** it matches `gitdir:` and `gitdir/i:` against that gitdir, trying the canonical form first
  as git does, and `onbranch:` against `rk-run/<item>`. It follows git's `prepare_include_condition_pattern`:
  - `./` becomes the including file's directory;
  - a relative pattern gets `**/`;
  - a trailing `/` gets `**`.

  The matcher is `wildmatch` with `WM_PATHNAME`.
- **Conditions left unevaluated:** a `~` pattern (this crate reads no environment, ADR-0083), `hasconfig:`, and
  anything unknown. Each goes to the debug notes, and X2's scan is the backstop, as the P4-X mutants below show.
- **Relative origins:** git prints a repository's own files relative to the checkout (`file:.git/config`), so a
  relative origin is joined to the main worktree and never to the process's current directory (AC-529).
- **Includes git already follows:** an include that also matches the user's checkout is folded a second time, next
  to git's own expansion. That leaves the last-wins result unchanged.
- **A condition that matches only the user's checkout:** its include, which git's `--includes` followed, is not
  removed. That is recorded, not built.

**RED on an entry-shaped stub** (the API present; `check_filters` read nothing and `scan_for_pointers` scanned
nothing): nine rows went red, each for its row's reason. The fork test read *"the fork commit selects `nodrv`; the
working tree selects nothing: ADMITTED"*, and the no-driver, local, global, system, environment and `includeIf` rows
read *"…: ADMITTED (Entered { … filter_notes: [] })"* in the same form. Two rows need more:
- **`a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree`** went red on the
  stub at its precondition, *"the read does not see `fx` as selected and effective: FilterCheck { selected: [], notes:
  [] }"*. Its row's own reason (the read admits, and the tree keeps its pointers) was read under `S8D-II-M22`: *"a
  pointer file at a selected path after checkout: ADMITTED"*.
- **`the_pointer_scan_never_removes_a_tree_this_call_did_not_create`** was **green** on the stub. P3's reuse exists,
  so the stub cannot produce its row's reason (*"re-created by force; no reuse to observe"*). It was read red on the
  stage-entry form: `ENTRY-FORM`, P3's M2 shape, which force-removes and re-creates. That reading is *"re-entry was
  refused: WorktreeRefused { refusal: PointerFilesAfterCheckout(PointerHits { … first: "a.bin", scope: "system" …"*,
  because the re-created tree was scanned and refused. The row was also read red under `S8D-II-M25`.

The five controls were green on the stub, as their rows require. The BrittonGames row first carried an assertion on
the debug notes, and it went red on the stub for that alone. The assertion moved to the environment row's `0` arm,
which is red at entry anyway.

**GREEN:** the fourteen, selected module-qualified with `--exact`: *"test result: ok. 14 passed; 0 failed; 0 ignored;
0 measured; 105 filtered out"*. `includeIf` is exercised by three cases:
- a `gitdir:` include at `system` scope;
- an `onbranch:` include at `system` scope;
- an `onbranch:` include at `local` scope, with a relative path.

For each case the test first proves that the user's checkout reads the working `smudge`, so the include matches the
run tree only. **(2)'s rk-git selection**, run verbatim from this file with its zero-match control, passes: rc 0, the
42 ok lines, and *"42 passed; 0 failed; 0 ignored"*.

**Mutants**, from `p4filter_mutants.py` (every anchor asserted once, tmp plus `os.replace`, restored in `finally`, and
the sha256 `7733e96d149a…` verified after each). Each read rc 101 and *"0 passed; N failed"*, and each named test's
own stdout section carries the reason:

| mutant | killed by, and the reason read |
| --- | --- |
| `S8D-II-M5` (`--local` added to the driver read) | the `system` and `global` skip rows: *"not the skipped-smudge refusal at system scope: FilterDriverNotEffective { driver: "fx" …"*; the BrittonGames control: *"creation was REFUSED … FilterDriverNotEffective { driver: "lfs" …"* |
| `S8D-II-M12` (refuse any selected driver) | the BrittonGames control, as M5's |
| `S8D-II-M13` (refuse any declared driver) | the declared-but-unselected control and the BrittonGames control: *"creation was REFUSED … driver: "mutant""* |
| `S8D-II-M14` (the empty case refused) | the empty control: *"no filter anywhere: creation was REFUSED"* |
| `S8D-II-M15` (`check-attr` without `--source`) | the fork row: *"the working tree selects nothing: ADMITTED"* |
| `S8D-II-M21` (`GIT_LFS_SKIP_SMUDGE` ignored) | the environment row: *"GIT_LFS_SKIP_SMUDGE=1: ADMITTED"* |
| `S8D-II-M22` (scan skipped) | the pointer row: *"a pointer file at a selected path after checkout: ADMITTED"* |
| `S8D-II-M23` (scan walks every file) | the unselected-header control: *"PointerFilesAfterCheckout(PointerHits { … first: "docs/lfs-spec.txt""* |
| `S8D-II-M24` (scan reads the fork blob) | the smudged control: *"a committed pointer the driver smudges: creation was REFUSED … PointerFilesAfterCheckout"* |
| `S8D-II-M25` (scan and rollback at re-entry) | the never-removes row: *"re-entry was refused: … PointerFilesAfterCheckout"* |
| `S8D-II-M26` (no removal on refusal) | the pointer row's absence assertion: *"the fresh tree `…item-1` is still on disk after the refusal"* |
| `P4-X-FOLD` (a matched run-tree include never folded) | the `includeIf` row: *"`gitdir:**/.git/worktrees/**`: not the skipped-smudge refusal: PointerFilesAfterCheckout"*. The scan caught what the read missed |
| `P4-X-GITDIR` (the gitdir condition matched against the common dir) | as `P4-X-FOLD` |
| `P4-X-ORIGIN` (a relative origin resolved against the process's current directory) | the `includeIf` row's `local` case: *"`onbranch:rk-run/**`: not the skipped-smudge refusal: PointerFilesAfterCheckout(PointerHits { tree: "…inclocal…"* |

**Guards (read-only lines):**
- **Checkpoint 8d-ii (3)'s R8/V1 and X2 floors**, extracted verbatim, pass (`--show-scope` and `--source` on code
  lines, no `"--local"`, the header on a code line). Each has a synthetic control that fires against a planted copy:
  `--show-scope` removed, `--source` renamed, `"--local"` added, and the header commented out.
- **The rest of (3):**
  - the force floor over `worktree.rs` reads 0;
  - the `LC_ALL` call form is still on a code line, after `.envs(env)`;
  - the root floor reads the same 12 lines outside the worktree module as at entry (`run.rs` 4, `rk-cli` tests 6,
    `worktree_include` 2), and none is added.
- **Checkpoint 6's `git_exec.rs` lines** hold: `Command::new("git")`, `hooksPath` and `ifexists` are each at least 1.
- **s8a's GitOp pin** passes, and its three controls fire.

**Suite:** fmt passes, and clippy on `rk-git` (all targets) exits 0. None of this package's lines is among its
warnings: the remaining `worktree.rs` hits are P2's and P3's nursery doc-paragraph lines, and the two test-file
redundant clones at lines 128 and 129 predate this stage. The whole suite through the helper reads rc 0, **1013 passed
/ 0 failed / 10 ignored over 44 result lines**, which is 999 plus the 14. At the end:
- `rk-run/rk-fix-doctor-test-count` is at `73c087a82245…`, with its `Story:` trailer and its registration;
- the worktree path set is unchanged: the main checkout, the legacy tree and the operator's `redkiln-rs.swt`, which
  dates from 2026-09-06;
- the store's sha256 is unchanged (`e83d295e…`, backed up to `target/store-backups/p4filter-before-suite.sqlite3`);
- `.redkiln/worktrees` is still absent.

**Left for later packages, recorded rather than hidden:**
- `rk-cli` does not yet call `enter`, so nothing in production reads `GIT_LFS_SKIP_SMUDGE`. The `env.rs` read and the
  debug-level printing of `filter_notes` are P6's.
- Checkpoint (0c) and (0d) run against this repository, and a checkpoint package runs them.

*(dated 2026-09-27, s8d-ii build, package P5-INIT-EXCLUDE.)* **§8d-ii.5's `info/exclude` write (D21-45 W4) is built
in `init`, and the two `init` rows are green.**

**What was built:**
- **`rk_git::worktree::RUN_ROOT_IGNORE_ROW`** is `/.redkiln/worktrees/`, declared beside `derive`. A `const`
  assertion ties its length to the path bound's 20. It lives in the worktree module because (3)'s root floor
  (`"[^"]*redkiln[^"]*worktrees[^"]*"`) flags that literal on any code line outside that module, in `src/` and
  `tests/` alike. So `init` and the tests take the row from there, and P6's shared builders can do the same.
- **`init`** (`crates/rk-cli/src/verbs/init.rs`) gains `EXCLUDE_BEGIN` (`# redkiln-runtime`, on a code line),
  `EXCLUDE_END` and `exclude_block()`. After the scaffold and before the store, it appends the block to
  `repo.common_dir/info/exclude`, creating `info/` and the file when git's template did not.
  - It writes only when no line of the file equals the marker, with a trailing CR dropped. A line that merely starts
    with the marker is the operator's.
  - The write is an append. It is preceded by one `\n` only when the operator's last line is unterminated.
  - `.gitignore` is never written.
  - A write error prints and exits `ConfigError`.
- **The module doc** reads *"It does exactly ~~two~~ three things"*, by a dated edit. *"installs no hook (AC-412)"* is
  intact, and a paragraph says why `CreatedNames` does not change. The test file's module doc takes the same edit.
- **The AC-531 reading note** is in its own block in the register and names ADR-0180. The checkpoint's `acnote.py`,
  run verbatim over AC-531 alone, reads `NOTES-OK`. Its controls both read `MISSING`: HEAD's register, and AC-438,
  the next block.

**RED, then GREEN:**
- **RED**, with the row constant present and `init` unchanged:
  - `init_writes_the_redkiln_runtime_exclude_block_once` read *"init writes no exclude block, or not exactly one
    append of it after the template's bytes"*, and its `left` was git's template alone.
  - `init_leaves_the_operators_exclude_lines_untouched` read *"init writes no exclude block after the operator's
    lines"*, and its `left` was the operator's bytes alone.
  - Totals: *"0 passed; 2 failed"*.
- **GREEN:** the `init::` module read *"7 passed; 0 failed; 0 ignored"*. That is the two new rows plus AC-437's two,
  AC-531's two and AC-412's one.

**How the tests are built:**
- **The first row** starts from the pre-stage initialised shape. `init` runs, then `info/exclude` is put back to its
  template bytes, so `config.toml` and the store exist and there is no block. It then asserts:
  - the byte-exact single append;
  - that `git check-ignore` reports the run tree ignored, at a path derived through `derive`. A control asserts it is
    not ignored before the block exists;
  - that no `.gitignore` exists;
  - byte identity across a second `init`;
  - and, with `info/` removed, that the file is created holding exactly the block, and stays so across another `init`.
- **The second row** starts from an operator's file with CRLF lines, no trailing newline, and a
  `# redkiln-runtime-notes: …` look-alike line.

**Mutants**, from `p5initexclude_mutants.py`. Every anchor was asserted once, each write went through tmp plus
`os.replace`, each file was restored in `finally`, and the sha256 `3503382edde9…` was verified after each. Each read
rc 101 and *"0 passed; 1 failed"*:

| mutant | killed by, and the reason read |
| --- | --- |
| `S8D-II-M6` (append on every run) | `init_writes_the_redkiln_runtime_exclude_block_once`: *"a second init changed info/exclude; it appends only when the marker is absent"* |
| `P5-X-PREFIX` (marker matched as a prefix) | `init_leaves_the_operators_exclude_lines_untouched`: the look-alike line suppressed the block (`left` is the operator's bytes alone) |
| `P5-X-GLUE` (no separator before the block) | the same row: `left` ends `/scratch/# redkiln-runtime` |
| `P5-X-REWRITE` (truncate instead of append) | the same row: `left` is the block alone |

**Guards (read-only lines):**
- **Checkpoint 8d-ii (3):**
  - the `# redkiln-runtime` code-line floor on `init.rs` passes. Its two synthetic controls fire: HEAD's `init.rs`,
    and a copy with the marker only in comments.
  - The root floor reads the same 12 lines as at entry; neither `init` file is among them.
- **Checkpoint 7d:**
  - its absence bans (lines `rk_absent` … `set_var|remove_var`, with the helpers verbatim) read 37 `PASS` and rc 0;
  - its `DENYLIST` and `INIT_UNREAD_WINDOW_RUNS` lines pass;
  - no `Command::new` is in `init.rs`.
- No older checkpoint's command line reads `init.rs`, `tests/it/init.rs` or the new constant.
- **Other checks:** `check-refs` passes, `fmt` passes, and clippy on `rk-cli` and `rk-git` (all targets) exits 0 with
  no warning on this package's lines.

**Suite:** the whole suite through the helper reads rc 0, **1015 passed / 0 failed / 10 ignored over 44 result
lines**, which is 1013 plus the 2. At the end:
- `rk-run/rk-fix-doctor-test-count` is at `73c087a82245…`, with its `Story:` trailer and its registration;
- the worktree path set is unchanged;
- the store's sha256 is `e83d295e…`, unchanged, and backed up to
  `target/store-backups/p5initexclude-before-suite.sqlite3`;
- `.redkiln/worktrees` is absent.

**Left for later packages:**
- P6 writes the row into every fixture and builder a `run` reaches. It should take the row from
  `RUN_ROOT_IGNORE_ROW`, or from `init`, never as a literal.
- This repository's own `info/exclude` is not written here: no `redkiln init` ran against it (AFK rule). Its tracked
  `.gitignore` row is W4's other half.

*(dated 2026-09-27, s8d-ii build, package P6-RKCLI-ROOT-MOVE.)* **§8d-ii.1's root move, §8d-ii.3's `rk-cli` half, §8d-ii.2's
liveness predicate, §8d-ii.4's carry and §8d-ii.7's amendments are built; the table's six `run_worktree` rows are green,
five seen red first and the pin green at entry by design, and the two witnesses stay green.**

**What `run` does now (`crates/rk-cli/src/verbs/run.rs`):**
- **Step (8) enters the tree; it no longer re-creates it.** `recreate_run_worktree` is deleted. `enter_run_worktree` calls
  `rk_git::worktree::enter` with this process as the lock owner (`run_lock_owner`: `std::process::id()` and
  `rk_containment::process_start_time` of it, refused by name if the OS reports none), the injected predicate
  `process_probe` (`is_alive`, then `process_start_time`, so `is_alive` is a production call, ADR-0180 decision 6),
  and `GIT_LFS_SKIP_SMUDGE` read through a new `Env::lfs_skip_smudge` in `env.rs` (the one environment read, ADR-0083).
  A refusal prints its `WorktreeRefusal` by name and exits `3`.
- **The discard reaches stderr, one line per path, until `s8e`:** *"redkiln: re-entry discarded `<path>` in <tree>
  (not landed at a committed step)"*.
- **The lock is released on every path the lane ends by.** The lane loop moved whole from `run` into `run_lane`, and
  `run` calls `release_run_worktree` after it, whatever it returned; a failed carry releases too.
- **The one derivation.** `run_tree_of` (`main_worktree`, `ItemId::parse`, `derive`) is what `story_trailer`,
  `terminal_read` and `reference_worktree` read; `run.rs` names no `worktrees` directory on any code line.
- **The review reference tree** (§8d-ii.3; ADR-0162 decision 10 cited): created at the fork the run tree's `RK_BASE`
  records (`worktree add --detach <path> <fork>`), with its own `RK_BASE` (`fork`, `item`) written after the add. A
  leftover at `<item>.review-ref` is force-removed only when it is registered and carries `RK_BASE` for this item;
  otherwise the pass refuses by name, exit `3`, and the leftover is kept. A run tree without `RK_BASE` refuses the pass
  rather than read a base off `HEAD`. The forced removal and its `remove_dir_all` fallback stay inside
  `remove_reference_worktree`, whose doc and `reference_worktree`'s doc are rewritten by dated edit, the old
  merge-base argument struck.
- **The carry, at creation only** (`carry_at_creation`, skipped for `Creation::Reused`): `s8c`'s `carry_set` over the
  main worktree, each carried file copied to the same relative path (skipped entries named on stderr), then `s8c`'s
  `setup` verb through `run_verb` on the toolchain `choose_toolchain` selects in the fresh tree, with the
  `setup_projects` declaration row of that toolchain's table and the `setup` row's timeout key resolved as the bound.
  No `registry.toml`, no detected adapter, no `setup_projects` row or an empty one: nothing is installed. Any refusal
  (the carry's, a copy, the registry, the row, the bound, or `setup`'s own) prints by name, releases the lock and exits
  `3` before any dispatch.

**The root floor's two `s8c` lines (§8d-ii.0's dated finding), settled by routing, not by widening `OWN`:**
`rk_git::worktree::RUN_ROOT_EXCLUDE_PATHSPEC` (`:(exclude).redkiln/worktrees`) is declared beside `derive`, with a
`const` assertion that it names the same root as `RUN_ROOT_IGNORE_ROW`, and `worktree_include::RUN_WORKTREES_EXCLUDE`
now names it (value unchanged). The `worktree_include` test plants its tree at `derive(&r, "x").path`. `OWN` is not
edited.

**The six test sites and the builders.** `tests/it/run.rs`'s five and `answer.rs`'s `owned_worktree` now read
`crate::run_tree` (`tests/it/main.rs`), which calls `derive`. `crate::ignore_run_root` appends
`RUN_ROOT_IGNORE_ROW` to `<common-dir>/info/exclude`, and `pack_min_repo` calls it, so every repository item 6's list
shows a `run` creating a tree in carries the row: `pack_min_repo` itself and, through it, `repo_with_open_gate`,
the `adr0128_*` tests, AC-438's `base`, `dispatch-1s` and `review` cases and `cost`'s run. AC-438's `none` case
(`temp_repo` plus `run_init`) gets the row from `init`'s block. **`temp_repo` does not take it, a deviation from
§8d-ii.0 item 6's "the two shared helpers":** `init::init_writes_the_redkiln_runtime_exclude_block_once` starts from
git's template exclude and asserts the run tree is *not* ignored before the block exists, and a row in `temp_repo`
would make that control false. No `temp_repo` repository reaches creation without `init` (measured: the whole suite
below, and no test failed with the ignore refusal's name). The other code builders (`synthetic_trailer_repo`,
`repo_seeded_from`, `repo_with_record`, `change_repo`, `gate_repo_with_questions`, `source_repo`) reach no creation.

**RED, then GREEN** (each selected module-qualified with `--exact`):
- `the_run_worktree_is_created_under_dot_redkiln_worktrees`, on the entry tree: *"no run tree at the derived path
  `…/.redkiln/worktrees/bug-1` (the old common-dir root `…/.git/redkiln/worktrees` HOLDS it: the tree was created
  under the common dir)"*. It also asserts the registration on `rk-run/bug-1`, the released lock, `RK_BASE` (fork =
  `HEAD`, item), the ignore, and on a second run the reuse and one stderr line per discarded path.
- `no_redkiln_reader_sees_a_planted_run_worktree`: **green at entry**, as its row requires (*"1 passed"* beside the row
  above). It compares `doctor::run_to`'s rendered findings (with a determinism control: two reads before planting
  must agree), `prepare_inputs`' `(RunInputs, tree hash)` and `through_step_seven`'s item, stage and pack, with and
  without `.redkiln/worktrees/planted/.redkiln/{catalog,cards,specs,items}`. The planted card sets `model` and has
  no `tools`, so a reader that saw it would report it.
- The two reference rows were read red on the root-moved tree with the reference tree still in its entry form (at
  `HEAD`, a leftover force-removed):
  `the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves` *"after HEAD moved to a4de23b…, the
  review reference tree was created at a4de23b… — not at the run tree's fork b50c862… that RK_BASE records (ADR-0180
  decision 10): it was created at HEAD"*; `a_leftover_reference_tree_without_rk_base_is_refused_and_kept` *"a leftover
  reference tree WITHOUT RK_BASE was removed (exit Some(4)); redkiln removes only a tree it made"*. The observation
  point is the rigged stub toolchain: its reference arm appends the tree's `HEAD` and `RK_BASE` to
  `<common-dir>/rk-fixture-reference-heads` (`record_reference`, reading files only), because decision 10 removes the
  tree before any test could read it. The leftover row carries its own control: the same leftover once marked is
  force-removed and the pass exits `4`.
- The two carry rows, on the tree without the carry: `creation_carries_worktreeinclude_files_and_runs_setup` *"the
  ignored, matched .env was not carried into the fresh run tree"* (`left: None`); `a_failed_setup_refuses_by_name`
  *"the failed install was not refused naming `setup` and the project (exit Some(10)); stderr: redkiln: the dispatch of
  `emit-report` (emit-report@1) did not commit."*. The fixture (`carry_repo`) is the review variant (a committed
  `Cargo.toml`, the rigged `cargo`), a committed `.gitignore` and `.worktreeinclude`, an ignored `.env`, a committed
  `web/` and a `registry.toml` declaring `setup_projects = ["web"]`. The success row copies the Node fixture's
  `package.json` and `pnpm-lock.yaml` and runs the real `pnpm install --frozen-lockfile --prefer-offline` (Checkpoint
  8d-ii (0)'s warm-store precondition), asserting `web/node_modules/.modules.yaml` in the tree, none in the checkout,
  and no re-carry on re-entry; the failure row is `s8c`'s own failing shape (a dependency and no lockfile).
- **GREEN:** the six, *"test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 143 filtered out"*. **(2)'s
  selections, extracted verbatim with its zero-match control** (`sel_tests rk-git 42`, `rk-containment 1`, `rk-cli
  10`): rc 0, so the six, `init`'s two and the two witnesses (`run::ac438_…_binary_arm`, `cost::cost_after_a_real_run_…`)
  each report `... ok` and the count is exactly 10.

**Mutants**, from `p6rkcliroot_mutants.py` (anchor asserted once, tmp plus `os.replace`, restored in `finally`, sha256
verified after each: `worktree.rs` `56c9063e…`, `rules.rs` `2226a890…`, `run.rs` `e5ba1712…`). Each read rc 101 and
*"0 passed; 1 failed"*:

| mutant | killed by, and the reason read |
| --- | --- |
| `S8D-II-M1` (`run_tree_root` under `<main>/.git/redkiln/worktrees`, the common dir) | `the_run_worktree_is_created_under_dot_redkiln_worktrees`: *"the old common-dir root `…/.git/redkiln/worktrees` exists after a run; nothing may be created there"* |
| `S8D-II-M9` (`doctor`'s card reader walks `.redkiln/` recursively for `cards/*.json`) | `no_redkiln_reader_sees_a_planted_run_worktree`: *"doctor's findings changed once a run tree was planted"*; the diff is `RK-D-CARD-TOOLS-ABSENT`, `RK-D-TIER-LEAK` and `RK-D-CACHE-PROFILE` each reading `cards/planted.json` |
| `S8D-II-M19` (the reference tree added at `HEAD`) | the reference row: *"… was created at 9c84f4c… — not at the run tree's fork 85dae09… … it was created at HEAD"* |

**Guards (read-only lines):**
- **Checkpoint 8d-ii (3), extracted verbatim, passes whole** with its in-block controls: the root floor over
  `crates/*/src` and `crates/*/tests` reads no line outside `OWN` (12 at entry, 0 now), `recreate_run_worktree` is on
  no code line, `run.rs`'s `"--force"` and `remove_dir_all` lie inside `remove_reference_worktree`, and the LC_ALL,
  `RK_BASE`, `--show-scope`, `--source`, pointer-header and `# redkiln-runtime` floors hold.
- **(7b)'s text guards, extracted verbatim, over 7f's block as (7)'s extractor takes it:** rc 0, and `bash -n` parses
  the block. Controls: over HEAD's 7f they fail (*"no code line in 7f clones this repository"*); over an amended copy
  with a `git worktree prune` code line, and one reading `.git/redkiln/runs` without `$clone`, each names its line.
  `s8e`'s (10d) text guards (the code-line `git clone` and no `worktree remove --force .git/redkiln/worktrees/` anywhere
  in the block) pass too. 7f itself was **not** replayed (AFK rule: no redkiln binary against this repository).
- **(9), extracted verbatim, passes whole** (`REC_WT=0180`, private scratch): the AC-518, AC-522, AC-529 and AC-531
  notes, design §5's `reconcile_started` row and §6's `The write set, enumerated` row, `s7d`, and the fixture doc.
  Controls: `acnote.py` over HEAD's register reads *"MISSING: AC-518, AC-522, AC-529"*; HEAD's design rows, `s7d` and
  stub each carry 0 of their needles.
- Checkpoint 7d's absence bans (P5's extraction) read 37 `PASS`, rc 0. No older checkpoint's code line reads the
  functions this package touched in `run.rs`. check-refs, check-bash-syntax (121 blocks), check-checkpoints and
  check-adr-options each flag 0.
- fmt passes; clippy on `rk-cli`, `rk-git` and `rk-containment` (all targets) exits 0, and none of the remaining
  warnings is on this package's lines.

**Suite:** the whole suite through the helper reads rc 0, **1021 passed / 0 failed / 10 ignored over 44 result lines**,
which is 1015 plus the six, and meets (4)'s `FLOOR=1021` exactly. At the end `rk-run/rk-fix-doctor-test-count` is at
`73c087a82245…` with its `Story:` trailer and registration, the worktree path set is unchanged, the store's sha256 is
`e83d295e…` (backed up to `target/store-backups/p6rkcliroot-before-suite.sqlite3`), and `.redkiln/worktrees` does not
exist in this repository.

**Design choices a later package or the operator may revisit, recorded rather than hidden:** the carry copies from the
main worktree (the root the tree hangs under), not from the checkout `run` started in; `setup` re-runs
`choose_toolchain` in the fresh tree, so its deferral lines can print twice per first run; `Entered::filter_notes` is
printed nowhere, because `rk-cli` has no debug level (§8d-ii.5's "debug level only" holds vacuously); `is_alive`
keeps its `#[doc(hidden)]` as `s7c` §7c.10 lists it, with a dated doc note in `windows.rs` and `posix.rs` that it is
now a production call.

*(dated 2026-09-27, s8d-ii build, package P7-GUARD-SWEEP.)* **The mutant ledger is complete. All twenty-six mutants were
re-applied to the FINAL tree and each is killed by its named test for its named reason.** The packages killed each
mutant on the tree of their day, and later packages changed `worktree.rs`, `run.rs` and `git_exec.rs` after them, so
those kills were claims about earlier files. The consolidated driver `p7guard_mutants.py` re-applied every mutant to the
private worktree `target/wt-p7`, an overlay of the working tree on `83cf836` whose 32 changed or untracked files were
compared byte for byte with the main checkout before and after. The drivers' rules held: each anchor asserted once,
tmp plus `os.replace`, restored in `finally`, sha256 verified (`worktree.rs` `a7e2018e3e12…`, `git_exec.rs`
`c9bcd2f29463…`, `init.rs` `6b63b2e670f5…`, `rules.rs` `2226a890a605…`, `run.rs` `7e9b086b16a6…`), and writes were
refused outside `target/wt-p7`. A kill counts only when all of these hold:
- the rc is non-zero;
- the count line reads *"0 passed; N failed"* for the N named tests;
- each named test reports `FAILED`;
- each test's own stdout section carries the reason.

Every mutant read rc 101 and *"KILLED 26 of 26"*. **No mutant survived, so no survivor needed an unobservability or
later-stage disposition.** `s8e` keeps the `lane_e2e` half of S8E-M20, as the table above says.

| mutant | killed by (module-qualified, `--exact`) | the reason read on the final tree |
| --- | --- | --- |
| `S8D-II-M1` | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` | *"the old common-dir root … exists after a run; nothing may be created there"* |
| `S8D-II-M2` | `worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip` | *"the registered tree was re-created, not reused: OnExistingBranch"* |
| `S8D-II-M3` | `worktree::a_tree_without_rk_base_is_refused_and_never_removed` | *"… was removed from disk (refused trees are never removed)"* |
| `S8D-II-M4` | `worktree::reentry_discards_in_flight_changes_and_reports_each_path` | *"the discard did not report each path"* |
| `S8D-II-M5` | the `system` and `global` skip rows and the BrittonGames control (3 of 3) | *"not the skipped-smudge refusal at system scope: FilterDriverNotEffective …"*, the same at global scope, and *"creation was REFUSED … driver: "lfs""* |
| `S8D-II-M6` | `init::init_writes_the_redkiln_runtime_exclude_block_once` | *"a second init changed info/exclude; it appends only when the marker is absent"* |
| `S8D-II-M7` | `gitops::every_git_child_runs_with_lc_all_c` | *"5 of 5 routes ran git without LC_ALL=C (W1)"* |
| `S8D-II-M8` | `worktree::a_lock_held_by_a_dead_pid_is_released` | *"a dead pid's lock was not released: … LockHeld { …, pid: 111, start: 111000 }"* |
| `S8D-II-M9` | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` | *"doctor's findings changed once a run tree was planted under the run-tree root"* |
| `S8D-II-M10` | `worktree::reentry_keeps_ignored_files` | *"an ignored carried file did not survive re-entry"* |
| `S8D-II-M11` | `worktree::creation_refuses_when_the_worktree_root_is_not_ignored` | *"creation under an unignored root: ADMITTED"* |
| `S8D-II-M12` | `worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently` | *"creation was REFUSED and must not be: … FilterDriverNotEffective { driver: "lfs" …"* |
| `S8D-II-M13` | the declared-but-unselected control and the BrittonGames control (2 of 2) | *"creation was REFUSED … driver: "mutant""*, in each |
| `S8D-II-M14` | `worktree::a_repo_with_no_filter_or_include_if_creates` | *"no filter anywhere: creation was REFUSED"* |
| `S8D-II-M15` | `worktree::the_filter_attribute_is_read_from_the_fork_commit` | *"the fork commit selects `nodrv`; the working tree selects nothing: ADMITTED"* |
| `S8D-II-M16` | `worktree::an_unlanded_tree_is_kept_with_its_reason` | *"an UNLANDED tree (its tip carries no closed mark) was removed"* |
| `S8D-II-M17` | `worktree::the_root_is_the_main_worktree_when_run_from_a_linked_worktree` | *"read from the linked worktree, the root is `…/rootmain.swt`, not the main worktree"* |
| `S8D-II-M18` | `worktree::the_length_rule_reads_the_actual_item_length` | *"a 1-character item was refused on a 64-character bound: … bound: 260, limit: 259"* |
| `S8D-II-M19` | `run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves` | *"the review reference tree was created at dec9cbb… — not at the run tree's fork 98d0f5e…"* |
| `S8D-II-M20` | `worktree::a_lock_whose_pid_now_names_another_process_is_released` | *"a lock whose pid now names another process was not released: … LockHeld"* |
| `S8D-II-M21` | `worktree::git_lfs_skip_smudge_in_the_environment_refuses_by_name` | *"GIT_LFS_SKIP_SMUDGE=1: ADMITTED"* |
| `S8D-II-M22` | `worktree::a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree` | *"a pointer file at a selected path after checkout: ADMITTED"* |
| `S8D-II-M23` | `worktree::a_pointer_header_at_a_path_no_filter_selects_does_not_refuse` | *"a pointer header at an unselected path: creation was REFUSED … PointerFilesAfterCheckout"* |
| `S8D-II-M24` | `worktree::a_committed_pointer_that_the_driver_smudges_creates_silently` | *"a committed pointer the driver smudges: creation was REFUSED … PointerFilesAfterCheckout"* |
| `S8D-II-M25` | `worktree::the_pointer_scan_never_removes_a_tree_this_call_did_not_create` | *"re-entry was refused: WorktreeRefused { refusal: PointerFilesAfterCheckout …"* |
| `S8D-II-M26` | the pointer row, by its absence assertion | *"the fresh tree `…\item-1` is still on disk after the refusal"* |

`S8D-II-M2` is re-applied in its final-tree form: `create` takes P4's `lfs_skip_smudge` argument, which P3's form
predates. The packages' extra mutants (`P4-X-*`, `P5-X-*`, `ENTRY-FORM`, J1 and J2) sit outside the table's
twenty-six and were not re-run.

**The count, re-measured off the suite logs.**
- The whole suite in the private tree reads **1021 passed / 0 failed / 10 ignored over 44 result lines**, which is
  Checkpoint 8d's recorded 971 (D21-78) plus 50, and so meets `FLOOR=1021`.
- By name, the log carries 1011 distinct test names against 961 in P1's stage-entry baseline: **51 new and 1 gone**.
  The gone name is `worktree::an_existing_worktree_path_is_refused_not_reused`, the rename. The 51 new are the table's
  53 less its 2 witnesses; the rename counts as one of them, so **50 are new in the pass count**.
- (2)'s three selections, run verbatim, read `rk-git` 42, `rk-containment` 1 and `rk-cli` 10, with every `... ok` line
  present and the zero-match control reading 0 passed.

*(dated 2026-09-27, s8d-ii build, package D2188.)* **D21-88 ruling 2's rollback, built red first; four mutants killed.**
- **Red, on the stub** (`roll_back_fresh` returning `Kept { reason: "not built" }`, so both tests compiled):
  `worktree::a_fresh_tree_whose_carry_failed_is_rolled_back_with_its_links_unlinked` read *"the fresh tree was not
  rolled back: Kept { …, reason: "not built" }"*, and
  `worktree::a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back` read *"a reused tree was kept for the
  wrong reason: not built"* (a stub's red; its stated reasons are read under `S8D-II-M29` and `S8D-II-M30`). *"0
  passed; 2 failed"*.
- **Red, before `run` called it:** `run_worktree::a_fresh_tree_whose_setup_failed_is_rolled_back_and_the_next_run_recarries`
  read *"the fresh tree whose setup failed was not rolled back: registered Some(["worktree …/.redkiln/worktrees/bug-1",
  …]), on disk true"*, and the amended `run_worktree::a_failed_setup_refuses_by_name` read *"after the refusal the
  fresh tree was not rolled back (D21-88 ruling 2)"*. *"0 passed; 2 failed"*.
- **Green:** the two `rk-git` tests *"2 passed; 0 failed"*; the two `rk-cli` tests with
  `run_worktree::creation_carries_worktreeinclude_files_and_runs_setup` *"3 passed; 0 failed"*.
- **Mutants** (`d2188_mutants.py`: each anchor asserted once, tmp plus `os.replace`, restored in `finally`, sha256
  verified; a kill needs rc non-zero, the exact *"0 passed; N failed"* line, each named test `FAILED`, and the
  reason in the output). **KILLED 4 of 4:**
  - `S8D-II-M27`, both `rk-cli` tests: *"not rolled back … registered Some(…), on disk true"*;
  - `S8D-II-M28`: *"the fresh tree was not rolled back: Kept { …, reason: "git reported the removal done and the
    tree is still on disk" }"*, which is finding 17's measurement in miniature (git's unforced removal leaves the
    junctions);
  - `S8D-II-M29`: *"a REUSED tree was rolled back"*;
  - `S8D-II-M30`: *"a tree locked by ANOTHER run was rolled back"*.
- **The pre-unlink over a real `pnpm` install is not re-measured here**: the failing fixture's install refuses before
  any fetch, so its tree holds no `node_modules`. The junction case is `rk-git`'s test, over `mklink /J` junctions.
- **Selections:** (2)'s `rk-git` list is 44 and `rk-cli` 11; `FLOOR=1024` (971 + 53).

*(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* **Refuting round 1's findings: D21-89's two rulings, the platform-limit tests' fixture, and the
minors, built red first; every new mutant killed, and the rk-git ledger re-killed on the amended tree.**
- **D21-89, in `rk_git::worktree::create`.** `branch_exists` became `branch_tip`, which reads
  `refs/heads/rk-run/<item>^{commit}` before any guard. `checked` is that tip, or start's `HEAD` on a first creation, and
  `prepare_creation` (so the length rule) and `check_filters` (so the scan's path set) read it. `RK_BASE`'s fork is
  `merge-base(start HEAD, tip)` on the existing-branch arm, read before the add through a new `GitOp::MergeBase` read,
  and start's `HEAD` otherwise; `Entered::fork` agrees, so `roll_back_fresh`'s marker check holds on either arm.
  Histories with no common ancestor return the wrapper's `merge-base` error before the add, with nothing created.
- **Round-1 minors in code.** A re-entry whose discard is incomplete unlocks the lock it took before returning
  `DiscardIncomplete`. Read (3)'s `gitdir:` match uses `admin_dir_name`, which predicts git's suffix.
- **RED, before the product change** (the six `rk-git` rows): *"0 passed; 6 failed"*, each for its row's reason:
  the ineffective-driver row *"the tip selects `nodrv`, declared at no scope; start's HEAD selects nothing: ADMITTED
  (Entered { … created: OnExistingBranch …"*; the pointer row *"a pointer file at a path the tip selects:
  ADMITTED"*; the length row *"the tip's worst case is 260; start's HEAD's is short: ADMITTED"*; the merge-base row
  *"RK_BASE records e76a1d3… and not the merge-base b018798… of start's HEAD e76a1d3… and the tip a39d7ed…"*; the
  discard row *"the refused re-entry left the run's own lock on the tree"*, `left: Some("redkiln run item-1 (pid 2
  start 2000)")`; the gitdir row *"`gitdir:**/.git/worktrees/item-11` (the run tree's suffixed gitdir): not the
  skipped-smudge refusal: PointerFilesAfterCheckout(…)"*, the scan catching what the read missed.
- **GREEN:** *"6 passed; 0 failed; 0 ignored; 0 measured; 121 filtered out"*. The two `rk-cli` pins: *"2 passed; 0
  failed"*.
- **The platform-limit tests (round 1's major).** Their fixtures were sized from `path_bound`, the function under
  test, so a dropped term moved the fixture with it. Measured first, on the unchanged tests: `R1FIX-PB-SUFFIX` and
  `R1FIX-PB-PLUS1` each read *"test result: ok. 1 passed"*, SURVIVED. The tests now size the fixture from the literal
  terms (`fixed_terms`: root + 20 + item + 11 + 1), assert `bound: 260` as a literal, and pin `path_bound(10, 6, 100)
  == 148` at the end; both mutants are then KILLED, *"a 260-character worst case: ADMITTED"*.
- **Mutants** (`r1fix8dii_mutants.py`: each anchor asserted once, tmp plus `os.replace`, restored in `finally`, sha256
  verified; a kill needs rc non-zero, the exact *"0 passed; N failed"* line, each named test `FAILED`, and the reason):
  **KILLED 9 of 9** on the final tree — the two PB mutants; `R1FIX-D2189-HEAD` (3 of 3, each *"… ADMITTED"*);
  `R1FIX-D2189-BASE` (the merge-base row's line above); `R1FIX-DISCARD-LOCK`; `R1FIX-GITDIR-SUFFIX` and, with the
  test's `item-11` case removed by the driver so the other direction is read, `R1FIX-GITDIR-SUFFIX-OTHER`
  (*"`gitdir:**/.git/worktrees/item-1` (another tree's gitdir): creation was REFUSED … SmudgeSkipped … sfxother-skip.cfg"*);
  `R1FIX-PROBE-NONE` (*"a lock held by a LIVE process (this test, pid …) did not refuse the run by name (exit
  Some(10))"*); `R1FIX-LFS-NONE` (*"GIT_LFS_SKIP_SMUDGE=1 in run's environment did not refuse creation by name (exit
  Some(10))"*).
- **The ledger re-killed on the amended `worktree.rs` and `git_exec.rs`:** P7's driver re-pointed at the main checkout
  (`r1fix8dii_p7rerun.py`), every anchor found once, *"KILLED 22 of 22"* for `S8D-II-M2`–`M5`, `M7`, `M8`, `M10`–`M18`
  and `M20`–`M26`; D2188's driver, *"KILLED 3 of 3"* for `S8D-II-M28`–`M30`. `M1`, `M6`, `M9`, `M19` and `M27` sit in
  files this package did not change in their anchored regions and were not re-run.
- **Selections:** (2)'s `rk-git` list is 50 and `rk-cli` 13; `FLOOR=1032` (971 + 61).
- **Design choices, recorded rather than hidden:** the tip is read, then the tree is added by the branch name, so a
  branch moved between the read and the add would be checked out unread (a concurrent writer of `rk-run/<item>`,
  which the lock does not yet cover because no tree is registered); `admin_dir_name` predicts git's name before the
  add and nothing re-reads the gitdir after it, the same race, with X2's scan as the backstop.

*(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)* **Refuting round 2's minors (dry: no blocker or major; the raw findings in the session scratchpad,
`s8dii-r2.json`): the reference tree's commit on the existing-branch arm, the merge-base fixture, and the records.**
- **The reference tree's commit.** On the existing-branch arm the run tree checks out the tip (D21-89 ruling 1), and
  `rk-cli`'s `reference_worktree` adds `<item>.review-ref` at `RK_BASE`'s fork, the merge-base (ruling 2). The
  bound's `.review-ref` term was paired with the tip's longest path. `create` now reads the merge-base **before**
  the guards; the length rule (`refuse_past_limit`, now over a list of commits, through a private
  `prepare_creation_over`; `prepare_creation`'s public signature is unchanged) takes the longest path over the tip
  and the merge-base, and `check_filters` runs a second time over the merge-base, whose refusal names it. Its
  selection is not the scan's path set; only its debug notes are kept. `PathPastLimit`, `FilterDriverNotEffective`
  and `SmudgeSkipped` gain a `commit` field, and their messages say *"at `<commit>`"* where they said *"at the fork
  commit"*. On a first creation both commits are start's `HEAD`, read once.
- **RED, before the product change** (the two rows, with the `commit` field already added so the test compiled):
  *"0 passed; 2 failed"* — *"the merge-base's worst case is 260; the tip's and start's HEAD's are short: ADMITTED
  (Entered { … created: OnExistingBranch …"* and *"the merge-base selects `nodrv`, declared at no scope; the tip
  selects nothing: ADMITTED"*. **GREEN:** the two rows with the merge-base row *"3 passed; 0 failed"*; the whole
  `worktree::` module *"57 passed; 0 failed"*.
- **The merge-base fixture.** With one commit a side, the fork was `tip^` and `HEAD^` both. Measured first, on the
  unchanged test: `R2FIX-MB-TIPPARENT` and `R2FIX-MB-HEADPARENT` each read *"test result: ok. 1 passed"*,
  SURVIVED. `run_branch_ahead` now makes two commits on each side and asserts the fork is neither parent; the test
  asserts `RK_BASE` equals the fixture's own `git merge-base <head> <tip>` and differs from both parents. Both
  mutants are then KILLED, *"RK_BASE records a39d7ed… and not the merge-base b018798…"* (the `tip^` one; the
  `HEAD^` one names `fc1c9ad…`). The three other `run_branch_ahead` rows stay green.
- **Mutants** (`r2fix8dii_mutants.py`, the R1FIX driver's discipline; a survivor needs the exact *"ok. N passed"*
  line): **KILLED 4 of 4** on the final tree. The round-1 driver re-killed `R1FIX-PB-SUFFIX`, `R1FIX-PB-PLUS1`,
  `R1FIX-D2189-HEAD` and `R1FIX-D2189-BASE` on the amended `create`, *"KILLED 4 of 4"*.
- **Records and prose, by dated edits:** `reference_worktree`'s doc and §8d-ii.3 no longer say `GitOp` carries no
  `merge-base`; ADR-0180 decision 12's D21-89 insertion reads *"read (1)"*, not *"reads (1) and (2)"*, and
  decisions 12 and 16 carry the merge-base reads, with a forbids line and an evidence line; ADR-0180's title and
  README index row name D21-88 and D21-89 (the census does not move); Checkpoint 8d-ii's W4 bullet and
  `path_bound`'s doc say *"the commit the tree checks out"* (wording only); §8d-ii.7, `s7f`'s prose note and
  `PROGRESS.md`'s 7f row count four amendments; the ledger's title reads *"2026-09-21 … 2026-09-28 (D21-1 …
  D21-89)"*.
- **Selections:** (2)'s `rk-git` list is 52 and `rk-cli` 13; `FLOOR=1034` (971 + 63).
- **Residuals, recorded rather than built:** the second filter read resolves drivers as the **run** tree does (read
  (3) matches the run tree's gitdir and branch, not the reference tree's detached gitdir); D21-48 X2's scan covers
  the run tree only; and the merge-base is read before the add, so the race the tip read has (a branch moved
  between the read and the add) applies to it too.

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
17. *(dated 2026-09-27, s8d-ii build, P3.)* **git's own unforced `worktree remove` leaves a `setup`-installed
    `node_modules`' junctions on disk** (`s8c` §8c.12 item 3, measured here without `--force`). It exits 0 and
    deregisters the tree, and leaves 114 dangling junctions, 0 files and 60 directories (git 2.55.0.windows.3). The
    lifecycle's removal unlinks every untracked link first, never walking through one, and then leaves nothing
    (§8d-ii.8's P3 note). That is ADR-0180 decision 9's *"a link inside the tree is removed, never its target"*.
    Under the package brief, a raw removal that leaves dangling junctions is a blocker the stage file did not settle,
    so it goes to the orchestrator for a ruling, with the built behaviour as the most reversible option. ~~*Open.*~~
    *(dated 2026-09-27, s8d-ii build, package D2188.)* *Closed by D21-88 ruling 1:* *"**(17) Keep the pre-unlink as built:** a link inside the tree is
    removed, never its target, then git's unforced removal runs, and any residue refuses as `WorktreeRemoveFailed`."*
    The measurement is in ADR-0180 decision 9 and its evidence (proposed) *(dated 2026-09-28, `s8d-ii` close: accepted on D21-91)*, and D21-88 ruling 2's rollback uses the
    same pre-unlink.
18. *(dated 2026-09-27, s8d-ii build, P6.)* **A tree whose `setup` failed is not set up again on re-entry.** The run
    refuses by name and releases the lock, and the fresh tree stays registered (no rule removes a tree that is not
    landed, and X2's rollback is the scan's alone). The next run re-enters it as `Creation::Reused`, and §8d-ii.4's
    *"A reused tree is not re-carried"* then skips both the carry and `setup`, so it runs over a tree with half its
    dependencies. The stage file does not settle it; the options, most reversible first, are to record the
    completed carry in the tree's gitdir (beside `RK_BASE`) and re-run an incomplete one at re-entry, or to roll the
    fresh tree back unforced on a carry failure as X2 does. ~~*Open; reported, not chosen.*~~
    *(dated 2026-09-27, s8d-ii build, package D2188.)* *Closed by D21-88 ruling 2:* *"**(18) Roll back on failure:** when a FRESH tree's carry or `setup`
    fails, the tree is removed (unforced, through the same pre-unlink path) before the run refuses by name"*. Built
    red first (§8d-ii.4, §8d-ii.8's D2188 note), recorded in ADR-0180 decisions 9 and 11 (proposed) *(dated 2026-09-28, `s8d-ii` close: accepted on D21-91)*. What remains: a
    rollback git refuses keeps the tree, named, and the next run reuses it without a carry (ADR-0180's consequences).
19. *(dated 2026-09-27, s8d-ii build, P6.)* **Checkpoint 7d's absence check on the old root is now vacuous.** `s7d`'s
    block asserts `test ! -d "$TMP1/.git/redkiln/worktrees"` after a refused run; the root is no longer there, so the
    line cannot fail. It still passes, and nothing older is re-run here; `s7d` is not edited beyond its two dated
    table notes.
20. *(dated 2026-09-27, s8d-ii build, P6.)* **Checkpoint 7f's replay installs the Node fixture in its clone.** The
    shipped `[toolchain.rust]` declares `setup_projects`, so the clone's run tree runs `pnpm install --frozen-lockfile
    --prefer-offline` at creation (a warm store: Checkpoint 8d-ii (0)), and `rm -rf "$c7f"` removes the resulting
    `node_modules` junctions. The replay has not been run (AFK rule); its wall time and the junction removal under
    `rm -rf` are unmeasured.
21. *(dated 2026-09-27, s8d-ii build, P6.)* **`temp_repo` does not take the ignore row** (§8d-ii.8's P6 note): `init`'s
    own row asserts that the root is not ignored before its block exists. Every repository a `run` creates a tree in
    carries the row through `pack_min_repo` or `init`.

*(dated 2026-09-27, s8d-ii build, package P7-GUARD-SWEEP: dispositions the build settles, and two findings of the guard
sweep.)*
- **Finding 7, re-measured on the built tree:** the worst case is **255 of 259** here for a 64-character item and
  **246** in Checkpoint 7f's `mktemp -d` clone for `rk-fix-doctor-test-count`. The longest tracked path is 140, and
  `core.longpaths` is unset (rc 1). Both readings come from (0b), run verbatim. The rule `path_bound` builds reads the
  actual item (`S8D-II-M18`, killed above).
- ~~**Findings 17 and 18 stay open.** No build fact settles either: 17 waits on the orchestrator's ruling, and 18 is
  reported and not chosen.~~ *(dated 2026-09-27, s8d-ii build, package D2188.)* Both are closed by D21-88 (rulings 1 and 2). Findings 19 and 20 are unchanged: 7d and 7f were not replayed here.
22. *(dated 2026-09-27, s8d-ii build, P7.)* **Two older lines were already red, and this stage adds three hits to
    them.** The lines are Checkpoint 6's workspace-wide `Command::new("git")` line and `ci.yml`'s "git shells out
    through one wrapper only" step.
    - **At `83cf836`**, both are RED on test-helper hits in `redkiln-migrate` and `rk-cli` tests (`main.rs`,
      `answer.rs`, `root.rs`, `support.rs`). PROGRESS row 6 carries *"re-run pending 7c/7d Command sites"*, and the
      `s6` line is *"RAISED for narrowing"*.
    - **This stage adds three helpers of the same shape,** each under `#[expect(clippy::disallowed_types)]`:
      `init.rs`'s `git_ignores`, and `run_worktree.rs`'s `git_out` and `git_rc`.
    - **Measured.** A differential over HEAD's tree and the working tree puts the untracked `run_worktree.rs` into a
      private copy of the index as intent-to-add. The pre-existing hits are identical, and the only new lines are those
      three.
    - **What is unchanged.** Checkpoint 7d's `src/`-scoped `Command::new("git")` ban and 7c's ban both pass, and so
      does Checkpoint 6's production `process::Command` line: HEAD's 14 hits, none added. No line's pass or fail state
      moves.
    - **Not amended here.** `s6` is closed and `ci.yml` is not this stage's. Folding the three into one shared test
      helper would still add one site. The narrowing belongs to whoever discharges row 6's mark.
23. *(dated 2026-09-27, s8d-ii build, P7.)* **A presence assertion is broken by a removal, which an added-line sieve
    cannot see.** The sweep therefore also checked removals:
    - every identifier that left its file;
    - every test name P1's baseline log carries and this log does not.

    Neither is named by any older checkpoint's bash line. The one gone test name,
    `an_existing_worktree_path_is_refused_not_reused`, and `recreate_run_worktree` appear only in the prose of
    `s7d`'s tables and `s8a`, never in a block. Of the older blocks' tokens, 765 resolve to entry test names, and none
    is missing now.

*(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* **Refuting round 1** (five finders, two refuters per blocker or major; the raw findings in the
session scratchpad, `s8dii-r1.json`):

24. **On the existing-branch arm, the filter read, the scan's path set and the length rule read start's `HEAD`,
    not the tip the tree checks out** (filed blocker; both refuters: major, one voting it refuted as a blocker because
    the code followed the stage text's *"at the fork commit `RK_BASE` records"*). A probe created a tree holding a
    pointer file with no refusal. *Closed by D21-89 ruling 1*, built red first (§8d-ii.8's R1FIX-8dii note).
25. **`RK_BASE` recorded start's moved `HEAD` as the fork on the existing-branch arm**, so the review reference tree
    would sit at the wrong base (major; one refuter minor, as P3 had disclosed it). *Closed by D21-89 ruling 2:
    `merge-base(start HEAD, tip)`.*
26. **The platform-limit tests sized their fixtures with `path_bound` itself** (major, both refuters), so a mutant
    dropping `.review-ref` + 1 survived the rk-git package. *Fixed:* literal terms, measured surviving before and
    killed after.
27. *Minors, each fixed or recorded:* (a) an incomplete discard kept the run's own lock: *fixed*, red first;
    (b) read (3) assumed the admin dir is `worktrees/<item>`, which git suffixes on a basename collision: *fixed* by
    predicting git's name, the between-read-and-add race *recorded* (X2 the backstop); (c) README's *"sixty-eight are
    partial"*: *fixed*, 75 − 4 = 71, by a dated recount; (d) ADR-0180 omitted ADR-0127 from `related` and `source`
    and dropped ADR-0162's bold on **unlike**: *fixed* (and the same quotation in §8d-ii.3); (e) §8d-ii.8's count gave
    the wrong reason for 53 and §8d-ii.0 item 8 still said *plus 50*: *fixed* by dated edits; (f) the amended
    Checkpoint 7f's catalog-load failure line left its clone: *fixed* by a dated amendment in `s7f`, its other lines
    untouched and nothing replayed (AFK rule); (g) the stage text named two of the five nested re-runs 8d skips:
    *fixed*; (h) no `rk-cli` test held the two values `rk-cli` hands `rk-git`: *two pins added*, each shown able to
    fail.

*(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)* **Refuting round 2** (five finders, two refuters per blocker or major; the raw findings in the session
scratchpad, `s8dii-r2.json`): **dry** — no blocker and no major. Its minors, each fixed or recorded (§8d-ii.8's
R2FIX-8dii note):

28. **On the existing-branch arm the reference tree's commit was neither bounded nor read** (two finders). The run
    tree checks out the tip, the reference tree the merge-base, and the `.review-ref` term was added to the tip's
    longest path. *Fixed:* the length rule reads both commits and V1's read runs over the merge-base too, each
    refusal naming the commit; built red first, both tip-only mutants killed. *Recorded:* the second read resolves
    drivers as the run tree does, and X2's scan never reads the reference tree.
29. **The merge-base test could not tell a merge-base from `tip^` or `HEAD^`** (two finders). *Fixed:* two commits a
    side, `RK_BASE` compared with the fixture's own `git merge-base`; both parent mutants survived before and are
    killed after.
30. **`reference_worktree`'s doc and §8d-ii.3 still said `GitOp` carries no `merge-base`** (two finders). *Fixed* by
    dated edits.
31. **ADR-0180 decision 12 applied "the fork commit" to read (2), a configuration read.** *Fixed:* read (1) only.
32. **ADR-0180's title and README index row did not name D21-88 or D21-89.** *Fixed* by dated clauses; the census
    (180 files; 175/4/1) does not move.
33. **Checkpoint 8d-ii's W4 bullet and `path_bound`'s doc said "the fork commit"**, which after D21-89 is not the
    commit read on the existing-branch arm. *Fixed:* wording only, no guard's meaning moves.
34. **§8d-ii.7, `s7f`'s prose note and `PROGRESS.md`'s 7f row said three amendments**; round 1 made a fourth.
    *Fixed* by dated notes; the PROGRESS cell stays one line.
35. **The ledger's title read D21-1 … D21-86 and 2026-09-27** while the file carried D21-89, dated 2026-09-28.
    *Fixed* by a dated title correction, recounted (`grep -c '^## D21-'` → 89).

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
  nothing. *(W2; ADR-0162 decision 10)* *(dated 2026-09-29, s8f build (R1FIX-8f-B-RECORDS), under D21-47's
  time-bound rule: "forces nothing" means no forced worktree removal. `s8f`'s re-entry discard runs the pathspec
  `git clean -f -- <path>…` that `s8f` §8f.4 prescribes, and (3)'s `worktree.rs` floor drops a code line whose argument
  list opens `["clean", "-f",`, with a control that keeps a `worktree remove -f` list and drops the `clean -f` one.)*
- **The filter check refuses only when pointer files would result, reading attributes from ~~the fork commit~~ the
  commit the tree checks out, and a
  post-checkout scan refuses by name and removes the fresh tree when a selected path holds a pointer anyway** ((0d),
  (2), (3)). `init` writes its block once. *(W4; R8; D21-47 V1; D21-48 X2)* *(dated 2026-09-28, s8d-ii build, package
  R2FIX-8dii; refuting round 2: wording only, no guard's meaning moves. The commit checked out is start's `HEAD` on a
  first creation and the branch tip on an existing branch, D21-89 ruling 1, as §8d-ii.5 defines `<fork>`; on that arm
  the read also runs over the merge-base the reference tree checks out, and the length rule reads both, §8d-ii.6.
  (3)'s `--source` floor still greps a code line only; the behaviour is (2)'s selections.)*
- *(dated 2026-09-27, s8d-ii build, package D2188.)* **A fresh tree whose carry or `setup` fails is rolled back, unforced and through the pre-unlink, and a
  reused tree never is** ((2)'s three D21-88 selections). *(D21-88)*
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
  skips its own re-runs of ~~8c and 7g~~ 8c, 8b, 7g, 7a and 5 *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)* when this block replays it. The card precondition sits inside the non-nested
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

*(dated 2026-09-27, s8d-ii build, package P1-RECORD.)* **The BEFORE readings, taken on the entry tree `83cf836`** in the
private tree `target/wt-p1rec` (read-only lines copied from the block, plus (2)'s selections through the no-dispatch
helper), each RED as listed: (2) every selection tried (`rk-git` two names, `rk-cli` two, `rk-containment` one) ran
**0 tests** (*"test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out"* for `rk-git`), which
`sel_tests` refuses; (3)'s root floor, 12 outside derivations (`run.rs` 4, `rk-cli` tests 6, and `s8c`'s two
`worktree_include` lines, §8d-ii.0's dated finding), and *"rk-git's worktree module names no worktrees directory"*;
`recreate_run_worktree` on 2 code lines; the force floor, 2 hits outside `remove_reference_worktree`; the `LC_ALL`,
`RK_BASE`, `--show-scope`, `--source`, LFS pointer-header and `# redkiln-runtime` floors, each absent; (7b), no `git
clone … --no-local`, no `mktemp -d`, 7f's cleanup and prune lines present, and 3 runs reads naming no clone; and
`REC_WT` read `____`, so the block refused before (9). Two (3) lines were GREEN at entry, as their form requires:
`rk-git`'s worktree module forces nothing, and nothing narrows a config read to `--local`. After this package,
`REC_WT` is `0180`.

*(dated 2026-09-27, s8d-ii build, package P7-GUARD-SWEEP.)* **Pre-checkpoint diagnostic on the built tree (not the
formal reading).** The block's lines were run as follows; every reading passed.
- **(0) to (0d), (3), (6), (8) and (9) in the main checkout.** These were extracted verbatim (`p7guard_mkdiag.py`),
  with `scratch` re-pointed to the package scratch and (1), (2), (4), (5) and (7) cut. The run read rc 0 and
  *"CP8D-II-END-OF-BLOCK reached"*:
  - the legacy ref at `73c087a…` with its stage-less mark;
  - `dispatch_completed` 0 in the store and 0 in the journals;
  - the warm `--offline` pnpm install;
  - the ignore row, with its control;
  - worst case 255 here and 246 in 7f's clone;
  - 0 filters selected at HEAD;
  - 0 pointer files, with both scan controls firing;
  - every (3) floor and in-block control;
  - `REC_WT=0180` and (9) whole.
- **(1), (2) and (5) in the private worktree `target/wt-p7`.** They were extracted verbatim (`p7guard_mkcargo.py`) and
  run under the shared wrapper. The run read rc 0 and reached its end marker, with `nd_check` clean:
  - the build;
  - the zero-match control;
  - `sel_tests rk-git 42`, `rk-containment 1` and `rk-cli 10`;
  - fmt `--check`;
  - clippy `--workspace --all-targets`.
- **(4) in the private worktree:** `cargo test --workspace --no-fail-fast` read rc 0 and **1021 passed / 0 failed / 10
  ignored over 44 result lines**, against `FLOOR=1021`.
- **(3)'s floors over the private worktree, with a planted violation for each** (`p7guard_floor_controls.py`). The
  tree passed, and each plant failed the block with its own message:
  - a `run.rs` derivation;
  - a `recreate_run_worktree` function;
  - a `"--force"` in `run.rs` outside `remove_reference_worktree`;
  - a `"--force"` in `worktree.rs`;
  - `LC_ALL` set to `X`;
  - a `"--local"`;
  - the `# redkiln-runtime` marker renamed.
- **Not run:** (7a) and (7b), the replays of 8d and 7f. They run a `redkiln` binary against this repository or its
  clone, so they are left to the formal checkpoint. (7b)'s text guards were run by P6.

**D21-49, the flip-stop assertions.** None of this block's lines requires `status: accepted`. (9) reads the status and
branches on it: while the record is proposed, no back-link may exist; once it is accepted, all three must. The replays
(8d and 7f) assert no record status either, as a grep of their blocks for `status` and `accepted` shows. So the block
passes before the flip and again after it. *(dated 2026-09-28, `s8d-ii` close: it passed before the flip, the formal reading at `497266b` with the record `proposed` (D21-90); ADR-0180 flipped `accepted` on D21-91, and the block is re-run after the flip commit.)*

**Older checkpoints' guard lines, run READ-ONLY; no line's result moved.**
- **Sieve.** Every `grep`, `rk_absent` and `rk_control` pattern in the blocks of Checkpoints 5, 6, 7a–7i and 8a–8d
  (786 patterns) was matched against the 6,234 lines this stage adds under `crates/` and `.redkiln/`. The untracked
  `run_worktree.rs` was counted whole. 48 candidates came back, and each was read in context (`p7guard_sieve.py`).
- **Differential.** Each ban whose scope could hold a candidate was run verbatim from its stage file's bytes over
  HEAD's tree (by `git archive`) and over the working tree (`p7guard_bandiff.py`, adapted from `s8d`'s). Every segment
  printed **0 new lines**:

  | segment | base FAIL | now FAIL | now PASS |
  | --- | --- | --- | --- |
  | 5's `ChangeSummary` ban | 0 | 0 | — |
  | 8a's `ChangeSummary` ban | 0 | 0 | — |
  | 8a's `"git"` literal and `rk_git` route | 0 | 0 | — |
  | 7b's ordinal, lexical and leave-the-model bans | 0 | 0 | — |
  | 7b's shipped-catalog line | 0 | 0 | — |
  | 7b-ii's classifier purity | 0 | 0 | — |
  | 7c's `rk_absent` bans | 0 | 0 | 40 |
  | 7c's `unsafe` and manifest lines | 0 | 0 | 5 |
  | 7d's absence bans | 0 | 0 | 37 |
  | 8b's `spikes/p9` sweep | 0 | 0 | — |

  **Controls:** a copy of the tree with one planted hit per segment prints a new FAIL in nine of the ten segments. The
  exception is 7b's shipped-catalog line, a presence assertion that no addition can turn red. The ten segments'
  plants were a second `unsafe` module in `rk-containment` for 7c's `unsafe` lines, and s8d's seven plants for the
  rest.
- **Checkpoint 6's three `crates` lines,** HEAD against now, with the untracked file included. The production
  `process::Command` line has the same 14 hits. `env::current_dir` in `src/` has 0. The positives are
  `Command::new("git")`=3, hooksPath=3, ifexists=7 and gix=1. The workspace-wide git line adds three test-helper hits
  to a line already red: finding 22.
- **Run verbatim from their own bytes in the main checkout** (`p7guard_seg.py`, which refuses any cargo, `redkiln` or
  git-mutating line). Each passed:
  - 8a's `GitOp` pin and exec-route segment (lines 1311–1370), with its controls;
  - 8d's `harness_binary` fixture guard (lines 3185–3192), with its control;
  - 7d's static presence floors (lines 3151–3194): `DENYLIST`, the literal list, `INIT_UNREAD_WINDOW_RUNS` and the
    report schema.
- **`ci.yml`'s read-only grep steps,** run with a private index copy that carries the untracked files, over HEAD and
  over the built tree (`p7guard_ci.py`). The outputs match except for finding 22's three lines:
  - AC-362, AC-442, AC-529 and the port inventory pass;
  - corpus residency and the image-tag step are red at HEAD and red, identically, now.
- **Removals and renames** (finding 23): none is named by an older block.
- **Selections.** `s8c`'s five `worktree_include::` selections exist once each and pass in the suite log.
- **Tools.** `check-refs`, `check-bash-syntax` (121 blocks), `check-checkpoints` and `check-adr-options` (17 records)
  each flag 0.

**The real repository after the sweep** matched its SO-1 readings on every point:
- the worktree path set;
- `.redkiln/worktrees` still absent;
- `rk-run/rk-fix-doctor-test-count` at `73c087a82245…`, with its `Story:` mark and registration;
- the run-record count at 31;
- `dispatch_completed` at 0 and 0;
- the store's sha256 at `e83d295e…`, backed up first to `target/store-backups/p7guardsweep-start.sqlite3`.

Only the private worktrees were added and removed: `target/wt-p7`, and `target/wt-p7h` at HEAD for the `ci.yml`
baseline. `git worktree remove --force target/wt-p7` left the pnpm junction skeleton that `s8c` §8c.12 item 3
describes: 114 links, 0 files and 65 directories. A walk that unlinks each link and never follows one removed it
(`p7guard_rmresidue.py`).

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
# (dated 2026-09-27, s8d-ii build, package P1-RECORD) The worktree record took 0180 at writing: ADR-0180,
# references/adr/0180-the-run-worktree-lives-in-dot-redkiln-is-reused-and-guarded.md, status proposed, named by
# ledger entry D21-87. This line read REC_WT=____ until then.
REC_WT=0180
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

# (dated 2026-09-27, s8d-ii build, package D2188) rk-git 42 -> 44 and rk-cli 10 -> 11: D21-88 ruling 2's three rollback tests.
# (dated 2026-09-28, s8d-ii build, package R1FIX-8dii) rk-git 44 -> 50 and rk-cli 11 -> 13: round 1's eight (D21-89 and the minors).
# (dated 2026-09-28, s8d-ii build, package R2FIX-8dii) rk-git 50 -> 52: round 2's two (the merge-base the reference tree checks out).
sel_tests rk-git 52 worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused worktree::an_item_id_that_case_folds_onto_a_registered_tree_is_refused worktree::the_root_is_the_main_worktree_when_run_from_a_linked_worktree worktree::a_bare_repository_is_refused_by_name worktree::a_symlinked_redkiln_or_worktrees_directory_is_refused worktree::a_created_worktree_whose_realpath_leaves_the_root_is_refused worktree::creation_refuses_when_the_worktree_root_is_not_ignored worktree::creation_refuses_a_path_past_the_platform_limit_by_name worktree::the_length_rule_reads_the_actual_item_length worktree::creation_writes_rk_base_into_the_worktree_gitdir worktree::creation_locks_the_worktree_with_the_pid_and_start_reason worktree::a_lock_held_by_a_dead_pid_is_released worktree::a_lock_whose_pid_now_names_another_process_is_released worktree::a_lock_held_by_a_live_pid_refuses_by_name worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip worktree::reentry_discards_in_flight_changes_and_reports_each_path worktree::reentry_keeps_ignored_files worktree::a_tree_without_rk_base_is_refused_and_never_removed worktree::a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed worktree::a_tree_registered_to_another_repository_is_refused worktree::a_tree_on_a_detached_head_is_refused_at_reentry_and_kept worktree::an_existing_run_branch_is_never_reset worktree::a_dirty_tree_is_kept_with_its_reason worktree::an_unlanded_tree_is_kept_with_its_reason worktree::a_locked_tree_is_not_removed worktree::a_clean_landed_marked_unlocked_tree_is_removed worktree::removal_never_follows_a_junction worktree::a_selected_filter_with_no_driver_at_any_scope_refuses_naming_the_path worktree::a_smudge_skipped_at_local_scope_refuses_naming_the_scope worktree::a_smudge_skipped_at_global_scope_refuses_naming_the_scope worktree::a_smudge_skipped_at_system_scope_refuses_naming_the_scope worktree::git_lfs_skip_smudge_in_the_environment_refuses_by_name worktree::an_include_if_that_skips_the_smudge_for_the_run_tree_refuses_naming_the_included_file worktree::the_filter_attribute_is_read_from_the_fork_commit worktree::a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently worktree::a_filter_declared_but_selected_by_no_path_does_not_refuse worktree::a_repo_with_no_filter_or_include_if_creates worktree::a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree worktree::the_pointer_scan_never_removes_a_tree_this_call_did_not_create worktree::a_pointer_header_at_a_path_no_filter_selects_does_not_refuse worktree::a_committed_pointer_that_the_driver_smudges_creates_silently gitops::every_git_child_runs_with_lc_all_c worktree::a_fresh_tree_whose_carry_failed_is_rolled_back_with_its_links_unlinked worktree::a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back worktree::an_existing_run_branch_whose_tip_selects_an_ineffective_driver_refuses worktree::an_existing_run_branch_whose_tip_leaves_a_pointer_is_scanned_refused_and_rolled_back worktree::an_existing_run_branch_whose_tip_is_past_the_path_limit_refuses worktree::a_tree_created_on_an_existing_run_branch_records_the_merge_base_as_its_fork worktree::a_reentry_whose_discard_is_incomplete_releases_its_own_lock_and_keeps_the_tree worktree::an_include_if_is_matched_against_the_suffixed_gitdir_git_assigns worktree::an_existing_run_branch_whose_merge_base_is_past_the_path_limit_refuses worktree::an_existing_run_branch_whose_merge_base_selects_an_ineffective_driver_refuses
sel_tests rk-containment 1 containment::process_start_time_reads_a_live_child_and_nothing_for_an_exited_one
sel_tests rk-cli 13 run_worktree::a_run_tree_locked_by_a_live_process_refuses_the_next_run_by_name run_worktree::git_lfs_skip_smudge_in_runs_environment_refuses_creation_by_name run_worktree::a_fresh_tree_whose_setup_failed_is_rolled_back_and_the_next_run_recarries run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees run_worktree::no_redkiln_reader_sees_a_planted_run_worktree run_worktree::creation_carries_worktreeinclude_files_and_runs_setup run_worktree::a_failed_setup_refuses_by_name run_worktree::the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves run_worktree::a_leftover_reference_tree_without_rk_base_is_refused_and_kept init::init_writes_the_redkiln_runtime_exclude_block_once init::init_leaves_the_operators_exclude_lines_untouched run::ac438_table_driven_exit_codes_over_fixtures_binary_arm cost::cost_after_a_real_run_renders_low_sample_through_the_binary
nd_check "named selections" || exit 1

# ---- (3) CODE-LINE FLOORS beneath section 8d-ii.8's behavioural tests — never the assertion.
test -f crates/rk-git/src/worktree.rs && test -f crates/rk-cli/src/verbs/run.rs && test -f crates/rk-git/src/git_exec.rs && test -f crates/rk-cli/src/verbs/init.rs || { echo "FAIL: a source file these floors read is absent (Halt 8)"; exit 1; }
# One derivation of the root. No code line under crates/*/src OR crates/*/tests derives a worktrees directory in any
# spelling — join("worktrees"), join(".redkiln/worktrees"), a joined literal — outside rk-git's worktree module and its
# own tests. Exempt: reads of git's OWN admin directory, <common-dir>/worktrees, which does not move.
ROOT_RE='"worktrees"|join\("[^"]*worktrees|"[^"]*redkiln[^"]*worktrees[^"]*"'
GIT_ADMIN='common(_dir)?(\([^)]*\))?\.join\("worktrees"\)|"\.git/worktrees/'
OWN='^crates/rk-git/(src|tests/it)/worktree\.rs:'
# (dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE; D21-47's time-bound rule: "A time-bound guard in an earlier
# checkpoint is amended by the stage that moves its premise, in the same commit.") s8f's rk-store backlog test plants a
# named item's copy under a run tree for its reader NOT to find, and rk-store depends on neither rk-ports nor rk-git
# (s8f section 8f.6 item 4), so it cannot take the path from rk-git's one derivation. Exactly that planted-path
# argument form in exactly that file is admitted; s8f's rk-cli doctor_artifact test now derives its tree through
# rk-git (crate::run_tree), and every other spelling in either file stays banned. Control below.
STORE_PLANT='^crates/rk-store/tests/it/backlog\.rs:[0-9]+:[[:space:]]*"\.redkiln/worktrees/x/\.redkiln/items/[^"]*",$'
raw=$(grep -rnE "$ROOT_RE" crates/*/src crates/*/tests --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the root search could not run (grep rc=$rc)"; exit 1; }
# (dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE) the filter gains `grep -vE "$STORE_PLANT"`.
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -vE "$OWN" | grep -vE "$GIT_ADMIN" | grep -vE "$STORE_PLANT" | grep .)
# CONTROL (dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE): the planted-path argument in backlog.rs is DROPPED;
# a derivation in the same file and a joined literal in rk-cli's doctor_artifact test are KEPT.
ctl=$(printf '%s\n' 'crates/rk-store/tests/it/backlog.rs:116:        ".redkiln/worktrees/x/.redkiln/items/in-worktree.toml",' 'crates/rk-store/tests/it/backlog.rs:9:    let t = root.join(".redkiln/worktrees").join("x");' 'crates/rk-cli/tests/it/doctor_artifact.rs:68:        self.root().join(".redkiln").join("worktrees").join(item)' | grep -E "$ROOT_RE" | grep -vE "$CODE_ONLY" | grep -vE "$OWN" | grep -vE "$GIT_ADMIN" | grep -vE "$STORE_PLANT" | grep -c .)
[ "$ctl" -eq 2 ] || { echo "FAIL: CONTROL — the store-plant exclusion kept $ctl of two derivations and one planted argument"; exit 1; }
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
# (dated 2026-09-29, s8f build: "forces nothing" reads worktree removals; s8f's pathspec `git clean -f` is admitted below.)
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
# (dated 2026-09-29, s8f build, package R1FIX-8f-B-RECORDS; D21-47's time-bound rule: "A time-bound guard in an earlier
# checkpoint is amended by the stage that moves its premise, in the same commit.") s8f's re-entry discard runs the
# pathspec clean s8f section 8f.4 prescribes ("`clean -f -- <path>...` with literal pathspecs when a baseline exists"),
# and its "-f" is git clean's, not a worktree removal's. This floor reads worktree-removal forms only: a code line
# whose argument list opens ["clean", "-f", is dropped; "--force", remove_dir_all and every other "-f" (a
# `worktree remove -f` included) stay banned. The line read:
# code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep .)
CLEAN_F='\[[[:space:]]*"clean",[[:space:]]*"-f",'
code=$(printf '%s\n' "$raw" | grep -vE "$LINE_ONLY" | grep -vE "$CLEAN_F" | grep .)
# CONTROL (dated 2026-09-29, s8f build): a `worktree remove -f` argument list is KEPT, a `clean -f` one is DROPPED.
ctl=$(printf '%s\n' '1523:            let mut args: Vec<&str> = vec!["clean", "-f", "--quiet", "--"];' '900:    exec.run(GitOp::WorktreeRemove, at, ["worktree", "remove", "-f", p])?;' | grep -E '"--force"|"-f"|remove_dir_all' | grep -vE "$LINE_ONLY" | grep -vE "$CLEAN_F")
[ "$(printf '%s\n' "$ctl" | grep -c .)" -eq 1 ] && printf '%s\n' "$ctl" | grep -qF '"worktree", "remove", "-f"' || { echo "FAIL: CONTROL — the clean-f exclusion did not keep worktree remove -f and drop clean -f: $ctl"; exit 1; }
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
# ---- recorded count plus 50 (section 8d-ii.0 item 8): ~~914 + 50 = 964~~ 971 + 50 = 1021 (dated 2026-09-27, s8d-ii
# ---- build, package P1-RECORD: Checkpoint 8d's recorded count is 971, D21-78; the stage-open baseline read 971)
# ---- on the final chain (D21-48 X2, 2026-09-24:
# ---- 46 -> 50; D21-48 floor propagation, 2026-09-24: was 901 + 50 = 951, before s8c 78 and s8d 62). Replace it by a dated edit before the first run.
# ---- (dated 2026-09-27, s8d-ii build, package D2188) re-derived from the same recorded 971: 971 + 53 = 1024, D21-88
# ---- ruling 2 adding three tests (section 8d-ii.8's count). It read FLOOR=1021 before.
# ---- (dated 2026-09-28, s8d-ii build, package R1FIX-8dii) re-derived from the same recorded 971: 971 + 61 = 1032,
# ---- refuting round 1's eight tests (section 8d-ii.8's count). It read FLOOR=1024 before.
# ---- (dated 2026-09-28, s8d-ii build, package R2FIX-8dii) re-derived from the same recorded 971: 971 + 63 = 1034,
# ---- refuting round 2's two tests (section 8d-ii.8's count). It read FLOOR=1032 before.
FLOOR=1034
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

  # (7a) CHECKPOINT 8d, the entry gate, whole. s8d's block honours RK_CP_NESTED=1 and skips its own re-runs of 8c, 8b,
  # 7g, 7a and 5 (dated 2026-09-28, s8d-ii build, package R1FIX-8dii: this read "of 8c and 7g").
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
