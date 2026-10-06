# W1 audit: crates/rk-ports (the five named ports, their fakes, conformance, parity)

Snapshot: worktree `audit-wt`, commit `36dd8ac` (branch build/phase-0-to-1). Read-only. No cargo run by me; I read the
separate audit build's logs at `scratchpad/audit/log-*.txt` (fmt rc=0, build rc=0, clippy rc=0, conformance rc=0,
workspace test rc=101 for reasons outside this crate, see section 3).

## Verdict

**Grade B (solid with minor gaps). Completeness against what Phase 1 requires of this area: about 95%.**

The ports crate does what Phase 1 asks of it. Five sync `dyn`-safe traits, five public fakes, one shared conformance
body per port, a number-asserted fake-per-port parity test, and compile-fail fixtures. Three of the five ports have a real
adapter that runs the same conformance body as the fake (EventStore/SQLite, Harness/Claude, Toolchain/Node and Rust).
Tracker and DocsSite are fake-only **by recorded design** until Phase 3 (ADR-0094 decision 2; design:267-268; AC-499
GIVEN; AC-507/508/510 are Phase 3 at `runbook/phase-1/s7a-ports-and-fakes.md:1229`), and nothing in the kernel or CLI
calls either, which an equality-asserting lint pins (`crates/rk-kernel/tests/it/conformance.rs:612-660`).

What holds the grade at B and not A: the Tracker conformance body's idempotency assertion is vacuous at the port level
(opaque unit newtypes), `Query::ByRun` is an inert "read everything" selector that callers use as such, the Harness real
row against the live CLI is `#[ignore]`d and operator-run (never CI), one sealed `Binary` arm waits on S-BIN (8i), and the
crate's doc comments are about 42% of its source lines with considerable stale history and stale "proposed" labels.

---

## 1. Map of the area

`crates/rk-ports/Cargo.toml:1-56`. Dependencies: `rk-types`, `serde`, `serde_json` only. No async runtime in the normal graph
(AC-528; stated at `Cargo.toml:37-44`). Features: `corpus = []` (empty, deliberate, `Cargo.toml:10-17`) and
`conformance = []` (`Cargo.toml:18-34`), consumed only on adapter DEV edges (confirmed: `rk-adapter-claude/Cargo.toml:61`,
`rk-adapter-sqlite/Cargo.toml:40`, `rk-adapter-toolchain-node/Cargo.toml:45`, `rk-adapter-toolchain-rust/Cargo.toml:66`;
`rk-cli` and `rk-kernel` name `rk-ports` as a normal dep without the feature).

Size: `src/` 2,218 lines (1,130 code, 934 comment/doc lines, rest blank); `tests/` 1,963 lines (1,247 code).

| Module | Lines | Public surface |
| --- | --- | --- |
| `lib.rs` | 120 | Module list plus two `compile_fail` doctests (`lib.rs:45`, `:97` per test log) pinning P10 and ADR-0115 clause 1 |
| `root.rs` | 43 | `Ports { store: Arc<Mutex<dyn EventStore>>, harness, tracker, docs, toolchain: Arc<dyn _> }` (`root.rs:31-43`) |
| `cancel.rs` | 41 | `CancelToken` (cloneable `Arc<AtomicBool>`, one-way, `cancel()`/`is_cancelled()`); ADR-0099 clause 1 |
| `event_store.rs` | 210 | `trait EventStore` with five methods: `append`, `append_tier1`, `query`, `snapshot`, `rebuild` (`event_store.rs:184-210`); `Query {ByItem, BySha, ByRun}` (String keys); `Snapshot`/`SnapshotRow`/`TierOneAnchor` |
| `harness.rs` | 38 | `trait Harness { fn dispatch(&self, &ActivitySpec, &mut dyn EventStore) -> DispatchOutcome }`; `DispatchOutcome { record, result }` |
| `tracker.rs` | 45 | `trait Tracker { project(&str) -> Result<ExternalRef>, inbox() -> Result<Vec<TypedInput>> }`; `ExternalRef(pub(crate) ())`, `TypedInput(pub(crate) ())` (`tracker.rs:30,34`) |
| `docs_site.rs` | 29 | `trait DocsSite { build(&DocSet), check_links(), drift(&[String]) -> Vec<Drift> }` |
| `toolchain.rs` | 533 | `trait Toolchain` with seven methods (`detect`, `build`, `test`, `lint`, `classify`, `change_summary`, `run_verb`; `toolchain.rs:392-446`); `Classification`; `ChangeSummary {Text, Binary}`; `TextChangeSummary`; sealed `BinaryChangeSummary(pub(crate) ())` (`toolchain.rs:153`); `StepBound`; `Selection`; `TestOutcome`/`TestRunner`/`TestStatus`; `VerbCall`/`VerbOutput`; `MutationFeasibility` + `FeasibilityError` (AC-337) |
| `fakes.rs` | 632 | `VecStore`, `FakeHarness` (scripted, token-aware), `BoardTracker`, `RecordingDocs`, `FixtureToolchain`; all `pub` library items, not `cfg(test)` |
| `conformance.rs` (feature-gated) | 527 | `conform_event_store`, `conform_harness`, `conform_tracker`, `conform_docs_site`, `conform_toolchain`, `validate_report`, `spec()`, `findings()`; `lib.rs:23-25` gates it with the attribute on its own line (Checkpoint 7c greps for that) |

