# Step 8a — Records owed and the three red checkpoints (Phase 1, part 8, sub-stage a)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8a` | Phase 1 | Checkpoint 7i | Checkpoint 8a | `s7i` | `s8b` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (re-measured at `a6349b6`). What changed:

- **ADR-0134 clause 5.** D21-43 N27 confirms this file's reading: the clause is about 7f's old text, and the
  re-scope is execution, not supersession. §8a.5 and finding (3) now say so.
- **Checkpoint 7c.** It is provisional after the `ac489` amendment, and its whole re-run is `s8j`'s (D21-43 N28).
  This stage does not re-run it (§8a.9; finding (5) is settled).
- **`s8i` supersedes three of this stage's pins by name** when it finalises `Binary`: the sealed-slot test,
  Checkpoint 8a's no-`pub`-field guard, and guard (6)'s `sealed_open` floor (§8a.2, §8a.9, guard (6)). *(Amended
  2026-09-25, s8a fix round 8: this read **two**. Fix round 7 added the floor and named its supersession only in
  its own comment, while `s8i`'s recorded constructor needs an impl block the floor fails on.)* *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* **Superseded now, by name, as this bullet said:** S-BIN has reported (F43, proposed *(dated 2026-10-15, `s8i` close, D21-154: F43 and ADR-0199 accepted)*) and ADR-0199 (proposed) opens the slot through one recorded checked constructor. The three pins are replaced by the record's construction rule: the `trybuild` case `ui::trybuild_pins_the_binary_change_summary_construction_rule` and, in guard (6), a construction-rule floor (`s8i` §8i.8).
- **An LFS pointer path is `Binary`** in the `Text` variant's manifest, whatever its extension says. This was checked
  against D21-45 W4 (§8a.2). There are two new tests, so the stage adds nine.
- **No accidental dispatch.** The card guard now runs before **both** whole runs that can reach `redkiln run` (7i and
  7f), and guard (11) asserts that no `dispatch_completed` row was added. The rule for a re-run after `s8h` is stated
  in §8a.10.
- **D21-44.** This stage derives no run-worktree path of its own. Checkpoint 7f's whole run removes the one legacy
  tree, `rk-fix-doctor-test-count` at `73c087a`, which must stay put until `s8j` re-points it. Guard (11) therefore
  restores that tree and asserts its ref is unmoved (finding (8)).
- **Floors.** The whole-suite floor is re-derived from the entry gate's reading and asserts the result-line count too.
  Only the entry gate and the checkpoints this stage's edits touch are re-run.

*Revised again 2026-09-24 for D21-46* (re-measured at `c5d19d7`). What changed:

- **Checkpoint 7e's FAIL message is amended here**, as D21-46's integration calls assign it: *"Checkpoint 7e's
  backtick-executing FAIL message, the same defect as 7h's (10a), is fixed by dated amendment in **s8a**."* The new
  §8a.8 gives the line, its replacement and how it is verified without a whole re-run. Guard (9a) asserts it and
  scans every `runbook/` bash block for a backtick bash would execute. Sections 8a.8 and 8a.9 become 8a.9 and 8a.10.
- **The line is not in Checkpoint 7e's block.** It is in `s7e` §7e.1's work block (finding (10)). So PROGRESS row 7e
  does not move, and guard (9a) asserts that no other bash byte of `s7e` changed.
- **`RK_CP_NESTED=1`** (D21-46's one re-run convention). When it is set, guard (11)'s whole re-runs are skipped, and
  every whole run guard (11) makes is launched with it.
- **The replay no-dispatch trap** (D21-46). Guard (11)'s whole runs get `CLAUDE_CONFIG_DIR` at an empty trap
  directory that must stay empty, and `REDKILN_LAUNCH_RECORD` unset. It is a third guard beside the card condition
  and the `dispatch_completed` count.
- **Counts.** Checkpoint 8a has twelve guards: (1) to (11) plus (9a). There are twelve findings, up from nine. The
  records, errata, tests and whole re-runs are unchanged: three records, two errata, nine new tests, and five
  whole re-runs (7i, 5, 7a, 7b, 7f).

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-40…D21-46 the
authority). What changed:

- **Cross-references.** `s8e`'s 7h (10a) amendment is `s8e` §8e.13 after D21-46 R1's renumbering, and W4 is
  `s8d-ii`'s. The line-number citation of the `s7e` line is dropped: the line is cited by its content only.
- **Finding (8) is settled by `s8d-ii` §8d-ii.7**, and finding (12) by `s8b` guard (0) and `s8i` (11), which now launch
  this block with `RK_CP_NESTED=1` under the trap.
- **The trap** asserts that it starts empty before any whole run, as every `s8` trap now does.
- **The floor** is labelled as the chain states it: re-derived at stage open from Checkpoint 7i's recorded count,
  740 + 9 = 749. *(Label corrected 2026-09-24, D21-47, review item 7: 740 is ADR-0164's evidence, not a count
  Checkpoint 7i records. The number is unchanged. See §8a.10.)*

*Adversarial-review fixes 2026-09-24 (D21-47)* (the review of this file at `4a8c832`, and the cross-file items
D21-47's assignment gives it, applied at `fc80687`). Each finding, and what fixed it:

- **1 (BLOCKER), guards (3) and (5) could not both pass.** (5) now compares only the **body** below the closing
  frontmatter `---`. It reads the staged blob against `S8A_BASE`'s blob with `fm_body.py`. A self-test proves that a
  back-link-only change passes, a one-byte body edit fails, and a file with no frontmatter is refused. Measured
  2026-09-24: 30 compared, 0 differ, against both `HEAD` and `4a8c832`.
- **2 (BLOCKER), the git-spawn floor was blind to `run::run_program`.** Guard (6) now bans a `"git"` or
  `"git.exe"` literal on every code line of the adapter's `src`. Four controls check it, one of them the
  `run_program(…, Path::new("git"), …)` form. It also asserts **positively** that `change_summary`'s own body calls
  `range_diff(` and that the crate names `rk_git::diff`. A mutant whose body names `range_diff` only in a comment
  and a string must fail. The adapter's `src` carries no such literal today (`grep` rc 1).
- **3 (MAJOR), an exec range read contradicts ADR-0077.** §8a.2 now reads the range **through gix**:
  `rk_git::diff::range_diff`, over `Repository::diff_tree_to_tree`. Measured: gix 0.87.1's default `basic` feature
  carries `blob-diff`, and the function exists at the pin. No `GitOp` variant is added, so neither ADR-0077 nor
  ADR-0105 clause 5 moves. Guard (6) asserts that `GitOp` is byte-identical to `S8A_BASE`'s and that `diff.rs`
  names no exec type. If gix cannot do the read, the stage halts.
- **4 (MAJOR), ADR-0008 was never cited.** It is now a fifth supersession in part in §8a.2's table: its Decision
  sentence and first forbids bullet, only as far as the two-arm shape and the `Text` arm. Guard (3)'s loop covers
  0008. Finding (13) is new.
- **5 (MAJOR), guard (7) is time-bound; cross-file item 2.** Guard (7) stays pinned. §8a.5 and the guard's comment
  state that **`s8e`'s Slice-Verdict commit amends it by dated line** (`1 passed`, ARMED absent) and lists 8a as
  provisional, under D21-47's time-bound rule. Guard (5)'s half is fixed by item 1. Guard (3)'s ADR-0099 back-link
  now admits a later stage's append.
- **6, the store and the legacy tree on a failure.** The store is **kept**, and §8a.1 says why. The legacy
  restore is now the idempotent `legacy_restore`, installed as an `EXIT` trap before the whole runs and called
  explicitly after them.
- **7, the floor's label.** Corrected in §8a.10, in guard (10)'s comment and above. 740 is ADR-0164's evidence.
- **8, R8 beside W4.** §8a.2 item 2a now cites D21-46 R8 and D21-47 V1 beside W4.
- **9, `as` aliases; multi-line impls.** `CS_BAN` gains a renaming-import arm, with three firing controls and one
  admitted numeric cast. §8a.3 states the one-line limit. (Removing the arm makes the first new control fail:
  measured.)
- **10, the `nbin`/`npub` floors.** Replaced by `sealed.py`. It reads code only, and it reads both the tuple and
  the brace field-list forms. Four planted controls check it. §8a.3 cites the `trybuild` case as the behavioural pin.
- **11, the BEFORE readings.** §8a.1 now runs them under guard (0)'s wrapper, with the card condition checked first.
- **12, unasserted edits.** Guard (3) now asserts the README's index rows and its census at both sites. Guard (5)
  asserts the erratum as **one** `E-0085-n` paragraph (n ≥ 3) naming ADR-0132, ADR-0161 and "thirty", and it
  asserts E-0094-2 inside ADR-0094's section. Guard (9) asserts the `s7a`/`s7b-ii`/`s7c` notes, and every
  "non-finalised newtype" line in the design and the register (10 at `fc80687`). §8a.2 names those ten as owed.
- **13, the 7e comment.** §8a.8 now says the comment is a new line inserted before the amended line.
- **D21-47's replay rule.** A new **guard (0)** builds the wrapper `rkw`: a fake `claude` that prints the pin first
  on `PATH`, `REDKILN_LAUNCH_RECORD` unset, and `CLAUDE_CONFIG_DIR` at an empty trap under `target/s8a-wrap/`,
  outside `target/s8a-ctl/`, which guard (4) deletes. Controls prove the fake answers, the trap starts empty, and
  `rkw` both unsets a planted launch record and sets the trap. Removing either half of `rkw` fails its control:
  measured. Every cargo run in (1), (7) and (10), and every whole run in (11), goes through `rkw`. The trap is
  re-checked at the block's end on both `RK_CP_NESTED` branches. Guard (11)'s card condition was already inside the
  non-nested branch. Finding (14) states the wrapper's Windows limit.
- **Cross-file item 13, the fixture child's name.** No change was needed. This file already names
  `rk-fixture-child-claude`, the `[[bin]]` in `crates/rk-adapter-claude/Cargo.toml` and D21-47's canonical harness.
- **Counts.** Checkpoint 8a has **thirteen** guards: (0) to (11) plus (9a). The records and errata are unchanged:
  three records and two errata. The ChangeSummary record now supersedes five records in part, not four. **Nine new
  tests**, and the floor is still 749. There are fifteen findings.

*Final consistency pass 2026-09-24* (the `s8` files read against each other after the D21-47 fix round; the brief's
four items). One line per edit:

- *Final consistency pass 2026-09-24:* guard (0) sources the shared `runbook/tools/nodispatch.sh` (`nd_setup`) in
  place of its own `sh` fake and trap; `rkw` is gone.
- *Final consistency pass 2026-09-24:* every cargo run in (1), (7) and (10) and every whole run in (11) goes through
  `nodispatch`, and `nd_check` follows each of them and closes the block.
- *Final consistency pass 2026-09-24:* whole runs launch as `nodispatch env RK_CP_NESTED=1 bash …`; a re-runner's
  wrapper is reused (`ND_ACTIVE=1`), not rebuilt.
- *Final consistency pass 2026-09-24:* §8a.1's BEFORE readings, §8a.10's wrapper bullet and nesting paragraph, and
  finding (14) now describe the compiled fake; (14)'s limit is settled.
- *Final consistency pass 2026-09-24:* guard (2) carries `CS_ADR`, `PACK_ADR`, `CODEX_ADR` and `S8A_BASE` as literals
  (`____` until written by dated edit) and refuses a placeholder; re-runners supply none.
- *Final consistency pass 2026-09-24:* guard (2) sets `FIX_ITEM=rk-fix-doctor-test-count` as a literal (§8a.1's item),
  so no re-runner supplies it and none leaks its own item into 7i's run.
- *Final consistency pass 2026-09-24:* the stage header states where each record's id is written (guard (2)).
- *Final consistency pass 2026-09-24:* floor re-checked: §8a.9's table gives nine new tests, so 740 + 9 = **749**
  stands; no literal changed.

---

**Why this step exists.** `spikes/DECISIONS-2026-09-21.md` D21-40 decision 4 reads *"Checkpoints 5, 7b and 7f are
repaired before Phase 1 exits. They are not carried as debt, even though none of them is on `s8`'s entry chain."*
All three are red at the authoring HEAD `ca8f387` (D21-40's table, re-measured by the lead):

- **Checkpoint 5** fails on its `ChangeSummary` line. Its filter chain returns six sites: the `use` line and the
  `change_summary` signature in each of `crates/rk-adapter-toolchain-node/src/lib.rs`,
  `crates/rk-adapter-toolchain-rust/src/lib.rs` and `crates/rk-cli/tests/it/review_pass.rs`. Two more sites break
  ADR-0099's forbids bullet to the letter but pass the filter, because it excludes `::ChangeSummary`. They are
  `crates/rk-cli/src/verbs/run.rs` (`-> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError>`, on
  `UndetectedToolchain`) and `crates/rk-kernel/tests/it/runtime.rs` (`CountingToolchain`). Its doc comment says
  *"The return type is written `rk_ports::toolchain::ChangeSummary` and the name is NOT imported"*, which is a
  deliberate step around the checkpoint. Eight sites in all, and every one is an import or the signature of the
  port's own method. The cause is a conflict between accepted records. ADR-0094 decision 1 and operator decision
  D17-9 require implementors. ADR-0099's forbids bullet bans the name. ADR-0135 clause 5 describes the ban and
  extends it.
- **Checkpoint 7b** pins `predicates parsed 285/287; failed 2/287`. `python spikes/p9/grammar-probe.py` prints
  `predicates parsed 278/279; failed 1/279` and `on_false forms 96/99`, because Step 7e's `spikes/p9` edits at
  `dddcdbd` dropped eight `when` predicates. ADR-0114 decision 1 said *"The two F42 parse failures and three
  `on_false` defects are repaired at Step 7e before the tag"*, but only `packs/incident.toml` was repaired. The
  other four defects are still in `spikes/p9/catalog-b/`.
- **Checkpoint 7f** fails with *"the three conformance lints are not all ARMED"*. `s7g` discharged two of them,
  removing both `#[ignore]`s in the commit that made them pass (`fbf3c4c`), which is `s7f`'s own convention; `s7h`
  removed none and only narrowed the trailer lint's reason. *(corrected 2026-09-25, s8a fix round 1: this said
  "`s7g` and `s7h` discharged two of them, each removing its `#[ignore]`".)* D21-9
  recorded this in advance: *"nothing downstream requires it green"*.

D21-41 (i) and D21-42 decisions 2, 22, 23 and 24a turn those repairs into records. Two of the records have
consequences beyond their checkpoints. The first finalises `ChangeSummary`'s **`Text` variant** now and seals its
**`Binary`** slot for S-BIN, which runs at `s8i`. So the port gains a real value, and the Rust adapter gains a real
`change_summary` body. The second moves the Codex adapter's phase to Phase 4 (ADR-0007).

🛑 **This stage takes decisions, and names them here in its header** (ADR-0092 rule 2.5). The two preceding stage
files each said they took none, and each was wrong. This stage writes **three records** plus two errata, and it
decides nothing that D21-41 or D21-42 did not already decide. None of the three is pre-numbered. Each one is *"a new
record (next free id at writing; `0165` at the time this file was authored)"*, allocated in writing order. Each one
is committed `proposed` and flips only on the operator's dated ledger line. Checkpoint 8a asserts `accepted`,
because every checkpoint amendment below rests on one of them. *(added 2026-09-24, final consistency pass; D21-47:
"Records are located by the id recorded at write time")* Each record's id is written into Checkpoint 8a guard (2) as a
literal, by a dated edit in the commit that writes the record, and so is `S8A_BASE`, the commit the stage opened on,
at stage open. The block refuses while a `____` placeholder remains, and nobody who re-runs it supplies an id.

🛑 **No real dispatch in this stage.** D21-40 decision 2 gates spend on the fixture proofs. The first stage that
dispatches to a real account is `s8i`. §8a.10 guards the one place this stage's exit runs `redkiln run`.

### 8a.1 The entry gate, and the three reds read before anything moves

Run Checkpoint 7i whole with `FIX_ITEM=rk-fix-doctor-test-count`, and confirm it ends with
`CP7I-END-OF-BLOCK reached`. Then extract Checkpoints 5, 7b and 7f from their files' own bytes and run each one
whole, exactly as Checkpoint 8a's helper does in its step (11). Record each rc and the line it failed on. These are this
stage's **BEFORE** readings (PIA-035), and all three must be rc=1 with the reasons listed above. A reading that is
already green means the tree has moved since `ca8f387`. In that case stop and re-measure; do not carry this file's
premise forward. `s7h` and `s7i` both paid for carrying a premise forward.

*(amended 2026-09-24, D21-47)* **The BEFORE readings run under the same wrapper as guard (11)'s whole runs**, never
bare: guard (0)'s fake `claude` first on `PATH` (it prints the pin), `REDKILN_LAUNCH_RECORD` unset, and
`CLAUDE_CONFIG_DIR` at guard (0)'s empty trap directory, which must still be empty afterwards. *(re-pointed
2026-09-24, final consistency pass)* That wrapper is the shared `runbook/tools/nodispatch.sh`: run each reading as
`nodispatch env RK_CP_NESTED=1 bash <extracted block>` after `nd_setup`, and `nd_check` after each. Guard (11)'s card
condition ~~(no shipped card carries `permission_mode`)~~ *(dated 2026-10-02, s8h build: amended by `s8h` §8h.14 in its work commit — all eight shipped cards carry `permission_mode`, and `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` reports its `... ok` line under the wrapper; the no-dispatch premise is the launch check)* is checked before the first of them. In this session
`CLAUDE_CONFIG_DIR` is the operator's real account, so a bare BEFORE run of 7i or 7f is the accident D21-47's
replay rule exists to prevent.

Back up `.git/redkiln/store.sqlite3` before the first whole run. Checkpoint 7f's block runs `redkiln run`, which
appends to the store. *(amended 2026-09-24, D21-47)* **The backup is for recovery by hand, and the store is kept,
not restored.** The rows 7i's and 7f's runs append are the record of those runs, and guard (11) asserts that none
of them is a `dispatch_completed`. Copying the backup over the live store would erase that record, and any row
written beside it.

**What this stage re-runs whole, and what it does not.** It re-runs its entry gate (7i) and the four checkpoints its
own edits repair or move: 5, 7a, 7b and 7f. It does **not** re-run the rest of the chain. Checkpoint 7c is made
provisional here, but its re-run is `s8j`'s (§8a.9). The line this stage amends in `s7e` is not re-run either,
because its block and Checkpoint 7e's both change the tree. §8a.8 says how it is verified instead.

**The legacy run worktree.** `git worktree list` at `a6349b6` shows
`.git/redkiln/worktrees/rk-fix-doctor-test-count` at `73c087a [rk-run/rk-fix-doctor-test-count]`, and it is clean.
D21-44 keeps it *"where it is"* until `s8j` re-points it. Checkpoint 7f's run removes it twice. The first time is
`recreate_run_worktree`'s `worktree remove --force` + `remove_dir_all` at run start. The second is 7f's own
cleanup line, `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count`, and after that line
nothing re-adds the tree. The ref itself does not move, because the `Story:` mint returns early for an item already
recorded (`// Already minted, by this lane or an earlier one. Writing again is drift, not a record.`). Guard (11)
records both before the whole runs, then restores the registration and asserts the ref. *(amended 2026-09-24,
D21-47)* The restore is also an `EXIT` trap, so a whole run that fails after 7f's cleanup line still leaves the
tree registered.

### 8a.2 `ChangeSummary` splits: a finalised `Text` variant and a sealed `Binary` slot (D21-42 decision 2)

**The record** (a new record, next free id at writing; `0165` at authoring) is D21-41 (i)'s Checkpoint 5 record
with D21-42 decision 2 folded into it, and it uses `## Considered options` with `### Option — <name>` headings
(§8a.7). It fixes the following:

1. **Home and opacity.** `ChangeSummary` is defined only in `rk-ports`, as an enum with two arms. `Text` carries a
   typed diff plus a manifest and is **finalised now**. `Binary` is a slot whose payload type is sealed: it cannot
   be constructed outside `rk-ports`, and no fake or adapter constructs it inside `rk-ports` either. S-BIN fills it
   at `s8i`. The record names the two payload types; this file calls them `TextChangeSummary` and
   `BinaryChangeSummary`, and if the record names them otherwise, Checkpoint 8a's patterns follow it in the same
   commit.
   - No crate outside `rk-ports` defines, aliases, redefines or implements a trait for any `…ChangeSummary` type.
   - An implementor of `Toolchain::change_summary` may import the type and name it in that method's signature.
     That includes production code and test doubles in any crate, `rk-kernel`'s `tests/` among them.
   - A `Toolchain` implementor may construct the `Text` arm.
   - `rk-kernel`'s production code still names the type nowhere. The surviving half of ADR-0135's fourth forbids
     bullet stands for `crates/rk-kernel/src/`.
   - **The sealed slot is temporary by design, and `s8i` ends it.** When S-BIN finalises `Binary`, `s8i`
     supersedes **by name** three pins this stage writes:
     `change_summary::binary_arm_is_not_constructible_outside_rk_ports`, with its `trybuild` case, and Checkpoint
     8a's guard (6) line that asserts the sealed payload declares no `pub` field, and guard (6)'s `sealed_open`
     floor, which asserts `OPEN impls=0 ctors=0 aliases=0` over `crates/rk-ports/src` *(amended 2026-09-25, s8a fix
     round 9: this read `OPEN impls=0 ctors=0`; the floor now also counts an impl or construction through a
     `type` alias or a renaming `use`, and refuses the alias)* and which any recorded constructor
     on the payload trips. *(Amended 2026-09-25, s8a fix round 8: this read **two pins**; fix round 7 added the
     floor.)* `s8i`'s file already says that each is *"superseded by name"* (the floor by its dated note there) and that row 8a becomes `passed <hash>, re-run pending`. The record states
     that the slot opens by a later record once S-BIN reports. ADR-0092 rule 1 then decides whether `s8i`'s
     record supersedes this one in part.
2. **The serialised `Text` value carries the members the shipped catalog already reads:**
   - `sha` — `adversarial-review.toml`'s `clone` step reads `inputs = { sha = "artifact:ChangeSummary.sha" }`;
   - `diff` — `change-summary.toml`'s `lint` step reads `inputs = { diff = "${summarise.outputs.diff}" }`;
   - `manifest` — per path: the path, its `classify` result, and its change kind; *(amended 2026-09-25, s8a fix
     round 15: since fix round 14 (item 2a (ii)) the row's classification is `classify`'s path rules
     (`classify::classify_by_path`) with the bytes decided on the range's blobs (`RangeEntry::binary`), never on the
     working tree, so a row can differ from the `classify` verb's answer for the same path. ADR-0165 decisions 3
     and 6 are amended before acceptance to match.)*
   - `files` — `mutation-testing.toml`'s `mutate` step reads `scope = "artifact:Changeset.files"`: the
     manifest's paths, in manifest order, serialised from `manifest` rather than stored.

   A field set that misses a member the catalog reads is a value that resolves nothing. *(Amended 2026-09-25,
   s8a fix round 10: the list named `sha`, `diff` and `manifest` only. `.redkiln/artifacts.toml` declares
   `[artifact.Changeset]` with `schema = "ChangeSummary"`, and nine catalog activities emit
   `Changeset = "${summary.outputs}"` over a `toolchain:change_summary` step, so `artifact:Changeset.files` is a
   member the catalog reads; the enumeration had counted references spelled `ChangeSummary` only. ADR-0165
   decision 3 is amended to match, and `text_variant_serialises_sha_diff_and_manifest` now asserts the key set
   `["diff", "files", "manifest", "sha"]` and `files`' value. The test keeps its name, so guard (7)'s
   `run_named` line does not move.)*

   **2a. An LFS pointer path is `Binary`, whatever `classify` answers.** The `Text` variant's diff is read from the
   object store. For an LFS-tracked path the object store holds a pointer blob, whatever the checkout's smudge
   config is, so the "diff" is the pointer's `oid`/`size` delta and says nothing about the content. That makes
   `text_diff` undecidable from git, as `s8i`'s author flagged. `classify` cannot see this. It decides by extension
   first (`TEXT_EXTS` holds `"json"`, `"svg"` and `"csv"`), and otherwise by the working-tree bytes (*"a NUL within
   the first 8 KiB is git's own binary heuristic"*), and a pointer file has no NUL. So the rule is written into
   the range read:
   - A path is an **LFS pointer path** when its blob at either end of the range has the git-lfs pointer form: under
     1024 bytes, with the first line `version https://git-lfs.github.com/spec/v1`. `rk-git`'s range read marks the
     path so. The read is decided on the blob and not on `.gitattributes`, because an attribute can be absent at
     one end or unset locally.
   - *(added 2026-09-25, s8a fix round 1)* **A binary blob is decided on the blob too.** `classify`'s byte test reads
     the working tree, and answers `Text` for a file the checkout does not hold. So a NUL-bearing (or non-UTF-8)
     blob deleted in the range, or present only at a head the checkout is not on, answered `Ok(Text)` with the
     path's row marked `Text`: the AC-497 failure. `rk-git`'s range read marks such a path `binary`, by the same
     test that writes git's `Binary files … differ` line, and the adapter refuses it by name as it does a `Binary`
     path (§8a.9's two added tests). *(Amended 2026-09-25, s8a fix round 12: the two tests were not one window.
     `classify` read 8192 bytes where the range read reads git's 8000 (`FIRST_FEW_BYTES`), so a blob whose first
     NUL sat in bytes 8000..8191 was `Binary` with the head checked out and `Text` with the base checked out.
     `rk-git` exports its window as `rk_git::diff::BINARY_SNIFF_BYTES` and `classify` reads it; a third case in
     `change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold` reads the same range at each end.)*
     *(Amended 2026-09-25, s8a fix round 14, two findings.* **(i)** *The rule's "or not UTF-8" half had no test:
     every binary fixture carried a NUL, and a NUL-only mark passed both suites. A Latin-1 `.txt` range (no NUL,
     not UTF-8) is folded into `range_diff_marks_a_binary_blob_at_either_end` (it must read `binary: true`) and
     into the adapter test above (it must refuse naming `notes.txt`; `classify` answers `Text` by extension).*
     **(ii)** *One window did not make the answer the range's: `change_summary` still called `classify`, whose
     byte fallback reads the WORKING TREE, so a path text at both ends was refused as `Binary` with a dirty tree
     or a checkout at a third commit. `change_summary` now takes `classify`'s path rules only
     (`classify::classify_by_path`) and lets `RangeEntry::binary` stand in for the byte read; the `classify`
     verb is unchanged. The adapter test gains a case that reads the range clean, dirty and at a third commit
     and requires one answer. Both cases are folded into existing tests, so guard (7)'s `run_named` counts do
     not move.)*
   - Such a path enters the manifest as **`Binary`**. The range then refuses through the re-dated `change_summary`
     row, as any `Binary` path does, and the refusal names the path. It never yields a `Text` whose `diff` is a
     pointer delta. What an LFS asset's summary carries (for example its size read from the pointer) is `Binary`'s
     content, and so `s8i`'s.
   - **This was checked against D21-45 W4, and W4 does not make the rule redundant.** W4 refuses to *create* a run
     worktree when *"a repo-local filter driver (for example LFS via `--local`) or an `includeIf` would yield
     pointer files"*. *(amended 2026-09-24, D21-47)* W4 as it now stands is W4 **as D21-46 R8 re-scoped it**
     (*"The effective config at every scope"*) **and as D21-47 V1 refined R8**: *"Refuse only when pointer files
     would result"*, with attributes read from the fork commit and a working installed driver admitted silently.
     Under every one of those readings, W4 protects the **working tree's** bytes. The range diff reads **blobs**, which are pointers
     under every filter configuration, so V1's admission of a working driver changes nothing here. The two rules agree: neither treats a pointer as a view of the content. W4
     is `s8d-ii`'s (D21-46 R1 moved it out of `s8e`), and this rule does not re-specify it.
   - This repository tracks nothing through LFS (`.gitattributes` has no `filter=lfs` row, measured at `a6349b6`),
     so the rule is proved on a fixture repository (§8a.9).
3. **Ratification.** The record ratifies PROGRESS §2's D17-1 readings 2 and 3: a string literal, a comment and the
   `Capability::ChangeSummary` vocabulary variant do not name the type. It names **D17-9** as the root of the
   conflict. D17-9 chose adapter implementors while it declared *"ADR-0099 decision 5's forbids, unchanged"*, and
   those two things could not both hold.

**Every supersession, in ADR-0092 rule 1's form.** Each item below moves a `## Decision` sentence or a **What this
forbids** bullet, so each is a supersession in part, never an erratum.

| superseded **in part** | the sentence or bullet that moves (quoted) | what stands |
| --- | --- | --- |
| **ADR-0099** clause 5 and its forbids | *"`ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`** until S-BIN reports; Checkpoint 5's grep for its absence covers `crates/` minus that one site."* and *"Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports."* | Clause 5's crate-name sentences. The back-link is an **append**: `superseded_by: ["0115", "0127", <id>]`. The record must say it overrides a forbids bullet that ADR-0127 affirmed, in 0127's words: *"every bullet of its forbids stand"*. |
| **ADR-0135** decision 4, decision 5, and the third and fourth forbids bullets | *"**It must not be "fixed" into a synthesised value.**"*; *(added 2026-09-25, s8a fix round 1)* decision 4's other sentences, only as far as the Rust adapter over a non-empty range that changes only text: its bold lead *"**`change_summary` maps to the port method and thereby reaches the adapters' refusal — which is the correct behaviour and is not a defect to repair.**"*, *"Both real adapters return `Err(registry::refusal(&self.registry, Capability::ChangeSummary))` from a row dated `2026-09-18` whose probe is `none`, citing S-BIN and ADR-0099 clause 5."*, *"A `tool` step naming `toolchain:change_summary` therefore reaches the port and receives `Unsupported`."* and *"The 11 such steps in the shipped pack are expected to refuse."*; *"Checkpoint 5's grep bans the bare type outside `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths."* and *"**Checkpoint 5's RED is untouched by this record.**"*; *"Forbids synthesising a `change_summary` value … and forbids editing either adapter's `change_summary` row to make the step succeed."*; *"Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel`"*, **only** as far as a test double's impl signature under `tests/`; *(added 2026-09-25, s8a fix round 5)* decision 5's bold lead *"**The kernel names `ChangeSummary` in no type position.**"* and its clause *"without the type ever appearing in a signature or a binding in `rk-kernel`"*, in the same scope as the fourth bullet | The map, decisions 1–3 and 6, and the ban on production `rk-kernel`. Decision 4 stands for the Node adapter, and for the Rust adapter's refusal of an empty range and of a range carrying a `Binary` or LFS pointer path, so the kernel's `change_summary("")` still receives `Unsupported` today. *(Amended 2026-09-25, s8a fix round 6: except its sentence *"Both real adapters return … from a row dated `2026-09-18` whose probe is `none`, citing S-BIN and ADR-0099 clause 5."*, which is superseded in part for the Rust adapter as a whole, because decision 6 re-dates and re-reasons that row and the `Binary`/LFS refusals go through `registry::refusal_naming`; it stands for the Node adapter only.)* Back-link: `["0148", <id>]`. |
| **ADR-0093** decision 4 | *"S-BIN blocks the design of `ChangeSummary` (AC-497, AC-356) and the Toolchain port's `change_summary` method body"*. S-BIN now blocks the `Binary` arm and its body only. *(Added 2026-09-25, s8a fix round 10)* Its bold lead, *"**S-BIN's gate is scoped to the `ChangeSummary` type, exactly as ADR-0008 words it.**"*, **only** as far as the `Text` arm: the gate is scoped to the `Binary` arm's payload and its body. | Decision 4's scope rule, and its forbids bullet unchanged. *(Added 2026-09-25, s8a fix round 10: the scope rule is *"it does not block Phase-1 commits, `rk-types`, `rk-store`, `rk-git`, the kernel or the adapters"*, which stands whole.)* |
| **ADR-0094** decision 1 | *"`change_summary` remaining an opaque non-finalised newtype until S-BIN (ADR-0093 decision 4)"*. ⚠️ **D21-42 decision 2 does not name this sentence**, but it is a `## Decision` sentence that the split moves, so rule 1 makes it a supersession too (finding (1) below). | Every other clause of decision 1. *(Noted 2026-09-25, s8a fix round 5:)* that includes *"`classify` and `change_summary` over `cargo metadata` and the changeset"*, which no sentence of ADR-0165 makes false; the Rust adapter's missing `cargo metadata` read is the implementation deviation owed a dated note against ADR-0094 decision 1 since `s7c` Slice 6, now covering the `Text` arm as well (ADR-0165 decision 9). *(Amended 2026-09-25, s8a, D21-51: the operator settled it. ADR-0165 decision 9 now supersedes that clause in part, **only** as far as `classify` and the `Text` arm: `classify` is a function of the path and the bytes at it and spawns nothing, and the `Text` arm is a function of the range read by `rk-git`. The `Binary` arm's inputs (S-BIN, `s8i`) and what `cargo metadata` is read for (`s8c` record T) are not decided. The `s7c` Slice 6 OWED note, for its `classify`/`change_summary` part, is discharged; RT-26's OWED note (the `--list` count route) is untouched. ADR-0094's rejection of a generic shell adapter now rests on the build/test argv, nextest detection, the `--list` count route and the toolchain-keyed kinds (ADR-0073).)* |
| **ADR-0008** `## Decision` and its first forbids bullet *(added 2026-09-24, D21-47)* | *"`ChangeSummary` is designed against it"* and *"`ChangeSummary` may not be designed and locked purely against a synthetic or paper fixture; it must be designed against the real UE project's actual artefacts."* These move **only** as far as the enum's two-arm shape and the `Text` arm, which this record finalises now, over a two-commit fixture repository, before S-BIN runs. ⚠️ **D21-42 decision 2 does not name ADR-0008 either**, and rule 1 makes it a supersession for the same reason (finding (13)). | The `Binary` arm's payload is designed against the real UE project's artefacts at `s8i` (S-BIN) and stays sealed until then. The UE fixture sentence, the Phase-2 adapter sentence and the second forbids bullet stand. *(amended 2026-09-25, s8a fix round 3: the Decision's "Concretely:" sentence restates the superseded phrase as *"runs against it and feeds the `ChangeSummary` type"* and *"Phase 1 only requires `ChangeSummary` to be "designed against the UE fixture""*. Those two clauses move with it, in part and only as far as the `Text` arm; the rest of that sentence stands. No row had named them.)* Back-link: `superseded_by: [<id>]`. |

**Owed beside the record, by dated note and not by rewrite.**

- **Design.** The paragraph that says *"meanwhile `ChangeSummary` is an **opaque, non-finalised newtype in
  `rk-ports`**, the one site Checkpoint 5's absence grep over `crates/` excludes"*, and §9.1's crate row
  *"`ChangeSummary` (an opaque, non-finalised newtype, the one site Checkpoint 5's absence grep excludes — ADR-0099
  clause 5)"*.
