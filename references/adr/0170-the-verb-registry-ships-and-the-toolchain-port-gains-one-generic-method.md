---
id: "0170"
title: "**The verb registry ships, and the `Toolchain` port gains one generic method.** `.redkiln/registry.toml` fills design §4.5's verb table (D21-10's gap): every row declares `outputs`, `live`, `timeout`, `inputs` and a `route` from a closed set, and a `toolchain` row reaches the seventh port method, `run_verb(&VerbCall) -> Result<VerbOutput, StepError>`, whose output the kernel types at the boundary. The Rust adapter's `smoke`, `sca_scan`, `package` and `setup` handlers are fixed here; the registry also declares the `[checklist.*]`, `[kb.*]` and `[feed.*]` source tables and a per-toolchain value table whose every value carries its warrant under ONE rule written before any reading. Supersedes, each in part and quoted, `0135` clause 1, clause 6, its Decision sentence on the six-method count and one forbids bullet with its second sentence, `0162`'s seventh-method forbids half, and `0137`'s Decision sentence that the count stays six (added 2026-09-26, refuting round 1); carries `D21-62` C (the advisory-database commit is read by the adapter, before and after the scan)"
status: proposed
date: 2026-09-26
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 decisions 1b, 4a and 6a, D21-43 N14, N25 and the integration call (*"The Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4)."*), D21-45 W3, D21-48 X4 and D21-62 C (TAKEN PROVISIONALLY; panel text spikes/afk-panels/D21-62-s8c.md); runbook/phase-1/s8c-toolchain-port-and-verbs.md §8c.4–§8c.6 and §8c.7 (the proposal this record fixes); references/adr/0135, references/adr/0162 and references/adr/0137 (the sentences superseded in part); references/2026-09-08-architecture-design.md §4.5 (*"`registry.toml` (toolchain verbs and feeds; a verb's `live = true` implies `memo = false` on every step using it)"*); crates/rk-kernel/src/workflow/memo.rs (`VerbRegistry`, the one parser this record extends)
supersedes: ["0135", "0162", "0137"]
superseded_by: []
related: ["0032", "0040", "0072", "0105", "0137", "0140", "0148", "0165", "0169", "0171"]
evidence: []
consumers: []
---

## Context and problem

`run_tool_step` routes through `const MAPPED_VERBS: [&str; 4] = ["build", "change_summary", "lint", "test"];`, and
`.redkiln/` ships no `registry.toml`. D21-10 recorded that absence *"against design §4.5's five-registry row"* as *"a
gap and deliberately not filled"*. Measured over `.redkiln/catalog/`'s `run =` lines: **113** `tool` steps across
**33** verbs. The verbs the later stages need (`smoke`, `sca_scan`, `package`, `item_create`, and a release tag) all
refuse by bare name under ADR-0135 clause 2, which is correct for a verb nobody routes.

The **94** `${toolchain.*}` substitutions in the catalog have no value source. Over the step `inputs` of the 102
catalog files there are **50** distinct `checklist:` / `kb:` / `toolchain:<feed>` keys, and nothing declares or holds
any of them.