Git is correctly not a port (ADR-0003; `lib.rs:5-11`); `rk-git` is concrete.

---

## 2. What Phase 1 requires of this area (and where it is recorded)

- Design §3.3 table (design:262-270) and port rules P1, P2, P7, P9, P10 (`references/2026-09-08-architecture-design.md`, port
  rules table): one sync object-safe trait per port, fake per port with parity asserted as a number (AC-481), conformance
  suite over `{fake, real}` (AC-499), `Unsupported(capability)` from the scripted responder (AC-500), no async (AC-528), git
  not a port (AC-483), classify/change_summary shape (AC-497).
- `runbook/phase-1/s7a-ports-and-fakes.md` (Step 7a) and later amendments by 7c, s8a, s8c, s8d, s8e (the stage files cite
  their own dated edits to this crate).
- Phase-1 subset of the area: AC-480..AC-495, AC-497..AC-500, AC-509, AC-511, AC-518, AC-528 (acceptance register :7779).
  AC-499 is explicitly four real rows in Phase 1: SQLite, Claude, Node, Rust (register :8022-8031).
- Excluded as Phase 3: AC-507, AC-508, AC-510 (`s7a:1229`).

Records state of Step 7a: `runbook/PROGRESS.md:30` says Checkpoint 7a was "re-run WHOLE and passed at `492e516` and again at
`abeb578`" (2026-09-28, inside Checkpoint 8e) after the s8e edits. That is a record's claim; I did not rerun the checkpoint.

---

## 3. Evidence: what the code and tests show

### 3.1 Which ports have a real adapter at this commit

| Port | Real adapter | Implementor | Real conformance row |
| --- | --- | --- | --- |
| EventStore | SQLite | `SqliteStore` at `crates/rk-adapter-sqlite/src/lib.rs:347` | `port_conformance_real_row_sqlite` (`rk-adapter-sqlite/tests/it/conformance.rs:~40-52`); passed in the audit test run (`log-test.txt:538`) |
| Harness | Claude Code | `ClaudeHarness` at `crates/rk-adapter-claude/src/lib.rs:510` | `port_conformance_real_row_claude` runs the shared body over the real adapter with a **fixture child**, not a real `claude` (`rk-adapter-claude/tests/it/conformance.rs:1-9`, passed `log-test.txt:200`). The real-CLI run is `live::live_dispatch_probe`, `#[ignore]`, paid, operator-run, "never wired into CI" (`tests/it/live.rs:1-8`, `:324-325`). |
| Toolchain | Node and Rust | `NodeToolchain` (`rk-adapter-toolchain-node/src/lib.rs:401`), `RustToolchain` (`rk-adapter-toolchain-rust/src/lib.rs:623`) | `port_conformance_real_row_node` and `_rust`. Rust passed (`log-test.txt:737`). **Node failed in the audit build only because the fixture project's `pnpm install --frozen-lockfile` had not been run in the fresh worktree** (panic text at `log-test.txt:650-655`, from `rk-adapter-toolchain-node/tests/it/support.rs:34`), a documented local-only precondition, an environment artifact and not a code defect. 14 of 18 Node tests failed for that one reason. |
| Tracker | **none** (fake-only; GitHub is Phase 3) | `BoardTracker` | fake row only |
| DocsSite | **none** (fake-only; mdBook is Phase 3) | `RecordingDocs` | fake row only |

