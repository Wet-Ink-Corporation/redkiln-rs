# Decisions taken for Step 7c, 2026-09-17 (D17-5…D17-11), the routings the stage takes, and what stays open

Same shape as `DECISIONS-2026-09-15.md`. Every entry in the first section was taken **by the operator** (Ryan
Britton), one at a time, with the grounding and the enumerated options in front of him, **before** Step 7c's
stage-open commit and before the record that depends on it was written. Grounding is a document clause or a
measured number, never taste.

**Numbering.** These carry **`D17`-ids** and continue the space `runbook/PROGRESS.md` §2 opened the same day:
**D17-1** (Checkpoint 5's `ChangeSummary` line), **D17-2** (Checkpoint 6's PIA-036 line), **D17-3** (the verify
gate's three key spellings — OPEN) and **D17-4** (AC-321's verify-run kind — OPEN) live in the tracker, so this
file numbers from **D17-5**. Cite them as `DECISIONS-2026-09-17 D17-<n>`. Residuals carry **`R-`** ids continuing
`DECISIONS-2026-09-15.md`'s R-1…R-6, so this file numbers from **R-7**.

**Where they came from.** Step 7c's reader pass produced a map of the stage's acceptance set (every id in
AC-200…AC-239 and AC-360…AC-399 plus the cross-listed ids, each with state, owner, cite and a proposed test) and a
critic pass that amended it; together they surfaced eighteen things the stage file may not decide and
twenty-four questions for the operator. Seven were decided (D17-5…D17-11), two of them by record (ADR-0126,
ADR-0127). Twenty-six were **routed** — seventeen on the first pass, and RT-18…RT-26 added on the three review rounds of the same day — a value picked and marked as a routing, with the record that fixes it OWED.
The rest are **OPEN** here with a recommended default and the slice that needs the answer, and the operator
answers each before that slice is cut. Where a number below has no source it is written UNCONFIRMED.

**Precedence, stated once.** ADR-0092's: accepted ADRs > the AC register > the SM register > spike files > the
runbook. The register is cited by symbol (id and quoted clause), never by bare line (D15-2).

---

**The seven decisions TAKEN.**

## D17-5 · The real dispatch runs in this session, in the foreground, as a named `#[ignore]` test

**The question.** The stage's "fifteenth line" — one real `claude -p` dispatch through the built adapter — had no
executable to run it: `rk-cli` is 7d's, and nothing on the tree at `6152219` can construct a `Config` and dispatch.

**The choice: an `#[ignore]` test `live_dispatch_probe` in `crates/rk-adapter-claude/tests/it/`, behind
`--features live`, run as `cargo test -p rk-adapter-claude --features live -- --ignored live_dispatch_probe`,
in the foreground with the operator present.** It reads the process environment **once, at its own edge**
(`std::env::var` is not clippy-banned; `set_var` / `remove_var` are — `clippy.toml` `disallowed-methods`), builds
a `Config` value, dispatches a haiku probe card into a temporary `rk-adapter-sqlite` store, retains the raw
`system/init` line and the raw result frame, and prints the record path for `test -s`.

**Alternatives named.** A throwaway binary under `spikes/` (rejected: a second composition root, and `spikes/`
is frozen at the corpus pin); waiting for 7d's `rk-cli` (rejected: golden constraint 4 — *"a rig that compiles has
not been run"* — names 7c, and the stage would close with its own adapter never having dispatched).

**Grounding.** AC-499's Harness row is *"local-only, named-runner"* (`crates/rk-ports/tests/it/conformance.rs`);
ADR-0083's *"process reads env once at the edge"*; golden constraint 2 (nothing depends on a process surviving —
the foreground rule, Halt 5). **It is never wired into CI.** The probe's card must satisfy AC-205 (StructuredOutput
in `tools:`), AC-214 (explicit `--permission-mode`), AC-215 (`--permission-prompts none`, stdin closed after the
write) and AC-383 (a `cache_profile`), so the binder's fixture cards (D-11) include it.

## D17-6 · Which spikes run at 7c