D21-42 1b took the route: *"**A registry-backed generic port method**, filling design §4.5's `registry.toml` verb
table (D21-10's gap). This is a seventh method: it supersedes ADR-0162's forbids bullet and ADR-0135's "six" in part,
and Checkpoint 7a is re-run."* 4a took the values: *"**A per-toolchain table in the 1b registry.** Each value carries
its measurement as its warrant; an unresolved key is refused by name."* N14 took the sources: *"**Declared
`[checklist.*]`, `[feed.*]` and `[kb.*]` tables in s8c's registry.** Undeclared keys are refused by name, and content
is authored only for the keys the lanes reach."*

**The count every record in this area repeats is false.** ADR-0135's `## Decision` says *"the count stays **six**, as
Checkpoint 7a asserts"*, and ADR-0162's drivers say *"Checkpoint 7a counts exactly six methods on the `Toolchain`
port"*. Checkpoint 7a counts no method: its only method-level group is a floor over the NAMES `fn detect` and `fn lint`
(`s7a` §7a.3: *"The floor pins **names**, never a signature"*). A re-run of Checkpoint 7a would pass over six methods,
seven or seventy, so the exact count is `s8c`'s own assertion
(`source_parse::toolchain_port_declares_exactly_seven_methods`).

At stage open, cargo-audit 0.22.2 and cargo-deny 0.20.2 were measured offline (D21-62 C). Neither reports its
advisory database's commit, so §8c.4's rule that the commit is *"read from the report's own database member"* would
refuse every scan.

## Decision drivers

- One verb table, one parser. `rk_kernel::workflow::memo::VerbRegistry` already parses `spikes/p9/registry.toml`'s
  `[verb]` shape for AC-018. A second reader would give one file two meanings.
- A silent default is the failure ADR-0140 was written about. An unresolved key, an undeclared source and an
  unrouted verb each refuse by name.
- AC-487: *"`rk-kernel` contains no comparison against an adapter identity"*. The value table reaches the kernel
  without the adapter's name.
- Golden constraint 6: *"size the ceiling against the slow one"*. A value's rule must be fixed before the reading it
  turns into a value, or the rule is chosen to fit.
- Golden constraint 8: a scan whose advisory database moved mid-lane is two instruments.
- Every D21-62 ruling is provisional, so its clause here must be undoable as one clause and one code branch.

## Considered options

### Option — One generic port method routed by a shipped registry (taken)

D21-42 1b's route, with the shapes §8c.4–§8c.6 proposed. Taken.

### Option — One port method per catalog verb

Rejected. It mints port surface for every verb a pack names: *"a sixth, seventh and thirtieth method"*, which
ADR-0135 rejected for the same reason and which `resolver.rs` forbids as invention.

### Option — Extend the adapters' five-token `Capability` enum into a verb table

Rejected, and ADR-0135's bullet against it stands. The enum carries dated probe rows about adapter support. A verb
table maps a verb to an output kind, a liveness flag, a bound and a route, which is a different question.

### Option — Read the advisory-database commit from the tool's report, and refuse when absent (the §8c.4 proposal)

Rejected by measurement (D21-62 C). Offline, `cargo audit --json --no-fetch` emits `database.last-commit: null`, and
`cargo deny --format json check advisories --audit-compatible-output` emits no database member. The rule would refuse
every scan on the pinned versions.

### Option — Let the scan fetch its databases so the report carries a commit

Rejected (D21-62 C option 3). It breaks the offline rule and golden constraint 8, moves the shared `~/.cargo`
databases, and still leaves cargo-deny without a commit.

## Decision

**1. `.redkiln/registry.toml` ships, and `VerbRegistry` is extended to read it.** Every verb row declares `outputs`
(the typed output kind), `live`, `timeout` (a `${toolchain.*}` key), `inputs` (the names the call carries) and
`route`, from a **closed** set matched exhaustively with no wildcard, ADR-0135 clause 1's own idiom:

- `port`: the four existing methods, `test`, `build`, `lint`, `change_summary`;
- `toolchain`: the generic method, decision 2;
- `store`: an rk-store write;
- `git`: through the journalled `GitOps` seam.

A later record may add a route (`s8d`'s, for `verdict_record`). It does so by record, not by editing the match.
`s8d`'s R1 adds a `side_effecting` column to every row this record lands.

**The rows this record lands:**

| verb | route | typed output | handler |
| --- | --- | --- | --- |
| `test`, `build`, `lint`, `change_summary` | `port` | `TestReport`, `BuildReport`, `LintReport`, `ChangeSummary` | the port methods (`test` and `build` return outcomes, ADR-0169) |
| `smoke` | `toolchain` | `SmokeResult` | Rust, decision 4; `live = true` (`s8b` §8b.5's hand-on) |
| `sca_scan` | `toolchain` | `ScanReport` | Rust, decision 5 |
| `package` | `toolchain` | `BuildArtifact` | Rust, decision 6 |
| `setup` | `toolchain` | `SetupResult` | Rust, decision 7; `live = true`; no catalog site, called by `s8d-ii` at run-worktree creation |
| `item_create` | `store` | `ItemRefs` | none: refused naming the route until `s8f` |
| `release_tag` | `git` | `ReleaseTag` | none: refused naming the route until `s8g` |

The other verbs have no row and refuse by **bare** name (ADR-0135 clause 2, unchanged); `mutate` and
`release_record` are among them.

**2. The seventh method.** `fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError>;`

- `VerbCall` carries the verb, its resolved inputs as `serde_json::Value` by name, and the `StepBound` (ADR-0171).
- `VerbOutput { kind: String, value: serde_json::Value }` is erased at the port. **The kernel types it at the
  boundary**, as `report_at_boundary` does for `VerifyReport`: a closed decode table maps each declared kind to its
  `rk-types` struct. An output whose `kind` is not the row's `outputs` is refused naming both kinds; a value that does
  not decode is refused naming the kind.
- **An adapter implements the verbs it can and refuses every other one by name**
  (`Unsupported { capability: "<verb>" }`). The Node adapter implements none at this stage.
- **`run_verb` is not a patch applier.** No `mutate` row lands, and ADR-0162's refusal of `ReproKind::Mutate` stands:
  a generic method is exactly what would make applying a patch from `rk-cli` tempting.

The port has **seven** methods: `detect`, `build`, `test`, `lint`, `classify`, `change_summary`, `run_verb`.

**3. The executor routes through the registry, in a fixed order.** In `run_tool_step`, `MAPPED_VERBS` is replaced by
a registry lookup. The lookup runs first, then ADR-0148's `test` refusal, then the route and input checks, and **only
then** is the step's bound resolved (ADR-0171). Each refusal is tellable from the others in the journal:

| case | outcome |
| --- | --- |
| no row | ADR-0135 clause 2's bare-verb refusal, unchanged |
| `test` | ADR-0148's sentence, unchanged (`s8d` reverses it) |
| a `port` row | its port method, given the step's bound |
| a `toolchain` row whose declared inputs the step cannot supply without the resolver | refused before the port, naming the verb and the first unresolved input (the resolver is `s8d`'s) |
| a `store` / `git` row | refused naming the route and the stage that builds it |

So on the shipped catalog nothing new walks at `s8c`: the verb implementations are proved over fixtures through the
port, and the executor's reach to `run_verb` is proved over a test-only row that declares no inputs.

**The typed outputs.** `BuildReport` (ADR-0169), `SmokeResult`, `ScanReport`, `BuildArtifact`, `SetupResult`,
`ItemRefs` and `ReleaseTag` are `rk-types` structs with `#[serde(deny_unknown_fields)]`, no `Default` and every member
required. They are **not** added to `EXPORTED` (which kinds an agent emits is `s8h`'s, D21-41 g), and they stay
`oneOf`-free.

**4. `smoke`.** The Rust handler builds, then runs each declared checklist row against the binaries that build
produced, never an installed `redkiln`. `SmokeResult` holds one row per checklist entry (`id`, the argv run, its
`cwd`, the expected exit, the actual exit, the stdout predicate and whether it held, `passed`), plus `passed`,
`failed` and `status`. The item form (`argv`, `cwd`, `expect_exit`, one stdout predicate) is `s8g` §8g.6's. It follows
1a's rule: **a checklist row that fails is `Ok` with `failed ≥ 1`**; `Err` means only that the build or a spawn could
not run, or that the run was killed.

**The scratch placement** (`s8g` §8g.6 leaves it to this record). The handler makes one empty directory per call under
ADR-0137 clause 6's scratch location, `<git-common-dir>/redkiln/tmp/smoke-<unique>`: outside every
`.redkiln/worktrees/<item>` and outside the project's checkout. It resolves each item's `cwd = "scratch"` to it, and
removes it on every path (success, failure, kill). A removal that fails is `Err` naming the path, never a silent leak.
The handler runs no git, so ADR-0105 clause 5's write set is untouched; a row that needs the directory to be a
repository makes it one in its own argv, which is `s8g`'s.

**5. `sca_scan`, and the scan's semantics** (the ones `s8g` §8g.6 reads, fixed here so the two files agree).

- **Both tools run, offline, each against a pinned local database.** `cargo audit --json --no-fetch --db
  <audit_db> --file <Cargo.lock>`, and `cargo deny --offline -L debug --format json check advisories
  --audit-compatible-output` under a config the adapter generates. *(Measured at stage open: cargo-deny 0.20.2 has no
  `check --disable-fetch`; its offline switch is the top-level `--offline`.)*
- **Severity.** Each advisory's CVSS base score maps onto the CVSS v3 qualitative bands: `critical` ≥ 9.0, `high`
  7.0–8.9, `medium` 4.0–6.9, `low` 0.0–3.9 (CVSS v3's *None* band, a score of exactly 0.0, has no member of its own
  and is counted `low`; it is never `unrated`, because it was scored). **An advisory with no CVSS is counted in `unrated`**; it is never counted as
  low or none, and never dropped. Both tools carry a per-advisory `cvss` member (a vector string or null). Scoring a
  vector needs a scorer, and this record names it: the RustSec **`cvss`** crate, licensed MIT OR Apache-2.0, both
  already on `deny.toml`'s allow list, so no licence amendment is needed. The alternative is in-tree CVSS v3.1
  base-score code. The builder records which landed and, if the crate, the `cargo deny check licenses` reading over
  it and its transitive dependencies. *(dated 2026-09-26, s8c build; D21-63, provisional.)* In-tree CVSS v3.1
  base-score code landed (v3.0 vectors scored by the same equations). A matched advisory whose vector that scorer
  cannot score (~~a CVSS v4.0 vector, or~~ a malformed one) refuses the scan by name (`cvss-unscorable`, naming the
  vector); it is never counted `unrated`, since it has a CVSS, and never banded by a guess. ~~Provisional under D21-63;
  the named alternative is the `cvss` crate's v4 scorer banded on these same bands (FIRST CVSS v4.0 §6 uses the v3
  qualitative scale), which reverses this clause as one dependency line and one match arm.~~
  *(dated 2026-09-26, s8c build, CVSS; D21-64 ruling 7, which replaces D21-63 option (1) with its (3').)* Both
  scorers landed. A `CVSS:3.0` or `CVSS:3.1` vector is scored by the in-tree code, and a `CVSS:4.0` vector by the
  RustSec `cvss` crate's v4 scorer, `cvss = { version = "=2.2.0", default-features = false, features = ["std",
  "v3", "v4"] }` in `rk-adapter-toolchain-rust` (the version rustsec 0.33 parses the database with). Both are
  banded on these same thresholds (FIRST CVSS v4.0 §6 uses the v3 qualitative scale), and a v4.0 score of exactly
  0.0 counts `low`, as v3's does. A differential test pins the in-tree v3 scorer against `cvss::v3` over every
  distinct v3 vector in the RustSec database. A vector neither scorer accepts (malformed, or any other version)
  refuses the scan by name (`cvss-unscorable`, naming the vector and the two remedies left: correct the vector, or
  rule it `unrated` so `sca-scan` routes to `scan.accept-findings`); it is never counted `unrated` and never banded
  by a guess. The crate scores a v4.0 vector **as written**: one that carries threat metrics (`E:`) is scored as
  CVSS-BT under FIRST v4.0's nomenclature, not CVSS-B, so for such a vector *"the CVSS base score"* above reads as
  the crate's score of the whole vector (flagged in the evidence; the ruling fixed CVE-2025-47273's 7.7).
- **`ScanReport`** carries `critical`, `high`, `medium`, `low` and **`unrated`**, each required; one entry per
  advisory (id, package, version, severity or none); and, **per tool**, the tool's name and version, its database's
  local path and **that database's commit** (D21-42 6a: *"the advisory-DB commit is recorded"*). **The two databases
  are two pins.** On this box `cargo audit` reads `~/.cargo/advisory-db` and `cargo deny` reads
  `~/.cargo/advisory-dbs/advisory-db-<hash>`, each its own git checkout.
- *(D21-62 C, provisional.)* **The adapter reads each database's commit itself.** It never relies on a tool's default
  location. Audit is given `--db <audit_db>`. Deny is given a generated config whose `[advisories] db-path` is a
  dedicated parent directory, written with forward slashes, that must hold **exactly one** child; deny opens
  `<db-path>/advisory-db-<url-hash>` there, so the adapter does not compute the hash. For each database the adapter
  reads `HEAD` through **one shared read-only `rk_git` head read** (built on gix) **before** its tool runs and
  **again after** it exits. The scan **refuses by name**, recording no `ScanReport`, when:
  (a) a database path is not a git checkout, or its `HEAD` does not resolve;
  (b) the before and after `HEAD`s differ (*database moved mid-scan*);
  (c) a non-null report member (audit's `last-commit`) disagrees with the adapter's read. The member is cross-checked,
  never allowed to overwrite the read;
  (d) deny's `db-path` holds a child count other than one;
  (e) deny's pinned DEBUG line, *"Opening advisory database at '<path>'"*, is missing or names a path other than the
  child the adapter read. **That line pins the path only, never the commit**, and its text is a per-deny-version pin
  that may break on an upgrade, which fails closed.

  If a later tool version emits the commit member, the member becomes the primary source and the adapter's read the
  cross-check, with no change to `ScanReport`. Whether a test seam that moves `HEAD` between the two reads is
  admissible in the adapter is left to the builder; if it is not, refusal (b) is asserted by a unit test of the
  comparator only, and the builder says so rather than claiming end-to-end coverage.
- `s8b`'s catalog edit is what reads `unrated` (D21-43 N25: *"**`sca-scan` requires `unrated == 0`** or goes to
  `scan.accept-findings`"*). Checkpoint 5's replay fetches both databases, so a Checkpoint 8c run prints both commits
  before and after it.

**6. `package`.** `cargo build --release --locked --message-format=json`, then the sha256
(`rk_types::digest::sha256`, the plain hash an independent `sha256sum` agrees with) of each **declared** shipped binary,
never a glob, because a release build also links the fixture `[[bin]]`s. `BuildArtifact` carries the profile, one
`(name, relative path, sha256)` per declared binary, the argv, the toolchain version, and the input `sha` together with
the project root's `HEAD`, read through **the same shared `rk_git` head read** as decision 5. **A declared binary the
build did not produce is refused, and so is a `sha`/`HEAD` mismatch**; an undeclared set is refused by name. `s8a`'s
Checkpoint 8a guard (6) lets the Rust adapter's code lines name `rk_git` only as `rk_git::diff` and `rk_git::path`, so
the commit that lands the shared head read amends that allowlist by a dated note in `s8a`, one amendment for both
reads (D21-62 C).

**7. `setup` (D21-43 N25; D21-45 W3).** The row is `toolchain`-routed, outputs `SetupResult`, is `live = true`, and
its timeout is `setup_timeout`. The project set is a declaration row, `setup_projects`, in the per-toolchain table. In
this repository `[toolchain.rust]` lists one project, `crates/rk-adapter-toolchain-node/tests/fixtures/project`, which
holds the only tracked `pnpm-lock.yaml`. For each declared project, in order, the Rust handler runs `pnpm install
--frozen-lockfile --prefer-offline --dir <project>` inside the worktree, with `pnpm` resolved through
`resolved.toolchain_programs`. **A non-zero install is `Err`**, naming `setup`, the project and the exit code; it is
never `Ok` with a failed row. An empty declared set is `Ok` with no projects. `SetupResult` has one row per project
(its path, the argv, the sha256 of the lockfile installed from) and the toolchain, and no `status`. The Node adapter
refuses `setup` by name at this stage.

**8. The source tables (D21-43 N14).**

- `[checklist.<id>]` and `[kb.<atom>]` each declare exactly one of `path` (relative to `.redkiln/`:
  `checklists/<id>.toml` or `kb/<atom>.md`) or `unauthored = "<reason, naming the owner>"`. A checklist is a TOML file
  of `[[item]]` rows (`id`, `text`) plus a `source` naming what it was authored from.
- `[feed.<toolchain>.<name>]` declares exactly one of `verb = "<a registry verb>"` or `unprovided = "<reason>"`.
- **Resolution** (the resolver is `s8d`'s): an **undeclared** key is refused by name, even for an optional (`?`)
  input; an `unauthored` or `unprovided` key is refused by name when mandatory and is `null` when optional (ADR-0032's
  Decision: *"Absent `?` inputs are `null`"*).
- **Loading** refuses, naming the entry, on a `path` that does not exist or holds no item, a feed `verb` that is not a
  registry row, and an entry that declares both forms or neither. The loader reads declared paths only.
- The **14** lane-reached keys are declared (six checklists and four atoms authored here, `checklist:smoke`
  `unauthored` naming `s8g` §8g.6, and the feeds `backlog`, `logs?` and `sbom?` `unprovided` on both toolchains); the
  other **36** are not declared and refuse by name wherever an activity reaches them. Every authored file's `source`
  names its provenance.

**9. The per-toolchain value table (D21-42 4a).** `registry.toml` carries `[toolchain.rust]` and `[toolchain.node]`,
keyed by the `toolchain` facet's value. Each `${toolchain.<key>}` has **either** a value with its warrant **or**
`unmeasured = "<reason>"`. A warrant is structured: `secs`; `measured`, the list of readings; `rule`, how the value
follows from them; `pin`, the 40-hex commit of this repository the readings were taken or read at; and exactly one of
`command` (a value measured here) or `source` (a value cited from an existing record). **The loader refuses a value
without a warrant; resolution refuses an `unmeasured` or absent key by name** (ADR-0171). `rk-cli`, which already
holds the name `real_toolchain` chose, selects the table and hands the kernel a flat `ToolchainValues` (key → seconds,
or key → `unmeasured`), so the kernel never sees `rust` (AC-487).

`[toolchain.rust]`'s timeout keys are exactly: **measured** `test_timeout` (on the runner the port selects: the
`--list` pass, `cargo nextest run` and the `cargo test --doc` companion, each child timed), `build_timeout`,
`scan_timeout` and `setup_timeout`; **cited** `implement_timeout`, `review_timeout` and `repro_timeout` (agent steps the
lanes reach, so none may be `unmeasured`); **unmeasured** `deploy_timeout`, `migration_timeout`, `cook_timeout`,
`ue_test_timeout` and `mutation_timeout` (no Phase-1 Rust verb deploys, migrates, cooks, runs Unreal automation or
mutates). The table also admits **declaration rows** carrying a `source`, not a `measured` list: `setup_projects`
(written here), `package_binaries` (`s8g` writes this repository's set) and `s8g`'s `claims` column (AC-349). Adding a
row is a record's act. Every key in `[toolchain.node]` is `unmeasured`.

**10. THE RULE for every measured key**, fixed here on 2026-09-26 **before any §8c.5 reading was taken**:

> **`secs` = twice the slowest reading in `measured`, rounded up to the next whole minute:
> `secs = 60 × ⌈(2 × max(measured)) / 60⌉`.** The slowest reading is the maximum over every pass (n = 3) and, for a
> key whose call spawns several children, over every child of every pass. It is never a median or a mean.

The rule is §8c.5's own worked example (*"twice the slowest reading, rounded up to the minute"*), written into the
stage file before any measurement existed and adopted verbatim, so no reading informed the choice. It sizes against
the slow condition (golden constraint 6) with a margin of one slowest-reading, because n = 3 cold readings are a wall's
first recording, not a p95. A reading that fetched from the network (pnpm's `downloaded` count, above zero) is labelled
a network reading in `measured` and is not mixed silently with local ones; the rule still takes the maximum. **A value
above the construction bound is legal and is not clamped**: ADR-0171's `min` makes the construction bound win, and
this record says so rather than rewriting the value.

The cited agent-step keys follow the same rule over their cited readings, with one exception the operator took:

- `review_timeout` cites AC-223's own evidence line, F31/S-TYPED *"CLEAN review median 1,071 s, slowest 4,561 s"*, as
  its class's dispatch-duration record. Under the rule, `measured = [4561]` gives `secs = 9180`. AC-223's floor
  (*"at least the recorded slow-condition figure for that class (its recorded maximum, not its median)"*) holds.
- *(D21-48 X4.)* **`implement_timeout` and `repro_timeout` are 4,800 s each**, *"cited to F31/S-TYPED's slowest agent
  dispatch (4,561 s) as a borrowed class"*. Each carries `secs = 4800`, `measured = [4561]`, a `source` naming
  F31/S-TYPED and the words *borrowed class*, and a `rule` naming the re-derivation below. **4,800 s is the operator's
  value, not the rule's output**: the rule over the same borrowed reading would give 9,180 s. It is flagged here so
  that the flip reads the two side by side.

**11. The borrowed class is re-derived once its own class is recorded (D21-48 X4).** X4 reads: *"Every fixture-proof
and real dispatch records its duration, and the record names the rule that re-derives each key from its own class's
recorded maximum once one exists."*

- **The record it reads.** A key's own class is the activity class its steps sit on: the implement class for
  `implement_timeout` (`tdd-red-green-refactor`'s `red`, `green` and `refactor`, `equivalence-boundary/write`,
  `docs-update/write`), and `minimal-repro/attempt` for `repro_timeout`. The class's dispatch-duration record is the
  set of recorded wall durations of that class's dispatches, each with its run and its pin.
- **The rule.** Once such a record exists, the key is re-derived from that class's recorded maximum by decision 10's
  rule, and never below AC-223's floor. The re-derivation is a dated record edit that replaces the `source` and drops
  *borrowed class*.
- **Until then 4,800 s stands.** It is never re-derived from a median, or from a class other than the key's own.
- This stage dispatches nothing and records no duration. The stages that run the fixture proofs and the real
  dispatches record each one's duration (§8c.12).

**12. `RK-D-TIMEOUT` gains a third arm.** A value of the exact form `${toolchain.<key>}`, in a pack activity table or
on a catalog step, is resolved through the `ToolchainValues` that `run` would hand the kernel, selected by the same
detection, so `doctor` and `run` cannot disagree about a key. A resolved key is numeric and the row is silent. An
**absent** key is an `Error` finding naming `${toolchain.<key>}` (`Unresolved`). An **`unmeasured`** key is an `Error`
finding naming its declared reason: the rule keeps one severity, `Severity::Error`, and AC-330 reads *"doctor fails if
no numeric `timeout_secs` is declared (an inherited default is a failure)"*. So on this repository `doctor` reports one
`Error` for each of the 23 catalog steps whose key is `unmeasured`, which is the true state of those activities. If no
adapter detects, one `Error` finding says so, and no key is guessed. AC-419's two-case fixture still yields exactly
two findings.

**13. What this record supersedes, each in part (ADR-0092 rule 1).**

- **ADR-0135 clause 1**, *"The map is closed and holds exactly four entries"*. The routing is the registry's closed
  route set (decision 1); the four `port` rows keep the map's spelling and its no-wildcard rule.
- **ADR-0135 clause 6**, *"The 29 unmapped verbs are recorded as an open gap, owned by no stage in this phase"*, for
  the five rows landed here: `smoke`, `sca_scan`, `package`, `item_create` and `release_tag`. The first four are among
  ADR-0135's 29 measured catalog verbs; `release_tag` is the verb of `s8g`'s new `tag-release` activity. `setup` has no
  catalog site. The gap stands for every other verb.
- **ADR-0135's `## Decision` sentence** *"It adds no method to the `Toolchain` port: the count stays **six**, as
  Checkpoint 7a asserts"*. The count moves to seven, and *"as Checkpoint 7a asserts"* is measured false.
- **ADR-0135's forbids bullet** *"Forbids adding a port method to satisfy a catalog verb"*, **for the one generic
  method only**, together with that bullet's second sentence, *"The port's six are ADR-0003's and ADR-0094's, and
  Checkpoint 7a counts them"*: the count moves, and *"Checkpoint 7a counts them"* is measured false. A method per
  verb stays forbidden.
- **ADR-0162's forbids bullet** *"Forbids a seventh method on the `Toolchain` port, and forbids applying a mutation
  patch from `rk-cli`"*, **its seventh-method half only**. The patch half stands here; `s8g`'s verify record may move
  it for the verify gate's declared AC mutations alone.
- *(dated 2026-09-26, s8c build, refuting round 1)* **ADR-0137's `## Decision` sentence** *"It does not add or remove a port method; the count
  stays six."*, in its closing paragraph *"What this record does NOT decide"*. The count moves to seven. The
  sentence sits under `## Decision`, so ADR-0092 rule 1 routes it here and not to an erratum, as E-0137-1 routed
  clause 5's false sentence. The rest of that paragraph (the artifact-kind sentence, the 29 unmapped verbs and
  criterion (1)) and every clause and forbids bullet of ADR-0137, as `0139` and `0140` left them, stand. Back-link:
  `superseded_by: ["0139", "0140", "0170"]`.

**What stands.** ADR-0135 clause 2 (the bare-name refusal); clause 3 (`classify` and `detect` are not routed); and
clauses 4 and 5 **only as ADR-0165 decision 9 left them**: decision 4 stands for the Node adapter, and for the Rust
adapter's refusal of an empty range and of a range carrying a `Binary` or LFS pointer path, with *"Both real adapters
return … from a row dated `2026-09-18` …"* standing for the Node adapter only; decision 5's bold lead and its clause
*"without the type ever appearing in a signature or a binding in `rk-kernel`"* stand for production `rk-kernel`, and
move only as far as a test double's impl signature under `tests/`, with its Checkpoint 5 grep sentences superseded by
ADR-0165. This record takes what stands from ADR-0165 decision 9's text, not from any restatement of it. ADR-0135's
bullet *"Forbids reading the adapters' `Capability` enum as a verb table, or extending it with catalog verbs"* stands:
**the verb table is `registry.toml`, and the five-token `Capability` enum is untouched**. Checkpoint 7c digests its
four byte-equal bodies. ADR-0162's refusal of `ReproKind::Mutate` stands.

**14. D21-10's gap is filled.** The pack's missing `registry.toml`, recorded against design §4.5's five-registry row,
ships under this record.

## Consequences

The port has seven methods, and the exact count is asserted by
`source_parse::toolchain_port_declares_exactly_seven_methods` and Checkpoint 8c (4), not by Checkpoint 7a's name floor.
Checkpoint 7a is re-run over the seven-method port. A catalog verb reaches a handler through a declared row, and every
other verb still refuses by its bare name. The 94 substitutions have a value source, and each unmeasured one refuses
at `run` and is an `Error` at `doctor`. Nothing new walks the shipped catalog at `s8c`: `smoke`, `sca_scan` and
`package` steps refuse before the port until `s8d`'s resolver supplies their inputs.

**What this forbids**

- A wildcard or default arm in the route match, or a route added by editing the match rather than by record.
- A port method per catalog verb. `run_verb` is the one generic method.
- Using `run_verb` to apply a patch, or landing a `mutate` row.
- An output whose `kind` differs from its row's `outputs` reaching the kernel as a typed value.
- A `smoke` handler that runs an installed `redkiln`, or that leaves its scratch directory behind without an error.
- Counting an advisory with no CVSS as `low` or none, or dropping it.
- Counting an advisory whose vector could not be scored as `unrated`, or banding it by a guess.
- *(dated 2026-09-26, s8c build, CVSS; D21-64 ruling 7.)* Refusing, or counting `unrated`, a `CVSS:4.0` vector the
  pinned `cvss` crate's v4 scorer scores; scoring a v4.0 vector by any other scorer; and an in-tree v3 score that
  differs from `cvss::v3`'s on a vector of the differential fixture.
- Recording a `ScanReport` whose database commit the adapter did not read, before and after the tool, from the path
  it passed that tool; or reading the commit from deny's log line.
- A scan that fetches.
- Globbing the release directory for `package`'s digests.
- A `setup` that returns `Ok` over a failed install.
- Resolving an undeclared `checklist:`, `kb:` or `toolchain:<feed>` key to `null`, even when optional.
- A value in the per-toolchain table without a warrant, or a warrant whose `rule` is not decision 10's.
- Choosing or changing decision 10's rule after a reading it applies to has been seen, except by a new record that
  says so.
- Re-deriving a borrowed-class key from a median, or from a class other than its own.
- The kernel naming the adapter whose value table it was handed.

## Evidence and confidence

Measured 2026-09-26 at stage open (package P1), every tool run through `runbook/tools/nodispatch.sh`.

- **cargo audit 0.22.2 `--json`**: top-level members `database`, `lockfile`, `settings`, `vulnerabilities`,
  `warnings`; `database` = `{advisory-count, last-commit, last-updated}`. **Under `--no-fetch`, `last-commit` and
  `last-updated` are null** over the real database and the fixture database alike, and with `--stale`. After a fetch
  (on a copy of the database) `last-commit` is set. Per-advisory `cvss` is present (a vector string or null). Exit 1
  when vulnerabilities are found.
- **cargo deny 0.20.2**: the offline flag is the top-level `--offline` (also `--frozen`), and `check --disable-fetch`
  does not exist. `--format json check advisories --audit-compatible-output` prints one cargo-audit-shaped object per
  database on stdout with `lockfile`, `settings`, `vulnerabilities`, `warnings`, and **no database member**. The path
  appears only in a DEBUG log record on stderr, *"Opening advisory database at
  'C:\Users\ryanm\.cargo\advisory-dbs\advisory-db-3157b0e258782691'"*, never its commit. Deny refuses a database
  directory that is not a git checkout. Per-advisory `cvss` is present. Exit 1 on findings.
- **The two databases** at stage open: `~/.cargo/advisory-db` and `~/.cargo/advisory-dbs/advisory-db-3157b0e258782691`,
  both at `e2111519ba6d14a5da59a7b2e5c8083ae8a37c01` (2026-09-25T19:51:57+02:00). Both had moved from `ef8244d2`, the
  2026-09-24 reading, which is the move a per-tool commit makes visible.
- **CVSS scoring**: the reports carry only the vector string; no `cvss` crate is in `Cargo.lock` at `6f70e47`.
- **The fixture database** (`crates/rk-adapter-toolchain-rust/tests/fixtures/advisory-db/`): both tools report
  `RUSTSEC-2099-0001` (`CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H`) and `RUSTSEC-2099-0002` (`cvss` null) against
  `itoa 1.0.18`. Both match registry packages only.
- **The `setup` project set**: `git ls-files` finds one tracked `pnpm-lock.yaml`, under
  `crates/rk-adapter-toolchain-node/tests/fixtures/project`. Its install at stage open: rc 0, *"Already up to date"*.
- **ADR-0137 clause 6's scratch location**: `$(git rev-parse --git-common-dir)/redkiln/tmp` = `.git/redkiln/tmp`
  exists.
- **§8c.5's readings: none taken when this record was written.** Decision 10's rule is dated 2026-09-26 and precedes
  every one of them. The readings, the runner and its pinned version beside each child's times, the measurement path's
  length beside the length of `.redkiln/worktrees/<item>` for the lane's item, and pnpm's `downloaded` count are
  written below by dated edit when §8c.5's measurement runs. The whole-suite wall times P1 recorded while capturing the
  goldens (170.5 s under `cargo test`, 90.2 s under nextest, warm, in the main checkout) are not §8c.5 readings: they
  are neither cold nor taken in a fresh worktree, and no value is derived from them.
- *(dated 2026-09-26, s8c build, P8.)* **§8c.5's readings, n = 3**, each pass §8c.5's script extracted from the stage
  file's own bytes and run through `runbook/tools/nodispatch.sh` (nested in the session's no-dispatch helper, so
  `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`), pin `6f70e47786474cce0466369a6e0f5e282321af1c` (HEAD; the working tree's
  uncommitted s8c work is NOT in the measured tree, which is a fresh detached worktree at that commit), in
  `.git/redkiln/tmp/c<HHMMSS>`, each with its own empty `CARGO_TARGET_DIR`. Runner: `cargo-nextest 0.9.146 (8af696ddc
  2026-09-21)`. Seconds, whole-second resolution (`date +%s`):

  | pass | setup (downloaded) | `--list` cold | `--list --ignored` | nextest run | `--doc` companion | build cold | scan (audit + deny) |
  | --- | --- | --- | --- | --- | --- | --- | --- |
  | 1 (11:11:39Z) | 2 (0) | 93 | 3 | 146 | 2 | 58 | 1 (0 + 1) |
  | 2 (11:17:01Z) | 8 (0) | 88 | 3 | 142 | 3 | 58 | 1 (0 + 1) |
  | 3 (11:22:29Z) | 4 (0) | 88 | 3 | 142 | 3 | 56 | 1 (1 + 0) |

  pnpm's `downloaded` count read 0 on every pass, so **no reading is a network reading**. Both advisory databases read
  `e2111519ba6d14a5da59a7b2e5c8083ae8a37c01` on every pass. The build and scan children ran the argvs the port and the
  verb run (§8c.5's script, amended by dated edit before the first pass: `--message-format=json`; audit `--db`; deny
  under a generated `db-path` config with `-L debug --format json … --audit-compatible-output`). Every pass exited 0,
  the worktree path set was unchanged and `nd_check` passed.
  **Decision 10's rule applied** (written before these readings): `test_timeout` = 60 × ⌈2 × 146 / 60⌉ = **300**
  (the slowest child, pass 1's nextest run); `build_timeout` = 60 × ⌈2 × 58 / 60⌉ = **120**; `scan_timeout` = 60 × ⌈2 ×
  1 / 60⌉ = **60**; `setup_timeout` = 60 × ⌈2 × 8 / 60⌉ = **60**. The loader recomputes each and refuses a mismatch.
  **Path lengths**: the script prints `path_len=24`, the length of the RELATIVE `.git/redkiln/tmp/c111140` (because
  `git rev-parse --git-common-dir` answers `.git` from the main checkout); its absolute form,
  `D:/repos/redkiln-rs/.git/redkiln/tmp/c111140`, is 44 characters. A run worktree at `.redkiln/worktrees/<item>`
  for this box's one existing item name, `D:/repos/redkiln-rs/.redkiln/worktrees/rk-fix-doctor-test-count`, is 63; the
  lane's item is not yet named, so that length moves with its id.
  **Observed, not a timing:** `git worktree remove --force` removed each pass's worktree registration and every
  regular file, but left pnpm's `node_modules` skeleton behind (114 dangling junctions and 0 files on passes 2 and 3;
  pass 1's, removed before it was counted, held no regular file either), which the path-set check (`git worktree list`) does not see. They were removed by hand. This is a
  Windows run-worktree removal fact for `s8d-ii`, handed on in the stage file's §8c.12 note.
  **Shipped**: `.redkiln/registry.toml`'s `[toolchain.rust]` ~~carries these readings, each labelled with its pass and
  child, under `command` warrants at the pin above.~~ *(dated 2026-09-26, s8c build, re-measure; D21-64 ruling 8.)* carried these readings until the re-measure
  below replaced them; they stay in the registry as dated comments.
  *(dated 2026-09-26, s8c build, refuting round 2.)* **`scan_timeout` also bounds a clippy `lint` site.** `scan_timeout` is read at two sites that run no scan. `.redkiln/catalog/tech-debt-assessment.toml`'s step on
  `run = "toolchain:lint"` declares `timeout_secs = "${toolchain.scan_timeout}"` (one of the four catalog sites the
  §8c.5 table counts; the other three are `sca_scan` steps), and `.redkiln/registry.toml`'s `lint` row names
  `timeout = "scan_timeout"` as its construction bound. The Rust adapter's `lint` runs `cargo clippy --workspace
  --all-targets`. The warrant's readings timed `cargo audit` and `cargo deny` only (1 s, so 60 s under decision 10's
  rule); no reading timed clippy. **The value does not warrant the lint sites**: under record B's min(step,
  construction) that step's clippy child is bounded at 60 s by a reading of another command. No later stage file
  names `tech-debt-assessment` or a lint timeout (measured before this note: a grep of `runbook/` for
  `tech-debt-assessment` and `toolchain:lint` found neither). Both shipped packs declare the activity
  (`[activity.tech-debt-assessment]` in `.redkiln/packs/story.toml` and `bug-standard.toml`); whether a Phase-1 lane
  selects it is not measured here. Giving
  the lint sites their own key (for example a measured `lint_timeout`) or a warrant taken on clippy is handed on,
  unassigned, in the stage file's §8c.12 (item 5); it is not decided here, and the shipped value is not changed.
- *(dated 2026-09-26, s8c build, re-measure; D21-64 ruling 8.)* **§8c.5's readings re-taken at the tree holding all of `s8c`'s work, n = 3.**
  The operator took the re-measure (D21-64 ruling 8: *"`s8c`'s value table is re-measured at the stage's work commit,
  three passes, and re-derived by record V's rule (written before any reading), by dated edit."*). The work commit
  did not yet exist, so the measured tree is a commit object made from a temporary index over the working tree,
  `2f3f6ab257c4b21c1bd16bcd4781f97475f958f5` (parent `366543a`, tree `4ba5d20f78ee72b095d0cd20a8444c11155dbc39`: every
  `s8c` package through `366543a`, plus the uncommitted leak fix of ruling 3 and the `cvss` crate of ruling 7). It is
  on no ref. §8c.5's script was extracted from the stage file's bytes with one substitution, its pin line (`git
  rev-parse HEAD` → that commit), and otherwise byte-identical to P8's extraction; each pass ran through the session's
  no-dispatch helper and `runbook/tools/nodispatch.sh` (`CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`), in
  `.git/redkiln/tmp/c<HHMMSS>` (`path_len=24`, as before), each with its own empty `CARGO_TARGET_DIR`. Runner:
  `cargo-nextest 0.9.146 (8af696ddc 2026-09-21)`. Seconds:

  | pass | setup (downloaded) | `--list` cold | `--list --ignored` | nextest run | `--doc` companion | build cold | scan (audit + deny) |
  | --- | --- | --- | --- | --- | --- | --- | --- |
  | 1 (15:05:21Z) | 1 (0) | 99 | 3 | 152 | 3 | 57 | 2 (1 + 1) |
  | 2 (15:11:03Z) | 2 (0) | 86 | 3 | 152 | 3 | 58 | 1 (0 + 1) |
  | 3 (15:16:40Z) | 4 (0) | 87 | 3 | 161 | 3 | 57 | 2 (1 + 1) |

  pnpm's `downloaded` count read 0 on every pass, so no reading is a network reading. Both advisory databases read
  `e2111519ba6d14a5da59a7b2e5c8083ae8a37c01` on every pass. Every pass exited 0, the worktree path set was unchanged
  and `nd_check` passed. **Decision 10's rule applied** (unchanged, written before any reading): `test_timeout` = 60 ×
  ⌈2 × 161 / 60⌉ = **360** (the slowest child, pass 3's nextest run; ~~300~~ at `6f70e47`); `build_timeout` = 60 ×
  ⌈2 × 58 / 60⌉ = **120**, `scan_timeout` = 60 × ⌈2 × 2 / 60⌉ = **60** and `setup_timeout` = 60 × ⌈2 × 4 / 60⌉ =
  **60**, each unchanged. `.redkiln/registry.toml`'s four measured rows now carry these readings at this pin, with the
  `6f70e47` readings kept as dated comments above them. The cited keys are not re-taken (`review_timeout` 9180, and
  X4's 4800 for `implement_timeout` and `repro_timeout`, keep their pin). **Two facts said, not hidden:** the pin is
  unreachable from any ref, so it resolves (Checkpoint 8c (11)'s `git cat-file -e`) only until a prune removes it,
  unless the commit that lands this work adopts it or the value is re-pinned by dated edit; and each pass again left
  its pnpm `node_modules` skeleton (114 dangling junctions, 0 files, 66 directories) under `.git/redkiln/tmp/`, removed
  by hand after the three passes.
- *(dated 2026-09-26, s8c build, CPREP.)* **The build's readings, stated here as proposed text.** Each was first
  written in a dated note of `runbook/phase-1/s8c-toolchain-port-and-verbs.md` (§8c.4, §8c.5) by the package that
  built it; this record now carries it, so the flip reads the record and not the stage file.
  - **Which scorer landed (decision 5 asks the builder to record it).** In-tree code, not the `cvss` crate:
    `verbs::severity_of` in `crates/rk-adapter-toolchain-rust/src/verbs.rs` computes the CVSS v3.1 base equations and
    `Roundup`, and scores v3.0 vectors by the same equations. ~~No dependency was added, `Cargo.lock` did not move, and
    no `deny.toml` licence question arises.~~ How a vector this scorer cannot score is treated is the subject of a
    separate ruling and is not recorded by this line.
  - *(dated 2026-09-26, s8c build, CVSS; D21-64 ruling 7.)* **Both, now: the in-tree code for v3.x and the `cvss`
    crate for v4.0.** The in-tree v3 code is now `verbs::cvss3_tenths` (made `pub` for the differential test), and
    `verbs::severity_of` sends a `CVSS:4.0/` vector to `cvss::v4::Vector::from_str(..).score().value()`. The crate's
    feature names at 2.2.0 are exactly `std`, `v3` and `v4` (its defaults are the same three; `serde` is its one
    optional dependency and stays off). `Cargo.lock` gains one package, `cvss 2.2.0` (checksum
    `f7fb220d3ce1b565af39cee5b89e47fd8dd1dab162900ee4363c8ee4169ee8a2`), with no dependencies of its own, and one
    edge from `rk-adapter-toolchain-rust`. **The deny readings** (cargo-deny 0.20.2, cargo-audit 0.22.2, through
    `runbook/tools/nodispatch.sh`, after the dependency landed): `cargo deny check licenses` rc 0, *"licenses ok"*,
    listing `cvss@2.2.0 (2): Apache-2.0, MIT`; its one warning is `license-not-encountered` for `MIT-0`, an
    allowance `cvss` cannot affect. `cargo deny check bans` rc 0, *"bans ok"*, 8 `duplicate` warnings (none
    names `cvss`, which has no dependencies). Checkpoint 5's advisories halves, run offline so neither database
    moved: `cargo deny --offline check advisories` rc 0, *"advisories ok"*; `cargo audit --no-fetch --deny
    unmaintained` rc 0 (one allowed `unsound` warning, RUSTSEC-2026-0306). Checkpoint 5's must-fail line,
    `cargo deny --all-features check bans`, still exits non-zero naming `banned` (rc 2).
  - *(dated 2026-09-26, s8c build, CVSS.)* **The v4.0 readings.** CVE-2025-47273's vector (the `cvss` crate's
    own regression vector, carrying `E:P` and every other metric `X`) scores **7.7, CVSS-BT**; the same vector with
    its threat and environmental metrics removed scores **8.7, CVSS-B** [measured 2026-09-26 by a probe inside the
    test binary, then removed]. Both band `high`. Of the RustSec database's 63 `CVSS:4.0` vectors, **2 carry a
    threat metric** (`E:U` and `E:P`) and the other 61 carry base metrics only, so for 61 the crate's score is the
    base score. **Flagged, not decided here:** record V says *"the CVSS base score"*, and D21-64 ruling 7 fixed
    the crate's score of the vector as written (7.7 for CVE-2025-47273). The two can differ in band only for a
    vector carrying non-base metrics; a threat metric can only lower the score (FIRST v4.0 scores `E:X` as `E:A`,
    the worst case), so scoring as written can band such a vector lower. Scoring base-only is one filter over the
    vector's metrics before the parse.
  - *(dated 2026-09-26, s8c build, CVSS.)* **The differential fixture**:
    `crates/rk-adapter-toolchain-rust/tests/fixtures/cvss/v3-vectors.txt`, the 92 distinct `CVSS:3.0`/`CVSS:3.1`
    vectors in `~/.cargo/advisory-db` at `e2111519ba6d14a5da59a7b2e5c8083ae8a37c01` (335 v3.1 and 33 v3.0
    lines). The in-tree scorer and `cvss::v3` agree on all 92, in tenths.
  - **Refusal (b) is covered by a comparator unit test, not end to end.** Decision 5 permits it. No seam that moves
    `HEAD` between the adapter's two reads was added, so `verbs::sca_scan_refuses_a_database_moved_mid_scan` asserts
    the comparator `verbs::db_unmoved` only, and the end-to-end mid-scan move is not claimed. Refusal (a) is
    `verbs::sca_scan_refuses_a_non_git_database` and refusal (d) is
    `verbs::sca_scan_refuses_a_deny_db_path_with_other_than_one_database` (two children, and none). The three are
    among the tests Checkpoint 8c (7) names.
  - **The input names the handlers read**, which are the shipped rows' `inputs`: `smoke` reads `checklist`, a list
    of items each `{ id, argv, cwd, expect_exit, stdout_contains }`, the one stdout predicate being *the row's stdout
    contains this text*, and `cwd` admits `scratch` only; `sca_scan` reads `audit_db` (the database audit is given by
    `--db`) and `deny_db` (the parent directory deny's generated `db-path` names, which must hold exactly one child),
    each an absolute path, because the adapter reads no environment (AC-217) and the default locations are the
    resolver's to supply (`s8d`); `package` reads `sha` and `binaries` (a non-empty list of target names); `setup`
    reads `projects` (paths relative to the project root; an absolute path, or one with `..`, is refused).
  - **`BuildArtifact`'s binary paths are relative to the adapter's cargo target directory** (for example
    `release/<name>`), not to the project root. `verbs::package_digests_only_the_declared_binaries_of_a_locked_release_build`
    asserts that reading, and `rk_types::verb_output::ArtifactBinary`'s doc, which said *"relative to the project
    root"*, was corrected to it (CPREP). ~~**Flagged, not decided:** the handler's `strip_prefix(&target).unwrap_or(exe)`
    records the absolute path, silently, for an executable outside that directory; cargo does not produce one under the
    adapter's own target directory, and no test reaches that arm.~~ *(dated 2026-09-26, s8c build, D63.)* **Fixed:**
    that fail-open is gone. `verbs::binary_path` refuses a binary outside the target directory by name,
    `package-binary-outside-target`, naming both paths, and never records an absolute path.
    `verbs::a_package_binary_outside_the_target_dir_is_refused_by_name` asserts it over `verbs::binary_path` only, and
    says so: cargo produces no executable outside its own `CARGO_TARGET_DIR` and no seam makes it, so the end-to-end
    case is not claimed (as with refusal (b)). A mutant restoring `unwrap_or(exe)` was killed by that test.
  - **The decode table's interim verdicts.** Until `s8d` binds step outputs, the decoded value's only channel is the
    step's `Result`. So a `failing` `BuildReport`, and a `SmokeResult` whose `status` is not `passing` (`failing` or
    `empty`), fail the step as `NonZero { code: None }` carrying the report's digest; the other five kinds are typed
    and dropped. No record names this reading; it is proposed here.
  - **No registry means a bare-name refusal.** `runtime::step::ToolRouting { registry, values }` is bound by
    `Runtime::with_tool_routing` and threaded by `replay::resume_routed`; `Runtime::new` and `replay::resume` bind
    none. A runtime without a registry refuses every verb, the four `port` verbs included, by its bare name: no
    built-in table stands behind the registry. In `rk-cli`, an absent `<worktree>/.redkiln/registry.toml` binds none,
    and a registry that does not load exits `ConfigError` naming it.
  - **The loader recomputes decision 10's rule on every `command` value** (`60 × ⌈2 × max(measured) / 60⌉`) and
    refuses a `secs` that differs, because the record forbids *"a warrant whose `rule` is not decision 10's"*. A
    `source` (cited) value is not recomputed, because X4's 4,800 s is the operator's value and not the rule's output.
  - **Declaration rows are exactly `{ value, source }`**, admitted for the three named rows `setup_projects`,
    `package_binaries` and `claims` only, and they are not in `ToolchainValues`. The shipped `setup_projects` is
    written in that shape.
  - **`release_tag`'s row choices.** It has no spike row, so its `timeout = "build_timeout"` (the key
    `spikes/p9/registry.toml` gives `release_record` and `verdict_record`) and `live = false` were the builder's
    choices; the `store` and `git` rows declare `inputs = []`, the names being their builders'. `smoke`'s row timeout
    is the spike's `deploy_timeout`, `unmeasured` on both toolchains, and every shipped `smoke` step carries a literal
    `timeout_secs = 600`, so no shipped step resolves the row's key.
  - **Two `rk-git` additions the `.worktreeinclude` reader needed (D21-45 W3, this record's decision 7 source), named
    here as proposed text because no record names them.** `GitError` gains `CarryRefused { refusal }`
    (`is_precondition()` is `true`); the enum's doc reads *"Closed; extended only by a successor record."*
    `GitExec::run_with_stdin` writes a stdin to the child (`check-ignore --stdin` needs one), and the workspace's one
    `std::process::Command::new("git")` literal moved from `run_with_env`'s body into a private `spawn`. The two read
    variants `GitOp::LsFiles` and `GitOp::CheckIgnore` are reads, and ADR-0105 clause 5's write set is untouched; that
    enum's doc reads *"Anything not here is a successor record, and adding a variant is that decision made visible."*
    As proposed, this record is the successor record both docs name, for these four and nothing else.
  - **Where and how §8c.5's values were measured.** At `6f70e47`, without the stage's uncommitted work (about eighty
    tests, several of which build fixture projects), in a fresh detached worktree, with the no-dispatch helper's
    `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`, which is slower than cargo's default and so errs long. The rule's factor of
    two covers the gap by construction, and the gap is not measured. **Whether to re-measure at the work commit is the
    operator's call**; a re-reading replaces the four values by dated record edit under decision 10's rule.
    *(dated 2026-09-26, s8c build, re-measure; D21-64 ruling 8.)* Taken: the re-reading at `2f3f6ab` (the evidence item above) found the
    slowest child 161 s against 146 s, so `test_timeout` moves 300 → 360; the other three values are unchanged.

**Confidence.** High on the route shape, the seventh method and the refusal order, which D21-42 1b, N14 and N25 force.
Medium on decision 5's provisional hardening (D21-62 C): it keeps every binding (the per-tool commit, offline, refuse
on an unknown or moving database) and changes only where the commit comes from, and its deny path check is a
per-version pin. The rule in decision 10 is a proposal the operator may replace by record before its readings are
taken; after they are taken, replacing it is itself a record that says why.