`grep` over `crates/**` for `impl <Port> for` outside `rk-ports` finds exactly these four real implementors plus test-local
doubles in `rk-cli`/`rk-kernel` tests. No implementor of `Tracker` or `DocsSite` exists anywhere outside the crate's own
fakes.

Neither the kernel nor the CLI ever calls the tracker or docs ports. `git grep` for `tracker|DocsSite|docs_site` in
`crates/rk-kernel/src` and `crates/rk-cli/src` returns only the two construction sites that fill `Ports` with the fakes:
`crates/rk-cli/src/verbs/run.rs:1016-1017` (the **production** `run` verb) and `crates/rk-cli/src/verbs/doctor.rs:220-221`.
The lint `crates/rk-kernel/tests/it/conformance.rs:585-660` asserts the uncalled set EQUALS `{docs, tracker}` (ADR-0134),
so it would fail if either gained a call site without a record.

Real-versus-fake wiring in the shipped `redkiln run`: store, harness, toolchain real; tracker and docs the fakes
(`run.rs:1009-1018`). `redkiln doctor` builds an entirely fake `Ports` (VecStore, NoDispatchHarness, fake docs/tracker,
absent FixtureToolchain; `doctor.rs:212-222`) on purpose, to prove it cannot dispatch (AC-414).

### 3.2 What is genuinely implemented and exercised

Tests (default `cargo test -p rk-ports`, audit log `log-test.txt:2551-2625`): 24 integration tests in one binary
(`tests/it/main.rs`), 2 `compile_fail` doctests, plus 5 trybuild cases under one harness test
(`tests/ui/*.rs` with committed `.stderr`); all passed. The `conformance` feature run: 24 + 2 passed
(`log-conformance.txt:35-86`). Note the workspace test run already enables the feature through the adapters' dev edges, so
`port_conformance_fake_rows` runs there too (`log-test.txt:2579`).

| Requirement | Instrument | Status |
| --- | --- | --- |
| AC-480 object-safe `Send + Sync` ports; `Ports` shape | `ports_struct::five_ports_are_object_safe_and_send_sync`, `ports_struct_holds_arc_dyn_send_sync` (`tests/it/ports_struct.rs:20,40`); `lib.rs` doctests; `tests/ui/arc_dyn_port_rejects_rc`, `bare_arc_store_has_no_mut` | Exercised, with measured mutation notes in the doc text |
| AC-481 fake-per-port parity as a number, from a source parse | `parity::port_fake_parity` (`tests/it/parity.rs:24`) asserting `traits.len() == fakes.len()` over `Source::parse()` (`tests/it/source_parse.rs`) | Exercised; parse is text-based (column-0 `pub trait`, `impl X for`) but guarded by non-empty-set floors and positive-control fixtures |
| AC-483 git is not a port | `denylist::git_surface_denylist_rejects_git_port` (`denylist.rs:95`) with positive controls plus `tests/ui/git_port_is_not_a_port` | Exercised |
| AC-528 no async in port traits | `denylist::no_async_in_port_traits` (`denylist.rs:139`) with positive and negative controls, `tests/ui/async_fn_in_a_port_trait` | Exercised (compile half). The `cargo tree` half belongs to Checkpoint 7b/7c; not rerun by me |
| AC-364 port half (no update/delete on EventStore) | `event_store_port::ac364_event_store_port_has_no_update_or_delete` (`event_store_port.rs:64`) | Exercised (source parse) |
| AC-497 classify/change_summary shape | `fake_behaviour::toolchain_fake_classifies_binary`; `change_summary::text_variant_serialises_*`, `binary_arm_is_not_constructible_outside_rk_ports` (`change_summary.rs:28,91`) | Exercised for the `Text` arm and refusal of `Binary` ranges. The `Binary` arm itself is sealed and **cannot be exercised** until S-BIN |
| AC-337 mutation feasibility declared | `feasibility::ac337_mutation_feasibility_is_declared` | Exercised |
| AC-499 conformance bodies | `conformance::port_conformance_fake_rows` (`tests/it/conformance.rs:~125-205`): one row per port, harness run over both Ok and Err scripts, docs row with one scripted drift, toolchain row with scripted range; plus `PHASE_1_REAL_ROWS` table naming the real rows' tests | Exercised on fakes; four real rows run in the adapters (see 3.1) |
| Port-set coverage of the matrix | The matrix derives its expected port set from the same `Source::parse()` and fails on a port with no fake row (`conformance.rs` tail) | Exercised |
| Fake behaviours (record T/V/B, ADR-0181/0182) | `fake_behaviour.rs`: 12 tests (red suite/red build are `Ok` with counts; `StepBound(0)` refused; `run_verb` refuses undeclared verbs by name; `VecStore::append_tier1`, `query ByItem`; exhaustive `StepError` match; cancel token) | Exercised |

