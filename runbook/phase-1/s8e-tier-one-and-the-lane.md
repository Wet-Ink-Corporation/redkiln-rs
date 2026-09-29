# Step 8e — Tier-1 writes git-first, the lane across processes, and `Slice-Verdict:` (Phase 1, part 8e)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8e` | Phase 1 | Checkpoint 8d-ii | Checkpoint 8e | `s8d-ii` | `s8f` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s7h §7h.2`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*D21-48 (2026-09-24)* (the operator's answers to the questions the s8 files left open; `spikes/DECISIONS-2026-09-21.md`'s
`## D21-48` is the authority; each edit dated on its own line):
- *D21-48 (2026-09-24):* **X10 settles ADR-0158's `rk-cli` bullet.** D21-48 X10: *""From `rk-cli`" names where the
  write runs. The adapter's `Capability::Rebuild` owns the SQL, so there is no supersession. The record states the
  reading."* The records table and §8e.9 now state that reading as decided, and the fallback (a supersession in part
  for `doctor --rebuild` alone, if the adversarial pass rejected the reading) is removed. The *"with a fallback
  supersession"* of MAJOR 1 below is withdrawn by this line. Checkpoint 8e (6) and (7) are unchanged: they already
  assert both clauses on code lines. No test, mutant or count moves.
- *D21-48 (2026-09-24):* **X11 accepts the rebuilt `opened` row's null `run_id` as a stated residual.** D21-48 X11:
  *"Accepted as a stated residual. RK-D-UNREAD reads null as an unknown run, never as a match. The `Gate:` grammar
  does not widen."* §8e.7 states it as decided. The rebuild half is pinned here: `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value`
  gains one assertion (every `opened` row the rebuild re-derives has a null `run_id`), and mutant S8E-M48 invents a
  value. The `RK-D-UNREAD` half is pinned where `run_of` widens, by `s8f`'s
  `doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match` (`s8f` §8f.5). At Checkpoint 8e the rule's
  `run_of` is still `as_i64`, which reads `null` as no run. No count moves: this stage stays at **74**.
- *D21-48 (2026-09-24), floor propagation:* the predecessors now state `s8c` **78**, `s8d` **62** and `s8d-ii` **50**, so Checkpoint 8d-ii's expected
  count is 740 + 9 + 25 + 78 + 62 + 50 = **964**, and this stage's floor is ~~964 + 74 = **1038**~~ (§8e.0 item 7,
  Checkpoint 8e (8)). The 947 + 74 = 1021 of the final consistency pass is superseded.
- *(dated 2026-09-28, s8e build)* **The floor is re-derived at stage open from Checkpoint 8d-ii's RECORDED count** (`runbook/PROGRESS.md` row 8d-ii: *"whole suite **1034 passed / 0 failed / 10 ignored over 44**"*, Checkpoint 8d-ii passed whole at `497266b`, re-run green after `be32cda`): 1034 + 74 = **1108**. The chain's expected 964 is superseded by the recorded 1034; the recorded count governs. The owed-in `lane_e2e` test(s) for MF3j and for `run()`'s review-routes binding (§8e.11's two receiver notes) are **outside** the 74, so when they are named the floor, Checkpoint 8e (2)'s `sel rk-cli 30` and §8e.12's count move again by dated edit.
- *(dated 2026-09-28, s8e build, package S8E-P2)* **The two owed-in tests are named, and the floor moves again** (§8e.11's two receiver notes): `lane_e2e::run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer` (MF3j) and `lane_e2e::run_binds_the_review_routes_for_a_shallow_clone_step` (`run()`'s review-routes binding). Both are new, so §8e.12's count moves to 82 names (~~80~~) and **75** new (~~73~~), the suite gains **76** (~~74~~), and the floor is 1034 + 76 = **1110** (~~1108~~). Checkpoint 8e (2) reads `sel rk-cli 32` (~~`sel rk-cli 30`~~) and (8) reads `FLOOR=1110` (~~`FLOOR=1108`~~).
- *(dated 2026-09-28, s8e build, package S8E-P6)* **One more kernel test, and the floor moves by one.** `tier_one::a_closed_mark_heals_as_the_lanes_end_transition` is new: the heal's `closed` arm (§8e.6) is a branch no `lane_e2e` fixture reaches, because a closed item's `run` returns before the heal. So §8e.12's count moves to **83** names (~~82~~) and **76** new (~~75~~), the suite gains **77** (~~76~~), and the floor is 1034 + 77 = **1111** (~~1110~~). Checkpoint 8e (2) reads `sel rk-kernel 25` (~~`sel rk-kernel 24`~~) and (8) reads `FLOOR=1111` (~~`FLOOR=1110`~~).
- *(dated 2026-09-28, s8e build, package S8E-P7)* **D21-93's test, and the floor moves by one.** The operator's ruling on §8e.0 item 8's STOP (`spikes/DECISIONS-2026-09-21.md` D21-93: *"built red-first with a mutant (the scope dropped) in §8e.8's package"*) adds `fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition`, which §8e.12's list did not carry. So §8e.12's count moves to **84** names (~~83~~), `rk-kernel` **26** (~~25~~), and **77** new (~~76~~); the suite gains **78** (~~77~~), and the floor is 1034 + 78 = **1112** (~~1111~~). Checkpoint 8e (2) reads `sel rk-kernel 26` (~~`sel rk-kernel 25`~~) and (8) reads `FLOOR=1112` (~~`FLOOR=1111`~~). Measured at the package's diagnostic whole-suite run: **1112 passed / 0 failed / 9 ignored over 44 results** (the ARMED lint's `#[ignore]` deleted: 10 → 9 ignored).
- *(dated 2026-09-28, s8e build, package S8E-RULINGS)* **D21-92's two tests, one replaced, and the floor moves by one.** The operator's ruling (`spikes/DECISIONS-2026-09-21.md` D21-92: *"A `null` `until` or gate `when` is journaled on the same rule as a `true` or `false`: **when its route continues the walk**. A `null` routed to a refusal leaves no row"*) is built and stated in **ADR-0187** (`proposed`; supersedes ADR-0174 decision 7 in part, its null sentence only). It adds `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` and `on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes` to §8e.12's list, and REMOVES `s8d`'s `on_false_exec::a_null_until_or_gate_when_is_not_journaled`, which pinned the rule the ruling reverses (Checkpoint 8d's selection is moved by dated edit in `s8d`'s file). So §8e.12's count moves to **86** names (~~84~~), `rk-kernel` **28** (~~26~~), and **79** new (~~77~~); the suite gains **79** (~~78~~): 79 new plus the un-ignored lint, less the one replaced test. The floor is 1034 + 79 = **1113** (~~1112~~). Checkpoint 8e (2) reads `sel rk-kernel 28` (~~`sel rk-kernel 26`~~) and (8) reads `FLOOR=1113` (~~`FLOOR=1112`~~).
- *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY)* **D21-94's entry refusal, built as D21-95 rules it, and the floor moves by three.** The operator's rulings (`spikes/DECISIONS-2026-09-21.md` D21-94: *"**The refusal moves to the lane's entry, before any dispatch** … The mint keeps its refusal as the backstop"*; D21-95 item 1: *"The entry check decides from declared `provides` (option A)"*) are built: one slice validator, `tier_one::validate_slice`, called by `seal_act` and by `run`'s new entry check, which walks the stage order from the current stage through `Pack::successor` and refuses by name when a reached stage has a catalog activity whose `provides` first is `Findings` (§8e.8's dated note; ADR-0186 amended, `proposed`). Three `rk-cli` tests are new: `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch`, its control `lane_e2e::a_lane_that_reaches_no_findings_stage_is_not_refused_for_an_undeclared_slice`, and the mint's pin `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop`. So §8e.12's count moves to **89** names (~~86~~), `rk-cli` **35** (~~32~~), and **82** new (~~79~~); the suite gains **82** (~~79~~), and the floor is 1034 + 82 = **1116** (~~1113~~). Checkpoint 8e (2) reads `sel rk-cli 35` (~~`sel rk-cli 32`~~) and (8) reads `FLOOR=1116` (~~`FLOOR=1113`~~). D21-95 item 2's gap (0 shipped activities provide `Findings` first, so the check and the seal are inert on the shipped packs) is owed to `s8g` by a dated receiver note in its §8g.1.
- *(dated 2026-09-28, s8e build, package D2196-8e)* **D21-96 built, and the floor moves by one.** The operator's ruling on R1FIX-8e's defect (c) (`spikes/DECISIONS-2026-09-21.md` D21-96: *"The live `answered` row of `t1_gate_transition` carries the gate's **bound** tree, taken from the gate record, so the tier-1 row equals what its mark carries and a rebuild reproduces it byte for byte"*) is built in `rk_store::tier_one::append` (§8e.9's D2196-8e note; ADR-0181 decision 3's dated note, `proposed`), and `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`'s lane now walks through a void. One `rk-store` test is new, `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening`, for the refusal branch no lane reaches. So §8e.12's count moves to **92** names (~~91~~), `rk-store` **7** (~~6~~), and **85** new (~~84~~); the suite gains **85** (~~84~~), and the floor is 1034 + 85 = **1119** (~~1118~~). Checkpoint 8e (2) reads `sel rk-store 7` (~~`sel rk-store 6`~~) and (8) reads `FLOOR=1119` (~~`FLOOR=1118`~~).

*Final consistency pass 2026-09-24* (the pass brief after D21-47's fix round; each edit dated on its own line):
- *Final consistency pass 2026-09-24:* the floor is re-derived once on the final chain: 740 + 9 (8a) + 25 (8b) + 69
  (8c) + 58 (8d) + 46 (8d-ii) = 947 at Checkpoint 8d-ii, plus this stage's 74 = **1021** (§8e.0 item 7, Checkpoint 8e
  (8)). The 984 and 991 below are superseded.
- *Final consistency pass 2026-09-24:* Checkpoint 8e sources the shared no-dispatch wrapper,
  `runbook/tools/nodispatch.sh`, in place of its inline shell-script fake. `nd` is `nodispatch` plus this stage's two
  extra unsets, and `nd_check` follows the cargo runs, each replay and the block's end ((0w), (10z); §8e.15). Finding
  16 is settled by it (§8e.13).
- *Final consistency pass 2026-09-24:* Checkpoint 8a carries its record ids and its base commit as literals (the one
  record-id convention), so (10h) no longer demands `CS_ADR`, `PACK_ADR`, `CODEX_ADR` or `S8A_BASE` from the
  environment. Finding 17 is settled (§8e.13).
- *Final consistency pass 2026-09-24:* the gate's `opened` transition row carries the opening run's `stage` and
  `run_id` (§8e.2, §8e.7), the carrier `s8f` §8f.2 item 14 reads for `answer`'s `artifact.write`. It adds one
  assertion to an existing `lane_e2e` test and mutant S8E-M47, so no count moves.
- *Final consistency pass 2026-09-24:* §8e.14's AC-518 bullet names `s8f`'s landing writes, which go through
  `runtime/tier_one.rs` and which `s8f` brings under the commit-scope lint in its own commit.

*Adversarial-review fixes 2026-09-24 (D21-47).* The review of this file at `4a8c832` found 0 BLOCKER, 6 MAJOR and 7
MINOR findings. The cross-file critic assigned two more here (#2 and #16), and D21-47 set the fix round's rules. Each
finding, and its fix:
- **MAJOR 1, the AC-162 reachability guard.** It now reads every call form of `append_tier1` and drops only
  definitions. The conformance arm is allow-listed with its reason. The rebuild's exemption is stated, and `rk-store`'s
  `tier_one` entry points are named and fenced to `rk-store` and `rk-adapter-sqlite` by a new floor. ADR-0158's
  `rk-cli` bullet is read in terms, with a fallback supersession (§8e.3, §8e.9, the records table, Checkpoint 8e (6)).
- **MAJOR 2, the removal trigger had no test.**
  `lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one`, with S8E-M37 and S8E-M38 (§8e.6;
  finding 14 settled).
- **MAJOR 3, instruments differing from the register.** AC-180 gets its checked-in version-1 fixture and a
  bytes-unchanged test. AC-163 gets a one-commit-per-row replay over the fixture lane. AC-518 gets a commit-scope
  write-path lint. AC-161 re-runs its kernel half. AC-161, AC-163 and AC-518 carry dated register re-point notes.
  AC-172 moves to "contributed to, not claimed" (§8e.2, §8e.11, §8e.14).
- **MAJOR 4, AC-160 had no owner.** It is claimed here on its `s6` trybuild fixture, re-run by name, which matches
  `s8j` §8j.9 (§8e.14).
- **MAJOR 5, the `Capability::Rebuild` either/or.** It is implemented. `Snapshot` gets its contents, both SQLite
  registry rows flip, the conformance arm runs the round trip, and a new registry test comes with S8E-M39 (§8e.9).
- **MAJOR 6, 7f's `git clone` guard was satisfied by a comment.** It reads code lines only, with a control each way,
  and a ref and worktree snapshot now brackets the 7f run (Checkpoint 8e (10d)).
- **MINOR 7.** (11) compares the `rk-run/` ref set and the worktree path set across the whole block.
- **MINOR 8, with D21-47's replay rule.** Every cargo run and every replay goes through `nd`: a fake `claude` that
  prints the pin, the trap, and `REDKILN_LAUNCH_RECORD`, `ANTHROPIC_API_KEY` and `CLAUDE_CODE_OAUTH_TOKEN` unset. The
  trap is checked after the whole block ((0w), (10z)). Finding 16 states the wrapper's limit.
- **MINOR 9.** The un-ignored lint's producer scope drops `rk-cli/src/doctor/`, with S8E-M45 (§8e.8).
- **MINOR 10.** (7b) reads SQL code lines only, with a control. (10e)'s label now names what `s8` (2) runs, and where
  D21-42 8c's re-measurement is.
- **MINOR 11.** The `terminal()` measurement is §8e.0 item 8, and the `verbs::run::run(` floor is in (7).
  `worktree_assigned`'s `reused`/`discarded` members and `answer`'s fail-closed branch get a test each, with S8E-M41
  and S8E-M42.
- **MINOR 12.** ADR-0131 decisions 4 and 8, and its pre-Checkpoint-8 bullet, are argued in §8e.10 and the records
  table, with a fallback supersession.
- **MINOR 13.** The ADR-0153 quotation is now decision 4's own words, and `runtime/` is stated as already a directory.
- **Cross #2.** 8a guard (7) is amended in the Slice-Verdict commit, and 8a joins the re-run set as provisional
  (§8e.13, (10h)). **Cross #16.** The drift-gate advance refusal is cited as `s8f`'s (§8e.7, "What it does not own").
- **D21-47's other rules.**
  - The fixture harness is `rk-fixture-child-claude` (§8e.11).
  - ADR-0105 clause 5 is cited as not superseded, because every git write here is a `commit` (the records table).
  - The checkpoint locates no record by title or by count.
  - This file carries no optional-`null` citation to revert.
- **The count** moves to 80 names and 73 new, and the floor to 917 + 74 = **991** (§8e.0 item 7, §8e.12) *(superseded by the final consistency pass: 947 + 74 = **1021**)*. The
  consistency pass's 984, below, is superseded.

*Revised 2026-09-24 for D21-46, at `c5d19d7`: the stage is split, two decisions are applied, and one convention is
adopted.* What changed:
- **R1, the split.** D21-44's relocation and D21-45 W1, W2 and W4, with W3's carry, moved to a new stage, `s8d-ii`
  ("Run worktrees"). Its Checkpoint 8d-ii is this stage's entry gate. The former sections numbered 8e.10 and 8e.11
  are gone, and the sections after them are renumbered §8e.10 to §8e.15. The records-table row, 30 named tests, the
  mutants, seven findings and the checkpoint lines that asserted that work moved with it. Where this stage's tier-1
  work depends on the new root, the one derivation, `RK_BASE`, the lock or the discard list, it cites `s8d-ii`. The
  surviving `S8E-` mutant names are kept, so their numbering has gaps.
- **R2.** `t1_gate`'s friction takes route (A), a forward migration to an append-only transitions table (§8e.2). The
  route (A)/(B) discussion is gone, and AC-180 is claimed here (§8e.14).
- **R3.** The interim `item` key is carried outside the canonical digest (§8e.10). The golden-log STOP that §8e.0
  carried is gone, and a test pins the golden logs' digests.
- **One re-run convention and one replay guard.** `RK_CP_NESTED` replaces this file's own draft variable for an
  entry-gate run. Every replay runs with `CLAUDE_CONFIG_DIR` at an empty trap directory and `REDKILN_LAUNCH_RECORD`
  unset (§8e.15).

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The floor chain is recomputed once**: Checkpoint 8d-ii's 917 + this stage's 67 = **984**, not the drafted 931
  (§8e.0 item 7). *(Superseded by the adversarial-review fixes above: 917 + 74 = 991.)*
- **The legacy registration survives `s8a`**, whose guard (11) restores it (§8e.0 item 1).
- **Finding 10 places the `init` backtick line where `s8a` found it**, in `s7e` §7e.1's work block, not Checkpoint
  7e's. Finding 15's citations are reconciled in the sibling files.
- **The trap** asserts that it starts empty, in `s8g`'s form.

*Revised 2026-09-24 for D21-43/44/45, at `a6349b6`.* What changed, and where (section numbers as they now stand):
- **D21-44 and D21-45:** now `s8d-ii`'s, by the note above.
- **N5:** the interim `item` key, with `query` honouring `ByItem`, is §8e.10. **N6:** every gate transition is
  git-first, and the `Gate:` value carries its state (§8e.7, §8e.3). **N7:** the planner reads the fold, and the
  `git_op` row settles the replay-journal residual (§8e.3, §8e.5). **N8:** the deadline is integer unix seconds
  (§8e.7). **N9:** the terminal `closed` mark (§8e.6). **N10:** the boundary seam a content checkpoint rides (§8e.3),
  with the staging rule cited to `s8f`.
- **Made explicit:** the extra supersessions this file found (the records table). **Fixed:** the 7h guard (10a)
  backtick defect (§8e.13). **Re-measured:** the `fsm.rs` comment and the `story_trailer` subject claim (§8e.13).
- **Checkpoint 8e:** re-runs only its entry gate plus the checkpoints this stage's own edits make provisional. It
  skips its nested re-runs when a later checkpoint calls it as an entry gate (§8e.15). Its floor is re-derived at
  stage open.

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
  producers; Gate and Story are both minted as of s7h.2; discharged by s7h"]` *(noted 2026-09-25, s8a fix round 1:
  since `s8a` §8a.5 (ADR-0160) that reason ends `discharged by s8e`, the tail §8e.13 reads)*, and ADR-0160 names the
  route: *"declare
  the slice on the item record (or decide that a fix lane has exactly one, and say so), then a `slice_sealed`
  producer at the review stage's boundary, then the trailer through the same `commit_with_trailers` path `Story:` now
  uses."*
- **The run worktree was missing too, and it is now `s8d-ii`'s.** D21-44 measured that a `-p` child could not edit
  a tree under `.git/redkiln/worktrees/`, and `recreate_run_worktree` destroyed the tree at every run start. By this
  stage's open, Checkpoint 8d-ii has moved it to `.redkiln/worktrees/<item>`, derived in one place, guarded by W1,
  and reused rather than re-created (W2). This stage records it (§8e.4) and keys its removal to the lane's end
  (§8e.6).
- **The lane loop has no automated test.** `grep -rn "verbs::run::run(" crates/*/tests` returns nothing. ADR-0161
  names the cost: two of its three defects *"are in the seam between the kernel's report and the composition root's
  loop — which no unit fixture in either crate covers"*.

**This step takes decisions, and they were already taken.** The operator took them in D21-41 (a) and (e), D21-42 7,
8a–8d, 24b and 24c, D21-43 N5–N10, and D21-46 R2 and R3 (`spikes/DECISIONS-2026-09-21.md`). They are
written as records at this stage, and each record takes **the next free id at writing** — this file assigns none.
Where a decision moves a `## Decision` sentence or a forbids bullet of an accepted record, the record is a supersession
in part (ADR-0092 rule 1), never an erratum. Every record is written `proposed` and flips only on the operator's dated
ledger line *(dated 2026-09-28, `s8e` close: ADR-0181 … ADR-0187 flipped `accepted` on D21-98 … D21-104, each on its own dated line under D21-64 ruling 10)*. Where this file found a residual the decisions do not name, it says so in §8e.13 rather than taking it
silently.

**What this step owns** (the plan's row for `s8e`, widened by the later entries, and narrowed by D21-46 R1):
- D21-41 **a**, and **e** in both halves — `Slice-Verdict:` AND the tier-1 writer.
- D21-42 **7**, **8a**, **8b**, **8c**, **8d**, **24b** (L3) and **24c** (`terminal_read` plus the first FakeHarness
  end-to-end test of `verbs::run::run`).
- D21-43 **N5**, **N6**, **N7**, **N8** and **N9**, and **N10**'s seam (the staging rule is `s8f`'s).
- D21-46 **R2** (the forward migration to an append-only gate-transitions table) and **R3** (the interim `item` key,
  outside the canonical digest).