- **Register.** AC-497 (*"an **opaque, non-finalised newtype in `rk-ports`**"*). AC-154 and AC-356 stay PROVISIONAL
  on the `Binary` half only.
- *(added 2026-09-24, D21-47)* **Every other line that says the same thing.** At `fc80687`,
  `grep -cF 'non-finalised newtype'` reads **7** in the design and **3** in the acceptance-criteria register. The
  two design sites and AC-497 above are three of those ten. Each of the ten takes a dated note citing the record, because each
  says a thing the split makes false. Guard (9)'s sweep asserts it, as it does for the Codex phase lines.
  *(amended 2026-09-25, s8a fix round 1: three more lines say the same thing in other words, "an opaque newtype in
  `rk-ports`": the design's BLOCKED `ChangeSummary` field-set row, its binary-change-items row, and the register's
  area row 3. Each takes the same note, and guard (9)'s sweep reads `opaque … newtype` as well, floor 13.)*
- **The README.** The index rows and back-link cells, the census, and the *"next free number"* sentence. The same
  sentence also appears in `00-preamble.md`.

**The code.** Write the tests first (§8a.9). Then make these changes:

- **`crates/rk-ports/src/toolchain.rs`.**
  - The enum replaces `pub struct ChangeSummary(pub(crate) ());`.
  - The doc comment's *"This is the one site in `crates/` the name may appear"* and *"no real adapter implements it
    at this stage"* are corrected, with dates.
  - `FixtureToolchain` returns `Text` for text paths it was scripted with. It refuses by name, never with an empty
    `Text`, for a range that carries a `Binary` path. *(amended 2026-09-25, s8a fix round 3: it also refuses by name
    for an empty or blank range and for a range nobody scripted, as ADR-0165's forbids bullet *"returning an empty
    `Text`, or any `Ok`, for an empty or unresolvable range"* reads with no scope. Both answered an empty `Text`, so
    the kernel's `change_summary("")` was an `Ok` on every walk binding the fake. `toolchain_fake_classifies_binary`
    asserts the refusals, and the conformance fake row scripts the body's `HEAD~1..HEAD`.)*
- **The range's diff is read by `rk-git`, through gix.** The Rust adapter's `Cargo.toml` says *"no git is spawned
  by this crate (ADR-0077 confines git to rk-git's one wrapper…)"*, and that stands. *(amended 2026-09-24, D21-47:
  this bullet said the read would use `GitOp`'s exec reads. ADR-0077's `## Decision` reads **"gix owns the read
  path"**, and ADR-0091 decision 1 confirmed gix as the read owner at the pin, so an exec read would move ADR-0077.)*
  - The range read is **`rk_git::diff::range_diff`**, in a new `crates/rk-git/src/diff.rs`. It resolves both ends of
    the range, takes the changed paths and their change kinds from `Repository::diff_tree_to_tree`, and reads each
    changed blob by object id for the LFS pointer check (item 2a) and the text diff. **Measured 2026-09-24:** the
    workspace pins `gix = "=0.87.1"` with default features. Its `basic` default carries `blob-diff`, and
    `gix-0.87.1/src/repository/diff.rs` declares `pub fn diff_tree_to_tree`. If the implementation names the
    function otherwise, guard (6)'s patterns follow it in the same commit. *(amended 2026-09-25, s8a fix round 7:
    the changed paths now come from gix-diff's tree diff, `gix::diff::tree`, over the two trees' bytes with a
    `Recorder`, not from `Repository::diff_tree_to_tree`. Measured in gix 0.87.1: `diff_tree_to_tree` begins by
    building a blob resource cache through `diff_resource_cache`, which loads the checkout's `.git/index` even with
    rename tracking off, so an index under 20 bytes panicked inside gix-index and a damaged one failed a committed
    range. Guard (6)'s presence check follows the call. A repository that is there but does not open, such as one
    whose config does not parse, is now `RangeError::Read` and not `Open`, so the adapter answers it
    `Transport { Io }`.)*
  - **No `GitOp` variant is added**, and `diff.rs` names neither `GitOp` nor `GitExec`. So ADR-0077 does not move,
    and neither does ADR-0105 clause 5. Under D21-47's clause-5 rule a read is never a supersession of that clause
    anyway. Guard (6) asserts that `GitOp`'s declaration is byte-identical to `S8A_BASE`'s. *(amended 2026-09-25,
    s8a fix round 2: it no longer asserts byte identity, which `s8c`, `s8d-ii` and `s8g`'s added variants would break
    at every later whole re-run of this checkpoint. It asserts that every `S8A_BASE` variant is still declared and that
    no variant added since is named for a diff, a range or an object read.)* *(amended 2026-09-29, `s8f`'s build
    (R1FIX-8f-B-RECORDS), under D21-47's time-bound rule, *"A time-bound guard in an earlier checkpoint is amended by the
    stage that moves its premise, in the same commit"*: `s8f` adds `GitOp::CatFile`, which ADR-0188 names (*"`cat-file
    -s` takes a read variant, `GitOp::CatFile`"*) and which `s8f` §8f.6 item 4 also uses to read the run ref's blob
    bytes. The pin admits exactly that name, and a second line pins every `"cat-file"` argument list on a code line of
    `crates/*/src` to `-s` or `blob`. Planted `RangeDiff` and `CatFileBatch` variants and a planted `-p` read still
    fire.)* *(amended 2026-09-25,
    s8a fix round 1: rk-git's free `git_exec::run` and `run_with_env` take no `GitOp`, so guard (6) also asserts that
    `diff.rs` and `path.rs` name no `git_exec` item and no `crate::` or `super::` item but `crate::error`, and it reads
    `diff_tree_to_tree` on code lines only, strings blanked.)*
  - If the pinned gix cannot produce this read, **halt.** An exec read would need a record that supersedes ADR-0077's
    read-path sentence in part, and that is a decision this stage does not hold. Do not shell out from the adapter.
  - `RustToolchain::change_summary`'s own body calls `range_diff(…)` directly. Guard (6) asserts that call
    positively, because a ban on spawning git alone passes on a body that reads nothing.
- **`RustToolchain::change_summary` returns `Ok(ChangeSummary::Text(…))`** when every path in the range classifies
  `Text` or `Generated`. *(Amended 2026-09-25, s8a fix round 15: "classifies" here and in the `Binary` refusal
  below is `classify`'s path rules with the bytes decided on the range's blobs, as item 2a (ii) made it, not the
  `classify` verb's working-tree answer.)*
  - A range that carries a `Binary` path refuses through the registry's dated `change_summary` row. The row is
    re-dated, and its `reason` is narrowed to the sealed arm.
  - A range that carries an LFS pointer path (item 2a) refuses the same way, naming the path.
  - An empty or unresolvable range refuses by name. It never returns an empty `Text`. ⚠️ **This matters today:**
    `crates/rk-kernel/src/runtime/step.rs` calls `"change_summary" => toolchain.change_summary("").map(|_| ())`.
    The kernel passes an **empty** range and discards the value until `s8d`'s resolver binds `artifact:Changeset.sha`.
    An adapter that answered `Ok` to `""` would turn eleven refusals into eleven vacuous greens.
  - *(added 2026-09-25, s8a fix round 14)* A range read that fails on the store answers `Err(Transport { Io })`,
    naming the object or path: `rk-git`'s `RangeError::Read`. It has three causes: a blob, tree or commit that
    cannot be read (fix round 4), a repository that is there but does not open (fix round 7, the note above), and
    a changed path that is not UTF-8 (fix round 8). ADR-0165 decision 6 is amended before acceptance to list this
    fourth outcome and to add "every changed path is UTF-8" to its `Ok` bullet.
- **The Node adapter's row, body and registry tests are unchanged.** Its `change_summary` still refuses. Finalising
  a Node body is not in D21-42 decision 2 and is not taken here.
- **Stale texts.** Four sites get dated corrections:
  - `crates/rk-kernel/tests/it/runtime.rs`'s path-qualification comment (the exclusion it avoided is gone);
  - the two adapters' `registry.rs` module docs (*"no `Ok` is constructible"*);
  - `crates/rk-cli/tests/fixtures/lint-controls/ac475_change_summary_control.rs`'s prose.
  - *(Added 2026-09-25, `s8a` fix round 5: three more sites.)* `crates/rk-kernel/src/runtime/step.rs`'s doc
    comment and arm comment on `change_summary` (*"Both real adapters return `Unsupported` from a row dated
    `2026-09-18`"* and *"never constructible outside `rk-ports`"*); the doc comment on
    `crates/redkiln-migrate/tests/it/output.rs`'s `ac475_zero_change_summary_values_in_output` (*"no `Ok` is
    constructible (S-BIN unrun)"*); and the register's AC-475 vacuity disclosure. Each gets a dated note; no
    assertion moves.

  AC-475's tripwire over the migrator is unaffected, and asserted.

### 8a.3 Checkpoint 5, repaired by amendment under the new record (D21-41 (i))

`s5`'s `ChangeSummary` statements are replaced by a dated amendment that cites the record. The ban now names what
the record forbids. The `::ChangeSummary` exclusion is **dropped**, so a path-qualified definition, alias or impl
fires like a bare one. The amendment contains:

- **The ban.** `crates/` minus `rk-ports`, code lines only, string literals blanked. It rejects any
  `struct`/`enum`/`type`/`trait`/`union` named `…ChangeSummary`, any `type X = …ChangeSummary` alias, any
  `impl … for …ChangeSummary`, and any inherent `impl …ChangeSummary`. *(amended 2026-09-24, D21-47)* It also
  rejects a renaming import, `…ChangeSummary as X`, whether on a `use` line or on one item line of a multi-line
  `use { … }`. The rule is a capital or `_` after `as`, so a numeric cast such as `Capability::ChangeSummary as
  usize` is admitted. *(amended 2026-09-25, s8a fix round 1: that rule let a lowercase renaming import through, and
  `use …::ChangeSummary as summary;` compiles under `rustc -D warnings`. Two arms are added, whatever the alias's
  case: a renaming of `…ChangeSummary` on a `use` line, and a line made only of import-list items one of which renames
  it. A numeric cast inside an expression is still admitted; a lowercase cast that is the whole of a list-item line
  fires. Checkpoint 5's copy and `s8`'s `S8_DEF` carry the same arms and controls.)* At `fc80687`, `grep -rn 'ChangeSummary[[:space:]]*as' crates/` finds nothing.
  **Stated limit:** the ban reads one line at a time. An `impl` split so that `for …ChangeSummary` starts a new line
  after `impl Trait` is not seen. `rustfmt` keeps `impl Trait for Type {` on one line unless it exceeds the width,
  and guard (10)'s `cargo fmt --check` holds the tree to that. A line-joined reading is the fix if one ever
  appears. *(amended 2026-09-25, s8a fix round 3: `cargo fmt --check` holds nothing past the width, because rustfmt
  itself wraps there. Measured 2026-09-25, rustfmt writes a long alias as `pub type X =` with
  `Result<…ChangeSummary, …>;` on the next line, and a long impl header with `for ChangeSummary` on a line of its own,
  and the ban fires on neither. The limit covers **any** definition rustfmt wraps, alias as well as impl. Checkpoint
  5's comment, guard (6)'s and `s8`'s `S8_DEF` state it in the same words. The line-joined reading stays owed.)*
  *(amended 2026-09-25, s8a fix round 13: the `impl … for` arm took only a name straight after `for`, so
  `impl Render for &ChangeSummary` and `impl Render for Box<…ChangeSummary>` passed, and both compile outside
  `rk-ports` under `rustc -D warnings`. The arm now takes any text between `for` and the name on the line. Two
  controls that fire are added, a reference and a `Box<…>`, in all three copies: guard (6), Checkpoint 5 and
  `s8`'s `S8_DEF`.)* *(amended 2026-09-25, s8a fix round 14: the definition arm and the `impl … for` arm matched
  the name as `[A-Za-z_]*ChangeSummary`, so `pub struct r#ChangeSummary;` (a type literally named `ChangeSummary`),
  `pub struct V2ChangeSummary;` and `impl Clone for Sha1ChangeSummary {` passed, and each compiles under
  `rustc -D warnings`. Both arms now take `[A-Za-z0-9_]*`, and the definition arm an optional `r#`. Three controls
  that fire are added in all three copies: guard (6) and Checkpoint 5 (29-31), `s8`'s `S8_DEF` (23-25).)*
- **The `rk-kernel` production ban.** `crates/rk-kernel/src`, code lines only, strings blanked: no
  `\bChangeSummary\b`.
- **The opacity pin inside `rk-ports`.** The ban excludes `rk-ports`, so this pin is what guards opacity there.
  `rk-ports` declares exactly one `pub enum ChangeSummary`, and the sealed payload type declares no `pub` field. *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* (S-BIN has reported: the no-`pub`-field half is superseded by name by ADR-0199's construction rule, which keeps the payload's fields private, `s8i` §8i.8.) The
  **behavioural** pin is the `trybuild` case in §8a.9, because a grep cannot prove non-constructibility.
  *(amended 2026-09-24, D21-47)* The two textual pins read **code only**, with comments stripped, so a commented-out
  declaration is not counted. They read the payload's field list in **both** forms, tuple `(…)` and brace `{ … }`,
  so a brace struct with a `pub` field is caught. Planted sources check each case. The pins are floors beneath
  the `trybuild` case, never a replacement for it.
- **Synthetic controls, through the same chain, one line at a time.** A combined control would pass on a single
  hit. These must fire: a definition, a `Text…` redefinition, a path-qualified alias, a trait impl, a path-qualified
  inherent impl, a generic impl on the sealed type, an impl on a line that also carries a string, and *(added
  2026-09-24)* a `use … as` rename, a braced-item rename and an `as _` import. These must be
  admitted: the `use` line, both signature spellings, `Self::ChangeSummary =>`, a bare `ChangeSummary,` variant
  line, a `Text` construction, a comment, a string, and *(added 2026-09-24)* a numeric cast of the vocabulary
  variant.

The positive control `pub struct ChangeSummary(` in `s5` no longer matches, and the same line appears in two other
places. Each one moves in the same commit, by dated amendment citing the record:

- **Checkpoint 7a's line**, *"expected exactly 1 — ADR-0099 clause 5 (:90-91) decides the type is an opaque,
  non-finalised newtype"*. Checkpoint 7a is re-run whole at this exit, because this stage moves its subject.
- **`s8`'s bullet and guard**, *"No finalised `ChangeSummary` type; no synthesised `ChangeSummary` value"*. The
  bullet becomes *"no `Binary` arm constructed outside `rk-ports`, no `ChangeSummary` definition outside it"*. The
  guard's comment *"RED with four hits in the two adapters"* is corrected: eight literal sites, six through the old
  filter. ⚠️ **The stage plan assigns this edit to no stage** (finding (2)). If it does not land here, Checkpoint 8's
  mechanical block, green at HEAD, goes red at this stage's commit.

### 8a.4 `catalog-b`'s four defects, and Checkpoint 7b re-pinned (D21-41 (i); D21-42 decision 23)

**The repair.** Patch terminator-agnostically and assert a hit count of exactly 1 per anchor before writing
(golden constraint 7; the memory rule on mixed line endings). Run the patch from Python, never with `sed -i` over
a smudged file.

| file | from | to | N's warrant |
| --- | --- | --- | --- |
| `spikes/p9/catalog-b/blameless-postmortem.toml` | `kind in [incident, security-vuln]` | `kind in [incident, 'security-vuln']` | ADR-0048's quoting rule; F42's own suggested repair |
| same, step `complete` | `on_false = "draft"` + `max     = 2` | `on_false = { to = "draft", max = 2 }` | the step's own sibling `max = 2`, which moves into the table. 62 of 66 loop-backs in `spikes/p9` use `max = 2` |
| same, step `reviewed` | `on_false = "draft"` + `max     = 1` | `on_false = { to = "draft", max = 1 }` | the step's own sibling `max = 1`; `max = 1` is the sibling convention's second form (3 of 66) |
| `spikes/p9/catalog-b/api-contract-spec.toml`, step `policy` | `on_false = "assemble"` + `max     = 2` | `on_false = { to = "assemble", max = 2 }` | the step's own sibling `max = 2` |

In each row the stray step-level `max` line is removed, because its value now lives in the table. This was measured
on 2026-09-24 over a `git archive HEAD spikes/p9` copy in the scratchpad, not over the tree, because the probe
rewrites a tracked file. After these four edits the probe prints `predicates parsed 279/279; failed 0/279`,
`mixed unparenthesised and/or (precedence-dependent) 0/279`, and
`on_false forms 99/99 in the closed set: {'loop-back': 69, 'keyword': 30}`, and it exits 0.

`catalog-b` is the unshipped extended file (F42's defect table: *"no — catalog-b is the unshipped extended file
(ADR-0027)"*). ADR-0072's rule for a **shipped** activity edit (version bump, changelog row, P6 re-run) therefore
does not reach it. Re-run the probe and commit the regenerated `spikes/p9/results-s-grammar.json` in the same
commit. The probe `json.dump`s into it on every run.

**The record** (a new record, next free id at writing) supersedes **ADR-0114 decision 1 in part**, at the sentence
*"The two F42 parse failures and three `on_false` defects are repaired at Step 7e before the tag; nothing else
stands between the v0 pack and its tag."* The record states what the tag's condition was **over the shipped
pack**: decision 1's condition (b) was met by the one in-scope repair, and `redkiln-pack-v0` stands. The other four
defects were never in a shipped pack, and they are repaired here, after the tag. Conditions (a) and (b), decision 2
and every forbids bullet stand. Back-link: ADR-0114 `superseded_by: [<id>]`. Checkpoint 7b's re-pin is folded into
this record (D21-42 decision 23), and it is a re-pin rather than a widening:

- `grep -qxF 'predicates parsed 279/279; failed 0/279'`;
- `grep -qxF 'mixed unparenthesised and/or (precedence-dependent) 0/279'`;
- `grep -qE '^on_false forms 99/99 in the closed set'`;
- the probe's rc is tightened from `-le 1` to `-eq 0`, because a clean probe exits 0;
- the stale comment *"s7e:117 expects 287/287 afterwards"* and the FAIL message naming `exists(ProblemRecord)` are
  rewritten, with dates.

**Owed beside it.**

- A dated correction to `spikes/F42-DRAFT.md`, under rule 2.4, beside its *"**285/287** parse; 2 fail"*.
- Dated notes in `s7e` §7e.4, whose *"the count settles at **278/279**"* was true until this stage. *(Amended
  2026-09-25, s8a fix round 8: this read `s7e` §7e.2. The sentence, and the note placed beside it, sit in `s7e` §7e.4. Amended 2026-09-25, s8a fix round 15: both refs qualified with `s7e`, since the bare §7e.4 failed `check-refs.py`.)*
- The historical-capture doc comments in `crates/rk-kernel/tests/it/probe_differential.rs` (*"valid **until `s7e`
  §7e.2 repairs the five defects**"*) and `crates/rk-kernel/tests/it/on_false.rs` (*"the **only live instances**"*).
  Both tests pin F42's historical measurement over literals and a captured fixture, and their assertions do not
  move.
- Dated notes in `s7b-kernel`'s pack-defects prose, whose *"repairs them there before the v0 tag"* and *"the **only
  live instances**"* (in the prose and in its `on_false_closed_set` test-list row) are the same two claims. *(Added
  2026-09-25, `s8a` fix round 4: this site was missing from the list.)*
- A dated note in `s7e` §7e.1, beside *"its three `on_false` defects and one bare hyphenated value (F42) stay
  where they are until a Phase-2 pass"*: this stage repairs all four, in Phase 1. *(Added 2026-09-25, `s8a` fix
  round 5: this site was missing from the list.)*
- *(Added 2026-09-25, `s8a` fix round 12: two more sites were missing from the list.)* A dated note in the design's
  §10.5 S-GRAMMAR row, beside *"repaired at Step 7e, not here"*: only `packs/incident.toml` was. And dated notes in
  `crates/rk-kernel/src/predicate/on_false.rs`, whose module doc says the three rows are the rule's *"only live red
  fixtures"*, repaired *"there before the v0 tag"*, and whose `UnknownKeyword` variant doc says *"the three live
  `catalog-b` defects"*. The code does not move.

⚠️ D21-41 (i) said *"re-pinned to 278/279"*. D21-42 decision 23, which was taken later and adds the `catalog-b`
repair, makes the measured reading 279/279. This stage pins what the probe prints after the repair.

### 8a.5 Checkpoint 7f: D21-9 executed

D21-9 already decided the disposition, and ADR-0134 decision clause 5 named the consequence (*"**Checkpoint 7f
becomes non-re-runnable at that commit.**"*). That sentence is about the text 7f carried then, and that text stays
non-re-runnable. The re-scope below is a new, dated text, not a reversal of the clause. **D21-43 N27 confirmed
this reading on 2026-09-24:** *"the clause is about 7f's old text. Re-scoping 7f is execution, not supersession."*
No record supersedes ADR-0134 here (finding (3)). Executing D21-9 has two parts:

1. **The tracker.** `PROGRESS.md` row 7f's *"RE-RUN PENDING"* becomes a terminal disposition that quotes D21-9, so
   that it does not read as dropping the preamble's re-run obligation. The `79e0ef4` pass stands as evidence about
   the text it attested.
2. **The literal is re-scoped to the discharged-state form**, by dated amendment in `s7f` §7f.5, so that Checkpoint
   7f can be run whole at this exit (D21-40 decision 4: *"repaired … not carried as debt"*). The block asserts the
   state that is true now:
   - exactly three `conformance_` lints;
   - `conformance::conformance_every_step_kind_has_an_executor` and
     `conformance::conformance_every_port_has_a_call_site` each `... ok`;
   - `conformance::conformance_every_trailer_key_has_a_producer` ignored with `ARMED: Slice-Verdict has 0
     producers`;
   - `test result: ok. 2 passed; 0 failed; 1 ignored`.

   The armed reason's tail *"discharged by s7h"* is false: s7h did not discharge it (ADR-0160). It is corrected to
   `discharged by s8e`, which is D21-41 (e)'s stage. `s7f`'s reason regex follows. `s7h` guard (6) greps only the
   reason's prefix and is unaffected.

⚠️ **Packet (i) option A read D21-9 as "a PROGRESS cell edit only … no checkpoint edit".** This stage does both,
because D21-40 decision 4 and this stage's brief require 7f to run whole at the exit (finding (3)). `s8e`'s
Slice-Verdict commit moves this literal again, together with `s7h` guards (6) and (7).

*(added 2026-09-24, D21-47; cross-file review item 2)* **Checkpoint 8a's own guard (7) pins the same armed state,
and it stays pinned here.** It asserts `ignored, ARMED: Slice-Verdict has 0 producers`, `discharged by s8e` and
`0 passed; 0 failed; 1 ignored`. That is true until `s8e` un-ignores the lint, and it is false from then on. So
every later replay of this block (`s8b` guard (0), `s8i`, `s8j`) would fail on it. D21-47's time-bound rule gives
the amendment to the stage that moves the premise, in the same commit. **`s8e`'s Slice-Verdict commit amends
Checkpoint 8a guard (7) by dated line** to `1 passed` with the ARMED reason absent, and it lists 8a as
provisional. This stage does not pre-empt that. A guard made conditional on the `#[ignore]` being present would
pass whether or not the lint was ever discharged.
*(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.13.)* **Done.** `s8e`'s Slice-Verdict commit deletes the
lint's `#[ignore]` and amends guard (7) by dated line: the lint's `... ok` line, `1 passed; 0 failed; 0 ignored`, and
the ARMED string absent from `conformance.rs` with grep's `rc` exactly 1. The three armed lines are replaced in the
block rather than struck, because a struck line would still carry the string Checkpoint 8e (10h) asserts absent.
Checkpoint 8a is `passed bf12a1c, re-run pending` in `PROGRESS.md` §1 until Checkpoint 8e (10h) re-runs it whole.

**7f's block runs `redkiln run`, so its whole re-run is a dispatch hazard once the cards bind.** At this stage
the hazard is closed. **0 of 8** shipped cards carry `permission_mode`, and `CardFile`'s load refuses before any
spawn (`.ok_or_else(|| ConfigError::PermissionModeAbsent { card: card() })?;`). Guard (11) asserts that condition
before either whole run and asserts afterwards that no `dispatch_completed` row appeared. From `s8h` on the cards
bind, so a later stage that re-runs 7f whole must follow §8a.10's rule.

### 8a.6 Codex's phase (D21-42 decision 22)

**The record** (a new record, next free id at writing) supersedes **ADR-0094 decision 2 in part**, at *"The Harness
port's second real adapter (Codex) is Phase 2 and stays behind the CI decision ADR-0010 defers"*, toward ADR-0007,
which is accepted and has not been superseded: *"the Codex adapter ships in Phase 4 ("reach"), and only after
S-CODEX passes."* The rest of decision 2 stands, including AC-499's phase clause. Back-link: ADR-0094
`superseded_by` gains this id beside §8a.2's.

**E-0094-2** (ERRATA, under ADR-0094) records that DECISIONS-2026-09-12 E12 was taken with ADR-0007's phase
sentence out of view. E12's row cites only ADR-0094 and the design. It corrects no Decision.

**Dated corrections**, each re-measured before it is written:

- **The design.** ⚠️ The phase-bearing S-CODEX row is in **design §10.4** (Open items): *"**Phase 2 is the
  phase** (ADR-0094 decision 2, 2026-09-11 …)"*. It is not in design §10.5. The design §10.5 S-CODEX row states no
  phase, and the design §10.2 phase table already says Phase 4 (finding (4)).
- **The registers carry six sites, not two.**
  - Four are in `references/2026-09-08-acceptance-criteria.md`: the `| 5 | The second harness |` row, the area's
    *"Phase-1 subset"* note, AC-496's body, and the *"Phasing means roughly half the area"* bullet.
  - Two are in `references/2026-09-08-success-measures.md`: SM-148's baseline correction and its table row.
    `success-measures.md` already says Phase 4 in its area summary.
  - The runbook's `s7a` (one site: AC-496's exclusion), `s7b-ii` (two sites) and `s7c` (one site) exclusion reasons
    also say *"Phase 2"*. They take dated notes.
  - *(Amended 2026-09-25, `s8a` fix round 7.)* The tracker's E12 row in `runbook/PROGRESS.md` also says *"**Phase
    2** — ADR-0094 decision 2 stands"*, and ADR-0167's Consequences promise a dated note on every runbook line that
    learned "Phase 2" from E12. It takes one. Sweep (9) does not read the tracker, so no count moves.
  - *(Amended 2026-09-25, `s8a` fix round 10.)* The same tracker's *"Tier-E records landed — 2026-09-12,
    `8f612a3`"* row says *"the Decision (Codex = Phase 2) stands"*. It takes the same dated note, in the E12
    row's form. Sweep (9) does not read the tracker, so no count moves.
  - *(Amended 2026-09-25, `s8a` fix round 12.)* Two crate doc sites say the same: the module doc of
    `crates/rk-kernel/src/verify/eligible.rs` (*"waits on `rk-adapter-codex`, Phase 2 (ADR-0094 decision 2)"*,
    noted on a `//!` line so AC-487's code-line denylist does not move) and the `ac239_gate_eligible_list_is_data`
    row of `crates/rk-kernel/tests/it/verify.rs`'s doc table (*"the Codex-present half (Phase 2)"*). Each takes the
    same dated note; the code and the tests do not move. Sweep (9) reads only the registers, so no count moves.

  Checkpoint 8a's sweep (9) requires every register line that matches `Codex[^|]*\bPhase 2\b` to cite the new
  record. At `ca8f387` it matches exactly those six lines.

  *(Amended 2026-09-25, `s8a` fix round 4: the registers carry **seven** sites, not six. AC-239's body, a fifth
  acceptance-register site, reads *"the second Harness adapter is **Phase 2**"* under the same 2026-09-11 ADR-0094
  decision 2 phase clause. It names no Codex, so the regex above could not see it. It takes the same dated note, and
  sweep (9)'s pattern is widened to `(Codex|[Hh]arness adapter)[^|]*\bPhase 2\b`, which matches seven lines.
  ERRATA E-0094-1's "Codex = Phase 2" clause is corrected by E-0094-2.)*

  *(Amended 2026-09-25, `s8a` fix round 15: sweep (9) asserted only that no matched line lacks the record, so it
  passed over zero lines, and the seven sites rewritten to "Phase 4" or deleted instead of noted passed it. It now
  carries a floor of seven matched lines, the newtype sweep's shape.)*

`rk-story-codex-adapter` is retired at `s8j` (D21-41 (f)), not here.

### 8a.7 ADR-0085's option headings, enforced from ADR-0164 on (D21-42 decision 24a)

`runbook/tools/check-adr-options.py` is added beside the other two checkers. It works as follows:

- It reads every `references/adr/NNNN-*.md` with `NNNN >= 0164`, or the records in `--dir <path>` for the controls.
- Under `## Considered options` it requires at least one `### Option — <name>` heading, and no `###` heading of any
  other form.
- It prints `scanned N records (>= 0164); flagged K`. It exits 1 when K > 0 **or when N = 0**. A lint over zero
  records is the `lattice.py --packs <path>` green that printed a baseline over zero packs.
- `INDEX.md`'s validation block gains it at the tracker commit.

**One dated erratum** goes under ADR-0085's section (next free `E-0085-n`; `E-0085-3` at authoring). It lists
ADR-0132 through ADR-0161, **thirty records**, as non-conforming to *"Each considered option is one `### Option —
<name>` heading"*. Measured at `ca8f387`: each of the thirty has zero such headings, and 0162, 0163 and 0164 carry
3, 7 and 4. **Their accepted bodies are not edited**, and Checkpoint 8a asserts that byte for byte against the
stage's base commit. All three of this stage's records are written in the conforming form. *(noted 2026-09-25, s8a
fix round 2: ADR-0090 and ADR-0117, accepted before ADR-0132, carry no `### Option — ` heading either. They are
outside decision 24a's list, so the erratum's count stays thirty, and E-0085-3 names them in a dated note. Whether
they join the list is the operator's.)*

### 8a.8 Checkpoint 7e's FAIL message: a backtick inside double quotes, amended (D21-46)

D21-46's integration calls read: *"Checkpoint 7e's backtick-executing FAIL message, the same defect as 7h's (10a),
is fixed by dated amendment in **s8a**."* `s8e` found it and left it here. Its finding 10 says *"This stage fixes
7h's only."* This is the execution of an operator decision, not a new one, so there is no record.

**The line, found by its content** at `c5d19d7`, never by a line number. It is the enabled-pack guard in `s7e`
§7e.1's block:

```text
test -f .redkiln/config.toml || { echo "FAIL: .redkiln/config.toml absent — Step 7d's `init` did not run, or did not scaffold it (E6)"; exit 1; }
```

Backticks inside double quotes are command substitution. When the guard fires, bash runs `init` as a command
before it echoes. Git Bash has no `init`, so it prints `command not found` on stderr and then a message reading
*"Step 7d's  did not run"*. A Linux host resolves `/sbin/init`. So the FAIL path runs a command nobody wrote and
prints a message that is wrong.

⚠️ **The line is not in Checkpoint 7e's block** (finding (10)). Checkpoint 7e's block is the one under
*"**Checkpoint 7e.** All thirteen"*, and it has no hit. D21-46 calls the line Checkpoint 7e's. The amendment is
the same either way. What changes is the tracker: no byte of Checkpoint 7e moves, so PROGRESS row 7e
does not move.

**The amendment**, dated and made in the same commit as §8a.4's `s7e` notes. The two backticks are removed and no
other byte changes. This is `s8e`'s form for 7h (10a):

```text
test -f .redkiln/config.toml || { echo "FAIL: .redkiln/config.toml absent — Step 7d's init did not run, or did not scaffold it (E6)"; exit 1; }
```

The fix removes the backticks. It does not switch to single quotes or escape them. The message contains an
apostrophe (`7d's`), so single quotes would need a `'"'"'` splice. An escaped backtick only stays safe until someone
re-quotes the line. A dated comment goes in as **a new line inserted directly before the amended line**, in the same
block. No existing line is turned into the comment. The comment contains no backtick. *(Wording corrected
2026-09-24, D21-47: this said "the directly preceding line … gets a dated comment", which could be read as editing
that line. Guard (9a)'s comparison drops the note line and requires every other line to match, so only an
inserted line passes.)*

```text
# [amended 2026-09-24, s8a §8a.8] D21-46: the two backticks around init are removed — inside double quotes they ran init as a command on the FAIL path.
```

**Neither block can be re-run whole, so the amendment is verified another way.** Checkpoint 7e's block ends in
`git add … && git commit` and `git tag -a redkiln-pack-v0`. The tag already exists, so a whole re-run fails on its
last line even on an unchanged tree, and a run that got further would mint a commit. `s7e` §7e.1's block runs `init` and
copies files over the live `.redkiln/`. Guard (9a) verifies the amendment in four other ways:

1. **Static.** The backticked bytes are gone from `s7e`, the new line appears exactly once, and the dated comment is
   present.
2. **No other bash byte moved.** Every bash block of `s7e` is compared with its counterpart in `S8A_BASE`. The block
   count must match. After the old line is mapped to the new one and the dated comment is dropped, every line must
   match. That covers Checkpoint 7e's block too. §8a.4's `s7e` notes are prose outside every block. If one lands
   inside a block, this comparison fails, and that is correct. **Control:** the same comparison, run without
   dropping the dated comment, must fail.
3. **Behavioural, from the file's own bytes.** The amended line is taken from `s7e` and run in an empty scratch
   directory with no `.redkiln/config.toml`, so the guard fires. A trap `init` that writes a marker comes first on
   `PATH`. The run must exit 1, print the message verbatim with `init` in it, and leave no marker. **Control:** the
   old line, taken from `S8A_BASE`'s bytes and run the same way, must leave the marker. Without that control, an
   amended run that left no marker would prove nothing, because the trap might simply never have been reached on
   `PATH`.
4. **Syntax.** `check-bash-syntax.py` passes over the corpus.

**The corpus-wide scan.** Guard (9a) also reads every bash block in `runbook/` for a backtick that bash would
execute: one outside quotes, one inside double quotes, or one in the body of a heredoc whose delimiter is unquoted.
It scans code only, so comments, single-quoted strings and the bodies of quoted-delimiter heredocs are skipped.
Quote state carries across lines, as it does in bash. ⚠️ A scan that works one line at a time, which is `s8e`
guard (10)'s form, reports two false hits at `c5d19d7`: one in `s8f` and one in `s8j`. In each, a single-quoted
Python program spanning several lines has backticks inside its own strings (finding (11)). At `c5d19d7` this scan
reads **99 blocks and 2 hits**: this line and 7h's (10a). Its controls:

- a planted bash block whose double-quoted `echo` message carries a backticked word must go red;
- `S8A_BASE`'s copy of `s7e` must go red on the `config.toml` line;
- a single-quoted needle, a comment, a multi-line single-quoted string and a quoted-heredoc body, each carrying
  backticks, must all pass;
- a file with no bash block must be refused (rc 2), because a scan over zero blocks reads as a green.

**7h's identical defect is `s8e`'s to fix, and it is admitted by name until then.** `s8e` §8e.13 amends 7h guard
(10a)'s FAIL message: *"The dated amendment removes the two backticks and changes no other byte of the line."*
Checkpoint 8e guard (7c) asserts that the old string is gone. This stage runs before `s8e`, so the scan admits
exactly one hit that is not this stage's: a `BT-HIT` in `s7h-lane-completes.md` on the line that reads `binary arm
did not pass`. Any other hit fails. Once `s8e` lands, that hit is gone and the scan reads 0 hits, and a re-run of
this block passes unchanged.

### 8a.9 Red first — the named tests

Each test is watched red for the **right** reason (the absent behaviour, not a compile error or a filter that
matches nothing) before the code that turns it green.

| crate | `tests/it` name | red because |
| --- | --- | --- |
| `rk-ports` | `change_summary::text_variant_serialises_sha_diff_and_manifest` | there is no `Text` arm, so no `sha`/`diff`/`manifest` members reach a catalog reference *(and no `files`; added 2026-09-25, s8a fix round 10)* |
| `rk-ports` | `change_summary::binary_arm_is_not_constructible_outside_rk_ports` | a `trybuild` case (`tests/ui/change_summary_binary_not_constructible.rs` plus a committed `.stderr`) that must fail for the **privacy** reason. It is its own harness test: Checkpoint 7a counts `trybuild_pins_the_reason_each_port_fixture_fails`'s four cases |
| `rk-ports` | `fake_behaviour::toolchain_fake_classifies_binary` *(amended)* | its `assert_eq!(format!("{summary:?}"), "ChangeSummary(())")` becomes: a text range yields `Text`, and a binary range refuses by name |
| `rk-git` | `diff::range_diff_lists_every_changed_path_with_its_change_kind` | no range read exists |
| `rk-git` | `diff::range_diff_refuses_an_unresolvable_range` | same |
| `rk-git` | `diff::range_diff_marks_an_lfs_pointer_path` | same. Over a fixture whose commit adds `data/table.json` as a git-lfs pointer blob, the path must come back marked as a pointer, and an ordinary `.json` beside it must not |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_text_variant_from_git_diff` | the body answers `Unsupported` from the registry. Over a two-commit fixture repository it must return `Text` with one manifest row per changed path |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_a_range_carrying_a_binary_path` | a `.uasset` in the range must refuse through the re-dated row and never yield a `Text` that silently omits the path (AC-497) |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_an_empty_range` | the kernel passes `""` today (§8a.2), and an `Ok` there is a vacuous green |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path` | `classify` answers `Text` for `data/table.json`. The range must refuse naming the path, never yield a `Text` whose `diff` is the pointer's `oid`/`size` delta (§8a.2 item 2a) |
| `rk-git` | `diff::range_diff_marks_a_binary_blob_at_either_end` *(added 2026-09-25, s8a fix round 1)* | the entry carried no binary mark. An extensionless NUL-bearing blob deleted in the range, and one added at a head the checkout is not on, must each come back marked, decided on the blobs |
| `rk-git` | `diff::range_diff_marks_a_missing_final_newline` *(added 2026-09-25, s8a fix round 1)* | git's `\ No newline at end of file` line was dropped, so a trailing-newline-only change read as a `-X`/`+X` no-op. The hunk bodies must equal `git diff`'s |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold` *(added 2026-09-25, s8a fix round 1)* | `classify` reads the working tree and answers `Text` for an absent file, so both ranges above answered `Ok(Text)` with the path's row marked `Text` (AC-497). Each must refuse naming the path |
| `rk-adapter-toolchain-rust` | `registry::ac489_capability_registry_dated` *(amended)* | its *"change_summary answers Unsupported from its registry row"* over `HEAD~1..HEAD` moves to the binary case |

That is nine new tests and two amended ones. *(amended 2026-09-25, s8a fix round 1: the three rows marked
added make twelve new tests, and guard (7)'s selections name them. Guard (10)'s floor stays **749**, which the three
only exceed; re-deriving it to 752 would move the floor chain every later stage computes from 749, and that is not
this round's to move.)* *(amended 2026-09-25, s8a fix round 3: thirteen, not twelve. D21-50 added the unfeatured
`recorded_log::the_recording_sources_are_the_committed_logs_inputs_byte_for_byte`, measured 753 passed over the
whole suite, and guard (7)'s `rk-kernel` selection now names it, 4 → 5. The floor stays **749** for the reason
above.)* *(amended 2026-09-25, s8a, D21-51: fifteen. `rk-adapter-toolchain-rust` gains
`classify::classify_is_invariant_to_a_cargo_manifest` and `classify::classify_source_spawns_nothing` (ADR-0165
decision 9's settlement), and guard (7)'s selection for that crate names both, 6 → 8. The `Text` arm's
git-as-authority case and the single-revision named refusal (decisions 4 and 6) are folded into
`change_summary_text_variant_from_git_diff` and `change_summary_refuses_an_empty_range`. The floor stays **749**.)* Unchanged, and asserted as regression pins:

- `rk-adapter-toolchain-node`'s `registry::ac489_capability_registry_dated` (Node still refuses);
- `rk-kernel`'s `probe_differential::grammar_agrees_with_the_s_grammar_probe` and `on_false::on_false_closed_set`;
- `redkiln-migrate`'s `output::ac475_zero_change_summary_values_in_output`.

**`s8i` supersedes three of these pins by name** (§8a.2 item 1): `binary_arm_is_not_constructible_outside_rk_ports`,
guard (6)'s no-`pub`-field line, and guard (6)'s `sealed_open` floor. *(Amended 2026-09-25, s8a fix round 8: this
read **two**.)* That is expected. It is not drift, and nobody should restore the pins. *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* Done: S-BIN has reported (F43, proposed *(dated 2026-10-15, `s8i` close, D21-154: F43 and ADR-0199 accepted)*), ADR-0199 (proposed) fixes the construction rule, and guard (6) carries it in their place.

⚠️ **Checkpoint 7c becomes provisional, and this stage does NOT re-run it.** Its named
`ac489_capability_registry_dated` is amended here. Its block must run from a shell outside Claude Code, and it
makes a paid haiku probe. **D21-43 N28 gives the whole re-run to `s8j`:** *"s8j re-runs it whole, from a plain
terminal through the launcher, with the live probes."* `PROGRESS.md` row 7c is marked *"passed …, re-run pending
`s8j` (D21-43 N28) — `s8a` amended `ac489_capability_registry_dated`"*. Finding (5) is settled by N28.

### 8a.10 Checkpoint 8a

- **The three records exist and are `accepted`.** Each carries its supersession back-links, and each uses `### Option
  — ` headings. The two errata exist. The README's index rows, census and next-free sentence are updated. The thirty non-conforming records' **bodies**, the bytes
  below the closing frontmatter `---`, are unchanged since `S8A_BASE`. *(amended 2026-09-24, D21-47: this compared
  the whole files. That contradicted guard (3), which requires a back-link on ADR-0135, one of the thirty. It would
  also have broken on any later stage's back-link to one of them, such as `s8c`'s and `s8d`'s to ADR-0140. A
  back-link is frontmatter, and an accepted body is what ADR-0092 keeps immutable.)*
- **`ChangeSummary` is split.** `Text` is finalised and `Binary` is sealed. The Rust adapter produces `Text` through
  `rk-git`, and refuses a binary path and an empty range by name. *(amended 2026-09-24, D21-47)* Its
  `change_summary` body calls `rk_git::diff::range_diff`, and that read is a gix read: `GitOp` is byte-unchanged and
  `diff.rs` names no exec type. *(amended 2026-09-25, s8a fix round 2: `GitOp` keeps every `S8A_BASE` variant and
  gains none named for a range read, so the later stages' planned variants do not break a whole re-run.)* The adapter's source carries no `"git"` literal on any code line, so it names git
  as no program, `run::run_program` included. *(amended 2026-09-25, s8a fix round 1: this said "so it spawns git by
  no route", which the literal ban does not show, because rk-git's `git_exec::run` spawns git with no literal at the
  caller. The adapter's code lines now name `rk_git` only as `rk_git::diff` and `rk_git::path` and name no
  `git_exec`, and `diff.rs` and `path.rs` name no exec item. A route through another crate, or a program name
  assembled at run time, is still not seen.)* *(amended 2026-09-25, s8a fix round 2: `diff.rs` and `path.rs` also
  name no `std::process`, `process::Command` or `gix::command` item and no `"git"` literal, so a spawn through an
  aliased std `Command` or through gix's own spawner is seen. A spawn through a re-export under another name, in
  another crate, is still not seen.)* *(amended 2026-09-25, s8a fix round 11: "an aliased std `Command`" held only for
  a flat import. A nested-brace `process::{Command as C, …}` import passed; the ban now fires on that import line,
  though not on the `C::new(…)` call line. And the call-site check did not tie the body's `range_diff(` to rk-git: a
  same-crate `fn range_diff` shadowing it passed. The name must now resolve through an unrenamed
  `use rk_git::diff::…range_diff…` import, or be called path-qualified, with no local definition or binding of it. A
  glob import from another crate that re-exports a `range_diff` is still not seen.)* *(amended 2026-09-25, s8a fix
  round 15: the `rk_git::diff`/`rk_git::path` allowlist is crate-wide over the adapter's `src`, and so a time-bound
  pin, the shape fix round 2 repaired for `GitOp`: `s8c`'s record V puts the `package` handler in this adapter and
  requires its `BuildArtifact` to carry the project root's `HEAD` read through `rk-git`, which neither module
  holds. `s8c` owns the pin: the commit that lands that HEAD read amends guard (6)'s allowlist by a dated note, or
  places the read where the allowlist already admits it, and `s8c` records which. Until then a whole re-run of
  this block after `s8c` is red on that line by design, not by drift.)*
  *(dated 2026-09-26, s8c build, P5; D21-62 C, provisional; record V = ADR-0170 decision 6, proposed.)* `s8c` took
  the first choice: guard (6)'s allowlist is AMENDED, by a dated note on its code line, to admit `rk_git::head`, the
  one shared read-only HEAD read (`crates/rk-git/src/head.rs`, `head_commit`, a gix open-at-the-directory read that
  spawns nothing). The Rust adapter's `sca_scan` handler reads each advisory database's commit through it before and
  after its tool runs, and its `package` handler reads the project root's `HEAD` through it, so one amendment covers
  both reads. `head.rs` joins `diff.rs` and `path.rs` under the exec-route ban, and a pass control admits the
  `rk_git::head` import while a fail control beside it still fires on `rk_git::gitops`. `s8c` §8c.4 records the choice.
- **Checkpoint 5's new ban holds**, including the `rk-kernel` production ban and the opacity pin. Every control
  fires or is admitted as designed.
- **`catalog-b` is repaired**, the probe prints 279/279, 0/279 and 99/99 at rc 0, and the committed JSON is
  unchanged by the re-run (compared as parsed JSON, not with `git diff` under autocrlf).
- **Checkpoint 7f's discharged state is asserted by name.**
- **The Codex phase sites carry their dated corrections, and E-0094-2 exists.**
- **The whole suite is green** above its floor, and fmt and clippy are green. The expected value is **re-derived
  at stage open from the last whole-suite reading before this stage**. That reading is **ADR-0164's evidence**:
  **740 passed / 0 failed / 9 ignored over 44 results**, committed at `992d257`, measured on the tree Checkpoint 7i
  passed. Add this stage's nine new tests: 740 + 9 = **749**. *(Label corrected 2026-09-24, D21-47: this said
  "Checkpoint 7i's recorded count". Checkpoint 7i's block runs no whole-suite count, and 740 is ADR-0164's
  number.)* If the reading at open differs, the literal in (10) is amended by dated note before the first run.
- **Checkpoint 7e's FAIL message is amended (§8a.8).** The backticked bytes are gone from `s7e`, no other bash byte
  of `s7e` moved since `S8A_BASE`, and the amended line, run from its own bytes, fails without running `init`.
  No `runbook/` bash block carries a backtick that bash would execute, apart from 7h (10a)'s, which `s8e` removes.
- **Checkpoints 7i (entry), 5, 7a, 7b and 7f each run whole from their own bytes and reach their last line.** No
  other checkpoint is re-run. 7c is `s8j`'s (D21-43 N28). Under `RK_CP_NESTED=1`, when another checkpoint runs
  this block as its entry gate, guard (11) skips these runs and says so (D21-46).
- **Neither whole run that can reach `redkiln run` dispatched.** ~~No shipped card carries `permission_mode`~~ *(dated 2026-10-02, s8h build: from `s8h` on every shipped card carries one, and the launch-check refusal test reports `ok` instead, `s8h` §8h.14)*, and
  the store and every run journal hold the same number of `dispatch_completed` rows after the runs as before. Each
  run had `CLAUDE_CONFIG_DIR` at an empty trap directory, which is still empty, and had `REDKILN_LAUNCH_RECORD`
  unset (D21-46).
- *(added 2026-09-24, D21-47; re-pointed 2026-09-24, final consistency pass)* **Every process-level run in this
  block uses one wrapper**, the shared `runbook/tools/nodispatch.sh`, set up by guard (0)'s `nd_setup`. That covers
  the build (1), the named selections (7), the whole suite (10) and the whole re-runs (11). The wrapper puts a
  **compiled** fake `claude` (`claude.exe` on Windows) first on `PATH`: it prints the pin for `--version`, and logs
  and refuses with 97 any other call. It drops every other `PATH` directory that holds a `claude`, unsets
  `REDKILN_LAUNCH_RECORD`, and points `CLAUDE_CONFIG_DIR` at an empty trap. `nd_check` asserts after each run, and
  again at the block's end on both `RK_CP_NESTED` branches, that the trap is still empty and that the fake logged no
  call. Because the fake is compiled, a Rust `Command::new("claude")` resolves it too, which settles finding (14).
- **The legacy run worktree is back where it was**, at `73c087a`, clean, with `rk-run/rk-fix-doctor-test-count`
  unmoved (D21-44; D21-42 18).

🛑 **The rule for any whole re-run of Checkpoint 7f after `s8h` lands** (and of 7g and 7h, which D21-43's integration
calls name, and of 7i's run arm). Once the cards bind, guard (11)'s card
condition is false, and this block **refuses** rather than run 7f bare. A stage that re-runs 7f whole from then on
does two things:
- it runs 7f against a fixture config whose `[adapters] harness_binary` names the `rk-fixture-child-claude` binary,
  which `s7d`'s `ConfigFile` row (RT-49) calls *"the one way a process-level test reaches `rk-fixture-child-claude`"*;
- it asserts that no real `claude` was spawned. The minimum is guard (11)'s before/after `dispatch_completed`
  equality, re-read as "every added row came from the fixture child".

`s8h` §8h.7's launch-record refusal makes a bare `redkiln run` refuse before any spawn. That is a second guard
against accident, not a substitute, and `s8h` says of it that *"the variable can be forged"*. The re-running stage
says in its own file which of these it does.

**Nesting and the no-dispatch trap (D21-46's integration calls).** When this block runs with `RK_CP_NESTED=1`, as
another checkpoint's entry gate, guard (11) skips its whole re-runs and prints that it did. Every other guard
runs. Each whole run that guard (11) does make is launched with `RK_CP_NESTED=1`, with `CLAUDE_CONFIG_DIR` at an
empty trap directory that must still be empty afterwards, and with `REDKILN_LAUNCH_RECORD` unset. From `s8h` on,
that makes the launch-record gate refuse before spawn. The trap uses `s8g`'s form, with its synthetic control.
It is a third guard beside the card condition and the `dispatch_completed` count, and it replaces neither.
*(re-pointed 2026-09-24, final consistency pass)* The trap and its control now come from the shared
`runbook/tools/nodispatch.sh`. When a re-runner already runs this block under that wrapper (`ND_ACTIVE=1`), guard (0)
verifies and reuses the enclosing wrapper rather than building a second one.

🛑 **ADR-0164's flip is not this stage's act.** It is `proposed` and flips only on the operator's ledger line. This
block does not read its status. *(dated 2026-09-26, s8c build: it flipped `accepted` on D21-66, under D21-64 ruling 2.)*

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# Run it AFTER `git add` of the work: (5), (8) and Checkpoint 7a read the index and the tracked set.
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`,
# MODULE-QUALIFIED and `--exact`, and asserts each `test <name> ... ok` line AND an exact count.
# Re-runs ONLY the entry gate (7i) and 5, 7a, 7b, 7f, which this stage's edits touch. 7c is s8j's (D21-43 N28).
# Under RK_CP_NESTED=1 (this block is another checkpoint's entry gate) guard (11)'s whole re-runs are skipped;
# every other guard runs (D21-46). Checkpoint 7e is amended, not re-run: guard (9a).
# [amended 2026-09-24, s8a D21-47] Every cargo run and every whole re-run below goes through guard (0)'s rkw wrapper.
# [amended 2026-09-24, final consistency pass] That wrapper is now the shared runbook/tools/nodispatch.sh: rkw became
# nodispatch, and nd_check follows every cargo run and every whole run.

# ---- (0) THE NO-DISPATCH WRAPPER (D21-47): the shared runbook/tools/nodispatch.sh. [re-pointed 2026-09-24, final
# ---- consistency pass: it replaces this guard's own sh fake and trap.] A COMPILED fake claude first on PATH (claude.exe
# ---- on Windows, which a Rust Command::new("claude") resolves) prints the pin for --version and logs, then refuses
# ---- with 97, any other call. Every other PATH dir holding a claude is dropped, REDKILN_LAUNCH_RECORD is unset, and
# ---- CLAUDE_CONFIG_DIR is an empty trap. nd_setup runs the helper's own controls; under an enclosing wrapper
# ---- (ND_ACTIVE=1, a re-runner's nodispatch) it verifies and reuses that one. In this session CLAUDE_CONFIG_DIR is the
# ---- operator's real account.
test -f runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh is absent — the no-dispatch wrapper cannot be built"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh"; exit 1; }
nd_setup "$PWD/target/s8a-nd" || exit 1   # NOT under target/s8a-ctl: (4) removes that
# A stage control beyond the helper's: a PLANTED launch record is removed, and a child sees the trap.
got=$(REDKILN_LAUNCH_RECORD=planted nodispatch sh -c 'printf "%s|%s" "${REDKILN_LAUNCH_RECORD-unset}" "$CLAUDE_CONFIG_DIR"'); rc=$?
[ "$rc" -eq 0 ] && [ "$got" = "unset|$ND_TRAP_NATIVE" ] || { echo "FAIL: CONTROL — nodispatch did not unset a planted REDKILN_LAUNCH_RECORD and point CLAUDE_CONFIG_DIR at the trap (got '$got', rc=$rc)"; exit 1; }
nd_check "guard (0)" || exit 1

# ---- (1) BUILD-OR-FAIL, FIRST. A stale binary makes every later whole run a statement about old code.
out=$(nodispatch cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8a NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "the build" || exit 1
test -f ./target/debug/redkiln || test -f ./target/debug/redkiln.exe || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) INPUTS, RECORDED AT WRITE TIME (D21-47: "Records are located by the id recorded at write time").
# ---- [amended 2026-09-24, final consistency pass] These were environment inputs every re-runner had to supply. Each
# ---- id is now written into its line below by a dated edit in the commit that writes its record, and S8A_BASE by a
# ---- dated edit at stage open. The block refuses while a ____ placeholder remains. FIX_ITEM is §8a.1's item, a literal,
# ---- so a re-runner's own FIX_ITEM (s8j's lane item) never reaches 7i's run.
# [written 2026-09-24, s8a, by dated edit] The three ids are the ones allocated in writing order (ADR-0165, ADR-0166,
# ADR-0167), and S8A_BASE is b8e7aa9, the HEAD this stage opened on. The ____ placeholders are gone.
CS_ADR=0165      # the ChangeSummary record (§8a.2)
PACK_ADR=0166    # the ADR-0114 record (§8a.4)
CODEX_ADR=0167   # the Codex-phase record (§8a.6)
S8A_BASE=b8e7aa9 # the commit this stage opened on
FIX_ITEM=rk-fix-doctor-test-count
for kv in "CS_ADR=$CS_ADR" "PACK_ADR=$PACK_ADR" "CODEX_ADR=$CODEX_ADR"; do
  printf '%s\n' "${kv#*=}" | grep -qxE '0[0-9]{3}' || { echo "FAIL: ${kv%%=*} is '${kv#*=}' in guard (2) — write the record's four-digit id here by dated edit the day it is written"; exit 1; }
done
printf '%s\n' "$S8A_BASE" | grep -qxE '[0-9a-f]{7,40}' && git rev-parse --verify --quiet "$S8A_BASE^{commit}" >/dev/null || { echo "FAIL: S8A_BASE is '$S8A_BASE' in guard (2) — write the commit this stage opened on here by dated edit; the untouched-bodies guard compares against it"; exit 1; }
export FIX_ITEM

# ---- (3) THE THREE RECORDS: one file each, accepted, supersessions declared, back-links present (ADR-0092 r1).
for id in "$CS_ADR" "$PACK_ADR" "$CODEX_ADR"; do
  printf '%s\n' "$id" | grep -qxE '0[0-9]{3}' || { echo "FAIL: '$id' is not a four-digit record id"; exit 1; }
  set -- references/adr/"$id"-*.md
  [ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: expected exactly one references/adr/$id-*.md"; exit 1; }
  grep -qxF 'status: accepted' "$1" || { echo "FAIL: $1 is not accepted — the checkpoint amendments rest on it; a proposed record flips only on the operator's dated line"; exit 1; }
done
cs_f=$(ls references/adr/"$CS_ADR"-*.md); pk_f=$(ls references/adr/"$PACK_ADR"-*.md); cx_f=$(ls references/adr/"$CODEX_ADR"-*.md)
# [amended 2026-09-24, s8a D21-47] 0008 joins the loop (its Decision and first forbids bullet move, §8a.2). 0099's
# back-link may be followed by a later stage's append; the three before it are what this stage wrote.
for t in 0008 0093 0094 0099 0135; do
  grep -qE '^supersedes: \[.*"'"$t"'"' "$cs_f" || { echo "FAIL: the ChangeSummary record does not declare it supersedes $t in part"; exit 1; }
  grep -qE '^superseded_by: \[.*"'"$CS_ADR"'"' references/adr/"$t"-*.md || { echo "FAIL: ADR-$t carries no back-link to $CS_ADR"; exit 1; }
done
grep -qE '^superseded_by: \["0115", "0127", "'"$CS_ADR"'"(, "[0-9]{4}")*\]$' references/adr/0099-*.md || { echo "FAIL: ADR-0099's back-link is not an APPEND to [0115, 0127]"; exit 1; }
grep -qE '^supersedes: \[.*"0114"' "$pk_f" && grep -qE '^superseded_by: \[.*"'"$PACK_ADR"'"' references/adr/0114-*.md || { echo "FAIL: the ADR-0114 supersession or its back-link is missing"; exit 1; }
grep -qE '^supersedes: \[.*"0094"' "$cx_f" && grep -qE '^superseded_by: \[.*"'"$CODEX_ADR"'"' references/adr/0094-*.md || { echo "FAIL: the ADR-0094 d2 supersession or its back-link is missing"; exit 1; }
grep -qF 'D17-9' "$cs_f" || { echo "FAIL: the ChangeSummary record does not root the conflict in D17-9 by name"; exit 1; }
set -- references/adr/[0-9][0-9][0-9][0-9]-*.md
n=$#; last=$(basename "${!n}"); last=${last%%-*}; next=$(printf '%04d' $((10#$last + 1)))
[ "$((10#$last))" -eq "$n" ] || { echo "FAIL: $n records but the last id is $last — the set is not contiguous"; exit 1; }
grep -qF "next free number is **\`$next\`**" references/adr/README.md && grep -qF "next free number is **\`$next\`**" runbook/00-preamble.md || { echo "FAIL: README or the preamble does not name $next as the next free number"; exit 1; }
# [added 2026-09-24, s8a D21-47] The README's index rows for the three records, and its census at both sites.
for id in "$CS_ADR" "$PACK_ADR" "$CODEX_ADR"; do
  grep -qE "^\| \[$id\]\($id-[^)]*\.md\) \|" references/adr/README.md || { echo "FAIL: the README index has no row for $id"; exit 1; }
done
grep -qE "^\*\*Purpose:\*\*.* \*\*$n records\*\*" references/adr/README.md || { echo "FAIL: the README's purpose line does not read **$n records**"; exit 1; }
grep -qF "The set is $n records over \`0001\`–\`$last\`" references/adr/README.md || { echo "FAIL: the README's census does not read $n records over 0001–$last"; exit 1; }
# [added 2026-09-25, s8a fix round 6] The preamble's own census, which s8a had left at 164 beside its next-free 0168.
grep -qF "\`references/adr/\` (**$n records**," runbook/00-preamble.md && grep -qF "\`0001\`–\`$last\` contiguous" runbook/00-preamble.md || { echo "FAIL: the preamble's census does not read $n records over 0001–$last"; exit 1; }

# ---- (4) ADR-0085 OPTION HEADINGS FROM 0164 ON, and the lint can go red: a mutated copy, and an empty dir.
out=$(python runbook/tools/check-adr-options.py 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: check-adr-options.py rc=$rc"; printf '%s\n' "$out"; exit 1; }
k=$(printf '%s\n' "$out" | sed -nE 's/^scanned ([0-9]+) records.*flagged 0$/\1/p')
[ -n "$k" ] && [ "$k" -ge 4 ] || { echo "FAIL: the option-heading lint scanned '${k:-none}' records (0164 plus this stage's three is 4)"; exit 1; }
rm -rf target/s8a-ctl && mkdir -p target/s8a-ctl/good target/s8a-ctl/bad target/s8a-ctl/empty || { echo "FAIL: cannot make the control dirs"; exit 1; }
cp references/adr/0164-*.md target/s8a-ctl/good/ && cp references/adr/0164-*.md target/s8a-ctl/bad/ || { echo "FAIL: cannot copy ADR-0164 for the controls"; exit 1; }
sed -i 's/^### Option — /### Option: /' target/s8a-ctl/bad/0164-*.md || { echo "FAIL: the control mutation did not run"; exit 1; }
out=$(python runbook/tools/check-adr-options.py --dir target/s8a-ctl/good 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the lint rejects an unmutated ADR-0164 (rc=$rc)"; exit 1; }
out=$(python runbook/tools/check-adr-options.py --dir target/s8a-ctl/bad 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the lint admitted '### Option: ' (rc=$rc) — it cannot go red"; exit 1; }
out=$(python runbook/tools/check-adr-options.py --dir target/s8a-ctl/empty 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the lint passed over ZERO records (rc=$rc)"; exit 1; }

# ---- (5) THE TWO ERRATA, and the thirty bodies byte-unchanged since the stage opened.
# [amended 2026-09-24, s8a D21-47] The erratum is ONE paragraph, E-0085-n with n >= 3, that names ADR-0132, ADR-0161 and
# "thirty"; E-0094-2 sits in ADR-0094's section and names ADR-0007 and E12. The thirty are compared on their BODIES
# (below the closing frontmatter ---), staged blob against S8A_BASE's blob: guard (3) itself requires a back-link on
# 0135, one of the thirty, and later stages add back-links (s8c's and s8d's to 0140). Whole-file equality was rc 1.
cat > target/s8a-ctl/errata.py <<'PY'
import re, sys
t = open("references/adr/ERRATA.md", encoding="utf-8").read().replace(chr(13), "")
def section(num):
    m = re.search(r"(?ms)^## ADR-" + num + r" .*?(?=^## ADR-|\Z)", t)
    return m.group(0) if m else ""
def paras(sec):
    return [p for p in re.split(r"\n\s*\n", sec) if p.strip()]
ok85 = [p for p in paras(section("0085")) if re.search(r"\*\*E-0085-([3-9]|[1-9][0-9])\b", p)
        and "ADR-0132" in p and "ADR-0161" in p and re.search(r"\bthirty\b", p, re.I)]
ok94 = [p for p in paras(section("0094")) if "E-0094-2" in p and "ADR-0007" in p and "E12" in p]
print("ERRATA e0085=%d e0094_2=%d" % (len(ok85), len(ok94)))
sys.exit(0 if len(ok85) == 1 and len(ok94) == 1 else 1)
PY
out=$(python target/s8a-ctl/errata.py 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the two errata are not each one paragraph in their section (E-0085-n naming ADR-0132, ADR-0161 and thirty; E-0094-2 naming ADR-0007 and E12): $out"; exit 1; }
set -- references/adr/013[2-9]-*.md references/adr/01[45][0-9]-*.md references/adr/016[01]-*.md
[ "$#" -eq 30 ] || { echo "FAIL: $# files in 0132..0161, expected 30"; exit 1; }
cat > target/s8a-ctl/fm_body.py <<'PY'
import subprocess, sys
NL = b"\n"
def body(b):
    if not b.startswith(b"---" + NL):
        return None
    end = b.find(NL + b"---" + NL, 3)
    return None if end < 0 else b[end + 5:]
def blob(spec):
    r = subprocess.run(["git", "cat-file", "blob", spec], capture_output=True)
    if r.returncode != 0:
        sys.exit("FM-BODY-FAIL: git cat-file blob %s rc=%d" % (spec, r.returncode))
    return r.stdout
if sys.argv[1] == "--selftest":
    raw = blob(sys.argv[2] + ":" + sys.argv[3])
    fm_only = raw.replace(b"superseded_by: [", b'superseded_by: ["9999", ', 1)
    body_edit = raw[:-2] + (b"X" if raw[-2:-1] != b"X" else b"Y") + raw[-1:]
    checks = [(body(raw) is not None, "the base file has a frontmatter"),
              (fm_only != raw and body(fm_only) == body(raw), "a back-link-only change passes"),
              (body(body_edit) != body(raw), "a one-byte body edit fails"),
              (body(b"no frontmatter" + NL) is None, "a file with no frontmatter is refused")]
    bad = [why for ok, why in checks if not ok]
    print("FM-BODY-SELFTEST " + ("OK" if not bad else "FAIL: " + "; ".join(bad)))
    sys.exit(1 if bad else 0)
base, paths = sys.argv[1], sys.argv[2:]
diff = []
for p in paths:
    a, b = body(blob(base + ":" + p)), body(blob(":" + p))
    if a is None or b is None or a != b:
        diff.append(p)
print("FM-BODY compared=%d differ=%d %s" % (len(paths), len(diff), " ".join(diff)))
sys.exit(1 if diff or not paths else 0)
PY
out=$(python target/s8a-ctl/fm_body.py --selftest "$S8A_BASE" "$(ls references/adr/0135-*.md)" 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qxF 'FM-BODY-SELFTEST OK' || { echo "FAIL: CONTROL — the body comparison cannot tell a back-link from a body edit: $out"; exit 1; }
out=$(python target/s8a-ctl/fm_body.py "$S8A_BASE" "$@" 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE '^FM-BODY compared=30 differ=0 ' || { echo "FAIL: an accepted BODY in 0132..0161 differs from S8A_BASE (rc=$rc) — the erratum lists them, it does not rewrite them: $out"; exit 1; }

# ---- (6) CHECKPOINT 5'S NEW BAN (the record's rule), code lines only, strings blanked, NO ::ChangeSummary escape.
# [amended 2026-09-25, s8a fix round 3] Stated limit, corrected: the ban reads one PHYSICAL line at a time, and rustfmt
# wraps any definition wider than its width, so fmt --check does NOT hold the tree to a one-line form for those. Measured
# 2026-09-25: rustfmt writes a long alias as `pub type X =` then `    Result<…ChangeSummary, …>;`, and a long impl
# header with `    for ChangeSummary` on its own line; the ban sees neither, alias or impl. A line-joined (statement)
# reading is the fix, owed and not taken here.
# [amended 2026-09-25, s8a fix round 1] The rename arm fired only on a capital or _ after `as`, so a lowercase
# renaming import (`use …::ChangeSummary as summary;`, `{ChangeSummary as cs, …}`) passed, and rustc -D warnings
# accepts one. Two arms are added: a renaming import on a `use` line, whatever the alias's case, and a line made only
# of import-list items one of which renames …ChangeSummary. Controls 20-23 fire; 19 and 24 (numeric casts) are
# admitted. Stated limit: a lowercase cast that is the whole of a list-item line (`X::ChangeSummary as u8,`) fires.
# [amended 2026-09-25, s8a fix round 13] The impl arm took only a (path-qualified) name straight after `for`,
# so `impl Render for &ChangeSummary`, `for &'a rk_ports::toolchain::ChangeSummary`, `for Box<ChangeSummary>` and
# `for Option<…BinaryChangeSummary>` passed, and each compiles outside rk-ports under rustc -D warnings. The arm
# now takes any text between `for` and the name on the line; no admitted control carries `impl`. Controls 27 and 28
# (a reference and a Box<…>) fire. The one-physical-line limit above is unchanged.
# [amended 2026-09-25, s8a fix round 14] The definition arm and the `impl … for` arm matched the name as
# [A-Za-z_]*ChangeSummary, so a raw identifier (`pub struct r#ChangeSummary;`, a type literally named
# ChangeSummary) and a digit in the prefix (`pub struct V2ChangeSummary;`, `impl Clone for Sha1ChangeSummary {`)
# passed, and each compiles under rustc -D warnings. Both arms now take [A-Za-z0-9_]*, and the definition arm an
# optional r# prefix. Controls 29, 30 and 31 fire.
CS_BAN='\b(struct|enum|type|trait|union)[[:space:]]+(r#)?[A-Za-z0-9_]*ChangeSummary\b|\btype[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[^=]*=.*\b[A-Za-z_]*ChangeSummary\b|\bimpl\b.*\bfor[[:space:]]+.*\b[A-Za-z0-9_]*ChangeSummary\b|\bimpl(<[^>]*>)?[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b|\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Z_]|\buse\b[^;]*\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Za-z_]|^[^:]+:[0-9]+:[[:space:]]*([A-Za-z_][A-Za-z0-9_:]*([[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*)?[[:space:]]*,[[:space:]]*)*([A-Za-z_][A-Za-z0-9_]*::)*[{]?[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*(,.*)?$'
# [amended 2026-09-25, s8a fix round 5] The string blank was `s/"([^"\]|\.)*"/""/g`: in ERE a backslash inside a
# bracket is literal and `\.` is a literal dot, so a string holding a backslash escape ("a\n") was never blanked and
# its closing quote paired with the NEXT string on the line, blanking the code between (a ChangeSummary impl, a
# diff_tree_to_tree call). It is now escape-aware, `[^"\\]|\\.` (a non-quote non-backslash, or a backslash and any
# byte), as sealed.py's \\[\s\S] has been since fix round 1. Planted controls carry a backslash-escaped string.
cs_filter() { grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\\]|\\.)*"/""/g' | grep -E "$CS_BAN"; }
raw=$(grep -rn 'ChangeSummary' crates/ --include='*.rs'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the ChangeSummary search found nothing or could not run (rc=$rc) — rk-ports' own enum must be seen"; exit 1; }
hits=$(printf '%s\n' "$raw" | cs_filter)
[ -z "$hits" ] || { echo "FAIL: a ChangeSummary definition, alias or impl outside rk-ports:"; printf '%s\n' "$hits"; exit 1; }
for ctl in 'F|x.rs:1:pub struct ChangeSummary;' 'F|x.rs:2:pub enum TextChangeSummary { A }' 'F|x.rs:3:type Cs = rk_ports::toolchain::ChangeSummary;' 'F|x.rs:4:impl Clone for ChangeSummary {' 'F|x.rs:5:impl rk_ports::toolchain::ChangeSummary {' 'F|x.rs:6:impl<T> From<T> for rk_ports::toolchain::BinaryChangeSummary {' 'F|x.rs:7:    let s = "x"; impl Copy for ChangeSummary {}' 'P|x.rs:8:use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};' 'P|x.rs:9:    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {' 'P|x.rs:10:    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {' 'P|x.rs:11:            Self::ChangeSummary => "change_summary",' 'P|x.rs:12:    ChangeSummary,' 'P|x.rs:13:        Ok(ChangeSummary::Text(text))' 'P|x.rs:14:    // impl Clone for ChangeSummary {}' 'P|x.rs:15:    let k = "impl Clone for ChangeSummary";' 'F|x.rs:16:use rk_ports::toolchain::ChangeSummary as Cs;' 'F|x.rs:17:    ChangeSummary as Summary,' 'F|x.rs:18:use rk_ports::toolchain::{ChangeSummary as _, Toolchain};' 'P|x.rs:19:    let n = Capability::ChangeSummary as usize;' 'F|x.rs:20:use rk_ports::toolchain::ChangeSummary as summary;' 'F|x.rs:21:use rk_ports::toolchain::{ChangeSummary as cs, Toolchain};' 'F|x.rs:22:    ChangeSummary as summary,' 'F|x.rs:23:    toolchain::{ChangeSummary as cs, Toolchain},' 'P|x.rs:24:        let n = kind as usize + Capability::ChangeSummary as usize;' 'F|x.rs:25:    const A: &str = "a\n"; impl Clone for ChangeSummary { fn n() -> &str { "z" } }' 'P|x.rs:26:    let k = "a\"impl Clone for ChangeSummary\"";' 'F|x.rs:27:impl Render for &ChangeSummary {' 'F|x.rs:28:impl Render for Box<rk_ports::toolchain::ChangeSummary> {' 'F|x.rs:29:pub struct r#ChangeSummary;' 'F|x.rs:30:pub struct V2ChangeSummary;' 'F|x.rs:31:impl Clone for Sha1ChangeSummary {'; do
  want=${ctl%%|*}; line=${ctl#*|}; got=$(printf '%s\n' "$line" | cs_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: control must FIRE and did not — the ban is vacuous for: $line"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: control must be ADMITTED and fired — the ban blocks what the record licenses: $line"; exit 1; }; fi
done
kraw=$(grep -rn 'ChangeSummary' crates/rk-kernel/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-kernel search could not run (rc=$rc)"; exit 1; }
# *(dated 2026-10-03, s8i build, R2FIX-8i)* OWED, NOT CHANGED HERE (s8i refuting rounds 1 and 2): k_filter blanks string
# literals but not char literals. A '"' char literal pairs its quote with the next string's and hides a type use between
# them. s8i's Checkpoint 8i (8) now blanks char literals first (s8i R1FIX-8i). None of the kernel's lines naming
# ChangeSummary carries a '"' char literal [measured 2026-10-03: grep -rn 'ChangeSummary' crates/rk-kernel/src | grep -cF
# with that literal -> 0 of 4], so this ban passes as it stands. Changing this guard is a change to Checkpoint 8a and
# needs its re-run.
k_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\\]|\\.)*"/""/g' | grep -E '\bChangeSummary\b'; }
khits=$(printf '%s\n' "$kraw" | k_filter)
[ -z "$khits" ] || { echo "FAIL: rk-kernel production code names the ChangeSummary type (ADR-0135's surviving forbids):"; printf '%s\n' "$khits"; exit 1; }
kctl=$(printf 'crates/rk-kernel/src/x.rs:1:fn f(s: &str) -> Option<ChangeSummary> { None }\n' | k_filter)
[ -n "$kctl" ] || { echo "FAIL: the rk-kernel ban's control did not fire"; exit 1; }
kctl=$(printf '%s\n' 'crates/rk-kernel/src/x.rs:2:    let r = check("a\n", ChangeSummary::Text(t), "z");' | k_filter)
[ -n "$kctl" ] || { echo "FAIL: the rk-kernel ban's control did not fire past a backslash-escaped string"; exit 1; }
nenum=$(grep -cE '^pub enum ChangeSummary[[:space:]]*\{' crates/rk-ports/src/toolchain.rs)
[ "$nenum" -eq 1 ] || { echo "FAIL: $nenum 'pub enum ChangeSummary {' in rk-ports, expected 1 — the split did not land"; exit 1; }
# The sealed-slot pins below (nbin, npub) are SUPERSEDED BY NAME at s8i when S-BIN finalises Binary (§8a.2 item 1).
# *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* Superseded now: S-BIN has reported (F43, proposed) and ADR-0199 (proposed) opens the slot. The
# asserting lines of the no-pub-field pin and of the sealed_open floor are struck below (each `# ~~…~~`); their readers
# stay written. The record's construction rule replaces them after the sealed_open block (s8i section 8i.8).
# [amended 2026-09-24, s8a D21-47] Read over CODE only (comments stripped) and over BOTH field-list forms, tuple (…)
# and brace { … }: the old grep counted a commented-out declaration and missed a brace struct's pub field. These are
# floors beneath the trybuild case (§8a.9), which is the behavioural pin.
# [amended 2026-09-25, s8a fix round 6] ONE left-to-right pass strips comments and blanks strings, so a `//`
# inside a string (a doc URL) no longer truncates the code after it; and the pub counter skips only a real
# restriction, pub(crate|self|super|in …), where `(?!\s*\()` also skipped `pub ()` and `pub (u8, u8)` — the
# direct way to open the slot read pubfields=0. Four more controls pin both; Checkpoint 5's copy carries the same reader.
cat > target/s8a-ctl/sealed.py <<'PY'
import re, sys
src = open(sys.argv[1], encoding="utf-8").read().replace(chr(13), "")
src = re.sub(r'//[^\n]*|/\*.*?\*/|"(?:[^"\\]|\\[\s\S])*"', lambda m: '""' if m.group(0)[0] == '"' else '', src, flags=re.S)
decls = list(re.finditer(r"\bstruct\s+BinaryChangeSummary\b", src))
pub = 0
for m in decls:
    i = m.end()
    if src[i:].lstrip().startswith("<"):
        i = src.index(">", i) + 1
    rest = src[i:].lstrip()
    if rest and rest[0] in "({":
        close, depth, j = {"(": ")", "{": "}"}[rest[0]], 0, 0
        for j, ch in enumerate(rest):
            depth += ch in "({[" and 1 or 0
            depth -= ch in ")}]" and 1 or 0
            if depth == 0:
                break
        pub += len(re.findall(r"\bpub\b(?!\s*\(\s*(?:(?:crate|self|super)\s*\)|in\s))", rest[1:j]))
print("SEALED decls=%d pubfields=%d" % (len(decls), pub))
PY
sealed() { python target/s8a-ctl/sealed.py "$1" 2>&1; }
# ~~out=$(sealed crates/rk-ports/src/toolchain.rs); rc=$?~~
# ~~[ "$rc" -eq 0 ] || { echo "FAIL: the sealed-slot reader could not run (rc=$rc): $out"; exit 1; }~~
# ~~[ "$out" = "SEALED decls=1 pubfields=0" ] || { echo "FAIL: rk-ports' sealed Binary payload reads '$out', expected one declaration with no pub field — none means the pin passes over nothing, a pub field means it is constructible"; exit 1; }~~
# ~~for ctl in 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary { pub bytes: u64 }' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub u64);' 'SEALED decls=1 pubfields=0|// pub struct BinaryChangeSummary(pub u64);  pub struct BinaryChangeSummary(pub(crate) ());' 'SEALED decls=1 pubfields=0|pub struct BinaryChangeSummary { bytes: u64, note: Vec<u8> }' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub ());' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub (u8, u8));' 'SEALED decls=1 pubfields=0|pub struct BinaryChangeSummary(pub(in crate::toolchain) ());' 'SEALED decls=1 pubfields=1|#[doc = "spec: https://git-lfs.com"] pub struct BinaryChangeSummary { pub bytes: () }'; do~~
# ~~  printf '%s\n' "${ctl#*|}" | sed 's/  pub struct/\npub struct/' > target/s8a-ctl/sealed_ctl.rs || { echo "FAIL: cannot write a sealed-slot control"; exit 1; }~~
# ~~  out=$(sealed target/s8a-ctl/sealed_ctl.rs); rc=$?~~
# ~~  [ "$rc" -eq 0 ] && [ "$out" = "${ctl%%|*}" ] || { echo "FAIL: CONTROL — the sealed-slot reader read '$out' for: ${ctl#*|} (want ${ctl%%|*})"; exit 1; }~~
# ~~done~~
# [amended 2026-09-25, s8a fix round 1] A fifth sealed-slot control: a line-continued string BEFORE the declaration,
# with a later string. Under the old string blank (\\. for an escape) the quotes re-pair across the declaration and
# the reader reports decls=0, losing the pub field that makes the payload constructible; this pins BOTH copies of
# the reader (Checkpoint 8a guard (6) and Checkpoint 5), which cs_cont.rs below does not reach.
# [amended 2026-09-25, s8a fix round 10] Since fix rounds 6 and 7 the reader is ONE re.sub pass under re.S, and
# under re.S `.` matches a newline, so the escape arms \\. and \\[\s\S] are the same pattern: reverting the
# blank alone leaves this control GREEN (measured: \\. with re.S reads decls=1 pubfields=1; [\s\S] without
# re.S reads the same). It goes red only when the escape arm is \\. AND re.S is dropped (decls=0 pubfields=0).
# So it pins that pair, not the blank; no input can tell \\. from \\[\s\S] under re.S, so no single-point
# control on the escape arm exists.
# ~~printf '%s\n' '#[error("a message \' '   continued")]' 'pub struct BinaryChangeSummary(pub u64);' 'const LATER: &str = "later";' > target/s8a-ctl/sealed_cont.rs || { echo "FAIL: cannot write the continued-string sealed-slot control"; exit 1; }~~
# ~~out=$(sealed target/s8a-ctl/sealed_cont.rs); rc=$?~~
# ~~[ "$rc" -eq 0 ] && [ "$out" = "SEALED decls=1 pubfields=1" ] || { echo "FAIL: CONTROL — a line-continued string before the declaration hid it from the sealed-slot reader: '$out' (want SEALED decls=1 pubfields=1)"; exit 1; }~~
# [amended 2026-09-25, s8a fix round 7] The trybuild case tried only the private tuple constructor, and rustc's
# E0423 text does not move when rk-ports adds a Default, a From, a Deserialize or a pub const, so each opened the slot
# with every pin green. The trybuild case now pins Default, From, `new` and Deserialize as their own errors; no
# compile-fail line can name every const, so this floor reads rk-ports' src (code only, one pass as sealed.py): no
# impl block for the payload but `impl serde::Serialize for BinaryChangeSummary`, and no `BinaryChangeSummary(`
# construction but the declaration. Superseded by name at s8i with the rest of the seal.
# [amended 2026-09-25, s8a fix round 8] Named as the third superseded pin in the header, section 8a.2 item 1,
# section 8a.9, ADR-0165 decision 7 and s8i's dated note, so s8i's amendment of this block removes it too.
# [amended 2026-09-25, s8a fix round 9] The floor counted only an impl header that spells BinaryChangeSummary, so an
# inherent impl through `type Bcs = BinaryChangeSummary;` or through `use super::BinaryChangeSummary as B;` read
# OPEN impls=0 ctors=0 and still opened the payload across the crate boundary (CS_BAN excludes rk-ports). The
# reader now collects, over every file first, each name that stands for the payload (a `type X = ...` alias naming
# it, a `BinaryChangeSummary as X` in a `use`), counts impls and constructions through any of them, and refuses the
# aliases themselves: the floor is OPEN impls=0 ctors=0 aliases=0. Three more controls pin it. Stated limit: an
# associated `type X = BinaryChangeSummary;` counts as an alias too; rk-ports declares none.
# [amended 2026-09-25, s8a fix round 12] A construction counted only when the name was followed by `(`, and Rust
# also builds a tuple struct in brace form, `BinaryChangeSummary { 0: () }`, which rk-ports may write: a pub const
# or an `impl ChangeSummary` fn built so read OPEN impls=0 ctors=0 aliases=0 with the trybuild case green. A name
# followed by `(` or `{` now counts, unless `struct` precedes it or it lies inside an impl header the impl count
# already reads (so `impl serde::Serialize for BinaryChangeSummary {` still reads 0). Two brace controls pin it.
# The output line does not change. Stated limit: a `-> BinaryChangeSummary {` return type counts as a construction.
cat > target/s8a-ctl/sealed_open.py <<'PY'
import re, sys
srcs = []
for f in sys.argv[1:]:
    src = open(f, encoding="utf-8").read().replace(chr(13), "")
    srcs.append(re.sub(r'//[^\n]*|/\*.*?\*/|"(?:[^"\\]|\\[\s\S])*"', lambda m: '""' if m.group(0)[0] == '"' else '', src, flags=re.S))
names, aliases = {"BinaryChangeSummary"}, 0
for src in srcs:
    for m in re.finditer(r"\btype\s+(\w+)[^=;{]*=[^;]*\bBinaryChangeSummary\b", src):
        aliases += 1; names.add(m.group(1))
    for u in re.finditer(r"\buse\b[^;]*;", src):
        for m in re.finditer(r"\bBinaryChangeSummary\s+as\s+(\w+)", u.group(0)):
            aliases += 1; names.add(m.group(1))
alt = "|".join(sorted(names))
impls = ctors = 0
for src in srcs:
    heads = []
    for m in re.finditer(r"\bimpl\b[^{;]*?\b(?:%s)\b[^{;]*\{" % alt, src):
        heads.append((m.start(), m.end()))
        if not re.fullmatch(r"impl\s+serde::Serialize\s+for\s+BinaryChangeSummary\s*\{", m.group(0)):
            impls += 1
    ctors += sum(1 for m in re.finditer(r"(\bstruct\s+)?\b(?:%s)\s*[({]" % alt, src) if not m.group(1) and not any(a <= m.start() < b for a, b in heads))
print("OPEN impls=%d ctors=%d aliases=%d" % (impls, ctors, aliases))
PY
# ~~out=$(python target/s8a-ctl/sealed_open.py $(find crates/rk-ports/src -name '*.rs' | LC_ALL=C sort) 2>&1); rc=$?~~
# ~~[ "$rc" -eq 0 ] && [ "$out" = "OPEN impls=0 ctors=0 aliases=0" ] || { echo "FAIL: rk-ports opens the sealed Binary payload: '$out' (want OPEN impls=0 ctors=0 aliases=0; rc=$rc)"; exit 1; }~~
# ~~for ctl in 'OPEN impls=1 ctors=0 aliases=0|impl BinaryChangeSummary { pub const EMPTY: Self = Self(()); }' 'OPEN impls=1 ctors=0 aliases=0|impl From<u8> for BinaryChangeSummary { fn from(_: u8) -> Self { Self(()) } }' 'OPEN impls=0 ctors=1 aliases=0|pub const SEALED: BinaryChangeSummary = BinaryChangeSummary(());' 'OPEN impls=0 ctors=0 aliases=0|pub struct BinaryChangeSummary(pub(crate) ());' 'OPEN impls=0 ctors=0 aliases=0|impl serde::Serialize for BinaryChangeSummary { fn f() { let s = "BinaryChangeSummary(())"; } }' 'OPEN impls=0 ctors=0 aliases=0|// impl BinaryChangeSummary { pub const EMPTY: Self = Self(()); }' 'OPEN impls=0 ctors=0 aliases=0|    Binary(BinaryChangeSummary),' 'OPEN impls=1 ctors=0 aliases=1|type Bcs = BinaryChangeSummary; impl Bcs { pub const EMPTY: Self = Self(()); }' 'OPEN impls=1 ctors=0 aliases=1|mod open { use super::BinaryChangeSummary as B; impl B { pub const EMPTY: Self = Self(()); } }' 'OPEN impls=0 ctors=1 aliases=1|use super::BinaryChangeSummary as B; pub const SEALED: B = B(());' 'OPEN impls=0 ctors=1 aliases=0|pub const SEALED: BinaryChangeSummary = BinaryChangeSummary { 0: () };' 'OPEN impls=0 ctors=1 aliases=0|impl ChangeSummary { pub fn open() -> Self { Self::Binary(BinaryChangeSummary { 0: () }) } }'; do~~
# ~~  printf '%s\n' "${ctl#*|}" > target/s8a-ctl/sealed_open_ctl.rs || { echo "FAIL: cannot write a sealed-open control"; exit 1; }~~
# ~~  out=$(python target/s8a-ctl/sealed_open.py target/s8a-ctl/sealed_open_ctl.rs 2>&1); rc=$?~~
# ~~  [ "$rc" -eq 0 ] && [ "$out" = "${ctl%%|*}" ] || { echo "FAIL: CONTROL — the sealed-open reader read '$out' for: ${ctl#*|} (want ${ctl%%|*})"; exit 1; }~~
# ~~done~~
# *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* THE RECORD'S CONSTRUCTION RULE replaces the three seal pins struck above (ADR-0199
# decision 2, proposed: "The payload's fields are private. It is constructed only through one public checked
# constructor in rk-ports"; s8i section 8i.8; ADR-0165 decision 7 provides the supersession by name). Measured on the
# new tree, read-only: the struck readers print SEALED decls=1 pubfields=0 (still true) and OPEN impls=1 ctors=0
# aliases=0 (the constructor's impl block, as fix round 8 foresaw). This reader carries sealed.py's one-pass
# comment strip and string blank and sealed_open.py's alias collection, over rk-ports' src (code only), and prints one
# line: one declaration; no pub field (a pub(crate|self|super|in …) restriction is not pub); no derive beyond Debug,
# Clone, PartialEq and Eq (Default and Deserialize are routes); no alias; no trait impl naming the payload but
# serde::Serialize; one inherent impl whose only public associated fn or const yielding Self is `new`; and one literal
# construction, the constructor's own `Self { .. }` (a `-> Self {` return type is not one). The behavioural pin is the
# trybuild case ui::trybuild_pins_the_binary_change_summary_construction_rule, which Checkpoint 8i (8) runs. Fifteen
# controls; the seventh is the old sealed slot itself, which this floor refuses (inherent=0 ctors=none). Stated limits:
# one physical source per file, read as a whole; a constructor spelled through a macro is not seen (the trybuild case
# and Checkpoint 8i (8)'s BIN_RE floor outside rk-ports are the other layers).
cat > target/s8a-ctl/ctor_rule.py <<'PY'
import re, sys
srcs = []
for f in sys.argv[1:]:
    src = open(f, encoding="utf-8").read().replace(chr(13), "")
    srcs.append(re.sub(r'//[^\n]*|/\*.*?\*/|"(?:[^"\\]|\\[\s\S])*"', lambda m: '""' if m.group(0)[0] == '"' else '', src, flags=re.S))
def body(src, i):
    depth = 0
    for j in range(i, len(src)):
        depth += src[j] in "({[" and 1 or 0
        depth -= src[j] in ")}]" and 1 or 0
        if depth == 0:
            return src[i:j + 1]
    return src[i:]
names, aliases = {"BinaryChangeSummary"}, 0
for src in srcs:
    for m in re.finditer(r"\btype\s+(\w+)[^=;{]*=[^;]*\bBinaryChangeSummary\b", src):
        aliases += 1; names.add(m.group(1))
    for u in re.finditer(r"\buse\b[^;]*;", src):
        for m in re.finditer(r"\bBinaryChangeSummary\s+as\s+(\w+)", u.group(0)):
            aliases += 1; names.add(m.group(1))
alt = "|".join(sorted(names))
decls = pubf = traits = inherent = literals = 0
derives, ctors = set(), []
for src in srcs:
    for m in re.finditer(r"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?\bstruct\s+BinaryChangeSummary\b\s*", src):
        decls += 1
        for d in re.findall(r"#\[derive\(([^)]*)\)\]", m.group(1)):
            derives.update(x.strip().split("::")[-1] for x in d.split(",") if x.strip())
        rest = src[m.end():]
        if rest[:1] and rest[:1] in "({":
            pubf += len(re.findall(r"\bpub\b(?!\s*\(\s*(?:(?:crate|self|super)\s*\)|in\s))", body(rest, 0)[1:-1]))
    heads = []
    for m in re.finditer(r"\bimpl\b[^{;]*?\b(?:%s)\b[^{;]*\{" % alt, src):
        heads.append((m.start(), m.end()))
        if re.fullmatch(r"impl\s+serde::Serialize\s+for\s+BinaryChangeSummary\s*\{", m.group(0)):
            continue
        if re.search(r"\bfor\b", m.group(0)):
            traits += 1
            continue
        inherent += 1
        blk = body(src, m.end() - 1)
        for f in re.finditer(r"\bpub\s+(?:const\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\(([^)]*)\)\s*(?:->\s*([^{;]*))?", blk):
            if not re.search(r"\bself\b", f.group(2)) and re.search(r"\b(?:Self|%s)\b" % alt, f.group(3) or ""):
                ctors.append(f.group(1))
        for c in re.finditer(r"\bpub\s+const\s+(\w+)\s*:\s*(?:Self|%s)\b" % alt, blk):
            ctors.append("const:" + c.group(1))
        literals += len(re.findall(r"\bSelf\s*[({]", re.sub(r"->\s*(?:Self|%s)\b" % alt, "-> _", blk)))
    literals += sum(1 for m in re.finditer(r"(\bstruct\s+)?\b(?:%s)\s*[({]" % alt, src) if not m.group(1) and not re.search(r"->\s*$", src[max(0, m.start() - 40):m.start()]) and not any(a <= m.start() < b for a, b in heads))
extra = sorted(derives - {"Debug", "Clone", "PartialEq", "Eq"})
print("RULE decls=%d pubfields=%d derives=%s aliases=%d traits=%d inherent=%d ctors=%s literals=%d" % (decls, pubf, ",".join(extra) or "ok", aliases, traits, inherent, ",".join(ctors) or "none", literals))
PY
out=$(python target/s8a-ctl/ctor_rule.py $(find crates/rk-ports/src -name '*.rs' | LC_ALL=C sort) 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ "$out" = "RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1" ] || { echo "FAIL: rk-ports' Binary payload breaks ADR-0199's construction rule: '$out' (want RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1; rc=$rc)"; exit 1; }
for ctl in 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=1 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { pub assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new,unchecked literals=2|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn unchecked(a: Vec<u8>) -> Self { Self { assets: a } } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=1 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { } impl From<u8> for BinaryChangeSummary { fn from(_: u8) -> Self { Self { assets: Vec::new() } } }' 'RULE decls=1 pubfields=0 derives=Default aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq, Default)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=1 traits=0 inherent=2 ctors=new,const:EMPTY literals=2|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { } type B = BinaryChangeSummary; impl B { pub const EMPTY: Self = Self { assets: Vec::new() }; }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=0 ctors=none literals=0|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary(pub(crate) ()); impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { } // impl BinaryChangeSummary { pub fn open() -> Self { Self { assets: vec![] } } }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=none literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new,empty literals=2|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub const fn empty() -> Self { Self { assets: Vec::new() } } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=2|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { } pub const SEALED: BinaryChangeSummary = BinaryChangeSummary { assets: Vec::new() };' 'RULE decls=1 pubfields=0 derives=Deserialize aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { let s = "pub fn open() -> Self { Self { x } }"; &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=0 traits=0 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { pub(crate) assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { }' 'RULE decls=1 pubfields=0 derives=ok aliases=1 traits=1 inherent=1 ctors=new literals=1|#[derive(Debug, Clone, PartialEq, Eq)] pub struct BinaryChangeSummary { assets: Vec<u8> } impl BinaryChangeSummary { pub fn new(assets: Vec<u8>, d: &str) -> Result<Self, E> { Ok(Self { assets }) } pub fn assets(&self) -> &[u8] { &self.assets } } impl serde::Serialize for BinaryChangeSummary { } use super::BinaryChangeSummary as P; impl<T> From<T> for P { fn from(_: T) -> Self { todo() } }'; do
  printf '%s\n' "${ctl#*|}" > target/s8a-ctl/ctor_rule_ctl.rs || { echo "FAIL: cannot write a construction-rule control"; exit 1; }
  out=$(python target/s8a-ctl/ctor_rule.py target/s8a-ctl/ctor_rule_ctl.rs 2>&1); rc=$?
  [ "$rc" -eq 0 ] && [ "$out" = "${ctl%%|*}" ] || { echo "FAIL: CONTROL — the construction-rule reader read '$out' for: ${ctl#*|} (want ${ctl%%|*})"; exit 1; }
done
# [amended 2026-09-24, s8a D21-47] NO GIT FROM THE ADAPTER, BY ANY ROUTE. The adapter spawns through
# run::run_program(config, dir, tag, program: &Path, args), so a Command::new("git") grep passes a
# run_program(…, Path::new("git"), …) call. The ban is a "git" or "git.exe" string literal on ANY code line of the
# adapter's src. Stated limit: a name assembled at run time (concat!, format!) is not seen, which is why the positive
# assertion below exists: change_summary's own body must call rk-git's range read.
g_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -E '"git(\.exe)?"'; }
graw=$(grep -rnE '"git(\.exe)?"' crates/rk-adapter-toolchain-rust/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the git-literal ban could not run (rc=$rc)"; exit 1; }
ghits=$(printf '%s\n' "$graw" | g_filter)
[ -z "$ghits" ] || { echo "FAIL: the Rust adapter names git as a program — the range is read through rk-git (ADR-0077):"; printf '%s\n' "$ghits"; exit 1; }
for ctl in 'F|x.rs:1:    let r = run::run_program(config, dir, "diff", Path::new("git"), &args)?;' 'F|x.rs:2:    let c = Command::new("git.exe");' 'P|x.rs:3:    // run_program(config, dir, "diff", Path::new("git"), &args) is what this crate must not do' 'P|x.rs:4:            if name == ".git" || name == "target" {'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | g_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: CONTROL — the git-literal ban did not fire on: ${ctl#*|}"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: CONTROL — the git-literal ban fired on: ${ctl#*|}"; exit 1; }; fi
done
# [amended 2026-09-25, s8a fix round 1] The "git" literal ban admitted rk-git's own exec route: the adapter depends
# on rk-git, whose pub git_exec::run / run_with_env spawn git with no "git" literal at the caller, and whose gitops,
# worktree, clone, tree, reconcile and version modules spawn through it. So every code line of the adapter's src that
# names rk_git may name only rk_git::diff (the gix range read, checked below) and rk_git::path (a path type that
# spawns nothing, checked below); any other rk_git item, and any git_exec token, fires.
# [amended 2026-09-25, s8a fix round 15] This allowlist is a time-bound pin owned by s8c: its record V's package
# handler reads the project root's HEAD through rk-git, which neither rk_git::diff nor rk_git::path holds. The s8c
# commit that lands that read amends this line by a dated note (or places the read inside the allowlist) and says which.
# [amended 2026-09-26, s8c build, P5; D21-62 C; record V = ADR-0170 decision 6, proposed] s8c takes the first of the
# two choices the round-15 note offers: the allowlist is AMENDED to admit rk_git::head, the ONE shared read-only HEAD
# read (crates/rk-git/src/head.rs, head_commit, a gix open-at-the-directory read that spawns nothing) through which the
# Rust adapter's sca_scan handler reads each advisory database's commit before and after its tool runs and its package
# handler reads the project root's HEAD. One amendment covers both reads. head.rs joins diff.rs and path.rs under the
# exec-route ban below, and a P control admits the head import while the F controls still fire. s8c §8c.4 records the
# choice. Was: sed -E 's/\brk_git::(diff|path)\b//g'.
r_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/\brk_git::(diff|path|head)\b//g' | grep -E '\brk_git\b|\bgit_exec\b'; }
rraw=$(grep -rnE '\brk_git\b|\bgit_exec\b' crates/rk-adapter-toolchain-rust/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk_git route search could not run (rc=$rc)"; exit 1; }
rhits=$(printf '%s\n' "$rraw" | r_filter)
[ -z "$rhits" ] || { echo "FAIL: the Rust adapter reaches rk-git beyond its range read and path type — a route to git (ADR-0077):"; printf '%s\n' "$rhits"; exit 1; }
for ctl in 'F|x.rs:1:    let o = rk_git::git_exec::run(dir, ["diff", "--name-status", range])?;' 'F|x.rs:2:use rk_git::{diff, git_exec};' 'F|x.rs:3:use rk_git::gitops;' 'F|x.rs:4:    let d = rk_git::diff::range_diff(root, r)?; let o = rk_git::git_exec::run(root, ["x"])?;' 'F|x.rs:5:    let o = git_exec::run_with_env(dir, args, env)?;' 'P|x.rs:6:use rk_git::diff::{ChangeKind as DiffChange, RangeError, range_diff};' 'P|x.rs:7:use rk_git::path::WorktreePath;' 'P|x.rs:8://! `rk_git::git_exec::run` is not called here' 'P|x.rs:9:    let head = rk_git::head::head_commit(root)?;' 'F|x.rs:10:    let head = rk_git::head::head_commit(root)?; let o = rk_git::gitops::status(root)?;'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | r_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: CONTROL — the rk_git route ban did not fire on: ${ctl#*|}"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: CONTROL — the rk_git route ban fired on: ${ctl#*|}"; exit 1; }; fi
done
# [amended 2026-09-25, s8a, at the stage's own build] BOTH string blanks (here and sealed.py above) read a
# backslash-escape as \\[\s\S], not \\.: a Rust string continued with a trailing backslash spans a newline, and
# `.` stopped at it, so the quote pairing shifted and the reader swallowed fn change_summary
# (CS-CALLS bodies=0 over the adapter's ConfigError #[error(...)] strings). A control below pins it: its body
# carries a later string, which is what re-pairs the quotes across the fn under the old reader (measured: old
# rc 1 bodies=0, new rc 0 bodies=1; without the later string the old reader also passes it).
# [amended 2026-09-25, s8a fix round 7] "BOTH string blanks" above was true of the escape but not of the order:
# cs_calls.py still stripped `//` comments BEFORE blanking strings, so a `//` inside a string (a URL, such as the
# git-lfs spec line `version https://git-lfs.github.com/spec/v1`) cut the line and its closing quote, the blank
# re-paired quotes across the file, and a correct adapter read bodies=0 (measured: rc 1, and rk_git_diff=0 with the
# const at module level). It now makes sealed.py's ONE left-to-right pass; a cs_url.rs control pins it.
# [amended 2026-09-25, s8a fix round 10] That one pass runs under re.S, where \\. and \\[\s\S] are the same
# pattern, so the cs_cont.rs control below no longer pins the escape arm "at the stage's own build" says it
# does: reverting the arm alone reads rc 0 bodies=1, dropping re.S alone reads the same, and only both together
# read rc 1 bodies=0. It pins the pair (escape arm \\. and no re.S), not the blank.
# [amended 2026-09-25, s8a fix round 11] The call-site check matched the bare token range_diff( in the body and,
# separately, any rk_git::diff text in the crate, so a same-crate `fn range_diff` shadowing rk-git's (dropped from
# the import, spawning a program named at run time) read CS-CALLS bodies=1 calling=1 rk_git_diff=1, rc 0. The name
# must now BE rk-git's: the adapter's code defines or binds no range_diff (fn, let, static, const, `as range_diff`,
# `range_diff as`), every `use` naming range_diff imports it from rk_git::diff unrenamed, and some such `use` exists
# or the body calls the path-qualified rk_git::diff::range_diff(. The cs_shadow.rs and cs_foreign.rs mutants pin it;
# cs_qual.rs is the path-qualified control. Stated limit: a glob import from another crate that re-exports a
# range_diff is not seen.
cat > target/s8a-ctl/cs_calls.py <<'PY'
import glob, re, sys
files = sys.argv[1:]
code = {}
for f in files:
    s = open(f, encoding="utf-8").read().replace(chr(13), "")
    code[f] = re.sub(r'//[^\n]*|/\*.*?\*/|"(?:[^"\\]|\\[\s\S])*"', lambda m: '""' if m.group(0)[0] == '"' else '', s, flags=re.S)
bodies = []
for f, s in code.items():
    for m in re.finditer(r"\bfn\s+change_summary\s*\(", s):
        i = s.index("{", m.end())
        depth = 0
        for j in range(i, len(s)):
            depth += (s[j] == "{") - (s[j] == "}")
            if depth == 0:
                break
        bodies.append(s[i:j + 1])
imports = any(re.search(r"\brk_git::diff\b", s) for s in code.values())
calls = [b for b in bodies if re.search(r"\brange_diff\s*\(", b)]
local = sum(len(re.findall(r"\bfn\s+range_diff\b|\b(?:let|static|const)\s+(?:mut\s+)?range_diff\b|\bas\s+range_diff\b|\brange_diff\s+as\b", s)) for s in code.values())
uses = [u for s in code.values() for u in re.findall(r"\buse\s+[^;]*;", s) if re.search(r"\brange_diff\b", u)]
foreign = [u for u in uses if not re.match(r"use\s+rk_git::diff::(?:range_diff\s*;|\{[^{}]*\brange_diff\b[^{}]*\}\s*;)", u)]
bound = local == 0 and not foreign and (bool(uses) or any(re.search(r"\brk_git::diff::range_diff\s*\(", b) for b in bodies))
print("CS-CALLS bodies=%d calling=%d rk_git_diff=%d local=%d foreign_use=%d" % (len(bodies), len(calls), int(imports), local, len(foreign)))
sys.exit(0 if len(bodies) == 1 and len(calls) == 1 and imports and bound else 1)
PY
set -- crates/rk-adapter-toolchain-rust/src/*.rs
out=$(python target/s8a-ctl/cs_calls.py "$@" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: RustToolchain::change_summary does not call rk_git::diff::range_diff in its own body ($out) — a ban on spawning git passes on a body that reads nothing"; exit 1; }
printf '%s\n' 'use rk_git::diff::range_diff;' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    // range_diff(&self.root, range)' '    Err(StepError::Unsupported { capability: "range_diff(" })' '}' > target/s8a-ctl/cs_mutant.rs || { echo "FAIL: cannot write the call-site mutant"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_mutant.rs 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: MUTANT — a change_summary whose body names range_diff only in a comment and a string passed the call-site check (rc=$rc): $out"; exit 1; }
printf '%s\n' 'use rk_git::diff::range_diff;' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    let d = range_diff(&self.root, range)?;' '    Ok(ChangeSummary::Text(d.into()))' '}' > target/s8a-ctl/cs_good.rs || { echo "FAIL: cannot write the call-site control"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_good.rs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — the call-site check rejected a body that calls range_diff (rc=$rc): $out"; exit 1; }
printf '%s\n' 'use rk_git::diff::range_diff;' '#[error("a message continued \' '     on the next line")]' 'struct E;' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    let d = range_diff(&self.root, range)?;' '    Ok(ChangeSummary::Text(d.with_note("x")))' '}' > target/s8a-ctl/cs_cont.rs || { echo "FAIL: cannot write the continued-string control"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_cont.rs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — a line-continued Rust string before fn change_summary hid the body from the call-site check (rc=$rc): $out"; exit 1; }
printf '%s\n' 'const LFS_SPEC: &str = "version https://git-lfs.github.com/spec/v1";' 'use rk_git::diff::range_diff;' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    let d = range_diff(&self.root, range)?;' '    Ok(ChangeSummary::Text(d.with_note("x")))' '}' > target/s8a-ctl/cs_url.rs || { echo "FAIL: cannot write the URL-string control"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_url.rs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — a // inside a string before fn change_summary hid the body from the call-site check (rc=$rc): $out"; exit 1; }
printf '%s\n' 'use rk_git::diff::{ChangeKind as DiffChange, RangeError};' 'fn range_diff(root: &Path, range: &str) -> Result<rk_git::diff::RangeDiff, RangeError> {' '    let prog = format!("{}{}", "gi", "t");' '    let _ = std::process::Command::new(prog).arg("diff").arg(range).current_dir(root).output();' '    Err(RangeError::Empty)' '}' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    let d = range_diff(&self.root, range)?;' '    Ok(ChangeSummary::Text(d.into()))' '}' > target/s8a-ctl/cs_shadow.rs || { echo "FAIL: cannot write the shadow mutant"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_shadow.rs 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: MUTANT — a local fn range_diff shadowing rk-git's passed the call-site check (rc=$rc): $out"; exit 1; }
printf '%s\n' 'use rk_git::diff::{ChangeKind as DiffChange, RangeError};' 'use crate::shim::range_diff;' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    let d = range_diff(&self.root, range)?;' '    Ok(ChangeSummary::Text(d.into()))' '}' > target/s8a-ctl/cs_foreign.rs || { echo "FAIL: cannot write the foreign-import mutant"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_foreign.rs 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: MUTANT — a range_diff imported from outside rk_git::diff passed the call-site check (rc=$rc): $out"; exit 1; }
printf '%s\n' 'fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError> {' '    let d = rk_git::diff::range_diff(&self.root, range)?;' '    Ok(ChangeSummary::Text(d.into()))' '}' > target/s8a-ctl/cs_qual.rs || { echo "FAIL: cannot write the path-qualified control"; exit 1; }
out=$(python target/s8a-ctl/cs_calls.py target/s8a-ctl/cs_qual.rs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — the call-site check rejected a path-qualified rk_git::diff::range_diff call (rc=$rc): $out"; exit 1; }
# The range read is a gix read: GitOp gains no range-read variant since S8A_BASE (no exec op added for it) and diff.rs names
# no exec type.
test -f crates/rk-git/src/diff.rs || { echo "FAIL: crates/rk-git/src/diff.rs is absent — the range read has no home"; exit 1; }
ge_base=$(git show "$S8A_BASE:crates/rk-git/src/git_exec.rs"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ge_base" ] || { echo "FAIL: cannot read git_exec.rs at S8A_BASE (rc=$rc)"; exit 1; }
# [amended 2026-09-25, s8a fix round 2] This was byte equality on the whole `pub enum GitOp { … }`. s8c (LsFiles,
# CheckIgnore), s8d-ii (§8d-ii.6) and s8g (Tag, Apply) add variants, and s8e, s8i and s8j re-run this checkpoint whole,
# so from s8c on the pin could never pass again, and no stage amended it (D21-47's time-bound rule). It now pins what it
# means: every S8A_BASE variant is still declared, and no variant added since is named for a diff, a range or an
# object read (Diff, Range, Show, RevList, CatFile). Planted controls fire. [amended 2026-09-29, s8f build: CatFile by
# exact name is admitted as s8f's read variant, `cat-file -s` and `cat-file blob` only; see op_check.]
op_names() { sed -n '/^pub enum GitOp {/,/^}/p' | grep -vE '^[[:space:]]*(//|#)' | grep -oE '^[[:space:]]+[A-Z][A-Za-z0-9]*' | tr -d ' \t' | LC_ALL=C sort -u; }
op_check() {  # $1 = S8A_BASE's variant names, $2 = the names read now; prints what moved, or nothing
  op_gone=$(LC_ALL=C comm -23 <(printf '%s\n' "$1") <(printf '%s\n' "$2"))
  op_added=$(LC_ALL=C comm -13 <(printf '%s\n' "$1") <(printf '%s\n' "$2"))
  [ -z "$op_gone" ] || printf 'removed or renamed since S8A_BASE: %s\n' $op_gone
  # [amended 2026-09-29, s8f build (R1FIX-8f-B-RECORDS), D21-47's time-bound rule: "A time-bound guard in an earlier
  # checkpoint is amended by the stage that moves its premise, in the same commit."] s8f adds GitOp::CatFile, which
  # §8f.4 and ADR-0188 prescribe ("`cat-file -s` takes a read variant, `GitOp::CatFile`") and §8f.6 item 4 also uses for
  # the run ref's blob bytes (`cat-file blob`). Neither is a diff or range read, and a read is not a supersession
  # (D21-47), so exactly the name CatFile is admitted; any other catfile-named variant (CatFileBatch, ...) still fires.
  # The line read: printf '%s\n' "$op_added" | grep -iE 'diff|range|show|revlist|catfile' | sed 's/^/added, named for a range read: /'
  printf '%s\n' "$op_added" | grep -iE 'diff|range|show|revlist|catfile' | grep -vxF 'CatFile' | sed 's/^/added, named for a range read: /'
  return 0
}
op_base=$(printf '%s\n' "$ge_base" | tr -d '\r' | op_names)
op_now=$(tr -d '\r' < crates/rk-git/src/git_exec.rs | op_names)
[ -n "$op_base" ] && [ -n "$op_now" ] || { echo "FAIL: GitOp's variants could not be read at S8A_BASE or now"; exit 1; }
op_bad=$(op_check "$op_base" "$op_now")
[ -z "$op_bad" ] || { echo "FAIL: GitOp moved since S8A_BASE — a range read on the exec wrapper moves ADR-0077's read path:"; printf '%s\n' "$op_bad"; exit 1; }
[ -z "$(op_check "$op_base" "$(printf '%s\n' $op_base LsFiles CheckIgnore Tag Apply | LC_ALL=C sort -u)")" ] || { echo "FAIL: CONTROL — the GitOp pin refused the later stages' planned variants"; exit 1; }
[ -n "$(op_check "$op_base" "$(printf '%s\n' $op_base RangeDiff | LC_ALL=C sort -u)")" ] || { echo "FAIL: CONTROL — the GitOp pin passed an added RangeDiff variant"; exit 1; }
[ -n "$(op_check "$op_base" "$(printf '%s\n' "$op_base" | grep -vx Log)")" ] || { echo "FAIL: CONTROL — the GitOp pin passed a removed S8A_BASE variant"; exit 1; }
# [added 2026-09-29, s8f build (R1FIX-8f-B-RECORDS), D21-47's time-bound rule] CatFile is admitted by exact name only,
# and only in its two prescribed forms: every "cat-file" argument list on a code line of crates/*/src is `-s` (ADR-0188
# decision 5's size read) or `blob` (§8f.6 item 4's run-ref bytes). No `-p`, `--batch` or range form. Planted controls fire.
[ -n "$(op_check "$op_base" "$(printf '%s\n' $op_base CatFileBatch | LC_ALL=C sort -u)")" ] || { echo "FAIL: CONTROL — the GitOp pin admitted CatFileBatch, not CatFile alone"; exit 1; }
[ -z "$(op_check "$op_base" "$(printf '%s\n' $op_base CatFile | LC_ALL=C sort -u)")" ] || { echo "FAIL: CONTROL — the GitOp pin refused CatFile, which s8f admits"; exit 1; }
cf_forms() { grep -vE '^[^:]*:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -oE '\["cat-file", "[^"]*"' | sed -E 's/^\["cat-file", "//; s/"$//' | grep -vxE -- '-s|blob'; return 0; }
cf_bad=$(grep -rnE '\["cat-file",' crates/*/src --include=*.rs | cf_forms)
[ -z "$cf_bad" ] || { echo "FAIL: a GitOp::CatFile call on a code line reads a form other than -s or blob:"; printf '%s\n' "$cf_bad"; exit 1; }
cf_n=$(grep -rnE '\["cat-file", "(-s|blob)"' crates/*/src --include=*.rs | wc -l)
[ "$cf_n" -ge 1 ] || { echo "FAIL: no cat-file call found in crates/*/src — the form pin reads nothing"; exit 1; }
[ -n "$(printf '%s\n' 'crates/x/src/y.rs:9:        ["cat-file", "-p", spec],' | cf_forms)" ] || { echo "FAIL: CONTROL — the cat-file form pin passed a -p read"; exit 1; }
[ -z "$(printf '%s\n' 'crates/x/src/y.rs:9:        ["cat-file", "blob", oid],' 'crates/x/src/y.rs:10:   &["cat-file", "-s", oid]' | cf_forms)" ] || { echo "FAIL: CONTROL — the cat-file form pin refused its two admitted forms"; exit 1; }
# [amended 2026-09-25, s8a fix round 1] The ban named only GitOp, GitExec and Command::new, so a diff.rs calling
# rk-git's free git_exec::run / run_with_env (neither takes a GitOp) passed all three checks, and the
# diff_tree_to_tree presence grep passed on diff.rs's own //! doc lines (and on a format! string). Now: diff.rs and path.rs (the adapter's
# other rk-git import, above) name no exec type, no git_exec item and no crate:: or super:: item except
# crate::error; diff_tree_to_tree is read on CODE lines only, strings blanked; planted controls fire.
# [amended 2026-09-25, s8a fix round 2] An aliased std Command (`use std::process::Command as Proc;`, then
# `Proc::new("git")`) and gix's own spawner (`gix::command::prepare("git")`) passed the ban. It now also names
# std::process, process::Command, gix::command and a "git" or "git.exe" literal, with planted controls.
# [amended 2026-09-25, s8a fix round 11] A nested-brace import that rustfmt keeps, `use std::{` / `io,` /
# `process::{Command as C, Stdio},` / `};` then `C::new(prog)`, names none of those tokens, so it passed. dx_pat
# gains a `process::{` arm, which fires on the import line; the `C::new(prog)` call line itself is still not seen.
dx_pat='\b(GitOp|GitExec|git_exec)\b|Command::new|\b(crate|super)::|\bstd::process\b|\bprocess::Command\b|\bprocess::\{|\bgix::command\b|"git(\.exe)?"'
dx_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | sed -E 's/\bcrate::error\b//g' | grep -E "$dx_pat"; }
# [amended 2026-09-26, s8c build, P5] head.rs, the shared HEAD read the rk_git allowlist above now admits, is held to
# the same exec-route ban as diff.rs and path.rs.
dx=$(grep -nHE "$dx_pat" crates/rk-git/src/diff.rs crates/rk-git/src/path.rs crates/rk-git/src/head.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the exec-type search over diff.rs and path.rs could not run (rc=$rc)"; exit 1; }
dxc=$(printf '%s\n' "$dx" | dx_filter)
[ -z "$dxc" ] || { echo "FAIL: rk-git's range read or path type reaches an exec route — the read is a gix read (ADR-0077):"; printf '%s\n' "$dxc"; exit 1; }
for ctl in 'F|d.rs:1:    let out = git_exec::run(root, ["diff", "--name-status", range])?;' 'F|d.rs:2:use crate::git_exec;' 'F|d.rs:3:    let o = crate::gitops::status(root)?;' 'F|d.rs:4:use super::worktree;' 'P|d.rs:5:use crate::error::GitError;' 'P|d.rs:6://! `git_exec::run` is not how this reads' 'F|d.rs:7:use std::process::Command as Proc;' 'F|d.rs:8:    let o = Proc::new("git").arg("diff").output()?;' 'F|d.rs:9:    let _ = gix::command::prepare(prog).spawn();' 'F|d.rs:10:use std::process;' 'F|d.rs:11:    let c = process::Command::new(prog);' 'P|d.rs:12:    let first = "version https://git-lfs.github.com/spec/v1";' 'P|d.rs:13://! nothing here spawns `git` through std::process' 'F|d.rs:14:    process::{Command as C, Stdio},' 'F|d.rs:15:use std::{io, process::{Command as C, Stdio}};'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | dx_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: CONTROL — the exec-route ban over diff.rs did not fire on: ${ctl#*|}"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: CONTROL — the exec-route ban over diff.rs fired on: ${ctl#*|}"; exit 1; }; fi
done
# [amended 2026-09-25, s8a fix round 5] The string blank was `s/"([^"\]|\.)*"/""/g`: in ERE a backslash inside a
# bracket is literal and `\.` is a literal dot, so a string holding a backslash escape ("a\n") was never blanked and
# its closing quote paired with the NEXT string on the line, blanking the code between (a ChangeSummary impl, a
# diff_tree_to_tree call). It is now escape-aware, `[^"\\]|\\.` (a non-quote non-backslash, or a backslash and any
# byte), as sealed.py's \\[\s\S] has been since fix round 1. Planted controls carry a backslash-escaped string.
# [amended 2026-09-25, s8a fix round 7] The read no longer calls Repository::diff_tree_to_tree: that builds a blob
# resource cache from the checkout's .git/index even with rename tracking off, so an index under 20 bytes panicked
# inside gix-index and a damaged one failed a committed range. diff.rs calls gix-diff's tree diff, `gix::diff::tree(`,
# over the two trees' bytes, and this presence check follows it as §8a.2 permits. A positive control is added.
dtt() { grep -nE '\bgix::diff::tree\(' | grep -vE '^[0-9]+:[[:space:]]*//' | sed -E 's/"([^"\\]|\\.)*"/""/g' | grep -qE '\bgix::diff::tree\('; }
dtt < crates/rk-git/src/diff.rs || { echo "FAIL: crates/rk-git/src/diff.rs does not call gix's tree diff (gix::diff::tree) on a code line"; exit 1; }
printf '%s\n' '//! 4. `gix::diff::tree(` over the two trees gives the changed entries.' '    let d = format!("gix::diff::tree( {e}");' '    let d = format!("gix::diff::tree( {e}\n");' '    let r = gix::diff::tree::Recorder::default();' | dtt && { echo "FAIL: CONTROL — the tree-diff check passed on a doc comment, a string and a Recorder alone"; exit 1; }
printf '%s\n' '    gix::diff::tree(' | dtt || { echo "FAIL: CONTROL — the tree-diff check did not pass on a real call line"; exit 1; }

# ---- (7) THE NAMED TESTS, per crate, each `... ok` and an exact count.
run_named() {  # $1 = package, $2 = exact count, then the module-qualified names
  pkg=$1; want=$2; shift 2
  out=$(nodispatch cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $pkg's named selection rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted or not selected"; exit 1; }; done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $pkg's selection did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
  nd_check "$pkg's named selection" || exit 1
}
# *(dated 2026-10-14, s8i build, S8I-C8-CP8I)* The superseded sealed-slot pin is replaced by name (§8a.2 item 1; s8i §8i.8;
# ADR-0199 decision 2, proposed): S8I-C4-PORTS removed binary_arm_is_not_constructible_outside_rk_ports, so this line
# failed 'renamed, deleted or not selected' (measured red by S8I-C8-CP8I re-deriving s8i's (11) on the merged tree).
# Its successor is the recorded-constructor test; the count stays 3. Struck:
# ~~run_named rk-ports 3 change_summary::text_variant_serialises_sha_diff_and_manifest change_summary::binary_arm_is_not_constructible_outside_rk_ports fake_behaviour::toolchain_fake_classifies_binary~~
run_named rk-ports 3 change_summary::text_variant_serialises_sha_diff_and_manifest change_summary::binary_is_constructible_only_through_its_recorded_constructor fake_behaviour::toolchain_fake_classifies_binary
# [amended 2026-09-25, s8a fix round 1] rk-git 3 -> 5 and the Rust adapter 5 -> 6: the binary mark, the
# no-final-newline marker and the adapter's refusal of a binary blob the working tree does not hold (§8a.9).
run_named rk-git 5 diff::range_diff_lists_every_changed_path_with_its_change_kind diff::range_diff_refuses_an_unresolvable_range diff::range_diff_marks_an_lfs_pointer_path diff::range_diff_marks_a_binary_blob_at_either_end diff::range_diff_marks_a_missing_final_newline
# [amended 2026-09-25, s8a, D21-51] the Rust adapter 6 -> 8: classify::classify_is_invariant_to_a_cargo_manifest and
# classify::classify_source_spawns_nothing (the code-only no-spawn lint over src/classify.rs) pin ADR-0165 decision 9's
# settlement that classify is a function of the path and its bytes. The Text arm's git-as-authority case and the
# single-revision named refusal are folded into change_summary_text_variant_from_git_diff and
# change_summary_refuses_an_empty_range, so those names do not move.
run_named rk-adapter-toolchain-rust 8 change_summary::change_summary_text_variant_from_git_diff change_summary::change_summary_refuses_a_range_carrying_a_binary_path change_summary::change_summary_refuses_an_empty_range change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path change_summary::change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold registry::ac489_capability_registry_dated classify::classify_is_invariant_to_a_cargo_manifest classify::classify_source_spawns_nothing
run_named rk-adapter-toolchain-node 1 registry::ac489_capability_registry_dated
# [amended 2026-09-25, s8a fix round 3] rk-kernel 4 -> 5: D21-50's unfeatured byte pin is named, so deleting or ignoring
# it turns this guard red; guard (10)'s floor sits below the suite and would not.
run_named rk-kernel 5 conformance::conformance_every_step_kind_has_an_executor conformance::conformance_every_port_has_a_call_site probe_differential::grammar_agrees_with_the_s_grammar_probe on_false::on_false_closed_set recorded_log::the_recording_sources_are_the_committed_logs_inputs_byte_for_byte
run_named redkiln-migrate 1 output::ac475_zero_change_summary_values_in_output
# PINNED ARMED, and TIME-BOUND (D21-47; cross-file item 2): s8e's Slice-Verdict commit un-ignores this lint and, in that
# same commit, amends THIS guard by dated line to "1 passed" with the ARMED reason absent and lists 8a as provisional.
# Until then it stays pinned; a guard conditional on the #[ignore] would pass whether or not the lint was discharged.
# [amended 2026-09-28, s8e build, package S8E-P7; s8e section 8e.13] DISCHARGED: the three lines that expected the lint
# ignored and ARMED, its reason naming s8e, and 0 passed with 1 ignored are REPLACED (a struck line in this bash block
# would still carry the string Checkpoint 8e's guard (10h) asserts absent). Now: the lint's ok line, exactly 1 passed with
# 0 ignored, and the ARMED string absent from conformance.rs with grep's rc exactly 1.
out=$(nodispatch cargo test -p rk-kernel --test it -- conformance::conformance_every_trailer_key_has_a_producer --exact 2>&1); rc=$?
nd_check "the trailer lint's selection" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: the discharged trailer lint's selection rc=$rc"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'test conformance::conformance_every_trailer_key_has_a_producer ... ok' || { echo "FAIL: the trailer lint did not report its ok line"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed; 0 ignored' || { echo "FAIL: the trailer lint's selection is not exactly 1 passed, 0 failed, 0 ignored"; exit 1; }
hits=$(grep -nF 'ARMED: Slice-Verdict has 0 producers' crates/rk-kernel/tests/it/conformance.rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the ARMED reason is still present in conformance.rs, or unreadable (grep rc=$rc)"; printf '%s\n' "$hits"; exit 1; }

# ---- (8) catalog-b REPAIRED, and the probe's three readings at rc 0; the committed JSON is what the probe writes.
grep -qF "kind in [incident, 'security-vuln']" spikes/p9/catalog-b/blameless-postmortem.toml || { echo "FAIL: security-vuln is not quoted"; exit 1; }
braw=$(grep -nE '^on_false[[:space:]]*=[[:space:]]*"' spikes/p9/catalog-b/*.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the bare on_false search could not run (rc=$rc)"; exit 1; }
b_filter() { grep -vE '"(continue|fail)"[[:space:]]*$'; }
[ -z "$(printf '%s\n' "$braw" | b_filter | grep .)" ] || { echo "FAIL: a bare step-name on_false remains in catalog-b"; exit 1; }
[ -n "$(printf 'x.toml:1:on_false = "draft"\n' | b_filter)" ] || { echo "FAIL: the bare on_false control did not fire"; exit 1; }
for want in 'on_false = { to = "draft", max = 2 }' 'on_false = { to = "draft", max = 1 }'; do grep -qxF "$want" spikes/p9/catalog-b/blameless-postmortem.toml || { echo "FAIL: blameless-postmortem lacks: $want"; exit 1; }; done
grep -qxF 'on_false = { to = "assemble", max = 2 }' spikes/p9/catalog-b/api-contract-spec.toml || { echo "FAIL: api-contract-spec's loop-back is not { to, max = 2 }"; exit 1; }
mkdir -p target/s8a-whole && git cat-file blob :spikes/p9/results-s-grammar.json > target/s8a-whole/committed.json || { echo "FAIL: cannot read the staged results-s-grammar.json — run this block after git add"; exit 1; }
out=$(PYTHONIOENCODING=utf-8 python spikes/p9/grammar-probe.py 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: grammar-probe rc=$rc — a clean corpus exits 0"; printf '%s\n' "$out" | grep -E 'FAIL'; exit 1; }
printf '%s\n' "$out" | grep -qxF 'predicates parsed 279/279; failed 0/279' || { echo "FAIL: the parse count is not 279/279"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'mixed unparenthesised and/or (precedence-dependent) 0/279' || { echo "FAIL: the mixed-connective count is not 0/279"; exit 1; }
printf '%s\n' "$out" | grep -qE '^on_false forms 99/99 in the closed set' || { echo "FAIL: the on_false count is not 99/99"; exit 1; }
python -c "import json,sys; a=json.load(open(sys.argv[1],encoding='utf-8')); b=json.load(open(sys.argv[2],encoding='utf-8')); sys.exit(0 if a==b and a['parsed']==279 and a['failed']==[] and a['on_false_ok']==99 else 1)" target/s8a-whole/committed.json spikes/p9/results-s-grammar.json || { echo "FAIL: the probe's JSON differs from the committed one, or does not read 279 / [] / 99"; exit 1; }
# [amended 2026-09-25, s8a fix round 5] The probe wrote results-s-grammar.json through a text-mode open with no newline
# argument, so on Windows the tracked file came back w/crlf on an unpinned path, the state ADR-0164 decision 6 says must
# not exist, and the parsed comparison above cannot see a terminator. grammar-probe.py now writes with newline="\n",
# and the terminator is asserted here, after the probe ran.
geol=$(git ls-files --eol spikes/p9/results-s-grammar.json)
case "$geol" in *w/lf[[:space:]]*) : ;; *) echo "FAIL: the probe left spikes/p9/results-s-grammar.json without w/lf (ADR-0164 decision 6): $geol"; exit 1 ;; esac

# ---- (9) THE CODEX PHASE: the design §10.4 row, and every register site that says Phase 2 carries a dated correction.
row=$(grep -F '| **S-CODEX — harness portability** |' references/2026-09-08-architecture-design.md); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$row" | grep -qF "ADR-$CODEX_ADR" || { echo "FAIL: design §10.4's S-CODEX row carries no dated correction citing ADR-$CODEX_ADR — it already says 'Phase 4' in an older note, so that word proves nothing"; exit 1; }
# [amended 2026-09-25, s8a fix round 4] AC-239's phase clause says "the second Harness adapter is **Phase 2**" and names
# no Codex, so the Codex-only pattern missed it; the pattern reads either wording now (seven lines).
craw=$(grep -nE '(Codex|[Hh]arness adapter)[^|]*\bPhase 2\b' references/2026-09-08-acceptance-criteria.md references/2026-09-08-success-measures.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the Codex-phase register sweep could not run (rc=$rc)"; exit 1; }
c_filter() { grep -vF "ADR-$CODEX_ADR"; }
[ -z "$(printf '%s\n' "$craw" | c_filter | grep .)" ] || { echo "FAIL: a register line still says Codex is Phase 2 with no dated correction citing ADR-$CODEX_ADR"; printf '%s\n' "$craw" | c_filter; exit 1; }
[ -n "$(printf 'x.md:1:the Codex adapter is Phase 2\n' | c_filter)" ] || { echo "FAIL: the Codex sweep's control did not fire"; exit 1; }
printf 'x.md:1:the second Harness adapter is **Phase 2**, behind\n' | grep -qE '(Codex|[Hh]arness adapter)[^|]*\bPhase 2\b' || { echo "FAIL: CONTROL — the Codex sweep cannot see a 'second Harness adapter' phase clause"; exit 1; }
# [added 2026-09-25, s8a fix round 15] The sweep above passes over zero lines, so seven sites rewritten to 'Phase 4' (or
# deleted) instead of noted passed it. It takes the newtype sweep's floor: seven lines, measured 2026-09-25 (§8a.6).
clines=$(printf '%s\n' "$craw" | grep -c .)
[ "$clines" -ge 7 ] || { echo "FAIL: the Codex register sweep read $clines line(s), below the 7 measured 2026-09-25 — a site was rewritten rather than noted"; exit 1; }
# [added 2026-09-24, s8a D21-47] The runbook's Codex exclusion notes (s7a one site, s7b-ii two, s7c one) cite the record.
# [amended 2026-09-25, s8a fix round 2] This counted LINES citing the record, and every note cites it on two lines, so
# one s7b-ii note met that file's floor of 2 and the second site could lose its note unseen. It now counts NOTES (the
# opener 's8a §8a.6, ADR-<id>' is on one line per note), and every 'Phase 2' Codex line must have a note opener within
# the four lines after it. A control file proves a two-line note counts once and an orphaned site is seen.
codex_orphans() { awk -v op="s8a §8a.6, ADR-$CODEX_ADR" '/is Phase 2|\*\*Phase 2\*\* per ADR-0094/ { site[NR] = $0 } index($0, op) { note[NR] = 1 } END { for (n in site) { ok = 0; for (k = n + 0; k <= n + 4; k++) if (k in note) ok = 1; if (!ok) print n ": " site[n] } }' "$1"; }
for spec in 's7a-ports-and-fakes.md 1' 's7b-ii-gates.md 2' 's7c-adapters.md 1'; do
  f=runbook/phase-1/${spec% *}; want=${spec#* }
  test -f "$f" || { echo "FAIL: $f is absent"; exit 1; }
  got=$(grep -cF "s8a §8a.6, ADR-$CODEX_ADR" "$f"); rc=$?
  [ "$rc" -le 1 ] && [ "$got" -ge "$want" ] || { echo "FAIL: $f carries $got note(s) opening 's8a §8a.6, ADR-$CODEX_ADR', expected >= $want — a 'Phase 2' Codex exclusion reason has no dated note"; exit 1; }
  orphan=$(codex_orphans "$f")
  [ -z "$orphan" ] || { echo "FAIL: $f has a 'Phase 2' Codex line with no ADR-$CODEX_ADR note opening within four lines:"; printf '%s\n' "$orphan"; exit 1; }
done
printf '%s\n' 'a rk-adapter-codex is Phase 2, ADR-0094 decision 2).' "  *(Noted 2026-09-24, s8a §8a.6, ADR-$CODEX_ADR: the Codex adapter is Phase 4" "  places it. ADR-$CODEX_ADR supersedes ADR-0094 decision 2 in part" 'b rk-adapter-codex is Phase 2, ADR-0094 decision 2).' > target/s8a-ctl/codex_ctl.md || { echo "FAIL: cannot write the Codex-note control"; exit 1; }
[ "$(grep -cF "s8a §8a.6, ADR-$CODEX_ADR" target/s8a-ctl/codex_ctl.md)" -eq 1 ] || { echo "FAIL: CONTROL — a two-line Codex note did not count as one note"; exit 1; }
[ "$(codex_orphans target/s8a-ctl/codex_ctl.md)" = "4: b rk-adapter-codex is Phase 2, ADR-0094 decision 2)." ] || { echo "FAIL: CONTROL — the Codex-note proximity check did not name the orphaned site (and only it)"; exit 1; }
# [added 2026-09-24, s8a D21-47] Every design and register line saying ChangeSummary is a "non-finalised newtype" carries
# a dated note citing the ChangeSummary record (§8a.2: 7 design + 3 register lines at fc80687, AC-497 among them).
# [amended 2026-09-25, s8a fix round 1] Widened to 'opaque … newtype' within one table cell: three more lines (design's
# BLOCKED field-set row and binary-change row, register area row 3) said "an opaque newtype in rk-ports" and the
# literal 'non-finalised newtype' could not see them. Floor 10 -> 13.
nraw=$(grep -nE 'non-finalised newtype|opaque[^|]*newtype' references/2026-09-08-architecture-design.md references/2026-09-08-acceptance-criteria.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the non-finalised-newtype sweep could not run (rc=$rc)"; exit 1; }
n_filter() { grep -vF "ADR-$CS_ADR"; }
[ -z "$(printf '%s\n' "$nraw" | n_filter | grep .)" ] || { echo "FAIL: a design or register line still calls ChangeSummary a non-finalised newtype with no dated note citing ADR-$CS_ADR:"; printf '%s\n' "$nraw" | n_filter | cut -c1-160; exit 1; }
[ -n "$(printf 'x.md:1:ChangeSummary is an opaque non-finalised newtype\n' | n_filter)" ] || { echo "FAIL: the newtype sweep's control did not fire"; exit 1; }
nlines=$(printf '%s\n' "$nraw" | grep -c .)
[ "$nlines" -ge 13 ] || { echo "FAIL: the newtype sweep read $nlines line(s), below the 13 measured 2026-09-25 (10 at fc80687) — a site was rewritten rather than noted"; exit 1; }

# ---- (9a) CHECKPOINT 7e's BACKTICK DEFECT, AMENDED (§8a.8; D21-46), AND NO RUNBOOK BASH BLOCK EXECUTES A BACKTICK.
# ---- Checkpoint 7e is NOT re-run whole: its block commits and tags. The amendment is verified statically, by a
# ---- no-other-bash-byte comparison with S8A_BASE, and by running the amended line from s7e's own bytes against a
# ---- trap init. No code line below carries a backtick; the scripts build one from chr(96) or printf.
S7E=runbook/phase-1/s7e-pack-authoring.md
test -f "$S7E" || { echo "FAIL: $S7E is absent (Halt 8)"; exit 1; }
mkdir -p target/s8a-ctl || { echo "FAIL: cannot make target/s8a-ctl"; exit 1; }
bt=$(printf '\140'); [ "${#bt}" -eq 1 ] || { echo "FAIL: could not build a backtick for the old-bytes probe"; exit 1; }
grep -qF "7d's ${bt}init${bt} did not run" "$S7E"; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: s7e still carries the backticked init in its FAIL message (grep rc=$rc) — the §8a.8 amendment did not land"; exit 1; }
new7e='test -f .redkiln/config.toml || { echo "FAIL: .redkiln/config.toml absent — Step 7d'"'"'s init did not run, or did not scaffold it (E6)"; exit 1; }'
n=$(grep -cxF "$new7e" "$S7E"); rc=$?
[ "$rc" -le 1 ] && [ "$n" -eq 1 ] || { echo "FAIL: the amended 7e line appears '${n:-none}' times in s7e (rc=$rc), expected exactly 1"; exit 1; }
note7e='D21-46: the two backticks around init are removed'
grep -qF "$note7e" "$S7E" || { echo "FAIL: s7e carries no dated note for the amendment ('$note7e')"; exit 1; }
cat > target/s8a-ctl/s7e_same.py <<'PY'
import subprocess, sys
BT, NL, CR = chr(96), chr(10), chr(13)
F = BT * 3
def blocks(t):
    out, cur = [], None
    for l in t.replace(CR + NL, NL).split(NL):
        if cur is None and l == F + "bash":
            cur = []
        elif cur is not None and l == F:
            out.append(cur)
            cur = None
        elif cur is not None:
            cur.append(l)
    return out
base_ref, path, new, note = sys.argv[1:5]
old = new.replace("7d's init did", "7d's " + BT + "init" + BT + " did")
if old == new:
    sys.exit("S7E-SAME-FAIL: the old line could not be derived from the new one")
r = subprocess.run(["git", "show", base_ref + ":" + path], capture_output=True)
if r.returncode != 0:
    sys.exit("S7E-SAME-FAIL: git show %s:%s rc=%d" % (base_ref, path, r.returncode))
a = blocks(r.stdout.decode("utf-8"))
b = blocks(open(path, encoding="utf-8").read())
if sum(blk.count(old) for blk in a) != 1:
    sys.exit("S7E-SAME-FAIL: S8A_BASE's s7e does not carry the old backticked line exactly once")
if len(a) != len(b):
    sys.exit("S7E-SAME-FAIL: %d bash blocks at S8A_BASE, %d now" % (len(a), len(b)))
a2 = [[new if l == old else l for l in blk] for blk in a]
b2 = [[l for l in blk if note not in l] for blk in b]
bad = [i + 1 for i in range(len(a2)) if a2[i] != b2[i]]
if bad:
    sys.exit("S7E-SAME-FAIL: bash block(s) %s of s7e differ from S8A_BASE beyond the one line and its dated note" % bad)
print("S7E-SAME-OK blocks=%d" % len(a))
PY
test -s target/s8a-ctl/s7e_same.py || { echo "FAIL: the s7e comparison script was not written"; exit 1; }
out=$(python target/s8a-ctl/s7e_same.py "$S8A_BASE" "$S7E" "$new7e" "$note7e" 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE '^S7E-SAME-OK blocks=[1-9]' || { echo "FAIL: s7e's bash moved beyond the §8a.8 amendment (rc=$rc):"; printf '%s\n' "$out"; exit 1; }
out=$(python target/s8a-ctl/s7e_same.py "$S8A_BASE" "$S7E" "$new7e" 'S8A-CONTROL-NO-SUCH-NOTE' 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: CONTROL — the s7e comparison passed with the dated note left in; it cannot see an added line"; exit 1; }
# BEHAVIOURAL: the amended line, from s7e's own bytes, takes its FAIL path and runs nothing. CONTROL: the old line,
# from S8A_BASE's bytes, runs the trap init, or the trap proves nothing.
p7e="$PWD/target/s8a-ctl/bt7e"
rm -rf "$p7e" && mkdir -p "$p7e/bin" "$p7e/cwd" || { echo "FAIL: cannot make the 7e probe dirs"; exit 1; }
printf '#!/bin/sh\n: > "%s/INIT-RAN"\n' "$p7e" > "$p7e/bin/init" && chmod +x "$p7e/bin/init" || { echo "FAIL: cannot write the trap init"; exit 1; }
git show "$S8A_BASE:$S7E" > "$p7e/base.md"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$p7e/base.md" ] || { echo "FAIL: cannot read s7e at S8A_BASE (rc=$rc)"; exit 1; }
grep -F 'test -f .redkiln/config.toml ||' "$p7e/base.md" > "$p7e/old.sh"; rc=$?
[ "$rc" -eq 0 ] && [ "$(grep -c . "$p7e/old.sh")" -eq 1 ] || { echo "FAIL: S8A_BASE's s7e does not carry exactly one config.toml guard line (rc=$rc)"; exit 1; }
grep -xF "$new7e" "$S7E" > "$p7e/new.sh"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$p7e/new.sh" ] || { echo "FAIL: cannot take the amended line from s7e (rc=$rc)"; exit 1; }
out=$(cd "$p7e/cwd" && PATH="$p7e/bin:$PATH" bash "$p7e/old.sh" 2>&1); rc=$?
[ "$rc" -eq 1 ] && [ -f "$p7e/INIT-RAN" ] || { echo "FAIL: CONTROL — the old 7e line did not run the trap init (rc=$rc); the probe cannot see a substitution"; printf '%s\n' "$out"; exit 1; }
rm -f "$p7e/INIT-RAN" || { echo "FAIL: cannot clear the trap marker"; exit 1; }
out=$(cd "$p7e/cwd" && PATH="$p7e/bin:$PATH" bash "$p7e/new.sh" 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the amended 7e line did not take its FAIL path with no config.toml (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
[ ! -e "$p7e/INIT-RAN" ] || { echo "FAIL: the amended 7e FAIL message still ran init as a command"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'FAIL: .redkiln/config.toml absent — Step 7d'"'"'s init did not run, or did not scaffold it (E6)' || { echo "FAIL: the amended 7e line did not print its message verbatim:"; printf '%s\n' "$out"; exit 1; }
# THE CORPUS SCAN: every runbook bash block, code only, quote state carried across lines as bash carries it.
cat > target/s8a-ctl/bt.py <<'PY'
import glob, os, sys
BT, SQ, DQ, BS, NL, CR, TAB = chr(96), chr(39), chr(34), chr(92), chr(10), chr(13), chr(9)
F = BT * 3
def heredoc(line):
    i = line.find("<<")
    while i >= 0:
        rest = line[i + 2:]
        if rest[:1] == "<":
            i = line.find("<<", i + 3)
            continue
        rest = rest.lstrip("-").lstrip()
        if rest[:1] in (SQ, DQ):
            end = rest.find(rest[0], 1)
            if end > 1:
                return rest[1:end], False
        elif rest[:1] == BS and len(rest) > 1:
            return rest[1:].split()[0], False
        else:
            w = ""
            for ch in rest:
                if ch.isalnum() or ch == "_":
                    w += ch
                else:
                    break
            if w:
                return w, True
        i = line.find("<<", i + 2)
    return None
def scan(body):
    state, hd = None, None
    for k, line in enumerate(body, 1):
        if hd is not None:
            if line.strip() == hd[0]:
                hd = None
            elif hd[1] and BT in line:
                yield k
            continue
        j, hit = 0, False
        while j < len(line):
            c = line[j]
            if state is None:
                if c == "#" and (j == 0 or line[j - 1] in " ;(" + TAB):
                    break
                if c == BS:
                    j += 2
                    continue
                if c in (SQ, DQ):
                    state = c
                elif c == BT:
                    hit = True
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
                    hit = True
            j += 1
        if hit:
            yield k
        if state is None:
            h = heredoc(line.split(" #")[0])
            if h:
                hd = h
    if state is not None or hd is not None:
        yield -len(body)
def blocks(text):
    lines = [l.rstrip(CR) for l in text.split(NL)]
    i = 0
    while i < len(lines):
        if lines[i] == F + "bash":
            j = i + 1
            while j < len(lines) and lines[j] != F:
                j += 1
            yield i + 1, lines[i + 1:j]
            i = j + 1
        else:
            i += 1
paths = sys.argv[1:] or sorted(glob.glob("runbook/**/*.md", recursive=True))
n, nb = 0, 0
for path in paths:
    p = path.replace(os.sep, "/")
    for start, body in blocks(open(path, encoding="utf-8").read()):
        nb += 1
        for k in scan(body):
            n += 1
            if k < 0:
                print("BT-OPEN %s:%d the block ends inside a quote or a heredoc" % (p, start - k))
            else:
                print("BT-HIT %s:%d %s" % (p, start + k, body[k - 1].strip()[:160]))
print("BT-SCAN blocks=%d hits=%d" % (nb, n))
sys.exit(1 if n else (2 if nb == 0 else 0))
PY
test -s target/s8a-ctl/bt.py || { echo "FAIL: the backtick scan was not written"; exit 1; }
ctl=target/s8a-ctl/btctl
rm -rf "$ctl" && mkdir -p "$ctl" || { echo "FAIL: cannot make the scan's control dir"; exit 1; }
python - "$ctl" <<'PY' || { echo "FAIL: cannot write the scan's controls"; exit 1; }
import os, sys
B = chr(96); F = B * 3; d = sys.argv[1]
def w(name, lines):
    open(os.path.join(d, name), "w", encoding="utf-8", newline=chr(10)).write(chr(10).join([F + "bash"] + lines + [F, ""]))
w("bad.md", ['echo "FAIL: a planted ' + B + 'init' + B + ' ran"; exit 1'])
w("good.md", ["grep -qF 'a " + B + "quoted" + B + " needle' f", "# a comment naming " + B + "init" + B,
              "x='multi", "line " + B + "literal" + B + "'", "cat > f <<'EOT'", 's = "' + B + 'a' + B + '"', "EOT"])
open(os.path.join(d, "none.md"), "w", encoding="utf-8", newline=chr(10)).write("no fenced block here" + chr(10))
PY
out=$(python target/s8a-ctl/bt.py "$ctl/bad.md" 2>&1); rc=$?
[ "$rc" -eq 1 ] && printf '%s\n' "$out" | grep -qF 'a planted' || { echo "FAIL: CONTROL — the scan passed a planted backtick inside a double-quoted echo (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
out=$(python target/s8a-ctl/bt.py "$p7e/base.md" 2>&1); rc=$?
[ "$rc" -eq 1 ] && printf '%s\n' "$out" | grep -qF 'config.toml absent' || { echo "FAIL: CONTROL — the scan passed S8A_BASE's s7e, which carries the defect (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
out=$(python target/s8a-ctl/bt.py "$ctl/good.md" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — the scan flagged a backtick bash does not execute (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
out=$(python target/s8a-ctl/bt.py "$ctl/none.md" 2>&1); rc=$?
[ "$rc" -eq 2 ] || { echo "FAIL: CONTROL — the scan did not refuse a file with no bash block (rc=$rc); zero blocks would read as a green"; exit 1; }
out=$(python target/s8a-ctl/bt.py 2>&1); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the corpus scan could not run (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
nb=$(printf '%s\n' "$out" | sed -nE 's/^BT-SCAN blocks=([0-9]+) hits=[0-9]+$/\1/p')
[ -n "$nb" ] && [ "$nb" -ge 99 ] || { echo "FAIL: the corpus scan read '${nb:-none}' bash blocks, below the 99 measured at c5d19d7"; exit 1; }
other=$(printf '%s\n' "$out" | grep -E '^BT-(HIT|OPEN) ' | grep -vE '^BT-HIT runbook/phase-1/s7h-lane-completes\.md:[0-9]+ .*binary arm did not pass')
[ -z "$other" ] || { echo "FAIL: a runbook bash block carries a backtick bash would execute (only 7h (10a)'s is admitted, until s8e):"; printf '%s\n' "$other"; exit 1; }
echo "backtick scan: $nb blocks; $(printf '%s\n' "$out" | grep -c '^BT-HIT ') hit(s), all admitted (7h (10a), s8e's to remove)"

# ---- (10) FMT, CLIPPY, AND THE WHOLE SUITE above its floor. Expected value, re-derived at stage open from the last
# ---- whole-suite reading before this stage, ADR-0164's evidence (740 passed / 0 failed / 9 ignored over 44 results,
# ---- 992d257; Checkpoint 7i's block records no suite count): 740 + 9 = 749, 9 being this stage's new tests.
# ---- The 44-result floor stays. [label corrected 2026-09-24, s8a D21-47]
out=$(nodispatch cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nodispatch cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(nodispatch cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the whole suite rc=$rc (a fresh checkout needs pnpm install for the node fixture first)"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
nd_check "fmt, clippy and the whole suite" || exit 1
total=$(printf '%s\n' "$out" | awk '/^test result:/{for(i=1;i<NF;i++) if($(i+1) ~ /^passed;?$/) s+=$i} END{print s+0}')
[ "$total" -ge 749 ] || { echo "FAIL: the whole suite passed $total tests, below the floor of 749"; exit 1; }
nres=$(printf '%s\n' "$out" | grep -cE '^test result:')
[ "$nres" -ge 44 ] || { echo "FAIL: $nres test result lines, below the 44 of the entry reading — a target stopped being built or run"; exit 1; }
printf '%s\n' "$out" | grep -E '^test result:' | grep -vqE ' 0 failed;' && { echo "FAIL: a result line reports failures"; exit 1; }

# ---- (11) THE ENTRY GATE AND FOUR OTHER CHECKPOINTS, WHOLE, FROM THEIR OWN BYTES. The helper copies the ONE bash
# ---- block that carries an anchor and appends a sentinel, so "rc 0 and the sentinel" means the block ran to its end.
# ---- SKIPPED WHOLE under RK_CP_NESTED=1 (D21-46's one re-run convention): a checkpoint that re-runs this one as its
# ---- entry gate makes its own whole runs. Every run below is launched with RK_CP_NESTED=1, with CLAUDE_CONFIG_DIR at
# ---- an empty trap directory that must still be empty afterwards, and with the launch-record variable unset.
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8A (11) SKIPPED: RK_CP_NESTED=1 — this block is another checkpoint's entry gate; its whole re-runs ran at Checkpoint 8a's own close"
else
  extract() {
    python - "$1" "$2" "$3" <<'PY'
import re, sys
fence = chr(96) * 3
pat = re.compile(re.escape(fence) + "bash\n(.*?)" + re.escape(fence), re.S)
text = open(sys.argv[1], encoding="utf-8").read()
found = [m.group(1) for m in pat.finditer(text) if sys.argv[2] in m.group(1)]
if len(found) != 1:
    sys.exit("FAIL: %d bash blocks in %s carry the anchor, expected exactly 1" % (len(found), sys.argv[1]))
open(sys.argv[3], "w", encoding="utf-8", newline="\n").write(found[0] + "\necho CP8A-WHOLE-RUN-SENTINEL\n")
PY
  }
  whole() {  # $1 = label, $2 = stage file, $3 = anchor
    extract "$2" "$3" "target/s8a-whole/$1.sh" || { echo "FAIL: could not extract Checkpoint $1"; exit 1; }
    out=$(nodispatch env RK_CP_NESTED=1 bash "target/s8a-whole/$1.sh" 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $1 run whole rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
    printf '%s\n' "$out" | grep -qxF 'CP8A-WHOLE-RUN-SENTINEL' || { echo "FAIL: Checkpoint $1 did not reach its last line"; exit 1; }
    nd_check "Checkpoint $1's whole run" || exit 1
  }
  common=$(LC_ALL=C git rev-parse --path-format=absolute --git-common-dir); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$common" ] || { echo "FAIL: cannot resolve the git common dir (rc=$rc)"; exit 1; }
  store="$common/redkiln/store.sqlite3"
  cp "$store" target/s8a-whole/store.before.sqlite3 || { echo "FAIL: cannot back up the store before the whole runs"; exit 1; }
  # 7i and 7f can reach `redkiln run`. Neither may dispatch: every shipped card lacks permission_mode, so the card
  # load refuses before any spawn. From s8h on that is false: s8h §8h.14 re-scopes this guard in its own work commit
  # (all cards carry a mode, and the launch-check refusal test runs by name), by D21-47's time-bound rule.
  set -- .redkiln/cards/*.json
  [ "$#" -eq 8 ] && [ -f "$1" ] || { echo "FAIL: expected 8 shipped cards"; exit 1; }
  # *(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments; s8h §8h.14; D21-47's time-bound rule)*
  # AMENDED in s8h's work commit: the eight shipped cards now carry permission_mode (s8h §8h.2), so "no card carries a
  # mode" is false and is no longer the no-dispatch premise. The premise is the launch check (s8h §8h.7): every whole
  # run below starts with REDKILN_LAUNCH_RECORD removed, so a real-harness `run` refuses before any spawn, and the named
  # test proves that refusal here. The two lines this replaces read:
  #   pm=$(grep -l 'permission_mode' "$@"); rc=$?
  #   [ "$rc" -eq 1 ] || { echo "FAIL: a card carries permission_mode (rc=$rc) — 7i's or 7f's run could reach a REAL dispatch; stop and re-run through §8a.10's rule (D21-40 decision 2)"; exit 1; }
  mkdir -p target/s8a-ctl || { echo "FAIL: cannot make the card-mode control directory"; exit 1; }
  printf '{"tools": ["StructuredOutput"]}\n' > target/s8a-ctl/no-mode.json || { echo "FAIL: cannot write the card-mode control"; exit 1; }
  if grep -q '"permission_mode"' target/s8a-ctl/no-mode.json; then echo "FAIL: SYNTHETIC CONTROL — the card-mode probe sees a mode in a card that carries none"; exit 1; fi
  pm=$(grep -l '"permission_mode"' "$@"); rc=$?
  [ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$pm" | grep -c .)" -eq 8 ] || { echo "FAIL: not all eight shipped cards carry permission_mode (rc=$rc): $pm — s8h §8h.2's cards are not in place"; exit 1; }
  out=$(nodispatch cargo test -p rk-cli --test it -- run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the launch-check refusal test rc=$rc — the replays' no-dispatch premise (s8h §8h.7) is not proved"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'test run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn ... ok' || { echo "FAIL: run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn did not report ok — renamed, deleted or not selected"; exit 1; }
  printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the launch-check selection did not run exactly 1 test (a filter that matches nothing is a green)"; exit 1; }
  nd_check "8a (11) launch-check premise" || exit 1
  # NO-DISPATCH, MEASURED: dispatch_completed rows in the store AND in every run journal, before and after.
  dcount() {  # $1 = store path; prints "<store rows> <journal rows>"
    python - "$1" "$(dirname "$1")/runs" <<'PY'
import glob, os, sqlite3, sys
db, runs = sys.argv[1], sys.argv[2]
c = sqlite3.connect("file:" + db.replace(os.sep, "/") + "?mode=ro", uri=True)
s = c.execute("select count(*) from t2_event where kind = 'dispatch_completed'").fetchone()[0]
j = 0
for f in glob.glob(os.path.join(runs, "*", "events.jsonl")):
    j += sum(1 for line in open(f, encoding="utf-8") if '"kind":"dispatch_completed"' in line)
print(s, j)
PY
  }
  d_before=$(dcount "$store"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$d_before" ] || { echo "FAIL: cannot count dispatch_completed before the whole runs (rc=$rc)"; exit 1; }
  mkdir -p target/s8a-ctl/dstore/runs/r1 || { echo "FAIL: cannot make the dispatch-count control"; exit 1; }
  python - target/s8a-ctl/dstore/store.sqlite3 <<'PY' || { echo "FAIL: cannot seed the dispatch-count control store"; exit 1; }
import sqlite3, sys
c = sqlite3.connect(sys.argv[1])
c.execute("create table if not exists t2_event (id integer primary key, kind text, payload text, anchor_sha text)")
c.execute("delete from t2_event")
c.execute("insert into t2_event (kind, payload) values ('dispatch_completed', '{}')")
c.commit()
PY
  printf '{"kind":"dispatch_completed","payload":{}}\n' > target/s8a-ctl/dstore/runs/r1/events.jsonl || { echo "FAIL: cannot seed the control journal"; exit 1; }
  [ "$(dcount target/s8a-ctl/dstore/store.sqlite3)" = "1 1" ] || { echo "FAIL: the dispatch counter's control did not read 1 1 — the no-dispatch guard is blind"; exit 1; }
  # THE LEGACY TREE (D21-44: it stays put until s8j). 7f's run and 7f's cleanup both remove it; record it first.
  legacy="$common/redkiln/worktrees/rk-fix-doctor-test-count"
  lref=$(git rev-parse --verify --quiet 'refs/heads/rk-run/rk-fix-doctor-test-count^{commit}'); lpin=$(git rev-parse --verify --quiet '73c087a^{commit}')
  [ -n "$lref" ] && [ -n "$lpin" ] && [ "$lref" = "$lpin" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is '$lref', not 73c087a ('$lpin') — the untouched fixture item moved before this stage ran (D21-42 18)"; exit 1; }
  wtl=$(LC_ALL=C git worktree list --porcelain); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: git worktree list rc=$rc"; exit 1; }
  lreg=0; printf '%s\n' "$wtl" | grep -qxF "worktree $legacy" && lreg=1
  # [amended 2026-09-24, s8a D21-47] THE RESTORE IS AN EXIT TRAP TOO: a whole run that fails after 7f's cleanup line
  # exits through it, so the legacy tree is re-registered on every exit path. Idempotent; the explicit call below
  # and the trap cannot both add it. The store is KEPT, not restored (§8a.1): its backup is for recovery by hand.
  legacy_restore() {
    [ "$lreg" -eq 1 ] || return 0
    wl=$(LC_ALL=C git worktree list --porcelain) || { echo "FAIL: git worktree list failed during the legacy restore"; return 1; }
    printf '%s\n' "$wl" | grep -qxF "worktree $legacy" && return 0
    o=$(LC_ALL=C git worktree add "$legacy" rk-run/rk-fix-doctor-test-count 2>&1) || { echo "FAIL: cannot restore the legacy run worktree at $legacy"; printf '%s\n' "$o"; return 1; }
  }
  trap 'legacy_restore || { echo "FAIL: the legacy run worktree was NOT re-registered on exit — restore it by hand (D21-44)"; exit 1; }' EXIT
  # THE NO-DISPATCH TRAP (s8g's form; D21-46; D21-47). guard (0) made it, proved it empty by a control, and every
  # whole run below goes through nodispatch: CLAUDE_CONFIG_DIR at the trap, the launch-record variable unset, the
  # compiled fake claude first on PATH. A third guard beside the card condition and the dispatch_completed count, not a
  # substitute: a child launched with CLAUDE_CONFIG_DIR set explicitly escapes the trap (s8g's stated limit).
  # [re-pointed 2026-09-24, final consistency pass] The emptiness readings are the shared helper's nd_check.
  nd_check "before the whole runs" || exit 1
  whole 7i runbook/phase-1/s7i-cost.md 'CP7I-END-OF-BLOCK reached'
  whole 5 runbook/phase-1/s5-workspace-ci-rk-types.md 'FAIL: deny.toml absent'
  whole 7a runbook/phase-1/s7a-ports-and-fakes.md 'FAIL: cargo test -p rk-ports exited'
  whole 7b runbook/phase-1/s7b-kernel.md 'spikes/p9/grammar-probe.py absent'
  whole 7f runbook/phase-1/s7f-conformance-and-records.md 'CP7F-END-OF-BLOCK reached'
  d_after=$(dcount "$store"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$d_after" ] || { echo "FAIL: cannot count dispatch_completed after the whole runs (rc=$rc)"; exit 1; }
  [ "$d_after" = "$d_before" ] || { echo "FAIL: dispatch_completed moved '$d_before' -> '$d_after' (store, journals) — a whole run DISPATCHED (D21-40 decision 2)"; exit 1; }
  nd_check "after the whole runs" || exit 1
  lnow=$(git rev-parse --verify --quiet 'refs/heads/rk-run/rk-fix-doctor-test-count^{commit}')
  [ "$lnow" = "$lref" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count moved '$lref' -> '$lnow' during the whole runs"; exit 1; }
  if [ "$lreg" -eq 1 ]; then
    legacy_restore; rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: the legacy run worktree could not be re-registered after the whole runs (rc=$rc)"; exit 1; }
    lh=$(LC_ALL=C git -C "$legacy" rev-parse HEAD); rc=$?
    [ "$rc" -eq 0 ] && [ "$lh" = "$lref" ] || { echo "FAIL: the legacy run worktree is at '$lh', not $lref"; exit 1; }
    lst=$(LC_ALL=C git -C "$legacy" status --porcelain); rc=$?
    [ "$rc" -eq 0 ] && [ -z "$lst" ] || { echo "FAIL: the restored legacy run worktree is not clean (rc=$rc)"; exit 1; }
  fi
  trap - EXIT
fi
# [added 2026-09-24, s8a D21-47] THE TRAP, ONCE MORE, ON BOTH BRANCHES: every cargo run above went through the wrapper
# too. [re-pointed 2026-09-24, final consistency pass] nd_check: the trap is empty and the fake logged no call.
nd_check "the block's end" || exit 1
echo "CP8A-END-OF-BLOCK reached"
```

**Closing.** The stage closes in two commits: the work plus its checkpoint output, then the tracker. That second
commit updates `PROGRESS.md`:

- rows 5, 7b and 7f move to their passed hashes;
- row 7a is re-run;
- row 7c is marked provisional (§8a.9);
- row 7e does **not** move: the §8a.8 amendment changes no byte of Checkpoint 7e's block (guard (9a); finding (10));
- a new `s8a` row is added.

Adversarial refuting rounds run over the diff until a round moves no code and no guard.

**Findings for the lead, recorded rather than absorbed:**

1. D21-42 decision 2 names ADR-0099 cl.5 and ADR-0093 d4. **ADR-0094 decision 1's** *"remaining an opaque
   non-finalised newtype until S-BIN"* is also a `## Decision` sentence that the split moves, and so are **ADR-0135
   decision 4 and its third forbids bullet**, which forbid editing the adapter's `change_summary` row to succeed.
   §8a.2 supersedes all of them in part.
2. **`s8`'s bullet and guard go red at this stage's commit.** They read *"No finalised `ChangeSummary` type"* and
   check `pub struct ChangeSummary(`. The plan assigns that edit to no stage, so §8a.3 takes it. Checkpoint 7a's
   declaration line breaks the same way.
3. **Packet (i) A(3) says Checkpoint 7f takes no checkpoint edit.** This stage's brief asks for a re-scope, and
   D21-40 decision 4 requires 7f to be repaired. §8a.5 does both. `s8e` must move the literal again. ADR-0134
   decision clause 5 says, in its `## Decision`, that 7f *"becomes non-re-runnable at that commit"*. This file reads
   that sentence as being about the old text, so the re-scope does not supersede it. **Settled by D21-43 N27**
   (2026-09-24): *"The reading is confirmed."* No supersession.
4. **The design's phase-bearing S-CODEX row is §10.4's, not §10.5's**, and there are six register sites, not two
   *(seven: AC-239's clause was missed; amended 2026-09-25, `s8a` fix round 4, §8a.6)*.
5. **Checkpoint 7c becomes provisional here**, and its re-run needs a paid probe. **Settled by D21-43 N28:**
   `s8j` re-runs it whole, from a plain terminal through the launcher, with the live probes. This stage does not.
6. **The kernel passes `change_summary("")` today.** A Text body that accepts an empty range would be a vacuous
   green over eleven shipped steps. The empty-range refusal is named as a test.
7. **The stage plan's "via git diff" must not be read as a shell-out from the adapter.** The adapter's own manifest
   comment confines git to `rk-git`.
8. **Checkpoint 7f's whole run removes the legacy run worktree, which D21-44 says stays put** (2026-09-24
   revision). Both `recreate_run_worktree`'s `worktree remove --force` and 7f's own cleanup line remove it, and
   nothing re-adds it. This stage restores it in guard (11). **For `s8d-ii`** (D21-46 R1 moved the root out of
   `s8e`): once the root moves to `.redkiln/worktrees/<item>`, 7f's cleanup line still names
   `.git/redkiln/worktrees/rk-fix-doctor-test-count`.
   A whole re-run of 7f would then remove the legacy tree and leave the new one registered. That is the hazard
   the cleanup line was written to disarm (*"makes every later repo-root grep double-count against a stale tree"*),
   and D21-44 asks every recursive walk over `.redkiln/` to exclude `worktrees/`. *Settled by `s8d-ii` §8d-ii.7
   (consistency pass 2026-09-24):* 7f's run arm moves to a throwaway clone and the cleanup line is deleted, in the
   commit that moves the root. This stage's restore covers the one 7f run that precedes it.
9. **An LFS pointer path would have made the `Text` variant lie** (2026-09-24 revision). `classify` answers
   `Text` for a pointer file with a text extension, or with no NUL, and the object-store diff of an LFS path is a
   pointer delta. §8a.2 item 2a classifies it `Binary` on the blob, with two named tests.
10. **The backtick defect D21-46 assigns to "Checkpoint 7e" is not in Checkpoint 7e's block** (2026-09-24, D21-46
    revision). The line *"FAIL: .redkiln/config.toml absent — Step 7d's `init` did not run"* is in `s7e` §7e.1's
    work block, cited here by its content and never by a line number. Checkpoint 7e's block, under *"**Checkpoint 7e.** All
    thirteen"*, has no hit. The fix is the same wherever the line sits (§8a.8), but PROGRESS row 7e does not move
    and does not become provisional, because no byte of its block changes. Guard (9a) asserts that. Neither block
    can be re-run whole: Checkpoint 7e's commits and tags an existing tag, and `s7e` §7e.1's runs `init` over the live
    `.redkiln/`.
11. **A backtick scan that works line by line reports false hits** (2026-09-24). At `c5d19d7`, a per-line
    quote-aware scan of all 99 bash blocks reports four hits. Two are real (7e's line and 7h's (10a)). Two are
    false, in `s8f` and `s8j`, where a single-quoted Python program spanning several lines carries backticks in
    its own strings. Guard (9a)'s scan carries quote state across lines and reports exactly the two real hits,
    which agrees with `s8e` finding 10 (its finding 14 before D21-46 R1's renumbering). `s8e` guard (10)'s scan is per-line, but it reads only the blocks `s8e`
    re-runs, and none of those has a multi-line single-quoted string with a backtick. If it is ever widened to the
    corpus, it needs the same change.
12. **`RK_CP_NESTED` is adopted here for guard (11), and the two stages that re-run this block whole do not set
    it yet** (2026-09-24). `s8b` guard (0) runs it with `out=$(bash "$cp8a" 2>&1)`, and `s8i` runs it with
    `out=$(bash "$T/$blk.sh" 2>&1)`. Neither passes `RK_CP_NESTED=1`, so each still repeats the five whole runs.
    D21-46 says *"every file adopts it"*. Those two edits belong to those files, not to this one. *Settled in the
    consistency pass 2026-09-24:* both now launch this block with `RK_CP_NESTED=1`, `REDKILN_LAUNCH_RECORD` unset and
    `CLAUDE_CONFIG_DIR` at an empty trap.
13. **ADR-0008 is also moved by the split, and D21-42 decision 2 does not name it** (2026-09-24, D21-47 fix round).
    Its forbids bullet reads *"`ChangeSummary` may not be designed and locked purely against a synthetic or paper
    fixture"*. This stage finalises the `Text` arm and the two-arm shape over a two-commit fixture repository before
    S-BIN runs. Rule 1 makes that a supersession in part, as in finding (1). The `Binary` payload is still designed
    against the UE project, at `s8i`. §8a.2's table carries the row, and guard (3) checks the back-link.
14. **The fake `claude` shadows shell callers only** (2026-09-24, D21-47 fix round). D21-47's replay rule puts a
    fake `claude` that prints the pin first on `PATH`. On Windows, Rust's `Command::new("claude")` resolves
    `claude.exe` and nothing else, and this box has both `~/.local/bin/claude` and `claude.exe`. A `sh` script
    named `claude` is therefore invisible to a Rust-spawned child. That child would find the real `claude.exe`
    further down `PATH`. For Rust children, the guards are the trap `CLAUDE_CONFIG_DIR`, the unset launch record
    (which from `s8h` on refuses before spawn), the card condition and the `dispatch_completed` count. **For the
    lead:** if D21-47 means the fake to shadow Rust children too, it has to be a `claude.exe`, for example a copy of
    a built fixture binary that prints the pin. Every `s8` file that states this rule inherits the same limit.
    *Settled in the final consistency pass 2026-09-24:* the lead built that fake, compiled from
    `runbook/tools/fake-claude.rs` by `runbook/tools/nodispatch.sh`, and guard (0) now uses it.
15. **The range read is a gix read, measured rather than assumed** (2026-09-24, D21-47 fix round). The earlier
    text said the read would use `GitOp`'s exec reads. Those are `RevParse`, `Status` and `Log`, which the hybrid
    already keeps on the executable, although ADR-0077's Decision names gix as the owner of rev-parse and status.
    That existing drift is not this stage's to settle. This stage adds no exec read to it.

---