The conformance bodies are substantive. `conform_event_store` (`conformance.rs:136-250`) covers append-then-query identity,
an `append_tier1` arm (BySha selects exactly the anchored row; ByItem does not leak across items), and a snapshot then
rebuild round trip that is identity. `conform_toolchain` (`:462-527`) covers detect, build green, whole-suite `test` through
`validate_report` (shape, `passed + failed == selected`, status derived from counts, runner named), lint, per-path
classification, and a `change_summary` capability arm. `conform_harness` runs on both termination arms and asserts that an
`Err` termination appended a row (AC-360). It prints `PROBE_RAN_CONFORMANCE=1` as its last statement so the live probe's grep
is evidence the body ran to its end.

Real adapters actually run these bodies: the claim in the crate docs that "the same body" runs on fake and real is true for
SQLite, Claude (fixture child) and Rust in the audit log, and the Node row exists and is wired (failed here only for the
missing fixture install).

### 3.3 Fake-only, stubbed, ARMED-not-run, deferred, or missing

1. **Tracker and DocsSite are contract shells.** `ExternalRef(pub(crate) ())` and `TypedInput(pub(crate) ())`
   (`tracker.rs:30,34`) carry no data; `Tracker::project`/`inbox` cannot express an idempotency key, and no record assigns the
   types a home or contents ("A record naming their home and their contents is OWED", `tracker.rs:23-28`). `DocSet` is an
   opaque newtype from `rk-types` with contents OWED (`docs_site.rs:1-15`). This is intentional for Phase 1 and the
   register excludes AC-507/508/510 until Phase 3, but it means **these two ports are not yet a real specification**; a
   Phase-3 adapter will force a signature change.
2. **`conform_tracker`'s idempotency assertion is vacuous at the port level.** `first == again` compares two unit newtypes
   (equal by construction). The source says so itself: "ARMED, NOT ASSERTED ... removing `BoardTracker`'s dedup guard left
   this assertion passing" (`conformance.rs:~321-336`). The only discriminating observation is the fake's own `board()` window
   in the matrix test (`tests/it/conformance.rs:155-160`), which does not carry to a real row. The body still adds the
   `inbox`-unchanged invariant. Honest, but weak.
3. **The `Binary` arm of `ChangeSummary` is sealed** (`toolchain.rs:153`, with a serialiser that refuses by name,
   `toolchain.rs:155-162`). It is the only item in this crate still blocked on an outside event: Step 8i (S-BIN) opens it
   (`runbook/phase-1/s8i-s-bin.md:364-372`, `:2364-2366`: "If S-BIN cannot run to completion, the `Binary` variant stays
   sealed and this stage does not close"). The sealed-slot test (`change_summary.rs:91`) and
   `tests/ui/change_summary_binary_not_constructible` are explicitly to be superseded by 8i. Every adapter and fake refuses a
   range carrying a Binary path by name rather than answering it (`fakes.rs:~560-585`; AC-497's reviewer-instead-of-diff
   value is therefore not yet deliverable).
4. **`Query::ByRun` is inert.** On both the fake and the SQLite store `ByRun(_)` returns every row (`fakes.rs:129`,
   `rk-adapter-sqlite/src/lib.rs:383`, ADR-0182 decision 5). Production callers use it deliberately as "read all"
   (`crates/rk-cli/src/verbs/status.rs:131`, `cost.rs:186`, `doctor/mod.rs:529`). The enum variant's name now misdescribes its
   behaviour. Also `Query`'s keys are untyped `String`s (item, SHA, run), a type-design debt the docs openly route ("a typed
   item id, a typed commit SHA and a typed run id are three type-design decisions", `event_store.rs:35-39`). The interim
   `ByItem` also returns unkeyed rows (`fakes.rs:130`), a documented but surprising semantic.
5. **Harness real row against the live CLI is operator-only.** `port_conformance_real_row_claude` uses a fixture child; the
   run over the real `claude -p` is `#[ignore]` + paid + never in CI (`rk-adapter-claude/tests/it/live.rs:1-8`). AC-499's
   register text itself accepts a "local-only, named-runner" Harness row until ADR-0010's CI decision, so this is
   per-design, but the CI matrix row remains ARMED (`s7c` §7c.22 per the doc comments at `conformance.rs:~35-36`).
6. **AC-500's route-around half is still ARMED, not built.** The fake-side half (scripted `Unsupported(token)` and the single
   event arm) is tested (`fake_behaviour.rs:159`), but the fake-test doc says the pack route-around half "is not here and is
   not claimed" (`fake_behaviour.rs:151-158`) and `rk-kernel`'s own docs say AC-500 "is ARMED, not claimed"
   (`crates/rk-kernel/src/lib.rs:239-240`). My `git grep` for `route_around|route-around` in `crates/rk-kernel/tests` found
   no discharging test; I could not locate one anywhere in the snapshot, so it is listed below as unverified rather than
   asserted missing.
