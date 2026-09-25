# Step 8i — S-BIN: review without a diff, and the `Binary` variant finalised (Phase 1, part 8i)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8i` | Phase 1 | Checkpoint 8h | Checkpoint 8i | `s8h` | `s8j` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** D21-42 decision 2 took the `ChangeSummary` question in two halves: *"**Split the type now,
and run S-BIN before exit.** The `Text` variant (typed diff plus manifest) is finalised now. The `Binary` variant stays
sealed and non-constructible until **S-BIN runs as a Phase-1 sub-stage before Checkpoint 8** (about 10 Opus dispatches
plus UE automation, METHOD-S §3/§7)."* `s8a` took the first half. This stage takes the second: it **runs S-BIN as
registered**, then **finalises the `Binary` variant by record** — the field set S-BIN's findings support, and the
review route they support — and lifts every design and register row that says the type is blocked on an unrun spike,
each with a dated note.

S-BIN's question is METHOD-S's: *"can the reviewer verify an item whose change is not a text diff?"* Its gate is
narrow and stays narrow — ADR-0093 decision 4: *"S-BIN blocks the design of `ChangeSummary` (AC-497, AC-356) and the
Toolchain port's `change_summary` method body; it does not block Phase-1 commits, `rk-types`, `rk-store`, `rk-git`,
the kernel or the adapters."* And the type may not be finalised any other way — ADR-0008's forbids: *"`ChangeSummary`
may not be designed and locked purely against a synthetic or paper fixture; it must be designed against the real UE
project's actual artefacts."* So if S-BIN cannot run, the variant stays sealed and this stage does not close (§8i.6).

**This stage decides nothing about the method.** METHOD-S binds its own changes: *"Any later change is disclosed here
with the date and the records it post-dates"*, and for this spike specifically *"no seed is changed after a
dispatch"*. What the 2026-09-09 entry left open is settled **before the first dispatch** by one dated amendment
(§8i.2), whose fixture decisions are the operator's. The records this stage writes are: that amendment; the S-BIN
results in the corpus repository; a finding draft; one new ADR for the `Binary` variant (next free id at writing — no
id is pre-assigned here); dated register and design notes; and ledger lines for each go-ahead.

**This is the first stage of the Step 8 sequence that spends against the real account.** The stage plan: *"Real
spend happens only at: s8i (S-BIN), s8j (the probes, after the fixture proofs), and s8 (the real runs). Each run gets
an operator go-ahead at that moment."* Every batch in §8i.5 is a separate go-ahead, given in the moment and written
to the ledger before the batch starts.

### 8i.0 What this file's brief said that the tree does not — measured 2026-09-24 at `ca8f387`

The facts brief and the stage plan were right about the question, the cost and the fixture's registration. Ten things
they say or imply are not what the tree holds, and each would have cost a dispatch or a record had it been carried in.

1. **There is no `spikes/runner` in this repository, and the runner has no S-BIN driver.** Step 2 moved the runner to
   `D:/repos/redkiln-corpus/runner` (ADR-0013; `s2`'s *"pinned SHA, fixtures, JSONL and the spike runner all live
   there"*). Its `review-probe` refuses anything but E3's conditions — the bail string reads *"expected clean, s-silent
   or s-subtle"* [measured, `runner/src/main.rs`] — and METHOD-S §6 "How to run" registers commands for S-VAR and
   S-TYPED only. **S-BIN's driver must be built** before anything is dispatched, and it is instrument code (§8i.3).
2. **Results do not go under `spikes/results/`.** `.gitignore` carries `spikes/results/*` and re-admits only
   `RUNBOOK-instrument.txt`. Spike records live in `redkiln-corpus/results/` (ADR-0013) — the precedent is corpus
   commit `f39b2f8`, *"v0+2: Step 7c's live probe and the three D17-6 spike runs, retained raw"*, which did not move
   `CORPUS_PIN`. The corpus working tree is **detached at the pin `61f590e`** while its `main` is at `f39b2f8`
   [measured, `git -C D:/repos/redkiln-corpus branch -a`].
3. **METHOD-S §1 is S-VAR, not a shared-method section.** The shared rules are METHOD-S §0, *"Rules inherited from
   METHOD, METHOD-E3 and METHOD-H"* (isolation, pre-flight checks on every record, *"a void is never a miss"*,
   resume keyed per `(arm, slug, repeat)`, *"nothing is inferred from a single draw"*), and the dispatch ceiling is
   the 3600 s the §7 amendments settled for S-VAR and S-TYPED.
4. **Under the registered scoping every item's `text_diff` is empty.** The 2026-09-09 entry registers
   *"`ChangeSummary.text_diff` carries the item's source and test paths only"*. The non-asset paths of items 1, 3, 4
   and 5 are `CLAUDE.md`, `_workspace/backlog/STATUS.md`, `_workspace/backlog/game_foundation_backlog.json` and, for
   item 1, `_workspace/references/UE58_Editor_And_MCP_Gotchas.md` — all excluded [measured, `git show --numstat
   --format=` on each item commit]. The entry's *"four of five items are not diff-free"* is git's view before the
   scoping. The design's §10 register row goes one step further and is wrong: *"four of the five carry a source/test
   `text_diff` scoped per METHOD-S §7 (2026-09-09); only item 2 is diff-free"*. A dated correction is owed (§8i.8).
5. **The failure route the entry named for seed 5 does not exist at seed 5's review commit.** The entry:
   *"`Inventory_StashLoop.cqtest.cpp` loads `WBP_StashPanel` by hard path, so seed 5 must be checked against that spec
   specifically."* That spec was added by `2b0814f78` (2026-08-31, *"close 8.5.6"*), which is **not an ancestor of
   `bf9c499a1`** [measured, `git merge-base --is-ancestor`]. The same entry's corrected instrument reviews *"each item
   … at its own commit"*, so the greenness check for seed 5 is the whole `OurGame.*` suite as it stood at
   `bf9c499a1`. Recorded in the amendment, not dropped silently.
6. **No capture of any item's own subject is tracked at any item's commit.** `_workspace/runbooks/_captures/` holds
   11 PNGs at the commits of items 1, 2, 4 and 5 and 17 at item 3's, every one a `CameraFidelity_*` or an
   `InventoryGym_*` station capture [measured, `git ls-tree -r`]. Item 4's own commit says where its captures went:
   *"Captures: ScreenShot00034 (before) and ScreenShot00035 (after), named in the row notes since Saved/ is
   gitignored."* So the entry's *"17 PNGs tracked under `_workspace/runbooks/_captures/` at the pin"* feeds no item a
   capture of its own change.
7. **The review tree hands item 4 its answer.** The entry forbids the story row's diagnosis in item 4's `spec.md`, but
   the tree at `a71265925` carries it: `_workspace/backlog/game_foundation_backlog.json` reads *"**Known open at
   hand-off: `WBP_LoadoutScreen`’s `LoadoutFill` alpha is 0.10 where 0.97 was intended, so the left column is
   translucent**"* [measured, `git show a71265925:_workspace/backlog/game_foundation_backlog.json`]. A reviewer with
   `Read` in that tree reads it. The registration scoped `text_diff` and `spec.md` and left the tree.
8. **Halt 2 is live.** `claude --version` reads `2.1.281 (Claude Code)`; `RUNBOOK-instrument.txt` reads
   `2.1.278 (Claude Code)` [measured 2026-09-24]. §8i.5 says what this stage does about it.
9. **The registered decision rule is not total.** METHOD-S §3.4's three branches cover *"2/2 detection and ≤1/3 false
   positive"*, *"A seed missed and its finding would have needed a field the summary lacks"*, and *"Both seeds missed
   with the summary fields cited but not the asset"*. Two outcomes reach none of them: **2/2 detection with ≥ 2/3
   false positives**, and **a seed missed where the reviewer names no missing field and cites no summary field**.
   Golden constraint 10: a bar that cannot decide an outcome is reported as not deciding it, and that is registered
   before the data (§8i.2 item 9).
10. **`spikes/FINDINGS.md`'s index stops at F40** although `F41-DRAFT.md` and `F42-DRAFT.md` exist, and the index says
    *"A later F-number is added here the day its draft is written."* Not this stage's to repair; reported so the S-BIN
    row is not the third missing one.

What the brief got right, re-measured: the BrittonGames pin `041ead2cb284afd3004d07c2826f31d05b2799fd` is HEAD of
`D:/repos/BrittonGames`, with exactly the two modified paths the entry names (`AnimationGym.umap`,
`OurGame.uproject`); the five item commits and both seed parents resolve (`a71265925^` = `cc0cf2d01…`, `bf9c499a1^`
= `b0cff782d…`); every LFS object both seeds need is **present locally** under `.git/lfs/objects`; the engine's
`UnrealEditor-Cmd.exe` is installed at `C:\Program Files\Epic Games\UE_5.8`; `verify_headless.ps1` exists at every
item commit; and the project's last recorded report reads `succeeded 849`, `succeededWithWarnings 180`, `failed 0`,
`notRun 0` over 1029 tests, SHA-256 `c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6` [all
measured 2026-09-24, read-only].

