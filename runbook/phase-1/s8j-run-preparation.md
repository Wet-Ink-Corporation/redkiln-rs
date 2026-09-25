# Step 8j — Fixture proof of both lanes, CI green, and the run preparation (Phase 1, part 8, tenth sub-stage)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8j` | Phase 1 | Checkpoint 8i | Checkpoint 8j | `s8i` | `s8` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45.* The draft at `a1d35ac` was written before those three ledger entries. This
revision changes the following:

- **N2** (§8j.1, §8j.2, §8j.4): the story's authored `Spec` is whatever the story pack's mandatory `specification`
  stage selects, which is `bdd-scenarios` today. It is never `user-story-ac` by name.
- **Slice and project** (§8j.1, §8j.4, §8j.5): both lane items carry `s8e`'s `slice`. `[project] name` is already
  in `.redkiln/config.toml`, and the lane fixture copies it.
- **Question loading** (§8j.2): the loader and `.redkiln/processes/` are `s8d`'s. This stage declares only the two
  human-root questions. RT-59 is cited as settled by R-48, and it leaves this stage's record.
- **D21-44** (§8j.1(c), §8j.1(d), §8j.4, §8j.8, §8j.10):
  - the legacy run worktree moves from `.git/redkiln/worktrees/` to `.redkiln/worktrees/`, with its ref kept, where
    `s8e`'s re-entry refuses it;
  - Checkpoints 7h and 7i are re-pointed at named fixture items;
  - the lane fixture and every recursive walk over `.redkiln/` in `s8` skip `.redkiln/worktrees/`.
- **N28 and the probes** (§8j.6):
  - the pin moves by `s8h` §8h.8's steps 1–3, which are cited here and not restated. That procedure's own
    `live_dispatch_probe` dispatch is taken inside Checkpoint 7c's run;
  - Checkpoint 7c is re-run whole and unamended, from a plain terminal, as the child of the launcher's `wrap`
    mode. That run is also the `live_dispatch_probe` re-run;
  - the probe names are `s8h`'s;
  - every result is run and recorded by the operator, in `s8j-evidence.txt`.
- **The launcher in `s8`** (§8j.8 A1, A7): all three of `s8`'s `redkiln run` sites go through `tools/live-run.sh run`.
- **Ownership** (§8j.8 A1, A5.2, A8):
  - the quoted-predicate guard is `s8b`'s form, adopted here and not re-specified;
  - `verify_completed`'s `item` and `adapter` members are bound from `s8g`;
  - `s8e`'s anchored rebuild differential joins `s8`'s (2), and `s8`'s walks over `.redkiln/` skip `worktrees/`
    (`s8e` §8e.9, §8e.10);
  - AC-114's `curation.decision` lint is this stage's by `s8g`'s assignment, and it is owed with the promotion.
- **The proofs' module** (§8j.4) is `lane_proof`, because `s8e` owns `lane_e2e` and its fixture-toolchain lane.
- **The gates this stage adds** (§8j.2, §8j.3) each take a `[[gate_expiry]]` row in `s8h`'s table.
- **C2 and C6 are excluded** (§8j.5), because `s8f`'s render path calls the locator.
- **`s8g`'s hand-on** (§8j.1(e)): both lane items declare the `tag-release` trial.
- **Checkpoint 8j** (§8j.10):
  - the entry chain is re-run once, through Checkpoint 8i, and the checkpoint says so;
  - every re-run is fenced against a real dispatch;
  - the whole-suite floor is re-derived from Checkpoint 8i's recorded count;
  - the block runs from a plain terminal.

---

**Why this stage exists.** `s8a`…`s8i` build what criterion (1) and (1b) need. Nothing before this stage proves the
parts work **together, as a lane, through the entry point the real runs will use**. D21-40 decision 2 authorises the
real runs *"once the sub-stages' fixture proofs pass, and not before. That includes a FakeHarness end-to-end test of
`verbs::run::run`"*. `s8e` wrote the first such test (D21-42 24c). This stage writes the proofs that the spend gate
turns on: each lane, from `triage` to its landing, through `verbs::run::run`, with the real Rust toolchain adapter
and across real process boundaries.

It also prepares the run so that `s8` has nothing left to decide:

- the two items the runs are pointed at (D21-41 (f); D21-42 18);
- the human roots those items start from (D21-41 (f): *"decided once for both lanes"*);
- the bound on the first real invocation (D21-42 19b);
- the CLI pin, Checkpoint 7c's whole re-run and the two live probes `s8h` authored (D21-42 16; D21-43 N28;
  D21-41 (g));
- the legacy run worktree's re-point, and Checkpoints 7h and 7i re-pointed at named fixture items (D21-44);
- CI green on the pushed branch (D21-42 14a);
- the amendments to `s8`'s own instruments that the re-measurement of D21-40 found, and that the sub-stages make
  necessary (§8j.8).

**The order is fixed, and every paid or published act in it is a gate.** First the fixture proofs (§8j.4). Then,
and only then, the pin move, Checkpoint 7c's whole re-run and the probes (§8j.6), which spend. Last, CI on the
frozen code (§8j.7), which publishes. **No block in this file dispatches to a real account, and no block pushes.** Checkpoint 8j reads what
the gated acts recorded; it never repeats them.

🛑 **Every push and every paid dispatch in this stage is an explicit operator go-ahead, given at that moment and for
that act.** A go-ahead for one push does not cover the next push, and a go-ahead for one probe does not cover the
next. D21-42 14a: *"Each push is confirmed with the operator."* D21-40 decision 2: *"If a real run is refused
unexpectedly, the next step is to stop and report, not to spend again."* The same rule governs the probes.

**This stage takes decisions, and says so up front.** The last three stage files that said *"this step takes no
decision"* were wrong by eighteen, eleven and one records (s7i's own correction: *"`s7g` took eighteen … `s7h` eleven"*, and s7i took `ADR-0163`). This stage writes **two records
at `proposed`**, each with the next free id at writing, and neither id is assigned here:

1. **The human-root questions** (§8j.2). Decided by D21-41 (f). What is left to the record is the two questions'
   ids and options, in the spelling `s8d`'s loader reads, and the name the step-4 gate binds under.
2. **The boundary hold** (§8j.3). D21-42 19b decides *what*: *"The first real fix-lane invocation stops at the
   analysis → specification boundary for the operator to review."* No record decides *how*, and no mechanism for it
   exists today.

Each record flips to `accepted` only on the operator's dated ledger line (ADR-0126's route). It also writes ledger
entries in `spikes/DECISIONS-2026-09-21.md`, the file that D21-40…D21-42 extend:

- the story candidate, with its reading of ADR-0011/E7;
- the Codex item's retirement;
- the fix-item reservation;
- the note on the fixture item;
- the re-point of the legacy checkout (D21-44);
- the reading of ADR-0114 decision 3 behind the `tag-release` trial declarations (§8j.1(e));
- the (g) flip, which is the operator's.

---

### 8j.1 The items: the story, the Codex item retired, 7i's empty case moved, the fixture item noted and its checkout re-pointed

**(a) The story item — `.redkiln/items/rk-story-harden-spike-pack.toml`.** D21-41 (f): *"A full lane entered at
`triage`. The candidate is to harden `spikes/p9/packs/spike.toml`, a Phase-2 story whose change stays inside
`spikes/p9/`."*

- `id = "rk-story-harden-spike-pack"`, `kind = "story"`, `stage = "triage"`.
- `slice = "harden-spike-pack"`. This is the declared slice from `s8e` §8e.8, and the story item has exactly one.
  The value is not the item id, not the project name, and not a stage name, and it has no whitespace. Those are the
  refusals `s8e` §8e.8 puts at the mint.
- `[facets]`: `cos = "standard"`, `criticality = "low"`, `blast_radius = "low"`, `size = "low"`,
  `external_dep = false`.
- `[trial] activity = "tag-release"`. See (e).

The `<project>` component of its trailers is `.redkiln/config.toml`'s `[project] name = "redkiln-rs"`, which is
already present at `a6349b6` [measured]. This stage adds nothing to the config.

The packet (f) option A measured why the facets must be low. With the Codex item's `external_dep = true` and
`blast_radius = "med"`, the selection gained *"five kinds"*: APIContract, ThreatModel, IntegrationReport,
RolloutPlan and FlagRecord. Low facets are the candidate's real profile. They are not chosen to make the run
cheap.

**What the lane authors is the `Spec` its mandatory `specification` stage selects.** D21-43 N2 takes *"The Spec the
story lane's mandatory specification stage selects"*, which is `bdd-scenarios` today, and supersedes D21-41 (f)'s
naming of `user-story-ac`. The premise it corrects is quoted in the same entry: *"`user-story-ac` has
`max_rigor = "recommended"`, so it is never admitted at the story pack's **mandatory** specification stage"*. No
line of this file names the producing activity as a literal. The proofs read it from the run (§8j.4).

**What "harden" means: the story's intent, not its spec.** The lane's own `specification` stage authors the
acceptance criteria, and this file must not write them in advance. The intent is stated in the item's header
comment and in the ledger row: *bring `spikes/p9/packs/spike.toml` up to the contract the two shipped packs were
brought to at Step 7e, so that it loads through the kernel's pack loader and could carry a lane — without installing
it.* It starts from these measured gaps, and each can be checked again:

- **No lane order.** `grep -c 'stage_order' spikes/p9/packs/spike.toml` gives **0**. `[policy] stage_order` is
  ADR-0153's only ordered stage carrier. The loader reads it as optional, so a spike item would load and then could
  not cross a single boundary.
- **No `stale_policy`.** Both shipped packs record one per cost class, with decision 5's warrant (ADR-0117). The
  spike pack records none.
- **No gate deadline scalar.** Both shipped packs carry `gate_deadline_secs`, and the spike pack does not. The value
  belongs to the RT-57 record that `s8h` writes (D21-42 19c).
- **Its twelve kinds are all registered and provided.** Every kind named by its baselines and require rows is in
  `.redkiln/artifacts.toml` and has at least one provider in `.redkiln/catalog/`. The five `spike-*` /
  `technical-spike` activities are installed. The hardening keeps this true under a mechanical check instead of by
  inspection. This was measured on 2026-09-24 over ProblemStatement, CompetitorAnalysis, SpikeCharter, SpikePlan,
  Estimate, ExperimentResults, ReviewVerdict, SpikeReport, DecisionRecord, ClosureRecord, KBAtoms and
  Retrospective.

🛑 **It is not installed.** ADR-0011 says *"Phase 1 ships exactly two process packs: `bug-standard` and `story`"*,
and E7 rejected *"Install twelve, enable two via `[packs] enabled`"*. The packet (f) risk list requires the ledger
row to state the reading under which a landed edit under `spikes/p9/` ships no pack: *"Landing a spikes/p9 edit
under a story lane is a reading of ADR-0011/E7, not something they expressly permit, so the D24 row must state it."*
**This stage does not touch `spike.toml`.** Checkpoint 8j asserts that its bytes are unchanged since `ca8f387`. A
stage that did the story's work in advance would leave the lane nothing to author.

⚠️ **The lattice goldens do not move here.** `s8b` records that *"When `s8j` hardens `spikes/p9/packs/spike.toml`
(D21-41 f), `pk:` moves and both goldens"* must regenerate in that commit. This stage hardens nothing. The edit is
the story lane's, and it lands on `rk-run/rk-story-harden-spike-pack` at `s8`. The goldens therefore regenerate in
the commit that merges that ref, which is the operator's hand-off after `s8` and not a commit on this stage's branch.

**(b) The Codex item is retired: `.redkiln/items/rk-story-codex-adapter.toml` is deleted, by a dated ledger
entry.** The entry quotes the warrants:

- ADR-0007: *"the Codex adapter ships in Phase 4 ("reach"), and only after S-CODEX passes"*;
- E-0094-1: *"`rk-adapter-codex` is out of Phase 1 under every reading"*;
- the record `s8a` wrote for D21-42 22, which supersedes ADR-0094 decision 2's phase clause toward ADR-0007.

The file is deleted rather than marked retired, for two reasons. `ItemRecord` has no retired field. A story item
left in `.redkiln/items/` is also one `redkiln run` away from a lane on a Phase-4 story. The item is named by
`runbook/phase-1/s7i-cost.md` (seven occurrences), by the ledger and by the packet. Apart from its own file, no
`.rs` or `.toml` names it [measured at `a6349b6`]. So (c) lands **in the same commit**.

**(c) Checkpoint 7i's empty case moves to `rk-cost-empty-case`, and Checkpoints 7h and 7i stop reading
`$FIX_ITEM`.** D21-41 (f): *"Checkpoint 7i's empty case moves to a dedicated item that nobody dispatches, and its
PRECONDITION guard is kept."* A new `.redkiln/items/rk-cost-empty-case.toml` has `id = "rk-cost-empty-case"`,
`kind = "docs"` and `stage = "triage"`. Its header says only this:

> *dedicated to Checkpoint 7i's empty case; nobody dispatches it; the PRECONDITION guard in Checkpoint 7i is what
> protects that case*

🛑 **It does not say the item "can never run".** The packet (f) measured `derive_lane`'s
`(Some(given), None) => Ok(given)`, which runs a `kind = docs` item whenever `--lane` is passed. The guard is the
protection, and the header must not claim more.

**Both checkpoints are re-pointed at named fixture items, in one dated edit each.** `s8e` records the need: *"`s8j`
re-points 7h and 7i when it mints the fresh item"*. The reason is the environment. `s8` sets `FIX_ITEM` to the item
§8j.5 reserves (§8j.8 A1). A Checkpoint 7h or 7i run in that shell would read the reserved item's run instead of the
fixture it was built over. Both checkpoints stop reading the variable:

- **Checkpoint 7i** (`runbook/phase-1/s7i-cost.md`):
  - its seven `rk-story-codex-adapter` occurrences become `rk-cost-empty-case`;
  - every `$FIX_ITEM` becomes `$FIXTURE_ITEM`;
  - the line that opens `test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM (rk-fix-doctor-test-count at this pin)`
    becomes `FIXTURE_ITEM=rk-fix-doctor-test-count`;
  - one dated comment is added above that line.

  The PRECONDITION line keeps its shape and names the new item:
  `FAIL (PRECONDITION): rk-cost-empty-case owns $owned readings — …`.
- **Checkpoint 7h** (`runbook/phase-1/s7h-lane-completes.md`):
  - every `$FIX_ITEM` becomes `$FIXTURE_ITEM`;
  - the line that opens `test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item the lane ran` becomes two
    lines: `FIXTURE_ITEM=rk-fix-doctor-test-count`, and a guard that `.redkiln/items/$FIXTURE_ITEM.toml` exists;
  - one dated comment is added above them.

  Its ref reads are unchanged. `rk-run/rk-fix-doctor-test-count` carries exactly one `Story:` trailer,
  `redkiln-rs/rk-fix-doctor-test-count`, on `73c087a` [measured at `a6349b6`], and that is what its (2) and (3)
  count and parse.

The preamble's rule applies: *"A checkpoint that changed after it passed is provisional"*. So `PROGRESS.md` rows 7h
and 7i are marked `passed …, re-run pending (s8j re-point)`, and Checkpoint 8j re-runs both whole.

**(d) `rk-fix-doctor-test-count` stays the untouched fixture item, with a dated note, and its checkout is re-pointed.**
D21-42 18: *"stays the untouched fixture item, prose defect included, with a dated note"*. Its header opens *"Step 8
criterion (1), AC-473: the fix-lane self-hosting item"*, which is no longer true. A dated comment under it says
three things:

- the item is now the fixture item that Checkpoints 7h and 7i read;
- criterion (1)'s item is the one §8j.5 reserves;
- neither the item nor `rk-run/rk-fix-doctor-test-count` (at `73c087a`) is to be run, rebased or pruned.

**Only `#` lines change.** Checkpoint 8j asserts that the file's non-comment lines equal those at `ca8f387`, and that
the ref has not moved.

