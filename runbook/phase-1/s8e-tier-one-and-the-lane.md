# Step 8e — Tier-1 writes git-first, the lane across processes, `Slice-Verdict:`, and the run worktree (Phase 1, part 8e)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8e` | Phase 1 | Checkpoint 8d | Checkpoint 8e | `s8d` | `s8f` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s7h §7h.2`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45, at `a6349b6`.* What changed, and where:
- **D21-44:** the run worktree moves to `.redkiln/worktrees/<item>`. The record and every derivation it moves are in
  §8e.10. The walks over `.redkiln/` and their controls are in §8e.10 too, and the worst-case path length is
  measured in §8e.0.
- **D21-45:** W1 (all), W2 (reuse, never force-remove, which replaces `recreate_run_worktree`) and W4 (the
  filter-driver refusal, plus `init`'s `# redkiln-runtime` exclude block) are in §8e.11. That section also holds W3's
  carry at creation, which calls `s8c`'s reader and `setup` verb.
- **N5:** the interim `item` key, with `query` honouring `ByItem`, is §8e.12. **N6:** every gate transition is
  git-first, and the `Gate:` value carries its state (§8e.7, §8e.3). **N7:** the planner reads the fold, and the
  `git_op` row settles the replay-journal residual (§8e.3, §8e.5). **N8:** the deadline is integer unix seconds
  (§8e.7). **N9:** the terminal `closed` mark (§8e.6). **N10:** the boundary seam a content checkpoint rides (§8e.3),
  with the staging rule cited to `s8f`.
- **Made explicit:** the extra supersessions this file found (the records table). **Fixed:** the 7h guard (10a)
  backtick defect (§8e.15). **Re-measured:** the `fsm.rs` comment and the `story_trailer` subject claim (§8e.15).
- **Checkpoint 8e:** re-runs only its entry gate plus the checkpoints this stage's own edits make provisional (7a, 7h
  and 7f). It skips its nested re-runs when a later checkpoint calls it as an entry gate (§8e.17). Its floor is
  re-derived at stage open.

---

**Why this step exists.** Four things the lane needs are recorded as missing, and they are one piece of work
because they meet at the same boundary act. **Measured at `ca8f387`, 2026-09-24, read-only, and re-read at
`a6349b6` wherever this revision cites a fact for the first time:**

- **The tier-1 store layer has no writer.** `crates/rk-store/src/migrations/0001_initial.sql` declares
  `t1_stage_transition`, `t1_gate` and `t1_slice_seal` under the header *"These rows never exist without their trailer
  commit. A failed git write leaves ZERO of them (AC-160..AC-163), which is why every one is anchored to a commit
  sha."* The `EventStore` port is still `append` / `query` / `snapshot` / `rebuild`, and `SqliteStore::append` writes
  `"INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2, NULL)"`. ADR-0158 records this and routes it:
  *"a tier-1 write method on `EventStore` (with its fake and conformance row), called from the same place the trailer
  commit is made, taking the `anchor_sha` the commit returns"*.
- **Every tier-1 write on the tree is store-first.** `Runtime::step`'s advance arm is
  `self.append(stage_transition_event(from, to))?;`, and the `Story:` commit comes later, in `rk-cli`'s
  `story_trailer`, after `resume` has returned. `resume` appends the stage-entry row through `rt.append` with no
  trailer at all. `answer` appends `outcome.events` (the `gate_answered` row) **before** it calls
  `commit_with_trailers`. ADR-0109 decision 2 says tier 1 *"is written to git first"*, and its forbids list says
  *"Forbids a tier-1 transition recorded only in the store"*.
- **A lane does not cross processes.** ADR-0157 decision 4: *"Across processes, a second invocation reports `Done`
  and exits 0 having done nothing"*, with the route *"named here as OWED"*. The stall is in the seed:
  `through_step_seven` reads the stage from the item record (*"the stage is read once, from the record, and is not
  defaulted"*) and **discards the reconcile it performs** — the call is `let _ = startup` … `.reconcile(&repo.exec,
  &repo.worktree, entry)`. So the `Reconciled<State>` ADR-0109 decision 1 makes the planner's only input is built on
  every entry and thrown away.
- **`Slice-Verdict:` has no producer.** The ARMED lint still reads `#[ignore = "ARMED: Slice-Verdict has 0
  producers; Gate and Story are both minted as of s7h.2; discharged by s7h"]`, and ADR-0160 names the route: *"declare
  the slice on the item record (or decide that a fix lane has exactly one, and say so), then a `slice_sealed`
  producer at the review stage's boundary, then the trailer through the same `commit_with_trailers` path `Story:` now
  uses."*
- **The run worktree cannot be edited by the implementer, and it is destroyed at every run start.** D21-44 measured
  it: a `-p` child with `--permission-mode acceptEdits --permission-prompts none` left `probe.txt` reading
  **`original`** under `.git/redkiln/worktrees/probe-n1-in`, with *"**Edit AND Write**, both refused"*, while the same
  dispatch under `.redkiln/worktrees/probe-n1b` wrote it. And `recreate_run_worktree` opens with
  `["worktree", "remove", "--force", path]` and then `let _ = std::fs::remove_dir_all(&path);` on every run. So a
  resumed run's tree never survives to be resumed, which D21-45 W2 reverses.
- **The lane loop has no automated test.** `grep -rn "verbs::run::run(" crates/*/tests` returns nothing. ADR-0161
  names the cost: two of its three defects *"are in the seam between the kernel's report and the composition root's
  loop — which no unit fixture in either crate covers"*.

**This step takes decisions, and they were already taken.** The operator took them in D21-41 (a) and (e), D21-42 7,
8a–8d, 24b and 24c, D21-43 N5–N10, D21-44, and D21-45 W1, W2 and W4 (`spikes/DECISIONS-2026-09-21.md`). They are
written as records at this stage, and each record takes **the next free id at writing** — this file assigns none.
Where a decision moves a `## Decision` sentence or a forbids bullet of an accepted record, the record is a supersession
in part (ADR-0092 rule 1), never an erratum. Every record is written `proposed` and flips only on the operator's dated
ledger line. Where this file found a residual the decisions do not name, it says so in §8e.15 rather than taking it
silently.

**What this step owns** (the plan's row for `s8e`, widened by the three later entries):
- D21-41 **a**, and **e** in both halves — `Slice-Verdict:` AND the tier-1 writer.
- D21-42 **7**, **8a**, **8b**, **8c**, **8d**, **24b** (L3) and **24c** (`terminal_read` plus the first FakeHarness
  end-to-end test of `verbs::run::run`).
- D21-43 **N5**, **N6**, **N7**, **N8** and **N9**, and **N10**'s seam (the staging rule is `s8f`'s).
- **D21-44** whole (its *"Consequences, owned by `s8e`"*).
- D21-45 **W1**, **W2**, **W4** (the worktree half and `init`'s exclude block, *"`redkiln init`'s exclude block is
  `s8e`'s too"*), and **W3's carry at creation**.

**What it does not own, and who does.** ADR-0164's flip (the operator's ledger line).
- **`s8c`:** the `.worktreeinclude` reader and the per-toolchain `setup` verb (D21-45 W3; D21-43 N25). This stage
  calls both at creation and re-specifies neither.
- **`s8f`:** the landing checkpoint commit and its staging rule (D21-41 b; D21-42 9; D21-43 N10). This stage builds
  the seam the content act rides (§8e.3) and says what it guarantees. `s8f` also owns `dispatch_completed`'s
  `activity_id_at_version` and the cost breakdown (D21-42 12b), and cites §8e.12 for the general `item` key.
- **`s8g`:** the verify record and `verify_completed`.
- **`s8h`:** cards, models, the launcher and the run environment. That includes the `gate_expiry` table (D21-42 21a), to
  which this stage adds no row because it mints no gate kind (§8e.7). It also includes the live probe's edit inside
  `.redkiln/worktrees/<probe>`, and the protected-segment limit (`<wt>/.git`, `.cargo`, `.claude`, `.husky`) mapped
  to the typed `permission_denials` failure (D21-45).
- **`s8a`:** how the `Text` variant classifies an LFS pointer path. §8e.11's filter-driver refusal is what it
  verifies against.
- **`s8b`:** its own catalog-quality walks over `.redkiln/`, which exclude `worktrees/` (D21-44).
- **`s8j`:** the fresh fix item and the story item (D21-42 18; D21-41 f), each carrying `slice` (§8e.8) and
  `[project] name`. `s8j` also re-points or retires the legacy `rk-fix-doctor-test-count` tree (D21-44) and re-points
  Checkpoints 7h and 7i to fixture items. And it owns every amendment to `s8-exit-criterion.md`: the mechanical
  set-and-order assertion D21-42 7 puts on criterion (1), the anchored differential in (2) (§8e.9), `s8`'s
  `grep -r … .redkiln/` lines excluding `worktrees/` (§8e.10), and the three `redkiln run` sites through the launcher.

**No stage before `s8i` dispatches to a real account, and this one dispatches nothing.** Every run in it is over a
fixture repository, a scripted `FakeHarness`, or the fixture child.

**Order of work, and why.**
1. §8e.0's measurements, before any code.
2. §8e.1: D21-42 24b says L3 is fixed *"before the writer builds on it"*.
3. The run worktree's placement and lifecycle (§8e.10, §8e.11). §8e.4's `worktree_assigned` row records the path, so
   the path must be final first.
4. The writer (§8e.2), the kernel-owned order (§8e.3) and the ownership row (§8e.4). Every mark written after that
   goes through them.
5. The interim `item` key (§8e.12), because §8e.4 and the fold read by item.
6. The three axes and the terminal mark (§8e.5–§8e.8), then the rebuild (§8e.9).

The end-to-end tests (§8e.13) are written **first, red**, and they go green last.

## Records this step writes

Each record below is **a new record (next free id at writing)**, in the ADR-0085 `### Option — <name>` heading form
that D21-42 24a makes binding from ADR-0164 onward. The adversarial pass may fold two into one. It may not drop a
supersession listed here without a reason written into the record. Every supersession listed is **explicit**, including
the ones this file found beyond the ledger's list.

| record | executes | supersedes in part | cites without superseding |
| --- | --- | --- | --- |
| **the stage lives on the run ref** | D21-41 a; D21-42 8d's two defaults; D21-43 N7 (the planner reads the fold) and N9 (the terminal mark) | ADR-0105 clause 3 (the `Story:` value delegation to design §5.4's row); ADR-0155 decisions 1, 2 and 5(a), and its bullets *"A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes."* and *"A `Story:` value carrying a stage, a boundary, or any third token, until the grammar is widened by a record that supersedes `0105` clause 3."*; ADR-0157 decisions 4 and 5, and its bullets *"Describing a lane as resumable across processes, or a lane's completion as anything other than within-one-process."* and *"Reading a second invocation's exit 0 as progress, or as evidence that the lane finished."*; ADR-0161's bullet *"Describing the lane loop as covered by tests, or reading a green suite as evidence about it."*, and only once §8e.13's tests pass | ADR-0109 decisions 1 (N7 is execution under it), 3 (**NOT superseded**: resume reads all three axes once `Slice-Verdict:` lands) and 4; ADR-0161 decision 1 (`stages_entered` keeps replays idempotent); ADR-0159 |
| **the tier-1 writer, git-first, owned by the kernel** | D21-41 e (second half); D21-42 8a, 8c, 24b's placement; D21-43 N7 (the `git_op` journal) and N10 (the seam a content act rides) | ADR-0158 decision 3 (*"No document, record or checkpoint may state that AC-162 is met, that a tier-1 row is written, or that a failed git write leaves zero store rows"*) and its bullet *"Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis"* — **only** for the criteria this stage measures (§8e.16); ADR-0155 decision 5(b) (*"No document may state that AC-162 is met for this axis, or for any."*) and its bullet *"Stating or implying that AC-162 is satisfied for stage changes 2..N of a lane"*, on the same condition | ADR-0003 (the port **count** stays five; a method is a surface change); ADR-0105 clauses 1 (N7 executes it), 2, 4, 8 and 9; ADR-0109 decision 2; ADR-0118 clause 4 |
| **`Gate:` carries kind, reach and state** | D21-42 8b; D21-43 N6 (which *"Extends D21-42 8b's record"*) and N8 | design §5.4's `Gate:` grammar row, and its *"RFC 3339"* deadline spelling (N8); ADR-0105 clause 3's value delegation, for `Gate:`; design §5's `gate_answer_void` row's tier (`2`) and producer (*"`reconcile()`"*) | ADR-0105 clause 10 (a fresh mark per re-reach); ADR-0109 decision 1 (a void answer re-opens the gate) and decision 2 (its parenthetical read as illustrative — §8e.7 says why); AC-174; `s7d` §7d.18's RAISED widening item, which this discharges |
| **the slice is declared, and sealed on both outcomes** | D21-41 e (first half) | **nothing**. It honours ADR-0160's first bullet **in full** — *"Emitting a `Slice-Verdict:` trailer whose `<slice>` is the item id, the project, the stage, the run id, or any value not declared as a slice."* — and §8e.8 refuses each of those five by name | ADR-0160 decision 6; ADR-0162 decision 2 (scoped to its own work, so re-stated here, not superseded); ADR-0161 decisions 1–2; ADR-0159 (the refusal pattern) |
| **the run worktree lives at `.redkiln/worktrees/<item>`, is reused, and is guarded** | D21-44; D21-45 W1, W2, W4, and W3's carry | ADR-0105 clause 6's *"A worktree is **re-created**, never reused, per run"*, and the *"a reused worktree"* member of its bullet *"Forbids a wrapper write not on clause 5's list, a `\\?\` path handed to git, and a reused worktree."*; ADR-0105 clause 5's enumerated write set, for the variants §8e.11 names; ADR-0137 decision 3's path literal *"`<common-dir>/redkiln/worktrees/<item>`"* — its rule, *"`project_root` is the run-owned worktree"*, stands | ADR-0018 (nothing of it moves — §8e.10 says why); ADR-0105 clause 8; ADR-0109 decision 4; AC-518 and AC-529 (dated notes); Halt 6 |
| **the interim `item` key** | D21-43 N5 | ADR-0131's first forbids bullet (*"Forbids putting an item id, a run id, a commit SHA, a stage, … inside `rk-types::Event.payload` for the purpose of addressing"*) in part, for the item-scoped members §8e.12 lists, as ADR-0163 did for `rate_limit_reading.item` | ADR-0163 (the precedent, and the move to ADR-0131's tag); ADR-0131 clause 8 |
| **`worktree_assigned`**, **`stage_empty`** and **`git_op`**, three kinds | D21-42 8d; D21-42 7; D21-43 N7 | nothing — minted by ADR-0116's route: *"The table row and the `rk-types` variant land in the same change."* | ADR-0105 clauses 1 and 8; AC-529; the design §5 `reconcile_started` row's RT-47 members (§8e.4 says what happens to them) |

**Attendant edits, each dated and each in the commit that makes it true:**
- design §5.4's `Story:` and `Gate:` rows;
- design §5's event table: three new rows, the `gate_answer_void` row, and the interim `item` member per §8e.12;
- design §5's `reconcile_started` row, whose recomputation clause names *"`<common-dir>/redkiln/worktrees/<item>/`"*;
- design §3.3's `EventStore` row (a dated note naming the tier-1 method);
- dated register notes on **AC-444** (its drift counts are pinned to the grammar table, and it is corpus-bound
  Phase-2, so the note re-pins nothing Phase 1 asserts), **AC-518** and **AC-529** (§8e.10);
- dated notes on `s7d` §7d.5's two tables that name the old path;
- a dated re-amendment of `s7h` §7h.2, whose ADR-0155 decision-6 erratum says *"the first stage boundary"*;
- the dated amendments to Checkpoints 7h and 7f that §8e.15 spells out;
- a ledger entry.

### 8e.0 Stage-open measurements — re-taken at Checkpoint 8d's close, before any code

Each is read-only. A 🛑 line is a STOP: the finding goes to the operator, and the stage does not choose around it.

1. **Where the legacy tree is registered.** `git worktree list --porcelain` at `a6349b6` lists
   `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count` at `73c087a`. Checkpoint 7f's block ends
   with `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count`, and `s8a` runs 7f whole. So by
   this stage's open the registration may already be gone (§8e.15, finding 13). Record which. Checkpoint 8e asserts
   the ref and its mark, never the registration.