7. **Fake/real divergence on the Harness Ok path is a recorded cost**: `FakeHarness` appends nothing on `Ok`, while
   `ClaudeHarness` appends `dispatch_completed` on every termination (D21-72 option B, stated at `fakes.rs:36-42`). The
   conformance body therefore asserts the append only on the error arm. It is a conscious, recorded gap in "parity".
8. **Snapshot fidelity differs between fake and real**: the fake keeps only the anchor sha, so snapshot rows carry no `tree`
   or `opening_run` (`fakes.rs:~148-160`); the shared body asserts only anchor-sha parity.
9. **`FixtureToolchain` is a narrow double**: `Affected` and `Named` selections and any unscripted verb return `Unsupported`
   by name; build/lint always pass unless scripted (`fakes.rs:~510-540`). Fine for its job, but the Node adapter implements no
   verb and refuses `change_summary` (`rk-adapter-toolchain-node/src/lib.rs:527-540`); only the Rust adapter answers
   `change_summary` and verbs (`rk-adapter-toolchain-rust/src/lib.rs:728,837`).

---

## 4. Code quality

**Structure.** Clean, small, one concern per module. 1,130 code lines for five ports, five fakes and five conformance bodies.
The crate depends only on `rk-types`, and its feature split keeps test-support code out of the kernel's normal graph. Traits
are narrow and `Send + Sync` throughout. The one non-bare holder, `Arc<Mutex<dyn EventStore>>` (`root.rs:38`), is an
ADR-0115 decision documented in place and pinned by tests plus a doctest.

**Error design.** One error type across all ports, `rk_types::error::StepError` (eleven variants). Red builds and red suites are
`Ok` values with counts, `Err` is reserved for cannot-run/killed/unreadable (`toolchain.rs:395-413`). Capability gaps are
`Unsupported { capability }` carrying a token. `FeasibilityError` is a small typed enum with `Display` and `Error`
(`toolchain.rs:464-495`). Sealed and opaque types are used deliberately (`pub(crate) ()` fields), with trybuild snapshots
pinning the failure reasons. Good.

**Escape hatches in NON-TEST code.** The census (`scratchpad/audit/escape-hatches.txt:10`) shows `rk-ports unwrap/expect=38
panics=4 allow=0 unsafe=0`. I spot-checked every one with `grep` over `crates/rk-ports/src` (excluding doc comments):

- **`#[allow]` / `#[expect]` attributes: 0. `unsafe`: 0. `todo!`/`unimplemented!`/`dbg!`: 0.** The workspace denies
  `unsafe_code` (`Cargo.toml` `[workspace.lints.rust]`) and this crate inherits (`Cargo.toml:55-56`).
- Of the census's 38 `unwrap/expect` and 4 `panic!` hits, **both real `panic!` calls (`conformance.rs:116,488`; the other two census hits are doc-comment mentions) and 26 of the expects are in `src/conformance.rs`**, a
  feature-gated test-support module whose job is to assert; these are `expect("append")`-style assertions and
  `panic!` with diagnostic text (`conformance.rs:91,116,139-525`). Acceptable by design, not production paths; the module is
  never in a normal dependent's build (`lib.rs:23-25`; Checkpoint 7c's `cargo tree` line).