**The checkout under `.git` is re-pointed to `.redkiln/worktrees/` (D21-44).** D21-44: *"The one existing run-owned
worktree, `rk-fix-doctor-test-count` at `73c087a`, stays where it is. It is the undispatched fixture item (D21-42
18), and its re-point is s8j's."* The checkout is moved to the new root, and it is not removed. There are three
reasons:

- **At the new root it guards the item.** It carries no `RK_BASE` marker, because it predates D21-45 W1. So `s8e`'s
  re-entry refuses any run of the item by name. D21-45 W2: *"A tree without `RK_BASE`, or with commits the run ref
  lacks, is **refused, never removed**"*. With the registration gone, `s8e` §8e.15 records the opposite outcome:
  *"If it is gone …, the run creates `.redkiln/worktrees/rk-fix-doctor-test-count` on the existing branch and mints
  stage-bearing marks"*, which breaks 7h.
- **Nothing that reads the fixture depends on the checkout's path.** Checkpoint 7h reads the ref and
  `.git/redkiln/runs/`. Checkpoint 7i reads the store. The ref stays at `73c087a`.
- **It removes the last registration under `.git/redkiln/worktrees/`.** That is the placement D21-44 measured
  unwritable, and the one in which D21-40 found the doc screen reading a stale `runbook/` copy. At the new root, every
  walk over `.redkiln/` skips it (§8j.8 A5.3, A5.5; `s8e` §8e.10), and `.gitignore`'s `.redkiln/worktrees/` row keeps
  it out of `git status` [measured at `a6349b6`].

The re-point is `git worktree move .git/redkiln/worktrees/rk-fix-doctor-test-count
.redkiln/worktrees/rk-fix-doctor-test-count`. The operator runs it after five reads:

1. `git -C .git/redkiln/worktrees/rk-fix-doctor-test-count status --porcelain` is empty.
2. Its HEAD equals `rk-run/rk-fix-doctor-test-count`'s tip, `73c087a`.
3. `git worktree list --porcelain` shows no `locked` line for it.
4. `git check-ignore -q .redkiln/worktrees/rk-fix-doctor-test-count` succeeds.
5. Neither `.redkiln` nor `.redkiln/worktrees` is a symlink. That is W1's guard, applied by hand.

Reads 1–3 held at `a6349b6` [measured]: the tree was clean, HEAD was `73c087a82245…`, and it was not locked. If any
read differs, STOP. A dirty or advanced tree is not the fixture D21-42 18 describes. The worst-case path under the new
root is 204 characters: 64 for the prefix, plus 140 for ADR-0114's file name, which is the tree's longest path at
`73c087a` [measured]. That is under the 260-character limit. The ref and `.git/redkiln/runs/` are not touched.

This interacts with two other stages:

- Checkpoint 7f's old cleanup line is `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count`.
  It is deleted by `s8e`'s 7f amendment (`s8e` §8e.15). That file's finding 13 notes that `s8a` runs 7f first, and
  that `s8a`'s block restores the registration afterwards. So reads 1–3 are taken at this stage, not trusted from
  the measurement at `a6349b6`.
- `s8a`'s Checkpoint 8a restores the old registration only if it was present when its block began. After the
  re-point it was not, so a chain re-run leaves the moved tree alone.

