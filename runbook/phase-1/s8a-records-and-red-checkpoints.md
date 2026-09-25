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
- **`s8i` supersedes two of this stage's pins by name** when it finalises `Binary`: the sealed-slot test and
  Checkpoint 8a's no-`pub`-field guard (§8a.2, §8a.9, guard (6)).
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
- **Checkpoint 7f** fails with *"the three conformance lints are not all ARMED"*. `s7g` and `s7h` discharged two of
  them, each removing its `#[ignore]` in the commit that made it pass, which is `s7f`'s own convention. D21-9
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
condition (no shipped card carries `permission_mode`) is checked before the first of them. In this session
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
     supersedes **by name** two pins this stage writes:
     `change_summary::binary_arm_is_not_constructible_outside_rk_ports`, with its `trybuild` case, and Checkpoint
     8a's guard (6) line that asserts the sealed payload declares no `pub` field. `s8i`'s file already says that
     both are *"superseded by name"* and that row 8a becomes `passed <hash>, re-run pending`. The record states
     that the slot opens by a later record once S-BIN reports. ADR-0092 rule 1 then decides whether `s8i`'s
     record supersedes this one in part.
2. **The serialised `Text` value carries the members the shipped catalog already reads:**
   - `sha` — `adversarial-review.toml`'s `clone` step reads `inputs = { sha = "artifact:ChangeSummary.sha" }`;
   - `diff` — `change-summary.toml`'s `lint` step reads `inputs = { diff = "${summarise.outputs.diff}" }`;
   - `manifest` — per path: the path, its `classify` result, and its change kind.

   A field set that misses a member the catalog reads is a value that resolves nothing.

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
| **ADR-0135** decision 4, decision 5, and the third and fourth forbids bullets | *"**It must not be "fixed" into a synthesised value.**"*; *"Checkpoint 5's grep bans the bare type outside `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths."* and *"**Checkpoint 5's RED is untouched by this record.**"*; *"Forbids synthesising a `change_summary` value … and forbids editing either adapter's `change_summary` row to make the step succeed."*; *"Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel`"*, **only** as far as a test double's impl signature under `tests/` | The map, decisions 1–3 and 6, and the ban on production `rk-kernel`. Back-link: `["0148", <id>]`. |
| **ADR-0093** decision 4 | *"S-BIN blocks the design of `ChangeSummary` (AC-497, AC-356) and the Toolchain port's `change_summary` method body"*. S-BIN now blocks the `Binary` arm and its body only. | Decision 4's scope rule, and its forbids bullet unchanged. |
| **ADR-0094** decision 1 | *"`change_summary` remaining an opaque non-finalised newtype until S-BIN (ADR-0093 decision 4)"*. ⚠️ **D21-42 decision 2 does not name this sentence**, but it is a `## Decision` sentence that the split moves, so rule 1 makes it a supersession too (finding (1) below). | Every other clause of decision 1. |
| **ADR-0008** `## Decision` and its first forbids bullet *(added 2026-09-24, D21-47)* | *"`ChangeSummary` is designed against it"* and *"`ChangeSummary` may not be designed and locked purely against a synthetic or paper fixture; it must be designed against the real UE project's actual artefacts."* These move **only** as far as the enum's two-arm shape and the `Text` arm, which this record finalises now, over a two-commit fixture repository, before S-BIN runs. ⚠️ **D21-42 decision 2 does not name ADR-0008 either**, and rule 1 makes it a supersession for the same reason (finding (13)). | The `Binary` arm's payload is designed against the real UE project's artefacts at `s8i` (S-BIN) and stays sealed until then. The UE fixture sentence, the Phase-2 adapter sentence and the second forbids bullet stand. Back-link: `superseded_by: [<id>]`. |

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
- **The README.** The index rows and back-link cells, the census, and the *"next free number"* sentence. The same
  sentence also appears in `00-preamble.md`.

**The code.** Write the tests first (§8a.9). Then make these changes:

