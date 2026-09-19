# Decisions taken for Step 7d, 2026-09-18 (D18-1…D18-18; D18-19 on 2026-09-19), the routings the stage takes, and what stays open

Same shape as `DECISIONS-2026-09-17.md`. Every entry in the first section was taken **by the operator** (Ryan
Britton), one at a time, with the grounding and the enumerated options in front of him, **before** Step 7d's
stage-open commit and before the record that depends on it was written. Grounding is a document clause or a
measured number, never taste.

**Numbering.** These carry **`D18`-ids** and open a new space: `runbook/PROGRESS.md` §2 holds no `D18` row before
this file, so it numbers from **D18-1**. Cite them as `DECISIONS-2026-09-18 D18-<n>`. Routings carry **`RT-`** ids
continuing `DECISIONS-2026-09-17.md`'s RT-1…RT-27, so this file numbers from **RT-28**; open items carry **`O-`**
ids continuing that file's O-1…O-11, so from **O-12**; residuals carry **`R-`** ids continuing its R-7…R-24, so
from **R-25**.

**Where they came from.** Step 7d's stage file was **153 lines and un-hardened** at `239e899`, with eight defects the
plan of 2026-09-18 enumerates (a stale `§12.10` line pointer, a stale phase-note pointer, a checkpoint grep that
cannot match the key every record fixes, a `doctor` line that fails a correct binary by construction, an
errata sentence the register had already absorbed, a compound AC claimed whole, an AC-414 reading no composition
root can satisfy, a companion-kind count off by one). Before any of it was rewritten the **C-01 pass** the stage
file itself mandates — *"one slice finder + refuter over AC-400–AC-439 / SM-121–SM-130 / design §8.1–8.6 with the
decidability and two-implementer lenses, before the router is written"* — ran as one finder, two adversarial
refuters and an adjudicator re-reading every citation at the primary source. **Twenty-seven divergences survived,
seven of them blocking**; the adjudication is recorded whole in `runbook/PROGRESS.md` §6 (*C-01 DISCHARGED*) and
its items are cited below by their ids (`D-01`…`D-22`, `RA-M-01`…`RA-M-07`, `RB-M-01`…`RB-M-09`) from
the adjudication file of 2026-09-18. Four of the seven blocking items were **operator decisions** — a register THEN
that had to move (D-01), two grammars in two Decision sentences of one ADR (D-02), a grammar in a Decision
sentence three later records contradict (D-04), and a scaffold no record had named (D-14) — and they are D18-17
below; three were the stage file's to take (D-07, RA-M-02, RB-M-01) and are D18-9, D18-15 and the poisoned-lock
entry under D18-4. The rest of this file is what the hardening needed decided or routed before the first line of
the new stage file was written.

**Precedence, stated once.** ADR-0092's: accepted ADRs > the AC register > the SM register > spike files > the
runbook. The register is cited by symbol (id and quoted clause), never by bare line (D15-2); a line number appears
only inside a dated parenthetical.

---

**The eighteen decisions TAKEN.**

## D18-1 · Harden first

**The choice.** `runbook/phase-1/s7d-cli-and-migrate.md` (153 lines at `239e899`) is **rewritten in the `s7c` shape
before any code is cut** — header, gate check, `§7d.1` routing table, per-behaviour sections quoting each claimed
AC by id and clause, fixtures, red-first bullets, manifests, slices, Checkpoint 7d with `rk_test` / `rk_absent` /
`rk_control` and BEFORE readings, the acceptance set partitioned as `s7b-ii` §7b-ii.14 partitions it — and
committed as a **stage-open commit that does not run the checkpoint**.

**Alternative named.** Cutting code against the 153-line file and hardening at close (rejected: `00-preamble.md`
says a checkpoint is hardened *before its first run*; D17-7 is the precedent; the C-01 pass found the file's own
checkpoint fails a correct binary at its `doctor` line — D-07 — and greps for a key spelling nothing writes — D-09).

**Grounding.** `00-preamble.md`'s maintenance rule; D17-7; the eight defects in the plan's §A; C-01 items D-07, D-09,
D-20, D-22.

## D18-2 · The signal path is `ctrlc = "3"` in `rk-cli` only

**The question.** ADR-0099 decision 2 fires the `CancelToken` from *"the CLI's signal path (Ctrl-C / SIGTERM)"*;
no signal path exists on the tree (no `ctrlc`, `signal-hook` or `SetConsoleCtrlHandler` in any `Cargo.toml`
[measured 2026-09-18, `239e899`]); `CancelledBy::Signal` is reachable through nothing; tokio is confined to
`rk-adapter-claude` and `unsafe` to `rk-containment` (ADR-0127), so the composition root has no console-handler
primitive to reach for. C-01 RA-M-04.

