# Step 8f — The landing checkpoint, rk-store's render path, and doctor over the store (Phase 1, Step 8's sub-stage f)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8f` | Phase 1 | Checkpoint 8e | Checkpoint 8f | `s8e` | `s8g` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45, at `a6349b6`.* What changed:
- **N10.** A content checkpoint is made at every committed step that changed the worktree, as one boundary act
  with the step's commit (§8f.4). This replaces "checkpoint when due" and the void-and-re-run route. AC-013's note
  and the route (i)/(ii)/(iii) choice are gone.
- **N11.** `structured_drift` also opens a drift gate that blocks the stage's exit until it is answered (§8f.5).
  AC-127 holds, and AC-125's "resolving" means answering that gate. Both are now claimed.
- **N12.** This stage owns AC-159's `doctor` rule (§8f.9) and the `rk-store` read seam through which `s8d`'s
  resolver emits `artifact.read` (§8f.5). AC-126 and AC-472's emission clause are now claimed. AC-372's doctor
  clause is read, by dated note, as a test of `cost`'s renderer with AC-415's red case (§8f.8).
- **N5.** The general interim `item` key is `s8e`'s record, and this file cites it. The telemetry-key record keeps
  `activity_id_at_version` and the cost breakdown (§8f.8).
- **D21-44 / D21-45 W2.** The run-owned worktree is `.redkiln/worktrees/<item>`, and re-entry reuses it (`s8d-ii`
  owns both since D21-46 R1). The re-entry rules in §8f.4 are rewritten over reuse.
- **Settled residuals.** Where the canonical typed value lives (§8f.5), the managed path (§8f.6), and how
  `activity_id_at_version` reaches the adapter (§8f.8) are stated as the records' decisions, not left open.
- **Handed on.** The `landing` and drift gate classes each need a `gate_expiry` row, in `s8h`'s table (§8f.1).
- **Floor.** The floor is re-derived at stage open from Checkpoint 8e's recorded count (§8f.2 item 9). The
  Checkpoint re-runs only 8e, plus 7i, which this stage makes provisional (§8f.13).
- **Found in revision:** `item_create`'s refusal of an existing id would refuse its own re-execution (§8f.7).
  `artifact.read`'s seam lands after the stage that emits through it (§8f.2 item 10).

*Revised 2026-09-24 for D21-46 R7, at `c5d19d7`.* What changed:
- **R7 (1): the `toolchain:backlog` producer.** A `store`-route read, `backlog`, over the item records the store
  names. Each record is read by its exact path, never by a walk of `.redkiln/` (§8f.7). `[feed.rust.backlog]` and
  `[feed.node.backlog]` move from `unprovided` to `verb = "backlog"`. `s8c` §8c.12's handed-on item 3 closes here.
- **R7 (2): AC-127's vouched-root half.** The attached root is stored at the next entry, and a change to it is
  advisory drift under ADR-0025 (§8f.5). `doctor` gains `RK-D-DRIFT-KIND`, the row that tells the two drift kinds
  apart (§8f.9). AC-127 is claimed whole, and the exclusion that RAISED it with no owner is removed (§8f.11).
- **R7 (3): the unprovided feeds.** The feed-to-activity list is measured over `.redkiln/catalog/` and handed to
  `s8d`'s walk table by citation (§8f.7). It finds that `s8d` §8d.9's `closure-record` rows cannot hold at
  Checkpoint 8d.
- **Counts.** New tests go from 64 to **79**, and mutants M32–M43 are added (§8f.12). The drafted floor goes from 954
  to **969**, still re-derived at stage open (§8f.2 item 9). `doctor`'s rule table goes from 39 ids to **41**.
  *(Stale, corrected 2026-09-24: 969 took a stale 954 base. The consistency pass recomputed the floor to 1063, and
  D21-47's fixes make it **1068** over **84** new tests (§8f.2 item 9). The final consistency pass makes it
  **1105**.)*
- **Found in revision (R7):** `s8c` §8c.4 cites ADR-0032 for an optional unprovided feed's `null`, and ADR-0032's
  frontmatter reads `superseded_by: ["0103"]`. The rule stands. Re-pointing the citation is `s8c`'s (§8f.7).
  *(Reversed 2026-09-24 by D21-47: ADR-0103 supersedes ADR-0032 only in part, and ADR-0032's Decision still stands
  and is the right citation. §8f.7.)*

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The run worktree's path, reuse and refusal are `s8d-ii`'s** (D21-46 R1). `s8e` keeps `worktree_assigned` and the
  discard's journal row. Every sentence that gave all of it to `s8e` now names the split.
- **The telemetry-key record, restated under R3** (§8f.3, §8f.8). `dispatch_completed`'s `item` is `s8e` §8e.10's
  interim key, carried **beside** the payload and outside the canonical digest, which supersedes nothing (D21-46 R3).
  `activity_id_at_version` stays a **payload** member on D21-42 12b's ADR-0163 route, so the record still supersedes
  ADR-0131's first forbids bullet in part for that one member, and ADR-0163's bullet with it.
- **After R2 the gate table is `t1_gate_transition`**: §8f.5's stage-exit fold and Checkpoint 8f (1b)/(8)'s store probe
  read it, because `s8e`'s `0002` migration drops `t1_gate`.
- **Checkpoint 8f (9) adopts D21-46's re-run convention and replay guard**: it skips its whole re-runs of 8e and 7i
  under `RK_CP_NESTED=1`, and otherwise launches both with `RK_CP_NESTED=1`, `REDKILN_LAUNCH_RECORD` unset and
  `CLAUDE_CONFIG_DIR` at an empty trap.
- **The floor chain is recomputed once**: Checkpoint 8e's 984 + this stage's 79 = **1063**, not the drafted 969.
  *(D21-47 moves it to 984 + 84 = **1068**, and the final consistency pass to 1021 + 84 = **1105**.)*
- **`s8c` §8c.4's re-pointed citation and `s8d` §8d.9's re-claimed walk table** are quoted from their current bytes.
  *(The re-pointed citation is reverted by D21-47. §8f.7 now cites ADR-0032's Decision directly.)*

*Adversarial-review fixes 2026-09-24 (D21-47)* (review of the file at `4a8c832`: 0 BLOCKER, 7 MAJOR, 9 MINOR, plus
the cross-file items D21-47 assigns here; every fix below was checked against the tree at `fc80687`):
- **M1 → §8f.2 item 10, §8f.5.** rk-store's read seam is the **only** emitter of `artifact.read` (D21-47). `s8d`'s
  kernel function calls a named stub that emits nothing, and this stage re-points that one call to the seam. The
  stage-open STOP now fires on any `artifact.read` append outside the seam.
- **M2 → §8f.5, §8f.9, §8f.2 item 14, §8f.12.** `RK-D-UNREAD` keys on the payload's `stage` and `run`, and it skips a
  row that lacks either, with no report. So every `artifact.write` and `artifact.read` row now carries both, and
  `answer`'s producer does too. A read row also carries `schema_version` and the tree hash it served (design §5.4).
  The rule's `run_of` also reads a string `run`. New test:
  `doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes`.
  New mutants: M44 and M45.
- **M3 / cross #6 → §8f.1, §8f.7, §8f.10, §8f.13.** `debt-logging` refuses at `file` at Checkpoint 8d, and its handler
  is this stage's. Its rows flip to **walks** by the same dated line, so `s8d`'s claims become 13 of 15 and 11 of 13.
  8d is now **provisional**, and (9) re-runs it whole. New test:
  `landing::debt_logging_walks_past_file_through_item_create`.
  New mutant: M46.
- **M4 → §8f.3, §8f.4.** Under D21-47, `commit-tree`, `update-ref` and `read-tree` on the worktree's own index are
  writes that are not on clause 5's list. They **supersede ADR-0105 clause 5 in part**. That is a supersession, not a
  "reading" of clause 3. The list is consolidated beside `s8d-ii`'s worktree record and `s8g`'s R3, whose sets of
  operations do not overlap.
- **M5 → §8f.3, §8f.8, Checkpoint 8f (2).** The telemetry-key record also supersedes in part ADR-0163's bullet
  *"Forbids reading item ownership off `dispatch_completed` rows"*, for the carried `item` key only. (2) asserts that
  the record quotes it.
- **M6 → §8f.4, §8f.10 (AC-245).** The record states which reading of "due" it takes. A landing that refuses a path
  lands nothing, so **no checkpoint commit is due** at its gate, and clause 4's no-checkpoint-due derivation is the
  right tree. New test: `landing::a_refused_landing_is_not_due_and_answer_derives_the_gates_tree`. New mutant: M48.
- **M7 → §8f.7 (and the history notes above).** The optional-`null` rule cites ADR-0032's Decision, *"Absent `?`
  inputs are `null`"*, which still stands. ADR-0103 decision 4 is predicate semantics. A refused mandatory input is
  N14's.
- **m8 → Checkpoint 8f (3).** The C2 needle is `\bresolved\b`, so "unresolved" cannot satisfy it. A control proves
  that. The AC register §0.3's two live "only C2" sentences must be gone.
- **m9 → §8f.3, Checkpoint 8f (2).** Each record is located by the id written into its `REC_` line at writing. The
  flip is a dated `flips to` `accepted` heading.
- **m10 → §8f.2 item 3.** The STOP no longer fires on a property that holds by design at Checkpoint 8e. It fires
  only when the step-grain act would need a second seam.
- **m11 → §8f.5.** `Advisory.upstream` becomes `Option<String>`, and `None` serialises as JSON `null`. The named
  regressions are AC-525's existing tests. New test:
  `advisory::an_own_change_advisory_writes_upstream_null_with_all_four_fields`.
  New mutant: M50.
- **m12 → §8f.4, §8f.6, §8f.2 item 15.** The item appears twice in the path. That is kept, so that merged items
  cannot collide. Every staged path is refused by name when it exceeds `s8d-ii`'s worst-case length bound. New
  test: `landing::a_staged_path_over_the_worst_case_length_is_refused_by_name`. New mutant: M49.
- **m13 → §8f.2 items 5 and 6.** The `to_json()` quote is ADR-0131's (line 67) and `s7d` §7d's ownership table's. It
  is not D18-14's own. The store facts now read `s8e`'s v2 rebuild.
- **m14 → Checkpoint 8f (6), (6b), (7).** The no-float floor covers `rk-types` (`CostUsd`) and the shared renderer's
  file. The no-walk floor covers all of `rk-store/src`. Each grep has a planted-hit control.
- **m15 → §8f.7, §8f.12, Checkpoint 8f (0b).** `item_create`'s binary test points `harness_binary` at
  `rk-fixture-child-claude`. Every process-level run in the block runs under the D21-47 wrapper: a fake `claude` that
  prints the pin, `REDKILN_LAUNCH_RECORD` unset, and a `CLAUDE_CONFIG_DIR` trap proved empty before and after.
- **m16 → §8f.10 (AC-478), Checkpoint 8f (3).** The AND line is removed in place, and a dated note goes after
  *Verified by:*. The "silently lossy" **Note** takes a dated correction.
- **Cross #16 → §8f.1, §8f.5.** This stage owns the stage advance's refusal on an unanswered drift gate. `s8e` builds
  the advance, and this stage adds the refusal to it. New mutant: M47.
- **Cross #18.** The stale 954 → 969 line above is corrected.
- **Counts.** New tests go from 79 to **84**, and mutants M44–M50 are added. The floor is 984 + 84 = **1068**,
  re-derived at stage open. Checkpoint 8f (9) re-runs 8e, 8d and 7i. *(Floor superseded by the final consistency
  pass below: 1021 + 84 = **1105**.)*

*Final consistency pass 2026-09-24* (the eleven `s8` files read against each other after the D21-47 fix round, at
`fc80687` plus that round's edits; D21-47 and the pass's brief the authority):
- *Final consistency pass 2026-09-24:* **The shared no-dispatch wrapper.** Checkpoint 8f (0b) sources
  `runbook/tools/nodispatch.sh` and calls `nd_setup` in place of its inline shell-script fake. Every process-level run
  goes through `nodispatch`, and `nd_check` runs after the build, each named selection, the whole suite, fmt, clippy,
  each replay and at the end. The shell-script fake is gone: on Windows a Rust `Command::new("claude")` resolves only
  `claude.exe`, so the helper compiles its fake. The planted-launch-record and trap-path controls stay, as this
  block's own assertions beyond the helper (§8f.13).
- *Final consistency pass 2026-09-24:* **Record ids.** The three `REC_` lines were already literals written at
  writing, refusing while they read `unassigned`. No re-runner supplies an id, and none of the replays in (9) needs one.
- *Final consistency pass 2026-09-24:* **The floor.** Checkpoint 8e's expected value is 740 + 9 + 25 + 69 + 58 + 46 +
  74 = **1021**, from each predecessor's new-test count as its file now states it. So Checkpoint 8f's is 1021 + 84 =
  **1105** (§8f.2 item 9, Checkpoint 8f (5)). This stage's 84 agrees with §8f.12's lists.
- *Final consistency pass 2026-09-24:* **`s8d`'s stub, by name.** §8f.2 item 10 and §8f.5 name what `s8d` §8d.3
  built: `runtime::reference::read_artifact_input` calls an `ArtifactReadSeam`, whose production value at Checkpoint
  8d is the non-emitting stub `UnwiredReadSeam`. This stage re-points the composition root's value. The stub is kept,
  not deleted, because `s8d`'s named test reads it and (9) re-runs Checkpoint 8d.
- *Final consistency pass 2026-09-24:* **The landing routes through `s8e`'s boundary module (AC-518).** `s8e`'s lint
  `tier_one::every_run_commit_resolves_through_the_worktree_resolver` sees `commit_with_trailers` only, and the landing
  commits with `commit-tree` and `update-ref`. This stage widens the lint (amended, not added), claims the halves `s8e`
  leaves to it, selects the lint in Checkpoint 8f (4), and adds mutant M51 (§8f.4, §8f.11, §8f.12).
- *Final consistency pass 2026-09-24:* **`s8d`'s `side_effecting` column.** `s8d`'s R1 makes a writing tool step commit
  and be served on re-entry. `item_create` is one, so its write lands at its own step and is no longer an example of
  work a boundary carries (§8f.4). Its byte-identical serve now rests on the `on_false` loop-back alone (§8f.7).
- *Final consistency pass 2026-09-24:* **`s8d-ii`'s length bound** is computed on the item's actual length. §8f.2 item
  15 and §8f.4 now read it that way.
- *Final consistency pass 2026-09-24:* **Checkpoint 8d's `LANE_WALK` floors.** §8f.10's `s8d` row now cites them: a
  flip to **walks** raises them, so no guard line moves.
- *Final consistency pass 2026-09-24:* **`s8c`'s `backlog` hand-off is re-quoted** from its current §8c.4 row (§8f.7).
- *Final consistency pass 2026-09-24:* **The opening run's `run_id` on a gate row is specified by `s8e`.** Its §8e.7
  puts `stage` and `run_id` on `t1_gate_transition`'s `opened` row. §8f.2 item 14 and §8f.5 now read that carrier,
  and the STOP fires only if Checkpoint 8e's row lacks it. The rebuilt row's null `run_id` is named (§8f.2 item 14).

*D21-48 (2026-09-24)* (the operator's answers to the questions the s8 files left open; `spikes/DECISIONS-2026-09-21.md`'s
`## D21-48` is the authority):
- *D21-48 (2026-09-24):* **X11 decides the rebuilt `opened` row's null `run_id`.** D21-48 X11: *"Accepted as a
  stated residual. RK-D-UNREAD reads null as an unknown run, never as a match. The `Gate:` grammar does not widen."*
  §8f.2 item 14 states it as decided, and the render record cites X11. The work it adds is here, because this stage
  widens `run_of` to read the JSON value's canonical text, and a naive widening would read `null` as the run
  `"null"`, so that every null-run row matched every other: §8f.5 now says `run_of` reads `null` (and an absent
  `run`) as no run. New test: `doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match`. New mutant: M52.
- *D21-48 (2026-09-24):* **Counts.** New tests go from 84 to **85** (`rk-cli`, the rest, 23 → 24; Checkpoint 8f (4)'s
  `rk-cli` selection 45 → 46). The floor is 1021 + 85 = **1106** (§8f.2 item 9, Checkpoint 8f (5)), still re-derived
  at stage open from Checkpoint 8e's recorded count. `s8g`'s `PREV` follows it.
- *D21-48 (2026-09-24), floor propagation:* the predecessors now state `s8c` **78**, `s8d` **62** and `s8d-ii` **50**, so Checkpoint 8e's expected
  count is 740 + 9 + 25 + 78 + 62 + 50 + 74 = **1038**, and this stage's floor is 1038 + 85 = **1123** (§8f.2 item 9,
  Checkpoint 8f (5)). The 1021 + 85 = 1106 above is superseded.

---

**Why this step exists.** AC-473 asks for *"every lane stage from intake to a landed commit"*, and nothing on the tree
can produce a landed commit. Every fact below was read at `ca8f387` on 2026-09-24 and re-read at `a6349b6` for this
revision. §8f.2 re-takes each one at Checkpoint 8e's close, because `s8a`–`s8e` move code this stage builds on.

- **Nothing stages the harness's work.** `crates/rk-cli/src/verbs/run.rs` makes both of its tier-1 commits through
  `rk_git::trailer::write::commit_with_trailers(…)` with the last argument `true` (allow-empty), and no `git add`
  precedes either. `answer.rs` is the third caller, with the same shape. Since `s7h` a run leaves commits on
  `rk-run/<item>`, but they are empty trailer commits, so the run makes **zero content commits**. At Checkpoint 8e
  these commits move into the kernel's boundary function (`s8e` §8e.3), and they are still empty.
- **Re-entry destroys what was not committed.** `recreate_run_worktree` runs `["worktree", "remove", "--force",
  path]` and then `std::fs::remove_dir_all(&path)` before it re-adds the branch. Its comment says *"The teardown's
  failure is ignored ON PURPOSE"*. Commits on the branch survive this, and nothing else does. D21-45 W2 replaces it
  with reuse (*"Reuse, never force-remove"*), and that work is `s8d-ii`'s (D21-46 R1).
- **The gate binds the wrong tree.** `prepare_inputs` hashes `&resolved.worktree` (the invoking checkout) with
  `rk_git::tree::TreeBasis::WorkingState`, at stage start and before any dispatch. `terminal_read` copies that value
  into the `Gate:` trailer. `answer` hashes the run-owned worktree.
- **`rk-store` has no render, write or read path.** `crates/rk-store/src/lib.rs` declares `artifacts`, `canon`,
  `conn`, `identity`, `locator`, `memo`, `retention`, `schema_lint` and `tier`. None of them writes a file or journals a
  read. `crates/rk-kernel/src/runtime/step.rs` reads `StepKind::Emit => StepVerdict::Emitted,` and writes nothing.
  `artifact.write` has one producer, `answer.rs`'s `store_attachments`. That producer journals
  `rk_types::digest::sha256(&bytes)` of a file the operator supplied, and it writes no artifact file.
  `artifact.read` has **no** producer: outside `rk-types`, only the three kernel fold arms and `doctor`'s
  `RK-D-UNREAD` name it.
- **`structured_drift` has no producer.** `EventKind::StructuredDrift` appears in no `src/` outside `rk-types`,
  except the three kernel fold arms (`breaker.rs`, `fsm.rs`, `memo.rs`). AC register §0.3's contradiction **C2** is the
  last one still open: *"Is a hand-edit inside a rendered section drift, or is it silently overwritten?"*
- **`dispatch_completed` carries no item, and the adapter cannot see an activity id.** `cost.rs`'s module doc says
  *"`dispatch_completed` carries no item"*. `ActivitySpec` is `card_digest`, `tools`, `schema`, `timeout_secs`,
  `inputs`, so no field carries `activity_id_at_version`.
- **`doctor` reads a file, not the store.** `crates/rk-cli/src/doctor/mod.rs` builds its journal with
  `let journal = read_journal(&worktree.join("events.jsonl"));`. The field's own doc says *"Read from the STORE in the
  shipped binary"*, which is false. `RK-D-HEADROOM-MARGIN` takes the `.last()` `rate_limit_reading` row and reads
  `r.payload.get("windows")?.get("five_hour")?.as_i64()` off it. A flat reading carries `"windows": null` (ADR-0163
  decision 3), so when a flat reading is last the rule goes silent. None of the table's 39 rule ids compares a
  managed artifact's digests (AC-159).
- **`GitOp::Integrate` has no caller** outside `crates/rk-git/src/git_exec.rs`, and in Phase 1 it keeps none (§8f.4).