2. **The worst-case run-tree path (D21-44's last consequence).** At `a6349b6` the longest tracked path is 140
   characters, `references/adr/0114-the-executed-once-gate-moves-to-extended-to-shipped-promotion-and-the-v0-pack-tags-on-lint-parse-and-a-visible-ledger.md`.
   `D:/repos/redkiln-rs/.redkiln/worktrees/` is 39 characters. The derived review tree is named
   `<item>.review-ref` (`reference_worktree`'s `format!("{item}.review-ref")`), so the worst case is 39 + 64 + 11 +
   1 + 140 = **255**. Windows' 260-character `MAX_PATH` leaves 259 usable. `git config --get core.longpaths` prints
   nothing on this box. Under the old root (43 characters) the same sum was **259**, at the limit. The worktree
   record states the bound and the rule for exceeding it (§8e.11). Checkpoint 8e recomputes the sum from the tree.
3. 🛑 **Golden logs and pinned fixtures that hold an item-scoped kind (N5).** §8e.12 adds `item` to the payload of
   every item-scoped kind. ADR-0131's cost ground is *"every golden log's canonical events change, the corpus
   regenerates, `CORPUS_PIN` moves"*. ADR-0163 escaped it because *"no golden log contains a `rate_limit_reading`
   row"*. This stage's kinds (`stage_transition`, `gate_opened` and more) almost certainly do appear in golden logs.
   Read every golden log under `CORPUS_PIN` and every `-text`-pinned fixture in `.gitattributes`, and count the rows
   per kind. If any row would move, STOP and put the cost to the operator: either regenerate and move `CORPUS_PIN`
   by record, or have the replay read a missing interim key as absent rather than as divergence, with that rule
   stated.
4. **The tier-1 tables are empty everywhere.** `s8f` §8f.2 measured `t1_*` at 0 rows in the shipped store at
   `ca8f387`. Re-read it `mode=ro`. §8e.2's writer-side resolutions rely on no existing row having to migrate.
5. **Checkpoint 7f's current literal and run arm**, as `s8a` left them: the ignored-lint reason, the exact result
   line, and the `redkiln run rk-fix-doctor-test-count --lane fix` over the product repository. §8e.15 amends all three.
6. **No code walks `.redkiln/` recursively.** At `a6349b6`, every `read_dir` under `crates/*/src` that touches
   `.redkiln/` reads one named subdirectory, one level deep: `ctx.redkiln().join("cards")` and
   `.join("specs")` in `doctor/rules.rs`, `redkiln_dir.join("skills")` in `doctor/roles.rs`,
   `root.join(".redkiln").join("catalog")` in `run.rs`'s `catalog_sources`, and `Catalog::from_dir` over a directory
   it is handed. `init`'s `preexisting_rk_names` is *"Top level only"*. Re-measure
   it. A recursive walk found here takes the exclusion and its control in the same commit (§8e.10).
7. **`init` writes no hook.** D21-45 says *"`init` already writes the hooks marker (RT-51)"*. At `a6349b6`,
   `crates/rk-cli/src/verbs/init.rs` writes `.redkiln/config.toml` and opens the store, and nothing else. The only
   `# redkiln-hook` on the tree is `doctor`'s recogniser, `const MARKER: &str = "# redkiln-hook";`. So the exclude
   block is `init`'s **first** write into the common directory (§8e.11; §8e.15, finding 10).
8. **The test module names are free.** This stage adds `rk-kernel` `tier_one`, and `rk-cli` `lane_e2e`,
   `slice_verdict`, `worktree_assigned` and `run_worktree`. It extends `rk-git` `trailers`, `worktree` and `gitops`,
   `rk-cli` `init`, `answer` and `doctor`, and `rk-kernel` `lane_order` and `fsm`. If a predecessor created one of the
   new modules, the tests go into it. The module-qualified names in §8e.14 stay as they are.
9. **The whole-suite floor.** It is re-derived at stage open from Checkpoint 8d's recorded passed count, plus this
   stage's **93**: 92 new tests, plus the one ARMED lint un-ignored. The expected value at drafting is **944**. That
   is ADR-0164's 740 (`992d257`) plus the drafted new-test counts of `s8a` (9), `s8b` (16), `s8c` (40) and `s8d` (46),
   which give 851, plus 93. Replace 944 in Checkpoint 8e's `FLOOR=` line with the re-derived value, by a dated edit,
   before the block's first run.

### 8e.1 L3 first — the writer mutex halts, and is never recovered (D21-42 24b)

`crates/rk-store/src/conn.rs`'s `Writer::transaction` still reads:

```rust
let mut guard = self
    .conn
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
```

ADR-0118 clause 3 forbids exactly this spelling (*"not by `unwrap_or_else(std::sync::PoisonError::into_inner)`"*),
and clause 4 scopes the rule to **both** locks and says of this line that *"its repair is owed"*. The tier-1 writer
§8e.2 adds runs through this mutex, so the repair lands first, in its own commit.

**The halt's shape is fixed by record and not re-decided here.** Clause 1: *"No `StepError` is constructed, nothing
crosses a port boundary, no retry"*. So the adapter's `map_err(|e| io(e.to_string()))` may not carry a poison
across the port as `StepError::Transport`. The house mechanism is the kernel's `.expect(LOCK_HALTS)` panic, which
`rk-cli`'s hook turns into exit 70 (`is_lock_halt`). `rk-store` depends on `rk-types` alone and cannot name
`rk_kernel::runtime::LOCK_HALTS`, while `is_lock_halt`'s doc forbids a second copy of the string (*"two copies can
drift, and the drift would be invisible"*). So the constant moves **down** to `rk-types`, and
`rk_kernel::runtime::LOCK_HALTS` becomes a re-export of it. That is an implementation move, not a decision: there is
still one string.

🛑 **Red first.** `store::writer_poison_is_a_halt_and_never_recovered` is written before the fix. It poisons the
writer with a transaction closure that panics, catches that one panic in the test, calls `transaction` again, and
asserts the second call panics with a payload containing the constant **and writes no row**. It is RED at
`ca8f387`: the second call recovers the guard and commits.

### 8e.2 The tier-1 writer on the port (D21-41 e, second half; ADR-0158 decision 4)

**One method, on the one port that owns persistence.** `EventStore` gains
`fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError>`. `TierOneAnchor` carries
the trailer commit's sha and its tree. The method writes the `t2_event` row with `anchor_sha` set **and**, when the
event's kind has a tier-1 table, that table's row, all in **one** transaction. The fold keeps reading `t2_event`
through `query`, so no reader changes. A tier-2 row of the same boundary act (`stage_empty`, and the step row a content
checkpoint carries under N10 — §8e.3) goes through the same method and is anchored to the same commit. The method
name is the record's. The anchored-tier-2 use argues for a wider name such as `append_anchored`. If the adversarial
pass renames it, the tests, the lint in §8e.3 and Checkpoint 8e rename in the same commit.

- **The port count stays five.** ADR-0003 says *"The kernel exposes exactly five ports"*. A method changes a port's
  surface, not the count. Checkpoint 7a is re-run whole at Checkpoint 8e, because it gates the port surface.
- **`rk-ports`' AC-364 source parse moves from four to five, in the same commit, by the record.**
  `event_store_port::ac364_event_store_port_has_no_update_or_delete` asserts `methods.len()` is `4`, and its message
  says *"a fifth is a write path a record has to admit"*. The tier-1 record is that admission. The method name
  carries none of `FORBIDDEN_VERBS`, and the append-only rule is unchanged.
- **The fake and the conformance row.** `VecStore` implements the method (it keeps its anchored rows beside its
  events). `rk_ports::conformance::conform_event_store` gains a tier-1 arm that both rows run: an anchored append is
  queryable as the event, and the row reports its anchor.
- **The SQL lives in `rk-store`, not in the adapter.** ADR-0158's bullet *"Writing `t1_*` rows from `rk-cli`, or any
  SQL against them outside `rk-store`"* stands un-superseded. So `rk-store` gains a `tier_one` module of statement
  functions over a `rusqlite::Transaction`, and `SqliteStore::append_tier1` calls them inside `Writer::transaction`.
  Checkpoint 8e asserts that no code line under any `crates/*/src/` outside `crates/rk-store/src/` names a `t1_`
  table.

**The schema frictions, resolved at the writer and not by a migration.** The brief measured two, and this file found
a third:

1. **`t1_stage_transition.tree_hash TEXT NOT NULL`.** Resolved by ADR-0105 clause 4, whose first sentence already
   answers it: *"When a checkpoint commit is due, the recorded tree is that commit's tree."* The anchor is the trailer
   commit, so the row's tree is `<anchor>^{tree}`, read through `GitOp::RevParse`, which is on the wrapper's read
   list. An empty commit's tree is its parent's, which is deterministic.
2. **`t1_stage_transition.from_stage TEXT NOT NULL`.** The lane-entry row (`stage_entry`'s `{"to": …}`) has no
   `from`, and `stage_entry`'s own doc says so. **Resolution: `t1_stage_transition` holds boundaries — rows with a
   `from` — and a lane entry is not a transition between two stages.** The entry mark's tier-1 durability is its
   `Story:` trailer plus the anchored `t2_event` row, and the rebuild reproduces exactly that split (§8e.9).
   **Rejected: a `0002` migration relaxing the column.** `rk-store`'s `SCHEMA_VERSION` is `1`, and the identity check
   refuses a store stamped lower with *"A migration from {} to {} is required; there is no implicit in-place
   migration"* (AC-180). A bump would refuse the shipped `.git/redkiln/store.sqlite3` and every fixture store, and it
   would break Checkpoint 7i's `redkiln cost` over the shipped store (§8e.15). If the record takes the migration
   anyway, it owns an explicit AC-180 migration and the 7i re-point in the same stage.
3. **`t1_gate.token TEXT NOT NULL UNIQUE` beside `answered INTEGER NOT NULL DEFAULT 0`** — found here, and now bound
   by N6: *"`t1_gate` is append-only and state is a fold."* The `answered` column can only be written by an `UPDATE`,
   which AC-364 and N6 both forbid. `UNIQUE` on `token` forbids a second row for one token, so the v1 table cannot
   hold one row per transition either. **This file's route (B): `t1_gate` holds each token's OPENING row, appended
   once and never updated. Each later transition (answered, expired, voided) is tier 1 through its state-bearing
   `Gate:` trailer (§8e.7) plus its anchored `t2_event` row. A gate's state is the fold over its opening and its
   anchored transitions.** That keeps schema version 1 and every consequence friction 2 lists. The record states that
   `answered` is never written, and design §5 carries a dated note saying so. A fresh token per re-reach (ADR-0105
   clause 10) keeps `UNIQUE` satisfiable. **Route (A), which the record may take instead:** a `0002` migration that
   re-creates `t1_gate` as one row per transition, with a `state` column and without `UNIQUE` or `answered`. It carries
   friction 2's costs whole (an explicit AC-180 migration of every store, and the 7i re-point). (B) reads N6's
   *"`t1_gate` is append-only"* literally and its *"state is a fold"* as a fold over the gate's tier-1 rows in both
   tables. **The record names the route, and the operator's ledger line confirms that reading** (§8e.15, finding 2).

### 8e.3 Git-first, owned by the kernel (D21-42 8a; D21-43 N6, N7 and N10's seam)

**The decision:** *"The kernel owns it, through the journalled `GitOps` seam. It commits (zero writes on replay),
then appends the t1 and t2 rows with `anchor_sha`. `reconcile` heals a commit without a row."*

**One boundary function, in one module.** `rk-kernel` gains `runtime/tier_one.rs` (`runtime.rs` becomes the module
root of a `runtime/` directory). Its function takes:
- a `&mut rk_git::gitops::GitOps`;
- the store;
- the `RunOwnership` read from the store's `worktree_assigned` row (§8e.4);
- one **boundary act**: an optional trailer, the act's content, and the rows the act anchors.

In order, it:

1. authorises the target through `rk_git::worktree::authorise_tier1_write`;
2. commits through `GitOps` (`--allow-empty`, the trailer, `-c core.hooksPath=<empty>` and `-c
   trailer.ifexists=add` — ADR-0105 clauses 2 and 3);
3. re-reads `HEAD` and the tree;
4. calls `append_tier1` for every row of the act, each anchored to that commit.

A failed commit returns before step 4, so no row exists. That is AC-162's second clause **by construction**, and the
fault-injection test watches it. Git is not a port (ADR-0003's bullet *"Git may not be abstracted behind a port"*), so
the `GitOps` handle travels beside `Ports`, not inside it. `resume` gains it as a parameter on the live path, as
`ReplayPorts` already carries one on the replay path.

**The seam a content checkpoint rides (N10: *"The commit and the content checkpoint are one boundary act"*).** At
Checkpoint 8e the act's content has one arm, **empty**: step 2 is the `--allow-empty` trailer commit. `s8f` adds the
landed arm. Its staging rule, and the `GitOp` variants its route needs, are `s8f`'s, and this file does not specify
them. The landed arm changes **what** step 2 commits. It does not change the order: commit, re-read, then every row
anchored to the one commit. The seam guarantees three things `s8f` builds on:
- **An act may carry several rows**, tier 1 and tier 2, each anchored to its one commit.
  `tier_one::a_boundary_anchors_every_row_of_the_act_to_its_commit` proves it with a `stage_transition` and its
  `stage_empty` in one act.
- **An act with neither a trailer nor content is refused**, because an empty commit that records nothing is noise on
  the run ref (`tier_one::an_act_with_neither_trailer_nor_content_is_refused`). A trailer-less act with content is the
  shape N10's per-step checkpoint takes, and `s8f` builds and tests it.
- **The heal below covers tier-1 marks only.** A content commit whose step row never reached the store is `s8f`'s
  landing record's to settle, and §8e.15 names it for that record.

**Every tier-1 act on the tree routes through it:**

- the lane entry (`stage_entry`, today appended with `rt.append`);
- the advance (`Runtime::step`'s `AdvanceStage` arm, today `self.append(stage_transition_event(from, to))?;`);
- the lane's end, the terminal `closed` mark (§8e.6);
- **every gate transition (N6):**
  - the opening (`OpenGate`, today appended in the kernel and committed later by `terminal_read`);
  - the answer (`answer.rs`, today append-then-commit — **the order N6 fixes**);
  - the expiry (`run.rs` step (9)'s expiry pass, today store-only);
  - the void (the kernel's `gate_answer_void_event`, today a tier-2 row with no trailer);
- the seal (§8e.8).

`story_trailer` and `terminal_read`'s commit are **deleted**, not re-ordered. ADR-0155's bullet *"Writing the
`Story:` trailer from `terminal_read`, or from anywhere after the store row is readable"* is then true by
construction.

**The void is minted by the kernel, never by `reconcile`.** ADR-0109 decision 1 says *"Reconcile mutates nothing in
git (AC-174)"* and *"A gate answer bound to a tree hash that no longer matches is void and the gate re-opens with the
drift attached (AC-176…)"*. So `reconcile` keeps **observing** the void and returns it in its `Reconciled<State>`.
The kernel's entry path (§8e.5's seed reads the same output) then mints the `voided` transition through this function.
AC-174 holds unchanged.

**Zero writes on replay, and the replay's git journal (N7).** In `Mode::Replay`, `GitOps::run` *"matches on the mode
*first* and returns the recorded result"*. The boundary's replay arm therefore performs no commit, and
`tier_one::replay_of_a_boundary_performs_zero_git_writes` reads `writes_performed()` before and after. The residual
this file's first draft left to the record is **settled by N7**: *"A tier-2 `git_op` row is minted so replay serves
git writes from the journal. Golden logs are read, by stated rule, as 'no git writes'."*
- **`git_op` is minted by ADR-0116's route** (§8e.4 lists it with the other two kinds). Its payload is
  `GitInvocation`'s own members: `op`, `cwd`, `argv_digest`, `status`, `stdout`, `stderr_digest`. The live `GitOps` is
  built over a `GitJournal` that appends one `git_op` row per invocation. That is ADR-0105 clause 1's *"records every
  git invocation and its result to the EventStore"*, executed.
- **The stated rule for golden logs:** a recorded journal that holds **no** `git_op` row is a run that performed no
  git write. The boundary's replay arm serves no commit and compares the rest of the journal as it does today. So the
  committed golden logs are **not re-recorded to go green**:
  `recorded_log::write_run_and_read_run_round_trip_every_recorded_input` and the golden-log replay stay green with
  their fixtures untouched. `tier_one::a_journal_without_git_op_rows_replays_as_no_git_writes` pins the rule, and
  `tier_one::replay_serves_the_boundary_from_its_git_op_rows` pins the other branch.
- The anchor still rides in `t2_event.anchor_sha`, a column and not part of the payload, so no existing canonical
  event moves for it.

**The heal — the converse of AC-163.** AC-163's instrument *"fails on a store row with no matching commit"*. Its
converse is a crash between step 2 and step 4: a trailer commit on `rk-run/<item>` whose sha anchors no row. At every
entry, after `reconcile_at_entry`, the kernel's `tier_one::heal` appends the missing rows for every reconciled mark
whose anchor is reachable from `rk-run/<item>` and absent from the store. This is git → store, the direction ADR-0109
decision 1 permits, and it writes nothing to git (AC-174). A mark whose anchor is unreachable heals nothing, and a
separate test covers that branch (§8e.14).

**AC-162's instruments, all three, named by the register.** *"a fault-injection test with a git backend that fails
the commit, asserting zero store rows afterwards; a second fault-injection fixture installing a hook that rewrites the
commit message, which must have no effect on the trailer commit … plus a reachability lint that the tier-1 store
writer is callable only from the commit path."* The lint is a source lint over **code lines only**, and it asserts that
every call site of `append_tier1` under `crates/*/src/` is inside `crates/rk-kernel/src/runtime/tier_one.rs`. It is a
floor. The two fault injections are the criterion, as the register says in terms. The gate transitions N6 moves
each get their own git-first test, because *a fixture per function is not a fixture per branch*:
`tier_one::a_gate_answer_is_committed_before_its_row`, `tier_one::a_gate_expiry_is_committed_before_its_row` and
`tier_one::a_gate_void_is_committed_before_its_row`, beside `rk-cli`'s `answer::a_failed_answer_commit_leaves_zero_rows`
over the real verb.

### 8e.4 The run-owned worktree is read from the store (D21-42 8d; AC-529; R-63)

**The decision:** *"A `worktree_assigned` row is written at run start. Every tier-1 write is checked against it and
refused with a typed rk-git error on mismatch."*

Today the two verbs disagree, which is R-63. `answer`'s `recorded_ownership` reads the pair off the last
`reconcile_started` row (RT-47, *"ROUTED, not decided"*). `run`'s `story_trailer` and `terminal_read` **rebuild it by
convention**: `resolved.common_dir.join("redkiln").join("worktrees").join(item)` on `format!("rk-run/{item}")`.
That convention also moves (§8e.10), which is one more reason no reader may keep it.

- **The kind.** `worktree_assigned { item, worktree, branch, reused, discarded }` is minted by ADR-0116's route: a
  design §5 row and the `rk-types` variant in one change, with `tier()`, `ALL`, the completeness and snapshot tests,
  and the no-catch-all arms in `fsm.rs`, `breaker.rs` and `memo.rs`. It is written at **every** run start, on a
  created tree and on a reused one. `reused` says which, and `discarded` lists every in-flight path the re-entry threw
  away (§8e.11, W2's *"discarded explicitly and journalled"*).
- **The deleted-database question, answered in terms by the record.** If the database is deleted, `answer` finds no
  row and refuses (fail-closed) until a `run` writes one. Under W2 that `run` re-enters the registered, marked tree
  and writes the row without re-creating anything. The record states whether that is a tier-2 disposal or a
  next-action change.
- **It carries the item, and every reader filters by it.** `recorded_ownership` filters by kind alone, and
  `SqliteStore::query` ignores its selector — `fn query(&self, _by: Query)` runs `"SELECT kind, payload FROM t2_event
  ORDER BY id"` (RT-4). With two items in one store, `answer` would take the last pair of **either** item. The target
  and the ownership would then both come from that one row, so `authorise_tier1_write`'s comparison would pass for
  the wrong run. §8e.12 makes `query` honour `ByItem`, and this reader goes through it.
- **Every tier-1 write resolves its `RunOwnership` from this row**, through §8e.3's function, and never from `cwd` or
  a path convention. A mismatch is the existing typed `GitError::ForeignWorktree { target_path, target_branch,
  owned_path, owned_branch }`, which already names all four values AC-529 asks for.
- **RT-47's pair on `reconcile_started`.** The record either withdraws those members by a dated design §5 note, or
  keeps them as tier-2 context and says no reader resolves ownership from them. It may not leave two sources.
- **AC-529 becomes real on all three of its instruments.**
  1. A second worktree of the same repository on another branch attempts the write and gets the typed refusal, with
     `HEAD` unchanged on both branches and zero new rows.
  2. The run-owned worktree succeeds, as the positive control.
  3. A test asserts that the resolution reads the store, and a mutant that resolves by convention kills it. That test
     gives the fixture a `worktree_assigned` row whose path **differs** from the convention path. With equal paths the
     mutant is unobservable (the lesson of a fixture per branch).

### 8e.5 The stage lives in git: a stage-bearing `Story:` at every boundary (D21-41 a; D21-43 N7)

**The decision:** *"A stage-bearing `Story:` trailer on `rk-run/<item>` at every boundary, folded by `reconcile`,
seeding `through_step_seven` so `run` and `answer` both see it."* N7 adds *"The planner reads `reconcile()`'s output
(execution under ADR-0109 d1), and D21-41 (a)'s stage seed comes from its fold."*

- **The grammar.** The value becomes `<project> "/" <slug> SP <stage>`.
  - `classify`'s `Story` arm splits on whitespace first. Today it calls `split_qualified(value)` on the whole value.
    Unchanged, it would parse `redkiln-rs/x analysis` as the slug `x analysis`, which is ADR-0155 option 2's failure
    mode (*"That it type-checks is what makes it dangerous"*).
  - One token is the **legacy stage-less mark**. Two tokens are a stage-bearing mark. Three or more are drift, with a
    new `MalformedReason` surfaced and never coerced (AC-183).
  - The stage token contains no whitespace and no `/`, and the producer refuses a stage that violates this at the
    mint (the ADR-0161 decision 3 pattern).
  - The token `closed` is reserved for the lane's end (§8e.6). A pack that declares a stage named `closed` is refused
    at load.
- **The fold.** `reconcile` keeps `stories_committed` (membership) and gains a latest stage per `(project, slug)`.
  Latest wins, commits are folded oldest-first, and this is the rule `Slice-Verdict:` already uses (*"Latest wins:
  commits are folded oldest-first."*). A repeated **identical** `(project, slug, stage)` stays
  `DuplicateStoryTrailer` drift. The record states the order assumption the packet flagged: `read_commits` runs
  `log --all --reverse` without `--topo-order`. For one linear run ref, the default walk emits a child before its
  parent, so `--reverse` puts the parent first.
- **The one stage-less mark on the tree is admitted, read-only** (D21-42 8d's default).
  `rk-run/rk-fix-doctor-test-count` at `73c087a` carries `Story: redkiln-rs/rk-fix-doctor-test-count`. It folds as
  membership with **no stage**, so the seed falls back to the item record, which is today's behaviour exactly. A
  stage-less mark never resets a stage a later-or-earlier stage-bearing mark set. The producer never writes one
  again.
- **The seed.** `through_step_seven` keeps the `Outcome` its reconcile returns, instead of `let _ = startup`.
  `resolved.stage` becomes the item's reconciled latest stage, and falls back to the record only when git carries no
  stage-bearing mark for the item. `answer` resolves through `run::through_step_seven`, so both verbs see it. This is
  also the first time ADR-0109 decision 1's `Reconciled<State>` reaches the planner rather than being discarded. The
  same output carries the void observations §8e.3's entry path mints.
- **The mint.** The mint is §8e.3's boundary function, at entry and at every advance. The idempotence read changes
  from *"item already committed"* to *"this `(item, stage)` already committed"*. ADR-0161's `stages_entered` guard
  still forbids re-entry within a lane.
- **`doctor --rebuild` takes `to` from the marks.** A stage-bearing mark gives its own stage. A legacy mark falls back
  to `.redkiln/items/<slug>.toml`'s `stage`, which is the current rule, and it stays listed unresolvable when the
  record has none (AC-165). A `closed` mark rebuilds as the lane's end (§8e.6).

**The two forbids bullets the first draft raised rather than took are now listed in the records table** (ADR-0157's
*"Reading a second invocation's exit 0 as progress, or as evidence that the lane finished."* and ADR-0161's
*"Describing the lane loop as covered by tests, or reading a green suite as evidence about it."*). A second invocation
**does** progress once this lands, and §8e.13's tests exist to contradict the second. Under ADR-0092 rule 1, moving
either one is a supersession, and the stage-location record takes both.

### 8e.6 Every stage the lane may enter is entered and trailered; an empty one says so; the end is marked (D21-42 7; D21-43 N9)

**The decision:** *"Every `stage_order` stage whose rigor is not `forbidden` is entered and carries its trailer. An
empty stage records `stage_empty { reason }`."*

- **The set is the pack's declared order, filtered.** It is the same filter `Pack::successor` already applies
  (ADR-0153: *"successor skips forbidden and ends the lane"*). For `bug-standard` that is ten of twelve stages:
  `discovery` and `operate` are `forbidden`.
- **An empty stage is entered, trailered, and passed, and it is not skipped.** `triage` carries **0** of the shipped
  activities, and `s7h`'s record says the plan there is empty. Today the lane advances out of such a stage with no
  record of why nothing ran. The kind `stage_empty { stage, reason }` is minted by ADR-0116's route, and its reason is
  a closed set that the record fixes (for example, *no activity applicable at this rigor*). Its tier comes from the
  deleted-database test. The `Story:` mark carries the entry either way, so it is expected to be tier 2, and the
  record says so or says why not. It is anchored to the entry's commit as a row of the same act (§8e.3).
- **The lane's end is a mark (N9: *"A terminal `Story: <project>/<item> closed` mark carrying closeout's content
  checkpoint."*).** Until now the last stage was marked when it was **entered**, and nothing marked its **exit**
  (`s8f` §8f.2 item 2 raised it as a STOP). When `Pack::successor` ends the lane, the kernel mints
  `Story: <project>/<item> closed` through §8e.3's function, with the empty content arm at Checkpoint 8e. `s8f`'s
  landed arm makes it carry closeout's content checkpoint. The fold reads `closed` as the item's latest state. A
  `run` on a closed item reports that it is closed, exits 0, and mints nothing. That exit 0 is **Done**, read from
  git, and it is not ADR-0157's stall. Removal of the run tree (§8e.11) is keyed to this mark.
- **What `s8` gains, and what it does not.** D21-42 7 says *"s8 asserts set and order equality mechanically from the
  lane's own trailer commits"*. That amendment is `s8j`'s. This stage proves the property over a fixture lane
  (`lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order`), with no pack literal: the
  expected list is computed from the fixture pack's own `stage_order` and `stages`, followed by `closed`.

### 8e.7 `Gate:` carries kind, reach and state (D21-42 8b; D21-43 N6, N8), and `terminal_read` reads the awaited gate (D21-42 24c)

**Why the triple is not enough, in the rebuild's own words.** `doctor/rebuild.rs`'s header says: *"The fold's
`gate_opened` arm REQUIRES `kind` and `deadline` and **derives** the token from `(kind, tree_hash, deadline,
reach)` … The `Gate:` trailer carries `(token, tree_hash, deadline)` and no `kind` or `reach`."* So no gate survives a
deleted database, and AC-161's *"the gate records the trailers carry"* has nothing to carry.

- **The value becomes `<token> SP <tree_hash> SP <deadline> SP <kind> SP <reach> SP <state>`**, where `<state>` is one
  of `opened`, `answered`, `expired` or `voided`. That is N6's *"a `Gate:` trailer carrying its state"*.
  - The record fixes each token's grammar. The kind contains no whitespace, and the mint refuses one that does
    (`tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint`).
  - **Whether `answered` carries the option.** `gate_answered_event` records `"verdict": verdict`. If the chosen
    option decides the next action, the deleted-database test puts it in git, and the answered form carries the
    option id. The record decides, by that test, and says so.
  - A three-token value becomes legacy drift under the widened grammar. There are **0** `Gate:` trailers across all
    refs today (`git log --all --format='%(trailers:key=Gate,valueonly=true)'` prints nothing), so no repository value
    moves. Only fixtures and F12's corpus are read differently, and that is AC-444's dated note.
- **An opening and an answer are told apart in git — the first draft's finding 5, settled by N6.** `answer` writes
  *"THE SAME TRIPLE `run` writes"*, and `reconcile` keeps the latest mark per token. So after a deleted database the
  rebuild could not tell an answered gate from an open one, and the next action differs (`AwaitGate` against
  proceed). The state token ends that. `reconcile` folds a gate's latest state per token
  (`trailers::reconcile_folds_a_gates_latest_state_per_token`), and
  `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered` is AC-161's comparison for it.
- **`gate_expired` has its carrier — the first draft's finding 1, settled by N6.** `EventKind::tier()` lists
  `GateExpired` under `Tier::One`, and `trailer_axis()` maps it to `None`. The expiry pass appended it to the store
  alone, which is the shape ADR-0109's bullet forbids. It is now a `Gate: … expired` mark through §8e.3's function,
  and `trailer_axis()` maps it to `Gate`.
- **`gate_answer_void` moves to tier 1, by the gate record.** Design §5's row reads *"`gate_answer_void` | 2 (drift
  observation) | … | `reconcile()`"*. N6 makes the voiding a git-first transition, so the row's tier becomes `1`, its
  producer becomes the kernel at entry (§8e.3), and `reconcile` keeps only the observation. `EventKind::tier()` and
  `trailer_axis()` move with it.
- **ADR-0109 decision 2's list, read as illustrative.** Its parenthetical is *"Tier 1 (stage transitions, gate
  opened/answered, verdicts, slice seals)"*. `gate_expired` is already tier 1 in design §5 and in `EventKind::tier()`,
  so the list was never exhaustive of the design. The gate record states that reading and names it for its
  adversarial pass. If the list is judged enumerative, the record supersedes that parenthetical in part.
- **The deadline is integer unix seconds (N8: *"Supersedes design §5.4's RFC 3339 spelling in part"*).** Both
  producers already write integers. `terminal_read` builds `format!("{token_hex} {tree_hash} {deadline}")` from
  `as_i64`, and `answer.rs` builds `record.deadline().unix_seconds()`. ADR-0163 already found that *"no crate formats
  RFC 3339"*. The row is amended to match, and an RFC 3339 deadline in a `Gate:` value is malformed drift
  (`trailers::an_rfc3339_gate_deadline_is_malformed_drift`). `rk-git`'s `grammar` test that feeds
  `Gate: tok-1 {sha} 2026-09-13T00:00:00Z` is re-pointed in the same commit. `t1_gate.deadline` is `TEXT` and holds the
  decimal.
- **`terminal_read` stops committing and reads the fold.** Its scan (`e.kind == …GateOpened &&
  e.payload.get("token").is_some_and(|t| !t.is_null())`) has *"no answered/void/expired filter"* (ADR-0161's recorded
  fourth finding). It re-commits a `Gate:` for a dead token and prints that the gate is open. After §8e.3 the commit
  belongs to the kernel's `OpenGate` act, so `terminal_read` only reports. It reports the token `fsm::awaiting_gate()`
  returns over the recorded journal, which excludes answered, void and expired tokens. A re-run after an answer
  therefore mints no second opening and prints no open gate.
- **The rebuild now writes gates.** `events_from_trailers`'s `Gate:` arm stops listing every mark unresolvable. It
  emits each gate's opening into `t1_gate` and each later transition as its anchored row, from the state tokens. That
  discharges `s7d` §7d.18's RAISED widening item.
- **Every new gate state needs a `gate_expiry` row only where it opens a gate.** The table is `s8h`'s (D21-42 21a).
  This stage mints no new gate **kind**, so it adds no row. `s8f`'s `drift` and `landing` kinds are `s8f`'s to hand on.

### 8e.8 `Slice-Verdict:` — the slice is declared, and sealed on both review outcomes (D21-41 e, first half)

**The identity is declared, not derived** (ADR-0159's pattern; ADR-0160's route).

- **The key.** `ItemRecord` (today `id`, `kind`, `stage`, `[facets]`) gains an optional `slice`. It is optional in
  the type so the refusal can name the item, as `stage` is.
- **The refusals, at the mint and by name.** ADR-0159 decision 2's siting: *"The refusal is at the mint, not at
  startup"*. Each of ADR-0160's five named values is refused:
  - an absent `slice` refuses the seal by name (*"any value not declared as a slice"*);
  - a `slice` equal to the item id refuses, which is D21-41 e's rule;
  - a `slice` equal to the project name, or to any stage in the pack's `stage_order`, also refuses;
  - the run id cannot be known at the record, and it contains a digest, so a declared slice can never equal it. The
    record says so.
  - A slice with whitespace refuses. The value is *"exactly two whitespace-separated tokens"*.

  The cost of siting the refusal at the mint is stated rather than hidden: a fix lane without a slice walks to its
  review stage before it refuses. The record may move the refusal to entry, and if it does it says why ADR-0159's
  siting does not apply.
- **Cardinality.** A fix-lane item declares exactly one slice (D21-41 e: *"one per fix-lane item"*). The record says
  the same of the story item `s8j` mints.
- **The seal sits on the review stage's exit outcome, because the boundary cannot carry the rejecting verdict.**
  ADR-0161 decision 2: *"The lane loop returns a non-`Exit::Success` status immediately, **before** reading
  `advanced`."* A `changes-requested` outcome crosses no boundary. So the producer is `review_pass`'s outcome, which
  ADR-0162 wired without a stage literal:
  - `blocked.is_empty()` seals `approved`;
  - `Some(Exit::FindingsBlock)` seals `changes-requested` **before** it returns.

  Both seals go through §8e.3's function: the `Slice-Verdict: <project>/<slice> <verdict>` commit, then
  `t1_slice_seal` plus the anchored `slice_sealed` row. The record states that it reads ADR-0160 decision 6's *"at
  the review stage's boundary"* as *"at the review stage's exit outcome"*, and why that moves no sentence. If the
  adversarial pass finds that it does, the record takes it as a supersession in part.
- **`fsm::RunState::terminal()` puts a seal first — a risk measured at the stage's open, not assumed.** Its order is
  *"1. a `slice_sealed` → [`TerminalState::Sealed`]"*. After an `approved` seal at `review`, every later stage's
  recorded terminal state is `Sealed`. The stage's open measures whether any reader (`explain`, the replay
  comparison) misreads a lane that continued past review. If one does, the slice record says how the fold scopes a
  seal.
- **The ARMED lint is un-ignored in the commit whose real fixture run reaches the producer**, on ADR-0162's
  precedent that a real `redkiln run` over a fixture corpus reaches a producer. The runs are the binary:
  - `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` runs over `exit/findings-block/`, which
    exits 4, and reads `Slice-Verdict:` off `rk-run/<item>` with git's own parser;
  - `slice_verdict::approved_is_sealed_on_the_advance_path` runs over a new `slice/approved/` corpus, which exits 0.

  The `#[ignore = "ARMED: …"]` line is deleted in that commit. No erratum is owed for it, because it is source and
  not a record. ADR-0162 decision 2 (*"The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that"*)
  was scoped to that record's work. The slice record cites it and re-states Checkpoint 7h guards (6) and (7), as
  §8e.15 spells out.
- 🛑 **No document claims the review gate is guarded against false approval** (ADR-0101 clause 5; S-SUBTLE-CB is
  UNRUN). A seal records a verdict. It says nothing about the verdict's calibration.

### 8e.9 The rebuild moves into `rk-adapter-sqlite`, and criterion (2) is re-measured (D21-42 8c)

**The decision:** *"Moves into rk-adapter-sqlite (`open_at` / `Capability::Rebuild`, the [s7d] §7d.18 RAISED item).
rk-cli's duplicated INSERT is retired, and criterion (2) is re-measured against ADR-0105 cl.9's real projection."* (The
bracketed `[s7d]` is inserted into the quotation here: the ledger's bare reference names `s7d`'s section.)

- **The duplicated statement goes.** `rebuild.rs` carries *"🛑 THE SECOND COPY OF `SqliteStore::append`'s
  STATEMENT"*. `rk-adapter-sqlite` gains `SqliteStore::open_at(path)`, which targets the staging file, and the rebuild
  writes through it. It writes the `t1_*` rows (boundaries, gate openings and seals, each anchored to its own
  trailer commit) and the anchored `t2_event` rows of tier-1 kinds, through `rk-store`'s `tier_one` statements. The
  git read and the item-record fallback stay in `rk-cli`, because the order *"git → store, never the reverse"* is
  the doctor's. Checkpoint 8e asserts that no `INSERT INTO` code line remains under `crates/rk-cli/src/`.
- **`Capability::Rebuild` is settled, one way or the other.** The port's `rebuild(&Snapshot)` takes an opaque
  `Snapshot` with no field set. Either the record gives it a meaning and the registry row flips, or the git rebuild
  stays an inherent adapter method and the row keeps its dated refusal, with the record saying which. The conformance
  row's `unsupported` list follows the choice.
- **Criterion (2) is re-measured against the real projection.** ADR-0105 clause 9 defines it: *"AC-184's **tier-1
  projection** = the tier-1 rows whose anchor is reachable from the clone's refs"*. `projection::rebuild_differential`
  cannot measure that. Its own header says *"The ROUTE is not measured, only the EQUALITY"*, and `rk-kernel` may not
  depend on an adapter (AC-484). So the anchored differential is a new `rk-cli` test,
  `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`:
  1. run a fixture lane that writes boundary, gate (every state) and seal rows, and ends with its `closed` mark;
  2. take the `t1_*` rows whose anchors `git merge-base --is-ancestor` finds reachable from the repository's refs, in
     canonical form;
  3. delete the store and run `doctor --rebuild`;
  4. compare under the canonical serialiser;
  5. add an adversarial row whose anchor is unreachable, which must fall outside the projection.

  The kernel test keeps its name. `s8` (2)'s filter selects it (`cargo test -p rk-kernel -- rebuild_differential`),
  and its header gains a dated line naming the anchored test as the half that measures the route. Adding the anchored
  test to `s8` (2)'s own block is `s8j`'s instrument amendment, and it is listed there rather than made here.

### 8e.10 The run worktree moves to `.redkiln/worktrees/<item>` (D21-44)

**The decision:** *"Run-owned worktrees move to **`.redkiln/worktrees/<item>`**, git-ignored. This mirrors Claude
Code's own `.claude/worktrees/<name>` convention"*. N1b measured it writable: *"`.redkiln/worktrees/probe-n1b`
(Claude-style, in-repo) | **`edited`** | **none**"*. D21-45 explains why: `checkPathSafetyForAutoEdit` treats a path
as sensitive if *"ANY segment"* is one of its list, and `.git` is on it while `.redkiln` is not.

**What the record supersedes, measured rather than taken from the ledger's summary.** D21-44 says *"A record
superseding in part ADR-0018's and ADR-0105's common-dir worktree placement"*. Read against the records:
- **ADR-0018 names no worktree path.** Its `## Decision` says the kernel *"resolves the repository's common dir
  (`git rev-parse --git-common-dir`) and refuses to write when the current worktree's HEAD is not the run's branch"*.
  Its context places **the store** at the common dir: *"One store per repository lives at `git rev-parse
  --git-common-dir`/redkiln/"*. The store does not move. So the record cites ADR-0018 and supersedes none of it.
- **ADR-0105 names no path either.** Clause 6 is about re-creation, and that is superseded — by W2, not by the move
  (§8e.11).
- **The path literal lives in ADR-0137 decision 3:** *"**`project_root` is the run-owned worktree** — `<common-dir>/redkiln/worktrees/<item>`"*.
  That is `## Decision` text, so under ADR-0092 rule 1 the record supersedes the literal in part, and the rule stands.
  The same path appears in design §5's `reconcile_started` row (*"the worktree and branch rebuild deterministically
  from the item id (`<common-dir>/redkiln/worktrees/<item>/` on `rk-run/<item>`)"*) and in `s7d` §7d.5's tables. Each
  takes a dated note.

**Every derivation moves to one function.** At `a6349b6` the old root is derived at **four** code sites in
`crates/rk-cli/src/verbs/run.rs`: `story_trailer`, `terminal_read`, `reference_worktree` (`format!("{item}.review-ref")`)
and `recreate_run_worktree` (`common_dir.join("redkiln").join("worktrees").join(item)`). There are **six** test code
sites in `crates/rk-cli/tests/it/run.rs` and `answer.rs`. After this stage, `rk_git::worktree` owns the one
derivation. It takes the repository root and a validated item (§8e.11's W1) and returns the path and the branch
together. `story_trailer` and `terminal_read`'s commit are deleted (§8e.3). `run`, `reference_worktree` and every test
support helper call the function. Checkpoint 8e's code-line floor asserts that no code line outside
`crates/rk-git/src/worktree.rs` names a `"worktrees"` directory. The one exception is `doctor`'s
`ctx.common_dir.join("worktrees")`, which is **git's own** administrative directory and does not move.

**The ignore rows are load-bearing, not tidy.** A run tree is now a full checkout inside the checkout's own directory.
Without an ignore, `git status --porcelain` in the user's checkout lists it, and AC-518's instrument
(*"`git status --porcelain` empty, `git diff-index --quiet HEAD` clean, and HEAD unmoved"*) fails on a correct run.
- **This repository:** `.gitignore` already carries `.redkiln/worktrees/`. D21-44 says of that row that it *"is inert
  until s8e moves the root"*.
- **A target repository:** `init` writes the exclude block (§8e.11, W4).
- **Every fixture repository a lane runs in** gains the ignore, in the commit that moves the root. That is the same
  kind of forced edit as `[project] name`, and the one most likely to be missed.
- **Creation refuses by name** when `git check-ignore` does not report `.redkiln/worktrees/<item>` as ignored
  (`worktree::creation_refuses_when_the_worktree_root_is_not_ignored`). The alternative is a run that dirties the
  user's checkout by existing. This refusal is the worktree record's to confirm (§8e.15, finding 15).

**The dated register notes.**
- **AC-518**'s THEN reads *"refused unless its target is a run-owned worktree or the store under the resolved common
  dir"*. The note records that the run-owned worktree now sits inside the checkout's directory and is a separate
  checkout. It records that the instrument's `git status --porcelain` holds through the ignore rows. And it records
  that `lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it` is the instrument's run.
- **AC-529** carries no path. Its Verified-by reads *"the store's worktree row and not `std::env::current_dir`"*. The
  note names that row as `worktree_assigned` (§8e.4). It records the move, and D21-44's *"Ownership stays with the
  store row, so AC-529 stays real wherever the path is"*.

**Every recursive walk over `.redkiln/` excludes `worktrees/`, with a control.** A run tree carries a full copy of
`.redkiln/`, so any walker that recurses would read a second catalog, a second set of cards, and stale runbook text.
- **In the code this stage owns, the walk count is zero** (§8e.0 item 6). The guard is behavioural, not a promise.
  `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` plants
  `.redkiln/worktrees/planted/.redkiln/{catalog,cards,specs,items}` with **invalid** contents. It asserts that
  `doctor`'s findings, the catalog load and the item resolution are byte-equal with and without the planted tree. Its
  synthetic control is mutant `S8E-M32`, which makes `doctor`'s card reader recursive and must turn it RED.
- **Outside this stage's code, each owner applies the exclusion:**
  - `s8-exit-criterion.md`'s recursive greps over `crates/ .redkiln/` are `s8j`'s (`s8j` owns every `s8` amendment).
    At `a6349b6` they are the `require.*\b(criticality|…)\b` search, the `append(_route)? *= *true` search, and the
    `findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)` search. So is the mechanical block's
    `grep -rn … .`, which D21-40 found reading `.git/redkiln/worktrees/`. It would now read `.redkiln/worktrees/`, so
    excluding `.git/` alone no longer suffices.
  - `s8b`'s catalog-quality walks are `s8b`'s.
  - `s8j`'s own greps are `s8j`'s.
- **This stage's own checkpoint** walks nothing under `.redkiln/` recursively.

### 8e.11 The run-worktree lifecycle: Claude Code's safety practices (D21-45 W1, W2, W4; W3's carry)

**The decisions.** W1: *"**Adopt all**"*. W2: *"**Reuse, never force-remove.** … **This replaces
`recreate_run_worktree`'s `worktree remove --force` + `remove_dir_all` at every run start.**"* W4: *"**Both.**
Worktree creation is refused, with a diagnosis, when a repo-local filter driver … or an `includeIf` would yield
pointer files. **`redkiln init` writes an idempotent `# redkiln-runtime` block to `<commondir>/info/exclude`**"*.
W3's carry: *"**Honour the project's existing `.worktreeinclude`**"* and *"**Dependencies come through N25's
per-toolchain `setup` verb.**"*

**W1 — the guards, each a named test in `rk-git`'s `worktree` module.**
- **The item id is validated as a path component and a branch component, once, in the derivation §8e.10 names.**
  The rules are:
  - at most 64 characters;
  - the one segment matches `^[a-zA-Z0-9._-]+$`;
  - it is not `.`, `..` or `.git` (case-folded, because Windows is);
  - it has no leading `.`, and no `.lock` suffix (git's ref rules for `rk-run/<item>`);
  - it has no `.review-ref` suffix. That one is found here: `reference_worktree` names its tree `<item>.review-ref`
    in the same directory, so an item id ending that way would collide with another item's reference tree.

  One table-driven test asserts each row's own refusal reason.
- **Symlink guards.** Creation refuses when `.redkiln`, `.redkiln/worktrees` or `<item>` is a symlink, or on Windows
  a junction. After `git worktree add`, the created tree's canonical path must lie under the canonical
  `.redkiln/worktrees/`, or the tree is refused and the refusal names both paths. Claude Code's reason is the one
  quoted in D21-45: *"A repository-committed symlink at .claude … could redirect worktree creation outside the
  repository"*.
- **The `RK_BASE` marker.** At creation, `RK_BASE` is written into the tree's own gitdir (`git rev-parse --git-dir`
  from inside it). It holds the fork sha and the item id, so ownership is recoverable from git alone (ADR-0109
  decision 4). Redkiln never removes a tree without it.
- **The lock.** `git worktree lock --reason "redkiln run <item> (pid N start T)"` at every run start, and unlocked at
  the run's end. A lock whose pid `rk_containment::is_alive` reports dead is released. A lock held by a live pid
  refuses the run by name. `is_alive` reads the pid alone, and its own doc names *"the documented ambiguity of this
  API"*. So a reused pid reads as alive, and the run refuses. That is fail-closed, and the record says so.
- **The re-entry check.** A registered tree is reused only if it is a separate checkout **of this repository**:
  `git rev-parse --git-common-dir` from inside it equals the repository's, and `git worktree list --porcelain` lists
  its path.
- **`LC_ALL=C` on every parsed git call.** `GitExec`'s child environment sets it, at the one process-exec site
  (`run_with_env`'s `Command::new("git")`), so no parse depends on the operator's locale
  (`gitops::every_git_child_runs_with_lc_all_c`).
- **Windows removal never follows a junction.** `worktree::removal_never_follows_a_junction` puts an ignored junction
  inside a tree that points at a sentinel directory outside it. It removes the tree, and asserts the sentinel's
  contents survive. On a platform without junctions the arm is a symlink, and the test says which it ran.

**W2 — reuse, never force-remove.** This replaces `recreate_run_worktree` whole, and its doc's *"**Re-created, never
reused** (ADR-0105 clause 6…)"* with it.
- **First creation:** `git worktree add --no-track -b rk-run/<item> <path> HEAD`. The fork is the frozen,
  CI-green `HEAD`, not `origin/<default>` (D21-45's *"Where redkiln deliberately differs"*).
- **The branch exists but no tree is registered:** `git worktree add <path> rk-run/<item>`. That happens after a
  landed removal, or after a deleted tree. **Never `-B`**, which *"resets an existing branch and would destroy a
  resumable run"* (`worktree::an_existing_run_branch_is_never_reset`).
- **Re-entry:** the registered tree is reused when it carries `RK_BASE`, passes the re-entry check, and its `HEAD`
  equals the `rk-run/<item>` tip. In-flight changes are read first, path by path, through `GitOp::Status`
  (`--porcelain=v1 -z --untracked-files=all`). They are discarded explicitly (`reset --hard HEAD`, then `clean -fd`),
  and listed on that run's `worktree_assigned.discarded` (§8e.4). That is N10's semantics: what was not landed at a
  committed step is not work. `clean` runs **without `-x`**, so ignored files survive re-entry: W3's carried files and
  the `setup` verb's `node_modules` stay (`worktree::reentry_keeps_ignored_files`).
- **Refused, never removed:** a tree with no `RK_BASE`, a tree registered to another repository, and a tree whose
  `HEAD` has commits the run ref lacks. Each refusal names the path and the reason.
- **Removal** happens only when the tree is clean, landed (`HEAD` equals the run-ref tip), marked and unlocked. It is
  keyed to the lane's `closed` mark (§8e.6), and it uses `git worktree remove` **without `--force`**. Anything else is
  kept, with a reason in Claude Code's form (*"N changed file(s) would be lost"*). Redkiln never removes a tree it did
  not create. The review reference tree follows the same rule. It is detached and read-only by D21-41 (g)'s grants, so
  it is clean unless something wrote to it, and then it is kept and named.
- **Halt 6 is answered, not overridden.** Halt 6: *"If a fixture tree will not delete, that is the handle, and reusing
  the tree corrupts the next arm."* Reuse here is of a run's **own** tree, by the same item, only when its `HEAD` is
  the run ref's tip, and only after an explicit, journalled discard. A tree that will not delete is kept and named,
  and it is never handed to another item. The record states this, and it answers ADR-0105's rejected option
  *"reuse worktrees across runs"* (*"a fresh worktree per run is the only state the kernel can assert"*) in the same
  terms. The asserted state is now `RK_BASE` + `HEAD` = tip + an empty status after the discard.
- **Removal is by run state, never by age.** There is no `cleanupPeriodDays` analogue.

**W3's carry, at creation only.** After `git worktree add` and before the lock is released to the run, creation:
1. calls `s8c`'s `.worktreeinclude` reader (ignored ∩ matched, symlinks skipped, escapes refused) and copies what it
   returns;
2. then runs `s8c`'s per-toolchain `setup` verb (`pnpm install --frozen-lockfile --prefer-offline` for each declared
   Node fixture project).

A `setup` failure is `s8c`'s by-name refusal, and the run surfaces it and stops
(`run_worktree::a_failed_setup_refuses_by_name`). `settings.local.json` is not copied, because children run with
isolated settings (`s8h`). No `node_modules` symlink is made (W3: *"they share mutable state across runs"*). A reused
tree is not re-carried: its ignored files survived the discard.

**W4 — git hygiene.**
- **The filter-driver refusal.** Before `git worktree add`, creation reads the repository-local config:
  `git config --local --get-regexp` over `^filter\.` and over `^includeif\.`. It refuses, with a diagnosis naming each
  key, when a repo-local filter driver or an `includeIf` exists, because either would yield pointer files in the run
  tree (D21-45: *"LFS `--local` would yield pointer files"*). Three tests: the filter, the `includeIf`, and a control
  repository with neither that creates. **This refusal is what `s8a`'s `Text` variant verifies an LFS pointer path
  against**, so a run tree never materialises pointers through a local filter.
- **`init` writes the `# redkiln-runtime` block.** `init` appends to `<commondir>/info/exclude`, creating the file if
  git's template did not:

  ```
  # redkiln-runtime
  /.redkiln/worktrees/
  # end redkiln-runtime
  ```

  The write is idempotent. A second `init` finds the marker and leaves the file byte-identical, which extends
  ADR-0107 decision 2's *"A second `init` leaves this file untouched"* to this file. Lines the operator wrote are
  never touched. `.gitignore` is never written (D21-45: *"never to `.gitignore`"*). This repository keeps its tracked
  `.gitignore` row as well. The block is written in the common directory, beside the store, which AC-531's manifest
  lists *"separately and unlinted"*, so `CreatedNames` is unchanged.

**The `GitOp` variants this takes, named because *"adding a variant is that decision made visible"*** (`git_exec.rs`).
ADR-0105 clause 5 enumerates the wrapper's write set and says *"Anything not listed is a successor"*. So the worktree
record supersedes clause 5 in part, for:
- the writes: `worktree lock`, `worktree unlock`, `reset --hard HEAD` and `clean -fd`, the last two on the owned
  tree only;
- the reads: `worktree list --porcelain`, `config --local --get-regexp` and `check-ignore`.

Every one of them runs through `GitOps` (N7's `git_op` row), and on replay none of them runs.

**The worst-case path (§8e.0 item 2).** The record states the bound: `len(root) + 20 + 64 + 11 + 1 +` the longest
tracked path, measured **255** of **259** at `a6349b6`, with no `core.longpaths`. It also states the rule. This file
recommends that creation refuse by name when the computed worst case exceeds 259 and `core.longpaths` is not `true`,
rather than check out a partial tree (`worktree::creation_refuses_a_path_past_the_platform_limit_by_name`).

### 8e.12 The interim `item` key, and `query` honours `ByItem` (D21-43 N5)

**The decision:** *"An interim `item` key on every item-scoped event kind, moving to ADR-0131's tag later as
ADR-0163's interim key does; `query` honours `ByItem`. Supersedes ADR-0131's first forbids bullet in part, for those
members."* It settles the first draft's finding 7. `stage_transition_event` carried no item, `SqliteStore::query`
ignored its selector, and a second item's `run` in the same store would fold the first item's transitions.
`stages_entered` could then refuse its advance. `s8` runs a fix item **and** a story item in one repository.

- **The members.** The record enumerates every item-scoped kind and adds `item` to each payload, in the shape
  ADR-0163 gave `rate_limit_reading.item`. A kind it judges not item-scoped is listed by name with its reason.
  `fsm::every_item_scoped_kind_carries_the_interim_item_key` iterates `EventKind::ALL` against that list, so a kind
  added later without a decision fails it. `dispatch_completed`'s `item` is one of these members. `s8f` cites this
  record for it and keeps `activity_id_at_version` and the breakdown (D21-42 12b).
- **`query(ByItem(x))` returns only rows whose `item` is `x`**, in `rk-store`'s SQL over the payload, and in
  `VecStore`. The conformance arm asserts it for both rows. Every reader that folds one item reads through it: the
  kernel's fold input, `answer`'s ownership read (§8e.4), and `explain`. `BySha` and `ByRun` stay as they are, and the
  record says whether they are still inert.
- **What an item-less row does under `ByItem`.** A row written before this stage carries no `item`. The record fixes
  whether `ByItem` excludes it or reads it as the one item the store has ever held, and says which. This file
  recommends exclusion plus a named `doctor` finding, because a guess about ownership is the defect N5 exists to end.
- **The cost, stated as ADR-0163 stated it.** The key breaks ADR-0131's *"Waiting accretes nothing"* for these
  members, and it moves to ADR-0131's reserved `item` tag when that wiring lands. §8e.0 item 3 is the STOP on the
  golden-log cost.
- `lane_e2e::two_items_in_one_store_do_not_fold_together` runs two items' lanes in one fixture repository and one
  store. It asserts each item's stage, gates and `stages_entered` are its own.

### 8e.13 The first end-to-end tests of `verbs::run::run` (D21-42 24c; D21-40 decision 2)

**The seam is one field, and the code says so in advance.** `run()` always builds
`rk_adapter_claude::ClaudeHarness::new(binder.into_config(), token.clone())`, so no test can hand it a
`FakeHarness`. `Startup`'s own doc says it is shaped for this: *"The three seams a verb's outer `run` takes, as one
value — so a fourth seam is one field rather than a fourth parameter on six functions."* So `Startup` gains
`harness: &dyn HarnessFactory`. `Startup::real()` builds `ClaudeHarness` exactly as today, and a test passes a factory
over `rk_ports::fakes::FakeHarness::scripted(…)`. Every test below calls **`rk_cli::verbs::run::run(`** by that name,
so `grep -rn "verbs::run::run(" crates/*/tests` stops returning nothing. That grep is a floor, and the tests are the
assertion.

**The fixture lane.** `pack-min` gains a lane variant with a declared `stage_order` that includes:

- a `forbidden` stage;
- an empty stage;
- a stage whose activity opens a gate;
- a review stage over `review-min`.

It also gains an item with a `slice`. Its `config.toml`, and every fixture repository a lane runs in, gains `[project]
name` and the `.redkiln/worktrees/` ignore (§8e.10). **Those edits are forced, and they are the ones most likely to be
missed.** Entry now mints a `Story:` mark (§8e.6), and ADR-0159 decision 2 refuses at the mint. Creation refuses an
un-ignored root. So every existing binary fixture that runs a stage would change its exit from its row to `3`:
AC-438's table, `cost`'s run, and `exit/findings-block/`. Checkpoints 7h (10a) and 7i (3) are the witnesses. For the
same reason, the item that `exit/findings-block/` runs gains a `slice`, or its seal refuses and exit 4 becomes 3.

**The tests, in `crates/rk-cli/tests/it/lane_e2e.rs`:**

- `lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer` — open, then `answer`, then
  re-run. Git's parser counts exactly one `opened` and one `answered` `Gate:` state for the token on `rk-run/<item>`,
  and the re-run's stdout reports no open gate.
- `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane` — the stall ADR-0157 recorded,
  closed. Invocation 1 exits 0 at the gate, `answer` applies, and invocation 2 (a fresh `run`, which opens its own
  store) crosses the next boundary and mints that stage's mark. This is a **two-hop** walk, because a one-hop fixture
  cannot judge a multi-hop guard (ADR-0161).
- `lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order` — the list of stage tokens read
  off the run ref equals the pack's filtered `stage_order`, in order, followed by `closed`.
- `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason`.
- `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` — after hop 2 the store file is deleted. The next
  `run` seeds from the reconciled mark, and its stage-level next action is canonically equal to the one before the
  deletion (AC-161's own comparison).
- `lane_e2e::terminal_read_reports_only_an_awaited_gate` — an answered gate, a void gate and an expired gate, one
  branch each. None prints *"is open"* and none commits.
- `lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child` — the real `redkiln` binary with
  `RK_FIXTURE_MODE=replay:<frames>`: `run` exits 0 at a gate, then `redkiln answer`, then `redkiln run` crosses the
  next boundary. This is the one arm where "process" means a process.
- `lane_e2e::the_lane_end_mints_a_terminal_closed_mark` (N9).
- `lane_e2e::a_closed_item_reruns_as_done_and_mints_nothing` (N9).
- `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered` (N6; AC-161).
- `lane_e2e::two_items_in_one_store_do_not_fold_together` (N5).
- `lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it` (D21-44; AC-518's instrument:
  `git status --porcelain` empty, `git diff-index --quiet HEAD` clean, `HEAD` unmoved).
- `lane_e2e::a_second_invocation_reuses_the_run_worktree` (W2 on the run path). The tree's inode-level identity is
  not observable portably, so the test plants an **ignored** file after invocation 1 and asserts it is still there
  after invocation 2, which a re-creation would delete.

### 8e.14 Named tests, red first, and the mutants

**Every test of a NEW behaviour is written before the code it covers and watched RED for the reason in its row.** A
test first seen green is not evidence (PIA-035). **Five rows are pins over behaviour that must NOT move, or that no
code violates today**:
- `projection::rebuild_differential`;
- `lane_order::adr0161_…`;
- `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record`;
- `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree`;
- `worktree::a_repo_with_no_local_filter_or_include_if_creates`, the control.

They are green at entry by design, and each is shown able to fail by the mutant named for it:
- ADR-0161's `M10` for the lane-order pin;
- `projection.rs`'s own source lint for the tier filter;
- `S8E-M18` for the legacy rule;
- `S8E-M32` for the planted tree;
- `S8E-M23`'s inverse for the control. It makes the filter check refuse always, and the control must go RED.

Each test is selected in Checkpoint 8e by its module-qualified name with `--exact`. A bare function name under
`--exact` runs zero tests and exits 0.

| crate | test | red at entry because |
| --- | --- | --- |
| `rk-git` | `trailers::story_value_with_a_stage_parses_as_a_stage_bearing_mark` | the slug absorbs the stage |
| `rk-git` | `trailers::the_stage_less_legacy_mark_parses_and_carries_no_stage` | no stage field exists |
| `rk-git` | `trailers::a_story_value_with_three_tokens_is_malformed_drift` | it parses as a story |
| `rk-git` | `trailers::the_widened_gate_value_carries_kind_reach_and_state` | six tokens are legacy drift |
| `rk-git` | `trailers::a_gate_value_without_kind_reach_and_state_is_legacy_drift` | the triple is valid today |
| `rk-git` | `trailers::reconcile_folds_the_latest_stage_mark_per_item` | no per-item stage |
| `rk-git` | `trailers::a_repeated_identical_stage_mark_is_duplicate_story_drift` | slug-only duplicate |
| `rk-git` | `trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage` | no stage to keep |
| `rk-git` | `trailers::the_closed_mark_parses_as_the_lanes_end` | `closed` parses as a stage |
| `rk-git` | `trailers::reconcile_folds_a_gates_latest_state_per_token` | no state token |
| `rk-git` | `trailers::an_rfc3339_gate_deadline_is_malformed_drift` | any deadline token parses |
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
| `rk-git` | `worktree::a_repo_local_filter_driver_refuses_creation_with_a_diagnosis` | no config read |
| `rk-git` | `worktree::an_include_if_refuses_creation_with_a_diagnosis` | no config read |
| `rk-git` | `worktree::a_repo_with_no_local_filter_or_include_if_creates` | green — the control; no refusal to observe |
| `rk-git` | `gitops::every_git_child_runs_with_lc_all_c` | no `LC_ALL` |
| `rk-store` | `store::writer_poison_is_a_halt_and_never_recovered` | `into_inner` recovers |
| `rk-store` | `store::tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree` | no statement exists |
| `rk-store` | `store::t1_gate_takes_one_opening_row_per_token_and_is_never_updated` | no statement exists |
| `rk-ports` | `event_store_port::ac364_event_store_port_has_no_update_or_delete` (asserts five) | the method count is four |
| `rk-ports` | `fake_behaviour::vec_store_append_tier1_is_queryable_with_its_anchor` | no method |
| `rk-ports` | `fake_behaviour::vec_store_query_by_item_returns_only_that_items_rows` | the selector is ignored |
| `rk-adapter-sqlite` | `conformance::sqlite_store_conforms_including_the_tier_one_arm` | no arm |
| `rk-adapter-sqlite` | `store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction` | `anchor_sha` is NULL |
| `rk-adapter-sqlite` | `store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows` | no `open_at` |
| `rk-adapter-sqlite` | `store::query_by_item_returns_only_that_items_rows` | `_by` is ignored |
| `rk-kernel` | `tier_one::a_failed_commit_leaves_zero_store_rows` | the row is appended first |
| `rk-kernel` | `tier_one::a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit` | no kernel commit path |
| `rk-kernel` | `tier_one::the_commit_exists_before_any_row_is_readable` | store first |
| `rk-kernel` | `tier_one::the_tier_one_writer_is_called_only_from_the_boundary` | no writer, and its PREMISE asserts a call site exists |
| `rk-kernel` | `tier_one::replay_of_a_boundary_performs_zero_git_writes` | no git on the boundary |
| `rk-kernel` | `tier_one::a_commit_without_a_row_is_healed_at_entry` | no heal |
| `rk-kernel` | `tier_one::a_heal_writes_no_row_for_an_unreachable_anchor` | no heal (this branch's own fixture) |
| `rk-kernel` | `tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint` | no mint in the kernel |
| `rk-kernel` | `tier_one::a_gate_answer_is_committed_before_its_row` | `answer` appends first |
| `rk-kernel` | `tier_one::a_gate_expiry_is_committed_before_its_row` | the expiry is store-only |
| `rk-kernel` | `tier_one::a_gate_void_is_committed_before_its_row` | the void has no trailer |
| `rk-kernel` | `tier_one::a_boundary_anchors_every_row_of_the_act_to_its_commit` | no act |
| `rk-kernel` | `tier_one::an_act_with_neither_trailer_nor_content_is_refused` | no act |
| `rk-kernel` | `tier_one::the_boundary_journals_a_git_op_row_per_invocation` | no `git_op` kind |
| `rk-kernel` | `tier_one::replay_serves_the_boundary_from_its_git_op_rows` | no `git_op` kind |
| `rk-kernel` | `tier_one::a_journal_without_git_op_rows_replays_as_no_git_writes` | no boundary on replay |
| `rk-kernel` | `conformance::conformance_every_trailer_key_has_a_producer` (un-ignored) | `#[ignore]`, and 0 seals |
| `rk-kernel` | `projection::rebuild_differential` | green — the kernel half; its dated header line is new |
| `rk-kernel` | `lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again` | green — the regression witness for the seed change |
| `rk-kernel` | `lane_order::a_pack_stage_named_closed_is_refused_at_load` | `closed` is an ordinary name |
| `rk-kernel` | `fsm::every_item_scoped_kind_carries_the_interim_item_key` | no `item` member |
| `rk-cli` | the thirteen `lane_e2e::…` tests in §8e.13 | `verbs::run::run(` has no FakeHarness seam |
| `rk-cli` | `slice_verdict::approved_is_sealed_on_the_advance_path` | no seal |
| `rk-cli` | `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` | no seal |
| `rk-cli` | `slice_verdict::an_absent_slice_is_refused_by_name` | no key |
| `rk-cli` | `slice_verdict::a_slice_equal_to_the_item_id_is_refused` | no key |
| `rk-cli` | `worktree_assigned::a_tier1_write_from_a_foreign_worktree_is_refused_typed` | no row kind |
| `rk-cli` | `worktree_assigned::the_positive_control_from_the_run_owned_worktree_succeeds` | no row kind |
| `rk-cli` | `worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention` | convention path |
| `rk-cli` | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` | the common-dir root |
| `rk-cli` | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` | green — the pin; `S8E-M32` shows it can fail |
| `rk-cli` | `run_worktree::creation_carries_worktreeinclude_files_and_runs_setup` | no carry |
| `rk-cli` | `run_worktree::a_failed_setup_refuses_by_name` | no `setup` call |
| `rk-cli` | `init::init_writes_the_redkiln_runtime_exclude_block_once` | `init` writes no exclude |
| `rk-cli` | `init::init_leaves_the_operators_exclude_lines_untouched` | `init` writes no exclude |
| `rk-cli` | `answer::a_failed_answer_commit_leaves_zero_rows` | `answer` appends first |
| `rk-cli` | `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` | no `t1_*` rows |
| `rk-cli` | `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` | gates unresolvable |
| `rk-cli` | `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record` | green today — the legacy rule's pin |
| `rk-cli` | `root::adr0118_a_poisoned_writer_mutex_halts_exit_70` | the writer recovers |

**The count.** 97 names: `rk-git` 35, `rk-store` 3, `rk-ports` 3, `rk-adapter-sqlite` 4, `rk-kernel` 21, `rk-cli` 31.
Four already exist: the AC-364 parse, the ARMED lint, `rebuild_differential` and the ADR-0161 pin. One is a rename. So
92 are new, and with the un-ignored lint the suite gains **93** passed (§8e.0 item 9).

The existing `story_axis::adr0155_…` tests and `doctor::ac427_…`'s synthetic repository are **re-pointed** in the
record's commit, and none is deleted. `story_value` gains the stage, and the old single-mark assertions become the
legacy-mark fixture. The six test sites that derive the old root (§8e.10) are re-pointed through the one derivation.
The same goes for `rk-git`'s `worktree::control_a_worktree_can_be_created_and_removed`, whose removal now needs a
clean, marked, unlocked tree.

**The mutants.** Each mutant is applied and restored **from Python, never the shell**: a backtick once swallowed a
restore, and the next backup captured the mutant. Each diff is read. Each must turn the named test RED **for the
named reason**, printed and read, not merely "the suite went red". Names carry an `S8E-` prefix, because `M7` and
`M10` are already taken by `s7h`.

| mutant | change | must redden |
| --- | --- | --- |
| S8E-M1 | `through_step_seven` seeds from the item record, ignoring the reconciled stage | `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane`, `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` |
| S8E-M2 | the mint's idempotence read restored to *"item already committed"* (ADR-0155's first boundary only) | `lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order` |
| S8E-M3 | `append_tier1` before the commit | `tier_one::a_failed_commit_leaves_zero_store_rows`, `tier_one::the_commit_exists_before_any_row_is_readable` |
| S8E-M4 | `append_tier1` drops the `t1_*` insert | `store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction`, `doctor::ac184_…` |
| S8E-M5 | the heal disabled | `tier_one::a_commit_without_a_row_is_healed_at_entry` |
| S8E-M6 | the heal ignores reachability | `tier_one::a_heal_writes_no_row_for_an_unreachable_anchor` |
| S8E-M7 | no seal on the `FindingsBlock` path | `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` |
| S8E-M8 | an absent slice defaults to the item id | `slice_verdict::an_absent_slice_is_refused_by_name` |
| S8E-M9 | ownership resolved by path convention | `worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention` |
| S8E-M10 | `terminal_read`'s old filter (latest tokened `gate_opened`) restored | `lane_e2e::terminal_read_reports_only_an_awaited_gate` |
| S8E-M11 | `unwrap_or_else(std::sync::PoisonError::into_inner)` restored | `store::writer_poison_is_a_halt_and_never_recovered`, `root::adr0118_a_poisoned_writer_mutex_halts_exit_70` |
| S8E-M12 | the boundary commits in `Mode::Replay` | `tier_one::replay_of_a_boundary_performs_zero_git_writes` |
| S8E-M13 | the `Gate:` value drops kind and reach | `trailers::the_widened_gate_value_carries_kind_reach_and_state`, `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` |
| S8E-M14 | `stage_empty` not appended | `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` |
| S8E-M15 | the rebuild writes `t2_event` only | `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` |
| S8E-M16 | `reconcile` keeps the first stage mark, not the latest | `trailers::reconcile_folds_the_latest_stage_mark_per_item` |
| S8E-M17 | a legacy mark resets the stage | `trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage` |
| S8E-M18 | the rebuild drops the item-record fallback for a stage-less mark | `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record` |
| S8E-M19 | the run-tree root derived under the common dir again | `run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees` |
| S8E-M20 | re-entry force-removes and re-creates (`recreate_run_worktree` restored) | `worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip`, `lane_e2e::a_second_invocation_reuses_the_run_worktree` |
| S8E-M21 | a tree without `RK_BASE` is removed rather than refused | `worktree::a_tree_without_rk_base_is_refused_and_never_removed` |
| S8E-M22 | the discard is silent (no paths on the row) | `worktree::reentry_discards_in_flight_changes_and_reports_each_path` |
| S8E-M23 | the filter-driver read skipped | `worktree::a_repo_local_filter_driver_refuses_creation_with_a_diagnosis` (and its inverse, refuse always, reddens the control) |
| S8E-M24 | `init` appends the block on every run | `init::init_writes_the_redkiln_runtime_exclude_block_once` |
| S8E-M25 | `query` ignores `ByItem` again | `store::query_by_item_returns_only_that_items_rows`, `lane_e2e::two_items_in_one_store_do_not_fold_together` |
| S8E-M26 | `answer` appends `gate_answered` before its commit | `answer::a_failed_answer_commit_leaves_zero_rows`, `tier_one::a_gate_answer_is_committed_before_its_row` |
| S8E-M27 | the answer's `Gate:` mark written with state `opened` | `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered` |
| S8E-M28 | no `closed` mark at the lane's end | `lane_e2e::the_lane_end_mints_a_terminal_closed_mark` |
| S8E-M29 | the live `GitOps` journal writes no `git_op` row | `tier_one::the_boundary_journals_a_git_op_row_per_invocation`, `tier_one::replay_serves_the_boundary_from_its_git_op_rows` |
| S8E-M30 | `LC_ALL` not set on the child | `gitops::every_git_child_runs_with_lc_all_c` |
| S8E-M31 | a dead pid's lock is never released | `worktree::a_lock_held_by_a_dead_pid_is_released` |
| S8E-M32 | `doctor`'s card reader made recursive | `run_worktree::no_redkiln_reader_sees_a_planted_run_worktree` |
| S8E-M33 | `clean -fdx` in the discard | `worktree::reentry_keeps_ignored_files` |

A mutant that survives is not waved through. It is either an **unobservable property**, which moves to a source lint
with its reason written down, as `projection.rs`'s tier filter did, or a **missing fixture branch**. Both are recorded
by the mutant's name.

### 8e.15 What moves in other checkpoints, how 7h and 7i stay runnable, and what this file found

**Checkpoint 7h is amended, dated, in the commit that un-ignores the lint, and re-run at Checkpoint 8e.** Guards (6)
and (7) assert the lint is ARMED and names `Slice-Verdict` alone. After §8e.8 both are RED by design. That is the
*"Checkpoint 7h guards (6)/(7) … amended before the seal lands"* the packet names, and the preamble's *"A checkpoint
that changed after it passed is provisional"* applies: 7h's §1 row becomes `passed 490c755, re-run pending`.

- **(6)** becomes: `conformance::conformance_every_trailer_key_has_a_producer --exact`, with its `test … ok` line and
  `1 passed; 0 failed; 0 ignored`.
- **(7)** becomes: the `ARMED: Slice-Verdict has 0 producers` string absent from `conformance.rs`, with `rc` exactly
  1.
- The block's bullet *"`Slice-Verdict:` has a producer. ❌"* is dated ✅ with the two `slice_verdict::` tests named.
- 🛑 **The same amendment repairs guard (10a)'s FAIL message — a live defect.** At `a6349b6` it reads
  `echo "FAIL: AC-438's binary arm did not pass (rc=$rc) — a real `redkiln run` over the review fixture did not exit 4"`.
  Backticks inside double quotes are **command substitution**, so the FAIL path runs `redkiln run` before it echoes.
  The dated amendment removes the two backticks and changes no other byte of the line. Checkpoint 8e asserts the old
  string is gone. It also runs a quote-aware scan over every block it re-runs, which must find no backtick bash would
  execute. A synthetic line with the defect proves the scan can go red.

**Checkpoint 7f is amended, dated, in the same commit, and re-run at Checkpoint 8e.** After `s8a` its literal expects
the lint ignored, with `test result: ok. 2 passed; 0 failed; 1 ignored` (`s8a` §8a.5). `s8a` says in terms that
*"`s8e`'s Slice-Verdict commit moves this literal again"*. Three edits:
1. **The literal:** `3 passed; 0 failed; 0 ignored`, each of the three lints `... ok`, and the `ARMED:` reason absent.
   `s7f`'s reason regex follows.
2. **The run arm moves to a throwaway clone.** 7f's block runs `./target/debug/redkiln run rk-fix-doctor-test-count
   --lane fix` in **this** repository. Under this stage's binary that run would either mint stage-bearing marks on the
   untouched fixture item (D21-42 18), or refuse at W2's re-entry check. The first breaks 7h's (2) and (3). The second
   fails 7f's own positive half. So the arm runs in a `git clone` of `HEAD` under 7f's own scratch directory, and it
   reads that clone's `.git/redkiln/runs`. ADR-0164's `* text=auto eol=lf` row is what makes a fresh checkout test the
   same bytes (`992d257`: *"a new checkout under autocrlf=true tests the same bytes"*). The clone's `rk-run/` refs and
   store are its own, so nothing in this repository moves.
3. **The cleanup line is deleted:** `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count`. It
   force-removes the legacy tree D21-44 keeps for `s8j`, and W2 forbids a forced removal.

The arm still cannot dispatch at this stage, because 0 shipped cards carry `permission_mode`. Checkpoint 8e asserts
that before it re-runs 7f. From `s8h` on, the brief's no-accidental-dispatch rule applies: 7f's arm runs only through
`s8h`'s fixture-child route, and Checkpoint 8e **refuses** to re-run 7f once a card carries `permission_mode`.

**How Checkpoints 7h and 7i stay runnable.** Both read `rk-run/rk-fix-doctor-test-count`, and 7i also reads the
shipped store and `redkiln cost`.

- **No lane runs against the product repository at this stage.** Every run is in a fixture repository or 7f's clone.
  So the ref stays at `73c087a` with its one legacy mark. 7h's (2) (exactly one real `Story:` trailer) and (3) (the
  value `redkiln-rs/rk-fix-doctor-test-count`, slug equal to the item) keep reading what they read today, and the
  legacy mark is exactly what §8e.5 admits read-only. Checkpoint 8e asserts the ref's sha and the mark's value at its
  start and at its end.
- 🛑 **Running `redkiln run rk-fix-doctor-test-count` in this repository under this stage's binary would break 7h on its
  own.** If the legacy registration still holds the branch, W2 refuses. If it is gone (§8e.0 item 1), the run
  creates `.redkiln/worktrees/rk-fix-doctor-test-count` on the existing branch and mints stage-bearing marks. 7h's (2)
  would then count more than one, and (3)'s slug would carry a stage token. D21-42 18 keeps that item *"the untouched
  fixture item"*, and `s8j` re-points 7h and 7i when it mints the fresh item. Until then nobody runs it.
- **The shipped store keeps schema version 1**, because §8e.2's frictions are resolved at the writer (route (B)). So
  `redkiln cost` still opens it, and 7i's `mode=ro` probe reads the same `t2_event`. Checkpoint 8e backs up and
  restores `.git/redkiln/store.sqlite3` (with its WAL, its SHM and the `rebuilt` sidecar) around `s8` (2)'s
  `doctor --rebuild`, and it byte-compares the restore.
- **The binary fixtures 7h (10a) and 7i (3) run get `[project] name`, the ignore and, for `findings-block`, a
  `slice`**, in the commit that makes entry mint (§8e.13). Their exits do not move.

**Findings made while authoring this file, each measured** (1–9 from the first draft, and their disposition now):

1. **`gate_expired` is tier 1 with no trailer axis.** *Settled by N6*: it is a `Gate: … expired` mark (§8e.7).
2. **`t1_gate` implies an `UPDATE` AC-364 forbids.** *Bound by N6* (*"append-only"*). **Still owed: the operator's
   line confirming route (B)'s reading** of *"state is a fold"*, or taking route (A) and its migration (§8e.2,
   friction 3).
3. **Claiming AC-162 needs more than D21-41 e names.** *Made explicit*: ADR-0155 decision 5(b) and its bullet, and
   ADR-0158 decision 3 and its bullet, are in the records table.
4. **Two forbids bullets the stage-location decision does not list.** *Made explicit*: ADR-0157's exit-0 bullet and
   ADR-0161's lane-loop bullet are in the records table.
5. **An answered gate is indistinguishable from an open one in git.** *Settled by N6*'s state token (§8e.7).
6. **The `deadline` spelling.** *Settled by N8*: integer unix seconds (§8e.7).
7. **A multi-item store folds items together.** *Settled by N5* (§8e.12), and the golden-log cost is §8e.0 item 3's
   STOP.
8. **D21-42 18 versus the packet's "added to the shipped items".** Unchanged: no shipped item gains a key here, and
   `s8j`'s fresh item carries its slice.
9. **Two stale comments, re-measured at `a6349b6`** (hygiene):
   - `crates/rk-kernel/src/workflow/fsm.rs`'s `apply` still says *"TOTAL OVER THE CLOSED 29, WITH NO CATCH-ALL ARM"*
     over `pub const ALL: [Self; 38]`. This stage adds three kinds (41), and the commit that mints them touches that
     match, so it corrects the comment to name no number.
   - `story_trailer`'s comment says *"`parse_body` splits every line at column 0 on `": "`"* and that a colon subject
     *"parses as `UnknownKey`"* on every commit. That is **false of the production path**. `read_commits` runs
     `"--format=%H\x1f%ct\x1f%s\x1f%b\x1e"` and calls `parse_body(body)` on the body alone, so a subject never
     reaches the parser. The one caller that feeds a subject to `parse_body` is `crates/rk-cli/tests/it/story_axis.rs`,
     through `grammar::parse_body(&format!("story x entered the lane\n\n{line}"))`. `story_trailer` is deleted
     (§8e.3), and its comment with it. `answer.rs`'s subject `format!("answer({}): {}", args.item, args.option)` is
     therefore harmless to `reconcile`. The boundary function keeps colon-free subjects anyway: git's own
     `%(trailers)` reads a one-paragraph message's last paragraph, and the cost is nothing.

**New findings from this revision:**

10. **D21-45's premise *"`init` already writes the hooks marker (RT-51)"* is false** (§8e.0 item 7). `init` writes no
    hook, and the `# redkiln-hook` marker exists only as `doctor`'s recogniser. W4's ownership stands. The exclude
    block is `init`'s first write into the common directory, so its idempotence is new, not inherited.
11. **D21-44's supersession targets are not where it says** (§8e.10). ADR-0018 and ADR-0105 name no worktree path.
    The literal is ADR-0137 decision 3's. And the decision that actually moves in ADR-0105 is clause 6 plus its
    *"reused worktree"* bullet, which D21-45 W2 moves and neither ledger entry names.
12. **W1, W2 and W4 need `GitOp` variants outside ADR-0105 clause 5's list** (§8e.11). So clause 5 is superseded in
    part too.
13. 🛑 **Checkpoint 7f's cleanup line removes the legacy tree before `s8j` can re-point it.** 7f ends with
    `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count`, and `s8a` runs 7f whole at its
    exit, before this stage. D21-44 says that tree *"stays where it is"* until `s8j`. This file deletes the line at
    §8e.15's 7f amendment, but `s8a`'s run comes first. Owner: `s8a` (or the lead), before `s8a` runs 7f.
14. **Checkpoint 7e has the same backtick defect as 7h (10a).** At `a6349b6`, `s7e-pack-authoring.md` line 52's FAIL
    message is `"… Step 7d's `init` did not run …"`, inside double quotes, so its FAIL path runs `init`. A quote-aware
    scan of every `runbook/` bash block finds exactly these two. 7e's is not this stage's to amend. It is reported.
15. **AC-518's instrument now depends on an ignore row** (§8e.10). Every fixture repository needs one, and creation
    refuses a root that is not ignored. That refusal is a behaviour no ledger line names, so the worktree record takes
    it explicitly or says why not.
16. **The review reference tree shares the item namespace.** `<item>.review-ref` sits beside `<item>`, so the
    validator refuses an item id with that suffix (§8e.11).
17. **The worst-case path is 255 of 259 at `a6349b6`, and it was 259 under the old root** (§8e.0 item 2). The margin
    is four characters on this repository, and a target repository with a longer root or deeper paths has less.
18. **A content commit whose step row never reached the store is not covered by the tier-1 heal** (§8e.3). A kill
    between `s8f`'s landed commit and its step row leaves the run ref ahead of the journal, and W2's re-entry then
    reuses a tree whose `HEAD` already holds the step's effect. `s8f`'s landing record settles how that step is read.

### 8e.16 Acceptance set

**Claimed: AC-161, AC-162, AC-163, AC-184 (re-measured), AC-529, AC-172 (the reconciled state now reaches the
planner), AC-174 (reconcile still writes nothing to git: the void is minted by the kernel), AC-183 (a malformed
`Story:` or `Gate:` value is drift, never coerced), AC-364 (the append-only port with five methods), and AC-518 (every
commit is on `rk-run/<item>`, and the user's checkout is unchanged with the tree inside it).** AC-161 and AC-162 are
claimed **over fixture repositories, as measured**, and only once the supersessions in the records table are accepted.
Until the operator's ledger line flips those records, no document states that either is met. AC-161 now covers
answered, expired and voided gates as well as open ones (N6).

**Excluded:**

- **AC-160**, the `trybuild` tier-annotation fixture: not this stage's, and it does not depend on `t1_*`.
- **AC-164**, whose truncation is `s8` (2)'s kernel half, unchanged here.
- **AC-180**: not triggered, because the schema stays at version 1 under route (B). It becomes this stage's if the
  record takes route (A).
- **AC-444**, corpus-bound and Phase-2: the dated note only.
- **AC-473 and AC-521**: `s8`'s, over real runs.
- **AC-245**'s gate `tree_hash`: `s7b-ii`'s, cited for clause 4's derivation only. Its untracked-file reading is
  `s8f`'s.

### 8e.17 Checkpoint 8e

- **The writer mutex halts** (ADR-0118 clause 3), and no poison recovery remains in `rk-store`. *(24b)*
- **`EventStore` has five methods**, the fake and both conformance rows exercise the tier-1 arm, `query` honours
  `ByItem`, and design §3.3 carries the dated note. *(D21-41 e; N5)*
- **Every tier-1 act is git-first through the kernel, every gate transition included.** AC-162's two fault
  injections pass, the reachability lint passes, replay performs zero git writes and is served from `git_op` rows,
  and a commit without a row heals. *(8a; N6; N7)*
- **No `t1_` SQL outside `rk-store`, and no `INSERT` left in `rk-cli`.** *(8c; ADR-0158's bullet)*
- **The lane crosses invocations and processes.** Every non-forbidden stage is entered and trailered in declared
  order, an empty stage records why, and the lane's end is a `closed` mark. *(D21-41 a; D21-42 7; 24c; N9)*
- **`Slice-Verdict:` is sealed on both outcomes by a real fixture run**, and the ARMED lint is discharged.
  *(D21-41 e)*
- **A foreign-worktree tier-1 write is refused, typed, with ownership read from the store.** *(8d; AC-529)*
- **The run tree lives at `.redkiln/worktrees/<item>`, derived in one place, reused and never force-removed**, guarded
  by W1's checks, refused over a repo-local filter or `includeIf`, and ignored through `init`'s block. The worst-case
  path is within the limit. *(D21-44; W1; W2; W4)*
- **The anchored differential holds**, and the whole suite, fmt and clippy are green. *(8c)*
- **Checkpoint 8d (the entry gate) re-runs whole. So do the three checkpoints this stage's edits make provisional:
  7a (the port surface), 7h and 7f (both amended).** `s8` (2) runs with the store restored byte-for-byte around the
  rebuild, which is D21-42 8c's re-measurement. So does `s8` (3)'s asserted half, the kill-and-resume property the
  seed change touches. Nothing older is re-run: 7b, 7c, 7g and 7i are not this stage's provisional set. The block skips
  all of these nested re-runs when a later checkpoint calls it as an entry gate with `RK_CP_ENTRY_ONLY=1`, and it sets
  the same variable for the checkpoints it calls. A checkpoint that does not honour the variable still runs correctly,
  only slower.
- **`rk-run/rk-fix-doctor-test-count` and its legacy mark are unchanged**, so 7h and 7i stay runnable.

**BEFORE (PIA-035): on the entry tree these lines FAIL:**
- (2)'s selections, because the tests do not exist;
- (3), because the ARMED string is present;
- (4), because of `conn.rs`'s `into_inner`;
- (5), because the port has four methods;
- (6), because nothing calls the writer;
- (7)'s `rk-cli` `INSERT` line, because of `rebuild.rs`;
- (7b)'s root and `recreate_run_worktree` floors, because of `run.rs`'s four derivations and its `"--force"`;
- (7c), because 7h's backtick is present.

A block that passed on `a6349b6` would be asserting nothing.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a SCRIPT (save it and `bash` it). Every guard ends in `exit 1`, and the helper functions below
# exit the script, which only holds outside a subshell.
#
# Conventions, each one paid for on this tree: every cargo reading is `out=$(…); rc=$?` and never a pipe's status
# (under build-lock contention cargo exits 0 having run ZERO tests); every selection is `--test it`, MODULE-QUALIFIED
# and `--exact`, asserting each test's own `... ok` line AND an exact count, because a bare name under `--exact` runs
# zero tests and exits 0 (measured at s7h guard (10a)); source greps read CODE lines only, anchored to grep's
# `path:LINE:` prefix, because a guard can be satisfied by the prose beside it; and every synthetic control proves a
# guard can go RED. Scratch files live under .git/ (Windows Python cannot see Git Bash's /tmp). Python helpers are
# written without a single backslash, because a heredoc on this box has eaten one before.
scratch=.git/cp8e-scratch
rm -rf "$scratch" && mkdir -p "$scratch/store-backup" || { echo "FAIL: the scratch directory $scratch could not be made"; exit 1; }
CODE_ONLY='^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'

# ---- (0) BEFORE READINGS, taken before any cargo run and before any prune.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
rw_before=$(ls -A .redkiln/worktrees 2>/dev/null)
ref_before=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is absent — Checkpoints 7h and 7i read it (see this file's section 8e.15)"; exit 1; }
case "$ref_before" in
  73c087a*) ;;
  *) echo "FAIL: rk-run/rk-fix-doctor-test-count is at $ref_before, not 73c087a — something ran the untouched fixture item (D21-42 18) and 7h guards (2)/(3) will not hold"; exit 1 ;;
esac
legacy=$(git log -1 --format='%(trailers:key=Story,valueonly=true)' "$ref_before"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git could not read the legacy mark (rc=$rc)"; exit 1; }
legacy=$(printf '%s\n' "$legacy" | grep .)
[ "$legacy" = "redkiln-rs/rk-fix-doctor-test-count" ] || { echo "FAIL: the one real Story: mark reads '$legacy', not the stage-less legacy value this stage admits read-only"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: the node fixture has no node_modules — run pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project first, or 10 node tests fail for a reason unrelated to this stage"; exit 1; }
# D21-44: the run-tree root is ignored in THIS repository, or every run tree reads as untracked (AC-518).
git check-ignore -q .redkiln/worktrees/cp8e-probe; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: .redkiln/worktrees/ is not ignored here (check-ignore rc=$rc) — D21-44's .gitignore row is missing"; exit 1; }
# CONTROL: a tracked file is NOT ignored, so the line above can go red.
git check-ignore -q Cargo.toml; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: CONTROL — check-ignore called a tracked file ignored (rc=$rc), so the ignore guard cannot fail"; exit 1; }

# ---- (0b) THE WORST-CASE RUN-TREE PATH (D21-44; section 8e.0 item 2): root + /.redkiln/worktrees/ + a 64-character
# ---- item + .review-ref + / + the longest tracked path, against the 259 characters MAX_PATH leaves.
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
[ "$worst" -le 259 ] || [ "$lp" = "true" ] || { echo "FAIL: the worst-case run-tree path is $worst characters (longest tracked path $longest) and core.longpaths is not true — the worktree record's length rule must refuse this at creation"; exit 1; }
# CONTROL: the same sum over a 200-character path exceeds the limit, so the line above can go red.
ctl=$(( ${#prefix} + 64 + 11 + 1 + 200 ))
[ "$ctl" -gt 259 ] || { echo "FAIL: CONTROL — a 200-character path summed to $ctl, within the limit, so the length guard cannot fail"; exit 1; }
echo "worst-case run-tree path: $worst characters (longest tracked path $longest; the limit is 259 without core.longpaths)"

# ---- (1) BUILD-OR-FAIL, before any binary is read (Halt 1).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8e NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists — every binary-arm test below would be about nothing"; exit 1; }

# ---- (2) THE NAMED TESTS, per crate, each selected exactly. `sel <package> <count> <names…>`.
sel() {
  pkg=$1; want=$2; shift 2
  [ "$#" -eq "$want" ] || { echo "FAIL: the $pkg list names $# tests and its count says $want — the list and the assertion disagree"; exit 1; }
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted, still ignored, or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed; 0 ignored" || { echo "FAIL: the $pkg selection did not run exactly $want tests with none ignored"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
# CONTROL: a filter that matches nothing EXITS 0. This is why `sel` asserts each ok line and an exact count.
out=$(cargo test -p rk-git --test it -- trailers::s8e_control_no_such_test --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the zero-match control did not run (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed' || { echo "FAIL: CONTROL — a zero-match filter did not report 0 passed, so this block cannot tell it from a real run"; exit 1; }

sel rk-git 35 trailers::story_value_with_a_stage_parses_as_a_stage_bearing_mark trailers::the_stage_less_legacy_mark_parses_and_carries_no_stage trailers::a_story_value_with_three_tokens_is_malformed_drift trailers::the_widened_gate_value_carries_kind_reach_and_state trailers::a_gate_value_without_kind_reach_and_state_is_legacy_drift trailers::reconcile_folds_the_latest_stage_mark_per_item trailers::a_repeated_identical_stage_mark_is_duplicate_story_drift trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage trailers::the_closed_mark_parses_as_the_lanes_end trailers::reconcile_folds_a_gates_latest_state_per_token trailers::an_rfc3339_gate_deadline_is_malformed_drift worktree::an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused worktree::a_symlinked_redkiln_or_worktrees_directory_is_refused worktree::a_created_worktree_whose_realpath_leaves_the_root_is_refused worktree::creation_refuses_when_the_worktree_root_is_not_ignored worktree::creation_refuses_a_path_past_the_platform_limit_by_name worktree::creation_writes_rk_base_into_the_worktree_gitdir worktree::creation_locks_the_worktree_with_the_pid_and_start_reason worktree::a_lock_held_by_a_dead_pid_is_released worktree::a_lock_held_by_a_live_pid_refuses_by_name worktree::reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip worktree::reentry_discards_in_flight_changes_and_reports_each_path worktree::reentry_keeps_ignored_files worktree::a_tree_without_rk_base_is_refused_and_never_removed worktree::a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed worktree::a_tree_registered_to_another_repository_is_refused worktree::an_existing_run_branch_is_never_reset worktree::a_dirty_tree_is_kept_with_its_reason worktree::a_locked_tree_is_not_removed worktree::a_clean_landed_marked_unlocked_tree_is_removed worktree::removal_never_follows_a_junction worktree::a_repo_local_filter_driver_refuses_creation_with_a_diagnosis worktree::an_include_if_refuses_creation_with_a_diagnosis worktree::a_repo_with_no_local_filter_or_include_if_creates gitops::every_git_child_runs_with_lc_all_c
sel rk-store 3 store::writer_poison_is_a_halt_and_never_recovered store::tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree store::t1_gate_takes_one_opening_row_per_token_and_is_never_updated
sel rk-ports 3 event_store_port::ac364_event_store_port_has_no_update_or_delete fake_behaviour::vec_store_append_tier1_is_queryable_with_its_anchor fake_behaviour::vec_store_query_by_item_returns_only_that_items_rows
sel rk-adapter-sqlite 4 conformance::sqlite_store_conforms_including_the_tier_one_arm store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows store::query_by_item_returns_only_that_items_rows
sel rk-kernel 21 tier_one::a_failed_commit_leaves_zero_store_rows tier_one::a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit tier_one::the_commit_exists_before_any_row_is_readable tier_one::the_tier_one_writer_is_called_only_from_the_boundary tier_one::replay_of_a_boundary_performs_zero_git_writes tier_one::a_commit_without_a_row_is_healed_at_entry tier_one::a_heal_writes_no_row_for_an_unreachable_anchor tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint tier_one::a_gate_answer_is_committed_before_its_row tier_one::a_gate_expiry_is_committed_before_its_row tier_one::a_gate_void_is_committed_before_its_row tier_one::a_boundary_anchors_every_row_of_the_act_to_its_commit tier_one::an_act_with_neither_trailer_nor_content_is_refused tier_one::the_boundary_journals_a_git_op_row_per_invocation tier_one::replay_serves_the_boundary_from_its_git_op_rows tier_one::a_journal_without_git_op_rows_replays_as_no_git_writes conformance::conformance_every_trailer_key_has_a_producer projection::rebuild_differential lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again lane_order::a_pack_stage_named_closed_is_refused_at_load fsm::every_item_scoped_kind_carries_the_interim_item_key
sel rk-cli 31 lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order lane_e2e::an_empty_stage_records_stage_empty_with_its_reason lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted lane_e2e::terminal_read_reports_only_an_awaited_gate lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child lane_e2e::the_lane_end_mints_a_terminal_closed_mark lane_e2e::a_closed_item_reruns_as_done_and_mints_nothing lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered lane_e2e::two_items_in_one_store_do_not_fold_together lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it lane_e2e::a_second_invocation_reuses_the_run_worktree slice_verdict::approved_is_sealed_on_the_advance_path slice_verdict::changes_requested_is_sealed_on_the_findings_block_path slice_verdict::an_absent_slice_is_refused_by_name slice_verdict::a_slice_equal_to_the_item_id_is_refused worktree_assigned::a_tier1_write_from_a_foreign_worktree_is_refused_typed worktree_assigned::the_positive_control_from_the_run_owned_worktree_succeeds worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention run_worktree::the_run_worktree_is_created_under_dot_redkiln_worktrees run_worktree::no_redkiln_reader_sees_a_planted_run_worktree run_worktree::creation_carries_worktreeinclude_files_and_runs_setup run_worktree::a_failed_setup_refuses_by_name init::init_writes_the_redkiln_runtime_exclude_block_once init::init_leaves_the_operators_exclude_lines_untouched answer::a_failed_answer_commit_leaves_zero_rows doctor::ac184_rebuild_differential_over_anchored_tier_one_rows doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record root::adr0118_a_poisoned_writer_mutex_halts_exit_70

# ---- (3) THE ARMED LINT IS DISCHARGED — its ignore reason is gone, not reworded (ADR-0160's bullet; D21-41 e).
test -f crates/rk-kernel/tests/it/conformance.rs || { echo "FAIL: conformance.rs is absent (Halt 8)"; exit 1; }
hits=$(grep -nF 'ARMED: Slice-Verdict has 0 producers' crates/rk-kernel/tests/it/conformance.rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the ARMED reason is still present or unreadable (grep rc=$rc) — the lint was not discharged in the commit whose fixture run reaches the producer"; printf '%s\n' "$hits"; exit 1; }

# ---- (4) L3: NO POISON RECOVERY IN rk-store (ADR-0118 clauses 3 and 4). A floor; (2)'s two poison tests are the assertion.
raw=$(grep -rnE 'into_inner|PoisonError|catch_unwind' crates/rk-store/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-store poison search could not run (grep rc=$rc)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$hits" ] || { echo "FAIL: a poison recovery on a CODE line in rk-store (D21-42 24b):"; printf '%s\n' "$hits"; exit 1; }

# ---- (5) THE PORT SURFACE: five methods on EventStore, and the design carries the dated note (ADR-0003's count stays five).
test -f crates/rk-ports/src/event_store.rs || { echo "FAIL: the EventStore port file is absent (Halt 8)"; exit 1; }
n=$(grep -cE '^[[:space:]]*fn [a-z0-9_]+\(' crates/rk-ports/src/event_store.rs); rc=$?
[ "$rc" -le 1 ] && [ "$n" -eq 5 ] || { echo "FAIL: EventStore declares $n methods (grep rc=$rc), expected 5 — design section 3.3's four plus the tier-1 writer"; exit 1; }
hits=$(grep -nF 'append_tier1' references/2026-09-08-architecture-design.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the design carries no dated note naming the tier-1 method (rc=$rc) — a port-surface change lands with its design row"; exit 1; }

# ---- (6) AC-162's REACHABILITY FLOOR: every call of the writer is in the kernel's boundary module. The lint test in
# ---- (2) is the structural assertion and the two fault injections are the criterion; this names a stray caller by line.
raw=$(grep -rnF '.append_tier1(' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the reachability search could not run (grep rc=$rc)"; exit 1; }
calls=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -n "$calls" ] || { echo "FAIL: no code line calls append_tier1 — a writer nothing calls is the s7g lesson again"; exit 1; }
stray=$(printf '%s\n' "$calls" | grep -v '^crates/rk-kernel/src/runtime/tier_one\.rs:')
[ -z "$stray" ] || { echo "FAIL: append_tier1 is called outside the kernel's tier-one boundary:"; printf '%s\n' "$stray"; exit 1; }

# ---- (7) NO t1_ SQL OUTSIDE rk-store (ADR-0158's bullet), AND rk-cli's DUPLICATED INSERT IS RETIRED (8c).
raw=$(grep -rnE 't1_(stage_transition|gate|slice_seal)' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the t1_ search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-store/src/' | grep .)
[ -z "$code" ] || { echo "FAIL: a code line outside rk-store names a t1_ table:"; printf '%s\n' "$code"; exit 1; }
raw=$(grep -rn 'INSERT INTO' crates/rk-cli/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-cli INSERT search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: rk-cli still carries an INSERT on a code line — the rebuild's second copy (s7d section 7d.18) is not retired:"; printf '%s\n' "$code"; exit 1; }
# CONTROL: the same filter KEEPS a code line and DROPS a comment line, so the two emptiness checks above can go red.
ctl=$(printf '%s\n' 'crates/rk-cli/src/x.rs:9:        tx.execute("INSERT INTO t1_gate (token) VALUES (?1)", p)?;' 'crates/rk-cli/src/x.rs:10:    // INSERT INTO t1_gate belongs to rk-store' | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-store/src/' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the code-line filter kept $ctl of one code line and one comment line"; exit 1; }
# terminal_read reads the fold's awaited gate — a floor beneath lane_e2e::terminal_read_reports_only_an_awaited_gate.
raw=$(grep -nF 'awaiting_gate(' crates/rk-cli/src/verbs/run.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the awaiting_gate search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in run.rs calls awaiting_gate — terminal_read still scans for any tokened gate_opened (ADR-0161's fourth finding)"; exit 1; }

# ---- (7b) THE RUN TREE (D21-44; D21-45 W1, W2, W4). CODE-LINE floors beneath section 8e.14's behavioural tests.
test -f crates/rk-git/src/worktree.rs && test -f crates/rk-cli/src/verbs/run.rs && test -f crates/rk-git/src/git_exec.rs && test -f crates/rk-cli/src/verbs/init.rs || { echo "FAIL: a source file these floors read is absent (Halt 8)"; exit 1; }
# One derivation of the root: no code line outside rk-git's worktree module names a "worktrees" directory, except
# doctor's read of git's OWN administrative directory, <common-dir>/worktrees, which does not move.
raw=$(grep -rnF '"worktrees"' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the root search could not run (grep rc=$rc)"; exit 1; }
DOCTOR_ADMIN='^crates/rk-cli/src/doctor/rules\.rs:[0-9]+:.*common_dir\.join\("worktrees"\)'
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-git/src/worktree\.rs:' | grep -vE "$DOCTOR_ADMIN" | grep .)
[ -z "$code" ] || { echo "FAIL: a code line outside rk-git's worktree module derives a worktrees directory — the run-tree root is derived in one place (D21-44):"; printf '%s\n' "$code"; exit 1; }
own=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep '^crates/rk-git/src/worktree\.rs:')
[ -n "$own" ] || { echo "FAIL: rk-git's worktree module names no worktrees directory on a code line — the one derivation does not exist"; exit 1; }
# CONTROL: a run.rs code line is KEPT; the doctor admin-dir line and a comment are DROPPED.
ctl=$(printf '%s\n' 'crates/rk-cli/src/verbs/run.rs:9:    let p = common.join("redkiln").join("worktrees").join(item);' 'crates/rk-cli/src/doctor/rules.rs:12:    let Ok(e) = std::fs::read_dir(ctx.common_dir.join("worktrees")) else {' 'crates/rk-cli/src/verbs/run.rs:10:    // the old root joined "worktrees" under the common dir' | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-git/src/worktree\.rs:' | grep -vE "$DOCTOR_ADMIN" | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the root filter kept $ctl of one derivation, one admin-dir read and one comment"; exit 1; }
# W2: recreate_run_worktree is gone, and run.rs forces no removal.
raw=$(grep -rn 'recreate_run_worktree' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the recreate search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: recreate_run_worktree is still on a code line — D21-45 W2 replaces it:"; printf '%s\n' "$code"; exit 1; }
raw=$(grep -nE '"--force"|remove_dir_all' crates/rk-cli/src/verbs/run.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the force-removal search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: run.rs still forces a removal on a code line — W2 is never force-remove:"; printf '%s\n' "$code"; exit 1; }
# W1: LC_ALL=C at the one git exec site. W4: init's exclude marker.
raw=$(grep -nF 'LC_ALL' crates/rk-git/src/git_exec.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the LC_ALL search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in git_exec.rs sets LC_ALL — W1's locale pin is absent"; exit 1; }
raw=$(grep -nF '# redkiln-runtime' crates/rk-cli/src/verbs/init.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the exclude-marker search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -n "$code" ] || { echo "FAIL: init.rs carries no # redkiln-runtime marker on a code line — W4's exclude block is absent"; exit 1; }

# ---- (7c) CHECKPOINT 7h's (10a) FAIL MESSAGE NO LONGER EXECUTES `redkiln run` (section 8e.15). The quote-aware scan
# ---- over every re-run block is in (10); this is the named defect, by its exact bytes.
test -f runbook/phase-1/s7h-lane-completes.md || { echo "FAIL: s7h's file is absent (Halt 8)"; exit 1; }
hits=$(grep -nF 'a real `redkiln run` over the review fixture' runbook/phase-1/s7h-lane-completes.md); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: 7h guard (10a)'s FAIL message still carries backticks inside double quotes (grep rc=$rc):"; printf '%s\n' "$hits"; exit 1; }

# ---- (8) THE WHOLE SUITE, no fail-fast, with a floor. FLOOR is re-derived at stage open (section 8e.0 item 9):
# ---- Checkpoint 8d's recorded passed count + this stage's 93. 944 is the drafted expectation; replace it by a dated edit.
FLOOR=944
tally() { printf '%s\n' "$1" | awk '/^test result:/ { p += $4; f += $6; i += $8; n += 1 } END { print p+0, f+0, i+0, n+0 }'; }
# CONTROL: a run of zero tests tallies below the floor.
read -r cp cf ci cn <<< "$(tally 'test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out')"
[ "$cp" -lt "$FLOOR" ] || { echo "FAIL: CONTROL — a zero-test run tallied $cp passed, not below the floor $FLOOR"; exit 1; }
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
read -r passed failed ignored results <<< "$(tally "$out")"
[ "$failed" -eq 0 ] && [ "$passed" -ge "$FLOOR" ] && [ "$results" -ge 44 ] || { echo "FAIL: the suite read passed=$passed failed=$failed ignored=$ignored over $results results — the floor is $FLOOR passed over at least 44 results"; exit 1; }
echo "whole suite: passed=$passed failed=$failed ignored=$ignored results=$results (floor $FLOOR) — record these in PROGRESS.md"
out=$(cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }

# ---- (9) THE WORKTREE PATH SET, compared BEFORE the re-run blocks, because Checkpoint 7h prunes.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

# ---- (10) RE-RUN FROM THEIR OWN BYTES: the entry gate (8d) and the three checkpoints this stage's edits make
# ---- provisional (7a, 7h, 7f), plus s8 (2) (D21-42 8c's re-measurement) and s8 (3)'s asserted half (the seed
# ---- change). Nothing older. SKIPPED WHOLE when a later checkpoint re-runs this one as its entry gate and sets
# ---- RK_CP_ENTRY_ONLY=1; set for every block this one calls. The extractor takes the ONE bash block of a file that
# ---- contains a needle, and refuses on zero or two. The fence string is built from chr(96), so no triple backtick
# ---- appears inside this block.
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
# The quote-aware backtick scan: a backtick outside quotes or inside double quotes is command substitution. Written
# without a backslash; chr(92) is the escape character.
cat > "$scratch/bt.py" <<'PY'
import sys
BT, SQ, DQ, BS = chr(96), chr(39), chr(34), chr(92)
n = 0
for path in sys.argv[1:]:
    for no, line in enumerate(open(path, encoding="utf-8").read().split(chr(10)), 1):
        if line.strip().startswith("#"):
            continue
        state, j = None, 0
        while j < len(line):
            c = line[j]
            if state is None:
                if c == "#" and (j == 0 or line[j - 1] in " ;" + chr(9)):
                    break
                if c == BS:
                    j += 2
                    continue
                if c in (SQ, DQ):
                    state = c
                elif c == BT:
                    n += 1
                    print("%s:%d %s" % (path, no, line.strip()[:120]))
                    break
            elif state == SQ:
                if c == SQ:
                    state = None
            else:
                if c == BS:
                    j += 2
                    continue
                if c == DQ:
                    state = None
                elif c == BT:
                    n += 1
                    print("%s:%d %s" % (path, no, line.strip()[:120]))
                    break
            j += 1
print("BT-COUNT %d" % n)
sys.exit(1 if n else 0)
PY
test -s "$scratch/bt.py" || { echo "FAIL: the backtick scan was not written"; exit 1; }
# CONTROL: the scan FLAGS 7h's old line shape, and PASSES a single-quoted backtick.
printf '%s\n' 'echo "FAIL: a real `redkiln run` did not exit 4"' > "$scratch/bt-bad.sh" || { echo "FAIL: the scan control could not be written"; exit 1; }
printf '%s\n' "grep -F 'a real \`x\` ran' f" > "$scratch/bt-good.sh" || { echo "FAIL: the scan control could not be written"; exit 1; }
out=$(python "$scratch/bt.py" "$scratch/bt-bad.sh" 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: CONTROL — the backtick scan passed a backtick inside double quotes (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
out=$(python "$scratch/bt.py" "$scratch/bt-good.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — the backtick scan flagged a single-quoted backtick (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
# CONTROL: the extractor REFUSES a needle no block carries, so a renamed checkpoint cannot be skipped silently.
out=$(python "$scratch/extract.py" runbook/phase-1/s7a-ports-and-fakes.md 'S8E-CONTROL-NO-SUCH-NEEDLE' "$scratch/none.sh" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: CONTROL — the extractor accepted a needle no block contains"; exit 1; }
ext() {
  test -f "$1" || { echo "FAIL: $1 does not exist — the checkpoint it holds cannot be re-run (Halt 8)"; exit 1; }
  out=$(python "$scratch/extract.py" "$1" "$2" "$3" 2>&1); rc=$?
  [ "$rc" -eq 0 ] && [ -s "$3" ] || { echo "FAIL: the block in $1 could not be extracted whole (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
  out=$(python "$scratch/bt.py" "$3" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the block in $1 carries a backtick bash would execute:"; printf '%s\n' "$out"; exit 1; }
}

if [ "${RK_CP_ENTRY_ONLY:-0}" = "1" ]; then
  echo "CP8E: nested re-runs skipped — RK_CP_ENTRY_ONLY=1 was set by the checkpoint re-running this one as its entry gate"
else
  # (10a) CHECKPOINT 8d, the entry gate, whole. If it needs an environment variable, set it as s8d says; this block does not guess.
  ext runbook/phase-1/s8d-step-grain-and-resolver.md 'echo "CP8D-END-OF-BLOCK reached"' "$scratch/cp8d.sh"
  out=$(RK_CP_ENTRY_ONLY=1 bash "$scratch/cp8d.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8d, re-run whole, exited $rc — this stage's entry gate no longer holds"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8D-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8d exited 0 without reaching its end marker"; exit 1; }

  # (10b) CHECKPOINT 7a whole — the gate on the port surface this stage changed. Its block has no end marker, so one is appended.
  ext runbook/phase-1/s7a-ports-and-fakes.md 'pub store: Arc<Mutex<dyn EventStore>>' "$scratch/cp7a.sh"
  printf '\necho "CP7A-EXTRACTED-END reached"\n' >> "$scratch/cp7a.sh" || { echo "FAIL: the 7a end marker could not be appended"; exit 1; }
  out=$(RK_CP_ENTRY_ONLY=1 bash "$scratch/cp7a.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7A-EXTRACTED-END reached' || { echo "FAIL: Checkpoint 7a stopped before its last line"; exit 1; }

  # (10c) CHECKPOINT 7h whole, AS AMENDED BY THIS STAGE (section 8e.15). Its guards (6)/(7) are the discharge's
  # witnesses, and ext() has already refused a block that still carries (10a)'s backtick.
  ext runbook/phase-1/s7h-lane-completes.md 'echo "CP7H-END-OF-BLOCK reached"' "$scratch/cp7h.sh"
  out=$(RK_CP_ENTRY_ONLY=1 FIX_ITEM=rk-fix-doctor-test-count bash "$scratch/cp7h.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7h, re-run whole, exited $rc — if guard (6) or (7) names ARMED, the dated amendment did not land"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7H-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7h exited 0 without reaching its end marker"; exit 1; }

  # (10d) CHECKPOINT 7f whole, AS AMENDED (section 8e.15): its run arm is in a throwaway clone, and it can reach no
  # dispatch. Refused outright once a shipped card carries permission_mode (s8h has landed; no-accidental-dispatch).
  cards=$(ls .redkiln/cards/*.json 2>/dev/null)
  [ -n "$cards" ] || { echo "FAIL: no shipped card under .redkiln/cards — the dispatch precondition would be about nothing (Halt 8)"; exit 1; }
  raw=$(grep -l 'permission_mode' .redkiln/cards/*.json); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: a shipped card carries permission_mode (grep rc=$rc) — 7f's run arm could reach a real dispatch; re-run 7f only through s8h's fixture-child route, not from this block"; printf '%s\n' "$raw"; exit 1; }
  ext runbook/phase-1/s7f-conformance-and-records.md 'echo "CP7F-END-OF-BLOCK reached"' "$scratch/cp7f.sh"
  grep -qF 'git clone' "$scratch/cp7f.sh" || { echo "FAIL: Checkpoint 7f's run arm is not re-pointed to a throwaway clone — run here, it would advance the untouched fixture item (D21-42 18)"; exit 1; }
  hits=$(grep -nF 'worktree remove --force .git/redkiln/worktrees/' "$scratch/cp7f.sh"); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: Checkpoint 7f still force-removes the legacy run tree (grep rc=$rc) — D21-44 keeps it for s8j:"; printf '%s\n' "$hits"; exit 1; }
  out=$(RK_CP_ENTRY_ONLY=1 bash "$scratch/cp7f.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7f, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7F-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7f exited 0 without reaching its end marker"; exit 1; }

  # (10e) s8 CRITERION (2) whole — it runs doctor --rebuild, which REPLACES the shipped store. Back up the store, its
  # WAL, its SHM and the `rebuilt` sidecar; restore them whatever the block did; byte-compare; only then read its rc.
  sd=$(git rev-parse --git-common-dir); rc=$?
  [ "$rc" -eq 0 ] && [ -f "$sd/redkiln/store.sqlite3" ] || { echo "FAIL: no shipped store under $sd/redkiln (rc=$rc)"; exit 1; }
  sd="$sd/redkiln"
  for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
    if [ -f "$sd/$f" ]; then cp -p "$sd/$f" "$scratch/store-backup/$f" || { echo "FAIL: could not back up $f — criterion (2) NOT RUN"; exit 1; }; fi
  done
  ext runbook/phase-1/s8-exit-criterion.md 'echo "(2) asserted: AC-184 differential' "$scratch/cp8-2.sh"
  printf '\necho "CP8-2-EXTRACTED-END reached"\n' >> "$scratch/cp8-2.sh" || { echo "FAIL: the (2) end marker could not be appended"; exit 1; }
  out2=$(bash "$scratch/cp8-2.sh" 2>&1); rc2=$?
  for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
    rm -f "$sd/$f"
    if [ -f "$scratch/store-backup/$f" ]; then cp -p "$scratch/store-backup/$f" "$sd/$f" || { echo "FAIL: RESTORE of $f failed — restore it by hand from $scratch/store-backup before anything else runs"; exit 1; }; fi
  done
  for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
    if [ -f "$scratch/store-backup/$f" ]; then cmp -s "$scratch/store-backup/$f" "$sd/$f" || { echo "FAIL: the restored $f differs from its backup — the shipped store is not the one this block found"; exit 1; }; fi
  done
  [ "$rc2" -eq 0 ] || { echo "FAIL: s8 criterion (2), re-run whole, exited $rc2"; printf '%s\n' "$out2" | tail -30; exit 1; }
  printf '%s\n' "$out2" | grep -qxF 'CP8-2-EXTRACTED-END reached' || { echo "FAIL: s8 criterion (2) stopped before its last line"; exit 1; }

  # (10f) s8 CRITERION (3), ASSERTED HALF — the block is run whole with ITEM UNSET. Its asserted half must pass, and it
  # must then stop at exactly one FAIL, the operational arm's `set ITEM` (D21-40's re-measurement reads the same).
  ext runbook/phase-1/s8-exit-criterion.md 'echo "(3) asserted: AC-013' "$scratch/cp8-3.sh"
  out=$(env -u ITEM bash "$scratch/cp8-3.sh" 2>&1); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: s8 (3) with ITEM unset exited $rc, expected 1 at the operational arm"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qxF '(3) asserted: AC-013 N-step kill-and-resume, digest equality, zero re-dispatch' || { echo "FAIL: s8 (3)'s asserted half did not report — kill_and_resume_n_step is red or unselected"; printf '%s\n' "$out" | tail -20; exit 1; }
  fails=$(printf '%s\n' "$out" | grep '^FAIL')
  [ "$fails" = "FAIL: set ITEM to the item the run is on" ] || { echo "FAIL: s8 (3) stopped somewhere other than the operational arm:"; printf '%s\n' "$fails"; exit 1; }
fi

# ---- (11) AFTER: the fixture item's ref and mark are untouched, and no lane ran in this repository.
ref_after=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ "$ref_after" = "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count moved during this block ($ref_before -> $ref_after)"; exit 1; }
rw_after=$(ls -A .redkiln/worktrees 2>/dev/null)
[ "$rw_before" = "$rw_after" ] || { echo "FAIL: .redkiln/worktrees changed during this block ('$rw_before' -> '$rw_after') — a lane ran against the product repository"; exit 1; }
rm -rf "$scratch" || { echo "FAIL: the scratch directory $scratch could not be removed"; exit 1; }

echo "CP8E-END-OF-BLOCK reached"
```

---