**(e) Both lane items declare `tag-release`'s trial (handed on by `s8g`).** `s8g` §8g.7 defers *"The trial's REAL
run on this repository"* to this stage, because *"It needs a landed commit and a journalled BuildArtifact for its
item, which only a lane run produces"*. It also requires that AC-114's promotion *"must precede any lane run whose
release stage is to select `tag-release`"*. On this repository only `s8`'s runs land a commit. Each of those runs
forks with `.redkiln/catalog` unchanged since the CI-green frozen sha (§8j.8 A1's `lane_check`), so a promotion
cannot sit between them. This file therefore takes the following route:

- The reserved fix item (§8j.5) and the story item each carry `s8g`'s item-record trial declaration,
  `[trial] activity = "tag-release"`, in the spelling `s8g`'s record fixes.
- Each lane's release stage then admits `tag-release` from `.redkiln/catalog-extended/` for that item's one run.
  That run is the trial's real run. There are two trials, and each is one run of one activity.
- The §8j.4 proofs carry the same declarations, so the fixture lanes walk `release` through `tag-release` before any
  real run does.
- **AC-114's promotion is not an act of this stage.** It follows `s8`'s fix-lane run and cites that run's id. It
  moves `.redkiln/catalog` only after `s8`'s runs, so no fork straddles it.

🛑 **This is a reading of ADR-0114 decision 3, not something the decision states, and the ledger row says so.** The
decision's *"a one-run, one-activity trial the operator declares"* is read here as admitting one declaration per
lane item. If the operator reads it as one trial in all, the story item's declaration is dropped. The promotion then
moves ahead of the story run, with a CI re-run in between (§8j.7).

**AC-114's residue lint is this stage's, and it is owed with the promotion.** `s8g` §8g.7 assigns it here: *"AC-114's
residue lint does not exist, and its owner is `s8j`."* It gives two reasons:

- *"The lint's first non-vacuous input is the promotion decision that `s8j`'s real trial produces."*
- Built earlier, it *"would either fail all 102 shipped activities, none of which carries a `curation.decision` id,
  or ship with a grandfather rule that no record takes"*. `s8g` reports that backfill as *"a decision owed upward"*.

On this repository the real trial is `s8`'s run and not this stage's, as shown above. So this stage takes the
ownership but builds the lint in the promotion's commit, after `s8`'s fix-lane run. It builds the lint only once the
operator has taken the backfill decision. The ledger row names the lint, its owner and both preconditions. Until
then no line of this file claims AC-114.

### 8j.2 The human roots: one question per human kind, declared once for both lanes

D21-41 (f): *"IntakeRecord and BugReport stay `source = "human"` (D12 #15), supplied through a pack-declared gate
question whose option attaches the kind (ADR-0129 d3/d4); this is decided once for both lanes."* It is on this
stage's path because each lane's first activity needs its root:

- The fix lane's analysis activities `minimal-repro` and `exploitability-analysis` both carry
  `requires = ["BugReport"]` [measured].
- Three of the five Spec providers require IntakeRecord: `bdd-scenarios`, `use-case-spec` and `user-story-ac`
  [measured: their `requires` rows]. `bdd-scenarios` is the one the story pack's mandatory specification stage
  selects today (D21-43 N2).

**Who owns what (D21-43's integration calls).** *"The loader and `.redkiln/processes/` are **s8d**'s. The
pack-declared human-root questions are **s8j**'s. s8g's `.redkiln/questions.toml` is dropped in favour of s8d's
loader."*

- `s8d` §8d.6 builds the loader: *"This stage adds a TOML loader into the existing `QuestionRegistry::with_question`
  and `bind` seams, used by both `run` … and `answer`"*. Its file home is `.redkiln/processes/questions.toml` by
  default, and its record fixes that. It also defers this stage's part: *"`s8j` authors the pack-declared human-root
  questions for `IntakeRecord` and `BugReport` into `.redkiln/processes/`."* The loader *"reads
  `.redkiln/processes/*.toml` and does not recurse"*, so a run worktree under `.redkiln/worktrees/` is never under it.
- This stage builds no loader and no second registry file. It adds entries to the one file `s8d`'s record fixes,
  in that file's spelling.

**The work.**

1. **Two questions**, one per human kind the lanes demand. Each has one option whose `attaches` names its kind:
   - BugReport, for the fix lane's analysis stage;
   - IntakeRecord, for the story lane's specification stage.

   The ids are `root.bug-report` and `root.intake-record`, and the record may re-spell them by a dated correction
   here. Each question's text states what the attachment is for, and nothing about the item.
2. **The binding.** Selection's step-4 gate is `Unsatisfiable { kind, … }` with `NoAdmissibleProducer` on a
   `source = "human"` kind (AC-054: *"returns `Unsatisfiable { kind, failed_predicates }` and opens a gate"*). For
   each of the two kinds that gate is bound to its question through `s8d`'s `bind` route. After the binding,
   answering the gate with the attachment writes the root under ADR-0129 decision 4: *"accepted and stored as a
   human-supplied root"*.
3. **Present on re-invocation.** The packet (f) risk list names this as unmeasured: *"whether a re-invocation
   classifies the attached root as Present"*. The proofs in §8j.4 measure it.
4. **Each new gate class has an expiry row.** `s8h` refuses a run at load when a reachable gate class has no
   `[[gate_expiry]]` row (its `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load`). So each of the
   two questions' gate classes gets a fail-closed row in `s8h`'s table, which the operator signs row by row like the
   others (D21-42 21a; AC-276). This stage drafts the rows and does not sign them.

**The human-root files the real runs attach are committed now**, so that `s8`'s attach can be reproduced:

- `.redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md` states the intent from §8j.1(a) and nothing more;
- `.redkiln/intake/<fix-item>.BugReport.md` is written once §8j.5's pick is made.

`.redkiln/intake/` is a new directory that no reader scans. The record states the path convention, and Checkpoint
8j's whole-suite line proves the new directory disturbs nothing.

**The record** (next free id at writing, `proposed`, with ADR-0085's `### Option — <name>` headings per D21-42 24a)
decides three things:

- the two question ids and their options, in the spelling `s8d`'s record fixes;
- the gate-kind name the step-4 gate binds under;
- the path convention above.

**RT-59 is not this record's.** It was settled by R-48 on 2026-09-20, by a dated note on design §5's `artifact.write`
row: *"every activity output, **and `redkiln answer` for an admitted attachment**"*. `PROGRESS.md`'s hygiene recount
also says so: *"**RT-59** by its dated `artifact.write` producing-step note (R-48)"*. The record cites that note and
decides nothing about it. `answer.rs` still carries *"RT-59: that `answer` is this row's producer is ROUTED, not
decided"*. That comment is stale, and it is corrected by a dated comment-only edit in this stage's work commit.

The record **supersedes nothing**. Carrying out ADR-0129 decision 3's *"Pack-declared questions … are Step 7e's"* at
a later stage is execution order, not a moved Decision sentence.

**Tests, red first.** All are in `rk-cli` `--test it`, beside `s8d`'s `processes::` tests:

- `processes::the_two_human_root_questions_load_and_each_attaches_its_kind`. This is red today, because neither
  question exists.
- `processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question`. It checks both directions: every
  `source = "human"` kind that a selected activity of either lane requires without a `?` is attached by exactly one
  question, and every question's `attaches` names a `source = "human"` kind in `.redkiln/artifacts.toml`.
- `processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question`.

The behavioural half is §8j.4's `lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes`.

**Mutant `M-attach-dropped`.** The BugReport question's option loses its `attaches`, which is a content mutation and
not a loader mutation. It must turn `processes::the_two_human_root_questions_load_and_each_attaches_its_kind` and the
`lane_proof` attach test red.

### 8j.3 The staged first run: the analysis → specification hold (D21-42 19b)

**The mechanism: a human-authored hold on the item, answered like any other gate.**

1. The reserved fix item's record carries `hold_before = "specification"`. The key is spelt that way here; the
   record may re-spell it, and a re-spelling takes a dated correction in this file.
2. When the lane loop's `AdvanceStage { from, to }` would enter the held stage, and no answered gate for that hold
   exists on the item, the kernel **opens a gate instead of advancing**. The gate uses the default question
   (`approve` / `changes-requested`; ADR-0129 decision 3).
3. `run` exits 0. Golden constraint 2, `run` never waits, and ADR-0128's *"a run that STOPS AT A GATE exits 0"* both
   hold unchanged.
4. The operator reads the analysis stage's outputs through `redkiln explain <item>`.
5. The operator answers with `redkiln answer <item> approve --explain-sufficed <yes|no> --toml-opened <yes|no>`
   (ADR-0129 decision 1's grammar). The answer is spent (`gate_answer_spent`, ADR-0123).
6. The next `redkiln run <item>` enters `specification`. In `s8` that run goes through `tools/live-run.sh run`
   (§8j.8 A1). It is a new process, so the first real invocation also crosses a process boundary on purpose, under
   the record `s8e` wrote for D21-41 (a).
7. `changes-requested` does not advance the lane. The rule is D21-40 decision 2's: stop and report.

**Why this and not another route.**

- **A `run` flag.** ADR-0128 *"Forbids a second `run` grammar"*, and an environment variable read by `run` is the
  same flag under another name.
- **A per-item dispatch cap** (D21-42 19a). It stops wherever the count runs out, mid-stage and not at the boundary.
- **A pack-level gate at `specification`.** It would hold every fix-lane item for ever.

The hold is item-level, and it is human-authored in the checkout before the run. It is not something the run writes.
So ADR-0157's forbids bullet, *"Writing an item's advanced stage back to `.redkiln/items/` (AC-518)"*, is untouched:
nothing writes the item record.

**The hold's gate is a gate like any other.** Its transitions follow `s8e`'s git-first gate lifecycle. D21-43 N6:
*"Every gate transition (opened, answered, expired, voided) is a git-first tier-1 event"*, and the hold changes
nothing about that. Its gate class takes a fail-closed `[[gate_expiry]]` row in `s8h`'s table, as §8j.2's two do.
Without the row, `s8h`'s load lint refuses every run of the item.

**The record** (next free id at writing, `proposed`) covers four things:

- the item-record key;
- the gate cause;
- the rule that an answered hold never re-fires;
- the refusal of a hold that names a stage the item's pack does not order, at load and by name.

If `gate_opened`'s reason set is closed in `rk-types`, the new reason lands in the same change as its schema
snapshot. That is ADR-0116 decision 5's rule applied by analogy: *"The table row and the `rk-types` variant land in
the same change"*. The record supersedes nothing that this file could find. It states that finding and invites
refutation.

**Tests, red first.** All are in `rk-cli` `--test it`, over the §8j.4 fixture:

- `lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification`;
- `lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification`;
- `lane_proof::boundary_hold_changes_requested_does_not_advance`.

**Mutants.**

- `M-hold-ignored`: the hold is not consulted. It must turn the first test red.
- `M-hold-spent-refires`: an answered hold opens again. It must turn the second test red.

### 8j.4 Both lanes proven end to end under the FakeHarness — the fixture proofs D21-40 decision 2 gates on

**What is proved, and through what.**

- Every proof enters through `rk_cli::verbs::run::run`. The only exception is the binary arm, which enters through
  the built `redkiln` binary. No test calls `replay::resume` or any kernel seam directly.
- The harness is the FakeHarness. It is injected through the seam `s8e` added for its first end-to-end test (D21-42
  24c), and extended here where the lanes need more.
- The toolchain is the **real** `rk-adapter-toolchain-rust`. `detect()` chooses it over the run-owned worktree, as
  ADR-0137 decision 2 says. It is never `FixtureToolchain`, and never the `rk-fixture-toolchain` child binary.
  `[adapters] toolchain_programs` is left unset for Rust, so `program` is `cargo` (ADR-0137 decision 7).
- A proof run over a fixture toolchain would prove nothing about AC-473's *"the toolchain stage ran through
  `rk-adapter-toolchain-rust` behind the `Toolchain` port"*.

**Why a module of its own.** `s8e` owns `crates/rk-cli/tests/it/lane_e2e.rs`. Its tests run a lane variant of
`pack-min`, which is a fixture lane, and they are not bound to the real adapter. The proofs here are in
`crates/rk-cli/tests/it/lane_proof.rs`, module `lane_proof`, for two reasons:

- the fixture-toolchain floor in Checkpoint 8j can read one file that holds only proofs over the real adapter;
- `s8e`'s tests are not made to carry a constraint they were not written under.

**The fixture repository.** It is materialised per test into a temporary directory, as a fresh git repository:

- a one-crate Cargo workspace from `crates/rk-cli/tests/fixtures/lane/`, with a planted defect and no failing test
  committed;
- a **byte copy of the shipped `.redkiln/` tree, taken at test time**: packs, catalog, `catalog-extended`, cards,
  processes, artifacts, facets, roles and config. It is not a fixture pack, and not a committed duplicate that could
  drift. The proof runs over the registry `s8` will run, so a registry defect fails here and not at spend time. The
  copy includes `config.toml`'s `[project] name`, which `s8e` requires of *"every fixture repository a lane runs
  in"*;
- 🛑 **the copy skips `.redkiln/worktrees/`** (D21-44: *"a run worktree is a full repository copy"*). After the copy,
  the support code asserts that no directory named `worktrees` exists under the destination `.redkiln/`. A copy
  taken after `s8`'s runs would otherwise nest whole repositories into every proof;
- the repository is initialised through the product's own `init` path. Its `<commondir>/info/exclude` therefore
  carries the `# redkiln-runtime` block that `s8e` writes (D21-45 W4). AC-518's porcelain assertion below then
  measures the product's own exclusion of `.redkiln/worktrees/`, not a `.gitignore` written for the test;
- the two lane items, carrying the real items' facets, `slice`, `hold_before` (fix only) and `[trial]` declaration
  (§8j.1(e)).

Each test builds its crate in the run-owned worktree's own `target/` (ADR-0137 decision 3), so a proof costs real
`cargo` time. That is the price of the real adapter. The run-owned worktree is at
`<fixture>/.redkiln/worktrees/<item>` (D21-44). The first proof asserts that it lies there, and that no path under
the fixture's `.git` is a registered worktree.

**The scripted harness.**

- Each scripted response is keyed by `(activity@version, step id, iteration path)`, which is D21-42 3b's step-grain
  key without its item. An `on_false` loop-back re-dispatches under a new iteration path (D21-42 4d) and gets its
  own response.
- Each response carries a closed output that validates against the `rk-types` `EXPORTED` schema `s8h` produced.
- **An unscripted dispatch fails the test by name, and there is no default response.** So the set of steps that
  dispatched is asserted, not assumed.
- The implementer's scripted response carries the file writes the double performs **inside the run-owned
  worktree**: a failing test at `red`, and the fix at `green`. The Rust adapter then observes a red suite, and after
  that a green one, through `Toolchain::test`'s outcome report (D21-42 1a).
- **Every gate the lane opens is answered with its question's first option**, as `s8d`'s walk does. That includes
  AC-079's `unknown_facet` gate for `change` at triage (D21-43 N3: *"`null` routes to AC-079's `unknown_facet` gate at
  triage, and the operator supplies it"*). The proofs assert the set of gates opened. They do not assume it.

**The ten `lane_proof` tests.** All are in `crates/rk-cli/tests/it/lane_proof.rs` and run under `rk-cli`
`--test it`. Each is written red first, and each was watched fail for the named reason before it was made green.

| test | asserts |
| --- | --- |
| `lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness` | From `triage`, the fix lane reaches a kernel-made content checkpoint commit on `rk-run/<item>` (D21-41 (b)). The commit's tree differs from the fork point under the crate's `src/`. The run-owned worktree is `<fixture>/.redkiln/worktrees/<item>` (D21-44). AC-518: the fixture's own checkout is unchanged. `git status --porcelain` is empty, `git diff-index --quiet HEAD` passes, and HEAD has not moved. |
| `lane_proof::fix_lane_trailers_every_non_forbidden_stage_in_order` | Git's own parser reads the stage tokens of the `Story:` trailers on `<fork>..rk-run/<item>`, oldest first (`%(trailers:key=Story,valueonly=true)` with `--reverse`), taking the last whitespace token of each value. Those tokens equal `bug-standard`'s `stage_order` filtered to `stages[s] != "forbidden"`, followed by the terminal `closed` token of D21-43 N9's *"`Story: <project>/<item> closed` mark"*. The comparison is set **and** order, and it includes a `stage_empty` stage (D21-42 7). |
| `lane_proof::fix_lane_verify_completed_names_the_rust_adapter` | At least one `verify_completed` row carries the item in its `item` member, and every such row's `adapter` member is `rk-adapter-toolchain-rust`. The member names are `s8g`'s (D21-42 10b). |
| `lane_proof::story_lane_spec_is_landed_and_journalled` | The story lane's `Spec` is written by `rk-store` under the item's managed artifact directory (D21-42 5b). `git cat-file -e` finds it on `rk-run/<item>`. The item's **last** `artifact.write` row with `kind = "Spec"` carries a `digest` equal to the sha256 of the landed blob. That is ADR-0116 decision 3, *"not tree membership"*. The producing activity is read from the run's own record of what the specification stage selected, and never typed. It is `bdd-scenarios` at authoring (D21-43 N2), and the test does not name it. |
| `lane_proof::story_lane_trailers_every_non_forbidden_stage_in_order` | As the fix-lane twin, over `story`'s order. The whole lane runs, not only up to the Spec. (1b) asserts *"every story-lane stage"*, and a proof that stopped at `specification` would leave `implement`, `verify`, `review` and `release` unproven for the story pack. |
| `lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes` | The first invocation stops at the step-4 gate on BugReport or IntakeRecord and exits 0. `answer --attach` through §8j.2's question succeeds. The re-invocation classifies the root as Present. |
| `lane_proof::gate_then_answer_then_second_process_continues_binary_arm` | **The multi-process arm.** It uses `env!("CARGO_BIN_EXE_redkiln")`, with `[adapters] harness_binary` set to the `rk-fixture-child-cli` child, extended to replay a per-dispatch script. It also carries a fixture launch record, because `s8h` §8h.7 requires one of *"Binary-level tests that drive the real harness at the fixture child"*. Each `run` stops at a gate, the gate is answered, and the next `run` is a new process. The gates come in the order the lane reaches them, and they include triage's `unknown_facet` gate for `change`, the human-root gate and the §8j.3 hold. The last `run` reaches the landing. **That is at least five separate processes.** The lane continues across each boundary from the stage-bearing trailer (D21-41 (a)). The final trailer sequence equals the in-process arm's. |
| `lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification` | see §8j.3 |
| `lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification` | see §8j.3 |
| `lane_proof::boundary_hold_changes_requested_does_not_advance` | see §8j.3 |

🛑 **A one-hop fixture cannot judge a multi-hop guard.** The lanes cross nine and ten boundaries, and the binary arm
crosses at least four process boundaries. A green that held on the first hop would prove nothing about the fifth.
That is why the stage lists are compared **whole**, and why the multi-process arm ends at the landing and not at the
first successful re-entry.

**Mutants.** Each is run from Python, never the shell. Each must flip the named test red, and a survivor blocks.

| mutant | the change | must redden |
| --- | --- | --- |
| `M-seed-record` | `through_step_seven` seeds the stage from the item record, ignoring the reconciled trailer | `…_second_process_continues_binary_arm`, `…_answered_lets_the_next_process_enter_specification` |
| `M-land-empty` | the landing commits `allow_empty` with nothing staged | `fix_lane_reaches_a_landed_content_commit_under_fake_harness` |
| `M-fixture-toolchain` | the composition root builds `FixtureToolchain` | `fix_lane_verify_completed_names_the_rust_adapter` |
| `M-skip-empty-stage-trailer` | a `stage_empty` stage mints no trailer | both `…_trailers_every_non_forbidden_stage_in_order` |
| `M-spec-digest-pre-render` | `artifact.write`'s digest is taken over the pre-render value, not the written bytes | `story_lane_spec_is_landed_and_journalled` |
| `M-worktree-under-git` | the run-worktree root is derived under the common dir again | `fix_lane_reaches_a_landed_content_commit_under_fake_harness` |
| `M-attach-dropped`, `M-hold-ignored`, `M-hold-spent-refires` | see §8j.2 and §8j.3 | as named there |

⚠️ **A fixture per function is not a fixture per branch.** Some proofs turn on a branch: `stage_empty` against a
stage that selected activities, or `approve` against `changes-requested`. Each branch then has its own assertion.
The mutant table exists to show that each branch can be observed.

🛑 **What this section does not claim.**

- It does not claim AC-473 or AC-521. Those are claimed by real runs on this repository, which are `s8`'s.
- It does not claim that the review gate is guarded against false approval. ADR-0101 clause 5 forbids that until
  S-SUBTLE-CB runs.
- It describes cross-process continuation only as far as the record `s8e` wrote for D21-41 (a) permits. ADR-0157
  decision 5 stands wherever that record did not supersede it.

### 8j.5 The fix item: a shortlist, the operator's pick, a reservation, and a fresh item (D21-42 18)

D21-42 18: *"a shortlist of **at least three behavioural defects**: reproducible by a failing `cargo test`, one crate,
not in machinery the run exercises, and not scheduled for a sub-stage fix. The operator picks one, it is reserved in
the ledger, and a fresh item is minted."*

**The survey method.**

1. **Exclude what the run exercises.** Drop every function reachable from the call graphs of `verbs::run::run`,
   `verbs::answer`, the lane's `rk-kernel` runtime, replay, fsm and selection, all of `rk-git`, every `rk-adapter-*`,
   `rk-containment`, `rk-store`'s render and read path (`s8f`), and the verbs `s8`'s blocks read (`explain`'s stage
   and output paths, `cost`, `doctor --rebuild`).
   - The check is **static**: a caller walk up to those entry points.
   - It is also **dynamic**: with the candidate's repro test added, all ten `lane_proof` tests stay green. That is
     necessary, not sufficient.
   - `s8f`'s render path puts `rk_store::locator` on the run path. At every entry, the drift pass in `s8f` §8f.5
     *"locates the section by anchor (the locator's byte range)"*. So C2 and C6 are **excluded** under this rule.
     Once `s8f` has landed, the callers of every remaining `rk-store` candidate are walked again.
2. **Exclude what is scheduled.** Drop any candidate whose file or function is named by `s8a`…`s8i`, or listed among
   their owned decisions in the plan. For example, doctor's journal source and `RK-D-HEADROOM-MARGIN` are D21-42 12c,
   and rk-store's `conn.rs` poison unwrap is L3.
3. **Reproduce in a throwaway tree, and never in the crate.** Extract HEAD into `target/` (`git archive`), append the
   repro test to the named module there, and watch it fail for the named reason. 🛑 **The repro test is not committed
   into any crate.** If it were, it would turn CI red, and it would hand the lane's `red` step its answer. It is
   committed as `.redkiln/intake/<fix-item>.repro.rs`, with `.redkiln/intake/<fix-item>.repro.toml` giving its
   `crate`, `module`, `test` and the `expect` substring of its failure. The BugReport the real run attaches points at
   both.
4. **One crate, low blast radius.** The fix touches one crate's `src/`, in about one function.
5. **Present at least three to the operator.** For each, give the file, the quoted code, the contract it breaks
   (quoted from its own doc, an ADR or the register), the red output from step 3, and why it is outside the run.

**Candidates found by reading the code at `ca8f387`.** These are **candidates only**. Each was checked by tracing the
code, and none has yet been run red. Step 3 runs each one before it is presented.

| # | crate · function | the defect (quoted code) | the contract it breaks | repro test (proposed) | caveat |
| --- | --- | --- | --- | --- | --- |
| C1 | `redkiln-migrate` · `legacy::fence::split` | `let tail = src[after + close..].trim_start_matches("\n```");`. `trim_start_matches` strips the pattern **repeatedly**, so a fence that directly follows the closing one loses its opening backticks. | `prose` is *"Everything outside it"* (the `Fenced` doc). Over a yaml fence followed directly by a `diff` fence, `prose` should begin with the second fence's three backticks. It begins with the bare word `diff`. | a new test in `redkiln-migrate` `tests/it`, over `split` | Only `LegacyLedger.prose` carries it, so the impact is low. It is the migrator, which is outside the run. |
| C2 | `rk-store` · `locator::scan` | `let is_fence = trimmed.starts_with("```") \|\| trimmed.starts_with("~~~");` is judged on the **trimmed** line and **before** the HTML-comment branch. So an indented ```` ``` ```` (four or more spaces), or an unmatched ```` ``` ```` inside `<!-- … -->`, opens a fence, and every later heading becomes invisible. | The module's own rule, under AC-150: a heading-looking line *"inside a fenced code block, an indented block, an HTML comment … is content"*. It says so in terms: *"An indented block is code, not a heading."* | `locator::an_indented_fence_marker_is_not_a_fence` and `locator::a_fence_marker_inside_an_html_comment_is_not_a_fence` in `rk-store` `tests/it`. Each resolves `## Target` after the decoy, and each gets `Occurrences { count: 0 }` today. | **EXCLUDED under step 1.** At `a6349b6`, `locator::` has no caller outside its own test file, but `s8f` §8f.5's render path puts it on the run path. |
| C3 | `rk-cli` · `verbs::explain::top_level_conjuncts` / `false_conjunct` | `if depth == 0 && src[i..].starts_with(" and ")`. Depth counts only `( [ ) ]`, so the split cuts inside a quoted string, and it cuts `a or b and c` into `a or b` and `c`. | Its own doc: *"A disjunction is **not** decomposed — no single disjunct of a false `or` is "the" reason, so naming one would be inventing a cause."* | a `#[cfg(test)]` unit in `explain.rs` (the function is private). `false_conjunct("kind == 'x and y'", …)` returns `kind == 'x`, not the whole predicate. | `explain` is a read-only verb. `s8` reads its stage output, and this path runs only when explaining a false predicate. |
| C4 | `rk-cli` · `doctor::rules::margin` (`RK-D-MARGIN`) | `let Some(margins) = table(&pack, "margins") else { continue; };` and `margins.get(window).and_then(toml::Value::as_integer)`. A pack with no `[margins]`, with a missing window, or with a non-integer margin passes silently. | AC-377: *"AND `redkiln doctor` reaches the same verdict without dispatching a model call"*. Its Verified-by includes *"a fixture config with no margin that"* must fail. | a doctor fixture under `crates/rk-cli/tests/fixtures/doctor/` with `[margins] five_hour = 10` and no `seven_day`. It should produce an `RK-D-MARGIN` error, and today it produces nothing. | This is not `RK-D-HEADROOM-MARGIN`, which is D21-42 12c's. Re-verify step 2 against `s8f`'s file. |
| C5 | `rk-types` · `schema::lint::walk` | `Value::Object(_) => walk(child, &p, child_inside_union && key != "properties", out)` resets the inside-union flag under every `properties` map. So a union under an object that is itself inside a union is never flagged. | AC-131's instrument is the `NestedUnion` rule, *"A union with a union descendant (AC-131)"*. | `schema_no_oneof::nested_union_through_properties_is_caught` | **Lower confidence.** The `key != "properties"` exclusion is explicit and may be deliberate. Read its history before presenting it. |
| C6 | `rk-store` · `locator::parse_heading` | `text: rest.trim().trim_end_matches('#').trim()` strips a trailing `#` that has no space before it, so `## C#` resolves as `C`. | The locator's ATX reading, in which a closing `#` run needs a preceding space. | `locator::a_trailing_hash_without_a_space_is_heading_text` | **EXCLUDED under step 1**, as C2 is. |

The survey's pass also found three candidates that the method **excludes**:

- `rk-types` `Digest`'s derived `Deserialize` skips `from_hex`'s validation. Its doc says *"The only constructor: 64
  lowercase hex characters"*. But digests are deserialised on the run and replay paths, which fails rule 1.
- `rk-kernel` `toml_subset` admits a table defined twice.
- `catalog::walk` drops a non-string `deviation_ok` entry silently.

The last two sit on the pack and catalog load path the run takes.

**What is presented.** Four candidates remain: C1, C3, C4 and C5. That meets the floor of three. C5 carries its
lower-confidence caveat, and C4 carries its re-check against `s8f`'s file.

**Reservation and mint.**

1. The operator picks one candidate.
2. The pick is reserved by a dated ledger entry. The entry carries one machine-readable line,
   `reserved-fix-item: <id> <crate>:<function>`, and it forbids fixing the defect by hand or in any stage before
   `s8`.
3. A fresh `.redkiln/items/<id>.toml` is minted with an `id` of the form `rk-fix-<slug>` and these fields:
   - `kind = "bug"`, `stage = "triage"`;
   - `hold_before = "specification"` (§8j.3);
   - `slice = "<slug>"`: the id without its `rk-fix-` prefix, so never the id itself (`s8e` §8e.8);
   - `[trial] activity = "tag-release"` (§8j.1(e));
   - `[facets]`: `cos = "standard"`, `has_repro = true`, and low `criticality`, `blast_radius` and `size`.
4. The item's BugReport (§8j.2) and its repro pair (step 3) are committed with it.
5. **Neither `rk-run/<id>` nor `.redkiln/worktrees/<id>` exists at this stage's close.** The run ref forks at `s8`'s
   first `run`, from a HEAD descending from the CI-green frozen commit (§8j.7). `s8`'s amended (1) asserts that
   ancestry, and it asserts that the code paths are unchanged between the two.

### 8j.6 The probe session: the pin move, Checkpoint 7c whole, and `s8h`'s two live probes — paid, after the fixture proofs, each an operator go-ahead

**Entry condition.** Every `lane_proof` test and every mutant cycle in §8j.4 is green, and that is recorded. D21-40
decision 2 authorises the spend *"once the sub-stages' fixture proofs pass, and not before."*

🛑 **Who runs it, and where.** The operator runs every step below in one sitting, the probe session. They run from a
plain terminal, or through the operator's own `! python …` invocation, and never from a shell an agent spawned.
Every dispatch goes through the committed launcher `s8h` authored, `tools/live-run.sh` (D21-42 17a: *"A committed
launcher script is the only entry to a real run"*), against `~/.claude-personal` (17b). A Claude Code session
exports `CLAUDECODE`, `CLAUDE_CODE_*`, `CLAUDE_PID` and `CLAUDE_EFFORT`, and the launcher refuses each of them
(`s8h` §8h.7, step 2). That refusal is the design working, not an
obstacle to route around (Halt 7). D21-44 records the same wall from the other side: *"the session's permission layer
refused to spawn a `claude` child from inside Claude Code"*. **The operator records each result** in
`runbook/phase-1/s8j-evidence.txt` (format below) as it happens. No agent writes a `pass` line. If `s8h`'s launcher
offers no entry for a dispatch below, **STOP and raise it**. Do not run the dispatch bare.

**The order, and what each step spends.**

1. **The freeze and the pin move, by `s8h` §8h.8's steps 1–3, which are not restated here.** `s8h` authored the
   procedure and says it is *"executed at `s8j`"*. Its step 4 is the `live_dispatch_probe` dispatch, and step 2
   below takes that dispatch as part of Checkpoint 7c's whole run, so the probe is not dispatched twice. Its step 5
   is steps 3 and 4 below. Steps 1–3 cover four things:
   - the operator's `DISABLE_AUTOUPDATER` setting;
   - the instrument file re-written by `printf`, never by `tee`;
   - the constant, its prose, its literal test and every `claude_code_version` fixture, moved together;
   - `PERMISSION_PROMPTS_FLOOR` asserted below the pin.

   If `claude --version` already equals `TESTED_CLI_VERSION` (`"2.1.278"` at `a6349b6` [measured]), the move is
   recorded as unneeded. The mismatch policy stays `MismatchPolicy::Refuse`, hard-coded in `real_harness_config`
   (`crates/rk-cli/src/verbs/run.rs` [measured]). That is why the pin moves before any dispatch. Under D17-11's
   amendment, *"the adapter refused all three dispatches at the init line"*.
   **Spend: none.**
2. **Operator go-ahead, then Checkpoint 7c, run whole and unamended, as the launcher's `wrap` child (D21-43 N28).**
   `s8a` makes 7c provisional and says *"its whole re-run is `s8j`'s (D21-43 N28)"*. D21-43 N28: *"**s8j re-runs it
   whole**, from a plain terminal through the launcher, with the live probes."* `s8h` §8h.7 built the entry for
   exactly this: *"`wrap` is the entry for a child that does not dispatch through `rk-adapter-claude`"*. Its second
   caller is named there, *"`s8j`'s whole re-run of Checkpoint 7c"*. 7c's probe does dispatch through the adapter,
   but `s8h` §8h.7 names it a `wrap` caller anyway. Two things still bind the version: the adapter's own `Refuse` policy,
   and the block's pin guard.
   - The block is extracted verbatim from `runbook/phase-1/s7c-adapters.md`, the last fenced block that echoes
     `CP7C-END-OF-BLOCK reached`, into a file under `target/`.
   - It runs as `tools/live-run.sh wrap -- bash <that file>`.
   - The launcher sets `CLAUDE_CONFIG_DIR` and `DISABLE_AUTOUPDATER`, writes the launch record, and exports
     `REDKILN_LAUNCH_RECORD` to the child (steps 4, 5 and 8). The block's own bare line,
     `cargo test -p rk-adapter-claude --features live -- --ignored --nocapture live_dispatch_probe`, therefore passes
     the probe's first statement: *"The three `live::` probe tests assert the same variable as their first
     statement"*.
   - Under `wrap`, step 6 records the CLI version and does not compare it. The block compares it itself, against the
     pin, as Halt 2's guard.
   - Nothing in `s7c-adapters.md` is edited. The block passes only if it reaches its end marker, and the launcher's rc
     is the child's.

   **This one whole run is also the `live_dispatch_probe` re-run** that D21-42 16 asks for after a pin move, so the
   probe is not dispatched a second time. **Spend: one haiku dispatch.** `PROGRESS.md` row 7c is updated only from
   this run.
3. **Operator go-ahead, then `tools/live-run.sh probe live::live_confinement_and_denial_probe`.** This is `s8h`'s
   probe 1 (D21-42 21d), and its pass condition is `s8h` §8h.11's. That condition includes the edit *inside* a
   worktree under `.redkiln/worktrees/`, which D21-44's N1b arm measured writable. 🛑 If the CLI cannot enforce the
   deny rules, **the work HALTS** and the gap is reported as a containment finding. D21-42 21d takes this in exactly
   these words.
   **Spend: one Sonnet dispatch.**
4. **Operator go-ahead, then `tools/live-run.sh probe live::live_init_equality_probe`.** This is `s8h`'s probe 2
   (21b): the tools, mode, model and auth are read from the init line.
   **Spend: three dispatches, one per pinned model id** (D21-43 N17 Q4).
5. **A failed or unexpectedly refused step stops the stage.** Nothing is re-spent until the operator has read the
   report. That is D21-40 decision 2's rule: *"stop and report, not to spend again"*.
6. **The (g) record flips only after both probes pass.** D21-41 (g): *"The record is accepted only after two live
   probes pass, and those probes run after the fixture proofs."* That record is `s8h`'s Record A. The flip is the
   operator's dated ledger line. Only the frontmatter, the README index cell and the census move.

The session spends five dispatches in all. Each launch leaves its launch record under
`<git-common-dir>/redkiln/launches/` (`s8h` §8h.7, step 8). Its basename goes on the evidence line, so Checkpoint 8j
can find it.

**The evidence file** is `runbook/phase-1/s8j-evidence.txt`. It is **not under `spikes/results/`**, because
`ci.yml`'s AC-465 job fails on any tracked path there except `RUNBOOK-instrument.txt`. It is space-separated, holds
one fact per line, and is appended in execution order:

```
pin-move <old-version> <new-version|unchanged> <yyyy-mm-dd>
checkpoint-7c <cli-version> pass <yyyy-mm-dd> <launch-record basename>
probe live::live_confinement_and_denial_probe <cli-version> pass <yyyy-mm-dd> <launch-record basename>
probe live::live_init_equality_probe <cli-version> pass <yyyy-mm-dd> <launch-record basename>
record-g <four-digit id of the record s8h wrote for D21-41 (g)>
legacy-worktree-repointed rk-fix-doctor-test-count 73c087a <yyyy-mm-dd>
ci <github-run-id> <40-hex head sha> <yyyy-mm-dd>
fix-item <reserved id> <40-hex frozen sha>
story-item rk-story-harden-spike-pack <40-hex frozen sha>
```

### 8j.7 CI green on the pushed branch — every job, both OS legs (D21-42 14a)

D21-42 14a: *"`ci.yml` green on the pushed branch, every job, both OS legs, is a Checkpoint 8 condition. Each push
is confirmed with the operator."* The branch has never been pushed (`git branch -r` shows `origin/main` only), and
`ci.yml` has never run. Its own comments say so: *"ARMED, NOT DISCHARGED … this workflow is authored and has never
run."*

**The known missing step.** In the `gates` job, before `test`, add the step below. The warrant is `992d257`'s message,
*"the first fresh run failed 10 rk-adapter-toolchain-node tests on the documented pnpm preflight (node_modules is
git-ignored), and ci.yml has no pnpm step at all"*.

- `pnpm` is installed through `npm` at the version that wrote `pnpm-lock.yaml`: `lockfileVersion: '9.0'`, and pnpm
  10.33.0 as measured on this box by `s7c`.
- No marketplace action is added. `ci.yml`'s own supply-chain stance is *"NO MARKETPLACE ACTION beyond
  `actions/checkout`"*.

```yaml
      # D21-42 14a (s8j §8j.7): the Node toolchain adapter's fixture project needs node_modules before
      # `cargo test --workspace`, or ten rk-adapter-toolchain-node tests fail at tests/it/support.rs's
      # preflight (992d257). pnpm comes from npm at the version that wrote pnpm-lock.yaml -- no
      # marketplace action -- and --frozen-lockfile makes a lockfile drift fail HERE.
      - name: node fixture dependencies (pnpm, frozen lockfile)
        shell: bash
        run: |
          node --version
          npm install --global pnpm@10.33.0
          pnpm --version
          pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project
          test -e crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules/.bin/vitest \
            || { echo "FAIL: no node_modules/.bin/vitest after pnpm install -- the node suite would fail at its preflight"; exit 1; }
```

**Whatever else a first CI run needs is found by running it, and it is not guessed at here.** The loop:

1. 🛑 Get the operator's go-ahead, then `git push -u origin build/phase-0-to-1` (the first time) or `git push`.
2. Read the run with `gh run view <id> --json conclusion,headSha,headBranch,jobs`.
3. For each failing job, diagnose from the log. A code defect is fixed red-first, in its own commit. A test that
   fails on one OS and not the other is a **finding**, never a `#[cfg_attr(…, ignore)]`: AC-121's cross-OS half is
   the reason the matrix has two legs.
4. 🛑 Get the operator's go-ahead, then push again.

The ARMED comments in `ci.yml` get dated, comment-only corrections once their job has run green. The history of the
ARMED reading is kept, not deleted.

**The frozen commit.** The last green run is taken over the commit that closes this stage's work, after §8j.6's pin
move. Its id and head sha go into `s8j-evidence.txt` as `ci …`. The `fix-item` and `story-item` lines carry the
same sha as their frozen sha. The tracker commit that records them changes no code path, and Checkpoint 8j asserts that.

### 8j.8 `s8`'s instrument amendments — the replacement text

This stage owns the plan's list of `s8` amendments, and this section is the deliverable. It also takes one
amendment the plan does not list, which `s8h` §8h.7 hands on: every `redkiln run` in `s8` goes through
`tools/live-run.sh run` (A1, A7). Once that lands, `s8` runs from a plain terminal, because the launcher refuses
a Claude-spawned shell. At this stage's execution each replacement is applied to
`runbook/phase-1/s8-exit-criterion.md` with a dated comment, and Checkpoint 8j proves it landed. This file does not edit `s8` while it is being authored.

**Bindings.** `verify_completed`'s two members are bound here, from `s8g`: *"The two member names, `item` and
`adapter`, are **fixed here** so `s8j`'s probe binds to them rather than to a guess."* One set of names is still
unknown at authoring: the catalog tests over `.redkiln/` that `s8b` names. Its draft names
`rigor::rigor_monotonicity`, `quality_block::d2_named_form_refuses_any_member_list`,
`lattice::p6_lattice_over_the_shipped_pack` and `lattice::p6_validate_over_the_shipped_pack`, among others. The set
is written below as a `BIND:` value, and the block that carries it refuses to run while it is unbound. When the
amendment is applied, the value is replaced with the names `s8b`'s file fixed. Checkpoint 8j (8) asserts two
things: `s8` carries no `BIND:` anywhere, and every test name `s8` selects exists in `cargo test --workspace --
--list`.

**A0 — the header row.** Replace `` | `s8` | Phase 1 | Checkpoint 7i | Checkpoint 8 | `s7i` | — | `` with
`` | `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — | ``.

**A1 — criteria (1) and (1b), the whole fenced block.** This one replacement covers seven items:

- build first;
- both run lines through `tools/live-run.sh run` (`s8h` §8h.7; D21-42 17a);
- the run's trailers scoped to its own fork, which descends from the frozen sha;
- every lane stage asserted as **set and order** equality (D21-42 7), ending in D21-43 N9's terminal `closed` mark;
- the toolchain identity read from `verify_completed` (D21-42 10b);
- the landed content change;
- (1b)'s digest read from `artifact.write` against the landed blob, with `git cat-file -e` on the run ref (ADR-0116
  decision 3; D21-42 5b). `git ls-files --error-unmatch` is dropped, because it is the tree-membership assertion
  ADR-0116 decision 3 rejects.

⚠️ **The plan says "scoped to commits made by this invocation"**, and that scope is too narrow. §8j.3's hold alone
makes the fix lane at least two invocations, and a set-equality over one invocation's commits would fail a correct
lane. The scope is therefore the run's own fork, `merge-base(HEAD, rk-run/<item>)`. The fork must be ahead of that
point, and must descend from the frozen sha. The fork also rules out the stale-ref shape `s7h` found, *"0 commits
ahead"*.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1; Checkpoint 8j proves it landed). Build first (golden constraint 3). Run from a
# plain terminal: every run goes through s8h's launcher, which refuses a Claude-spawned shell.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — criterion (1) NOT RUN (Halt 8)"; exit 1; }
# verify_completed's member names, fixed by s8g (D21-42 10b): `item` scopes a row, `adapter` names the chosen adapter.
VC_ITEM_KEY=item
VC_ADAPTER_KEY=adapter
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent — s8h's launcher is the only entry to a real run (D21-42 17a)"; exit 1; }
grep -qxF 'name = "redkiln-rs"' .redkiln/config.toml || { echo "FAIL: [project] name is not redkiln-rs — the Story: value's <project> would not be what this block compares"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — the frozen sha this run must descend from is unrecorded"; exit 1; }
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
# The expected stage list, derived from the pack and never typed (D21-42 7): stage_order minus forbidden.
order='import sys,tomllib; p=tomllib.load(open(sys.argv[1],"rb"))["policy"]; print(" ".join(s for s in p["stage_order"] if p["stages"][s]!="forbidden"))'
# The lane's stage tokens: git's own trailer parser, oldest first, the LAST token of each Story: value; every value
# must be exactly `<project>/<item> <stage>` (the stage-bearing form of D21-41 (a)), and the last is N9's `closed`.
lane_check() {   # $1 item  $2 pack file — prints nothing on success, the defect otherwise
  exp=$(python -c "$order" "$2"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$exp" ] || { echo "the stage order could not be read from $2 (rc=$rc)"; return 1; }
  exp="$exp closed"   # D21-43 N9: a complete lane ends in the terminal 'Story: <project>/<item> closed' mark (s8e)
  git rev-parse --verify --quiet "refs/heads/rk-run/$1" >/dev/null || { echo "no rk-run/$1 ref — no run"; return 1; }
  base=$(git merge-base HEAD "rk-run/$1") || { echo "rk-run/$1 shares no history with HEAD"; return 1; }
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {print $3}' "$ev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $ev"; return 1; }
  git merge-base --is-ancestor "$frozen" "$base" || { echo "rk-run/$1 forked from $base, which does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" "$base" -- crates Cargo.toml Cargo.lock rust-toolchain.toml .redkiln/packs .redkiln/catalog .redkiln/catalog-extended .redkiln/cards .redkiln/processes .redkiln/artifacts.toml .redkiln/registry.toml tools || { echo "code changed between the CI-green frozen sha and the run's fork — re-run CI and re-record (s8j §8j.7)"; return 1; }
  ahead=$(git rev-list --count "$base..rk-run/$1") || { echo "rev-list failed"; return 1; }
  [ "$ahead" -ge 1 ] || { echo "rk-run/$1 is $ahead commits ahead of its fork — a stale ref is not a run"; return 1; }
  lg=$(git log --reverse "$base..rk-run/$1" --format='%(trailers:key=Story,valueonly=true)') || { echo "git log over the run failed"; return 1; }
  bad=$(printf '%s\n' "$lg" | awk -v want="redkiln-rs/$1" 'NF && ($1!=want || NF!=2)')
  [ -z "$bad" ] || { echo "Story: values not of the form redkiln-rs/$1 <stage>: $bad"; return 1; }
  got=$(printf '%s\n' "$lg" | awk 'NF {printf "%s%s", sep, $NF; sep=" "}')
  [ "$got" = "$exp" ] || { echo "stages: expected '$exp', the run's trailers say '$got' — if explain shows an open gate, answer it and re-run this block (s8j §8j.3's hold is the first); a gated lane is not failed, and it is not a pass"; return 1; }
}

# (1) — AC-473, the fix lane
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item s8j §8j.5 reserved — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
test -f ".redkiln/items/$FIX_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIX_ITEM.toml does not exist"; exit 1; }
tools/live-run.sh run "$FIX_ITEM" --lane fix || { echo "FAIL: the fix-lane run exited non-zero"; exit 1; }   # through s8h's launcher (D21-42 17a); a gate-open exits 0 (C-01 D-01); lane_check distinguishes
out=$(./target/debug/redkiln explain "$FIX_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$FIX_ITEM" .redkiln/packs/bug-standard.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1): $why"; exit 1; }
base=$(git merge-base HEAD "rk-run/$FIX_ITEM") || { echo "FAIL: no merge base"; exit 1; }
git diff --quiet "$base" "rk-run/$FIX_ITEM" -- crates; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL (1): the run landed no content change under crates/ (rc=$rc) — trailers without a landed commit (D21-41 (b))"; exit 1; }
vc='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",("verify_completed",))]; a=sorted({str(x.get(sys.argv[4])) for x in r if x.get(sys.argv[3])==sys.argv[2]}); print(" ".join(a))'
ids=$(python -c "$vc" "$store" "$FIX_ITEM" "$VC_ITEM_KEY" "$VC_ADAPTER_KEY"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the verify_completed probe did not run (rc=$rc)"; exit 1; }
[ "$ids" = "rk-adapter-toolchain-rust" ] || { echo "FAIL (1): the verify stage's toolchain identity is '$ids', not exactly rk-adapter-toolchain-rust (AC-473; D21-42 10b)"; exit 1; }
printf '%s\n' "$out"; git log "$base..rk-run/$FIX_ITEM" --format='%h %s%n%(trailers)'   # → READ: no stage performed by the old redkiln; the landed commit is the fix. The run id is the evidence.

# (1b) — AC-521, the story-lane twin
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM (rk-story-harden-spike-pack)"; exit 1; }
test -f ".redkiln/items/$STORY_ITEM.toml" || { echo "FAIL: .redkiln/items/$STORY_ITEM.toml does not exist"; exit 1; }
tools/live-run.sh run "$STORY_ITEM" --lane story || { echo "FAIL: the story-lane run exited non-zero"; exit 1; }   # through s8h's launcher (D21-42 17a)
out=$(./target/debug/redkiln explain "$STORY_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$STORY_ITEM" .redkiln/packs/story.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1b): $why"; exit 1; }
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the Spec's path under the item's managed artifact directory (D21-42 5b)"; exit 1; }
git cat-file -e "rk-run/$STORY_ITEM:$AUTHORED_STORY" || { echo "FAIL (1b): $AUTHORED_STORY is not on rk-run/$STORY_ITEM"; exit 1; }
git cat-file blob "rk-run/$STORY_ITEM:$AUTHORED_STORY" > "$tdir/spec.blob" || { echo "FAIL: could not read the landed Spec blob"; exit 1; }
landed=$(sha256sum "$tdir/spec.blob") || { echo "FAIL: sha256sum failed"; exit 1; }
landed=${landed%% *}
aw='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? ORDER BY id",("artifact.write",))]; d=[x.get("digest") for x in r if x.get("kind")=="Spec" and x.get("item")==sys.argv[2]]; print(d[-1] if d else "")'
dg=$(python -c "$aw" "$store" "$STORY_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the artifact.write probe did not run (rc=$rc)"; exit 1; }
test -n "$dg" || { echo "FAIL (1b): no artifact.write row with kind Spec for $STORY_ITEM — the authored story is not reachable from the event log (AC-521)"; exit 1; }
[ "$dg" = "$landed" ] || { echo "FAIL (1b): the last Spec artifact.write digest $dg is not the landed blob's $landed (ADR-0116 decisions 3 and 6)"; exit 1; }
printf '%s\n' "$out"   # → READ against AC-521: every story-lane stage (asserted above), the Spec authored by the run. The run id and the story id are the evidence.
```

**A2 — (2)'s operator half: build first, and back up and restore the store around `doctor --rebuild`.** Replace the
three lines from `# The operator path must still work (AC-427), but its STDOUT is not the comparison.` through
`./target/debug/redkiln doctor --rebuild || { … }` with the block below. Either repair would have served: back up
and restore, or run (7) before (2). Back up and restore is taken because it makes the block order-independent. The
restore uses SQLite's own backup API, because a bare `cp` of a WAL-mode database copies an inconsistent pair.

```bash
# The operator path must still work (AC-427), but its STDOUT is not the comparison. AMENDED at s8j (s8j §8j.8 A2):
# built first, and the store `doctor --rebuild` replaces is backed up and restored, so (7) reads the store the run wrote.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (2)'s operator half NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2)'s operator half NOT RUN (Halt 8)"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
STORE_BAK="$store.s8-bak"
bk='import sqlite3,sys; s=sqlite3.connect(sys.argv[1]); d=sqlite3.connect(sys.argv[2]); s.backup(d); d.close(); s.close()'
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate","t1_slice_seal","t2_event")))'
before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$before" ] || { echo "FAIL: could not count the store's rows (rc=$rc)"; exit 1; }
python -c "$bk" "$store" "$STORE_BAK" || { echo "FAIL: the store could not be backed up — doctor --rebuild NOT RUN"; exit 1; }
./target/debug/redkiln doctor --rebuild; drc=$?
python -c "$bk" "$STORE_BAK" "$store" || { echo "FAIL: the store could NOT be restored from $STORE_BAK — restore it by hand before anything reads it"; exit 1; }
after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$after" = "$before" ] || { echo "FAIL: the restored store's row counts ($after) are not the pre-rebuild ones ($before)"; exit 1; }
rm -f "$STORE_BAK"
[ "$drc" -eq 0 ] || { echo "FAIL: doctor --rebuild exited $drc (AC-427)"; exit 1; }
```

**A3 — (3)'s operational arm: build before `test -x`.** Insert these two lines immediately above the line that
opens `test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s operational arm`:

```bash
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A3): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — (3)'s operational arm NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
```

**A4 — (7), the whole fenced block.** D21-42 12a: *"An honest rendering passes, with two assertions: owned readings
== `dispatch_completed` for the item, and no rendering over n = 0. This replaces s8's unreadable AC-394 READ line."*
The `item` member on `dispatch_completed` is D21-42 12b's interim key, which is *"extended to `dispatch_completed`"*.

```bash
# AMENDED at s8j (s8j §8j.8 A4): built first; AC-394 asserted as owned readings == dispatch_completed (D21-42 12a).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (7) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (7) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item whose cost rows are being read"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
own='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); n=lambda k: sum(1 for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",(k,)) if json.loads(p).get("item")==sys.argv[2]); print(n("rate_limit_reading"), n("dispatch_completed"))'
cnt=$(python -c "$own" "$store" "$ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$cnt" ] || { echo "FAIL: the ownership probe did not run (rc=$rc)"; exit 1; }
read -r owned dc <<< "$cnt"
[ "$dc" -ge 1 ] || { echo "FAIL (7): no dispatch_completed row names $ITEM — a reading over n = 0 is not a reading (D21-42 12a)"; exit 1; }
[ "$owned" = "$dc" ] || { echo "FAIL (7): $ITEM owns $owned readings and $dc dispatch_completed rows — AC-394 is one reading per dispatch (ADR-0163 decision 2)"; exit 1; }
out=$(./target/debug/redkiln cost "$ITEM") || { echo "FAIL: redkiln cost exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: redkiln cost printed nothing"; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: a headroom report carrying none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
if printf '%s\n' "$out" | grep -qE ' nothing to attribute |(^|[^0-9])n=0([^0-9]|$)'; then echo "FAIL (7): a rendering over n = 0 for an item that owns $dc dispatches (D21-42 12a)"; printf '%s\n' "$out"; exit 1; fi
printf '%s\n' "$out"   # → READ: no per-run headroom figure (AC-373).
```

**A5 — Checkpoint 8's mechanical block.** Five edits. Each is anchored on a line that exists today, and each anchor
is asserted to exist before it is replaced (golden constraint 7).

*A5.1 — the catalog checks run over `.redkiln/` (D21-42 13a, 13b).* Replace the three lines that open
`out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?` with the lines below. Today's line walks
`spikes/p9/catalog`, because `rigor.rs` holds `const SHIPPED_CATALOG: &str = "spikes/p9/catalog";`.

```bash
CATALOG_TESTS="BIND:s8b-13a-catalog-tests"   # s8b's module-qualified names: rigor monotonicity, AC-527's d2-form lint and the ported lattice, over .redkiln/ with a non-zero walked count (D21-42 13a, 13b)
case "$CATALOG_TESTS" in *BIND:*) echo "FAIL: unbound slot — apply s8j §8j.8's binding"; exit 1;; esac
n_sel=$(printf '%s\n' $CATALOG_TESTS | grep -c .)
out=$(cargo test -p rk-kernel --test it -- $CATALOG_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the .redkiln/ catalog selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
for t in $CATALOG_TESTS; do printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok"; exit 1; }; done
printf '%s\n' "$out" | grep -qE "test result: ok\. $n_sel passed; 0 failed" || { echo "FAIL: the catalog selection did not run exactly $n_sel tests"; exit 1; }
```

*A5.2 — the quoted-predicate guard is `s8b`'s form (D21-41 (h): "widen Checkpoint 8's guard").* `s8b` owns the form:
*"`s8j` widens Checkpoint 8's quality-shorthand block to it. This file does not edit `s8`."* Checkpoint 8b's section
(4) is that form, and it runs there first. This file does not restate it.

At application, the four lines of `s8`'s block that begin with the line opening
`raw=$(grep -rnE 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *='` and end with
its `syn=$(printf 'a.toml:1:expr = …` control are replaced. The replacement is the code lines of Checkpoint 8b
section (4), copied verbatim from `s8b`'s file as landed:

- the pattern;
- the `EXCL` list, with each path's stated reason;
- both synthetic controls.

The one change in the copy is A5.5's `--exclude-dir=worktrees` on its `grep -rnE … crates/ .redkiln/` line. Copying
from the file rather than retyping the form here means the two cannot drift. Any further must-refuse fixture that
`s8b` adds is already in its `EXCL`. Checkpoint 8j (8) asserts that the unquoted-only pattern is gone from `s8`'s
code lines and that `s8b`'s pattern is present.

*A5.3 — the doc screen reads neither `.git/` nor a run worktree.* Replace
`grep -rn 'four questions on twenty items' --include=*.md . ; rc=$?` with the two lines below. D21-40 measured this
line reading a stale `runbook/` copy under `.git/redkiln/worktrees/`. D21-44 moves run worktrees to
`.redkiln/worktrees/`, where the same line would read the same stale copy again. `--exclude-dir` matches a base name,
so the first line proves that no tracked `.md` is hidden by the second exclusion. A5.5's control proves that the
exclusion reaches `.redkiln/worktrees/`.

```bash
wtmd=$(git ls-files -- '*.md' | grep -E '(^|/)worktrees/'); test -z "$wtmd" || { echo "FAIL: a tracked .md lies under a directory named worktrees — the screen below would skip it:"; printf '%s\n' "$wtmd"; exit 1; }
grep -rn --exclude-dir=.git --exclude-dir=worktrees 'four questions on twenty items' --include=*.md . ; rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: the phrase is gone (rc=$rc) — design §10.5's row may have been deleted"; exit 1; }   # → each hit READ; .git/ and worktrees/ excluded (D21-40; D21-44)
```

*A5.4 — two new stanzas, appended after the block's last line* (the `--stay` by-path stanza's guard that opens
`printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails`). The first is
ADR-0164's first rule. The second is D21-42 14a's CI condition.

```bash
# ADR-0164: `* text=auto eol=lf` is .gitattributes' FIRST rule (every -text row below it still wins for its paths).
first=$(grep -vE '^[[:space:]]*(#|$)' .gitattributes | head -1)
[ "$first" = '* text=auto eol=lf' ] || { echo "FAIL: .gitattributes' first rule is '$first', not ADR-0164's row"; exit 1; }
if od -c .gitattributes | grep -qF '\r'; then echo "FAIL: .gitattributes carries a CR byte (ADR-0164 decision 4)"; exit 1; fi
# D21-42 14a: ci.yml green on the pushed branch, every job, both OS legs — read from the recorded run, never re-pushed.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} ' runbook/phase-1/s8j-evidence.txt | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in runbook/phase-1/s8j-evidence.txt"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha $ci_sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln || { echo "FAIL: code changed since the green CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }
```

The bullet list gains two bullets:

- ***`ci.yml` is green on the pushed branch, every job, both OS legs*** (D21-42 14a), *read from the recorded run and
  never re-pushed by this checkpoint.*
- ***`.gitattributes` opens with `* text=auto eol=lf`*** (ADR-0164).

Its count sentence *"Those seventeen are not read, they are run."* becomes *"Those nineteen are not read, they are
run."*.

*A5.5 — no recursive walk over `.redkiln/` enters a run worktree (D21-44).* D21-44 gives the rule to `s8e`:
*"Every recursive walk over `.redkiln/` as a whole excludes `worktrees/`, each with a synthetic control, because a run
worktree is a full repository copy. This includes Checkpoint 8's `grep -r … .redkiln/` lines"*. `s8e` §8e.10 hands
`s8`'s share to this stage: *"`s8-exit-criterion.md`'s recursive greps over `crates/ .redkiln/` are `s8j`'s"*. Three walks in the block are affected:

- the line that opens `raw=$(grep -rnE 'require.*\b(criticality|`;
- the line that opens `grep -rnE 'append(_route)? *= *true'`;
- A5.2's copied `grep -rnE … crates/ .redkiln/` line.

Each gains `--exclude-dir=worktrees` after `-rnE`. Insert the stanza below immediately above the first of them.
`--exclude-dir` matches a base name, so it also skips the four tracked fixture files that lie under directories named
`worktrees` in `crates/*/tests/fixtures/` [measured at `a6349b6`: none carries any of the three walked patterns]. The
stanza's first guard keeps that true, and its control proves the skip drops a run-worktree copy and keeps a pack.

```bash
# AMENDED at s8j (s8j §8j.8 A5.5; D21-44, whose walk rule is s8e's): the three walks over .redkiln/ below skip every
# directory named worktrees — a run worktree under .redkiln/worktrees/<item> is a full repository copy.
WALKED='require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b|append(_route)? *= *true|findings\.none\(evidence in \[|quality_shorthand *='
lost=$(git ls-files | grep -E '(^|/)worktrees/' | while IFS= read -r wf; do grep -lE "$WALKED" "$wf"; done)
test -z "$lost" || { echo "FAIL: a tracked file under a directory named worktrees carries a walked pattern — the skip would hide it:"; printf '%s\n' "$lost"; exit 1; }
ctl=target/s8-a55; rm -rf "$ctl"; mkdir -p "$ctl/.redkiln/worktrees/x" "$ctl/.redkiln/packs" || { echo "FAIL: cannot create $ctl"; exit 1; }
printf 'append = true\n' > "$ctl/.redkiln/worktrees/x/a.toml" && printf 'append = true\n' > "$ctl/.redkiln/packs/b.toml" || { echo "FAIL: cannot write the worktrees-skip controls"; exit 1; }
hits=$(grep -rlE --exclude-dir=worktrees 'append(_route)? *= *true' "$ctl/.redkiln" --include=*.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$hits" = "$ctl/.redkiln/packs/b.toml" ] || { echo "FAIL: the worktrees skip is wrong (rc=$rc) — it must drop the run-worktree copy and keep the pack: $hits"; exit 1; }
rm -rf "$ctl"
```

**A6 — (6): build before the binary runs.** This one is not on the plan's list. It is added because (6)'s two lint
lines invoke `./target/debug/redkiln` with no build and no presence guard, which is the shape golden constraint 3
exists for. Insert these lines above the line that opens `./target/debug/redkiln lint --facet-stability`. The two
lint lines themselves stay bare reads of an exit code, as `s8` explains.

```bash
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A6): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (6) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (6) NOT RUN (Halt 8)"; exit 1; }
```

**A7 — (3)'s operational arm runs through the launcher.** This amendment comes from `s8h` §8h.7, not from the plan:
*"`s8-exit-criterion.md` invokes `./target/debug/redkiln run` at three sites. After this stage, each one refuses
without a launch record, so each must go through `tools/live-run.sh run`."* A1 carries (1)'s and (1b)'s run sites.
This replaces the third, the line that opens `./target/debug/redkiln run "$ITEM" --lane fix || { echo "FAIL: the
re-invocation exited non-zero"`, with the line below. The comment above it, which opens
`# Now kill the running process mid-graph`, gains *"the run was started through tools/live-run.sh run (s8j §8j.8
A7), so `/T` on the launcher's PID takes its redkiln child with it"*.

```bash
tools/live-run.sh run "$ITEM" --lane fix || { echo "FAIL: the re-invocation exited non-zero"; exit 1; }   # AMENDED at s8j (s8j §8j.8 A7): through s8h's launcher, the only entry to a real run (D21-42 17a). A re-invocation that reaches a gate exits 0 (C-01 D-01), so this guard passes on a gated run too; the digest diff below is the assertion.
```

**A8 — (2) gains the anchored rebuild differential (`s8e` §8e.9).** This one is not on the plan's list either. `s8e`
builds `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`, because the kernel test cannot measure ADR-0105
clause 9's route. `s8e` hands its selection on: *"Adding the anchored test to `s8` (2)'s own block is `s8j`'s
instrument amendment, and it is listed there rather than made here."* Insert the lines below immediately above the
line `# The operator path must still work (AC-427), but its STDOUT is not the comparison.`, which is A2's anchor. The
kernel selections above them stay as they are.

```bash
# AMENDED at s8j (s8j §8j.8 A8; s8e §8e.9): the ROUTE of ADR-0105 clause 9's tier-1 projection — rows whose anchor is
# reachable from the refs, the store deleted, doctor --rebuild, canonical equality — which the kernel test cannot see.
ANCHOR_TESTS="doctor::ac184_rebuild_differential_over_anchored_tier_one_rows"
out=$(cargo test -p rk-cli --test it -- $ANCHOR_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the anchored rebuild differential did not pass (rc=$rc) — AC-184's route is unmeasured"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qxF "test $ANCHOR_TESTS ... ok" || { echo "FAIL: $ANCHOR_TESTS did not report ok by name"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the anchored differential selection did not run exactly one test"; exit 1; }
```

### 8j.9 Acceptance set

**Claimed.**

- **AC-410**: the positive control through a pack-declared question, over both human roots.
- **AC-054**: the step-4 gate is answerable, and a re-invocation reads the root as Present.
- **AC-518**: the user's checkout is unchanged across a full lane run. This is its Verified-by, *"an integration test
  asserting the user checkout is unchanged across a full lane run"*, in `lane_proof`.
- **AC-226**: after the pin move, the configured `Refuse` outcome is still the asserted one.
- The first run of every `ci.yml` job that `ci.yml` marks *"ARMED, NOT DISCHARGED"*:
  - **AC-498**, the no-egress job;
  - **AC-465**;
  - **AC-362**;
  - **AC-441**, **AC-442** and **AC-466**;
  - **AC-481**, **AC-483** and **AC-528**;
  - **AC-529**.

**Excluded, each for its own reason.**

- **AC-473** and **AC-521** are claimed by `s8`'s real runs. The fixture proofs are the spend gate D21-40 decision 2
  names, not the claim.
- **AC-160…AC-163** belong to `s8e`. ADR-0158's forbids bullet, *"Claiming AC-160, AC-161, AC-162 or AC-163 is met,
  on any axis"*, stands wherever `s8e`'s record did not lift it.
- **AC-184** is `s8`'s (2).
- **AC-372, AC-373, AC-394 and AC-395** are `s8`'s (7).
- **AC-276** belongs to `s8h`.
- **AC-114**'s promotion follows `s8`'s fix-lane run (§8j.1(e)). This stage does not claim it, and it does not
  build the promotion's `curation.decision` lint.
- **AC-121's cross-OS half.** The two legs make it checkable, but no instrument named here asserts it, so it is not
  claimed.

### 8j.10 Checkpoint 8j

- **Both lanes are proven end to end under the FakeHarness**, through `verbs::run::run` with the real Rust adapter,
  across real process boundaries. That is ten named `lane_proof` tests, plus three human-root question tests.
- **The items are as §8j.1 and §8j.5 say.**
  - The story item exists at `triage`, with low facets, its slice and its trial declaration.
  - The Codex item is gone.
  - The empty-case item exists, and Checkpoint 7i names it.
  - The fixture item's non-comment lines and ref are untouched. Its checkout is re-pointed to `.redkiln/worktrees/`,
    clean and without an `RK_BASE` marker, so `s8e`'s re-entry refuses it.
  - The reserved fix item exists with its hold, slice, trial declaration, BugReport and repro pair. It has no run
    ref.
  - The reserved defect **is still red** at HEAD, in a throwaway tree.
- **`spike.toml` is untouched** since `ca8f387`.
- **Checkpoints 7h and 7i read named fixture items**, and neither reads `$FIX_ITEM`.
- **The pin, the instrument record, Checkpoint 7c's whole run and both probe results agree.** Each run has its
  launch record. The (g) record is `accepted` only with both probes recorded as passing.
- **CI is green on the recorded run**: every job, both legs, and no code change since.
- **`s8`'s amendments landed**:
  - no `BIND:`;
  - no `test -x` before a build;
  - no bare `redkiln run`, since every run goes through the launcher;
  - every selected test exists.
- **The entry chain re-runs once, through Checkpoint 8i, and Checkpoints 7h and 7i re-run whole.** Each reaches its
  end marker, fenced against a real dispatch.
- **The whole suite, fmt and clippy are green, and the three runbook checkers pass.**
- **This block dispatched nothing, and it changed no store row, no `rk-run/*` ref and no launch record.**

**Run it from a Git Bash that Claude Code did not spawn.** Checkpoints 8h and 8i refuse a Claude-spawned shell, and
this block re-runs them. Set every input the chain's blocks name at their tops (8h's `REC_A`, `REC_B`,
`A_SUPERSEDES` and `TOKENS_APPETITE`, 8i's own, and so on). Each block refuses by name when one is missing. Budget
hours: (10) re-runs the chain.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# 🛑 THIS BLOCK DISPATCHES NOTHING AND PUSHES NOTHING. CI status and probe results are READ from the run ids and lines
# recorded in runbook/phase-1/s8j-evidence.txt. Every cargo reading is `out=$(…); rc=$?`; every selection is `--test it`,
# module-qualified and `--exact`, with each test's own `... ok` line and an exact count asserted.

# ---- (0) THIS SHELL, THEN READ FIRST: every quantity a dispatch, a run or a launch would move. None may move.
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_|CLAUDE_PID=|CLAUDE_EFFORT=)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — Checkpoints 8h and 8i, which (10) re-runs, refuse a Claude-spawned shell"; exit 1; }
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
gcd=$(git rev-parse --git-common-dir) || { echo "FAIL: git rev-parse --git-common-dir failed"; exit 1; }
store="$gcd/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate","t1_slice_seal","t2_event")), c.execute("SELECT COUNT(*) FROM t2_event WHERE kind IN (?,?,?)",("dispatch","dispatch_completed","rate_limit_reading")).fetchone()[0])'
rows_before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$rows_before" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }
refs_before=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
[ "$rc" -eq 0 ] && [ -n "$refs_before" ] || { echo "FAIL: no rk-run/* ref could be read (rc=$rc) — rk-run/rk-fix-doctor-test-count at least must exist"; exit 1; }
launches="$gcd/redkiln/launches"
nl_before=$(find "$launches" -type f 2>/dev/null | sort)
# The transcript tripwire: a real `claude -p` persists a session .jsonl under its config dir's projects/ (the adapter
# passes no --no-session-persistence). It is a DETECTOR beside the prevention in (10), not the prevention.
CFG_DIRS=""
for d in "$HOME/.claude-personal/projects" "$HOME/.claude/projects"; do [ -d "$d" ] && CFG_DIRS="$CFG_DIRS $d"; done
test -n "$CFG_DIRS" || { echo "FAIL: no Claude projects directory under \$HOME — the transcript tripwire has no baseline"; exit 1; }
sess_before=$(find $CFG_DIRS -type f -name '*.jsonl' 2>/dev/null | sort)   # unquoted on purpose: a word per directory
tdir=target/s8j-cp8j; rm -rf "$tdir"; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir (a git-bash /tmp path is invisible to Windows Python, so scratch lives under target/)"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — nothing this stage's gated acts recorded can be read"; exit 1; }
if od -c "$ev" | grep -qF '\r'; then echo "FAIL: $ev carries a CR byte — a field read from it would carry one too"; exit 1; fi

