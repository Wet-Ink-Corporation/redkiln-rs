# Step 7c — The adapters (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7c` | Phase 1 | Checkpoint 7b-ii | Checkpoint 7c | `s7b-ii` | `s7d` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this file was rewritten before its first run.** Until 2026-09-17 this stage was 211 lines: eight prose
paragraphs, five red-first bullets against nine checkpoint filters, one acceptance paragraph claiming
**AC-200–AC-239 and AC-360–AC-399 by bare range**, and a checkpoint of nine `rk_test` lines whose match was an
aggregate `test result: ok` string. It was hardened in the `s7b-ii` shape **before any code was cut**
(DECISIONS-2026-09-17 D17-7; `00-preamble.md`'s maintenance rule — *"a pass is evidence about a specific text, not
about a stage"* — has no pass to make provisional here, which is the only cheap moment to do this). What the 211-line form
could not catch, per checkpoint group: **the build** — `cargo build -p <four>` was the only gate, against
`halts.md` Halt 1's *"`cargo build --workspace` from Step 5 on"*, and no fmt / clippy / workspace-test / audit / deny
line, so a lock move that `deny` refuses went unseen; **the named tests** — `rk_test` grepped `test result: ok\.
[1-9][0-9]* passed`, the substring-filter defect every hardened block since Checkpoint 6 replaced with
`^test .*\b${t}\b.* \.\.\. ok$`, so a near-miss name or a filter matching nothing passed; **the bans** —
`rk_absent 'set_var|remove_var'` and `rk_absent 'unwrap_or\(0\)|unwrap_or_default\(\)'` searched **all of
`crates/`** with no comment filter and no positive control, and both **fail today on the untouched tree** (2 doc-comment
hits; 32 hits in `rk-git`/`rk-kernel`/`rk-types`) [measured 2026-09-17] — a guard that fires on a comment is fixed by a
comment filter with a control, never dropped (Halt 7); **the derive loop** — `printf | while read … exit 1` exits a
pipeline subshell and never stops the block, then prints `PASS` unconditionally (`halts.md` shape (c)), and it
searched `crates/rk-adapter-claude/src/` for `struct Usage` when the record puts `Usage` in `rk-types` (§7c.1); **the
fifteenth line** — *"one real dispatch"* had no command, no record path, no `test -s`, no instrument capture, and no
executable existed at 7c to perform it; **the acceptance set** — the bare ranges swept in seven PROVISIONAL or
human-gated ids, **twenty** kernel- or types-owned ids (the fifteen in §7c.22's kernel-owned bullet plus AC-361/AC-367/
AC-387 in the ARMED table and AC-373/AC-382 in the ledger — counted off §7c.22 by `grep -o 'AC-[0-9]\{3\}'`
2026-09-17), three `doctor` ids and one id built and claimed at 7b-ii (AC-239), and omitted **twenty-eight** ids other
records route here (the list §7c.22's opening paragraph carries, counted the same way). Every one of those is named
below in the partition it belongs to. **The record this rewrite executes is `spikes/DECISIONS-2026-09-17.md`** — the
seven decisions **D17-5…D17-11**, the routings **RT-1…RT-26** (nine of them, RT-18…RT-26, added on the three
review rounds of the same day), the open questions **O-1…O-11** and the residuals **R-7…R-22**, cited here by those ids; the reader map, the critique and the operator's answers it was distilled from
(`7c-map.json`, `7c-critic.json`, `7c-operator-decisions.md`, 2026-09-17) are working notes in the session
scratchpad, not in the tree, and nothing below cites them as a record. The critic's amendments win over the map where
the two disagree, and the seven decisions bind both.

---

> ### 🚦 GATE CHECK
> ```bash
> ( grep -qE '^\| 7b-ii — The three gates \|.*checkpoint 7b-ii passed `164e212`' D:/repos/redkiln-rs/runbook/PROGRESS.md \
>     || { echo "GATE OPEN — PROGRESS.md row 7b-ii does not read 'passed 164e212'. Go back to Step 7b-ii; no work in this stage may start."; exit 1; }
>   echo "GATE DISCHARGED — proceed" )   # a subshell: a pasted FAIL must not kill an interactive shell
> ```
> **Three golden constraints reach this stage by name** (`00-preamble.md`'s *"Where each constraint actually bites"*
> table names Step 7c on rows 2, 4 and 12). **Constraint 2 — nothing may depend on a process surviving:** the live
> probe (§7c.15) runs in the **foreground**, operator present, and its one wait — the `$(cargo test …)` capture that
> blocks until libtest returns — is **bounded by the probe card's `timeout_secs`**, whose expiry is the **adapter's
> own timeout arm** — a `tokio::time::timeout` race inside `dispatch`, never a firing of the shared `CancelToken`,
> which has no timeout arm and no cause (§7c.10) — and that arm kills the child tree through the group; it is the
> failure alternative Halt 5 asks for, and the block names the bound on the line (§7c.15). There is no grep-shaped
> waiter in this stage's block. A background job on this box dies at ~35 min (`HANDOFF`), and `issue-81` was killed
> three times as a background job before completing in 116 s in the foreground. **The fence is pasted whole into a
> Git Bash the operator opened — never a Claude Code tool shell — started as `env -u ANTHROPIC_FOUNDRY_API_KEY bash`;
> the block's first preflights refuse any other shell before a single gate runs** (§7c.21). **Constraint 4 — a rig that
> compiles and passes its unit tests has NOT been run:** three E3 defects survived "built and validated" because
> none touched the filesystem or the CLI; Checkpoint 7c's last group is a real dispatch, validated before any set,
> with a haiku probe. **Constraint 12 — test-first, one discriminating test per criterion:** §7c.18 carries one
> red-first bullet per checkpoint test name; the checkpoint is the floor, not the evidence.
>
> **And the standing prohibition on what this stage's close-out may say.** **ADR-0101 clause 5**: *"Until it runs,
> no document may claim the review gate is guarded against false approval, and no FP rate from S-VAR or S-TYPED is
> evidence of overall calibration."* AC-312's framing-variant fixtures are **authored here and left unrun** (§7c.16);
> Checkpoint 7c's pass says **the adapters exist, the real rows conform, and one dispatch ran** — not in
> `PROGRESS.md`, not in a commit message, not in a milestone row may it be written up as more.

### 7c.1 What this step builds, and what it may not decide

**Six crates, not four** — by DECISIONS-2026-09-17 D17-10, recorded as **ADR-0127** (written before stage-open on
2026-09-17 and committed in the stage-open commit beside ADR-0126, the ADR README's count and ADR-0099's
`superseded_by`; its **code** lands at Slice 2, §7c.20): `rk-adapter-sqlite` (`EventStore`), `rk-adapter-claude` (`Harness`), `rk-adapter-toolchain-node` and
`rk-adapter-toolchain-rust` (`Toolchain`; ADR-0094 decision 1 — *"Phase 1 ships four real adapters … The crate is
`rk-adapter-toolchain-rust`"*), and **`rk-containment`**, which owns the one commented `#![allow(unsafe_code)]` module
(the Windows Job Object), the POSIX `process_group(0)` + `killpg` arm, and `run_bounded(cmd, timeout)` for the
token-less toolchain adapters. ADR-0127 amends design §9.1's crate list with a sixth Phase-1 crate and supersedes
**ADR-0099 decision 3 in part** — the clause locating the unsafe module *"in one module of `rk-adapter-claude`"* —
and nothing else in ADR-0099. Design §9.1 remains the crate-name authority under ADR-0099 decision 5, so the names are
these and not `rk-adapter-node`. The sixth deliverable is the set of `rk-types`/`rk-ports`/`rk-store` **edits** that
re-open Checkpoints 5, 6 and 7a (§7c.19). One crate per adapter, each owning its runtime handle, constructed once at
the composition root and never per call (design §9.1's adapter row).

**The record for the Harness adapter's shape is ADR-0099** (`dispatch(&self, spec: &ActivitySpec, store: &mut dyn
EventStore) -> DispatchOutcome`; *"`dispatch` blocks to completion"*; the `CancelToken` *"constructed at the root,
cloned into the adapter at construction (never per call)"*; *"The kernel never calls cancel."*), **ADR-0095 + ADR-0112**
for the closed **eleven**-variant `StepError` (the register's AC-229 text still says *"eight"* — stale; the ADR
governs; `PROGRESS.md` §1's Step 5 row and §6's *"Owed out of Step 5"* row record the erratum as owed), **ADR-0108** for the configuration-time `StructuredOutput` check, **ADR-0110**
for headroom in ticks, **ADR-0113** for the five TTL states, **ADR-0078** for the TTL pair, **ADR-0010** for
subscription-only auth, **ADR-0083** for configuration as a value, **ADR-0076** for async confined behind sync port
methods, **ADR-0102** for the lint policy (one `allow`, everything else `#[expect]`), **ADR-0118** for the poisoned
lock, **ADR-0105** for the store's SQLite rules and the digest-only record, **ADR-0122/0123/0106** for retries, resume
and cost, and **ADR-0094** for the Rust adapter and the two conformance reals. Read each before its section.

**What this step may not decide** (`00-preamble.md`: *"Where a stage is tempted to decide something else, it
stops."*):

- **Nothing about the crate layout beyond design §9.1 and ADR-0127.** Module file names inside a crate this stage
  creates are the implementer's (as `s7b` §7b.4 held for the kernel); the lift table in §7c.2 names files so the
  checkpoint can name them, and that is a table, not a record.
- **Nothing that lands a schema in a closed crate without disposing of the pass it disturbs.** `rk-types` gains
  `DispatchRecord` fields and a `usage` module (ADR-0126, §7c.11); `rk-ports` gains a `conformance` module behind a
  feature and `PHASE_1_REAL_ROWS` filled (§7c.13); `rk-store`'s duplicate `retention::DispatchRecord` collapses onto
  `rk-types`'. Each edit marks that crate's row `passed <hash>, re-run pending 7c <crate>` in `PROGRESS.md` §1 **in the
  same commit** and re-runs Checkpoint 5, 6 or 7a there (`00-preamble.md`: *"A checkpoint that changed after it passed
  is provisional"*; precedent `4be8987`).
- **Nothing about where a criterion is allocated.** Where an id is orphaned, double-claimed or mis-routed, this file
  **raises it and does not settle it** (`s7a` §7a.11: *"naming a destination is not routing it there"*). §7c.22 names
  each one.

**The routing table.** Each row below was **routed, not decided** — the operator chose a working shape so the build
can proceed, and each open row owes a record. Every row carries its DECISIONS-2026-09-17 id (`D17-n` decided, `RT-n`
routed, `O-n` open) or names the source that has none. **The checkpoint asserts the DECIDED rows** (`usage.rs`
exists, `ALL` is 38, `TESTED_CLI_VERSION` equals the pin) **and polices routed rows only as existence and shape
floors that move with the record** — every one of them named here so no reader takes a green line as a decision:
RT-15's tokio graph (a ban), RT-2's `f64` absence (a ban), RT-13's `Command::new("git")` grep and its
`tokio::process::Command` ban (bans), RT-14's rename (two `test -d` lines), RT-11's `test_timeout` and `gate_record`
keys in the Rust pack fixture (two syntax-anchored greps — the slot's *spelling* is 7e's and the lines are re-pointed
when the pack schema record spells it), RT-16's `PHASE_1_REAL_ROWS` location, four-row count and non-empty
assertion lists (three lines re-pointed if a record moves the rows), RT-1's cap through `argv_cap_refuses_before_spawn`
(the test pins the routed value as a floor), RT-21's worktree key through `dispatch_cwd_is_the_spec_worktree_never_process_cwd`,
RT-22's contract through `port_conformance_real_row_rust`, RT-24's four byte-equal `enum Capability` bodies (a
digest comparison), RT-25's three tracker names (a shell guard) and RT-26's `--list` route (an anchored grep over
the Rust adapter's source). A record that overturns one of those routings makes
its floor line **stale, not wrong**: the line is re-pointed in the same commit as the record's code, the way
Checkpoint 5's `ChangeSummary` line was (D17-1), and the block never reads a routed value as decided.

| Type | Home | Warrant | Status |
| --- | --- | --- | --- |
| `Usage`, `CacheCreation`, `WindowReading` | `rk-types` (`crates/rk-types/src/usage.rs`) | ADR-0099 decision 1 makes `DispatchRecord` `rk-types`'; `s7a` §7a.5's table says *decided by the record*; `usage`/`timings` are its fields | **DECIDED by ADR-0126** (D17-8). Only `Margins` is adapter-side |
| The usage-bearing event kind (AC-360) | `EventKind::DispatchCompleted`, wire `dispatch_completed`, tier 2 | ADR-0120/0123/0124/0125's route: the closed table gains one kind named by a criterion | **DECIDED by ADR-0126** (D17-8), written before stage-open; its `rk-types` code lands at Slice 1; supersedes nothing |
| The `api_retry` count channel (AC-232) | `DispatchRecord.api_retry_events: u32` on the same record | ADR-0122 decision 6 mints no per-attempt kind; the count is a record field | Routed by ADR-0126; **a record naming the kernel wiring of AC-026's charge is OWED (R-22** — DECISIONS-2026-09-17; not DECISIONS-2026-09-15's R-2, which is ADR-0122 decision 5(c)'s AC-391 clarification and was mis-cited here before the 2026-09-17 review**)** |
| `DispatchRecord`'s two TTL fields (AC-210) | non-optional members on the `dispatch_completed` payload | AC-210 THEN: *"both values are recorded on the dispatch event"*; ADR-0126 decision 1's payload table names them | **Carried on ADR-0126 decision 1's table; nothing owed** (D17-8's dated amendment note in DECISIONS-2026-09-17 records that the record's first draft omitted them and the committed text does not) |
| The raw `system/init` the probe retains | **the probe's own output file only** (`RECORD_PATH`, §7c.15) — **not** a `DispatchRecord` member | no record names an `init_raw` member (0 hits over `references/` and `spikes/*.md` [measured 2026-09-17]); ADR-0126 decision 1 fixes the payload's names | **RT-18**; routed away from the record; **a record naming raw-init retention (cap, digest, tier) is OWED** if any consumer wants it on the row |
| `DispatchOutcome` | `rk-ports` (unchanged since 7a) | ADR-0099 decision 1; O-11 | stands; not moved (O-11's default) |
| `Snapshot(pub(crate) ())`, `ChangeSummary(pub(crate) ())` | `rk-ports`, unchanged; real adapters return `Err(StepError::Unsupported { capability })` from a dated registry entry | **D17-9**; AC-238/489/492; Checkpoint 7a's six-method count stands | Decided as to the port; **a record naming the `Snapshot` contents is OWED** (`s7a` §7a.5); `ChangeSummary` waits on S-BIN |
| The argv cap constant | `rk-adapter-claude::command::ARGV_CAP_BYTES = 32_767` | Windows `CreateProcess`'s documented command-line limit [documented]; DECISIONS-2026-09-11 #18 fixes the cap's existence, not its value | **RT-1**; **a record naming the value is OWED** |
| AC-374's decimal digits | a raw-bytes digit scanner over `StreamLine.raw`, into `u16` hundredths | `serde_json` without `arbitrary_precision` hands a visitor an `f64` (critic gap); enabling the feature unifies workspace-wide and moves `rk-types` snapshots. **The named alternative:** `serde_json`'s `raw_value` feature (`Box<RawValue>` on the `utilization` field yields the exact decimal bytes and, unlike `arbitrary_precision`, changes no `Number` representation workspace-wide — snapshots unaffected); **it is named and NOT taken**, because the scanner is the operator's routing of 2026-09-17 and a review may not re-route it (RT-16's lesson, above); it is the alternative the OWED record weighs | **RT-2**; **a record naming the parse path is OWED**; the scanner's contract — a structural scan, not a substring search — is fixed in §7c.8; the `f64` ban covers `headroom`, `throttle`, `event` and the cost scanner (§7c.21) |
| `total_cost_usd`'s type | `Option<CostUsd>` — **`rk_types::usage::CostUsd(String)`, the decimal digits verbatim as the CLI printed them**, taken from the raw result bytes by the same scanner AC-374 uses for ticks, **never an `f64`** and never a fixed-scale integer | `rk_types::DispatchRecord` derives `Eq` (`crates/rk-types/src/activity.rs`) and an `Option<f64>` member does not compile under it; `rk-store`'s canonical serialiser **refuses floats** (`canon.rs`: *"Floats are refused"*), so a float in the `dispatch_completed` payload passes no canonical comparison; and a `u64` micro-dollar scale (this row's first form, `CostMicros`) would truncate or round a value with more than six decimals — the recorded F20 value `2.337147` is the runner's `f64` re-serialisation, so **the CLI's digit count is UNCONFIRMED** at the pin and a fixed scale cannot be chosen honestly | **RT-19 as amended 2026-09-17 on the third review** (added to DECISIONS-2026-09-17 on the first); the cost scanner lives in `headroom.rs` beside the digit scanner it shares, so the AC-374 `f64` ban's module filter reaches it (§7c.21); **a record naming the cost unit and any arithmetic form is OWED** (ADR-0126 decision 6's typed-payload record; the rollup that needs a number is AC-361's, 7d's) |
| The expired per-activity timeout's variant | **`StepError::Cancelled { by: CancelledBy::Timeout }`**, the cause on the `dispatch_completed` row | ADR-0099 decision 2: *"triggered by exactly two owners: the CLI's signal path (Ctrl-C / SIGTERM) and the per-activity timeout … When the token fires, the adapter kills the child tree, and `dispatch` returns `Err(StepError::Cancelled { by })`"*; AC-225's THEN names `by: timeout`; ADR-0095's table carries **both** variants, so no accepted record reads the other way | **RT-3 as amended 2026-09-17** (the routed default is the record's reading); `Timeout { after_secs }` with ADR-0095's retry-once class is the **raised alternative**; **a record settling which path, if any, produces `Timeout` is OWED**. **The mechanism and the `by` rule travel with this row** (§7c.10): `rk_ports::CancelToken` is `Arc<AtomicBool>` with `cancel()` / `is_cancelled()` and **no cause and no timeout arm** [measured 2026-09-17, `crates/rk-ports/src/cancel.rs`], and it is ONE object cloned in at construction (ADR-0099 decision 2) — so the adapter never fires it, and firing it on one dispatch's timeout would cancel every later one. `dispatch` races the child's exit against its own `tokio::time::timeout(spec.timeout_secs)` and a `tokio::time::interval(CANCEL_POLL_MS)` poll of `is_cancelled()`; **`by: Timeout` on the timer arm, `by: Signal` on the token arm, `Operator` unreachable at 7c** — a routed rule, part of this row's OWED record. `CANCEL_POLL_MS` is a named constant beside `ARGV_CAP_BYTES` whose value is chosen at Slice 5 and written there (**UNCONFIRMED** until then; no record names it); neither `tokio::time::sleep` nor `std::thread::sleep` is called (clippy.toml) |
| `Query::ByItem` on the SQLite row | match `VecStore` (whatever `conform_event_store` asserts) | the kernel's gate fold is item-less | **RT-4**; **the typed-payload record is OWED** (`s7a` §7a.5) |
| The toolchain adapters' program path | a `program: PathBuf` construction value on each toolchain adapter (default `cargo`; `npx.cmd` on Windows / `npx` elsewhere — §7c.14), as `harness_binary` is on the Harness adapter | AC-344's missing-binary case needs an absolute nonexistent path to point at (a PATH-less child env does not induce it — measured, §7c.14); the per-OS `npx` resolution is the value's default | built here; a construction value, not a routing — no record owed |
| The toolchain timeout | construction-time config on each toolchain adapter | ADR-0099 decision 2's *"under the same timeout"* has no port parameter to travel on; a port parameter re-opens Checkpoint 7a's six-method count | **RT-5**; **a record naming the bound's source is OWED** |
| `VerifyReport` transport | an inherent `fn verify_report(&self, selection)` on each toolchain adapter; port unchanged | `Toolchain::test()` returns `u32` | **RT-6**; **the transport into `rk_kernel::verify` is 7d's composition root — a record is OWED (R-9)** |
| Two toolchains, one `Arc<dyn Toolchain>` in `Ports` | unchanged | AC-350 needs a composite or a `Ports` change | **RT-7 — ARMED to 7d**; **a record naming the composite is OWED (R-8)** |
| `Default` on usage-adjacent structs | dropped on all of them; `Margins` has none | AC-362; PIA-062; Halt 11 | **RT-8**; no record beyond ADR-0126's forbids |
| The binder (D-11) | `rk-adapter-claude::binder`, with fixture cards | DECISIONS-2026-09-14 D-11 settles the build scope | Built here; **the routing record R-5 is OWED** |
| Configuration-time refusals across the port | **none cross it.** Every card-level, env-level and argv-level lint (AC-203/205/212/214/217/383/488/493/526/377, the argv cap) runs in `Config::new` and `Binder::bind` **before** `dispatch` is called, so `dispatch` receives only a bound, validated spec and `ConfigError` is never a return of `dispatch` (§7c.4) | `Harness::dispatch` returns `DispatchOutcome { record, result: Result<Typed, StepError> }` and nothing else (ADR-0099 decision 1); ADR-0112 closes the set at eleven | Routed; **the residuals — the `StepError` `dispatch` returns when `spec.card_digest` names no card in the validated store, when `spec.inputs` lacks a string `"worktree"` (RT-21's sibling), and for AC-226's configured *refusal* of a version mismatch read from the child's own `system/init` (no record names its variant; the child is killed through the group and the row carries `Observed(v)`) — are `Transport { kind: Spawn, detail }` before spawn for the first two and `Transport { kind: Decode, detail }` for the third, each with its `dispatch_completed` row (RT-20 as amended on the third review), and a record naming them is OWED** |
| **The child's working directory** | **`ActivitySpec.inputs` carries a fixed key `"worktree"`** — the run-owned worktree path the kernel's activity runtime fills from the store; the binder puts it on every spec it materialises and a card it cannot fill is a bind-time `ConfigError`; the adapter reads it into a `WorktreePath` at the top of `dispatch` and uses it as `Command::current_dir`; **inside `dispatch`** a spec whose `inputs` lacks a string `"worktree"` is `Err(Transport { kind: Spawn, detail })` before spawn with its `dispatch_completed` row — never a `ConfigError`, which `dispatch` cannot return (§7c.4; RT-20's sibling) | Nothing the adapter receives carries a cwd: `rk_types::ActivitySpec` is ADR-0099:61's five fields and no sixth (`card_digest`, `tools`, `schema`, `timeout_secs`, `inputs`), `Config` is pins, TTLs, margins, the captured map and `harness_binary`, `env::current_dir` is banned in every `crates/*/src` (`ci.yml`'s AC-529 rule), and the adapter is constructed once at the composition root (design §9.1) — so neither a cwd read nor a per-worktree adapter is available; the lifted runner read `spec.cwd` for `current_dir` (§7c.2). AC-529's clause is the ground: *"the run-owned worktree is resolved from the store, never from cwd"* | **RT-21 as amended on the third review** (added to DECISIONS-2026-09-17 on the first; its `ConfigError`-from-`dispatch` wording contradicted ADR-0099 decision 1's `DispatchOutcome` and is withdrawn); a sixth spec field would re-open Checkpoint 7a's record-fixed shape; **a record naming the worktree channel is OWED**; asserted by `dispatch_cwd_is_the_spec_worktree_never_process_cwd` (§7c.18) and RAISED (§7c.22) |
| AC-377's decimal margin rule (`0.005 → 1 tick`) | a pack margin in decimal is parsed to hundredths by **ceiling**; readings are never rounded | **O-3 — OPEN**, DECISIONS-2026-09-17's recommended default | built as the default pending the operator's answer before the throttle slice; RAISED (§7c.22) |
| `--system-prompt` beside `--agents`/`--agent` | **both are passed** on a `static` card dispatch | **O-5 — OPEN**, DECISIONS-2026-09-17's recommended default; the combination is unmeasured at the pin (PIA-066) | built as the default pending the operator's answer before the command-builder slice; the probe is its first measurement (§7c.5) |
| `rk-store` `conn.rs`'s `unwrap_or_else(PoisonError::into_inner)` | **not repaired at 7c**; `rk-adapter-sqlite` adds no recovery of its own | ADR-0118 decision 4 names the repair as owed and unscheduled | **RT-9**; `PROGRESS.md` §2 row (R-7); **a record naming the repair's stage is OWED** |
| The Node technology pack's owner | none at 7c | ADR-0094 decision 1 names only the Rust pack; 7e is the pack-authoring stage | **RT-10 — RAISED; a record naming the owner is OWED** |
| AC-213's `test_timeout` in the Rust pack fixture | the human input, with a `gate_record` slot | AC-213 is HUMAN-GATED | **RT-11**; the gate-record form is 7e's |
| The verify-run `EventKind` (AC-321, D17-4) | none | ADR-0116's route; ADR-0124/0125 minted none | **RT-12 — ARMED**; 7c's toolchain adapters write no verify-run event; **a record naming the kind is OWED** |
| `tests/fixtures/rk-adapter-sqlite` | renamed `tests/fixtures/rk-adapter-edge-fixture` in the stage-open commit; `rk-kernel/Cargo.toml` and `ac484_adapter_edge_fixture` follow in the same commit | Cargo refuses two members named `rk-adapter-sqlite`; the `rk-adapter-` prefix keeps AC-484's rule keyed | **RT-14**; done at stage-open |
| `tokio` | `[workspace.dependencies] tokio = "1"` (caret; `rt-multi-thread`, `process`, `io-util`, `time`, `sync`, `macros`); a dependency of `rk-adapter-claude` **only** — never of `rk-containment` (ADR-0127 decision 2) | ADR-0076; AC-528; ADR-0127 | **RT-15**; the checkpoint's `cargo tree -i tokio` line polices it as a ban |
| `rk-ports` conformance bodies | **the operator's routing, restored on the third review**: `rk-ports` gains `[features] conformance = []` and `#[cfg(feature = "conformance")] pub mod conformance;` in `rk-ports/src`, carrying the per-port bodies **and** their `spec()`/`findings()` helpers, so the real rows run the **same** bodies. `rk-ports`' own conformance test runs as `cargo test -p rk-ports --features conformance -- port_conformance_fake_rows` — **a package enables its own feature for its own tests on the command line, exactly as `crates/rk-ports/Cargo.toml`'s `corpus = []` comment already says (*"run as `cargo test --workspace --features corpus`"*)**; the second review's ground for an unconditional module (*"a package cannot enable its own feature for its tests without a self dev-dependency"*) was false and is struck here and in DECISIONS-2026-09-17 RT-16. Each adapter takes the bodies as a **dev**-dependency feature — `[dev-dependencies] rk-ports = { path = "../rk-ports", features = ["conformance"] }` beside its normal `rk-ports` edge — so **the one consequence the second review never named is stated**: under resolver 2, `cargo test --workspace` / `cargo clippy --all-targets` unify `conformance` onto `rk-ports` for every package in that build, `rk-kernel`'s included; that is test-only code compiled into a test build and nothing else, and `rk-kernel`'s **normal** graph never enables it because only adapters' dev-dependencies name it — asserted by the block's `cargo tree -p rk-kernel -e features` line (S-FEAT). Each body is a **required core** plus a **capability arm** parameterised by the row's registry (§7c.13, §7c.14); `conform_harness` prints `PROBE_RAN_CONFORMANCE=1` **itself, as its last statement**, so the probe's line is evidence the body ran and not a token the probe prints about itself (§7c.15) | AC-499's *"the same assertions execute against both implementations"*; D17-9 makes `snapshot`/`rebuild`/`change_summary` `Unsupported` on the real rows, which the unchanged bodies `.expect()`; the operator's routing of 2026-09-17 (*"behind `feature = "conformance"`"*) | **RT-16 as re-amended 2026-09-17** (the feature is the operator's word; the core/arm split and the spec-parameterised Harness body are the review's amendments and stand); Checkpoint 7a re-run in the same commit; **the `s7a` §7a.10 note is OWED** |
| `rk-adapter-sqlite` → `rk-git` edge | `exec.run(GitOp::RevParse, worktree, ["rev-parse", "--git-common-dir"])` for the store path (AC-370) and `exec.run(GitOp::Log, worktree, ["log", "--max-parents=0", "--format=%H"])` for `Identity.root_commit` (AC-369) — **the verb travels in `args`**: `GitExec::run` injects it only for `InterpretTrailers` [measured 2026-09-17, `crates/rk-git/src/git_exec.rs`], as every caller on the tree spells it (`worktree.rs`: `["rev-parse", "--abbrev-ref", "HEAD"]`); `GitOp` is a closed enum with no `rev-list`, so the root commit is `Log`'s first line | ADR-0077's one wrapper; `rk_store::identity::Identity { root_commit, common_dir, schema_version }` requires the root commit and nothing on the tree derives it (`grep -rn root_commit crates/*/src` → `identity.rs` only [measured 2026-09-17]) | **RT-17**; a new edge, named in §7c.19; **`rk-adapter-claude` takes NO normal `rk-git` edge** (the lifted git snapshot is not lifted — §7c.2) |
| CI's `Command` grep (`ci.yml`) | narrowed to its stated warrant `Command::new("git")` in the stage-open commit; clippy `disallowed-types` + per-site `#[expect]` is the instrument. **Two more CI edits in the same commit:** the AC-362 grep's pathspec narrows from `crates/rk-adapter-claude/*` to `crates/rk-adapter-claude/src/*` (plus `crates/*/src/usage.rs`), and the git rule gains an anchored exclusion `grep -vE '^crates/[^/]+/tests/fixtures/lint-controls/'` — each keeping its positive half — so the committed positive-control files (§7c.17) do not turn CI red on the tree the block passes | ADR-0077 forbids git spawns outside the wrapper, not all spawns; a control file that carries the banned token is the ban's instrument, not its violation | **RT-13**; the `Command` narrowing is on the tree (unstaged); **the AC-362 pathspec and the lint-controls exclusion are NOT on the tree at the time of this rewrite** [measured 2026-09-17: `ci.yml` still greps `'crates/rk-adapter-claude/*'` and `grep -c lint-controls ci.yml` = 0] — both land in the stage-open commit (§7c.20 item 0), which re-checks that all three edits are in its diff; the AC-465 allowlist for the extracted fixture rows (RT-23, below) is a fourth edit in the same commit |
| `TESTED_CLI_VERSION` | a `Config` value — `pub const TESTED_CLI_VERSION: &str = "2.1.275";` in `crates/rk-adapter-claude/src/config.rs`, the declaration form the block's anchored grep reads — never read from the process environment | ADR-0078: the pin is recorded with the measurement; Halt 2 | **DECIDED by D17-11**: `RUNBOOK-instrument.txt` re-teed 2.1.269 → 2.1.275 at stage-open |
| **Extracted corpus rows tracked as fixtures** | the rows the tests read (§7c.17's `recorded/` files: seven `is_error` frames, one `api_retries` line, e1-smoke's three rows, the h1 dispatch rows) are **copied into the tree** with bytes + sha256 per file and named in `ci.yml`'s AC-465 rule as an allowlist beside `RUNBOOK-instrument.txt`; `rk-kernel`'s three already-tracked `*.jsonl` (`findings/ac282_f20_golden_transcript.jsonl`, `logs/implement-bf1d8cec3b22d288/events.jsonl`, `retry/api-retry-rows.jsonl` [measured 2026-09-17, `git ls-files '*.jsonl'`]) are **7b/7b-ii's debt, named as such in the same allowlist and not folded in silently** | AC-465's THEN — *"no corpus artifact (… E3 fixtures, spike JSONL) is tracked in the product repo AND the only reference to them is the pinned SHA"* — and its instrument's *"pin-manifest allowlist"*: whether an extracted row is an *"E3 fixture"* is a reading no record makes, and the alternative (every recorded-frame test behind `--features corpus`, reading `git show $CORPUS_SHA:<path>` — AC-464) would put AC-206/AC-218/AC-221/AC-232/AC-374/AC-380/AC-200 out of the un-featured claude loop | **RT-23** (added to DECISIONS-2026-09-17 on the third review) — routed; **a dated in-place amendment to AC-465 naming what the pin-manifest allowlist may hold is OWED** (D15-2's form) and listed under RAISED (§7c.22) |
| **The capability vocabulary and the registry row type's home** | **one shape, declared in each crate** — four per-crate `registry.rs` files each declaring `struct Row { capability: Capability, verified_on: String, probe: Probe }`, `enum Capability` (the crate's vocabulary) and `enum Probe { Recorded { .. }, None { reason } }` with `#[derive(serde::Deserialize)]` + `#[serde(deny_unknown_fields)]` (§7c.7); the four `enum Capability` bodies are **byte-equal** and the block asserts it by digest; 7d's bridge is `rk_kernel::resolver::CapabilityClaim::new(capability, verified_on, probe.as_str())` (`crates/rk-kernel/src/resolver.rs`: `probe: String` [measured 2026-09-17]) | the kernel says *"No capability registry exists in this workspace"* and holds no vocabulary; a shared type in `rk-types`/`rk-ports` re-opens Checkpoint 5/7a for a criterion (AC-492's *"canonical vocabulary"*) no record homes | **RT-24** (added on the third review) — routed; **a record naming the vocabulary's home is OWED**; the byte-equality line is the floor that keeps four copies one vocabulary until then |
| **The tracker-credential names on the denylist (AC-509)** | `GITHUB_TOKEN`, `GH_TOKEN`, `GH_ENTERPRISE_TOKEN` — the GitHub tracker's ambient credentials (the design's tracker comparison names GitHub's ambient `GITHUB_TOKEN` and no other tracker at Phase 1) — as exact names on `DENYLIST` | AC-509 names no variable (*"the orchestrator holds a tracker credential"*); a committed denylist is source and the probe's outcome on this box is decided by it, so the names cannot be left to an implementer's guess (§7c.6) | **RT-25** (added on the third review) — routed; **a record (or a dated register note under AC-509) naming the tracker credential set is OWED**; the block's shell guard refuses a shell carrying any of the three, as it does `ANTHROPIC_FOUNDRY_API_KEY` |
| **The Rust adapter's count source on the pinned stable toolchain** | **libtest's stable `--list`**: `selected_test_count` = the `: test$` lines of `cargo test <selection> -- --list` minus those of `-- --list --ignored`; `total_test_count` from the unfiltered `--list`; pass/fail from the run's exit status (RT-22); `VerifyReport.argv` is the run vector and the count vectors are that vector plus `-- --list` / `-- --list --ignored` | `libtest --format json` (this file's second-review source) **does not exist on 1.98.1**: a compiled test binary answers `--format json --list` with `error: The "json" format is only accepted on the nightly compiler with -Z unstable-options` and refuses `-Z` outright [measured 2026-09-17, `target/debug/deps/it-*.exe`]; `--format junit` is gated the same way; `cargo nextest` is ABSENT (§7c.14); `RUSTC_BOOTSTRAP=1` is a workaround Halt 7 forbids; the prose summary line is what AC-329 bans | **RT-26** (added on the third review) — routed; it touches ADR-0094 decision 1's *"test = `cargo test --workspace`"* only by adding how counts are read, and **a dated note against ADR-0094 decision 1 (or a successor record) is OWED**; a machine-readable *per-test result* on stable Rust is nextest's and is ARMED (§7c.22) |

**Line numbers into the AC register are not used in this file** (DECISIONS-2026-09-15 D15-2; `s7b-ii` §7b-ii.1
records the +39 drift that motivated the rule). Every register citation is by id and quoted clause. **Every external
anchor into this file today is by LINE — sixteen of them** [measured 2026-09-17 by `grep -n -oE 's7c\`?:[0-9]+'
over `runbook/phase-1/*.md` and `runbook/PROGRESS.md`]: `s7a`:1074, :1142, :1187, :1208, :1221, :1224 and :1236;
`s7b-ii`:329, :844, :856, :861, :901, :1004 and :1010; `s7b`:1977; `PROGRESS.md`:354 — and every one of them
drifts with this rewrite: they read against the pre-hardening pin (`6152219`) and `PROGRESS.md` §2 records that in
the stage-open commit, re-anchoring each to a `§7c.N` heading by symbol (R-21); the closed stage files are not
edited here.

### 7c.2 The lift — five modules at one SHA, with edits

**Source: `$CORPUS_REPO/runner/src/` at `CORPUS_SHA` = `61f590e51cbb00301c310e46aab615563a7787b8`, read through the
object store (`git -C "$CORPUS_REPO" show "$CORPUS_SHA:runner/src/<file>"`), never a working tree** (AC-464;
PIA-018/PIA-036). Every lifted module's header names that SHA and the blob it was lifted from, **the commit SHA
first and the blob second, in `crates/rk-git/src/reconcile.rs`'s form** — the checkpoint reads the **first** 40-hex
token in the file as the lift SHA, so a header that named the blob first would FAIL on a true fact — and Checkpoint
7c checks each header with Checkpoint 6's four-statement shape plus the pinned-value equality. *"Lifted" means with
edits* (PIA-062); the lift table fixes what is lifted, edited, rewritten and **not lifted**, by symbol:

| corpus file (lines at pin) | lands as | lifted | edited | rewritten | NOT lifted |
| --- | --- | --- | --- | --- | --- |
| `event.rs` (505) | `crates/rk-adapter-claude/src/event.rs` (the line classifier) **and** `crates/rk-types/src/usage.rs` (`Usage`, `CacheCreation` — a second lift destination, held to the same header statements by the checkpoint); the headroom parse is **split out into a NEW module, `crates/rk-adapter-claude/src/headroom.rs`** (the digit scanner and the `WindowReading` parse — not lifted; §7c.8) | `StreamLine { kind, raw }`'s shape (raw retained on every line — AC-200); `EventKind::classify()` (never fails; non-JSON → `Malformed`) with **no `deny_unknown_fields`** (AC-201); `ToolCall`; `ApiRetry` and `api_retry_from`; the tests `ttl_split_reads_the_nested_cache_creation_object` and `an_absent_split_is_none_not_zero` | **`StreamLine.raw: serde_json::Value` → `raw: String`, the line's bytes** — at the pin `raw` is a parsed `Value` (`event.rs`:18; `runner/Cargo.toml` enables no `preserve_order`), so what the runner retained was a re-serialisation with sorted keys and `f64` numbers, and lifting it as-is would leave AC-200's byte-for-byte and AC-374's digit scanner *"over `StreamLine.raw`"* with no bytes to read; `Usage` and `CacheCreation` **move to `rk-types`** (§7c.1) and lose `Default`; `ApiRetry` and the rate-limit reading lose `Default` (RT-8); `cache_read_share`, `ttl_split`, `total_input` propagate absence (no `unwrap_or(0)`); `.unwrap_or_default()` in `api_retry_from` removed; tests that used `Usage::default()` become struct literals | the headroom reading: `f64` utilisation → `u16` ticks via the digit scanner (AC-374), in `headroom.rs`; the SDK `rate_limit_info { status, resetsAt, utilization }` shape tolerated beside `unifiedWindows` (AC-380); `seven_day_opus` / `seven_day_sonnet` keys **tolerated and ignored, never named** (AC-375) | **`rate_limit_from_result`** — a headroom source reading `result.rate_limit`, which AC-219 and AC-380 forbid |
| `throttle.rs` (233) | `crates/rk-adapter-claude/src/throttle.rs` | the `Guard` / `Verdict` shape | `Margins` loses `impl Default` (its `max_five_hour: 0.90` was a code margin — AC-377) and becomes integer `margin_ticks` per window from the pack; `Guard::check` compares `ticks >= 100 - margin_ticks` (AC-376) — the margin is consumed by `Guard::check`, **not** by `batch_draw` | **`batch_draw`** — `u16` first/last, `delta_ticks`, `resolution_limited = delta_ticks <= 1` (not `delta < 0.02`), `dispatches.max(1)` dropped, `n < 10` → `low_sample`, per-`resetsAt` split, not-attributable (AC-372); its tests become tick vectors | the `f64` arithmetic in any form |
| `runner.rs` (802) | `crates/rk-adapter-claude/src/runner.rs` | `build_args` **element order, and it is the golden order** (§7c.5): `-p --output-format stream-json --verbose --model <tier> [--effort] --agents <json> [--agent <name>] --json-schema <schema> [--allowed-tools] --permission-mode <m> [--strict-mcp-config] extra_flags` — `--strict-mcp-config` is pushed **after** `--permission-mode` (verified at the pin: `--permission-mode` at `build_args`' tail, `--strict-mcp-config` after it, `extra_flags` last [measured 2026-09-17]); h1-gap's seven-element vector ends in strict only because no optional element was present, and F20's recorded card argv has strict last; prompt on piped stdin then shutdown; `system/init` → effective config + isolation; the drain loop keeping the **last** `rate_limit_event` and collecting `api_retries` and unknown lines; record-on-every-path via `finish()`; `extract_digest` schema validation (`jsonschema` `=0.56.0`) | the discarded stdin write errors become `Transport { kind: Io }`; isolation extends to `NotIsolated { surface, diff_digest }` as a set digest (AC-202) | **`kill_tree`** (`taskkill /T /F`) → `rk-containment` + `CancelToken` (AC-224); **the timeout branch** (`StepError::Timeout(spec.timeout)`, which calls the forbidden tree walk) → the adapter's own timeout arm (§7c.10); **the usage-limit classification** — `UsageLimit { resets_at: None }` hard-coded, and `detect_usage_limit` reading `r.get("resets_at")` from the result — → `RateLimited { resets_at }` from the last non-`allowed` `rate_limit_event` with `detected_by` recorded (AC-523); `.unwrap_or_default()` on the stderr task and in `detect_usage_limit` removed | **the prose-JSON recovery** (the text-parse fallback on the result path — AC-208); **the pre-spawn git snapshot** (`fingerprint::git_snapshot` before spawn) — **not lifted, on the third review**: ADR-0126 decision 1's payload table has no snapshot member, no criterion in this stage's set consumes a tree digest (the worktree's state around a dispatch is the kernel's commit protocol's, AC-012/AC-013), and a lifted value with no carrier is an omission dressed as a lift; so `rk-adapter-claude` takes **no normal `rk-git` edge** (§7c.19) and the probe's temp repository is minted through a **dev** `rk-git` edge (§7c.15) |
| `step.rs` (348) | `crates/rk-adapter-claude/src/card.rs` | `Tier`, `Effort`, `accepts_effort`; `AgentCard` + `to_agents_json` (`tools` becomes required and non-empty — AC-493); the `StepSpec` fields the builder needs | `cache_profile: static \| default` becomes required (AC-383) | — | **`StepError`** (six variants incl. `UsageLimit` and a unit `Cancelled`) — mapped onto `rk_types::StepError`'s eleven; **`retryable()`** (the kernel's); `experiment`/`arm`/`occurrence`/`resume_key` (spike keys) |
| `journal.rs` (315) | **no file** — the two principles land as prose in `crates/rk-adapter-claude/src/record.rs`'s module doc, which cites `runner/src/journal.rs` at `61f590e5…` by SHA **with no blob check**, because no bytes are lifted; there is no `journal.rs` in the adapter and the PIA-036 loop does not name one | the forbidden-root refusal (*never write inside the tree under test*) and append-only discipline as **principles** | — | — | any file-writing journal (a second write path — design §9.1's `rk-store` row; ADR-0099 decision 1's `&mut dyn EventStore` is the only sink); `completed_keys` / `last_warm_mark` / `read_pinned_sha` (reads `REDKILN_SHA=`, not `CORPUS_SHA=`); `anyhow` |

**Dependencies the lift pulls in and what happens to them:** `fingerprint.rs` — `sha256_hex` becomes
`rk_types::digest`; `EffectiveConfig`'s `Default` is dropped; the `GitSnapshot` shell-out is **not lifted** (above).
`result_meta.rs` — `ResultMeta`'s `Default` and `unwrap_or_default` dropped; `looks_like_usage_limit` (a substring
match) survives **only** as the disclosed `text_marker` fallback (AC-523). `hygiene.rs` — the `Environment` struct
only; `assert_clean` reads `std::env::var` and is not lifted (the adapter never reads the process environment —
AC-217); `MIN_CLAUDE_VERSION (2,1,251)` is a floor, not the pin. **Runtime:** the runner is `#[tokio::main]` default
`multi_thread`; the adapter owns a `tokio::runtime::Runtime` (`multi_thread`) constructed once and calls
`block_on` from the sync `dispatch` (ADR-0076: never `Handle::block_on` from a thread the runtime drives; a
`current_thread` runtime freezes spawned tasks between `block_on` calls and the stderr reader is a spawned task);
`Send + Sync` is statically asserted on the adapter type (F41 Limit 1). **Design §9.1's own adapter row names
`journal.rs` in the lift list** while its `rk-store` row makes the store the single write path — an intra-design
tension; the principles-only reading above is taken and a dated design note is OWED (§7c.22).

**A stale number, corrected:** the 211-line file said the `ephemeral_1h_*`/`ephemeral_5m_*` fields are present on
*"954 of 954 recorded runs"*. ADR-0113 decision 2 carries **621 creations / 950 records** (the accepted ADR governs);
PIA-067's denominator is runner-typed step records. The fields are present as a nested object the runner already
parses; **absence stays `None`** because a `Some(0)` would make a TTL claim look verified (Halt 11).

### 7c.3 The stream decoder — unknown events retained, nothing refused

- **AC-200** — **PROVISIONAL: discharged under the 64 KiB cap, gating the LABEL not the BUILD** (the test's fixture
  lines are under the cap and the test asserts so; *"verbatim"* above the cap is RAISED, §7c.22). *"GIVEN a `claude -p` dispatch whose stream contains an event type the adapter's schema does
  not know (F4: `system/thinking_tokens` at 4–6 per dispatch; F1: a top-level `rate_limit_event`, both undocumented) /
  WHEN the dispatch completes / THEN the unknown event is present verbatim in the event log for that run, with its raw
  JSON preserved."* *Verified by:* *"a replay assertion over a recorded fixture stream with three synthetic unknown
  event types injected — the log must contain all three byte-for-byte; plus a property test that log round-trip
  preserves unrecognised fields."* **The carrier is `DispatchRecord.stdout`** (`Retained`, 64 KiB cap then digest +
  head/tail — ADR-0105 clause 7), not a minted event kind; the cap's tension with *"verbatim"* is **RAISED** (§7c.22).
  **The recorded exemplar is not a stream, and its bytes are the runner's** (§7c.17): the F20 row's
  `step.unknown_events` carries five `tool_progress` heartbeat frames and e1-smoke's rows carry `rate_limit_event`
  frames — **the runner's re-serialised unknown lines** (parsed `serde_json::Value`s written back with alphabetically
  sorted keys and `f64` numbers, not the CLI's bytes — the F20 row's keys read `elapsed_time_seconds, heartbeat,
  parent_tool_use_id, …` in sorted order [measured 2026-09-17]) — those are the recorded unknown lines the replay
  reads, with the three synthetic types injected beside them, and AC-200's *byte-for-byte* is over **those** bytes;
  the first CLI-verbatim unknown line is the probe's (§7c.15).
- **AC-201** — BINDING. *"GIVEN the same stream containing an unknown event type or an unknown field on a known event /
  WHEN the adapter parses it / THEN the dispatch still resolves to its normal outcome and no `StepError` is raised by the
  unknown shape alone."* *Verified by:* *"unit test on the stream decoder with `deny_unknown_fields` explicitly
  absent; a fixture stream carrying a future-shaped event that must still yield `Ok`."* The checkpoint bans the token
  `deny_unknown_fields` over the **stream decoder's files** (`event.rs`, `init.rs`) **and over
  `crates/rk-types/src/usage.rs`** as the floor, with a positive control — the result frame's `usage` object
  deserialises into `rk_types::Usage`, and a `#[serde(deny_unknown_fields)]` there (the house form `VerifyReport`
  carries) would fail every dispatch whose usage block gains a field, which is exactly *"an unknown field on a known
  event"*; so `ac201_future_shaped_event_still_ok`'s fixture carries, beside the future-shaped event, **a result
  frame whose `usage` object carries an unknown key** as its second red — **module-scoped, not crate-scoped**, because AC-489's registry is *required* to deny unknown keys
  (§7c.7: *"`serde` deny-unknown + required fields"*) and `rk_types::VerifyReport` already carries
  `#[serde(deny_unknown_fields)]` as the house form for closed records; a crate-wide ban and AC-489 cannot both hold.

### 7c.4 Isolation and adoption — decided from the child's own `system/init`

Every `system/init` line in a 7c fixture is **SCRIPTED** and labelled so (§7c.17): the corpus at the pin retains **no
raw `claude -p` stream transcript** — the runner kept only the parsed `EffectiveConfig` (`session_id`, `model`,
`agent`, `cwd`, `permission_mode`, `tools`, `mcp_servers`, `slash_commands_count`, `init_digest`). The first retained
raw `system/init` is the probe's (§7c.15).

- **AC-202** — BINDING (§0.3 owner; twin AC-195). *"GIVEN a dispatch that passes `--strict-mcp-config` / WHEN the child
  emits `system/init` / THEN isolation is decided from the child's own reported tool/server list, and a list that does
  not match the declared configuration fails the dispatch as `StepError::NotIsolated { surface: tools | mcp_servers |
  env, diff_digest }` — the presence of the flag in argv is never accepted as evidence of isolation, because an
  ignored flag looks identical to an honoured one (brief §C.11). SM-061's instrument is a **set digest** of the
  reported list against the declared one, not a count (PIA-016)."* *Verified by:* *"unit test with a scripted
  responder whose `system/init` reports 280 tools while argv carries the flag — the dispatch must fail; a lint that no
  isolation assertion reads the adapter's own argv."* **The lint half is built, not only the unit half**: the block's
  `rk_absent` over `\bargv\b|\bargs\b|build_args|extra_flags`, module-filtered to `init|isolation` (the files that
  read `system/init` and decide adoption/isolation), with the control `ac202_argv_isolation_control.rs` (§7c.21) —
  a second review found both AC-202's and AC-214's lint halves claimed and built by nothing.
- **AC-204** — BINDING (twin of owner AC-386). *"GIVEN a dispatch that names a card via `--agent` / WHEN `system/init`
  reports the child's effective tool set / THEN adoption holds only if that set is **equal** to the card's declared
  tool set, and any difference — extra or missing — fails the dispatch as `StepError::NotIsolated { surface: tools, ..
  }`; a superset is not adoption. `NotAdopted` is reserved for the wrong agent answering (`{ expected_agent, observed
  }`), never for tool-set inequality."* *Verified by:* *"unit tests for the three cases (equal / subset / superset)
  against a scripted `system/init`; the superset case must fail with `NotIsolated`; a fourth case where `system/init`
  names a different agent must fail with `NotAdopted`."*
- **AC-386** — BINDING (owner). *"GIVEN a dispatch that passed `--strict-mcp-config` and a card `tools` allowlist / WHEN
  the child's own `system/init` reports a tool set that is not equal to the card's effective allowlist / THEN the
  dispatch fails with `StepError::NotIsolated` and is never retried."* *Verified by:* *"an adapter test with a scripted
  `system/init` reporting extra tools, asserting the typed error and zero retry attempts."* The zero-retry half reads
  `rk_types::StepError::retry_class()` (built at Step 5) — the adapter asserts it, the kernel owns it.
- **AC-203** — BINDING; its `doctor` row ARMED to 7d. *"GIVEN a dispatch specification carrying `--agents` with no
  `--agent <name>` / WHEN the adapter builds the command line / THEN the dispatch is refused as a configuration error
  before any process is spawned, because `--agents` is inert alone."* *Verified by:* *"unit test on the command
  builder; a fixture spec that must fail; a `doctor` rule-table row at zero model cost."*
- **AC-226** — BINDING; its `doctor` row ARMED to 7d. *"GIVEN a child whose `system/init` reports a CLI version
  different from `TESTED_CLI_VERSION` / WHEN the adapter validates the child / THEN the configured outcome for a
  mismatch fires — either refusal, or a recorded `Degraded`-style allowance that the configuration explicitly opted
  into — and the test asserts the *configured* one, not merely that some typed outcome occurred."* *Verified by:*
  *"unit test with a scripted `system/init` at a higher and a lower version, run under both configurations, each
  asserting its own outcome; a `doctor` row showing pinned vs detected version at zero model cost."* Four assertions.
  `TESTED_CLI_VERSION = "2.1.275"` is a `Config` value (D17-11).

**Configuration-time refusals are an adapter-side error type, never a `StepError` variant — and they never cross
the port.** ADR-0112 closes the set at eleven and forbids a twelfth without a successor record, and
`Harness::dispatch` returns `DispatchOutcome { record, result: Result<Typed, StepError> }` and nothing else
(ADR-0099 decision 1), so a refusal that happened *inside* `dispatch` would have no representable outcome. **The
path, stated:** every card-level, environment-level and argv-level lint — AC-203 (`--agents` without `--agent`),
AC-205/AC-417 (`StructuredOutput` absent from a schema-bearing card), AC-212 (model/effort not from the bound card),
AC-214 (no explicit `--permission-mode`), AC-217/AC-488 (a denylisted name in the captured map), AC-526 (the TTL pair),
AC-377 (a margin under two ticks), AC-383 (no `cache_profile`), AC-493 (empty or absent `tools`), and the argv cap —
runs at **`Config::new`** (the environment and margin lints, over the captured map) and at **`Binder::bind`** (the
card and argv lints, over the digest-keyed card store, so a `Config` cannot hold a card that fails them). `dispatch`
receives only a bound, validated `ActivitySpec` whose `card_digest` names a validated card; `ConfigError` is the
return of `Config::new` / `Binder::bind` and **never** of `dispatch`. So *"dispatch count 0"* in every red-first
bullet below means **the bind or construction refused and the spawn marker never appeared** (§7c.17: the fixture
child writes one marker file per invocation, so a count is a file count, not a mock). Configuration refusals are
therefore **not termination paths of `dispatch`** and `ac360_one_event_per_termination_path` does not drive them
(§7c.11). The residuals — `dispatch` handed a spec whose `card_digest` names no card in the validated store, or
whose `inputs` lacks a string `"worktree"` (RT-21) — are routed to `Transport { kind: Spawn, detail }` before
spawn, with their `dispatch_completed` row, and AC-226's configured *refusal* (a runtime outcome read from the
child's own `system/init`, so not a configuration-time lint) is routed to `Transport { kind: Decode, detail }` after
the child is killed through the group; the record naming all three is OWED (RT-20 as amended, §7c.1's table).

### 7c.5 The command builder, its lints, and per-profile delivery

One builder assembles every argv; the two lints ADR-0108 decision 4 names — the `StructuredOutput` check and the
schema budget — share it (*"the two lints share the command builder"*), the argv cap sits on the same path (AC-523's
fourth instrument), and no isolation assertion reads what it produced (AC-202).

- **AC-205** — BINDING = the stage's `structured_output_required`; its `doctor` row (AC-095/AC-417) ARMED to 7d.
  *"GIVEN a dispatch that carries a JSON schema and whose effective tool set — **the card's declared `tools:` list and
  nothing else** (ADR-0108 decision 2; a card with no `tools:` key is a configuration error, AC-493's doctor row) —
  does not include `StructuredOutput` / WHEN the adapter prepares the dispatch / THEN it is refused as a configuration
  error before any process is spawned."* *Verified by:* *"unit test on the dispatch builder; a fixture card lacking
  `StructuredOutput` that must fail; a `doctor` rule-table row reporting the same card at configuration time at zero
  model cost (AC-095, AC-417). The check is on the argv the adapter assembles — the same command builder that lints
  AC-132's per-request budget precondition (ADR-0108 decision 4)."* The test asserts **dispatch count 0**. F20 is why
  this cannot be a runtime check: a `tools` allowlist silently disables `--json-schema`, `structured_output` is absent,
  and the process **exits 0 reporting success** (Halt 3).
- **AC-417 (card-validator half)** — CROSS-LISTED with `s7d`'s AC-400–AC-439 range; the `doctor` row is 7d's, the
  card-validator unit test is here because the card type is 7c's (D-11). *"GIVEN a card whose `tools` list omits
  `StructuredOutput` — or a card with **no `tools:` key** … — and whose role has at least one activity demanding a JSON
  schema / WHEN `redkiln doctor` runs / THEN that card is reported as a configuration error at configuration time,
  before any dispatch."* *Verified by:* *"a `doctor` rule row; a red fixture card that must fail; a red fixture card
  with no `tools:` key that must fail (the new doctor row, PIA-060); a card-validator unit test."* The two red fixture
  cards are `structured_output_required`'s and `ac493_empty_or_absent_tools_rejected`'s; the split is recorded at
  both ends (`s7d`'s matching note is OWED — §7c.22).
- **AC-493 (adapter half)** — BINDING; `rk-plugin-gen` half Phase 2; `doctor` row ARMED to 7d. *"GIVEN a capability
  whose adapter mapping resolves to an empty or absent tool list, or a card with no `tools:` key at all / WHEN that
  mapping or card is loaded / THEN it is rejected as an error naming the capability (or the card) — the pre-spawn
  effective tool set is the card's declared list and nothing else, never "inherit everything" (ADR-0108 decision 2;
  the `doctor` row for a `tools:`-less card, PIA-060) / AND it is never passed on as an empty or absent allowlist."*
  *Verified by:* *"a unit test on the tool mapper; a fixture mapping with `tools = []` and a second with `tools`
  absent, both of which must fail; a `doctor` rule-table row."*
- **AC-132 (builder-lint half)** — BINDING; the schema half is `rk-types`' (Step 5). *"GIVEN any schema `rk-types`
  exports across the closed-output boundary, under the precondition that **one output schema is sent per dispatch and
  no strict user-defined tool is on the adapter's command line** (so per-schema equals the vendor's per-request scope;
  the adapter's command builder lints that precondition on its argv — ADR-0099, ADR-0108) / WHEN the schema lint runs
  / THEN the schema declares at most 24 optional parameters."* Its third instrument: *"the argv lint on the adapter's
  command builder, with a fixture that adds a strict tool and must re-open the budget explicitly."*
- **AC-208** — BINDING (twin AC-281/AC-318). *"GIVEN a dispatch result whose typed value is absent or invalid / WHEN
  the adapter attempts recovery / THEN no structured value is recovered by reading the assistant's prose, and the
  failure surfaces as `StepError::Schema`."* *Verified by:* *"a lint/grep rule that the adapter crate contains no
  JSON-from-text extraction (no regex or fenced-block scraper) on the result path; a fixture whose prose contains
  valid JSON that must still fail."* The runner's text-parse branch is the forbidden shape and is **not lifted**
  (§7c.2); the lint half is the block's `rk_absent` over `Regex::new` / `regex::` / a `from_str` over a
  `.text`/`.prose` member, crate-wide over `rk-adapter-claude/src` with the control `ac208_prose_scrape_control.rs`
  (§7c.21) — the fixture test alone would leave the register's first instrument unbuilt.
- **AC-209** — BINDING (static profile). *"GIVEN two dispatches of the same card version with different volatile
  inputs / WHEN the adapter assembles the command / THEN the `--system-prompt` bytes are byte-identical between them
  (a volatile input that reached the system prompt would change them)."* *Verified by:* *"a golden test hashing the
  assembled system prompt across a matrix of volatile inputs (all hashes equal); a replay assertion that the recorded
  prefix digest on both events is the same."*
- **AC-211** — **PROVISIONAL on S-H1c (unrun), gating the LABEL not the BUILD.** *"GIVEN a card whose `cache_profile`
  is `static` (AC-383) and no explicit selection of the append route for it / WHEN the adapter delivers the card / THEN
  the static `--system-prompt` route is used and `--append-system-prompt` / `--exclude-dynamic-system-prompt-sections`
  are absent from argv; under `default` the adapter omits `--system-prompt` and prepends the card body to the user
  message (AC-383's mechanism)."* *Verified by:* *"unit test on the command builder; a golden argv test per profile; a
  card fixture asserting the `static` default."* Built and tested as `golden_argv_per_profile`; **not claimed as
  evidenced** (`s6`'s acceptance-set paragraph's standing form, *"Build them; do not claim them."*). **Both** append
  flags are asserted absent — the 211-line file grepped only one.
- **AC-383 (golden-argv + card-validator halves)** — BINDING here; the `doctor` row and the pack-loader half ARMED to
  7d. *"GIVEN an agent card / WHEN it is loaded / THEN it declares a `cache_profile` naming which axis it is optimised
  against — `static` (commit and worktree churn) or `default` (time and human gates) / AND a card that declares neither
  is refused as a configuration error before any dispatch / AND the profile selects the adapter's delivery mechanism:
  under `static` the card body is delivered as `--system-prompt` (AC-211); under `default` the adapter **omits
  `--system-prompt` and prepends the card body to the user message** — F28's measured arm (18,603 cache-creation flat
  at both 65.2 min and 49.7 h) — so `default` is a delivery route, not a lint-only label."* *Verified by:* *"a
  card-validator unit test; a `doctor` rule-table row; a fixture card with no profile that must fail; a **golden argv
  test per profile** (`static`: `--system-prompt` present; `default`: absent, card body at the head of the user
  message)."*

**The golden PREFIX is CORPUS-MEASURED; the card-dispatch SUFFIX is authored.** F28's `default` arm is
`results/h1-gap.jsonl` at the pin (33,834 bytes, sha256
`4fd395ab2fb56332d99427ad5d669bc3882e3490960d7234f10df0e45082f8cb`), whose **six dispatch rows (of 15 lines** — the
rest are mark/read rows with no `flags`**)** all carry the flags array `["-p", "--output-format", "stream-json",
"--verbose", "--model", "haiku", "--strict-mcp-config"]` — no `--system-prompt`, no `--agents`/`--agent`, prompt on
stdin [measured 2026-09-17 by `git show` at the pin]. The `static` arm is `results/h1-sysprompt.jsonl` (11,417 bytes,
sha256 `0ec4f17f38f0301f6f74a98423f60f622cc73d2bf3fed35fbb41c8a0e2edc43d`; `claude_version` `2.1.261 (Claude Code)`),
whose **two dispatch rows (of 5 lines)** carry the same seven elements followed by `"--system-prompt", "You are a
review assistant. Answer concisely."` — a **45-byte** system prompt [measured 2026-09-17]. **The golden vector has
ONE element order, and its source is the lifted `build_args` (§7c.2)** — the second review's *"measured prefix plus
authored suffix"* said two things, because h1-gap's seven-element vector ends in `--strict-mcp-config` only because
no optional element was present, while `build_args` pushes strict **after** `--permission-mode` and F20's recorded
card argv has strict last. So `golden_argv_per_profile` pins, per profile: `-p --output-format stream-json --verbose
--model <tier> [--effort <e>] --agents <json> --agent <name> --json-schema <schema> --permission-mode <m>
--strict-mcp-config --permission-prompts none` and, under `static` only, `--system-prompt <card body>` appended
**last** (where h1-sysprompt's `extra_flags` put it, after strict [measured 2026-09-17]) — the h1-gap / h1-sysprompt
vectors are the measured **element set and the order of the first six**, not a prefix, and the authored elements
(`--agents`, `--agent`, `--json-schema`, `--permission-mode`, `--permission-prompts none`) are each warranted by a
criterion (AC-203, AC-205, AC-214, AC-215) and none measured in that combination at the pin; the one recorded
card-dispatch argv is the F20 transcript's (`--agents … --agent … --json-schema … --permission-mode
bypassPermissions --strict-mcp-config`, no `--system-prompt` —
`crates/rk-kernel/tests/fixtures/findings/ac282_f20_golden_transcript.jsonl`), and **the probe (§7c.15) is the first
measurement of the full vector**. The test is red by any element out of that stated order. Whether the CLI accepts `--system-prompt` beside `--agents`/`--agent` is **O-5
(OPEN, DECISIONS-2026-09-17)** — the golden vector pins the default O-5 recommends (*"Both are passed"*), the probe
measures it, and a refusal by the CLI is a Halt, not a re-tune. The test's body asserts **both** profiles were
exercised (a count of 2) and fails if either is absent from the fixture list. It pins **nothing about the cache
claim**: the static arm's 1.000 share was measured with a 45-byte prompt, never a 6.6 KB card (audit PIA-012's
caveat). **A stale attribution, corrected:** the 211-line file said the 18,603-token flat-prefix figure *"is
`static`'s only"*; by the corpus (SM-113's baseline) and by AC-384's own lint message (*"against 18,603 flat under
the default prompt"*) it attaches to the **`default`** prompt. AC-383's parenthetical scoping it to `static` and
AC-384's message disagree with each other — a register-side reconciliation is OWED (§7c.22); this file quotes both
and hard-codes neither.

- **AC-212** — BINDING (PIA-015); `doctor` row ARMED to 7d; depends on the binder (§7c.12). *"GIVEN a dispatch whose
  `model` or `--effort` value cannot be traced to the **resolved (bound) card** — the dispatch card the binder
  materialises by injecting the role's tier into the authored card; the authored card file itself may not carry
  `model` or `effort` (AC-118, AC-418), so the values are traceable only through the role (see the glossary entry in
  §0) / WHEN the adapter prepares the dispatch / THEN it is refused as a configuration error before spawn — including
  when an environment variable or the parent process would supply the value, which is never read."* *Verified by:*
  *"unit tests for (a) a bound card whose role declares no `effort`, (b) a bound card whose role declares no `model`,
  (c) an env var set while the bound card is complete (the env value must not appear in argv); a `doctor` rule-table
  row."* Case (c) is a `Config` map carrying the value, never `set_var` (ADR-0083).
- **AC-214** — BINDING **on its unit half only; the lint half is ARMED** (§7c.22). *"GIVEN any dispatch / WHEN the
  command line is assembled / THEN `--permission-mode` is present with an explicit value taken from the card or pack;
  there is no implicit default."* *Verified by:* *"unit test on the command builder; a lint that the flag is
  unconditional on the dispatch path."* No grep in this stage's block can read whether a `push("--permission-mode")`
  sits outside every `if`/`match`/`Option` guard without parsing Rust, and a source-parse test that tried would be
  green for a reason unrelated to its name on the first refactor; the matrix test `ac214_permission_mode_explicit`
  is the unit half and the lint half is placed in the ARMED table with that reason rather than claimed.
- **AC-215** — **SPLIT: the unit half is BINDING; the integration half is ARMED to the probe / 7d.** The
  **[surface unverified]** tag on the version floor is lifted by exactly one observation, named here: the live probe
  (§7c.15) dispatches with `--permission-prompts none` on argv at 2.1.275 (≥ the documented 2.1.259 floor) and the
  CLI accepts it — `live_dispatch_probe` asserts the flag is on the argv it recorded **and** the dispatch reached
  `system/init`, and the pin is recorded beside `TESTED_CLI_VERSION` in `Config`; until the probe has run the tag
  stands. The integration instrument (*"a fixture that requests a disallowed tool, which must terminate rather than
  hang"*) needs the real CLI and is **not** in the probe's assertions: it is ARMED (§7c.22) as a second `--features
  live` `#[ignore]` card, `live_disallowed_tool_terminates`, which is authored here and **not run** at 7c (a second
  paid dispatch the operator did not decide); a weaker implementation that passes the flag, shuts stdin and still
  stalls on a prompt passes the unit half, which is why the unit half alone does not discharge the id. *"GIVEN a dispatch against the pinned CLI
  version, which must be at or above the version that provides `--permission-prompts` / WHEN the child would otherwise
  prompt for a permission decision / THEN the child cannot wait on a prompt: `--permission-prompts none` is passed and
  the child's stdin is closed, so a prompt-shaped stall is impossible rather than merely unlikely."* *Verified by:*
  *"unit test asserting the flag and a null/closed stdin handle; an integration test with a fixture that requests a
  disallowed tool, which must terminate rather than hang under a wall-clock bound."* **"Prompt on stdin" and "stdin
  closed" reconcile only as write-then-EOF**: the prompt is written to the piped stdin and the handle is shut
  (`runner.rs`'s pattern), and the unit test asserts the shutdown after the write. The register's floor is
  **2.1.259** [documented]; the pin is 2.1.275, above it, recorded beside `TESTED_CLI_VERSION` in `Config`.
- **AC-216** — BINDING (twin of owner AC-511). *"GIVEN any dispatch on a subscription-authenticated account / WHEN the
  command line is assembled / THEN `--bare` never appears in argv, under any card, pack or configuration."* *Verified
  by:* *"a property test asserting the token is absent from every argv the builder can produce across the
  configuration matrix (the matrix, not a grep, is the instrument — a plain lint over today's source passes
  vacuously); plus a lint as a cheap second line."*
- **AC-422 (adapter half)** — CROSS-LISTED with 7d; **compound and unsplit in the register** (*"Split this before it
  reaches the runbook"* — not done; RAISED). *"GIVEN a subscription-authenticated environment / WHEN `redkiln doctor`
  runs, or any dispatch is composed / THEN the presence of any name on `rk-adapter-claude`'s committed environment
  denylist … in the captured environment is reported as a configuration error, and `--bare` appears in no composed
  argument vector."* Its adapter instruments — *"an assertion that the `doctor` row and AC-217's construction test
  read one list; a unit test over the argument builder asserting `--bare` is emitted on no input"* — are
  `ac422_denylist_is_one_exported_list` and AC-216's property test; the `doctor` row is 7d's.
- **AC-523's argv-cap instrument** — BINDING = the stage's `argv_cap_refuses_before_spawn`: *"an assertion that the
  argv-length guard (the command builder refuses before spawn with a typed configuration error at the cap — decision
  #18 of 2026-09-11; fixture at the cap) is on the same builder path."* The fixture sits **at** `ARGV_CAP_BYTES`
  (32,767 — routed, §7c.1) and is refused; one byte under it is not. `--system-prompt-file` is **S-SPF**, registered
  and unused (§7c.16).
- **AC-269** — WHOLLY 7c's (`s7b-ii` §7b-ii.14 excluded it, not a half). *"a card-validator test over every card in the
  catalog"* and *"a build-configuration test that the answer API is not linked into the adapter's tool registry"*. The
  catalog iterated at 7c is the binder's fixture cards (`.redkiln/` does not exist until 7e; O-6's default) — **the
  catalog is RAISED**, §7c.22. **SPLIT:** the card-validator half is DISCHARGED here
  (`ac269_answer_api_absent_from_cards`); the build-configuration half (*"not linked into the adapter's tool
  registry"*) would be a `cargo metadata` assertion that `rk-adapter-claude` has no edge to `rk-cli`, and **`rk-cli`
  does not exist at 7c** (§7c.22), so that assertion is vacuous and its red cannot be produced — it is **ARMED to
  7d** with that reason and re-run there.

### 7c.6 The environment builder — a refusal, never a subtraction

- **AC-217** — BINDING (twin of owner AC-488) = the stage's `env_denylist_config_map`. *"GIVEN any dispatch, where the
  child environment is **the captured parent environment minus a committed denylist**
  (`rk-adapter-claude/src/env_denylist.rs`: `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_BASE_URL`, the
  `CLAUDE_CODE_USE_*` vendor-routing variables and every other name the file lists — decision #16 of 2026-09-11,
  inherit minus denylist rather than an allowlist, because Claude Code reads many undocumented variables: proxy,
  locale, `TERM`, editor) / WHEN the child environment is constructed / THEN the adapter asserts no denylisted name is
  present in the captured environment, and a denylisted name found there fails the dispatch as a typed configuration
  error rather than being stripped and passed through / AND the adapter never reads the process environment itself —
  it receives the captured map as a value in `Config`."* *Verified by:* *"a unit test constructing a `Config` whose
  captured environment carries the key, asserting the typed error and a dispatch count of 0 (no `std::env::set_var`
  anywhere — ADR-0083); an assertion on the constructed child env map that no denylisted name is present; the
  `doctor` row (AC-422) and this test read the **same committed list**."* **The 211-line file said "the captured map
  minus a committed denylist" as a strip; the register's THEN says REFUSAL** — `env_denylist_config_map` asserts the
  typed error and dispatch count 0, not a subtraction. The child is spawned with `env_clear()` + the map (the runner
  set neither TTL variable and inherited everything). **What the committed list holds, stated so the probe's outcome
  on this box is decided by the file and not by luck:** the register's three exact names, the `CLAUDE_CODE_USE_*`
  prefix the register names, the tracker-credential names (AC-509) — **`GITHUB_TOKEN`, `GH_TOKEN`,
  `GH_ENTERPRISE_TOKEN`, the GitHub tracker's ambient credentials, as the stage's routed reading (RT-25, §7c.1):
  the register's AC-509 names no variable and the design's tracker comparison names only GitHub's ambient
  `GITHUB_TOKEN`, so the set is routed with its record OWED rather than left to an implementer's guess; none of the
  three is set in this session's shell [measured 2026-09-17]** — and — **added here as an exact name** —
  `ANTHROPIC_FOUNDRY_API_KEY`, which is a vendor credential of the same kind as `ANTHROPIC_API_KEY` and is
  **persisted at user scope on this box** (`[Environment]::GetEnvironmentVariable('ANTHROPIC_FOUNDRY_API_KEY','User')`
  is set; Machine is not [measured 2026-09-17]), so it is in every fresh Git Bash, not only in a Claude-spawned
  session. The list is exact names plus the one register-named prefix; it is **not** `ANTHROPIC_*`-prefix-keyed
  (a prefix rule would be a stage decision no record makes). Consequence for the probe (§7c.15): on this box
  `Config::new` **refuses** the captured map as the criterion says it must, and the probe is run under
  `env -u ANTHROPIC_FOUNDRY_API_KEY bash` — an operator step outside the process, never `set_var`/`remove_var` — so the
  refusal is recorded as the criterion working and never routed around (Halt 7). `env_denylist_config_map`'s fixture
  map carries `ANTHROPIC_FOUNDRY_API_KEY` as a positive case beside `ANTHROPIC_API_KEY`.
- **AC-488 (construction half)** — BINDING = the stage's `api_key_refuses_construction`; **CROSS-LISTED with `s7d`,
  whose acceptance set already carries AC-488** — the split is: 7c = construction refusal on the `Config` map value;
  7d = the composition root reads the environment **once at the edge** and hands the map in (ADR-0083). *"GIVEN
  `ANTHROPIC_API_KEY` is present in the environment the Claude adapter would dispatch from / WHEN the adapter is
  constructed at startup / THEN construction fails with a typed configuration error naming the variable, and no
  dispatch is attempted."* *Verified by:* *"a unit test with the variable set asserting the typed error and a dispatch
  count of 0; a `doctor` rule-table row."* The matching note in `s7d` is OWED (§7c.22).
- **AC-210** — BINDING. *"GIVEN any dispatch / WHEN the child environment is constructed / THEN
  `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and** `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` are set explicitly and both
  values are recorded on the dispatch event (ADR-0078 forbids setting the first without the second; AC-526 is the
  refusal criterion)."* *Verified by:* *"unit test on the environment builder asserting both variables; a replay
  assertion that no dispatch event lacks either TTL field."* The two fields on `dispatch_completed` are
  **non-optional** (§7c.11). *"This criterion asserts only that the variables are set and recorded — F10 measured a ~1
  h TTL, not that this variable controls it, and the criterion must not be read as evidence that the TTL was
  extended."*
- **AC-526** — BINDING (added 2026-09-11; both arms); its `doctor` row ARMED to 7d (AC-423). *"GIVEN a configuration,
  pack value or environment builder path that would set one of `CLAUDE_CODE_PROMPT_CACHE_TTL` and
  `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` without the other, or with two different values / WHEN the adapter is
  constructed or `redkiln doctor` runs / THEN the configuration is refused as a typed configuration error naming both
  variables, before any dispatch / AND a constructed child environment never carries exactly one of the two."*
  *Verified by:* *"a unit test on the environment builder over the four cases (neither / first only / second only /
  both equal) asserting refusal on the two single cases and on a mismatched pair; a `doctor` rule-table row with a red
  fixture; an assertion over the suite's recorded dispatch events that every event carries both fields with one
  value."* **The 211-line file omitted the mismatched-values arm**; `ac526_ttl_pair_refuses_single_or_mismatch` covers
  all five cases (four from the table plus the mismatched pair). ADR-0078 (ii) is the record.
- **AC-509** — BINDING (claimed by no stage until now; `s7a` §7a.11 routes it here). *"GIVEN the orchestrator holds a
  tracker credential / WHEN a child process is spawned for any activity / THEN the child's environment contains no
  tracker credential / AND the assertion is made from the child's own reported environment, not from the parent's
  spawn arguments."* *Verified by:* *"an integration test spawning a fixture child that echoes its environment; a
  `doctor` rule-table row."* The tracker credential names are on the same denylist; the fixture child is the
  spawnable fixture binary §7c.17 names, so the test reads what the child saw, not what the parent passed. **What
  the test can discriminate, honestly:** a tracker credential can never reach the child under this file's own rules
  — AC-217/AC-488 make `Config::new` refuse a map carrying one, and no test may `set_var` one into the process
  (ADR-0083) — so *"no tracker credential in the echo"* is satisfied **by construction through AC-217's refusal**,
  cited, not tested. What `ac509_child_echoes_env_without_tracker_credential` tests is the mutation AC-509 actually
  guards: the child in `echo-env` mode, spawned through the real `dispatch`, echoes an environment **equal to the
  constructed map plus the two TTL variables and nothing else** — red by `env_clear()` removed (the echo then carries
  the outer `cargo test`'s `CARGO_MANIFEST_DIR`, `CARGO_PKG_NAME`, `PATH`, `USERPROFILE`), and by any denylisted name
  in the echo.

### 7c.7 Classification — over ELEVEN variants, exhaustively, keyed on the record

`rk-adapter-claude::classify` maps every termination onto `rk_types::StepError` (Step 5's closed set — `Transport`,
`Schema`, `Timeout`, `Cancelled`, `NonZero`, `RateLimited`, `NotAdopted`, `NotIsolated`, `Platform`, `Unsupported`,
`Overage`) with an **exhaustive `match` and no `_ =>` arm**, and `spoke()` is `rk-types`' (true for `Schema | Timeout
| Cancelled | NonZero | RateLimited | Platform`), **not** the 211-line file's *"only `StepError::Schema` means the
agent produced a judgement"* — that was the runner's `step.rs` semantics, and the stage was wrong.

- **AC-229** — KERNEL/TYPES-OWNED, CROSS-LISTED: the variant set and the kernel's exhaustive match were built at Step
  5 (`crates/rk-types/tests/it/step_error_is_total.rs`) and 7b. 7c's obligation is the adapter's own exhaustive match
  — `ac229_adapter_maps_every_failure_to_closed_variant` — and no `anyhow` (ADR-0102; the CI anyhow-confinement
  rule). The register's *"eight closed `StepError` variants"* is stale against ADR-0112's eleven.
- **AC-206** — BINDING (§0.3 owner; twins AC-439, AC-282, AC-324, AC-142 — same mechanism). *"GIVEN a dispatch that
  passed a schema and returns exit 0 with `subtype: success` but no `structured_output` field / WHEN the adapter
  classifies the result / THEN the result is `Err(StepError::Schema { .. })` and never `Ok` — the fail-open path that
  F20 records is unrepresentable."* *Verified by:* *"unit test over a fixture stream reproducing F20's recorded shape
  (exit 0, `subtype: success`, no `structured_output`); a type-level guarantee that the schema-bearing dispatch path
  has no constructor for `Ok` without a validated value."* The fixture is the **recorded result frame** of the F20
  golden transcript (§7c.17), not a stream.
- **AC-439 (adapter half)** — CROSS-LISTED with `s7d`'s range; discharged by the same test. *"GIVEN a dispatch whose
  process exits 0 and to which a JSON schema was passed / WHEN the stream carries no `structured_output` / THEN the
  step resolves to `StepError::Schema` and is never recorded as success — the exit code is not evidence."* *Verified
  by:* *"an adapter unit test over the recorded F20 exit-0 schema-void fixture; a replay assertion across the recorded
  stream corpus that no schema-void frame maps to success."* **The second conjunct is unsatisfiable at the pin** —
  there is no recorded stream corpus (§7c.17); the honest form is a replay over every `step.result_raw` frame in the
  recorded results, and the conjunct is RAISED (§7c.22).
- **AC-282 (adapter half)** — CROSS-LISTED: the `rk-kernel` half was discharged at 7b-ii (`ac282_structured_output_required`
  over the same golden transcript); the adapter half is AC-206's test. **This is the note `s7b-ii` §7b-ii.14 said was
  OWED in `s7c` recording the AC-282/AC-318 split**: AC-282's kernel instruments are 7b-ii's, its adapter classification
  is 7c's, and AC-318 is pure `rk-kernel` — 7b-ii's, claimed there. PIA-047's placement erratum remains OWED.
- **AC-218** — BINDING (§0.3 owner; twin AC-428). *"GIVEN a result stream carrying `is_error: true` or an
  `api_error_status` / WHEN the process exits **with any code** / THEN the dispatch resolves to `StepError::Platform {
  status, retry_after }` — the exit code is not consulted to overturn it."* *Verified by:* *"unit test on the four-way
  matrix (exit 0/non-zero × error flag set/unset); the exit-0-plus-`is_error` cell must be an error, never `Ok`. …
  the `is_error` / `api_error_status` frames are the seven recorded in `e3.jsonl` at the corpus pin — the must-fail
  fixture for the matrix and for the runbook's `spoke()` rule — not a fixture of our own construction."* The seven
  frames are in `results/e3.jsonl` at the pin (the register's `spikes/results/` spelling predates Step 2's move;
  §7c.17 gives bytes and sha256); `grep -c '"is_error":true'` = **7** [measured 2026-09-17].
- **AC-428 (adapter half; a re-issued id)** — CROSS-LISTED with `s7d`'s range; discharged by AC-218's test. *"GIVEN a
  dispatch whose process exits with any code / WHEN the stream carries an `is_error` or `api_error_status` frame / THEN
  the step resolves to `StepError::Platform` and is never recorded as success (see AC-218 and §0.3 row C5 on
  `Platform`'s absence from ADR-0095's table)."* *Verified by:* *"an adapter unit test
  over the recorded `e3.jsonl` frames at the corpus pin; a replay assertion across the recorded stream corpus."* Same
  second-conjunct caveat as AC-439.
- **AC-523 (classification arms)** — BINDING (added 2026-09-11) = the stage's `rate_limited_from_stream` plus the
  zero-backoff table. *"GIVEN a child that terminates with a usage-limit result — the structured `subtype` first; a
  text marker on stderr or the result only as a **disclosed fallback** / WHEN the adapter classifies the result / THEN
  the dispatch resolves to `StepError::RateLimited { resets_at: Option<Timestamp> }`, with `resets_at` taken from the
  last `rate_limit_event` in the same stream (the window whose `status` is not `allowed`), else `None` so the kernel
  falls back to the pack-declared delay (AC-030's degradation) / AND the detection surface is recorded on the
  `rate_limited` event (`detected_by: subtype | text_marker`), and a text-marker detection is never silent / AND the
  retry class is "wait until `resets_at`, else the pack delay; never backoff" — no attempt of the activity budget and
  no exponential delay is spent on it (an adapter that maps a usage-limit result to backoff is forbidden by
  ADR-0095)."* *Verified by:* *"a unit test over a recorded stream fixture (a `rate_limit_event` with a non-`allowed`
  status followed by a usage-limit result) asserting the variant, the `resets_at` value and `detected_by: subtype`; a
  second fixture with the structured subtype absent and the text marker present, asserting `detected_by: text_marker`
  on the event; a table-driven test on the retry classifier asserting zero backoff attempts for `RateLimited`; …"*
  **Neither half of the first fixture is recordable at the pin**: the corpus carries `"status":"allowed"` on all 967
  rate-limit readings and `"subtype":"success"` on all 1,907 result frames, and the only usage-limit strings are
  authored in the runner's own source (`terminal_reason: usage_limit_reached`) [measured 2026-09-17, critic gap 2].
  So: the `text_marker` arm, the zero-backoff table and the argv cap are DISCHARGED here; the **`subtype` arm is
  built over a SYNTHETIC frame whose subtype string is an assumption (UNCONFIRMED — the CLI's usage-limit subtype is
  recorded nowhere at the pin) and carries [surface unverified]**, ARMED to S-RETRY's exhaustion-on-result arm
  (§7c.16), and **named** — `ac523_subtype_detection_synthetic_frame` (`// SCRIPTED — the CLI's usage-limit subtype
  string is an assumption`), in the claude loop under the same PROVISIONAL label the other built-not-claimed tests
  carry, red by `detected_by` written as `text_marker` or by `resets_at` not taken from the last non-`allowed` event.
  Neither test is labelled "recorded". The runner's `.get("resets_at")` on the result is rewritten to read
  the last `rate_limit_event`; **the per-window `status` the THEN names is not in the recorded payload shape**
  (e1-smoke's `unifiedWindows.{five_hour,seven_day}` carry `resetsAt` + `utilization` only; `status` is top-level) —
  the fixture reads the top-level `status` and the per-window shape is UNCONFIRMED until the probe retains a raw
  `rate_limit_event`. Halts 10/13: the text-marker set is the runner's `looks_like_usage_limit` list **recorded, never
  tuned**; count positives before precision.
- **AC-221** — **PROVISIONAL, built and NOT claimed** · **[surface unverified — no true-valued `isUsingOverage`
  instance in the recorded runs]**: the register's harness scope note says the tag *"now survives only where a value has
  never been observed (a true `isUsingOverage`, AC-221/AC-378)"* and *"A green synthetic test on an unverified surface
  is still an unproven criterion"*, so AC-221 sits beside AC-378 in §7c.22's PROVISIONAL sub-list until a true-valued
  reading is recorded — S-RETRY's exhaustion arm (§7c.16) is the candidate.
  *"GIVEN a `rate_limit_event` whose `rate_limit_info.isUsingOverage` is `true` / WHEN a dispatch is requested / THEN
  it is refused as `StepError::Overage` before spawn."* *Verified by:* *"unit test on the refusal against the
  **recorded** false-valued payload with the boolean flipped to `true` (the only construction the record licenses)."*
  The recorded payload is e1-smoke's raw `rate_limit_info` (`isUsingOverage: false`, `overageStatus: rejected`,
  `overageDisabledReason: out_of_credits`) — **not** e0-headroom/s-var, which carry the runner's parsed `f64`
  `headroom` object (critic gap 5).
- **AC-378** — **PROVISIONAL (no true-valued reading recorded), built and not claimed**: the retry table maps
  `Overage` → zero attempts (kernel's, 7b) and the adapter spawns nothing after the flipped reading
  (`ac378_overage_zero_attempts_no_spawn`), in **exactly AC-376's shape** (§7c.8): `dispatch` returns
  `Err(StepError::Overage)`, the fake store holds exactly one `dispatch_completed` row carrying `cli_version:
  Pinned(..)` and no reading beyond the flipped one, the spawn-marker count is 0, and **no `step_refused` row was
  written by the adapter** — the `step_refused { class: overage }` arm is `rk_types::StepError::Overage.event()`'s
  (Step 5), asserted by name as the kernel's kind, never written here. (The first form of this bullet asserted a
  `step_refused` row in the fake store, which only the adapter could have written in that test and which `ac376_…`
  forbids — the two tests could not both pass.)
- **AC-225** — BINDING. *"GIVEN a dispatch terminated by the adapter's own cancel (the `CancelToken` fired) / WHEN the
  child exits with no result message — exit 143 on POSIX; the adapter's own `TerminateProcess` code on Windows, where
  143 never obtains / THEN the outcome is `StepError::Cancelled { by: signal | timeout | operator }` and never
  `NonZero(..)`, `Timeout` or `Schema` — the classifier is keyed on the **cancel token having fired**, not on the exit
  code, and the exit code is recorded per platform."* *Verified by:* *"unit test on the classifier for the cancel path
  on each platform (token fired + platform exit code → `Cancelled`); a negative test that an *externally* killed child
  (token not fired, same exit code) does not classify as `Cancelled`."* On Windows the code is **the adapter's own**
  (PIA-059): `rk_containment::windows::CANCEL_EXIT_CODE`, a named constant whose numeric value is chosen at Slice 2
  and written there (**UNCONFIRMED** until then — no record fixes it); the positive control terminates with it and
  the negative control is an external kill with the **same** code. The expired per-activity timeout is ADR-0099
  decision 2's second owner and its outcome is **`Cancelled { by: Timeout }`** — the routed default (RT-3 as
  amended, §7c.1) — **but the token itself carries no cause and no timeout arm** (`CancelToken(Arc<AtomicBool>)`,
  `cancel()` / `is_cancelled()` only [measured 2026-09-17]): `by` is the adapter's classification of **which arm of
  its own race fired** — its `tokio::time::timeout(spec.timeout_secs)` → `Timeout`; the polled token → `Signal`;
  `Operator` unreachable at 7c (§7c.10). `ac225_cancelled_keyed_on_token_not_exit_code`'s **positive control has two
  arms: the token fired by the test (→ `Cancelled { by: Signal }`) and the card's `timeout_secs` expiring on a child
  in `hold` mode (→ `Cancelled { by: Timeout }`)**, both with `CANCEL_EXIT_CODE`; the negative control is the
  external kill with the same code and neither arm fired. `Timeout { after_secs }` is the raised alternative no test
  pins.
- **AC-238 (7c half)** — SPLIT: the kernel lint half was built at 7b (`resolver.rs`); 7c ships a dated registry entry
  per adapter. *"GIVEN an activity requiring a capability the selected adapter's registry does not record as verified
  / WHEN the kernel resolves the dispatch / THEN it fails with `StepError::Unsupported(capability)` derived from the
  registry entry (`{capability, verified_on, probe}`), and the kernel contains no adapter-identity branch."*
- **AC-489 / AC-492** — BINDING for all four adapters. AC-489: *"GIVEN a capability row in an adapter's registry / WHEN
  the workspace is built / THEN a row missing `capability`, `verified_on` or `probe` fails the build naming the row /
  AND no capability is readable by the kernel except through a row carrying all three."* *Verified by:* *"a registry
  schema test at build time (`serde` deny-unknown + required fields); a fixture registry with a bare `{capability}` row
  that must fail the build."* AC-492: *"GIVEN an adapter registry naming a capability token or a scope token the
  canonical vocabulary does not define / WHEN the workspace is built / THEN the build fails naming the unknown token /
  AND no code path maps an unknown token to a default, an empty set or a no-op."* *Verified by:* *"a build-time
  vocabulary test; a fixture registry with `capability = "reads_minds"` that must fail; a lint forbidding a catch-all
  match arm in the token mapper."* `snapshot`, `rebuild` (SQLite) and `change_summary`, `affected-selection` (both
  toolchains) are the entries that yield `Unsupported` (D17-9; AC-328). **The registry row's shape — ONE shape,
  DECLARED IN EACH CRATE** (four `registry.rs` files, each with its own `struct Row`, `enum Capability` and `enum
  Probe`, `#[derive(serde::Deserialize)]` + `#[serde(deny_unknown_fields)]`, which is why every adapter manifest
  carries `serde` with `derive` — `serde_json` does not provide the derive macro — §7c.19; the block asserts the four
  `enum Capability` bodies byte-equal so four copies stay one vocabulary, and 7d's bridge is
  `CapabilityClaim::new(capability, verified_on, probe.as_str())` — RT-24, §7c.1): `{capability, verified_on, probe}` with **every member required** (a missing member
  is a build/deserialise failure naming the row — AC-489's red in every crate) and `probe` a **closed enum**
  `Probe::{Recorded { .. }, None { reason }}` — so a row is present, dated and *explicitly* probe-less, never
  absent-member; AC-328's `affected-selection` rows carry `Probe::None { reason: "S-SEL-NODE unrun" }` and are red
  by `basis: affected` under `Probe::None`, not by a missing member. (The first form of this file had the toolchain
  rows *lacking* `probe` while the claude/sqlite rows were red by a *missing* member — one registry type cannot do
  both.) **BINDING for all four means asserted by name in all four**: `ac489_capability_registry_dated` and `ac492_unknown_capability_token_fails` run in
  `rk-adapter-claude`, `rk-adapter-sqlite` and both toolchain crates (§7c.21); the SQLite entries are exactly the
  `snapshot`/`rebuild` rows D17-9 names, and a `test -f registry.rs` plus a `verified_on` grep — which an empty stub
  satisfies — is never the whole assertion. **The Claude registry's rows, named** (the second review found none):
  `dispatch` — `verified_on: "2026-09-17"`, `probe: Probe::Recorded { source }` naming the corpus pin's recorded
  dispatches (`61f590e5…`, `results/e3.jsonl`; the F20 transcript on the tree) at Slice 4, **re-dated in the close
  commit to the probe's corpus commit** (§7c.15), which is the first dispatch at the pinned 2.1.275; and every
  capability the Harness port does not implement (`snapshot`, `rebuild`, `change_summary`, `affected-selection`)
  **absent from the Claude registry, not `Probe::None`** — a Harness claims no store or toolchain capability, and
  `ac238_claude_registry_entry_dated` asserts the `dispatch` row's three members and that the registry names no
  token outside the crate's `enum Capability` (the `reads_minds` red).

### 7c.8 Headroom in `u16` ticks — the digit scanner, the guard, the batch delta

Utilisation is parsed **from the retained raw line bytes** (`StreamLine.raw`) by a digit scanner into hundredths
(`u16`), never through `serde_json`'s `f64` (routed, §7c.1). **The scanner's contract, stated so it is
implementable and not a substring search** (the third review's finding): it is a **structural** scan over the
line's bytes — it walks the JSON with an escape-aware string skipper, so a `"utilization"` **inside a string value**
is never matched; it locates `utilization` **by key path**, separately under `unifiedWindows.five_hour` and
`unifiedWindows.seven_day` (two identical keys under two parents), and tolerates the SDK `rate_limit_info.utilization`
placement (§7c.8 AC-380, a synthetic shape); at each site it reads the decimal digits after the point into ticks
(`0.04` → 4; `0.1` → 10; more than two decimals is a `Decode`-class refusal, never a rounding — AC-374's *"at the
resolution reported"* is 0.01 at every measurement, F6); and the `total_cost_usd` scanner in the same module reads
the result frame's digits **verbatim into `CostUsd(String)`** (RT-19 as amended, §7c.1) — no scale, no truncation,
whatever digit count the CLI prints. A `// SYNTHETIC` fixture line carrying `"utilization"` inside a string value
beside the two real keys is the scanner's own red. The throttle input struct has exactly two window fields; the
margin lives in `Guard::check`; `batch_draw` is rewritten. Halt 12: **do not move `margin_ticks` on a firing**.
**`Margins` reaches the adapter as a `Config` value** (ADR-0083) — the adapter never parses a pack file; the pack's
margin is read by 7d/7e's pack reader into the `Config` the composition root builds.

- **AC-374** — BINDING. *"GIVEN a `rate_limit_event` payload / WHEN its `unifiedWindows.{five_hour,seven_day}.utilization`
  values are stored / THEN they are parsed from the JSON **decimal digits** into an integer tick count (hundredths,
  `u16`) and stored so — "verbatim" means the decimal digits as reported, never an `f64` — at the resolution reported /
  AND no interpolation, smoothing, rescaling or extrapolation to finer resolution is performed anywhere in the
  pipeline."* *Verified by:* *"a round-trip test from a recorded payload fixture asserting the stored ticks re-render to
  the recorded decimal digits byte-for-byte; a type test that the stored field is an integer tick type and no `f64`
  utilisation field exists; a CI grep rule forbidding `interpolate`/`smooth` in the headroom module."* **The
  round-trip test is two-part, and says so:** (1) the recorded e1-smoke line (`utilization: 0.04` on both windows —
  §7c.17) for provenance — which **cannot discriminate** an `f64` path, because `0.04` re-renders as `0.04` either
  way, and which is itself the runner's re-serialisation of a parsed value, one `f64` round-trip from raw; (2) a
  SYNTHETIC line labelled `// SYNTHETIC — 0.10/0.20 trailing-zero digits are unrecorded at the pin` carrying `0.10`,
  `0.20`, `0.09` **and `0.29`, `0.57`, `0.58`**, whose stored ticks (10, 20, 9, 29, 57, 58) must re-render to the
  source bytes — and **the honest limit of the round-trip half, stated:** it is red by a **truncating** `f64` path
  (`(v * 100.0) as u16` gives 28 for `0.29`, 56 for `0.57`, 57 for `0.58` [measured 2026-09-17 in Python]); a
  **rounding** `f64` path (`(v * 100.0).round()`) stores 10 from `0.10` and re-renders `0.10` from tick 10, so it
  survives every two-digit value — only a re-render that goes back *through* the `f64` prints `0.1`, and a
  tick-storing implementation never does that. **The rounding path is refused by the LINT, and only the lint**: the
  type test checks only that the stored field is `u16`, which `(v * 100.0).round() as u16` satisfies, and a bare
  `\bf64\b` ban does not see `serde_json::Value::as_f64()` (`_` is a word character, so `\b` does not fall between
  `_` and `f` — the obvious spelling of the rounding path evaded the second review's ban); so the block's AC-374
  pattern is `interpolat|smooth|\bf64\b|\bf32\b|as_f64|as_f32|from_f64|\.round\(|as_number\(` over the headroom /
  throttle / event modules with `as_f64(` in its control (§7c.21), and this vector discriminates **truncation only**
  — the round-trip half is provenance plus truncation, not the whole instrument. The register's 0.09/0.10/0.20
  vectors are synthetic by construction and are labelled so.
- **AC-372 (calculator half)** — BINDING here; the `redkiln cost`/`status` renderers and the doctor rule ARMED to 7d /
  Step 8. *"GIVEN a headroom delta computed between two `rate_limit_event` readings that **share one `resetsAt`** — a
  batch that crosses a window reset is split per window and the partial window is reported with its own `n`, as F29
  did — over an **explicitly named dispatch set** … `utilization` is parsed from the JSON decimal into an **integer
  tick count** (hundredths, `u16` — AC-374), and the delta is `delta_ticks` / WHEN `delta_ticks <= 1` … / THEN the
  emitted reading carries `resolution_limited: true` / AND a set with `n < 10` is reported `low_sample` and no delta is
  quoted … / AND `redkiln cost <item>` prints a per-item delta only when the item's dispatches were the only traffic in
  the set, and otherwise prints `not attributable` with the interleaving count / AND the `redkiln cost` and `redkiln
  status` renderers print the flags alongside any reading that carries them."* *Verified by:* *"a unit test over the
  delta calculator in ticks at 10/10 (flagged), 10/11 (flagged …), 10/13 (not flagged), and at the decimal bases
  **0.09/0.10, 0.10/0.12, 0.20/0.21** …; a reset-crossing fixture asserting two per-window deltas each with its own
  `n`; an interleaved-traffic fixture asserting `not attributable` and the count; an `n = 9` fixture asserting
  `low_sample` and no delta; snapshot tests of the two named renderers …; a doctor rule that a flagged reading is never
  rendered bare. `throttle.rs`'s `batch_draw` is **rewritten, not lifted** (its `delta < 0.02` on `f64` is the
  defect)."* The 211-line file's `tick_arithmetic_vectors` under-enumerated: it is extended to the three tick pairs and
  three decimal bases, and joined by `ac372_reset_crossing_splits_per_window`, `ac372_low_sample_at_n9` and
  `ac372_interleaved_not_attributable`. Whether the batch-delta-over-the-log logic belongs to the adapter or to
  `redkiln cost` at 7d is **RAISED** (§7c.22); 7c builds the calculator.
- **AC-375** — BINDING. *"GIVEN the throttle decision path / WHEN it computes whether a dispatch may proceed / THEN it
  consults only `unifiedWindows.five_hour` and `unifiedWindows.seven_day` / AND no code path reads, derives, estimates
  or synthesises a per-model-family headroom value."* *Verified by:* *"a CI grep/lint rule over `rk-adapter-claude`
  forbidding a family-keyed headroom identifier; a unit test on the throttle input struct asserting it has exactly two
  window fields."* The 211-line file's *"parse `seven_day_opus`/`seven_day_sonnet`"* is narrowed to
  **tolerate-and-ignore**: the decoder does not fail on the keys and names no field for them.
- **AC-376** — **PROVISIONAL, built and tested, NOT claimed as evidenced — gating the LABEL, not the BUILD**: the
  register's AND (*"the refusal is appended as a `rate_limited` event"*) is inverted by the test below, which asserts
  no `rate_limited` row was written *by the adapter*; ADR-0126 decision 6 obliges the `dispatch_completed` row on
  that path and says in terms that *"a register amendment naming that split is owed"*, so until it lands the AND
  binds as written (ADR-0092: the record outranks the register only where the record decides, and 0126 says it does
  not decide this) and the id sits in §7c.22's PROVISIONAL sub-list, not under DISCHARGED — the same reasoning that
  removed AC-200/AC-227's claims on an earlier review. *"GIVEN a configured margin `margin_ticks` on a visible window / WHEN the last-read utilisation
  for that window, in ticks, satisfies `ticks >= 100 - margin_ticks` / THEN the adapter refuses to dispatch and returns
  `StepError::RateLimited { resets_at }` without spawning a process / AND the refusal is appended as a `rate_limited`
  event."* *Verified by:* *"a unit test on the throttle in ticks with a scripted `rate_limit_event` at the boundary and
  one tick below it, and at the decimal margins 0.07 and 0.18 against bases 0.09/0.10/0.20 …; an assertion that no
  child process was spawned in the refusing case; an assertion that the refusal row is present in the `EventStore`
  fake (a refusal never reaches AC-360's dispatch path, so it needs its own assertion)."* The stage lacked both
  closing assertions; `ac376_throttle_refuses_without_spawn_and_appends_row` carries them. **Which row the adapter
  appends on this path, stated once (§7c.11):** exactly one `dispatch_completed` carrying the refusal —
  `headroom_before` as the reading that fired, no spawn,
  `cli_version: CliVersion::Pinned(..)` because no child reported one, **and no outcome member**: the row is the
  completed `rk_types::DispatchRecord` and ADR-0126 decision 1's payload table names no `result`, which ADR-0099
  decision 1 keeps outside the record (`DispatchOutcome { record, result }`); the refusal's class is what `dispatch`
  *returns* (`Err(RateLimited { resets_at })`) and what the kernel's `rate_limited` row will carry, and the test
  asserts it from the return value — appended through the handed-in store; the **`rate_limited`**
  kind the THEN names is the **kernel's** `StepError::event()` arm written after `dispatch` returns, one producer,
  and the adapter never appends it (a second producer of a kernel kind is the double the kernel's fold cannot tell
  apart). The test asserts the `dispatch_completed` refusal row in the fake, the file-marker spawn count 0, and that
  no `rate_limited` row was written *by the adapter*; the register's *"appended as a `rate_limited` event"* against
  ADR-0126's *"a refusal before spawn"* obligation is RAISED as a producer question (§7c.22).
- **AC-220** — BINDING; `doctor` row ARMED to 7d. *"GIVEN the last recorded `five_hour` utilisation places the account
  within the configured margin of the limit / WHEN a new dispatch is requested / THEN the dispatch is refused with
  `StepError::RateLimited { resets_at }` before spawn, and the margin is at least two instrument ticks (≥0.02 at the
  measured 0.01 resolution)."* *Verified by:* *"unit test over a margin table including the 0.02 boundary; a config
  lint that rejects any margin below two ticks; a `doctor` row showing the current margin and last reading."*
- **AC-377 (config half)** — BINDING; the panicking-adapter `doctor` half ARMED to 7d. *"GIVEN a configuration whose
  window margin, parsed to ticks, is `margin_ticks < 2` — and the margin comes from the pack, never from a code
  default: `Margins` implements no `Default` … / WHEN the configuration is loaded / THEN it is refused as a
  configuration error before any dispatch / AND `redkiln doctor` reaches the same verdict without dispatching a model
  call."* *Verified by:* *"a config-validator unit test with margins 0.005 / 0.01 / 0.02 (1 / 1 / 2 ticks — the decimal
  parse must not round 0.01 to 2 or 0.02 to 1) and at 0.07 / 0.18; a fixture config that must fail; a fixture config
  with no margin that must fail (no default); a compile-fail or grep rule that `Margins` implements no `Default`; a
  `doctor` integration test with the harness adapter replaced by a fake that panics on dispatch."* **The margin parse
  rule for `0.005 → 1 tick` is UNCONFIRMED against AC-374's "no rescaling"**: the test pins the register's vector
  under **DECISIONS-2026-09-17 O-3's recommended default** — a margin is a floor, so a decimal pack margin is parsed to
  hundredths by **ceiling**, for margins only, never for readings — **pending the operator's answer before the throttle
  slice**; O-3 is OPEN, and the rule is RAISED (§7c.22).
- **AC-380** — BINDING. *"GIVEN a harness result envelope / WHEN headroom is read / THEN it is read from the top-level
  `rate_limit_event` stream event / AND `result.rate_limit` is never read, and a null there is never interpreted as
  "no limit in force"."* *Verified by:* *"a fixture replay of a recorded stream in which `result.rate_limit` is null
  and `rate_limit_event` carries values, asserting the values are picked up; a grep rule on `result.rate_limit`."*
  Both shapes parse, with a named error on neither — **and they are not alike in provenance**: the `unifiedWindows`
  arm is the **recorded** one (e1-smoke's rows, §7c.17); the flat SDK `rate_limit_info { status, resetsAt,
  utilization }` shape is **unrecorded at the pin** — `rate_limit_info` appears in the corpus only in e1-smoke's rows
  (as the `unifiedWindows` carrier) and in `runner/src/event.rs`, and no row carries a flat `utilization` [measured
  2026-09-17, `git grep -c rate_limit_info` at the pin] — so its fixture line is labelled `// SYNTHETIC — the SDK
  rate_limit_info flat shape is unrecorded at the pin; its key names are an assumption`, the flat arm of
  `ac380_rate_limit_event_sole_source_both_shapes` is built **[surface unverified]** under the same PROVISIONAL label
  as `ac523_subtype_detection_synthetic_frame`, and AC-380 is DISCHARGED on the recorded arm only (§7c.22).
- **AC-219** — BINDING. *"GIVEN a completed dispatch / WHEN the adapter records headroom / THEN the values come from the
  top-level `rate_limit_event` (`unifiedWindows.{five_hour,seven_day}.utilization`) and `result.rate_limit` is never
  treated as a source."* *Verified by:* *"unit test on a recorded stream where `result.rate_limit` is null and the
  event carries values — the event must be recorded; a lint that `result.rate_limit` is not read."* The runner's
  `rate_limit_from_result` is **not lifted** (§7c.2); the checkpoint bans `result\.rate_limit|rate_limit_from_result`.
- **AC-381** — subject `rk-kernel`, but the calculator's input struct lives here — **allocation RAISED, built here**.
  *"GIVEN any cache metric … / WHEN a headroom figure or a headroom projection is computed / THEN the cache metric is
  not an input to it / AND a cache-derived "headroom saved" field does not exist on any event, projection or
  report."* *Verified by:* *"a type-level separation — the headroom calculator's input struct cannot name a cache field
  — asserted by a compile-fail test; a CI grep rule for `headroom_saved` / `cache_headroom`; a positive test that the
  headroom calculator still produces a batch delta from the two window readings alone, so the criterion is not
  satisfied by removing the calculator."* The compile-fail case is in
  `rk-adapter-claude`'s trybuild harness (§7c.19).

### 7c.9 TTL — FIVE states, not three (ADR-0113)

ADR-0113 decision 1 fixes **five** states and the 211-line file listed three (*verified 1 h*, *TTL-unverified*,
*no-write*) — wrong by omission. The classifier is **new code over the lifted `ttl_split()`**: **verified 1 h**
(`ephemeral_1h_input_tokens > 0 && ephemeral_5m_input_tokens == 0`); **verified 5 m** (the converse); **mixed** (both
positive, reported with both counts); **`TTL-unverified`** (the object absent, or both tiers zero *and*
`cache_creation_input_tokens > 0`); **`no-write`** (all three zero — no TTL claim). The tier split is recorded on
`dispatch_completed` (§7c.11). Billing mode (`isUsingOverage`) is recorded beside every cache measurement (ADR-0078).

- **AC-362** — BINDING (ARMED-vacuous at Step 5 until now). *"GIVEN a `usage` block from the harness that does not
  report a field the event schema knows about (e.g. any cache-TTL split) / WHEN the event is constructed / THEN the
  field is recorded as absent, and the type that carries it is an `Option` with no `Default` impl and no `unwrap_or(0)`
  at any call site, so a consumer cannot read absent as zero without a visible match."* *Verified by:* *"a fixture
  replay of a recorded `usage` block with the TTL fields missing, asserting `None` not `Some(0)`; a serialiser test
  that absent and zero round-trip distinguishably; a CI grep rule forbidding `unwrap_or(0)` / `unwrap_or_default()` on
  the usage struct's fields **and a `#[derive(Default)]` on `Usage` or `CacheCreation` … `ttl_split()`,
  `cache_read_share()` and `total_input()` return `None` / propagate absence when a field is absent, rather than
  being deleted (PIA-062)**."* The register's evidence gloss (*"F3 measured … absent"*) is stale against ADR-0113 (the
  nested object is present); the absence fixture is **synthetic** and says so. `Usage`/`CacheCreation` live in
  `rk-types` (ADR-0126), so the tests are `-p rk-types` and the CI grep's `crates/*/src/usage.rs` scope already
  anticipates it; the derive check searches `rk-types` for `Usage`/`CacheCreation`/`WindowReading` and
  `rk-adapter-claude` for `Margins`/`ApiRetry`.
- **AC-385** — BINDING. *"GIVEN a dispatch / WHEN the child reports its own `system/init` / THEN the observed CLI version
  is recorded on the event, and a mismatch against `TESTED_CLI_VERSION` is surfaced as a typed warning attached to
  every cache metric taken in that run / AND `CLAUDE_CODE_PROMPT_CACHE_TTL` is passed explicitly, never inherited."*
  *Verified by:* *"a fixture replay asserting the version lands on the event; an adapter test asserting the env var is
  on the spawned command line; a report assertion that cache metrics under a version mismatch carry the warning."*
  **The warning's carrier, stated** (the third review found none): the row carries `cli_version` and
  `instrument_version` and **no warning member** (ADR-0126 decision 1's table names none), so the warning is
  **derived on read** by a named adapter accessor, `rk_adapter_claude::record::mismatch_warning(&DispatchRecord) ->
  Option<VersionMismatch { observed, pinned }>` — `Some` exactly when `cli_version` is `Observed(v)` and `v !=
  instrument_version`, `None` on `Pinned` (no measurement to warn about) — and *"attached to every cache metric"* is
  the reader's join over that accessor (7d's `redkiln cost`). `ac385_version_on_event_and_mismatch_warning` asserts
  the row's `Observed(v)` under a scripted init at a foreign version **and** the accessor's `Some`; red by the
  accessor returning `None` under a mismatch, or by a pre-spawn row written `Observed`. **AC-226's allowance arm
  records the same thing**: under the opted-in `Degraded` configuration the dispatch proceeds to `Ok` and its row
  carries `Observed(v)` beside the pin with the accessor `Some`; under the refuse configuration the child is killed
  through the group and `dispatch` returns `Err(Transport { kind: Decode, detail })` with its row (RT-20 as amended,
  §7c.1) — `ac226_…`'s four assertions are those two rows and those two returns.
- **AC-267** — **PROVISIONAL, built and NOT claimed; Halt 9: not measurable.** The adapter fills `gate_resumed`'s
  `cache_creation_input_tokens` / `total_cost_usd` from usage (`Option`, never `Some(0)`; ADR-0123 decision 2; summed
  across the dispatch's turns per ADR-0106 decision 2). **What the adapter can test is only the sum and the `Option`**
  on `dispatch_completed` — `ac267_resume_cost_is_summed_across_turns_and_option` (red by a last-turn-only value or a
  `Some(0)`); a test named for a `gate_resumed` fill path would be green for a reason unrelated to its name, because
  the adapter cannot reach `gate_resumed`: the kernel writes it **before** dispatch with `{prefix_rewrites, token}` only
  (R-17). The threshold *"against the recorded baseline"* has no baseline (none exists; the probe's n = 1 is reported
  at n = 1, never as one), and the fill path needs a kernel change or a second event — **RAISED** (§7c.22).
- **AC-382 (helper half)** — `cache_read_share()` carries `n` and refuses `n < 10` as an automatic-decision input; the
  share reporter's home is UNCONFIRMED and the pack-selection rejection is 7b's — in the ledger (§7c.22).

### 7c.10 Containment — `rk-containment`, ADR-0127, and where the halts bite

**The crate.** `rk-containment` owns **one** `#![allow(unsafe_code)]` module, `ffi.rs` — commented with ADR-0102's
override rule on the line above the attribute — holding **both** arms under `#[cfg(windows)]` / `#[cfg(unix)]`
sections, because `libc::killpg` and `libc::kill(-pgid, 0)` are `unsafe extern "C"` calls too and the workspace's
`unsafe_code = "deny"` would refuse a `posix.rs` without its own `allow`, which ADR-0102 forbids and the checkpoint's
exactly-one count would catch. `windows.rs` and `posix.rs` are **safe wrappers** that call into `ffi.rs` and carry no
`unsafe`. The Windows arm: a Job Object with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` created **before** spawn, the child
assigned before it can fork, the handle held for the child's lifetime, enumeration **via the job**, and
`CANCEL_EXIT_CODE` (§7c.7) passed to `TerminateProcess`. The POSIX arm: `CommandExt::process_group(0)` + `killpg`,
enumeration via the pgid. **The spawn shape, stated (RT-13 amended):** `rk-containment` spawns a
**`std::process::Command`** into the group — on Windows with `creation_flags(CREATE_SUSPENDED)` (available on both
`std` and tokio `Command`s; what `std`'s spawn does *not* keep is the primary-thread handle, which it closes right
after `CreateProcessW`), assigns the suspended process to the job, then resumes it by `OpenThread(THREAD_SUSPEND_RESUME,
…)` + `ResumeThread` over a `CreateToolhelp32Snapshot`/`Thread32First` enumeration by pid, all inside `ffi.rs` — and
returns a **`Contained { child: std::process::Child, group }`** handle. **What `rk-adapter-claude` does with it —
the API tokio actually offers, measured, not assumed:** `tokio::process::Child` has **no `from_std`** — in tokio
1.53.1 (the lock's version) the only `from_std` constructors in `src/process/mod.rs` are `ChildStdin::from_std`,
`ChildStdout::from_std` and `ChildStderr::from_std`, and the one API that accepts a caller-spawned `std` child,
`Command::spawn_with`, is `#[cfg(tokio_unstable)]` and is not taken [measured 2026-09-17]. So the adapter takes the
three pipes with `child.stdin/stdout/stderr.take()` and wraps **each pipe** with `tokio::process::ChildStdin` /
`ChildStdout` / `ChildStderr::from_std` (tokio's `process` feature) for its async drain and its write-then-shutdown
of stdin, awaits exit with `tokio::task::spawn_blocking(move || child.wait())` on its `multi_thread` runtime, and on
the cancel arm kills **through the group handle** (never `tokio::process::Child::kill`) so `wait()` returns.
`tokio::process::Command` and `tokio::process::Child` appear nowhere; the block's `tokio::process::Command` ban
stands (§7c.21). That is O-1's recommended default as amended (spawn suspended, resume after assignment — RT-13 as
amended twice, DECISIONS-2026-09-17) and it is **measured at Slice 2 by two `-p rk-containment` tests that exist at
Slice 2**, not by the `-p rk-adapter-claude` tests Slice 5 lands: `orphan_grandchild_reaped_via_job` (the fixture
child in `orphan` mode; red by a PID walk) and `breakaway_attempt_fails_without_breakaway_ok` (the fixture child in
`breakaway` mode; red by a breakaway that succeeds), both `cfg(windows)` (§7c.18) — Slice 5's claude tests
re-assert the same through the real `dispatch`. **The assign-before-fork obligation has two parent-side
discriminators, and NO child-side line — because of a fact about the harness that runs the tests:** on this box a
test binary run by `cargo test` is **already inside a Job Object whose extended limit carries
`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`** (cargo's own job), while the same binary run directly is in a job **without**
that flag — `IsProcessInJob=1 KILL_ON_JOB_CLOSE=true` under `cargo test`, `KILL_ON_JOB_CLOSE=false` run directly
[measured 2026-09-17 by the third review's scratch crate over `windows-sys` 0.61.2,
`QueryInformationJobObject(NULL, JobObjectExtendedLimitInformation)`]. So the second review's child-side reading —
*exit 3 unless the job carries `KILL_ON_JOB_CLOSE`*, with a negative control spawned directly that must exit 3 —
**cannot fire under the only harness that runs the test**: a child spawned directly inherits cargo's job, reads
TRUE, exits 0, and the test "reports itself powerless" on every run, which under `rk_test` is a FAIL of the block.
That line is **dropped as an instrument** (the bin may print what it sees; nothing asserts it). The two lines that
hold are both the parent's, both inside `spawn_into_job`'s test-only, `#[doc(hidden)]` reading, both taken **before
`ResumeThread`**: (1) **`IsProcessInJob(child_handle, our_job_handle)` against the adapter's OWN job handle** — TRUE
only for a child assigned to *that* job and unaffected by cargo's outer job; and (2) **`ResumeThread`'s return
value, the previous suspend count, exactly 1 when the primary thread has never run**;
`ac224_job_object_created_before_spawn` asserts both. A mutant that spawns un-suspended and assigns afterwards
fails (1) at the moment (2) would be taken, or returns 0 from `ResumeThread`, or never calls it — red every run.
**Two consequences of the same fact, named:** the adapter's `AssignProcessToJobObject` on the suspended child is a
**nested-job assign** (Windows 8+ semantics; the parent job is cargo's under test, and the CLI's or none in
production), and the `CREATE_BREAKAWAY_FROM_JOB` negative control's surface — a spawn error, or a child inside the
job — is measured **across the nested hierarchy**, not against the adapter's job alone (O-2's UNCONFIRMED stays
UNCONFIRMED until Slice 2 measures it under both harnesses). The mechanism that holds is **recorded as the answer
to DECISIONS-2026-09-17 O-1 (as amended) and O-2 in the Slice 2 commit body**, with the cargo-job fact beside it —
facts measured by the fixture, never an edit to ADR-0127 (the ADR README's rule); if the measurement contradicts
0127's *"assigned before it can fork"*, that is a successor record.

**Cancel and timeout — the mechanism, since the token has neither.** `rk_ports::CancelToken` is
`CancelToken(Arc<AtomicBool>)` with `cancel()` and `is_cancelled() -> bool` and nothing else [measured 2026-09-17]:
no cause, no timeout arm, no wake-up. It is **one object**, cloned into the adapter at construction and never per
call (ADR-0099 decision 2), so **the adapter never fires it** — firing it on one dispatch's expiry would cancel every
later dispatch. `dispatch` therefore races three futures on its `multi_thread` runtime: the child's exit
(`spawn_blocking(move || child.wait())`), **its own `tokio::time::timeout(Duration::from_secs(spec.timeout_secs))`**,
and a `tokio::time::interval(CANCEL_POLL_MS)` poll of `token.is_cancelled()` (a poll, because the token cannot
notify; `tokio::time::sleep` and `std::thread::sleep` are clippy-banned, and `interval` is neither). The `by` rule
(RT-3, §7c.1): **`Cancelled { by: Timeout }` on the timer arm, `Cancelled { by: Signal }` on the token arm,
`Operator` unreachable at 7c**; on either arm the child is killed **through the group handle** and the row records
`cancelled_by`. Every sentence in this file that once said *"the token's timeout arm"* means this arm. **Two more
public, `#[doc(hidden)]` test-support functions the tests need and the crate plan first omitted** (the third
review): `rk_containment::terminate_pid(pid, exit_code)` — `ac225_…`'s external kill from outside the adapter — and
`rk_containment::is_alive(pid)` — `OpenProcess` + `GetExitCodeProcess == STILL_ACTIVE`, the job-independent
liveness probe `cancel_reaps_process_tree`, `ac278_…` and the orphan tests read; both are `windows-sys` calls that
live in `ffi.rs` behind safe wrappers (so no test re-introduces `unsafe` or `windows_sys` outside the one module),
both spawn nothing and leave no containment, and both are counted in Slice 2's `ffi.rs` site list. They are outside
ADR-0127 decision 2's enumerated surface (create, spawn into, kill, enumerate, drop; `run_bounded`) as test support
and are named as such; **no public function of this crate spawns a child outside a job** — the breakaway attempt
the fixture makes is the fixture's own (§7c.17), never a library function.
`run_bounded(cmd, timeout)` — a bounded child for the token-less toolchain adapters, output to a **file** (a pipe
fills and deadlocks — DECISIONS-2026-09-09 #31), **waited on without a sleep and without tokio**: a `std::thread` calls
`child.wait()` and sends the status on an `mpsc` channel, the caller does `recv_timeout(timeout)`, and on `Timeout`
it kills the group and reaps; neither `std::thread::sleep` nor `tokio::time::sleep` (both clippy-banned) is called. It
returns an **`rk-containment`-local outcome enum** — `Exited(ExitStatus) | TimedOut | OsKilled` — and each adapter
maps it onto `rk_types::StepError` on its own side, so the crate depends on **no `rk-types`**. **Deps, exactly
ADR-0127 decision 2's:** `std`; `windows-sys = "=0.61.2"` (in the lock; exact 0.x pin, ADR-0100) with the features
`Win32_Foundation`, `Win32_Security`, `Win32_System_JobObjects`, `Win32_System_Threading`, `Win32_System_Diagnostics_ToolHelp` (the
crate has no default features — without the list nothing links) *(corrected 2026-09-18 at Slice 2: this listed FOUR; `CreateJobObjectW`'s binding in windows-sys 0.61.2 is behind `Win32_Security` because its `lpJobAttributes` is `*const SECURITY_ATTRIBUTES` — `src/Windows/Win32/System/JobObjects/mod.rs` carries `#[cfg(feature = "Win32_Security")]` on it [measured] — so the fifth feature is forced, not chosen)*; `libc = "=0.2.189"` (in the lock; exact pin) — each
with a workspace row saying why; `nix` avoided; **no tokio, no `rk-types`, no `rk-ports`, no `rk-adapter-*`** (ADR-0127
decision 2 and its forbids; RT-15). All three spawning adapters depend on it, which is what dissolves the 211-line
file's contradiction: ADR-0099 forbade any Windows child outside a Job Object while confining `unsafe` to one module of
`rk-adapter-claude` and giving the toolchain adapters no token.

- **AC-224** — BINDING; **the Windows arm is the LIVE arm on this box; the POSIX arm is ARMED to a recorded CI
  `ubuntu-latest` run** (`ci.yml` has never run — `PROGRESS.md`). *"GIVEN a dispatch that is cancelled by the
  `CancelToken` the composition root owns (the CLI's signal path or the per-activity timeout — the kernel never calls
  cancel) / WHEN the adapter terminates the child / THEN the whole containment group is killed and no descendant of the
  dispatch survives the call — on **Windows** the group is a Job Object created before spawn with
  `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, the child assigned before it can fork and the handle held for the child's
  lifetime …; on **POSIX** it is the process group from `CommandExt::process_group(0)`, killed with `killpg`."*
  *Verified by:* *"an integration test (`cargo test -p rk-adapter-claude -- cancel_reaps_process_tree`) spawning a
  fixture that forks a grandchild **and a detached/breakaway grandchild** (so a tree walk from the child's PID would
  miss it), cancelling, then enumerating the group via the job (Windows) or the pgid (POSIX) and asserting no live
  PID; run on both CI targets as a regression against F22's 12,000 s survivor."* **The 211-line file had this
  backwards** — *"the Windows arm is cfg-gated, so a zero-match run IS the platform hole"* — when `00-preamble.md`
  says every block runs on this Windows box: the Windows arm is asserted **by name** here and a zero-match run is a
  FAIL; the POSIX arm is the hole and is ARMED. **The fixture that discriminates a job kill from the forbidden PID walk
  is the ORPHANED grandchild — and the direct child must be ALIVE when the token fires** (O-2's default as corrected
  on the third review; ADR-0127 decision 7 fixes that the job never sets `BREAKAWAY_OK`): the second review's
  fixture spawned a grandchild and *exited immediately*, so `dispatch` had already returned (NonZero/Decode) before
  any cancel could fire, the WHEN of AC-224 (*"WHEN the adapter terminates the child"*) never occurred, the cancel
  path's kill code never ran, and the orphan was reaped anyway by `KILL_ON_JOB_CLOSE` when the job handle dropped on
  return — so a mutant that walks PIDs but still creates and holds the job passed on every run. The fixture's
  `orphan` mode is therefore **three-level**: the direct child spawns an intermediate, the intermediate spawns the
  **holder** (which writes its pid marker and holds), the intermediate **exits**, and the direct child itself
  **HOLDS** — so the child is alive with `dispatch` in flight when the test fires the token from a second thread, and
  the holder's parent is dead (a PID walk from the live child finds nothing; only the job holds the holder — F22's
  shape, the register's *"so a tree walk from the child's PID would miss it"*). The discriminating observation is
  **the holder's liveness after `dispatch` returns**, read by the job-independent `is_alive(pid)` on the marker's
  pid: dead under the job kill, **alive under the PID-walk mutant**. And the kill must complete before the handle
  drops: `spawn_into_job`'s test-only hook records **kill-then-drop order**, and the test asserts it, so a mutant
  that relies on `KILL_ON_JOB_CLOSE` at drop rather than killing through the job is red too. A breakaway grandchild
  cannot be the positive control: under a job that does not grant `JOB_OBJECT_LIMIT_BREAKAWAY_OK` the breakaway
  attempt either fails at spawn or lands inside the job as an ordinary descendant, which a PID walk from the live
  child also finds — so it discriminates nothing. It is the **negative control**: `cancel_reaps_process_tree` asserts
  the breakaway attempt **FAILS** (a spawn error or in-job — which of the two is UNCONFIRMED until the fixture runs,
  O-2, and is measured across the nested job hierarchy cargo's harness adds, above) and that the holder is dead after
  the kill. The Windows arm's named test is `ac224_windows_job_object_reaps_orphaned_grandchild`; its red is the
  holder surviving the kill — **assign order is not its red** (that is `ac224_job_object_created_before_spawn`'s
  `ResumeThread` / `IsProcessInJob(child, our_job)` reading, in `rk-containment`).
- **AC-278 (containment primitive)** — **ARMED, NOT DISCHARGED — by half, and the primitive is built here.**
  ARMED-FROM-7b-ii (`s7b-ii` §7b-ii.14: *"AC-278 (the process table after exit — `rk-cli`/adapter, 7d/7c; no
  process is spawned on the gate path …)"*); the enumeration API is designed here so descendants are enumerable
  **from outside**. AC-278's GIVEN is *"the kernel has opened a gate and the opening invocation has exited
  (AC-240)"* and its THEN, verbatim: *"no child, background, detached or daemon process belonging to that run
  remains alive"*; its instrument: *"an integration test that enumerates the descendants of the run's job object /
  process group after exit"*. `ac278_no_descendant_survives_exit` in `rk-containment` enumerates the job/pgid after
  a **fixture child** exits — that is the primitive over a fixture, **never the gate path**, on which no process is
  spawned at 7c; the id's GIVEN runs where AC-240's invocation exists, 7d, where `s7b-ii` armed it, and this file
  places the id in the ARMED table by that half (§7c.22) rather than discharging a register subject `rk-kernel`
  over a child the kernel never spawned. AC-254/AC-257 stay `rk-git`/`rk-cli`'s.
- **AC-225** — §7c.7; the classifier is keyed on the token, and the Windows code is `CANCEL_EXIT_CODE`.
- **AC-331 (toolchain half)** — ARMED-FROM-7b-ii to 7c. *"GIVEN a verify run killed by its timeout or by the OS / WHEN
  the gate is evaluated / THEN the result is `Unverifiable` and the gate does not advance the item / AND the outcome is
  never recorded as a pass or as a test failure attributable to the candidate."* *Verified by:* *"an integration test
  that kills the child process mid-run and asserts the recorded outcome class; an assertion that no `GreenAt(sha)` is
  minted for a killed run."* `run_bounded`'s timed-out run is `Timeout`, never a failing test
  (`ac331_killed_verify_is_unverifiable`, in each toolchain crate); the `GreenAt` half is the kernel's, 7b-ii.

**The halts, where they bite here.** **Halt 1** — `cargo build --workspace` is the first line, never `-p`. **Halt 2**
— the instrument moved 2.1.269 → 2.1.275 at stage-open; the fifteenth line compares `claude --version` to the
re-teed file and FAILs on any further move. **Halt 3** — the probe asserts `structured_output` from the raw retained
payload, never exit 0. **Halt 4** — any digest that differs across a kill boundary is a halt, not a retry: the cancel
tests record the child's stdout digest before and after the kill and the block compares nothing across it. **Halt 5**
— the probe is foreground, and its one wait (the `$(cargo test …)` capture) is bounded by the probe card's
`timeout_secs` (§7c.15) through the adapter's own timeout arm (above) — the failure alternative; no grep-shaped
waiter exists in the block. **Halt 6** — every cancel test spawns its **own**
fixture tree under a fresh temp dir (parallel arms share nothing); a tree that will not delete is the handle, and
the test reports it rather than reusing the tree. **Halt 7** — every absence ban has a comment filter and a control.
**Halt 8** — shapes (c)/(d)/(f)/(g)/(h) are the rule the block is written under. **Halt 9** — AC-267 is not
measurable; n = 1 is reported at n = 1. **Halts 10/13** — the text-marker set is recorded, never tuned. **Halt 11** —
`Option` over `0`. **Halt 12** — a firing throttle is a reading, not a reason to move `margin_ticks`.

### 7c.11 The record and `dispatch_completed` — ADR-0126

**The kernel drops `outcome.record`** (`runtime/activity.rs` keeps `outcome.result` and appends a pre-dispatch
`dispatch` row without usage) — so every AC-360 field must be appended **by the adapter itself** through the handed-in
`&mut dyn EventStore`, on **every** termination path. **ADR-0126** (written before stage-open on 2026-09-17 and
committed in the stage-open commit; its `rk-types` code lands at Slice 1; tier 2, on ADR-0120/0123/0124/0125's exact
route — the closed `EventKind` table gains one kind named by a criterion; supersedes nothing, and its decision 6 says
in terms why ADR-0113 decision 1's *"on every `step_*` event that carries usage"* is satisfied rather than
withdrawn) mints **`dispatch_completed`**, and `rk_types::DispatchRecord` grows: `usage: Option<Usage>`, `timings`,
`total_cost_usd: Option<CostUsd>` (the decimal digits verbatim as a `String` newtype — never an `f64`, which
`#[derive(Eq)]` refuses and `rk-store`'s canonical serialiser refuses, and never a fixed scale, which would truncate a
digit count the pin does not record; RT-19 as amended, §7c.1), `headroom_before` / `headroom_after:
Option<WindowReading>` (`u16` ticks × exactly two windows — AC-375), **`cli_version: CliVersion`** — a non-optional
enum, `Observed(String)` on every path that read the child's `system/init` and `Pinned(String)` (the `Config` pin) on
every path that terminated before one was read (the pre-spawn refusals of AC-221/AC-376, a spawn failure, a kill
before init), so the provenance is distinguishable and AC-385's mismatch warning attaches only to `Observed` (ADR-0126
decision 1; the shape's history — D17-8 first fixed `cli_version: String` — is D17-8's dated note in
DECISIONS-2026-09-17, and **ADR-0126 is committed at `status: proposed` until the operator confirms the two-armed
value**, §7c.20 item 0; AC-227's *"non-optional field"* holds on both arms) — `api_retry_events: u32`,
the two TTL fields (non-optional — AC-210; carried on ADR-0126's table, §7c.1), **`cancelled_by:
Option<CancelledBy>`** (the cancel cause — `rk_types::error::CancelledBy`, the type that exists) and **`detected_by:
Option<DetectedBy>`** (a new `rk_types::usage::DetectedBy { Subtype, TextMarker }` — the enum AC-523 names and no
crate declares today), each `Some` only on the path where it applies. **No `result` member**: the row is the
completed record and nothing else — ADR-0126 decision 1's payload table names no outcome member and ADR-0099
decision 1 keeps `result` outside the record (`DispatchOutcome { record, result }`); the first form of this file put
`result: Err(..)` on the refusal rows (§7c.8), which no record covers and which `StepError`'s and `Typed`'s derives
(no `Serialize`; `Typed` is `PartialEq` without `Eq`) could not carry under `DispatchRecord`'s `#[derive(Eq)]`. **The
observed version's source field on `system/init` is [surface unverified]**, exactly as AC-511's auth field is: the
runner's `EffectiveConfig` at the pin read eight keys and no version (`fingerprint.rs`), so the key the adapter reads
`Observed(..)` from is named by the probe's raw init (§7c.15) and is UNCONFIRMED until then. **No `init_raw`
member**: the raw `system/init` lives on the probe's own output file (§7c.15), not on the record (RT-18, §7c.1).
**Every new type the record gains, with its home and its shape — one line, so Slice 1 declares nothing by
guess** (the third review found three members with no declared type: `timings`, `CliVersion`'s module, the cost
newtype): all in **`crates/rk-types/src/usage.rs`** — `Usage` (the counters as the harness names them, each
`Option<u64>`, with `cache_creation: Option<CacheCreation>`), `CacheCreation { ephemeral_1h_input_tokens:
Option<u64>, ephemeral_5m_input_tokens: Option<u64> }`, `WindowReading { five_hour: u16, seven_day: u16 }` (ticks;
`resets_at` per window is the typed-payload record's), **`Timings { wall_ms: u64, api_ms: Option<u64> }`** (the
adapter's only `Instant::now` sites are the two that bound `wall_ms`, each under `#[expect(clippy::disallowed_methods,
reason = …)]` — §7c.19), **`CliVersion { Observed(String), Pinned(String) }`**, **`CostUsd(String)`**, and
**`DetectedBy { Subtype, TextMarker }`** — each `Debug, Clone, PartialEq, Eq, Serialize, Deserialize` and **no
`Default`**; the derive-Default walk's pair list names `Timings` beside `Usage`/`CacheCreation`/`WindowReading`
(§7c.21). `rk-store`'s duplicate `retention::DispatchRecord` collapses onto `rk-types`'. **The serde derives Slice 1
adds, named and closed over every member the row carries:** `Serialize` + `Deserialize` on `DispatchRecord`,
`Usage`, `CacheCreation`, `WindowReading`, `Timings`, `CliVersion`, `CostUsd`, `Retained`, **`CancelledBy`** (`crates/rk-types/src/error.rs` —
today `Debug, Clone, Copy, PartialEq, Eq` only, so a `cancelled_by` member is E0277 without it; Checkpoint 5's
provisional mark covers `error.rs` too) and **`DetectedBy`** (today the types derive `Debug, Clone, PartialEq, Eq`
only), because `Event.payload` is `serde_json::Value`. **`Retained::Verbatim`'s bytes and the head/tail bytes travel
as the derived `Vec<u8>` form — a JSON array of integers**, which is byte-lossless (a UTF-8-lossy text is not, and
*"verbatim"* (AC-200) has to survive the round trip) and needs no encoder: the first form of this file said *base64*,
which no workspace crate provides (`base64` is in the lock only transitively, 0.22.1 / 0.23.1, and a new 0.x edge
would need an exact-pinned row under ADR-0100 decision 2 that the stage does not add); `rk-store`'s canonical
serialiser accepts integer arrays (it refuses only floats — `canon.rs`). Argv and env travel as **digests only**;
stdout/stderr verbatim to the cap then digest + head/tail; the child environment **never** (ADR-0105 clause 7).
`EventKind::ALL` moves 37 → 38 and Checkpoint 5 re-runs in the same commit (§7c.19).

- **AC-360 (adapter half)** — BINDING; `EventKind` carrier DECIDED by ADR-0126. *"GIVEN an activity that performs one
  harness dispatch / WHEN the dispatch terminates for any reason — success, `StepError`, cancellation or timeout /
  THEN exactly one dispatch event is appended carrying every token counter the harness reports verbatim,
  `total_cost_usd`, the `five_hour` and `seven_day` headroom readings taken before and after, and the pinned
  harness/CLI version / AND no dispatch path exists that can terminate without appending that event."* *Verified by:*
  *"a kernel test that drives every `StepError` variant of ADR-0095's table through the dispatch path and asserts one
  event each — the event arms are the table's (`step_failed { class }`, `step_cancelled`, `rate_limited`,
  `step_refused { class }`; ADR-0105's event-kind table carries them); an exhaustive-match test over `StepError` so a
  new variant fails to compile without an event arm; the `EventStore` fake (ADR-0003) as the assertion surface. The
  `DispatchRecord` half of ADR-0099's `DispatchOutcome` (argv digest, env digest, instrument version, usage, timings,
  raw retention per ADR-0105 clause 7 — argv and environment as digests only, the child environment never) is what
  this event carries."* **The pinned version the THEN names is `instrument_version`** (the `Config` pin); the observed
  one is `cli_version` (AC-227/AC-385) — two members, two questions. **`ac360_one_event_per_termination_path` drives
  every path the adapter can produce through the real `dispatch`** with the fake store as the surface and the fixture
  child (§7c.17) as the process, asserting exactly one `dispatch_completed` per return; how each variant is induced,
  and which are **pre-spawn**: `Ok` (a replayed init + result frame with `structured_output`); `Schema` (the F20
  frame — exit 0, no `structured_output`); `Transport { Io }` (stdin write refused: the fixture child in
  `close-stdin-early` mode **with a prompt fixture larger than the pipe buffer — ≥ 1 MiB — so the parent's write
  blocks until the child's close and fails deterministically**; a small prompt lands in the buffer before the child
  closes its end, milliseconds into startup, and the `Io` path would then be induced only on a lost race),
  `Transport { Spawn }` (a non-existent `harness_binary` path), `Transport { Decode }` (a non-JSON line where a frame
  is required); `NonZero` (`exit:7` mode, no result frame); `Platform` (a replayed `is_error: true` frame from
  `results/e3.jsonl`); `NotAdopted` / `NotIsolated` (a replayed init naming the wrong agent / a superset tool set);
  `Cancelled { by: Signal | Timeout }` (the child in `hold` mode: the token fired by the test from a second thread
  while `dispatch` blocks → `Signal`; the card's `timeout_secs` expiring on the adapter's own timer → `Timeout`, §7c.10);
  **pre-spawn**: `RateLimited` (the throttle over a flipped `headroom_before` reading — AC-376) and `Overage` (the
  flipped e1-smoke payload — AC-221), each with `cli_version: Pinned` and spawn count 0; and **two variants the
  adapter never produces, named so the count against ADR-0112's eleven reproduces**: **`Unsupported`** (the kernel's
  resolver does, AC-238) and **`Timeout { after_secs }`** (no Harness path produces it under RT-3 as amended — the
  expired per-activity timeout is the adapter's own timer arm and yields `Cancelled { by: Timeout }`, §7c.1) — each
  arm exists only because the match is exhaustive; **what those two arms do is NOT asserted by any test** — they are
  unreachable, their append is by construction of the exhaustive match, and a `_ =>` absence grep (`ac229_…`) cannot
  observe what an arm does — and the day a record routes a path to `Timeout` (RT-3's OWED record) `ac360_…` gains
  the case. **Nine variants (`Transport` by three kinds) plus `Ok` are driven; two arms — `Unsupported`, `Timeout {
  after_secs }` — exist undriven; nine plus two is ADR-0112's eleven.** Configuration refusals are not termination paths of
  `dispatch` (§7c.4). **Two rows on failure** (the kernel's pre-dispatch `dispatch` row plus the adapter's
  `dispatch_completed`) against *"exactly one dispatch event"* is RAISED, not settled (§7c.22).
- **AC-227** — **DISCHARGED for `dispatch_completed` only; the other harness rows are RAISED.** *"GIVEN any harness
  event written to the event store / WHEN it is persisted / THEN it carries the CLI version the dispatch actually ran
  against."* *Verified by:* *"a replay assertion over a full run that no harness event lacks the version field; a
  schema requirement (non-optional field) on the event type."* `ac227_every_harness_event_carries_cli_version` iterates
  every row the **adapter** appends (one kind: `dispatch_completed`) and asserts the non-optional `CliVersion` on each,
  including the pre-spawn rows, where a version written as `Observed` is the red (the pin masquerading as a
  measurement — Halt 11's shape). *"Any harness event"* also reaches the **kernel's** pre-dispatch `dispatch` row,
  which carries no version and which ADR-0126 decision 3 keeps, and the kernel's `rate_limited` / `step_*` rows —
  those are not the adapter's to write and the tension is RAISED (§7c.22; ADR-0126 decision 6 names it).
- **AC-232 (adapter half)** — BINDING; the kernel's AC-026 charge was built at 7b and is **unwired** (R-22 owed —
  DECISIONS-2026-09-17; `retry.rs` names the wiring record OWED in terms).
  *"GIVEN a stream containing `system/api_retry` events / WHEN the adapter processes them / THEN each is recorded and
  counted against the activity's attempt budget, and the adapter issues no retry of its own for those attempts."*
  *Verified by:* *"unit test on a **recorded** fixture stream (an `e3.jsonl` row with two inner retries, at the corpus
  pin) asserting budget decremented by two and exactly one child spawn."* The row is **`results/e3.jsonl` line 6** at
  the pin — slug `fix-advance-write-lock-gate-scope`, arm `b-clean`, `step.api_retries` of length 2 (both `attempt:
  1`, a restart sequence — one charge per EVENT, ADR-0122 decision 1) [measured 2026-09-17]; 26 rows of the file's 101
  carry `api_retries`. **What that row holds is the runner's PARSED array** (`[{"attempt":1,"error":"unknown",
  "error_status":null,"max_retries":10,"retry_delay_ms":618}, …]`), not two `system/api_retry` stream lines — so the
  two lines the adapter's drain loop consumes are **re-synthesised from the recorded parsed fields** (`attempt`,
  `error_status`, `retry_delay_ms`) and labelled `// RECONSTRUCTED from results/e3.jsonl:6 — raw line not retained at
  the pin` (§7c.17's labelling rule covers them); the test is not labelled "recorded". The count travels as
  `api_retry_events` on the record (routed, §7c.1); the budget decrement is the kernel's and is asserted at the adapter
  as the count only; *"exactly one child spawn"* is the fixture child's marker-file count.
- **AC-391** — **ARMED, NOT DISCHARGED, by register instruction** (R-22 owed for the wiring; DECISIONS-2026-09-15's
  R-2 — ADR-0122 decision 5(c)'s clarification of its class-scoped clause — is the other owe; ADR-0122 decision 6 mints no per-attempt
  kind). Its adapter half is AC-232's.
- **AC-373** — kernel-owned negative; the record carries **readings**, no delta field, asserted inside AC-360's shape
  test — in the ledger (§7c.22).

### 7c.12 The binder — D-11

DECISIONS-2026-09-14 D-11 settles that the binder is **built at 7c** and its routing record R-5 is owed.
`rk-adapter-claude::binder` takes a role's tier and an authored card, injects `model`/`effort` (the authored card
carries neither — AC-118/AC-418), **runs every card-level and argv-level lint of §7c.4 at bind time** (so
`Binder::bind` returns `ConfigError` and `dispatch` never sees an unlintable spec), and materialises the
`ActivitySpec` the port takes (`card_digest`, `tools`, `schema`, `timeout_secs`, `inputs`) beside a digest-keyed card
store the adapter reads the body from — **how the card BODY reaches the adapter** (`inputs` vs a digest-keyed store)
is routed as the store and RAISED (§7c.22). **The child's working directory travels on the spec, not on the
adapter** (RT-21, §7c.1): `ActivitySpec.inputs` carries the fixed key `"worktree"` — the run-owned worktree path the
kernel's activity runtime fills from the store (AC-529's clause: *"the run-owned worktree is resolved from the store,
never from cwd"*) — and the adapter reads it into a `WorktreePath` at the top of `dispatch`, before the spawn.
**Two refusals, at two ends, and only one of them is a `ConfigError`**: at bind time the binder puts the key on every
spec it materialises and a card it cannot fill is `Binder::bind`'s `ConfigError`, so a spec without it never reaches
`dispatch` *through the binder*; **inside `dispatch`** — which the test drives directly with a key-less spec, and
which returns `DispatchOutcome { record, result }` and nothing else (ADR-0099 decision 1; §7c.4) — a spec whose
`inputs` lacks a string `"worktree"` is **`Err(Transport { kind: Spawn, detail })` before spawn with its
`dispatch_completed` row**, RT-20's sibling (§7c.1; the second review had `dispatch` returning a `ConfigError` it
cannot represent). `Command::current_dir` is that path and never the process's, which `env::current_dir` cannot
supply (banned in every `crates/*/src` by `ci.yml`'s AC-529 rule). The test is
`dispatch_cwd_is_the_spec_worktree_never_process_cwd` (§7c.18; red by a fixture child observed in the process cwd,
and by a key-less spec reaching a spawn or returning anything but that variant with its row).
**The adapter's constructor, stated once** (the third review found it unstated): **`ClaudeHarness::new(config:
Config, token: CancelToken) -> Result<Self, ConfigError>`** — `Config` is the pins (`TESTED_CLI_VERSION`, declared
`pub const TESTED_CLI_VERSION: &str = "2.1.275";` in `config.rs`, the form the block's anchored grep reads), the two
TTLs, **`margins: Margins`** (ADR-0083: a value handed in, never a pack the adapter parses — §7c.8), the captured
map and `harness_binary`; the token is the root's clone (ADR-0099 decision 2); there is no `GitExec` argument
because the pre-spawn snapshot is not lifted (§7c.2) and the adapter has no normal `rk-git` edge (§7c.19).
**`Config` carries one more value, the test seam every spawn-observing test needs: `harness_binary: PathBuf`**
(default `claude`; the probe sets `claude`, the tests set the fixture child of §7c.17 through
`env!("CARGO_BIN_EXE_rk-fixture-child-claude")` — the adapter's own copy of the bin, named differently from
`rk-containment`'s so the two never collide on one output path, §7c.17), so scripted `system/init` lines, result
frames, `is_error` frames, `api_retry` lines and every `StepError`-inducing termination are driven **through the
real `dispatch`**, the real containment and the real decoder — never a mock of the port. **How the fixture child
learns its mode, since the argv is the golden vector and may carry no mode flag** (§7c.5's `golden_argv_per_profile`
pins it): the test puts `RK_FIXTURE_MODE=<mode>[:<arg>]` and `RK_FIXTURE_MARKER_DIR=<dir>` into the `Config`
captured map — names no denylist carries and no adapter `src/` file reads — the child reads them at its own edge
(the bin is outside every `std::env::var` ban's scope, §7c.17), ignores the argv it receives, and `ac325_…`-shaped
assertions in the claude tests read that argv back from the marker file and assert it equals the golden vector: the
seam pins the argv and the env carries the script. The fixture
cards under `tests/fixtures/cards/` include AC-212's three, AC-205/AC-417/AC-493's red cards, AC-383's no-profile card,
AC-269's catalog, and **the probe's haiku card**, which satisfies AC-205 (`StructuredOutput` in `tools`), AC-214
(explicit `permission-mode`), AC-215 and AC-383 (`cache_profile`).

### 7c.13 `rk-adapter-sqlite` — the real `EventStore` row over `rk-store`

`impl EventStore` over `rk_store::conn::Writer` — **no second writer connection** (ADR-0105 clause 7: one writer per
process behind a mutex, readers one per thread, WAL, `busy_timeout` 5000 ms, bundled). **The adapter holds the
`Writer` only and opens a `Reader` per `query` call**: `rk_store::conn::Reader` wraps a bare `rusqlite::Connection`,
which is `Send` but `!Sync` (`conn.rs`'s own doc says so), and `EventStore: Send + Sync`, so a struct holding a
`Reader` field cannot implement the port (E0277); per-call readers are ADR-0105 clause 7's *"readers one per thread"*
read literally. **On open the adapter applies the schema itself**: `rk-store` exposes no migration runner —
`Writer::open` creates the file and sets WAL/`busy_timeout` and nothing else, and `0001_initial.sql` is reachable only
through `rk_store::schema_lint::all()` — so the adapter runs that SQL through `Writer::transaction(|tx|
tx.execute_batch(sql))`, then `identity::initialise` / `read` / `check` (AC-369) — **whose `Identity {
root_commit, common_dir, schema_version }` needs two git answers the adapter takes through `GitExec` (RT-17,
§7c.1): `exec.run(GitOp::RevParse, worktree, ["rev-parse", "--git-common-dir"])` and `exec.run(GitOp::Log, worktree,
["log", "--max-parents=0", "--format=%H"])`, the first line of which is the root commit** (`GitOp` has no `rev-list`;
`run` injects the verb only for `InterpretTrailers`, so the verb is in `args` as every caller spells it [measured
2026-09-17]); every fixture repository `ac369_…`/`ac370_…` build therefore carries **at least one commit**, or
`Log` prints nothing and the open refuses for a fixture reason — and **that** is what
`ac365_missing_store_file_recreated` asserts (an `INSERT INTO t2_event` on a fresh file fails without it). `append`
inserts a `t2_event(kind, payload, anchor_sha)` row; `query` returns ordered by id and `ByItem` matches `VecStore`
(routed); `snapshot`/`rebuild` return `Unsupported` from a dated registry entry (routed, §7c.1). The store path is
`<git rev-parse --git-common-dir>/redkiln/store.sqlite3` through `rk-git`'s `GitExec` (AC-370) — **and the two
things that call needs, named:** the constructor is **`SqliteStore::open(exec: &GitExec, worktree: &WorktreePath) ->
Result<Self, StoreOpenError>`**, taking the `GitExec` the composition root minted (7d's; `GitExec::new(hooks_empty,
toolchain)` needs a `GitToolchain` that only `rk_git::version::assert_at_startup(cwd)` mints, which spawns `git
--version` — so the adapter never constructs one, and the tests mint theirs with `assert_at_startup` over a temp
hooks directory), and **the join rule**: `git rev-parse --git-common-dir` prints a cwd-relative path (`.git`) from a
main worktree, and `--path-format=absolute` exists only from git 2.31 while `rk-git`'s floor is `GIT_FLOOR = (2, 20)`,
so the adapter joins a relative answer onto the worktree path (`worktree.join(answer)`) before `Writer::open` /
`store_path(common_dir)`; `ac370_one_store_per_common_dir` carries a red-first case where a main-worktree open must
resolve `.git` to the absolute common dir. **It inherits
`rk-store`'s `PoisonError::into_inner` recovery and adds none of its own** (ADR-0118; raised, §7c.1). **The
conformance real row runs `rk-ports`' own `conform_event_store` body** from the `conformance` feature module
(`rk-ports = { path = "../rk-ports", features = ["conformance"] }` under `[dev-dependencies]` — RT-16 as re-amended,
§7c.1), which is **a required core plus a capability arm**: the core is the append/query
round-trip every `EventStore` must pass; the arm takes the row's registry and, where the row declares `snapshot` /
`rebuild` unsupported, asserts `Err(Unsupported { capability })` naming the dated entry, otherwise the `Ok` path —
the unchanged body's `store.snapshot().expect("snapshot")` would panic on every real row D17-9 governs. **What that
split is and is not:** the real-row tests report a declared-unsupported capability as a **named, expected
`Err(Unsupported { capability })`**, and any *other* assertion that passes on the fake and fails on the real row
surfaces as a conformance divergence naming the assertion (AC-499's AND); but AC-499's THEN says *"the same
assertions execute against both implementations"* and its AND says a fake-pass/real-fail *"is reported as a
conformance divergence"* — the arm expects away exactly the case the AND names, and calling that *the divergence
form* was this file's reading, not the register's. D17-9 decides the adapters' return value, not how AC-499's AND
reads, so **a dated register amendment is OWED** (§7c.22) stating that a capability the row's registry declares
unsupported is an expected `Err(Unsupported { capability })`, not a divergence. `port_conformance_fake_rows` re-runs
over the split body (Checkpoint 7a provisional). The real row adds `journal_mode = wal` and `busy_timeout = 5000`
assertions.

- **AC-364 (sqlite half + port source-parse half)** — BINDING. *"GIVEN an appended event / WHEN any later operation
  attempts to modify or delete it / THEN the store refuses the write / AND a correction is expressible only as a new
  appended event that references the corrected one."* *Verified by:* *"a store trait test asserting no
  `update`/`delete` method exists on the `EventStore` port; an integration test that a correction produces two rows."*
  No named port-absence test exists (the trait has four methods; `rk-store`'s `there_is_no_delete_against_a_tier_one_table`
  is tier-1 only — critic gap 7), so both halves are authored: `ac364_correction_appends_never_mutates` here and
  `ac364_event_store_port_has_no_update_or_delete` in `rk-ports` (source-parse, Checkpoint 7a provisional).
- **AC-365 (7c half)** — SPLIT: *"GIVEN a repository whose event store file has been deleted / WHEN `redkiln run <item>`
  is invoked / THEN the next action it takes is identical to the action taken with the store intact / AND the store is
  repopulated from git without operator intervention."* 7c owns only *the deleted file is recreated on open*
  (`ac365_missing_store_file_recreated`); next-action equality is AC-161's / 7d / Step 8.
- **AC-368** — BINDING, **homed in `rk-adapter-claude`, not here**: its instrument is a property of the *caller* of
  `append` — the Harness adapter's `dispatch_completed` append — not of the SQLite store. *"GIVEN a tier-2 append
  that fails (disk full, lock contention, corrupt store) / WHEN the failure is observed / THEN the activity continues
  and completes, and the failure is surfaced as a warning with a counter / AND no tier-2 append failure is ever mapped
  to a `StepError` that fails the activity."* *Verified by:* *"a fault-injecting `EventStore` fake (ADR-0003 makes the
  fake a hard requirement) that fails every append, asserting the fixture run still completes; a test asserting the
  tier-1 path does the opposite."* So `ac368_tier2_append_failure_is_warning` is `-p rk-adapter-claude`: a
  fault-injecting fake handed to `dispatch` — **and the warning half is PROVISIONAL, built and NOT claimed, gating
  the label**: ADR-0126's forbids reads *"Forbids a Harness adapter path — `Ok`, any `StepError`, a cancel, a
  timeout, a refusal before spawn — that returns from `dispatch` without appending a `dispatch_completed` row"*, and
  its decision 6 says of AC-368 against AC-360 that *"both cannot hold as worded"* and *"leaves the reconciliation to
  a dated register amendment"*; a test that pinned `Ok(Typed)` after a failed append as the adapter's behaviour would
  decide R-16 by test, in the direction the accepted record's forbids reads against. So the test asserts **only what
  both records permit until the amendment lands**: the append is **attempted** on the failing store (the fake counts
  the call), the failure is **counted and typed** on the adapter's warning counter, and **no panic**; the
  `Ok(Typed)`-after-failed-append assertion is present, **labelled in the test as the R-16 reading under test, not a
  claim**, and the id sits in §7c.22's PROVISIONAL sub-list — **R-16's register amendment is OWED** (§7c.22). What
  the SQLite crate can observe is only that a failing append returns a typed `Err` and never panics:
  `ac368_sqlite_append_error_is_typed_not_panic` (red by an `unwrap` on a busy store) — that negative is discharged.
  The tier-1 negative is `rk-store`'s (Step 6), not the tier-2-only adapter's.
- **AC-369 (sqlite half)** — BINDING; `doctor` row ARMED to 7d. *"GIVEN a store whose identity rows (root commit +
  common-dir, schema version, harness versions) do not match the repository it is opened against / WHEN the store is
  opened / THEN the open is refused with a typed error and nothing is written / AND no "adopt this store" fallback
  path exists."* *Verified by:* *"a unit test opening a store against a second fixture repo; a `doctor` rule-table
  row."* Step 6's `rk_identity` table is the surface.
- **AC-370 (three-worktree half)** — BINDING; `doctor` row ARMED to 7d. *"GIVEN a repository with one or more linked
  worktrees / WHEN any worktree opens the store / THEN it opens exactly one store located at `git rev-parse
  --git-common-dir`/redkiln/ / AND a store file discovered under a linked worktree's own `.git` directory is reported
  by `doctor` as a configuration error, not silently used."* *Verified by:* *"an integration test creating three
  worktrees and asserting one store path and one set of rows; a `doctor` rule row for the stray-store case."* Step 6's
  AC-178 tests ran over a temp dir handed in as `common_dir` and created no linked worktree (critic gap 8), so the
  three-worktree test is genuinely 7c's, as is the `--git-common-dir` derivation.
- **AC-371** — **CROSS-LISTED to `rk-store` (Step 6), not discharged here** — the same reasoning as AC-398's. *"GIVEN
  a repo-referencing event row / WHEN it is written / THEN it is keyed by SHA or tree hash and by no branch name or
  path."* *Verified by:* *"a schema test asserting no branch/path column is part of any key."* Its register subject
  is `rk-store`, and every key column is `rk-store`'s: the adapter applies `0001_initial.sql` unchanged from
  `rk_store::schema_lint::all()` and writes only `t2_event(kind, payload, anchor_sha)`, so **no adapter mutation can
  add a branch or path key column** and a `-p rk-adapter-sqlite` test named for it would be green for a reason
  unrelated to its name. The second review's `ac371_keys_are_sha_only` is dropped from the sqlite loop and from
  Slice 3's subject; the schema test is Step 6's to carry.
- **AC-398 (row-count half)** — **CROSS-LISTED to `rk-store` (Step 6), not discharged here.** *"… THEN the row is
  surfaced as drift with its original values intact / AND it is never deleted, rewritten or silently dropped from the
  report."* Its instrument's *"the store's row count is unchanged before and after"* a history rewrite has **no
  adapter code under it**: the rewrite/reconcile path is `rk-store`'s (`reconcile.rs`, Checkpoint 6) and the adapter
  exposes no delete, so no adapter mutation can flip a row-count test — a `-p rk-adapter-sqlite` test named for it
  would be green for a reason unrelated to its name. The first form of this file listed
  `ac398_orphaned_rows_never_dropped` in the sqlite loop; it is dropped, and the id is placed in CROSS-LISTED
  (§7c.22) with the drift-report renderer ARMED to 7d.
- **AC-399** — **ARMED WHOLE, NOT DISCHARGED — the carrier is undecided and no adapter half exists in the register.**
  *"GIVEN a run in which any
  dispatch failed `StepError::NotIsolated` / WHEN cost, token or cache metrics from that run are read / THEN every
  such row is marked non-comparable in the store / AND no non-comparable row is included in any cost aggregate …"*
  *"a store test asserting the mark is set on all of that run's rows"* **cannot be authored without deciding a
  schema, which this stage may not do** (§7c.1): the only table the adapter writes is `t2_event(id, kind, payload,
  anchor_sha)` (`0001_initial.sql`, applied unchanged) — no run column, no mark column — and `rk_types::Event` is
  `{kind, payload}`; a mark as an `UPDATE` of appended rows is what AC-364 forbids, a mark as a new kind is minted by
  record or not at all (ADR-0116), and a `0002` migration is a schema record. The carrier is RAISED (§7c.22). **The
  second review's "adapter half" is withdrawn**: it put a `comparable: false` member on the NotIsolated row's
  payload, but ADR-0126 decision 1 fixes the payload as *"the completed `rk_types::DispatchRecord`"* with named
  members and *"fixes the names"*, §7c.11 says the row is the completed record and nothing else (the ground on which
  it struck a `result` member), and `comparable` is neither a `DispatchRecord` member nor in the record's table — a
  member can only enter through decision 6's typed-payload record or a successor; and the register defines no
  adapter half (AC-399's subject is `rk-store` / `redkiln cost`, its THEN *"every such row … in the store"*, and a
  flag on the one row of the dispatch that failed is not all of that run's rows). **What the adapter contributes is
  already built and named elsewhere**: the `Err(NotIsolated { .. })` outcome (AC-202/AC-204/AC-386) and the
  `dispatch_completed` row that records it (AC-360) — the mark's carrier (a payload member by record, a marking
  kind by ADR-0116, or a `0002` migration) is the ARMED table's row (§7c.22), and the `redkiln cost` exclusion is
  7d's. `ac399_not_isolated_row_carries_comparable_false` is deleted from the claude loop and from Slice 5's subject.
- **AC-243 (real-store replay)** — CROSS-LISTED; the kernel half was discharged at 7b-ii. 7c claims **no new
  instrument**: `ac243_gate_open_replay_over_sqlite_store` re-runs the kernel's fixture over the real store
  (`rk-kernel` as a **dev**-dependency of `rk-adapter-sqlite` — an allowed direction; AC-484 forbids the reverse).

### 7c.14 The toolchains — Node and Rust, `Toolchain` twice, one pack

**ADR-0094 decision 1** is the record: *"build = `cargo build --workspace`, test = `cargo test --workspace` (or
`cargo nextest run` where present, recorded on the report), `classify` and `change_summary` over `cargo metadata` and
the changeset"*. `cargo nextest` is **ABSENT** on this box [measured 2026-09-17], so the Rust adapter takes the
`cargo test --workspace` fallback and **records the fallback on the report** (`VerifyReport.argv` is the vector
actually run) — and **the presence arm is unobservable on this box**: `nextest_fallback_recorded_on_report` pins the
fallback's recording only, and *"`cargo nextest run` where present"* is ARMED to a box that has it (§7c.22), which is
the honest limit an implementation that never considers nextest at all would also pass. **How the Rust adapter
counts on the pinned STABLE toolchain — RT-26, §7c.1, because the second review's source did not exist**:
`libtest --format json` is nightly-only on 1.98.1 (a compiled test binary answers `--format json --list` with
`error: The "json" format is only accepted on the nightly compiler with -Z unstable-options`, and `-Z` is refused
outright; `--format junit` is gated the same way [measured 2026-09-17, `target/debug/deps/it-*.exe`]), nextest is
absent, and `RUSTC_BOOTSTRAP=1` is a workaround Halt 7 forbids. So the counts come from **libtest's stable `--list`**:
`selected_test_count` = the lines matching `: test$` from `cargo test <selection> -- --list` **minus** those from
`-- --list --ignored` (the fixture's `#[ignore]` red and slow tests are listed by `--list` and separately by
`--list --ignored`; `--list --ignored` prints only the ignored set [measured 2026-09-17]); `total_test_count` from the
unfiltered `--list` the same way; **pass/fail from the run's exit status** (RT-22's contract). `--list` output is
libtest's one-line-per-test list, parsed by its `: test` suffix — not the prose summary AC-329 bans (`test result:
ok. N passed; …`), which the adapter never reads. The `--list` route counts what would run, not what passed; a
machine-readable per-test result on stable Rust is nextest's and is ARMED (§7c.22). `VerifyReport.argv` records the
**run** vector; the count vectors are that vector plus `-- --list` / `-- --list --ignored`, and `ac325_…` asserts
both were handed to the OS as recorded. `cargo metadata --format-version 1` is read through `serde_json`, not a `cargo_metadata` crate (no new
dep). `rustc -V` is recorded as `toolchain_version`; **`+1.98.1` is never hard-coded** — a third copy of the pin
(ADR-0102/ADR-0083 keep it in `rust-toolchain.toml` and `rust-version`). The Node adapter's test argv is **the full
vector `<npx.cmd|npx> vitest run --reporter=json --outputFile=<tmp> <file> [-t <pattern>]`, stated once** — the JSON
reporter's two flags are on the vector because the counts come from its output file, so a test that pinned only the
tail `vitest run <file> [-t <pattern>]` would either fail on the reporter flags or pass vacuously as a subsequence;
`d34_vitest_invoked_directly_never_pnpm_forwarded` asserts **that ordered vector minus the program element**, never
`pnpm`, `test` or `--` — as separate argv elements, never `pnpm test --`** (DECISIONS-2026-09-09 #34: the positional
filter did not survive pnpm's forwarding — the pnpm form ran unrelated files and never finished inside 900 s; the
direct form reported *Test Files 1 passed, Tests 6 passed, 15.57 s*; #20 parsed the selection into real argv; #31
bounded the step with output to a file; #29's fan-out cap did not take). `pnpm` 10.33.0 **is present** on this box
[measured 2026-09-17], so #34's defect is reproducible locally as AC-325's wrapper fixture. **The program element is
resolved per OS**: on Windows the launcher is `npx.cmd` (`/c/Program Files/nodejs/` holds `npx`, a POSIX shell script,
beside `npx.cmd` and `npx.ps1` [measured 2026-09-17]), and `std::process::Command::new("npx")` resolves only `.exe`
through `CreateProcess` — so the adapter names `npx.cmd` on Windows and `npx` elsewhere, AC-325's recorded vector
carries the **resolved** name, and `d34_…` never asserts the literal `npx`. Both adapters' children run
under `rk_containment::run_bounded` with a **construction-time** timeout (routed, §7c.1). **The toolchain children's
environment is the Harness rule, not the inherited environment** (the third review found it unspecified): each
toolchain adapter spawns with `env_clear()` plus the same captured `Config` map the Harness adapter uses — the
AC-217/AC-488 denylist refused at construction, AC-509's tracker names included — plus `CARGO_TARGET_DIR` (Rust);
and because `npx.cmd` → `cmd.exe` → `node` → npm under an empty map is **unmeasured** (the only measurement, below,
is `cargo` under `env_clear()`), **Slice 6's first act is a measurement on this box** that `npx.cmd vitest run
<file>` starts under that map, recording the names the map must carry (`SystemRoot`, `PATH`, `USERPROFILE`,
`APPDATA` are the candidates; which of them are needed is UNCONFIRMED until measured) in the Slice 6 commit body and
in this section as [measured]. **Every toolchain test and
both real rows are constructed over a committed FIXTURE PROJECT, never over the repository running the checkpoint**
(§7c.17): a Rust adapter over `D:/repos/redkiln-rs` would run `cargo test --workspace` from inside Checkpoint 7c's own
`cargo test --workspace`, re-entering `port_conformance_real_row_rust` without bound on the same target dir. The Rust
fixture is a standalone crate carrying its **own empty `[workspace]` table** (a nested workspace root stops cargo's
upward walk — the root `Cargo.toml`'s *"current package believes it's in a workspace when it's not"* trap — without
touching the root `exclude` line Checkpoint 5 greps literally), placed **outside** the `tests/fixtures/*` member glob
at `crates/rk-adapter-toolchain-rust/tests/fixtures/project/`, with `CARGO_TARGET_DIR` set by the adapter's env map
to a per-test temp dir so concurrent tests share no build lock; the Node fixture is
`crates/rk-adapter-toolchain-node/tests/fixtures/project/` with a committed `package.json`, `pnpm-lock.yaml`, one
`*.test.ts` file (two tests, one filterable by `-t`) and `node_modules/` git-ignored, installed **once, locally,
before the block** by `pnpm install --frozen-lockfile --dir <project>` (network; the block's preflight line refuses
if `node_modules/.bin/vitest` is absent, because `npx` without it prompts on a closed stdin). The self-hosting run
over `redkiln-rs` itself is Step 8's. `classify` answers honestly for non-Rust paths (`spikes/*.py` is not `Rust`) —
ADR-0029 (i): *"computed per item from affected paths via `Toolchain::classify`, never per repo"*.

**The Rust technology pack** is a 7c deliverable (ADR-0094 decision 1: *"A Rust technology pack binds the adapter's
artifact kinds (`TestReport`, `Changeset`) per ADR-0029"*). No technology pack of any kind exists on the tree (the
twelve under `spikes/p9/packs/` are **process** packs). Its shape is ADR-0029's four clauses — (i) facts via
`classify`; (ii) producers for existing artifact kinds keyed to `rust`; (iii) contributed kinds and additive
`contribute` rows; (iv) `timeout_secs` / `cost_class` overrides — and it lives as a **fixture** under
`crates/rk-adapter-toolchain-rust/tests/fixtures/pack/` until 7e installs it into `.redkiln/`. Its `[mutation]`
`feasible`/`reason` spelling is consumed per D17-3. **Who reads the TOML, since the workspace has no TOML
dependency** (the third review's finding: `crates/rk-kernel/src/toml_subset.rs`'s header says *"this workspace has no
TOML dependency"* and calls adding one a dependency decision the stage stops at; `toml 1.1.6` is in the lock only as
trybuild's dev-only transitive; the one in-tree reader is `rk_kernel::toml_subset`, `pub`): **the fixture is read in
tests only, through `rk_kernel::toml_subset`, so `rk-adapter-toolchain-rust` takes `rk-kernel` as a
dev-dependency** (the allowed direction — AC-484 forbids the reverse; the same edge Node already carries for
AC-334, §7c.19), feeding `[mutation] feasible`/`reason` to `rk_ports::toolchain::MutationFeasibility::declared(Option<bool>,
Option<&str>)`; **the runtime pack reader is 7d/7e's** and the adapter's library parses no pack —
`rust_pack_fixture_binds_kinds_per_adr0029` is a `tests/it` test over the fixture, red by a producer not keyed to
`rust`. No `toml` row is raised here; if 7e wants the runtime reader outside the kernel, that is its dependency
decision. **AC-213's `test_timeout`** (**twenty** catalog activities under
`spikes/p9/catalog` and **four** `rk-kernel` catalog fixtures declare `${toolchain.test_timeout}` [measured 2026-09-17
by `grep -rl`]) is the HUMAN input the pack carries: the fixture names a `gate_record` slot for it, and
**no CI job passes or fails on it** (§7c.22). The Rust TYPE is `rk_types::verify::VerifyReport`, never a `TestReport`
struct. The Node technology pack's owner is RAISED (§7c.1).

- **AC-328** — BINDING for both toolchain adapters (ARMED from 7b-ii). *"GIVEN a toolchain adapter whose
  `affected-selection` capability has no probe result recorded at the current version pin / WHEN the kernel resolves a
  verify activity for that toolchain / THEN selection is refused with `StepError::Unsupported("affected-selection")`
  and the pack routes to an explicit whole-suite policy / AND no `basis: affected` record can be produced for that
  toolchain while the probe entry is absent or stale."* *Verified by:* *"capability-registry unit test; a fixture
  adapter with an expired `verified_on` that must produce `Unsupported`; an assertion that no run under that fixture
  emits a record with `basis: affected`."* **Both** toolchain adapters ship `affected-selection` as a registry entry
  with **no probe** and yield `Unsupported` — no affected-selection probe result exists at any pin, on the tree or in
  any record [measured 2026-09-17: `grep -rn` over `references/`, `runbook/`, `spikes/*.md` finds none], and a row
  with an invented `verified_on`/`probe` is exactly what AC-489/AC-492 refuse; `ac328_affected_selection_unsupported_without_probe`
  runs in both crates. A Node affected-selection probe (`vitest --changed` / `related`) is **OWED as a named spike**
  (§7c.16, §7c.22 ARMED).
- **AC-329 (adapter half)** — SPLIT: the field-semantics half was discharged at 7b-ii (`b213137`,
  `rk_types::verify::VerifyReport`, nine required members, no `Default`) — **this is the matching note `s7b-ii`
  §7b-ii.9 said was OWED in `s7c`**. *"GIVEN a `VerifyReport` produced by any toolchain adapter / WHEN it is validated
  at the boundary / THEN it carries `basis`, `basis_reason`, `argv`, `selected_test_count`, `total_test_count`,
  `wall_secs`, `toolchain_version` and `suite_sha`, and validation fails if any is absent … / AND no field of the
  report is obtained by parsing prose or markdown (brief §C.5 forbids parsing JSON out of prose)."* *Verified by:* *"schema conformance test over every adapter's
  fixture reports; a negative fixture missing each field in turn; a lint forbidding markdown parsing in the toolchain
  adapters."* Counts come from structured test output — libtest's stable `--list` (Rust; RT-26, above) and vitest's
  JSON reporter file (Node) — never prose. **And the fixture projects carry a decoy that makes a prose parse
  observable, placed where the toolchain prints it unconditionally** (the third review: libtest captures a passing
  test's stdout, so a decoy printed from a test body never reaches the adapter unless it runs `--nocapture`, which
  nothing says it does): the Rust fixture project carries a **`build.rs`** emitting
  `println!("cargo:warning=test result: ok. 99 passed; 0 failed")` **and** `println!("cargo:warning=test phantom ...
  ok")`, which cargo echoes on stderr on every build of the fixture — the `--list` pass included — as `warning:
  <pkg>@<ver>: test result: ok. 99 passed; 0 failed`; the Node fixture's decoy is `console.log('Tests  99 passed')`,
  which vitest echoes. So a summary-line parser reports 99, an unanchored `... ok` line-counter over-counts by one,
  and the structured source reports 2 — that is `ac329_…`'s red on both sides, since a `selected_test_count: 2`
  parsed from prose is byte-identical on the report to one read from `--list`. The register's third instrument,
  *"a lint forbidding markdown parsing in the toolchain adapters"*, is the block's `rk_absent` over the summary and
  per-test spellings — `\.\.\. ok|\.\.\. FAILED|test result:|[0-9]+ passed;|Tests  [0-9]+ passed` (the second
  review's bare `passed;` matched any statement ending in a field named `passed`, which the honest adapter reads —
  Halt 7's too-wide guard) — with the control `ac329_prose_parse_control.rs` (§7c.21).
- **AC-325** — ARMED from 7b-ii to 7c. *"GIVEN a toolchain selection command that passes through a wrapper … / WHEN
  the test process is executed / THEN the argv actually handed to the OS process API is recorded on the run event as
  an argument vector, not the templated command string / AND re-executing the recorded argv verbatim reproduces the
  same executed-test count."* *Verified by:* *"an adapter test asserting the recorded argv differs from the template
  when a wrapper rewrites it; a fixture wrapper that drops its trailing arguments, which must be visible in the
  record; a re-execution test on the recorded vector asserting the same test count."*
- **AC-497 (classify half)** — **PROVISIONAL (S-BIN unrun)**; *"both Phase-1 Toolchain adapters … implement `classify`
  and leave `change_summary`'s body to S-BIN"*. `ac497_classify_binary_paths` over a `.uasset`/`.umap` fixture;
  `change_summary` returns `Unsupported` (routed).
- **AC-334 (real rows)** — ARMED-FROM-7b-ii; **the F25 fixture is a vitest suite, so its real-row run is the NODE
  adapter's**: `crates/rk-kernel/tests/fixtures/findings/ac015_inversion/record.json` reads `"suite": "npx vitest run
  test/implement-contract.test.ts"`, `"suite_filter": "-t \"AC-015\""`, over the JavaScript predecessor — a
  `cargo test`-driving Toolchain cannot execute it. `ac334_flip_predicate_real_toolchain` is therefore `-p
  rk-adapter-toolchain-node` behind the `corpus` feature — **the feature gates the TEST MODULE
  (`#[cfg(feature = "corpus")] mod ac334;` in `tests/it`), never the edge**: `rk-kernel` is an unconditional
  dev-dependency (cargo refuses `optional = true` on `[dev-dependencies]` outright, so *"dev, `corpus` only"* is a
  manifest that does not parse), and dev edges are not normal edges, so `cargo tree -p rk-kernel -e normal` and the
  AC-484 walk are untouched. **The predecessor checkout is materialised ONCE, into a persistent local-only path, by a
  named preflight run before Checkpoint 7c — never by the test and never into a temp dir**: `target/predecessor-f21f55fc/`
  (git-ignored), cloned from `$CORPUS_REPO/predecessor-47faec9f.bundle` and checked out at
  **`f21f55fc355eb49b42faf2eb07cad266b647ec1e`** — the F25 record's own SHA (`record.json`'s provenance names it; its
  inversion patch's hunk is `@@ -1797,7 +1797,7 @@` there, and at `REDKILN_SHA` `47faec9f…`, of which it is an
  ancestor, the same text has moved to :1917, so the recorded measurement is at `f21f55fc`), then `pnpm install
  --frozen-lockfile --dir target/predecessor-f21f55fc` (network; local-only; **no junction to the live clone's
  `node_modules`** — CORPUS_PIN says corpus-bound tests never read the machine-local clone). The test reads that path
  from a `corpus`-feature constant, **applies the inversion through `git -C $PRED apply` and reverts it with `git
  -C $PRED apply -R` in a `Drop` guard** (so the persistent path is byte-identical before and after every run, a
  killed or failed attempt included — the third review: a checkout left carrying the inversion would make the next
  run's `apply` refuse, or make the *mutation survived* baseline the mutant itself and invert the verdict silently),
  **asserts `git -C $PRED status --porcelain` is empty after the revert** inside the test, and runs `npx vitest run
  …` **under `run_bounded` with the Node adapter's construction-time timeout** — the bound Halt 5 wants; the block's
  preflight lines refuse when `target/predecessor-f21f55fc/node_modules/.bin/vitest` is absent (§7c.21), for the
  same reason the fixture project's does (`npx` without `node_modules` prompts on a closed stdin), **and when the
  checkout is dirty** (a previous run left the inversion applied — the operator runs `git -C $PRED checkout -- .`
  and re-runs; Halt 6's shape, the same tree reused across arms). **The Rust
  adapter has no ordering-inversion fixture**: nothing authored is called F25, and AC-334's Rust half is **ARMED**
  (§7c.22) until a Rust inversion fixture is authored and labelled authored.
- **AC-344 (adapter share only)** — **CROSS-LISTED with 7d's verify runner; not discharged here.** Its register
  subject is `rk-kernel-verify` and its instrument is *"a cannot-run fixture (missing binary) that must yield
  `Unverifiable`"* — the gate's `Unverifiable { cannot_run }`, a class `rk_types::StepError` does not carry, so no
  adapter can produce the outcome the row names. **The adapter's share** is `ac344_missing_binary_is_not_a_candidate_failure`:
  an absent `cargo`/`npx` is `Transport { kind: Spawn }`, never a failing test or `NonZero` — the input 7d's verify
  runner maps to `Unverifiable { cannot_run }`; the gate half is 7d's (§7c.22). **The fixture is an ABSOLUTE nonexistent program path, and each
  toolchain adapter has a `program: PathBuf` construction value to point at it** (as `harness_binary` is for the
  Harness adapter, §7c.12; the per-OS `npx.cmd`/`npx` resolution above lives in that value's default, which
  `d34_…` reads): a *"PATH-less child env"* cannot induce the condition — measured 2026-09-17 with a scratch
  program, `Command::new("cargo").env_clear()` spawns (exit 0) with no `PATH` in the child env and with
  `PATH=C:\nonexistent`, because `std` on Windows resolves a bare program name against the application directory,
  cwd, the system directories and the *parent's* `PATH` regardless of the child map — so the test's red is
  `<tempdir>/no-such-cargo.exe`, which `CreateProcessW` refuses deterministically (`ERROR_FILE_NOT_FOUND`).
- **AC-499** — BINDING for three rows, local-only for the fourth. *"WHEN the shared per-port conformance suite runs /
  THEN the same assertions execute against both implementations and both pass / AND an assertion that passes on the
  fake but fails on the real adapter is reported as a conformance divergence naming the assertion."* *"four rows in
  Phase 1: SQLite, Claude, Node, Rust … The Harness real row … is a **local-only, named-runner** row."* The 211-line
  file named `fake_conformance`, which exists nowhere; the suite is `rk-ports`' `port_conformance_fake_rows` with
  `PHASE_1_REAL_ROWS` empty. **Its second instrument, *"a CI matrix row per instantiation"*, is neither built nor
  claimed**: the stage's `ci.yml` edits add no matrix rows and `ci.yml` has never run, so that instrument sits in
  the ARMED table (§7c.22) and only the three by-name block lines are claimed. Real rows are
  `port_conformance_real_row_sqlite` / `_node` / `_rust`, each running the
  **same body** from `rk-ports`' `conformance` feature module (RT-16 as re-amended, §7c.1) — `conform_toolchain`'s
  **required core** is `detect` + `classify` over the row's `(path, classification)` pairs plus `build`/`test`/`lint`
  over the fixture project **with a count assertion the row parameterises** (`expected_test_count: u32` — 2 for the
  fixture projects, 0 for the fake), so an adapter whose `build`/`test`/`lint` return `Ok` without spawning anything
  is red; its **capability arm** takes the row's registry and asserts `Err(Unsupported { capability })` from the
  dated entry for `change_summary` (the unchanged body's `.expect("change_summary(range)")` would panic on every real
  row D17-9 governs) — and `PHASE_1_REAL_ROWS` is filled per slice (Checkpoint 7a provisional). **Two things the core
  needs stated once:** (1) **`Toolchain::test`'s contract on a red suite is `Err(StepError::NonZero { .. })`**, and
  `Ok(count_ran)` only on a green suite (RT-22, §7c.1 — `toolchain.rs` is silent on it; AC-331's *"attributable to the
  candidate"* needs a failing test distinguishable from a killed run, which an `Ok(count)` over a red suite erases);
  (2) so the core's unselected `test(&[])` never meets a red suite, **the fixture projects' failing test and the slow
  (AC-331) test are `#[ignore]` / skip-marked and reachable only by selection** (`-- --ignored <name>` / `-t <name>`)
  from `ac329_…` and `ac331_…`, and the default `cargo test --workspace` / `npx vitest run` over each fixture is green
  with two tests counted (the two passing tests, the decoy among them). **The Harness row runs the same body too:**
  `conform_harness(harness, store, spec: &ActivitySpec)` is parameterised by a **spec**, not a registry (RT-16 as
  amended again, DECISIONS-2026-09-17) — the fake row passes `spec()`, and `live_dispatch_probe` (§7c.15) passes its
  bound probe-card spec and calls the body **before** its own prints; **`conform_harness` itself prints
  `PROBE_RAN_CONFORMANCE=1` as its last statement** (the third review: a token the probe printed about itself was
  satisfiable by a probe that never called the body), so the block's grep is evidence the body ran to its end;
  without that a fourth row would be claimed by a test that never ran the shared body.

### 7c.15 The live probe — the fifteenth line, and AC-511

**DECISIONS-2026-09-17 D17-5.** The real dispatch runs **in this session, foreground, operator present**, driven by
an `#[ignore]` test `live_dispatch_probe` in `crates/rk-adapter-claude/tests/it/`, behind `--features live`, invoked
as `cargo test -p rk-adapter-claude --features live -- --ignored --nocapture live_dispatch_probe`. It reads the
process environment **once, at its own edge** (`std::env::var` is not clippy-banned and needs no `#[expect]`;
`set_var`/`remove_var` are banned; no adapter `src/` file reads the environment — the block's AC-217 ban — and the
probe is in `tests/it`, outside that ban's scope), builds `Config` (`TESTED_CLI_VERSION = "2.1.275"`, both TTLs, the
captured map, `harness_binary = "claude"`), **initialises a temp git repository with a root commit** (so AC-369's
identity rows can be written) and dispatches the haiku probe card (§7c.12) into a temp `rk-adapter-sqlite` store
(`rk-adapter-sqlite` and `rk-git` are **dev**-dependencies of `rk-adapter-claude` for this — a dev edge between two
adapters never reaches `rk-kernel`'s normal graph, which the AC-484 walk follows by normal/build kinds only; the
adapter has **no normal `rk-git` edge** since the pre-spawn snapshot is not lifted, §7c.2; the probe mints its
`GitExec` with `rk_git::version::assert_at_startup` over a temp hooks directory, as the sqlite tests do; §7c.19).
**The probe card's `timeout_secs` is 300 s — a bound chosen, not a class measurement**: the smaller of the two
literal timeouts most common on `S`-class catalog activities under `spikes/p9/catalog` (`600` and `300`; the first
form of this sentence gave per-file counts that did not reproduce under any stated counting rule and they are
withdrawn) — and its expiry is the adapter's own timeout arm (§7c.10), which kills the child tree through the group:
that is the wall-clock bound on the block's `$(cargo test …)` capture and Halt 5's failure alternative, named on the
line. **The shell it runs from is a Git Bash NOT spawned by Claude Code**: a Claude-spawned shell carries
`CLAUDECODE=1` and a **session-dependent** set of `CLAUDE_CODE_*` names into the captured map (8–11 measured across
two sessions on 2026-09-17; the guard keys on the prefix, never on a count), and the block's **first** preflights
refuse such a shell, and one carrying `ANTHROPIC_FOUNDRY_API_KEY` or a tracker name, before any gate runs (§7c.21);
it runs under `env -u ANTHROPIC_FOUNDRY_API_KEY bash` (§7c.6 — the persisted user-scope credential name the denylist
carries, so `Config::new` would otherwise refuse, as it should). **The probe runs the shared conformance body
first**: `conform_harness(harness, store, &probe_spec)` from `rk-ports`' `conformance` feature module (§7c.14), which
**itself prints `PROBE_RAN_CONFORMANCE=1` as its last statement** — the AC-499 Harness row is the same body over the
real adapter, not a body of the probe's own, and the token is the body's, not the probe's. **What the probe prints,
each value on its own line and read to end of line (a temp path may carry a space), and the block reads:**
`RECORD_PATH=<file>` — the probe's own text file holding the raw `system/init` line, the raw `result` frame and the
`dispatch_completed` row **serialised with `serde_json::to_string` (compact, one line, no space after a colon)** —
the form the block's `instrument_version` grep is written against, and it is written shape-tolerant anyway;
`RESULT_RAW_SHA256=<hex>` — **the sha256 of the bytes the probe wrote to `RESULT_RAW_PATH`**, so the block can
assert the raw file it reads is the frame the probe retained (a digest, the row's own carrier per ADR-0105 clause 7
— the second review's `grep '"structured_output"' $rec` asserted nothing, since the key is present on every path);
`ARGV_PATH=<file>` — **the argv vector the builder produced, verbatim, one element per line** (the row carries argv
as a digest only — ADR-0105 clause 7 — so AC-215's tag lift needs its own carrier, and this is it);
`OBSERVED_CLI_VERSION=<v>` — **the version taken from the raw `system/init`'s version field**, printed on its own so
the block can compare it to the pin (a grep for the pin over `RECORD_PATH` would find `instrument_version`, which IS
the pin, on the serialised row whatever the child reported — Halt 11's shape inside the one line meant to tell them
apart; the field's name is [surface unverified] until this print names it, §7c.11); `RESULT_RAW_PATH=<file>` —
**only** the verbatim `result` frame, so Halt 3's assertion reads the raw payload and not a typed field that may
serialise as `"structured_output": null`; `STORE_PATH=<file>` — the SQLite file; and `COMPLETED_ROWS=<n>` /
`DISPATCH_ROWS=<n>` — counts the probe takes **after** the dispatch through the real `EventStore::query`, so the
block can assert exactly one `dispatch_completed` row was appended rather than `test -s` on a file whose schema page
alone makes it non-empty; **`DISPATCH_ROWS` is asserted `0` with its reason** — the probe calls `Harness::dispatch`
directly and no kernel runs, so the kernel's pre-dispatch `dispatch` row is never written there. **Where the raw
frames live afterwards:** `RECORD_PATH` and `RESULT_RAW_PATH` are temp files the block `test -s`es and nothing keeps,
and AC-465 forbids a tracked file under `spikes/results/` — so the probe's raw `system/init` and `result` frames take
**the route O-9 took for S-TTL1**: a new commit in the corpus repository (`D:/repos/redkiln-corpus`), named by SHA
with bytes + sha256 in the close commit, never moving `CORPUS_PIN`; AC-202/AC-204's fixtures and AC-511's field read
that commit through `git show <sha>:<path>`. It is the AC-499 Harness row and is **never wired into CI**. The block builds fresh first (`cargo test` does not rebuild — Halt 1 / golden constraint 3), captures `claude
--version` and compares it to the re-teed `RUNBOOK-instrument.txt` (Halt 2), `test -s` every path, and asserts
`structured_output` **from the raw retained result frame** as a non-empty object or list, with `subtype: success` and
`is_error: false` from the same frame (Halt 3). Cost: one haiku dispatch (≈ $0.03 by ADR-0113's F3 figure). A second
`--features live` `#[ignore]` card, `live_disallowed_tool_terminates` (AC-215's integration half), is **authored and
not run** at 7c (§7c.5).

- **AC-511** — BINDING first conjunct; **[surface unverified]** second. *"GIVEN any argument vector the Claude adapter
  builds for a dispatch / WHEN the adapter lint runs and the test suite's recorded dispatch events are read / THEN
  `--bare` appears in no argument vector, in source or in any recorded dispatch / AND the session's auth mode is
  asserted from the child's own `system/init` report to be the subscription session, not inferred from the argument
  vector alone …"* *Verified by:* *"a source lint over argument-vector construction; an event-log assertion over the
  suite's recorded dispatches; a fixture code path that adds the flag, which must fail the lint naming the call
  site."* *"the `system/init` field that names the session's auth mode is **unnamed in the corpus** … so the second
  AND is **[surface unverified]** until the one-dispatch probe in runbook Step 7c's preamble retains a raw
  `system/init` payload and names it; if none exists the conjunct is dropped and a named open item with
  `rk-adapter-claude` as consumer takes its place."* `apiKeySource` appears in **zero** corpus files at the pin and the
  runner read no auth field [measured 2026-09-17]; the field name is **UNCONFIRMED** until the probe's raw
  `system/init` is read. The close commit either names it (and the probe test asserts on it) or records the named
  open item in `spikes/DECISIONS-2026-09-17.md` — the block asserts only that the raw `system/init` was retained.

### 7c.16 Spikes run here, spikes cited, and the AC-312 fixtures

**Run at 7c (D17-6):** **S-CANCEL-WIN** — one fixture-cancel-rm run on Windows, the Halt 6 handle case; which kill
path it exercises is **UNCONFIRMED** (design §10.4 names a mid-vitest victim, §10.5 frames it as the Harness Job
Object cancel) — both are run: the Job Object arm through `rk-containment`'s own `orphan_grandchild_reaped_via_job`
and `breakaway_attempt_fails_without_breakaway_ok` at **Slice 2** (§7c.10 — `cancel_reaps_process_tree` is Slice 5's
and cannot be Slice 2's instrument), and the
mid-`vitest` victim under `run_bounded`'s timeout path (O-10's default) at **Slice 6**, after the Node adapter and its
fixture project exist; the result is written into the design §10.5 row with the victim named; $0. **S-RETRY** — its throttle-margin and
exhaustion-on-result arms (haiku telemetry only; consumer the exhaustion classifier, §7c.7); the payload-capture arm
was discharged. **S-TTL1** — narrowed by ADR-0113 decision 3 to a subagent-card dispatch with both TTL variables set
(*"does a dispatch carrying a subagent card report `ephemeral_1h_input_tokens > 0` for the subagent's own cache
creation, and does the same dispatch on usage-credit billing report 5 m?"*), raw `usage` and `rate_limit_info`
retained verbatim, ≈ $0.03 per dispatch; the card must grant subagent tooling. **Its raw frames are retained in the
corpus repository as a new commit, without moving `CORPUS_PIN`** (O-9's default) — so after that commit the corpus
checkout's `HEAD` is no longer the pin, and Checkpoint 7c's precondition is therefore written as AC-464 states it
(*"resolves through this SHA via the object store, never through a working tree"*): `git cat-file -e
"$CORPUS_SHA^{commit}"` plus a clean tree, never `HEAD` equality; every corpus read in this file is `git show
$CORPUS_SHA:<path>`. Each is run or its deferral recorded in the close commit. **Registered, not run:** **S-SPF** — `--system-prompt-file` cache-prefix sharing; the argv cap ships
and the file route stays unused (DECISIONS-2026-09-11 #18). **Consumed elsewhere, cited not run:** **R-SQLITE** — its
empirical two-process contention half was **consumed at Step 6** (AC-181, `crates/rk-store/tests/it/contention.rs`;
`PROGRESS.md` records it 2026-09-13); what remains — whether 5000 ms suffices under the self-hosting run's load — is
Step 8 (3). Design §10.5's *"Not started"* row is stale and gets a **dated note**, not a run. **Not 7c's to run but
gating its labels:** S-H1c (AC-211's PROVISIONAL), S-BIN (AC-497's `change_summary` body). **OWED as a named spike,
registered here:** **S-SEL-NODE** — a Node affected-selection probe (`vitest --changed` / `related`) at the pin, the
one thing that could put a `probe` on either toolchain's `affected-selection` row (AC-328; §7c.14); until it runs
both rows carry no probe and yield `Unsupported`. **Enabled by 7c, not
owed:** S-SCHEMA-CAP (`rk-types`'; its two dispatches need this adapter), S-FEAT (the `cargo tree` lines cite it).

**AC-312** — **PROVISIONAL (S-SUBTLE-CB unrun)**; DECISIONS-2026-09-11 #17. *"GIVEN a release regression corpus
containing both negative-control fixtures (no seeded defect), seeded `reproduced` fixtures, and the **prior-approval
framing variants** (known-defective items whose card asserts the change was reviewed and approved — authored at
runbook Step 7c, unrun until S-SUBTLE-CB draws) / WHEN the review gate runs the release regression / THEN **every
seeded `reproduced` fixture produces a block and every control produces zero.**"* 7c **authors** the framing-variant
fixtures under `crates/rk-adapter-claude/tests/fixtures/framing/` and leaves them unrun; the only honest checkpoint
line is file existence; **no false-approval-guard claim** (ADR-0101 clause 5). **7b-ii's lesson applies here, and
it is the halt this stage is likeliest to hit**: the block's ADR-0101 grep is a substring rule that cannot read a
negation — Checkpoint 7b-ii's first run failed on `src/review.rs:31`, which carried *"guarded against false
approval"* inside the sentence denying it, and `164e212` reworded it (`PROGRESS.md`: *"The rule is a substring rule
over the crate and cannot read a negation; it was right to fire"*). So **the phrase may not appear in a 7c crate
even to deny it**; the `framing/` fixtures' module docs and file headers say what the fixtures *are* (an approval-
framed card, a defective item), never what they do not claim.

### 7c.17 Where the fixtures live

**No raw `claude -p` stream transcript exists at the pin.** `git grep -c '"subtype": *"init"' -- results` over the
corpus at `61f590e5…` returns **0 files**; `results/e3.jsonl`'s rows are runner-typed (`step.{result_raw,
unknown_events, event_census, effective, flags, …}`) and the runner retained only the parsed `EffectiveConfig`
[measured 2026-09-17, critic gap 1]. **And what the runner did retain is its own re-serialisation, not the CLI's
bytes:** at the pin `StreamLine.raw`, `step.unknown_events` and `step.result_raw` are `serde_json::Value`
(`runner/src/event.rs`:18, `runner.rs`:66 and :104; `runner/Cargo.toml` enables no `preserve_order`), so every
"recorded" frame in the corpus is a parsed object written back with **alphabetically sorted keys and `f64` numbers**
— the F20 `result_raw` on the tree reads `api_error_status, duration_api_ms, …, type, usage, uuid` in sorted order
[measured 2026-09-17]. So every "recorded stream fixture" the register names is re-stated here as **recorded result
frame / recorded unknown lines (the runner's re-serialised CONTENT) + SCRIPTED `system/init`**, and **every stream
line in a fixture is labelled** for what it is: `// RE-SERIALISED at the pin (runner Value; key order and number repr
are not the CLI's)` for a line reconstructed from a `step.result_raw` or `step.unknown_events` value; `// SCRIPTED —
not recorded at the pin` for an authored `system/init`, a non-`allowed` `rate_limit_event` or a usage-limit result;
`// RECONSTRUCTED from <path>:<line> — raw line not retained at the pin` for a line re-synthesised from the runner's
parsed fields (AC-232's two `system/api_retry` lines, §7c.11); and `// SYNTHETIC` for a value no recorded row carries
(AC-374's trailing-zero digits, §7c.8; AC-218's exit-0 cell, below). `rate_limited_from_stream`'s `resets_at` source
(a non-`allowed` `rate_limit_event`) is unrecorded at the pin and carries **[surface unverified]** beside the
`subtype` arm until the probe or S-RETRY retains one. What **is** recorded, read via `git -C "$CORPUS_REPO" show
"$CORPUS_SHA:<path>"` [all measured 2026-09-17]:

| source at the pin | bytes | sha256 | what it is |
| --- | --- | --- | --- |
| `results/e3.jsonl` | 3,991,124 | `2547a5c08624986099f795bf878274cdbe6e84104a171d85103b8e27be02211f` | 101 rows; the **seven** `"is_error":true` result frames at lines 14, 16, 17, 18, 19, 21 and 26 (AC-218/AC-428) — **every one of them `exit_code` 1, `subtype: success`, `api_error_status` 500 or 529** [measured 2026-09-17], a fact worth recording under AC-523's subtype caveat: a `success` subtype coexists with `is_error: true` at the pin; line 6 with two `api_retries` (AC-232); `step.result_raw` frames for the AC-439/AC-428 replay |
| `results/instrument-checks/e1-smoke.jsonl` | 3,091 | `30d9e67d1ac8583e0bb8c7ce24ffd561940e010ec69452bbfcdfe4b61db8361a` | 3 rows; the **only** recorded `rate_limit_info` payload (`unifiedWindows.{five_hour,seven_day}.utilization` 0.04; top-level `status`/`resetsAt`/`rateLimitType`/`isUsingOverage`/`overageStatus`/`overageDisabledReason`) — carried inside `unknown_events`, i.e. **the runner's re-serialised unknown line, one `f64` round-trip from raw**, not the CLI's bytes — AC-374's round-trip provenance half (the discriminating half is synthetic, §7c.8), AC-380's both-shapes, AC-221's flip-to-true, AC-200's recorded unknown lines |
| `results/h1-gap.jsonl` | 33,834 | `4fd395ab2fb56332d99427ad5d669bc3882e3490960d7234f10df0e45082f8cb` | F28's `default` arm — the golden `default` argv (§7c.5): the six dispatch rows' `flags` arrays |
| `results/h1-sysprompt.jsonl` | 11,417 | `0ec4f17f38f0301f6f74a98423f60f622cc73d2bf3fed35fbb41c8a0e2edc43d` | the `static` arm — the golden `static` argv with a 45-byte `--system-prompt`: the two dispatch rows' `flags` arrays |
| `crates/rk-kernel/tests/fixtures/findings/ac282_f20_golden_transcript.jsonl` (already on the tree, 7b-ii's copy) | 24,794 | `457cfa4abc925c28799c8cda47666c1acd5841953ab4616e22c54685d39a33ee` | the F20 row: `step.result_raw` (exit 0, `subtype: success`, `is_error: false`, **no** `structured_output`, `--json-schema` on argv) for AC-206/AC-439/AC-282; `step.unknown_events` with five `tool_progress` frames for AC-200 — **the runner's re-serialised unknown lines** (keys `elapsed_time_seconds, heartbeat, parent_tool_use_id, …` in sorted order [measured]), the same kind of thing as e1-smoke's frame, so AC-200's byte-for-byte replay is over the runner's bytes on both rows and the first CLI-verbatim unknown line is the probe's |

**What is copied into the tree, and why not the sources whole.** AC-465's THEN is *"no corpus artifact (… E3
fixtures, spike JSONL) is tracked in the product repo"* and its *Verified by* adds *"a fixture commit re-adding a
JSONL results file must fail"*; `ci.yml`'s corpus-residency rule fails on any tracked `*.jsonl` except
`RUNBOOK-instrument.txt`. Copying the 4 MB `results/e3.jsonl` (101 rows, of which seven frames and one line are used)
and three more spike JSONL files into `crates/*/tests/fixtures/` would re-create the violation ADR-0013's split
removed, by the criterion's own wording — and the first form of this table said *"copied with bytes + sha256"* of the
whole files. So **only the extracted rows the tests read are copied**, into small fixture files under
`crates/rk-adapter-claude/tests/fixtures/recorded/`, each named for its source and line (`e3.L14.result_raw.jsonl`,
…, `e3.L6.api_retries.jsonl`, `e1-smoke.L1-3.jsonl`, `h1-gap.dispatch-rows.jsonl`, `h1-sysprompt.dispatch-rows.jsonl`),
each with **its own bytes + sha256 recorded beside the source's** in a sibling `PROVENANCE.md` that names
`$CORPUS_SHA:<path>:<line>` per file. **That copying is a ROUTING, not a reading the stage may take on its own
(RT-23, §7c.1)**: AC-465's THEN names *"E3 fixtures, spike JSONL"* and its AND says *"the only reference to them is
the pinned SHA"*, and whether an extracted row is an *"E3 fixture"* is decided by no record — so the copies are
routed with **a dated in-place amendment to AC-465 OWED**, naming what its *"pin-manifest allowlist"* may hold
(§7c.22 RAISED). Those paths are added to `ci.yml`'s AC-465 rule as a named allowlist **in the stage-open commit**
(§7c.20 item 0), asserted by name rather than by silence, the way `RUNBOOK-instrument.txt`'s exemption is; and the
three `*.jsonl` fixtures `rk-kernel` already tracks (`findings/ac282_f20_golden_transcript.jsonl`,
`logs/implement-bf1d8cec3b22d288/events.jsonl`, `retry/api-retry-rows.jsonl` [measured 2026-09-17, `git ls-files
'*.jsonl'`]) are **named in that allowlist as 7b's and 7b-ii's debt, with a comment saying so** — 7b-ii's single-row
F20 copy already collides with the pathspec and CI has never run — **not folded into 7c's rows silently**.
Anything larger stays behind `--features corpus` and is read via `git show $CORPUS_SHA:<path>`.

Only the five sources above are recorded; **`e0-headroom.jsonl` and `s-var.jsonl` are not** — they carry the runner's
parsed `f64` `headroom` object, not the raw payload (critic gap 5). **AC-218's four-way matrix is labelled for what
it is, cell by cell** (the second review's *"three cells over the seven recorded frames"* did not reproduce: seven
frames that all carry `exit_code` 1 and `is_error: true` populate ONE cell): **the non-zero × set cell** over the
seven recorded frames; **the exit-0 × unset cell** over the F20 `result_raw` (exit 0, `is_error: false`); **the
non-zero × unset cell** — no recorded frame is identified for it at the pin (whether one exists in `results/e3.jsonl`
is UNCONFIRMED — the seven `is_error` frames were counted, the unset non-zero frames were not) — so it is authored
and labelled `// SYNTHETIC — no recorded exit≠0 / is_error-unset frame identified at the pin`; and **the exit-0 ×
set cell**, the discriminating one, is **not recorded**: its exit code is substituted and the cell is labelled
`// SYNTHETIC exit code — every recorded is_error frame exited 1 at the pin`. `ac218_…` names each cell's source
in its body. Everything else under `crates/<crate>/tests/fixtures/<cluster>/` is authored: `streams/` (scripted
`system/init` cases for AC-202/204/226/386 — including one with the **same tool count** as the card and one
substituted name, so a count-based isolation check goes red (AC-202); the AC-201 future-shaped line; the AC-523
synthetic frames labelled as such; the RECONSTRUCTED `api_retry` pair; **the ≥ 1 MiB prompt fixture** that makes
`Transport { Io }` deterministic, §7c.11); `cards/` (§7c.12); `argv/` (the at-the-cap fixture, the golden vectors);
`framing/` (AC-312 — at least a defective item **and** its approval-framed card, so two non-empty files); `usage/`
(the synthetic absence fixture for AC-362); `registry/` (the bare-row and `reads_minds` red registries for
AC-489/492); `worktrees/` (AC-370's three-worktree repo builder); `pack/` (the Rust technology pack, §7c.14);
**`project/`** under each toolchain crate (§7c.14: the Rust crate with its own `[workspace]` table and **two
passing tests — one of them the AC-329 decoy that prints `test result: ok. 99 passed; 0 failed` — plus a failing test
and a slow test, both `#[ignore]` and reachable only by `-- --ignored <name>` from `ac329_…`/`ac331_…`**, so the
conformance core's unselected run is green at count 2; the Node package with `package.json`, `pnpm-lock.yaml`, one
`*.test.ts` with **two passing tests — one the decoy `console.log('Tests  99 passed')`, one filterable by `-t`** —
a skip-marked failing test and a skip-marked slow test reachable by `-t <name>` only, and the AC-325
argument-dropping wrapper); **`bin/` — the spawnable fixture child**, declared `[[bin]] name = "rk-fixture-child"
path = "tests/fixtures/bin/rk-fixture-child.rs"` in **`rk-containment`** and, **from the same source file** (`path =
"../rk-containment/tests/fixtures/bin/rk-fixture-child.rs"`), a second `[[bin]]` **named `rk-fixture-child-claude`**
in **`rk-adapter-claude`** — a **different name from one source**, because two workspace packages declaring one
`[[bin]]` name compile to one output path (`target/debug/rk-fixture-child.exe`): cargo warns *"output filename
collision"* (rust-lang/cargo#6313), the two link jobs race for the file, and on this box with the MSVC linker
`cargo build` exits 101 with `link.exe … exit code: 1104` for **both** packages [reproduced 2026-09-17 in a
two-package scratch workspace] — Halt 1's own first line goes red before any test runs. `CARGO_BIN_EXE_<name>` is
set only for the owning package's own tests and `cargo test -p rk-adapter-claude` builds no dependency's bins, so
each spawning crate reaches its own copy through the compile-time `env!("CARGO_BIN_EXE_rk-fixture-child")` /
`env!("CARGO_BIN_EXE_rk-fixture-child-claude")` (never `std::env::var`, never `current_dir`). **The shared source
uses only `std`** — it depends on no crate, names no `windows_sys::` symbol and no `unsafe` (the `rk-adapter-claude`
copy compiles against that crate's dependencies, which carry no `windows-sys` row, and under the workspace's
`unsafe_code = "deny"`): the second review's three library wrappers it called (`is_process_in_job`,
`job_kill_on_close_is_set`, `spawn_breakaway_attempt`) are **gone** — the two job queries because the child-side
reading is no instrument under cargo's own job (§7c.10), and the breakaway spawner because a public library function
whose purpose is to spawn a child that tries to leave containment is outside ADR-0127 decision 2's surface and is
the shape ADR-0099's forbids names; the breakaway attempt is **the bin's own** — `std::process::Command::new(<self>)
.creation_flags(0x0100_0000 /* CREATE_BREAKAWAY_FROM_JOB */).spawn()`, a plain `u32` `std` takes — under its own
`#[expect(clippy::disallowed_types, reason = "…")]`, as its `orphan` spawns are. **Those spawns are not a violation
of ADR-0127's forbids, and the record's own text says so** (the third review: a stage may not narrow an accepted
record's forbids by reading, and 0127 was untracked at the time, so its first forbids bullet was worded before the
stage-open commit — *"except through `rk-containment` in those crates' library code; a test fixture binary that
spawns its own descendant to exercise containment is the instrument of AC-224/AC-278, not a violation"*); no OWED
note against the record is needed and none is carried. The bin is not under `src/`, so no ban reads it, and the
library ban `\.(spawn|output|status)\(\)` over the three adapters' `src/` (§7c.21) is what polices the forbids.
**How it learns its mode** (§7c.12): from `RK_FIXTURE_MODE=
<mode>[:<arg>]` and `RK_FIXTURE_MARKER_DIR=<dir>` in the environment the adapter constructed — the argv is the golden
vector and carries no mode flag — read at the bin's own edge (it is outside every `std::env::var` ban's scope). Its
modes: `replay:<file>` (cat a SCRIPTED transcript to stdout — init line, optional `rate_limit_event`, result frame —
then exit), `exit:<code>`, `hold` (stay alive until killed), `close-stdin-early` (close stdin before the parent's
write — `Transport { Io }`, with the ≥ 1 MiB prompt), **`wait-stdin-eof:<file>`** (block on stdin until EOF, **then
replay the scripted transcript `<file>` and exit 0** — so the correct implementation's outcome is `Ok` and a
never-shut stdin hangs to the bound and `ac215_…` reads `Cancelled { by: Timeout }`, red; the second review's
EOF-then-exit form emitted no transcript, so `Ok` was unreachable on the correct implementation too), **`spew:<bytes>`** (write N bytes to
stdout then exit; N > 64 KiB is `run_bounded_kills_tree_at_timeout`'s pipe red — output through a pipe deadlocks,
DECISIONS-2026-09-09 #31), **`orphan`** (three-level, §7c.10: spawn an intermediate that spawns a **holder** —
which writes its pid marker and holds — and exits, then **hold** — so the direct child is alive when the token
fires and the holder's parent is dead; AC-224's positive control), `breakaway` (attempt `CREATE_BREAKAWAY_FROM_JOB`
as above — AC-224's negative control, must fail), `echo-env` (AC-509; also echoes its cwd, RT-21). **It asserts
nothing about the job it is in**: under `cargo test` every child inherits cargo's `KILL_ON_JOB_CLOSE` job (§7c.10),
so a child-side job reading discriminates nothing; the assign-order discriminators are the parent's, in
`spawn_into_job`'s test-only reading. It writes **one marker file per invocation** into `RK_FIXTURE_MARKER_DIR`,
**carrying `pid=<own pid>` and the argv it received, one element per line**, and in `orphan` mode the holder's own
marker carrying the holder's pid — so *"exactly one child spawn"* and *"dispatch count 0"* are file counts, `ac225_…`'s
external kill has a pid to `rk_containment::terminate_pid` with `CANCEL_EXIT_CODE`, and the liveness probe `ac278_…` and the
cancel tests read (`rk_containment::is_alive(pid)` — `OpenProcess` + `GetExitCodeProcess == STILL_ACTIVE` behind
the safe wrapper, §7c.10) is
**independent of the job the implementation under test created** — a mutant with no job cannot be enumerated *via
the job*, so the survivor check must not depend on it. It holds itself alive with `std::thread::park_timeout` /
`Condvar::wait_timeout` or a blocking `stdin().read_line`, **never tokio and never `sleep`** (ADR-0127 decision 2;
clippy.toml); and **`lint-controls/`** under `crates/rk-adapter-claude/tests/fixtures/` — the positive controls for
every absence ban in Checkpoint 7c, one file per ban, named for it (`ac211_append_flag_control.rs`,
`ac224_taskkill_control.rs`, …), **never under `tests/ui/`** (the trybuild glob root — `s7b-ii` §7b-ii.10's reason).
**Every control is checked through the ban's own pipeline** (`rk_control`, §7c.21: the same search and the same
comment filter, FAILing when no NON-comment line matches — a control whose token sits on a `//` line proves
nothing about the ban), and eight have a stated form: `adr0102_allow_attribute_control.rs` carries **both**
attribute forms — `#![allow(dead_code)]` (inner) and `#[allow(dead_code)]` (outer) — and never
`allow(unsafe_code)`, so the ban's `#!?\[allow\(` pattern is controlled on both spellings and the workspace-wide
exactly-one count is not disturbed by its own control; `ac362_derive_default_control.rs` carries **two** structs —
`#[derive(Debug, Default)]` then, on the next line, `pub struct Usage {}`, **and** `#[derive(Debug, Default)]` then a
plain `// SYNTHETIC: absent stays None` comment line then `pub struct WindowReading {}` — and `impl Default for
CacheCreation`, because the derive-Default walk (`dd_hits`, §7c.21) anchors on a line that BEGINS `pub struct
<Name>` and walks the attribute block above it, so a one-line `#[derive(Debug, Default)] pub struct Usage {}` would
report no hit, and a walk that stopped at a plain `//` line (the second review's did — only `///` and `//!` were
continuation forms) would miss the second struct's derive; both are asserted on the control; **the six structs the
walk searches are declared `pub struct` at column 0** for the same reason; `adr0118_poison_recovery_control.rs`
carries the canonical spelling `unwrap_or_else(PoisonError::into_inner)` (the one `rk-store`'s `conn.rs` uses);
`ac374_f64_headroom_control.rs` carries `as_f64(` beside `f64` (the rounding path's spelling, §7c.8);
`adr0127_direct_spawn_control.rs` carries a literal `.spawn()`; `ac202_argv_isolation_control.rs` carries
`build_args`; and `ac208_prose_scrape_control.rs` / `ac329_prose_parse_control.rs` carry `Regex::new(` and `test
result: ok. 99 passed` respectively. No shipped artefact is a fixture; `.redkiln/` does not exist.

### 7c.18 Red first

**Red first (golden constraint 12; ADR-0019).** `00-preamble.md`'s table names **Step 7c** among the stages that write
Rust. The checkpoint below is the floor, not the evidence: every `cargo test … -- <name>` line is satisfied by an
implementation that nothing pins. Author each named test **before** the code it describes, run it, and **watch it
fail for the right reason** — the absent behaviour, not a compile error, a typo'd filter or a missing fixture. One
bullet per checkpoint test name; the name binds the test to its id; halves are red separately; other-crate tests are
labelled with their `-p`. The five names the 211-line file introduced are kept; one of them
(`cancel_reaps_process_tree`) is the register's own (AC-224's *Verified by*), the other four are this file's.
**"Dispatch count 0" and "exactly one spawn" are file counts** — the fixture child's marker files (§7c.17). **What a
configuration-refusal test asserts, in order:** `Err(ConfigError::..)` from `Config::new` / `Binder::bind` — on a
strip-and-pass implementation that call returns `Ok`, so the `is_err()` assertion is red by itself — **and, if the
call returned `Ok`, the test goes on to dispatch once through the fixture child and asserts a marker count of 0**:
the marker count is the belt that turns a warning-not-refusal implementation red on the second assertion, and it is
vacuous unless the `Ok` branch dispatches, which every such test below does — **every refusal bullet below names
that red** (the second review left six without it: `ac526_…`, `ac212_…` case (c), `ac214_…`, `ac377_…`, `ac383_…`,
`ac493_…`). Asserting only the count with no `is_err()` would be green on a refusal that never spawns for an
unrelated reason; asserting only `is_err()` accepts an implementation that refuses and spawns anyway. Each bullet's
*"red by a marker file appearing"* means that second assertion. **When each half lands, since the belt needs the
spawn path Slice 4 does not build** (the third review; §7c.20): at **Slice 4** the refusal tests land with the
`is_err()` half only and are watched red on it; at **Slice 5**, once `runner.rs` and the fixture child's spawn path
exist, each gains its marker-count belt and is watched red on that half separately (a stubbed `dispatch` at Slice 4
would give the belt an `Ok` branch that reaches no spawn — a red for the wrong reason); both slice subjects name the
affected tests, so each R2/V3 body quotes the red it produced.

**`-p rk-adapter-claude`:**

- `api_key_refuses_construction` — AC-488. Red by a `Config` map carrying `ANTHROPIC_API_KEY` that constructs, or by
  a marker file appearing; a weaker red accepts a warning, or a map value read from the process environment.
- `structured_output_required` — AC-205/AC-417. Red by a card without `StructuredOutput` that binds, or by a marker
  file appearing; a weaker red accepts a runtime `Schema` error (F20's exit 0 is why it must be configuration-time)
  or a dispatch count of 1.
- `env_denylist_config_map` — AC-217. Red by a denylisted name that is **stripped** and passed through (a marker
  file appears); a weaker red accepts subtraction — the register says REFUSAL and dispatch count 0.
- `golden_argv_per_profile` — AC-211 (PROVISIONAL) / AC-383. Red by swapping the profiles' routes, by **any element
  out of the ONE stated order** (the lifted `build_args` order with `--strict-mcp-config` after `--permission-mode`
  and `--system-prompt` last under `static` — §7c.5), and separately by either append flag present; the body asserts
  **both** `static` and `default` were exercised (a count of 2) and fails if a profile is absent from the fixture
  list; a weaker red accepts either argv, compares element sets instead of order, checks only one append flag, or
  loops over a one-profile fixture.
- `argv_cap_refuses_before_spawn` — AC-523's cap. Red by the at-the-cap fixture binding (a marker file appears); a
  weaker red refuses only one byte over, or refuses after spawn.
- `rate_limited_from_stream` — AC-523 `text_marker` arm. Red by `resets_at: None` hard-coded (the runner's shape) or a
  backoff attempt; a weaker red accepts a `resets_at` read from the result frame.
- `ac523_text_marker_detection_disclosed` — red by a text-marker detection that records `detected_by: subtype` or
  nothing. **The zero-backoff table is not the adapter's to break**: `RateLimited → RetryClass::WaitUntilReset` is
  `rk_types::StepError::retry_class()` (Step 5) consumed by `rk-kernel`'s `retry.rs` (7b), and no adapter mutation
  moves it — as `ac386_…`'s zero-retry half already says — so `ac523_zero_backoff_table` is homed **`-p rk-types`**
  (the other-crates loop; no named test on the tree asserts that row today [measured 2026-09-17: `grep -rn
  'WaitUntilReset' crates/rk-types/tests/` → 0]) and the adapter keeps only what it can break: that the usage-limit
  result CLASSIFIES as `RateLimited` (`rate_limited_from_stream`).
- `ac523_subtype_detection_synthetic_frame` — PROVISIONAL, built, not claimed (`// SCRIPTED — the CLI's usage-limit
  subtype string is an assumption`; [surface unverified]); red by `detected_by` written as `text_marker`, or by
  `resets_at` not taken from the last non-`allowed` `rate_limit_event` (§7c.7).
- `tick_arithmetic_vectors` — AC-372. Red by `f64` subtraction on the **0.10/0.12** pair — under `f64` the delta is
  0.01999999999999999, which `< 0.02` flags, while ticks read 2 and do not [measured 2026-09-17 in Python] — or by
  `delta < 0.02` in any form; **0.09/0.10 and 0.20/0.21 agree under both rules** (`0.10 − 0.09` = 0.010000000000000009
  flags either way) and are provenance vectors, not discriminators, which the first form of this bullet had backwards;
  a weaker red uses only the tick pairs and never the decimal bases.
- `ac372_reset_crossing_splits_per_window` / `ac372_low_sample_at_n9` / `ac372_interleaved_not_attributable` — red by
  one delta across a reset, by a delta quoted at n = 9, by a per-item delta under interleaving.
- `cancel_reaps_process_tree` — AC-224 (register-fixed name; Windows arm live). The fixture child runs in the
  three-level `orphan` mode (§7c.10, §7c.17): it spawns an intermediate that spawns the holder (pid marker) and
  exits, and **the child itself holds** — so `dispatch` is in flight, blocked on the child, when **the test fires the
  token from a second thread**; the test reads the holder's liveness through `is_alive(pid)` **after `dispatch`
  returns**. Red by reaping only what a PID walk from the live child finds — **the holder is alive at return** (its
  parent is dead; only the job holds it) — by the job handle dropped before the kill completed (the
  `spawn_into_job` test-only hook records kill-then-drop order and the test asserts it, so a mutant that relies on
  `KILL_ON_JOB_CLOSE` at drop is red), and separately by the `breakaway` attempt **succeeding** (the negative
  control must fail at spawn or land in the job). A weaker red reaps only the direct child, walks PIDs, lets the
  child exit before the cancel (then the WHEN never occurs and `KILL_ON_JOB_CLOSE` at drop reaps for it), or uses a
  breakaway grandchild as the positive control (which a PID walk from the live child also finds — §7c.10).
- `ac224_windows_job_object_reaps_orphaned_grandchild` — the `cfg(windows)` arm asserted by name; red by **the
  holder surviving the kill** (the same observation through the real `dispatch`); assign order is not this test's
  red — that is `ac224_job_object_created_before_spawn`'s `ResumeThread` / `IsProcessInJob(child, our_job)` reading
  in `rk-containment`.
- `ac200_unknown_events_retained_verbatim` — red by dropping an unknown line or re-serialising it; a weaker red
  compares parsed values, not bytes.
- `ac201_future_shaped_event_still_ok` — red by `deny_unknown_fields` on any stream struct.
- `ac202_isolation_from_system_init_not_argv` — red by a scripted `system/init` with 280 tools passing while argv
  carries the flag, **and** by a scripted init with the card's tool COUNT and one substituted name passing (the
  same-count fixture, §7c.17) — a count-based implementation is green on the first and red on the second.
- `ac204_adoption_equal_subset_superset_wrong_agent` — four cases; red on a superset accepted, red on a **subset
  accepted**, red on a wrong agent classified `NotIsolated`.
- `ac386_extra_tools_not_isolated_no_retry` — red by extra tools classified `Ok` or `NotAdopted` (the adapter's
  mutation); the zero-retry half reads `rk_types::StepError::retry_class()` and no adapter mutation moves it.
- `ac203_agents_without_agent_refused` — red by a bind succeeding with `--agents` and no `--agent` (a marker file
  appears).
- `ac226_version_mismatch_configured_outcome` — four assertions; red by one outcome fixed regardless of configuration.
- `ac206_exit0_success_without_structured_output_is_schema` — AC-206/AC-439/AC-282 adapter half over the F20 recorded
  frame; red by an `Ok` constructor reachable without a validated value.
- `ac208_prose_json_never_recovered` — red by a fixture whose prose carries valid JSON yielding `Ok`.
- `ac209_system_prompt_hash_invariant` — red by any volatile input reaching the system prompt bytes.
- `ac210_both_ttl_vars_set_and_recorded` — runtime red: one variable set, or a `dispatch_completed` row lacking
  either TTL field; the non-optional-field half is `-p rk-types ac360_dispatch_record_carries_the_usage_fields`.
- `ac526_ttl_pair_refuses_single_or_mismatch` — five cases; red on either single case or the mismatched pair
  accepted, **or by a marker file appearing** on any refusing case; a weaker red omits the mismatch arm.
- `ac212_model_effort_from_bound_card_only` — three cases; red on an env value reaching argv; on cases (a) and (b)
  **and on (c) with the bound card incomplete**, red by a marker file appearing.
- `ac214_permission_mode_explicit` — the unit half only (the lint half is ARMED — §7c.5); red by a default when the
  card is silent, **or by a marker file appearing** for a silent card.
- `ac215_permission_prompts_none_stdin_closed` — red by the flag absent, and red by stdin left open after the write
  **observably**: the fixture child runs in `wait-stdin-eof:<transcript>` mode (block until stdin reaches EOF, then
  replay the scripted transcript and exit 0 — §7c.17), so the correct implementation's outcome is **`Ok`** and a
  never-shut stdin hangs to the card's bound with the outcome `Cancelled { by: Timeout }` — the second review's
  EOF-then-exit mode emitted no transcript, so `Ok` was unreachable on the correct implementation too and the test
  was red for the wrong reason; the assertion is *outcome is `Ok` and wall time well under the card's bound*.
- `ac216_bare_absent_across_matrix` — a property test; red by `--bare` on any matrix point; a weaker red is a grep.
- `ac511_bare_never_in_argv_lint_with_control` — red by the control fixture (a call site that adds the flag) not being
  named by the lint.
- `ac422_denylist_is_one_exported_list` — the test **iterates the exported `DENYLIST` slice** and asserts
  `Config::new` refuses a map carrying each name in it; red by any name in the export that `Config::new` accepts (a
  private copy missing a name). "A second list" is not something a test observes; the iteration is.
- `ac218_is_error_is_platform_regardless_of_exit` — four cells, each with its source named (§7c.17): non-zero × set
  over the seven recorded frames; exit-0 × unset over the F20 `result_raw`; non-zero × unset SYNTHETIC (no recorded
  frame identified); exit-0 × set with a **SYNTHETIC exit code** labelled so; red on the exit-0 × set cell
  classifying `Ok`, and on the non-zero × unset cell classifying `Platform`.
- `ac219_headroom_from_rate_limit_event_only` / `ac380_rate_limit_event_sole_source_both_shapes` — red by a reading
  taken from `result.rate_limit`, or by a named error on either payload shape.
- `ac220_refuse_within_margin_two_ticks` — red at the 0.02 boundary; a weaker red accepts a margin of 1 tick.
- `ac221_overage_refuses_before_spawn` — PROVISIONAL, built, not claimed ([surface unverified]); red by a marker
  file appearing after the flipped recorded payload.
- `ac378_overage_zero_attempts_no_spawn` — PROVISIONAL, built, not claimed; `dispatch` returns `Err(Overage)`, the
  fake holds exactly one `dispatch_completed` row with `cli_version: Pinned`, marker count 0, and **no `step_refused`
  row written by the adapter** (§7c.7); red by a marker file, by no refusal row, or by a `step_refused` row the
  adapter wrote (the kernel's kind).
- `ac225_cancelled_keyed_on_token_not_exit_code` — two positive arms (the token fired by the test → `Cancelled {
  by: Signal }`; the card's `timeout_secs` expiring on the adapter's own timer → `Cancelled { by: Timeout }`, §7c.7,
  §7c.10) and one negative: red by an external kill with `CANCEL_EXIT_CODE` classifying `Cancelled` (the test calls
  `rk_containment::terminate_pid` on the pid the marker file carries, from outside the adapter, with the same code —
  §7c.17), and by the two arms' `by` values swapped; a weaker red keys on the code, or drives one arm only.
- `ac227_every_harness_event_carries_cli_version` — runtime red: a `dispatch_completed` row lacking the version, or
  a pre-spawn row whose version is written as `Observed` (the pin masquerading as a measurement); the type half is
  `-p rk-types`.
- `ac229_adapter_maps_every_failure_to_closed_variant` — a source-parse over the file that constructs the row
  (`classify.rs`), in 7b's AC-028 shape (`crates/rk-kernel/tests/it/retry.rs`: names asserted present, THEN the
  absences): **admission first** — each of the eleven `StepError::<Variant>` names is present in the file — **then**
  no `_ =>`, no `_ if`, no `matches!(`; red by any name absent or any wildcard. The second review's absence-only
  form was satisfiable by a `classify.rs` with no `match` over `StepError` at all. The exhaustiveness itself is
  observable only by deleting an arm — the crate stops compiling (E0004), and a weaker implementation with a `_ =>`
  arm survives the deletion. The adapter crate cannot add a variant to `rk_types::StepError`, so no cfg-gated
  variant is the red; and this test observes nothing about what the two undriven arms *do* (§7c.11).
- `ac232_api_retry_counted_per_event_no_own_retry` — over the two lines RECONSTRUCTED from `results/e3.jsonl` line 6
  (§7c.11); red by a count of 1 (terminal attempt) or a second marker file.
- `ac238_claude_registry_entry_dated` / `ac489_capability_registry_dated` / `ac492_unknown_capability_token_fails` —
  red by a row with a missing member (`capability`, `verified_on` or `probe` — each required, `probe` a closed
  `Probe::{Recorded, None { reason }}`, §7c.7) building, by `reads_minds` building, by a catch-all arm in the token
  mapper. The name is honest about its scope: one crate's run cannot check "per adapter", so AC-238's per-adapter half is
  the four `ac489_…` lines (one per crate), not this test.
- `ac360_one_event_per_termination_path` — every path §7c.11 lists, each induced through the real `dispatch` by the
  fixture child's mode / a replayed frame / a flipped payload / the token, including the two pre-spawn refusals; red
  by any path returning before the append, or by a pre-spawn row with `cli_version: Observed`. Nine variants plus
  `Ok` are driven; `Unsupported` and `Timeout { after_secs }` are not (the adapter produces neither — §7c.11; their
  arms exist by exhaustiveness and nothing asserts what they do).
- `ac374_ticks_roundtrip_decimal_digits` — two-part (§7c.8): the recorded e1-smoke line for provenance (cannot
  discriminate — `0.04` survives an `f64`) and the SYNTHETIC `0.10`/`0.20`/`0.09`/`0.29`/`0.57`/`0.58` line, red by a
  **truncating** `f64` path (`0.29` → 28, `0.57` → 56, `0.58` → 57); a rounding `f64` path survives every two-digit
  value and is refused by the type test and the block's `f64` ban, not by this vector — the honest limit §7c.8 states.
- `ac375_throttle_input_has_two_windows_only` — red by a third field.
- `ac376_throttle_refuses_without_spawn_and_appends_row` — red by a marker file in the refusing case, by no
  `dispatch_completed` refusal row in the fake, or by a `rate_limited` row the **adapter** wrote (the kernel's kind,
  §7c.8).
- `ac377_margin_parse_vectors_and_no_default` — red by 0.01 → 2 ticks, by a no-margin config loading, by a `Default`,
  **or by a marker file appearing** after a sub-two-tick or margin-less `Config`.
- `ac381_headroom_calculator_still_delivers_a_delta` — the positive half; the compile-fail half is the trybuild case
  `ac381_cache_field_on_headroom_input.rs`. Red by removing the calculator.
- `ac383_card_without_profile_refused` — red by a profile-less card loading, **or by a marker file appearing**.
- `ac385_version_on_event_and_mismatch_warning` — red by the row lacking `Observed(v)` under a scripted foreign
  version, by `mismatch_warning(&record)` returning `None` while `v != instrument_version` (§7c.9), or by a
  pre-spawn row written `Observed`; a weaker red asserts only that the version landed.
- `adr0113_ttl_classifier_five_states` — red by three states (the 211-line file's set): `verified 5m` and `mixed`
  must be distinct outcomes.
- `ac267_resume_cost_is_summed_across_turns_and_option` — PROVISIONAL, built, not claimed; red by a last-turn-only
  value or a `Some(0)` (§7c.9; the `gate_resumed` fill path is R-17's).
- `ac493_empty_or_absent_tools_rejected` — two fixtures; red on either loading, **or by a marker file appearing**.
- `ac132_builder_lints_one_schema_no_strict_tools` — red by a strict tool passing the lint silently.
- `ac269_answer_api_absent_from_cards` — the card-validator half; red by a card naming `answer`. The build-config
  half is ARMED to 7d (no `rk-cli` exists to have an edge — §7c.5).
- `ac368_tier2_append_failure_is_warning` — PROVISIONAL, built, not claimed (§7c.13): a fault-injecting fake handed
  to `dispatch`; red by the append **not attempted** on the failing store (the fake's call count 0), by the failure
  not counted and typed on the warning counter, or by a panic; the `Ok(Typed)`-after-failed-append assertion is
  present and labelled in the test as R-16's reading under test, not a claim.
- `ac509_child_echoes_env_without_tracker_credential` — the fixture child in `echo-env` mode through the real
  spawn path, asserting the echo **equals the constructed map plus the two TTL variables and nothing else**; red by
  `env_clear()` removed (the echo then carries the outer cargo's `CARGO_MANIFEST_DIR`/`PATH`) and by any denylisted
  name in the echo; the tracker-credential half is satisfied by construction through AC-217's refusal (§7c.6).
- `dispatch_cwd_is_the_spec_worktree_never_process_cwd` — RT-21 (§7c.12; AC-529's clause cited, not claimed): the
  fixture child in `echo-env` mode also echoes its cwd; red by a child observed in the process's cwd instead of the
  spec's `"worktree"` input, and by a key-less spec driven straight into `dispatch` returning anything but
  `Err(Transport { kind: Spawn, .. })` with marker count 0 and exactly one `dispatch_completed` row (RT-20's sibling).
- `trybuild_pins_the_reason_each_claude_fixture_fails` — the harness, named for its rule; AC-381's case commits a
  `.stderr`.
- `live_dispatch_probe` — `#[ignore]`, `--features live`; not red-first in the ordinary sense — it is the fifteenth
  line (§7c.15), bounded by the card's `timeout_secs = 300`, and fails on any of: no `structured_output` in the raw
  result frame, no `system/init` retained, `OBSERVED_CLI_VERSION` ≠ pin, `COMPLETED_ROWS` ≠ 1, `DISPATCH_ROWS` ≠ 0,
  `--permission-prompts none` absent from the argv it wrote to `ARGV_PATH` (AC-215's tag lift), and the shared
  conformance body not run (`PROBE_RAN_CONFORMANCE=1` absent — §7c.15).
- `live_disallowed_tool_terminates` — `#[ignore]`, `--features live`; AC-215's integration half, **authored, not run
  at 7c** (ARMED, §7c.22); when run, red by a hang past the card's bound instead of `Cancelled`/`NonZero`.

**`-p rk-containment`:** `ac278_no_descendant_survives_exit` (the primitive over a fixture child, never the gate
path — the id is ARMED by that half, §7c.10; red by a survivor after exit, read by `is_alive` on the marker pids —
§7c.17); `ac224_job_object_created_before_spawn` (`cfg(windows)`; **two parent-side discriminators, both taken
before `ResumeThread` inside `spawn_into_job`'s test-only reading**: `IsProcessInJob(child_handle, our_job_handle)`
TRUE against the adapter's OWN job — unaffected by the `KILL_ON_JOB_CLOSE` job cargo wraps every test process in
[measured 2026-09-17, §7c.10] — and `ResumeThread` returning a previous suspend count of exactly 1; red every run
by assign-after-spawn, which fails the first or returns 0 from / never calls the second; **no child-side line** —
the second review's exit-3 reading and its directly-spawned negative control cannot fire under cargo's job and are
dropped as instruments); `orphan_grandchild_reaped_via_job` (`cfg(windows)`; the fixture child in three-level
`orphan` mode with the child holding; **liveness of the holder read after `group.kill()` and BEFORE the group
handle is dropped**, so `KILL_ON_JOB_CLOSE` at drop cannot do the kill's work; red by a PID walk that misses the
holder — Slice 2's measurement of O-1/O-2, §7c.10);
`breakaway_attempt_fails_without_breakaway_ok` (`cfg(windows)`; the fixture child in `breakaway` mode; red by a
breakaway that succeeds); `run_bounded_kills_tree_at_timeout` (red by a survivor after the bound, by output
through a pipe — the fixture child in `spew:<bytes>` mode with N > 64 KiB, which deadlocks a pipe and only a file
survives — or by `TimedOut` reported as an exit status).

**`-p rk-adapter-sqlite`:** `port_conformance_real_row_sqlite` (red by a body that differs from the fake's, and by a
body that `.expect()`s a capability the registry does not carry — the unchanged `snapshot().expect(..)` shape);
`ac489_capability_registry_dated` / `ac492_unknown_capability_token_fails` (red by a `snapshot` row without
`verified_on` loading; by `reads_minds` building; by `snapshot` returning `Unsupported` with no registry row behind
it);
`wal_and_busy_timeout_are_enforced` (red by `journal_mode ≠ wal` or `busy_timeout ≠ 5000`);
`ac364_correction_appends_never_mutates` (red by an `UPDATE`); `ac365_missing_store_file_recreated` (red by an open
error, or by an `INSERT` failing on a fresh file because no schema was applied — §7c.13);
`ac368_sqlite_append_error_is_typed_not_panic` (red by a panic/`unwrap` on a busy store; the warning half is
`-p rk-adapter-claude`, PROVISIONAL, §7c.13); `ac369_identity_mismatch_refuses_open` (red by an adopt path, and by
a root commit not taken from `git log --max-parents=0` — the fixture repos carry at least one commit, §7c.13);
`ac370_one_store_per_common_dir` (three worktrees; red by a second store file);
`ac243_gate_open_replay_over_sqlite_store` (red by a `dispatch` row inside an open gate). AC-398 and **AC-371** have
no adapter instrument (both cross-listed to `rk-store`, §7c.13 — every key column is `rk-store`'s schema, applied
unchanged) and AC-399 is ARMED whole — none has a `-p rk-adapter-sqlite` test, and earlier forms of this list named
one for each.

**`-p rk-adapter-toolchain-node` and `-p rk-adapter-toolchain-rust`, each, over the crate's `project/` fixture:**
`port_conformance_real_row_node` / `_rust` (red by `test(&[..])` returning a count other than the row's
`expected_test_count` of 2, or `Ok` without a spawn marker — the fixture project's tests write one — and by
`.expect("change_summary")` on the unchanged body; the row supplies its own `(path, classification)` pairs, so a
disagreement with the fake's table is not something this body observes); `ac328_affected_selection_unsupported_without_probe`
(red by a `basis: affected` record under `Probe::None`, or by a row carrying an invented `Probe::Recorded`);
`ac329_report_conforms_no_prose_parse` (nine negatives, one per member; red by the decoy — the Rust fixture's
`build.rs` emits `cargo:warning=test result: ok. 99 passed; 0 failed` and `cargo:warning=test phantom ... ok` on
every build, the Node fixture's test prints `Tests  99 passed`; a summary parser reports 99, an unanchored `... ok`
line-counter reports 3, and the structured source — `--list` (Rust) / the JSON reporter file (Node) — reports 2 —
§7c.14);
`ac325_recorded_argv_is_os_vector` (red by the template recorded instead of the vector; on the Rust side also by the
count vectors — the run vector plus `-- --list` / `-- --list --ignored` — not being what was handed to the OS);
`ac331_killed_verify_is_unverifiable` (red by a failing-test outcome after a mid-run kill);
`ac497_classify_binary_paths` (PROVISIONAL; red by `.uasset` as `Text`);
`ac344_missing_binary_is_not_a_candidate_failure` (the adapter's share of a CROSS-LISTED id, §7c.14: the adapter's
`program` construction value set to an ABSOLUTE nonexistent path, `<tempdir>/no-such-cargo.exe` — a PATH-less child
env cannot induce it; red by the spawn refusal classified as a failing test or `NonZero` instead of `Transport {
kind: Spawn }`);
`ac489_capability_registry_dated` / `ac492_unknown_capability_token_fails` (red by a row with a missing member
building — the same red as the claude/sqlite crates, §7c.7; by `reads_minds` building). **Node only:**
`d34_vitest_invoked_directly_never_pnpm_forwarded` (asserts the ordered vector `vitest run --reporter=json
--outputFile=<tmp> <file> [-t <pattern>]` minus the program element — §7c.14; red by `pnpm test --` in the vector,
by `test`/`--` between the program and `vitest`, or by the reporter flags absent or out of place);
`ac334_flip_predicate_real_toolchain` (`corpus` feature; applies the inversion with `git apply` and reverts it in a
`Drop` guard, asserting the checkout clean afterwards — §7c.14; red by the F25 inversion surviving against the real
Node adapter, and by a dirty checkout after the run). **Rust only:**
`nextest_fallback_recorded_on_report` (red by a report whose `argv` names `nextest` on a box without it; the
presence arm is unobservable on this box — nextest absent, measured — and is ARMED, §7c.14);
`rust_pack_fixture_binds_kinds_per_adr0029` (a `tests/it` test reading the fixture through `rk_kernel::toml_subset`
over the dev edge — §7c.14; red by a pack producer not keyed to `rust`, or by `[mutation] feasible`/`reason` not
reaching `MutationFeasibility::declared`).

**Other crates:** `-p rk-types` — `ac362_absent_usage_is_none_not_zero`, `ac362_absent_and_zero_roundtrip_distinct`
(red by `Some(0)` / by absent and zero serialising identically), `adr0126_dispatch_completed_is_tier_two` (red by
`ALL` at 37 or a tier-1 tag), `ac360_dispatch_record_carries_the_usage_fields` (red by a `Default` or an `f64`
reading), `ac523_zero_backoff_table` (re-homed from the adapter — red by any backoff class for `RateLimited` in
`retry_class()`); `-p rk-ports` — `ac364_event_store_port_has_no_update_or_delete` (source-parse; red by adding `fn delete`),
`port_conformance_fake_rows` (re-run under `--features conformance` — RT-16; red by a real row asserting a
different body); `-p rk-store` —
`retention_record_is_rk_types_dispatch_record` (red by two `DispatchRecord` types); `-p rk-kernel` —
`ac484_adapter_edge_fixture` (re-run after the rename; red by the old fixture name).

### 7c.19 The manifests, and what this stage may not disturb

Every new crate's `Cargo.toml` carries `edition.workspace = true`, `rust-version.workspace = true`,
`license.workspace = true`, `publish.workspace = true`; `[[test]] name = "it" path = "tests/it/main.rs"` (one
integration binary per crate, `mod` per case — ADR-0083); `[lints] workspace = true` **last** and nothing else in
`[lints]` (ADR-0102 decision 2); the four adapters carry `[features] corpus = []` (corpus-bound tests, local-only —
AC-464/PIA-002; CI runs without it) and `rk-adapter-claude` also `live = []`. **`#[expect(clippy::…, reason = "…")]`
per site, never `#[allow(`** — every `Command` site (`disallowed_types` bans the TYPE `std::process::Command`
workspace-wide, so **every mention in the five new crates** counts: `rk-containment`'s spawn site in `ffi.rs`, the
`run_bounded(cmd: std::process::Command, …)` signature, and the fixture bin's `orphan`/`breakaway` grandchild spawns
— `rk-containment` is not an adapter and the bin is outside `src/`, and both are inside the workspace clippy gate)
and every `Instant::now` / `SystemTime::now` site (`disallowed_methods`) carries one (precedent
`rk-git/src/git_exec.rs`; house form `rk-store/src/conn.rs`); Slice 2's commit body lists its `#[expect]` sites
(`ffi.rs` spawn, `run_bounded`, `rk-fixture-child.rs`) so the count of expectations is read against the count of
sites. **`std::env::var` is not on clippy.toml's disallowed list**, so it takes no `#[expect]` (one
would be an `unfulfilled_lint_expectations` diagnostic and turn the clippy gate red); and **no adapter `src/` file
reads the process environment at all** — the block's AC-217 ban — so the only `std::env::var` in the five crates is
`live_dispatch_probe`'s, in `tests/it`, outside the ban's scope. The **only** `allow` in the workspace is
`rk-containment`'s commented `#![allow(unsafe_code)]` in `ffi.rs` (§7c.10), and Checkpoint 7c asserts exactly one
non-comment attribute workspace-wide and zero `#[allow(` — inner `#![allow(` or outer `#[allow(` — in the five new
crates, **`src/`, `tests/it` and the compiled fixture bin (`crates/rk-containment/tests/fixtures/bin`) alike**, which
is exactly the scope the block's line names.
`HashMap`/`HashSet` are clippy-banned — `BTreeMap` for the env map. `thiserror` in every library (ADR-0102 decision 6;
design §9.3's pin table) — it has **no workspace row today** and no crate on the tree uses it (`rk-kernel` writes a
hand-written `Display`), so the stage-open commit adds `thiserror = "2"` (2.x, caret under ADR-0100) to
`[workspace.dependencies]`; `anyhow` in none (the CI confinement rule). `tempfile = "3"` is written directly in each
new crate's `[dev-dependencies]`, as `rk-git` and `rk-kernel` write it (no workspace row exists and none is added).
**`jsonschema`'s workspace row is `jsonschema = "=0.56.0"` with default features ON**: a member writing `{ workspace
= true, default-features = false }` gets cargo's warning that `default-features` is ignored, so `rk-adapter-claude`'s
normal graph carries jsonschema's default resolvers through the workspace row and the `cargo tree -i tokio` reading
includes whatever they pull; the earlier *"default features off"* claim was untrue on this row and is withdrawn —
turning the defaults off on the workspace row (and giving `rk-types`' dev-dep the feature set its validation tests
need) is a stage-open change the operator may take, and is **not** assumed here. `rk-adapter-claude`'s trybuild
harness is named for its rule — `trybuild_pins_the_reason_each_claude_fixture_fails`, in `tests/it/ui.rs` over
`tests/ui/`, AC-381's case with a committed `.stderr` — and the checkpoint filters on that name with `--test it`,
never on `trybuild`. `windows-sys` has **no default features**: `rk-containment`'s row names
`Win32_Foundation`, `Win32_Security`, `Win32_System_JobObjects`, `Win32_System_Threading` and `Win32_System_Diagnostics_ToolHelp`
(§7c.10; five — the `Win32_Security` correction of 2026-09-18 is recorded there) or nothing links.

| crate | depends on | new workspace rows |
| --- | --- | --- |
| `rk-containment` | `std`; `windows-sys` (windows; the five features above — four until 2026-09-18), `libc` (unix) — **and nothing else**: no `rk-types` (its outcome enum is its own), no tokio, no `rk-ports`, no `rk-adapter-*` (ADR-0127 decision 2 and forbids; RT-15). `[[bin]] name = "rk-fixture-child"` at `tests/fixtures/bin/rk-fixture-child.rs` (the bin uses only `std` — §7c.17); the two `#[doc(hidden)]` test-support wrappers `terminate_pid` / `is_alive` (§7c.10); dev: `tempfile` | `windows-sys = "=0.61.2"`, `libc = "=0.2.189"` |
| `rk-adapter-claude` | `rk-types`, `rk-ports`, `rk-containment`, `tokio`, `serde` (derive), `serde_json`, `jsonschema =0.56.0` (defaults on — above), `thiserror`; **no normal `rk-git`** (the pre-spawn snapshot is not lifted — §7c.2) and **no `rk-store`** (after Slice 1's collapse nothing in `rk-store::retention` is the adapter's — `Retained`, `STDIO_CAP`, `HEAD_TAIL` and the retention constructor live in `rk-types`, and a normal edge would pull `rusqlite` (bundled SQLite) into the Harness crate's tree for nothing; the block asserts `rk-adapter-claude`'s normal graph never reaches `rusqlite`); dev: **`rk-ports = { path = "../rk-ports", features = ["conformance"] }`** (RT-16 — the bodies, test-only), **`rk-git`** (the probe's temp repository — a dev edge), `insta`, `proptest`, `trybuild`, `tempfile`, **`rk-adapter-sqlite`** (the probe's temp store — a dev edge between adapters never reaches `rk-kernel`'s normal graph), and a second `[[bin]] name = "rk-fixture-child-claude"` from `rk-containment`'s source file (§7c.17 — a distinct target name from one source, because one bin name in two packages is one output path and a link race Halt 1's line loses) | `tokio = "1"`, `thiserror = "2"` |
| `rk-adapter-sqlite` | `rk-types`, `rk-ports`, `rk-store` (`rusqlite =0.40.2` bundled inherited), `rk-git` (`--git-common-dir`, the root commit), **`serde` (derive — the registry's `Deserialize` + `deny_unknown_fields`, RT-24)**, **`serde_json`** (`t2_event.payload TEXT` ↔ the `serde_json::Value` `rk_types::event::Event` carries), `thiserror`; dev: `rk-ports` with `features = ["conformance"]`, `rk-kernel` (AC-243 replay only), `tempfile` | — |
| `rk-adapter-toolchain-node` | `rk-types`, `rk-ports`, `rk-git`, `rk-containment`, **`serde` (derive — the registry)**, `serde_json` (the vitest reporter file), `thiserror`; a `program: PathBuf` construction value (default `npx.cmd` on Windows / `npx` elsewhere — §7c.14) and a construction-time timeout (RT-5); dev: `rk-ports` with `features = ["conformance"]`, `rk-kernel` (an **unconditional** dev edge — cargo refuses `optional = true` on `[dev-dependencies]` — used only by `#[cfg(feature = "corpus")] mod ac334;` in `tests/it`, §7c.14), `tempfile` | — |
| `rk-adapter-toolchain-rust` | as Node (`program` default `cargo`; `serde` for the registry, `serde_json` for `cargo metadata`); dev: `rk-ports` with `features = ["conformance"]`, **`rk-kernel`** (the dev edge the third review added — `rk_kernel::toml_subset` reads the pack fixture in `rust_pack_fixture_binds_kinds_per_adr0029`, §7c.14; AC-334's Rust half stays ARMED, this edge is for the pack), `tempfile` | — |

**What this stage may not disturb, and how each disturbance is disposed of.** Three closed crates are edited and each
row goes provisional **in the same commit**: **`rk-types`** (Checkpoint 5, passed) — `DispatchRecord`'s new fields,
`usage.rs`, `EventKind::DispatchCompleted` (ADR-0126); re-run Checkpoint 5's block, including its `insta` snapshots
and its type-scoped `ChangeSummary` grep, and the `PROGRESS.md` §1 row reads `passed <hash>, re-run pending 7c
rk-types` until it does. **`rk-store`** (Checkpoint 6, passed) — `retention::DispatchRecord` collapses onto
`rk-types`', **and two compile consequences of the collapse are named here so Slice 1 is not surprised by them**:
(a) `rk_store::retention::DispatchRecord::new(argv, env, instrument_version, stdout, stderr)` is an inherent `impl`
(`retention.rs`:40-72); once the type is `rk_types::activity::DispatchRecord` an inherent `impl` in `rk-store` is
**E0116**, so the constructor **moves to `rk-types`** and its five-argument signature cannot build the grown record
(`cli_version`, `api_retry_events`, the two TTL strings and `timings` are required members) — the new shape is
`rk_types::activity::DispatchRecord::new(argv, env, instrument_version, stdout, stderr, RecordTail { cli_version,
api_retry_events, prompt_cache_ttl, subagent_prompt_cache_ttl, timings, usage, total_cost_usd, headroom_before, headroom_after, cancelled_by, detected_by })` *(corrected 2026-09-18 at Slice 1: this spelt the TTL pair as one member, `ttl`, against §7c.11's own "the two TTL fields" and AC-210's "both values are recorded"; Slice 1 declares the two `String` members and no `CacheTtl` wrapper — the review's spec-F5)*
(or a builder of the same closure), with `rk_store::retention` re-exporting the type as it already re-exports
`Retained` / `STDIO_CAP` / `HEAD_TAIL`, and `looks_like_credential` staying where it is; (b)
**`crates/rk-store/tests/it/store.rs`** calls the five-argument `new` (`:338`) and destructures the five-field record
(`:350`), so it is in Slice 1's edit list and Checkpoint 6's own suite would not compile until it is edited. Re-run
Checkpoint 6 whole (its lift-SHA lines are `reconcile.rs`'s and are untouched).
**`rk-ports`** (Checkpoint 7a, passed) — the `conformance` feature module and `PHASE_1_REAL_ROWS` filled; re-run
Checkpoint 7a, whose *"exactly one conformance test line"* assertion and six-method `Toolchain` count must survive
unchanged. **`rk-kernel`** (Checkpoints 7b/7b-ii, passed) — **edited at Slice 1, and it cannot be otherwise**: the kernel has
three exhaustive `match` blocks over `EventKind` with no catch-all (`workflow/fsm.rs`, `workflow/breaker.rs`,
`workflow/memo.rs`, each closing on `| EventKind::AdvisoryDismissed => {}` [measured 2026-09-17]), so adding
`EventKind::DispatchCompleted` is E0004 until each gains the no-state-change arm ADR-0126 decision 5 names; and
three of its integration tests build `DispatchRecord` by struct literal (`tests/it/replay.rs`, `resolver.rs`,
`runtime.rs`), as does `rk-ports`' `fakes.rs` `fake_record()`, so growing the record is E0063 in both until the
literals gain the members; **and three more of its integration tests pin `EventKind::ALL.len()` to 37 and go red the
moment ADR-0126 lands** — `tests/it/breaker.rs`:557-558, `tests/it/fsm.rs`:389-390 and `tests/it/memo.rs`:1158-1159
[measured 2026-09-17], each carrying a clause the minting record must answer (*"none of them is a breaker mark"*;
*"opens no gate"*; *"whoever mints it must decide whether the memo's payload record is still OWED"*), beside
`rk-types`' own pin at `tests/it/event_kinds.rs`:13 (the *"pin test gains a case"* ADR-0126 decision 5 names). Slice
1 therefore edits `rk-types`, `rk-store` (`retention.rs`, `tests/it/store.rs`), **`rk-kernel`** (the three matches,
the three test literals, the three `ALL.len()` pins moved 37 → 38 each with a dated in-place note carrying
ADR-0126's answer to its clause — `dispatch_completed` is tier-2 telemetry: not a breaker mark, opens no gate, and
the memo's typed-payload record stays OWED (ADR-0126 decisions 4 and 6) — and the **fourteen** *"closed 37"* /
*"closed at 37"* strings over `crates/`, listed by file:line so the count reproduces — **eleven doc comments in
`rk-kernel/src`**: `workflow/breaker.rs`:149, :490; `workflow/budget.rs`:72, :886, :910, :952;
`workflow/degrade.rs`:316; `workflow/fsm.rs`:1288; `workflow/memo.rs`:122, :653; `workflow/retry.rs`:533 — **and
three assertion messages in `rk-kernel/tests/it`**, inside the very `ALL.len()` pins this slice moves:
`tests/it/breaker.rs`:564, `tests/it/fsm.rs`:396, `tests/it/memo.rs`:1163 [measured 2026-09-17 by `grep -rn
'closed 37\|closed at 37' crates/rk-kernel/src` → 11 and over `crates/` → 14; the first form of this sentence said
four, then seven, then eleven over a narrower path than its own post-edit reading — three undercounts by the same
route (Halt 7)] — corrected in place, dated, with the pre-edit reading `14 over crates/ (11 in rk-kernel/src, 3 in
rk-kernel/tests/it)` and the post-edit reading `grep -rn 'closed 37\|closed at 37' crates/ → 0` in the commit body)
and **`rk-ports`**
(`fake_record()`), marks the `PROGRESS.md` §1 rows for **5, 6, 7a, 7b
and 7b-ii** `passed <hash>, re-run pending 7c <crate>` in that commit, and re-runs Checkpoint 5 and 6 whole and the
named lines of 7a, 7b and 7b-ii there — a `cargo build --workspace` (Halt 1) that stays green is the proof the edit
was made, not a claim that it was not. Separately, `ac484_adapter_edge_fixture` and the optional dep follow the
fixture rename at stage-open (already on the tree, uncommitted, at the time of this rewrite [measured 2026-09-17]),
and Checkpoint 7b's `ac484_…` line is re-run by name in the block below. **The existing checkpoint lines re-run here
by name**: Checkpoint 5's `EventKind::ALL` count (now 38), Checkpoint 7a's `port_conformance_fake_rows`, Checkpoint
7b's `ac484_adapter_edge_fixture`; Checkpoint 6 is re-run whole in Slice 1's commit and not repeated in the block.
The corpus pin (`CORPUS_PIN`) does **not** move.

### 7c.20 The slices

Every slice lands in the record's voice with a body in the R2/V3 form (`s7b-ii` §7b-ii.13's precedent): what was built
per module and rule; fixtures with provenance bytes/sha256; **RED quoted** — the failure text of each named test
before the code; deps added and `deny ok`; the five gates' results; which checkpoint lines ran; provisional re-runs
with the `PROGRESS.md` §1 mark in the same commit; raised-not-settled; PROVISIONAL labels kept; the ADR-0101
disclaimer; the `Co-Authored-By` trailer. Commit subjects take the form **`Adapters, <slice> (<crate>): <claim in the
record's voice> (<AC ids>)`**.

0. **Stage-open** (tracker commit; **in progress at the time of this rewrite** — the fixture rename is staged, the
   `ci.yml` / `Cargo.lock` / `rk-kernel` / instrument-file edits are unstaged, `PROGRESS.md` and `00-preamble.md` are
   untouched [measured 2026-09-17 by `git status`]): row 7c **IN PROGRESS**; the three BEFORE prints recorded; the
   instrument version read from the re-teed file (2.1.269 → 2.1.275 recorded in `PROGRESS.md` §1/§3/§6 — Halt 2);
   the fixture rename; **four `ci.yml` edits, re-checked in the diff before the commit** — the `Command` grep
   narrowed (on the tree), the AC-362 pathspec narrowed to `'crates/rk-adapter-claude/src/*' 'crates/*/src/usage.rs'`
   and the git rule's `grep -vE '^crates/[^/]+/tests/fixtures/lint-controls/'` exclusion (both NOT on the tree at
   the time of this rewrite — RT-13, §7c.1), and the AC-465 allowlist naming the extracted-row fixtures (RT-23) and,
   as 7b/7b-ii's named debt, `rk-kernel`'s three tracked `*.jsonl` (§7c.17); the `thiserror` workspace row (§7c.19);
   **this stage file (the rewrite, modified on the tree) and `spikes/DECISIONS-2026-09-17.md` (untracked)**, which
   the two records cite by id; **ADR-0127 committed at `status: accepted`** as final text, its forbids' first bullet
   worded to the library-code rule before the commit (§7c.17); **ADR-0126 committed at `status: proposed`** — its
   decision 1 fixes `cli_version` as the two-armed `CliVersion::{Observed, Pinned}`, which departs from D17-8 as the
   operator first took it (`cli_version: String`; the note under D17-8 says so), and under ADR-0092 an *accepted*
   record outranks the decisions log, so committing it accepted would bind the operator to a shape he has not
   confirmed; the ADR README's lifecycle (`proposed → accepted`) exists for this gap, and **a follow-up commit flips
   it to `accepted` with `date` unchanged once the operator's confirmation is written as a dated line under D17-8**
   — Checkpoint 7c's ADR-0126 line greps `^status: accepted`, so the block cannot pass before that flip; neither
   record carries a *"first draft"* note (the history is D17-8's dated note in DECISIONS-2026-09-17) — with the ADR
   README's count and ADR-0099's `superseded_by` (both already edited on the tree); **`00-preamble.md`'s
   next-free-number sentence corrected `0126` → `0128`, dated** (R-20, as ADR-0125's commit did); the
   external-anchor drift note in §2 covering all sixteen anchors (§7c.1); the `.gitignore` row for
   `target/predecessor-*/` (§7c.14); **"Checkpoint 7c is NOT run"**.
1. **Slice 1 — `rk-types` + `rk-store` + `rk-kernel` + `rk-ports`**: ADR-0126's kind lands; `usage.rs`;
   `DispatchRecord` grown (with the serde derives — `CancelledBy` and `DetectedBy` included — and `CliVersion`,
   §7c.11) and its constructor moved to `rk-types` with `RecordTail` (§7c.19); `DispatchCompleted` minted (`ALL`
   38); the `rk-store` collapse with `tests/it/store.rs`:338/:350 edited; the kernel's three `EventKind` matches,
   three test literals, three `ALL.len()` pins (37 → 38, dated notes) and fourteen *"closed (at) 37"* strings —
   eleven in `src`, three in the pins' own assertion messages (§7c.19); `rk-ports`' `fake_record()` and the
   `[features] conformance = []` module with its core/capability-arm bodies, the spec-parameterised Harness body
   that prints its own `PROBE_RAN_CONFORMANCE=1` (RT-16 as re-amended); Checkpoints 5 and 6 re-run whole,
   7a/7b/7b-ii by named line; §1 rows for 5, 6, 7a, 7b, 7b-ii provisional then discharged. Subject: `Adapters, slice 1 (rk-types, rk-store, rk-kernel, rk-ports): the
   dispatch record carries usage, timings and readings, and one kind carries them (AC-360, AC-362, AC-227, AC-232,
   AC-364 port half)`.
2. **Slice 2 — `rk-containment`**: ADR-0127's crate lands; `ffi.rs` with the one `allow` and both arms,
   `CANCEL_EXIT_CODE` chosen and recorded, **the assign-before-fork mechanism measured by this crate's own
   `orphan_grandchild_reaped_via_job` / `breakaway_attempt_fails_without_breakaway_ok` and the parent-side
   `ResumeThread` count and `IsProcessInJob(child, our_job)` reading in `ac224_job_object_created_before_spawn`,
   the cargo-`KILL_ON_JOB_CLOSE`-job fact beside them, and recorded as the answer to DECISIONS-2026-09-17 O-1 (as
   amended) and O-2 in this commit's body** (never an edit to ADR-0127); `windows.rs` / `posix.rs` safe wrappers and
   the two `#[doc(hidden)]` test-support wrappers `terminate_pid` / `is_alive` (§7c.10); `run_bounded` with its
   channel wait; the fixture child binary and its modes (the three-level `orphan`, the bin's own breakaway attempt);
   **the `#[expect(clippy::disallowed_types)]` site list** (`ffi.rs` spawn, `run_bounded`, `rk-fixture-child.rs`'s
   `orphan`/`breakaway` spawns) and the `ffi.rs` site list (spawn, assign, resume, kill, enumerate, `terminate_pid`,
   `is_alive`) read against the site counts; the two new workspace dep rows; S-CANCEL-WIN's Job Object arm run
   through the orphan test. Subject: `Adapters, slice 2 (rk-containment): a Job Object created before spawn kills
   the tree, and a bounded run kills at the bound (AC-278 primitive; AC-224/AC-225 are re-asserted through the real
   dispatch at slice 5)`.
3. **Slice 3 — `rk-adapter-sqlite`**: the real row over the core/capability-arm body; the schema applied on open; the
   store halves; the `rk-git` edge (`--git-common-dir`, `log --max-parents=0`); the registry with its
   `ac489_`/`ac492_` tests; AC-243's real-store replay. Subject: `Adapters, slice 3 (rk-adapter-sqlite): one store
   per common dir, append only, refuse a stranger (AC-364, AC-365, AC-368 sqlite negative, AC-369, AC-370, AC-243,
   AC-489/AC-492 sqlite row, AC-499)`.
4. **Slice 4 — `rk-adapter-claude` part A (configuration-time)**: `config.rs` (with `harness_binary`, `margins`,
   `TESTED_CLI_VERSION`), `env_denylist.rs` (RT-25's three tracker names), `env.rs`, `command.rs`, `card.rs` (the one
   lift this slice carries, with its header), `binder.rs`, `registry.rs` (the named `dispatch` row); the golden
   argvs; the lint-controls; the second `[[bin]]` (`rk-fixture-child-claude`). **The refusal tests land here with
   their `is_err()` half only** (§7c.18): `api_key_refuses_construction`, `structured_output_required`,
   `env_denylist_config_map`, `argv_cap_refuses_before_spawn`, `ac203_…`, `ac212_…`, `ac214_…`, `ac377_…`,
   `ac383_…`, `ac493_…`, `ac526_…` — each watched red on that half; their marker-count belt is Slice 5's. **Not in
   this slice**: AC-210's runtime half, AC-215 and AC-509 (each needs `runner.rs`'s spawn path) and the trybuild
   harness (its one case, AC-381, names `headroom.rs`'s input struct — a case authored before the module exists
   fails to compile for the wrong reason and its `.stderr` drifts at Slice 5), all of which are Slice 5's. Subject:
   `Adapters, slice 4 (rk-adapter-claude A): what is refused before spawn is refused as configuration — is_err
   halves (AC-203, AC-205, AC-209, AC-211–AC-214, AC-216, AC-217, AC-526, AC-383, AC-488, AC-493, AC-132, AC-269
   card half, AC-511, AC-238, AC-489, AC-492)`.
5. **Slice 5 — `rk-adapter-claude` part B (runtime + dispatch)**: the remaining lifts with headers (`event.rs`,
   `throttle.rs`, `runner.rs`; `journal.rs`'s principles in `record.rs`'s doc); `headroom.rs` (the structural
   scanner, `CostUsd`), `classify.rs`, `init.rs`, `record.rs` (`mismatch_warning`), the TTL classifier,
   `dispatch_completed` on every path, the three-way race and `CANCEL_POLL_MS` (§7c.10); **the marker-count belt
   added to each Slice 4 refusal test and watched red on that half**; the cancel and AC-224/AC-225 tests over
   Slice 2's primitive; AC-210's runtime half, AC-215's unit half and AC-509 (moved here from Slice 4); the trybuild
   harness with AC-381's case and its `.stderr`; the worktree-key test (RT-21); **the `[features] live = []` row,
   `live_dispatch_probe` and `live_disallowed_tool_terminates` authored (`#[ignore]`, `--features live`), the probe
   card and its role fixture** (the second review authored the probe in no slice and only ran it at the close);
   S-RETRY and S-TTL1 run or deferred; the AC-312 fixtures authored. Subject: `Adapters, slice 5 (rk-adapter-claude
   B): the stream is consumed inside, every termination appends one row, and headroom is integer ticks (AC-200–AC-202,
   AC-204, AC-206, AC-208, AC-210, AC-215, AC-218–AC-221, AC-224–AC-227, AC-232, AC-360, AC-368 provisional,
   AC-372–AC-375, AC-376 provisional, AC-377, AC-378, AC-380–AC-382, AC-385, AC-386, AC-509, AC-523, AC-267
   provisional; the Slice 4 belts)`.
6. **Slice 6 — the toolchains**: **first, two preflights on this box** — the `npx.cmd` under-`env_clear()`
   measurement (§7c.14; the names the map must carry, recorded [measured]) and **the AC-334 predecessor checkout
   materialised once** (clone from the bundle, checkout `f21f55fc…`, `pnpm install --frozen-lockfile --dir
   target/predecessor-f21f55fc`; network; local-only — moved here from the close step, because Slice 6's commit
   claims the Node real row and `ac334_…` cannot be watched red or green without it); then both crates over their
   `project/` fixtures (the Rust fixture's `build.rs` decoy); the Rust pack fixture read through the `rk-kernel` dev
   edge; the `--list` count route and the nextest fallback recorded; the full vitest vector with the per-OS program
   element; AC-334's Node real-row run; S-CANCEL-WIN's `vitest` arm. Subject: `Adapters, slice 6
   (rk-adapter-toolchain-node, -rust): two toolchains, one report shape, no prose (AC-325, AC-328, AC-329, AC-331,
   AC-334 Node row, AC-344 adapter share, AC-497, AC-499 three rows)`.
7. **Close — two commits, not one** (`00-preamble.md`: *"A stage closes in two commits"*; `s7b-ii` §7b-ii.13's
   precedent named both): the live probe run in the foreground from a non-Claude shell (§7c.15) — the predecessor
   preflight is Slice 6's, above; then **(a) the close commit** — `Step 7c close: the adapters exist,
   the real rows conform, one dispatch ran` — carrying the block's recorded output and end marker, the statement
   count re-counted, the probe's provenance (the corpus-commit SHA holding its raw `system/init` and `result`
   frames, with bytes + sha256; the observed CLI version; the cost), AC-511's field named or the open item recorded,
   AC-215's tag lift recorded, the S-CANCEL-WIN / S-RETRY / S-TTL1 run-or-deferral notes, and the three validators'
   triple; then **(b) the tracker commit** `Tracker: Checkpoint 7c passed at <hash of (a)> — the adapters exist, the
   real rows conform, one dispatch ran` with `PROGRESS.md` §1's Result cell carrying the probe's CLI version, cost
   and `system/init` digest, §6's statement count + end marker + validator triple, and row 7d → UNBLOCKED.

### 7c.21 Checkpoint 7c

**Counted mechanically off the block below on 2026-09-17, after the block was final and not before it** — comments
and blanks dropped, backslash continuations joined; **re-counted after each 2026-09-17 review revision** (the first
count of this file read 222 / 132 / 90 and its stated sub-decomposition did not reproduce; the second read 286 / 163
/ 123; the third read 314 / 184 / 130 and was superseded by the round-3 repairs — the shell guards moved to the top
and a tracker guard beside them, the third helper `rk_control` replacing twenty-four bare control greps, two bans
and their controls, the `--list` route's grep, the ADR-0126 status grep, the feature-gate and S-FEAT lines, the
`enum Capability` digest, the two toolchain edge lines, the `rk-git` absence, the predecessor dirty check, the
filter control's positive half, the second `dd_hits` control, the digest compare replacing a vacuous grep, and two
named tests deleted; Halt 7: count it off the script). **The rule that produces the number is stated so it
reproduces:** a *statement that can stop the block* is one carrying its own `exit 1`, **or a call to one of the
three stopping helpers, each of which carries its `exit 1` inside its definition** — the second clause is new to
this stage because `s7b-ii`'s block defined no helper; the fourth helper, `dd_hits`, prints and never exits, so a
call to it is a capture, not a stop. **352 non-comment statements, of which 211 can stop the block and 141
cannot.** The 211 are **149** statements carrying their own `exit 1` outside the helper definitions (the
top-of-block `uname` guard, the three shell guards under it — two of them `env | grep … && { …; exit 1; }` —, the
one-line `for d in $SRC_SCOPE` guard, the two `&& { …; exit 1; }` manifest bans, the `allow(unsafe_code` control
ban and the two python closers `' || { …; exit 1; }` among them) plus **62** helper calls (`rk_test` × 15 — five in
loop bodies, ten direct; `rk_absent` × 23; `rk_control` × 24 — a call inside a `for` body is one statement however
many times the loop runs). The 141 that cannot are not unguarded assertions and the decomposition reproduces:
**3** BEFORE readings (PIA-035 — prints for comparison, not assertions); **40** helper-definition lines
(`rk_test() {` … `}` six, `rk_absent() {` … `}` fourteen — the admission check and the exclusion argument are six
of them —, `rk_control() {` … `}` five, `dd_hits() {` … `}` fifteen including its embedded python; **the six
`exit 1`s inside the definitions are `rk_test`'s two, `rk_absent`'s three and `rk_control`'s one; `dd_hits` carries
none**, and each runs only when its helper is called); **42** `out=` / `raw=` / `hits=` / `h=` / `c=` / `caps=` /
`m=` / `dst=` / `ctl=` / `uf=` / `live=` / `pin=` / `meta=` / `PRED=` / `CORPUS_*=` / `SRC_SCOPE=` / `LC=`-style
captures whose guard is the statement beside or below them (a capture that carries its own `exit 1` on the same
line — `lift=`, `n=`, `d=`, `want=`, the filter control's `raw=` — is among the 149, not here); **29** `for` /
`done` / `set --` lines of the thirteen multi-line bash loop constructs (thirteen `for`s counting the two nested
ones, thirteen `done`s, three `set --`; the one-line `for d in $SRC_SCOPE` carries its own `exit 1` and is among the
149, and the `caps=$(for …)` digest is one capture line); **18** lines of the embedded AC-484 python walk (its
opener and seventeen inner lines, its own `for d in …` loop, the `assert` control and the `reached:` print among
them) and **6** of the Halt 3 frame check (its opener and five inner lines), each guarded by the closing `' || { …;
exit 1; }` line counted among the 149; **2** clippy lines, `cargo clippy … ; rc=$?; echo "exit=$rc"`, each guarded
by the line below; and **1** end-marker `echo`. 3 + 40 + 42 + 29 + 18 + 6 + 2 + 1 = 141; 352 − 141 = 211.
**Re-count and re-write every number in this paragraph after any repair to this block** (Halt 7: count it off the
script, not off this sentence).

**No line in this block is `[WILL EXIST]`, and the absence is deliberate.** No `redkiln` or `doctor` line appears —
every `doctor` half is in §7c.22's ARMED table, routed to 7d, and `./target/debug/redkiln` is `rk-cli`'s (Step 7d).
The live probe is not `[WILL EXIST]` either: it is a feature-gated test in this stage's own crate, built by the block
before it runs, and `halts.md`'s carve-out is for a path a later stage produces, not for a path this stage owes.
Nothing in `check-checkpoints.py`'s allowlist is added for this file.

**Every absence-based ban below carries a positive control, or says in place why it cannot.** Each of the
twenty-three `rk_absent` bans is (a) guarded — the scope directories are `test -d`'d first and the helper FAILs on
`grep` rc 2; and (c) positively controlled by a committed file under
`crates/rk-adapter-claude/tests/fixtures/lint-controls/` that carries the banned token **on a non-comment line,
checked through the ban's own search and comment filter by `rk_control`** (a bare `grep -q` over the control proved
only that the token was somewhere in the file — on a `//` line it would satisfy the control while the ban filtered
it out, the third review's finding), so a pattern that silently stops matching is caught rather than reported as a
clean tree. **Clause (b), paired with a behavioural test, holds for thirteen of them and NOT for ten, and the split
is named rather than claimed universally** (the second review said "each"): paired — the append flags
(`golden_argv_per_profile`), `taskkill` (`cancel_reaps_process_tree`), `result.rate_limit` (`ac219_…`/`ac380_…`),
the AC-374 pattern (`ac374_…`), the family keys (`ac375_…`), `unwrap_or(0)` (`ac362_…`), `--bare` (`ac216_…`/
`ac511_…`), `deny_unknown_fields` (`ac201_…`), the AC-202 argv pattern (`ac202_…`), `env::var`
(`env_denylist_config_map`), `pnpm test` (`d34_…`), the AC-208 scraper pattern (`ac208_…`) and the AC-329 prose
pattern (`ac329_…`); **lint-only floors whose evidence is the rule they cite** — tokio in `rk-containment`
(ADR-0127 d2), `#!?\[allow\(` (ADR-0102 d3), `set_var|remove_var` (ADR-0083), the poison-recovery spellings
(ADR-0118), `anyhow` (ADR-0102 d6), `Command::new("git")` (ADR-0077), `+1.98.1` (ADR-0102/0083), `Handle::block_on`
(ADR-0076), the direct-spawn call forms (ADR-0127 forbids) and the `tokio::process` types (RT-13). A grep a comment
could satisfy is never the whole assertion — comment lines are filtered after the status is asserted, Halt 7.
**Three bans are module-path filtered, never file-path scoped** (`rk_absent`'s fourth argument, `s7b-ii`
§7b-ii.13's shape): AC-374's over `headroom|throttle|event`, AC-201's over `event|init|usage` and AC-202's over
`init|isolation`, so a `headroom/` directory layout is never a grep rc=2 and never a layout-reason FAIL — **and a
filtered ban asserts ADMISSION before it filters**: the helper FAILs unless at least one `.rs` file under the scope
matches the filter, because a filter that admits no file (a digit scanner named `ticks.rs`, a cost scanner named
`money.rs`) would make the ban vacuous while the block stays green (Halt 8 shape (g); the module names are the
implementer's, §7c.1, and this is what keeps that freedom honest); the two modules the AC-374 ban must reach,
`headroom` and `throttle`, are asserted to exist beside it **in the admission shape** (a `find | grep -c` over the
filter regex, so `headroom/mod.rs` passes — the second review's `test -f` by exact path contradicted the sentence
before it), and the AC-374 filter carries its own control **in both directions** (the lint-controls path must be
excluded by the regex, with `grep -H` forcing the filename prefix a single-file `grep -r` omits, and a synthetic
`headroom.rs` path must be admitted). **One ban takes the helper's fifth argument, an exclusion regex applied after
the comment filter**: the `#!?\[allow\(` ban over both attribute forms excludes the one permitted line, the
commented `#![allow(unsafe_code)]` the count above it already pinned to exactly one. The `allow(unsafe_code)` count,
the `impl Default` search and the `PHASE_1_REAL_ROWS` ban carry the same comment filter and a count or a control
each; the derive-Default walk must report hits on both structs of its control file. The `cargo tree` bans carry
their controls inline (the output must still name `rk-kernel` / `rk-adapter-claude` / `rk-containment` /
`rk-ports`), and the AC-484 python walk asserts it reached `rk-types` and `rk-ports`. **One ban has no control and
the trade is stated where it lives**: the ADR-0101 clause 5 false-approval guard, whose pattern would match its own
FAIL message if it searched the file it is written in (`s7a` §7a.10's shape); it polices the five crates only, and
what `PROGRESS.md`, a commit message or this stage's close-out says is policed by reading. **7b-ii's lesson
applies to it**: the rule is a substring rule that cannot read a negation — Checkpoint 7b-ii's first run failed on
`src/review.rs:31`, which carried the phrase inside the sentence denying it, and `164e212` reworded it — so the
phrase may not appear in a 7c crate even to deny it; the `framing/` fixtures' docs say what they are, not what they
do not claim (§7c.16).

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# ---- THIS BOX FIRST (00-preamble: every block runs on this Windows box): the cfg(windows) arms below are asserted
# ---- BY NAME under this guard; the POSIX arm is ARMED to a recorded ubuntu CI run (ci.yml has never run). The guard
# ---- sits ABOVE every rk_test so no Windows-arm name can pass under the POSIX arm before the guard fires.
case "$(uname -s)" in MINGW*|MSYS*) : ;; *) echo "FAIL: this block runs on the Windows box; the POSIX arm is ARMED to CI and is not this block's evidence"; exit 1;; esac
# ---- THE SHELL, SECOND (the third review: these two guards depended on nothing the block builds and sat ~440
# ---- lines down, after every gate and loop had run): not one Claude Code spawned (its CLAUDECODE / CLAUDE_CODE_*
# ---- names would enter the probe's captured map — §7c.15), not one carrying the persisted user-scope
# ---- ANTHROPIC_FOUNDRY_API_KEY or a tracker credential (both on the committed denylist — §7c.6, RT-25) — otherwise
# ---- Config::new refuses, correctly, and the probe never runs; run under `env -u ANTHROPIC_FOUNDRY_API_KEY bash`.
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — a Claude-spawned shell puts CLAUDECODE/CLAUDE_CODE_* into the probe's captured map (§7c.15)"; exit 1; }
[ -z "${ANTHROPIC_FOUNDRY_API_KEY:-}" ] || { echo "FAIL: ANTHROPIC_FOUNDRY_API_KEY is set in this shell — it is on the committed denylist (§7c.6) and Config::new will refuse; run the block under 'env -u ANTHROPIC_FOUNDRY_API_KEY bash' (never set_var)"; exit 1; }
env | grep -qE '^(GITHUB_TOKEN|GH_TOKEN|GH_ENTERPRISE_TOKEN)=' && { echo "FAIL: a tracker credential (GITHUB_TOKEN / GH_TOKEN / GH_ENTERPRISE_TOKEN — RT-25, AC-509) is set in this shell — it is on the committed denylist and Config::new will refuse; unset it outside the process"; exit 1; }

# ---- BEFORE readings (PIA-035): each PRINTS on the untouched tree and is silent after the step. They are
# ---- READINGS FOR COMPARISON, NOT ASSERTIONS — nothing below consumes them; recorded in the stage-open commit.
test ! -d crates/rk-adapter-claude && echo "BEFORE: no rk-adapter-claude yet"
test ! -d crates/rk-containment && echo "BEFORE: no rk-containment yet"
test ! -f crates/rk-types/src/usage.rs && echo "BEFORE: no rk-types usage module yet"

# ---- Three stopping helpers, defined once. The rule they exist for: never put the command whose status matters
# ---- on the LEFT of a pipe (halts.md shape (c)). rk_test asserts cargo's own status FIRST and then that a test of
# ---- EXACTLY that name ran and passed — the `--` filter is a SUBSTRING match, so an aggregate `test result: ok`
# ---- line is satisfied by a near-miss name and a filter matching nothing exits 0 (s6:211-214, s7a:939); its
# ---- optional FOURTH argument is extra cargo flags (the one use: `--features conformance` for rk-ports, RT-16).
# ---- rk_absent is the split-pipe shape: the search status is asserted BEFORE the comment filter runs, so a
# ---- missing scope directory is a FAIL (rc=2) and never a clean tree; comment lines are excluded because the
# ---- house pattern is a module header quoting the rule it governs (Halt 7). Scope: the FIVE new crates' src/.
# ---- The optional FOURTH argument is a MODULE-PATH filter (s7b-ii's shape): the search runs over the whole scope
# ---- and is filtered to files whose path names the module, so a `headroom.rs` / `headroom/mod.rs` layout choice
# ---- (§7c.1: module file names are the implementer's) is never a grep rc=2 and never a layout-reason FAIL.
# ---- rk_control runs a ban's SAME search and SAME comment filter over the lint-controls directory and FAILs when
# ---- no NON-comment line matches: a control whose token sits on a `//` line, or one a later edit turns into a doc
# ---- comment, would satisfy a bare `grep -q` while the ban filtered it out — and then prove nothing (the third
# ---- review; twenty-three hand-maintained controls drift over six slices).
rk_test() {      # <pkg> <test-name> <what> [<extra cargo flags>]
  out=$(cargo test -p "$1" ${4:-} -- "$2" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { printf '%s\n' "$out" | tail -20; echo "FAIL: cargo test -p $1 ${4:-} -- $2 exited $rc"; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b$2\b.* \.\.\. ok$" \
    || { echo "FAIL: no test named $2 ran and passed in $1 — $3"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  echo "PASS: $2 — $3"
}
SRC_SCOPE="crates/rk-adapter-claude/src crates/rk-adapter-sqlite/src crates/rk-adapter-toolchain-node/src crates/rk-adapter-toolchain-rust/src crates/rk-containment/src"
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
LC=crates/rk-adapter-claude/tests/fixtures/lint-controls
rk_control() {   # <pattern — the SAME one the ban uses> <control file name under $LC>
  c=$(grep -rnE "$1" "$LC" --include="$2" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
  [ -n "$c" ] || { echo "FAIL: control $2 carries no NON-COMMENT line matching '$1' — the ban is vacuous (or the control file is absent)"; exit 1; }
  echo "PASS: control $2 carries the banned token"
}

# ---- the corpus precondition, BEFORE any corpus read (AC-464): the pin resolves THROUGH THE OBJECT STORE, never a
# ---- working tree — so HEAD is not required to equal the pin (S-TTL1's raw frames land in the corpus as a later
# ---- commit without moving CORPUS_PIN, O-9), but the object must exist there and the checkout must be clean.
test -f CORPUS_PIN || { echo "FAIL: CORPUS_PIN absent"; exit 1; }
CORPUS_REPO=$(grep '^CORPUS_REPO=' CORPUS_PIN | cut -d= -f2-)
CORPUS_SHA=$(grep '^CORPUS_SHA=' CORPUS_PIN | cut -d= -f2-)
test -d "$CORPUS_REPO" || { echo "FAIL: corpus absent at $CORPUS_REPO"; exit 1; }
echo "$CORPUS_SHA" | grep -qE '^[0-9a-f]{40}$' || { echo "FAIL: CORPUS_SHA is not an unabbreviated 40-hex SHA"; exit 1; }
git -C "$CORPUS_REPO" cat-file -e "$CORPUS_SHA^{commit}" || { echo "FAIL: CORPUS_SHA $CORPUS_SHA is not a commit in the corpus object store (AC-464: every read resolves through this SHA, never a working tree)"; exit 1; }
d=$(git -C "$CORPUS_REPO" status --porcelain); [ -z "$d" ] || { echo "FAIL: the corpus checkout is dirty; AC-464 refuses before any read"; printf '%s\n' "$d"; exit 1; }

# ---- PREFLIGHTS that must hold BEFORE `cargo test --workspace` runs the real toolchain rows (§7c.14, §7c.17): the
# ---- Rust fixture project is its OWN workspace root (never a member of this one, never in the root exclude line
# ---- Checkpoint 5 greps); the Node fixture project has its lockfile installed; and the AC-334 predecessor checkout
# ---- was materialised ONCE into its persistent local-only path. A box without the installs would otherwise fail
# ---- or stall (npx prompting on a closed stdin until run_bounded's bound) inside the workspace gate, sixty lines
# ---- before the message that explains it.
test -f crates/rk-adapter-toolchain-rust/tests/fixtures/project/Cargo.toml || { echo "FAIL: the Rust toolchain fixture project is absent — every Rust-adapter test needs a crate to run cargo IN that is not this workspace"; exit 1; }
grep -qE '^\[workspace\]' crates/rk-adapter-toolchain-rust/tests/fixtures/project/Cargo.toml || { echo "FAIL: the Rust fixture project carries no [workspace] table — cargo walks up to this root and fails 'believes it's in a workspace when it's not'"; exit 1; }
test -f crates/rk-adapter-toolchain-node/tests/fixtures/project/pnpm-lock.yaml || { echo "FAIL: the Node toolchain fixture project has no committed lockfile"; exit 1; }
test -e crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules/.bin/vitest || { echo "FAIL: run 'pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project' ONCE before this block (network; local-only) — npx without node_modules prompts on a closed stdin"; exit 1; }
PRED=target/predecessor-f21f55fc
test -d "$PRED/.git" || { echo "FAIL: the AC-334 predecessor checkout is absent at $PRED — run the preflight ONCE before this block (§7c.14: git clone \$CORPUS_REPO/predecessor-47faec9f.bundle $PRED && git -C $PRED checkout f21f55fc355eb49b42faf2eb07cad266b647ec1e; network; local-only)"; exit 1; }
ph=$(git -C "$PRED" rev-parse HEAD); [ "$ph" = "f21f55fc355eb49b42faf2eb07cad266b647ec1e" ] || { echo "FAIL: $PRED is at $ph, not f21f55fc… — the F25 record's provenance SHA (its inversion patch's hunk is @@ -1797 there; at REDKILN_SHA the text has moved)"; exit 1; }
test -e "$PRED/node_modules/.bin/vitest" || { echo "FAIL: run 'pnpm install --frozen-lockfile --dir $PRED' ONCE before this block (network; local-only) — npx without node_modules prompts on a closed stdin; no junction to the live clone's node_modules"; exit 1; }
d=$(git -C "$PRED" status --porcelain); [ -z "$d" ] || { echo "FAIL: $PRED is dirty — a previous ac334 run left the inversion applied (its Drop guard did not run); run 'git -C $PRED checkout -- .' and re-run (Halt 6: the same tree reused across arms)"; printf '%s\n' "$d"; exit 1; }

# ---- the five CI gates, each guarded, workspace-scoped (halts.md Halt 1: `cargo build --workspace`, never -p).
# ---- `deny` bites on contact: tokio, windows-sys, libc and the adapters move Cargo.lock.
cargo build --workspace || { echo "FAIL: cargo build --workspace (Halt 1's instrument from Step 5 on) — nothing below is meaningful and no dispatch happens (GC3)"; exit 1; }
cargo fmt --all --check || { echo "FAIL: cargo fmt --all --check"; exit 1; }
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red — AC-001-AC-004's instrument IS this invocation gated on its exit code; nursery WARNINGS are permitted (ADR-0102)"; exit 1; }   # → exit=0
# The feature-gated code (the probe behind `live`, the corpus-bound tests behind `corpus`) is compiled by NO line
# above; this second clippy line lints it, per crate so rk-kernel's own feature is never enabled by it.
cargo clippy -p rk-adapter-claude -p rk-adapter-sqlite -p rk-adapter-toolchain-node -p rk-adapter-toolchain-rust --all-targets --features rk-adapter-claude/live,rk-adapter-claude/corpus,rk-adapter-sqlite/corpus,rk-adapter-toolchain-node/corpus,rk-adapter-toolchain-rust/corpus; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red under the live/corpus features — the probe and the corpus-bound tests are unlinted by the workspace line"; exit 1; }   # → exit=0
out=$(cargo test --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }   # → a FLOOR ONLY
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses"; exit 1; }
# The crate-wide run per NEW crate, asserted separately: every other line below is FILTERED, so a unit test can
# fail under a name no filter names while every guarded line stays green.
for p in rk-containment rk-adapter-sqlite rk-adapter-claude rk-adapter-toolchain-node rk-adapter-toolchain-rust; do
  out=$(cargo test -p "$p" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p $p exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: $p compiled but ran 0 tests"; exit 1; }
done

# ---- the two records this stage committed at stage-open, and the kind ADR-0126 mints (EventKind::ALL 37 -> 38;
# ---- Checkpoint 5 re-run in Slice 1's commit — its ALL-count line is re-run here by value).
ls references/adr/0126-*.md >/dev/null 2>&1 || { echo "FAIL: ADR-0126 (dispatch_completed) is not on the tree — the AC-360 carrier is undecided"; exit 1; }
grep -q '^status: accepted$' references/adr/0126-*.md || { echo "FAIL: ADR-0126 is not at status: accepted — it was committed proposed at stage-open pending the operator's confirmation of CliVersion::{Observed, Pinned} (§7c.20 item 0); the flip commit has not landed"; exit 1; }
ls references/adr/0127-*.md >/dev/null 2>&1 || { echo "FAIL: ADR-0127 (rk-containment) is not on the tree — the sixth crate has no record"; exit 1; }
grep -q '"dispatch_completed"' crates/rk-types/src/event_kind.rs || { echo "FAIL: no dispatch_completed wire name in rk-types EventKind (ADR-0126)"; exit 1; }
grep -qE 'pub const ALL: \[Self; 38\]' crates/rk-types/src/event_kind.rs || { echo "FAIL: EventKind::ALL is not 38 — ADR-0126 adds exactly one kind to 7b-ii's 37"; exit 1; }
test -f crates/rk-types/src/usage.rs || { echo "FAIL: crates/rk-types/src/usage.rs absent — Usage/CacheCreation live in rk-types by ADR-0126 (s7a §7a.5)"; exit 1; }
test ! -d tests/fixtures/rk-adapter-sqlite || { echo "FAIL: tests/fixtures/rk-adapter-sqlite still exists — Cargo cannot hold two members named rk-adapter-sqlite; the rename was the stage-open commit's"; exit 1; }
test -d tests/fixtures/rk-adapter-edge-fixture || { echo "FAIL: the renamed AC-484 fixture crate is absent"; exit 1; }

# ---- PIA-036: each lifted module names the corpus SHA it was lifted from, and it is THE pinned SHA. Checkpoint
# ---- 6's four statements (the header names a 40-hex SHA; CORPUS_PIN has carried it; the file exists in the
# ---- corpus at it; the blob the header names is that file's blob there) plus the equality this stage adds.
# ---- Pairs are "<corpus file> <destination path>": four lifts into rk-adapter-claude and ONE into rk-types
# ---- (usage.rs is lifted from event.rs — §7c.2). journal.rs lifts no bytes and has no file (§7c.2), so no pair.
for pair in "runner.rs crates/rk-adapter-claude/src/runner.rs" "event.rs crates/rk-adapter-claude/src/event.rs" \
            "step.rs crates/rk-adapter-claude/src/card.rs" "throttle.rs crates/rk-adapter-claude/src/throttle.rs" \
            "event.rs crates/rk-types/src/usage.rs"; do
  set -- $pair
  dst=$2
  lift=$(grep -oE '\b[0-9a-f]{40}\b' "$dst" | head -1); test -n "$lift" || { echo "FAIL: $dst's header names no 40-hex corpus SHA (PIA-036)"; exit 1; }
  [ "$lift" = "61f590e51cbb00301c310e46aab615563a7787b8" ] || { echo "FAIL: $dst names lift SHA $lift, not the pinned 61f590e5… every 7c header must carry"; exit 1; }
  git log --format= -p -- CORPUS_PIN | grep -qE "^\+CORPUS_SHA=$lift\$" || { echo "FAIL: $dst names lift SHA $lift, which CORPUS_PIN has never carried (PIA-036)"; exit 1; }
  lift_blob=$(git -C "$CORPUS_REPO" rev-parse --verify -q "$lift:runner/src/$1") || { echo "FAIL: runner/src/$1 is not in the corpus at lift SHA $lift"; exit 1; }
  grep -q "$lift_blob" "$dst" || { echo "FAIL: $dst's header does not name the lifted file's blob $lift_blob at $lift"; exit 1; }
done

# ---- manifests (ADR-0102 d2, ADR-0083): workspace-inherited keys, EXACTLY one integration binary, [lints] last.
for c in rk-containment rk-adapter-sqlite rk-adapter-claude rk-adapter-toolchain-node rk-adapter-toolchain-rust; do
  m=crates/$c/Cargo.toml
  test -f "$m" || { echo "FAIL: $m absent"; exit 1; }
  for k in edition rust-version license publish; do
    grep -qE "^$k(\.workspace[[:space:]]*=[[:space:]]*true|[[:space:]]*=[[:space:]]*\{[[:space:]]*workspace[[:space:]]*=[[:space:]]*true)" "$m" || { echo "FAIL: $m does not inherit $k from the workspace"; exit 1; }
  done
  n=$(grep -cE '^\[\[test\]\]' "$m" | tr -d '[:space:]'); [ "${n:-0}" -eq 1 ] || { echo "FAIL: $m declares ${n:-0} [[test]] targets, not one (ADR-0083: one integration binary)"; exit 1; }
  grep -qE '^name[[:space:]]*=[[:space:]]*"it"' "$m" || { echo "FAIL: $m declares no [[test]] name = \"it\" (ADR-0083: one integration binary)"; exit 1; }
  grep -qE '^path[[:space:]]*=[[:space:]]*"tests/it/main.rs"' "$m" || { echo "FAIL: $m's integration binary is not tests/it/main.rs"; exit 1; }
  test -f crates/$c/tests/it/main.rs || { echo "FAIL: crates/$c/tests/it/main.rs absent"; exit 1; }
  grep -qE '^\[lints\]' "$m" || { echo "FAIL: $m has no [lints] table"; exit 1; }
  tail -n 2 "$m" | grep -qE '^workspace[[:space:]]*=[[:space:]]*true' || { echo "FAIL: [lints] workspace = true is not the LAST table in $m (ADR-0102 d2: workspace = true and nothing else)"; exit 1; }
done
for c in rk-adapter-sqlite rk-adapter-claude rk-adapter-toolchain-node rk-adapter-toolchain-rust; do
  grep -qE '^corpus[[:space:]]*=[[:space:]]*\[\]' crates/$c/Cargo.toml || { echo "FAIL: crates/$c has no [features] corpus = [] (AC-464/PIA-002: corpus-bound tests are local-only)"; exit 1; }
done
grep -qE '^live[[:space:]]*=[[:space:]]*\[\]' crates/rk-adapter-claude/Cargo.toml || { echo "FAIL: rk-adapter-claude has no [features] live = [] — the probe is feature-gated (D17-5)"; exit 1; }
for c in rk-adapter-claude rk-adapter-sqlite rk-adapter-toolchain-node rk-adapter-toolchain-rust; do
  test -f crates/$c/src/registry.rs || { echo "FAIL: crates/$c/src/registry.rs absent — every rk-adapter-* ships a capability registry (AC-489)"; exit 1; }
  grep -qE '^[[:space:]]*(pub(\([a-z]+\))? )?verified_on[[:space:]]*:' crates/$c/src/registry.rs || { echo "FAIL: crates/$c's registry row type declares no verified_on member (AC-489: {capability, verified_on, probe}) — a comment or a TODO naming the token is not a member"; exit 1; }
  grep -qE '^[[:space:]]*(pub(\([a-z]+\))? )?probe[[:space:]]*:[[:space:]]*Probe\b' crates/$c/src/registry.rs || { echo "FAIL: crates/$c's registry row type declares no required probe: Probe member (§7c.7: probe is a closed Probe::{Recorded, None { reason }}, never absent)"; exit 1; }
  grep -qE '#\[serde\([^]]*deny_unknown_fields' crates/$c/src/registry.rs || { echo "FAIL: crates/$c's registry carries no #[serde(deny_unknown_fields)] attribute — AC-489's instrument is 'serde deny-unknown + required fields' (the converse of the AC-201 ban, which is module-scoped for this reason)"; exit 1; }
  grep -qE '^serde(\.workspace)?[[:space:]]*=' crates/$c/Cargo.toml || { echo "FAIL: crates/$c has no serde dependency — the registry's derive(Deserialize) lives in serde, not serde_json (§7c.19)"; exit 1; }
done
# RT-24: one vocabulary, four copies — the four `enum Capability` bodies are byte-equal (a digest over the sed
# range), and the range matched something (else four empty digests are trivially equal — the control).
grep -qE '^pub enum Capability[[:space:]{]' crates/rk-adapter-claude/src/registry.rs || { echo "FAIL: no column-0 'pub enum Capability' in rk-adapter-claude's registry.rs — the byte-equality line below would compare four empty ranges (RT-24, AC-492)"; exit 1; }
caps=$(for c in rk-adapter-claude rk-adapter-sqlite rk-adapter-toolchain-node rk-adapter-toolchain-rust; do sed -n '/^pub enum Capability[[:space:]{]/,/^}/p' crates/$c/src/registry.rs | sha256sum | cut -d' ' -f1; done | sort -u | grep -c . | tr -d '[:space:]')
[ "${caps:-0}" -eq 1 ] || { echo "FAIL: the four adapters' enum Capability bodies differ (${caps:-0} distinct digests) — four private vocabularies are not AC-492's canonical one (RT-24, §7c.7)"; exit 1; }   # → 1
# rk-containment's manifest: ADR-0127 decision 2's dependency sentence, asserted on the manifest and the source.
grep -qE '^tokio' crates/rk-containment/Cargo.toml && { echo "FAIL: rk-containment's manifest names tokio — ADR-0127 d2: its dependencies are std, windows-sys (Windows) and libc (Unix)"; exit 1; }
grep -qE '^rk-(types|ports|adapter-[a-z-]+)' crates/rk-containment/Cargo.toml && { echo "FAIL: rk-containment's manifest names an rk-* crate — ADR-0127 forbids rk-ports and any rk-adapter-*, and d2 names no rk-types"; exit 1; }
rk_absent '\btokio\b' 'ADR-0127 d2: no tokio in rk-containment — the fixture child and run_bounded wait without it (§7c.10, §7c.17)' 'crates/rk-containment/src crates/rk-containment/tests'
rk_control '\btokio\b' adr0127_tokio_control.rs
# RT-16: rk-ports' conformance bodies are behind `[features] conformance = []`, enabled by the adapters' DEV edges
# only; rk-kernel's NORMAL graph never enables it (S-FEAT — the feature edge walk names rk-ports, the control).
grep -qE '^conformance[[:space:]]*=[[:space:]]*\[\]' crates/rk-ports/Cargo.toml || { echo "FAIL: rk-ports has no [features] conformance = [] — the operator's routing (RT-16 as re-amended): the bodies are feature-gated, never unconditional"; exit 1; }
grep -qE '^#\[cfg\(feature = "conformance"\)\][[:space:]]*$' crates/rk-ports/src/lib.rs || { echo "FAIL: rk-ports/src/lib.rs does not gate the conformance module with #[cfg(feature = \"conformance\")] on its own line above the mod item (RT-16)"; exit 1; }
out=$(cargo tree -p rk-kernel -e features 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-kernel -e features exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
hits=$(printf '%s\n' "$out" | grep -E 'rk-ports feature "conformance"')
[ -z "$hits" ] || { echo "FAIL: rk-kernel's normal feature graph enables rk-ports/conformance — only the adapters' dev-dependencies may (RT-16; S-FEAT)"; printf '%s\n' "$hits"; exit 1; }   # → silent
printf '%s\n' "$out" | grep -q 'rk-ports' || { echo "FAIL: the feature tree names no rk-ports — the ban above searched text that names nothing (positive control)"; exit 1; }
# (the toolchain fixture-project preflights sit ABOVE the five gates — the workspace gate runs the real rows.)

# ---- the ONE allow(unsafe_code), in rk-containment, in the module-level form, commented (ADR-0102, ADR-0127);
# ---- and ZERO other #[allow( in the five new crates, src/ AND tests/ (everything else is #[expect]). The count is
# ---- taken over ATTRIBUTE lines only (anchored on `#!?[allow(`), so the required comment above the attribute may
# ---- name the token (Halt 7: a guard that fires on a comment is fixed by a filter, never dropped); the
# ---- lint-controls directory is excluded from the count (its ADR-0102 control carries #[allow(dead_code)], §7c.17).
h=$(grep -rnE '^[[:space:]]*#!?\[allow\(unsafe_code' crates/ --include=*.rs --exclude-dir=lint-controls); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the allow(unsafe_code) search could not run (grep rc=$rc)"; exit 1; }
c=$(printf '%s\n' "$h" | grep -c . | tr -d '[:space:]')
[ "${c:-0}" -eq 1 ] || { echo "FAIL: expected exactly ONE allow(unsafe_code) attribute in the workspace, found ${c:-0} (ADR-0102; ADR-0127)"; printf '%s\n' "$h"; exit 1; }   # → 1
printf '%s\n' "$h" | grep -q '^crates/rk-containment/src/' || { echo "FAIL: the one allow(unsafe_code) is not in rk-containment (ADR-0127 supersedes ADR-0099 d3's location clause IN PART)"; exit 1; }
printf '%s\n' "$h" | grep -qE '#!\[allow\(unsafe_code\)\]' || { echo "FAIL: the allow is not the module-level #![allow(unsafe_code)] form"; exit 1; }
uf=$(printf '%s\n' "$h" | head -1 | cut -d: -f1); un=$(printf '%s\n' "$h" | head -1 | cut -d: -f2)
sed -n "$((un - 1))p" "$uf" | grep -qE '^[[:space:]]*(//|/\*|\*)' || { echo "FAIL: the allow(unsafe_code) in $uf is not commented on the line above it (ADR-0102's override rule requires the comment)"; exit 1; }
raw=$(grep -rn 'unsafe' crates/rk-containment/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the unsafe search could not run (grep rc=$rc)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -v "^$uf:")
[ -z "$hits" ] || { echo "FAIL: an unsafe token outside $uf — ADR-0127 confines unsafe to the ONE module; libc::killpg is unsafe too, so the POSIX arm lives in it (§7c.10)"; printf '%s\n' "$hits"; exit 1; }   # → silent
# Both attribute forms — `#[allow(` and the INNER `#![allow(` (a crate-level `#![allow(clippy::pedantic)]` carries `#!`
# before `[` and the outer-only pattern never matched it) — over src/, tests/it AND the compiled fixture bin; the one
# permitted line, the commented `#![allow(unsafe_code)]` counted above, is excluded by the fifth argument.
rk_absent '#!?\[allow\(' 'ADR-0102 d3: no #[allow( / #![allow( in the five new crates, src/, tests/it and the fixture bin — every override is #[expect(clippy::…, reason)]' "$SRC_SCOPE crates/rk-adapter-claude/tests/it crates/rk-adapter-sqlite/tests/it crates/rk-adapter-toolchain-node/tests/it crates/rk-adapter-toolchain-rust/tests/it crates/rk-containment/tests/it crates/rk-containment/tests/fixtures/bin" '' 'allow\(unsafe_code\)'
rk_control '^[[:space:]]*#\[allow\(dead_code' adr0102_allow_attribute_control.rs
rk_control '^[[:space:]]*#!\[allow\(dead_code' adr0102_allow_attribute_control.rs
grep -q 'allow(unsafe_code' "$LC"/adr0102_allow_attribute_control.rs && { echo "FAIL: the ADR-0102 control spells allow(unsafe_code) and disturbs the exactly-one count above"; exit 1; }

# ---- derive-Default as CAPTURED hits, never a piped while (the 211-line form exited a subshell). Usage,
# ---- CacheCreation, WindowReading and Timings are rk-types' (ADR-0126); Margins and ApiRetry are the adapter's.
# ---- The derive is found by a CONTIGUOUS-ATTRIBUTE WALK upward from `pub struct NAME` (over `#[`, `///`, `//!`,
# ---- plain `//` and `/* */` comment lines, blank and continuation lines — a `-B3` window misses a stacked
# ---- serde/schemars attribute set and mis-attributes the previous struct's derive; a walk that stopped at a plain
# ---- `//` line missed a derive with an ordinary comment between it and the struct, the third review's finding);
# ---- the impl search is comment-filtered; the control file must report hits on BOTH its structs.
dd_hits() {      # <file> <struct>: prints derive hits (the contiguous attribute block above the struct naming Default)
  python -c '
import re, sys
path, name = sys.argv[1], sys.argv[2]
lines = open(path, encoding="utf-8").read().split("\n")
for i, l in enumerate(lines):
    if re.match(r"^pub(\(crate\))? struct " + re.escape(name) + r"\b", l):
        j = i - 1; attrs = []
        while j >= 0 and (lines[j].strip() == "" or re.match(r"^\s*(#!?\[|///|//!|//|/\*|\*|\)|\]|[A-Za-z_,\s]+[,)\]]\s*$)", lines[j])):
            attrs.insert(0, lines[j]); j -= 1
        text = " ".join(a.strip() for a in attrs)
        for m in re.finditer(r"derive\(([^)]*)\)", text):
            if re.search(r"\bDefault\b", m.group(1)): print(f"{path}: derive({m.group(1).strip()}) on {name}")
' "$1" "$2"
}
for pair in "rk-types Usage" "rk-types CacheCreation" "rk-types WindowReading" "rk-types Timings" "rk-adapter-claude Margins" "rk-adapter-claude ApiRetry"; do
  set -- $pair
  f=$(grep -rlE "^pub(\(crate\))? struct $2\b" crates/$1/src/); test -n "$f" || { echo "FAIL: no pub struct $2 at column 0 in crates/$1/src — the Default ban has nothing to search (AC-362/AC-377; the six structs are declared pub struct at column 0, §7c.17)"; exit 1; }
  hits=$(for m in $f; do dd_hits "$m" "$2"; done)
  [ -z "$hits" ] || { echo "FAIL: $2 derives Default in crates/$1 — PIA-062 requires Option over 0, with NO Default (AC-362/AC-377)"; printf '%s\n' "$hits"; exit 1; }   # → silent
  raw=$(grep -rnE "impl Default for $2\b" crates/$1/src/); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: the impl Default search for $2 could not run (grep rc=$rc)"; exit 1; }
  hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
  [ -z "$hits" ] || { echo "FAIL: $2 has a hand-written Default in crates/$1 (AC-362/AC-377)"; printf '%s\n' "$hits"; exit 1; }   # → silent
done
ctl=crates/rk-adapter-claude/tests/fixtures/lint-controls/ac362_derive_default_control.rs
hits=$(dd_hits "$ctl" Usage); test -n "$hits" || { echo "FAIL: the derive-Default walk reports no hit on its own control ($ctl carries #[derive(Debug, Default)] on its own line ABOVE pub struct Usage {} — a one-line form is never found by the walk, §7c.17) — the ban above is vacuous"; exit 1; }
hits=$(dd_hits "$ctl" WindowReading); test -n "$hits" || { echo "FAIL: the derive-Default walk reports no hit on the control's SECOND struct ($ctl carries #[derive(Debug, Default)], then a plain // comment line, then pub struct WindowReading {} — the walk must cross an ordinary comment, §7c.17) — the ban above is vacuous on that layout"; exit 1; }
grep -qE 'impl Default for CacheCreation' "$ctl" || { echo "FAIL: the derive-Default control carries no impl Default for CacheCreation — the impl half of the ban is vacuous"; exit 1; }

# ---- the named tests, one per criterion, asserted BY NAME in the crate the design names.
for t in api_key_refuses_construction structured_output_required env_denylist_config_map golden_argv_per_profile \
         argv_cap_refuses_before_spawn rate_limited_from_stream tick_arithmetic_vectors cancel_reaps_process_tree \
         ac523_text_marker_detection_disclosed ac523_subtype_detection_synthetic_frame \
         ac372_reset_crossing_splits_per_window ac372_low_sample_at_n9 ac372_interleaved_not_attributable \
         ac200_unknown_events_retained_verbatim ac201_future_shaped_event_still_ok \
         ac202_isolation_from_system_init_not_argv ac203_agents_without_agent_refused \
         ac204_adoption_equal_subset_superset_wrong_agent ac386_extra_tools_not_isolated_no_retry \
         ac226_version_mismatch_configured_outcome ac206_exit0_success_without_structured_output_is_schema \
         ac208_prose_json_never_recovered ac209_system_prompt_hash_invariant ac210_both_ttl_vars_set_and_recorded \
         ac526_ttl_pair_refuses_single_or_mismatch ac212_model_effort_from_bound_card_only \
         ac214_permission_mode_explicit ac215_permission_prompts_none_stdin_closed ac216_bare_absent_across_matrix \
         ac511_bare_never_in_argv_lint_with_control ac422_denylist_is_one_exported_list \
         ac218_is_error_is_platform_regardless_of_exit ac219_headroom_from_rate_limit_event_only \
         ac380_rate_limit_event_sole_source_both_shapes ac220_refuse_within_margin_two_ticks \
         ac221_overage_refuses_before_spawn ac378_overage_zero_attempts_no_spawn \
         ac225_cancelled_keyed_on_token_not_exit_code ac227_every_harness_event_carries_cli_version \
         ac229_adapter_maps_every_failure_to_closed_variant ac232_api_retry_counted_per_event_no_own_retry \
         ac238_claude_registry_entry_dated ac489_capability_registry_dated ac492_unknown_capability_token_fails \
         ac360_one_event_per_termination_path ac368_tier2_append_failure_is_warning ac374_ticks_roundtrip_decimal_digits \
         ac375_throttle_input_has_two_windows_only ac376_throttle_refuses_without_spawn_and_appends_row \
         ac377_margin_parse_vectors_and_no_default ac381_headroom_calculator_still_delivers_a_delta \
         ac383_card_without_profile_refused ac385_version_on_event_and_mismatch_warning \
         adr0113_ttl_classifier_five_states ac267_resume_cost_is_summed_across_turns_and_option \
         ac493_empty_or_absent_tools_rejected ac132_builder_lints_one_schema_no_strict_tools \
         ac269_answer_api_absent_from_cards ac509_child_echoes_env_without_tracker_credential \
         dispatch_cwd_is_the_spec_worktree_never_process_cwd ; do
  rk_test rk-adapter-claude "$t" "the adapter criterion this name binds (§7c.18)"
done
for t in ac278_no_descendant_survives_exit run_bounded_kills_tree_at_timeout ; do
  rk_test rk-containment "$t" "containment (ADR-0127; AC-278, AC-331)"
done
for t in port_conformance_real_row_sqlite wal_and_busy_timeout_are_enforced ac364_correction_appends_never_mutates \
         ac365_missing_store_file_recreated ac368_sqlite_append_error_is_typed_not_panic ac369_identity_mismatch_refuses_open \
         ac370_one_store_per_common_dir ac243_gate_open_replay_over_sqlite_store \
         ac489_capability_registry_dated ac492_unknown_capability_token_fails ; do
  rk_test rk-adapter-sqlite "$t" "the store half this name binds (§7c.13)"
done
for p in rk-adapter-toolchain-node rk-adapter-toolchain-rust; do
  for t in ac328_affected_selection_unsupported_without_probe ac329_report_conforms_no_prose_parse \
           ac325_recorded_argv_is_os_vector ac331_killed_verify_is_unverifiable ac497_classify_binary_paths \
           ac344_missing_binary_is_not_a_candidate_failure ac489_capability_registry_dated ac492_unknown_capability_token_fails ; do
    rk_test "$p" "$t" "the toolchain criterion this name binds (§7c.14)"
  done
done
rk_test rk-adapter-toolchain-node port_conformance_real_row_node 'AC-499: the Node real row runs rk-ports'"'"' own body'
rk_test rk-adapter-toolchain-node d34_vitest_invoked_directly_never_pnpm_forwarded 'DECISIONS-2026-09-09 #34: <npx.cmd|npx> vitest run <file> [-t <pattern>] as separate argv elements'
rk_test rk-adapter-toolchain-rust port_conformance_real_row_rust 'AC-499: the Rust real row (ADR-0094)'
rk_test rk-adapter-toolchain-rust nextest_fallback_recorded_on_report 'ADR-0094 d1: nextest ABSENT on this box (measured 2026-09-17), so the fallback is recorded on the report'
# RT-26: the Rust count route on stable is libtest's `--list` (libtest --format json is nightly-only on 1.98.1,
# measured); the route's spelling is asserted on the adapter's source, anchored on a string literal, beside the
# ac325_ line that asserts it on the recorded vectors.
grep -rqE '"--list"' crates/rk-adapter-toolchain-rust/src --include=*.rs || { echo "FAIL: the Rust adapter's source names no \"--list\" argv element — the stable count route (RT-26, §7c.14) is not the one built; a prose parse or a nightly-only reporter is the alternative and both are refused"; exit 1; }
rk_test rk-adapter-toolchain-rust rust_pack_fixture_binds_kinds_per_adr0029 'ADR-0094 d1: the Rust technology pack binds TestReport/Changeset keyed to rust (read in tests through rk_kernel::toml_subset over the dev edge)'
# AC-334's real row is the NODE adapter's (the F25 fixture is a vitest suite — §7c.14) and is corpus-bound; it reads
# the persistent predecessor checkout the preflight above asserted (never clones, never installs) and runs vitest
# under run_bounded with the Node adapter's construction-time timeout — the bound on THIS wait (Halt 5).
out=$(cargo test -p rk-adapter-toolchain-node --features corpus -- ac334_flip_predicate_real_toolchain 2>&1); rc=$?   # bounded by run_bounded's timeout (Halt 5)
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p rk-adapter-toolchain-node --features corpus -- ac334_flip_predicate_real_toolchain exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\bac334_flip_predicate_real_toolchain\b.* \.\.\. ok$' || { echo "FAIL: no test named ac334_flip_predicate_real_toolchain ran and passed — AC-334 real row: the F25 inversion is mutation-survived against the real Node adapter"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# ---- THE IDS WHOSE SUBJECT CRATE IS NOT AN ADAPTER, each filtered IN THE CRATE THE DESIGN NAMES — and each of
# ---- those crates has a passed checkpoint, so its row is provisional in the same commit (§7c.19).
for p_t in "rk-types ac362_absent_usage_is_none_not_zero" \
           "rk-types ac362_absent_and_zero_roundtrip_distinct" \
           "rk-types adr0126_dispatch_completed_is_tier_two" \
           "rk-types ac360_dispatch_record_carries_the_usage_fields" \
           "rk-types ac523_zero_backoff_table" \
           "rk-ports ac364_event_store_port_has_no_update_or_delete" \
           "rk-store retention_record_is_rk_types_dispatch_record" \
           "rk-kernel ac484_adapter_edge_fixture" ; do
  set -- $p_t
  rk_test "$1" "$2" "its register subject crate is $1 (design §9.1), and its checkpoint row is provisional until re-run"
done
# rk-ports' own conformance run enables its own feature on the command line (RT-16 — the way its corpus feature is
# already run); the test module is #[cfg(feature = "conformance")] and a run without the flag would select nothing.
rk_test rk-ports port_conformance_fake_rows 'AC-499: the fake rows over the core/capability-arm bodies (Checkpoint 7a provisional)' '--features conformance'
# PHASE_1_REAL_ROWS: guarded (the const must still be where this line looks), counted (four rows), then the
# empty-list ban with the comment filter — an absence check with no guard and no control asserts nothing (Halt 8 (g)).
grep -q 'PHASE_1_REAL_ROWS' crates/rk-ports/tests/it/conformance.rs || { echo "FAIL: PHASE_1_REAL_ROWS moved out of crates/rk-ports/tests/it/conformance.rs — re-point this line"; exit 1; }
# counted in EITHER layout: one-line `("EventStore", "SQLite", &[…]),` or rustfmt's multi-line form once the assertion
# lists fill and the tuple exceeds max_width 100 (no rustfmt.toml overrides it), where `"SQLite",` ends its own line.
c=$(grep -cE '"(SQLite|Claude|Node|Rust)",[[:space:]]*(&\[|$)' crates/rk-ports/tests/it/conformance.rs | tr -d '[:space:]')
[ "${c:-0}" -eq 4 ] || { echo "FAIL: PHASE_1_REAL_ROWS no longer carries four rows (found ${c:-0}) — AC-499 names four"; exit 1; }   # → 4
raw=$(grep -nE '"(SQLite|Claude|Node|Rust)", &\[\]' crates/rk-ports/tests/it/conformance.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the PHASE_1_REAL_ROWS search could not run (grep rc=$rc)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*)')
[ -z "$hits" ] || { echo "FAIL: PHASE_1_REAL_ROWS still carries an empty assertion list — AC-499's four rows are filled per slice"; printf '%s\n' "$hits"; exit 1; }   # → silent

# ---- the trybuild harness, filtered on the HARNESS TEST'S OWN NAME with --test it (its file is fixed at
# ---- crates/rk-adapter-claude/tests/it/ui.rs); never two free filters after a doubled `--`.
out=$(cargo test -p rk-adapter-claude --test it -- trybuild_pins_the_reason_each_claude_fixture_fails 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trybuild must-fail suite did not pass (rc=$rc) — a fixture COMPILED, or its .stderr drifted"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_claude_fixture_fails\b.* \.\.\. ok$' || { echo "FAIL: no test of that name ran and passed — a filter matching nothing exits 0"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
test -f crates/rk-adapter-claude/tests/ui/ac381_cache_field_on_headroom_input.rs || { echo "FAIL: AC-381's compile-fail case is absent — the type-level separation is the criterion's first instrument"; exit 1; }
test -s crates/rk-adapter-claude/tests/ui/ac381_cache_field_on_headroom_input.stderr || { echo "FAIL: AC-381's fixture commits no .stderr, so it is green whenever compilation fails for ANY reason"; exit 1; }

# ---- the absence bans: each GUARDED (test -d), PAIRED with a named test above, and POSITIVELY CONTROLLED under
# ---- tests/fixtures/lint-controls/ (never tests/ui/, the trybuild glob root — §7c.17). Scoped to the five new
# ---- crates' src/, NOT all of crates/ — the all-of-crates form fails today on other crates' debt (32 hits).
for d in $SRC_SCOPE; do test -d "$d" || { echo "FAIL: $d absent — every ban below would read as a pass over a missing directory (Halt 8 shape (a))"; exit 1; }; done
test -d "$LC" || { echo "FAIL: lint-controls absent — an absence ban with no control is not an assertion"; exit 1; }
rk_absent 'set_var|remove_var' 'ADR-0083: set_var/remove_var are unsafe fn in edition 2024 — forbidden workspace-wide, tests included; the adapter constructs the CHILD env instead' "$SRC_SCOPE crates/rk-adapter-claude/tests/it crates/rk-adapter-sqlite/tests/it crates/rk-adapter-toolchain-node/tests/it crates/rk-adapter-toolchain-rust/tests/it crates/rk-containment/tests/it"
rk_control 'set_var|remove_var' adr0083_set_var_control.rs
rk_absent 'append-system-prompt|exclude-dynamic-system-prompt-sections' 'AC-211: BOTH append flags are absent from argv in every profile — S-H1c is unrun'
rk_control 'append-system-prompt|exclude-dynamic-system-prompt-sections' ac211_append_flag_control.rs
rk_absent 'taskkill|kill_tree' 'AC-224: the runner'"'"'s taskkill /T /F tree walk is NOT lifted — containment is the Job Object / pgid (ADR-0099 d3, ADR-0127)'
rk_control 'taskkill|kill_tree' ac224_taskkill_control.rs
rk_absent 'result\.rate_limit|rate_limit_from_result' 'AC-219/AC-380: result.rate_limit is never a headroom source; the runner'"'"'s rate_limit_from_result is NOT lifted'
rk_control 'result\.rate_limit|rate_limit_from_result' ac219_result_rate_limit_control.rs
# AC-374's ban is MODULE-PATH filtered (fourth argument), never file-path scoped: headroom (the digit scanner AND the
# total_cost_usd scanner, which lives beside it — RT-19, §7c.1), throttle, and event (the line classifier that hands
# the raw bytes over). The helper asserts ADMISSION (≥ 1 file matches the filter) before it filters; the two modules
# the ban must reach are asserted to exist in the ADMISSION shape (a `headroom/mod.rs` layout is admitted, §7c.1 —
# the second review's `test -f` by exact path would have failed it for the layout reason the prose said cannot
# happen); and the pattern covers the ROUNDING path's spellings (as_f64 etc. — `\bf64\b` does not see `as_f64`,
# since `_` is a word character; §7c.8).
n=$(find crates/rk-adapter-claude/src -name '*.rs' | grep -cE '/([^/]*/)?(headroom)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no headroom module under crates/rk-adapter-claude/src — the digit scanner and cost scanner's module (§7c.2, §7c.8), which the AC-374 ban must reach"; exit 1; }
n=$(find crates/rk-adapter-claude/src -name '*.rs' | grep -cE '/([^/]*/)?(throttle)(\.rs|/)' | tr -d '[:space:]'); [ "${n:-0}" -ge 1 ] || { echo "FAIL: no throttle module under crates/rk-adapter-claude/src — the lifted guard (§7c.2), which the AC-374 ban must reach"; exit 1; }
rk_absent 'interpolat|smooth|\bf64\b|\bf32\b|as_f64|as_f32|from_f64|\.round\(|as_number\(' 'AC-374: no interpolation, smoothing, f64/f32, as_f64/as_f32/from_f64, .round( or as_number( in the headroom / throttle / event modules — the lint is the discriminator for the rounding path (§7c.8)' 'crates/rk-adapter-claude/src' 'headroom|throttle|event'
rk_control 'interpolat|smooth|\bf64\b|\bf32\b|as_f64|as_f32|from_f64|\.round\(|as_number\(' ac374_f64_headroom_control.rs
grep -q 'as_f64(' "$LC"/ac374_f64_headroom_control.rs || { echo "FAIL: the AC-374 control does not carry as_f64( — the rounding path's spelling is uncontrolled"; exit 1; }
# the module-path filter's OWN control, both directions: it must EXCLUDE the lint-controls path (grep -H forces the
# filename prefix on a single-file operand — without it grep -r over one file prints `line:content` and the path
# regex could never match, the third review's finding: the second review's line passed unconditionally) and it must
# ADMIT a headroom.rs path.
raw=$(grep -HnE '\bf64\b' "$LC"/ac374_f64_headroom_control.rs | grep -E '^[^:]*/([^:]*/)?(headroom|throttle|event)(\.rs|/)'); [ -z "$raw" ] || { echo "FAIL: the module-path filter admitted the lint-controls path — the filter regex is wrong"; exit 1; }
printf '%s\n' "crates/rk-adapter-claude/src/headroom.rs:1:x" | grep -qE '^[^:]*/([^:]*/)?(headroom|throttle|event)(\.rs|/)' || { echo "FAIL: the module-path filter admits no headroom.rs line — the filter regex is wrong in the other direction"; exit 1; }
rk_absent 'headroom_saved|cache_headroom|seven_day_(opus|sonnet)|five_hour_(opus|sonnet)' 'AC-381/AC-375: no cache-derived headroom field and no family-keyed headroom identifier'
rk_control 'headroom_saved|cache_headroom|seven_day_(opus|sonnet)|five_hour_(opus|sonnet)' ac375_family_key_control.rs
rk_absent 'unwrap_or\(0\)|unwrap_or_default\(\)' 'AC-362: no unwrap_or(0)/unwrap_or_default() on the usage path — a Some(0) makes a TTL claim look verified (Halt 11)' 'crates/rk-adapter-claude/src crates/rk-types/src/usage.rs'
rk_control 'unwrap_or\(0\)|unwrap_or_default\(\)' ac362_unwrap_or_zero_control.rs
# ADR-0118's shape is a POISON recovery in any spelling — `PoisonError`, `catch_unwind`, or an `unwrap_or_else(…
# into_inner)` over a lock — never every `.into_inner()`: BufWriter/Cursor/Arc/Mutex::into_inner are ordinary in a
# crate that writes child output to a file and waits on a channel, and a ban that wide refuses valid work (Halt 7).
rk_absent 'PoisonError|catch_unwind|unwrap_or_else\([^)]*into_inner' 'ADR-0118: a poisoned store lock halts the run — no poison recovery in any spelling in the new crates (rk-store conn.rs'"'"'s own violation is RAISED, not repaired here)'
rk_control 'unwrap_or_else\(PoisonError::into_inner\)' adr0118_poison_recovery_control.rs
rk_absent '\banyhow\b' 'ADR-0102 d6: anyhow is confined to rk-cli/redkiln-migrate; thiserror in every library'
rk_control '\banyhow\b' adr0102_anyhow_control.rs
rk_absent '"--bare"' 'AC-511/AC-216: --bare in no argv construction site (the property test is the instrument; this is the cheap second line)'
rk_control '"--bare"' ac511_bare_flag_control.rs
# AC-201's ban is MODULE-PATH filtered to the stream decoder (event, init) AND reaches rk-types' usage.rs (the result
# frame's `usage` object deserialises into rk_types::Usage — an unknown key there is "an unknown field on a known
# event", §7c.3): AC-489 REQUIRES deny_unknown_fields on every registry.rs (asserted positively above), so a
# crate-wide ban and AC-489 cannot both hold.
rk_absent 'deny_unknown_fields' 'AC-201: the stream decoder and the usage type tolerate unknown fields and event types' 'crates/rk-adapter-claude/src crates/rk-types/src/usage.rs' 'event|init|usage'
rk_control 'deny_unknown_fields' ac201_deny_unknown_fields_control.rs
# AC-202's lint half (§7c.4): no isolation assertion reads the adapter's own argv — the files that read system/init
# and decide adoption/isolation name no argv, args, build_args or extra_flags.
rk_absent '\bargv\b|\bargs\b|build_args|extra_flags' 'AC-202: no isolation assertion reads the adapter'"'"'s own argv — isolation is decided from the child'"'"'s system/init (the flag in argv is never evidence)' 'crates/rk-adapter-claude/src' 'init|isolation'
rk_control '\bargv\b|\bargs\b|build_args|extra_flags' ac202_argv_isolation_control.rs
rk_absent 'std::env::var|env::var\(|env::vars\(' 'AC-217: the adapters never read the process environment — the captured map is a Config value (ADR-0083); only the live probe reads env, at its edge, in tests/it'
rk_control 'std::env::var|env::var\(|env::vars\(' ac217_env_var_control.rs
rk_absent 'Command::new\("git"\)' 'ADR-0077: no git spawn outside rk-git'"'"'s one wrapper — --git-common-dir and the root commit go through GitExec (the Harness adapter takes no git edge at all)'
rk_control 'Command::new\("git"\)' adr0077_git_command_control.rs
rk_absent 'pnpm test' 'DECISIONS-2026-09-09 #34: never pnpm test -- forwarding; vitest run directly' 'crates/rk-adapter-toolchain-node/src'
rk_control 'pnpm test' d34_pnpm_test_control.rs
rk_absent '\+1\.98\.1' 'ADR-0102/ADR-0083: no third copy of the toolchain pin — rust-toolchain.toml and rust-version are the two the bump PR moves' 'crates/rk-adapter-toolchain-rust/src'
rk_control '\+1\.98\.1' adr0102_toolchain_pin_control.rs
rk_absent 'Handle::block_on|current_thread' 'ADR-0076: an adapter-owned multi_thread Runtime, block_on from the sync port method only, never Handle::block_on' 'crates/rk-adapter-claude/src'
rk_control 'Handle::block_on|current_thread' adr0076_handle_block_on_control.rs
# ADR-0127's central forbids, as a floor with a control (the third review found it had none): no adapter LIBRARY
# calls std Command's spawn()/output()/status() — a Command is BUILT in the adapter and HANDED to rk-containment
# (spawn_into_job / run_bounded). The pattern is the zero-argument call form, so tokio's `handle.spawn(future)` and
# `spawn_blocking(` are not matched (Halt 7: a guard that would fire on the stderr reader's task is too wide).
rk_absent '\.(spawn|output|status)\(\)' 'ADR-0127 d3/forbids: no direct spawn in an adapter library — every child is spawned through rk-containment' 'crates/rk-adapter-claude/src crates/rk-adapter-toolchain-node/src crates/rk-adapter-toolchain-rust/src'
rk_control '\.(spawn|output|status)\(\)' adr0127_direct_spawn_control.rs
rk_absent 'tokio::process::(Command|Child)\b' 'RT-13 as amended: the tokio Command/Child TYPES are unused in the Harness adapter — the three PIPES are wrapped with tokio::process::ChildStdin/ChildStdout/ChildStderr::from_std and the std child is waited on in spawn_blocking (§7c.10); tokio::process::Child has no from_std' 'crates/rk-adapter-claude/src'
rk_control 'tokio::process::(Command|Child)\b' adr0127_spawn_path_control.rs
# AC-208's lint half (§7c.5): no JSON-from-prose scraper on the result path — no regex engine, no fenced-block scrape,
# no from_str over a text member. Crate-wide over the Harness adapter's src/ (no module filter, so no vacuity).
rk_absent 'Regex::new|\bregex::|from_str\([^)]*\.(text|prose|content)\b' 'AC-208: no JSON-from-prose extraction (regex or fenced-block scraper) on the result path — the runner'"'"'s text-parse branch is NOT lifted' 'crates/rk-adapter-claude/src'
rk_control 'Regex::new|\bregex::|from_str\([^)]*\.(text|prose|content)\b' ac208_prose_scrape_control.rs
# AC-329's lint half (§7c.14): no prose parse of toolchain output in either toolchain adapter — the counts come from
# libtest's `--list` (RT-26) / vitest's JSON reporter file; the fixture projects' decoys make a prose parse
# observable. The summary line, the per-test `... ok` / `... FAILED` lines and the vitest summary are banned; a bare
# `passed;` is not (it matched any statement ending in a field named `passed`, which the honest adapter reads).
rk_absent '\.\.\. ok|\.\.\. FAILED|test result:|[0-9]+ passed;|Tests  [0-9]+ passed' 'AC-329: no prose or markdown parse of toolchain output in the toolchain adapters (brief §C.5)' 'crates/rk-adapter-toolchain-node/src crates/rk-adapter-toolchain-rust/src'
rk_control '\.\.\. ok|\.\.\. FAILED|test result:|[0-9]+ passed;|Tests  [0-9]+ passed' ac329_prose_parse_control.rs

# ---- the dependency graph (AC-528, AC-484/AC-005, S-FEAT): rk-kernel's normal graph names no async runtime and
# ---- no adapter; tokio is reached from rk-adapter-claude ONLY (ADR-0127 d2: never from rk-containment; RT-15); the
# ---- Harness crate's normal graph never reaches rusqlite (no rk-store edge, §7c.19). The python walk carries its
# ---- own positive control: it must have reached rk-types and rk-ports, or it walked nothing.
out=$(cargo tree -p rk-kernel -e normal 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-kernel -e normal exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
hits=$(printf '%s\n' "$out" | grep -E '\b(tokio|async-std|futures|async-trait)\b')
[ -z "$hits" ] || { echo "FAIL: rk-kernel's normal graph names an async runtime (AC-528; ADR-0076) — the adapters' tokio leaked"; printf '%s\n' "$hits"; exit 1; }   # → silent
hits=$(printf '%s\n' "$out" | grep -E '\brk-(adapter-[a-z-]+|containment)\b')
[ -z "$hits" ] || { echo "FAIL: rk-kernel's normal graph reaches an adapter or rk-containment (AC-484/AC-005)"; printf '%s\n' "$hits"; exit 1; }   # → silent
printf '%s\n' "$out" | grep -q '^rk-kernel ' || { echo "FAIL: the tree output does not start with rk-kernel — the two bans above searched the wrong text"; exit 1; }
out=$(cargo tree --workspace -e normal -i tokio 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -i tokio exited $rc — tokio is expected in the lock (rk-adapter-claude)"; printf '%s\n' "$out" | tail -20; exit 1; }
hits=$(printf '%s\n' "$out" | grep -E '\brk-(kernel|types|ports|store|git|containment)\b')
[ -z "$hits" ] || { echo "FAIL: a closed crate or rk-containment depends on tokio through its normal graph (S-FEAT's shape; AC-528; ADR-0127 d2)"; printf '%s\n' "$hits"; exit 1; }   # → silent
printf '%s\n' "$out" | grep -q 'rk-adapter-claude' || { echo "FAIL: the inverted tokio tree names no rk-adapter-claude — the ban above searched text that names nothing (positive control)"; exit 1; }
out=$(cargo tree -p rk-adapter-claude -e normal 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-adapter-claude -e normal exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
hits=$(printf '%s\n' "$out" | grep -E '\b(rusqlite|libsqlite3-sys|rk-store)\b')
[ -z "$hits" ] || { echo "FAIL: rk-adapter-claude's normal graph reaches rk-store / rusqlite — the Harness crate has no store edge (§7c.19)"; printf '%s\n' "$hits"; exit 1; }   # → silent
printf '%s\n' "$out" | grep -q 'rk-containment' || { echo "FAIL: rk-adapter-claude's normal graph does not name rk-containment — the ban above searched the wrong text (positive control; ADR-0127 d3)"; exit 1; }
hits=$(printf '%s\n' "$out" | grep -E '\brk-git\b')
[ -z "$hits" ] || { echo "FAIL: rk-adapter-claude's normal graph reaches rk-git — the pre-spawn snapshot is not lifted and the probe's git edge is dev-only (§7c.2, §7c.19)"; printf '%s\n' "$hits"; exit 1; }   # → silent
# ADR-0127 d3's "all three spawning adapters depend on it": the two toolchain crates' normal graphs name it too (the
# second review asserted the edge for rk-adapter-claude only).
out=$(cargo tree -p rk-adapter-toolchain-node -e normal 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-adapter-toolchain-node -e normal exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -q 'rk-containment' || { echo "FAIL: rk-adapter-toolchain-node's normal graph does not name rk-containment (ADR-0127 d3: every spawning adapter depends on it)"; exit 1; }
out=$(cargo tree -p rk-adapter-toolchain-rust -e normal 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-adapter-toolchain-rust -e normal exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -q 'rk-containment' || { echo "FAIL: rk-adapter-toolchain-rust's normal graph does not name rk-containment (ADR-0127 d3: every spawning adapter depends on it)"; exit 1; }
meta=$(cargo metadata --format-version 1 --locked 2>/dev/null); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo metadata exited $rc"; exit 1; }
printf '%s' "$meta" | python -c '
import json, sys
m = json.load(sys.stdin)
byid = {n["id"]: n for n in m["resolve"]["nodes"]}
root = next(p["id"] for p in m["packages"] if p["name"] == "rk-kernel")
seen, todo = set(), [root]
while todo:
    n = todo.pop()
    if n in seen: continue
    seen.add(n)
    for d in byid[n]["deps"]:
        if any(k["kind"] in (None, "build") for k in d["dep_kinds"]): todo.append(d["pkg"])
pname = {p["id"]: p["name"] for p in m["packages"]}
names = sorted({pname[i] for i in seen})
assert "rk-types" in names and "rk-ports" in names, names
bad = [x for x in names if x.startswith("rk-adapter-") or x == "rk-containment"]
if bad: print("reached:", ", ".join(bad), file=sys.stderr)
sys.exit(1 if bad else 0)
' || { echo "FAIL: cargo metadata: rk-kernel reaches an rk-adapter-* or rk-containment node through its normal/build graph (AC-484's AND: 'an added edge fails CI naming the adapter crate' — the names are on stderr above), or the walk reached neither rk-types nor rk-ports (the control)"; exit 1; }

# ---- the Windows cancel arm, BY NAME, on THIS box, under the uname guard at the top of the block.
rk_test rk-adapter-claude ac224_windows_job_object_reaps_orphaned_grandchild 'AC-224 Windows arm: with the child ALIVE and dispatch in flight, the token fires and the Job Object (KILL_ON_JOB_CLOSE, created before spawn) reaps the orphaned holder a PID walk misses, before the handle drops; the breakaway attempt fails'
rk_test rk-containment ac224_job_object_created_before_spawn 'ADR-0099 d3 / ADR-0127: the job exists before the child does — two parent-side readings before ResumeThread: IsProcessInJob(child, our_job) TRUE and a previous-suspend-count of 1; no child-side line (cargo wraps every test process in a KILL_ON_JOB_CLOSE job — §7c.10)'
rk_test rk-containment orphan_grandchild_reaped_via_job 'O-1/O-2 measured at Slice 2 in the crate that owns the primitive: the orphaned holder (parent exited, child alive) is reaped via the job, read after group.kill() and before the handle drops, which a PID walk misses'
rk_test rk-containment breakaway_attempt_fails_without_breakaway_ok 'O-2 negative control: a CREATE_BREAKAWAY_FROM_JOB attempt fails under a job that never grants BREAKAWAY_OK (ADR-0127 d7)'

# ---- what is authored and left unrun (AC-312, S-SUBTLE-CB; AC-215's integration half) and the Rust pack fixture:
# ---- existence of NON-EMPTY files, honestly (Halt 8 (h): a zero-byte file is not an authored fixture).
c=$(find crates/rk-adapter-claude/tests/fixtures/framing -type f -size +0 2>/dev/null | grep -c . | tr -d '[:space:]')
[ "${c:-0}" -ge 2 ] || { echo "FAIL: fewer than two non-empty AC-312 framing-variant fixture files (a defective item AND its approval-framed card) — authored here, unrun until S-SUBTLE-CB (DECISIONS-2026-09-11 #17)"; exit 1; }
grep -rqE '^[[:space:]]*(async[[:space:]]+)?fn[[:space:]]+live_disallowed_tool_terminates\b' crates/rk-adapter-claude/tests/it/ || { echo "FAIL: AC-215's integration half (live_disallowed_tool_terminates, #[ignore], --features live) is not authored as a fn — a comment or a TODO naming it is not an authored test; it is ARMED, not skipped"; exit 1; }
test -f crates/rk-adapter-toolchain-rust/tests/fixtures/pack/pack.toml || { echo "FAIL: the Rust technology pack fixture is absent (ADR-0094 d1)"; exit 1; }
# RT-11: the two keys are routed spellings and these are existence floors that move with the pack schema record (§7c.1);
# anchored on `key =` so a `#` comment naming the token does not satisfy them.
grep -qE '^[[:space:]]*test_timeout[[:space:]]*=' crates/rk-adapter-toolchain-rust/tests/fixtures/pack/pack.toml || { echo "FAIL: the Rust pack carries no test_timeout key — AC-213's human input has no slot"; exit 1; }
grep -qE '^[[:space:]]*gate_record[[:space:]]*=' crates/rk-adapter-toolchain-rust/tests/fixtures/pack/pack.toml || { echo "FAIL: the Rust pack names no gate_record key for test_timeout — AC-213 is a human gate, and no CI job may pass or fail on it"; exit 1; }
raw=$(grep -rniE 'guarded against false approval|calibrated against false approval' crates/rk-adapter-claude/ crates/rk-adapter-sqlite/ crates/rk-adapter-toolchain-node/ crates/rk-adapter-toolchain-rust/ crates/rk-containment/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the false-approval search could not run (grep rc=$rc; 2 = no such path)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a claim that the review gate is guarded against false approval appears in a 7c crate — ADR-0101 clause 5 forbids it until S-SUBTLE-CB runs"; printf '%s\n' "$raw"; exit 1; }   # → silent

# ---- THE FIFTEENTH LINE, as statements. A freshly BUILT probe (cargo test does not rebuild — Halt 1/GC3); the
# ---- instrument captured and compared to the re-teed file (Halt 2); every path from the probe's own print, then
# ---- test -s (Halt 8 (h)); structured_output asserted from the RAW result frame the probe wrote to RESULT_RAW_PATH
# ---- — a non-empty object or list, beside subtype success and is_error false — never from a typed field (which
# ---- serialises as "structured_output": null on the F20 shape) and never from the exit code (Halt 3); the
# ---- dispatch_completed row counted through the real EventStore::query, never inferred from a non-empty sqlite
# ---- file (its schema page alone makes it non-empty). Foreground, operator present (GC2). One haiku dispatch.
# ---- THE WAIT: the `$(cargo test …)` capture blocks until libtest returns, and its bound is the probe card's
# ---- timeout_secs = 300 (§7c.15) expiring on the ADAPTER'S OWN timeout arm (the token has none — §7c.10), which
# ---- kills the child tree through the group — Halt 5's failure alternative. The block's ONE other waiter is the
# ---- AC-334 line above, bounded by run_bounded's timeout.
# ---- THE SHELL was checked at the TOP of the block (the CLAUDECODE / ANTHROPIC_FOUNDRY_API_KEY / tracker guards sit
# ---- under the uname guard, so a wrong shell fails before any gate runs — §7c.6, §7c.15).
test -s spikes/results/instrument-checks/RUNBOOK-instrument.txt || { echo "FAIL: RUNBOOK-instrument.txt absent or EMPTY (Halt 8 (h): a failed re-tee leaves a 0-byte file)"; exit 1; }
pin=$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)
printf '%s' "$pin" | grep -qE '^2\.1\.[0-9]+ \(Claude Code\)$' || { echo "FAIL: the instrument file does not read '2.1.N (Claude Code)': '$pin'"; exit 1; }
live=$(claude --version 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: claude --version exited $rc"; exit 1; }
[ "$live" = "$pin" ] || { echo "FAIL: Halt 2 — claude --version is '$live' and the pinned instrument is '$pin'. Record the move in PROGRESS.md §1/§3/§6 and re-tee; do not continue silently"; exit 1; }
grep -qE "^[[:space:]]*pub const TESTED_CLI_VERSION:[[:space:]]*&str[[:space:]]*=[[:space:]]*\"${pin%% *}\"" crates/rk-adapter-claude/src/config.rs || { echo "FAIL: config.rs declares no 'pub const TESTED_CLI_VERSION: &str = \"${pin%% *}\"' (ADR-0078: the pin is recorded with the measurement, as a Config value — §7c.12 fixes the declaration form; a doc comment or a test string naming the version is not the constant)"; exit 1; }
cargo build -p rk-adapter-claude --features live --tests || { echo "FAIL: the live probe did not build fresh — do not run a stale binary (Halt 1)"; exit 1; }
out=$(cargo test -p rk-adapter-claude --features live -- --ignored --nocapture live_dispatch_probe 2>&1); rc=$?   # bounded by the card's timeout_secs = 300 (Halt 5)
[ "$rc" -eq 0 ] || { echo "FAIL: live_dispatch_probe exited $rc — the one real dispatch did not complete (GC4)"; printf '%s\n' "$out" | tail -40; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\blive_dispatch_probe\b.* \.\.\. ok$' || { echo "FAIL: no test named live_dispatch_probe ran and passed — an #[ignore] test not selected exits 0 printing 0 passed"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# each path is printed on its own line and read TO END OF LINE (a temp path may carry a space — a `[^[:space:]]+`
# read would truncate it silently and the `test -s` below would fail with a misleading message, Halt 11's class).
rec=$(printf '%s\n' "$out" | sed -n 's/^RECORD_PATH=//p' | head -1); test -n "$rec" || { echo "FAIL: the probe printed no RECORD_PATH= line"; exit 1; }
test -s "$rec" || { echo "FAIL: the record file $rec is absent or empty"; exit 1; }
raw=$(printf '%s\n' "$out" | sed -n 's/^RESULT_RAW_PATH=//p' | head -1); test -n "$raw" || { echo "FAIL: the probe printed no RESULT_RAW_PATH= line — Halt 3 has no raw frame to read"; exit 1; }
test -s "$raw" || { echo "FAIL: the raw result frame file $raw is absent or empty"; exit 1; }
argv=$(printf '%s\n' "$out" | sed -n 's/^ARGV_PATH=//p' | head -1); test -n "$argv" || { echo "FAIL: the probe printed no ARGV_PATH= line — the row carries argv as a digest only (ADR-0105 c7), so AC-215's tag lift has no carrier"; exit 1; }
test -s "$argv" || { echo "FAIL: the argv file $argv is absent or empty"; exit 1; }
sto=$(printf '%s\n' "$out" | sed -n 's/^STORE_PATH=//p' | head -1); test -n "$sto" || { echo "FAIL: the probe printed no STORE_PATH= line"; exit 1; }
test -s "$sto" || { echo "FAIL: the sqlite store $sto is absent or empty (Halt 8 (h): a file check, NOT evidence of a row)"; exit 1; }
printf '%s\n' "$out" | grep -q '^PROBE_RAN_CONFORMANCE=1$' || { echo "FAIL: the probe did not run rk-ports' shared conform_harness body over the real adapter (AC-499: the same assertions execute against both implementations — §7c.14, §7c.15)"; exit 1; }
python -c '
import json, sys
f = json.load(open(sys.argv[1], encoding="utf-8"))
v = f.get("structured_output")
ok = isinstance(v, (dict, list)) and len(v) > 0 and f.get("subtype") == "success" and f.get("is_error") is False
sys.exit(0 if ok else 1)
' "$raw" || { echo "FAIL: Halt 3 — the RAW result frame at $raw carries no non-empty structured_output beside subtype success / is_error false; an exit code of 0 is not evidence (F20)"; exit 1; }
# the raw frame file IS the frame the probe retained: the probe prints the sha256 of the bytes it wrote, and the block
# digests the file it read (ADR-0105 c7's carrier; the second review's token-presence grep asserted nothing, since
# "structured_output" is on the serialised row on every path, as null on the F20 shape).
want=$(printf '%s\n' "$out" | sed -n 's/^RESULT_RAW_SHA256=//p' | head -1); test -n "$want" || { echo "FAIL: the probe printed no RESULT_RAW_SHA256= line"; exit 1; }
[ "$(sha256sum "$raw" | cut -d' ' -f1)" = "$want" ] || { echo "FAIL: the raw result frame at $raw does not digest to the RESULT_RAW_SHA256 the probe printed — the file read is not the frame retained"; exit 1; }
grep -qE '"subtype":[[:space:]]*"init"' "$rec" || { echo "FAIL: the raw system/init was not retained — AC-511's probe and AC-202/AC-204's first recorded init both need it"; exit 1; }
# the pin as instrument_version (the Config value serialised on the row) — an existence floor, NOT the measurement;
# shape-tolerant on the colon's whitespace (§7c.15 fixes compact serde_json::to_string, and a pretty writer must not
# turn a true fact into a FAIL that blames the pin — Halt 11's class):
grep -qE "\"instrument_version\":[[:space:]]*\"${pin%% *}\"" "$rec" || { echo "FAIL: the serialised row carries no instrument_version equal to the pin ${pin%% *} (AC-360's pinned version; ADR-0126 d1)"; exit 1; }
# the OBSERVED version, printed by the probe from the raw init's version field on its own line — a grep for the pin
# over $rec could not discriminate (instrument_version IS the pin on the same file; Halt 11's shape):
obs=$(printf '%s\n' "$out" | sed -n 's/^OBSERVED_CLI_VERSION=//p' | head -1); test -n "$obs" || { echo "FAIL: the probe printed no OBSERVED_CLI_VERSION= line from the raw system/init (AC-227/AC-385)"; exit 1; }
[ "$obs" = "${pin%% *}" ] || { echo "FAIL: the child reported CLI version '$obs' and the pin is '${pin%% *}' — AC-226's mismatch outcome fired on the probe, or the init field the probe read is not the version (§7c.11)"; exit 1; }
grep -qx -- '--permission-prompts' "$argv" || { echo "FAIL: the argv the builder produced carries no --permission-prompts element — AC-215's [surface unverified] tag is not lifted by this run"; exit 1; }
n=$(printf '%s\n' "$out" | grep -oE '^COMPLETED_ROWS=[0-9]+' | head -1 | cut -d= -f2); [ "${n:-0}" -eq 1 ] || { echo "FAIL: expected exactly one dispatch_completed row in the store through EventStore::query, got ${n:-0} (AC-360; ADR-0126)"; exit 1; }   # → 1
n=$(printf '%s\n' "$out" | grep -oE '^DISPATCH_ROWS=[0-9]+' | head -1 | cut -d= -f2); [ "${n:-1}" -eq 0 ] || { echo "FAIL: expected zero kernel 'dispatch' rows — the probe calls Harness::dispatch directly and no kernel runs (§7c.15), so a non-zero count means the adapter wrote the kernel's kind"; exit 1; }   # → 0
echo "CP7C-END-OF-BLOCK reached"
```

### 7c.22 Acceptance set

The claimed set is the register's harness area — **AC-200..AC-239 minus AC-207 (retired) plus AC-523 and AC-526**
(allocated outside the exhausted range) — and its event-log / cost / throttling area — **AC-360..AC-399** — **plus**
the ids other records route here by name and the 211-line file omitted: **AC-488** (construction half), **AC-511**,
**AC-499** (four rows), **AC-484/AC-005** (the `cargo tree`/metadata half), **AC-489/AC-492/AC-493/AC-509**
(`s7a` §7a.11), **AC-132** (builder-lint half), **AC-528** (re-run), **AC-497** (classify half), **AC-325/AC-328/AC-329/
AC-331/AC-334/AC-350** (armed from 7b-ii), **AC-278** (armed from 7b-ii; the containment primitive), **AC-344**
(the adapter's share of the verify runner's missing-binary case), **AC-312** (fixtures), and the **adapter halves of
AC-282, AC-243, AC-267, AC-269, AC-279** cross-listed from 7b-ii and of **AC-417, AC-422, AC-428, AC-439**
cross-listed from 7d. It is written as explicit ranges minus the ids excluded, each named with its reason
(`00-preamble.md`'s legend; PIA-070, PIA-071) — and the Totality sentence at the end covers exactly this list, so
no id is discharged that the list does not name (the second review discharged AC-278 and AC-344 unlisted).

**This section claims by id ONLY what a discriminating test at this stage can pin.** Everything else is **ARMED, NOT
DISCHARGED**, by id and reason (golden constraint 10: powerless is reported as powerless).

**At 7c, `crates/` holds `rk-git`, `rk-kernel`, `rk-ports`, `rk-store`, `rk-types` and nothing else** [measured
2026-09-17 at `6152219`]. **There is no `rk-cli` and no `doctor`; `.redkiln/` does not exist; no lane has run; `ci.yml`
has never run; the corpus retains no raw stream transcript.**

**DISCHARGED HERE, by a discriminating test at this stage** — each named in §7c.18 and asserted by name in the block:

- **Stream, isolation, adoption:** AC-201 (`ac201_…`, with the `usage`-level unknown key), AC-202 (`ac202_…` plus
  the AC-202 argv-isolation ban and its control — both instruments), AC-203 (builder half; `ac203_…`), AC-204
  (`ac204_…`), AC-226 (both configurations; `ac226_…`; the refusal's variant is routed under RT-20 and RAISED below),
  AC-386 (`ac386_…`). AC-200 is PROVISIONAL (below).
- **Builder and card:** AC-205 (`structured_output_required`), AC-208 (`ac208_…`), AC-209 (`ac209_…`), AC-212
  (`ac212_…`), AC-214 (**unit half only**; `ac214_…`; the lint half — *"a lint that the flag is unconditional on the
  dispatch path"* — is ARMED below), AC-215 (**unit half only**; `ac215_…`; the [surface unverified] tag on the floor is
  lifted by the probe's recorded `--permission-prompts none` argv at 2.1.275 ≥ 2.1.259, asserted in the block; the
  integration half is ARMED below), AC-216 (`ac216_…`), AC-383 (golden-argv and card-validator halves;
  `golden_argv_per_profile`, `ac383_…`), AC-493 (`ac493_…`), AC-132 (builder-lint half; `ac132_…`), AC-269
  (**card-validator half only**; `ac269_answer_api_absent_from_cards`; the catalog iterated is RAISED below; the
  build-configuration half is ARMED below), AC-523's argv cap (`argv_cap_refuses_before_spawn`).
- **Environment:** AC-217 (`env_denylist_config_map`), AC-210 (`ac210_…`), AC-526 (both arms; `ac526_…`), AC-509
  (`ac509_…`; the tracker names are RT-25's routing), AC-488 (construction half; `api_key_refuses_construction`),
  AC-511 (first conjunct; `ac511_…`).
- **Classification:** AC-206 (`ac206_…`), AC-218 (`ac218_…`), AC-523's `text_marker` arm, zero-backoff table and
  `detected_by` disclosure (`rate_limited_from_stream`, `ac523_text_marker_detection_disclosed`; the zero-backoff
  table is `rk_types::StepError::retry_class()`'s and is asserted `-p rk-types` as `ac523_zero_backoff_table` — no
  adapter mutation moves it),
  AC-225 (`ac225_…`; the adapter's own timer arm yields `Cancelled { by: Timeout }`, the polled token `Signal` —
  §7c.10), AC-227 (**for `dispatch_completed` only**; `ac227_…`; the kernel's version-less rows are RAISED below),
  AC-232 (adapter half; `ac232_…`), AC-238 (registry half; `ac238_claude_registry_entry_dated` over the named
  `dispatch` row plus the four per-crate `ac489_…` lines), AC-489 and AC-492 (**all four adapters** — `ac489_…`,
  `ac492_…` in each crate's loop, plus the block's `enum Capability` digest — RT-24). AC-221 is PROVISIONAL (below).
- **Headroom and TTL:** AC-372 (calculator half; `tick_arithmetic_vectors` + three `ac372_…`), AC-374 (`ac374_…`
  discriminates truncation; the extended lint discriminates rounding — §7c.8), AC-375 (`ac375_…`), AC-377 (config
  half; `ac377_…`, under O-3's default), AC-380 (**the recorded `unifiedWindows` arm**; `ac380_…`; the flat SDK arm
  is PROVISIONAL below), AC-219 (`ac219_…`), AC-220 (`ac220_…`), AC-362 (`-p rk-types` `ac362_…` ×2), AC-385
  (`ac385_…`, through the `mismatch_warning` accessor — §7c.9), AC-381 (positive half
  `ac381_headroom_calculator_still_delivers_a_delta`; compile-fail half in the trybuild harness; **allocation
  RAISED** below — built here, subject `rk-kernel`). AC-376 is PROVISIONAL (below).
- **Record and containment:** AC-360 (adapter half; `ac360_…` and `-p rk-types` `ac360_dispatch_record_carries_the_usage_fields`),
  AC-368 (**the sqlite negative only** — `ac368_sqlite_append_error_is_typed_not_panic`; the warning half is
  PROVISIONAL below), AC-224 (Windows arm; `cancel_reaps_process_tree`,
  `ac224_windows_job_object_reaps_orphaned_grandchild`; POSIX arm ARMED below). AC-278 is ARMED (below) by half —
  its primitive is built here and asserted by name (`-p rk-containment` `ac278_…`), its GIVEN is the gate path.
- **Store:** AC-364 (both halves; `ac364_…` and `-p rk-ports` `ac364_event_store_port_has_no_update_or_delete`), AC-365
  (recreate half; `ac365_…`), AC-369 (`ac369_…`), AC-370 (`ac370_…`). AC-398 and AC-371 are CROSS-LISTED (below), not
  discharged here; AC-399 is ARMED whole (below).
- **Toolchains:** AC-328 (both crates; `ac328_…`; both rows carry no probe), AC-329 (adapter half; `ac329_…`, the
  Rust side over the `--list` route — RT-26), AC-325 (`ac325_…`), AC-331 (toolchain half; `ac331_…`), AC-334 (**the
  Node real row**, `corpus`-gated `ac334_…`; the Rust half is ARMED below), AC-499 (**three rows by name** over the
  core/capability-arm bodies; the Claude row local-only via `live_dispatch_probe`, which runs the SAME
  spec-parameterised `conform_harness` body — which prints its own `PROBE_RAN_CONFORMANCE=1` — before the probe's
  prints, RT-16, §7c.14; **the CI-matrix instrument is ARMED below**). AC-344 is CROSS-LISTED (below).
- **Built and tested, NOT claimed as evidenced (PROVISIONAL, gating the label not the build):** AC-200 (discharged
  under the 64 KiB cap; *"verbatim"* above it is RAISED — `ac200_…`), AC-211 (S-H1c; `golden_argv_per_profile`),
  AC-221 (no true-valued `isUsingOverage` reading; built over the flipped e1-smoke payload; `ac221_…` — S-RETRY's
  exhaustion arm is the candidate lift), AC-267 (no baseline — Halt 9; `ac267_resume_cost_…`), AC-378 (no true-valued
  reading; `ac378_…`), AC-497 (S-BIN; `ac497_…`), AC-523's `subtype` arm (synthetic frame; [surface unverified];
  `ac523_subtype_detection_synthetic_frame`), **AC-376** (built and tested as
  `ac376_throttle_refuses_without_spawn_and_appends_row`, assertions unchanged; the `rate_limited` producer question
  is RAISED and the register amendment OWED — §7c.8), **AC-368's warning half** (built as
  `ac368_tier2_append_failure_is_warning`, narrowed to what both records permit; R-16's register amendment is OWED —
  §7c.13), and **AC-380's flat SDK arm** (`// SYNTHETIC`, [surface unverified] — §7c.8).

**CROSS-LISTED — claimed at both sites, the adapter half claimed here** (the opposite of excluded):

- **AC-282** — kernel half discharged at 7b-ii; adapter half here via `ac206_…` (**the split note `s7b-ii` §7b-ii.14
  owed to this file**; PIA-047's placement erratum still OWED).
- **AC-243** — kernel half 7b-ii; the real-store replay `ac243_gate_open_replay_over_sqlite_store` here, no new
  instrument. **AC-267** — see PROVISIONAL above. **AC-269** — wholly here. **AC-279** — kernel count discharged at
  7b-ii; **no record names an adapter instrument** for it, so nothing is claimed here beyond the usage fields the
  count could read — RAISED.
- **AC-329** — field-semantics half 7b-ii (`b213137`); adapter half here (**the matching note `s7b-ii` §7b-ii.9
  owed**).
- **AC-417, AC-428, AC-439** — inside `s7d`'s AC-400–AC-439 range; the card-validator / adapter halves are here
  (`structured_output_required`, `ac218_…`, `ac206_…`), the `doctor` rows and the stream-corpus replay conjuncts are
  7d's or RAISED; **matching notes in `s7d` OWED**.
- **AC-488, AC-422, AC-423** — `s7d` carries AC-488 by name and the other two by range; 7c = construction refusal /
  the one exported list / the TTL rule (AC-526) that one `doctor` row will read; 7d = the edge read and the rows.
- **AC-229** — the set is Step 5's, the kernel match 7b's; the adapter's exhaustive match is here (`ac229_…`).
- **AC-484, AC-005, AC-528** — the kernel/workspace halves were **claimed and discharged at `s5` (AC-005's
  `deny.toml`/fixture half) and `s7b` (AC-484's `cargo metadata` walk and fixture commit; AC-528's `rk-kernel` clause
  over the resolved graph; AC-005's `cargo tree -i` half)** — `s7b` §7b.15's claimed-by-id list and `s5`'s
  acceptance set carry them, and the first form of this section placed all three under DISCHARGED HERE, a third-site
  claim with no matching note (the double claim this file's own totality rule forbids). What 7c adds is the **first
  non-vacuous run** of those assertions now that real `rk-adapter-*` crates are on the tree (`s7b`'s block says of
  its AC-484 line *"by package absence at 7b; falsifiable from 7c"* and of AC-005's inverse assertion *"ARMED, not
  discharged, and is re-run at Checkpoint 7c"*), extended to `rk-containment` (ADR-0127 decision 3): the `cargo
  tree` / `cargo metadata` lines and `-p rk-kernel ac484_adapter_edge_fixture` in the block. **Matching notes in
  `s5` and `s7b` OWED**.
- **AC-398** — the row-count-unchanged instrument is `rk-store`'s (`reconcile.rs`, Checkpoint 6) and the adapter
  exposes no delete, so no adapter test can discriminate it (§7c.13); the drift-report renderer is ARMED to 7d.
- **AC-371** — register subject `rk-store`; its *"schema test asserting no branch/path column is part of any key"*
  is over `0001_initial.sql`, which the adapter applies unchanged and cannot add a key column to, so no adapter
  mutation flips it (§7c.13) — Step 6's to carry; **a matching note in `s6` is OWED**.
- **AC-344** — register subject `rk-kernel-verify`; its instrument's *"cannot-run fixture (missing binary) that
  must yield `Unverifiable`"* names a class `StepError` does not carry; the adapter's share is
  `ac344_missing_binary_is_not_a_candidate_failure` (`Transport { kind: Spawn }`, never a failing test — the input
  7d's verify runner maps to `Unverifiable { cannot_run }`, §7c.14); **a matching note in `s7d` is OWED**.
- **AC-334** — the predicate is 7b-ii's; its real row is here — the **Node** row (the F25 fixture is a vitest suite);
  the Rust half is ARMED below.

**ARMED, NOT DISCHARGED — by id and by reason.** Checkpoint 7c may not be read as discharging any of these, and each is
recorded in `PROGRESS.md` §1 as carried forward:

| id | register subject and instrument | missing at 7c | where it can run |
| --- | --- | --- | --- |
| **AC-203, AC-205/AC-417, AC-212, AC-220, AC-226, AC-369, AC-370, AC-377, AC-383, AC-422, AC-423, AC-493, AC-526** *(their `doctor` halves only)* | a `doctor` rule-table row | `rk-cli` / `doctor` | **7d**. Each has a half that DOES run here and is listed as discharged above; both states are carried |
| **AC-366, AC-397, AC-395** | `doctor --rebuild` / the rebuilt-store rule / `redkiln cost` rendering | `rk-cli` | **7d**; AC-395 also Step 8 exit (7) |
| **AC-372** *(renderers + doctor rule)*, **AC-398** *(drift report)*, **AC-365** *(next-action equality)* | the renderer, the report, the FSM replay | `rk-cli` / kernel replay | **7d** / **Step 8** |
| **AC-399** *(whole)* | *"every such row is marked non-comparable in the store"* / *"no non-comparable row is included in any cost aggregate"*; a store test over all of a run's rows | **the carrier is undecided** — a payload member by record (ADR-0126 decision 6's typed-payload record), a marking kind by ADR-0116, or a `0002` migration; the register defines no adapter half, and the adapter's share (the `Err(NotIsolated)` outcome and the `dispatch_completed` row that records it) is built under AC-202/AC-204/AC-360 | **when the carrier record lands** (the mark) and **7d** (the `redkiln cost` exclusion); the second review's `comparable: false` payload flag is withdrawn (§7c.13) |
| **AC-278** *(the gate-path GIVEN)* | *"GIVEN the kernel has opened a gate and the opening invocation has exited (AC-240)"* — an enumeration of the run's job / pgid after exit | no process is spawned on the gate path at 7c (`s7b-ii` §7b-ii.14's own row); the enumeration primitive and `-p rk-containment` `ac278_no_descendant_survives_exit` over a fixture child are built here (§7c.10) | **7d**, where AC-240's invocation exists and `s7b-ii` armed it |
| **AC-214** *(lint half)* | *"a lint that the flag is unconditional on the dispatch path"* | no grep can read whether a `push("--permission-mode")` sits outside every guard without parsing Rust (§7c.5); the unit half `ac214_…` runs here | a source-parse instrument named by a later stage, or a register note narrowing the instrument to the matrix test |
| **AC-499** *(the CI-matrix instrument)* | *"a CI matrix row per instantiation"* beside the suite | the stage's `ci.yml` edits add no matrix rows and `ci.yml` has never run; the three real rows run by name in the block and the fourth locally | **a CI run** with three named matrix rows (SQLite, Node, Rust), when `ci.yml` first runs |
| **RT-26's per-test-result arm** *(the Rust adapter)* | a machine-readable *per-test result* from the Rust toolchain | `libtest --format json` is nightly-only on 1.98.1 and nextest is absent [measured]; the `--list` route counts what would run and reads pass/fail from the exit status (§7c.14) | a box with nextest, or a stable libtest reporter; the arm is unobservable here |
| **AC-224** *(POSIX arm)* | `process_group(0)` + `killpg`, enumerated via pgid | a POSIX box; `ci.yml` has never run | **a recorded CI `ubuntu-latest` run** |
| **AC-215** *(integration half)* | *"an integration test with a fixture that requests a disallowed tool, which must terminate rather than hang under a wall-clock bound"* | the real CLI, a second paid dispatch the operator did not decide | authored as `live_disallowed_tool_terminates` (`--features live`, `#[ignore]`, §7c.15); **run at the operator's word or at 7d** |
| **AC-269** *(build-configuration half)* | *"a build-configuration test that the answer API is not linked into the adapter's tool registry"* | `rk-cli`, whose absence makes a no-edge assertion vacuous | **7d**, re-run there |
| **AC-334** *(Rust half)* | the flip predicate over a Rust ordering-inversion fixture | no such fixture exists and nothing authored may be called F25 (a vitest suite) | when a Rust inversion fixture is authored and labelled authored; the Node row runs here |
| **ADR-0094 decision 1's `cargo nextest run` presence arm** | *"(or `cargo nextest run` where present, recorded on the report)"* | `cargo nextest` is ABSENT on this box [measured 2026-09-17]; `nextest_fallback_recorded_on_report` pins the fallback's recording only | a box that has nextest; the arm is unobservable here and an implementation that never considers nextest passes the fallback test |
| **AC-328** *(a probe on either `affected-selection` row)* | *"capability-registry unit test; a fixture adapter with an expired `verified_on`"* — the expired-probe arm | any recorded affected-selection probe at the pin (none exists) | **S-SEL-NODE** (§7c.16), a named spike OWED; both rows yield `Unsupported` until it runs |
| **AC-223** | the pack timeout ≥ recorded slow-condition maximum lint | a pack lint reading the event store | **7d/7e**; the per-activity timeout as the in-band cancel is built here |
| **AC-384** | the `static`-after-gate pack lint naming ~30k vs 18,603 | pack lint | **7e** (pack-lint owner); the 18,603 attribution conflict is RAISED below |
| **AC-350** | one verify result per claiming toolchain | `Ports` holds ONE `Arc<dyn Toolchain>` | **7d** (composite at the composition root; record OWED) |
| **AC-321** | a replayable verify-run event | a minted verify-run `EventKind` (D17-4) | when the record is written; 7c's toolchain adapters write no verify-run event |
| **AC-320, AC-339, AC-341, AC-348, AC-349, AC-353, AC-355, AC-359, AC-338** *(harness/content-hash half)* | the verify **activity** runner, a disposable clone around a real mutation run, the CI/Tracker integration halves, affected-area selection + its 200-change-set property test, the binder over toolchain claims, `taskkill` against a verify run, the selection regression suite, ledger recompute after re-open | the verify activity runner is the composition root's, not an adapter's; 7c ships the toolchain adapters it needs | **7d** for the runner; **Step 8** for the kill; *"naming a destination is not routing it there"* — **a re-routing record is OWED** for this block, which `s7b-ii` armed to 7c and no adapter criterion claims |
| **AC-391** | inner retries as tier-2 events, one charge per event | a per-attempt kind (ADR-0122 d6 mints none); the wiring record R-22 (DECISIONS-2026-09-17) and the clarification R-2 (DECISIONS-2026-09-15) | **ARMED by register instruction**; the adapter half is AC-232's |
| **AC-361, AC-367, AC-387** | item cost rollup; every `EventKind` carries a tier tag (built at Step 5 by shape, `ALL: [Self; 38]` with `tier()`); the cost model over measured cells | **claimed by NO stage** (0 hits in `runbook/` and `crates/` for AC-367 [measured 2026-09-17]) | kernel-side; **ARMED with reason — an owner record is OWED** |
| **AC-498** | `rk-kernel` + pack-validation suites under `--network none` | a CI run | CI, never run; 7c keeps adapters out of `rk-kernel`'s graph (asserted above) |
| **AC-312** | the release-regression corpus job | S-SUBTLE-CB | fixtures authored here, unrun; ADR-0101 clause 5 governs |
| **AC-356** | — | — | **not ARMED here — see Excluded below** (PROVISIONAL on S-BIN); this row is a pointer, not a second placement |

**Excluded, by id and reason** — only genuinely excluded ids are here:

- **AC-207** — retired, never reuse. **AC-255, AC-297** — cut (register §0.4).
- **AC-213, AC-396** — **HUMAN-GATED and not machine-decidable**: the new-activity-class timeout and the per-item
  token appetite are human gate answers; AC-213's machine residue (a pack timeout with no distribution and no gate
  record) is the pack lint's, 7e. **AC-357/AC-358's human halves** — human-gated (their machine halves are unowned —
  RAISED). **No CI job may pass or fail on AC-213, AC-357, AC-358 or AC-396.**
- **AC-239** — **built and claimed at 7b-ii on its list-is-data half** (`ac239_gate_eligible_list_is_data`, `fdd9a4b`);
  its Codex half is Phase 2 (ADR-0094 decision 2). The 211-line file held it by bare range with no reason — **this is
  the reason-bearing note `s7b-ii` §7b-ii.14 said was OWED**; no second list is built here.
- **AC-356** — PROVISIONAL on S-BIN; `change_summary`'s body waits (AC-497's `classify` is built).
- **Kernel-owned rows swept in by the old bare ranges, each with its adapter share stated:** AC-222 (durable retry
  mark — AC-030's; the adapter emits `RateLimited` and never backs off), AC-230 (gate blocks on `Err` — the adapter's
  honest `spoke()` per `rk-types`), AC-231 (`Ok(empty)` vs `Err` — raw payload retained beside the typed view), AC-233
  (backoff base/cap — `retry.rs`), AC-234 (never-retried classes — `retry_class()`), AC-235 (one re-prompt — the
  adapter exposes the spawn count), AC-236 (Degraded, unclaimed by `rk-kernel` — the adapter reports the effective
  model from `system/init`), AC-237 (breaker), AC-363 (substitution recorded — the adapter records the model actually
  dispatched), AC-379, AC-389, AC-390, AC-392, AC-393 (budget rules), AC-404 (AC-030's twin; adapter share =
  classification only — its *"no `resets_at` field evidenced"* is stale against AC-523). **AC-394** — port-owned, 7a's
  signature; kept. **AC-026, AC-030** — kernel, 7b/7d.
- **AC-228, AC-388** — the fan-out scheduler, kernel-side, **claimed by no stage**, and needing a *first response
  began* signal ADR-0099's blocking dispatch does not expose — RAISED, not claimed.
- **AC-262, AC-351** — mis-routed here by `s7b-ii` §7b-ii.14 (a kernel answer-channel differential; a pack lint over
  `facets.toml`) — neither is adapter work; RAISED for re-routing, not claimed.
- **AC-189, AC-196, AC-197, AC-198** — routed to *"7c's and Step 6's"* by `s7b` §7b.9 and claimed by neither; a
  scheduler property test and `rk-git`'s merge queue — RAISED for re-routing (scheduler → kernel, merge queue →
  `rk-git`); AC-198's *no surviving child process* overlaps `rk-containment`'s API, which is cited, not the id.
- **AC-291, AC-292** — orphaned; PIA-047 routed them to Step 6, which never claimed them, and `s7b-ii` proposed an
  erratum re-assigning them here — **undecided; not claimed without the erratum**.
- **AC-254, AC-257** — `rk-git` / `rk-cli` halves of the gate's exit containment; AC-278's containment half is here.
- **AC-263, AC-282's kernel half, AC-318** — 7d's, 7b-ii's, 7b-ii's.

**CLAIMED AND BUILT HERE, NOT INDIVIDUALLY ENUMERATED ABOVE — the ledger of what is still owed, not a discharge.**
The remainder of the two claimed ranges, by id: **AC-373** (the record carries before/after **readings** and no delta
field — asserted inside `ac360_…`'s shape; the trybuild/grep home is UNCONFIRMED — kernel-owned negative),
**AC-382** (the `cache_read_share()` helper with `n` and the `n < 10` refusal — inside `ac362_…`; the share
reporter's home is UNCONFIRMED; the pack-selection rejection is 7b's). Each is reached only by the crate-wide floor.
**Before this step is RUN, either author a named test per id and add it to the block, or move the id to ARMED with
its reason.**

**RAISED, NOT SETTLED — allocation questions this file may not decide** (`00-preamble.md`; `s7a` §7a.11), each ending in
an OWED record and recorded in `PROGRESS.md` §2:

- **The routing table's open rows** (§7c.1) — **the records OWED, listed by id so the count reproduces off the
  list and not off arithmetic** (the first form of this bullet did arithmetic that did not reproduce and counted a
  correction ADR-0126's committed table already carries): (1) RT-1 the argv cap value; (2) RT-2 the decimal-digit
  parse path; (3) RT-3 as amended, the `Timeout` path if any; (4) RT-4 `Query::ByItem`; (5) RT-5 the toolchain
  timeout's source; (6) RT-6 the `VerifyReport` transport (R-9); (7) RT-7 the composite `Toolchain` (R-8); (8) RT-9
  `conn.rs`'s repair stage (R-7); (9) RT-10 the Node pack's owner; (10) RT-12 the verify-run kind; (11) RT-18
  raw-`system/init` retention, if any consumer wants it on the row; (12) RT-19 `total_cost_usd`'s unit
  (`CostUsd`, the verbatim digits, as amended); (13) RT-20 the `StepError` for a spec whose `card_digest` names no
  validated card, for one without a `"worktree"` input, and for AC-226's refusal (as amended); (14) RT-21 the
  child's worktree channel (the `"worktree"` key on `inputs`, or a sixth spec field by a successor record); (15)
  RT-22 `Toolchain::test`'s contract on a red suite; (16) D17-9's `Snapshot` contents record (RT-4's sibling; `s7a`
  §7a.5); (17) R-5 the binder's routing record (D-11); (18) **R-22** the kernel wiring of AC-026's charge from
  `api_retry_events` (**not** DECISIONS-2026-09-15's R-2, which this file first cited by mistake — R-2 is ADR-0122
  decision 5(c)'s AC-391 clarification and stays owed under that id); (19) **RT-23** the dated in-place amendment to
  AC-465 naming what its pin-manifest allowlist may hold (the extracted corpus rows; D15-2's form); (20) **RT-24**
  the capability vocabulary's home (four byte-equal copies until then); (21) **RT-25** the tracker-credential set
  on the denylist (a record, or a dated register note under AC-509); (22) **RT-26** the Rust adapter's `--list`
  count route as a dated note against ADR-0094 decision 1 (or a successor record) — **twenty-two records OWED**,
  the same list and count DECISIONS-2026-09-17's closing paragraph carries. RT-11's owed item (the gate-record form)
  is 7e's and RT-16's (the `s7a` §7a.10 note) is `s7a`'s, listed under the split notes below, not here; RT-8, RT-13,
  RT-14, RT-15 and RT-17 owe none. **The two TTL fields are not owed**: ADR-0126 decision 1's table carries them.
- **AC-360's "exactly one dispatch event"** against the kernel's pre-dispatch `dispatch` row plus the adapter's
  `dispatch_completed` (two rows on failure), and **AC-368's tier-2-append-fails-is-a-warning** against AC-360's *"no
  dispatch path exists that can terminate without appending that event"* when the `dispatch_completed` append itself
  fails (R-16; `ac368_tier2_append_failure_is_warning` is its live surface, **built and NOT claimed** — its
  `Ok`-after-failed-append assertion is labelled as R-16's reading under test, and the id is PROVISIONAL, because
  ADR-0126's forbids reads against that reading and its decision 6 leaves the reconciliation to a register
  amendment) — **a record reconciling the three is OWED**.
- **AC-227 / AC-376 / AC-360 on the pre-spawn refusals**: AC-227's *"any harness event"* against the kernel's
  version-less `dispatch` row (ADR-0126 decision 3 keeps it) and its `rate_limited` row; AC-376's *"appended as a
  `rate_limited` event"* against ADR-0126's *"a refusal before spawn"* obligation — one producer (the kernel's
  `event()` arm) or two (AC-376 is PROVISIONAL on this, not discharged); and AC-360's *"pinned harness/CLI version"*
  against AC-227/AC-385's *observed* one on a row no child reported into, answered here by `CliVersion::{Observed,
  Pinned}` (ADR-0126 decision 1; **the record is committed `proposed` until the operator confirms that shape**,
  §7c.20 item 0) — **a dated register amendment naming the producer and the version semantics on pre-spawn rows is
  OWED**.
- **AC-226's refusal variant and the two `dispatch`-time residuals** (RT-20 as amended): no record names the
  `StepError` a version-mismatch refusal returns, nor the variant for a spec without a validated card or without a
  `"worktree"` input; routed to `Transport { kind: Decode | Spawn, detail }` with their rows — **the RT-20 record is
  OWED** (counted above).
- **DECISIONS-2026-09-17 O-3 and O-5** are built as their recommended defaults (the ceiling rule for a decimal pack
  margin; `--system-prompt` beside `--agents`/`--agent`) and **remain OPEN** until the operator answers before the
  throttle / command-builder slice; a CLI refusal of the O-5 combination on the probe is a Halt.
- **AC-200's "verbatim"** against `Retained`'s 64 KiB stdout cap — **a record naming the carrier's cap policy is OWED**.
- **AC-267's fill path**: the kernel writes `gate_resumed` before dispatch — **a kernel change or a second event, by
  record**. **AC-279's adapter instrument** — named by no record.
- **AC-372's batch-delta-over-the-log owner** (adapter vs `redkiln cost`; O-7's default is taken), **AC-381's
  allocation** (subject `rk-kernel`, input struct here — built and listed DISCHARGED above under that caveat),
  **AC-382's reporter home**, **AC-373's instrument home**.
- **AC-377's `0.005 → 1 tick`** against AC-374's *no rescaling* — **a register amendment stating the margin rounding
  rule is OWED**.
- **AC-383 vs AC-384 on 18,603's profile** — the register disagrees with itself; **a dated register amendment is
  OWED**. **ADR-0016's unqualified `--system-prompt` clause vs the `default` profile route** (AC-383/AC-211/DECISIONS
  #19) — a register-level refinement of an ADR sentence; **cite as such or mint a record**.
- **AC-439/AC-428's "replay across the recorded stream corpus"** — unsatisfiable at the pin; **a register amendment
  restating the conjunct over `step.result_raw` frames is OWED**. **AC-523's `subtype` string** and **the per-window
  `status` shape** — the probe's to answer or a named open item.
- **AC-511's auth-mode field** — named from the probe's raw `system/init` in the close commit, or a named open item in
  `spikes/DECISIONS-2026-09-17.md` with `rk-adapter-claude` as consumer.
- **AC-269's catalog** at 7c (the binder's fixture cards; `.redkiln/` is 7e's) — **a note in `s7e` is OWED**.
- **The verify-gate block armed here by `s7b-ii`** (AC-320/339/341/348/349/353/355/359/338) — **a re-routing record is
  OWED**; **AC-262/AC-351, AC-189/196–198, AC-291/292, AC-228/388, AC-357/358's machine halves, AC-361/367/387** — as
  above, each **an owner record OWED**.
- **Design §9.1's `journal.rs` lift row vs its `rk-store` single-write-path row** — **a dated design note is OWED**;
  **design §10.5's R-SQLITE "Not started" row** — stale; **a dated note is OWED** (the run was Step 6's).
- **The register's AC-229 "eight"** — stale vs ADR-0112; **a dated in-place amendment is OWED** (D15-2).
- **The register's AC-224 parenthetical** — *"(the `unsafe` FFI in one module of `rk-adapter-claude` under a commented
  `#![allow(unsafe_code)]`, ADR-0102)"* — stale vs ADR-0127 (its Consequences say so in terms: *"takes a dated
  in-place amendment per D15-2, not an erratum"*); **a dated in-place amendment is OWED**.
- **AC-499's *"the same assertions"* / *"reported as a conformance divergence naming the assertion"* against D17-9's
  `Unsupported` rows** — the core/capability-arm split (§7c.13, §7c.14) expects away exactly the fake-pass/real-fail
  case the AND names, and D17-9 decides the adapters' return value, not how the AND reads; **a dated register
  amendment is OWED** stating that a capability the row's registry declares unsupported is an expected
  `Err(Unsupported { capability })`, not a divergence, while every other fake-pass/real-fail still surfaces as one.
- **The child's working directory** (RT-21; §7c.12) — nothing the adapter receives carries it and `env::current_dir`
  is banned; routed to `ActivitySpec.inputs["worktree"]` filled by the kernel's activity runtime from the store
  (AC-529's clause); **a record naming the channel is OWED** and a sixth spec field is a successor to ADR-0099
  decision 1's five, not a stage edit.
- **AC-399's carrier** — *"every such row is marked non-comparable in the store"* has no carrier on `t2_event(id,
  kind, payload, anchor_sha)` (no run key, no mark column); an `UPDATE` is what AC-364 forbids, a marking kind is
  minted by record (ADR-0116), a `0002` migration is a schema record, and a payload member enters only through
  ADR-0126 decision 6's typed-payload record — **the carrier is a record OWED**; the id is ARMED whole (above) and
  the second review's `comparable: false` flag is withdrawn.
- **AC-465's pin-manifest allowlist and the extracted corpus rows** (RT-23) — whether an extracted row copied under
  `tests/fixtures/recorded/` is an *"E3 fixture"* the THEN forbids is a reading no record makes; the copies are
  routed with **a dated in-place amendment to AC-465 OWED** naming what the allowlist may hold, and `rk-kernel`'s
  three tracked `*.jsonl` are named in it as 7b/7b-ii's debt (§7c.17).
- **The split notes at the other end** — `s7d` (AC-417, AC-422, AC-423, AC-428, AC-439, AC-488, AC-344), `s7b-ii`
  (AC-282, AC-329, AC-239, AC-278 — the notes this file carries are the 7c ends), `s6` (AC-371, AC-398) — **OWED as
  amendments to those files**, not edited here.
- **The register reconciliation is OWED and not performed here**: this file claims the two areas' ids and names its
  discharges and arms by id; the per-area binding/provisional/human-gated counts are a register-side count.

**Checkpoint 7c is NOT a milestone.** `INDEX.md`'s milestone table reads *"**M4 — the system runs** | Checkpoint 7e
passed (after 7c, 7d) | four adapters, a CLI, a migrator and a loadable pack set"*: M4 needs **7d and 7e**, and this
file attributes nothing to it. **ADR-0101 clause 5 bounds what the pass may be read as**: the adapters exist, the
real rows conform, one dispatch ran and its record carries the instrument version — and **no document may claim the
review gate is guarded against false approval**, not `PROGRESS.md`, not a commit message, not any milestone row;
AC-312's fixtures are authored and unrun, and S-SUBTLE-SYM beside it is registered and not designed.

**Spikes this step consumes:** F1, F3, F4, F6, F7, F8, F10, F14, F15, F16, F20, F21, F22, F28 and F41 are read, not run;
**R-SQLITE's empirical half was consumed at Step 6** (AC-181) and its load half is Step 8 (3) — cited, not run.
**Spikes it runs:** S-CANCEL-WIN, S-RETRY, S-TTL1 (§7c.16), each run or its deferral recorded in the close commit.
**Spikes it owes or that gate its labels:** S-SPF (registered, unrun), **S-SEL-NODE** (registered here, §7c.16 —
the affected-selection probe AC-328 needs before either row can carry one), S-H1c (AC-211), S-BIN (AC-497/AC-356),
S-SUBTLE-CB (AC-312), S-FEAT and S-SCHEMA-CAP (enabled, not owed).

**Totality.** Every id in AC-200..AC-239 (minus AC-207) and AC-360..AC-399, plus AC-523 and AC-526, plus every
cross-listed and record-routed id named in the opening paragraph (AC-278 and AC-344 now among them), appears in
**exactly one** of: DISCHARGED HERE (including its PROVISIONAL sub-list), CROSS-LISTED, ARMED, Excluded, or the
ledger — and each RAISED entry names an id already placed in one of those five. **Every id whose `doctor` / renderer
/ report / next-action / POSIX / integration / build-configuration / Rust / probe / lint / CI-matrix / gate-path
half sits in the ARMED table** is placed by its **half**, each half named once — AC-215, AC-224, AC-269, AC-334,
AC-278, AC-214 and AC-499 are examples, not the list: a scripted scan over this section on 2026-09-17 after the
third review's repairs finds **thirty-seven** ids named in more than one of the five partitions, each by a half
(AC-203's builder half here, its `doctor` half ARMED) or by a pointer sentence that says where the id is placed
(*"AC-398 and AC-371 are CROSS-LISTED (below), not discharged here"*; *"AC-399 is ARMED whole (below)"*), and **no
id named in the opening paragraph or the two ranges is absent from every partition** — and a pointer row (AC-356's
ARMED row) or a pointer sentence is not a placement. The moves this revision made, so the scan's difference from the
last one reproduces: AC-278 DISCHARGED → ARMED (by half), AC-344 and AC-371 DISCHARGED → CROSS-LISTED, AC-399
DISCHARGED (adapter half) → ARMED (whole), AC-368's warning half and AC-376 DISCHARGED → PROVISIONAL, AC-214's lint
half and AC-499's CI-matrix instrument → ARMED rows. `grep -o 'AC-[0-9]\{3\}'` over this section against the two
ranges and the opening paragraph is the re-derivation; it was re-done by script on 2026-09-17 after this revision
and is re-done at the close commit.