- **Production-path expects: 10, all in `src/fakes.rs`**: nine are `.lock().expect(LOCK)` on the fakes' own `Mutex`es
  (`fakes.rs:210,324,330,336,347,363,369,373,390`), with `LOCK` stating ADR-0118's rule that a poisoned lock halts and is never
  recovered (`fakes.rs:69`); one is `serde_json::to_value(record).expect(...)` on a record whose members all derive
  `Serialize` (`fakes.rs:271`). These are shipped in the production binary because `redkiln run` and `doctor` link the fakes
  (`run.rs:1016`, `doctor.rs:217-222`), but the tracker/docs ones are never reached (section 3.1), `VecStore` has no lock and
  no expect, and `FakeHarness` is not used by `run`. Risk is low and policy-compliant, but it is a panic in library code that
  the crate's own lints do not forbid (no `expect_used` in `[workspace.lints.clippy]`, `Cargo.toml` lints block).
- `FixtureToolchain::test` uses `u32::try_from(...).unwrap_or(u32::MAX)` (`fakes.rs` toolchain `test`), a non-panicking
  saturation. Fine.

**Clippy/format.** `cargo fmt --check` clean; `cargo clippy --workspace --all-targets` exit 0. The library emits 4 nursery
warnings (`log-clippy.txt:428-480`): two `too_long_first_doc_paragraph`, one `significant_drop_tightening` at `fakes.rs:336`, one
`missing_const_for_fn` on `FixtureToolchain::absent` at `fakes.rs:461`. Warn-level only (workspace sets `nursery = warn`;
CI gates on exit code). Cosmetic.

**Dead code.** None found. Every `pub` item has a caller (the fakes' "own window" accessors are used by tests; `Snapshot`,
`TierOneAnchor` are used by SQLite). `Tracker`/`DocsSite` traits are deliberately uncalled by the kernel (equality-pinned).

**Comment-to-code noise.** This is the real quality debt. 934 of 2,218 source lines (42%) are comments or docs, versus 1,130
lines of code, and 584 of 1,963 test lines (30%). Many are amendment archaeology rather than contract:
- Doc comments embed strikethrough and dated edit history (`~~...~~`, "(dated ...)", "(amended ...)", "(Noted ...)",
  "(Corrected ...)": 39 occurrences across `fakes.rs`, `toolchain.rs`, `event_store.rs`, `conformance.rs`; e.g. the four-way
  "opaque newtype count" correction trail at `toolchain.rs:10-26` and `event_store.rs:46-53`, which ends with a count that
  is itself a correction of a correction).
- **Stale status labels**: 23 lines in `src/` still describe records as `proposed` (e.g. ADR-0181 at `event_store.rs:14,42`,
  `conformance.rs:133,190`; record T/ADR-0169 at `conformance.rs:386`; ADR-0171 at `toolchain.rs`), while the ADR files'
  frontmatter reads `status: accepted` (ADR-0181, 0182, 0165, 0169, 0170, 0171, 0176, 0185; e.g.
  `references/adr/0181-...md:3`). The code is right and the labels are behind.
- Some doc claims are now false or self-contradicting and are patched by later paragraphs rather than rewritten
  (`fakes.rs:~70-90` still opens with "`query` returns every appended event whatever the selector" under a struck-through
  marker; `toolchain.rs` module doc still says "four opaque newtypes", then "three", then "four").
Readers must parse layered corrections to learn the current contract; the contract itself is correct in code.

**Test depth.** Unit-level behaviour tests on the fakes (12), integration-level matrix over all five ports, source-parse
structural tests (parity, denylist, event-store method set, toolchain method count), compile-fail (trybuild with committed
`.stderr` plus two doctests), and positive-control fixtures in the lint-style tests. Mutation-hardening is a stated practice
(multiple "[measured: ... left this test green]" notes drove changes). There are no golden/snapshot tests in this crate beyond
the `.stderr` files and no property tests (the workspace has `proptest` only in `rk-kernel`). Depth is good for a 1,130-line
crate; the main weakness is the vacuous Tracker idempotency assertion and text-parse brittleness (the parity test counts
`pub trait` at column 0, so a differently formatted trait declaration could evade it; the code acknowledges this and holds
it with a PIA-059 fixture).

---

## 5. Risks