**The choice.** **S-CANCEL-WIN, S-TTL1 and S-RETRY are RUN at 7c. S-SPF stays registered and unrun.** R-SQLITE's
empirical half was **consumed at Step 6** — `crates/rk-store/tests/it/contention.rs` opens *"AC-181 — two
processes … This is also R-SQLITE's empirical half"* and `PROGRESS.md` §1 row 6 records it discharged
2026-09-13 — and its load half (*whether 5000 ms suffices under the self-hosting run's load*) is Step 8 (3): 7c
**cites** it and does not run it. Design §10.5's R-SQLITE row still reads *"Not started"* and gets a dated note
(R-10 below), not a run.

**Alternatives named.** Running all five (rejected: R-SQLITE is already consumed and S-SPF's `--system-prompt-file`
route is unused by decision — DECISIONS-2026-09-11 #18); deferring S-CANCEL-WIN to Step 8 (rejected: it is the
handle-leak check on **this** stage's kill path, and Halt 6 — one fixture tree per cancel test — has to be
exercised where the kill is built).

**Grounding.** Design §10.5 rows for S-CANCEL-WIN (*"Does the adapter's cancel leave a Windows handle that
prevents removing the fixture tree"*), S-TTL1 (ADR-0113 decision 3's narrowing to the subagent bucket and the
billing mode, $0.021–0.027 per dispatch per design §10.5's row [measured, F3] — the design's figure, not re-measured here),
S-RETRY (the throttle-margin and exhaustion-on-result arms), S-SPF (*registered*); the critic's finding that the
map had mis-stated R-SQLITE as owed to 7c.

## D17-7 · Harden first

**The choice.** The stage file `runbook/phase-1/s7c-adapters.md` (211 lines at `6152219`) is **rewritten in the
`s7b-ii` shape before any code is cut** — header, gate check, numbered `§7c.N` sections, the routing table, the
per-behaviour sections quoting each claimed AC, the fixtures section, red-first bullets, manifest and checkpoint
in the hardened form, the acceptance set partitioned as `s7b-ii` §7b-ii.14 partitions it.

**Alternative named.** Cutting code against the 211-line file and hardening at close (rejected: `00-preamble.md`
says a checkpoint is hardened *before its first run*, and the reader map counted eleven substantive conflicts
between that file and the records it cites — a checkpoint whose `printf | while read … exit 1` cannot stop the
block, an aggregate-only `rk_test`, a `cargo build -p` gate where Halt 1 wants `--workspace`, a Windows arm read
as the platform hole when it is the live arm on this box).

## D17-8 · The AC-360 carrier — ADR-0126 mints `dispatch_completed`

**The question.** AC-360's THEN names *"exactly one dispatch event … carrying every token counter"*; design §5's
table has a cost row with a tier, a payload and a producer and **no wire name**; `EventKind` is closed at 37; the
kernel appends a `dispatch` row **before** the call and **drops `outcome.record`** after it
(`crates/rk-kernel/src/runtime/activity.rs`, measured at `6152219`); `rk_types::DispatchRecord` has five fields and
its module doc says the rest is *"owed at 7c"*; `Usage` exists in no crate; AC-232's `system/api_retry` count has
no channel (ADR-0122 decision 6 mints none).

**The choice: a new tier-2 kind, `dispatch_completed`, minted by record — ADR-0126 — on ADR-0120 / 0123 / 0124 /
0125's exact route; it supersedes nothing.** The adapter appends it on **every** termination path through the
`&mut dyn EventStore` it is handed. `rk_types::DispatchRecord` grows `usage: Option<Usage>`, `timings`,
`total_cost_usd: Option<..>`, `headroom_before` / `headroom_after: Option<WindowReading>` (u16 ticks × exactly two
windows — AC-374, AC-375), `cli_version: String`, `api_retry_events: u32`, and carries the cancel cause and
`detected_by` where they apply. **`Usage` and `CacheCreation` live in `rk-types`** (`crates/rk-types/src/usage.rs`)
— decided by ADR-0099 decision 1's *"Both are `rk-types`"* read through `s7a` §7a.5 — and only `Margins` stays
adapter-side. **No `Default` on any of them** (AC-362; PIA-062). `rk-store`'s duplicate `retention::DispatchRecord`
collapses onto the `rk-types` type. The kernel's pre-dispatch `dispatch` row **stands**; two rows per dispatch is
the honest count and ADR-0126 decision 3 says so. Checkpoint 5 (and 6, for the `rk-store` collapse) go provisional
and are re-run in the same commit; `PROGRESS.md` §1 rows carry the mark in that commit.

*(Amended on review 2026-09-17, before the stage-open commit — the history lives here, not in the record:
`cli_version` is a **two-armed `CliVersion { Observed(String), Pinned(String) }`** (ADR-0126 decision 1 — `Observed`
on every path that read the child's `system/init`; `Pinned`, the `Config` pin, on every path that terminated before
one was read, so a pre-spawn row cannot write the pin as a measurement — Halt 11's shape), not the `cli_version:
String` this entry first fixed, which was jointly unsatisfiable with the forbids on pre-spawn refusals (no child
exists to report one); and the two TTL values `CLAUDE_CODE_PROMPT_CACHE_TTL` / `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`
are members of decision 1's payload table (AC-210's *"both values are recorded on the dispatch event"*), which the
first draft of the record omitted. The two-armed value departs from this decision as first taken, so **the operator
confirms it before the stage-open commit** — `s7c` §7c.20 item 0 carries that confirmation as a named step; until
it is given, the enum is the stage's reading of the record and not the operator's word — **and so ADR-0126 is committed at
`status: proposed` in the stage-open commit and flipped to `accepted` by a follow-up commit once the confirmation
is written here as a dated line** (the third review: an accepted record outranks this log under ADR-0092, and a
record committed accepted would bind the operator to a shape he has not confirmed).)*

**Confirmed 2026-09-17 (dated line, the operator's word).** The two-armed `CliVersion { Observed(String), Pinned(String) }`
is confirmed as the `cli_version` member — asked with the `String` alternative named (a pre-spawn row would then carry the
pin with no marker distinguishing it from an observation) and answered *Confirm the two-armed enum*. ADR-0126 flips
`proposed → accepted` in the follow-up commit this line licenses, `date` unchanged.

**Alternatives named (the two the operator was shown).** A second row of the existing `dispatch` kind after the
call (rejected: one kind, two producers, two payload shapes, told apart by a payload string — ADR-0123's rejected
option one kind over); the usage fields on the `step_*` failure rows with success left to `activity_committed`
(rejected: the success path is where the tokens are spent and `activity_committed` never sees the stream; AC-360's
*every termination path* would be four kinds plus a kernel row). ADR-0126 §Considered options carries both.

**Grounding.** AC-360, AC-362, AC-227, AC-385, AC-232 by clause; ADR-0099 decision 1; ADR-0113 decision 1 (*"on
every `step_*` event that carries usage"*); ADR-0122 decisions 1 and 6; ADR-0116's minting rule; the measured drop
of `outcome.record`.

## D17-9 · The opaque newtypes stay opaque; real adapters return `Unsupported` from a dated registry entry

**The question.** `rk_ports::Snapshot(pub(crate) ())` and `ChangeSummary(pub(crate) ())` are unconstructible
outside `rk-ports`, so a real `EventStore::snapshot` / `rebuild` and a real `Toolchain::change_summary` can return
no `Ok`. Options: an `rk-ports` constructor (re-opens Checkpoint 7a and its six-method count), a decided contents
record, or `Err(Unsupported)`.

**The choice: no port change.** `rk-adapter-sqlite`'s `snapshot` / `rebuild` and both toolchain adapters'
`change_summary` return `Err(StepError::Unsupported { capability })` from a **dated capability-registry entry**
`{capability, verified_on, probe}` (AC-238 / AC-489 / AC-492). Checkpoint 7a's six-method count stands. The
Snapshot contents record stays OWED; `ChangeSummary` stays S-BIN's (ADR-0099 decision 5's forbids, unchanged;
the type-scoped grep of D17-1).

**Alternatives named.** A `pub` constructor on each newtype (rejected: makes `Ok` reachable for a type whose
contents no record has decided — a constructor is a decision in disguise); a contents record now (rejected: S-BIN
is unrun and no evidence exists for `ChangeSummary`'s shape; `Snapshot`'s consumer is `doctor --rebuild` at 7d).

**Grounding.** ADR-0112 decision 2 (`Unsupported` is a closed variant *"from a registry {capability, verified_on,
probe}"*); AC-238's *"no adapter-identity branch in kernel"*; design §9.1's `rk-ports — ChangeSummary` row
(**BLOCKED** on S-BIN).

## D17-10 · Toolchain containment — a sixth crate, `rk-containment`, by ADR-0127

**The question.** ADR-0099's forbids (*"any Windows child outside a Job Object created before the spawn"*),
decision 2 (the toolchain adapters spawn bounded commands *without a token*) and decision 3 (the `unsafe` FFI
*"in one module of `rk-adapter-claude`"*) cannot all hold: the toolchain adapters' `cargo` / `vitest` children are
Windows children, a Job Object is FFI, and ADR-0102 permits exactly one `allow(unsafe_code)`.

**The choice: a new crate `rk-containment` — ADR-0127 — owning the ONE commented `#![allow(unsafe_code)]`
module** (Windows Job Object: `KILL_ON_JOB_CLOSE`, created before spawn, child assigned before it can fork, handle
held for the child's lifetime, enumeration via the job; POSIX `process_group(0)` + `killpg`) **plus a token-less
`run_bounded(cmd, timeout)`** for the two toolchain adapters. All three spawning adapters depend on it;
`rk-kernel` never does; it depends on no tokio, no `rk-ports`, no adapter. ADR-0127 supersedes ADR-0099 decision 3
**in part** — the location clause only — and amends design §9.1's crate list (a dated design note is owed, R-10).
The checkpoint asserts exactly one `allow(unsafe_code)` **in `rk-containment`** and zero other `#[allow(` in the
five new crates. Deps `windows-sys = "=0.61.2"` and `libc = "=0.2.189"` (both already in `Cargo.lock`), exact-pinned
with a workspace row each (ADR-0100 decision 2); `nix` avoided.

**Alternatives named.** The toolchain crates depending on `rk-adapter-claude` (rejected: tokio and the Harness
runtime leak into every toolchain tree; the dependency direction inverts); narrowing 0099's forbids to the Harness
child (rejected: a known grandchild hole — F22's 12,000 s survivor, HANDOFF defect (3)'s killed `vitest` — traded
for a packaging sentence). ADR-0127 §Considered options carries both, plus the second-`allow` and safe-wrapper
options.

## D17-11 · Instrument moved (Halt 2): `claude --version` 2.1.269 → 2.1.275

**The measurement.** `claude --version` on this box, 2026-09-17: **2.1.275 (Claude Code)**.
`spikes/results/instrument-checks/RUNBOOK-instrument.txt` reads **2.1.269 (Claude Code)** (22 bytes, tracked; the
only file AC-465 permits under `spikes/results/`). Halt 2 fires.

**The choice.** Re-tee the file (Checkpoint 0's precedent for 2.1.268 → 2.1.269); record the move
**2.1.269 → 2.1.275** in `PROGRESS.md` §1 / §3 / §6 and in the stage-open commit; **`TESTED_CLI_VERSION =
"2.1.275"` is a `Config` value** — ADR-0078: *the pin is recorded with the measurement* — and is never read from the
process environment. The design's `2.1.263` and the register's `>= 2.1.259` floor (AC-215) are not moved by this;
the pin the adapter tests against is the one the probe (D17-5) dispatches with.

**Alternative named.** Recording the move only in `PROGRESS.md` and the dispatch record, leaving the file at
2.1.269 (rejected: the file is the instrument-of-record golden constraint 8 names, and a checkpoint line compares
`claude --version` against it — a stale file makes that line fail on a true fact).

---

## D17-12 · `jsonschema`'s workspace row goes `default-features = false` (taken 2026-09-18, before Slice 5)

**The question.** `s7c` §7c.19 recorded that `jsonschema = "=0.56.0"` on the workspace row has its default features ON
and that turning them off is *"a stage-open change the operator may take, and is not assumed here"*. Slice 4 measured
the consequence of leaving them on: as a NORMAL edge of `rk-adapter-claude`, 0.56.0's default resolvers (`resolve-http`
→ `reqwest` → `rustls`, `aws-lc-rs`) enter the shipped graph and `cargo deny check licenses` exits 4 on six licences —
ISC, MIT-0, CDLA-Permissive-2.0 and the aws-lc pair — that pass today only because cargo-deny ignores dev-only graphs
(`rk-types` carries the crate as a dev-dependency). ADR-0100 decision 4 and `deny.toml` make a new licence a decision,
not a default.

**The choice: `jsonschema = { version = "=0.56.0", default-features = false }` on the workspace row.** The adapter
validates a LOCAL output schema (ADR-0095: one output schema per dispatch, never a remote `$ref`), and `rk-types`'
validation tests compile local schemas too — no resolver feature is needed anywhere. Measured after the change: `cargo
test -p rk-types` 67/0; `cargo deny check bans advisories licenses` ok; `reqwest` gone from the graph entirely;
`cargo tree -i tokio -e normal` names `rk-adapter-claude` alone. Checkpoint 5's row goes provisional in the same
commit (the root manifest is Step 5's) and is re-run whole.

**Alternative named (rejected).** Allowing the six licences in `deny.toml` with dated rows — faster, but it ships an
HTTP client and a TLS stack inside a Harness adapter that never makes an HTTP request, and widens the licence set for
nothing the product uses.

**Also recorded here, because it was found by this re-run.** Slice 3 (`a52c0af`) and Slice 4 (`6c309e2`) each touched
a Step-5 surface without marking row 5 provisional — Slice 3's `enum Capability { …, ChangeSummary, … }` made
Checkpoint 5's `ChangeSummary` type-ban fire (it fires on the VARIANT, not the type — reading 3 below), and Slice 4
added the `tokio` workspace row. Neither re-ran Checkpoint 5; both are re-run and discharged by the commit that carries
this entry. The lesson lands in `s7c` §7c.19's rule as read: *any* edit to the root manifest is a Step-5 edit.

---

## Routed at 7c — a value picked, a routing-table row owed in `s7c`, and a **record OWED** for each

Each row below is a **routing, not a decision** (`s7a` §7a.5: *"naming a destination is not routing it there"*).
The hardened stage file carries each in its `| Type | Home | Warrant | Status |` table with status *routed, not
decided — a record naming the home is OWED*, and Checkpoint 7c asserts none of them as decided.

| # | what is routed | the value taken at 7c | why this value, by clause | record OWED |
| --- | --- | --- | --- | --- |
| RT-1 | The argv cap constant | **32,767 bytes** — Windows `CreateProcess`'s documented command-line limit; the fixture sits AT the cap | DECISIONS-2026-09-11 #18 caps the argv before spawn and fixes no number; the design says *"near 32 KB"*; the platform's documented limit is the only sourced figure | a dated register amendment naming the constant on AC-523's argv-cap instrument |
| RT-2 | AC-374's decimal digits | a **raw-bytes digit scanner over `StreamLine.raw`** into u16 hundredths; **no** `serde_json` `arbitrary_precision` | the critic measured that `serde_json` without the feature hands the visitor an `f64` (digits gone; `0.10` re-renders `0.1`), and enabling the feature unifies workspace-wide and moves `rk-types`' snapshots (Cargo.toml's one `serde_json` row) | the typed-payload record (ADR-0126 decision 6) names the parse path |
| RT-3 | The `StepError` variant for an expired per-activity timeout | default **`StepError::Cancelled { by: Timeout }`**, the cause on the `dispatch_completed` row; **`Timeout { after_secs }` with ADR-0095's retry-once class is the RAISED alternative — the operator is asked before Slice 5**. *(Amended 2026-09-17, same day, on review: this row first read "default `Timeout { after_secs }`, `Cancelled { by }` RAISED" — the reading no accepted record supports. ADR-0099 decision 2 names the per-activity timeout as one of the token's two owners and says in terms *"When the token fires, the adapter kills the child tree, and `dispatch` returns `Err(StepError::Cancelled { by })`"*; AC-225's THEN names `by: timeout`; ADR-0095's table carries BOTH variants, so "two records read differently" did not hold.)* | ADR-0099 decision 2 and AC-225 read one way; ADR-0095's table forbids neither; the routed default is the record's reading and the question left is which path, if any, produces `Timeout` | a record naming the path, if any, that yields `Timeout { after_secs }` on the Harness side — and, part of the same record, the `by` rule the third review added: `rk_ports::CancelToken` is `Arc<AtomicBool>` with no cause and no timeout arm and is one object cloned in at construction, so the adapter never fires it; `dispatch` races the child's exit against its own `tokio::time::timeout(spec.timeout_secs)` and an `interval(CANCEL_POLL_MS)` poll of `is_cancelled()`, classifying `by: Timeout` on the timer arm and `by: Signal` on the token arm, `Operator` unreachable at 7c (`s7c` §7c.10) |
| RT-4 | `Query::ByItem` semantics for the SQLite row | **match `VecStore`** — whatever `conform_event_store` asserts | AC-499: the same conformance bodies over `{fake, real}`; the kernel's item-less gate fold reads everything | the typed-payload record (`s7a` §7a.5) |
| RT-5 | The toolchain adapters' timeout source | **construction-time configuration** on each toolchain adapter | ADR-0099 decision 2's *"under the same timeout"* via a port parameter is **RAISED** — it would re-open Checkpoint 7a's six-method count; `ActivitySpec.timeout_secs` never reaches a `Toolchain` call (`crates/rk-ports/src/toolchain.rs`) | a record deciding whether the port grows a timeout parameter (ADR-0127 decision 7 names it) |
| RT-6 | `VerifyReport` transport | an **inherent method** on each toolchain adapter, `fn verify_report(&self, selection)`; port unchanged | `Toolchain::test` returns `u32`; `rk_types::verify::VerifyReport` (V1, `b213137`) has nine required members; the transport into `rk_kernel::verify::report_at_boundary` is **RAISED for 7d's composition root** | a record for the port or the event that carries the report (R-9) |
| RT-7 | `Ports` holds ONE `Arc<dyn Toolchain>` while Phase 1 has two toolchain adapters | **ARMED to 7d** (AC-350) | `crates/rk-ports/src/root.rs` has one slot; a composite or a `Ports` change re-opens 7a; the composition root is 7d's | a record for the composite (R-8) |
| RT-8 | `Default` on usage-adjacent structs | **dropped on all** lifted usage-adjacent structs — `Usage`, `CacheCreation`, the `RateLimit` / `Headroom` readings, `ApiRetry`; `Margins` has no `Default` (AC-377) | AC-362 names only `Usage` / `CacheCreation`; PIA-062 and golden constraint 10's shape (a zero satisfies every threshold) make the wider ban the safe reading; **Option over 0 everywhere** (Halt 11) | none beyond ADR-0126's forbids — recorded here as the reading taken |
| RT-9 | `rk-store` `conn.rs`'s `unwrap_or_else(PoisonError::into_inner)` — an ADR-0118 violation in a passed checkpoint | **NOT repaired at 7c**; `rk-adapter-sqlite` adds no recovery of its own; a `PoisonError \| into_inner() \| catch_unwind` absence ban over `crates/rk-adapter-*/src` with a positive control | ADR-0118 says of `conn.rs` *"its repair is owed"* and *"does not schedule it into a stage it does not own"*; 7c does not own `rk-store` | the repair, scheduled by a `PROGRESS.md` §2 row (R-7) |
| RT-10 | The Node technology pack's owner | **RAISED** — 7e is the pack-authoring stage. The **Rust** technology pack is 7c's (ADR-0094 decision 1): a fixture under `crates/rk-adapter-toolchain-rust/tests/fixtures/pack/` until 7e installs it under `.redkiln/` | ADR-0094 decision 1 names the Rust pack as 7c's deliverable and no record names the Node pack's stage; no technology pack of any kind exists on the tree (the twelve packs under `spikes/p9/packs` are process packs) | a dated `s7e` amendment claiming the Node pack, or a record |
| RT-11 | AC-213's `test_timeout` for the technology pack | the Rust pack fixture carries it as the **human input it is**, with a gate-record placeholder named as such; **no CI job passes or fails on it** | AC-213 is HUMAN-GATED (*"set via human gate; no global default"*); ADR-0029 (iv) makes `${toolchain.test_timeout}` the tech pack's to supply | the gate-record form for a pack timeout (7e / the pack schema record) |
| RT-12 | The verify-run `EventKind` (AC-321, D17-4) | **ARMED** — 7c's toolchain adapters write no verify-run event | ADR-0116: a kind is minted by record; D17-4 is open and 7c does not close it silently | the record D17-4 already owes |
| RT-13 | CI's `Command` grep (`.github/workflows/ci.yml`, the *process::Command* structural rule) | **narrowed to its stated warrant** `Command::new("git")` in the stage-open commit; clippy `disallowed-types` + per-site `#[expect(clippy::disallowed_types, reason = …)]` is the real instrument. **every spawn, the Harness child included, is a `std::process::Command` inside `rk-containment`** (on Windows spawned `CREATE_SUSPENDED`, assigned to the job, resumed by `OpenThread` + `ResumeThread` over a Toolhelp32 thread enumeration by pid — the primary-thread handle `std` closes after `CreateProcessW` is re-opened, not kept); `rk-containment` returns a `Contained { child: std::process::Child, group }` handle; `rk-adapter-claude` takes the three pipes with `child.stdin/stdout/stderr.take()` and wraps **each pipe** with `tokio::process::ChildStdin::from_std` / `ChildStdout::from_std` / `ChildStderr::from_std` (tokio's `process` feature) for its async drain and write-then-shutdown, awaits exit with `tokio::task::spawn_blocking(move || child.wait())` on its `multi_thread` runtime, and kills through the group handle so `wait()` returns — so `tokio::process::Command` and `tokio::process::Child` appear nowhere and tokio never enters `rk-containment` (ADR-0127 decision 2 and forbids). *(Amended 2026-09-17 on review: this cell first read "`tokio::process::Command` is `rk-adapter-claude`'s only", which cannot coexist with ADR-0127's forbids that every spawn goes through `rk-containment` unless the crate spawns a `std` child and the adapter wraps it — the shape now stated; O-1's "`tokio::process::Command` exposes no suspended spawn" is moot for the same reason, and `creation_flags(CREATE_SUSPENDED)` exists on both `Command`s regardless. Amended a second time the same day: the first amendment said the adapter wraps the child with `tokio::process::Child::from_std`, **which does not exist** — in tokio 1.53.1, the lock's version, `from_std` exists only on `ChildStdin` / `ChildStdout` / `ChildStderr` (`src/process/mod.rs`, measured 2026-09-17), and the one API taking a caller-spawned `std` child, `Command::spawn_with`, is `#[cfg(tokio_unstable)]` and is not taken.)* | ADR-0077's forbids is *git* spawns outside the one wrapper; the grep as written fires on any `Command` token and is evadable by `use std::process::{Command, Stdio}` (the critic's finding) | none — a CI rule narrowed to the clause it cites; recorded in the stage-open commit |
| RT-14 | `tests/fixtures/rk-adapter-sqlite` (a workspace member with the real crate's name) | renamed **`tests/fixtures/rk-adapter-edge-fixture`** (keeps the `rk-adapter-` prefix AC-484's rule keys on); `crates/rk-kernel/Cargo.toml` and `ac484_adapter_edge_fixture` follow in the same commit | Cargo refuses duplicate member names; the fixture's own header says rename/retire at 7c; AC-484's third instrument keys on the prefix | none — a fixture rename recorded in the stage-open commit |
| RT-15 | tokio | workspace row `tokio = "1"` (caret; features `rt-multi-thread, process, io-util, time, sync, macros`); a dependency of **`rk-adapter-claude` only**; `cargo tree -p rk-kernel -e normal` must stay free of it (AC-528; S-FEAT is the check) | ADR-0076 / ADR-0099: adapter-owned runtime; the runner's feature list at the corpus pin; tokio 1.53.1 is already in the lock (dev-only via `jsonschema → reqwest`) | none — the `Cargo.toml` row and its comment |
| RT-16 | Where the real conformance rows run | `rk-ports` gains a **`conformance` module behind `[features] conformance = []`** (`#[cfg(feature = "conformance")] pub mod conformance;`), exposing the per-port bodies (with their `spec()`/`findings()` helpers) so the real rows run the **same** bodies (AC-499); each body is a **required core plus a capability arm** parameterised by the row's registry, so a row that declares `snapshot`/`rebuild`/`change_summary` unsupported (D17-9) asserts `Err(Unsupported { capability })` where the unchanged body would `.expect()` and panic; Checkpoint 7a re-run in the same commit; `PHASE_1_REAL_ROWS` filled per slice. *(Amended 2026-09-17 on review: the unchanged bodies `.expect()` exactly the methods D17-9 makes `Unsupported`. Amended again the same day: the **Harness body is parameterised by a spec, not a registry** — `conform_harness(harness, store, spec: &ActivitySpec)` — because the fake row's fixed `spec()` names no `card_digest` the real adapter's validated card store holds; the fake row passes `spec()`, the live probe passes its bound probe-card spec and calls the body before its own prints, so the AC-499 Harness row runs the SAME body and not a body of its own. **Re-amended on the third review, same day:** the second review had re-routed this row to an *unconditional* module on the ground that "a package cannot enable its own feature for its own tests without a self dev-dependency, so `cargo test -p rk-ports` would have compiled nothing" — **that ground was false** (`cargo test -p rk-ports --features conformance` enables it, exactly as `crates/rk-ports/Cargo.toml`'s `corpus = []` comment already runs the corpus feature), a review may not re-route an operator's routing on a wrong premise, and the sentence is struck; the operator's `feature = "conformance"` is restored, each adapter names the feature on a **dev** `rk-ports` edge, and the one real consequence is named in `s7c` §7c.1 (resolver-2 unification puts the bodies into every package's *test* build; `rk-kernel`'s normal graph never enables it — asserted by `cargo tree -p rk-kernel -e features`). `conform_harness` prints `PROBE_RAN_CONFORMANCE=1` itself as its last statement, so the probe's line is evidence the body ran.)* | AC-499: *"one parameterised suite per port"* over `{fake, real}`; the bodies are private today (`crates/rk-ports/tests/it/conformance.rs`) and `fake_conformance` — the 211-line stage's name — exists nowhere | a note in `s7a` §7a.10 (Checkpoint 7a) recording that its `port_conformance_fake_rows` assertion now runs the same bodies over real rows |
| RT-17 | `rk-adapter-sqlite` → `rk-git` edge | added, for `git rev-parse --git-common-dir` through `GitExec` (ADR-0077's one wrapper) | AC-370's store path is `<git rev-parse --git-common-dir>/redkiln/`; nothing on the tree resolves a linked worktree to its common dir (`rk-store` takes `common_dir: &Path`; the critic's finding); the three-worktree test is genuinely 7c's | none beyond the crate plan — the edge is ADR-0077's rule applied |
| RT-18 *(added on review 2026-09-17)* | The raw `system/init` the probe retains | **the probe's own output file only** (`RECORD_PATH`, `s7c` §7c.15) — not a `DispatchRecord` member | no record names an `init_raw` member (0 hits over `references/` and `spikes/*.md` [measured 2026-09-17]); ADR-0126 decision 1 fixes the payload's names | a record naming raw-init retention (cap, digest, tier) if any consumer wants it on the row |
| RT-19 *(added on review 2026-09-17; amended on the third review)* | `total_cost_usd`'s type on the row | **`Option<CostUsd>`** — `rk_types::usage::CostUsd(String)`, the decimal digits verbatim as the CLI printed them, taken from the raw result bytes by the same scanner AC-374 uses for ticks, never an `f64` and never a fixed scale *(this row first said `CostMicros`, `u64` micro-dollars — withdrawn: the recorded F20 value `2.337147` is the runner's `f64` re-serialisation, so the CLI's digit count is UNCONFIRMED at the pin and a six-decimal scale would truncate or round a longer value silently)* | `rk_types::DispatchRecord` derives `Eq` and an `Option<f64>` does not compile under it; `rk-store`'s canonical serialiser refuses floats (`canon.rs`) and accepts strings | the typed-payload record (ADR-0126 decision 6) names the cost unit and any arithmetic form (AC-361's rollup, 7d, is the consumer that needs a number) |
| RT-20 *(added on review 2026-09-17; amended on the third review)* | The `StepError` `dispatch` returns for a spec whose `card_digest` names no card in the validated store — **and its two siblings**: a spec whose `inputs` lacks a string `"worktree"` (RT-21), and AC-226's configured *refusal* of a version mismatch read from the child's own `system/init` | **`Transport { kind: Spawn, detail }`** before spawn, with its `dispatch_completed` row, for the first two; **`Transport { kind: Decode, detail }`** after the child is killed through the group, with its row carrying `Observed(v)`, for the third | ADR-0112 closes the set at eleven and `ConfigError` never crosses the port (`s7c` §7c.4); each residual needs some closed variant and no record names one | a record naming the variants for the three residuals |
| RT-21 *(added on review 2026-09-17)* | **The child's working directory** — nothing the adapter receives carries it: `ActivitySpec` has ADR-0099:61's five fields and no sixth, `Config` is pins, TTLs, the captured map and `harness_binary`, and `env::current_dir` is banned in every `crates/*/src` (`ci.yml`'s AC-529 rule) | **`ActivitySpec.inputs` carries a fixed key `"worktree"`** whose value is the run-owned worktree path the kernel's activity runtime fills from the store (AC-529's clause: *"the run-owned worktree is resolved from the store, never from cwd"*); the binder puts the key on every spec it materialises (a card it cannot fill is a bind-time `ConfigError`); the adapter reads it into a `WorktreePath` at the top of `dispatch` and, for a spec driven in without it, returns `Err(Transport { kind: Spawn, detail })` with its row — never a `ConfigError`, which `dispatch` cannot return *(amended on the third review: this row first had `dispatch` refusing as `ConfigError`, which ADR-0099 decision 1's `DispatchOutcome` cannot represent; and the "`GitExec` snapshot's `cwd`" clause is withdrawn — the pre-spawn snapshot is not lifted, `s7c` §7c.2)* | `inputs: Vec<(String, serde_json::Value)>` is the only free-form channel on the spec; a sixth field re-opens Checkpoint 7a's record-fixed shape; a per-worktree adapter contradicts design §9.1's *"constructed once at the composition root"* | a record naming the worktree channel (the key on `inputs`, or a sixth spec field by a successor to ADR-0099 decision 1's five) |
| RT-22 *(added on review 2026-09-17)* | `Toolchain::test`'s contract on a red suite | **`Err(StepError::NonZero { .. })`** when the suite fails; `Ok(count_ran)` only on a green suite | `crates/rk-ports/src/toolchain.rs`'s `fn test(&self, selection: &[String]) -> Result<u32, StepError>` is silent on failure; AC-331's *"attributable to the candidate"* needs a failing test to be distinguishable from a killed run (`Unverifiable`), which an `Ok(count)` over a red suite erases | a record on the port's contract (the `s7a` §7a.10 note's sibling) |
| RT-23 *(added on the third review 2026-09-17)* | Extracted corpus rows copied into the tree as fixtures (the seven `is_error` frames, the `api_retries` line, e1-smoke's rows, the h1 dispatch rows — `s7c` §7c.17) against AC-465's *"no corpus artifact (… E3 fixtures, spike JSONL) is tracked"* | **copied, with bytes + sha256 per file, and named in `ci.yml`'s AC-465 rule as an allowlist**; `rk-kernel`'s three already-tracked `*.jsonl` are named in the same allowlist as 7b/7b-ii's debt, never folded in silently | whether an extracted row is an "E3 fixture" is a reading no record makes; the alternative (every recorded-frame test behind `--features corpus`) would take seven claude tests out of the un-featured checkpoint loop | a dated in-place amendment to AC-465 naming what its pin-manifest allowlist may hold (D15-2's form) |
| RT-24 *(added on the third review 2026-09-17)* | The capability vocabulary's and the registry row type's home (AC-489/AC-492 across four adapters; the kernel says *"No capability registry exists in this workspace"*) | **one shape, declared in each crate**: four `registry.rs` files each with `struct Row { capability: Capability, verified_on: String, probe: Probe }`, `serde::Deserialize` + `deny_unknown_fields`; the four `enum Capability` bodies **byte-equal**, asserted by digest in Checkpoint 7c; 7d bridges to `CapabilityClaim::new(.., probe.as_str())` | a shared type in `rk-types`/`rk-ports` re-opens Checkpoint 5/7a for a criterion no record homes | a record naming the vocabulary's home |
| RT-25 *(added on the third review 2026-09-17)* | The tracker-credential names on the committed denylist (AC-509 names no variable) | **`GITHUB_TOKEN`, `GH_TOKEN`, `GH_ENTERPRISE_TOKEN`** — the GitHub tracker's ambient credentials (the design's tracker comparison names only GitHub's `GITHUB_TOKEN`); Checkpoint 7c's shell guard refuses a shell carrying any | a committed denylist is source and the probe's outcome on the box is decided by it, so the set cannot be an implementer's guess | a record, or a dated register note under AC-509, naming the tracker credential set |
| RT-26 *(added on the third review 2026-09-17)* | The Rust adapter's test-count source on the pinned STABLE toolchain — `libtest --format json` is nightly-only on 1.98.1 (`error: The "json" format is only accepted on the nightly compiler with -Z unstable-options` [measured 2026-09-17]), `--format junit` the same, nextest absent, `RUSTC_BOOTSTRAP=1` a Halt 7 workaround | **libtest's stable `--list`**: `selected_test_count` = the `: test$` lines of `cargo test <selection> -- --list` minus `-- --list --ignored`; `total_test_count` from the unfiltered `--list`; pass/fail from the run's exit status (RT-22); `VerifyReport.argv` is the run vector and the count vectors are that vector plus the `--list` forms | ADR-0094 decision 1 fixes `test = cargo test --workspace` and says nothing about how counts are read; the prose summary line is what AC-329 bans; the `--list` route counts what would run, and a per-test result on stable is nextest's (ARMED) | a dated note against ADR-0094 decision 1, or a successor record |

---

## OPEN — the operator answers each before the slice that needs it

Each carries a **recommended default**, grounded, and the **slice** that cannot be cut without the answer. None
is decided by this file.

**Taken 2026-09-17 (dated line, the operator's word): all eleven defaults, O-1 through O-11, as recommended** — asked
as one batch with each default restated in one clause and the alternative *take all except ones I'll name* offered,
and answered *Take all 11 defaults*. Each row below therefore reads as TAKEN on its default; a later reversal is a
dated line under the row, never an edit to it.

| # | the question | recommended default | why | needed before |
| --- | --- | --- | --- | --- |
| O-1 *(amended in place 2026-09-17 on review — the tokio premise is retired by RT-13's amendment: `tokio::process::Command` is nobody's spawner)* | **Does spawn-suspended → assign → resume hold through `std`** — a `std::process::Command` spawned with `creation_flags(CREATE_SUSPENDED)` inside `rk-containment`, assigned by `AssignProcessToJobObject`, then resumed via `OpenThread(THREAD_SUSPEND_RESUME)` + `ResumeThread` over a Toolhelp32 thread enumeration by pid (`std` closes the primary-thread handle after `CreateProcessW`, so it is re-opened, not kept) — measured by the orphan and breakaway fixtures, and by the parent-side reading that `ResumeThread` returned a previous suspend count of exactly 1 | **RT-13's shape**: spawn suspended inside `rk-containment` and resume after assignment, so ADR-0099 decision 3's *"assigns the child to it before the child can fork"* is literal rather than probabilistic; the Harness adapter wraps the three pipes, never the child. **UNCONFIRMED** until the slice measures it that the re-opened primary thread resumes cleanly and that `IsProcessInJob` is already true when the child's first instruction runs | a race the record forbids is not a design; the orphan fixture (positive control), the breakaway fixture (negative control) and the suspend-count reading are the instruments that decide whether the default holds (`s7c` §7c.10, §7c.18) | the `rk-containment` slice |
| O-2 | **The detached-grandchild fixture's mechanism**: a `CREATE_BREAKAWAY_FROM_JOB` attempt, or an orphaned grandchild (parent exits, child lives) that a PID walk misses and the job holds | **Both, as two controls**: the orphaned grandchild is the positive control (the job must reap it — F22's shape); the breakaway attempt is the negative control that must **fail** because the job never grants `JOB_OBJECT_LIMIT_BREAKAWAY_OK` (ADR-0127 decision 7 fixes that the job does not set it). Whether the failing breakaway surfaces as a spawn error or a child inside the job is **UNCONFIRMED until the fixture runs** | AC-224 names *"a grandchild AND a detached/breakaway grandchild"*; a breakaway that succeeded would be the containment hole, so the fixture must show it cannot | the `rk-containment` slice |
| O-3 | **AC-377's `0.005 → 1 tick`** under AC-374's *no rescaling*: the register's vector asserts 1 tick for a margin with a thousandths digit and no rule says how | **Pack margins are declared as integer `margin_ticks`** (the throttle rewrite already makes `Margins` integer ticks per window); a decimal margin in a pack is parsed to hundredths by **ceiling** (`0.005 → 1`, `0.01 → 1`, `0.02 → 2`), never by rounding to nearest — a larger margin refuses earlier, which is the safe direction (Halt 12). **Utilization readings are never rounded** — AC-374's ban is on the reading, not the margin | AC-374 forbids rescaling the *utilization*; AC-377 is about the pack's margin; ceiling is the only rule under which every register vector and *"margin_ticks < 2 refused"* both hold | the throttle slice |
| O-4 | **AC-226's configured mismatch outcome for the probe dispatch** (refuse, or an opted-in `Degraded`) now that `TESTED_CLI_VERSION` is `2.1.275` (D17-11) | The probe's `Config` pins `2.1.275`, the measured binary, so **no mismatch arises on the probe**; the adapter's default when one does is **refuse**, and `Degraded` is opt-in per `Config` — both configurations are tested (AC-226's *Verified by* asks for four assertions) | AC-226's THEN: *"the CONFIGURED outcome (refusal or opted-in Degraded)"*; a default of `Degraded` would let a version drift pass silently, which AC-385's warning exists to prevent | the live probe (the last slice) |
| O-5 | **Whether the `static` profile's `--system-prompt` coexists with `--agents` / `--agent`** (the adoption mechanism AC-202 / AC-204 rely on) — the corpus at the pin measured `--system-prompt` without `--agents` (`results/h1-sysprompt.jsonl`, a 45-byte prompt) and `--agents` without `--system-prompt` (the runner's `build_args`); the combination is unmeasured (PIA-066) | **Both are passed**: the card body as `--system-prompt` (AC-211's static route) **and** `--agents` / `--agent` (adoption); `golden_argv_per_profile` pins that argv; the probe (D17-5) is the first measurement of the combination and AC-211 stays PROVISIONAL on S-H1c regardless | AC-202 / AC-204 need `--agent` for the child's `system/init` to report an adopted tool set; AC-211 needs `--system-prompt` for `static`; nothing forbids both and the probe measures whether the CLI accepts them together | the command-builder slice |
| O-6 | **Which catalog AC-269's card-validator iterates at 7c** (`.redkiln/` does not exist until 7d / 7e) | **The fixture card set 7c ships** — the binder's cards (D-11), the probe card included — stated as *the only cards on the tree*, with the validator re-run over `.redkiln/` at 7e when the packs are installed | AC-269: *"No card exposes answer/mint/resolve"* is a property of every card that exists; at 7c those are the fixture cards | the card / binder slice |
| O-7 | **Where ADR-0110 decision 1's batch-delta logic lives** — the adapter or `redkiln cost` at 7d; 7c does the tick arithmetic, but `low_sample` / not-attributable / the reset-split need a named dispatch set and `n` | **The calculator is `rk-adapter-claude`'s** — pure functions over readings: `delta_ticks`, `resolution_limited`, `low_sample` at `n < 10`, the per-`resetsAt` split, not-attributable on interleaving — and the **reader over the log** that assembles the dispatch set and prints flags is 7d's (`redkiln cost`, the `doctor` rule) | the map's AC-372 row already splits it this way (calculator half DISCHARGE-HERE); the register's vectors are pure arithmetic and testable without a store | the throttle slice |
| O-8 | **Does 7c author `mutation:` declarations** (ADR-0104 / AC-334) for its rows, and where — none of the in-scope register rows carries one | **No.** AC-334's real-toolchain flip is 7c's only mutation-facing obligation (over the F25 ordering-inversion fixture); the declaration's spelling is D17-3's, open, and the spec/pack type that carries it is the migration stage's | writing declarations into a schema whose key spellings are OPEN (D17-3) would decide them by default | the close-out |
| O-9 | **Where S-TTL1's raw usage and `rate_limit_info` are retained**, given `AC-465` (only `RUNBOOK-instrument.txt` tracked under `spikes/results/`) and the map's note that the corpus pin may not move twice (the sentence is `PROGRESS.md` row L2's — cited by symbol; the map's `:103` was one line off, the row is at :104 on this tree — *"the pin should move once, with 7b's work, not twice"*, quoted by `s7b` §7b.2 (*"`PROGRESS.md`:103 (row L2) fixes it: the pin should move 'once, with 7b's work, not twice'"*); it scopes 7b's single move and states no rule about later moves, and `s7b` §7b.11's cost argument — *"**cannot** move before Slice 3 is green without forcing a second move, and AC-464 would then freeze the first SHA into every test that read it"* — is the ground for the default below, a cost, not a rule *(corrected on the third review 2026-09-17: this cell first said nothing in `PROGRESS.md` states the sentence and attributed it to §7b.11 — both wrong against the tree)*) | Retain the raw frames **in the corpus repository as a new commit** (`D:/repos/redkiln-corpus`), named by SHA in the spike's result note, **without moving `CORPUS_PIN`** — the pin governs lifts and fixture reads, not where a spike's raw output lives; the 7c fixtures that need bytes copy them with bytes + sha256 recorded | AC-465 forbids the tracked path; a pin move is a `s7b` §7b.11-shaped act with its own re-runs (D17-2 shows what a move costs) | the S-TTL1 run |
| O-10 | **S-CANCEL-WIN's victim**: the Harness Job Object path (design §10.5) or a mid-`vitest` Node child (design §10.4) | **One fixture serves both readings now that the kill is shared**: a mid-`vitest` victim under `rk-containment`'s job, killed by `run_bounded`'s timeout path (the Node toolchain's), with Halt 6's own fixture tree; the Harness path is the same primitive with a token in front of it | ADR-0127 makes the two paths one mechanism; the handle-leak question is about the *victim* (`vitest`), not the caller | the `rk-containment` slice (run after the Node adapter exists) |
| O-11 | **`DispatchOutcome`'s home** (`rk-ports` today, per 7a; `s7a` §7a.5 lists it among the routed-not-decided) | **Stays in `rk-ports` where 7a declared it** beside the `Harness` trait, since it is the trait's return type and ADR-0099 decision 1 declares both in one block; the record owed is the §7a.5 routing record (R-5's sibling), not a 7c move | a type that appears in a port trait's signature lives with the port; moving it re-opens 7a for no criterion | none at 7c — record owed |

---

## Residuals raised at 7c and not settled here (R-7…)

| # | what is owed | raised by |
| --- | --- | --- |
| R-7 | **`rk-store` `conn.rs`'s `unwrap_or_else(std::sync::PoisonError::into_inner)`** — a live ADR-0118 violation in a passed checkpoint (Checkpoint 6). NOT repaired at 7c (RT-9); `rk-adapter-sqlite` inherits it through `rk-store`'s single writer. Needs a scheduled repair with Checkpoint 6 re-run, by whichever stage next owns `rk-store` — a `PROGRESS.md` §2 row — ADR-0118 itself says *"carrying it as a tracked debt in `PROGRESS.md` is owed and is not done here"* | the map's binding clauses; ADR-0118 |
| R-8 | **`Ports` holds one `Arc<dyn Toolchain>`** and Phase 1 has two toolchain adapters (ADR-0094); AC-350's *"one verify result per claiming toolchain"* has no slot. ARMED to 7d (RT-7); a composite adapter or a `Ports` change (re-opens 7a) needs a record | the map's conflicts; `crates/rk-ports/src/root.rs` |
| R-9 | **`VerifyReport` transport** from a `Toolchain` adapter into `rk_kernel::verify::report_at_boundary` — `test()` returns `u32`; 7c ships an inherent method (RT-6); the port growing a method (Checkpoint 7a counts six) or the adapter writing it as an event (a kind — ADR-0116) is a record for 7d's composition root | AC-329's adapter half; `s7b-ii` §7b-ii.14 |
| R-10 | **Two dated design notes**: §9.1's crate table gains `rk-containment` and its `-claude` row stops claiming the containment module (ADR-0127 decision 6); §9.1's `Origin` column lists `journal.rs` among `-claude`'s lifts while §9.1's `rk-store` row makes the store the single write path (the critic: intra-design, not stage-vs-record). And **§10.5's R-SQLITE row** reads *"Not started"* while its empirical half was consumed at Step 6 (D17-6) — a dated note, not a run | ADR-0127; D17-6; the critic's gaps |
| R-11 | **The `s7d` split notes** — ids in `s7d`'s declared ranges whose adapter half 7c discharges and which need a matching note at the other end: AC-428 (via `ac218_`), AC-439 (via `ac206_`; its *"replay across the recorded stream corpus"* conjunct is unsatisfiable at the pin — no stream corpus exists), AC-417 (card-validator half), AC-404 (kernel cross-ref; the register's *"no resets_at field evidenced"* is stale against AC-523), AC-488 (construction refusal at 7c, edge env read at 7d), AC-422 / AC-423 (adapter halves at 7c, `doctor` rows at 7d) | the critic's acceptance_set_delta |
| R-12 | **Mis-routed and orphaned ids, not claimed at 7c**: AC-189 / AC-196 / AC-197 / AC-198 (routed by `s7b`:849 to *"7c's and Step 6's"*, claimed by neither — scheduler ids re-route to `rk-kernel`, merge-queue ids to `rk-git`; AC-198's *"no surviving child process"* overlaps AC-224's containment API and is cited as an overlap, not claimed); AC-262 / AC-351 (mis-routed by `s7b-ii`:877 — neither is adapter work); **AC-291 / AC-292** (orphaned; PIA-047's remediation routed them to Step 6, which never claimed them; `s7b-ii`'s proposed erratum targets `s6`'s set, not 7c). Each needs a dated routing amendment, not stage prose | the map; the critic |
| R-13 | **ADR-0016's default-profile refinement question.** ADR-0016's delivery clause is one unqualified sentence (`--system-prompt` = the card body); AC-383 / AC-211 / DECISIONS-2026-09-11 #19 add a `default` profile route (omit `--system-prompt`, prepend the card body to the user message — F28's measured arm, the `h1-gap` argv). Whether that is a register-level refinement of an ADR (defensible under ADR-0092) or a record owed is **flagged, not resolved**; 7c builds both profiles as the register words them. If a record is wanted it takes its own number | the map's conflicts; DECISIONS-2026-09-11 #19's own *"neither ADR states the default-profile argv"* |
| R-14 | **AC-357's machine half** — the gate node carrying three payload fields for a mutation-infeasible AC — is claimed by no stage (ADR-0125 decision 7 names the human-answer carrier as the verify slice's and not a kind; the payload's owner is still unnamed) | the map; `s7b-ii` §7b-ii.14 |
| R-15 | **AC-367, AC-361, AC-387 are claimed by no stage** (0 hits in `runbook/` and `crates/` for AC-367 and AC-387 by id; AC-361's rollup has no owner row). AC-367's *tier tag on every kind* is built by shape (`EventKind::tier()` is exhaustive) and never claimed by id. Listed in 7c's ARMED table with that reason; a stage must claim each by id or re-route it | the critic's gap; the map |
| R-16 | **AC-368 against AC-360 on the `dispatch_completed` append itself**: a tier-2 append failure *"is surfaced as a warning with a counter"* and never fails the activity; *"no dispatch path exists that can terminate without appending that event"*. When the append of the usage row fails both cannot hold as worded. ADR-0126 decision 6 names it and does not resolve it; a dated register amendment is owed | ADR-0126; the map's conflicts |
| R-17 | **The `gate_resumed` fill path** (ADR-0123 decision 2): the kernel writes `gate_resumed` *before* the resumed dispatch with `{prefix_rewrites, token}`; the adapter's two metric fields exist on `dispatch_completed` after it. A kernel change or a reader joining the two rows — not decided by ADR-0126 (decision 6) | ADR-0126; `crates/rk-kernel/src/workflow/gate.rs` |
| R-18 | **AC-511's second conjunct** (auth mode asserted from the child's own `system/init`): no auth-mode field is named by any record and `apiKeySource` appears in zero corpus files at the pin. The probe (D17-5) retains raw `system/init`; if no such field exists at 2.1.275 the conjunct is **dropped with this row as the named open item** — the field name, if any, is written into the probe's result note. Likewise AC-523's `detected_by: subtype` string (only an authored `terminal_reason: usage_limit_reached` exists at the pin) carries `[surface unverified]` until the probe or S-RETRY's exhaustion arm observes one | the map; the critic's unresolved gap |
| R-19 | **The typed per-kind payload record** (`s7a` §7a.5) — `dispatch_completed` is the first kind with a fully typed producer-side payload; the record giving every kind its type is still owed and `rk-adapter-sqlite` is its first real consumer | ADR-0126 decision 6 |
| R-20 | **`runbook/00-preamble.md`'s next-free-number sentence** reads `0126` and is stale by two after this pass (the ADR README is corrected to `0128` in the same change); the preamble takes its dated correction in the stage-open commit, as ADR-0125's commit did | this file |
| R-21 | **External anchors into `s7c` are by line — sixteen** [measured 2026-09-17 by `grep -n -oE 's7c\`?:[0-9]+'` over `runbook/phase-1/*.md` and `runbook/PROGRESS.md`]: `s7a`:1074, :1142, :1187, :1208, :1221, :1224, :1236; `s7b-ii`:329, :844, :856, :861, :901, :1004, :1010; `s7b`:1977; `PROGRESS.md`:354 *(this row first listed eight)* — and they drift when the 211-line file is rewritten (D17-7); `PROGRESS.md` §2 records that they read against the pre-hardening pin, and each is re-anchored to a `§7c.N` heading by symbol (D15-2's rule) | the map's hardening conventions |
| R-22 *(added on review 2026-09-17)* | **The kernel wiring of AC-026's inner-attempt charge** — a record routing `system/api_retry` events from the adapter stream (`DispatchRecord.api_retry_events`, ADR-0126) into the kernel's charge call site; `crates/rk-kernel/src/workflow/retry.rs` names it in terms (*"A record routing inner attempts from the adapter stream to the kernel is **OWED** and is named here, not written here"*). It is **not** DECISIONS-2026-09-15's R-2 (that is ADR-0122 decision 5(c)'s clarification of AC-391's class-scoped clause), which `s7c` first cited for it by mistake | `retry.rs`; ADR-0126; the review of 2026-09-17 |

---

## What these decisions unblock

D17-5 gives the stage its fifteenth line. D17-6 fixes the spike list the close-out reports against. D17-7 is the
order of work. D17-8 (ADR-0126) unblocks AC-360, AC-362, AC-232, AC-227, AC-385 and the `rk-types` edit that
Checkpoint 5's re-run attests. D17-9 unblocks `rk-adapter-sqlite`'s `impl EventStore` and both toolchain adapters'
`impl Toolchain` without a port change. D17-10 (ADR-0127) unblocks every spawn in the stage. D17-11 unblocks the
stage-open commit's instrument line.

**Still owed after this pass, and named so nobody rediscovers them** *(the list as corrected on review 2026-09-17;
the first form of this paragraph did arithmetic that did not reproduce)*: the routing records this table owes, by
row — **RT-1, RT-2, RT-3, RT-4, RT-5, RT-6, RT-7, RT-9, RT-10, RT-12, RT-18, RT-19, RT-20, RT-21, RT-22, RT-23, RT-24, RT-25, RT-26** (nineteen;
RT-8, RT-13, RT-14, RT-15 and RT-17 owe none; RT-11's owed item is 7e's gate-record form and RT-16's is the `s7a`
§7a.10 note, and `s7c` §7c.22 lists those two under the other stage's owes rather than here); plus **D17-9's
`Snapshot` contents record**, **R-5** (the binder's routing record, DECISIONS-2026-09-14 D-11) and **R-22** (the
AC-026 wiring) — **twenty-two records OWED** *(eighteen before the third review added RT-23…RT-26)*, the list `s7c` §7c.22 carries by the same ids and the same count. The
TTL fields on ADR-0126's table are **not** owed: the record's table carries them. Then the eleven open questions
(O-1…O-11, each answered before its slice), and R-7…R-22. Nothing in this file claims a criterion discharged; that
is the checkpoint's to say, and only after it runs.