### 8i.1 Entry — what must hold before anything is built or spent

- **Checkpoint 8h passed**, recorded in `PROGRESS.md` §1.
- **`s8a`'s split is on the tree.** The `Text` variant is finalised and the `Binary` slot is sealed, under the names
  and in the crate `s8a`'s record gives them. `s8a`'s file, read at authoring, says *"`ChangeSummary` is defined only
  in `rk-ports`, as an enum with two arms"*, calls the payloads `TextChangeSummary` and `BinaryChangeSummary`, and
  pins the sealed arm with `change_summary::binary_arm_is_not_constructible_outside_rk_ports`. At this stage's open, measure both and write them into §8i.8 by dated
  note before the first Rust edit; every test name below that says `change_summary::` follows the type into that crate.
- **The fixture-proof condition, read before B1 and confirmed by the operator in the B1 go-ahead line.** D21-42's
  closing paragraph: *"The spend gates (the probes, the real runs and S-BIN's dispatches) remain behind D21-40
  decision 2's fixture-proof condition"*. D21-40 decision 2: *"The criterion-(1), (1b), (3)-operational and (7) runs
  may dispatch against the real account **once the sub-stages' fixture proofs pass**, and not before. That includes a
  FakeHarness end-to-end test of `verbs::run::run`"*. The stage plan puts S-BIN before `s8j`'s lane proofs. **The
  reading this file takes:** S-BIN does not dispatch through `redkiln` — no lane, no kernel, no harness adapter — so
  the proofs that gate it are (i) Checkpoint 8h, which stands on every sub-stage proof through `s8h` including `s8e`'s
  FakeHarness end-to-end test, and (ii) S-BIN's own zero-spend dry run (§8i.3). **If the operator reads the
  condition as `s8j`'s lane proofs, this stage stops after B0** and the order is re-cut by a ledger line, never by an
  edit to this file.
- **Operator present, foreground.** D17-5's form: *"In this session, foreground, operator present"*. Every UE
  automation run is the operator's hand (§8i.4).

### 8i.2 The pre-dispatch amendment — what it must settle, and whose decision each item is

One dated entry appended to METHOD-S §7, opening with the literal text **`s8i pre-dispatch amendment`** in its bold
lead (Checkpoint 8i finds its commit by that string), committed to this repository **before B1**. Like the 2026-09-09
entry it changes nothing in METHOD-S §3 — *"the question, the decision rule, the prediction and the sizing stand
exactly as registered"* — and Checkpoint 8i asserts that §3's bytes at HEAD equal §3's bytes at the amendment's
commit, and that every S-BIN record post-dates it. Items marked **(operator)** are fixture decisions the entry left
open or the measurement exposed; this file states the options and a recommendation and takes none of them.

1. **Corrections 4 and 5 of §8i.0**, recorded as measured, with the commands.
2. **(operator) The captures producer** — the decision the entry says in terms it *"does not take"*. (a) `captures`
   empty on every item, recorded as *no producer at the item's commit* (§8i.0 item 6); (b) the operator takes
   captures by hand on each of the five review trees through the project's `/ue-verify` route, before B2, by one
   written procedure applied identically to clean and seeded trees — a seeded tree's capture shows the seed, which is
   exactly what a real `Binary` summary would carry; (c) UE's automation screenshot comparison, which the entry
   measured as *"has never run"* and which would first need its own greenness. Recommended: (b), because it is the
   producer the project uses; (a) if the operator will not take ten perceptual captures by hand.
3. **(operator) What the review tree holds besides the change** (§8i.0 item 7). (a) The tree as the commit has it,
   with the leak recorded as a named threat; (b) the backlog and status prose (`_workspace/backlog/`, and item 1's
   gotchas file) removed from **all five** trees identically, and recorded. Recommended: (b) — the METHOD-S §7 entry already
   excluded the same files from `text_diff` *"because leaving them in would decide §3.5's prediction by fixture
   construction rather than by the reviewer"*, and the tree is the same hazard one directory over.