- **`crates/rk-ports/src/toolchain.rs`.**
  - The enum replaces `pub struct ChangeSummary(pub(crate) ());`.
  - The doc comment's *"This is the one site in `crates/` the name may appear"* and *"no real adapter implements it
    at this stage"* are corrected, with dates.
  - `FixtureToolchain` returns `Text` for text paths it was scripted with. It refuses by name, never with an empty
    `Text`, for a range that carries a `Binary` path.
- **The range's diff is read by `rk-git`, through gix.** The Rust adapter's `Cargo.toml` says *"no git is spawned
  by this crate (ADR-0077 confines git to rk-git's one wrapper…)"*, and that stands. *(amended 2026-09-24, D21-47:
  this bullet said the read would use `GitOp`'s exec reads. ADR-0077's `## Decision` reads **"gix owns the read
  path"**, and ADR-0091 decision 1 confirmed gix as the read owner at the pin, so an exec read would move ADR-0077.)*
  - The range read is **`rk_git::diff::range_diff`**, in a new `crates/rk-git/src/diff.rs`. It resolves both ends of
    the range, takes the changed paths and their change kinds from `Repository::diff_tree_to_tree`, and reads each
    changed blob by object id for the LFS pointer check (item 2a) and the text diff. **Measured 2026-09-24:** the
    workspace pins `gix = "=0.87.1"` with default features. Its `basic` default carries `blob-diff`, and
    `gix-0.87.1/src/repository/diff.rs` declares `pub fn diff_tree_to_tree`. If the implementation names the
    function otherwise, guard (6)'s patterns follow it in the same commit.
  - **No `GitOp` variant is added**, and `diff.rs` names neither `GitOp` nor `GitExec`. So ADR-0077 does not move,
    and neither does ADR-0105 clause 5. Under D21-47's clause-5 rule a read is never a supersession of that clause
    anyway. Guard (6) asserts that `GitOp`'s declaration is byte-identical to `S8A_BASE`'s.
  - If the pinned gix cannot produce this read, **halt.** An exec read would need a record that supersedes ADR-0077's
    read-path sentence in part, and that is a decision this stage does not hold. Do not shell out from the adapter.
  - `RustToolchain::change_summary`'s own body calls `range_diff(…)` directly. Guard (6) asserts that call
    positively, because a ban on spawning git alone passes on a body that reads nothing.
- **`RustToolchain::change_summary` returns `Ok(ChangeSummary::Text(…))`** when every path in the range classifies
  `Text` or `Generated`.
  - A range that carries a `Binary` path refuses through the registry's dated `change_summary` row. The row is
    re-dated, and its `reason` is narrowed to the sealed arm.
  - A range that carries an LFS pointer path (item 2a) refuses the same way, naming the path.
  - An empty or unresolvable range refuses by name. It never returns an empty `Text`. ⚠️ **This matters today:**
    `crates/rk-kernel/src/runtime/step.rs` calls `"change_summary" => toolchain.change_summary("").map(|_| ())`.
    The kernel passes an **empty** range and discards the value until `s8d`'s resolver binds `artifact:Changeset.sha`.
    An adapter that answered `Ok` to `""` would turn eleven refusals into eleven vacuous greens.
- **The Node adapter's row, body and registry tests are unchanged.** Its `change_summary` still refuses. Finalising
  a Node body is not in D21-42 decision 2 and is not taken here.
- **Stale texts.** Four sites get dated corrections:
  - `crates/rk-kernel/tests/it/runtime.rs`'s path-qualification comment (the exclusion it avoided is gone);
  - the two adapters' `registry.rs` module docs (*"no `Ok` is constructible"*);
  - `crates/rk-cli/tests/fixtures/lint-controls/ac475_change_summary_control.rs`'s prose.

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
  usize` is admitted. At `fc80687`, `grep -rn 'ChangeSummary[[:space:]]*as' crates/` finds nothing.
  **Stated limit:** the ban reads one line at a time. An `impl` split so that `for …ChangeSummary` starts a new line
  after `impl Trait` is not seen. `rustfmt` keeps `impl Trait for Type {` on one line unless it exceeds the width,
  and guard (10)'s `cargo fmt --check` holds the tree to that. A line-joined reading is the fix if one ever
  appears.
- **The `rk-kernel` production ban.** `crates/rk-kernel/src`, code lines only, strings blanked: no
  `\bChangeSummary\b`.
- **The opacity pin inside `rk-ports`.** The ban excludes `rk-ports`, so this pin is what guards opacity there.
  `rk-ports` declares exactly one `pub enum ChangeSummary`, and the sealed payload type declares no `pub` field. The
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
- Dated notes in `s7e` §7e.2, whose *"the count settles at **278/279**"* was true until this stage.
- The historical-capture doc comments in `crates/rk-kernel/tests/it/probe_differential.rs` (*"valid **until `s7e`
  §7e.2 repairs the five defects**"*) and `crates/rk-kernel/tests/it/on_false.rs` (*"the **only live instances**"*).
  Both tests pin F42's historical measurement over literals and a captured fixture, and their assertions do not
  move.

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

  Checkpoint 8a's sweep (9) requires every register line that matches `Codex[^|]*\bPhase 2\b` to cite the new
  record. At `ca8f387` it matches exactly those six lines.

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
stage's base commit. All three of this stage's records are written in the conforming form.

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
| `rk-ports` | `change_summary::text_variant_serialises_sha_diff_and_manifest` | there is no `Text` arm, so no `sha`/`diff`/`manifest` members reach a catalog reference |
| `rk-ports` | `change_summary::binary_arm_is_not_constructible_outside_rk_ports` | a `trybuild` case (`tests/ui/change_summary_binary_not_constructible.rs` plus a committed `.stderr`) that must fail for the **privacy** reason. It is its own harness test: Checkpoint 7a counts `trybuild_pins_the_reason_each_port_fixture_fails`'s four cases |
| `rk-ports` | `fake_behaviour::toolchain_fake_classifies_binary` *(amended)* | its `assert_eq!(format!("{summary:?}"), "ChangeSummary(())")` becomes: a text range yields `Text`, and a binary range refuses by name |
| `rk-git` | `diff::range_diff_lists_every_changed_path_with_its_change_kind` | no range read exists |
| `rk-git` | `diff::range_diff_refuses_an_unresolvable_range` | same |
| `rk-git` | `diff::range_diff_marks_an_lfs_pointer_path` | same. Over a fixture whose commit adds `data/table.json` as a git-lfs pointer blob, the path must come back marked as a pointer, and an ordinary `.json` beside it must not |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_text_variant_from_git_diff` | the body answers `Unsupported` from the registry. Over a two-commit fixture repository it must return `Text` with one manifest row per changed path |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_a_range_carrying_a_binary_path` | a `.uasset` in the range must refuse through the re-dated row and never yield a `Text` that silently omits the path (AC-497) |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_an_empty_range` | the kernel passes `""` today (§8a.2), and an `Ok` there is a vacuous green |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path` | `classify` answers `Text` for `data/table.json`. The range must refuse naming the path, never yield a `Text` whose `diff` is the pointer's `oid`/`size` delta (§8a.2 item 2a) |
| `rk-adapter-toolchain-rust` | `registry::ac489_capability_registry_dated` *(amended)* | its *"change_summary answers Unsupported from its registry row"* over `HEAD~1..HEAD` moves to the binary case |

That is nine new tests and two amended ones. Unchanged, and asserted as regression pins:

- `rk-adapter-toolchain-node`'s `registry::ac489_capability_registry_dated` (Node still refuses);
- `rk-kernel`'s `probe_differential::grammar_agrees_with_the_s_grammar_probe` and `on_false::on_false_closed_set`;
- `redkiln-migrate`'s `output::ac475_zero_change_summary_values_in_output`.

**`s8i` supersedes two of these pins by name** (§8a.2 item 1): `binary_arm_is_not_constructible_outside_rk_ports`
and guard (6)'s no-`pub`-field line. That is expected. It is not drift, and nobody should restore the pins.

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
  `diff.rs` names no exec type. The adapter's source carries no `"git"` literal on any code line, so it spawns
  git by no route, `run::run_program` included.
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
- **Neither whole run that can reach `redkiln run` dispatched.** No shipped card carries `permission_mode`, and
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
block does not read its status.

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
CS_ADR=____      # the ChangeSummary record (§8a.2)
PACK_ADR=____    # the ADR-0114 record (§8a.4)
CODEX_ADR=____   # the Codex-phase record (§8a.6)
S8A_BASE=____    # the commit this stage opened on
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
CS_BAN='\b(struct|enum|type|trait|union)[[:space:]]+[A-Za-z_]*ChangeSummary\b|\btype[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[^=]*=.*\b[A-Za-z_]*ChangeSummary\b|\bimpl\b.*\bfor[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b|\bimpl(<[^>]*>)?[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b|\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Z_]'
cs_filter() { grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\]|\.)*"/""/g' | grep -E "$CS_BAN"; }
raw=$(grep -rn 'ChangeSummary' crates/ --include='*.rs'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the ChangeSummary search found nothing or could not run (rc=$rc) — rk-ports' own enum must be seen"; exit 1; }
hits=$(printf '%s\n' "$raw" | cs_filter)
[ -z "$hits" ] || { echo "FAIL: a ChangeSummary definition, alias or impl outside rk-ports:"; printf '%s\n' "$hits"; exit 1; }
for ctl in 'F|x.rs:1:pub struct ChangeSummary;' 'F|x.rs:2:pub enum TextChangeSummary { A }' 'F|x.rs:3:type Cs = rk_ports::toolchain::ChangeSummary;' 'F|x.rs:4:impl Clone for ChangeSummary {' 'F|x.rs:5:impl rk_ports::toolchain::ChangeSummary {' 'F|x.rs:6:impl<T> From<T> for rk_ports::toolchain::BinaryChangeSummary {' 'F|x.rs:7:    let s = "x"; impl Copy for ChangeSummary {}' 'P|x.rs:8:use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};' 'P|x.rs:9:    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {' 'P|x.rs:10:    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {' 'P|x.rs:11:            Self::ChangeSummary => "change_summary",' 'P|x.rs:12:    ChangeSummary,' 'P|x.rs:13:        Ok(ChangeSummary::Text(text))' 'P|x.rs:14:    // impl Clone for ChangeSummary {}' 'P|x.rs:15:    let k = "impl Clone for ChangeSummary";' 'F|x.rs:16:use rk_ports::toolchain::ChangeSummary as Cs;' 'F|x.rs:17:    ChangeSummary as Summary,' 'F|x.rs:18:use rk_ports::toolchain::{ChangeSummary as _, Toolchain};' 'P|x.rs:19:    let n = Capability::ChangeSummary as usize;'; do
  want=${ctl%%|*}; line=${ctl#*|}; got=$(printf '%s\n' "$line" | cs_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: control must FIRE and did not — the ban is vacuous for: $line"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: control must be ADMITTED and fired — the ban blocks what the record licenses: $line"; exit 1; }; fi
done
kraw=$(grep -rn 'ChangeSummary' crates/rk-kernel/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-kernel search could not run (rc=$rc)"; exit 1; }
k_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\]|\.)*"/""/g' | grep -E '\bChangeSummary\b'; }
khits=$(printf '%s\n' "$kraw" | k_filter)
[ -z "$khits" ] || { echo "FAIL: rk-kernel production code names the ChangeSummary type (ADR-0135's surviving forbids):"; printf '%s\n' "$khits"; exit 1; }
kctl=$(printf 'crates/rk-kernel/src/x.rs:1:fn f(s: &str) -> Option<ChangeSummary> { None }\n' | k_filter)
[ -n "$kctl" ] || { echo "FAIL: the rk-kernel ban's control did not fire"; exit 1; }
nenum=$(grep -cE '^pub enum ChangeSummary[[:space:]]*\{' crates/rk-ports/src/toolchain.rs)
[ "$nenum" -eq 1 ] || { echo "FAIL: $nenum 'pub enum ChangeSummary {' in rk-ports, expected 1 — the split did not land"; exit 1; }
# The sealed-slot pins below (nbin, npub) are SUPERSEDED BY NAME at s8i when S-BIN finalises Binary (§8a.2 item 1).
# [amended 2026-09-24, s8a D21-47] Read over CODE only (comments stripped) and over BOTH field-list forms, tuple (…)
# and brace { … }: the old grep counted a commented-out declaration and missed a brace struct's pub field. These are
# floors beneath the trybuild case (§8a.9), which is the behavioural pin.
cat > target/s8a-ctl/sealed.py <<'PY'
import re, sys
src = open(sys.argv[1], encoding="utf-8").read().replace(chr(13), "")
src = re.sub(r"(?s)/\*.*?\*/", "", src)
src = re.sub(r"(?m)//.*$", "", src)
src = re.sub(r'"(?:[^"\\]|\\.)*"', '""', src)
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
        pub += len(re.findall(r"\bpub\b(?!\s*\()", rest[1:j]))
print("SEALED decls=%d pubfields=%d" % (len(decls), pub))
PY
sealed() { python target/s8a-ctl/sealed.py "$1" 2>&1; }
out=$(sealed crates/rk-ports/src/toolchain.rs); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the sealed-slot reader could not run (rc=$rc): $out"; exit 1; }
[ "$out" = "SEALED decls=1 pubfields=0" ] || { echo "FAIL: rk-ports' sealed Binary payload reads '$out', expected one declaration with no pub field — none means the pin passes over nothing, a pub field means it is constructible"; exit 1; }
for ctl in 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary { pub bytes: u64 }' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub u64);' 'SEALED decls=1 pubfields=0|// pub struct BinaryChangeSummary(pub u64);  pub struct BinaryChangeSummary(pub(crate) ());' 'SEALED decls=1 pubfields=0|pub struct BinaryChangeSummary { bytes: u64, note: Vec<u8> }'; do
  printf '%s\n' "${ctl#*|}" | sed 's/  pub struct/\npub struct/' > target/s8a-ctl/sealed_ctl.rs || { echo "FAIL: cannot write a sealed-slot control"; exit 1; }
  out=$(sealed target/s8a-ctl/sealed_ctl.rs); rc=$?
  [ "$rc" -eq 0 ] && [ "$out" = "${ctl%%|*}" ] || { echo "FAIL: CONTROL — the sealed-slot reader read '$out' for: ${ctl#*|} (want ${ctl%%|*})"; exit 1; }
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
cat > target/s8a-ctl/cs_calls.py <<'PY'
import glob, re, sys
files = sys.argv[1:]
code = {}
for f in files:
    s = open(f, encoding="utf-8").read().replace(chr(13), "")
    s = re.sub(r"(?s)/\*.*?\*/", "", s)
    s = re.sub(r"(?m)//.*$", "", s)
    code[f] = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
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
print("CS-CALLS bodies=%d calling=%d rk_git_diff=%d" % (len(bodies), len(calls), int(imports)))
sys.exit(0 if len(bodies) == 1 and len(calls) == 1 and imports else 1)
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
# The range read is a gix read: GitOp is byte-identical to S8A_BASE's (no exec op added) and diff.rs names no exec type.
test -f crates/rk-git/src/diff.rs || { echo "FAIL: crates/rk-git/src/diff.rs is absent — the range read has no home"; exit 1; }
ge_base=$(git show "$S8A_BASE:crates/rk-git/src/git_exec.rs"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ge_base" ] || { echo "FAIL: cannot read git_exec.rs at S8A_BASE (rc=$rc)"; exit 1; }
op_base=$(printf '%s\n' "$ge_base" | tr -d '\r' | sed -n '/^pub enum GitOp {/,/^}/p')
op_now=$(tr -d '\r' < crates/rk-git/src/git_exec.rs | sed -n '/^pub enum GitOp {/,/^}/p')
[ -n "$op_base" ] && [ "$op_now" = "$op_base" ] || { echo "FAIL: GitOp moved since S8A_BASE (or could not be read) — a range read on the exec wrapper moves ADR-0077's read path"; exit 1; }
dx=$(grep -nE '\b(GitOp|GitExec)\b|Command::new' crates/rk-git/src/diff.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the exec-type search over diff.rs could not run (rc=$rc)"; exit 1; }
dxc=$(printf '%s\n' "$dx" | grep -vE '^[0-9]+:[[:space:]]*//' | grep .)
[ -z "$dxc" ] || { echo "FAIL: rk-git's range read names an exec type — it is a gix read (ADR-0077):"; printf '%s\n' "$dxc"; exit 1; }
grep -qE '\bdiff_tree_to_tree\b' crates/rk-git/src/diff.rs || { echo "FAIL: crates/rk-git/src/diff.rs does not call gix's diff_tree_to_tree"; exit 1; }

# ---- (7) THE NAMED TESTS, per crate, each `... ok` and an exact count.
run_named() {  # $1 = package, $2 = exact count, then the module-qualified names
  pkg=$1; want=$2; shift 2
  out=$(nodispatch cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $pkg's named selection rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted or not selected"; exit 1; }; done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $pkg's selection did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
  nd_check "$pkg's named selection" || exit 1
}
run_named rk-ports 3 change_summary::text_variant_serialises_sha_diff_and_manifest change_summary::binary_arm_is_not_constructible_outside_rk_ports fake_behaviour::toolchain_fake_classifies_binary
run_named rk-git 3 diff::range_diff_lists_every_changed_path_with_its_change_kind diff::range_diff_refuses_an_unresolvable_range diff::range_diff_marks_an_lfs_pointer_path
run_named rk-adapter-toolchain-rust 5 change_summary::change_summary_text_variant_from_git_diff change_summary::change_summary_refuses_a_range_carrying_a_binary_path change_summary::change_summary_refuses_an_empty_range change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path registry::ac489_capability_registry_dated
run_named rk-adapter-toolchain-node 1 registry::ac489_capability_registry_dated
run_named rk-kernel 4 conformance::conformance_every_step_kind_has_an_executor conformance::conformance_every_port_has_a_call_site probe_differential::grammar_agrees_with_the_s_grammar_probe on_false::on_false_closed_set
run_named redkiln-migrate 1 output::ac475_zero_change_summary_values_in_output
# PINNED ARMED, and TIME-BOUND (D21-47; cross-file item 2): s8e's Slice-Verdict commit un-ignores this lint and, in that
# same commit, amends THIS guard by dated line to "1 passed" with the ARMED reason absent and lists 8a as provisional.
# Until then it stays pinned; a guard conditional on the #[ignore] would pass whether or not the lint was discharged.
out=$(nodispatch cargo test -p rk-kernel --test it -- conformance::conformance_every_trailer_key_has_a_producer --exact 2>&1); rc=$?
nd_check "the armed trailer lint's selection" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: the armed trailer lint's selection rc=$rc"; exit 1; }
printf '%s\n' "$out" | grep -qF 'test conformance::conformance_every_trailer_key_has_a_producer ... ignored, ARMED: Slice-Verdict has 0 producers' || { echo "FAIL: the trailer lint is not ARMED naming Slice-Verdict"; exit 1; }
printf '%s\n' "$out" | grep -qF 'discharged by s8e' || { echo "FAIL: the armed reason still names a stage that did not discharge it (ADR-0160)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed; 1 ignored' || { echo "FAIL: the armed selection did not report exactly one ignored"; exit 1; }

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

# ---- (9) THE CODEX PHASE: the design §10.4 row, and every register site that says Phase 2 carries a dated correction.
row=$(grep -F '| **S-CODEX — harness portability** |' references/2026-09-08-architecture-design.md); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$row" | grep -qF "ADR-$CODEX_ADR" || { echo "FAIL: design §10.4's S-CODEX row carries no dated correction citing ADR-$CODEX_ADR — it already says 'Phase 4' in an older note, so that word proves nothing"; exit 1; }
craw=$(grep -nE 'Codex[^|]*\bPhase 2\b' references/2026-09-08-acceptance-criteria.md references/2026-09-08-success-measures.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the Codex-phase register sweep could not run (rc=$rc)"; exit 1; }
c_filter() { grep -vF "ADR-$CODEX_ADR"; }
[ -z "$(printf '%s\n' "$craw" | c_filter | grep .)" ] || { echo "FAIL: a register line still says Codex is Phase 2 with no dated correction citing ADR-$CODEX_ADR"; printf '%s\n' "$craw" | c_filter; exit 1; }
[ -n "$(printf 'x.md:1:the Codex adapter is Phase 2\n' | c_filter)" ] || { echo "FAIL: the Codex sweep's control did not fire"; exit 1; }
# [added 2026-09-24, s8a D21-47] The runbook's Codex exclusion notes (s7a one site, s7b-ii two, s7c one) cite the record.
for spec in 's7a-ports-and-fakes.md 1' 's7b-ii-gates.md 2' 's7c-adapters.md 1'; do
  f=runbook/phase-1/${spec% *}; want=${spec#* }
  test -f "$f" || { echo "FAIL: $f is absent"; exit 1; }
  got=$(grep -cF "ADR-$CODEX_ADR" "$f"); rc=$?
  [ "$rc" -le 1 ] && [ "$got" -ge "$want" ] || { echo "FAIL: $f carries $got line(s) citing ADR-$CODEX_ADR, expected >= $want — its 'Phase 2' Codex exclusion reason has no dated note"; exit 1; }
done
# [added 2026-09-24, s8a D21-47] Every design and register line saying ChangeSummary is a "non-finalised newtype" carries
# a dated note citing the ChangeSummary record (§8a.2: 7 design + 3 register lines at fc80687, AC-497 among them).
nraw=$(grep -nF 'non-finalised newtype' references/2026-09-08-architecture-design.md references/2026-09-08-acceptance-criteria.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the non-finalised-newtype sweep could not run (rc=$rc)"; exit 1; }
n_filter() { grep -vF "ADR-$CS_ADR"; }
[ -z "$(printf '%s\n' "$nraw" | n_filter | grep .)" ] || { echo "FAIL: a design or register line still calls ChangeSummary a non-finalised newtype with no dated note citing ADR-$CS_ADR:"; printf '%s\n' "$nraw" | n_filter | cut -c1-160; exit 1; }
[ -n "$(printf 'x.md:1:ChangeSummary is an opaque non-finalised newtype\n' | n_filter)" ] || { echo "FAIL: the newtype sweep's control did not fire"; exit 1; }
nlines=$(printf '%s\n' "$nraw" | grep -c .)
[ "$nlines" -ge 10 ] || { echo "FAIL: the newtype sweep read $nlines line(s), below the 10 measured at fc80687 — a site was rewritten rather than noted"; exit 1; }

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
  pm=$(grep -l 'permission_mode' "$@"); rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: a card carries permission_mode (rc=$rc) — 7i's or 7f's run could reach a REAL dispatch; stop and re-run through §8a.10's rule (D21-40 decision 2)"; exit 1; }
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
4. **The design's phase-bearing S-CODEX row is §10.4's, not §10.5's**, and there are six register sites, not two.
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