**This step takes three records, and it says so at the start.** `s7g`, `s7h` and `s7i` each opened with *"This step
takes no decision"* and were each corrected at their own work. `s7i`'s correction reads *"this is the third
consecutive stage whose file said this sentence and was wrong"*. The decisions this stage executes are taken:
D21-41 (b); D21-42 5a, 5b, 6a (this stage's half), 9, 12b and 12c; and D21-43 N10, N11, N12, and N5 for the member
this stage keeps; and D21-46 R7. The ledger is `spikes/DECISIONS-2026-09-21.md`. What those decisions leave free is named in §8f.3
as each record's to settle. Nothing here is left for the implementer to settle. **No record number is
pre-assigned**: each record takes the next free id at writing, because several `s8` sub-stages write records and
their order is not fixed. On 2026-09-24, `references/adr/README.md` gives **`0165`** as the next free id.

🛑 **No real dispatch happens at this stage.** Every behavioural proof runs a `FakeHarness` or a fixture corpus
(D21-40 decision 2: real runs *"once the sub-stages' fixture proofs pass, and not before"*).

### 8f.1 What this stage owns, what it hands on, and the order inside it

| decision (ledger) | what it makes | section |
| --- | --- | --- |
| D21-41 (b); D21-42 9; D21-43 N10 | The kernel stages the run-owned worktree. A content checkpoint is made at every committed step that changed the tree, in one boundary act with that step's commit. Every tier-1 boundary rides on a content commit when one is due. Refused paths are refused by name. There is no integration. `explain` prints the hand-off | §8f.4 |
| D21-42 5a; D21-43 N11 | One record owns `rk-store`'s render and write path. **C2** is resolved as surface-then-restore, and the surface opens a drift gate that blocks the stage's exit. The stage advance's refusal on an unanswered drift gate is this stage's: `s8e` builds the advance, and this stage adds the refusal (D21-47, cross #16) | §8f.5 |
| D21-43 N12 (the seam); D21-47 | `rk-store`'s read seam, the **sole** emitter of `artifact.read`: one row per read, refused before the entry's `reconcile`. `s8d`'s non-emitting stub is re-pointed to it | §8f.5 |
| D21-42 5b | Rendered artifacts land in the item's managed directory inside the run-owned worktree. The run path gets its `artifact.write` producer | §8f.6 |
| D21-42 6a (this stage's half) | `DebtRegisterEntry`'s in-repo item write, through `rk-store`, with provenance. `debt-logging` walks past `file` in both lanes | §8f.7 |
| D21-42 12b; D21-43 N5 (cited), N12 (AC-372) | `dispatch_completed` gains `activity_id_at_version`, and its `item` is `s8e`'s N5 key. AC-395's per-activity breakdown and AC-372's `status` half are built | §8f.8 |
| D21-42 12c; D21-43 N12 (AC-159) | `doctor` reads the store's journal. The flat-reading silencing of `RK-D-HEADROOM-MARGIN` is fixed. AC-159's rule is built | §8f.9 |
| D21-46 R7 (1) | The `toolchain:backlog` producer: a `store`-route read of the item records the store names, each by its exact path. `[feed.*.backlog]` moves from `unprovided` to `verb = "backlog"` | §8f.7 |
| D21-46 R7 (2) | AC-127's vouched-root half: the attached root is stored at the next entry, and its change is advisory drift (ADR-0025). `doctor`'s `RK-D-DRIFT-KIND` tells the two drift kinds apart | §8f.5, §8f.9 |
| D21-46 R7 (3) | Unprovided feeds refuse by name where the input is mandatory. The feed-to-activity list goes to `s8d`'s walk table | §8f.7 |

**Built on, not rebuilt.** Checkpoint 8e is the entry gate. It delivers D21-42 8a: *"The kernel owns it, through the
journalled `GitOps` seam. It commits (zero writes on replay), then appends the t1 and t2 rows with `anchor_sha`."*
`s8e` §8e.3 places that in one boundary function, `rk-kernel`'s `runtime/tier_one.rs`. **This stage extends that
function and does not route around it.** The landing replaces the function's empty commit with the landed one. It
also adds the step-grain boundary act that D21-43 N10 needs. The seam that act runs on is `s8e`'s to support, and
the staging rule is this stage's. Checkpoint 8e also delivers these, and this stage cites each by owner and does
not re-specify it:
- **The replay journal for git** is D21-43 N7's tier-2 `git_op` row (`s8e`): *"A tier-2 `git_op` row is minted so
  replay serves git writes from the journal"*. The landing's reads and writes are served on replay from it.
- **Every gate transition is git-first** (N6, `s8e`), so the `landing` and drift gates below open and close
  through it.
- **The lane's end** is N9's terminal `Story: <project>/<item> closed` mark (`s8e`).
- **The run-owned worktree** lives at `.redkiln/worktrees/<item>` (D21-44), and it is reused rather than
  re-created (D21-45 W2). `s8d-ii` owns the path and the reuse (D21-46 R1), and `s8e` owns `worktree_assigned` and
  the discard's journal row.
- **The interim `item` key** on every item-scoped event kind (N5) is `s8e`'s record.

From `s8d` (Checkpoint 8d) this stage takes the resolved emit binding, step-grain commits and re-entry (D21-42
3a/3b). It gives the emit arm its effect, and it gives the resolver's `artifact:` arm the read seam N12 names. From
`s8c` it takes the verb registry's `store` route. `s8c` lands the `item_create` row as *"Refused naming the route
until `s8f`"*, and this stage builds its handler (§8f.7). `s8c` also declares `[feed.*.backlog]` `unprovided` and
names this stage as the candidate owner of its read (`s8c` §8c.12 item 3). D21-46 R7 makes it this stage's (§8f.7).

**Handed on, by name.**
- `s8g` builds the other four producers in 6a.
- **`s8h`'s fail-closed `gate_expiry` table** (D21-42 21a) needs two rows for the gate classes this stage mints:
  `landing.refused-path` (§8f.4) and `render.structured-drift` (§8f.5). The landing and render records fix the
  spellings. `s8h` §8h.10 also refuses *"a row whose class is neither a catalog question, nor
  `unknown_facet:<registered facet>`, nor `escalation`"*, so `s8h` must admit these two kernel classes by name.
  Both rows are signed there, row by row (AC-276), and nothing is defaulted here. Until `s8h` lands, the two
  classes reach no expiry, because no table exists at this stage.
- `s8j` makes `s8`'s instrument amendments: (1)'s non-empty diff, and (1b)'s `artifact.write` digest plus
  `git cat-file -e` on the run ref. This stage supplies the mechanisms those lines read (§8f.6), and it does not edit
  `s8-exit-criterion.md`.
- **`s8d`'s walk table (D21-46 R7).** §8f.7's feed-to-activity list goes to `s8d` §8d.9 by citation. At Checkpoint
  8d, `closure-record` stops at `audit` in both lanes, refused by name on `toolchain:backlog`, and its owner is this
  stage. `s8d` §8d.9 now records it as refusing (consistency pass 2026-09-24). This stage's close flips the rows back
  to **walks** by dated line (§8f.10). *Amended 2026-09-24 (D21-47, cross #6):* `debt-logging` also stops at
  Checkpoint 8d. It stops at `file` in both lanes, on `item_create`, whose handler is this stage's (§8f.7). The same
  dated line flips its two rows to **walks**, so `s8d` §8d.9's claims then read **13 of 15** (fix lane) and **11 of
  13** (story lane). The rows and the two `lane_walk` expectations are Checkpoint 8d's premise, so moving them makes
  8d **provisional**, and Checkpoint 8f (9) re-runs it whole.

**Order.** (i) The stage-open measurements (§8f.2). (ii) The three records, written `proposed` before any code they
authorise. (iii) The landing: `rk-git`'s primitive, then the kernel wiring on `s8e`'s seam (the step-grain act and
the boundary act), then the `rk-cli` end-to-end tests. (iv) The render, write and read paths, then the drift gate
and the stage advance's refusal on it, then the vouched-root store and its advisory, then the emit producer and the
resolver's read, then the item write, then the `backlog` read and the two feed rows. (v) The `activity_id_at_version` member, then
`cost` and `status`. (vi) `doctor`'s journal, then AC-159's rule, then
`RK-D-DRIFT-KIND`. (vii) The dated notes (§8f.10). (viii) Checkpoint
8f.

### 8f.2 Stage-open measurements — re-taken at Checkpoint 8e's close, before any code

Each is read-only. A 🛑 line is a STOP. The stage does not choose around it: the finding goes to the operator.

1. 🛑 **`s8e`'s seam is where the boundary commit is made.** The kernel owns the stage-boundary and gate commits
   through the journalled `GitOps` seam, and the tier-1 rows carry `anchor_sha`. If the commits are still made in
   `rk-cli` over `GitExec`, the landing has no seam to live on. Building it in `rk-cli` would widen the gap the
   packet's (b) named: *"The new add/commit, like the existing three commits, goes through `GitExec`, not the
   journalled `GitOps`"*.
2. 🛑 **N9's terminal mark exists.** The draft of this file stopped here because `s8e` minted `Story:` only at entry
   and advance. D21-43 N9 settles it: *"A terminal `Story: <project>/<item> closed` mark carrying closeout's content
   checkpoint"*, and it is `s8e`'s to build. Measure that Checkpoint 8e's lane writes it. If it does not, the lane's
   last work has no trailer to ride on (ADR-0109 decision 2): STOP.
3. 🛑 **The step-grain boundary act fits `s8e`'s seam and `s8d`'s commit.** N10 needs the step's commit row to be
   appended **after** the content checkpoint commit, with that commit as its `anchor_sha`. At Checkpoint 8e the
   function takes stage and gate acts only, and that is by design. `s8e` §8e.3 says *"A trailer-less act with
   content is the shape N10's per-step checkpoint takes, and `s8f` builds and tests it"*. Widening the act's key to
   a step is this stage's extension of the function, not a STOP. Measure two things:
   - whether `s8e`'s act type can carry content with no trailer, and several rows anchored to one commit;
   - where `s8d`'s walk appends an agent step's commit row and an activation's `emit` commit.

   STOP only if one of these holds, because each would need a second seam:
   - the act type cannot carry a trailer-less act with content, and cannot be widened without changing its order of
     commit, re-read, then rows (`s8e`'s to support);
   - `s8d`'s step commit row is appended by a call site that cannot be moved into an act. That is the case if it
     appends the row before its own git write, or appends it in a transaction the act cannot enclose.

   The seam's support is `s8e`'s (D21-43 N10's boundary act runs on D21-42 8a's seam), and this stage does not build
   a second seam.
4. 🛑 **No golden log or pinned fixture holds a `dispatch_completed` row.** Read the golden logs under `CORPUS_PIN`
   and every `-text`-pinned fixture in `.gitattributes`. ADR-0131's *cheap* ground is *"every golden log's canonical
   events change, the corpus regenerates, `CORPUS_PIN` moves"*. ADR-0163 answered it with *"no golden log contains a
   `rate_limit_reading` row"*. If a `dispatch_completed` row sits in a golden log, §8f.8's member moves it. STOP and
   put the cost to the operator. `s8e`'s N5 key meets the same question first; read how Checkpoint 8e answered it.
5. **What `ActivitySpec`'s serialised form enters.** Does it enter a `run_id`, a card digest, a memo key, or
   ADR-0151's pinned run-record fixture? The `activity_id_at_version` carrier (§8f.8) is omitted when `None`. That
   follows `Config`'s precedent, which `s7d`'s ownership table records as *"omitted from `to_json()` when `None`"*
   and which ADR-0131's drivers call *"D18-14's byte-stability property"*. The quotation is theirs, not D18-14's own
   words (corrected 2026-09-24, D21-47). This measurement shows whether that care is enough.
6. **The run-owned worktree and the shipped store.** At `a6349b6` one run-owned worktree is registered. It is the
   legacy tree `.git/redkiln/worktrees/rk-fix-doctor-test-count` on `rk-run/rk-fix-doctor-test-count` at `73c087a`,
   which stays where it is until `s8j` re-points it (D21-44). Its `status --porcelain=v1 --untracked-files=all` is
   **empty**. New run-owned worktrees are created under `.redkiln/worktrees/<item>`, and `.gitignore` carries the
   row `.redkiln/worktrees/` (D21-44's commit). At `a6349b6` the shipped store (`mode=ro`) stamped version 1 and
   held `t2_event` `dag_assembled` 1, `reconcile_started` 3 and `stage_transition` 2, with 0 rows in each `t1_*`
   table. *Stale after `s8e` (corrected 2026-09-24, D21-47):* `s8e` §8e.2 rebuilds the shipped store at version 2 in
   its migration's commit, and the rebuild drops tier 2. Re-read at Checkpoint 8e's close, `mode=ro`, and check
   three things. `rk_identity.schema_version` must be 2. `t1_gate` must be absent and `t1_gate_transition` present.
   The per-table counts must equal the after-counts recorded in `s8e`'s migration commit body. Checkpoint 8f (1b)
   and (8) read those counts. They do not read the `a6349b6` numbers.
7. **The denylist and the size limit's warrant.** At `a6349b6`, **0** of the 1,836 tracked paths match §8f.4's
   denylist globs (1,826 at `ca8f387`). The largest tracked file is still **1,439,568 bytes**
   (`references/audit/2026-09-10/GROUNDING-MERGED.json`). Any limit the landing record declares is read against that
   number.
8. **The test module names are free.** This stage adds these modules: `rk-git` `landing`; `rk-store` `render`,
   `item_write`, `backlog` and `vouched`; `rk-kernel` `emit`, `step_landing` and `feed`; and `rk-cli` `landing`,
   `item_create`, `status`, `doctor_journal`, `doctor_artifact` and `doctor_drift`. If a predecessor stage created one of them, the tests go into it. The
   module-qualified names in §8f.12 stay as they are.
9. **The whole-suite floor, re-derived at stage open from Checkpoint 8e's recorded count.** Checkpoint 8f's
   `FLOOR` is Checkpoint 8e's recorded passed count plus this stage's **85** new tests (64, plus D21-46 R7's 15, plus
   D21-47's 5, plus D21-48 X11's 1). The chain, recomputed once under D21-46, gives the expected value. `ca8f387`'s whole suite measured
   740 passed, 0 failed and 9 ignored over 44 results (ADR-0164's evidence, `992d257`). *Final consistency pass
   2026-09-24:* the stages then add, as each file now states its new tests, `s8a` 9, `s8b` 25, `s8c` 78, `s8d` 62,
   `s8d-ii` 50 and `s8e` 74. That gives 740 + 9 + 25 + 78 + 62 + 50 + 74 = 1038 at Checkpoint 8e, and 1038 + 85 =
   **1123** here. *(This supersedes 984 + 84 = 1068, whose base took the predecessors' counts before their D21-47
   fixes, and 1021 + 84 = 1105, before D21-48 X11's test. *D21-48 (2026-09-24), floor propagation:* and 1021 + 85 = 1106,
   before `s8c`'s 78, `s8d`'s 62 and `s8d-ii`'s 50.)* The recorded count governs. Replace 1123 with the
   recorded count plus 85, by a dated edit of that line,
   before the block's first run. The floor is a lower bound, and it must not be a stale one.
10. 🛑 **The `artifact:` arm's stub, and that nothing outside the seam emits (D21-47).** N12 reads *"`artifact.read`
    is emitted by s8d's resolver through s8f's rk-store read seam"*. D21-47 settles who owns it: *"`artifact.read`
    has exactly one emitter: rk-store's read seam (s8f). s8d's kernel function calls a named, non-emitting stub
    that s8f re-points."* Measure at Checkpoint 8d's close:
    - The named stub that `s8d`'s `artifact:` arm calls. *Named 2026-09-24 (final consistency pass), from `s8d`
      §8d.3:* the kernel function `runtime::reference::read_artifact_input` calls a trait object, `ArtifactReadSeam`,
      and at Checkpoint 8d its one production value is the non-emitting stub `UnwiredReadSeam`, *"which appends
      nothing"*. Record the path of each. `rk-kernel` cannot depend on `rk-store`, so this stage re-points the
      composition root's `ArtifactReadSeam` value to the rk-store seam. `UnwiredReadSeam` is **kept**, not deleted:
      `s8d`'s `reference_resolver::every_resolved_artifact_input_calls_the_read_seam_once_and_the_stub_emits_no_row`
      reads it, and (9) re-runs Checkpoint 8d, which selects that test by name. It is no longer the composition
      root's value, and it appends nothing, so it is not a second emitter.
    - Every code-line `EventKind::ArtifactRead` construction and every `"artifact.read"` append under
      `crates/*/src/`, outside `rk-types` and the fold arms.

    STOP in any of these cases, because each makes a second emitter or leaves no call site to re-point:
    - the arm has no single named call;
    - the stub appends a row;
    - any site outside the seam appends `artifact.read`.

    After this stage the seam is the only producer, and `emit::every_artifact_input_resolution_appends_one_artifact_read`
    counts exactly one row per resolved input.
11. 🛑 **The feed rows, the `store` route and the resolver's feed path, as Checkpoint 8e leaves them (D21-46 R7).**
    Read `.redkiln/registry.toml`. `[feed.rust.*]` and `[feed.node.*]` should each declare `backlog`, `logs` and
    `sbom` as `unprovided` (`s8c` §8c.4), and `item_create`'s row should be on the `store` route. Then read how
    `s8d`'s resolver resolves a feed that declares `verb`.
    - If the resolver has no path from a feed's `verb` to a `store`-route handler, the producer has no call site.
      STOP: the resolver is `s8d`'s.
    - Read what Checkpoint 8d's two `lane_walk` tests measured for `closure-record`. If a test records it as walking
      over an `unprovided` mandatory feed, the resolver served a mandatory absence as a value, which is the silent
      `null` N14 forbids. STOP.
    - Read whether Checkpoint 8e's `lane_e2e` reaches `closeout`. If it does, its expectation for `closure-record`
      moves in this stage's commit, by the same dated line as `s8d`'s (§8f.10).
    - Read whether an `s8c` census test pins `backlog` as `unprovided`. If one does, it moves by that dated line too.
12. 🛑 **Where an attached root's bytes can be read at the next entry (D21-46 R7).** At `c5d19d7`, `answer.rs`'s
    `store_attachments` journals `kind`, `item`, `schema_version` and `digest`, and nothing names the attached
    path. The render record names the carrier that brings the path to the next entry (§8f.5). This file's route is
    one repo-relative `path` member on the attach's row. Read the golden logs under `CORPUS_PIN` and every
    `-text`-pinned fixture for an attach `artifact.write` row, as item 4 does for `dispatch_completed`. If one holds
    such a row, the carrier moves it. STOP and put the cost to the operator.
13. **The `doctor` rule table's size.** At `c5d19d7`, `crates/rk-cli/src/doctor/` names **39** distinct `RK-D-` ids.
    This stage adds two, `RK-D-ARTIFACT-MUTATION` and `RK-D-DRIFT-KIND` (§8f.9). Re-count at open. A
    predecessor's rule moves the base, and it does not move the two.
14. 🛑 **Where `answer`'s `artifact.write` takes its `stage` and `run` (D21-47, review M2).** `RK-D-UNREAD`
    (`crates/rk-cli/src/doctor/rules.rs`, `fn unread`) keys a row on `str_field("stage")` and `str_field("kind")`, and
    it counts runs from `payload.get("run")`. A row that lacks `stage` or `run` is passed over by a `continue`, and
    nothing reports it. At `fc80687`, `store_attachments` journals `kind`, `item`, `schema_version` and `digest`, so the
    rule never counts one of its rows. The render record fixes the values: `stage` is the stage of the answered gate,
    and `run` is the `run_id` of the run that opened that gate. Measure that the gate rows `answer` reads after
    Checkpoint 8e (N6's `t1_gate_transition` and its `opened` row) carry both. If neither carries the opening
    run's `run_id`, STOP. That carrier is `s8e`'s, and this stage does not guess the value. *Settled 2026-09-24
    (final consistency pass):* `s8e` §8e.7 now specifies the carrier: *"The `opened` row of `t1_gate_transition`
    carries the opening stage run's `stage` and its `run_id`"*. They are columns of that row, not members of
    `gate_opened`'s payload, and `answer` reads the pair from its token's `opened` row. So the STOP now fires only if
    Checkpoint 8e's `opened` row lacks either column. One residual is `s8e`'s and stated there: an `opened` row that
    `doctor --rebuild` re-derives has a null `run_id`. An attach answered after a deleted database therefore writes
    `run: null`. *Decided 2026-09-24 (D21-48 X11):* *"Accepted as a stated residual. RK-D-UNREAD reads null as an
    unknown run, never as a match. The `Gate:` grammar does not widen."* The render record states the case, cites
    X11, and does not invent a value. `RK-D-UNREAD` passes that row over, and §8f.5's widened `run_of` keeps doing so
    (`doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match`).
15. **The worst-case path bound the landing must respect (D21-47, review m12).** `s8d-ii`'s worktree record states
    the bound `len(root) + 20 + len(item) + 11 + 1 +` the longest path tracked at the fork commit, computed on the item
    being created. It measured 255 of the 259 characters that `MAX_PATH` leaves for a 64-character item, and
    `core.longpaths` is unset. *(Re-read 2026-09-24, final consistency pass: `s8d-ii`'s fix round replaced the
    constant 64 with the item's own length.)* That bound reads tracked paths at the fork, and a landing adds
    tracked paths afterwards. The managed directory repeats the item (§8f.6):
    `.redkiln/artifacts/<item>/<file>` is 19 + 64 + 1 characters before its file name. Read the bound's
    formula and its constant from the record as `s8d-ii` wrote it. §8f.4's length rule uses those values and
    derives neither.

### 8f.3 The three records

Each record is written in ADR-0085's form: frontmatter, context, drivers, `### Option — <name>` headings (D21-42 24a
enforces these by lint from ADR-0164 on), decision, consequences with a **What this forbids** block, and evidence.
Each is committed `proposed`. Each flips only on the operator's dated line under `spikes/DECISIONS-2026-09-21.md`
(ADR-0126's route). *Amended 2026-09-24 (D21-47):* Checkpoint 8f locates each record **by the id it took at
writing**. That id is written into the block's `REC_LANDING`, `REC_RENDER` or `REC_TELEMETRY` line by a dated edit,
in the commit that writes the record. Title phrases collide: `s8g`'s tag-release record also says "landed commit".
So a title phrase is only a working name. The flip is the operator's dated ledger heading,
`## D21-<n> · ADR-<id> … flips to `accepted``. Every record this stage writes that supersedes in part adds its id
to the superseded record's `superseded_by` (ADR-0092).

| record | working title | decides | relation to the accepted set |
| --- | --- | --- | --- |
| **the landing record** | `landed commit` | §8f.4 whole: what "landed" means; the step-grain boundary act (N10) and the boundary checkpoint; how a step checkpoint names its key for `reconcile`'s heal; the staging rule and its baseline; the denylist, the size limit, the path-length bound and the form of the refusal; how each commit is made; what "due" means, and the gate's tree at a refused landing; no integration; the hand-off; the discarded set at re-entry | Execution under ADR-0105 clauses 3–4 and ADR-0109 decision 2, as D21-41 (b) says, as D21-43 N10 widens it. It records that N10 replaces (b)'s *one* content checkpoint per boundary. It must **state its reading** of ADR-0105's forbids bullet *"forbids including untracked files in it"*, and its reading of clause 4's *"when a checkpoint commit is due"* (§8f.4: a landing that refuses lands nothing, so none is due). **Supersedes in part: ADR-0105 clause 5** (*"Anything not listed is a successor"*) and its bullet *"Forbids a wrapper write not on clause 5's list"*, for three writes: `commit-tree`, `update-ref refs/heads/rk-run/<item>`, and `read-tree HEAD` on the run-owned worktree's own index. D21-47 reads clause 5 one way: *"A git WRITE not on its closed list is a supersession in part"*. Clause 3's *"plumbing is `commit-tree` + `update-ref` where the wrapper needs an exact parent"* is the design warrant, and it does not substitute for the supersession. **One consolidated clause-5 list**, each part in its own record with no operation in two: `s8d-ii`'s worktree record (`worktree lock`/`unlock`, the `RK_BASE` marker, `reset --hard`, `clean -fd`); `s8g`'s R3 (`tag`, `apply`); this record (the three above). Reads are not supersessions (D21-47), so `status` and `rev-parse` here are none. Clause 6's *"re-created, never reused"* is `s8d-ii`'s D21-44/W2 record's to supersede (D21-46 R1), and this record cites it |
| **the render record** | `render and write path` | §8f.5–§8f.7 and §8f.9's AC-159 rule: layout-C rendering, section stamps, prose, the drift pass (C2) and its drift gate (N11), where the canonical typed value is kept, the managed directory, the write seam, the read seam (N12), one `artifact.write` per write and one `artifact.read` per read, `.redkiln/items/` as a managed tree, and the unrecorded-mutation rule. Under D21-46 R7 it also decides the `backlog` read and its output shape (§8f.7), the vouched root's store and its advisory drift (§8f.5), the carrier of an attach's path (§8f.2 item 12) and `RK-D-DRIFT-KIND` (§8f.9) | It owns AC-411, AC-477 and AC-478, which are RAISED with no owner record. ADR-0005/0006 (as refined by ADR-0075) and ADR-0116 stand. It drives dated register amendments (§8f.10). It states N11's reading of AC-125 and records that AC-127 holds unamended (§8f.5). It states its reading of ADR-0025 for a vouched root's own change, and ADR-0025 stands unamended (§8f.5) |
| **the telemetry-key record** | `dispatch_completed carries` | §8f.8–§8f.9: the `activity_id_at_version` member and its carrier, the breakdown grammar, the `status` renderer, AC-372's doctor clause read as a renderer test (N12), and `doctor`'s journal source and five-hour reading | **Supersedes in part:** ADR-0131's first forbids bullet, for `dispatch_completed.activity_id_at_version` alone, until 0131's tag is wired: that member is a **payload** member on D21-42 12b's ADR-0163 route. The `item` key is N5's, carried beside the payload by `s8e`'s record under D21-46 R3, which supersedes nothing; this record cites it. It also supersedes in part ADR-0163 decision 4's closed-grammar sentence, decision 5's not-delivered list and its sentence on `doctor`'s journal, and ADR-0163's bullet on *"any other payload correlation key"* for this one member (§8f.8). *Added 2026-09-24 (D21-47, review M5):* it also supersedes in part ADR-0163's bullet *"Forbids reading item ownership off `dispatch_completed` rows, which carry none, or off journal position"*. That covers the first half only: `cost`'s breakdown reads ownership off the `item` key `s8e` carries beside the row. The journal-position half stands. The record quotes the bullet, and Checkpoint 8f (2) reads the quotation |

### 8f.4 The landing checkpoint (D21-41 (b); D21-42 9; D21-43 N10)

**What "landed" means in Phase 1.** A landed commit is a kernel-made content checkpoint commit on `rk-run/<item>`.
After the lane's terminal mark (N9), the diff `<base>..rk-run/<item>` is not empty. **Nothing is integrated.**
Design §7.7's rule stands: *"The queue, not the agent, is the authority on what lands"*. The design §10.2 Phase-2 row
schedules *"project DAG in parallel worktrees plus a merge queue"*. So `GitOp::Integrate` keeps no caller. AC-196 and
AC-197 receive a dated disposition that routes them to Phase 2's `rk-git` merge-queue stage (§8f.10). The operator
makes the hand-off: they merge `rk-run/<item>`, and `explain` prints the command (below).

**When a checkpoint is made — two kinds of point (D21-43 N10).**
- **At every committed step that changed the worktree.** `s8d` commits each agent step under its step-grain key,
  and each `side_effecting` tool step the same way (`s8d` R1; final consistency pass 2026-09-24). An activation
  completes when its `emit` binding commits (`s8d` §8d.2). When such a commit finds the staged tree
  different from `HEAD^{tree}`, the kernel lands it **in the same boundary act**. Through `s8e`'s seam it first makes
  the content checkpoint commit (git-first, D21-42 8a). Only then does it append the step's commit row, whose
  `anchor_sha` is that commit. A step that changed nothing makes no commit. N10 says: *"Land at every committed step
  that changed the worktree. The commit and the content checkpoint are one boundary act."* This replaces the
  draft's single checkpoint "when due" at a boundary, and its void-and-re-run route. A committed step's tree effect
  is never left unlanded, so there is nothing to void.
- **At every tier-1 boundary.** These are a stage's `Story:`, a `Gate:` transition (N6) and the lane's terminal
  `closed` mark (N9). The trailer rides on a content commit when the staged tree differs from `HEAD^{tree}`. AC-245
  requires this: *"when a checkpoint commit is due, the trailer commit's tree"*. When no checkpoint is due, the
  trailer rides on an empty commit: ADR-0109 decision 2's *"a trailer on a checkpoint commit or an empty commit
  carrying the trailer"*, and AC-163's empty commit. **What "due" means is the landing record's stated reading of
  ADR-0105 clause 4 (D21-47, review M6).** A checkpoint commit is due when a landing **will commit**: the staging set
  passes every refusal check, and its tree differs from `HEAD^{tree}`. A landing that refuses any path lands nothing
  (below), so **no checkpoint commit is due** at its gate. Under N10 a boundary usually finds the tree already landed.
  What it still carries is the work of steps that do not commit, since `s8d` §8d.2 says *"A `check` step and an
  **idempotent** `tool` step do not commit"*, and the work done at entry: the render restore (§8f.5) and the vouched
  root's store (§8f.5). *Corrected 2026-09-24 (final consistency pass):* `item_create`'s write is no longer an example.
  `s8d`'s R1 makes a `side_effecting` tool step commit like an agent step (its cases include *"a second item or a
  second tag"*), so N10 lands `item_create`'s write at its own step.

**A step checkpoint carries no tier-1 trailer.** It is neither a stage boundary nor a gate, and design §5.4's tier-1
grammar has no step row. The commit names its step-grain key in its message, so that `reconcile`'s heal can pair a
commit with no row (D21-42 8a: *"`reconcile` heals a commit without a row"*). The landing record fixes that line's
form, and it states that the fold never reads the line as tier-1 state. A crash between the commit and the row is
8a's own window. The heal needs the step's output value as well as its key, so the record names where that value is
read at heal time. If none can be recovered, the run **refuses by name** and names the commit. It never
re-dispatches the step over its own landed edit.

**What is staged.** Two sets are staged: tracked modifications and deletions, and files the run created. Nothing
`.gitignore` excludes is ever staged. *Files the run created* are the untracked files that are not in the
**baseline**. The baseline is the untracked set read at worktree creation, as
`status --porcelain=v1 -z --untracked-files=all` through the seam, and journalled. It is read through the seam so
that a replay is served the recorded set. It is journalled so that a later process can read it, which matters more
now that the worktree is reused across processes (D21-45 W2). Standard excludes are honoured, and an ignored file is
never a candidate. Staged text is normalised by `.gitattributes`: its first row is `* text=auto eol=lf` (ADR-0164),
so a landing's tree is the same on every box. **One function computes this set.** The landing stages from it, and
`s8d-ii`'s re-entry discards exactly it (below).

**What is refused, and how.** Every path to be staged is checked, run-created or tracked, before anything is written.
- **The denylist.** A path is refused if it matches `.env`, `.env.*`, `*.pem`, `*.key` or `id_*`, or one of the
  credential-file patterns the record fixes. The record's list starts with `credentials*`, `*.p12`, `*.pfx`,
  `.netrc` and `.pgpass`.
- **The size limit.** A blob over the declared limit is refused. The limit is a named constant with its warrant, and
  §8f.2 item 7 gives the measured ceiling it is read against.
- **The path-length bound (D21-47, review m12).** A staged path is refused when `s8d-ii`'s worst-case bound would
  exceed 259 characters with it as the longest tracked path and `core.longpaths` is not `true`. The bound is
  `len(root) + 20 + len(item) + 11 + 1 + len(path)`, computed on the landing's own item as `s8d-ii` computes it at
  creation, and read from that record (§8f.2 item 15). `s8d-ii` refuses at creation
  over the paths the fork tracks, and this rule covers the paths a landing adds after the fork. Without it, a landed
  path could make the next creation or the review tree unreachable on a box without `core.longpaths`.

A refusal is **by name**: it gives each path and the rule it broke. It is **never a silent drop**. Halt 7 applies
to anyone tempted to widen a glob or raise the limit to make a run pass.

The *form* of the gate-worthy exit is the landing record's to fix. This file's route, which the record adopts or
replaces with a stated reason:
- The landing lands **nothing**, and the step whose act it was **does not commit**. So no committed step is left
  with an unlanded effect. The landing opens a human gate of class `landing.refused-path`, whose record names every
  refused path and its rule.
- Its `Gate:` commit (N6) is **empty**. No checkpoint is due (the reading above), so the gate's `tree_hash` is ADR-0105
  clause 4's no-checkpoint-due derivation over the run-owned worktree. That is `write-tree` on a temporary index built
  from the current index plus tracked modifications, with untracked files excluded. It is **not** the empty commit's
  tree. The refused step's edits are still in the worktree when `answer` runs, because they are discarded only at the
  next entry (below). So `answer`'s identical derivation gives the same tree, and it cannot void the gate (AC-176).
  `landing::a_refused_landing_is_not_due_and_answer_derives_the_gates_tree` asserts three things, and M48 (the empty
  commit's tree) flips it:
  - the gate's `tree_hash` is not the empty commit's tree when a tracked file is modified;
  - `answer`'s derivation equals the gate's `tree_hash`;
  - the answer is applied, not voided.

  The cost is stated. The derivation writes a refused tracked blob into the object store as a loose object. No ref
  reaches it, and nothing lands it. An untracked refused file never enters the derivation, because clause 4 excludes
  untracked files.
- `run` exits `0` at the gate. That is golden constraint 2, and `crates/rk-cli/src/exit.rs`'s `Success` row: *"**and a
  run that opens a gate** (D-01: AC-403's "exits 0", AC-240, AC-024)"*.
- The gate's question (ADR-0129 decisions 3–4) offers `exclude` and `abort`. `exclude` means every later landing of
  this lane leaves exactly those paths unstaged, and names them each time. `abort` stops the lane.
- The class needs a `gate_expiry` row in `s8h`'s table (§8f.1).

A new exit-table row was considered and rejected: AC-438's table is a closed, snapshotted set, and a gate uses the
one answer channel (ADR-0009) without waiting.

**The cost is stated, not hidden.** The refused step did not commit. At the next entry `s8d-ii`'s reuse discards its
in-flight edits (W2), and the step re-dispatches with the exclusion in force. The record weighs one more option:
salvage the refused landing against its recorded tree.

**How each commit is made — one derivation, not two.** A `Gate:` value carries its tree hash, so the tree must be
known before the commit exists. It must also be *that commit's* tree. The same route serves a step checkpoint:
1. Stage onto a **temporary index** under the common dir. Its name is derived per item, never random: `tree.rs`
   says *"The name is derived, never random"*, and one worktree per concurrent item (Halt 6) makes a fixed name a
   collision. On that index run `read-tree HEAD`, then `add -u`, then `add -- <run-created paths>`.
2. Run `write-tree` on it to get **T**. ADR-0105 clause 5 lists *"`write-tree` on a temporary index"*.
3. Compose the message (with the trailer at a boundary, or with the step-key line at a step) through the existing
   `interpret_trailers_in_place` (which passes `--if-exists=add`).
4. Run `commit-tree T -p <tip>`.
5. Before moving any ref, re-read the new object's tree through `TreeBasis::CheckpointCommit`. It must equal T.
   `trailer/write.rs` cites `s6` §6.2 constraint 4 for the rule: *"anything written by the executable and then read is
   RE-READ, never assumed"*. On a mismatch no ref moves.
6. `update-ref refs/heads/rk-run/<item> <new> <tip>`.
7. Re-sync the worktree's own index with `read-tree HEAD`. `tree.rs` already classes `read-tree` under `GitOp::Add`.

`commit-tree` and `update-ref` are clause 3's named plumbing. Clause 3 is the design warrant, not the licence. They
are also writes that are not on clause 5's closed list, and so is step 7's `read-tree HEAD` on the worktree's own index
(step 1's `read-tree` runs on the temporary index, which `tree.rs` already does under `GitOp::Add`). D21-47 reads
clause 5 one way: *"A git WRITE not on its closed list is a supersession in part. That covers s8f's
`commit-tree`/`update-ref`"*. So the landing record **supersedes ADR-0105 clause 5 in part** for these three writes
(§8f.3), and ADR-0105 gains its id in `superseded_by`. The record names each `GitOp` variant, because *"adding a
variant is that decision made visible"* (`git_exec.rs`). `status` and `rev-parse` are reads, and they supersede
nothing.
Every one of these operations goes through `s8e`'s seam, reads included. On replay the recorded T and commit are
served from N7's `git_op` rows, and `rk-git` performs **zero** writes (ADR-0105 clause 1).

**The landing's writes stay inside `s8e`'s boundary module (AC-518; final consistency pass 2026-09-24).** `s8e`'s
AC-518 lint, `tier_one::every_run_commit_resolves_through_the_worktree_resolver`, puts every code-line call of
`commit_with_trailers` under `crates/rk-cli/src/` and `crates/rk-kernel/src/` in `runtime/tier_one.rs`. `s8e` scopes
it to the commit write: *"The file-write and index-change halves are the landing's, and `s8f` claims them"*. The
landing commits with `commit-tree` and `update-ref`, not `commit_with_trailers`, so the lint as `s8e` leaves it
cannot see a landing made anywhere else. This stage **widens that lint**, which is amended and not added:
- every code-line call of `rk-git`'s landing primitive under `crates/rk-cli/src/` and `crates/rk-kernel/src/` is in
  `runtime/tier_one.rs`;
- the landing record's `commit-tree` and `update-ref` `GitOp` variants are named on code lines only inside that
  primitive, under `crates/rk-git/src/`.

The step-grain act and the boundary act both reach git from `runtime/tier_one.rs`, so nothing lands around the
function `s8e` built. Mutant M51 moves the landing call into a helper outside that file, and the widened lint must
turn red. Checkpoint 8f (4) selects the lint by name.

A rejected alternative: stage the worktree's real index and run porcelain `git commit`. That needs `write-tree` on a
real index, which clause 5 does not list. It also derives T from equal inputs rather than from the object committed.

**`answer` does not land.** After a landing, the owned worktree's no-checkpoint-due tree equals `HEAD`'s tree, so the
answer's derivation matches the gate's. An answer's effect on an artifact, such as the drift gate's
`move-to-prose` (§8f.5), is performed by the next `run` at entry and lands there. A refusal or halt in mid-step does
not land. Re-entry's discard (below) is what keeps that sound.

**The gate's tree — the repair the packet bundled with (b).** `gate_opened`'s `tree_hash` is the landing's T, taken
over the run-owned worktree when the gate opens. It is no longer `prepare_inputs`' stage-start `WorkingState` over
the checkout. `prepare_inputs` itself hashes the run-owned worktree, since that is the tree the stage's inputs come
from. `run_id` is `format!("{}-{}", stage, &digest[..16])` over inputs, so its *values* move and its *grain* does not
(ADR-0152). §8f.2 item 5's measurement also covers whether a pinned fixture depends on the old basis.

**The hand-off.** `explain <item>` gains a landing block after the recorded resolution. The block is read from git
through `GitOp::RevParse` and from the tier-1 rows' `anchor_sha`, and it never computes a selection (AC-432). It
prints the tip of `rk-run/<item>`, how many commits it is ahead of the base and how many of them carry content, and
the **exact** operator command, which this file's route gives as `git merge --no-ff rk-run/<item>`. The record fixes
the command and the lines, and `explain::explain_prints_the_landed_commit_and_the_merge_command` pins them.
**No squash and no rebase**, and that is not style. Tier-1 state is keyed by commit SHA, never by path (ADR-0018;
design §5.5). A squashed hand-off would orphan every trailer the run wrote.

**Re-entry over a reused worktree (D21-44; D21-45 W2; N10).** The draft's re-entry section decided around
`recreate_run_worktree`'s teardown and left a void route open. Both premises are gone. The worktree is
`.redkiln/worktrees/<item>`, and W2 decides re-entry: *"Re-entry reuses the registered worktree when its HEAD equals
the `rk-run/<item>` tip. Uncommitted in-flight changes are discarded explicitly and journalled, which is N10's
semantics. A tree without `RK_BASE`, or with commits the run ref lacks, is **refused, never removed**."* `s8d-ii`
owns the reuse and the refusal, and `s8e` the discard's journal row (D21-46 R1). This stage owns what lands, so it
fixes three things:
1. **The discarded set is the staging set.** What re-entry discards is exactly what a landing would have staged at
   that moment: tracked modifications, plus untracked files less the baseline, honouring ignores. One function
   computes it (above), and it is printed path by path. It is never silent.
2. **Only an uncommitted step can lose work, and it has no commit to serve.** Under N10 every committed step's tree
   effect is on `rk-run/<item>` before its commit row exists. So what re-entry discards belongs to a step with no
   commit, and `s8d`'s walk re-dispatches it (*"the first step with no commit executes"*). This closes the draft's
   lossy resume (Halt 4) without a void record, and AC-013 stands unamended.
3. **A human's edit to a run's artifact is a commit on `rk-run/<item>`.** An uncommitted edit left in the run-owned
   worktree is in-flight dirt: it is named and discarded at the next entry. A committed edit reaches the render pass
   (§8f.5). There prose is accepted, and structure is surfaced, restored and gated.

### 8f.5 `rk-store`'s render, write and read path (D21-42 5a; D21-43 N11, N12)

**Layout C, rendered, never parsed.** Human-facing kinds are rendered as layout C (ADR-0006, and ADR-0075, which
removes `plan.md`): typed frontmatter, read-only digest-stamped structured sections, and opaque prose sections. Each
section is a pure function of *(typed value, template version)* (AC-121, AC-123). Each structured section carries a
stamp over its bytes and the template version, so AC-128's template-drift kind is buildable later. The stamp's form
is the record's, and it must not be a line the locator reads as a heading. An HTML comment is already one of
`locator.rs`'s decoys. Machine-only kinds are canonical JSON sidecars rendered to `.md` on demand (AC-144). **Nothing
parses structure out of markdown**, and `store::locator` stays the one module that looks at it (AC-120).

**Where the canonical typed value lives — settled by the render record.** The value is a canonical JSON sidecar
(`rk_store::canon::canonical`) in the item's managed directory (§8f.6). Only `rk-store` writes it, and the landing
commits it. So the value survives the deleted-database test (ADR-0109 decision 2), and it is read back *"from the
store, not from the markdown"* (AC-477). Parsing it out of the `.md` stays forbidden (ADR-0005). A store table was
considered and rejected: after a deleted database no git fact could rebuild it.

**C2 — surface, restore, and gate (D21-42 5a; D21-43 N11).** At every entry, after `s8d-ii`'s re-entry discard and
`reconcile_at_entry`, `rk-store` passes over the item's managed directory in the run-owned worktree. It does this for
each structured section and for the typed frontmatter:
- It locates the section by anchor (the locator's byte range) and compares the bytes on disk with the rendering of
  the typed value.
- On a difference it appends **one `structured_drift` event**. Design §5.4's row gives the payload *"section or root,
  old digest, new digest, the causing commit"*; the artifact, the anchor and both digests are AC-124's. **Then** it
  re-renders the section from the type (AC-411: *"the next render restores the value from the type"*).
- The restore is a write, so it appends one `artifact.write`.
- **Then the kernel opens a drift gate** of class `render.structured-drift`, git-first (N6). Its `Gate:` commit is the
  checkpoint that lands the restore. N11: *"`structured_drift` opens a drift gate that blocks the stage's exit until
  it is answered."* The stage's steps continue to dispatch, because the typed value is already whole. What is blocked
  is the boundary. **This refusal is this stage's (D21-47, cross #16). `s8e` builds the stage advance and does not
  specify it.** This stage adds one check to `s8e`'s advance, before the advance's boundary act. The check folds the
  item's gate transitions (`t1_gate_transition`, D21-46 R2; N6) and refuses to leave the stage while any gate of
  class `render.structured-drift` for the item at this stage has an open state as its latest. The refusal is by
  name: it names the class, the gate's token and the drifted artifact. No boundary commit is made, and no row is
  appended. When the stage has nothing left to run, `run` exits `0` at the gate, like any gate. The class needs a
  `gate_expiry` row in `s8h`'s table (§8f.1). `landing::a_drift_gate_blocks_the_stage_exit_until_it_is_answered`
  proves the refusal. M47, which removes the check but still opens the gate, flips it, and M23, which opens no gate,
  flips it too.
- A prose section edited on disk is accepted whole as the field's new value, with no drift record and no gate
  (AC-152).

**The drift gate's two answers.**
- `confirm-restore`: the restore stands, and the gate closes.
- `move-to-prose <section>`: the drifted bytes are taken from the causing commit named in the event's payload, not
  from the event, which carries digests only. `rk-store` appends them to the named prose section through the writer,
  as one `artifact.write`, at the next entry. The move puts the text in opaque prose, so no drift text is
  deserialized into a typed field (AC-125's architecture test).

Either answer is a typed write through a gate answer, which is AC-125's *"resolving the drift requires a typed write
through the CLI, web or a gate answer"*. N11 reads AC-125's "resolving" as answering this gate, and a dated reading
note records that (§8f.10). **AC-127 holds unamended**, since *"the same change on a store-rendered structured section
does block until resolved"* is now true. Uncommitted edits never reach this pass, because re-entry discards them
first (§8f.4).

**AC-127's vouched-root half (D21-46 R7).** AC-127 reads: *"GIVEN a human-supplied (vouched-root) artifact whose
content has changed since it was vouched / WHEN the store reconciles it / THEN the change is recorded as advisory drift
and does not block the stage / AND the same change on a store-rendered structured section does block until
resolved."* Its *Verified by* reads: *"a table-driven test over {vouched root, rendered section} × {changed,
unchanged} asserting the four outcomes; a `doctor` rule-table row distinguishing the two drift kinds."* The C2 pass
above is the second clause, and this is the first. ADR-0025's decision reads: *"a human-supplied artifact is recorded
with `derived_from = []`, and this **never blocks or re-runs**. When an upstream artifact in the same stage
subsequently changes, the *next* gate the item reaches carries an advisory note"*. Its forbids block reads:
*"Blocking or re-running work on a human-supplied artifact purely because `derived_from` is empty or because an
upstream changed"*, and *"Silently doing nothing when an upstream a human artifact depended on changes — the advisory
is mandatory, even though it does not block."*

**The reading the render record states.** ADR-0025's decision names an *upstream* change. AC-127's GIVEN names the
root's *own* content, and it cites ADR-0025 as its decision. `rk-types` already carries the variant, and its doc reads
*"A human-supplied (vouched-root) artifact changed since vouching. **Advisory: does not block** (AC-127)"*. So the
record reads ADR-0025's rule as governing both changes: the drift never blocks, never re-runs and is never silent. The
upstream half is AC-525's, built at `s7b` in `rk-kernel`'s `runtime/advisory.rs`. This stage builds the own-change
half on that module, and it does not re-claim AC-525.

- **The root is stored at the next entry.** ADR-0129 decision 4 reads *"accepted and stored as a human-supplied
  root"*. Today `answer`'s `store_attachments` journals the digest and stores no file. `answer` does not land
  (§8f.4), so the store happens at the next `run`'s entry, as `move-to-prose` does. `rk-store`'s writer reads the
  attached bytes at the path the attach recorded (§8f.2 item 12 measures the carrier's cost). It checks that their
  digest equals the attach's, then writes them into the item's managed directory as the root's file, with one
  `artifact.write`. A mismatch is **refused by name**, naming the kind, the path and both digests, and nothing is
  stored. The entry's boundary lands the file (§8f.4). The **vouched digest** is the attach's, and nothing else
  moves it.
- **The pass.** It runs in the same entry pass as C2, after `reconcile_at_entry`. For each vouched root in the managed
  directory it compares the file's digest with the vouched digest. A vouched root is not layout C: it has no section,
  and the locator never reads it, so here the whole file is the instrument. That is the opposite of AC-159's rule,
  which is why §8f.9 scopes vouched roots out of it.
- **Changed.** The pass appends one `drift_advisory` in AC-525's four-field shape, `{ root, upstream, old_digest,
  new_digest }`. `upstream` is `null`, which marks the root's own change. `old_digest` is the vouched digest and
  `new_digest` the file's: `Drift::Advisory`'s `vouched` and `observed`.
  - **The type change (D21-47, review m11).** At `fc80687`, `rk-kernel`'s `runtime/advisory.rs` declares
    `upstream: String`, `upstream() -> &str`, and `triple() -> (String, String, String)`. `from_signal` is documented
    as *"**The only constructor**"*. This stage makes four changes:
    - `upstream` becomes `Option<String>`, `upstream()` returns `Option<&str>`, and `triple()` returns
      `(String, Option<String>, String)`. So the own-change key `(root, None, new_digest)` is never equal to an
      upstream key.
    - A second constructor is added, `Advisory::own_change(root, vouched, observed)`. The doc of `from_signal` is
      corrected in the same change, so that it names both constructors.
    - `to_json` writes `"upstream": null` for `None`. The key stays present, which keeps *"all four fields and no
      fifth"* (`Advisory::FIELDS` is unchanged). `Some(s)` serialises as the same string it does today, so no
      existing `drift_advisory` row, golden log or `carried_by_gate` payload changes a byte.
    - The regressions are AC-525's existing tests, which must pass unedited: `advisory::ac525_drift_advisory_once`
      in Checkpoint 8f (4), and the whole suite.
      `advisory::an_own_change_advisory_writes_upstream_null_with_all_four_fields`
      is new. Design §5.4's row also names *"the causing
  commit"*, and the render record states whether this case carries it. The advisory goes through `AdvisorySink`, so
  it is appended once per `(root, null, new_digest)` per run (AC-525's amendment (3)). The next `gate_opened` in the
  run carries it through `carried_by_gate`, which is ADR-0025's *"the next gate the item reaches"*. **Nothing else
  happens.** Nothing is restored or rewritten, no gate opens, no step re-runs, and the root stays `Present`
  (AC-067). The stage advance's drift refusal (above, this stage's) refuses only on an unanswered
  `render.structured-drift` gate, so the stage exit is not blocked.
- **Unchanged.** Nothing is appended.
- **Re-vouching is an attach.** The advisory does not move the vouched digest. So a later run over the same change
  appends one more advisory, which is the run-scoped cost AC-525's amendment (3) states. To re-vouch, the operator
  attaches again through `answer`.
- **Readers.** A root has no `emit` binding, so the read seam's check against a committed `emit` digest does not
  apply to it. The seam serves the root's bytes on the run ref with their digest, and it appends its one
  `artifact.read` as for any kind.
- **Which edits reach the pass.** An edit to the root is a commit on `rk-run/<item>`, as §8f.4's re-entry item 3
  says of any run artifact. An uncommitted edit is discarded at re-entry, before this pass runs.

**No artifact type carries a drift field or a signature-comparison field.** AC-478's *Verified by* keeps that
assertion. The one clause of AC-478 that falls is *"AND no comparison of stored-vs-regenerated rendering is
performed, so no drift state exists to report"*, by a dated amendment (§8f.10). The drift gate is a gate row, not an
artifact field.

**The single write path (AC-122).** The writer is a struct seam in `rk-store`, on `GitOps`' precedent: *"A struct,
never a trait"*, since Checkpoint 7a counts the port traits and a new one would move the port surface. It is
journalled, and on replay it performs zero writes and serves the recorded digest. No filesystem-write handle into a
managed directory is exported from `rk-store`. A `trybuild` compile-fail case proves that a sibling crate cannot
write there.

**One `artifact.write` per write.** Each write appends exactly one row with these members:
- `kind` and `item`;
- `stage` and `run`, added by D21-47 (review M2);
- `schema_version`, which is the kind's registered `schema` string or `null`, per design §5.4's R-49 note;
- the content `digest`.

The digest comes from the same `rk-types` function the typed-artifact digests use (ADR-0116 decision 6: *"No second
digest algorithm enters the system"*). That function is the one `answer.rs` already calls.

**`stage` and `run`, and why the reader needs them.** `RK-D-UNREAD` keys on `str_field("stage")` and counts runs from
`payload.get("run")`. A row that lacks either is passed over by a `continue`, and nothing reports it (§8f.2 item 14).
- **The values, fixed by the render record.** `stage` is the stage whose step wrote the row. `run` is that stage
  run's `run_id` (`format!("{}-{}", stage, &digest[..16])`, ADR-0152). For `answer`'s attach, `stage` is the answered
  gate's stage, and `run` is the `run_id` of the run that opened it. Both are read from the token's `opened` row in
  `t1_gate_transition`, which carries them (`s8e` §8e.7; §8f.2 item 14).
- **`answer`'s producer gains both members in this stage's commit.** Until it does, its rows are invisible to the
  rule.
- **The rule's `run_of` widens.** It reads `run` as a string or as an integer. The key is the JSON value's canonical
  text, so an integer `0` and a string `"0"` stay distinct. **A `null` `run` is an unknown run, never a key**
  (D21-48 X11: *"RK-D-UNREAD reads null as an unknown run, never as a match"*). `run_of` returns no run for `null`,
  as for an absent `run` or any other JSON type, so the row is passed over as today. The canonical text of `null` is
  never a run: two null-run rows are not one run, and a null-run row is not any named run.
  `doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match` pins it. Its hand-built journal, at `unread_window_runs = 2`, writes one
  unread `(stage, kind)` once under a string `run` and twice under `run: null`, and the rule is silent: one known
  run, below the window. Its control arm adds a second string `run` to the same journal, and the rule fires. A
  `run_of` that keyed `null` by its text would count two runs in the first arm and fire, and mutant M52 is exactly
  that. *(D21-48 (2026-09-24).)* `s7d`'s integer fixtures therefore read as before, and
  `ac426_ac472_unread_window_49_not_flagged_51_flagged` and `ac426_rule_fire_leaves_write_set_unchanged` are this
  change's regressions in Checkpoint 8f (4).
- **The test.** `doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes` runs a `FakeHarness`
  lane with an emit, a resolved `artifact:` input and an answered attach. It asserts that every `artifact.write` and
  `artifact.read` row carries a string `stage` and a `run`. It then runs `RK-D-UNREAD` over that journal at
  `unread_window_runs = 1`. The emitted-and-read kind is silent, and the attached kind, which nothing reads, fires.
- **The mutants.** M44 drops `stage` from the writer's row. M45 drops `run` from `answer`'s. Each silences a finding,
  and each must flip the test.

**The read seam (D21-43 N12; D21-47).** The reader is the write seam's sibling in `rk-store`, and it is a struct too.
It serves an artifact's typed value from its sidecar, together with the value's digest. It appends **one
`artifact.read`** carrying `kind`, `item`, `stage`, `run`, `schema_version`, the tree hash it served (the run ref's
tree the bytes were read from) and the digest. Those are design §5.4's *"kind, item id, `schema_version`, the tree
hash served"*, plus the two keys `RK-D-UNREAD` reads. That is AC-472's first clause: *"an `artifact.read` event is
emitted naming the artifact kind and id"*. It **refuses** a read when the current entry has no reconcile record. That
is AC-126's *"`reconcile()` runs before any read of the artifact is served"*, and no flag, cache or fast path skips
it. **The seam is the only emitter** (D21-47: *"`artifact.read` has exactly one emitter: rk-store's read seam
(s8f)"*). `s8d`'s resolver *calls* it: every resolved `artifact:` input reaches it through the one call that `s8d`'s
kernel function, `runtime::reference::read_artifact_input`, makes to its `ArtifactReadSeam`. At Checkpoint 8d that
value is the non-emitting stub `UnwiredReadSeam`, and this stage re-points the composition root's value to the seam
(§8f.2 item 10; the names are `s8d`'s, final consistency pass 2026-09-24).
The resolver appends nothing itself. The seam checks that the digest it serves
equals the committed `emit` binding's digest, which is what `s8d`'s arm reads. A mismatch is a halt (Halt 4), never a
silent choice of one. On replay the seam serves the recorded digest and appends nothing new. Once rows exist,
`RK-D-UNREAD` (AC-472's second clause, built at `s7d`) reads real ones. Today it reads none, because nothing
produces them.

### 8f.6 Where rendered artifacts land, and the run-path `artifact.write` producer (D21-42 5b)

**The managed directory — settled by the render record.** An item's rendered artifacts are written by `rk-store`
only, under `.redkiln/artifacts/<item>/` inside the **run-owned worktree** (ADR-0137 clause 3: *"**`project_root` is
the run-owned worktree**"*). From the checkout that is `.redkiln/worktrees/<item>/.redkiln/artifacts/<item>/`
(D21-44). The item appears twice, and that is kept on purpose (D21-47, review m12). The inner `<item>` is what keeps
two items' artifacts apart once the operator merges their run refs into one tree (§8f.4's hand-off). Dropping it
would make every merge a collision. The cost is length. `.redkiln/artifacts/<item>/` is up to 19 + 64 + 1
characters, before the file name, inside a run tree whose root is itself up to 39 + 64 characters. §8f.4's
path-length bound refuses a staged path that would exceed `s8d-ii`'s worst case, so the doubled item is
paid for by a named refusal and never by an unreachable checkout. The path is excluded from nothing but itself, and
this file reads that as follows:
- No `.gitignore` row covers it. At `a6349b6` the `.redkiln/` rows are `.redkiln/telemetry/` and
  `.redkiln/worktrees/`, and the comment reads *"`.redkiln/` is tracked as of Step 7e"*.
- The landing's staging rule does not skip it, and no loader or `doctor` walk skips it inside the tree it is in.
- The one exclusion that touches it is D21-44's rule that every recursive walk over the checkout's `.redkiln/`
  skips `worktrees/`. That rule covers the run-worktree copy of the directory itself.
- So the store reaches that copy by the path in its `worktree_assigned` row (`s8e`), and never by a walk.

The landing (§8f.4) commits the directory to `rk-run/<item>`. The checkout's own tree is never written (AC-518).

**The producer.** The `emit` arm stops being inert. It hands the value `s8d`'s resolver bound to the writer, and the
writer appends the `artifact.write`. This executes ADR-0116's forbids bullet *"Forbids an activity output with no
`artifact.write` event"*. It also executes design §5.4's producing step, which R-48 widened to *"every activity
output, **and `redkiln answer` for an admitted attachment**"*. `answer`'s producer is unchanged. The emit's commit is
a committed step that changed the tree, so N10 lands it in the same act (§8f.4).

**What `s8` reads, and this stage proves over a fixture.** `landing::emitted_artifact_digest_matches_its_bytes_on_the_run_ref`
runs a `FakeHarness` lane through `verbs::run::run`. It asserts that the emitted artifact's `artifact.write` digest
equals the `rk-types` digest of the bytes at `rk-run/<item>:<path>`. Those are D21-42 5b's two halves, and `s8j`
writes them into (1b).

### 8f.7 `DebtRegisterEntry`: the in-repo item write (D21-42 6a, this stage's half), and the `backlog` read (D21-46 R7)

`debt-logging` runs `extract` (an agent) and then `file`, with `run = "toolchain:item_create"` and
`inputs = { items = "${extract.outputs.items}" }`. D21-42 6a: *"DebtRegisterEntry is an rk-store item write with
provenance; human-sourced facets are null and go to `unknown_facet`."*

- **The verb row.** `s8c` lands `item_create` on its `store` route with no handler: *"Refused naming the route until
  `s8f`"*. This stage builds the handler. `s8c`'s record V owns the Rust verb handlers as a class, and this one is
  the `store` route's. The kernel serves the row into `rk-store`'s writer, **never** a `Toolchain` adapter. An
  adapter that wrote `.redkiln/items/` would be a second write path.
- **The managed tree.** The render record reads `.redkiln/items/` in the run-owned worktree as a second
  `rk-store`-managed tree. AC-122's *"a path under a managed artifact directory"* is read to include it, and the
  record states that reading. The landing commits it.
- **The file.** It is `<id>.toml` in `ItemRecord`'s key set (`id`, `kind`, `stage = "triage"`, `[facets]`), plus a
  `[provenance]` table. The table names the producing activity at its version, the run, and the source `ImplReport`
  digest. `ItemRecord` is `#[derive(Debug, Clone, Deserialize)]` with no `deny_unknown_fields`, so today's loader
  tolerates the table.
- **An existing id — found in revision.** The draft said an existing id is *"refused, never overwritten"*. But
  `file` can run again after its file has landed. An `on_false` loop-back (D21-42 4d) re-executes it under a new
  iteration path, and it meets the file it wrote. *(Corrected 2026-09-24, final consistency pass: the draft also named
  re-entry, but `item_create` is a writing verb, which `s8d`'s R1 marks `side_effecting` (*"a second item"*). So
  re-entry serves its committed step and does not re-execute it. The loop-back is the case that remains.)* So the
  rule is split in two. A file byte-identical to the rendering is **served**: nothing is written and no
  `artifact.write` is appended. A file that differs is **refused by name, never overwritten**.
- **Facets.** A facet whose registry `source` is `human` (or `llm`) is **never written**. That holds even when the
  extracted value carries one: golden constraint 1, and DECISIONS-2026-09-12 #15. A withheld value is **named** in
  the provenance table, not dropped silently. `build_item` already nulls a declared facet the record omits (*"Every
  declared facet the record does not carry, as an abstention"*). So when a minted item is run, the selection
  abstains and opens an `unknown_facet` gate. `item_create::a_minted_debt_item_opens_an_unknown_facet_gate_when_run`
  asserts it through the binary. Its temporary repository's `[adapters] harness_binary` names the fixture child,
  `rk-fixture-child-claude` (the test reads it through `env!("CARGO_BIN_EXE_rk-fixture-child-claude")`, the
  `pack-min` fixtures' route; D21-47), so no real `claude` can be spawned. It also runs inside Checkpoint 8f's wrapper
  (the fake `claude` and the trap).
- **No artifact kind is minted.** `ItemRefs` is a value, and the artifact registry is not widened.
- **`debt-logging` walks past `file` once the handler lands (D21-47, cross #6).** At Checkpoint 8d, `debt-logging`
  stops at `file` in both lanes, on `item_create`. `s8d` §8d.9 records the owner: *"`debt-logging`: `item_create` —
  `s8f` (6a)"*. This stage's handler is what moves that row. The same dated line that flips `closure-record` (§8f.10)
  flips `debt-logging`'s two rows to **walks**. `landing::debt_logging_walks_past_file_through_item_create` proves it
  through `verbs::run::run`. A `FakeHarness` `extract` outputs two items, the activity walks past `file`, and the
  run ref holds two item files. M46 leaves the row refusing, and it flips the test.

**The `toolchain:backlog` producer (D21-46 R7).** R7 reads: *"s8f builds the `toolchain:backlog` producer (a
store-route read of items)"*. `s8c` §8c.4's feed row hands it on: *"Its only honest producer is a `store`-route read of the
item tree, which `s8f` builds under D21-46 R7"* (re-quoted 2026-09-24 by the final consistency pass: `s8c`'s draft
said no stage built one). `closure-record`'s `audit` step reads it
as a mandatory input (`backlog = "toolchain:backlog"`), and so does `problem-record`'s `draft`.

- **The verb row.** `backlog` takes the `store` route. It outputs `BacklogSnapshot`, declares no `inputs`, and is
  `live = true`, because what it reads moves. Its other columns follow `item_create`'s row, which is the route's
  first. `s8c`'s route set is closed, and it describes the `store` arm as *"an rk-store write"*. This row is the
  arm's first read, and record V's class of handlers takes it as it takes `item_create` (above). No route is added.
- **The feed rows.** `[feed.rust.backlog]` and `[feed.node.backlog]` change from `unprovided = "…"` to
  `verb = "backlog"`. `s8c`'s loader refuses a feed `verb` that is not a registry row, so the verb row and the two
  feed entries land in one commit. The value does not depend on the toolchain, and no `Toolchain` adapter is
  called, because the `store` route is the kernel's.
- **What it reads: the item records the store names, and nothing else.** An id is named when a tier-1
  stage-transition row carries it as `item`, or when a journalled `ItemRefs` output of an `item_create` step lists
  it (the write above). Each named id is read at `.redkiln/items/<id>.toml` in the run-owned worktree **by its exact
  path**. That is `doctor --rebuild`'s precedent in `rebuild.rs`: *"Read `.redkiln/items/<slug>.toml`'s `stage` for
  every slug the trailers committed."* The producer **never walks** `.redkiln/` or `.redkiln/items/`. So a file that
  no row names is not in the feed, and D21-44's `worktrees/` exclusion holds by construction, as it does for
  AC-159's rule (§8f.9). The read goes through `rk-store`, so the item tree's one reader lives in its one writer's
  crate.
- **The output shape.** `BacklogSnapshot` is a value in `rk-types`, serialised through `rk_store::canon::canonical`.
  - `items` is a list of entries ordered by `id`. Each entry carries `id`, `kind`, `stage`, `facets`, `provenance`,
    `record_digest` and `named_by`.
  - `facets` is the record's `[facets]` table verbatim. An absent facet stays absent and is never defaulted.
  - `provenance` is the `[provenance]` table the write above adds, or `null`.
  - `record_digest` is the `rk-types` digest of the record's bytes.
  - `named_by` is a sorted subset of `stage_transition` and `item_write`.
  - `absent` is a list, ordered by `id`, of every named id with no record in this worktree, each with its
    `named_by`. A named id is **listed absent, never dropped**. An item minted on another lane's run ref is absent
    here until the operator merges that ref (§8f.4), and the audit reads that as a fact, not as an error.
- **What it costs, stated.** A journalled `ItemRefs` output is tier 2. After the deleted-database test, an item that
  was minted and never run drops out of the feed until it is run. Tier-1 names survive, because `doctor --rebuild`
  refolds them from `Story:` trailers. The render record states this cost.
- **What it does not do.** An item record is not an artifact kind, so the read appends no `artifact.read`, and
  `RK-D-UNREAD` never reads it. On replay the step's resolved input is served from the journal, as `s8d` serves
  every resolved input, and the read touches no file.

**The unprovided feeds, and which activities stop on each (D21-46 R7).** R7 reads: *"Unprovided feeds refuse by name,
and the walk table records each activity that stops on one."* `s8c` §8c.4's resolution rule has two halves, and each
has its own authority:
- A **mandatory** `unauthored` or `unprovided` key is refused by name. That is N14.
- An **optional** one is `null`. That is ADR-0032's own Decision, *"Absent `?` inputs are `null`"*. It still stands:
  ADR-0103 supersedes ADR-0032 *"**in part**"*, and it withdraws none of its rules *"except 0032's "vacuously true" for
  `any`"*.

ADR-0103 decision 4 is predicate `null` semantics, not this rule (D21-47, review M7). The citation reverts to
ADR-0032 in `s8c`, `s8d` and here. R7 does not turn an optional `?` input into a refusal. So an activity **stops** on
a feed exactly when it reaches an `unprovided` feed through a mandatory input.

The measurement is over the step `inputs` of the 102 files in `.redkiln/catalog/` at `c5d19d7`. The three feeds
`s8c` declares are reached **six** times, in six activities, and **two** of the reaches are mandatory. The lane
column reads `s8d` §8d.9's drafted walk tables.

| feed | activity/step | input | selected in a Phase-1 lane | stops on it before this stage | after this stage |
| --- | --- | --- | --- | --- | --- |
| `backlog` | `closure-record/audit` | mandatory | the fix lane's and the story lane's `closeout` | **yes**, in both lanes: refused by name at `audit`, naming `toolchain:backlog` and its `unprovided` reason | resolves through `backlog`, and the activity walks past `audit` |
| `backlog` | `problem-record/draft` | mandatory | neither | it would, if selected | resolves through `backlog` |
| `logs` | `root-cause/hypothesise` | optional (`logs?`) | the fix lane's `analysis` | no: `null` | unchanged: `unprovided`, so `null` |
| `sbom` | `sca-scan/scan` | optional (`sbom?`) | both lanes' `verify` | no: `null` | unchanged |
| `sbom` | `exploitability-analysis/sca` | optional (`sbom?`) | neither | no | unchanged |
| `sbom` | `dependency-update/rescan` | optional (`sbom?`) | neither | no | unchanged |

**Handed to the walk table's owner, by citation.** `s8d` §8d.9 owns the walk table, and this list is handed to it.
`s8d` closes before this stage, so at Checkpoint 8d **`closure-record` stops at `audit` in both lanes**. It is refused
by name on `toolchain:backlog`, and its owner is this stage. `s8d` §8d.9's drafted rows said it walked, and its claims
read 12 of 15 and 10 of 13. Under the rule above they read 11 of 15 and 9 of 13 at Checkpoint 8d, with a fourth
refusal owned by `s8f`.
- The correction is `s8d`'s, in its own file, and it is made (consistency pass 2026-09-24): `s8d` §8d.9 now claims
  *"11 of 15 walk, and 4 refuse by name"* for the fix lane and *"9 of 13 walk"* for the story lane. It is named here
  so that nobody bends `lane_walk::…` to the drafted row.
- This stage's close flips the two `closure-record` rows back to **walks** by dated line, and the two `debt-logging`
  rows with them (§8f.7 above). The two `lane_walk` tests' expected verdicts move in the same commit (§8f.10).
- After this stage **no lane activity stops on a feed**, and neither `closure-record` nor `debt-logging` stops at
  all. `s8d` §8d.9's claims read **13 of 15** and **11 of 13**, and the two left refusing in each lane are
  `smoke-test` and `rolling-deploy`, which are `s8g`'s (D21-47, cross #6).
  `landing::closure_record_walks_past_audit_on_the_backlog_feed` proves it through `verbs::run::run`.
- `feed::every_shipped_unprovided_feed_is_reached_only_as_optional` pins the list. A later catalog edit that makes
  a mandatory reach of an `unprovided` feed fails it by name.

**Found in revision (R7).** `s8c` §8c.4 cites ADR-0032 for the optional `null`, and ADR-0032's frontmatter reads
`superseded_by: ["0103"]`. The rule is unchanged here, and re-pointing the citation is `s8c`'s. *Settled in the
consistency pass 2026-09-24:* `s8c` now cites ADR-0103 decision 4, and the quotation above reads its current bytes.
*Reversed 2026-09-24 (D21-47):* the re-point was wrong. `superseded_by` marks a supersession **in part**. ADR-0103's
own text reads *"none of their stated rules is withdrawn except 0032's "vacuously true" for `any`"*. So ADR-0032's
Decision is the citation, as the paragraph above now gives it.

### 8f.8 `dispatch_completed`'s interim members, AC-395's breakdown and AC-372's `status` half (D21-42 12b)

**The two members, and whose record each rests on.** `dispatch_completed` carries `item` beside its payload and
`activity_id_at_version` in its payload, each a string or `null` (restated 2026-09-24 under D21-46 R3).
- **`item` is N5's** (D21-43): *"An interim `item` key on every item-scoped event kind, moving to ADR-0131's tag
  later as ADR-0163's interim key does; `query` honours `ByItem`."* 12b's `item` is one case of it.
  D21-46 R3 fixes where it lives: *"**Carried outside the canonical digest**, as ADR-0131 places its tag set."*
  `s8e` §8e.10 carries it beside the event, outside `payload`, so its record supersedes nothing of ADR-0131 (`s8e`
  finding 12), and this file cites that record and does not re-specify it. If Checkpoint 8e's item-scoped kinds do
  not include `dispatch_completed`, this stage adds the key on `s8e`'s carrier, taking its value as the reading's
  is taken: from `ActivitySpec.inputs`' `"item"` (`ITEM_KEY`, whose doc says the key is *"Read for attribution
  only"*).
- **`activity_id_at_version` is this stage's.** It is interim too, and it is a **payload** member, as D21-42 12b
  extends ADR-0163's interim key (*"ADR-0163's interim key is extended to `dispatch_completed`"*). R3 moves N5's
  `item` only. When ADR-0131's tag is wired it moves to a tag, and the sentences allowing it lapse, on ADR-0163's own
  clause. The telemetry-key record therefore supersedes in part:
  - ADR-0131's first forbids bullet (*"Forbids putting an item id, a run id, … inside `rk-types::Event.payload` for
    the purpose of addressing"*), for this one member alone;
  - ADR-0163's bullet *"Forbids reading this record's `0131` supersession as licensing any other payload correlation
    key, on any other kind"*, for this one member alone. N5's `item` is `s8e`'s.
  - ADR-0163's bullet *"Forbids reading item ownership off `dispatch_completed` rows, which carry none, or off journal
    position"*, for its first half only (D21-47, review M5). The breakdown below reads a row's owner from the `item`
    key `s8e` carries beside it. Such a row now carries one, so the bullet's ground, *"which carry none"*, is gone
    for it. A `dispatch_completed` row whose `item` is `null` or absent is owned by no item, on ADR-0163's own bullet
    *"Forbids treating a reading with a `null` or absent `item` as owned by any item"*, applied here to this row. The
    **journal-position** half stands whole. Pairing a completion with a preceding `dispatch` row stays forbidden
    (below).

  Each of ADR-0131's grounds is answered, as ADR-0163 answered them.

**The carrier — settled by the telemetry-key record.** The adapter cannot see an activity id, because `ActivitySpec`
has no such field. The record takes a member on `ActivitySpec` that is outside the prompt and outside every
digest. It is set when the spec is built, and omitted from the serialised form when `None` (§8f.2 item 5). This
leaves `s8d`'s per-step specs on `RecordedSpec` (`s8d` §8d.2) as they are. Each carries the member of its own
activity. Two alternatives were rejected:
- An `inputs` key. It is rejected on ADR-0163's own reasoning, since it would *"move the prompt bytes of every
  existing binding"*.
- Pairing with the kernel's preceding `dispatch` row. That is journal position, and it is forbidden.

**The breakdown (AC-395).** `cost <item>` gains per-activity lines over the `dispatch_completed` rows the item owns:
dispatches, token counters, cache counters, wall clock and `total_cost_usd`. The record fixes the line forms, and
`cost_output_grammar_is_closed` widens to hold them.
- `total_cost_usd` is summed **as decimals**, with no binary float anywhere (ADR-0133 clause 5: *"sum them **as
  decimals** with no intermediate binary floating-point step"*).
- An absent counter is **never zero** (AC-362). A sum over a group with an absent member names how many were absent.
- No line carries a headroom figure (AC-373).
- An item owning neither a reading nor a `dispatch_completed` row prints exactly the one empty line. That is 12a's
  *"no rendering over n = 0"*, and Checkpoint 7i (5)'s assertion.

**The `status` half (AC-372).** `status` prints an item's draw lines, with every flag, through **the same rendering
function** `cost` uses. That function lives in `crates/rk-cli/src/verbs/cost.rs`, and `status.rs` calls it. It has
no second home, so Checkpoint 8f (6)'s no-float floor covers it by covering that file. A flag therefore cannot be
dropped on one surface and kept on the other (*"the `redkiln cost` and `redkiln status` renderers print the flags
alongside any reading that carries them"*). An item with no reading gets no headroom line. The shipped store holds zero readings, so `status` output on the shipped tree does not move.

**AC-372's doctor clause — read as a renderer test (D21-43 N12).** *Verified by* names *"a doctor rule that a flagged
reading is never rendered bare"*. Renderings are not journalled, so such a rule has no input `doctor` can read, and
it could never fire. N12 takes the reading: *"AC-372's unfireable doctor rule is read, by dated note, as a test of
`cost`'s renderer, with AC-415's red case."* It is built as
`cost::a_flagged_reading_is_never_rendered_bare_and_a_bare_rendering_is_rejected`. The test drives the shared
renderer over every flag-bearing vector and asserts each flag in the output. Its red case is AC-415's
discipline: a bare rendering of a flagged reading, which the same check must reject. No `doctor` rule id is added
for it, so AC-415's rule-table join is unchanged. The dated note is §8f.10's AC-372 row.

### 8f.9 `doctor` over the store (D21-42 12c; D21-43 N12)

**The journal source is explicit.** `Ctx::build` stays the one builder (*"ONE BUILDER, AND THE REASON IS THE FAILURE
A SECOND ONE CAUSES"*) and gains a journal-source parameter:
- The verb passes the store's rows, the whole journal. The selector is inert, which the field doc already says.
- A fixture test may name `events.jsonl`.
- The verb **never** names the fixture source.

The false field doc is corrected in the same change. ADR-0163 decision 5's *"`doctor`'s journal never comes from the
store's event rows"* stops being true, and the telemetry-key record lists it among what it supersedes.

**The five-hour reading.** `RK-D-HEADROOM-MARGIN` reads the latest reading that **answers for `five_hour`**. It
reads it exactly as the throttle's `RateLimitReading::ticks(Window::FiveHour)` does: a unified reading's window, and
a flat reading when `rate_limit_type` is `None` or names that window (`None => Some(*utilization)`,
`Some(named) if named == window.as_str() => Some(*utilization)`). A flat reading that names `seven_day` is skipped,
so it no longer silences the rule. **This deliberately differs from `cost`**, which counts a flat reading with no
named window as `unwindowed` and forms no sample, because attribution never extrapolates. The margin rule shows the
guard's own margin, and AC-220's two halves must agree. The telemetry-key record states the difference so that
nobody "unifies" the two later.

**AC-159's rule — the unrecorded artifact mutation (D21-43 N12: *"AC-159 goes to s8f"*).** A new rule id,
`RK-D-ARTIFACT-MUTATION`, is added to the table. The render record fixes the spelling. The rule works like this:
- **What it reads.** For every managed directory the store names, it recomputes each structured section's digest and
  the frontmatter digest. It does this through `rk-store`'s locator, since `doctor` parses no markdown itself
  (AC-120). The directories are each `worktree_assigned` row's run-owned worktree (`s8e`), plus the checkout's own
  `.redkiln/artifacts/` and `.redkiln/items/`.
- **What it compares.** It compares those digests with the rendering of the canonical typed value (§8f.5).
- **What it reports.** A difference that no `artifact.write` accounts for is reported. So is a difference that no
  reconcile-accepted prose edit accounts for, since AC-159's GIVEN reads *"the reconcile that accepted it counts as
  the write record"*. The report is an unrecorded artifact mutation that names the artifact, both digests and the
  last recorded write.
- **Scope.** A prose-only edit is not a mutation (AC-152). A whole-file digest is not the instrument (PIA-043's
  amendment). A vouched root is out of scope. It is not layout C, and its change is `drift_advisory`'s (§8f.5).
  Reporting it here would report every advisory as a mutation.
- **No walk.** The rule **never walks** `.redkiln/` recursively. D21-44's `worktrees/` exclusion therefore holds by
  construction, and a synthetic control proves it (§8f.12).
- **Its rejecting fixture.** `ac415_every_rule_id_has_a_rejecting_fixture` joins rule ids to
  `crates/rk-cli/tests/fixtures/doctor/<id>/` in both directions, so the rule lands with its rejecting fixture: one
  managed file edited out-of-band. Its control is a file rewritten *through* the store (AC-159's *Verified by*). The
  fixture is data, and it adds no `#[test]` to `doctor.rs`.

**AC-127's doctor row: the two drift kinds (D21-46 R7).** AC-127's *Verified by* asks for *"a `doctor` rule-table row
distinguishing the two drift kinds"*. A new rule id, `RK-D-DRIFT-KIND`, is added, and the render record fixes the
spelling.
- **What it reads.** It reads the store's journal (the source above): every `structured_drift`, every
  `drift_advisory` and every `gate_opened`, in append order.
- **How it tells them apart.** It reads the blocking property from `rk_types::drift::Drift::blocks()`, whose doc says
  *"AC-127 is the criterion this encodes"*. It never reads it from a second table. A structured drift blocks, and an
  advisory does not.
- **What it reports.** It reports two crossings, and each finding names the drift kind, the artifact or root, and
  the digests.
  - **(a)** A `drift_advisory` followed in the same entry by a `render.structured-drift` gate naming its root. That
    is an advisory promoted to a blocking gate, which ADR-0025's first forbids bullet rules out, as does AC-067's
    *"no advisory can promote itself to `Stale` or to a blocking gate"*.
  - **(b)** A `structured_drift` with no `render.structured-drift` gate naming its artifact opened after it in the
    same entry. That is a blocking drift that did not block (N11).
- **Its rejecting fixture.** One journal under `crates/rk-cli/tests/fixtures/doctor/RK-D-DRIFT-KIND/` trips arm
  (a). A lane journal that carries both kinds, each handled correctly, is the control. Each arm has its own named
  test in `doctor_drift`, because a fixture per function is not a fixture per branch.

**The tests go in new modules, `doctor_journal`, `doctor_artifact` and `doctor_drift`.** They do not go in `tests/it/doctor.rs`.
D21-42 18 keeps `rk-fix-doctor-test-count` as *"the untouched fixture item, prose defect included"*. Its defect is
`doctor.rs`'s prose count (*"thirty-eight"*) against its **39** `#[test]` functions (re-counted at `a6349b6`). A test
added to that file would move the number the fixture item is about. The two new rules do move the rule table from
39 ids to 41. `doctor.rs`'s prose *"The table is thirty-eight"* was already stale, and it is left alone for the same
reason.

### 8f.10 Register, design and runbook notes this stage owes

Every note is a **dated in-place amendment**, never an erratum (DECISIONS-2026-09-15 D15-2). Each goes **after** its
criterion's *Verified by:* line, as AC-472's amendments do. Checkpoint 8f reads the THEN and *Verified by* spans, and
a note inside them would read as the criterion.

| site | amendment | Checkpoint 8f finds it by |
| --- | --- | --- |
| AC-245 | The untracked-file property in *Verified by* is scoped to the no-checkpoint-due basis. At a due checkpoint, and at every step checkpoint (N10), run-created files are staged before the derivation, so the derivation still sees no untracked file. It also records the landing record's reading of *"when a checkpoint commit is due"* (D21-47, review M6): a refused landing lands nothing, so no checkpoint commit is due at its gate, and the gate's tree is the no-checkpoint-due derivation (§8f.4) | `run-created` and `refused landing` in the entry |
| AC-473 | Its reading of *"a landed commit"*: §8f.4's first paragraph. Integration is Phase 2's merge queue | `rk-run/` in the entry |
| AC-521 | Its reading of *"authored into this repository's own backlog"*: the story artifact on `rk-run/<item>`, reachable through its `artifact.write` digest. The operator's merge is the hand-off | `rk-run/` in the entry |
| AC-478 | **The edit form (D21-47, review m16).** The THEN's AND line, *"AND no comparison of stored-vs-regenerated rendering is performed, so no drift state exists to report"*, is removed in place. A dated note after *Verified by:* quotes the removed line. It names what replaces it: surface-then-restore, with one `structured_drift` event and a drift gate (D21-42 5a; N11). The THEN's first line stands, because the restore regenerates the markdown over the edit. *Verified by*'s *"no artifact carries a drift or signature-comparison field"* stays. The entry's **Note**, *"this makes hand-edits to rendered markdown silently lossy"*, is false once the edit is surfaced. It takes a dated in-place correction that reads, in part, *"no longer silently lossy"* | the THEN no longer carries the phrase; `structured_drift` after *Verified by:*; `no longer silently lossy` in the entry |
| AC-125 | A reading note (D21-43 N11): "resolving the drift" means answering the drift gate that the `structured_drift` opened, with `confirm-restore` or `move-to-prose` | `drift gate` in the entry |
| AC register §0.3 row C2, and the section's two live "only C2" sentences (*"only C2 is still unresolved"* and *"exactly one of the five contradictions, C2, remains unresolved"*, both bold today) | C2 is resolved, on D21-42 5a, D21-43 N11 and the render record. The row says so. Both sentences are amended in place, with a date. Each replaced clause is quoted in italics without its bold, as the 2026-09-15 amendment quoted *"C1 is now decidable by measurement (PIA-068)"* | the word `resolved` in the row, matched as `\bresolved\b` so that `unresolved` cannot satisfy it (D21-47, review m8); neither bold sentence left in the register §0.3 |
| AC-196, AC-197 | A dated disposition: Phase 2's `rk-git` merge-queue stage (design §10.2's Phase-2 scope) | `Phase 2` in each entry |
| AC-372 | *Verified by*'s doctor-rule clause is read as a test of `cost`'s renderer, which `status` shares, with AC-415's red case (D21-43 N12; §8f.8) | ``test of `cost` `` in the entry |
| design §5.4 | The tier-1 row: a checkpoint at every committed step that changed the tree (N10) and at every boundary when due. The `structured_drift` row: its payload, and that it opens a drift gate. The `artifact.write` and `artifact.read` rows: their producers exist. The `dispatch_completed` row: `activity_id_at_version` (the `item` note is `s8e`'s, under N5). Under D21-46 R7: the `drift_advisory` row gains its second producing case, a vouched root's own change with `upstream = null` (§8f.5), and the `artifact.write` row gains the attach's path carrier as the render record fixes it | read at the adversarial pass |
| `s8c` | `s8c` §8c.4's `backlog` feed row and `s8c` §8c.12's handed-on item 3 take a dated note: the producer is built at `s8f` (D21-46 R7), and `[feed.*.backlog]` reads `verb = "backlog"` (§8f.7) | read at the adversarial pass |
| `s8d` | One dated line covers `s8d` §8d.9's two `closure-record` rows, its two `debt-logging` rows (D21-47, cross #6) and the lanes' claimed counts. Both activities **walk** once this stage's `backlog` producer and `item_create` handler land. The claims then read **13 of 15** and **11 of 13**. The two `lane_walk` tests' expected verdicts move in the same commit (§8f.7). Checkpoint 8d's (6a) floors the `LANE_WALK` counts at 11 and 9, and `s8d` §8d.9 reads *"A later stage that flips a row to **walks** raises `n` and needs no edit there"*. So no guard line is amended (re-read 2026-09-24, final consistency pass). The rows are still its premise, so **8d is provisional**, and Checkpoint 8f (9) re-runs it whole (D21-47's time-bound-guard rule) | the two `lane_walk` tests in Checkpoint 8f (5)'s whole suite, and Checkpoint 8d re-run whole in (9) |
| `s7i` | `s7i` (5)'s `GRAMMAR` widens to the breakdown forms. Its "owns no reading" branch also requires zero owned `dispatch_completed` rows, because after §8f.8 an item with dispatches and no reading prints more than one line. `s7i` §7i.2's *"nothing else"* takes a dated note | Checkpoint 7i re-run whole (Checkpoint 8f (9)) |
| `s7d` | `s7d` §7d.7's *"prints **from the recorded resolution only**"* takes a dated note for the landing block (§8f.4) | read at the adversarial pass |

**No longer owed, and why.** Three rows the draft carried are dropped:
- **AC-127** holds as written (N11). D21-46 R7 builds its other half, and that needs no note either.
- **AC-013** needs no note, because N10 leaves no committed step with a void tree (§8f.4).
- **AC-518's and AC-529's path wording** are `s8d-ii`'s notes under D21-44 (D21-46 R1).

### 8f.11 Acceptance set

**Claimed:**
- AC-120, held: no parser enters.
- AC-121, its same-OS half.
- AC-122, AC-123, AC-124, AC-144, AC-152.
- AC-125: the restore, the drift gate as the only resolution, and no drift text reaching a deserializer.
- AC-126: the read seam refuses before the entry's reconcile. A lane's replay shows every `artifact.read` after
  its entry's reconcile record. `run` has no skip flag.
- AC-127, whole. A structured drift blocks the stage's exit until it is answered. A changed vouched root is recorded
  as advisory drift and does not block (D21-46 R7). `RK-D-DRIFT-KIND` is the doctor row that distinguishes the two
  kinds.
- AC-067, held: no advisory promotes itself to a blocking gate (`RK-D-DRIFT-KIND` arm (a)).
- AC-159: `RK-D-ARTIFACT-MUTATION`, with its rejecting fixture and its through-the-store control.
- AC-163, the landing's empty commit.
- AC-220, the `doctor` half.
- AC-245.
- AC-372, the `status` half and its doctor clause as read by N12.
- AC-395, the breakdown.
- AC-411, AC-477.
- AC-472, its emission clause over a `FakeHarness` lane. The AND clause is `RK-D-UNREAD`'s, from `s7d`, and it now
  reads real rows.
- AC-478, as amended.
- AC-518, the landing's halves: the checkout is unchanged, and only the run ref moves. The file-write and
  index-change halves that `s8e` leaves to this stage are claimed through `s8e`'s AC-518 lint as §8f.4 widens it
  (final consistency pass 2026-09-24).

**Excluded, each by name:**
- **AC-121's cross-OS half**: it needs CI on both OS legs (D21-42 14a, `s8j`).
- **AC-525**: built at `s7b`. The own-change advisory reuses its sink and does not re-claim it. Its existing tests
  are held as regressions over the `Option<String>` change (§8f.5; Checkpoint 8f (4)).
- **AC-128**: no template V+1 exists in Phase 1. The stamp carries the version, so it is buildable later.
- **AC-153**: its CLI-only mutation channel is not built here.
- **AC-196, AC-197**: Phase 2.
- **AC-473, AC-521**: the runs are `s8`'s. This stage supplies the landed commit and the producer.
- **AC-529**: `s8e`'s 8d.

### 8f.12 Tests, red first, and the mutants that must flip them

Every test is written first and **watched fail for the right reason**: the behaviour is absent, not a compile error,
a typo'd filter or a missing fixture (golden constraint 12). Names are module-qualified. **85 new tests**: 64, plus
D21-46 R7's 15, plus D21-47's 5, plus D21-48 X11's 1.

- **`rk-git` `landing` (10):**
  - `landing::run_created_file_is_staged_and_an_ignored_file_is_not`
  - `landing::baseline_untracked_file_is_never_staged`
  - `landing::tracked_modification_and_deletion_are_staged`
  - `landing::not_due_makes_an_empty_commit_and_due_makes_one_content_commit`
  - `landing::denylisted_path_is_refused_by_name_and_no_ref_moves` (table-driven over every glob)
  - `landing::blob_at_the_limit_lands_and_one_byte_over_is_refused`
  - `landing::the_trailer_tree_is_the_commits_own_tree`
  - `landing::replay_serves_the_journal_and_performs_zero_writes`
  - `landing::a_repository_hook_does_not_run_on_a_landing_commit` (ADR-0105 clause 2's fixture)
  - `landing::a_staged_path_over_the_worst_case_length_is_refused_by_name` (D21-47, review m12: one byte under the
    bound lands, and one over is refused by name with nothing landed, under `core.longpaths` unset)
- **`rk-store` `render` (15):**
  - `render::layout_c_renders_frontmatter_stamped_sections_and_prose`
  - `render::render_is_a_function_of_value_and_template_only` (AC-123: the sentinel and the two-prior differential)
  - `render::render_is_byte_identical_twice_and_lf_only`
  - `render::structured_edit_emits_structured_drift_then_restores` (table over a section and the frontmatter)
  - `render::prose_edit_is_accepted_with_no_drift_record`
  - `render::one_artifact_write_per_write_carrying_the_rk_types_digest`
  - `render::no_artifact_type_carries_a_drift_or_signature_field`
  - `render::typed_value_reads_back_from_the_store_never_from_the_markdown`
  - `render::machine_only_kind_reads_its_sidecar_and_survives_md_deletion`
  - `render::ac122_a_sibling_crate_cannot_write_a_managed_path` (`trybuild`)
  - `render::replay_performs_zero_writes`
  - `render::move_to_prose_appends_the_drifted_bytes_from_the_causing_commit`
  - `render::drift_text_never_reaches_a_deserializer` (AC-125's architecture test: behavioural over a drifted
    section whose text parses as the field's type, paired with a code-lines-only scan)
  - `render::read_appends_one_artifact_read_naming_kind_and_id`
  - `render::a_read_before_the_entrys_reconcile_is_refused`
- **`rk-store` `item_write` (4):**
  - `item_write::debt_item_is_written_under_items_with_its_provenance`
  - `item_write::human_sourced_facets_are_never_written_and_are_named`
  - `item_write::an_existing_item_id_is_refused_never_overwritten` (a DIFFERING file)
  - `item_write::a_byte_identical_re_execution_is_served_and_writes_nothing`
- **`rk-store` `backlog` (3), D21-46 R7:**
  - `backlog::reads_only_the_items_the_store_names_each_by_exact_path` (the fixture holds an unnamed
    `.redkiln/items/stray.toml` and a named id's copy under `.redkiln/worktrees/x/.redkiln/items/`, and neither
    appears)
  - `backlog::a_named_item_with_no_record_is_listed_absent_never_dropped`
  - `backlog::the_snapshot_is_canonical_ordered_by_id_and_byte_identical_twice`
- **`rk-store` `vouched` (4), D21-46 R7:**
  - `vouched::ac127_four_outcomes_over_root_and_section_by_changed_and_unchanged` (a table with one row per arm:
    root changed, root unchanged, section changed, section unchanged)
  - `vouched::the_attached_root_is_stored_at_the_next_entry_under_its_vouched_digest` (a row with mismatching bytes
    is refused by name, and nothing is stored)
  - `vouched::a_changed_root_is_never_restored_rewritten_or_gated`
  - `vouched::one_advisory_per_run_for_one_observed_digest`
- **`rk-kernel` `emit` (3):**
  - `emit::every_activity_output_appends_one_artifact_write`
  - `emit::an_emit_replayed_writes_nothing_and_serves_its_recorded_digest`
  - `emit::every_artifact_input_resolution_appends_one_artifact_read`
- **`rk-kernel` `step_landing` (2):**
  - `step_landing::a_committed_step_that_changed_the_tree_lands_before_its_commit_row`
  - `step_landing::a_committed_step_that_changed_nothing_makes_no_commit`
- **`rk-kernel` `advisory` (1), D21-47, review m11.** This is an existing module, and the test goes into it:
  - `advisory::an_own_change_advisory_writes_upstream_null_with_all_four_fields` (the key is present with `null`, the
    triple is distinct from any upstream triple, and an upstream advisory's bytes are unchanged)
- **`rk-kernel` `feed` (2), D21-46 R7:**
  - `feed::closure_record_audit_resolves_toolchain_backlog_through_the_store_route` (over the shipped catalog and
    registry)
  - `feed::every_shipped_unprovided_feed_is_reached_only_as_optional` (a census over the shipped catalog and
    registry that pins §8f.7's list. Its red case is a synthetic mandatory reach of an `unprovided` feed, refused by
    name, naming the feed and the activity/step)
- **`rk-cli` `landing` (16)**. Each is a `FakeHarness` lane through `verbs::run::run`, on the harness `s8e`'s
  `lane_e2e` tests built:
  - `landing::fake_harness_edit_lands_on_the_run_ref_and_survives_a_second_run`
  - `landing::gate_tree_hash_is_its_own_commits_tree_and_answer_does_not_void_it`
  - `landing::the_user_checkout_is_unchanged_across_a_landing_lane`
  - `landing::only_the_run_ref_moves_across_a_lane`
  - `landing::a_refused_path_stops_the_lane_at_a_landing_gate_with_nothing_landed`
  - `landing::a_kill_after_a_committed_writer_step_resumes_over_its_landed_edit`
  - `landing::reentry_names_every_discarded_path` (the discarded set equals the staging set)
  - `landing::emitted_artifact_digest_matches_its_bytes_on_the_run_ref`
  - `landing::the_lane_end_lands_the_final_stages_work` (on N9's `closed` mark)
  - `landing::a_drift_gate_blocks_the_stage_exit_until_it_is_answered`
  - `landing::answering_the_drift_gate_either_way_releases_the_stage_exit` (table: `confirm-restore`,
    `move-to-prose`)
  - `landing::every_artifact_read_in_a_lane_follows_its_entrys_reconcile` (AC-126's replay assertion)
  - `landing::closure_record_walks_past_audit_on_the_backlog_feed` (D21-46 R7)
  - `landing::a_changed_vouched_root_does_not_block_the_stage_exit_and_the_next_gate_carries_it` (D21-46 R7)
  - `landing::a_refused_landing_is_not_due_and_answer_derives_the_gates_tree` (D21-47, review M6)
  - `landing::debt_logging_walks_past_file_through_item_create` (D21-47, cross #6)
- **`rk-cli`, the rest (24):**
  - `explain::explain_prints_the_landed_commit_and_the_merge_command`
  - `item_create::a_minted_debt_item_opens_an_unknown_facet_gate_when_run` (its repository's `[adapters]
    harness_binary` is `rk-fixture-child-claude`; D21-47, review m15)
  - `cost::per_activity_breakdown_from_owned_dispatch_completed_rows`
  - `cost::per_activity_cost_sums_decimals_verbatim_and_absent_is_never_zero`
  - `cost::no_breakdown_for_an_item_that_owns_no_dispatch_completed_row`
  - `cost::a_flagged_reading_is_never_rendered_bare_and_a_bare_rendering_is_rejected` (AC-372 by N12; AC-415's red
    case)
  - `status::status_prints_every_flag_beside_its_reading`
  - `status::status_prints_no_headroom_line_for_an_item_with_no_reading`
  - `run::no_cli_flag_skips_the_entry_reconcile` (AC-126's surface half)
  - `doctor_journal::a_present_store_is_the_journal` (through the binary, with no `events.jsonl` in the tree)
  - `doctor_journal::events_jsonl_is_read_only_in_fixture_mode`
  - `doctor_journal::headroom_margin_fires_on_a_flat_last_reading`
  - `doctor_journal::headroom_margin_reads_the_latest_five_hour_bearing_reading`
  - `doctor_journal::the_doctor_verb_never_selects_the_fixture_source` (a code-lines-only scan, paired with the first
    `doctor_journal` test, because *a source lint passes on code nothing calls*)
  - `doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes` (D21-47, review M2)
  - `doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match` (D21-48 X11) [red: the widened `run_of` keys `null` by its text, so two null-run rows count as a
    second run and the first arm fires]
  - `doctor_artifact::an_out_of_band_structured_edit_is_reported_as_an_unrecorded_mutation`
  - `doctor_artifact::a_rewrite_through_the_store_is_not_reported`
  - `doctor_artifact::a_prose_edit_accepted_by_reconcile_is_not_reported`
  - `doctor_artifact::an_unassigned_copy_under_redkiln_worktrees_is_never_read` (the synthetic control for D21-44's
    exclusion: a mutated copy no `worktree_assigned` row names)
  - `doctor_artifact::a_vouched_root_edit_is_not_reported_as_an_unrecorded_mutation` (D21-46 R7)
  - `doctor_drift::an_advisory_that_opened_a_blocking_gate_is_reported` (arm (a); D21-46 R7)
  - `doctor_drift::a_structured_drift_with_no_drift_gate_is_reported` (arm (b); D21-46 R7)
  - `doctor_drift::a_lane_journal_carrying_both_kinds_correctly_is_clean` (the control; D21-46 R7)
- **`rk-adapter-claude` `dispatch` (1):**
  - `dispatch::dispatch_completed_carries_activity_id_at_version_or_null`

`cost::cost_output_grammar_is_closed` is **amended**, not added. So is `s8e`'s
`tier_one::every_run_commit_resolves_through_the_worktree_resolver`, widened to the landing's writes (§8f.4; final
consistency pass 2026-09-24). `doctor::ac415_every_rule_id_has_a_rejecting_fixture`
gains two fixture directories, `RK-D-ARTIFACT-MUTATION` and `RK-D-DRIFT-KIND`, and no test. `RK-D-UNREAD`'s `run_of`
is widened (§8f.5). Its two `s7d` tests, `doctor::ac426_ac472_unread_window_49_not_flagged_51_flagged` and
`doctor::ac426_rule_fire_leaves_write_set_unchanged`, are regressions and not new tests, and so is
`advisory::ac525_drift_advisory_once`.

**Mutants.** Each is authored, run and **must flip the named test**. A survivor blocks (golden constraint 12). The
mutant cycles run **from Python**, never the shell: a backtick once swallowed a restore, and the next backup
captured the mutant. The driver lives in the scratchpad. After every cycle it verifies the file's restored bytes by
hash and by `git diff --quiet -- <file>`. A mutant is only evidence about the branch it sits on, so where a test is
table-driven, each arm has its own row (*a fixture per function is not a fixture per branch*).

| id | mutant | must flip |
| --- | --- | --- |
| M1 | stage tracked paths only (drop the run-created `add`) | `landing::run_created_file_is_staged_and_an_ignored_file_is_not`; `landing::fake_harness_edit_lands_on_the_run_ref_and_survives_a_second_run` |
| M2 | do not subtract the baseline | `landing::baseline_untracked_file_is_never_staged` |
| M3 | list untracked with ignores disabled | `landing::run_created_file_is_staged_and_an_ignored_file_is_not` |
| M4 | always `allow_empty` (never a content commit) | `landing::not_due_makes_an_empty_commit_and_due_makes_one_content_commit` |
| M5 | drop the denylist check | `landing::denylisted_path_is_refused_by_name_and_no_ref_moves` |
| M6 | size comparison `>` → `>=` | `landing::blob_at_the_limit_lands_and_one_byte_over_is_refused` |
| M7 | the `Gate:` tree from the stage-start `WorkingState` | `landing::gate_tree_hash_is_its_own_commits_tree_and_answer_does_not_void_it` |
| M8 | append the step's commit row with no step landing (N10 dropped) | `step_landing::a_committed_step_that_changed_the_tree_lands_before_its_commit_row`; `landing::a_kill_after_a_committed_writer_step_resumes_over_its_landed_edit` |
| M9 | the renderer reads the prior section text | `render::render_is_a_function_of_value_and_template_only` |
| M10 | restore without the event | `render::structured_edit_emits_structured_drift_then_restores` |
| M11 | the event without the restore | `render::structured_edit_emits_structured_drift_then_restores` |
| M12 | prose treated as structured | `render::prose_edit_is_accepted_with_no_drift_record` |
| M13 | no `artifact.write` on the restore write | `render::one_artifact_write_per_write_carrying_the_rk_types_digest` |
| M14 | write a human-sourced facet | `item_write::human_sourced_facets_are_never_written_and_are_named` |
| M15 | overwrite an existing, differing item file | `item_write::an_existing_item_id_is_refused_never_overwritten` |
| M16 | `activity_id_at_version` written without its `@version` | `dispatch::dispatch_completed_carries_activity_id_at_version_or_null` |
| M17 | `total_cost_usd` summed through `f64` | `cost::per_activity_cost_sums_decimals_verbatim_and_absent_is_never_zero` (a vector an `f64` sum mis-states) |
| M18 | the breakdown printed for an item that owns no rows | `cost::no_breakdown_for_an_item_that_owns_no_dispatch_completed_row`; `cost::render_empty_is_nothing_to_attribute_never_low_sample` |
| M19 | `status` renders a draw without its flags | `status::status_prints_every_flag_beside_its_reading` |
| M20 | the verb passes the fixture source | `doctor_journal::a_present_store_is_the_journal` |
| M21 | the margin rule reverts to `.last()` | `doctor_journal::headroom_margin_fires_on_a_flat_last_reading` |
| M22 | `explain` prints `--squash` | `explain::explain_prints_the_landed_commit_and_the_merge_command` |
| M23 | the drift is recorded and restored, and no gate opens | `landing::a_drift_gate_blocks_the_stage_exit_until_it_is_answered` |
| M24 | the read seam serves before the entry's reconcile | `render::a_read_before_the_entrys_reconcile_is_refused`; `landing::every_artifact_read_in_a_lane_follows_its_entrys_reconcile` |
| M25 | the read seam appends no `artifact.read` | `render::read_appends_one_artifact_read_naming_kind_and_id`; `emit::every_artifact_input_resolution_appends_one_artifact_read` |
| M26 | AC-159's rule compares a whole-file digest | `doctor_artifact::a_prose_edit_accepted_by_reconcile_is_not_reported` |
| M27 | AC-159's rule ignores the store's write record | `doctor_artifact::a_rewrite_through_the_store_is_not_reported` |
| M28 | the shared renderer drops `resolution_limited` | `cost::a_flagged_reading_is_never_rendered_bare_and_a_bare_rendering_is_rejected`; `status::status_prints_every_flag_beside_its_reading` |
| M29 | `item_create` refuses a byte-identical re-execution | `item_write::a_byte_identical_re_execution_is_served_and_writes_nothing` |
| M30 | a step that changed nothing still makes an (empty) commit | `step_landing::a_committed_step_that_changed_nothing_makes_no_commit` |
| M31 | `move-to-prose` reads the drifted bytes from the event rather than the causing commit | `render::move_to_prose_appends_the_drifted_bytes_from_the_causing_commit` |
| M32 | the `backlog` producer walks `.redkiln/items/` (`read_dir`) instead of reading the named ids | `backlog::reads_only_the_items_the_store_names_each_by_exact_path` |
| M33 | a named id with no record is dropped from the snapshot | `backlog::a_named_item_with_no_record_is_listed_absent_never_dropped` |
| M34 | `[feed.rust.backlog]` is left `unprovided` | `feed::closure_record_audit_resolves_toolchain_backlog_through_the_store_route`; `landing::closure_record_walks_past_audit_on_the_backlog_feed` |
| M35 | an `unprovided` mandatory feed resolves to `null` instead of refusing | `feed::every_shipped_unprovided_feed_is_reached_only_as_optional` (its red case) |
| M36 | a vouched root's change is classed blocking and opens a `render.structured-drift` gate | `vouched::ac127_four_outcomes_over_root_and_section_by_changed_and_unchanged` (the root-changed row); `landing::a_changed_vouched_root_does_not_block_the_stage_exit_and_the_next_gate_carries_it` |
| M37 | a vouched root's change appends nothing | `vouched::ac127_four_outcomes_over_root_and_section_by_changed_and_unchanged` (the root-changed row) |
| M38 | a changed root is restored to its vouched bytes | `vouched::a_changed_root_is_never_restored_rewritten_or_gated` |
| M39 | the advisory is appended at every entry pass, with no run-scoped de-duplication | `vouched::one_advisory_per_run_for_one_observed_digest` |
| M40 | the stored root's digest is not checked against the attach's | `vouched::the_attached_root_is_stored_at_the_next_entry_under_its_vouched_digest` |
| M41 | `RK-D-DRIFT-KIND`'s arm (a) is removed | `doctor_drift::an_advisory_that_opened_a_blocking_gate_is_reported`; `doctor::ac415_every_rule_id_has_a_rejecting_fixture` |
| M42 | `RK-D-DRIFT-KIND`'s arm (b) is removed | `doctor_drift::a_structured_drift_with_no_drift_gate_is_reported` |
| M43 | `RK-D-ARTIFACT-MUTATION` reads vouched roots | `doctor_artifact::a_vouched_root_edit_is_not_reported_as_an_unrecorded_mutation` |
| M44 | the writer's `artifact.write` row omits `stage` (D21-47, review M2) | `doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes` |
| M45 | `answer`'s attach `artifact.write` row omits `run` (D21-47, review M2) | `doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes` |
| M46 | `item_create`'s `store`-route row is left refusing (D21-47, cross #6) | `landing::debt_logging_walks_past_file_through_item_create` |
| M47 | the stage advance's drift check is removed, and the drift gate still opens (D21-47, cross #16) | `landing::a_drift_gate_blocks_the_stage_exit_until_it_is_answered` |
| M48 | a refused landing's gate records the empty commit's tree (D21-47, review M6) | `landing::a_refused_landing_is_not_due_and_answer_derives_the_gates_tree` |
| M49 | the landing's path-length bound is dropped (D21-47, review m12) | `landing::a_staged_path_over_the_worst_case_length_is_refused_by_name` |
| M50 | an own-change advisory writes `"upstream": ""` in place of `null` (D21-47, review m11) | `advisory::an_own_change_advisory_writes_upstream_null_with_all_four_fields` |
| M51 | the landing's call to `rk-git`'s primitive moves into a helper outside `runtime/tier_one.rs` (final consistency pass 2026-09-24) | `tier_one::every_run_commit_resolves_through_the_worktree_resolver` |
| M52 | `RK-D-UNREAD`'s `run_of` keys a `null` `run` by its canonical text `null` (D21-48 X11) | `doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match` |

### 8f.13 Checkpoint 8f

- **The three records exist and are accepted.** Each is located by the id written into its `REC_` line at writing,
  never by a title phrase (D21-47). Each has two or more `### Option — ` headings and a **What this forbids** block.
  Each has its README row, and a dated DECISIONS heading that flips it to `accepted`. The landing record supersedes
  ADR-0105 clause 5 in part and quotes it. The telemetry-key record supersedes 0131 and 0163 in part and quotes
  ADR-0163's ownership bullet. Each superseded record names the superseding id in `superseded_by`.
- **The register carries every note §8f.10 names.** These notes are the BEFORE assertions (PIA-035). Each was
  measured **false** on the untouched tree. The AC-125 and AC-372 needles were re-measured at `a6349b6`: 0 hits for
  `drift gate` and 0 for ``test of `cost` `` in the whole register. D21-47's needles were measured at `fc80687`:
  - `refused landing` is absent from AC-245.
  - `structured_drift` and `no longer silently lossy` are absent from AC-478.
  - Row C2 has no `\bresolved\b`.
  - Both bold "only C2" sentences are present in the register §0.3.
- **All 85 named tests pass, plus two amended tests and seven existing guards.** The amended tests are the grammar
  test and `s8e`'s AC-518 lint as §8f.4 widens it (final consistency pass 2026-09-24). The guards are the four
  drafted, plus `RK-D-UNREAD`'s two `s7d` tests and `advisory::ac525_drift_advisory_once` (D21-47). Each is selected by its
  module-qualified name, with its own `... ok` line and an exact count.
- **The whole suite passes.** `--no-fail-fast`, rc 0, `passed >= FLOOR`, zero failed. fmt and clippy are green.
- **No integration and no float.** These are code-lines-only floors beneath the behavioural tests. The no-float
  floor covers `cost.rs`, which holds the renderer `status` shares, and `status.rs`. It also covers `rk-store/src`,
  and `rk-types/src`, where `CostUsd` lives (D21-47, review m14). Each floor is one function. It is first run over a
  planted tree, where it must keep the planted code line and drop the planted comment, and only then over the real
  paths. At `fc80687` all three floors read 0 code-line hits.
- **The feeds are as §8f.7 lists them, and the `backlog` producer never walks (D21-46 R7).** A registry probe with a
  synthetic control reads the three feeds on both toolchains. A code-lines-only floor sits beneath
  `backlog::reads_only_the_items_the_store_names_each_by_exact_path`. It covers the whole of `rk-store/src`, because
  every file the producer reaches is in that crate (§8f.7), and it has its planted-hit control.
- **This block leaves the shipped store unchanged.** It re-runs its entry gate, Checkpoint 8e, whole. It also
  re-runs two checkpoints this stage makes provisional. Checkpoint 8d is one, because §8f.10 moves its walk-table rows
  and the `lane_walk` expectations (D21-47, cross #6). Checkpoint 7i is the other, because §8f.10 amends `s7i`. It
  re-runs nothing else. All three reach their end markers. The worktree path set is unchanged, and the checkout is
  clean.
- **Nested cost and dispatch, stated.** 8e, 8d and 7i are launched with `RK_CP_NESTED=1` (D21-46), so 8e and 8d
  skip their own whole re-runs, and under `RK_CP_NESTED=1` this block re-runs none of the three. **Every process-level
  run in this block goes through one wrapper (D21-47)**: the cargo build, the named selections, the whole suite,
  fmt, clippy and each replay. The wrapper is the shared helper `runbook/tools/nodispatch.sh` (final
  consistency pass 2026-09-24). Its fake `claude` is **compiled**, so it is the `claude.exe` that a Rust
  `Command::new("claude")` resolves on Windows. It prints the pin for `--version`, and it logs and refuses anything
  else. Every other `PATH` directory holding a `claude` is stripped, `REDKILN_LAUNCH_RECORD` is unset, and
  `CLAUDE_CONFIG_DIR` points at an empty trap. This session's `CLAUDE_CONFIG_DIR` is the operator's real account.
  `nd_setup` proves the trap empty with a planted-file control before the first run. `nd_check` proves the trap empty
  and the fake uncalled after the build, each named selection, the whole suite, fmt, clippy, each replay and the
  block's end. A nested replay reuses the enclosing wrapper. The stage records the wall
  time it measured. No real `claude` can be spawned by this block at this stage. `s8h` has not landed, and at
  `a6349b6` **0** of the 8 shipped cards carry `permission_mode`, which `card::lint_card` refuses as
  `PermissionModeAbsent` at bind. 7i's run arm is a
  `cargo test` over a fixture transcript. After `s8h` lands, a later whole re-run of this block (from `s8j`) passes
  through 8e's, 8d's and 7i's blocks as they stand then. Those blocks point `[adapters] harness_binary` at
  `target/debug/rk-fixture-child-claude` under D21-47's rule, and they assert that no real `claude` was spawned.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status: under build-lock contention cargo exits 0 having run
# ZERO tests. Every selection is `--test it`, MODULE-QUALIFIED and `--exact` — a bare name under `--exact` runs zero
# tests and exits 0 — and asserts each test's own `... ok` line AND an exact count. A source grep is a FLOOR over CODE
# lines beneath a behavioural test, never the assertion, and (7) drives every helper red on synthetic input first.

# ---- (0) THE WORKTREE PATH SET, read FIRST — before any cargo run and before any prune.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }

# ---- (0b) THE WRAPPER every process-level run below goes through (D21-47): the shared helper, sourced, never retyped
# ---- (final consistency pass 2026-09-24). nd_setup compiles a fake `claude` (claude.exe on Windows: a Rust
# ---- Command::new("claude") resolves only claude.exe, so a shell-script fake would leave the real binary to answer)
# ---- that prints the pin for --version and logs and refuses anything else; strips every other PATH directory holding
# ---- a claude; unsets REDKILN_LAUNCH_RECORD; points CLAUDE_CONFIG_DIR at an empty trap; and runs its own controls.
# ---- This session's CLAUDE_CONFIG_DIR is the operator's real account. Inside an enclosing wrapper (this block replayed
# ---- by s8g) it reuses that one. `nodispatch <cmd>` runs one command under it; `nd_check <label>` fails if the trap
# ---- holds anything or the fake was called.
. runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh could not be sourced from the repo root"; exit 1; }
nd_setup "$PWD/target/cp8f/nd" || exit 1
# This block's own controls, beyond the helper's: a planted launch record does not reach a wrapped process, and a
# wrapped process reads the trap as its CLAUDE_CONFIG_DIR.
got=$(export REDKILN_LAUNCH_RECORD=planted; nodispatch sh -c 'printf %s "${REDKILN_LAUNCH_RECORD-unset}"')
[ "$got" = unset ] || { echo "FAIL (control): the wrapper did not unset a planted REDKILN_LAUNCH_RECORD (read '$got')"; exit 1; }
got=$(nodispatch sh -c 'printf %s "$CLAUDE_CONFIG_DIR"')
[ -n "$got" ] && [ "$got" = "$ND_TRAP_NATIVE" ] || { echo "FAIL: the wrapper did not point CLAUDE_CONFIG_DIR at the trap (read '$got')"; exit 1; }
nd_check "(0b)'s controls" || exit 1

# ---- (1) BUILD-OR-FAIL, and the one environment precondition the whole suite has (a fresh checkout fails ten node
# ---- tests at tests/it/support.rs's preflight without it).
out=$(nodispatch cargo build --workspace 2>&1); rc=$?
nd_check "(1)'s build" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8f NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: run pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project first — the suite below would fail for a reason unrelated to this stage"; exit 1; }

# ---- (1b) THE SHIPPED STORE'S ROW COUNTS, read-only, BEFORE this block's own sections. `mode=ro`, never `immutable=1`
# ---- (immutable ignores the WAL). The nested re-runs in (9) are AFTER the after-read and assert their own stores.
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store"; exit 1; }
# After s8e's 0002 migration (D21-46 R2) the gate table is t1_gate_transition; t1_gate no longer exists.
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t2_event","t1_stage_transition","t1_gate_transition","t1_slice_seal")))'
store_before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$store_before" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }

# ---- (2) THE THREE RECORDS, located by the id each took AT WRITING (D21-47; §8f.3), never by a title phrase: s8g's
# ---- tag-release record also says "landed commit". Each REC_ line is written by a dated edit in the commit that
# ---- writes its record. BEFORE: all three read `unassigned`, so (2) is red until the records exist and are accepted.
REC_LANDING=unassigned
REC_RENDER=unassigned
REC_TELEMETRY=unassigned
recs='import glob,re,sys
names=("landing","render","telemetry")
ids=dict(zip(names,sys.argv[1:4]))
need={"landing":(["0105"],["Anything not listed is a successor"]),"render":([],[]),"telemetry":(["0131","0163"],["reading item ownership off"])}
readme=open("references/adr/README.md",encoding="utf-8").read()
dec="".join(open(f,encoding="utf-8").read() for f in sorted(glob.glob("spikes/DECISIONS-2026-*.md")))
bad=[]
if len(set(ids.values()))!=3: bad.append("two REC_ lines name one id")
for n in names:
    i=ids[n]
    if not re.fullmatch(r"[0-9]{4}",i) or int(i)<165:
        bad.append("%s record: its REC_ line reads %r, not an id >= 0165 written at the writing of the record"%(n,i)); continue
    fs=glob.glob("references/adr/"+i+"-*.md")
    if len(fs)!=1: bad.append("%s record: %d files carry id %s"%(n,len(fs),i)); continue
    t=open(fs[0],encoding="utf-8").read()
    if len(re.findall("^### Option \u2014 ",t,re.M))<2: bad.append(i+": fewer than two Option headings (ADR-0085; D21-42 24a)")
    if "**What this forbids**" not in t: bad.append(i+": no What this forbids block")
    if not re.search(r"^status: accepted\s*$",t,re.M): bad.append(i+": not accepted - a record flips only on the operator dated line")
    if "["+i+"](" not in readme: bad.append(i+": no README index row")
    if not re.search(r"^## D[0-9]+-[0-9]+ . ADR-"+i+r"\b[^\n]*flips to `accepted`",dec,re.M): bad.append(i+": no dated DECISIONS heading flips ADR-"+i+" to accepted")
    m=re.search(r"^supersedes:(.*)$",t,re.M); sup=m.group(1) if m else ""
    for a in need[n][0]:
        if a not in sup: bad.append("%s: the %s record does not supersede %s in part"%(i,n,a))
        g=glob.glob("references/adr/"+a+"-*.md")
        s2=open(g[0],encoding="utf-8").read() if len(g)==1 else ""
        m2=re.search(r"^superseded_by:(.*)$",s2,re.M)
        if not (m2 and i in m2.group(1)): bad.append("ADR-%s: superseded_by does not name %s"%(a,i))
    for q in need[n][1]:
        if q not in t: bad.append("%s: the %s record does not quote %r"%(i,n,q))
print("\n".join(bad)); sys.exit(1 if bad else 0)'
out=$(python -c "$recs" "$REC_LANDING" "$REC_RENDER" "$REC_TELEMETRY" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the records this stage owes are missing or malformed:"; printf '%s\n' "$out"; exit 1; }

# ---- (3) THE REGISTER NOTES §8f.10 NAMES. BEFORE (measured at ca8f387; the AC-125 and AC-372 needles re-measured at
# ---- a6349b6; D21-47's needles at fc80687): every probe below reads FALSE. The AC-478 read is scoped to the THEN ..
# ---- *Verified by:* span, which is why §8f.10 places every amendment AFTER *Verified by:*. The C2 needle is a word
# ---- match, `\bresolved\b`, so "unresolved" cannot satisfy it, and its control runs first (D21-47, review m8).
reg='import re,sys
t=open("references/2026-09-08-acceptance-criteria.md",encoding="utf-8").read()
def entry(n):
    m=re.search(r"^\*\*AC-%d\*\*.*?(?=^\*\*AC-\d+\*\*|^#)"%n,t,re.S|re.M)
    return m.group(0) if m else ""
word=re.compile(r"\bresolved\b",re.I)
if word.search("only C2 is still unresolved") or not word.search("C2 is resolved"):
    print("CONTROL: the C2 needle cannot tell resolved from unresolved"); sys.exit(1)
bad=[]
for n,w in [(245,"run-created"),(245,"refused landing"),(473,"rk-run/"),(521,"rk-run/"),(196,"Phase 2"),(197,"Phase 2"),(372,"test of `cost`"),(125,"drift gate")]:
    e=entry(n)
    if not e: bad.append("AC-%d: entry not found"%n)
    elif w not in e: bad.append("AC-%d: no dated note carrying %r"%(n,w))
e=entry(478); i=e.find("THEN"); j=e.find("*Verified by:*")
if i<0 or j<0: bad.append("AC-478: THEN or Verified-by not found")
else:
    if "so no drift state exists to report" in e[i:j]: bad.append("AC-478: the THEN still says no drift state exists")
    if "structured_drift" not in e[j:]: bad.append("AC-478: no dated note after Verified by naming structured_drift")
    if "no longer silently lossy" not in e: bad.append("AC-478: the silently-lossy Note carries no dated correction")
s=t.find("### 0.3"); u=t.find("### 0.4")
sec=t[s:u] if 0<=s<u else ""
if not sec: bad.append("AC register section 0.3 not found")
rows=[l for l in sec.split("\n") if l.startswith("| C2 |")]
if len(rows)!=1 or not word.search(rows[0]): bad.append("AC register section 0.3: row C2 is not marked resolved")
for live in ("**only C2 is still unresolved**","**exactly one of the five contradictions, C2, remains unresolved**"):
    if live in sec: bad.append("AC register section 0.3 still carries the live sentence "+live)
print("\n".join(bad)); sys.exit(1 if bad else 0)'
out=$(python -c "$reg" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: register notes owed by this stage are missing:"; printf '%s\n' "$out"; exit 1; }

# ---- (4) THE NAMED TESTS, per crate. `check_sel` reads a CAPTURED output; `run_sel` captures it. (7) drives
# ---- `check_sel` red on synthetic input, so a helper that passes everything cannot hide here.
check_sel() {   # $1 label  $2 exact count  $3 captured output  $4.. module-qualified names
  label=$1; want=$2; cap=$3; shift 3
  for t in "$@"; do
    printf '%s\n' "$cap" | grep -qxF "test $t ... ok" || { echo "FAIL: $label: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$cap" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$cap" | grep -qE "^test result: ok\. $want passed; 0 failed" || { echo "FAIL: $label: the selection did not run exactly $want tests"; printf '%s\n' "$cap" | grep -E '^test result:'; exit 1; }
}
run_sel() {     # $1 crate  $2 exact count  $3.. module-qualified names
  crate=$1; want=$2; shift 2
  out=$(nodispatch cargo test -p "$crate" --test it -- "$@" --exact 2>&1); rc=$?
  nd_check "$crate's named selection" || exit 1
  [ "$rc" -eq 0 ] || { echo "FAIL: $crate: the named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  check_sel "$crate" "$want" "$out" "$@"
}
run_sel rk-git 10 landing::a_staged_path_over_the_worst_case_length_is_refused_by_name landing::run_created_file_is_staged_and_an_ignored_file_is_not landing::baseline_untracked_file_is_never_staged landing::tracked_modification_and_deletion_are_staged landing::not_due_makes_an_empty_commit_and_due_makes_one_content_commit landing::denylisted_path_is_refused_by_name_and_no_ref_moves landing::blob_at_the_limit_lands_and_one_byte_over_is_refused landing::the_trailer_tree_is_the_commits_own_tree landing::replay_serves_the_journal_and_performs_zero_writes landing::a_repository_hook_does_not_run_on_a_landing_commit
run_sel rk-store 26 render::layout_c_renders_frontmatter_stamped_sections_and_prose render::render_is_a_function_of_value_and_template_only render::render_is_byte_identical_twice_and_lf_only render::structured_edit_emits_structured_drift_then_restores render::prose_edit_is_accepted_with_no_drift_record render::one_artifact_write_per_write_carrying_the_rk_types_digest render::no_artifact_type_carries_a_drift_or_signature_field render::typed_value_reads_back_from_the_store_never_from_the_markdown render::machine_only_kind_reads_its_sidecar_and_survives_md_deletion render::ac122_a_sibling_crate_cannot_write_a_managed_path render::replay_performs_zero_writes render::move_to_prose_appends_the_drifted_bytes_from_the_causing_commit render::drift_text_never_reaches_a_deserializer render::read_appends_one_artifact_read_naming_kind_and_id render::a_read_before_the_entrys_reconcile_is_refused item_write::debt_item_is_written_under_items_with_its_provenance item_write::human_sourced_facets_are_never_written_and_are_named item_write::an_existing_item_id_is_refused_never_overwritten item_write::a_byte_identical_re_execution_is_served_and_writes_nothing backlog::reads_only_the_items_the_store_names_each_by_exact_path backlog::a_named_item_with_no_record_is_listed_absent_never_dropped backlog::the_snapshot_is_canonical_ordered_by_id_and_byte_identical_twice vouched::ac127_four_outcomes_over_root_and_section_by_changed_and_unchanged vouched::the_attached_root_is_stored_at_the_next_entry_under_its_vouched_digest vouched::a_changed_root_is_never_restored_rewritten_or_gated vouched::one_advisory_per_run_for_one_observed_digest
run_sel rk-kernel 10 tier_one::every_run_commit_resolves_through_the_worktree_resolver advisory::an_own_change_advisory_writes_upstream_null_with_all_four_fields advisory::ac525_drift_advisory_once emit::every_activity_output_appends_one_artifact_write emit::an_emit_replayed_writes_nothing_and_serves_its_recorded_digest emit::every_artifact_input_resolution_appends_one_artifact_read step_landing::a_committed_step_that_changed_the_tree_lands_before_its_commit_row step_landing::a_committed_step_that_changed_nothing_makes_no_commit feed::closure_record_audit_resolves_toolchain_backlog_through_the_store_route feed::every_shipped_unprovided_feed_is_reached_only_as_optional
run_sel rk-adapter-claude 2 dispatch::dispatch_completed_carries_activity_id_at_version_or_null dispatch::ac360_one_event_per_termination_path
run_sel rk-cli 46 landing::a_refused_landing_is_not_due_and_answer_derives_the_gates_tree landing::debt_logging_walks_past_file_through_item_create doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes doctor_journal::unread_reads_a_null_run_as_an_unknown_run_never_a_match doctor::ac426_ac472_unread_window_49_not_flagged_51_flagged doctor::ac426_rule_fire_leaves_write_set_unchanged landing::fake_harness_edit_lands_on_the_run_ref_and_survives_a_second_run landing::gate_tree_hash_is_its_own_commits_tree_and_answer_does_not_void_it landing::the_user_checkout_is_unchanged_across_a_landing_lane landing::only_the_run_ref_moves_across_a_lane landing::a_refused_path_stops_the_lane_at_a_landing_gate_with_nothing_landed landing::a_kill_after_a_committed_writer_step_resumes_over_its_landed_edit landing::reentry_names_every_discarded_path landing::emitted_artifact_digest_matches_its_bytes_on_the_run_ref landing::the_lane_end_lands_the_final_stages_work landing::a_drift_gate_blocks_the_stage_exit_until_it_is_answered landing::answering_the_drift_gate_either_way_releases_the_stage_exit landing::every_artifact_read_in_a_lane_follows_its_entrys_reconcile explain::explain_prints_the_landed_commit_and_the_merge_command item_create::a_minted_debt_item_opens_an_unknown_facet_gate_when_run cost::per_activity_breakdown_from_owned_dispatch_completed_rows cost::per_activity_cost_sums_decimals_verbatim_and_absent_is_never_zero cost::no_breakdown_for_an_item_that_owns_no_dispatch_completed_row cost::a_flagged_reading_is_never_rendered_bare_and_a_bare_rendering_is_rejected status::status_prints_every_flag_beside_its_reading status::status_prints_no_headroom_line_for_an_item_with_no_reading run::no_cli_flag_skips_the_entry_reconcile doctor_journal::a_present_store_is_the_journal doctor_journal::events_jsonl_is_read_only_in_fixture_mode doctor_journal::headroom_margin_fires_on_a_flat_last_reading doctor_journal::headroom_margin_reads_the_latest_five_hour_bearing_reading doctor_journal::the_doctor_verb_never_selects_the_fixture_source doctor_artifact::an_out_of_band_structured_edit_is_reported_as_an_unrecorded_mutation doctor_artifact::a_rewrite_through_the_store_is_not_reported doctor_artifact::a_prose_edit_accepted_by_reconcile_is_not_reported doctor_artifact::an_unassigned_copy_under_redkiln_worktrees_is_never_read cost::cost_output_grammar_is_closed cost::render_empty_is_nothing_to_attribute_never_low_sample run::ac438_table_driven_exit_codes_over_fixtures_binary_arm doctor::ac415_every_rule_id_has_a_rejecting_fixture landing::closure_record_walks_past_audit_on_the_backlog_feed landing::a_changed_vouched_root_does_not_block_the_stage_exit_and_the_next_gate_carries_it doctor_artifact::a_vouched_root_edit_is_not_reported_as_an_unrecorded_mutation doctor_drift::an_advisory_that_opened_a_blocking_gate_is_reported doctor_drift::a_structured_drift_with_no_drift_gate_is_reported doctor_drift::a_lane_journal_carrying_both_kinds_correctly_is_clean

# ---- (5) THE WHOLE SUITE, and the two lint gates. FLOOR is §8f.2 item 9's expected value, re-derived at stage open from
# ---- Checkpoint 8e's recorded count: 1038 + 85 = 1123 (85 = 64, plus D21-46 R7's 15, plus D21-47's 5, plus D21-48
# ---- X11's 1; 1038 = 740 + 9 + 25 + 78 + 62 + 50 + 74; final consistency pass 2026-09-24, D21-48 2026-09-24; the
# ---- D21-48 floor propagation moved it from 1021 + 85 = 1106), by a
# ---- dated edit of the line below before the first run. A filtered green can select around every catching test; the whole suite cannot. Every cargo run is under
# ---- the (0b) wrapper, and nd_check follows each one.
FLOOR=1123
out=$(nodispatch cargo test --workspace --no-fail-fast 2>&1); rc=$?
nd_check "(5)'s whole suite" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|^test result:' | head -30; exit 1; }
results=$(printf '%s\n' "$out" | grep -c '^test result: ')
passed=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="passed;") s+=$(i-1)} END{print s+0}')
failed=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="failed;") s+=$(i-1)} END{print s+0}')
[ "$results" -ge 44 ] || { echo "FAIL: only $results test binaries reported — 992d257 had 44; a binary vanished"; exit 1; }
[ "$failed" -eq 0 ] || { echo "FAIL: $failed tests failed under an rc of 0"; exit 1; }
[ "$passed" -ge "$FLOOR" ] || { echo "FAIL: $passed passed, below the floor of $FLOOR"; exit 1; }
out=$(nodispatch cargo fmt --all --check 2>&1); rc=$?
nd_check "(5)'s fmt" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nodispatch cargo clippy --workspace --all-targets 2>&1); rc=$?
nd_check "(5)'s clippy" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc (ADR-0102 gates on the exit code)"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }

# ---- (6) THREE FLOORS, CODE LINES ONLY, beneath the behavioural tests — never the assertion. The comment filter is s7i's,
# ---- anchored to the `path:LINE:` prefix grep prints; it CANNOT see a rustfmt continuation line of a product, code
# ---- after a closing `*/`, a renamed identifier or a macro-built one, and says so rather than claiming them away.
# ---- Each floor is ONE function, run first over a PLANTED tree, where it must keep the planted code line and drop the
# ---- planted comment, and only then over the real paths (D21-47, review m14). At fc80687 all three read 0 code lines.
f_integrate() { grep -rn 'GitOp::Integrate' "$@" --include=*.rs; }
f_float() { grep -rnwE 'f32|f64' "$@" --include=*.rs; }
f_walk() { grep -rnE 'read_dir|WalkDir|walkdir|glob::' "$@" --include=*.rs; }
pl="$PWD/target/cp8f-planted"
rm -rf "$pl" && mkdir -p "$pl/src" || { echo "FAIL: could not create the planted tree"; exit 1; }
planted='fn x() -> std::io::Result<()> { let _op = GitOp::Integrate; let _v: f64 = 0.0; let _d = std::fs::read_dir(".")?; Ok(()) }'
printf '%s\n' '// GitOp::Integrate f64 read_dir in a comment' "$planted" > "$pl/src/a.rs" || { echo "FAIL: could not write the planted file"; exit 1; }
for f in f_integrate f_float f_walk; do
  raw=$("$f" "$pl/src"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL (control): $f found nothing in the planted tree (grep rc=$rc) — it could not go red"; exit 1; }
  code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
  [ "$code" = "$pl/src/a.rs:2:$planted" ] || { echo "FAIL (control): $f over the planted tree did not keep exactly the code line and drop the comment:"; printf '%s\n' "$code"; exit 1; }
done
test -d crates/rk-git/src || { echo "FAIL: crates/rk-git/src is absent — the floors below would grep nothing"; exit 1; }
raw=$(f_integrate crates/*/src); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the no-integration floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -v '^crates/rk-git/src/git_exec\.rs:' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: GitOp::Integrate has a CODE-line caller — Phase 1 integrates nothing (design §7.7; §8f.4):"; printf '%s\n' "$code"; exit 1; }
for p in crates/rk-cli/src/verbs/cost.rs crates/rk-cli/src/verbs/status.rs crates/rk-store/src crates/rk-types/src crates/rk-types/src/usage.rs; do
  test -e "$p" || { echo "FAIL: $p is absent — the no-float floor would grep nothing there"; exit 1; }
done
raw=$(f_float crates/rk-cli/src/verbs/cost.rs crates/rk-cli/src/verbs/status.rs crates/rk-store/src crates/rk-types/src); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the no-float floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: a binary float on a CODE line of cost (and the renderer status shares), status, rk-store or rk-types (CostUsd) (ADR-0133 clause 5; AC-374):"; printf '%s\n' "$code"; exit 1; }

# ---- (6b) THE FEEDS (D21-46 R7). The probe reads the three feeds s8c declares, on both toolchains: `backlog` is
# ---- provided by the `backlog` verb, and `logs` / `sbom` stay `unprovided` (§8f.7's list). It reads the TOML on stdin,
# ---- because git-bash's /tmp is invisible to Windows Python. The control comes first. The no-walk floor is CODE lines
# ---- only, beneath backlog::reads_only_the_items_the_store_names_each_by_exact_path — never the assertion. It covers
# ---- ALL of crates/rk-store/src, because every file the producer reaches is in that crate (§8f.7; D21-47, review m14);
# ---- (6)'s planted tree already drove f_walk red.
fd='import sys,tomllib; f=tomllib.load(sys.stdin.buffer).get("feed",{}); print(" ".join(c+"."+k+"="+("verb:"+str(v["verb"]) if "verb" in v else ("unprovided" if "unprovided" in v else "neither")) for c in ("node","rust") for k,v in sorted(f.get(c,{}).items()) if k in ("backlog","logs","sbom")))'
want='node.backlog=verb:backlog node.logs=unprovided node.sbom=unprovided rust.backlog=verb:backlog rust.logs=unprovided rust.sbom=unprovided'
ctl_src=$(printf '[feed.rust.backlog]\nunprovided = "no producer"\n')
ctl=$(python -c "$fd" <<<"$ctl_src"); rc=$?
[ "$rc" -eq 0 ] && [ "$ctl" = 'rust.backlog=unprovided' ] || { echo "FAIL (control): the feed probe read '$ctl' (rc=$rc) over an unprovided backlog — it could not go red"; exit 1; }
test -f .redkiln/registry.toml || { echo "FAIL: .redkiln/registry.toml is absent — the feed probe would read nothing"; exit 1; }
got=$(python -c "$fd" < .redkiln/registry.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = "$want" ] || { echo "FAIL: the feed rows read '$got' (rc=$rc), expected '$want' — the backlog producer is not declared, or §8f.7's list moved without its dated line"; exit 1; }
test -f crates/rk-store/src/backlog.rs || { echo "FAIL: crates/rk-store/src/backlog.rs is absent — the producer the no-walk floor guards does not exist"; exit 1; }
raw=$(f_walk crates/rk-store/src); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the no-walk floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: rk-store walks a directory on a CODE line — the backlog producer reads the ids the store names, by exact path (§8f.7):"; printf '%s\n' "$code"; exit 1; }

# ---- (7) SYNTHETIC CONTROLS — every helper above is driven RED once, or its green means nothing.
( check_sel ctl 1 'test a::b ... ok' a::b ) >/dev/null 2>&1 && { echo "FAIL (control): check_sel accepted an output with no test result line"; exit 1; }
( check_sel ctl 1 "$(printf 'test a::b ... ok\ntest result: ok. 12 passed; 0 failed')" a::b ) >/dev/null 2>&1 && { echo "FAIL (control): check_sel accepted 12 passed as 1"; exit 1; }
( check_sel ctl 1 "$(printf 'test a::c ... ok\ntest result: ok. 1 passed; 0 failed')" a::b ) >/dev/null 2>&1 && { echo "FAIL (control): check_sel accepted a run that did not select a::b"; exit 1; }
ctl=$(printf '%s\n' 'crates/x/src/a.rs:3:    // GitOp::Integrate' 'crates/x/src/a.rs:4:    let op = GitOp::Integrate;' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ "$ctl" = 'crates/x/src/a.rs:4:    let op = GitOp::Integrate;' ] || { echo "FAIL (control): the code-line filter did not keep the code line and drop the comment: $ctl"; exit 1; }
blk_of='import re,sys
q=chr(96)*3
t=open(sys.argv[1],encoding="utf-8").read()
b=[m.group(1) for m in re.finditer(q+"bash\n(.*?)"+q,t,re.S) if sys.argv[2] in m.group(1)]
if len(b)!=1:
    sys.stderr.write("%d fenced bash blocks in %s carry %s\n"%(len(b),sys.argv[1],sys.argv[2])); sys.exit(3)
sys.stdout.buffer.write(b[0].encode("utf-8"))'
python -c "$blk_of" runbook/phase-1/s7i-cost.md 'NO-SUCH-NEEDLE-S8F' >/dev/null 2>&1; rc=$?
[ "$rc" -eq 3 ] || { echo "FAIL (control): the block extractor did not refuse a needle no block carries (rc=$rc)"; exit 1; }

# ---- (8) THIS BLOCK LEFT THE SHIPPED STORE ALONE. Every test above ran in its own temporary repository.
store_after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$store_after" ] || { echo "FAIL: the post-run store probe did not run (rc=$rc)"; exit 1; }
[ "$store_after" = "$store_before" ] || { echo "FAIL: this block moved the shipped store ($store_before -> $store_after; t2_event, t1_stage_transition, t1_gate_transition, t1_slice_seal)"; exit 1; }

# ---- (8b) THE WORKTREE PATH SET IS UNCHANGED BY THIS BLOCK'S OWN SECTIONS, compared BEFORE the nested re-runs, since
# ---- a nested block may perform a REAL `git worktree prune` (Checkpoint 7h's guard (9) does, if 8e re-runs 7h).
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }

# ---- (9) ONLY THE ENTRY CHECKPOINT (8e) AND THE ONE CHECKPOINT THIS STAGE MAKES PROVISIONAL (7i, whose s7i GRAMMAR and
# ---- owns-no-reading branch §8f.10 amends), RE-RUN WHOLE from their own files' bytes; nothing older is re-run here.
# ---- The extractor writes BYTES (Python's text-mode stdout would turn every LF into CRLF on this box, and `bash -c`
# ---- would then read a CR at the end of every command). A block that exits 0 without its end marker did not finish.
# ---- No real `claude` is reachable at this stage: s8h has not landed and no shipped card carries permission_mode.
# ---- D21-46: SKIPPED under RK_CP_NESTED=1 (this block is s8g's entry gate); otherwise all three replays run under the
# ---- (0b) wrapper (`nodispatch`: the compiled fake claude, REDKILN_LAUNCH_RECORD unset, the trap) with RK_CP_NESTED=1,
# ---- and nd_check follows each one. A replayed block that sources the helper reuses this wrapper (ND_ACTIVE=1). 8d is re-run because
# ---- §8f.10 moves its walk-table rows and lane_walk expectations, so it is provisional (D21-47, cross #6).
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8F (9) SKIPPED: RK_CP_NESTED=1 — this block is another checkpoint's entry gate; its re-runs of 8e, 8d and 7i ran at Checkpoint 8f's own close"
else
  replay_whole() {   # $1 file  $2 tag (CP8E …)  $3 what to read on failure
    test -f "$1" || { echo "FAIL: $1 is absent — Checkpoint $2 cannot be re-run (Halt 8)"; exit 1; }
    blk=$(python -c "$blk_of" "$1" "echo \"$2-END-OF-BLOCK reached\""); rc=$?
    [ "$rc" -eq 0 ] && [ -n "$blk" ] || { echo "FAIL: Checkpoint $2's block could not be extracted (rc=$rc) — its end marker is not $2-END-OF-BLOCK, or two blocks carry it"; exit 1; }
    out=$(nodispatch env RK_CP_NESTED=1 FIX_ITEM=rk-fix-doctor-test-count bash -c "$blk" 2>&1); rc=$?
    nd_check "Checkpoint $2's replay" || exit 1
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $2, re-run whole, exited $rc — $3"; printf '%s\n' "$out" | tail -30; exit 1; }
    printf '%s\n' "$out" | grep -qxF "$2-END-OF-BLOCK reached" || { echo "FAIL: Checkpoint $2 exited 0 without reaching its end marker"; printf '%s\n' "$out" | tail -10; exit 1; }
  }
  replay_whole runbook/phase-1/s8e-tier-one-and-the-lane.md CP8E "this stage's entry gate no longer holds"
  replay_whole runbook/phase-1/s8d-step-grain-and-resolver.md CP8D "read §8f.10's s8d row: its closure-record and debt-logging rows and both lane_walk expectations move at this stage"
  replay_whole runbook/phase-1/s7i-cost.md CP7I "read §8f.10's s7i row: its GRAMMAR and its owns-no-reading branch are amended at this stage"
fi

# ---- (9b) THE TRAP IS STILL EMPTY AND THE FAKE UNCALLED at the block's end, after every wrapped run above: the build,
# ---- the selections, the whole suite, fmt, clippy and, outside RK_CP_NESTED, the three replays. Anything in the trap
# ---- means a real claude ran under the wrapper; a logged call means something tried to dispatch.
nd_check "the end of the wrapped runs" || exit 1

# ---- (10) NO STALE REGISTRATION after the nested blocks: a dry-run prune must find nothing.
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }

# ---- (11) AC-518: THE CHECKOUT IS CLEAN. The tracker row may be unstaged between the stage's two close commits
# ---- (00-preamble), so the guard names what it tolerates rather than demanding a bare-empty tree.
st=$(git status --porcelain 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git status could not run (rc=$rc)"; exit 1; }
dirty=$(printf '%s\n' "$st" | grep -v '^ M runbook/PROGRESS\.md$' | grep .)
[ -z "$dirty" ] || { echo "FAIL: the checkout is not clean (AC-518; the stage's work is committed before this block runs):"; printf '%s\n' "$dirty"; exit 1; }

echo "CP8F-END-OF-BLOCK reached"
```

### 8f.14 Close

The stage closes in **two commits**. The first carries the work, the three records, every §8f.10 note, the mutant
results (id, file, the test it flipped, and the flip observed) and the checkpoint output. The second is the tracker
commit, which writes the first commit's hash into `PROGRESS.md` §1 (00-preamble).

**Adversarial refuting rounds continue until a round moves no code and no guard.** Each round refutes against the
full corpus, not a condensation of it.

Each record's id is written into its `REC_` line in Checkpoint 8f (2), by a dated edit, in the commit that writes the
record (D21-47). The records flip only on the operator's dated lines, and Checkpoint 8f (2) is red until they do.
The work commit also carries the dated line in `s8d` §8d.9 that flips both `closure-record` rows and both
`debt-logging` rows to walks. That line marks Checkpoint 8d as provisional, and it is re-run whole in (9).

A `PROGRESS.md` row that still reads *"a LANDED run leaves zero commits"* takes a dated correction at this close:
since `s7h` a run leaves empty commits, and after this stage it leaves content commits. The next stage is `s8g`.