4. **The fix is unreachable** (F17; METHOD-S §0's `fix_unreachable` pre-flight). F17's measured failure: a reviewer
   with `Bash` *"checked the fix out, reviewed the **real** code"*. So each review tree is a **fresh single-commit
   repository** holding only that tree: a neutral commit message, no remote, no other ref, and neither the fixed
   pointer blob nor its LFS object present. A reviewer that reads or runs `git` against a path outside its review
   tree voids the record as `void: out-of-tree` — METHOD-S §0, *"a void is never a miss"*.
5. **The instrument, named.** The driver and its subcommand (§8i.3); the arm label, carrying every variant (METHOD-S
   §6: *"Arm labels carry every instrument variant"*); model and effort as E3's (`--model opus --effort xhigh`), with
   the model id **observed from each init line** recorded — the set reports one observed id or is reported per id;
   the card's tools `Read` and `Bash` (METHOD-S §3.3: *"the reviewer has `Read` and `Bash` in a disposable clone with
   the engine available"*); `--strict-mcp-config`; METHOD-S §2's typed schema as amended 2026-09-06, scored under the
   **strict** policy (§3.4: *"under the typed schema and strict policy from §2"*).
6. **The rendering.** METHOD-S §3.3's draft fields, *"Rendered into the prompt in place of E3's 'files this change was
   scoped to touch' list"*, under the entry's registered mappings — `text_diff` source and test paths only;
   `assets[].size` read from the LFS pointer (*"never inferred from whether the size line moved"*);
   `succeededWithWarnings` and `notRun` *"folded into `passed`"* with the raw report behind `report_ref`. Each item's
   rendered prompt is written to the results and hashed **before** its first dispatch.
7. **The ceiling**, sized against the slowest measured greenness run (golden constraint 6: *"When two conditions differ
   in shape, size the ceiling against the slow one"*), never below 3600 s, and the same for every dispatch.
8. **Order.** Pass 1 is items 1–5 in order; pass 2 is items 1–5 in order; strictly sequential, one UE process at a
   time (Halt 6).
9. **(operator) The two outcomes the rule does not decide** (§8i.0 item 9). Registered here as *"not decided by the
   registered rule"*, reported as exactly that, and handed to the record with that stated. Recommended: for the
   `Binary` variant an undecided pass takes the **human-gate route** (§8i.8) — the conservative direction, since an
   automated verdict on a class with no evidence is what design §0.1 forbids.
10. **The caps and the void rule** (§8i.5): one retry per `(pass, item)` at the same ceiling, resume keyed per
    `(arm, item, pass)`.
11. **The CLI version** (§8i.5): recorded on every record; the set runs under one version or is reported per version.

A seed that goes red at its greenness check is handled as registered: *"If a seed goes red it is replaced from the
remaining 27 asset-only commits and this amendment is extended, never quietly edited."* Extension happens before B2;
after the first Opus dispatch no seed, item, rendering or ceiling moves. A change that turns out to be needed after
that point discards the set, as METHOD-S's own 2026-09-05 entry did (*"after 10 dispatches, all of which are
DISCARDED"*), and is re-registered.

### 8i.3 The instrument — an S-BIN driver in the corpus runner, test-first

**Where.** `D:/repos/redkiln-corpus/runner`, a new module `src/sbin.rs` behind a new subcommand (named at writing and
recorded in the amendment). Every write to `D:/repos/redkiln-corpus` is the operator's, or made on the operator's
go-ahead in the moment — `s7b` §7b.11 precondition 7 reserved *"the corpus write and the pin edit"* to the operator,
and `f39b2f8` is the precedent for a results commit that leaves `CORPUS_PIN` alone. The operator puts the corpus on
`main` and clean before B0 (it is detached at the pin today, §8i.0 item 2). Halt 1 and golden constraint 3 apply:
`cargo build` in `runner/` before every use of its binary.

**Named tests**, run as `cargo test --bin redkiln-spike-runner -- sbin::tests::<name> --exact`, each authored and
**watched red for the stated reason** before the code that turns it green (golden constraint 12's order). Each mutant
is authored and run **from Python, never the shell**, and must flip its test.

| test (`sbin::tests::`) | pins | mutant that must flip it |
| --- | --- | --- |
| `seed_is_confirmed_by_blob_oid_not_by_file_count` | the entry's *"each seed is confirmed by blob oid … and not by a file count"* | compare `git diff --stat` counts |
| `review_repo_holds_one_commit_and_no_fix_object` | §8i.2 item 4 | check the working tree only, not the object store and `.git/lfs` |
| `text_diff_carries_source_and_test_paths_only` | §8i.2 item 6 | admit `_workspace/backlog/` |
| `asset_size_is_read_from_the_lfs_pointer` | §8i.2 item 6 | infer size from whether the pointer's size line moved |
| `warned_and_not_run_fold_into_passed_with_the_raw_report_referenced` | §8i.2 item 6 | drop `succeededWithWarnings` |
| `a_digest_failing_the_typed_schema_is_void_schema_not_a_miss` | METHOD-S §0 rule 2 | score the invalid digest |
| `exit_zero_without_structured_output_is_void_not_success` | Halt 3 | read exit 0 as a completed dispatch |
| `strong_detection_needs_a_blocking_finding_naming_the_seeded_path` | METHOD-S §3.4 primary | credit any blocking finding |
| `cited_fields_are_extracted_mechanically_from_the_digest` | METHOD-S §3.4 *"Recorded"* | read cited fields from the prose |
| `an_out_of_tree_read_voids_the_record` | §8i.2 item 4 | audit `Read` paths only, not `Bash` commands |
| `api_key_source_other_than_none_voids_before_scoring` | D21-42 17a | skip the init check |
| `a_version_move_between_dispatches_halts_the_set` | Halt 2 | compare against the floor, not the set's first version |
| `the_dispatch_cap_refuses_before_spawning` | §8i.5 caps | check the cap after the child is spawned |
| `resume_is_keyed_per_arm_item_pass` | METHOD-S §0 | key on `item` alone |
| `a_surviving_unreal_process_halts_before_the_next_dispatch` | Halt 6 | skip the process check |

**The audit.** `results/audit-s.py` gains a `bin` subcommand: per pass and per item, detection on each seed (strong,
under strict), approval on each clean item, the fields cited, whether the reviewer ran the automation itself, and
wall clock — **no figure pools the two passes** (the 2026-09-11 entry: *"no figure pools them, and a 2/2 in one pass
beside a 1/2 in the other is reported as exactly that"*). It is checked against a hand-built fixture of three records
before it reads a real one.

**The zero-spend dry run — S-BIN's fixture proof.** The driver runs end to end over item 1's real review tree with the
runner's global `--claude-bin` pointed at a stub that replays a recorded stream; the record it writes carries
`"dry_run": true`, goes to `results/s-bin/dry-run.jsonl` and **never** to `results/s-bin.jsonl`, and the audit reads
it. Golden constraint 4 is the reason: *"A rig that compiles and passes its unit tests has NOT been run."* The test
run's full output is kept as `results/s-bin/driver-tests.txt`.

### 8i.4 The fixture — five review trees, two seeds, and the operator's UE runs

Everything here costs no model spend, and all of it happens in B0, before B1.

**Materialise each review tree outside both repositories** under one root (named in the amendment and in the finding's
pins as `review_root`), from `D:/repos/BrittonGames` read-only: a disposable clone at the item's commit whose LFS
objects are read from BrittonGames' own store, so BrittonGames' `.git` gains no worktree registration and its working
tree, with its two modified paths, is never touched. Prove the LFS read rather than assume it: for every asset in the
change, the materialised file's SHA-256 equals the `oid` in its pointer. Then apply the seed as registered —

```bash
# item 4, in a tree at a71265925 (registered 2026-09-09; METHOD-S §7)
#   git checkout a71265925^ -- OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset
# item 5, in a tree at bf9c499a1
#   git checkout bf9c499a1^ -- OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset OurGame/Content/UI/Inventory/WBP_StashPanel.uasset
```

— apply §8i.2 item 3's decision identically to all five, and commit each tree as the **only** commit of a fresh
repository (§8i.2 item 4). Confirm each seed by blob oid (`git rev-parse <seed>^:<path>` against the fresh
repository's `git rev-parse HEAD:<path>`), and confirm the fix unreachable (the fixed pointer blob absent from the
object store, its LFS object absent from `.git/lfs/objects`, `git rev-list --all --count` = 1). Write all of it to
`results/s-bin/fixture.json`.

**The operator's hand, with the editor closed** (`verify_headless.ps1`: *"When the editor is RUNNING, do not use this
script"*):

1. In each of the five review trees: `pwsh -File _workspace/tooling/verify_headless.ps1` — the default filter, every
   `OurGame.*` test, build then run. The script writes `_workspace/tooling/TestResults/` **inside that tree**, never
   in BrittonGames.
2. The **greenness** the registration requires: `failed` = 0 and exit 0 on both seeded trees. *"a seed the suite
   catches measures nothing about review-without-a-diff."* A red seed is replaced as §8i.2 registers, before B2.
3. Each tree's `index.json` and `headless.log` are copied to `results/s-bin/greenness/item-<n>/` and hashed; that
   report is the item's `tests.report_ref`.
4. The build and suite wall clock of each run is recorded; the slowest sets §8i.2 item 7's ceiling.
5. If §8i.2 item 2 took (b), the captures are taken now, by the written procedure, on each tree.
6. The pin's own 2026-09-05 report is copied as registered (*"Before dispatch it is copied into the spike's own
   results directory with its SHA-256 recorded"*) to `results/s-bin/pin-report-2026-09-05/index.json`; its SHA-256
   must be the `c3dae802…6bc6` measured at authoring (§8i.0), or it was overwritten after 2026-09-05 and the copy is
   not the registered report.

### 8i.5 Spend — the batches, the caps, the environment, the version

| batch | what | model spend | gate |
| --- | --- | --- | --- |
| **B0** | driver and audit built test-first (§8i.3); dry run; five review trees, seeds by blob oid, fix unreachable (§8i.4); the operator's five UE runs; captures if chosen; the amendment committed | none | operator go-ahead for the corpus writes |
| **B1** | at most **3 haiku** flag probes on a throwaway tree — the card adopted with `Read`+`Bash` under `--json-schema` (F20's measured hazard: *"an agent card's `tools` allowlist silently disables `--json-schema`"*), `apiKeySource` `none`, the init tool set equal to the card's. Not S-BIN records. | ≤ 3 haiku | **go-ahead B1**, which also confirms §8i.1's fixture-proof reading |
| **B2** | pass 1: items 1–5 | 5 Opus | **go-ahead B2** |
| **B3** | pass 2: items 1–5, after pass 1's audit has been read | 5 Opus | **go-ahead B3** |
| retries | a void retried once at the same ceiling, inside its batch; each named in the ledger | ≤ 4 Opus in all | inside B2/B3's go-ahead |
| **B4** | only if a pass fires METHOD-S §3.4 branch 2 (*"add the field, re-run that item"*): a further dated amendment adds the field to the rendering, then that item is re-run in each pass that fired | ≤ 4 Opus | **go-ahead B4**, after its amendment |

**The caps, enforced by the driver before it spawns** (`the_dispatch_cap_refuses_before_spawning`): **18 Opus
dispatches** and **3 haiku**, counting voids, and **$90 API-equivalent** summed over every S-BIN record's
`total_cost_usd`, probes included. The warrant: METHOD-S §3.6 sizes S-BIN at *"**10 dispatches**, ~$30 plus UE
automation wall clock"*; S-TYPED's median was *"$3.03/review"* (METHOD-S §7, 2026-09-11 item 3), and a reviewer that
builds and runs a UE suite is slower, so 18 × ~$5 bounds it. Reaching either cap halts the stage; raising one is a
ledger line, never an edit here. Throttle margins are the runner's (METHOD-S §0, *"throttle margins honoured"*).

**The go-ahead line.** Before each batch the operator writes, in the ledger entry this stage opens (next free D21 id at
writing), one dated line of the fixed form `S-BIN go-ahead B<n> <UTC time> — <scope>, <caps used so far>`.
Checkpoint 8i reads them. D21-40 decision 2 binds every batch: *"If a real run is refused unexpectedly, the next step
is to stop and report, not to spend again."*

**The environment.** D21-42 17a: *"A committed launcher script is the only entry to a real run. It refuses
`CLAUDECODE`/`CLAUDE_CODE_*` and denylisted names, sets `CLAUDE_CONFIG_DIR` explicitly, freezes auto-update and
captures the version. The adapter refuses unless init reports `apiKeySource == "none"`."* S-BIN launches through
`s8h`'s launcher if it exposes an entry for an arbitrary child; if it does not, the operator's shell applies the same
refusals first, and the driver enforces the `apiKeySource` half itself
(`api_key_source_other_than_none_voids_before_scoring`). The account is D21-42 17b's: *"`~/.claude-personal`."*

```bash
# The operator's shell before B1..B4 — a Git Bash OUTSIDE Claude Code (s7c §7c.15's rule). Refusals, never strips.
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: a Claude-spawned shell — CLAUDECODE/CLAUDE_CODE_* would reach the child (s7c §7c.15)"; exit 1; }
for v in ANTHROPIC_API_KEY ANTHROPIC_AUTH_TOKEN ANTHROPIC_BASE_URL ANTHROPIC_FOUNDRY_API_KEY GITHUB_TOKEN GH_TOKEN GH_ENTERPRISE_TOKEN; do
  [ -z "${!v:-}" ] || { echo "FAIL: $v is set — a denylisted name (s7c §7c.6); unset it outside the process, never strip it"; exit 1; }
done
export CLAUDE_CONFIG_DIR="$HOME/.claude-personal"
test -d "$CLAUDE_CONFIG_DIR" || { echo "FAIL: no $CLAUDE_CONFIG_DIR — D21-42 17b's account is not on this box"; exit 1; }
ver=$(claude --version 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ver" ] || { echo "FAIL: claude --version did not run (rc=$rc)"; exit 1; }
echo "S-BIN instrument version at this batch: $ver — every record carries it; a move since the set's first record HALTS (Halt 2)"
```

**The version, and why the pin does not move here.** Halt 2 fires at this stage's open (§8i.0 item 8): *"Record it; do
not silently continue."* It is recorded — in `PROGRESS.md`, in every record, in the finding. `TESTED_CLI_VERSION`
is `rk-adapter-claude`'s pin, and S-BIN does not dispatch through that adapter; D21-42 16 moves it *"**once**, at probe
time"*, which is `s8j`. What S-BIN needs is that its own set does not straddle a version: auto-update is frozen by
the launcher's mechanism from B1 to the last dispatch, and `a_version_move_between_dispatches_halts_the_set` enforces
it (*"Any set that spans a version boundary is reported per-version or it is not reported"*).

### 8i.6 Kill and abort conditions

Each of these stops the stage where it stands. The batch in progress is recorded as far as it went, the ledger gets a
dated line, and nothing resumes without a new go-ahead.

- **The runner does not build** (Halt 1), or a driver test is not observed red first.
- **The dry run fails, or any B1 probe shows** `--json-schema` inert, a tool set unequal to the card's, or
  `apiKeySource` other than `none` (Halt 3; F20). No Opus dispatch follows a failed B1.
- **A seed is caught by the suite** and no replacement from the 27 asset-only commits passes greenness — the spike
  cannot run as registered.
- **A fix is reachable** in any review tree, or a seed's blob oid does not match.
- **The CLI version moves** between the set's first record and any later dispatch (Halt 2).
- **A UE process survives a dispatch** (Halt 6); the tree it held is rebuilt before any further dispatch.
- **The same item voids twice** in one pass — a fixture defect, not noise.
- **A cap is reached** (§8i.5).
- **The account is refused unexpectedly** (D21-40 decision 2).
- **Anything in METHOD-S §3, a seed, an item, the rendering or the ceiling would have to change after the first Opus
  dispatch** — the set is discarded, re-registered, and re-run under a new go-ahead.

**If S-BIN cannot run to completion, the `Binary` variant stays sealed** and this stage does not close. ADR-0008
forbids the paper route (*"may not be designed and locked purely against a synthetic or paper fixture"*); D21-42
decision 2 requires S-BIN *"before Checkpoint 8"*. Whether Phase 1 may exit with the variant sealed is a new operator
decision in the ledger — it would supersede that half of D21-42 decision 2 — and is not taken by this file.

### 8i.7 What is recorded, and where

**In the corpus (`D:/repos/redkiln-corpus`, one commit on `main`, `CORPUS_PIN` not moved — the `f39b2f8` precedent):**
`results/s-bin.jsonl` (append-only; every record, voids included); `results/s-bin/raw/` (each dispatch's stdout,
verbatim); `results/s-bin/rendered/item-<n>.txt`; `results/s-bin/greenness/item-<n>/`; `results/s-bin/fixture.json`;
`results/s-bin/pin-report-2026-09-05/index.json`; `results/s-bin/dry-run.jsonl`; `results/s-bin/driver-tests.txt`;
`results/s-bin/audit.txt`; `runner/src/sbin.rs`; `results/audit-s.py`.

**Every S-BIN record** carries, beside the digest: `spike` = `"s-bin"`, `pass`, `item`, `arm`, `void` (`null` or the
named reason), `rerun` (B4 only), `dry_run`, `cli_version`, `observed_model`, `api_key_source`, `fix_unreachable`,
`seed_blob_verified` (`null` on a clean item), `ceiling_secs`, `started_at` (unix seconds), `wall_secs`,
`total_cost_usd`, `ran_automation`, `cited_fields`, `detected` (`null` on a clean item), and `policy_verdict`
(strict).

**In this repository:** the METHOD-S §7 amendment (§8i.2), and B4's if one is needed; the finding draft
`spikes/F<next>-DRAFT.md` (next free F id at writing), headed `## F<n> — S-BIN …`, in F31's form — *Established by*
(dispatches, dates, arm, model, CLI version, corpus commit, cost), each pass's table with its detection, false
positives and cited fields, METHOD-S §3.4 evaluated **per pass**, §3.5's prediction against the outcome, the voids
named, and the limits (§3.6: *"n=5 is a type-design sample, not a rate"*; the entry's perceptual-seed selection) —
**and a pins block**, which Checkpoint 8i parses, of exactly this form:

```text
<!-- s-bin-pins
corpus_commit=<40-hex corpus commit holding every file below>
cli_version=<x.y.z>
observed_model=<the one model id every valid record's init reported>
ceiling_secs=<n>
review_root=<the absolute directory the review trees were built under>
file=<corpus-relative path> sha256=<64 hex>
-->
```

with one `file=` line per corpus file listed above and per raw transcript. Also: a row in `spikes/FINDINGS.md`'s
index; the ledger entry with every go-ahead; the `Binary` record (§8i.8); the dated notes (§8i.8); and the tracker rows.

### 8i.8 From the finding to the `Binary` variant

**The record.** One new ADR, the next free id at writing, in ADR-0085's form as D21-42 24a enforces it from ADR-0164
on (`### Option — <name>` headings; a **What this forbids** block). Its evidence names the F id, `S-BIN` and the corpus
commit. Whether it **supersedes `s8a`'s record in part** or only relates to it is ADR-0092 rule 1's test, applied to
the text `s8a` actually wrote: if it moves a `## Decision` sentence or a forbids bullet of `s8a`'s record, it is a
supersession-in-part; if `s8a`'s record already says the slot opens by record once S-BIN reports, it is execution under
it. Its `## Decision` carries three lines Checkpoint 8i parses:

- `Binary field set (<N> fields): <the fields, each named>` — the fields the findings support, and no field the
  findings do not;
- `Binary review route: reviewer` **or** `Binary review route: human gate`;
- `Binary producers in Phase 1: none` **or** the source files that construct one.

It must also say, by name, what evidence a `binary`-class AC gets. The design refused to switch the per-AC mutation
norm off for a whole class blind (*"a substantive gate decision"*), and AC-356's struck clause left it on *"AC-337's
`not-feasible(reason)` route until S-BIN reports"*. The record either keeps that route or changes it, and says which.

**How the finding maps to the record** — per pass, never pooled:

| what the passes show | the record |
| --- | --- |
| branch 1 in **both** passes (*"2/2 detection and ≤1/3 false positive"*) | the field set as registered in METHOD-S §3.3 and the entry's mappings, less any field no reviewer cited only if the record argues it; **route: reviewer** |
| branch 2 in a pass | the field the reviewer's own prose names is added (B4); if the re-run detects, **route: reviewer** with it; if not, the pass is read as branch 3 |
| branch 3 in a pass (*"binary review needs a *rendered* representation … recorded as a requirement"*) | **the variant is not dropped.** It keeps the manifest and test fields the reviewers did cite, and **route: human gate** — an item with a `Binary`-classified path cannot receive an automated review verdict; the review stage opens a human gate carrying the typed summary (AC-357's shape) until a producer of the rendered representation exists. The requirement is recorded against ADR-0008's Phase-2 Unreal adapter |
| the passes disagree | the more conservative route, and the disagreement reported as a stability finding |
| an outcome the rule does not decide | as the amendment registered (§8i.2 item 9), stated as such |

**If the answer is "no, the reviewer cannot verify without a diff"**, the design has already written the falsifier,
and the record answers it clause by clause rather than by silence: *"`ChangeSummary` is not a type problem but an
evidence problem. The review gate's applicability to binary-asset toolchains is withdrawn, ADR-0008's Phase-2 Unreal
adapter loses its premise, and the five toolchain-keyed kinds (ADR-0073) are re-cut."* The `Binary` variant then
encodes that answer — the human-gate route and the recorded requirement — and each of the three consequences is
either taken in the record or named as owed with an owner.

**The Rust — named, red first, each with its mutant.** `CS` below is the crate `s8a`'s record homes the type in
(`rk-ports` at `ca8f387`; ADR-0099 clause 5). `s8a`'s own test that the slot is sealed —
`change_summary::binary_arm_is_not_constructible_outside_rk_ports`, with its `trybuild` case — is **superseded by
name** in the same commit, and so is Checkpoint 8a's guard that the sealed payload has no `pub` field. Checkpoint 8a
names both, so row 8a becomes `passed <hash>, re-run pending` (the preamble's maintenance rule) and `s8a`'s block
takes a dated amendment. Whether the Rust adapter's
`change_summary::change_summary_refuses_a_range_carrying_a_binary_path` stands follows the record's `Binary producers
in Phase 1` line: with `none` it stands unchanged.

| test | where | red because | mutant |
| --- | --- | --- | --- |
| `change_summary::binary_is_constructible_only_through_its_recorded_constructor` | `CS`, `--test it` | the slot is sealed; nothing constructs | an unchecked constructor |
| `change_summary::binary_rejects_a_text_patch_for_a_binary_classified_path` | `CS` | no check exists | accept the patch (AC-497's AND) |
| `ui::trybuild_pins_the_binary_change_summary_construction_rule` | `CS`; a new `tests/ui` case with a committed `.stderr` | its committed `.stderr` names the recorded constructor rule, and today the case fails for `s8a`'s sealed-slot reason instead | make the fields `pub` |
| `change_summary::binary_field_<name>`, one per field in the record | `CS` | the field does not exist | named per field in the record (e.g. size inferred rather than read from the pointer) |
| `render::ac154_binary_entries_render_from_typed_fields` | `rk-store`, a golden file pinned `-text` (ADR-0151) | no binary rendering | drop a typed field |
| `render::ac154_no_binary_content_is_embedded_parsed_or_diffed_as_text` | `rk-store` | no binary rendering | embed the asset's bytes |
| `review_pass::ac497_binary_classification_assembles_no_diff_and_carries_the_typed_summary` | `rk-cli` | the review input has no `Binary` path | call the diff assembler for a `Binary` path |
| `review_pass::binary_item_reaches_the_reviewer_with_the_typed_summary` **or** `review_pass::binary_item_refuses_an_automated_verdict_and_opens_a_human_gate` — the one the record's route names | `rk-cli` | no route exists | the other route's behaviour |
| `classify::ac497_classify_binary_paths` (exists) | both toolchain adapters | — (re-run) | — |

If `s8f` named rk-store's render module otherwise, the two `render::` tests follow it and this table takes a dated
note first. Mutant cycles run **from Python, never the shell**.

**Statuses, stated honestly and by dated note — the register owns the wording** (ADR-0092):

- **AC-154** — to binding only if the record is accepted **and** both `render::ac154_*` tests pass; otherwise it stays
  PROVISIONAL with the reason written.
- **AC-356** — binding if the route is `reviewer`; if it is `human gate`, its THEN (*"binary paths are represented by
  manifest plus capture references and never by text diff"*) is amended by dated note to what the record fixed, and
  its evidence standard is the record's.
- **AC-497** — its *"the reviewer receives a typed `ChangeSummary` value"* holds on the reviewer route; on the
  human-gate route the register amends the recipient to the gate node, and the `ac497_*` test asserts the amended
  wording.
- **AC-475** — S-BIN reported, but its note says *"When S-BIN settles the type, this criterion is replaced by a lift
  criterion with its own measured share"*. That measurement is not this stage's and has no owner; it stays
  PROVISIONAL with that said, and the owed lift criterion is raised in the tracker.
- **AC-049** — not moved: its own note scopes S-BIN to *"the `ChangeSummary` type design only (ADR-0093 decision 4)"*.

Each of the three criteria's blocks gains a note in exactly this form, which Checkpoint 8i finds:
`*Amended <date> (s8i, S-BIN reported — F<n>):* …`. And **every line** of the design and the register that states
S-BIN's pending state — at `ca8f387`, **21** design lines and **17** register lines carry `S-BIN` beside one of
`unrun`, `UNRUN`, `not been run`, `NOT been run`, `until S-BIN`, `before S-BIN`, `S-BIN reports` or `BLOCKED`
[measured] — gets an inline dated note containing `[S-BIN reported (s8i)` and is never rewritten. That set includes
design §0.1's `ChangeSummary` row, the status line's *"Four spikes gate that label — **S-BIN**, …"* (S-BIN leaves the
design-final gate; the other three stay), the §10 register rows, register §0.3's row 3, and the design sentence §8i.0
item 4 corrects.

**Two guards whose condition lapses here, and whose they are.** Checkpoint 5's `ChangeSummary` line bans the type
outside its home *"until S-BIN reports"* (ADR-0099 clause 5's words, as `s8a`'s record restated them): if `s8a`'s
record kept that condition, it lapses in this stage's record commit and `s5`'s line takes a dated amendment in the
same commit, because the render and review sites above name the type. And `s8`'s standing prohibition, which `s8a`'s
file rewrites to *"no `Binary` arm constructed outside `rk-ports`, no `ChangeSummary` definition outside it"*: the
render and review tests above construct `Binary` outside `rk-ports`, and `s8`'s guard greps all of `crates/`, tests
included. So this stage's record commit amends that bullet and guard by dated note to the record's construction rule
— the stage whose change makes a guard false owns its amendment, as `s8a` did for the same bullet.

### 8i.9 Checkpoint 8i

- **The S-BIN records exist and are pinned** — the finding draft, its pins, every pinned corpus file byte-for-byte at
  the pinned commit on the corpus's `main`, ten valid records one per `(pass, item)`, one CLI version, one model,
  `apiKeySource` `none`, the fix unreachable and the seeds verified on every one, the caps not exceeded, and the
  amendment committed before the first record with METHOD-S §3 unchanged since.
- **The driver's tests pass at the pinned corpus commit**, rebuilt from the object store, not from a working tree.
- **The `Binary` variant is constructible only as the record says** — the record exists and is accepted, and its
  named tests pass in the counts its field set implies; no `Binary` construction appears on a code line outside the
  type's crate except where the record names a producer.
- **AC-154, AC-356 and AC-497 carry their dated notes**, and no design or register line still states S-BIN as pending
  without one.
- **The whole suite is green**, fmt and clippy clean, and Checkpoints 8h and 5 re-run whole and green.
- **This block dispatches nothing**: it spawns no `claude` of its own, it refuses to run an extracted block that
  carries a dispatch shape, and it fails if a new session transcript appears in the account while it runs.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`. Every cargo reading is
# `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`, MODULE-QUALIFIED and `--exact`, and asserts
# each test's own `... ok` line AND an exact count: a bare name under `--exact` runs zero tests and exits 0.
case "$(uname -s)" in MINGW*|MSYS*) : ;; *) echo "FAIL: this block runs on the Windows box that holds the corpus"; exit 1;; esac
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — (0)'s tripwire reads new session transcripts, and a Claude session writes them"; exit 1; }
T=$(mktemp -d); rc=$?
[ "$rc" -eq 0 ] && [ -d "$T" ] || { echo "FAIL: no scratch directory (rc=$rc)"; exit 1; }

# ---- (0) THE WORKTREE PATH SET and THE NO-DISPATCH TRIPWIRE, both read FIRST. A dispatch by anything this block
# ---- runs — a test, an extracted checkpoint — starts a session whose transcript is a NEW *.jsonl under the account.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
CFG="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
test -d "$CFG/projects" || { echo "FAIL: no $CFG/projects — the tripwire would compare two empty listings"; exit 1; }
sess_before=$(find "$CFG/projects" -type f -name '*.jsonl' | sort)
test -n "$sess_before" || { echo "FAIL: the account holds no session transcript — the tripwire has no baseline"; exit 1; }

# ---- (1) BUILD-OR-FAIL.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8i NOT RUN (Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (2) THE FINDING DRAFT AND ITS PINS. Found by its headline, never by a pre-assigned number.
drafts=$(grep -lE '^## F[0-9]+ — S-BIN' spikes/F*-DRAFT.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: no S-BIN finding draft (grep rc=$rc) — S-BIN has not reported"; exit 1; }
[ "$(printf '%s\n' "$drafts" | grep -c .)" -eq 1 ] || { echo "FAIL: more than one S-BIN draft:"; printf '%s\n' "$drafts"; exit 1; }
DRAFT=$drafts
SBIN_F=$(sed -n 's/^## \(F[0-9][0-9]*\) — S-BIN.*/\1/p' "$DRAFT")
[ -n "$SBIN_F" ] && [ "spikes/$SBIN_F-DRAFT.md" = "$DRAFT" ] || { echo "FAIL: the headline id '$SBIN_F' does not name its own file $DRAFT"; exit 1; }
grep -qF "| $SBIN_F | " spikes/FINDINGS.md || { echo "FAIL: spikes/FINDINGS.md's index has no $SBIN_F row"; exit 1; }
pins=$(sed -n '/^<!-- s-bin-pins$/,/^-->$/p' "$DRAFT")
[ -n "$pins" ] || { echo "FAIL: $DRAFT carries no s-bin-pins block (§8i.7)"; exit 1; }
pin() { printf '%s\n' "$pins" | sed -n "s/^$1=//p"; }
SBIN_CORPUS=$(pin corpus_commit); SBIN_CLI=$(pin cli_version); SBIN_MODEL=$(pin observed_model); SBIN_CEIL=$(pin ceiling_secs); SBIN_ROOT=$(pin review_root)
printf '%s\n' "$SBIN_CORPUS" | grep -qxE '[0-9a-f]{40}' || { echo "FAIL: corpus_commit is not exactly one full sha: '$SBIN_CORPUS'"; exit 1; }
printf '%s\n' "$SBIN_CLI" | grep -qxE '[0-9]+\.[0-9]+\.[0-9]+' || { echo "FAIL: cli_version is not exactly one version: '$SBIN_CLI'"; exit 1; }
[ -n "$SBIN_MODEL" ] && [ -n "$SBIN_ROOT" ] || { echo "FAIL: observed_model or review_root is not pinned"; exit 1; }
[ "$SBIN_CEIL" -ge 3600 ] 2>/dev/null || { echo "FAIL: ceiling_secs '$SBIN_CEIL' is not a number >= 3600 (§8i.2 item 7)"; exit 1; }

# ---- (3) EVERY PINNED FILE, BYTE-FOR-BYTE, FROM THE CORPUS OBJECT STORE — never from its working tree (AC-464's rule).
CORPUS_REPO=$(sed -n 's/^CORPUS_REPO=//p' CORPUS_PIN)
test -d "$CORPUS_REPO/.git" || { echo "FAIL: CORPUS_PIN names no repository ('$CORPUS_REPO')"; exit 1; }
git -C "$CORPUS_REPO" cat-file -e "$SBIN_CORPUS^{commit}" || { echo "FAIL: corpus commit $SBIN_CORPUS does not exist"; exit 1; }
git -C "$CORPUS_REPO" merge-base --is-ancestor "$SBIN_CORPUS" main || { echo "FAIL: $SBIN_CORPUS is not on the corpus's main — a dangling commit is not a record"; exit 1; }
files=$(printf '%s\n' "$pins" | sed -n 's/^file=//p')
for req in results/s-bin.jsonl results/s-bin/fixture.json results/s-bin/driver-tests.txt results/s-bin/dry-run.jsonl results/s-bin/audit.txt results/s-bin/pin-report-2026-09-05/index.json runner/src/sbin.rs results/audit-s.py; do
  printf '%s\n' "$files" | grep -q "^$req sha256=" || { echo "FAIL: the pins block does not pin $req"; exit 1; }
done
for n in 1 2 3 4 5; do
  for req in "results/s-bin/greenness/item-$n/index.json" "results/s-bin/rendered/item-$n.txt"; do
    printf '%s\n' "$files" | grep -q "^$req sha256=" || { echo "FAIL: the pins block does not pin $req"; exit 1; }
  done
done
nraw=$(printf '%s\n' "$files" | grep -c '^results/s-bin/raw/')
[ "$nraw" -ge 10 ] || { echo "FAIL: $nraw raw transcripts pinned; ten dispatches retain ten"; exit 1; }
while read -r path want; do
  want=${want#sha256=}
  git -C "$CORPUS_REPO" cat-file blob "$SBIN_CORPUS:$path" > "$T/blob"; rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $path is not in corpus commit $SBIN_CORPUS"; exit 1; }
  got=$(sha256sum "$T/blob"); got=${got%% *}
  [ "$got" = "$want" ] || { echo "FAIL: $path is $got, pinned $want"; exit 1; }
done <<< "$files"
git -C "$CORPUS_REPO" cat-file blob "$SBIN_CORPUS:results/s-bin/pin-report-2026-09-05/index.json" > "$T/pinreport"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the pin's report is not in the corpus commit"; exit 1; }
got=$(sha256sum "$T/pinreport"); got=${got%% *}
[ "$got" = "c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6" ] || { echo "FAIL: the copied pin report is not the 2026-09-05 report measured at authoring (§8i.4 step 6)"; exit 1; }

# ---- (4) THE RECORDS SAY WHAT THE PINS SAY: one valid record per (pass, item), one version, one model, `none`, the fix
# ---- unreachable, the seeds verified, the caps held. JSON is parsed, not grepped.
git -C "$CORPUS_REPO" cat-file blob "$SBIN_CORPUS:results/s-bin.jsonl" > "$T/s-bin.jsonl"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/s-bin.jsonl" ] || { echo "FAIL: results/s-bin.jsonl is absent or empty at the pinned commit"; exit 1; }
probe='import json,sys
recs=[json.loads(l) for l in open(sys.argv[1],encoding="utf-8") if l.strip()]
sb=[r for r in recs if r.get("spike")=="s-bin" and not r.get("dry_run")]
ok=[r for r in sb if r.get("void") is None and not r.get("rerun")]
keys=sorted((int(r["pass"]),int(r["item"])) for r in ok)
want=[(p,i) for p in (1,2) for i in (1,2,3,4,5)]
print("pairs", "ok" if keys==want else "BAD"+repr(keys))
print("cli", ",".join(sorted({str(r.get("cli_version")) for r in sb})))
print("model", ",".join(sorted({str(r.get("observed_model")) for r in ok})))
print("aks", ",".join(sorted({str(r.get("api_key_source")) for r in sb})))
print("unreach", all(r.get("fix_unreachable") is True for r in ok))
print("seed", all(r.get("seed_blob_verified") is True for r in ok if int(r["item"]) in (4,5)))
print("ceil", ",".join(sorted({str(r.get("ceiling_secs")) for r in sb})))
print("first", min(int(r["started_at"]) for r in sb))
print("opus", sum(1 for r in sb if "haiku" not in str(r.get("observed_model"))))
print("cost", int(sum(float(r.get("total_cost_usd") or 0) for r in sb)))'
pv=$(python -c "$probe" "$T/s-bin.jsonl"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$pv" ] || { echo "FAIL: the record probe did not run (rc=$rc)"; exit 1; }
rv() { printf '%s\n' "$pv" | sed -n "s/^$1 //p"; }
[ "$(rv pairs)" = "ok" ] || { echo "FAIL: not exactly one valid record per (pass, item): $(rv pairs)"; exit 1; }
[ "$(rv cli)" = "$SBIN_CLI" ] || { echo "FAIL: the records span CLI versions '$(rv cli)', pinned $SBIN_CLI (Halt 2)"; exit 1; }
[ "$(rv model)" = "$SBIN_MODEL" ] || { echo "FAIL: the valid records report models '$(rv model)', pinned $SBIN_MODEL"; exit 1; }
[ "$(rv aks)" = "none" ] || { echo "FAIL: apiKeySource '$(rv aks)' on an S-BIN record (D21-42 17a)"; exit 1; }
[ "$(rv unreach)" = "True" ] && [ "$(rv seed)" = "True" ] || { echo "FAIL: a valid record with the fix reachable or a seed unverified"; exit 1; }
[ "$(rv ceil)" = "$SBIN_CEIL" ] || { echo "FAIL: the records ran under ceilings '$(rv ceil)', pinned $SBIN_CEIL"; exit 1; }
[ "$(rv opus)" -le 18 ] && [ "$(rv cost)" -le 90 ] || { echo "FAIL: a cap was exceeded — $(rv opus) Opus dispatches, \$$(rv cost) (§8i.5)"; exit 1; }

# ---- (5) THE AMENDMENT CAME FIRST, AND METHOD-S §3 HAS NOT MOVED SINCE.
acs=$(git log --reverse --format='%H %ct' -S 's8i pre-dispatch amendment' -- spikes/METHOD-S.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$acs" ] || { echo "FAIL: no commit introduced the s8i pre-dispatch amendment (§8i.2)"; exit 1; }
set -- $acs; ACOMMIT=$1; ATS=$2
[ "$ATS" -lt "$(rv first)" ] || { echo "FAIL: the first S-BIN record ($(rv first)) does not post-date the amendment's commit ($ATS)"; exit 1; }
git show "$ACOMMIT:spikes/METHOD-S.md" > "$T/ms-then.md"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: METHOD-S.md unreadable at $ACOMMIT"; exit 1; }
awk '/^## 3\. S-BIN/{p=1} /^## 4\. S-MIG/{p=0} p' "$T/ms-then.md" > "$T/s3-then"; rc=$?
awk '/^## 3\. S-BIN/{p=1} /^## 4\. S-MIG/{p=0} p' spikes/METHOD-S.md > "$T/s3-now"; rc2=$?
[ "$rc" -eq 0 ] && [ "$rc2" -eq 0 ] && [ -s "$T/s3-now" ] || { echo "FAIL: METHOD-S §3 could not be extracted"; exit 1; }
cmp -s "$T/s3-then" "$T/s3-now" || { echo "FAIL: METHOD-S §3 changed after the pre-dispatch amendment — the registered method moved"; exit 1; }
ledger=$(grep -hoE 'S-BIN go-ahead B[0-4]' spikes/DECISIONS-*.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: no S-BIN go-ahead line in any ledger (§8i.5)"; exit 1; }
for b in B0 B1 B2 B3; do
  printf '%s\n' "$ledger" | grep -qx "S-BIN go-ahead $b" || { echo "FAIL: no go-ahead line for $b"; exit 1; }
done

# ---- (6) THE DRIVER'S TESTS, RE-RUN AT THE PINNED CORPUS COMMIT — rebuilt from `git archive`, so the corpus's own
# ---- working tree (detached, and not this block's to move) is never read or written.
git -C "$CORPUS_REPO" archive --format=tar "$SBIN_CORPUS" runner > "$T/runner.tar"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git archive of runner/ at $SBIN_CORPUS (rc=$rc)"; exit 1; }
tar -xf "$T/runner.tar" -C "$T"; rc=$?
[ "$rc" -eq 0 ] && [ -f "$T/runner/src/sbin.rs" ] || { echo "FAIL: the archived runner has no src/sbin.rs"; exit 1; }
out=$(cd "$T/runner" && cargo build 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the runner does not build at the pinned commit (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
SB_TESTS="sbin::tests::seed_is_confirmed_by_blob_oid_not_by_file_count sbin::tests::review_repo_holds_one_commit_and_no_fix_object sbin::tests::text_diff_carries_source_and_test_paths_only sbin::tests::asset_size_is_read_from_the_lfs_pointer sbin::tests::warned_and_not_run_fold_into_passed_with_the_raw_report_referenced sbin::tests::a_digest_failing_the_typed_schema_is_void_schema_not_a_miss sbin::tests::exit_zero_without_structured_output_is_void_not_success sbin::tests::strong_detection_needs_a_blocking_finding_naming_the_seeded_path sbin::tests::cited_fields_are_extracted_mechanically_from_the_digest sbin::tests::an_out_of_tree_read_voids_the_record sbin::tests::api_key_source_other_than_none_voids_before_scoring sbin::tests::a_version_move_between_dispatches_halts_the_set sbin::tests::the_dispatch_cap_refuses_before_spawning sbin::tests::resume_is_keyed_per_arm_item_pass sbin::tests::a_surviving_unreal_process_halts_before_the_next_dispatch"
out=$(cd "$T/runner" && cargo test --bin redkiln-spike-runner -- $SB_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the driver's tests did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $SB_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 15 passed; 0 failed' || { echo "FAIL: the driver selection did not run exactly 15 tests"; exit 1; }

# ---- (7) THE RECORD: exactly one ADR whose evidence names this finding, accepted, in ADR-0085's form, with its three
# ---- machine-read lines (§8i.8).
adrs=$(grep -lE "^evidence:.*\"$SBIN_F\"" references/adr/0*.md); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$adrs" | grep -c .)" -eq 1 ] || { echo "FAIL: not exactly one ADR cites $SBIN_F as evidence"; exit 1; }
ADR=$adrs; b=$(basename "$ADR"); ADR_ID=${b%%-*}
[ "$((10#$ADR_ID))" -ge 165 ] || { echo "FAIL: $ADR is not a record written from 0165 on"; exit 1; }
grep -qE '^evidence:.*"S-BIN"' "$ADR" || { echo "FAIL: $ADR does not name S-BIN as evidence"; exit 1; }
grep -qx 'status: accepted' "$ADR" || { echo "FAIL: $ADR is not accepted — it flips on the operator's dated ledger line before this block runs"; exit 1; }
[ "$(grep -c '^### Option — ' "$ADR")" -ge 2 ] || { echo "FAIL: $ADR lacks ADR-0085's option headings (D21-42 24a)"; exit 1; }
grep -q 'What this forbids' "$ADR" || { echo "FAIL: $ADR has no What this forbids block"; exit 1; }
grep -qE "\b$ADR_ID\b" references/adr/README.md || { echo "FAIL: the ADR README index has no $ADR_ID row"; exit 1; }
NFIELDS=$(sed -n 's/.*Binary field set (\([0-9][0-9]*\) fields).*/\1/p' "$ADR")
[ -n "$NFIELDS" ] && [ "$(printf '%s\n' "$NFIELDS" | grep -c .)" -eq 1 ] && [ "$NFIELDS" -ge 1 ] || { echo "FAIL: $ADR states no single 'Binary field set (N fields)'"; exit 1; }
ROUTE=$(sed -n 's/.*Binary review route: \(reviewer\|human gate\)\b.*/\1/p' "$ADR")
[ "$ROUTE" = "reviewer" ] || [ "$ROUTE" = "human gate" ] || { echo "FAIL: $ADR states no single Binary review route: '$ROUTE'"; exit 1; }
PRODUCERS=$(sed -n 's/^.*Binary producers in Phase 1: //p' "$ADR")
[ -n "$PRODUCERS" ] && [ "$(printf '%s\n' "$PRODUCERS" | grep -c .)" -eq 1 ] || { echo "FAIL: $ADR states no single 'Binary producers in Phase 1' line"; exit 1; }

# ---- (8) THE VARIANT IS CONSTRUCTIBLE ONLY AS THE RECORD SAYS — by name, then the per-field tests by count.
CS_CRATE=${CS_CRATE:-rk-ports}
def=$(grep -rnE '^[[:space:]]*pub (struct|enum) ChangeSummary\b' "crates/$CS_CRATE/src" --include=*.rs); rc=$?
[ "$rc" -eq 0 ] && [ -n "$def" ] || { echo "FAIL: ChangeSummary is not defined in crates/$CS_CRATE/src — set CS_CRATE to the crate s8a's record homes it in"; exit 1; }
rk_named() {
  pkg=$1; shift
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $# passed; 0 failed" || { echo "FAIL: the $pkg selection did not run exactly $# tests"; exit 1; }
}
rk_named "$CS_CRATE" change_summary::binary_is_constructible_only_through_its_recorded_constructor change_summary::binary_rejects_a_text_patch_for_a_binary_classified_path ui::trybuild_pins_the_binary_change_summary_construction_rule
rk_named rk-store render::ac154_binary_entries_render_from_typed_fields render::ac154_no_binary_content_is_embedded_parsed_or_diffed_as_text
if [ "$ROUTE" = "reviewer" ]; then RT=review_pass::binary_item_reaches_the_reviewer_with_the_typed_summary; else RT=review_pass::binary_item_refuses_an_automated_verdict_and_opens_a_human_gate; fi
rk_named rk-cli review_pass::ac497_binary_classification_assembles_no_diff_and_carries_the_typed_summary "$RT"
rk_named rk-adapter-toolchain-node classify::ac497_classify_binary_paths
rk_named rk-adapter-toolchain-rust classify::ac497_classify_binary_paths
out=$(cargo test -p "$CS_CRATE" --test it -- change_summary::binary_field_ 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the per-field selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
nf=$(printf '%s\n' "$out" | grep -cE '^test change_summary::binary_field_[a-z0-9_]+ \.\.\. ok$')
[ "$nf" -eq "$NFIELDS" ] || { echo "FAIL: $nf per-field tests passed; the record fixes $NFIELDS fields — one test per field"; exit 1; }
printf '%s\n' "$out" | grep -qE "test result: ok\. $NFIELDS passed; 0 failed" || { echo "FAIL: the per-field selection did not run exactly $NFIELDS tests"; exit 1; }
# A FLOOR beneath those tests, over CODE lines only (a guard satisfied by prose reports evidence it does not have):
# a `Binary` construction outside the type's own crate appears only in a file the record names as a Phase-1 producer.
BIN_RE='ChangeSummary::(Binary|binary)\b|BinaryChangeSummary[[:space:]]*(\{|::new\b|\()'
raw=$(grep -rnE "$BIN_RE" crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the construction floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -v "^crates/$CS_CRATE/" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
if [ "$PRODUCERS" = "none" ]; then
  [ -z "$code" ] || { echo "FAIL: a Binary construction on a code line, and the record names no Phase-1 producer:"; printf '%s\n' "$code"; exit 1; }
else
  while IFS=: read -r f _; do
    [ -z "$f" ] && continue
    case ", $PRODUCERS," in *", $f,"*) : ;; *) echo "FAIL: $f constructs Binary and is not a producer the record names"; exit 1;; esac
  done <<< "$code"
fi
syn=$(printf 'crates/x/src/y.rs:1:    let s = ChangeSummary::Binary(b);\n' | grep -E "$BIN_RE" | grep -v "^crates/$CS_CRATE/" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))')
[ -n "$syn" ] || { echo "FAIL: the floor's exclusions swallow a synthetic construction — the floor is vacuous"; exit 1; }

# ---- (9) STATUSES, HONESTLY: the three criteria carry the dated note, and no line states S-BIN pending without one.
for ac in AC-154 AC-356 AC-497; do
  blk=$(awk -v id="**$ac**" 'index($0,id)==1{p=1;print;next} p&&/^\*\*AC-[0-9]+\*\*/{p=0} p' references/2026-09-08-acceptance-criteria.md)
  [ -n "$blk" ] || { echo "FAIL: $ac's block was not found in the register"; exit 1; }
  printf '%s\n' "$blk" | grep -qF "(s8i, S-BIN reported — $SBIN_F)" || { echo "FAIL: $ac carries no dated s8i note naming $SBIN_F"; exit 1; }
done
STALE='unrun|UNRUN|not been run|NOT been run|until S-BIN|before S-BIN|S-BIN reports|BLOCKED'
for doc in references/2026-09-08-architecture-design.md:21 references/2026-09-08-acceptance-criteria.md:17; do
  f=${doc%%:*}; floor=${doc##*:}
  hits=$(grep -nE 'S-BIN' "$f"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $f no longer mentions S-BIN at all — its lines were rewritten, not noted"; exit 1; }
  stale=$(printf '%s\n' "$hits" | grep -E "$STALE" | grep -vF '[S-BIN reported (s8i)')
  [ -z "$stale" ] || { echo "FAIL: $f still states S-BIN as pending without a dated s8i note:"; printf '%s\n' "$stale" | cut -c1-160; exit 1; }
  nnote=$(printf '%s\n' "$hits" | grep -cF '[S-BIN reported (s8i)')
  [ "$nnote" -ge "$floor" ] || { echo "FAIL: $f carries $nnote dated s8i notes; $floor lines stated S-BIN pending at ca8f387"; exit 1; }
done
syn=$(printf '22:| ChangeSummary | BLOCKED | S-BIN registered and unrun |\n' | grep -E "$STALE" | grep -vF '[S-BIN reported (s8i)')
[ -n "$syn" ] || { echo "FAIL: the stale-line guard cannot see a synthetic stale line — it is vacuous"; exit 1; }

# ---- (10) THE WHOLE SUITE, fmt, clippy. 740 passed at ca8f387 (0 failed, 9 ignored, 44 results); later stages only add.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked' | head -20; exit 1; }
res=$(printf '%s\n' "$out" | grep -E '^test result: ')
[ -n "$res" ] || { echo "FAIL: the suite printed no result line"; exit 1; }
bad=$(printf '%s\n' "$res" | grep -vE '^test result: ok\. [0-9]+ passed; 0 failed')
[ -z "$bad" ] || { echo "FAIL: a result line is not ok with 0 failed:"; printf '%s\n' "$bad"; exit 1; }
total=$(printf '%s\n' "$res" | awk '{s+=$4} END{print s+0}')
nres=$(printf '%s\n' "$res" | grep -c .)
[ "$total" -ge 740 ] && [ "$nres" -ge 44 ] || { echo "FAIL: $total passed over $nres results — below the ca8f387 floor of 740 over 44"; exit 1; }
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc (the gate is its exit code — ADR-0102 decision 5)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (11) CHECKPOINTS 8h AND 5, RE-RUN WHOLE from their own stage files' bytes — each scanned FIRST, over code lines,
# ---- for a dispatch shape, and refused rather than run if it carries one (this block may not dispatch, even by proxy).
FENCE=$(printf '\140\140\140')
DISPATCH_RE='claude[[:space:]]+(-p|--print)([[:space:]]|$)|--features[[:space:]]+live|live_dispatch_probe|--ignored|redkiln-spike-runner|review-probe|(redkiln|\$RK"?)[[:space:]]+run([[:space:]]|$)'
# The synthetic control is a `claude -p` line, NOT the live-probe spelling: s8j's re-run of this block refuses any
# block whose text carries that spelling, comments included, so this block must never quote it.
syn=$(printf 'claude -p "review this"\n' | grep -E "$DISPATCH_RE")
[ -n "$syn" ] || { echo "FAIL: the dispatch scan cannot see a synthetic dispatch line — it is vacuous"; exit 1; }
awk -v f="$FENCE" -v s='echo "CP8H-END-OF-BLOCK reached"' '$0==f"bash"{inb=1;buf="";has=0;next} inb&&$0==f{inb=0;if(has){printf "%s",buf;n++};next} inb{buf=buf $0 "\n";if($0==s)has=1} END{exit n==1?0:1}' runbook/phase-1/s8h-cards-and-run-environment.md > "$T/cp8h.sh"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/cp8h.sh" ] || { echo "FAIL: exactly one Checkpoint 8h block ending in its CP8H sentinel was not found in s8h"; exit 1; }
awk -v f="$FENCE" 'index($0,"**Checkpoint 5.** All of:")==1{armed=1;next} armed&&$0==f"bash"{inb=1;next} inb&&$0==f{exit} inb{print}' runbook/phase-1/s5-workspace-ci-rk-types.md > "$T/cp5.sh"; rc=$?
[ "$rc" -eq 0 ] && [ "$(grep -c . "$T/cp5.sh")" -ge 20 ] || { echo "FAIL: the Checkpoint 5 block was not extracted from s5"; exit 1; }
printf '\necho "CP5-END-OF-BLOCK reached"\n' >> "$T/cp5.sh"
for blk in cp8h cp5; do
  hit=$(grep -vE '^[[:space:]]*#' "$T/$blk.sh" | grep -nE "$DISPATCH_RE")
  [ -z "$hit" ] || { echo "FAIL: the extracted $blk block carries a dispatch shape and is NOT run — establish the guard's scope (golden constraint 9):"; printf '%s\n' "$hit"; exit 1; }
  out=$(bash "$T/$blk.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $blk re-run exited $rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  last=$(printf '%s\n' "$out" | grep . | tail -1)
  case "$last" in *END-OF-BLOCK\ reached) : ;; *) echo "FAIL: $blk did not reach its end-of-block sentinel (last line: $last)"; exit 1;; esac
done

# ---- (12) NOTHING DISPATCHED, NOTHING LEAKED: no new session transcript, the worktree path set unchanged, no stale
# ---- registration, and no review tree left registered in BrittonGames or left on disk (Halt 6).
sess_after=$(find "$CFG/projects" -type f -name '*.jsonl' | sort)
new=$(comm -13 <(printf '%s\n' "$sess_before") <(printf '%s\n' "$sess_after"))
[ -z "$new" ] || { echo "FAIL: a new session transcript appeared while this block ran — something dispatched (or you started a Claude session; re-run without one):"; printf '%s\n' "$new"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }
bg=$(git -C D:/repos/BrittonGames worktree list --porcelain); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: BrittonGames' worktree list could not be read (rc=$rc)"; exit 1; }
printf '%s\n' "$bg" | grep -qF "$SBIN_ROOT" && { echo "FAIL: a review tree is registered in BrittonGames' .git — §8i.4 forbids it"; exit 1; }
test ! -e "$SBIN_ROOT" || { echo "FAIL: the review root $SBIN_ROOT still exists — remove the trees (Halt 6: a tree that will not delete is the handle)"; exit 1; }

echo "CP8I-END-OF-BLOCK reached"
```

⚠️ **What this block cannot see, said rather than claimed away.** The tripwire in (0)/(12) sees a dispatch that
**starts a session in the account named by `CLAUDE_CONFIG_DIR`**; a dispatch under another config directory, or one
that writes no transcript, is invisible to it, which is why (11) also refuses an extracted block by its text. The
construction floor in (8) matches `ChangeSummary::Binary`, `ChangeSummary::binary` and a `BinaryChangeSummary`
literal, `::new` or tuple construction (the payload name `s8a`'s file uses); if `s8a`'s record names the
variant, its payload or its constructor otherwise, the regex takes a dated amendment first, and a construction assembled by a
macro is invisible to it — the named tests above it are the assertion, the floor is not. The records probe in (4)
trusts the driver's own fields; the raw transcripts beside them are pinned so that an adjudication can re-derive every
field, and the finding says whether one was run.

### 8i.10 Close

The two-commit close (`00-preamble.md`'s legend: *"(1) the work and its checkpoint outputs, then (2) a tracker commit
writing that commit's own hash into §1"*). The work commit carries the METHOD-S amendment's successor notes if any,
the finding and its index row, the ledger lines, the `Binary` record with its README index cell, census and the
preamble's record count, the Rust and its golden files with their `-text` rows, the register and design notes, the
`s5` Checkpoint 5 amendment if its condition lapsed, `s8a`'s dated amendment for the superseded sealed-slot test,
`s8`'s bullet and guard amended to the record's construction rule, and
Checkpoint 8i's output. The record flips from `proposed` only on the operator's dated ledger line, which precedes the
checkpoint run. The tracker commit writes the hash, marks row 8a `passed <hash>, re-run pending` if its block names the
superseded test, and raises the two owed items this stage found and does not own: AC-475's lift criterion, and the F41/F42 gap in
`spikes/FINDINGS.md`'s index. Adversarial refuting rounds
over this file and its checkpoint continue until a round moves no code and no guard.