**The choice: `ctrlc = "3"` as a normal dependency of `rk-cli` and of no other crate**, `set_handler(move ||
token.cancel())` installed once in `main.rs` before any verb runs. Measured 2026-09-18 with `cargo search`: **ctrlc
3.5.2**, so ≥ 1.0 and a **caret** workspace row with a comment (ADR-0100 decision 2's exact-pin rule is for 0.x);
MIT / Apache-2.0; its `windows-sys` requirement is `"0.61"` and unifies with the workspace's `=0.61.2` pin (one copy;
measured 2026-09-18 from the crate manifest — *corrected the same day: this read "a second copy beside 0.61.2, a
`deny.toml` duplicate warn"*); the crate is taken with its `termination` feature so `SIGTERM` / `SIGHUP` reach the
handler on POSIX. Outside a dispatch the handler cancels a token the composition root polls at each step boundary
(`is_cancelled()`), and the process exits through the table's `SignalOutsideDispatch` row (D18-4) having written nothing
partial. **The operator was asked** (the plan's §H item 2).

**Alternatives named.** A signal wrapper in `rk_containment` (rejected: ADR-0127 decision 2 scopes that crate to
containment primitives and `run_bounded`; a console-control handler is a third surface and would need an annex to a
record accepted yesterday); deferral to Step 8 (rejected: the exit criterion's `taskkill /T /F` arm is a kill, not
a signal, so Step 8 would never exercise the path, and AC-438's *"terminates for any reason"* has no row without
it).

**Grounding.** ADR-0099 decision 2 by clause; `rk_types::CancelledBy::Signal` (`error.rs`); ADR-0100 decision 2;
the measured absence.

## D18-3 · `.redkiln/config.toml` is read with `toml = "1"` into a closed struct

**The choice.** `toml = "1"` (measured 2026-09-18: **toml 1.1.6**, caret row) deserialised through `serde` into a
`ConfigFile` struct (`rk-cli`; *corrected 2026-09-18: this read `Config`, which is the kernel value D18-14 declares —
one name for two types*) carrying `#[serde(deny_unknown_fields)]` on every table — `[adapters]`, `[packs]`, `[doctor]`. An
unknown key is a configuration error naming the key; AC-474's *"no configuration key … enables a dual-write"* is
then a property of the struct, asserted by a snapshot of its key set, not by a grep for a spelling.

**Alternative named.** The kernel-internal `toml_subset::parse` hand-walk (rejected: it exists for pack TOML under
the kernel's no-serde-surface rule and would make the config reader a second parser to keep closed by hand; design
§9.2 already names `toml` for the config file).

**Grounding.** Design §9.2; AC-474's Verified-by; AC-426's *"a required key, no default"* (a missing `[doctor]
unread_window_runs` is a deserialisation error, which is the required-key rule for free).

## D18-4 · The exit-code table — a stage header-table row under AC-438, not an ADR

**The question.** AC-438 said, before the same-day D-01 amendment struck *gate-open* from the list, *"a closed,
snapshot-pinned table in which success, configuration error (including any failing error-severity `doctor` rule),
gate-open, findings-block and each runtime error class are distinct values"*, but no exit-code type exists (`rk-types` has `StepError` at eleven variants and no code), and every
termination that is **not** a `StepError` — clap's usage error, `explain` with no record (AC-432), a store opened
from another repository (AC-413), an unprefixed `init` name (AC-531), ADR-0118's poisoned-lock halt, a signal
outside a dispatch — has no named row (C-01 D-12, RB-M-01, RA-M-04). The plan of 2026-09-18 first proposed an ADR
for the table.

**The choice: the table is decided by a row in the stage file's header table that names AC-438 as its record, and
the committed `insta` snapshot of `Exit::ALL` is the record of the values.** Why not an ADR: AC-438's own THEN says
*the committed snapshot* is what pins the table — the register already owns the instrument, and ADR-0092 (*"where
it must decide, it says so in its header table and names the record"*) makes the header row the runbook's lawful
place to enumerate rows the register left implicit; ADR-0118 clause 6 hands the halt's row to *"the CLI's surface,
owed at Step 7d"*, which is the same place. An ADR would be a second record of a table the snapshot already
records, and would have to be superseded on every renumbering. **The values:**

| code | name | when |
| --- | --- | --- |
| 0 | `Success` | a completed run, a completed verb — **and a run that opens a gate** (D18-17 D-01), and the AC-450-style no-op (*"a run on a complete item is a no-op that says so"*), and a `Void` answer — recorded as `gate_answered` + `gate_answer_void`, the gate staying open (AC-406) — which is a recorded answer, not a refusal (the stage file's H-1) |
| 2 | `Usage` | clap's default kept: an unknown argument or verb — the AC-400 refusal of `--resume` |
| 3 | `ConfigError` | an error-severity `doctor` rule; the AC-413 / AC-426 / AC-486 / AC-531 refusals; a `run` with no enabled pack (D18-8); a lane neither given nor derivable (ADR-0128 d2) |
| 4 | `FindingsBlock` | the review gate's block |
| 5 | `NoRecord` | `explain` with no recorded resolution (AC-432) |
| 10–20 | one per `StepError` variant, **in `error.rs` order** | Transport 10 · Schema 11 · Timeout 12 · Cancelled 13 · NonZero 14 · RateLimited 15 · NotAdopted 16 · NotIsolated 17 · Platform 18 · Unsupported 19 · Overage 20 |
| 70 | `PoisonedLockHalt` | ADR-0118 clause 6 — the store `Mutex` poisoned; the run stops, no `StepError`, no recovered guard, no subsequent activity; the diagnostic names the lock |
| 130 | `SignalOutsideDispatch` | Ctrl-C / SIGTERM with no dispatch in flight (D18-2) |

The mapping `StepError → Exit` is an **exhaustive `match` with no wildcard arm**, so a twelfth variant fails to
compile (AC-438's *"a new `StepError` variant with no mapping fails to compile"*); `Exit::ALL` is snapshot-pinned
and a distinctness test runs over it. **Noted, not moved:** AC-438's evidence line names *"SIGTERM → 143"* as
*outside the brief*; `Cancelled` maps to 13 and a signal outside a dispatch to 130 — the snapshot is the record and
the evidence line is a citation, not a THEN. The plan's first table (`2 ConfigError · 3 GateOpen · 64 Usage`) is
withdrawn by this entry: gate-open has no row (D-01), clap's 2 is kept rather than remapped, and ConfigError moves
to 3.

**Alternatives named.** An ADR (rejected above); letting clap's 2, anyhow's 1 and a panic's 101 leak un-snapshotted
(rejected: *"for any reason"* is the THEN; the poisoned-lock halt would surface as Rust's panic text and no test
could pin it — C-01 RB-M-01 reading A).

**Grounding.** AC-438 by clause; ADR-0118 clauses 5 and 6; `rk_types::StepError` at eleven (`error.rs`, ADR-0112);
C-01 D-12 / RB-M-01 / RA-M-04.

## D18-5 · Where `N` lives — the `[doctor]` key is read by the `RK-D-UNREAD` row; `init` writes 50

**The choice.** One value, two sites, one literal. `[doctor] unread_window_runs` is **required with no default**
(AC-426) and is read by exactly one rule row — the unread-artifact row, id `RK-D-UNREAD` — which flags a stage's
produced artifact kind with zero reads over the last `N` runs (the predicate C-01 D-08 fixes: kind-per-stage, the
owner AC-426's wording, quoted by ADR-0116). `init` writes `unread_window_runs = 50` — DECISIONS-2026-09-12 E9's
value, the fourth of the preamble's seven decisions — from one constant `INIT_UNREAD_WINDOW_RUNS` in `verbs/init.rs`
and nowhere else. The 49-run and 51-run fixtures are authored **red first**; the 153-line file's *"cannot be authored
until N is declared"* bullet is struck (D-20).

**Grounding.** AC-426 (*"a required key, no default"*); AC-472 (*"zero reads across the last N runs"*); E9; SM-139.
The stale twins — AC-472's *"declared constant in the rule table"* and *"N is not yet chosen"*, AC-147's windowless
form, SM-139's *"N is undeclared"* title, design §8.4's row — take dated amendments at stage-open (D18-18).

## D18-6 · `doctor`'s stdout stays unspecified (E5); tests assert on the `Report` value

**The choice.** No pinned stdout contract. Tests assert on `doctor::Report` (the typed value: rule id, severity,
subject, message per finding); Checkpoint 7d asserts stdout **non-empty** and the exit code from D18-4's table; the
rule-id floor is asserted twice: `doctor_rule_table_covers_every_register_named_row` over the `RULES` const, and
Checkpoint 7d's grep of the temp repository's stdout for each floor id. The one thing stdout must carry is one line per
**evaluated** rule beginning with its id token and a status word (`ok` included), so the floor is readable off a clean
repository — a minimum, not a schema. *(Corrected 2026-09-18, same day: this read "the floor line reads the emitted ids
through a named test over the `Report`, not by parsing prose" and "each fired rule's id token" — under which a
conforming doctor prints nothing on a clean repository and the block's FLOOR loop fails it for a reason unrelated to
the rule table.)*

**Alternative named.** A `--json` flag with a pinned schema (rejected: E5 declined a stdout contract on the ground
that every Step 8 assertion has a typed instrument; a schema written to satisfy SM-123's fallback would be deciding
D-19 by default — it is O-12 instead).

**Grounding.** DECISIONS-2026-09-12 E5; AC-414's *"reports every expected rule"* read on the value; SM-123 (O-12).

## D18-7 · `doctor` never constructs a real Harness — `NoDispatchHarness`

**The choice.** The `Ports` value `doctor` runs with carries `harness = NoDispatchHarness`, a fake whose `dispatch`
panics naming the rule that reached it; `doctor` never calls `rk_adapter_claude::Config::new` or
`ClaudeHarness::new`. This is the only reading of AC-414's *"the doctor code path does not link a live adapter"*
that a composition root linking all four reals can satisfy (C-01 D-10): *live adapter* = a **constructed
`Harness` implementor**; `env_denylist`'s consts (AC-422's row) and the version-probe function (AC-423's row,
`claude --version` — a spawn, not a dispatch, D-11) stay readable from the doctor module. AC-414's Verified-by is
amended toward that definition (D18-18). SM-121's dispatch count during `doctor` is asserted at 0 over the AC-415
corpus.

**Grounding.** AC-414 by clause; SM-121; AC-422 / AC-423 (the adapter halves 7c discharged, the doctor rows 7d's).

## D18-8 · `run` on an init-only repository is a configuration outcome, not a partial run

**The choice.** On a repository where only `init` has run: load `config.toml` → resolve the adapter set (AC-485 /
AC-486) → `reconcile_at_entry(Entry::Run)` (AC-402) → **`ConfigError` "no process pack enabled"**, exit 3, **exactly
one store row — the invocation's `reconcile_started` (AC-172: *"exactly one `reconcile_started` event per
invocation"*) — and zero dispatches** *(corrected 2026-09-18, same day: this read "zero store writes", which AC-172's
instrument cannot hold beside; the stage file's RT-46 reads it for AC-172)*. The full loop — dispatch, memo, gate — is exercised in-process through
`rk_kernel::replay::resume` — the store-seeded twin of `replay::record` the stage file adds as a kernel edit, because
`record()` folds a fresh local vector and breaks on `OpenGate` without appending it *(corrected 2026-09-18, same day, by
the second hardening round: this read `replay::record`)* — against the fakes and the synthetic `tests/fixtures/pack-min/`
pack, never against the empty product repo. Checkpoint 7d asserts the refusal, its message and that no commit landed, on a temp repo; the one
`reconcile_started` row and the zero dispatch count are `run_on_init_only_repo_is_a_config_outcome`'s (`s7d` §7d.5,
§7d.14).

**Alternative named.** Shipping a fixture pack under `.redkiln/packs/` at 7d so `run` can run (rejected: E7 fixes
the two Phase-1 packs as 7e's and `doctor` loads every pack in that directory — a fixture pack would sit in front
of Checkpoint 7e's `doctor`).

**Grounding.** E6 / E7; AC-485 / AC-486 by clause; AC-402; ADR-0107 decision 1's no-op sentence.

## D18-9 · Checkpoint 7d's `init`, `doctor`, `run` and dry-run lines target temporary repositories

**The choice.** Every checkpoint line that runs the binary against a repository runs it against a repo made with
`mktemp -d` + `git init` (a fixture copied in where the line needs one), **never the product repo**. The product
repo's `.redkiln/` is 7e's to create (O-15). This dissolves C-01 D-07 (a `doctor` before `init` in a repo with no
config fails a correct binary; an `init` at the product root leaves an untracked scaffold on every run) and RB-M-09.

**Grounding.** AC-426's required key; AC-437's *"identical tree"*; D-07's resolution (*"run both against a
temporary fixture repository"*).

## D18-10 · The dry-run fixture is synthetic: `legacy-mini`, no predecessor bytes

**The choice.** `crates/redkiln-migrate/tests/fixtures/legacy-mini/` — hand-authored: one initiative, one project,
two stories; `story.md` / `spec.md` with the predecessor's frontmatter **shape**, a fenced-yaml `_ledger.md`, a
`_review.md`, one free-form gate `report.md` (AC-477), one diff-prose item (AC-475), `.redkiln/config.yaml` —
**no bytes copied from `D:/repos/redkiln` at `47faec9f`** (AC-465 forbids tracking corpus material; AC-445's
corpus half stays ARMED-local), with a `PROVENANCE.md` carrying a sha256 per file. Tests and the checkpoint copy it
into a temp git repo (D18-9).

**Grounding.** AC-465 / AC-445 by clause; the `ci.yml` AC-465 allowlist rule (RT-23's shape — nothing here needs a
row because nothing is extracted).

## D18-11 · The migration branch and worktree

**The choice.** Branch **`rk-migrate/<target-HEAD-short>`**; worktree **`<git-common-dir>/redkiln/migrate-wt/`**,
**re-created never reused** — `rk_git::worktree::add` refuses an existing path, and that refusal **is** AC-522's
clause — its THEN: *"it writes only into a dedicated migration branch checked out in its own worktree (created through
the wrapper), so an interrupted run — killed between write and commit — leaves the target's original branch and working
tree untouched"*; its Verified-by: *"re-created, never reused — ADR-0105 clause 6"* — not an obstacle to route around. Both names carry the `rk-` prefix (AC-531's regex, applied to what a redkiln binary
creates outside `.redkiln/`).

**Grounding.** AC-522; ADR-0105 clause 6; `rk_git::worktree::{add, remove}` as built at Step 6.

## D18-12 · AC-450's kill seam is a printed phase marker

**The choice.** The binary prints **`phase: written`** to stdout after the write phase and before the commit; the
AC-450 kill test spawns the **real** binary, kills it on that line (F38's shape — a real kill, not a mocked one), and
re-runs; an in-process twin drives the same seam through a phase hook. **No flag and no environment variable** toggles
the seam (AC-474's ban on a compatibility or dual-write route covers a test-only route too). A digest that differs
across the boundary is **Halt 4**, not a retry.

**Grounding.** AC-450 by clause; AC-474; F38; `halts.md` 4.

## D18-13 · The denylist lint walks the built `clap::Command` tree

**The choice.** `ac400_denylist_absent_from_parser_tree` builds the `clap::Command`, walks every subcommand, argument
and alias recursively and asserts no long, short or alias name matches `const DENYLIST: [&str; 11]` in `cli.rs` —
ADR-0107 decision 3's eight names plus the register's `--retry-step`, `--from-step`, `--again`. It is **not** a
source grep (evadable by a `const` or a format string). Its mutant — add `--resume` to `run` — must flip it.

**Grounding.** AC-400's *"a router lint over the parser definition"*; ADR-0107 decision 3; the RT-13 lesson (a grep
against a spelling is evadable).

## D18-14 · `Config` is declared in `rk-kernel`, constructed at `rk-cli`

**The question.** ADR-0083's Decision says *"Configuration reaches the kernel as a value: a `Config` struct
constructed at the composition root and passed in"*; no `pub struct Config` exists in `rk-kernel`, `rk-types` or
`rk-ports` [measured 2026-09-18]; no record names the declaring crate (C-01 RB-M-05).

**The choice: `pub struct Config` in `crates/rk-kernel/src/config.rs`** — a **Step-7d edit to a Step-7b crate**,
marked in the stage file's manifests section with `rk-kernel`'s checkpoint lines re-run — **constructed only at
`rk-cli`'s composition root**, threaded into `RunInputs` by a new `with_config`, and **recorded on the run-start
event** (AC-485's *"the adapter set recorded on the run-start event is the one used for every subsequent
dispatch"*). The process reads its environment in exactly one module of `rk-cli` while building it (design §9.1's
one-place property; the checkpoint gains the owed grep confining `std::env::var` to that module). Design §9.1 takes a
dated note.

**Alternatives named.** `rk-types` (rejected: `rk-types` holds wire types with `Eq` and canonical serialisation;
`Config` carries adapter construction values that are never an event payload — what the run-start event records is
the resolved adapter identity, not the struct); an `rk-cli`-private struct with the kernel keeping per-value APIs
(out: ADR-0083's sentence is a Decision sentence and *"reaches the kernel"* is its verb).

**Grounding.** ADR-0083 by clause; AC-485 / AC-486; design §9.1's environment paragraph (*"has no criterion"* — the
grep is the owed instrument).

## D18-15 · `catalog search` and the loader's `keywords` field are 7d's; `lint` wraps the kernel's facet lints

**The choice.** The verb list is **`run`, `answer`, `status`, `explain`, `doctor` (`--rebuild`), `init`, `catalog
search`, `lint`** — eight, not the 153-line file's six. `catalog search` is a Phase-1 verb in design §8.1 pinned by
AC-436; the 7b loader has **no `keywords` field**, so AC-436's permutation test passed by construction (C-01
RA-M-02: *"a check that cannot fail"*). 7d adds `keywords: Vec<String>` to the loader's `Activity` (`catalog.rs`; `Step` is an activity's `[[step]]`, not the
activity — *corrected 2026-09-18, twice the same day: the first correction itself read `Step` as "a pack step"*; a
kernel edit under D18-14's rule), the verb searches it, and AC-436's property test runs over a **synthetic catalog** with a **positive
control** (a search that finds an activity by keyword) until 7e ships v0; the unreachability lint asserts the
evaluator never reads the field. `redkiln lint --facet-stability | --facet-provenance` wraps the two kernel lints
`rk-kernel`'s `lib.rs` says are *owed a CLI surface*. Design §9.1's `rk-cli` row is amended toward §8.1 (D18-18).

**Grounding.** Design §8.1's `catalog search` row (*"keyword search over an engine-ignored field"*, Phase 1);
AC-436 by clause; `rk-kernel/src/lib.rs`'s owed-surface comment.

## D18-16 · The doctor surface is `doctor` and `doctor --rebuild` only

**The choice.** No `--fix`, no `--probe`. Design §8.1 gives `doctor --fix` **no phase** (CONDITIONAL, §8.8: *"no
install mechanism is decided"*) and `doctor --probe` is *"an unnamed verb until one exists"*; ADR-0092 has the runbook
*defer to the design for phase order*. The AC-400 snapshot **pins the absence** of both. AC-402's 2026-09-12
rationale clause (*"can act via `--fix` and `--rebuild`"*) takes a dated note reading *`--rebuild` (and, if it ships,
`--fix`)* (D18-18). `doctor --rebuild` is claimed as **verb + direction** (AC-429: no git object, ref or file
written); next-action fidelity is `s8`'s over the local corpus (owner AC-184; C-01 D-21).

**Grounding.** Design §8.1 rows by phase column; AC-402; AC-427 / AC-429; C-01 RB-M-06, D-21.

## D18-17 · The four C-01 operator decisions

Each was BLOCKING in the adjudication — a place two competent implementers reading the records build different
binaries — and none was the stage file's to take.

**D-01 · A run that opens a gate EXITS 0.** The register said both *"the process exits 0"* (AC-403's THEN; AC-240,
AC-024 twin it) and that gate-open is a snapshot value *"distinct"* from success (AC-438); no accepted Decision
sentence broke the tie (ADR-0009's *"exits 0"* is Context). **Taken: AC-403's side.** *Gate-open* is struck from
AC-438's distinct-value list by dated amendment and design §8.1's exit-code paragraph is amended to match; the
table (D18-4) has **no gate row**. *Alternative rejected:* a distinct non-zero code — it would move three register
THENs, put an ERRATA entry on ADR-0009, and rewrite the `s7d` and `s8` lines that already assume 0; four records
against one, and design §8.1's own *"a read of the world, a decision, and an exit"* describes a success.

**D-02 · One `run` grammar → ADR-0128 (proposed).** ADR-0107 fixed `run <item>` (decision 1) and `run --lane fix
--item <item>` (decision 4) in two Decision sentences; the register, the design and `s8` carry both; whether
`--lane` is required, optional or derivable was stated nowhere, and `fix`/`story` named no pack. **Taken:
`redkiln run <item> [--lane fix|story]`** — positional item; the lane optional, derived from the item's kind when
derivable, refused as `ConfigError` when neither given nor derivable; `fix → bug-standard`, `story → story` (E7).
Written as **ADR-0128, a successor-in-part of ADR-0107** (decision 4's grammar sentence only; every other clause
stands), committed `status: proposed`; the operator flips it. *Alternatives rejected:* two required flags
(contradicts decision 1 and four register THENs); positional only with no flag (an item whose kind is not derivable
could never run). An erratum was not available — ADR-0092 forbids one on a Decision sentence.

**D-04 · The `answer` grammar and the question registry → ADR-0129 (proposed).** ADR-0009's Decision sentence
named `approve|changes-requested` as the literal grammar; ADR-0063 makes an option a registry id that *"may declare
`attaches`"*; AC-410 has `answer` *"passed an attachment"*; the tree has no registry; AC-410's three instruments
pass on a refuse-all validator. The token half was **refuted** (AC-248: `redkiln answer <item> approve`, no token
argument — the CLI resolves it). **Taken: `redkiln answer <item> <option> [--attach <kind>=<path>]`**, plus the
required AC-435 flag (D-05 below); token CLI-resolved from the open gate record, **no `--token`**; options
validated against an ADR-0063 question registry whose **`pub` type lands in `rk-kernel` at 7d** with the default
question (`approve` / `changes-requested`) built in; **pack-declared questions are 7e's**; AC-410 gains a
**positive control** (an admitted attachment accepted and stored as a human-supplied root). Written as **ADR-0129, a
successor-in-part of ADR-0009** (the grammar sentence; the CLI-only channel clause stands verbatim), committed
`proposed`. *Alternatives rejected:* a two-literal `clap` enum (AC-410's WHEN unreachable, AC-435's field
uncarried, ADR-0063 dead on arrival); the registry deferred wholesale to 7e (the option is again an untyped string
and the positive control cannot be written).

**D-05, taken with D-04 · AC-435's machine half.** `answer` takes **two required** flags — whether `explain` sufficed,
and whether a TOML was opened — default names **`--explain-sufficed yes|no --toml-opened yes|no`**, the implementer may
rename them and the stage file names the ones shipped (RT-31) — and the `gate_answered` payload gains **one** member,
`explain_sufficiency { sufficed, toml_opened }`. *(As first recorded 2026-09-18 this was one flag with the second datum
read as its `no` arm; the stage-open commit RAISED *one field or two?* for the operator. **Answered 2026-09-19: two data,
one field** — the THEN says *"the answer (yes/no, plus whether a TOML was opened) is recorded as a required field"*, and
SM-126's pre-registered kill, *"opened a TOML anyway"*, is the cell *sufficed = yes AND opened = yes*, unrepresentable
by one boolean. ADR-0129 decision 1 corrected before acceptance.)* Design §5's row (which also names an `answerer` the
kernel never writes — R-26) is amended toward the register; **R-26 is closed 2026-09-19**: the operator confirmed the
routed source (RT-42 — the git author identity read through the `rk-git` wrapper), and the field stays.

**D-14 · `init` is EAGER.** No record named what `init` creates beyond `config.toml` (E6 recorded it as owed); under
lazy creation AC-437's *"identical store identity rows"* compared nothing and an empty `[adapters]` left a bare-init
repo unrunnable under AC-486. **Taken:** `config.toml` names the four compiled-in reals — `harness =
"rk-adapter-claude"`, `store = "rk-adapter-sqlite"`, `toolchains = ["rk-adapter-toolchain-node",
"rk-adapter-toolchain-rust"]` — with `[packs] enabled = []` (7e fills) and `[doctor] unread_window_runs = 50`
(D18-5); `init` **creates the SQLite store** at `<git-common-dir>/redkiln/store.sqlite3` via
`rk_adapter_sqlite::SqliteStore::open`, so AC-437 compares **three** real identity rows (`root_commit`,
`common_dir`, `schema_version` — boot-id was cut, `identity.rs`; RA-M-07). AC-531's regex is scoped by dated
amendment to **the working tree, refs and hooks** — the git common dir is not user-visible namespace, so
`redkiln/` under it is not red. AC-437's Verified-by quotes the scaffold. *Alternative rejected:* lazy creation
with an empty `[adapters]` (AC-437 and AC-531 run over nothing; `run` refuses until the operator hand-edits a
file `init` was supposed to write). **D-15, taken with it:** `init` is **non-interactive** — no verb reads stdin in
Phase 1; AC-403's Verified-by loses *"outside `init`"* by dated amendment. **D-18, taken with it:** verbs discover
the git common dir from the inherited cwd **through the `rk-git` wrapper** (`rev-parse` via `GitExec` with `.`),
never `std::env::current_dir`; `-C <path>` is optional; `ci.yml`'s AC-529 rule gains a comment scoping the ban to
library and tier-1 code paths.

**Grounding for all four.** The C-01 adjudication items D-01, D-02, D-04, D-05, D-14, D-15, D-18 by id; ADR-0092
(erratum vs supersession; the runbook's header-table licence); AC-403, AC-438, AC-400, AC-401, AC-186, AC-264,
AC-248, AC-405, AC-406, AC-410, AC-435, AC-437, AC-531, AC-485, AC-486 by clause; ADR-0107, ADR-0009, ADR-0063,
ADR-0083 by Decision sentence; `identity.rs` as built at Step 6.

**Confirmed 2026-09-19 (dated line, the operator's word).** ADR-0128 — one `run` grammar, `redkiln run <item>
[--lane fix|story]`, with the given-vs-derived disagreement named as the same configuration error and the exit check's
digest named as each committed step's `output_digest` (rounds 1 and 3 of 2026-09-19, each asked with its alternatives
named) — and ADR-0129 — the `answer` grammar with **two required flags** `--explain-sufficed yes|no --toml-opened yes|no`
and **one** `gate_answered` member `explain_sufficiency { sufficed, toml_opened }`, the kernel refusing a missing
declared attachment inside `gate::answer` with the CLI refusing first, `answerer` kept as routed (round 2) — are
confirmed as written on this date. Both flip `proposed → accepted` in the commit this line licenses, `date` unchanged
(ADR-0126's route); ADR-0107 and ADR-0009 take their `superseded_by` links and the README its status cells in the same
commit. R-36 is closed by it.

## D18-18 · The C-01 pass's dated amendments land at stage-open, not accrued

**The choice.** Every dated in-place amendment the C-01 adjudication owes to a record above the runbook is landed
**in the stage-open commit** by the amendments author, in D15-2's form (a dated parenthetical at the site; never an
erratum; never a line-number citation outside the parenthetical), rather than accrued to the close as 7c's
register debt was. **By id:**

- **AC register:** AC-438 (gate-open struck from the distinct-value list — D-01; the non-StepError rows enumerated
  — D-12); AC-400 Verified-by (the re-invocation reads `redkiln run <item>` — D-02 / ADR-0128 d4); AC-405 and
  AC-406 Verified-by (the flags named; the symbol is `rk_kernel::workflow::gate::answer` behind AC-262's wrapper
  and the crate-graph instrument is a `deny.toml` ban list — D-04 / D-13); AC-410 Verified-by (the positive control
  — D-04); AC-437 Verified-by (the scaffold quoted; the store
  created — D-14); AC-531 (regex scoped to the working tree, refs and hooks — D-14); AC-403 Verified-by (*"outside
  `init`"* struck — D-15); AC-436 Verified-by (the positive control — RA-M-02); AC-472 (*"declared constant in the
  rule table"* → the repository's `unread_window_runs` key; *"N is not yet chosen"* → 50 — D-08 / D-20); AC-147
  (gains the window — D-08); AC-414 Verified-by (*live adapter* = a constructed `Harness` — D-10); AC-423
  Verified-by (the shipped probe named — D-11); AC-415 Verified-by (the non-empty floor — RA-M-01); AC-409
  Verified-by (the compile-time clause → a loader test — RA-M-05); AC-413 GIVEN (boot-id dropped — RA-M-07);
  AC-402 (the `--fix` rationale clause — RB-M-06).
- **SM register:** SM-139 (N = 50 — D-20); SM-129 (*per dispatch*, not *on every event* — D-11); SM-124 (*the
  local-only corpus suite* — D-21). SM-123's instrument takes only a dated note recording that it has no source and that
  7d claims nothing on it — the instrument itself is not re-pointed (O-12).
- **Not landed at stage-open, by design:** AC-435's Verified-by note naming the shipped machine-half flag — RT-31 lands
  it **at close**, once the spelling shipped is known; the register carries no 2026-09-18 note under AC-435 until then.
  Four further twin texts that still read N undeclared or a rule-table constant — design §8.10's open-items row (under
  *"What this section leaves open"*, not `## 10.`), the SM register's open-items row for SM-139, the AC register's §0 open-items row 14 and its §14 narrative — take the same
  dated amendment as SM-139's in this commit (found by the first hardening round).
- **Design:** §8.1's exit-code paragraph (D-01), `run` row and exit-check paragraph (D-02), `answer` row (D-04);
  §5's `gate_answered` row (D-05); §9.1's `rk-cli` row (`explain`, `init`, `catalog search`, `lint` — RA-M-02) and a
  `Config` note (RB-M-05 / D18-14); §8.4's unread row (N = 50 — D-20) and its boot-id row (RA-M-07).
- **Runbook and CI, also this commit's:** `s8`'s three `--lane … --item …` invocations re-spelled to the ADR-0128
  grammar and its two `explain --run … --stages` lines rewritten against the trailer log (D-02, D-03 — RT-33 records the
  shape); the `ci.yml` AC-529 scope comment (D-18) — a comment, the rule itself unchanged.

**Alternative named.** Accruing them to the close, as 7c did with RT-23's AC-465 note (rejected: the C-01 pass
exists so the stage file is written against corrected records; a stage file quoting a clause the same commit
knows to be wrong is the two-implementer hazard one commit later).

**Grounding.** DECISIONS-2026-09-15 D15-2; ADR-0092 clause 2.2 (design toward register) and its erratum rule; the
adjudication's *OWED* line per item.

---

## D18-19 · The six `s7b-ii` gate `doctor` rows are 7d's (taken 2026-09-19)

**Concern.** `s7b-ii` §7b-ii.14 discharged the kernel / store / policy halves of AC-290, AC-306, AC-310, AC-314, AC-530
and AC-455 and ARMED their `doctor` rows *"to 7d"*, but no §12 criterion carries those rows, so at `c709a59` they were
orphaned — `s7b-ii`'s own note: *"until `s7d`'s set carries them the rows are orphaned. Owed: an amendment adding both to
`s7d`'s acceptance set, or an erratum re-assigning them."* The stage file may not allocate a criterion to itself
(`s7a` §7a.11, *"naming a destination is not routing it there"*), so it built no row and raised **O-16**.

**Taken: allocate all six to 7d.** `s7d`'s acceptance set lists them **CROSS-LISTED** — the 7b-ii halves discharged
there, the `doctor` halves discharged here — with rows `RK-D-REVIEW-REEXEC` (AC-290), `RK-D-QUARANTINE-LIVE` (AC-306,
warning — a listing), `RK-D-SINK-COUNT` (AC-310), `RK-D-ADVISORY-LEDGER` (AC-530), `RK-D-CORPUS-SHA` (AC-314),
`RK-D-VERIFICATION-POLICY` (AC-455), built in Slice 4 with one rejecting fixture each in the AC-415 corpus; the
register-named floor grows twenty-three → twenty-nine and the table thirty-two → thirty-eight. The closed `s7b-ii` file
is not edited (R-21); its note stands as history and this entry is the amendment it asked for.

**Grounding.** Each criterion's Verified-by already names the instrument (*"a doctor rule …"*, *"a doctor rule-table
row"*, *"`redkiln doctor` errors when …"*, *"a `doctor` rule reporting the same at configuration time"*); only the owner
was missing, and `rk-doctor` is `rk-cli`'s (design §9.1, ADR-0099 clause 5). All six are reads over the event log or the
pack — data `doctor` already opens — with synthetic fixtures in the `unread/runs-49.jsonl` shape.

**Alternatives rejected.** Allocating but labelling ARMED until Step 8 (same build cost, weaker label, the AC-415 join
counts them anyway); deferring the rows to Step 8 (six orphans through M4 and an incomplete table when *"the system
runs"*); an erratum striking the `doctor` instrument from six criteria (weakens the register to remove an inconvenience).

## Routed at 7d — a value picked, a routing-table row owed in `s7d`, and a **record OWED** for each

Each row below is a **routing, not a decision** (`s7a` §7a.11: *"naming a destination is not routing it there"*).
The hardened stage file carries each in its `| Type | Home | Warrant | Status |` table or in the section that executes
it (RT-43 in §7d.1's prose, RT-48 in §7d.13, RT-51 in §7d.8's rule table, RT-52 in §7d.7, RT-53 in §7d.11), every one
re-listed in §7d.18 RAISED, with status *routed, not
decided — a record naming the home is OWED*, and Checkpoint 7d asserts none of them as decided. **RT-41…RT-59 are the
stage file's own routings** — RT-41…RT-46 from its first draft, RT-47…RT-59 from the hardening rounds — added here in the
stage-open commit so every `RT-` id a reader follows lands on one item.

| # | what is routed | the value taken at 7d | why this value, by clause | record OWED |
| --- | --- | --- | --- | --- |
| RT-28 | Where the migration `Report` schema lives and its shape | **`crates/redkiln-migrate/schema/report.schema.json`**, `additionalProperties: false`, `{corpus_sha, tool_version, per_kind_slot: [{kind, slot, lifted, lifted_with_loss, failed}]}`, **no `total`** | AC-448's *"the report lands in the same commit"*; AC-468's rejection of a total-only or folded form; E13's amendment placed AC-468 on the report-schema reading | a record fixing the report schema as the migration's contract (the register's phase note names it as Phase 1's) |
| RT-29 | The ADR-0063 question registry's file and type name | **`crates/rk-kernel/src/workflow/question.rs`**, `pub struct QuestionRegistry` with `Question { id, options: Vec<Option_ { id, attaches: Vec<Kind> }> }` and `QuestionRegistry::default_question()` | ADR-0129 decision 3 names the crate and the stage, not the module; the gate module (`workflow/gate.rs`) is its one consumer | ADR-0129's flip to `accepted`, and 7e's pack-schema record naming the TOML that declares a question |
| RT-30 | The lint-controls directory for 7d's source lints (the env-read grep, the stdin grep, the `rk_adapter_*` constructor ban in the doctor module) | **`crates/rk-cli/tests/fixtures/lint-controls/`** — one positive control file per ban, checked through the ban's own pipeline (`rk_control`); already inside `ci.yml`'s anchored `^crates/[^/]+/tests/fixtures/lint-controls/` exclusion (RT-13's shape), so no CI edit *(corrected 2026-09-18, same day: this read `crates/rk-cli/tests/lint-controls/`, "one positive and one negative control file per lint", "named in `ci.yml`" — a path the CI grep would not exclude, so a control carrying a banned token would have failed CI)* | golden constraint 12's *a checkpoint grep is not this*; a grep with no positive control is Halt 8 (g) | none — a fixture layout recorded in the stage-open commit |
| RT-31 | The AC-435 machine-half flags' names | **`--explain-sufficed yes\|no --toml-opened yes\|no`** as the defaults (two flags, one record field `explain_sufficiency` — D-05 as answered 2026-09-19); the implementer may rename; the stage file names the ones shipped and AC-435's Verified-by is amended to those names at close | ADR-0129 decision 1 names it as *the default the implementer may rename*; D-05's *"flag naming is the implementer's"* | the close-commit's dated note on AC-435 |
| RT-32 | The doctor rule-id vocabulary | **`RK-D-<NOUN>`** — `RK-D-GATE-OUTPUTS` (409), `RK-D-HOOKS` (412), `RK-D-STORE-ID` (413), `RK-D-ROLE-UNRESOLVED` / `-AMBIGUOUS` (416), `RK-D-CARD-TOOLS` / `-ABSENT` (417), `RK-D-TIER-LEAK` (418), `RK-D-TIMEOUT` (419), `RK-D-DANGLING-ROLE` / `-SLOT` / `-ATTACH` (420), `RK-D-DEAD-DEF` / `RK-D-UNFILLED-SLOT` (421), `RK-D-ENV-DENYLIST` (422/488), `RK-D-CLI-VERSION` / `-TTL` (423/526), `RK-D-FACET-UNKNOWN` / `-VALUE` / `-NEW` (424), `RK-D-FACET-FACT` (425), `RK-D-ADAPTER` (486), `RK-D-UNREAD` (426/472, warning) — one rejecting fixture directory per id under `tests/fixtures/doctor/<rule-id>/` | AC-415's join is over *the rule ids the implementer declares*; RA-M-01's floor needs the register-named rows enumerable by id; `rk-` is the prefix (DECISIONS-2026-09-11 #8) | a dated note on AC-415 naming the vocabulary, or a record if `doctor_rule_fired` is minted (O-12) |
| RT-33 | `s8`'s lines that the C-01 pass found on the losing side — the three `redkiln run --lane … --item …` invocations (D-02) and `explain --run "$RUN_ID" --stages` (D-03) | **re-spelled in the stage-open commit** (the amendments author's `s8` edit): `redkiln run "$ITEM" --lane fix` / `--lane story` (positional item, the flag kept because the fixture items' derivability is 7e's), `explain "$ITEM"` positional, and the stage-listing assertion moved to `git log --grep` over the `Story:` / `Slice-Verdict:` / `Gate:` trailer commits, since `explain <item>` prints the recorded resolution and has no `--stages`; each line carries its dated *was* comment | ADR-0128 decision 4; C-01 D-03's resolution (*"rewritten against a decided surface — the event log/store — rather than an explain mode"*); `s8` is an unopened stage file, so a dated in-place rewrite of its checkpoint lines before its first run is the D17-7 rule, not R-21's | none beyond the `s8` edit itself — ADR-0128 is the record |
| RT-34 | `Config`'s field set and whether the captured environment is one of them | **`ConfigFile { adapters, packs, doctor }`** (`rk-cli`, the `deny_unknown_fields` reader of D18-3) plus `Env` (the captured map, read once — AC-422's doctor input and AC-488's adapter input are the same map) — both inputs to constructing the `rk-kernel` `Config` of D18-14; `captured_env` is an `rk-cli` value handed to `AdapterSet::resolve`, not a kernel field *(corrected 2026-09-18, same day: this read `Config { adapters, packs, doctor, captured_env }`, one name for two types)* | RB-M-05 reading A; design §9.1's *"reads its environment in exactly one place … while building `Config`"* | the design §9.1 dated note (D18-14) |
| RT-35 | Where the `Exit` enum lives | **`crates/rk-cli/src/exit.rs`** — not `rk-types`, although AC-438's crate tag reads *`rk-cli`, `rk-types`* | `rk-types` owns `StepError`; the exit code is the CLI's surface (ADR-0118 clause 6's wording); `redkiln-migrate` has its own smaller table and shares no code with `rk-cli` (AC-441) | a dated note on AC-438's crate tag |
| RT-36 | AC-405's *"no tracker or web crate"* list | **a `deny.toml` ban list**: `octocrab`, `reqwest`, `hyper`, `axum`, `ureq`, `tokio-tungstenite`, `warp`, `actix-web` — the stage authors it; AC-405's amended Verified-by names the instrument | D-13: *"a `deny.toml` ban list of tracker/web crates the stage authors — instrument authoring, not a threshold"* | none — AC-405's amendment (D18-18) is the record |
| RT-37 | The migrator's porcelain read | **one call site**: `GitExec::run(GitOp::Status, wt, ["status", "--porcelain"])` in `run_contract.rs`; no helper added to `rk-git` | `rk-git` has no porcelain helper [measured]; adding one re-opens a closed stage's surface for one caller | none |
| RT-38 | The `phase: written` marker's channel | **stdout**, one line, exact bytes `phase: written\n`, printed after the last write and before `commit_with_trailers` | D18-12; a stderr line would interleave with `anyhow`'s error output on the kill path | none — recorded in D18-12 |
| RT-39 | The `ci.yml` edits the stage-open commit carries | **the AC-529 scope comment (D-18) alone** *(corrected 2026-09-18 before the stage-open commit: this also listed the AC-442 legacy-format grep with its allowlist and the AC-441 `cargo metadata` edge line; both land in Slice 5 with the code that makes them non-vacuous — `s7d` §7d.15)* | each names its criterion by id and quoted clause; RT-13's narrowing rule (a CI grep cites the clause it enforces) | none — CI rules recorded in the stage-open commit |
| RT-41 | The `reconcile_started` payload's `adapter_set` and `config_digest` members (AC-485's *"the adapter set recorded on the run-start event"*; design P3 names a run-start event no §5 row carries — `EventKind::ALL` is 38 with no `run_started`) | **routed onto the existing per-invocation `reconcile_started` row as two payload members**, written by the CLI (the row is the CLI's — `reconcile_at_entry` writes nothing); `ac485_…` reads the first row as the floor | ADR-0116 permits minting a kind by record only; design §5's row is *"invocation id, HEAD, the drift set that followed"* and is the nearest carrier | a dated design §5 note widening the row's payload, or a minted `run_started` kind by ADR-0116's route |
| RT-42 | `answerer` on `gate_answered` (design §5 as amended 2026-09-18: *"the implementer confirms this at Step 7d or the field is dropped"*; R-26) | **the author identity the wrapper stamps on the run-owned branch's commits**, read through `GitOp::Log --format=%an <%ae> -1` over `rk-run/<item>` **before** the `gate::answer` call — so the kernel can write it on the `gate_answered` row it builds (`AnswerMeta`, `s7d` §7d.6) — and asserted equal to the answer commit's author after the commit *(corrected 2026-09-18, same day, by the hardening round: this read "read back … on the sha `commit_with_trailers` returned", which is after the row the field sits on has been written)* — one op already on the closed `GitOp` enum, no `rk-git` edit | `GitOp` has no `Config`/`Var` read; the commit's author is the one identity the wrapper already records | the dated design §5 note confirming the source (or dropping the field) — R-26 |
| RT-43 | The order of an `answer`'s three reads | **reconcile, then read the gate record, then `tree_hash`** — a hash before the reconcile would be a hash of a tree the reconcile may report drifted against | ADR-0105 clause 4 (one hash algorithm); AC-406's void case is red on the other order | none — a test (`ac406_…`), not a record |
| RT-44 | What `doctor`'s stdout must carry, given E5 left it unpinned | **one line per evaluated rule beginning with its `RK-D-*` id token and a status word** (`ok` included) — an existence floor the block greps, not a schema | D18-6 as corrected; RA-M-01's floor needs the register-named ids readable off a clean repository | none beyond D18-6 — a pinned contract would be E5's rejected option B |
| RT-45 | The three `doctor` rows ADR-0105 clauses 6 and 7 name as Decision sentences and no §12 criterion carries (*"a `doctor` orphans row"*; *"the common dir is not on a network filesystem"*; *"a rule row for credential-shaped values in any retained field"*) | **`RK-D-WORKTREE-ORPHAN`, `RK-D-NETWORK-FS`, `RK-D-CREDENTIAL-RETAINED`** built to the accepted record in the same rule table, **claimed under no AC id**, outside the RA-M-01 floor | an accepted Decision sentence outranks the register's silence; a row a record names is built, a criterion a record lacks is not invented | a register criterion per row |
| RT-46 | Where a refused `run`'s one row goes (no pack; non-derivable lane; a lane whose pack is not enabled — the refusals reached after the reconcile; the AC-486 adapter refusal and AC-413's foreign store refuse before it and write nothing) | **exactly one `reconcile_started` row and nothing else** — D18-8 as corrected reads the same way | AC-172's *"exactly one `reconcile_started` event per invocation"* is the register's | none — D18-8's dated correction is the record |
| RT-47 | Where the store carries the run-owned worktree and branch a tier-1 write resolves *from the store* (ADR-0105 clause 8; AC-529; AC-518 — `s7d` H-5) | **`RunOwnership { worktree, branch }` on the invocation's `reconcile_started` row beside RT-41's members**; at 7d `run` re-creates `<common-dir>/redkiln/worktrees/<item>/` on `rk-run/<item>` (both `rk-` names, AC-531) and the `Gate:` commit goes through `rk_git::worktree::authorise_tier1_write`, never to `cwd` | the accepted clause says *from the store* and no table or §5 payload carries the pair [measured 2026-09-18]; the guard already ships in `worktree.rs` | a dated design §5 note, or the ADR-0116-route kind RT-41 may mint |
| RT-48 | The `stage` member on the synthetic unread fixtures' `artifact.write` / `artifact.read` payloads (`s7d` §7d.13) | **ADR-0116 decision 1's members (`kind`, `item`, `schema_version`, `digest`) plus `stage`** — the per-stage discriminator D-08's kind-per-stage predicate needs | no record's payload carries a stage; the fixtures are the rule's read set until 7b's emitter lands | a dated design §5 note widening the two rows' payloads |
| RT-49 | The two optional `[adapters]` keys `harness_binary` and `toolchain_programs` (`s7d` §7d.1's `ConfigFile` row) | **on the AC-474 key snapshot, not in the scaffold** — the one way a process-level test reaches `rk-fixture-child-claude` without the env toggle AC-474 forbids | the alternative — a `[dev]` table, or a compile-time `cfg(test)` path — is rejected because the shipped binary must be the one under test | a design §9.2 dated note naming the `[adapters]` key set |
| RT-50 | Where `run <item>` reads the item record | **`.redkiln/items/<item>.toml`** with a top-level `kind`, a top-level `stage` (the item's current stage — the `stage: &str` every kernel entry takes, which no fold or pack read yields; an absent key is exit 3 naming the item — added 2026-09-18 by the third hardening round) and a `[facets]` table — an existence floor the fixtures follow | ADR-0128 decision 2's *"the item record's kind where that record makes it derivable"* names the record and not its path | none beyond 7e's pack schema record, which re-points it |
| RT-51 | How `RK-D-HOOKS` recognises a redkiln-authored hook | **a `# redkiln-hook` line in the file's first 512 bytes**, matched at line start | no record names one because no Phase-1 verb writes a hook; the alternative — any non-`*.sample` file — fires on the operator's own hooks | a dated note on AC-412 |
| RT-52 | Which digest `explain` prints for `s8`'s digest-equality read | **both, labelled** — the `dag_assembled` row's inputs digest and each committed step's `output_digest`; **DECIDED 2026-09-19 (O-13 closed): the equality's subject is the committed steps' `output_digest`**, named in ADR-0128 decision 4 | printing both is harmless under AC-432's byte-identity; the inputs digest may legitimately differ across a resume that re-derives over a changed tree, a committed output may not | none — ADR-0128 decision 4 carries it |
| RT-53 | `redkiln-migrate`'s exit values (RT-35: *"its own smaller table"*) | **`0` success, dry run and the AC-450 no-op; `2` clap usage; `3` the AC-522 refusal and the AC-447 / AC-448 aborts; `1` any other `anyhow` error** | unpinned by any register criterion (AC-438 governs `redkiln`); asserted by `refuses_dirty_target` and the block's dry run | none unless a record asks for a snapshot |
| RT-54 | The pack loader's diagnostic mode | **`rk_kernel::catalog::load_diagnostic`** — every load error typed, `doctor` its only caller | AC-414's corpus of load-rejected fixtures must each yield their finding; the strict loader stops at the first error, and calling it per file would surface one defect of two | a record naming the diagnostic mode as the loader's surface |
| RT-55 | The node-id ↔ `Activity::id_at_version` join and the per-node spec supplier (`replay.rs`'s header: the spec is the binder's output and *"a record routing spec assembly is OWED"*; `spec_for` refuses a dispatched node with no `with_spec` binding; `ClaudeHarness::dispatch` refuses a `card_digest` its validated store does not hold) | **`rk-cli`'s `adapters.rs` binds every node the run will dispatch** through `Binder::new(config)` + `bind(BindRequest { … })` → `inputs.with_spec(node_id, RecordedSpec::supplied(..))`, then `ClaudeHarness::new(binder.into_config(), token)`; **`replay::resume` takes a spec-supplier seam** (`&mut dyn FnMut(&str) -> Result<RecordedSpec, ReplayError>`) consulted where `record()` calls `spec_for`, because node ids are known only inside the loop (`s7d` §7d.5 steps (8)–(9)) | without it every loop test is red on `inputs/specs.json` — the wrong reason — and the same twin resumes nothing; the fake ignores `card_digest` but `spec_for` does not | the record routing spec assembly (the join and the supplier) that `replay.rs` names as OWED |
| RT-56 | How `doctor` learns a store was rebuilt (AC-397's marker — its WHEN/THEN name none; `rk-store`'s schema carries no rebuild column or table) | **a sidecar file `<common-dir>/redkiln/rebuilt`**, written by `doctor::rebuild` after the rename with the rebuild's source `HEAD` sha, removed by the next successful `run`; the ONLY discriminator `RK-D-REBUILT-TIER2` reads (the rebuild's rows land in `t2_event`, the one table `query` reads, so *tier-2 emptiness* is not a table fact at this pin — `s7d` §7d.8) | `rk-store` is not edited at 7d; a table would be a closed-crate schema change | a dated design §5 / §9.2 note naming the marker (and the tier split it presupposes) |
| RT-57 | The `PackExtras` key floor — `margin_ticks`, the two TTL keys, `gate_deadline_secs`, per-activity `timeout_secs`, the gate-class expiry action | **`rk-cli`'s own serde reader over the SAME `.redkiln/packs/<pack>.toml` bytes**, `deny_unknown_fields` OFF (`s7d` §7d.5 step (7)) | `rk_kernel::select::Pack` exposes none of them and `toml_subset` ignores unknown keys; the kernel's `Pack` is not widened (no 7b re-open) | none beyond 7e's pack schema record, which re-points the floor |
| RT-58 | The report `kind` vocabulary's item rows beside the fourteen companion wire names (RT-28 fixes the schema's shape, not the column's value set) | **`initiative`, `project`, `story`, `spec`, `report`, `config`, `process`**, the two loss carriers under `report` and `spec` (`s7d` §7d.11 step (4), §7d.13) | the block's report check and two implementers must count the same rows | folded into RT-28's OWED schema record |
| RT-59 | The human-supplied root's carrier (ADR-0063: *"stored as a human-supplied root"*, naming no row; ADR-0116 decision 1 fixes `artifact.write` as *emitted by every activity output*) | **an `artifact.write` row written by `answer`** for an admitted attachment, in ADR-0116 decision 1's payload shape, the binding path as the instance name inside `kind`'s slot (`s7d` §7d.6) — `answer` is a verb, not an activity output, so the row's **producer is widened** under this routing | the kind exists and nothing on the tree writes it yet; a widened payload would be schema drift (ADR-0116 decision 5) and is not written | a dated design §5 note or an ADR-0116-route record naming the producer (beside any payload widening) |
| RT-40 | Which of the two toolchain adapters the one `Ports.toolchain` slot holds at 7d (R-8 / RT-7, ARMED from 7c) | **no composite at 7d** — `Ports.toolchain` holds the Rust adapter on this workspace; the Node adapter is constructed for the `doctor` adapter row only; AC-350 stays ARMED (R-8) *(withdrawn as a routing 2026-09-18, same day: this read "the composition root composes a `SelectingToolchain` … `Ports` is unchanged" — a composite is a new type no record names, `s7c` RT-7 says the record naming it is OWED (R-8), and a routing may not mint the type the owed record is for)* | R-8: a `Ports` change re-opens Checkpoint 7a; AC-350's *"one verify result per claiming toolchain"* needs both reachable | the composite record R-8 already owes, **before** any composite type is written |

---

## OPEN — the operator answers each before the slice that needs it

Each carries a **recommended default**, grounded, and the **slice** that cannot be cut without the answer. None
is decided by this file.

| # | the question | recommended default | why | needed before |
| --- | --- | --- | --- | --- |
| O-12 | **SM-123's instrument** is a `doctor_rule_fired` event with `rule_id` and `severity`; the closed `EventKind` table (38 at `239e899`) has no doctor kind and ADR-0116 permits minting one *by record or not at all*; the stdout fallback has no pinned schema (E5, D18-6). C-01 D-19 **ANSWERED 2026-09-19: deferred as recommended** — `doctor_rule_fired` is minted by an ADR in ADR-0116's form (tier-2, `{rule_id, severity, subject}`) when a stage first needs the measure (Step 8's SM readout); 7d claims nothing on SM-123 and prints one line per evaluated rule (RT-32). The kept-vs-reverted join SM-123 also needs has no record naming what *"the resulting configuration change"* is and is raised with the kind. Rejected: minting now as ADR-0130 (a source with no join is half a measure, one more kernel kind at a stage carrying eight kernel edits); re-pointing SM-123 at `doctor --json` (needs the stdout contract E5 declined). | **Mint `doctor_rule_fired` by record** — an ADR in ADR-0116's form adding a tier-2 kind `{rule_id, severity, subject}` to design §5's table, written when a stage first needs the measure (Step 8's SM readout), not at 7d; the alternative — an SM amendment re-pointing SM-123 at `doctor --json` — needs the stdout contract E5 declined | a measure whose instrument exists in no table has no source; a kind is the cheaper record and lands on the route 0120/0123/0124/0125/0126 took | Step 8's SM readout — **7d claims nothing on SM-123** |
| O-13 | **E5's owed ADR-0107 erratum** — decision 4 asserts *"digest equality via `redkiln explain <item>`"* and names **no field**; ADR-0128 decision 4 keeps that sentence and inherits the gap **ANSWERED 2026-09-19:** each committed step's `output_digest` in the recorded resolution, **named in ADR-0128 decision 4** (its decision 5 widened by that clause before acceptance) — not an ERRATA entry, which ADR-0092 :64/:140 would forbid on a Decision sentence. E5's owed item is closed by ADR-0128, not by an erratum. | an `ERRATA.md` entry (erratum-grade — a Decision sentence's *instrument*, not its rule) naming the field as the recorded resolution's `output_digest` of the last committed step, the value AC-432's byte-identity test already compares | E5 records it as owed; `explain` prints the recorded resolution only (D-03), so the digest it prints is the resolution's | the `explain` slice (Slice 3) — the test that asserts digest equality needs the field named |
| O-14 | **AC-476, the migration-commit human gate** — *"a human answers a gate whose question is whether the recorded losses … are acceptable"*; the 153-line file claimed it in its v0 set, but v0 opens no gate and a gate question needs the ADR-0129 registry **ANSWERED 2026-09-19: NOT split** — 7d claims nothing on AC-476; the AND clause is instrumented under AC-522 (`ac447_exactly_one_commit_on_migration_branch` / `ac522_…`: the commit lands on `rk-migrate/<short>`, never the target), a dated note on AC-476's Verified-by says so, and the gate is the first landing stage's. Rejected: an AC-435-style split (a HUMAN GATE id claimed by half where the register names no half, duplicating AC-522's instrument); moving the AND clause into AC-522 (already there in substance). | **7d claims AC-476 as machine-half only**: the migrator writes the differential report the gate reads and never lands the commit on a machine verdict (the branch is `rk-migrate/…`, never the target); the gate itself is `s8`'s or 7e's once a pack declares the question | AC-476 is HUMAN GATE by its own Verified-by; the machine-decidable half is *"the commit does not land on a machine verdict alone"* | the acceptance-set partition (stage-open) — where AC-476 is placed in Excluded with no half claimed; the behaviour that would satisfy the AND clause is built and labelled this default until the operator answers (`s7d` §7d.11, §7d.18) |
| O-16 | **The `doctor` rows `s7b-ii` §7b-ii.14 ARMED to 7d that no §12 criterion carries** — AC-290, AC-306, AC-310, AC-314 and AC-530's rows, and AC-455's *"a `doctor` rule reporting the same at configuration time"*; `s7b-ii`: *"until `s7d`'s set carries them the rows are orphaned. Owed: an amendment adding both to `s7d`'s acceptance set, or an erratum re-assigning them"* **ANSWERED 2026-09-19: all six allocated to 7d — D18-19**; six rows, six fixtures, Slice 4; floor 23 → 29. | **an amendment adding them to `s7d`'s set with their `RK-D-*` rows**, or an erratum re-assigning them — the stage file may not perform the allocation itself (`s7a` §7a.11) and builds no row for them until the word lands | *naming a destination is not routing it there*; a row built under no criterion would be RT-45's shape without RT-45's Decision sentence behind it | Slice 4 (`doctor`) — until then the rows are RAISED, not built, not claimed |
| O-15 | **Does the product repo get its own `init` at 7e** — Checkpoint 7d never touches it (D18-9); `.redkiln/` does not exist at HEAD; 7e installs two packs under `.redkiln/packs/` and fills `[packs] enabled` **ANSWERED 2026-09-19: yes, at 7e** — `redkiln init` run once at the product root in the commit that installs the two packs and fills `[packs] enabled`; the scaffold is committed with something in it; the store stays untracked by construction; 7e's stage-open records it. Rejected: an init-only scaffold at 7d's close (an empty-pack config in the product tree for a stage that cannot run against it); hand-authoring `config.toml` at 7e (bypasses the verb AC-437/AC-531 pin, on the one repository that matters). | **Yes, at 7e**, as the commit that fills `[packs] enabled` — `redkiln init` run once at the product root, its scaffold committed with the two packs; the store at `<common-dir>/redkiln/store.sqlite3` is **not** tracked (it is outside the working tree by construction) | E6 (*"`init` scaffolds it at Step 7d; Step 7e fills it"*) — the scaffold's first appearance in the product repo is the commit that has something to put in it | Step 7e's stage-open |

---

## Residuals raised at 7d and not settled here (R-25…)

| # | what is owed | raised by |
| --- | --- | --- |
| R-25 | **AC-411 is claimed by no stage** (`grep -rn AC-411 runbook/` → 0 before this pass) and is `rk-store` only — a store test mutating a structured section → `structured_drift` + restored render, plus a lint banning markdown-to-structure parsers in `rk-store`. 7d's acceptance set lists it with the disposition *rk-store, new here* (D-22) because the migrator's round trip (AC-470) is its first real consumer; its **C2-vs-AC-478** question (whether `schema_version` stamping is a structured section) stays open. *(Noted 2026-09-18, same day, by the first hardening round: `rk-store` has **no render path at this pin** — no `fn render` under `crates/rk-store/src`; `locator` is the one module that reads markdown — so "new here" would mean building the store's render path inside a stage whose closed-crate rule forbids it; `s7d` RAISES AC-411, and AC-478 and AC-477's render half with it, for an owner record and claims none by proximity.)* | C-01 D-22; the decidability table's AC-411 row |
| R-26 | **Design §5's `gate_answered` row names an `answerer`** the kernel never writes and whose source (git identity, OS user, a flag) no record names. The dated amendment for D-05 adds the AC-435 field; whether `answerer` gets a source or is dropped is stated by the 7d stage file for its build and **owed a record** **CLOSED 2026-09-19:** the operator confirmed the routed source (RT-42 — the git author identity of the run-owned branch's `HEAD`, read through the `rk-git` wrapper before the `gate::answer` call); the field stays; design §5's row carries the dated confirmation. | C-01 D-05 / RB-M-08 |
| R-27 | **AC-421's warning vs AC-102's threshold** — an unreferenced skill is a warning (AC-057) and a threshold rule (AC-102) may make it an error; register §0.3 lists the interaction unresolved; the `RK-D-DEAD-DEF` row ships at warning severity and says so | the decidability table's AC-421 row |
| R-28 | **AC-445 is ARMED-local**: its corpus half needs the 1,312-file predecessor tree, which reaches this box only as Step 2's local-only suite; D18-10's `PROVENANCE.md` is the Phase-1 evidence that no predecessor byte is tracked, not a discharge | D18-10; AC-465 |
| R-29 | **SM-128's second half** (*"structured writes not originating from `rk-cli`"*) has no writer-identity source on any event; the AC-405 / AC-411 lints are proxies. A record naming a writer-identity field, or an SM amendment narrowing the measure, is owed | the decidability table's SM-128 row |
| R-30 | **SM-125 and SM-126 need production answers**: both instruments exist at 7d (`gate_answer_void` / `gate_answered`; the AC-435 field) and neither rate has an n before `s8` at the earliest; 7d asserts the instruments' existence and reports the rates as **not measurable** | the decidability table's SM-125 / SM-126 rows; golden constraint 10 |
| R-31 | **AC-438's evidence line names *"SIGTERM → 143"*** (*outside the brief*) while the table (D18-4) maps `Cancelled` to 13 and a signal outside a dispatch to 130. The snapshot is the record; the evidence line is stale as a hint and takes a dated note when AC-438 is next amended — it was not folded into the D-01 amendment | D18-4 |
| R-32 | **DECISIONS-2026-09-11 #3 lists *"five plus `ledger`"*** (`_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger`) while `rk_types::companion::CompanionKind` has **six** variants and `PHASE_2_KINDS` eight. The migrator lifts the six the type declares; the decisions log's *"five plus `ledger`"* **is** six, which the type spells as six variants (`LedgerCompanion` for the `_ledger` sidecar, `Ledger` for the census's bare kind); the phrasing is **noted, not relitigated** (the plan's §A item 8 read it as five) | the plan of 2026-09-18 §A.8; `crates/rk-types/src/companion.rs` |
| R-33 | **AC-427's fidelity** is `s8`'s (`rebuild_differential` / `stage_level_next_action` over the local corpus, owner AC-184) and **SM-124's *"CI job over the `redkiln-corpus` pin"* cannot run in CI** — `CORPUS_PIN` names `D:/repos/redkiln-corpus`, a local path. The SM-124 amendment (D18-18) reads it as the local-only suite until a fetchable pin exists; a fetchable pin is nobody's yet | C-01 D-21 |
| R-34 | **The `s7c` split notes and ARMED rows naming 7d** (R-11: AC-417 / 422 / 423 / 428 / 439 / 488 / 344; the doctor halves of AC-203 / 205 / 212 / 220 / 226 / 369 / 370 / 377 / 383 / 422 / 423 / 493 / 526; AC-366 / 397 / 395's rebuild; AC-269's build-config half; AC-278's gate path; AC-350 (R-8); AC-361's cost rollup (RT-19)) each need a `§7d.22` disposition — *discharged / ARMED on / excluded* — which the stage file gives and this log does not; **AC-439's *replay across the recorded stream corpus* conjunct** stays unsatisfiable at the pin (no stream corpus exists) and is excluded by name | DECISIONS-2026-09-17 R-11; `s7c` §7c.22 |
| R-35 | **External anchors into `s7d` are by LINE — three**, all in `s7a` §7a.9's AC-485 / AC-486 / AC-488 rows, each reading *"claimed by id at `s7d`:117"* [measured 2026-09-18 by `grep -rn -oE 's7d\`?:[0-9]+' runbook/`; `PROGRESS.md` §5's C-01 row says *"noted in s7d"* with no line]. Line 117 of the 153-line file is the acceptance-set sentence naming those three ids; it drifts when the file is rewritten (D18-1). On R-21's rule the closed `s7a` is not edited by this stage's commit: each anchor reads against the pre-hardening pin `239e899` (`git show 239e899:runbook/phase-1/s7d-cli-and-migrate.md`) and is re-anchored to the `§7d.N` acceptance-set heading by symbol when `s7a` is next opened. `s7d`'s own outward anchors are by `§N.M` from its first hardened line | R-21; `s7a` §7a.9 |
| R-36 | **The two `proposed` records** — ADR-0128 and ADR-0129 — are committed `proposed` at stage-open on ADR-0126's precedent and flip to `accepted` by a follow-up commit once the operator's dated confirmation line is written under D18-17; until then the stage file's grammar is the record's reading and not the operator's word, and Checkpoint 7d's AC-400 snapshot line is written against the proposed grammar **CLOSED 2026-09-19** — the dated line is written under D18-17 and the flip commit carries both records `accepted`. | D18-17; ADR-0092 (an accepted record outranks this log) |

---

## What these decisions unblock

D18-1 is the order of work. D18-2, D18-3 and the dependency rows unblock Slice 1's manifest (`clap` 4.6.7, `anyhow`
1.0.104, `toml` 1.1.6, `ctrlc` 3.5.2, `trycmd` 1.2.1, `assert_cmd` 2.2.2 — all ≥ 1.0, caret rows with a comment,
measured 2026-09-18 by `cargo search`; **the root `Cargo.toml` rows land in Slice 1, not at stage-open**, so
Checkpoint 5 goes provisional in that slice's commit and not in this one). D18-4 unblocks `exit.rs` and every test
that asserts a code. D18-5 through D18-9 unblock the doctor and init slices and Checkpoint 7d's temp-repo prelude.
D18-10 through D18-12 unblock the migrator. D18-13 unblocks the surface slice. D18-14 and D18-15 name two of the
kernel edits (a `Config` type; a `keywords` field and `catalog::search`); ADR-0129 decision 3 adds the question registry,
and the stage file adds `catalog::load_diagnostic` (RT-54), the `gate::answer` arguments, `replay::resume`, the strict
loader's AC-409 refusal and `pub const LOCK_HALTS` — `s7d` §7d.15 is the authoritative list (eight), and each edit marks
`rk-kernel` as a Step-7b crate touched at 7d with 7b/7b-ii re-run. D18-16
fixes the snapshot's doctor rows. D18-17 (ADR-0128, ADR-0129, the eager `init`, gate-open exits 0) unblocks the
router, `answer`, `init` and AC-438's table. D18-18 means the stage file quotes corrected clauses.

**Still owed after this pass, and named so nobody rediscovers them:** the routing records this table owes, by row —
**RT-28, RT-29, RT-31, RT-32, RT-34, RT-35, RT-41, RT-42, RT-45, RT-47, RT-48, RT-49, RT-51, RT-54, RT-55, RT-56, RT-59**
(seventeen; RT-30, RT-33, RT-36, RT-37, RT-38, RT-39, RT-43, RT-44, RT-46, RT-50, RT-52, RT-53, RT-57 and RT-58 owe none
beyond the edits or the records they name, and RT-40 is withdrawn — its record is R-8's); the two record flips **ADR-0128 → `accepted`** and **ADR-0129 → `accepted`** on the
operator's dated line (R-36 — **done 2026-09-19**); **O-12** (answered 2026-09-19 — deferred: the kind is minted by ADR when Step 8 first needs the measure), **O-13** (closed 2026-09-19 — ADR-0128 decision 4 names the digest), **O-14** (closed 2026-09-19 — not split; AC-522 instruments the AND clause), **O-15** (answered 2026-09-19 — yes, at 7e, in the commit that fills `[packs] enabled`) (the product repo's `init`) and
**O-16** (closed 2026-09-19 — D18-19 allocates the six rows here), each answered before the slice or stage named; and **R-25…R-36**. The
amendments D18-18 lists are **not** owed: the stage-open commit carries them. Nothing in this file claims a criterion
discharged; that is the checkpoint's to say, and only after it runs.
