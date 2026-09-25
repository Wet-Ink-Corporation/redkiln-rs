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
- **D21-44 / D21-45 W2.** The run-owned worktree is `.redkiln/worktrees/<item>`, and re-entry reuses it (`s8e`
  owns both). The re-entry rules in §8f.4 are rewritten over reuse.
- **Settled residuals.** Where the canonical typed value lives (§8f.5), the managed path (§8f.6), and how
  `activity_id_at_version` reaches the adapter (§8f.8) are stated as the records' decisions, not left open.
- **Handed on.** The `landing` and drift gate classes each need a `gate_expiry` row, in `s8h`'s table (§8f.1).
- **Floor.** The floor is re-derived at stage open from Checkpoint 8e's recorded count (§8f.2 item 9). The
  Checkpoint re-runs only 8e, plus 7i, which this stage makes provisional (§8f.13).
- **Found in revision:** `item_create`'s refusal of an existing id would refuse its own re-execution (§8f.7).
  `artifact.read`'s seam lands after the stage that emits through it (§8f.2 item 10).

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
  with reuse (*"Reuse, never force-remove"*), and that work is `s8e`'s.
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
this stage keeps. The ledger is `spikes/DECISIONS-2026-09-21.md`. What those decisions leave free is named in §8f.3
as each record's to settle. Nothing here is left for the implementer to settle. **No record number is
pre-assigned**: each record takes the next free id at writing, because several `s8` sub-stages write records and
their order is not fixed. On 2026-09-24, `references/adr/README.md` gives **`0165`** as the next free id.

🛑 **No real dispatch happens at this stage.** Every behavioural proof runs a `FakeHarness` or a fixture corpus
(D21-40 decision 2: real runs *"once the sub-stages' fixture proofs pass, and not before"*).

### 8f.1 What this stage owns, what it hands on, and the order inside it