1. **Tracker/DocsSite interfaces are unvalidated shapes.** Any Phase-3 adapter (GitHub, mdBook) will probably change the
   trait signatures and the opaque types, with ripples into `Ports`, the kernel's port lint, the fakes and the conformance
   bodies. The conformance idempotency check is not live until `ExternalRef` has contents. Low impact for Phase 1 exit,
   real for Phase 3.
2. **`ChangeSummary::Binary` is the one Phase-1 blocker living in this crate** and depends on the in-flight S-BIN stage
   (8i). If S-BIN does not complete, the variant stays sealed and 8i does not close (`s8i-s-bin.md:2364`), and per the stage
   file Phase 1 exit with a sealed variant would need a new operator decision (`:2366`).
3. **Inert/untyped query selectors** (`ByRun`, `String` keys, interim "unkeyed rows match every item") are a latent
   correctness trap as the store grows beyond the single-item interim; the semantics are recorded (ADR-0182) but the type
   still invites misuse.
4. **Doc rot is the maintenance risk**: nearly half of the crate's lines are narrative, with stale `proposed` labels and
   layered corrections. New contributors (or a model-driven stage) could act on a superseded sentence.
5. **Panics in shipped fake code** (nine mutex expects plus one serde expect) are policy-justified (ADR-0118) but are real
   `expect` calls in library code linked into the production binary.
6. **Parity is syntactic plus a derived-from-source count, not behavioural for two ports.** For Tracker and DocsSite the
   "conformance" is a smoke test of the fake against itself. The Harness Ok-path journal differs between fake and real by a
   recorded decision (D21-72).
7. **Environment sensitivity of real conformance rows**: the Node row needs a one-time `pnpm install` per checkout (fails
   red on a fresh clone); the live Harness row needs paid, operator-run dispatch. Neither is in the default CI-safe path.
8. **Snapshot/rebuild parity gap**: the fake's snapshot omits `tree`/`opening_run`, so a rebuild round trip is only fully
   exercised on the SQLite row.

---

## 6. Unverified (explicit)

- I did not run the Step 7a checkpoint block, `cargo tree -p rk-kernel -e normal`/`cargo tree -p rk-ports -e normal` (AC-528's
  graph half), or the Step 7c feature-graph line. I relied on the records and the audit build's logs.
- AC-500's route-around half (a pack-level path that executes on `Unsupported`, with the mutation-style check): I found no
  test for it in `rk-kernel/tests` by `git grep`, and the records say "ARMED, not claimed", but I did not exhaustively search
  `rk-cli` tests or later stage files for a discharge.
- Whether the live `ClaudeHarness` conformance row has been run recently against the real CLI: the records (R-18 closed "by
  the probe" at 2.1.277, `PROGRESS.md:180`) say it has been run at least once on 2026-09-18; I did not check later runs or
  their results. The s8i capture sessions are separate and out of this area.
- I did not read `redkiln-corpus` or any CI workflow to confirm which conformance rows are CI matrix rows.
- Exact line numbers marked with `~` (for example in `fakes.rs` toolchain section and `tests/it/conformance.rs`) are
  approximate to within a few lines; the files and function names are exact.
- The Node row failure is attributed to the missing fixture install by the panic text; I did not install and rerun it.
- The working tree at `D:/repos/redkiln-rs` has uncommitted changes (per the session's git status). This report covers the
  pinned `36dd8ac` snapshot only.

## 7. Summary table

| Item | Result |
| --- | --- |
| Ports declared | 5 (EventStore 5 methods, Harness 1, Tracker 2, DocsSite 3, Toolchain 7) |
| Fakes | 5, all `pub` library items |
| Real adapters | EventStore: SQLite; Harness: Claude; Toolchain: Node and Rust. Tracker/DocsSite: none (Phase 3) |
| Real conformance rows passing in audit build | sqlite, claude (fixture child), rust pass; node red only for missing `pnpm install` |
| Tests in crate | 24 integration + 5 trybuild cases (one harness test) + 2 doctests; all green; fmt/clippy exit 0 |
| Escape hatches (non-test) | 0 allow/unsafe/todo; 10 expects in `fakes.rs`; 26 expects + 2 panics in feature-gated `conformance.rs` (assertions) |
| Open items inside this crate | Binary arm sealed (S-BIN, 8i); Tracker idempotency vacuous; AC-500 route-around ARMED |
| Grade | B |
