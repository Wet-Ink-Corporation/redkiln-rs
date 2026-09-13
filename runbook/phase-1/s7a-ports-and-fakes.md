# Step 7a — `rk-ports` and the fakes (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7a` | Phase 1 | Checkpoint 6 | Checkpoint 7a | `s6` | `s7b` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


> ### 🚦 GATE CHECK
> ```bash
> ( ls D:/repos/redkiln-rs/references/adr/0090-*.md >/dev/null 2>&1 \
>     || { echo "GATE OPEN — no ADR-0090. Go back to Step 4; no work in this stage may start."; exit 1; }
>   echo "GATE DISCHARGED — proceed" )   # a subshell: a pasted FAIL must not kill an interactive shell
> ```
> **`0090`, not `0089`** — see Step 5's banner for why the token moved.
>
> **Golden constraint 1, the LLM-facet block, restated here** because `00-preamble.md`:194 names a Step 7a banner
> and until 2026-09-13 this file was the only stage in that row that carried none. ADR-0079 is `proposed` and
> **forbids being amended into a decision**; ADR-0090 supersedes it in full and is the gate's token. **Nothing in
> this step touches the selection path** — `rk-ports` declares five traits and their fakes, keys no `require` row,
> and reads no `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro` —
> so a discharged gate is a **formality here, which is exactly why it is checked here**, one stage before 7b, where
> it is not (`s5`'s own reasoning, and `s4`:621: *"Run that line at the head of Steps 5, 6, 7a and 7b"*).

Step 7 is four sub-steps — 7a ports, 7b kernel, 7c adapters, 7d CLI — followed by 7e pack authoring, **strictly in
this order**. Each sub-step's checkpoint is its own; do not batch them.

### 7a.1 What this step builds, and what it may not decide

Five named ports — event store, harness, tracker, docs site, toolchain (**ADR-0003**:34, *"The kernel exposes
exactly five ports, each with one real adapter named for Phase 1 and one fake"*; the five-port inventory, the
fake-per-port rule and git-not-a-port all stand after ADR-0094's partial supersession) — each a **sync** trait with
one fake, and a real adapter where Phase 1 ships one: SQLite, Claude Code, Node **and Rust** for the toolchain
(ADR-0094) — four reals across three ports; `Tracker` and `DocsSite` have a fake only until Phase 3, and AC-499's
matrix carries the phase clause. **Git is deliberately not a port** (**ADR-0003**:44, :57-58 — *"No sixth port may
be added without a named substitute **or a fake-only justification equivalent to EventStore's (Principle 9)**"*,
*"Git may not be abstracted behind a port"*; **the disjunct is quoted because dropping it states a stricter rule than
the record** — it is the half AC-482's evidence line turns on (*"a port exists where a substitute has been named, or
where the fake alone pays for it"*, register :6945) and the half ADR-0003:38's `EventStore` row — real adapter
*"none planned"* — relies on to be legal); it is a concrete crate the
kernel names (AC-483, whose own `decisions` line reads ADR-0003 + ADR-0105 decision 1), and its `GitOps` seam is a
struct (ADR-0105 clause 1), so the port-inventory denylist stands. **§7a.7 builds that denylist rather than
asserting that it stands.**

Port traits are sync because native `async fn` in trait is **still not dyn-compatible** and `async_fn_in_dyn_trait`
remains nightly-only through 1.98.1 (R-ASYNC, discharged 2026-09-10): "no generics leak into the core" and
"dyn-dispatched port" are in direct tension the moment a port method is async. Every `.await` lives in an adapter.

**What this step may not decide.** `00-preamble.md`:53 — *"Where a stage is tempted to decide something else, it
stops."* Two things this file used to leave open have since been decided **in records**, and are executed here, not
re-opened: lock poisoning (**ADR-0118**, written 2026-09-13 — §7a.4) and AC-481's derivation mechanism (the
register's own dated amendment of 2026-09-13 — §7a.6). Three things are **not** decided by any record and are
**routed with a stated warrant and an owed record named** — the crate homes of `Typed`, `SchemaId`,
`DispatchOutcome` and `EventStore::append`'s parameter (§7a.5). A routing is an execution consequence and is
reversible by the record that is owed; it is not a decision, and it is written down as such.

### 7a.2 The port shape — ADR-0099, and ADR-0115's narrowing

**The port shape is ADR-0099's, and it compiled before this step started (F41, S-PORTS, 2026-09-11):**

- **The Harness signature is ADR-0099 clause 1** (`references/adr/0099-...`:50-55), restated in design §3.3's
  Harness cell (design:266) and as port rule P10 (design:288):
  `Harness::dispatch(&self, &ActivitySpec, &mut dyn EventStore) -> DispatchOutcome { record: DispatchRecord,
  result: Result<Typed, StepError> }` **blocks to completion**; the adapter's declared timeout (AC-223) is the
  in-band cancel; **cancellation is a `CancelToken`** cloned into the adapter at construction and triggered by the
  composition root's signal path or the per-activity timeout — **never by the kernel**, and never a handle the
  kernel holds. Only the Harness port carries it in Phase 1. **Do not start from ADR-0076's *"a handle minted when
  the operation started"* wording** (ADR-0076:54): ADR-0099 supersedes ADR-0076 **in part on exactly that clause**
  (ADR-0099:31, *"the cancel clause and the evidence line on prior art"*; the README's supersession table records
  the withdrawal by name at README:232) and rejects `start() -> Handle` / `wait(Handle)` at ADR-0099:95-96.
  **Nothing in this step's instrument set catches the withdrawn shape** — neither fence in §7a.8 is about a cancel
  handle, and the exactly-2 fence count in Checkpoint 7a passes a crate carrying it. It is caught by reading the
  trait against ADR-0099 clause 1, and that is stated here rather than left to a checkpoint that cannot do it.
- **Every port trait carries `Send + Sync` as supertraits, and the composition root holds each as
  `Arc<dyn Port + Send + Sync>`** — a fake that holds an `Rc` does not compile, which is F41's first
  `compile_fail` doctest. **F41's second doctest is not carried over; §7a.8 says what replaces it and why.**
- **The EventStore port is the one exception, and it is a record, not a workaround: `Arc<Mutex<dyn EventStore>>`**
  (**ADR-0115** clause 1, :57-58, narrowing ADR-0099 clause 4). An `Arc<dyn T>` yields `&T` and never `&mut T`, so
  a bare `Arc` cannot produce the `&mut dyn EventStore` clause 1's `dispatch` takes — the two clauses were not
  jointly satisfiable, and F41 compiled this shape. What clause 4 still forbids is a **user-defined** wrapper
  between the `Arc` and the trait object (`struct StoreHandle(Arc<dyn EventStore>)` and every shape like it): the
  test is whether the type adds a second dispatch or a seam a fake must be swapped at, and `Mutex` adds neither
  (ADR-0115 clause 3, :73-76). The lock is taken by the composition root at the call boundary and released when
  the dispatch returns; **no port method acquires it, and it is never held across a dispatch to another port**
  (clause 4, :78-80).
- **ADR-0115 clause 5 scopes the seam** (:82-83): *"The `Mutex` is the one permitted interior-mutability seam in
  the port set. A second port that later needs `&mut` takes a successor to this record, not a local decision"* —
  with the forbids bullet at :126. **The scope is the *holding* seam — how the composition root holds a port — and
  not a fake's own internal state.** The reference shape's `FakeHarness` holds `script: Mutex<Vec<Result<Typed,
  StepError>>>` (`spikes/s-ports/src/lib.rs`:184) and locks it at :202; that is not a second seam, and an operator
  who meets it must not read clause 5 as forbidding it (golden constraint 9: establish the guard's scope).

**`Ports` — the struct that holds all five — is `rk-ports`', and the warrant is stated so an operator who
disagrees routes rather than silently re-homes it.** ADR-0115:63-71 gives the struct and names no crate;
design:1704's `rk-ports` row enumerates this crate's contents (*"the five port traits, `Send + Sync` supertraits,
held as `Arc<dyn Port + Send + Sync>` (the **EventStore** port is `Arc<Mutex<dyn EventStore>>` … **ADR-0115**,
narrowing ADR-0099 clause 4); one fake each; the `CancelToken`; the `ChangeSummary` newtype"*) and does **not**
list `Ports`; design:1707 makes `rk-cli` the composition root that **constructs** one, at Step 7d. The type must
exist before its only construction site. It is therefore authored here — the reference shape already puts it in
the port crate (`spikes/s-ports/src/lib.rs`:173) — **or ADR-0115's narrowing ships unexercised for three
sub-steps.** This is a routing, not a decision; a record naming the home is owed.

**Two downstream restatements of clause 4's un-narrowed form survive, and this stage states the discrepancy rather
than resolving it (ADR-0092 precedence).** Design §3.3 P10 still ends *"every port is held as `Arc<dyn Port + Send
+ Sync>` … a bare trait object and not a newtype"* (design:288) with no ADR-0115 carve-out, and ADR-0105 clause 11
still reads *"Ports are `Arc<dyn Port + Send + Sync>` (ADR-0099)"* at :102-103 as a `## Decision` sentence — a sourced
restatement of the clause ADR-0115 narrowed, not an independent decision against it. ADR-0115 carries
`supersedes: ["0099"]` only and names §3.3 P10 solely inside a **rejected** option. **The design's other three loci
and the register do carry the narrowing** — design §3.9 (:409), §9.1's `rk-ports` row (:1704), §9.7 (:1823) and
AC-480's AND clause (register :6912) — so §3.3 P10 and ADR-0105 clause 11 are the two stale ones. Step 7a builds
ADR-0115's shape because it is the later record and the only one that compiles [measured, F41; ADR-0115:130-132];
**a dated amendment at design:288 is owed, and which mechanism corrects each locus is ADR-0092's call, not this
file's.** *(DECISIONS-2026-09-12 E2 ordered the narrowing mirrored at design §3.3 P10, §9.1 and AC-497; §9.1
landed, P10 did not, and the register narrowing landed in **AC-480** rather than the **AC-497** E2 named — AC-497
(register :7106) carries none. Two of E2's three named sites are unexecuted as named. Recorded here, not resolved
here.)*

**An operator who follows a citation to P10 rather than reading this section writes `Arc<dyn EventStore>` and
cannot produce the `&mut` clause 1 needs.** That is why §7a.8's second `compile_fail` fence is the bare-`Arc` form.

### 7a.3 The five ports' method sets, and the five fakes

**Each trait's method set is design §3.3's responsibility column (design:265-269), not the spike's.**
`spikes/s-ports/` is a **shape check, not an interface**: it matches design §3.3 on the `Harness` row only, and on
the other four it is wrong [measured 2026-09-13, both files read side by side]. Write the method **names** from
design:265-269:

| Port | Method set — design §3.3's own words | Required fake | What the spike has instead |
| --- | --- | --- | --- |
| `EventStore` | *"append; query by item / SHA / run; snapshot; rebuild"* (design:265) | **in-memory `Vec`** | `append` + `read` (`spikes/s-ports/src/lib.rs`:145-146) — no query, snapshot or rebuild |
| `Harness` | *"`dispatch(&ActivitySpec, &mut dyn EventStore) -> DispatchOutcome { record, result }` — blocks to completion"* (design:266); ADR-0099 clause 1 is the Rust | **scripted responder** | matches (lib.rs:149-152) — the **only** row that does |
| `Tracker` | *"`project(item) -> ExternalRef`; `inbox() -> Vec<TypedInput>`; idempotent writes"* (design:267) | **in-memory board** | `sync(&self, item: &str)` (lib.rs:155) |
| `DocsSite` | *"`build(DocSet)`, `check_links`, `drift(changed_sources)`"* (design:268) | **no-op recorder** | `publish(path, body)` (lib.rs:159) |
| `Toolchain` | *"detect; `build`; `test(selection)`; `lint`; `classify(path) -> Text/Binary/Generated`; `change_summary(range) -> ChangeSummary`"* (design:269) | **fixture-driven** | build/test/classify/change_summary, no `detect`, no `lint`, and `classify(&self, paths: &[String]) -> Vec<String>` (lib.rs:165-170) where design:269 and AC-497's header (register :7106) both fix `Text/Binary/Generated` |

**Checkpoint 7a's trait and impl counts do not reach a method**, so an implementer told only "start from the spike"
ships four traits that contradict the design. That is why the table above is in the stage file and not left to the
spike — and why Checkpoint 7a now carries a **method floor** (the group after the supertrait count) that requires the
six design-named methods the spike does not have and refuses the two spike methods design:267-268 replaced. The floor
pins **names**, never a signature: a checkpoint that pinned a signature would read as a decision, and no record fixes
one for the four non-Harness ports.

**The table fixes the method NAMES; everything below the names is a ROUTING, not a decision, and is reversible by
the record that is owed** (§7a.1's rule). Design §3.3's Responsibility column is prose for all five rows and
ADR-0099:50-55 is the only Rust signature in any record [measured 2026-09-13], so an implementer handed only the
names invents parameter types, return types and an error type, and Checkpoint 7a goes green over whatever they
invent. The routing this stage takes, written down so a later record can reverse it:

- **Every fallible port method returns `Result<_, StepError>`**, `rk-types`' eleven-variant enum. `StepError` is the
  one error type any record puts across a port boundary (ADR-0099:57, *"the `result` is the typed output or a
  `StepError` from ADR-0095's table"*).
- **`EventStore`'s query trio is one method with a selector**, because design:265 (*"query by item / SHA / run"*)
  fixes neither the arity nor the key types: `fn query(&self, by: Query) -> Result<Vec<Event>, StepError>` with
  `Query` an `rk-ports` enum of three variants. `snapshot`/`rebuild` take an opaque `Snapshot`.
- **`DocsSite::build` takes `&DocSet` and `drift` returns `rk-types`' `Drift`.** `Drift` is on the tree
  (`crates/rk-types/src/drift.rs`); **`DocSet` is `rk-types`' by design:1699 and is NOT on the tree** — `grep -rn
  'DocSet' crates/` returns only `rk-types/src/lib.rs`'s own doc header [measured 2026-09-13] — so 7a authors it
  there as an opaque newtype on the warrant below, and a record fixing its contents is **OWED**.
- **`Toolchain::classify` returns a named three-variant enum**, `pub enum Classification { Text, Binary, Generated }`,
  in **`rk-ports`**. It is named and homed here because `toolchain_fake_classifies_binary` has nothing to assert
  otherwise. **Routed, not decided — no record assigns it**; the warrant is that `Binary` is the discriminant
  AC-497's port shape turns on (register :7109) and that its only consumer before 7c is the port's own fake, while
  `rk-types`' *"typed artifacts"* row (design:1699) does not name it. **A record naming the home is OWED**, and the
  checkpoint's re-declaration ban deliberately does not cover it.
- **What no record fixes and this stage does not pretend it does:** `Query`'s key types, `Snapshot`, `test`'s
  selection type, `change_summary`'s range type, and whether `detect` returns a bool or a version. Each is a
  routing. **Checkpoint 7a asserts none of them**, deliberately.

**The Toolchain port's `ChangeSummary` is an opaque, non-finalised newtype authored in `rk-ports` at this step** —
`pub struct ChangeSummary(pub(crate) ());`, the spelling `spikes/s-ports/src/lib.rs`:163 and F41:20 carry — until
S-BIN reports (**ADR-0099 clause 5**, :90-91). **It is the one site in `crates/` the name may appear** (ADR-0099's
forbids bullet at :131: *"Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until
S-BIN reports"*), and **Checkpoint 5's grep for its absence excludes exactly this crate** (ADR-0099:90-91, *"Checkpoint
5's grep for its absence covers `crates/` minus that one site"*; `crates/rk-types/src/lib.rs`:9-12 states the Step-5
side of it, and an implementer who reads only that will believe declaring the type is a gate violation). The rule is
about where the **type** lives, not about one textual occurrence: the prescribed shape necessarily carries the name
on `change_summary`'s return type and on the fake's impl too, so the checkpoint counts the **declaration** and not
the string. **Its `change_summary` BODY is not written here** — that is 7c's, behind S-BIN (§7a.11; ADR-0093 decision
4) — so the Phase-1 fake returns `ChangeSummary(())` and the trait method is declared and unimplemented by any real
adapter at this stage.

**`ExternalRef` and `TypedInput` are assigned to no crate by any record** — design:267 is their only occurrence in
this corpus and design:1699's `rk-types` row lists `DocSet` and neither of them [measured 2026-09-13]. **They are not
a halt, because a halt on them takes the other four ports with it**: Checkpoint 7a requires five trait declarations
and five fakes, so `Tracker` must exist. They are **routed to `rk-ports`, provisionally and by name**, on the
narrowest warrant available: `Tracker` and `DocsSite` are fake-only until Phase 3 (design:267-268; AC-499's GIVEN,
register :7133-7135), so no adapter and no kernel consumer of either type exists before Phase 3, and a type nothing
outside this crate names is the crate's own until a record says otherwise. Each is authored as an **opaque newtype in
the `ChangeSummary` shape** — `pub struct ExternalRef(pub(crate) ());`, `pub struct TypedInput(pub(crate) ());` —
deliberately: an opaque newtype declares the port shape without inventing a Phase-3 schema, which is the treatment
ADR-0099 clause 5 gives the one other blocked type. **No field set is invented and no idempotency key is designed**
(that is AC-508's, Phase 3). **A record naming their home and their contents is OWED** before the GitHub adapter is
written. **The cost, not smoothed:** four opaque newtypes then stand in `rk-ports` — `ChangeSummary`, `ExternalRef`,
`TypedInput` and `Snapshot` — and only the first has a record behind it. *(The previous revision of this paragraph
said to halt on these two. That instruction and the checkpoint's five-trait count were jointly unsatisfiable, and
this is the repair.)*

**The five fakes are design §3.3's Required-fake column**, and AC-481's evidence line (register :6917) names the
same five verbatim. **The reference shape ships two of them** — `VecStore` for `EventStore` (lib.rs:221-222) and
`FakeHarness` for `Harness` (lib.rs:183, :195); its own header says *"one scripted fake"* (lib.rs:4). `Tracker`,
`DocsSite` and `Toolchain` are implemented only by the test-local `struct T` / `struct D` / `struct C` inside
`#[cfg(test)] mod tests` (lib.rs:325-327). **Those are stubs, not fakes, and three fakes are Step 7a's to author.**
All five are `pub` library items, because a `#[cfg(test)]` item is invisible to a source parse that runs as a test
of the crate under its own manifest — see §7a.6.

**What else the reference shape does not hand you.** `spikes/s-ports/` declares its own `StepError` and `spoke()`
under a `// ---- rk-types (the shapes ADR-0099 names)` header (lib.rs:40; the enum at :54, `spoke()` at :77)
**because it is a throwaway single crate**. `rk-types` now owns both (`crates/rk-types/src/error.rs`) at eleven
variants pinned by `step_error_is_total` at Step 5 (`s5` §5.3; ADR-0095 as ADR-0112 extends it). Delete the spike's
copies and import them — **and the payloads differ, so every fake's error construction changes with them**
[measured 2026-09-13, both files read side by side]: `NonZero` takes `code: Option<i32>` and a `Digest` stderr digest
(`error.rs`:72-75) where the spike takes `i32` and `String` (lib.rs:59); `RateLimited` takes `Option<Timestamp>`
(`error.rs`:76-78) where the spike takes `Option<u64>` (lib.rs:60); `NotIsolated` takes a `Digest` diff digest
(`error.rs`:83-86) where the spike takes `String` (lib.rs:62). `TransportKind`, `CancelledBy` and `IsolationSurface`
are `rk-types`' too (`error.rs`:13, :21, :33), so the spike's three copies (lib.rs:69, :71, :73) go with the enum,
and `Digest` / `Timestamp` come from `crates/rk-types/src/digest.rs`:23 and `time.rs`:15. `rk-types` also carries
`retry_class()` (:129) and `event()` (:154), which the spike has not. **Re-implementing `spoke()` in `rk-ports` is
forbidden twice over:** the spike's `matches!` form expands to an implicit catch-all that would silently accept a
twelfth variant, and the arm-per-variant form trips `clippy::match_same_arms` — a **pedantic** lint, denied
**individually** at `Cargo.toml`:65 because ADR-0102 decision 5 deliberately does *not* deny the pedantic group (the
enumerated subset at `Cargo.toml`:59-67), so the `all = { level = "deny", priority = -1 }` entry at :58 does not
reach it. *(§7a.6's Trap 2 is the parallel case the other way: `disallowed_types` is in `clippy::style` ⊂
`clippy::all`, so :58 does carry it.)*

**Two of F41's seven artefacts do not transfer.** `five_ports_are_object_safe_and_send_sync` (lib.rs:265) and
`ports_struct_holds_arc_dyn_send_sync` (lib.rs:324) carry over unchanged, and so does the first `compile_fail`
doctest. The other three unit tests — `dispatch_blocks_and_returns_record_and_result` (:276, kernel at :279),
`cancel_token_is_the_roots_and_yields_cancelled` (:287, kernel at :291) and
`step_error_is_matched_exhaustively_with_one_event_arm_each` (:299, kernel at :317) — and F41's second doctest all
construct `kernel::Kernel`, the spike's own `pub mod kernel` at lib.rs:229, which `rk-ports` may not contain
(design:1704 enumerates this crate's contents and names no kernel module). **Re-home each against
`Harness::dispatch` directly**, calling the port on a fake instead of through a kernel, or carry it to Checkpoint 7b
and record the deferral. **Nothing here may add a kernel module to `rk-ports` to make a spike test compile.**

**What F41's headline attests, and what it does not.** F41:1 reads *"5/5 unit tests and 2/2 doctests on rustc
1.97.1 and 1.98.1, clippy clean"* and that restatement is faithful to the record that governs restated numbers.
But F41's own Correction (F41:70-75) extended the crate to the **eleven**-variant `StepError` — which is the crate
on this tree (`spikes/s-ports/src/lib.rs`:52) — and **re-ran it on 1.98.1 only**; clippy was likewise scoped to the
pin (`cargo +1.98.1 clippy --all-targets -- -D clippy::all`, F41:7-8). So the pin is attested for the code you start
from and **1.97.1 is not**. That is all Step 7a needs, since 1.98.1 is the pin (DECISIONS-2026-09-11 #7), and a
dated correction to F41:1's toolchain scope is **owed** (golden constraint 8). *(design:1704 and design:1953 also
credit S-PORTS with a `cargo tree -p rk-kernel -e normal` check — *"(S-PORTS checks it on 1.98.1)"*, status *"DONE
2026-09-11 — F41"*. **F41 ran no `cargo tree`**: `grep -rn 'cargo tree' spikes/F41-DRAFT.md spikes/s-ports/`
returns nothing, and F41:6-8's Established-by names only `cargo test` on two toolchains and `cargo +1.98.1 clippy`
[measured 2026-09-13]. A dated correction at design:1704 and :1953, or against F41's status, is **owed** and is
recorded here, not resolved here.)*

**F41 is `exclude`d from this workspace** (`Cargo.toml`:18), so its clippy-clean result and its formatting are
**not this workspace's gates**: `cargo fmt --check` inside `spikes/s-ports` prints **15** `Diff in …` hunks
[measured 2026-09-13]. Reformat and re-lint everything you lift, or Checkpoint 7a's `fmt` gate fails on contact.

### 7a.4 Lock poisoning — decided, in ADR-0118

**ADR-0115:135-138 named lock poisoning as a residual and assigned it to Step 7a. It is now closed, by a record:
`references/adr/0118-a-poisoned-store-lock-halts-the-run.md`, accepted 2026-09-13.** This step executes that
record; it does not re-open it.

- **The decision (ADR-0118 clause 1): a poisoned store lock halts the run.** The run stops at the poisoned
  acquisition. **No `StepError` is constructed, nothing crosses a port boundary, no retry, no next activity, no
  gate answer.**
- **The warrant, tagged honestly.** ADR-0105 clause 7 (:78-79) puts one writer connection per process behind a
  mutex with no queue, so a poisoned lock means a panic unwound while that single writer held it and process state
  is untrusted — **[inferred] from clause 7's one-writer premise; it is not measured, and no poisoned store lock
  has been observed in this programme** (ADR-0118's own tagging). The decisive argument is AC-360 (register
  :5192-5197): *"AND no dispatch path exists that can terminate without appending that event"* — appending goes
  through the store the poisoned lock guards, so a returned `StepError` would manufacture exactly the path AC-360
  forbids. A halt does not create it: the dispatch path does not terminate, the run stops.
- **What does not change.** ADR-0118 carries `supersedes: []`. **ADR-0112's `StepError` stays CLOSED at eleven
  variants** (:51, and *"Forbids a twelfth variant without a successor to this record"* at :96), and
  `crates/rk-types/src/error.rs` is untouched by it. ADR-0115's clauses 1–5 all stand.
- **Two record-consistency items this record's arrival created, named and NOT repaired here** (neither edit is to
  this file). **(1)** `INDEX.md`:60 now gives this stage's *decisions it executes* column as **ADR-0118**, while
  `00-preamble.md`:26 still reads *"It takes **seven** decisions"*, its table at :29-37 carries no row for it, the
  eighth at :39 is the two fixture crates, and `INDEX.md`:91 still says *"`00-preamble.md`'s table names seven this
  runbook executes"* [all measured 2026-09-13]. Either the preamble's table gains a row and `INDEX.md`:91 moves with
  it, or `INDEX.md`:60 is wrong; both edits are **owed**. **(2)** ADR-0118's `source:` key cites
  `runbook/phase-1/s7a-ports-and-fakes.md :45-46`, which is where the lock-poisoning open item stood in the
  **127-line** revision of this file at `5580f19` (*"Lock **poisoning** is an open item ADR-0115 names and does not
  close — it must not be silently recovered with `unwrap_or_else(|e| e.into_inner())`"*). The repair pass of the
  same day rewrote this file and :45-46 now carries the git-surface denylist sentence; the sentences the record drew
  on are **this section**. An accepted record is never edited (README *Immutability rule*), and ADR-0092 decision 1
  makes a **citation** exactly what an erratum may correct, so **`E-0118-1` in `references/adr/ERRATA.md` is OWED**
  — no `## Decision` sentence moves and the decision is unchanged. It is not written here because `ERRATA.md` is not
  this file. The general hazard is `PROGRESS.md`:318's: a record citing a line number in a file a later stage may
  rewrite.
- **The prohibition survives and is widened to the spelling that shipped** (ADR-0118 clause 3): a poisoned store
  lock may not be silently recovered — not by `unwrap_or_else(|e| e.into_inner())`, not by
  `unwrap_or_else(std::sync::PoisonError::into_inner)`, not by any other spelling of `PoisonError::into_inner`,
  and not by a `catch_unwind` that resumes the run.
- **Scope: both store locks** (clause 4) — the port handle's `Arc<Mutex<dyn EventStore>>` and `rk-store`'s
  writer-connection `Mutex<Connection>`.
- **There is a live violation on this tree and it is not this step's to repair.**
  `crates/rk-store/src/conn.rs`:139-140 reads `.lock().unwrap_or_else(std::sync::PoisonError::into_inner)` with no
  comment, in the file whose module doc quotes ADR-0105 clause 7 [measured, `5580f19`]. It shipped at Step 6.3
  (`7aa02c7`), before ADR-0118 existed. ADR-0118 clause 4 names it a violation from acceptance and says its repair
  is **owed**; carrying it as a tracked item in `PROGRESS.md` is **owed and not done**. It is named here so the
  implementer meets it as a known debt and not as a discovery — and note the shape: it is the **method-path**
  spelling, so a grep written against ADR-0115's **closure** spelling does not find it. **The assertion below may
  not be widened to `crates/` to reach it, which is the move this paragraph invites** (golden constraint 9 —
  establish the guard's scope before you obey it *and before you route around it*). ADR-0118's own measurement is
  the warrant: repo-wide, `grep -rn 'into_inner' --include=*.rs crates/` returns **two** hits — `conn.rs`:140 and
  `rk-git`'s **benign newtype accessor** at `crates/rk-git/src/reconciled.rs`:41 — so a widened grep fails a correct
  tree on a line ADR-0118 forbids nothing about (ADR-0118:210-214: *"which is why the Step 7a assertion is scoped to
  `crates/rk-ports/src/` and why a bare repo-wide `into_inner` grep is not the instrument"*). The `conn.rs` repair is
  **`rk-store`'s and is owed**; it is not reachable from this step's assertion and must not be made so.

**What Step 7a owes, and what it does not.** ADR-0115 clause 4 puts the acquisition at the composition root, and
design:1707 makes the composition root `rk-cli`, at **Step 7d**. **Step 7a writes no `lock()` call site at all.**
So 7a owes the **prohibition as an assertion** — that `crates/rk-ports/src/` contains no poison recovery — in Halt
8's hardened form, the directory proven present first and the search's status captured and asserted before anything
is read from its output, matched on `into_inner`, `PoisonError` and `catch_unwind` rather than on the closure text —
the bare `into_inner` token **stays** in the alternation because `match lock() { Err(guard) => guard.into_inner() }`
contains neither `PoisonError` nor `unwrap_or_else`, and clause 3 forbids *"any other spelling"*. **Two exclusions
keep the floor off a correct crate, and the residual width is stated rather than hidden:** a comment line (the
house pattern is a module header naming the governing record — `crates/rk-store/src/lib.rs`:1-20 — and a header that
quotes this prohibition carries the token), and the declaration `pub fn into_inner`, which is already a house API at
`crates/rk-git/src/reconciled.rs`:41. **What is left over-wide, named:** `Mutex::into_inner` on a value the caller
owns is not poison recovery and this floor would still refuse it, so a fake that consumes its own script mutex that
way trips a rule ADR-0118 does not state. That is the scope, and it is the finding rather than the verdict (golden
constraint 9): spell such an accessor `.get_mut()` or `.lock().expect(…)`, or take the line to the operator — do not
widen the exclusion silently. **That assertion is a floor, not evidence** (golden constraint 12). **Step 7d owes the discriminating test** and is the only stage that can write
one: poison the store lock from a panicking thread, assert the run stops, no `StepError` value is produced and no
recovered guard exists. Exit code, diagnostic wording, and panic-vs-abort are **owed at Step 7d under ADR-0107**
(ADR-0118 clause 6) and are not decided here.

**The cost, carried rather than smoothed:** ADR-0118 rejects "halt and append a poisoning event" on ADR-0105
clause 7 — recording it would need a second writer connection while the first is poisoned — so **the halt leaves no
mark in the event log.**

### 7a.5 Where the types live

**ADR-0099 clause 1's last sentence is the routing, and it is quoted in full** (`0099`:61-62): *"`ActivitySpec`
carries `StepSpec`'s fields (card digest, tools, schema id, timeout, inputs); `DispatchRecord` carries
`StepRecord`'s. **Both are `rk-types`.**"* *(Clause 1's prose is five sentences — blocks-to-completion, the in-band
timeout, the `record`/`result` pair, `Typed`, and this one. The previous revision called it the third.)* The
reference shape declares all of these locally under its own `// ---- rk-types (the shapes ADR-0099 names)` header at
lib.rs:40 — `SchemaId` :43, `Typed` :47, `StepError` :54, `ActivitySpec` :88, `DispatchRecord` :98, `Usage` :107,
`DispatchOutcome` :114 [measured 2026-09-13] — **because it is a throwaway single crate**; `rk-ports` imports and
re-declares none of them. `StepSpec` and `StepRecord` are **not types to author** —
they are the retired runner's names, cited as field provenance only, and exist nowhere on this tree [measured].

| Type | Home | Warrant | Status |
| --- | --- | --- | --- |
| `ActivitySpec` | `rk-types` | ADR-0099:62, *"Both are `rk-types`"* | **decided by the record**; not on the tree yet (`grep -rn 'ActivitySpec' crates/` → 0 hits, measured 2026-09-13) |
| `DispatchRecord` | `rk-types` | ADR-0099:62, same sentence | **decided by the record**, and **collided with on the tree** — see below |
| `StepError` | `rk-types` | shipped at Step 5, `crates/rk-types/src/error.rs`:57, eleven variants | decided and present |
| `DispatchOutcome` | `rk-ports` | **routed, not decided.** No record assigns it. It is declared in ADR-0099 clause 1's own code block (:54) as `dispatch`'s return type, and the trait cannot be declared without it. design:1704 does not list it; neither does design:1699's `rk-types` row | **a record naming the home is OWED** |
| `Typed` | `rk-types` | **routed, not decided.** No record assigns it. ADR-0099:59-60 makes it *"an erased, validated value — `serde_json::Value` plus the schema id it validated against — which the kernel deserialises"*: it is produced by the adapter and consumed by the kernel, so it crosses the boundary in both directions exactly as `StepError` does, and `rk-types` already owns schema derivation and the digests (design:1699) | **a record naming the home is OWED** |
| `SchemaId` | `rk-types` | **routed, not decided.** Same warrant: it is the schema id half of `Typed`, and schema derivation is `rk-types`' by design:1699 | **a record naming the home is OWED** |
| `EventStore::append`'s parameter | `rk-types` | **routed, not decided.** ADR-0105 clause 7 (:81-82) places the event-kind table (kind · tier · payload fields · producing step) in design §5 **with `rk-types` as the owning crate**, and `rk-types` already ships the closed `EventKind` (`crates/rk-types/src/event_kind.rs`:37) and the closed `FailureClass` / `RefusalClass` (`event.rs`:12, :24) | **a record naming the record type is OWED** |

**The spike's `pub enum Event` is not carried over.** `spikes/s-ports/src/lib.rs`:121 declares
`Event { StepFailed { class: String }, … }` — stringly-typed where `rk-types` already ships closed class enums, and
a throwaway by its own doc comment. **No kind-plus-payload record type exists on this tree** [measured]: `rk-types`
has `EventKind` and the class enums and no record that carries a kind with its payload.

**A full kind-plus-payload record is NOT authored at Step 7a, because it cannot be.** Design §5's event-kind table
gives payload fields in **prose, not types** — the column is headed *"Payload fields (named in this document)"* and
every row is English (design:701-712, e.g. *"the step key and exactly its eight components"*, *"the failed predicates
and each facet's evaluated value"*) — and two `EventKind` variants already carry the note *"Name owed to design §5
(given there in prose only)"* in source (`crates/rk-types/src/event_kind.rs`:50, :57) [measured 2026-09-13]. Typing
all ~28 payloads off that table is a design task with a guaranteed halt inside it, and Step 7a is not the stage that
takes it. *(The previous revision said to read the field set off design §5 and to halt where it fixes none. The halt
was not a contingency — it was certain, and it would have stopped the stage before `rk-ports` existed.)*

**What Step 7a authors instead, in `rk-types`, is the minimum `append` needs and nothing more:**
`pub struct Event { pub kind: EventKind, pub payload: serde_json::Value }`, with
`fn append(&mut self, event: Event) -> Result<(), StepError>`. The `payload` is `serde_json::Value`
**provisionally and visibly** — `rk-types` already depends on `serde_json` and already ships the closed `EventKind`
(`event_kind.rs`:37) and the closed `FailureClass` / `RefusalClass` (`event.rs`:12, :24), so **no class is
stringly-typed**: a caller that wants a class uses those enums and serialises one. What this does **not** do is fix
the per-kind payload field sets. **The typed-payload record, per kind, off a design §5 table that must first be given
types, is OWED**, its earliest real consumer is the SQLite adapter at 7c, and it is **reported as ARMED, NOT
DISCHARGED** (golden constraint 10): no assertion in Checkpoint 7a claims the payload is typed.

**The `DispatchRecord` collision is real, and it is named here so the implementer does not discover it.** Three
shapes exist under one name and **none of them is the record's**:

| Where | Field set | Measured |
| --- | --- | --- |
| `crates/rk-store/src/retention.rs`:52 | `argv_digest: Digest`, `env_digest: Digest`, `instrument_version: String`, `stdout: Retained`, `stderr: Retained` | on the tree, in **`rk-store`** — a crate ADR-0099:62 does not assign it to |
| `spikes/s-ports/src/lib.rs`:98 | `argv_digest: String`, `env_digest: String`, `instrument_version: String`, `wall_clock_ms: u64`, `usage: Option<Usage>`, `stdout_head: String` | stringly-typed digests, no `stderr`, no size cap |
| **What the record requires** | argv digest, env digest, instrument version, **usage**, **timings**, and ADR-0105 clause 7 retention | **ADR-0099**:58 and **AC-360** (register :5202-5204). `Usage` exists in no crate — one hit tree-wide, `spikes/s-ports/src/lib.rs`:107 [measured] |

**Which one the record makes authoritative:** the **field set of ADR-0099:58 / AC-360**, homed in **`rk-types`** by
ADR-0099:62. Neither shipped shape is it, and the two shipped shapes disagree with each other on the digest type,
on `stderr` and on `usage`/`timings`.

**What Step 7a does about it: names it, authors the record's home minimally, and leaves the RECONCILIATION to 7c.**
`rk-ports` authors **no** `DispatchRecord` of its own — that is forbidden by ADR-0099:62 and asserted by Checkpoint
7a — and imports the one `rk-types` owns.

**Step 7a therefore edits `crates/rk-types/` as well as `crates/rk-ports/`, because `rk-ports` cannot declare a
trait without four types that are on no crate today.** `DispatchOutcome`'s fields and `Harness::dispatch`'s
parameters name `ActivitySpec`, `DispatchRecord`, `Typed` and `SchemaId`, and `grep -rnE '^pub (struct|enum|type) '
crates/rk-types/src/` returns **32** items, **none of them** [measured 2026-09-13]. Executed as the previous revision
read — *"imports the one `rk-types` will own"* — the crate does not compile, `Harness` cannot be declared, and every
group in Checkpoint 7a is unreachable. The four are authored in `rk-types` (ADR-0099:62's assignment, which is the
record's) with the **minimum** field sets the records fix and nothing more:

- **`SchemaId`** — a newtype over the schema id, in the shape `rk-types`' existing newtypes take (`digest.rs`:23,
  `text.rs`:9, `time.rs`:15).
- **`Typed { schema: SchemaId, value: serde_json::Value }`** — ADR-0099:59-60's own two halves, verbatim
  (*"`serde_json::Value` plus the schema id it validated against"*).
- **`ActivitySpec`** — card digest (`Digest`), tools, schema id (`SchemaId`), timeout, inputs: ADR-0099:61's five,
  and no sixth.
- **`DispatchRecord`** — `argv_digest: Digest`, `env_digest: Digest`, `instrument_version: String`, plus the
  ADR-0105 clause 7 retention fields `rk-store` already types (`crates/rk-store/src/retention.rs`:52's `Retained`).
  **`argv` and `env` are digests only — ADR-0105 clause 7 — and that is an authoring constraint here, not an
  observation about the tree.** **`usage` and `timings` are NOT authored here and the type carries no field for
  them:** `Usage` exists in no crate (one hit tree-wide, `spikes/s-ports/src/lib.rs`:107 [measured]), the field set
  is ADR-0099:58 / AC-360's, and its completion is owed at 7c where the first adapter constructs one. **A
  `DispatchRecord` without `usage` and `timings` is incomplete by the record and is written down as such** — it is
  not the record's field set and this step does not claim it is.

**`rk-store::retention::DispatchRecord` is untouched by this step.** Two types under one name in two crates is the
collision named above; collapsing it is 7c's, and Checkpoint 7a asserts only that **`rk-ports`** declares neither. **No erratum is available**: ADR-0092 decision 1, as ADR-0115:38-41 quotes it, reserves errata for evidence
lines and citations and requires a **supersession** to move a `## Decision` sentence, and `ERRATA.md` carries zero
entries for 0099 deliberately. There is also **no record contradiction to correct** — ADR-0099:58 already defers
retention to ADR-0105, so the two records agree; what disagrees is the **tree**. **Owed, and named rather than
scheduled here:** the reconciliation of `rk-store`'s declaration with ADR-0099:62's assignment, and the completion
of the field set (`usage`, `timings`, the `Usage` type). **Its earliest real consumer is 7c**, where an adapter
first constructs one; it must land before then, and this step does not settle it.

### 7a.6 The parity mechanism — AC-481, as amended 2026-09-13

**The register's registration attributes were never built, and the register now says so.** As authored on
2026-09-11 AC-481's GIVEN derived the trait set from `#[port]` on every `pub trait` and the fake set from
`#[fake_for(Trait)]` on every fake. **Neither is a Rust attribute**: applied to a five-port shape under the pin
they give ``error: cannot find attribute `port` in this scope`` and ``error: cannot find attribute `fake_for` in
this scope`` [measured 2026-09-13, rustc 1.98.1 (48a229cea 2026-09-01); the same two errors under 1.97.1]. An
attribute macro needs a crate with `proc-macro = true`, and design §9.1 — *"Ten crate families — **and this table
is the crate-name authority**"* (design:1695, per ADR-0099 clause 5) — names no macro crate among the ten
[documented]. **The register was amended on 2026-09-13** (register :6918-6919 and the dated parenthetical at
:6926-6943): both sets are derived from **a parse of `rk-ports`' own source text**. **No proc-macro crate is
created and design §9.1's crate authority is not amended.**

**The three clauses that bind and are unchanged** (register :6918-6923):

1. the sets are derived **from source** and **never** from a hand-maintained inventory list;
2. the equality is asserted **as a number**, not as a spot check;
3. **a port trait added without a fake fails the suite naming the missing fake, by name, from the same source
   parse** (the AND clause at :6922) — and the PIA-059 fixture at :6923-6925 adds the sixth trait **and edits no
   inventory**.

**What the amendment costs, restated here because the register states it rather than smoothing it:** an attribute
registration is a **declared** set and a source parse is an **inferred** one, so the parse can disagree with
authorial intent in two directions — a `pub trait` the author did not mean to register is counted, and a fake the
parse cannot see is not. **Neither direction is visible in the count alone, because both sides can move together**
[inferred]. **The PIA-059 fixture is what holds the parse honest.**

**The mechanism, specified, because three obvious implementations fail here and two of the three fail silently:**

- **Where the parse reads.** A recursive `std::fs::read_to_string` walk of
  `concat!(env!("CARGO_MANIFEST_DIR"), "/src")` **at test time**. Prefer it over `include_str!` of each file: an
  `include_str!` list is itself an inventory a fixture must edit, which is precisely what clause 1 forbids and what
  PIA-059 caught. A `build.rs` is third-best — its failure is a build error, not a red test.
- **Trap 1 — the assertion's spelling.** Checkpoint 7a and CI both grep for the literal
  `traits.len() == fakes.len()` (design:1783 makes it a CI structural assertion by that name). The idiomatic
  `assert_eq!(traits.len(), fakes.len())` **does not contain that string** and would fail the checkpoint on a
  correct crate. Spell it `assert!(traits.len() == fakes.len(), "port traits with no fake: {missing:?}")` — one
  statement that carries both the literal the grep needs and clause 3's by-name message. **rustfmt reflows that
  statement and the literal survives**: the break falls at the commas, leaving `traits.len() == fakes.len(),` whole
  on one line, so the checkpoint's parity grep and `ci.yml`'s both still match [measured 2026-09-13 on the pin —
  `cargo fmt` then `cargo fmt --check` (exit 0) and the checkpoint's own grep]. Do **not** wrap the comparison
  yourself across a line break to shorten it: that is the one edit that satisfies `cargo fmt --all --check` and
  fails the grep.
- **Trap 2 — the container.** Collect into `BTreeSet`, **never** `HashSet` or `HashMap`: both are in
  `clippy.toml`:30-32's `disallowed-types`, denied through `all = { level = "deny", priority = -1 }` at
  `Cargo.toml`:58, and CI lints test targets (`--all-targets`). A parity test written with a `HashSet` fails
  `clippy` and takes the whole gate with it.
- **Trap 3 — what a naive parse counts.** A line-wise match on `impl <Trait> for` counts a **doc-comment** line as
  a fake: `spikes/s-ports/src/lib.rs`:16 is `//! impl Harness for NotSendHarness {` inside a `compile_fail` block.
  It also counts the `#[cfg(test)]` stubs at lib.rs:325-327 if they are carried over. **Strip comment lines before
  parsing, anchor the `impl` match at column 0, and require the literal ` for `.** A parse that counts four fakes
  because three of them are comments passes while the crate is wrong — the inverse of the PIA-059 failure the
  fixture exists for.
- **Trap 4 — what the `impl` match counts that is not a fake, and it fails RED on a correct crate.** A line-wise
  `impl <Name> for` parse inserts **every** trait impl into the fake set, not only the five ports:
  `impl Debug for FakeHarness`, `impl Default for VecStore`, any `From`. `fakes.len()` then exceeds `traits.len()`
  and the test goes red printing `port traits with no fake: {}` — a failure message that names nothing, which is
  AC-481's AND clause broken in the **failing** direction. [measured 2026-09-13, a two-port scratch crate on the pin:
  one added `impl fmt::Debug for FakeHarness` turns the suite red with exactly that message.] **Intersect the parsed
  impl set with the parsed trait set before comparing**, so both sides are the same universe:

  ```rust
  let fakes: BTreeSet<String> = fakes.intersection(&traits).cloned().collect();
  let missing: BTreeSet<_> = traits.difference(&fakes).collect();
  assert!(traits.len() == fakes.len(), "port traits with no fake: {missing:?}");
  ```

  With the intersection the count equality and the by-name set say the same thing and cannot disagree, and the
  PIA-059 fixture still fails naming the sixth trait [measured, same crate: adding `pub trait Sixth: Send + Sync {}`
  with no fake gives `port traits with no fake: {"Sixth"}`]. Without it the two can disagree, and the one that fires
  is the one that names nothing.
- **Trap 5 — the parse's own shape trips the deny set and takes the whole clippy gate.** The natural nested form,
  `if let Some(rest) = line.strip_prefix("pub trait ") { if let Some(name) = … { } }`, trips
  **`clippy::collapsible_if`** — inside `clippy::all`, denied at `Cargo.toml`:58 — and `#[allow]` is not available
  (ADR-0102 decision 3, :58-59; `#[expect(…, reason = "…")]` only, and there is no reason to state). Write it as an
  **edition-2024 let-chain**: `if let Some(rest) = line.strip_prefix("pub trait ") && let Some(name) =
  rest.split(':').next() { … }`. [measured 2026-09-13 on the pin: the nested form gives
  ``error: this `if` statement can be collapsed … -D clippy::collapsible-if implied by -D clippy::all`` and the
  build fails; the let-chain form gives `cargo clippy --all-targets -- -D clippy::all` exit 0 and
  `cargo fmt --check` exit 0 with the Trap-1 literal intact.] **And keep every port item ABOVE the
  `#[cfg(test)] mod tests`** — `clippy::items_after_test_module` is in the same group.

**The failure message is a clause, not a nicety.** A test that only goes red does not discharge AC-481's AND
clause. The message must name the trait or traits with no fake, from the same source parse.

### 7a.7 What the acceptance set's fixtures are, and where they live

**`crates/rk-ports/tests/ui/` carries four `trybuild` cases, and they do two different jobs.** `trybuild = "1"` is
already in `[workspace.dependencies]` at `Cargo.toml`:49, put there with the reason stated in the manifest
(*"their instrument is a compile FAILURE, which only a must-fail harness can observe"*), and it is already a
dev-dependency of `rk-store` and `rk-git`. Each case commits a `.stderr` snapshot, so a case that starts failing
for a **different** reason goes red. That is the job the `compile_fail` fences cannot do — §7a.8, measured.

| Case | Job | Source |
| --- | --- | --- |
| `arc_dyn_port_rejects_rc.rs` | **pins the reason** of doctest fence 1 | design §3.3 P10's *"a compile-fail fixture for a port trait without the supertraits"* (design:288) |
| `bare_arc_store_has_no_mut.rs` | **pins the reason** of doctest fence 2 | ADR-0115:130-132, *"The bare `Arc<dyn EventStore>` form does not compile against `&mut dyn EventStore` — that is the finding"* |
| `git_port_is_not_a_port.rs` | **a criterion's own named fixture** | AC-483's Verified-by: *"a compile-fail fixture declaring `trait GitPort`"* (register :6958) |
| `async_fn_in_a_port_trait.rs` | **a criterion's own named fixture** | AC-528's Verified-by: *"a source lint over `rk-ports` … **with a compile-fail fixture adding one**"* (register :7162-7163) |

**The harness test's NAME is load-bearing and is fixed here, because the checkpoint filters on it.** The four cases
are run by **one** `#[test]` in `crates/rk-ports/tests/it/ui.rs`, declared `mod ui;` from `tests/it/main.rs`, called
**`trybuild_pins_the_reason_each_port_fixture_fails`**:

```rust
#[test]
fn trybuild_pins_the_reason_each_port_fixture_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/arc_dyn_port_rejects_rc.rs");
    t.compile_fail("tests/ui/bare_arc_store_has_no_mut.rs");
    t.compile_fail("tests/ui/git_port_is_not_a_port.rs");
    t.compile_fail("tests/ui/async_fn_in_a_port_trait.rs");
}
```

The name is stated rather than left to the implementer for a mechanical reason: **cargo's `--` filter matches the
TEST's own name**, not the crate, the target or the file, and **the house convention names a `trybuild` harness after
its rule, not after the tool** — `crates/rk-git/tests/it/reconciled.rs`:24 is `reconcile_runs_at_every_entry` and
`crates/rk-store/tests/it/tier_ui.rs`:9 is `a_state_field_without_a_tier_is_a_compile_error`, and **neither name, nor
its module, contains the substring `trybuild`** [measured 2026-09-13]. A filter spelled `-- trybuild` therefore
selects **zero** tests in both existing members and `cargo test` still exits **0** (`0 passed; N filtered out`). The
name above is the house form — it names the rule — and carries the token the checkpoint filters on. *(The previous
revision of Checkpoint 7a filtered on `trybuild` and named no harness anywhere, so it could not pass on a crate built
exactly as this section prescribes. `s7b`:159-162 has the same defect and its repair is owed — §7a.11.)*

Generate each `.stderr` once with `TRYBUILD=overwrite`, **read it**, and commit it: an unread snapshot pins whatever
reason the case actually had, which is the defect the suite exists to close.

**The last two may not be authored as doctests under `src/`**, and the reason is mechanical rather than stylistic:
Checkpoint 7a pins the `compile_fail` fence count under `src/` at exactly **2** — the two fences of §7a.8, and
nothing else — so a third would fail the checkpoint **for obeying the register**; and AC-528's would additionally
put an `async fn` under the async denylist, whose pass condition is `rc` exactly 1. Under `tests/ui/` neither
collides: the fence count greps `crates/rk-ports/src/`, and the async denylist greps the same directory.

**AC-528's second fixture is not a file — it is a commit, and it is run by hand.** The register's words are *"a
fixture commit adding `tokio` to `rk-ports` that must fail"* (:7163). Add `tokio = "1"` to
`crates/rk-ports/Cargo.toml`, run Checkpoint 7a's `cargo tree -p rk-ports` pair, **watch them fail naming tokio**,
then revert and record the observation. Without it that line is a Halt 8 shape (g) pass condition — the absence of
output — that has never been shown able to fail.

**AC-483's denylist is built here, not merely asserted.** A test named **`git_surface_denylist_rejects_git_port`** —
the name the checkpoint's `--` filter matches, and the only spelling of it in this file — over the **same
source-derived trait set the parity test parses** refuses any `pub trait` in `rk-ports` whose name or method names
match the git surface — `trailer`, `worktree`, `clone`, `reconcile`, `GitPort` — which is the register's own
enumeration at :6957. The checkpoint's grep below is the floor; the test and the `trait GitPort` fixture are the
evidence. **This is the one instrument in this step's set that is fully buildable here**: `rk-git` landed at Step 6
(`7aa02c7`) and `rk-ports` is this step's.

**Three of this step's assertions belong in CI, not only in this checkpoint** (design:1783 names each by id): the
`rk-ports` parity assertion (AC-481), the `async fn` / future-returning denylist over `crates/rk-ports/src/`
(AC-528), and the port-inventory git denylist (AC-483). Add them to `.github/workflows/ci.yml`'s `structural` job
in the same commit as the crate, **comment-filtered and anchored, with the tracked-file positive control**, for the
reason written out at `ci.yml`:186-192. `cargo tree -p rk-kernel -e normal` (AC-528's second half) and the
`rk-kernel` API-surface lint (AC-480:6914) are added at 7b. **Reported as armed, not discharged** (golden
constraint 10): `PROGRESS.md`:307 records that `ci.yml` is **authored and has never run**, so a CI row added here
is armed and nothing more.

### 7a.8 The two `compile_fail` doctests, and what `compile_fail` cannot pin

**F41's second doctest cannot be carried over.** It calls `kernel::Kernel::new(…)` and `k.run_with(run_generic)`
against a `kernel` module the spike keeps in its own crate solely for that purpose
(`spikes/s-ports/src/lib.rs`:229-256; `run_with` at :247). `rk-ports` may not contain a kernel module
(design:1704) and `rk-kernel` does not exist until 7b. **Pasted in, it fails with E0433, unresolved module
`kernel` — and `compile_fail` passes on that**, inside the very assertion Checkpoint 7a annotates as proof that
both of F41's fences are carried over. That is Halt 8's shape one level down.

**So the second fence is re-homed on a rule `rk-ports` owns.** The two fences `rk-ports` carries are:

1. **`arc_dyn_port_rejects_rc`** — F41's first doctest, unchanged: a fake holding an `Rc` cannot become
   `Arc<dyn Harness>`. This is design §3.3 P10's own named instrument, *"a compile-fail fixture for a port trait
   without the supertraits"* (design:288), in the form F41 wrote it.
2. **`bare_arc_store_has_no_mut`** — the **withdrawn** holding shape: a `Ports` whose `store` field is
   `Arc<dyn EventStore>` cannot produce the `&mut dyn EventStore` that `Harness::dispatch` takes, because
   `Arc<dyn T>` yields `&T` and never `&mut T`. **This is the only instrument at any level that discriminates
   ADR-0115 from the clause it narrows** — Checkpoint 7a's supertrait count reads **5 under either shape**, because
   ADR-0115:57-58 leaves the supertraits on the trait declaration unchanged — and it is the shape AC-480's AND
   clause carries (register :6912). It is `rk-ports`-local, needs no kernel, and is ADR-0115:130-132's own measured
   finding.

**`no_generics_leak` is not written here.** It needs a kernel entry point to fail against. **AC-480 stays in this
stage's acceptance set** — register :6902 puts it there and ADR-0092 outranks this runbook — but its generics-leak
instrument is 7b's: AC-480's WHEN is *"WHEN `rk-kernel` is compiled"* (register :6910) and its Verified-by is *"an
API-surface lint over `rk-kernel` …; a compile-fail test fixture that introduces one such parameter and must not
build"* (:6914-6915). Checkpoint 7b already runs `cargo test -p rk-kernel -- trybuild` for exactly that class of
fixture (`s7b`:159-162).

**What `compile_fail` alone cannot pin: the reason.** A bare `compile_fail` is green whenever compilation fails for
**any** reason. The obvious remedy — annotating an expected error code on the fence — **is inert at this pin, and
this was measured on this box rather than assumed.** A three-case probe crate, `cargo test --doc`:

| Fence info string | Body actually fails with | Result |
| --- | --- | --- |
| `compile_fail,E0277` | E0308 (mismatched types) | **ok** |
| `compile_fail,E9999` (a code that does not exist) | E0308 | **ok** |
| `compile_fail,E0277` | E0433 (unresolved module) | **ok** |

`test result: ok. 3 passed; 0 failed`, exit 0 — **identical on rustc 1.97.1 (8bab26f4f 2026-07-14) and on the pin,
rustc 1.98.1 (48a229cea 2026-09-01)** [measured 2026-09-13, this box]. Error-code checking in doctests is
nightly-only, and no nightly run was made here, so that half is **not measured** and is not relied on.

**Therefore:** keep both doctests, because F41 carries them and the checkpoint counts them — `doctest = false` runs
zero doctests and still exits 0 — and **carry the reason in the `trybuild` suite of §7a.7**, whose committed
`.stderr` files are what actually pin it. The doctests also cannot be filtered by name: `cargo test --doc` names a
doctest positionally (`test src\lib.rs - (line 11) - compile fail`), which is a second reason the `trybuild` cases
are the evidence.

### 7a.9 Red first

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence: every one of
its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins. Author each named test
**before** the code it describes, run it, and watch it fail for the right reason — the absent behaviour, not a
compile error or a filter that matched nothing.

- `port_fake_parity` — the assertion is spelled `traits.len() == fakes.len()` and is **evaluated**. Red by adding a
  sixth trait with no fake: the test must fail **without anyone editing an inventory**, and **the failure message
  must name that trait** (AC-481's AND clause, register :6922; the PIA-059 fixture at :6923-6925). A grep that the
  string exists is not this test; keep the grep as the floor and the fixture as the evidence. **The fixture's name,
  path and feature gate are not decided here** — that is the treatment the two must-fail fixture crates already got
  (`00-preamble.md`:39; DECISIONS-2026-09-12 E4, executed at `s5` §5.1a as feature-gated members so the default
  gates stay green). Until that decision lands this stage states the requirement and does not name the fixture;
  write the checkpoint assertion the moment it is named.
- `ports_struct_holds_arc_dyn_send_sync` — the `Ports` struct holds `store: Arc<Mutex<dyn EventStore>>` and the
  other four as `Arc<dyn …>`, and moves the struct across a `std::thread::spawn` — the only assertion that shows
  the `Send + Sync` supertraits buy what ADR-0099 clause 4 says they buy. Red by reverting the store field to
  `Arc<dyn EventStore>`: the test must fail and `Harness::dispatch`'s `&mut dyn EventStore` argument must stop
  compiling with it [measured, ADR-0115:130-132]. F41 carries this test already (`spikes/s-ports/src/lib.rs`:324).
- `git_surface_denylist_rejects_git_port` — the port-inventory lint's denylist over the git surface (trailers,
  worktrees, clone, `reconcile`), authored red against `tests/ui/git_port_is_not_a_port.rs` (AC-483's Verified-by,
  register :6958). **This is §7a.7's `port_inventory_denylist` under its one binding name** — the checkpoint's
  `--` filter matches the test's own name, so the two spellings the previous revision carried could not both be
  right.
- `port_conformance_fake_rows` — **AC-499's fake half**, and the instrument the previous revision claimed the id
  without. One parameterised body per port (register :7136-7137, *"the shared per-port conformance suite"*),
  instantiated over the **fake** row of all five ports, with each Phase-1 **real** row present as a named, **empty**
  row — SQLite, Claude, Node, Rust (register :7139-7140) — rather than absent. Red by deleting one port's fake row:
  the suite must fail **naming that port**. The real rows are 7c's to fill. **The Harness real row runs a real
  dispatch and is a local-only, named-runner row until ADR-0010's CI decision is taken** (register :7140-7141); it
  may not be wired into CI here or at 7c.
- `trybuild_pins_the_reason_each_port_fixture_fails` — the four `tests/ui/` cases of §7a.7 under one
  `trybuild::TestCases`. Red by making one case compile, or by deleting its committed `.stderr`; a case that starts
  failing for a **different** reason is what the `.stderr` snapshot catches, and it is the job the `compile_fail`
  fences measurably cannot do (§7a.8).
- `toolchain_fake_classifies_binary` — the fixture-driven Toolchain fake returns `Binary` for a binary path and
  assembles no textual diff for it (AC-497's first named instrument, register :7117). Red by scripting the fixture
  and watching the fake return `Text`; a grep over the newtype declaration is not this test.
- `harness_fake_produces_unsupported` — the scripted-responder Harness fake produces
  `StepError::Unsupported(capability)` carrying the token when a test scripts a capability as absent (AC-500's
  first clause; register :7149's *"an integration test on the fake"*). The route-around half needs a pack and is
  7b/7e's.
- `no_async_in_port_traits` — AC-528's compile-fail fixture (register :7162). Red by adding `async fn probe(&self);`
  to one port trait, running the checkpoint's async line, and watching it FAIL; then revert. The durable form is
  `tests/ui/async_fn_in_a_port_trait.rs`, because the checkpoint line is a text rule and the fixture must also show
  the trait is no longer dyn-compatible.
- `no_async_runtime_in_the_graph` — AC-528's fixture **commit** (register :7163). Red by adding `tokio = "1"` to
  `crates/rk-ports/Cargo.toml`, running the `cargo tree` pair, watching them FAIL naming tokio, then reverting and
  recording the observation.
- The two `compile_fail` doctests are red by construction — that is what `compile_fail` means — but assert they
  **ran**, which is why Checkpoint 7a counts them, and **pin the reason in `tests/ui/` rather than on the fence**
  (§7a.8, measured).
- **Write the sync rationale as a module header.** `crates/rk-types/src/lib.rs`:6-7 carries exactly this citation
  (*"It may depend on **no async runtime** — no `tokio`, no `async-std`, no `futures`, no `async-trait`
  (ADR-0076)"*) and `crates/rk-store/src/lib.rs`:1-20 is the house pattern of naming the governing record in the
  header. The checkpoint's async guard excludes comment lines by construction, so it will not halt on yours — the
  unnarrowed form would have, which is why it was narrowed. A signature wrapped by `rustfmt` can still slip a
  `-> Pin<Box<dyn Future<…>>>` past a single-line regex; the `compile_fail` fixture AC-528 names is what pins that.

### 7a.10 The manifest and the file layout

**The reference shape's manifest is wrong for this workspace, and three members already fix the right one.**
`spikes/s-ports/Cargo.toml` carries a literal `edition`, `[lints.rust] unsafe_code = "deny"` and nothing else — no
`[lints] workspace = true`, no `corpus` feature, no `[[test]]` stanza, no inherited fields. Copying it, as §7a.3
tells you to copy the code, produces a crate **outside the workspace deny set** (`Cargo.toml`:51-52 and :54-68) and
outside ADR-0083's one-integration-binary rule, **over which every assertion in Checkpoint 7a goes green**.
`crates/rk-ports/Cargo.toml` carries instead, matching `rk-types`, `rk-store` and `rk-git`:

- `edition` / `rust-version` / `license` / `publish` inherited with `.workspace = true`;
- `[features] corpus = []`, declared **empty** — 7a has no corpus-bound test, but the feature must exist or a
  `--features corpus` invocation naming this package exits **101** with *"none of the selected packages contains
  these features"*; the reason is written out at `crates/rk-types/Cargo.toml`:10-17;
- `[dependencies] rk-types = { path = "../rk-types" }` plus `serde` / `serde_json` from the workspace table, and
  nothing else — **no async runtime**, which is what AC-528's fixture commit is a commit against;
- `[dev-dependencies] trybuild.workspace = true`, **plus `rk-types = { path = "../rk-types" }` repeated**, with the
  reason stated in the manifest as the other members state it: `crates/rk-types/Cargo.toml`:31-32 carries it
  verbatim — *"Repeated for the integration binary: a crate's own `[dependencies]` are not in scope inside
  `tests/`"* — and `crates/rk-store/Cargo.toml`:41-42 repeats `rk-types` for exactly that. AC-500's instrument is
  *"an integration test on the fake"* (register :7149) and it names `StepError::Unsupported { capability }`;
- `[[test]] name = "it"` / `path = "tests/it/main.rs"` stated explicitly — **one** integration binary with a `mod`
  per case, never one file per case (ADR-0083);
- `[lints] workspace = true` as the last stanza. **This line is load-bearing** (ADR-0102 decision 2 — *"A member
  manifest's `[lints]` table is `workspace = true` and nothing else"*): without it the crate is under neither
  `unsafe_code = "deny"` nor the clippy deny set, and every lint assertion in the block below is vacuous. The only
  in-source override permitted is `#[expect(clippy::<lint>, reason = "…")]`, never `#[allow]` (ADR-0102 decision 3,
  :58).

**The crate's file layout is not this step's to decide** (`00-preamble.md`:53). The house layout is one module per
concern with `lib.rs` carrying a doc header and `pub mod` lines — `grep -c '^pub mod' crates/<c>/src/lib.rs` reads
**12** for `rk-types`, **8** for `rk-store` and **11** for `rk-git` [measured 2026-09-13] — and F41 is a single
file only because it is a throwaway. **Every assertion below is therefore recursive over `crates/rk-ports/src/`, never scoped to
`src/lib.rs`**, so the checkpoint does not impose a single-file library that no record decides and that all three
existing members contradict.

**Checkpoint 7a.** Counted mechanically off the block on 2026-09-13, **after the repair pass of that date and not
before it** — comments and blanks dropped — **fifteen assertion groups** carried by **100 non-comment statements, of
which 62 can stop the block and 38 cannot**. *Statements that can fail* is the unit the tracker records, and it is
named here because a revision before this one said "All nine assertions" in no unit at all: `PROGRESS.md`:290 counts
it that way for Checkpoint 4 (twenty), `PROGRESS.md`:28 for Checkpoint 5 (thirty-three of thirty-three ran) and
`s7b`:149-150 for Checkpoint 7b (twenty). **The 38 that cannot stop the block are not unguarded assertions**, and
the decomposition reproduces: **2** BEFORE readings (PIA-035 — prints for comparison, not assertions); **30**
`out=` / `n=` / `hits=` / `files=` / `raw=` capture halves whose guard is the statement immediately below them;
**1** clippy line, `cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"`, whose last statement is an
`echo` and whose guard is likewise the line below; **1** `printf '%s\n' "$out"` that puts the AC-481 parity
assertion's own location into the checkpoint's record; and **2 + 2**, the `for` and `done` of the two loops.
2 + 30 + 1 + 1 + 2 + 2 = 38; 100 − 38 = 62. *(The previous revision gave 75 / 50 / 25 with a decomposition of
2 + 21 + 2 that summed to 25 but did not reproduce off its own text — twenty captures, not twenty-one, with the
clippy line the unnamed twenty-first. Both the totals and the kinds are re-derived here.)*
**Re-count and re-write every number in this paragraph after any repair to this block**: a number written before
the block was final is the defect Checkpoints 4 and 7b each carried once.

**Hardened 2026-09-13, twice, and both times before its first run** — the preamble's rule
(`00-preamble.md`:230-235) is that hardening lands *before* the pass, or the recorded pass attests the old text.
What the previous form could not catch is written above each group. **The second pass fixed four lines that could
not pass on a correct crate** — the `-- trybuild` filter that selects zero tests, the manifest async grep with no
comment exclusion, the `xargs grep -n` that loses its path prefix on a single-file crate, and the unanchored
`Arc<Mutex<dyn EventStore>>` substring a comment satisfies — **and added three groups the previous form asserted
nothing for**: the design-named method floor, ADR-0102 decision 3's `#[allow]` prohibition, and the direct
`compile fail … ok` count. Every `grep … | wc -l` was converted to the capture-then-count form `halts.md`:82-83
prescribes, and every string comparison `[ "$n" = N ]` to `-eq`, which errors loudly on a non-numeric capture
instead of printing a FAIL naming a count that was never measured.

**No line in this block is `[WILL EXIST]`, and the absence is deliberate rather than an oversight.** A checkpoint
runs after its step's work, so every path the block names — `crates/rk-ports/`, its manifest, its `tests/ui/` cases
— is this step's own output and exists by the time the block runs; `.github/workflows/ci.yml` already exists. The
**one** thing that would not exist is `cargo tree -p rk-kernel -e normal`, and §7a.11 states why the carve-out is
not available to it (`halts.md`:89-91) and records the criterion half it leaves undischarged.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }

# ---- BEFORE readings (PIA-035): each PRINTS on the untouched tree and is silent after the step. They are
# ---- READINGS FOR COMPARISON, NOT ASSERTIONS — nothing below consumes them, and §7a.10 counts them among the
# ---- statements that cannot stop the block. Run them first and record both prints. (The previous revision's
# ---- comment said "each FAILS on the untouched tree", which is the opposite of what they do and contradicted
# ---- this block's own header; an operator resolving that at the moment of the first run is the defect.) The
# ---- revision before that carried a BEFORE assertion already green before the step ran, which attests nothing.
test ! -d crates/rk-ports && echo "BEFORE: no rk-ports crate yet"           # → prints on the untouched tree; after the step, silent
test ! -f crates/rk-ports/Cargo.toml && echo "BEFORE: no rk-ports manifest" # → prints on the untouched tree

# ---- the five CI gates, each guarded. This block previously ran ONE of them, package-scoped, on a
# ---- line nothing consumed: `cargo build -p rk-ports && cargo test -p rk-ports` short-circuits on a
# ---- failed build and the block continues. Checkpoints 5 and 6 each run all five (s5:379-390,
# ---- s6:193-199). `fmt` bites on contact: spikes/s-ports is `exclude`d from this workspace
# ---- (Cargo.toml:18) and is NOT rustfmt-clean against it — 15 hunks, measured 2026-09-13. `deny`
# ---- bites because adding rk-ports moves Cargo.lock and the licence set is re-derived when the lock
# ---- moves (s6:189-192).
cargo build --workspace || { echo "FAIL: cargo build --workspace (Halt 1's instrument from Step 5 on)"; exit 1; }   # golden constraint 3: build before you test
cargo fmt --all --check || { echo "FAIL: cargo fmt --all --check — reformat what you lift from spikes/s-ports"; exit 1; }
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red — the parity test is where clippy::disallowed_types (clippy.toml:30-32) and clippy::match_same_arms (Cargo.toml:65) bite"; exit 1; }   # → exit=0; nursery WARNINGS are permitted, the gate is the exit code (PIA-030)
out=$(cargo test --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }   # → a FLOOR ONLY, and vacuous against a tree that already ships rk-types, rk-store and rk-git tests: any of the three satisfies it. The assertion that bears on THIS step is the `cargo test -p rk-ports` pair below.
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses"; exit 1; }
# The crate-wide run, asserted separately: every other test line below is FILTERED, so any of
# rk-ports' unit tests can fail under another name while every guarded line stays green.
out=$(cargo test -p rk-ports 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p rk-ports exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: rk-ports compiled but ran 0 tests"; exit 1; }

# ---- the manifest, asserted the way Checkpoint 7b asserts rk-kernel's (s7b:164-166). Nothing here
# ---- asserted the manifest at all: without `[lints] workspace = true` the crate is outside the
# ---- workspace deny set and EVERY lint assertion in this block passes over an unlinted crate.
test -f crates/rk-ports/Cargo.toml || { echo "FAIL: crates/rk-ports/Cargo.toml absent"; exit 1; }
grep -q '^\[lints\]' crates/rk-ports/Cargo.toml || { echo "FAIL: no [lints] table in crates/rk-ports/Cargo.toml — the workspace lints do not apply to the new member (ADR-0102 decision 2)"; exit 1; }
grep -q '^workspace = true$' crates/rk-ports/Cargo.toml || { echo "FAIL: rk-ports' [lints] is not 'workspace = true' and nothing else (ADR-0102 decision 2)"; exit 1; }
grep -q '^corpus = \[\]$' crates/rk-ports/Cargo.toml || { echo "FAIL: rk-ports declares no corpus feature — a --features corpus run naming this package exits 101 (rk-types/Cargo.toml:10-17)"; exit 1; }
grep -q '^name = "it"$' crates/rk-ports/Cargo.toml || { echo "FAIL: no [[test]] name = \"it\" stanza — ADR-0083's one integration binary per crate"; exit 1; }
test -f crates/rk-ports/tests/it/main.rs || { echo "FAIL: crates/rk-ports/tests/it/main.rs absent (ADR-0083)"; exit 1; }
# `cargo tree` reads the RESOLVED graph; AC-528's fixture is a commit adding `tokio` to THIS
# manifest (register :7163), so the manifest is asserted too — they are not substitutes.
# COMMENT LINES ARE EXCLUDED, for the reason this workspace's own manifests state: the house pattern is a
# commented manifest naming the governing record and the forbidden crate in prose (crates/rk-store/Cargo.toml's
# rusqlite block is 20 lines; crates/rk-git/Cargo.toml's is 14), and §7a.10 tells the implementer to write
# `no async runtime` in this one. Unfiltered, the line halts on a CORRECT manifest — the same defect
# ci.yml:186-190 records for the source rule, which was the FIFTH Step-6 assertion to trip on prose naming
# the thing it governs. NOTE THE ANCHOR: a grep over a SINGLE NAMED FILE prints `line:content` with NO path
# prefix (measured 2026-09-13), so it is `^[0-9]+:` here and NOT the two-colon src/ form below.
raw=$(grep -nE 'tokio|async-trait|async-std|futures' crates/rk-ports/Cargo.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: rk-ports' manifest could not be read (grep rc=$rc; 2 = no such file) — a count of 0 here would be a fake 0; AC-528, ADR-0076"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*#')
[ -z "$hits" ] || { echo "FAIL: rk-ports' manifest names an async runtime on a non-comment line — AC-528, ADR-0076"; printf '%s\n' "$hits"; exit 1; }   # → silent

# ---- the async denylist. Comment lines are EXCLUDED, which is ADR-0076's own scope — the rule is
# ---- about DECLARATIONS, and this file hands the implementer a module header that explains why the
# ---- ports are sync (§7a.9). Five assertions in Step 6 tripped on prose naming the thing they
# ---- govern (ci.yml:186-190). NOTE THE FILTER'S SHAPE: `grep -vE '^\s*//'` can NEVER match, because
# ---- grep prints `path:line:content` and the line starts with the PATH — measured 2026-09-13, and
# ---- recorded at ci.yml:186-190 as the form that does not work. The alternation stays as wide as the
# ---- register's, which names BoxFuture too (:7159-7160); only the comment exclusion narrows.
# ---- `-H` BELOW IS LOAD-BEARING: grep prints the path prefix only when given more than ONE file, so on a crate
# ---- whose src/ holds a single .rs file — exactly the F41 shape §7a.3 says to start from — the output is
# ---- `line:content`, the two-colon filter matches NOTHING, and a doc comment naming `async fn` halts the block
# ---- on a correct crate [measured 2026-09-13]. ci.yml:195 is never in that case: it searches every member.
# ---- AND THE PIPELINE'S STATUS IS DELIBERATELY NOT CAPTURED. `xargs` exits 123 when grep matches nothing
# ---- [measured 2026-09-13], so HEAD's `hits=$(grep -rnE …); rc=$?; [ "$rc" -eq 1 ]` form is NOT transplantable
# ---- to this shape and an rc assertion here would fire on the clean case. That is a real TRADE and it is
# ---- recorded rather than implied: what this form buys is the tracked-file positive control below (Halt 8
# ---- shape (e), which the rc form did not close); what it gives up is the guarantee that EVERY grep
# ---- invocation succeeded — a single unreadable file yields rc 2, empty `hits`, and a silent pass. The
# ---- guarantee that the search ran at all is `test -d` plus `test -n "$files"`, not an rc.
test -d crates/rk-ports/src || { echo "FAIL: crates/rk-ports/src absent"; exit 1; }
files=$(git ls-files 'crates/rk-ports/src/*.rs' 'crates/rk-ports/src/**/*.rs')
test -n "$files" || { echo "FAIL: no TRACKED .rs files under crates/rk-ports/src — git ls-files lists only tracked paths, so run this after git add; a count of 0 here is a fake 0"; exit 1; }   # THE POSITIVE CONTROL: without it an empty file set is indistinguishable from a clean crate (Halt 8 shape (g))
hits=$(printf '%s\n' "$files" | xargs grep -HnE 'async fn|impl Future|-> .*Future|BoxFuture' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//')
[ -z "$hits" ] || { echo "FAIL: async fn / future-returning method declared in a port trait (ADR-0076; AC-528, register :7159-7160)"; printf '%s\n' "$hits"; exit 1; }   # → silent

# ---- the five port traits. Recursive over src/, because this step may not decide the file layout
# ---- (00-preamble.md:53): the house layout is a doc-header lib.rs plus `pub mod` lines
# ---- (crates/rk-types/src/lib.rs:14-25). Anchored on the DECLARATION: a bare `grep -c 'Send + Sync'`
# ---- counts doc-comment and helper lines too and reads 10, not 5, on the F41 reference.
out=$(grep -rhE '^pub trait [A-Za-z][A-Za-z0-9_]*: Send \+ Sync' crates/rk-ports/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the port-trait grep errored (rc=$rc) — crates/rk-ports/src/ is unreadable, so a count of 0 would be a fake 0"; exit 1; }
n=$(printf '%s' "$out" | grep -c . | tr -d '[:space:]')
[ "${n:-0}" -eq 5 ] || { echo "FAIL: ${n:-0} port traits declare Send + Sync, expected 5 (ADR-0099 decision 4)"; exit 1; }
# ADR-0115's narrowing, ASSERTED rather than only described. The supertrait count above reads 5 under
# EITHER holding shape, because ADR-0115:57-58 leaves the supertraits on the trait declaration
# unchanged — so without this line nothing at any level discriminates ADR-0115 from the clause it
# narrows, while AC-480 (register :6912) is in this stage's acceptance set.
# ANCHORED ON THE FIELD DECLARATION, not on the substring: `Arc<Mutex<dyn EventStore>>` appears verbatim in
# §7a.2 of this file, in ADR-0115:57-58 and in design:1704, so the module header §7a.9 asks for satisfies an
# unanchored grep over a crate that does not hold the port that way — s6:249-250's house note, on the ONE
# line that discriminates ADR-0115. The field name is the reference shape's (spikes/s-ports/src/lib.rs:174)
# and §7a.9's `ports_struct_holds_arc_dyn_send_sync` fixes it.
grep -rqE '^[[:space:]]*pub store: Arc<Mutex<dyn EventStore>>' crates/rk-ports/src/ || { echo "FAIL: the Ports struct does not DECLARE store: Arc<Mutex<dyn EventStore>> — ADR-0115 clause 1, narrowing ADR-0099 clause 4; AC-480's AND clause, register :6912"; exit 1; }

# ---- the method sets, as a FLOOR. The counts above are satisfied by five traits carrying the SPIKE's
# ---- method sets, four of which contradict design:265-269 (§7a.3, measured). This does NOT pin a
# ---- signature — no record fixes one for these four ports, and a checkpoint that pinned one would read
# ---- as a decision — it only requires the design's own method NAMES and refuses the two the design
# ---- replaced. `grep -rq` is used, so a missing directory cannot read as a pass: `test -d` above ran.
for m in 'fn project' 'fn inbox' 'fn check_links' 'fn drift' 'fn detect' 'fn lint' ; do
  grep -rqE "^[[:space:]]*$m" crates/rk-ports/src/ || { echo "FAIL: no '$m' declared in rk-ports — design §3.3's responsibility column (design:265-269) is the method-set authority, not spikes/s-ports (§7a.3)"; exit 1; }
done
hits=$(grep -rnE '^[[:space:]]*fn (sync|publish)\(' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: a spike method survived into rk-ports (grep rc=$rc) — Tracker::sync and DocsSite::publish are s-ports' shapes, replaced by design:267-268"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1

# ADR-0118 clause 5: Step 7a writes no lock() call site, so what it owes is the PROHIBITION as an
# assertion. Matched on `into_inner`, NOT on ADR-0115's closure spelling — the one live instance this
# corpus has (crates/rk-store/src/conn.rs:139-140) is in the method-path spelling and a grep for the
# closure text misses it. `catch_unwind` joins the alternation because clause 3 forbids it too, and the
# bare `into_inner` token STAYS because `match lock() { Err(guard) => guard.into_inner() }` contains
# neither `PoisonError` nor `unwrap_or_else`. Two exclusions keep it off a correct crate: a comment line
# (the house pattern is a module header naming the governing record — crates/rk-store/src/lib.rs:1-20 —
# and one that quotes this prohibition carries the token), and the declaration `pub fn into_inner`, which
# is already a house API at crates/rk-git/src/reconciled.rs:41 [measured 2026-09-13]. The residual
# over-width is stated at §7a.4 and is NOT closed silently. SCOPE IS crates/rk-ports/src/ AND MAY NOT BE
# WIDENED to crates/ to reach rk-store's known debt: ADR-0118:210-214 says why. Floor, not evidence;
# Step 7d owes the discriminating test.
raw=$(grep -rnE 'into_inner|PoisonError|catch_unwind' crates/rk-ports/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the poison-recovery search could not run (grep rc=$rc; 2 = no such path) — ADR-0118 clauses 1 and 3"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|pub fn into_inner)')
[ -z "$hits" ] || { echo "FAIL: a poison recovery in rk-ports — ADR-0118 clauses 1 and 3 forbid every spelling of PoisonError::into_inner and a catch_unwind that resumes the run"; printf '%s\n' "$hits"; exit 1; }   # → silent

# ---- ADR-0102 decision 3 (:58-59) is a PROHIBITION, and the clippy gate above CANNOT see it: an
# ---- `#[allow(clippy::…)]` silences the lint and clippy exits 0 over the crate the override is hiding.
# ---- Same standard as the poison line: a prohibition this step cannot exercise is owed as an assertion
# ---- (§7a.4's rule, applied to the record §7a.10 states in prose and asserted nowhere before this pass).
# ---- The pattern does NOT match the ONE permitted form, a commented crate- or module-level
# ---- `#![allow(unsafe_code)]` (ADR-0102 decision 2, :55-57) — that is `#![`, not `#[`.
hits=$(grep -rn '#\[allow(' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: an #[allow] in rk-ports (grep rc=$rc; 2 = no such path) — the only in-source override is #[expect(clippy::<lint>, reason = \"…\")] (ADR-0102 decision 3, :58-59)"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1

# ---- rk-ports re-declares no rk-types shape. THREE warrants, and the message must not conflate them:
# ---- ActivitySpec and DispatchRecord are the RECORD's (ADR-0099:62, "Both are rk-types"), and
# ---- DispatchRecord already exists TWICE on this tree under one name with two incompatible field sets
# ---- (§7a.5), so a third declared here is the collision made permanent. StepError is rk-types' by
# ---- ADR-0095 as ADR-0112 extends it, shipped at Step 5. Typed and SchemaId are THIS STAGE'S ROUTING
# ---- (§7a.5: "routed, not decided … a record naming the home is OWED") — the third line is a floor that
# ---- keeps 7a from pre-empting that record, NOT a record's prohibition, and the FAIL text says so.
hits=$(grep -rnE '^pub (struct|enum) (ActivitySpec|DispatchRecord)\b' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: rk-ports re-declares ActivitySpec or DispatchRecord (grep rc=$rc) — ADR-0099 clause 1, :62: 'Both are rk-types'"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1
hits=$(grep -rnE '^pub enum StepError\b' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: rk-ports re-declares StepError (grep rc=$rc) — rk-types owns it at eleven variants, ADR-0095 as ADR-0112 extends it, shipped at Step 5"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1
hits=$(grep -rnE '^pub (struct|enum) (Typed|SchemaId)\b' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: rk-ports declares Typed or SchemaId (grep rc=$rc) — §7a.5 ROUTES both to rk-types on a stated warrant and NO RECORD DECIDES IT; the record naming the home is OWED and 7a may not pre-empt it in either direction"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1

# ---- AC-483's git-surface denylist, the one criterion in this set fully buildable HERE (rk-git
# ---- landed at Step 6, 7aa02c7). The previous revision asserted in prose that "the port-inventory
# ---- denylist stands" and ran nothing. The grep is the FLOOR; the named test and the trait GitPort
# ---- fixture are the evidence.
hits=$(grep -rnE '^pub trait [A-Za-z]*(Git|Trailer|Worktree|Clone|Reconcile)' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: a port trait names the git surface (grep rc=$rc) — AC-483 register :6957 forbids it and ADR-0105 clause 1 puts GitOps in rk-git"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1
test -f crates/rk-ports/tests/ui/git_port_is_not_a_port.rs || { echo "FAIL: AC-483's compile-fail fixture is absent (register :6958)"; exit 1; }
test -f crates/rk-ports/tests/ui/async_fn_in_a_port_trait.rs || { echo "FAIL: AC-528's compile-fail fixture is absent (register :7162)"; exit 1; }
test -f crates/rk-ports/tests/ui/arc_dyn_port_rejects_rc.rs || { echo "FAIL: the trybuild case pinning doctest fence 1's REASON is absent (§7a.7; design:288)"; exit 1; }
test -f crates/rk-ports/tests/ui/bare_arc_store_has_no_mut.rs || { echo "FAIL: the trybuild case pinning doctest fence 2's REASON is absent (§7a.7; ADR-0115:130-132)"; exit 1; }
# The trybuild suite is what pins WHY each case fails: a bare `compile_fail` is green whenever
# compilation fails for ANY reason, and an expected error code on a doctest fence is INERT at this pin
# (measured 2026-09-13, 1.97.1 and 1.98.1 — §7a.8). Each tests/ui case commits a .stderr.
# THE FILTER IS THE HARNESS TEST'S OWN NAME, NOT THE TOOL'S: cargo's `--` filter is a substring match over
# TEST names, and the house convention names a trybuild harness after its RULE — rk-git's is
# `reconcile_runs_at_every_entry` (tests/it/reconciled.rs:24), rk-store's is
# `a_state_field_without_a_tier_is_a_compile_error` (tests/it/tier_ui.rs:9), and neither contains the
# substring `trybuild` [measured 2026-09-13]. A filter spelled `trybuild` therefore selects ZERO tests and
# cargo exits 0 (`0 passed; N filtered out`), so the previous revision COULD NOT PASS on a crate built as
# §7a.7 prescribes. §7a.7 now fixes the name, and this is the line that consumes it.
out=$(cargo test -p rk-ports --test it -- trybuild_pins_the_reason_each_port_fixture_fails 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trybuild must-fail suite did not pass (rc=$rc) — a case compiled, or its .stderr drifted"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_port_fixture_fails\b.* \.\.\. ok$' || { echo "FAIL: no test named trybuild_pins_the_reason_each_port_fixture_fails ran and passed — cargo's -- filter is a SUBSTRING match over TEST names, so an aggregate 'test result: ok' line is satisfied by a near-miss name and a filter matching nothing exits 0 (s6:211-217's repaired form)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }

# ---- AC-481's parity, in three parts: the assertion EXISTS, it RAN under its own name, and the sets
# ---- it parses are the five. The old form asserted only `test result: ok. N passed`, which cargo's
# ---- SUBSTRING filter satisfies from a near-miss name — the exact defect Checkpoint 6 repaired at
# ---- s6:211-217. Nothing asserts the test CAN fail: that is AC-481's sixth-trait fixture
# ---- (register :6923-6925), which is the evidence while this grep is the floor.
test -d crates/rk-ports || { echo "FAIL: crates/rk-ports absent"; exit 1; }
# `-rn`, and the hit is PRINTED: for a criterion whose whole point is that the assertion exists and is
# evaluated, the checkpoint's recorded output must carry the assertion's file, line and text and not only a
# count. HEAD printed it; the intervening `grep -rl … | wc -l` form did not, and PROGRESS.md would have
# carried a number where a citation belongs.
out=$(grep -rn 'traits.len() == fakes.len()' crates/rk-ports/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the parity-literal grep errored (rc=$rc) — crates/rk-ports/ is unreadable, so a count of 0 would be a fake 0"; exit 1; }
# COMMENT LINES ARE EXCLUDED, and without this the line CANNOT FAIL for the property it names — measured
# on the first real run, 2026-09-13. The crate carries the literal on two COMMENT lines
# (tests/it/parity.rs:50 and tests/it/source_parse.rs:43, both explaining the rule), so deleting the
# assertion at tests/it/parity.rs:56 leaves this grep green. That is the SIXTH assertion in this
# programme to be satisfied by prose naming the thing it governs (five in Step 6, ci.yml:186-190), and
# the first one this block introduced while fixing the other five. NOTE THE FILTER'S SHAPE: grep prints
# `path:line:content` here, so the line starts with the PATH and `grep -vE '^\s*//'` can never match.
out=$(printf '%s\n' "$out" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//')
[ -n "$out" ] || { echo "FAIL: the AC-481 parity assertion is absent from NON-COMMENT source — it must be spelled 'traits.len() == fakes.len()' (design §3 P2, design:280; register :6923). assert_eq! does not contain this string, and a comment quoting it is not the assertion"; exit 1; }
printf '%s\n' "$out"   # → the assertion's location, into the checkpoint's record
out=$(cargo test -p rk-ports -- port_fake_parity 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test port_fake_parity exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\bport_fake_parity\b.* \.\.\. ok$' || { echo "FAIL: no test named port_fake_parity ran and passed — the filter is a SUBSTRING match"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# The five fakes, counted. The trait count above and the parity test's own sets are otherwise
# UNLINKED: a parity test over 3 parsed traits reports 3 == 3 and passes. F41 ships two library fakes
# (FakeHarness, VecStore); Tracker, DocsSite and Toolchain fakes are authored here (register :6917).
out=$(grep -rhE '^impl (EventStore|Harness|Tracker|DocsSite|Toolchain) for ' crates/rk-ports/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the port-impl grep errored (rc=$rc) — crates/rk-ports/src/ is unreadable, so a count of 0 would be a fake 0"; exit 1; }
n=$(printf '%s' "$out" | grep -c . | tr -d '[:space:]')
[ "${n:-0}" -eq 5 ] || { echo "FAIL: ${n:-0} port impls in rk-ports/src, expected 5 — one fake per port (ADR-0003; register :6917 names all five by shape)"; exit 1; }

# ---- the two compile_fail doctests. The `-ge 2` floor does not compose with the fence count into the
# ---- property claimed: a fence whose info string is `compile_fail,ignore` matches the fence regex, contributes
# ---- 0 to `passed`, and its slot is made up by an unrelated doctest — both lines green, one rule
# ---- never checked. The zero-ignored line closes that, stated as the rule it is.
out=$(cargo test -p rk-ports --doc 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --doc exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' | head -1)
[ "${n:-0}" -ge 2 ] || { echo "FAIL: ${n:-0} doctests passed against the 2 compile_fail fences asserted below — one of them never ran (doctest = false runs 0 and still exits 0)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\..*; 0 ignored' || { echo "FAIL: a doctest was IGNORED — an ignore-marked compile_fail fence satisfies the fence count below and asserts nothing"; printf '%s\n' "$out" | tail -20; exit 1; }
# THE DIRECT ASSERTION, which the aggregate above and the fence count below do NOT compose into: a fence in
# a cfg-gated or otherwise uncollected module contributes 0 to `passed` AND 0 to `ignored`, and any two
# ordinary doctests make up its slot — all three lines green, one rule never checked. rustdoc's own naming
# for a compile_fail doctest is `test src\lib.rs - (line N) - compile fail ... ok` [measured 2026-09-13 on
# the pin, a three-fence probe crate: two compile_fail fences and one ordinary fence print exactly that,
# and `test result: ok. 3 passed`]. A filter matching nothing would read 0 here and fail LOUDLY, which is
# the safe direction.
n=$(printf '%s\n' "$out" | grep -cE '^test .* - compile fail \.\.\. ok$' | tr -d '[:space:]')
[ "${n:-0}" -eq 2 ] || { echo "FAIL: ${n:-0} compile_fail doctests RAN and passed, expected exactly 2 — the aggregate count above and the fence count below do not compose into this"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
out=$(grep -rnE '[`]{3}compile_fail' crates/rk-ports/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the compile_fail fence grep errored (rc=$rc) — crates/rk-ports/src/ is unreadable, so a count of 0 would be a fake 0"; exit 1; }
n=$(printf '%s' "$out" | grep -c . | tr -d '[:space:]')
[ "${n:-0}" -eq 2 ] || { echo "FAIL: ${n:-0} compile_fail doctests in rk-ports/src, expected exactly 2 — the Rc-holding fake and the bare-Arc store (§7a.8). AC-480's generics-leak fixture is 7b's, and AC-483's and AC-528's are trybuild cases under tests/, deliberately outside this count"; exit 1; }

# ---- CancelToken and ChangeSummary, anchored on the DECLARATION. Both were substring/line matches a
# ---- COMMENT satisfies (s6:249-250, the house note) — and the sentence THIS STAGE FILE writes at
# ---- §7a.2 is exactly such a line for CancelToken, while two comment lines quoting the newtype would
# ---- make the ChangeSummary count 2 and halt the block on a correct crate.
grep -rqE '^[[:space:]]*pub struct CancelToken' crates/rk-ports/src/ || { echo "FAIL: CancelToken is not DECLARED in rk-ports — design:1704's rk-ports row names the CancelToken as this crate's content; ADR-0099 decision 2 makes the composition root its owner, which is why it is never an adapter local, but it names no crate"; exit 1; }
out=$(grep -rhE '^[[:space:]]*pub struct ChangeSummary\(pub\(crate\) \(\)\);' crates/rk-ports/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the ChangeSummary grep errored (rc=$rc) — crates/rk-ports/src/ is unreadable, so a count of 0 would be a fake 0"; exit 1; }
n=$(printf '%s' "$out" | grep -c . | tr -d '[:space:]')
[ "${n:-0}" -eq 1 ] || { echo "FAIL: ${n:-0} 'pub struct ChangeSummary(pub(crate) ());' declarations, expected exactly 1 — ADR-0099 clause 5 (:90-91) decides the type is an opaque, non-finalised newtype in rk-ports and that this is the ONE site in crates/ the name may appear (:131); this exact SPELLING is not the record's but F41:20's and spikes/s-ports/src/lib.rs:163's, carried forward so the grep has one form to match"; exit 1; }

# ---- AC-497's, AC-499's and AC-500's fake-side instruments, which a grep over a type declaration cannot see.
for t in toolchain_fake_classifies_binary harness_fake_produces_unsupported git_surface_denylist_rejects_git_port ports_struct_holds_arc_dyn_send_sync port_conformance_fake_rows ; do
  out=$(cargo test -p rk-ports -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: rk-ports $t exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: no test named $t ran and passed — the -- filter is a SUBSTRING match"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done

# ---- AC-528's rk-ports half. `-e normal` deliberately excludes dev-dependencies, matching register
# ---- :7161's normal-graph wording; it must not later be widened to --all-features. NOTHING IS RUN ON
# ---- rk-kernel HERE: measured on this tree 2026-09-13, `cargo tree -p rk-kernel -e normal` exits 101
# ---- with "package ID specification `rk-kernel` did not match any packages", which halts.md:85-86
# ---- makes a FAILED checkpoint — see §7a.11.
out=$(cargo tree -p rk-ports -e normal); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-ports exited $rc — nothing was resolved, so a count of 0 would be a fake 0"; exit 1; }
# THE PARENTHESISED PATH IS STRIPPED BEFORE MATCHING. `cargo tree` renders every workspace member as
# `<name> v<ver> (<absolute checkout path>)`, so the denylist is matched against this box's directory
# names as well as against crate names: a repository cloned under a path containing `futures` or
# `tokio` would FAIL a correct crate. That is the Step-6 over-width class exactly (a correct crate read
# wrong because of an incidental text property, s6:222-225), and it is cheaper to remove than to
# document. Registry dependencies carry no path, so nothing an async runtime would arrive as is lost.
if printf '%s\n' "$out" | sed 's/ ([^)]*)$//' | grep -qE 'tokio|async-std|futures|async-trait'; then echo "FAIL: rk-ports pulls an async runtime (AC-528, ADR-0076)"; printf '%s\n' "$out" | sed 's/ ([^)]*)$//' | grep -nE 'tokio|async-std|futures|async-trait'; exit 1; fi   # → silent: → 0

# ---- design:1783 makes three of this step's assertions CI's. The floor is that they landed; whether
# ---- they RAN is a different question, and PROGRESS.md:307 records that ci.yml has never run.
test -f .github/workflows/ci.yml || { echo "FAIL: ci.yml absent"; exit 1; }
# Same defect, second instance — and a comment filter is NOT enough here, which is the finding.
# Falsified 2026-09-13 by deleting ci.yml's real parity row: the block still exited 0. Three lines in
# ci.yml carry the literal — :205 a COMMENT, :227 the grep that IS the rule, and :228 the rule's own
# `echo "FAIL: …"` MESSAGE. The message is not a comment, so excluding comments leaves the assertion
# satisfied by the error text of the very rule it exists to prove is present. **An assertion that its
# target's own failure message satisfies is a new Halt 8 shape, and this is the first instance in this
# programme.** So the anchor is the INVOCATION, not the literal: if the rule is deleted its grep line
# goes with it, while neither a comment nor a FAIL string can invoke anything.
out=$(grep -n "grep -rq 'traits.len() == fakes.len()' crates/rk-ports/" .github/workflows/ci.yml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the ci.yml parity-invocation grep errored (rc=$rc)"; exit 1; }
out=$(printf '%s\n' "$out" | grep -vE '^[0-9]+:[[:space:]]*#')
[ -n "$out" ] || { echo "FAIL: ci.yml does not INVOKE the AC-481 parity grep over crates/rk-ports/ — design:1783 names it as a CI structural assertion, and a comment or a FAIL message quoting the literal is not the rule"; exit 1; }
grep -q 'crates/rk-ports/src' .github/workflows/ci.yml || { echo "FAIL: ci.yml carries no rk-ports structural rule — AC-483's git denylist and AC-528's async denylist are CI's (design:1783)"; exit 1; }
```

**"The assertion present and passing" was an intention; the parity group above is it as commands** — the
`traits.len() == fakes.len()` line proves the assertion EXISTS, the `port_fake_parity` capture proves it RAN under
its own name and passed, and the `impl … for` count links the test's sets to the five ports rather than leaving a
parity test over three parsed traits to report `3 == 3`. The number matters (AC-481) because a fake that quietly
stops tracking its trait is exactly what this catches, and a `traits.len()` that is never evaluated catches
nothing. **What none of these lines proves is that the test can fail**: that is the sixth-trait fixture of §7a.9,
and it is the evidence while these are the floor.

### 7a.11 Acceptance set

The register assigns this step its set at register :6902 — *"Runbook Step 7a's acceptance set is the Phase-1 subset
of this area … **AC-480–AC-495, AC-497–AC-500, AC-509, AC-511, AC-518, AC-528**"* — and under ADR-0092 that
assignment outranks this file. **This section does not re-scope it. It states, per id, which criteria Checkpoint 7a
discriminates and which it carries**, because golden constraint 10's form is that powerless is reported as
powerless and never as a pass, and golden constraint 12 — the **Discriminate** bullet, `00-preamble.md`:178-179,
*"Every AC in the stage's acceptance set needs a test that fails when the behaviour is absent"* — requires a
discriminating test per claimed id **at the stage**. *(Re-measured 2026-09-13: the previous revision cited :175-176,
which is the tail of the **Red** bullet. The ADR-0118 pass moved the preamble's bullets by +3 and this citation was
not re-measured with its siblings.)* The set is written as an explicit range minus the ids it excludes, each named
with its reason (`00-preamble.md`:227-229; **PIA-071, PIA-070** — the two audit concerns that give every Phase-1
stage's acceptance set this shape, and that the sibling sets still carry by name at `s7b`:238 and `s7c`:193).

**DISCHARGED HERE, by a discriminating test at this stage:**

- **AC-481 — its number and its by-name message; its FIXTURE is ARMED, not discharged.** The source-derived parity
  count (register :6921) and the failure that names the missing trait (:6922) are discharged by `port_fake_parity`
  and the parity group of Checkpoint 7a, per §7a.6. **The PIA-059 fixture — a sixth trait with no fake, editing no
  inventory (register :6923-6925) — is the half that shows the test CAN fail, and §7a.9 does not name it**: its
  name, path and feature gate get the treatment DECISIONS-2026-09-12 E4 gave the two must-fail fixture crates, and
  that decision has not been taken for this one. Until it is, the fixture is **ARMED, NOT DISCHARGED**, no
  checkpoint line asserts it, and **AC-481 is claimed in part** — which is what the parity group's own comment
  already records. Write the checkpoint assertion in the same commit as the fixture's name. *(The previous revision
  claimed AC-481 "in full" on three components while §7a.9 declined to provide one of them. This is the one place
  the file stated a gap in one section and claimed the opposite in another.)*
- **AC-480 — its AND clause only** (register :6912): every port held as `Arc<dyn Port + Send + Sync>`, the
  EventStore port as `Arc<Mutex<dyn EventStore>>`, `Send + Sync` supertraits on all five, a bare trait object and
  not a newtype. Discriminated by the supertrait count, the `Arc<Mutex<dyn EventStore>>` assertion,
  `ports_struct_holds_arc_dyn_send_sync` and `tests/ui/bare_arc_store_has_no_mut.rs`.
- **AC-483 — its `rk-ports` half**: no trait in `rk-ports` abstracts git, and a trait matching the git surface is
  rejected (register :6956-6957), by the denylist test and the `trait GitPort` fixture. **This is the one id in
  this step's set whose instrument is fully buildable here.**
- **AC-497 — the port shape**, which the register says *is* asserted (*"only the port shape below is asserted,
  never the type's contents"*, register :7107-7108): the opaque `ChangeSummary` newtype **and** the fixture-driven
  Toolchain fake returning `Binary` over a binary fixture, which is the first of the row's two named instruments
  (:7117). **`change_summary`'s BODY is not written at this step** — the trait method is declared and the fake
  returns `ChangeSummary(())`; no real implementation is authored here. `ChangeSummary`'s **field set** stays
  PROVISIONAL on S-BIN and its `change_summary` body is 7c's
  (ADR-0093 decision 4, :81-84, which scopes S-BIN's gate to that type and that body and *"does not block Phase-1
  commits, `rk-types`, `rk-store`, `rk-git`, the kernel or the adapters"*).
- **AC-499 — its fake half only**: the shared per-port conformance suite is authored here with the fake
  instantiation wired and each real instantiation a named, empty row, **discriminated by `port_conformance_fake_rows`
  in Checkpoint 7a's named-test group** (§7a.9). *(The previous revision listed AC-499 as discharged here and named
  no test, no file and no checkpoint line for it — a discharge with no instrument, which is what golden constraint
  12's Discriminate bullet forbids. The suite's fake half is buildable at 7a, so the instrument is added rather than
  the claim downgraded.)* **Carry the register's qualifier, which this
  file previously dropped:** *"The Harness real row runs a real dispatch and therefore needs the CI decision
  ADR-0010 defers — until it is taken, that row is a **local-only, named-runner** row"* (register :7140-7141). It
  may not be wired into CI here or at 7c. **The register is self-inconsistent here and this step states it rather
  than choosing:** AC-499 says *"the §14 State line carries the dependency"* and the State line at register :6899
  carries no AC-499, no local-only row and no ADR-0010 dependency. **An amendment to register :6899 is owed.**
- **AC-500 — its fake half only**: the scripted responder produces `StepError::Unsupported(capability)` carrying
  the token (register :7149's *"an integration test on the fake"*).
- **AC-528 — its `rk-ports` clause only**: no port trait declares an `async fn` or a future-returning method
  (register :7159), by the comment-filtered, positively-controlled denylist plus `cargo tree -p rk-ports -e normal`
  and AC-528's two must-fail fixtures.

**ARMED, NOT DISCHARGED — by id and by reason** (golden constraint 10; the in-tree precedent is `PROGRESS.md`:307,
*"AC-005 is discharged only in PART"*). At 7a, `crates/` holds `rk-git`, `rk-store`, `rk-types` and the `rk-ports`
this step writes [measured 2026-09-13]. **There is no `rk-kernel`, no `rk-adapter-*`, no `rk-cli` or `doctor`, no
pack and no lane run.** Checkpoint 7a may not be read as discharging any of the following, and each is recorded in
`PROGRESS.md` §1 as carried forward:

| id | Register subject and instrument | Missing at 7a | Where it can run |
| --- | --- | --- | --- |
| **AC-484** | `rk-kernel` (:6963); *"a `cargo metadata` dependency-graph assertion in CI; a fixture commit adding `rk-adapter-sqlite`"* (:6968) | `rk-kernel` | **7b** — `s7b`:168-169 already runs it; 7b's own set does not name the id, so 7a is its only claimant |
| **AC-485** | `rk-cli`, `rk-kernel` (:6971); a mid-run config-mutation integration test (:6976) | `rk-cli` | **7d** — claimed by id at `s7d`:117 |
| **AC-486** | `rk-cli` (`doctor`), `rk-kernel` (:6979); startup-resolution test + a `doctor` rule row (:6984) | `rk-cli`, `doctor` | **7d** — claimed by id at `s7d`:117 |
| **AC-487** | `rk-kernel` (:6989); *"a kernel lint (denylist: adapter crate names, `downcast_ref`, `downcast_mut`); a compile-fail fixture"* (:6995) | `rk-kernel` | **7b** — **this id appears in no runbook file by name and inside no other stage's declared range** [measured] |
| **AC-488** | `rk-adapter-claude` (:7000) | the Claude adapter | **7c** builds it; **7d** claims it by id at `s7d`:117 |
| **AC-489** | `rk-adapter-*` capability registry (:7027); a build-time registry schema test (:7032) | any adapter | **7c** |
| **AC-490** | `rk-cli` (`doctor`) (:7035); a `doctor` rule row over a back-dated registry (:7040) | `doctor` | **7d** |
| **AC-491** | `rk-cli` (`doctor --probe`) (:7043) | `doctor` | **7d** |
| **AC-492** | `rk-adapter-*`, build (:7053); a build-time vocabulary test (:7058) | any adapter | **7c** |
| **AC-493** | `rk-adapter-claude`, `rk-plugin-gen` (:7061); a tool-mapper unit test (:7067) | the Claude adapter; the `rk-plugin-gen` half is **Phase 2** with AC-501–AC-506 | **7c** |
| **AC-494** | `rk-kernel`, `rk-types` (:7074); a resolver unit test (:7079) | `rk-kernel` | **7b** |
| **AC-495** | `rk-kernel` pack validation (:7082); a pack-validation test over two fixture packs (:7087) | `rk-kernel`, packs | **7b** |
| **AC-498** | kernel test suite (:7122); *"a CI job with network egress denied and no adapter crates linked; a dispatch counter asserted at 0"* (:7127) | `rk-kernel` | **7b** |
| **AC-509** | `rk-adapter-claude`, dispatch (:7242); *"an integration test spawning a fixture child that echoes its environment; a `doctor` rule-table row"* (:7247) | the Claude adapter, `doctor` | **7c / 7d** — **claimed by no stage but this one** [measured] |
| **AC-511** | `rk-adapter-claude` (:7008); a source lint over argv construction + an event-log assertion (:7015-7016) | the Claude adapter | **7c** — `s7c`:185 already runs the one-dispatch probe, but 7c's set does not claim the id |
| **AC-518** | `rk-kernel`, `rk-git` (:7311); *"an integration test asserting the user checkout is unchanged across a full lane run"* (:7316) | `rk-kernel`, a lane | **Step 8** — **claimed by no stage but this one** [measured]. It names no port, so there is no port-shape half of it to carry here |

**Plus these halves of ids discharged in part above:** AC-480's first THEN clause and its API-surface lint over
`rk-kernel` (register :6911, :6914) — **7b**, where `s7b`:62 names it in prose only (*"no generic parameter bound by a port trait (AC-480)"*) and no checkpoint block in any
stage file carries it; AC-483's *"`rk-kernel` depends on `rk-git` as a concrete crate"* (:6956) — **7b**; AC-497's
input-assembly test, whose WHEN is *"the review activity's inputs are assembled"* (:7114), the kernel's — **7b**;
AC-499's four `{fake, real}` matrix rows (:7139-7140) — **7c**, whose set does not claim AC-499 today, so **adding
it to `s7c`'s set is owed**; AC-500's route-around path and its mutation-style check (:7149) — **7b/7e**, both of
which need a pack.

**AC-528's `rk-kernel` clause is asserted by nothing, anywhere, today — and that is recorded rather than
smoothed.** *(This paragraph replaces one that cited a reconciliation item **"R11"**. That id resolves to no
reconciliation register in this corpus: repo-wide, the only other `R11` is an unrelated risk row at
`references/2026-08-31-rust-inversion-RFC.md`:254 [measured]. **The id is withdrawn and no replacement is
minted** — there is no bare-`R<n>` namespace here; the live ones are `PIA-NNN`, `PROGRESS.md` §2's `C`-rows,
`ERRATA.md`'s ADR-keyed `E-NNNN-N`, and risk ids like §7a.1's `R-ASYNC`. The paragraph also directed a correction
at the register. **The register is right and nothing is owed to it for the scope claim**: AC-528's clause names
`cargo tree -p rk-kernel -e normal` (register :7161), ADR-0076's forbids clause says the same of the kernel, and
`s5` §5.2's CI table says `rk-kernel`; under ADR-0092 all three outrank this file. What is owed is the
**stage-assignment**, which is execution order and this runbook's.)*

- **Nothing is run on `rk-kernel` here.** `cargo tree -p rk-kernel -e normal` exits **101** on this tree with
  ``error: package ID specification `rk-kernel` did not match any packages`` [measured 2026-09-13], and
  `halts.md`:85-86 makes a checkpoint line you could not run a **failed checkpoint**. The previous revision's
  instruction to *"run it on both here"* was unexecutable.
- **It does not qualify for the `[WILL EXIST]` carve-out either,** and the reason is stated rather than assumed:
  `halts.md`:89-91 requires a `[WILL EXIST]` line's expectation to be written as *"exit != 0 AND stderr names the
  failing clause"*, and **a missing package satisfies neither half** — cargo's message names the package, not the
  clause. So no line for it is added to Checkpoint 7a, marked or unmarked.
- **And no block in this runbook asserts it.** Checkpoint 7b's `cargo tree -p rk-kernel -e normal` capture
  (`s7b`:168) is consumed for AC-484's `rk-adapter-` count at :169; 7b's async check is a manifest grep over
  `crates/rk-kernel/Cargo.toml` (`s7b`:165), which sees **direct dependencies only** and not the *"directly or
  transitively"* the clause requires; `.github/workflows/ci.yml` carries no `cargo tree` line; and
  `grep -rn 'AC-528' runbook/` returns this file alone [all measured 2026-09-13].
- **Owed, and named:** the cross-listing of AC-528's `rk-kernel` clause — and of its first THEN clause's second
  half, *"a trait that does fails to compile against the trait-object bound `rk-kernel` holds"* (register
  :7159-7160), which a source grep cannot discharge — into **Checkpoint 7b**, with an assertion over the resolved
  graph; **the substitution of the named-test form for the aggregate `test result: ok. N passed` at `s7b`:157 and
  `s7b`:161**, which still carry the form `s6`:211-217 repaired and this block replaces throughout — `s7b`:159-162
  additionally filters on `-- trybuild`, which selects zero tests against both existing harness names and exits 0,
  the same defect §7a.7 repairs here [measured 2026-09-13]; and the reconciliation of register §14.3's
  Phase-1-subset note (:6902) with the sixteen ids above.
  **`s7b`'s acceptance set is not edited in this pass.** The reconciliation is owed to the operator, against the
  register and against `s7b`/`s7c`/`s7d`, and a bare deferral that names no destination converts a mis-assignment
  into an orphan — `s7b`:238-259 and `s7c`:193-203 claim **none** of the sixteen today [measured].

**Excluded, by id and reason:**

- **AC-501–AC-506** — Phase 2, `rk-plugin-gen` (register :6902). **AC-502 is inside this range and is not listed
  again**: it is `rk-plugin-gen`'s per-dialect artifact-parity criterion (register :7176; its THEN at :7179 reads *"the
  per-dialect artifact counts are equal and each source entry is present in every dialect"*) and names neither S-CODEX nor a second harness. The previous revision
  excluded it a second time on a warrant that belongs to AC-496.
- **AC-507 / AC-508 / AC-510** — Phase 3 (GitHub and mdBook).
- **AC-512 / AC-514–AC-517 / AC-519 in part** — CONDITIONAL; the receipt and rename halves are unbuildable until a
  distribution mechanism is decided (register :6902).
- **AC-496** — PROVISIONAL: its **pass bar** rests on S-CODEX, registered and unrun, and the Codex adapter is
  **Phase 2** per ADR-0094 decision 2, behind the CI decision ADR-0010 defers (register :7093-7098; ERRATA
  E-0094-1). Its config-time refusal and `doctor` row are implementable today and the register routes them to
  **7b/7d** (PIA-072); only the Codex-present half waits.
- **AC-482** — human-gated: *"routed to a human gate; not machine-decidable"* (register :6949-6950); the
  machine-checkable half is AC-481 (:6951), which this step discharges **in part** — its number and its by-name
  message, with the PIA-059 fixture armed and not discharged (above).
- **AC-513** — retired; never reuse.
- **AC-239** — not inside this step's claimed range on any reading, and **its stage is disputed; this step raises
  it and does not settle it.** The register assigns the list-is-data half to *"the runbook's **7b/7d** acceptance
  sets (PIA-072)"* (register :3588-3589) and the id is PROVISIONAL on S-CODEX (:3575). Measured: **`s7c`:193 is the
  only set that contains it**, via AC-200–AC-239 minus AC-207; **no `s7b` range reaches it** — `s7b`:238-255 runs
  AC-001–AC-039, AC-040–AC-079, AC-080–AC-118, AC-520/AC-525/AC-527/AC-530, AC-455 and then AC-240–AC-359, so the
  gates range opens one id past it — and `s7d`'s set does not name it. *(The previous revision said "kept, in 7b's
  set", which is false; the revision after it said `s7b`'s ranges START at AC-240, which is also false — the
  conclusion held on a stated reason that did not.)* The allocation is owed a
  correction in `s7b`'s or `s7d`'s set, not here.
- **AC-005's second half** — handed forward by Step 5 (`PROGRESS.md`:28 and :307: *"only the `[bans].deny`
  mechanism is exercised here; the `rk-adapter-*` entries and the `cargo tree -i` assertion belong to 7a/7c"*).
  **Excluded here, and it is 7c's**: the remainder is `[bans].deny` entries for `rk-adapter-*` crates and a
  `cargo tree -i` assertion over **`rk-kernel`'s** graph (register :281-284, whose subject line is `rk-kernel`),
  and neither `rk-kernel` nor any `rk-adapter-*` exists at 7a. **No `cargo tree -i` line is added to Checkpoint
  7a**: measured on this tree 2026-09-13, `cargo tree -i durable` exits **101** with ``error: package ID
  specification `durable` did not match any packages``, so the condition that should pass
  — the banned crate's absence — makes the command fail, and inverting the guard makes it Halt 8 shape (g). Naming
  the handoff is all that is owed **of this checkpoint**; it was named by neither stage before this revision.
  **But naming a destination is not routing it there:** `grep -rn 'AC-005' runbook/` returns `00-preamble.md`:39,
  `PROGRESS.md`:28/:306/:307 and `s5` only — `s7c`:193-203 does not claim it [measured 2026-09-13] — so **adding
  AC-005's `cargo tree -i` half to `s7c`'s acceptance set is OWED**, on the same ground as AC-499's above, and
  `s7c` is not edited in this pass. A bare deferral that names no destination converts a mis-assignment into an
  orphan, and this file states that standard nine bullets above.

The `rk-port-toolchain` label on register rows is a **subject tag** for this crate, not a crate name (design §9.1 is
the crate authority, design:1695; the mapping is the register's §0).

**One more thing this step's entry and exit gates do not carry, recorded and not repaired here:** `INDEX.md`:61
gives Step 7b's entry gate as the 🛑 STOP banner alone, so **Checkpoint 7a is the only exit checkpoint in the chain
that is no stage's entry gate** [measured, INDEX.md:52-66 read column by column; corroborated at
`PROGRESS.md`:31 and `s7b`:5]. The runbook's own combined form exists one row above at `INDEX.md`:58 (*"🚦 gate +
Checkpoint 2"*). Repairing it edits `INDEX.md`, `PROGRESS.md` and `s7b`, none of which is this file, so it is
**owed**, not done.

**Spikes this step consumed:** S-PORTS (F41, done). **Spikes it owes:** none — S-BIN is 7c's `change_summary` body,
and its gate reaches no further (ADR-0093 decision 4, :81-84).
