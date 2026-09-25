# Step 8i — S-BIN: review without a diff, and the `Binary` variant finalised (Phase 1, part 8i)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8i` | Phase 1 | Checkpoint 8h | Checkpoint 8i | `s8h` | `s8j` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (re-measured at `a6349b6`). What changed:

- **The operator's four S-BIN decisions are taken** (D21-43 N19–N22), so §8i.2 records them and no longer offers
  options. Captures are taken by hand through `/ue-verify` on all five trees, by one procedure (N19; §8i.2 item 2,
  §8i.4). Backlog and status prose are stripped identically from all five (N20; item 3). The two undecided outcomes
  are registered as undecided and take the human-gate route (N21; item 9, §8i.8). Seed 5 is replaced by the
  registered procedure before dispatch (N22; new item 12). The gate is Checkpoint 8h plus the dry run, confirmed on
  the B1 line (N22; §8i.1). The CLI is frozen from S-BIN on (N22; §8i.5).
- **`/ue-verify` is a Claude Code command, not a script** (§8i.0 item 11, measured). Each capture is an interactive
  session on the account, with the editor open. So the five capture sessions sit in B1, under its go-ahead, and they
  are recorded.
- **Two items the brief assigned here**: AC-475's lift criterion (§8i.8), and `spikes/FINDINGS.md`'s missing F41/F42
  rows (§8i.7). Both are in the work commit, and Checkpoint 8i asserts both.
- **The "four of five" design sentence** gets its dated correction at both of its lines. Guard (9) asserts it.
- **The Checkpoint 8h re-run could not have run as drafted.** Its dispatch scan refused s8h's own `--list --ignored`
  and `clippy --features live` lines. The scan now refuses only shapes that execute, and each pattern has a
  synthetic control. The inputs the re-run blocks read are checked by name first (guard (11)).
- **Re-runs**: 8h whole (the entry gate), plus 8a and 5, which this stage's edits make provisional. Nothing else is
  re-run directly. The nested cost is stated. The block runs with no launch record, so any nested `redkiln run`
  refuses before it spawns (s8h §8h.7). The tripwire watches both account roots.
- **Floor**: re-derived from Checkpoint 8h's recorded count, plus this stage's 6 + N.
- **LFS** is s8a §8a.2 item 2a's rule, cited and not re-specified. **D21-44/45**: no run-worktree path appears in this
  file. W4 was measured against BrittonGames (§8i.0 item 12).

*Revised again 2026-09-24 for D21-46* (s8h read at `c5d19d7`). What changed:

- **S-BIN launches through `s8h`'s launcher.** D21-46's integration call: *"s8i uses s8h's `wrap` mode for S-BIN's
  corpus runner"*. Every runner invocation in B1–B4 is a `tools/live-run.sh wrap -- <argv…>` child, and so is each
  B1 capture session (§8i.5). The operator-shell route is withdrawn. S-BIN's own guards stay, as a second layer: the
  dispatch cap, the CLI freeze, the `apiKeySource` check and the transcript tripwire. `wrap` records the CLI version
  before and after each launch and does not compare it to the adapter pin. §8i.5 quotes s8h for both.
- **One re-run convention, `RK_CP_NESTED=1`** (D21-46). Checkpoint 8i's (11) replays 8h, 8a and 5 with it set. When
  the block is itself run with it set, (11) replays nothing.