# ---- (1) BUILD-OR-FAIL.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8j NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE FIXTURE PROOFS (§8j.4) AND THE HUMAN-ROOT QUESTION TESTS (§8j.2): one rk-cli selection of thirteen.
test -d crates/rk-cli/tests/fixtures/lane || { echo "FAIL: the lane fixture is absent — the proofs would materialise nothing"; exit 1; }
test -f crates/rk-cli/tests/it/lane_proof.rs || { echo "FAIL: crates/rk-cli/tests/it/lane_proof.rs is absent"; exit 1; }
R_TESTS="lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness lane_proof::fix_lane_trailers_every_non_forbidden_stage_in_order lane_proof::fix_lane_verify_completed_names_the_rust_adapter lane_proof::story_lane_spec_is_landed_and_journalled lane_proof::story_lane_trailers_every_non_forbidden_stage_in_order lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes lane_proof::gate_then_answer_then_second_process_continues_binary_arm lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification lane_proof::boundary_hold_changes_requested_does_not_advance processes::the_two_human_root_questions_load_and_each_attaches_its_kind processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question"
out=$(cargo test -p rk-cli --test it -- $R_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the lane proofs and question tests did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
for t in $R_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 13 passed; 0 failed' || { echo "FAIL: the selection did not run exactly 13 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
# The proofs use the REAL Rust adapter: no proof names the fixture toolchain child or FixtureToolchain on a CODE line —
# not even to assert its absence, which the verify_completed identity test asserts behaviourally. A FLOOR. The file is
# lane_proof.rs alone: s8e's lane_e2e.rs runs pack-min's fixture lane and is not bound by this.
raw=$(grep -nE 'rk-fixture-toolchain|FixtureToolchain' crates/rk-cli/tests/it/lane_proof.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the fixture-toolchain floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: a lane proof names a fixture toolchain on a code line — AC-473's adapter identity would be proved over a double:"; printf '%s\n' "$code"; exit 1; }
syn=$(printf '12:    let t = FixtureToolchain::default();\n' | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -E 'rk-fixture-toolchain|FixtureToolchain'); test -n "$syn" || { echo "FAIL: the comment filter swallowed a code line — the floor is vacuous"; exit 1; }

# ---- (3) THE ITEMS (§8j.1, §8j.5), THE SLICES, AND THE RE-POINTED CHECKOUT.
itm='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); f=d.get("facets",{}); t=d.get("trial",{}); print(d.get("id"), d.get("kind"), d.get("stage"), d.get("hold_before","-"), d.get("slice","-"), f.get("cos","-"), f.get("criticality","-"), f.get("blast_radius","-"), f.get("size","-"), str(f.get("external_dep","-")).lower(), str(f.get("has_repro","-")).lower(), t.get("activity","-"))'
test ! -e .redkiln/items/rk-story-codex-adapter.toml || { echo "FAIL: the Codex item is not retired (ADR-0007; D21-41 (f))"; exit 1; }
s=$(python -c "$itm" .redkiln/items/rk-story-harden-spike-pack.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the story item did not parse (rc=$rc)"; exit 1; }
[ "$s" = "rk-story-harden-spike-pack story triage - harden-spike-pack standard low low low false - tag-release" ] || { echo "FAIL: the story item is '$s', not a story at triage with its slice, low facets, external_dep = false and the tag-release trial"; exit 1; }
e=$(python -c "$itm" .redkiln/items/rk-cost-empty-case.toml); rc=$?
[ "$rc" -eq 0 ] && [ "${e%% *}" = "rk-cost-empty-case" ] || { echo "FAIL: the empty-case item is absent or mis-named ('$e', rc=$rc)"; exit 1; }
fx_line=$(grep -E '^fix-item rk-fix-[a-z0-9-]+ [0-9a-f]{40}$' "$ev" | tail -1)
test -n "$fx_line" || { echo "FAIL: no fix-item line in $ev — the reservation is unrecorded"; exit 1; }
read -r _ fx fx_frozen <<< "$fx_line"
grep -qE "^reserved-fix-item: $fx [a-z0-9_-]+:[A-Za-z0-9_:]+\$" spikes/DECISIONS-2026-09-21.md || { echo "FAIL: $fx is not reserved in the ledger (D21-42 18)"; exit 1; }
f=$(python -c "$itm" ".redkiln/items/$fx.toml"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: .redkiln/items/$fx.toml did not parse (rc=$rc)"; exit 1; }
[ "$f" = "$fx bug triage specification ${fx#rk-fix-} standard low low low - true tag-release" ] || { echo "FAIL: the reserved item is '$f', not a bug at triage holding before specification, with slice ${fx#rk-fix-}, low facets, has_repro and the tag-release trial"; exit 1; }
# s8e §8e.8's refusals, read here rather than trusted to the mint: a slice that is the id, the project or a stage.
slc='import sys,tomllib
d=tomllib.load(open(sys.argv[1],"rb")); p=tomllib.load(open(sys.argv[2],"rb"))["policy"]; s=d.get("slice")
bad=[]
if not isinstance(s,str) or not s: bad.append("no slice")
elif any(c.isspace() for c in s): bad.append("whitespace in the slice")
elif s in (d.get("id"), sys.argv[3]) or s in p["stage_order"]: bad.append("the slice "+s+" is the item id, the project or a stage")
print("; ".join(bad) if bad else "ok"); sys.exit(1 if bad else 0)'
for pair in "rk-story-harden-spike-pack:story" "$fx:bug-standard"; do
  v=$(python -c "$slc" ".redkiln/items/${pair%%:*}.toml" ".redkiln/packs/${pair##*:}.toml" redkiln-rs); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: ${pair%%:*}'s slice: $v (s8e §8e.8)"; exit 1; }
done
printf 'id = "rk-fix-x"\nkind = "bug"\nstage = "triage"\nslice = "rk-fix-x"\n' > "$tdir/slice-red.toml" || { echo "FAIL: could not write the slice control"; exit 1; }
v=$(python -c "$slc" "$tdir/slice-red.toml" .redkiln/packs/bug-standard.toml redkiln-rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the slice check accepted a slice equal to the item id (rc=$rc: $v) — it is vacuous"; exit 1; }
if git rev-parse --verify --quiet "refs/heads/rk-run/$fx" >/dev/null; then echo "FAIL: rk-run/$fx already exists — the fresh item's run must fork from the frozen HEAD at s8's first run"; exit 1; fi
test ! -e ".redkiln/worktrees/$fx" || { echo "FAIL: .redkiln/worktrees/$fx exists — nothing may have run the reserved item"; exit 1; }
test -s ".redkiln/intake/$fx.BugReport.md" || { echo "FAIL: no BugReport for $fx"; exit 1; }
test -s .redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md || { echo "FAIL: no IntakeRecord for the story item"; exit 1; }
old=$(git show ca8f387:.redkiln/items/rk-fix-doctor-test-count.toml | grep -vE '^[[:space:]]*(#|$)')
new=$(grep -vE '^[[:space:]]*(#|$)' .redkiln/items/rk-fix-doctor-test-count.toml)
[ -n "$old" ] && [ "$old" = "$new" ] || { echo "FAIL: rk-fix-doctor-test-count's non-comment lines changed — it is the UNTOUCHED fixture item (D21-42 18)"; exit 1; }
r_now=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); r_pin=$(git rev-parse --verify --quiet "73c087a^{commit}")
[ -n "$r_now" ] && [ -n "$r_pin" ] && [ "$r_now" = "$r_pin" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is '$r_now', not 73c087a ('$r_pin') — Checkpoints 7h and 7i read it; two empty reads never compare equal here"; exit 1; }
# D21-44: the legacy checkout is re-pointed from .git/redkiln/worktrees/ to .redkiln/worktrees/, its ref kept, and it
# carries no RK_BASE marker, so s8e's re-entry refuses any run of the item (D21-45 W2; §8j.1(d)).
wtl=$(git worktree list --porcelain); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree list failed (rc=$rc)"; exit 1; }
if printf '%s\n' "$wtl" | grep -qE '^worktree .*/\.git/redkiln/worktrees/'; then echo "FAIL: a worktree is still registered under .git/redkiln/worktrees/ — the legacy placement was not re-pointed (s8j §8j.1(d); D21-44)"; exit 1; fi
top=$(git rev-parse --show-toplevel) || { echo "FAIL: git rev-parse --show-toplevel failed"; exit 1; }   # D:/… — the form `git worktree list` prints; Git Bash's pwd prints /d/…
lwt="$top/.redkiln/worktrees/rk-fix-doctor-test-count"
printf '%s\n' "$wtl" | grep -qxF "worktree $lwt" || { echo "FAIL: the legacy checkout is not registered at $lwt"; exit 1; }
lh=$(git -C "$lwt" rev-parse HEAD); rc=$?
[ "$rc" -eq 0 ] && [ "$lh" = "$r_pin" ] || { echo "FAIL: the re-pointed checkout is at '$lh', not 73c087a (rc=$rc)"; exit 1; }
lst=$(git -C "$lwt" status --porcelain); rc=$?
[ "$rc" -eq 0 ] && [ -z "$lst" ] || { echo "FAIL: the re-pointed checkout is not clean (rc=$rc)"; exit 1; }
lgd=$(git -C "$lwt" rev-parse --absolute-git-dir); rc=$?
[ "$rc" -eq 0 ] && [ -d "$lgd" ] || { echo "FAIL: the re-pointed checkout's gitdir could not be read (rc=$rc)"; exit 1; }
test ! -e "$lgd/RK_BASE" || { echo "FAIL: the legacy checkout carries an RK_BASE marker — s8e's re-entry would ADOPT it instead of refusing it"; exit 1; }
git check-ignore -q .redkiln/worktrees/rk-fix-doctor-test-count || { echo "FAIL: .redkiln/worktrees/ is not ignored — the re-pointed checkout dirties git status (AC-518)"; exit 1; }
test ! -e "$gcd/redkiln/worktrees/rk-fix-doctor-test-count" || { echo "FAIL: the old checkout directory is still on disk under .git"; exit 1; }
grep -qE '^legacy-worktree-repointed rk-fix-doctor-test-count 73c087a [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" || { echo "FAIL: the re-point is not recorded in $ev"; exit 1; }
# The story is the lane's to author: this stage did not touch spike.toml.
test -f spikes/p9/packs/spike.toml || { echo "FAIL: spikes/p9/packs/spike.toml is gone"; exit 1; }
git diff --quiet ca8f387 HEAD -- spikes/p9/packs/spike.toml || { echo "FAIL: spike.toml changed since ca8f387 — the story lane would have nothing to author"; exit 1; }

# ---- (4) THE RESERVED DEFECT IS STILL RED AT HEAD — in a throwaway tree, never in the crate.
rmeta=".redkiln/intake/$fx.repro.toml"; rsrc=".redkiln/intake/$fx.repro.rs"
test -s "$rmeta" && test -s "$rsrc" || { echo "FAIL: $fx's repro pair is absent"; exit 1; }
rp='import sys,tomllib; d=tomllib.load(open(sys.argv[1],"rb")); print(d["crate"]); print(d["module"]); print(d["test"]); print(d["expect"])'
meta=$(python -c "$rp" "$rmeta"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: $rmeta did not parse (rc=$rc)"; exit 1; }
{ read -r rcrate; read -r rmod; read -r rtest; read -r rexpect; } <<< "$meta"
[ -n "$rcrate" ] && [ -n "$rmod" ] && [ -n "$rtest" ] && [ -n "$rexpect" ] || { echo "FAIL: $rmeta lacks crate/module/test/expect"; exit 1; }
test -f "crates/$rcrate/tests/it/$rmod.rs" || { echo "FAIL: crates/$rcrate/tests/it/$rmod.rs does not exist"; exit 1; }
n=$(grep -cE "fn $rtest\(" "crates/$rcrate/tests/it/$rmod.rs")
[ "$n" = "0" ] || { echo "FAIL: the repro test is committed INTO the crate — it would turn CI red and hand the lane's red step its answer"; exit 1; }
git archive --format=tar -o "$tdir/head.tar" HEAD || { echo "FAIL: git archive HEAD failed"; exit 1; }
mkdir -p "$tdir/repro" && tar -xf "$tdir/head.tar" -C "$tdir/repro" || { echo "FAIL: could not extract HEAD"; exit 1; }
cat "$rsrc" >> "$tdir/repro/crates/$rcrate/tests/it/$rmod.rs" || { echo "FAIL: could not append the repro"; exit 1; }
out=$(cd "$tdir/repro" && CARGO_TARGET_DIR=../../s8j-repro-target cargo test -p "$rcrate" --test it -- "$rmod::$rtest" --exact 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the reserved defect's repro PASSES at HEAD — the defect was fixed before the run (D21-42 18)"; exit 1; }
printf '%s\n' "$out" | grep -qxF "test $rmod::$rtest ... FAILED" || { echo "FAIL: the repro did not run and fail by name — red for another reason is not the defect"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qF "$rexpect" || { echo "FAIL: the repro failed without its expected message '$rexpect' — a red for the wrong reason"; exit 1; }

# ---- (5) CHECKPOINTS 7i AND 7h ARE RE-POINTED (§8j.1(c)): code lines name the fixture items, never $FIX_ITEM.
x7='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
b=[x for x in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in x]
code="\n".join(l for l in (b[-1].split("\n") if b else []) if not l.strip().startswith("#"))
print(len(b), code.count("FIX_ITEM"), sum(1 for l in code.split("\n") if l.strip()=="FIXTURE_ITEM=rk-fix-doctor-test-count"), code.count("rk-story-codex-adapter"), code.count("rk-cost-empty-case"), code.count("FAIL (PRECONDITION): rk-cost-empty-case owns"))'
c7=$(python -c "$x7" runbook/phase-1/s7i-cost.md 'CP7I-END-OF-BLOCK reached'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7i could not be read (rc=$rc)"; exit 1; }
read -r nb c_fix c_asg c_old c_new c_pre <<< "$c7"
[ "$nb" -ge 1 ] && [ "$c_fix" = "0" ] && [ "$c_asg" = "1" ] && [ "$c_old" = "0" ] && [ "$c_new" -ge 7 ] && [ "$c_pre" = "1" ] || { echo "FAIL: Checkpoint 7i's code lines: $c_fix FIX_ITEM, $c_asg fixture assignments, $c_old Codex-item names, $c_new empty-case names, $c_pre PRECONDITION guards (blocks: $nb)"; exit 1; }
c7=$(python -c "$x7" runbook/phase-1/s7h-lane-completes.md 'CP7H-END-OF-BLOCK reached'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7h could not be read (rc=$rc)"; exit 1; }
read -r nb c_fix c_asg _ _ _ <<< "$c7"
[ "$nb" -ge 1 ] && [ "$c_fix" = "0" ] && [ "$c_asg" = "1" ] || { echo "FAIL: Checkpoint 7h's code lines: $c_fix FIX_ITEM and $c_asg fixture assignments (blocks: $nb) — it still reads the environment"; exit 1; }
# SYNTHETIC CONTROL: a block that still reads $FIX_ITEM must count it.
bt=$(printf '%s' '`' '`' '`')
printf '%s\n' "$bt""bash" 'test -n "$FIX_ITEM" || exit 1' 'echo "CP7X-END-OF-BLOCK reached"' "$bt" > "$tdir/x7-red.md" || { echo "FAIL: could not write the re-point control"; exit 1; }
c7=$(python -c "$x7" "$tdir/x7-red.md" 'CP7X-END-OF-BLOCK reached'); rc=$?
read -r _ c_fix _ _ _ _ <<< "$c7"
[ "$rc" -eq 0 ] && [ "$c_fix" = "1" ] || { echo "FAIL: the re-point reader did not count a \$FIX_ITEM read (rc=$rc: $c7) — it is vacuous"; exit 1; }

# ---- (6) THE PIN, THE INSTRUMENT RECORD, 7c's WHOLE RUN AND THE PROBES AGREE; (g) IS ACCEPTED ONLY WITH BOTH PASSED.
inst=$(tr -d '\r\n' < spikes/results/instrument-checks/RUNBOOK-instrument.txt)
ver=${inst%% *}
pin=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9]+\.[0-9]+\.[0-9]+)";$/\1/p' crates/rk-adapter-claude/src/config.rs)
[ -n "$ver" ] && [ "$ver" = "$pin" ] || { echo "FAIL: the instrument record says '$ver' and TESTED_CLI_VERSION says '$pin' (Halt 2; D21-42 16)"; exit 1; }
grep -qxF "    assert_eq!(TESTED_CLI_VERSION, \"$pin\");" crates/rk-adapter-claude/tests/it/config.rs || { echo "FAIL: the literal pin test does not carry $pin"; exit 1; }
out=$(cargo test -p rk-adapter-claude --test it -- config::tested_cli_version_is_the_pin_as_a_config_value --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the pin test did not pass exactly once (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'test config::tested_cli_version_is_the_pin_as_a_config_value ... ok' || { echo "FAIL: the pin test did not report ok by name"; exit 1; }
pm=$(grep -E '^pin-move [0-9.]+ ([0-9.]+|unchanged) [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$pm" || { echo "FAIL: no pin-move line in $ev"; exit 1; }
read -r _ pm_old pm_new _ <<< "$pm"
[ "$pm_new" = "$pin" ] || { [ "$pm_new" = "unchanged" ] && [ "$pm_old" = "$pin" ]; } || { echo "FAIL: the recorded pin move ($pm_old -> $pm_new) does not end at the pin $pin"; exit 1; }
# Each paid run is recorded at the pin, with its launch record on disk.
# Checkpoint 7c ran whole as a `wrap` child (§8j.6 step 2), each probe as a `probe` child: the launch record's name
# carries its mode (<UTC>-<mode>.txt, s8h §8h.7 step 8), and each record is on disk.
for pm2 in "checkpoint-7c:wrap" "probe live::live_confinement_and_denial_probe:probe" "probe live::live_init_equality_probe:probe"; do
  p=${pm2%:*}; md=${pm2##*:}
  ln=$(grep -E "^$p $pin pass [0-9]{4}-[0-9]{2}-[0-9]{2} [^ ]+\$" "$ev" | tail -1)
  test -n "$ln" || { echo "FAIL: no passing '$p' recorded at the pin $pin — the (g) record may not flip (D21-41 (g); D21-43 N28)"; exit 1; }
  lr=${ln##* }
  case "$lr" in *-"$md".txt) : ;; *) echo "FAIL: '$p' names launch record $lr, which is not a $md launch"; exit 1;; esac
  test -s "$launches/$lr" || { echo "FAIL: '$p' names launch record $lr, which is not under $launches — it did not go through the launcher (D21-42 17a)"; exit 1; }
done
rg_id=$(sed -nE 's/^record-g ([0-9]{4})$/\1/p' "$ev" | tail -1)
test -n "$rg_id" || { echo "FAIL: no record-g line names the (g) record"; exit 1; }
rg_file=$(ls references/adr/"$rg_id"-*.md 2>/dev/null)
test -n "$rg_file" && test -f "$rg_file" || { echo "FAIL: ADR-$rg_id's file does not exist"; exit 1; }
grep -qxF 'status: accepted' "$rg_file" || { echo "FAIL: ADR-$rg_id is not accepted — both probes are recorded as passing, so the operator's dated flip is what is missing"; exit 1; }

# ---- (7) CI: GREEN ON THE RECORDED RUN, EVERY JOB, BOTH LEGS; THE CODE UNCHANGED SINCE; THE pnpm STEP PRESENT.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$ev" | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in $ev"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
[ "$fx_frozen" = "$ci_sha" ] || { echo "FAIL: the fix item's frozen sha $fx_frozen is not the CI-green sha $ci_sha"; exit 1; }
grep -qxF "story-item rk-story-harden-spike-pack $ci_sha" "$ev" || { echo "FAIL: the story item's frozen sha is not the CI-green sha"; exit 1; }
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln || { echo "FAIL: code or registry changed since CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
cy=$(grep -nE '^[[:space:]]+pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project$' .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ci.yml has no pnpm install step on a code line (D21-42 14a)"; exit 1; }
ct=$(grep -nE '^[[:space:]]+run: cargo test --workspace$' .github/workflows/ci.yml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ci.yml's test gate line was not found — the ordering check would compare nothing"; exit 1; }
[ "${cy%%:*}" -lt "${ct%%:*}" ] || { echo "FAIL: the pnpm step comes after cargo test — the node suite would still fail at its preflight"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
# SYNTHETIC CONTROL first: a run missing the windows leg must be refused, or the checker is vacuous.
v=$(python -c "$cichk" "$ci_sha" <<< "{\"conclusion\":\"success\",\"headSha\":\"$ci_sha\",\"headBranch\":\"build/phase-0-to-1\",\"jobs\":[{\"name\":\"gates (ubuntu-latest)\",\"conclusion\":\"success\"},{\"name\":\"no egress (AC-498)\",\"conclusion\":\"success\"},{\"name\":\"structural assertions\",\"conclusion\":\"success\"}]}"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the CI checker accepted a run with no windows leg (rc=$rc: $v) — it is vacuous"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }

# ---- (8) s8's AMENDMENTS LANDED (§8j.8): header, no BIND:, no test -x before a build, no bare run, the instruments.
s8chk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]
if "| `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — |" not in t: bad.append("header row")
if "BIND:" in t: bad.append("an unbound BIND: slot")
blocks=re.findall(f+"bash\n(.*?)"+f,t,re.S)
code="\n".join(l for b in blocks for l in b.split("\n") if not l.strip().startswith("#"))
if "git ls-files --error-unmatch" in code: bad.append("(1b) still asserts tree membership")
if "findings\\.none\\(evidence in \\[Reproduced, MutationSurvived\\]\\)" in code: bad.append("the unquoted-only d2 pattern survives (A5.2)")
for l in code.split("\n"):
    if re.search(r"grep\s+-r\S*\s", l) and re.search(r"\s\.redkiln/?(\s|\)|$)", l) and "--exclude-dir=worktrees" not in l: bad.append("a walk over .redkiln/ without --exclude-dir=worktrees (A5.5): "+l.strip()[:60])
for need in ("--exclude-dir=.git","--exclude-dir=worktrees","%(trailers:key=Story,valueonly=true)","stage_order","verify_completed","artifact.write","git cat-file -e","dispatch_completed","gh run view","* text=auto eol=lf","STORE_BAK","tools/live-run.sh run","ac184_rebuild_differential_over_anchored_tier_one_rows","findings\\.none\\(evidence in \\[|quality_shorthand"):
    if need not in code: bad.append("absent from code lines: "+need)
if "Those nineteen are not read, they are run." not in t: bad.append("the bullet count sentence")
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$s8chk" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: s8's amendments did not land: $v"; exit 1; }
# Golden constraint 3 in every s8 block: no `test -x ./target/debug/redkiln` before a guarded `cargo build --workspace`.
# Its OWN checker, so its synthetic control can fail for this reason and no other.
txchk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]
blocks=re.findall(f+"bash\n(.*?)"+f,t,re.S)
for i,b in enumerate(blocks):
    built=False
    for ln in b.split("\n"):
        s=ln.strip()
        if s.startswith("#"): continue
        if "cargo build --workspace" in s and "rc=$?" in s: built=True
        if "test -x ./target/debug/redkiln" in s and not built: bad.append("block %d: test -x before a build" % i)
print("; ".join(bad) if bad else "ok %d blocks" % len(blocks))
sys.exit(1 if bad or not blocks else 0)'
v=$(python -c "$txchk" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an s8 block runs the binary before building it: $v"; exit 1; }
# D21-42 17a in every s8 block: no bare `redkiln run` on a code line, and the launcher at all three run sites (A1, A7).
rnchk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
bad=[]; n=0
for i,b in enumerate(re.findall(f+"bash\n(.*?)"+f,t,re.S)):
    for ln in b.split("\n"):
        s=ln.strip()
        if s.startswith("#"): continue
        if re.search(r"(target/debug/redkiln(\.exe)?|\$\{?RK\}?)\"?\s+run(\s|$)", s): bad.append("block %d: a bare run: %s" % (i, s[:80]))
        if re.match(r"tools/live-run\.sh run ", s): n+=1
if n < int(sys.argv[2]): bad.append("%d launcher run lines, expected at least %s" % (n, sys.argv[2]))
print("; ".join(bad) if bad else "ok %d launcher runs" % n)
sys.exit(1 if bad else 0)'
v=$(python -c "$rnchk" runbook/phase-1/s8-exit-criterion.md 3); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an s8 run site does not go through tools/live-run.sh: $v"; exit 1; }
# SYNTHETIC CONTROLS: a build-first violation, an unskipped walk and a bare run must each be refused; the correct forms
# must pass.
bt=$(printf '%s' '`' '`' '`')
printf '%s\n' "$bt""bash" 'test -x ./target/debug/redkiln || exit 1' "$bt" > "$tdir/s8-syn-red.md" || { echo "FAIL: could not write the red control"; exit 1; }
printf '%s\n' "$bt""bash" 'out=$(cargo build --workspace 2>&1); rc=$?' 'test -x ./target/debug/redkiln || exit 1' "$bt" > "$tdir/s8-syn-green.md" || { echo "FAIL: could not write the green control"; exit 1; }
v=$(python -c "$txchk" "$tdir/s8-syn-red.md"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the build-first checker accepted a test -x with no build before it (rc=$rc: $v) — it is vacuous"; exit 1; }
v=$(python -c "$txchk" "$tdir/s8-syn-green.md"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the build-first checker refused a correctly ordered block (rc=$rc: $v) — it cannot pass"; exit 1; }
printf '%s\n' "$bt""bash" './target/debug/redkiln run "$X" --lane fix || exit 1' "$bt" > "$tdir/run-red.md" || { echo "FAIL: could not write the bare-run control"; exit 1; }
printf '%s\n' "$bt""bash" 'tools/live-run.sh run "$X" --lane fix || exit 1' "$bt" > "$tdir/run-green.md" || { echo "FAIL: could not write the launcher-run control"; exit 1; }
printf '%s\n' "$bt""bash" 'raw=$(grep -rnE x crates/ .redkiln/); rc=$?' "$bt" > "$tdir/walk-red.md" || { echo "FAIL: could not write the walk control"; exit 1; }
v=$(python -c "$s8chk" "$tdir/walk-red.md"); rc=$?
[ "$rc" -eq 1 ] && printf '%s\n' "$v" | grep -qF 'without --exclude-dir=worktrees' || { echo "FAIL: the amendment checker accepted a walk over .redkiln/ with no worktrees skip (rc=$rc) — the A5.5 check is vacuous"; exit 1; }
v=$(python -c "$rnchk" "$tdir/run-red.md" 0); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the run-site checker accepted a bare redkiln run (rc=$rc: $v) — it is vacuous"; exit 1; }
v=$(python -c "$rnchk" "$tdir/run-green.md" 1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the run-site checker refused a launcher run (rc=$rc: $v) — it cannot pass"; exit 1; }
# Every test name s8 selects through a *_TESTS variable exists.
nm='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
for v in re.findall(r"^[A-Z_]*TESTS=\"([^\"]*)\"",t,re.M):
    for n in v.split(): print(n)'
names=$(python -c "$nm" runbook/phase-1/s8-exit-criterion.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$names" ] || { echo "FAIL: s8 selects no test through a *_TESTS variable (rc=$rc) — A5.1 did not land"; exit 1; }
lst=$(cargo test --workspace -- --list 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --list rc=$rc"; exit 1; }
for n in $names; do printf '%s\n' "$lst" | grep -qxF "$n: test" || { echo "FAIL: s8 selects '$n', which no test binary lists"; exit 1; }; done

# ---- (9) THE THREE RUNBOOK CHECKERS, from the repo root: pass, and zero flagged.
out=$(python runbook/tools/check-checkpoints.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE 'flagged 0;' || { echo "FAIL: check-checkpoints rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }
out=$(python runbook/tools/check-refs.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE '^unresolved bare section refs: 0 ' || { echo "FAIL: check-refs rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }
out=$(python runbook/tools/check-bash-syntax.py 2>&1); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$out" | grep -qE ' 0 failed$' || { echo "FAIL: check-bash-syntax rc=$rc"; printf '%s\n' "$out" | tail -10; exit 1; }

# ---- (10) RE-RUN, WHOLE: THE ENTRY CHAIN ONCE, THROUGH ITS HEAD, AND THE TWO CHECKPOINTS THIS STAGE EDITED.
# THE CHAIN, ONCE, AND SAID SO. Every sub-stage checkpoint re-runs its own entry gate whole: 8i re-runs 8h, 8h re-runs
# 8g, and so on down to 8a, which re-runs 7i. Each also re-runs the older checkpoints its own edits made provisional,
# as its own file names them (8a: 5, 7a, 7b, 7f; 8e: 7h, 7f; 8f: 7i; 8i: 5; at authoring). ONE run of Checkpoint 8i is one
# run of the whole chain. Running 8a…8h separately here as well would repeat the chain once per link, and it is not
# done. 7h and 7i ARE run directly as well: §8j.1(c) edits both after every link that nests them was written. A nested
# run reads the file as it stands, so the chain runs the edited text too, and the direct runs are what name their
# markers in this block. The duplicate costs minutes. The chain costs hours.
# NO ACCIDENTAL DISPATCH (the brief's rule for every re-run of a checkpoint that executes `redkiln run` once s8h has
# landed — 7f inside 8a, 7h's and 7i's binary arms, and whatever else the chain carries):
#   · PREVENTION. Each block runs with REDKILN_LAUNCH_RECORD, FIX_ITEM, STORY_ITEM and ITEM unset. After s8h, a run
#     whose composition root chose the REAL harness refuses at entry without a launch record, "before
#     `reconcile_started` is appended and before any worktree operation" (s8h §8h.7). So no chain line can reach a real
#     spawn unless it forges the variable or calls the launcher, and the screen below refuses both on a code line.
#     Pointing a re-scoped checkpoint's own `run` at the fixture child through `[adapters] harness_binary` or a fixture
#     config is that checkpoint's owner's work (7f is s8a's), and this block does not re-do it.
#   · DETECTION. (12) asserts that the store's dispatch rows, every rk-run/* ref, the launch records and the session
#     transcripts are unchanged.
xblk='import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
f=chr(96)*3
hits=[b for b in re.findall(f+"bash\n(.*?)"+f,t,re.S) if sys.argv[2] in b]
if not hits: sys.exit(3)
open(sys.argv[3],"w",encoding="utf-8",newline="\n").write(hits[-1])'
SCREEN='tools/live-run\.sh[[:space:]]+(run|probe)|REDKILN_LAUNCH_RECORD=|git[[:space:]]+push'
syn=$(printf 'out=$(tools/live-run.sh probe live::x 2>&1)\n' | grep -E "$SCREEN"); test -n "$syn" || { echo "FAIL: the dispatch screen cannot see a launcher probe line — it is vacuous"; exit 1; }
# The screen reads every chain block its own file marks, before anything runs. It is a FLOOR: a nested block that a
# parent assembles at run time (5's, 7a's, 7b's) is screened by that parent, and (12)'s detectors cover all of them.
for pair in s7d-cli-and-migrate:CP7D s7f-conformance-and-records:CP7F s7h-lane-completes:CP7H s7i-cost:CP7I s8a-records-and-red-checkpoints:CP8A s8b-catalog-quality:CP8B s8c-toolchain-port-and-verbs:CP8C s8d-step-grain-and-resolver:CP8D s8e-tier-one-and-the-lane:CP8E s8f-landing-and-rendering:CP8F s8g-verify-and-restored-demands:CP8G s8h-cards-and-run-environment:CP8H s8i-s-bin:CP8I; do
  sf="runbook/phase-1/${pair%%:*}.md"; mk="${pair##*:}-END-OF-BLOCK reached"; blk="$tdir/${pair%%:*}.sh"
  test -f "$sf" || { echo "FAIL: $sf does not exist — its checkpoint cannot be re-run (Halt 8)"; exit 1; }
  python -c "$xblk" "$sf" "echo \"$mk\"" "$blk"; rc=$?
  [ "$rc" -eq 0 ] && test -s "$blk" || { echo "FAIL: no block in $sf echoes '$mk' (rc=$rc)"; exit 1; }
  hit=$(grep -vE '^[[:space:]]*#' "$blk" | grep -nE "$SCREEN")
  [ -z "$hit" ] || { echo "FAIL: $sf's checkpoint would launch, forge a launch record or push on a code line — this block runs nothing that does:"; printf '%s\n' "$hit"; exit 1; }
done
for pair in s7h-lane-completes:CP7H s7i-cost:CP7I s8i-s-bin:CP8I; do
  sf="runbook/phase-1/${pair%%:*}.md"; mk="${pair##*:}-END-OF-BLOCK reached"; blk="$tdir/${pair%%:*}.sh"
  out=$(env -u REDKILN_LAUNCH_RECORD -u FIX_ITEM -u STORY_ITEM -u ITEM bash "$blk" 2>&1); rc=$?
  last=$(printf '%s\n' "$out" | grep . | tail -1)
  [ "$rc" -eq 0 ] && [ "$last" = "$mk" ] || { echo "FAIL: $sf's checkpoint rc=$rc, last line '$last'"; printf '%s\n' "$out" | tail -25; exit 1; }
done

# ---- (11) THE WHOLE SUITE, NO FAIL-FAST, WITH A FLOOR; fmt; clippy.
# The floor is RE-DERIVED AT STAGE OPEN from Checkpoint 8i's recorded passed count (PROGRESS.md row 8i), plus this
# stage's 13 new tests. 780 is the drafted expectation: s8h's drafted floor (Checkpoint 8g's count + 40), with no
# workspace test added in s8i's draft. Edit the literal, with a date, when 8i's recorded count differs.
CP8I_PASSED=780
floor=$((CP8I_PASSED + 13))
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|test result:' | tail -30; exit 1; }
nres=$(printf '%s\n' "$out" | grep -cE '^test result: ')
nbad=$(printf '%s\n' "$out" | grep -E '^test result: ' | grep -vc ' 0 failed;')
tot=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="passed;") p+=$(i-1)} END{print p+0}')
[ "$nres" -ge 44 ] && [ "$nbad" = "0" ] && [ "$tot" -ge "$floor" ] || { echo "FAIL: $nres results, $nbad with failures, $tot passed — the floor is 44 results, 0 failures and $floor passed (Checkpoint 8i's $CP8I_PASSED plus this stage's 13; a floor is not a count)"; exit 1; }
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red (rc=$rc; never -D warnings on the CLI — ADR-0083)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (12) NOTHING DISPATCHED, NOTHING RUN, NOTHING LAUNCHED, NOTHING WRITTEN.
rows_after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$rows_after" = "$rows_before" ] || { echo "FAIL: the store moved during this block ($rows_before -> $rows_after) — a re-run block wrote to it, or something dispatched"; exit 1; }
refs_after=$(git for-each-ref --format='%(refname) %(objectname)' refs/heads/rk-run/); rc=$?
[ "$rc" -eq 0 ] && [ "$refs_after" = "$refs_before" ] || { echo "FAIL: an rk-run/* ref moved or appeared during this block — a lane ran:"; diff <(printf '%s\n' "$refs_before") <(printf '%s\n' "$refs_after"); exit 1; }
nl_after=$(find "$launches" -type f 2>/dev/null | sort)
[ "$nl_after" = "$nl_before" ] || { echo "FAIL: a launch record appeared during this block — something went through tools/live-run.sh"; exit 1; }
sess_after=$(find $CFG_DIRS -type f -name '*.jsonl' 2>/dev/null | sort)   # unquoted on purpose, as in (0)
new=$(comm -13 <(printf '%s\n' "$sess_before") <(printf '%s\n' "$sess_after"))
[ -z "$new" ] || { echo "FAIL: a new session transcript appeared while this block ran — something dispatched (or you started a Claude session; re-run without one):"; printf '%s\n' "$new"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

echo "CP8J-END-OF-BLOCK reached"
```

**Stage close.** The stage closes in two commits, as the preamble requires. The first commit holds the work:

- the items and intake files;
- the records at `proposed`;
- the lane fixture and the tests;
- `ci.yml` and the pin move;
- the `s7i` and `s7h` re-points;
- the `answer.rs` comment correction;
- the `s8` amendments;
- the Checkpoint 8j output.

The legacy checkout's re-point changes no tracked file, because `.redkiln/worktrees/` is ignored, so it belongs to
neither commit. The operator performs it before Checkpoint 8j and records it in the evidence file. The second commit is the tracker:

- `PROGRESS.md` §1 with the first commit's hash, and rows 7c, 7h and 7i;
- the `runbook/phase-1/s8j-evidence.txt` lines that name that commit;
- the ledger rows;
- `INDEX.md`'s execution order gaining `8j`.

Adversarial refuting rounds run until a round moves no code and no guard.

---
