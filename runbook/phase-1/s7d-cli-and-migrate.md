# Step 7d — `rk-cli`, and `redkiln-migrate` v0 (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7d` | Phase 1 | Checkpoint 7c | Checkpoint 7d | `s7c` | `s7e` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this file was rewritten before its first run.** Until 2026-09-18 this stage was 153 lines: seven prose
paragraphs, four red-first bullets against two checkpoint filters, one acceptance paragraph claiming **AC-400–AC-439
by bare range**, and a checkpoint of twenty-eight lines in two fences. It is hardened in the `s7c` shape **before any
code is cut** (DECISIONS-2026-09-17 D17-7's precedent, carried as **DECISIONS-2026-09-18 D18-1**; `00-preamble.md`'s
maintenance rule — *"a pass is evidence about a specific text, not about a stage"* — has no pass to make provisional
here, which is the only cheap moment to do this). What the 153-line form could not catch, per checkpoint group: **the
`doctor` line** — `redkiln doctor` was required to exit 0 with output at the product root, **before** `init` had run
and in a repository with no `.redkiln/`, while AC-426 makes `unread_window_runs` *"a required key, no default"* and
AC-438 folds an error-severity rule into the configuration-error code — so the line selected for a doctor that
tolerates an absent configuration, the PIA-059 shape (C-01 **D-07**, blocking); **the window grep** — the regex
`unread[_-]?artifact[_-]?window|window_n` cannot match the key every record fixes, `unread_window_runs`, so a correct
implementation reported *"N is not declared"* unless the source carried a decoy identifier (C-01 **D-09**); **the
doctor floor** — over an empty rule table AC-414's per-fixture join, AC-415's per-id join and SM-121's dispatch count
are all vacuous, and the block's only doctor assertion was that stdout was non-empty (C-01 **RA-M-01**); **the
targets** — `init && init` ran against the product repository, leaving `.redkiln/config.toml` untracked in a tree the
stage never said it would commit (RB-M-09, absorbed by D-07); **the named tests** — the two `cargo test -p
redkiln-migrate -- <filter>` lines matched the aggregate `test result: ok. N passed` string, the substring-filter
defect every hardened block since Checkpoint 6 replaced with `^test .*\b${t}\b.* \.\.\. ok$`; **the red bullets** —
`doctor_window_n_fixtures` said the N−1/N+1 fixtures *"cannot be authored until N is declared"* while the same file's
closing paragraph declared `unread_window_runs = 50` as *"now taken"* (D-20); **the pointers** — the register's
§12.10 was cited as *"line 5132"* and the §13 phase note as *":6263"*, both moved (DECISIONS-2026-09-15 D15-2 is the
rule: id and clause, never a line); **the acceptance set** — the bare range swept in AC-428 and AC-439, whose adapter
halves 7c discharged, AC-404 and AC-408, which are kernel behaviours built at 7b/7b-ii, AC-435's human half and AC-427's
fidelity half, and omitted AC-531's proper place, AC-411 (claimed by no stage), AC-263 (handed here by `s7b-ii`), the
`doctor` halves `s7c` ARMED here, and every record the C-01 pass found undecided (D-22). Every one of those is named
below in the partition it belongs to. **The records this rewrite executes** are `spikes/DECISIONS-2026-09-18.md` —
the decisions **D18-1…D18-19** — D18-1…D18-13 the plan recorded, D18-14…D18-16 the three the hardening needed
decided, D18-17 the four blocking C-01 items that were the operator's (D-01, D-02, D-04, D-14) with D-05/D-15/D-18, D18-18
the amendments' landing, D18-19 (2026-09-19) the six `s7b-ii` gate `doctor` rows allocated here; three blocking items were this file's (D-07 → D18-9, RA-M-02 → D18-15, RB-M-01 → the
poisoned-lock entry under D18-4) — and the routings and open items numbered on from DECISIONS-2026-09-17 (**RT-28+**,
**O-12+**, **R-25+**; this file's own routings are **RT-41…RT-59** — RT-41…RT-46 from the first draft, RT-47…RT-59 added by
the hardening rounds — and its open item **O-16**, added to that log's tables in the same commit) —
**ADR-0128** (the one `run` grammar) and **ADR-0129** (the `answer` grammar and the question registry), both committed
`proposed` at stage-open, and the **C-01 adjudication** recorded in `PROGRESS.md` §6 and reproduced in §7d.2. The
plan file, the finder/refuter/adjudicator outputs and the operator's answers are working notes in the session
scratchpad, not in the tree, and nothing below cites them as a record.

---

> ### 🚦 GATE CHECK
> ```bash
> ( grep -qE '^\| 7c — The adapters \|.*checkpoint passed `2791f1d`' D:/repos/redkiln-rs/runbook/PROGRESS.md \
>     || { echo "GATE OPEN — PROGRESS.md row 7c does not read 'passed 2791f1d'. Go back to Step 7c; no work in this stage may start."; exit 1; }
>   echo "GATE DISCHARGED — proceed" )   # a subshell: a pasted FAIL must not kill an interactive shell
> ```
> **One golden constraint reaches this stage by name and three more by what it builds** (`00-preamble.md`'s *"Where
> each constraint actually bites"* table names Step 7d on row 12; rows 2, 3 and 4 reach it through the verbs it builds). **Constraint 2 — nothing may
> depend on a process surviving:** `run` at a gate is a `Gate:` trailer, a `gate_opened` row and an exit 0 (§7d.5);
> `run` at a rate limit is a durable mark and an exit (AC-404's exit half); the signal path fires a token and the
> process exits through the closed table (§7d.10) — no verb waits on a channel, prompt, socket or sleep, and the
> checkpoint's one bounded wait is the `$(cargo test …)` capture, bounded by each process-spawning test's own 30 s
> wall-clock bound (§7d.14; Halt 5's alternative) — libtest itself bounds nothing. **Constraint 3 — `cargo build`
> first:** the block's first gate is `cargo build --workspace`, and every `./target/debug/redkiln` line below it runs
> the binary that build produced. **Constraint 4 — a rig that compiles has not been run:** six verbs are run as
> processes against a temporary repository the block mints itself (`init` twice, `doctor` twice, `run` refused two
> ways, `explain` at `5`, `status` at `0`, `catalog search` at `0`) and
> the migrator's dry run is invoked as a process against a second one, so a defect a unit test cannot reach — a
> `clap` tree that parses `--help` and refuses a real item, an `init` whose second run rewrites a byte — has one line
> that can fail on it. **Constraint 12 — test-first, one discriminating test per criterion:** §7d.14 carries one
> red-first bullet per checkpoint test name; the checkpoint is the floor, not the evidence.
>
> **This checkpoint has NO live dispatch line, and so needs no shell guard against a Claude-spawned Git Bash.** Every
> `Harness` the block reaches is a fake or the `NoDispatchHarness` (§7d.8); `doctor`'s one child process is
> `claude --version` (a spawn, never a dispatch — D-11); the block therefore runs from any Git Bash on this box,
> including one Claude Code opened. What it does refuse is a shell carrying a name on the committed environment
> denylist — `ANTHROPIC_FOUNDRY_API_KEY` is persisted at user scope on this box (`s7c` §7c.21) and `doctor`'s
> `RK-D-ENV-DENYLIST` row would correctly fire on it, turning the temp-repo `doctor` line's expected 0 into a 3 for a
> reason unrelated to the binary — so the block's first guards refuse that shell and name `env -u`.
>
> **And the standing prohibition on what this stage's close-out may say.** **ADR-0101 clause 5**: *"Until it runs,
> no document may claim the review gate is guarded against false approval, and no FP rate from S-VAR or S-TYPED is
> evidence of overall calibration."* Nothing in this stage runs S-SUBTLE-CB; Checkpoint 7d's pass says **the CLI
> exists, the gate is a trailer and an exit, doctor is a rule table, and migrate v0 ran a dry run** — not in
> `PROGRESS.md`, not in a commit message, not in a milestone row may it be written up as more. `INDEX.md`'s *"M4 —
> the system runs"* needs Checkpoint 7e after this one, and this file attributes nothing to it.

### 7d.1 What this step builds, and what it may not decide

**Two crates: `rk-cli` (binary `redkiln`) and `redkiln-migrate` (binary `redkiln-migrate`), and one closed crate edited
— `rk-kernel` (eight edits, §7d.15; no other closed crate is touched — the console-signal test-support wrapper an earlier
draft put in `rk-containment` is **not built**, §7d.10).** ***Amended 2026-09-21, Step 7d Slice 4, on the operator's
decision of that date: a SECOND closed crate is edited — `rk-adapter-claude`, for the CLI pin and nothing else.***
*§7d.15 carries the amendment, what it permits, what it does not, and what it costs; this sentence is the summary
and loses to it, the same precedence that settled the fixture count. The edit is the bump RAISED on 2026-09-19 and
left OWED (PROGRESS.md §3), which Checkpoint 7d's Halt 2 blocks the stage close on. Note also, and separately, that
`rk-kernel`'s own count reads **nine** against this sentence's eight — Slice 3 took five where four were enumerated
(R-59, mechanism B) — which was RAISED on 2026-09-20 at `0e73835` and is not re-counted here.* `rk-cli` is the **composition root** design §9.1 names: a `clap`
router over the kernel with `run <item> [--lane fix|story]` (ADR-0128), `answer <item> <option> [--attach
<kind>=<path>] --explain-sufficed yes|no --toml-opened yes|no` (ADR-0129 decision 1 — two required flags, one record field; §7d.6), `status`, `explain <item>`, `doctor` and `doctor --rebuild`,
`catalog search <query>`, `init` — the verbs design §8.1 lists at Phase 1 — plus `lint --facet-stability|--facet-provenance
<facets.toml>`, which **no §8.1 row carries** and which `rk-kernel`'s `lib.rs` header and `s8` (6) name as `rk-cli`'s,
wrapping **two of the three** kernel lints that header says are owed a CLI surface (*"`redkiln lint --facet-stability`,
`--facet-provenance` and the registry lint are `rk-cli`'s (Step 7d)"*) — the registry lint's surface is **not built at
7d** and is RAISED in §7d.18 (no §12 criterion names a verb for it; `s8` (6) invokes only the two facet lints), and the
missing design §8.1 `lint` row is RAISED beside it. It constructs the `CancelToken` and owns the
signal path (ADR-0099 decision 2; **D18-2**: `ctrlc = "3"` with its `termination` feature, §7d.10), reads the process
environment in exactly one place while building `Config` (ADR-0083: *"The process may read environment variables once, at the edge, while building
`Config`"*), resolves the four real adapters once at startup (AC-485/AC-486; ADR-0094 decision 1) and owns the exit
table (AC-438). `redkiln-migrate` is *"a separate binary, not a verb"* (design §8.1's last row; AC-441) and the only
workspace member that may depend on a YAML crate (ADR-0100 decision 1). The `rk-kernel` edits are **eight** and each is
named in §7d.15 with the checkpoint rows it re-opens: (1) `pub struct Config` in `crates/rk-kernel/src/config.rs`
(ADR-0083's *"a `Config` struct constructed at the composition root and passed in"* — C-01 RB-M-05 found no crate
declared the type; **D18-14**); (2) a `keywords: Vec<String>` field on the catalog loader's **`Activity`** (AC-436's *"an
activity's engine-ignored `keywords` field, which `redkiln catalog search` searches"* — RA-M-02 found the loader dropped
it; **D18-15**; `crates/rk-kernel/src/catalog.rs` has `pub struct Activity` beside `pub struct Step` (an activity's
`[[step]]`), and all 102 shipped activity files under `spikes/p9/catalog/` already carry `keywords = [...]` [measured
2026-09-18: `grep -l '^keywords' spikes/p9/catalog/*.toml | wc -l` → 102], which the loader drops) with
`catalog::search`; (3) the ADR-0063 question registry type with the default question built in (ADR-0129 decision 3);
(4) `catalog::load_diagnostic` — the pack loader's diagnostic mode that returns every load error typed instead of the
first, which `doctor` alone calls (§7d.8) (**RT-54** — routed, not decided: a second loader entry point in a Step-7b
crate; the alternative — `doctor` calling the strict loader per file and collecting first errors — is named and
rejected because a pack with two defects would surface one; the record naming the diagnostic mode as the loader's
surface is **SETTLED 2026-09-20 at Step 7d Slice 4 cycle 1**, by a dated amendment note on `design` §9.1's
`rk-kernel` row — `catalog::load_diagnostic` and `catalog::load_diagnostic_dir` returning `CatalogLoad`, every defect
typed as a `CatalogDiagnostic { error: CatalogError, kind: DiagnosticKind }`; `CatalogError` stays at three variants
and `GateOutputs` is a `DiagnosticKind`, not a fourth one. The note says why it is a note and not a supersession
(ADR-0092): no Decision sentence fixes the loader's entry-point count, and ADR-0045 is enforced by edit (7) rather
than withdrawn. **The rejected alternative is why the accumulation is INSIDE the activity walk**: two defects in two
files would each surface under per-file first errors, so the sentence above is only true of two defects in ONE
activity file, which is the shape §7d.14's red names); (5) `workflow/gate.rs` — `gate::answer` gains **three parameters** — `registry: &QuestionRegistry`
and `attachments: &[Attachment]`, which it validates against before it closes the mark (ADR-0063's *"the kernel refuses
to close the mark unless that attachment is supplied"*), and `meta: AnswerMeta { explain_sufficiency, answerer }` whose
members `gate_answered_event` writes kernel-side (§7d.6, §7d.15; the ~20 call sites in `crates/rk-kernel/tests/it/gates.rs`
each gain the three arguments and `GateError` gains `OptionRefused(OptionError)`); (6) `replay::resume` — the store-seeded
twin of `replay::record` that folds the item's recorded journal before `next_action`, steps `NextAction::OpenGate` through
`Runtime::step` so the `gate_opened` row is written, and takes the per-node **spec supplier** `record()`'s loop lacks a
seam for (§7d.5 step (9): `record()` folds a fresh local vector and **breaks** on `OpenGate` without appending it, and its
loop calls `inputs.spec_for(step_id)?`, which refuses any dispatched node with no `with_spec` binding
[measured 2026-09-18, `crates/rk-kernel/src/replay.rs`], so a second `run` over a stored answer would re-derive the gate
at reach 1 — AC-264, AC-186 — and a first `run` would refuse on `inputs/specs.json` before any dispatch; **RT-55**); (7) `Catalog::from_sources` gains the refusal of a `Gate` step carrying `outputs`
(AC-409's WHEN: *"when the configuration is loaded"* — today the strict loader parses `outputs =` for every `StepKind`
[measured 2026-09-18, `catalog.rs`], so `run` would load the graph AC-409 says is rejected); (8) `pub const LOCK_HALTS`
in `runtime.rs` — a visibility change only, so `rk-cli`'s panic hook matches the kernel's text instead of a second copy
of it (§7d.4).

**The records that govern**, read before their sections: **ADR-0107** for the one idempotent `run` entry, the
denylist, the snapshot and the idempotent `init` — with **ADR-0128** superseding its decision 4's grammar sentence in
part (`accepted` 2026-09-19; §7d.3, §7d.5); **ADR-0009** for the CLI-only gate channel — with **ADR-0129** superseding its
Decision sentence *as a grammar* in part (`accepted` 2026-09-19; §7d.6); **ADR-0017** for the `(token, tree_hash, deadline)`
record; **ADR-0063** for options that `attaches = <Kind>` and the human-supplied root; **ADR-0099** decision 2 for the
token and its two owners; **ADR-0102** decision 6 for `anyhow` confined to these two binaries; **ADR-0105** clauses 3
(trailer plumbing), 4 (`tree_hash`), 6 (worktrees re-created never reused), 7 (the store's `doctor` rows) and 8
(`status` does not reconcile); **ADR-0083** for configuration as a value and the one environment read; **ADR-0118**
clauses 5 and 6 for the poisoned-lock halt and what 7d owes on it; **ADR-0074** (supersedes ADR-0004) for principle 4
— absence of evidence is a block — behind AC-438/AC-439; **ADR-0094** for the four reals; **ADR-0100** for the
dependency policy every new row obeys; **ADR-0092** for what this file may and may not decide; **ADR-0109** for
reconcile at every action-causing entry; **ADR-0116** for the route by which an event kind is minted (by record or
not at all — §7d.8 on SM-123); **ADR-0002/0013** for the clean break and the pinned corpus behind `redkiln-migrate`;
**ADR-0075** clause 6 for the opaque-prose treatment the eight Phase-2 companion kinds take (the list is AC-143's and
DECISIONS-2026-09-11 #3's, spelled by `rk_types::companion::PHASE_2_KINDS`); **ADR-0091** decision 1 for who owns the
common-dir read (H-3 departs from it on 7c's shipped path and says so). Read each before its section.

**What this step may not decide** (`00-preamble.md`: *"Where a stage is tempted to decide something else, it
stops."*):

- **Nothing about the crate layout beyond design §9.1.** Module file names inside `rk-cli` and `redkiln-migrate` are
  the implementer's (as `s7b` §7b.4 held for the kernel and `s7c` §7c.1 for the adapters); the routing table below
  names files so the checkpoint can name them, and that is a table, not a record. Where the checkpoint must reach
  into a module it does so by module-path filter (`s7c` §7c.21's `rk_absent` fourth argument), never by exact path.
- **Nothing that lands a type in a closed crate without disposing of the pass it disturbs.** `rk-kernel` gains
  `config.rs`, a loader field, a registry type, a diagnostic loader mode, three parameters on `gate::answer`, a store-seeded
  `replay::resume`, the strict loader's AC-409 refusal and one `pub` visibility (the eight edits above); each slice that
  edits it marks `PROGRESS.md` §1's rows 7b and 7b-ii
  `passed <hash>, re-run pending 7d rk-kernel` **in the same commit** and re-runs both checkpoints whole there
  (§7d.15; `00-preamble.md`: *"A checkpoint that changed after it passed is provisional"*; precedent `s7c` §7c.19).
  The root `Cargo.toml` rows land at Slice 1 and make Checkpoint 5 provisional the same way.
- **Nothing about where a criterion is allocated.** Where an id is orphaned, double-claimed or mis-routed — AC-411,
  AC-455's and AC-530's `doctor` rows (settled by the operator as D18-19 on 2026-09-19, not by this file), the `s7b-ii` block armed to 7c that names the composition root, SM-123's
  instrument — this file **raises it and does not settle it** (`s7a` §7a.11: *"naming a destination is not routing
  it there"*). §7d.18 names each one.
- **Nothing a register THEN or an ADR Decision sentence fixes.** Four of the seven blocking C-01 items (D-01, D-02,
  D-04, D-14) were decided by the operator on 2026-09-18 **before this file was written** (DECISIONS-2026-09-18 D18-17);
  the other three (D-07, RA-M-02, RB-M-01) were execution-order and instrument questions this file takes under ADR-0092
  clause 2.5 (D18-9, D18-15, the poisoned-lock entry under D18-4), and each is recorded in §7d.2 with its record; this
  file executes them and re-opens none.

**Where this stage DOES decide, it says so here and names the record** (ADR-0092 clause 2.5: *"where it must
decide, it says so in its header table and names the record"*). Five rows, each a decision of execution order or of
a surface ADR-0092 assigns to the runbook, none a threshold or an instrument:

| # | This stage decides | Record | Where |
| --- | --- | --- | --- |
| H-1 | **The exit-code table's values** — `0` Success (also gate-open, by D-01; also the AC-450-style no-op; also a `Void` answer, recorded as void — AC-406), `2` Usage, `3` ConfigError, `4` FindingsBlock, `5` NoRecord, `10`–`20` one per `StepError` variant in `error.rs` order, `70` PoisonedLockHalt, `130` SignalOutsideDispatch | **AC-438** says *"a closed, snapshot-pinned table"* and *"a committed snapshot of the exit-code table"* — the committed snapshot IS the record, so no ADR is minted (**D18-4**; the plan's first form of D18-4 named an ADR-0128 for the table and was withdrawn when ADR-0128 was needed for the `run` grammar) | §7d.4 |
| H-2 | **The non-`StepError` rows the closed table must carry** — usage (clap's default `2`, kept: clap's own refusals, an undeclared option, an attachment no declared option admits or a declared attachment absent — AC-410, ADR-0129 decision 4), configuration error (AC-413/AC-426/AC-486/AC-531 refusals, the error-severity `doctor` rule, ADR-0128's non-derivable lane and its given-vs-derived disagreement (decision 2 as clarified 2026-09-19), D18-8's no-pack repository, unknown keys, the git floor, and an `answer` with no resolvable live token (AC-248) or a spent token (AC-407) — refusals of the repository's recorded state, read as configuration of the item), no-record (AC-432), findings-block, the ADR-0118 halt (clause 6: *"the CLI's surface, owed at Step 7d"*), signal outside a dispatch (RA-M-04) | AC-438's *"for any reason"* (C-01 **D-12**) | §7d.4 |
| H-3 | **The common-dir discovery rule** — every verb discovers the repository from the **inherited cwd through the `rk-git` wrapper**, in this order: a **bootstrap** `GitExec` whose `hooks_empty` is a `tempfile::TempDir` under the OS temp directory (`GitExec::new` creates that directory at construction [measured 2026-09-18, `crates/rk-git/src/git_exec.rs`], and the working hooks directory lives under a common dir not yet known, so the first wrapper cannot be the working one); through it `GitExec::run(GitOp::RevParse, &WorktreePath::new(Path::new(cwd_arg))?, ["rev-parse", "--show-toplevel"])` for the working-tree root that holds `.redkiln/`, `cwd_arg` being `-C <path>` or the literal `"."` — the child git resolves `"."` against the inherited cwd; then `rk_adapter_sqlite::path::common_dir(&bootstrap, &WorktreePath::new(Path::new(&toplevel))?)` — the `pub` reader 7c shipped — for the store's common dir; then the **working** `GitExec` (`hooks_empty = <git-common-dir>/redkiln/hooks-empty/`) that every later op goes through. **The `"."` is used for the one `--show-toplevel` read ONLY**: the `WorktreePath` every later op receives — `SqliteStore::open`, `reconcile_at_entry`, `tree_hash`, `commit_with_trailers` — is over the **absolute** `--show-toplevel` answer, so the identity row's `common_dir` is one absolute string whatever directory the verb was invoked from (§7d.3); never `std::env::current_dir`; `-C <path>` is optional | C-01 **D-18**; AC-529's ban is about tier-1 writes and is untouched — `ci.yml` carries the scope comment (landed 2026-09-18). **ADR-0091** decision 1 says *"gix owns the common-dir read … the wrapper's `rev-parse --git-common-dir` fallback is not commissioned"*; this stage reuses the executable read 7c shipped in `rk-adapter-sqlite/src/path.rs::common_dir` (Checkpoint 7c passed `2791f1d` on it) and adds no second reader — `rk-cli` never issues its own `rev-parse --git-common-dir`, and the one new read this stage adds is `--show-toplevel`, which ADR-0091 does not speak to; `redkiln-migrate`, which may not depend on `rk-adapter-sqlite` (AC-466), issues the one `rev-parse --git-common-dir` its worktree path needs through its own bootstrap wrapper and is named in the RAISED item — the ADR-0091-vs-tree divergence is RAISED in §7d.18 for an erratum or a dated note, not settled here | §7d.3, §7d.11 |
| H-4 | **Where N lives** — `[doctor] unread_window_runs` is the per-repository key (AC-426: *"a required key, no default"*), read by the one rule row `RK-D-UNREAD` (AC-472 as amended 2026-09-18); `init` writes `INIT_UNREAD_WINDOW_RUNS = 50` as **one** literal | **D18-5**; the value is DECISIONS-2026-09-12 **E9**'s and the preamble's decision 4 | §7d.8, §7d.9 |
| H-5 | **The `Gate:` trailer commit is a tier-1 write and goes to the run-owned worktree, never to `cwd`** — `run` at a gate and `answer` on `Applied` call `rk_git::worktree::authorise_tier1_write(exec, &owned_path, &ownership)` and commit at the `OwnedWorktree` it returns, where `ownership: RunOwnership { worktree, branch }` is the pair the store records for the item (**RT-47** — routed, not decided: at 7d `run` re-creates the worktree under `<common-dir>/redkiln/worktrees/<item>/` on branch `rk-run/<item>` — both `rk-` names, AC-531 — through `GitExec::run(GitOp::WorktreeAdd, …, ["worktree", "add", "-b", &branch, path, "HEAD"])` on a first run and `rk_git::worktree::add` when the branch exists (the migrator's shape, §7d.11 step (3) — a previous run's tree is torn down first by `GitExec::run(GitOp::WorktreeRemove, …, ["worktree", "remove", "--force", path])`, the raw-`run` form step (3) already uses, because `rk_git::worktree::remove` passes no `--force` and git refuses a tree holding a killed run's uncommitted files [measured 2026-09-18, rc=128: *"contains modified or untracked files, use --force"*]; the `--force` is licensed only because the tree is run-owned, re-created never reused — ADR-0105 clause 6 — and never the user's checkout), and records the pair on the invocation's `reconcile_started` row beside RT-41's members; the record naming the ownership carrier is OWED, §7d.18 RAISED); `tree_hash` at gate open and at answer is taken over that tree, and the reals' `project_root` is that path | **ADR-0105 clause 8** (accepted Decision sentence): *"A tier-1 write resolves the run-owned worktree **from the store**, regardless of cwd, and a target that is not run-owned is refused naming its path and branch"*; **AC-529**'s THEN: *"the run-owned worktree is resolved **from the store**, never from cwd"*; **AC-518**'s THEN: an orchestrator commit from the user's checkout *"is refused unless its target is a run-owned worktree"*; design §3: *"The orchestrator never writes to the user's checkout"*. The hash basis follows **ADR-0017** — *"the tree hash the human saw"* — and the human reviews the run's output where the reals' `project_root` put it, the run-owned worktree; AC-406's void is therefore a move of THAT tree. The guard already ships — `rk_git::worktree::authorise_tier1_write(exec, target, &RunOwnership { worktree, branch }) -> OwnedWorktree`, `RunOwnership`, `OwnedWorktree` [measured 2026-09-18, `crates/rk-git/src/worktree.rs`] — and this row executes it; what it decides is only the 7d carrier of the pair *(dated 2026-09-27, `s8d-ii` build, P6, under ADR-0180, proposed *(dated 2026-09-28, `s8e` close: accepted on D21-91, at `s8d-ii`'s close)*: from `s8d-ii` the run-owned worktree is `.redkiln/worktrees/<item>` under the main worktree, derived by `rk_git::worktree::derive` (D21-44), and it is entered, not re-created: reused on re-entry after an explicit discard and never force-removed (D21-45 W2). The teardown described in the first column, and its `--force` licence, are retired with `recreate_run_worktree`; the tier-1 rule this row executes stands)* | §7d.5, §7d.6 |

**The routing table.** Each row below was **routed, not decided** — the operator chose a working shape so the build
can proceed, and each open row owes a record. Every row carries its DECISIONS-2026-09-18 id (`D18-n` decided, `RT-n`
routed, `O-n` open) or names the source that has none. **The checkpoint asserts the DECIDED rows** (the `Exit` snapshot
exists, the `DENYLIST` has eleven names, `INIT_UNREAD_WINDOW_RUNS` is one literal, ADR-0128 and ADR-0129 are
`accepted` at the close) **and polices routed rows only as existence and shape floors that move with the record** —
named here so no reader takes a green line as a decision: RT-41's `reconcile_started` payload (a test reads the first
row; the field name is a floor), RT-42's `answerer` source (`ac435_…` asserts the field equals the answer commit's `%an <%ae>`; the read precedes the `gate::answer` call — §7d.6), RT-31's
two flag names `--explain-sufficed` / `--toml-opened` (the snapshot pins the shipped spellings; ADR-0129 decision 1 makes them renameable),
RT-43's `Entry::Answer`-then-`tree_hash` order (a test, not a record), RT-45's three ADR-0105 rows (asserted by
`adr0105_doctor_rows_network_fs_credential_orphan` and `ac415_…`'s join, outside the register-named `FLOOR` — the block
has one floor list and they are not on it). The log's own routings RT-28…RT-40 are carried in the same table, each tagged on the row that
executes it (RT-40 is withdrawn there by a dated line — no composite is built, §7d.12). A record that overturns one of
those routings makes its floor line **stale, not wrong**: the line is re-pointed in the same commit as the record's code, the way Checkpoint 5's
`ChangeSummary` line was (D17-1), and the block never reads a routed value as decided.

| Type | Home | Warrant | Status |
| --- | --- | --- | --- |
| `Exit` (the closed exit enum, `Exit::ALL`, `impl From<&StepError>`) | `rk-cli` (`crates/rk-cli/src/exit.rs`) | AC-438's subject tags are `rk-cli`, `rk-types`; the table is the binary's surface and a `StepError` → code map belongs beside the process that exits; `rk-types` stays a library with no exit vocabulary | **DECIDED by D18-4 as a header-table row (H-1/H-2)**; the committed snapshot is the record (values); the enum's **home** in `rk-cli` rather than `rk-types` is DECISIONS **RT-35**, which owes a dated note on AC-438's crate tag |
| `Config` (the value ADR-0083 names) | **`rk-kernel`** (`crates/rk-kernel/src/config.rs`) — a Step 7d edit to a closed crate | ADR-0083's Decision: *"Configuration reaches the kernel as a value: a `Config` struct constructed at the composition root and passed in"* — the kernel must accept it, so the kernel declares it (C-01 **RB-M-05**); design §9.1's `rk-cli` row carries the dated note (landed 2026-09-18) | **DECIDED by D18-14** (operator, 2026-09-18; alternatives named there: `rk-types`, an `rk-cli`-private struct); constructed only at `rk-cli`'s root, threaded into `RunInputs::with_config` as an `Option<Config>` that is **omitted from `to_json()` when `None`** (so every recorded `run_id` / `inputs_digest` on the tree is byte-stable and the 7b/7b-ii re-run is the proof), its digest recorded on the CLI-written `reconcile_started` row (RT-41) — **no kernel event payload changes** |
| `ConfigFile` (`.redkiln/config.toml`'s closed struct: `adapters { harness, store, toolchains, harness_binary?, toolchain_programs? }`, `packs { enabled }`, `doctor { unread_window_runs: u32 }`) | `rk-cli` (`crates/rk-cli/src/config.rs`), `serde` + `#[serde(deny_unknown_fields)]`, read by `toml = "1"` | design §9.2 names `toml`; AC-474's *"config-surface snapshot test over the full key list"* wants a closed struct so the key list is enumerable; **D18-3** (as corrected 2026-09-18: the log spells this struct `ConfigFile`, so the name `Config` is free for the kernel value D18-14 declares — one rename, stated here); **RT-34** fixes its field set — `captured_env` is **not** a field: the captured map is an `rk-cli` value (`Env`) handed to `AdapterSet::resolve` beside the file | **DECIDED by D18-3**; the `toml_subset` hand-walk was the named alternative. The two **optional** `[adapters]` keys are **RT-49** — routed, not decided: `harness_binary` (default `"claude"` — the adapter's own default) and `toolchain_programs = { node = "node", rust = "cargo" }` (defaults resolved on the child's `PATH`) are the one way a process-level test reaches `rk-fixture-child-claude` without the env toggle AC-474 forbids (§7d.14); the alternative — a `[dev]` table, or a compile-time `cfg(test)` path — is named and rejected because the shipped binary must be the one under test; both are on the AC-474 key snapshot, neither is in the scaffold, neither is a write-path toggle, and **a design §9.2 dated note naming the `[adapters]` key set is OWED** (§7d.18 RAISED) |
| The scaffold `init` writes | `crates/rk-cli/src/verbs/init.rs`, the bytes fixed verbatim in §7d.9 | AC-437's Verified-by as amended 2026-09-18 (C-01 **D-14**): *"`init` is EAGER and its scaffold is fixed verbatim"* — the four compiled-in reals, `[packs] enabled = []`, `[doctor] unread_window_runs = 50`, the store created through `SqliteStore::open` | **DECIDED by D18-17 (D-14)** (operator, 2026-09-18; the E6-owed record; rejected: lazy creation with an empty `[adapters]`) |
| `AdapterSet` (`resolve(&ConfigFile, &Env) -> Result<Resolved, Refusal>`; the union of the four `registry.rs` capability vocabularies) | `rk-cli` (`crates/rk-cli/src/adapters.rs`) | AC-485/AC-486: resolved *"at startup"*, a run-scoped constant; the four per-crate `enum Capability` bodies are byte-equal (`s7c` §7c.1 RT-24) and `rk-cli` is the one place that reads all four | Built here; **RT-24's record naming the vocabulary's home is still OWED** (7c's row; this file does not re-raise it under a new id) |
| The `reconcile_started` payload's `adapter_set` and `config_digest` members | the first row of every action-causing invocation (`rk-kernel`'s `fsm.rs` comment: *"`reconcile_started` is `rk-git`'s"*; the CLI is its writer — `reconcile_at_entry` writes nothing) | AC-485's THEN — *"the adapter set recorded on the run-start event"* — and design P3 name a **run-start event no §5 row carries**: `EventKind::ALL` is 38 and has no `run_started` [measured 2026-09-18, `crates/rk-types/src/event_kind.rs`]; ADR-0116 permits minting a kind by record only, and design §5's `reconcile_started` row is *"invocation id, HEAD, the drift set that followed"* | **RT-41** — routed onto the existing per-invocation row as two payload members; **a record is OWED**: either a dated design §5 note widening the row's payload, or a minted `run_started` kind by ADR-0116's route. `ac485_…` reads the first row and is the floor |
| `doctor`'s `Rule` / `RULES` (`&[Rule { id, severity, check }]`, ids `RK-D-*`) | `rk-cli` (`crates/rk-cli/src/doctor/rules.rs`) — data, not a match | AC-415: *"the `doctor` rule table, which is data with a stable id per rule"*; ADR-0099 clause 5: `rk-doctor` → `rk-cli` | Built here; the id set is §7d.8's table, and the checkpoint's floor is the register-named subset (**RA-M-01**) |
| `NoDispatchHarness` (a `Harness` whose `dispatch` panics naming the rule that reached it) | `rk-cli` (`crates/rk-cli/src/no_dispatch.rs`) — a `src/` type, because `rk_ports::root::Ports` has five mandatory `Arc<dyn …>` fields [measured 2026-09-18, `crates/rk-ports/src/root.rs`] and the register's *"`doctor`'s `Ports.harness` is a fake"* (AC-414 as amended) needs one the binary can hold; **constructed at the composition root**, never inside the `doctor` module, and handed in on the `Ports` the root builds for the `doctor` path (§7d.8) | AC-414's Verified-by as amended 2026-09-18 (C-01 **D-10**): *"'does not link a live adapter' means 'does not construct a `Harness` implementor'"*; **D18-7**: `doctor` never constructs a real `Harness` | **DECIDED by D18-7**; the lint half is a source scan over the `doctor` module for the four adapters' constructor paths (§7d.8) |
| `QuestionRegistry` (question id → options → per-option `attaches` kinds; the default question `approve` / `changes-requested` built in) | **`rk-kernel`** (a Step 7d edit; `crates/rk-kernel/src/workflow/question.rs` is the implementer's name) | ADR-0129 decision 3: *"A `pub` registry type lands in `rk-kernel` at Step 7d … with the default question built in … Pack-declared questions … are Step 7e's"*; ADR-0063's Decision | **DECIDED by ADR-0129 (proposed)**; the file and type name are DECISIONS **RT-29**'s; pack-declared questions are 7e's — at 7d a non-default question exists **in-process only** (a test-constructed registry, §7d.6) |
| The `explain_sufficiency` field and its two flags | one payload member on `gate_answered` — `explain_sufficiency { sufficed, toml_opened }` (design §5's `gate_answered` row as amended 2026-09-18 and corrected 2026-09-19), written **kernel-side** through `AnswerMeta` (§7d.6); the flags `--explain-sufficed yes\|no` and `--toml-opened yes\|no`, both required | ADR-0129 decision 1 as accepted 2026-09-19: two required flags, one record field — the register's THEN *"the answer (yes/no, plus whether a TOML was opened) is recorded as a required field"* is two data in one field, and SM-126's kill (*"opened a TOML anyway"*) is the cell *sufficed = yes AND opened = yes*, which the proposed record's one-flag reading could not represent; the flags' names are *"the default the implementer may rename at Step 7d; the stage file names the ones shipped"* | **RT-31** — the shipped names are `--explain-sufficed` / `--toml-opened` (this file names them; the snapshot pins them); **the AC-435 Verified-by dated note naming the shipped flags and the field is OWED at the close commit** (the register carries no 2026-09-18 note under AC-435 [measured: `git diff` of the register has no hunk there]) |
| `answerer` on `gate_answered` | the **git author identity the wrapper stamped on the run-owned branch's `HEAD`**, read through the wrapper **before** the `gate::answer` call — `GitExec::run(GitOp::Log, owned.path(), ["log", "-1", "--format=%an <%ae>"])` over `rk-run/<item>`, the author of the gate-open commit `run` made, which is the configured identity the answer commit will carry — so the kernel can write it on the `gate_answered` row it builds; asserted equal to the answer commit's `%an <%ae>` after it (RT-42 as corrected 2026-09-18) | design §5's `gate_answered` row as amended 2026-09-18: *"the intended source is the git author identity read through the `rk-git` wrapper at `answer` time — the implementer confirms this at Step 7d or the field is dropped"*; `GitOp` is a closed enum with no `Config`/`Var` read, and `Log` is on it | **RT-42** — routed to the run-owned branch's author identity (one op, already on the enum, no `rk-git` edit); **confirmed by the operator 2026-09-19 (R-26 closed; design §5's row carries the dated confirmation)** |
| `keywords: Vec<String>` on the loader's `Activity` (AC-436: *"an activity's engine-ignored `keywords` field"*; `catalog.rs` has `pub struct Activity` at one site and `pub struct Step` — an activity's `[[step]]` — at another) | **`rk-kernel`** (`crates/rk-kernel/src/catalog.rs`, a Step 7d edit) and `rk_kernel::catalog::search(&Catalog, &str)` | AC-436's GIVEN names the field and the verb; the 7b loader has no such field [measured 2026-09-18: `grep -n keywords crates/rk-kernel/src/catalog.rs` → 0] so the property test passed by construction (C-01 **RA-M-02**) | **DECIDED by D18-15** (operator, 2026-09-18); the unreachability lint is a source scan over `crates/rk-kernel/src/select/` and `predicate/` for the identifier |
| The `--lane` derivation source | the item record's kind, when the record makes it derivable; else the flag; else a refusal (exit 3) | ADR-0128 decision 2 | **DECIDED by ADR-0128 (proposed)**; the in-kernel carrier is `rk_kernel::select::item::Item::kind()` (`Item::new(id, kind)` — the `kind` facet, already a kernel type [measured 2026-09-18, `crates/rk-kernel/src/select/item.rs`]); *where* `run <item>` reads the item record from in a repository is routed here as `.redkiln/items/<item>.toml` (**RT-50** — an existence floor the fixtures follow; 7e's pack schema record re-points it; added to DECISIONS-2026-09-18's routed table) — 7d's fixtures carry a top-level `kind` key, a top-level **`stage`** key (the item's current stage — the `stage: &str` every kernel entry takes: `RunInputs::new(stage)`, `fsm::fold(item, stage, ..)`, `fsm::stage_action(item, stage, ..)`, `RunState::initial(item, stage)` [measured 2026-09-18, `crates/rk-kernel/src/replay.rs`, `workflow/fsm.rs`]; `stage_transition_event(from, to)` carries no `item` and `Pack::stage_specs()` is a set with no first, so no fold or pack read yields it — read once per invocation by `run`, `status` and `explain` alike, an absent key exit `3` naming the item — *"no stage on the item record"* — and the fixtures spell `stage = "implement"` or the `pack-min/` stage name) and a `[facets]` table (§7d.5 step (7)) and the test pins the refusal, not the keys' spelling |
| `redkiln-migrate`'s `Report` and `schema/report.schema.json` | `redkiln-migrate` (`crates/redkiln-migrate/src/report.rs`, `crates/redkiln-migrate/schema/report.schema.json`) | AC-448: *"per-(kind, slot) counts of lifted / lifted-with-loss / failed and the corpus SHA"*; AC-467: *"both the corpus SHA and the lifter/tool version"*; AC-468: *"three required integer fields, no `total`-only form"* | Built here (DECISIONS **RT-28**: the home and the shape; a record fixing the schema as the migration's contract is OWED there); the schema is committed and validated by `jsonschema` in-process |
| The legacy readers (`frontmatter.rs`, `ledger.rs`, `review.rs`, `config_yaml.rs`) | `redkiln-migrate` (`crates/redkiln-migrate/src/legacy/`) — the only `yaml_serde` users and the only site of the literals `template_sig` / `rendered_sig` | AC-442: *"every site that reads or writes an old redkiln file format … is located inside the `redkiln-migrate` crate"*; ADR-0100 decision 1 | Built here; `ci.yml`'s AC-442 lint lands with Slice 5 (§7d.15, DECISIONS **RT-39** as corrected 2026-09-18: only the AC-529 comment landed at stage-open). The migrator's porcelain read is **RT-37**'s one call site (`GitExec::run(GitOp::Status, …)` in `run_contract.rs`; no `rk-git` helper) |
| The migration branch and worktree | branch `rk-migrate/<target-HEAD-short>`; worktree `<git-common-dir>/redkiln/migrate-wt/`, **re-created, never reused** | AC-522's THEN: *"a dedicated migration branch checked out in its own worktree (created through the wrapper)"* and its Verified-by: *"re-created, never reused — ADR-0105 clause 6"*; `rk_git::worktree::add` refuses an existing path, which IS the clause | **DECIDED by D18-11** |
| The AC-450 kill seam | the binary prints `phase: written` on stdout after the write phase and before `commit`; the test kills on that line; no flag or env toggle | AC-450's *"a kill test (`taskkill /T /F` between the write phase and the commit)"*; AC-474 forbids a configuration key or env var that changes the write path | **DECIDED by D18-12**; the channel is **RT-38**'s (stdout, exact bytes `phase: written\n`); the in-process twin is a phase hook |
| The `ctrlc` dependency and the signal path | `rk-cli` only (`crates/rk-cli/src/signal.rs`: `ctrlc::set_handler(move \|\| token.cancel())`) | ADR-0099 decision 2: *"triggered by exactly two owners: the CLI's signal path (Ctrl-C / SIGTERM) and the per-activity timeout"*; no signal path exists on the tree [measured 2026-09-18: no `ctrlc`, `signal-hook` or `SetConsoleCtrlHandler` in any `Cargo.toml`] (C-01 **RA-M-04**) | **DECIDED by D18-2** (operator, 2026-09-18) — `ctrlc = { version = "3", features = ["termination"] }` (the feature is what makes `SIGTERM`/`SIGHUP` reach the handler on POSIX — ctrlc's `lib.rs`: *"Handling of `SIGTERM` and `SIGHUP` can be enabled with `termination` feature"*; on Windows the handler fires for CTRL_C, CTRL_BREAK and CTRL_CLOSE without it); alternatives named and not taken: an `rk_containment::signal` wrapper (needs an ADR-0127 annex — **and this file adds none**: the console-signal test-support sender an earlier draft put in `rk-containment` is not built, §7d.10), deferral to Step 8 |
| The exit row for a signal outside a dispatch | `Exit::SignalOutsideDispatch = 130` | RA-M-04: a Ctrl-C with no dispatch in flight produces no `StepError::Cancelled`, so it needs a non-`StepError` row (H-2) | **DECIDED by D18-4 (H-1)** |
| The poisoned-lock halt's exit and diagnostic | `Exit::PoisonedLockHalt = 70`; the diagnostic names the lock (`store` — the `Arc<Mutex<dyn EventStore>>`; `rk-store`'s writer mutex recovers its own poison today, RT-9/R-7 RAISED, so it is not observable here) and the verb at which the acquisition failed — a `std::sync::PoisonError` carries no panic payload, so none is printed; reached by an explicit `std::process::exit(70)` in `main.rs` after the message, never by an unhandled panic's 101 (§7d.4) | ADR-0118 clause 6: *"The process exit code, the diagnostic's wording, and whether the stop is reached by an unhandled panic or an explicit abort are the CLI's surface, owed at Step 7d"*; clause 5: 7d *"owes the discriminating test"* (C-01 **RB-M-01**) | **DECIDED by D18-4 (H-1/H-2)**; what ADR-0118 fixed — loud, names the lock, never a step failure, gate answer or completed run — is not reopened |
| `doctor`'s surface | `doctor` and `doctor --rebuild` **only**; `--fix` is CONDITIONAL (design §8.1: phase `—`, *"no install mechanism is decided"*) and `--probe` is *"an unnamed verb until one exists"* | C-01 **RB-M-06**; ADR-0092 clause 2.5 defers phase order to the design; AC-402's amendment of 2026-09-18 reads *"`--rebuild` (and, if it ships, `--fix`)"* | **DECIDED by D18-16** (the design's phase column); the snapshot pins `--fix`'s and `--probe`'s absence |
| `doctor`'s stdout | one line per **evaluated** rule — fired or not — beginning with its id token and a status word (`ok` included); the layout is **not pinned** — no schema, no snapshot | DECISIONS-2026-09-12 **E5**: stdout unspecified; **D18-6** as corrected 2026-09-18: tests assert on the `doctor::Report` value, the checkpoint asserts the exit code and greps each register-named id off a clean repository's stdout; RA-M-01's floor needs the ids to be **present**; the id vocabulary `RK-D-<NOUN>` is **RT-32**'s | **RT-44** — the rule-id token per evaluated row is the ONE thing this file requires of stdout, asserted as an existence floor (the named test `doctor_rule_table_covers_every_register_named_row` is the `Report`-level instrument and the block's FLOOR loop its process-level twin); a pinned contract would be E5's rejected option B |
| `doctor`'s version probe | `claude --version` on the installed binary through `rk_containment::run_bounded` (10 s; the child's stdout redirected to a file under `<common-dir>/redkiln/` — `run_bounded` returns an `Outcome` with no captured output and requires file-backed stdio [measured 2026-09-18, `crates/rk-containment/src/bounded.rs`] — read back and compared on its first whitespace-delimited token, since the binary prints `2.1.277 (Claude Code)` and the pin is `2.1.277`), compared to `TESTED_CLI_VERSION` (`2.1.277` at this pin) | AC-423's Verified-by as amended 2026-09-18 (C-01 **D-11**): *"the shipped probe is named. `doctor` runs `claude --version`"*; a spawn, not a dispatch — SM-121 stays 0 | **DECIDED** (the register's *"installed"*); the `VersionProbe` seam is the test's stub |
| The three ADR-0105 clause 6/7 `doctor` rows (a failed `worktree remove`'s orphan; the common dir on a network filesystem; a credential-shaped value in a retained field) | `RK-D-WORKTREE-ORPHAN`, `RK-D-NETWORK-FS`, `RK-D-CREDENTIAL-RETAINED` in the same table | ADR-0105 clauses 6 and 7 are Decision sentences that name `doctor` rows (*"a `doctor` orphans row"*; *"`doctor` checks the common dir is not on a network filesystem"*; *"`doctor` carries a rule row for credential-shaped values in any retained field"*); **no §12 criterion carries them** | **RT-45** — built to the accepted record and **claimed under no AC id**; a register criterion per row is OWED (§7d.18 RAISED). They are outside the RA-M-01 floor |
| The `doctor` rows `s7b-ii` §7b-ii.14 ARMED to 7d that no §12 criterion carries (AC-290, AC-306, AC-310, AC-314, AC-530's rows; AC-455's *"a `doctor` rule reporting the same at configuration time"*) | **six rows in `rk-cli`'s `doctor` table** — `RK-D-REVIEW-REEXEC`, `RK-D-QUARANTINE-LIVE`, `RK-D-SINK-COUNT`, `RK-D-ADVISORY-LEDGER`, `RK-D-CORPUS-SHA`, `RK-D-VERIFICATION-POLICY` (§7d.8; Slice 4) | `s7b-ii` §7b-ii.14 RAISED: *"until `s7d`'s set carries them the rows are orphaned. Owed: an amendment adding both to `s7d`'s acceptance set, or an erratum re-assigning them"* — the allocation is the operator's, **taken 2026-09-19 as DECISIONS-2026-09-18 D18-19** | **DECIDED (D18-19)**; O-16 closed; CROSS-LISTED in §7d.18, not claimed, no row built; the operator's word adds them to §7d.8's table or re-assigns them |
| SM-123's `doctor_rule_fired` event | none | SM-123's instrument names an event kind `EventKind::ALL` does not carry, and ADR-0116 permits minting by record only; E5 left stdout unpinned so the fallback has no schema (C-01 **D-19**) | **O-12 — OPEN**; 7d claims nothing on SM-123; the operator mints the kind by record (ADR-0116's route) or re-points the instrument at machine-readable output, which then needs the contract E5 declined |
| The composite `Toolchain` (two toolchain reals, one `Arc<dyn Toolchain>` slot) | **not built at 7d** — `Ports.toolchain` is filled with the Rust adapter on this workspace and the Node adapter is constructed and held for the `doctor` adapter row only | `s7c` §7c.1 RT-7: *"ARMED to 7d; a record naming the composite is OWED (R-8)"*; a composite is a `Ports` change that re-opens Checkpoint 7a or a new type no record names | **RT-7 / R-8 stand as 7c left them**; DECISIONS **RT-40** (which routed a `SelectingToolchain` composite here) is **withdrawn by a dated line in the log** — a composite is a new type no record names, and R-8 already owes that record; AC-350 stays ARMED (§7d.12); the two-toolchain case is Step 8's or a record's |
| The `VerifyReport` transport into `rk_kernel::verify` | **not wired at 7d** — the verify activity runner is not in this stage's set (the `s7b-ii` verify block is RAISED for re-routing, §7d.18) | `s7c` §7c.1 RT-6 / R-9: *"the transport into `rk_kernel::verify` is 7d's composition root — a record is OWED"* | **R-9 stands**; RAISED, not built; the `redkiln run` loop this stage builds reaches a gate step and a memoised step through fakes, not a verify activity |
| `Channel` (`Cli`, `TrackerComment`, `WebWrite`) and `verbs::answer::submit(channel, item, option, attachments)` | `rk-cli` (`crates/rk-cli/src/verbs/answer.rs`) — an `rk-cli` enum; the one non-`Cli` arm returns `Err(ChannelNotEnabled { phase: 1 })` **before any store read**, and `submit(Channel::Cli, …)` is the only path that reaches `gate::answer` (so the AC-405 call site stays one) | AC-263 (`rk-cli`, `rk-ports`): *"a port-level test driving a stub non-CLI channel against the answer API"* — no `Channel` type exists under `crates/` [measured 2026-09-18] and `rk-ports` is not edited (§7d.15), so the register's `rk-ports` tag is **met by no port type**: the stub drives `rk-cli`'s wrapper | Built here; the `rk-ports` tag is RAISED (§7d.18) — the register's own vacuity warning (a match arm over an enum with one live variant) is disclosed, not hidden |
| The lint-controls directory | `crates/rk-cli/tests/fixtures/lint-controls/` — the path `ci.yml`'s anchored `^crates/[^/]+/tests/fixtures/lint-controls/` exclusion already covers (RT-13's shape); positive controls only, each checked through the ban's own pipeline (`rk_control`) | DECISIONS **RT-30** as corrected 2026-09-18 (it read `crates/rk-cli/tests/lint-controls/`, a path the CI grep would not exclude) | Built here; nothing owed beyond the correction |
| `s8`'s re-spelt lines | `redkiln run "$ITEM" --lane fix` / `--lane story`, `explain "$ITEM"`, the stage listing over the trailer commits — landed 2026-09-18, read with **`git log --all`**, because H-5 puts the run's commits on `rk-run/<item>` and never on the checkout's `HEAD` (a bare `git log` there would fail a correct binary by construction; `rk-git`'s own reconcile reads `log --all` [measured 2026-09-18, `crates/rk-git/src/reconcile.rs`]) | DECISIONS **RT-33** | Landed; nothing owed |
| The `deny.toml` tracker/web ban list | eight `[bans].deny` rows — `octocrab`, `reqwest`, `hyper`, `axum`, `ureq`, `tokio-tungstenite`, `warp`, `actix-web` (Slice 1) | DECISIONS **RT-36**; AC-405's Verified-by as amended (D-13: *"and their kin"*) | Built here; the block greps each row and the `cargo metadata` walk asserts none is in `rk-cli`'s closure |
| Where a refused `run`'s one row goes | a refused `run` that reached the reconcile (no pack; non-derivable lane; a lane whose pack is not enabled) writes **exactly one** `reconcile_started` row and nothing else; the AC-486 adapter refusal and the AC-413 foreign-store refusal happen at steps (4)–(5), **before** the reconcile, and write **zero** rows (AC-486: *"before any store write"*) | AC-172's instrument: *"exactly one `reconcile_started` event per invocation"*; **D18-8** read *"zero store writes"* until its dated correction of 2026-09-18 — the two could not both hold, and AC-172 is the register's | **RT-46** — read for AC-172; `run_on_init_only_repo_is_a_config_outcome` asserts `store.len() == 1` and the kind; **D18-8 reads the same way** (corrected in the same commit) |
| The run-owned worktree and branch a tier-1 write resolves *from the store* (H-5) | `<common-dir>/redkiln/worktrees/<item>/` on `rk-run/<item>`, re-created by `run` and recorded as `RunOwnership { worktree, branch }` on the invocation's `reconcile_started` row beside RT-41's two members | ADR-0105 clause 8 and AC-529 say *from the store* and name no table or event that carries the pair [measured 2026-09-18: no store table and no §5 payload names a run-owned worktree]; `authorise_tier1_write` takes the pair as a value | **RT-47** — routed onto the same per-invocation row RT-41 uses; **the record naming the ownership carrier is OWED** (a dated design §5 note, or the same ADR-0116-route kind RT-41 may mint) *(dated 2026-09-27, `s8d-ii` build, P6, under ADR-0180, proposed *(dated 2026-09-28, `s8e` close: accepted on D21-91, at `s8d-ii`'s close)*: the pair's path is now `.redkiln/worktrees/<item>/` under the main worktree on `rk-run/<item>`, not `<common-dir>/redkiln/worktrees/<item>/`, and the tree is reused rather than re-created (D21-44; D21-45 W2); the carrier, the `reconcile_started` row, is unchanged)* |
| The pack loader's diagnostic mode | `rk_kernel::catalog::load_diagnostic` — every load error typed, `doctor` its only caller (§7d.8) | AC-414's *"every fixture's expected rule fires"* over a corpus whose fixtures are load-rejected by construction; the strict loader stops at the first error | **RT-54** — routed, not decided (a second loader entry point in a Step-7b crate; the per-file strict-loader alternative is named and rejected in §7d.1); the record naming the diagnostic mode as the loader's surface is OWED |
| The node-id ↔ `Activity::id_at_version` join and the per-node spec supplier | `rk-cli`'s `adapters.rs` binds every node the run will dispatch through `rk_adapter_claude::binder::Binder::new(config)` + `bind(BindRequest { … }) -> ActivitySpec`, then `inputs.with_spec(node_id, RecordedSpec::supplied(activity_id_at_version, spec))`, and `ClaudeHarness::new(binder.into_config(), token)` (§7d.5 step (8)); because node ids are known only after `select()` runs inside the loop, `replay::resume` takes a **spec-supplier seam** — `&mut dyn FnMut(&str) -> Result<RecordedSpec, ReplayError>` — consulted where `record()` calls `spec_for` (the sixth kernel edit, §7d.1) | `replay.rs`'s own header: the spec is the binder's output and *"a record routing spec assembly is OWED"*; `spec_for` refuses a dispatched node with no binding, and `ClaudeHarness::dispatch` refuses a `card_digest` its validated store does not hold [measured 2026-09-18, `crates/rk-kernel/src/replay.rs`, `crates/rk-adapter-claude/src/lib.rs`, `binder.rs`] — so a `run` with no binding step is red on `inputs/specs.json` before any dispatch, the wrong reason | **RT-55** — routed, not decided; **the record routing spec assembly (the join and the supplier) is OWED** (`replay.rs` names it); the fake-harness tests still need `with_spec` / the supplier — the fake ignores `card_digest` but `spec_for` does not |
| The rebuild marker | a **sidecar file** `<common-dir>/redkiln/rebuilt` written by `doctor::rebuild` after the rename, its content the rebuild's source `HEAD` sha, removed by the next successful `run`; read by `RK-D-REBUILT-TIER2` (§7d.8) | AC-397's WHEN/THEN name no marker and `rk-store`'s schema carries no rebuild column or table [measured 2026-09-18, `crates/rk-store/src/migrations/`]; `rk-store` is not edited at 7d (§7d.15) | **RT-56** — routed, not decided; **a dated design §5 / §9.2 note naming how `doctor` learns a store was rebuilt is OWED** |
| The `PackExtras` key floor | `margin_ticks`, the two TTL keys, `gate_deadline_secs`, per-activity `timeout_secs`, the gate-class expiry action — `rk-cli`'s own serde reader over the SAME `.redkiln/packs/<pack>.toml` bytes, `deny_unknown_fields` OFF (§7d.5 step (7)) | `rk_kernel::select::Pack` exposes none of them and `toml_subset` ignores unknown keys [measured 2026-09-18, `crates/rk-kernel/src/select/pack.rs`]; something must parse them and the kernel's `Pack` is not widened (no 7b re-open for this) | **RT-57** — routed, not decided; an existence floor **7e's pack schema record re-points** — nothing further owed by this stage |
| The report `kind` vocabulary's item rows | `initiative`, `project`, `story`, `spec`, `report`, `config`, `process` beside the fourteen companion wire names (§7d.11 step (4), §7d.13) | RT-28 fixes the schema's shape (`kind` a string) and not the vocabulary; the block's report check and two implementers must count the same rows | **RT-58** — routed, not decided; folded into **RT-28's OWED schema record** (the vocabulary is a column's value set, not a second record) |
| The human-supplied root's carrier | an `artifact.write` row written by **`answer`** for an admitted attachment, in ADR-0116 decision 1's payload shape (§7d.6) | ADR-0063: the attachment is *"stored as a human-supplied root"*, naming no row; ADR-0116 decision 1 fixes `artifact.write` as *emitted by every activity output* (its producing-step column) — `answer` is a verb, not an activity output, so the row's **producer is widened** here | **RT-59** — routed, not decided; **a dated design §5 note or an ADR-0116-route record naming the producer is OWED**, beside the payload-widening question §7d.18 already carries |

**Line numbers into the AC register are not used in this file** (DECISIONS-2026-09-15 D15-2; `s7b-ii` §7b-ii.1
records the +39 drift that motivated the rule; C-01's scope pass found every 2026-09-12 pointer moved again). Every
register citation is by id and quoted clause; a line number appears only inside a dated parenthetical recording what
was measured. **Every external anchor into this file today is by LINE — three** [measured 2026-09-18 by `grep -rn -oE
's7d\`?:[0-9]+'` over `runbook/` and `spikes/`], all in `s7a` §7a.9's AC-485 / AC-486 / AC-488 rows, each reading
*"claimed by id at `s7d`:117"* (DECISIONS-2026-09-18 R-35) — line 117 of the 153-line file was the acceptance-set
sentence naming those three ids, now §7d.18's opening paragraph; `PROGRESS.md` §5's C-01 row read *"noted in s7d"*
with no line until this commit flipped it to *done*. Each reads against the pre-hardening pin `239e899` (`git show 239e899:runbook/phase-1/s7d-cli-and-migrate.md`)
and is re-anchored to §7d.18 by symbol when `s7a` is next opened (R-21's rule); the closed stage files are not edited
here. The C-01 adjudication's own line anchors into the 153-line text are working notes in the session scratchpad, not
in the tree.

### 7d.2 The C-01 pass

**Before this step, not after (audit §8 C-01).** The 153-line file's closing paragraph carried the mandate verbatim:
*"the CLI/doctor criteria AC-400–AC-439 and SM-121–SM-130 had no owning slice in the 2026-09-10 audit — 27 of 40
criteria were never named by any finder. Run one slice finder + refuter over them (register §12, SM §1.11, design
§8.1–8.6, this file) with the decidability and two-implementer lenses before the router is written, and record the
result in `PROGRESS.md`."* **The pass has been run** — on 2026-09-18, before this rewrite, as one finder, two adversarial
refuters (the decidability lens; the two-implementer lens) and an adjudicator that re-read every citation at the
primary source, on `s7b` §7b.9's C-02 form. It is recorded in `PROGRESS.md` §6 as **C-01 DISCHARGED** and the §5 row
is flipped to done; this section states the results so the stage file carries them and not a pointer.

**First: the scope pointers were stale, and are corrected here.** Audit C-01's *"AC 4970–5471"* and *"design 1259–1437"*
are **line numbers** in the 2026-09-10 versions, not id ranges — register :4970 sits inside AC-310's text *(measured 2026-09-18)*; the
subject is the register's **§12** (AC-400..AC-439 plus AC-531, 41 in force, with the re-issued AC-428), its **§12.10**,
and design **§8.1–§8.6**. *"RUNBOOK 2079–2125"* names a monolith that no longer exists; the content is this file plus
`s8`'s exit-check block. The 153-line file's own citations — the register's §12.10 as *"line 5132"*, the phase note as *":6263"*, the
claim *"AC-400–AC-439"* (which should read AC-400–AC-439 plus AC-531, plus AC-263 which `s7b-ii` §7b-ii.14 hands here)
— were each off, and design §9.1's `rk-cli` row omitted `explain`, `init` and `catalog search` until its amendment of
2026-09-18. Every DECISIONS-2026-09-12 register pointer had moved (E9's, E13's) and most §0.3 row citations in all three
reports were off by a line or more. **Cite by id and clause** (D15-2) is the rule this paragraph re-learns.

**Twenty-seven divergences survived adjudication, seven of them blocking**; three finder items were dropped as decided
by the records; thirteen refuter misses were admitted, five by folding into finder items. Each is listed with its
resolution as **TAKEN** (an operator decision of 2026-09-18, recorded in `spikes/DECISIONS-2026-09-18.md`, or a record
already landed) or **RECORDED** (owed above this file, listed again in §7d.18 RAISED). Blocking first.

**Blocking — each a place two competent implementers reading the records build different binaries:**

1. **D-01 — the exit code of the most common `run` outcome was contradicted inside the register** (AC-403, AC-438,
   AC-240, AC-024). AC-403's THEN — *"the `Gate:` trailer and `gate_opened` event are durably written and the process
   exits 0"* — AC-240's *"terminates with exit code 0"* and AC-024's *"exits 0"* against AC-438's *"success,
   configuration error …, gate-open, findings-block and each runtime error class are distinct values"*; register THENs
   at equal precedence, and no accepted ADR Decision broke the tie (ADR-0009's *"the process exits 0"* is Context;
   ADR-0087 says only *"the process may die"*; ADR-0107 is silent). **TAKEN** (DECISIONS-2026-09-18 D18-17; rejected: a distinct non-zero gate-open code moving
   three register THENs and an ERRATA entry on ADR-0009): a run that opens a gate **exits 0**;
   *gate-open* is struck from AC-438's distinct-value list by dated amendment (landed 2026-09-18) and design §8.1's
   exit-code paragraph is amended to match; the exit table has no gate row (§7d.4).
2. **D-02 — `run` had two grammars in two ADR-0107 Decision sentences** (AC-400, AC-401, AC-264, AC-186, AC-403).
   Decision 1's *"`run <item>` is the one entry point"* against decision 4's *"re-invoke `redkiln run --lane fix
   --item <item>`"*; the register carried both (positional in AC-401/AC-403/AC-186/AC-264; two-flag in AC-400's
   Verified-by), so did the design and `s8`; whether `--lane` was required, optional or derivable was stated nowhere,
   and `fix`/`story` mapped to no pack name (E7's packs are `bug-standard` and `story`). An erratum may not touch a
   Decision sentence (ADR-0092). **TAKEN** (DECISIONS-2026-09-18 D18-17; rejected: two required flags; positional-only
   with no lane flag): `run <item> [--lane fix|story]` — positional item, `--lane` optional and
   derived from the item's kind when derivable, a missing non-derivable lane a configuration error, `fix →
   bug-standard`, `story → story` — as **ADR-0128**, superseding ADR-0107 decision 4's grammar sentence in part,
   committed `proposed` at stage-open; AC-400's Verified-by, design §8.1 and `s8`'s three invocations amended toward it
   (§7d.3, §7d.5).
3. **D-04 — `answer` had no decided grammar** (AC-405, AC-406, AC-407, AC-410). ADR-0009's Decision fixed `redkiln
   answer <item> approve|changes-requested` — two literal verdicts, no attachment argument — while ADR-0063's Decision
   makes an option a registry-declared id that *"may declare `attaches = <Kind>`"*; AC-410 has `answer` *"passed an
   attachment"*; `gate::answer` takes `verdict: &str` untyped and no registry existed; AC-410's three instruments were
   all satisfied by a refuse-all validator (absorbing RA-M-06). The token half was REFUTED: AC-248's *"`redkiln answer
   <item> approve` without a resolvable live token exits non-zero"* has no token argument — the CLI resolves it.
   **TAKEN** (DECISIONS-2026-09-18 D18-17; rejected: a two-literal `clap` enum; the registry deferred wholesale to
   7e): `answer <item> <option> [--attach <kind>=<path>]` with the token CLI-resolved from the open gate record
   (no `--token`), options validated against an ADR-0063 question registry whose **type lands in `rk-kernel` at 7d
   with the default question built in** (`approve` / `changes-requested`) and whose pack-declared questions are 7e's;
   AC-410 gains a **positive control** (an admitted attachment is accepted and stored as a human-supplied root) — as
   **ADR-0129**, superseding ADR-0009's Decision sentence as a grammar in part, `proposed` at stage-open and `accepted`
   2026-09-19 (§7d.6).
4. **D-07 — Checkpoint 7d failed a correct binary.** The 153-line block ran `redkiln doctor` from the product root,
   required exit 0, **before** `init`, in a repository with no `.redkiln/`; AC-426 makes `unread_window_runs` required
   with no default and AC-438 folds an error-severity rule into the configuration-error code, so the line selected for
   a doctor that tolerates an absent configuration (PIA-059). It absorbed RB-M-09: `init` left `.redkiln/config.toml`
   untracked in the product repo with nothing saying whether it was committed. **TAKEN** by this file (execution order
   is the runbook's): every `init`, `doctor`, `run` and dry-run line runs against **temporary repositories the block
   mints with `mktemp -d` and `git init`** (**D18-9**), never the product repository, whose `.redkiln/` is 7e's and is
   **never created by this stage**; the disposition of the product repo's scaffold is therefore *not created here*
   (§7d.17). Reading B — a tolerant doctor — is not a competent reading and is not built.
5. **D-14 — what `init` creates was undecided** (AC-437, AC-531, AC-413, AC-485, AC-486). E6 recorded the record as
   owed; three forks hung on it: whether `init` creates the store (under lazy creation AC-437's *"identical store
   identity rows"* compares nothing; under eager creation AC-531's regex was red on `redkiln`), what `[adapters]`
   carries (an empty table leaves a bare-`init` repository unrunnable under AC-486), and AC-427's sanction of an
   absent store. The finder's boot-id mechanism was REFUTED (identity.rs cut it; three fields). **TAKEN** (DECISIONS-2026-09-18
   D18-17; rejected: lazy store creation with an empty `[adapters]`): `init` is
   **EAGER** — the scaffold names the four compiled-in reals, `[packs] enabled = []`, `[doctor] unread_window_runs =
   50`, and creates the store through `rk_adapter_sqlite::SqliteStore::open` so AC-437 compares three real identity
   rows; AC-437's Verified-by is amended to fix the scaffold verbatim and AC-531's regex is scoped by dated amendment
   to the working tree, refs and hooks (both landed 2026-09-18) (§7d.9).
6. **RA-M-02 (merging RB-M-02) — `catalog search` was absent from the verb list and the loader had no `keywords`
   field** (AC-436, AC-400). Design §8.1 lists the verb at Phase 1 inside the range the file claimed; the file's verb
   list and design §9.1's `rk-cli` row omitted it; `grep keywords crates/rk-kernel/src` found only a lexer's doc
   comment, so AC-436's permutation test *"asserts an unchanged selection"* on a loader that drops the field before
   selection sees it — a check that cannot fail. **TAKEN** by this file (the register and design already decide Phase
   1): the verb list gains `catalog search`; the loader reads `keywords` (an `rk-kernel` edit); AC-436's Verified-by
   gains a positive control (a search that finds an activity by keyword — landed 2026-09-18); the property test runs
   over a synthetic catalog until 7e ships v0; design §9.1's row is corrected (landed) (§7d.3, §7d.7).
7. **RB-M-01 — ADR-0118 clauses 5 and 6 hand 7d the poisoned-lock test and the halt's exit and diagnostic, and the
   file named none of it** (AC-438). Clause 5: *"Step 7d owes the discriminating test … a fixture that poisons the
   store lock … and asserts the run stops — no `StepError` value produced, no recovered guard, no subsequent
   activity"*; clause 6: the exit code, wording and mechanism are *"the CLI's surface, owed at Step 7d"*; and
   `PROGRESS.md` records a live poison recovery in `rk-store`'s `conn.rs` this test would catch. **TAKEN** by this
   file (the record delegates it): the fixture joins §7d.14 red first; the halt has its own row in the snapshot-pinned
   table (`70`, H-1/H-2); the diagnostic names the lock; ADR-0118's fixed half is not reopened (§7d.4, §7d.10).

**Non-blocking — each decided by the records but owed a correction:**

8. **D-03** (AC-430–AC-433) — `s8`'s `explain --run "$RUN_ID" --stages` was a runbook-only shape; every §12 THEN,
   ADR-0107 decision 4 and design §8.1 write `explain <item>`. **TAKEN**: 7d builds `explain <item>` only; `s8`'s two
   lines are the losing record and were rewritten against the event log (landed 2026-09-18). **TAKEN 2026-09-19**: the
   digest d4 asserts is each committed step's `output_digest`, named in **ADR-0128 decision 4** (O-13 closed; §7d.7).
9. **D-05** (AC-435, SM-126, AC-405; absorbing RB-M-08) — AC-435's THEN records the yes/no *"as a required field on the
   gate answer"* and its Verified-by's machine half is *"the gate answer cannot be submitted with the field absent"*;
   design §5's `gate_answered` row lacked it and named an `answerer` the kernel never writes. **TAKEN**: `answer`
   takes **two required flags**, `--explain-sufficed yes|no --toml-opened yes|no`, and `gate_answered` gains **one**
   member, `explain_sufficiency { sufficed, toml_opened }` (ADR-0129 decision 1 as accepted; design §5's `gate_answered`
   row amended 2026-09-18, corrected 2026-09-19). The stage-open commit shipped the proposed record's one-flag reading
   and RAISED the two-data reading for the operator; **answered 2026-09-19: two data, one field** — SM-126's kill counts
   the cell *sufficed = yes AND opened = yes*, which one boolean cannot carry (D18-17 D-05 as amended). `answerer`'s
   source is routed (RT-42, §7d.1) and **confirmed 2026-09-19** (R-26 closed: the git author identity through the wrapper).
10. **D-08** (AC-426, AC-472, AC-147, SM-139) — three texts for one rule. **TAKEN** by the records: the predicate is
    **kind-per-stage over the last N runs, N = the repository's `unread_window_runs`** (AC-426, the register's §0.3 owner; ADR-0116's
    Context quotation of the kind-level form with N = 50, and its decision 4's produced-vs-consumed reading); AC-472's *"declared constant in the rule table"* and AC-147's
    windowless form were amended toward the owner (landed 2026-09-18) (§7d.8).
11. **D-09** (AC-426, AC-472) — the checkpoint regex could not match `unread_window_runs`. **TAKEN** by this file: the
    block greps the key, and asserts `init`'s scaffold writes `unread_window_runs = 50` (§7d.17).
12. **D-10** (AC-414, AC-422, AC-423) — *"does not link a live adapter"* had no cargo-graph meaning for the composition
    root that constructs all four reals, and a source ban on `rk_adapter_*` in the doctor module collided with the
    AC-422 row that reads `env_denylist` and the AC-423 probe. **TAKEN**: AC-414's Verified-by amended (landed) —
    *live adapter* means a constructed `Harness` implementor; the lint's denylist is the four adapters' constructor
    paths; `env_denylist` consts and the probe stay readable; the `HarnessPort` fake is the behavioural half (§7d.8).
13. **D-11** (AC-423, SM-129, SM-121) — no record named the shipped version probe. **TAKEN** for the register's own
    word *"installed"*: `claude --version` on the installed binary through `rk_containment::run_bounded`, compared to
    `TESTED_CLI_VERSION` — a child process that is not a dispatch, so SM-121's counter stays 0; AC-423's Verified-by
    names it and SM-129's *"on every event"* is corrected to per dispatch (both landed 2026-09-18) (§7d.8).
14. **D-12** (AC-438, AC-432, AC-413, AC-531, AC-400) — the table is keyed on `StepError` yet governs *"for any
    reason"*; usage errors, no-record, foreign-store refusals, `init`'s naming refusal, the halt and a signal outside a
    dispatch had no named row. **TAKEN** by this file as header-table row **H-2** (§7d.1), with the values in H-1;
    a dated note on AC-438 recording the enumeration landed 2026-09-18 (§7d.4).
15. **D-13** (AC-405) — *"the gate-answer store method"* does not exist; the symbol is
    `rk_kernel::workflow::gate::answer` behind AC-262's single wrapper, and *"no tracker or web crate"* named no list.
    **TAKEN**: AC-405's Verified-by amended (landed) to name the symbol and to make the crate-graph instrument a
    `deny.toml` ban list of tracker/web crates this stage authors — DECISIONS-2026-09-18 RT-36's eight: `octocrab`,
    `reqwest`, `hyper`, `axum`, `ureq`, `tokio-tungstenite`, `warp`, `actix-web` (the register's *"and their kin"*)
    (§7d.6, §7d.15).
16. **D-15** (AC-403, AC-437) — the stdin lint's *"outside `init`"* licensed an interactive `init` that E6, the
    scaffold, design §8.1's idempotence and the block's `init && init` all contradict. **TAKEN**: no verb reads stdin
    in Phase 1; AC-403's Verified-by loses the exemption (landed 2026-09-18); `init` reads no stdin (§7d.9).
17. **D-18** (AC-400, AC-437, AC-413) — CI's `env::current_dir` ban in every `crates/*/src` against bare checkpoint
    verbs; AC-529 and ADR-0105 clause 8 are about tier-1 writes and say nothing about verb discovery. **TAKEN** by
    this file as header-table row **H-3**: discovery from the inherited cwd through the wrapper, `-C <path>` optional;
    `ci.yml` carries the scope comment (landed 2026-09-18) (§7d.3).
18. **D-19** (SM-123) — `doctor_rule_fired` names an event kind no table carries; minting is by record (ADR-0116);
    E5 left stdout unpinned. **RECORDED** as **O-12**: 7d claims nothing on SM-123 until the operator mints the kind or
    re-points the instrument (§7d.8, §7d.18).
19. **D-20** (SM-139, AC-472, AC-426) — SM-139, AC-472's Gap and design §8.4 still said N was undeclared against E9 and
    the file's own closing paragraph. **TAKEN**: N = 50 (E9); the 49-run and 51-run fixtures are red first here; the
    three stale texts were amended (landed 2026-09-18); four further twin texts still read N undeclared or a rule-table
    constant — design §8.10's open-items row (under *"What this section leaves open"*, not `## 10.`), the SM register's open-items row for SM-139, the AC register's §0
    open-items row 14 and its §14 narrative — and take the same dated amendment, landed in this commit (§7d.8, §7d.13).
20. **D-21** (AC-427, SM-124) — the *"delete-and-rebuild CI job over the `redkiln-corpus` pin"* cannot run in CI
    (`CORPUS_PIN` names a local path); `s8` settles fidelity on the kernel's local corpus tests; the §0.3 owner is
    AC-184. **TAKEN**: 7d claims AC-427 as **verb + direction** (AC-429's git → store) and **fidelity at `s8`**; SM-124's
    instrument was amended to the local-only suite (landed) (§7d.8, §7d.18).
21. **D-22** (AC-411, AC-439, AC-428, AC-404, AC-408, AC-263) — the bare range re-asserted twins 7c discharged and kernel
    behaviours 7b built, and AC-411 was claimed by no stage. **TAKEN** by this file: §7d.18 carries a disposition per
    id — twin-discharged-at-7c (AC-428, AC-439), kernel-built (AC-404 bar its exit, AC-408), `rk-store`-owned and
    **RAISED** (AC-411: DECISIONS-2026-09-18 R-25 lists it *"rk-store, new here"*; **the store has no render path at
    this pin** [measured 2026-09-18: no `fn render` under `crates/rk-store/src`; `locator` is the one module that reads
    markdown], so *new here* would mean building `rk-store`'s render path inside a stage whose closed-crate rule forbids
    it — R-25 takes a dated line recording that, and the id waits for an owner record), machine-half-only
    (AC-435), verb-only (AC-427), not-7d (SM-122, SM-123, SM-124).
22. **RA-M-01** (AC-414, AC-415, SM-121) — over an empty rule table every join is vacuous; AC-414's own *Repaired* note
    says it once *"passed trivially on a doctor that performs no checks"*. **TAKEN** by this file: the checkpoint joins
    the register-named rows — AC-409, 412, 413, 416–426, 486, 488 — to the emitted ids so an empty table fails, and a
    named test asserts the same over the `RULES` const; AC-415's Verified-by gained the non-empty floor (landed)
    (§7d.8, §7d.17).
23. **RA-M-04 (merging RB-M-04)** (AC-403, AC-438) — ADR-0099 decision 2 assigns the signal path to the CLI and no
    criterion, dependency or exit row covered it; `CancelledBy::Signal` was reachable through nothing. **TAKEN**:
    **D18-2** names the mechanism (`ctrlc = "3"`, `rk-cli` only), §7d.10 names the test and the two rows (`13` in a
    dispatch, `130` outside one).
24. **RA-M-05** (AC-409) — the *"compile-time test that the gate step type exposes no `outputs`"* is unwritable against
    7b's single `Step` struct (`output_type: Option<String>` on every kind). **TAKEN**: AC-409's Verified-by amended
    (landed) to a loader test asserting a `Gate` step carrying `outputs` is rejected at load, with the fixture in the
    AC-415 corpus (§7d.8).
25. **RA-M-07 (merging RB-M-07)** (AC-413, AC-437) — boot-id in AC-413's GIVEN and design §8.4 against
    `rk-store::identity`'s three fields and AC-369's cut. **TAKEN**: boot-id dropped from AC-413's GIVEN and design §8.4
    (landed 2026-09-18); the doctor fixture is authored over three fields (§7d.8).
26. **RB-M-05** (AC-485, AC-486, AC-422) — ADR-0083's Decision names a `Config` value no crate declared. **TAKEN**
    (operator, D18-14): `pub struct Config` in `rk-kernel`, constructed only at the root, threaded into `RunInputs`, its
    digest on the CLI-written `reconcile_started` row (RT-41); design §9.1's dated note landed; the composition root's one environment read is asserted
    by the block's `std::env::var` confinement grep (§7d.5, §7d.17).
27. **RB-M-06** (AC-402, AC-400) — AC-402's 2026-09-12 rationale named `--fix`, which design §8.1 marks CONDITIONAL with
    no phase. **TAKEN** (ADR-0092 clause 2.5 defers phase order to the design): the surface is `doctor` and `doctor
    --rebuild` only; the snapshot pins `--fix`'s absence; AC-402's note reads *"`--rebuild` (and, if it ships,
    `--fix`)"* (landed) (§7d.8).

**Dropped as decided by the records, with why:** **D-06** — every `doctor` invocation reconciles: AC-402's GIVEN lists
`doctor` unqualified and its Verified-by enumerates *"the three entry paths"* as a closed set, AC-172 names the same
three, ADR-0109 decision 1 governs by *"can cause an action"*, `rk-git`'s `Entry` enum has a flagless `Doctor`, and
`reconcile_at_entry` reads git commits only so a `--rebuild` against an absent store reconciles fine; the `--fix`
question survived as RB-M-06. **D-16** — `status` reads the store: ADR-0105 clause 8's *"`status` does not
reconcile"* and AC-172 forbid it, and reading trailer commits directly IS a reconcile under another name; the absence
of an output contract is E5's deliberate gap, not two readings. **D-17** — AC-429's instrument exists: the AC-174
`.git`-fingerprint test in `rk-git`'s suite (`reconcile_mutates_nothing_in_git`) fingerprints every file under `.git`
around `reconcile_at_entry(.., Entry::Doctor)`, and ADR-0105 clause 5 makes the one wrapper the only git exec, so
*"any write attempt"* lands in the fingerprint (the adjudicator wrote *"a journal assertion"*; `GitExec` keeps no
journal — the `GitJournal` is `gitops::GitOps`'s, the kernel's replay seam — so the fingerprint is the whole
instrument, §7d.5); both implementers reach the same observable.

**The decidability table, in one paragraph.** Of the 41 §12 ids plus SM-121..SM-130 and SM-139, the adjudicator found
**decidable at 7d as written**: AC-400, 401, 402, 404, 405, 406, 407, 408, 411, 412, 415, 416–426, 429, 430, 431, 432,
433, 434, 439 and AC-428 (the last two discharged at 7c), SM-121, SM-127, SM-130, SM-139; **partial** until an amendment
or a decision landed: AC-403 (D-01), AC-409 (RA-M-05), AC-410 (D-04), AC-413 (RA-M-07), AC-414 (D-10, RA-M-01), AC-435
(the machine half), AC-436 (RA-M-02), AC-437 (D-14), AC-438 (D-01, D-12, RB-M-01, RA-M-04), AC-531 (D-14), SM-125,
SM-126, SM-128, SM-129; **not-7d**: AC-427 (fidelity at `s8`), SM-122 (the v0 catalog is 7e's), SM-124 (`s8`); **no**:
SM-123 (D-19). Every *partial* is now decidable by the amendments and decisions above, and §7d.18 places each id
accordingly.

### 7d.3 The router

**The tree.** One `clap` `Command` named `redkiln`, derive-built in `crates/rk-cli/src/cli.rs`, with these
subcommands and no others: `run <ITEM> [--lane <LANE>]` (`LANE` a `clap::ValueEnum` closed at `fix` | `story` —
ADR-0128 decision 1: *"a third value is a usage error, not a configuration error"*); `answer <ITEM> <OPTION>
[--attach <KIND>=<PATH>]... --explain-sufficed <yes|no> --toml-opened <yes|no>` (ADR-0129 decision 1 — two required flags, one record field; §7d.1's routing row, §7d.6); `status`; `explain <ITEM>`; `doctor
[--rebuild]`; `init`; `catalog search <QUERY>`; `lint (--facet-stability | --facet-provenance) <FACETS_TOML>
[--stability-record <PATH>] [--pinned-corpus <PATH>]` (the two mode flags in an exclusive group, `s8`'s invocation form;
the kernel functions are `facet_stability_rule(&Registry, &BTreeMap<String, StabilityRecord>, corpus_pin: &str)` and
`facet_provenance_rule(&Registry, &PinnedCorpus)` [measured 2026-09-18], so `--stability-record` is read through
`Registry::stability_records_from_str` (absent → an empty map, the zero-`llm` branch), the pin string is `CORPUS_PIN`'s
`CORPUS_SHA=` line read from the repository root — a file read, not an env read — and `--pinned-corpus` is read through
`Registry::pinned_corpus_from_str` (absent → `<CORPUS_REPO>/results/` per `CORPUS_PIN`, the `s8` form; the tests pass a
fixture file)). One global option, `-C
<PATH>` (H-3), and clap's own `--help` / `--version`. **Every verb is a read of the world, a decision, and an exit**
(design §8.1); no verb prompts, and no verb reads stdin (D-15).

- **AC-400** — BINDING. *"GIVEN the shipped `clap` router for Phase 1, whose committed argument snapshot contains `run`
  with its full flag set / WHEN any invocation passes a flag or verb whose name appears on the committed resume-shaped
  denylist (`resume`, `continue`, `stay`, `--stay`, `--resume`, `--continue`, `--from`, `--pick-up`, `--retry-step`,
  `--from-step`, `--again` — ADR-0107 decision 3's list plus this register's earlier three) / THEN the invocation is
  refused as an unknown argument, and no denylisted name appears anywhere in the snapshot."* *Verified by:* *"a
  committed snapshot test of the full `clap` argument surface (`trycmd`/`assert_cmd` over `--help` of every verb,
  subcommands × long flags), which fails on any diff, plus a router lint over the parser definition that fails the
  build on a name matching the committed denylist."* Three instruments, three tests: `ac400_help_snapshot_every_verb`
  (`trycmd` over `tests/cmd/*.trycmd`, one file per verb and one for the root, each `--help` captured; a `--help` on
  `catalog search`, `doctor`, `lint` and every verb with a sub-surface); `ac400_denylist_absent_from_parser_tree` —
  **the lint is a test walking the built `clap::Command` tree** (**D18-13**; the walk is shared by the four tree tests
  and carries its own coverage control — it returns the visited name set, and each test first asserts that set contains
  `run`, `--lane`, `catalog`, `search`, `<QUERY>`, `lint`, `--stability-record`, `doctor` and `--rebuild` — a nested
  subcommand, a nested positional and a nested long flag — so a walker that stops at the top level or skips args is red
  before any absence is asserted): every subcommand name, alias, long flag
  and short flag, recursively, against `pub const DENYLIST: [&str; 11]` in `cli.rs`, **exact match on the name with
  and without its `--`** (a prefix rule would refuse `--from-scratch` on a future verb for a name the list does not
  carry — the register's own *Limit*: *"the denylist is name-based"* — and a substring rule would fire on `continue`
  inside `--continue-on-error`; the exact-vs-prefix rule the C-01 decidability table said was unstated is stated here
  as **exact**); and `ac400_resume_refused_as_unknown_argument` (`assert_cmd`: `redkiln run --resume` exits `2` and
  stderr matches `unexpected argument '--resume'`). **The behavioural check the Verified-by names** — *"the runbook's
  Phase-1 exit check (3) … re-invoke … and assert digest equality via `redkiln explain <item>`"* — is `s8`'s, in the
  ADR-0128 grammar — `redkiln run "$ITEM" --lane fix`, the optional flag kept per RT-33 until 7e's item schema makes
  the fixture item's lane derivable (ADR-0128 decision 4 spells the re-invocation `redkiln run <item> [--lane fix]`
  and decision 6 names the same grammar for the three amended records).
- **AC-186** — BINDING (`rk-cli`; twin of AC-264). *"GIVEN the shipped CLI surface / WHEN it is enumerated / THEN no
  `--resume`, `--continue` or equivalent flag exists on any command AND `redkiln run <item>` is the sole entry for
  both a first run and a resumed run."* *Verified by:* *"a `clap` surface snapshot test that must be updated
  deliberately to add any flag."* The snapshot and the tree walk; the *sole entry* half is ADR-0128 decision 1's
  *"no `--item` flag"* plus the absence of any other verb that takes an item and causes action — `answer` causes an
  action on a gate, not a run, and the tree has no `resume`.
- **AC-264** — BINDING (`rk-cli`; `s7b-ii` §7b-ii.14 ARMED its CLI half here). *"GIVEN an item whose gate has been
  answered / WHEN the operator resumes it / THEN `redkiln run <item>` is sufficient and `--resume` is not an accepted
  flag on any command."* *Verified by:* *"a CLI snapshot test of the accepted flag set; a test asserting `redkiln run
  --resume` exits as an unknown argument."* The same two tests; the *sufficient* half is the named test
  `ac264_run_after_applied_answer_resumes_past_gate`: open a gate (`ac403_…`'s fixture), `answer <item> approve
  --explain-sufficed yes --toml-opened no` (`Applied`), then `redkiln run <item>` with no flag — exit `0`, the fold shows the gate step
  committed and the next step reached (a second `dispatch` row, or `run` completing the pack), and no `--resume` was
  needed; red by a `run` that re-opens the gate or refuses (AC-186's *"a resumed run"* is the same assertion).
- **AC-402 (snapshot half)** — BINDING. Its Verified-by's second instrument: *"the argument-surface snapshot (AC-400)
  additionally fails the build on any `--no-reconcile` / `--skip-reconcile` / `--fast` flag"* —
  `ac402_no_skip_reconcile_flag_in_snapshot` walks the same tree for the three names (exact) and the snapshot diff is
  the second line. The three-paths half is §7d.5's.
- **AC-441 (router half)** — BINDING (`rk-cli`). *"GIVEN the shipped `redkiln` CLI router / WHEN its full subcommand
  table is enumerated / THEN no subcommand performs or triggers a migration AND `redkiln-migrate` is reachable only as
  its own binary."* *Verified by:* *"a snapshot test of the `clap` command tree asserting the absence of a migrate
  verb; a build-graph assertion that no `rk-*` crate depends on the `redkiln-migrate` crate."*
  `ac441_no_migrate_verb_in_tree` walks the tree for `migrate` (exact, any depth); the build-graph half is
  `redkiln-migrate`'s `ac466_ac441_…` (§7d.11) and the block's `cargo metadata` walk.
- **The `doctor` surface** (RB-M-06; **D18-16**) — `doctor_fix_flag_absent_from_snapshot` walks the tree for `--fix` and `--probe`
  (exact) and fails on either; the snapshot pins the same. CONDITIONAL rows do not ship flags.
- **ADR-0128's grammar** — `adr0128_run_lane_value_enum_closed`: `redkiln run x --lane hotfix` exits `2` (usage) and
  stderr names the two admissible values; `redkiln run --item x` exits `2` (no `--item`); `redkiln run x --lane fix`
  parses. **ADR-0129's grammar** — `adr0129_answer_grammar_positional_option_no_token_flag`: `answer x approve
  --token t` exits `2`; `answer x approve` without `--explain-sufficed`, and `answer x approve --explain-sufficed yes`
  without `--toml-opened`, each exit `2` (clap's required-flag error — the register's *"cannot be submitted with the field
  absent"* is a usage refusal, before any store read); `answer x approve --explain-sufficed maybe --toml-opened no` exits
  `2`; and the snapshot carries exactly these two sufficiency flags (ADR-0129 decision 1 fixes two — the operator's
  2026-09-19 answer; a parser that grew a third, or dropped one, is red here).
- **Common-dir discovery (H-3; D-18)** — every verb resolves the repository in H-3's order: a **bootstrap**
  `GitExec::new(tempdir.path().join("hooks-empty"), toolchain)` whose hooks directory is a `tempfile::TempDir` under the
  OS temp directory (`GitExec::new` creates its `hooks_empty` at construction, and the working hooks directory sits under
  a common dir not yet known — the first wrapper cannot be the working one, so `tempfile = "3"` is a **normal**
  dependency of both binaries, §7d.15); through it `GitExec::run(GitOp::RevParse, &WorktreePath::new(Path::new(cwd_arg))?,
  ["rev-parse", "--show-toplevel"])` for the working tree that holds `.redkiln/` (`WorktreePath::new` takes a `&Path`
  and returns a `Result` [measured 2026-09-18, `crates/rk-git/src/path.rs`]), where `cwd_arg` is `-C <path>` when given
  and the literal `"."` otherwise — the child git resolves `"."` against the **inherited** cwd, so no Rust code path names
  `std::env::current_dir` and `ci.yml`'s AC-529 rule is satisfied unchanged over `crates/rk-cli/src`; then
  `rk_adapter_sqlite::path::common_dir(&bootstrap, &worktree)` — the `pub` reader 7c shipped, which joins git's relative
  `--git-common-dir` answer onto the worktree it is handed [measured 2026-09-18, `crates/rk-adapter-sqlite/src/path.rs`]
  — never `rk-cli`'s own `rev-parse --git-common-dir`; then the working `GitExec` under `<git-common-dir>/redkiln/hooks-empty/`.
  **The `"."` is used for the one `--show-toplevel` read only.** The `WorktreePath` every later op receives —
  `SqliteStore::open`, `reconcile_at_entry`, `tree_hash`, `commit_with_trailers` — is `WorktreePath::new(Path::new(&toplevel))?`
  over the **absolute** `--show-toplevel` answer: `identity::opening` stamps `common_dir.display()` as joined and
  `rk_store::identity::check` compares the strings byte-wise [measured 2026-09-18, `crates/rk-adapter-sqlite/src/identity.rs`,
  `crates/rk-store/src/identity.rs`], so a `"."` handed on would stamp `./.git` at `init` and refuse a `status` from a
  subdirectory as another repository's store — an AC-413 false positive. Asserted two ways: the block's
  `rk_absent 'env::current_dir'` over both new crates with its control, and
  `common_dir_discovered_from_inherited_cwd_via_wrapper` — spawn `redkiln status` with `Command::current_dir` set to a
  **subdirectory** of a fixture repository and no `-C`, assert exit `0` and that the store it opened is the
  repository's (the identity row's `common_dir` equals the **absolute** `git rev-parse --path-format=absolute
  --git-common-dir` run from the root); a second arm spawns `redkiln -C <repo> status` from an unrelated temp cwd and
  asserts the same identity row — red by a `-C` the parser accepts and the discovery ignores; red by an
  implementation that treats cwd as the repository root (it fails to find `.redkiln/`), by one that stamps a relative
  path, and separately by one that reads `current_dir()` (the ban). Before any git op,
  `rk_git::version::assert_at_startup(cwd)` runs once (ADR-0091 decision 4's `>= 2.20` floor — the pre-`GitExec` read
  the unlabelled `rk_git::git_exec::run` performs); its refusal is a configuration error, exit `3`.
- **The trycmd files are the snapshot**; `cargo insta` is not used for them (`trycmd` 1.2.1 owns its `.trycmd`/
  `.stdout` pair). `trycmd = "1"` and `assert_cmd = "2"` are `>= 1.0` caret rows (ADR-0100 decision 2 pins 0.x only)
  — the plan's line reading `trycmd = "=0.15.x"` was measured wrong on 2026-09-18 (`cargo search trycmd` → 1.2.1) and
  is corrected here.

### 7d.4 Exit codes

The register's **§12.10** (*"Exit codes, and a platform error that returns success"*) belongs to the
acceptance-criteria register, not the design — the design has no §12, and the 153-line file said so after its own
predecessor sent the reader to the wrong document; it is written qualified everywhere in this file (`AC §12.10`, *the
register's §12.10*) because `check-refs.py` rejects the bare form.

- **AC-438** — BINDING (`rk-cli`, `rk-types`), as amended 2026-09-18 (D-01, D-12). *"GIVEN the closed `StepError`
  enum / WHEN `redkiln` terminates for any reason / THEN its exit code comes from a closed, snapshot-pinned table in
  which success, configuration error (including any failing error-severity `doctor` rule), findings-block and each
  runtime error class are distinct values; a new `StepError` variant with no mapping fails to compile."* *Verified by:*
  *"an exhaustive-match test over `StepError` → exit code; a committed snapshot of the exit-code table; a table-driven
  CLI test over fixtures × expected code, including the `doctor` fixtures of AC-415."* **The table** (H-1), as
  `pub enum Exit` with explicit discriminants in `crates/rk-cli/src/exit.rs` and `pub const ALL: [Exit; 18]` (**eighteen
  rows** — seven non-`StepError` rows including `0` Success, plus the eleven `StepError` rows; counted off the table
  below, Halt 7):

  | code | variant | warrant |
  | --- | --- | --- |
  | `0` | `Success` | success; **also a run that opens a gate** (D-01: AC-403 *"exits 0"*, AC-240, AC-024), **the AC-450-style no-op** (ADR-0107 decision 1: *"a no-op that says so"*) and **a `Void` answer** (recorded as void, the gate stays open — AC-406; H-1) |
  | `2` | `Usage` | clap's default, **kept**: an unknown argument (the AC-400 refusal), a missing required flag (ADR-0129's `--explain-sufficed` / `--toml-opened`), a closed-enum value outside its set (ADR-0128's lane), an option the gate's question does not declare (ADR-0129 decision 3: *"a usage error with zero events"*), an attachment no declared option admits or a declared attachment absent (AC-410; ADR-0129 decision 4; H-2) |
  | `3` | `ConfigError` | AC-438's *"configuration error (including any failing error-severity `doctor` rule)"*: AC-413's foreign store, AC-426's missing key, AC-486's adapter refusal, AC-531's naming refusal, ADR-0128 decision 2's non-derivable lane, D18-8's *"no process pack enabled"*, an unknown or dual-write key under `deny_unknown_fields` (AC-474), the git version floor, and an `answer` with no resolvable live token (AC-248) or a spent token (AC-407) — refusals of the repository's recorded state, read as configuration of the item (H-2) |
  | `4` | `FindingsBlock` | AC-438's *"findings-block"*: the review gate's block verdict surfaced by `run` (`review_block`, ADR-0124) — the value exists so the row is distinct; its producer is 7b-ii's kernel |
  | `5` | `NoRecord` | AC-432's *"an item with no recorded resolution is an error rather than a recomputation"* (D-12) |
  | `10` | `Transport` | `StepError::Transport { kind, detail }` |
  | `11` | `Schema` | `StepError::Schema { attempts, last_error }` (AC-439/AC-428's terminal class) |
  | `12` | `Timeout` | `StepError::Timeout { after_secs }` |
  | `13` | `Cancelled` | `StepError::Cancelled { by }` — **all three `CancelledBy` arms**, `Signal` included when the signal landed **during** a dispatch (§7d.10); the register's evidence line names *"SIGTERM → 143"* as an out-of-brief observation, and the table does not adopt it — 143 is the shell's convention for an unhandled SIGTERM, and this process handles it |
  | `14` | `NonZero` | `StepError::NonZero { code, stderr_digest }` |
  | `15` | `RateLimited` | `StepError::RateLimited { resets_at }` — after the durable retry mark (AC-404's exit half) |
  | `16` | `NotAdopted` | `StepError::NotAdopted { expected_agent, observed }` |
  | `17` | `NotIsolated` | `StepError::NotIsolated { surface, diff_digest }` |
  | `18` | `Platform` | `StepError::Platform { status, retry_after }` (AC-428's class) |
  | `19` | `Unsupported` | `StepError::Unsupported { capability }` |
  | `20` | `Overage` | `StepError::Overage { reason }` |
  | `70` | `PoisonedLockHalt` | ADR-0118 clause 6 (RB-M-01); H-2 |
  | `130` | `SignalOutsideDispatch` | a Ctrl-C / CTRL_BREAK / SIGTERM with no dispatch in flight (RA-M-04); H-2. 130 is the conventional Ctrl-C code and is distinct from every row above; it is a value in the snapshot, not an appeal to convention |

  `impl From<&StepError> for Exit` is **one `match` with eleven arms and no wildcard**. `ac438_step_error_mapping_is_exhaustive`
  is **not** a `trybuild` case — the exhaustiveness is the compiler's on the real `match` — it is a source assertion
  that the match body carries **no `_ =>` and no bare-identifier catch-all** (`other =>`, `e =>` — either would make a
  twelfth variant compile silently, exactly what the THEN forbids): the scan asserts each of the eleven
  `StepError::<Variant>` heads appears exactly once as an arm, and it is red by adding `_ => Exit::Transport` (or
  `other => Exit::Transport` in place of one arm) to the match; `ac438_exit_table_snapshot` is an `insta` snapshot of `Exit::ALL` rendered `code variant` per line
  (committed under `crates/rk-cli/tests/it/snapshots/`); `ac438_exit_codes_distinct` asserts the eighteen codes are
  pairwise distinct and that `0`, `2`, `3`, `4`, `5`, `70`, `130` are exactly the non-`StepError` rows (a table that
  grew a row without a snapshot update is red on the snapshot; one that collapsed two rows is red here);
  `ac438_table_driven_exit_codes_over_fixtures` (§7d.5) drives the binary over fixtures × expected code — the
  error-severity AC-415 doctor fixtures at `3` and the warning/ok-severity ones (`RK-D-DEAD-DEF`, `RK-D-UNFILLED-SLOT`,
  `RK-D-UNREAD`, `RK-D-HEADROOM-MARGIN`, `RK-D-REBUILT-TIER2`) at `0` (*"warnings never move the
  code"*, §7d.8), the `--resume` fixture at `2`, the no-record fixture at `5`, the no-pack repository at `3`, the
  gate-open fixture at `0`; **the `70` row is exercised in-process only** — no fixture can poison a spawned process's
  lock without the seam AC-474 forbids, so §7d.10's `adr0118_…` is its instrument and the table-driven test asserts the
  row's presence in `Exit::ALL`; the eleven `StepError` rows are driven **in-process** through the verb function with a
  `FakeHarness::scripted` returning each variant (`rk_ports::fakes::FakeHarness` is a library value no spawned process
  can be handed), and the binary is driven for the classes the fixture child can produce through the real adapter —
  `Transport` via a missing `harness_binary`, `Schema` via a malformed frame — ***corrected 2026-09-20, settling residual R-60: a malformed frame does NOT yield `Schema`.*** *Measured through the binary, `init` plus a malformed line exits **`10`**, not `11`: `classify.rs` rule 8 takes a stream with no parseable result frame to `Transport { kind: Decode }`, and `extract_digest` is rule 10. The `Schema` (`11`) fixture is authored as **the other arm of `extract_digest`** instead, which is where an `11` is actually reachable* — a `hold` child under a 1 s
  `timeout_secs` → **`13`** (the adapter classifies its timer arm `Cancelled { by: Timeout }` — `Arm::Timer =>
  Some(CancelledBy::Timeout)` [measured 2026-09-18, `crates/rk-adapter-claude/src/runner.rs`]; `s7c` §7c.10, RT-3 — so
  this fixture is the binary-level instance of row `13`, not `12`, and a fixture whose `expected` reads `12` is red on a
  correct binary), `Platform` via an `is_error` frame; **`Timeout` (row `12`) is reached in-process only** — nothing in
  `rk-adapter-claude` produces `StepError::Timeout` at `239e899` — as are `NotAdopted`, `NotIsolated`, `Unsupported`,
  `Overage`, `RateLimited` and the other two `Cancelled` arms, and the bullet in §7d.14 names which; `ac438_doctor_error_rule_exits_config_error`
  (§7d.8) is the doctor-row instance by name.
- **AC-439 and AC-428** — **discharged at 7c** under their §0.3 owners AC-206 and AC-218 (`s7c` §7c.22 CROSS-LISTED:
  *"the card-validator / adapter halves are here (`structured_output_required`, `ac218_…`, `ac206_…`), the `doctor`
  rows and the stream-corpus replay conjuncts are 7d's or RAISED"*). Nothing for 7d to build: the class each names
  (`Schema`, `Platform`) has its row above, and the *"replay assertion across the recorded stream corpus"* conjunct is
  unsatisfiable at the pin (no stream corpus exists) — `s7c` RAISED the register amendment restating it over
  `step.result_raw` frames, and this file **does not claim either id** (§7d.18 Excluded). This is the matching note
  `s7c` §7c.22 said was OWED to this file for AC-428 and AC-439.
- **The one thing AC §12.10 is named for** — *"a platform error that returns success"* — is Halt 3 and is not a CLI
  behaviour: the adapter classifies (7c), the kernel records (7b), and `redkiln run` exits through row `11` or `18`
  **never through `0`**; `ac438_table_driven_exit_codes_over_fixtures` carries one fixture whose scripted harness
  returns `Ok`-shaped exit-0 bytes with `structured_output` absent and asserts `11`, which is the CLI's end of F20.
- **ADR-0118's row** (RB-M-01). Clause 1: *"A poisoned store lock halts the run. The run stops at the poisoned
  acquisition. No `StepError` is constructed, nothing crosses a port boundary, no retry, no next activity, no gate
  answer."* Clause 4 makes both locks — `Arc<Mutex<dyn EventStore>>` and `rk-store`'s writer mutex — the rule's
  scope. Clause 5: *"The acquisition is the composition root's"*; so `rk-cli` acquires through one function,
  `store::acquire(&Arc<Mutex<dyn EventStore>>) -> Result<MutexGuard<'_, dyn EventStore>, Halt>`, whose `Err(_)` arm
  (the poisoned acquisition — the type is matched, never recovered) returns `Err(Halt::PoisonedLock { lock, verb })`;
  **every verb returns `Result<Exit, Halt>`**, and **`std::process::exit` has exactly two call sites, both in `main.rs`**:
  `std::process::exit(code_for(&result))` at the end of `main` — which renders an `Err(Halt::PoisonedLock)` through
  `render_halt` to stderr (`redkiln: halt — the store lock is poisoned (store): a writer panicked while holding it;
  nothing after <verb> ran.`) and maps it to `70` (a `std::sync::PoisonError` carries only the guard — `into_inner` /
  `get_ref` — and no panic payload, so none is printed) — and the panic hook's `std::process::exit(70)` below. **The
  table exit and the halt arm are ONE call through `code_for`, never two** — an implementer who writes a third
  `process::exit` for the halt arm fails the block's count (§7d.17) on a correct binary. The halt is **an explicit exit
  after the message, never the unhandled panic's 101** (clause 6's third question,
  answered). Because the kernel acquires the same `Arc<Mutex>` itself (`Runtime::store_guard`'s
  `.lock().expect(LOCK_HALTS)`, `crates/rk-kernel/src/runtime.rs` [measured 2026-09-18]), a poison first observed inside
  `replay::resume` is a panic that unwinds out of the kernel: `main.rs` installs one `std::panic::set_hook` whose hook,
  when the payload carries the kernel's `LOCK_HALTS` text (*"a poisoned store lock halts the run"* — `const LOCK_HALTS`
  is private in `runtime.rs` at `239e899` [measured 2026-09-18] and becomes `pub const` at Slice 2, a visibility change
  and the eighth kernel edit, so the hook matches `rk_kernel::runtime::LOCK_HALTS` and not a second copy of the string;
  the `.expect` payload is the `String` `"{LOCK_HALTS}: {PoisonError:?}"`, so `is_lock_halt` downcasts to `String` /
  `&str` and substring-matches), prints the same
  diagnostic and calls `std::process::exit(70)` — the hook is the second of the two `process::exit` sites (the first is
  `code_for`'s at the end of `main`), it is not `catch_unwind` and it recovers nothing (clause 3); any other panic keeps
  Rust's default 101. Every `lock()` on a `Mutex` in `rk-cli/src` goes
  through `store::acquire` (the block's `rk_absent '\.lock\(\)'` over `crates/rk-cli/src` with the module-path
  exclusion of the one acquiring module **and of the stdio handles** `io::stdout().lock()` / `io::stderr().lock()`, which
  are not Mutexes and are how a verb prints a table without interleaving — ADR-0118 clause 4's scope is the two store
  locks, and a ban scoped wider than its warrant blocks valid work, golden constraint 9; control in `lint-controls/`), and the block counts the `process::exit(` sites
  at exactly two, both under `main.rs`. No `PoisonError::into_inner`, `catch_unwind`, `unwrap_or_else(… into_inner)`, `map_err(… into_inner)` or
  `lock()…into_inner()` appears in either new crate (clause 3; the same ban `s7c` §7c.21 ran, re-scoped — naming the
  type in the halting arm is not recovery, so the ban is on the recovery spellings, not on the identifier, and
  `rk_git::reconciled::Reconciled<T>::into_inner` — the one way to take AC-172's `Reconciled<State>` out of its
  wrapper after `reconcile_at_entry` [measured 2026-09-18, `crates/rk-git/src/reconciled.rs`] — is **not** matched,
  because a pattern that fired on every `.into_inner()` would fail a correct binary or steer it away from the wrapper).
  `rk-store`'s `conn.rs` recovery is **RAISED, not repaired** (RT-9 / R-7 stand as 7c left them). The discriminating
  test is §7d.10's `adr0118_poisoned_store_lock_halts_run_exit_70`.
- **The signal rows** are §7d.10's. **Gate-open at `0`** is §7d.5's `ac403_…`, whose exit assertion is `0` and would
  have been unauthorable before D-01.
### 7d.5 `run`

**The order of a `run`, stated once so every test below reads against it** (`crates/rk-cli/src/verbs/run.rs`):
(1) `rk_git::version::assert_at_startup`; (2) the common dir (H-3) and `.redkiln/config.toml` read into `ConfigFile`
(`deny_unknown_fields`; an unknown key, a missing `[doctor] unread_window_runs`, or any dual-write-shaped key is exit
`3`); (3) `Env::capture()` — **the one environment read**, already performed in `main.rs` and handed in as a value;
(4) `AdapterSet::resolve(&config_file, &env)` — names against the compiled-in four and the `[adapters]` keys **only**:
`harness_binary` from the optional `[adapters] harness_binary` key (default `"claude"`, the adapter's own; on the AC-474
key snapshot; §7d.1's `ConfigFile` row, RT-49), `toolchain_programs` likewise, and the denylist read through
`rk_adapter_claude::env_denylist::first_denylisted` over the captured map (AC-422/AC-488's one list) — **every
pack-sourced value is read at (7) and applied at (8)**, so an init-only repository reaches (7)'s refusal and never a (4)
refusal; `resolve` returns names and constructor values only and **constructs no port implementor** (`Resolved` holds no
`Arc<dyn …>` — the real `Harness` is built at (8), never inside `resolve`, which is what keeps the `doctor`-module lint
of §7d.8 meaningful one call away); a refusal here is exit `3` **before any process is spawned and before any store
write** (AC-486) — zero rows; (5) `SqliteStore::open` (AC-413's identity check; a foreign store is exit `3`, zero rows);
(6) `reconcile_at_entry(exec, repo, Entry::Run)` (AC-402) and the invocation's one `reconcile_started` row carrying
`adapter_set` and `config_digest` (RT-41) and, once the run-owned worktree is re-created at (8), the item's
`RunOwnership { worktree, branch }` (RT-47; H-5) — the row is written once (8) has re-created the worktree, with all
four members, or at (7)'s refusal without the pair (no worktree was created), and is in either case the invocation's
first and only `reconcile_started` row (RT-46); (7) the pack set: an empty `[packs] enabled` is exit `3` with *"no process pack enabled"*
(**D18-8**) **before any lane derivation** — so the refusal is the same whatever item was named; then the lane from
`--lane` or the item's kind (ADR-0128 decision 2), the pack from the lane (decision 3) read from
`.redkiln/packs/<pack>.toml`, the catalog from `.redkiln/catalog/*.toml`, `.redkiln/artifacts.toml` and
`.redkiln/facets.toml` (`s7e` §7e.1's layout; E7 — `with_pack` / `with_catalog` / `with_artifacts` / `with_facets` all
take source strings, and the paths are stated once here so two implementers read the same files), the item record from
`.redkiln/items/<item>.toml` (§7d.1's lane row, RT-50) — its top-level `kind` is `Item::new(id, kind)`'s, its top-level
**`stage`** is the `stage: &str` every kernel entry below takes (`RunInputs::new(stage)`, `fsm::fold`, `fsm::stage_action`,
`RunState::initial` — read once here and nowhere else; absent → exit `3` naming the item, *"no stage on the item
record"*; RT-50), and its
`[facets]` table populates the `Item` through `with_facet_str` / `with_facet_bool` / `with_facet_ordinal` /
`with_facet_set` (one value per facet the registry declares; **an absent facet is `with_facet_null`**, which is how a
`require` row abstains and a gate opens — AC-079 / ADR-0089; no fact is computed at 7d — AC-425's computed facts are
7e's registry — and the fixtures spell every facet they need; the key set is an existence floor 7e's item schema may
re-point) [measured 2026-09-18, `crates/rk-kernel/src/select/item.rs`: the builder is the only way facets reach an
`Item`] — and the pack's membership in the enabled set (a lane whose pack is not enabled is exit `3` naming the pack).
**The adapter-facing pack values** — `margin_ticks`, the two TTL keys, `gate_deadline_secs`, `timeout_secs` per
activity, the gate-class expiry action — **are read by `rk-cli`'s own `PackExtras`** (serde over the SAME
`.redkiln/packs/<pack>.toml` bytes, `deny_unknown_fields` OFF because the kernel owns the rest of the file), stated here
so two implementers read the same keys: `rk_kernel::select::Pack` exposes none of them (its accessors are `rigor`,
`require_attempt_cap`, `require_token_appetite`, `require_breaker_threshold` / `_expiry`, `family_for_role`,
`stage_specs`, `policy` [measured 2026-09-18, `crates/rk-kernel/src/select/pack.rs`]) and the kernel's `toml_subset`
walker ignores unknown keys, so the file may carry them and something must parse them; the kernel's `Pack` is **not**
widened (no 7b re-open for this), and a record fixing these keys in the pack schema is 7e's (RT-57; §7d.18 RAISED); (8) `Config`
constructed (§7d.1's `rk-kernel` type); the run-owned worktree re-created (H-5 / RT-47: `<common-dir>/redkiln/worktrees/<item>/`
on `rk-run/<item>`, the migrator's first-run shape — a previous run's tree torn down first with `["worktree", "remove",
"--force", path]` through `GitExec::run(GitOp::WorktreeRemove, …)`, because `rk_git::worktree::remove` passes no
`--force` and git refuses a tree a killed run left files in [measured 2026-09-18, rc=128]; licensed by ADR-0105 clause 6
— run-owned, re-created never reused — and by nothing else); the real `Harness` built with the pack's margins and TTLs
(`rk_adapter_claude::config::Config::new` — `Config` is **not** re-exported at the crate root [measured 2026-09-18,
`crates/rk-adapter-claude/src/lib.rs`: `use crate::config::{Config, ConfigError}`, no `pub use`] → **the binding step,
in `adapters.rs`** (RT-55): `let mut binder = Binder::new(config)`, and for every DAG node the run will dispatch
`binder.bind(BindRequest { role_name, role: &Role { model, effort } (the pack's role table, through `PackExtras`), card:
&AuthoredCard (the pack's `cards/` file — the same file `doctor`'s `roles.rs` reader reads), schema: &Schema,
timeout_secs: <the activity's PackExtras value>, worktree: &owned_worktree, inputs })? -> ActivitySpec`, then `inputs =
inputs.with_spec(node_id, RecordedSpec::supplied(activity_id_at_version, spec))` — **only then**
`ClaudeHarness::new(binder.into_config(), token)`, `on_version_mismatch = Refuse` (`ClaudeHarness::dispatch` refuses a
`card_digest` its validated store does not hold — *"names no card in the validated store"*, a `Transport` — and
`Binder::new(config).bind(..)` then `binder.into_config()` is the one way the store is filled [measured 2026-09-18,
`crates/rk-adapter-claude/src/binder.rs`, `lib.rs`]; `Binder` and `BindRequest` are named in `adapters.rs`, outside
the `doctor` module the AC-414 lint filters to, so the lint's set needs no widening), the two toolchain reals with `program` from
`toolchain_programs`, `timeout` from the pack's `timeout_secs`, `project_root` = **the run-owned worktree**, `scratch` =
`<common-dir>/redkiln/scratch/<name>/`, `child_env` = the captured map (`rk_adapter_toolchain_*::Config::new` takes
exactly those five [measured 2026-09-18]), `SqliteStore` as the store, `BoardTracker` / `RecordingDocs` for the two
ports with no Phase-1 real (ADR-0094 decision 2); (9) **before the gate read**, every open gate of the item whose
recorded deadline `< now` is resolved through `rk_kernel::workflow::gate::expire(&state, token,
Timestamp::from_unix_seconds(now), &action, &tree_hash)` and its events appended, where `action` is the pack's declared
expiry row for the gate class read through `PackExtras` and linted by `gate::lint_expiry_policy_row` (AC-260) — **no
default** (`gate::answer` returns `Err(GateError::Expired)` only when a `gate_expired` row is already in the journal — ***corrected 2026-09-20, R-44: unreachable for a real expiry; the record is KEPT in `gates_open` and `gate_record` is tested first, so the CLI filters expired tokens itself. The full correction is at §7d.6.***;
a gate whose deadline merely passed is still open to it [measured 2026-09-18, `crates/rk-kernel/src/workflow/gate.rs`],
so *resolved at that read* — AC-257 — is this call); then
`RunInputs::new(stage).with_pack(..).with_catalog(..).with_artifacts(..).with_facets(..)
.with_item(&item).with_runtime(RuntimeContext::supplied(&tree_hash, deadline).results_at(now)).with_config(&config)` (plus the
`with_spec` bindings of step (8)) — ***`.results_at(now)` added by dated amendment 2026-09-20, settling residual R-39.***
*This chain read `.with_runtime(RuntimeContext::supplied(&tree_hash, deadline))` from the stage-open rewrite of
2026-09-18 until then, and `results_at` appeared **nowhere in this file** (`grep -c results_at
runbook/phase-1/s7d-cli-and-migrate.md` → **0**, measured 2026-09-19 at Slice 3 cycle 1's close and again
2026-09-20). The chain as prescribed **refuses every dispatching run**: `RuntimeContext::supplied` leaves
`results_at` `None` [`crates/rk-kernel/src/replay.rs`:535, :548], `derive`'s dispatch envelope and `resume` both call
`require_results_at()` [:1264, :1544], and that call **refuses rather than defaulting** — *"no `results_at` was
recorded, and the instant an `activity_committed` carries"* [:608-612] — because `rk_types::time::Timestamp` has no
`now()` and **a zero `at` is a commit stamped at the epoch** (`replay.rs`'s header, `s7b` §7b.11's 4i clause). So a root
following this chain verbatim would have failed on `inputs/runtime.json` **before any dispatch** — the WRONG REASON
for four of cycle 5's five tests, which is the failure golden constraint 12 exists to prevent. **The kernel side was
never the defect and is unchanged**: the builder, the refusal and its two call sites all predate 7d and are `s7b` §7b.11's.
**Only this file was wrong, and only by omission.** The `rk-cli` half landed at Slice 3 cycle 5 (`41d8a03`) in
`base_inputs`, chained off **the ONE wall-clock read** the paragraph above fixes — the same `now` that makes
`deadline`, never a second read, since two reads would let a run's own deadline and its recorded commit instant
disagree by however long step (8) took. `crates/rk-cli/src/verbs/run.rs`:999-1006 carries the 🛑 note at the site.* →
**`rk_kernel::replay::resume(&inputs, ports, &prior, &mut supply_spec)`** — the sixth kernel edit (§7d.1, §7d.15), where
`supply_spec: &mut dyn FnMut(&str) -> Result<RecordedSpec, ReplayError>` is the spec-supplier seam consulted where
`record()` calls `spec_for` (RT-55: node ids are known only after `select()` runs inside the loop, so a node bound at
(8) is served from `inputs`, and a node first seen inside the loop is bound through the same `Binder` the closure
holds; the fake-harness tests hand a supplier over `RecordedSpec::supplied(..)` values — the fake ignores `card_digest`,
`spec_for` does not); `prior` is the
store's journal for the item, so the fold `next_action` reads is the recorded one and a second `run` resumes past an
applied answer instead of re-deriving `OpenGate` at reach 1 (`record()` folds a **fresh local vector** each call and
**breaks** on `NextAction::OpenGate` without appending anything — its recorded events carry only `select()`'s
announcement rows, which the fold counts as `gates_announced` and moves no state on [measured 2026-09-18,
`crates/rk-kernel/src/replay.rs`, `crates/rk-kernel/src/workflow/fsm.rs`]; `resume` steps `OpenGate(record)` through
`Runtime::step`, which appends `runtime::gate_opened_event(record)` — the `(kind, token, tree_hash, deadline)` row
`ac403_…` reads — before returning; `record()` itself is unchanged and stays the recording seam) — where `deadline`
(`i64`, unix seconds) is `now + the pack's gate_deadline_secs` and `now` is **the one wall-clock read**,
`src/clock.rs`'s `Clock::now()` under a single `#[expect(clippy::disallowed_methods, reason = "AC-001: the composition
root reads the clock once; the kernel is clock-free")]` (the ban is `clippy.toml`'s AC-001 row; ADR-0102 decision 3 for
the `#[expect]` form — ADR-0083 has no clock clause) — the kernel *"reads it and never makes one"* [measured
2026-09-18, `crates/rk-kernel/src/replay.rs`], and the same value is what `answer` and `run` compare a gate's recorded
deadline against (AC-248 / AC-257's expiry read); the block bans `SystemTime::now|Instant::now|Utc::now` outside that
module with a control (§7d.17); (10) the terminal read: a `gate_opened` among the resumed run's new events → **the
`Gate:` trailer commit is a TIER-1 WRITE** (H-5) and goes through `rk_git::worktree::authorise_tier1_write(exec,
&owned_path, &ownership)` (ADR-0105 clause 8; AC-529; AC-518 — **never `cwd`**), where `ownership` is the
`RunOwnership { worktree, branch }` recorded at (6), then `commit_with_trailers(exec, owned.path(), msg, &[Trailer { key:
TrailerKey::Gate, value: format!("{token} {tree_hash} {deadline_secs}") }], allow_empty = true)` (`OwnedWorktree.path`
is a private field with a `pub const fn path(&self) -> &WorktreePath` accessor [measured 2026-09-18,
`crates/rk-git/src/worktree.rs`] — the private field IS the mechanism: *was this write checked?* is answered by the
type) with `deadline_secs = record.deadline().unix_seconds()` — the integer the `gate_opened` row carries. **Design §5.4's
`Gate:` row says `deadline` RFC 3339**, and `chrono = "^0.4"` IS a workspace row (ADR-0100 decision 2's exempt line
[measured 2026-09-18, root `Cargo.toml`]), so the format is the design's to fix, not a manifest constraint: this file
writes the integer so the trailer and the event agree on one value, and **the design §5.4 row's format takes a dated
note — OWED, RAISED in §7d.18** (design toward the built record, as its other §5 notes did; ADR-0092 clause 2.5); the
trailer grammar itself accepts any whitespace-free token [measured 2026-09-18,
`crates/rk-git/src/trailer/grammar.rs`: three `split_whitespace` tokens, a 40-hex middle] — `rk_git::trailer::grammar::Trailer`
is a plain `{ key, value }` struct with no constructor [measured 2026-09-18], and the triple is the one `Gate:` value the
grammar parses — then exit `0`; a `RateLimited` → the mark is the kernel's row, exit `15`; a `StepError` → its row;
`Ok` → `0`. **`run` never waits**: no channel, prompt, socket or sleep is entered on any arm (golden constraint 2).
**The fixture checkout is never written**: every commit `run` makes lands in the run-owned worktree on `rk-run/<item>`,
and `HEAD`, `git status --porcelain` and `git diff-index --quiet HEAD` of the checkout the binary was spawned in are
unchanged across every `run` in this file (AC-518).

- **AC-402 (three paths)** — BINDING (`rk-cli`, `rk-git`), as amended 2026-09-12 and noted 2026-09-18. *"GIVEN any
  entry point that can cause an action (`run`, `answer`, `doctor`) — `status` and `explain` are read-only and do not
  reconcile (ADR-0105 clause 8; AC-172) / WHEN it begins / THEN `reconcile(recorded, world)` has run before any
  decision is taken, and no CLI flag can skip it."* *Verified by:* *"a kernel test enumerating the three entry paths
  and asserting `reconcile` is called on each, and a negative case asserting `status` and `explain` do not call it."*
  `ac402_reconcile_called_on_run_answer_doctor` drives the three verbs in-process through a CLI-level seam — the verbs
  take `&dyn EntryReconciler` (one impl delegating to `rk_git::reconciled::reconcile_at_entry`; the test's impl records
  the call and the `Entry` value) and a counting `EventStore` fake whose first `query` / `append` records whether the
  reconcile call preceded it — asserting exactly one reconcile call with the right `Entry` **before** the first store
  read on each verb; `ac402_status_and_explain_do_not_reconcile` asserts zero on both. **`GitExec` has no journal**
  (`rk_git::GitExec` is `{ hooks_empty, toolchain }` and `run()` shells out directly; the `GitJournal` lives on
  `gitops::GitOps`, the kernel's replay seam [measured 2026-09-18, `crates/rk-git/src/git_exec.rs`]) — every git-side
  assertion in this file is a git-side fact (`rev-list --count`, `for-each-ref`, the `.git` fingerprint) or a CLI seam,
  never *the journal*. `rk_git::reconciled::Entry` has exactly `Run`,
  `Answer`, `Doctor` [measured 2026-09-18, `crates/rk-git/src/reconciled.rs`] — the type is the enumeration.
- **AC-401** — BINDING (`rk-cli`, `rk-kernel`). *"GIVEN an item whose last activity committed a step and whose world
  has not changed / WHEN `redkiln run <item>` is invoked a second time / THEN no dispatch is issued for the memoised
  committed step and its recorded output digest is unchanged."* *Verified by:* *"a kernel test over a fake
  `HarnessPort` that counts dispatches and fails on a second dispatch for a memo key already present."* The kernel's
  instrument is 7b's; **7d adds the wiring**: `ac401_second_run_issues_no_dispatch_for_memoised_step` runs the verb
  twice against `pack-min/` with `FakeHarness::scripted` and asserts, after the second run, **exactly one `dispatch`
  row in the store** (the fake has no counter [measured 2026-09-18, `crates/rk-ports/src/fakes.rs`: fields `script`,
  `token`; a second dispatch would also exhaust the one-element script into `Transport { detail: "script exhausted" }`),
  that the second run's `next_action` never returned `Dispatch` for the committed node (the store-seeded fold's
  `is_committed` — `replay::resume`, step (9)), and that the recorded `activity_committed` digest is unchanged. **No
  `MemoHit` row is asserted**: `EventKind::MemoHit` is written only by the `workflow::memo` projection and by no
  runtime [measured 2026-09-18, `crates/rk-kernel/src/workflow/memo.rs`, `runtime.rs`], and a second `record()` over a
  committed activation would have failed with `ReplayError::NotCommitted` (`run_activity` returns early with `commit:
  None` on a committed state and `dispatch_through` refuses it) — which is why the resume entry exists. SM-130's 0-gap arm.
- **AC-403** — BINDING (`rk-cli`, `rk-kernel`), as amended 2026-09-18 (D-15). *"GIVEN an activity that reaches a human
  gate / WHEN `redkiln run <item>` opens that gate / THEN the `Gate:` trailer and `gate_opened` event are durably
  written and the process exits 0 without waiting on any channel, prompt, socket or sleep."* *Verified by:* *"an
  integration test that runs `run` to a gate under a fake clock, asserting the trailer and event are on disk and that
  the process exits with no live handle held; plus a lint forbidding blocking stdin reads in `rk-cli`."*
  `ac403_gate_open_writes_trailer_and_event_and_exits_zero_without_waiting` spawns the real binary against a fixture
  repository whose pack carries **one `require` row reading a facet the item record leaves null**
  (`Item::with_facet_null`), so `select()` opens an **abstention gate** (`GateReason::UnknownFacet`, `fsm::next_action`
  step 2 — at the pin a gate is opened ONLY by iterating `selection.gates()`; a `StepKind::Gate` step inside an activity
  opens nothing, the loader maps it to `Evidence::HumanAnswer` and the runtime dispatches the whole activity to the
  harness [measured 2026-09-18, `crates/rk-kernel/src/workflow/fsm.rs`, `select/step2.rs`, `catalog.rs`]), under a
  wall-clock bound of 30 s (Halt 5's alternative — the kill bound) ~~**and an assertion that the binary exited within
  5 s** (the gate-only pack dispatches nothing, so a `sleep`, a channel wait or a socket poll of any human-visible
  length is red; the 30 s figure is the kill, the 5 s figure is the assertion — `ac404_…`'s 2 s bound is the same
  shape)~~ *(dated 2026-09-30, s8g build, package CI-AC403-8g.)* The 5 s figure failed on a loaded box with the binary unchanged: 6.2 s, 6.4 s and 9.4 s in Checkpoint 6's three whole runs at 4/4 threads (`s6`'s measurement note). The test alone took 5.03 s on 2026-09-30. The kill bound's expiry is now the assertion. The kill bound is `SPAWN_BOUND` (30 s), taken through `try_output_within`, under an asserted premise `10 × 30 s <= gate_deadline_secs` (900 s, read from the installed pack). So a run that waits for the answer, or waits out the deadline, is red. The `deadline` pin below, which the test had not carried, is now asserted. Stated loss: a BOUNDED wait under 30 s is no longer red at this test. A sleep is held by `clippy.toml`'s `disallowed-methods` rows. A timed channel or socket poll has no instrument (mutant M5, the test's doc comment). — the register's *fake clock* is not injectable into a spawned binary without a toggle AC-474 bars, so the
  bound replaces it and the deadline is asserted **structurally against the test's own clock reads around the spawn**:
  `t_before + gate_deadline_secs <= deadline <= t_after + gate_deadline_secs`, both reads test-side under the
  `#[expect(clippy::disallowed_methods, …)]` §7d.15 permits, so a `deadline` computed from anything but `now + the
  pack's gate_deadline_secs` is red (no event carries an `opened_at`: `rk_types::Event` is `{ kind, payload }` with no
  timestamp and the `gate_opened` payload is `(kind, reach, token, tree_hash, deadline)` [measured 2026-09-18,
  `crates/rk-types/src/event.rs`, `crates/rk-kernel/src/runtime.rs`]; the fake-clock substitution is RAISED for a
  dated Verified-by note, §7d.18), and asserts:
  exit **`0`** (D-01); `git -C <run-owned worktree> log -1 --format=%B` (the worktree and branch the store records for the
  item — H-5) carries a `Gate:` trailer whose value parses under `TrailerKey::Gate`'s grammar, and **the fixture
  checkout's own `HEAD`, `git status --porcelain` and `git diff-index --quiet HEAD` are unchanged across the run**
  (AC-518); the store's last row is `gate_opened` with `(kind, token, tree_hash, deadline)`; and **no live handle** — the
  process is reaped (`try_wait` returns `Some`) and the enumeration of its job's descendants is empty — the test spawns
  the binary through `rk_containment::spawn_into_job` (`rk-cli` depends on the crate) and reads `Group::pids()` after the
  exit, since a `std::process::Command` child has no job to enumerate; the *no live handle* clause is asserted **as a
  floor only** — the gate-only pack spawns no child, so the empty set proves nothing about containment (§7d.12's AC-278
  row) and the register's clause is not claimed on it. The lint half is
  `ac403_no_blocking_stdin_read_in_rk_cli` — a source scan of `crates/rk-cli/src` for `stdin()` and `io::stdin` with
  the comment filter (not `read_line(`, which is `BufRead::read_line` on any reader — `RK-D-HOOKS` reads hook files
  line-wise — golden constraint 9), and the block's `rk_absent` with its control. AC-240's *"exits 0 within a
  bounded wall-clock budget"* is the same assertion under its twin's name (register §0.3 pairs AC-240 with AC-278;
  AC-278 is ARMED, §7d.12).
- **AC-404 (exit half)** — kernel-built at 7b (owner AC-030's `retry_at(resets_at)` mark under the fake clock);
  7d adds **only the exit**: `ac404_rate_limited_writes_retry_mark_and_exits` drives the verb function **in-process**
  with a `Ports` whose scripted harness returns `Err(RateLimited { resets_at: Some(now + 3600) })` — a value well
  beyond the 2 s bound, so an implementation that sleeps to `resets_at` is red on the clock (`FakeHarness` is a library
  value no spawned process can be handed, §7d.4) — and asserts the `rate_limited` row with the mark, the returned
  `Exit::RateLimited` (`15`), and a wall clock under 2 s (the register's *"no sleep, timer or in-process wait is
  entered"*). The kernel half is not re-asserted under a new name.
- **AC-485** — BINDING (`rk-cli`, `rk-kernel`). *"GIVEN a run whose adapter set was resolved at startup / WHEN
  `.redkiln/config.toml` is edited mid-run to name a different adapter / THEN the running run's adapter set does not
  change, and the adapter set recorded on the run-start event is the one used for every subsequent dispatch in that
  run."* *Verified by:* *"an integration test that mutates the config file between two activities and asserts adapter
  identity on both dispatch events; a replay assertion that the adapter set is a run-scoped constant in the event
  log."* `ac485_adapter_set_resolved_once_and_run_scoped`: a two-activity pack; a test-local `Harness` impl
  (`tests/it/support.rs::MutatingHarness`) whose second `dispatch` rewrites `config.toml`'s `harness` to
  `rk-adapter-linear` before delegating to a `FakeHarness::scripted` (`rk_ports::fakes::FakeHarness` replays a
  `Vec<Result<Typed, StepError>>` and can run no side effect between dispatches [measured 2026-09-18,
  `crates/rk-ports/src/fakes.rs`]); **the discriminating assertions are the seams** — the `ConfigFile` reader is
  called exactly once and `AdapterSet::resolve` exactly once (counting seams the verb function takes), both dispatches
  went through the one `Harness` object handed in (the impl counts its own calls), and the first row of the run —
  `reconcile_started` (RT-41) — carries `adapter_set` equal to the resolved four and `config_digest`; replaying the log
  through `rk_kernel::replay` reads the same constant. The register's *"adapter identity on both dispatch events"* has
  **no `DispatchRecord` member to read** [measured 2026-09-18, `crates/rk-types/src/activity.rs`: `argv_digest`,
  `env_digest`, `instrument_version`, `stdout`, `stderr`, `usage`, `timings`, `total_cost_usd`, the headroom pair,
  `cli_version`, `api_retry_events`, the two TTLs — no adapter identity] and is met through the routed carrier, which is
  why AC-485 sits in §7d.18's PROVISIONAL sub-list. Red by a second `AdapterSet::resolve` call anywhere after step (4)
  or a second `ConfigFile` read.
- **AC-486** — BINDING (`rk-cli` (`doctor`), `rk-kernel`). *"GIVEN `.redkiln/config.toml` names an adapter that is
  not compiled in, or names one whose required configuration is absent / WHEN the run starts / THEN the run is
  refused as a configuration error before any process is spawned and before any store write AND `redkiln doctor`
  reports the same defect at configuration time at zero model cost."* *Verified by:* *"a startup-resolution unit
  test; a `doctor` rule-table row; a fixture config naming `rk-adapter-linear` that must fail."*
  `ac486_misconfigured_adapter_refuses_before_spawn_or_store_write` over `tests/fixtures/config/unknown-adapter.toml`
  (`harness = "rk-adapter-linear"`): exit `3`, stderr naming `rk-adapter-linear` (the fixture's `[packs] enabled = []`
  means a warn-and-continue implementation also exits `3` at step (7) with *no process pack enabled* — the message and
  the store-file absence are what tell the two apart), the harness fixture child's marker count 0 (`s7c` §7c.17's
  file-count shape, reused through `rk-adapter-claude`'s `rk-fixture-child-claude`), the store file's byte length unchanged (a
  `SqliteStore::open` that ran would have created it — the fixture repository is prepared **without** a store so the
  absence is observable); **a second arm over `tests/fixtures/config/missing-store.toml`** (the scaffold with the
  `store =` line removed) exits `3` naming the absent key with the same zero-spawn / no-store assertions — the GIVEN's
  second disjunct, *"or names one whose required configuration is absent"*, which a resolver that never checks required
  configuration would pass without it. **The test has two arms by slice**: at Slice 2 its **in-process arm** —
  `AdapterSet::resolve(&unknown_adapter_config, &clean_env)` returns `Err(Refusal)` naming `rk-adapter-linear`, zero
  marker files, no store file — and at Slice 3, when `run` exists, the **spawned arm** above (in Slice 2 `run` is Slice
  1's stub and would exit `3` for the stub's reason, §7d.16). The `doctor` row is `RK-D-ADAPTER` (§7d.8). The positive control
  `ac486_init_scaffold_resolves_four_reals` resolves the §7d.9 scaffold and asserts four names and zero refusals —
  the bare-`init` repository is runnable (D-14's ground) — under a **test-constructed clean `Env`**: no test calls
  `Env::capture()`; the captured-map inputs of every `resolve` and every `doctor` `Ctx` are values the test builds, so
  this box's persisted `ANTHROPIC_FOUNDRY_API_KEY` (the gate check) cannot turn a scaffold test red for the denylist's
  reason (golden constraint 9).
- **Config construction (ADR-0083; RB-M-05)** — `adr0083_config_constructed_once_at_root_and_threaded`: the
  `rk_kernel::Config` value the run carries has the digest the first row recorded; `RunInputs::with_config` is called
  exactly once; and no `std::env::var` is reached outside `crates/rk-cli/src/env.rs` (a source scan; the block's grep
  is the floor). The kernel-side test is `-p rk-kernel`'s `adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs`.
- **The lane (ADR-0128)** — `adr0128_lane_derived_from_item_or_refused`: an item fixture carrying `kind = "bug"` runs
  under `bug-standard` without `--lane`; one carrying `kind = "story"` under `story`; one carrying no kind and no
  `--lane` exits `3` naming the item and *fix, story*; `--lane story` on the `bug` item exits `3` (a disagreement is
  refused, decision 2). `adr0128_lane_maps_to_pack_fix_bug_standard_story_story` asserts the two-row map and that a
  lane whose pack is not in `[packs] enabled` exits `3` naming the pack (decision 3). **What carries the kind on the
  item record is 7e's pack schema**; 7d's fixtures use a top-level `kind` key in `.redkiln/items/<item>.toml` and the test
  pins the refusal and the map, not the key's spelling — stated so a 7e rename re-points a fixture, not a decision.
  `pack-min/` is installed in the lane fixtures **twice**, as `.redkiln/packs/bug-standard.toml` and
  `.redkiln/packs/story.toml` (ADR-0128 decision 3's two names), with `[packs] enabled = ["bug-standard", "story"]`; the
  not-enabled arm of `adr0128_lane_maps_…` removes `story` from `enabled` and runs the story item.
- **The init-only repository (D18-8; RT-46)** — `run_on_init_only_repo_is_a_config_outcome`: the §7d.9 scaffold, no
  pack; exit `3`; stderr carries *"no process pack enabled"*; the store holds **exactly one row**, `reconcile_started`
  (AC-172's *"exactly one `reconcile_started` event per invocation"*; RT-46 — D18-8 read *"zero store writes"* until
  its dated correction of 2026-09-18 and now reads the same way), and zero `dispatch` rows; the harness marker count is 0. Red by a run that proceeds to select over an
  empty pack set (it would write `dag_assembled` or fail later with a different message), and by one that writes the
  first row twice.
- **The table-driven test** — `ac438_table_driven_exit_codes_over_fixtures` (§7d.4) runs the binary over
  `tests/fixtures/exit/<name>/` × the expected code from a committed table and asserts each; the eleven `StepError` rows are
  exercised **in-process** — the verb function with a `Ports` whose `FakeHarness::scripted` returns each variant,
  asserting the returned `Exit` (verbs return `Result<Exit, Halt>`) — each script carries the variant once per attempt
  the pack's cap admits (`pack-min/` declares an attempt cap of `1`; the kernel refuses to default it — AC-029),
  because the fake returns `Transport { kind: Spawn, detail: "script exhausted" }` when the script runs dry [measured
  2026-09-18, `crates/rk-ports/src/fakes.rs`] and a retried class would otherwise land on row `10` for a reason unrelated
  to the mapping — and the binary is driven for the rows a process
  can reach without a fake (§7d.4 names them); the in-process table is the eleven-row instrument, not the weaker red.

### 7d.6 `answer`

**The order of an `answer`** (`crates/rk-cli/src/verbs/answer.rs`): steps (1)–(6) as `run`'s with `Entry::Answer`;
then **the expiry pass** of `run`'s step (9) — every open gate of the item whose recorded deadline `< now` is resolved
through `gate::expire(…)` with the pack's declared action and its events appended (AC-257: *"observed and resolved at
that read"*; without it `gate::answer` would `Applied` a gate whose deadline merely passed, because it refuses `Expired`
only on a `gate_expired` row already in the journal — ***CORRECTED 2026-09-20, settling residual R-44: this is FALSE against the tree and the refusal is UNREACHABLE for a real expiry.*** *`workflow/fsm.rs`'s `GateExpired` arm **keeps the record in `gates_open`** (its own line: *"2026-09-17 (slice C): the record is KEPT in `gates_open` — an escalation names it and `stage_action` carries it"*) and inserts into `gates_expired` beside it, removing nothing. `gate::answer` then tests `state.gate_record(presented)` — which reads `gates_open` — **before** `state.is_expired(presented)`, and an expired token still in `gates_open` returns `Applied` or `Void` and never reaches the `Expired` branch at all [measured 2026-09-20, `crates/rk-kernel/src/workflow/gate.rs`:186-220, `workflow/fsm.rs`:1474-1488]. **So the CLI filters expired tokens itself**, which is what the shipped `answer` does; the kernel's `Expired` is reachable only for a token no longer in `gates_open`. The module header's *"the `GateExpired` arm removes the token from `gates_open`"* is a **historical** account of the pre-2026-09-17 behaviour under *"What the middle row was before"*, not the current arm — read the code, not the prose.* — then the item's **most recent gate record** is read from the
store — **and at 7d the store is ONE journal per repository**: `SqliteStore::query` ignores its selector and returns
every event [measured 2026-09-18, `crates/rk-adapter-sqlite/src/lib.rs`, `fn query(&self, _by: Query)`] (RT-4's
store-side `Query::ByItem` is routed, not built), the `gate_opened` payload carries `{deadline, kind, reach, token,
tree_hash}` and **no `item`**, the `gate_answered` / `gate_answer_void` / `gate_answer_spent` / `gate_expired` rows are
keyed on the token alone, and `fsm::fold(item, stage, events)` uses `item` only for `RunState::initial` — it does not
partition the events it is handed [measured 2026-09-18, `crates/rk-kernel/src/runtime.rs`, `workflow/gate.rs`,
`workflow/fsm.rs`]. So `answer <item>` reads the repository's runtime-shaped `gate_opened` rows with no
`gate_answered` / `gate_answer_*` / `gate_expired` successor for the token, **the fixtures are single-item by
construction** (`ac406_…`, `ac407_…`, `ac248_…`, `ac435_…` are labelled single-item), and **the per-item partition is
RAISED** (§7d.18): a dated design §5 note adding `item` to the `gate_opened` payload (an ADR-0116 decision 5 payload
change — a kernel edit) or RT-4's store-side filter; no CLI-side filter by an item id the rows do not carry is written.
The read: **none** → exit `3`, AC-248's outcome, **zero answer rows** — the invocation's one `reconcile_started` row
(AC-172; RT-46) is the baseline every refusal below is counted against: `ac248_…`, `ac410_…` and `ac435_…` assert the
store's row count equals **exactly one more** than before the invocation, that the added row is `reconcile_started`,
and that no `gate_*` / `artifact.*` row was appended; only `ac263_…` (which refuses before any store read) and
`ac486_…` (step (4)) assert a literally unchanged count; **live and open** → its token is presented;
**expired** → exit `3`, AC-248's outcome, the expiry resolved by the pass above (its `gate_expired` row IS written;
*zero answer rows* means zero `gate_answered` / `gate_answer_*` rows); **already answered** → its spent token IS
presented, `gate::answer` returns `RefusedSpent`, one `gate_answer_spent` row, exit `3` (AC-407; ADR-0123) — the rule is
stated once here because `ac248_…` and `ac407_…` read the same store state and demand different effects; the live
token is the presented digest — **the human never types it** (ADR-0129 decision 2). **This lookup is the CLI's and is
not the item-keyed answer AC-248 and ADR-0017 reject**: the answer handed to `gate::answer` and to `reconcile()` is
keyed on the token and bound to the tree hash (ADR-0017), so AC-248's THEN — *"no open gate on that item is matched by
item identity"* — governs what `reconcile()` accepts, not how the CLI finds the live token; a dated note on AC-248
saying so is OWED with the flip commit (§7d.18 RAISED). Then `tree_hash(exec, owned.path(), common_dir,
&TreeBasis::WorkingState)` **over the run-owned worktree the store records for the item** (H-5 — never `cwd`; ADR-0105
clause 4's one algorithm; RT-43: the order is reconcile, then read the gate, then hash — ***corrected 2026-09-20, R-50: the last two cannot hold in that order. `gate::expire` TAKES a tree hash, so the hash necessarily PRECEDES the expiry pass; the order is reconcile, then hash, then read the gate. RT-43's substance — reconcile before hash — is unaffected and is the clause that was routed*** — a hash before the reconcile
would be a hash of a tree the reconcile may report drifted against; a recorded worktree that no longer exists is exit
`3` naming its path, a refusal of the item's recorded state); the gate's **question** is resolved from the gate record's
`kind` (`unknown_facet:<axis>` — no gate carries a question id on the tree) → **the default question at 7d** in the
`rk-kernel` `QuestionRegistry`; `<option>` and every `--attach <kind>=<path>` are validated against it **first at the CLI
boundary** (`QuestionRegistry::validate(question, option, &attachments)` — the refusal is a usage error, exit `2`, zero
answer rows: the invocation's `reconcile_started` row stands, AC-172) **and again inside `gate::answer`**, which takes the registry and the attachments and calls the same
`validate` before it closes the mark — ADR-0063's Decision, *"the kernel refuses to close the mark unless that
attachment is supplied"*, is an accepted record's sentence, and ADR-0129 decision 4 says the same — the kernel refuses
inside `gate::answer`, the CLI first — so an `Applied` is **not** reachable by a caller that skips the CLI
check; the `answerer` is read **before the call** through the wrapper — `GitExec::run(GitOp::Log, owned.path(), ["log",
"-1", "--format=%an <%ae>"])` over the run-owned branch, the author identity the wrapper stamped on the gate-open commit
`run` made, which is the configured identity the answer commit will carry (RT-42 as corrected: read before the call so
the kernel can write it; `ac435_…` asserts it equals the answer commit's `%an <%ae>` after the commit); then **the
single call site** — `rk_kernel::workflow::gate::answer(&state, &gate, &presented, &tree_hash, option, &registry,
&attachments, AnswerMeta { explain_sufficiency, answerer })` (the fifth kernel edit: **three parameters added** to today's
five — `state, gate, presented, tree_hash, verdict` [measured 2026-09-18, `crates/rk-kernel/src/workflow/gate.rs`] —
§7d.15) — and its `Result<AnswerOutcome, GateError>` — `Err(GateError::NeverOpened)` is AC-248's named outcome, exit `3`,
zero answer rows; `Err(GateError::OptionRefused(OptionError::…))` is the kernel-side ADR-0063 refusal, exit `2`, zero answer rows;
`Err(GateError::WrongGate { .. })` cannot occur when the CLI resolved the token itself and is rendered as exit `3` naming
the defect; every other `GateError` (`Expired`, `EscalateOnEscalation`) is exit `3` with its `Display` — and on
`Ok(AnswerOutcome { disposition, events })`: `Applied` → the events appended (the `gate_answered` row carries
`explain_sufficiency` and `answerer` **as the kernel wrote them** — never inserted by the CLI into the
returned `Event`, so the row stays the kernel's and AC-262's wrapper stays the single caller-facing entry), then the
`Gate:` trailer commit **as a tier-1 write** — `authorise_tier1_write(exec, owned.path(), &ownership)` then
`commit_with_trailers` at the `OwnedWorktree`, never `cwd` (H-5; ADR-0105 clause 8; AC-518) — whose value is **the same
triple** `<token> <tree_hash_at_answer> <deadline_secs>` — the one value `rk-git`'s grammar parses (design §5.4: *"A
`Gate:` value that is not the triple is legacy drift"*; `reconcile.rs` keeps the latest mark per token, so the
reconciled `GateRecord.commit` becomes the answer commit) — a design §5.4 row for an answer-specific value is RAISED
(§7d.18), not invented here; exit `0`; `Void` → `gate_answered` + `gate_answer_void` appended, **no trailer commit** (the
gate stays open — AC-406), exit `0` (the answer was recorded, as void); `RefusedSpent` → one `gate_answer_spent` row, exit
`3` (AC-407: *"the second answer is refused"*). `gate::answer` reads no clock and computes no tree hash [measured
2026-09-18, `crates/rk-kernel/src/workflow/gate.rs`]; the CLI supplies both.

- **AC-405** — BINDING (`rk-cli`, `rk-store`), as amended 2026-09-18 (D-13). *"GIVEN a Phase 1 build / WHEN the Phase
  1 dependency closure and the call graph are inspected / THEN it contains no tracker or web crate, and the gate-answer
  store method has exactly one call site, in `rk-cli`'s `answer` command."* *Verified by:* *"a crate-graph test
  asserting no GitHub or web crate in the Phase 1 dependency closure, plus a lint asserting exactly one call site of
  the gate-answer store method"* — the symbol is `rk_kernel::workflow::gate::answer` behind AC-262's single wrapper,
  and the crate list is `deny.toml`'s. `ac405_gate_answer_has_exactly_one_call_site`: a source scan over every
  `crates/*/src` for `gate::answer(` / `workflow::gate::answer(` with the comment filter and for the import spellings
  `use …gate::answer` / `gate::{… answer …}` (a bare `answer(` after an import evades the qualified pattern), counting an
  import as a site, exempting the definition in `rk-kernel/src/workflow/gate.rs` (no other `rk-kernel/src` file calls
  it — the FSM folds `gate_answered` rows and never calls `answer()` [measured 2026-09-18: `grep -rn 'gate::answer('
  crates/rk-kernel/src` → the definition's module only]; the only callers on the tree are the kernel's own integration
  tests) and every `tests/`,
  asserting exactly one hit and that it is under `crates/rk-cli/src/verbs/answer.rs`. The crate-graph half is `deny.toml`'s eight `[bans].deny` rows `octocrab`, `reqwest`,
  `hyper`, `axum`, `ureq`, `tokio-tungstenite`, `warp`, `actix-web` (DECISIONS-2026-09-18 RT-36; §7d.15; `cargo deny
  check bans` is the fifth gate and the block runs it) — none is in `Cargo.lock` today [measured 2026-09-18], so the
  rows bite on contact and ban nothing present.
- **AC-262 (the wrapper)** — kernel-owned; 7d's share is that `answer` calls through the one `pub` wrapper and that
  the differential's CLI side is this verb. Not claimed here beyond that sentence (`s7c` §7c.22 RAISED it for
  re-routing).
- **AC-406** — BINDING (`rk-cli`, `rk-git`), as amended 2026-09-18. *"GIVEN an open gate whose record binds `(token,
  tree_hash, deadline)` / WHEN `redkiln answer` is invoked with a valid token but the tree hash no longer matches the
  recorded one / THEN the answer is recorded as void, the gate stays open with the observed and recorded tree hashes
  attached, and no stage transition occurs — the answer is never reconciled into a verdict."* *Verified by:* *"an
  integration test that answers after a commit lands, asserting a `gate_answer_void` event carrying both tree hashes,
  `gate.state == open`, and no stage transition."* `ac406_answer_after_commit_is_void_gate_stays_open`: open a gate
  (the `ac403_…` fixture), commit a file **in the run-owned worktree on `rk-run/<item>`** (the tree `tree_hash` reads —
  H-5), `answer <item> approve --explain-sufficed yes --toml-opened no`; assert `gate_answer_void` with `recorded` ≠
  `observed`, the fold's gate state `open`, no `stage_transition` row after the void, **no new `Gate:` trailer commit**
  (`git rev-list --count rk-run/<item>` unchanged since the test's commit), and the fixture checkout's `HEAD` and
  porcelain unchanged throughout (AC-518). Red by an answer that hashes
  before reconciling (RT-43) or that commits on `Void`.
- **AC-407** — BINDING (`rk-cli`, `rk-store`). *"GIVEN a gate answered once with its single-use token / WHEN the same
  token is presented again / THEN the second answer is refused and no second `gate_answered` event is written."*
  *Verified by:* *"a store test replaying the same token twice and asserting exactly one `gate_answered` event."*
  `ac407_second_token_refused_one_gate_answered_event`: `answer` twice on one gate; the second exits `3`, the store has
  exactly one `gate_answered` and exactly one `gate_answer_spent` (ADR-0123's kind), and `git rev-list --count
  rk-run/<item>` in the run-owned worktree is unchanged across the second `answer` (no commit landed) — the fixture
  checkout's `HEAD` never moved at all (AC-518). The kernel twin (AC-246) is
  7b-ii's; this is the CLI end.
- **AC-248 (CLI half)** — BINDING (`s7b-ii` §7b-ii.14 ARMED it here). *"a CLI test that `redkiln answer <item>
  approve` without a resolvable live token exits non-zero with a named outcome."*
  `ac248_answer_without_live_token_exits_named_outcome`: an item with **no gate record at all** → exit `3`, stderr
  names the outcome (*"no open gate on <item>: no live token to answer"*) and **zero answer rows** are written — the
  store gained exactly the invocation's `reconcile_started` row and no `gate_*` / `artifact.*` row (AC-172's one row is
  the baseline — the rule stated at the head of this section); the invocation carries the required flag — `answer
  <item> approve --explain-sufficed yes --toml-opened no` — so the `3` is the store's refusal, not the parser's `2`; an item with an **expired** gate → the same, and the
  store shows the expiry resolved at that read (AC-257's *"observed and resolved at that read"*).
- **AC-410** — BINDING (`rk-cli`, `rk-types`), as amended 2026-09-18 (D-04: the positive control). *"GIVEN a gate
  question whose registry entry declares its options and each option's admissible attachment types / WHEN `redkiln
  answer` is passed an attachment that no declared option admits / THEN the answer is refused at the CLI boundary and
  nothing is written."* *Verified by:* *"a unit test on the attachment validator; a red fixture carrying an unbound
  attachment that must fail; a store assertion that the refused path wrote no event"* — plus the positive control.
  `ac410_unadmitted_attachment_refused_zero_events`: a fixture question `decide` with option `proceed` admitting
  `attaches = ["Findings"]` — declared **in-process**: the test constructs a `QuestionRegistry` holding the default
  question plus `decide` **and binds the fixture gate's kind to `decide` through the registry's resolution seam**
  (`QuestionRegistry::bind(gate_kind, question_id)` — the same seam 7e's pack loader will call; the default binding
  stays `unknown_facet:*` → the default question, so under the default binding alone the fixture gate could never
  resolve to `decide`, `proceed` would be an undeclared option, and the negative would exit `2` for the wrong reason —
  `UnknownOption`, not the attachment — while the positive control could never pass), and drives
  `verbs::answer::run_with(registry, …)` (the verb's function, not the binary); no pack
  TOML declares a question at 7d (ADR-0129 decision 3: pack-declared questions are 7e's), and the binary-level `answer`
  is exercised against the default question (`ac406_…`, `ac407_…`, `ac435_…`) **plus one binary-level twin of the
  negative that needs no seam**: `answer <item> approve --attach Findings=<path> --explain-sufficed yes --toml-opened no` exits `2` —
  `approve` admits nothing; in-process, `answer <item> proceed --attach Spec=<path> --explain-sufficed yes --toml-opened no` exits `2` (usage — the
  refusal is at the CLI boundary, before the store) and **the refusal's variant is asserted**:
  `OptionError::AttachmentNotAdmitted { kind }`, never `UnknownOption` (an `UnknownOption` refusal is red); the store
  gained exactly the invocation's `reconcile_started` row and no `gate_*` / `artifact.*` row (AC-172's one row is the
  baseline — §7d.6's rule); `answer <item> proceed
  --explain-sufficed yes --toml-opened no` **without** the declared attachment exits `2` with `OptionError::AttachmentMissing { kind }`
  (ADR-0063's *"refuses to close
  the mark unless that attachment is supplied"* — refused at the CLI boundary first and, for any caller that skips
  it, inside `gate::answer` by the same `QuestionRegistry::validate`; §7d.6's order). `ac410_admitted_attachment_stored_as_human_root`: `--attach Findings=<path>` is
  accepted, the gate is `Applied`, and `answer` appends one `artifact.write` row — **the producer is routed** (RT-59):
  ADR-0116 decision 1's producing step is *every activity output*, and `answer` is a verb, so the row's producer is
  widened here under a routing — *the human-supplied root's carrier is an `artifact.write` row written by `answer`; a
  dated design §5 note or an ADR-0116-route record is OWED* (§7d.1's table, §7d.18) — the kind exists and nothing in the
  tree writes it yet [measured 2026-09-18: `grep -rn ArtifactWrite crates/rk-kernel/src` → match arms only] — with
  payload in **ADR-0116 decision 1's shape** — `{ kind: "<Kind>", item, schema_version: <the kind's registered schema
  version>, digest: sha256(file) }` — ***corrected 2026-09-20, settling residual R-49: there IS no registered schema VERSION to read.*** *`ArtifactKind` carries `schema: Option<String>` and **no version field at all** [measured 2026-09-20, `crates/rk-kernel/src/select/artifacts.rs`:59-65 — the fields are `name`, `source`, `keyed_by`, `schema`]. ADR-0116 decision 1's member keeps its name, and its value is the kind's registered `schema` string when one is registered and `null` when none is: **the id is written and a version is NOT invented.** Inventing one would put a number in a tier-1 payload that no registry could ever confirm.* — with the binding path `<gate step>.attachments.<Kind>` carried as the artifact's
  **instance name inside `kind`'s registered slot**, not as a fifth payload member (ADR-0063's `derived_from = []` root is
  what the ABSENCE of an `artifact.read` predecessor means for this row; a widened payload — an `instance` or
  `derived_from` member — is a design §5 dated note or an ADR-0116-route record, RAISED in §7d.18, not written here: a
  stage file may not re-shape an event kind an accepted ADR fixed, decision 5's schema-drift rule), which is the
  **human-supplied root** the test asserts by row and payload; whether a downstream slot reads it as
  `${decide.attachments.Findings}` is the kernel's binding resolver at 7e and is not asserted here — red by a validator
  that refuses everything, which passes the
  three negative instruments and is the PIA-059 shape D-04 named.
- **AC-263** — BINDING (`rk-cli`, `rk-ports`; from `s7b-ii`). *"GIVEN Phase 1, and an answer presented through a stub
  tracker-comment or web-write channel / WHEN it is submitted / THEN it is refused with a named 'channel not enabled
  in this phase' outcome, no gate state changes, and no answer event is written."* *Verified by:* *"a port-level test
  driving a stub non-CLI channel against the answer API and asserting the named refusal and the unchanged gate; a
  build-configuration test that the tracker answer route does not compile in."*
  `ac263_stub_channel_refused_named_outcome`: a test-only `StubTrackerChannel` drives
  `rk_cli::verbs::answer::submit(Channel::TrackerComment, item, option, attachments)` — `Channel` is an `rk-cli` enum
  (`Cli`, `TrackerComment`, `WebWrite`; §7d.1's routing row), the one non-`Cli` arm returns `Err(ChannelNotEnabled {
  phase: 1 })` **before any store read**, and `submit(Channel::Cli, …)` is the only path that reaches `gate::answer` (so
  the AC-405 call site stays one); the fold is unchanged and the store gained no row; `rk-ports` and `gate::answer` are
  untouched (§7d.15), so the register's `rk-ports` tag is met by no port type (RAISED, §7d.18). The build-configuration half is the `deny.toml` rows and the block's
  `cargo metadata` line: `rk-cli`'s normal graph reaches no tracker crate.
- **AC-435 (machine half)** — the human half is **human-gated and excluded** (§7d.18). *"THEN the answer (yes/no, plus
  whether a TOML was opened) is recorded as a required field on the gate answer"*; Verified-by: *"the machine-checkable
  half is that the gate answer cannot be submitted with the field absent."*
  `ac435_answer_refuses_without_explain_sufficiency_and_records_it`: without `--explain-sufficed`, or with it and without
  `--toml-opened`, exit `2` (the parser's — before any store read, so the store is untouched); with `--explain-sufficed
  yes --toml-opened yes` on one gate (SM-126's kill cell: `explain` sufficed and the reviewer opened a TOML *anyway*) and
  `--explain-sufficed no --toml-opened no` on another, the `gate_answered` payload carries one member
  `explain_sufficiency: { sufficed: "yes", toml_opened: "yes" }` / `{ "no", "no" }` verbatim — ***corrected 2026-09-20,
  settling residual R-45: these two cells are BOTH ON THE DIAGONAL and cannot catch a one-flag record.*** *A mutant
  writing `"toml_opened": sufficiency.sufficed` — the exact shape ADR-0129 decision 1 was corrected away from, and the
  one SM-126's pre-registered kill needs — agrees with the record on `{yes,yes}` and on `{no,no}` and is caught by
  NEITHER prescribed cell. **The discriminating cells are the OFF-DIAGONAL ones**, `{yes,no}` and `{no,yes}`, and the
  floor is therefore **all four**. The shipped test already runs all four and is stronger than this prescription
  [measured 2026-09-20, `crates/rk-cli/tests/it/answer.rs`:760 — `for (sufficed, toml_opened) in [("yes","yes"),
  ("yes","no"),("no","yes"),("no","no")]`]; the correction is recorded here so no later reader trims the test back to
  this floor and re-opens the escape* (design §5's `gate_answered`
  row as amended 2026-09-18 and corrected 2026-09-19; ADR-0129 decision 1 as accepted — **two data, one field**, the
  operator's answer of 2026-09-19 to the question the stage-open commit carried) and `answerer` equal to the answer commit's `%an <%ae>`
  (RT-42). SM-126's instrument is the field; the first thirty answers accrue from `s8` at the earliest.
- **The question registry (`-p rk-kernel`)** — `adr0129_question_registry_default_question_two_options` (the
  registry's default question is `approve` / `changes-requested`, neither with `attaches`; a gate step naming no
  question resolves to it) and `adr0063_option_with_attaches_refuses_close_without_attachment` (a registry question
  whose option declares `attaches` makes `QuestionRegistry::validate(&question_id, &option_id, &attachments) ->
  Result<(), OptionError>` return `Err(OptionError::AttachmentMissing { kind })` when the kinds are absent and `Ok(())`
  when present, and makes `gate::answer` — which calls it before it closes the mark — return
  `Err(GateError::OptionRefused(..))` with zero events over an otherwise-`Applied` state: the kernel's refusal is the one
  ADR-0063 fixes, and the CLI's earlier refusal is the usage-error twin; the question is constructed in the test, as
  `ac410_…`'s is; no pack TOML declares one at 7d). Pack-declared questions are **7e's**; the type is
  7d's.
- **What `answer` does not do**: it does not reconcile the answer into a verdict (the FSM's fold does, on the next
  `run`); it does not take `--token`; it does not read stdin; it does not default either AC-435 datum (both flags are
  required — the register's *"cannot be submitted with the field absent"*); it never commits to the checkout it was spawned in (H-5).

### 7d.7 `status` and `explain`

- **`status`** — a **roll-up read** (design §8.1: *"roll-up read; read-only, so it does not reconcile"*). It opens
  the store, reads the item's events, and prints `rk_kernel::workflow::fsm::stage_action(item, stage, &events)` per
  item — **no `reconcile_at_entry`** (ADR-0105 clause 8: *"AC-172: `status` does not reconcile"*; `Entry` has no
  `Status` variant, so the type forbids it), **no clock** (the fold is over recorded events), no git write.
  `ac402_status_and_explain_do_not_reconcile` is its instrument. Staleness is the accepted store-as-cache consequence
  (C-01 D-16 dropped). Its stdout is unpinned (E5).
- **`explain <item>`** — the grammar D-03 settled (positional item; no `--run`, no `--stages`). It prints **from the
  recorded resolution only**, and **the recorded resolution is the persisted run record**, not the event log alone:
  no event carries AC-430's rows or AC-431's fields — `dag_assembled` is `{inputs_digest, nodes: [{id, activity_id,
  instance, cost_class, cost_points}], edges, rows_fired (a count), skipped: [{stage, kind, requiring_activity, reason}],
  gated, appetite, cost}`, `activity_committed` is `{activity_id_at_version, at, attempt, item, result, run_id}`, and
  `RecordedSpec` lives in the run record's `inputs/specs.json`, not on any event [measured 2026-09-18,
  `crates/rk-kernel/src/select/step5.rs`, `runtime/activity.rs`, `replay.rs`]. So **`run` persists the record**:
  after `replay::resume` returns, `rk_kernel::replay::write_run(&<common-dir>/redkiln/runs/, &recorded_run)` — a
  filesystem write outside the working tree, beside the store, never a git write (`write_run` lays down `run.json`, the
  logged `RunInputs` sources under `inputs/` — `pack/`, `catalog/`, `artifacts/`, `facets.toml`, `context.json`,
  `artifact-state.json`, `runtime.json`, `specs.json` — and `events.jsonl` [measured 2026-09-18, `replay.rs`]) — and
  **`explain` reads it back through `rk_kernel::replay::read_run(dir)`** for the item's most recent run (the run id from
  the store's last `activity_committed` / `dag_assembled` row's `run_id` — ***corrected 2026-09-20, R-55: the `dag_assembled` half is UNIMPLEMENTABLE as written. That row's payload is `{inputs_digest, nodes, edges, rows_fired, skipped, gated, appetite, cost}` and carries **neither a `run_id` nor an `item`** [measured 2026-09-20, `crates/rk-kernel/src/select/step5.rs`:248-273] — GAP 10. The `activity_committed` half stands; the resolution reads the run id from there alone***), rendered by a function in
  `crates/rk-cli/src/verbs/explain.rs` that takes `&RecordedRun` and nothing else: AC-430's rows are read off the
  recorded pack and catalog **sources** joined on `dag_assembled.nodes[].activity_id` — a read of logged inputs, never a
  selection; AC-431's card is the recorded `RecordedSpec.card_digest`; AC-434's `unknown` facets are the recorded
  `context.json`. Two limits are RAISED (§7d.18) and place AC-431 and AC-433 in the PROVISIONAL sub-list: **skills,
  fillers and tier are on no recorded value** (`RecordedSpec::to_json` carries `card_digest`, `tools`, `schema`,
  `timeout_secs`, `inputs` — the binder's binding is not logged), and **no admissibility exclusion is recorded**
  (`SkipReason` has exactly two stage variants, `ForbiddenStage` and `ForbiddenStageViaRequires` [measured 2026-09-18,
  `crates/rk-kernel/src/select/step1.rs`]).
  *(dated 2026-09-29, `s8f` build, package S8F-P8-DOCTOR-RULES-AND-NOTES; `s8f` §8f.4 and §8f.10; the landing record
  ADR-0188 decision 12, proposed.)* *"prints **from the recorded resolution only**"* now governs the RESOLUTION block,
  not the whole output. `explain` gains a **landing block** after the recorded resolution (`s8f` §8f.4), and its source
  is not the run record: it is read from git through `GitOp::RevParse` (the tip of `rk-run/<item>`, how many commits it
  is ahead of the base and how many of them carry content) and from the tier-1 rows' `anchor_sha`, and it prints the
  operator's hand-off command, `git merge --no-ff rk-run/<item>`. It computes no selection, so AC-432 holds (the
  unreachability scan still reads `explain.rs`). An item with no run ref prints a blank line and `landing: nothing
  landed (no rk-run/<item>)`, which is what the three `explain` snapshots (`ac430`, `ac433`, `ac434`) gained.
  - **AC-430** — BINDING. *"GIVEN an item whose DAG has been selected / WHEN `redkiln explain <item>` runs / THEN the
    output prints, for every included activity, BOTH its admissibility row from the activity (`provides`, `requires`,
    `applicable_if`, `method_rank`, `min_rigor`) AND the requirement rows from the process pack (`baseline` and each
    matching `require` row) that put it in the DAG."* *Verified by:* *"a snapshot test on a fixture item asserting
    both row families for every included node, and failing if either family is empty while the other is
    populated."* `ac430_explain_prints_both_row_families` — an `insta` snapshot over the run-record directory
    `tests/fixtures/explain/two-activities/` (a `write_run` layout: `run.json`, `inputs/`, `events.jsonl`), plus the
    explicit both-or-neither assertion per node.
  - **AC-431** — BINDING, **PROVISIONAL at 7d** (§7d.18). *"THEN the printed resolution names the card, the attached skills, the inserted slot
    fillers and the effective tier for that activity."* *Verified by:* *"a snapshot test asserting all four resolution
    fields per activity, and failing when a resolution field present in the run record is absent from `explain`
    output."* `ac431_explain_names_card_skills_fillers_tier` — the card (by its recorded `card_digest`) per activity read
    from the recorded `RecordedSpec`, red by dropping it; **skills, fillers and tier are on no recorded value** (the
    binder's binding is never logged — `RecordedSpec` carries `card_digest`, `tools`, `schema`, `timeout_secs`,
    `inputs` [measured 2026-09-18]), so the test asserts the one field the record has and the id is claimed by that
    field only until the binder's binding is recorded (RAISED, §7d.18).
  - **AC-432** — BINDING (`rk-cli`). *"GIVEN a run whose resolution was written to the run record / WHEN `redkiln
    explain` renders it / THEN the output is derived from the recorded resolution only; `explain` performs no
    selection, and an item with no recorded resolution is an error rather than a recomputation."* *Verified by:* *"a
    replay assertion — `explain` over a replayed event log is byte-identical to `explain` at run time — plus a kernel
    test asserting the selection engine is not reachable from the `explain` code path."*
    `ac432_explain_over_replayed_log_is_byte_identical`: render at run time from the run record `run` wrote, replay
    that record through `rk_kernel::replay::replay(dir, ..)` (the run-record **directory** — `replay` takes a `&Path`
    [measured 2026-09-18, `replay.rs`]), render the replayed record again, compare bytes; the declared mutant is a
    render that reads the clock or the process environment (a `Clock::now()` timestamp or a `%USERNAME%` in the
    header), which the replay cannot reproduce. `ac432_explain_from_record_only_and_errors_without_resolution`:
    an item with no run record and no `dag_assembled` row → exit **`5`** (`NoRecord`, H-2) and stderr *"no recorded resolution for
    <item>; explain does not recompute"*; and the unreachability half — a source scan that `crates/rk-cli/src/verbs/
    explain.rs` names no `rk_kernel::select` path and the module-boundary shape 7b's `module_boundary.rs` uses, as a
    test in `rk-cli`. F36 is the ground: recomputing selection *"would print something that never ran"*.
  - **AC-433** — BINDING, **PROVISIONAL at 7d** (§7d.18). *"GIVEN an activity that is admissible in the catalog but excluded from this item's DAG /
    WHEN `redkiln explain <item>` runs / THEN the output names the specific clause that excluded it — the false
    `applicable_if` sub-predicate, the unmet `min_rigor`, or the absent `require` row — rather than reporting 'not
    selected'."* *Verified by:* *"a snapshot test over a fixture carrying one exclusion of each kind, asserting the
    named clause and failing on a bare 'not selected' string."* `ac433_explain_names_excluding_clause_never_not_selected`
    over the run-record directory `explain/three-exclusions/`; the bare string is asserted absent. **The excluding
    clause is not on the record**: `dag_assembled.skipped[].reason` is `forbidden_stage` or
    `forbidden_stage_via_requires` and nothing else, so the three clause kinds the THEN names are printed by reading the
    recorded admissibility row beside the recorded `context.json` and the recorded pack — the false `applicable_if`
    sub-predicate through `rk_kernel::predicate` (its `on_false` naming — ***corrected 2026-09-20, R-56: `predicate::on_false` is a MODULE and is not AC-433's sub-predicate namer. The namer is ADR-0103 clause 6's closed form set plus `predicate::classify_on_false` [measured 2026-09-20, `crates/rk-kernel/src/predicate/on_false.rs`:98, re-exported at `predicate.rs`:68]*** — over the recorded vector; a read of two logged
    inputs through the predicate module, never `select`, so AC-432's unreachability scan stands), the unmet `min_rigor`
    against the recorded pack's rigor, the absent `require` row against the recorded pack's rows — **disclosed as a
    re-reading of recorded inputs, not a recorded exclusion**; the register's *from the record only* is met in full
    only when a kernel-recorded `excluded` row lands (a design §5 dated note or an ADR-0116-route payload — RAISED,
    §7d.18), which is why the id is PROVISIONAL.
  - **AC-434** — BINDING (display only; the selection engine's block is untouched). *"GIVEN a context vector in which
    one or more facets are `unknown` / WHEN `redkiln explain` prints the admissibility rows that consumed them / THEN
    each `unknown` facet is rendered as `unknown` with its position-derived treatment stated, and is never rendered
    as a measured band value."* `ac434_unknown_facet_rendered_unknown` over a run record whose `inputs/context.json` carries `unknown` on four axes (`explain/unknown-four-axes/`), asserting each of the four
    renders the literal `unknown` with its position-derived treatment stated (the positive half — a renderer that prints
    nothing for an `unknown` facet passes the band check alone), and red if any renders as a band.
  - **AC-435's human half** — excluded (§7d.18); the machine half is §7d.6's.
- **`catalog search <query>`** (RA-M-02; **D18-15**) — `rk_kernel::catalog::search(&catalog, query)` over the `keywords`
  field the loader now reads on `Activity`; an absent `.redkiln/catalog/` is `Catalog::default()` for this read —
  `Catalog::from_sources` refuses an empty set by design (*"an empty catalog is an error, not an empty success"*
  [measured 2026-09-18, `crates/rk-kernel/src/catalog.rs`]) and that refusal stays `run`'s (the no-pack refusal is
  `run`'s — D18-8), so the block's `catalog search` on the init-only repository exits `0`; prints the matching activity ids. **AC-436** — BINDING (`rk-cli`, `rk-kernel`), as amended
  2026-09-18. *"GIVEN an activity's engine-ignored `keywords` field, which `redkiln catalog search` searches / WHEN
  the `keywords` of every activity in the catalog are mutated arbitrarily and selection is re-run over a fixed context
  vector / THEN the selected activity set is byte-identical."* *Verified by:* *"a property test that permutes
  `keywords` across the shipped catalog and asserts an unchanged selection, plus a lint asserting `keywords` is
  unreachable from the admissibility evaluator"* — plus the positive control. `-p rk-kernel`
  `ac436_keywords_permutation_leaves_selection_unchanged` (a `proptest` over a **synthetic** catalog — the shipped
  catalog is 7e's — permuting and rewriting `keywords`, asserting the ordered set of selected activity ids is equal — the ids, not a
  serialisation of `Activity` (which now carries `keywords`) and not the run's `inputs_digest` (the catalog source
  string feeds `RunInputs::to_json()`, so that digest moves with the permutation by design and is not the register's
  THEN));
  `ac436_keywords_unreachable_from_admissibility_evaluator` (a source scan over `crates/rk-kernel/src/select/` and
  `predicate/` for the identifier `keywords`, comment-filtered, zero hits; control: `catalog.rs` names it); `-p rk-cli`
  `ac436_catalog_search_finds_by_keyword` (the positive control: a search for a keyword one activity carries returns
  exactly that id; a search for one no activity carries returns nothing and exit `0`; **the keyword is a token that
  appears in no other field of any activity** — asserted by the test over the synthetic catalog's serialised bytes
  minus `keywords` — so a search that greps ids or descriptions instead of `keywords` is red too) — red by a loader
  that drops the field, which is what the tree does today.
- **`lint`** — `lint_wraps_kernel_facet_rules_and_exits_by_table`: `--facet-stability <fixture>` over a facets file
  with an `llm`-sourced facet exits `3` and names the violation (`rk_kernel::facet_stability::facet_stability_rule`'s
  fail-closed arm, AC-071's); `--facet-provenance` likewise over a declared-surface violation
  (`facet_provenance_rule`, AC-520's); a clean pair exits `0`. The lints' criteria are 7b's and 7e's; **7d claims the
  surface only** and neither id.
- **`s8`'s two `explain --run … --stages` lines were the losing record** (D-03) and were rewritten on 2026-09-18
  against the event log; the digest field d4 asserts is **named — ADR-0128 decision 4 (2026-09-19, O-13 closed): each
  committed step's `output_digest` in the recorded resolution**, F38's measured quantity. What `explain` prints is the
  recorded resolution whole (AC-430–AC-432); it prints **both** digests, labelled — the `dag_assembled` row's inputs
  digest and each committed step's `output_digest` (**RT-52**, decided) — and `s8`'s exit check (3) reads the
  `output_digest` lines for its equality; the inputs digest is printed, not asserted, because a resumed run that
  re-derives its DAG over a tree a committed step changed may legitimately differ there.

### 7d.8 `doctor`

**The rule table IS data** (AC-415: *"the `doctor` rule table, which is data with a stable id per rule"*): `pub static
RULES: &[Rule]` in `crates/rk-cli/src/doctor/rules.rs`, `Rule { id: &'static str, severity: Severity, check: fn(&Ctx)
-> Vec<Finding> }`, run in order by `verbs/doctor.rs` over a `Ctx` built once **by the composition root** — the `ConfigFile`, the enabled
packs and their catalog through `rk_kernel`'s loader **in its diagnostic mode** (`catalog::load_diagnostic`, the 7d
kernel edit — **RT-54**, §7d.1 — that returns every `CatalogError` typed instead of the first: `Io`, `Toml`, `Shape` —
the three variants the loader has [measured 2026-09-18, `crates/rk-kernel/src/catalog.rs`] — with the new
`Gate`-with-`outputs` refusal among the `Shape`s; **the loader reads no `timeout_secs` and validates no facet** (the only
`timeout_secs` reads in `rk-kernel` are `RecordedSpec`'s in `replay.rs`; facet validation is `facet_registry_rule`'s and
`select/step2.rs`'s), so `load_diagnostic` types only what the loader sees and maps only `GateOutputs` →
`RK-D-GATE-OUTPUTS`; **`RK-D-TIMEOUT`** reads per-activity `timeout_secs` through `rk-cli`'s own `PackExtras` (absent →
`Missing`; non-integer → `NonNumeric`) *(**amended 2026-09-21, Step 7d Slice 4 cycle 5 batch 2: the reader is `rk-cli`'s own PERMISSIVE view of the pack document, not `PackExtras`.** The prescription cannot be built as written. `verbs::run::ActivityExtras.timeout_secs` is a **required `u64`** [measured 2026-09-21], so an activity table declaring none and one declaring `"900"` are the SAME `serde` failure — the two arms this row exists to tell apart become indistinguishable. Worse, the failure is not local: `PackExtras` also carries `margins` and `cache`, both with required members, so ONE absent `timeout_secs` would silence **`RK-D-MARGIN`** and **`RK-D-TTL`** as well, three rows blinded by one defect — and a pack declaring `[cache] prompt_ttl` alone refuses for the same reason, which is `RK-D-TTL`'s own rejecting fixture. **The same asymmetry settled the card reader one batch earlier** and is stated once for both: a LOADER refuses a document so nothing downstream reads a half-parsed one, while a LINTER must read the parts that are well formed in order to report the part that is not. A doctor rule built on a loader's type reports nothing precisely when there is most to report — `rk_adapter_claude::card::CardFile` is `deny_unknown_fields`, so a card with one unknown key would hide a missing `StructuredOutput` behind the typo. The rows read `toml::Value` and `serde_json` views carrying only the members they assert on. **What is unchanged is the prescription's substance**: the two arms are still `Missing` and `NonNumeric`, and `RK-D-TIMEOUT` still reads per-activity `timeout_secs`*)* and **`RK-D-FACET-UNKNOWN` / `-VALUE` / `-NEW`** run
`rk_kernel::facet_registry::facet_registry_rule` and `Registry::from_toml` over `facets.toml` plus the pack's `require`
rows — both in `rk-cli`'s doctor rules, **no loader widening** — so a load-rejected fixture still yields its finding; the
strict loader `run` uses gains **one** refusal of its own, a `Gate` step carrying `outputs` (`CatalogError::Shape`,
AC-409's WHEN — the seventh kernel edit, §7d.1), and `load_diagnostic` reports the same refusal typed), `facets.toml` and the fact registry *(read as ONE value, not two — dated note 2026-09-20, Step 7d Slice 4 cycle 2. This phrase reads as two `Ctx` members and names one type: `rk_kernel::facet_registry` declares no `Registry` of its own, it imports `crate::registry::Registry` [measured 2026-09-20, `crates/rk-kernel/src/facet_registry.rs`:118], so the file and the fact registry are the source and the parsed value of a single thing and a `Ctx` with a field for each would hold it twice. **What IS a second member is `PredicateSites`** — `facet_registry_rule(registry, sites)`'s second parameter, the catalog's `applicable_if` expressions and the packs' `require` rows. Its own doc says why it cannot be folded into the first: a rule reading only `require` rows passes *“a catalog whose `applicable_if` names an unregistered facet, which is `binary_change`'s measured case”*. So the members are `registry: Option<Registry>` and `sites: PredicateSites`, and this is a clarification of the phrase rather than a change to what it prescribes)*, the captured environment, **the store handle
the root opened** (`Ctx.store: Option<Result<SqliteStore, StoreOpenError>>` — `Some(Ok(store))` on a clean open,
`Some(Err(Refused))` for a foreign store, carried in and not refused before the table, so `RK-D-STORE-ID` renders it
as its row; `None` on the `--rebuild` path below; `use rk_adapter_sqlite::{SqliteStore, StoreOpenError};` is the natural
line for this field and is **admitted** — naming a type constructs nothing, the constructor ban below keeps
`SqliteStore::open` out of the module, and the import ban is scoped to the port-implementor constructors' types),
the git common dir, and a `VersionProbe`. **`doctor` does NOT call
`AdapterSet::resolve` as a startup refusal** (that is `run`'s step (4)): it builds its `Ctx` from the `ConfigFile` and
reports adapter defects only through `RK-D-ADAPTER`, which is why the row and its id are observable on the real binary.
**`doctor` reconciles at entry** (`Entry::Doctor`; AC-402; C-01 D-06 dropped) and **never constructs a real `Harness`**
(**D18-7**; AC-414 as amended): the root builds the `doctor` path's `Ports` — `harness: NoDispatchHarness` (the `src`
type of §7d.1, panicking on `dispatch` and naming the rule), `store` the handle above, `tracker` / `docs` the two Phase-1
stand-ins, `toolchain` the `rk_ports::fakes::FixtureToolchain::default().absent()` stand-in (`absent` is a builder over
`self` — `pub fn absent(mut self) -> Self` on a type that derives `Default` [measured 2026-09-18,
`crates/rk-ports/src/fakes.rs`] — not an associated constructor; `doctor` dispatches nothing and calls no
toolchain method — D18-7's reading extended to the toolchain slot; the Rust real needs a `timeout` from a pack's
`timeout_secs`, and the init-only repository has no pack) — and hands it in; **the `doctor` module constructs no port
implementor**, and the block's source scan over it bans the register's denylist verbatim (AC-414 as amended: *"the four
adapters' constructor paths — `rk_adapter_claude::ClaudeHarness::new` / `Config::new`, `rk_adapter_sqlite::SqliteStore::open`,
the two `rk_adapter_toolchain_*::*Toolchain::new` — and any `dyn Harness` acquisition"*; the toolchain `Config::new`s and
`rk_adapter_claude::config::Config::new` construct no port implementor and `RK-D-ADAPTER` reaches them through
`AdapterSet::resolve` in `adapters.rs`, outside the filtered module — and `resolve` itself returns names and constructor
values, never a constructed port (§7d.5 step (4)); `env_denylist::` and `config::TESTED_CLI_VERSION` stay importable —
`Config` and `TESTED_CLI_VERSION` live at `rk_adapter_claude::config::…`, `lib.rs` re-exports neither [measured
2026-09-18, `crates/rk-adapter-claude/src/lib.rs`], so the ban's constructor arm is `rk_adapter_claude::config::Config::new`
plus a bare `\bConfig::new\(` for the imported spelling, and the toolchain `Config` import is **not** banned because
the prose above permits it), and a second filtered ban bans the `use rk_adapter_claude::{Config, ClaudeHarness}` /
`use rk_adapter_claude::config::Config` / `use rk_adapter_toolchain_*::*Toolchain` import spellings that would evade a
fully-qualified pattern — **the port-implementor constructors' types only**: `use rk_adapter_sqlite::SqliteStore` is
**not** banned, because `Ctx.store` names that type and a type import constructs nothing (AC-414 as amended bans the
CONSTRUCTOR paths, and `SqliteStore::open` stays on the first ban) — `ac414_doctor_module_names_no_adapter_constructor`
walks imports as well as call sites. Its one child process is the version probe (D-11). Its stdout is one line per
**evaluated** rule — fired or not — `<id> <status> <detail>` with `ok` a status word, and **nothing else about that
line is pinned** (RT-44; E5; D18-6 as corrected 2026-09-18 — the block's FLOOR loop reads the register-named ids off a
clean repository, which only holds because every evaluated rule prints its row). Exit `0` when no error-severity rule
fired, `3` when one did (AC-438's *"including any failing error-severity `doctor` rule"*), warnings never move the
code.

**The rows, each pinning the register row it is built for** — id · severity · the criterion's clause · the rejecting
fixture directory under `crates/rk-cli/tests/fixtures/doctor/<id>/` (AC-415: one per id):

| id | sev | pins | clause it reads | fixture |
| --- | --- | --- | --- | --- |
| `RK-D-GATE-OUTPUTS` | error | AC-409 (as amended 2026-09-18) | *"rejecting any graph reference that resolves to `outputs` of a gate step"*; the loader test replaces the compile-time half (RA-M-05) | a pack whose `Gate` step declares `outputs = …` (the loader's `CatalogError::Shape` / `GateOutputs`, the seventh kernel edit) **and** whose downstream step binds `${gate.outputs}` — the row fires on the typed load error, and the per-row test asserts `RK-D-GATE-OUTPUTS` fires and **no `RK-D-DANGLING-*` row does**, so the fixture is rejected for AC-409's reason and not as a dangling reference (a gate declaring no `outputs` under a `${gate.outputs}` binding would be a dangling reference, a different predicate) |
| `RK-D-HOOKS` | error | AC-412 | *"fails on any redkiln-authored hook in `.git/hooks`"* — *redkiln-authored* is a hook file whose first 512 bytes carry the line `# redkiln-hook` (one literal, declared once beside `INIT_UNREAD_WINDOW_RUNS`; matched at line start with a word boundary, never as a substring of another word — a `fork-join` comment does not fire; golden constraint 9) (**RT-51** — routed: the marker is the stage's; no record names how a redkiln-authored hook is recognised because no Phase-1 verb writes one; the alternative — any non-`*.sample` file — is rejected as firing on the operator's own hooks; a dated note on AC-412 is OWED) | a repo with `.git/hooks/pre-commit` carrying `# redkiln-hook` (fires) **and, in `doctor/clean/` (the positive control every per-row test runs second, §7d.13), a `dot-git/hooks/pre-commit` that is an operator hook with no marker and a `# fork-join` comment: no finding** — so the any-non-sample recogniser RT-51 rejected is red on the clean run (a `clean/` with no hooks could not make it red) |
| `RK-D-STORE-ID` | error | AC-413 (as amended: three fields), AC-369's doctor half — the root opens the store and hands `Ctx.store` in as a `Result`, so a `StoreOpenError::Refused` renders **as this row** (exit `3`), never as a pre-row refusal | *"a store carrying repo identity (root commit + common-dir) and schema version … opened from a different repository"*; one store per common dir | a store whose `root_commit` row is another repository's |
| `RK-D-STRAY-STORE` | error | AC-370 (`doctor` half — `s7c` ARMED) | *"a store file discovered under a linked worktree's own `.git` directory is reported by `doctor` as a configuration error, not silently used"* — the row walks `<common-dir>/worktrees/*/` for a `redkiln/store.sqlite3` | a linked worktree whose own gitdir carries `redkiln/store.sqlite3` |
| `RK-D-ROLE-UNRESOLVED`, `RK-D-ROLE-AMBIGUOUS` | error | AC-416 | *"a role resolves to zero cards, or to two cards at equal selector specificity"* — a role→card resolver `rk-cli` authors for `doctor` (`doctor/roles.rs`: cards' `fulfils` and skills' `attach.roles` read from the pack's `cards/` / `skills/` files by an `rk-cli` serde reader whose field set is stated in §7d.13 — the pack schema is 7e's and may re-point it), yielding `Unresolved` (zero cards) or `Ambiguous` (two at equal specificity); **not** the binder's error variants and **not** the kernel's resolver: `Binder::bind` takes a `BindRequest` whose card the caller already chose, `ConfigError` has no unresolved/ambiguous variant, `rk_kernel::resolver` resolves capabilities, and `AuthoredCard` has no `fulfils` field [measured 2026-09-18, `crates/rk-adapter-claude/src/binder.rs`, `config.rs`, `card.rs`; `crates/rk-kernel/src/resolver.rs`]; the card `fulfils` / skill `attach.roles` schema has no record and 7d fixes only the doctor reader's floor (§7d.18 RAISED) | two fixtures |
| `RK-D-CARD-TOOLS`, `RK-D-CARD-TOOLS-ABSENT` | error | AC-417, AC-205 and AC-493 (`doctor` halves — `s7c` ARMED) | *"a card whose `tools` list omits `StructuredOutput` — or a card with no `tools:` key"*; *"the effective tool set is the card's declared list and nothing else"* | two fixtures; the card-validator unit test is 7c's `structured_output_required` |
| `RK-D-TIER-LEAK` | error | AC-418, AC-212 (`doctor` half) | *"any card or slot filler also sets a model or effort"* | a card with `model`; a filler with `effort` |
| `RK-D-TIMEOUT` | error | AC-419 | *"declares no `timeout_secs`, or declares a non-numeric one"* | two fixtures |
| `RK-D-DANGLING-ROLE`, `RK-D-DANGLING-SLOT`, `RK-D-DANGLING-ATTACH` | error | AC-420 | *"a card `fulfils`, a skill `attach.roles`, or a slot filler naming a slot … that no enabled pack declares"* — one row per reference kind | three fixtures |
| `RK-D-DEAD-DEF`, `RK-D-UNFILLED-SLOT` | warning | AC-421 | *"referenced by no enabled pack, or a declared slot is neither filled nor marked optional"* — *dead definition* at warning (AC-057's warning class stands; the AC-102 threshold interaction is the register's §0.3 open row) | two fixtures |
| `RK-D-ENV-DENYLIST` | error | AC-422 (`doctor` half), AC-488 (`doctor` half), AC-217's one list | *"the presence of any name on `rk-adapter-claude`'s committed environment denylist … in the captured environment is reported as a configuration error"* — reads `rk_adapter_claude::env_denylist::first_denylisted` over the **captured map**, never a second literal | a captured map carrying `ANTHROPIC_API_KEY` |
| `RK-D-CLI-VERSION` | error | AC-423 (version arm), AC-226 (`doctor` half) | *"the installed CLI version … differs from the pin"* — the `VersionProbe` (real: `claude --version` via `run_bounded`, 10 s) vs `rk_adapter_claude::config::TESTED_CLI_VERSION` (`config.rs`; the root re-exports nothing); *"naming both the observed and the pinned value"* | a stubbed probe returning `2.1.200` |
| `RK-D-TTL` | error | AC-423 (TTL arms), AC-526 (`doctor` half) | *"either TTL variable differs from the pin, or one TTL variable is set without the other"* — read from the enabled pack's declared pair (7c's `ConfigValues` fields), not the shell | a pack naming one TTL; a pack naming `5m` |
| `RK-D-FACET-UNKNOWN`, `RK-D-FACET-VALUE`, `RK-D-FACET-NEW` | error | AC-424 | *"a facet that does not exist, a value outside that facet's enumeration, or a technology pack adds a new facet"* | three fixtures |
| `RK-D-FACET-FACT` | error | AC-425 | *"a facet has no registered computed fact in the context vector, or a computed fact is used in a `require` row with no registry entry"* — the three contested cases as fixtures | `recent_deploy`, `binary_change`, `dynamism` |
| `RK-D-ADAPTER` | error | AC-486 (`doctor` half) | *"names an adapter that is not compiled in, or names one whose required configuration is absent"* | `unknown-adapter.toml` (`rk-adapter-linear`) |
| `RK-D-UNREAD` | **warning** | AC-426 (owner), AC-472 (as amended), AC-147 (as amended), SM-139 | *"a stage's produced artifact accumulates zero downstream reads over that window"* — **kind-per-stage over the last N runs**, N = `[doctor] unread_window_runs` (D-08; H-4); *"nothing is deleted or rewritten"* | `unread/runs-51.jsonl` flags; `unread/runs-49.jsonl` does not |
| `RK-D-AGENTS-WITHOUT-AGENT` | error | AC-203 (`doctor` half — `s7c` ARMED) | *"a dispatch specification carrying `--agents` with no `--agent <name>`"* over each bound card at configuration time | a card whose binding yields no agent name |
| `RK-D-CACHE-PROFILE` | error | AC-383 (`doctor` half — `s7c` ARMED) | *"a card that declares neither is refused as a configuration error"* | a card with no `cache_profile` |
| `RK-D-MARGIN` | error | AC-377 (`doctor` half — `s7c` ARMED) | a pack margin under two ticks, or a pack with no margin (*"never a code default"*) | a pack with `margin_ticks = 1` |
| `RK-D-HEADROOM-MARGIN` | warning | AC-220 (`doctor` half — `s7c` ARMED) | the last recorded `rate_limit_reading` within the pack's margin — a run started now would be refused before spawn | an event log whose last reading is 99 ticks |
| `RK-D-REBUILT-TIER2` | **ok** | AC-397 (`s7c` ARMED here) | *"the tier-2 emptiness is reported with status `ok` and an explicit 'expected after rebuild' reason AND no rule row reports it as an error or a warning"* — reads the rebuild marker, which is a **sidecar file** `<common-dir>/redkiln/rebuilt` written by `doctor::rebuild` after the rename (its content the rebuild's source `HEAD` sha) and removed by the next successful `run` (**RT-56** — routed, not decided; a dated design §5 / §9.2 note is OWED); **never a table** — `rk-store`'s schema carries no rebuild column or table [measured 2026-09-18, `crates/rk-store/src/migrations/`] and `rk-store` is not edited at 7d (§7d.15); **the sidecar is the row's ONLY discriminator** — the store's tier tables are not one at this pin, because the rebuild's rows land in `t2_event` (the one table `query` reads — the `--rebuild` paragraph below), so *tier-2 emptiness* is not something the row can read off the store; the tier split is RAISED beside RT-56 | a freshly rebuilt fixture store beside its `rebuilt` sidecar — the row emits an `ok`-severity finding with the reason `expected after rebuild` when the sidecar is present and **no finding otherwise**; `ac415_…` counts findings of any severity, and `doctor/clean/` carries no sidecar so the row is silent there |
| `RK-D-WORKTREE-ORPHAN`, `RK-D-NETWORK-FS`, `RK-D-CREDENTIAL-RETAINED` | error / error / error (the severities design §8.4's row fixes: *"error (orphans, network FS) / error (credential)"* — the design outranks the runbook, ADR-0092 clause 2.5) | **ADR-0105 clauses 6 and 7** — no AC id (RT-45) | *"a `doctor` orphans row"*; *"the common dir is not on a network filesystem"*; *"a rule row for credential-shaped values in any retained field"* (`rk_store::retention::looks_like_credential`) | an orphaned worktree entry (read from `<common-dir>/worktrees/<name>/gitdir` on the filesystem — `GitOp` has no `worktree list` op and `rk-git` is not edited); a UNC common dir (Windows); a retained stdout carrying `sk-ant-…` |
| `RK-D-REVIEW-REEXEC` | error | AC-290's `doctor` half (kernel half 7b-ii; **allocated here by DECISIONS-2026-09-18 D18-19**, the operator, 2026-09-19 — O-16) | *"a doctor rule that every finding classed `reproduced` or `mutation-survived` carries either a re-execution record"* (or the `Unverifiable` variant the same clause names) — read off the completed reviews in the event log | `doctor/RK-D-REVIEW-REEXEC/events.jsonl`: one `reproduced` finding with no re-execution record |
| `RK-D-QUARANTINE-LIVE` | **warning** | AC-306's `doctor` half (D18-19) | *"a doctor rule listing live quarantines and their ages"* — one finding per live quarantine, its age from the record's `until` against the invocation's `now` (a listing, not a defect: warning) | `doctor/RK-D-QUARANTINE-LIVE/events.jsonl`: one live quarantine |
| `RK-D-SINK-COUNT` | error | AC-310's `doctor` half (D18-19) | *"`redkiln doctor` errors when a completed review's non-blocking finding count exceeds its sink's record count"* | `doctor/RK-D-SINK-COUNT/events.jsonl`: a completed review with two non-blocking findings and one sink record |
| `RK-D-ADVISORY-LEDGER` | error | AC-530's `doctor` row (D18-19) | *"a doctor rule-table row that errors when the live count"* ≠ `created − retired − dismissed` (AC-530's `advisory_retired` accounting) | `doctor/RK-D-ADVISORY-LEDGER/events.jsonl`: three created, one retired, live count two |
| `RK-D-CORPUS-SHA` | error | AC-314's `doctor` half (D18-19) | *"a doctor rule"* that *"the recorded sha resolves in the corpus repo's history"* — resolved through the wrapper (`GitOp::RevParse` over the configured corpus path); an absent or unconfigured corpus path is the same error naming the path | `doctor/RK-D-CORPUS-SHA/events.jsonl`: a regression run recording a sha no repository holds |
| `RK-D-VERIFICATION-POLICY` | error | AC-455's third instrument (D18-19; the two policy-table rows are 7b-ii's) | *"a `doctor` rule reporting the same at configuration time"* — a spec whose criterion carries `verification: None` under an enabled pack whose policy requires it (the `rk-kernel` policy-table function 7b-ii built, called over the repository's specs) | `doctor/RK-D-VERIFICATION-POLICY/`: a `.redkiln/` with a requiring pack and one spec with `verification: None` |

**Thirty-eight rule ids** (counted off the table's first column with its compound cells split — Halt 7: count it off
the table, and re-count after any edit; thirty-two until 2026-09-19, when D18-19 allocated the six `s7b-ii` gate rows
here), each with a rejecting fixture; the **register-named floor** RA-M-01 asks the
checkpoint to join is the subset a criterion in the register's §9, §12 or §14 names by instrument — `RK-D-GATE-OUTPUTS`, `RK-D-HOOKS`, `RK-D-STORE-ID`, `RK-D-ROLE-UNRESOLVED`, `RK-D-ROLE-AMBIGUOUS`,
`RK-D-CARD-TOOLS`, `RK-D-CARD-TOOLS-ABSENT`, `RK-D-TIER-LEAK`, `RK-D-TIMEOUT`, `RK-D-DANGLING-ROLE`,
`RK-D-DANGLING-SLOT`, `RK-D-DANGLING-ATTACH`, `RK-D-DEAD-DEF`, `RK-D-UNFILLED-SLOT`, `RK-D-ENV-DENYLIST`,
`RK-D-CLI-VERSION`, `RK-D-TTL`, `RK-D-FACET-UNKNOWN`, `RK-D-FACET-VALUE`, `RK-D-FACET-NEW`, `RK-D-FACET-FACT`,
`RK-D-ADAPTER`, `RK-D-UNREAD`, and the six gate rows `RK-D-REVIEW-REEXEC`, `RK-D-QUARANTINE-LIVE`, `RK-D-SINK-COUNT`,
`RK-D-ADVISORY-LEDGER`, `RK-D-CORPUS-SHA`, `RK-D-VERIFICATION-POLICY` — **twenty-nine**, the rows AC-409, 412, 413,
416–426, 486 and 488 demand plus the six AC-290, 306, 310, 314, 455 and 530 name (D18-19); the nine
others (the five `s7c`-ARMED halves' own rows `RK-D-AGENTS-WITHOUT-AGENT`, `RK-D-CACHE-PROFILE`, `RK-D-MARGIN`,
`RK-D-HEADROOM-MARGIN`, `RK-D-STRAY-STORE`; `RK-D-REBUILT-TIER2`; and RT-45's three) are outside the floor and are
asserted by their own tests. `doctor_rule_table_covers_every_register_named_row` asserts the twenty-nine are in
`RULES`; the block greps each from the temp repo's `doctor` output (§7d.17).

- **AC-414** — BINDING, as amended 2026-09-18 (D-10). *"GIVEN any configuration, however large, and a rule table each
  of whose ids has a rejecting fixture (AC-415) / WHEN `redkiln doctor` runs to completion over that fixture corpus /
  THEN every fixture's expected rule fires and zero harness dispatches are issued and zero model tokens are spent."*
  *Verified by:* *"the `doctor` integration test injects a `HarnessPort` fake that fails the test on any dispatch,
  over the AC-415 fixture corpus so that a doctor which does nothing fails the same test; plus a CI assertion that
  the `doctor` code path does not link a live adapter"* — read as *does not construct a `Harness` implementor*.
  `ac414_doctor_over_corpus_fires_every_expected_rule_with_zero_dispatches`: for every `doctor/<id>/`, run the rules
  over a `Ctx` the test builds and a `Ports` whose `harness` is `NoDispatchHarness` (panics on `dispatch`) and whose
  `store` is a counting fake; assert `<id>` is among the fired ids, the fixture child's marker count is 0 and no
  `dispatch` row was appended (the behavioural floor AC-414 names); the corpus loop is the test's and is
  **self-controlled**: before the loop it asserts the enumerated directory set (minus `clean/`) is non-empty and equals
  `RULES`' id set, so an empty or short corpus is red in this test and not only in the floor test (AC-414's own
  *Repaired* note is the hole — *"passed trivially on a doctor that performs no checks"* — and a loop over nothing is
  Halt 8 (g) one level up). `ac414_doctor_module_names_no_adapter_constructor`
  is the lint half (the source scan above). SM-121's counter is the fake's.
- **AC-415** — BINDING, as amended 2026-09-18 (RA-M-01). *"every id has at least one committed fixture configuration
  that the rule actually rejects, and a rule with no rejecting fixture fails the build"* — *"and the id set is
  non-empty and contains every id a criterion names"*. `ac415_every_rule_id_has_a_rejecting_fixture`: `RULES` ⊆
  directory names under `fixtures/doctor/`, each fixture run through **its own rule only** yields ≥ 1 finding, and a
  directory with no rule is an error too (a stale fixture is a lie about the table; **`clean/` is the one named
  exemption** — the positive control every per-row test runs, §7d.13).
- **The per-row tests** are named in §7d.14, one per row family (`ac409_…`, `ac412_…`, `ac413_…`, …); each runs the
  rule over its fixture **and over the clean fixture** `doctor/clean/` asserting no finding — the second half is what
  makes a rule that always fires red.
- **AC-426 / AC-472** — `ac426_ac472_unread_window_49_not_flagged_51_flagged` over the two synthetic logs; N read from
  a `ConfigFile` whose `unread_window_runs = 50`; the same logs under `unread_window_runs = 60` flag neither (the key
  is live, not the constant); `ac426_rule_fire_leaves_write_set_unchanged` runs the rule over `runs-51.jsonl`, asserts ≥ 1 `RK-D-UNREAD`
  finding (the positive half — a rule that never fires leaves the write set unchanged trivially, RA-M-01's shape), and
  digests the store and the tree before and after; `ac426_unread_window_runs_required_no_default` (§7d.9's config tests): `missing-doctor.toml` (no `[doctor]` table)
  and `missing-key.toml` (`[doctor]` present, no `unread_window_runs`) are each exit `3` on `run` and `doctor` alike, and
  no `Default` impl, `unwrap_or(`, or `serde(default` attribute exists on the field in `config.rs` (a source scan over
  the three spellings — `#[serde(default = …)]` is the one-line way to defeat the required-key rule, and the scan sees it).
- **AC-423 / D-11** — `ac423_ac526_cli_version_and_ttl_mismatch_rejected` under a stubbed probe (the seam);
  `ac423_probe_is_one_version_spawn_not_a_dispatch`: `doctor` under the real probe spawns **exactly one** child, its
  argv is `[<harness_binary>, "--version"]` and carries no `-p` (read from the argv the fixture child records in its
  marker file — the marker-file count through the fixture child in `harness_binary`'s place, `s7c` §7c.17's shape), and
  the dispatch counter is 0; the child runs in `exit:0` mode (not the default `hold`, which would stall the probe to
  `run_bounded`'s 10 s bound on every run), and the test asserts the spawn count, the argv and the dispatch counter only
  — the row's finding on an empty version string is `ac423_ac526_…`'s subject under the stub.
- **`doctor --rebuild`** (`crates/rk-cli/src/doctor/rebuild.rs`): `GitOp::Log` over the trailer commits →
  `rk_git::reconcile` (which yields the git-side `State { projects, bare_story_ids, gates, unknown_keys }`, not events
  [measured 2026-09-18, `crates/rk-git/src/reconcile.rs`]) → **`doctor::rebuild::events_from_trailers(&State) ->
  Vec<Event>`** (new here, and stated so two implementers build one shape: `Story:` marks → `stage_transition`,
  `Slice-Verdict:` → `slice_sealed`; **`Gate:` marks are NOT rebuilt as `gate_opened` rows** — the fold's
  runtime-shaped `gate_opened` arm REQUIRES `kind` and `deadline` and DERIVES the token from `(kind, tree_hash,
  deadline, reach)`, ignoring any `token` in the payload [measured 2026-09-18, `crates/rk-kernel/src/workflow/fsm.rs`],
  while the `Gate:` triple carries `(token, tree_hash, deadline)` and no `kind` or `reach`, so a row built from a trailer
  is either `MalformedPayload` or derives a token that is not the trailer's; the rebuild records each `Gate:` mark as the
  `rk_git::reconcile::GateRecord` fact it is (commit sha, token, tree hash, deadline) and lists them as *unresolvable
  from git alone* until a record widens the triple or makes the token the fold's key — RAISED in §7d.18 beside the design §5.4
  answer-value row, and `ac427_…`'s synthetic repository carries **no gate**; tier-1 kinds only, payloads built from
  the trailer values and the commit sha ***— and, for `Story:` marks, from a THIRD source, amended 2026-09-21, Step 7d Slice 4 cycle 9, on the operator's decision of that date.*** *The prescription as written cannot be built, and the measurements are these. `fsm::apply`'s `StageTransition` arm makes **`to` mandatory** — `required_str(event, "stage_transition", "to")` — while `from` is optional and defaults to the fold's running stage [measured 2026-09-21, `crates/rk-kernel/src/workflow/fsm.rs`:1318-1325]. `ParsedTrailer::Story` is `{ project, slug }` and `ProjectState.stories_committed` is a `BTreeSet<String>` of slugs: **no stage anywhere**, and no commit sha either [`crates/rk-git/src/trailer/grammar.rs`:93-137, `crates/rk-git/src/reconcile.rs`:79-85]. Reading the item record at the trailer commit's TREE is foreclosed by this stage's own floor: `GitOp`'s read set is `RevParse | Status | Log` with no blob read, its doc says *“anything not here is a successor record”*, and §7d.15 declares `rk-git` not edited [`crates/rk-git/src/git_exec.rs`:27-52]. **So the third source is `.redkiln/items/<slug>.toml`'s `stage` key in the checkout** — the only in-repo source of a stage at this pin, and the value RT-50 already makes authoritative for an item everywhere else. **Its limit travels with it: only the LAST transition per item is recoverable**, so the rebuild emits ONE `stage_transition` per committed item, anchored on that item's last `Story:` commit, and an item carrying two marks rebuilds to one row; `ac427_…`'s repository therefore carries one mark per item and states the limit in its doc comment. A mark whose record is absent or declares no `stage` is **listed unresolvable, never coerced** (AC-165), exactly as a `Gate:` mark is. The signature widens with the source — `events_from_trailers(&[CommitTrailers], &State, &ItemStages) -> Rebuilt` — because `&State` alone supplies neither the `to` nor the commit sha this sentence names. **This is the third §7d.8 prescription this slice that could not be built as written**, after `fulfils`-on-a-card (cycle 4) and `PackExtras` (cycle 5 batch 2), and it takes the same form of correction. A dated design §5.4 note is OWED and is RAISED in §7d.18 beside the `Gate:` one*, no tier-2 row, which is exactly what `RK-D-REBUILT-TIER2` reports as `ok`) →
  `rk_kernel::workflow::fsm::fold` per item → `fsm::stage_action` (the stage-level next action `status` prints;
  `fsm::next_action` needs a `Selection` a git-only rebuild does not have and is not called), written to a **new store
  file and renamed over the old** (ADR-0105 clause 7: *"a rebuild writes to a separate file and renames"* — through
  `rk_store::conn::Writer::open(&staging)` over `<common-dir>/redkiln/rebuild/` (the writer places the file at
  `<staging>/redkiln/store.sqlite3`), `rk_store::schema_lint::all()` applied and `rk_store::identity::initialise` stamped
  with the repository's identity, then `std::fs::rename` over `store.sqlite3` — `SqliteStore::open` derives its path
  from `--git-common-dir` and cannot target a second file). **How the rebuilt `Event`s are WRITTEN into the staging
  store, stated once**: `doctor::rebuild` inserts each rebuilt `Event` through `Writer::transaction` with `rk-cli`'s own
  `INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2, ?3)` — `anchor_sha` the trailer commit's sha —
  **duplicating `SqliteStore::append`'s statement**, which is the only event `INSERT` on the tree (`append` writes
  `t2_event` with `anchor_sha` NULL; `EventStore::rebuild` on `SqliteStore` is a typed `Capability::Rebuild` refusal;
  `Writer` exposes `transaction` and nothing else; the `t1_stage_transition` / `t1_gate` / `t1_slice_seal` tables are
  written by nothing [measured 2026-09-18, `crates/rk-adapter-sqlite/src/lib.rs`, `crates/rk-store/src/conn.rs`,
  `migrations/0001_initial.sql`]) — **disclosed as a second copy of one statement** and RAISED (§7d.18) for an
  `rk-adapter-sqlite` `open_at(path)` / `Capability::Rebuild` implementation in a record, a closed-crate edit this stage
  does not make; and because `SqliteStore::query` reads `t2_event` alone, `stage_action` on both sides of `ac427_…`
  reads the same table (a rebuild that wrote the `t1_*` tables would make every item initial for a layout reason, and
  `RK-D-REBUILT-TIER2`'s *tier-2 emptiness* is not a table fact at this pin — the row reads the `rebuilt` sidecar only,
  RT-56). **For `--rebuild` the root does NOT hand the open handle
  in**: `Ctx.store` is `None` / dropped before the rename (the `SqliteStore` and every `Reader` are dropped, which
  checkpoints and deletes the WAL — `Writer` holds a `Mutex<Connection>` in WAL mode [measured 2026-09-18,
  `crates/rk-store/src/conn.rs`], and on Windows `std::fs::rename` over a file another handle holds open without
  `FILE_SHARE_DELETE` fails with a sharing violation), the staging store's `Writer` is dropped, then `store.sqlite3` (and
  any `-wal` / `-shm` sidecar) is renamed over, the `rebuilt` sidecar written (the `RK-D-REBUILT-TIER2` marker), and the
  rebuilt store re-opened for the identity check **by the ROOT** — `adapters.rs`'s open path, the module the block's
  `doctor` filter does not admit — after `verbs::doctor` has returned; **neither `doctor/rebuild.rs` nor
  `verbs/doctor.rs` may name `SqliteStore::open`** (the block's D-10 ban is module-path filtered to `doctor`, and its
  filter regex admits both `src/doctor/*.rs` and `src/verbs/doctor.rs` — a re-open placed in either fails the ban on a
  correct binary); `ac427_…` runs on Windows and is red on a
  sharing violation.
  - **AC-427** — **verb + direction at 7d; fidelity at `s8`** (D-21). *"GIVEN a repository whose tier-1 facts live in
    git trailers, with the store deleted or absent / WHEN `redkiln doctor --rebuild` runs / THEN a store is
    reconstructed from git alone, and the computed next action for every item in the pinned corpus is identical to
    the recorded one."* `ac427_rebuild_from_git_alone_matches_recorded_next_action` over a **synthetic** trailer
    repository (three items, two `Story:` stage marks, one `Slice-Verdict:` seal — **no gate**, the `Gate:` limit
    above) whose pre-delete store is populated through the normal write path — the test appends the
    `stage_transition` / `slice_sealed` rows beside the `Story:` / `Slice-Verdict:` commits it makes through the wrapper
    (no 7d verb writes those trailers) — **never by a prior `--rebuild`** (a pre-delete store that `--rebuild` itself
    produced would make both sides one function of one input, a comparison with no failure mode); the test asserts the
    pre-delete `stage_action` set is non-initial for at least one item, then deletes the store, rebuilds, and compares
    `stage_action` per item to the pre-delete reading. The *"pinned corpus"* and *"CI job"* halves are the local-only corpus suite's (`s5` §5.2;
    owner AC-184) and are not claimed.
  - **AC-429** — BINDING (`rk-git`). *"THEN no git object, ref or working-tree file has been written: reconciliation is
    git → store, never the reverse."* `ac429_rebuild_writes_no_object_ref_or_file`: the AC-174 fingerprint shape
    (`rk-git`'s `reconcile_mutates_nothing_in_git`), preceded by two positive controls — the fingerprint set is
    non-empty and names `HEAD` and at least one ref, and the rebuild happened (the store file was recreated and holds
    ≥ 1 tier-1 row after the run) — so a `--rebuild` that does nothing is red before the negative is read (Halt 8
    (d)/(g)); then every file under `.git` **except `.git/redkiln/**`** (the store, its `rebuild/` staging directory,
    the `rebuilt` sidecar and `hooks-empty/` — redkiln's own files under the common dir, not git objects; a rebuild
    renames a new store over `store.sqlite3` by design, so a fingerprint that included them would be red on a correct
    binary and green only through an exclusion nobody reviewed) digested before and after, `HEAD`, the ref list and the
    tree hash compared, **with the positive control that `.git/redkiln/store.sqlite3`'s digest DID change** (the rebuild
    happened) — the fingerprint IS the write-op assertion (`GitExec` has no journal to read;
    `rk-git`'s own AC-174 test uses the same fingerprint alone); AC-429's clause is *"no git object, ref or
    working-tree file"*, which the store is not.
  - **AC-397** — `ac397_rebuilt_store_tier2_emptiness_is_ok_expected` over the rebuilt fixture.
  - **AC-366** — *"a CI job that clones, rebuilds and diffs tier-1 projections"* — a CI run; ARMED (§7d.12).
- **AC-290, AC-306, AC-310, AC-314, AC-455, AC-530 — the six `s7b-ii` gate rows** (D18-19; O-16 closed 2026-09-19) — one
  test per row, each over its `events.jsonl` (or `.redkiln/` skeleton) fixture and over `doctor/clean/`:
  `ac290_reproduced_finding_without_reexecution_record_doctor_row`, `ac306_live_quarantines_listed_with_ages_doctor_row`
  (warning; the age is `now − until`'s sign and magnitude off a supplied `now`, no clock read),
  `ac310_sink_count_below_finding_count_doctor_row`, `ac530_advisory_ledger_imbalance_doctor_row`,
  `ac314_regression_corpus_sha_unresolvable_doctor_row` (the corpus path is a `ConfigFile` key the fixture points at a
  temp repository; the rejecting fixture's sha resolves nowhere), `ac455_verification_none_under_requiring_pack_doctor_row`
  (calls the 7b-ii policy-table function; the must-pass control — the same spec under a pack that does not require
  verification — yields no finding). The kernel halves of all six stay 7b-ii's (CROSS-LISTED, §7d.18).
- **AC-438's doctor instance** — `ac438_doctor_error_rule_exits_config_error`: the binary over `doctor/RK-D-ADAPTER/`
  exits `3`; over `doctor/clean/` exits `0`; over a warning-only fixture exits `0`.
- **The journal rows' READINGS, taken 2026-09-21 at Step 7d Slice 4 cycle 5** — because the sketches above name an event vocabulary that does not exist, and two implementers reading them build different fixtures and different rules. **`EventKind::ALL` is CLOSED at 38** [measured 2026-09-21, `crates/rk-types/src/event_kind.rs`:145-184], and of the kinds these rows' fixture sketches imply, `usage.reading`, `findings.recorded`, `finding.sunk`, `review.completed`, `quarantine.opened`, `advisory.created`, `advisory.ledger` and `regression.run` are **none of them**. What the rows read instead, each off design §5.4 and the kernel's own producers: **`RK-D-HEADROOM-MARGIN`** → the LAST `rate_limit_reading`'s `five_hour` utilisation, *inside the margin* being `100 − ticks <= margin`; **`RK-D-REVIEW-REEXEC`** → a `review_block` row whose `computed_class` is `reproduced` and whose **`re_execution_id` is null** — that member is `ac307_members`' own (`crates/rk-kernel/src/review/confirm.rs`) and is exactly what AC-290 asks after; **`RK-D-QUARANTINE-LIVE`** → a `quarantine_granted` not closed by `quarantine_removed` or `quarantine_expired`, with `until` **as unix seconds** (design §5.4 names the member and no record fixes its format — a READING) and the age `now − until` off `Ctx::now`, the root's one wall-clock read; **`RK-D-ADVISORY-LEDGER`** → the half of AC-530's `live_sink == created − retired − dismissed` the journal can settle alone, since the live count is the STORE's: every `advisory_retired` / `advisory_dismissed` names a finding the sink held, and names it once; **`RK-D-SINK-COUNT`** → AC-310's `findings_in == blocks + sink_records`, with `findings_in` read off the review's `artifact.write` row of kind `Findings` through a **`findings` member — a READING under RT-48**, which already routes these rows' payloads and records that a dated design §5 note widening them is OWED; nothing else in the journal carries a review's finding count. **And `RK-D-CORPUS-SHA` reads neither a journal row nor a `ConfigFile` key**, though this section names both: there is no regression `EventKind`, and `ConfigFile` is `deny_unknown_fields` over `[adapters]` / `[packs]` / `[doctor]` with no corpus key [measured 2026-09-21, `crates/rk-cli/src/config.rs`]. It reads `.redkiln/regression.toml`'s `corpus_sha` and asks **git** whether it resolves (`rev-parse --verify`), because a 40-hex string is not a commit and only a repository can say so. 7e's pack and record schemas may re-point any of these.
- **What is NOT a row and why**: **SM-123's `doctor_rule_fired`** (O-12, answered 2026-09-19: minted by ADR when Step 8 first needs the measure) — no event is written per rule (**AC-455's
  and AC-530's rows**, RAISED as O-16 at stage-open, ARE rows since D18-19 — the table above); **AC-147's** own row is `RK-D-UNREAD` (it is AC-426's twin, as
  amended); **AC-395's `redkiln cost`** and **AC-372's renderers** — `s7c` ARMED them here and they are ARMED on to
  Step 8 with the cost unit record RT-19 still OWED (§7d.12); **AC-398's drift report** — ARMED on (§7d.12).

### 7d.9 `init`

**Non-interactive, eager, idempotent.** `redkiln init` reads no stdin (D-15), prompts nothing, and does exactly two
things: writes `.redkiln/config.toml` with the bytes below **if and only if the file is absent** (a present file is
left untouched, byte for byte, whatever it says — ADR-0107 decision 2: *"a second `init` on an initialised repository
changes nothing and exits 0"*), and calls `rk_adapter_sqlite::SqliteStore::open(&exec, &worktree)`, which creates
`<git-common-dir>/redkiln/store.sqlite3` and stamps its three identity rows on first open and re-reads them on the
second (AC-437's *"identical store identity rows"*; D-14). It installs **no hook** (AC-412) and creates **no git
ref**. The scaffold, verbatim — these bytes are the fixture `ac437_init_scaffold_bytes_pinned` compares against, LF
line endings, no trailing whitespace, one trailing newline:

```toml
# Written by `redkiln init` (Step 7d). Step 7e fills [packs].enabled; every other key is read at startup
# (AC-485/AC-486) and by `redkiln doctor` (AC-426). A second `init` leaves this file untouched (ADR-0107 d2).

[adapters]
harness = "rk-adapter-claude"
store = "rk-adapter-sqlite"
toolchains = ["rk-adapter-toolchain-node", "rk-adapter-toolchain-rust"]

[packs]
enabled = []

[doctor]
unread_window_runs = 50
```

`INIT_UNREAD_WINDOW_RUNS: u32 = 50` is **one literal** in `crates/rk-cli/src/verbs/init.rs` (H-4; D18-5), and the
scaffold is rendered from it, so the block's two greps — the literal's declaration and the key in the written file —
read one value. The comment header is part of the pinned bytes.

- **AC-437** — BINDING (`rk-cli`), as amended 2026-09-18 (D-14). *"GIVEN a repository / WHEN `redkiln init` is run
  twice in succession / THEN the second run makes no change — identical tree, identical store identity rows — and
  exits 0."* *Verified by:* *"an integration test asserting an empty `git status --porcelain` diff, unchanged store
  identity rows and exit 0 after the second `init`"* — with the scaffold fixed verbatim.
  `ac437_init_twice_identical_tree_and_identity_rows`: spawn the binary twice with `Stdio::null()` on stdin (D-15);
  both exit `0`; the porcelain listing after the second equals the listing after the first (`?? .redkiln/` on both);
  the working-tree digest is equal; `rk_store::identity::Identity { root_commit, common_dir, schema_version }` read
  through `rk_store::conn::Reader::open(&common_dir)` and `rk_store::identity::read(reader.conn())` after each is equal
  (three fields — RA-M-07; `SqliteStore` exposes no identity getter). `ac437_init_scaffold_bytes_pinned`: the written
  file's bytes equal `tests/fixtures/init/config.toml.expected` (the block above). Red by a second `init` that rewrites
  the file (a changed mtime is not the assertion — bytes and porcelain are), by one that re-stamps identity — observable
  only as a changed store-file sha256 across the second `init` (the three rows are deterministic — `root_commit`,
  `common_dir`, `schema_version` — and equal either way, so the row-equality assertion alone cannot detect it), which
  the test also compares after the `Reader` is dropped — and by a
  scaffold that drifts from the amendment's key set.
- **AC-531** — BINDING (`rk-cli` (`init`)), as amended 2026-09-18 (D-14: scoped to the working tree, refs and hooks).
  *"GIVEN a repository on which `redkiln init` has run / WHEN the files, directories, git refs and hook-free entries
  it created are enumerated / THEN every name it created outside the repository's own `.redkiln/` directory matches
  the committed lint regex `^rk-[a-z0-9][a-z0-9-]*$` (AC-506's regex), and `init` refuses to create a name that does
  not AND a pre-existing name matching the prefix that `init` did not create is reported, never overwritten."*
  *Verified by:* *"a naming lint over `init`'s manifest of created names with a red fixture (an unprefixed name) that
  must fail; an integration test enumerating the tree diff after `init` and asserting every created name matches the
  regex; a fixture with a pre-existing `rk-` name asserting the report and no overwrite."* `init` builds a
  `CreatedNames` manifest (working-tree paths outside `.redkiln/`, refs, hooks — **empty at 7d**, because the scaffold
  is inside `.redkiln/` and the store is in the common dir, listed separately and unlinted) and runs the lint over it
  before writing. `ac531_init_created_names_match_regex`: the lint over a synthetic manifest — `rk-foo` passes,
  `foo`, `redkiln-foo`, `rk-Foo` and `rk-` fail (the regex's own edges), and the real tree diff after `init` (working tree, `refs/`, `.git/hooks`) enumerates **exactly**
  `{.redkiln/, .redkiln/config.toml}` — the two created paths inside the exempt directory are the enumerator's positive
  control — and nothing outside `.redkiln/` (an assertion that would go red the day `init` creates one, and one an
  enumerator that lists nothing cannot pass).
  `ac531_preexisting_rk_name_reported_not_overwritten`: a pre-existing `rk-notes/` directory in the working tree;
  `init` exits `0`, reports it on stderr as pre-existing and not `init`'s, and its bytes are unchanged. **AC-506 is
  Phase 2** (`rk-plugin-gen`) and is excluded; the regex is one string, declared once in `rk-cli` and quoted here.
- **AC-412** — BINDING. `ac412_init_installs_zero_hooks`: `.git/hooks` has the same entry set before and after `init`
  (git's `*.sample` files are not `init`'s); the `doctor` row is `RK-D-HOOKS`.
- **AC-413** — BINDING (`rk-store`), as amended 2026-09-18 (three fields). `ac413_store_opened_from_other_repo_refused_zero_rows`:
  a store created in repository A, its file copied under repository B's common dir; `redkiln status` in B exits `3`
  naming the identity mismatch, and the store file's sha256 is unchanged across the refused `status` (zero writes — a
  spawned binary has no counting seam; the zero-reads half of the THEN is `rk-store`'s own AC-179/AC-180 instrument at
  Step 6) — the read half is `rk-store`'s `open` refusing before any query.
- **The config fixtures** (`tests/fixtures/config/`): `missing-doctor.toml` (no `[doctor]` — exit `3`;
  `ac426_unread_window_runs_required_no_default`); `missing-key.toml` (`[doctor]` present, no `unread_window_runs` —
  exit `3`, the same test's second arm); `unknown-key.toml` (`[doctor] unread_window_runss = 50` — exit `3`
  under `deny_unknown_fields`); `dual-write.toml` (`[compat] dual_write = true` — exit `3`; AC-474's
  `ac474_config_surface_snapshot_has_no_dual_write_key` also snapshots the `ConfigFile` schema's full key list and
  asserts no key matches `dual|compat|legacy`); `unknown-adapter.toml` (AC-486) — **the §7d.9 scaffold byte-for-byte with
  ONE line changed**, `harness = "rk-adapter-linear"` in place of `"rk-adapter-claude"`, so it is a complete `ConfigFile`,
  the load succeeds, the rule table runs, and `RK-D-ADAPTER` is the ONLY row that can fire (the block's `grep -q
  RK-D-ADAPTER` depends on it); `missing-store.toml` (AC-486's GIVEN's second disjunct — *"names one whose required
  configuration is absent"*) — the scaffold with the `store =` line removed, exit `3` naming the absent key
  (`ac486_…`'s second arm, §7d.5).
- **What `init` does not do**: it does not enable a pack (7e's `[packs] enabled` edit — E6/E7), does not write
  `facets.toml` or any pack file, does not touch the product repository in any checkpoint line (D18-9), and does not
  create a name it would have to lint.

### 7d.10 The signal path

**One dependency, one handler, one token** (**D18-2**): `ctrlc = { version = "3", features = ["termination"] }` in
`rk-cli`'s manifest only (3.5.2 at 2026-09-18; `>= 1.0`, a caret row under ADR-0100 decision 2; MIT/Apache; the
`termination` feature is what makes `SIGTERM` / `SIGHUP` reach the handler on POSIX — ctrlc's `lib.rs`: *"Handling of
`SIGTERM` and `SIGHUP` can be enabled with `termination` feature"* — and without it a `SIGTERM` would end the process
through the default disposition at 143, the code the table refuses to adopt; on Windows the handler fires for CTRL_C,
CTRL_BREAK and CTRL_CLOSE regardless). Its `windows-sys` edge is `"0.61"` [measured 2026-09-18 from the crate's
`Cargo.toml`], which **unifies** with `rk-containment`'s `=0.61.2` pin — **no second version and no `multiple-versions`
warning is expected**; a `cargo deny` warning naming `windows-sys` after Slice 1 is a defect to investigate, not an
expected row. Its `cfg(unix)` edge is `nix = "0.31"` (`signal` feature only) [measured 2026-09-18, the same manifest] — a
transitive 0.x that enters `Cargo.lock` at Slice 1 for the POSIX arm (`Cargo.lock` is platform-agnostic) and is not a
direct row (ADR-0100 decision 2's pin rule is for direct rows); it is expected, stated so the lock diff is not misread
against the workspace manifest's rk-containment comment that *`nix` is not taken*; and its `cfg(target_vendor =
"apple")` edge is `dispatch2 = "0.3"` [measured 2026-09-18, the same manifest] — a second transitive 0.x that enters
`Cargo.lock` for the same reason and is likewise not a direct row. In `main.rs`, before any verb runs: `let token = CancelToken::new(); let t = token.clone();
ctrlc::set_handler(move || t.cancel())` — the handler does **nothing but fire the token** (no store write, no exit, no
print: a handler runs on a foreign thread and the store lock may be held). **`ctrlc::set_handler` may be called once
per process** (`Error::MultipleHandlers` on a second call [measured 2026-09-18, ctrlc `src/lib.rs`]), so `main.rs` is its
only call site and **no in-process test installs it**; `signal.rs` exposes the closure as a `pub fn` (of `rk-cli`'s
library target, `src/lib.rs` — §7d.15: an integration test under `tests/it/` cannot link a `[[bin]]` or name a
`pub(crate)` item) the integration test calls directly. The token is cloned into `ClaudeHarness::new(config, token)` at construction (ADR-0099 decision 2:
*"cloned into the adapter at construction (never per call)"*); the adapter's polled arm turns a fired token into
`Err(StepError::Cancelled { by: CancelledBy::Signal })` (`s7c` §7c.10's rule: *"`by: Signal` on the token arm"*), the
kernel records it, and `run` exits **`13`**. **Outside a dispatch** — the token fires while `run` is reconciling,
reading the store, or between steps — the root checks `token.is_cancelled()` at each step boundary and returns
`Ok(Exit::SignalOutsideDispatch)`, **`130`**, with *"interrupted; nothing after <step> ran"*, having written nothing
partial (every store write is one row; every git write is one commit). The two named alternatives — an
`rk_containment::signal` wrapper (which would need an ADR-0127 annex, since 0127 decision 2 fixes that crate's two
surfaces) and deferral to Step 8 — were named to the operator and not taken (D18-2).

**No console signal is delivered by any test at 7d, and no closed crate gains a sender for one.** An earlier draft of
this file added a `#[doc(hidden)]` `send_ctrl_break` to `rk-containment`'s one `unsafe` module; it is **not built**:
`GenerateConsoleCtrlEvent` is `unsafe` FFI, `unsafe` is confined to `rk-containment` (ADR-0127; the workspace
`unsafe_code = "deny"`), ADR-0127 decision 2 enumerates that crate's surface as exactly two, and D18-2 itself rejected a
signal wrapper there because *"a console-control handler is a third surface and would need an annex to a record
accepted yesterday"* — a runbook may not widen an accepted record's surface ahead of the record. Two further reasons the
process-level test is not authored here: under `cargo test` the test binary is not in its own process group, so a
CTRL_BREAK to *own_group* reaches `cargo`, libtest and the Git Bash running this block (Halt 6's shape); and the
fixture child spawned by the adapter shares `redkiln`'s console and installs no handler, so a CTRL_BREAK to `redkiln`'s
group also ends the child (`STATUS_CONTROL_C_EXIT` → `Outcome::OsKilled`) and the adapter's `cancelled_by` becomes a race
between the token arm and the child-death arm — flaky by construction. **The deterministic instrument for the two rows
is in-process**, and the console-signal delivery is **ARMED** (§7d.18) to a record that adds a sender, or to Step 8's
operator-driven run.

- `signal_handler_fires_cancel_token` — the `signal.rs` closure called directly on a test token: `is_cancelled()` flips;
  the closure returns; it wrote nothing (a counting store fake sees zero appends), printed nothing (stdout/stderr
  captured empty) and did not exit. Red by a closure that exits, prints or writes before firing. **The weaker red is
  stated as the instrument's limit**: this does not prove the OS delivers the signal to the closure — that is
  `ctrlc`'s contract and the ARMED process arm's.
- `signal_during_dispatch_is_cancelled_by_signal_exit_13` — the integration test RA-M-04 names, **in-process**: the
  verb function with a real `ClaudeHarness` (`harness_binary` pointed at `rk-fixture-child-claude` in `hold` mode —
  alive until killed; `s7c` §7c.17) and a `CancelToken` the test fires from a thread after the fixture child's marker file appears under
  `RK_FIXTURE_MARKER_DIR` (`s7c` §7c.17's observable — the store `Mutex` is held by the kernel for the whole `dispatch`
  call [measured 2026-09-18, `crates/rk-kernel/src/runtime/activity.rs`: `store_guard()` taken around
  `harness.dispatch`], so a store poll for the `dispatch` row would miss the window between the append and the lock
  and block until the `hold` child dies, which it never does without the token — red for the wrong reason at the 30 s
  bound); asserts, after the verb returns, the `dispatch` row and that the `dispatch_completed` row carries
  `cancelled_by: signal`, the fixture child is dead (`is_alive(pid)` false
  — the adapter killed the tree through the job) and the verb returned `Ok(Exit::Cancelled)` (`13`). Bounded at 30 s
  (Halt 5). Red by `Ok(Exit::SignalOutsideDispatch)` (the root returned without letting the adapter record), by a live
  child, or by `cancelled_by: timeout`.
- `signal_outside_dispatch_exits_130_writes_nothing_partial` — the verb function called with a token fired **before**
  `run` starts (no env toggle — `RK_TEST_PREFIRE_CANCEL` is **not** used; AC-474 forbids it; the token is a function
  argument); returns `Ok(Exit::SignalOutsideDispatch)`; the store gained at most the `reconcile_started` row; no commit.
- `adr0118_acquire_returns_halt_on_poisoned_lock` — the in-process twin of the ADR-0118 clause 5 fixture (Slice 2):
  a thread panics while holding the store `Mutex`, is joined, and the poisoned `Arc` is handed to `store::acquire`,
  which returns `Err(Halt::PoisonedLock { lock: "store", .. })` — **never a guard, never a `StepError`**; the source
  scan bans the recovery spellings. Red by `Ok(guard)` (a recovery) or by a panic (exit 101's cause).
- `adr0118_poisoned_store_lock_halts_run_exit_70` — the ADR-0118 clause 5 fixture at the verb (Slice 3): the verb
  function called with the same poisoned `Arc`; `run` reaches its first acquisition and returns
  `Err(Halt::PoisonedLock)`; the library's two `pub` functions (`src/lib.rs`, §7d.15) — `code_for(&Result<Exit, Halt>) -> i32`, which the
  test asserts returns **`70`**, and `render_halt(&Halt) -> String`, which the test asserts names the lock (*"the store
  lock is poisoned (store)"*) — are the surface the thin `main.rs` exits through; the panic hook's predicate
  `is_lock_halt(&PanicHookInfo) -> bool` is asserted true on **the payload a real kernel-side poison produces** — the
  test poisons the `Arc<Mutex<dyn EventStore>>`, calls `Runtime::store_guard` under `std::panic::catch_unwind` **in the
  test** (the `catch_unwind` ban is over `src/` only) and hands `catch_unwind`'s payload to `is_lock_halt` — and false on
  any other payload; a test that built its own `String` around `rk_kernel::runtime::LOCK_HALTS` (the `pub const`, §7d.4)
  would never observe the format the kernel's `.lock().expect(LOCK_HALTS)` actually produces, and a shipped payload of a
  different shape would leave the hook at 101 with every test green. **The process-level `70` is not observable**: no fixture can poison a spawned process's lock without a
  seam AC-474 forbids, so the row is asserted through these functions and its presence in `Exit::ALL` (§7d.4). Then:
  **no `StepError` value was produced**, **no recovered guard** (the source scan), **no subsequent activity** (the
  fixture harness's marker count is 0; the store's row count is what it was). Red by
  `unwrap_or_else(PoisonError::into_inner)` anywhere on the path (the run continues and dispatches), by a plain
  `.lock().unwrap()` (a panic — exit 101's cause — not `Err(Halt)`), and by a halt that appends a row on the way out
  (ADR-0118's rejected third option).
- **What is NOT claimed**: the process-level console-signal delivery on Windows (CTRL_BREAK to a `redkiln` spawned with
  `CREATE_NEW_PROCESS_GROUP`, the shape a future sender would take) and the POSIX arm (`SIGTERM`, `SIGINT`) — both
  **ARMED** (§7d.18): the first to a record adding a sender or to Step 8's operator-driven run, the second to a recorded
  ubuntu CI run as 7c's AC-224 POSIX arm is; the box guard at the top of the block keeps the Windows arm's name from
  passing under the POSIX one.
### 7d.11 `redkiln-migrate` v0

**"v0" is defined (PIA-002), and this is the definition** — the register's §13 phase note as amended 2026-09-12 (E13):
*"What Phase 1's Step 7d owes is a v0 — the binary exists (AC-441); the structural assertions AC-441/AC-466/AC-474; the
report schema AC-448/AC-468; the run contract AC-447/AC-450/AC-522; and a dry run over a small fixture — with a
checkpoint line that invokes `redkiln-migrate`."* Everything corpus-bound is **Phase 2's acceptance set** and runs in
the local-only `corpus` suite (`s5` §5.2) when it runs at all; this stage claims the structural and dry-run rows only,
each below by clause, and excludes the rest by id (§7d.18). **E13 has landed** — the 153-line file said the AC-468 and
AC-475 errata were *"raised, not resolved"*; the register's phase note carries the amendment (*"AC-468 splits: its
schema conformance of the differential report is Phase 1's … AC-475 is removed from the corpus-bound list and splits
the same way"*), so this file claims both on exactly those readings and raises nothing about their placement.

**The binary** (`crates/redkiln-migrate/src/main.rs`): `redkiln-migrate --target <path> [--dry-run]` and `--version`;
`clap`, `anyhow` (ADR-0102 decision 6 — the only two crates that may), no other verb. Its exit values are **RT-53**
(routed, not decided; unpinned by any register criterion — AC-438 governs `redkiln` only, and DECISIONS RT-35 says the
migrator *"has its own smaller table"*): `0` success, dry run and the AC-450 no-op; `2` clap usage; `3` the AC-522
dirty-target refusal and the AC-447 / AC-448 aborts before writing; `1` any other `anyhow` error — asserted by
`refuses_dirty_target` (`3`) and the block's dry run (`0`), and not snapshot-pinned until a record asks for it. **Its dependencies reach no
`rk-ports`, no `rk-kernel`, no `rk-adapter-*`** — it converts files and commits once; it dispatches nothing and has
no port to dispatch through (§7d.15's manifest; the block's `cargo metadata` walk). **Its run contract**, in order:
(1) `version::assert_at_startup`; (2) H-3's bootstrap: a `GitExec` whose `hooks_empty` is a `tempfile::TempDir`, through
which `rev-parse --show-toplevel` and — because this crate may not depend on `rk-adapter-sqlite` (AC-466) — the one
`rev-parse --git-common-dir` its worktree path needs are read (the ADR-0091 divergence, named in §7d.18 RAISED), then the
working `GitExec` under `<git-common-dir>/redkiln/hooks-empty/`; then `GitExec::run(GitOp::Status, target, ["status",
"--porcelain"])` — non-empty is a refusal **before any write**, naming every dirty path (AC-522; no porcelain helper exists in `rk-git` — this is the
one call site, in `run_contract.rs`); (3) the branch `rk-migrate/<target-HEAD-short>` and the worktree
`<git-common-dir>/redkiln/migrate-wt/` — on a **first** run through `GitExec::run(GitOp::WorktreeAdd, &target,
["worktree", "add", "-b", &branch, wt_path, "HEAD"])`, because the branch does not exist yet and `rk_git::worktree::add`
runs `git worktree add <path> <branch>`, which needs an existing commit-ish (`GitOp` has no branch or update-ref op and
`rk-git` is not edited [measured 2026-09-18, `crates/rk-git/src/git_exec.rs`, `worktree.rs`]), after the same
`path.exists()` refusal `worktree::add` performs; on a later run, when the branch exists, through `rk_git::worktree::add`
— **re-created, never reused**: a previous run's worktree is torn down first through `GitExec::run(GitOp::WorktreeRemove,
&target, ["worktree", "remove", "--force", wt_path])` — the same raw-`run` form this step already uses for `worktree add
-b` (no `rk-git` edit; `WorktreeRemove` is on the enum) — because `rk_git::worktree::remove` passes no `--force` and a
killed run (`idempotent_across_kill`; `ac522_…`'s stale `migrate-wt/`) leaves written, uncommitted files git refuses to
remove (*"contains modified or untracked files, use --force to delete it"*, rc=128 [measured 2026-09-18 on this box];
`remove_dir_all` + `add` fails too — *"missing but already registered worktree; use add -f or prune"* — and `GitOp` has
no prune op), so a re-run through the unforced helper exits RT-53's `1` before `add` and both tests are red for the
wrapper's reason; `worktree::remove`'s typed refusal stands for the non-forced case, and the `--force` is licensed only
because the tree is a run-owned worktree re-created never reused (ADR-0105 clause 6), never the user's checkout; then
`add` refuses an existing path (D18-11; ADR-0105 clause 6); (4) the read: every legacy file under `.bklg/` and `.redkiln/` of the target, through `legacy/` (the only `yaml_serde` users),
into the six typed sidecar values (`rk_types::companion::{IntegrationCompanion, GroundingCompanion,
DecompositionCompanion, ReviewCompanion, LedgerCompanionSidecar}` and `rk_types::artifact::ledger::AcLedger` — the census
kind's type lives under `artifact::ledger`, imported privately by `companion.rs` [measured 2026-09-18]), each validated in-process against the
schema `rk_types::companion::write_companion(kind)` returns — that function is a schema lookup, not a writer [measured
2026-09-18, `crates/rk-types/src/companion.rs`] — and serialised through `rk_store::canon::canonical`, and opaque prose
for the eight `PHASE_2_KINDS` (AC-143; the treatment is ADR-0075 clause 6's); free-form gate results and diff prose →
opaque + a loss marker (AC-477's carry; AC-475's tripwire). **The report's `kind` column** carries the fourteen companion
kinds by their wire names (`_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger`, `ledger`; `closure`,
`design`, `design-review`, `slices`, `storymap`, `implementation`, `plan`, `retrospective` — `CompanionKind::as_str` and
`PHASE_2_KINDS` [measured 2026-09-18, `crates/rk-types/src/companion.rs`]), plus one row-kind each for the item records
(`initiative`, `project`, `story`, `spec`), the free-form gate result (`report`) and the legacy configuration (`config`,
`process`); **the loss carriers land under `report` and `spec`** — stated so the block's report check and two
implementers count the same rows; (5) `--dry-run` stops here and prints the report — the JSON
document and nothing else — to stdout (the block parses the whole stream as one document), exit `0`, having written
**nothing in the working tree, refs or objects** (no worktree, no branch — the dry run runs steps 1, 2 and 4 only; the
two filesystem writes both binaries make before any git write are the bootstrap's temp hooks directory and
`<git-common-dir>/redkiln/hooks-empty/` — neither inside the working tree, both outside every porcelain listing); (6) the write phase into the
worktree, then `report.json` beside it; (7) `phase: written` on stdout (**D18-12**, the AC-450 kill seam — a print,
never a flag or env toggle); (8) `tree_hash(exec, wt, common_dir, &TreeBasis::WorkingState)` compared to the prior
migration commit's tree when one exists — equal is the **no-op** (AC-450: *"defined as tree-digest equality with the
prior migration commit's tree"*), report to stdout, exit `0`, no commit; (9) `commit_with_trailers(exec, wt,
"redkiln-migrate: <corpus_sha> <tool_version>", &[], false)` — **one commit**, on the migration branch only, carrying
the layout and the report (AC-447, AC-448); a second write phase in one run is an abort before writing (a phase
counter the test injects against).

- **AC-441 (build-graph half)** — *"a build-graph assertion that no `rk-*` crate depends on the `redkiln-migrate`
  crate."* `ac466_ac441_migrate_depends_on_no_harness_adapter_or_ports` reads `cargo metadata` in-process: no package
  whose name starts `rk-` has `redkiln-migrate` in its dependencies; `redkiln-migrate`'s normal/build closure names no
  `rk-adapter-*`, `rk-ports` or `rk-kernel`. The block's python walk is the same assertion outside the crate. The
  router half is §7d.3's.
- **AC-442** — BINDING (workspace lint). *"GIVEN the redkiln-rs workspace / WHEN the legacy-format provenance lint
  runs in CI / THEN every site that reads or writes an old redkiln file format (`.redkiln/processes/*.yaml`, old
  `.bklg` frontmatter, `template_sig`/`rendered_sig`) is located inside the `redkiln-migrate` crate AND a
  legacy-format handling site anywhere else fails the build."* *Verified by:* *"a CI lint over the workspace with an
  explicit allowlist of one crate plus one named site — `rk-store`'s old-layout detector (AC-440's predicate: manifest
  absent and `.bklg/` present), which reads a path, never a format; a fixture PR that adds a legacy reader to `rk-store`
  and must fail; a fixture that widens the detector to parse frontmatter and must fail."* The lint is a `ci.yml` step
  (Slice 5, §7d.15): `git ls-files 'crates/*/src/**/*.rs'` × `grep -nE 'template_sig|rendered_sig|processes/.*\.yaml|yaml_serde'`,
  comment-filtered, allowlisting `^crates/redkiln-migrate/src/legacy/` and the one `rk-store` detector path by name;
  the two must-fail instruments are `ac442_lint_flags_legacy_reader_outside_legacy_module`: the same scan function
  run over a temp tree holding `ac442_template_sig_control.rs` at a synthetic `crates/rk-store/src/` path and a second
  copy that parses `---` frontmatter, asserting each is flagged (the control file also lives under
  `crates/rk-cli/tests/fixtures/lint-controls/` for the block's `rk_control`).
  `ac442_legacy_readers_only_under_legacy_module`: the same scan in-process over `crates/*/src/**/*.rs` (the `ci.yml`
  step's scope — **never `tests/`**, where `lint-controls/ac442_template_sig_control.rs` carries `template_sig` by design
  and this crate's own `tests/it/` names both literals to assert their absence; a scan over all of `crates/` is red on a
  correct tree), asserting every hit is under
  `crates/redkiln-migrate/src/legacy/` — **self-controlled** first, as every source-scan test in this file is (§7d.14). **Instrument limit stands as written**: the lint is syntactic.
- **AC-443** — BINDING (`rk-git`). *"the manifest contains exactly one entry, `reconcile.rs` … AND a workspace file
  whose content hash matches any file in the old tree at its pin, and is not that entry, fails CI."* The manifest is
  `provenance.toml` at Step 6 and its test is `rk-git`'s; **7d's share is negative**: `redkiln-migrate` copies **no
  file** from the predecessor — every module is new — and `legacy-mini/PROVENANCE.md` records that no byte under the
  fixture is a predecessor byte (D18-10). **Labelled PROVISIONAL with AC-445** (§7d.18): AC-443's own instrument is a
  content-hash comparison against the old tree at `47faec9f`, which is local-only, and
  `legacy_mini_provenance_matches_bytes` pins the note's self-consistency, not the absence of predecessor bytes — it
  cannot fail on a copied file whose sha256 is listed in the note; the manifest test is `s6`'s.
- **AC-445** — *"no workspace test file matches an old test file by content hash, or by body hash after a declared
  normalisation"* — the same instrument over test paths. **ARMED-local**: the hash comparison needs the predecessor
  tree at `47faec9f`, which is `D:/repos/redkiln`'s working copy and the corpus bundle, both local-only; **the
  provenance note is the Phase-1 evidence** — `legacy-mini/PROVENANCE.md` states the fixture is hand-authored and lists
  each file's sha256, and the test that reads it (`legacy_mini_provenance_matches_bytes`) is red if a file's bytes
  drift from the note.
- **AC-446** — a regression corpus binding recorded predecessor bugs to named tests: **not this stage's** (its table
  is `rk-kernel`'s and the `--stay` half is built there); excluded by id.
- **AC-447** — BINDING. *"THEN exactly one commit is created on the target branch — the dedicated migration
  branch/worktree AC-522 requires AND a run that would produce more than one commit aborts before writing, and a run
  that would produce zero commits aborts before writing unless it is AC-450's no-op case."*
  `ac447_exactly_one_commit_on_migration_branch`: `git rev-list --count <target-HEAD>..rk-migrate/<short>` is `1`;
  `ac447_second_write_phase_aborts_before_writing`: a test hook that requests a second write phase; the run aborts
  with nothing written to the worktree and no commit.
- **AC-448** — BINDING. *"THEN a differential report is emitted in the same commit, carrying per-(kind, slot) counts
  of lifted / lifted-with-loss / failed and the corpus SHA AND a run that writes the new layout without emitting a
  report is refused."* *Verified by:* *"a schema-validated report artifact (`jsonschema`) plus an end-to-end test
  that deletes the report step and must fail."* `ac448_report_lands_in_the_same_commit_and_validates` (the commit's
  tree carries `report.json`; it validates against `schema/report.schema.json` through `jsonschema` in-process);
  `ac448_write_without_report_is_refused` (the report step disabled by hook → the run refuses before commit).
- **AC-449** — BINDING. *"THEN the two output trees are byte-identical and the two report digests are equal AND the
  identity holds across the swept nuisance parameters: `TZ`, directory-read order, and hash-map seed."*
  `ac449_two_targets_byte_identical_under_tz_and_read_order`: two empty targets built from `legacy-mini`; the second
  run under a different `TZ` in the **child's** environment (the test spawns the binary — `set_var` is banned) and a
  shuffled read order — **in-process**: the library's lift entry (`redkiln_migrate::lift::run(&target, &mut dyn
  DirOrder)`) driven twice, once with the natural listing and once reversed, tree digests and report digests equal (a
  seam is unreachable from a spawned process without the flag or env toggle AC-474 forbids, so the read-order arm
  cannot be a spawn); the `TZ` arm spawns the binary and **is disclosed as weak** — the migrator reads no clock (the
  block's `SystemTime::now|…` ban over its `src`), so `TZ` can move only git's commit metadata, which the tree digest
  excludes; the reader takes an ordering seam; `BTreeMap` everywhere makes the hash-seed arm moot and the
  test says so — `HashMap` is clippy-banned workspace-wide.
- **AC-450** — BINDING. *"THEN the run is a no-op, defined as tree-digest equality with the prior migration commit's
  tree — the re-run's report is written to stdout … AND the tool never emits a second normalisation commit, and
  AC-447's zero-commit abort does not fire on this case AND a run killed between write and commit leaves the target
  branch untouched (AC-522's worktree) and the next run re-reaches the same tree digest."*
  `ac450_second_run_is_noop_report_to_stdout_no_commit` (migrate twice; `git diff --exit-code`; one commit; the
  second run's stdout is a report that validates); **`idempotent_across_kill`** (the 153-line file's name, kept):
  spawn the real binary, read stdout until `phase: written`, kill it (`rk_containment::terminate_pid` through the
  job — F38's shape), assert the kill landed inside the window — `git rev-list --count <target-HEAD>..rk-migrate/<short>`
  is `0` and `migrate-wt/` holds written files (a commit that raced ahead of the kill would make the run an
  uninterrupted one measured under the kill test's name) — and that the target branch's `HEAD` and tree are unchanged;
  run again, assert the count is now `1` and the tree digest equals an uninterrupted run's. **A digest that differs across the boundary is Halt 4, not a retry.**
  `phase_written_marker_is_printed_before_commit`: the marker line is read on stdout while `git for-each-ref
  refs/heads/rk-migrate/*` is still empty (the test polls the ref after each stdout line; the marker precedes the first
  non-empty ref listing).
- **AC-463** — the Phase-2 exit check's mechanical definition of *"green"*. **Named, not run**:
  `ac463_corpus_differential_runner_named` is `#[cfg(feature = "corpus")] #[ignore]`, exists as a `fn`, and reads
  `CORPUS_PIN`; the block asserts it is authored (a `grep` for the `fn`), never that it passed.
- **AC-465** — the corpus-residency lint is `ci.yml`'s (Step 2); **7d's share is that `legacy-mini` is synthetic and
  tracks no corpus artifact** (D18-10) — the fixture is hand-authored, its `PROVENANCE.md` says so and carries sha256s,
  and `ci.yml`'s AC-465 rule needs **no allowlist row for the migrator** because no `.jsonl` and no predecessor bytes
  enter its tree (the four rows `rk-cli`'s synthetic `.jsonl` fixtures need are §7d.13's and §7d.15's). The block's
  `git ls-files` check over the fixture asserts no `*.jsonl` and no `spikes/` path under it.
- **AC-466** — BINDING. *"THEN zero harness dispatches are made and the binary links no harness adapter AND an attempt
  to construct a harness port inside the migration path fails at build time."* *Verified by:* *"a build-graph
  assertion that `redkiln-migrate` does not depend on `rk-adapter-claude`/`-codex`; a run-level assertion of zero
  `dispatch` events in the event log."* The build-graph half is `ac466_ac441_…`; the *"fails at build time"* half is
  the same fact stated as a type — with no `rk-ports` edge there is no `Harness` trait to implement or hold; the
  run-level half is vacuous here in the exact sense the register means (no event log is written by the migrator)
  and the test asserts the migration commit's tree carries no `store.sqlite3`.
- **AC-468 (schema half)** — BINDING (E13). *"THEN lifted, lifted-with-loss and failed are three distinct counts AND
  a report or consumer that presents only a combined total, or folds with-loss into lifted, fails validation."*
  `ac468_schema_rejects_total_only_and_folded_forms`: the committed schema requires the three integer fields per
  `per_kind_slot` row and `additionalProperties: false`, so a `total`-only row and a row carrying `total` beside the
  three both fail; a folded form (with-loss added into lifted) is refused by **the fixture-count check over
  `legacy-mini`** (no consumer exists in v0, and a JSON schema cannot detect folding — both fields are integers): summed
  over `slot`, the report's `report` rows read `lifted = 0, lifted_with_loss = 1, failed = 0` and its `spec` rows
  `lifted = 1, lifted_with_loss = 1, failed = 0` (§7d.13's one free-form `report.md`, one clean `spec.md` and one
  diff-prose `spec.md`), asserted exactly, so `lifted = 2` (folded) or `lifted_with_loss = 0` (dropped) is red; the
  block's dry-run python carries the same two exact counts beside `loss >= 1` (§7d.17). The
  corpus-scale assertion is Phase 2's.
- **AC-470** — BINDING (`rk-store` canonical serialiser). *"THEN the bytes are identical for 100% of migrated artifacts
  AND no byte-parity with the old YAML serialiser is attempted or asserted anywhere."*
  `ac470_migrated_artifacts_round_trip_through_canonical`: every artifact in the migration commit's tree read back and
  re-serialised by `rk_store::canon` is byte-equal; the absence half is AC-442's lint (no YAML emitter outside
  `legacy/`, and `legacy/` only **reads**).
- **AC-471** (`rk-types`) — *"every artifact schema declares a `schema_version`, and no schema declares a template- or
  rendered-signature field"* — the enumeration test is `rk-types`' (Step 5); **7d's share**: the literals
  `template_sig` / `rendered_sig` are read under `legacy/` and **never written** — AC-442's lint over the migrator's
  own output path (the `lift.rs` scan in `ac442_…`).
- **AC-472** — its `artifact.read` half is the kernel's; its `doctor` half is §7d.8's `RK-D-UNREAD`. Listed here because
  the register files it under §13.
- **AC-473** — the self-hosting release check: **Step 8's**; excluded by id.
- **AC-474** — BINDING (clean break, negative). *"THEN no emitter of an old redkiln file format exists outside
  `redkiln-migrate` AND no configuration key, flag or environment variable enables a dual-write or compatibility-write
  mode."* *Verified by:* *"the AC-442 provenance lint … plus a config-surface snapshot test over the full key list,
  asserting no dual-write key exists."* `ac474_no_dual_write_key_flag_or_env` (the migrator's flag set is the
  snapshot — `--target`, `--dry-run`, `--version`, nothing else; no `std::env::var` in `crates/redkiln-migrate/src`
  — the block's ban) and `rk-cli`'s `ac474_config_surface_snapshot_has_no_dual_write_key` (§7d.9).
- **AC-475** — **PROVISIONAL (S-BIN unrun) and claimed here as a negative tripwire only** (E13). *"THEN it is carried
  as opaque content with a migration-loss marker AND no `ChangeSummary` value is synthesised."*
  `ac475_zero_change_summary_values_in_output`: no artifact in the migration commit's tree carries a `ChangeSummary`
  value and `crates/redkiln-migrate/src` names the identifier nowhere (the block's `rk_absent`); the loss-marker
  carry over `legacy-mini`'s one diff-prose item is built as `diff_prose_carried_opaque_with_loss_marker` and claimed
  under **no id** (E13: *"The **loss-marker migration fixture** half stays Phase 2's"*).
  **Vacuity disclosed as the register discloses it**: the second conjunct is trivially true while the type does not
  exist; it is kept as the tripwire against a type appearing in the migration path before S-BIN.
- **AC-476** — **HUMAN GATE**, *"routed to a human gate; **not machine-decidable**"* whole (its Verified-by; the
  register names no machine half — contrast AC-435, whose Verified-by does): *"a human answers a gate whose question is
  whether the recorded losses … are acceptable AND the commit does not land on a machine verdict alone."* **Not
  claimed.** DECISIONS-2026-09-18 O-14 asked whether its AND clause is a machine half; **answered 2026-09-19: not
  split** — the clause is AC-522's to instrument, and it is: the commit lands on `rk-migrate/<short>` and never on the
  target branch (`ac447_exactly_one_commit_on_migration_branch` asserts the target's original branch unchanged; the block
  asserts `HEAD` unchanged on the second temp repository, §7d.17), a dated note on AC-476's Verified-by records the
  ownership, and the report the gate reads is `report.json` (AC-448). The gate itself is the first landing stage's;
  the id is placed in Excluded (§7d.18).
- **AC-477** — BINDING (`rk-store` render path) — **ARMED whole, not claimed** (§7d.18, beside AC-478). *"GIVEN a typed
  gate result / WHEN the artifact carrying it is rendered to markdown and the typed value is read back from the store /
  THEN the value read back equals the value written, and the markdown is derived from it on every read AND no
  production code path reconstructs the typed value from the rendered markdown."* *Verified by:* *"a render round-trip
  test (typed → markdown, then typed re-read from the store, not from the markdown)"*. **Its WHEN needs a render**, and
  `rk-store`'s render path does not exist at this pin [measured 2026-09-18: no `fn render` under `crates/rk-store/src`;
  `locator` is the one module that reads markdown] — so a *value read back* half with no render is AC-470's canonical
  round trip under a second id, not a half of this criterion, and none is claimed. What is built:
  `typed_gate_result_round_trips_through_canonical` (**no id**) — a typed gate result written by the lift, serialised
  through `rk_store::canon`, re-read from the store's typed value, equal — the shape the render round-trip will extend
  when the path exists; and the sibling `free_form_gate_result_carried_opaque_with_loss_marker` (claimed under no id, as
  `diff_prose_…` is) lifts `legacy-mini`'s `report.md` free-form gate result as opaque prose with a loss marker (never
  parsed into a typed gate result — S-MIG decision 2's *"emitted by the toolchain and rendered, never parsed"*). The
  render path is the owner record AC-411 and AC-478 also wait on (§7d.18 RAISED).
- **AC-478** — BINDING (`rk-store` render path) — **ARMED, not claimed**. *"THEN the typed values are returned unchanged
  and the markdown is regenerated from them, overwriting the edit AND no comparison of stored-vs-regenerated rendering
  is performed."* Its Verified-by — *"a mutation test that edits the rendered markdown of a stored artifact and asserts
  the next read returns the original typed values and rewrites the file"* — needs the render path the tree does not
  carry, and this stage edits no `rk-store` `src/` (§7d.15); the earlier draft's `ac478_schema_version_stamped_on_every_artifact`
  asserted AC-471's clause under AC-478's name and is renamed `migrated_artifacts_carry_schema_version_and_no_sig`
  (every migrated artifact carries `schema_version` and no `*_sig` field — **AC-471's write half**, §7d.18's ledger). The
  render path's owner record is the one AC-411 also waits on (§7d.18 RAISED).
- **AC-522** — BINDING (run contract). Quoted in the order above. **`refuses_dirty_target`** (the 153-line file's
  name, kept): one untracked file, then one modified file; each refused naming the path; `git status --porcelain` of
  the target equal before and after; no `rk-migrate/*` ref and no `migrate-wt` directory exist after.
  `ac522_worktree_recreated_never_reused`: a stale `migrate-wt/` left from a killed run is removed and re-added
  (`worktree::add`'s refusal of an existing path is the clause), and the run's commit lands on the migration branch
  only — the target's original branch and working tree byte-identical before and after.
- **The six typed kinds and the eight opaque** — `adr0075_six_typed_kinds_eight_opaque`: `rk_types::companion::CompanionKind`
  has **six** variants (`Integration`, `Grounding`, `Decomposition`, `Review`, `LedgerCompanion`, `Ledger`) and
  `PHASE_2_KINDS` has eight [measured 2026-09-18, `crates/rk-types/src/companion.rs`]; DECISIONS-2026-09-11 #3's wording
  is *"five plus `ledger`"* — the five sidecars and one bare census kind — which the type spells as six variants
  because `_ledger` (the sidecar) and `ledger` (the census's bare kind) are distinct wire names; **noted, not
  relitigated**. The test lifts one fixture of each of the fourteen kinds — `legacy-mini` carries one file per kind
  (§7d.13) — and asserts typed for six, opaque for eight.

### 7d.12 The `s7c` split notes

`s7c` §7c.22 owes this file *"the split notes at the other end"* for seven ids and names 7d in fourteen ARMED rows.
Each has a disposition here; none is a second placement.

| id | what 7c did | disposition at 7d |
| --- | --- | --- |
| **AC-417** | the card-validator half (`structured_output_required`, `ac493_…`) | **DISCHARGED here** — the `doctor` half: `RK-D-CARD-TOOLS` / `RK-D-CARD-TOOLS-ABSENT` and `ac417_card_without_structured_output_or_tools_key_rejected` (§7d.8) |
| **AC-422** | the adapter half (`ac422_denylist_is_one_exported_list`; AC-216's `--bare` matrix) | **DISCHARGED here** — the `doctor` half: `RK-D-ENV-DENYLIST` reads `env_denylist::first_denylisted`; `ac422_config_reads_the_one_denylist` asserts `rk-cli` carries no second `ANTHROPIC_API_KEY` literal (the block's ban). The compound id stays unsplit in the register (RAISED, as 7c left it) |
| **AC-423** | the TTL rule (AC-526) one row would read | **DISCHARGED here** — `RK-D-CLI-VERSION` / `RK-D-TTL` under the stubbed probe; the shipped probe named (D-11) |
| **AC-428** | the adapter half (`ac218_…`) under owner AC-218 | **EXCLUDED here** — twin discharged at 7c; the stream-corpus replay conjunct is unsatisfiable at the pin and its register amendment is 7c's RAISED item (§7d.4) |
| **AC-439** | the adapter half (`ac206_…`) under owner AC-206 | **EXCLUDED here** — as AC-428 |
| **AC-488** | the construction refusal (`api_key_refuses_construction`) | **DISCHARGED here** — the edge read (the one environment read at the root, handed to `Config::new` as a value) and the `doctor` half (`RK-D-ENV-DENYLIST`; `ac422_ac488_denylisted_env_name_is_a_doctor_error`) |
| **AC-344** | `ac344_missing_binary_is_not_a_candidate_failure` — `Transport { kind: Spawn }`, never a failing test | **EXCLUDED here** — the verify runner that maps it to `Unverifiable { cannot_run }` is not built at 7d (the `s7b-ii` verify block is RAISED for re-routing, §7d.18); the note 7c owed is this row |
| **AC-203, AC-205, AC-212, AC-220, AC-226, AC-377, AC-383, AC-493, AC-526** *(doctor halves)* | built the adapter halves | **DISCHARGED here** as the rows `RK-D-AGENTS-WITHOUT-AGENT`, `RK-D-CARD-TOOLS`, `RK-D-TIER-LEAK`, `RK-D-HEADROOM-MARGIN`, `RK-D-CLI-VERSION`, `RK-D-MARGIN`, `RK-D-CACHE-PROFILE`, `RK-D-CARD-TOOLS-ABSENT`, `RK-D-TTL` (§7d.8) |
| **AC-369, AC-370** *(doctor halves)* | the sqlite store halves | **DISCHARGED here** as `RK-D-STORE-ID` (AC-369: identity; one store per common dir) and `RK-D-STRAY-STORE` (AC-370: *"a store file discovered under a linked worktree's own `.git` directory is reported by `doctor` as a configuration error"* — its own row and fixture, `ac370_stray_store_under_linked_worktree_is_an_error`, not AC-413's fixture); the moved-repository policy and the multi-root rule stay RAISED under 7c's R-23 |
| **AC-366, AC-397, AC-395** | ARMED: `doctor --rebuild` / the rebuilt-store rule / `redkiln cost` | **AC-397 DISCHARGED here** (`RK-D-REBUILT-TIER2`); **AC-366 ARMED on to a CI run** (*"a CI job that clones, rebuilds and diffs"*; `ci.yml` has never run); **AC-395 ARMED on to Step 8** — `redkiln cost <item>` is **not a 7d verb**: it needs the cost unit and arithmetic form RT-19's OWED record names (`CostUsd(String)` has no sum), and design §8.1 lists no `cost` verb at Phase 1; building it would decide the record |
| **AC-372** *(renderers + doctor rule)*, **AC-398** *(drift report)*, **AC-365** *(next-action equality)* | ARMED to 7d / Step 8 | **ARMED on to Step 8** — the renderers are `redkiln cost`'s; the drift report renderer needs `rk-store`'s reconcile output surfaced by a verb no record names at Phase 1; AC-365's next-action equality is `s8`'s `stage_level_next_action` (the AC-427 fidelity half, D-21) |
| **AC-399** *(the `redkiln cost` exclusion)* | ARMED whole; carrier undecided | **ARMED on** — no carrier record has landed |
| **AC-278** *(the gate-path GIVEN)* | ARMED: *"the enumeration of the run's job after exit"*; primitive built | **ARMED on to Step 8, by half**: at 7d the gate path dispatches through a fake harness and spawns **no child by construction**, so an enumeration after `ac403_…`'s exit is empty for a reason unrelated to containment; the GIVEN's real form — a gate reached after a real dispatch — is `s8`'s fix-lane run. `ac403_…` asserts the empty descendant set anyway, as a floor, and does not claim the id |
| **AC-269** *(build-configuration half)* | ARMED: a `cargo metadata` assertion that `rk-adapter-claude` has no edge to `rk-cli`, vacuous before `rk-cli` existed | **DISCHARGED here, by construction — not by an instrument that can fail**: `rk-cli` depends on all four `rk-adapter-*` crates, so the reverse edge is a dependency cycle cargo refuses at build; the block's `back == []` walk is a floor that records the fact and cannot be red on any tree that builds (the PIA-059 shape, disclosed — as AC-449's hash-seed arm is discharged on the clippy ban, not on a test). The half is placed DISCHARGED on that construction and this sentence says so |
| **AC-350** *(composite `Toolchain`)* | ARMED: `Ports` holds one slot; R-8 | **ARMED on** — no composite is built (§7d.1's routing row; DECISIONS RT-40 withdrawn by a dated line the same day); the two-toolchain case waits on R-8 |
| **AC-323** *(the verify result's basis on the CLI status line)* | `s7b-ii` §7b-ii.14: *"AC-323 is the exception and is wholly ARMED: its register subject is `rk-cli` / `rk-store-artifacts` and no `rk-cli` exists at this stage, so it has no half that runs here"* | **ARMED on** — its THEN (a verify result's basis and reason rendered beside the result) needs the verify activity runner this file does not build (the `s7b-ii` verify block, RAISED); not Excluded, because no half of it ran anywhere |
| **AC-361** *(item cost rollup)*, **AC-367**, **AC-387** | ARMED with reason: an owner record OWED | **ARMED on** — kernel-side; the rollup needs RT-19's cost unit; 7d builds no `cost` verb |
| **AC-215** *(integration half)* | ARMED: `live_disallowed_tool_terminates` authored, `--features live`, `#[ignore]`; *"run at the operator's word or at 7d"* | **ARMED on to Step 8** — this checkpoint has no live line by design (the gate check), and a paid dispatch the operator did not decide is not added under cover of another stage; the card exists and its run is one `cargo test --features live` line the operator can issue |
| **AC-223** | ARMED: a pack lint reading the recorded slow-condition maximum from the event store | **ARMED on to 7e** — a pack lint over an event store with recorded classes; `.redkiln/` is 7e's |
| **AC-320, AC-339, AC-341, AC-348, AC-349, AC-353, AC-355, AC-359, AC-338** *(the verify block `s7b-ii` armed to 7c, naming the composition root)* | RAISED: *"a re-routing record is OWED"* | **RAISED here too, not built** — the verify activity runner is neither an adapter's nor this stage's set (no §12 or §13 criterion names it); a record routing the block to a stage is OWED (§7d.18) |
| **AC-321** *(verify-run kind)* | ARMED on the record | **ARMED on** — no kind minted |
| **AC-498** | CI, never run | **ARMED on** |

**PIA-047's `redkiln answer` round-trip, DEFERRED here by `s7b-ii` §7b-ii.14** (*"the round-trip cannot run here … DEFERRED
to Checkpoint 7d and recorded there"*): it runs as the spawned-binary tests `ac406_…`, `ac407_…` and `ac435_…` (open a
gate with `run`, answer with the real binary, read the store), not as a block line — the init-only temp repository has
no pack and can open no gate; the erratum against PIA-047 `s7b-ii` said is OWED stays OWED (§7d.18 RAISED).

**The `s7b-ii` rows naming 7d** (`s7b-ii` §7b-ii.14's ARMED table and its 2026-09-17 parentheticals) are placed the same
way, in §7d.18: **DISCHARGED here** — AC-240 (through `ac403_…`), AC-248 (`ac248_…`), AC-257 (`ac257_expired_gate_resolved_by_fresh_process`
— a gate whose deadline passed with no process alive is observed and resolved at the next `run`'s read, in a fresh
process; the kernel's `expire()` is 7b-ii's), AC-263 (`ac263_…`), AC-264 (`ac400_resume_…`); **EXCLUDED** — AC-241,
AC-251, AC-256, AC-260, AC-268, AC-277, AC-280, AC-300, AC-327, AC-330, AC-337, AC-349 (their *"doctor/CLI
halves"* name instruments — a CLI integration test of a kernel behaviour — that add no assertion the kernel test did
not make and no `doctor` row a §12 criterion names; this file does not re-assert them under new names); **ARMED** — AC-323 (wholly, with the
verify runner — the table above); **RAISED** —
AC-254's render (needs a diff render over
AC-245's algorithm no record shapes as a verb) — RAISED.

### 7d.13 Where the fixtures live

**No corpus byte enters the tree** (AC-465; D18-10). Every fixture under the two new crates is authored, and the one
that imitates the predecessor's layout says so in a provenance note beside it.

- **`crates/redkiln-migrate/tests/fixtures/legacy-mini/`** — a synthetic old-layout tree: `.bklg/init-1/initiative.md`
  (YAML frontmatter `id/uid/type/slug/title/status/process/stage`), `.bklg/init-1/proj-1/project.md`,
  `.bklg/init-1/proj-1/story-1/{story.md,spec.md,_ledger.md,_review.md,_integration.md,_grounding.md,_decomposition.md,ledger.md}`
  (`_ledger.md` a fenced-yaml ledger sidecar; `ledger.md` the census's bare `ledger` kind; `_review.md` with
  `verdict`/`rubric`; `_integration.md`, `_grounding.md`, `_decomposition.md` the other three typed sidecars — **the six
  typed `CompanionKind`s, one file each**), `.bklg/init-1/proj-1/story-2/{story.md,spec.md,report.md}` — `report.md`
  carrying a **free-form gate result** (AC-477) and `spec.md` a **diff-prose section** (AC-475's carry), one file per
  Phase-2 kind (`closure.md`, `design.md`, …, eight — the opaque set), `.redkiln/config.yaml` and `.redkiln/processes/
  bug-standard.yaml`. **`PROVENANCE.md`** beside it: *hand-authored 2026-09-18 for Step 7d; fourteen companion kinds, one
  file each; no byte copied from `D:/repos/redkiln` at `47faec9f` or from `redkiln-corpus`; shapes follow the
  predecessor's frontmatter key set as read, not its files* — and a sha256 per file; `legacy_mini_provenance_matches_bytes` reads the note and re-digests.
  The report rows it yields are §7d.11 step (4)'s vocabulary — the fourteen companion wire names plus `initiative`,
  `project`, `story`, `spec`, `report`, `config`, `process` — with the two loss carriers under `report` and `spec`. A
  **`.gitattributes` at the repository root** (landed in **Slice 2** with `init/config.toml.expected` and the exit-table
  snapshot, extended in Slice 5 with the `legacy-mini/**` row — a file whose bytes a test pins is protected from the
  slice that lands it, not three slices later: any checkout event between the slices — a branch switch, a stash, the
  operator's review clone — would rewrite an LF fixture to CRLF and turn the block's `eol` guards and
  `ac437_init_scaffold_bytes_pinned` red for a reason unrelated to the binary, the failure the row exists to prevent)
  marks `crates/rk-cli/tests/fixtures/init/config.toml.expected -text`, `crates/rk-cli/tests/it/snapshots/*.snap -text`
  (the block's ten `^<code> <Variant>$` greps over the exit-table snapshot are end-of-line anchored and miss on
  `accepted\r`) and `crates/redkiln-migrate/tests/fixtures/legacy-mini/** -text`, so a checkout under `core.autocrlf=true` —
  this box's setting, with no `.gitattributes` anywhere in the tree
  today [measured 2026-09-18: `git config --get core.autocrlf` → `true`; `git ls-files --eol Cargo.toml` → `w/mixed`;
  7c's `.snap` files read `w/lf` only because they were never re-checked out] —
  cannot rewrite bytes a test pins (the sha256s here; the `cmp -s` and the `$`-anchored greps in the block). Tests and the checkpoint **copy it into
  a temp git repository** (`git init` + one commit) before any run; nothing runs against the tracked copy. It carries no
  `*.jsonl` and no `spikes/` path.
- **`crates/redkiln-migrate/schema/report.schema.json`** — `{ corpus_sha (40-hex string), tool_version (string),
  per_kind_slot: [ { kind, slot, lifted: integer, lifted_with_loss: integer, failed: integer } ] }`,
  `additionalProperties: false` at both levels, `required` on every field, **no `total`** (AC-468). Draft 2020-12; the
  crate validates its own report against it before writing.
- **`crates/rk-cli/tests/fixtures/doctor/<rule-id>/`** — one rejecting directory per id in §7d.8's table (thirty-eight),
  each a minimal repository skeleton (`.redkiln/config.toml`, a `packs/` tree or an `events.jsonl` as the rule needs,
  a `README` naming the rule and the clause) — parts of a skeleton that live under `.git/` or in a store are not tracked
  as such (git refuses to track a nested `.git/`, and a SQLite file is not a reviewable fixture): the hook file sits at
  `dot-git/hooks/pre-commit`, the orphan entry at `dot-git/worktrees/<name>/gitdir`, the stray store's linked-worktree
  gitdir at `dot-git/worktrees/<name>/redkiln/store.sqlite3.toml` (a description), and the two store fixtures are
  `identity.toml` / `rebuilt` descriptions; each per-row test (and `ac415_…`) **materialises** the directory into a
  `tempfile` repository (`git init`, `dot-git/` → `.git/`, the store built through `rk_store::conn::Writer::open` and
  `identity::initialise` from the description) before running the rule, so a fixture that materialises to nothing is
  red on the rule's own finding count — plus **`doctor/clean/`** — a configuration every rule passes, the positive
  control every per-row test runs second; it carries a `dot-git/hooks/pre-commit` that is an **operator hook** with no
  `# redkiln-hook` marker and a `# fork-join` comment, so `RK-D-HOOKS`'s recogniser is red on the clean run if it fires
  on any non-`*.sample` file (RT-51's rejected alternative — §7d.8). The `roles.rs` reader's field set (§7d.8's AC-416 row) is the floor these
  fixtures spell: a card file's `fulfils: [<role>]` and a skill file's `attach.roles: [<role>]`, both under the pack's
  `cards/` and `skills/` directories — 7e's pack schema may re-point it. ***Amended 2026-09-21, Step 7d Slice 4 cycle 4, on the operator's decision of that date: the SKILL half stands as written; the CARD half is re-pointed to a sidecar, and the directory shape to the layout the binary reads.*** **(1) `fulfils` cannot live on a card file.** `rk_adapter_claude::card::CardFile` is `#[serde(deny_unknown_fields)]` over exactly seven names — `description`, `prompt`, `tools`, `cache_profile`, `permission_mode`, `model`, `effort` — and `fulfils` is refused as an unknown field [measured 2026-09-21]; §7d.1 permits **one closed crate edited** (`rk-kernel`), so widening it is foreclosed, and a card carrying the key would be a shape no production pack could use because `run`'s binder loads every card through that type. **Slice 3 met this and raised it to this slice by name** (`crates/rk-cli/tests/fixtures/pack-min/cards/README.md`: *“Slice 4 must either widen `CardFile` or carry the role→card map somewhere other than the card”*) — this is the second. The carrier is **`.redkiln/roles.toml`**, `[[card]] file / fulfils / selector`, read by `doctor/roles.rs` and by nothing else. **`selector` is load-bearing**: AC-416's second half is *“two cards at **equal selector specificity**”*, and a bare list of card names has no specificity, so `RK-D-ROLE-AMBIGUOUS` would be a row that could never fire and its AC-415 rejecting fixture could not exist. **(2) There is no per-pack `cards/` or `skills/` directory.** A pack is a FLAT FILE, `.redkiln/packs/<name>.toml`, and cards are at `.redkiln/cards/<role>.json` keyed by file name [measured 2026-09-21, `crates/rk-cli/src/verbs/run.rs`:553-555 and :1427-1429]; skills are `.redkiln/skills/<name>.toml`. The fixtures spell the layout the binary reads. **(3) The skill half collides with nothing** — measured the same day, NOTHING outside `crates/rk-cli/src/doctor/roles.rs` reads skills at all, so `attach.roles` is spelled exactly as this sentence writes it. 7e's pack schema may still re-point all of it, which is what the sentence already says.
- **`crates/rk-cli/tests/fixtures/unread/{runs-49,runs-51}.jsonl`** — synthetic event logs: 49 and 51 runs of one
  stage producing kind `k` with `artifact.write` and never an `artifact.read` of it (D-08's kind-per-stage form) — each
  row's payload carrying ADR-0116 decision 1's members (`kind`, `item`, `schema_version`, `digest`) plus `stage` — the
  per-stage discriminator D-08's kind-per-stage predicate needs and no record's payload carries (**RT-48**: routed here
  as the row's read set; a dated design §5 note widening the two rows' payloads is OWED, §7d.18 RAISED; no kernel
  producer of either kind exists yet [measured 2026-09-18: `grep -rn ArtifactWrite crates/rk-kernel/src` → match arms
  only], and 7b's emitter, when it lands, writes these keys or re-points this rule),
  beside a second kind read once per run so a rule that flags everything is red; labelled `// SYNTHETIC` in a sibling
  note (JSONL carries no comments). `ci.yml`'s AC-465 rule fails on any tracked `*.jsonl` except the named ones [measured 2026-09-18,
  `.github/workflows/ci.yml`: `git ls-files … '*.jsonl'` against an allowlist], so this stage adds **four rows** by name,
  each commented *synthetic 7d fixtures, no corpus provenance*: `crates/rk-cli/tests/fixtures/explain/*/events.jsonl`
  (the run-record directories' event logs, §7d.7) and
  `crates/rk-cli/tests/fixtures/exit/*/*.jsonl` (Slice 3), `crates/rk-cli/tests/fixtures/unread/*.jsonl` and
  `crates/rk-cli/tests/fixtures/doctor/*/events.jsonl` (Slice 4 — `RK-D-UNREAD`, `RK-D-HEADROOM-MARGIN`,
  `RK-D-CREDENTIAL-RETAINED`'s and the five D18-19 gate rows' skeletons carry one); that is an allowlist edit, stated
  here so it is not read as a silent widening, and these four are the only AC-465 edits this stage makes.
- **`crates/rk-cli/tests/fixtures/config/{missing-doctor,missing-key,unknown-key,dual-write,unknown-adapter,missing-store}.toml`** — §7d.9
  (`unknown-adapter.toml` is the scaffold with one line changed; `missing-store.toml` the scaffold with the `store =`
  line removed — AC-486's second disjunct; `dual-write.toml` IS the AC-474 red fixture, which is
  why the block's dual-write sweep excludes `tests/`).
- **`crates/rk-cli/tests/fixtures/init/config.toml.expected`** — the scaffold bytes (§7d.9), marked `-text` in the
  root `.gitattributes` (above, Slice 2) so `core.autocrlf` cannot rewrite the LF bytes `init` writes and the block's `cmp -s`
  compares; the block asserts the checked-out `eol` is `w/lf` before comparing — and asserts the same of the committed
  exit-table snapshot before its `$`-anchored row greps.
- **`crates/rk-cli/tests/fixtures/pack-min/`** — the smallest loadable pack: one role, one card with `tools:
  [StructuredOutput]`, `cache_profile = "static"`, `timeout_secs` on every activity, `margin_ticks`, both TTLs, an
  attempt cap of `1` (so a one-element `FakeHarness` script is never exhausted into `Transport`, §7d.5), a
  `gate_deadline_secs` and a gate-class expiry row (`PackExtras`' keys, §7d.5 step (7)), **one `require` row reading a
  facet the gate items leave null** (the abstention gate — `GateReason::UnknownFacet`; a `StepKind::Gate` step opens
  nothing at the pin, §7d.5), one `tool` step; a two-activity variant for `ac485_…` and `ac401_…`; a variant declaring a
  **1-second** `gate_deadline_secs` for `ac257_…` and `ac248_…`'s expired arm (the deadline passes through a test-side
  wait past it — a test may sleep; the product never does — no clock toggle, AC-474 — written as
  `mpsc::Receiver::recv_timeout` on a never-sent channel, or as `std::thread::sleep` under a per-site
  `#[expect(clippy::disallowed_methods, reason = "a test-side wait past a 1 s gate deadline, not a decision (AC-003)")]`,
  because `std::thread::sleep` is on `clippy.toml`'s workspace-wide `disallowed-methods` and the block's clippy gate runs
  `--all-targets`); items `bug-item.toml` (`kind =
  "bug"`, `stage = "implement"`), `story-item.toml`, `kindless-item.toml` (ADR-0128's refusal), a `stageless-item.toml`
  (RT-50's *"no stage on the item record"* refusal), and `gate-item.toml` with the `require`d facet
  unset beside them — every item but the stageless one carries the top-level `stage` key (RT-50).
- **`crates/rk-cli/tests/fixtures/explain/{two-activities,three-exclusions,unknown-four-axes,no-resolution}/`** —
  **run-record directories** in `replay::write_run`'s layout (`run.json`, `inputs/{pack,catalog,artifacts}/`,
  `inputs/facets.toml`, `inputs/context.json`, `inputs/artifact-state.json`, `inputs/runtime.json`,
  `inputs/specs.json`, `events.jsonl`), authored to the kernel's schema — `explain` reads them through
  `replay::read_run` (§7d.7); `no-resolution/` holds a store-shaped `events.jsonl` with no `dag_assembled` row and no
  run record; their `events.jsonl` files are in the AC-465 allowlist as above.
- **`crates/rk-cli/tests/fixtures/exit/<name>/`** — the table-driven fixtures (§7d.4), one directory per expected
  code, with a `expected` file carrying the integer.
- **`crates/rk-cli/tests/fixtures/facets/{clean,llm-sourced,undeclared-surface}.toml`** — `lint`'s inputs.
- **`crates/rk-cli/tests/fixtures/lint-controls/`** — **one** directory for both crates' bans (the helper takes one
  `LC`; a control is a file carrying a token, and which crate's ban it controls is in its name) — the positive controls
  for every absence ban in Checkpoint 7d, one file per ban, named for it, **checked through the ban's own pipeline**
  (`rk_control`, §7d.17): `ac529_current_dir_control.rs` (`std::env::current_dir()`),
  `ac403_stdin_control.rs` (`io::stdin().read_line(`), `ac422_api_key_literal_control.rs` (`"ANTHROPIC_API_KEY"`),
  `ac474_dual_write_control.rs` (`dual_write`), `adr0100_yaml_control.rs` (`yaml_serde::`), `adr0118_poison_recovery_control.rs`
  (`unwrap_or_else(PoisonError::into_inner)`), `adr0118_lock_control.rs` (`.lock()`), `adr0083_env_var_control.rs`
  (`std::env::var(`), `adr0083_clock_control.rs` (`SystemTime::now()`), `ac442_template_sig_control.rs` (`template_sig`),
  `ac475_change_summary_control.rs` (`ChangeSummary`), `ac405_gate_answer_control.rs` (**both** spellings the count
  searches, on non-comment lines: a call `gate::answer(` and an import `use rk_kernel::workflow::gate::answer;` — the
  control is checked with the count's SAME pattern, import alternation included, so a broken import branch is caught),
  `d10_adapter_constructor_control.rs` (`rk_adapter_claude::config::Config::new(` — the real path; the crate root
  re-exports no `Config`), `d10_adapter_import_control.rs` (`use rk_adapter_claude::config::Config;`), `ac436_keywords_control.rs` (`keywords`), `adr0102_allow_attribute_control.rs`
  (`#[allow(` — checked once through the ban's own `#!?\[allow\(` pattern), `adr0083_set_var_control.rs` (`set_var(`),
  `adr0077_git_command_control.rs` (`Command::new("git")`) — **eighteen files for nineteen `rk_absent` lines and one count
  line**: the two `std::env::var` bans share `adr0083_env_var_control.rs`, the two clock bans share
  `adr0083_clock_control.rs`, `ac405_gate_answer_control.rs` controls the `gate::answer(` count, and the ADR-0101 phrase
  sweep has no control by design (§7d.17). Never under `tests/ui/`. `ci.yml`'s anchored `lint-controls/` exclusion already covers
  `crates/[^/]+/tests/fixtures/lint-controls/` for every crate, so the new directory needs no CI edit.
- **Temp repositories** (D18-9): every integration test and every checkpoint line that runs a verb or the migrator
  mints its own with `tempfile::tempdir()` (tests) or `mktemp -d` (the block), `git init`, one commit
  (`SqliteStore::open` needs a root commit for its identity row), and — for the migrator — the `legacy-mini` copy.
  **The product repository's `.redkiln/` is never created by this stage**, and the block asserts it by comparing its
  presence at the top and the bottom of the block (`had_redkiln` / `now_redkiln`) — compared, not asserted absent,
  because O-15 gives the product repository its own `init` at 7e and every re-run of this block after that would
  false-fail on `test ! -d .redkiln`. **How a spawned `redkiln` reaches the fixture child**:
  `rk-fixture-child-claude` is a `[[bin]]` of `rk-adapter-claude`, and cargo sets `CARGO_BIN_EXE_rk-fixture-child-claude`
  *"only for the owning package's own tests"* (its manifest comment), so `rk-cli`'s tests locate it as the sibling of
  `env!("CARGO_BIN_EXE_redkiln")` in the same target directory
  (`Path::new(env!("CARGO_BIN_EXE_redkiln")).with_file_name("rk-fixture-child-claude")`, `.exe` on Windows), which
  `cargo build --workspace` — the block's first gate (GC3) — has built; ***AMENDED 2026-09-21 at Step 7d's close:
  a third `[[bin]]` copy IS added, and the one-`[[bin]]` manifest line does NOT stand.*** *This read "no third
  `[[bin]]` copy is added, so the one-`[[bin]]` manifest line stands" — true as a plan and false on the tree since
  `a65a903` (Slice 4 cycles 6–8), which added `rk-fixture-child-cli` to `rk-cli` and amended nothing. The warrant is
  measured and stands: one bin name in two packages is one output path, cargo warns of an output filename collision,
  the two link jobs race and the MSVC linker exits 1104 for both [reproduced 2026-09-17], and
  `ac423_probe_is_one_version_spawn_not_a_dispatch` reaches the child through
  `env!("CARGO_BIN_EXE_rk-fixture-child-cli")`, which cargo sets only for the owning package's own tests. **Checkpoint
  7d failed on this at its first whole run** and its manifest loop now expects TWO for `rk-cli` and ONE for
  `redkiln-migrate`, with a second line pinning the extra target's NAME so the permission is to that target and not to
  a third.* ***AMENDED 2026-10-01 (Weigh-In wi-b846d9):*** *the TWO did not stand either: `5e67ac0` (2026-09-23) added a third `[[bin]]`, `rk-fixture-toolchain`, the review fixture's stub `cargo` (ADR-0162), which `rk-cli`'s tests reach through `env!("CARGO_BIN_EXE_rk-fixture-toolchain")` and so must live in `rk-cli`. **Checkpoint 7d failed on this at its first whole replay inside Checkpoint 8g** (2026-09-30); the manifest loop now expects THREE for `rk-cli`, and a third line pins that target's NAME.* The sibling-location mechanism above is unchanged and is still how `rk-fixture-child-claude` is reached; the fixture repository's `config.toml` names it in the optional `[adapters]
  harness_binary` key (§7d.1's `ConfigFile` row — a key on the AC-474 snapshot, not a toggle), and its mode travels in
  the CHILD environment the test spawns `redkiln` with (`RK_FIXTURE_MODE`, `RK_FIXTURE_MARKER_DIR`, via `Command::env`,
  never `set_var`), which the adapter's `child_env` passes through to the child.
### 7d.14 Red first

**Red first (golden constraint 12; ADR-0019).** `00-preamble.md`'s table names **Step 7d** among the stages that write
Rust. The checkpoint below is the floor, not the evidence: every `cargo test … -- <name>` line is satisfied by an
implementation that nothing pins. Author each named test **before** the code it describes, run it, and **watch it
fail for the right reason** — the absent behaviour, not a compile error, a typo'd filter or a missing fixture. One
bullet per checkpoint test name; the name binds the test to its id; halves are red separately; other-crate tests are
labelled with their `-p`. The two names the 153-line file introduced for the migrator (`refuses_dirty_target`,
`idempotent_across_kill`) are kept; its `resume_is_refused` becomes `ac400_resume_refused_as_unknown_argument` and its
`doctor_window_n_fixtures` becomes `ac426_ac472_unread_window_49_not_flagged_51_flagged`, each renamed because the
old name carried a claim the new test pins more narrowly. **"Dispatch count 0" and "exactly one spawn" are file
counts** — `rk-fixture-child-claude`'s marker files (`s7c` §7c.17), reached by pointing `harness_binary` at it — the
fixture repository's `[adapters] harness_binary` key (§7d.1's `ConfigFile` row) for a spawned `redkiln`, the
`rk_adapter_claude::Config` value for an in-process verb; the binary is located as the sibling of
`env!("CARGO_BIN_EXE_redkiln")` and its mode travels in the child environment (§7d.13). **Every in-process source-scan
test is self-controlled**: it runs its scan over its `lint-controls/` file and asserts ≥ 1 hit **before** asserting zero
hits over `src/`, so a scanner that visits nothing is red (Halt 8 (a)/(g); the shell `rk_control` covers the block's
greps, this rule covers the Rust tests' scanners). **Every
process-spawning test is bounded** (30 s unless stated; Halt 5) and spawns with `Stdio::null()` on stdin (D-15).
**Every negative-only test names its mutant and is watched red on it, ADR-0104's form** —
`ac412_init_installs_zero_hooks` (an `init` that writes `.git/hooks/pre-commit`), `ac402_status_and_explain_do_not_reconcile`
(a `reconcile_at_entry(Entry::Doctor)` call added to `status`), `ac441_no_migrate_verb_in_tree` (a `migrate` subcommand),
`doctor_fix_flag_absent_from_snapshot` (`--fix` on `doctor`), `ac402_no_skip_reconcile_flag_in_snapshot` (`--fast` on
`run`), `ac429_rebuild_writes_no_object_ref_or_file` (a `GitOp::Add` in `rebuild.rs`),
`ac436_keywords_permutation_leaves_selection_unchanged` (an evaluator clause reading `keywords`); each slice body quotes
the mutant's red beside the test's green, because a negative observed green on the untouched tree is not red-first
evidence (golden constraint 12: *"A test never observed red is not evidence that it can fail"*).
**Every refusal test asserts two things in order**: the exit code (or `Err`) — red by itself on an implementation that
proceeds — **and** the side-effect count (store rows, marker files, commits, tree bytes) that turns a
warn-and-continue implementation red on the second assertion. Each bullet names its red and the weaker red a shallow
test would accept.

**`-p rk-cli`:**

- `ac400_help_snapshot_every_verb` — AC-400, AC-186, AC-264, AC-441 (router half). Red by any diff between `--help` of
  the root and every verb and the committed `tests/cmd/*.trycmd` pair; a weaker red snapshots the root only, or
  regenerates the snapshot in the same run (`TRYCMD=overwrite` is never set in CI or the block).
- `ac400_denylist_absent_from_parser_tree` — AC-400 (lint half), ADR-0107 decision 3 (D18-13). Red by adding
  `--resume` to `run`'s derive (the mutant this bullet declares), by an alias `resume` on `run`, or by a subcommand
  three levels down carrying `--from`; a weaker red greps source text (a comment or a doc string naming `--resume`
  would fire, a flag built from a `const` would not), or walks the top level only. The same test also scans the
  committed `tests/cmd/*.trycmd` files for the eleven names (comment-free files, exact-token match) after asserting the
  file set is non-empty — the register's third conjunct, *"no denylisted name appears anywhere in the snapshot"*, which
  the tree walk alone does not pin (it pins the parser, not the snapshot bytes).
- `ac400_resume_refused_as_unknown_argument` — AC-400, AC-264. Red by `redkiln run --resume` exiting anything but `2`,
  or by stderr not matching `unexpected argument '--resume'`; a weaker red asserts a non-zero exit (a `run` that parsed
  `--resume` and then failed on a missing item also exits non-zero).
- `ac402_no_skip_reconcile_flag_in_snapshot` — AC-402 (snapshot half). Red by `--no-reconcile`, `--skip-reconcile` or
  `--fast` anywhere in the tree; a weaker red checks `run` only.
- `ac441_no_migrate_verb_in_tree` — AC-441 (router half). Red by a `migrate` subcommand or alias at any depth.
- `doctor_fix_flag_absent_from_snapshot` — RB-M-06; design §8.1's CONDITIONAL rows. Red by `--fix` or `--probe` on
  `doctor`.
- `adr0128_run_lane_value_enum_closed` — ADR-0128 decisions 1 and 2. Red by `--lane hotfix` accepted, by `--item`
  accepted, by `--lane` required; a weaker red tests `fix` and `story` only.
- `adr0129_answer_grammar_positional_option_no_token_flag` — ADR-0129 decisions 1 and 2, AC-435 (usage arm). Red by
  `--token` accepted, by `answer x approve` accepted without `--explain-sufficed`, by `--explain-sufficed yes` accepted
  without `--toml-opened`, by `--explain-sufficed maybe` accepted; a weaker red asserts one of the four.
- `ac438_exit_table_snapshot` — AC-438. Red by any change to `Exit::ALL`'s rendered `code variant` lines against the
  committed `insta` snapshot; a weaker red snapshots the count.
- `ac438_exit_codes_distinct` — AC-438's *"distinct values"*. Red by two variants sharing a code, and by the
  non-`StepError` set being anything but `{0, 2, 3, 4, 5, 70, 130}`; a weaker red checks distinctness only.
- `ac438_step_error_mapping_is_exhaustive` — AC-438's *"a new `StepError` variant with no mapping fails to compile"*.
  Red by a `_ =>` arm in `From<&StepError>` (the declared mutant) or by fewer than eleven arms; a weaker red trusts the
  compiler without checking for the wildcard that would defeat it.
- `ac403_no_blocking_stdin_read_in_rk_cli` — AC-403 (lint half), D-15. Red by `stdin()` or `io::stdin` on a non-comment
  line under `crates/rk-cli/src` (not `read_line(` — any `BufRead` has one; golden constraint 9); a weaker red exempts
  `init`.
- `common_dir_discovered_from_inherited_cwd_via_wrapper` — H-3, D-18. Red by a `status` from a subdirectory that fails
  to find the repository, by an identity row stamped relative (`./.git`) instead of the absolute common dir, by a `-C
  <repo>` the parser accepts and the discovery ignores (the second arm), and separately by `env::current_dir` in either
  crate (the block's ban); a weaker red runs from the root only.
- `ac485_adapter_set_resolved_once_and_run_scoped` — AC-485, RT-41. Red by a second `AdapterSet::resolve` or a second
  `ConfigFile` read after the mid-run config edit (the counting seams), by a dispatch reaching any object but the one
  `Harness` handed in, or by the first row lacking `adapter_set` / `config_digest`; a weaker red mutates the config
  after the run, or asserts *adapter identity on both dispatch events* — a field `DispatchRecord` does not carry.
- `ac486_misconfigured_adapter_refuses_before_spawn_or_store_write` — AC-486. Red by exit ≠ `3`, by stderr not naming
  `rk-adapter-linear` (a warn-and-continue implementation also exits `3`, at step (7)), by a marker file appearing, or
  by the store file appearing; a weaker red asserts the exit only. A second arm over `tests/fixtures/config/missing-store.toml`
  (the scaffold with the `store =` line removed) exits `3` naming the absent key with the same zero-spawn / no-store
  assertions — the GIVEN's second disjunct, red by a resolver that never checks required configuration. **Two arms by
  slice**: Slice 2 lands the in-process arm (`AdapterSet::resolve` over the two fixtures returns `Err(Refusal)` naming
  `rk-adapter-linear` / the absent key, zero marker files, no store file); Slice 3 adds the spawned-`run` arm (§7d.5).
- `ac486_init_scaffold_resolves_four_reals` — AC-486 (positive control), D-14. Red by `AdapterSet::resolve` refusing
  the §7d.9 scaffold or resolving fewer than four names.
- `ac426_unread_window_runs_required_no_default` — AC-426's *"a required key, no default"*. Red by
  `missing-doctor.toml` or `missing-key.toml` loading, and by a `Default` impl, an `unwrap_or(` or any `serde(default`
  attribute on the field in `config.rs` (a source scan over the three spellings); a weaker red tests the missing table
  only. **Slice 2 asserts the `ConfigFile` load arm**; the `run` / `doctor` exits at `3` are asserted when those verbs
  exist (Slices 3 and 4).
- `ac474_config_surface_snapshot_has_no_dual_write_key` — AC-474 (config half). Red by `dual-write.toml` loading
  under `deny_unknown_fields`, and by any key in the `ConfigFile` schema snapshot matching `dual|compat|legacy`.
- `ac437_init_twice_identical_tree_and_identity_rows` — AC-437, ADR-0107 decision 2. Red by a second `init` that
  rewrites `config.toml` (bytes differ), changes the porcelain listing, re-stamps an identity row, or exits non-zero;
  a weaker red compares mtimes, or runs `init` once.
- `ac437_init_scaffold_bytes_pinned` — AC-437 as amended (D-14). Red by any byte of the written file differing from
  `init/config.toml.expected`; a weaker red parses both and compares values (a reordered key or a dropped comment
  would pass).
- `ac531_init_created_names_match_regex` — AC-531 (lint + tree diff). Red by `foo`, `redkiln-foo`, `rk-Foo` or `rk-`
  passing the lint, by `rk-foo` failing it, or by a name outside `.redkiln/` in the tree diff after `init`; a weaker
  red tests the regex on one string.
- `ac531_preexisting_rk_name_reported_not_overwritten` — AC-531's AND. Red by `rk-notes/` altered or by no report on
  stderr; a weaker red asserts exit `0` alone.
- `ac412_init_installs_zero_hooks` — AC-412 (init half). Red by any non-`*.sample` entry appearing under
  `.git/hooks`.
- `ac413_store_opened_from_other_repo_refused_zero_rows` — AC-413 as amended. Red by exit ≠ `3`, or by the store
  file's sha256 changing across the refused `status` (a spawned binary has no counting seam; the zero-reads half of the
  THEN is `rk-store`'s own AC-179/AC-180 instrument at Step 6); a weaker red asserts the refusal only.
- `ac422_config_reads_the_one_denylist` — AC-422 (one-list half), AC-488 (edge half). Red by a second
  `"ANTHROPIC_API_KEY"` literal in `crates/rk-cli/src` (a source scan), or by `AdapterSet::resolve` not refusing a
  captured map carrying `ANTHROPIC_FOUNDRY_API_KEY` (a name only the adapter's list carries); a weaker red tests
  `ANTHROPIC_API_KEY` alone.
- `adr0083_config_constructed_once_at_root_and_threaded` — ADR-0083, RB-M-05. Red by `with_config` called other than
  once, by a `std::env::var` outside `env.rs` (a source scan), or by the first row's `config_digest` differing from
  the value's digest.
- `signal_handler_fires_cancel_token` — ADR-0099 decision 2, D18-2. Red by the `signal.rs` closure not flipping the
  token, by it writing a row, printing, or exiting; **this IS the closure-level test** — the OS-delivery arm is ARMED
  (§7d.10), and no test signals its own process group.
- `adr0118_acquire_returns_halt_on_poisoned_lock` — ADR-0118 clauses 1, 3, 5; RB-M-01 (Slice 2). Red by
  `store::acquire` returning `Ok(guard)` over a poisoned `Arc` (a recovery), or by a panic (exit 101's cause) instead
  of `Err(Halt::PoisonedLock)`.
- `signal_outside_dispatch_exits_130_writes_nothing_partial` — H-2, RA-M-04 (Slice 3). Red by the verb function
  returning anything but `Ok(Exit::SignalOutsideDispatch)` on a pre-fired token, by a second row, or by a commit; a
  weaker red asserts the exit only.
- `adr0118_poisoned_store_lock_halts_run_exit_70` — ADR-0118 clauses 1, 3, 5, 6; RB-M-01 (Slice 3). Red by the verb
  returning `Ok(_)` or a `StepError` row over the poisoned `Arc` (a recovery or a `StepError`), by `code_for` mapping the
  halt to anything but `70`, by `render_halt` not naming the lock, by `is_lock_halt` false on the kernel's `LOCK_HALTS`
  payload, by a marker file (subsequent activity), or by a new row (a recovery that wrote); a weaker red asserts a
  non-`Ok` return alone.
- `ac402_reconcile_called_on_run_answer_doctor` — AC-402 (three paths), AC-172's CLI end. Red by any of the three
  reaching a store read before its recorded `EntryReconciler` call, by a wrong `Entry`, or by zero calls (the seam and
  the counting store fake are the observables — `GitExec` has no journal, §7d.5); a weaker red counts calls without
  ordering.
- `ac402_status_and_explain_do_not_reconcile` — AC-402's negative, ADR-0105 clause 8. Red by one recorded reconcile call
  on either.
- `ac403_gate_open_writes_trailer_and_event_and_exits_zero_without_waiting` — AC-403, AC-240 (twin), D-01, H-5. Red by
  exit ≠ `0`, by no `Gate:` trailer on the run-owned branch's `HEAD`, by a `Gate:` trailer (or any commit) on the fixture
  checkout's `HEAD` (AC-518 — the breach `authorise_tier1_write` refuses), by no `gate_opened` row, by a wall clock over
  ~~the 5 s assertion (the 30 s figure is the kill)~~ the 30 s kill bound, whose expiry is the assertion *(dated 2026-09-30, s8g build, package CI-AC403-8g; §7d.5's AC-403 bullet)*, by a `deadline` outside `[t_before, t_after] + gate_deadline_secs`, or by a live descendant; a weaker red asserts the trailer without the exit code, or reads `HEAD` of the
  checkout.
- `ac404_rate_limited_writes_retry_mark_and_exits` — AC-404 (exit half). Red by exit ≠ `15`, by no mark, or by a wall
  clock over 2 s.
- `ac405_gate_answer_has_exactly_one_call_site` — AC-405 as amended (D-13). Red by zero or two non-test call sites of
  `gate::answer(` outside `rk-kernel/src/workflow`, or by the one site not being under `verbs/answer.rs`.
- `ac406_answer_after_commit_is_void_gate_stays_open` — AC-406. Red by `Applied` after a commit (a hash taken before
  the reconcile — RT-43), by a missing `gate_answer_void`, by a stage transition, by a new trailer commit on the run
  branch, or by any commit on the fixture checkout (AC-518).
- `ac407_second_token_refused_one_gate_answered_event` — AC-407. Red by two `gate_answered` rows, by `git rev-list
  --count rk-run/<item>` moving across the second `answer`, or by exit `0` on the replay.
- `ac248_answer_without_live_token_exits_named_outcome` — AC-248 (CLI half), AC-257's read. Red by exit `0`, by an
  unnamed outcome, by any **answer** row written on an item with **no gate record** (the store gains exactly the
  invocation's `reconcile_started` row — AC-172; §7d.6's baseline rule — and no `gate_*` / `artifact.*` row), or by an
  expired gate answered as live.
- `ac264_run_after_applied_answer_resumes_past_gate` — AC-264's *"`redkiln run <item>` is sufficient"*, AC-186's
  *"a resumed run"*. Red by a `run` after an `Applied` answer that re-opens the gate, refuses, or fails to reach the
  next step; a weaker red asserts the exit alone.
- `ac257_expired_gate_resolved_by_fresh_process` — AC-257. Red by a gate whose deadline passed not resolving at the
  next `run` in a fresh process (no `gate_expired` row; or `gate::answer` applying it — the expiry pass of §7d.5 step
  (9) is what makes the deadline observable, since the kernel refuses `Expired` only on a row already in the journal — ***corrected 2026-09-20, R-44: that refusal is unreachable for a real expiry; see §7d.6***);
  a weaker red resolves it in the same process that opened it — the deadline passes through the `pack-min/` variant
  declaring a 1-second gate deadline and a test-side wait past it (a test may sleep; the product never does), no clock
  toggle (AC-474); `ac248_…`'s expired arm uses the same variant.
- `ac410_unadmitted_attachment_refused_zero_events` — AC-410 (negative), ADR-0063's refusal. Red by exit ≠ `2` on an
  unadmitted kind or on a missing declared attachment, by the refusal's variant being anything but
  `OptionError::AttachmentNotAdmitted { kind }` / `AttachmentMissing { kind }` (an `UnknownOption` refusal — the fixture
  gate resolving to the default question instead of `decide` — is red), or by any answer row written (the
  `reconcile_started` baseline, §7d.6); its binary-level twin (`approve --attach Findings=…` exits `2`) needs no seam.
- `ac410_admitted_attachment_stored_as_human_root` — AC-410 (positive control, as amended). Red by an admitted
  attachment refused (the refuse-all validator D-04 named), by no `artifact.write` row, or by a row whose payload is
  outside ADR-0116 decision 1's shape (`kind`, `item`, `schema_version`, `digest` — an `instance` or `derived_from`
  member is schema drift, decision 5).
- `ac263_stub_channel_refused_named_outcome` — AC-263. Red by `submit(Channel::TrackerComment, …)` applied, by an
  unnamed error, by a store read before the refusal, or by a row written; a weaker red has no stub channel and tests
  nothing (the register's own vacuity warning — a match arm over an enum with one live variant — is disclosed in §7d.6).
- `ac435_answer_refuses_without_explain_sufficiency_and_records_it` — AC-435 (machine half), ADR-0129 decision 1. Red by
  an answer accepted without either flag, by `gate_answered` lacking `explain_sufficiency` or either of its two members,
  by a recorded member differing from its flag (`sufficed: yes, toml_opened: yes` on one gate — SM-126's kill cell —
  `no` / `no` on another), or by `answerer` differing from the answer commit's `%an <%ae>` (RT-42); a weaker red tests
  the refusal only, one flag only, or one value.
- `ac401_second_run_issues_no_dispatch_for_memoised_step` — AC-401 (wiring), SM-130. Red by two `dispatch` rows, by
  the second run's `next_action` returning `Dispatch` for the committed node (a fold not seeded from the store), by
  `ReplayError::NotCommitted` (the `record()` path), or by the `activity_committed` digest changing.
- `ac430_explain_prints_both_row_families` — AC-430. Red by either family empty for any node while the other is
  populated, and by the snapshot diff.
- `ac431_explain_names_card_skills_fillers_tier` — AC-431 (**PROVISIONAL** — the card by its recorded `card_digest` is
  the one field the run record carries; skills, fillers and tier are on no recorded value, §7d.7). Red by the card
  absent when the record has it; the three unrecorded fields are RAISED, not asserted.
- `ac432_explain_from_record_only_and_errors_without_resolution` — AC-432 (both halves), H-2's `5`. Red by exit ≠ `5`
  on `no-resolution/`, by any output derived from a fresh selection, or by `explain.rs` naming a `select` path.
- `ac432_explain_over_replayed_log_is_byte_identical` — AC-432's replay assertion. Red by one byte differing; the
  declared mutant is a render that reads the clock or the process environment (a `Clock::now()` timestamp or a
  `%USERNAME%` in the header), which the replay cannot reproduce — a weaker red renders the same record twice and calls
  it a replay.
- `ac433_explain_names_excluding_clause_never_not_selected` — AC-433 (**PROVISIONAL** — the clause is re-read off the
  recorded rows and the recorded `context.json`, not off a recorded exclusion, §7d.7). Red by the bare string `not
  selected`, or by an exclusion of one of the three kinds not naming its clause.
- `ac434_unknown_facet_rendered_unknown` — AC-434. Red by any of four `unknown` axes rendered as a band, or by any of
  the four not rendered as the literal `unknown` with its treatment stated (a renderer that omits them passes the band
  check alone).
- `run_on_init_only_repo_is_a_config_outcome` — D18-8, RT-46, AC-172. Red by exit ≠ `3`, by a message other than *"no
  process pack enabled"*, by a store row count ≠ 1, by the one row not being `reconcile_started`, or by a marker
  file.
- `adr0128_lane_derived_from_item_or_refused` — ADR-0128 decision 2. Red by a kindless item running under a default
  lane, by a disagreeing `--lane` accepted, or by a derivable kind not deriving.
- `adr0128_lane_maps_to_pack_fix_bug_standard_story_story` — ADR-0128 decision 3. Red by either mapping wrong, or by
  a lane whose pack is not enabled running.
- `ac436_catalog_search_finds_by_keyword` — AC-436 (positive control), RA-M-02. Red by a search over a catalog whose
  one activity carries the keyword returning nothing — the tree's field-dropping loader today; the keyword is a token
  that appears in **no other field of any activity** (asserted by the test over the synthetic catalog's serialised bytes
  minus `keywords`), so a search over ids or descriptions is red too.
- `lint_wraps_kernel_facet_rules_and_exits_by_table` — the CLI surface `rk-kernel`'s `lib.rs` owes; AC-071/AC-520
  are not claimed. Red by the `llm`-sourced fixture exiting `0`, by the clean pair exiting non-zero, or by an exit
  outside the table.
- `ac438_table_driven_exit_codes_over_fixtures` — AC-438's third instrument. Red by any fixture's code differing from
  its `expected` file. **Through the binary**: `2`, `3`, `5`, `0` (gate-open), the error-severity AC-415 doctor
  fixtures at `3`, the warning/ok-severity ones at `0`, and the adapter-classifiable classes through the fixture child
  (`Transport` via a missing `harness_binary`, `Schema` via **the second arm of `extract_digest`** (***corrected 2026-09-20, R-60: a malformed frame exits `10` via `classify.rs` rule 8's `Transport { kind: Decode }`, not `11`; see §7d.4***), `Cancelled { by: Timeout }` (row `13`)
  via `hold` under a 1 s `timeout_secs` — the adapter's timer arm, `s7c` §7c.10 — `Platform` via an `is_error` frame).
  **In-process**: all eleven `StepError` rows through the verb
  function with `FakeHarness::scripted` returning each variant, one element per admitted attempt (`Timeout`, `NotAdopted`,
  `NotIsolated`, `Unsupported`, `Overage`, `RateLimited` and the other two `Cancelled` arms are reachable only here); `70` through
  `code_for` (§7d.10). The doctor-fixture arm lands in Slice 4 with `fixtures/doctor/` and `verbs/doctor.rs` (§7d.16). A weaker red drives the in-process table
  alone, or the binary alone.
- `signal_during_dispatch_is_cancelled_by_signal_exit_13` — RA-M-04, ADR-0099 decision 2, AC-225's `Signal` arm
  (in-process; §7d.10). Red by `Ok(Exit::SignalOutsideDispatch)` (the root returned before the adapter recorded), by
  `cancelled_by: timeout`, or by a live child.
- `ac415_every_rule_id_has_a_rejecting_fixture` — AC-415. Red by a rule id with no `doctor/<id>/`, by a fixture its own
  rule does not reject, or by a fixture directory with no rule; a weaker red joins names without running the rule.
- `doctor_rule_table_covers_every_register_named_row` — RA-M-01. Red by any of the twenty-nine floor ids absent from
  `RULES`; a weaker red asserts `RULES` non-empty.
- `ac414_doctor_over_corpus_fires_every_expected_rule_with_zero_dispatches` — AC-414, SM-121. Red by a fixture whose
  expected id does not fire, by the `NoDispatchHarness` panicking (a dispatch was issued), or by the enumerated
  directory set (minus `clean/`) being empty or unequal to `RULES`' id set (the self-control before the loop); a weaker
  red runs one fixture, or loops over whatever is there.
- `ac414_doctor_module_names_no_adapter_constructor` — AC-414 as amended (D-10), D18-7. Red by `ClaudeHarness::new`,
  `rk_adapter_claude::config::Config::new`, a bare `Config::new(` after a `use rk_adapter_claude::config::Config`,
  `NodeToolchain::new`, `RustToolchain::new`, `SqliteStore::open` or `dyn Harness` named on a non-comment line under the
  `doctor` module — call sites **and** `use` imports; `env_denylist::` and `config::TESTED_CLI_VERSION` are admitted;
  self-controlled over `d10_adapter_constructor_control.rs` first.
- `ac409_gate_outputs_binding_rejected` · `ac412_redkiln_hook_in_git_hooks_is_an_error` ·
  `ac413_ac369_ac370_store_identity_rows` · `ac370_stray_store_under_linked_worktree_is_an_error` ·
  `ac416_role_unresolved_and_ambiguous_rejected` ·
  `ac417_card_without_structured_output_or_tools_key_rejected` · `ac418_tier_leak_on_card_model_and_filler_effort` ·
  `ac419_missing_or_non_numeric_timeout_rejected` · `ac420_dangling_role_slot_attach_rejected` ·
  `ac421_dead_definition_and_unfilled_slot_reported` · `ac422_ac488_denylisted_env_name_is_a_doctor_error` ·
  `ac423_ac526_cli_version_and_ttl_mismatch_rejected` · `ac424_unknown_facet_value_and_new_facet_rejected` ·
  `ac425_facet_without_registered_fact_rejected` · `ac486_unknown_adapter_is_a_doctor_error` ·
  `ac203_agents_without_agent_doctor_row` · `ac383_card_without_cache_profile_doctor_row` ·
  `ac377_margin_under_two_ticks_doctor_row` · `ac220_last_reading_within_margin_doctor_row` ·
  `adr0105_doctor_rows_network_fs_credential_orphan` ·
  `ac290_reproduced_finding_without_reexecution_record_doctor_row` ·
  `ac306_live_quarantines_listed_with_ages_doctor_row` ·
  `ac310_sink_count_below_finding_count_doctor_row` ·
  `ac530_advisory_ledger_imbalance_doctor_row` ·
  `ac314_regression_corpus_sha_unresolvable_doctor_row` ·
  `ac455_verification_none_under_requiring_pack_doctor_row` (the six D18-19 gate rows) — one per row family of §7d.8's table, each: the rule over its
  rejecting fixture(s) yields the finding with the right id and severity, **and** over `doctor/clean/` yields none.
  Red by a rule that never fires (the first half) and by one that always fires (the second); a weaker red omits the
  clean run. `ac413_…` is red over a store whose `root_commit` is another repository's and green over one whose
  `common_dir` was re-stamped by a move (the three-field reading; a boot-id assertion is unauthorable and none is
  written). `ac412_…` is red on the clean run by a recogniser that fires on any non-`*.sample` hook — `clean/` carries
  an operator hook with no marker (§7d.13). `ac409_…` is red by `RK-D-DANGLING-*` firing beside `RK-D-GATE-OUTPUTS` on
  its fixture (the gate declares `outputs`, so the binding is not dangling — §7d.8's row).
- `ac423_probe_is_one_version_spawn_not_a_dispatch` — AC-423 as amended (D-11), SM-121. Red by a spawn count ≠ 1, by
  `-p` on the child's argv, or by the dispatch counter ≠ 0.
- `ac426_ac472_unread_window_49_not_flagged_51_flagged` — AC-426, AC-472 as amended, AC-147 as amended, SM-139; N = 50
  (E9). Red by the 49-run log flagging, by the 51-run log not flagging, by the same logs under N = 60 flagging, or by
  the read-once kind flagging; a weaker red hard-codes 50 and tests one log.
- `ac426_rule_fire_leaves_write_set_unchanged` — AC-426's *"nothing is deleted or rewritten"*. Red by zero
  `RK-D-UNREAD` findings over `runs-51.jsonl` (the positive half — a rule that never fires passes the write-set check
  trivially), then by any byte of the store or the tree changing across the rule.
- `ac397_rebuilt_store_tier2_emptiness_is_ok_expected` — AC-397. Red by any severity but `ok`, or by the reason string
  absent.
- `ac427_rebuild_from_git_alone_matches_recorded_next_action` — AC-427 (verb + direction). Red by any item's
  `stage_action` differing after delete-and-rebuild over the synthetic trailer repository, or by the pre-delete set
  being initial for every item (a pre-delete store `--rebuild` itself produced); a weaker red asserts the store file
  exists, or seeds the pre-delete store with a rebuild.
- `ac429_rebuild_writes_no_object_ref_or_file` — AC-429. Red by an empty fingerprint set, by a rebuild that recreated
  no store or wrote no tier-1 row (the two positive controls), then by any `.git` file digest, `HEAD`, ref or tree hash
  differing (the fingerprint is the whole negative instrument); the declared mutant is a `GitOp::Add` in `rebuild.rs`.
- `ac438_doctor_error_rule_exits_config_error` — AC-438's doctor instance. Red by exit ≠ `3` over `RK-D-ADAPTER`'s
  fixture, ≠ `0` over `clean/`, or ≠ `0` over a warning-only fixture.

**`-p redkiln-migrate`:**

- `refuses_dirty_target` — AC-522. Red by a write of any kind before the refusal (worktree, ref, file), by the
  refusal not naming the dirty path, or by only one of the two variants (untracked; modified) refused; a weaker red
  checks the refusal after a partial write.
- `idempotent_across_kill` — AC-450's kill case, AC-522, D18-12. Red by the kill landing outside the window
  (`rev-list --count <target-HEAD>..rk-migrate/<short>` already `1` before the re-run, or `migrate-wt/` empty), by the
  target branch's `HEAD` or tree moving across the kill, by the re-run's tree digest differing from the uninterrupted
  run's (**Halt 4**, not a retry), or by the kill landing anywhere but after `phase: written`; a weaker red simulates
  the kill in-process only, or asserts the target branch alone (which the commit never lands on by construction).
- `phase_written_marker_is_printed_before_commit` — D18-12. Red by the marker absent or printed after the `Commit`
  op.
- `ac450_second_run_is_noop_report_to_stdout_no_commit` — AC-450, AC-447's no-op exemption. Red by a second commit,
  by a non-empty `git diff`, by exit ≠ `0`, or by the report not on stdout.
- `ac447_exactly_one_commit_on_migration_branch` — AC-447, AC-522. Red by a count ≠ 1 or a commit on the target's
  original branch.
- `ac447_second_write_phase_aborts_before_writing` — AC-447's AND. Red by any file written in the second phase, or by
  a commit.
- `ac448_report_lands_in_the_same_commit_and_validates` — AC-448, AC-467's required fields. Red by `report.json`
  absent from the commit's tree, or by it failing the schema.
- `ac448_write_without_report_is_refused` — AC-448's AND. Red by a commit landing with the report step disabled.
- `ac468_schema_rejects_total_only_and_folded_forms` — AC-468 (schema half). Red by a `total`-only row validating, by
  a row with an extra key validating, or by the consumer accepting `lifted + lifted_with_loss` folded; a weaker red
  validates the happy path only.
- `ac475_zero_change_summary_values_in_output` — AC-475 (tripwire). Red by a `ChangeSummary` value in any migrated
  artifact, or by the identifier in `crates/redkiln-migrate/src`.
- `diff_prose_carried_opaque_with_loss_marker` — claimed under **no id** (AC-475's loss-marker half is Phase 2's by
  E13; built here as the migrator's carry, as `free_form_gate_result_…` is) over `legacy-mini`'s diff-prose item. Red by
  the section typed, or by no loss marker.
- `ac449_two_targets_byte_identical_under_tz_and_read_order` — AC-449. Red by either tree digest or report digest
  differing across the in-process read-order arm (natural listing, then reversed, through the lift entry's `DirOrder`
  seam) or the spawned `TZ` arm (disclosed weak — the migrator reads no clock, §7d.11); a weaker red runs twice under
  one environment, or drives the seam from a spawned process (unreachable without a toggle AC-474 forbids).
- `ac466_ac441_migrate_depends_on_no_harness_adapter_or_ports` — AC-466, AC-441 (build-graph half). Red by any
  `rk-adapter-*`, `rk-ports` or `rk-kernel` node in `redkiln-migrate`'s normal/build closure, or any `rk-*` package
  depending on `redkiln-migrate`; the walk must reach `rk-types` and `rk-git` (its control).
- `ac470_migrated_artifacts_round_trip_through_canonical` — AC-470. Red by one artifact's re-serialisation differing.
- `typed_gate_result_round_trips_through_canonical` — claimed under **no id** (AC-477 is ARMED whole — its WHEN needs a
  render path the tree does not carry, §7d.11). Red by a typed gate result written by the lift reading back unequal from
  the store's typed value.
- `free_form_gate_result_carried_opaque_with_loss_marker` — no id (AC-477's carry; S-MIG decision 2). Red by the
  free-form result parsed into a typed gate result, or by no loss marker.
- `migrated_artifacts_carry_schema_version_and_no_sig` — AC-471 (write half), over the migrator's output. Red by an
  artifact without `schema_version` or with a `*_sig` field. (AC-478 is ARMED — §7d.11.)
- `ac442_lint_flags_legacy_reader_outside_legacy_module` — AC-442's two must-fail instruments. Red by the scan not
  flagging `ac442_template_sig_control.rs` at a synthetic `crates/rk-store/src/` path, or not flagging a copy that
  parses `---` frontmatter.
- `ac474_no_dual_write_key_flag_or_env` — AC-474 (migrator half). Red by a flag outside `{--target, --dry-run,
  --version}`, or by `std::env::var` in the crate's `src`.
- `ac442_legacy_readers_only_under_legacy_module` — AC-442, AC-471 (write half). Red by `template_sig`,
  `rendered_sig`, `yaml_serde` or a `processes/*.yaml` path on a non-comment line outside `src/legacy/` in any
  `crates/*/src/**/*.rs` (never `tests/`, where the lint-control and this crate's own assertions name the literals),
  and by `lift.rs` writing either literal.
- `ac522_worktree_recreated_never_reused` — AC-522, D18-11, ADR-0105 clause 6. Red by a stale `migrate-wt/` reused
  (its pre-existing file surviving), by the target branch moving, or by a `git` spawn outside the wrapper (the block's
  `Command::new("git")` ban is the instrument; `GitExec` keeps no journal).
- `adr0075_six_typed_kinds_eight_opaque` — AC-143 (the list), ADR-0075 clause 6 (the treatment), DECISIONS-2026-09-11
  #3. Red by any of six lifting opaque, any of eight lifting typed, a fifteenth kind accepted, or fewer than fourteen
  kinds found in `legacy-mini`.
- `legacy_mini_provenance_matches_bytes` — D18-10, AC-445's and AC-443's Phase-1 evidence (both PROVISIONAL, §7d.18),
  AC-465. Red by any fixture file's sha256 differing from `PROVENANCE.md`, or by a file under the fixture the note does
  not list; it cannot be red on a copied predecessor file whose sha256 the note lists, which is why neither id is
  discharged on it.
- `ac463_corpus_differential_runner_named` — **authored, `#[cfg(feature = "corpus")] #[ignore]`, not run**; the block
  asserts the `fn` exists.

**`-p rk-kernel`** (each re-opens Checkpoints 7b and 7b-ii — §7d.15):

- `adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs` — ADR-0083, RB-M-05, D18-14. Red by
  `RunInputs` having no `with_config`, by a `None` config changing `to_json()` (and so every recorded `run_id` — the
  7b fixtures are the control), or by `Config` being constructible from the environment inside the kernel (a source
  scan: no `std::env` in `config.rs`).
- `catalog_load_diagnostic_returns_every_load_error_typed` — the fourth kernel edit (§7d.1, §7d.8; RT-54). Red by
  `load_diagnostic` stopping at the first error over a pack carrying **two `Shape` defects** — a `Gate` step with
  `outputs` **and** a second malformed step (the loader's own vocabulary is `Io` / `Toml` / `Shape`; it reads no
  `timeout_secs` and validates no facet, §7d.8), or by the strict loader's first-error result differing from
  `load_diagnostic`'s first typed error.
- `ac409_gate_step_with_outputs_rejected_at_load` — AC-409 as amended (RA-M-05), the seventh kernel edit. Red by
  `Catalog::from_sources` accepting a `Gate` step carrying `outputs` (AC-409's WHEN — the strict loader parses `outputs =`
  for every `StepKind` today); positive controls in the same test: the same pack with the gate's `outputs` removed loads,
  and a `Tool` step carrying `outputs` loads — so a loader that refuses every `outputs` or every `Gate` step is red; a
  weaker red asserts `load_diagnostic`'s refusal alone and leaves `run` loading the graph, or has no positive control.
- `replay_resume_seeds_fold_from_journal_and_steps_gate_open` — the sixth kernel edit (§7d.5 step (9); RT-55). Red by a
  `resume` over a journal holding an applied answer re-deriving `OpenGate` at reach 1 (a fresh local fold), by no
  `gate_opened` row appended when `next_action` returns `OpenGate` (the `record()` break), by a dispatched node whose
  binding the supplier served being refused on `inputs/specs.json` (the seam is not consulted), or by `record()`'s own
  recorded output changing (its 7b fixtures are the control).
- `adr0129_gate_answered_carries_answer_meta_kernel_side` — the fifth kernel edit. Red by `gate_answered_event`'s payload
  lacking `explain_sufficiency` or `answerer` when `gate::answer` is given an `AnswerMeta`, or by the fold
  reading anything but `token` / `tree_hash` / `verdict` (the 7b-ii tests, passing `&QuestionRegistry::default()`, `&[]`
  and `AnswerMeta::default()` at each of their ~20 call sites, are the control; every exhaustive match over `GateError`
  in the kernel's tests gains the `OptionRefused` arm).
- `ac436_keywords_permutation_leaves_selection_unchanged` — AC-436 (property test). Red by a permuted or rewritten
  `keywords` changing the ordered set of selected activity ids — reachable only once the loader reads the field, which
  is why the positive control exists; the declared mutant is an evaluator clause reading `keywords`.
- `ac436_keywords_unreachable_from_admissibility_evaluator` — AC-436 (lint). Red by the identifier `keywords` on a
  non-comment line under `select/` or `predicate/`; its control is `catalog.rs`'s field.
- `adr0129_question_registry_default_question_two_options` — ADR-0129 decision 3. Red by the default question
  carrying anything but `approve` / `changes-requested`, either with `attaches`, or by a gate step naming no question
  not resolving to it.
- `adr0063_option_with_attaches_refuses_close_without_attachment` — ADR-0063, ADR-0129 decision 4. Red by
  `QuestionRegistry::validate` returning `Ok(())` with the declared kind absent, by `gate::answer` returning `Applied`
  (or writing any event) over an otherwise-`Applied` state when the attachment is missing — the kernel's refusal, ADR-0063's
  Decision — or by `validate` refusing the attachment when present.

### 7d.15 The manifests, and what this stage may not disturb

Every new crate's `Cargo.toml` carries `edition.workspace = true`, `rust-version.workspace = true`,
`license.workspace = true`, `publish.workspace = true`; `version = "0.0.0"`; `[features] corpus = []`; one `[[test]]
name = "it" path = "tests/it/main.rs"` (ADR-0083); `[lints] workspace = true` **last** and nothing else in `[lints]`
(ADR-0102 decision 2). **`#[expect(clippy::…, reason = "…")]` per site, never `#[allow(`** — and the lint named is the
one that fires, because `clippy.toml` has two lists: **`disallowed-types`** carries `std::process::Command`,
`tokio::process::Command`, `std::collections::HashMap` and `HashSet`; **`disallowed-methods`** carries
`std::time::Instant::now`, `SystemTime::now`, `std::thread::sleep`, `tokio::time::sleep`, `env::set_var` and their kin
[measured 2026-09-18, `clippy.toml`] — an `#[expect(clippy::disallowed_methods)]` on a `Command` site leaves
`disallowed_types` firing at deny level and adds an `unfulfilled_lint_expectations` warning, and the block's `cargo
clippy --workspace --all-targets` gate goes red on the first such site. So: every `std::process::Command` mention (the
migrator's `Command` handed to `GitExec` is inside `rk-git`; the CLI's probe `Command` handed to `run_bounded`, and
every test that spawns the binary, each take one) and every `HashMap` / `HashSet` (none: `BTreeMap` everywhere) take
**`#[expect(clippy::disallowed_types, reason = "…")]`** (the house form — `crates/rk-git/src/git_exec.rs`, the fixture
child); every `Instant::now` / `SystemTime::now` a test reads for a wall-clock bound (`ac403_…`, `ac404_…`, the 30 s
bounds of §7d.14) and every test-side `std::thread::sleep` past a 1 s gate deadline (`ac257_…`, `ac248_…`'s expired arm,
§7d.13) take **`#[expect(clippy::disallowed_methods, reason = "a test-side bound, not a decision (AC-001)")]`** (the
sleep's reason names AC-003), or are written as
`mpsc::Receiver::recv_timeout`, which reads no clock in our code (`rk-containment`'s `bounded.rs` is the precedent). **No `src/` file in either crate reads the process environment** except `rk-cli`'s
`src/env.rs` (`Env::capture()`, the one read ADR-0083 permits), and `redkiln-migrate` reads it nowhere — the block's
`rk_absent` with the module-path exclusion of `env` for `rk-cli` and none for the migrator. `anyhow` is a normal
dependency of **exactly these two crates** (ADR-0102 decision 6; `ci.yml`'s confinement rule already names both).

**`crates/rk-cli/Cargo.toml`, in full** (the rows the block asserts are these; the description text is the
implementer's):

```toml
[package]
name = "rk-cli"
version = "0.0.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true
description = "The composition root — the `redkiln` binary (design §9.1; Step 7d)."

[[bin]]
name = "redkiln"
path = "src/main.rs"
# src/lib.rs is the library the tests link (an integration test cannot link a [[bin]] or name a pub(crate) item);
# main.rs only installs the handler and the panic hook and exits through code_for (§7d.10).

# ADDED 2026-09-21 at Step 7d's close, recording what Slice 4 cycle 7 (`a65a903`) already landed: the spawnable
# fixture child, compiled FROM rk-containment's source file under a THIRD target name. One bin name in two packages is
# one output path — cargo warns of an output filename collision, the two link jobs race and the MSVC linker exits 1104
# for both [reproduced 2026-09-17]. Tests reach it through env!("CARGO_BIN_EXE_rk-fixture-child-cli"), which cargo sets
# only for the owning package's own tests, never through std::env::var or a path under target/. §7d.13 carries the
# amendment; the block's manifest loop expects TWO here and ONE in the migrator.
[[bin]]
name = "rk-fixture-child-cli"
path = "../rk-containment/tests/fixtures/bin/rk-fixture-child.rs"
test = false
bench = false
doc = false

[features]
# Corpus-bound tests, local-only (AC-464/PIA-002); run as `cargo test --workspace --features corpus`.
corpus = []

[dependencies]
rk-types = { path = "../rk-types" }
rk-ports = { path = "../rk-ports" }
rk-kernel = { path = "../rk-kernel" }
rk-store = { path = "../rk-store" }
rk-git = { path = "../rk-git" }
rk-containment = { path = "../rk-containment" }              # run_bounded for the doctor version probe (D-11)
rk-adapter-claude = { path = "../rk-adapter-claude" }         # the four reals (ADR-0094 d1), constructed once here
rk-adapter-sqlite = { path = "../rk-adapter-sqlite" }
rk-adapter-toolchain-node = { path = "../rk-adapter-toolchain-node" }
rk-adapter-toolchain-rust = { path = "../rk-adapter-toolchain-rust" }
clap.workspace = true
anyhow.workspace = true        # ADR-0102 d6: this crate and redkiln-migrate ONLY
toml.workspace = true          # D18-3: the ConfigFile reader
ctrlc.workspace = true         # D18-2: the signal path, this crate ONLY
serde.workspace = true
serde_json.workspace = true
tempfile = "3"                 # H-3: the bootstrap GitExec's hooks directory, before the common dir is known

[dev-dependencies]
trycmd.workspace = true        # AC-400's --help snapshot
assert_cmd.workspace = true
insta.workspace = true         # the Exit::ALL snapshot
proptest.workspace = true

[[test]]
name = "it"
path = "tests/it/main.rs"

[lints]
workspace = true
```

**`crates/redkiln-migrate/Cargo.toml`, in full**:

```toml
[package]
name = "redkiln-migrate"
version = "0.0.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true
description = "The old-corpus converter — a separate binary, never a `redkiln` verb (AC-441; ADR-0002, ADR-0013). v0 at Step 7d."

[[bin]]
name = "redkiln-migrate"
path = "src/main.rs"

[features]
# The corpus-bound Phase-2 acceptance set (the register's §13 phase note), local-only: `cargo test -p redkiln-migrate --features corpus -- --ignored`.
corpus = []

[dependencies]
rk-types = { path = "../rk-types" }      # companion kinds, digests, typed artifacts
rk-store = { path = "../rk-store" }      # the canonical serialiser (AC-470) and the store the lift writes into
rk-git = { path = "../rk-git" }          # status --porcelain, worktree add/remove, commit_with_trailers, tree_hash (AC-522)
yaml_serde.workspace = true              # ADR-0100 d1: the ONE crate that may depend on a YAML reader; src/legacy/ only
serde.workspace = true
serde_json.workspace = true
jsonschema.workspace = true              # the report validated against schema/report.schema.json (AC-448/AC-468)
clap.workspace = true
anyhow.workspace = true                  # ADR-0102 d6
tempfile = "3"                           # H-3: the bootstrap GitExec's hooks directory (§7d.11 step (2))
# NO rk-ports, NO rk-kernel, NO rk-adapter-* (AC-466; AC-441's build-graph half) — the block's cargo metadata walk polices it.

[dev-dependencies]
rk-containment = { path = "../rk-containment" }   # terminate_pid / is_alive for idempotent_across_kill (dev edge only)
insta.workspace = true

[[test]]
name = "it"
path = "tests/it/main.rs"

[lints]
workspace = true
```

**The root `Cargo.toml` rows — landing in Slice 1, not at stage-open** (so the stage-open commit stays records-only,
as 7c's was bar `ci.yml`), each with its comment, each `>= 1.0` and therefore a caret range under ADR-0100 decision 2
(the 0.x exact-pin rule does not apply; versions measured 2026-09-18 with `cargo search`):

```toml
# ADDED 2026-09-18 (Step 7d Slice 1, s7d §7d.15). All six are >= 1.0, so caret ranges under ADR-0100 decision 2.
clap       = { version = "4", features = ["derive"] }   # 4.6.7; rk-cli and redkiln-migrate — the two binaries
anyhow     = "1"    # 1.0.104; rk-cli and redkiln-migrate ONLY (ADR-0102 d6) — ci.yml's confinement rule names both
toml       = "1"    # 1.1.6; rk-cli's ConfigFile reader with serde deny_unknown_fields (DECISIONS-2026-09-18 D18-3)
ctrlc      = { version = "3", features = ["termination"] }   # 3.5.2; rk-cli ONLY — the signal path ADR-0099 d2 assigns
                    # to the CLI (D18-2); `termination` = SIGTERM/SIGHUP reach the handler on POSIX; its windows-sys
                    # requires "0.61" and unifies with rk-containment's =0.61.2 (one version; no deny multiple-versions warning)
trycmd     = "1"    # 1.2.1; dev — AC-400's --help snapshot (NOT 0.x: the plan's "=0.15.x" was measured wrong)
assert_cmd = "2"    # 2.2.2; dev — the process-level refusals
```

`yaml_serde = "=0.10.7"` is already a workspace row used by nothing until Slice 5. **Any root manifest edit makes
Checkpoint 5 provisional** (D17-12's precedent): Slice 1's commit marks `PROGRESS.md` §1's Step 5 row `passed <hash>,
re-run pending 7d root manifest` and re-runs Checkpoint 5 whole in the same commit.

**`deny.toml`** gains eight `[bans].deny` rows in Slice 1 — `{ name = "octocrab" }`, `{ name = "reqwest" }`, `{ name =
"hyper" }`, `{ name = "axum" }`, `{ name = "ureq" }`, `{ name = "tokio-tungstenite" }`, `{ name = "warp" }`, `{ name =
"actix-web" }` (DECISIONS-2026-09-18 RT-36) — under a comment naming AC-405's crate-graph instrument as
amended (D-13) and stating none is in `Cargo.lock` at `239e899` [measured 2026-09-18], so the rows ban nothing present
and bite on contact; `cargo deny check bans` is the block's fifth gate. The same edit is a Checkpoint 5 disturbance
and is covered by Slice 1's whole re-run.

**`ci.yml`** — four edits, each in the slice whose code makes it non-vacuous: the AC-529 **scope comment** (D-18) landed
2026-09-18 with the records; the AC-465 allowlist rows — `crates/rk-cli/tests/fixtures/explain/*/events.jsonl` and
`crates/rk-cli/tests/fixtures/exit/*/*.jsonl` (Slice 3), `crates/rk-cli/tests/fixtures/unread/*.jsonl` and
`crates/rk-cli/tests/fixtures/doctor/*/events.jsonl` (Slice 4) — four rows, each commented *synthetic 7d fixtures, no
corpus provenance* (§7d.13); the **AC-442 legacy-format lint** (Slice 5) — `git ls-files 'crates/*/src/**/*.rs' | xargs grep -nE
'template_sig|rendered_sig|yaml_serde|processes/[^"]*\.ya?ml'`, comment-filtered, allowlisting
`^crates/redkiln-migrate/src/legacy(\.rs|/)` (layout-neutral: a `legacy.rs` module root beside `legacy/`, §7d.1's rule
that module file names are the implementer's) and the one `rk-store` old-layout detector path by name (AC-440's
predicate), failing on any other hit; and the
**AC-441 metadata line** (Slice 5) — a python walk over `cargo metadata` asserting no package named `rk-*` lists
`redkiln-migrate` among its dependencies and that `redkiln-migrate`'s normal/build closure names no `rk-adapter-*`,
`rk-ports` or `rk-kernel`, with the control that it reached `rk-types`. `ci.yml` has never run; these are authored to
the same discipline as the block's lines and the block runs their local twins.

**What this stage may not disturb, and how each disturbance is disposed of.** **One closed crate is edited** and its
rows go provisional **in the same commit**: **`rk-kernel`** (Checkpoints 7b and 7b-ii, both `passed 6a771d1` — the
re-run recorded at 7c Slice 1; re-read §1 before writing the mark) — the **eight** edits of §7d.1, by slice. Slice 2 adds
`src/config.rs` (`pub struct Config`, `RunInputs::with_config` — the member is `Option<Config>` and is **omitted from
`to_json()` when `None`**, so every recorded `run_id` / `inputs_digest` on the tree is byte-stable and the 7b/7b-ii
re-run is the proof; the digest is recorded on the CLI-written `reconcile_started` row (RT-41) and **no kernel event
payload changes for `Config`**) and `pub const LOCK_HALTS` in `runtime.rs` (visibility only; the text is unchanged —
`rk-cli`'s panic hook matches `rk_kernel::runtime::LOCK_HALTS`, §7d.4). Slice 3 adds `keywords` to `catalog::Activity`
and `catalog::search`; `workflow/question.rs` (the registry, the default question, `QuestionRegistry::validate`);
**`workflow/gate.rs`** — `gate::answer` gains **three parameters** (the one signature change, against today's five —
`state, gate, presented, tree_hash, verdict`): `registry: &QuestionRegistry` and `attachments: &[Attachment]`, validated
by `validate` **inside** the kernel before the mark closes (ADR-0063's *"the kernel refuses to
close the mark unless that attachment is supplied"* — an accepted Decision sentence; the CLI performs the same
validation first so the refusal is a usage error), and `meta: AnswerMeta { explain_sufficiency, answerer }` whose
members `gate_answered_event` adds to its payload **kernel-side** (ADR-0129 decision 1's *"the `gate_answered` payload
gains the field"*; design §5 as amended; RT-42) — never the CLI editing the returned `Event`, so the row stays the
kernel's and AC-262's wrapper stays the single caller-facing entry; `GateError` gains `OptionRefused(OptionError)`;
**the ~20 `gate::answer(` call sites in `crates/rk-kernel/tests/it/gates.rs`** [measured 2026-09-18] are edited to pass
`&QuestionRegistry::default()`, `&[]`, `AnswerMeta::default()`, every exhaustive match over `GateError` in those tests
gains the new arm, and their assertions stand unchanged; and **`replay::resume`** (the store-seeded loop of §7d.5 step
(9) with its spec-supplier seam, RT-55; `record()`
unchanged). Slice 4 adds `catalog::load_diagnostic` (the diagnostic loader mode `doctor` alone calls, §7d.8; RT-54) and
the strict loader's **one** behavioural change — `Catalog::from_sources` refuses a `Gate` step carrying `outputs`
(`CatalogError::Shape`, AC-409's WHEN — `run` must not load the graph AC-409 says is rejected, so this is not
`load_diagnostic`'s alone). Each of the three slices marks §1's rows 7b and 7b-ii `passed <hash>, re-run pending 7d
rk-kernel` and **re-runs both checkpoints whole** in its commit (the kernel's exhaustive `match`es over `EventKind` are
untouched — no kind is minted — so no E0004 cascade of 7c's shape is expected, and a `cargo build --workspace` that stays
green is the proof). **`rk-containment` is NOT edited** (§7d.10: the console-signal sender an earlier draft added there
is not built, so Checkpoint 7c's row is never provisional in this stage). **Checkpoint 6's production-code
`std::process::Command` ban** (`s6` §6.x — the line after `test -s crates/rk-git/src/git_exec.rs`, which counts
non-comment `std::process::Command|tokio::process::Command` hits under `crates/*/src` outside `git_exec.rs` and demands
zero) **is disturbed by both new crates** — `rk-cli`'s probe `Command` handed to `run_bounded`, the `#[expect]` sites
that name the type — **and already reads 13 non-comment hits at `239e899`** [measured 2026-09-18: `git ls-files
'crates/*/src/*.rs' 'crates/*/src/**/*.rs' | xargs grep -nE 'std::process::Command|tokio::process::Command' | grep -vE
'^crates/rk-git/src/git_exec\.rs:' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//' | wc -l` → 13: `rk-containment`'s
`bounded.rs`, `ffi.rs`, `posix.rs`, `windows.rs` and the three adapters' `run.rs` / `runner.rs`] from 7c's sites that
Checkpoint 6 (`passed 6a771d1`, pre-7c-Slice-2) never saw — 7c's undeclared debt, which this stage inherits and must not
claim green: Slice 1 marks §1's Step 6 row `passed 6a771d1, re-run pending 7c/7d Command sites` and RAISES the `s6` line
for narrowing to `Command::new("git")` (its stated warrant, ADR-0077) or an anchored `#[expect]`-aware filter (§7d.18);
this stage does not amend the closed `s6` file. **`rk-types`, `rk-ports`, `rk-store`, `rk-git` and the four
adapters are not edited** (the porcelain read is a call site in the migrator through the existing `GitOp::Status`;
`answerer` reads through the existing `GitOp::Log`; the first-run worktree is `GitExec::run(GitOp::WorktreeAdd, …)` with
`-b`; the poisoned-lock acquisition is `rk-cli`'s function over the existing `Arc<Mutex<dyn EventStore>>`; `GitExec`
has no journal and none is added [measured 2026-09-18, `crates/rk-git/src/git_exec.rs`] — every git-side assertion in
this file is a git-side fact or a CLI seam) — if a slice finds it must edit one, it stops and this section is amended
first. The corpus pin does **not** move.

***AMENDED 2026-09-21, Step 7d Slice 4, on the operator's decision of that date: `rk-adapter-claude` is opened for
ONE edit and one only — the CLI pin.*** *This is the amendment the sentence above requires, taken before a byte of
that crate moved, and it is narrow by construction. **What it permits**: `TESTED_CLI_VERSION` moves `2.1.277` →
`2.1.278`, with the prose that states the pin, the tests that assert it, and the scripted `system/init` fixtures
whose `claude_code_version` must equal the pin to reach the adoption check at all (`init::decide` refuses a
measured version ≠ the pin under `MismatchPolicy::Refuse` — AC-226 as designed, which is how the 2.1.275 → 2.1.277
move was found). **What it does NOT permit**: any other change to that crate, and no change at all to `rk-types`,
`rk-ports`, `rk-store`, `rk-git` or the other three adapters.*

*Why it is taken rather than deferred again. The move was **recorded and half-taken on 2026-09-19** (PROGRESS.md
§3, Step 7d Slice 1 `b2f2842`): `claude --version` read `2.1.278` against a `2.1.277` pin, Halt 2 fired, the
instrument file was re-teed and `TESTED_CLI_VERSION` was deliberately NOT moved with it because this section
forbade it — the operator's call that day was *“record + re-tee, RAISE the adapter edit”*. Checkpoint 7d's Halt 2
compares three things — the live binary, the instrument file and this constant — and only the third now disagrees,
so the stage cannot close until the RAISED bump is either taken or the CLI is pinned back. It is taken.*

*What it costs, stated rather than discovered. **(a)** The pin is not a number but the version the adapter's
recorded evidence was measured at: `init.rs`'s RT-27 shape lines and `streams/README.md`'s *“The 2.1.277 init
shape”* section are MEASUREMENTS of 2026-09-18 and are **not rewritten** — they keep the version they were taken
at, and a dated confirmation at 2.1.278 is appended to them once the live probe has run. ***Corrected
2026-09-21, later the same day: this clause also named `registry.rs`'s probe record, and on reading it that is
WRONG.*** *That row is 7c's **capability** provenance — `"capability": "dispatch"`, `verified_on: 2026-09-17`,
a `probe.recorded.source` naming the corpus pin's 101 recorded dispatch rows — and not a statement about the
CLI pin at all. Appending to it would change a runtime `const` under an amendment that permits the pin and
nothing else, and re-dating a `verified_on` is 7c's decision, not this stage's. It is left exactly as it
stands, and the correction is recorded here rather than the work quietly not done.* **(b)** A pin this crate
has not been dispatched at is a claim without evidence, so **`live_dispatch_probe` at 2.1.278 is OWED** — one
paid haiku dispatch, `#[ignore]` behind `[features] live`, run by the operator in the foreground as its own
ignore-reason prescribes; the precedent move (`cee6587`, 2.1.275 → 2.1.277) re-grounded the evidence the same
way. ***DISCHARGED 2026-09-21: the operator ran it and it PASSED at the new pin.*** *`OBSERVED_CLI_VERSION`
and `INSTRUMENT_VERSION` both `2.1.278`, `PROBE_RAN_CONFORMANCE=1`, `COMPLETED_ROWS=1`, `DISPATCH_ROWS=0`,
`TTL_STATE=no-write`, `apiKeySource="none"`, `RESULT_RAW_SHA256=d13c2d1a55d2f52a3a4851930e5b646778770d4ba01f21cb75c78e73f6aebfee`,
64,526 bytes of stdout retained verbatim. **RT-27's shape is UNCHANGED at the new release** — the retained raw
`system/init` carries `agents` and no `agent`, `probe` in the list, `model` the full haiku id and `tools`
`["StructuredOutput"]` — so the two records in (a) take a re-measured confirmation rather than a rewrite. Six
keys appeared beside them and none is read: `effective_config` picks named keys off a `serde_json::Value`.* **(c)** Checkpoint 7c's D17-11 anchored grep reads this constant,
so 7c's row goes **provisional** until it is re-run — the same disposition Slice 1 gave Step 6's row, and it is
recorded in PROGRESS.md §1 rather than absorbed. **(d)** §7d.1's *“one closed crate edited”* is disturbed and takes
its own dated note there.*

### 7d.16 The slices

Every slice lands in the record's voice with a body in the R2/V3 form (`s7b-ii` §7b-ii.13's precedent; `s7c` §7c.20):
what was built per module and rule; fixtures with provenance bytes/sha256; **RED quoted** — the failure text of each
named test before the code; deps added and `deny ok`; the five gates' results; which Checkpoint 7d lines already ran
green and which are still `[WILL EXIST]`-shaped by absence; provisional re-runs with the `PROGRESS.md` §1 mark in the
same commit; raised-not-settled; the ADR-0101 disclaimer; the `Co-Authored-By` trailer. Commit subjects take the form
**`CLI, slice N (<crate>): <claim in the record's voice> (<AC ids>)`**. Each slice compiles on the last; the order is
surface before behaviour (a stable snapshot and exit table first, so the denylist lint exists before any verb can grow
a flag), config and `init` before the loop (a `Ports` and a loadable temp repository are the loop's inputs), `doctor`
after the loop (`--rebuild` compares against what `run` writes; the `Entry` seam is shared), migrate last and alone
(it shares no code with `rk-cli` — AC-441; its dependency set and the `anyhow` set each change in exactly one commit
the `cargo metadata` lines watch red → green).

0. **Stage-open** (tracker commit; **the commit this rewrite lands in**): this file (the rewrite); the C-01 pass
   recorded in `PROGRESS.md` §6 and §5 flipped; `spikes/DECISIONS-2026-09-18.md` (D18-1…D18-18 — D18-17 carrying the four
   operator decisions D-01/D-02/D-04/D-14 with D-05/D-15/D-18 — RT-28…RT-59, O-12…O-16, R-25…R-36; RT-41…RT-59 and O-16
   are this file's routings and open item, added to the log's tables in this commit — RT-47…RT-59 by the hardening
   rounds, with RT-42's same-day correction); **ADR-0128 and ADR-0129 committed `proposed`** — each departs a Decision
   sentence of an accepted record on the operator's word of 2026-09-18, and under ADR-0092 an *accepted* record
   outranks the decisions log, so committing either `accepted` before the operator's dated confirmation line is
   written under D18-17 (the row that records the four C-01 operator decisions; the line is a `**Confirmed <date> (dated
   line, the operator's word).**` paragraph appended to D18-17, in DECISIONS-2026-09-17 D17-8's form, and the flip commit
   cites it as `6f88e37` cited D17-8's) would bind him to a shape he has not
   confirmed (ADR-0126's precedent, `s7c` §7c.20 item
   0); **a follow-up commit flips both to `accepted` with `date` unchanged**, and Checkpoint 7d's two `^status:
   accepted$` greps cannot pass before it *(done 2026-09-19 — the flip commit)*; the ADR README's count and its two `proposed` index rows, and
   `00-preamble.md`'s next-free-number sentence (`0128` → `0130`, dated) — **ADR-0107's and ADR-0009's `superseded_by`
   and the README's status cells for `0107` and `0009` follow in the flip commit** (§7d.16 item 6; the supersession-map
   rows for both are already on the tree, marked `proposed`): an accepted
   record is not marked superseded by a `proposed` one (ADR-0092 clause 2.1: *"Accepted ADRs govern every decision and
   change only by supersession"* — a proposed record has superseded nothing yet; ADR-0126 set no precedent for the
   link, since it superseded nothing); the register, SM and design dated amendments
   listed in §7d.2 (landed with the records); `s8`'s three `run` invocations and two `explain` lines re-spelt;
   `ci.yml`'s AC-529 scope comment; `PROGRESS.md` §1 row 7d → IN PROGRESS with the two BEFORE readings recorded
   (`BEFORE: no rk-cli yet`, `BEFORE: no redkiln-migrate yet`), §2's D18/RT/O rows and the external-anchor drift note
   (§7d.1); the three validators' triple; **"Checkpoint 7d is NOT run"**. **No root `Cargo.toml` row, no `deny.toml`
   row, no crate** — the stage-open commit is records and this file.
1. **Slice 1 — `rk-cli` surface + exit table**: `crates/rk-cli/{Cargo.toml, src/lib.rs (the library the tests link —
   `pub mod cli`, `pub mod exit`, `pub mod verbs`, `pub fn code_for`, `pub fn render_halt`), src/main.rs (a thin caller:
   `parse → verbs::dispatch → std::process::exit(code_for(&result))` — ONE `process::exit` site at this slice; the
   handler, the panic hook and the second site are Slice 2's, so **the block's two-site count is first satisfiable in
   Slice 2** and reads as `[WILL EXIST]`-shaped by absence in Slice 1's body), src/cli.rs (the tree,
   DENYLIST), src/exit.rs (`Exit`, `ALL`, `From<&StepError>` **and the `Halt` enum**, declared here so `pub fn
   code_for(&Result<Exit, Halt>) -> i32` and `pub fn render_halt(&Halt) -> String` compile at this slice with no
   `store` module — `src/store.rs`, which returns a `Halt`, `src/signal.rs` and `pub fn is_lock_halt`, which matches the
   kernel's `LOCK_HALTS` and needs Slice 2's `pub const` visibility edit, are Slice 2's: as first drafted this slice
   listed `pub mod store` and `pub mod signal` in `lib.rs` and could not have compiled without Slice 2's files and kernel
   edit, and a Slice 1 that took the kernel edit would re-open 7b/7b-ii, which §7d.15 assigns to Slices 2–4 only),
   src/verbs/mod.rs (eight stubs — one per §7d.3 subcommand — each exiting 3
   with "not built at slice 1")}`, `tests/it/{main,surface,exit}.rs`, `tests/cmd/*.trycmd`; the six root rows and the
   eight `deny.toml` rows; **Checkpoint 5 re-run whole** (§1 row provisional then discharged in the tracker commit). Tests
   (§7d.14's first twelve): `ac400_help_snapshot_every_verb`, `ac400_denylist_absent_from_parser_tree` (mutant: add
   `--resume` to `run`), `ac400_resume_refused_as_unknown_argument`, `ac402_no_skip_reconcile_flag_in_snapshot`,
   `ac441_no_migrate_verb_in_tree`, `doctor_fix_flag_absent_from_snapshot`, `adr0128_run_lane_value_enum_closed`,
   `adr0129_answer_grammar_positional_option_no_token_flag`, `ac438_exit_table_snapshot`, `ac438_exit_codes_distinct`,
   `ac438_step_error_mapping_is_exhaustive`, `ac403_no_blocking_stdin_read_in_rk_cli`. Subject: `CLI, slice 1
   (rk-cli): one router, one denylist, one closed exit table (AC-400, AC-186, AC-264, AC-402 snapshot half, AC-441
   router half, AC-438 table half, AC-403 lint half; ADR-0128/0129 grammars)`.
2. **Slice 2 — config root, adapters, `init`, signal, the halt**: `src/env.rs` (`Env::capture()`, the one read),
   `src/config.rs` (`ConfigFile`), `src/adapters.rs` (`AdapterSet`), `src/repo.rs` (H-3 discovery), `src/signal.rs`,
   `src/store.rs` (the acquiring function returning `Err(Halt)`), `src/clock.rs` (the one clock read), `src/verbs/init.rs`
   (the scaffold; `INIT_UNREAD_WINDOW_RUNS`; `CreatedNames`); `lib.rs` gains `pub mod store`, `pub mod signal`, `pub fn
   is_lock_halt`, and `main.rs` its handler and panic hook (the second `process::exit` site — the block's count is
   satisfiable from here); **`rk-kernel/src/config.rs` + `RunInputs::with_config`, and `pub const LOCK_HALTS`**
   (Checkpoints 7b/7b-ii re-run whole); fixtures `config/*.toml` (`missing-store.toml` included), `init/config.toml.expected`,
   **the root `.gitattributes`** with its `crates/rk-cli/tests/fixtures/init/config.toml.expected -text` and
   `crates/rk-cli/tests/it/snapshots/*.snap -text` rows (§7d.13; the `legacy-mini/** -text` row is appended in Slice 5 —
   a byte-pinned fixture is protected from the slice that lands it, on this `core.autocrlf=true` box), the `lint-controls/`
   directory seeded; the `-p rk-kernel` `adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs`. Tests:
   **twelve** — the ones that need no verb beyond `init`: `ac486_misconfigured_adapter_refuses_before_spawn_or_store_write`
   (**its in-process arm** — `AdapterSet::resolve` over `unknown-adapter.toml` and `missing-store.toml` returns
   `Err(Refusal)` naming `rk-adapter-linear` / the absent key, zero marker files, no store file; the spawned-`run` arm of
   §7d.5 lands in Slice 3 with the verb),
   `ac486_init_scaffold_resolves_four_reals`, `ac426_unread_window_runs_required_no_default` (the `ConfigFile` load arm),
   `ac474_config_surface_snapshot_has_no_dual_write_key`, `ac437_init_twice_identical_tree_and_identity_rows`,
   `ac437_init_scaffold_bytes_pinned`, `ac531_init_created_names_match_regex`, `ac531_preexisting_rk_name_reported_not_overwritten`,
   `ac412_init_installs_zero_hooks`, `ac422_config_reads_the_one_denylist`,
   `signal_handler_fires_cancel_token`, `adr0118_acquire_returns_halt_on_poisoned_lock` (the **poisoned-lock fixture's
   in-process twin** is this slice's — C-01 RB-M-01). The six that spawn or drive `status` / `run` — `common_dir_…`,
   `ac485_…`, `ac413_…`, `signal_outside_dispatch_…`, `adr0118_poisoned_store_lock_halts_run_exit_70` and
   `adr0083_config_constructed_once_at_root_and_threaded` (it reads *the `rk_kernel::Config` value the run carries* and
   counts `with_config` — no run exists before Slice 3) — are Slice 3's,
   because in Slice 2 the verbs are Slice 1's stubs and each would be red for the stub's `3`, not for the absent
   behaviour (golden constraint 12: *the right reason*). Subject: `CLI, slice 2 (rk-cli, rk-kernel config): the root
   reads its environment once, resolves four adapters once, scaffolds init eagerly, fires the token, and refuses a
   poisoned lock with a Halt (AC-486, AC-437, AC-531, AC-412, AC-422 one-list half, AC-426 key half, AC-474 config half;
   ADR-0083, ADR-0099 d2, ADR-0118 c5)`.
3. **Slice 3 — the loop: `run`, `answer`, `status`, `explain`, `catalog search`, `lint`**: `src/verbs/{run,answer,
   status,explain,catalog,lint}.rs`, `tests/it/support.rs` (the scripted-harness helpers, the `EntryReconciler` recorder, the counting store fake);
   **`rk-kernel`'s `catalog::Activity.keywords` + `catalog::search`, `workflow/question.rs`, `workflow/gate.rs`
   (`AnswerMeta`, the registry arguments) and `replay::resume`** — ***corrected 2026-09-20, settling residual R-59: this enumerates FOUR and the slice landed FIVE.*** *The fifth is **mechanism B**, taken on the operator's decision at cycle 10 (`76af103`) and raised rather than absorbed. **The compounding, stated so it is not re-counted silently:** §7d.1 and §7d.15 each enumerate the stage's kernel edits as **EIGHT** — Slice 2 two, this slice four, Slice 4 two — so with this slice's fifth the tree will read **NINE against eight** at Slice 4's close. Item 4's *"the third and last kernel disturbance"* is a separate count and is CORRECT as written: it counts disturbance OCCASIONS (Slices 2, 3, 4), each one whole re-run of Checkpoints 7b and 7b-ii, and Slice 4 is indeed the last* — (Checkpoints 7b/7b-ii re-run whole); fixtures
   `pack-min/` (the three variants and the five items), the `explain/*/` run-record directories and `exit/*/*.jsonl` (+ the two AC-465
   allowlist rows), `exit/<name>/`, `facets/*.toml`; the `-p rk-kernel` `ac436_…` pair, `adr0129_question_registry_…`,
   `adr0063_option_with_attaches_…`, `adr0129_gate_answered_carries_answer_meta_kernel_side`,
   `replay_resume_seeds_fold_from_journal_and_steps_gate_open`. Tests: **thirty-three** — §7d.14's `-p rk-cli` bullets
   from `ac402_status_and_explain_do_not_reconcile` through `signal_during_dispatch_is_cancelled_by_signal_exit_13`
   (twenty-seven; with `ac264_run_after_applied_answer_resumes_past_gate` among them; `ac402_reconcile_called_on_run_answer_doctor`
   drives `doctor` in-process and is Slice 4's, where `verbs/doctor.rs` exists — in this slice it would be red for the
   stub's `3`; `ac438_table_driven_exit_codes_over_fixtures` lands here over the `exit/<name>/` fixtures and grows its
   doctor-fixture arm in Slice 4, where `fixtures/doctor/` exists) plus the six moved from Slice 2 (`common_dir_…`, `ac485_…`, `ac413_…`, `signal_outside_dispatch_…`,
   `adr0118_poisoned_store_lock_halts_run_exit_70`, `adr0083_config_constructed_once_at_root_and_threaded`) and
   `ac486_…`'s spawned-`run` arm; `run` also writes the run record `explain` reads (`replay::write_run`, §7d.7) — **the C-01 additions folded in**: `catalog search` and its positive
   control, `lint`, the AC-435 flag, `Config` threading, the AC-410 positive control, AC-248/AC-257, the lane tests, and
   the resumed-run test AC-264/AC-186 needed. Subject: `CLI, slice 3 (rk-cli, rk-kernel keywords + question registry): run is a read, a decision
   and an exit; the gate is a trailer and exit 0; answer resolves its own token; explain prints the record (AC-401,
   AC-402 three paths, AC-403, AC-404 exit half, AC-405–AC-407, AC-410, AC-248, AC-257, AC-263, AC-435 machine half,
   AC-430–AC-434, AC-436, AC-438 fixtures; ADR-0128 lane, ADR-0129 registry)`.
4. **Slice 4 — `doctor`**: `src/doctor/{mod,rules,rebuild,probe}.rs`, `src/verbs/doctor.rs`, `src/no_dispatch.rs`
   (the `NoDispatchHarness` the root hands the `doctor` path); `src/doctor/roles.rs` (the role→card resolver, §7d.8); **`src/adapters.rs`'s store-open path** *(added 2026-09-20, Step 7d Slice 4 cycle 2: this list omitted it and was short by one. §7d.8 requires `Ctx.store` to carry “the store handle **the root opened**” and names “`adapters.rs`'s open path, the module the block's `doctor` filter does not admit” for the rebuild's re-open — while Checkpoint 7d's D-10 ban forbids `SqliteStore::open` on any non-comment line under a module path matching `doctor`, which admits `src/doctor/*.rs` AND `src/verbs/doctor.rs`. Measured 2026-09-20 before the edit: all six `SqliteStore::open` call sites in `rk-cli` were under `src/verbs/`, none in a module the filter excludes — so `doctor` had no open path it was permitted to use and the two sentences could not both be satisfied. This item is a summary of §7d.8 and loses to it, the same precedence that settled the fixture count; the edit is execution of what §7d.8 already names, not a new decision. Also measured: the Slice-1 `doctor` stub was an **inline `fn doctor()` in `src/verbs/mod.rs`** calling a shared `not_built` helper — there was no `src/verbs/doctor.rs` to replace, so that path is CREATED here and both the stub and `not_built`, whose last caller it was, are gone)*; **`rk-kernel`'s `catalog::load_diagnostic` and
   `Catalog::from_sources`'s AC-409 refusal** (Checkpoints 7b/7b-ii re-run whole — the third and last kernel
   disturbance); fixtures `doctor/<rule-id>/` (***thirty-eight — corrected 2026-09-20; this read "thirty-two" and was STALE.*** *D18-19 allocated the six `s7b-ii` gate rows into the table on 2026-09-19, moving it 32 → 38, and this item was not updated with it. §7d.8 says **"Thirty-eight rule ids … each with a rejecting fixture"** and carries the changed number's own history in the same sentence; §7d.13 says **"one rejecting directory per id in §7d.8's table (thirty-eight)"**. Counted independently off the table's first column, which is Halt 7's own instruction: `grep -oE 'RK-D-[A-Z0-9-]+' | sort -u` → 39 tokens, less `RK-D-DANGLING-`, a prose prefix of the compound cell `RK-D-DANGLING-{ROLE,SLOT,ATTACH}` → **38**. This item is a summary of the other two and loses to both. AC-415's red is a set equality in BOTH directions, so building thirty-two leaves it red*) and `doctor/clean/`, `unread/{runs-49,runs-51}.jsonl` and
   `doctor/*/events.jsonl` (+ the two AC-465 allowlist rows), the synthetic trailer repository builder for `ac427_…`;
   the `-p rk-kernel` `catalog_load_diagnostic_returns_every_load_error_typed` and
   `ac409_gate_step_with_outputs_rejected_at_load`. Tests: **thirty-eight** — §7d.14's thirty-seven from
   `ac415_every_rule_id_has_a_rejecting_fixture` through `ac438_doctor_error_rule_exits_config_error` (the compound
   bullet's twenty-six names among them, `ac370_stray_store_under_linked_worktree_is_an_error` and the six D18-19 gate
   rows included) plus
   `ac402_reconcile_called_on_run_answer_doctor` (moved from Slice 3) — **the rule-id floor**
   `doctor_rule_table_covers_every_register_named_row` is this slice's (RA-M-01). Subject: `CLI, slice 4 (rk-cli
   doctor): a rule table as data, thirty-eight ids each with a
   rejecting fixture, zero dispatches, one version probe, rebuild git → store (AC-409, AC-412–AC-427, AC-429, AC-486
   doctor half, AC-488 doctor half, AC-397, the s7c doctor halves, the six s7b-ii gate rows AC-290/306/310/314/455/530 —
   D18-19; SM-121)`.
5. **Slice 5 — `redkiln-migrate` v0**: `crates/redkiln-migrate/{Cargo.toml, src/main.rs, src/legacy/{frontmatter,
   ledger,review,config_yaml}.rs, src/lift.rs, src/report.rs, src/run_contract.rs, schema/report.schema.json}`,
   `tests/it/*`, fixtures `legacy-mini/` + `PROVENANCE.md`, the `crates/redkiln-migrate/tests/fixtures/legacy-mini/**
   -text` row appended to the root `.gitattributes` (Slice 2's file, §7d.13), the migrator's controls added to
   `lint-controls/`; `ci.yml`'s AC-442 lint and AC-441 metadata
   line. Tests: §7d.14's twenty-three run `-p redkiln-migrate` names plus the authored, unrun `ac463_…` (twenty-four
   bullets). Subject:
   `CLI, slice 5 (redkiln-migrate): v0 exists — refuses a dirty target before any write, writes into its own
   worktree, commits once with its report, and is a no-op by tree digest (AC-441 build half, AC-442, AC-447–AC-450,
   AC-466, AC-468 schema half, AC-470, AC-474 migrator half, AC-475 tripwire, AC-522; AC-443/AC-445
   PROVISIONAL; AC-477/AC-478 ARMED)`.
6. **Close — two commits, not one** (`00-preamble.md`: *"A stage closes in two commits"*): **the ADR-0128/ADR-0129
   flip commit precedes both** (or the gap is recorded and the block cannot pass — its two `^status: accepted$` lines
   say so); then **(a) the close commit** — `Step 7d close: the CLI exists, the gate is a trailer and an exit, doctor
   is a rule table, migrate v0 ran` — carrying the block's recorded output from a Git Bash on this box (no live line;
   the gate check), the end marker, the statement count re-counted and written into §7d.17's prose, the two temp
   repositories' digests as printed, and the three validators' triple; then **(b) the tracker commit** — `Tracker:
   Checkpoint 7d passed at <hash of (a)> — the CLI exists, the gate is a trailer and an exit, doctor is a rule table,
   migrate v0 ran; row 7e UNBLOCKED` — with `PROGRESS.md` §1's Result cell, §6's statement count + end marker +
   validator triple, every provisional row (5, 7b, 7b-ii) discharged with its re-run's hash, and a new memory
   note superseding `runbook-position-2026-09-18` on position. **ADR-0101 clause 5 bounds what (a) and (b) may say.**
### 7d.17 Checkpoint 7d

**Counted mechanically off the block below on 2026-09-18, and RE-COUNTED 2026-09-21 after the two close-time
amendments (§7d.13's `[[bin]]` permission and the `.lock()` ban's third exclusion), as the next sentence requires** — comments and
blanks dropped, backslash continuations joined; **re-count and re-write every number in this paragraph after any
repair to this block** (Halt 7: count it off the script, not off this sentence). **The rule that produces the number is
`s7c` §7c.21's and is restated so it reproduces:** a *statement that can stop the block* is one carrying its own `exit
1`, **or a call to one of the three stopping helpers, each of which carries its `exit 1` inside its definition**; a
call inside a `for` body is one statement however many times the loop runs. **344 non-comment statements, of
which 189 can stop the block and 155 cannot** *(dated 2026-09-27, s8d build, package R3FIX-8d: **345 / 190 / 155**, re-counted off the block by this paragraph's rule after the ADR-0100 d1 ban took its lockfile-name exclusion and ONE admission line, a statement carrying its own `exit 1`; the ban's old line is kept as a comment, so `rk_absent` stays × 19 and the helper calls 40; the 149 statements with their own `exit 1` are 150, and the admission lines before excluded bans are five; `s8d` §8d.11)* *(343 / 188 / 155 until 2026-09-21; the close-time amendment added ONE
statement carrying its own `exit 1` — the `rk-fixture-child-cli` name line — and the other two edits were within-line,
so only this bucket moved).* The 189 are **149** statements carrying their own `exit 1` outside the
helper definitions (the `uname` `case` guard, the two shell-guard `[ -z "$bad" ] || { …; exit 1; }` lines, the
one-line `for d in $SRC_SCOPE` guard, the `&& { …; exit 1; }` manifest and snapshot bans, the `git`/`cp`/`mv` prelude
lines of the two temp repositories, the two `eol` guards — before the `cmp` and before the snapshot's row greps — the two `sums` guards, the hooks-directory `test
-d`, the two `git status` `rc` guards, the `RK-D-ADAPTER` `row=` / `ok` pair, the refused run's `refs=` / `test ! -d …/worktrees` pair (the third hardening round's two lines), the `.redkiln` presence compare, the two
admission lines before the excluded-module bans and the two before the `template_sig` and clock bans, the
`process::exit(` count and its `main.rs` check, the `rk-fixture-child-cli` name line added 2026-09-21, and the three
python closers `' || { …; exit 1; }` among them) plus
**40** helper calls (`rk_test` × 3 — all three in loop bodies, none direct — a call inside a `for` body is one statement
however many names the loop runs; `rk_absent` × 19; `rk_control` × 18). The 155 that cannot are not unguarded
assertions and the decomposition reproduces: **2** BEFORE readings (PIA-035 — prints for comparison, not assertions);
**25** helper-definition lines (`rk_test() {` … `}` six, `rk_absent() {` … `}` fourteen, `rk_control() {` … `}` five;
the six `exit 1`s inside them are `rk_test`'s two, `rk_absent`'s three and `rk_control`'s one, and each runs only when
its helper is called); **32** captures whose guard is the statement beside or below them — the two `bad=` shell-guard
sweeps, `had_redkiln=` and `now_redkiln=` (compared by the line after the second), `SRC_SCOPE=`, `LC=`, `pin=`,
`live=`, the two `out=$(cargo test …)` captures, `m=`, the `INIT_UNREAD_WINDOW_RUNS` `c=`, the `process::exit` `raw=` /
`hits=`, the `gate::answer` `raw=` / `c=`, the ADR-0101 `raw=`, `meta=`, `dig1=`, `por2=`, `dig2=`, the seven
`out=$(… redkiln …)` / `out=$(./target/debug/redkiln-migrate …)` captures, `FLOOR=`, `refs=`, and the dual-write
`raw=` / `hits=` (every capture that carries its own `exit 1` on the same line — `TMP1=`, `TMP2=`, `snap=`, `d=`, `n=`,
`por1=`, `files1=`, `files2=`, `sums1=`, `sums2=`, both `eol=`, `row=`, `head2=`, the refused run's `refs=` and the `c=` counts with a trailing test — is
among the 148, not here; the dry run's `refs=` over `rk-migrate/*` is guarded by the two lines below it and stays here); **22** `for … ; do` / `done` lines of the eleven multi-line loop constructs; **66** lines of
the three embedded python bodies (their openers and inner lines — the `COMPANION` set is the line added by the second
hardening round, the `by_kind=` / `assert by_kind` pair the two added by the third), each guarded by the closing `' || { …; exit 1; }` line counted among the 148; **2** clippy lines,
`cargo clippy … ; rc=$?; echo "exit=$rc"`, each guarded by the line below; **5** informational `echo`es after a passed
group (`init is idempotent…`, `resume is refused…`, `run on an init-only…`, `explain, status and catalog search…`,
`migrate v0 dry run…`); and **1** end-marker `echo`.
2 + 25 + 32 + 22 + 66 + 2 + 5 + 1 = 155; 344 − 155 = 189.

**No line in this block is `[WILL EXIST]`, and the absence is deliberate.** Every binary the block runs is built by the
block's first gate from this stage's own crates; `halts.md`'s carve-out is for a path a later stage produces, not for a
path this stage owes. **Nothing in `check-checkpoints.py`'s allowlist is added for this file.**

**Why this block needs no shell guard against a Claude-spawned Git Bash, and what it guards instead.** `s7c` §7c.21's
`CLAUDECODE` / `CLAUDE_CODE_*` guard existed because the live probe captured the shell's environment into a recorded
map that `Config::new` then linted; this block has **no live line** — its `doctor` lines run under the real
`ClaudeHarness` constructor **never** (D18-7), and its one child is `claude --version`. What it does guard: **a shell
carrying a name on the committed denylist**, because `doctor`'s `RK-D-ENV-DENYLIST` row reads the captured map and
would fire correctly on `ANTHROPIC_FOUNDRY_API_KEY` (persisted at user scope on this box), turning the temp repository's
expected `0` into a `3` for a reason unrelated to the binary — the guard is a presence sweep over the shell's variable NAMES, ASCII-case-insensitive as the denylist itself compares,
and names `env -u <name> bash`; beside it a second sweep refuses `TRYCMD`, `INSTA_UPDATE`, `INSTA_FORCE_PASS`,
`INSTA_FORCE_UPDATE` and `INSTA_FORCE_UPDATE_SNAPSHOTS` (the last two are insta's deprecated aliases of
`INSTA_UPDATE=force`, still read at the pinned 1.48.0 [measured 2026-09-18, insta `src/env.rs`]), under
any of which the two snapshot tests regenerate or force-pass and pin nothing;
and **Halt 2**: `claude --version` must equal the pinned instrument, because `RK-D-CLI-VERSION` reads the installed
binary (D-11) and a moved CLI would fire it — a move is recorded, never worked around.

**The two temporary repositories** (D18-9). `TMP1` is `mktemp -d` + `git init -q -b main` + one empty commit (the root
commit `SqliteStore::open`'s identity row needs); `init` runs there twice, then `doctor` twice (the scaffold at `0`;
`unknown-adapter.toml` copied over it at `3`, then restored), then the two refused `run`s (`--resume` at `2`; `some-item`
at `3`, followed by a check that the refusal created no `rk-run/*` branch and no run-owned worktree — step (8) never ran),
then `explain` at `5`, `status` at `0` and `catalog search` at `0` — the gate check's six verbs.
`TMP2` is the same prelude plus a copy of `legacy-mini/` committed as its second commit; the migrator's `--dry-run`
runs there, its stderr captured beside it as `$TMP2.err` (never a fixed `/tmp` path a second run would truncate — Halt 8
(h)), and the block asserts `HEAD` and the porcelain listing are unchanged by it. **Neither is the product
repository, whose `.redkiln/` is 7e's**; both, and the `.err` file, are removed at the end of the block on success and
left in place on a FAIL for inspection.

**`doctor` on the init-only repository exits `0`, and this is why** (the table, H-1; the rows, §7d.8): the scaffold names
the four compiled-in reals (`RK-D-ADAPTER` ok), `[packs] enabled = []` is `init`'s own state and not a defect (E6: 7e
fills it; the *"no process pack enabled"* refusal is `run`'s, D18-8, not a `doctor` row — a repository on which only
`init` has run *"must be loadable"*), so every pack- and facet-dependent row reports `ok` with the detail *no enabled
pack*; the store was just created (`RK-D-STORE-ID` ok); no hook, no orphan, a local filesystem; the guards above made
`RK-D-ENV-DENYLIST` and `RK-D-CLI-VERSION` pass for the right reason; and `RK-D-UNREAD` sees zero runs. The `3` branch
is exercised in the same block by copying `unknown-adapter.toml` over the scaffold and running `doctor` again — a
positive control for the row and for AC-438's doctor clause on the real binary.

**Every `rk_absent` ban below carries a positive control**, checked through the ban's own search and comment filter by
`rk_control` with **the ban's exact pattern** (`s7c` §7c.21's rule — nineteen `rk_absent` lines, eighteen `rk_control`
lines: the two `std::env::var` bans share `adr0083_env_var_control.rs`, the two clock bans share
`adr0083_clock_control.rs`, and `ac405_gate_answer_control.rs` controls the `gate::answer(` **count**, not an absence);
**one absence sweep has no control by design** — the ADR-0101 phrase sweep scans `tests/` too, so its control would be
its own hit, and it is written in `s7c` §7c.21's raw `grep -rniE` form instead of through `rk_absent`. The scope
directories are `test -d`'d first. Three bans are module-path **filtered** (`rk_absent`'s fourth argument — the two
`doctor`-module bans to `doctor`, the kernel `keywords` ban to `select|predicate`), and the helper asserts admission
before filtering; four carry a module **exclusion** (the fifth argument) — `std::env::var` excludes `rk-cli`'s `env`
module, `SystemTime::now|Instant::now|Utc::now` its `clock` module, `.lock()` its `store` module **and the two stdio
handles** (`io::stdout().lock()` / `io::stderr().lock()` are not Mutexes — golden constraint 9), `template_sig|rendered_sig`
the migrator's `legacy` module (`legacy.rs` or `legacy/`, layout-neutral) *(dated 2026-09-27, s8d build, package R3FIX-8d: and a fifth — the ADR-0100 d1 `yaml` ban excludes the `"pnpm-lock.yaml"` file-name literal in `rk-cli`'s `change_deriver` module only, a lockfile named, not read; `s8d` §8d.11)* — and each exclusion is preceded by its own
admission line (`n=$(find
… | grep -cE …)`), so an exclusion that names no existing module is a FAIL, not a silent pass; the `yaml` and
`ANTHROPIC_API_KEY` bans are simply scoped to `crates/rk-cli/src`. A grep a comment could satisfy is never the whole
assertion.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# ---- THIS BOX FIRST (00-preamble: every block runs on this Windows box): the in-process signal tests kill the fixture child
# ---- through a Job Object (rk-containment's Windows arm), and the OS-delivery arms — a console CTRL_BREAK on Windows, SIGTERM on
# ---- POSIX — are ARMED (§7d.10, §7d.18): neither is a line in this block.
case "$(uname -s)" in MINGW*|MSYS*) : ;; *) echo "FAIL: this block runs on the Windows box; the POSIX arm is ARMED to CI and is not this block's evidence"; exit 1;; esac
# ---- THE SHELL, SECOND — but NOT s7c's CLAUDECODE guard: this block has no live dispatch line (§7d.17's prose), so a
# ---- Claude-spawned Git Bash is fine. What is refused is a shell carrying a committed-denylist name, because the temp
# ---- repository's `doctor` line expects 0 and RK-D-ENV-DENYLIST would correctly turn it into 3 (run under
# ---- `env -u ANTHROPIC_FOUNDRY_API_KEY bash`; never set_var).
# ---- presence, not value (a set-but-empty variable still fires the row), and ASCII-case-insensitive, because the
# ---- denylist compares that way (env_denylist.rs: "Names compare ASCII-case-insensitively").
bad=$(env | cut -d= -f1 | grep -iE '^(ANTHROPIC_API_KEY|ANTHROPIC_AUTH_TOKEN|ANTHROPIC_BASE_URL|ANTHROPIC_FOUNDRY_API_KEY|GITHUB_TOKEN|GH_TOKEN|GH_ENTERPRISE_TOKEN|CLAUDE_CODE_USE_.*)$')
[ -z "$bad" ] || { echo "FAIL: a committed-denylist name is set in this shell (rk-adapter-claude/src/env_denylist.rs) and doctor's RK-D-ENV-DENYLIST row would fire for a reason unrelated to the binary; run the block under 'env -u <name> bash' for each of:"; printf '%s\n' "$bad"; exit 1; }
# ---- the two snapshot tools' override variables: under any of them a snapshot test regenerates or force-passes and pins nothing.
bad=$(env | cut -d= -f1 | grep -E '^(TRYCMD|INSTA_UPDATE|INSTA_FORCE_PASS|INSTA_FORCE_UPDATE|INSTA_FORCE_UPDATE_SNAPSHOTS)$')
[ -z "$bad" ] || { echo "FAIL: a snapshot-override variable is set — ac400_help_snapshot_every_verb and ac438_exit_table_snapshot would regenerate or force-pass (AC-400, AC-438); unset:"; printf '%s\n' "$bad"; exit 1; }

# ---- the product repository's .redkiln/ PRESENCE, read now and compared at the end (O-15: 7e creates the product scaffold,
# ---- so presence is compared across the block, never asserted absent — D18-9).
had_redkiln=no; if test -d .redkiln; then had_redkiln=yes; fi
# ---- BEFORE readings (PIA-035): each PRINTS on the untouched tree and is silent after the step. They are
# ---- READINGS FOR COMPARISON, NOT ASSERTIONS — nothing below consumes them; recorded in the stage-open commit.
test ! -d crates/rk-cli && echo "BEFORE: no rk-cli yet"
test ! -d crates/redkiln-migrate && echo "BEFORE: no redkiln-migrate yet"

# ---- Three stopping helpers, copied VERBATIM from s7c §7c.21 (the rules they exist for are stated there: never a
# ---- status-bearing command left of a pipe; the search status asserted before the comment filter; a control checked
# ---- through the ban's own pipeline). Only SRC_SCOPE and LC differ: the two new crates' src/, and rk-cli's controls.
rk_test() {      # <pkg> <test-name> <what> [<extra cargo flags>]
  out=$(cargo test -p "$1" ${4:-} -- "$2" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { printf '%s\n' "$out" | tail -20; echo "FAIL: cargo test -p $1 ${4:-} -- $2 exited $rc"; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b$2\b.* \.\.\. ok$" \
    || { echo "FAIL: no test named $2 ran and passed in $1 — $3"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  echo "PASS: $2 — $3"
}
SRC_SCOPE="crates/rk-cli/src crates/redkiln-migrate/src"
rk_absent() {    # <pattern> <what> [<scope dirs, default SRC_SCOPE>] [<module-path filter regex>] [<exclusion regex>]
  scope=${3:-$SRC_SCOPE}
  raw=$(grep -rnE "$1" $scope --include=*.rs); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: the search for '$1' could not run (grep rc=$rc; 2 = a scope path is missing) — a 0 here would be a fake 0 (Halt 8): $2"; exit 1; }
  if [ -n "${4:-}" ]; then
    # ADMISSION before filtering: a module-path filter that admits no file makes the ban vacuous (a `ticks.rs`
    # digit scanner would never be searched). At least one .rs file under the scope must match the filter.
    n=$(find $scope -name '*.rs' | grep -cE "/([^/]*/)?($4)(\.rs|/)" | tr -d '[:space:]')
    [ "${n:-0}" -ge 1 ] || { echo "FAIL: no module matching '$4' exists under $scope — the filtered ban for '$1' would be vacuous: $2"; exit 1; }
    raw=$(printf '%s\n' "$raw" | grep -E "^[^:]*/([^:]*/)?($4)(\.rs|/)")
  fi
  hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
  [ -z "${5:-}" ] || hits=$(printf '%s\n' "$hits" | grep -vE "$5")
  [ -z "$hits" ] || { echo "FAIL: $2"; printf '%s\n' "$hits"; exit 1; }
  echo "PASS: no non-comment hit — $2"
}
LC=crates/rk-cli/tests/fixtures/lint-controls
rk_control() {   # <pattern — the SAME one the ban uses> <control file name under $LC>
  c=$(grep -rnE "$1" "$LC" --include="$2" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
  [ -n "$c" ] || { echo "FAIL: control $2 carries no NON-COMMENT line matching '$1' — the ban is vacuous (or the control file is absent)"; exit 1; }
  echo "PASS: control $2 carries the banned token"
}

# ---- Halt 2, BEFORE any doctor line: RK-D-CLI-VERSION reads the INSTALLED binary (D-11), so the temp repository's
# ---- expected 0 depends on `claude --version` equalling the pin. A move is recorded in PROGRESS.md §1/§3/§6 and the
# ---- instrument file re-teed; it is never worked around here.
test -s spikes/results/instrument-checks/RUNBOOK-instrument.txt || { echo "FAIL: RUNBOOK-instrument.txt absent or EMPTY (Halt 8 (h))"; exit 1; }
pin=$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)
printf '%s' "$pin" | grep -qE '^2\.1\.[0-9]+ \(Claude Code\)$' || { echo "FAIL: the instrument file does not read '2.1.N (Claude Code)': '$pin'"; exit 1; }
live=$(claude --version 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: claude --version exited $rc"; exit 1; }
[ "$live" = "$pin" ] || { echo "FAIL: Halt 2 — claude --version is '$live' and the pinned instrument is '$pin'. Record the move and re-tee; do not continue silently"; exit 1; }
grep -qE "^[[:space:]]*pub const TESTED_CLI_VERSION:[[:space:]]*&str[[:space:]]*=[[:space:]]*\"${pin%% *}\"" crates/rk-adapter-claude/src/config.rs || { echo "FAIL: TESTED_CLI_VERSION in rk-adapter-claude/src/config.rs is not \"${pin%% *}\" — doctor's RK-D-CLI-VERSION compares against it (§7d.8)"; exit 1; }

# ---- the five CI gates, each guarded, workspace-scoped (halts.md Halt 1: `cargo build --workspace`, never -p).
# ---- `deny` bites on contact: clap, ctrlc, toml, trycmd, assert_cmd and the eight tracker/web bans move Cargo.lock.
cargo build --workspace || { echo "FAIL: cargo build --workspace (Halt 1's instrument from Step 5 on) — nothing below is meaningful (GC3)"; exit 1; }
cargo fmt --all --check || { echo "FAIL: cargo fmt --all --check"; exit 1; }
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red — AC-001-AC-004's instrument IS this invocation gated on its exit code; nursery WARNINGS are permitted (ADR-0102)"; exit 1; }   # → exit=0
cargo clippy -p rk-cli -p redkiln-migrate --all-targets --features rk-cli/corpus,redkiln-migrate/corpus; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red under the corpus features — ac463's corpus-bound runner is unlinted by the workspace line"; exit 1; }   # → exit=0
out=$(cargo test --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }   # → a FLOOR ONLY
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses (the eight AC-405 tracker/web rows are among the bans — D-13, RT-36)"; exit 1; }
# The crate-wide run per NEW crate, asserted separately: every other line below is FILTERED, so a unit test can fail
# under a name no filter names while every guarded line stays green.
for p in rk-cli redkiln-migrate; do
  out=$(cargo test -p "$p" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p $p exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: $p compiled but ran 0 tests"; exit 1; }
done
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — the [[bin]] is not named redkiln (Halt 8)"; exit 1; }
test -x ./target/debug/redkiln-migrate || { echo "FAIL: ./target/debug/redkiln-migrate absent after a green build — v0 does not exist (PIA-002)"; exit 1; }

# ---- the two records this stage committed at stage-open, flipped to accepted before the close (§7d.16 item 6): the
# ---- block runs at the close, so `proposed` is a FAIL here by design — the close is blocked until the operator's
# ---- dated confirmation line under D18-17 lands and the flip commit follows (carrying ADR-0107's and ADR-0009's superseded_by links).
ls references/adr/0128-*.md >/dev/null 2>&1 || { echo "FAIL: ADR-0128 (the one run grammar) is not on the tree"; exit 1; }
grep -q '^status: accepted$' references/adr/0128-*.md || { echo "FAIL: ADR-0128 is not at status: accepted — it was committed proposed at stage-open pending the operator's dated confirmation (§7d.16 item 0); the flip commit has not landed and the close is blocked"; exit 1; }
ls references/adr/0129-*.md >/dev/null 2>&1 || { echo "FAIL: ADR-0129 (the answer grammar and the question registry) is not on the tree"; exit 1; }
grep -q '^status: accepted$' references/adr/0129-*.md || { echo "FAIL: ADR-0129 is not at status: accepted — the flip commit has not landed and the close is blocked"; exit 1; }
grep -qE '^superseded_by: \[.*"0128".*\]' references/adr/0107-*.md || { echo "FAIL: ADR-0107's superseded_by does not name 0128 (a supersession in part is still a supersession) — the link lands in the flip commit with the status, never before it (§7d.16 item 0)"; exit 1; }
grep -qE '^superseded_by: \[.*"0129".*\]' references/adr/0009-*.md || { echo "FAIL: ADR-0009's superseded_by does not name 0129"; exit 1; }
grep -q 'C-01 DISCHARGED' runbook/PROGRESS.md || { echo "FAIL: PROGRESS.md carries no C-01 DISCHARGED row — the pass this file's §7d.2 reproduces is unrecorded"; exit 1; }

# ---- manifests (ADR-0102 d2, ADR-0083): workspace-inherited keys, EXACTLY one integration binary, [lints] last, corpus.
for c in rk-cli redkiln-migrate; do
  m=crates/$c/Cargo.toml
  test -f "$m" || { echo "FAIL: $m absent"; exit 1; }
  for k in edition rust-version license publish; do
    grep -qE "^$k(\.workspace[[:space:]]*=[[:space:]]*true|[[:space:]]*=[[:space:]]*\{[[:space:]]*workspace[[:space:]]*=[[:space:]]*true)" "$m" || { echo "FAIL: $m does not inherit $k from the workspace"; exit 1; }
  done
  n=$(grep -cE '^\[\[test\]\]' "$m" | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: $m declares ${n:-0} [[test]] targets, not one (ADR-0083: one integration binary)"; exit 1; }
  grep -qE '^name[[:space:]]*=[[:space:]]*"it"' "$m" || { echo "FAIL: $m declares no [[test]] name = \"it\""; exit 1; }
  grep -qE '^path[[:space:]]*=[[:space:]]*"tests/it/main.rs"' "$m" || { echo "FAIL: $m's integration binary is not tests/it/main.rs"; exit 1; }
  test -f crates/$c/tests/it/main.rs || { echo "FAIL: crates/$c/tests/it/main.rs absent"; exit 1; }
  grep -qE '^\[lints\]' "$m" || { echo "FAIL: $m has no [lints] table"; exit 1; }
  tail -n 2 "$m" | grep -qE '^workspace[[:space:]]*=[[:space:]]*true' || { echo "FAIL: [lints] workspace = true is not the LAST table in $m (ADR-0102 d2)"; exit 1; }
  grep -qE '^corpus[[:space:]]*=[[:space:]]*\[\]' "$m" || { echo "FAIL: $m has no [features] corpus = [] (AC-464/PIA-002)"; exit 1; }
  n=$(grep -cE '^\[\[bin\]\]' "$m" | tr -d '[:space:]'); [ "${n:-0}" -eq "$(test "$c" = rk-cli && echo 3 || echo 1)" ] || { echo "FAIL: $m declares ${n:-0} [[bin]] targets — rk-cli carries THREE (redkiln; the rk-fixture-child-cli target Slice 4 cycle 7 added; the rk-fixture-toolchain target 5e67ac0 added) and redkiln-migrate ONE (AMENDED 2026-09-21 and 2026-10-01, s7d §7d.13)"; exit 1; }
  grep -qE '^anyhow(\.workspace)?[[:space:]]*=' "$m" || { echo "FAIL: $m does not depend on anyhow — ADR-0102 d6 confines it to exactly these two crates, and ci.yml's rule expects both"; exit 1; }
done
grep -qE '^name[[:space:]]*=[[:space:]]*"redkiln"' crates/rk-cli/Cargo.toml || { echo "FAIL: rk-cli's [[bin]] is not named redkiln"; exit 1; }
n=$(grep -cE '^name[[:space:]]*=[[:space:]]*"rk-fixture-child-cli"' crates/rk-cli/Cargo.toml | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: rk-cli's SECOND [[bin]] is not the rk-fixture-child-cli target (${n:-0} name lines, not one) — the 2026-09-21 and 2026-10-01 amendments permit exactly the two fixture targets named here and NO other, so a fourth would pass the count above and fail on these two name lines (s7d §7d.13)"; exit 1; }   # → 1
n=$(grep -cE '^name[[:space:]]*=[[:space:]]*"rk-fixture-toolchain"' crates/rk-cli/Cargo.toml | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: rk-cli's THIRD [[bin]] is not the rk-fixture-toolchain target (${n:-0} name lines, not one) — the 2026-10-01 amendment (Weigh-In wi-b846d9) permits that target, which 5e67ac0 added for the review fixture's stub cargo (ADR-0162), and NO other (s7d §7d.13)"; exit 1; }   # → 1
n=$(grep -cE '^name[[:space:]]*=[[:space:]]*"redkiln-migrate"' crates/redkiln-migrate/Cargo.toml | tr -d '[:space:]'); [ "${n:-0}" -eq 2 ] || { echo "FAIL: crates/redkiln-migrate/Cargo.toml has ${n:-0} 'name = \"redkiln-migrate\"' lines, not two — the [package] name AND the [[bin]] name (§7d.15); the [package] line alone satisfies a -q grep"; exit 1; }   # → 2
grep -qE '^ctrlc(\.workspace)?[[:space:]]*=' crates/rk-cli/Cargo.toml || { echo "FAIL: rk-cli has no ctrlc row — the signal path (D18-2) has no dependency"; exit 1; }
grep -qE '^toml(\.workspace)?[[:space:]]*=' crates/rk-cli/Cargo.toml || { echo "FAIL: rk-cli has no toml row (D18-3)"; exit 1; }
grep -qE '^yaml_serde(\.workspace)?[[:space:]]*=' crates/redkiln-migrate/Cargo.toml || { echo "FAIL: redkiln-migrate has no yaml_serde row (ADR-0100 d1)"; exit 1; }
grep -qE '^jsonschema(\.workspace)?[[:space:]]*=' crates/redkiln-migrate/Cargo.toml || { echo "FAIL: redkiln-migrate has no jsonschema row — the report cannot be validated in-process (AC-448)"; exit 1; }
grep -qE '^rk-(ports|kernel|adapter-[a-z-]+)[[:space:]]*=' crates/redkiln-migrate/Cargo.toml && { echo "FAIL: redkiln-migrate's manifest names rk-ports, rk-kernel or an rk-adapter-* (AC-466; AC-441's build-graph half; §7d.15)"; exit 1; }
grep -qE '^ctrlc(\.workspace)?[[:space:]]*=' crates/redkiln-migrate/Cargo.toml && { echo "FAIL: redkiln-migrate names ctrlc — D18-2 confines the signal path to rk-cli"; exit 1; }
# root rows (Slice 1) and the ADR-0100 pin: each exactly once; the six new rows are >= 1.0 caret ranges (ctrlc is an inline table carrying its termination feature).
test -f Cargo.toml || { echo "FAIL: workspace Cargo.toml absent (Halt 8)"; exit 1; }
for row in 'clap[[:space:]]*=[[:space:]]*\{[[:space:]]*version[[:space:]]*=[[:space:]]*"4"' 'anyhow[[:space:]]*=[[:space:]]*"1"' 'toml[[:space:]]*=[[:space:]]*"1"' 'ctrlc[[:space:]]*=[[:space:]]*\{[[:space:]]*version[[:space:]]*=[[:space:]]*"3"' 'trycmd[[:space:]]*=[[:space:]]*"1"' 'assert_cmd[[:space:]]*=[[:space:]]*"2"'; do
  n=$(grep -cE "^$row" Cargo.toml | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: the root row matching '$row' is present ${n:-0} times, not once (§7d.15; ADR-0100 d2 caret rows)"; exit 1; }
done
n=$(grep -c 'yaml_serde = "=0.10.7"' Cargo.toml | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: the exact ADR-0100 pin yaml_serde = \"=0.10.7\" is not present exactly once"; exit 1; }   # → 1
for b in octocrab reqwest hyper axum ureq tokio-tungstenite warp actix-web; do
  grep -qE "^[[:space:]]*\{[[:space:]]*name[[:space:]]*=[[:space:]]*\"$b\"" deny.toml || { echo "FAIL: deny.toml carries no ban row for $b — AC-405's crate-graph instrument as amended (D-13; DECISIONS-2026-09-18 RT-36's eight)"; exit 1; }
done

# ---- the DECIDED rows' floors (§7d.1): the denylist has eleven names; N is ONE literal and the key is read (D-09);
# ---- the exit snapshot exists and carries the rows H-1 fixes; the report schema is closed with three counts and no total.
grep -rqE 'pub const DENYLIST: \[&str; 11\]' crates/rk-cli/src --include=*.rs || { echo "FAIL: no 'pub const DENYLIST: [&str; 11]' in rk-cli — ADR-0107 d3's eight plus the register's three (AC-400)"; exit 1; }
for name in resume continue stay '--stay' '--resume' '--continue' '--from' '--pick-up' '--retry-step' '--from-step' '--again'; do
  grep -rqF --include='*.rs' -- "\"$name\"" crates/rk-cli/src || { echo "FAIL: the denylist source carries no literal \"$name\" (AC-400's list)"; exit 1; }
done
c=$(grep -rnE 'INIT_UNREAD_WINDOW_RUNS[[:space:]]*:[[:space:]]*u32[[:space:]]*=[[:space:]]*50' crates/rk-cli/src --include=*.rs | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -c . | tr -d '[:space:]')   # comment-filtered: a doc comment quoting the declaration (rk-adapter-claude/src/config.rs:108's habit) is not a second literal
[ "${c:-0}" -eq 1 ] || { echo "FAIL: INIT_UNREAD_WINDOW_RUNS: u32 = 50 is declared ${c:-0} times in rk-cli, not once (D18-5; E9)"; exit 1; }   # → 1
grep -rq 'unread_window_runs' crates/rk-cli/src --include=*.rs || { echo "FAIL: rk-cli's source never names the key unread_window_runs — AC-426's required key is not read (D-09: the old regex could not match it)"; exit 1; }
snap=$(ls crates/rk-cli/tests/it/snapshots/*ac438_exit_table_snapshot*.snap 2>/dev/null | head -1); test -n "$snap" || { echo "FAIL: no committed insta snapshot for ac438_exit_table_snapshot — AC-438's 'committed snapshot of the exit-code table' is absent"; exit 1; }
test -s "$snap" || { echo "FAIL: the exit-table snapshot is EMPTY (Halt 8 (h))"; exit 1; }
eol=$(git ls-files --eol "$snap" | grep -oE 'w/[a-z]+' | tr -d '[:space:]'); [ "$eol" = "w/lf" ] || { echo "FAIL: the exit-table snapshot is checked out as '$eol', not LF — the \$-anchored row greps below would miss on CRLF; the .gitattributes -text row for tests/it/snapshots (§7d.13, Slice 2) is missing or not applied"; exit 1; }
for row in '^0 Success$' '^2 Usage$' '^3 ConfigError$' '^4 FindingsBlock$' '^5 NoRecord$' '^10 Transport$' '^13 Cancelled$' '^20 Overage$' '^70 PoisonedLockHalt$' '^130 SignalOutsideDispatch$'; do
  grep -qE "$row" "$snap" || { echo "FAIL: the exit-table snapshot has no row matching '$row' (H-1; §7d.4)"; exit 1; }
done
grep -qiE 'gate.?open' "$snap" && { echo "FAIL: the exit-table snapshot carries a gate-open row — D-01: a run that opens a gate exits 0 and the table has no gate row"; exit 1; }
c=$(grep -cE '^[0-9]+ [A-Za-z]+$' "$snap" | tr -d '[:space:]'); [ "${c:-0}" -eq 18 ] || { echo "FAIL: the exit-table snapshot has ${c:-0} rows, not 18 (§7d.4's table: 7 non-StepError rows incl. 0 Success + 11 StepError rows)"; exit 1; }   # → 18
test -s crates/redkiln-migrate/schema/report.schema.json || { echo "FAIL: schema/report.schema.json absent or empty (AC-448/AC-468)"; exit 1; }
python -c '
import json, sys
s = json.load(open(sys.argv[1], encoding="utf-8"))
assert s.get("additionalProperties") is False, "top level is not closed"
req = set(s.get("required", []))
assert {"corpus_sha", "tool_version", "per_kind_slot"} <= req, req
row = s["properties"]["per_kind_slot"]["items"]
assert row.get("additionalProperties") is False, "row is not closed"
rreq = set(row.get("required", []))
assert {"kind", "slot", "lifted", "lifted_with_loss", "failed"} <= rreq, rreq
assert "total" not in row["properties"] and "total" not in s["properties"], "a total key exists"
for k in ("lifted", "lifted_with_loss", "failed"):
    assert row["properties"][k]["type"] == "integer", k
print("schema: closed, three integer counts, no total")
' crates/redkiln-migrate/schema/report.schema.json || { echo "FAIL: the report schema is not AC-468's shape — three required integer counts per row, additionalProperties false, no total (E13's schema half)"; exit 1; }

# ---- the named tests, one per criterion, asserted BY NAME in the crate the design names (§7d.14 is the list).
for t in ac400_help_snapshot_every_verb ac400_denylist_absent_from_parser_tree ac400_resume_refused_as_unknown_argument \
         ac402_no_skip_reconcile_flag_in_snapshot ac441_no_migrate_verb_in_tree doctor_fix_flag_absent_from_snapshot \
         adr0128_run_lane_value_enum_closed adr0129_answer_grammar_positional_option_no_token_flag \
         ac438_exit_table_snapshot ac438_exit_codes_distinct ac438_step_error_mapping_is_exhaustive \
         ac403_no_blocking_stdin_read_in_rk_cli common_dir_discovered_from_inherited_cwd_via_wrapper \
         ac485_adapter_set_resolved_once_and_run_scoped ac486_misconfigured_adapter_refuses_before_spawn_or_store_write \
         ac486_init_scaffold_resolves_four_reals ac426_unread_window_runs_required_no_default \
         ac474_config_surface_snapshot_has_no_dual_write_key ac437_init_twice_identical_tree_and_identity_rows \
         ac437_init_scaffold_bytes_pinned ac531_init_created_names_match_regex ac531_preexisting_rk_name_reported_not_overwritten \
         ac412_init_installs_zero_hooks ac413_store_opened_from_other_repo_refused_zero_rows ac422_config_reads_the_one_denylist \
         adr0083_config_constructed_once_at_root_and_threaded signal_handler_fires_cancel_token \
         signal_outside_dispatch_exits_130_writes_nothing_partial adr0118_acquire_returns_halt_on_poisoned_lock \
         adr0118_poisoned_store_lock_halts_run_exit_70 \
         ac402_reconcile_called_on_run_answer_doctor ac402_status_and_explain_do_not_reconcile \
         ac403_gate_open_writes_trailer_and_event_and_exits_zero_without_waiting ac404_rate_limited_writes_retry_mark_and_exits \
         ac405_gate_answer_has_exactly_one_call_site ac406_answer_after_commit_is_void_gate_stays_open \
         ac407_second_token_refused_one_gate_answered_event ac248_answer_without_live_token_exits_named_outcome \
         ac264_run_after_applied_answer_resumes_past_gate \
         ac257_expired_gate_resolved_by_fresh_process ac410_unadmitted_attachment_refused_zero_events \
         ac410_admitted_attachment_stored_as_human_root ac263_stub_channel_refused_named_outcome \
         ac435_answer_refuses_without_explain_sufficiency_and_records_it ac401_second_run_issues_no_dispatch_for_memoised_step \
         ac430_explain_prints_both_row_families ac431_explain_names_card_skills_fillers_tier \
         ac432_explain_from_record_only_and_errors_without_resolution ac432_explain_over_replayed_log_is_byte_identical \
         ac433_explain_names_excluding_clause_never_not_selected ac434_unknown_facet_rendered_unknown \
         run_on_init_only_repo_is_a_config_outcome adr0128_lane_derived_from_item_or_refused \
         adr0128_lane_maps_to_pack_fix_bug_standard_story_story ac436_catalog_search_finds_by_keyword \
         lint_wraps_kernel_facet_rules_and_exits_by_table ac438_table_driven_exit_codes_over_fixtures \
         signal_during_dispatch_is_cancelled_by_signal_exit_13 \
         ac415_every_rule_id_has_a_rejecting_fixture doctor_rule_table_covers_every_register_named_row \
         ac414_doctor_over_corpus_fires_every_expected_rule_with_zero_dispatches ac414_doctor_module_names_no_adapter_constructor \
         ac409_gate_outputs_binding_rejected ac412_redkiln_hook_in_git_hooks_is_an_error ac413_ac369_ac370_store_identity_rows \
         ac370_stray_store_under_linked_worktree_is_an_error \
         ac416_role_unresolved_and_ambiguous_rejected ac417_card_without_structured_output_or_tools_key_rejected \
         ac418_tier_leak_on_card_model_and_filler_effort ac419_missing_or_non_numeric_timeout_rejected \
         ac420_dangling_role_slot_attach_rejected ac421_dead_definition_and_unfilled_slot_reported \
         ac422_ac488_denylisted_env_name_is_a_doctor_error ac423_ac526_cli_version_and_ttl_mismatch_rejected \
         ac423_probe_is_one_version_spawn_not_a_dispatch ac424_unknown_facet_value_and_new_facet_rejected \
         ac425_facet_without_registered_fact_rejected ac426_ac472_unread_window_49_not_flagged_51_flagged \
         ac426_rule_fire_leaves_write_set_unchanged ac486_unknown_adapter_is_a_doctor_error \
         ac203_agents_without_agent_doctor_row ac383_card_without_cache_profile_doctor_row ac377_margin_under_two_ticks_doctor_row \
         ac220_last_reading_within_margin_doctor_row ac397_rebuilt_store_tier2_emptiness_is_ok_expected \
         ac290_reproduced_finding_without_reexecution_record_doctor_row ac306_live_quarantines_listed_with_ages_doctor_row ac310_sink_count_below_finding_count_doctor_row \
         ac530_advisory_ledger_imbalance_doctor_row ac314_regression_corpus_sha_unresolvable_doctor_row ac455_verification_none_under_requiring_pack_doctor_row \
         adr0105_doctor_rows_network_fs_credential_orphan ac427_rebuild_from_git_alone_matches_recorded_next_action \
         ac429_rebuild_writes_no_object_ref_or_file ac438_doctor_error_rule_exits_config_error ; do
  rk_test rk-cli "$t" "the CLI criterion this name binds (§7d.14)"
done
for t in refuses_dirty_target idempotent_across_kill phase_written_marker_is_printed_before_commit \
         ac450_second_run_is_noop_report_to_stdout_no_commit ac447_exactly_one_commit_on_migration_branch \
         ac447_second_write_phase_aborts_before_writing ac448_report_lands_in_the_same_commit_and_validates \
         ac448_write_without_report_is_refused ac468_schema_rejects_total_only_and_folded_forms \
         ac475_zero_change_summary_values_in_output diff_prose_carried_opaque_with_loss_marker \
         ac449_two_targets_byte_identical_under_tz_and_read_order ac466_ac441_migrate_depends_on_no_harness_adapter_or_ports \
         ac470_migrated_artifacts_round_trip_through_canonical typed_gate_result_round_trips_through_canonical \
         free_form_gate_result_carried_opaque_with_loss_marker migrated_artifacts_carry_schema_version_and_no_sig \
         ac474_no_dual_write_key_flag_or_env ac442_legacy_readers_only_under_legacy_module \
         ac442_lint_flags_legacy_reader_outside_legacy_module ac522_worktree_recreated_never_reused \
         adr0075_six_typed_kinds_eight_opaque legacy_mini_provenance_matches_bytes ; do
  rk_test redkiln-migrate "$t" "the migrator criterion this name binds (§7d.11, §7d.14)"
done
# ---- THE IDS WHOSE SUBJECT CRATE IS rk-kernel, filtered IN THAT CRATE — its checkpoint rows (7b, 7b-ii) are
# ---- provisional in the slice commits that edited it (§7d.15).
for t in adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs ac436_keywords_permutation_leaves_selection_unchanged \
         ac436_keywords_unreachable_from_admissibility_evaluator adr0129_question_registry_default_question_two_options \
         adr0063_option_with_attaches_refuses_close_without_attachment catalog_load_diagnostic_returns_every_load_error_typed \
         ac409_gate_step_with_outputs_rejected_at_load replay_resume_seeds_fold_from_journal_and_steps_gate_open \
         adr0129_gate_answered_carries_answer_meta_kernel_side ; do
  rk_test rk-kernel "$t" "its register subject crate is rk-kernel (design §9.1), edited at 7d; its checkpoint rows were re-run in the slice commit"
done
# what is authored and left unrun: AC-463's corpus-bound runner, as a fn (a comment or a TODO naming it is not a test).
grep -rqE '^[[:space:]]*fn[[:space:]]+ac463_corpus_differential_runner_named\b' crates/redkiln-migrate/tests/it/ || { echo "FAIL: ac463_corpus_differential_runner_named is not authored as a fn (#[cfg(feature = \"corpus\")] #[ignore]) — it is named, not skipped"; exit 1; }

# ---- the absence bans: each GUARDED (test -d), PAIRED with a named test above, and POSITIVELY CONTROLLED under
# ---- crates/rk-cli/tests/fixtures/lint-controls/ (one directory for both crates' bans — §7d.13).
for d in $SRC_SCOPE; do test -d "$d" || { echo "FAIL: $d absent — every ban below would read as a pass over a missing directory (Halt 8 shape (a))"; exit 1; }; done
test -d "$LC" || { echo "FAIL: lint-controls absent — an absence ban with no control is not an assertion"; exit 1; }
rk_absent 'env::current_dir' 'AC-529 / H-3 / D-18: neither binary reads the process cwd — the common dir is discovered through the rk-git wrapper with "." (ci.yml carries the scope comment)'
rk_control 'env::current_dir' ac529_current_dir_control.rs
rk_absent 'io::stdin|stdin\(\)' 'AC-403 as amended (D-15): no blocking stdin read anywhere in rk-cli — no verb, init included, reads stdin (read_line( is not in the pattern: any BufRead has one — golden constraint 9)' 'crates/rk-cli/src'
rk_control 'io::stdin|stdin\(\)' ac403_stdin_control.rs
rk_absent '"ANTHROPIC_API_KEY"' 'AC-422 / AC-488: rk-cli carries no second ANTHROPIC_API_KEY literal — the ONE list is rk_adapter_claude::env_denylist (§7d.5, §7d.8)' 'crates/rk-cli/src'
rk_control '"ANTHROPIC_API_KEY"' ac422_api_key_literal_control.rs
rk_absent 'dual.write|compat.write' 'AC-474: no dual-write / compat-write route in either binary'
rk_control 'dual.write|compat.write' ac474_dual_write_control.rs
# (dated 2026-09-27, s8d build, package R3FIX-8d; s8d §8d.11) A LOCKFILE NAME IS NOT A YAML READ, and the ban takes
# the fifth-argument exclusion the bans below carry, with its admission line before it. s8d's change deriver
# (crates/rk-cli/src/change_deriver.rs, R6 = ADR-0178, proposed) ports spikes/p1/derive-facets.py's generated table,
# which names the lockfile "pnpm-lock.yaml" as a string literal compared to a changed path's file name; it opens
# no file and parses nothing. `\byaml\b` matched that literal (`.` is a non-word character), so the ban read FAIL on
# the s8d tree. The exclusion is scoped to that ONE module AND that ONE literal: any other yaml token anywhere in
# rk-cli/src, that module included, still fires. Residual, named: a YAML read written on the SAME line as that
# literal in that module would be excluded with it. Was (the line is kept, commented, so its bytes stay readable):
# ~~rk_absent 'yaml_serde|serde_yaml|\byaml\b' 'ADR-0100 d1: rk-cli touches no YAML — redkiln-migrate/src/legacy is the one reader' 'crates/rk-cli/src'~~
n=$(find crates/rk-cli/src -name '*.rs' | grep -cE '/([^/]*/)?(change_deriver)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no change_deriver module under crates/rk-cli/src — the ADR-0100 d1 yaml ban's lockfile-name exclusion names no module (s8d §8d.11)"; exit 1; }
rk_absent 'yaml_serde|serde_yaml|\byaml\b' 'ADR-0100 d1: rk-cli touches no YAML — redkiln-migrate/src/legacy is the one reader (the change deriver'"'"'s "pnpm-lock.yaml" file-name literal is excluded: it names a lockfile, it reads none)' 'crates/rk-cli/src' '' '^crates/rk-cli/src/([^:]*/)?change_deriver\.rs:[0-9]+:.*"pnpm-lock\.yaml"'
rk_control 'yaml_serde|serde_yaml|\byaml\b' adr0100_yaml_control.rs
n=$(find crates/redkiln-migrate/src -name '*.rs' | grep -cE '/([^/]*/)?(legacy)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no legacy module under crates/redkiln-migrate/src — the one permitted site of template_sig/rendered_sig has no home (§7d.11)"; exit 1; }
rk_absent 'template_sig|rendered_sig' 'AC-442 / AC-471: the legacy signature literals appear ONLY under redkiln-migrate/src/legacy (read, never written)' "$SRC_SCOPE" '' '^crates/redkiln-migrate/src/legacy(\.rs|/)'
rk_control 'template_sig|rendered_sig' ac442_template_sig_control.rs
rk_absent 'PoisonError::into_inner|catch_unwind|unwrap_or_else\([^)]*into_inner|map_err\([^)]*into_inner|lock\(\)[^;]*into_inner\(\)' 'ADR-0118 c3: a poisoned store lock halts — no poison recovery in any spelling in either binary (Reconciled::into_inner on AC-172'"'"'s wrapper is not a lock recovery and is not matched; naming the type in the halting arm is not recovery; rk-store conn.rs is RAISED, not repaired: RT-9/R-7)'
rk_control 'PoisonError::into_inner|catch_unwind|unwrap_or_else\([^)]*into_inner|map_err\([^)]*into_inner|lock\(\)[^;]*into_inner\(\)' adr0118_poison_recovery_control.rs
# ADR-0118 c6 / §7d.4: verbs return Result<Exit, Halt>; std::process::exit has EXACTLY two sites, both in main.rs — the table
# exit and the panic hook's — so the in-process twins can observe a return value instead of a dead test process.
raw=$(grep -rnE 'process::exit\(' crates/rk-cli/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the process::exit search could not run (grep rc=$rc)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
c=$(printf '%s\n' "$hits" | grep -c . | tr -d '[:space:]'); [ "${c:-0}" -eq 2 ] || { echo "FAIL: rk-cli/src carries ${c:-0} process::exit( sites, not exactly two in main.rs (§7d.4: the table exit and the panic hook; verbs return Exit)"; printf '%s\n' "$hits"; exit 1; }   # → 2
printf '%s\n' "$hits" | grep -vqE '^crates/rk-cli/src/main\.rs:' && { echo "FAIL: a process::exit( site lies outside main.rs (§7d.4)"; printf '%s\n' "$hits"; exit 1; }
# every .lock() in rk-cli goes through the ONE acquiring function (§7d.4): the ban is module-path filtered to EXCLUDE
# the store module (the fifth argument), and admission is asserted on the module the exclusion names.
n=$(find crates/rk-cli/src -name '*.rs' | grep -cE '/([^/]*/)?(store)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no store module under crates/rk-cli/src — the one acquiring function (exit 70) has no home (§7d.4)"; exit 1; }
rk_absent '\.lock\(\)' 'ADR-0118 c5: the acquisition is the composition root'"'"'s ONE function — no other Mutex .lock() in rk-cli (io::stdout().lock() / io::stderr().lock() are excluded: not Mutexes — golden constraint 9)' 'crates/rk-cli/src' '' '^crates/rk-cli/src/([^:]*/)?(store|no_dispatch)(\.rs|/)|std(out|err)\(\)\.lock\(\)'
rk_control '\.lock\(\)' adr0118_lock_control.rs
# the ONE environment read (ADR-0083; RB-M-05's owed manifest-level check): std::env::var confined to rk-cli's env module,
# and absent from redkiln-migrate entirely (AC-474: no env toggle).
n=$(find crates/rk-cli/src -name '*.rs' | grep -cE '/([^/]*/)?(env)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no env module under crates/rk-cli/src — the one environment read has no home (§7d.5)"; exit 1; }
rk_absent 'std::env::var|env::var\(|env::vars\(|env::var_os\(' 'ADR-0083: the process reads its environment ONCE, in rk-cli'"'"'s env module, while building Config' 'crates/rk-cli/src' '' '^crates/rk-cli/src/([^:]*/)?env(\.rs|/)'
rk_absent 'std::env::var|env::var\(|env::vars\(|env::var_os\(' 'AC-474: redkiln-migrate reads no environment variable — no env toggle can change its write path' 'crates/redkiln-migrate/src'
rk_control 'std::env::var|env::var\(|env::vars\(|env::var_os\(' adr0083_env_var_control.rs
# the ONE wall-clock read (AC-001 / ADR-0083; §7d.5): confined to rk-cli's clock module; redkiln-migrate reads no clock.
n=$(find crates/rk-cli/src -name '*.rs' | grep -cE '/([^/]*/)?(clock)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no clock module under crates/rk-cli/src — the one wall-clock read has no home (§7d.5)"; exit 1; }
rk_absent 'SystemTime::now|Instant::now|Utc::now' 'AC-001 / ADR-0083: the clock is read in rk-cli'"'"'s clock module only — the gate deadline and the expiry read (AC-248/AC-257) take that one value' 'crates/rk-cli/src' '' '^crates/rk-cli/src/([^:]*/)?clock(\.rs|/)'
rk_absent 'SystemTime::now|Instant::now|Utc::now' 'AC-001: redkiln-migrate reads no clock — git stamps the commit' 'crates/redkiln-migrate/src'
rk_control 'SystemTime::now|Instant::now|Utc::now' adr0083_clock_control.rs
rk_absent '\bChangeSummary\b' 'AC-475 tripwire: no ChangeSummary value or identifier in the migrator (S-BIN unrun)' 'crates/redkiln-migrate/src'
rk_control '\bChangeSummary\b' ac475_change_summary_control.rs
# AC-405's single call site is a COUNT, not an absence: exactly one non-comment `gate::answer(` in rk-cli/src.
raw=$(grep -rnE 'gate::answer\(|use [^;]*gate::(\{[^}]*\banswer\b|answer\b)' crates/rk-cli/src --include=*.rs); rc=$?   # a call site OR an import of `answer` (a bare `answer(` after an import evades the qualified pattern)
[ "$rc" -le 1 ] || { echo "FAIL: the gate::answer search could not run (grep rc=$rc)"; exit 1; }
c=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -c . | tr -d '[:space:]')
[ "${c:-0}" -eq 1 ] || { echo "FAIL: rk-cli/src carries ${c:-0} non-comment call sites of gate::answer(, not exactly one (AC-405 as amended, D-13)"; printf '%s\n' "$raw"; exit 1; }   # → 1
rk_control 'gate::answer\(|use [^;]*gate::(\{[^}]*\banswer\b|answer\b)' ac405_gate_answer_control.rs   # the SAME pattern the count uses, import alternation included (the control carries a call AND a `use …gate::answer;` line)
# D-10 / D18-7: the doctor module constructs no Harness implementor (the constructor paths), module-path filtered to doctor.
rk_absent 'ClaudeHarness::new|rk_adapter_claude::config::Config::new|\bConfig::new\(|NodeToolchain::new|RustToolchain::new|SqliteStore::open|dyn Harness' 'AC-414 as amended (D-10) / D18-7: the doctor module constructs no port implementor and opens no store — the register'"'"'s denylist at its REAL paths (Config lives at rk_adapter_claude::config; the bare Config::new( arm catches the imported spelling); the root builds its Ports and opens its store' 'crates/rk-cli/src' 'doctor'
rk_control 'ClaudeHarness::new|rk_adapter_claude::config::Config::new|\bConfig::new\(|NodeToolchain::new|RustToolchain::new|SqliteStore::open|dyn Harness' d10_adapter_constructor_control.rs
# the import spellings that evade a fully-qualified pattern (`use rk_adapter_claude::Config;` then `Config::new(`): banned in the same module.
rk_absent 'use rk_adapter_claude::(config::)?(\{[^}]*\b(Config|ClaudeHarness)\b|Config\b|ClaudeHarness\b)|use rk_adapter_toolchain_(node|rust)::(\{[^}]*Toolchain\b|[A-Za-z]*Toolchain\b)' 'D-10 / D18-7: the doctor module imports no port-implementor constructor type — env_denylist, config::TESTED_CLI_VERSION and the SqliteStore TYPE (Ctx.store, §7d.8 — the root opens it; the constructor ban above keeps SqliteStore::open out) stay importable; the toolchain Config (no port implementor, §7d.8) is not banned' 'crates/rk-cli/src' 'doctor'
rk_control 'use rk_adapter_claude::(config::)?(\{[^}]*\b(Config|ClaudeHarness)\b|Config\b|ClaudeHarness\b)|use rk_adapter_toolchain_(node|rust)::(\{[^}]*Toolchain\b|[A-Za-z]*Toolchain\b)' d10_adapter_import_control.rs
# AC-436's lint half, on the kernel: `keywords` is unreachable from the evaluator (select/, predicate/); the field exists (control).
rk_absent '\bkeywords\b' 'AC-436: keywords is unreachable from the admissibility evaluator' 'crates/rk-kernel/src' 'select|predicate'
grep -qE '^[[:space:]]*(pub(\([a-z]+\))?[[:space:]]+)?keywords[[:space:]]*:' crates/rk-kernel/src/catalog.rs || { echo "FAIL: rk-kernel's catalog loader declares no keywords member — AC-436's field is still dropped (RA-M-02) and the ban above is vacuous"; exit 1; }
rk_control '\bkeywords\b' ac436_keywords_control.rs
# ADR-0102 d3: no #[allow( / #![allow( in either new crate, src/ and tests/it (every override is #[expect]).
rk_absent '#!?\[allow\(' 'ADR-0102 d3: no #[allow( / #![allow( in rk-cli or redkiln-migrate, src/ and tests/it — every override is #[expect(clippy::…, reason)]' "$SRC_SCOPE crates/rk-cli/tests/it crates/redkiln-migrate/tests/it"
rk_control '#!?\[allow\(' adr0102_allow_attribute_control.rs
rk_absent 'set_var|remove_var' 'ADR-0083: set_var/remove_var forbidden workspace-wide, tests included — a test that needs a different TZ spawns the binary with a child env' "$SRC_SCOPE crates/rk-cli/tests/it crates/redkiln-migrate/tests/it"
rk_control 'set_var|remove_var' adr0083_set_var_control.rs
rk_absent 'Command::new\("git"\)' 'ADR-0077: no git spawn outside rk-git'"'"'s one wrapper — status --porcelain, worktree add/remove, commit, log and rev-parse all go through GitExec'
rk_control 'Command::new\("git"\)' adr0077_git_command_control.rs
# ADR-0101 clause 5 — the ONE absence line with no rk_control, because a control would BE the forbidden phrase in a 7d crate
# (the sweep covers tests/ too); s7c §7c.21's whole-crate, case-insensitive form, over every file kind.
raw=$(grep -rniE 'guarded against false approval|calibrated against false approval' crates/rk-cli/ crates/redkiln-migrate/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the ADR-0101 sweep could not run (grep rc=$rc)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a claim that the review gate is guarded against false approval appears in a 7d crate — ADR-0101 clause 5 forbids it until S-SUBTLE-CB runs (the 7b-ii lesson: even to deny it)"; printf '%s\n' "$raw"; exit 1; }   # → silent

# ---- the dependency graph (AC-441, AC-466, ADR-0100 d1, ADR-0102 d6, AC-405/D-13): the YAML set, the anyhow set, no
# ---- rk-* -> redkiln-migrate edge, redkiln-migrate reaches no rk-adapter-*/rk-ports/rk-kernel, no adapter reaches rk-cli
# ---- (AC-269's build-configuration half), and rk-cli's closure names no tracker/web crate. The walks carry their own
# ---- positive controls (they must have reached rk-types) or they walked nothing.
meta=$(cargo metadata --format-version 1 --locked 2>/dev/null); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo metadata --locked exited $rc"; exit 1; }
printf '%s' "$meta" | python -X utf8 -c '
import json, sys
m = json.load(sys.stdin)
pk = {p["id"]: p for p in m["packages"]}
byid = {n["id"]: n for n in m["resolve"]["nodes"]}
name = {i: p["name"] for i, p in pk.items()}
def closure(root_name, kinds=(None, "build")):
    root = next(i for i, p in pk.items() if p["name"] == root_name)
    seen, todo = set(), [root]
    while todo:
        n = todo.pop()
        if n in seen: continue
        seen.add(n)
        for d in byid[n]["deps"]:
            if any(k["kind"] in kinds for k in d["dep_kinds"]): todo.append(d["pkg"])
    return {name[i] for i in seen}
members = {name[i] for i in m["workspace_members"]}
yaml = sorted(p["name"] for p in pk.values() if p["name"] in members and any("yaml" in d["name"] for d in p["dependencies"]))
assert yaml == ["redkiln-migrate"], ("YAML dep set", yaml)
anyhow = sorted(p["name"] for p in pk.values() if p["name"] in members and any(d["name"] == "anyhow" for d in p["dependencies"]))
assert anyhow == ["redkiln-migrate", "rk-cli"], ("anyhow set", anyhow)
edge = sorted(p["name"] for p in pk.values() if p["name"].startswith("rk-") and any(d["name"] == "redkiln-migrate" for d in p["dependencies"]))
assert edge == [], ("rk-* -> redkiln-migrate", edge)
mig = closure("redkiln-migrate")
assert "rk-types" in mig and "rk-git" in mig, ("control: the migrate walk reached nothing", sorted(mig))
bad = sorted(x for x in mig if x.startswith("rk-adapter-") or x in ("rk-ports", "rk-kernel", "rk-containment"))
assert bad == [], ("redkiln-migrate reaches", bad)
cli = closure("rk-cli")
assert "rk-types" in cli and "rk-kernel" in cli and "rk-adapter-claude" in cli, ("control: the rk-cli walk reached nothing", sorted(cli))
web = sorted(x for x in cli if x in ("octocrab", "reqwest", "hyper", "axum", "ureq", "tokio-tungstenite", "warp", "actix-web"))
assert web == [], ("rk-cli reaches a tracker/web crate", web)
back = sorted(p["name"] for p in pk.values() if p["name"].startswith("rk-adapter-") and any(d["name"] == "rk-cli" for d in p["dependencies"]))
assert back == [], ("an adapter depends on rk-cli", back)
print("metadata: yaml", yaml, "anyhow", anyhow, "no reverse edges, migrate closure clean, cli closure clean")
' || { echo "FAIL: cargo metadata — one of: the YAML set is not exactly [redkiln-migrate] (ADR-0100 d1); the anyhow set is not exactly [redkiln-migrate, rk-cli] (ADR-0102 d6); an rk-* crate depends on redkiln-migrate (AC-441); redkiln-migrate reaches rk-adapter-*/rk-ports/rk-kernel (AC-466); rk-cli reaches a tracker/web crate (AC-405); an adapter depends on rk-cli (AC-269 build half); or a walk reached nothing (the control) — the failing assertion is printed above"; exit 1; }

# ---- THE VERBS AS PROCESSES, against a TEMP repository this block mints (D18-9) — never the product repository (its
# ---- .redkiln/ is 7e's). One commit first: SqliteStore::open needs a root commit for its identity row (§7d.9).
TMP1=$(mktemp -d); test -d "$TMP1" || { echo "FAIL: mktemp -d produced no directory"; exit 1; }
git -C "$TMP1" init -q -b main || { echo "FAIL: git init in $TMP1"; exit 1; }
git -C "$TMP1" -c user.name=cp7d -c user.email=cp7d@example.invalid commit -q --allow-empty -m "root" || { echo "FAIL: the root commit in $TMP1"; exit 1; }
# init, twice: both exit 0; the scaffold's bytes; the porcelain listing and the working-tree digest equal across the
# second run (AC-437 as amended, D-14); the store created eagerly in the common dir; the N literal in the written file (D-09).
( cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln init </dev/null ) || { echo "FAIL: the first redkiln init exited non-zero in $TMP1 (AC-437)"; exit 1; }
test -f "$TMP1/.redkiln/config.toml" || { echo "FAIL: init wrote no .redkiln/config.toml (E6; D-14)"; exit 1; }
eol=$(git ls-files --eol crates/rk-cli/tests/fixtures/init/config.toml.expected | grep -oE 'w/[a-z]+' | tr -d '[:space:]'); [ "$eol" = "w/lf" ] || { echo "FAIL: init/config.toml.expected is checked out as '$eol', not LF — core.autocrlf rewrote a byte-pinned fixture; the .gitattributes row §7d.13 names is missing or not applied"; exit 1; }
cmp -s "$TMP1/.redkiln/config.toml" crates/rk-cli/tests/fixtures/init/config.toml.expected || { echo "FAIL: the scaffold init wrote differs byte-for-byte from init/config.toml.expected (AC-437 as amended: the scaffold is fixed verbatim — §7d.9)"; diff "$TMP1/.redkiln/config.toml" crates/rk-cli/tests/fixtures/init/config.toml.expected | head -20; exit 1; }
grep -qE '^unread_window_runs[[:space:]]*=[[:space:]]*50$' "$TMP1/.redkiln/config.toml" || { echo "FAIL: the written scaffold does not carry unread_window_runs = 50 (E9; D-09)"; exit 1; }
grep -qE '^enabled[[:space:]]*=[[:space:]]*\[\]$' "$TMP1/.redkiln/config.toml" || { echo "FAIL: the written scaffold does not carry [packs] enabled = [] (E6: 7e fills it)"; exit 1; }
test -s "$TMP1/.git/redkiln/store.sqlite3" || { echo "FAIL: init did not create the store at <common-dir>/redkiln/store.sqlite3 (D-14: init is EAGER; AC-437's identity rows compare nothing without it)"; exit 1; }
por1=$(git -C "$TMP1" status --porcelain); test -n "$por1" || { echo "FAIL: the porcelain listing after init is EMPTY — .redkiln/ should be untracked here (Halt 8 (d): the comparison below would be over nothing)"; exit 1; }
files1=$(cd "$TMP1" && find . -path ./.git -prune -o -type f -print | LC_ALL=C sort); test -n "$files1" || { echo "FAIL: the working tree after init lists no file — the digest below would be of nothing (Halt 8 (d))"; exit 1; }
sums1=$(cd "$TMP1" && printf '%s\n' "$files1" | xargs -d '\n' sha256sum); rc=$?; [ "$rc" -eq 0 ] && [ -n "$sums1" ] || { echo "FAIL: sha256sum over the tree after the first init exited $rc or printed nothing (Halt 8 (c): a status-bearing command left of a pipe)"; exit 1; }
dig1=$(printf '%s\n' "$sums1" | sha256sum | cut -d' ' -f1)
( cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln init </dev/null ) || { echo "FAIL: the SECOND redkiln init exited non-zero (AC-437: idempotent — ADR-0107 d2)"; exit 1; }
por2=$(git -C "$TMP1" status --porcelain)
[ "$por1" = "$por2" ] || { echo "FAIL: the porcelain listing changed across the second init (AC-437)"; printf '%s\n--\n%s\n' "$por1" "$por2"; exit 1; }
files2=$(cd "$TMP1" && find . -path ./.git -prune -o -type f -print | LC_ALL=C sort); test -n "$files2" || { echo "FAIL: the working tree after the second init lists no file (Halt 8 (d))"; exit 1; }
sums2=$(cd "$TMP1" && printf '%s\n' "$files2" | xargs -d '\n' sha256sum); rc=$?; [ "$rc" -eq 0 ] && [ -n "$sums2" ] || { echo "FAIL: sha256sum over the tree after the second init exited $rc or printed nothing (Halt 8 (c))"; exit 1; }
dig2=$(printf '%s\n' "$sums2" | sha256sum | cut -d' ' -f1)
[ "$dig1" = "$dig2" ] || { echo "FAIL: the working-tree digest changed across the second init — a byte was rewritten (AC-437)"; exit 1; }
test -d "$TMP1/.git/hooks" || { echo "FAIL: $TMP1/.git/hooks absent after init — git init creates it; the hook count below would be over nothing (Halt 8 (a))"; exit 1; }
n=$(find "$TMP1/.git/hooks" -type f ! -name '*.sample' | grep -c . | tr -d '[:space:]'); [ "${n:-0}" -eq 0 ] || { echo "FAIL: init installed ${n:-0} hook(s) under .git/hooks (AC-412)"; exit 1; }   # → 0
echo "init is idempotent on $TMP1 (tree digest $dig2)"
# doctor on the init-only repository: exit 0 (the prose above says why), output non-empty, and the register-named
# rule-id FLOOR present in the output (RA-M-01) — an empty table fails here whatever the exit code.
out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln doctor </dev/null 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: doctor exited $rc on the init-only temp repository, expected 0 (the scaffold names four compiled-in reals; an empty enabled set is init's own state, not a defect — §7d.17); an error-severity row fired or the table is not the one §7d.8 fixes"; printf '%s\n' "$out" | tail -40; exit 1; }
test -n "$out" || { echo "FAIL: doctor printed nothing — the rule table is E5-unpinned in layout but each row prints its id (RT-44)"; exit 1; }
FLOOR="RK-D-GATE-OUTPUTS RK-D-HOOKS RK-D-STORE-ID RK-D-ROLE-UNRESOLVED RK-D-ROLE-AMBIGUOUS RK-D-CARD-TOOLS RK-D-CARD-TOOLS-ABSENT RK-D-TIER-LEAK RK-D-TIMEOUT RK-D-DANGLING-ROLE RK-D-DANGLING-SLOT RK-D-DANGLING-ATTACH RK-D-DEAD-DEF RK-D-UNFILLED-SLOT RK-D-ENV-DENYLIST RK-D-CLI-VERSION RK-D-TTL RK-D-FACET-UNKNOWN RK-D-FACET-VALUE RK-D-FACET-NEW RK-D-FACET-FACT RK-D-ADAPTER RK-D-UNREAD RK-D-REVIEW-REEXEC RK-D-QUARANTINE-LIVE RK-D-SINK-COUNT RK-D-ADVISORY-LEDGER RK-D-CORPUS-SHA RK-D-VERIFICATION-POLICY"
for id in $FLOOR; do
  printf '%s\n' "$out" | grep -qE "(^|[^A-Z0-9-])${id}([^A-Z0-9-]|$)" || { echo "FAIL: doctor's output names no row ${id} — the register-named floor (AC-409, 412, 413, 416-426, 486, 488, and the D18-19 six AC-290/306/310/314/455/530; RA-M-01) is not met; over an empty table every join is vacuous"; exit 1; }
done
c=$(printf '%s\n' "$FLOOR" | wc -w | tr -d '[:space:]'); [ "${c:-0}" -eq 29 ] || { echo "FAIL: the FLOOR list above has ${c:-0} ids, not the 29 §7d.8 counts — re-count both (Halt 7)"; exit 1; }   # → 29
# the 3 branch on the real binary (AC-438's doctor clause; AC-486's doctor half): the rejecting adapter fixture over the scaffold.
# unknown-adapter.toml is the §7d.9 scaffold byte-for-byte with `harness` alone changed to rk-adapter-linear — a COMPLETE
# ConfigFile, so the load succeeds and the rule table runs; doctor does NOT call AdapterSet::resolve as a startup refusal
# (that is run's step (4)) — it reports adapter defects only through RK-D-ADAPTER, which is why the id is observable here.
cp "$TMP1/.redkiln/config.toml" "$TMP1/config.toml.keep" || { echo "FAIL: could not keep the scaffold aside"; exit 1; }
cp crates/rk-cli/tests/fixtures/config/unknown-adapter.toml "$TMP1/.redkiln/config.toml" || { echo "FAIL: the unknown-adapter fixture is absent (AC-486)"; exit 1; }
out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln doctor </dev/null 2>&1); rc=$?
[ "$rc" -eq 3 ] || { echo "FAIL: doctor exited $rc over a config naming rk-adapter-linear, expected 3 (AC-438: an error-severity rule is the configuration-error code; AC-486's doctor half)"; printf '%s\n' "$out" | tail -20; exit 1; }
row=$(printf '%s\n' "$out" | grep -E '(^|[^A-Z0-9-])RK-D-ADAPTER([^A-Z0-9-]|$)'); test -n "$row" || { echo "FAIL: doctor printed no RK-D-ADAPTER row at all (RT-44: every evaluated rule prints its id)"; exit 1; }
printf '%s\n' "$row" | grep -qvE '^[[:space:]]*RK-D-ADAPTER[[:space:]]+ok([[:space:]]|$)' || { echo "FAIL: doctor exited 3 but the RK-D-ADAPTER row's STATUS word reads ok — a different rule fired (RK-D-CLI-VERSION or RK-D-ENV-DENYLIST would be the shell's, not the binary's); the ok row is printed on every run and a bare grep -q for the id cannot fail (RT-44: the id token, then the status word — read positionally, so an 'ok' inside the DETAIL text of a fired row cannot fail a correct binary)"; printf '%s\n' "$out" | tail -20; exit 1; }
mv "$TMP1/config.toml.keep" "$TMP1/.redkiln/config.toml" || { echo "FAIL: could not restore the scaffold"; exit 1; }
# run --resume: refused as an UNKNOWN ARGUMENT — exit 2 (clap's default, kept — H-1) and stderr naming the flag (AC-400/AC-264; ADR-0107 d3).
out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln run --resume </dev/null 2>&1); rc=$?
[ "$rc" -eq 2 ] || { echo "FAIL: run --resume exited $rc, not 2 — either ACCEPTED (0) or refused for a reason that is not the parser's (ADR-0107's denylist is breached or the usage row moved)"; printf '%s\n' "$out" | tail -5; exit 1; }
printf '%s\n' "$out" | grep -qiE 'unexpected argument' && printf '%s\n' "$out" | grep -q -- '--resume' || { echo "FAIL: run --resume exited 2 but stderr is not an unknown-argument refusal naming --resume"; printf '%s\n' "$out" | tail -5; exit 1; }
echo "resume is refused as an unknown argument"
# run <item> on the init-only repository: a CONFIGURATION outcome — exit 3, "no process pack enabled" (D18-8; RT-46), before any lane derivation.
out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln run some-item </dev/null 2>&1); rc=$?
[ "$rc" -eq 3 ] || { echo "FAIL: run on the init-only repository exited $rc, expected 3 (D18-8: an empty [packs] enabled is a configuration error before any selection)"; printf '%s\n' "$out" | tail -10; exit 1; }
printf '%s\n' "$out" | grep -qi 'no process pack enabled' || { echo "FAIL: run exited 3 but stderr does not say 'no process pack enabled' — a different refusal fired first"; printf '%s\n' "$out" | tail -10; exit 1; }
n=$(git -C "$TMP1" rev-list --count HEAD | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: $TMP1 has ${n:-0} commit(s) after the refused run — one root commit was expected, a refused run writes no commit"; exit 1; }   # → 1
# step (7)'s refusal precedes step (8)'s worktree re-creation (§7d.5; H-5 / RT-47): the refused run created no rk-run/* branch
# and no run-owned worktree — the pass condition is an absence, so the rc is checked and the positive control is the
# rev-list line above (a repository that IS there).
refs=$(git -C "$TMP1" for-each-ref 'refs/heads/rk-run/*' 2>&1); rc=$?; [ "$rc" -eq 0 ] && [ -z "$refs" ] || { echo "FAIL: the refused run created an rk-run/* branch (or for-each-ref exited $rc) — step (8) must follow step (7)'s refusal (§7d.5; H-5)"; printf '%s\n' "$refs"; exit 1; }
test ! -d "$TMP1/.git/redkiln/worktrees" || { echo "FAIL: the refused run created a run-owned worktree under <common-dir>/redkiln/worktrees/ — step (8) never ran (H-5 / RT-47)"; exit 1; }
echo "run on an init-only repository is a configuration outcome"
# the three read verbs as processes (golden constraint 4: a stub never replaced is caught only here): explain at 5 (H-1's
# NoRecord — AC-432), status at 0 (a roll-up read over an empty store is not an error), catalog search at 0 (an absent
# .redkiln/catalog/ is an empty catalog for a READ; the no-pack refusal is run's, D18-8).
out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln explain some-item </dev/null 2>&1); rc=$?
[ "$rc" -eq 5 ] || { echo "FAIL: explain on an item with no recorded resolution exited $rc, expected 5 (NoRecord — AC-432; H-1)"; printf '%s\n' "$out" | tail -5; exit 1; }
printf '%s\n' "$out" | grep -qi 'no recorded resolution' || { echo "FAIL: explain exited 5 but did not name the missing resolution (AC-432: an error, not a recomputation)"; printf '%s\n' "$out" | tail -5; exit 1; }
( cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln status </dev/null >/dev/null ) || { echo "FAIL: status exited non-zero on the init-only repository (a roll-up read over an empty store is not an error; ADR-0105 c8)"; exit 1; }
out=$(cd "$TMP1" && D:/repos/redkiln-rs/target/debug/redkiln catalog search nothing-here </dev/null 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: catalog search over an empty catalog exited $rc, expected 0 (AC-436: a search that finds nothing is not an error)"; printf '%s\n' "$out" | tail -5; exit 1; }
echo "explain, status and catalog search ran as processes: 5, 0, 0"

# ---- redkiln-migrate v0 AS A PROCESS, against a SECOND temp repository built from the synthetic legacy-mini fixture
# ---- (D18-10: no predecessor byte; PROVENANCE.md beside it). --version, then --dry-run --target: exit 0, the report on
# ---- stdout is shape-checked in python against the schema's stated shape (no python jsonschema on this box; the
# ---- in-process jsonschema validation is ac448_report_lands_in_the_same_commit_and_validates), and the target is UNCHANGED by it.
./target/debug/redkiln-migrate --version || { echo "FAIL: redkiln-migrate --version failed — v0 does not exist (PIA-002)"; exit 1; }
test -f crates/redkiln-migrate/tests/fixtures/legacy-mini/PROVENANCE.md || { echo "FAIL: legacy-mini/PROVENANCE.md absent — the fixture's no-corpus-byte statement and its sha256s are the AC-445/AC-465 evidence (D18-10)"; exit 1; }
[ -n "$(git ls-files crates/redkiln-migrate/tests/fixtures/legacy-mini/PROVENANCE.md)" ] || { echo "FAIL: legacy-mini is not tracked — the AC-465 residency count below would be over nothing (Halt 8 (e))"; exit 1; }
n=$(git ls-files 'crates/redkiln-migrate/tests/fixtures/legacy-mini/*.jsonl' 'crates/redkiln-migrate/tests/fixtures/legacy-mini/spikes/*' | grep -c . | tr -d '[:space:]'); [ "${n:-0}" -eq 0 ] || { echo "FAIL: the legacy-mini fixture tracks ${n:-0} corpus-shaped path(s) (AC-465)"; exit 1; }   # → 0
TMP2=$(mktemp -d); test -d "$TMP2" || { echo "FAIL: mktemp -d produced no second directory"; exit 1; }
git -C "$TMP2" init -q -b main || { echo "FAIL: git init in $TMP2"; exit 1; }
git -C "$TMP2" -c user.name=cp7d -c user.email=cp7d@example.invalid commit -q --allow-empty -m "root" || { echo "FAIL: the root commit in $TMP2"; exit 1; }
cp -r crates/redkiln-migrate/tests/fixtures/legacy-mini/. "$TMP2/" || { echo "FAIL: copying legacy-mini into $TMP2"; exit 1; }
git -C "$TMP2" add -A || { echo "FAIL: git add in $TMP2"; exit 1; }
git -C "$TMP2" -c user.name=cp7d -c user.email=cp7d@example.invalid commit -q -m "legacy-mini" || { echo "FAIL: the legacy-mini commit in $TMP2"; exit 1; }
head2=$(git -C "$TMP2" rev-parse HEAD); test -n "$head2" || { echo "FAIL: no HEAD in $TMP2"; exit 1; }
d=$(git -C "$TMP2" status --porcelain 2>&1); rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: git status exited $rc in $TMP2 — the prelude is wrong, not the migrator (Halt 8 (g): an absent listing is not a clean one)"; printf '%s\n' "$d"; exit 1; }
[ -z "$d" ] || { echo "FAIL: $TMP2 is dirty before the dry run — the prelude is wrong, not the migrator"; printf '%s\n' "$d"; exit 1; }
out=$(./target/debug/redkiln-migrate --dry-run --target "$TMP2" </dev/null 2>"$TMP2.err"); rc=$?   # stderr beside the temp repository, never a fixed /tmp path a second run would truncate (Halt 8 (h))
[ "$rc" -eq 0 ] || { echo "FAIL: redkiln-migrate --dry-run exited $rc"; tail -20 "$TMP2.err"; exit 1; }
test -n "$out" || { echo "FAIL: the dry run printed no report (AC-448)"; exit 1; }
printf '%s\n' "$out" | python -X utf8 -c '
import json, sys
r = json.loads(sys.stdin.read())
assert set(r) == {"corpus_sha", "tool_version", "per_kind_slot"}, sorted(r)
assert isinstance(r["per_kind_slot"], list) and len(r["per_kind_slot"]) >= 1, "no per-(kind, slot) rows"
for row in r["per_kind_slot"]:
    assert set(row) == {"kind", "slot", "lifted", "lifted_with_loss", "failed"}, sorted(row)
    for k in ("lifted", "lifted_with_loss", "failed"): assert isinstance(row[k], int) and row[k] >= 0, (k, row[k])
assert "total" not in r and not any("total" in row for row in r["per_kind_slot"]), "a total key is present (AC-468)"
kinds = {row["kind"] for row in r["per_kind_slot"]}
COMPANION = {"_integration", "_grounding", "_decomposition", "_review", "_ledger", "ledger", "closure", "design", "design-review", "slices", "storymap", "implementation", "plan", "retrospective"}
assert COMPANION <= kinds, ("legacy-mini carries one file per companion kind — fourteen (§7d.11 step (4), §7d.13); missing", sorted(COMPANION - kinds))
lifted = sum(row["lifted"] for row in r["per_kind_slot"]); loss = sum(row["lifted_with_loss"] for row in r["per_kind_slot"])
assert lifted >= 1, "nothing lifted — the migrator read no fixture file"
assert loss >= 1, "no lifted-with-loss row — legacy-mini carries one free-form gate result and one diff-prose item that MUST land with a loss marker (AC-475 carry)"
by_kind = {k: tuple(sum(row[f] for row in r["per_kind_slot"] if row["kind"] == k) for f in ("lifted", "lifted_with_loss", "failed")) for k in ("report", "spec")}
assert by_kind == {"report": (0, 1, 0), "spec": (1, 1, 0)}, ("AC-468 folded-form check over legacy-mini: report rows must read lifted 0 / with-loss 1 / failed 0 and spec rows 1 / 1 / 0 (one clean spec, one diff-prose spec, one free-form report — §7d.13); lifted 2 is a fold, with-loss 0 a drop", by_kind)
print("report: three counts per row, no total,", len(r["per_kind_slot"]), "rows,", len(kinds), "kinds (the fourteen companion kinds present),", lifted, "lifted,", loss, "with loss; report/spec counts exact")
' || { echo "FAIL: the dry-run report is not AC-448/AC-468's shape or read nothing — three distinct integer counts per (kind, slot), corpus_sha, tool_version, no total, the fourteen companion kinds present (the item, report and config rows are extra rows, §7d.11 step (4)), >= 1 lifted and >= 1 with loss, and the report/spec counts exact — no folded form (E13: the schema half is what Phase 1 owes; AC-468's folded-form conjunct)"; printf '%s\n' "$out" | head -20; exit 1; }
[ "$(git -C "$TMP2" rev-parse HEAD)" = "$head2" ] || { echo "FAIL: the dry run moved HEAD in $TMP2 — a dry run writes nothing (§7d.11 step 5)"; exit 1; }
d=$(git -C "$TMP2" status --porcelain 2>&1); rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: git status exited $rc after the dry run — the migrator damaged the repository (Halt 8 (g))"; printf '%s\n' "$d"; exit 1; }
[ -z "$d" ] || { echo "FAIL: the dry run dirtied the target (AC-522: nothing is written outside the migration worktree, and a dry run writes nothing at all)"; printf '%s\n' "$d"; exit 1; }
test ! -d "$TMP2/.git/redkiln/migrate-wt" || { echo "FAIL: the dry run created the migration worktree — a dry run stops before step 3 (§7d.11)"; exit 1; }
refs=$(git -C "$TMP2" for-each-ref 'refs/heads/rk-migrate/*' 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: for-each-ref exited $rc in $TMP2"; exit 1; }
[ -z "$refs" ] || { echo "FAIL: the dry run created rk-migrate/* branch(es):"; printf '%s\n' "$refs"; exit 1; }
echo "migrate v0 dry run on $TMP2 (HEAD $head2): report conforms, target untouched"

# ---- the 153-line file's second fence, folded in and corrected (D-09: the KEY, not a regex that could not match it;
# ---- the dual-write grep now paired with a control above). Its `test -d crates/rk-cli/src` guard is the SRC_SCOPE loop's.
grep -rn 'unread_window_runs' crates/rk-cli/src --include=*.rs >/dev/null || { echo "FAIL: N is not read in rk-cli — AC-472/AC-426 have no truth value (D-09)"; exit 1; }
raw=$(grep -rnE 'dual.write|compat.write' $SRC_SCOPE Cargo.toml crates/rk-cli/Cargo.toml crates/redkiln-migrate/Cargo.toml); rc=$?   # src/ and the three manifests ONLY: tests/ names the ac474_*dual_write* tests and config/dual-write.toml IS the red fixture (§7d.9)
[ "$rc" -le 1 ] || { echo "FAIL: the dual-write search could not run (grep rc=$rc)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*|#)')
[ -z "$hits" ] || { echo "FAIL: a dual-write/compat-write route exists in a binary's src/ or a manifest (AC-474: no configuration key, flag or environment variable enables one)"; printf '%s\n' "$hits"; exit 1; }   # → silent
now_redkiln=no; if test -d .redkiln; then now_redkiln=yes; fi
[ "$had_redkiln" = "$now_redkiln" ] || { echo "FAIL: the product repository's .redkiln/ presence changed during this block ($had_redkiln -> $now_redkiln) — D18-9: every verb ran against a temp repository (O-15: 7e creates the product scaffold, so presence is compared, not asserted absent)"; exit 1; }
rm -rf "$TMP1" "$TMP2" "$TMP2.err" || { echo "FAIL: could not remove the temp repositories (Halt 6: a held handle) — inspect $TMP1 and $TMP2"; exit 1; }
echo "CP7D-END-OF-BLOCK reached"
```

### 7d.18 Acceptance set

The claimed set is the register's **cli-doctor** area — **AC-400..AC-439 plus AC-531 (41 in force; AC-428 is a re-issued
id — read its hazard note)** — and the **v0** rows of its **migration** area (the register's §13 phase note as amended
2026-09-12 E13) — **AC-441, AC-442, AC-443, AC-445–AC-450, AC-463, AC-465, AC-466, AC-468, AC-470–AC-474, AC-476–AC-478,
plus AC-522 and AC-475** — **plus** the ids other records route here by name: **AC-263** (`s7b-ii` §7b-ii.14), **AC-485,
AC-486, AC-488** (§14's composition-root rows), the `s7c` split ids and ARMED rows (§7d.12), the `s7b-ii` rows naming
7d (§7d.12's last paragraph), **AC-186, AC-264, AC-240, AC-248, AC-257** (the CLI halves), **AC-172, AC-529** (the CLI
ends of two `rk-git` rules), **AC-147** (AC-426's §0.3 twin, amended 2026-09-18 toward the owner's row) and **AC-225**
(its `Signal` arm's production; 7c's classification). It is written as explicit ranges minus the ids excluded, each named with its reason
(`00-preamble.md`'s legend; PIA-070, PIA-071) — and the Totality sentence at the end covers exactly this list, so no id
is discharged that the list does not name.

**This section claims by id ONLY what a discriminating test at this stage can pin.** Everything else is **ARMED, NOT
DISCHARGED**, by id and reason (golden constraint 10: powerless is reported as powerless).

**At 7d's stage-open, `crates/` holds `rk-git`, `rk-kernel`, `rk-ports`, `rk-store`, `rk-types`, `rk-containment` and the
four adapters, and nothing else** [measured 2026-09-18 at `239e899`]. **There is no `rk-cli`, no `redkiln-migrate`, no
`.redkiln/` anywhere, no lane has run, `ci.yml` has never run, and the one recorded live dispatch is 7c's probe.**

**DISCHARGED HERE, by a discriminating test at this stage** — each named in §7d.14 and asserted by name in the block:

- **Router and exit table:** AC-400 (three instruments; `ac400_…` ×3), AC-186 and AC-264 (the snapshot and the refusal),
  AC-402 (both halves — the snapshot ban and the three-paths test with its negative), AC-441 (router half
  `ac441_…`; build-graph half `ac466_ac441_…` and the block's walk), AC-438 (the table, the exhaustive match, the
  snapshot, the fixtures × codes, the doctor instance — all under H-1/H-2 with gate-open at `0` by D-01), AC-403 (the
  integration test at exit `0`; the lint half over all of `rk-cli`), AC-240 (through `ac403_…`, its twin's instrument;
  the register's §0.3 row pairs it with AC-278, which is ARMED below).
- **The loop:** AC-401 (the wiring; the kernel instrument is 7b's), AC-404 (**exit half only**; the retry-policy unit
  test is 7b's under owner AC-030), AC-485 (**PROVISIONAL** — see the sub-list: the dispatch-event identity half has no
  `DispatchRecord` field to read and the carrier is routed, RT-41), AC-486 (both halves — startup refusal and
  `RK-D-ADAPTER`), AC-405 (the
  call-site count and the `deny.toml` rows), AC-406, AC-407, AC-248 (CLI half), AC-257, AC-410 (both directions — the
  positive control added by the 2026-09-18 amendment), AC-263, AC-435 (**machine half only** — the human half is
  excluded below), AC-172's CLI end (one `reconcile_started` per action-causing invocation, through `ac402_…` and
  `run_on_init_only_…`). **AC-529's CLI end is ARMED below**, not discharged: its THEN — *"the run-owned worktree is
  resolved **from the store**, never from cwd"* — is the store-resolved ownership check on the `Gate:` commit through
  `authorise_tier1_write` (H-5), claimed only once the ownership carrier record (RT-47) lands; the `env::current_dir`
  ban is `ci.yml`'s rule that cites AC-529 and is not a half of it (§7d.2 item 17's own reading).
- **`explain`, `catalog search`, `status`:** AC-430 (the two row families off the recorded pack and catalog sources, joined
  on the recorded `dag_assembled` nodes), AC-431 (**PROVISIONAL** — see the sub-list: the card by its recorded digest is
  the one field the run record carries; skills, fillers and tier are on no recorded value), AC-432 (both halves; exit
  `5`; the record is the persisted run record `run` writes — §7d.7), AC-433 (**PROVISIONAL** — the clause is re-read off
  recorded rows and the recorded vector, not off a recorded exclusion), AC-434, AC-436
  (the property test, the lint and the positive control).
- **`init`:** AC-437 (as amended — the scaffold bytes, the porcelain, the three identity rows), AC-531 (as amended —
  the lint, the tree diff over the three surfaces, the pre-existing name), AC-412 (both halves), AC-413 (as amended —
  three fields; the store test and the `doctor` fixture).
- **`doctor`:** AC-414 (as amended — the corpus run under `NoDispatchHarness` and the constructor lint), AC-415 (as
  amended — the join and the non-empty floor), AC-409 (as amended — the loader rejection), AC-416 (**`doctor`-row half**
  — the rule row and the two red fixtures; the *binder unit test* half its Verified-by names — *"a binder unit test per
  case (unresolved, ambiguous) asserting an error return and no binding"* — is unwritable against `Binder::bind` as
  built, which takes a caller-chosen card (§7d.8), and is RAISED below for a dated Verified-by note in RA-M-05's form),
  AC-417 (**`doctor`
  half**; the card-validator half is 7c's), AC-418, AC-419, AC-420, AC-421, AC-422 (**`doctor` half and the one-list
  assertion**; the `--bare` half is 7c's), AC-423 (as amended — the stubbed probe, the shipped probe named), AC-424,
  AC-425, AC-426 (N = 50; kind-per-stage; the write set), AC-472 (**`doctor` half** as amended; the `artifact.read`
  emission is the kernel's), AC-147 (as amended, the same row), AC-429, AC-488 (**`doctor` half and the edge read**;
  the construction refusal is 7c's), the `s7c`-ARMED `doctor` halves of AC-203 (`ac203_agents_without_agent_doctor_row`),
  AC-205 and AC-493 (both carried by `ac417_card_without_structured_output_or_tools_key_rejected` — the `RK-D-CARD-TOOLS`
  / `-ABSENT` rows), AC-212 (carried by `ac418_tier_leak_on_card_model_and_filler_effort`), AC-220
  (`ac220_last_reading_within_margin_doctor_row`), AC-226 (carried by `ac423_ac526_cli_version_and_ttl_mismatch_rejected`),
  AC-369 and AC-370 (`ac413_ac369_ac370_store_identity_rows`, `ac370_stray_store_under_linked_worktree_is_an_error`),
  AC-377 (`ac377_margin_under_two_ticks_doctor_row`), AC-383 (`ac383_card_without_cache_profile_doctor_row`), AC-526
  (carried by `ac423_ac526_…`) — each a row and a test, §7d.8, the carrying test named where its name carries another id
  (the ledger's rule) — and AC-397.
- **`redkiln-migrate` v0:** AC-441 (build half), AC-442 (the in-process scan and its two must-fail instruments; the
  `ci.yml` step authored), AC-447, AC-448, AC-449 (**two of three nuisance parameters swept** by the test — `TZ` and read order; the
  hash-seed arm is discharged by construction — `std::collections::HashMap` / `HashSet` are clippy-banned workspace-wide
  (`clippy.toml`, AC-004) and the block's clippy gate is the instrument, so no output can depend on a seed; stated so
  the third arm is not read as skipped), AC-450 (both — the no-op and the kill), AC-466 (**build-graph half**; the run-level zero-`dispatch` clause is vacuous with no event log and is
  asserted as *no `store.sqlite3` in the commit tree*), AC-468 (**schema half**, E13 — the folded-form conjunct on the
  `legacy-mini` exact counts), AC-470, AC-474 (both halves — the
  migrator's flag set and `rk-cli`'s key snapshot), AC-522; the negative on this stage's files for AC-465 (no
  corpus-shaped path tracked — the block's `git ls-files` line; `PROVENANCE.md`). **AC-477 is ARMED whole below** — its
  WHEN needs a render, and a *value read back* half with no render is AC-470 under a second id (§7d.11); the round-trip
  test is kept under no id.
- **Signal and halt:** the ADR-0118 clause 5 test and clause 6 surface (`adr0118_…` ×2; no register id — the record's
  own obligation), the ADR-0099 decision 2 signal path **in-process** (`signal_…` ×3 — the closure, the token during a
  dispatch, the token outside one; the register's AC-225 `Signal` arm is 7c's classification, its production is here;
  the OS-delivery arm is ARMED below).
- **Built and tested, NOT claimed as evidenced (PROVISIONAL, gating the label not the build):** **AC-475** (S-BIN
  unrun; the tripwire is built as `ac475_…`, and the second conjunct is trivially true while the type does not exist —
  the register's own vacuity disclosure; the loss-marker half is Phase 2's by E13 and `diff_prose_…` claims no id),
  **AC-445** (ARMED-local: the hash comparison needs the local predecessor tree; `legacy_mini_provenance_matches_bytes`
  is the Phase-1 evidence), **AC-443's negative** (ARMED-local with AC-445: the content-hash comparison against
  `47faec9f` needs the local predecessor tree; `PROVENANCE.md`'s no-copied-byte statement is the Phase-1 evidence and
  `legacy_mini_provenance_matches_bytes` pins the note, not the absence),
  **AC-427 verb + direction** (built as `ac427_…` over a synthetic trailer repository; **fidelity over the pinned corpus
  is `s8`'s** and the id is not claimed whole — D-21), **AC-485's `reconcile_started` carrier** (RT-41 — the members are
  built and read; the record naming the run-start carrier is OWED, and the register's *"adapter identity on both
  dispatch events"* has no `DispatchRecord` field [measured 2026-09-18], so the id is discharged on its THEN through the
  routed carrier and its carrier is labelled routed), **AC-431** (built as `ac431_…` over the run record; the card by
  its recorded `card_digest` is asserted and **skills, fillers and tier are on no recorded value** — `RecordedSpec` logs
  `card_digest`, `tools`, `schema`, `timeout_secs`, `inputs` and the binder's binding is not logged; the id is claimed
  by that field only until the binding is recorded — RAISED), **AC-433** (built as `ac433_…`; the three clause kinds
  are re-read off the recorded admissibility rows, the recorded pack and the recorded `context.json` through
  `rk_kernel::predicate`, never `select` — disclosed as a re-reading of recorded inputs, because `SkipReason` records
  only the two stage exclusions and no admissibility exclusion is on the record; the register's *from the record only*
  is met in full when a kernel-recorded `excluded` row lands — RAISED). **AC-476** is **not claimed** at all — HUMAN GATE, *"not
  machine-decidable"* whole; O-14 (OPEN) asks whether its AND clause (*"the commit does not land on a machine verdict
  alone"*) is a machine half, and until the operator answers, the migrator's behaviour that would satisfy it (the
  commit lands on `rk-migrate/<short>` and never on the target; `ac447_…`; the block's `HEAD` check) is built and
  labelled O-14's default; the id is placed in Excluded below (this sentence is a pointer, not a placement).

**CROSS-LISTED — claimed at both sites, the CLI half claimed here** (the opposite of excluded):

- **AC-417, AC-422, AC-423, AC-488** — 7c's adapter halves (`s7c` §7c.22); the `doctor` rows and the edge read here.
  These are the matching notes `s7c` said were OWED to this file.
- **AC-404** — kernel half 7b (owner AC-030); the exit here.
- **AC-401** — kernel instrument 7b; the wiring here.
- **AC-402, AC-172** — `rk-git`'s `Entry` and `reconcile_at_entry` at Step 6; the three-verb enumeration and the
  negative here.
- **AC-412, AC-413, AC-486** — the `rk-store` / startup halves and the `doctor` rows, both here in different tests.
- **AC-472, AC-147** — the `artifact.read` emission is 7b's (ADR-0116); the `doctor` row here.
- **AC-441, AC-466, AC-474** — one half in each binary, both here.
- **AC-269** *(build-configuration half)* — the card-validator half 7c's; the no-adapter-reaches-`rk-cli` walk here
  (the block's `back == []` assertion — **satisfied by construction, not by an instrument that can fail**: `rk-cli` →
  `rk-adapter-*` makes the reverse edge a cycle cargo rejects, so the line is a floor that records the fact, §7d.12).
  The catalog iterated at 7c (the binder's fixture cards) stays RAISED under
  7c's `s7e` note.
- **AC-240, AC-248, AC-257, AC-263, AC-264** — kernel halves 7b-ii; CLI halves here.
- **AC-290, AC-306, AC-310, AC-314, AC-455, AC-530** — kernel / store / policy halves discharged at 7b-ii; their `doctor`
  rows here (`RK-D-REVIEW-REEXEC`, `RK-D-QUARANTINE-LIVE`, `RK-D-SINK-COUNT`, `RK-D-CORPUS-SHA`,
  `RK-D-VERIFICATION-POLICY`, `RK-D-ADVISORY-LEDGER`; §7d.8, §7d.14). Allocated here by the operator on 2026-09-19 —
  **DECISIONS-2026-09-18 D18-19**, closing O-16 and `s7b-ii` §7b-ii.14's *"until `s7d`'s set carries them the rows are
  orphaned"*; the closed `s7b-ii` file is not edited (R-21) and its note stands as history.

**ARMED, NOT DISCHARGED — by id and by reason.** Checkpoint 7d may not be read as discharging any of these, and each is
recorded in `PROGRESS.md` §1 as carried forward:

| id | register subject and instrument | missing at 7d | where it can run |
| --- | --- | --- | --- |
| **AC-427** *(fidelity half)* | *"the computed next action for every item in the pinned corpus is identical to the recorded one"* — the CI job over the `redkiln-corpus` pin | a fetchable pin (`CORPUS_PIN` is a local path); the owner is AC-184 | **`s8`** (`rebuild_differential`, `stage_level_next_action`, local-only); SM-124 as amended |
| **AC-529** *(CLI end)* | *"the run-owned worktree is resolved **from the store**, never from cwd"* — the ownership check `authorise_tier1_write` performs on the `Gate:` commit (H-5) | the record naming where the store carries `RunOwnership` (RT-47 — routed onto `reconcile_started`'s payload); the `env::current_dir` ban is `ci.yml`'s rule, not a half of this id | the RT-47 record, then `ac403_…` / `ac406_…`'s tier-1 assertions claim it |
| **AC-366** | *"a CI job that clones, rebuilds and diffs tier-1 projections"* | a CI run; `ci.yml` has never run | a recorded CI run |
| **AC-278** *(gate-path GIVEN)* | the run's job / process group enumerated after the opening invocation exits | a gate reached after a **real** dispatch — at 7d the gate path spawns no child by construction (fake harness), so the empty set proves nothing about containment; `ac403_…` asserts it as a floor | **Step 8**'s fix-lane run |
| **AC-395, AC-361, AC-372** *(renderers)*, **AC-398**, **AC-399**, **AC-365**, **AC-367**, **AC-387** | `redkiln cost`, the cost rollup, the batch-delta renderer, the drift report, the non-comparable exclusion, next-action equality, tier tags, the cost model | a `cost` verb no Phase-1 record names; the cost unit and arithmetic form (RT-19's OWED record); AC-399's carrier | **Step 8** or the records; `s7c` §7c.22 left them ARMED and this file carries them on |
| **AC-350** | one verify result per claiming toolchain | a composite `Toolchain` (R-8) | a record, then the composition root |
| **AC-215** *(integration half)* | `live_disallowed_tool_terminates` — a paid dispatch | the operator's word; this block has no live line by design | **Step 8**, or one `cargo test -p rk-adapter-claude --features live -- --ignored live_disallowed_tool_terminates` at the operator's word |
| **AC-223** | a pack lint over the recorded slow-condition maximum | `.redkiln/` and recorded classes | **7e** |
| **AC-321**, **AC-498**, **AC-312** | a verify-run kind; CI under `--network none`; S-SUBTLE-CB | the record; a CI run; the spike | as `s7c` left them |
| **AC-463** | the Phase-2 exit check's mechanical definition | the frozen corpus in CI | Phase 2; the runner is authored `#[ignore]` behind `corpus` |
| **AC-445** *(hash half)* | the content-hash comparison over test paths against `47faec9f` | the predecessor tree, local-only | the local `corpus` suite; the provenance note runs here |
| **AC-477**, **AC-478** — the render path | AC-477: *"WHEN the artifact carrying it is rendered to markdown and the typed value is read back from the store"*, *"a render round-trip test (typed → markdown, then typed re-read from the store, not from the markdown)"*; AC-478: *"a mutation test that edits the rendered markdown of a stored artifact and asserts the next read returns the original typed values and rewrites the file"* | `rk-store`'s render path, which does not exist at this pin [measured 2026-09-18: no `fn render` under `crates/rk-store/src`]; this stage edits no `rk-store` `src/`; AC-477's WHEN is not exercised without a render, so no half of it is claimed — `typed_gate_result_round_trips_through_canonical` is kept under no id (§7d.11) | the owner record AC-411 also waits on (RAISED below), then the stage that builds the render path |
| **The signal path's OS-delivery arm, Windows** | a console `CTRL_BREAK` delivered to a `redkiln` spawned with `CREATE_NEW_PROCESS_GROUP`, exiting `13` / `130` through the table | a safe sender — `GenerateConsoleCtrlEvent` is `unsafe` FFI confined to `rk-containment`, whose surface ADR-0127 decision 2 closes at two, and D18-2 rejected widening it without an annex (§7d.10) | a record adding the sender (an ADR-0127 annex), or Step 8's operator-driven run |
| **The signal path's POSIX arm** | `SIGINT` / `SIGTERM` (the `termination` feature) delivered to the process; exit `13` / `130` on Linux | a POSIX box; `ci.yml` has never run | a recorded CI `ubuntu-latest` run (as 7c's AC-224 POSIX arm) |
| **SM-125, SM-126, SM-128, SM-129** | the void rate; `explain` sufficiency; write-path violations; pin drift | production answers; the first thirty gate answers; a writer-identity source; a drift | the instruments exist (`gate_answer_void`, `explain_sufficiency`, the AC-405/AC-411 lints, the probe); the rates accrue from `s8` |
| **SM-130** *(the >0 arm)* | re-dispatch of a memoised step | a run in which a memo is hit under the real harness | `s8`; the 0-gap arm runs here in `ac401_…` |

**Excluded, by id and reason** — only genuinely excluded ids are here:

- **The corpus-bound migration ids** (the register's §13 phase note; PIA-002): **AC-440, AC-444, AC-451, AC-452, AC-453,
  AC-454, AC-456, AC-457, AC-458, AC-459, AC-460, AC-461, AC-462, AC-464, AC-467, AC-469, AC-479** — the 1,312-file
  corpus and the 118-ref history reach this box only as Step 2's local-only suite; they are **the Phase-2 acceptance
  set for the full migration**. Until 2026-09-12 this step claimed nine of them on an instrument it cannot run here,
  the PIA-059 shape; it claims none now. **AC-446** — the bug-provenance table is `rk-kernel`'s (its `--stay` half is
  built there); **AC-473, AC-521** — the self-hosting release check, Step 8's.
- **AC-455** — `rk-kernel` **policy**, not the migrator's (its GIVEN/WHEN is a spec with `verification: None` against a
  pack policy); cross-listed to 7b-ii where the two policy-table rows are discharged; **its `doctor` rule is claimed
  here** (`RK-D-VERIFICATION-POLICY`, D18-19 — CROSS-LISTED above; O-16 closed 2026-09-19).
- **AC-505, AC-506** — Phase 2 (`rk-plugin-gen`); AC-531 is the Phase-1 criterion for the `init` half of the naming
  rule and is claimed above.
- **AC-435's human half** — **HUMAN-GATED and not machine-decidable** (*"routed to a human gate"*); the machine half is
  claimed above; **no CI job may pass or fail on it**.
- **AC-476** — **HUMAN GATE**, *"routed to a human gate; not machine-decidable"* whole (*"a human answers a gate whose
  question is whether the recorded losses … are acceptable AND the commit does not land on a machine verdict alone"*);
  the register names no machine half, O-14 asked whether the AND clause is one — **answered 2026-09-19: not split**; the
  clause is instrumented under AC-522 (§7d.11) and a dated note on AC-476 says so — and this file claims nothing on the
  id; the migration branch is never integrated by this stage; **no CI job may pass or fail on the judgement**.
- **AC-428, AC-439** — twins discharged at 7c under owners AC-218 and AC-206; the stream-corpus replay conjunct is
  unsatisfiable at the pin and its amendment is 7c's RAISED item (§7d.4). Not re-asserted.
- **AC-408** — kernel-built at 7b-ii (*"a kernel test per terminal-path variant under a fake clock; an exhaustive-match
  test"*); the CLI adds no instrument; `ac257_…` exercises the expired path through `run` and claims AC-257, not
  AC-408.
- **AC-411** — `rk-store`'s render path (*"a store test that mutates a structured section on disk"*; *"a lint forbidding
  any markdown-to-structure parser in `rk-store`"*): claimed by **no stage** [measured 2026-09-18: `grep -rn AC-411
  runbook/` → this file only]. DECISIONS-2026-09-18 R-25 lists it *"rk-store, new here"*; **the store has no render path
  at this pin** [measured 2026-09-18: no `fn render` under `crates/rk-store/src`], so *new here* would mean building
  `rk-store`'s render path inside a stage whose closed-crate rule forbids it (§7d.15) — R-25 carries a dated line
  recording that, this stage **will not claim it by proximity**, and it is **RAISED** below for an owner, with AC-478.
- **AC-427's fidelity half** — ARMED above; the verb and direction are claimed.
- **SM-122** — the v0 catalog is 7e's; no CI timing step is named. **SM-123** — no source (O-12). **SM-124** — `s8`'s.
- **The `s7b-ii` doctor/CLI halves that add no instrument** — AC-241, AC-251, AC-256, AC-260, AC-268, AC-277, AC-280,
  AC-300, AC-327, AC-330, AC-337, AC-349 (§7d.12's last paragraph): each names a CLI integration test of a kernel
  behaviour already pinned by its kernel test or a `doctor` row no §12 criterion carries; not re-asserted under a new
  name, so that the double-claim rule holds. (AC-323 is **not** among them: `s7b-ii` armed it whole, no half of it ran
  anywhere, and it is ARMED above with the verify runner.)
- **The verify block** `s7b-ii` armed to 7c naming the composition root (AC-320, AC-338, AC-339, AC-341, AC-348, AC-349,
  AC-353, AC-355, AC-359) — no §12 or §13 criterion names the verify activity runner; RAISED for re-routing, not claimed.
- **AC-344** — the verify runner's `Unverifiable { cannot_run }` mapping is not built here (the note 7c owed, §7d.12).

**CLAIMED AND BUILT HERE, NOT INDIVIDUALLY ENUMERATED ABOVE — the ledger of what is still owed, not a discharge.** The
remainder of the claimed set, by id: **AC-465's negative** on this stage's files (asserted by the block's `git ls-files`
line and `PROVENANCE.md` — the CI rule is Step 2's); **AC-471's write half** (inside `ac442_…` and
`migrated_artifacts_carry_schema_version_and_no_sig`); **AC-225's `Signal` production** (inside
`signal_during_dispatch_…`). AC-172's CLI end is enumerated in the loop bullet and AC-529's CLI end in the ARMED
table, so neither is repeated here. Each is reached by a named test whose name carries another id, and is listed here
so the Totality scan finds it placed. **Before this step is RUN, each either gains its own bullet in §7d.14 or stays
here with the test that carries it named — none is reached only by the crate-wide floor.**

**RAISED, NOT SETTLED — allocation questions this file may not decide** (`00-preamble.md`; `s7a` §7a.11), each ending in
an OWED record and recorded in `PROGRESS.md` §2. The register, SM and design dated amendments §7d.2 lists as *landed*
were landed 2026-09-18 with the records and are **not** re-raised; what remains owed after the stage-open commit:

- **ADR-0128 and ADR-0129 are `accepted` since 2026-09-19** — the operator's dated confirmation line under D18-17 and
  the flip commit (`date` unchanged) carrying ADR-0107's and ADR-0009's `superseded_by` links and the README's status
  cells for `0107` and `0009` landed the day after stage-open (§7d.16 item 0); the block's two `^status: accepted$`
  greps read them (§7d.17). *(History: both were committed `proposed` at `c709a59`.)* **The one question the stage-open commit carried is ANSWERED** (2026-09-19, the operator, recorded under
  D18-17 D-05): AC-435's THEN — *"the answer (yes/no, plus whether a TOML was opened) is recorded as a required field"*
  — records **two data in one field**. SM-126's pre-registered kill is *"more than 15 of the first 30 answers report that
  the reviewer opened a TOML **anyway**"* — the cell *sufficed = yes AND opened = yes* — which a single boolean cannot
  represent, so the one-flag reading the proposed record first carried could never count the kill. ADR-0129 decision 1
  now names **two required flags**, `--explain-sufficed yes|no --toml-opened yes|no`, and **one `gate_answered` member**,
  `explain_sufficiency { sufficed, toml_opened }`; this file builds exactly that (§7d.6, `ac435_…`), and the two records
  were corrected in place before acceptance (ADR-0128 decision 4 spells `redkiln run <item> [--lane fix]`; ADR-0129
  decision 4 has the kernel refuse inside `gate::answer` with the CLI refusing first, as §7d.6 builds). **One note is
  still carried for the flip commit:** **ADR-0129 decision 2**'s CLI lookup of the live token by item is not the item-keyed answer AC-248 and ADR-0017
  reject (the answer handed on is token-keyed), which **a dated note on AC-248's THEN** should record and decision 6's
  amendment list should name — **landed with the flip commit (2026-09-19)**, a dated note under AC-248's *Why*.
- **AC-435's Verified-by dated note** naming the shipped flags `--explain-sufficed` / `--toml-opened` and the one
  record field `explain_sufficiency { sufficed, toml_opened }` — **landed with the flip commit (2026-09-19)**, ahead of the
  close, so ADR-0129 decision 6 reads true on acceptance
  (DECISIONS-2026-09-18 RT-31); the register carries no 2026-09-18 note under AC-435 today.
- **AC-416's binder half** — its Verified-by names *"a binder unit test per case (unresolved, ambiguous) asserting an
  error return and no binding"*; `Binder::bind` takes a `BindRequest` whose card the caller already chose and
  `ConfigError` has no unresolved/ambiguous variant (§7d.8), so the instrument is the `doctor` resolver's, not the
  binder's — **a dated amendment to AC-416's Verified-by is OWED** (RA-M-05's form: the register owns the instrument,
  ADR-0092 clause 2.2); the `doctor`-row half is claimed above, the binder half is not.
- **AC-403's fake clock** — its Verified-by says *"runs `run` to a gate under a fake clock"*; a spawned binary takes no
  clock without a toggle AC-474 forbids, so `ac403_…` asserts the deadline structurally between two test-side clock reads
  under a wall-clock bound (§7d.5) — **a dated Verified-by note on AC-403 is OWED with the close commit**.
- **The per-item partition of the store** (§7d.6) — at 7d the store is one journal per repository: `SqliteStore::query`
  ignores its selector, the `gate_opened` payload carries no `item`, the answer rows are keyed on the token, and
  `fsm::fold` partitions nothing; a two-item repository would fold both items' gates into one state, so the fixtures are
  single-item and the CLI writes no filter over an id the rows do not carry — **a dated design §5 note adding `item` to
  the `gate_opened` payload (an ADR-0116 decision 5 payload change, a kernel edit) or RT-4's store-side `Query::ByItem`
  is OWED** before a two-item repository may be run.
- **Design §5.4's `deadline` format** — the row says RFC 3339 and the `gate_opened` row carries unix seconds; this file
  writes the integer so trailer and event agree (§7d.5 step (10)), and `chrono` is a workspace row, so the format is the
  design's to fix — **a dated note on the §5.4 `Gate:` row is OWED** (design toward the built record).
- **The rebuild's write path and the store's tier split** (§7d.8) — `doctor::rebuild` inserts into `t2_event` through
  `Writer::transaction` with a second copy of `SqliteStore::append`'s statement, because `SqliteStore` cannot target a
  staging file and `EventStore::rebuild` on it is a typed refusal; **an `rk-adapter-sqlite` `open_at(path)` /
  `Capability::Rebuild` implementation is OWED a record** (a closed-crate edit this stage does not make), and with it
  the tier split `RK-D-REBUILT-TIER2`'s *tier-2 emptiness* presupposes — at this pin the row reads the `rebuilt`
  sidecar only (RT-56).
- **`explain`'s two unrecorded inputs** (§7d.7) — the binder's binding (skills, fillers, tier — AC-431) is on no
  recorded value, and no admissibility exclusion is recorded (`SkipReason` carries the two stage variants only —
  AC-433); **a record logging the binding beside `RecordedSpec` and a design §5 dated note (or ADR-0116-route payload)
  for a recorded `excluded` row are OWED**; both ids sit in the PROVISIONAL sub-list until they land.
- **The registry lint's CLI surface** — `rk-kernel`'s `lib.rs` header names three lints as `rk-cli`'s (Step 7d); this
  file builds `lint --facet-stability` and `--facet-provenance` only. No §12 criterion names a verb for the registry
  lint and `s8` (6) invokes only the two facet lints; a record (or a `lib.rs` header correction) naming its surface or
  its stage is OWED. **Design §8.1's `lint` row** is owed beside it: §9.1's `rk-cli` row was amended 2026-09-18 to name
  `lint`, and §8.1's verb table has no row for it, so the design disagrees with itself until a dated row lands.
- **ADR-0091 decision 1 against the tree** — *"gix owns the common-dir read … the wrapper's `rev-parse --git-common-dir`
  fallback is not commissioned"*, while 7c shipped the executable read (`rk-adapter-sqlite/src/path.rs::common_dir`,
  Checkpoint 7c `passed 2791f1d`) and H-3 reuses it (`rk-cli` calls `path::common_dir` and issues no `rev-parse
  --git-common-dir` of its own); `redkiln-migrate` may not depend on `rk-adapter-sqlite` (AC-466) and issues the one
  `rev-parse --git-common-dir` its worktree path needs through its bootstrap wrapper (§7d.11 step (2)) — the same
  divergence at a second site, disclosed here; an erratum or a dated note on ADR-0091 is OWED — this file does not
  settle it.
- **Design §5.4's `Gate:` value on the answer commit, and the triple's missing `kind` / `reach`** — the one value
  the grammar parses is the triple, so `answer` re-writes it (§7d.6); a §5.4 row for an answer-specific value, if one
  is wanted, is a design decision no record has taken and is not invented here. And because the triple carries no
  `kind` and no `reach` while the fold's `gate_opened` arm requires `kind` and derives the token from `(kind,
  tree_hash, deadline, reach)`, **`doctor --rebuild` cannot rebuild a gate row from git alone** (§7d.8) — a rebuild
  reads an open and an answered gate alike as a `Gate:` mark and records both as unresolvable; a record widening the
  triple (a dated design §5.4 note) or making the token the fold's key is OWED before `ac427_…`'s repository may carry a
  gate, and the limit is stated in that test's doc comment.
- **Design §5.4's `stage_transition` row against the `Story:` grammar** (§7d.8, amended 2026-09-21) — the row's
  payload is *“item, from, to, tree hash, the trailer commit”* and the fold makes **`to` mandatory**, while the
  `Story:` trailer's whole value is `<project>/<slug>` and carries no stage; `rk_git::reconcile::State` carries none
  either, and `GitOp` has no blob read with which to fetch one from the trailer commit's tree [measured 2026-09-21].
  `doctor --rebuild` therefore reads `to` from `.redkiln/items/<slug>.toml`'s `stage` in the checkout, which
  recovers **only the LAST transition per item** — an item with two `Story:` marks rebuilds to one row, and the
  rejection history the second axis exists for survives on the `Slice-Verdict:` axis alone. **A dated design §5.4
  note is OWED**, naming either a widened `Story:` value carrying the stage or the item record as the declared
  source, before a repository whose items move more than once may be rebuilt; the limit is stated in
  `ac427_…`'s doc comment and in `doctor/rebuild.rs`'s header.
- **AC-263's `rk-ports` tag** — the register tags AC-263 `rk-cli`, `rk-ports`; the `Channel` enum and `submit` wrapper
  are `rk-cli`'s and `rk-ports` is not edited (§7d.15), so the tag is met by no port type; a register note or a port
  type is OWED, and the register's own vacuity warning (one live variant) stands disclosed (§7d.6).
- **PIA-047's `redkiln answer` round-trip, DEFERRED here by `s7b-ii` §7b-ii.14** — it runs as the spawned-binary tests
  `ac406_…`, `ac407_…` and `ac435_…` (open a gate with `run`, answer with the real binary, read the store), not as a
  block line — the init-only temp repository has no pack and can open no gate; **the erratum against PIA-047 stays
  OWED** (`s7b-ii` §7b-ii.14: *"an erratum against PIA-047 is OWED for prescribing a `rk-cli` instrument to a
  pre-`rk-cli` stage"*).
- **SM-123's instrument** (O-12; D-19) — an ADR in ADR-0116's form minting `doctor_rule_fired`, or an SM register
  amendment re-pointing the instrument at machine-readable `doctor` output, which then needs the stdout contract E5
  declined. 7d claims nothing on SM-123.
- **AC-476's gate** — the migration commit review is a human gate no stage schedules; the record naming when the v0
  branch is proposed for landing (if ever — v0's branch is a fixture's) is the operator's.
- **RT-41 — the run-start carrier**: AC-485's *"recorded on the run-start event"* and design P3 name a kind no table
  carries; routed onto `reconcile_started`'s payload; a dated design §5 note or an ADR-0116-route kind is OWED.
- **RT-42 — `answerer`'s source**: routed to the git author identity the wrapper stamped on the run-owned branch's
  `HEAD`, read through `GitOp::Log` **before** the `gate::answer` call and asserted equal to the answer commit's author
  after it (the corrected entry below carries the detail); the
  dated design §5 note confirming (or dropping the field) is OWED, as the 2026-09-18 amendment itself says.
- **RT-45 — the three ADR-0105 clause 6/7 `doctor` rows** (`RK-D-WORKTREE-ORPHAN`, `RK-D-NETWORK-FS`,
  `RK-D-CREDENTIAL-RETAINED`): built to the accepted record, claimed under no AC id; **a register criterion per row is
  OWED**.
- **RT-46 — the refused run's one row**: D18-8's *"zero store writes"* against AC-172's *"exactly one
  `reconcile_started` event per invocation"* was read for AC-172 and D18-8 took its dated correction in the same commit
  — **nothing further owed**; listed so the routing's history is findable.
- **RT-47 — the run-owned worktree's carrier** (H-5): ADR-0105 clause 8 and AC-529 say a tier-1 write resolves the
  run-owned worktree *from the store* and no store table or event carries the pair today; routed onto the
  `reconcile_started` payload beside RT-41's members as `RunOwnership { worktree, branch }`; **the record naming the
  ownership carrier is OWED** (a dated design §5 note, or the same ADR-0116-route kind RT-41 may mint), and AC-529's
  CLI end stays ARMED until it lands.
- **RT-48 — the `stage` member on the unread fixtures' `artifact.write` / `artifact.read` payloads** (§7d.13): ADR-0116
  decision 1's members plus the per-stage discriminator D-08's predicate needs; **a dated design §5 note widening the two
  rows' payloads is OWED**, and the fixtures re-point when 7b's emitter lands.
- **RT-49 — the two optional `[adapters]` keys** `harness_binary` and `toolchain_programs` (§7d.1's `ConfigFile` row):
  the one way a process-level test reaches the fixture child without an env toggle; **a design §9.2 dated note naming
  the `[adapters]` key set is OWED**.
- **RT-50 — where `run <item>` reads the item record** (`.redkiln/items/<item>.toml`, a top-level `kind` and a
  `[facets]` table): an existence floor; **7e's pack schema record re-points it** — nothing further owed by this stage.
- **RT-51 — the `# redkiln-hook` marker** by which `RK-D-HOOKS` recognises a redkiln-authored hook (§7d.8): no record
  names one because no Phase-1 verb writes a hook; **a dated note on AC-412 is OWED**.
- **RT-52 — the digest `explain` prints for the exit check**: both, labelled; the equality's subject is each committed
  step's `output_digest` — decided by ADR-0128 decision 4 on 2026-09-19 (O-13 closed; §7d.7). No record owed.
- **RT-53 — `redkiln-migrate`'s exit values** (§7d.11): `0` / `2` / `3` / `1` as stated, unpinned by any criterion; a
  record is owed only if one asks for a snapshot — until then a routing.
- **RT-54 — `catalog::load_diagnostic`**, the loader's diagnostic mode (§7d.1, §7d.8): a second loader entry point in a
  Step-7b crate; **the record naming it as the loader's surface is OWED**.
- **RT-55 — the node-id ↔ `Activity::id_at_version` join and the spec supplier** (§7d.1, §7d.5 step (8)–(9)): `rk-cli`'s
  `adapters.rs` binds through `Binder` and `with_spec`, and `replay::resume` takes a supplier seam where `record()` calls
  `spec_for`; **the record routing spec assembly is OWED** — `replay.rs`'s own header names it.
- **RT-56 — the rebuild marker** (§7d.8): the `<common-dir>/redkiln/rebuilt` sidecar, `doctor`'s one way of knowing a
  store was rebuilt; **a dated design §5 / §9.2 note is OWED**.
- **RT-57 — the `PackExtras` key floor** (§7d.5 step (7)): the keys `rk-cli` reads off the pack file the kernel's `Pack`
  does not expose; an existence floor **7e's pack schema record re-points** — nothing further owed by this stage.
- **RT-58 — the report `kind` vocabulary's item rows** (§7d.11 step (4)): folded into **RT-28's OWED schema record**.
- **RT-59 — the human-supplied root's carrier** (§7d.6): an `artifact.write` row written by `answer`, a verb — ADR-0116
  decision 1's producing step is *every activity output*, so the producer is widened under this routing; **a dated
  design §5 note or an ADR-0116-route record naming the producer is OWED**, beside the payload-widening question the
  `artifact.write` bullet below carries.
- **RT-42, corrected 2026-09-18 by the hardening round**: `answerer` must reach the kernel **before** `gate::answer`
  writes the `gate_answered` row, so it is read from the run-owned branch's `HEAD` through `GitOp::Log` before the call —
  the author identity the wrapper stamped on the gate-open commit, which is the configured identity the answer commit
  will carry — and asserted equal to the answer commit's `%an <%ae>` after it (§7d.6); **confirmed by the operator
  2026-09-19** — the field stays, design §5's row carries the dated confirmation, R-26 is closed.
- **The `PackExtras` keys** (`margin_ticks`, the two TTLs, `gate_deadline_secs`, per-activity `timeout_secs`, the
  gate-class expiry action — §7d.5 step (7); RT-57): `rk-cli` reads them off the pack file the kernel's `Pack` does not expose;
  **a record fixing these keys in the pack schema is 7e's** and is named here so the reader is not invented twice.
- **The card `fulfils` / skill `attach.roles` schema** the `doctor` role→card resolver reads (§7d.8's AC-416 row,
  §7d.13): no record names it; 7d fixes only the doctor reader's floor, and **7e's pack schema record owns it**.
- **The `artifact.write` row `answer` writes for an admitted attachment** (§7d.6; RT-59): ADR-0116 decision 1's shape is kept
  and the binding path travels as the instance name inside `kind`'s slot; **if a widened payload (an `instance` or
  `derived_from` member) is wanted, it is a design §5 dated note or an ADR-0116-route record**, not this file's — and
  **the producer is widened beside it**: decision 1's producing step is *every activity output*, `answer` is a verb, so
  the row's producer needs the same dated note or record (RT-59).
- **Checkpoint 6's `std::process::Command` ban** (§7d.15): it already reads 13 non-comment hits at `239e899` from 7c's
  `rk-containment` and adapter sites and gains this stage's; **the `s6` line is RAISED for narrowing** to
  `Command::new("git")` (ADR-0077, its stated warrant) or an anchored `#[expect]`-aware filter — Slice 1 marks the Step 6
  row provisional; the closed `s6` file is not amended here.
- **AC-411's owner, and the `rk-store` render path AC-477 and AC-478 also need** — the render-path
  criteria are claimed by no stage and the path does not exist at this pin (§7d.11); **an owner record (or an `s6`
  amendment with Step 6's row marked provisional) is OWED**, and R-25's *"new here"* carries a dated line recording why
  it is not.
- **The verify block's re-routing record** (AC-320/338/339/341/348/349/353/355/359) — `s7c` raised it; this file does
  not absorb it.
- **AC-278's twin row** — the register's §0.3 pairs AC-240 and AC-278; 7d discharges AC-240 and arms AC-278; the row
  stands as written and needs no amendment, stated so the split is not read as a defect.
- **A console-signal sender for the OS-delivery test** (§7d.10) — not built, because it is `unsafe` FFI outside
  ADR-0127 decision 2's two surfaces and D18-2 rejected widening that crate without an annex; **if the operator wants the
  process-level arm before Step 8, an ADR-0127 annex (a dated Consequences note recording a `#[doc(hidden)]`
  test-support sender) is the record that would license it**, landed before the slice that adds it. Until then the arm
  is ARMED above.
- **AC-422 remains COMPOUND** in the register (*"Split this before it reaches the runbook"* — not done); both halves are
  claimed at their sites; the split is the register's, OWED as 7c left it.
- **The `deny.toml` tracker/web list** — eight names this stage authored as AC-405's instrument (DECISIONS RT-36; D-13's
  *"instrument authoring, not a threshold"*); a name the list lacks is caught by review, as AC-400's denylist limit is; stated, not
  owed.
- **The register reconciliation is OWED and not performed here**: this file claims the cli-doctor area's 41 ids, the
  v0 rows and the routed ids by name, and names its discharges and arms; the per-area binding/provisional/human-gated
  counts are a register-side count.

**Checkpoint 7d is NOT a milestone.** `INDEX.md`'s *"M4 — the system runs | Checkpoint 7e passed (after 7c, 7d)"* needs
**7e** after this; this file attributes nothing to it. **ADR-0101 clause 5 bounds what the pass may be read as**: the
CLI exists, the gate is a trailer and an exit, `doctor` is a rule table, migrate v0 ran a dry run — and **no document
may claim the review gate is guarded against false approval**, not `PROGRESS.md`, not a commit message, not any
milestone row.

**Spikes this step consumes:** F19, F20, F22, F36 and F38 are read, not run — F38 for the `taskkill /T /F` shape the
kill test reproduces, F20 for the exit-0 schema void the table refuses to call success, F36 for why `explain` never
recomputes. **Spikes it runs:** **none**. **Spikes it owes or that gate its labels:** S-BIN (AC-475), S-SUBTLE-CB
(AC-312, via ADR-0101 clause 5's prohibition on the close-out), S-CANCEL-WIN (its Job Object arm ran at 7c; the CLI's
signal arm here reuses the same containment and records nothing new against the spike).

**Totality.** Every id in AC-400..AC-439 plus AC-531, every v0 migration id the opening paragraph lists plus AC-522 and
AC-475, and every cross-listed and record-routed id named in the opening paragraph and in §7d.12, appears in **exactly
one** of: DISCHARGED HERE (including its PROVISIONAL sub-list — a CROSS-LISTED note beside a discharged id names the
other site and is not a second placement), ARMED, Excluded, or the ledger — and each RAISED entry names an id already
placed in one of those four. **Every id whose `doctor` / exit / CLI / fidelity /
integration / hash / POSIX half sits in another partition is placed by its half**, each half named once — AC-404,
AC-416 (the `doctor`-row half DISCHARGED, the binder half RAISED), AC-417, AC-422, AC-427, AC-435, AC-445, AC-468,
AC-475 and AC-488 are examples, not the list; AC-477 is placed whole in ARMED and by no half. **A pointer sentence
(*"AC-278 is ARMED below"*) is not a placement.** `grep -o 'AC-[0-9]\{3\}'` over this section against the two ranges and
the opening paragraph is the re-derivation; it is run by script at the stage-open commit and re-run at the close
commit, and the difference between the two runs is written into the close commit's body.