- **One no-dispatch guard for replays** (D21-46, in s8g's form). (11)'s replays run with `CLAUDE_CONFIG_DIR` at an
  empty trap directory that must still be empty afterwards, and with `REDKILN_LAUNCH_RECORD` unset.
- **Two checkpoint defects fixed.** (12)'s tripwire listed `$CFG/projects`, and nothing sets `$CFG`, so its "after"
  listing was never the two account roots that (0) read. It now reads `$CFGS` and asserts a non-empty listing. (11)'s
  dispatch scan did not match the launcher's `wrap` mode. It now does, with a synthetic control each way.

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
Items 11 and 12 were added at the 2026-09-24 revision, measured at `a6349b6`.

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
   `bf9c499a1`. Recorded in the amendment, not dropped silently. D21-43 N22 then took the consequence: *"Seed 5 is
   **replaced by the registered procedure** before dispatch"* (§8i.2 item 12).
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
   `Read` in that tree reads it. The registration scoped `text_diff` and `spec.md` and left the tree. Re-measured at
   `a6349b6`: at `a71265925`, `LoadoutFill` occurs in text files only under `_workspace/backlog/`, once in
   `epics/E8_Inventory/E8_Backlog.md` and three times in the backlog JSON [measured, `git grep -c LoadoutFill
   a71265925 -- '*.md' '*.json' '*.cpp' '*.h' '*.py'`]. `CLAUDE.md` does not name it, but it carries a status table
   whose *Progress* row items 1 and 3 move [measured, `git show --numstat`]. D21-43 N20 strips both kinds of prose
   (§8i.2 item 3).
8. **Halt 2 is live.** `claude --version` reads `2.1.281 (Claude Code)`; `RUNBOOK-instrument.txt` reads
   `2.1.278 (Claude Code)` [measured 2026-09-24]. §8i.5 says what this stage does about it. D21-43 N22 freezes the
   CLI from S-BIN on.
9. **The registered decision rule is not total.** METHOD-S §3.4's three branches cover *"2/2 detection and ≤1/3 false
   positive"*, *"A seed missed and its finding would have needed a field the summary lacks"*, and *"Both seeds missed
   with the summary fields cited but not the asset"*. Two outcomes reach none of them: **2/2 detection with ≥ 2/3
   false positives**, and **a seed missed where the reviewer names no missing field and cites no summary field**.
   Golden constraint 10: a bar that cannot decide an outcome is reported as not deciding it, and that is registered
   before the data (§8i.2 item 9). D21-43 N21 registers both as undecided: if either occurs, the `Binary` variant
   takes the human-gate route.
10. **`spikes/FINDINGS.md`'s index stops at F40** although `F41-DRAFT.md` and `F42-DRAFT.md` exist, and the index says
    *"A later F-number is added here the day its draft is written."* **The 2026-09-24 revision brief assigns the
    repair to this stage**, since it writes a FINDINGS row. The work commit adds F41's and F42's rows beside S-BIN's
    (§8i.7), and guard (2) asserts that every draft is indexed.
11. **`/ue-verify` is a Claude Code slash command that drives the running editor through Monolith MCP. It is not a
    script.** BrittonGames' `.claude/commands/ue-verify.md` opens: *"Verify OurGame in the running UE 5.8 editor via
    Monolith MCP — load key assets, run PIE, scan logs, screenshot"*. Its precondition is *"the editor must be open on
    OurGame with the Monolith server up"*, and its capture step is *"`editor_query("capture_scene_preview", …)` for a
    visual"*. Its defaults are animation assets, and *"If `$ARGUMENTS` names specific assets or a map, verify those
    instead of the defaults."* The plugin (`OurGame/Plugins/Monolith/`) and `.mcp.json` are tracked at all five item
    commits, and the command file's last change before each item is `70c4a5214` (2026-07-25) [measured, `git
    ls-tree -r` and `git log -1 <item> -- .claude/commands/ue-verify.md`]. That has three consequences. Each capture
    is an **interactive session on the account**: it spends, and it writes a transcript. It runs with the editor
    **open**, while `verify_headless.ps1` needs it closed. And Monolith listens on one port, so only one editor can
    run at a time. **Unmeasured:** whether `capture_scene_preview` shows a widget blueprint's subject (items 1, 4 and
    5 are UMG widgets). §8i.6 makes a "no" a stop.
12. **D21-45 W4 would not refuse BrittonGames.** W4 refuses a run worktree when *"a repo-local filter driver (for
    example LFS via `--local`) or an `includeIf` would yield pointer files"*. BrittonGames carries its LFS filter at
    **system** scope (`filter.lfs.clean`, `.smudge`, `.process`, `required true`). `git config --local
    --get-regexp '^filter\.'` and `'^includeif'` both return rc 1 [measured]. S-BIN's review trees are fresh
    repositories (§8i.4) and not redkiln run worktrees, so W4 does not govern them. The fact matters for the
    `Binary` record's Phase-2 producer (§8i.8).

What the brief got right, re-measured: the BrittonGames pin `041ead2cb284afd3004d07c2826f31d05b2799fd` is HEAD of
`D:/repos/BrittonGames`, with exactly the two modified paths the entry names (`AnimationGym.umap`,
`OurGame.uproject`); the five item commits and both seed parents resolve (`a71265925^` = `cc0cf2d01…`, `bf9c499a1^`
= `b0cff782d…`); every LFS object both registered seeds need is **present locally** under `.git/lfs/objects` (seed 5
is replaced, §8i.2 item 12, and its replacement's objects are measured in B0); the engine's
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
  FakeHarness end-to-end test of `verbs::run::run`"*. The stage plan puts S-BIN before `s8j`'s lane proofs. **Taken
  by D21-43 N22:** *"S-BIN is gated on Checkpoint 8h plus its zero-spend dry run, confirmed on the B1 line."* S-BIN
  does not dispatch through `redkiln`: no lane, no kernel, no harness adapter. So the proofs that gate it are (i)
  Checkpoint 8h, which stands on every sub-stage proof through `s8h`, including `s8e`'s FakeHarness end-to-end test,
  and (ii) S-BIN's own zero-spend dry run (§8i.3). The B1 go-ahead line names both, by hash (§8i.5), and Checkpoint 8i
  reads it.
- **Operator present, foreground.** D17-5's form: *"In this session, foreground, operator present"*. Every UE
  automation run is the operator's hand (§8i.4).

### 8i.2 The pre-dispatch amendment — what it must settle, and whose decision each item is

One dated entry appended to METHOD-S §7, opening with the literal text **`s8i pre-dispatch amendment`** in its bold
lead (Checkpoint 8i finds its commit by that string), committed to this repository **before B1**. Like the 2026-09-09
entry it changes nothing in METHOD-S §3 — *"the question, the decision rule, the prediction and the sizing stand
exactly as registered"* — and Checkpoint 8i asserts that §3's bytes at HEAD equal §3's bytes at the amendment's
commit, and that every S-BIN record post-dates it. Four items were **(operator)** fixture decisions that the entry
left open or the measurement exposed. The operator took all four on 2026-09-24 (D21-43 N19–N22). The amendment
records each decision and cites its ledger line. It does not re-argue them.

1. **Corrections 4 and 5 of §8i.0**, recorded as measured, with the commands.
2. **The captures producer. Taken, D21-43 N19:** *"**The operator captures by hand** on all five review trees through
   `/ue-verify`, with one procedure applied identically to clean and seeded trees."* The entry said it *"does not
   take"* this decision, and the operator has now taken it. The amendment registers the procedure in writing
   before B1. It states five things:
   - The tree the editor opens is the stripped review tree (item 3).
   - `/ue-verify`'s `$ARGUMENTS` are derived for every item by **one rule**: the item's own changed asset paths, plus
     the map that places them if the rule needs one. They are never chosen per tree.
   - The capture step is `capture_scene_preview`. Its file is written to `_s-bin/captures/` inside the tree, at the
     same relative path on all five trees, and copied to `results/s-bin/captures/item-<n>/`.
   - `monolith_status()` must report the tree's own project before anything is captured. The editor is closed and
     no `UnrealEditor` process survives before the next tree (Halt 6).
   - The session's model and its cost are noted per session in `results/s-bin/captures/procedure-log.txt`, with the
     operator's yes/no on whether the capture shows the item's subject.

   Only the capture file reaches the reviewer. The session's own report does not. A seeded tree's capture shows the
   seed, which is exactly what a real `Binary` summary would carry. §8i.0 item 11 is why the sessions run in B1
   (§8i.5): they are interactive sessions on the account. They run after the greenness runs and before any
   rendering is hashed, so before B2, as the revision brief states N19.
3. **What the review tree holds besides the change. Taken, D21-43 N20:** *"**Backlog and status prose stripped from
   all five trees identically**, and recorded."* The amendment registers **one** set, and strips it from all five
   trees before each tree's one commit. The set is:
   - `_workspace/backlog/`, whole (18 files at every item commit [measured, `git ls-tree -r --name-only <item>
     _workspace/backlog`]);
   - `_workspace/references/UE58_Editor_And_MCP_Gotchas.md`;
   - `CLAUDE.md`'s status table (its *Focus*, *Tier*, *Verify* and *Progress* rows), removed by a byte-level rule
     that the amendment writes down.

   `fixture.json` records the set per tree, and it must be one set across the five
   (`stripped_path_set_is_identical_across_all_five_trees`). The amendment also registers, per seeded item, the
   **leak tokens** that no non-asset file of any stripped tree may contain. For item 4 that is at least `LoadoutFill`
   (§8i.0 item 7). For seed 5's replacement it is the tokens its own diagnosis uses. The fixture asserts zero hits
   over all five trees (`review_tree_carries_no_registered_leak_token`). The reason is the one the METHOD-S §7 entry
   gave for `text_diff`: *"because leaving them in would decide §3.5's prediction by fixture construction rather
   than by the reviewer"*. The tree is the same hazard one directory over.
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
9. **The two outcomes the rule does not decide. Taken, D21-43 N21:** *"**Registered as undecided before dispatch.** If
   either occurs, the `Binary` variant takes the human-gate route."* The amendment names both outcomes (§8i.0
   item 9). `undecided-fp` is 2/2 detection with ≥ 2/3 false positives. `undecided-silent` is a seed missed, with no
   missing field named and no summary field cited. Each is reported as *"not decided by the registered rule"*. The
   finding's pins carry each pass's outcome (§8i.7), and Checkpoint 8i asserts that the record's route follows from
   them (§8i.8).
10. **The caps and the void rule** (§8i.5): one retry per `(pass, item)` at the same ceiling, resume keyed per
    `(arm, item, pass)`.
11. **The CLI version** (§8i.5): recorded on every record. D21-43 N22: *"The CLI is **frozen from S-BIN on**, and the set
    never spans a version."* So a version move halts the set. It is not reported per version.
12. **Seed 5 is replaced. Taken, D21-43 N22:** *"Seed 5 is **replaced by the registered procedure** before dispatch."*
    The registered procedure is the entry's: *"If a seed goes red it is replaced from the remaining 27 asset-only
    commits and this amendment is extended, never quietly edited."* The amendment registers the selection rule
    **before any candidate is examined**. A candidate must meet all of these:
    - It is an asset-only commit from the entry's pool of 29, and not one of the five items' commits.
    - Its own message states the defect it fixes. The seed reverts its own asset or assets to its immediate parent,
      in a tree at that commit, as seeds 4 and 5 did.
    - It shares the registered seeds' selection basis, a perceptual defect. METHOD-S §7's selection threat applies
      unchanged and is restated.
    - Its constructed `spec.md` has a clause that the seeded state violates, and the clause is named before
      dispatch. That rules out the defect the entry found in seed 5: *"Seed 5 violates no registered AC of 8.5.5g"*.
    - It is green at its own commit (§8i.4), and every LFS object it needs is present locally.

    The operator picks among the candidates that meet the rule. The amendment lists every candidate examined, with
    the reason each was passed over, and the command that enumerated the pool. The pool must count 29, or the
    difference is recorded. Item 5 is then that commit, reviewed at that commit. `bf9c499a1` is recorded as
    replaced, and §8i.0 item 5 is the reason.

A seed that goes red at its greenness check, the replacement included, is handled as registered: *"If a seed goes red it is replaced from the
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
| `stripped_path_set_is_identical_across_all_five_trees` | §8i.2 item 3 (D21-43 N20) | strip each tree by its own item's diff |
| `review_tree_carries_no_registered_leak_token` | §8i.2 item 3 | scan `_workspace/backlog/` only |
| `every_item_carries_a_capture_reference_or_rendering_refuses` | §8i.2 item 2 (D21-43 N19) | render captures for the seeded items only |

**The audit.** `results/audit-s.py` gains a `bin` subcommand: per pass and per item, detection on each seed (strong,
under strict), approval on each clean item, the fields cited, whether the reviewer ran the automation itself, and
wall clock — **no figure pools the two passes** (the 2026-09-11 entry: *"no figure pools them, and a 2/2 in one pass
beside a 1/2 in the other is reported as exactly that"*). It is checked against a hand-built fixture of three records
before it reads a real one.

**The zero-spend dry run — S-BIN's fixture proof.** The driver runs end to end over item 1's real review tree with the
runner's global `--claude-bin` pointed at a stub that replays a recorded stream; the record it writes carries
`"dry_run": true`, goes to `results/s-bin/dry-run.jsonl` and **never** to `results/s-bin.jsonl`, and the audit reads
it. Golden constraint 4 is the reason: *"A rig that compiles and passes its unit tests has NOT been run."* The captures
do not exist yet, because they are taken in B1. So the dry run renders item 1 with a synthetic capture reference,
and its record marks that it did. The test run's full output is kept as `results/s-bin/driver-tests.txt`. Its
sha256 is named on the B1 line (§8i.5).

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
# item 5: REPLACED before dispatch (D21-43 N22; §8i.2 item 12). In a tree at the replacement commit R:
#   git checkout R^ -- <each asset R changed>
```

— strip §8i.2 item 3's registered set identically from all five and scan each for the registered leak tokens. Then
commit each tree as the **only** commit of a fresh repository (§8i.2 item 4). Confirm each seed by blob oid (`git rev-parse <seed>^:<path>` against the fresh
repository's `git rev-parse HEAD:<path>`), and confirm the fix unreachable (the fixed pointer blob absent from the
object store, its LFS object absent from `.git/lfs/objects`, `git rev-list --all --count` = 1). Write all of it to
`results/s-bin/fixture.json`, keyed `items.<n>`. Each item carries at least `stripped` (the path set removed),
`leak_hits` (the registered-token scan's count) and `captures` (tree-relative paths and SHA-256). `captures` is filled
after B1 and before any rendering is hashed. Checkpoint 8i reads these three keys.

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
5. The captures are **not** taken here. Each is an interactive session on the account (§8i.0 item 11), so they run in
   B1, under its go-ahead (below).
6. The pin's own 2026-09-05 report is copied as registered (*"Before dispatch it is copied into the spike's own
   results directory with its SHA-256 recorded"*) to `results/s-bin/pin-report-2026-09-05/index.json`; its SHA-256
   must be the `c3dae802…6bc6` measured at authoring (§8i.0), or it was overwritten after 2026-09-05 and the copy is
   not the registered report.

**The captures, in B1: the operator's hand, the editor open, one tree at a time** (D21-43 N19; §8i.2 item 2's
registered procedure). For items 1 to 5 in order:

1. Open the editor on the tree's `OurGame/OurGame.uproject`.
2. From the §8i.5 plain terminal, after its second-layer pre-check, start the Claude Code session in the tree as a
   `wrap` child (§8i.5). The launcher sets the account's `CLAUDE_CONFIG_DIR` and the freeze, and records the version
   before and after the session.
3. Run `/ue-verify` with the registered `$ARGUMENTS`. Confirm that `monolith_status()` reports this tree's project.
4. Confirm that the capture landed at the registered path. Copy it to `results/s-bin/captures/item-<n>/` and hash it.
5. Write the procedure-log line: the tree, the session's model and cost, the capture's sha256, and the operator's
   yes/no on whether it shows the item's subject. That is a judgement on framing, never on the defect.
6. Close the session and the editor. No `UnrealEditor` process survives (Halt 6).

A "no" on a seeded tree is a stop (§8i.6). It never falls back to empty captures.

### 8i.5 Spend — the batches, the caps, the environment, the version

| batch | what | model spend | gate |
| --- | --- | --- | --- |
| **B0** | driver and audit built test-first (§8i.3); dry run; seed 5's replacement selected under the registered rule (§8i.2 item 12); five stripped review trees, seeds by blob oid, fix unreachable (§8i.4); the operator's five UE runs; the amendment committed | none | operator go-ahead for the corpus writes |
| **B1** | at most **3 haiku** flag probes on a throwaway tree: the card adopted with `Read`+`Bash` under `--json-schema` (F20's measured hazard: *"an agent card's `tools` allowlist silently disables `--json-schema`"*), `apiKeySource` `none`, and the init tool set equal to the card's. **Then the five `/ue-verify` capture sessions** (§8i.4, D21-43 N19), each a `wrap` child. None of these is an S-BIN record. | ≤ 3 haiku + 5 operator sessions | **go-ahead B1**, whose line names the fixture proofs it rests on (D21-43 N22) |
| **B2** | pass 1: items 1–5 | 5 Opus | **go-ahead B2** |
| **B3** | pass 2: items 1–5, after pass 1's audit has been read | 5 Opus | **go-ahead B3** |
| retries | a void retried once at the same ceiling, inside its batch; each named in the ledger | ≤ 4 Opus in all | inside B2/B3's go-ahead |
| **B4** | only if a pass fires METHOD-S §3.4 branch 2 (*"add the field, re-run that item"*): a further dated amendment adds the field to the rendering, then that item is re-run in each pass that fired | ≤ 4 Opus | **go-ahead B4**, after its amendment |

**The caps, enforced by the driver before it spawns** (`the_dispatch_cap_refuses_before_spawning`): **18 Opus
dispatches** and **3 haiku**, counting voids, and **$90 API-equivalent** summed over every S-BIN record's
`total_cost_usd`, probes included. The capture sessions are not the driver's to count, because it does not spawn
them. The operator adds their noted cost to the running total, and the B2 line's *caps used so far* includes it.
The warrant: METHOD-S §3.6 sizes S-BIN at *"**10 dispatches**, ~$30 plus UE
automation wall clock"*; S-TYPED's median was *"$3.03/review"* (METHOD-S §7, 2026-09-11 item 3), and a reviewer that
builds and runs a UE suite is slower, so 18 × ~$5 bounds it. Reaching either cap halts the stage; raising one is a
ledger line, never an edit here. Throttle margins are the runner's (METHOD-S §0, *"throttle margins honoured"*).

**The go-ahead line.** Before each batch the operator writes, in the ledger entry this stage opens (next free D21 id at
writing), one dated line of the fixed form `S-BIN go-ahead B<n> <UTC time> — <scope>, <caps used so far>`. The B1
line carries one more clause, `; fixture proofs: Checkpoint 8h <hash>, dry run <sha256 of results/s-bin/dry-run.jsonl>`,
which is D21-43 N22's *"confirmed on the B1 line"*. Checkpoint 8i reads all of them. D21-40 decision 2 binds every batch: *"If a real run is refused unexpectedly, the next step
is to stop and report, not to spend again."*

**The environment.** D21-42 17a: *"A committed launcher script is the only entry to a real run. It refuses
`CLAUDECODE`/`CLAUDE_CODE_*` and denylisted names, sets `CLAUDE_CONFIG_DIR` explicitly, freezes auto-update and
captures the version. The adapter refuses unless init reports `apiKeySource == "none"`."* **Every S-BIN dispatch
batch launches through `s8h`'s launcher, in its `wrap` mode.** D21-46: *"s8i uses s8h's `wrap` mode for S-BIN's
corpus runner"*. s8h §8h.7 names this stage as one of `wrap`'s two callers: *"`s8i`'s S-BIN batches through the
corpus runner."* The draft of this section said *"S-BIN launches through `s8h`'s launcher if it exposes an entry for
an arbitrary child"*, and routed through the operator's shell because the launcher then had no such entry and
refused at step 6 on the pin. s8h at `c5d19d7` answers both, so that route is withdrawn.

**What `wrap` does with the version, in s8h's words** (§8h.7, read at `c5d19d7`). Its step 6 row: *"`wrap`: record
only, and never compare to the pin (below)"*. And below it: *"Steps 1–5, 8 and 9 apply unchanged. Step 6 records and
does not compare, because `TESTED_CLI_VERSION` binds the adapter, and neither caller is the adapter."* Step 8's launch
record holds the claude path, *"version and sha256 before"*, and *"then version and sha256 after, and the rc"*. Step
9: *"a difference fails the launch non-zero"*, naming Halt 2. So `wrap` **records the CLI version before and after
each launch, and does not compare it to the adapter pin.** The installed 2.1.281 against the pin's 2.1.278 (§8i.0
item 8) does not refuse a `wrap` launch, and S-BIN relies on no comparison to `TESTED_CLI_VERSION`. Checkpoint 8h (10)
asserts that `--check wrap` accepts a CLI unequal to the pin. s8h also says *"`wrap` is not a way around the pin for
`run` or `probe`"*. If a later revision of s8h makes `wrap` compare to the pin, a B1–B4 launch refuses at step 6. The
stage then stops for a ledger line (§8i.6), and it does not fall back to a shell route unrecorded.

**What launches through `wrap`.** Every runner invocation that can spawn `claude` does: B1's haiku probes, B2, B3,
the retries inside them, and B4. B1's five capture sessions do too (§8i.4). Each invocation is one launch with one
launch record. B0 spends nothing and launches nothing. Its dry run spawns only the stub (§8i.3), which is the one
place `--claude-bin` is passed. Every wrapped invocation resolves `claude` from the PATH that the launcher resolved
and recorded. The launcher runs **from a plain terminal, never a Claude Code session** (s8h §8h.7: its step 2
*"refuses"* a session's variables and *"does not strip them"*), and from `D:/repos/redkiln-rs`, so the launch record
lands under this repository's git common dir. The child changes into the directory it needs itself. The account is
D21-42 17b's, *"`~/.claude-personal`"*, which the launcher sets at its step 4.

**S-BIN's own guards stay, as a second layer.** None of them is delegated to the launcher, and the launcher does
none of them:
- **The dispatch cap**, enforced by the driver before it spawns (`the_dispatch_cap_refuses_before_spawning`). The
  launcher counts nothing.
- **The CLI freeze.** The launcher exports `DISABLE_AUTOUPDATER=1` to its child (step 5), and it does not read the
  account's `settings.json`. The pre-check below reads that row. `wrap`'s step 9 sees a move inside one launch only.
  A move between two launches is caught by the driver's set-level
  `a_version_move_between_dispatches_halts_the_set`, which compares every record to the set's first version. Every
  record's `cli_version` must also equal the `CLI_VERSION` of its launch record.
- **The `apiKeySource` check**, the driver's own (`api_key_source_other_than_none_voids_before_scoring`). The
  launcher reads no init line.
- **The transcript tripwire**, Checkpoint 8i (0) and (12), over both account roots.

```bash
# Before each of B1..B4, in a Git Bash OUTSIDE Claude Code (s7c §7c.15; s8h §8h.7). The launcher is the route, and
# these lines are S-BIN's second layer. Refusals, never strips.
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
L=tools/live-run.sh
test -f "$L" || { echo "FAIL: $L is absent — Checkpoint 8h builds it, and S-BIN has no other route (D21-46)"; exit 1; }
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: a Claude-spawned shell — the launcher refuses it at step 2 (s8h §8h.7)"; exit 1; }
[ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is already set — this shell is inside a launch; open a fresh terminal"; exit 1; }
for v in ANTHROPIC_API_KEY ANTHROPIC_AUTH_TOKEN ANTHROPIC_BASE_URL ANTHROPIC_FOUNDRY_API_KEY GITHUB_TOKEN GH_TOKEN GH_ENTERPRISE_TOKEN; do
  [ -z "${!v:-}" ] || { echo "FAIL: $v is set — a denylisted name (s7c §7c.6); unset it outside the process, never strip it"; exit 1; }
done
acct="$HOME/.claude-personal"
test -d "$acct" || { echo "FAIL: no $acct — D21-42 17b's account is not on this box"; exit 1; }
# D21-43 N22: the CLI is frozen from S-BIN on. s8h §8h.8's mechanism is the child's environment AND the account's
# settings. The launcher does the first; this reads the second.
grep -q '"DISABLE_AUTOUPDATER"' "$acct/settings.json" || { echo "FAIL: $acct/settings.json has no DISABLE_AUTOUPDATER env row — the freeze (s8h §8h.8) is not in place before B1"; exit 1; }
# The launcher's own refusals, for this mode: --check spawns nothing and writes no launch record (s8h §8h.7).
out=$(bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the launcher refused wrap in this shell (rc=$rc) — fix what it names; S-BIN never routes around it"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "LAUNCH_CHECK=ok" "MODE=wrap" "DISABLE_AUTOUPDATER=1"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check wrap did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
ver=$(printf '%s\n' "$out" | sed -n 's/^CLI_VERSION=//p')
[ -n "$ver" ] || { echo "FAIL: --check wrap recorded no CLI_VERSION"; printf '%s\n' "$out"; exit 1; }
echo "S-BIN instrument at this batch: CLI_VERSION=$ver — wrap records it and does not compare it to the adapter pin; a move since the set's first record HALTS (Halt 2)"
```

```bash
# Then each launch of the batch, from the same terminal and directory. The argv after `--` runs exactly as given
# (s8h §8h.7 step 7), and the launcher's rc is the child's. IN_DIR changes into the directory the child needs, so the
# launch record stays under this repository. SBIN_ARGV is the driver's subcommand and arguments, as the amendment
# registers them (§8i.2 item 5). No --claude-bin is passed.
IN_DIR='cd "$1" && shift && exec "$@"'
[ "${#SBIN_ARGV[@]}" -gt 0 ] || { echo "FAIL: SBIN_ARGV is empty — name the registered subcommand and its arguments"; exit 1; }
out=$(cd D:/repos/redkiln-corpus/runner && cargo build 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the runner does not build (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
bash "$L" wrap -- bash -c "$IN_DIR" in-dir D:/repos/redkiln-corpus runner/target/debug/redkiln-spike-runner "${SBIN_ARGV[@]}"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the wrapped runner exited $rc — stop and report, never spend again (D21-40 decision 2); a version move is Halt 2 (s8h §8h.7 step 9)"; exit 1; }
# B1's capture sessions (§8i.4): one interactive wrap child per review tree, TREE naming it.
test -d "${TREE:-}" || { echo "FAIL: TREE does not name a review tree"; exit 1; }
bash "$L" wrap -- bash -c "$IN_DIR" in-dir "$TREE" claude; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the wrapped capture session exited $rc — stop for a ledger line (§8i.6)"; exit 1; }
```

**Unmeasured, and a stop rather than a fallback:** s8h does not say whether `wrap` hands the terminal through to an
interactive child, and `--check wrap` cannot show it. If the first capture session cannot be driven under `wrap`,
the stage stops for a ledger line (§8i.6). It is not re-routed through a bare shell.

**The version: frozen from S-BIN on, and the pin still moves only at `s8j`.** D21-43 N22: *"The CLI is **frozen from
S-BIN on**, and the set never spans a version."* The mechanism is s8h §8h.8's: `DISABLE_AUTOUPDATER=1` in the
child's environment, and in the account's `settings.json` `env` block. s8h's text puts the settings row in place
before `s8j`'s procedure. N22 moves it forward to **before B1**, and it stays through `s8`. The operator writes it,
because it is user configuration. The pre-check above refuses without it. The detectors are `wrap`'s before-and-after
version and sha256 within each launch, and the driver's set-level check across launches. Halt 2 fires at this stage's open (§8i.0 item 8): *"Record it; do not silently continue."* It is recorded
in `PROGRESS.md`, in every record, and in the finding. `TESTED_CLI_VERSION` is `rk-adapter-claude`'s pin, and S-BIN
does not dispatch through that adapter. That is why `wrap` does not compare to it. D21-42 16 moves the pin *"**once**, at probe time"*, which is `s8j`.
**Handed on to `s8j`:** with the freeze in place from B1, the version `s8j` pins must be the S-BIN pins block's
`cli_version`. The adapter pin `TESTED_CLI_VERSION` equals S-BIN's pinned `cli_version`. If it is not, the freeze failed, and that is `s8j`'s Halt 2 to report. Inside the set,
`a_version_move_between_dispatches_halts_the_set` enforces the rule.

### 8i.6 Kill and abort conditions

Each of these stops the stage where it stands. The batch in progress is recorded as far as it went, the ledger gets a
dated line, and nothing resumes without a new go-ahead.

- **The runner does not build** (Halt 1), or a driver test is not observed red first.
- **The dry run fails, or any B1 probe shows** `--json-schema` inert, a tool set unequal to the card's, or
  `apiKeySource` other than `none` (Halt 3; F20). No Opus dispatch follows a failed B1.
- **A seed is caught by the suite** and no replacement from the 27 asset-only commits passes greenness, or **no
  candidate meets seed 5's registered replacement rule** (§8i.2 item 12). Either way the spike cannot run as
  registered.
- **A capture session cannot show the item's subject on a seeded tree**, or `monolith_status()` reports another
  tree's project. Then N19's producer cannot run as decided. The stage stops for a ledger line, and it never falls
  back to empty captures silently.
- **A fix is reachable** in any review tree, or a seed's blob oid does not match.
- **The CLI version moves** between the set's first record and any later dispatch (Halt 2).
- **A UE process survives a dispatch** (Halt 6); the tree it held is rebuilt before any further dispatch.
- **The same item voids twice** in one pass — a fixture defect, not noise.
- **A cap is reached** (§8i.5).
- **The account is refused unexpectedly** (D21-40 decision 2).
- **The launcher refuses** at `--check wrap` or at a launch, its step 9 fails a launch on a version move, or a capture
  session cannot be driven as a `wrap` child. S-BIN never routes around the launcher (D21-46).
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
`results/s-bin/audit.txt`; `results/s-bin/captures/item-<n>/` and `results/s-bin/captures/procedure-log.txt`
(D21-43 N19); `runner/src/sbin.rs`; `results/audit-s.py`.

**Every S-BIN record** carries, beside the digest: `spike` = `"s-bin"`, `pass`, `item`, `arm`, `void` (`null` or the
named reason), `rerun` (B4 only), `dry_run`, `cli_version`, `observed_model`, `api_key_source`, `fix_unreachable`,
`seed_blob_verified` (`null` on a clean item), `ceiling_secs`, `started_at` (unix seconds), `wall_secs`,
`total_cost_usd`, `ran_automation`, `cited_fields`, `capture_refs`, `detected` (`null` on a clean item), and
`policy_verdict` (strict).

**In this repository:** the METHOD-S §7 amendment (§8i.2), and B4's if one is needed; the finding draft
`spikes/F<next>-DRAFT.md` (next free F id at writing), headed `## F<n> — S-BIN …`, in F31's form — *Established by*
(dispatches, dates, arm, model, CLI version, corpus commit, cost), each pass's table with its detection, false
positives and cited fields, METHOD-S §3.4 evaluated **per pass**, §3.5's prediction against the outcome, the voids
named, and the limits (§3.6: *"n=5 is a type-design sample, not a rate"*; the entry's perceptual-seed selection;
seed 5's replacement and why). It also carries a section headed `### AC-475's lift share`, with the measurement and
its command (§8i.8), **and a pins block**, which Checkpoint 8i parses, of exactly this form:

```text
<!-- s-bin-pins
corpus_commit=<40-hex corpus commit holding every file below>
cli_version=<x.y.z>
observed_model=<the one model id every valid record's init reported>
ceiling_secs=<n>
review_root=<the absolute directory the review trees were built under>
rule_outcome_pass1=<branch1|branch2-detected|branch3|undecided-fp|undecided-silent>
rule_outcome_pass2=<the same five>
ac475_lift_share=<k>/<n>
file=<corpus-relative path> sha256=<64 hex>
-->
```

with one `file=` line per corpus file listed above, per raw transcript and per capture. Also in this repository:
- **Three rows in `spikes/FINDINGS.md`'s index**, in its own `| F | draft | spike | headline |` form: **F41**
  (`S-PORTS`) and **F42** (`S-GRAMMAR`), missing since their drafts were written (§8i.0 item 10; assigned here by the
  2026-09-24 revision brief), and the S-BIN row. Each headline is condensed from the draft's own `## F<n> —` line.
- The ledger entry with every go-ahead.
- The `Binary` record and the dated notes, AC-475's replacement among them (§8i.8).
- The tracker rows.

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
It also names where a Phase-2 `Binary` producer's run worktree comes from under D21-45 W4, or names that as owed. On
BrittonGames, W4 would not refuse (§8i.0 item 12). What an LFS path's summary carries is already assigned: s8a §8a.2
item 2a makes an LFS pointer path `Binary` in the `Text` variant's manifest, and says *"What an LFS asset's summary
carries (for example its size read from the pointer) is `Binary`'s content, and so `s8i`'s."* §8i.2 item 6's
`assets[].size`, read from the pointer, is that content. This stage does not re-specify the pointer rule.

**How the finding maps to the record** — per pass, never pooled:

| what the passes show | the record |
| --- | --- |
| branch 1 in **both** passes (*"2/2 detection and ≤1/3 false positive"*) | the field set as registered in METHOD-S §3.3 and the entry's mappings, less any field no reviewer cited only if the record argues it; **route: reviewer** |
| branch 2 in a pass | the field the reviewer's own prose names is added (B4); if the re-run detects, **route: reviewer** with it; if not, the pass is read as branch 3 |
| branch 3 in a pass (*"binary review needs a *rendered* representation … recorded as a requirement"*) | **the variant is not dropped.** It keeps the manifest and test fields the reviewers did cite, and **route: human gate** — an item with a `Binary`-classified path cannot receive an automated review verdict; the review stage opens a human gate carrying the typed summary (AC-357's shape) until a producer of the rendered representation exists. The requirement is recorded against ADR-0008's Phase-2 Unreal adapter |
| the passes disagree | the more conservative route, and the disagreement reported as a stability finding |
| an outcome the rule does not decide (`undecided-fp`, `undecided-silent`) | **route: human gate** (D21-43 N21: *"If either occurs, the `Binary` variant takes the human-gate route."*), with the outcome reported as *"not decided by the registered rule"* |

**The route, mechanically.** The pins carry `rule_outcome_pass1` and `rule_outcome_pass2`. Each is one of five
values:
- `branch1`;
- `branch2-detected`, where B4's re-run detected;
- `branch3`, which includes a branch 2 whose re-run missed;
- `undecided-fp`;
- `undecided-silent`.

The route is `reviewer` exactly when both passes are `branch1` or `branch2-detected`. Otherwise it is `human gate`.
Guard (7) asserts that the record's route line agrees.

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
takes a dated amendment. Checkpoint 8i then re-runs that amended block whole (guard (11)), and that discharges the
pending re-run. Whether the Rust adapter's `change_summary::change_summary_refuses_a_range_carrying_a_binary_path`
and `s8a`'s `change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path` stand follows the record's
`Binary producers in Phase 1` line. With `none`, both stand unchanged.

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
- **AC-475 is this stage's.** It had no owner, and the 2026-09-24 revision brief assigns it here. Its note says:
  *"When S-BIN settles the type, this criterion is replaced by a lift criterion with its own measured share; until
  then any criterion asserting a `ChangeSummary` lift rate would be invented."* The share is measured here, at zero
  model cost, before the replacement is written. The corpus is the pinned one (`CORPUS_PIN`'s `CORPUS_SHA`, read
  through the object store).
  - The denominator *n* is the diff-prose sections that `redkiln-migrate` carries as `LiftedWithLoss` with a marker
    naming a diff-prose section. That is the carry `diff_prose_carried_opaque_with_loss_marker` pins on its fixture.
  - The numerator *k* is those sections whose carried content the record's `Text` variant accepts as a patch.

  The finding records the share under `### AC-475's lift share`, with its command, and the pins block records it as
  `ac475_lift_share=<k>/<n>`. The register then replaces the criterion by dated amendment. The replacement is a lift
  criterion stating `lift share <k>/<n>` and asserting at least that share, with the remainder carried opaque with a
  marker. The same note says two things. Its corpus-scale verification is Phase 2's, where E13 placed the
  loss-marker half. And the tripwire `ac475_zero_change_summary_values_in_output` stands until a lift is built.
  **If *n* = 0**, the note says the corpus holds no diff-prose section to lift, and asserts no rate. F40's index
  line is the precedent: *"recall has no denominator"*.
- **AC-049** — not moved: its own note scopes S-BIN to *"the `ChangeSummary` type design only (ADR-0093 decision 4)"*.

Each of the four criteria's blocks (AC-154, AC-356, AC-497, AC-475) gains a note in exactly this form, which
Checkpoint 8i finds:
`*Amended <date> (s8i, S-BIN reported — F<n>):* …`. And **every line** of the design and the register that states
S-BIN's pending state — at `ca8f387`, **21** design lines and **17** register lines carry `S-BIN` beside one of
`unrun`, `UNRUN`, `not been run`, `NOT been run`, `until S-BIN`, `before S-BIN`, `S-BIN reports` or `BLOCKED`
[measured] — gets an inline dated note containing `[S-BIN reported (s8i)` and is never rewritten. That set includes
design §0.1's `ChangeSummary` row, the status line's *"Four spikes gate that label — **S-BIN**, …"* (S-BIN leaves the
design-final gate; the other three stay), the §10 register rows, register §0.3's row 3, and the design sentence §8i.0
item 4 corrects.

**The "four of five" sentence also gets a dated correction of its own, because it is false and not merely pending.**
Both design lines that carry it (*"four of the five carry a source/test `text_diff` scoped per METHOD-S §7
(2026-09-09) and only item 2 is diff-free"*, and §10's *"Four of the five items carry a source/test `text_diff`"*)
gain `[Corrected <date> (s8i): under the registered scoping every item's text_diff is empty — s8i §8i.0 item 4]`,
and each line keeps its words. The note on §10's line also records seed 5's replacement. At `a6349b6`, exactly two
design lines carry `only item 2 is diff-free` [measured, `grep -c`]. Guard (9) asserts that every such line carries
the correction.

**Two guards whose condition lapses here, and whose they are.** Checkpoint 5's `ChangeSummary` line bans the type
outside its home *"until S-BIN reports"* (ADR-0099 clause 5's words, as `s8a`'s record restated them): if `s8a`'s
record kept that condition, it lapses in this stage's record commit and `s5`'s line takes a dated amendment in the
same commit, because the render and review sites above name the type. And `s8`'s standing prohibition, which `s8a`'s
file rewrites to *"no `Binary` arm constructed outside `rk-ports`, no `ChangeSummary` definition outside it"*: the
render and review tests above construct `Binary` outside `rk-ports`, and `s8`'s guard greps all of `crates/`, tests
included. So this stage's record commit amends that bullet and guard by dated note to the record's construction rule
— the stage whose change makes a guard false owns its amendment, as `s8a` did for the same bullet.

### 8i.9 Checkpoint 8i

- **The S-BIN records exist and are pinned.**
  - The finding draft and its pins exist.
  - Every pinned corpus file matches byte-for-byte at the pinned commit on the corpus's `main`.
  - There are ten valid records, one per `(pass, item)`, with one CLI version, one model and `apiKeySource` `none`.
    The fix is unreachable and the seeds are verified on every one, and the caps were not exceeded.
  - The amendment was committed before the first record, and METHOD-S §3 is unchanged since.
  - The captures are pinned for all five items. The stripped set is one set across the five, and no registered
    leak token is left.
  - The B1 line names Checkpoint 8h and the pinned dry run (D21-43 N19, N20, N22).
- **The driver's eighteen tests pass at the pinned corpus commit**, rebuilt from the object store, not from a working
  tree.
- **The `Binary` variant is constructible only as the record says.** The record exists and is accepted. Its named
  tests pass in the counts its field set implies. No `Binary` construction appears on a code line outside the type's
  crate, except where the record names a producer. Its route is the one the pinned per-pass outcomes give (D21-43
  N21).
- **The notes are in place.**
  - AC-154, AC-356, AC-497 and AC-475 carry their dated notes. AC-475's states the measured lift share.
  - Both "four of five" design lines carry their dated correction.
  - No design or register line still states S-BIN as pending without a note.
  - Every finding draft has its index row, F41 and F42 included.
- **The whole suite is green**, over the floor re-derived from Checkpoint 8h's recorded count plus this stage's 6 + N,
  and fmt and clippy are clean. **Checkpoint 8h** (the entry gate) and **Checkpoints 8a and 5** (made provisional by
  this stage's edits) re-run whole and green. No other checkpoint is re-run directly. Each replay runs with
  `RK_CP_NESTED=1` (D21-46), which tells a block that honours it to skip its own nested whole re-runs. When this block
  is itself run with `RK_CP_NESTED=1`, it replays nothing. ⚠️ A replayed block that does not yet honour the variable
  still runs its own chain (8h re-runs 8g, and so on down through 8a and 5, and 8a's run reaches 7f and 7i). That
  costs hours, not correctness, and it is stated here rather than avoided.
- **This block dispatches nothing.** It spawns no `claude` of its own. It refuses an extracted block that carries a
  shape that executes a dispatch. It runs with no launch record, so a nested `redkiln run` refuses before it spawns.
  Its replays run with `CLAUDE_CONFIG_DIR` at an empty trap directory that must still be empty afterwards (D21-46,
  s8g's form). And it fails if a new session transcript appears under either account root while it runs.

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
# Both account roots are watched: D21-42 17b's ~/.claude-personal, and the default ~/.claude a stray child would use.
CFGS=""
for c in "$HOME/.claude-personal" "$HOME/.claude"; do [ -d "$c/projects" ] && CFGS="$CFGS $c/projects"; done
[ -n "$CFGS" ] || { echo "FAIL: neither ~/.claude-personal/projects nor ~/.claude/projects exists — the tripwire would compare two empty listings"; exit 1; }
sess_before=$(find $CFGS -type f -name '*.jsonl' | sort)
test -n "$sess_before" || { echo "FAIL: the account holds no session transcript — the tripwire has no baseline"; exit 1; }
# No launch record in this shell, so any nested `redkiln run` refuses before any spawn (s8h §8h.7). How the nested
# 7f/7h/7i runs point at the fixture child is their re-running stages' (s8a, s8e, s8f), and is not re-specified here.
[ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is set — a nested redkiln run could reach the real harness; unset it"; exit 1; }
# The inputs this block's direct re-runs read (guard (11)), checked by name here rather than failing late inside them.
# The list is s8h's and s8a's at this revision; re-read both blocks' INPUTS at stage open.
for v in REC_A REC_B A_SUPERSEDES TOKENS_APPETITE FIX_ITEM CS_ADR PACK_ADR CODEX_ADR S8A_BASE; do
  [ -n "${!v:-}" ] || { echo "FAIL: $v is unset — Checkpoint 8h or 8a reads it"; exit 1; }
done
# THE FLOOR's base (guard (10)), re-derived at stage open from Checkpoint 8h's recorded count by a dated edit to these
# two values. 780 is s8h's drafted floor (740 at ca8f387 plus its 40), the expected value; 44 is the result-line floor.
FLOOR_8H_PASSED=780
FLOOR_8H_RESULTS=44

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
# Every finding draft is indexed. F41 and F42 were missing at authoring, and their rows are this stage's (§8i.7).
BAR=$(printf '\174')   # the index's column bar, built so that no literal bar sits on this code line
idx_missing() { for d in spikes/F[0-9]*-DRAFT.md; do n=${d#spikes/}; n=${n%-DRAFT.md}; grep -qF "$BAR $n $BAR \`$d\` $BAR" "$1" || printf '%s\n' "$n"; done; }
miss=$(idx_missing spikes/FINDINGS.md)
[ -z "$miss" ] || { echo "FAIL: spikes/FINDINGS.md's index has no row for:"; printf '%s\n' "$miss"; exit 1; }
printf '| F | draft | spike | headline |\n' > "$T/idx"
[ -n "$(idx_missing "$T/idx")" ] || { echo "FAIL: the index check cannot see a synthetic index with no rows — it is vacuous"; exit 1; }
pins=$(sed -n '/^<!-- s-bin-pins$/,/^-->$/p' "$DRAFT")
[ -n "$pins" ] || { echo "FAIL: $DRAFT carries no s-bin-pins block (§8i.7)"; exit 1; }
pin() { printf '%s\n' "$pins" | sed -n "s/^$1=//p"; }
SBIN_CORPUS=$(pin corpus_commit); SBIN_CLI=$(pin cli_version); SBIN_MODEL=$(pin observed_model); SBIN_CEIL=$(pin ceiling_secs); SBIN_ROOT=$(pin review_root)
printf '%s\n' "$SBIN_CORPUS" | grep -qxE '[0-9a-f]{40}' || { echo "FAIL: corpus_commit is not exactly one full sha: '$SBIN_CORPUS'"; exit 1; }
printf '%s\n' "$SBIN_CLI" | grep -qxE '[0-9]+\.[0-9]+\.[0-9]+' || { echo "FAIL: cli_version is not exactly one version: '$SBIN_CLI'"; exit 1; }
[ -n "$SBIN_MODEL" ] && [ -n "$SBIN_ROOT" ] || { echo "FAIL: observed_model or review_root is not pinned"; exit 1; }
[ "$SBIN_CEIL" -ge 3600 ] 2>/dev/null || { echo "FAIL: ceiling_secs '$SBIN_CEIL' is not a number >= 3600 (§8i.2 item 7)"; exit 1; }
SBIN_O1=$(pin rule_outcome_pass1); SBIN_O2=$(pin rule_outcome_pass2); SBIN_SHARE=$(pin ac475_lift_share)
printf '%s\n' "$SBIN_SHARE" | grep -qxE '[0-9]+/[0-9]+' || { echo "FAIL: ac475_lift_share is not exactly one k/n: '$SBIN_SHARE' (§8i.8)"; exit 1; }

# ---- (3) EVERY PINNED FILE, BYTE-FOR-BYTE, FROM THE CORPUS OBJECT STORE — never from its working tree (AC-464's rule).
CORPUS_REPO=$(sed -n 's/^CORPUS_REPO=//p' CORPUS_PIN)
test -d "$CORPUS_REPO/.git" || { echo "FAIL: CORPUS_PIN names no repository ('$CORPUS_REPO')"; exit 1; }
git -C "$CORPUS_REPO" cat-file -e "$SBIN_CORPUS^{commit}" || { echo "FAIL: corpus commit $SBIN_CORPUS does not exist"; exit 1; }
git -C "$CORPUS_REPO" merge-base --is-ancestor "$SBIN_CORPUS" main || { echo "FAIL: $SBIN_CORPUS is not on the corpus's main — a dangling commit is not a record"; exit 1; }
files=$(printf '%s\n' "$pins" | sed -n 's/^file=//p')
for req in results/s-bin.jsonl results/s-bin/fixture.json results/s-bin/driver-tests.txt results/s-bin/dry-run.jsonl results/s-bin/audit.txt results/s-bin/pin-report-2026-09-05/index.json results/s-bin/captures/procedure-log.txt runner/src/sbin.rs results/audit-s.py; do
  printf '%s\n' "$files" | grep -q "^$req sha256=" || { echo "FAIL: the pins block does not pin $req"; exit 1; }
done
for n in 1 2 3 4 5; do
  for req in "results/s-bin/greenness/item-$n/index.json" "results/s-bin/rendered/item-$n.txt"; do
    printf '%s\n' "$files" | grep -q "^$req sha256=" || { echo "FAIL: the pins block does not pin $req"; exit 1; }
  done
  printf '%s\n' "$files" | grep -qE "^results/s-bin/captures/item-$n/[^ ]+ sha256=" || { echo "FAIL: no capture pinned for item $n — D21-43 N19 captures all five trees"; exit 1; }
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
# The fixture says what N19 and N20 decided: one stripped set across all five, no leak token left, captures on all five.
git -C "$CORPUS_REPO" cat-file blob "$SBIN_CORPUS:results/s-bin/fixture.json" > "$T/fixture.json"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/fixture.json" ] || { echo "FAIL: results/s-bin/fixture.json is absent or empty at the pinned commit"; exit 1; }
fx='import json,sys
f=json.load(open(sys.argv[1],encoding="utf-8"))
it={int(k):v for k,v in f["items"].items()}
sets={tuple(sorted(v["stripped"])) for v in it.values()}
print("items", ",".join(str(i) for i in sorted(it)))
print("stripped", "ok" if len(sets)==1 and len(next(iter(sets)))>0 else "BAD")
print("leaks", sum(int(v["leak_hits"]) for v in it.values()))
print("caps", ",".join(str(i) for i in sorted(it) if it[i].get("captures")))'
fv=$(python -c "$fx" "$T/fixture.json"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$fv" ] || { echo "FAIL: the fixture probe did not run (rc=$rc)"; exit 1; }
fvv() { printf '%s\n' "$fv" | sed -n "s/^$1 //p"; }
[ "$(fvv items)" = "1,2,3,4,5" ] || { echo "FAIL: fixture.json does not hold exactly items 1-5: '$(fvv items)'"; exit 1; }
[ "$(fvv stripped)" = "ok" ] || { echo "FAIL: the stripped path set is not one non-empty set across all five trees (D21-43 N20)"; exit 1; }
[ "$(fvv leaks)" = "0" ] || { echo "FAIL: $(fvv leaks) registered leak-token hits remain in the stripped trees (§8i.2 item 3)"; exit 1; }
[ "$(fvv caps)" = "1,2,3,4,5" ] || { echo "FAIL: captures recorded for items '$(fvv caps)', not all five (D21-43 N19)"; exit 1; }

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
# D21-43 N22: the gate (Checkpoint 8h plus the zero-spend dry run) is "confirmed on the B1 line", by hash.
DRY_SHA=$(printf '%s\n' "$files" | sed -n 's/^results\/s-bin\/dry-run\.jsonl sha256=//p')
b1=$(grep -hE 'S-BIN go-ahead B1 ' spikes/DECISIONS-*.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$b1" ] && [ -n "$DRY_SHA" ] || { echo "FAIL: no B1 go-ahead line (rc=$rc), or no pinned dry run"; exit 1; }
printf '%s\n' "$b1" | grep -qE 'fixture proofs: Checkpoint 8h [0-9a-f]{7,40}' || { echo "FAIL: the B1 line does not name Checkpoint 8h's hash (D21-43 N22)"; exit 1; }
printf '%s\n' "$b1" | grep -qF "dry run $DRY_SHA" || { echo "FAIL: the B1 line does not name the pinned dry run's sha256 $DRY_SHA (D21-43 N22)"; exit 1; }

# ---- (6) THE DRIVER'S TESTS, RE-RUN AT THE PINNED CORPUS COMMIT — rebuilt from `git archive`, so the corpus's own
# ---- working tree (detached, and not this block's to move) is never read or written.
git -C "$CORPUS_REPO" archive --format=tar "$SBIN_CORPUS" runner > "$T/runner.tar"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git archive of runner/ at $SBIN_CORPUS (rc=$rc)"; exit 1; }
tar -xf "$T/runner.tar" -C "$T"; rc=$?
[ "$rc" -eq 0 ] && [ -f "$T/runner/src/sbin.rs" ] || { echo "FAIL: the archived runner has no src/sbin.rs"; exit 1; }
out=$(cd "$T/runner" && cargo build 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the runner does not build at the pinned commit (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
SB_TESTS="sbin::tests::seed_is_confirmed_by_blob_oid_not_by_file_count sbin::tests::review_repo_holds_one_commit_and_no_fix_object sbin::tests::text_diff_carries_source_and_test_paths_only sbin::tests::asset_size_is_read_from_the_lfs_pointer sbin::tests::warned_and_not_run_fold_into_passed_with_the_raw_report_referenced sbin::tests::a_digest_failing_the_typed_schema_is_void_schema_not_a_miss sbin::tests::exit_zero_without_structured_output_is_void_not_success sbin::tests::strong_detection_needs_a_blocking_finding_naming_the_seeded_path sbin::tests::cited_fields_are_extracted_mechanically_from_the_digest sbin::tests::an_out_of_tree_read_voids_the_record sbin::tests::api_key_source_other_than_none_voids_before_scoring sbin::tests::a_version_move_between_dispatches_halts_the_set sbin::tests::the_dispatch_cap_refuses_before_spawning sbin::tests::resume_is_keyed_per_arm_item_pass sbin::tests::a_surviving_unreal_process_halts_before_the_next_dispatch sbin::tests::stripped_path_set_is_identical_across_all_five_trees sbin::tests::review_tree_carries_no_registered_leak_token sbin::tests::every_item_carries_a_capture_reference_or_rendering_refuses"
out=$(cd "$T/runner" && cargo test --bin redkiln-spike-runner -- $SB_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the driver's tests did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $SB_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 18 passed; 0 failed' || { echo "FAIL: the driver selection did not run exactly 18 tests"; exit 1; }

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
# The route follows the pinned per-pass outcomes (§8i.8's mapping; D21-43 N21 sends an undecided pass to the human gate).
route_for() { r=reviewer; for o in "$@"; do case "$o" in branch1|branch2-detected) : ;; branch3|undecided-fp|undecided-silent) r="human gate" ;; *) echo "BAD:$o"; return 0 ;; esac; done; echo "$r"; }
[ "$(route_for branch1 undecided-fp)" = "human gate" ] && [ "$(route_for branch1 branch2-detected)" = "reviewer" ] && [ "$(route_for branch1 nonsense)" = "BAD:nonsense" ] || { echo "FAIL: the route mapping fails its synthetic controls"; exit 1; }
want=$(route_for "$SBIN_O1" "$SBIN_O2")
case "$want" in BAD:*) echo "FAIL: a pinned rule outcome is not one of the five registered (§8i.8): $want"; exit 1;; esac
[ "$ROUTE" = "$want" ] || { echo "FAIL: $ADR's route is '$ROUTE'; the pinned outcomes ($SBIN_O1, $SBIN_O2) give '$want' (§8i.8)"; exit 1; }
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

# ---- (9) STATUSES, HONESTLY: the four criteria carry the dated note, AC-475 its measured share, the "four of five"
# ---- lines their correction, and no line states S-BIN pending without a note.
acblk() { awk -v id="**$1**" 'index($0,id)==1{p=1;print;next} p&&/^\*\*AC-[0-9]+\*\*/{p=0} p' references/2026-09-08-acceptance-criteria.md; }
for ac in AC-154 AC-356 AC-497 AC-475; do
  blk=$(acblk "$ac")
  [ -n "$blk" ] || { echo "FAIL: $ac's block was not found in the register"; exit 1; }
  printf '%s\n' "$blk" | grep -qF "(s8i, S-BIN reported — $SBIN_F)" || { echo "FAIL: $ac carries no dated s8i note naming $SBIN_F"; exit 1; }
done
blk=$(acblk AC-475)
printf '%s\n' "$blk" | grep -qF "lift share $SBIN_SHARE" || { echo "FAIL: AC-475's replacement does not state the measured lift share $SBIN_SHARE (§8i.8)"; exit 1; }
grep -qxF "### AC-475's lift share" "$DRAFT" || { echo "FAIL: $DRAFT has no '### AC-475's lift share' section"; exit 1; }
ff=$(grep -nF 'only item 2 is diff-free' references/2026-09-08-architecture-design.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: no design line carries 'only item 2 is diff-free' — the two lines were rewritten, not corrected"; exit 1; }
[ "$(printf '%s\n' "$ff" | grep -c .)" -ge 2 ] || { echo "FAIL: fewer than the two 'four of five' design lines measured at a6349b6"; exit 1; }
COR_RE='\[Corrected [0-9]{4}-[0-9]{2}-[0-9]{2} \(s8i\)'
unc=$(printf '%s\n' "$ff" | grep -vE "$COR_RE")
[ -z "$unc" ] || { echo "FAIL: a 'four of five' design line carries no dated s8i correction (§8i.8):"; printf '%s\n' "$unc" | cut -c1-160; exit 1; }
syn=$(printf '610:| x | only item 2 is diff-free |\n' | grep -vE "$COR_RE")
[ -n "$syn" ] || { echo "FAIL: the correction guard cannot see a synthetic uncorrected line — it is vacuous"; exit 1; }
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

# ---- (10) THE WHOLE SUITE, fmt, clippy. The floor is Checkpoint 8h's recorded count (set in (0)) plus this stage's
# ---- 3 CS + NFIELDS per-field + 2 render + 2 review_pass tests, less s8a's superseded sealed-slot test: 6 + NFIELDS.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked' | head -20; exit 1; }
res=$(printf '%s\n' "$out" | grep -E '^test result: ')
[ -n "$res" ] || { echo "FAIL: the suite printed no result line"; exit 1; }
bad=$(printf '%s\n' "$res" | grep -vE '^test result: ok\. [0-9]+ passed; 0 failed')
[ -z "$bad" ] || { echo "FAIL: a result line is not ok with 0 failed:"; printf '%s\n' "$bad"; exit 1; }
total=$(printf '%s\n' "$res" | awk '{s+=$4} END{print s+0}')
nres=$(printf '%s\n' "$res" | grep -c .)
floor=$((FLOOR_8H_PASSED + 6 + NFIELDS))
[ "$total" -ge "$floor" ] && [ "$nres" -ge "$FLOOR_8H_RESULTS" ] || { echo "FAIL: $total passed over $nres results — below the floor of $floor over $FLOOR_8H_RESULTS (Checkpoint 8h's $FLOOR_8H_PASSED + 6 + $NFIELDS)"; exit 1; }
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc (the gate is its exit code — ADR-0102 decision 5)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (11) CHECKPOINT 8h (the entry gate) AND CHECKPOINTS 8a AND 5 (made provisional by this stage's edits), RE-RUN
# ---- WHOLE from their own stage files' bytes. Nothing else is re-run directly. BOUNDED (D21-46): every replay runs
# ---- with RK_CP_NESTED=1, and when THIS block runs under RK_CP_NESTED=1 it replays nothing. ⚠️ A replayed block that
# ---- does not honour the variable still runs its own chain (8h -> 8g -> ... -> 8a -> 7f, 7i; 8g -> 5): hours. Each
# ---- block is scanned FIRST, over code lines, for a shape that EXECUTES a dispatch, and is refused rather than run if
# ---- it carries one. Listing ignored tests (`--list`) or compiling with the live feature does not execute one; s8h's
# ---- block does both, so both stay admissible. NO DISPATCH (D21-46, s8g's form): every replay runs with
# ---- CLAUDE_CONFIG_DIR at an empty trap directory that must still be empty afterwards, and REDKILN_LAUNCH_RECORD unset.
FENCE=$(printf '\140\140\140')
DISPATCH_RE='claude[[:space:]]+(-p|--print)([[:space:]]|$)|redkiln-spike-runner|review-probe|(redkiln(\.exe)?|\$RK"?|\$\{RK\}"?)[[:space:]]+run([[:space:]]|$)|(live-run\.sh|\$L)"?[[:space:]]+(run|probe|wrap)([[:space:]]|$)'
TESTRUN_RE='cargo[[:space:]]+test[^;|]*(--features[[:space:]]+live|--ignored|--include-ignored)'
# The synthetic controls never spell a live run literally: s8j's re-run of this block refuses a CODE line carrying
# the live-feature spelling, the ignored-test flag after `--`, or a push, so the controls are assembled by printf.
syn=$(printf 'claude -p "review this"\n' | grep -E "$DISPATCH_RE")
[ -n "$syn" ] || { echo "FAIL: the dispatch scan cannot see a synthetic claude -p line — it is vacuous"; exit 1; }
syn=$(printf 'bash "$L" %s live::x\n' probe | grep -E "$DISPATCH_RE")
[ -n "$syn" ] || { echo "FAIL: the dispatch scan cannot see a synthetic launcher probe — it is vacuous"; exit 1; }
syn=$(printf 'bash "$L" %s -- bash -c x\n' wrap | grep -E "$DISPATCH_RE")
[ -n "$syn" ] || { echo "FAIL: the dispatch scan cannot see a synthetic wrap launch — S-BIN's route (D21-46) would pass it"; exit 1; }
syn=$(printf 'out=$("${clean[@]}" bash "$L" --check %s -- true 2>&1)\n' wrap | grep -E "$DISPATCH_RE")
[ -z "$syn" ] || { echo "FAIL: the dispatch scan refuses a --check wrap line — s8h's block spawns nothing there and could never be re-run"; exit 1; }
syn=$(printf 'cargo test -p x --features %s --test it -- %s live::x\n' live --ignored | grep -E "$TESTRUN_RE" | grep -v -- '--list')
[ -n "$syn" ] || { echo "FAIL: the dispatch scan cannot see a synthetic ignored-test run — it is vacuous"; exit 1; }
syn=$(printf 'out=$(cargo test -p x --features %s --test it -- --list %s 2>&1)\n' live --ignored | grep -E "$TESTRUN_RE" | grep -v -- '--list')
[ -z "$syn" ] || { echo "FAIL: the dispatch scan refuses a --list line — s8h's block could never be re-run"; exit 1; }
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8I (11) SKIPPED: this block is itself a replay (RK_CP_NESTED=1); its re-runs ran at Checkpoint 8i's own close"
else
xmark() { awk -v f="$FENCE" -v s="echo \"$2-END-OF-BLOCK reached\"" '$0==f"bash"{inb=1;buf="";has=0;next} inb&&$0==f{inb=0;if(has){printf "%s",buf;n++};next} inb{buf=buf $0 "\n";if($0==s)has=1} END{exit n==1?0:1}' "$1"; }
xmark runbook/phase-1/s8h-cards-and-run-environment.md CP8H > "$T/cp8h.sh"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/cp8h.sh" ] || { echo "FAIL: exactly one Checkpoint 8h block ending in its CP8H sentinel was not found in s8h"; exit 1; }
xmark runbook/phase-1/s8a-records-and-red-checkpoints.md CP8A > "$T/cp8a.sh"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/cp8a.sh" ] || { echo "FAIL: exactly one Checkpoint 8a block ending in its CP8A sentinel was not found in s8a"; exit 1; }
awk -v f="$FENCE" 'index($0,"**Checkpoint 5.** All of:")==1{armed=1;next} armed&&$0==f"bash"{inb=1;next} inb&&$0==f{exit} inb{print}' runbook/phase-1/s5-workspace-ci-rk-types.md > "$T/cp5.sh"; rc=$?
[ "$rc" -eq 0 ] && [ "$(grep -c . "$T/cp5.sh")" -ge 20 ] || { echo "FAIL: the Checkpoint 5 block was not extracted from s5"; exit 1; }
printf '\necho "CP5-END-OF-BLOCK reached"\n' >> "$T/cp5.sh"
# The trap sits under target/, which Windows processes can see (Git Bash's /tmp is invisible to them).
mkdir -p target/cp8i || { echo "FAIL: could not create target/cp8i"; exit 1; }
trap_dir="$PWD/target/cp8i/claude-config-trap"
rm -rf "$trap_dir" && mkdir -p "$trap_dir" || { echo "FAIL: could not create the CLAUDE_CONFIG_DIR trap"; exit 1; }
# Synthetic control: the emptiness reading sees one file, or its silence afterwards proves nothing.
: > "$trap_dir/control" || { echo "FAIL: could not write the trap's control file"; exit 1; }
n=$(find "$trap_dir" -mindepth 1 | grep -c .)
[ "$n" -eq 1 ] || { echo "FAIL: the trap's emptiness reading saw $n entries where one was written"; exit 1; }
rm -f "$trap_dir/control" || { echo "FAIL: could not clear the trap's control file"; exit 1; }
for blk in cp8h cp8a cp5; do
  hit=$(grep -vE '^[[:space:]]*#' "$T/$blk.sh" | grep -nE "$DISPATCH_RE")
  hit2=$(grep -vE '^[[:space:]]*#' "$T/$blk.sh" | grep -nE "$TESTRUN_RE" | grep -v -- '--list')
  [ -z "$hit$hit2" ] || { echo "FAIL: the extracted $blk block carries a shape that executes a dispatch and is NOT run — establish the guard's scope (golden constraint 9):"; printf '%s\n' "$hit" "$hit2"; exit 1; }
  out=$(env -u REDKILN_LAUNCH_RECORD RK_CP_NESTED=1 CLAUDE_CONFIG_DIR="$trap_dir" bash "$T/$blk.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $blk re-run exited $rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  last=$(printf '%s\n' "$out" | grep . | tail -1)
  case "$last" in *END-OF-BLOCK\ reached) : ;; *) echo "FAIL: $blk did not reach its end-of-block sentinel (last line: $last)"; exit 1;; esac
done
left=$(find "$trap_dir" -mindepth 1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trap directory could not be read after the replays (rc=$rc)"; exit 1; }
[ -z "$left" ] || { echo "FAIL: something wrote into the trap CLAUDE_CONFIG_DIR during the replays — a real claude was spawned:"; printf '%s\n' "$left" | head -10; exit 1; }
fi

# ---- (12) NOTHING DISPATCHED, NOTHING LEAKED: no new session transcript, the worktree path set unchanged, no stale
# ---- registration, and no review tree left registered in BrittonGames or left on disk (Halt 6).
sess_after=$(find $CFGS -type f -name '*.jsonl' | sort)
test -n "$sess_after" || { echo "FAIL: the after-listing of $CFGS is empty — the tripwire compared nothing"; exit 1; }
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
**starts a session under `~/.claude-personal` or `~/.claude`**. A dispatch under another config directory, or one
that writes no transcript, is invisible to it. That is why (11) also refuses an extracted block by its text. The
text scan reads only the three blocks it extracts, not the blocks they extract in turn. The nested 7f/7h/7i
`redkiln run`s are covered by the missing launch record, which makes them refuse before any spawn (s8h §8h.7), and
by the tripwire, and a real `claude` that inherits a replay's environment writes into (11)'s trap. A child launched
with `CLAUDE_CONFIG_DIR` set explicitly escapes the trap (s8g states the same limit); the launcher sets it at its step
4, which is why (11) refuses a launcher line by its text. The
construction floor in (8) matches `ChangeSummary::Binary`, `ChangeSummary::binary` and a `BinaryChangeSummary`
literal, `::new` or tuple construction (the payload name `s8a`'s file uses); if `s8a`'s record names the
variant, its payload or its constructor otherwise, the regex takes a dated amendment first, and a construction assembled by a
macro is invisible to it — the named tests above it are the assertion, the floor is not. The records probe in (4)
trusts the driver's own fields; the raw transcripts beside them are pinned so that an adjudication can re-derive every
field, and the finding says whether one was run.

### 8i.10 Close

The two-commit close (`00-preamble.md`'s legend: *"(1) the work and its checkpoint outputs, then (2) a tracker commit
writing that commit's own hash into §1"*). The work commit carries the METHOD-S amendment's successor notes if any,
the finding and the three index rows (F41, F42, S-BIN), the ledger lines, the `Binary` record with its README index cell, census and the
preamble's record count, the Rust and its golden files with their `-text` rows, the register and design notes
(AC-475's replacement and the "four of five" corrections among them), the
`s5` Checkpoint 5 amendment if its condition lapsed, `s8a`'s dated amendment for the superseded sealed-slot test,
`s8`'s bullet and guard amended to the record's construction rule, and
Checkpoint 8i's output. The record flips from `proposed` only on the operator's dated ledger line, which precedes the
checkpoint run. The tracker commit writes the hash. It records row 8a as re-run and passed at this commit, because
Checkpoint 8i re-ran 8a's amended block whole (guard (11)), and it records row 5 the same way. AC-475's lift
criterion and the F41/F42 index gap are no longer raised as owed. The revision brief assigned both here, and both
are in the work commit. Adversarial refuting rounds
over this file and its checkpoint continue until a round moves no code and no guard.