| decision (ledger) | what it makes | section |
| --- | --- | --- |
| D21-41 (b); D21-42 9; D21-43 N10 | The kernel stages the run-owned worktree. A content checkpoint is made at every committed step that changed the tree, in one boundary act with that step's commit. Every tier-1 boundary rides on a content commit when one is due. Refused paths are refused by name. There is no integration. `explain` prints the hand-off | §8f.4 |
| D21-42 5a; D21-43 N11 | One record owns `rk-store`'s render and write path. **C2** is resolved as surface-then-restore, and the surface opens a drift gate that blocks the stage's exit | §8f.5 |
| D21-43 N12 (the seam) | `rk-store`'s read seam: one `artifact.read` per read, refused before the entry's `reconcile` | §8f.5 |
| D21-42 5b | Rendered artifacts land in the item's managed directory inside the run-owned worktree. The run path gets its `artifact.write` producer | §8f.6 |
| D21-42 6a (this stage's half) | `DebtRegisterEntry`'s in-repo item write, through `rk-store`, with provenance | §8f.7 |
| D21-42 12b; D21-43 N5 (cited), N12 (AC-372) | `dispatch_completed` gains `activity_id_at_version`, and its `item` is `s8e`'s N5 key. AC-395's per-activity breakdown and AC-372's `status` half are built | §8f.8 |
| D21-42 12c; D21-43 N12 (AC-159) | `doctor` reads the store's journal. The flat-reading silencing of `RK-D-HEADROOM-MARGIN` is fixed. AC-159's rule is built | §8f.9 |

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
  re-created (D21-45 W2). `s8e` owns the path, `worktree_assigned`, the reuse and the journalled discard.
- **The interim `item` key** on every item-scoped event kind (N5) is `s8e`'s record.

From `s8d` (Checkpoint 8d) this stage takes the resolved emit binding, step-grain commits and re-entry (D21-42
3a/3b). It gives the emit arm its effect, and it gives the resolver's `artifact:` arm the read seam N12 names. From
`s8c` it takes the verb registry's `store` route. `s8c` lands the `item_create` row as *"Refused naming the route
until `s8f`"*, and this stage builds its handler (§8f.7).

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

**Order.** (i) The stage-open measurements (§8f.2). (ii) The three records, written `proposed` before any code they
authorise. (iii) The landing: `rk-git`'s primitive, then the kernel wiring on `s8e`'s seam (the step-grain act and
the boundary act), then the `rk-cli` end-to-end tests. (iv) The render, write and read paths, then the drift gate,
then the emit producer and the resolver's read, then the item write. (v) The `activity_id_at_version` member, then
`cost` and `status`. (vi) `doctor`'s journal, then AC-159's rule. (vii) The dated notes (§8f.10). (viii) Checkpoint
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
   appended **after** the content checkpoint commit, with that commit as its `anchor_sha`. Measure where `s8d`'s
   walk appends an agent step's commit row and an activation's `emit` commit, and whether `s8e`'s boundary function
   can be called there with a step-grain key rather than a stage boundary. If the row is appended before any git
   write, or the function takes only stage and gate boundaries, STOP. The seam's support is `s8e`'s
   (D21-43 N10's boundary act runs on D21-42 8a's seam), and this stage does not build a second seam.
4. 🛑 **No golden log or pinned fixture holds a `dispatch_completed` row.** Read the golden logs under `CORPUS_PIN`
   and every `-text`-pinned fixture in `.gitattributes`. ADR-0131's *cheap* ground is *"every golden log's canonical
   events change, the corpus regenerates, `CORPUS_PIN` moves"*. ADR-0163 answered it with *"no golden log contains a
   `rate_limit_reading` row"*. If a `dispatch_completed` row sits in a golden log, §8f.8's member moves it. STOP and
   put the cost to the operator. `s8e`'s N5 key meets the same question first; read how Checkpoint 8e answered it.
5. **What `ActivitySpec`'s serialised form enters.** Does it enter a `run_id`, a card digest, a memo key, or
   ADR-0151's pinned run-record fixture? The `activity_id_at_version` carrier (§8f.8) is omitted when `None`, on
   D18-14's `Config` precedent (*"omitted from `to_json()` when `None`"*). This measurement shows whether that care is
   enough.
6. **The run-owned worktree and the shipped store.** At `a6349b6` one run-owned worktree is registered. It is the
   legacy tree `.git/redkiln/worktrees/rk-fix-doctor-test-count` on `rk-run/rk-fix-doctor-test-count` at `73c087a`,
   which stays where it is until `s8j` re-points it (D21-44). Its `status --porcelain=v1 --untracked-files=all` is
   **empty**. New run-owned worktrees are created under `.redkiln/worktrees/<item>`, and `.gitignore` carries the
   row `.redkiln/worktrees/` (D21-44's commit). The shipped store (`mode=ro`) holds `t2_event` `dag_assembled` 1,
   `reconcile_started` 3 and `stage_transition` 2. All three `t1_*` tables hold 0 rows.
7. **The denylist and the size limit's warrant.** At `a6349b6`, **0** of the 1,836 tracked paths match §8f.4's
   denylist globs (1,826 at `ca8f387`). The largest tracked file is still **1,439,568 bytes**
   (`references/audit/2026-09-10/GROUNDING-MERGED.json`). Any limit the landing record declares is read against that
   number.
8. **The test module names are free.** This stage adds these modules: `rk-git` `landing`; `rk-store` `render` and
   `item_write`; `rk-kernel` `emit` and `step_landing`; and `rk-cli` `landing`, `item_create`, `status`,
   `doctor_journal` and `doctor_artifact`. If a predecessor stage created one of them, the tests go into it. The
   module-qualified names in §8f.12 stay as they are.
9. **The whole-suite floor, re-derived at stage open from Checkpoint 8e's recorded count.** Checkpoint 8f's
   `FLOOR` is Checkpoint 8e's recorded passed count plus this stage's **64** new tests. The drafted chain gives the
   expected value. `ca8f387`'s whole suite measured 740 passed, 0 failed and 9 ignored over 44 results (ADR-0164's
   evidence, `992d257`). The drafted floors then add `s8a` 7, `s8b` 16, `s8c` 40, `s8d` 46 and `s8e` 41, which gives
   890 at Checkpoint 8e and **954** here. Replace 954 with the recorded count plus 64, by a dated edit of that line,
   before the block's first run. The floor is a lower bound, and it must not be a stale one.
10. 🛑 **Where `s8d`'s `artifact:` arm reads, and whether it already emits.** N12: *"`artifact.read` is emitted by
    s8d's resolver through s8f's rk-store read seam"*. `s8d` closes before this stage, so the seam does not exist
    when `s8d`'s resolver is built. As drafted, `s8d` §8d.3's `artifact:` row reads the committed `emit` binding
    *"from the journal (tier 2)"*. Measure the arm at Checkpoint 8d's close. If it calls a stub or a named function,
    this stage re-points that one call site to the seam. If it already appends `artifact.read` itself, STOP: that is
    a second producer outside the seam, and the owner must be settled before either is kept.

### 8f.3 The three records

Each record is written in ADR-0085's form: frontmatter, context, drivers, `### Option — <name>` headings (D21-42 24a
enforces these by lint from ADR-0164 on), decision, consequences with a **What this forbids** block, and evidence.
Each is committed `proposed`. Each flips only on the operator's dated line under `spikes/DECISIONS-2026-09-21.md`
(ADR-0126's route). Checkpoint 8f locates each record **by a phrase in its title**, so the title carries that
phrase.

| record | title carries | decides | relation to the accepted set |
| --- | --- | --- | --- |
| **the landing record** | `landed commit` | §8f.4 whole: what "landed" means; the step-grain boundary act (N10) and the boundary checkpoint; how a step checkpoint names its key for `reconcile`'s heal; the staging rule and its baseline; the denylist, the size limit and the form of the refusal; how each commit is made; the gate's tree; no integration; the hand-off; the discarded set at re-entry | Execution under ADR-0105 clauses 3–5 and ADR-0109 decision 2, as D21-41 (b) says, as D21-43 N10 widens it. It records that N10 replaces (b)'s *one* content checkpoint per boundary. It must **state its reading** of ADR-0105's forbids bullet *"forbids including untracked files in it"*. It must also state its reading of clause 3's *"plumbing is `commit-tree` + `update-ref` where the wrapper needs an exact parent"* as the warrant for the two `GitOp` variants §8f.4 needs. If writing finds that a `## Decision` sentence moves, that is a supersession, and the record says so (ADR-0092 rule 1). Clause 6's *"re-created, never reused"* is `s8e`'s D21-44/W2 record's to supersede, and this record cites it |
| **the render record** | `render and write path` | §8f.5–§8f.7 and §8f.9's AC-159 rule: layout-C rendering, section stamps, prose, the drift pass (C2) and its drift gate (N11), where the canonical typed value is kept, the managed directory, the write seam, the read seam (N12), one `artifact.write` per write and one `artifact.read` per read, `.redkiln/items/` as a managed tree, and the unrecorded-mutation rule | It owns AC-411, AC-477 and AC-478, which are RAISED with no owner record. ADR-0005/0006 (as refined by ADR-0075) and ADR-0116 stand. It drives dated register amendments (§8f.10). It states N11's reading of AC-125 and records that AC-127 holds unamended (§8f.5) |
| **the telemetry-key record** | `dispatch_completed carries` | §8f.8–§8f.9: the `activity_id_at_version` member and its carrier, the breakdown grammar, the `status` renderer, AC-372's doctor clause read as a renderer test (N12), and `doctor`'s journal source and five-hour reading | **Supersedes in part:** ADR-0131's first forbids bullet, for `dispatch_completed.activity_id_at_version` alone, until 0131's tag is wired. The `item` member is N5's and rests on `s8e`'s record, which this record cites. It also supersedes in part ADR-0163 decision 4's closed-grammar sentence, decision 5's not-delivered list and its sentence on `doctor`'s journal, and ADR-0163's bullet on *"any other payload correlation key"* for this one member (§8f.8) |

### 8f.4 The landing checkpoint (D21-41 (b); D21-42 9; D21-43 N10)

**What "landed" means in Phase 1.** A landed commit is a kernel-made content checkpoint commit on `rk-run/<item>`.
After the lane's terminal mark (N9), the diff `<base>..rk-run/<item>` is not empty. **Nothing is integrated.**
Design §7.7's rule stands: *"The queue, not the agent, is the authority on what lands"*. The design §10.2 Phase-2 row
schedules *"project DAG in parallel worktrees plus a merge queue"*. So `GitOp::Integrate` keeps no caller. AC-196 and
AC-197 receive a dated disposition that routes them to Phase 2's `rk-git` merge-queue stage (§8f.10). The operator
makes the hand-off: they merge `rk-run/<item>`, and `explain` prints the command (below).

**When a checkpoint is made — two kinds of point (D21-43 N10).**
- **At every committed step that changed the worktree.** `s8d` commits each agent step under its step-grain key, and
  an activation completes when its `emit` binding commits (`s8d` §8d.2). When such a commit finds the staged tree
  different from `HEAD^{tree}`, the kernel lands it **in the same boundary act**. Through `s8e`'s seam it first makes
  the content checkpoint commit (git-first, D21-42 8a). Only then does it append the step's commit row, whose
  `anchor_sha` is that commit. A step that changed nothing makes no commit. N10 says: *"Land at every committed step
  that changed the worktree. The commit and the content checkpoint are one boundary act."* This replaces the
  draft's single checkpoint "when due" at a boundary, and its void-and-re-run route. A committed step's tree effect
  is never left unlanded, so there is nothing to void.
- **At every tier-1 boundary.** These are a stage's `Story:`, a `Gate:` transition (N6) and the lane's terminal
  `closed` mark (N9). The trailer rides on a content commit when the staged tree differs from `HEAD^{tree}`. AC-245
  requires this: *"when a checkpoint commit is due, the trailer commit's tree"*. Otherwise it rides on an empty
  commit: ADR-0109 decision 2's *"a trailer on a checkpoint commit or an empty commit carrying the trailer"*, and
  AC-163's empty commit. Under N10 a boundary usually finds the tree already landed. What it still carries is the
  work of steps that do not commit, since `s8d` §8d.2 says *"A `tool` or `check` step does not commit"*. An example
  is `item_create`'s write (§8f.7). The other is the entry-time render restore (§8f.5).

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
`s8e`'s re-entry discards exactly it (below).

**What is refused, and how.** Every path to be staged is checked, run-created or tracked, before anything is written.
- **The denylist.** A path is refused if it matches `.env`, `.env.*`, `*.pem`, `*.key` or `id_*`, or one of the
  credential-file patterns the record fixes. The record's list starts with `credentials*`, `*.p12`, `*.pfx`,
  `.netrc` and `.pgpass`.
- **The size limit.** A blob over the declared limit is refused. The limit is a named constant with its warrant, and
  §8f.2 item 7 gives the measured ceiling it is read against.

A refusal is **by name**: it gives each path and the rule it broke. It is **never a silent drop**. Halt 7 applies
to anyone tempted to widen a glob or raise the limit to make a run pass.

The *form* of the gate-worthy exit is the landing record's to fix. This file's route, which the record adopts or
replaces with a stated reason:
- The landing lands **nothing**, and the step whose act it was **does not commit**. So no committed step is left
  with an unlanded effect. The landing opens a human gate of class `landing.refused-path`, whose record names every
  refused path and its rule.
- Its `Gate:` commit (N6) is **empty**. Its tree is ADR-0105 clause 4's no-checkpoint-due derivation, so `answer`'s
  identical derivation cannot void the gate (AC-176).
- `run` exits `0` at the gate. That is golden constraint 2, and `crates/rk-cli/src/exit.rs`'s `Success` row: *"**and a
  run that opens a gate** (D-01: AC-403's "exits 0", AC-240, AC-024)"*.
- The gate's question (ADR-0129 decisions 3–4) offers `exclude` and `abort`. `exclude` means every later landing of
  this lane leaves exactly those paths unstaged, and names them each time. `abort` stops the lane.
- The class needs a `gate_expiry` row in `s8h`'s table (§8f.1).

A new exit-table row was considered and rejected: AC-438's table is a closed, snapshotted set, and a gate uses the
one answer channel (ADR-0009) without waiting.

**The cost is stated, not hidden.** The refused step did not commit. At the next entry `s8e`'s reuse discards its
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

`commit-tree` and `update-ref` are clause 3's named plumbing, and the `GitOp` variants they need are added under that
clause. The record names each variant, because *"adding a variant is that decision made visible"* (`git_exec.rs`).
Every one of these operations goes through `s8e`'s seam, reads included. On replay the recorded T and commit are
served from N7's `git_op` rows, and `rk-git` performs **zero** writes (ADR-0105 clause 1).

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
semantics. A tree without `RK_BASE`, or with commits the run ref lacks, is **refused, never removed**."* `s8e` owns
the reuse, the refusal and the discard's journal row. This stage owns what lands, so it fixes three things:
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

**C2 — surface, restore, and gate (D21-42 5a; D21-43 N11).** At every entry, after `s8e`'s re-entry discard and
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
  is the boundary: `s8e`'s stage advance folds `t1_gate` (N6) and refuses to leave the stage while a drift gate for
  the item is unanswered. When the stage has nothing left to run, `run` exits `0` at the gate, like any gate. The
  class needs a `gate_expiry` row in `s8h`'s table (§8f.1).
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

**No artifact type carries a drift field or a signature-comparison field.** AC-478's *Verified by* keeps that
assertion. The one clause of AC-478 that falls is *"AND no comparison of stored-vs-regenerated rendering is
performed, so no drift state exists to report"*, by a dated amendment (§8f.10). The drift gate is a gate row, not an
artifact field.

**The single write path (AC-122).** The writer is a struct seam in `rk-store`, on `GitOps`' precedent: *"A struct,
never a trait"*, since Checkpoint 7a counts the port traits and a new one would move the port surface. It is
journalled, and on replay it performs zero writes and serves the recorded digest. No filesystem-write handle into a
managed directory is exported from `rk-store`. A `trybuild` compile-fail case proves that a sibling crate cannot
write there.

**One `artifact.write` per write.** Each write appends exactly one row: kind, item, `schema_version`, and the content
digest. `schema_version` is the kind's registered `schema` string, or `null`, per design §5.4's R-49 note. The digest
comes from the same `rk-types` function the typed-artifact digests use (ADR-0116 decision 6: *"No second digest
algorithm enters the system"*). That function is the one `answer.rs` already calls.

**The read seam (D21-43 N12).** The reader is the write seam's sibling in `rk-store`, and it is a struct too. It
serves an artifact's typed value from its sidecar, together with the value's digest, and it appends **one
`artifact.read`** naming the kind and the item. That is AC-472's first clause: *"an `artifact.read` event is emitted
naming the artifact kind and id"*. It **refuses** a read when the current entry has no reconcile record. That is
AC-126's *"`reconcile()` runs before any read of the artifact is served"*, and no flag, cache or fast path skips it.
The emitter is `s8d`'s resolver. Every resolved `artifact:` input goes through this seam, and N12 names that split
of ownership. §8f.2 item 10 says how the one call site is re-pointed. The seam checks that the digest it serves
equals the committed `emit` binding's digest, which is what `s8d`'s arm reads. A mismatch is a halt (Halt 4), never a
silent choice of one. On replay the seam serves the recorded digest and appends nothing new. Once rows exist,
`RK-D-UNREAD` (AC-472's second clause, built at `s7d`) reads real ones. Today it reads none, because nothing
produces them.

### 8f.6 Where rendered artifacts land, and the run-path `artifact.write` producer (D21-42 5b)

**The managed directory — settled by the render record.** An item's rendered artifacts are written by `rk-store`
only, under `.redkiln/artifacts/<item>/` inside the **run-owned worktree** (ADR-0137 clause 3: *"**`project_root` is
the run-owned worktree**"*). From the checkout that is `.redkiln/worktrees/<item>/.redkiln/artifacts/<item>/`
(D21-44). The path is excluded from nothing but itself, and this file reads that as follows:
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

### 8f.7 `DebtRegisterEntry`: the in-repo item write (D21-42 6a, this stage's half)

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
  `file` is a `tool` step, and `s8d` §8d.2 re-executes a tool step on re-entry. An `on_false` loop-back (D21-42 4d)
  re-executes it under a new iteration path. Either way it meets the file it wrote, once that file has landed. So
  the rule is split in two. A file byte-identical to the rendering is **served**: nothing is written and no
  `artifact.write` is appended. A file that differs is **refused by name, never overwritten**.
- **Facets.** A facet whose registry `source` is `human` (or `llm`) is **never written**. That holds even when the
  extracted value carries one: golden constraint 1, and DECISIONS-2026-09-12 #15. A withheld value is **named** in
  the provenance table, not dropped silently. `build_item` already nulls a declared facet the record omits (*"Every
  declared facet the record does not carry, as an abstention"*). So when a minted item is run, the selection
  abstains and opens an `unknown_facet` gate. `item_create::a_minted_debt_item_opens_an_unknown_facet_gate_when_run`
  asserts it through the binary.
- **No artifact kind is minted.** `ItemRefs` is a value, and the registry is not widened.

### 8f.8 `dispatch_completed`'s interim members, AC-395's breakdown and AC-372's `status` half (D21-42 12b)

**The two members, and whose record each rests on.** `dispatch_completed`'s payload carries `item` and
`activity_id_at_version`, each a string or `null`.
- **`item` is N5's** (D21-43): *"An interim `item` key on every item-scoped event kind, moving to ADR-0131's tag
  later as ADR-0163's interim key does; `query` honours `ByItem`."* 12b's `item` is one case of it.
  `s8e`'s record supersedes ADR-0131's first forbids bullet for it, and this file cites that record and does not
  re-specify it. If Checkpoint 8e's item-scoped kinds do not include `dispatch_completed`, this stage adds the
  member under `s8e`'s record, and it copies it exactly as the reading's is copied: from `ActivitySpec.inputs`'
  `"item"` (`ITEM_KEY`, whose doc says the key is *"Read for attribution only"*).
- **`activity_id_at_version` is this stage's.** It is interim too. When ADR-0131's tag is wired it moves to a tag,
  and the sentences allowing it lapse, on ADR-0163's own clause. The telemetry-key record therefore supersedes in part:
  - ADR-0131's first forbids bullet (*"Forbids putting an item id, a run id, … inside `rk-types::Event.payload` for
    the purpose of addressing"*), for this one member alone;
  - ADR-0163's bullet *"Forbids reading this record's `0131` supersession as licensing any other payload correlation
    key, on any other kind"*, for this one member alone. N5's `item` is `s8e`'s.

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
function** `cost` uses. A flag therefore cannot be dropped on one surface and kept on the other (*"the `redkiln cost`
and `redkiln status` renderers print the flags alongside any reading that carries them"*). An item with no reading
gets no headroom line. The shipped store holds zero readings, so `status` output on the shipped tree does not move.

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
  amendment).
- **No walk.** The rule **never walks** `.redkiln/` recursively. D21-44's `worktrees/` exclusion therefore holds by
  construction, and a synthetic control proves it (§8f.12).
- **Its rejecting fixture.** `ac415_every_rule_id_has_a_rejecting_fixture` joins rule ids to
  `crates/rk-cli/tests/fixtures/doctor/<id>/` in both directions, so the rule lands with its rejecting fixture: one
  managed file edited out-of-band. Its control is a file rewritten *through* the store (AC-159's *Verified by*). The
  fixture is data, and it adds no `#[test]` to `doctor.rs`.

**The tests go in new modules, `doctor_journal` and `doctor_artifact`.** They do not go in `tests/it/doctor.rs`.
D21-42 18 keeps `rk-fix-doctor-test-count` as *"the untouched fixture item, prose defect included"*. Its defect is
`doctor.rs`'s prose count (*"thirty-eight"*) against its **39** `#[test]` functions (re-counted at `a6349b6`). A test
added to that file would move the number the fixture item is about. The new rule does move the rule table from 39
ids to 40. `doctor.rs`'s prose *"The table is thirty-eight"* was already stale, and it is left alone for the same
reason.

### 8f.10 Register, design and runbook notes this stage owes

Every note is a **dated in-place amendment**, never an erratum (DECISIONS-2026-09-15 D15-2). Each goes **after** its
criterion's *Verified by:* line, as AC-472's amendments do. Checkpoint 8f reads the THEN and *Verified by* spans, and
a note inside them would read as the criterion.

| site | amendment | Checkpoint 8f finds it by |
| --- | --- | --- |
| AC-245 | The untracked-file property in *Verified by* is scoped to the no-checkpoint-due basis. At a due checkpoint, and at every step checkpoint (N10), run-created files are staged before the derivation, so the derivation still sees no untracked file | `run-created` in the entry |
| AC-473 | Its reading of *"a landed commit"*: §8f.4's first paragraph. Integration is Phase 2's merge queue | `rk-run/` in the entry |
| AC-521 | Its reading of *"authored into this repository's own backlog"*: the story artifact on `rk-run/<item>`, reachable through its `artifact.write` digest. The operator's merge is the hand-off | `rk-run/` in the entry |
| AC-478 | Its THEN loses only *"AND no comparison of stored-vs-regenerated rendering is performed, so no drift state exists to report"*, and surface-then-restore replaces it. *Verified by*'s *"no artifact carries a drift or signature-comparison field"* stays | the THEN no longer carries the phrase |
| AC-125 | A reading note (D21-43 N11): "resolving the drift" means answering the drift gate that the `structured_drift` opened, with `confirm-restore` or `move-to-prose` | `drift gate` in the entry |
| AC register §0.3 row C2, and the section's two "only C2 remains" sentences | C2 is resolved, on D21-42 5a, D21-43 N11 and the render record | `resolved` in the row |
| AC-196, AC-197 | A dated disposition: Phase 2's `rk-git` merge-queue stage (design §10.2's Phase-2 scope) | `Phase 2` in each entry |
| AC-372 | *Verified by*'s doctor-rule clause is read as a test of `cost`'s renderer, which `status` shares, with AC-415's red case (D21-43 N12; §8f.8) | ``test of `cost` `` in the entry |
| design §5.4 | The tier-1 row: a checkpoint at every committed step that changed the tree (N10) and at every boundary when due. The `structured_drift` row: its payload, and that it opens a drift gate. The `artifact.write` and `artifact.read` rows: their producers exist. The `dispatch_completed` row: `activity_id_at_version` (the `item` note is `s8e`'s, under N5) | read at the adversarial pass |
| `s7i` | `s7i` (5)'s `GRAMMAR` widens to the breakdown forms. Its "owns no reading" branch also requires zero owned `dispatch_completed` rows, because after §8f.8 an item with dispatches and no reading prints more than one line. `s7i` §7i.2's *"nothing else"* takes a dated note | Checkpoint 7i re-run whole (Checkpoint 8f (9)) |
| `s7d` | `s7d` §7d.7's *"prints **from the recorded resolution only**"* takes a dated note for the landing block (§8f.4) | read at the adversarial pass |

**No longer owed, and why.** Three rows the draft carried are dropped:
- **AC-127** holds as written (N11).
- **AC-013** needs no note, because N10 leaves no committed step with a void tree (§8f.4).
- **AC-518's and AC-529's path wording** are `s8e`'s notes under D21-44.

### 8f.11 Acceptance set

**Claimed:**
- AC-120, held: no parser enters.
- AC-121, its same-OS half.
- AC-122, AC-123, AC-124, AC-144, AC-152.
- AC-125: the restore, the drift gate as the only resolution, and no drift text reaching a deserializer.
- AC-126: the read seam refuses before the entry's reconcile. A lane's replay shows every `artifact.read` after
  its entry's reconcile record. `run` has no skip flag.
- AC-127, the rendered-section clause: a structured drift blocks the stage's exit until it is answered.
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
- AC-518, the landing: the checkout is unchanged, and only the run ref moves.

**Excluded, each by name:**
- **AC-121's cross-OS half**: it needs CI on both OS legs (D21-42 14a, `s8j`).
- **AC-127's vouched-root half, and its *Verified by*'s "`doctor` rule-table row distinguishing the two drift
  kinds"**: advisory drift on a human-supplied root (ADR-0025) is not built here. **No stage owns it.** It is
  RAISED to the lead, and it is not claimed.
- **AC-128**: no template V+1 exists in Phase 1. The stamp carries the version, so it is buildable later.
- **AC-153**: its CLI-only mutation channel is not built here.
- **AC-196, AC-197**: Phase 2.
- **AC-473, AC-521**: the runs are `s8`'s. This stage supplies the landed commit and the producer.
- **AC-529**: `s8e`'s 8d.

### 8f.12 Tests, red first, and the mutants that must flip them

Every test is written first and **watched fail for the right reason**: the behaviour is absent, not a compile error,
a typo'd filter or a missing fixture (golden constraint 12). Names are module-qualified. **64 new tests.**

- **`rk-git` `landing` (9):**
  - `landing::run_created_file_is_staged_and_an_ignored_file_is_not`
  - `landing::baseline_untracked_file_is_never_staged`
  - `landing::tracked_modification_and_deletion_are_staged`
  - `landing::not_due_makes_an_empty_commit_and_due_makes_one_content_commit`
  - `landing::denylisted_path_is_refused_by_name_and_no_ref_moves` (table-driven over every glob)
  - `landing::blob_at_the_limit_lands_and_one_byte_over_is_refused`
  - `landing::the_trailer_tree_is_the_commits_own_tree`
  - `landing::replay_serves_the_journal_and_performs_zero_writes`
  - `landing::a_repository_hook_does_not_run_on_a_landing_commit` (ADR-0105 clause 2's fixture)
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
- **`rk-kernel` `emit` (3):**
  - `emit::every_activity_output_appends_one_artifact_write`
  - `emit::an_emit_replayed_writes_nothing_and_serves_its_recorded_digest`
  - `emit::every_artifact_input_resolution_appends_one_artifact_read`
- **`rk-kernel` `step_landing` (2):**
  - `step_landing::a_committed_step_that_changed_the_tree_lands_before_its_commit_row`
  - `step_landing::a_committed_step_that_changed_nothing_makes_no_commit`
- **`rk-cli` `landing` (12)**. Each is a `FakeHarness` lane through `verbs::run::run`, on the harness `s8e`'s
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
- **`rk-cli`, the rest (18):**
  - `explain::explain_prints_the_landed_commit_and_the_merge_command`
  - `item_create::a_minted_debt_item_opens_an_unknown_facet_gate_when_run`
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
  - `doctor_artifact::an_out_of_band_structured_edit_is_reported_as_an_unrecorded_mutation`
  - `doctor_artifact::a_rewrite_through_the_store_is_not_reported`
  - `doctor_artifact::a_prose_edit_accepted_by_reconcile_is_not_reported`
  - `doctor_artifact::an_unassigned_copy_under_redkiln_worktrees_is_never_read` (the synthetic control for D21-44's
    exclusion: a mutated copy no `worktree_assigned` row names)
- **`rk-adapter-claude` `dispatch` (1):**
  - `dispatch::dispatch_completed_carries_activity_id_at_version_or_null`

`cost::cost_output_grammar_is_closed` is **amended**, not added. `doctor::ac415_every_rule_id_has_a_rejecting_fixture`
gains a fixture directory, not a test.

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

### 8f.13 Checkpoint 8f

- **The three records exist and are accepted.** Each has two or more `### Option — ` headings and a **What this
  forbids** block, and each has its README row and its dated DECISIONS line. The telemetry-key record supersedes 0131
  and 0163 in part.
- **The register carries every note §8f.10 names.** These notes are the BEFORE assertions (PIA-035). Each was
  measured **false** on the untouched tree. The AC-125 and AC-372 needles were re-measured at `a6349b6`: 0 hits for
  `drift gate` and 0 for ``test of `cost` `` in the whole register.
- **All 64 named tests pass, plus the amended grammar test and four existing guards.** Each is selected by its
  module-qualified name, with its own `... ok` line and an exact count.
- **The whole suite passes.** `--no-fail-fast`, rc 0, `passed >= FLOOR`, zero failed. fmt and clippy are green.
- **No integration and no float.** These are code-lines-only floors beneath the behavioural tests, and each has a
  synthetic control that drives it red.
- **This block leaves the shipped store unchanged.** It re-runs only its entry gate, Checkpoint 8e, whole. It also
  re-runs Checkpoint 7i whole, because §8f.10 amends `s7i` and that makes 7i provisional. It re-runs nothing else.
  Both reach their end markers. The worktree path set is unchanged, and the checkout is clean.
- **Nested cost and dispatch, stated.** A whole re-run of 8e runs whatever 8e's own block re-runs. That is the
  cross-cutting rule's depth, and the stage records the wall time it measured rather than skipping a line. No real
  `claude` can be spawned by this block at this stage. `s8h` has not landed, and at `a6349b6` **0** of the 8 shipped
  cards carry `permission_mode`, which `card::lint_card` refuses as `PermissionModeAbsent` at bind. 7i's run arm is a
  `cargo test` over a fixture transcript. After `s8h` lands, a later whole re-run of this block (from `s8j`) passes
  through 8e's and 7i's blocks as they stand then. Those blocks point `[adapters] harness_binary` at the fixture
  child under the cross-cutting rule, and they assert that no real `claude` was spawned.

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

# ---- (1) BUILD-OR-FAIL, and the one environment precondition the whole suite has (a fresh checkout fails ten node
# ---- tests at tests/it/support.rs's preflight without it).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8f NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: run pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project first — the suite below would fail for a reason unrelated to this stage"; exit 1; }

# ---- (1b) THE SHIPPED STORE'S ROW COUNTS, read-only, BEFORE this block's own sections. `mode=ro`, never `immutable=1`
# ---- (immutable ignores the WAL). The nested re-runs in (9) are AFTER the after-read and assert their own stores.
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store"; exit 1; }
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t2_event","t1_stage_transition","t1_gate","t1_slice_seal")))'
store_before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$store_before" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }

# ---- (2) THE THREE RECORDS, located by the title phrase §8f.3 requires, among ids >= 0165. BEFORE: none exists.
recs='import glob,os,re,sys
phr=["landed commit","render and write path","dispatch_completed carries"]
hits={p:[] for p in phr}
readme=open("references/adr/README.md",encoding="utf-8").read()
dec="".join(open(f,encoding="utf-8").read() for f in glob.glob("spikes/DECISIONS-2026-*.md"))
for f in sorted(glob.glob("references/adr/0*.md")):
    base=os.path.basename(f)
    if not base[:4].isdigit() or int(base[:4])<165: continue
    t=open(f,encoding="utf-8").read()
    m=re.search(r"^title:(.*)$",t,re.M)
    title=m.group(1).lower() if m else ""
    for p in phr:
        if p in title: hits[p].append((base[:4],t))
bad=[]
for p in phr:
    if len(hits[p])!=1:
        bad.append("%d records carry %r in their title, not exactly 1"%(len(hits[p]),p)); continue
    i,t=hits[p][0]
    if len(re.findall("^### Option \u2014 ",t,re.M))<2: bad.append(i+": fewer than two Option headings (ADR-0085; D21-42 24a)")
    if "**What this forbids**" not in t: bad.append(i+": no What this forbids block")
    if not re.search(r"^status: accepted\s*$",t,re.M): bad.append(i+": not accepted - a record flips only on the operator dated line")
    if "["+i+"](" not in readme: bad.append(i+": no README index row")
    if ("ADR-"+i) not in dec: bad.append(i+": no dated DECISIONS line names ADR-"+i)
if len(hits[phr[2]])==1:
    sup=re.search(r"^supersedes:(.*)$",hits[phr[2]][0][1],re.M)
    if not (sup and "0131" in sup.group(1) and "0163" in sup.group(1)): bad.append("the telemetry-key record does not supersede 0131 and 0163 in part")
print("\n".join(bad)); sys.exit(1 if bad else 0)'
out=$(python -c "$recs" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the records this stage owes are missing or malformed:"; printf '%s\n' "$out"; exit 1; }

# ---- (3) THE REGISTER NOTES §8f.10 NAMES. BEFORE (measured at ca8f387; the AC-125 and AC-372 needles re-measured at
# ---- a6349b6): every probe below reads FALSE. The AC-478 read is scoped to the THEN .. *Verified by:* span, which is
# ---- why §8f.10 places every amendment AFTER *Verified by:*.
reg='import re,sys
t=open("references/2026-09-08-acceptance-criteria.md",encoding="utf-8").read()
def entry(n):
    m=re.search(r"^\*\*AC-%d\*\*.*?(?=^\*\*AC-\d+\*\*|^#)"%n,t,re.S|re.M)
    return m.group(0) if m else ""
bad=[]
for n,w in [(245,"run-created"),(473,"rk-run/"),(521,"rk-run/"),(196,"Phase 2"),(197,"Phase 2"),(372,"test of `cost`"),(125,"drift gate")]:
    e=entry(n)
    if not e: bad.append("AC-%d: entry not found"%n)
    elif w not in e: bad.append("AC-%d: no dated note carrying %r"%(n,w))
e=entry(478); i=e.find("THEN"); j=e.find("*Verified by:*")
if i<0 or j<0: bad.append("AC-478: THEN or Verified-by not found")
elif "so no drift state exists to report" in e[i:j]: bad.append("AC-478: the THEN still says no drift state exists")
rows=[l for l in t.split("\n") if l.startswith("| C2 |")]
if len(rows)!=1 or "resolved" not in rows[0].lower(): bad.append("AC register section 0.3: row C2 is not marked resolved")
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
  out=$(cargo test -p "$crate" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $crate: the named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  check_sel "$crate" "$want" "$out" "$@"
}
run_sel rk-git 9 landing::run_created_file_is_staged_and_an_ignored_file_is_not landing::baseline_untracked_file_is_never_staged landing::tracked_modification_and_deletion_are_staged landing::not_due_makes_an_empty_commit_and_due_makes_one_content_commit landing::denylisted_path_is_refused_by_name_and_no_ref_moves landing::blob_at_the_limit_lands_and_one_byte_over_is_refused landing::the_trailer_tree_is_the_commits_own_tree landing::replay_serves_the_journal_and_performs_zero_writes landing::a_repository_hook_does_not_run_on_a_landing_commit
run_sel rk-store 19 render::layout_c_renders_frontmatter_stamped_sections_and_prose render::render_is_a_function_of_value_and_template_only render::render_is_byte_identical_twice_and_lf_only render::structured_edit_emits_structured_drift_then_restores render::prose_edit_is_accepted_with_no_drift_record render::one_artifact_write_per_write_carrying_the_rk_types_digest render::no_artifact_type_carries_a_drift_or_signature_field render::typed_value_reads_back_from_the_store_never_from_the_markdown render::machine_only_kind_reads_its_sidecar_and_survives_md_deletion render::ac122_a_sibling_crate_cannot_write_a_managed_path render::replay_performs_zero_writes render::move_to_prose_appends_the_drifted_bytes_from_the_causing_commit render::drift_text_never_reaches_a_deserializer render::read_appends_one_artifact_read_naming_kind_and_id render::a_read_before_the_entrys_reconcile_is_refused item_write::debt_item_is_written_under_items_with_its_provenance item_write::human_sourced_facets_are_never_written_and_are_named item_write::an_existing_item_id_is_refused_never_overwritten item_write::a_byte_identical_re_execution_is_served_and_writes_nothing
run_sel rk-kernel 5 emit::every_activity_output_appends_one_artifact_write emit::an_emit_replayed_writes_nothing_and_serves_its_recorded_digest emit::every_artifact_input_resolution_appends_one_artifact_read step_landing::a_committed_step_that_changed_the_tree_lands_before_its_commit_row step_landing::a_committed_step_that_changed_nothing_makes_no_commit
run_sel rk-adapter-claude 2 dispatch::dispatch_completed_carries_activity_id_at_version_or_null dispatch::ac360_one_event_per_termination_path
run_sel rk-cli 34 landing::fake_harness_edit_lands_on_the_run_ref_and_survives_a_second_run landing::gate_tree_hash_is_its_own_commits_tree_and_answer_does_not_void_it landing::the_user_checkout_is_unchanged_across_a_landing_lane landing::only_the_run_ref_moves_across_a_lane landing::a_refused_path_stops_the_lane_at_a_landing_gate_with_nothing_landed landing::a_kill_after_a_committed_writer_step_resumes_over_its_landed_edit landing::reentry_names_every_discarded_path landing::emitted_artifact_digest_matches_its_bytes_on_the_run_ref landing::the_lane_end_lands_the_final_stages_work landing::a_drift_gate_blocks_the_stage_exit_until_it_is_answered landing::answering_the_drift_gate_either_way_releases_the_stage_exit landing::every_artifact_read_in_a_lane_follows_its_entrys_reconcile explain::explain_prints_the_landed_commit_and_the_merge_command item_create::a_minted_debt_item_opens_an_unknown_facet_gate_when_run cost::per_activity_breakdown_from_owned_dispatch_completed_rows cost::per_activity_cost_sums_decimals_verbatim_and_absent_is_never_zero cost::no_breakdown_for_an_item_that_owns_no_dispatch_completed_row cost::a_flagged_reading_is_never_rendered_bare_and_a_bare_rendering_is_rejected status::status_prints_every_flag_beside_its_reading status::status_prints_no_headroom_line_for_an_item_with_no_reading run::no_cli_flag_skips_the_entry_reconcile doctor_journal::a_present_store_is_the_journal doctor_journal::events_jsonl_is_read_only_in_fixture_mode doctor_journal::headroom_margin_fires_on_a_flat_last_reading doctor_journal::headroom_margin_reads_the_latest_five_hour_bearing_reading doctor_journal::the_doctor_verb_never_selects_the_fixture_source doctor_artifact::an_out_of_band_structured_edit_is_reported_as_an_unrecorded_mutation doctor_artifact::a_rewrite_through_the_store_is_not_reported doctor_artifact::a_prose_edit_accepted_by_reconcile_is_not_reported doctor_artifact::an_unassigned_copy_under_redkiln_worktrees_is_never_read cost::cost_output_grammar_is_closed cost::render_empty_is_nothing_to_attribute_never_low_sample run::ac438_table_driven_exit_codes_over_fixtures_binary_arm doctor::ac415_every_rule_id_has_a_rejecting_fixture

# ---- (5) THE WHOLE SUITE, and the two lint gates. FLOOR is §8f.2 item 9's, RE-DERIVED AT STAGE OPEN from Checkpoint
# ---- 8e's recorded passed count + this stage's 64 new tests, by a dated edit of the line below before the first run.
# ---- The drafted value is 954: 740 (992d257; ADR-0164's evidence) + s8a 7 + s8b 16 + s8c 40 + s8d 46 + s8e 41 = 890,
# ---- + 64. A filtered green can select around every catching test; the whole suite cannot.
FLOOR=954
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --no-fail-fast rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|^test result:' | head -30; exit 1; }
results=$(printf '%s\n' "$out" | grep -c '^test result: ')
passed=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="passed;") s+=$(i-1)} END{print s+0}')
failed=$(printf '%s\n' "$out" | awk '/^test result: /{for(i=1;i<=NF;i++) if($i=="failed;") s+=$(i-1)} END{print s+0}')
[ "$results" -ge 44 ] || { echo "FAIL: only $results test binaries reported — 992d257 had 44; a binary vanished"; exit 1; }
[ "$failed" -eq 0 ] || { echo "FAIL: $failed tests failed under an rc of 0"; exit 1; }
[ "$passed" -ge "$FLOOR" ] || { echo "FAIL: $passed passed, below the floor of $FLOOR"; exit 1; }
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy --workspace --all-targets rc=$rc (ADR-0102 gates on the exit code)"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }

# ---- (6) TWO FLOORS, CODE LINES ONLY, beneath the behavioural tests — never the assertion. The comment filter is s7i's,
# ---- anchored to the `path:LINE:` prefix grep prints; it CANNOT see a rustfmt continuation line of a product, code
# ---- after a closing `*/`, a renamed identifier or a macro-built one, and says so rather than claiming them away.
test -d crates/rk-git/src || { echo "FAIL: crates/rk-git/src is absent — the floors below would grep nothing"; exit 1; }
raw=$(grep -rn 'GitOp::Integrate' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the no-integration floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -v '^crates/rk-git/src/git_exec\.rs:' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: GitOp::Integrate has a CODE-line caller — Phase 1 integrates nothing (design §7.7; §8f.4):"; printf '%s\n' "$code"; exit 1; }
raw=$(grep -rnwE 'f32|f64' crates/rk-cli/src/verbs/cost.rs crates/rk-cli/src/verbs/status.rs crates/rk-store/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the no-float floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: a binary float on a CODE line of cost, status or rk-store (ADR-0133 clause 5; AC-374):"; printf '%s\n' "$code"; exit 1; }

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
[ "$store_after" = "$store_before" ] || { echo "FAIL: this block moved the shipped store ($store_before -> $store_after; t2_event, t1_stage_transition, t1_gate, t1_slice_seal)"; exit 1; }

# ---- (8b) THE WORKTREE PATH SET IS UNCHANGED BY THIS BLOCK'S OWN SECTIONS, compared BEFORE the nested re-runs, since
# ---- a nested block may perform a REAL `git worktree prune` (Checkpoint 7h's guard (9) does, if 8e re-runs 7h).
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }

# ---- (9) ONLY THE ENTRY CHECKPOINT (8e) AND THE ONE CHECKPOINT THIS STAGE MAKES PROVISIONAL (7i, whose s7i GRAMMAR and
# ---- owns-no-reading branch §8f.10 amends), RE-RUN WHOLE from their own files' bytes; nothing older is re-run here.
# ---- The extractor writes BYTES (Python's text-mode stdout would turn every LF into CRLF on this box, and `bash -c`
# ---- would then read a CR at the end of every command). A block that exits 0 without its end marker did not finish.
# ---- No real `claude` is reachable at this stage: s8h has not landed and no shipped card carries permission_mode.
f8e=runbook/phase-1/s8e-tier-one-and-the-lane.md
test -f "$f8e" || { echo "FAIL: $f8e is absent — the entry checkpoint cannot be re-run (Halt 8)"; exit 1; }
blk=$(python -c "$blk_of" "$f8e" 'echo "CP8E-END-OF-BLOCK reached"'); rc=$?
[ "$rc" -eq 0 ] && [ -n "$blk" ] || { echo "FAIL: Checkpoint 8e's block could not be extracted (rc=$rc) — its end marker is not CP8E-END-OF-BLOCK, or two blocks carry it"; exit 1; }
out=$(FIX_ITEM=rk-fix-doctor-test-count bash -c "$blk" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8e, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP8E-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8e exited 0 without reaching its end marker"; printf '%s\n' "$out" | tail -10; exit 1; }
blk=$(python -c "$blk_of" runbook/phase-1/s7i-cost.md 'echo "CP7I-END-OF-BLOCK reached"'); rc=$?
[ "$rc" -eq 0 ] && [ -n "$blk" ] || { echo "FAIL: Checkpoint 7i's block could not be extracted (rc=$rc)"; exit 1; }
out=$(FIX_ITEM=rk-fix-doctor-test-count bash -c "$blk" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7i, re-run whole, exited $rc — read §8f.10's s7i row: its GRAMMAR and its owns-no-reading branch are amended at this stage"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP7I-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7i exited 0 without reaching its end marker"; printf '%s\n' "$out" | tail -10; exit 1; }

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

The records flip only on the operator's dated lines, and Checkpoint 8f (2) is red until they do. A `PROGRESS.md` row
that still reads *"a LANDED run leaves zero commits"* takes a dated correction at this close: since `s7h` a run
leaves empty commits, and after this stage it leaves content commits. The next stage is `s8g`.