**What it does not own, and who does.** ADR-0164's flip (the operator's ledger line).
- **`s8d-ii`:** the run worktree whole (D21-44; D21-45 W1, W2, W4 and W3's carry; D21-46 R8). That is the root
  `.redkiln/worktrees/<item>` and its one derivation, the item-id validator, the symlink guards, `RK_BASE`, the lock,
  the re-entry check, reuse and the discard, the removal rule, the filter-driver refusal at every scope, `init`'s
  exclude block, `LC_ALL=C`, the ignore rows and Checkpoint 7f's run arm and cleanup line. This stage **consumes** it:
  §8e.4's `worktree_assigned` row journals the path, `reused` and the discard list `s8d-ii`'s lifecycle function
  returns, and §8e.6's `closed` mark is the trigger for `s8d-ii`'s removal rule.
- **`s8f`:** the landing checkpoint commit and its staging rule (D21-41 b; D21-42 9; D21-43 N10). This stage builds
  the seam the content act rides (§8e.3) and says what it guarantees. `s8f` also owns `dispatch_completed`'s
  `activity_id_at_version` and the cost breakdown (D21-42 12b), and cites §8e.10 for the general `item` key. It also
  owns the stage advance's refusal on an unanswered `render.structured-drift` gate (`s8f` §8f.5). This stage's advance
  folds the gate transitions (§8e.7) and builds no drift refusal of its own.
- **`s8g`:** the verify record and `verify_completed`.
- **`s8h`:** cards, models, the launcher and the run environment. That includes the `gate_expiry` table (D21-42 21a), to
  which this stage adds no row because it mints no gate kind (§8e.7).
- **`s8j`:** the fresh fix item and the story item (D21-42 18; D21-41 f), each carrying `slice` (§8e.8) and
  `[project] name`. `s8j` also re-points Checkpoints 7h and 7i to fixture items. And it owns every amendment to
  `s8-exit-criterion.md`: the mechanical set-and-order assertion D21-42 7 puts on criterion (1), the anchored
  differential in (2) (§8e.9), and the three `redkiln run` sites through the launcher.

**No stage before `s8i` dispatches to a real account, and this one dispatches nothing.** Every run in it is over a
fixture repository, a scripted `FakeHarness`, or the fixture child.

**Order of work, and why.**
1. §8e.0's measurements, before any code.
2. §8e.1: D21-42 24b says L3 is fixed *"before the writer builds on it"*.
3. Nothing of the run worktree. `s8d-ii` made its path final at this stage's entry gate, which is why §8e.4's
   `worktree_assigned` row can record it.
4. The writer and R2's migration (§8e.2), the kernel-owned order (§8e.3) and the ownership row (§8e.4). Every mark
   written after that goes through them.
5. The interim `item` key (§8e.10), because §8e.4 and the fold read by item.
6. The three axes and the terminal mark (§8e.5–§8e.8), then the rebuild (§8e.9).

The end-to-end tests (§8e.11) are written **first, red**, and they go green last.

## Records this step writes

Each record below is **a new record (next free id at writing)**, in the ADR-0085 `### Option — <name>` heading form
that D21-42 24a makes binding from ADR-0164 onward. The adversarial pass may fold two into one. It may not drop a
supersession listed here without a reason written into the record. Every supersession listed is **explicit**, including
the ones this file found beyond the ledger's list.

| record | executes | supersedes in part | cites without superseding |
| --- | --- | --- | --- |
| **the stage lives on the run ref** | D21-41 a; D21-42 8d's two defaults; D21-43 N7 (the planner reads the fold) and N9 (the terminal mark) | ADR-0105 clause 3 (the `Story:` value delegation to design §5.4's row); ADR-0155 decisions 1, 2 and 5(a), and its bullets *"A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes."* and *"A `Story:` value carrying a stage, a boundary, or any third token, until the grammar is widened by a record that supersedes `0105` clause 3."*; ADR-0157 decisions 4 and 5, and its bullets *"Describing a lane as resumable across processes, or a lane's completion as anything other than within-one-process."* and *"Reading a second invocation's exit 0 as progress, or as evidence that the lane finished."*; ADR-0161's bullet *"Describing the lane loop as covered by tests, or reading a green suite as evidence about it."*, and only once §8e.11's tests pass | ADR-0109 decisions 1 (N7 is execution under it), 3 (**NOT superseded**: resume reads all three axes once `Slice-Verdict:` lands) and 4; ADR-0161 decision 1 (`stages_entered` keeps replays idempotent); ADR-0159 |
| **the tier-1 writer, git-first, owned by the kernel** | D21-41 e (second half); D21-42 8a, 8c, 24b's placement; D21-43 N7 (the `git_op` journal) and N10 (the seam a content act rides) | ADR-0158 decision 3 (*"No document, record or checkpoint may state that AC-162 is met, that a tier-1 row is written, or that a failed git write leaves zero store rows"*) and its bullet *"Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis"* — **only** for the criteria this stage measures (§8e.14), which are all four: AC-160 on its `s6` trybuild fixture, re-run here; ADR-0155 decision 5(b) (*"No document may state that AC-162 is met for this axis, or for any."*) and its bullet *"Stating or implying that AC-162 is satisfied for stage changes 2..N of a lane"*, on the same condition | ADR-0003 (the port **count** stays five; a method is a surface change); ADR-0105 clauses 1 (N7 executes it), 2, 4, 8 and 9; ADR-0105 clause 5, **not superseded**: every git write this stage adds is a `commit`, which its closed list names (*"`commit` (incl. `--allow-empty`, `--trailer`)"*), and the heal and the rebuild write nothing to git (D21-47's clause-5 rule); ADR-0158's bullet *"Writing `t1_*` rows from `rk-cli`, or any SQL against them outside `rk-store`"*, **not superseded**: honoured on the reading D21-48 X10 takes (*""From `rk-cli`" names where the write runs. The adapter's `Capability::Rebuild` owns the SQL, so there is no supersession."*), which the record states (§8e.9); ADR-0109 decision 2; ADR-0118 clause 4 |
| **`Gate:` carries kind, reach and state** | D21-42 8b; D21-43 N6 (which *"Extends D21-42 8b's record"*) and N8; D21-46 R2 (the `0002` forward migration to an append-only `t1_gate_transition` table, §8e.2) | design §5.4's `Gate:` grammar row, and its *"RFC 3339"* deadline spelling (N8); ADR-0105 clause 3's value delegation, for `Gate:`; design §5's `gate_answer_void` row's tier (`2`) and producer (*"`reconcile()`"*) | ADR-0105 clause 10 (a fresh mark per re-reach); ADR-0109 decision 1 (a void answer re-opens the gate) and decision 2 (its parenthetical read as illustrative — §8e.7 says why); ADR-0131 decision 6 (the store is disposable, which is R2's ground); AC-174; AC-180 (the migration is explicit, never in place); `s7d` §7d.18's RAISED widening item, which this discharges |
| **the slice is declared, and sealed on both outcomes** | D21-41 e (first half) | **nothing**. It honours ADR-0160's first bullet **in full** — *"Emitting a `Slice-Verdict:` trailer whose `<slice>` is the item id, the project, the stage, the run id, or any value not declared as a slice."* — and §8e.8 refuses each of those five by name | ADR-0160 decision 6; ADR-0162 decision 2 (scoped to its own work, so re-stated here, not superseded); ADR-0161 decisions 1–2; ADR-0159 (the refusal pattern) |
| **the interim `item` key** | D21-43 N5; D21-46 R3 (*"**Carried outside the canonical digest**, as ADR-0131 places its tag set"*) | **nothing.** The key is carried beside the event, outside `payload` and outside `canonical_events`, so ADR-0131's first forbids bullet (*"Forbids putting an item id, a run id, a commit SHA, a stage, … **inside** `rk-types::Event.payload` for the purpose of addressing"*) is honoured, not superseded. N5's *"Supersedes ADR-0131's first forbids bullet in part"* is overtaken by R3 (§8e.13, finding 12) | ADR-0131 decision 1 (the reserved `item` key), decision 2 (*"Tags are carried on the event and are OUTSIDE `canonical_events`"*, the placement R3 follows), its second forbids bullet (honoured: `canonical_events` stays `{kind, payload}`), decision 4 (whose substance, an `item` on the item-less tier-1 kinds, the interim carrier delivers ahead of the tag), and decision 8 with its bullet *"Forbids editing `rk-ports`, `rk-kernel`, `rk-adapter-sqlite` or any other closed crate **for this record** before Checkpoint 8 is recorded as passed"* — honoured on the argument §8e.10 states, and superseded in part for the interim carrier alone if the adversarial pass rejects it; ADR-0163 (the precedent; `rate_limit_reading.item` stays where ADR-0163 put it) |
| **`worktree_assigned`**, **`stage_empty`** and **`git_op`**, three kinds | D21-42 8d; D21-42 7; D21-43 N7 | nothing — minted by ADR-0116's route: *"The table row and the `rk-types` variant land in the same change."* | ADR-0105 clauses 1 and 8; AC-529; the design §5 `reconcile_started` row's RT-47 members (§8e.4 says what happens to them) |

**Attendant edits, each dated and each in the commit that makes it true:**
- design §5.4's `Story:` and `Gate:` rows;
- design §5's event table: three new rows, the `gate_answer_void` row, and a note that the interim `item` key sits
  beside the payload per §8e.10;
- design §3.3's `EventStore` row (a dated note naming the tier-1 method);
- dated register notes on **AC-444** (its drift counts are pinned to the grammar table, and it is corpus-bound
  Phase-2, so the note re-pins nothing Phase 1 asserts) and **AC-180** (its fixture's `N−1` is now version 1, the
  fixture is checked in, and the explicit migration is the rebuild, §8e.2);
- dated register re-point notes on **AC-161** (the route half's crate and repository), **AC-163** (the replay's corpus
  is the fixture lane) and **AC-518** (the write-path lint's scope at this stage is the commit write), each as §8e.14
  states it;
- a `.gitattributes` row pinning `crates/rk-store/tests/fixtures/*.sqlite3` as `binary` (§8e.2's AC-180 fixture);
- a dated re-amendment of `s7h` §7h.2, whose ADR-0155 decision-6 erratum says *"the first stage boundary"*;
- the dated amendments to Checkpoints 7h, 7f and 8a that §8e.13 spells out, with 8a's `PROGRESS.md` §1 row set to
  `re-run pending` (D21-47: the stage that moves an earlier checkpoint's premise amends its guard in the same commit);
- a ledger entry.

*(dated 2026-09-28, s8e build, package S8E-P4)* **Two records written, both `proposed`:** the stage-location row is **ADR-0183** (`references/adr/0183-the-stage-lives-on-the-run-ref-a-stage-bearing-story-mark-at-every-boundary.md`; supersedes in part `0105`, `0155`, `0157` and `0161`, every supersession this table lists, the `0161` bullet only once §8e.11's tests pass) and the `Gate:` row is **ADR-0184** (`references/adr/0184-the-gate-value-carries-kind-reach-and-state-with-an-integer-deadline.md`; supersedes in part `0105` clause 3's `Gate:` delegation, plus the two design rows by dated line). Neither supersedes ADR-0109: decision 3 is honoured (D21-42 8d), and decision 2's list is read as illustrative with the fallback stated. Back-links and the README Supersessions-table rows move in each flip commit, as `0181`'s do. *(dated 2026-09-28, `s8e` close: they moved in the flip commit, on D21-100 and D21-101: `0183` appended to the `superseded_by` of `0105`, `0155`, `0157` and `0161`, and `0184` to `0105`'s, with five Supersessions-table rows; `0181`'s two, on `0155` and `0158`, moved on D21-98.)* README and preamble census: 184 records, 176 accepted / 4 superseded / 4 proposed, next free id **0185**, 82 supersession links from 56 records with 75 reciprocated, forbids blocks 170/11/3 (measured by script). Attendant edits made in this package: design §5.4's `Story:` and `Gate:` rows, design §5's `gate_answer_void` and `gate_expired` rows, the AC-444 register note, and the `s7h` §7h.2 re-amendment. No ledger entry was written (the next id, D21-94, is left to the orchestrator so parallel packages do not collide).

*(dated 2026-09-28, s8e build, package S8E-P7)* **The slice row is written, `proposed`: ADR-0186** (`references/adr/0186-the-slice-is-declared-and-sealed-on-both-review-outcomes.md`). It supersedes nothing, as the row says: ADR-0160's first bullet is honoured in full (five values refused by name at the mint, the run id stated as undeclarable), decision 6's *"at the review stage's boundary"* is read as *"at the review stage's exit outcome"* with the fallback supersession stated, ADR-0162 decision 2 is re-stated with Checkpoint 7h guards (6)/(7) and 8a guard (7) as amended here, and the S-SUBTLE-CB non-claim is named. It also states D21-93's fold scoping of a seal (decision 4) and its residual (a healed or rebuilt seal carries no `stage`). ADR-0181 gains dated notes on decision 11 (the `Snapshot` row, the append-only `rebuild`, the rebuild's rows being the heal's) and on its evidence. README and preamble census, measured by script: 186 records, 176 accepted / 4 superseded / 6 proposed, next free id **0187**, forbids blocks 172/11/3, supersession links unchanged. Attendant edits made in this package: design §5's `slice_sealed` row and §3.3's `EventStore` row (dated), the AC-161 and AC-163 register re-point notes and a line on AC-518, the dated amendments to Checkpoints 7h, 8a and 7f with their prose, and `PROGRESS.md` §1's 7h, 8a and 7f rows set `re-run pending`. No ledger entry was written (the next id is the orchestrator's to assign).

*(dated 2026-09-28, s8e build, package S8E-RULINGS)* **D21-92's record is written, `proposed`: ADR-0187** (`references/adr/0187-a-routing-null-is-journaled-when-its-route-continues-the-walk.md`). It supersedes ADR-0174 decision 7 in part, its null sentence only (the D2177-8d paragraph's `null` bullet, the replay sentence's *"and it is not `null`"*, and *"or a `null`"* in the D21-77 forbids bullet), and discharges the residual that bullet named. ADR-0174's `superseded_by` back-link and the README Supersessions-table row move in `0187`'s flip commit, as `0180`'s did. *(dated 2026-09-28, `s8e` close: they moved on D21-104; `0187` is appended to ADR-0174's `superseded_by`.)* README and preamble census: 187 records, 176 accepted / 4 superseded / 7 proposed, next free id **0188**, 83 supersession links from 57 records with 75 reciprocated, forbids blocks 173/11/3 (measured by script). Design §5's `activity_committed` row takes a dated amendment. D21-94's three confirmations are dated one-line notes in ADR-0182 decision 4, ADR-0184 decisions 2 and 5 and ADR-0185 decision 3.

*(dated 2026-09-28, s8e build, package S8E-P5)* **The three-kinds row is written, `proposed`: ADR-0185** (`references/adr/0185-worktree-assigned-stage-empty-and-git-op-three-tier-two-kinds.md`). It supersedes nothing, as the row says. It fixes `stage_empty`'s tier (2) and closed reason set (one member, `no_applicable_activity`, `rk_types::event_kind::STAGE_EMPTY_REASONS`), the deleted-database reading for `worktree_assigned` (a tier-2 disposal: `answer` refuses fail-closed and the next `run` re-enters and writes it again, so no next action of `run` changes), and RT-47's disposition (**kept** as tier-2 context that no reader resolves ownership from; `answer`'s `recorded_ownership` is deleted, and design §5.4 carries the dated note). Design §5.4's event table gains the three rows. README and preamble census, measured by script: 185 records, 176 accepted / 4 superseded / 5 proposed, next free id **0186**, supersession links unchanged (82 from 56 records, 75 reciprocated), forbids blocks 171/11/3. The AC-518 register re-point note is written (§8e.14). No ledger entry was written (the next id is the orchestrator's to assign, so parallel packages do not collide).

### 8e.0 Stage-open measurements — re-taken at Checkpoint 8d-ii's close, before any code

Each is read-only. A 🛑 line is a STOP: the finding goes to the operator, and the stage does not choose around it.

1. **Where the legacy tree is registered.** At `c5d19d7`, `git worktree list` lists
   `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count` at `73c087a`. `s8a` runs Checkpoint 7f whole,
   and 7f's block, as `s8a` leaves it, ends with `git worktree remove --force
   .git/redkiln/worktrees/rk-fix-doctor-test-count`. `s8a` guard (11) restores the registration after that run, so it
   should still be present (`s8d-ii` §8d-ii.9, finding 4). Record which. Checkpoint 8e asserts the ref and its mark,
   never the registration.
2. 🛑 **The shipped store, and every checked-in store, before R2's migration.** Read `rk_identity.schema_version` and
   the row count per table and per `t2_event` kind, `mode=ro`. At `c5d19d7` the shipped `.git/redkiln/store.sqlite3`
   stamps version **1** and holds `t1_stage_transition` 0, `t1_gate` 0, `t1_slice_seal` 0 and `t2_event` **6**
   (`dag_assembled` 1, `reconcile_started` 3, `stage_transition` 2). `git ls-files` lists no `.sqlite3` file, because
   every test store is created at open. This stage checks in the first one, §8e.2's AC-180 fixture: a version-1 store
   minted on the entry tree, before `0002` lands, with zero `t1_*` rows. These six tier-2 rows are what the
   migration's rebuild drops (§8e.2), and the record lists them. If a `t1_*` row is found anywhere, STOP: §8e.2
   drops `t1_gate` on the precondition that it is empty.
3. **The golden logs' canonical digests (R3).** Read every golden log under `CORPUS_PIN` and every `-text`-pinned
   fixture in `.gitattributes`. Count the rows per item-scoped kind, and record each golden log's canonical digest.
   R3 carries the key outside the digest, so none may move, and
   `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` pins the recorded values (§8e.10).
   This is a baseline, not a STOP. A digest that moves is a defect in the carrier, not a cost to put to the operator.
4. **The tier-1 tables are empty everywhere.** `s8f` §8f.2 measured `t1_*` at 0 rows in the shipped store at
   `ca8f387`, and item 2 re-reads it. §8e.2's writer-side resolutions, and the migration's drop of `t1_gate`, rely on
   no existing row having to migrate.
5. **Checkpoint 7f's current literal and run arm**, as `s8a` and `s8d-ii` left them: the ignored-lint reason, the exact
   result line, and the run arm, which `s8d-ii` §8d-ii.7 moved into a throwaway clone. §8e.13 amends the literal.
   **And Checkpoint 8a guard (7)'s armed-lint lines**, as `s8a` §8a.10 left them: the `ignored, ARMED:` line, the
   reason's stage name and the `1 ignored` count. §8e.13 amends them too.
6. **The test module names are free.** This stage adds `rk-kernel` `tier_one`, and `rk-cli` `lane_e2e`,
   `slice_verdict` and `worktree_assigned`. It extends `rk-git` `trailers`, `rk-store` `store`, `rk-kernel` `replay`,
   `lane_order` and `fsm`, and `rk-cli` `answer` and `doctor`. If a predecessor created one of the new modules, the
   tests go into it. The module-qualified names in §8e.12 stay as they are.
7. **The whole-suite floor.** The expected value is **re-derived at stage open from Checkpoint 8d-ii's recorded
   count**, plus this stage's **74**: 73 new tests, plus the one ARMED lint un-ignored. Computed once, on the final
   chain: 740 (ADR-0164's evidence, `992d257`) + 9 (8a) + 25 (8b) + 78 (8c) + 62 (8d) + 50 (8d-ii) = 964 at
   Checkpoint 8d-ii, so ~~964 + 74 = **1038**~~. Replace ~~1038~~ the value in Checkpoint 8e's `FLOOR=` line with Checkpoint 8d-ii's
   recorded count plus 74, by a dated edit, before the block's first run. It is never lowered to make a red run
   green. *(Earlier drafts read 931, 944, 984 and 991 over stale chains. The adversarial review of 2026-09-24 moved
   this stage's own share from 67 to 74, and the final consistency pass of 2026-09-24 re-derived the chain.
   *D21-48 (2026-09-24), floor propagation:* it read 947 + 74 = 1021 before `s8c`'s 78, `s8d`'s 62 and `s8d-ii`'s 50.)*
   *(dated 2026-09-28, s8e build)* **The floor is re-derived at stage open from Checkpoint 8d-ii's RECORDED count** (`runbook/PROGRESS.md` row 8d-ii: *"whole suite **1034 passed / 0 failed / 10 ignored over 44**"*, Checkpoint 8d-ii passed whole at `497266b`, re-run green after `be32cda`): 1034 + 74 = **1108**. The chain's expected 964 is superseded by the recorded 1034; the recorded count governs. The owed-in `lane_e2e` test(s) for MF3j and for `run()`'s review-routes binding (§8e.11's two receiver notes) are **outside** the 74, so when they are named the floor, Checkpoint 8e (2)'s `sel rk-cli 30` and §8e.12's count move again by dated edit. Checkpoint 8e (8) now reads ~~`FLOOR=1108`~~ `FLOOR=1110` *(dated 2026-09-28, s8e build, package S8E-P2)*: §8e.11's two owed-in tests are named, `lane_e2e::run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer` and `lane_e2e::run_binds_the_review_routes_for_a_shallow_clone_step`, and 1034 + 76 = **1110**. *(dated 2026-09-28, s8e build, package S8E-P7: ~~`FLOOR=1110`~~ ~~`FLOOR=1111`~~ `FLOOR=1112` — S8E-P6's kernel test (1111), then D21-93's `fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition` (1034 + 78 = **1112**).)* *(dated 2026-09-28, s8e build, package S8E-RULINGS)* ~~`FLOOR=1112`~~ `FLOOR=1113`: D21-92's two `on_false_exec` tests are new and `s8d`'s `a_null_until_or_gate_when_is_not_journaled` is replaced, so 1034 + 79 = **1113**. *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY)* ~~`FLOOR=1113`~~ `FLOOR=1116`: D21-94's entry refusal adds three `rk-cli` tests (§8e.12's dated paragraph), so 1034 + 82 = **1116**. *(dated 2026-09-28, s8e build, package D2196-8e)* ~~`FLOOR=1116`~~ ~~`FLOOR=1118`~~ `FLOOR=1119`: refuting round 1's two `rk-cli` tests (package R1FIX-8e, recorded in §8e.12 and Checkpoint 8e (8)) made 1034 + 84 = **1118**, and D21-96's `rk-store` test `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` makes 1034 + 85 = **1119**.
8. **Every reader of `fsm::RunState::terminal()`, and what it makes of a lane that continued past review** (§8e.8).
   At `fc80687` two code lines read it: `replay.rs`'s comparison (`state.terminal().canonical_bytes()`) and
   `RunState`'s own JSON (`"terminal": self.terminal().to_json()`). Record every caller of that JSON as well. For
   each reader, record whether an `approved` seal at `review` followed by later stages reads as `Sealed`, and whether
   that misleads it. 🛑 If one does, the slice record says how the fold scopes a seal before §8e.8's producer lands.

**Stage-open readings** *(dated 2026-09-28, s8e build, package S8E-P0; HEAD `766854c` on `build/phase-0-to-1`,
before any code. Store backed up first: `.git/redkiln/store.sqlite3`, its `-wal` (0 B) and `-shm`, and the
`rebuilt` sidecar, copied to `target/store-backups/s8e-open-766854c/`, with sha256 matching:
`e83d295e37a786d98db17a981696cfe1f2d9c6eb9983cf65eb453cfe86721130` for the main file. Every cargo run went
through the no-dispatch helper inside a private `git worktree add --detach target/wt-s8e-p0 HEAD`. That tree held
three throwaway probe tests, was never committed, and was removed afterwards.)*

- **Item 1.** `git worktree list` →
  `D:/repos/redkiln-rs/.git/redkiln/worktrees/rk-fix-doctor-test-count 73c087a [rk-run/rk-fix-doctor-test-count]`.
  The legacy registration is **still present**, beside the main checkout and `D:/repos/redkiln-rs.swt 8b64201
  [spike/s-h1b-s-wt]`. `git rev-parse rk-run/rk-fix-doctor-test-count` → `73c087a82245b342811208655323ae4f49bf879b`.
  `git log -1 --format=%B rk-run/rk-fix-doctor-test-count | grep '^Story:'` →
  `Story: redkiln-rs/rk-fix-doctor-test-count`. `git for-each-ref refs/heads/rk-run/` lists that one ref only.
- **Item 2 (🛑, CLEARED).** `git ls-files '*.sqlite3'` → empty. The shipped store was read through Python
  `sqlite3` with `mode=ro`, and its sha256 did not change across the read. `rk_identity` →
  `(id 1, root_commit cc5a2589450a5a08136692b0b4fd03cfd5be54d3, common_dir D:/repos/redkiln-rs/.git,
  schema_version 1)`. Row counts: `t1_stage_transition` 0, `t1_gate` 0, `t1_slice_seal` 0, `t2_event` **11**,
  `t2_memo` 0, `t2_usage` 0. Per `t2_event` kind: `dag_assembled` 1, `gate_opened` 1, `reconcile_started` **7**,
  `stage_transition` 2. Every row has `anchor_sha` NULL. **No `t1_*` row exists anywhere, so the STOP does not fire.**
  **Drift from the recorded state:** the six rows recorded at `c5d19d7` are now eleven, adding four
  `reconcile_started` and one tokened `gate_opened` (ids 5–11). So the "six tier-2 rows" that §8e.2 says the
  migration's rebuild drops must be re-counted when that rebuild runs, and the record lists what is present then.
- **Item 3 (baseline).** There is one golden log under `CORPUS_PIN`: `redkiln-corpus` at
  `61f590e51cbb00301c310e46aab615563a7787b8` holds `golden-logs/implement-bf1d8cec3b22d288/` only. Its
  `events.jsonl` is the same git blob, `c2da0225807ff925d37f330eb79fe0e91da34035`, as the committed
  `crates/rk-kernel/tests/fixtures/logs/implement-bf1d8cec3b22d288/events.jsonl`. **The canonical digest was printed
  from inside the `rk-kernel` test binary** as `rk_types::digest::sha256(read_run(dir).canonical_events())`, over
  both copies. Both read
  `canonical_events_sha256=73f7553df2f84cc7b931a39b276fcfdb0b36371bb93ea461a9b10ca8a47e4e90 lines=8 bytes=2450`,
  which equals the raw file's sha256, so the committed file is already canonical. Rows per kind in it:
  `activity_committed` 2, `dag_amended` 1, `dag_assembled` 1, `dispatch` 2, `gate_opened` 1, `stage_transition` 1.
  There is no `slice_sealed`. The per-kind counts of the other `-text`-pinned event fixtures, read from `HEAD`'s
  blobs, all with zero CR:
  - `doctor/`: `RK-D-ADVISORY-LEDGER` advisory_created 2 / advisory_retired 2; `RK-D-CREDENTIAL-RETAINED`
    dispatch_completed 1; `RK-D-HEADROOM-MARGIN` rate_limit_reading 3; `RK-D-QUARANTINE-LIVE` quarantine_granted 2 /
    quarantine_removed 1; `RK-D-REVIEW-REEXEC` review_block 2; `RK-D-SINK-COUNT` advisory_created 1 / artifact.write
    1 / review_block 1; `RK-D-UNREAD` artifact.read 51 / artifact.write 102; `clean` activity_committed 1.
  - `cost/`, 22 `rows.jsonl`: `dispatch`, `dispatch_completed` and `rate_limit_reading` only, plus `memo_miss` 1 in
    `empty-counts-dispatch-rows-only`. `own-noise-kinds` also holds activity_committed 2, dag_assembled 1,
    memo_miss 1, reconcile_started 2 and stage_transition 1.
  - `exit/*/frames.jsonl` and `cost/run-frames.jsonl` are stream-json frames, not events.
- **Item 4.** `t1_*` = 0 in the shipped store (item 2's reading), and 0 in the only other store this stage creates,
  the fixture below.
- **Item 5.** Checkpoint 7f, as `s8a` and `s8d-ii` left it, reads three lines.
  - The literal (`s7f` block):
    `grep -qE 'test result: ok\. 2 passed; 0 failed; 1 ignored'`.
  - The ARMED line:
    `grep -qF 'test conformance::conformance_every_trailer_key_has_a_producer ... ignored, ARMED: Slice-Verdict has 0 producers'`.
  - The reason regex: `grep -vE 'discharged by s8e'`, with an armed count of exactly 1.

  The run arm is the throwaway clone: `clone="$c7f/c"`, then
  `git clone --quiet --no-local . "$clone"`, then `init` and `run rk-fix-doctor-test-count --lane fix` with the clone
  as the working directory, ending `rm -rf "$c7f"`. Checkpoint 8a guard (7) reads:
  `grep -qF 'test conformance::conformance_every_trailer_key_has_a_producer ... ignored, ARMED: Slice-Verdict has 0 producers'`,
  `grep -qF 'discharged by s8e'` and `grep -qE 'test result: ok\. 0 passed; 0 failed; 1 ignored'`.
  `crates/rk-kernel/tests/it/conformance.rs` carries
  `#[ignore = "ARMED: Slice-Verdict has 0 producers; Gate and Story are both minted as of s7h.2; discharged by s8e"]`.
- **Item 6.** The new modules are free: `rk-kernel` `tests/it/tier_one.rs` and `rk-cli` `tests/it/lane_e2e.rs`,
  `slice_verdict.rs` and `worktree_assigned.rs` are absent, and no `main.rs` declares them. The modules to extend
  exist and are declared: `rk-git` `trailers`, `rk-store` `store`, `rk-kernel` `replay`, `lane_order` and `fsm`, and
  `rk-cli` `answer`, `doctor` and `root`. **One finding for §8e.10:** `crates/rk-kernel/tests/it/replay.rs` opens
  with `#![cfg(feature = "corpus")]`, so the whole module compiles only under `--features corpus`. A test named
  `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` would therefore run zero times under
  Checkpoint 8e (2)'s unfeatured `sel rk-kernel 24`, which asserts its `... ok` line, and zero times under (8)'s
  unfeatured whole suite. The name and the placement cannot both stand as written.
- **Item 7.** 1034 + 74 = **1108**, by the dated edits above and in Checkpoint 8e (8). *(dated 2026-09-28, s8e build, package S8E-P2)* Moved to 1034 + 76 = **1110** once §8e.11's two owed-in tests were named. *(dated 2026-09-28, s8e build, package S8E-P6)* Moved to 1034 + 77 = **1111** (~~1110~~) once `tier_one::a_closed_mark_heals_as_the_lanes_end_transition` was named.
- **Item 8 (🛑, RAISED; not cleared).** Two code lines read `terminal()`:
  - `replay.rs`'s `fsm_residue` (`as_text(state.terminal().canonical_bytes()?)?`);
  - `RunState::to_json` (`"terminal": self.terminal().to_json()`).

  The JSON member's only reader is `RunState::canonical_bytes()`. That has no caller under `crates/*/src/` besides
  `fsm.rs`'s own unit test, and its test callers (`gates.rs`, `fsm.rs`) compare two folds of one journal.
  `fsm_residue` feeds both `replay::record` and `replay::resume`. The `run` verb persists the resume's value through
  `replay::write_run` as `run.json`'s `terminal_state`. The resume folds the store's whole prior journal, not just
  this run's rows.

  **Measured in-binary** by folding `review → slice_sealed{approved} → release`, with four arms:
  - no gate: `terminal={"terminal":"sealed","verdict":"approved"}`;
  - an open gate at `release`: `awaiting_gate=Some(d2d32e9a…)` **but** `terminal={"terminal":"sealed",…}`;
  - a further `release → operate`: `sealed`;
  - control, the same gate with no seal: `terminal={"terminal":"awaiting_gate",…}`.

  So an `approved` seal at `review` reads as `Sealed` in both readers for every later stage, and it shadows an open
  gate. It misleads the `fsm_residue` reader: every post-review run's persisted `terminal_state` says `sealed`,
  including a run halted at an open gate. AC-008's terminal-state half also cannot tell a kernel that does nothing
  after the seal from one that works, because both sides compute `sealed`. The shipped packs reach this:
  `bug-standard` and `story` both order `release`, `operate` and `closeout` after `review`. No product code
  reads the persisted `terminal_state` today (`explain` does not render it), and §8e.7's `terminal_read` reads
  `awaiting_gate()`, which the probe shows is correct past a seal. Per this item's STOP, the fold's scoping of a seal
  is put to the operator before §8e.8's producer lands, and this package does not choose it.
- **Terminators, byte-counted** (Python `bytes.count`, not grep). Every file this stage patches is **LF with 0 CR**
  in both the working tree and the index blob:
  - runbook: this file, `runbook/PROGRESS.md`, and the `s7h`, `s8a` and `s7f` stage files;
  - records and design: `references/2026-09-08-architecture-design.md`, `references/2026-09-08-acceptance-criteria.md`
    (the register), `spikes/decisions-2026-09-14-drafts/C-register-amendments.md`, `spikes/DECISIONS-2026-09-21.md`,
    `references/adr/README.md` and `.gitattributes`;
  - code: `rk-store/src/conn.rs`, `rk-cli/src/verbs/run.rs`, `rk-cli/src/verbs/answer.rs`,
    `rk-cli/tests/it/answer.rs`, `rk-cli/src/doctor/rebuild.rs`, `rk-kernel/tests/it/conformance.rs`,
    `rk-kernel/src/workflow/fsm.rs` and `rk-kernel/tests/it/fsm.rs`.

  Patches write LF.
- **AC-180's version-1 fixture (§8e.2), minted on the entry tree before `0002` exists.**
  `crates/rk-store/tests/fixtures/store-v1.sqlite3` is 40,960 B, sha256
  `603d60041fc41e0788deb8cc80957c330dd3d8101b2cdadb887fbfd42f055dd3`, git blob
  `9796718ca4ff7b415bc8609b29fae6bd86db9f93` (`hash-object` is equal with and without filters). It was minted
  through `rk-store` alone, in `SqliteStore::open_over_registry`'s order:
  - `Writer::open`, then in one transaction `identity::read` (none), every `schema_lint::all()` migration unchanged,
    and `identity::initialise`;
  - then three rows through the adapter's own `INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2,
    NULL)`.

  It stamps `schema_version` **1** with a path-neutral identity: `root_commit ac180ac180ac180ac180ac180ac180ac180ac180`,
  `common_dir /fixture/store-v1/.git`. Read back with `immutable=1`: `t1_stage_transition` 0, `t1_gate` 0,
  `t1_slice_seal` 0, `t2_event` 3 (`reconcile_started` 1, `stage_transition` 2), `t2_memo` 0, `t2_usage` 0. The
  writer closed before the copy and left no WAL content.

  `identity::check` compares `root_commit` **before** `schema_version`. So a test that wants the migration refusal
  opens against that same root, or checks the read stamp against an opening identity that carries it.
  `.gitattributes` gains `crates/rk-store/tests/fixtures/*.sqlite3 binary`, and `git check-attr` reads
  `binary: set`, `text: unset`, `diff: unset` and `merge: unset`.

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

**The schema frictions.** The brief measured two, and this file found a third. Two are resolved at the writer, and
the third takes D21-46 R2's migration:

1. **`t1_stage_transition.tree_hash TEXT NOT NULL`.** Resolved by ADR-0105 clause 4, whose first sentence already
   answers it: *"When a checkpoint commit is due, the recorded tree is that commit's tree."* The anchor is the trailer
   commit, so the row's tree is `<anchor>^{tree}`, read through `GitOp::RevParse`, which is on the wrapper's read
   list. An empty commit's tree is its parent's, which is deterministic.
2. **`t1_stage_transition.from_stage TEXT NOT NULL`.** The lane-entry row (`stage_entry`'s `{"to": …}`) has no
   `from`, and `stage_entry`'s own doc says so. **Resolution: `t1_stage_transition` holds boundaries — rows with a
   `from` — and a lane entry is not a transition between two stages.** The entry mark's tier-1 durability is its
   `Story:` trailer plus the anchored `t2_event` row, and the rebuild reproduces exactly that split (§8e.9).
   **Not relaxed by the migration.** R2's `0002` migration (friction 3) could relax the column too. It does not,
   because the resolution above is a meaning, not a workaround.
3. **`t1_gate.token TEXT NOT NULL UNIQUE` beside `answered INTEGER NOT NULL DEFAULT 0`** — found here, bound by N6
   (*"`t1_gate` is append-only and state is a fold."*), and **settled by D21-46 R2**: *"**(A) A forward migration to
   an append-only transitions table.** The store is disposable by design (ADR-0131 d6; the deleted-database test), so
   old stores rebuild from git."* The `answered` column can only be written by an `UPDATE`, which AC-364 and N6 both
   forbid, and `UNIQUE` on `token` forbids a second row for one token. So:
   - **The migration.** `rk-store` gains `migrations/0002_gate_transitions.sql` and its `.keys` file. That is the
     schema lint's form: *"each migration is a pair"*, and `schema_lint.rs`'s list of shipped migrations gains the
     pair. It creates **`t1_gate_transition`**, one row per transition: `token`, `state` (`opened`, `answered`,
     `expired` or `voided`), `tree_hash`, `deadline`, `kind`, `reach`, the answered form's option where §8e.7's record
     puts it in git, the opening run's `stage` and `run_id` on the `opened` row (§8e.7), and `anchor_sha`. It carries
     `UNIQUE (token, state)`, and it has no column a later write could update. Its `.keys` declares
     `t1_gate_transition.tree_hash = tree` and `t1_gate_transition.anchor_sha = sha`
     (AC-166). It **drops `t1_gate`**, whose rows are 0 in every store (§8e.0 items 2 and 4). A table named as tier 1
     that nothing writes is exactly what ADR-0158 found misleading. It also adds the interim `item` column beside the
     payload (§8e.10). `SCHEMA_VERSION` becomes **2**.
   - **A gate's state is the fold over its transition rows.** Each row is written by §8e.3's boundary function, beside
     its state-bearing `Gate:` trailer (§8e.7). A fresh token per re-reach (ADR-0105 clause 10) keeps
     `(token, state)` unique.
   - **Old stores are rebuilt, never migrated in place (AC-180).** The identity check still refuses a version-1 store
     with *"A migration from {} to {} is required; there is no implicit in-place migration"*. The explicit migration is
     `doctor --rebuild`, which re-derives a version-2 store from git into a staging file (§8e.9's `open_at`) **without
     first opening the old one through the identity check**, and then swaps it in. The store is disposable (ADR-0131
     decision 6: *"the store stays disposable under the deleted-database test"*), so what the rebuild drops is tier 2
     by definition, and the record lists it. The test is
     `doctor::a_version_one_store_is_refused_by_name_and_rebuild_re_derives_it_at_version_two`. `rk-store`'s AC-180
     test, `store::an_older_schema_is_refused_and_no_migration_happens`, is written against `SCHEMA_VERSION - 1`, so it
     moves to version 1 with no edit. **The register's own instrument did not exist, and this stage builds it.**
     AC-180's Verified-by is *"a checked-in fixture store at version N-1; an assertion that the file's bytes are
     unchanged after the refused open"*, and every test store on the tree is created at open (§8e.0 item 2).
     `store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged` reads
     `crates/rk-store/tests/fixtures/store-v1.sqlite3`, a version-1 store minted on the entry tree before `0002` lands,
     holding a few `t2_event` rows and zero `t1_*` rows, and pinned `binary` in `.gitattributes`. The test copies it,
     opens the copy, asserts the refusal names the migration, and compares the copy's sha256 with the blob's.
   - **The shipped store is rebuilt in the migration's commit.** At `c5d19d7` it stamps version 1 and holds six
     `t2_event` rows and no tier-1 row (§8e.0 item 2). The operator backs it up, runs `doctor --rebuild`, and records
     the before and after counts in the commit body. Checkpoint 7i reads the shipped store through `redkiln cost` and a
     `mode=ro` probe, so the rebuild makes 7i provisional, and Checkpoint 8e re-runs it (§8e.13).
   - **The record states that `answered` is never written again**, because the column no longer exists. Design §5
     carries no `t1_*` column list, so the note is the record's and the migration header's.

*(dated 2026-09-28, s8e build, package S8E-P3)* **§8e.2 built, uncommitted, at `766854c`; its record is
ADR-0181 (proposed).** What landed, and the choices the section left to the record, each stated there:
- **The port.** `EventStore::append_tier1(&mut self, event: Event, anchor: &TierOneAnchor)` is the fifth method
  (`crates/rk-ports/src/event_store.rs`); `TierOneAnchor` carries the trailer commit's sha, its tree and, for a gate
  opening only, the opening run's `stage` and `run_id` (`with_opening_run`). `event_store_port::ac364_…` asserts
  **five** and names `append_tier1`. The five test doubles under `crates/*/tests` that implement the port gained the
  method as a plain `append` (stated in each).
- **The SQL.** `rk-store` gains `tier_one` (`insert_stage_transition`, `insert_gate_transition`, `insert_slice_seal`,
  and `append`, the one-transaction body), `events` (the `t2_event` insert and the selector's `SELECT`, now the one
  copy of each) and `migrate` (the runner and the one opening transaction). `SqliteStore::append`,
  `append_tier1` and `query` call them; the adapter names no `t1_` table. The kind → table mapping is ADR-0181
  decision 3's closed table: a `stage_transition` without `from` (a lane entry) writes the anchored `t2_event` row only
  (friction 2); `gate_answer_void` maps to `voided` although design §5 still lists it tier 2 (N6; §8e.7 moves the row);
  a missing `NOT NULL` member refuses as `StoreError::TierOne` and the transaction leaves zero rows.
- **R2's migration.** `migrations/0002_gate_transitions.{sql,keys}`, listed by `schema_lint::all()`:
  `t1_gate_transition` (`UNIQUE (token, state)`, `state` by `CHECK`, `option_id` for the answered form's option,
  nullable descriptive columns — the answered, expired and void payloads do not carry every member), `DROP TABLE
  t1_gate`, `ALTER TABLE t2_event ADD COLUMN item` plus an index; `SCHEMA_VERSION` = **2**. **A choice the section did
  not foresee, taken in the record (decision 6):** `0002` adds a column and cannot re-run, while the opener re-ran
  every migration on every open, so `rk_store::migrate::apply` counts applied migrations in the store's own
  `PRAGMA user_version` and runs each once. `doctor --rebuild`'s staging store goes through the same runner
  (`crates/rk-cli/src/doctor/rebuild.rs`, a two-line dated edit; its INSERT, §8e.9's to retire, is unchanged).
- **AC-180.** `store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged` opens a copy of the
  P0 fixture through `rk_store::migrate::open` against the fixture's own root, reads the refusal *"A migration from 1
  to 2 is required"*, and compares the copy's sha256 with `603d6004…`. `store::an_older_schema_is_refused_and_no_migration_happens`
  moved to version 1 with no edit and is green.
- **Red, then green** (each run by module-qualified name with `--exact` through `nd.sh`; counts `1 passed`/`1 failed`
  read per run). Compile-stage reds on the entry tree: `rk-store` E0433 *"cannot find `tier_one` in `rk_store`"* and
  *"cannot find `migrate`"*; `rk-ports` E0432 *"unresolved import `rk_ports::event_store::TierOneAnchor`"* and E0599
  *"no method named `append_tier1`"* / *"`with_item`"*; `rk-adapter-sqlite` the same plus E0432 on the `TIER_ONE_*`
  constants; `rk-kernel` E0432 *"unresolved import `rk_types::event_kind::ItemKey`"*. Behavioural reds against a
  skeleton (the port method appending as `append`, the selector ignored, `SCHEMA_VERSION` 1, no `0002`):
  `event_store_port::ac364_…` `left: 4 right: 5`; `store::t1_gate_transition_…` *"no such table:
  t1_gate_transition"*; `store::migration_0002_…` *"R2's migration stamps schema version 2 / left: 1 right: 2"*;
  `store::a_checked_in_version_one_store_…` *"the version-1 fixture OPENED under schema version 1"*;
  `fake_behaviour::vec_store_append_tier1_…` BySha returned two rows; `fake_behaviour::vec_store_query_by_item_…`
  *"ByItem(item-a) … never item-b's"* with item-b's row present; `conformance::sqlite_store_conforms_including_the_tier_one_arm`
  the arm's BySha returned the unanchored core row; `store::append_tier1_…` *"no such column: item"*;
  `store::query_by_item_…` every row returned, keys dropped. `store::tier_one_statements_…` was red only at compile
  (*"no statement exists"*). All twelve named tests, the AC-160 pin `tier_ui::a_state_field_without_a_tier_is_a_compile_error`
  and `store::an_older_schema_…` are green.
- **Mutants** (`scratchpad/s8e/s8e_p3_mutants.py`: anchor asserted once, tmp + `os.replace`, restored in `finally`,
  sha256 verified): **S8E-M4** (store half; the `t1_stage_transition` insert dropped) reddens
  `store::append_tier1_…` *"the t1_stage_transition row is written beside it … left: []"*; **S8E-M25** (store half; the
  SQL `WHERE` ignores the item) reddens `store::query_by_item_…` *"never item-b's"*, and its fake variant reddens
  `fake_behaviour::vec_store_query_by_item_…`; **S8E-M34** (an `UPDATE` per token before the insert) reddens
  `store::t1_gate_transition_…` at *"a second `opened` row for one token is refused (UNIQUE (token, state))"* — the
  update path accepted it; **S8E-M40** (the opening identity stamped in its own transaction before the check) reddens
  `store::a_checked_in_version_one_store_…` *"the version-1 fixture OPENED under schema version 2"*. **A variant that
  survived, recorded as unobservable:** a *no-op* re-stamp (`UPDATE rk_identity SET common_dir = common_dir`) committed
  before the refusal leaves the main file's bytes identical in WAL mode — the checkpoint copies back an identical page
  and the header change counter does not move — so AC-180's bytes criterion holds under it by construction. M4's
  `doctor::ac184_…` half and M25's `lane_e2e` half are §8e.9's and §8e.10's kernel half's, and are owed there.
- **The shipped store is NOT rebuilt (deferred operator gate).** It still stamps version 1 (sha256 `e83d295e…`,
  unchanged across this package), so every `SqliteStore::open` over this repository now refuses it by name, and
  Checkpoint 8e (7b)'s last line stays red until the operator runs the rebuild. Prepared, not run:
  `b=target/store-backups/s8e-migration-$(git rev-parse --short HEAD); mkdir -p "$b" && cp .git/redkiln/store.sqlite3 .git/redkiln/store.sqlite3-wal .git/redkiln/store.sqlite3-shm .git/redkiln/rebuilt "$b"/ && sha256sum "$b"/*`,
  then the before counts (`python -c` over `file:.git/redkiln/store.sqlite3?mode=ro`: `schema_version`, and `count(*)`
  per table and per `t2_event` kind; at `766854c`: version 1, `t2_event` 11, `t1_*` 0), then the go-ahead line
  **`! bash -c '. runbook/tools/nodispatch.sh && nd_setup "$PWD/target/nd-operator" && nodispatch target/debug/redkiln doctor --rebuild; nd_check operator'`**
  (the operator's `!` line, from the repository root, after `cargo build -p rk-cli` through the same wrapper), then the after counts, which must read version 2; both sets go in the
  migration commit's body. Until §8e.9 lands, that rebuild writes the anchored `t2_event` rows only (no `t1_*` row and
  no interim key), so running it after §8e.9 re-derives more; the section's *"in the migration's commit"* is the
  orchestrator's to schedule.
- *(dated 2026-09-28, `s8e` build, the operator-approved rebuild (Weigh-In `wi-ef7b00`, run by the orchestrator from the
  main checkout at `7bec7cd`).)* **The shipped store is rebuilt and stamps version 2.** Backup (store, `-wal`, `-shm`,
  `rebuilt`) at `target/store-backups/s8e-migration-7bec7cd/`. Run exactly as prepared above: `nodispatch target/debug/redkiln
  doctor --rebuild` → rc 0, *"rebuilt 1 tier-1 row(s) from git at 7bec7cd…"*, *"rebuilt-store-identity ok
  root_commit=cc5a2589… schema_version=2"*; `nd_check` clean (trap empty). **Before:** schema version 1, user_version 0;
  `t1_gate` 0, `t1_slice_seal` 0, `t1_stage_transition` 0; `t2_event` 11 (`reconcile_started` 7, `stage_transition` 2,
  `dag_assembled` 1, `gate_opened` 1), none anchored; sha256 `e83d295e…`. **After:** schema version 2, user_version 2;
  `t1_gate_transition` 0, `t1_slice_seal` 0, `t1_stage_transition` 0; `t2_event` 1 (`stage_transition`, anchored and keyed)
  — the legacy fixture's one stage-less `Story:` mark, a lane entry, which ADR-0181 decision 3 maps to the anchored
  `t2_event` row only; the ten tier-2 rows are dropped, as §8e.13 item 13 records; sha256 `09020ee9…`. The rebuild's
  *"tier-1 row(s)"* wording counts that anchored entry row (a minor, named). `rk-run/rk-fix-doctor-test-count` still at
  `73c087a` with its worktree.

### 8e.3 Git-first, owned by the kernel (D21-42 8a; D21-43 N6, N7 and N10's seam)

**The decision:** *"The kernel owns it, through the journalled `GitOps` seam. It commits (zero writes on replay),
then appends the t1 and t2 rows with `anchor_sha`. `reconcile` heals a commit without a row."*

**One boundary function, in one module.** `rk-kernel` gains `runtime/tier_one.rs`. `runtime.rs` is already the
module root of a `runtime/` directory, beside `activity.rs`, `advisory.rs`, `ledger.rs`, `marks.rs` and `step.rs`.
Its function takes:
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
  landing record's to settle, and §8e.13 names it for that record.

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
separate test covers that branch (§8e.12).

**AC-162's instruments, all three, named by the register.** *"a fault-injection test with a git backend that fails
the commit, asserting zero store rows afterwards; a second fault-injection fixture installing a hook that rewrites the
commit message, which must have no effect on the trailer commit … plus a reachability lint that the tier-1 store
writer is callable only from the commit path."* The lint is a source lint over **code lines only**. It names the
writer in **every call form**: a method call, a path call such as `EventStore::append_tier1(…)` or
`<S as EventStore>::append_tier1(…)`, and a bare path handed on as a function value. It drops definitions
(`fn append_tier1`) and nothing else. Every remaining hit under `crates/*/src/` must be inside
`crates/rk-kernel/src/runtime/tier_one.rs`, with one exception, allow-listed with its reason: the tier-1 arm of
`crates/rk-ports/src/conformance.rs`. That arm sits behind the `conformance` feature, is called only from the two
conformance rows under `tests/`, and exercises the port rather than applying a transition. It is a floor. The two
fault injections are the criterion, as the register says in terms.

**The rebuild is exempt from the lint, and the record says so in terms.** `doctor --rebuild` writes tier-1 rows whose
commits already exist (git → store, §8e.9), so there is no commit for the row to precede. It never calls
`append_tier1`. It reaches `rk-store`'s `tier_one` statement functions through the adapter's `rebuild` (§8e.9). The
record names `rk-store`'s `tier_one` entry points, and those two paths, `SqliteStore::append_tier1` and
`SqliteStore::rebuild`, are their only callers: Checkpoint 8e asserts that no code line outside `crates/rk-store/src/`
and `crates/rk-adapter-sqlite/src/` names `rk_store::tier_one`. The gate transitions N6 moves
each get their own git-first test, because *a fixture per function is not a fixture per branch*:
`tier_one::a_gate_answer_is_committed_before_its_row`, `tier_one::a_gate_expiry_is_committed_before_its_row` and
`tier_one::a_gate_void_is_committed_before_its_row`, beside `rk-cli`'s `answer::a_failed_answer_commit_leaves_zero_rows`
over the real verb.

*(dated 2026-09-28, s8e build, package S8E-P5)* **Built, and what the build read into this section.** `crates/rk-kernel/src/runtime/tier_one.rs` holds the one boundary function, `tier_one::commit_act(git: &mut GitOps, recorder, store, target, act)`, in this section's order: authorise (live only), commit through the journalled seam (`GitOps::commit_with_trailers`, a new method in `rk-git`'s `trailer/write.rs` whose argv is `commit_with_trailers`'s byte for byte), re-read `HEAD` and `HEAD^{tree}` through the seam, then `append_tier1` for every row of the act and one `git_op` row per invocation, all anchored to the one commit. `tier_one::Boundary` owns the seam for a stage and `Runtime::with_boundary` binds it; `replay::resume_routed` gains `boundary: Option<Boundary>` and `rk-cli`'s `run` binds one per stage. Routed through it: the lane entry (`resume_routed`'s first `stage_transition`, with a stage-bearing `Story:` mark), the advance (`Runtime::step`'s `AdvanceStage` arm), the opening (`Runtime::step`'s `OpenGate` arm, the walk's `gate` step in `runtime/step.rs`, and `run`'s derived-facet gates), the answer (both rows of a void, each its own act: `answered <option>` then `voided`), and the expiry (in `run` and in `answer`). `story_trailer` and `terminal_read`'s commit are deleted; `answer`'s direct commit is deleted. The `closed` mark (§8e.6) and the seal (§8e.8) are not built here; `tier_one::Mark::Closed` and `Mark::Seal` exist for their producers. The heal runs at `run`'s entry after the `worktree_assigned` row and at `answer`'s after its ownership read. Readings the build took, each named for the adversarial pass: (a) **the unbound machinery seam** — `Runtime::new` binds no boundary, and an unbound runtime keeps the store-only tier-1 arms for the kernel suites, `replay::resume` (which passes `None`), `rk-cli`'s in-process `run_with` and the golden-log recorder `replay::record`; no production path is unbound (ADR-0185 decision 9); (b) **a failed act writes no `git_op` row** — its invocations are dropped with the error, because AC-162's *"zero store rows"* binds harder (ADR-0185 decision 6); (c) **the heal re-derives `Story:` stage marks and `Gate:` marks only**, skipping `closed` and `Slice-Verdict:` marks by name until §8e.6/§8e.8 fix their rows; (d) **`status` and `cost` read the whole store through `ByRun`** (inert, every row), because `ByItem("")` stops returning the rows the boundary now keys — ADR-0182 decision 4's owed whole-store selector, taken as a reading; (e) `tier_one::every_run_commit_resolves_through_the_worktree_resolver` reads the boundary's single `commit_with_trailers(` call site after its `authorise_tier1_write(`, which is why the replay arm shares the one call site rather than having its own. The §8e.12 table names **17** `tier_one::` tests (the package brief said 18; `sel rk-kernel 24` = 17 + 7 is consistent with 17). All 17 are green after a recorded red (the module's absence, `E0432`, then an entry-shaped boundary, store first); mutants S8E-M3, M5, M6, M12, M26 (kernel variant), M29, M43 (tier_one half), M44 and M46 are killed for their stated reasons.

### 8e.4 The run-owned worktree is read from the store (D21-42 8d; AC-529; R-63)

**The decision:** *"A `worktree_assigned` row is written at run start. Every tier-1 write is checked against it and
refused with a typed rk-git error on mismatch."*

Today the two verbs disagree, which is R-63. `answer`'s `recorded_ownership` reads the pair off the last
`reconcile_started` row (RT-47, *"ROUTED, not decided"*). `run`'s `story_trailer` and `terminal_read` **rebuild it by
convention**: `resolved.common_dir.join("redkiln").join("worktrees").join(item)` on `format!("rk-run/{item}")`.
`s8d-ii` routed that convention through `rk_git::worktree`'s one derivation, under the new root (`s8d-ii` §8d-ii.1).
A derivation is still a convention, so no reader may resolve ownership from it.

- **The kind.** `worktree_assigned { item, worktree, branch, reused, discarded }` is minted by ADR-0116's route: a
  design §5 row and the `rk-types` variant in one change, with `tier()`, `ALL`, the completeness and snapshot tests,
  and the no-catch-all arms in `fsm.rs`, `breaker.rs` and `memo.rs`. It is written at **every** run start, on a
  created tree and on a reused one. `reused` says which, and `discarded` lists every in-flight path the re-entry threw
  away. `s8d-ii`'s lifecycle function returns that list (`s8d-ii` §8d-ii.3), and until this row exists `run` prints
  it on stderr. This row is where W2's *"discarded explicitly and journalled"* is finally journalled.
  `worktree_assigned::a_reentry_records_reused_and_its_discard_list` runs two invocations over one item and dirties a
  tracked file in the run tree between them. The first row must read `reused: false` with an empty list, and the
  second `reused: true` listing that path.
- **The deleted-database question, answered in terms by the record.** If the database is deleted, `answer` finds no
  row and refuses (fail-closed) until a `run` writes one. Under W2 (`s8d-ii` §8d-ii.3) that `run` re-enters the
  registered, marked tree
  and writes the row without re-creating anything. The record states whether that is a tier-2 disposal or a
  next-action change. `worktree_assigned::answer_refuses_fail_closed_without_an_assignment_row` opens a gate, deletes
  the store and runs `answer`. It must refuse by name, write no row, make no commit, and resolve nothing from the
  convention path.
- **It carries the item, and every reader filters by it.** `recorded_ownership` filters by kind alone, and
  `SqliteStore::query` ignores its selector — `fn query(&self, _by: Query)` runs `"SELECT kind, payload FROM t2_event
  ORDER BY id"` (RT-4). With two items in one store, `answer` would take the last pair of **either** item. The target
  and the ownership would then both come from that one row, so `authorise_tier1_write`'s comparison would pass for
  the wrong run. §8e.10 makes `query` honour `ByItem`, and this reader goes through it.
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

*(dated 2026-09-28, s8e build, package S8E-P5)* **Built.** `run` writes `worktree_assigned { item, worktree, branch, reused, discarded }` (ADR-0185) at every start, after the invocation's `reconcile_started` row, from `rk_git::worktree::enter`'s `Entered` (`created == Reused`, `discarded`); the stderr lines stay as the operator's echo. The one resolver is `rk_cli::verbs::run::assigned_ownership` over `rk_kernel::runtime::tier_one::assigned_ownership`, through `ByItem`; `run` builds each stage's boundary from it, and `answer` reads it first and refuses, fail-closed and by name, when there is none. RT-47's pair is **kept** on `reconcile_started` as tier-2 context with no reader. The five `worktree_assigned::` tests and `answer::a_failed_answer_commit_leaves_zero_rows` are green after a recorded red (no row kind; `answer` reads `reconcile_started`; `answer` appends first); S8E-M9 (the resolver by convention, in `answer`), M26 (`rk-cli` variant), M41 and M42 are killed. The first test's foreign write is driven through the kernel boundary with the foreign tree as target and the store's ownership, and reads `GitError::ForeignWorktree` with all four values. `answer::ac406_…`'s assertion (3), *"no trailer commit"* for a void, is re-pointed by dated note to exactly two trailer commits (`answered`, then `voided` at the tip), by D21-43 N6.

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
**does** progress once this lands, and §8e.11's tests exist to contradict the second. Under ADR-0092 rule 1, moving
either one is a supersession, and the stage-location record takes both.

*(dated 2026-09-28, s8e build, package S8E-P4)* **Built (grammar, fold, reservation; ADR-0183, proposed).**
- `rk-git`: `ParsedTrailer::Story` gains `stage: Option<StoryStage>` (`Stage(String)` or `Closed`); `classify_story` splits on whitespace first (zero or one token: the pre-widening reading exactly; two: a stage-bearing or `closed` mark; three or more, a stage token with `/`, or a two-token value with no `project/` form: `MalformedReason::MalformedStoryValue` → `unparsable_trailer` drift). `CLOSED_STAGE_TOKEN = "closed"`. `ProjectState` gains `story_marks` (the identity set for `DuplicateStoryTrailer`) and `stages: slug → StageMark { stage, commit }`, latest wins oldest-first, a stage-less mark never sets or resets it. `ParsedTrailer::to_trailer` is the one renderer, checked by parsing its own output, so a producer refuses what does not round-trip.
- `rk-kernel`: `Pack::from_toml` refuses a stage named `closed` in `policy.stages`, `policy.stage_order` or a `[stage.closed]` table, naming it reserved.
- The order assumption is stated in ADR-0183 decision 4 and on `read_commits`' doc comment; `--topo-order` is not added (it would reorder the F12 corpus read AC-444 pins).
- Existing producers moved only as far as the value: `story_trailer` takes the advanced stage and writes `<project>/<item> <stage>` through the renderer, still once per item (the idempotence read, the seed and the mint at every boundary are §8e.3's and this section's seed bullet, not built here). `doctor --rebuild` takes `to` from a stage-bearing mark, anchored on its commit, keeps the item-record rule for a stage-less mark, and lists a `closed` mark (§8e.6 rebuilds it).
- Re-pointed, none deleted: `story_axis::adr0155_the_emitted_story_value_parses_as_a_qualified_story`, `story_axis::adr0155_a_second_identical_story_mark_is_drift_not_a_transition` and `story_axis::adr0159_an_empty_project_degrades_to_a_bare_story_and_records_no_transition` (the last forced by `story_value`'s new stage parameter: an empty project's `/<item> <stage>` is now surfaced drift, and the stage-less arm stays as the legacy fixture); `doctor::ac427_…`'s synthetic repository now carries `demo/s-1 review` and `demo/s-2 implement`, the stages its item records name.
- Red first, then green: the seven `Story:` `trailers::` rows and `lane_order::a_pack_stage_named_closed_is_refused_at_load` (each red line is in the package report). Mutants S8E-M16 and S8E-M17 each redden their row and were restored (sha256 checked).

*(dated 2026-09-28, s8e build, package S8E-P6)* **Built: the seed and the mint (ADR-0183 decisions 5, 6 and dated decision 10, proposed).**
- **The seed.** `through_step_seven` keeps its reconcile `Outcome` (~~`let _ = startup`~~). `rk_cli::verbs::run::seeded_stage` reads the item's latest mark from `State.projects[<project>].stages[<item>]`. A stage-bearing mark is `resolved.stage`. A `closed` mark sets `Resolved::lane_closed` (§8e.6). The record's `stage` is the fallback, taken only when git seeds nothing: no `[project] name`, no mark, or only the legacy stage-less mark. RT-50's refusal fires only when neither git nor the record gives a stage. `answer` resolves through the same function, so it folds at the seeded stage too.
- **The mint, idempotent per `(item, stage)`.** The read is inside the boundary function: `tier_one::stage_already_marked` (live arm, reads only) finds the commit on `rk-run/<item>` that already carries this `(project, item, stage)` mark. If there is one, the act commits nothing and writes no row (`Committed::AlreadyMarked`), and a missing row is the heal's to restore. `replay::resume_routed` also takes the entry act only for a stage its journal has not ENTERED. Before, it took the act whenever the journal held no `dag_assembled`. After a deleted store, the heal restores the entry row from the mark, and the plan's rows are appended without a second entry. ADR-0161's `stages_entered` guard is unchanged (`lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again`, re-run green).
- **What the build found and did not change:** a stage entered by an ADVANCE never gets its plan's own `dag_assembled` row. `plan_assembled` still reads the item's journal as a whole, so only the lane's first stage records one. This predates `s8e`. It is named here and not changed, because the two-items test needs only the item key (§8e.10).
- **Green after S8E-P2's recorded red:** `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane` (red: *"invocation 2 did not cross `implement -> review`"*) and `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` (red: *"PREMISE: hop 2 must cross into `review`"*). S8E-M1 (the seed from the record) reddens both, and S8E-M2 (the read restored to *"item already committed"*) reddens `lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order`, each for its stated reason (sha256 restored).

### 8e.6 Every stage the lane may enter is entered and trailered; an empty one says so; the end is marked (D21-42 7; D21-43 N9)

**The decision:** *"Every `stage_order` stage whose rigor is not `forbidden` is entered and carries its trailer. An
empty stage records `stage_empty { reason }`."*

- **The set is the pack's declared order, filtered.** It is the same filter `Pack::successor` already applies.
  ADR-0153 decision 4: it *"returns the next element after `stage` whose `rigor(..) != Some(Rigor::Forbidden)`,
  skipping forbidden entries rather than stopping at them, and `None` at the end of the order"*. For `bug-standard`
  that is ten of twelve stages:
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
  git, and it is not ADR-0157's stall. Removal of the run tree is keyed to this mark. `s8d-ii` §8d-ii.3 built the
  rule (clean, landed, marked, unlocked, and never `--force`), and this stage wires its trigger.
  `lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one` is the trigger's lane-level test.
  Two fixture items run to their `closed` mark. The first leaves a clean tree whose `HEAD` is the run-ref tip, which
  is what *landed* means at Checkpoint 8e, where the content arm is empty. That tree is gone afterwards and its
  registration with it. The second item's scripted `FakeHarness` step writes a tracked file that nothing lands, so
  its tree is dirty at the mark. That tree is kept, and `s8d-ii`'s named reason is printed. Mutants S8E-M37 and
  S8E-M38 (§8e.12) redden one arm each.
- **What `s8` gains, and what it does not.** D21-42 7 says *"s8 asserts set and order equality mechanically from the
  lane's own trailer commits"*. That amendment is `s8j`'s. This stage proves the property over a fixture lane
  (`lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order`), with no pack literal: the
  expected list is computed from the fixture pack's own `stage_order` and `stages`, followed by `closed`.

*(dated 2026-09-28, s8e build, package S8E-P6)* **Built (ADR-0183 dated decision 10; ADR-0185 decision 4 and a dated residual; all proposed).**
- **`stage_empty`.** `replay::resume_routed` records `stage_empty { stage, reason: "no_applicable_activity" }` once per `(item, stage)` when the stage's plan holds no node. On the entry `resume_routed` takes itself (the lane's first stage), the row is a second row of the entry act, anchored to its commit. On a stage entered by an advance, the row is appended beside it, unanchored: the advance's act is taken before that stage's plan is selected. ADR-0185 states this as a residual.
- **The `closed` mark and its row.** The kernel reports the lane's end (`RecordedRun::lane_ended`: `Done` with no successor supplied). `run`'s lane loop then mints `tier_one::close_act(<last stage>)` through the stage's boundary, but only after the stage's review pass has not blocked. The act is `Story: <project>/<item> closed`, with the empty content arm, anchoring one tier-1 row: `stage_transition { from: <last stage>, to: "closed" }`. `closed` is reserved, so the row names no stage. The heal re-derives it (the new `tier_one::a_closed_mark_heals_as_the_lanes_end_transition`; red with the arm disabled: *"the `closed` mark was not healed after its stage mark"*). A pack with no `stage_order` has a one-stage lane, and its end is marked too.
- **A closed item.** `run` reports *"`<item>` is closed"*, writes the invocation's one `reconcile_started` row (AC-172) and exits 0. It does not enter the run tree, commit, dispatch or write a tier-1 row.
- **The removal trigger.** When the invocation minted the mark, `run` calls `rk_git::worktree::remove_landed` AFTER it releases the run's lock (the rule removes only an unlocked tree). A kept tree's `s8d-ii` reason goes to stderr. A refused removal is named on stderr, the tree is kept, and the lane's exit stands. This last point is a reading, named for the adversarial pass.
- **Green after S8E-P2's recorded red:** `every_non_forbidden_stage_is_entered_and_trailered_in_declared_order`, `an_empty_stage_records_stage_empty_with_its_reason`, `the_lane_end_mints_a_terminal_closed_mark`, `a_closed_item_reruns_as_done_and_mints_nothing`, `a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one`, `the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it`. S8E-M14, M28, M37 (clean arm) and M38 (a forced removal: dirty arm) each redden their named test for its stated reason.

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
  `Gate: tok-1 {sha} 2026-09-13T00:00:00Z` is re-pointed in the same commit. `t1_gate_transition.deadline` is
  `TEXT` and holds the decimal.
- **`terminal_read` stops committing and reads the fold.** Its scan (`e.kind == …GateOpened &&
  e.payload.get("token").is_some_and(|t| !t.is_null())`) has *"no answered/void/expired filter"* (ADR-0161's recorded
  fourth finding). It re-commits a `Gate:` for a dead token and prints that the gate is open. After §8e.3 the commit
  belongs to the kernel's `OpenGate` act, so `terminal_read` only reports. It reports the token `fsm::awaiting_gate()`
  returns over the recorded journal, which excludes answered, void and expired tokens. A re-run after an answer
  therefore mints no second opening and prints no open gate.
- **The rebuild now writes gates.** `events_from_trailers`'s `Gate:` arm stops listing every mark unresolvable. It
  emits every transition, the opening included, into `t1_gate_transition` as its anchored row, from the state tokens. That
  discharges `s7d` §7d.18's RAISED widening item.
- **The opening carries the run that opened it** (final consistency pass 2026-09-24; `s8f` §8f.2 item 14). The
  `opened` row of `t1_gate_transition` carries the opening stage run's `stage` and its `run_id`
  (`format!("{}-{}", stage, &digest[..16])`, ADR-0152). The kernel's `OpenGate` act holds both and writes them.
  They are columns of the row, not members of `gate_opened`'s payload, so no canonical event moves: a committed
  fixture log under `crates/rk-kernel/tests/fixtures/logs/` holds a `gate_opened` row [measured]. The later
  transitions leave both null. `answer` reads the pair from its token's `opened` row, and `s8f` puts it on
  `answer`'s `artifact.write` (`s8f` §8f.2 item 14). The `Gate:` value does not carry the run, so an `opened` row that
  `doctor --rebuild` re-derives has a null `run_id`. **The operator accepted that residual** (D21-48 X11: *"Accepted
  as a stated residual. RK-D-UNREAD reads null as an unknown run, never as a match. The `Gate:` grammar does not
  widen."*). The gate record states it and cites X11. It touches `RK-D-UNREAD`'s tier-2 bookkeeping, not the next
  action the deleted-database test protects. The rebuild never invents a value, and `Gate:` gains no run token.
  `lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer` asserts the pair, and mutant
  S8E-M47 writes a null `run_id`. `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` asserts the
  residual's shape: every `opened` row the rebuild re-derives has a null `run_id`, and mutant S8E-M48 fills one in.
  How `RK-D-UNREAD` reads that null is `s8f`'s to pin, where it widens `run_of`:
  `doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match` (`s8f` §8f.5). *(D21-48 (2026-09-24).)*
- **Every new gate state needs a `gate_expiry` row only where it opens a gate.** The table is `s8h`'s (D21-42 21a).
  This stage mints no new gate **kind**, so it adds no row. `s8f`'s `drift` and `landing` kinds are `s8f`'s to hand on.
  So is the stage advance's refusal on an unanswered `render.structured-drift` gate (`s8f` §8f.5): `s8f` specifies
  and tests it on the advance this stage makes git-first.

*(dated 2026-09-28, s8e build, package S8E-P4)* **Built (grammar and fold; ADR-0184, proposed).** Each record-owned choice this section left free is stated in ADR-0184:
- **Whether `answered` carries the option: YES**, by the deleted-database test — `gate::answered_facet` binds an answered `unknown_facet` gate's verdict as the facet value, and `runtime::step`'s `gate_answer_at` routes a gate step on it. The answered form is `… answered <option>`, so **the state is the SIXTH token, not the last**.
- **An extension this section did not raise, taken by the same test and named for the adversarial pass (and for the operator):** an `expired` mark carries its terminal action — `expired fail`, `expired escalate` or `expired auto-decide <verdict>` — because `fail` halts and `auto-decide` answers. `opened` and `voided` carry nothing. So a value has six to eight tokens.
- **ADR-0109 decision 2's parenthetical is read as illustrative**; the record does not supersede it and states the fallback (supersede the parenthetical in part) if the adversarial pass judges it enumerative.
- **Token grammar:** tree 40-hex; deadline unsigned decimal unix seconds (an RFC 3339, signed or fractional deadline is `MalformedGateValue` → `unparsable_trailer` drift); kind and option without whitespace; reach a positive integer. A value whose token count is not 6-8 — the old triple included — is `LegacyGateValue` legacy drift.
- `reconcile`'s `GateRecord` carries `deadline: i64`, `kind`, `reach`, the latest `state` and `commit`, and a `history` of every transition (oldest first) for the rebuild.
- `EventKind::trailer_axis()` maps `GateExpired` **and** `GateAnswerVoid` to `Gate`; `GateAnswerVoid`'s tier is `One`. `rk-types`' `event_kinds::exactly_five_kinds_are_tier_one` (now six) and `event_kinds::the_trailer_axes_belong_to_the_tier_one_kinds` (the `GateExpired` exception removed) are re-pointed, red then green. Design §5's `gate_answer_void` row moves to tier 1 with the kernel at entry as producer, and the `gate_expired` row names its trailer, by dated line.
- `rk-git`'s `grammar::a_gate_trailer_carries_the_triple`, which fed `Gate: tok-1 {sha} 2026-09-13T00:00:00Z`, is re-pointed to the widened value with an integer deadline (name kept).
- The existing producers write the widened value through `ParsedTrailer::to_trailer`: `terminal_read` (`… <kind> <reach> opened`, from the row's `kind` and `reach`) and `answer` (`… answered <option>`, from the gate record). Nothing else in them moved; §8e.3 deletes both writes.
- `lane_e2e` (S8E-P2's, still red) is re-pointed in three places for the sixth-token state: `m.len() == 6 && m[5] == state` → `m.len() >= 6`, `m.len() != 6` → `m.len() < 6`, and `m.last() == Some(&state)` → `m.get(5) == Some(&state)`.
- Red first, then green: the four `Gate:` `trailers::` rows. Mutant S8E-M13's `rk-git` half, run in two variants (the renderer drops kind and reach; the parser drops them), reddens `trailers::the_widened_gate_value_carries_kind_reach_and_state` both times; its `doctor::` half is §8e.9's.
- **AC-444's F12 replay, measured under both grammars** (`cargo test -p rk-git --features corpus`, `CORPUS_PIN` `61f590e5`, entry grammar in a detached worktree at `d80609b`): every asserted property holds (74 slices, 15 ever rejected, legacy `Gate:` drift > 0, no coerced gate); bare story ids 31 → 22, legacy `Gate:` drift 15 → 14, total drift 45 → 54. The register note records it.

*(dated 2026-09-28, s8e build, package S8E-P6)* **Built: the runtime half.**
- **`terminal_read` only reports, and only the awaited gate.** It folds the recorded journal and prints the token `fsm::awaiting_gate()` returns. That token is never an answered, void or expired one. ~~The latest tokened `gate_opened` row, unfiltered.~~ The commit went with §8e.3.
- **Every transition is git-first.** The opening (`OpenGate`, carrying the opening run's `stage` and `run_id`), the answer, the void and the expiry go through the boundary (§8e.3, S8E-P5). This package added nothing there beyond the seed that `answer` now resolves through.
- **The void, and the reading taken.** §8e.3 says reconcile *"keeps observing the void"* and the entry path mints it. On this tree, the void is decided by the kernel (`gate::answer`, comparing the gate's bound tree with the tree at the answer) at `answer`'s entry, which reconciled. It is committed git-first as `answered <option>` then `voided` (S8E-P5). `reconcile` observes it as the token's `voided` state, and the heal restores its rows. No path produces a void that git has observed and the store has not, other than the heal's case. So no separate entry-path mint is built, and `rk-git`'s reconcile computes no tree comparison. This is a reading, named for the adversarial pass and for the operator.
- **Green after S8E-P2's recorded red:** `lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer`, `lane_e2e::terminal_read_reports_only_an_awaited_gate` (its answered arm was red on *"printed the dead token … as open"*), `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered` and `lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child`. S8E-M10 (the old filter) reddens the terminal-read test, S8E-M27 (the answer's mark written `opened`) the deleted-store test, and S8E-M47 (the `opened` row's `run_id` null) the first test.

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
  recorded terminal state is `Sealed`. §8e.0 item 8 measures, at the stage's open, whether any reader (the replay
  comparison, and every caller of `RunState`'s JSON) misreads a lane that continued past review. If one does, the
  slice record says how the fold scopes a seal.
- **The ARMED lint is un-ignored in the commit whose real fixture run reaches the producer**, on ADR-0162's
  precedent that a real `redkiln run` over a fixture corpus reaches a producer. The runs are the binary:
  - `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` runs over `exit/findings-block/`, which
    exits 4, and reads `Slice-Verdict:` off `rk-run/<item>` with git's own parser;
  - `slice_verdict::approved_is_sealed_on_the_advance_path` runs over a new `slice/approved/` corpus, which exits 0.

  The `#[ignore = "ARMED: …"]` line is deleted in that commit. No erratum is owed for it, because it is source and
  not a record. **The lint's producer scope drops `crates/rk-cli/src/doctor/` in the same commit.** `doctor/rebuild.rs`
  is the READER direction: it re-derives rows from trailers, and once its `Gate:` and `Slice-Verdict:` arms name
  `TrailerKey::` variants it could satisfy the lint with no producer anywhere. That is the reason the lint already
  excludes `rk-git/src/trailer/`, and its doc says so for both. Mutant S8E-M45 deletes the seal while `doctor/`
  names `TrailerKey::SliceVerdict`, and the lint must stay red. **Checkpoint 8a guard (7) is amended in the same
  commit** (§8e.13), because it asserts the lint is ignored and ARMED. ADR-0162 decision 2 (*"The `Slice-Verdict:`
  axis is NOT touched, and the checkpoint enforces that"*) was scoped to that record's work. The slice record cites it
  and re-states Checkpoint 7h guards (6) and (7) and Checkpoint 8a guard (7), as §8e.13 spells out.
- 🛑 **No document claims the review gate is guarded against false approval** (ADR-0101 clause 5; S-SUBTLE-CB is
  UNRUN). A seal records a verdict. It says nothing about the verdict's calibration.

*(dated 2026-09-28, s8e build, package S8E-P7; ADR-0186, proposed.)* **Built as written, with four readings for the
adversarial pass:**
- **The siting.** The refusals are at the mint, in `rk_kernel::runtime::tier_one::seal_act`, before any git call;
  `rk-cli`'s `review_pass` calls it on both outcomes and hands the act to a SIBLING of the stage's boundary
  (`Boundary::sibling`), because `resume` has already taken the stage's own. A refused seal is exit 3, named on stderr
  (*"the item `review-1` declares no `slice`"*, *"… which is the item id"*), and commits nothing.
- **D21-93's scope, built.** `slice_sealed`'s payload is `{ project, slice, stage, verdict }`; the fold records the
  stage (`RunState::sealed_stage`, not serialised) and `terminal()` reads `Sealed` only while the fold is in it.
  `fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition` (rk-kernel) is the test; mutant **S8E-M50** (the scope dropped) is its red. A healed or rebuilt seal
  carries no `stage` and is scoped to the stage it folds in: ADR-0186 decision 4 states the residual.
- **The lint's spelling moved, so the lint is re-pointed as well as un-ignored.** After §8e.3, `TrailerKey::` appears on
  no code line of `rk-cli/src/` or `rk-kernel/src/` (the renderer is `rk-git`'s `ParsedTrailer::to_trailer`), and the
  old pattern failed at its own anchor with all three axes produced. The lint now counts calls of the boundary's act
  constructors — `Mark::Stage(` / `close_act(` (Story), `Act::gate(` / `Act::gate_opened(` (Gate), `seal_act(`
  (Slice-Verdict) — outside `runtime/tier_one.rs`, whose definitions are its anchor, with `rk-cli/src/doctor/` dropped.
  S8E-M45 is run in two variants (`TrailerKey::SliceVerdict`, and `seal_act(`, each named in `doctor/rebuild.rs`
  with the seal deleted); the lint stays red on both.
- **The unbound seam seals nothing**, as it mints no `closed` mark (ADR-0185's residual): `run_with` passes no
  boundary. `rk-git`'s renderer gains the `Slice-Verdict` arm, round-trip checked like the other two.

*(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; `spikes/DECISIONS-2026-09-21.md` D21-94 and D21-95; ADR-0186
as amended, proposed.)* **The refusal moves to the lane's entry, and the mint keeps it as the backstop.** D21-94: *"The
refusal moves to the lane's entry, before any dispatch, for a lane that will reach a review stage"*; D21-95 item 1 reads
*"will reach a review stage"* from declared `provides`. Built:
- **One validator.** `rk_kernel::runtime::tier_one::validate_slice(declared, item, project, stage_order)` holds the five
  refusals above in the mint's order; `seal_act` calls it first, then refuses an absent `[project] name` as ADR-0159's
  `NoProject` (an absent project skips only the project comparison, so it is never an entry refusal).
- **The entry check** sits in `verbs::run::run`, after the closed-item read and before the store, the run tree, any
  commit or any harness. It walks from the current stage through `Pack::successor` (a `forbidden` current stage is
  skipped too; bounded by the order's length), and when a reached stage has a catalog activity whose `provides` first
  is `review_pass::FINDINGS_SCHEMA`, it calls the validator and refuses by name: exit 3, AC-172's one
  `reconcile_started` row, zero dispatches, nothing committed. No stage is compared to a string literal (ADR-0162's
  forbids). A lane that reaches no such stage is not refused (ADR-0159 decision 2's reason, kept for every other lane).
  An unloadable pack or catalog decides nothing here; the path that loads it refuses by name, and the mint is the
  backstop.
- **The stated gap** (D21-95 item 2): **0** of the 102 shipped activities provide `Findings` first (the shipped
  `code-review` provides `["ReviewVerdict"]`), so on the shipped packs the entry check and the seal are both inert;
  `s8g` owes the agreement by its receiver note (§8g.1).
- `slice_verdict::an_absent_slice_is_refused_by_name` and `…_equal_to_the_item_id_is_refused` now meet the entry
  refusal first and stay green unchanged; the mint's own refusal is pinned in process by
  `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop`.

### 8e.9 The rebuild moves into `rk-adapter-sqlite`, and criterion (2) is re-measured (D21-42 8c)

**The decision:** *"Moves into rk-adapter-sqlite (`open_at` / `Capability::Rebuild`, the [s7d] §7d.18 RAISED item).
rk-cli's duplicated INSERT is retired, and criterion (2) is re-measured against ADR-0105 cl.9's real projection."* (The
bracketed `[s7d]` is inserted into the quotation here: the ledger's bare reference names `s7d`'s section.)

- **The duplicated statement goes.** `rebuild.rs` carries *"🛑 THE SECOND COPY OF `SqliteStore::append`'s
  STATEMENT"*. `rk-adapter-sqlite` gains `SqliteStore::open_at(path)`, which targets the staging file, and the rebuild
  writes through it. It writes the `t1_*` rows (boundaries, every gate transition and seals, each anchored to its own
  trailer commit) and the anchored `t2_event` rows of tier-1 kinds, through `rk-store`'s `tier_one` statements. The
  git read and the item-record fallback stay in `rk-cli`, because the order *"git → store, never the reverse"* is
  the doctor's. Checkpoint 8e asserts that no `INSERT INTO` code line remains under `crates/rk-cli/src/`.
- **ADR-0158's bullet, read honestly.** It forbids *"Writing `t1_*` rows from `rk-cli`, or any SQL against them outside
  `rk-store`"*. After this stage no `rk-cli` code line names a `t1_` table, an `INSERT INTO` or `rk_store::tier_one`
  (Checkpoint 8e (6) and (7)), so the second clause holds outright. The first clause is a reading, and the operator
  took it. `doctor --rebuild` is invoked from `rk-cli`, and the rows it causes are written by the adapter's
  `rebuild`. D21-48 X10: *""From `rk-cli`" names where the write runs. The adapter's `Capability::Rebuild` owns the
  SQL, so there is no supersession. The record states the reading."* So the tier-1 record states that reading,
  cites X10, and supersedes nothing of ADR-0158. *(D21-48 (2026-09-24): the fallback supersession for `doctor
  --rebuild` alone, drafted for an adversarial pass that rejected the reading, is removed.)*
- **`Capability::Rebuild` is implemented, and the registry rows flip.** D21-42 8c names it (*"`open_at` /
  `Capability::Rebuild`"*), so the first draft's either/or is withdrawn. The port's `rebuild(&Snapshot)` takes
  `Snapshot(pub(crate) ())`, whose contents no record had decided. `rk-adapter-sqlite`'s registry row says so: *"rebuild
  takes the same undecided Snapshot (D17-9); its consumer is doctor --rebuild at 7d"*. The tier-1 record decides
  them. A `Snapshot` is the ordered list of anchored rows a store holds: each event with its `anchor_sha`, its `t1_*`
  row where its kind has one, and its interim `item` (§8e.10). `rk-ports` gives it a public constructor.
  - `SqliteStore::snapshot` reads those rows. `SqliteStore::rebuild` writes them, in one transaction, into a store
    opened with `open_at` on the staging file, through `rk-store`'s `tier_one` statements.
  - `doctor --rebuild` builds the `Snapshot` from git and calls `rebuild` through the port, so `rk-cli` names no
    statement.
  - **Both rows flip together.** `conform_event_store` treats a registry that declares *"one of snapshot/rebuild
    unsupported and not the other"* as a defect. Each row becomes `Probe::Recorded { source }`, naming its test.
    `unsupported_tokens()` then returns nothing, so the SQLite conformance row's arm is empty and the body's
    snapshot-then-rebuild round trip runs over the real store. `VecStore`'s pair follows the new `Snapshot`.
  - `registry::ac489_capability_registry_dated` pins both rows as probe-less, and it is re-pointed in the same
    commit. `registry::snapshot_and_rebuild_rows_carry_a_recorded_probe` is the new assertion, and mutant S8E-M39
    restores `Probe::None` on the `rebuild` row alone.
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

*(dated 2026-09-28, s8e build, package S8E-P7; ADR-0181 decision 11 and its dated note, proposed.)* **Built, with the
choices it took named:**
- **`open_at` is called from the root.** Checkpoint 7d's D-10 ban forbids `SqliteStore::open` — a prefix of
  `SqliteStore::open_at` — on any code line under a `doctor` module path, so `doctor/rebuild.rs` calls
  `crate::adapters::open_rebuild_target`, which calls `SqliteStore::open_at(<staging>, identity)`. The identity is
  derived from git; the repository's old store is never opened (S8E-M36 opens it first and turns the version-1 test
  red with *"A migration from 1 to 2 is required"*).
- **The rebuild's rows are the heal's.** `tier_one::marked_commits` (new, shared by the heal and the rebuild) walks each
  item's `rk-run/<item>` ref; the rebuild anchors every row to its commit and that commit's tree and hands the
  `Snapshot` to `EventStore::rebuild`. A mark no run ref reaches is listed unresolvable (AC-165). The legacy stage-less
  rule stays in `rk-cli` for an item whose ref carries no stage-bearing mark. `rebuild` is append-only (a store whose
  rows are a prefix of the snapshot's gains the rest), so the conformance round trip is identity and no `UPDATE` or
  `DELETE` exists. The conformance body's round trip now runs AFTER the tier-1 arm.
- **Re-pointed, none deleted:** `doctor::ac427_…`'s synthetic repository puts each item's marks on its own
  `rk-run/<slug>` ref and keys its pre-delete rows by the interim `item`; `doctor::ac429_…` and `ac427_…`'s
  `stage_actions` read the whole store through `ByRun` (~~`ByItem("")`~~, which misses keyed rows — S8E-P5's route for
  `support::store_rows`); `registry::ac489_capability_registry_dated` pins both rows recorded.
- **Two tests were green at their first run, recorded honestly.** `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record`
  is a pin by design (S8E-M18 reddens it). `doctor::a_version_one_store_is_refused_by_name_and_rebuild_re_derives_it_at_version_two`
  was green at its first run because S8E-P3 had already routed the staging file through `migrate::apply` with a
  git-derived identity; §8e.12's red reason (*"`SCHEMA_VERSION` is 1"*) was stale, and S8E-M36 is its red.
- **What `ac184_…`'s lane carries, and two findings it measured.** The lane (`Lane` variant, `lane-1`) writes its
  boundaries, an opening and an answer, the review's seal and the `closed` mark; the planted unreachable row falls
  outside the projection. It does NOT carry `voided` or `expired` (`rebuild_recovers_every_gate_state_from_the_widened_gate_value`
  rebuilds every state from synthetic marks), because driving a void inside the lane measured two defects, reported for
  the operator and ~~NOT repaired here~~ *((a) is repaired by package R1FIX-8e, below; (b) is `s8f`'s)*: (a) `answer` presents `open_gates().filter(..).last()` — the LAST token in token
  order — so after a void and a fresh gate it presents the spent token about half the time (*"has already been
  answered"*); (b) `run`'s gate binds the user checkout's tree (`prepare_inputs` hashes `resolved.worktree`) while
  `answer` hashes the run-owned tree, so once the run tree moves every later answer is void too.
- **`projection::rebuild_differential`'s header gains its dated line**, naming
  `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` as the half that measures the route; the kernel test
  is unchanged and pins green.

*(dated 2026-09-28, s8e build, package R1FIX-8e)* **Defect (a) is repaired here, under ADR-0157's stall** (refuting round 1's spec-conformance major: no stage or
ledger entry owned it, and it stalls any lane with a second gate — an ANSWERED gate followed by a later stage's gate, not
only a void, because neither record ever leaves `gates_open`). `answer` now presents the AWAITED gate
(`RunState::awaiting_gate`, the reader `terminal_read` already uses) and falls back to *"opened, and not expired"* only
when nothing is awaited, which is AC-407's spent presentation: `rk_cli::verbs::answer::presented_gate`. Red first:
`answer::answer_presents_the_awaited_gate_over_a_spent_token_that_sorts_higher` folds an answered G1 and a voided G1, each
above an open G2 in token order (found deterministically by walking `reach`), and was red under `.last()` (*"answer
presents the ANSWERED token 3d264e16… over the awaited 019d34cc…"*); its third arm keeps AC-407's fallback. S8E-M57
(`.last()` restored) is killed by it.
- **`lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit` now carries `voided`.** Its lane walks THROUGH A VOID
  (`lane_e2e::walk_the_lane_through_a_void`: G1 opened; a tracked file of the run tree changed, uncommitted, so the
  answer is void, AC-406; the file restored; G2 opened, answered and applied; the lane walked to `closed`), so AC-163's
  instrument reads two openings, two answers and a void. Defect (b) is not reached, because the run tree is restored
  before G2 binds.
- **`doctor::ac184_…`'s lane still carries no `voided`, because walking it through the same void MEASURED a third
  defect (c), reported and ~~NOT repaired here~~:** *(repaired by package D2196-8e under D21-96, below)* the void answer's live `answered` row records the ANSWER's tree hash
  (`gate_answered`'s payload `tree_hash`, the moved tree), while its `Gate: … answered` mark carries the gate's BOUND
  tree (ADR-0184 decision 1: *"the gate's bound tree (ADR-0105 clause 4), the same on every mark for the token"*), so the
  rebuilt row's `tree_hash` differs and the differential fails (*"only recorded: …|answered|70566bf3…"*, *"only rebuilt:
  …|answered|e9ead599…"*). ADR-0181 decision 11 says the rebuilt row is *"the row the live act anchored to that commit"*;
  over a void it is not. Which tree an `answered` transition row carries is a record's choice (ADR-0181, ADR-0184), and
  it is put to the operator as this package's blocker; the ac184 lane is unchanged until then. *(dated 2026-09-28, s8e build, package D2196-8e)* ~~the ac184 lane is unchanged until then~~ — ruled by D21-96 and built; the lane now walks through the void (the note below).
- **`expired` is not reachable inside the ac184 lane, recorded rather than built.** `pack.lane.toml` declares
  `gate_deadline_secs = 900` and one `[[gate_expiry]]` row, `action = "fail"`, for its one gate class, so an in-lane
  expiry needs a variant whose deadline races every answer in the lane against a 1 s clock, and a `fail` expiry does not
  walk on to the `closed` mark the differential ends with. `every_tier_one_row_…`'s expiry arm (`pack.gate-1s.toml`) and
  `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` carry `expired`.

*(dated 2026-09-28, s8e build, package D2196-8e)* **Defect (c) is repaired, as D21-96 rules it** (*"The live `answered` row of `t1_gate_transition` carries the
gate's **bound** tree, taken from the gate record, so the tier-1 row equals what its mark carries and a rebuild
reproduces it byte for byte. The answer's observed tree is not lost: it stays in the anchored tier-2
`gate_answer_void` row."*; ADR-0181 decision 3's dated note, `proposed`). *(dated 2026-09-28, s8e build, package
R2FIX-8e: the ruling is quoted verbatim; its "tier-2" names the `t2_event` table the anchored row lives in. The kind
is tier 1, ADR-0184's *"`gate_answer_void` moves to tier 1"*; nothing is reclassified.)*
- **The writer.** `rk_store::tier_one::append` takes an `answered` row's `tree_hash` from its token's `opened` row in
  `t1_gate_transition` — the gate record the store holds, whose tree is the gate's bound tree (ADR-0184 decision 1:
  *"the gate's bound tree (ADR-0105 clause 4), the same on every mark for the token"*) — and no longer from the
  `gate_answered` payload's `tree_hash`, which is the answer's observed tree. The payload is unchanged (no canonical
  event moves). The live act, the heal and the rebuild all write through this one function, so the three agree by
  construction; for an applied answer the two trees are equal and nothing moves. **A reading this siting takes, stated
  in the record:** an `answered` row for a token the store never opened is refused as `StoreError::TierOne` and the
  transaction leaves zero rows, rather than falling back to the payload's tree (decision 3: *"A tier-1 row never
  invents a member"*). No live path reaches it: the kernel's `Act::gate` needs the fold's gate record, an opening is
  git-first and its row precedes the answer's, and the heal and the rebuild walk the marks in ref order.
- **The lane.** `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` now runs
  `lane_e2e::walk_the_lane_through_a_void` (two openings, two answers, one `voided`, the seal and `closed`). RED at
  this package's entry for exactly the reported reason (*"only recorded: `…|answered|6aaf314e…`"*, *"only rebuilt:
  `…|answered|6ae68e31…`"*), green after the writer change. Before the store is deleted it also asserts D21-96's
  second sentence on the LIVE store: the anchored `gate_answer_void` row carries `observed_tree_hash`, distinct from
  its `bound_tree_hash`. A rebuilt void row carries the bound tree only (the `Gate:` value holds no observed tree), so
  the observed tree is a fact of the live store and never of the AC-184 projection, which compares `t1_*` rows.
- **The refusal branch has its own fixture:** `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` (red against the pre-D21-96 writer: *"left: 0000…"*, *"right:
  bbbb…"*, the payload's observed tree read). S8E-M58 (the observed tree restored) is killed
  by both tests; S8E-M59 (the payload's tree taken when no `opened` row exists) by the `rk-store` test alone, since no
  lane reaches an answer without an opening.
- **What is NOT done here:** `expired` is still not in the ac184 lane (the bullet above), and defect (b) is `s8f`'s.

### 8e.10 The interim `item` key, outside the canonical digest, and `query` honours `ByItem` (D21-43 N5; D21-46 R3)

**The decisions.** N5: *"An interim `item` key on every item-scoped event kind, moving to ADR-0131's tag later as
ADR-0163's interim key does; `query` honours `ByItem`. Supersedes ADR-0131's first forbids bullet in part, for those
members."* R3 then fixes where the key lives: *"**Carried outside the canonical digest**, as ADR-0131 places its tag
set. Digests and golden logs are unchanged, and `query` still uses the key."* Together they settle the first draft's
finding 7. `stage_transition_event` carried no item, `SqliteStore::query` ignored its selector, and a second item's
`run` in the same store would fold the first item's transitions. `stages_entered` could then refuse its advance. `s8`
runs a fix item **and** a story item in one repository.

- **Where the key lives: beside the event, not in it.** ADR-0131 decision 2 is the placement R3 cites: *"Tags are
  carried on the event and are OUTSIDE `canonical_events`. `canonical_events` stays `{kind, payload}`, byte for
  byte."* So the key is a member carried beside the event, outside `payload`, `canonical_events` and `event_json`. The
  store persists it in the `t2_event.item` column that §8e.2's `0002` migration adds, as it persists `anchor_sha`. The
  record fixes the Rust carrier. It honours ADR-0131's second forbids bullet (*"Forbids `canonical_events` or
  `event_json` gaining any member beyond `kind` and `payload`"*). Because the key is not in the payload, it does not
  move the first bullet either (§8e.13, finding 12).
- **The members.** The record enumerates every item-scoped kind and sets the key on each. A kind it judges not
  item-scoped is listed by name with its reason. `fsm::every_item_scoped_kind_carries_the_interim_item_key` iterates
  `EventKind::ALL` against that list, so a kind added later without a decision fails it. `dispatch_completed` is one of
  these kinds. `s8f` cites this record for it and keeps `activity_id_at_version` and the breakdown (D21-42 12b).
  `rate_limit_reading.item` stays in the payload where ADR-0163 put it, and the record says whether that kind carries
  the key beside it as well.
- **`query(ByItem(x))` returns only rows whose `item` is `x`**, in `rk-store`'s SQL over the `item` column, and in
  `VecStore`. The conformance arm asserts it for both rows. Every reader that folds one item reads through it: the
  kernel's fold input, `answer`'s ownership read (§8e.4), and `explain`. `BySha` and `ByRun` stay as they are, and the
  record says whether they are still inert.
- **What an item-less row does under `ByItem`.** A row written before this stage carries no `item`. The record fixes
  whether `ByItem` excludes it or reads it as the one item the store has ever held, and says which. This file
  recommends exclusion plus a named `doctor` finding, because a guess about ownership is the defect N5 exists to end.
- **The golden logs do not move (R3), and a test says so.** With the key outside `canonical_events`, no golden log's
  canonical events change, `CORPUS_PIN` does not move, and `run_id` is untouched, which is ADR-0131 decision 2's own
  consequence list. `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` pins it in two arms.
  The first: every committed golden log's canonical digest equals the value §8e.0 item 3 recorded. The second: one
  event with the key and the same event without it have the same canonical digest. Mutant S8E-M35, which writes the
  key into `payload`, must redden the second arm.
- **The cost, stated as ADR-0163 stated it.** ADR-0131's decision driver *"Waiting accretes nothing"* is broken for
  these members in one respect: a carrier exists before ADR-0131's tag wiring does. It moves to ADR-0131's reserved
  `item` tag when that wiring lands, and the column's meaning does not change. The golden-log cost that §8e.0 used to
  STOP on does not arise.
- **ADR-0131 decision 8 and its forbids bullet, argued rather than dismissed.** Decision 8: *"This record edits no
  crate, and the wiring lands after Checkpoint 8."* Its bullet: *"Forbids editing `rk-ports`, `rk-kernel`,
  `rk-adapter-sqlite` or any other closed crate **for this record** before Checkpoint 8 is recorded as passed"*. This
  stage edits those crates before Checkpoint 8, so the bullet binds unless the edit is not *for this record*.
  - **The argument that it is not.** The interim key executes D21-43 N5 and D21-46 R3, not ADR-0131. It widens no
    `Query` to a tag conjunction (decision 1), reserves no key vocabulary, adopts no happenstance (decision 5), and
    leaves `ByItem` meaning what decision 1 says it means: the reserved keys *"carry exactly the meaning they carry
    today"*.
  - **The point an adversarial reader will press.** The carrier does deliver, ahead of the tag, decision 4's
    substance: *"The three item-less tier-1 kinds gain an `item` tag."* It is a stand-in for decision 4, and it moves to
    the tag when the wiring lands, as ADR-0163's member does.
  - The record states the argument and names it for its adversarial pass. If the pass rejects it, the record takes
    the bullet as a supersession in part, for the interim carrier alone.
- `lane_e2e::two_items_in_one_store_do_not_fold_together` runs two items' lanes in one fixture repository and one
  store. It asserts each item's stage, gates and `stages_entered` are its own.

*(dated 2026-09-28, s8e build, package S8E-P3)* **§8e.10's store and type half built, uncommitted; its record is
ADR-0182 (proposed).** `rk_types::event::Event` gains a private `item: Option<Box<str>>` (`with_item`, `item`) —
`Box<str>`, not `String`, because a `String` pushed `verify::exit::ExitRefused` past `clippy::result_large_err`'s 128
bytes; `EventKind::item_key()` → `ItemKey::{Carried, InPayload, NotItemScoped}`, total and pinned by
`fsm::every_item_scoped_kind_carries_the_interim_item_key` (30 carried, `rate_limit_reading` in its payload, seven
not item-scoped, each with its reason). `query` honours `ByItem` in `rk_store::events::select` and in `VecStore`.
**The record-level choices, each taken in ADR-0182 and named for the operator:**
- **`ByItem` on an item-less row: included, as the interim** (decision 4) — the section *recommends* exclusion plus a
  `doctor` finding, and this package does not take it, because no producer sets the key yet, every reader goes through
  `ByItem` (`rk_kernel::runtime::journal_query`, and `cost`/`status` through `journal_query("")`), and exclusion would
  empty every one of those reads. Exclusion is a one-clause change once the producers key; the `doctor` finding is
  owed. **This departs from the section's recommendation and is put to the operator.**
- `BySha` selects by anchor (ADR-0181 decision 12; the conformance arm's port-level observation); `ByRun` stays inert.
- `rate_limit_reading` keeps ADR-0163's payload member and does not carry the key beside it (one source).
- ADR-0131 decision 8 is argued, not dismissed; a supersession in part for the carrier alone is the fallback.

**The replay test's placement (§8e.0 item 6's finding), fixed without a rename.** `crates/rk-kernel/tests/it/replay.rs`'s
module-wide `#![cfg(feature = "corpus")]` became one `#[cfg(feature = "corpus")]` on each of its 69 items (a dated note
in the file says why), so `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` runs in the
unfeatured suite under Checkpoint 8e (2)'s `sel rk-kernel 24` and (8), and nothing else in the module does:
`cargo test -p rk-kernel --test it -- --list` selects 0 names matching `golden_log_replay|ac161_ac164_delete_db_replay`
unfeatured (Checkpoint 7b's differential) and 1 `replay::` name; with `--features corpus` it lists 14 and the test's
corpus arm (the `CORPUS_PIN` copy is the committed copy's bytes at `CORPUS_SHA`) passes. Arm 1 pins
`73f7553d…4e90`; arm 2 is killed by **S8E-M35** (the key written into `payload`: *"arm 2: carrying the interim key
moved … left: 251601bc…"*).

*(dated 2026-09-28, s8e build, package S8E-P6)* **`lane_e2e::two_items_in_one_store_do_not_fold_together` is green** (red at S8E-P2/P5: *"the gate item's own marks … left: [implement]"*). The first item's unkeyed `dag_assembled` row read as the second item's under the interim inclusive reading, so the second item's entry was skipped. The fix is producer keying on the kernel's side: `Runtime::with_item` keys every row `Runtime::append` writes, and `replay::resume_routed` binds the run's item. The executor's own guard appends in `runtime/activity.rs` and every harness-appended row stay item-less, on ADR-0182's residual (its dated note). S8E-M25's lane half (`rk_store::events::select` ignoring `ByItem`) reddens the test for its stated reason.

### 8e.11 The first end-to-end tests of `verbs::run::run` (D21-42 24c; D21-40 decision 2)

**The seam is one field, and the code says so in advance.** `run()` always builds
`rk_adapter_claude::ClaudeHarness::new(binder.into_config(), token.clone())`, so no test can hand it a
`FakeHarness`. `Startup`'s own doc says it is shaped for this: *"The three seams a verb's outer `run` takes, as one
value — so a fourth seam is one field rather than a fourth parameter on six functions."* So `Startup` gains
`harness: &dyn HarnessFactory`. `Startup::real()` builds `ClaudeHarness` exactly as today, and a test passes a factory
over `rk_ports::fakes::FakeHarness::scripted(…)`. Every test below calls **`rk_cli::verbs::run::run(`** by that name,
so `grep -rn "verbs::run::run(" crates/*/tests` stops returning nothing. That grep is a floor, and the tests are the
assertion. Checkpoint 8e (7) runs it over code lines.

**The fixture lane.** `pack-min` gains a lane variant with a declared `stage_order` that includes:

- a `forbidden` stage;
- an empty stage;
- a stage whose activity opens a gate;
- a review stage over `review-min`.

It also gains an item with a `slice`. Its `config.toml`, and every fixture repository a lane runs in, gains `[project]
name`. The `.redkiln/worktrees/` ignore is already there for every fixture repository a run creates a worktree in,
because `s8d-ii` added it with the root (`s8d-ii` §8d-ii.1), and the new lane variant takes it too. **The `[project]
name` edit is forced, and it is the one most likely to be missed.** Entry now mints a `Story:` mark (§8e.6), and
ADR-0159 decision 2 refuses at the mint. So every existing binary fixture that runs a stage would change its exit from
its row to `3`:
AC-438's table, `cost`'s run, and `exit/findings-block/`. Checkpoints 7h (10a) and 7i (3) are the witnesses. For the
same reason, the item that `exit/findings-block/` runs gains a `slice`, or its seal refuses and exit 4 becomes 3.

**The tests, in `crates/rk-cli/tests/it/lane_e2e.rs`:**

- `lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer` — open, then `answer`, then
  re-run. Git's parser counts exactly one `opened` and one `answered` `Gate:` state for the token on `rk-run/<item>`,
  and the re-run's stdout reports no open gate. The token's `opened` row in `t1_gate_transition` carries the opening
  run's `stage` and `run_id` (§8e.7).
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
  `RK_FIXTURE_MODE=replay:<frames>`, and the fixture repository's `config.toml` naming
  `target/debug/rk-fixture-child-claude` as `[adapters] harness_binary` (D21-47's canonical fixture harness): `run`
  exits 0 at a gate, then `redkiln answer`, then `redkiln run` crosses the next boundary. This is the one arm where
  "process" means a process.
- `lane_e2e::the_lane_end_mints_a_terminal_closed_mark` (N9).
- `lane_e2e::a_closed_item_reruns_as_done_and_mints_nothing` (N9).
- `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered` (N6; AC-161).
- `lane_e2e::two_items_in_one_store_do_not_fold_together` (N5).
- `lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it` (D21-44; AC-518's instrument:
  `git status --porcelain` empty, `git diff-index --quiet HEAD` clean, `HEAD` unmoved), over `s8d-ii`'s root and
  ignore rows. It is the first full-lane run of that instrument, and `s8d-ii` §8d-ii.10 claims nothing ahead of it.
- `lane_e2e::a_second_invocation_reuses_the_run_worktree` (W2 on the run path). `s8d-ii` §8d-ii.3 built the reuse,
  and this is its first lane-level run. The tree's inode-level identity is not observable portably, so the test plants
  an **ignored** file after invocation 1 and asserts it is still there after invocation 2, which a re-creation would
  delete.
- `lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one` (N9's removal trigger, §8e.6).
- `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit` (AC-163's replay instrument, over the fixture lane).
  After a lane that writes every tier-1 kind, three things hold:
  - each `t1_*` row's `anchor_sha` names a commit on `rk-run/<item>` whose trailer matches the row's kind and value;
  - no two rows of distinct transitions share a commit;
  - every tier-1 trailer commit on the ref anchors a row.

  An adversarial arm plants a `t1_stage_transition` row whose anchor names no commit, and the assertion must fail on
  it. That is the register's *"which fails on a store row with no matching commit"*.

*(dated 2026-09-27, `s8d` build, package F3-D2174; receiver note; `spikes/DECISIONS-2026-09-21.md` D21-74 ruling 2;
R2 = ADR-0173 decision 9 as amended, proposed.)* *(dated 2026-09-27, `s8d` close: accepted on D21-80)* **Owed here: the run path's run-branch binding.** `verbs::run::run`
binds `change_deriver::RunBranchDiff` into the routing with `ToolRouting::with_run_branch`, and `"affected"` inside
`Changeset`'s own producer (`tdd-red-green-refactor`'s `green-verify`) reads it. `s8d` proves the reader
(`change_deriver::the_run_branch_reader_reads_the_items_changeset_paths_through_rk_git`) and the kernel's use of it
(`test_selection::inside_the_changeset_producer_affected_resolves_over_the_run_branch_diff`, and the two `lane_walk`
tests over a git fixture). It could not prove the call site: `run()` builds `ClaudeHarness` with no seam, every path
to a `toolchain:test` step passes an agent step first, and no live dispatch was allowed. So mutant MF3j (the binding
removed) SURVIVED all 140 `rk-cli` tests (`s8d` §8d.10's F3-D2174 note). Once this section's `HarnessFactory` seam
exists, one `lane_e2e` test whose fixture lane reaches a `selection = "affected"` step inside `Changeset`'s producer
kills MF3j: the step must reach the port with the run branch's paths, and must not refuse *"no run-branch reader is
bound"*.
*(dated 2026-09-27, `s8d` build, package R2FIX-8d; receiver note; refuting round 2's tests-mutants finding.)* **Also owed here: `run()`'s review-routes binding** (`with_review_routes`, R5 = ADR-0177 decision 2, proposed *(dated 2026-09-27, `s8d` close: accepted on D21-84)*). It has MF3j's shape, and no `rk-cli` test reaches its call site. **Discharged in `s8d` and not owed here:** `run()`'s `sca_scan` database binding (R8 = ADR-0179 decision 4). `adapter_selection::run_binds_sca_scans_advisory_databases_before_the_step_reaches_the_port` kills its mutant R2-A without a dispatch. Its fixture activity opens with a zero-agent `tool` step, and the exit code tells a kernel input refusal (19) from the handler's own refusal (10). The same shape may reach both bindings owed here before the `HarnessFactory` seam exists. A zero-agent fixture activity's first `tool` step would take `selection = "affected"` for MF3j, or `toolchain:shallow_clone` for the review routes. That is a route this section may take, not a claim that it works: `s8d` did not build either.

*(dated 2026-09-28, s8e build, package S8E-P2)* **Both owed bindings are discharged here, through the seam, and the route above is the one taken.** Each test drives `rk_cli::verbs::run::run(` over `pack-min` with a ZERO-AGENT `implement` activity, the stub `cargo` as the one Rust candidate (`support::install_review_toolchain`), a scripted factory that is never asked to dispatch, and an environment of `clean_env` plus the process-start keys the toolchain adapters refuse to run without (`PATH`, `PATHEXT`, `SystemRoot`, `SystemDrive`, `TMP`, `TEMP`; no credential):
- `lane_e2e::run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer` (MF3j). The activity produces `Changeset`, and its first step is `toolchain:test` with `selection = "affected"`. The exit code is the channel, as in `adapter_selection`'s R2-A test. The CONTROL runs with an empty `HEAD..rk-run/bug-1` and must read 19 (the kernel's `carries no change` refusal, before the port). The CASE plants one commit changing `src/lib.rs` on `rk-run/bug-1` before the run and must read anything but 19. Measured: control `Unsupported` (19), case `NonZero` (14), the stub's `cargo metadata` answer reaching the adapter. Under MF3j (`with_run_branch` removed) the case reads 19, *"no run-branch reader is bound"*, and the test is red.
- `lane_e2e::run_binds_the_review_routes_for_a_shallow_clone_step` (the review routes). The activity's first step is `toolchain:shallow_clone` at `HEAD`'s sha. The observation is the clone: `<common-dir>/redkiln/clones/bug-1/<sha>/README`. Measured: exit 0 and the clone present. With `with_review_routes` removed the run exits 19 and no clone exists, and the test is red.

*(dated 2026-09-28, s8e build, package S8E-P2)* **Instrument notes for the fifteen, each a reading of this section rather than a change to it.**
- **Stdout is not readable in process.** `run`'s stdout is the test harness's captured stream, so `run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer` cannot read *"the re-run's stdout reports no open gate"*. It asserts the git and store halves, and `terminal_read_reports_only_an_awaited_gate` reads stdout: each of its three arms opens `gate-1`'s gate in process and re-runs as the binary over the fixture child. After §8e.7 `terminal_read` commits nothing, so S8E-M10 is observable only on stdout, and that arm is the one that carries it.
- **The void arm reads the dead token, not every open line.** After a void answer the kernel's next act is a fresh gate (AC-252), which is awaited and may be printed. The arm asserts that no *"is open"* line and no new opening names the VOID token. Measured at the build: the answered and expired arms are red (the old filter re-commits and re-prints the dead token), and the void arm is already green, because the latest tokened opening is the fresh one.
- **The gate stage's gate is the abstention gate.** The fixture lane's `implement` opens `unknown_facet:has_repro` from its `require` row over an item that omits `has_repro`, the gate every `pack-min` gate test opens, rather than a catalog `gate` step, which would need a `.redkiln/processes/` question.
- **The removal trigger's two arms run in two repositories.** One store holding both items would make the test read §8e.10's `ByItem` before the trigger. Measured: in one repository the second item's lane stalled on the first item's `stages_entered` and never dispatched.
- **The reuse test runs over `gate-1`** (`PackVariant::Base`, one stage), so its second invocation awaits the same gate on today's binary and on the stage's. It plants its ignored file under the run tree's `.redkiln/worktrees/`, which `info/exclude`'s run-root row ignores in every worktree of the repository. Green at the build, and red under S8E-M20.
- **`[project] name` and `slice` landed with the tests, ahead of the mint that forces them.** `pack-min/config.toml` declares `name = "rk-fixture"`, and `items/review-item.toml` (the item `exit/findings-block/` runs) declares `slice = "review-1-fix"`. Both are inert until §8e.6 and §8e.8 mint. AC-438's table, `cost`'s run and `exit/findings-block/` keep their exits (measured green after the edit). `change_deriver`'s fixture had appended its own `[project]` table, which is now a duplicate key, so it asserts the shared one instead.

*(dated 2026-09-28, s8e build, package R3FIX-8e; refuting round 3's minor.)* **The process-boundary test now runs both
`run` invocations through the binary**, as this section prescribes. Its first invocation had been in process
(`verbs::run::run(` over an empty scripted factory), a departure no instrument note above recorded. It is now
`redkiln -C <root> run lane-1` over the fixture child, with `RK_FIXTURE_MODE=replay:<frames>` (the lane's gate opens
before any dispatch, so the transcript is not read), and its exit `0` is asserted, with exactly one answerable gate
opened. The second process's exit is asserted too: **`Schema` (11)**, measured. After it crosses into `review`, that
stage's `review-min` dispatch replays the same `implement-frames.jsonl` transcript, which yields no `Findings`, and the
process prints *"the dispatch of `review-min` (review-min@1) did not commit"*. The crossing is this section's claim
(the `review` mark, and ~~a fixture-child spawn marker~~ *(R4FIX-8e: two, below)*). The exit pins where the process stopped after crossing, so a
second process that halts or refuses before the boundary fails by name. Green after the edit.

*(dated 2026-09-28, s8e build, package R4FIX-8e; refuting round 4's tests-mutants minor.)* **The spawn assertion is
pinned.** It read ~~`spawned_after > spawned_before`~~, which `review-min`'s spawn alone satisfied, so its message
(*"its implement stage never dispatched"*) claimed more than it checked. The second process now spawns **exactly
two** fixture children, counted as the `*.marker` files it added, and exactly one marker's `--json-schema` argv
carries the title `ImplReport` (`implement`'s dispatch) and exactly one the title `Findings` (`review-min`'s).
Measured green (*"test lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child ... ok"*). Two
synthetic controls, run from a restore-verified driver: expecting three spawns reddens it with *"it spawned 2 with
schema titles [Some("Findings"), Some("ImplReport")] (exit Some(11))"*, and renaming `pack-min`'s `Findings.json`
title reddens it with *"exactly one of the second process's two fixture children must carry the `Findings`
`--json-schema`; titles [Some("ImplReport"), Some("FindingsX")]"*. Both files were byte-identical after restore.
**The second process's `Schema` (11) exit is a fixture artifact, not a product defect**, as round 4 judged:
`RK_FIXTURE_MODE=replay:<implement-frames.jsonl>` feeds the same `ImplReport` transcript to every dispatch in the
process, so `review-min`'s dispatch reads no `Findings` and does not commit. The exit assertion stays: it pins where
the process stopped, and a per-dispatch replay is outside this stage's scope.

### 8e.12 Named tests, red first, and the mutants

**Every test of a NEW behaviour is written before the code it covers and watched RED for the reason in its row.** A
test first seen green is not evidence (PIA-035). **Three rows are pins over behaviour that must NOT move, or that no
code violates today**. The first draft named three; the adversarial review added two, re-run for a claim:
- `projection::rebuild_differential`;
- `projection::stage_level_next_action` (AC-161's kernel half);
- `lane_order::adr0161_…`;
- `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record`;
- `tier_ui::a_state_field_without_a_tier_is_a_compile_error` (AC-160's `s6` fixture).

They are green at entry by design, and each is shown able to fail by the mutant named for it:
- ADR-0161's `M10` for the lane-order pin;
- `projection.rs`'s own source lint for the tier filter, for both `projection::` rows;
- `S8E-M18` for the legacy rule;
- `s6`'s recorded AC-160 mutant (the fallback rule accepting an unannotated field, which produced `mismatch`).

The two run-worktree pins this list used to carry (the planted tree and the filter control) moved to `s8d-ii` with
their work (`s8d-ii` §8d-ii.8).

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
| `rk-store` | `store::writer_poison_is_a_halt_and_never_recovered` | `into_inner` recovers |
| `rk-store` | `store::tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree` | no statement exists |
| `rk-store` | `store::t1_gate_transition_appends_one_row_per_transition_and_is_never_updated` | no table exists |
| `rk-store` | `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` *(dated 2026-09-28, s8e build, package D2196-8e)* | the `answered` row takes the payload's tree (*"left: 0000…"*, *"right: bbbb…"*, measured against the pre-D21-96 writer); red under S8E-M58 and S8E-M59 |
| `rk-store` | `store::migration_0002_adds_the_transitions_table_and_the_item_column_and_stamps_version_two` | `SCHEMA_VERSION` is 1 and no `0002` exists |
| `rk-store` | `store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged` | no checked-in fixture store exists |
| `rk-store` | `tier_ui::a_state_field_without_a_tier_is_a_compile_error` | green — AC-160's `s6` instrument, a pin re-run for the claim |
| `rk-ports` | `event_store_port::ac364_event_store_port_has_no_update_or_delete` (asserts five) | the method count is four |
| `rk-ports` | `fake_behaviour::vec_store_append_tier1_is_queryable_with_its_anchor` | no method |
| `rk-ports` | `fake_behaviour::vec_store_query_by_item_returns_only_that_items_rows` | the selector is ignored |
| `rk-adapter-sqlite` | `conformance::sqlite_store_conforms_including_the_tier_one_arm` | no arm |
| `rk-adapter-sqlite` | `store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction` | `anchor_sha` is NULL |
| `rk-adapter-sqlite` | `store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows` | no `open_at` |
| `rk-adapter-sqlite` | `store::query_by_item_returns_only_that_items_rows` | `_by` is ignored |
| `rk-adapter-sqlite` | `registry::ac489_capability_registry_dated` (re-pointed: both rows carry a recorded probe) | both rows are probe-less |
| `rk-adapter-sqlite` | `registry::snapshot_and_rebuild_rows_carry_a_recorded_probe` | both rows are `Probe::None` |
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
| `rk-kernel` | `tier_one::every_run_commit_resolves_through_the_worktree_resolver` | `run.rs` and `answer.rs` call `commit_with_trailers` directly |
| `rk-kernel` | `tier_one::a_closed_mark_heals_as_the_lanes_end_transition` *(dated 2026-09-28, s8e build, package S8E-P6)* | the heal skips a `closed` mark by name |
| `rk-kernel` | `conformance::conformance_every_trailer_key_has_a_producer` (un-ignored) | `#[ignore]`, and 0 seals |
| `rk-kernel` | `projection::rebuild_differential` | green — the kernel half; its dated header line is new |
| `rk-kernel` | `projection::stage_level_next_action` | green — AC-161's kernel half, a pin re-run for the claim |
| `rk-kernel` | `lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again` | green — the regression witness for the seed change |
| `rk-kernel` | `lane_order::a_pack_stage_named_closed_is_refused_at_load` | `closed` is an ordinary name |
| `rk-kernel` | `fsm::every_item_scoped_kind_carries_the_interim_item_key` | no `item` member |
| `rk-kernel` | `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` | no key carrier; its first arm alone is a pin, and its second cannot be written |
| `rk-kernel` | `fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition` *(dated 2026-09-28, s8e build, package S8E-P7; D21-93)* | `terminal()` reads `Sealed` at a release gate after a seal made in review |
| `rk-kernel` | `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` *(dated 2026-09-28, s8e build, package S8E-RULINGS)*; D21-92) | pass 1's `null` `until` leaves no row (*"left: [("g", "repeat:2", …)]"*) |
| `rk-kernel` | `on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes` *(dated 2026-09-28, s8e build, package S8E-RULINGS)*; D21-92) | green — a pin: a refusal-routed `null` never left a row; red under S8E-M52 |
| `rk-cli` | the fifteen `lane_e2e::…` tests in §8e.11 | `verbs::run::run(` has no FakeHarness seam |
| `rk-cli` | `lane_e2e::run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer` *(dated 2026-09-28, s8e build, package S8E-P2)* | `verbs::run::run(` has no FakeHarness seam; green once it exists, red under MF3j |
| `rk-cli` | `lane_e2e::run_binds_the_review_routes_for_a_shallow_clone_step` *(dated 2026-09-28, s8e build, package S8E-P2)* | `verbs::run::run(` has no FakeHarness seam; green once it exists, red under S8E-M49 |
| `rk-cli` | `slice_verdict::approved_is_sealed_on_the_advance_path` | no seal |
| `rk-cli` | `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` | no seal |
| `rk-cli` | `slice_verdict::an_absent_slice_is_refused_by_name` | no key |
| `rk-cli` | `slice_verdict::a_slice_equal_to_the_item_id_is_refused` | no key |
| `rk-cli` | `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch` *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-94, D21-95)* | the lane dispatches `implement` and `review` and only the mint refuses (*"the lane dispatched 2 time(s) … left: 2, right: 0"*) |
| `rk-cli` | `lane_e2e::a_lane_that_reaches_no_findings_stage_is_not_refused_for_an_undeclared_slice` *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-95)* | green — the control: a lane with no `Findings` stage and no `slice` dispatches and exits 0 |
| `rk-cli` | `lane_e2e::the_entry_check_skips_a_forbidden_current_stage_with_a_findings_activity` *(dated 2026-09-28, s8e build, package R1FIX-8e)* | green — a pin on the guard `findings_stage_reached` already carries (round 1: no fixture branch reached it); red under S8E-M56 (*"a `forbidden` current stage's `Findings` activity was read by the entry check"*) |
| `rk-cli` | `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop` *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-94)* | no shared validator (`validate_slice` does not resolve); a pin on the mint thereafter, red under S8E-M55 |
| `rk-cli` | `worktree_assigned::a_tier1_write_from_a_foreign_worktree_is_refused_typed` | no row kind |
| `rk-cli` | `worktree_assigned::the_positive_control_from_the_run_owned_worktree_succeeds` | no row kind |
| `rk-cli` | `worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention` | convention path |
| `rk-cli` | `worktree_assigned::a_reentry_records_reused_and_its_discard_list` | no row kind |
| `rk-cli` | `worktree_assigned::answer_refuses_fail_closed_without_an_assignment_row` | `answer` reads `reconcile_started` |
| `rk-cli` | `answer::a_failed_answer_commit_leaves_zero_rows` | `answer` appends first |
| `rk-cli` | `answer::answer_presents_the_awaited_gate_over_a_spent_token_that_sorts_higher` *(dated 2026-09-28, s8e build, package R1FIX-8e)* | `answer` presents the LAST token in token order (*"answer presents the ANSWERED token 3d264e16… over the awaited 019d34cc…"*); red under S8E-M57 |
| `rk-cli` | `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` | no `t1_*` rows; *(dated 2026-09-28, s8e build, package D2196-8e)* re-pointed through a void (D21-96): red at the package's entry, only recorded `…\|answered\|6aaf314e…`, only rebuilt `…\|answered\|6ae68e31…`; red under S8E-M58 |
| `rk-cli` | `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` | gates unresolvable |
| `rk-cli` | `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record` | green today — the legacy rule's pin |
| `rk-cli` | `doctor::a_version_one_store_is_refused_by_name_and_rebuild_re_derives_it_at_version_two` | `SCHEMA_VERSION` is 1 |
| `rk-cli` | `root::adr0118_a_poisoned_writer_mutex_halts_exit_70` | the writer recovers |

**The count.** 80 names: `rk-git` 11, `rk-store` 6, `rk-ports` 3, `rk-adapter-sqlite` 6, `rk-kernel` 24, `rk-cli` 30.
Seven already exist: the AC-364 parse, the ARMED lint, `rebuild_differential`, the ADR-0161 pin, AC-160's `tier_ui`
fixture, `stage_level_next_action` and the re-pointed `ac489_…`. So **73** are new, and with the un-ignored lint the
suite gains **74** passed (§8e.0 item 7). The adversarial review of 2026-09-24 added ten names, seven of them new:
the removal trigger, the AC-163 replay, the AC-180 fixture, the registry flip, the AC-518 commit lint, and the two
`worktree_assigned` rows. Before it the count was 70 names and 66 new. The earlier count was 97 names. The 30
run-worktree names, one of them a rename, moved to `s8d-ii` (`s8d-ii` §8d-ii.8), and R2 and R3 added three.

*(dated 2026-09-28, s8e build, package S8E-P2)* **82 names** (~~80~~): `rk-cli` **32** (~~30~~), the other crates unchanged. §8e.11's two owed-in tests are new, so **75** are new (~~73~~) and the suite gains **76** (~~74~~). The floor is 1034 + 76 = **1110** (§8e.0 item 7).

*(dated 2026-09-28, s8e build, package S8E-P6)* **83 names** (~~82~~): `rk-kernel` **25** (~~24~~), with `tier_one::a_closed_mark_heals_as_the_lanes_end_transition` added for the heal's `closed` arm (§8e.6). So **76** are new (~~75~~), the suite gains **77** (~~76~~), and the floor is 1034 + 77 = **1111**.

*(dated 2026-09-28, s8e build, package S8E-P7)* **84 names** (~~83~~): `rk-kernel` **26** (~~25~~), with D21-93's `fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition`. So **77** are new (~~76~~), the suite gains **78** (~~77~~), and the floor is 1034 + 78 = **1112**. Measured at the package's diagnostic whole-suite run: 1112 passed / 0 failed / 9 ignored over 44. **Also re-pointed in this package, none deleted:** `doctor::ac427_…`, `doctor::ac429_…` (§8e.9's note), `registry::ac489_…`, and `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit`, which reads ids as text (its premise failed on its own SQL read) and gains an EXPIRY arm over `pack.gate-1s.toml`, because the fixture lane lets no gate expire and S8E-M43's lane half was otherwise a missing fixture branch.

*(dated 2026-09-28, s8e build, package S8E-RULINGS)* **86 names** (~~84~~): `rk-kernel` **28** (~~26~~), with D21-92's `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` and `on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes` (ADR-0187, proposed). So **79** are new (~~77~~). `s8d`'s `on_false_exec::a_null_until_or_gate_when_is_not_journaled` is **deleted**, not re-pointed: it pinned the rule D21-92 reverses, and D21-92 names its replacement. The suite therefore gains **79** (~~78~~) — 79 new, plus the un-ignored lint, less the replaced test — and the floor is 1034 + 79 = **1113**.

*(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY)* **89 names** (~~86~~): `rk-cli` **35** (~~32~~), with D21-94's entry refusal as D21-95 rules it: `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch`, its control `lane_e2e::a_lane_that_reaches_no_findings_stage_is_not_refused_for_an_undeclared_slice`, and the mint's pin `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop` (ADR-0186 as amended, proposed). So **82** are new (~~79~~), the suite gains **82** (~~79~~), and the floor is 1034 + 82 = **1116**.

*(dated 2026-09-28, s8e build, package R1FIX-8e)* **91 names** (~~89~~): `rk-cli` **37** (~~35~~), with refuting round 1's two: `answer::answer_presents_the_awaited_gate_over_a_spent_token_that_sorts_higher` (the answer's gate selection, §8e.9's R1FIX-8e note) and `lane_e2e::the_entry_check_skips_a_forbidden_current_stage_with_a_findings_activity` (the entry check's `forbidden` guard). So **84** are new (~~82~~), the suite gains **84** (~~82~~), and the floor is 1034 + 84 = **1118**. **Also re-pointed, none deleted:** `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit` (the lane walks through a void), `answer::ac248_answer_without_live_token_exits_named_outcome` (its expired arm asserts the expiry's `gate_expired` row is anchored to a `Gate: … expired` commit; S8E-M43c), and `lane_e2e::a_second_invocation_reuses_the_run_worktree` (its planted directory is taken through `rk-git`'s one derivation, not a joined literal — round 1's blocker).

*(dated 2026-09-28, s8e build, package D2196-8e)* **92 names** (~~91~~): `rk-store` **7** (~~6~~), with D21-96's `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` (the writer's refusal branch, which no lane reaches). So **85** are new (~~84~~), the suite gains **85** (~~84~~), and the floor is 1034 + 85 = **1119**. **Also re-pointed, none deleted:** `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` (its lane walks through a void, and it pins the observed tree in the live `gate_answer_void` row).

*(dated 2026-09-28, s8e build, package R3FIX-8e)* **92 names, unchanged; no test is new, so the floor stays 1034 + 85
= 1119.** Checkpoint 8e (2)'s `rk-cli` selection names one more existing test, `answer::ac248_answer_without_live_token_exits_named_outcome`
(S8E-M43c's only killer, re-pointed by R1FIX-8e), so it reads `sel rk-cli 38` (~~`sel rk-cli 37`~~); it is outside this
table's count, as the other re-pointed pre-existing tests are. **Also re-pointed, none deleted:**
`lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` (the row's `anchor_sha` is the `Story: … triage` entry
commit's sha, and its reason is a member of `STAGE_EMPTY_REASONS`; S8E-M60, S8E-M61) and
`lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child` (both `run` invocations are the binary, and both
exits are asserted; §8e.11's R3FIX-8e note).

The existing `story_axis::adr0155_…` tests and `doctor::ac427_…`'s synthetic repository are **re-pointed** in the
record's commit, and none is deleted. `story_value` gains the stage, and the old single-mark assertions become the
legacy-mark fixture. The six test sites that derived the old root were re-pointed by `s8d-ii` (`s8d-ii` §8d-ii.1).

**The mutants.** Each mutant is applied and restored **from Python, never the shell**: a backtick once swallowed a
restore, and the next backup captured the mutant. Each diff is read. Each must turn the named test RED **for the
named reason**, printed and read, not merely "the suite went red". Names carry an `S8E-` prefix, because `M7` and
`M10` are already taken by `s7h`. S8E-M19, M21 to M24 and M30 to M33, and the `rk-git` half of M20, moved to
`s8d-ii` as `S8D-II-M1` to `S8D-II-M10` (`s8d-ii` §8d-ii.8). The names that stay are kept, so the numbering has gaps.

| mutant | change | must redden |
| --- | --- | --- |
| S8E-M1 | `through_step_seven` seeds from the item record, ignoring the reconciled stage | `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane`, `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` |
| S8E-M2 | the mint's idempotence read restored to *"item already committed"* (ADR-0155's first boundary only) | `lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order` |
| S8E-M3 | `append_tier1` before the commit | `tier_one::a_failed_commit_leaves_zero_store_rows`, `tier_one::the_commit_exists_before_any_row_is_readable` |
| S8E-M4 | `append_tier1` drops the `t1_*` insert | `store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction`, `doctor::ac184_…` |
| S8E-M5 | the heal disabled | `tier_one::a_commit_without_a_row_is_healed_at_entry` |
| S8E-M6 | the heal ignores reachability | `tier_one::a_heal_writes_no_row_for_an_unreachable_anchor` |
| S8E-M7 | no seal on the `FindingsBlock` path | `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` |
| S8E-M8 | an absent slice defaults to the item id | ~~`slice_verdict::an_absent_slice_is_refused_by_name`~~ *(dated 2026-09-28, s8e build, package R1FIX-8e)* split by siting and measured, the two rows below (round 1: since the entry refusal, a mint-sited default survives this test) |
| S8E-M8a *(dated 2026-09-28, s8e build, package R1FIX-8e)* | the default in `seal_act` alone (the mint siting): `validate_slice(declared.or(Some(item)), …)` | `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop` (*"the mint refused None for the wrong reason … left: ItemId, right: Absent"*); measured GREEN: `slice_verdict::an_absent_slice_is_refused_by_name`, `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch` (the entry refuses first) |
| S8E-M8b *(dated 2026-09-28, s8e build, package R1FIX-8e)* | the default in the shared `validate_slice` (entry and mint): an absent slice refused as `ItemId` | `slice_verdict::an_absent_slice_is_refused_by_name` (*"the refusal names the ABSENCE … not another rule"*: the entry names the item-id rule), `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop`, `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch` |
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
| S8E-M20 | the run path re-creates the tree on re-entry (the `rk-git` half is `s8d-ii`'s `S8D-II-M2`) | `lane_e2e::a_second_invocation_reuses_the_run_worktree` |
| S8E-M25 | `query` ignores `ByItem` again | `store::query_by_item_returns_only_that_items_rows`, `lane_e2e::two_items_in_one_store_do_not_fold_together` |
| S8E-M26 | `answer` appends `gate_answered` before its commit | `answer::a_failed_answer_commit_leaves_zero_rows`, ~~`tier_one::a_gate_answer_is_committed_before_its_row`~~ *(dated 2026-09-28, s8e build, package R1FIX-8e)* the kernel test is struck from this `rk-cli` change: it never reaches `answer.rs` and was measured GREEN under it (S8E-M26a, P5's name). `answer::a_failed_answer_commit_leaves_zero_rows` alone kills it (*"the failed commit left rows beyond the invocation's baseline: [ReconcileStarted, GateAnswered]"*) |
| S8E-M26b *(dated 2026-09-28, s8e build, package R1FIX-8e)* | the KERNEL boundary appends an act's `gate_answered` row before its commit (P5's kernel variant) | `tier_one::a_gate_answer_is_committed_before_its_row` (*"a row was appended while HEAD was not yet the act's commit"*), `answer::a_failed_answer_commit_leaves_zero_rows` |
| S8E-M27 | the answer's `Gate:` mark written with state `opened` | `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered` |
| S8E-M28 | no `closed` mark at the lane's end | `lane_e2e::the_lane_end_mints_a_terminal_closed_mark` |
| S8E-M29 | the live `GitOps` journal writes no `git_op` row | `tier_one::the_boundary_journals_a_git_op_row_per_invocation`, `tier_one::replay_serves_the_boundary_from_its_git_op_rows` |
| S8E-M34 | the gate-transition writer updates one row per token instead of appending one per transition | `store::t1_gate_transition_appends_one_row_per_transition_and_is_never_updated` |
| S8E-M35 | the interim key written into `payload` | `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` (its second arm) |
| S8E-M36 | `doctor --rebuild` opens the old store through the identity check before rebuilding | `doctor::a_version_one_store_is_refused_by_name_and_rebuild_re_derives_it_at_version_two` |
| S8E-M37 | the `closed` mark does not invoke `s8d-ii`'s removal rule | `lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one` (its clean arm) |
| S8E-M38 | the trigger removes the tree with `--force`, skipping the clean check | `lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one` (its dirty arm) |
| S8E-M39 | the SQLite registry's `rebuild` row restored to `Probe::None` | `registry::snapshot_and_rebuild_rows_carry_a_recorded_probe` |
| S8E-M40 | the version-1 open stamps the identity row before it refuses | `store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged` |
| S8E-M41 | `worktree_assigned` written with `reused: false` and an empty `discarded` on re-entry | `worktree_assigned::a_reentry_records_reused_and_its_discard_list` |
| S8E-M42 | `answer` falls back to the convention path when no `worktree_assigned` row exists | `worktree_assigned::answer_refuses_fail_closed_without_an_assignment_row` |
| S8E-M43 | a gate expiry's row appended with no commit (the store-only path restored) | `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit`, ~~`tier_one::a_gate_expiry_is_committed_before_its_row`~~ *(dated 2026-09-28, s8e build, package R1FIX-8e)* the kernel test is struck from the `rk-cli` change: sited as S8E-M43a (`run`'s expiry rows through `run::gate_rows_through`'s store-only arm) it was measured GREEN; the lane test's expiry arm alone kills it (its anchor reads `""`) |
| S8E-M43b *(dated 2026-09-28, s8e build, package R1FIX-8e)* | the KERNEL boundary appends an expiry's row with no commit and reports `NoGitWrite` (P5's kernel variant) | `tier_one::a_gate_expiry_is_committed_before_its_row` (*"a live act reported NoGitWrite"*), `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit` |
| S8E-M43c *(dated 2026-09-28, s8e build, package R1FIX-8e)* | `answer`'s expiry rows through `run::gate_rows_through`'s store-only arm | `answer::ac248_answer_without_live_token_exits_named_outcome` (its expired arm, re-pointed here: *"not anchored to a `Gate: … expired` commit … anchors [None]"*). SURVIVED every named test before the re-point (a missing fixture branch) |
| S8E-M44 | `answer.rs` calls `commit_with_trailers` directly again | `tier_one::every_run_commit_resolves_through_the_worktree_resolver` |
| S8E-M45 | the `review_pass` seal deleted while `doctor/rebuild.rs` names `TrailerKey::SliceVerdict` | `conformance::conformance_every_trailer_key_has_a_producer` |
| S8E-M46 | an `rk-cli` code line calls `EventStore::append_tier1(…)` in path form | `tier_one::the_tier_one_writer_is_called_only_from_the_boundary` |
| S8E-M47 | the `opened` transition row written with a null `run_id` | `lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer` |
| S8E-M48 | the rebuild fills a re-derived `opened` row's `run_id` from the `Gate:` commit's `Story:` stage and digest, in place of null (D21-48 X11) | `doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` |
| MF3j | `run()`'s `routing.with_run_branch(..)` binding removed (`s8d` §8d.10's name, kept; *(dated 2026-09-28, s8e build, package S8E-P2)*) | `lane_e2e::run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer` |
| S8E-M50 *(dated 2026-09-28, s8e build, package S8E-P7; D21-93)* | the seal's stage scope dropped: `terminal()` reads `Sealed` whenever a seal was folded | `fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition` |
| S8E-M51a *(dated 2026-09-28, s8e build, package S8E-RULINGS)*; D21-92) | a continuing `null` `until` not journaled (the old rule) | `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` (its `until` arm) |
| S8E-M51b *(dated 2026-09-28, s8e build, package S8E-RULINGS)*; D21-92) | a continuing `null` gate `when` not journaled (the old rule) | `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` (its `when` arm) |
| S8E-M51c *(dated 2026-09-28, s8e build, package S8E-RULINGS)*; D21-92) | a journaled `null` read back as no decision, so the re-entry re-decides it | `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` (its re-entry assertion) |
| S8E-M52 *(dated 2026-09-28, s8e build, package S8E-RULINGS)*; D21-92) | a `null` `until` routed to a refusal journaled | `on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes` |
| S8E-M53 *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-94)* | the entry check dropped (`findings_stage_reached(..)` filtered to `None`) | `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch` (*"dispatched 2 time(s)"*); `slice_verdict::an_absent_slice_is_refused_by_name` stays green under it, on the mint's backstop |
| S8E-M54 *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-95)* | the walk reads the current stage only (`at = None` in place of `pack.successor(stage)`) | `lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch` (*"dispatched 2 time(s)"*: `triage` has no `Findings` activity) |
| S8E-M55 *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-94)* | the validator's call dropped from `seal_act` (the declared value minted as it stands) | `slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop` (*"the mint accepted None: its backstop is gone"*) |
| S8E-M56 *(dated 2026-09-28, s8e build, package R1FIX-8e)* | the entry check's `forbidden` guard dropped in `findings_stage_reached` | `lane_e2e::the_entry_check_skips_a_forbidden_current_stage_with_a_findings_activity` (*"a `forbidden` current stage's `Findings` activity was read by the entry check … exit Some(3)"*) |
| S8E-M57 *(dated 2026-09-28, s8e build, package R1FIX-8e)* | `answer`'s presented gate restored to `open_gates().filter(not expired).last()` | `answer::answer_presents_the_awaited_gate_over_a_spent_token_that_sorts_higher` (*"answer presents the ANSWERED token … over the awaited …"*); `answer::ac407_…` stays green (the fallback) |
| S8E-M58 *(dated 2026-09-28, s8e build, package D2196-8e)* | the `answered` row's tree restored to the payload's observed tree (`text(event, "tree_hash")` first, the bound tree only as its fallback) | `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` (only recorded `…\|answered\|af107778…`, only rebuilt `…\|answered\|3a288bb5…`), `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` (*"left: 0000…"*, *"right: bbbb…"*) |
| S8E-M59 *(dated 2026-09-28, s8e build, package D2196-8e)* | the payload's tree taken when the store holds no `opened` row (the refusal dropped) | `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` (*"was not refused by the bound-tree rule: Ok(())"*); no lane reaches the branch |
| S8E-M60 *(dated 2026-09-28, s8e build, package R3FIX-8e; refuting round 3's major)* | `replay::resume_routed`'s entry act takes the entry row alone: `1 + usize::from(empty_row.is_some())` → `1`, so the `stage_empty` row is appended beside the act, unanchored | `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` (*"the `stage_empty` row is not anchored to the entry act's `Story: rk-fixture/lane-2 triage` commit … left: None, right: Some("e475104c…")"*). Measured SURVIVING the test at `259f714` (*"ok. 1 passed"*), which read no anchor (a missing assertion), in a private worktree; KILLED after the re-point |
| S8E-M61 *(dated 2026-09-28, s8e build, package R3FIX-8e)* | the `stage_empty` reason outside the closed set: `STAGE_EMPTY_REASONS[0]` → `"no applicable activity"` | `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` (*"the row's reason Some("no applicable activity") is not one of the closed set ["no_applicable_activity"]"*). Measured SURVIVING at `259f714` (the test accepted any non-empty reason); KILLED after the re-point |
| S8E-M49 | `run()`'s `routing.with_review_routes(..)` binding removed (`s8d` R2FIX-8d's review-routes mutant; *(dated 2026-09-28, s8e build, package S8E-P2)*) | `lane_e2e::run_binds_the_review_routes_for_a_shallow_clone_step` |

A mutant that survives is not waved through. It is either an **unobservable property**, which moves to a source lint
with its reason written down, as `projection.rs`'s tier filter did, or a **missing fixture branch**. Both are recorded
by the mutant's name.

### 8e.13 What moves in other checkpoints, how 7h and 7i stay runnable, and what this file found

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

**Checkpoint 8a is amended, dated, in the same commit, and re-run at Checkpoint 8e** (the cross-file review's
second blocker; D21-47: *"A time-bound guard in an earlier checkpoint is amended by the stage that moves its premise,
in the same commit"*). 8a guard (7) pins the trailer lint as ignored. Its lines expect `... ignored, ARMED:
Slice-Verdict has 0 producers`, the reason's `discharged by s8e`, and `0 passed; 0 failed; 1 ignored`. After §8e.8
all three are RED by design, so `s8i`'s and `s8j`'s replays of 8a would fail. The dated amendment replaces them with:
- `test conformance::conformance_every_trailer_key_has_a_producer ... ok`;
- `1 passed; 0 failed; 0 ignored`;
- the `ARMED: Slice-Verdict has 0 producers` string absent from `conformance.rs`, with grep's `rc` exactly 1.

8a's §1 row becomes `passed …, re-run pending`, and 8a joins this stage's provisional set (§8e.15). Checkpoint 8e
asserts the amendment on 8a's extracted bytes before it re-runs 8a.

**Checkpoint 7f is amended, dated, in the same commit, and re-run at Checkpoint 8e.** After `s8a` its literal expects
the lint ignored, with `test result: ok. 2 passed; 0 failed; 1 ignored` (`s8a` §8a.5). `s8a` says in terms that
*"`s8e`'s Slice-Verdict commit moves this literal again"*. One edit is this stage's, and `s8d-ii` made the other two:
1. **The literal:** `3 passed; 0 failed; 0 ignored`, each of the three lints `... ok`, and the `ARMED:` reason absent.
   `s7f`'s reason regex follows.
2. **The run arm in a throwaway clone** and 3. **the deleted cleanup line** are `s8d-ii`'s (`s8d-ii` §8d-ii.7),
   because its binary is the one that would break 7f without them. This stage's binary adds a reason. Run in this
   repository, 7f's arm would mint stage-bearing marks on the untouched fixture item (D21-42 18), which breaks 7h's
   (2) and (3). Checkpoint 8e re-asserts both amendments on the extracted bytes before it re-runs 7f.

The arm still cannot dispatch at this stage, because 0 shipped cards carry `permission_mode`. Checkpoint 8e asserts
that before it re-runs 7f. From `s8h` on, the brief's no-accidental-dispatch rule applies: 7f's arm runs only through
`s8h`'s fixture-child route, and Checkpoint 8e **refuses** to re-run 7f once a card carries `permission_mode`.

*(dated 2026-09-28, s8e build, package R2FIX-8e)* **Checkpoint 7c is amended, dated, and re-run at `s8j`, not
here.** ADR-0185's three kinds move `EventKind::ALL` from 38 to 41, the premise of 7c's ALL-count guard, so that line
moves to 41 by D21-47's time-bound rule (finding 24). **Checkpoint 8d was amended, dated, by package S8E-RULINGS**
(D21-92's two `on_false_exec` names replace the one the ruling reverses in its `run_named` list), and `s8f`'s
Checkpoint 8f (9) re-runs it whole (finding 25). Both join the provisional set (§8e.15); Checkpoint 8e replays neither.

**How Checkpoints 7h and 7i stay runnable.** Both read `rk-run/rk-fix-doctor-test-count`, and 7i also reads the
shipped store and `redkiln cost`.

- **No lane runs against the product repository at this stage.** Every run is in a fixture repository or 7f's clone.
  So the ref stays at `73c087a` with its one legacy mark. 7h's (2) (exactly one real `Story:` trailer) and (3) (the
  value `redkiln-rs/rk-fix-doctor-test-count`, slug equal to the item) keep reading what they read today, and the
  legacy mark is exactly what §8e.5 admits read-only. Checkpoint 8e asserts the ref's sha and the mark's value at its
  start and at its end.
- 🛑 **Running `redkiln run rk-fix-doctor-test-count` in this repository under this stage's binary would break 7h on its
  own.** If the legacy registration still holds the branch, W2's re-entry check refuses (`s8d-ii` §8d-ii.3). If it
  is gone (§8e.0 item 1), the run
  creates `.redkiln/worktrees/rk-fix-doctor-test-count` on the existing branch and mints stage-bearing marks. 7h's (2)
  would then count more than one, and (3)'s slug would carry a stage token. D21-42 18 keeps that item *"the untouched
  fixture item"*, and `s8j` re-points 7h and 7i when it mints the fresh item. Until then nobody runs it.
- **The shipped store moves to schema version 2**, by the explicit rebuild §8e.2 describes (R2). `redkiln cost` then
  opens it, and 7i's `mode=ro` probe reads its `t2_event`, which now holds what the rebuild re-derived. That makes 7i
  provisional, and Checkpoint 8e re-runs it whole. Checkpoint 8e asserts the shipped store stamps version 2 before any
  re-run. It backs up and restores `.git/redkiln/store.sqlite3` (with its WAL, its SHM and the `rebuilt` sidecar)
  around `s8` (2)'s `doctor --rebuild`, and it byte-compares the restore.
- **The binary fixtures 7h (10a) and 7i (3) run get `[project] name` and, for `findings-block`, a `slice`**, in the
  commit that makes entry mint (§8e.11). `s8d-ii` already gave them the ignore. Their exits do not move.

**Findings made while authoring this file, each measured** (1–9 from the first draft, and their disposition now):

1. **`gate_expired` is tier 1 with no trailer axis.** *Settled by N6*: it is a `Gate: … expired` mark (§8e.7).
2. **`t1_gate` implies an `UPDATE` AC-364 forbids.** *Bound by N6* (*"append-only"*), and *settled by D21-46 R2*:
   route (A), the forward migration to `t1_gate_transition` (§8e.2, friction 3).
3. **Claiming AC-162 needs more than D21-41 e names.** *Made explicit*: ADR-0155 decision 5(b) and its bullet, and
   ADR-0158 decision 3 and its bullet, are in the records table.
4. **Two forbids bullets the stage-location decision does not list.** *Made explicit*: ADR-0157's exit-0 bullet and
   ADR-0161's lane-loop bullet are in the records table.
5. **An answered gate is indistinguishable from an open one in git.** *Settled by N6*'s state token (§8e.7).
6. **The `deadline` spelling.** *Settled by N8*: integer unix seconds (§8e.7).
7. **A multi-item store folds items together.** *Settled by N5* (§8e.10), and its golden-log cost by *R3*: the key
   is outside the canonical digest, and a test pins the digests.
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

**Findings from the D21-43/44/45 revision.** Its findings 10 to 13 and 15 to 17 were about the run worktree, and they
moved to `s8d-ii` §8d-ii.9 with the work they describe. The two that stay are renumbered:

10. **`s7e` §7e.1's work block has the same backtick defect as 7h (10a).** It is not in Checkpoint 7e's own block
    (`s8a` finding (10), found by content). At `a6349b6`, `s7e-pack-authoring.md`'s FAIL message
    reads `"… Step 7d's `init` did not run …"`, inside double quotes, so its FAIL path runs `init`. A quote-aware scan
    of every `runbook/` bash block found exactly these two. D21-46 assigns 7e's fix to `s8a` (*"is fixed by dated
    amendment in **s8a**"*). This stage fixes 7h's only.
11. **A content commit whose step row never reached the store is not covered by the tier-1 heal** (§8e.3). A kill
    between `s8f`'s landed commit and its step row leaves the run ref ahead of the journal. W2's re-entry (`s8d-ii`
    §8d-ii.3) then reuses a tree whose `HEAD` already holds the step's effect. `s8f`'s landing record settles how that
    step is read.

**Findings from this split (D21-46):**

12. **R3 overtakes N5's supersession.** N5 says the interim key *"Supersedes ADR-0131's first forbids bullet in
    part"*. That bullet forbids the key **inside** `payload`. R3 carries it outside, so the record supersedes nothing
    of ADR-0131, and it says so rather than taking a supersession it no longer needs. `s8f` cites §8e.10 for
    `dispatch_completed`'s `item`, and its text should be read against the carrier R3 fixed.
13. **R2's rebuild drops the shipped store's tier-2 rows, and it makes 7i provisional.** At `c5d19d7` those are six
    rows (`dag_assembled` 1, `reconcile_started` 3, `stage_transition` 2). Nothing reads them as authority, and the
    store is disposable, but Checkpoint 7i reads this store. So 7i joins this stage's re-run set (§8e.15).
14. **The removal trigger has no lane-level test.** `s8d-ii`'s `rk-git` tests pin the removal rule. No test here runs a
    lane to its `closed` mark and asserts that a clean, landed tree is then removed and a dirty one kept. The
    behaviour is §8e.6's. The record names it, and the adversarial pass decides whether it needs a
    `lane_e2e` row. *Settled by the adversarial review of 2026-09-24:* it does, and
    `lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one` is it (§8e.6), with S8E-M37 and
    S8E-M38.
15. **Citations of this file's old section numbers, which this stage does not edit.** `s8j` cites `s8e` for the
    `.redkiln/` walk exclusion under its old number (now `s8d-ii` §8d-ii.1), and for the 7f amendment under its old
    number (now §8e.13 plus `s8d-ii` §8d-ii.7). `s8c` names `s8e` as the caller of `setup` at creation, which is now
    `s8d-ii`. `s8f`'s Checkpoint 8f runs this block without `RK_CP_NESTED` and without the trap, and its `FLOOR=954`
    was drafted from this file's former 944. Each is the lead's to reconcile under D21-46's integration calls.
    *Settled in the consistency pass 2026-09-24:* `s8j` and `s8c` cite the moved sections, and `s8f` (9) replays this
    block with `RK_CP_NESTED=1` under the trap over the recomputed floor chain.

**Findings from the adversarial review (D21-47):**

16. **The fake `claude` guards a shell's spawn, not a Rust spawn on Windows.** D21-47 puts a fake `claude` that prints
    the pin first on PATH for every replay and every process-level cargo run, and Checkpoint 8e does. The fake is a
    shell script with no extension, as `s8h`'s (10) writes it. Git Bash finds it, but a Rust
    `Command::new("claude")` on Windows resolves `claude.exe`, which it is not. For a Rust spawn, the trap
    `CLAUDE_CONFIG_DIR` and the unset launch record stay the guards, and the trap's emptiness afterwards is the
    evidence. The lead decides whether the fake also needs an `.exe` form. *Settled in the final consistency pass
    2026-09-24:* the shared wrapper, `runbook/tools/nodispatch.sh`, compiles its fake from
    `runbook/tools/fake-claude.rs` to `claude.exe` and removes every other `PATH` directory holding a `claude`,
    `claude.exe` or `claude.cmd`. A Rust spawn therefore finds only the fake, which logs the call and exits 97.
    Checkpoint 8e sources it.
17. **Checkpoint 8a reads four values from the environment.** Its block refuses unless `CS_ADR`, `PACK_ADR`,
    `CODEX_ADR` and `S8A_BASE` are set: the ids 8a's records took at writing, and the commit 8a opened on. A replay
    inherits them. Checkpoint 8e checks they are exported before it re-runs 8a, so a missing value fails with its
    name rather than inside 8a's block. *Settled in the final consistency pass 2026-09-24:* under the one record-id
    convention, 8a's block carries the four values as literals written by dated edit, and it refuses while a
    placeholder remains. A replay supplies nothing, so (10h)'s environment check is deleted.

*(dated 2026-09-28, s8e build, package S8E-P7)* **The three amendments are made, in the Slice-Verdict commit's work.**
Checkpoint 7h guard (6) runs the lint by its module-qualified name and requires its `... ok` line and `1 passed; 0
failed; 0 ignored`; guard (7) requires the ARMED string absent from `conformance.rs` with grep's `rc` exactly 1; the
Slice-Verdict bullet is dated ✅; guard (10a)'s FAIL message lost its two backticks and nothing else. Checkpoint 8a guard
(7) requires the same `... ok` line, `1 passed; 0 failed; 0 ignored` and the absent string. Checkpoint 7f's literal is
all three lints `... ok`, `3 passed; 0 failed; 0 ignored`, none ignored, the ARMED string absent. **In each bash block
the superseded lines are REPLACED, not struck**, with a dated comment saying what they asserted: a struck line would
still carry the ARMED string, which Checkpoint 8e (10h) asserts absent from 8a's extracted block, and (7c) asserts the
old 7h FAIL message absent. The amended lines were exercised read-only over `nd.sh` output (each passes; the old armed
output shape fails each, a control). `PROGRESS.md` §1's 7h, 8a and 7f rows read `re-run pending`.

*(dated 2026-09-28, s8e build, package R1FIX-8e)* **Refuting round 1** (five finders, two refuters per blocker or major; the raw findings in the session
scratchpad, `s8e-r1.json`):

18. **A new `lane_e2e` line tripped Checkpoint 8d-ii guard (3)**, the one-derivation root floor, so Checkpoint 8e (10a)
    would fail (blocker, both refuters): `a_second_invocation_reuses_the_run_worktree` planted its directory at
    `tree.join(".redkiln").join("worktrees")`. *Fixed:* the directory is the parent of `crate::run_tree(&tree, …)`,
    `rk-git`'s one derivation over the run tree the test holds, so 8d-ii's file is not amended. Guard (3)'s lines,
    extracted from 8d-ii's own bytes and run READ-ONLY, pass on the tree and fail on HEAD's tree naming exactly that
    line (the control). **Every other READ-ONLY guard Checkpoint 8e (10) replays was swept** (8d-ii, 7a, 7h, 7f, 8a, `s8` (2) and (3), 7i, each
    block extracted by 8e's own needle and run in two throwaway clones, this package's tree and HEAD, with cargo, `nd`
    and the `redkiln` binary stubbed and every `exit` turned into a marker so each block ran to its end): the one
    difference from HEAD is 8d-ii guard (3), cleared; 7h guard (5) reads the package's own uncommitted edits, which
    the commit clears; every other marker is identical at HEAD and traced to a stubbed cargo or binary run, or to the
    sweep's own rewrite (8a's s7e-line guard, read directly: 1 line; 8d-ii (0b)'s path length, 255 here and 271 only
    under the clone's longer root).
19. **An untracked `.kb/` failed Checkpoint 7h guard (5)** (blocker; environmental, the operator's decision-intake
    files). *Handled outside the tree* by the orchestrator (a local `.git/info/exclude` row); nothing here changes.
20. **`answer` presented the last token in token order, not the awaited gate** (major, both refuters; unowned). *Fixed*
    red first under ADR-0157's stall (§8e.9's R1FIX-8e note; S8E-M57 killed), and the AC-163 lane now walks through a
    void. Walking the ac184 lane through it measured **defect (c)**, the void answer's `answered` row's tree, which
    is put to the operator. *(dated 2026-09-28, s8e build, package D2196-8e)* **Ruled and built:** D21-96 (the row carries the gate's bound tree; the observed tree stays in the anchored `gate_answer_void` row), built red first in `rk_store::tier_one::append`; the ac184 lane walks through the void; S8E-M58 and S8E-M59 killed (§8e.9's D2196-8e note; ADR-0181, `proposed`).
21. **S8E-M8 at the mint survived its named test** (filed major; both refuters minor). *Re-pointed* and split by
    siting, both measured (§8e.12: M8a, M8b); `slice_verdict.rs`'s module and test docs and ADR-0186's rejected
    option corrected by dated edits.
22. **S8E-M26 and S8E-M43 named kernel tests that cannot observe their `rk-cli` change** (major; one refuter
    refuted, citing P5's kernel variants). *Both readings hold, and the rows now say so:* the kernel killers are
    struck from the `rk-cli` changes (M26a, M43a measured GREEN on them) and named for the kernel variants (M26b,
    M43b measured red), and the two kernel tests' docs name what reddens them. Running `answer`'s expiry siting
    (S8E-M43c) found a survivor; `ac248_…`'s expired arm now asserts the expiry's anchor and kills it.
23. *Minors, each fixed:* (a) the entry check's `forbidden` guard had no fixture branch: a pin and S8E-M56;
    (b) ADR-0185 decision 9 still said the heal skips a `Slice-Verdict:` mark: a dated note; (c) design §5's
    `stage_empty` row claimed an anchor the advance path does not give: a dated clause naming the residual;
    (d) ADR-0186's title, `source:` and README row put the refusal at the mint only: dated amendments naming the
    entry, D21-94 and D21-95; (e) ADR-0186's two option headings both read taken: the first's *(taken)* struck,
    `check-adr-options.py` still flags 0; (f) ADR-0181 decision 3's `gate_answer_void` bullet said design §5
    *"still lists"* tier 2: a dated note; (g) `rk-kernel`'s `tests/it/main.rs` said `replay` is gated whole: a
    dated note; (h) `rk-adapter-sqlite`'s conformance docs described the pre-P7 unsupported arm: dated re-points.

*(dated 2026-09-28, s8e build, package R2FIX-8e)* **Refuting round 2** (five finders, two refuters per blocker or
major; the raw findings in the session scratchpad, `s8e-r2.json`). **No blocker.** Two majors and five minors (the first minor names two quotations, 26 (a) and (b)):

24. **Checkpoint 7c's `EventKind::ALL` guard went stale** (major, both refuters): this stage's three kinds (ADR-0185,
    `proposed`) moved `pub const ALL: [Self; 38]` to `[Self; 41]`, and 7c's line
    `grep -qE 'pub const ALL: \[Self; 38\]' crates/rk-types/src/event_kind.rs || …` printed its FAIL on the tree. D21-47:
    *"A time-bound guard in an earlier checkpoint is amended by the stage that moves its premise, in the same
    commit."* *Fixed:* the line is amended in `s7c-adapters.md` by dated in-place edit to `\[Self; 41\]` (ADR-0126's 38
    plus ADR-0185's three), the replaced line kept struck in a dated comment, and `s7c`'s AC-367 table row carries a
    dated note. Extracted from 7c's own bytes and run READ-ONLY, the amended line passes on the tree, and fails over
    `766854c`'s `event_kind.rs` (38) and over a synthetic 42 (the controls). **Every other stage file was grepped** for
    a literal `EventKind` count, an `ALL` size or a kind list a checkpoint line asserts: no other checkpoint line reads
    one (the `ALL.len()` pins live in test code, which the suite runs). 7c joins this stage's provisional set
    (§8e.15); its whole re-run is `s8j`'s, and `s8j` §8j.6's *"nothing in `s7c-adapters.md` is edited except A12's
    `-H`"* carries a dated receiver note. `PROGRESS.md` §1's 7c row reads `passed 2791f1d, re-run pending`.
25. **Checkpoint 8d's block was edited by this stage and not marked provisional** (major; one refuter minor, citing
    `s8f`'s whole re-run of 8d). Package S8E-RULINGS's dated edit to 8d's `run_named rk-kernel "on_false, telemetry,
    breaker"` list (D21-92; ADR-0187, `proposed`) removed `on_false_exec::a_null_until_or_gate_when_is_not_journaled`
    and added `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed` and
    `on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes` (`want` is `$#`,
    so no count literal moves; both names resolve to one `fn` each in `rk-kernel`'s `tests/it/on_false_exec.rs`, the
    removed one to none). The preamble's *"A checkpoint that changed after it passed is provisional"* applies.
    *Fixed:* `PROGRESS.md` §1's 8d row reads `passed b183eb9/0b470ee, re-run pending`, and §8e.15 names 8d provisional
    with its re-run at `s8f`'s Checkpoint 8f (9), whole, rather than an 8d replay added here.
26. *Minors, each fixed:* (a) ADR-0186's `source:` quoted a D21-41 row e sentence the ledger does not carry: replaced
    by the ledger's exact row e text, dated; (b) ADR-0184 quoted §8e.7 as *"by that test, and say so"*: now *"by that
    test, and says so"*, dated; (c) `gate_answer_void` was called a tier-2 row in ADR-0181's own prose, §8e.9's
    D2196-8e note and `doctor.rs`'s ac184 comments, though it is tier 1 (ADR-0184; `EventKind::tier()`): corrected to
    the anchored `t2_event` row, with D21-96's quoted text left verbatim and a dated note beside it in ADR-0181;
    (d) the ledger's title moved 91 → 96 with no dated title note: one added, with its measured count; (e) `PROGRESS.md`
    §1's 8e row still read *"Not started"*: in progress through interim 5; (f) the 7a and 7i rows lacked the
    re-run pending marks §8e.15 implies: added, cleared by Checkpoint 8e (10b) and (10g).

*(dated 2026-09-28, s8e build, package R3FIX-8e)* **Refuting round 3** (five finders, two refuters per blocker or
major; the raw findings in the session scratchpad, `s8e-r3.json`). **No blocker.** One major and eight minors:

27. **The empty first stage's `stage_empty` anchor was untested** (major, both refuters). §8e.6: *"On the entry
    `resume_routed` takes itself (the lane's first stage), the row is a second row of the entry act, anchored to its
    commit."* `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` never read `anchor_sha`, and it accepted
    any non-empty reason. *Fixed:* the test asserts the row's `anchor_sha` equals the sha of the one
    `Story: rk-fixture/lane-2 triage` commit on the run ref, and that the reason is a member of `STAGE_EMPTY_REASONS`.
    Measured in a private worktree at `259f714`: S8E-M60 (`1 + usize::from(empty_row.is_some())` → `1`) and S8E-M61 (a
    reason outside the set) each SURVIVED the old test (*"ok. 1 passed"*) and are KILLED by the new one (§8e.12's rows).
    `replay.rs` was byte-identical after each cycle (sha256 `7991293d…`).
28. *Minors, each fixed:*
    (a) The docs in `fsm.rs`, `breaker.rs` and `memo.rs` counted 38 closed kinds and named a *"thirty-ninth"*. They
        now name no count (the closed set, `EventKind::ALL`), with the old words struck and dated. The same sweep
        re-worded the other *"closed 38"* docs (`budget.rs`, `degrade.rs`, `rk-cli`'s `doctor/mod.rs` and
        `doctor/rules.rs`). ~~`grep -rn 'closed 38\|thirty-ninth' crates --include=*.rs` now finds only struck text.~~
        *(dated 2026-09-28, s8e build, package R4FIX-8e; refuting round 4's minor, item 29 (a).)* That grep could not
        match the spelling *"closed at 38"*, and six live docs said it. They are now struck and dated like the
        others. The claim grep is widened to
        `grep -rnE '(EventKind|kinds?|variants?|vocabulary)[^.]{0,60}\b(38|thirty-eight)\b' crates --include=*.rs | grep -v '~~'`,
        and item 29 (a) records what it returns.
    (b) The process-boundary test ran its first invocation in process. Both are now the binary, and both exits are
        asserted (§8e.11's R3FIX-8e note).
    (c) S8E-M43c's only killer, `answer::ac248_answer_without_live_token_exits_named_outcome`, was not in Checkpoint
        8e (2). It is named now: `sel rk-cli 38`, by dated edit. The floor does not move, because the test is not new.
    (d) ADR-0186 quoted ADR-0159 decision 2 with a full stop where the source has a comma. It now quotes through
        *"…nothing to do with this."*, by a dated note (the record is `proposed`).
    (e) Checkpoint 8e (5)'s probe-less floor was `grep -cF` on the tag, a colon, one space and a brace, over every line.
        It is now whitespace-robust (`"none"[[:space:]]*:[[:space:]]*[{]`) and reads code lines only, with a synthetic
        control. Extracted from this file's bytes and run READ-ONLY: it passes on the tree. Over a copy of
        `registry.rs` with a compact row (`{"none":{`) or a space-before-colon row planted, it FAILS, naming the row.
        The old line counted 0 on both planted copies, so it passed them. A copy that differs only by a doc line naming
        the compact tag passes. `registry.rs`'s *"spelled without the space"* doc sentence carries a dated note.
    (f) Checkpoint 5 was neither marked provisional nor re-run, though this stage moves `crates/rk-types`. `s8d`'s
        precedent is a replay (§8d.11; its Checkpoint 8d (13)). So Checkpoint 8e gains (10i), a conditional whole
        replay of Checkpoint 5 (§8e.15). `PROGRESS.md` §1's row 5 reads `passed 0b470ee, re-run pending`. Its READ-ONLY
        parts were run from this file's bytes. The extractor takes Checkpoint 5's one block (166 lines), and the
        backtick scan reads 0. The `git diff --quiet 766854c` arm reads 1 (moved), and the same arm against `HEAD` reads
        0 (the control). Checkpoint 5 itself runs at Checkpoint 8e's formal run.
    (g) `s8g`'s `verify_completed` will move 7c's amended `[Self; 41]` guard with no receiver note. A dated receiver
        note now sits at `s8g` §8g.1, move 2: it amends the line in its work commit (D21-47).
    (h) `s8j`'s head note listed `s8c`'s 7c amendment but not `s8e`'s. A dated `s8e` clause is added there.

*(dated 2026-09-28, s8e build, package R4FIX-8e)* **Refuting round 4** (five finders, two refuters per blocker or
major; the raw findings in the session scratchpad, `s8e-r4.json`). **DRY: no blocker and no major.** Three distinct
minors (four finders filed the kind-count sweep), each fixed:

29. *Minors, each fixed:*
    (a) Item 28 (a)'s sweep missed the spelling *"closed at 38"*: six live docs, in `rk-kernel`'s
        `workflow/budget.rs` (two) and `workflow/retry.rs`, and in `rk-cli`'s `doctor/mod.rs`, `doctor/rules.rs` and
        `tests/it/doctor.rs`. Each now names the closed set (`EventKind::ALL`) with no count, with the old words
        struck and dated (R4FIX-8e). The widened grep of item 28 (a), with its `grep -v '~~'`, now returns ONE line:
        `crates/rk-kernel/tests/it/fsm.rs`'s assertion message *"EventKind is closed at 41 (… s8e's three-kinds
        record the three beyond 38)"*. That line is not a stale count: it states the current 41, and its 38 is
        the history of the three kinds, so it stays. Without the `grep -v`, the same grep returns 12 lines: those
        11 carry `~~`, and the twelfth is that assertion message. The narrower `grep -rnE 'closed (at )?\**38|38 variants|38 kinds|thirty-ninth'
        crates --include=*.rs | grep -v '~~'` returns 0, and a multi-line scan found no unstruck count
        wrapped across a doc line. The *"thirty-eight"* hits in `doctor/rules.rs`, `no_dispatch.rs` and
        `tests/it/doctor.rs` count `doctor`'s rules, not kinds, and are outside this sweep.
    (b) ADR-0185's Evidence still said the `stage_empty` producer was *"Not measured"*. That clause is struck,
        and a dated note (R4FIX-8e, recording R3FIX-8e's measurement) cites
        `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` and S8E-M14, M60 and M61. The record stays
        `proposed`.
    (c) The process-boundary test's spawn assertion (`>`) was satisfied by `review-min`'s spawn alone. It is
        pinned to exactly two spawns, one carrying the `ImplReport` schema title and one the `Findings` title,
        with two synthetic controls. The `Schema` (11) exit is recorded as a fixture artifact (§8e.11's R4FIX-8e
        note). No test is added, so the floor does not move.

### 8e.14 Acceptance set

**Claimed: AC-160, AC-161, AC-162, AC-163, AC-184 (re-measured), AC-529, AC-174 (reconcile still writes nothing to
git: the void is minted by the kernel), AC-183 (a malformed `Story:` or `Gate:` value is drift, never coerced),
AC-364 (the append-only port with five methods), AC-518 (every commit is on `rk-run/<item>`, and the user's checkout
is unchanged with `s8d-ii`'s tree inside it), and AC-180 (R2's migration is explicit: a version-1 store is refused by
name and rebuilt, never migrated in place).** AC-160 to AC-163 are claimed **over fixture repositories, as
measured**, and only once the supersessions in the records table are accepted. Until the operator's ledger line
flips those records, no document states that any of them is met. *(dated 2026-09-28, `s8e` close: ADR-0181 and ADR-0183 flipped `accepted` on D21-98 and D21-100, so the supersessions in the records table are accepted and in force; AC-160 to AC-163 stand claimed over fixture repositories, as measured.)* AC-161 now covers answered, expired and voided
gates as well as open ones (N6).

**AC-160's owner is this stage**, as `s8j` §8j.9's exclusion already says (*"AC-160…AC-163 belong to `s8e`"*). Its
instrument is `s6`'s: `tier_ui::a_state_field_without_a_tier_is_a_compile_error`, a `trybuild` compile-fail fixture
with its stderr pinned, which Checkpoint 8e re-runs by name. Nothing about it depends on a `t1_*` row. What kept it
unclaimable was ADR-0158's bullet, which named AC-160 with the other three, and the tier-1 record lifts that bullet
for the criteria this stage measures.

**The instruments, and where they differ from the register's Verified-by.** Each difference carries a dated
register note in the commit that makes it true (the attendant edits):
- **AC-161.** The register names *"the delete-the-database replay test in `rk-kernel` over the pinned corpus repo"*.
  The kernel half is `projection::stage_level_next_action`, the equality over tier-1 rows, re-run here. The route,
  a deleted store file and a real `run` seeding from git, cannot live in `rk-kernel`, which may not depend on an
  adapter (AC-484). It is `rk-cli`'s `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` and
  `lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered`, over fixture repositories. The pinned-corpus
  arm is not built, because the corpus carries no stage-bearing mark. The note re-points the route's crate and
  repository and says the corpus arm is owed.
- **AC-163.** The register names *"unit test asserting `HEAD` advanced and the trailer parses; a corpus replay
  asserting one commit per tier-1 transition, which fails on a store row with no matching commit"*. The unit test is
  `tier_one::a_boundary_anchors_every_row_of_the_act_to_its_commit`. The replay is
  `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit`, over the fixture lane, because F12's corpus is a git
  bundle with no store to compare against. The note re-points the corpus to the fixture lane.
- **AC-180.** As the register names it: `store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged`
  over a checked-in version-1 store (§8e.2).
- **AC-518.** The register names *"an integration test asserting the user checkout is unchanged across a full lane run
  … plus a write-path lint asserting every mutating call resolves through the worktree resolver"*. The integration
  test is `lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it`. The lint is
  `tier_one::every_run_commit_resolves_through_the_worktree_resolver`. Its scope at this stage is the commit write:
  every code-line call of `commit_with_trailers` under `crates/rk-cli/src/` and `crates/rk-kernel/src/` is in
  `runtime/tier_one.rs`, where `authorise_tier1_write` precedes it. `redkiln-migrate`'s one call is named with its
  reason: it commits a migration branch, not a run. The file-write and index-change halves are the landing's, and
  `s8f` claims them (`s8f` §8f.11). The note records the lint's scope at each stage. `s8f`'s landing extends the
  boundary function rather than routing around it (`s8f` §8f.1), so its own writes, `commit-tree` and `update-ref`,
  also sit in `runtime/tier_one.rs`. `s8f` brings them under this lint in the commit that adds them (final
  consistency pass 2026-09-24).

*(dated 2026-09-28, s8e build, package S8E-P7)* The AC-161 and AC-163 re-point notes are written in the register as
the two bullets above state them (AC-163's replay also carries the expiry arm §8e.12's note names); AC-518's was
S8E-P5's, and it gains one line that the seal's commit rides the same boundary. Nothing states any criterion met.

**Contributed to, not claimed:**

- **AC-172.** Its instruments are *"the planner accepting only a `Reconciled<State>` value (type-level …); an
  entry-point enumeration test asserting exactly one `reconcile_started` event per invocation"*. Both are `s6`'s and
  `s7d`'s, and this stage changes neither. It stops the seed discarding the reconciled outcome (§8e.5), which makes it
  a consumer of that type, not its instrument. The first draft claimed it on that change, and that claim is withdrawn.

**Excluded:**

- **AC-164**, whose truncation is `s8` (2)'s kernel half, unchanged here.
- **AC-444**, corpus-bound and Phase-2: the dated note only.
- **AC-473 and AC-521**: `s8`'s, over real runs.
- **AC-245**'s gate `tree_hash`: `s7b-ii`'s, cited for clause 4's derivation only. Its untracked-file reading is
  `s8f`'s.

### 8e.15 Checkpoint 8e

- **The writer mutex halts** (ADR-0118 clause 3), and no poison recovery remains in `rk-store`. *(24b)*
- **`EventStore` has five methods**, the fake and both conformance rows exercise the tier-1 arm, `query` honours
  `ByItem`, and design §3.3 carries the dated note. `Capability::Rebuild` is implemented: the SQLite registry declares
  `snapshot` and `rebuild` with a recorded probe, and the real conformance row runs the round trip. *(D21-41 e; N5;
  8c)*
- **Every tier-1 act is git-first through the kernel, every gate transition included.** AC-162's two fault
  injections pass, the reachability lint passes over every call form, replay performs zero git writes and is served
  from `git_op` rows, a commit without a row heals, and every tier-1 row has exactly one trailer commit (AC-163).
  *(8a; N6; N7)*
- **No `t1_` SQL outside `rk-store`, and no `INSERT` left in `rk-cli`.** *(8c; ADR-0158's bullet)*
- **The lane crosses invocations and processes.** Every non-forbidden stage is entered and trailered in declared
  order, an empty stage records why, and the lane's end is a `closed` mark that removes a clean, landed run tree and
  keeps a dirty one. A test under `crates/*/tests` calls `verbs::run::run(`. *(D21-41 a; D21-42 7; 24c; N9)*
- **`Slice-Verdict:` is sealed on both outcomes by a real fixture run**, and the ARMED lint is discharged.
- *(dated 2026-09-28, s8e build, package S8E-D2194-ENTRY; D21-94, D21-95.)* **An undeclared slice is refused at the
  lane's entry, before any dispatch**, on a fixture lane that reaches a `Findings` stage, through the validator the mint
  shares; a lane that reaches none is not refused; the mint keeps the refusal as the backstop. Inert on the shipped
  packs until `s8g` (0 shipped activities provide `Findings` first).
  *(D21-41 e)*
- **A foreign-worktree tier-1 write is refused, typed, with ownership read from the store.** *(8d; AC-529)*
- **The gate's transitions are append-only rows under schema version 2, and the shipped store stamps 2.** A
  checked-in version-1 store is refused with its bytes unchanged. *(R2; AC-180)*
- **The interim `item` key moves no golden log's canonical digest.** *(N5; R3)*
- **The anchored differential holds**, and the whole suite, fmt and clippy are green. *(8c)*
- **Checkpoint 8d-ii (the entry gate) re-runs whole. So do the five checkpoints this stage's edits make provisional:
  7a (the port surface), 7h, 7f and 8a (all three amended), and 7i (the shipped store it reads moved to schema
  version 2).** `s8` (2) re-runs whole with the store restored byte-for-byte around its `doctor --rebuild`. That
  block runs the kernel half (`rebuild_differential`, `stage_level_next_action`) and AC-427's operator path; D21-42
  8c's re-measurement is `doctor::ac184_…`, selected in (2). So does `s8` (3)'s asserted half, the kill-and-resume
  property the seed change touches. ~~Nothing older is re-run: 7b, 7c and
  7g are not this stage's provisional set.~~ *(dated 2026-09-28, s8e build, package R2FIX-8e; refuting round 2's two
  majors.)* Two more checkpoints are provisional by this stage's edits, and **Checkpoint 8e re-runs neither**; each is
  re-run whole by the stage that already owns its re-run. **Checkpoint 7c:** this stage's three kinds (ADR-0185,
  `proposed`) move `EventKind::ALL` from 38 to 41, so 7c's guard over `pub const ALL: [Self; 38]` is amended by dated
  edit to 41 (D21-47's time-bound rule; §8e.13, finding 24). Its whole re-run is `s8j`'s (*"The whole of 7c still
  runs"*, `s8j` §8j.6, split under D21-47), because 7c's live-probe half cannot run here. **Checkpoint 8d:** package
  S8E-RULINGS edited its `run_named rk-kernel "on_false, telemetry, breaker"` list by dated edit under D21-92
  (§8e.13, finding 25). Under `RK_CP_NESTED=1`, (10a)'s 8d-ii replay skips its own nested 8d re-run, so no block on
  this stage's chain runs the edited text; `s8f`'s Checkpoint 8f does, whole: *"It also re-runs two checkpoints this
  stage makes provisional. Checkpoint 8d is one"*, and its block carries `replay_whole
  runbook/phase-1/s8d-step-grain-and-resolver.md CP8D …` (`s8f` §8f.13, its (9)). No 8d replay is added to Checkpoint 8e; the two D21-92 names
  run in this block's own (2) selection (`sel rk-kernel 28`). 7b and 7g stay outside the provisional set.
  *(dated 2026-09-28, s8e build, package R3FIX-8e; refuting round 3's crossfile minor.)* **Checkpoint 5 is
  provisional too, and Checkpoint 8e re-runs it whole** (its (10i)): this stage moves `crates/rk-types`
  (ADR-0185's three kinds and ADR-0182's interim item key, both `proposed`; `git diff --stat 766854c HEAD --
  crates/rk-types` reads four files, 189 insertions, 13 deletions at `259f714`), and `s8d`'s precedent is a
  conditional replay, not a mark: *"Checkpoint 5 re-runs when `crates/rk-types` differs from the commit this stage
  was entered at"* (`s8d` §8d.11). (10i) decides with `git diff --quiet 766854c -- crates/rk-types` against a root-commit
  control, extracts Checkpoint 5's block by its `deny.toml` line, appends an end marker, and brackets the replay with
  the advisory databases' commits, as Checkpoint 8d (13) does. `PROGRESS.md` §1's row 5 reads re-run pending until it
  passes. The block skips all of these nested re-runs when it is itself run with
  `RK_CP_NESTED=1`, D21-46's one convention (*"running as another checkpoint's entry gate: skip your own nested whole
  re-runs"*). It sets the same variable for the checkpoints it calls. A checkpoint that does not honour the variable
  still runs correctly, only slower.
- **No replay and no cargo run of this block can reach a real `claude`** (D21-47). The block sources the shared
  wrapper, `runbook/tools/nodispatch.sh`, and every one runs through `nd`, which is `nodispatch` with
  `ANTHROPIC_API_KEY` and `CLAUDE_CODE_OAUTH_TOKEN` also unset. That is a compiled fake `claude` first on `PATH` that
  prints the pin for `--version` and logs and refuses anything else, every other `PATH` directory holding a `claude`
  removed, `CLAUDE_CONFIG_DIR` at an empty trap directory, and `REDKILN_LAUNCH_RECORD` unset, so `s8h`'s gate refuses
  before spawn once it exists. `nd_check` (the trap still empty, no fake call logged) passes after the cargo runs,
  after each replay and at the block's end. The operator's own `CLAUDE_CONFIG_DIR` is the real account, which is why
  the stage's own cargo runs are wrapped too. *(final consistency pass 2026-09-24)*
- **`rk-run/rk-fix-doctor-test-count` and its legacy mark are unchanged**, so 7h and 7i stay runnable. **No
  `rk-run/` ref moved and the worktree path set is unchanged across the whole block**, so no lane ran in this
  repository, even one whose tree was then removed.

**BEFORE (PIA-035): on the entry tree these lines FAIL:**
- (2)'s selections, because the tests do not exist;
- (3), because the ARMED string is present;
- (4), because of `conn.rs`'s `into_inner`;
- (5), because the port has four methods and both SQLite registry rows are probe-less;
- (6), because nothing calls the writer;
- (7)'s `rk-cli` `INSERT` line, because of `rebuild.rs`, and its `verbs::run::run(` floor, because no test calls it;
- (7b), because no `0002` migration exists, no fixture store is checked in, and the shipped store stamps schema
  version 1;
- (7c), because 7h's backtick is present.

A block that passed on `c5d19d7` would be asserting nothing.

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
# The run-worktree floors, the ignore row and the worst-case path are Checkpoint 8d-ii's; (10a) re-runs it whole.
CODE_ONLY='^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'

# ---- (0w) THE NO-DISPATCH WRAPPER (D21-47), set up before the first cargo run, because CLAUDE_CONFIG_DIR in the
# ---- operator's session is the operator's REAL account. The shared helper, runbook/tools/nodispatch.sh: nd_setup
# ---- compiles a fake claude.exe first on PATH (the pin for --version; anything else is logged and exits 97), removes
# ---- every other PATH directory holding a claude, unsets REDKILN_LAUNCH_RECORD, points CLAUDE_CONFIG_DIR at an empty
# ---- trap, and runs its own controls; inside an enclosing wrapper it verifies and reuses that one. `nd` adds this
# ---- stage's two extra unsets. Every cargo run and every replay below goes through `nd`, and nd_check follows each.
. runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh could not be sourced"; exit 1; }
nd_setup "$PWD/target/s8e/nd" || exit 1
nd() { nodispatch env -u ANTHROPIC_API_KEY -u CLAUDE_CODE_OAUTH_TOKEN "$@"; }
# CONTROL: this stage's extra unset reaches the child, beside the helper's launch-record unset and its trap.
got=$(ANTHROPIC_API_KEY=s8e-control nd sh -c 'printf "%s|%s|%s" "${ANTHROPIC_API_KEY-unset}" "${REDKILN_LAUNCH_RECORD-unset}" "$CLAUDE_CONFIG_DIR"'); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = "unset|unset|$ND_TRAP_NATIVE" ] || { echo "FAIL: CONTROL — the wrapper's child saw '$got' (rc=$rc), not 'unset|unset|$ND_TRAP_NATIVE'"; exit 1; }

# ---- (0) BEFORE READINGS, taken before any cargo run and before any prune.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
rw_before=$(ls -A .redkiln/worktrees 2>/dev/null)
ref_before=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is absent — Checkpoints 7h and 7i read it (see this file's section 8e.13)"; exit 1; }
case "$ref_before" in
  73c087a*) ;;
  *) echo "FAIL: rk-run/rk-fix-doctor-test-count is at $ref_before, not 73c087a — something ran the untouched fixture item (D21-42 18) and 7h guards (2)/(3) will not hold"; exit 1 ;;
esac
legacy=$(git log -1 --format='%(trailers:key=Story,valueonly=true)' "$ref_before"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git could not read the legacy mark (rc=$rc)"; exit 1; }
legacy=$(printf '%s\n' "$legacy" | grep .)
[ "$legacy" = "redkiln-rs/rk-fix-doctor-test-count" ] || { echo "FAIL: the one real Story: mark reads '$legacy', not the stage-less legacy value this stage admits read-only"; exit 1; }
# Every rk-run/ ref and its sha: a lane that ran here and then removed its tree still leaves a ref behind (see (11)).
runs_before=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
[ "$rc" -eq 0 ] && [ -n "$runs_before" ] || { echo "FAIL: git for-each-ref over refs/heads/rk-run/ rc=$rc or empty — the legacy ref alone makes it non-empty"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: the node fixture has no node_modules — run pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project first, or 10 node tests fail for a reason unrelated to this stage"; exit 1; }

# ---- (1) BUILD-OR-FAIL, before any binary is read (Halt 1).
out=$(nd cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8e NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists — every binary-arm test below would be about nothing"; exit 1; }
nd_check "build" || exit 1

# ---- (2) THE NAMED TESTS, per crate, each selected exactly. `sel <package> <count> <names…>`.
sel() {
  pkg=$1; want=$2; shift 2
  [ "$#" -eq "$want" ] || { echo "FAIL: the $pkg list names $# tests and its count says $want — the list and the assertion disagree"; exit 1; }
  out=$(nd cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted, still ignored, or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed; 0 ignored" || { echo "FAIL: the $pkg selection did not run exactly $want tests with none ignored"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
# CONTROL: a filter that matches nothing EXITS 0. This is why `sel` asserts each ok line and an exact count.
out=$(nd cargo test -p rk-git --test it -- trailers::s8e_control_no_such_test --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the zero-match control did not run (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed' || { echo "FAIL: CONTROL — a zero-match filter did not report 0 passed, so this block cannot tell it from a real run"; exit 1; }

sel rk-git 11 trailers::story_value_with_a_stage_parses_as_a_stage_bearing_mark trailers::the_stage_less_legacy_mark_parses_and_carries_no_stage trailers::a_story_value_with_three_tokens_is_malformed_drift trailers::the_widened_gate_value_carries_kind_reach_and_state trailers::a_gate_value_without_kind_reach_and_state_is_legacy_drift trailers::reconcile_folds_the_latest_stage_mark_per_item trailers::a_repeated_identical_stage_mark_is_duplicate_story_drift trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage trailers::the_closed_mark_parses_as_the_lanes_end trailers::reconcile_folds_a_gates_latest_state_per_token trailers::an_rfc3339_gate_deadline_is_malformed_drift
# (dated 2026-09-28, s8e build, package D2196-8e) sel rk-store 7, was 6 (struck: sel rk-store 6): D21-96's
# store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening is named.
sel rk-store 7 store::writer_poison_is_a_halt_and_never_recovered store::tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree store::t1_gate_transition_appends_one_row_per_transition_and_is_never_updated store::migration_0002_adds_the_transitions_table_and_the_item_column_and_stamps_version_two store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged tier_ui::a_state_field_without_a_tier_is_a_compile_error store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening
sel rk-ports 3 event_store_port::ac364_event_store_port_has_no_update_or_delete fake_behaviour::vec_store_append_tier1_is_queryable_with_its_anchor fake_behaviour::vec_store_query_by_item_returns_only_that_items_rows
sel rk-adapter-sqlite 6 conformance::sqlite_store_conforms_including_the_tier_one_arm store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows store::query_by_item_returns_only_that_items_rows registry::ac489_capability_registry_dated registry::snapshot_and_rebuild_rows_carry_a_recorded_probe
# (dated 2026-09-28, s8e build, package S8E-P6) sel rk-kernel 25, was 24 (struck: sel rk-kernel 24): the heal's
# closed arm (section 8e.6) has its own test, tier_one::a_closed_mark_heals_as_the_lanes_end_transition.
# (dated 2026-09-28, s8e build, package S8E-P7) sel rk-kernel 26, was 25 (struck: sel rk-kernel 25): D21-93's
# fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition is named.
# (dated 2026-09-28, s8e build, package S8E-RULINGS) sel rk-kernel 28, was 26 (struck: sel rk-kernel 26): D21-92's
# two on_false_exec tests are named (ADR-0187, proposed).
sel rk-kernel 28 on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes fsm::an_approved_seal_reads_sealed_only_until_the_next_stage_transition tier_one::every_run_commit_resolves_through_the_worktree_resolver projection::stage_level_next_action tier_one::a_failed_commit_leaves_zero_store_rows tier_one::a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit tier_one::the_commit_exists_before_any_row_is_readable tier_one::the_tier_one_writer_is_called_only_from_the_boundary tier_one::replay_of_a_boundary_performs_zero_git_writes tier_one::a_commit_without_a_row_is_healed_at_entry tier_one::a_heal_writes_no_row_for_an_unreachable_anchor tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint tier_one::a_gate_answer_is_committed_before_its_row tier_one::a_gate_expiry_is_committed_before_its_row tier_one::a_gate_void_is_committed_before_its_row tier_one::a_boundary_anchors_every_row_of_the_act_to_its_commit tier_one::an_act_with_neither_trailer_nor_content_is_refused tier_one::the_boundary_journals_a_git_op_row_per_invocation tier_one::replay_serves_the_boundary_from_its_git_op_rows tier_one::a_journal_without_git_op_rows_replays_as_no_git_writes conformance::conformance_every_trailer_key_has_a_producer projection::rebuild_differential lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again lane_order::a_pack_stage_named_closed_is_refused_at_load fsm::every_item_scoped_kind_carries_the_interim_item_key replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key tier_one::a_closed_mark_heals_as_the_lanes_end_transition
# (dated 2026-09-28, s8e build, package S8E-P2) sel rk-cli 32, was 30 (struck: sel rk-cli 30): section 8e.11's two
# owed-in lane_e2e tests are named (MF3j; the review-routes binding).
# (dated 2026-09-28, s8e build, package S8E-D2194-ENTRY) sel rk-cli 35, was 32 (struck: sel rk-cli 32): D21-94's
# entry refusal (as D21-95 rules it), its control, and the mint's backstop pin are named (ADR-0186 amended, proposed).
# (dated 2026-09-28, s8e build, package R1FIX-8e) sel rk-cli 37, was 35 (struck: sel rk-cli 35): refuting round 1's
# answer-selection test and the entry check's forbidden-stage pin are named.
# (dated 2026-09-28, s8e build, package R3FIX-8e) sel rk-cli 38, was 37 (struck: sel rk-cli 37): S8E-M43c's only
# killer, answer::ac248_answer_without_live_token_exits_named_outcome (re-pointed by R1FIX-8e, not new), is named.
sel rk-cli 38 answer::ac248_answer_without_live_token_exits_named_outcome answer::answer_presents_the_awaited_gate_over_a_spent_token_that_sorts_higher lane_e2e::the_entry_check_skips_a_forbidden_current_stage_with_a_findings_activity lane_e2e::an_undeclared_slice_is_refused_at_entry_before_any_dispatch lane_e2e::a_lane_that_reaches_no_findings_stage_is_not_refused_for_an_undeclared_slice slice_verdict::the_mint_refuses_an_undeclared_slice_as_the_backstop lane_e2e::a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit worktree_assigned::a_reentry_records_reused_and_its_discard_list worktree_assigned::answer_refuses_fail_closed_without_an_assignment_row lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order lane_e2e::an_empty_stage_records_stage_empty_with_its_reason lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted lane_e2e::terminal_read_reports_only_an_awaited_gate lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child lane_e2e::the_lane_end_mints_a_terminal_closed_mark lane_e2e::a_closed_item_reruns_as_done_and_mints_nothing lane_e2e::an_answered_gate_survives_a_deleted_store_as_answered lane_e2e::two_items_in_one_store_do_not_fold_together lane_e2e::the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it lane_e2e::a_second_invocation_reuses_the_run_worktree slice_verdict::approved_is_sealed_on_the_advance_path slice_verdict::changes_requested_is_sealed_on_the_findings_block_path slice_verdict::an_absent_slice_is_refused_by_name slice_verdict::a_slice_equal_to_the_item_id_is_refused worktree_assigned::a_tier1_write_from_a_foreign_worktree_is_refused_typed worktree_assigned::the_positive_control_from_the_run_owned_worktree_succeeds worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention answer::a_failed_answer_commit_leaves_zero_rows doctor::ac184_rebuild_differential_over_anchored_tier_one_rows doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record doctor::a_version_one_store_is_refused_by_name_and_rebuild_re_derives_it_at_version_two root::adr0118_a_poisoned_writer_mutex_halts_exit_70 lane_e2e::run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer lane_e2e::run_binds_the_review_routes_for_a_shallow_clone_step
nd_check "named selections" || exit 1

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
# Capability::Rebuild is implemented (D21-42 8c; section 8e.9): no row of the SQLite registry is probe-less. A floor;
# (2)'s two registry tests are the assertion.
reg=crates/rk-adapter-sqlite/src/registry.rs
test -f "$reg" || { echo "FAIL: the SQLite registry file is absent (Halt 8)"; exit 1; }
# (dated 2026-09-28, s8e build, package R3FIX-8e; refuting round 3) The count reads CODE lines only and is
# whitespace-robust: the tag, any spacing, a colon, any spacing, a brace. It replaced (struck) a grep -cF on the exact
# bytes of the tag, colon, one space and brace, which a compact or space-before-colon row passed and which a doc line
# could satisfy.
NONE_TAG='"none"[[:space:]]*:[[:space:]]*[{]'
raw=$(grep -nHE "$NONE_TAG" "$reg"); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the probe-less search could not run (grep rc=$rc)"; exit 1; }
n=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -c .)
[ "$n" -eq 0 ] || { echo "FAIL: the SQLite registry still declares $n probe-less row(s) — snapshot and rebuild flip together (section 8e.9):"; printf '%s\n' "$raw" | grep -vE "$CODE_ONLY"; exit 1; }
# CONTROL: a compact row and a space-before-colon row are COUNTED, and a doc line naming the tag is DROPPED.
ctl=$(printf '%s\n' '    "probe": {"none":{"reason": "x"}}' '    "probe": { "none" : { "reason": "x" } }' '//! `{"none": {"reason": …}}` names the tag' | grep -nHE "$NONE_TAG" --label="$reg" - | grep -vE "$CODE_ONLY" | grep -c .)
[ "$ctl" -eq 2 ] || { echo "FAIL: CONTROL — the probe-less count kept $ctl of two rows and one doc line"; exit 1; }

# ---- (6) AC-162's REACHABILITY FLOOR: every use of the writer, in EVERY call form (a method call, a path call such as
# ---- EventStore::append_tier1(...), a bare path handed on as a function value), is in the kernel's boundary module.
# ---- Definitions are dropped. The one allow-listed caller is rk-ports' conformance arm, behind the `conformance`
# ---- feature and called only from the two conformance rows under tests/ (section 8e.3). The lint test in (2) is the
# ---- structural assertion and the two fault injections are the criterion; this names a stray caller by line.
DEFN='^[^:]+:[0-9]+:.*fn[[:space:]]+append_tier1'
ALLOWED='^crates/rk-kernel/src/runtime/tier_one\.rs:|^crates/rk-ports/src/conformance\.rs:'
raw=$(grep -rnw 'append_tier1' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the reachability search could not run (grep rc=$rc)"; exit 1; }
calls=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -vE "$DEFN" | grep .)
[ -n "$calls" ] || { echo "FAIL: no code line uses append_tier1 — a writer nothing calls is the s7g lesson again"; exit 1; }
stray=$(printf '%s\n' "$calls" | grep -vE "$ALLOWED")
[ -z "$stray" ] || { echo "FAIL: append_tier1 is used outside the kernel's tier-one boundary and the conformance arm:"; printf '%s\n' "$stray"; exit 1; }
inb=$(printf '%s\n' "$calls" | grep -c '^crates/rk-kernel/src/runtime/tier_one\.rs:')
[ "$inb" -ge 1 ] || { echo "FAIL: the only uses of append_tier1 are the conformance arm's — the boundary never writes"; exit 1; }
# CONTROL: the filter KEEPS a path-form call and a function-value use outside the boundary, and DROPS a definition and a comment.
ctl=$(printf '%s\n' 'crates/rk-cli/src/x.rs:3:    EventStore::append_tier1(&mut *s, e, &a)?;' 'crates/rk-cli/src/x.rs:4:    acts.map(SqliteStore::append_tier1);' 'crates/rk-ports/src/event_store.rs:60:    fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError>;' 'crates/rk-cli/src/x.rs:5:    // append_tier1 belongs to the kernel' | grep -vE "$CODE_ONLY" | grep -vE "$DEFN" | grep -vE "$ALLOWED" | grep -c .)
[ "$ctl" -eq 2 ] || { echo "FAIL: CONTROL — the reachability filter kept $ctl of two stray uses, one definition and one comment"; exit 1; }
# rk-store's tier_one statements are named only inside rk-store and rk-adapter-sqlite (the adapter's append_tier1 and
# rebuild, section 8e.3): the rebuild's exemption from the lint above is not a second route in from rk-cli.
raw=$(grep -rnE 'rk_store::(\{[^}]*)?tier_one' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk_store::tier_one search could not run (grep rc=$rc)"; exit 1; }
stray=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -vE '^crates/(rk-store|rk-adapter-sqlite)/src/' | grep .)
[ -z "$stray" ] || { echo "FAIL: a code line outside rk-store and rk-adapter-sqlite names rk-store's tier_one statements:"; printf '%s\n' "$stray"; exit 1; }

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
# The lane loop has tests: a code line under crates/*/tests calls verbs::run::run( (a floor beneath section 8e.11; ADR-0161).
raw=$(grep -rnF 'verbs::run::run(' crates/*/tests --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the verbs::run::run( search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -n "$code" ] || { echo "FAIL: no code line under crates/*/tests calls verbs::run::run( — the lane loop is still untested (ADR-0161)"; exit 1; }

# ---- (7b) R2: THE FORWARD MIGRATION EXISTS AS A PAIR, NOTHING UPDATES A GATE ROW, AND THE SHIPPED STORE STAMPS 2.
# ---- Floors beneath section 8e.12's migration and rebuild tests (D21-46 R2; AC-180).
mig=crates/rk-store/src/migrations/0002_gate_transitions
test -f "$mig.sql" && test -f "$mig.keys" || { echo "FAIL: the 0002 migration is not a .sql/.keys pair — the schema lint's form (D21-46 R2)"; exit 1; }
# SQL CODE lines only: a `--` comment naming the table must not satisfy either line.
SQLCOMMENT='^[[:space:]]*--'
CREATE_T='CREATE[[:space:]]+TABLE[[:space:]]+(IF[[:space:]]+NOT[[:space:]]+EXISTS[[:space:]]+)?t1_gate_transition'
sqlcode=$(grep -vE "$SQLCOMMENT" "$mig.sql"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: 0002 has no SQL code line (grep rc=$rc)"; exit 1; }
printf '%s\n' "$sqlcode" | grep -qiE "$CREATE_T" || { echo "FAIL: no SQL code line in 0002 creates t1_gate_transition"; exit 1; }
printf '%s\n' "$sqlcode" | grep -qiE 'DROP[[:space:]]+TABLE[[:space:]]+(IF[[:space:]]+EXISTS[[:space:]]+)?t1_gate([[:space:]]|;|$)' || { echo "FAIL: no SQL code line in 0002 drops t1_gate"; exit 1; }
# CONTROL: a comment naming the CREATE does not satisfy the code-line reading.
ctl=$(printf '%s\n' '-- CREATE TABLE t1_gate_transition is below' | grep -vE "$SQLCOMMENT" | grep -ciE "$CREATE_T")
[ "$ctl" -eq 0 ] || { echo "FAIL: CONTROL — an SQL comment satisfied the CREATE TABLE reading"; exit 1; }
# AC-180's checked-in version-1 fixture store (section 8e.2): tracked, pinned binary, and stamping version 1.
# Read with immutable=1, because the file is checked in and a reader must not touch it.
fx=crates/rk-store/tests/fixtures/store-v1.sqlite3
git ls-files --error-unmatch "$fx" >/dev/null 2>&1 || { echo "FAIL: AC-180's checked-in version-1 fixture store $fx is not tracked"; exit 1; }
attr=$(git check-attr binary -- "$fx"); rc=$?
[ "$rc" -eq 0 ] && [ "$attr" = "$fx: binary: set" ] || { echo "FAIL: $fx is not pinned binary in .gitattributes (read '$attr', rc=$rc) — autocrlf could smudge it"; exit 1; }
fxv=$(python -c 'import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?immutable=1",uri=True); print(c.execute("SELECT schema_version FROM rk_identity WHERE id = 1").fetchone()[0])' "$fx"); rc=$?
[ "$rc" -eq 0 ] && [ "$fxv" = "1" ] || { echo "FAIL: the AC-180 fixture stamps schema version '$fxv' (rc=$rc), not 1 — it is not version N-1"; exit 1; }
grep -qE '^t1_gate_transition\.anchor_sha[[:space:]]*=[[:space:]]*sha' "$mig.keys" || { echo "FAIL: 0002's .keys does not declare t1_gate_transition.anchor_sha = sha (AC-166)"; exit 1; }
raw=$(grep -rniE 'UPDATE[[:space:]]+t1_' crates/rk-store/src --include=*.rs --include=*.sql); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the UPDATE search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*--' | grep .)
[ -z "$code" ] || { echo "FAIL: a code line UPDATEs a t1_ table — the gate's lifecycle is append-only (N6; AC-364):"; printf '%s\n' "$code"; exit 1; }
# CONTROL: the filter KEEPS an UPDATE statement and DROPS an SQL comment naming one.
ctl=$(printf '%s\n' 'crates/rk-store/src/tier_one.rs:4:    tx.execute("UPDATE t1_gate SET answered = 1", [])?;' 'crates/rk-store/src/migrations/0002_gate_transitions.sql:2:-- no UPDATE t1_ ever' | grep -vE "$CODE_ONLY" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*--' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the UPDATE filter kept $ctl of one statement and one SQL comment"; exit 1; }
# The shipped store stamps schema version 2 (read-only; mode=ro, not immutable=1, so the WAL is read).
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store"; exit 1; }
ver=$(python -c 'import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(c.execute("SELECT schema_version FROM rk_identity WHERE id = 1").fetchone()[0])' "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$ver" = "2" ] || { echo "FAIL: the shipped store stamps schema version '$ver' (rc=$rc), not 2 — R2's explicit rebuild was not run in the migration's commit"; exit 1; }

# ---- (7c) CHECKPOINT 7h's (10a) FAIL MESSAGE NO LONGER EXECUTES `redkiln run` (section 8e.13). The quote-aware scan
# ---- over every re-run block is in (10); this is the named defect, by its exact bytes.
test -f runbook/phase-1/s7h-lane-completes.md || { echo "FAIL: s7h's file is absent (Halt 8)"; exit 1; }
hits=$(grep -nF 'a real `redkiln run` over the review fixture' runbook/phase-1/s7h-lane-completes.md); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: 7h guard (10a)'s FAIL message still carries backticks inside double quotes (grep rc=$rc):"; printf '%s\n' "$hits"; exit 1; }

# ---- (8) THE WHOLE SUITE, no fail-fast, with a floor. Expected value, re-derived at stage open from Checkpoint 8d-ii's
# ---- recorded count plus 74 (section 8e.0 item 7): ~~964 + 74 = 1038~~ on the final chain (D21-48 floor propagation,
# ---- 2026-09-24: was 947 + 74 = 1021, before s8c 78, s8d 62 and s8d-ii 50). Replace it by a dated edit
# ---- before the first run.
# ---- (dated 2026-09-28, s8e build) RE-DERIVED: Checkpoint 8d-ii's RECORDED count 1034 (PROGRESS.md row 8d-ii) + 74
# ---- = 1108, replacing the chain's 1038 (struck: FLOOR=1038). The owed-in MF3j and review-routes lane_e2e test(s)
# ---- (section 8e.11's receiver notes) are OUTSIDE the 74; this line and sel rk-cli 30 move again when they are named.
# ---- (dated 2026-09-28, s8e build, package S8E-P2) NAMED: both owed-in tests, so 1034 + 76 = 1110 (struck:
# ---- FLOOR=1108), and (2) reads sel rk-cli 32.
# ---- (dated 2026-09-28, s8e build, package S8E-P6) one more rk-kernel test (tier_one::a_closed_mark_heals_as_the_lanes_end_transition),
# ---- so 1034 + 77 = 1111 (struck: FLOOR=1110), and (2) reads sel rk-kernel 25.
# ---- (dated 2026-09-28, s8e build, package S8E-P7) D21-93's fsm test, so 1034 + 78 = 1112 (struck: FLOOR=1111), and (2)
# ---- reads sel rk-kernel 26.
# ---- (dated 2026-09-28, s8e build, package S8E-RULINGS) D21-92's two on_false_exec tests are new and s8d's
# ---- a_null_until_or_gate_when_is_not_journaled is replaced, so 1034 + 79 = 1113 (struck: FLOOR=1112), and (2)
# ---- reads sel rk-kernel 28.
# ---- (dated 2026-09-28, s8e build, package S8E-D2194-ENTRY) D21-94's entry refusal adds three rk-cli tests, so
# ---- 1034 + 82 = 1116 (struck: FLOOR=1113), and (2) reads sel rk-cli 35.
# ---- (dated 2026-09-28, s8e build, package R1FIX-8e) refuting round 1 adds two rk-cli tests, so 1034 + 84 = 1118
# ---- (struck: FLOOR=1116), and (2) reads sel rk-cli 37.
# ---- (dated 2026-09-28, s8e build, package D2196-8e) D21-96 adds one rk-store test, so 1034 + 85 = 1119
# ---- (struck: FLOOR=1118), and (2) reads sel rk-store 7.
FLOOR=1119
tally() { printf '%s\n' "$1" | awk '/^test result:/ { p += $4; f += $6; i += $8; n += 1 } END { print p+0, f+0, i+0, n+0 }'; }
# CONTROL: a run of zero tests tallies below the floor.
read -r cp cf ci cn <<< "$(tally 'test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out')"
[ "$cp" -lt "$FLOOR" ] || { echo "FAIL: CONTROL — a zero-test run tallied $cp passed, not below the floor $FLOOR"; exit 1; }
out=$(nd cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
read -r passed failed ignored results <<< "$(tally "$out")"
[ "$failed" -eq 0 ] && [ "$passed" -ge "$FLOOR" ] && [ "$results" -ge 44 ] || { echo "FAIL: the suite read passed=$passed failed=$failed ignored=$ignored over $results results — the floor is $FLOOR passed over at least 44 results"; exit 1; }
echo "whole suite: passed=$passed failed=$failed ignored=$ignored results=$results (floor $FLOOR) — record these in PROGRESS.md"
out=$(nd cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nd cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }
nd_check "whole suite, fmt and clippy" || exit 1

# ---- (9) THE WORKTREE PATH SET, compared BEFORE the re-run blocks, because Checkpoint 7h prunes.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

# ---- (10) RE-RUN FROM THEIR OWN BYTES: the entry gate (8d-ii) and the five checkpoints this stage's edits make
# ---- provisional (7a, 7h, 7f and 8a, all amended or re-surfaced, and 7i, whose shipped store R2 rebuilt), plus s8 (2)
# ---- (the kernel half and AC-427's operator path, around a rebuild of the shipped store) and s8 (3)'s asserted half
# ---- (the seed change). Nothing older. SKIPPED WHOLE under RK_CP_NESTED=1 (D21-46: running as another checkpoint's
# ---- entry gate), and set for every block this one calls. Every replay runs through (0w)'s `nd` wrapper (D21-47),
# ---- and nd_check follows it.
# ---- (dated 2026-09-28, s8e build, package R2FIX-8e) Two more are provisional and NOT replayed here (section 8e.15):
# ---- 7c (its ALL-count guard amended 38 -> 41; its whole re-run is s8j's) and 8d (S8E-RULINGS's run_named edit;
# ---- s8f's Checkpoint 8f (9) re-runs it whole). This block's replay set is unchanged.
# ---- (dated 2026-09-28, s8e build, package R3FIX-8e) One more joins the replay set: Checkpoint 5, whole, when
# ---- crates/rk-types moved since the entry commit (10i), on Checkpoint 8d (13)'s precedent (section 8e.15).
# ---- The extractor takes the ONE bash block of a file that contains a needle, and refuses on zero or two. The fence
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

if [ "${RK_CP_NESTED:-0}" = "1" ]; then
  echo "CP8E: nested re-runs skipped — RK_CP_NESTED=1 was set by the checkpoint re-running this one as its entry gate"
else
  # The trap and the fake claude are (0w)'s; every replay below goes through `nd`.
  # (10a) CHECKPOINT 8d-ii, the entry gate, whole. Under RK_CP_NESTED=1 it skips its own nested 8d and 7f re-runs.
  ext runbook/phase-1/s8d-ii-run-worktrees.md 'echo "CP8D-II-END-OF-BLOCK reached"' "$scratch/cp8dii.sh"
  out=$(nd env RK_CP_NESTED=1 bash "$scratch/cp8dii.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8d-ii, re-run whole, exited $rc — this stage's entry gate no longer holds"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8D-II-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8d-ii exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 8d-ii replay" || exit 1

  # (10b) CHECKPOINT 7a whole — the gate on the port surface this stage changed. Its block has no end marker, so one is appended.
  ext runbook/phase-1/s7a-ports-and-fakes.md 'pub store: Arc<Mutex<dyn EventStore>>' "$scratch/cp7a.sh"
  printf '\necho "CP7A-EXTRACTED-END reached"\n' >> "$scratch/cp7a.sh" || { echo "FAIL: the 7a end marker could not be appended"; exit 1; }
  out=$(nd env RK_CP_NESTED=1 bash "$scratch/cp7a.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7A-EXTRACTED-END reached' || { echo "FAIL: Checkpoint 7a stopped before its last line"; exit 1; }
  nd_check "Checkpoint 7a replay" || exit 1

  # (10c) CHECKPOINT 7h whole, AS AMENDED BY THIS STAGE (section 8e.13). Its guards (6)/(7) are the discharge's
  # witnesses, and ext() has already refused a block that still carries (10a)'s backtick.
  ext runbook/phase-1/s7h-lane-completes.md 'echo "CP7H-END-OF-BLOCK reached"' "$scratch/cp7h.sh"
  out=$(nd env RK_CP_NESTED=1 FIX_ITEM=rk-fix-doctor-test-count bash "$scratch/cp7h.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7h, re-run whole, exited $rc — if guard (6) or (7) names ARMED, the dated amendment did not land"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7H-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7h exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 7h replay" || exit 1

  # (10d) CHECKPOINT 7f whole, AS AMENDED (section 8e.13 and s8d-ii's 8d-ii.7): its run arm is in a throwaway clone, and it can reach no
  # dispatch. Refused outright once a shipped card carries permission_mode (s8h has landed; no-accidental-dispatch).
  cards=$(ls .redkiln/cards/*.json 2>/dev/null)
  [ -n "$cards" ] || { echo "FAIL: no shipped card under .redkiln/cards — the dispatch precondition would be about nothing (Halt 8)"; exit 1; }
  raw=$(grep -l 'permission_mode' .redkiln/cards/*.json); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: a shipped card carries permission_mode (grep rc=$rc) — 7f's run arm could reach a real dispatch; re-run 7f only through s8h's fixture-child route, not from this block"; printf '%s\n' "$raw"; exit 1; }
  ext runbook/phase-1/s7f-conformance-and-records.md 'echo "CP7F-END-OF-BLOCK reached"' "$scratch/cp7f.sh"
  # The clone is read from CODE lines: a comment naming a clone does not re-point the arm.
  SHCOMMENT='^[[:space:]]*#'
  CLONE='(^|[^[:alnum:]_./-])git[[:space:]]([^#]*[[:space:]])?clone([[:space:]]|$)'
  code7f=$(grep -vE "$SHCOMMENT" "$scratch/cp7f.sh"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7f's extracted block has no code line (grep rc=$rc)"; exit 1; }
  printf '%s\n' "$code7f" | grep -qE "$CLONE" || { echo "FAIL: no CODE line of Checkpoint 7f runs git clone — its run arm is not re-pointed to a throwaway clone, and run here it would advance the untouched fixture item (D21-42 18)"; exit 1; }
  # CONTROL: a comment-only clone does not satisfy the reading, and a code-line clone does.
  ctl=$(printf '%s\n' '  # the arm runs in a git clone of HEAD' | grep -vE "$SHCOMMENT" | grep -cE "$CLONE")
  [ "$ctl" -eq 0 ] || { echo "FAIL: CONTROL — a comment naming a git clone satisfied the code-line reading"; exit 1; }
  ctl=$(printf '%s\n' '  git clone -q . "$clone" || exit 1' | grep -vE "$SHCOMMENT" | grep -cE "$CLONE")
  [ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — a code-line git clone was not recognised"; exit 1; }
  hits=$(grep -nF 'worktree remove --force .git/redkiln/worktrees/' "$scratch/cp7f.sh"); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: Checkpoint 7f still force-removes the legacy run tree (grep rc=$rc) — D21-44 keeps it for s8j:"; printf '%s\n' "$hits"; exit 1; }
  # BEHAVIOURAL: the run executes in the clone, so no rk-run/ ref and no worktree path of THIS repository moves.
  r7f_before=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$r7f_before" ] || { echo "FAIL: could not read the rk-run/ refs before 7f (rc=$rc)"; exit 1; }
  w7f_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
  out=$(nd env RK_CP_NESTED=1 bash "$scratch/cp7f.sh" 2>&1); rc=$?
  r7f_after=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/)
  w7f_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
  [ "$r7f_after" = "$r7f_before" ] || { echo "FAIL: Checkpoint 7f moved an rk-run/ ref of THIS repository — its run did not execute under the clone:"; diff <(printf '%s\n' "$r7f_before") <(printf '%s\n' "$r7f_after"); exit 1; }
  [ "$w7f_after" = "$w7f_before" ] || { echo "FAIL: Checkpoint 7f changed this repository's worktree path set — its run did not execute under the clone:"; diff <(printf '%s\n' "$w7f_before") <(printf '%s\n' "$w7f_after"); exit 1; }
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7f, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7F-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7f exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 7f replay" || exit 1

  # (10h) CHECKPOINT 8a whole, AS AMENDED BY THIS STAGE (section 8e.13): its guard (7) pinned the trailer lint as ignored
  # and ARMED, and this stage's Slice-Verdict commit discharged the lint. 8a carries its record ids and base commit as
  # literals in its own block (section 8e.13, finding 17), so this replay supplies nothing.
  ext runbook/phase-1/s8a-records-and-red-checkpoints.md 'echo "CP8A-END-OF-BLOCK reached"' "$scratch/cp8a.sh"
  hits=$(grep -nF 'ignored, ARMED: Slice-Verdict has 0 producers' "$scratch/cp8a.sh"); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: Checkpoint 8a guard (7) still expects the trailer lint ignored and ARMED (grep rc=$rc) — the dated amendment did not land:"; printf '%s\n' "$hits"; exit 1; }
  grep -qF 'test conformance::conformance_every_trailer_key_has_a_producer ... ok' "$scratch/cp8a.sh" || { echo "FAIL: Checkpoint 8a guard (7) does not assert the discharged lint's ok line"; exit 1; }
  out=$(nd env RK_CP_NESTED=1 bash "$scratch/cp8a.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8a, re-run whole, exited $rc — if guard (7) names ARMED, the dated amendment did not land"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8A-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8a exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 8a replay" || exit 1

  # (10e) s8 CRITERION (2) whole. Its block runs the kernel half (rebuild_differential, stage_level_next_action) and
  # AC-427's operator path, doctor --rebuild, which REPLACES the shipped store. It is NOT D21-42 8c's re-measurement:
  # that is doctor::ac184_rebuild_differential_over_anchored_tier_one_rows, selected in (2), and adding it to s8 (2)'s
  # own block is s8j's amendment. Back up the store, its WAL, its SHM and the `rebuilt` sidecar; restore them whatever
  # the block did; byte-compare; only then read its rc.
  sd=$(git rev-parse --git-common-dir); rc=$?
  [ "$rc" -eq 0 ] && [ -f "$sd/redkiln/store.sqlite3" ] || { echo "FAIL: no shipped store under $sd/redkiln (rc=$rc)"; exit 1; }
  sd="$sd/redkiln"
  for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
    if [ -f "$sd/$f" ]; then cp -p "$sd/$f" "$scratch/store-backup/$f" || { echo "FAIL: could not back up $f — criterion (2) NOT RUN"; exit 1; }; fi
  done
  ext runbook/phase-1/s8-exit-criterion.md 'echo "(2) asserted: AC-184 differential' "$scratch/cp8-2.sh"
  printf '\necho "CP8-2-EXTRACTED-END reached"\n' >> "$scratch/cp8-2.sh" || { echo "FAIL: the (2) end marker could not be appended"; exit 1; }
  out2=$(nd env RK_CP_NESTED=1 bash "$scratch/cp8-2.sh" 2>&1); rc2=$?
  for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
    rm -f "$sd/$f"
    if [ -f "$scratch/store-backup/$f" ]; then cp -p "$scratch/store-backup/$f" "$sd/$f" || { echo "FAIL: RESTORE of $f failed — restore it by hand from $scratch/store-backup before anything else runs"; exit 1; }; fi
  done
  for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
    if [ -f "$scratch/store-backup/$f" ]; then cmp -s "$scratch/store-backup/$f" "$sd/$f" || { echo "FAIL: the restored $f differs from its backup — the shipped store is not the one this block found"; exit 1; }; fi
  done
  [ "$rc2" -eq 0 ] || { echo "FAIL: s8 criterion (2), re-run whole, exited $rc2"; printf '%s\n' "$out2" | tail -30; exit 1; }
  printf '%s\n' "$out2" | grep -qxF 'CP8-2-EXTRACTED-END reached' || { echo "FAIL: s8 criterion (2) stopped before its last line"; exit 1; }
  nd_check "s8 (2) replay" || exit 1

  # (10f) s8 CRITERION (3), ASSERTED HALF — the block is run whole with ITEM UNSET. Its asserted half must pass, and it
  # must then stop at exactly one FAIL, the operational arm's `set ITEM` (D21-40's re-measurement reads the same).
  ext runbook/phase-1/s8-exit-criterion.md 'echo "(3) asserted: AC-013' "$scratch/cp8-3.sh"
  out=$(nd env -u ITEM RK_CP_NESTED=1 bash "$scratch/cp8-3.sh" 2>&1); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: s8 (3) with ITEM unset exited $rc, expected 1 at the operational arm"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qxF '(3) asserted: AC-013 N-step kill-and-resume, digest equality, zero re-dispatch' || { echo "FAIL: s8 (3)'s asserted half did not report — kill_and_resume_n_step is red or unselected"; printf '%s\n' "$out" | tail -20; exit 1; }
  fails=$(printf '%s\n' "$out" | grep '^FAIL')
  [ "$fails" = "FAIL: set ITEM to the item the run is on" ] || { echo "FAIL: s8 (3) stopped somewhere other than the operational arm:"; printf '%s\n' "$fails"; exit 1; }
  nd_check "s8 (3) replay" || exit 1

  # (10g) CHECKPOINT 7i whole — provisional because R2 rebuilt the shipped store it reads (section 8e.13). Run after
  # (10e), so it reads the restored version-2 store.
  ext runbook/phase-1/s7i-cost.md 'echo "CP7I-END-OF-BLOCK reached"' "$scratch/cp7i.sh"
  out=$(nd env RK_CP_NESTED=1 FIX_ITEM=rk-fix-doctor-test-count bash "$scratch/cp7i.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7i, re-run whole over the rebuilt shipped store, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP7I-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7i exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 7i replay" || exit 1

  # (10i) (dated 2026-09-28, s8e build, package R3FIX-8e; refuting round 3's crossfile minor, on s8c's and s8d's
  # precedent: Checkpoint 8d (13) re-runs Checkpoint 5 whole when crates/rk-types moved since its entry commit.)
  # CHECKPOINT 5, ONLY IF crates/rk-types MOVED since this stage's entry commit 766854c (Checkpoint 8d-ii's close;
  # section 8e.0's stage-open readings). This stage moves it (ADR-0185's three kinds, ADR-0182's interim item key),
  # so the arm is expected to run. Checkpoint 5's block carries no end marker; it is extracted by its deny.toml line and
  # one is appended, as (10b) does for 7a. Its cargo audit and cargo deny FETCH the advisory databases
  # run::advisory_db_row_inputs binds for sca_scan, so the replay is bracketed by their commits (8d (13)'s dbs()).
  # SYNTHETIC CONTROL first: the diff must see a change against the root commit, which carries no crates/rk-types.
  entry8e=766854c
  git cat-file -e "$entry8e^{commit}" 2>/dev/null || { echo "FAIL: the s8e entry commit $entry8e names no commit"; exit 1; }
  git merge-base --is-ancestor "$entry8e" HEAD || { echo "FAIL: HEAD does not descend from the s8e entry commit $entry8e"; exit 1; }
  root8e=$(git rev-list --max-parents=0 HEAD | tail -1)
  git diff --quiet "$root8e" -- crates/rk-types; rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: CONTROL — git diff --quiet over crates/rk-types against the root commit returned $rc, not 1"; exit 1; }
  git diff --quiet "$entry8e" -- crates/rk-types; rc=$?
  if [ "$rc" -eq 1 ]; then
    dbs() { for d in "$HOME/.cargo/advisory-db" "$HOME"/.cargo/advisory-dbs/*/; do [ -d "$d" ] || continue; h=$(git -C "$d" rev-parse HEAD 2>/dev/null); printf '%s=%s ' "$d" "${h:-none}"; done; }
    ext runbook/phase-1/s5-workspace-ci-rk-types.md 'FAIL: deny.toml absent' "$scratch/cp5.sh"
    printf '\necho "CP5-EXTRACTED-END reached"\n' >> "$scratch/cp5.sh" || { echo "FAIL: the Checkpoint 5 end marker could not be appended"; exit 1; }
    db_before_5=$(dbs)
    out=$(nd env RK_CP_NESTED=1 bash "$scratch/cp5.sh" 2>&1); rc=$?
    echo "ADVISORY-DBS before CP5: $db_before_5"
    echo "ADVISORY-DBS after CP5:  $(dbs)"
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 5, re-run whole over the moved crates/rk-types, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
    printf '%s\n' "$out" | grep -qxF 'CP5-EXTRACTED-END reached' || { echo "FAIL: Checkpoint 5 stopped before its last line"; exit 1; }
    nd_check "Checkpoint 5 replay" || exit 1
    echo "ok: Checkpoint 5 re-run whole (crates/rk-types moved since $entry8e)"
  elif [ "$rc" -eq 0 ]; then
    echo "ok: crates/rk-types unchanged since $entry8e — Checkpoint 5 is not re-run"
  else
    echo "FAIL: git diff over crates/rk-types could not run (rc=$rc)"; exit 1
  fi

fi

# ---- (10z) THE TRAP IS STILL EMPTY AND THE FAKE LOGGED NO CALL — neither this block's own cargo runs nor any replay
# ---- spawned a real claude or asked the fake for more than its version (D21-47). The helper's controls proved both
# ---- readings at nd_setup.
nd_check "end" || exit 1

# ---- (11) AFTER: the fixture item's ref and mark are untouched, and no lane ran in this repository. The tree listing
# ---- alone cannot see a lane that COMPLETED, because the closed mark removes its tree; the ref set and the worktree
# ---- path set can. The path set is comparable here because (9) found no stale registration for 7h's prune to drop.
ref_after=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ "$ref_after" = "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count moved during this block ($ref_before -> $ref_after)"; exit 1; }
runs_after=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
[ "$rc" -eq 0 ] && [ "$runs_after" = "$runs_before" ] || { echo "FAIL: the rk-run/ refs changed during this block (rc=$rc) — a lane ran against the product repository, even if its tree was then removed:"; diff <(printf '%s\n' "$runs_before") <(printf '%s\n' "$runs_after"); exit 1; }
wt_end=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_end" = "$wt_before" ] || { echo "FAIL: this block changed the worktree PATH SET by its end:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_end"); exit 1; }
rw_after=$(ls -A .redkiln/worktrees 2>/dev/null)
[ "$rw_before" = "$rw_after" ] || { echo "FAIL: .redkiln/worktrees changed during this block ('$rw_before' -> '$rw_after') — a lane ran against the product repository"; exit 1; }
rm -rf "$scratch" || { echo "FAIL: the scratch directory $scratch could not be removed"; exit 1; }

echo "CP8E-END-OF-BLOCK reached"
```

---
