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

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **§8i.5 re-quotes `s8h` §8h.7's restated `wrap` guarantee.** The sentence it quoted, which ended
  "neither caller is the adapter", is no longer in `s8h`'s file. The substance is unchanged: `wrap` records the
  version and never compares it to the adapter pin.
- **The floor chain is recomputed once**: Checkpoint 8h's ~~1168~~ + this stage's 6 + N, in place of the drafted 780.
  *(dated 2026-10-02, s8i build)* Superseded by the recorded 1404 (the stage-open block below).
- **The trap in (11)** asserts that it starts empty, and (0) names `s8d-ii` among the stages that re-run 7f.

*Adversarial-review fixes 2026-09-24 (D21-47)* (the review of this file at `4a8c832`, 3 BLOCKER, 7 MAJOR, 14 MINOR,
and the cross-file critic's items assigned here; D21-47's rules applied). Each finding, and its fix:

- **B1** (METHOD-S's immutability check compared against the amendment's own commit, over §3 only). Checkpoint 8i (5)
  now requires METHOD-S.md to be append-only over the whole file: the amendment's commit begins with its parent's
  bytes and appends the amendment, HEAD begins with the amendment's bytes, and the working tree begins with HEAD's.
  A synthetic control proves a §0 edit made inside the amendment's own commit is seen.
- **B2** (review trees built once and reused across captures and both passes). §8i.2 item 4 and §8i.4 now register
  five kinds of copy, and no tree is reused. The base repository is frozen. Greenness runs, capture sessions and every
  dispatch each get a byte copy at a path no earlier run used. The driver checks every dispatch copy before it spawns:
  a clean status including untracked and ignored files, one commit at the review commit, the blob oids, a leak scan
  over untracked files too, and no account `projects/<slug>` for the path. Each result is written into the record, and
  (4) asserts them on every valid record. Three driver tests pin it.
- **B3** (the route trusted hand-written `rule_outcome` pins). (4) now computes each pass's outcome from the records
  and `missing-fields.jsonl`. It re-runs the pinned `audit-s.py bin` over the same records, requires the pinned
  `audit.txt` byte-for-byte, and requires the pins, the audit and its own computation to agree. A B4 re-run must
  exist exactly for a missed seed with a named field, and each named field's quote is found verbatim in the digest.
  (7) derives the route from the computed outcomes.
- **M4** (the floor regex flagged `ChangeSummary::Binary(b) =>` match arms). (8)'s floor now matches only
  constructor calls: an associated function of `BinaryChangeSummary`, or a lower-case `binary…` constructor. Privacy
  and the `trybuild` case carry the struct literal. Two controls must fire, and three must be admitted: a match arm,
  an `if let` and a comment.
- **M5** (the review-input assembler and the `Binary` human-gate route had no owner; `rk-cli`'s `review_pass` is
  ADR-0162's re-execution pass). This stage owns them, in a new `rk-cli` module `review_input`. The owner is not
  rk-kernel, because Checkpoint 8a's `rk-kernel` production ban forbids it. Where the review dispatch's input is
  assembled is measured at stage open, and a missing seam is a stop for a record (§8i.8). The two tests move to
  `review_input::`.
- **M6** (METHOD-S §0's `card_adopted` and `api_error_status` were dropped). Both are record fields (§8i.7) and
  §8i.2 item 5 registers them, with two driver tests and their mutants. (4) asserts `card_adopted` true and
  `api_error_status` null on every valid record.
- **M7** (driver tests could spawn a real `claude`). One stub helper serves every driver test, and a test pins it
  (§8i.3). B0's test and mutant runs, and every process-level cargo run in Checkpoint 8i (6), (8) and (10), run under
  D21-47's no-dispatch wrapper. The wrapper puts a fake `claude` first on PATH, drops every real `claude` directory
  from PATH, unsets `REDKILN_LAUNCH_RECORD` and points `CLAUDE_CONFIG_DIR` at a trap. Controls prove each part, and
  (12) asserts the trap empty and the fake uncalled.
- **M8** (a reviewer with `Bash` can damage BrittonGames or read answer files). BrittonGames is snapshotted before
  B2 and after every batch into a pinned file (§8i.5). A moved snapshot is a stop, and (12) asserts the series
  unchanged. The answer-bearing paths are named in the amendment for the out-of-tree void. Prevention is not
  claimed, because Claude Code's permission rules cannot fence `Bash`; this is said in §8i.2 item 4.
- **M9** (nothing proved which runner dispatched). The batch pre-check requires `runner/` and `results/audit-s.py`
  committed and clean. Each record carries `runner_commit` and `runner_sha256`, and a driver test pins the refusal.
  (4) asserts one runner commit, an ancestor of the pinned commit with the same `runner/` tree.
- **M10** (the supersession missed Checkpoint 5's copy of the opacity pin, and `s8`'s final-label bullet).
  §8i.8 supersedes the pin by name in `s5`'s amended block in the same commit. `s8`'s bullet naming S-BIN, S-H1c and
  P8 takes an inline dated note, which (9) asserts.
- **MINORs.**
  - `MINTED_CLASSES` joins (11)'s input list. The inputs are demanded only when the replays run, in the non-nested
    branch (D21-47).
  - The stale nested-chain warning is replaced: (11) refuses a replayed 8h or 8a block that no longer honours
    `RK_CP_NESTED`.
  - Checkpoint 8h's replayed `claude --version` now reaches the fake, which prints the pin (cross #12).
  - The caps are re-summed from the records, `probes.jsonl` and the procedure log, as a number against 90.00. A
    missing cost is refused, and the haiku cap is asserted (cross #17).
  - The model rule now matches the check: one observed id, and a second is a stop.
  - B4 is named as the one registered exception to the freeze.
  - The go-ahead lines are read from the ledger entry recorded at write time. They are timed against the records,
    and the Checkpoint 8h hash resolves to the commit that `PROGRESS.md` row 8h records.
  - The README check needs the index row, with a prose control.
  - The launch blocks are split, and each runs in a subshell.
  - Citations: the stage-plan quote is replaced by D21-42's and D21-47's own words, D17-5's *"in this session"* is
    declined, and ADR-0093 decision 4 carries `s8a`'s supersession.
  - A dry run in the set is refused, never filtered.
  - Platform: a short review root, a terminator-agnostic `CLAUDE.md` strip with its hit count asserted, a disk-space
    precondition, and Python inputs under `target/` rather than `/tmp`.
  - (12)'s BrittonGames registration check is path-normalised and has a control.
  - A "no" on a clean tree's capture is a stop too.
- **D21-47 rules.** The finding, the record and the ledger entry are located by the ids written into Checkpoint 8i
  (0) at write time, never by headline or by an "exactly one" count.
- **Not fixed, and why.** None was withheld.
- **Found here and raised.** One outcome is not registered: one seed missed, summary fields cited, no missing field
  named. It reaches no branch of METHOD-S §3.4, and D21-43 N21 names only two outcomes. It is not mapped here. The
  checkpoint fails on it, and the decision is owed before the amendment (§8i.2 item 9).
- **Tests.** The workspace gains 7 + N new tests, with s8a's sealed-slot test superseded, so the floor adds 6 + N, as
  before. The corpus driver gains 7 tests, 18 → 25.

*Final consistency pass 2026-09-24* (the eleven `s8` files read against each other after the D21-47 fix round; the
pass brief and the lead's integration findings are the authority). One line per edit:
- *Final consistency pass 2026-09-24:* B0 and Checkpoint 8i (0) source D21-47's shared wrapper, `runbook/tools/nodispatch.sh`
  (`nd_setup`, then the scoped `nodispatch`), in place of their inline shell-script fake, which a Windows process never
  resolves. `nd_check` follows every process-level cargo run, every replay, and (12).
- *Final consistency pass 2026-09-24:* the floor's base is re-derived once more from the files as they now state their counts:
  Checkpoint 8h's expected ~~1222 (1171 + 51), so the floor is 1222 + 6 + N = 1228 + N~~. *(dated 2026-10-02, s8i build)* Superseded
  by the recorded 1404 (the stage-open block below).
- *Final consistency pass 2026-09-24:* (11) no longer demands the replayed blocks' record ids from the environment: each block carries its
  own as literals (D21-47), and this block supplies none.
- *Final consistency pass 2026-09-24:* (11)'s dispatch scan admits Checkpoint 8h (10)'s two sandboxed `wrap` runs and its argv-screen
  list by two reasoned, counted exemptions, each tied to its premise, with controls both ways. Without them the scan
  refused 8h's block.
- *Final consistency pass 2026-09-24:* §8i.5 re-quotes `s8h` §8h.7 step 8 from its current bytes, and records `s8h`'s answer on whether
  `wrap` hands the terminal to an interactive child and where the launch record resolves.

*D21-48 (2026-09-24)* (the operator's answers to the questions the `s8` files left owed; X3 and X16 are this file's).
One line per edit:
- *D21-48 (2026-09-24):* X3 registers the third undecided outcome. §8i.2 item 9's owed decision is replaced by X3's
  words: `undecided-partial` (exactly one seed missed, counted after any B4 re-run, summary fields cited, no missing
  field named) is named in the amendment beside N21's two and routed to the human gate, before dispatch.
- *D21-48 (2026-09-24):* §8i.0 item 9 cites X3 beside N21. §8i.8's route table gains the `undecided-partial` row, its
  mechanical list and the pins block's outcome set go from five values to six, and §8i.3's six audit fixtures replace
  `unregistered` with `undecided-partial`.
- *D21-48 (2026-09-24):* guard (4)'s computation maps the shape to `undecided-partial`, and nothing computes as
  `unregistered`. The computation moves into one shared snippet, `outc`, which scores the real records and also seven
  synthetic record sets. Those sets reach each of the six values, one of them twice (branch 3 both ways). A computed
  outcome outside the six fails the block. Guard (7)'s `route_for` maps `undecided-partial` to the human gate, and a
  control proves it.
- *D21-48 (2026-09-24):* §8i.6's stop *"A pass reaches the unregistered outcome … while its decision is still owed"* is
  removed, because the decision is taken.
- *D21-48 (2026-09-24):* X16 decides the gate path. §8i.8's hedge (*"If either is missing unless the kernel names the
  type, that is a stop for a record"*) is replaced by X16's type-neutral kernel gate request. It has a closed reason,
  `automated-verdict-refused`, and carries the summary as an opaque attachment. It is built on the human-gate route only.
  The measurement of the review-input seam stays, and it is now the only stop left there. The premise was measured at
  `fc80687`: `GateRecord` carries a free `kind` string and no attachment.
- *D21-48 (2026-09-24):* X16's tests are three `gate_request::` tests in rk-kernel, human-gate route only, with a
  mutant each. The human-gate `review_input::` test also pins the hand-off to the request. Guard (8) runs the three
  tests on that route. On the reviewer route it requires that the reason is not spelled. On either route it asserts
  that `crates/rk-kernel/src` names no `ChangeSummary` on a code line, with controls both ways. ADR-0135's forbids
  bullet and Checkpoint 8a's ban stand.
- *D21-48 (2026-09-24):* **Tests.** The workspace gains 7 + N + G new tests, where G = 3 on the human-gate route and 0
  on the reviewer route, with s8a's sealed-slot test superseded. The floor adds 6 + N + G. This was 7 + N new and
  6 + N on the floor. The floor chain is ~~1222 + 6 + N + G = 1228 + N + G~~, re-derived at stage open from Checkpoint
  8h's recorded count. The corpus driver's 25 tests do not change.
- *D21-48 (2026-09-24), floor propagation:* the predecessors now state `s8c` **78**, `s8d` **62**, `s8d-ii` **50**, `s8f` **85** and `s8h` **54**, so
  Checkpoint 8h's expected count is ~~**1243** (1189 + 54)~~ and this stage's floor ~~1243 + 6 + N + G = **1249 + N + G**~~
  (§8i.10's bullet, Checkpoint 8i (0) and (10)). The 1222 base above is superseded.

*Stage open, 2026-10-02 (s8i build, an AFK session; entry HEAD `09018dd`).* Each line below is a dated note. The
measurements sit in the sections named, and nothing here is a go-ahead or a dispatch.
- *(dated 2026-10-02, s8i build)* **Floor, re-derived from the RECORDED count, not the drafted chain.** `PROGRESS.md` row 8h records
  Checkpoint 8h *"1404 passed, 0 failed, 9 ignored over 44 at `d24d15a` (floor 1404)"*, and D21-138 reads *"(12)'s
  whole suite read **1404 passed / 0 failed / 9 ignored over 44 result lines** (FLOOR 1404)"*. So the base is
  **1404** over **44** result lines, the drafted 1243 (and the older 1168 and 1222) is struck, Checkpoint 8i (0) sets
  `FLOOR_8H_PASSED=1404` and `FLOOR_8H_RESULTS=44`, and this stage's floor is **1404 + 6 + NFIELDS + NGATE** (NGATE 3
  on the human-gate route, 0 on the reviewer route). This build adds 0 workspace tests before S-BIN reports: the
  driver's 25 tests are the corpus runner's, not this workspace's. NFIELDS and NGATE are known only after S-BIN reports.
- *(dated 2026-10-02, s8i build)* **Entry held**: Checkpoint 8h passed WHOLE at `d24d15a` and re-ran green after the Record B flip
  (`9ff08fb`) at `b2ab1dc` (tracker `09018dd`). ADR-0198 (Record B) is accepted (D21-139); ADR-0197 (Record A) stays
  proposed until `s8j`'s probes; D21-128 stays open (§8i.1).
- *(dated 2026-10-02, s8i build)* **`s8a`'s split, the review-input seam and the render module** are measured and written into §8i.8
  before any Rust edit. `CS` = `rk-ports`. The seam is reachable without the kernel naming the type, so §8i.8's
  remaining STOP does not fire. One unforeseen defect is measured there and raised: Checkpoint 8i (8)'s kernel type
  scan fires on two string-literal code lines at HEAD.
- *(dated 2026-10-02, s8i build)* **The corpus** is detached at the pin `61f590e`, and its `main` is at `285e011`, not `f39b2f8` (§8i.0
  item 2's dated note). The build route is D21-140 item 2's private copy and patch (§8i.3).
- *(dated 2026-10-02, s8i build)* **Halt 2**: pin 2.1.278, the 2026-09-24 reading 2.1.281, now 2.1.288 (§8i.0 item 8's dated note; §8i.5).
- *(dated 2026-10-02, s8i build)* **Spend**: S-BIN's cap is US$90.00 total (D21-140 item 1, amending D21-105 item 2's US$30 for S-BIN only).
  The spend table is in §8i.5, and B4 does not fit at its bound.
- *(dated 2026-10-02, s8i build)* **`spikes/FINDINGS.md`** gains F41's and F42's rows (§8i.0 item 10; §8i.7).
- *(dated 2026-10-02, s8i build)* **Ids reserved** (each is the next free at writing; if one is taken first, the next free id is used and
  this line takes a dated note): ledger ~~**D21-141**~~ **D21-143** *(dated 2026-10-03, s8i build)* D21-141 and D21-142 were taken first; D21-143 is written OPEN on 2026-10-03 for this stage's S-BIN entry, which holds the go-ahead lines;
  ADR **0199** for the post-dispatch `Binary` record, NOT written in this build; finding **F43** for the S-BIN draft,
  also post-dispatch. `SBIN_LEDGER`, `BIN_ADR` and `SBIN_F` are written into Checkpoint 8i (0) by dated edit, each
  on the day its record is written (D21-47), never before.

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
the kernel or the adapters."* (`s8a`'s record supersedes that decision in part, and in `s8a` §8a.2's table S-BIN
"now blocks the `Binary` arm and its body only"; the scope rule stands.) And the type may not be finalised any other way — ADR-0008's forbids: *"`ChangeSummary`
may not be designed and locked purely against a synthetic or paper fixture; it must be designed against the real UE
project's actual artefacts."* So if S-BIN cannot run, the variant stays sealed and this stage does not close (§8i.6).

**This stage decides nothing about the method.** METHOD-S binds its own changes: *"Any later change is disclosed here
with the date and the records it post-dates"*, and for this spike specifically *"no seed is changed after a
dispatch"*. What the 2026-09-09 entry left open is settled **before the first dispatch** by one dated amendment
(§8i.2), whose fixture decisions are the operator's. The records this stage writes are: that amendment; the S-BIN
results in the corpus repository; a finding draft; one new ADR for the `Binary` variant (next free id at writing — no
id is pre-assigned here); dated register and design notes; and ledger lines for each go-ahead.

**This is the first stage of the Step 8 sequence that spends against the real account.** D21-42 states the gate:
*"The spend gates (the probes, the real runs and S-BIN's dispatches) remain behind D21-40 decision 2's
fixture-proof condition, and each real run is still a separate operator go-ahead in the moment."* D21-47 adds:
*"Every real run gets its own block and its own go-ahead."* Every batch in §8i.5 is a separate go-ahead, given in
the moment and written to the ledger before the batch starts.

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
   *(dated 2026-10-02, s8i build)* Re-measured at stage open, read-only; the item's own text is left as written. The corpus working
   tree is still **detached at the pin `61f590e`** (`61f590e51cbb00301c310e46aab615563a7787b8`) and clean (`git -C
   D:/repos/redkiln-corpus status --porcelain=v1` prints 0 lines). Its `main` is now at **`285e011`**
   (`285e011c70cfa0fc92d91089792834846d99bcf7`, *"S-SEV: Jev's runs (rater of record, D21-58), both sealed keys, the
   Jev-filled sheets and the scored result"*), not the `f39b2f8` stated above. `git log --oneline 61f590e..main` lists
   four commits: `f39b2f8`, then the three S-SEV commits `a2f4fab`, `1b3e102` and `285e011`. Both `f39b2f8` and the pin
   are ancestors of `main` [measured, `git merge-base --is-ancestor`, rc 0 each]. `runner/src/sbin.rs` exists at
   neither (`git cat-file -e main:runner/src/sbin.rs` exits 128). §8i.3's *"The operator puts the corpus on `main` and
   clean before B0"* is a **deferred operator gate**, and under D21-140 item 2 the build never writes to the corpus or
   its `.git` (§8i.3).
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
   *(dated 2026-10-02, s8i build)* Re-read at stage open. `spikes/results/instrument-checks/RUNBOOK-instrument.txt` still reads
   `2.1.278 (Claude Code)`, and the 2026-09-24 reading above was `2.1.281`. `claude --version` now reads **`2.1.288
   (Claude Code)`**, from `/c/Users/ryanm/.local/bin/claude` [measured 2026-10-02 from the build session's shell, which
   agrees with the operator's reading handed to this build; the operator's plain-terminal reading is the one of record,
   and under `nd.sh` the fake prints the pin instead]. Halt 2 is still live, and it is recorded, not silently
   continued. The CLI has moved twice since authoring, which is what N22's freeze before B1 is for.
   `~/.claude-personal/settings.json` has no `DISABLE_AUTOUPDATER` row (`grep -c DISABLE_AUTOUPDATER` prints 0). The
   operator writes that row, because it is user configuration, and it is a deferred gate before B1 (§8i.5).
9. **The registered decision rule is not total.** METHOD-S §3.4's three branches cover *"2/2 detection and ≤1/3 false
   positive"*, *"A seed missed and its finding would have needed a field the summary lacks"*, and *"Both seeds missed
   with the summary fields cited but not the asset"*. Two outcomes reach none of them: **2/2 detection with ≥ 2/3
   false positives**, and **a seed missed where the reviewer names no missing field and cites no summary field**.
   Golden constraint 10: a bar that cannot decide an outcome is reported as not deciding it, and that is registered
   before the data (§8i.2 item 9). D21-43 N21 registers both as undecided: if either occurs, the `Binary` variant
   takes the human-gate route. *D21-48 (2026-09-24):* a third shape reaches no branch either: exactly one seed
   missed, with summary fields cited and no missing field named. D21-48 X3 registers it the same way, as
   `undecided-partial`, *"routed to the human gate, as D21-43 N21 routes the other two"* (§8i.2 item 9).
10. **`spikes/FINDINGS.md`'s index stops at F40** although `F41-DRAFT.md` and `F42-DRAFT.md` exist, and the index says
    *"A later F-number is added here the day its draft is written."* **The 2026-09-24 revision brief assigns the
    repair to this stage**, since it writes a FINDINGS row. The work commit adds F41's and F42's rows beside S-BIN's
    (§8i.7), and guard (2) asserts that every draft is indexed.
    *(dated 2026-10-02, s8i build)* Done at stage open: the index gains the F41 (`S-PORTS`) and F42 (`S-GRAMMAR`) rows, each headline
    condensed from its draft's `## F<n> —` line (§8i.7). Guard (2)'s `idx_missing`, run read-only over the edited
    index, prints nothing. The S-BIN row (reserved F43) is written with its draft, after dispatch.
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
  *(dated 2026-10-02, s8i build)* Held. `PROGRESS.md` row 8h and D21-138 record Checkpoint 8h passed WHOLE at `d24d15a`, at 1404/0/9 over 44
  result lines, from the operator's plain Git Bash. Row 8h also records the re-run green after the Record B flip commit
  `9ff08fb`, at `b2ab1dc` (tracker `09018dd`).
- **`s8a`'s split is on the tree.** The `Text` variant is finalised and the `Binary` slot is sealed, under the names
  and in the crate `s8a`'s record gives them. `s8a`'s file, read at authoring, says *"`ChangeSummary` is defined only
  in `rk-ports`, as an enum with two arms"*, calls the payloads `TextChangeSummary` and `BinaryChangeSummary`, and
  pins the sealed arm with `change_summary::binary_arm_is_not_constructible_outside_rk_ports`. At this stage's open, measure both and write them into §8i.8 by dated
  note before the first Rust edit; every test name below that says `change_summary::` follows the type into that crate.
  *(dated 2026-10-02, s8i build)* Measured and written into §8i.8 before any Rust edit: `pub enum ChangeSummary {` and
  `pub struct BinaryChangeSummary(pub(crate) ());` in `crates/rk-ports/src/toolchain.rs`, and the sealed-slot test
  `change_summary::binary_arm_is_not_constructible_outside_rk_ports` with its `trybuild` case
  `tests/ui/change_summary_binary_not_constructible.rs`. So `CS` = `rk-ports`.
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
  *(dated 2026-10-02, s8i build)* Not yet confirmable. The dry run is built in B0 (§8i.3), so its sha256 does not exist yet. The B1
  go-ahead line is prepared for the operator in the reserved ledger entry ~~D21-141~~ D21-143, naming Checkpoint 8h's recorded
  hash and the dry run's sha256 once the dry run has run. Nothing this build writes is a go-ahead.
  *(dated 2026-10-03, s8i build)* The entry is **D21-143**: D21-141 and D21-142 were taken first, and D21-141's id note reads
  *"That entry takes the next free id when it is written"*. Its B1 line is drafted unsigned, in §8i.5's fixed form,
  naming Checkpoint 8h `d24d15a` and amendment `285b659fbcd3cd4d424541c4211e3b6398ea26ed`. The dry run's sha256 is a
  placeholder, owed by the operator's dry run, because this build could not run it (§8i.3's dated note).
- **Operator present, foreground.** D17-5 is headed *"The real dispatch runs in this session, in the foreground"*
  (`DECISIONS-2026-09-17`). Only its foreground, operator-present half carries here. Its *"in this session"* does
  not: every S-BIN launch is made from a plain terminal, never a Claude Code session, because the launcher refuses a
  session's variables (s8h §8h.7; §8i.5). Every UE automation run is the operator's hand (§8i.4).
  *(dated 2026-10-02, s8i build)* The build runs in an AFK Claude Code session, so it prepares and never runs. Every S-BIN launch, every
  capture session, every UE automation run, and every formal checkpoint from `s8h` on is the operator's. Each is run
  from a plain terminal, behind its own go-ahead (D21-140 item 1: *"Every S-BIN launch is the operator's, made from
  a plain terminal and behind its own go-ahead line"*, and *"No subagent dispatches."*).

### 8i.2 The pre-dispatch amendment — what it must settle, and whose decision each item is

One dated entry appended to METHOD-S §7, opening with the literal text **`s8i pre-dispatch amendment`** in its bold
lead (Checkpoint 8i finds its commit by that string), committed to this repository **before B1**. Like the 2026-09-09
entry it changes nothing in METHOD-S §3 — *"the question, the decision rule, the prediction and the sizing stand
exactly as registered"* — and Checkpoint 8i asserts that §3's bytes at HEAD equal §3's bytes at the amendment's
commit, and that every S-BIN record post-dates it. Four items were **(operator)** fixture decisions that the entry
left open or the measurement exposed. The operator took all four on 2026-09-24 (D21-43 N19–N22). The amendment
records each decision and cites its ledger line. It does not re-argue them.

*(dated 2026-10-02, s8i build)* **Written, uncommitted.** The amendment is appended to `spikes/METHOD-S.md` §7 as one entry whose bold lead
opens with the literal. The file begins with HEAD's bytes and the literal occurs once. It covers items 1–12, and it
ends with a deferred-extensions list naming what is appended later: seed 5's replacement R (the candidates, the
pick, its clause, its leak tokens and `$ARGUMENTS`), the item-7 ceiling, `SBIN_NEED_BYTES`, the greenness results
and the pin report's SHA-256, and B4's amendment if one is needed. `review_root` is fixed now as `D:/sb`. Item 10
registers the driver's USD cap at 90.00, citing D21-140. The entry must be committed before seed 5's candidates are
examined, because it registers item 12's rule. Three choices this file left open are made in the entry and
disclosed there:
- the `$ARGUMENTS` rule adds no map;
- the item-7 ceiling is fixed as a formula, 3600 + `W_max` rounded up to 300 s;
- `report_ref` and `log_ref` point into the review tree (§8i.4's dated note).

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
     operator's yes/no on whether the capture shows the item's subject. *(dated 2026-10-06, s8i build, R10FIX-8i; s8i
     refuting round 10's minor)* D21-149 and D21-150 add two values, both with `sha256=none`: `n/a` for an item whose
     every asset `capture_scene_preview` rejected (`captures: []`; items 2 and 3, so item 3 has no capture), and
     `stalled` for a session stalled on the instrument, at most one per item, ahead of its binding line (item 5).
     METHOD-S's D21149-8i-A extension, items 2 and 3; Checkpoint 8i (4)'s `caplog_ok` (§8i.4 step 5's dated notes).

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

   *(dated 2026-10-03, s8i build, D21147-8i)* **One generated file is written back after the backlog strip (D21-147
   item 1).** The operator ruled on 2026-10-03: *"After the existing whole-directory strip, one generated file is
   written at `_workspace/backlog/game_foundation_backlog.json`, byte-identical in all five trees"*. The set above
   keeps its entries, and `_workspace/backlog/` is still stripped whole. Right after that removal, the strip writes
   the registered stub back: 109 bytes, one line, LF, sha256
   `211a4dcf7be9eb68ef73dfcb6e67a832b1713f1e3c9aa8f148fb52df189f8787`, holding exactly `8.3.5a`, `8.3.5b` and `8.5.8`,
   each `todo`, and no other field or story. METHOD-S's D21147-8i extension registers the bytes. It also records that
   they match the shape the item-2 and item-3 `NoInertCapability` specs read, and discloses the stub as fixture content:
   at item 3's commit, `8.3.5a` and `8.5.8` read `done`. The stub holds no registered leak token. The driver writes and
   checks it (§8i.3's D21147-8i note).
4. **The fix is unreachable** (F17; METHOD-S §0's `fix_unreachable` pre-flight). F17's measured failure: a reviewer
   with `Bash` *"checked the fix out, reviewed the **real** code"*. So each review tree is a **fresh single-commit
   repository** holding only that tree: a neutral commit message, no remote, no other ref, and neither the fixed
   pointer blob nor its LFS object present. A reviewer that reads or runs `git` against a path outside its review
   tree voids the record as `void: out-of-tree` — METHOD-S §0, *"a void is never a miss"*.
   - **No tree is reused.** The single-commit repository is built once and frozen, and nothing runs in it. The
     greenness runs, the capture sessions and every Opus dispatch each run in their own byte copy of it, at a path no
     earlier run used, and each copy is deleted afterwards. A copy is not a clone, so it gains no remote. A Claude
     Code session keys its transcripts and auto-memory by its working directory's path, so a reused path would hand a
     dispatch the memory of a capture session or of an earlier pass. A reviewer with `Bash` can also change the tree
     it is given. §8i.4 registers the copies, and the checks the driver makes on each one before it spawns.
   - **The answer-bearing paths outside the tree are named** in the amendment, so that the out-of-tree rule is
     checked against them by name. They are `D:/repos/redkiln-rs` (METHOD-S §7 states the seeds, and this file
     names them), `D:/repos/redkiln-corpus` (`fixture.json` and the rendered prompts), `D:/repos/BrittonGames` (the
     fix itself), the account's `projects/` (the capture sessions' transcripts) and the review root's other copies.
     Claude Code's permission rules cannot fence `Bash`, so prevention is not claimed. The defence is detection, the
     void. For BrittonGames' two uncommitted paths it is also the snapshot series (§8i.5).
5. **The instrument, named.** The driver and its subcommand (§8i.3); the arm label, carrying every variant (METHOD-S
   §6: *"Arm labels carry every instrument variant"*); model and effort as E3's (`--model opus --effort xhigh`), with
   the model id **observed from each init line** recorded — the set must report one observed id, as it must one CLI
   version, and a record reporting a second id halts the set (§8i.6);
   the card's tools `Read` and `Bash` (METHOD-S §3.3: *"the reviewer has `Read` and `Bash` in a disposable clone with
   the engine available"*); `--strict-mcp-config`; METHOD-S §0's pre-flight fields recorded on every record — `card_adopted` (*"adoption
   verified by tool-set equality"*), `api_error_status` (*"platform errors named as such"*, a void and never a
   miss) and `fix_unreachable`, with `node_modules_linked` stated as not applying to a UE tree; METHOD-S §2's typed schema as amended 2026-09-06, scored under the
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
9. **The outcomes the rule does not decide. Taken, D21-43 N21:** *"**Registered as undecided before dispatch.** If
   either occurs, the `Binary` variant takes the human-gate route."* The amendment names N21's two outcomes (§8i.0
   item 9). `undecided-fp` is 2/2 detection with ≥ 2/3 false positives. `undecided-silent` is a seed missed, with no
   missing field named and no summary field cited. Each is reported as *"not decided by the registered rule"*. The
   finding's pins carry each pass's outcome (§8i.7), and Checkpoint 8i asserts that the record's route follows from
   them (§8i.8). It computes each outcome from the records rather than trusting the pins (§8i.9, guard (4)).
   **The third undecided outcome. Taken, D21-48 X3:** *"**Registered before dispatch as a third undecided outcome,
   `undecided-partial`**, routed to the human gate, as D21-43 N21 routes the other two."* The shape was found at
   the 2026-09-24 adversarial-review fix (D21-47). It is exactly one seed missed, with summary fields cited and no
   missing field named. The seed is counted after any B4 re-run, so a seed whose named field detected on its re-run
   is not missed. Branch 3 needs *"Both seeds missed"*, branch 2 needs a named field, and N21's two do not cover
   the shape. So the amendment names `undecided-partial` beside `undecided-fp` and `undecided-silent`, in the same
   form, and reports it as *"not decided by the registered rule"*. It is registered in the amendment's commit,
   before B1, like the other two. The amendment therefore registers three undecided outcomes. Every outcome
   Checkpoint 8i computes is one of six registered values, and nothing is computed as `unregistered`. Guard (4)'s
   synthetic record sets reach each of the six (§8i.9).
   *D21-48 (2026-09-24): this replaces the owed operator decision that stood here.*
   **Branch 2's named field is a judgement, and it is named as one** (METHOD-S §0: *"reported with the judgement it
   needs named"*). After each pass's audit the operator reads each missed seed's digest and writes
   `results/s-bin/missing-fields.jsonl`, one line per missed seed: `{pass, item, missing_fields: [{field, quote}]}`.
   Each quote is verbatim from a `claim` or `repro` in that record's digest. The file exists, possibly empty, when no
   seed was missed. Checkpoint 8i re-finds every quote.
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

    *(dated 2026-10-02, s8i build)* **The pool does not count 29.** The 2026-09-09 entry recorded its 29 without a command. The
    amendment registers this one: non-merge commits reachable from the pin that change at least one `.uasset` or
    `.umap`, with every other path inside item 3's strip set. Under it the pool counts **148** of 575, all five
    items included [measured 2026-10-02 by a count-only script that printed no candidate id or message]. As this
    item provides, the difference is recorded in the amendment, and the definition is not narrowed to reach 29.

A seed that goes red at its greenness check, the replacement included, is handled as registered: *"If a seed goes red it is replaced from the
remaining 27 asset-only commits and this amendment is extended, never quietly edited."* Extension happens before B2;
after the first Opus dispatch no seed, item, rendering or ceiling moves, with **one registered exception, B4**. METHOD-S §3.4's own
branch 2 says *"add the field, re-run that item"*, so a B4 amendment adds the named field to that item's rendering for
its re-run only, committed before B4's go-ahead. The pass's original records stand, and nothing else moves. A change that turns out to be needed after
that point discards the set, as METHOD-S's own 2026-09-05 entry did (*"after 10 dispatches, all of which are
DISCARDED"*), and is re-registered.

### 8i.3 The instrument — an S-BIN driver in the corpus runner, test-first

**Where.** `D:/repos/redkiln-corpus/runner`, a new module `src/sbin.rs` behind a new subcommand (named at writing and
recorded in the amendment). Every write to `D:/repos/redkiln-corpus` is the operator's, or made on the operator's
go-ahead in the moment — `s7b` §7b.11 precondition 7 reserved *"the corpus write and the pin edit"* to the operator,
and `f39b2f8` is the precedent for a results commit that leaves `CORPUS_PIN` alone. The operator puts the corpus on
`main` and clean before B0 (it is detached at the pin today, §8i.0 item 2). Halt 1 and golden constraint 3 apply:
`cargo build` in `runner/` before every use of its binary.

*(dated 2026-10-02, s8i build)* **The build route, taken by the operator (D21-140 item 2):** *"The driver (`runner/src/sbin.rs`) and
`audit-s.py bin` are built and tested in a private `git archive` copy of the corpus's `main` under `target/`."*,
*"The operator receives a patch with its apply and commit commands."* and *"Nothing writes to
`D:/repos/redkiln-corpus` or its `.git`."* The sentence above, *"Every write to `D:/repos/redkiln-corpus` is the
operator's"*, holds unchanged. Applying and committing the patch is the operator's write, made after the operator
has put the corpus on `main` and clean (§8i.0 item 2's dated note). B0's gate, *"operator go-ahead for the corpus
writes"*, is therefore that apply and commit, and it is a deferred operator gate.

*(dated 2026-10-02, s8i build)* **The fixture and pre-spawn half, built (package S8I-P4).** The private copy is
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P4-corpus`, a `git archive` of corpus `main` at `285e011`. It is not under
`target/`, because a package manifest anywhere under `D:/repos/redkiln-rs` finds this workspace's root, which does
not list it, and cargo refuses (*"current package believes it's in a workspace when it's not"*, the root
`Cargo.toml`'s own comment). The orchestrator's note allows `afk-tools/sp/s8i/` for this case. Nothing wrote to
`D:/repos/redkiln-corpus` or its `.git`. `runner/src/sbin.rs` holds 13 of the table's 25 tests: 1–5, 16–18 and
21–25. Each was watched red for its stated reason, then green, and each table mutant flipped its test (13/13, from
a Python driver, with the sha256 restore verified). The other 12 tests (6–15, 19, 20) are the dispatch half's and
are not built yet, so B0's `25 passed` line cannot pass until they are. The subcommand is `sbin-probe`. It reads
`runner_commit` and then refuses before any spawn, because the dispatch half does not exist yet. `runner_commit`
reads `git status --porcelain --untracked-files=all -- runner/ ':(exclude)runner/target/'`. The exclusion is there
because the corpus has no `.gitignore`, and Halt 1's `cargo build` creates `runner/target/` as an untracked
directory. On the copy, the whole runner suite is 147 passed and 1 failed. The failure,
`p7::tests::the_driver_matches_the_committed_catalog`, predates this work: it reads `../p9/catalog/root-cause.toml`,
a path relative to `spikes/runner` in this repository, and it fails the same way on the unmodified copy (134 passed
and 1 failed before `sbin.rs`). The operator's patch is `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P4-sbin-driver.patch`
(sha256 `706da895…51cb0`). It adds `runner/src/sbin.rs` (sha256 `213040ff…4e04`) and 6 lines to `runner/src/main.rs`.
It applies cleanly to a fresh `main` extract, and the result is byte-identical to the copy. Applying it is the
deferred B0 gate above, made after the corpus is on `main` and clean: `git -C D:/repos/redkiln-corpus -c
core.autocrlf=false apply --check <patch>`, then the same command without `--check`, then `git add
runner/src/main.rs runner/src/sbin.rs` and a commit. Later packages that add the dispatch half build on this copy,
or on the patch, so the operator receives one `sbin.rs`.

*(dated 2026-10-02, s8i build)* **The dispatch half, built (package S8I-P5).** It is built in a second private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P5-corpus`, made from P4's copy, so the operator receives one `sbin.rs`.
The operator's patch is now `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P5-sbin-driver.patch` (sha256
`6423a025…6e4bef`). It is cumulative against a fresh `main` extract at `285e011`: it adds `runner/src/sbin.rs` (sha256
`89473ef4…760d8017`, 4,325 lines) and P4's same 6 lines in `runner/src/main.rs`. It **supersedes** P4's patch, so apply
only this one. It applied cleanly to a fresh extract with `core.autocrlf=false`, and the result was byte-identical to
the copy. The apply and commit commands are P4's, with this patch's path. The table's other 12 tests (6–15, 19, 20)
now exist. Each was watched red for its stated reason with its guard unwritten, then green. Each table mutant
flipped its test: 12/12 from `s8i_S8I-P5_mutants.py`. P4's 13 mutants were re-run over the final bytes, so 25/25
flipped, and the sha256 restore was verified. B0's selection under `nd.sh`, run in the copy as the last reading,
printed `test result: ok. 25 passed; 0 failed`, and `nd_check` was clean. The whole runner suite reads 159 passed
and 1 failed. The failure is the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`. clippy
exits 0 with nothing for `sbin.rs`, and `sbin.rs` is rustfmt-clean. These choices were made in the driver and are
disclosed here:
- **Void order, every void before scoring.** The checks run in this order: timeout, `api-error`
  (`api_error_status` non-null), `no-init`, `api-key-source` (not `none`, or absent), `card-not-adopted` (init
  tools ≠ `e3::CARD_TOOLS`), `out-of-tree`, `no-result`, `non-zero-exit`, `no-structured-output` (exit 0 with no
  `structured_output`, which is Halt 3), then `schema: …` (§2's typed schema as amended, via `jsonschema`). A void
  record carries `detected` and `policy_verdict` as null.
- **`total_cost_usd` is never null.** A dispatch that reports no cost, such as a timeout or a crash, is charged the
  spend table's per-dispatch bound (5.00 Opus, 0.50 haiku), and `total_cost_source` reads `bound` rather than
  `reported`. A missing cost in an earlier record or probe is refused before spawning, never read as zero.
- **Out-of-tree.** ~~Every string in every tool input is audited, `Read` and `Bash` alike.~~ *(dated 2026-10-03, s8i build, R4FIX-8i: only what a tool reads is audited: a `Bash` command's words in read positions and path fields, never the digest, a `description` or other prose (METHOD-S's R4FIX-8i extension, item 2))* Each is folded (`\` to
  `/`, lower case) and checked by name against amendment item 4's roots: `/repos/redkiln-rs`,
  `/repos/redkiln-corpus`, `/repos/brittongames`, `/.claude-personal/projects` and `/.claude/projects`. It is also
  checked against any path under the review root, in its `D:/sb`, `/d/sb` and `/mnt/d/sb` spellings, other than
  the dispatch copy's own. Paths outside all of these, such as the engine install, are not out-of-tree.
  *(dated 2026-10-03, s8i build, R3FIX-8i)* Before that test, each string's separators are collapsed and its `.` and `..` resolved, and a
  relative path is resolved against the child's cwd: the copy, as moved by `cd` (METHOD-S's R3FIX-8i extension, item 1).
*(dated 2026-10-03, s8i build, R5FIX-8i)* Assignment values are audited and `$VAR` is expanded in read positions. A
  recursive read (`grep -r`/`-R`, `rg`, `find`, `ls -R`, `tree`, `du`, `git grep --no-index` or `git -C <dir> grep`, a
  `Glob` or `Grep` tool path) of a registered root or an ancestor of one reads that root. Spellings fold (`/c/`,
  `/mnt/c/`, `~`, `$HOME`, `$USERPROFILE`, `$TEMP`, `/tmp`) before every test, a POSIX path's `..` resolved first. The
  roots list gains the sweep-measured directories: `D:/r6c`, `D:/repos/redkiln-sev`, `~/.claude-personal`,
  `~/.claude-backup`, `~/AppData/Local/JetBrains`, `~/AppData/Local/Packages/Claude_pzs8sxrjxfjjc`, and under the temp
  directory `claude`, `tmp.uDZZzvzEF4`, `h.md`, `l143` and `ms0` (METHOD-S's R5FIX-8i extension, items 2 and 3).
  *"Paths outside all of these, such as the engine install, are not out-of-tree"* still holds for the runtime void.
  PROVISIONALLY, the child's own background-task output, `<temp>/claude/<copy slug>/<session>/tasks/`, is in-tree
  (that extension's item 4; ~~returned to the orchestrator as a blocker~~ *(dated 2026-10-03, s8i build, R6FIX-8i: ruled by D21-145, TAKEN
  PROVISIONALLY under D21-105, "That own-slug `tasks/` path is exempt, in every spelling"; it leaves to the operator
  a pre-spawn check for that directory and whether the shadow allowlist carries it; METHOD-S's R6FIX-8i extension,
  item 1)*).
- **Shadow out-of-tree audit.** *(dated 2026-10-03, s8i build, R5FIX-8i)* D21-144's allowlist runs over the kept raw
  transcript's tool uses, never the digest, as a shadow: it never voids, retries, counts toward a cap or halts. A
  read-position path outside the dispatch copy is flagged unless it is under the copy slug's `tool-results` (every
  spelling), `C:/Windows`, `C:/Program Files*`, Git Bash's `/usr`, `/bin`, `/etc`, `/dev/null` or `/mingw64`, or
  `~/AppData/Local/UnrealEngine` (listed provisionally until B1 measures the engine's paths). A temp path is allowed
  only where the same session wrote it first (the `Write` tool, a `>`/`>>` target, a `cp`/`mv` destination, a
  `$(mktemp)` variable). A read-position word with an unresolved `$VAR`, `${…}` or `$(…)` is flagged. The record
  carries `oot_shadow` `{verdict, flagged_reads, allowlist_version}` (version ~~`r5fix-8i-1`~~ `r6fix-8i-1` *(dated 2026-10-03, s8i build, R6FIX-8i)*). A flagged record is held
  unscored: `audit-s.py bin` shows it as `HELD:oot-shadow` and gives its pass `BAD-held` until the operator clears or
  voids it in `results/s-bin/oot-shadow-rulings.jsonl`, at the item-4 gate before B1 and before each later batch is
  scored (METHOD-S's R5FIX-8i extension, item 5). *(dated 2026-10-03, s8i build, R6FIX-8i)* A `cp`/`mv` into a directory (a trailing `/`, a last `.`
  or `..`, the temp directory or an ancestor of it, or several sources) writes `<dest>/<basename of each source>`, never
  the directory; a written path covers a read only below the temp directory; and input that reaches execution and that
  the audit cannot read (xargs's input from another program's pipe or an unstaged file, a shell's or interpreter's
  stdin from such a pipe) is flagged (METHOD-S's R6FIX-8i extension, items 2 and 3).
- **Cited fields** are read from the digest's findings only, never from the prose. A field counts when its name
  appears as the rendering writes it (dotted, indexed or in backticks), or when a finding names a path the summary
  attributes to it. The token table is `FIELD_TOKENS`. A finding naming an item's asset path cites `assets`.
- **The caps** are counted from `--out` (records) and `results/s-bin/probes.jsonl`, voids included: 18 Opus, 3
  haiku, and USD 90.00 per D21-140. The check runs before the copy is made. **The set-level version check** compares
  the CLI version read by `<claude-bin> --version` before each dispatch with the set's first record's `cli_version`.
  A record's `cli_version` is init's `claude_code_version` when present. If it differs from the reading before the
  spawn, the set halts after the record is written. **Halt 6** reads `tasklist` before the dispatch (a survivor
  refuses the spawn) and after it (a survivor halts the set, the copy is deleted, and its `tree_path` in the record
  keeps the path from being reused). A second `observed_model` ~~among valid records~~ *(dated 2026-10-03, s8i build, R1FIX-8i: among every record that carries one, voids included, as METHOD-S item 5 and Checkpoint 8i (4)'s `model` line read it)* halts the set. So does the same
  `(pass, item)` voiding twice.
- **The record** carries all 33 §8i.7 names *(dated 2026-10-03, s8i build, R5FIX-8i): 34, with `oot_shadow`)*, which `the_dispatch_cap_refuses_before_spawning` asserts against a
  list transcribed from §8i.7. It also carries `attempt`, `cli_version_before`, `init_tools`,
  `node_modules_linked` (null), `exit_code`, `timed_out`, `total_cost_source`, `out_of_tree`, `synthetic_capture`,
  `rendered_sha256`, `model` and `effort`. *(dated 2026-10-06, s8i build, D21149-8i-A: `synthetic_capture` is always false, because D21-149 item 3
  rules that the dry run no longer injects a synthetic capture; `capture_refs` carries each capture's `asset_index`)*
- **The fixture keys the dispatch reads**, owed by the package that writes `fixture.json`: per item, `slug`,
  `spec_path`, `report_path`, `ac_ids`, `asset_actions`, `build_exit`, `text_diff`, `seed_blobs` (seeded items, each
  path's `<seed>^` blob), `fix_blobs` and `fix_lfs_oids`; at the top level, `leak_tokens` (every registered token)
  and `ceiling_secs`. The driver refuses at dispatch any key it needs and does not find. Each review repository is
  read at `<review_root>/rev/item-<n>`.
- **The argv.** The registered argv is unchanged. Three flags are added, and their defaults keep it valid:
  `--redkiln` (default `D:/repos/redkiln`, E3's card source), `--arm` (default `b-typed-sbin`) and `--rerun` (B4
  only, and it must carry `b-typed-sbin-plus-<field>`). The registered dry-run argv has no `--timeout-secs`, so the
  dry run uses 3600. **For the dry run's stub (P8):** the driver first runs `<stub> --version` and needs a version
  token back, then replays the stream from stdout. A `.cmd` stub's argv goes through `cmd.exe`, whose command line is
  limited to 8,191 characters.
- **Not built here:** ~~the per-copy `SBIN_NEED_BYTES` free-space check (the batch pre-check's `df` covers each
  batch),~~ *(dated 2026-10-06, s8i build, D21149-8i-A: the per-copy check is built under D21-151, `dispatch_one` refusing before every dispatch
  copy; the batch pre-check stays as a second layer; this section's D21149-8i-A note)* `audit-s.py bin`, and the dry
  run itself.

**Named tests**, run as `cargo test --bin redkiln-spike-runner -- sbin::tests::<name> --exact`, each authored and
**watched red for the stated reason** before the code that turns it green (golden constraint 12's order). Each mutant
is authored and run **from Python, never the shell**, and must flip its test.

| test (`sbin::tests::`) | pins | mutant that must flip it |
| --- | --- | --- |
| `seed_is_confirmed_by_blob_oid_not_by_file_count` | the entry's *"each seed is confirmed by blob oid … and not by a file count"* | compare `git diff --stat` counts; *(dated 2026-10-03, s8i build, R2FIX-8i)* also `dispatch_one` recording `seed_blob_verified` true without checking, and not refusing a seeded item with no `seed_blobs` |
| `review_repo_holds_one_commit_and_no_fix_object` | §8i.2 item 4 | check the working tree only, not the object store and `.git/lfs`; *(dated 2026-10-03, s8i build)* also `commit_single` as plain `git add -A`, as `add -A -f` without the tracked-set check, without the index check, and force-adding every ignored file; *(dated 2026-10-03, s8i build, R1FIX-8i)* also `dispatch_one` checking no fix blob in its own copy |
| `text_diff_carries_source_and_test_paths_only` | §8i.2 item 6 | admit `_workspace/backlog/` |
| `asset_size_is_read_from_the_lfs_pointer` | §8i.2 item 6 | infer size from whether the pointer's size line moved; *(dated 2026-10-03, s8i build)* also read the pointer from the work-tree file, in `review_pointer`, `asset_entries` or `render_summary` |
| `warned_and_not_run_fold_into_passed_with_the_raw_report_referenced` | §8i.2 item 6 | drop `succeededWithWarnings` |
| `a_digest_failing_the_typed_schema_is_void_schema_not_a_miss` | METHOD-S §0 rule 2 | score the invalid digest |
| `exit_zero_without_structured_output_is_void_not_success` | Halt 3 | read exit 0 as a completed dispatch |
| `strong_detection_needs_a_blocking_finding_naming_the_seeded_path` | METHOD-S §3.4 primary | credit any blocking finding; *(dated 2026-10-03, s8i build, R2FIX-8i)* also `dispatch_one` scoring a seeded item as clean (`seeded_paths` empty) |
| `cited_fields_are_extracted_mechanically_from_the_digest` | METHOD-S §3.4 *"Recorded"* | read cited fields from the prose; *(dated 2026-10-03, s8i build, R3FIX-8i)* also `dispatch_one` passing no asset paths to scoring |
| `an_out_of_tree_read_voids_the_record` | §8i.2 item 4 | audit `Read` paths only, not `Bash` commands; *(dated 2026-10-03, s8i build, R2FIX-8i)* also `ran_automation` counting a command that only names the suite; *(dated 2026-10-03, s8i build, R3FIX-8i)* also no `..` resolution, no separator collapse, relative paths ignored, no `cd` tracking, and `dispatch_one` handing the audit no copy path or another review root; *(dated 2026-10-03, s8i build, R4FIX-8i)* also the digest or a `Bash` description audited, a subshell's `cd` persisted, the tool-results exemption dropped, a `cd` option read as its target, a `cd` after a keyword missed, a grep's pattern or `echo`'s operands audited, no trailing-punctuation trim, `popd` untracked, a heredoc's body not read or read as commands, `grep -f`'s file, an input redirect, a `Grep` path or `git -C`'s directory not read, the whole session directory exempt, the exemption tested before resolution, a grep option's value taken as the pattern, a `Bash` command's separators not collapsed, and `dispatch_one` handing the audit no account roots; *(dated 2026-10-03, s8i build, R6FIX-8i)* also `|` read as a list end; an `echo`/`printf`'s output not carried into the pipe, or not staged when redirected; xargs's input words dropped, its `-I` replacement not applied, or its `-a` or `<` file not read; a shell's `-c` string, staged script, staged `<` file, piped echo, heredoc or here-string not run; `source`, `eval` or a staged `./f` not run; an interpreter's staged script, staged `<` file or piped echo not read; the `Write` tool's content not staged; no depth bound on a run inside a run; an empty `(` dropping the pipe; *(dated 2026-10-03, s8i build, R7FIX-8i)* also each precommand not peeled (`timeout`, `nice`, `nohup`, `env`, `stdbuf`, `time`, `command`, `exec`, one row each); `timeout`'s `-k` value or its duration, `nice`'s `-n` value, `env`'s `-u` value or its assignments, or `stdbuf`'s `-o` value not skipped; `env -C`'s directory not applied; only the first precommand peeled; a precommand's own words not audited; xargs not running its `-I` command over unknown input; a heredoc fed to a shell also read as data; a here-string to another program, or on a line with no program, not read as data |
| `api_key_source_other_than_none_voids_before_scoring` | D21-42 17a | skip the init check |
| `a_version_move_between_dispatches_halts_the_set` | Halt 2 | compare against the floor, not the set's first version; *(dated 2026-10-03, s8i build, R1FIX-8i)* also, on `dispatch_one`'s path: the set check reading no records, no Halt 2 when init's CLI differs from the pre-read, no halt on a second observed model, the first model read from valid records only, and the model compared only when this dispatch is valid; *(dated 2026-10-03, s8i build, R2FIX-8i)* also probe's loop reading the version once per run, ignoring a pre-spawn or a post-record halt; `dispatch_one` skipping the launch-record check, admitting no launch record or a different version, or recording no launch version; and the launch record read from the first of two `CLI_VERSION_BEFORE` lines |
| `the_dispatch_cap_refuses_before_spawning` | §8i.5 caps | check the cap after the child is spawned; *(dated 2026-10-03, s8i build, R1FIX-8i)* also `cap_state` skipping voided records, and `fixture.json`'s `ceiling_secs` ignored, defaulted to `--timeout-secs`, or accepted when it differs; *(dated 2026-10-03, s8i build, R2FIX-8i)* also a cost-less dispatch charged 0.00, not the bound; *(dated 2026-10-03, s8i build, R5FIX-8i)* also the record without `oot_shadow` |
| `resume_is_keyed_per_arm_item_pass` | METHOD-S §0 | key on `item` alone; *(dated 2026-10-03, s8i build, R1FIX-8i)* also `dispatch_one` resuming over no records, and no halt in the run that writes an item's second void; *(dated 2026-10-03, s8i build, R2FIX-8i)* also probe's loop (`run_items`) ignoring a post-record halt |
| `a_surviving_unreal_process_halts_before_the_next_dispatch` | Halt 6 | skip the process check |
| `stripped_path_set_is_identical_across_all_five_trees` | §8i.2 item 3 (D21-43 N20) | strip each tree by its own item's diff; *(dated 2026-10-03, s8i build, D21146-8i)* also the launcher not in the registered set (D21-146 item 1); *(dated 2026-10-03, s8i build, D21147-8i)* also the stub not written, written before the removal or at other bytes, the registered stub's bytes moved, or a further story admitted (D21-147 items 1 and 2) |
| `review_tree_carries_no_registered_leak_token` | §8i.2 item 3 | scan `_workspace/backlog/` only; *(dated 2026-10-03, s8i build, R2FIX-8i)* also `dispatch_one` not refusing an empty token list, and scanning its copy for the compiled item-4 token only |
| ~~`every_item_carries_a_capture_reference_or_rendering_refuses`~~ `captures_name_their_asset_index_and_a_rejected_asset_has_none` *(dated 2026-10-06, s8i build, D21149-8i-A: inverted and renamed under D21-149 item 3)* | §8i.2 item 2 (D21-43 N19) *(dated 2026-10-06, s8i build, D21149-8i-A: as D21-149 item 1 amends it: a rejected asset carries no capture, an all-rejected item carries `captures: []`, a capture names its asset by explicit index)* | render captures for the seeded items only; *(dated 2026-10-03, s8i build, R1FIX-8i)* also F1's hand-written brief (E3's wording changed, no truncation), B4's rendering written over the pass's, B4's field not added, and a B4 field replacing one the summary renders; *(dated 2026-10-06, s8i build, D21149-8i-A)* also an item with no capture refusing rendering (the rule D21-149 inverts), a capture named by position not by its index, its path not matched to its index, a repeated or out-of-order index, an index past the item's assets, a synthetic capture rendered or recorded for an empty list, `synthetic_capture` true, the empty list rendered with a marker, the record's asset index dropped, and the seeded-only row re-anchored on the new code |
| `card_not_adopted_voids_before_scoring` | METHOD-S §0's `card_adopted` | score a record whose init tool set differs from the card's |
| `a_platform_error_is_recorded_as_api_error_status_and_voids` | METHOD-S §0's *"platform errors named as such (`api_error_status`)"* | record a platform error as a scored miss |
| `each_dispatch_runs_in_a_fresh_copy_at_a_never_used_path` | §8i.2 item 4, §8i.4 | reuse the previous dispatch's path; *(dated 2026-10-03, s8i build, R1FIX-8i)* also a B4 re-run's copy path or raw stem without its arm, a re-run with no valid missed original or after a detected one, the arm and `--rerun` disagreeing, and any field name accepted; *(dated 2026-10-03, s8i build, R3FIX-8i)* also a B4 re-run with no `--only-item` run over the default five items, and an `--only-item` outside 1–5 accepted |
| `a_dirty_or_mismatched_copy_refuses_before_spawning` | §8i.4's pre-spawn checks | read tracked files only, not untracked and ignored ones; *(dated 2026-10-03, s8i build, R1FIX-8i)* also `dispatch_one` ignoring the pre-spawn miss; *(dated 2026-10-03, s8i build, D21146-8i)* also the executable-path check ignored or never reported (D21-146 item 3); each of `.bat`, `.cmd`, `.ps1`, `.psm1`, `.sh`, `.bash` and `.py` not read (one row each); the extension matched case-sensitively; Python read whole, every string or any string standing alone blanked, comments read as code, strings not lexed, or a triple quote ended at its first quote; no spelling fold; the sweep's drive or home roots not checked; a UTF-16 file not decoded; the walk not descending; *(dated 2026-10-03, s8i build, D21147-8i)* also the stub check ignored or never reported (D21-147 item 2); a copy with no backlog directory passing; a second file beside the stub admitted; the stub's bytes not compared; CRLF folded, a missing LF or a byte-order mark tolerated; the stub compared as JSON |
| `an_account_project_dir_for_the_copy_path_refuses_before_spawning` | §8i.4 (transcripts and auto-memory are keyed by path) | check `~/.claude` only, not `~/.claude-personal`; *(dated 2026-10-03, s8i build, R1FIX-8i)* also exercised through `dispatch_one`, whose miss the row above's mutant removes; *(dated 2026-10-03, s8i build, R2FIX-8i)* also the production account roots without `~/.claude-personal` |
| `a_dirty_runner_tree_refuses_before_spawning` | §8i.7's `runner_commit` | record `runner_commit` without reading `runner/`'s status; *(dated 2026-10-03, s8i build, R1FIX-8i)* also `dispatch_one` recording `HEAD` without reading `runner/`'s status; *(dated 2026-10-03, s8i build, R5FIX-8i)* also `runner_commit` reading `HEAD`, not the commit that last changed `runner/` |
| `every_test_spawns_only_an_injected_stub_never_claude` | the stub rule below | the helper returns the default `claude` |
| `an_assignment_carried_path_voids` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144 ~~(a)~~ *(dated 2026-10-03, s8i build, R6FIX-8i: Ruled item 1, "assignment tracking and `$X` expansion, with assignment values audited")* | an assignment's value not audited; no `$VAR` expansion |
| `a_recursive_read_of_an_ancestor_voids` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144 ~~(b)~~ *(dated 2026-10-03, s8i build, R6FIX-8i: Ruled item 1, "the ancestor rule")* | grep's `-r` ignored; `rg` not recursive; `find`, `ls -R`, `tree`, `du` not walked; `git grep --no-index` or `git -C <dir> grep` not walked; no walk of the cwd; a root read only when named exactly; the account roots' `projects/` not walked; a `cd` target not folded |
| `a_tool_glob_or_grep_on_an_ancestor_voids` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144 ~~(b)~~ *(dated 2026-10-03, s8i build, R6FIX-8i: Ruled item 1, "the ancestor rule")* | a `Glob`/`Grep` tool path not walked; a `Glob` pattern's literal directory not walked; `Glob`'s path read as a plain path |
| `a_sweep_measured_root_voids` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144 ~~(d)~~ *(dated 2026-10-03, s8i build, R6FIX-8i: Ruled item 1, "named roots taken from a **measured read-only sweep**"; and D21-145)* | the sweep's roots not checked by name; each sweep root dropped (one row each); `$TEMP`/`$TMP` or `/tmp` not folded; a POSIX path folded before its `..`; the own task-output exemption dropped, or widened to the whole slug; *(dated 2026-10-03, s8i build, R6FIX-8i)* also `~`, `$HOME`, `$USERPROFILE`, `/x/` or `/mnt/x/` not folded for the own task output (D21-145's "in every spelling", with a home the temp directory sits under); *(dated 2026-10-03, s8i build, R7FIX-8i)* also a shell's `-c` string, or `eval`'s operands, also audited as an operand word (the own task output inside `bash -c`, the `timeout 300 bash -c 'while …'` wait and `eval`) |
| `tool_results_c_and_tilde_spellings_are_exempt` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144 ~~(c)~~ *(dated 2026-10-03, s8i build, R6FIX-8i: Ruled item 1, "one spelling fold applied before every containment test")* | `~`, `$HOME`, `$USERPROFILE`, `/mnt/x/` or `/x/` not folded; no fold before the containment tests; *(dated 2026-10-03, s8i build, R7FIX-8i)* also a shell's `-c` string, `eval`'s operands or xargs's command also audited as operand words; no operand exempt from the operand audit; a heredoc or here-string fed to a shell also read as data, or not marked as run; xargs not running its command over unknown input |
| `the_shadow_audit_flags_reads_outside_the_allowlist` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144, the shadow audit | the shadow never flags; each allowlist entry dropped (one row each); a trailing `*` read as a boundary; `tool-results` or an in-copy read flagged; the verdict always clear; the shadow computed over no tool uses |
| `a_temp_read_written_earlier_in_the_session_is_not_flagged` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144, the temp rule | temp allowed wholesale; a write not recorded; a `>` target, a `cp`/`mv` destination or the `Write` tool's path read as a read; a `mktemp` variable untracked; a descriptor before `>` read as a word; *(dated 2026-10-03, s8i build, R6FIX-8i)* also a directory destination recorded as written; a write to the temp directory itself covering its subtree; the temp directory, a trailing `/`, a last `.` or `..`, or several sources not read as a directory destination; the source's basename dropped |
| `an_unresolved_var_in_a_read_position_is_flagged_not_voided` *(dated 2026-10-03, s8i build, R5FIX-8i)* | D21-144, the shadow's unresolved rule | an unresolved variable not flagged; a heredoc body's variable flagged; a bare or exported assignment untracked; a subshell's variables outliving it; a single-quoted `$` read as a variable; *(dated 2026-10-03, s8i build, R6FIX-8i)* also `||` read as a pipe; xargs input, or a shell's or interpreter's piped stdin, that the audit cannot read not flagged; a sub-run's variables outliving it |
| `a_volume_under_sbin_need_bytes_refuses_before_the_copy` *(dated 2026-10-06, s8i build, D21149-8i-A)* | METHOD-S item 4's *"Before each copy, the free space on the root's volume must be at least `SBIN_NEED_BYTES`"* (D21-151) | the check removed; the boundary read as `>`; an unreadable reading passing; the check made after the copy; the corpus's volume read, not the review root's; the registered figure moved; `df`'s header unchecked, or a further line admitted; the check skipped on a retry, or on a B4 re-run |

**No driver test reaches a real `claude`.** The runner's global `--claude-bin` defaults to `claude`. So every test takes
its binary from one helper, `stub_claude()`, which returns a stub path and asserts that its file name is neither
`claude` nor `claude.exe`, and no test reads the default. The runs also go through D21-47's no-dispatch wrapper. A test
that bypassed the helper would then find no real `claude` to spawn, and the trap and the fake's log would show it. The
wrapper is D21-47's shared helper, `runbook/tools/nodispatch.sh`, which B0 below and Checkpoint 8i (0) both source. The
Python mutant driver runs once per mutant as `nodispatch python <script> <id>`, so every cargo it spawns inherits the
wrapper, and `nd_check <id>` follows each.

```bash
# B0: the driver's tests, and every red-first and mutant run, under the no-dispatch wrapper (D21-47). From a Git Bash
# OUTSIDE Claude Code, in a subshell so that a FAIL never closes the terminal.
(
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: run this from a Git Bash outside Claude Code"; exit 1; }
[ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is set — open a fresh terminal"; exit 1; }
# D21-47's shared wrapper: a compiled fake claude first on PATH, every other claude directory dropped, the trap, and
# REDKILN_LAUNCH_RECORD unset in each `nodispatch` child. nd_setup proves each part by a control.
test -f runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh is absent — D21-47's shared wrapper"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh"; exit 1; }
nd_setup "$PWD/target/s8i-b0/nd" || exit 1
out=$(cd D:/repos/redkiln-corpus/runner && nodispatch cargo test --bin redkiln-spike-runner -- sbin::tests:: 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the driver's tests did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
# (dated 2026-10-03, s8i build, R5FIX-8i) 33 tests: §8i.3's 25 and R5FIX-8i's 8 named tests (D21-144). It read 25.
# (dated 2026-10-06, s8i build, D21149-8i-A) 34 tests: D21-151 adds a_volume_under_sbin_need_bytes_refuses_before_the_copy;
# D21-149 item 3's inverted capture test is renamed, not added. It read 33. Run again over the D21149-8i runner commit (§8i.5).
printf '%s\n' "$out" | grep -qE 'test result: ok\. 34 passed; 0 failed' || { echo "FAIL: the sbin selection did not run exactly the 34 tests §8i.3 names"; exit 1; }
nd_check "B0 driver tests" || exit 1
printf '%s\n' "$out" > D:/repos/redkiln-corpus/results/s-bin/driver-tests.txt || { echo "FAIL: could not keep the test output"; exit 1; }
echo "B0 driver tests: 34 passed under the no-dispatch wrapper"
)
```

**The audit.** `results/audit-s.py` gains a `bin` subcommand, invoked as ~~`python results/audit-s.py bin
<s-bin.jsonl> <missing-fields.jsonl>`~~ *(dated 2026-10-03, s8i build, R7FIX-8i: `python results/audit-s.py bin <s-bin.jsonl>
<missing-fields.jsonl> <oot-shadow-rulings.jsonl>`, the dated paragraph after "The audit holds a shadow-flagged
record")*. It reads nothing else and ends its output with two lines,
`rule_outcome_pass1=<outcome>` and `rule_outcome_pass2=<outcome>`, computed by §8i.8's mapping from the records
alone. Its output is kept as `results/s-bin/audit.txt`. Checkpoint 8i re-runs the pinned script over the pinned
records and requires the same bytes. It reports, per pass and per item, detection on each seed (strong,
under strict), approval on each clean item, the fields cited, whether the reviewer ran the automation itself, and
wall clock — **no figure pools the two passes** (the 2026-09-11 entry: *"no figure pools them, and a 2/2 in one pass
beside a 1/2 in the other is reported as exactly that"*). It is checked against a hand-built fixture of three records, and
against one hand-built fixture per rule outcome, before it reads a real one. There are six: `branch1`,
`branch2-detected`, `branch3`, `undecided-fp`, `undecided-silent` and `undecided-partial`, the six §8i.9's guard (4)
computes. *D21-48 (2026-09-24): X3 registered `undecided-partial`, and it replaces the `unregistered` fixture.*

*(dated 2026-10-02, s8i build)* **The audit, built (package S8I-P6).** It is built in a third private copy, a
`git archive` of corpus `main` (`285e011`) taken over `results/` only, at
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P6-corpus`. Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`. The
patch touches only `results/`, so it is independent of P5's `runner/` patch, and the operator applies both:
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P6-audit-bin.patch` (sha256
`0d2964ba50ec32987e80381094d58cb62d7f56e967bc4b7e5518ea33bfd17463`). It applies cleanly to a fresh extract with
`core.autocrlf=false`, and the result is byte-identical to the copy. The patch carries these files:
- `results/audit-s.py`, which gains `bin` (sha256 after: `6cf91638f74432bb540ff4cd4ae7d7f37a054ed121ab564110d46a685360e42c`, CRLF
  kept);
- `results/s-bin/audit-fixtures/`, holding seven fixtures and `check-audit-bin.py`.

What it does and how it was checked:
- **The outcome.** `bin_outcome` is a transcription of Checkpoint 8i guard (4)'s `outc` snippet. A scratch check
  reads that snippet from this file, never from a copy, and runs it over every fixture. On all seven it gives the
  same two values as the audit, and a perturbed control moves both together.
- **The figures.** It reports per pass and per item. The pass figures are detection on each seed (strong, under
  strict), approval on each clean item, the fields cited, whether the reviewer ran the automation, and wall clock.
  A void row is shown but not scored. No figure pools the passes.
- **The fixtures.** Each per-outcome fixture holds two passes, and pass 2 has a different outcome, or reaches
  `branch3` by the other route, so pooling shows. All seven were red first, with "unknown subcommand: audit-s.py
  printed its usage (rc=2)", and are green now ("fixtures: 7 checked, 0 failed").
- **Mutants.** Six were run in the copy only, with the restore verified by sha256, and all six were killed:
  `undecided-partial` collapsed to `branch3`; pooling in `bin_rec`, in detection, in the report's detection count and
  in the fields tally; and the false-positive threshold loosened.
- **`var` and `typed`.** Five invocations over their committed inputs give byte-identical stdout and exit codes
  before and after. The usage text, printed for a bad argv, gains its `bin` line, so that output differs by design.
- **A choice to check.** When a pass cannot be scored, for example when only pass 1 has run (§8i.2 item 8 reads
  pass 1's audit before pass 2) or for the dry run's single record, the audit still exits 0. Its outcome line then
  carries guard (4)'s own `BAD-<reason>` token, for example `rule_outcome_pass2=BAD-record`, and never
  `unregistered`. The three-record fixture yields `BAD-record` for both passes by design. On the six per-outcome
  fixtures both lines match the six-value pattern. Guard (4) already fails a `BAD` outcome over real records.

*(dated 2026-10-03, s8i build, R5FIX-8i)* **The audit holds a shadow-flagged record (D21-144).** `bin` takes an
optional third argument, `results/s-bin/oot-shadow-rulings.jsonl`: one line per ruling, `{"raw_ref": …, "ruling":
"clear" | "void"}`. It reads nothing else. A flagged record with no ruling is shown as `HELD:oot-shadow` and not
scored, and its pass's outcome is `BAD-held`; a cleared one is scored; an operator void is `out-of-tree(operator)`,
never a miss. Three fixtures (`held`, `held-cleared`, `held-voided`) were red first and are green (*"fixtures: 10
checked, 0 failed"*); eight new mutants and P6's six all killed. `var` and `typed` print the same bytes before and
after over their committed inputs. The superseding patch is `S8I-R5FIX-8i-audit-bin.patch` (§8i.3's R5FIX-8i note).

*(dated 2026-10-03, s8i build, R7FIX-8i; s8i refuting round 7's minor)* **The registered invocation, amended.** `results/s-bin/audit.txt` is
written as `python results/audit-s.py bin results/s-bin.jsonl results/s-bin/missing-fields.jsonl
results/s-bin/oot-shadow-rulings.jsonl`, the argv Checkpoint 8i (4) re-runs over the pinned records and compares byte
for byte. Written with two arguments, an `audit.txt` over any flagged and ruled record shows that record held and its
pass `BAD-held`, so it could not reproduce under (4). The rulings file is committed with every batch's results, an
empty file when no record is flagged, and is pinned (§8i.7; Checkpoint 8i (3)). METHOD-S's R7FIX-8i extension, item 4,
registers the same. The dry run's own `audit.txt` (step 4's script) audits the dry run's single record, which no
ruling names, and is unchanged.

**The zero-spend dry run — S-BIN's fixture proof.** The driver runs end to end over a dispatch copy of item 1's review repository with the
runner's global `--claude-bin` pointed at a stub that replays a recorded stream; the record it writes carries
`"dry_run": true`, goes to `results/s-bin/dry-run.jsonl` and **never** to `results/s-bin.jsonl` (every record in the set carries
`"dry_run": false`, and Checkpoint 8i refuses a set in which one does not), and the audit reads
it. Golden constraint 4 is the reason: *"A rig that compiles and passes its unit tests has NOT been run."* The captures
do not exist yet, because they are taken in B1. ~~So the dry run renders item 1 with a synthetic capture reference,
and its record marks that it did.~~ *(dated 2026-10-06, s8i build, D21149-8i-A: B1 took them, and D21-149 item 3 rules that *"the dry run no longer
injects a synthetic capture"*. The re-run dry run renders item 1 with item 1's real B1 capture, `capture-1.png` of
`WBP_StashRow` (asset 1, sha256 `9ba295de…98d0`), committed in the dry run's review repository, and its record
carries that capture with `synthetic_capture` false.)* The test run's full output is kept as `results/s-bin/driver-tests.txt`. Its
sha256 is named on the B1 line (§8i.5).

*(dated 2026-10-03, s8i build)* **The dry run: prepared, NOT run (package S8I-P8).** Two things stopped it. Both are recorded here and
neither is worked around.
- **A permission denial, final under the AFK rules.** The driver refuses unless `runner/` is committed and clean
  (`runner_commit`, §8i.7), so the dry run needs a corpus checkout whose `runner/` holds `sbin.rs` in a commit. The
  package cloned corpus `main` (`285e011`) to a private path, `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P8-corpus`
  (`git clone --no-local`, remote removed). That clone is read-only on `D:/repos/redkiln-corpus`, whose HEAD stayed
  `61f590e`, `main` stayed `285e011`, and status stayed empty. P5's and P6's patches applied there, giving `sbin.rs`
  `89473ef43a26910ed8e9113cdade4aef559d53044e2720c77b8f1e67760d8017` and `audit-s.py`
  `6cf91638f74432bb540ff4cd4ae7d7f37a054ed121ab564110d46a685360e42c`. The session's permission classifier then refused
  the commit in that clone (*"[Modify Shared Resources]"*), and refused a later read in the same clone too. The
  refused command was `git add runner/src/main.rs runner/src/sbin.rs results/audit-s.py results/s-bin/audit-fixtures
  && git … commit -m "s8i B0: …"`, run in the clone. So the dry run is the operator's, from a plain terminal, after
  the B0 corpus commit. The clone is left as it stands, with both patches applied and uncommitted. The packet in §8i.5
  does not use it.
- **A driver defect of the kind the dry run exists to catch (golden constraint 4).** It was found by reading the code
  and measuring the tree, not by a run. `render_summary` reads each asset's pointer with
  `std::fs::read(review_repo.join(p))`, `asset_entries` does the same, and both then call `parse_lfs_pointer`. But the
  review trees hold the **materialised** asset in the work tree, and the pointer only as the committed blob. In
  `D:/sb/base/item-1`, `OurGame/Content/UI/Inventory/WBP_StashRow.uasset` is 110,727 bytes of binary (its first
  bytes are `\301\203*\236`). `git cat-file -p HEAD:<path>` prints `version https://git-lfs.github.com/spec/v1`,
  `oid sha256:33800c41a55e0043b508abc896bd1e9f011f7a945cfedb65aeed6bc7f325030e` and `size 110727`. So rendering
  refuses on every item, in the dry run and in B2 alike. The refusal comes before any spawn and costs nothing, but no
  pass could run. Amendment item 6 says size and `lfs_oid` are read *"**from the LFS pointer in the review tree**"*.
  The committed pointer blob is that pointer, so the fix needs no change to how the amendment reads. **Owed to the
  driver before B0's commit:**
  - read the pointer as `git cat-file blob HEAD:<path>` in the review repository, in both `render_summary` and
    `asset_entries`;
  - strengthen `asset_size_is_read_from_the_lfs_pointer`, red first, so its review tree's work file holds non-pointer
    bytes while its committed blob is the pointer, which keeps the selection at 25;
  - add the mutant "read the work-tree file" to that test's row;
  - re-run the 25 tests and every mutant.

  This package did not make the change, because its private copy was refused.

What is prepared. All of it is zero spend, and none of it is in a repository:
- **The stub**, `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P8-stub/sbin-stub.exe`, compiled from `sbin_stub.rs`
  (sha256 `e21e63b5c2f46a356fbbc9aa3073e1b331ab3066e6933d7f83d8b37fc14ad808`). It is never named `claude`.
  - `--version` prints `0.0.0 (S-BIN dry-run stub, not Claude Code)`.
  - Any other call reads the prompt from stdin, logs the spawn to `sbin-stub.log` beside itself, and replays
    `sbin-stub-stream.jsonl` (sha256 `9ce5dbde9ddbe413de982bac3d7591df15a9d5ede9d05ba1757ed378212adf76`), with
    `@@CWD@@` replaced by its working directory.
  - The stream holds an init line (`apiKeySource` `none`; tools `Read`, `Bash`, `StructuredOutput`; model
    `sbin-stub-replay`; version `0.0.0`), one `Read` inside the copy, one `Bash` `git log`, and a result with
    `total_cost_usd` 0 and a digest with no findings in the typed schema's shape.
  - A control run outside the driver replayed the five lines with the path substituted, and logged one spawn.
- **The dry run's review repository**, `D:/sb/dry/rev/item-1`, about 21.5 GB. It is a byte copy of base item 1 plus
  two files:
  - `_s-bin/report/index.json`, the pin's 2026-09-05 report (`c3dae802…6bc6`), as a stand-in, because no greenness
    run exists yet;
  - `_s-bin/report/headless.log`, a one-line synthetic note.

  It is one parentless commit, ~~`378a183fa265f00b6af2a3be02ba2d8adbf87b36`~~ *(dated 2026-10-03, s8i build, D21147-8i:
  rebuilt by D21146-8i as `48cd028e…` and again by D21147-8i as `a8d9478b54556d6617963cab622b385109928ab3`, §8i.4's dated
  notes)* *(dated 2026-10-06, s8i build, D21149-8i-D: rebuilt again by D21149-8i-A as `cc1b4b4526cc4aece63165c9a13eec893767205b` (tree `44931f6ba29a325db7a69569357dd6a34b6bec83`, dates pinned), with a third file, item 1's real B1 capture `_s-bin/captures/capture-1.png` (`9ba295de…98d0`), under D21-149 item 3; §8i.5's D21149-8i-A packet)*, with the message `review tree` and the
  identity `s-bin <s-bin@invalid>`. Checked: `rev-list --all --count` is 1; `refs/heads/main` is the only ref; there
  is no remote; status with `--ignored` is empty; the base commit is pruned; and the tree is base item 1's plus
  exactly those two files. The review root is `D:/sb/dry`, so the dry run's dispatch copy is `D:/sb/dry/d/p1i1a1`,
  never a set path.
- **The dry run's inputs**, under `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P8-dryrun/`. The fixture is
  `results/s-bin/fixture.json` (sha256 ~~`4b16a580d81fdb468ea78a3a9ed00288c348161f537657ddb70c8cc9e28aee51`~~ *(dated
  2026-10-03, s8i build, D21147-8i: re-derived by D21146-8i as `8f98bf43…ea1` and by D21147-8i as
  `62f6469c11e3bc5e656b565ba28c79709ad30526b954d88017ebd69024b6a8b1`, §8i.4's dated notes)* *(dated 2026-10-06, s8i build, D21149-8i-D: re-derived by D21149-8i-A as `6d93cb9e8897579ff2e87316bd0f80794680382a128083f50695e0c59dd1caf6`, item 1's `review_commit` and `captures`, with `asset_index`, moved; §8i.5's D21149-8i-A packet)*). It is P7's
  staged fixture with these changes:
  - `review_root` is `D:/sb/dry`;
  - item 1's `review_commit` is the commit above;
  - `leak_tokens` is `LoadoutFill` plus D21-142's three R tokens;
  - marked stand-ins fill `build_exit` (0) and the spec and report (placeholders).

  The registered argv names `results/s-bin/fixture.json`, so the script puts this file at that path for the dry run
  only and removes it afterwards. A copy stays at `results/s-bin/dry-run/fixture.json`.
- **The operator's script**, `D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P8_dryrun.sh` (sha256
  ~~`622f93f98df15865eadf8faa7fe2bc3a13b296dece2a12f544da2a3d882ba27f`~~ *(dated 2026-10-03, s8i build, R2FIX-8i: ~~`1599099691282b5334a3b1e9612b38dedcf331205e04e08b3ea1e7ba04a9b7f3`~~, after its runner-clean check excluded `runner/target/`)* *(dated 2026-10-03, s8i build, R6FIX-8i; s8i refuting round 6: `5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c`, after it requires the 33-test reading the B0 block writes, not 25, which refused every dry run, and the record's 34 §8i.7 names)* *(dated 2026-10-06, s8i build, D21149-8i-A: `7c95fb14df3d3ddd14133cd44eaa3bd073da1b61861d58cec0f3f708c5c8bf08`, after the operator's two fixes of 2026-10-05 (`c3353777…351f`, `c25b1387…a007`, §8i.5 step 4): it requires the 34-test reading and item 1's real capture with `synthetic_capture` false; old bytes kept as `S8I-P8-dryrun-before-D21149.sh`)*; `bash -n` clean).
  - Before it runs, it refuses inside Claude Code, and it requires the B0 commit and `driver-tests.txt`.
  - It runs the registered dry-run argv under `nodispatch`.
  - It asserts one record, with `dry_run` true, `synthetic_capture` ~~true~~ false *(dated 2026-10-06, s8i build, D21149-8i-A: and `capture_refs` equal to item 1's real capture with its asset index, D21-149 item 3)*, `void` null, the ~~33~~ 34 §8i.7 fields *(dated 2026-10-03, s8i build, R6FIX-8i: `oot_shadow` added, R5FIX-8i)* and
    `total_cost_usd` 0. It also asserts that `results/s-bin.jsonl` does not exist, that the stub logged one spawn, and
    that `nd_check` is clean.
  - It then runs `audit-s.py bin` over the record and keeps the output as `results/s-bin/dry-run/audit.txt`. Both
    outcome lines read `BAD-record`, by P6's disclosed design.
  - Last, it prints the two sha256 values the B1 line names.

*(dated 2026-10-03, s8i build)* **The driver defect fixed, and `commit_single` keeps the tracked-but-ignored files (package S8I-F1).** The
owed items in S8I-P8's note above are discharged, in a fourth private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-F1-corpus`, made from P5's copy (P5's copy is left at its own bytes,
`89473ef4…760d8017`). Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`: HEAD `61f590e`, `main` `285e011`,
status empty, before and after.
- **The pointer.** A new `review_pointer(review_repo, path)` reads `git cat-file blob HEAD:<path>` in the review
  repository and parses that. `asset_entries` and `render_summary` both call it, and neither reads the work file any
  more. Amendment item 6's text is unchanged; the METHOD-S extension of 2026-10-03 records how it reads.
- **`commit_single(tree, tracked)`.** `tracked` is the base tree's tracked set (the item commit's `git ls-tree -r
  --name-only` paths with the strip applied). After `git add -A`, every ignored file in `tracked` is force-added as a
  `:(literal)` pathspec, any other ignored file refuses (untracked junk is neither committed nor silently dropped),
  and the index must then equal `tracked` exactly. `None` is a throwaway tree, the flag probe's, where any ignored
  file refuses. `-f` is never applied to the whole tree.
- **Red first, in the two existing tests, so the selection stays at 25.** `asset_size_is_read_from_the_lfs_pointer`
  now commits the pointers in a review repository and overwrites the work files with binary bytes, then also renders
  through `render_summary`. Red on the old read: *"the pointer is read from the committed blob, not the materialised
  work file: OurGame/Content/UI/V.uasset in the review tree"*, caused by *"an LFS pointer is UTF-8 text"*.
  `review_repo_holds_one_commit_and_no_fix_object` gains a tree whose `.gitignore` matches two tracked files (one
  under an ignored directory), an ignored junk file outside the set, a stray untracked file, and a throwaway tree with
  an ignored file. Red on plain `add -A`: *"a tracked-but-ignored file must be committed: Config/keep.log"*. Both
  green after the guards.
- **Mutants, from `s8i_S8I-F1_mutants.py`, in the copy only, restore verified by sha256.** Seven new:
  `review_pointer` reads the work file (the S8I-P8 defect); `asset_entries` and `render_summary` each bypass it with
  the work file; `commit_single` as plain `add -A`; as `add -A -f` with no tracked-set check (S8I-P7's build); without
  the index check; and force-adding every ignored file. All seven flipped. The last first SURVIVED: with a tracked set
  given, the index check already refuses junk, so the refusal is observable only on a `None` tree. The throwaway-tree
  case was added, and the mutant then flipped. P4's 13 and P5's 12 were re-run over the final bytes: **32/32
  flipped**.
- **Readings, last in the copy, all under `nd.sh`.** The sbin selection: `test result: ok. 25 passed; 0 failed`
  (the count is unchanged, because both tests were strengthened, not added). The whole runner suite: 159 passed, 1
  failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`. clippy exits 0 with nothing for
  `sbin.rs`, and `sbin.rs` is rustfmt-clean.
- **The patch supersedes P5's.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-F1-sbin-driver.patch` (sha256
  `dd3ee74ec8dd2752cdffb42ffb4fb3491f32de3543bcc97003f029ca23414f3a`) is cumulative against a fresh `main` extract at `285e011`. It adds
  `runner/src/sbin.rs` (sha256 `64207927cf4e6b9da1fbf29b8ee116e918dc43ef3569fc5586a55eecbe504de7`) and P4's same
  6 lines in `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh `git archive` of `runner/`, it gives a
  `runner/src` byte-identical to the copy. Apply only this one, not P4's or P5's. P6's audit patch is unchanged and is
  applied beside it. §8i.5's packet names it as `DRV`.
- **Still the operator's:** the B0 corpus commit, B0's block and the dry run. The permission denial recorded above
  still stands, so the dry run runs from a plain terminal after the B0 commit, with this patch in it.

*(dated 2026-10-03, s8i build, R1FIX-8i)* **Refuting round 1's findings, fixed in the driver (package R1FIX-8i).** A fifth private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R1-corpus`, made from F1's copy (F1's copy keeps its own bytes,
`64207927…4de7`). Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`. What changed:
- **B4's re-run could not dispatch.** Its copy path and raw stem left the arm out, so a re-run claimed the pass's own
  `p<pass>i<n>a1` and halted, and nothing added the named field. Now a re-run (`--rerun`, arm
  `b-typed-sbin-plus-<field>`, `<field>` of `[A-Za-z0-9_]`) dispatches into `<root>/d/p<pass>i<n>a<attempt>-plus-<field>`,
  keeps its raw transcript at `raw/p<pass>i<n>a<attempt>-plus-<field>.jsonl`, and renders
  `rendered/item-<n>-plus-<field>.txt`. That rendering is the pass's summary with one key added, `<field>`, whose value
  is `fixture.json`'s `items.<n>.b4_fields.<field>` (B4's amendment supplies it). A field the summary already renders
  refuses, and the pass's rendering never moves. `dispatch_one` dispatches a re-run only after the pass's valid record
  for that item with `detected` false, and refuses an arm and `--rerun` that disagree. METHOD-S's R1FIX-8i extension of
  2026-10-03 registers where these go; the field and its content stay deferred extension 5's.
- **The second void halts the run that produced it.** `dispatch_one` returns a halt when the record it wrote is a void
  at attempt 2, so no later item in that run is spawned.
- **The ceiling is the fixture's.** `check_ceiling`: a real pass refuses before any spawn when `fixture.json` carries no
  `ceiling_secs`, or when `--timeout-secs` differs from it. The dry run, which comes before extension 2, runs at 3600 s.
  `probe` checks it before rendering, and `dispatch_one` checks it again before the copy.
- **A second observed model** halts the set over every record that carries one, voids included, as METHOD-S item 5
  and Checkpoint 8i (4)'s `model` line read it.
- **The rendering is E3's brief.** `render_prompt` now takes E3's own `build_prompt` text, its 24,000- and
  16,000-character truncation included, and replaces only the file-list slot with the summary. The earlier brief had
  changed two of E3's phrases and dropped the truncation (round 1, refuted from major to minor). No rendering had been
  hashed.
- **The tests drive `dispatch_one`.** Round 1 measured that `dispatch_one`'s own pre-spawn-miss, `runner_commit` and
  fix-unreachable refusals, its set-version check, its init-version Halt 2 and its second-model halt could each be
  deleted with 25/25 green. The four `…refuses_before_spawning` tests exercised `spawn_checked`, which only tests call,
  and the rig always passed. Each now has a rig case through `dispatch_one` with the injected stub, asserting the halt
  and zero spawns, in the existing test the table's row names. `the_dispatch_cap_refuses_before_spawning` now seeds
  voided records for both caps, and `resume_is_keyed_per_arm_item_pass` also resumes through `dispatch_one`: a valid
  record skips, and a void retries as attempt 2. **The selection stays at 25.** Every case was added to an existing
  test, so B0's and Checkpoint 8i (6)'s pinned count, and METHOD-S item 5's *"Its 25 named tests"*, stand.
- **Red first.** The behaviour-neutral scaffolding went in first, with the guards unwritten, and five tests went red
  for their stated reasons:
  - `each_dispatch_runs_in_a_fresh_copy_at_a_never_used_path`: *"a valid original, then a B4 re-run, dispatches once:
    Halted("dispatch copy path … p1i1a1 was used by an earlier record or a capture copy")"*;
  - `every_item_carries_a_capture_reference_or_rendering_refuses`: *"the rendering keeps E3's brief and its
    truncation: no "Read the actual source and tests""*;
  - `resume_is_keyed_per_arm_item_pass`: *"the second void halts the run that produced it: None"*;
  - `the_dispatch_cap_refuses_before_spawning`: *"a real pass needs the fixture's registered ceiling: Recorded {…}"*;
  - `a_version_move_between_dispatches_halts_the_set`: *"ver-model-void: a second observed id halts the set: None"*.

  The rig cases over guards that already existed passed at once. Their red is their mutants, below.
- **Mutants, from `s8i_R1FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** Twenty-four new,
  round 1's M1–M8 among them: the B4 copy path and raw stem without the arm, B4's rendering over the pass's file or without its field or replacing a rendered one, F1's hand-written brief, no in-run twice-void halt, the fixture's ceiling unread, defaulted or accepted when it differs, `dispatch_one` ignoring the pre-spawn miss, recording `HEAD` without `runner/`'s status, checking no fix blob, reading no records for the set version or for resume, without the init-version Halt 2 or the second-model halt, the first model read from valid records only or compared only on a valid dispatch, `cap_state` skipping voids, and a re-run with no missed original, after a detected one, with a disagreeing arm and flag, or under any field name. All 24 flipped. F1's 7, P4's 13 and P5's 12 were re-run over the final bytes:
  **56/56 flipped**.
- **Readings, last in the copy, all under `nd.sh`.** The sbin selection: `test result: ok. 25 passed; 0 failed`. The
  whole runner suite: 159 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`. clippy exits 0 with nothing for `sbin.rs`, and `sbin.rs` is rustfmt-clean.
- **The patch supersedes F1's.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R1FIX-8i-sbin-driver.patch` (sha256
  `1922bb473b14a2bd57914e0ce7092e95accd3766eaca39ae80ab214192c12b08`) is cumulative against a fresh `main` extract at `285e011`. It adds
  `runner/src/sbin.rs` (sha256 `5be7cb7508da0618628a946755b0654719239c1a23795afbcddfef07229352b0`, 5,163 lines) and
  P4's same 6 lines in `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh `git archive` of `runner/`,
  it gives a `runner/src` byte-identical to the copy. Apply only this one, not P4's, P5's or F1's. P6's audit patch is
  applied beside it. §8i.5's packet names it as `DRV`.

*(dated 2026-10-03, s8i build, R2FIX-8i)* **Refuting round 2's findings, fixed in the driver (package R2FIX-8i).** A sixth private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R2-corpus`, made from R1's copy (R1's copy keeps its own bytes,
`5be7cb75…52b0`). Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`. What changed:
- **The version is read before each dispatch.** `probe`'s item loop is now `run_items`. It runs `<claude-bin>
  --version` before every item and passes that reading to the dispatch as its `cli_version`, as the bullet above
  prescribes (*"read by `<claude-bin> --version` before each dispatch"*). Before, one reading at start-up served all
  five items. A CLI that moved mid-pass then passed item k's pre-spawn check, and only the post-record Halt 2 caught
  it, after the spend.
- **Every halt stops the loop, and a test now drives the loop.** `run_items` bails on a pre-spawn halt and on every
  post-record halt: the second void, the second model, Halt 2 from init, and Halt 6. The tests call it with the
  injected stub over a two-item rig, and item 2 is never spawned. Round 2 measured that probe's bail could be dropped
  with 25/25 green, because no test reached `probe`.
- **The launch record (METHOD-S item 11).** The driver reads the launch record that `REDKILN_LAUNCH_RECORD` names. Its
  version key is `CLI_VERSION_BEFORE`. The launcher writes `printf 'CLI_VERSION_BEFORE=%s\n' "$cver"` into the record
  and prints `printf 'CLI_VERSION=%s\n' "$cver"` only to stdout, both from the same reading, so the record has no
  `CLI_VERSION` key. A real dispatch refuses before the spawn in two cases: when no launch record is set (§8i.5: every
  dispatch launches through `wrap`), and when the record's version differs from the reading taken before that
  dispatch (Halt 2). The dry run is not wrapped (§8i.3) and needs none. Records also carry `launch_record` and
  `launch_cli_version`, and so do B1's probe records. Checkpoint 8i (4)'s new `launch` line re-checks the equality on
  every record.
- **Seeded items through `dispatch_one`.** A seeded rig item (`seeded` set, `seed_blobs` read from the review
  repository's HEAD) is dispatched:
  - a blocking finding naming the seed records `detected` true and `seed_blob_verified` true;
  - an advisory naming it records `detected` false, never null;
  - a wrong seed oid halts with zero spawns;
  - a seeded item with no `seed_blobs` halts with zero spawns.
- **The fixture's tokens, an empty token list, and the cost bound.** A copy holding D21-142's `white-on-white` refuses
  when the fixture registers that token, and dispatches when it does not. An empty token list refuses. A dispatch that
  reports no cost records 5.00, with `total_cost_source` `bound`.
- **`ran_automation` counts a run, not a mention.** A `Bash` command counts when one of its segments invokes the
  suite: `pwsh` or `powershell` naming `verify_headless`, the `verify_headless.ps1` script itself, or `UnrealEditor-Cmd`
  with `Automation RunTests`. A `cat`, `grep`, `ls` or `echo` naming them does not count.
- **The account roots.** `default_account_roots` reads the environment and calls `account_roots_for(home,
  CLAUDE_CONFIG_DIR)`. A test now asserts the list: both roots, `~/.claude-personal` first, and a third
  `CLAUDE_CONFIG_DIR` added.
- **`fix_unreachable` stays a constant, by construction** (§8i.7's dated note gives the reason).
- **Red first.** The behaviour-neutral scaffolding went in first, with the guards unwritten. Two tests went red for
  their stated reasons:
  - `a_version_move_between_dispatches_halts_the_set`, twice. First: *"a CLI that moves between two items halts the
    run: ()"* (the version was read once, so item 2 dispatched). Then, after the per-item read: *"a launch record whose
    CLI differs from the pre-read refuses before the spawn: Recorded {…}"*.
  - `an_out_of_tree_read_voids_the_record`: *"`cat _workspace/tooling/verify_headless.ps1` mentions the automation and
    does not run it"*.

  The rig cases over guards that already existed passed at once: the seeded item, the tokens, the cost bound, the
  roots and the loop's halts. Their red is their mutants, below. **The selection stays at 25**: every case was added to
  an existing test.
- **One earlier mutant was masked, and the test was narrowed.** R1FIX-8i's M13, *"dispatch_one's set-version check
  reads no records"*, first SURVIVED over the new bytes. In its `ver-set` case the launch record said 2.1.288 and the
  read said 2.1.290, so the new launch check halted with "Halt 2" first. The case now gives the launch record the
  read's version. That is a move between launches, which only the set check catches, and M13 then flipped.
- **Mutants, from `s8i_R2FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** Seventeen are new:
  - round 2's Q1–Q7, with Q7 run on both the second void and the second model;
  - the loop reading the version once, and the loop ignoring a pre-spawn halt;
  - `dispatch_one` skipping the launch check, admitting no launch record, admitting a different version, and
    recording no launch version;
  - the launch record read from the first of two `CLI_VERSION_BEFORE` lines;
  - `ran_automation` back to substrings;
  - the account roots without `~/.claude-personal`.

  All 17 flipped. R1FIX-8i's 24, F1's 7, P4's 13 and P5's 12 were re-run over the final bytes: **73/73 flipped**.
- **Not built, and why.**
  - The per-copy `SBIN_NEED_BYTES` check (METHOD-S item 4) is still not built. Its figure is extension 3's, which is
    not registered yet, and the driver has no free-space call without a new dependency. It is owed item (f) in §8i.5.
  - `flag_probe`'s cap, Halt 6 and project-dir checks are still reached by no test. Its new launch check is
    `check_launch`, which is tested.
  - No test reaches `probe`'s read of `REDKILN_LAUNCH_RECORD`. If `probe` read no record, every real pass would refuse
    before any spawn.
- **Readings, last in the copy, all under `nd.sh`.** The sbin selection: `test result: ok. 25 passed; 0 failed`. The whole runner suite:
  159 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`. clippy exits 0 with nothing for `sbin.rs`, and `sbin.rs` is rustfmt-clean.
- **The patch supersedes R1FIX-8i's.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R2FIX-8i-sbin-driver.patch` (sha256
  `815111521f351e61b8ea884124eb26ef34321c9767dd48792311f9cf4a8868c1`) is cumulative against a fresh `main` extract at `285e011`. It adds
  `runner/src/sbin.rs` (sha256 `4f64c7d0c50500c9eb28562cd9bcdea05bc5bcba71c59a6788d8baf7053c8636`, 5,648 lines) and
  P4's same 6 lines in `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh `git archive` of `runner/`,
  it gives a `runner/src` byte-identical to the copy. Apply only this one, not P4's, P5's, F1's or R1FIX-8i's. P6's
  audit patch is applied beside it. §8i.5's packet names it as `DRV`.
- **The dry-run script's runner-clean check** now excludes `runner/target/`, as `runner_commit` does. Its sha256 is now
  `1599099691282b5334a3b1e9612b38dedcf331205e04e08b3ea1e7ba04a9b7f3`. The pre-check block in §8i.5 is changed the
  same way. A scratch-repo control, `s8i_R2FIX-8i_cleanctl.sh`, shows the old form refusing a tree whose only change
  is `runner/target/`, and the new form passing it. Both forms refuse a modified or untracked runner file and a
  modified audit.

*(dated 2026-10-03, s8i build, R3FIX-8i)* **Refuting round 3's findings, fixed in the driver (package R3FIX-8i).** A seventh private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R3-corpus`, made from R2's copy (R2's copy keeps its own bytes,
`4f64c7d0…8636`). Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`, to `D:/sb`, or to `D:/repos/BrittonGames`.
What changed:
- **The out-of-tree void implements the registered rule.** The bullet above checks each string *"against any path
  under the review root, in its `D:/sb`, `/d/sb` and `/mnt/d/sb` spellings, other than the dispatch copy's own"*, and
  METHOD-S item 4 names *"every other copy under `D:/sb`"* as answer-bearing. Round 3 measured, from inside the test
  binary, that `<copy>/../../base/item-1/…`, `../../base/item-1/…`, a doubled-backslash `D:\\sb\\…` and `git -C
  "D:\\repos\\BrittonGames"` each gave 0 hits. The fold never collapsed `//`, and the own-copy test accepted any `..`
  after the copy's prefix. Round 3 also measured that bases 1–4 hold a post-fix `WBP_GameVitals.uasset` (`9e7f8829…`),
  so another base is answer-bearing for seed 5. Now `out_of_tree_reads` does three things:
  - It folds each string and collapses each run of `/` (`norm_sep`). It does the same to the review root and the copy
    path, so the answer-bearing names match `D:\\repos\\BrittonGames` and `D://repos//BrittonGames`.
  - It resolves `.` and `..` in every path before the containment test (`resolve_path`). That covers each root
    spelling found anywhere in the string and every absolute token.
  - It resolves every relative token against the child's cwd. The cwd starts at the copy. A `cd` or `pushd` in a
    `Bash` command moves it, and the move persists into later `Bash` calls, because Claude Code's `Bash` tool keeps its
    working directory between calls. ~~A `cd` inside any other string, such as a finding's repro command or a
    description, moves only that string's own cwd.~~ *(dated 2026-10-03, s8i build, R4FIX-8i: no other string is read now)* A relative path that resolves under the review root outside the
    copy, or under an answer-bearing root, voids. One that stays in the copy, or lands elsewhere such as the engine
    install, does not.

  METHOD-S's R3FIX-8i extension, item 1, records the normalisation. ~~It implements item 4's rule and does not widen it.~~ *(dated 2026-10-03, s8i build, R4FIX-8i: withdrawn. It audited prose too, and voided 59 of 754 corpus records; METHOD-S's R4FIX-8i extension, item 1)*
- **`dispatch_one`'s wiring into scoring is tested.** Round 3 measured three mutants surviving 25/25: `asset_paths: &[]`
  (major 2), and `tree_path: ""` and `review_root: "Q:/elsewhere"` (a minor). Two tests now cover them:
  - `cited_fields_are_extracted_mechanically_from_the_digest` dispatches a rig item whose finding names only its asset
    path, and asserts `cited_fields` `["assets"]` on the record.
  - `an_out_of_tree_read_voids_the_record` dispatches two rig items. A `Read` inside the dispatch copy leaves `void`
    null. A `Read` of the review repository beside it, under the rig's root, voids as `out-of-tree`.
- **B4 names its one item.** `probe`'s item list is now `pass_items(--only-item, --rerun)`. A re-run with no
  `--only-item` refuses before anything renders, and so does an item outside 1–5. Before this, a B4 re-run over the
  default list reached item 1 first and refused there. B4's argv is in §8i.5's dated note and in METHOD-S's R3FIX-8i
  extension, item 2.
- **Red first.** The test cases went in first:
  - `an_out_of_tree_read_voids_the_record` went red for its stated reason: *"each of these must void as out-of-tree:
    ["<copy>/../../base", "../../base", "doubled backslashes under the root", "doubled backslashes to BrittonGames",
    "doubled slashes to BrittonGames", "a file URL to BrittonGames", "a file URL under the root", "Read
    <copy>/./../sibling", "cd, then ../../../base", "ls ..", "a relative Glob pattern into a sibling copy"]"*. It went
    green on the normalisation. A new control then gave a second red: *"paths that stay in the copy must not void:
    [("a repro command in the digest that changes directory", …)]"*. That went green once a `cd` outside a `Bash`
    command moved only its own string's cwd. These in-copy controls do not void: `./OurGame/…`;
    `OurGame/../OurGame/x`; `cd OurGame && ls ../_s-bin/report`; an absolute `cd` into the copy; a revision range
    `HEAD~3..HEAD`; `<copy>/OurGame/../CLAUDE.md`; the engine install reached through `..`; and a `cd` that persists
    across two `Bash` calls.
  - `each_dispatch_runs_in_a_fresh_copy_at_a_never_used_path` went red with `pass_items` scaffolded at the old
    behaviour: *"a B4 re-run with no --only-item must refuse: Ok([1, 2, 3, 4, 5])"*. It went green after the guard.
  - The three wiring cases passed at once, because the wiring was right. Their red is their mutants. Over the R2 bytes
    and the old tests, `R3-W1`, `R3-W2` and `R3-W3` each SURVIVED, and each flips now.

  **The selection stays at 25**: every case went into an existing test.
- **Mutants, from `s8i_R3FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** Nine are new and all nine
  flipped:
  - the three wiring mutants;
  - no `..` resolution, no separator collapse, relative paths ignored, and no `cd` tracking;
  - a B4 re-run with no `--only-item` run over the default five, and an `--only-item` outside 1–5 accepted.

  Only the two file-URL cases kill "no separator collapse". For a plain absolute token the resolver also drops empty
  segments, so the collapse shows only where a name sits mid-token, as in `file:///D:\\repos\\…`. R2FIX-8i's 17,
  R1FIX-8i's 24, F1's 7, P4's 13 and P5's 12 were re-run over the final bytes: **82/82 flipped**.
- **Recorded, not changed.**
  - No test reaches `probe`'s call of `pass_items`, just as none reaches `probe`'s launch-record read (R2FIX-8i's
    note).
  - The relative-path rule audits every string, as the bullet above does. So a bare `..` token anywhere in a tool
    input, a finding's prose included, resolves out of the copy and voids. A void is retried once, and a second void
    halts (§8i.6). This residual is disclosed for the operator. It is not narrowed here, because narrowing a
    registered check is the operator's call. *(dated 2026-10-03, s8i build, R4FIX-8i: refuting round 4 measured it wider than a bare `..`, at 59 of 754 corpus records. The audit now reads only what a tool reads (the R4FIX-8i note below), and the operator confirms that narrowing before B1)*
  - Checkpoint 8i (4)'s `haiku` line reads every probe's `observed_model`, so a probe with no init line prints `haiku
    BAD` (round 3, refuted to minor). The line is left as it stands. A probe with no init line is a failed B1, so it is
    Halt 3: §8i.6 says *"No Opus dispatch follows a failed B1"*, and packet step 8 says *"A probe that fails refuses
    non-zero: stop and report."* Any later probe needs a ledger line and a new go-ahead first, and a guard change
    would be settled at that go-ahead. The line fails safe, because it is stricter than the stage. Its one fault in
    that case is the misleading "a cap was exceeded" message.
- **Readings, last in the copy, all under `nd.sh`.** The sbin selection: `test result: ok. 25 passed; 0 failed`. The whole runner suite: 159 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`.
  clippy exits 0 with nothing for `sbin.rs`, and `sbin.rs` is rustfmt-clean.
- **The patch supersedes R2FIX-8i's.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R3FIX-8i-sbin-driver.patch` (sha256
  `0bc288756c57010517c32d4f71169db4c8e601411261be74f6a8e315f58ea7dc`) is cumulative against a fresh `main` extract at `285e011`. It adds
  `runner/src/sbin.rs` (sha256 `8115008816e5efce3e76c85d2a6942363b72fa32249ad310b82e2165ac545fe4`, 6,022 lines) and
  P4's same 6 lines in `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh `git archive` of `runner/`,
  it gives a `runner/src` byte-identical to the copy. Apply only this one, not P4's, P5's, F1's, R1FIX-8i's or
  R2FIX-8i's. P6's audit patch is applied beside it. §8i.5's packet names it as `DRV`.

*(dated 2026-10-03, s8i build, R4FIX-8i)* **Refuting round 4's findings, fixed in the driver (package R4FIX-8i).** An eighth private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R4-corpus`, made from R3's copy (R3's copy keeps its own bytes,
`8115008816e5…545fe4`). Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`, to `D:/sb`, or to `D:/repos/BrittonGames`.
- **What round 4 found.** R3FIX-8i's audit read every string of every tool input. That included the digest, which
  arrives as a `StructuredOutput` tool use, and a `Bash` `description`. So these voided, and none of them is a read:
  - a relative import quoted in a real S-TYPED repro (`s-typed.jsonl`, memoize-repo-root);
  - a quoted `#include "../…"`;
  - a prose `..`;
  - a line of the tree's own `_s-bin/report/headless.log` naming `D:/sb/green/item-<n>`;
  - `CLAUDE.md`'s `rootFolder` line;
  - the copy's own path followed by a full stop.

  Three other faults:
  - A read of the child's own saved output under `<account>/projects/<copy slug>/<session>/tool-results/` voided.
  - `cd -- X` and a `cd` after `then` were missed.
  - A `cd` inside `( … )` persisted, which missed a real read of a sibling.
- **What the audit reads now (METHOD-S's R4FIX-8i extension, item 2).** `read_inputs` names the inputs through which a
  tool reads: a `Bash` `command`; `Read`'s `file_path`; `Glob`'s `pattern` and `path`; `Grep`'s `path` and `glob`; and
  for any other tool its `file_path`, `notebook_path` and `path`. The digest, a `Bash` `description` and a `Task`
  prompt are never read.
  - A `Bash` command is lexed. Its program, its operands, the file after `<` or `>`, and the absolute paths in a
    heredoc's body are audited. The operands of `echo` and `printf` are not, and neither is a grep's pattern.
    *(dated 2026-10-03, s8i build, R6FIX-8i: an `echo` or `printf`'s output that reaches execution is audited as what runs: piped into `xargs`, a shell or
    an interpreter, or written to a file the session later runs; so are the `Write` tool's content when its file is
    run, a shell's `-c` string, `eval`'s operands, and a heredoc or here-string fed to a shell (METHOD-S's R6FIX-8i
    extension, item 3))*
  - `cd` is tracked after `then`, `do`, `{`, `!` and assignments, with options skipped. `popd` and `cd -` restore the
    cwd. A `cd` inside `( … )` or `$( … )` ends at its `)`.
  - Trailing punctuation is trimmed from each word, `..` aside.
  - A path under `<account root>/projects/<copy slug>/<session>/tool-results/` is in-tree once resolved (item 3). The
    account roots are the ones the driver already knows (`default_account_roots`), now handed to scoring as
    `ScoreInput::account_roots` by `dispatch_one` and `probe`.
  - **The rule's words decide the headless.log case (item 4).** A finding, an `echo` or a grep pattern that quotes the
    greenness copy's path is a mention, and it does not void. A `Read`, `cat`, `ls` or `git -C` of that path voids.
- **Corpus replay, measured** (read-only, from inside a test binary in a separate replay copy,
  `S8I-R4FIX-8i-replay`, under `nd.sh`, never shipped). The input was every record in
  `D:/repos/redkiln-corpus/results/*.jsonl` that carries reviewer tool uses, 754 records in 15 files, with the copy
  `D:/sb/d/p1i1a1` and the root `D:/sb`. The records hold tool names only, and their one recorded input is the digest.
  - **R4: 0 of 754 void.** R3's driver: 59 of 754 (7.8%).
  - The same replay over the 838 fixture reviewer transcripts under `~/.claude-personal/projects/D--repos-redkiln-rs-spikes-fixtures-*`
    (13,147 tool uses, each fixture mapped onto the copy and each slug onto the copy's): R3 voided 218, R4 voids 69.
    Each of the 69 is a real out-of-tree read. 68 name a `D:/repos/redkiln-rs` path outside the session's own fixture
    in a read position, and one runs `ls ..` and `cd ../.. && diff` over its siblings. The voids are listed in
    `R4FIX-8i-replay-r4.out`.
- **Red first.** The cases went into `an_out_of_tree_read_voids_the_record` before any code. On R3's code it went red
  for its stated reason, in one assertion: *"each of these must void as out-of-tree: ["a subshell's cd does not
  outlive it, in one command", "a subshell's cd does not persist into the next call", "popd restores the directory
  pushd left"]; paths that stay in the copy must not void: [("a real S-TYPED digest's relative import …"), …]"*. It
  listed 18 false voids, each shown with its evidence string (`R4FIX-8i-redA.out`). It went green on the new audit.
  Then:
  - A `dispatch_one` rig case, a `Read` of the child's own saved output, leaves `void` null. Its red is its mutant,
    `R4-W1`.
  - A later must-void case, a doubled-backslash file URL to BrittonGames inside a heredoc, pins the `Bash` command's
    own separator collapse.

  Every round-3 must-void case still voids: `<copy>/../../base` as a `Read` and in a `cat`, `../../base`, `D:\\sb\\base`,
  `git -C "D:\\repos\\BrittonGames" log`, and now a subshell followed by a relative read of a sibling. **The
  selection stays at 25.**
- **Mutants, from `s8i_R4FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** 21 are new,
  and each flipped. Among them are the four scoping rules the package names:
  - `R4-S1`: audit the digest again;
  - `R4-S2`: audit a `description`;
  - `R4-S3`: persist a subshell's `cd`;
  - `R4-S4`: drop the tool-results exemption.

  The rest cover `cd` options and keywords, the grep pattern, `echo`, punctuation, `popd`, the heredoc body (not read,
  or read as commands), `grep -f`, an input redirect, `Grep`'s path, `git -C`, the whole session directory, the
  exemption tested before resolution, a grep option's value, the command's separator collapse, and the wiring. Three
  earlier rows no longer find their code, because R4 rewrote it. Each is re-run under the same wording, re-anchored:
  `R3-N3@R4`, `R3-N4@R4` and P5's `M10@R4`. The earlier tables were re-run over the final bytes: **102 of 103 flipped** (this package's 21 new rows and 3 re-anchored rows; R3FIX-8i's 7 others, R2FIX-8i's 17, R1FIX-8i's 24, F1's 7, P4's 13 and P5's 11 others).
  One row survives, and it is recorded rather than counted: R3-N2, *"no separator collapse"* in `norm_sep`. It is no
  longer observable, because every audited word is resolved before the containment test and `resolve_path` drops empty
  segments. `R4-S20` carries the `Bash` command's own collapse, and it flips.
- **Readings, last in the copy, all under `nd.sh`.**
  - The sbin selection: `test result: ok. 25 passed; 0 failed`.
  - The whole runner suite: 159 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`.
  - clippy exits 0, with nothing for `sbin.rs`.
  - `sbin.rs` is rustfmt-clean.
- **Not done here.** The cwd model assumes the `Bash` tool keeps a `cd` that leaves the copy. A read-only probe of the
  CLI binary for a cwd reset was refused by the permission classifier, so this is disclosed in METHOD-S's R4FIX-8i
  extension, item 6. That item also discloses the other residuals: relative paths inside inline code, a bare `cd`, and
  `git log --grep=`. The narrowing is recorded by an appended extension. Confirming it before B1 is the operator's
  (the review packet).
- **The patch supersedes R3FIX-8i's.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R4FIX-8i-sbin-driver.patch` (sha256
  `934ad2c15c94f5443c0d45aa880851d73f2d6e7b511d9798a14f2b114ce847b4`) is cumulative against a fresh `main` extract at `285e011`. It adds `runner/src/sbin.rs` (sha256
  `af5e29b4c5aeeea8b62dc442834f358bb180f0521aae9bf068fb2fdd62d082ae`, 6727 lines) and P4's same 6 lines in `runner/src/main.rs`. Applied with
  `core.autocrlf=false` to a fresh `git archive` of `runner/`, it gives a `runner/src` byte-identical to the copy.
  Apply only this one, not P4's, P5's, F1's, R1FIX-8i's, R2FIX-8i's or R3FIX-8i's. P6's audit patch is applied beside
  it. §8i.5's packet names it as `DRV`.

*(dated 2026-10-03, s8i build, R5FIX-8i)* **Refuting round 5's findings, fixed in the driver and the audit (package
R5FIX-8i, under D21-144).** A ninth private copy, `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R5-corpus`, made from
R4's copy (R4's keeps its bytes, `af5e29b4…2d082ae`), and an audit copy, `S8I-R5A-corpus`, made from P6's. Nothing
wrote to `D:/repos/redkiln-corpus` or its `.git`, to `D:/sb`, `D:/repos/BrittonGames`, `D:/r6c` or the sweep's other
roots.
- **The sweep (D21-144 ~~(d)~~ *(dated 2026-10-03, s8i build, R6FIX-8i: Ruled item 1, "named roots taken from a **measured read-only sweep**")*), read-only, over C: and D:.** 5,066 start points, 15 patterns (the registered tokens, the
  seed and fix oids, the item commits, the fixture's seed statement), an explicit skip list (the OS and toolchain
  directories, junction aliases, the roots item 4 registers). 397 files hold a hit, bounded by eleven roots: `D:/r6c`,
  `D:/repos/redkiln-sev`, `~/.claude-personal`, `~/.claude-backup`, `~/AppData/Local/JetBrains`,
  `~/AppData/Local/Packages/Claude_pzs8sxrjxfjjc`, and under the temp directory `claude`, `tmp.uDZZzvzEF4`, `h.md`,
  `l143` and `ms0`. *(dated 2026-10-03, s8i build, R6FIX-8i: corrected, METHOD-S's R6FIX-8i extension, item 5: 397 unique hit paths, 378 under the
  eleven roots and 19 outside them, 17 to mypy's `report.py` and one DriveFS cache copy of it listed twice, once through
  the legacy `Application Data` junction; the client document came from the separate OneDrive pass)* 17 hits state no seed (mypy's `report.py`, *"entirely whitespace"*, and one client document's
  prose) and are not registered, a judgement named in the METHOD-S extension. The OneDrive roots were checked by
  metadata and their locally stored recent files only. The copies are bounded, so no escalation to B follows.
- **What was built (METHOD-S's R5FIX-8i extension).** The runtime void's assignments, ancestor rule, spelling fold and
  named roots; the shadow audit and the record's `oot_shadow`; `audit-s.py bin`'s hold; `runner_commit` as the commit
  that last changed `runner/`. The replay found two more faults, fixed red first: a POSIX path is resolved before the
  fold (`/tmp/../c/…`), and a descriptor before a redirect (`2>`) is not a word.
- **Red first.** Each of the eight new tests went red for its stated reason with only behaviour-neutral scaffolding in
  (`R5FIX-8i-red.out`); `the_dispatch_cap_refuses_before_spawning` went red on *"the record lacks §8i.7's
  `oot_shadow`"*, and `a_dirty_runner_tree_refuses_before_spawning` on *"a results-only commit does not move
  runner_commit"*. The audit's three hold fixtures went red (*"rule_outcome_pass1=BAD-held"* missing, and the usage
  text on a third argument). All went green.
- **Replays, from inside the test binary under `nd.sh`, in a separate replay copy never shipped.** Corpus records: 0
  of 754 records void (15 files). The records carry only the digest, which is never audited, so this replay audits no
  read input (`read_inputs=0`) and is not evidence of correctness (round 5's minor). Transcripts: 838 sessions, 13,147
  tool uses. 71 runtime voids, every one card-shaped: R4FIX-8i's 69, each still voiding, and 2 new. One is genuine
  (`find /d -name "*.js" -path "*/workflows/*"`, a walk of D:). One is a false void: a session read a mistyped
  spelling of its own task-output path under `<temp>/claude`, a directory that does not exist. Exempting it would open
  other slugs, so it is disclosed. Before the task-output exemption and the two replay fixes, 64 more sessions voided
  on their own task output, and one more on `/tmp/../c/...`. The shadow audit flags 312 of the 838 sessions (10,786
  reads), all 71 voids among them, so 241 sessions are flagged and not voided: the allowlist's measured false-flag
  rate. By class of the flagged path: `D:/repos/redkiln-rs` 4,109 reads in 67 sessions; temp not written first 2,420
  in 58; Git Bash words outside the allowlist (most are regex or sed operands read as paths) 1,544 in 263; other D:
  paths 1,013 in 22; other words (escapes read as paths) 584 in 156; unresolved `$VAR` or `$(...)` 364 in 116; the
  session's own task output 305 in 77; other C: paths 257 in 10; the review root outside the copy 185 in 17; the home
  directory 5 in 2. The `/tmp/*.bak` write-then-restore pattern is clear; one `rm` of a temp path never written stays
  flagged.
- **Mutants, from `s8i_R5FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** This package's rows: 78
  rows, 77 flipped, and R5-B15 survives unobservably: the three repositories add no ancestor the sweep root
  `D:/repos/redkiln-sev` does not (recorded, not counted). The earlier tables over the final bytes: 96 rows, 95
  flipped, with R3-N2 unobservable as R4FIX-8i recorded; ten earlier rows whose code was rewritten are carried by
  re-anchored `@R5` rows of the same wording (R4-S3, R4-S14, R4-S15, R4-S16, R4-S18, R3-N4@R4 and P4's M24, plus
  R4FIX-8i's three). The audit's: 14 of 14 killed (8 new, P6's 6). The R4FIX-8i driver's summary line read *"SUMMARY
  103/103 flipped"* with R3-N2 surviving; it counted the unobservable survivor as flipped (round 5's minor). This
  driver prints the two separately.
- **Readings, last in the copy, under `nd.sh`.** The sbin selection: `test result: ok. 33 passed; 0 failed`. The whole
  runner suite: 167 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`.
  clippy exits 0 with nothing for `sbin.rs`; `sbin.rs` is rustfmt-clean.
- **The patches supersede R4FIX-8i's and P6's.**
  `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R5FIX-8i-sbin-driver.patch` (sha256
  `3357c3fb79cd7b0324b8ab40bfaf474be64e4140fd12817754f8633fe0e26bb0`), cumulative against a fresh `main` extract at
  `285e011`: `runner/src/sbin.rs` (sha256 `bcdd084840e5c6bae3bc0eb887cafa7031915f4ed7eb8211f362925d5476e50f`, 8,024
  lines) and P4's 6 lines in `runner/src/main.rs`.
  `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R5FIX-8i-audit-bin.patch` (sha256
  `e0243d5d4edeb9a7b538889e346d488d6e60c61a2f036c31ea0ccf7cb4eb4d32`), cumulative against `285e011`'s
  `results/audit-s.py` (sha256 after: `bd6df55b28690913772c0fedef03c3a71526412447617d0566b1d758b16817dd`, CRLF kept)
  and every file of `results/s-bin/audit-fixtures/` (ten fixtures). Each applied with `core.autocrlf=false` to a fresh
  extract gives byte-identical files. Apply only these two. §8i.5's packet names them as `DRV` and `AUD`.
- **Not done here, and owed.** ~~The panel's ruling on the provisional task-output exemption and on~~ *(dated 2026-10-03, s8i build, R6FIX-8i: the
  exemption is ruled by D21-145; the operator still confirms)* the unregistered
  incidental hits ~~(the blocker returned with this package)~~ *(dated 2026-10-03, s8i build, R7FIX-8i; s8i refuting round 7's minor: that
  parenthesis belonged to the struck clause, the provisional task-output exemption, which was the blocker R5FIX-8i
  returned and D21-145 rules. The unregistered incidental hits were never the blocker; they are owed to the operator at
  item 4's gate, §8i.5 (g))*; ~~refuting round 6~~ *(dated 2026-10-03, s8i build, R6FIX-8i: fixed, the note below)*; the operator's housekeeping and the
  item-4 gate (§8i.5's owed items (g) and (h)); the engine-path allowlist entries from B1's transcripts.

*(dated 2026-10-03, s8i build, R6FIX-8i)* **Refuting round 6's findings, fixed in the driver, the dry-run script and
Checkpoint 8i (package R6FIX-8i, under D21-144 and D21-145).** A tenth private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R6-corpus`, made from R5's copy (R5's keeps its bytes, `bcdd0848…e50f`).
The audit is unchanged, so `S8I-R5FIX-8i-audit-bin.patch` stands and `S8I-R5A-corpus` is untouched. Nothing wrote to
`D:/repos/redkiln-corpus` or its `.git` (HEAD `61f590e`, `main` `285e011`, status empty), to `D:/sb`,
`D:/repos/BrittonGames`, `D:/r6c` or `%TEMP%/claude`.
- **The temp rule (major).** `cp X /tmp/` and `cp X "$TEMP"` recorded the temp directory itself as written, so every
  later temp read was clear. A directory destination (a trailing `/`, a last `.` or `..`, the temp directory or an
  ancestor of it, or several sources) now receives `<dest>/<basename of each source>`, and a written path covers a read
  only below the temp directory (METHOD-S's R6FIX-8i extension, item 2). The allowlist's version is `r6fix-8i-1`.
- **Output that reaches execution (major).** An `echo` or `printf`'s operands stay a mention until the output reaches
  execution. Piped into `xargs`, a shell or an interpreter, or written to a file the session later runs (`bash f`,
  `./f`, `source f`, `xargs < f`, `python f`), the text is audited as what runs. So are the `Write` tool's content when
  its file is run, a shell's `-c` string, `eval`'s operands, and a heredoc or here-string fed to a shell. xargs's input,
  or a shell's or interpreter's stdin, that the audit cannot read is flagged in shadow, never voided (that extension,
  item 3). The residuals are disclosed in its item 6: inline code is not lexed, a profile junction spelling is not
  folded, a heredoc-written script is not staged, a program between an `echo` and its executor makes the text unknown,
  and a `cp` into an existing temp subdirectory named without a trailing `/` covers that subdirectory.
- **The dry-run script (major).** `s8i_S8I-P8_dryrun.sh` required `test result: ok. 25 passed`, but packet step 3's B0
  block writes a 33-test `driver-tests.txt`, so step 4 refused every time. It now requires 33, the count the B0 block
  and Checkpoint 8i (6) assert, and checks the record's 34 §8i.7 names (`oot_shadow` added). Its R2FIX-8i bytes are kept
  as `R6FIX-8i-dryrun-before.sh`. Its sha256 is now `5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c`, and `bash -n` is clean. A scratch
  control, `s8i_R6FIX-8i_dryrunctl.py`, runs the script's two `driver-tests.txt` lines, extracted from the script, over
  the B0 selection's real output: the new script accepts it, and refuses a 25 reading, a reading with one failure and an
  empty file; the old script refuses the real output (`R6FIX-8i-dryrunctl.out`).
- **Minors.** D21-145 is reflected here, in §8i.5 (g), in METHOD-S (the R6FIX-8i extension, item 1), in the ledger and
  in PROGRESS. The `~`, `$HOME`, `$USERPROFILE`, `/c/` and `/mnt/c/` spellings of the own `tasks/` are tested. The
  ledger's title-move dates and the sweep's counts are corrected by dated notes. The lettered `D21-144 (a)` … `(d)` cites
  in the table and in the R5FIX-8i note quote the clause. Checkpoint 8i (4)'s shadow and held lines run before its pairs
  line, so a held record prints the hold message. `s8i_R6FIX-8i_guard4ctl.py` runs the block's own lines over synthetic
  values, on this text and on HEAD's (`R6FIX-8i-guard4ctl.out`).
- **Red first.** The cases went in first, into the existing tests, so **the selection stays at 33**. With only the
  cases in, three tests went red for their stated reasons (`R6FIX-8i-red.out`):
  - `a_temp_read_written_earlier_in_the_session_is_not_flagged`: *"a cp into /tmp/, then another temp file must flag
    (void false, clear)"*, and five more of the same kind;
  - `an_out_of_tree_read_voids_the_record`: *"execution: each of these must void at runtime and flag in shadow: ["echo a
    base path into xargs cat (void false, shadow clear)", …]"*, 17 cases;
  - `an_unresolved_var_in_a_read_position_is_flagged_not_voided`: *"a path grepped from an in-copy log into xargs must
    flag and not void (void false, clear)"*, and four more.

  All three went green on the code (`R6FIX-8i-green.out`). `a_sweep_measured_root_voids`'s home-spelling cases passed
  at once, because the code was right; their red is their mutants. Cases added after the code to give each new branch an
  observable case passed, and their red is their mutants too, except one: an echo piped into `( bash )` went red on the
  new code (*"echo a relative climb into a subshell's bash (void false, shadow flagged)"*, because an empty `(` took the
  pipe's text) and green after the fix (`R6FIX-8i-greenC.out`, `R6FIX-8i-greenD.out`).
- **Replays** (that extension, item 4): the same 71 voids of 838 sessions, with the same 632 evidence lines; 312
  sessions flagged, 10,793 reads (7 new, all in 6 sessions already flagged); 754 records, 0.
- **Mutants, from `s8i_R6FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** This package's rows: 40
  rows, 40 flipped. One of them, R6-E25 (no depth bound on a run inside a run), flips by overflowing the test thread's
  stack, which aborts the binary before it prints FAILED; the driver counts it only on that overflow and reports it as
  FLIPPED-ABORT. Two are R5FIX-8i rows whose code this package rewrote, carried as re-anchored `@R6` rows of the same
  wording (R5-H14, R5-H17). The earlier tables over the final bytes: 172 rows, 170 flipped, with R3-N2 and R5-B15
  unobservable as recorded. One earlier row, R5-A1, first failed to link, because the binary that R6-E25 had just
  aborted was still locked; re-run alone, it flipped (`R6FIX-8i-mutants-rerunA1.out`). Outputs:
  `R6FIX-8i-mutants-all.out`, `R6FIX-8i-mutant-logs/`.
- **Readings, last in the copy, under `nd.sh`.** The sbin selection: `test result: ok. 33 passed; 0 failed`, each of
  the 33 reporting `ok`. The whole runner suite: 167 passed, 1 failed, the same pre-existing
  `p7::tests::the_driver_matches_the_committed_catalog`. clippy exits 0 with nothing for `sbin.rs`; `sbin.rs` is
  rustfmt-clean.
- **The patch supersedes R5FIX-8i's driver patch.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R6FIX-8i-sbin-driver.patch`
  (sha256 `10b0e266fe9015a687a97e55991a30c5d0796300c373cd682c78087da3060fae`), cumulative against a fresh `main` extract at `285e011`:
  `runner/src/sbin.rs` (sha256 `6ebe56ffaadc3612d57eb4d19f25daeb7a0aac41421345f3144b88d927c2e54e`, 8,717 lines) and P4's 6 lines in
  `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh extract, it gives a `runner/src` byte-identical to
  the copy. Apply it with `S8I-R5FIX-8i-audit-bin.patch`, and no other. §8i.5's packet names them as `DRV` and `AUD`.
- **Not done here.** What follows an operator void after the AFK run is not registered (round 6's refuted minor, not in
  this package). Still owed as before: the operator's item-4 gate and housekeeping (§8i.5's owed items (g) and (h)), the
  engine-path allowlist entries from B1's transcripts, ~~and refuting round 7~~ *(dated 2026-10-03, s8i build, R7FIX-8i: fixed, the note
  below)*. The code comments in `sbin.rs` that cite
  `D21-144 (a)` … `(d)` are left; those letters are the ruling record's (the ledger's D21-145 note maps them).

*(dated 2026-10-03, s8i build, R7FIX-8i)* **Refuting round 7's findings, fixed in the driver, Checkpoint 8i and the
records (package R7FIX-8i, under D21-144 and D21-145).** An eleventh private copy,
`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R7-corpus`, made from R6's copy (R6's keeps its bytes, `6ebe56ff…e54e`).
The audit is unchanged, so `S8I-R5FIX-8i-audit-bin.patch` stands and `S8I-R5A-corpus` is untouched. Nothing wrote to
`D:/repos/redkiln-corpus` or its `.git` (HEAD `61f590e`, `main` `285e011`, status empty), to `D:/sb`,
`D:/repos/BrittonGames`, `D:/r6c` or `%TEMP%/claude`.
- **A text the audit runs is audited only as that run (blocker).** A shell's `-c` string and `eval`'s operands were
  followed as a run (R6FIX-8i) and also still audited as one operand word of the shell or of `eval`. That word does not
  resolve to the path inside it, so the own `tool-results` and own task-output exemptions (R4FIX-8i item 3, D21-145)
  were skipped and the word voided by name. The reviewer's real wait on its own background task in the replay,
  `timeout 300 bash -c 'while [ ! -f "<own tasks/…output>" ]; do sleep 2; done; tail -200 "…"'`, voided. Now a
  shell's `-c` string, `eval`'s operands and xargs's command are audited only as the run, and a heredoc or here-string
  fed to a shell that reads stdin only as that run, no longer also as data. A path in any of them voids exactly where
  it would void typed bare (METHOD-S's R7FIX-8i extension, item 1).
- **Precommands are peeled (major).** `timeout` (its `-k`/`-s` values and its duration), `nice` (`-n`), `nohup`,
  `env` (`-i`, `-u`, `-C`, `-S`, `NAME=value`), `stdbuf` (`-i`/`-o`/`-e`), `time` (`-f`/`-o`), `command` and `exec`
  (`-a`) are peeled, repeatedly, so the program after them (a shell with `-c`, a staged script, a heredoc or piped
  stdin, an interpreter, `eval`) is followed as if typed bare; their own words are audited as operands, and `env -C`
  moves the program's directory. `time`, `command`, `exec` and `env` leave `SH_PREFIX`. xargs runs its command once
  even over unknown or empty input, so that command is followed too (that extension, item 2).
- **Minors.** The registered audit invocation passes `results/s-bin/oot-shadow-rulings.jsonl` as its third argument,
  as Checkpoint 8i (4) does (the dated paragraph after "The audit holds a shadow-flagged record", and that extension,
  item 4). §8i.7 and Checkpoint 8i (3) require the rulings file pinned always, an empty file when nothing is flagged;
  `s8i_R7FIX-8i_guard3ctl.py` runs (3)'s required-pin lines, extracted from this file and from HEAD's, over synthetic
  pins blocks (`R7FIX-8i-guard3ctl.out`). The "Not done here, and owed" line of the R5FIX-8i note is re-attached by
  dated note. METHOD-S's R5FIX-8i item 3 quotation is corrected by that extension, item 5. D21-143 gains a dated
  paragraph naming this patch. The ledger's title-move dates were already corrected by R6FIX-8i's dated note, so
  nothing is changed there.
- **Red first.** The cases went into the existing tests, so **the selection stays at 33**. With only the cases in, on
  R6FIX-8i's code, three tests went red for their stated reasons (`R7FIX-8i-red.out`):
  - `a_sweep_measured_root_voids`: *"the own task output read inside bash -c, timeout bash -c and eval must not void"*,
    nine cases (native, `C:/` and `/c/` spellings, each in the three positions);
  - `tool_results_c_and_tilde_spellings_are_exempt`: *"the own tool-results read inside a run must not void"*, eight
    cases (`bash -c`, `sh -c`, the `timeout 300 bash -c 'while …'` wait, `eval`, a heredoc and a here-string fed to
    `bash`, and xargs's `bash -c`);
  - `an_out_of_tree_read_voids_the_record`: *"execution: each of these must void at runtime and flag in shadow"*, 18
    wrapped climbs, and *"these must not void"*, 13 controls (the own `tool-results` under each precommand, and a base
    path echoed in a heredoc fed to `bash`).

  Another slug in each position voided before and after. All three went green on the code (`R7FIX-8i-green.out`).
  The climbs under `command`, `exec` and `env -C` already voided on R6FIX-8i's code (the first two through
  `SH_PREFIX`, the third by accident of the one-word resolution), so their red is their mutants. Three cases added
  after the code give the peel's operand audit and the here-string's deferred read an observable case, and their red
  is their mutants (`R7FIX-8i-greenC.out`, `R7FIX-8i-greenD.out`).
- **Replays** (that extension, item 3), R6FIX-8i's driver and this one, both re-run here: the same 71 voids of 838
  sessions; 632 evidence lines go to 631. The one removed is session `d695a1ce`'s own-task wait. That session still
  voids, on its `find /d /repos/redkiln-rs -maxdepth 2 …` and on its `find "C:\Users\ryanm\AppData\Local\Temp\claude"
  -name …`, a walk of the registered temp root that holds every slug. 312 sessions flagged under both; flagged reads go
  from 10,793 to 10,794. 754 records, 0.
- **Mutants, from `s8i_R7FIX-8i_mutants.py`, in the copy only, restore verified by sha256.** This package's rows:
  38 rows, 38 flipped, seven of them the re-anchored rows named next. Seven earlier rows whose code this package rewrote are carried as re-anchored `@R7` rows of the same
  wording (R6-E5, R6-E6, R6-E7, R6-E9, R6-E11, R4-S11, R4-S12). The earlier tables over the final bytes: 205 rows, 203 flipped (R6-E25 by overflowing the stack, FLIPPED-ABORT, as R6FIX-8i records), with R3-N2 and R5-B15 unobservable as recorded.
  Outputs: `R7FIX-8i-mutants-mine.out`, `R7FIX-8i-mutants-prior.out`, `R7FIX-8i-mutant-logs/`.
- **Readings, last in the copy, under `nd.sh`.** The sbin selection: `test result: ok. 33 passed; 0 failed`, each of
  the 33 reporting `ok`. The whole runner suite: 167 passed, 1 failed, the same pre-existing
  `p7::tests::the_driver_matches_the_committed_catalog`. clippy exits 0 with nothing for `sbin.rs`; `sbin.rs` is
  rustfmt-clean.
- **The patch supersedes R6FIX-8i's driver patch.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-R7FIX-8i-sbin-driver.patch`
  (sha256 `5b84a5392ac5127427253057b634ed70f620d03e52f0fca00ee116d883d700ed`), cumulative against a fresh `main` extract at `285e011`:
  `runner/src/sbin.rs` (sha256 `0efe267f6872e2c91b85b9792f661c84308cc59b19d74ce6e58257b140938a93`, 9,173 lines) and P4's 6 lines in
  `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh extract, it gives a `runner/src` byte-identical to
  the copy. Apply it with `S8I-R5FIX-8i-audit-bin.patch`, and no other. §8i.5's packet names them as `DRV` and `AUD`.
- **Not done here.** Still owed as before: the operator's item-4 gate and housekeeping (§8i.5's owed items (g) and
  (h)), the engine-path allowlist entries from B1's transcripts, what follows an operator void after the AFK run, and
  refuting round 8. A program-and-args wrapper outside the peeled list (`winpty`, `watch`, `xvfb-run`) is not peeled
  and is disclosed in that extension, item 6.

*(dated 2026-10-03, s8i build, D21146-8i)* **D21-146 built in the driver: `precheck_exec_paths`, and the launcher in the registered strip set (package D21146-8i).** The operator ruled on 2026-10-03 (D21-146, Weigh-In wi-938b68): *"The driver gains a precheck that refuses a copy holding an absolute answer-bearing path in an executable file (.bat, .cmd, .ps1, .sh, or Python outside docstrings), with a named test."* The work was done in a new private copy, `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21146-corpus`, made from `S8I-R7-corpus` (whose `sbin.rs` is R7FIX-8i's final bytes, `0efe267f…8a93`). Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`.
- **The check.** `exec_path_hits(copy)` reads every file whose extension, without regard to case, is `.bat`, `.cmd`, `.ps1`, `.psm1`, `.sh`, `.bash` or `.py`, with `.git` skipped. A batch file, a PowerShell script and a shell script are read whole. Python is read without its module docstring and its `#` comments, so any other docstring is read as code. A file with a UTF-16 byte-order mark is decoded as UTF-16. Each text is folded (`norm_sep`) and checked for `ANSWER_BEARING` by name and for `SWEEP_ROOTS`. `precheck` records the files in `precheck_exec_paths`, and `miss()` refuses before the spawn, naming them. A refusal writes no record, so §8i.7's 34 names are unchanged. `registered_strip_set()` gains `STRIP_LAUNCHER` (`_workspace/tooling/RunEditor_OurGame.bat`), METHOD-S's D21146-8i extension, item 1.
- **Red first, as named cases in existing tests, so the selection stays at 33** (`D21146-8i-red.out`, `D21146-8i-redA2.out`). The first test is `a_dirty_or_mismatched_copy_refuses_before_spawning`. It holds 14 must-refuse cases: the launcher as shipped, `.cmd`, `.ps1`, `.psm1`, `.sh` by its mount spelling, `.bash`, a `~/.claude-backup` path, an upper-case `.PS1` naming `D:/r6c`, doubled backslashes, UTF-16, Python code after its docstring, a function's docstring, a `#` inside a string, and a `'''` docstring followed by code. On R7FIX-8i's code, before the code below existed, all 14 spawned, and the test failed with *"an absolute answer-bearing path in an executable file must refuse, naming precheck_exec_paths and the file"*. The second test is `stripped_path_set_is_identical_across_all_five_trees`, where each of the five trees now ships the launcher. It failed with *"item 1: the launcher is stripped (D21-146 item 1)"*. Both went green on the code (`D21146-8i-green.out`). The first test also holds five must-spawn controls, which spawn:the measured `author_*.py` module docstring, a shebang and a coding line before a *USAGE* docstring, a Python comment, a Markdown mention, and a tree-relative launcher. It also checks `dispatch_one`'s own refusal: a rig whose review repository holds the launcher halts with `precheck_exec_paths`, spawning nothing and writing no record.
- **Mutants, from `s8i_D21146-8i_mutants.py`, in the copy only, restore verified by sha256.** This package's 22 rows flipped, 22 of 22. Together with every earlier table over the final bytes: 265 rows, 263 flipped. R6-E25 flipped by overflowing the stack (FLIPPED-ABORT), and R3-N2 and R5-B15 are unobservable, as recorded (`D21146-8i-mutants-all.out`, `D21146-8i-mutant-logs/`).
- **The control over the real trees**, from inside the test binary over the shipped code, in a control copy never shipped (`D21146-8i-ctl`). Each old base yields `["_workspace/tooling/RunEditor_OurGame.bat"]`; each rebuilt base yields `[]` (`D21146-8i-ctl-old.out`, `D21146-8i-ctl-new.out`).
- **Readings, last in the copy, under `nd.sh`.** The sbin selection: `test result: ok. 33 passed; 0 failed`. The whole runner suite: 167 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog`. clippy exits 0 with nothing for `sbin.rs`; `sbin.rs` is rustfmt-clean.
- **The patch supersedes R7FIX-8i's driver patch.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21146-8i-sbin-driver.patch` (sha256 `a24c896f7cdd49cdef72a10053a2827b461d232bbfbe366bbaf4cf6cf13afc0d`), cumulative against a fresh `main` extract at `285e011`: `runner/src/sbin.rs` (sha256 `86b662a2983ce1a80d55afcf25de0f065ca273f99fac44efd3ce990fcf86e34d`, 9,562 lines) and P4's 6 lines in `runner/src/main.rs`. Applied with `core.autocrlf=false` to a fresh extract, it gives a `runner/src` byte-identical to the final bytes. Apply it with `S8I-R5FIX-8i-audit-bin.patch`, and no other. §8i.5's packet names them as `DRV` and `AUD`.

*(dated 2026-10-03, s8i build, D21147-8i)* **D21-147 item 2 built in the driver: the strip writes the backlog stub, and `precheck_backlog_stub` refuses a copy whose stub differs (package D21147-8i-A).** The operator ruled on 2026-10-03 (D21-147): *"The driver writes and checks it: the strip writes the stub, the identical-strip-set test admits exactly that one file, and a named check refuses a copy whose stub differs from the registered bytes."* Per D21-140 item 2, the work was done in a fresh private copy, `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21147-corpus`. It is a `git archive` of the corpus's `main` at `285e011` (`D21147-8i-A-main.tar`) with `DRV` (D21146-8i's patch) and `AUD` applied, and before any change its sbin selection read 33 passed (`D21147-8i-A-base-sel.out`). The copy sits under the scratch directory and not under `target/`, because `sbin.rs` holds the registered leak tokens. Nothing wrote to `D:/repos/redkiln-corpus` or its `.git`; the corpus was only read, by `git archive`.
- **The code.** Two new constants: `BACKLOG_STUB_PATH` (`_workspace/backlog/game_foundation_backlog.json`) and `BACKLOG_STUB`, the registered bytes. `strip_tree` writes the stub right after it removes `STRIP_DIR`. `backlog_stub_ok(copy)` holds only when `_workspace/backlog/` holds exactly one entry, the stub file, at the registered bytes. `precheck` records the result as `precheck_backlog_stub`, and `miss()` refuses before the spawn, naming it. A refusal writes no record, so §8i.7's 34 names are unchanged. `registered_strip_set()` is unchanged: the stub is written content, not a fifth entry.
- **Red first, as named cases in existing tests, so the selection stays at 33** (`D21147-8i-A-red.out`). In `stripped_path_set_is_identical_across_all_five_trees`, each of the five trees now ships a real-shaped backlog JSON. The test asserts that after the strip `_workspace/backlog/` holds exactly the stub, at the registered bytes, and that the real file is among those removed. It also pins the stub's sha256 and its shape. On D21146-8i's code it failed with *"item 1: the backlog is stripped whole, and only the registered stub is written back (D21-147 item 1)"*. `a_dirty_or_mismatched_copy_refuses_before_spawning` gains nine must-refuse cases: the stub missing, a status other than `todo`, CRLF, no trailing LF, a byte-order mark, a further story, a further field, the same ids pretty-printed, and a second file beside the stub. It also checks `dispatch_one`'s own refusal, on a rig whose review repository's stub differs. On D21146-8i's code all nine spawned, and the test failed with *"a copy whose backlog is not exactly the registered stub must refuse, naming precheck_backlog_stub (D21-147 item 2)"*. `review_copy` now writes the registered stub, so every spawn control in the suite holds it and still spawns. Both tests went green on the code (`D21147-8i-A-green2.out`).
- **Mutants, from `s8i_D21147-8i-A_mutants.py`, in the copy only, restore verified by sha256.** This package's 14 rows flipped, 14 of 14 (`D21147-8i-A-mutants.out`). Every earlier table, D21146-8i's included, was re-run over the final bytes: 265 rows, 263 flipped. R6-E25 flipped by overflowing the stack (FLIPPED-ABORT), and R3-N2 and R5-B15 survived as recorded unobservable (`D21147-8i-A-mutants-prior.out`, `D21147-8i-A-mutants-all.out`, `D21147-8i-A-mutant-logs/`). With this package's rows: 279 rows, 277 flipped.
- **The control over the real trees**, from inside the test binary over the shipped code, in a control copy that is never shipped (`D21147-8i-A-ctl`, built by `s8i_D21147-8i-A_mkctl.py`). Each base at `D:/sb/base/item-1..5` built before D21-147, and the dry run's review repository `D:/sb/dry/rev/item-1`, yields `backlog_stub_ok=false`, so the new driver refuses all six until they are rebuilt under D21-147 item 3. `exec_path_hits` yields `[]` for each (`D21147-8i-A-ctl-old.out`).
- **Readings, under `nd.sh`.** The sbin selection in the copy: `test result: ok. 33 passed; 0 failed` (`D21147-8i-A-sel.out`). The whole runner suite, in the pristine extract with this patch and `AUD` applied: 167 passed, 1 failed, the same pre-existing `p7::tests::the_driver_matches_the_committed_catalog` (`D21147-8i-A-verify-whole.out`). clippy exits 0, with nothing for `sbin.rs` (a `type_complexity` finding on the new case table was answered with a local type alias, not a suppression). `sbin.rs` is rustfmt-clean.
- **The patch supersedes D21146-8i's driver patch.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21147-8i-sbin-driver.patch` (sha256 `0bf69ab4a1a1e8a396f278c63d1cd4fb943ac64ffc1898392a3c6e8cce54da3c`) is cumulative against a fresh `main` extract at `285e011`. It holds `runner/src/sbin.rs` (sha256 `1cba3304c92d37d09386b65d3c2efb9ab6852765e584062015e1f62eadd5aa29`, 9,773 lines) and P4's 6 lines in `runner/src/main.rs` (unchanged). `s8i_D21147-8i-A_patch.sh` applied it with `core.autocrlf=false`, together with `S8I-R5FIX-8i-audit-bin.patch`, to a second pristine extract. It applies cleanly and gives a `runner/src` and `results/audit-s.py` byte-identical to the copy's (0 differences). The sbin selection there reads `test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 135 filtered out` (`D21147-8i-A-verify-sel.out`). Apply it with `AUD`, and no other. §8i.5's packet names it as `DRV` once its step 2 is re-pinned. That is outside this package, and the dry-run script is unchanged (sha256 `5d112e09…2c2c`) because the selection stays at 33 tests and the record at 34 names.

*(dated 2026-10-06, s8i build, D21149-8i-A)* **D21-149 item 3 and D21-151 built in the driver: captures by explicit asset index, an empty capture list rendered neutrally, no synthetic capture, and the free-space check before every dispatch copy (package D21149-8i-A).** D21-149 item 3 reads *"The driver accepts an empty capture list and renders it neutrally; captures carry an explicit asset index; the dry run no longer injects a synthetic capture; `every_item_carries_a_capture_reference_or_rendering_refuses` is inverted, red first."* D21-151 reads *"The driver checks it before every dispatch copy, retries and B4 re-runs included, against the registered `SBIN_NEED_BYTES` = 34,221,223,118 … and refuses before the copy with a named test, red first."* Built in a fresh private `git archive` copy of corpus `main` at `cc7f222` (`D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21149-corpus`, archive sha256 `bc60a606…3673`). Disclosed: the first `git archive -o` was given a relative path while `-C` named the corpus, so the archive landed in the corpus working tree as one untracked file (`D21149-8i-A-main.tar`); it was moved out at once, and the corpus's status, `--untracked-files=all` without `runner/target/`, read empty after. Nothing else wrote to `D:/repos/redkiln-corpus` or its `.git`.
- **The code.** `CaptureRef` gains `asset_index`, the 1-based position of the captured asset in the item's `asset_paths` (git's order). `capture_entries` now calls `item_capture_entries` per item. An empty list renders `captures: []`. Each capture must name its asset by an index that starts at 1, is unique and rises in git's order, at the registered `capture-<index>.png`, and its note is that asset's object path, never the asset at the capture's position. The render switch `synthetic_capture` and `DispatchPlan.synthetic_capture` are removed. `render_summary` and the record read the item's own captures in every run, the dry run included. The record keeps its `synthetic_capture` key, now always false, and its `capture_refs` carry each capture's `asset_index`. `SBIN_NEED_BYTES` = `34_221_223_118` is a constant. `DispatchPlan` gains an injected `free_bytes` reader. `dispatch_one` reads it on the review root's volume after the copy path is claimed and before `copy_dir`, and it halts on a reading under the figure or an unreadable reading. `probe` wires `free_bytes_of`, which is `df -B1 --output=avail <root>` as the batch pre-check reads it (Git Bash's coreutils, no new dependency), parsed by `parse_df_avail` (one `Avail` header and one byte count, or it refuses).
- **Red first** (`D21149-8i-A-red.out`), over behaviour-neutral scaffolding (`s8i_D21149-8i-A_phaseA.py`: the field and the reader, read by nothing):
  - `captures_name_their_asset_index_and_a_rejected_asset_has_none`: *"an all-rejected item and a partly previewable one both render: item 2 carries no capture reference; rendering refuses (one procedure on all five trees)"*;
  - `a_volume_under_sbin_need_bytes_refuses_before_the_copy`: *"one byte under the registered figure refuses: Recorded { … }"*.

  Both are green after the guards (`D21149-8i-A-green.out`). The capture test replaces `every_item_carries_a_capture_reference_or_rendering_refuses`. Its new head asserts an all-rejected item rendering `[]`, a partly previewable item (asset 1 rejected, asset 2 captured at `capture-2.png`) naming `/Game/UI/W3`, five refusals (index 0, an index past the assets, a path that is not its index's, a repeated index, an out-of-order index) and their control. Its kept R1FIX-8i half (E3's brief, B4's rendering) is unchanged. It now ends on the neutral rendering: the summary with `captures: []` equals the captured one in every other key, no word in `reject`, `unsupported`, `n/a`, `no capture`, `synthetic` or `stalled` appears in either prompt, a dry run over an item with no capture records `capture_refs` `[]` and `synthetic_capture` false, and a real record's capture carries `asset_index` 1. The free-space test covers one byte under the figure (halt, 0 spawns, no copy, the review root read once), an unreadable reading, a retry, a B4 re-run, the exact figure as the control, and `df`'s measured output and five malformed ones.
- **Mutants, from `s8i_D21149-8i-A_mutants.py`, in the copy only, restore verified by sha256, with TMP/TEMP under `sp/s8i`.** This package's rows: D149-1 to D149-11 flipped, 11 of 11. D149-11 is M18 (*"render captures for the seeded items only"*) re-anchored on the new code, and M18 is superseded. D151-1 to D151-10 flipped, 10 of 10. D151-11 (`probe` wiring an always-ample reader) SURVIVED and is recorded as unobservable: `probe` reads the cwd's corpus and `current_exe`, and no test reaches it. The dry run is the first run of `free_bytes_of`. Every earlier table was re-run with the renamed test mapped (`D21149-8i-A-mutants-all.out`): **174 of 176 run flipped**, and the 2 survivors are the recorded unobservables D151-11 and R5-B15. 124 rows were **not run (ENV-UNREAD)**, for the reason below. All 124 anchor exactly once in both the pristine and the final `sbin.rs`, and none overlaps a changed line (`D21149-8i-A-envrows.out`).
- **The four audit tests red under this package's TMP, a blocker returned.** The AFK rules put the tests' scratch (`std::env::temp_dir`) under `sp/s8i`, which lies under the sweep root `d:/repos/redkiln-sev` (`SWEEP_ROOTS`). So `a_sweep_measured_root_voids`, `a_temp_read_written_earlier_in_the_session_is_not_flagged`, `an_out_of_tree_read_voids_the_record` and `the_shadow_audit_flags_reads_outside_the_allowlist` void every in-copy read and fail. They fail the same way on the pristine copy at `cc7f222` (29 passed, 4 failed; `D21149-8i-A-base-sel-pristine.out`) and after this change. The four failure messages are the same, and no line of the audit moved. The 34-of-34 reading is owed to the operator's B0 block (§8i.5's D21149-8i-A packet, step 3a), which runs with the default TEMP.
- **Readings, under `nd.sh`.**
  - The sbin selection, in the copy and in a pristine `cc7f222` extract with the patch applied (`D21149-8i-A-sel.out`, `D21149-8i-A-verify-sel.out`): `running 34 tests` and `test result: FAILED. 30 passed; 4 failed`, the four above.
  - The whole runner suite: 164 passed and 5 failed, the four above and the pre-existing `p7::tests::the_driver_matches_the_committed_catalog`.
  - clippy reports nothing for `sbin.rs` (only `main.rs`'s two pre-existing warnings), and `sbin.rs` is rustfmt-clean.
- **The patch.** `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21149-8i-sbin-driver.patch` (sha256 `a904b4c02fa5d0d0b89fe962e7c240d3c9d1ff0b03a272d6f824792a15bae637`) is against corpus `main` at `cc7f222`, where `DRV` (`0bf69ab4…`) and `AUD` (`e0243d5d…`) are already committed. It changes `runner/src/sbin.rs` only (+382/−104), giving sha256 `bcdf0364f5d2927ae48a940a0236abe1ee0ee022cceb1331d74415d448436bc1` (10,051 lines). Applied with `core.autocrlf=false` to a second fresh extract of `cc7f222`, it gives a `runner/src` byte-identical to the copy, with nothing else moved (`D21149-8i-A-patch.out`). It names no `target/` path. It is a new runner commit (§8i.5's D21149-8i-A packet), not a superseding `DRV`: B0's commit stands.
- **The selection is 34 tests, and the record keeps its 34 names.** The B0 block, the dry-run script and Checkpoint 8i (6) are re-pinned by dated edit (above, §8i.5, §8i.9).

### 8i.4 The fixture — five review trees, two seeds, and the operator's UE runs

Everything here costs no model spend, and all of it happens in B0, before B1.

**Materialise each item's base tree outside both repositories.** They sit under one short root, named in the
amendment and in the finding's pins as `review_root`. The root is short, `D:/sb` or similar, because paths get long.
The longest path BrittonGames tracks at the pin is 164 characters (`OurGame/Plugins/SyntyUIFantasyWarriorHUD/…`
[measured, `git ls-tree -r --name-only HEAD`]), and a UE build adds deeper `Intermediate/` paths under Windows'
260-character limit. The source is `D:/repos/BrittonGames`, read-only: a disposable clone at the item's commit whose
LFS objects are read from BrittonGames' own store. BrittonGames' `.git` gains no worktree registration, and its
working tree, with its two modified paths, is never touched. Prove the LFS read rather than assume it: for every
asset in the change, the materialised file's SHA-256 equals the `oid` in its pointer. Then apply the seed as
registered —

```bash
# item 4, in a tree at a71265925 (registered 2026-09-09; METHOD-S §7)
#   git checkout a71265925^ -- OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset
# item 5: REPLACED before dispatch (D21-43 N22; §8i.2 item 12). In a tree at the replacement commit R:
#   git checkout R^ -- <each asset R changed>
```

— and strip §8i.2 item 3's registered set identically from all five. `CLAUDE.md`'s four status rows are removed
by a **terminator-agnostic** byte rule: a line matches with or without a trailing CR, and the strip asserts exactly
four rows removed per tree. `CLAUDE.md` is LF at every item commit today [measured, byte count], and the rule must
not depend on that. Scan each tree for the registered leak tokens. Then commit each tree as the **only** commit of a
fresh repository, the item's **base repository** (§8i.2 item 4). Confirm each seed by blob oid (`git rev-parse
<seed>^:<path>` against the fresh repository's `git rev-parse HEAD:<path>`). Confirm the fix unreachable: the fixed
pointer blob is absent from the object store, its LFS object is absent from `.git/lfs/objects`, and `git rev-list
--all --count` is 1.

**Five kinds of copy, and no tree reused** (§8i.2 item 4). A Claude Code session keys its transcripts and
auto-memory by its working directory's path. For example, `D:\repos\BrittonGames` becomes
`~/.claude-personal/projects/D--repos-BrittonGames`, with every character outside `[A-Za-z0-9]` turned into `-`
[measured, the account's `projects/` listing]. And a reviewer with `Bash` can change the tree it is given. So:

| copy | path | made | holds | afterwards |
| --- | --- | --- | --- | --- |
| base repository | `<root>/base/item-<n>` | B0 | the stripped tree, with its seed on items 4 and 5, as one commit | frozen; nothing runs in it |
| greenness copy | `<root>/green/item-<n>` | B0 | a byte copy of the base | the UE run; its report is copied out; deleted |
| capture copy | `<root>/cap/item-<n>` | B1 | a byte copy of the base | the capture session; only the capture file leaves; deleted |
| review repository | `<root>/rev/item-<n>` | after B1, before any rendering is hashed | a fresh single-commit repository of the base tree plus `_s-bin/captures/<file>` | frozen; its commit is `items.<n>.review_commit` |
| dispatch copy | `<root>/d/p<pass>i<n>a<attempt>` | by the driver, before each Opus dispatch | a byte copy of the review repository | the dispatch; deleted once its record is written and no UE process survives |

*(dated 2026-10-02, s8i build)* The amendment (§8i.2 item 4) registers the root as `D:/sb`. The review repository also holds the
greenness report at `_s-bin/report/index.json` and `_s-bin/report/headless.log`, so that `tests.report_ref` and
`build.log_ref` are readable inside the tree. Read from the corpus, they would be an out-of-tree read and would void
the record. Captures sit at `_s-bin/captures/capture-<k>.png`, k in `$ARGUMENTS` order.

A copy is a byte copy of the directory, `.git` included. It is never a clone, so it gains no remote. Every item's
capture file sits at the same relative path in its review repository. **Before each spawn, the driver checks the
dispatch copy and refuses on any miss.** It writes each result into the record:
- `precheck_clean`: `git status --porcelain --untracked-files=all --ignored` prints nothing.
- `precheck_blob_oids`: `git rev-list --all --count` is 1, `HEAD` is `items.<n>.review_commit`, and every asset path's
  blob oid is the fixture's.
- `precheck_leak_hits`: the registered leak tokens over every non-asset file, tracked or not, count 0.
- `precheck_no_project_dir`: neither account root holds `projects/<slug>` for the copy's path.
- `tree_path`: the copy's path, which no earlier record and no capture copy used.

*(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* Two more pre-spawn refusals are built and are **not** record fields, because a refusal writes no record and §8i.7's 34 names are unchanged: `precheck_exec_paths` (D21-146: no executable file names an absolute answer-bearing path) and `precheck_backlog_stub` (D21-147: `_workspace/backlog/` holds exactly the registered 109-byte stub, sha256 `211a4dcf7be9eb68ef73dfcb6e67a832b1713f1e3c9aa8f148fb52df189f8787`). The sentence *"It writes each result into the record"* covers the five above only.

Before each copy, the free space on the root's volume must be at least `SBIN_NEED_BYTES`. The amendment registers
that figure from B0's measurements: the base tree's size, plus the growth one greenness run left behind, plus a tenth.

Write all of it to `results/s-bin/fixture.json`, keyed `items.<n>`. Each item carries at least these keys:
- `stripped`, the path set removed;
- `leak_hits`, the registered-token scan's count;
- `asset_oids`;
- `review_commit`;
- `capture_path`, the capture copy's path;
- `captures`, tree-relative paths and SHA-256.

`captures`, `capture_path` and `review_commit` are filled after B1 and before any rendering is hashed. Checkpoint 8i
reads them.

**The operator's hand, with the editor closed** (`verify_headless.ps1`: *"When the editor is RUNNING, do not use this
script"*):

1. In each item's greenness copy: `pwsh -File _workspace/tooling/verify_headless.ps1` — the default filter, every
   `OurGame.*` test, build then run. The script writes `_workspace/tooling/TestResults/` **inside that copy**, never
   in BrittonGames or in the base.
2. The **greenness** the registration requires: `failed` = 0 and exit 0 on both seeded trees. *"a seed the suite
   catches measures nothing about review-without-a-diff."* A red seed is replaced as §8i.2 registers, before B2.
3. Each tree's `index.json` and `headless.log` are copied to `results/s-bin/greenness/item-<n>/` and hashed; that
   report is the item's `tests.report_ref`.
4. The build and suite wall clock of each run is recorded, and the slowest sets §8i.2 item 7's ceiling. So is the
   copy's size before and after the run, which sets `SBIN_NEED_BYTES`. The greenness copy is then deleted.
5. The captures are **not** taken here. Each is an interactive session on the account (§8i.0 item 11), so they run in
   B1, under its go-ahead (below).
6. The pin's own 2026-09-05 report is copied as registered (*"Before dispatch it is copied into the spike's own
   results directory with its SHA-256 recorded"*) to `results/s-bin/pin-report-2026-09-05/index.json`; its SHA-256
   must be the `c3dae802…6bc6` measured at authoring (§8i.0), or it was overwritten after 2026-09-05 and the copy is
   not the registered report.

**The captures, in B1: the operator's hand, the editor open, one tree at a time** (D21-43 N19; §8i.2 item 2's
registered procedure). For items 1 to 5 in order:

1. Make the item's capture copy from its base repository, then open the editor on the copy's `OurGame/OurGame.uproject`.
2. From the §8i.5 plain terminal, after its second-layer pre-check, start the Claude Code session in the capture copy
   as a `wrap` child (§8i.5). The launcher sets the account's `CLAUDE_CONFIG_DIR` and the freeze, and records the version
   before and after the session.
3. Run `/ue-verify` with the registered `$ARGUMENTS`. Confirm that `monolith_status()` reports this copy's project.
4. Confirm that the capture landed at the registered path. Copy it to `results/s-bin/captures/item-<n>/` and hash it.
5. Write the procedure-log line. It is one line per session, with space-separated fields and no spaces inside a value:
   `item=<n> tree=<capture copy path> model=<id> cost_usd=<x.xx> sha256=<64 hex> shows_subject=<yes|no>
   utc=<YYYY-MM-DDTHH:MM:SSZ>`. `shows_subject` is the operator's judgement on framing, never on the defect.
   *(dated 2026-10-03, s8i build, R1FIX-8i)* The field above reads `sha256=<64 hex, one per capture file, comma-joined>`, as METHOD-S's
   amendment registers it (item 2, step 5) and §8i.5's packet writes it. ~~Item 3 has two captures, so its line carries
   two hashes.~~ ~~Checkpoint 8i (4) splits the field on `,` and requires exactly the item's pinned capture hashes, each
   once.~~
   *(dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's major)* The tie Checkpoint 8i makes, named. (4)'s
   `caplog_ok` splits an item's binding `yes` line's field on `,` and requires exactly the item's `fixture.json` capture
   hashes, each once. (4)'s `capfiles_ok` requires the pins block's `results/s-bin/captures/item-<n>/<name>` lines to be
   exactly the item's `fixture.json` captures, sha256 and path basename, each once, and none for an item with
   `captures: []`. (3) requires each pinned file to be its own blob at the pinned corpus commit. So the procedure log,
   `fixture.json` and the committed capture bytes are one set. The struck sentence named "pinned" hashes, which (4)
   did not read.
   *(dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's minors)* D21-149 and D21-150 widen the line's
   grammar. An item whose every asset `capture_scene_preview` rejected gets `sha256=none shows_subject=n/a` and
   `captures: []` (items 2 and 3, so item 3 has no capture, and the struck "two captures" no longer holds). A session
   that stalled on the instrument gets `sha256=none shows_subject=stalled`, at most one per item, ahead of the item's
   binding line, which is its last (item 5). Every line carries its model and cost. Registered by METHOD-S's
   D21149-8i-A extension, items 2 and 3; checked by Checkpoint 8i (4)'s `caplog_ok`. The log at corpus `cc7f222` holds
   six lines in this form.
6. Close the session and the editor. No `UnrealEditor` process survives (Halt 6). Delete the capture copy.

When the five captures are taken, build each item's review repository (the table above) before any rendering is
hashed.

A "no" on any tree is a stop (§8i.6), on a clean tree as much as a seeded one. N19 applies one procedure across
the five, and a clean item without its subject would be reviewed on less than the seeded ones. It never falls back
to empty captures. *(dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's minor)* Qualified by D21-149: an
empty capture list that is the tool's own rejection of every asset is not a fallback (`n/a`, METHOD-S's D21149-8i-A
extension, item 2), and "never falls back" stands for a framing failure.

*(dated 2026-10-02, s8i build)* **The base repositories, built (package S8I-P7); seed 5 awaits R.** Zero spend, and
read-only on `D:/repos/BrittonGames`. Its HEAD (`ee6e450`), the sha256 of `git --no-optional-locks status --porcelain`
(`a6751b0c…69ed`) and that of `git worktree list --porcelain` (`ba14505a…e0ab`) were the same before and after, and
`find .git -newer <ref>` lists nothing in its `.git`. Each tree was checked out from a disposable `git clone
--no-checkout --shared` under `D:/sb/tmp-p7` into `D:/sb/base/item-<n>`, as a separate work tree with the LFS smudge
skipped and `core.autocrlf=false`. Seed 4 was applied with `git checkout a71265925^ -- <path>`. Every LFS pointer was
then replaced by its object, read from BrittonGames' own `.git/lfs/objects`, and each materialised file's SHA-256 and
size equal its pointer's `oid` and `size` (30,014, 30,015, 30,022 and 30,014 ~~objects~~ *(dated 2026-10-03, s8i build, R3FIX-8i)* LFS pointer paths; 0 missing; the distinct objects number 29,576, 29,577, 29,584 and 29,576 [measured: `find .git/lfs/objects -type f | wc -l`, read-only]). The clone was then
deleted, and `D:/sb/tmp-p7` is gone. The strip removed `CLAUDE.md`'s four rows, each prefix once, from every tree,
and 19 files: `_workspace/backlog/`'s 18 and the gotchas reference. The set is one set, and the removed files are
the same 19 on all four trees. In the work tree `CLAUDE.md` carries CRLF (153 CRs, from BrittonGames'
`*.md text eol=crlf`) while its blob is LF, so the terminator-agnostic rule was exercised on CRLF bytes. `LoadoutFill`
counts 0 over every non-asset file of every tree, both before and after the commit. Each tree is the one commit of a
fresh repository: `main`, `review tree`, `s-bin <s-bin@invalid>`, `git rev-list --all --count` = 1, `refs/heads/main`
alone, no remote and no tag. `git status --porcelain --untracked-files=all --ignored` prints nothing. Each committed
tree equals BrittonGames' item tree **entry for entry** (seed applied, strip removed, `CLAUDE.md` = the byte rule
over BrittonGames' blob).

| item | commit | base commit | base tree | asset blob oid(s) | base bytes (`.git` included) |
| --- | --- | --- | --- | --- | --- |
| 1 | `cc0cf2d01` | ~~`85d73729bc908b6dce10eb40df8fa0ac27816ca1`~~ *(dated 2026-10-03, s8i build, D21146-8i)* ~~`37f0f0e2f679d58316edbf4da7f23ccebca8dde2`~~ *(dated 2026-10-03, s8i build, D21147-8i)* `58e9896335f2009a7772fb99d6df9cf123418dd4` | ~~`59b90c86`~~ ~~`7f1018de`~~ `6782b95b` | `WBP_StashRow` `75dc20f8` | ~~21,540,846,625~~ ~~21,540,843,118~~ 21,540,844,470 |
| 2 | `fa1ea72f4` | ~~`78a5ed29a4ac93d3b7685e35c6d6b9cebe7112a8`~~ *(dated 2026-10-03, s8i build, D21146-8i)* ~~`bdff598b83b4cbb5f1f27ca901697cf8aa615994`~~ *(dated 2026-10-03, s8i build, D21147-8i)* `e8954b617f4a8b30e7ed2b1dfd221655caf65186` | ~~`ca9a46bb`~~ ~~`3354f080`~~ `72ff014f` | `FTEST_Inventory_Stash` `f2335143` | ~~21,541,178,231~~ ~~21,541,174,637~~ 21,541,173,139 |
| 3 | `e83463288` | ~~`536eb55f3c3529bf5943b6c8ecf06da873824c8e`~~ *(dated 2026-10-03, s8i build, D21146-8i)* ~~`de3b4ca53758f014c321f1321ed9b6ce7c3997b9`~~ *(dated 2026-10-03, s8i build, D21147-8i)* `efcc24297540af65905c699cb41a25464dd69082` | ~~`81e0c570`~~ ~~`a2e25cc9`~~ `86bc1c51` | `BP_GamePlayerState` `f22e58e2`, `DA_Item_Sword` `2f9895f5` | ~~21,560,069,565~~ ~~21,560,066,805~~ 21,560,067,392 |
| 4 | `a71265925` | ~~`fd692b75ac96c6d94bf1de084e64bb03c422aaba`~~ *(dated 2026-10-03, s8i build, D21146-8i)* ~~`1553eda41de0b0f5a25a38ed5cdbea9d6258af6c`~~ *(dated 2026-10-03, s8i build, D21147-8i)* `2878d79dda6ae72a1143a63937834ed3fb327eff` | ~~`59b90c86`~~ ~~`7f1018de`~~ `6782b95b` | `WBP_LoadoutScreen` `9415f098` (the seed) | ~~21,540,845,865~~ ~~21,540,843,759~~ 21,540,843,094 |
| 5 | ~~**awaiting R**~~ | — | — | — | — |
| 5 *(dated 2026-10-03, s8i build)* | `6a101bdcd` | ~~`292cabad7e073a977010e7e84261cd4ad204389e`~~ *(dated 2026-10-03, s8i build, D21146-8i)* ~~`5f90d80ef4fed42822f955b944b5804cabfc3da6`~~ *(dated 2026-10-03, s8i build, D21147-8i)* `e0620bfaac31a388fc2863a4f01f8b3f1c16a0a4` *(dated 2026-10-04, s8i build, D21148-8i-A)* rebuilt and verified at `D:/sb/base-d21148/item-5`, NOT yet swapped in: `c2f948ecb902d972f19c2b4b87491be22159f314` *(dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's minor)* swapped into `D:/sb/base/item-5` by the operator on 2026-10-04 (the operator's swap note below); re-read 2026-10-06: HEAD `c2f948ec`, tree `a8b469e1` | ~~`143377a3`~~ ~~`370b48cd`~~ `f1c3d6cf` *(dated 2026-10-04, s8i build, D21148-8i-A)* `a8b469e1` | `WBP_GameVitals` `23c79a63` (the seed) | ~~10,604,498,965~~ ~~10,604,497,923~~ 10,604,498,743 *(dated 2026-10-04, s8i build, D21148-8i-A)* 10,604,502,675 |

- **Seed 4 is confirmed by blob oid.** `a71265925^:OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset` =
  `9415f0986275a66d01885cc74f58c33e1957e183` = base item 4's `HEAD:` blob. The fixed pointer blob `70e2a2e3…aa37c` is
  absent from base item 4's object store, and its LFS object `02b53079…397db` is absent from its `.git/lfs/objects`.
  The same probes return *present* on the seed's blob and LFS object (the control).
- **Measured, not chosen: base item 1 and base item 4 hold the same tree** (`59b90c86`). `a71265925^` *is* item 1's
  commit `cc0cf2d01`, and `a71265925` changes only `WBP_LoadoutScreen.uasset` plus two stripped backlog files. So the
  seeded tree and the clean item 1 tree are byte-identical. They differ only in the commit (and so `review_commit`),
  in the asset the summary names, and in the captures. Clean item 1's tree also carries seed 4's defect. This is the
  registration's own consequence, and it is recorded rather than worked around.
  *(dated 2026-10-03, s8i build, R1FIX-8i)* What it means for scoring, stated for the operator before B1 and not decided here. Under the
  registered rule a block on a clean item counts toward `undecided-fp` (≥ 2 of the 3 clean items blocked). So a
  reviewer of item 1 that blocks on `WBP_LoadoutScreen.uasset`, a defect really present in its tree but outside item
  1's change and its spec, is scored as a false positive. Re-labelling or replacing item 1, or a scoring carve-out,
  would re-open a registered selection or rule, and is the operator's. Refuting round 1 rated this a minor, and no
  ruling is taken here.
- **`git add -A -f`, not `add -A`.** BrittonGames tracks 11 files that its own `.gitignore` matches (`git ls-files -ci
  --exclude-standard`: 10 under `OurGame/Plugins/Monolith/Tools/`, and one Synty report CSV). A plain `add -A` drops
  them silently, so the commit would not be the tree, and `status --ignored` would show them. The directory holds only
  the item's tracked set, so `-f` adds exactly that set. The entry-for-entry comparison proves it. **Owed to the
  driver:** P4/P5's `commit_single` runs `add -A`. If it builds the review repositories, it drops the same 11 files.
  It needs `-f`, or the review repositories are built the way the bases were. Recorded here, not edited in the patch.
- **Item 5 awaits R.** P3 found that each of its three qualifiers is refused by amendment item 3's `CLAUDE.md` strip
  (prefix counts 1/0/0/0). Base item 5 waits on that blocker's ruling and the operator's pick. When R is picked, the
  same script builds it: `python D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P7_build.py 5`, after an `ITEMS[5]`
  entry is added.
- **`fixture.json`, drafted and staged (not in the corpus).** It is at
  `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P7-results/results/s-bin/fixture.json`. It holds `items.1`..`4`, with
  `stripped`, `leak_hits`, `asset_paths`, `asset_oids`, `asset_actions`, `seed_blobs`, `fix_blobs`, `fix_lfs_oids` and
  `text_diff` (empty), plus provenance keys. `review_commit`, `capture_path` and `captures` are null/null/[] until
  after B1. `build_exit` waits for the greenness runs. `slug`, `spec_path`, `report_path` and `ac_ids` belong to the
  constructed-spec work, and `ceiling_secs` waits for extension 2.
- **The pin report: the registered source was overwritten.** BrittonGames' `_workspace/tooling/TestResults/index.json`
  now hashes `65960f78…1a8a` (mtime 2026-09-25 12:41 -07:00), not `c3dae802…6bc6`. A byte-identical copy of the
  2026-09-05 report survives at `_workspace/_runs/8.3.3g-20260904-2334/final-suite.json` (mtime 2026-09-05 01:40
  -07:00, `reportCreatedOn` `2026.09.05-08.40.33`, 849/180/0/0). A second copy is at
  `_workspace/_runs/story-8.3.3f/baseline-suite.json`. The staged copy, at
  `…/S8I-P7-results/results/s-bin/pin-report-2026-09-05/index.json`, was taken from the first. Its SHA-256 is
  `c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6`, which is what guard (3) asserts. Copying from
  that path is disclosed for the operator's acceptance at the corpus commit.
- **Disk.** Four bases take 86.2 GB, and D: went from 316 GB to 235 GB free. Each base is about 10.8 GB of work tree
  plus 10.7 GB in `.git/lfs`. At peak, five bases, five review repositories and one dispatch copy need about 237 GB
  plus one run's growth.

*(dated 2026-10-03, s8i build)* **Base item 5, built (package S8I-F1).** R = `6a101bdcd` (D21-142 item 2). Built exactly as items 1–4, by
`s8i_S8I-F1_build5.py`, which `s8i_S8I-F1_mkbuild5.py` derives from P7's script by anchored replacements: `ITEMS[5]`,
D21-142's second `CLAUDE.md` rule and R's three leak tokens, nothing else. A disposable `git clone --no-checkout
--shared` under `D:/sb/tmp-f1` (deleted afterwards); seed 5 applied as `git checkout 6a101bdcd^ --
OurGame/Content/UI/HUD/WBP_GameVitals.uasset`; 11,886 ~~LFS objects~~ *(dated 2026-10-03, s8i build, R3FIX-8i)* LFS pointer paths (11,447 distinct objects, measured the same way) materialised from BrittonGames' store, each
file's SHA-256 and size equal to its pointer's, none missing. BrittonGames' HEAD (`ee6e450`), status sha256
(`a6751b0c…69ed`) and worktree-list sha256 (`ba14505a…e0ab`) were the same before and after.
- **`CLAUDE.md`, by the old-table rule.** `git merge-base --is-ancestor 07fe80ab6 <item>` reads rc 0 at items 1–4
  (item 3's rule, unchanged) and rc 1 at R, so the second rule applied and removed `| **Epic** |`, `| **Feature** |`,
  `| **Focus** |` and `| **Next** |`, each once (4 rows), from a CRLF work file (111 CRs; 19,439 → 14,663 bytes).
  Controls (`s8i_S8I-F1_controls.py`, `CONTROLS_OK`): the rule refuses 3 rows, a duplicate and the new table; item
  3's rule refuses the old table (1/0/0/0); LF, CRLF and mixed input strip alike.
- **The strip set is items 1–4's**, `["CLAUDE.md#status-rows", "_workspace/backlog/",
  "_workspace/references/UE58_Editor_And_MCP_Gotchas.md"]`. It removed 8 files, not 19: R's `_workspace/backlog/`
  holds 7 files.
- **Leak tokens.** `LoadoutFill`, `white-on-white`, `VERIFIED COLD` and `entirely white` count 0 over every non-asset
  file of all five base trees (`s8i_S8I-F1_leak5.py`, `ALL_ZERO`; 3,643, 3,650, 3,666, 3,643 and 3,243 files), and 0 in
  base 5 before and after its commit. The scan's controls find each token when planted and skip `.git` and assets.
- **One commit.** Base commit `292cabad7e073a977010e7e84261cd4ad204389e`, tree
  `143377a3878f9e40a5d937826da9e0ffe9e0d8f8`; `main`, `review tree`, `s-bin <s-bin@invalid>`; `rev-list --all --count`
  1, `refs/heads/main` alone, no remote; `status --porcelain --untracked-files=all --ignored` empty; the tree equals
  BrittonGames' R tree entry for entry (15,066 entries) with the seed applied and the strip removed. Committed with
  `add -A -f`, as items 1–4 were: R tracks 10 files its `.gitignore` matches.
- **The seed, by blob oid.** `6a101bdcd^:OurGame/Content/UI/HUD/WBP_GameVitals.uasset` =
  `23c79a6337215a7a208d55676081f0fc33fcbeb7` = base 5's `HEAD:` blob. The fixed pointer blob
  `c84ea7acc32c9ff499b6ee6ad3444b3692e96f18` is absent from base 5's object store, and its LFS object
  `59aa3581…01538510` from its `.git/lfs/objects`. The asset's work file hashes to its pointer's oid `86123be7…0261b`.
- **The record** is `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-F1-out/item-5.json`. A copy is at
  `…/S8I-P7-out/item-5.json` so that `s8i_S8I-P7_green.py table` counts base 5's bytes. `fixture.json`'s `items.5` is
  still to be added from it (the staged fixture holds items 1–4).
- **Disk.** Base 5 is 10.6 GB; D: reads 203 GB free.

The greenness runs below now include item 5: its base exists. Its suite at R is the one `verify_headless.ps1` runs
there, far smaller than the pin's 1,029 tests (S8I-P3: about 175, from the commits' own messages).

*(dated 2026-10-03, s8i build, D21146-8i)* **The five bases, rebuilt without the editor launcher (D21-146 items 1 and 2).** The table's struck base commits, trees and bytes are the bases P7 and F1 built. The values beside them are the rebuilt bases, now at `D:/sb/base/item-1..5`. They were built by `s8i_D21146-8i_build.py`, which `s8i_D21146-8i_mkbuild.py` derives from F1's script by nine anchored replacements: the launcher's removal, the strip set's fourth entry `_workspace/tooling/RunEditor_OurGame.bat`, and a new base root and record directory. They were built beside the old bases, under `D:/sb/base-d21146`, from a disposable `--no-checkout --shared` clone under `D:/sb/tmp-d21146`, deleted afterwards. The old bases were replaced only after all five verified (`s8i_D21146-8i_verify.py`, `ALL_OK`, `D21146-8i-verify-all.json`). Read-only on BrittonGames: HEAD `ee6e450`, status sha256 `a6751b0c…69ed`, worktree-list sha256 `ba14505a…e0ab` and config sha256 the same before and after.
- **Each new tree is the old tree less the launcher**, entry for entry. No other entry is added or changed, so the seeds, `CLAUDE.md`'s byte rules and the other strip entries are the old bases' bytes. Base items 1 and 4 still hold one tree (`7f1018de67a96c2c0b545d024838ea62b126c837`).
- **The checks P7 and F1 ran, re-run.** Every LFS pointer's work file matches its pointer's SHA-256 and size, and its object is in the base's `.git/lfs/objects`: 30,014, 30,015, 30,022, 30,014 and 11,886 pointer paths, 0 bad. The four leak tokens count 0 over every non-asset file of all five, before and after each commit. The seeds match by blob oid (`9415f098…` at item 4, `23c79a63…` at item 5). The fixed blobs and their LFS objects are absent, and the control probes fire. Each base is a single commit with `status --ignored` empty. The stripped set is one set across the five, `["CLAUDE.md#status-rows", "_workspace/backlog/", "_workspace/references/UE58_Editor_And_MCP_Gotchas.md", "_workspace/tooling/RunEditor_OurGame.bat"]`. It removed 20 files at items 1 to 4 and 9 at item 5. Verifying an old base against itself fails (the control).
- **The staged fixture follows.** `S8I-P7-results/results/s-bin/fixture.json`'s items 1 to 4 carry the new `stripped`, `files_removed`, `base_commit`, `base_tree` and `base_bytes`. The superseded file is kept under `S8I-D21146-out/superseded/`. Item 5 is still owed to it, as F1 recorded. `S8I-P7-out/item-<n>.json` are now the rebuilt records, so that `s8i_S8I-P7_green.py table` reads the new bytes. P7's and F1's are kept under `S8I-D21146-out/superseded/`.
- **The dry run's review repository, rebuilt** (§8i.3's S8I-P8 note). It was a byte copy of the old base item 1, so it held the launcher, which `precheck_exec_paths` refuses. It is rebuilt from the rebuilt base item 1 by `s8i_D21146-8i_review.py`, which is S8I-P8's script with its two pins of base item 1 moved (`37f0f0e2…`, tree `7f1018de…`). It is now one parentless commit, `48cd028e2b83b65e12d456c4ca2e2a258cbed593`, superseding `378a183f…7b36`, with `VERIFY OK`: base item 1's tree plus exactly the two `_s-bin/report/` files. S8I-P8's `s8i_S8I-P8_dryinputs.py` was re-run unchanged, and the dry run's fixture now hashes `8f98bf430ddf6f461217eab9f89305b6d279bdec011f5282aac691e67f728ea1`, superseding `4b16a580…ee51`. It moves item 1's `review_commit` and items 1 to 4's base keys, and nothing else. The superseded inputs are kept under `S8I-D21146-out/superseded/S8I-P8-dryrun/`. The check reads `[]` over the rebuilt dry-run repository.
- **The measured class** (D21-146 item 4) is listed in METHOD-S's D21146-8i extension, item 2. Only the launcher held a path in executable code. The eight `author_*.py` and five `synty/` scripts (two in item 5) hold theirs in module-docstring usage text, and they are not stripped.

*(dated 2026-10-03, s8i build, D21147-8i)* **The five bases, rebuilt with the backlog stub (D21-147 item 3; package D21147-8i-B).** The operator ruled on 2026-10-03 (D21-147): *"The five bases are rebuilt at zero spend, with the leak-token, blob-oid, LFS and strip checks re-run; the dry run's review repository is rebuilt from the new base item 1, and its commit and fixture hash move; all five greenness runs are redone on the new bases"*. The table's twice-struck values are P7's and F1's bases, the once-struck ones D21146-8i's, and the last values are the D21-147 bases, now at `D:/sb/base/item-1..5`. They were built by `s8i_D21147-8i_build.py`, which `s8i_D21147-8i-B_mkbuild.py` derives from D21146-8i's script by eight anchored replacements: the stub's bytes (asserted by sha256 and length at load), its write right after the whole-directory backlog strip (tmp + `os.replace`, re-read, and `_workspace/backlog/` holding that one file), the expected tree admitting exactly that one blob, a check of the committed stub, and a new base root, clone root and record directory (`S8I-D21147-out`). They were built beside the old bases, under `D:/sb/base-d21147`, from a disposable `--no-checkout --shared` clone under `D:/sb/tmp-d21147`, which the script deleted. The old bases were replaced only after all five verified (`s8i_D21147-8i-B_verify.py`, `ALL_OK`, `D21147-8i-B-verify-all.json`), by `s8i_D21147-8i-B_swap.py` (`SWAP_OK`), which deleted each old base after re-reading the new one's HEAD, tree, status and count in place; `D:/sb` now holds `base`, `dry` and an empty `green` only. D: read 163.70 GB free before and after. Read-only on BrittonGames: HEAD `ee6e450`, status sha256 `a6751b0c…69ed`, worktree-list sha256 `ba14505a…e0ab` and config sha256 `f8f1c5ed…adea` the same before and after (`D21147-8i-B-bg-before.json`, `D21147-8i-B-bg-after.json`), and no `.git/worktrees` entry.
- **Each new tree is the D21146-8i tree plus the stub**, entry for entry: one entry only in the new tree, `_workspace/backlog/game_foundation_backlog.json`, none only in the old, none changed (33,569 → 33,570 entries at items 1 and 4, 33,577 → 33,578, 33,594 → 33,595, and 15,065 → 15,066 at item 5). So the seeds, `CLAUDE.md`'s byte rules, the four strip entries and the launcher's removal are D21146-8i's bytes. Base items 1 and 4 still hold one tree (`6782b95bf399274efbc6e20ea28ab00a5976c958`).
- **The stub, in every base.** `HEAD`'s `_workspace/backlog/` lists exactly `100644 blob 999761537faf3961b3424f1cb3a36bbcd2cec967`, the blob of the registered bytes. The work tree's `_workspace/backlog/` holds that one file, sha256 `211a4dcf7be9eb68ef73dfcb6e67a832b1713f1e3c9aa8f148fb52df189f8787`, LF, which parses as `{"stories":[…]}` with `8.3.5a`, `8.3.5b` and `8.5.8`, each `{"id","status":"todo"}` and nothing else. BrittonGames' `.gitattributes` reads `*.json text eol=crlf` at every item commit, so the work file is written, never checked out, and stays LF; its blob is the same LF bytes, and `status --ignored` is empty. At every item the stub replaced a real `game_foundation_backlog.json` among the removed files, which are D21146-8i's: 20 at items 1 to 4, 9 at item 5. In bases 2 and 3 the stub's reader, `Architecture_NoInertCapability.spec.cpp`, is BrittonGames' blob (sha256 `81e90ac9…8628` and `af0f7643…f396`, the ones D21147-8i-A read).
- **The checks P7, F1 and D21146-8i ran, re-run.** Every LFS pointer's work file matches its pointer's SHA-256 and size, and its object is in the base's `.git/lfs/objects`: 30,014, 30,015, 30,022, 30,014 and 11,886 pointer paths (29,576, 29,577, 29,584, 29,576 and 11,447 distinct objects), 0 bad. The four leak tokens count 0 over every non-asset file of all five (3,643, 3,650, 3,666, 3,643 and 3,243 files), the stub included, before and after each commit. The seeds match by blob oid (`9415f098…` at item 4, `23c79a63…` at item 5); the fixed blobs and their LFS objects are absent, and the control probes fire. Each base is a single commit, `refs/heads/main` alone, no remote or tag, `status --porcelain --untracked-files=all --ignored` empty. The stripped set is one set across the five and is D21146-8i's four entries; every record carries `written` = the stub. No executable file holds an absolute answer-bearing path in code. The verifier's controls fire: the registered stub passes, and CRLF, a missing LF, a byte-order mark, a `done` status, a pretty-printed equivalent, a second file, a missing stub and a missing directory each fail; verifying a D21146-8i base against itself reads `NOT_OK` (`D21147-8i-B-verify-ctl-oldold.out`).
- **The driver's own check over the real trees** (D21147-8i-A's control copy `D21147-8i-A-ctl`, over the shipped code, under `nd.sh`): each new base and the rebuilt dry-run repository read `backlog_stub_ok=true` and `exec_path_hits=[]`, where the D21146-8i base item 1 and the old dry-run repository read `false` (`D21147-8i-B-ctl-new.out`, `D21147-8i-B-ctl-final.out`).
- **The staged fixture follows.** `S8I-P7-results/results/s-bin/fixture.json`'s items 1 to 4 carry the new `base_commit`, `base_tree` and `base_bytes` (`stripped` and `files_removed` are unchanged: the stub is written, not removed), and its `draft` names the stub; it now hashes `797bae1da3f1deb02241002db620d5457d6765977aefb7c98c75d71336b27997 *(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* superseded by `b931e4b19effd7a8fb233d3cfee68f8844fa37ca30d8e25e26ca8911e745b3ca`: its `draft` wrongly said `files_removed` moved`, superseding `abe90a20…6846`, which is kept under `S8I-D21147-out/superseded/`. Item 5 is still owed to it, as F1 recorded. `S8I-P7-out/item-<n>.json` are now the D21-147 records, so that `s8i_S8I-P7_green.py table` reads the new bytes; D21146-8i's are kept under `S8I-D21147-out/superseded/`.
- **The dry run's review repository, rebuilt** (§8i.3's S8I-P8 note). It was a byte copy of the D21146-8i base item 1, with no stub, which `precheck_backlog_stub` refuses. It is rebuilt from the new base item 1 by `s8i_D21147-8i-B_review.py`, which `s8i_D21147-8i-B_mkreview.py` derives from S8I-P8's script (sha256 `83a7793d…c8df4`) with its two pins of base item 1 moved (`58e98963…`, tree `6782b95b…`). It is now one parentless commit, `a8d9478b54556d6617963cab622b385109928ab3` (tree `4bf6ff94d001720f2f550448cb3bcc9d42bfb7e5`), superseding `48cd028e…`, with `VERIFY OK`: base item 1's tree plus exactly the two `_s-bin/report/` files (`D21147-8i-B-dry-build.out`). S8I-P8's `s8i_S8I-P8_dryinputs.py` (sha256 `c7f4f413…56c3`) was re-run unchanged, and the dry run's fixture now hashes `62f6469c11e3bc5e656b565ba28c79709ad30526b954d88017ebd69024b6a8b1`, superseding `8f98bf43…ea1`. It moves item 1's `review_commit` and items 1 to 4's `base_commit`, `base_tree` and `base_bytes`, and nothing else; the placeholder spec and report are byte-identical. The superseded inputs are kept under `S8I-D21147-out/superseded/S8I-P8-dryrun/`, and the old review commit in `superseded/dry-rev-head.txt`. The dry-run script `s8i_S8I-P8_dryrun.sh` is unchanged (sha256 `5d112e09…2c2c`): it reads its inputs from `S8I-P8-dryrun`, and the selection stays at 33 tests and the record at 34 names. §8i.5's step-4 pins of the commit and fixture hash are outside this note.
- **The superseded greenness runs, set aside.** Three runs were staged on the D21146-8i bases: item 1 (892 tests, 0 failed, exit 0), item 2 (920, 2 failed, exit 1) and item 3 (997, 2 failed, exit 1). Item 3's two failures are the same strip-caused `OurGame.Architecture.NoInertCapability` pair as item 2's (its anti-blindness control on `8.3.5b`, and the waiver list). All three are superseded by D21-147 item 3, item 3's included, and were moved, not deleted, to `S8I-D21147-out/superseded/greenness/item-<n>/` (`s8i_D21147-8i-B_supersede.py`). The staging directory is empty, so the five re-runs start clean. `s8i_D21147-8i-B_prepcheck.py` reads `prepare`'s preconditions without running it, and they hold for all five: the base repository exists, no greenness copy exists, D:'s free space is above 1.1 × the base, no state is staged, and `S8I-P7-out/item-<n>.json` is the D21-147 record whose `base_commit` is the base's HEAD (`PREPARE_PRECONDITIONS_HOLD`, `D21147-8i-B-prepcheck.out`).

**The operator's greenness runs (not run here).** Run them from a plain Git Bash **outside Claude Code**, with the
Unreal editor **closed**, one item at a time, each in a subshell. The helper refuses inside a Claude Code session,
and the refusal has been shown to fire. It also refuses a reused copy, and it refuses (Halt 6) when an
`UnrealEditor` process exists before a run or survives one.

```bash
( H=D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P7_green.py; N=1   # then N=2, 3, 4, and 5 once R's base exists
  python "$H" prepare $N && python "$H" run $N && python "$H" collect $N && python "$H" cleanup $N )
python D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P7_green.py table
```

`prepare` makes a byte copy of `D:/sb/base/item-<n>` to `D:/sb/green/item-<n>`, with `.git` included and never a
clone, after a free-space check. It records the size before. `run` runs the registered `pwsh -File
_workspace/tooling/verify_headless.ps1` in the copy, which builds and then runs every `OurGame.*` test, and records
the exit and the wall clock. `collect` copies `index.json` and `headless.log` to
`…/S8I-P7-results/results/s-bin/greenness/item-<n>/`, hashes each, folds `passed`, and records the size after.
`cleanup` deletes the copy. Fill in:

| item | seeded | exit | failed | wall s (build + suite) | suite s (`totalDuration`) | copy bytes before | copy bytes after | green (`failed`=0, exit 0) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | no | 0 | 0 | 693.1 | 41.0 | 21,540,844,470 | 31,061,486,590 | yes *(dated 2026-10-04, s8i, operator sitting)* |
| 2 | no | 0 | 0 | 728.0 | 51.4 | 21,541,173,139 | 31,067,922,220 | yes *(dated 2026-10-04, s8i, operator sitting)* |
| 3 | no | 0 | 0 | 755.2 | 56.0 | 21,560,067,392 | 31,110,202,834 | yes *(dated 2026-10-04, s8i, operator sitting)* |
| 4 | **yes** | 0 | 0 | 702.7 | 41.1 | 21,540,843,094 | 31,061,607,828 | yes *(dated 2026-10-04, s8i, operator sitting)* |
| 5 | **yes** *(dated 2026-10-04, s8i build, D21148-8i-B)* run only on the rebuilt base `c2f948ec`, after the operator swaps it into `D:/sb/base/item-5`; the run on `e0620bfa` (175 tests, 2 failed, exit 1) is superseded (D21-148), and this one binds | | | | | | | must be yes |
| 5 | **yes** *(dated 2026-10-04, s8i, operator sitting)* the binding run on `c2f948ec` | 0 | 0 | 644.4 | 13.1 | 10,604,502,675 | 19,583,418,307 | yes |

The two formulas, pending the operator's numbers:
- `ceiling_secs` = 3600 + `W_max`, rounded up to the next multiple of 300 s, where `W_max` is the slowest wall clock
  (amendment item 7). That is never below 3600 s. The package brief's "max(slowest, 3600 s)" is not the registered
  rule.
- `SBIN_NEED_BYTES` = ceil((base size + growth) × 1.1), using the largest base and the largest growth over the five
  (amendment item 4).

`table` prints both once five runs exist. A red seed is replaced under item 12 before B2. The METHOD-S extension
paragraph for deferred extensions 2, 3 and 4 is drafted at `…/S8I-P7-extension-draft.md` and is **not** appended.

*(dated 2026-10-03, s8i build, D21147-8i)* **`UnrealTraceServer.exe` is not Halt 6 (D21-147 item 4).** The operator ruled on 2026-10-03: *"Before and after each greenness run the operator confirms no `UnrealEditor*` process and ends `UnrealTraceServer.exe`; the helper is not changed."* The helper's `unreal_procs()` matches any process name containing `unreal`, which is broader than the registered rule, and it stopped items 1 and 2 on `UnrealTraceServer.exe` alone. So, around each of the five re-runs on the D21-147 bases: before `run`, confirm no `UnrealEditor*` process and end `UnrealTraceServer.exe`; after it, the same. A surviving `UnrealEditor*` process is still Halt 6. The re-runs start from an empty staging directory (the superseded runs are under `S8I-D21147-out/superseded/greenness/`). The METHOD-S extension *(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* paragraph for deferred extensions 2, 3 and 4 is unchanged by this ruling, and it is appended only after the five re-runs.

*(dated 2026-10-04, s8i build, D21148-8i-A)* **Base 5 rebuilt with BrittonGames' later verify script (D21-148; package D21148-8i-A). The swap into `D:/sb/base/item-5` is NOT done: do not run item 5's greenness until it is.** The operator ruled on 2026-10-04 (D21-148 item 1): *"Tree 5's `_workspace/tooling/verify_headless.ps1` is set to blob `ebe7e58f…`, so that all five trees carry one identical script."* Item 2: *"Base 5 alone is rebuilt at zero spend, with its leak-token, seed blob-oid, LFS, strip, D21-147 stub and exec-path checks re-run. Bases 1 to 4 and their green reports stand."*
- **Built** by `s8i_D21148-8i-A_build.py`, which `s8i_D21148-8i-A_mkbuild.py` derives from `s8i_D21147-8i_build.py` by eight anchored replacements: after the strip and the D21-147 stub, the work file `_workspace/tooling/verify_headless.ps1` is set to the bytes of blob `ebe7e58f3df5756ad016d9075512a27cd2417fae` (BrittonGames `71e8f6c91`, read-only by `cat-file`) via tmp + `os.replace`, and the expected tree carries that blob; a refusal unless the item tree's blob was R's `a2047f2d836d2d48e1f37c2145a2f1328091736f` and the checked-out work file equals R's blob bytes (so no eol filter applies to this path); a post-commit check of the committed blob; base 5 alone (any other item list refuses); and new base, clone and record roots (`D:/sb/base-d21148`, `D:/sb/tmp-d21148`, deleted by the script, `S8I-D21148-out`). Built beside the old base, at `D:/sb/base-d21148/item-5`.
- **Verified** (`s8i_D21148-8i-A_verify.py`, derived from `s8i_D21147-8i-B_verify.py` by seven anchored replacements; `ALL_OK`, `D21148-8i-A-verify-5.json`). Base commit `c2f948ecb902d972f19c2b4b87491be22159f314`, tree `a8b469e1c7fae157c2a6876f10907d4c82160e6c`, 10,604,502,675 bytes (`.git` included). Against the D21-147 base 5 (`e0620bfa`, tree `f1c3d6cf`) exactly one entry differs, `_workspace/tooling/verify_headless.ps1`, `a2047f2d` → `ebe7e58f`, mode 100644; none only in either tree (15,066 entries). The work file (10,057 bytes, LF, sha256 `4fa3509eb8c5f5dbeebf9c9ff18527fed499aca26806758f66d48d50a440d036`) hashes to `ebe7e58f` and is byte-identical to base 1's work file; bases 1 to 4 carry the same blob (`git rev-parse HEAD:_workspace/tooling/verify_headless.ps1`). The checks re-run: 11,886 LFS pointer paths (11,447 distinct objects), 0 bad; the four leak tokens 0 over 3,243 non-asset files, before and after the commit; the seed `23c79a63…` by blob oid; the fixed blob `c84ea7ac…` and its LFS object absent, the control probes present; one commit, `refs/heads/main` alone, no remote or tag, `status --porcelain --untracked-files=all --ignored` empty; the stripped set the one four-entry set, equal to items 1 to 4's records; the D21-147 stub at its registered bytes; no executable file with an absolute answer-bearing path in code. The verifier's controls fire, and verifying the D21-147 base 5 against itself reads `NOT_OK` (`D21148-8i-A-verify-ctl-oldold.out`). The driver's own checks, from D21147-8i-A's control copy under `nd.sh`, read `backlog_stub_ok=true exec_path_hits=[]` over the new base 5 (`D21148-8i-A-ctl.out`). Read-only on BrittonGames: HEAD `ee6e450`, status sha256 `a6751b0c…69ed`, worktree-list sha256 `ba14505a…e0ab` and config sha256 `f8f1c5ed…adea` the same before and after (`D21148-8i-A-bg-before.json`, `D21148-8i-A-bg-after.json`).
- **The driver needs no change** [read 2026-10-04, read-only, the private copy `S8I-D21147-corpus`, `sbin.rs` sha256 `1cba3304…aa29`, which is D21147-8i's shipped driver]. `FixtureItem` has no base commit or base tree key; `precheck_blob_oids` compares the dispatch copy's HEAD with the fixture's `review_commit`, which is filled after B1 from the base; `confirm_seed` and the BrittonGames tree comparison run only in tests; `TESTS_COMMAND` is the command string, not the script's bytes; and `precheck_exec_paths` (`.ps1` is in `EXEC_EXTS`) reads `[]` over the new base 5. The sbin selection reads 33 passed, 0 failed, and the whole runner suite 167 passed and 1 failed (`p7::tests::the_driver_matches_the_committed_catalog`, the same failure D21147-8i-A read), under `nd.sh` (`D21148-8i-A-sel.out`, `D21148-8i-A-whole.out`). No driver patch supersedes `S8I-D21147-8i-sbin-driver.patch`.
- **Item 5's red run, superseded and set aside** (D21-148 item 3: *"the red run is kept as superseded, with this diagnosis"*). `state.json`, `index.json` and `headless.log` were moved, not deleted, to `S8I-D21148-out/superseded/greenness/item-5/` (`s8i_D21148-8i-A_supersede.py`, `ITEM5_RED_SET_ASIDE`), after checking base `e0620bfa`, exit 1, 2 failed, 175 tests and the recorded file hashes. D21-148's reading, re-measured from the files (`S8I-D21148-out/superseded/greenness/item-5-diagnosis.json`): both failures are `OurGame.Combat.AttackRootMotion.BlendAttackRootMotionVelocity`, whose error events are `LogBlueprint` messages from `BP_AnimNotify_FoleyEvent` and `BP_NotifyState_MontageBlendOut` only; the log holds 1,383 `LogMonolithIndex` lines, 0 `-DisablePlugins=Monolith` and 0 `WBP_GameVitals`. Items 1 to 4's staged runs (892, 920, 997 and 892 tests, 0 failed, exit 0) each hold the flag twice and 0 indexer lines; they stay in place, byte-unchanged. The staging directory holds `item-1` to `item-4` only.
- **Not moved.** The dry run's review repository is still `a8d9478b…` (tree `4bf6ff94…`), and its fixture still hashes `62f6469c…b8b1`. The staged fixture still hashes `b931e4b1…b3ca` and holds `items.1` to `4` only (owed item (e)); its `pending.5` text predates R's pick and names no base value, so it is left as it is.
- **NOT done: the swap (a permission-classifier denial in the AFK session, final under its rule 8).** The script that would rename `D:/sb/base/item-5` aside, move `D:/sb/base-d21148/item-5` into its place, re-read HEAD, tree, status and count there, delete the old base and replace `S8I-P7-out/item-5.json` with `S8I-D21148-out/item-5.json` (base_path moved) was refused as irreversible local destruction, and was not written. So `D:/sb/base/item-5` is still the D21-147 base `e0620bfa` with R's script, `D:/sb/base-d21148/item-5` holds the verified rebuild, and `S8I-P7-out/item-5.json` is still the D21-147 record. `prepare 5` would copy the OLD base: `s8i_D21148-8i-A_prepcheck.py` reads `PREPARE_PRECONDITIONS_FAIL` for item 5 (the record and the base's script blob), while the rest hold (no greenness copy, `D:/sb/green` empty, no staged state, free space above 1.1 × the base) (`D21148-8i-A-prepcheck.out`). The swap is owed to the operator, then the prepcheck re-read, then the one binding re-run of item 5 (D21-148 item 3: it must read 175 tests; red again with Monolith disabled returns to the operator).

*(dated 2026-10-04, s8i, operator sitting)* **The swap is done, by the operator.** From a plain Git Bash the operator renamed `D:/sb/base/item-5` (`e0620bfa`) aside, moved `D:/sb/base-d21148/item-5` into its place, re-read it there (HEAD `c2f948ecb902d972f19c2b4b87491be22159f314`, `status --ignored` empty, `verify_headless.ps1` blob `ebe7e58f`), replaced `S8I-P7-out/item-5.json` with the D21148 record (`base_path` `D:/sb/base/item-5`; the D21-147 record and the build-time record are kept under `S8I-D21148-out/superseded/`), then deleted the old base. `s8i_D21148-8i-A_prepcheck.py` reads `PREPARE_PRECONDITIONS_HOLD` (its first run failed only on its own `p7 == rec` condition, because the D21148 record still named the build-time path; the record's `base_path` was aligned). Item 5's single binding greenness run may start.
- *(dated 2026-10-04, s8i build, D21148-8i-B)* **Re-read, read-only (package D21148-8i-B; `s8i_D21148-8i-B_check.py`, `D21148-8i-B-check.json`).** `D:/sb/base/item-5` still reads `e0620bfa…`, tree `f1c3d6cf…`, script blob `a2047f2d…`; `D:/sb/base-d21148/item-5` reads `c2f948ec…`, tree `a8b469e1…`, script blob `ebe7e58f…`, one commit, `status --ignored` empty, 10,604,502,675 bytes; their `ls-tree -r` differ in that one entry only (15,066 entries each). Bases 1 to 4 read script blob `ebe7e58f…`. In BrittonGames, read-only: `71e8f6c91:` holds `ebe7e58f…` (10,057 bytes, 0 CR, the flag twice; committed 2026-08-18), `6a101bdcd:` holds `a2047f2d…` (6,654 bytes, the flag 0 times), and `6a101bdcd` is an ancestor of `71e8f6c91`; the four leak tokens count 0 in `ebe7e58f…`. The superseded run's three files hash as METHOD-S's D21148-8i extension item 3 records; the staging directory holds `item-1` to `item-4`; the staged fixture still hashes `b931e4b1…b3ca` with `items.1` to `4`; `S8I-P7-out/item-5.json` still names `e0620bfa…`. BrittonGames' HEAD `ee6e450`, status, worktree-list and config sha256 the same before and after.

*(dated 2026-10-06, s8i build, D21149-8i-B)* **Owed item (e) built: the five constructed spec/report pairs, the five review repositories, and `fixture.json` completed (package D21149-8i-B; proposed, nothing in the corpus).** All zero spend, nothing dispatched, read-only on `D:/repos/BrittonGames` and on the corpus.
- **The review repositories, by the driver's own `commit_single`.** `s8i_D21149-8i-B_prep.py` byte-copied each base's work tree **without its `.git`** to `D:/sb/rev/item-<n>` and added `_s-bin/report/index.json` and `_s-bin/report/headless.log` (the item's greenness report, at the sha256 METHOD-S's extension of 2026-10-04 records) and, on items 1, 4 and 5, `_s-bin/captures/capture-1.png` (the B1 capture of asset 1, read from the corpus object store at `cc7f222`). Items 2 and 3 carry no capture (D21-149). Each tree was then committed by the shipped driver's `commit_single(tree, Some(tracked))`, called from an `#[ignore]`d test in a never-shipped control copy (`D21149-8i-B-ctl`, package A's `sbin.rs` `bcdf0364…` plus two control tests) under `nd.sh`, where `tracked` is the base's `git ls-files` plus exactly the added paths, so the 10 or 11 tracked-ignored files and the `*.log`-ignored report are force-added and nothing else is. The commit times are the build's own (not pinned).

  | item | review_commit | tree | entries | added to the base tree |
  | --- | --- | --- | --- | --- |
  | 1 | `601035c43c250c2d2041ccf33acc2ecdb74f69c1` | `5c922bd9737c06885f2731f329263a2160fe229b` | 33,573 | `_s-bin/captures/capture-1.png`, `_s-bin/report/headless.log`, `_s-bin/report/index.json` |
  | 2 | `c246d0e3da63e8d4c8788b4b5da8651530f0d93d` | `fb294132a790c3bcef36f9e4a1738eacf4f37644` | 33,580 | `_s-bin/report/headless.log`, `_s-bin/report/index.json` |
  | 3 | `73ce8121c755c8c08f71b55e9637650582c18f52` | `abf11da246b0af0547feefbed6392385ba16d27b` | 33,597 | `_s-bin/report/headless.log`, `_s-bin/report/index.json` |
  | 4 | `b928fa3e68b942242ebb8487c858fb2619237e43` | `1b7db6cb5c92756a158854149091296f42317b57` | 33,573 | `_s-bin/captures/capture-1.png`, `_s-bin/report/headless.log`, `_s-bin/report/index.json` |
  | 5 | `08a296c93cb832c79a93c45a5ecfde5d5f8755b8` | `39f16e4b68ceb9f2bc3f275a53ac3b3f5866f1ab` | 15,069 | `_s-bin/captures/capture-1.png`, `_s-bin/report/headless.log`, `_s-bin/report/index.json` |

- **Verified** (`s8i_D21149-8i-B_verify.py`, read-only, `ALL_OK`): one parentless commit on `main`, `review tree`, `s-bin <s-bin@invalid>`, `rev-list --all --count` 1, `refs/heads/main` alone, no remote; `status --porcelain --untracked-files=all --ignored` empty; the base commit absent; the tree equals the base tree **entry for entry** plus exactly the added paths; every asset's blob is the fixture's `asset_oid` and its LFS object is present and hashes to its pointer; the fixed blobs and LFS objects absent on items 4 and 5; the report and capture bytes as registered, the capture committed as its LFS pointer with its object present; `_workspace/backlog/` exactly the D21-147 stub; the four leak tokens 0 over every non-asset work file. The control, the same checks over a base, reads `NOT_OK`.
- **The driver's own checks** (`d21149b_ctl_prechecks_over_the_review_repos`, under `nd.sh` with `GIT_OPTIONAL_LOCKS=0`): over each review repository `precheck` reads `precheck_clean` true, `precheck_blob_oids` true, `precheck_leak_hits` 0, `precheck_no_project_dir` true, `precheck_exec_paths` `[]` and `precheck_backlog_stub` true; `check_fix_unreachable` passes; `check_ceiling` accepts 4500; each spec's `corpus::ac_ids` equals the fixture's; and `render_summary` with `render_prompt` render all five from the staged fixture (nothing written, nothing hashed into the results). Items 2 and 3 render `captures: []`.
- **`fixture.json`, completed and staged** at `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21149-stage/results/s-bin/fixture.json`, sha256 `a93044cfa901e80419b330a7b2933ed7c90e870e125c68e49111867d99ab0f2c`, superseding the draft `b931e4b1…b3ca` (kept unchanged under `S8I-P7-results/`). It adds `items.5` (base `c2f948ec`, D21-148), and per item `slug`, `spec_path`, `report_path`, `ac_ids`, `build_exit` (0 on all five), `captures` with `asset_index` (`[]` on items 2 and 3), `capture_path` (`D:/sb/cap/item-<n>`, the procedure log's `tree=`; item 5's stalled session and its retry both used `D:/sb/cap/item-5`, so the one path is recorded) and `review_commit`; at the top level `leak_tokens` the four registered tokens (the draft held `LoadoutFill` only), `ceiling_secs` 4500, and `pending` removed.
- **The constructed specs and reports** are staged beside it (`results/s-bin/spec-item-<n>.md`, `report-item-<n>.md`), built and disclosed by METHOD-S's D21149-8i-B extension. D21-149 item 4's check: no AC of item 2 or item 3 names a capture.
- **Not done: the corpus commit.** The operator copies `S8I-D21149-stage/results/s-bin/` into the corpus (with `S8I-P7-results`' greenness and pin reports, still owed there) and commits it, before any rendering is hashed. The five review repositories take about 97 GB on D: until §8i.5 step 12.

### 8i.5 Spend — the batches, the caps, the environment, the version

| batch | what | model spend | gate |
| --- | --- | --- | --- |
| **B0** | driver and audit built test-first (§8i.3); dry run; seed 5's replacement selected under the registered rule (§8i.2 item 12); five stripped review trees, seeds by blob oid, fix unreachable (§8i.4); the operator's five UE runs; the amendment committed | none | operator go-ahead for the corpus writes |
| **B1** | at most **3 haiku** flag probes on a throwaway tree: the card adopted with `Read`+`Bash` under `--json-schema` (F20's measured hazard: *"an agent card's `tools` allowlist silently disables `--json-schema`"*), `apiKeySource` `none`, and the init tool set equal to the card's. **Then the five `/ue-verify` capture sessions** (§8i.4, D21-43 N19), each a `wrap` child. None of these is an S-BIN record: the probes are written to `results/s-bin/probes.jsonl`, each with its `observed_model` and `total_cost_usd`, and each session's cost goes on its procedure-log line. | ≤ 3 haiku + 5 operator sessions | **go-ahead B1**, whose line names the fixture proofs it rests on (D21-43 N22) |
| **B2** | pass 1: items 1–5 | 5 Opus | **go-ahead B2** |
| **B3** | pass 2: items 1–5, after pass 1's audit has been read | 5 Opus | **go-ahead B3** |
| retries | a void retried once at the same ceiling, inside its batch; each named in the ledger | ≤ 4 Opus in all | inside B2/B3's go-ahead |
| **B4** | only if a pass fires METHOD-S §3.4 branch 2 (*"add the field, re-run that item"*): a further dated amendment adds the field to the rendering, then that item is re-run in each pass that fired | ≤ 4 Opus | **go-ahead B4**, after its amendment |

**The caps, enforced by the driver before it spawns** (`the_dispatch_cap_refuses_before_spawning`): **18 Opus
dispatches** and **3 haiku**, counting voids, and **$90 API-equivalent** summed over every S-BIN record's
`total_cost_usd`, probes included. The capture sessions are not the driver's to count, because it does not spawn
them. The operator adds their noted cost to the running total, and the B2 line's *caps used so far* includes it. Checkpoint 8i re-sums all
three from the pinned files: the records, `probes.jsonl` and the procedure log. It compares the sum as a number
against 90.00, refuses a missing cost rather than reading it as zero, and counts the haiku from `probes.jsonl`
against 3.
The warrant: METHOD-S §3.6 sizes S-BIN at *"**10 dispatches**, ~$30 plus UE
automation wall clock"*; S-TYPED's median was *"$3.03/review"* (METHOD-S §7, 2026-09-11 item 3), and a reviewer that
builds and runs a UE suite is slower, so 18 × ~$5 bounds it. Reaching either cap halts the stage; raising one is a
ledger line, never an edit here. Throttle margins are the runner's (METHOD-S §0, *"throttle margins honoured"*).

*(dated 2026-10-02, s8i build)* **The spend table: the cap, the per-batch bounds, and the check before each batch.** The cap is
**US$90.00 total** for S-BIN. In D21-140 item 1 the operator answered **"Raise to $90 (file's cap)"**, and the entry
reads: *"The amendment's item 10 registers the driver's USD cap at 90.00, citing this entry, and the spend table's cap
column reads 90.00."* That amends D21-105 item 2's US$30 for S-BIN's dispatches only. Every other live or paid step
stays deferred. The cap above is unchanged, so this is not an edit raising a cap. The bounds are planning figures,
and their warrant is above: METHOD-S §3.6's ~$30 over 10 dispatches, S-TYPED's median of $3.03, and 18 × ~$5. The
capture sessions are unmeasured, so the operator notes each one's actual cost on its procedure-log line.

| batch | contents | per-batch USD bound | cumulative bound | cap (D21-140; D21-105 item 2's 30.00 amended) | spent so far | fits at the bounds? |
| --- | --- | --- | --- | --- | --- | --- |
| B0 | driver and audit in the private copy (D21-140 item 2), the stub dry run, the five review trees, the operator's UE greenness runs | 0.00 | 0.00 | 90.00 | 0.00 | yes: no model spend |
| B1 | ≤ 3 haiku probes (≤ 0.50 in all), then 5 capture sessions (≤ 2.00 each, ≤ 10.00 in all, unmeasured) | 10.50 | 10.50 | 90.00 | 0.00 | yes |
| B2 | pass 1: 5 Opus (≤ 5.00 each) | 25.00 | 35.50 | 90.00 | 0.00 | yes |
| B3 | pass 2: 5 Opus (≤ 5.00 each) | 25.00 | 60.50 | 90.00 | 0.00 | yes, if B1 and B2 stayed within their bounds |
| retries | ≤ 4 Opus inside B2/B3 (≤ 5.00 each) | 20.00 | 80.50 | 90.00 | 0.00 | only while spent stays within the bounds above |
| B4 | ≤ 4 Opus re-runs (≤ 5.00 each), only on METHOD-S §3.4 branch 2 | 20.00 | 100.50 | 90.00 | 0.00 | **no, at the bounds**: only if spent before it is under 70.00 |
*(dated 2026-10-05, s8i, operator sitting)* **Spent so far, from the cost records: $0.021026.** B1's probe 1 (`results/s-bin/probes.jsonl`, `observed_model` `claude-haiku-4-5-20251001`, `total_cost_usd` 0.021026, `total_cost_source` reported; launch record `20261005T042425.618581700Z-wrap.txt`, CLI 2.1.289 before and after) showed all three properties: `card_adopted` true (init tools `[Read, Bash, StructuredOutput]` = `CARD_TOOLS`), `apiKeySource` `none`, and `--json-schema` live. The probes stop at 1 of 3; haiku used 1 of 3. Capture sessions: none yet. (90.00 − 0.02) = 89.98.

*(dated 2026-10-06, s8i, operator sitting)* **B1 is complete. Spent so far, from the cost records: $5.561026** (the probe $0.021026; the procedure log's six lines, $5.54). The capture sessions, items 1 to 5 in order, each through `wrap` (CLI 2.1.289 before and after, `RC=0`), Opus 5.5 with a trace of haiku: item 1 `WBP_StashRow`, captured, `shows_subject=yes`, $0.98; item 2 `FTEST_Inventory_Stash`, a World, rejected by `capture_scene_preview`, `n/a`, $0.90 (the §8i.6 stop, D21-149); item 3 `BP_GamePlayerState` and `DA_Item_Sword`, both rejected (`Unsupported asset_type: blueprint`, `… data_asset`), `n/a`, $1.29; item 4 `WBP_LoadoutScreen`, captured, `yes`, $0.61; item 5 `WBP_GameVitals`, first session stalled (`stalled`, $0.76, D21-150), retry on a fresh copy captured, `yes`, $1.00. Captures pinned: item 1 `9ba295de…98d0`, item 4 `97e93259…a1c4`, item 5 `0de69535…f026`. B1's bound was $10.50; (90.00 − 5.56) = 84.44. The B1 sessions ran with the operator's user-scope context and MCP servers (Rider, Monolith): capture sessions are not dispatches and carry no `--strict-mcp-config`. Teardown note: item 4's editor was still exiting when its teardown ran, and wrote `Saved/Logs/OurGame.log` (8 KB) into the removed copy's path; no process survived, and the remnant was removed with `D:/sb/cap`. A teardown waits for the editor's job to report done.


**The check.** Before each batch, and before each retry at its own 5.00 bound, the orchestrator sums the spend so far
from the cost records. Those are every record's `total_cost_usd`, `probes.jsonl` and the procedure log's `cost_usd`.
A missing cost is refused, never read as zero. The batch launches only if (90.00 − spent) > that batch's bound.
**Said plainly: B3, the retries and B4 may not fit.** At the bounds, B1 to B3 and all four retries reach 80.50, and B4
does not fit. B3 and the retries fit only if B1 and B2 came in at or under their bounds. If per-dispatch cost sits
near S-TYPED's $3.03, the whole table fits (18 × 3.03 + 10.50 = 65.04). Under D21-105 item 2's original US$30, B2
alone would not have fitted (35.50), and D21-140 superseded that cap for S-BIN. The driver's own caps stand
(`the_dispatch_cap_refuses_before_spawning`): 18 Opus, which is the table's 5 + 5 + 4 + 4, plus 3 haiku and 90.00
summed. The amendment's item 10 registers the driver's USD cap at 90.00, citing D21-140. After each batch, the
*spent so far* column is updated by dated edit from the records, never estimated. The build ends at the first B1
go-ahead line, prepared in ~~D21-141 (reserved)~~ D21-143 *(dated 2026-10-03, s8i build)* D21-141 and D21-142 were taken first;
D21-143 is drafted OPEN and unsigned, and not run.

**The go-ahead line.** Before each batch the operator writes, in the ledger entry this stage opens (next free D21 id at
writing), one dated line of the fixed form `S-BIN go-ahead B<n> <YYYY-MM-DDTHH:MM:SSZ> — <scope>, <caps used so far>`. The B1
line carries one more clause, `; fixture proofs: Checkpoint 8h <hash>, dry run <sha256 of results/s-bin/dry-run.jsonl>; amendment <40-hex commit of the
pre-dispatch amendment>`,
which is D21-43 N22's *"confirmed on the B1 line"*. The ledger entry's id is written into Checkpoint 8i's `SBIN_LEDGER` line (D21-47: a
record is located by the id recorded at write time), and Checkpoint 8i reads the lines from that entry only. It
resolves the 8h hash to a commit, which must be an ancestor of HEAD and the hash `PROGRESS.md` row 8h records. It
checks that the amendment named is the commit that introduced it. And it orders the lines against the records'
own start times: B2 before pass 1's first record, and B3 after pass 1's last and before pass 2's first. D21-40
decision 2 binds every batch: *"If a real run is refused unexpectedly, the next step
is to stop and report, not to spend again."*

**The environment.** D21-42 17a: *"A committed launcher script is the only entry to a real run. It refuses
`CLAUDECODE`/`CLAUDE_CODE_*` and denylisted names, sets `CLAUDE_CONFIG_DIR` explicitly, freezes auto-update and
captures the version. The adapter refuses unless init reports `apiKeySource == "none"`."* **Every S-BIN dispatch
batch launches through `s8h`'s launcher, in its `wrap` mode.** D21-46: *"s8i uses s8h's `wrap` mode for S-BIN's
corpus runner"*. s8h §8h.7 names this stage as one of `wrap`'s two callers: *"`s8i`'s S-BIN batches through the
corpus runner."* The draft of this section said *"S-BIN launches through `s8h`'s launcher if it exposes an entry for
an arbitrary child"*, and routed through the operator's shell because the launcher then had no such entry and
refused at step 6 on the pin. s8h at `c5d19d7` answers both, so that route is withdrawn.

**What `wrap` does with the version, in s8h's words** (§8h.7, re-read 2026-09-24 in the consistency pass). Its step 6
row: *"`wrap`: record only, and never compare to the pin (below)"*. And below it, the restated guarantee: *"At step 6 it
records and does not compare."*, because *"`wrap` does **not** guarantee that the child's CLI equals the adapter pin,
because `TESTED_CLI_VERSION` binds the adapter, and the launcher does not know whether its child dispatches through
it."* S-BIN's runner dispatches through no adapter, so nothing pins it but S-BIN's own guards. Step 8's launch record
holds *"the resolved `claude` path, and its version and sha256 before and after"* and *"the rc"* (`s8h` §8h.7's `answer`
paragraph, which lists step 8's members; step 8's row names them `CLAUDE_PATH`, `CLI_VERSION_BEFORE`,
`CLI_SHA256_BEFORE`, and *"after the child, `CLI_VERSION_AFTER`, `CLI_SHA256_AFTER` and `RC` (the child's)"*). Step 9:
*"a difference fails the launch non-zero"*, naming Halt 2. (*Final consistency pass 2026-09-24:* re-quoted; the earlier
*"then version and sha256 after, and the rc"* is not in `s8h`'s bytes.) So `wrap` **records the CLI version before and
after each launch, and does not compare it to the adapter pin.** The installed 2.1.281 against the pin's 2.1.278 (§8i.0
item 8) does not refuse a `wrap` launch, and S-BIN relies on no comparison to `TESTED_CLI_VERSION`. Checkpoint 8h (10)
asserts that `--check wrap` accepts a CLI unequal to the pin. s8h also says *"`wrap` is not a way around the pin for
`run` or `probe`"*. If a later revision of s8h makes `wrap` compare to the pin, a B1–B4 launch refuses at step 6. The
stage then stops for a ledger line (§8i.6), and it does not fall back to a shell route unrecorded.

**What launches through `wrap`.** Every runner invocation that can spawn `claude` does: B1's haiku probes, B2, B3, the
retries inside them, and B4. B1's five capture sessions do too (§8i.4). Each invocation is one launch with one launch
record. B0 spends nothing and launches nothing. Its dry run spawns only the stub (§8i.3), which is the one place
`--claude-bin` is passed. Every wrapped invocation resolves `claude` from the PATH that the launcher resolved and
recorded. The launcher runs **from a plain terminal, never a Claude Code session** (s8h §8h.7: its step 2 *"refuses"* a
session's variables and *"does not strip them"*), and from `D:/repos/redkiln-rs`, so the launch record lands under this
repository's git common dir (`s8h` §8h.7 step 8 reads `<git-common-dir>` *"in the launcher's own working directory
before the child starts, so a child that changes directory does not move it"*). The child changes into the directory it
needs itself. The account is D21-42 17b's, *"`~/.claude-personal`"*, which the launcher sets at its step 4.

**S-BIN's own guards stay, as a second layer.** None of them is delegated to the launcher, and the launcher does
none of them:
- **The dispatch cap**, enforced by the driver before it spawns (`the_dispatch_cap_refuses_before_spawning`). The
  launcher counts nothing.
- **The CLI freeze.** The launcher exports `DISABLE_AUTOUPDATER=1` to its child (step 5), and it does not read the
  account's `settings.json`. The pre-check below reads that row. `wrap`'s step 9 sees a move inside one launch only.
  A move between two launches is caught by the driver's set-level
  `a_version_move_between_dispatches_halts_the_set`, which compares every record to the set's first version. Every
  record's `cli_version` must also equal the `CLI_VERSION` of its launch record. *(dated 2026-10-03, s8i build, R2FIX-8i)* Enforced in two places since s8i refuting round 2. The driver reads `$REDKILN_LAUNCH_RECORD`'s `CLI_VERSION_BEFORE` (the record has no `CLI_VERSION` key; the launcher's stdout `CLI_VERSION=` is the same reading) before each dispatch. It refuses a real dispatch with no launch record or with another version, and it records `launch_record` and `launch_cli_version` (§8i.3's dated note). Checkpoint 8i (4)'s `launch` line re-checks every record.
- **The `apiKeySource` check**, the driver's own (`api_key_source_other_than_none_voids_before_scoring`). The
  launcher reads no init line.
- **The transcript tripwire**, Checkpoint 8i (0) and (12), over both account roots.

```bash
# Before each of B1..B4, in a Git Bash OUTSIDE Claude Code (s7c §7c.15; s8h §8h.7), in a subshell so that a FAIL never
# closes the terminal. The launcher is the route, and these lines are S-BIN's second layer. Refusals, never strips.
(
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
# The runner that dispatches is committed code: every record's runner_commit names what ran (§8i.7).
# (dated 2026-10-03, s8i build, R2FIX-8i; s8i refuting round 2) runner/target/ is excluded, as runner_commit excludes
# it (§8i.3): step 3's B0 block and every launch's cargo build create it, and the corpus has no .gitignore, so the
# plain check refused at every batch. Was: git -C D:/repos/redkiln-corpus status --porcelain -- runner results/audit-s.py
st=$(git -C D:/repos/redkiln-corpus status --porcelain --untracked-files=all -- runner results/audit-s.py ':(exclude)runner/target/'); rc=$?
[ "$rc" -eq 0 ] && [ -z "$st" ] || { echo "FAIL: the corpus runner or audit is not committed and clean (rc=$rc) — a record's runner_commit would not name the code that ran"; printf '%s\n' "$st"; exit 1; }
# Disk: one dispatch copy and one UE build must fit (§8i.4). Both values are the amendment's.
test -d "${SBIN_ROOT:-}" || { echo "FAIL: SBIN_ROOT does not name the registered review root"; exit 1; }
[ "${SBIN_NEED_BYTES:-0}" -gt 0 ] 2>/dev/null || { echo "FAIL: set SBIN_NEED_BYTES to the amendment's registered figure (§8i.4)"; exit 1; }
free=$(df -B1 --output=avail "$SBIN_ROOT" | tail -1 | tr -d ' ')
[ "$free" -ge "$SBIN_NEED_BYTES" ] 2>/dev/null || { echo "FAIL: $free bytes free under $SBIN_ROOT, below the registered $SBIN_NEED_BYTES"; exit 1; }
# The launcher's own refusals, for this mode: --check spawns nothing and writes no launch record (s8h §8h.7).
out=$(bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the launcher refused wrap in this shell (rc=$rc) — fix what it names; S-BIN never routes around it"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "LAUNCH_CHECK=ok" "MODE=wrap" "DISABLE_AUTOUPDATER=1"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check wrap did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
ver=$(printf '%s\n' "$out" | sed -n 's/^CLI_VERSION=//p')
[ -n "$ver" ] || { echo "FAIL: --check wrap recorded no CLI_VERSION"; printf '%s\n' "$out"; exit 1; }
echo "S-BIN instrument at this batch: CLI_VERSION=$ver — wrap records it and does not compare it to the adapter pin; a move since the set's first record HALTS (Halt 2)"
)
```

```bash
# Then each RUNNER launch of the batch (B1's probes, B2, B3, the retries, B4), from the same terminal, in a subshell.
# The argv after `--` runs exactly as given (s8h §8h.7 step 7), and the launcher's rc is the child's. IN_DIR changes
# into the directory the child needs, so the launch record stays under this repository. SBIN_ARGV is the driver's
# subcommand and arguments, as the amendment registers them (§8i.2 item 5). No --claude-bin is passed.
(
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
L=tools/live-run.sh
IN_DIR='cd "$1" && shift && exec "$@"'
[ "${#SBIN_ARGV[@]}" -gt 0 ] || { echo "FAIL: SBIN_ARGV is empty — name the registered subcommand and its arguments"; exit 1; }
out=$(cd D:/repos/redkiln-corpus/runner && cargo build 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the runner does not build (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
bash "$L" wrap -- bash -c "$IN_DIR" in-dir D:/repos/redkiln-corpus runner/target/debug/redkiln-spike-runner "${SBIN_ARGV[@]}"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the wrapped runner exited $rc — stop and report, never spend again (D21-40 decision 2); a version move is Halt 2 (s8h §8h.7 step 9)"; exit 1; }
)
```

```bash
# B1's capture sessions (§8i.4): one interactive wrap child per CAPTURE COPY — never a base or review repository and
# never a dispatch copy. A separate block and a separate subshell from the runner's launches.
(
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
L=tools/live-run.sh
IN_DIR='cd "$1" && shift && exec "$@"'
case "${TREE:-}" in */cap/item-[1-5]) : ;; *) echo "FAIL: TREE must name a capture copy <review_root>/cap/item-<n> (§8i.4), not '${TREE:-}'"; exit 1;; esac
test -d "$TREE" || { echo "FAIL: the capture copy $TREE does not exist — make it from the base repository first"; exit 1; }
bash "$L" wrap -- bash -c "$IN_DIR" in-dir "$TREE" claude; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the wrapped capture session exited $rc — stop for a ledger line (§8i.6)"; exit 1; }
)
```

```bash
# The BrittonGames snapshot: taken before B2 and after every batch (WHEN names which), each an appended line of the
# corpus's results/s-bin/brittongames-snapshots.txt. A reviewer with Bash can reach BrittonGames, whose working tree
# holds two uncommitted modified paths; a line that differs from the first is a stop (§8i.6). Read-only on BrittonGames.
(
BG=D:/repos/BrittonGames
S=D:/repos/redkiln-corpus/results/s-bin/brittongames-snapshots.txt
case "${WHEN:-}" in before-B2|after-B2|after-B3|after-B4) : ;; *) echo "FAIL: set WHEN to before-B2, after-B2, after-B3 or after-B4"; exit 1;; esac
t=$(mktemp -d); rc=$?
[ "$rc" -eq 0 ] && [ -d "$t" ] || { echo "FAIL: no scratch directory"; exit 1; }
h=$(git -C "$BG" rev-parse HEAD); rc=$?
[ "$rc" -eq 0 ] && [ -n "$h" ] || { echo "FAIL: BrittonGames HEAD unreadable (rc=$rc)"; exit 1; }
# (dated 2026-10-03, s8i build, R1FIX-8i) --no-optional-locks: a plain status may take index.lock and rewrite the index,
# which "Read-only on BrittonGames" forbids. Its output, and so status_sha256, is the same (the build measured it so).
git -C "$BG" --no-optional-locks status --porcelain=v1 --untracked-files=all > "$t/st"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: BrittonGames status unreadable (rc=$rc)"; exit 1; }
git -C "$BG" worktree list --porcelain > "$t/wt"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$t/wt" ] || { echo "FAIL: BrittonGames worktree list unreadable (rc=$rc)"; exit 1; }
vals=""
for f in "$t/st" "$BG/OurGame/Content/Maps/Gyms/AnimationGym.umap" "$BG/OurGame/OurGame.uproject" "$t/wt"; do
  v=$(sha256sum "$f"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$v" ] || { echo "FAIL: could not hash $f"; exit 1; }
  vals="$vals ${v%% *}"
done
set -- $vals
line="when=$WHEN head=$h status_sha256=$1 umap_sha256=$2 uproject_sha256=$3 worktrees_sha256=$4"
if [ -s "$S" ]; then
  first=$(head -1 "$S" | cut -d' ' -f2-)
  [ "${line#* }" = "$first" ] || { echo "FAIL: BrittonGames moved since the first S-BIN snapshot — a stop (§8i.6)"; printf 'first: %s\nnow:   %s\n' "$first" "${line#* }"; exit 1; }
else
  [ "$WHEN" = before-B2 ] || { echo "FAIL: no snapshot file, and this is not the before-B2 snapshot"; exit 1; }
fi
printf '%s\n' "$line" >> "$S" || { echo "FAIL: could not append the snapshot"; exit 1; }
echo "BrittonGames snapshot $WHEN recorded"
)
```

**Stated, but only partly measured, and a stop rather than a fallback.** `s8h` §8h.7 now states that `wrap` runs its
child *"in the foreground with the launcher's own stdin, stdout and stderr, never through a pipe, a `$(…)` capture or a
redirect"*, and Checkpoint 8h (10) asserts that stdin reaches a `wrap` child. That a TTY reaches it is not asserted
there, because a checkpoint captures its own output, so the first capture session is that reading. If it cannot be
driven under `wrap`, the stage stops for a ledger line (§8i.6). It is not re-routed through a bare shell. (*Final
consistency pass 2026-09-24:* the earlier text said `s8h` did not say.)

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

*(dated 2026-10-02, s8i build)* **Halt 2 at stage open** (§8i.0 item 8's dated note): the pin file reads 2.1.278, the 2026-09-24 reading
was 2.1.281, and the CLI now reads 2.1.288. The account row is absent: `~/.claude-personal/settings.json` carries no
`DISABLE_AUTOUPDATER`. Writing it is the operator's, as a **deferred gate before B1**, and the pre-check above
refuses without it. The set's version is the one the CLI reads once the row is written. The B1 pre-check's
`CLI_VERSION` records it. Halt 2 also goes on the stage's `PROGRESS.md` row, at the tracker edit (owed, not made here).

*(dated 2026-10-05, s8i, operator sitting)* **Packet step 5 done: the CLI is frozen at 2.1.289.** The operator added `"env": { "DISABLE_AUTOUPDATER": "1" }` to `~/.claude-personal/settings.json` (its ten top-level keys now include `env`; the other nine are unchanged). A plain-terminal `claude --version` read `2.1.289 (Claude Code)` both just before and just after the row was written: the CLI had moved again since stage open (2.1.278 pin, 2.1.281 on 2026-09-24, 2.1.288 at stage open). **2.1.289 is the set's version** (D21-43 N22), recorded under Halt 2; B1's pre-check `CLI_VERSION` and every launch record must read it, and `s8j` pins it. The B0 batch is complete: the corpus commits `40c0332` (the driver and the audit, byte-identical to the private copy) and `f94a196` (the dry run and `driver-tests.txt`), the B0 block's 33-test reading, and the dry run's sha256 `d730d14f11cfed7a68bcd8edba17efccf5faa62850b99f4dcf134b24faf4ed5a`. Spent so far: $0.00.

*(dated 2026-10-03, s8i build)* **The operator packet, from B0's corpus write to the snapshot before B2 (package S8I-P8).** All of
it is prepared and none of it has been run. Every step is the operator's, from a plain Git Bash outside Claude Code,
each in a subshell. The cap is US$90.00 (D21-140 item 1). Before each batch and each retry, the orchestrator checks
**(90.00 − spent) > the batch's bound**. The package brief's *"(30.00 − spent)"* is D21-105 item 2's figure from before
D21-140 amended it for S-BIN, and is not used. The go-ahead lines go into **D21-143** and are signed in place (its
*How to sign* paragraph).

**Owed before the steps below.** None of these is this package's work.
- **(a)** The driver's LFS-pointer read is fixed, and the 25 tests are re-read (§8i.3's dated note). Until then the
  driver patch is P5's `S8I-P5-sbin-driver.patch`, and the dry run cannot render.
- **(b)** METHOD-S extension 1 is appended: seed 5's R = `6a101bdcd` and D21-142's second strip rule. Then base item 5
  is built.
- **(c)** The five greenness runs are made (§8i.4). After them, extensions 2, 3 and 4 are appended: `ceiling_secs`,
  `SBIN_NEED_BYTES` and the greenness results. B1's pre-check reads `SBIN_NEED_BYTES`, so extension 3 comes before
  B1.
- **(d)** The operator accepts or rejects P7's pin-report source (§8i.4's dated note).
  *(dated 2026-10-04, s8i, operator sitting)* **(c) and (d) are done.** The five greenness runs are green (METHOD-S's extension of 2026-10-04 for deferred extensions 2, 3 and 4 is appended: `ceiling_secs` = 4500, `SBIN_NEED_BYTES` = 34221223118), and the operator ACCEPTED the pin-report source, byte-identical at `c3dae802…6bc6`.

*(dated 2026-10-03, s8i build)* **(a) and (b) are done (package S8I-F1).** (a): the driver reads the committed pointer, and the
superseding patch is `S8I-F1-sbin-driver.patch` (sha256 `dd3ee74ec8dd2752cdffb42ffb4fb3491f32de3543bcc97003f029ca23414f3a`, §8i.3's
dated note), so step 2's `DRV` names it. (b): METHOD-S's extension of 2026-10-03 is appended (seed 5's R, D21-142's
rule, the five bases, and the measured half of extensions 3 and 4), and base item 5 is built (§8i.4's dated note).
(c) and (d) remain the operator's. The dry run, step 4, is still the operator's after B0.

*(dated 2026-10-04, s8i build, D21148-8i-B)* **(c), item 5 (D21-148).** Items 1 to 4's runs are staged green on the D21-147 bases (892, 920, 997 and 892 tests, 0 failed, exit 0). Item 5's run on `e0620bfa` is superseded, an instrument defect and not a red seed, so item 12's replacement does not apply. Its one binding run is on base `c2f948ec`, once the operator swaps it into `D:/sb/base/item-5` (§8i.4's D21148-8i-A note). Extensions 2, 3 and 4 still follow the five runs.

*(dated 2026-10-03, s8i build, R1FIX-8i)* **The driver patch is superseded again, one owed item is added, and one fact is made visible
(s8i refuting round 1).** `DRV` is now `S8I-R1FIX-8i-sbin-driver.patch` (sha256
`1922bb473b14a2bd57914e0ce7092e95accd3766eaca39ae80ab214192c12b08`, §8i.3's dated note), and step 2's check names it.
- **(e)**, owed before any rendering is hashed, because no step named it:
  1. the constructed `spec.md` and implementation report for each of the five items, as the 2026-09-09 entry registers
     them (item 5's carries R's clause, METHOD-S extension of 2026-10-03), at the corpus paths `fixture.json`'s
     `spec_path` and `report_path` name;
  2. each item's review repository `D:/sb/rev/item-<n>`, built after B1 by `commit_single` with the base tree's
     tracked set together with `_s-bin/captures/capture-<k>.png` and `_s-bin/report/{index.json,headless.log}` (§8i.4's
     table), its commit written to `review_commit`;
  3. `fixture.json` completed: `items.5`; per item `slug`, `spec_path`, `report_path`, `ac_ids`, `build_exit`,
     `captures`, `capture_path` and `review_commit`; and at the top level `ceiling_secs` from extension 2, which a real
     pass now refuses without.

  The driver refuses before any spawn while any of these is missing, so nothing is spent on the gap, but nothing
  tracked it.
  *(dated 2026-10-06, s8i build, D21149-8i-B)* **(e) is built, proposed, nothing in the corpus** (§8i.4's D21149-8i-B note): the five spec/report pairs and `fixture.json` (sha256 `a93044cf…`) are staged under `S8I-D21149-stage/`, and the five review repositories are built by `commit_single` and verified, with the driver's own pre-spawn checks reading clean. The operator's corpus commit of the staged files is owed before any rendering is hashed.
- **A fact before B1, not a decision:** base items 1 and 4 hold the same tree (§8i.4's dated note), so clean item 1's
  tree carries seed 4's defect. Under the registered rule, a block on item 1 naming `WBP_LoadoutScreen.uasset` is
  scored as a false positive.
- **Step 11** returns the corpus to the pin before the formal Checkpoint 8i, because that block's replay of
  Checkpoint 5 needs it there (§8i.9).

*(dated 2026-10-03, s8i build, R2FIX-8i)* **`DRV` is superseded again, and one owed item is added (s8i refuting round 2).** `DRV` is now
`S8I-R2FIX-8i-sbin-driver.patch` (sha256 `815111521f351e61b8ea884124eb26ef34321c9767dd48792311f9cf4a8868c1`, §8i.3's dated
note), and step 2's check names it. The dry-run script's sha256 is now `15990996…a9b7f3` (step 4).
- **(f)**, owed before B2: METHOD-S item 4's per-copy free-space check, *"Before each copy, the free space on the
  root's volume must be at least `SBIN_NEED_BYTES`"*, is not in the driver (§8i.3, *"Not built here"*). Either
  it is built, or an appended METHOD-S extension registers that the batch pre-check above carries it. That choice
  narrows or keeps a registered check, so it is the operator's (METHOD-S's R2FIX-8i extension, item 5).
  *(dated 2026-10-06, s8i build, D21149-8i-A)* **(f) is ruled and built.** D21-151: the driver checks the free space
  before every dispatch copy (`a_volume_under_sbin_need_bytes_refuses_before_the_copy`, §8i.3's D21149-8i-A note), and
  the batch pre-check stays as a second layer. It lands in the corpus with the D21149-8i runner commit (the packet
  below, step 2a), before B2.

*(dated 2026-10-03, s8i build, R3FIX-8i)* **`DRV` is superseded again, B4's argv is written, and the close removes the review root
(s8i refuting round 3).**
- `DRV` is now `S8I-R3FIX-8i-sbin-driver.patch` (sha256 `0bc288756c57010517c32d4f71169db4c8e601411261be74f6a8e315f58ea7dc`, §8i.3's dated
  note), and step 2's check names it.
- **B4's argv.** A B4 re-run names its one item, and the driver refuses one that does not (§8i.3's dated note). It is
  the pass's argv with three flags added, run once per (pass, item) that fired branch 2 through the runner launch
  block above: `SBIN_ARGV=(sbin-probe --pass <p> --fixture results/s-bin/fixture.json --out results/s-bin.jsonl
  --model opus --effort xhigh --timeout-secs <ceiling> --arm b-typed-sbin-plus-<field> --rerun --only-item <n>)`. The
  field and its content stay with B4's amendment (deferred extension 5).
- **Step 12 removes the review root at the close.** Checkpoint 8i (12) ends with `test ! -e "$SBIN_ROOT"`, and its
  header reads *"no review tree left registered in BrittonGames or left on disk (Halt 6)"*. No step removed `D:/sb`.
  So a packet followed to the letter reached (12) with `D:/sb/base` and `D:/sb/rev` still in place, and (12) failed
  (round 3, refuted to minor). The faithful route is a close step, not a narrower guard, for these reasons:
  - Nothing in Checkpoint 8i reads a tree under the root [measured on this file]. Its only uses of `rev/item` are the
    synthetic `ctl` line and the `test ! -e`, and `treecommit` compares the records with `fixture.json`.
  - What the finding is re-derived from is committed in the corpus (§8i.7): renderings, raw transcripts, captures,
    greenness reports and `review_commit`.
  - *"Frozen"* in METHOD-S item 4's table means unchanged while the batches run, and B4 comes before the close.

  So step 12 removes `D:/sb` after the last results commit and the last snapshot, and before the formal Checkpoint 8i.
  What it gives up is the trees themselves: after the close, no dispatch can be re-run from them. Step 12 is never run
  before the close.

*(dated 2026-10-03, s8i build, R4FIX-8i)* **`DRV` is superseded again (s8i refuting round 4).** `DRV` is now
`S8I-R4FIX-8i-sbin-driver.patch` (sha256 `934ad2c15c94f5443c0d45aa880851d73f2d6e7b511d9798a14f2b114ce847b4`, §8i.3's dated note), and step 2's check names it.

*(dated 2026-10-03, s8i build, R5FIX-8i)* **`DRV` and `AUD` are superseded (s8i refuting round 5, D21-144).** `DRV` is
now `S8I-R5FIX-8i-sbin-driver.patch` (sha256 `3357c3fb79cd7b0324b8ab40bfaf474be64e4140fd12817754f8633fe0e26bb0`) and
`AUD` is `S8I-R5FIX-8i-audit-bin.patch` (sha256 `e0243d5d4edeb9a7b538889e346d488d6e60c61a2f036c31ea0ccf7cb4eb4d32`),
§8i.3's dated note; step 2 checks both.
- **(g)**, a deferred gate before B1: the operator confirms item 4's reading as METHOD-S's R4FIX-8i and R5FIX-8i
  extensions write it *(dated 2026-10-03, s8i build, R6FIX-8i: and R6FIX-8i's)*, ~~the provisional task-output exemption~~ *(dated 2026-10-03, s8i build, R6FIX-8i: the task-output exemption
  as D21-145 rules it, with the two questions it leaves: a pre-spawn check for `<temp>/claude/<copy slug>/`, and
  whether the shadow allowlist carries it)* and the unregistered incidental hits among it, and clears
  or voids every shadow-flagged replay or record. For a record, the ruling is one line in
  `results/s-bin/oot-shadow-rulings.jsonl`, `{"raw_ref": …, "ruling": "clear" | "void"}`, committed with that batch's
  results before the batch is scored. An operator void is `void: out-of-tree`, gets no retry during the AFK run, and
  does not count toward §8i.6's second-void stop.
- **(h)**, operator housekeeping before B1, because it deletes or moves directories: move `D:/r6c`, the
  `D:/repos/redkiln-sev` copies and `~/.claude-backup` off reachable paths, and clear `%TEMP%/claude`'s task outputs
  and the temp files the sweep named (`tmp.uDZZzvzEF4`, `h.md`, `l143`, `ms0`). This shrinks both the leak surface and
  the false-void surface. The registered roots stay registered either way.
  *(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* **(h) is done by deletion**, except `D:/repos/redkiln-sev`'s tooling, which goes after step 4 and before the B1 line (D21-144's dated line of 2026-10-03). **(g)** is answered: D21-144 A is confirmed with the replay flags cleared by class, and D21-145 is confirmed with an operator absence check before each batch (`ls %TEMP%/claude | grep -i '^D--sb-'` prints nothing) and own-session triage (their dated lines). Also cleared before B1: `%TEMP%/sbin-oot-wire-*`, residue of the mutant harness's FLIPPED-ABORT row R6-E25 (a stack overflow skips `Scratch`'s Drop). It holds synthetic rig files and the public stub, no leak token.

*(dated 2026-10-03, s8i build, R6FIX-8i)* **`DRV` is superseded again (s8i refuting round 6).** `DRV` is now
`S8I-R6FIX-8i-sbin-driver.patch` (sha256 `10b0e266fe9015a687a97e55991a30c5d0796300c373cd682c78087da3060fae`, §8i.3's dated note); `AUD` is
unchanged, `S8I-R5FIX-8i-audit-bin.patch`. Step 2 checks both. Step 4's script is re-pinned (sha256
`5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c`): it required the 25-test reading, and step 3's B0 block writes 33, so it refused every
dry run.

*(dated 2026-10-03, s8i build, R7FIX-8i)* **`DRV` is superseded again (s8i refuting round 7).** `DRV` is now
`S8I-R7FIX-8i-sbin-driver.patch` (sha256 `5b84a5392ac5127427253057b634ed70f620d03e52f0fca00ee116d883d700ed`, §8i.3's dated note); `AUD` is
unchanged, `S8I-R5FIX-8i-audit-bin.patch`. Step 2 checks both. Step 4's script is unchanged (sha256
`5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c`): the selection stays at 33 tests. D21-143 names the same pair in a dated paragraph,
so the B0 line's *"from the private-copy patches"* means them; the B1 line names no patch.

*(dated 2026-10-03, s8i build, R8-RECORD-8i)* **A decision owed before B1, recorded and not fixed (s8i refuting round 8, the last under the AFK
safety cap of 8).** No patch moves: `DRV`, `AUD` and step 4's script are those the R7FIX-8i paragraph above names.
- **(i)**, a **decision owed before B1**: every base tree ships an editor launcher that opens the answer-bearing
  project, and neither the runtime out-of-tree void nor the shadow audit sees it run.
  - *The residual.* `_workspace/tooling/RunEditor_OurGame.bat` is in all five bases, `D:/sb/base/item-1..5` (1,432
    bytes in items 1 to 4, 774 in item 5), and its line 13 in each reads `set PROJECT=D:\repos\BrittonGames\OurGame\OurGame.uproject`,
    followed by `start "" "%UE%" "%PROJECT%" -noensuredialog -NoZenAutoLaunch` [measured 2026-10-03, read-only on
    `D:/sb/base`]. The tree documents it as the way to launch the editor. `AGENTS.md` has *"[RunEditor_OurGame.bat](_workspace/tooling/RunEditor_OurGame.bat)
    | Launch the editor with `-noensuredialog`"*, and `.claude/skills/unreal-editor-mcp/SKILL.md` has *"Launch via
    `_workspace/tooling/RunEditor_OurGame.bat` (`-noensuredialog`)"*. The launcher is not in §8i.2 item 3's registered
    strip set, and the leak-token precheck scans registered content tokens, not paths. The runtime void reads
    read-position words, and `./_workspace/tooling/RunEditor_OurGame.bat` resolves inside the copy. The shadow audit
    reads a script's text only when the session staged it (METHOD-S's R6FIX-8i extension, item 3). Round 8's finder
    measured the run from inside the test binary, in a private copy (corpus `285e011` with both patches, under the
    tree `D:/sb/d/p1i4a1`, removed afterwards): `VOID=false hits=[]  FLAG=false []`. This package did not re-run that
    measurement. One refuter reached the same reading from the audit's code without re-running it. BrittonGames'
    working tree holds every item's fix, because the items are past commits.
  - *The class, measured.* Counting files under `_workspace/tooling/` (`*.py`, `*.bat`, `*.cmd`, `*.ps1`, `*.sh`) that
    name an absolute `D:/repos/BrittonGames` path gives 14 in each of items 1 to 4 and 3 in item 5 [measured 2026-10-03,
    read-only grep]. Items 1 to 4 hold the launcher, the eight `author_*.py`, and `synty/calibrate_deformation.py`,
    `synty/ci_validate.py`, `synty/diagnostics/diag_conform_axis.py`, `synty/diagnostics/proto_orient_bones.py` and
    `synty/strip_character_sources.py`. Item 5 holds the launcher, `synty/ci_validate.py` and
    `synty/strip_character_sources.py`. In every `.py` the path sits in the module docstring's usage text, as
    `-script="D:/repos/BrittonGames/..."` or, under *USAGE*, `sys.path.insert(0, r"D:/repos/BrittonGames/...")`. So
    running the script does not read that path, and a reviewer who types it voids by name. The launcher is the one
    file whose code runs the path.
  - *Mitigations, partial.*
    - MCP is isolated: the card's tools are `Read` and `Bash`, under `--strict-mcp-config` (§8i.2 item 5). The tree's
      Monolith server is therefore unavailable, and reaching the editor needs JSON-RPC written by hand to
      `localhost:9316`. The tree's `CLAUDE.md` says *"**Port 9316 has no auth.**"* and *"`editor_query("run_python", ...)`
      is arbitrary editor Python"*.
    - Halt 6 fires if the editor survives: *"A UE process survives a dispatch (Halt 6)"* (§8i.6). The tasklist that
      shows it comes after the record is written, though, and the tree ships `ue_editor_quit.py` for closing the
      editor.
    - The BrittonGames snapshot is a `git status`. BrittonGames' `.gitignore` ignores `**/Saved/*`,
      `**/Intermediate/*` and `**/DerivedDataCache/*`, so an editor session there may leave the snapshot unchanged.
    - The vote on severity split. One refuter called it minor: a chain of steps, with Halt 6 firing in the likely
      case. The other upheld it as major, because it is the tree's own documented route.
  - *The operator's options.*
    - **(a)** Neutralise the launcher and the absolute-path tooling scripts identically in all five trees. Either add
      them to §8i.2 item 3's registered strip set by an appended METHOD-S extension, or rewrite `PROJECT` relative
      (`%~dp0..\..\OurGame\OurGame.uproject`) and register that. Then rebuild the five bases and re-run the leak and
      blob checks. The capture copies are copied from the bases (step 9), so this also takes the launcher out of the
      capture sessions.
    - **(b)** Add a precheck that refuses a copy holding an absolute answer-bearing path in an executable file, with
      a named test showing that such a copy refuses before spawning.
    - **(c)** Accept the residual as disclosed (METHOD-S's R8-RECORD-8i extension, item 1), relying on MCP isolation,
      Halt 6 and the shadow-hold review.
  The choice either narrows or keeps a registered check, or rebuilds the frozen fixture, so it is the operator's. The
  AFK build does not take it. (c) is the most reversible: it changes no tree and no code.

*(dated 2026-10-03, s8i build, D21146-8i)* **Owed item (i) is resolved by D21-146.** The operator took (a) and (b) together on 2026-10-03 (Weigh-In wi-938b68, *"Neutralise in all 5"*). The launcher is removed from all five trees by METHOD-S's D21146-8i extension, item 1, and the five bases are rebuilt (§8i.4's dated note). The driver refuses a dispatch copy whose executable file names an answer-bearing path (`precheck_exec_paths`, §8i.3's dated note). The capture copies are made from the rebuilt bases (step 9), so the capture sessions no longer hold the launcher either.

*(dated 2026-10-03, s8i build, D21146-8i)* **`DRV` is superseded again (D21-146).** `DRV` is now `S8I-D21146-8i-sbin-driver.patch` (sha256 `a24c896f7cdd49cdef72a10053a2827b461d232bbfbe366bbaf4cf6cf13afc0d`, §8i.3's dated note); `AUD` is unchanged, `S8I-R5FIX-8i-audit-bin.patch`. Step 2 checks both. Step 4's script is unchanged (sha256 `5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c`): the selection stays at 33 tests, and the record keeps its 34 names. Step 4's inputs move: the dry run's review repository is rebuilt from the rebuilt base item 1 (commit `48cd028e2b83b65e12d456c4ca2e2a258cbed593`), and its fixture hashes `8f98bf430ddf6f461217eab9f89305b6d279bdec011f5282aac691e67f728ea1` (§8i.4's dated note). D21-143 names the same pair in a dated paragraph.

*(dated 2026-10-03, s8i build, D21147-8i)* **`DRV` is superseded again (D21-147).** The operator ruled on 2026-10-03 (D21-147 item 2): *"A new driver patch supersedes `S8I-D21146-8i-sbin-driver.patch`."* `DRV` is now `S8I-D21147-8i-sbin-driver.patch` (sha256 `0bf69ab4a1a1e8a396f278c63d1cd4fb943ac64ffc1898392a3c6e8cce54da3c`, §8i.3's D21147-8i note); `AUD` is unchanged, `S8I-R5FIX-8i-audit-bin.patch` (sha256 `e0243d5d…4d32`). Step 2 checks both. Step 4's script is unchanged (sha256 `5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c`): the new cases sit in two existing named tests, so the selection stays at 33 tests and the record keeps its 34 names, and neither §8i.3's B0 block nor Checkpoint 8i's counts move. Step 4's inputs move again, under D21-147 item 3 (*"the dry run's review repository is rebuilt from the new base item 1, and its commit and fixture hash move"*): `D:/sb/dry/rev/item-1` is now commit `a8d9478b54556d6617963cab622b385109928ab3`, superseding the D21146-8i note's `48cd028e…`, and the dry run's fixture hashes `62f6469c11e3bc5e656b565ba28c79709ad30526b954d88017ebd69024b6a8b1`, superseding `8f98bf43…ea1` (§8i.4's D21147-8i note). The operator's step-1 acceptance (D21-143's sitting paragraph) was of `48cd028e…`; the operator has not rebuilt or confirmed the new repository. Step 7's largest base moves with the bases (its dated comment). D21-143 names the new pair in a dated paragraph.

*(dated 2026-10-06, s8i build, D21149-8i-A)* **D21-149 item 3's sequence before B2: a new runner commit, the B0 block again, a new zero-spend dry run, and a re-signed B1 line (prepared, NOT run).** D21-149 item 3 reads *"That is a new runner commit, a new zero-spend dry run, and a re-signed B1 line naming it, all before B2."* The patch is `S8I-D21149-8i-sbin-driver.patch` (sha256 `a904b4c0…e637`, §8i.3's D21149-8i-A note), against corpus `main` at `cc7f222`. Steps 2a to 6a below follow it, from a plain Git Bash outside Claude Code, each in a subshell. They move these:
- **The dry-run script** is `7c95fb14df3d3ddd14133cd44eaa3bd073da1b61861d58cec0f3f708c5c8bf08`, superseding the operator's `c25b1387…a007`. It requires the 34-test reading and item 1's real capture, with `synthetic_capture` false. Its old bytes are kept as `S8I-P8-dryrun-before-D21149.sh`. Its record check was run over three records (`D21149-8i-A-dryctl.out`): it reads B0's `d730d14f` record (synthetic) BAD, the D21-149 shape ok, and the same shape without `asset_index` BAD.
- **The dry run's review repository** is rebuilt at `D:/sb/dry/rev/item-1`. It is base item 1 (`58e98963`), the same two `_s-bin/report/` stand-ins, and item 1's real B1 capture at `_s-bin/captures/capture-1.png`. The capture is read from the corpus object store at `cc7f222` (sha256 `9ba295de…98d0`, committed as the tree's LFS pointer, with its LFS object present). The repository is commit `cc1b4b4526cc4aece63165c9a13eec893767205b` (tree `44931f6ba29a325db7a69569357dd6a34b6bec83`), built by `s8i_D21149-8i-A_review.py` (sha256 `db9340d2…dd1c`) with its dates pinned at `1791244800 +0000`, so a rebuild reproduces it. It reads `VERIFY OK` (`D21149-8i-A-review-verify.out`). It supersedes `a8d9478b…`, which B0 removed. The operator has not rebuilt or confirmed it.
- **The dry run's fixture** (both copies under `S8I-P8-dryrun/`) is sha256 `6d93cb9e8897579ff2e87316bd0f80794680382a128083f50695e0c59dd1caf6`, superseding `62f6469c…b8b1`. Only `items.1.review_commit`, `items.1.captures` (`[{"asset_index": 1, "path": "_s-bin/captures/capture-1.png", "sha256": "9ba295de…98d0"}]`) and its stand-in note moved. The old bytes are kept as `S8I-P8-dryrun-fixture-before-D21149.json`.
- **The dry run's copy path.** B0's dry-run record is moved under `results/s-bin/superseded/dry-run-d730d14f/`, so the re-run's copy is `D:/sb/dry/d/p1i1a1` again. That is the path B0's deleted dry-run copy used, under the dry run's own root and never a set path. Checkpoint 8i's `fresh` line reads the set's records and the capture copies, not the dry run's.

```bash
# (dated 2026-10-06, s8i build, D21149-8i-A) PREPARED, NOT RUN. D21-149 item 3 and D21-151, before B2.
# 2a. The D21149-8i runner commit, from the private-copy patch against corpus main cc7f222.
( C=D:/repos/redkiln-corpus; S=D:/repos/redkiln-sev/afk-tools/sp/s8i; P=$S/S8I-D21149-8i-sbin-driver.patch
  [ "$(git -C "$C" rev-parse HEAD)" = cc7f2227459a1aed83629eecdb254587811f4495 ] || { echo "FAIL: corpus main is not cc7f222"; exit 1; }
  [ -z "$(git -C "$C" status --porcelain --untracked-files=all -- . ':(exclude)runner/target/')" ] || { echo "FAIL: the corpus is not clean"; exit 1; }
  [ "$(sha256sum < "$P" | cut -c1-64)" = a904b4c02fa5d0d0b89fe962e7c240d3c9d1ff0b03a272d6f824792a15bae637 ] || { echo "FAIL: $P is not D21149-8i-A's patch"; exit 1; }
  git -C "$C" -c core.autocrlf=false apply --check "$P" || { echo "FAIL: the patch does not apply"; exit 1; }
  git -C "$C" -c core.autocrlf=false apply "$P" || exit 1
  [ "$(sha256sum < "$C/runner/src/sbin.rs" | cut -c1-64)" = bcdf0364f5d2927ae48a940a0236abe1ee0ee022cceb1331d74415d448436bc1 ] || { echo "FAIL: sbin.rs is not the built bytes"; exit 1; }
  git -C "$C" add runner/src/sbin.rs || exit 1
  git -C "$C" commit -m "s8i: the S-BIN driver for D21-149 (captures by asset index, an empty list rendered, no synthetic capture) and D21-151 (free space before every dispatch copy)" || exit 1
  # (dated 2026-10-06, s8i, operator sitting) runner/target/ is excluded, as every other runner-clean check here excludes
  # it: the operator's run committed 7cc8131 cleanly and then read this FAIL on `?? runner/target/` alone.
  [ -z "$(git -C "$C" status --porcelain -- runner ':(exclude)runner/target/')" ] || { echo "FAIL: runner/ not clean after the commit"; exit 1; } )
# 3a. Section 8i.3's B0 block, as written: 34 tests. It rewrites results/s-bin/driver-tests.txt.
# 4a. B0's dry-run results moved aside, the empty leftover D:/sb/dry/d removed, the rebuilt review repository read,
#     then the dry run.
( C=D:/repos/redkiln-corpus; S=D:/repos/redkiln-sev/afk-tools/sp/s8i; O=results/s-bin/superseded/dry-run-d730d14f
  [ "$(git -C "$C" cat-file blob HEAD:results/s-bin/dry-run.jsonl | sha256sum | cut -c1-64)" = d730d14f11cfed7a68bcd8edba17efccf5faa62850b99f4dcf134b24faf4ed5a ] || { echo "FAIL: the committed dry run is not d730d14f"; exit 1; }
  mkdir -p "$C/$O/raw" "$C/$O/rendered" || exit 1
  git -C "$C" mv results/s-bin/dry-run.jsonl "$O/dry-run.jsonl" || exit 1
  git -C "$C" mv results/s-bin/dry-run "$O/dry-run" || exit 1
  git -C "$C" mv results/s-bin/raw/dry-run "$O/raw/dry-run" || exit 1
  git -C "$C" mv results/s-bin/rendered/dry-run "$O/rendered/dry-run" || exit 1
  if [ -d D:/sb/dry/d ]; then rmdir D:/sb/dry/d || { echo "FAIL: D:/sb/dry/d is not empty"; exit 1; }; fi
  python "$S/s8i_D21149-8i-A_review.py" verify | tail -1 | grep -qx 'VERIFY OK' || { echo "FAIL: D:/sb/dry/rev/item-1 is not the rebuilt review repository"; exit 1; }
  [ "$(git -C D:/sb/dry/rev/item-1 rev-parse HEAD)" = cc1b4b4526cc4aece63165c9a13eec893767205b ] || { echo "FAIL: the dry review repository moved"; exit 1; }
  [ "$(sha256sum < "$S/s8i_S8I-P8_dryrun.sh" | cut -c1-64)" = 7c95fb14df3d3ddd14133cd44eaa3bd073da1b61861d58cec0f3f708c5c8bf08 ] || { echo "FAIL: the dry-run script is not D21149-8i-A's"; exit 1; }
  [ "$(sha256sum < "$S/S8I-P8-dryrun/results/s-bin/fixture.json" | cut -c1-64)" = 6d93cb9e8897579ff2e87316bd0f80794680382a128083f50695e0c59dd1caf6 ] || { echo "FAIL: the dry-run fixture is not D21149-8i-A's"; exit 1; } )
bash D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P8_dryrun.sh
#    Then commit its results with driver-tests.txt and the superseded B0 dry run, and remove the 21.5 GB review repository:
( C=D:/repos/redkiln-corpus
  git -C "$C" add results/s-bin/superseded results/s-bin/dry-run.jsonl results/s-bin/driver-tests.txt results/s-bin/dry-run results/s-bin/raw/dry-run results/s-bin/rendered/dry-run || exit 1
  git -C "$C" commit -m "s8i: S-BIN's zero-spend dry run again over the D21149-8i runner (D21-149 item 3), B0's d730d14f kept as superseded" || exit 1
  python D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_D21149-8i-A_review.py remove )
# 6a. Re-sign D21-143's B1 line in place, naming the new dry run's sha256 from step 4a (D21-149 item 3).
```

*(dated 2026-10-06, s8i build, D21149-8i-D)* **Before B2: what is done, and what is owed in order (package D21149-8i-D, recording D21149-8i-A, -B and -C).** Everything here is zero spend except B2. It was read on 2026-10-06 with corpus `main` at `cc7f222` and clean.
- **Done.** B0 (`40c0332`, then `f94a196`). B1 is complete at $5.561026 (corpus `cc7f222`). The CLI is frozen at 2.1.289. The five bases are `58e98963`, `e8954b61`, `efcc2429`, `2878d79d` and `c2f948ec`. Their greenness runs are green and staged. `ceiling_secs` 4500 and `SBIN_NEED_BYTES` 34,221,223,118 are registered (METHOD-S's extension of 2026-10-04).
- **Built, proposed, not yet in the corpus.**
  - D21149-8i-A: the runner patch `a904b4c0…e637`, the dry-run script `7c95fb14…bf08`, its fixture `6d93cb9e…caf6` and its review repository `cc1b4b45…205b`.
  - D21149-8i-B, owed item (e): `fixture.json` `a93044cf…f2c`, the five spec/report pairs, and the five review repositories (§8i.4's D21149-8i-B table). Each `D:/sb/rev/item-<n>` HEAD was re-read read-only on 2026-10-06 and equals `fixture.json`'s `review_commit`.
  - D21149-8i-C: D21-141's (B) in Checkpoint 8i (8).
- **The operator's steps, in this order:**
  1. step 2a, the D21149-8i runner commit;
  2. step 3a, the B0 block, which is the binding 34-test reading;
  3. step 4a, the dry run and its results commit, then the dry run's review repository removed (about 21.5 GB);
  4. step 6a, D21-143's B1 line re-signed in place with the new dry run's sha256 (D21-149 item 3);
  5. ~~reading the five constructed pairs and accepting them or having them rebuilt (METHOD-S's D21149-8i-B extension: the seeded items' reports carry the commit message's own diagnosis), then~~ *(dated 2026-10-06, s8i build, R10FIX-8i: D21-152)* the reading is done: the operator accepts the five pairs exactly as built (D21-152 (K), METHOD-S's D21-152 extension), so none is rebuilt and none is rebuilt, and between steps 6a and 6b nothing more is owed *(dated 2026-10-06, R11FIX-8i: was "nothing is owed before step 6b but step 6a"; steps 2a, 3a and 4a are owed too)*; then step 6b below, owed item (e)'s corpus commit;
  6. step 10, the BrittonGames snapshot before B2;
  7. steps 10a and 10b below: the B2 line, the pre-check and pass 1.
- **Step 6b was run only against synthetic corpora** (`s8i_D21149-8i-D_test6b.py`, `D21149-8i-D-test6b.out`). On a repository in step 4a's state it commits 22 paths. It refuses a second run, a repository without step 4a's superseded dry run, an untracked file, and a staged file with one byte changed; in that last case it commits nothing.
- **Owed, but not before B2:** the refuting round scoped to Checkpoint 8i (8) (D21-141's (B)), before the formal Checkpoint 8i; steps 11 and 12 at the close.
- **Disk.** D: read 129,415,987,200 bytes free on 2026-10-06, with the five review repositories (about 97 GB) and the dry run's (about 21.5 GB) in place. Each dispatch copy is checked against `SBIN_NEED_BYTES` before it is made (D21-151) and removed after its dispatch (`dispatch_one`).
- **Spend.** $5.561026 after B1. B2's bound is 25.00, and (90.00 − 5.56) = 84.44.

*(dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's minor)* **D21-152 recorded: the constructed-pair reading is closed.** The operator accepts the five constructed spec/report pairs exactly as built (D21-152 (K), 2026-10-06, before B2 and before any rendering is hashed; report sha256 `9ae0c522…`, `a916fa90…`, `00696b45…`, `f0a9f60f…`, `8848e41c…`, the specs as METHOD-S's D21149-8i-B extension tables them). No report byte moves, so step 5 of the list above and step 6b's precondition are met. F43's contract gains D21-152 item 2's disclosure and item 3's recorded-only evidence-source tag (§8i.7's dated note). METHOD-S's D21-152 extension registers both. Owed before B2 is unchanged otherwise: steps 2a, 3a, 4a, 6a, 6b, 10, 10a and 10b.

```bash
# (dated 2026-10-06, s8i build, D21149-8i-D) PREPARED, NOT RUN. Owed item (e) into the corpus, then B2.
# 6b. After step 6a, never before step 4a: the dry-run script refuses while results/s-bin/fixture.json exists.
#     Only once the operator has read and accepted the five constructed spec/report pairs (METHOD-S's D21149-8i-B
#     extension, proposed). (dated 2026-10-06, s8i build, R10FIX-8i) Met: D21-152 accepts them as registered (K). It commits D21149-8i-B's staged fixture.json and pairs, S8I-P7's five greenness reports
#     (index.json and headless.log, §8i.4 step 3) and the pin's report (§8i.4 step 6), each checked by sha256 first.
( C=D:/repos/redkiln-corpus; S=D:/repos/redkiln-sev/afk-tools/sp/s8i
  E=$S/S8I-D21149-stage/results/s-bin; G=$S/S8I-P7-results/results/s-bin
  st=$(git -C "$C" status --porcelain --untracked-files=all -- . ':(exclude)runner/target/'); rc=$?
  [ "$rc" -eq 0 ] && [ -z "$st" ] || { echo "FAIL: the corpus is not clean (rc=$rc)"; printf '%s\n' "$st"; exit 1; }
  git -C "$C" cat-file -e HEAD:results/s-bin/superseded/dry-run-d730d14f/dry-run.jsonl || { echo "FAIL: step 4a's results commit is not HEAD's"; exit 1; }
  git -C "$C" cat-file -e HEAD:results/s-bin/dry-run.jsonl || { echo "FAIL: no dry run at HEAD; run step 4a first"; exit 1; }
  for p in results/s-bin/fixture.json results/s-bin/greenness results/s-bin/pin-report-2026-09-05 results/s-bin.jsonl; do
    [ ! -e "$C/$p" ] || { echo "FAIL: $p already exists in the corpus"; exit 1; }
  done
  ck() { [ "$(sha256sum < "$1" | cut -c1-64)" = "$2" ] || { echo "FAIL: $1 is not the staged bytes ($2)"; exit 1; }; }
  ck "$E/fixture.json" a93044cfa901e80419b330a7b2933ed7c90e870e125c68e49111867d99ab0f2c
  ck "$E/spec-item-1.md" 11f11168140da3c442d11406de39ce040bc93dd9a6b9843a45c9b32c47c6220e
  ck "$E/spec-item-2.md" 66923901b7d1ed52f37156743c2dba897c79090b5a6205101d26d32a9f31ccd3
  ck "$E/spec-item-3.md" c08d409b9d3b0a4853a5c103a610f59d76f72008142e36bc6bedacd7c20284fb
  ck "$E/spec-item-4.md" d4ebdb9e6b412ca21b4e3b9ea625294a30e43b0d20474d99fb6e66083897fd63
  ck "$E/spec-item-5.md" 25da68e72cd46864cbfd268390b70b29a5a805038871cd4a5c90783fe3bb9170
  ck "$E/report-item-1.md" 9ae0c522041d2b5b20519e2ea7436e476758c0b33ec42b17c5d6941acf0f20f5
  ck "$E/report-item-2.md" a916fa9068ea7715d95b5115dd82bdcb62bc7bcc721fbc99f74337846c2dac02
  ck "$E/report-item-3.md" 00696b4556e49db047a9b0bf6a5d90c7476f56b604ed37e80f31716f9ef0d7de
  ck "$E/report-item-4.md" f0a9f60f92aac3a5e9fe0f245e69c2a7e6ffb0bccd21009faf614c533deed99a
  ck "$E/report-item-5.md" 8848e41c685d8538df587e0cda7049837d55df82856ee15c7c9fa6f1c6a98cdd
  ck "$G/greenness/item-1/index.json" 49e3b3f084d2c3bcbb7a00fd679567e81eec0cb62b4ab8ea94a6c6a496f41a0c
  ck "$G/greenness/item-1/headless.log" 1a675cd0870be05b38560937794da23b0d57cc1b64fa63978eefe2bfe38aa27c
  ck "$G/greenness/item-2/index.json" f511374108406e549174ccefc127dca78037d3fb51fbfa22b4821ecb9a830582
  ck "$G/greenness/item-2/headless.log" 058e7c5bd8b378f516859f8d6d26e0b403a35bab8f995dc91bdb979d5356fa70
  ck "$G/greenness/item-3/index.json" 9176f1ff8ace344a5e993f5efb9c7fdbe74e058aa6e0e208bb3dbb6f545e32ed
  ck "$G/greenness/item-3/headless.log" f383279c4c0724684935200d0eda327e4f259f683e1152297bca516bbbfc7e45
  ck "$G/greenness/item-4/index.json" ea43a11b3839a40d0c679b67a9b9bd536b1b75ca7c3ab7dd1509a0b9248d9d9e
  ck "$G/greenness/item-4/headless.log" e3031a72ecb5e3b8f45584993afb80787cd1126c31f5471d505685f6b3df551c
  ck "$G/greenness/item-5/index.json" 755517b4312585def291fc8c1fc28a2806b7f3dfca69bf7128b5f8bd452c1139
  ck "$G/greenness/item-5/headless.log" fabbcc89d8c8daab2c2fd11441c1406cef897eb62cdd61a6ff3e87acfb5fccfb
  ck "$G/pin-report-2026-09-05/index.json" c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6
  cp "$E/fixture.json" "$C/results/s-bin/fixture.json" || exit 1
  for n in 1 2 3 4 5; do
    cp "$E/spec-item-$n.md" "$E/report-item-$n.md" "$C/results/s-bin/" || exit 1
    mkdir -p "$C/results/s-bin/greenness/item-$n" || exit 1
    cp "$G/greenness/item-$n/index.json" "$G/greenness/item-$n/headless.log" "$C/results/s-bin/greenness/item-$n/" || exit 1
  done
  mkdir -p "$C/results/s-bin/pin-report-2026-09-05" || exit 1
  cp "$G/pin-report-2026-09-05/index.json" "$C/results/s-bin/pin-report-2026-09-05/index.json" || exit 1
  git -C "$C" add -f results/s-bin/fixture.json results/s-bin/spec-item-1.md results/s-bin/spec-item-2.md results/s-bin/spec-item-3.md results/s-bin/spec-item-4.md results/s-bin/spec-item-5.md results/s-bin/report-item-1.md results/s-bin/report-item-2.md results/s-bin/report-item-3.md results/s-bin/report-item-4.md results/s-bin/report-item-5.md results/s-bin/greenness results/s-bin/pin-report-2026-09-05 || exit 1
  git -C "$C" commit -m "s8i: owed item (e): fixture.json completed, the five constructed specs and reports, the greenness and pin reports (D21-149 item 4, D21149-8i-B)" || exit 1
  n=$(git -C "$C" diff-tree --no-commit-id --name-only -r HEAD | wc -l)
  [ "$n" -eq 22 ] || { echo "FAIL: the commit holds $n paths, not 22"; exit 1; }
  st=$(git -C "$C" status --porcelain --untracked-files=all -- results); rc=$?
  [ "$rc" -eq 0 ] && [ -z "$st" ] || { echo "FAIL: results/ is not clean after the commit (rc=$rc)"; printf '%s\n' "$st"; exit 1; }
  git -C "$C" log --oneline -1 )
# 10. As written above: the BrittonGames snapshot before B2 (WHEN=before-B2, then §8i.5's snapshot block). Commit it
#     with B2's results, or alone: it is an appended line of results/s-bin/brittongames-snapshots.txt.
# 10a. The B2 line. Re-sum the spend from the cost records first ($5.561026 after B1); B2 launches only if
#     (90.00 - spent) > 25.00. Then write ONE new line under D21-143's go-ahead lines, in §8i.5's fixed form, its
#     time the UTC at which it is written, immediately before the batch:
#     S-BIN go-ahead B2 <YYYY-MM-DDTHH:MM:SSZ> — pass 1: items 1–5, five Opus dispatches from D:/sb/rev/item-1..5 through wrap, caps used so far $5.56
# 10b. §8i.5's pre-check block, as written, with step 7's SBIN_ROOT and SBIN_NEED_BYTES exported in the same shell; then
#     §8i.5's runner launch block, as written, with METHOD-S amendment item 5's B2 argv and fixture.json's ceiling_secs:
SBIN_ARGV=(sbin-probe --pass 1 --fixture results/s-bin/fixture.json --out results/s-bin.jsonl --model opus --effort xhigh --timeout-secs 4500)
#     Then §8i.5's BrittonGames snapshot with WHEN=after-B2, and B2's results commit.
```

| step | what | USD bound | check before it |
| --- | --- | --- | --- |
| 1 | the corpus on `main`, clean | 0.00 | none |
| 2 | sign B0, then the B0 corpus commit | 0.00 | none |
| 3 | §8i.3's B0 block: `driver-tests.txt` | 0.00 | none |
| 4 | the dry run, its results committed, the dry-run tree removed | 0.00 | none |
| 5 | the settings row, then `claude --version` | 0.00 | none |
| 6 | sign B1 with the dry run's sha256 | — | (90.00 − spent) > 10.50 |
| 7 | §8i.5's pre-check, with `SBIN_ROOT` and `SBIN_NEED_BYTES` | 0.00 | none |
| 8 | ≤ 3 haiku flag probes through `wrap` | ≤ 0.50 in all | (90.00 − spent) > 0.50 before each |
| 9 | 5 capture sessions through `wrap`, each with its procedure-log line | ≤ 2.00 each, ≤ 10.00 in all, unmeasured | (90.00 − spent) > 2.00 before each |
| 10 | the BrittonGames snapshot before B2 | 0.00 | none |
| 11 *(dated 2026-10-03, s8i build, R1FIX-8i)* | at the close, before the formal Checkpoint 8i: the corpus back on the pin, clean | 0.00 | none |
| 12 *(dated 2026-10-03, s8i build, R3FIX-8i)* | at the close, after the last results commit, the last snapshot and step 11, before the formal Checkpoint 8i: the review root `D:/sb` removed | 0.00 | none |

```bash
# 1. The corpus onto main. It is detached at the pin 61f590e today, and its main is at 285e011.
( C=D:/repos/redkiln-corpus
  [ -z "$(git -C "$C" status --porcelain)" ] || { echo "FAIL: the corpus is not clean"; exit 1; }
  git -C "$C" switch main || exit 1
  [ "$(git -C "$C" rev-parse HEAD)" = 285e011c70cfa0fc92d91089792834846d99bcf7 ] || { echo "FAIL: main is not 285e011; re-check the patches apply"; exit 1; } )
# 2. Sign D21-143's B0 line in place first. Then the B0 corpus commit, from D21-140 item 2's private-copy patches.
#    DRV is P5's patch until the LFS fix (owed item (a)) lands; after that it is the fixed patch.
#    (dated 2026-10-03, s8i build) The fix landed: DRV is S8I-F1's patch, superseding P5's S8I-P5-sbin-driver.patch.
#    (dated 2026-10-03, s8i build, R1FIX-8i) DRV is S8I-R1FIX-8i's patch, superseding S8I-F1's (sha256 dd3ee74e…4f3a).
#    (dated 2026-10-03, s8i build, R2FIX-8i) DRV is S8I-R2FIX-8i's patch, superseding S8I-R1FIX-8i's (sha256 1922bb47…2b08).
#    (dated 2026-10-03, s8i build, R3FIX-8i) DRV is S8I-R3FIX-8i's patch, superseding S8I-R2FIX-8i's (sha256 81511152…68c1).
#    (dated 2026-10-03, s8i build, R4FIX-8i) DRV is S8I-R4FIX-8i's patch, superseding S8I-R3FIX-8i's (sha256 0bc28875…ea7dc).
#    (dated 2026-10-03, s8i build, R5FIX-8i) DRV is S8I-R5FIX-8i's patch, superseding S8I-R4FIX-8i's (sha256 934ad2c1…47b4); AUD is
#    S8I-R5FIX-8i's audit patch, superseding S8I-P6's (sha256 0d2964ba…7463), and its sha256 is checked too.
#    (dated 2026-10-03, s8i build, R6FIX-8i) DRV is S8I-R6FIX-8i's patch, superseding S8I-R5FIX-8i's (sha256 3357c3fb…6bb0); AUD is unchanged.
#    (dated 2026-10-03, s8i build, R7FIX-8i) DRV is S8I-R7FIX-8i's patch, superseding S8I-R6FIX-8i's (sha256 10b0e266…0fae); AUD is unchanged.
#    (dated 2026-10-03, s8i build, D21146-8i) DRV is S8I-D21146-8i's patch, superseding S8I-R7FIX-8i's (sha256 5b84a539…00ed); AUD is unchanged.
#    (dated 2026-10-03, s8i build, D21147-8i) DRV is S8I-D21147-8i's patch, superseding S8I-D21146-8i's (sha256 a24c896f…fc0d); AUD is unchanged.
#    (dated 2026-10-06, s8i build, D21149-8i-D) Step 2 ran: B0's commit 40c0332 applied this DRV and AUD, and it is not
#    run again. DRV is not superseded: D21-149's and D21-151's driver change is step 2a's runner patch,
#    S8I-D21149-8i-sbin-driver.patch (sha256 a904b4c0…e637), a new commit over B0's (the D21149-8i-A packet above).
( C=D:/repos/redkiln-corpus; S=D:/repos/redkiln-sev/afk-tools/sp/s8i
  DRV=${DRV:-$S/S8I-D21147-8i-sbin-driver.patch}; AUD=$S/S8I-R5FIX-8i-audit-bin.patch
  [ "$(sha256sum < "$DRV" | cut -c1-64)" = 0bf69ab4a1a1e8a396f278c63d1cd4fb943ac64ffc1898392a3c6e8cce54da3c ] || { echo "FAIL: $DRV is not S8I-D21147-8i's patch"; exit 1; }
  [ "$(sha256sum < "$AUD" | cut -c1-64)" = e0243d5d4edeb9a7b538889e346d488d6e60c61a2f036c31ea0ccf7cb4eb4d32 ] || { echo "FAIL: $AUD is not S8I-R5FIX-8i's audit patch"; exit 1; }
  for p in "$DRV" "$AUD"; do git -C "$C" -c core.autocrlf=false apply --check "$p" || { echo "FAIL: $p does not apply"; exit 1; }; done
  for p in "$DRV" "$AUD"; do git -C "$C" -c core.autocrlf=false apply "$p" || exit 1; done
  git -C "$C" add runner/src/main.rs runner/src/sbin.rs results/audit-s.py results/s-bin/audit-fixtures || exit 1
  git -C "$C" commit -m "s8i B0: the S-BIN driver (sbin-probe) and audit-s.py bin (D21-140 item 2)" || exit 1
  [ -z "$(git -C "$C" status --porcelain -- runner results/audit-s.py)" ] || { echo "FAIL: runner/ or audit-s.py not clean after the commit"; exit 1; } )
# 3. §8i.3's B0 block, as written. It writes results/s-bin/driver-tests.txt.
# 4. The dry run. It spawns the stub only and prints the dry run's and driver-tests.txt's sha256.
#    (dated 2026-10-03, s8i build, R2FIX-8i) Its runner-clean check now excludes runner/target/, which step 3 builds;
#    without that, step 4 and step 7 refused after step 3. The script's sha256 is 1599099691282b5334a3b1e9612b38dedcf331205e04e08b3ea1e7ba04a9b7f3.
#    (dated 2026-10-03, s8i build, R6FIX-8i; s8i refuting round 6) It required the 25-test reading and step 3 writes 33,
#    so it refused every time. It now requires 33 and checks the record's 34 §8i.7 names. Its sha256 is 5d112e098e510bec52d4334e40ebe3d4a6c6f689cd0232d83f1d261105fd2c2c.
#    (dated 2026-10-03, s8i build, D21147-8i) The script is unchanged. Its inputs moved (D21-147 item 3): D:/sb/dry/rev/item-1
#    is commit a8d9478b54556d6617963cab622b385109928ab3, and the dry-run fixture hashes
#    62f6469c11e3bc5e656b565ba28c79709ad30526b954d88017ebd69024b6a8b1 (§8i.4's D21147-8i note).
#    (dated 2026-10-05, s8i, operator sitting) Line 36 called nd_setup with D:/repos/..., which runbook/tools/nodispatch.sh
#    refuses (it needs an absolute POSIX directory), so the script refused every dry run before it built; the operator's
#    first run read 'FAIL: nd_setup needs an absolute POSIX directory'. Fixed to /d/repos/redkiln-rs/target/s8i-dry/nd, the
#    one change (the old script is kept as S8I-P8-dryrun-before-nd.sh). Its sha256 is now
#    c3353777ec4828ac34b7028f4a85e0bae4418b18f775c30e0b8da3038d92351f, superseding 5d112e09...2c2c.
#    (dated 2026-10-05, s8i, operator sitting) The second run read 'sed: can't read crates/rk-adapter-claude/src/config.rs':
#    nd_setup reads that file and runbook/tools/fake-claude.rs relative to the cwd (nodispatch.sh:47,49), and the script
#    had already cd'd into the corpus. It now runs nd_setup from /d/repos/redkiln-rs and cd's back (old bytes kept as
#    S8I-P8-dryrun-before-cwd.sh). Its sha256 is now c25b1387d3045d2f1d6a4d2e52ea709e66dc47ae5ba9efae86a2b65eaa97a007.
#    (dated 2026-10-06, s8i build, D21149-8i-D) Step 4 ran: corpus f94a196, dry run d730d14f…ed5a. It is run again as
#    step 4a (the D21149-8i-A packet above), where the script is 7c95fb14…bf08 (the 34-test reading and item 1's
#    real capture), D:/sb/dry/rev/item-1 is commit cc1b4b45…205b, and the dry-run fixture hashes 6d93cb9e…caf6.
bash D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P8_dryrun.sh
#    Then commit its results in the corpus, and remove the dry run's 21.5 GB review repository:
( C=D:/repos/redkiln-corpus
  git -C "$C" add results/s-bin/dry-run.jsonl results/s-bin/driver-tests.txt results/s-bin/dry-run results/s-bin/raw/dry-run results/s-bin/rendered/dry-run || exit 1
  git -C "$C" commit -m "s8i B0: S-BIN's zero-spend dry run and the driver's test output" || exit 1
  python D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_S8I-P8_review.py remove )
# 5. The CLI freeze before B1 (D21-43 N22; s8h §8h.8). ~/.claude-personal/settings.json has no "env" block today
#    (measured: its top-level keys are autoMode, effortLevel, enableWorkflows, enabledPlugins, extraKnownMarketplaces,
#    pluginConfigs, skipWorkflowUsageWarning, statusLine, theme). Add, by hand:   "env": { "DISABLE_AUTOUPDATER": "1" }
#    Then read the set's version from a plain terminal:   claude --version
# 6. Sign D21-143's B1 line in place, with the dry run's sha256 from step 4, only if (90.00 - spent) > 10.50.
# 7. §8i.5's pre-check block, as written, with these two values set in the same shell first:
export SBIN_ROOT=D:/sb
# *(dated 2026-10-04, s8i, operator sitting)* extension 3's figure is appended to METHOD-S: SBIN_NEED_BYTES = 34221223118. The line below
# was `export SBIN_NEED_BYTES=0` (0 makes the pre-check refuse); the comment after it is kept as written.
export SBIN_NEED_BYTES=34221223118   # was 0: REPLACE 0 with extension 3's figure (0 makes the pre-check refuse). It is
                           # ceil((largest base + largest growth) x 1.1). The largest base alone is 21,560,069,565
                           # bytes (item 3), so the figure is above 23,716,076,522.
                           # (dated 2026-10-03, s8i build, D21147-8i) This supersedes the two lines above. The
                           # bases are D21-147's, and the largest is still item 3's, 21,560,067,392 bytes (§8i.4's
                           # table), so the figure is at least 23,716,074,132.
# 8. B1's flag probes, at most three. For each, check (90.00 - spent) > 0.50, then run §8i.5's runner launch block with:
SBIN_ARGV=(sbin-probe --flag-probe --model haiku --out results/s-bin/probes.jsonl)
#    Stop after the probe that shows all three properties: the card adopted with Read+Bash under --json-schema,
#    apiKeySource none, and the init tool set equal to the card's. A probe that fails refuses non-zero: stop and report.
# 9. The five capture sessions, items 1 to 5 in order. For each N, check (90.00 - spent) > 2.00, then:
N=1   # then 2, 3, 4, 5
( [ ! -e D:/sb/cap/item-$N ] || { echo "FAIL: D:/sb/cap/item-$N exists; a copy is never reused"; exit 1; }
  mkdir -p D:/sb/cap && cp -a D:/sb/base/item-$N D:/sb/cap/item-$N )
#    Open the editor on D:/sb/cap/item-$N/OurGame/OurGame.uproject. Then run §8i.5's capture block with:
TREE=D:/sb/cap/item-$N
#    In the session: /ue-verify <ARGS[N]>, confirming first that monolith_status() reports this copy's project.
#    Copy each capture to D:/repos/redkiln-corpus/results/s-bin/captures/item-$N/ and hash it with sha256sum.
#    Append ONE line to results/s-bin/captures/procedure-log.txt in the corpus:
#    item=<n> tree=D:/sb/cap/item-<n> model=<id> cost_usd=<x.xx> sha256=<64 hex,comma-joined> shows_subject=<yes|no> utc=<YYYY-MM-DDTHH:MM:SSZ>
#    Close the session and the editor. No UnrealEditor process may survive (Halt 6). Delete D:/sb/cap/item-$N.
#    A shows_subject=no on any tree is a stop (§8i.6).
#    (dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's minor) As run, under D21-149 and D21-150: one line
#    per SESSION. sha256=none shows_subject=n/a for an item whose every asset capture_scene_preview rejected (items 2
#    and 3; captures []); sha256=none shows_subject=stalled for a session stalled on the instrument, at most one per
#    item, ahead of its binding line, then one retry on a fresh copy (item 5). METHOD-S's D21149-8i-A extension items
#    2 and 3; Checkpoint 8i (4)'s caplog_ok.
# 10. Before B2: run §8i.5's BrittonGames snapshot block with
WHEN=before-B2
# 11. (dated 2026-10-03, s8i build, R1FIX-8i; s8i refuting round 1) At the close, after the last results commit
#     and BEFORE the formal Checkpoint 8i: the corpus back onto the pin. Checkpoint 8i (11) replays Checkpoint 5, whose
#     `cargo test --workspace --features corpus` runs rk-git's the_corpus_is_at_the_pin_and_clean_before_anything_is_read:
#     HEAD must be CORPUS_SHA and `git status --porcelain` empty. Checkpoint 8i reads the corpus only through its object
#     store (cat-file, archive, merge-base against main), so a detached HEAD at the pin serves it. CORPUS_PIN does not
#     move (§8i.7). runner/target/ is Halt 1's untracked build output (the corpus has no .gitignore). It is the one
#     path removed, and any other change refuses.
( C=D:/repos/redkiln-corpus; PIN=$(sed -n 's/^CORPUS_SHA=//p' D:/repos/redkiln-rs/CORPUS_PIN)
  printf '%s\n' "$PIN" | grep -qxE '[0-9a-f]{40}' || { echo "FAIL: CORPUS_PIN names no CORPUS_SHA"; exit 1; }
  st=$(git -C "$C" status --porcelain --untracked-files=all -- . ':(exclude)runner/target/'); rc=$?
  [ "$rc" -eq 0 ] && [ -z "$st" ] || { echo "FAIL: the corpus holds changes beyond runner/target/ (rc=$rc) — commit or report them, never discard them"; printf '%s\n' "$st"; exit 1; }
  rm -rf "$C/runner/target" || exit 1
  git -C "$C" switch --detach "$PIN" || exit 1
  [ "$(git -C "$C" rev-parse HEAD)" = "$PIN" ] && [ -z "$(git -C "$C" status --porcelain)" ] || { echo "FAIL: the corpus is not at the pin and clean"; exit 1; }
  git -C "$C" merge-base --is-ancestor "$PIN" main || { echo "FAIL: the pin is not on the corpus's main"; exit 1; } )
# 12. (dated 2026-10-03, s8i build, R3FIX-8i; s8i refuting round 3) At the close, after the last results commit, the
#     last BrittonGames snapshot (after-B3, or after-B4 when B4 ran) and step 11, and BEFORE the formal Checkpoint 8i:
#     remove the review root, which Checkpoint 8i (12) requires gone. Nothing Checkpoint 8i or the finding reads lives
#     under it: the records, renderings, raw transcripts, captures and greenness reports are in the corpus (§8i.7).
#     Never run this before the close: B4 dispatches from the review repositories.
( R=D:/sb
  test -d "$R" || { echo "FAIL: $R does not exist; check it is the registered review root before going on"; exit 1; }
  pl=$(tasklist //FO CSV //NH); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$pl" ] || { echo "FAIL: the process list is unreadable (rc=$rc)"; exit 1; }
  printf '%s\n' "$pl" | grep -qi '^"unrealeditor' && { echo "FAIL: an UnrealEditor process survives (Halt 6); close it first"; exit 1; }
  bg=$(git -C D:/repos/BrittonGames worktree list --porcelain); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$bg" ] || { echo "FAIL: BrittonGames' worktree list is unreadable (rc=$rc)"; exit 1; }
  printf '%s\n' "$bg" | grep -qiE '^worktree d:/sb(/|$)' && { echo "FAIL: a tree under $R is registered in BrittonGames (§8i.4 forbids it); report it, never prune it here"; exit 1; }
  rm -rf "$R"; rc=$?
  [ "$rc" -eq 0 ] && [ ! -e "$R" ] || { echo "FAIL: $R did not delete (rc=$rc); Halt 6: a tree that will not delete is the handle, so stop and report"; exit 1; }
  echo "the review root $R is removed" )
```

`ARGS[N]` follows amendment item 2's rule: the item commit's changed `.uasset`/`.umap` paths in git's order, mapped to
object paths, then the literal suffix ` — write the capture_scene_preview of the k-th asset above to
_s-bin/captures/capture-<k>.png`. The paths come from P7's fixture, and item 5's comes from R's diff, read-only:
- 1 (`cc0cf2d01`): `/Game/UI/Inventory/WBP_StashRow`
- 2 (`fa1ea72f4`): `/Game/Tests/FTEST_Inventory_Stash`
- 3 (`e83463288`): `/GameCore/Blueprints/BP_GamePlayerState /GameCore/Data/Items/DA_Item_Sword`
- 4 (`a71265925`): `/Game/UI/Inventory/WBP_LoadoutScreen`
- 5 (R = `6a101bdcd`, once extension 1 is appended): `/Game/UI/HUD/WBP_GameVitals` *(dated 2026-10-03, s8i build)* Extension 1 is appended (METHOD-S, 2026-10-03), and base item 5 is built.

After each batch, the spend table's *spent so far* column is updated by dated edit, from the records, the probes and
the procedure log.

*(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* **Refuting round 9 (D21-147 item 3's round on the changed fixture): no blocker, no major.** Five dimensions, every one returned; 12 minors. The documentation minors are fixed here. One coverage minor is recorded, not fixed, because it would re-cut `DRV`: `a_dirty_or_mismatched_copy_refuses_before_spawning`'s case *"the stub missing"* builds no backlog directory at all, so no case holds the directory without the stub; the code refuses that shape (`[one]` arity plus `stub.is_file()`). Two finders each wrote one file to `%TEMP%` by mistake and removed it at once (an earlier panel's five backlog JSONs, D21-147 item 5; round 9's `ms_old`, a METHOD-S copy); a scan of %TEMP% after found no leak-token file outside `%TEMP%/claude`.

*(dated 2026-10-06, s8i, R11FIX-8i)* **Refuting rounds 10 and 11.** Round 10 (base `7fcca61`) found no blocker and one major, upheld 2-0: Checkpoint 8i did not tie the pinned capture files to `fixture.json`; R10FIX-8i (`94082e2`) built `capfiles_ok` with 14 synthetic pins, 7/7 guard mutants killed, and fixed its seven minors. Round 11 (base `524aadd`) found no blocker and no major, and four minors. Two wording minors are fixed here. **Two are owed before the formal Checkpoint 8i, by dated edit with synthetic controls:** (a) a read-only guard that lists the pinned corpus commit's `results/s-bin/captures/` (`git ls-tree -r --name-only`) and requires its `item-<n>/` entries to equal the pinned capture paths, so that an unpinned committed capture refuses; (b) a §8i.9 criterion and a (9) guard that F43 carries D21-152 item 2's disclosure, D21-152 item 3's evidence-source tag on each blocking finding, and D21-149 item 2's confound. Neither touches B2's inputs.

### 8i.6 Kill and abort conditions

Each of these stops the stage where it stands. The batch in progress is recorded as far as it went, the ledger gets a
dated line, and nothing resumes without a new go-ahead.

- **The runner does not build** (Halt 1), or a driver test is not observed red first.
- **The dry run fails, or any B1 probe shows** `--json-schema` inert, a tool set unequal to the card's, or
  `apiKeySource` other than `none` (Halt 3; F20). No Opus dispatch follows a failed B1.
- **A seed is caught by the suite** and no replacement from the 27 asset-only commits passes greenness, or **no
  candidate meets seed 5's registered replacement rule** (§8i.2 item 12). Either way the spike cannot run as
  registered.
- **A capture session cannot show the item's subject on any tree**, clean or seeded, or `monolith_status()` reports another
  tree's project. Then N19's producer cannot run as decided. The stage stops for a ledger line, and it never falls
  back to empty captures silently.
- **A fix is reachable** in any review tree, or a seed's blob oid does not match.
- **The CLI version moves** between the set's first record and any later dispatch (Halt 2).
- **A UE process survives a dispatch** (Halt 6). The copy it held is deleted, and no later dispatch uses its path.
- **A second observed model id** appears in the set (§8i.2 item 5).
- **A pre-spawn check fails** on a dispatch copy (§8i.4), or the corpus runner is not committed and clean at a
  batch (§8i.5).
- **BrittonGames moved**: a snapshot line differs from the first (§8i.5).
- **The same item voids twice** in one pass — a fixture defect, not noise. *(dated 2026-10-03, s8i build, R5FIX-8i)* Only the driver's runtime voids count. An operator void of a shadow-flagged record is `void: out-of-tree`, never a miss, gets no retry during the AFK run, and does not count toward *"the same (pass, item) voiding twice"* (D21-144).
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
(D21-43 N19); `results/s-bin/probes.jsonl` (B1's haiku probes); `results/s-bin/missing-fields.jsonl` (branch 2's
named fields, possibly empty); `results/s-bin/brittongames-snapshots.txt` (§8i.5); `runner/src/sbin.rs`;
`results/audit-s.py`. *(dated 2026-10-03, s8i build, R5FIX-8i)* Also `results/s-bin/oot-shadow-rulings.jsonl`, the operator's rulings on shadow-flagged records, ~~when any record is flagged~~ *(dated 2026-10-03, s8i build, R7FIX-8i; s8i refuting round 7's minor: always, an empty file when no record is flagged, so the pins block names it and Checkpoint 8i (3) requires it)* *(dated 2026-10-03, s8i build, R8-RECORD-8i: the italic span above closed; s8i refuting round 8's minor)*.

**Every S-BIN record** carries these fields beside the digest (`digest`, the schema-valid structured output):
- `spike` = `"s-bin"`, `pass`, `item`, `arm`;
- `void`, `null` or the named reason;
- `rerun`, `true` on a B4 re-run only;
- `dry_run`, which is `false`;
- `cli_version`, `observed_model`, `api_key_source`;
- METHOD-S §0's pre-flight fields: `card_adopted`, and `api_error_status`, which is `null` unless a platform error;
- `fix_unreachable`, and `seed_blob_verified`, which is `null` on a clean item; *(dated 2026-10-03, s8i build, R2FIX-8i; refuting round 2, a minor)* `fix_unreachable` is `true` on every record the driver writes, by construction. `dispatch_one` runs `check_fix_unreachable` on its copy, and precheck's `precheck_blob_oids` requires a single commit (`rev-list --all --count` reads 1). Either miss halts before the spawn, so no record is written for a copy whose fix is reachable. That is METHOD-S item 5's *"true only when the dispatch copy passes item 4's checks"*. Checkpoint 8i (4)'s `fix_unreachable` read guards the pinned records file, and the driver's own refusals are what its tests pin;
- the dispatch copy's `tree_path` and `tree_commit`, and its four `precheck_*` results (§8i.4);
- `runner_commit`, the corpus commit the runner was built from, read with a clean `runner/` status *(dated 2026-10-03, s8i build, R5FIX-8i)* the commit that last changed `runner/`, so a results-only commit does not move it;
- `runner_sha256`, the running binary's own bytes;
- `ceiling_secs`, `started_at` (unix seconds), `wall_secs`;
- `total_cost_usd`, which is never `null`;
- `ran_automation`, `cited_fields`, `capture_refs`, `raw_ref`;
- `detected`, which is `null` on a clean item;
- `policy_verdict`: the strict policy's `{blocks, blocking_files, counts}`, in S-TYPED's `policy.strict` shape.
- *(dated 2026-10-03, s8i build, R5FIX-8i)* `oot_shadow`: the shadow audit's `{verdict, flagged_reads, allowlist_version}` (D21-144). The list is now 34 names, and `the_dispatch_cap_refuses_before_spawning` asserts all 34.

**In this repository:** the METHOD-S §7 amendment (§8i.2), and B4's if one is needed; the finding draft
`spikes/F<next>-DRAFT.md` (next free F id at writing), headed `## F<n> — S-BIN …`, in F31's form — *Established by*
(dispatches, dates, arm, model, CLI version, corpus commit, cost), each pass's table with its detection, false
positives and cited fields, METHOD-S §3.4 evaluated **per pass**, §3.5's prediction against the outcome, the voids
named, and the limits (§3.6: *"n=5 is a type-design sample, not a rate"*; the entry's perceptual-seed selection;
seed 5's replacement and why). *(dated 2026-10-06, s8i build, R10FIX-8i: D21-152 and D21-149 item 2)* It also
carries D21-152 item 2's disclosure: the seeded items' reports carry the commit's own diagnosis (the widget, the
pre-fix value or state, and for item 5 the cold-verification method), so a §3.4 branch-1 result reads as *"sufficient
given the report"*, not *"ChangeSummary alone sufficient"*, and branch 2's signal is less likely. Each blocking
finding carries D21-152 item 3's recorded-only evidence-source tag (read from the asset, by an editor, Python or binary
read or a capture, or only strings in the spec or report), never used in scoring. And beside the false-positive
outcome it states D21-149 item 2's confound: capture presence follows asset type, and both seeds are widgets. It also carries a section headed `### AC-475's lift share`, with the measurement and
its command (§8i.8), **and a pins block**, which Checkpoint 8i parses, of exactly this form:

```text
<!-- s-bin-pins
corpus_commit=<40-hex corpus commit holding every file below>
cli_version=<x.y.z>
observed_model=<the one model id every valid record's init reported>
ceiling_secs=<n>
review_root=<the absolute directory the review trees were built under>
rule_outcome_pass1=<branch1|branch2-detected|branch3|undecided-fp|undecided-silent|undecided-partial>
rule_outcome_pass2=<the same six>
ac475_lift_share=<k>/<n>
file=<corpus-relative path> sha256=<64 hex>
-->
```

with one `file=` line per corpus file listed above, per raw transcript and per capture. Also in this repository:
- **Three rows in `spikes/FINDINGS.md`'s index**, in its own `| F | draft | spike | headline |` form: **F41**
  (`S-PORTS`) and **F42** (`S-GRAMMAR`), missing since their drafts were written (§8i.0 item 10; assigned here by the
  2026-09-24 revision brief), and the S-BIN row. Each headline is condensed from the draft's own `## F<n> —` line.
  *(dated 2026-10-02, s8i build)* F41's and F42's rows are written at stage open, after F40's row and in its form. The S-BIN row waits for its
  draft, at the reserved F43. *(dated 2026-10-06, s8i build, R10FIX-8i)* F43's draft carries D21-152's disclosure
  and evidence-source tag and D21-149 item 2's confound (the contents above).
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
| branch 3 in a pass (*"binary review needs a *rendered* representation … recorded as a requirement"*) | **the variant is not dropped.** It keeps the manifest and test fields the reviewers did cite, and **route: human gate** — an item with a `Binary`-classified path cannot receive an automated review verdict; the review stage opens a human gate carrying the typed summary (AC-357's shape) until a producer of the rendered representation exists. The kernel carries that summary as an opaque attachment, through D21-48 X16's type-neutral gate request (below). The requirement is recorded against ADR-0008's Phase-2 Unreal adapter |
| the passes disagree | the more conservative route, and the disagreement reported as a stability finding |
| an outcome the rule does not decide (`undecided-fp`, `undecided-silent`) | **route: human gate** (D21-43 N21: *"If either occurs, the `Binary` variant takes the human-gate route."*), with the outcome reported as *"not decided by the registered rule"* |
| exactly one seed missed, counted after any B4 re-run, with summary fields cited and no missing field named (`undecided-partial`) | **route: human gate** (D21-48 X3: *"Registered before dispatch as a third undecided outcome"*, *"routed to the human gate, as D21-43 N21 routes the other two"*), with the outcome reported as *"not decided by the registered rule"* |

**The route, mechanically.** The pins carry `rule_outcome_pass1` and `rule_outcome_pass2`. Each is one of six
values (five until D21-48 X3):
- `branch1`;
- `branch2-detected`, where B4's re-run detected;
- `branch3`, which includes a branch 2 whose re-run missed;
- `undecided-fp`;
- `undecided-silent`;
- `undecided-partial` (D21-48 X3).

The route is `reviewer` exactly when both passes are `branch1` or `branch2-detected`. Otherwise it is `human gate`.
Checkpoint 8i does not trust the pins for these values. Guard (4) computes each pass's outcome from the
pinned records and `missing-fields.jsonl`. It re-runs the pinned `audit-s.py bin` over them and requires the pins,
the audit and its own computation to agree. A B4 re-run record exists for a `(pass, item)` exactly when that
missed seed has a named field. Guard (7) then asserts that the record's route line agrees with the computed
outcomes.

**If the answer is "no, the reviewer cannot verify without a diff"**, the design has already written the falsifier,
and the record answers it clause by clause rather than by silence: *"`ChangeSummary` is not a type problem but an
evidence problem. The review gate's applicability to binary-asset toolchains is withdrawn, ADR-0008's Phase-2 Unreal
adapter loses its premise, and the five toolchain-keyed kinds (ADR-0073) are re-cut."* The `Binary` variant then
encodes that answer — the human-gate route and the recorded requirement — and each of the three consequences is
either taken in the record or named as owed with an owner.

*(dated 2026-10-02, s8i build)* **Stage-open measurements, before any Rust edit** (read-only, at `09018dd`):

1. **`s8a`'s split.** `crates/rk-ports/src/toolchain.rs` declares `pub enum ChangeSummary {`,
   `pub struct TextChangeSummary {` and `pub struct BinaryChangeSummary(pub(crate) ());` [measured, `grep -n`]. The
   sealed-slot test is `change_summary::binary_arm_is_not_constructible_outside_rk_ports` in
   `crates/rk-ports/tests/it/change_summary.rs`, whose body is
   `t.compile_fail("tests/ui/change_summary_binary_not_constructible.rs");`. Its `trybuild` case is
   `crates/rk-ports/tests/ui/change_summary_binary_not_constructible.rs`, beside its `.stderr`. So **`CS` = `rk-ports`**,
   and Checkpoint 8i (8)'s default `CS_CRATE=rk-ports` stands.
2. **The review-input seam (`review_input`'s hook point).** The `step.rs` arm quoted below
   (`"change_summary" => toolchain.change_summary("").map(|_| ())`) no longer exists. `s8d` P4 replaced it with
   `ToolCall::Port(PortMethod::SummariseChanges) =>`, which calls
   `let summary = toolchain.change_summary(range, bound)?;` and serialises the value with
   `serde_json::to_value(&summary)` under `PortMethod::SummariseChanges.output_kind()`. The review dispatch's input
   is then assembled **type-neutrally in rk-kernel**:
   - The shipped `adversarial-review` activity's `review` agent step declares `change = "artifact:ChangeSummary"`.
   - `runtime::reference::resolve_inputs` resolves that binding to a `serde_json::Value`.
   - `agent_arm`'s `with_resolved_inputs(own, &step.id, &run.inputs)` joins it into the step's own
     `ActivitySpec.inputs`, which `rk-adapter-claude`'s `user_message` serialises into the prompt.

   rk-cli holds the *typed* value at two places, and the kernel types neither:
   - The `Toolchain` decorators in `crates/rk-cli/src/verbs/run.rs`, for example
     `impl rk_ports::toolchain::Toolchain for ChecklistBridge`, whose `change_summary` delegates
     `self.inner.change_summary(range, bound)`.
   - The per-step spec builder in the same file
     (`let mut step_specs: std::collections::BTreeMap<String, rk_types::activity::ActivitySpec> =`).

   `review_pass.rs` is ADR-0162's re-execution pass, and it is not the input seam. **`review_input` hooks at rk-cli's
   `Toolchain` decorator layer in `verbs/run.rs`.** There it reads the item's typed `ChangeSummary` for the item's range
   and builds what the reviewer receives, and the kernel carries the serialised value opaque. On the human-gate route,
   its refusal leaves through X16's type-neutral request. The seam is reachable **without the kernel naming the
   type**, so the STOP below does not fire on this measurement. Which of the two rk-cli places carries the code is the
   post-dispatch Rust package's choice, made under the record's route, and neither needs rk-kernel to name the type.
3. **rk-store's render module.** `crates/rk-store/src/render.rs` (`pub mod render;` in `lib.rs`) and
   `crates/rk-store/tests/it/render.rs` (`mod render;` in `tests/it/main.rs`) exist. So the two `render::ac154_*`
   tests keep the module name `render::`, and no rename note is owed.
4. **Measured and raised, not repaired here: Checkpoint 8i (8)'s kernel type scan cannot pass on today's tree.** Its
   `KT_RE` scan drops comment lines but does not blank string literals. At HEAD it fires on two kernel code lines that
   carry the artifact-kind *string* and do not name the type:
   - `crates/rk-kernel/src/lattice.rs`: `&& ["Changeset", "ImplReport", "ChangeSummary"]`
   - `crates/rk-kernel/src/workflow/memo.rs`: `Self::SummariseChanges => "ChangeSummary",`

   [measured: (8)'s own grep and `comment_line` filter, run read-only]. Checkpoint 8a's kernel filter (`k_filter`)
   blanks string literals before it matches, so 8a passes on the same lines. The options, most reversible first:
   - (a) Blank string literals in (8)'s scan as 8a's `k_filter` does, and add a control that admits a string-literal
     line.
   - (b) Admit the two lines by a counted, reasoned exemption.
   - (c) Rename the two literal sites, which is a kernel change no stage owes.

   This gates Checkpoint 8i, not B0 or B1, and it is left for a ruling.
   *(dated 2026-10-03, s8i build)* **Ruled and built.** D21-141 took (a), provisionally under D21-105 (Weigh-In
   wi-45f99f). Package S8I-P8 made the dated edit in (8): `kt_scan` blanks string literals with 8a's own `sed`,
   after `comment_line`, and the scan and its controls share that function. Two controls were added: the
   `lattice.rs` string-literal line is admitted, and a line that holds the type beside the same string is caught.
   (8)'s lines were run read-only from this file's bytes over `crates/rk-kernel/src` at `d349359`. The scan passed
   with all six controls, and the drafted pipeline still found the two lines above. Two in-memory mutants each
   failed the block: dropping the blanking, and blanking the type everywhere.
   *(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* **The operator REJECTED D21-141's (a) on 2026-10-03 and took (B)**, a counted exemption (D21-141's dated line): `kt_scan` loses its string and char blanking, and exactly the two lines above are exempt, matched on path plus the whole trimmed line, each once, exactly 2 admitted. It is owed by dated edit before the formal Checkpoint 8i, with its controls, and one refuting round scoped to (8). The paragraph above describes the superseded build.
   *(dated 2026-10-06, s8i build, D21149-8i-C)* **(B) is built** in (8) by dated edit. `kt_scan` loses both `sed` blankings; an `awk` stage admits a line only if its path and its whole trimmed line (spaces, tabs and CR stripped) equal one entry of `KT_EXEMPT`. It prints any entry that matched other than once, and any total other than 2, so a second match, a stale entry or a grown list fails the scan. The controls share `kt_scan` through `kt_ctl`, and each control is the two exempt lines plus one line, so only the added line can fire. Fourteen must fire and three must be admitted (the §8i.9 bullet lists them). (8)'s scan lines were run read-only from this file's bytes over `crates/rk-kernel/src` at `36dd8ac` (4 raw hits: two doc comments dropped, the two exempt lines admitted, nothing printed), with all 17 controls passing. Red first: run against the superseded blanking `kt_scan`, 9 of the 17 controls failed for their stated reasons (other path, duplicate, the three probes, both stale entries, twice-plus-stale, third entry). Twelve in-memory mutants of the scan were run: ten were killed. Two survive and are recorded. One keeps a trailing CR untrimmed; it is equivalent on the box this block runs on, because Git Bash's GNU grep 3.0 strips CR before `awk` sees the line (measured, and a CRLF synthetic base passes under both). The other empties the real-scan line `kt=…`; it is unobservable over a tree with no type use, since the controls exercise `kt_scan`, which that line calls. The refuting round scoped to (8) that D21-141's dated line requires is the orchestrator's and is not yet run. Proposed until that round.

**The Rust — named, red first, each with its mutant.** `CS` below is the crate `s8a`'s record homes the type in
(`rk-ports` at `ca8f387`; ADR-0099 clause 5). `s8a`'s own test that the slot is sealed —
`change_summary::binary_arm_is_not_constructible_outside_rk_ports`, with its `trybuild` case — is **superseded by
name** in the same commit, and so is Checkpoint 8a's guard that the sealed payload has no `pub` field. *(Noted 2026-09-25, s8a fix round 8:
so is guard (6)'s `sealed_open` floor, which `s8a` fix round 7 added. It asserts `OPEN impls=0 ctors=0 aliases=0` over
`crates/rk-ports/src` *(amended 2026-09-25, s8a fix round 9: it read `OPEN impls=0 ctors=0`; an alias or a renaming
`use` of the payload now counts)*, and the recorded constructor this stage writes is an impl block it fails on. `s8a`'s
block takes its dated amendment for the floor in the same commit as the no-`pub`-field line.)* So is the same pin in
Checkpoint 5's amended block: `s8a` §8a.3's *"the sealed payload type declares no `pub` field"*, which Checkpoint 8i
re-runs. `s5`'s block takes its dated amendment in the same commit, naming the pin that replaces it: the record's
construction rule, carried by the `trybuild` case below. Checkpoint 8a
names both, so row 8a becomes `passed <hash>, re-run pending` (the preamble's maintenance rule) and `s8a`'s block
takes a dated amendment. Checkpoint 8i then re-runs that amended block whole (guard (11)), and that discharges the
pending re-run. Whether the Rust adapter's `change_summary::change_summary_refuses_a_range_carrying_a_binary_path`
and `s8a`'s `change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path` stand follows the record's
`Binary producers in Phase 1` line. With `none`, both stand unchanged.
*(Noted 2026-09-25, `s8a` fix round 4: there are three such pins, not two. `s8a` fix round 1 added the Rust
adapter's `change_summary::change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold` (`s8a` §8a.9). Its
fate follows the same `Binary producers in Phase 1` line, and with `none` all three stand unchanged.)*

| test | where | red because | mutant |
| --- | --- | --- | --- |
| `change_summary::binary_is_constructible_only_through_its_recorded_constructor` | `CS`, `--test it` | the slot is sealed; nothing constructs | an unchecked constructor |
| `change_summary::binary_rejects_a_text_patch_for_a_binary_classified_path` | `CS` | no check exists | accept the patch (AC-497's AND) |
| `ui::trybuild_pins_the_binary_change_summary_construction_rule` | `CS`; a new `tests/ui` case with a committed `.stderr` | its committed `.stderr` names the recorded constructor rule, and today the case fails for `s8a`'s sealed-slot reason instead | make the fields `pub` |
| `change_summary::binary_field_<name>`, one per field in the record | `CS` | the field does not exist | named per field in the record (e.g. size inferred rather than read from the pointer) |
| `render::ac154_binary_entries_render_from_typed_fields` | `rk-store`, a golden file pinned `-text` (ADR-0151) | no binary rendering | drop a typed field |
| `render::ac154_no_binary_content_is_embedded_parsed_or_diffed_as_text` | `rk-store` | no binary rendering | embed the asset's bytes |
| `review_input::ac497_binary_classification_assembles_no_diff_and_carries_the_typed_summary` | `rk-cli`, the new `review_input` module | the review input has no `Binary` path | call the diff assembler for a `Binary` path |
| `review_input::binary_item_reaches_the_reviewer_with_the_typed_summary` **or** `review_input::binary_item_refuses_an_automated_verdict_and_opens_a_human_gate` — the one the record's route names | `rk-cli`, `review_input` | no route exists | the other route's behaviour |
| `gate_request::automated_verdict_refused_opens_a_gate_through_the_existing_reach_path` — **human-gate route only** (D21-48 X16) | `rk-kernel`, `--test it` (`tests/it/gate_request.rs`) | no gate request exists: a kernel gate opens only from a gate step's free `kind` string | open the gate with a fixed token instead of one derived through `GateRecord::derive_reach` |
| `gate_request::the_reason_is_closed_and_an_unknown_reason_is_refused_by_name` — **human-gate route only** | `rk-kernel` | no reason type exists | carry an unknown reason string through as free text (an `Other(String)` arm) |
| `gate_request::the_attachment_is_carried_byte_for_byte_and_never_parsed` — **human-gate route only** | `rk-kernel`; the fixture attachment is bytes that are neither UTF-8 nor JSON | no attachment exists | decode the attachment as UTF-8 JSON before carrying it, which fails on the fixture |
| `classify::ac497_classify_binary_paths` (exists) | both toolchain adapters | — (re-run) | — |

If `s8f` named rk-store's render module otherwise, the two `render::` tests follow it and this table takes a dated
note first. *D21-48 (2026-09-24):* on the human-gate route,
`review_input::binary_item_refuses_an_automated_verdict_and_opens_a_human_gate` also pins the hand-off to X16's
request. The refusal reaches the kernel with the reason `automated-verdict-refused`, and the attachment's bytes equal
`review_input`'s serialisation of the typed summary. A second mutant must flip it: open the gate through a gate
step's free `kind` string with no attachment. The three `gate_request::` rows are G = 3 of this stage's new tests
on the human-gate route, and none on the reviewer route. Mutant cycles run **from Python, never the shell**, as `nodispatch python <script>` (§8i.3).

**Who assembles the review input, and where the `Binary` route lives.** No stage owned either at `fc80687`:
- `rk-cli`'s `review_pass` is ADR-0162's re-execution pass (`crates/rk-cli/src/review_pass.rs`, called from
  `verbs/run.rs`).
- rk-kernel's step runtime maps the `change_summary` verb and drops the value: `"change_summary" =>
  toolchain.change_summary("").map(|_| ())` in `crates/rk-kernel/src/runtime/step.rs` [measured].

**This stage owns them**, in a new `rk-cli` module `review_input` (`crates/rk-cli/src/review_input.rs`, with its
tests in `crates/rk-cli/tests/it/review_input.rs`). They do not go in rk-kernel: Checkpoint 8a's `rk-kernel`
production ban, ADR-0135's surviving forbids bullet (`s8a` §8a.3), forbids `crates/rk-kernel/src` to name the
type. `review_input` reads the item's `ChangeSummary` through the Toolchain port, for the item's range, and builds
what the reviewer receives. For a `Binary`-classified path it assembles no diff. On the human-gate route it returns
a refusal of an automated verdict. That refusal opens the gate through D21-48 X16's type-neutral kernel gate
request (below), and the kernel never names the type.

**The gate path. Taken, D21-48 X16:** *"**A type-neutral gate path.** The kernel gains a gate request with a closed
reason (`automated-verdict-refused`) and the summary as an opaque attachment it never types. ADR-0135's forbids
bullet and Checkpoint 8a's ban stand."* The premise is measured. The kernel's gate record carries a free `kind`
string and no attachment: `GateRecord` holds `token: Digest`, `kind: String`, `tree_hash: String`,
`deadline: Timestamp` and `reach: u32`, and its constructors are `derive` and `derive_reach`
(`crates/rk-kernel/src/workflow/fsm.rs`) [measured at `fc80687`]. So no existing path can carry the refusal and the
summary unless the kernel names the type. On the human-gate route this stage adds three things to rk-kernel's
`workflow::gate`:
- **A closed reason type** with exactly one value, spelled `automated-verdict-refused`. The kernel refuses a reason
  string it does not know by name, and never carries one as free text.
- **An opaque attachment**: bytes and their digest. The kernel carries the bytes byte-for-byte on the opened gate's
  event and never parses, decodes or types them. `review_input` in rk-cli serialises the typed summary into it. Only
  rk-cli, and the render side that reads the gate, know that the bytes are a `ChangeSummary`.
- **A request** that opens the gate through the existing reach path, `gate::reach`. The token is derived by
  `GateRecord::derive_reach`, as every other gate's token is. The reason is the gate's kind, and the attachment's
  digest is carried beside it.

On the reviewer route nothing refuses, so the request is not built. Code that nothing calls would pass its own lint.
The `Binary` record names the request when its route is `human gate`. Checkpoint 8i (8) asserts both routes' sides.
On either route, `crates/rk-kernel/src` names no `ChangeSummary` on a code line.

At stage open, before the first Rust edit, measure where the review dispatch's input is assembled after
`s8e`–`s8h`. Write it here by dated note; `review_input` hooks there. If that seam cannot be reached unless the
kernel names the type, that alone is still a stop for a record. X16 decides the gate path, not the input seam. An
ADR-0135 supersession is not this file's to take, and the gate path needs none.
*D21-48 (2026-09-24): this replaces the stage-open measurement of "which kernel gate path can carry the refusal" and
its stop.*

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
S-BIN's pending state — at `ca8f387`, **21** design lines and **17** register lines *(dated 2026-10-03, s8i build, R4FIX-8i: at `730a4cc` the register holds **19**, after s8g-era edits; the design still holds 21 [measured: `grep -E 'S-BIN' <file> | grep -cE "$STALE"`]; guard (9)'s floor reads 19)* carry `S-BIN` beside one of
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

**Three lines whose condition lapses here, and whose they are.** Checkpoint 5's `ChangeSummary` line bans the type
outside its home *"until S-BIN reports"* (ADR-0099 clause 5's words, as `s8a`'s record restated them): if `s8a`'s
record kept that condition, it lapses in this stage's record commit and `s5`'s line takes a dated amendment in the
same commit, because the render and review sites above name the type. And `s8`'s standing prohibition, which `s8a`'s
file rewrites to *"no `Binary` arm constructed outside `rk-ports`, no `ChangeSummary` definition outside it"*: the
render and review tests above construct `Binary` outside `rk-ports`, and `s8`'s guard greps all of `crates/`, tests
included. So this stage's record commit amends that bullet and guard by dated note to the record's construction rule
— the stage whose change makes a guard false owns its amendment, as `s8a` did for the same bullet. And `s8`'s bullet *"No document, section or commit message describes this
design as final"*, whose next sentence names *"S-BIN, S-H1c and P8's unrun arms"* as gating that label: S-BIN
leaves that gate here. So the same commit gives the line carrying `S-BIN, S-H1c and P8` an inline dated note,
`[S-BIN reported (s8i) <date>: S-H1c and P8's unrun arms still gate the label]`, and never rewrites it. Guard (9)
asserts the note.

### 8i.9 Checkpoint 8i

- **The records are located by the ids recorded at write time** (D21-47). `SBIN_F`, `BIN_ADR` and `SBIN_LEDGER` are
  written into (0) by dated edit, never found by a headline or an "exactly one" count.
- *(dated 2026-10-03, s8i build, R5FIX-8i)* **The out-of-tree audit's replay readings (D21-144's acceptance), measured
  before any dispatch.** 838 reviewer transcripts: 838 sessions, 13,147 tool uses. 71 runtime voids, every one
  card-shaped: R4FIX-8i's 69, each still voiding, and 2 new. One is genuine (`find /d -name "*.js" -path
  "*/workflows/*"`, a walk of D:). One is a false void: a session read a mistyped spelling of its own task-output path
  under `<temp>/claude`, a directory that does not exist. Exempting it would open other slugs, so it is disclosed.
  Before the task-output exemption and the two replay fixes, 64 more sessions voided on their own task output, and one
  more on `/tmp/../c/...`. The shadow audit flags 312 of the 838 sessions (10,786 reads), all 71 voids among them, so
  241 sessions are flagged and not voided: the allowlist's measured false-flag rate. By class of the flagged path:
  `D:/repos/redkiln-rs` 4,109 reads in 67 sessions; temp not written first 2,420 in 58; Git Bash words outside the
  allowlist (most are regex or sed operands read as paths) 1,544 in 263; other D: paths 1,013 in 22; other words
  (escapes read as paths) 584 in 156; unresolved `$VAR` or `$(...)` 364 in 116; the session's own task output 305 in
  77; other C: paths 257 in 10; the review root outside the copy 185 in 17; the home directory 5 in 2. The
  `/tmp/*.bak` write-then-restore pattern is clear; one `rm` of a temp path never written stays flagged. 754 corpus
  records: 0 voids, auditing zero read inputs (the records carry only the digest), so not evidence of correctness. (4)
  now also requires every record to carry an `oot_shadow` verdict and no flagged record to be held unruled, and
  re-runs the audit with the rulings file. *(dated 2026-10-03, s8i build, R6FIX-8i)* Re-measured over R6FIX-8i's driver: the same 71 voids of 838
  sessions, with the same 632 evidence lines; 312 sessions flagged, 10,793 reads (7 new, all in 6 sessions already
  flagged); 754 records, 0 (METHOD-S's R6FIX-8i extension, item 4). (4) checks the shadow and the hold before the
  pairs, so a held record prints the hold message.
- **The S-BIN records exist and are pinned.**
  - The finding draft and its pins exist.
  - Every pinned corpus file matches byte-for-byte at the pinned commit on the corpus's `main`.
  - There are ten valid records, one per `(pass, item)`. They carry one CLI version, one model and `apiKeySource`
    `none`. *(dated 2026-10-03, s8i build, R2FIX-8i)* The `apiKeySource` line reads the valid records only, as this bullet says. A void
    with no init carries no source and is retried by design. Every record, voids included, also names its launch
    record and carries that record's CLI version, which equals its own `cli_version` (METHOD-S item 11).
  - On every valid record, `card_adopted` is true, `api_error_status` is null, the fix is unreachable and the seeds
    are verified.
  - Every dispatch copy passed its pre-spawn checks at a path used once. Its commit is its item's review commit.
  - One committed runner, carrying the pinned `runner/` tree, dispatched them all.
  - No dry run is in the set.
  - The caps held: at most 18 Opus dispatches and 3 haiku probes, and at most $90.00 over the records, the probes and
    the capture sessions. No cost is missing.
  - The amendment was committed before the first record. METHOD-S.md has been append-only from the amendment's parent
    on, over the whole file.
  - ~~The captures are pinned for all five items, and the procedure log says "yes" for each.~~ *(dated 2026-10-06, s8i build, D21149-8i-A: D21-149
    and D21-150. Every item has its procedure-log lines; its last line binds. An item whose every asset
    `capture_scene_preview` rejected (items 2 and 3) carries `captures: []`, no pinned capture, and a binding line
    `sha256=none shows_subject=n/a`. Every other item (1, 4 and 5) has its captures pinned and a binding `yes` line
    carrying exactly their hashes. At most one `shows_subject=stalled` line, with `sha256=none`, stands ahead of an
    item's binding line (item 5). Every line, a stalled one included, carries its model and cost, and every line's
    cost is summed into the cap.)* *(dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's major: each item's pinned
    capture files are exactly its `fixture.json` captures, sha256 and path basename, each once, and an item with
    `captures: []` pins none; (4)'s `capfiles_ok`, with its synthetic pins.)* The stripped set is one set across the five, and no registered leak token is left.
  - The go-ahead lines are one per batch, in order against the records. The B1 line names Checkpoint 8h's recorded
    pass, the pinned dry run and the amendment (D21-43 N19, N20, N22).
  - BrittonGames' snapshots are unchanged from before B2 through every batch.
- **Each pass's rule outcome is computed, not trusted.** The records, the pinned audit re-run over them and the pins
  agree. Every computed outcome is one of the six registered values (D21-48 X3 registered `undecided-partial`),
  and any other shape fails the block. Seven synthetic record sets run through the same computation first, and
  together they must reach all six values.
- **The driver's ~~twenty-five~~ ~~thirty-three~~ thirty-four tests pass at the pinned corpus commit.** *(dated 2026-10-06, s8i build, D21149-8i-A: D21-151's named test is added, and D21-149's inverted capture test is renamed, as guard (6) selects and asserts; it read thirty-three)* *(dated 2026-10-03, s8i build, R6FIX-8i: §8i.3's 25 and R5FIX-8i's 8 (D21-144), as guard (6) selects and asserts; it read twenty-five)* They are rebuilt from the object store, not
  from a working tree, under the no-dispatch wrapper.
- **The `Binary` variant is constructible only as the record says.**
  - The record exists and is accepted, and its README index row exists.
  - Its named tests pass, in the counts its field set implies.
  - No `Binary` construction call appears on a code line outside the type's crate, except where the record names a
    producer.
  - Its route is the one the computed per-pass outcomes give (D21-43 N21, D21-48 X3).
  - On the human-gate route, the refusal opens its gate through X16's type-neutral kernel request. Its three
    `gate_request::` tests pass, and the closed reason is spelled exactly once on a kernel code line. On the reviewer
    route the request is not built, and the reason is not spelled.
  - On either route, `crates/rk-kernel/src` names no `ChangeSummary` or `BinaryChangeSummary` on a code line. ~~Two~~ ~~Four~~
    synthetic controls must fire and ~~two~~ three must be admitted *(dated 2026-10-03, s8i build, R1FIX-8i: three and three since D21-141, and a fourth that must fire, a `'"'` char literal before a type use)* (D21-48 X16: ADR-0135's forbids bullet and Checkpoint
    8a's ban stand).
    *(dated 2026-10-06, s8i build, D21149-8i-C)* **D21-141's (B):** the scan blanks no string or char literal. Exactly two lines are exempt, matched on path plus the whole trimmed line, each exactly once, with exactly 2 admitted: `lattice.rs`'s `&& ["Changeset", "ImplReport", "ChangeSummary"]` and `memo.rs`'s `Self::SummariseChanges => "ChangeSummary",`. **Fourteen** synthetic controls must fire and **three** must be admitted. The fourteen are the four kept (a `use`, a field type, a type beside the string, the `'"'` char literal) plus ten new ones: a type use appended to an exempt line, exempt content at another path, a duplicated exempt line, the three blanking probes (a raw string with an inner quote, the closing line of a multi-line string, `r"\"`), each entry stale, one entry twice with the other stale, and a third entry. The three admitted are the two exempt lines alone, a doc comment and the `change_summary` method call.
- **The notes are in place.**
  - AC-154, AC-356, AC-497 and AC-475 carry their dated notes. AC-475's states the measured lift share.
  - Both "four of five" design lines carry their dated correction.
  - `s8`'s final-label bullet carries its dated note.
  - No design or register line still states S-BIN as pending without a note.
  - Every finding draft has its index row, F41 and F42 included.
- **The whole suite is green**, over the floor re-derived from Checkpoint 8h's recorded count (~~1243 expected:
  1189 + 54, D21-48 floor propagation, 2026-09-24; it read 1222~~ *(dated 2026-10-02, s8i build)* **1404** passed over **44** result lines,
  Checkpoint 8h's recorded count at `d24d15a`, `PROGRESS.md` row 8h and D21-138) plus this stage's 6 + N + G, and fmt and clippy are clean. G is 3 on the human-gate route and 0 on the
  reviewer route (D21-48 X16).
- **Checkpoints 8h, 8a and 5 re-run whole and green.** Checkpoint 8h is the entry gate. Checkpoints 8a and 5 are made
  provisional by this stage's edits. No other checkpoint is re-run directly.
  - Each replay runs with `RK_CP_NESTED=1` (D21-46), which 8h and 8a honour; Checkpoint 5 nests nothing.
  - A replayed 8h or 8a block that stops honouring the variable is refused, not run for hours.
  - When this block is itself run with `RK_CP_NESTED=1`, it replays nothing. Either way it supplies no input to a
    replay: each replayed block carries its own record ids as literals (D21-47).
  - *(dated 2026-10-03, s8i build, R1FIX-8i)* Checkpoint 5's replay runs `cargo test --workspace --features corpus`, whose rk-git test
    `the_corpus_is_at_the_pin_and_clean_before_anything_is_read` needs the corpus checkout at `CORPUS_PIN`'s
    `CORPUS_SHA`, and clean. The packet's step 1 puts the corpus on `main` for S-BIN's commits, so its step 11 puts it
    back on the pin, clean, before this block runs (§8i.5). This block reads the corpus only through its object
    store, so the pin serves it.
- **This block dispatches nothing.**
  - It spawns no `claude` of its own.
  - It refuses an extracted block that carries a shape that executes a dispatch.
  - It runs with no launch record, so a nested `redkiln run` refuses before it spawns.
  - Every process-level cargo run and every replay runs inside D21-47's shared wrapper, `runbook/tools/nodispatch.sh`:
    a compiled fake `claude` first on PATH that prints the pin, every other `claude` dropped from PATH, and
    `CLAUDE_CONFIG_DIR` at a trap proved to start empty. `nd_check` follows each.
  - Its dispatch scan admits Checkpoint 8h (10)'s two sandboxed `wrap` runs and argv-screen list only by counted,
    reasoned exemptions tied to their premises.
  - The trap must still be empty afterwards, and the fake must never have been called with anything but
    `--version`.
  - It fails if a new session transcript appears under either account root while it runs.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`. Every cargo reading is
# `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`, MODULE-QUALIFIED and `--exact`, and asserts
# each test's own `... ok` line AND an exact count: a bare name under `--exact` runs zero tests and exits 0.
case "$(uname -s)" in MINGW*|MSYS*) : ;; *) echo "FAIL: this block runs on the Windows box that holds the corpus"; exit 1;; esac
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — (0)'s tripwire reads new session transcripts, and a Claude session writes them"; exit 1; }
# Python reads files from $T, so $T sits under target/, which Windows processes can see (Git Bash's /tmp is not).
mkdir -p target/cp8i || { echo "FAIL: could not create target/cp8i"; exit 1; }
T=$(mktemp -d "$PWD/target/cp8i/t.XXXXXX"); rc=$?
[ "$rc" -eq 0 ] && [ -d "$T" ] || { echo "FAIL: no scratch directory (rc=$rc)"; exit 1; }

# ---- (0) THE IDS RECORDED AT WRITE TIME, THE TRIPWIRE, AND THE NO-DISPATCH WRAPPER — all read or built FIRST.
# D21-47: a record is located by the id recorded when it was written, never by a title phrase or an "exactly one"
# count. The three ids are written into these three lines by a dated edit in the work commit, on the day each record
# is written (§8i.10). Empty means not yet written, and the block stops here.
SBIN_F=
BIN_ADR=
# *(dated 2026-10-03, s8i build)* D21-143, S-BIN's go-ahead entry, was written (OPEN) on 2026-10-03; SBIN_LEDGER records it (D21-47).
SBIN_LEDGER=D21-143
printf '%s\n' "$SBIN_F" | grep -qxE 'F[0-9]+' || { echo "FAIL: SBIN_F (the S-BIN finding's id) is not recorded in this block — write it by dated edit the day the draft is written"; exit 1; }
printf '%s\n' "$BIN_ADR" | grep -qxE '0[0-9]{3}' || { echo "FAIL: BIN_ADR (the Binary record's four-digit id) is not recorded in this block"; exit 1; }
printf '%s\n' "$SBIN_LEDGER" | grep -qxE 'D[0-9]+-[0-9]+' || { echo "FAIL: SBIN_LEDGER (the ledger entry holding the go-ahead lines) is not recorded in this block"; exit 1; }
# A dispatch by anything this block runs — a test, an extracted checkpoint — starts a session whose transcript is a
# NEW *.jsonl under the account.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
# Both account roots are watched: D21-42 17b's ~/.claude-personal, and the default ~/.claude a stray child would use.
CFGS=""
for c in "$HOME/.claude-personal" "$HOME/.claude"; do [ -d "$c/projects" ] && CFGS="$CFGS $c/projects"; done
[ -n "$CFGS" ] || { echo "FAIL: neither ~/.claude-personal/projects nor ~/.claude/projects exists — the tripwire would compare two empty listings"; exit 1; }
sess_before=$(find $CFGS -type f -name '*.jsonl' | sort)
test -n "$sess_before" || { echo "FAIL: the account holds no session transcript — the tripwire has no baseline"; exit 1; }
# No launch record in this shell, so any nested `redkiln run` refuses before any spawn (s8h §8h.7).
[ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is set — a nested redkiln run could reach the real harness; unset it"; exit 1; }
# THE NO-DISPATCH WRAPPER, D21-47's shared helper runbook/tools/nodispatch.sh, round every process-level cargo run
# below AND every replay in (11). nd_setup compiles the fake `claude` (a claude.exe, because a Windows process asked for
# `claude` finds `claude.exe` and never a shell script), puts it first on PATH, drops every other PATH directory
# holding a claude, points CLAUDE_CONFIG_DIR at an empty trap in the D:/… form a Windows process resolves, and proves
# each part by a control; each `nodispatch` child also has REDKILN_LAUNCH_RECORD unset. A nested 7f/7h/7i run's
# harness is target/debug/rk-fixture-child-claude through `[adapters] harness_binary`, which its re-running stage sets
# (s8a, s8d-ii, s8e, s8f) and this block does not.
test -f runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh is absent — D21-47's shared wrapper"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh"; exit 1; }
nd_setup "$PWD/target/cp8i/nd" || exit 1
# THE FLOOR's base (guard (10)). Expected value, re-derived at stage open from Checkpoint 8h's recorded count: ~~1243~~
# ~~(740 at 992d257, ADR-0164's evidence, + 9 + 25 + 78 + 62 + 50 + 74 + 85 + 66 + 54 over 8a..8h)~~, so the floor is
# ~~1243~~ 1404 + 6 + NFIELDS + NGATE (NGATE is set in (8): 3 on the human-gate route, 0 on the reviewer route; D21-48 X16).
# Edit these two values by a dated edit before the first run; 44 is the result-line floor.
# (Final consistency pass 2026-09-24: was 1168, on sibling counts since revised. D21-48 floor propagation,
# 2026-09-24: was 1222, before s8c 78, s8d 62, s8d-ii 50, s8f 85 and s8h 54.)
# *(dated 2026-10-02, s8i build)* Re-derived from the RECORDED count, not the drafted chain: PROGRESS.md row 8h and D21-138 record
# Checkpoint 8h passed WHOLE at d24d15a, 1404 passed / 0 failed / 9 ignored over 44 result lines. The drafted 1243
# (and the older 1168 and 1222) is struck: FLOOR_8H_PASSED=1404, FLOOR_8H_RESULTS=44.
FLOOR_8H_PASSED=1404
FLOOR_8H_RESULTS=44

# ---- (1) BUILD-OR-FAIL.
out=$(nodispatch cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8i NOT RUN (Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "(1) build" || exit 1

# ---- (2) THE FINDING DRAFT AND ITS PINS, by the id recorded in (0).
DRAFT="spikes/$SBIN_F-DRAFT.md"
test -f "$DRAFT" || { echo "FAIL: $DRAFT does not exist — the id recorded in (0) names no draft"; exit 1; }
grep -qE "^## $SBIN_F — S-BIN" "$DRAFT" || { echo "FAIL: $DRAFT is not headed '## $SBIN_F — S-BIN …'"; exit 1; }
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
# (dated 2026-10-03, s8i build, R7FIX-8i; s8i refuting round 7's minor) results/s-bin/oot-shadow-rulings.jsonl is required too, always (an empty
# file when no record is flagged; §8i.7), so the rulings that decide which records are scored are pinned like the records.
for req in results/s-bin.jsonl results/s-bin/fixture.json results/s-bin/driver-tests.txt results/s-bin/dry-run.jsonl results/s-bin/audit.txt results/s-bin/pin-report-2026-09-05/index.json results/s-bin/captures/procedure-log.txt results/s-bin/probes.jsonl results/s-bin/missing-fields.jsonl results/s-bin/brittongames-snapshots.txt runner/src/sbin.rs results/audit-s.py results/s-bin/oot-shadow-rulings.jsonl; do
  printf '%s\n' "$files" | grep -q "^$req sha256=" || { echo "FAIL: the pins block does not pin $req"; exit 1; }
done
# (dated 2026-10-06, s8i build, D21149-8i-A) The items whose every asset capture_scene_preview rejected (D21-149; the
# procedure log's n/a lines at corpus cc7f222).
SBIN_NOCAP="2 3"
for n in 1 2 3 4 5; do
  for req in "results/s-bin/greenness/item-$n/index.json" "results/s-bin/rendered/item-$n.txt"; do
    printf '%s\n' "$files" | grep -q "^$req sha256=" || { echo "FAIL: the pins block does not pin $req"; exit 1; }
  done
  # (dated 2026-10-06, s8i build, D21149-8i-A) D21-149: capture_scene_preview rejected every asset of items 2 and 3 (a
  # World; a Blueprint class and a data asset), measured in B1, so they carry no capture and none is pinned; items 1, 4
  # and 5 each pin theirs. caplog_ok below ties this to the procedure log and fixture.json. Was, for every item:
  # printf '%s\n' "$files" | grep -qE "^results/s-bin/captures/item-$n/[^ ]+ sha256=" || { echo "FAIL: no capture pinned for item $n — D21-43 N19 captures all five trees"; exit 1; }
  case " $SBIN_NOCAP " in
    *" $n "*) printf '%s\n' "$files" | grep -qE "^results/s-bin/captures/item-$n/" && { echo "FAIL: a capture is pinned for item $n, every one of whose assets capture_scene_preview rejected (D21-149)"; exit 1; } ;;
    *) printf '%s\n' "$files" | grep -qE "^results/s-bin/captures/item-$n/[^ ]+ sha256=" || { echo "FAIL: no capture pinned for item $n — D21-43 N19; only items $SBIN_NOCAP carry none (D21-149)"; exit 1; } ;;
  esac
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

# ---- (4) THE RECORDS SAY WHAT THE PINS SAY — and the pins' rule outcomes are COMPUTED here, never trusted. One valid
# ---- record per (pass, item), one version, one model, `none`, METHOD-S §0's pre-flight fields and the per-dispatch
# ---- copy checks on EVERY valid record, a dry run refused rather than filtered, the caps held. JSON is parsed.
for f in s-bin.jsonl s-bin/fixture.json s-bin/probes.jsonl s-bin/missing-fields.jsonl s-bin/captures/procedure-log.txt s-bin/dry-run.jsonl s-bin/brittongames-snapshots.txt s-bin/audit.txt audit-s.py; do
  b=$(printf '%s' "$f" | tr '/' '_')
  git -C "$CORPUS_REPO" cat-file blob "$SBIN_CORPUS:results/$f" > "$T/$b"; rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: results/$f is not in corpus commit $SBIN_CORPUS"; exit 1; }
done
[ -s "$T/s-bin.jsonl" ] && [ -s "$T/s-bin_fixture.json" ] && [ -s "$T/s-bin_captures_procedure-log.txt" ] || { echo "FAIL: the records, the fixture or the procedure log is empty at the pinned commit"; exit 1; }
# (dated 2026-10-03, s8i build, R5FIX-8i) The shadow hold (D21-144): results/s-bin/oot-shadow-rulings.jsonl holds the operator's ruling on each
# shadow-flagged record, one line {"raw_ref": <its raw_ref>, "ruling": "clear" | "void"}. Absent means no ruling
# yet, so every flagged record is held unscored. The audit reads it as its third argument.
# (dated 2026-10-03, s8i build, R7FIX-8i) (3) now requires it pinned, so a set that passed (3) never takes the empty-file branch below.
if git -C "$CORPUS_REPO" cat-file -e "$SBIN_CORPUS:results/s-bin/oot-shadow-rulings.jsonl" 2>/dev/null; then
  git -C "$CORPUS_REPO" cat-file blob "$SBIN_CORPUS:results/s-bin/oot-shadow-rulings.jsonl" > "$T/s-bin_oot-shadow-rulings.jsonl" || { echo "FAIL: results/s-bin/oot-shadow-rulings.jsonl is unreadable at $SBIN_CORPUS"; exit 1; }
else
  : > "$T/s-bin_oot-shadow-rulings.jsonl" || exit 1
fi
# THE RULE-OUTCOME COMPUTATION (§8i.8's mapping), ONE snippet shared by the record probe below and by its synthetic
# record sets, so that the controls exercise the code that scores the real records and not a copy of it. It reads the
# globals V (the valid records) and mfm (the named missing fields). D21-48 X3: exactly one seed missed (after any B4
# re-run), fields cited, no missing field named is `undecided-partial`; nothing computes as `unregistered`.
outc='
def rec(p,i,rerun=False):
    c=[r for r in V if int(r["pass"])==p and int(r["item"])==i and bool(r.get("rerun"))==rerun]
    return c[0] if len(c)==1 else None
def quoted(r, fields):
    txt=" ".join(str(f.get(k,"")) for f in ((r.get("digest") or {}).get("findings") or []) for k in ("claim","repro"))
    return all(f.get("field") and f.get("quote") and f["quote"] in txt for f in fields)
def outcome(p):
    S=(4,5); C=(1,2,3)
    if any(rec(p,i) is None for i in S+C): return "BAD-record"
    det={i for i in S if rec(p,i).get("detected") is True}
    fp=sum(1 for i in C if (rec(p,i).get("policy_verdict") or {}).get("blocks") is True)
    missed=[i for i in S if i not in det]
    named=[i for i in missed if mfm.get((p,i))]
    if any(mfm.get((p,i)) for i in S+C if i not in missed): return "BAD-named-but-not-missed"
    if {int(r["item"]) for r in V if int(r["pass"])==p and r.get("rerun")}!=set(named): return "BAD-reruns"
    if not missed: return "branch1" if fp<=1 else "undecided-fp"
    if not all(quoted(rec(p,i), mfm[(p,i)]) for i in named): return "BAD-quote"
    if any(rec(p,i,True) is None for i in named): return "BAD-rerun-record"
    still=[i for i in missed if not (i in named and rec(p,i,True).get("detected") is True)]
    if not still: return "branch2-detected" if fp<=1 else "undecided-fp"
    if any(i in named for i in still): return "branch3"
    if any(not rec(p,i).get("cited_fields") for i in still): return "undecided-silent"
    if len(still)==2: return "branch3"
    if len(still)==1: return "undecided-partial"
    return "BAD-shape"
'
# The synthetic record sets: one pass, clean items 1-3 and seeds 4-5, each set built to reach one outcome. Every one of
# the six registered values is reached, `undecided-partial` among them, and branch 3 is reached both ways (both seeds
# missed; a named field whose B4 re-run missed). A computation that drifted back to `unregistered` fails here, before
# any real record is read.
octl='
def mk(p,i,**k):
    r={"pass":p,"item":i,"void":None,"rerun":False,"detected":None,"cited_fields":["assets"],"policy_verdict":{"blocks":False},"digest":{"findings":[]}}
    r.update(k); return r
def base(): return [mk(1,i) for i in (1,2,3)]+[mk(1,i,detected=True) for i in (4,5)]
def put(rs,i,**k):
    for r in rs:
        if r["item"]==i and not r["rerun"]: r.update(k)
    return rs
q={"field":"thumbnail","quote":"no rendered view of the widget"}
said={"findings":[{"claim":"there is no rendered view of the widget","repro":""}]}
cases=[("branch1", base(), {}),
       ("undecided-fp", put(put(base(),1,policy_verdict={"blocks":True}),2,policy_verdict={"blocks":True}), {}),
       ("undecided-partial", put(base(),5,detected=False), {}),
       ("undecided-silent", put(base(),5,detected=False,cited_fields=[]), {}),
       ("branch3", put(put(base(),4,detected=False),5,detected=False), {}),
       ("branch2-detected", put(base(),5,detected=False,digest=said)+[mk(1,5,rerun=True,detected=True)], {(1,5):[q]}),
       ("branch3", put(base(),5,detected=False,digest=said)+[mk(1,5,rerun=True,detected=False)], {(1,5):[q]})]
'"$outc"'
bad=[]
for want,V,mfm in cases:
    got=outcome(1)
    if got!=want: bad.append(want+"->"+got)
seen={w for w,_,_ in cases}
print("ok" if not bad and len(seen)==6 else "BAD:"+",".join(bad)+" reached="+str(len(seen)))'
ov=$(python -c "$octl"); rc=$?
[ "$rc" -eq 0 ] && [ "$ov" = "ok" ] || { echo "FAIL: the rule-outcome computation fails its synthetic record sets (rc=$rc): $ov (D21-48 X3)"; exit 1; }
# *(dated 2026-10-03, s8i build, R1FIX-8i)* s8i refuting round 1's blocker. METHOD-S's amendment (item 2, step 5) registers
# the procedure log's sha256 as "<64 hex, one per capture file, comma-joined>", ~~and item 3 has two captures~~. The drafted
# test, whole value IN capsha(item), could not accept item 3's correct line. The log's hashes are now split on ',' and
# must equal ~~the item's pinned capture hashes~~ fixture.json's capture hashes exactly, as a multiset: each capture logged once, none extra, none
# missing. ONE snippet, shared by the record probe below and by its synthetic logs, as `outc` is.
# (dated 2026-10-06, s8i, R11FIX-8i; refuting round 11) The struck words were stale: item 3 carries captures: [] (D21-149), and caplog_ok
# compares the log against fixture.json's captures, which capfiles_ok (R10FIX-8i, below) ties to the pinned files.
# (dated 2026-10-06, s8i build, D21149-8i-A) D21-149 and D21-150. An item whose every asset capture_scene_preview
# rejected carries captures: [] in fixture.json and a binding line "sha256=none shows_subject=n/a" (items 2 and 3); a
# capture session that stalled on the instrument has its own line "sha256=none shows_subject=stalled", at most one per
# item, ahead of that item's binding line (item 5); an item's last line binds. Every line carries its model and cost,
# and capcosts hands every line's cost_usd, a stalled session's included, to the probe's cap sum. Was:
#     cl=sorted(int(d.get("item","0")) for d in plog)
#     return cl==[1,2,3,4,5] and all(d.get("shows_subject")=="yes" and d.get("model") and caps(int(d["item"])) and sorted(str(d.get("sha256","")).split(","))==caps(int(d["item"])) for d in plog)
capl='
def caplog_ok(plog, it):
    caps=lambda i: sorted(str(c.get("sha256")) for c in (it.get(i) or {}).get("captures") or [])
    by={}
    for d in plog:
        by.setdefault(str(d.get("item","")),[]).append(d)
    if sorted(by)!=["1","2","3","4","5"]:
        return False
    for k,ls in by.items():
        i=int(k); pre=ls[:-1]; b=ls[-1]
        if len(pre)>1 or not all(d.get("shows_subject")=="stalled" and d.get("sha256")=="none" and d.get("model") and d.get("cost_usd") for d in pre):
            return False
        if not (b.get("model") and b.get("cost_usd")):
            return False
        if b.get("shows_subject")=="yes":
            if not (caps(i) and sorted(str(b.get("sha256","")).split(","))==caps(i)):
                return False
        elif b.get("shows_subject")=="n/a":
            if not (b.get("sha256")=="none" and isinstance((it.get(i) or {}).get("captures"),list) and caps(i)==[]):
                return False
        else:
            return False
    return True
def capcosts(plog):
    return [d.get("cost_usd") for d in plog]
'
# (dated 2026-10-06, s8i build, D21149-8i-A) The synthetic logs now carry an n/a item (2: captures []), a two-capture
# item (3) and, in one case, a stalled line ahead of item 5's binding line; and the cost sum is controlled. The eight
# earlier cases are kept, on an item-2 n/a line. Was: it={i:{"captures":([{"sha256":A},{"sha256":B}] if i==3 else
# [{"sha256":A}])} ...}, every line "yes", eight cases, and no cost control.
cctl='
A,B,C="a"*64,"b"*64,"c"*64
def IT(over):
    return {i:{"captures":[{"sha256":h} for h in over.get(i,[A])]} for i in (1,2,3,4,5)}
it=IT({2:[],3:[A,B]})
nokey=dict(it); nokey[2]={}
def ln(i,ss="yes",sha=A,cost="0.50",model="m"):
    return {"item":str(i),"model":model,"cost_usd":cost,"shows_subject":ss,"sha256":sha}
def L(over=None,before=None,after=None):
    out=[]
    for i in (1,2,3,4,5):
        out+=(before or {}).get(i,[])
        d=ln(i,"n/a","none") if i==2 else ln(i,sha=(A+","+B if i==3 else A))
        d.update((over or {}).get(i,{}))
        out.append(d)
    return out+(after or [])
st=lambda i: ln(i,"stalled","none")
cases=[("ok",L(),it),("ok",L({3:{"sha256":B+","+A}}),it),("ok",L(before={5:[st(5)]}),it),
       ("BAD",L({3:{"sha256":A}}),it),("BAD",L({3:{"sha256":A+","+B+","+C}}),it),("BAD",L({3:{"sha256":A+","+A}}),it),
       ("BAD",L({1:{"shows_subject":"no"}}),it),("BAD",L({4:{"sha256":C}}),it),("BAD",L()[:4],it),
       ("BAD",L({2:{"sha256":A}}),it),("BAD",L({2:{"shows_subject":"yes"}}),it),("BAD",L({1:{"shows_subject":"n/a","sha256":"none"}}),it),("BAD",L(),nokey),
       ("BAD",L(before={5:[st(5),st(5)]}),it),("BAD",L(after=[st(5)]),it),("BAD",L(before={5:[ln(5)]}),it),
       ("BAD",L(before={5:[ln(5,"stalled",A)]}),it),("BAD",L(before={5:[ln(5,"stalled","none",model="")]}),it),("BAD",L(before={5:[ln(5,"stalled","none",cost=None)]}),it),
       ("BAD",L({4:{"model":""}}),it),("BAD",L({4:{"cost_usd":None}}),it)]
'"$capl"'
bad=[str(n)+":"+w for n,(w,pl,t) in enumerate(cases) if ("ok" if caplog_ok(pl,t) else "BAD")!=w]
cs=capcosts(L(before={5:[st(5)]}))
if not (len(cs)==6 and abs(sum(float(c) for c in cs)-3.0)<1e-9):
    bad.append("costsum")
if None not in capcosts(L({4:{"cost_usd":None}})):
    bad.append("costnull")
print("ok" if not bad else "BAD:"+",".join(bad))'
clv=$(python -c "$cctl"); rc=$?
[ "$rc" -eq 0 ] && [ "$clv" = "ok" ] || { echo "FAIL: the procedure-log check fails its synthetic logs (rc=$rc): $clv — a two-capture item, an n/a item with captures [] and one stalled line ahead of a binding line pass; a missing, extra, repeated or foreign hash, an n/a line with a hash or for a captured item, a second or trailing stalled line, a stalled line with a hash, and a line without its model or cost do not; every line's cost is summed (METHOD-S amendment item 2; D21-149; D21-150)"; exit 1; }
# (dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's major) (3) checked each pinned file against its own
# corpus blob, and caplog_ok checked the procedure log against fixture.json, but nothing tied the pinned capture files
# to fixture.json's captures: a capture pinned with bytes X while fixture.json and the log both said Y, or one extra
# pinned capture for a captured item, passed (3) and (4). capfiles_ok ties them, ONE snippet shared by the record probe
# below and by its synthetic pins, as `capl` is. For every item 1-5, the pins block's
# results/s-bin/captures/item-<n>/<name> lines, as (sha256, <name>) pairs, must equal fixture.json's captures as
# (sha256, the basename of path) pairs, as a multiset: so the sorted pinned hashes equal caps(i) and the pinned
# basenames equal the captures' path basenames. An item with captures [] pins no capture file; an item with no
# captures list, a capture pinned twice, and a pinned capture path that is not item-<1..5>/<name> fail. With caplog_ok
# (the log's binding line against caps(i)) and (3) (each pinned file against its own blob), the procedure log,
# fixture.json and the committed capture bytes are one set.
capf='
CAPDIR="results/s-bin/captures/"
def capfiles_ok(lines, it):
    pinned={}
    for l in lines:
        p,sep,h=l.strip().partition(" sha256=")
        if not p.startswith(CAPDIR+"item-"):
            continue
        parts=p[len(CAPDIR):].split("/")
        if len(parts)!=2 or parts[0] not in ("item-1","item-2","item-3","item-4","item-5") or not parts[1] or not sep or not h:
            return False
        pinned.setdefault(int(parts[0][5:]),[]).append((h,parts[1]))
    for i in (1,2,3,4,5):
        cs=(it.get(i) or {}).get("captures")
        if not isinstance(cs,list):
            return False
        want=sorted((str(c.get("sha256")),str(c.get("path","")).replace(chr(92),"/").split("/")[-1]) for c in cs)
        if sorted(pinned.get(i,[]))!=want:
            return False
    return True
'
# The synthetic pins: the real shape (items 1, 4 and 5 one capture-1.png each, items 2 and 3 captures [], beside the
# procedure log's and other files' pins) reads ok, in any line order, and so does a two-capture item. BAD: one pinned
# hash differing from fixture.json; one extra pinned file for a captured item; a pinned file for a captures-[] item; a
# captured item with no pinned file; the same capture pinned twice; a pinned basename differing from the captures'
# path; two items' hashes swapped; a two-capture item with one pinned; an item-6 path; a nested path; an item with no
# capture pinned whose captures list is absent (item 2), which only the list check catches.
pctl='
H={i:str(i)*64 for i in (1,2,3,4,5)}
X="f"*64
def cap(h,k=1): return {"asset_index":k,"path":"_s-bin/captures/capture-%d.png"%k,"sha256":h}
def IT(over): return {i:{"captures":over.get(i,[cap(H[i])])} for i in (1,2,3,4,5)}
it=IT({2:[],3:[]})
two=IT({2:[],3:[cap(H[3],1),cap(X,2)]})
nokey=dict(it); nokey[2]={}
def F(i,name,h): return "results/s-bin/captures/item-%d/%s sha256=%s"%(i,name,h)
OTHER=["results/s-bin/captures/procedure-log.txt sha256="+X,"results/s-bin.jsonl sha256="+X]
real=OTHER+[F(i,"capture-1.png",H[i]) for i in (1,4,5)]
cases=[("ok",real,it),("ok",[F(i,"capture-1.png",H[i]) for i in (5,4,1)]+OTHER,it),
       ("ok",real+[F(3,"capture-2.png",X),F(3,"capture-1.png",H[3])],two),
       ("BAD",[l.replace(H[4],X) for l in real],it),
       ("BAD",real+[F(1,"capture-2.png",X)],it),
       ("BAD",real+[F(2,"capture-1.png",H[2])],it),
       ("BAD",[l for l in real if "/item-5/" not in l],it),
       ("BAD",real+[F(1,"capture-1.png",H[1])],it),
       ("BAD",[l.replace("item-4/capture-1.png","item-4/capture-2.png") for l in real],it),
       ("BAD",OTHER+[F(1,"capture-1.png",H[4]),F(4,"capture-1.png",H[1]),F(5,"capture-1.png",H[5])],it),
       ("BAD",real+[F(3,"capture-1.png",H[3])],two),
       ("BAD",real+[F(6,"capture-1.png",X)],it),
       ("BAD",real+["results/s-bin/captures/item-1/sub/capture-1.png sha256="+H[1]],it),
       ("BAD",real,nokey)]
'"$capf"'
bad=[str(n)+":"+w for n,(w,pl,t) in enumerate(cases) if ("ok" if capfiles_ok(pl,t) else "BAD")!=w]
print("ok" if not bad else "BAD:"+",".join(bad))'
pcv=$(python -c "$pctl"); rc=$?
[ "$rc" -eq 0 ] && [ "$pcv" = "ok" ] || { echo "FAIL: the pinned-capture check fails its synthetic pins (rc=$rc): $pcv — the real shape and a two-capture item pass; a pinned hash or name that differs from fixture.json, an extra, missing, repeated or swapped capture, a capture pinned for a captures-[] item, and a path outside item-<1..5>/<name> do not (R10FIX-8i; D21-149)"; exit 1; }
# *(dated 2026-10-03, s8i build, R2FIX-8i)* s8i refuting round 2 (a major). The apiKeySource line read EVERY record, voids
# included. A `no-init` or early-timeout void carries a null source, and the driver records and retries it by design
# (§8i.3's void order; METHOD-S item 10), so one such void failed a paid, valid set for good. The line was
# print("aks", ",".join(sorted({str(r.get("api_key_source")) for r in sb}))). It now reads the VALID records, as §8i.9's
# prose scopes it ("ten valid records ... apiKeySource none"). New beside it, METHOD-S item 11: every record's
# cli_version equals its launch record's CLI_VERSION. The driver records that version as launch_cli_version (the launch
# record's CLI_VERSION_BEFORE, §8i.3's dated note). Each check is ONE snippet, shared by the record probe below and by its
# synthetic record sets, as `outc` is.
aksl='
def aks_of(sb):
    return ",".join(sorted({str(r.get("api_key_source")) for r in sb if r.get("void") is None}))
def launch_ok(sb):
    return bool(sb) and all(r.get("launch_record") and r.get("launch_cli_version") is not None and r.get("launch_cli_version")==r.get("cli_version") for r in sb)
'
akctl='
def R(void=None, aks="none", cli="2.1.288", lv="2.1.288", lr="L"):
    return {"void":void,"api_key_source":aks,"cli_version":cli,"launch_cli_version":lv,"launch_record":lr}
'"$aksl"'
cases=[("aks","ok",[R(void="no-init",aks=None),R()]),("aks","ok",[R(void="timeout",aks=None),R()]),("aks","BAD",[R(aks="ANTHROPIC_API_KEY")]),("aks","BAD",[R(),R(aks=None)]),
       ("launch","ok",[R(),R(void="no-init",aks=None)]),("launch","BAD",[R(cli="2.1.290")]),("launch","BAD",[R(lv=None,lr=None)]),("launch","BAD",[])]
def got(k,sb): return ("ok" if aks_of(sb)=="none" else "BAD") if k=="aks" else ("ok" if launch_ok(sb) else "BAD")
bad=[str(n) for n,(k,w,sb) in enumerate(cases) if got(k,sb)!=w]
print("ok" if not bad else "BAD:"+",".join(bad))'
akv=$(python -c "$akctl"); rc=$?
[ "$rc" -eq 0 ] && [ "$akv" = "ok" ] || { echo "FAIL: the apiKeySource or launch-record check fails its synthetic record sets (rc=$rc): $akv — a retried void with no source must pass; a valid record with another source, or a record whose CLI differs from its launch record's, must not (D21-42 17a; METHOD-S item 11)"; exit 1; }
# (dated 2026-10-03, s8i build, R5FIX-8i) The shadow hold, ONE snippet shared by the record probe below and by its synthetic records, as `outc`
# is. A record whose oot_shadow verdict is flagged is held until a ruling clears it (then scored) or voids it (then
# never a miss, and gone from V); every record carries a verdict (§8i.7).
holdl='
def rulings_of(rl): return {str(x.get("raw_ref")): x.get("ruling") for x in rl}
def flagged(r): return (r.get("oot_shadow") or {}).get("verdict") == "flagged"
def held_of(sb, ru): return [r for r in sb if r.get("void") is None and flagged(r) and ru.get(str(r.get("raw_ref"))) not in ("clear", "void")]
def valid_of(sb, ru): return [r for r in sb if r.get("void") is None and not (flagged(r) and ru.get(str(r.get("raw_ref"))) != "clear")]
def shadow_ok(sb): return all((r.get("oot_shadow") or {}).get("verdict") in ("clear", "flagged") for r in sb)
'
hctl='
def R(ref, v="clear", void=None): return {"raw_ref": ref, "void": void, "oot_shadow": {"verdict": v}}
'"$holdl"'
sb=[R("a"), R("b", "flagged"), R("c", "flagged"), R("d", "flagged"), R("e", "flagged", void="timeout")]
ru=rulings_of([{"raw_ref": "c", "ruling": "clear"}, {"raw_ref": "d", "ruling": "void"}])
got=(sorted(r["raw_ref"] for r in held_of(sb, ru)), sorted(r["raw_ref"] for r in valid_of(sb, ru)), shadow_ok(sb), shadow_ok([{"raw_ref": "x"}]))
print("ok" if got == (["b"], ["a", "c"], True, False) else "BAD:" + repr(got))'
hv=$(python -c "$hctl"); rc=$?
[ "$rc" -eq 0 ] && [ "$hv" = "ok" ] || { echo "FAIL: the shadow-hold check fails its synthetic records (rc=$rc): $hv — a flagged record with no ruling must be held, a cleared one scored, an operator void dropped (D21-144)"; exit 1; }
probe='import json,sys,os,re
def jl(p): return [json.loads(l) for l in open(p,encoding="utf-8") if l.strip()]
recs=jl(sys.argv[1]); fx=json.load(open(sys.argv[2],encoding="utf-8")); probes=jl(sys.argv[3]); mf=jl(sys.argv[4])
plog=[dict(t.split("=",1) for t in l.split()) for l in open(sys.argv[5],encoding="utf-8").read().splitlines() if l.strip()]
dry=jl(sys.argv[6])
'"$holdl"'
ru=rulings_of(jl(sys.argv[7]))
it={int(k):v for k,v in fx["items"].items()}
print("items", ",".join(str(i) for i in sorted(it)))
sets={tuple(sorted(v["stripped"])) for v in it.values()}
print("stripped", "ok" if len(sets)==1 and len(next(iter(sets)))>0 else "BAD")
print("leaks", sum(int(v["leak_hits"]) for v in it.values()))
print("caps", ",".join(str(i) for i in sorted(it) if it[i].get("captures")))
print("capsnone", ",".join(str(i) for i in sorted(it) if it[i].get("captures") == []))
print("dry", "ok" if dry and all(r.get("dry_run") is True for r in dry) and all(r.get("dry_run") is False for r in recs) else "BAD")
sb=[r for r in recs if r.get("spike")=="s-bin"]
print("foreign", len(recs)-len(sb))
# (dated 2026-10-03, s8i build, R5FIX-8i) V excludes a flagged record that is held or operator-voided (D21-144). It read: V = every record with a null void.
V=valid_of(sb, ru)
print("held", len(held_of(sb, ru)))
print("shadow", "ok" if shadow_ok(sb) else "BAD")
ok=[r for r in V if not r.get("rerun")]
keys=sorted((int(r["pass"]),int(r["item"])) for r in ok)
print("pairs", "ok" if keys==[(p,i) for p in (1,2) for i in (1,2,3,4,5)] else "BAD"+repr(keys))
print("cli", ",".join(sorted({str(r.get("cli_version")) for r in sb})))
print("model", ",".join(sorted({str(r.get("observed_model")) for r in sb if r.get("observed_model") is not None})))
'"$aksl"'
print("aks", aks_of(sb))
print("launch", "ok" if launch_ok(sb) else "BAD")
def every(k, want): return all(r.get(k) == want and type(r.get(k)) is type(want) for r in V)
pre = every("card_adopted", True) and all(r.get("api_error_status") is None for r in V) and every("fix_unreachable", True)
pre = pre and every("precheck_clean", True) and every("precheck_blob_oids", True) and every("precheck_leak_hits", 0) and every("precheck_no_project_dir", True)
print("preflight", "ok" if pre else "BAD")
print("seed", all(r.get("seed_blob_verified") is True for r in V if int(r["item"]) in (4,5)))
norm=lambda p: os.path.normcase(os.path.normpath(str(p)))
used=[norm(r.get("tree_path")) for r in sb]+[norm(v.get("capture_path")) for v in it.values()]
print("fresh", "ok" if all(r.get("tree_path") for r in sb) and all(v.get("capture_path") for v in it.values()) and len(set(used))==len(used) else "BAD")
print("treecommit", "ok" if all(r.get("tree_commit") and r.get("tree_commit")==it[int(r["item"])].get("review_commit") for r in sb) else "BAD")
print("runner", ",".join(sorted({str(r.get("runner_commit")) for r in sb})))
print("runbin", "ok" if all(re.fullmatch("[0-9a-f]{64}", str(r.get("runner_sha256"))) for r in sb) else "BAD")
print("ceil", ",".join(sorted({str(r.get("ceiling_secs")) for r in sb})))
print("first", min(int(r["started_at"]) for r in sb))
print("reruns", sum(1 for r in sb if r.get("rerun")))
'"$capl"'
costs=[r.get("total_cost_usd") for r in sb]+[p.get("total_cost_usd") for p in probes]+capcosts(plog)
print("costnull", sum(1 for c in costs if c is None))
total=sum(float(c) for c in costs if c is not None)
print("opus", len(sb))
print("haiku", "ok" if len(probes)<=3 and all("haiku" in str(p.get("observed_model")) for p in probes) and not any("haiku" in str(r.get("observed_model")) for r in sb) else "BAD")
print("cost", "ok" if total<=90.0 else "BAD")
print("costsum", "%.2f" % total)
print("caplog", "ok" if caplog_ok(plog, it) else "BAD")
'"$capf"'
pf=[l for l in open(sys.argv[8],encoding="utf-8").read().splitlines() if l.strip()]
print("capfiles", "ok" if capfiles_ok(pf, it) else "BAD")
mfm={(int(m["pass"]),int(m["item"])): (m.get("missing_fields") or []) for m in mf}
'"$outc"'
print("o1", outcome(1)); print("o2", outcome(2))'
# (dated 2026-10-06, s8i build, R10FIX-8i) The pins block's file= lines, as (3) read them, are the probe's eighth argument,
# for capfiles_ok. The probe took seven arguments, and never read the pinned capture lines.
printf '%s\n' "$files" > "$T/s-bin_pinfiles" || { echo "FAIL: could not write the pins block's file= lines to $T"; exit 1; }
[ -s "$T/s-bin_pinfiles" ] || { echo "FAIL: the pins block's file= lines are empty"; exit 1; }
pv=$(python -c "$probe" "$T/s-bin.jsonl" "$T/s-bin_fixture.json" "$T/s-bin_probes.jsonl" "$T/s-bin_missing-fields.jsonl" "$T/s-bin_captures_procedure-log.txt" "$T/s-bin_dry-run.jsonl" "$T/s-bin_oot-shadow-rulings.jsonl" "$T/s-bin_pinfiles"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$pv" ] || { echo "FAIL: the record probe did not run (rc=$rc)"; exit 1; }
rv() { printf '%s\n' "$pv" | sed -n "s/^$1 //p"; }
[ "$(rv items)" = "1,2,3,4,5" ] || { echo "FAIL: fixture.json does not hold exactly items 1-5: '$(rv items)'"; exit 1; }
[ "$(rv stripped)" = "ok" ] || { echo "FAIL: the stripped path set is not one non-empty set across all five trees (D21-43 N20)"; exit 1; }
[ "$(rv leaks)" = "0" ] || { echo "FAIL: $(rv leaks) registered leak-token hits remain in the stripped trees (§8i.2 item 3)"; exit 1; }
# (dated 2026-10-06, s8i build, D21149-8i-A) D21-149: items 2 and 3 carry captures: [] (SBIN_NOCAP), the others their
# captures. The probe's capl include moved above its cost line, so the cap sum reads every procedure-log line through
# capcosts (D21-150 item 3); it stood above the caplog line, and the cost line read [d.get("cost_usd") for d in plog].
# Was: [ "$(rv caps)" = "1,2,3,4,5" ] || { echo "FAIL: captures recorded for items '$(rv caps)', not all five (D21-43 N19)"; exit 1; }
[ "$(rv caps)" = "1,4,5" ] && [ "$(rv capsnone)" = "$(printf '%s' "$SBIN_NOCAP" | tr ' ' ',')" ] || { echo "FAIL: captures recorded for items '$(rv caps)', captures [] for '$(rv capsnone)' — items $SBIN_NOCAP carry none (D21-149), every other item its own (D21-43 N19)"; exit 1; }
[ "$(rv dry)" = "ok" ] || { echo "FAIL: a record in results/s-bin.jsonl is not dry_run false, or the dry-run file holds no dry run only — a dry run in the set is refused, never filtered"; exit 1; }
[ "$(rv foreign)" = "0" ] || { echo "FAIL: $(rv foreign) records in results/s-bin.jsonl are not spike s-bin"; exit 1; }
# (dated 2026-10-03, s8i build, R5FIX-8i) The shadow hold: no flagged record is left unruled, and every record carries its verdict.
# (dated 2026-10-03, s8i build, R6FIX-8i; s8i refuting round 6) These two lines moved here, above the pairs line; they
# stood after the caplog line. V (valid_of) drops a held record, so a set holding one failed first as "not exactly one
# valid record per (pass, item)" and the hold message never printed, though the driver's resume treats that pair as done.
[ "$(rv shadow)" = "ok" ] || { echo "FAIL: a record carries no oot_shadow verdict (§8i.7, R5FIX-8i)"; exit 1; }
[ "$(rv held)" = "0" ] || { echo "FAIL: $(rv held) shadow-flagged records are held unscored — the operator clears or voids each in results/s-bin/oot-shadow-rulings.jsonl before scoring (D21-144)"; exit 1; }
[ "$(rv pairs)" = "ok" ] || { echo "FAIL: not exactly one valid record per (pass, item): $(rv pairs)"; exit 1; }
[ "$(rv cli)" = "$SBIN_CLI" ] || { echo "FAIL: the records span CLI versions '$(rv cli)', pinned $SBIN_CLI (Halt 2)"; exit 1; }
[ "$(rv model)" = "$SBIN_MODEL" ] || { echo "FAIL: the records report models '$(rv model)', pinned $SBIN_MODEL — one observed id (§8i.2 item 5)"; exit 1; }
[ "$(rv aks)" = "none" ] || { echo "FAIL: apiKeySource '$(rv aks)' on a valid S-BIN record (D21-42 17a)"; exit 1; }
[ "$(rv launch)" = "ok" ] || { echo "FAIL: a record names no launch record, or its cli_version is not its launch record's CLI_VERSION (METHOD-S item 11; §8i.5)"; exit 1; }
[ "$(rv preflight)" = "ok" ] || { echo "FAIL: a valid record lacks card_adopted true, carries an api_error_status, has the fix reachable, or fails a pre-spawn copy check (METHOD-S §0; §8i.4)"; exit 1; }
[ "$(rv seed)" = "True" ] || { echo "FAIL: a valid seeded record with its seed unverified by blob oid"; exit 1; }
[ "$(rv fresh)" = "ok" ] || { echo "FAIL: a dispatch copy's path was used twice, or is a capture copy's — no tree is reused (§8i.4)"; exit 1; }
[ "$(rv treecommit)" = "ok" ] || { echo "FAIL: a record's tree_commit is not its item's review_commit (§8i.4)"; exit 1; }
[ "$(rv runbin)" = "ok" ] || { echo "FAIL: a record carries no runner_sha256 (§8i.7)"; exit 1; }
[ "$(rv ceil)" = "$SBIN_CEIL" ] || { echo "FAIL: the records ran under ceilings '$(rv ceil)', pinned $SBIN_CEIL"; exit 1; }
[ "$(rv costnull)" = "0" ] || { echo "FAIL: $(rv costnull) records, probes or capture lines carry no cost — a missing cost is never read as zero (§8i.5)"; exit 1; }
[ "$(rv opus)" -le 18 ] && [ "$(rv haiku)" = "ok" ] && [ "$(rv cost)" = "ok" ] || { echo "FAIL: a cap was exceeded — $(rv opus) Opus dispatches, haiku $(rv haiku), \$$(rv costsum) over records, probes and capture sessions (§8i.5)"; exit 1; }
# (dated 2026-10-06, s8i build, D21149-8i-A) The message names D21-149's n/a and D21-150's stalled lines. Was: "the
# procedure log does not hold one 'yes' line per item with its model, cost and the pinned capture's sha256 (§8i.4)".
[ "$(rv caplog)" = "ok" ] || { echo "FAIL: the procedure log does not end each item on its binding line — 'yes' with exactly the item's capture hashes, or 'n/a' with sha256=none for an item with captures [] — with its model and cost, behind at most one 'stalled' line (§8i.4; D21-149; D21-150)"; exit 1; }
# (dated 2026-10-06, s8i build, R10FIX-8i; s8i refuting round 10's major) The pinned capture files are fixture.json's captures.
[ "$(rv capfiles)" = "ok" ] || { echo "FAIL: the pinned capture files are not fixture.json's captures — for each item, the pins block's results/s-bin/captures/item-<n>/<name> lines must be exactly its captures, sha256 and path basename, each once, and an item with captures [] pins none (§8i.4 step 5; R10FIX-8i)"; exit 1; }
# (dated 2026-10-03, s8i build, R6FIX-8i) The shadow and held lines that stood here moved above the pairs line (see there).
# The runner that dispatched: one commit, on the pinned commit's history, with the same runner/ tree it pins.
RUNNER_C=$(rv runner)
printf '%s\n' "$RUNNER_C" | grep -qxE '[0-9a-f]{40}' || { echo "FAIL: the records name runner commits '$RUNNER_C', not exactly one (§8i.7)"; exit 1; }
git -C "$CORPUS_REPO" merge-base --is-ancestor "$RUNNER_C" "$SBIN_CORPUS" || { echo "FAIL: the dispatching runner $RUNNER_C is not an ancestor of $SBIN_CORPUS"; exit 1; }
t_then=$(git -C "$CORPUS_REPO" rev-parse "$RUNNER_C:runner"); rc=$?
t_now=$(git -C "$CORPUS_REPO" rev-parse "$SBIN_CORPUS:runner"); rc2=$?
[ "$rc" -eq 0 ] && [ "$rc2" -eq 0 ] && [ -n "$t_then" ] && [ "$t_then" = "$t_now" ] || { echo "FAIL: runner/ at the dispatching commit is not runner/ at the pinned commit — (6) would test other code"; exit 1; }
# The rule outcomes: computed above from the records and missing-fields.jsonl, re-computed by the PINNED audit over
# the same records, and compared with the pins. Three sources must agree; the pins alone decide nothing.
O1=$(rv o1); O2=$(rv o2)
for o in "$O1" "$O2"; do case "$o" in branch1|branch2-detected|branch3|undecided-fp|undecided-silent|undecided-partial) : ;; *BAD*) : ;; *) echo "FAIL: a computed outcome '$o' is not one of the six registered values (§8i.2 item 9; D21-48 X3)"; exit 1;; esac; done
case "$O1 $O2" in *BAD*) echo "FAIL: the records cannot be scored: pass 1 $O1, pass 2 $O2"; exit 1;; esac
[ "$O1" = "$SBIN_O1" ] && [ "$O2" = "$SBIN_O2" ] || { echo "FAIL: the pins say ($SBIN_O1, $SBIN_O2); the records give ($O1, $O2) (§8i.8)"; exit 1; }
# (dated 2026-10-03, s8i build, R5FIX-8i) The pinned audit reads the rulings too, as its third argument.
python "$T/audit-s.py" bin "$T/s-bin.jsonl" "$T/s-bin_missing-fields.jsonl" "$T/s-bin_oot-shadow-rulings.jsonl" > "$T/audit-now.txt" 2> "$T/audit-err.txt"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/audit-now.txt" ] || { echo "FAIL: the pinned audit-s.py bin did not run over the pinned records (rc=$rc)"; tail -10 "$T/audit-err.txt"; exit 1; }
tr -d '\r' < "$T/audit-now.txt" > "$T/audit-now.lf" && tr -d '\r' < "$T/s-bin_audit.txt" > "$T/audit-pin.lf" || { echo "FAIL: could not normalise the two audits"; exit 1; }
cmp -s "$T/audit-now.lf" "$T/audit-pin.lf" || { echo "FAIL: the pinned audit over the pinned records does not reproduce the pinned audit.txt"; exit 1; }
grep -qx "rule_outcome_pass1=$O1" "$T/audit-now.lf" && grep -qx "rule_outcome_pass2=$O2" "$T/audit-now.lf" || { echo "FAIL: the audit's rule_outcome lines do not give ($O1, $O2)"; exit 1; }

# ---- (5) THE GO-AHEADS, THE AMENDMENT, AND METHOD-S APPEND-ONLY FROM THE AMENDMENT'S PARENT ON.
acs=$(git log --reverse --format='%H %ct' -S 's8i pre-dispatch amendment' -- spikes/METHOD-S.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$acs" ] || { echo "FAIL: no commit introduced the s8i pre-dispatch amendment (§8i.2)"; exit 1; }
set -- $acs; ACOMMIT=$1; ATS=$2
[ "$ATS" -lt "$(rv first)" ] || { echo "FAIL: the first S-BIN record ($(rv first)) does not post-date the amendment's commit ($ATS)"; exit 1; }
# Append-only, compared against the amendment's PARENT (an edit made in the amendment's own commit is visible) and
# over the whole file (§0-§6 and every earlier §7 entry), then the amendment against HEAD, then HEAD against the tree.
git show "$ACOMMIT^:spikes/METHOD-S.md" > "$T/ms-parent.md" && git show "$ACOMMIT:spikes/METHOD-S.md" > "$T/ms-amend.md" && git show "HEAD:spikes/METHOD-S.md" > "$T/ms-head.md"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: METHOD-S.md unreadable at $ACOMMIT^, $ACOMMIT or HEAD"; exit 1; }
ap='import sys
a,b,c,d=[open(p,"rb").read() for p in sys.argv[1:5]]
print("amend", "ok" if b.startswith(a) and b"s8i pre-dispatch amendment" in b[len(a):] else "BAD")
print("head", "ok" if c.startswith(b) else "BAD")
print("tree", "ok" if d.startswith(c) else "BAD")'
av=$(python -c "$ap" "$T/ms-parent.md" "$T/ms-amend.md" "$T/ms-head.md" spikes/METHOD-S.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$av" ] || { echo "FAIL: the append-only check did not run (rc=$rc)"; exit 1; }
for k in amend head tree; do
  printf '%s\n' "$av" | grep -qx "$k ok" || { echo "FAIL: METHOD-S.md is not append-only at '$k' — the registered method moved (§8i.2)"; printf '%s\n' "$av"; exit 1; }
done
printf '## 0\nA\n' > "$T/c-parent"; printf '## 0\nB\ns8i pre-dispatch amendment\n' > "$T/c-amend"
cv=$(python -c "$ap" "$T/c-parent" "$T/c-amend" "$T/c-amend" "$T/c-amend"); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$cv" | grep -qx 'amend BAD' || { echo "FAIL: the append-only check cannot see a §0 edit made in the amendment's own commit — it is vacuous"; exit 1; }
# The go-ahead lines, from the ledger entry recorded in (0) only.
awk -v h="## $SBIN_LEDGER " 'FNR==1{p=0} index($0,h)==1{p=1;print;next} p&&/^## /{p=0} p' spikes/DECISIONS-*.md > "$T/ledger"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/ledger" ] || { echo "FAIL: no ledger entry headed '## $SBIN_LEDGER ' in spikes/DECISIONS-*.md"; exit 1; }
grep -oE 'S-BIN go-ahead B[0-4] .*' "$T/ledger" > "$T/goahead"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the ledger entry $SBIN_LEDGER holds no S-BIN go-ahead line (§8i.5)"; exit 1; }
ga='import json,sys,re,datetime
L=[l for l in open(sys.argv[1],encoding="utf-8").read().splitlines() if l.strip()]
recs=[json.loads(l) for l in open(sys.argv[2],encoding="utf-8") if l.strip()]
ats=int(sys.argv[3]); g={}
for l in L:
    m=re.match("S-BIN go-ahead (B[0-4]) ([0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z) ", l)
    if not m:
        print("lines BADFORM"); sys.exit(0)
    t=datetime.datetime.strptime(m.group(2),"%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=datetime.timezone.utc).timestamp()
    g.setdefault(m.group(1),[]).append(t)
sb=[r for r in recs if r.get("spike")=="s-bin"]
p1=[r for r in sb if int(r["pass"])==1 and not r.get("rerun")]
p2=[r for r in sb if int(r["pass"])==2 and not r.get("rerun")]
rr=[r for r in sb if r.get("rerun")]
need=["B0","B1","B2","B3"]+(["B4"] if rr else [])
bad=[b for b in ("B0","B1","B2","B3","B4") if len(g.get(b,[]))!=(1 if b in need else 0)]
print("lines", "ok" if not bad else "BAD:"+",".join(bad))
if bad: sys.exit(0)
T={b:g[b][0] for b in need}
start=lambda rs: min(int(r["started_at"]) for r in rs)
end=lambda rs: max(int(r["started_at"])+int(r.get("wall_secs") or 0) for r in rs)
ok=T["B0"] < T["B1"] and ats < T["B1"] < T["B2"] < start(p1) and end(p1) < T["B3"] < start(p2) and (not rr or T["B4"] < start(rr))
print("order", "ok" if ok else "BAD")'
gv=$(python -c "$ga" "$T/goahead" "$T/s-bin.jsonl" "$ATS"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$gv" ] || { echo "FAIL: the go-ahead probe did not run (rc=$rc)"; exit 1; }
[ "$(printf '%s\n' "$gv" | sed -n 's/^lines //p')" = "ok" ] || { echo "FAIL: the go-ahead lines are not one per batch in the fixed form (B4 iff a re-run): $gv"; exit 1; }
[ "$(printf '%s\n' "$gv" | sed -n 's/^order //p')" = "ok" ] || { echo "FAIL: the go-ahead times do not precede their batches' records in order, or B1 precedes the amendment's commit (§8i.5)"; exit 1; }
# D21-43 N22: the gate (Checkpoint 8h plus the zero-spend dry run) is "confirmed on the B1 line", by hashes that resolve.
DRY_SHA=$(printf '%s\n' "$files" | sed -n 's/^results\/s-bin\/dry-run\.jsonl sha256=//p')
b1=$(grep -E '^S-BIN go-ahead B1 ' "$T/goahead"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$b1" ] && [ -n "$DRY_SHA" ] || { echo "FAIL: no B1 go-ahead line (rc=$rc), or no pinned dry run"; exit 1; }
H8=$(printf '%s\n' "$b1" | sed -n 's/.*fixture proofs: Checkpoint 8h \([0-9a-f]\{7,40\}\), .*/\1/p')
[ -n "$H8" ] && git rev-parse --verify --quiet "$H8^{commit}" >/dev/null || { echo "FAIL: the B1 line's Checkpoint 8h hash '$H8' does not resolve to a commit (D21-43 N22)"; exit 1; }
git merge-base --is-ancestor "$H8" HEAD || { echo "FAIL: Checkpoint 8h's $H8 is not an ancestor of HEAD"; exit 1; }
row8h=$(grep -E '^[|] 8h[^0-9a-z]' runbook/PROGRESS.md); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$row8h" | grep -qF "$H8" || { echo "FAIL: PROGRESS.md row 8h does not record $H8 as Checkpoint 8h's pass"; exit 1; }
printf '%s\n' "$b1" | grep -qF "dry run $DRY_SHA" || { echo "FAIL: the B1 line does not name the pinned dry run's sha256 $DRY_SHA (D21-43 N22)"; exit 1; }
B1A=$(printf '%s\n' "$b1" | sed -n 's/.*; amendment \([0-9a-f]\{40\}\).*/\1/p')
[ "$B1A" = "$ACOMMIT" ] || { echo "FAIL: the B1 line names amendment '$B1A'; the commit that introduced it is $ACOMMIT"; exit 1; }

# ---- (6) THE DRIVER'S TESTS, RE-RUN AT THE PINNED CORPUS COMMIT — rebuilt from `git archive` in a directory outside
# ---- this workspace, so the corpus's own working tree is never read or written, under the no-dispatch wrapper.
R=$(mktemp -d); rc=$?
[ "$rc" -eq 0 ] && [ -d "$R" ] || { echo "FAIL: no scratch directory for the runner (rc=$rc)"; exit 1; }
git -C "$CORPUS_REPO" archive --format=tar "$SBIN_CORPUS" runner > "$R/runner.tar"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git archive of runner/ at $SBIN_CORPUS (rc=$rc)"; exit 1; }
tar -xf "$R/runner.tar" -C "$R"; rc=$?
[ "$rc" -eq 0 ] && [ -f "$R/runner/src/sbin.rs" ] || { echo "FAIL: the archived runner has no src/sbin.rs"; exit 1; }
out=$(cd "$R/runner" && nodispatch cargo build 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the runner does not build at the pinned commit (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
# (dated 2026-10-03, s8i build, R5FIX-8i) R5FIX-8i's 8 named tests are added (D21-144), so the selection runs 33. It ran 25.
# (dated 2026-10-06, s8i build, D21149-8i-A) D21-149 item 3 inverts and renames every_item_carries_a_capture_reference_or_rendering_refuses
# to captures_name_their_asset_index_and_a_rejected_asset_has_none, and D21-151 adds a_volume_under_sbin_need_bytes_refuses_before_the_copy,
# so the selection runs 34. It ran 33, with the old name.
SB_TESTS="sbin::tests::seed_is_confirmed_by_blob_oid_not_by_file_count sbin::tests::review_repo_holds_one_commit_and_no_fix_object sbin::tests::text_diff_carries_source_and_test_paths_only sbin::tests::asset_size_is_read_from_the_lfs_pointer sbin::tests::warned_and_not_run_fold_into_passed_with_the_raw_report_referenced sbin::tests::a_digest_failing_the_typed_schema_is_void_schema_not_a_miss sbin::tests::exit_zero_without_structured_output_is_void_not_success sbin::tests::strong_detection_needs_a_blocking_finding_naming_the_seeded_path sbin::tests::cited_fields_are_extracted_mechanically_from_the_digest sbin::tests::an_out_of_tree_read_voids_the_record sbin::tests::api_key_source_other_than_none_voids_before_scoring sbin::tests::a_version_move_between_dispatches_halts_the_set sbin::tests::the_dispatch_cap_refuses_before_spawning sbin::tests::resume_is_keyed_per_arm_item_pass sbin::tests::a_surviving_unreal_process_halts_before_the_next_dispatch sbin::tests::stripped_path_set_is_identical_across_all_five_trees sbin::tests::review_tree_carries_no_registered_leak_token sbin::tests::captures_name_their_asset_index_and_a_rejected_asset_has_none sbin::tests::card_not_adopted_voids_before_scoring sbin::tests::a_platform_error_is_recorded_as_api_error_status_and_voids sbin::tests::each_dispatch_runs_in_a_fresh_copy_at_a_never_used_path sbin::tests::a_dirty_or_mismatched_copy_refuses_before_spawning sbin::tests::an_account_project_dir_for_the_copy_path_refuses_before_spawning sbin::tests::a_dirty_runner_tree_refuses_before_spawning sbin::tests::every_test_spawns_only_an_injected_stub_never_claude sbin::tests::an_assignment_carried_path_voids sbin::tests::a_recursive_read_of_an_ancestor_voids sbin::tests::a_tool_glob_or_grep_on_an_ancestor_voids sbin::tests::a_sweep_measured_root_voids sbin::tests::tool_results_c_and_tilde_spellings_are_exempt sbin::tests::the_shadow_audit_flags_reads_outside_the_allowlist sbin::tests::a_temp_read_written_earlier_in_the_session_is_not_flagged sbin::tests::an_unresolved_var_in_a_read_position_is_flagged_not_voided sbin::tests::a_volume_under_sbin_need_bytes_refuses_before_the_copy"
out=$(cd "$R/runner" && nodispatch cargo test --bin redkiln-spike-runner -- $SB_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the driver's tests did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $SB_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 34 passed; 0 failed' || { echo "FAIL: the driver selection did not run exactly 34 tests"; exit 1; }
nd_check "(6) the driver's tests" || exit 1

# ---- (7) THE RECORD: the ADR recorded in (0), naming this finding as evidence, accepted, in ADR-0085's form, with its
# ---- three machine-read lines (§8i.8).
set -- references/adr/"$BIN_ADR"-*.md
[ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: expected exactly one references/adr/$BIN_ADR-*.md for the id recorded in (0)"; exit 1; }
ADR=$1; ADR_ID=$BIN_ADR; b=$(basename "$ADR")
[ "$((10#$ADR_ID))" -ge 165 ] || { echo "FAIL: $ADR is not a record written from 0165 on"; exit 1; }
grep -qE "^evidence:.*\"$SBIN_F\"" "$ADR" || { echo "FAIL: $ADR does not name $SBIN_F as evidence"; exit 1; }
grep -qE '^evidence:.*"S-BIN"' "$ADR" || { echo "FAIL: $ADR does not name S-BIN as evidence"; exit 1; }
grep -qx 'status: accepted' "$ADR" || { echo "FAIL: $ADR is not accepted — it flips on the operator's dated ledger line before this block runs"; exit 1; }
[ "$(grep -c '^### Option — ' "$ADR")" -ge 2 ] || { echo "FAIL: $ADR lacks ADR-0085's option headings (D21-42 24a)"; exit 1; }
grep -q 'What this forbids' "$ADR" || { echo "FAIL: $ADR has no What this forbids block"; exit 1; }
# The README index ROW, opening the line — prose that merely names the id does not satisfy it.
readme=$(awk -v s="$BAR [$ADR_ID]($b) $BAR" 'index($0,s)==1' references/adr/README.md)
[ -n "$readme" ] || { echo "FAIL: the ADR README index has no row opening '| [$ADR_ID]($b) |'"; exit 1; }
ctl=$(printf 'Prose that names %s and %s in passing.\n' "$ADR_ID" "$b" | awk -v s="$BAR [$ADR_ID]($b) $BAR" 'index($0,s)==1')
[ -z "$ctl" ] || { echo "FAIL: the README row check is satisfied by prose — it is vacuous"; exit 1; }
NFIELDS=$(sed -n 's/.*Binary field set (\([0-9][0-9]*\) fields).*/\1/p' "$ADR")
[ -n "$NFIELDS" ] && [ "$(printf '%s\n' "$NFIELDS" | grep -c .)" -eq 1 ] && [ "$NFIELDS" -ge 1 ] || { echo "FAIL: $ADR states no single 'Binary field set (N fields)'"; exit 1; }
ROUTE=$(sed -n 's/.*Binary review route: \(reviewer\|human gate\)\b.*/\1/p' "$ADR")
[ "$ROUTE" = "reviewer" ] || [ "$ROUTE" = "human gate" ] || { echo "FAIL: $ADR states no single Binary review route: '$ROUTE'"; exit 1; }
# The route follows the per-pass outcomes (4) computed and checked (§8i.8's mapping; D21-43 N21 and D21-48 X3 send an
# undecided pass to the human gate).
route_for() { r=reviewer; for o in "$@"; do case "$o" in branch1|branch2-detected) : ;; branch3|undecided-fp|undecided-silent|undecided-partial) r="human gate" ;; *) echo "BAD:$o"; return 0 ;; esac; done; echo "$r"; }
[ "$(route_for branch1 undecided-fp)" = "human gate" ] && [ "$(route_for branch1 undecided-partial)" = "human gate" ] && [ "$(route_for branch1 branch2-detected)" = "reviewer" ] && [ "$(route_for branch1 unregistered)" = "BAD:unregistered" ] && [ "$(route_for branch1 nonsense)" = "BAD:nonsense" ] || { echo "FAIL: the route mapping fails its synthetic controls"; exit 1; }
want=$(route_for "$O1" "$O2")
case "$want" in BAD:*) echo "FAIL: a computed rule outcome is not one of the six registered (§8i.8): $want"; exit 1;; esac
[ "$ROUTE" = "$want" ] || { echo "FAIL: $ADR's route is '$ROUTE'; the computed outcomes ($O1, $O2) give '$want' (§8i.8)"; exit 1; }
PRODUCERS=$(sed -n 's/^.*Binary producers in Phase 1: //p' "$ADR")
[ -n "$PRODUCERS" ] && [ "$(printf '%s\n' "$PRODUCERS" | grep -c .)" -eq 1 ] || { echo "FAIL: $ADR states no single 'Binary producers in Phase 1' line"; exit 1; }

# ---- (8) THE VARIANT IS CONSTRUCTIBLE ONLY AS THE RECORD SAYS — by name, then the per-field tests by count.
CS_CRATE=${CS_CRATE:-rk-ports}
def=$(grep -rnE '^[[:space:]]*pub (struct|enum) ChangeSummary\b' "crates/$CS_CRATE/src" --include=*.rs); rc=$?
[ "$rc" -eq 0 ] && [ -n "$def" ] || { echo "FAIL: ChangeSummary is not defined in crates/$CS_CRATE/src — set CS_CRATE to the crate s8a's record homes it in"; exit 1; }
rk_named() {
  pkg=$1; shift
  out=$(nodispatch cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $# passed; 0 failed" || { echo "FAIL: the $pkg selection did not run exactly $# tests"; exit 1; }
}
rk_named "$CS_CRATE" change_summary::binary_is_constructible_only_through_its_recorded_constructor change_summary::binary_rejects_a_text_patch_for_a_binary_classified_path ui::trybuild_pins_the_binary_change_summary_construction_rule
rk_named rk-store render::ac154_binary_entries_render_from_typed_fields render::ac154_no_binary_content_is_embedded_parsed_or_diffed_as_text
if [ "$ROUTE" = "reviewer" ]; then RT=review_input::binary_item_reaches_the_reviewer_with_the_typed_summary; else RT=review_input::binary_item_refuses_an_automated_verdict_and_opens_a_human_gate; fi
rk_named rk-cli review_input::ac497_binary_classification_assembles_no_diff_and_carries_the_typed_summary "$RT"
# D21-48 X16: the refusal opens its gate through a TYPE-NEUTRAL kernel request, a closed reason `automated-verdict-refused`
# and the summary as an opaque attachment the kernel never types. It is built on the human-gate route only: on the
# reviewer route nothing refuses, and a request nothing calls would pass its own lint. NGATE feeds (10)'s floor.
comment_line() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'; }
KREASON='"automated-verdict-refused"'
raw=$(grep -rnF "$KREASON" crates/rk-kernel/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the closed-reason scan could not run (grep rc=$rc)"; exit 1; }
kr=$(printf '%s\n' "$raw" | grep -F "$KREASON" | comment_line)
if [ "$ROUTE" = "human gate" ]; then
  NGATE=3
  rk_named rk-kernel gate_request::automated_verdict_refused_opens_a_gate_through_the_existing_reach_path gate_request::the_reason_is_closed_and_an_unknown_reason_is_refused_by_name gate_request::the_attachment_is_carried_byte_for_byte_and_never_parsed
  [ "$(printf '%s\n' "$kr" | grep -c .)" -eq 1 ] || { echo "FAIL: the closed reason is not spelled exactly once on a kernel code line (D21-48 X16):"; printf '%s\n' "$kr"; exit 1; }
else
  NGATE=0
  [ -z "$kr" ] || { echo "FAIL: the reviewer route builds no gate request, yet a kernel code line spells the reason:"; printf '%s\n' "$kr"; exit 1; }
fi
ctl=$(printf 'crates/rk-kernel/src/workflow/gate.rs:1:    "automated-verdict-refused" => Ok(Self::AutomatedVerdictRefused),\n' | grep -F "$KREASON" | comment_line)
[ -n "$ctl" ] || { echo "FAIL: the closed-reason scan cannot see a synthetic code line — it is vacuous"; exit 1; }
ctl=$(printf 'crates/rk-kernel/src/workflow/gate.rs:1:    /// spelled "automated-verdict-refused" on the wire\n' | grep -F "$KREASON" | comment_line)
[ -z "$ctl" ] || { echo "FAIL: the closed-reason scan counts a doc comment as code"; exit 1; }
# ADR-0135's forbids bullet and Checkpoint 8a's ban STAND (D21-48 X16): on either route the kernel names no ChangeSummary
# on a CODE line. Its own `change_summary` verb and port-method call are admitted: they name a method, not the type.
# *(dated 2026-10-03, s8i build)* D21-141 (TAKEN PROVISIONALLY under D21-105, Weigh-In wi-45f99f): string literals are blanked
# before the match, as Checkpoint 8a's k_filter does, so the artifact-kind string "ChangeSummary" on a kernel code line is
# admitted and a type use is still caught. The scan and its controls share kt_scan. It replaces the drafted line
# ~~kt=$(printf … | grep -E "$KT_RE" | comment_line)~~ and its control pipeline.
# *(dated 2026-10-03, s8i, R9FIX-8i; refuting round 9)* The operator REJECTED that ruling on 2026-10-03 for (B), a counted two-line exemption with no
# blanking (D21-141's dated line). The lines below are the superseded build, ~~owed a dated edit before the formal run~~
# *(dated 2026-10-06, s8i build, D21149-8i-C)* built below.
# *(dated 2026-10-03, s8i build, R1FIX-8i)* s8i refuting round 1 (a minor): a char literal '"' paired its quote with the next
# string's and hid a type use between them. Char literals are blanked first; a lifetime ('a) is never one quote, one
# character and a quote, so it is left alone. One more control must fire (the last below). Checkpoint 8a's k_filter has
# the same blind spot, owed to s8a by dated note.
# *(dated 2026-10-03, s8i build, R2FIX-8i)* That note is now written beside s8a's k_filter (s8i refuting round 2).
KT_RE='\b(Binary)?ChangeSummary\b'
# *(dated 2026-10-06, s8i build, D21149-8i-C)* D21-141's (B), the operator's ruling of 2026-10-03, is built here. kt_scan
# loses its string and char blanking, so the R1FIX-8i char-literal note above describes the superseded build. Exactly two
# kernel code lines are exempt, each matched on its path plus its WHOLE TRIMMED line, never a line number. Each entry must
# match exactly once, and exactly 2 lines are admitted. A second match, a missing (stale) entry or a wrong total is printed
# and fails the scan. The controls take the two exempt lines as their base, so the counts hold and only the added line can
# fire. The stale, duplicate and list controls break the counts on purpose. Superseded, by this edit:
# ~~kt_scan() { comment_line | sed -E "s/'([^'\\\\]|\\\\.)'/''/g" | sed -E 's/"([^"\\]|\\.)*"/""/g' | grep -E "$KT_RE"; }~~
KT_EXEMPT='crates/rk-kernel/src/lattice.rs|&& ["Changeset", "ImplReport", "ChangeSummary"]
crates/rk-kernel/src/workflow/memo.rs|Self::SummariseChanges => "ChangeSummary",'
kt_scan() {
  comment_line | grep -E "$KT_RE" | KTX="$KT_EXEMPT" awk '
    BEGIN { n = split(ENVIRON["KTX"], e, "\n"); for (i = 1; i <= n; i++) { p = index(e[i], "|"); ep[i] = substr(e[i], 1, p - 1); el[i] = substr(e[i], p + 1); hit[i] = 0 } }
    { a = index($0, ":"); f = substr($0, 1, a - 1); r = substr($0, a + 1); t = substr(r, index(r, ":") + 1); gsub(/^[ \t\r]+|[ \t\r]+$/, "", t)
      x = 0; for (i = 1; i <= n; i++) if (f == ep[i] && t == el[i]) { hit[i]++; x = 1 }
      if (!x) print }
    END { m = 0; for (i = 1; i <= n; i++) { m += hit[i]; if (hit[i] != 1) print "EXEMPT ENTRY matched " hit[i] " times, not once: " ep[i] "|" el[i] }
      if (m != 2) print "EXEMPT admitted " m " lines, not 2" }'
}
raw=$(grep -rnE "$KT_RE" crates/rk-kernel/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the kernel type scan could not run (grep rc=$rc)"; exit 1; }
kt=$(printf '%s\n' "$raw" | grep -E "$KT_RE" | kt_scan)
[ -z "$kt" ] || { echo "FAIL: crates/rk-kernel/src names the type on a code line, or the two-line exemption does not hold — the kernel carries the summary opaque (D21-48 X16; D21-141 (B)):"; printf '%s\n' "$kt"; exit 1; }
# *(dated 2026-10-06, s8i build, D21149-8i-C)* D21-141 (B)'s controls: 14 must fire and 3 must be admitted. Each loop
# control is the exempt base plus one line; the appended and other-path controls REPLACE lattice.rs's base line, so the
# count rule cannot catch them for the path and whole-line rules. kt_ctl's third argument, when given, replaces the exempt
# list. Superseded:
# ~~[ -z "$kt" ] || { echo "FAIL: crates/rk-kernel/src names the type on a code line — the kernel carries the summary opaque (D21-48 X16):"; printf '%s\n' "$kt"; exit 1; }~~
# ~~for ctl in 'F|crates/rk-kernel/src/workflow/gate.rs:1:use rk_ports::ChangeSummary;' 'F|crates/rk-kernel/src/workflow/gate.rs:1:    attachment: BinaryChangeSummary,' 'A|crates/rk-kernel/src/workflow/gate.rs:1:    /// rk-cli serialises a ChangeSummary into these bytes' 'A|crates/rk-kernel/src/runtime/step.rs:1:        "change_summary" => toolchain.change_summary("").map(|_| ()),' 'A|crates/rk-kernel/src/lattice.rs:1:        && ["Changeset", "ImplReport", "ChangeSummary"]' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    fn kind(s: ChangeSummary) -> String { "ChangeSummary".into() }' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    let c = '"'"'"'"'"'; let x: ChangeSummary = y; let s = "z";'; do~~
# ~~want=${ctl%%|*}; line=${ctl#*|}; got=$(printf '%s\n' "$line" | grep -E "$KT_RE" | kt_scan)~~
# ~~if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: the kernel type scan cannot see a synthetic use — it is vacuous for: $line"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: the kernel type scan fires on a line that names no type: $line"; exit 1; }; fi~~
# ~~done~~
KT_BASE='crates/rk-kernel/src/lattice.rs:1:                                            && ["Changeset", "ImplReport", "ChangeSummary"]
crates/rk-kernel/src/workflow/memo.rs:2:            Self::SummariseChanges => "ChangeSummary",'
kt_ctl() {
  want=$1; input=$2; got=$(if [ "$#" -ge 3 ]; then KT_EXEMPT=$3; fi; printf '%s\n' "$input" | kt_scan)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: the kernel type scan admits a synthetic use or a broken count — it is vacuous for: $input"; exit 1; }
  else [ -z "$got" ] || { echo "FAIL: the kernel type scan fires on input that names no type outside its exemption: $got"; exit 1; }; fi
}
for ctl in 'A|crates/rk-kernel/src/workflow/gate.rs:1:    /// rk-cli serialises a ChangeSummary into these bytes' 'A|crates/rk-kernel/src/runtime/step.rs:1:        "change_summary" => toolchain.change_summary("").map(|_| ()),' 'F|crates/rk-kernel/src/workflow/gate.rs:1:use rk_ports::ChangeSummary;' 'F|crates/rk-kernel/src/workflow/gate.rs:1:    attachment: BinaryChangeSummary,' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    fn kind(s: ChangeSummary) -> String { "ChangeSummary".into() }' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    let c = '"'"'"'"'"'; let x: ChangeSummary = y; let s = "z";' 'F|crates/rk-kernel/src/lattice.rs:99:        && ["Changeset", "ImplReport", "ChangeSummary"]' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    let s = r#"a"b"#; let t: ChangeSummary = u; let v = "w";' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    end of the text"; let t: ChangeSummary = u; let v = "w";' 'F|crates/rk-kernel/src/workflow/memo.rs:1:    let p = r"\"; let t: ChangeSummary = u; let v = "w";'; do
  kt_ctl "${ctl%%|*}" "$KT_BASE"$'\n'"${ctl#*|}"
done
kt_ctl A "$KT_BASE"  # the two exempt lines alone are admitted
kt_ctl F 'crates/rk-kernel/src/lattice.rs:1:        && ["Changeset", "ImplReport", "ChangeSummary"] && k != ChangeSummary::KIND'$'\n'"${KT_BASE#*$'\n'}"  # a type use appended to lattice.rs's exempt line, in its place
kt_ctl F 'crates/rk-kernel/src/workflow/gate.rs:1:        && ["Changeset", "ImplReport", "ChangeSummary"]'$'\n'"${KT_BASE#*$'\n'}"  # lattice.rs's exempt content at another path, in its place
kt_ctl F "${KT_BASE#*$'\n'}"  # a stale entry: lattice.rs's line is gone
kt_ctl F "${KT_BASE%%$'\n'*}"  # a stale entry: memo.rs's line is gone
kt_ctl F "${KT_BASE%%$'\n'*}"$'\n'"${KT_BASE%%$'\n'*}"  # one entry twice, the other stale: 2 admitted, each count wrong
kt_ctl F "$KT_BASE"$'\n''crates/rk-kernel/src/workflow/gate.rs:1:    let k = "ChangeSummary";' "$KT_EXEMPT"$'\n''crates/rk-kernel/src/workflow/gate.rs|let k = "ChangeSummary";'  # a third entry, each matched once: 3 admitted
rk_named rk-adapter-toolchain-node classify::ac497_classify_binary_paths
rk_named rk-adapter-toolchain-rust classify::ac497_classify_binary_paths
out=$(nodispatch cargo test -p "$CS_CRATE" --test it -- change_summary::binary_field_ 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the per-field selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
nf=$(printf '%s\n' "$out" | grep -cE '^test change_summary::binary_field_[a-z0-9_]+ \.\.\. ok$')
[ "$nf" -eq "$NFIELDS" ] || { echo "FAIL: $nf per-field tests passed; the record fixes $NFIELDS fields — one test per field"; exit 1; }
printf '%s\n' "$out" | grep -qE "test result: ok\. $NFIELDS passed; 0 failed" || { echo "FAIL: the per-field selection did not run exactly $NFIELDS tests"; exit 1; }
nd_check "(8) the variant's tests" || exit 1
# A FLOOR beneath those tests, over CODE lines only (a guard satisfied by prose reports evidence it does not have): a
# CALL that constructs the payload — an associated function of BinaryChangeSummary, or a lower-case `binary…`
# constructor on the enum — outside the type's own crate appears only in a file the record names as a Phase-1 producer.
# A match arm or `if let` that DESTRUCTURES `ChangeSummary::Binary(b)` is not a construction and is admitted; a struct
# literal with private fields does not compile outside the crate, and the trybuild case above carries that.
BIN_RE='BinaryChangeSummary::[A-Za-z_][A-Za-z0-9_]*[[:space:]]*(::<[^>]*>)?[[:space:]]*\(|ChangeSummary::binary[a-z0-9_]*[[:space:]]*\('
raw=$(grep -rnE "$BIN_RE" crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the construction floor could not run (grep rc=$rc)"; exit 1; }
bin_code() { grep -E "$BIN_RE" | grep -v "^crates/$CS_CRATE/" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'; }
code=$(printf '%s\n' "$raw" | bin_code)
if [ "$PRODUCERS" = "none" ]; then
  [ -z "$code" ] || { echo "FAIL: a Binary construction on a code line, and the record names no Phase-1 producer:"; printf '%s\n' "$code"; exit 1; }
else
  while IFS=: read -r f _; do
    [ -z "$f" ] && continue
    case ", $PRODUCERS," in *", $f,"*) : ;; *) echo "FAIL: $f constructs Binary and is not a producer the record names"; exit 1;; esac
  done <<< "$code"
fi
for ctl in 'F|crates/x/src/y.rs:1:    let b = BinaryChangeSummary::from_pointer(p)?;' 'F|crates/x/src/y.rs:1:    let s = ChangeSummary::binary(b);' 'A|crates/x/src/y.rs:1:        ChangeSummary::Binary(b) => render_binary(b),' 'A|crates/x/src/y.rs:1:    if let ChangeSummary::Binary(b) = s {' 'A|crates/x/src/y.rs:1:    // BinaryChangeSummary::new(p) is the recorded constructor'; do
  want=${ctl%%|*}; line=${ctl#*|}; got=$(printf '%s\n' "$line" | bin_code)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: the floor cannot see a synthetic construction — it is vacuous for: $line"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: the floor fires on a line that constructs nothing: $line"; exit 1; }; fi
done

# ---- (9) STATUSES, HONESTLY: the four criteria carry the dated note, AC-475 its measured share, the "four of five"
# ---- lines their correction, `s8`'s final-label bullet its note, and no line states S-BIN pending without a note.
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
# `s8`'s bullet that S-BIN, S-H1c and P8 gate the design's "final" label: S-BIN leaves that gate here (§8i.8).
s8l=$(grep -nF 'S-BIN, S-H1c and P8' runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$s8l" ] || { echo "FAIL: s8's final-label bullet no longer names S-BIN, S-H1c and P8 — it was rewritten, not noted"; exit 1; }
printf '%s\n' "$s8l" | grep -qF '[S-BIN reported (s8i)' || { echo "FAIL: s8's final-label bullet carries no dated s8i note (§8i.8)"; exit 1; }
STALE='unrun|UNRUN|not been run|NOT been run|until S-BIN|before S-BIN|S-BIN reports|BLOCKED'
# (dated 2026-10-03, s8i build, R4FIX-8i) The register's floor is re-derived at 730a4cc: 19 stale-pattern S-BIN lines
# (17 at ca8f387; s8g-era edits added two). The design's stays 21. Replaces ~~acceptance-criteria.md:17~~.
for doc in references/2026-09-08-architecture-design.md:21 references/2026-09-08-acceptance-criteria.md:19; do
  f=${doc%%:*}; floor=${doc##*:}
  hits=$(grep -nE 'S-BIN' "$f"); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $f no longer mentions S-BIN at all — its lines were rewritten, not noted"; exit 1; }
  stale=$(printf '%s\n' "$hits" | grep -E "$STALE" | grep -vF '[S-BIN reported (s8i)')
  [ -z "$stale" ] || { echo "FAIL: $f still states S-BIN as pending without a dated s8i note:"; printf '%s\n' "$stale" | cut -c1-160; exit 1; }
  nnote=$(printf '%s\n' "$hits" | grep -cF '[S-BIN reported (s8i)')
  # (dated 2026-10-03, s8i build, R5FIX-8i) s8i refuting round 5's minor: the message read 'at ca8f387' for both floors.
  [ "$nnote" -ge "$floor" ] || { echo "FAIL: $f carries $nnote dated s8i notes; $floor lines stated S-BIN pending at its re-derivation (ca8f387 for the design, 730a4cc for the register)"; exit 1; }
done
syn=$(printf '22:| ChangeSummary | BLOCKED | S-BIN registered and unrun |\n' | grep -E "$STALE" | grep -vF '[S-BIN reported (s8i)')
[ -n "$syn" ] || { echo "FAIL: the stale-line guard cannot see a synthetic stale line — it is vacuous"; exit 1; }

# ---- (10) THE WHOLE SUITE, fmt, clippy, under the no-dispatch wrapper. The floor is Checkpoint 8h's recorded count (set
# ---- in (0)) plus this stage's 3 CS + NFIELDS per-field + 2 render + 2 review_input tests, less s8a's superseded
# ---- sealed-slot test, plus NGATE (D21-48 X16: 3 `gate_request::` tests on the human-gate route, 0 on the reviewer
# ---- route, set in (8)): 6 + NFIELDS + NGATE.
out=$(nodispatch cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked' | head -20; exit 1; }
res=$(printf '%s\n' "$out" | grep -E '^test result: ')
[ -n "$res" ] || { echo "FAIL: the suite printed no result line"; exit 1; }
bad=$(printf '%s\n' "$res" | grep -vE '^test result: ok\. [0-9]+ passed; 0 failed')
[ -z "$bad" ] || { echo "FAIL: a result line is not ok with 0 failed:"; printf '%s\n' "$bad"; exit 1; }
total=$(printf '%s\n' "$res" | awk '{s+=$4} END{print s+0}')
nres=$(printf '%s\n' "$res" | grep -c .)
case "$NGATE" in 0|3) : ;; *) echo "FAIL: NGATE '$NGATE' was not set by (8)"; exit 1;; esac
floor=$((FLOOR_8H_PASSED + 6 + NFIELDS + NGATE))
[ "$total" -ge "$floor" ] && [ "$nres" -ge "$FLOOR_8H_RESULTS" ] || { echo "FAIL: $total passed over $nres results — below the floor of $floor over $FLOOR_8H_RESULTS (Checkpoint 8h's $FLOOR_8H_PASSED + 6 + $NFIELDS + $NGATE)"; exit 1; }
out=$(nodispatch cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nodispatch cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc (the gate is its exit code — ADR-0102 decision 5)"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "(10) the whole suite, fmt and clippy" || exit 1

# ---- (11) CHECKPOINT 8h (the entry gate) AND CHECKPOINTS 8a AND 5 (made provisional by this stage's edits), RE-RUN
# ---- WHOLE from their own stage files' bytes. Nothing else is re-run directly. BOUNDED (D21-46): every replay runs
# ---- with RK_CP_NESTED=1, and when THIS block runs under RK_CP_NESTED=1 it replays nothing — and demands none of its
# ---- replays' inputs. 8h and 8a honour the variable (8h's (13), 8a's (11)); Checkpoint 5 nests nothing. A replayed
# ---- block that stops honouring it is refused below rather than run for hours. Each block is scanned FIRST, over code
# ---- lines, for a shape that EXECUTES a dispatch, and is refused if it carries one. Listing ignored tests (`--list`)
# ---- or compiling with the live feature does not execute one; s8h's block does both, so both stay admissible. NO
# ---- DISPATCH (D21-47): every replay runs inside (0)'s wrapper — the fake claude, no real claude on PATH, the trap.
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
# REASONED EXEMPTIONS, for Checkpoint 8h's block only (final consistency pass 2026-09-24). Each is admitted by SHAPE,
# COUNTED, and tied to its premise, so a new line of the shape is re-examined rather than admitted silently:
#   E1  a launcher run under 8h's sandbox environment array "${scl[@]}": env -i, a scratch HOME, and a PATH of a
#       scratch fake directory, /usr/bin and /mingw64/bin only, in a scratch repository. Its children are `sh -c` and
#       `cp`, and s8h (10) asserts no claude sits in those two system directories. Exactly 2 lines, and only while the
#       block defines scl that way.
#   E2  the argv-screen control's `for argv in` list: strings handed to `--check wrap`, which spawns nothing and refuses
#       each (s8h (10), WRAP_ARGV). Exactly 1 line, and only while its next code line runs `--check wrap -- $argv`.
ex8h() {  # $1 = an extracted block: prints its exempt code lines; returns 1 when a premise or a count fails
  code=$(grep -vE '^[[:space:]]*#' "$1")
  printf '%s\n' "$code" | grep -qE '^scl=\(env -i "HOME=\$sh_home" .*"PATH=\$sw:/usr/bin:/mingw64/bin"\)$' || { echo "E1's premise: no scl=(env -i … PATH=\$sw:/usr/bin:/mingw64/bin) definition"; return 1; }
  e1=$(printf '%s\n' "$code" | grep -E '"\$\{scl\[@\]\}"[[:space:]]+bash[[:space:]]+tools/live-run\.sh[[:space:]]+wrap[[:space:]]+--[[:space:]]')
  e2=$(printf '%s\n' "$code" | awk 'p ~ /^for argv in / && $0 ~ /--check wrap -- [$]argv/ {print p} {p=$0}')
  n1=$(printf '%s\n' "$e1" | grep -c .); n2=$(printf '%s\n' "$e2" | grep -c .)
  [ "$n1" -eq 2 ] && [ "$n2" -eq 1 ] || { echo "the exemption counts moved: E1=$n1 (want 2), E2=$n2 (want 1)"; return 1; }
  printf '%s\n' "$e1" "$e2"
}
# Controls, assembled by printf so no code line here spells a launcher run: a sandboxed pair and an argv list are
# admitted, an unsandboxed wrap beside them is still refused, and without the scl definition nothing is admitted.
{ printf '%s\n' 'scl=(env -i "HOME=$sh_home" "PATH=$sw:/usr/bin:/mingw64/bin")'
  printf 'out=$(cd "$sr" && "${scl[@]}" bash tools/live-run.sh %s -- sh -c x 2>&1); rc=$?\n' wrap
  printf 'out=$(cd "$sr" && "${scl[@]}" bash tools/live-run.sh %s -- cp a b 2>&1); rc=$?\n' wrap
  printf 'for argv in "./target/debug/redkiln %s rk-x"; do\n' run
  printf '  out=$("${clean[@]}" bash "$L" --check %s -- $argv 2>&1); rc=$?\n' wrap
  printf 'bash tools/live-run.sh %s -- sh -c x\n' wrap
} > "$T/ex-ctl.sh" || { echo "FAIL: could not write the exemption control"; exit 1; }
ex8h "$T/ex-ctl.sh" > "$T/ex-ctl.ex"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CONTROL — the exemptions refused a well-formed sandbox: $(cat "$T/ex-ctl.ex")"; exit 1; }
hit=$(grep -vE '^[[:space:]]*#' "$T/ex-ctl.sh" | grep -vxF -f "$T/ex-ctl.ex" | grep -cE "$DISPATCH_RE")
[ "$hit" -eq 1 ] || { echo "FAIL: CONTROL — with the exemptions applied the scan saw $hit launcher lines, not the one unsandboxed wrap"; exit 1; }
grep -v '^scl=' "$T/ex-ctl.sh" > "$T/ex-ctl2.sh" || { echo "FAIL: could not write the second exemption control"; exit 1; }
if ex8h "$T/ex-ctl2.sh" >/dev/null; then echo "FAIL: CONTROL — the exemptions admitted launcher runs with no sandbox definition"; exit 1; fi
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8I (11) SKIPPED: this block is itself a replay (RK_CP_NESTED=1); its re-runs ran at Checkpoint 8i's own close"
else
# The replayed blocks carry their own record ids as literals, written into each by dated edit at write time, and each
# refuses while one is not yet written (D21-47). This block supplies none (final consistency pass 2026-09-24: the
# environment list that stood here is withdrawn).
xmark() { awk -v f="$FENCE" -v s="echo \"$2-END-OF-BLOCK reached\"" '$0==f"bash"{inb=1;buf="";has=0;next} inb&&$0==f{inb=0;if(has){printf "%s",buf;n++};next} inb{buf=buf $0 "\n";if($0==s)has=1} END{exit n==1?0:1}' "$1"; }
xmark runbook/phase-1/s8h-cards-and-run-environment.md CP8H > "$T/cp8h.sh"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/cp8h.sh" ] || { echo "FAIL: exactly one Checkpoint 8h block ending in its CP8H sentinel was not found in s8h"; exit 1; }
xmark runbook/phase-1/s8a-records-and-red-checkpoints.md CP8A > "$T/cp8a.sh"; rc=$?
[ "$rc" -eq 0 ] && [ -s "$T/cp8a.sh" ] || { echo "FAIL: exactly one Checkpoint 8a block ending in its CP8A sentinel was not found in s8a"; exit 1; }
# *(dated 2026-10-03, s8i build, R1FIX-8i)* Checkpoint 5's block needs the corpus at CORPUS_PIN and clean (rk-git's corpus_f12
# test). §8i.5's packet step 11 returns it there after S-BIN's corpus commits, before this block runs.
awk -v f="$FENCE" 'index($0,"**Checkpoint 5.** All of:")==1{armed=1;next} armed&&$0==f"bash"{inb=1;next} inb&&$0==f{exit} inb{print}' runbook/phase-1/s5-workspace-ci-rk-types.md > "$T/cp5.sh"; rc=$?
[ "$rc" -eq 0 ] && [ "$(grep -c . "$T/cp5.sh")" -ge 20 ] || { echo "FAIL: the Checkpoint 5 block was not extracted from s5"; exit 1; }
printf '\necho "CP5-END-OF-BLOCK reached"\n' >> "$T/cp5.sh"
for blk in cp8h cp8a; do
  grep -qF '"${RK_CP_NESTED:-0}" = 1' "$T/$blk.sh" || { echo "FAIL: the extracted $blk block no longer honours RK_CP_NESTED — its replay would run its whole chain; establish why before running it"; exit 1; }
done
for blk in cp8h cp8a cp5; do
  : > "$T/$blk.ex" || { echo "FAIL: could not write $blk's exemption list"; exit 1; }
  if [ "$blk" = cp8h ]; then
    ex8h "$T/cp8h.sh" > "$T/cp8h.ex" || { echo "FAIL: Checkpoint 8h's reasoned exemptions do not hold — $(cat "$T/cp8h.ex"); re-examine (10) before re-running it"; exit 1; }
  fi
  hit=$(grep -vE '^[[:space:]]*#' "$T/$blk.sh" | grep -vxF -f "$T/$blk.ex" | grep -nE "$DISPATCH_RE")
  hit2=$(grep -vE '^[[:space:]]*#' "$T/$blk.sh" | grep -vxF -f "$T/$blk.ex" | grep -nE "$TESTRUN_RE" | grep -v -- '--list')
  [ -z "$hit$hit2" ] || { echo "FAIL: the extracted $blk block carries a shape that executes a dispatch and is NOT run — establish the guard's scope (golden constraint 9):"; printf '%s\n' "$hit" "$hit2"; exit 1; }
  out=$(nodispatch env RK_CP_NESTED=1 bash "$T/$blk.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $blk re-run exited $rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  last=$(printf '%s\n' "$out" | grep . | tail -1)
  case "$last" in *END-OF-BLOCK\ reached) : ;; *) echo "FAIL: $blk did not reach its end-of-block sentinel (last line: $last)"; exit 1;; esac
  nd_check "replay of $blk" || exit 1
done
fi

# ---- (12) NOTHING DISPATCHED, NOTHING LEAKED: the trap still empty and the fake never called, no new session
# ---- transcript, the worktree path set unchanged, no stale registration, BrittonGames unmoved across the batches by
# ---- its pinned snapshots, and no review tree left registered in BrittonGames or left on disk (Halt 6).
nd_check "(12), the whole block" || exit 1
sess_after=$(find $CFGS -type f -name '*.jsonl' | sort)
test -n "$sess_after" || { echo "FAIL: the after-listing of $CFGS is empty — the tripwire compared nothing"; exit 1; }
new=$(comm -13 <(printf '%s\n' "$sess_before") <(printf '%s\n' "$sess_after"))
[ -z "$new" ] || { echo "FAIL: a new session transcript appeared while this block ran — something dispatched (or you started a Claude session; re-run without one):"; printf '%s\n' "$new"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }
sv='import sys
L=[dict(t.split("=",1) for t in l.split()) for l in open(sys.argv[1],encoding="utf-8").read().splitlines() if l.strip()]
w=[d.get("when") for d in L]
need=["before-B2","after-B2","after-B3"]+(["after-B4"] if sys.argv[2]!="0" else [])
keys=("head","status_sha256","umap_sha256","uproject_sha256","worktrees_sha256")
same=len({tuple(d.get(k) for k in keys) for d in L})==1 and all(d.get(k) for d in L for k in keys)
print("snap", "ok" if L and w[0]=="before-B2" and all(x in w for x in need) and same else "BAD")'
snv=$(python -c "$sv" "$T/s-bin_brittongames-snapshots.txt" "$(rv reruns)"); rc=$?
[ "$rc" -eq 0 ] && [ "$snv" = "snap ok" ] || { echo "FAIL: the pinned BrittonGames snapshots are not one unchanged state from before B2 through every batch (§8i.5): $snv"; exit 1; }
bg=$(git -C D:/repos/BrittonGames worktree list --porcelain); rc=$?
[ "$rc" -eq 0 ] && [ -n "$bg" ] || { echo "FAIL: BrittonGames' worktree list could not be read (rc=$rc)"; exit 1; }
root_m=$(cygpath -m "$SBIN_ROOT"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$root_m" ] || { echo "FAIL: cygpath could not normalise the review root '$SBIN_ROOT'"; exit 1; }
under() { awk -v r="$(printf '%s' "$1" | tr 'A-Z' 'a-z')" '/^worktree /{p=tolower(substr($0,10)); if(p==r||index(p,r"/")==1) print}'; }
hit=$(printf '%s\n' "$bg" | under "$root_m")
[ -z "$hit" ] || { echo "FAIL: a review tree is registered in BrittonGames' .git — §8i.4 forbids it:"; printf '%s\n' "$hit"; exit 1; }
ctl=$(printf 'worktree %s/rev/item-1\n' "$(printf '%s' "$root_m" | tr 'a-z' 'A-Z')" | under "$root_m")
[ -n "$ctl" ] || { echo "FAIL: the registration check cannot see a synthetic worktree under the review root — it is vacuous"; exit 1; }
test ! -e "$SBIN_ROOT" || { echo "FAIL: the review root $SBIN_ROOT still exists — remove the trees (Halt 6: a tree that will not delete is the handle)"; exit 1; }

echo "CP8I-END-OF-BLOCK reached"
```

⚠️ **What this block cannot see, said rather than claimed away.**
- **The tripwire.** (0) and (12) see a dispatch that **starts a session under `~/.claude-personal` or `~/.claude`**.
  A dispatch under another config directory, or one that writes no transcript, is invisible to them. That is why the
  wrapper also removes every real `claude` from PATH and logs every call to the fake.
- **The text scan.** (11) refuses an extracted block by its text. It reads only the three blocks it extracts, not the
  blocks they extract in turn.
- **The nested runs.** The nested 7f/7h/7i `redkiln run`s are covered by the missing launch record, which makes them
  refuse before any spawn (s8h §8h.7). They are also covered by the wrapper, which they inherit.
- **An absolute path.** A child that names a real `claude` by absolute path escapes the PATH filter. A child launched
  with `CLAUDE_CONFIG_DIR` set explicitly escapes the trap, and s8g states the same limit. The launcher does both, at
  its steps 4 and 6, which is why (11) refuses a launcher line by its text.
- **The construction floor.** (8)'s floor matches constructor CALLS: an associated function of `BinaryChangeSummary`
  (the payload name `s8a`'s file uses), or a lower-case `binary…` constructor on the enum. If `s8a`'s record names
  the payload or its constructor otherwise, the regex takes a dated amendment first. A construction assembled by a
  macro is invisible to it. The named tests above it are the assertion, and the floor is not.
- **What (4) re-derives.** (4) re-derives the rule outcomes from the records' own fields, and the records' fields come
  from the driver. The raw transcripts beside them are pinned so that an adjudication can re-derive every field, and
  the finding says whether one was run. `missing-fields.jsonl` is the operator's judgement, named as one. (4) checks
  that its quotes exist in the digests, not that the reading is right.

### 8i.10 Close

The two-commit close (`00-preamble.md`'s legend: *"(1) the work and its checkpoint outputs, then (2) a tracker commit
writing that commit's own hash into §1"*). The work commit carries the METHOD-S amendment's successor notes if any,
the finding and the three index rows (F41, F42, S-BIN), the ledger lines, the `Binary` record with its README index cell, census and the
preamble's record count, the Rust and its golden files with their `-text` rows, the register and design notes
(AC-475's replacement and the "four of five" corrections among them), the
`s5` Checkpoint 5 amendment if its condition lapsed, `s8a`'s dated amendment for the superseded sealed-slot test,
`s8`'s bullet and guard amended to the record's construction rule, `s5`'s Checkpoint 5 block amended for
the superseded opacity pin, `s8`'s final-label bullet noted, `rk-cli`'s `review_input` module, rk-kernel's type-neutral gate request on the
human-gate route (D21-48 X16), Checkpoint 8i's three
recorded ids (`SBIN_F`, `BIN_ADR`, `SBIN_LEDGER`) written into (0) by dated edit, and
Checkpoint 8i's output. The record flips from `proposed` only on the operator's dated ledger line, which precedes the
checkpoint run. The tracker commit writes the hash. It records row 8a as re-run and passed at this commit, because
Checkpoint 8i re-ran 8a's amended block whole (guard (11)), and it records row 5 the same way. AC-475's lift
criterion and the F41/F42 index gap are no longer raised as owed. The revision brief assigned both here, and both
are in the work commit. Adversarial refuting rounds
over this file and its checkpoint continue until a round moves no code and no guard.
