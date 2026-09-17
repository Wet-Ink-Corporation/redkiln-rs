# Step 7b-ii — the three gates (Phase 1, part 3; `rk-kernel`'s gate half)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7b-ii` | Phase 1 | Checkpoint 7b | Checkpoint 7b-ii | `s7b` | `s7c` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§7b-ii.N` in this file means this file's own section; another stage's section is cited as `s7b §7b.4`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

**Why this is a file and not a paragraph.** Until 2026-09-13 the whole of this stage was seven lines of prose inside
`s7b` with six checkpoint assertions behind it, against an acceptance set of **120 criteria in force** — the register's
three gate areas, `human-gate` AC-240..AC-279 plus AC-524 (**40 in force**), `review-gate` AC-280..AC-319 plus AC-530
(**40**), `verify-gate` AC-320..AC-359 (**40**), read off the register's own area keys [measured 2026-09-13]. That is
the shape in which criteria are claimed on greens nothing pins — the failure ADR-0019 exists to stop and that
`00-preamble.md`:186-189 names in one line: *"Asserting that a test named `X` exists and exits 0 is satisfied by an
implementation nothing pins … The checkpoint is the floor, not the evidence."* The split is **execution order**, which
this runbook owns; it decides nothing the record does not, and `INDEX.md`'s stage table, its milestone table and Step
7c's entry gate move in the same change.

---

> ### 🚦 GATE CHECK
> ```bash
> ( ls D:/repos/redkiln-rs/references/adr/0090-*.md >/dev/null 2>&1 \
>     || { echo "GATE OPEN — no ADR-0090. Go back to Step 4; no work in this stage may start."; exit 1; }
>   echo "GATE DISCHARGED — proceed" )   # a subshell: a pasted FAIL must not kill an interactive shell
> ```
> **Golden constraint 1 reaches this stage through one clause and one only.** **AC-302** is the standing negative:
> *"the [review] tier derives only from the authored stage rigor weight and authored facets, and no LLM-emitted context
> vector is an input"*, verified by *"a signature/type test that `resolve_review_tier` takes no model-produced value;
> an architecture lint on the call graph"*. The record of the block is **ADR-0090** clauses 1 and 2, which supersede
> ADR-0079 **in full** (ADR-0090 decision 3); ADR-0079 is `status: superseded` and is history, not authority.
>
> **And one standing prohibition on what this stage's own close-out may say.** **ADR-0101 clause 5**: the
> prior-approval framing arm **S-SUBTLE-CB** is registered and unrun, so *until it runs no document may claim the
> review gate is guarded against false approval*, and no FP rate from S-VAR or S-TYPED is evidence of overall
> calibration. AC-312 carries the same sentence in the register. **Checkpoint 7b-ii's pass says the gates exist and
> fail closed. It may not be written up as a false-approval guarantee** — not in `PROGRESS.md`, not in a commit
> message, not in the milestone row it gates.

### 7b-ii.1 What this step builds, and what it may not decide

Three gates, all of them `rk-kernel`'s, all of them built on the crate `s7b` created:

1. **The human gate** — the record `(token, tree_hash, deadline)`, its single write-tree derivation, and the fresh
   record a loop-back re-reach mints (register §8).
2. **The review gate** — the classifier over ADR-0096's typed repro, the policy table, re-execution, quarantine, the
   advisory sink, the events and the replay determinism check (register §9).
3. **The verify gate** — the per-AC mutation harness over author-declared mutants, the bootstrap comparator and the AC
   evidence ledger (register §10).

**The record for the gates' behaviour is ADR-0074** (accepted; superseded *in part* by ADR-0096, for the typed repro
and the class computation only). It is the `decisions:` record on most of AC-280–AC-296 and AC-304–AC-313, and
**ADR-0104 decision 4 reads its clause 6 for quarantine**. Read it clause by clause before writing the policy table or
the re-execution path; ADR-0096 replaces only what its own decision 1 replaces. **ADR-0017** is the record for the gate
record itself — *"a single-use token, the tree hash the human saw, and an expiry with a terminal path (fail / escalate
/ auto-decide, never a hang); an answer whose tree hash no longer matches is void, not reconciled"* — and its rejected
option is literally reconciliation.

**What this step may not decide** (`00-preamble.md`:55 — *"Where a stage is tempted to decide something else, it
stops."*):

- **Nothing about the crate layout that `s7b` §7b.4 did not already state.** `rk-select`, `rk-catalog`, `rk-packs`,
  `rk-policy` and `rk-kernel-verify` are **modules of `rk-kernel`**, not crates — `references/2026-09-08-architecture-design.md` §9.1
  is the crate-name authority under ADR-0099 clause 5, and the register's `rk-*` label after each id is a **subject
  tag** mapped in its §0. So `cargo test -p rk-kernel -- <module>` is the invocation for every criterion labelled
  `rk-select`, `rk-catalog`, `rk-packs`, `rk-policy` or `rk-kernel-verify`.
- **Nothing that lands a schema in a closed crate without disposing of the pass it disturbs.** Fourteen ids in the
  claimed set carry `rk-types`, `rk-store` (`rk-store-artifacts`), `rk-ports` (`rk-port-toolchain`) or `rk-git` subject
  tags — §7b-ii.12 enumerates them. `00-preamble.md`:232-237: *"A checkpoint that changed after it passed is
  provisional … Mark the row `passed <hash>, re-run pending <reason>` in `PROGRESS.md` §1 and re-run the hardened
  block."* Any edit inside `rk-types`, `rk-store`, `rk-git` or `rk-ports` made here makes Checkpoint 5, 6 or 7a
  provisional and the row is marked **in the same commit**.
- **Nothing about where a criterion is allocated.** Where a criterion is orphaned or double-claimed, this file
  **raises it and does not settle it** (`s7a`:1217, :1220-1221 — *"naming a destination is not routing it there"*;
  *"a bare deferral that names no destination converts a mis-assignment into an orphan"*). §7b-ii.14 names each one.

**Line numbers into the AC register are not used in this file.** The register was under concurrent amendment on
2026-09-13 and its line numbers moved by **+39** during this stage's authoring — `**AC-244**` read at :3689 in the
audit dossier and at **:3728** on the tree an hour later [measured, both against a working tree `M` from `60f17c6`].
ADR-0119's `source:` key records the same hazard and takes the same route. **Every register citation here is by id and
named clause**, which is stable; where a line reference would be load-bearing, the clause is quoted instead. **The
`00-preamble.md` anchors in this file were re-measured on 2026-09-13 for the same reason** — the ADR-0119 paragraph
shifted every anchor below its `Inputs` block by **+2**, so the numbers here differ from the ones `s7a` and the audit
dossier carry; `s7b` §7b.1 lists the six that moved, and correcting the closed files is owed, not taken here.

### 7b-ii.2 The human gate — the record, the one derivation, the fresh mark

Three criteria, three instruments, and the first two are runnable with nothing but `rk-kernel` and `rk-types`.

- **AC-244 — the record is unconstructible without all three fields.** *"GIVEN a request to open a gate whose record
  lacks a token, a tree hash, or a deadline / WHEN the kernel processes it / THEN the gate is not opened and the run
  fails with a configuration error, leaving no trailer and no event."* Its *Verified by* is **two** instruments —
  *"a type-level test (the gate record is unconstructible without all three fields) plus a runtime test on the
  deserialisation path for records read back from the store"* — and a third, *"a `doctor` rule over existing open
  gates"*, which is `rk-cli`'s and is **ARMED, not discharged** here. The type-level half is the discrimination: a
  happy-path construction test pins nothing.
- **AC-245 — one algorithm, written once, used at gate open AND at answer** (ADR-0105 clause 4). The recorded
  `tree_hash` is the hash of the tree the human read: *"when a checkpoint commit is due, the trailer commit's tree;
  otherwise `git write-tree` on a temporary index built from the current index plus tracked modifications, **untracked
  files excluded**, through the wrapper"*. *Verified by* a unit test that the rendered tree and the recorded hash
  resolve to one object id, **plus a property test over dirty-worktree fixtures** — one with an untracked file whose
  presence must **not** change the hash, one with a tracked modification whose presence **must**. Amended 2026-09-11,
  PIA-050: the hash of an empty commit over a dirty worktree was undefined. The warrant is measured: **F19 found drift
  on 12/174 gates**, so a second derivation site is reachable in normal operation, not a theoretical risk.
- **AC-524 — a loop-back re-reach mints a fresh record** (ADR-0105 clause 10; ADR-0087). *Verified by* a kernel test
  over a fixture graph whose gate sits inside a two-iteration loop, asserting **two distinct tokens, two `gate_opened`
  events, the first token refused on the second reach, and the per-`(item, gate step)` counter at 2**, plus a replay
  assertion that the second mark's tree hash was taken at the re-reach. **PROVISIONAL on P8-B, which gates the LABEL
  and not the BUILD** — the register says the schema recorded the fresh-mark choice "pending P8" and P8-B has not run,
  and that ADR-0105 clause 10 fixes it provisionally until it does. So the fixture graph, the two tokens, the two
  events and the counter are **built and tested here and not claimed as evidenced** (`s6`:351-352's standing form,
  *"Build them; do not claim them"*).

The three properties ADR-0017 decides that no one of those three criteria carries alone are each a criterion inside
the claimed range and each is built: **AC-246** the single-use token (*"the second answer is refused as spent, records
a distinct event, and changes no item state"*), **AC-249** void-not-reconciled, **AC-250** that no reconciliation path
exists. Read ADR-0017 before writing any of them; the words *single-use*, *terminal path* and *void* are its, not this
file's.

**AC-524 is claimed by no other stage.** It is allocated outside the exhausted AC-240..AC-279 range (register §0.4),
`grep -rn 'AC-524' runbook/` returned `s7b`:118 *(at `60f17c6`; that file was rewritten from 264 to over 1,200
lines the same day, so the anchor reads only against the pin)* and `PROGRESS.md`:77 and nothing else before this
file existed [measured 2026-09-13], and Checkpoint 7b's block **carried** a `cargo test -p rk-kernel --
ac524_fresh_record_on_rereach` line from PIA-047 (`s7b`:226-229 at `60f17c6`) against an id in no acceptance set. It
is claimed here, in §7b-ii.14, with its PROVISIONAL label stated. *(Corrected 2026-09-17: this read "Checkpoint 7b
**has run** `ac524_fresh_record_on_rereach` since PIA-047". That was true of the block's TEXT at `60f17c6` and false
of any run: the split commit `80a40da` moved the line out of `s7b` the same day, the block that passed at `ddc57be`
does not contain it, and `git grep -n ac524 -- crates runbook/phase-1/s7b-kernel.md runbook/PROGRESS.md` returns
nothing on this tree [measured 2026-09-17]. The test is first authored at this stage, Slice B, red-first.)*

### 7b-ii.3 The review classifier — one total decision table, and two of the three are binding

**AC-284 — the class is the total decision table and nothing else.** `reproduced` iff `kind = run`, the command ran
and `observed ≠ expected_when_met`; `mutation-survived` iff `kind = mutate`, the patch was applied, the named suite
stayed green **and** the mutant discriminates (a reverted-fix patch, or an `expected.assertion` the gate observed flip
— ADR-0096 decision 2); `narrowed-pin` iff only the discriminating assertion flips, or a surviving mutant has an
assertion that did not flip; `advisory` in every remaining case, **including a surviving mutant with no assertion and
every unrecognised input**. *Verified by* a table-driven unit test with one row per branch **plus an `otherwise` row**;
an **exhaustive `match` over the closed four-variant `EvidenceClass` enum with no `_ =>` fallthrough to a blocking
class**; and a fuzz input that produces `advisory`, never a panic.

**AC-284 alone is PROVISIONAL** (S-CLASS-REPLAY; the register's §9 State line labels only AC-284 in this trio).
**AC-285 and AC-286 are BINDING** and neither is gated on a spike:

- **AC-285** — when the model-supplied `evidence` label disagrees with the computed class, the computed class is used,
  the model label is retained only on the persisted finding, and a `review_class_drift` event carries both. *Verified
  by* a unit test on a disagreeing fixture asserting class, event emission and block set, **plus an architecture lint
  that `finding.evidence_label` has no reader inside the policy module**.
- **AC-286** — 1,000 draws in one process and again in a fresh process give identical results, and the classifier
  reads no clock, no environment variable and no file. *Verified by* a `proptest` determinism property **plus a
  source-level lint over the classifier module forbidding `std::fs`, `std::time`, `std::env` and `SystemTime`/`Instant`
  call sites** — and the register states in terms why the lint is on the module's own syntax: *"a crate-level
  dependency-edge check alone would not catch a transitive route"*. AC-286 is the determinism instrument that answers
  F27's measured verdict flakiness (verdict flaky on 2/4 items over 21 draws; detection 0/7 flaky).

**AC-283 is the load-bearing type fact beside them** and is in the same range: no free-text field is an input type to
either the classifier or `policy::classify`. The register calls the signature/type test *"the load-bearing
instrument"* and the paired property test over rationale *"a corollary of the type [that] cannot fail while the
signature holds"* — so build the signature test and do not substitute the property test for it.

**AC-282 and AC-318 are one behaviour split into two, and both halves are `rk-kernel`'s.** AC-282: a review dispatch
that exits 0 with no `structured_output` block makes the step **fail with a missing-structured-output error and produce
no finding set — empty or otherwise**; its instruments are *"a `rk-kernel` unit test over a recorded transcript with
`structured_output` absent"* and *"a golden transcript fixture from F20"* — a recording, not an adapter. AC-318: an
architecture lint over the review path's call graph asserting **zero calls into any text-to-JSON extraction helper**,
plus a lint-fail fixture that adds one; the register's own split note says *"a runtime behaviour on an empty result and
a static property of the workspace are two behaviours with two instruments"*. This is the Halt 3 shape made mechanical
— F20 measured a tools allowlist silently voiding `--json-schema` at **exit 0, `subtype: success`**.

### 7b-ii.4 The policy table — the register's §9.4 in full, AC-294..AC-303

The whole of the register's §9.4 is this sub-step's, not the four ids the old prose named. **AC-297 is CUT and never
reusable** (register §0.4; the §9 area key), so a numeric range must not span it.

| id | what it fixes | the instrument that discriminates it |
| --- | --- | --- |
| **AC-294** | a block value is constructible only in `rk-kernel::policy`, and every block value originated in a `(class, stage_weight)` lookup | a private constructor **plus** a module-boundary architecture lint asserting **zero** construction sites outside `rk-kernel::policy`; **a compile-fail test (`trybuild`) that adds one** |
| **AC-295** | changing the stage's `ReviewWeight` from `Lenient` to `Strict` changes the block set **only** by adding `narrowed-pin` findings, with no re-dispatch and no change to the classifier's output | a replay test over a **recorded corpus run** at both weights asserting the set difference is exactly the `narrowed-pin` items; the classifier's output byte-compared across the two runs |
| **AC-296** | no `advisory` finding is in any block set, at any weight | a `proptest` quantified over every weight × every class |
| **AC-298** | a stage that sets no weight resolves to `Lenient` — `narrowed-pin` advises, it does not block | a default-resolution unit test plus a config golden file |
| **AC-299** | `ReadOnly` blocks nothing and dispatches no re-execution | an empty-block-set unit test at `ReadOnly` regardless of class; **a process-accounting test that the read-only tier spawns zero re-execution processes** |
| **AC-300** | `bug-standard` / the fix lane is execution-grade (`Strict`), and a pack that lowers it fails validation | a pack-validation rule and a negative pack fixture; a `doctor` rule-table row (7d's) |
| **AC-301** | a cheap-first cascade is **refused** with the reason *"cascade recall loss unmeasured (S-CASCADE unrun)"* | a pack-validation rule and a fixture pack that must fail. The refusal **lifts only when S-CASCADE reports** |
| **AC-302** | the review tier derives only from authored stage rigor weight and authored facets — **no LLM-emitted context vector is an input** | a signature/type test that `resolve_review_tier` takes no model-produced value; an architecture lint on the call graph |
| **AC-303** | no escalation removes the **review-confirm step** from the resolved graph, over every `(activity × rigor)` pair | a lattice lint over every `(activity × rigor)` pair; **the F37 `mandatory`+`has_repro` case as a named regression row**, which now asserts the ceiling holds |

**AC-303 and AC-105 are twins and must not be credited to one lint.** AC-105 (`s7b` §7b.8, `rk-catalog` CI) fails when
an escalation omits a **quality-apparatus step** (adversary, policy, or declared quality block) across the full
**facet** × rigor cross-product. AC-303 (`rk-kernel`) fails when an escalation removes **the review-confirm step**
across every **activity** × rigor pair. They share the F37 `mandatory`+`has_repro` regression row and nothing else.
The register's §0.3 exists for exactly this shape: a build that satisfies one twin and skips the other reads green in
both areas. **Two lints, two fixtures, and the checkpoint asserts each by its own name.**

**AC-302 is the standing negative for the BLOCKED selection engine.** The register says so in terms: *"P1's kill fired
4/4, F36: review tier may not be made to depend on it. Arm D precision/recall is now computed [measured, F39], but it
measures a *derived* vector, not the LLM-emitted one this criterion excludes, so it licenses nothing about the input
barred here."*

### 7b-ii.5 Re-execution — a finding blocks only when it flips

- **AC-287** — a `reproduced` or `mutation-survived` finding with **no** re-execution record at the reviewed SHA does
  **not** block, and its outcome is `Unverifiable { cannot_run }` — an outcome carried **beside** its class, never a
  change of class. *Verified by* a unit test with an empty re-execution record asserting an **empty block set**, plus a
  replay assertion over the corpus that `blocks ⊆ re-executed`.
- **AC-288** — a `reproduced` finding blocks iff the command **fails on the candidate and passes on the reference**.
  *Verified by* a **four-cell truth-table test** (candidate × reference); a seeded corpus fixture whose command flips
  must block, **and a paired fixture that fails on both must not**.
- **AC-289** — a `mutation-survived` finding blocks only if the suite is still green with the mutant applied **and the
  mutant discriminates**: a reverted-fix patch, or an `expected.assertion` the gate observed flip (ADR-0096 decision
  2). *"An equivalent mutant (green, no discriminating assertion) collapses to `narrowed-pin` or `advisory` and cannot
  block."* *Verified by* a mutation-harness integration test over the **S0.1 fixture set** — the 1 surviving mutant
  must block, the 9 killed must not — **plus an equivalent-mutant fixture (a no-op patch, green suite, no assertion)
  that must not block**; the suite name and mutant patch digest are recorded on the block event. **The workflow-only
  fixture is added once S0.1-SCOPE runs** and is ARMED, not discharged here: S0.1-SCOPE left **8 of 23 items unmeasured
  by construction**, their fixes touching only `workflows/`.
- **AC-290** — a re-execution that fails on both, or cannot run at all, yields `Unverifiable { both_fail | cannot_run
  | timeout }`, **never blocks and is never silently recorded as passing**; `ReExecution = Flipped | Unverifiable {..}`
  is the outcome enum and the class enum has four variants and no `Unverifiable`. *Verified by* **three negative
  fixtures, one per reason**, plus a replay assertion that no `Unverifiable` finding appears in any block set; its
  third instrument — *"a doctor rule that every finding classed `reproduced` or `mutation-survived` carries either a
  re-execution record or an `Unverifiable` reason"* — is `rk-cli`'s and is ARMED here.

### 7b-ii.6 Quarantine — and the one thing quarantine is not an answer to

**The mechanics** are AC-304 (a `Quarantined { reason, by, until }` finding is **absent from the block set and still
persisted and rendered with its reason**), **AC-305** (stable identity — the tuple *(AC id, `repro.kind`, repro command
digest, and for `kind = mutate` the digest of `mutation.patch` plus `mutation.suite`)* — *"identity stable under
rationale rewording and file-order change, unstable under a changed repro command or a changed mutation patch"*),
**AC-306** (an expired `until` re-raises the finding, emits `quarantine_expired`, and the re-raised finding *"blocks
iff its class blocks at that weight"* — a clock-injected unit test at `until ± 1s` at **both** lenient and strict) and
**AC-307** (every block and every override appends an event carrying `{ finding_id, ac, computed_class, model_label,
stage_weight, sha, re_execution_id, actor, adjudication: null }`, with the replay invariant `count(blocks) ==
count(block_events)` **and an architecture lint that the block path and the event append are the same code path**).

**A numeric AC-304–AC-307 cannot reach AC-319, and AC-319 is in the same register section.** AC-319 (`rk-types`): a
quarantine constructed with **no `until` is refused** — a type test that `until` is non-optional plus a negative
fixture that must fail to construct. Its own split note is the argument: *"Split out of AC-306: an expiry behaviour
and a construction-time refusal are two behaviours with two instruments."* Building the expiry and skipping the
construction refusal reads green in both.

**And quarantine is a review-gate state only.** ADR-0104 decision 4: *"Quarantine (ADR-0074 clause 6) is a review-gate
state: it stops a finding blocking review and stops it being re-raised until `until` … It is not an answer to AC-346
… AC-345 and SM-095 say 'review-gate only'; AC-346 says quarantine does not satisfy it."* The two criteria that make
that checkable are **AC-345** — a quarantined `mutation-survived` finding leaves the AC's evidence class and ledger
status unchanged *"while the review gate does not block"*, with *"an assertion that stage exit (AC-346) is still
refused for that AC"* — and **AC-346**, whose own fixture set is *"an integration test over a two-AC fixture with one
unpinned AC; **a fixture where the survivor's finding is quarantined and exit must still be refused**; an assertion
that the refusal event names the AC id"*. **AC-346's quarantined-survivor fixture is the discrimination.** A flip-
predicate test does not reach it.

### 7b-ii.7 The advisory sink — intake and exit are two invariants

- **AC-310** — every non-blocking finding is persisted to the item's advisory sink with its class, repro record and
  reason for not blocking, **and none is dropped**. *Verified by* a replay assertion **`findings_in == blocks +
  sink_records`** and, over the sink's lifetime, **`live_sink == created - retired - dismissed`**; plus *"the seven
  F31 `narrowed-pin` findings replayed as the seed fixture, which must all land in the sink"* — seeded under the
  **model-label** policy (ERRATA E-0074-2), so the seed is a seed and not a measurement of the Rust classifier, whose
  seed set is S-CLASS-REPLAY's. Its `doctor` rule-table row is `rk-cli`'s and is ARMED here.
- **AC-530** — *a record leaves the sink only by a recorded disposition.* An advisory record whose finding is not
  re-raised by the next review of the same item (its AC-305 identity absent from review *n+1*'s findings) gets an
  `advisory_retired { finding_id, reason: not_re_raised, review_id }` event and leaves the live sink — *"never
  silently, never by deletion"* — **and at every point `live_sink == created - retired - dismissed` holds**, where
  `dismissed` counts AC-311's human dispositions. *Verified by* a two-review integration test asserting the event and
  the count, **plus a replay assertion of the invariant over a fixture run carrying created, retired and dismissed
  records**; its doctor row is 7d's. The register's own note says why both exist: *"AC-310's `findings_in` invariant
  covered intake, not exit."*
- **AC-311 is HUMAN-GATED** — whether an advisory finding is worth a backlog item is a value call. Its machine-
  checkable half is *"every sink record reaches a terminal disposition or is surfaced as ageing (SM-089)"*, and that
  half is built here. **No CI job may pass or fail on AC-311.**

### 7b-ii.8 Replay determinism — two assertions, not one

**AC-313** — given one recorded `Findings` payload containing a `reproduced` finding, **its recorded re-execution
record**, and the reviewed SHA, running the confirm half 10 times gives *"the block set … identical on all 10 runs
**and equal to the expected non-empty set for that fixture**"*. *Verified by* a replay job (recorded findings, no model
dispatch, $0) asserting **both** set equality across k=10 **and** equality to the pinned expected set.

**The non-empty clause is the half the register repaired IN, and it is repaired in twice.** AC-313's own note: *"without
the non-empty clause, a gate that blocks nothing scores 10/10."* AC-312's note says the same thing about AC-313 from
the other side: its stated rescue *"only has meaning paired with AC-313"* is wrong, *"because AC-313's determinism
check is also satisfied by a permanently empty block set"*. A determinism-only test passes perfectly on a kernel whose
confirm half blocks nothing. **Red the two halves separately.**

**AC-312 is PROVISIONAL on S-SUBTLE-CB** and its framing-variant fixtures are **authored at Step 7c**, not here. Its
control half is *"a floor, not calibration"*, and ADR-0101 clause 5 is the standing prohibition this file's gate-check
banner already carries.

**AC-314** — a review regression run's record carries the **corpus repository's** commit sha, with a per-item
checkpoint sha stored under a differently named field; the defect it exists for is measured (F37: `emit.corpus_sha`
held a per-item checkpoint sha, so the key was misnamed). Its field-semantics test runs here; its doctor rule is 7d's.

### 7b-ii.9 The verify gate — the flip predicate, the comparator, the ledger

- **AC-334 owns the predicate** (ADR-0104 decision 2). Given an AC whose row carries an **author-declared**
  `mutation: { patch, suite }` — *"declared with the AC at the spec stage, a reverted-fix patch or one carrying a
  discriminating assertion locator per ADR-0096; never derived by a model, which AC-347 forbids"* — when the patch is
  applied at the candidate SHA, the named suite re-run and the tree reverted, **deterministic, zero dispatches**, the
  AC is `pinned` **only if the named assertion flips from pass to fail**, and `mutation-survived` if the suite stays
  green. *Verified by* the **issue-146-147 AC-015 (F25) ordering-inversion fixture as a regression test — it must
  yield `mutation-survived`, never `pinned`** — plus a unit test on the flip predicate. The measurement behind it:
  remove step 7 → RED, but **invert the ordering → 36/36 GREEN** [measured, F25, n=1 AC, one operator].
- **AC-317 is the invocation** and calls AC-334's predicate; the 2026-09-11 correction (PIA-051; ADR-0104 decision 2)
  withdrew the second predicate it used to carry. **Its instrument is the harness-invocation test *in the fix and story
  lanes***, and no lane runs at this stage — so AC-317's invocation half is **ARMED, not discharged**, and is
  discharged at Step 8, which consumes the fix lane. What is built and tested here is the harness itself and its
  report-time refusal over a fixture spec.
- **AC-335** is the paired negative and is in range: a fully green suite *"changes nothing"* about an AC whose mutant
  survived — it **remains `mutation-survived`, never `pinned`**. The warrant is measured: **four adjudicated defects
  survived fully green suites of 1,890/1,890 and 1,907/1,907** [F31].
- **AC-326 — the bootstrap comparator** (ADR-0104 decision 3). A run claiming `basis: affected` whose executed count
  is ≥ the most recent recorded whole-suite count at the pin is reclassified `basis: whole-suite` with reason
  `selection-ineffective` and emits `selection_ineffective`; **when no whole-suite count is recorded at the pin**, the
  claim is reclassified with reason `probe-unverified` and **its executed count seeds the comparator**, and **a pin
  bump resets the comparator**. *Verified by* four fixtures: the reclassifier unit test over `(selected, total)` pairs
  **including the boundary**; an integration fixture reproducing the historical `pnpm test -- --run <file>` shape which
  must reclassify; **a bootstrap fixture with an empty comparator asserting `probe-unverified` and the seeded count**;
  and **a pin-bump fixture asserting the comparator resets**. Carry the register's own *Known gap* forward rather than
  smoothing it: *"this detects only a filter that is fully inert. A filter that drops most of its arguments but not all
  — selecting 200 of 236 files — passes this check."* AC-327's probe and SM-097 are the compensating instruments and
  **neither is a substitute**.
- **AC-329 is split between this stage and 7c, and the split is stated at both ends.** Its **field-semantics half** is
  here: on an `affected` run `total_test_count` is the most recent recorded whole-suite count at that pin and the
  report names the run it came from via `total_test_count_source`; on the bootstrap run it is the run's own executed
  count. Its **adapter half is `s7c`'s**: *"schema conformance test over **every adapter's** fixture reports"* and *"a
  lint forbidding markdown parsing in the **toolchain adapters**"*, neither of which exists at this stage. Its subject
  tag is `rk-store-artifacts`, i.e. a `rk-store` module (design §9.1). **This site notes `s7c`:196; a matching note in
  `s7c` is OWED** — the register's §0.4 says a shared id conforms only when *"both sites note the other"*.
- **AC-333 — the AC evidence ledger** (`rk-store-artifacts`). Each AC row carries an **assertion locator — file, test
  identifier and the assertion within that test — and a row naming only a test file fails validation**; every AC id in
  the spec is materialised, an AC with no evidence materialised as **`unpinned` rather than omitted**; the status is
  **one closed enum per AC**, `pinned | mutation-survived | unpinned | not-feasible | human-answered`; and the ledger
  includes the six Phase-1 companion kinds' `ledger` record (AC-143). *Verified by* schema validation **including a
  schema fixture enumerating the closed status enum with no wildcard**; **a negative fixture whose row names a file
  path only, which must fail**; and **a completeness check asserting the ledger's AC id set equals the spec's**.
- **AC-337 — mutation feasibility is declared, and an absent declaration is a configuration error**; where a toolchain
  declares mutation infeasible, an AC records `mutation: not-feasible(reason)` which is **not `pinned`**. Its fixture
  pack half runs here; its `doctor` rule-table row is 7d's.

**AC-334's instrument needs a real adapter, and at this stage it has one fake.** The predicate *applies a declared
patch, re-runs a named suite and reverts the tree* — the run is the Toolchain port's. `rk-ports` ships the Toolchain
fake (Step 7a) and the four **real** toolchain rows are Step 7c's. So at 7b-ii `ac334_flip_predicate` is exercised
**against the Toolchain fake only**, over a scripted fixture whose suite result is scripted rather than executed.
**That is stated, and AC-334 is therefore reported ARMED, NOT DISCHARGED in §7b-ii.14** — a flip predicate proved over
a fake is evidence about the predicate, not about the harness against a real suite, and the honest label is the
register's own rule (golden constraint 10: report powerless as powerless, never as a pass).

### 7b-ii.10 Where the fixtures live

Every criterion in this stage is discharged by fixtures, and **none of them may be a shipped artefact**. `.redkiln/`
does not exist here — Step 7e derives it (`s7e` §7e.1), and `ls -d .redkiln` returns *"No such file or directory"*
[measured 2026-09-13] — and `spikes/p9/`'s packs and catalog are the corpus the grammar probe and the lattice
validator are measured against, so they may not be edited to make a red case.

Author every fixture under **`crates/rk-kernel/tests/fixtures/`**, beside the sub-tree `s7b` §7b.12 opens and in
five directories of its own (`s7b` §7b.12's table is Step 7b's set; **three** of these five — `gates/`, `findings/`,
`ledger/` — are added by this sub-step, **two** — `packs/` and `lint-controls/` — already exist from Step 7b and gain
files here, and `ui/` is shared because the crate has **one** trybuild harness) *(corrected 2026-09-17: this read
"these five are added by this sub-step". Measured on the untouched tree at `fc4c502`: `packs/` held 15 pack fixtures
and `lint-controls/` held `ac294_runstate_construction_control.rs` and `ac487_identity_check_control.rs`, both 7b's.
The consequence for §7b-ii.13 is stated where it bites: the `test -d …/lint-controls` line is a **guard** that already
passes on the untouched tree, and what discriminates this stage's two controls is the pair of `grep -q` lines that
name each file — not the directory's existence)*:

- `gates/` — AC-244's partial records (one per missing field), AC-245's dirty-worktree pair, AC-524's two-iteration
  loop graph;
- `findings/` — AC-284's one row per branch plus the `otherwise` row, AC-285's disagreeing fixture, AC-288's four
  truth-table cells, AC-289's S0.1 fixture set and its equivalent-mutant case, AC-290's three negative fixtures;
- `packs/` — AC-300's lowered-weight pack, AC-301's cascade pack, AC-337's `feasible = false` pack;
- `ledger/` — AC-333's schema fixture, its file-path-only negative and its completeness pair;
- `ui/` — the `trybuild` cases, in the crate's **one** harness (§7b-ii.12). **Compile-FAIL cases only**;
- `lint-controls/` — **the positive controls for the three absence-based greps in Checkpoint 7b-ii**:
  `ac285_policy_reads_the_model_label.rs` and `ac286_classifier_reads_the_clock.rs`. **They are here and not
  under `ui/` because `ui/` is the `trybuild::TestCases` glob root** (`s7b` §7b.4: one harness *"over every
  `crates/rk-kernel/tests/ui/` case"*): a file placed there must FAIL to compile, and a grep's positive control must
  be an ordinary file that carries the banned token — putting it in the glob makes `trybuild` report an unexpected
  success and turns the harness red, and it would owe a `.stderr` it can never have. Nothing compiles these two; they
  exist to prove the greps still match. **AC-294's control is the exception and stays in `ui/`**, because its control
  and its compile-fail case are the same file: the fixture must both construct a block value and fail to compile.

Each fixture is named for the criterion it discriminates and each one is committed. Where a fixture must be
byte-identical to a record — AC-282's golden F20 transcript, AC-334's issue-146-147 AC-015 ordering inversion — copy it
rather than pointing at the source.

### 7b-ii.11 Red first

**Red first (golden constraint 12; ADR-0019).** `00-preamble.md`:207 names **Step 7b-ii** among the stages that write
Rust — added there on 2026-09-13 in the same change as the split, so this file no longer relies on inheriting Step
7b's row. The checkpoint below is the floor, not the evidence: every one of its
`cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins. Author each named test **before**
the code it describes, run it, and **watch it fail for the right reason** — the absent behaviour, not a compile error
or a filter that matched nothing.

- `ac244_gate_record_requires_all_three` — **two instruments and the type-level one is the discrimination.** The gate
  record is **unconstructible** without a token, a tree hash and a deadline (a `trybuild` case that omits one), plus a
  runtime test on the deserialisation path. Red the deserialisation half against a record missing each field in turn;
  a single happy-path construction test pins nothing.
- `ac245_write_tree_one_algorithm` — **one algorithm, used at gate open AND at answer.** Red as a property test over
  dirty-worktree fixtures: an **untracked** file must not change the hash and a **tracked modification** must. F19
  measured drift on 12/174 gates, so the second-derivation-site failure is reachable; red against a second derivation.
- `ac524_fresh_record_on_rereach` — a re-reached gate mints a **fresh** `(token, tree_hash, deadline)`. Red on a path
  that reuses the prior record: the test must fail on the staleness, not on the token's shape. PROVISIONAL on P8-B.
- `ac246_token_is_single_use` — ADR-0017's *single-use*. The second answer is **refused as spent**, records a
  **distinct** event, and **changes no item state**. Red on a path that accepts the second answer; red the
  no-state-change half separately, because a refusal that still advances the item satisfies the first two.
- `ac249_stale_tree_hash_is_void` — an answer whose tree hash no longer matches is **void**, not reconciled. Red by
  reconciling it.
- `ac250_no_reconciliation_path` — the architecture lint that **no code path reconciles a void answer**; ADR-0017's
  rejected option is literally reconciliation. Red by adding a reconcile branch and watching the lint go red; a lint
  with no fixture is green over a gate path nobody has written.
- `ac270_gate_state_from_the_log_alone` — reconstruct every gate open, answer, void, expiry, escalation and re-open,
  and the terminal gate state, **from the event log with no working-tree read**, and compare to the live derivation.
  Red by having the reconstruction consult the tree. The register says this test **doubles as AC-258's
  clock-independence instrument**, so assert that half by name in the same test.
- `ac272_escalation_is_one_gate_mechanism` — an escalation raised inside an activity's internal graph and a
  stage-level gate emit **structurally identical** `gate_opened` records, plus a schema test that **one** gate type
  exists and not two. Red by giving the internal producer its own shape.
- `ac284_class_decision_table` — one row per branch **plus** the `otherwise` row, and an exhaustive `match` with no
  `_ =>` fallthrough to a blocking class. Red by adding a `_ => Reproduced` arm and watching the `otherwise` row fail.
- `ac285_class_drift_event` — red on a classifier that prefers the model label: the block set must change and
  `review_class_drift` must carry both labels.
- `ac286_classifier_is_deterministic` — 1,000 in-process draws plus a fresh process. Red by reading `SystemTime::now()`
  inside the classifier module and watching **the source-level lint** go red — the `proptest` alone will not catch a
  route that is deterministic per process.
- `ac283_no_free_text_reaches_policy` — the **signature** test. Red by adding `rationale: &str` to
  `policy::classify`'s parameter list: the test must stop compiling, which is the failure.
- `ac282_structured_output_required` — red on a kernel that reads an absent `structured_output` as zero findings; the
  step must be an **error**, not a success with an empty set. The F20 golden transcript is the fixture.
- `ac318_no_json_from_prose_in_the_review_path` — the architecture lint, with a lint-fail fixture that adds one call.
  Red by adding the call; a lint with no fixture is green on a review path that has never been written.
- `ac294_block_constructor_is_private_outside_policy` — the `trybuild` case, inside the crate's one harness. Red means
  it **compiles**, which is the failure. Paired with the module-boundary lint asserting zero construction sites.
- `ac295_weight_change_adds_only_narrowed_pin` — red on a policy that re-dispatches or re-classifies at `Strict`.
- `ac296_advisory_never_blocks` — the `proptest` over every weight × every class. Red by making one weight block
  `advisory`.
- `ac298_unset_weight_is_lenient` — a stage that sets **no** weight resolves to `Lenient`, so `narrowed-pin` advises
  rather than blocks: the default-resolution unit test plus the config golden file. Red by defaulting to `Strict`.
- `ac300_fix_lane_is_strict` — the pack-validation rule plus the **negative pack fixture that lowers the weight**,
  which must fail validation. Red the negative fixture; a rule with only a positive pack is green on a pack set that
  never lowers anything. Its `doctor` rule-table row is 7d's.
- `ac301_cheap_first_cascade_is_refused` — the pack-validation rule plus **a fixture pack that must fail**, with the
  refusal reason *"cascade recall loss unmeasured (S-CASCADE unrun)"* asserted in the message, because the reason is
  what the lift will later be keyed on. Red on a validator that accepts the cascade.
- `ac299_read_only_spawns_nothing` — **two assertions**: the empty block set, and the **process counter at zero**. Red
  the counter separately; an empty block set is satisfied by a tier that dispatched and then discarded.
- `ac302_review_tier_takes_no_model_value` — the signature/type test plus the call-graph lint. Red by threading a
  model-produced value into `resolve_review_tier`.
- `ac303_confirm_step_survives_escalation` — the `(activity × rigor)` lattice lint with the F37 `mandatory`+`has_repro`
  case as a **named regression row**. Red on a pre-ceiling catalog fixture (`s7b` §7b.8's fixture, without
  `max_rigor`), green on `spikes/p9/catalog/` as it stands with `bisect` ceilinged.
- `ac287_no_record_no_block` — red on a policy that blocks a `reproduced` finding with no re-execution record.
- `ac288_four_cell_truth_table` — red the "fails on both" cell separately; it is the one a flip-only implementation
  gets wrong.
- `ac289_equivalent_mutant_does_not_block` — the S0.1 surviving mutant must block and the nine killed must not, **and**
  the no-op patch with no assertion must not. Red the equivalent-mutant case; it is the discrimination.
- `ac290_unverifiable_never_blocks` — three negative fixtures, one per reason, plus the replay assertion.
- `ac281_findings_ingest_rejects_an_incomplete_repro` — **`rk-types`'s**, filtered `-p rk-types` in the block below.
  The `serde` round-trip plus `jsonschema` validation and **five** negative fixtures: missing command, missing
  observed, missing `expected_when_met`, a prose-only command (*"run the tests and see"*), and `kind = mutate` with
  no `mutation` (ADR-0096 decision 1 — **malformed, not `Unverifiable`, not a finding**). Red the prose-only case
  separately; it is the one a presence-only check gets wrong.
- `ac293_review_outcome_carries_the_reviewed_tree_digest` — **`rk-types`'s**, also filtered in its own crate. The
  digest field is **non-optional**: a type-level test, a round-trip capturing `basis`/`sha`, and a negative fixture
  with the digest removed that must **fail to construct**. Red means it constructs with a default or a `None`.
- `ac304_quarantined_is_absent_but_persisted` — **both** halves: absent from the block set, still persisted and
  rendered with its reason. Red the rendering half; a delete satisfies the first.
- `ac305_finding_identity_is_stable` — red by rewording the rationale (identity must hold) and separately by changing
  the repro command (identity must break). One direction alone pins nothing.
- `ac306_quarantine_expiry_re_raises` — clock-injected at `until ± 1s`, asserted at **both** lenient and strict.
- `ac319_quarantine_without_until_is_unconstructible` — the type test plus the negative fixture that must fail to
  construct. Red means it constructs.
- `ac307_every_block_has_an_event` — the replay invariant `count(blocks) == count(block_events)` **plus** the
  architecture lint that the block path and the event append are the same code path. Red by appending from a second
  site.
- `ac310_sink_accounting` — `findings_in == blocks + sink_records`. Red by dropping one sink record: the identity must
  fail **with both sides printed**, so the failure names which side moved.
- `ac530_sink_leaves_only_by_recorded_disposition` — the `advisory_retired` event and the count, **plus** the replay
  invariant `live_sink == created - retired - dismissed` over a fixture run carrying created, retired and dismissed
  records. Red by deleting a record instead of retiring it.
- `ac313_replay_k10` — **two assertions, red separately**: set identity across k=10, **and** equality to the pinned
  **non-empty** expected set. Red the non-empty half against a kernel whose confirm half blocks nothing; it satisfies
  the first perfectly.
- `ac314_corpus_sha_is_the_corpus_repos` — the field-semantics test asserting the recorded sha resolves in the corpus
  repository's history. Red on a per-item checkpoint sha in that field — which is the measured defect (F37).
- `ac334_flip_predicate` — the declared mutant **must flip** the test. A mutant that survives blocks, quarantined or
  not. ARMED at this stage against the Toolchain **fake** only (§7b-ii.9, §7b-ii.14).
- `ac015_inversion_regression` — **the canonical regression row, under its own name and its own bullet**, because a
  row named only inside another bullet's prose is a row nothing pins. The F25 issue-146-147 AC-015
  ordering-inversion fixture **must yield `mutation-survived`, never `pinned`**. The measurement behind it: remove
  step 7 → RED, but **invert the ordering → 36/36 GREEN** [measured, F25, n=1 AC, one operator]. Red on a harness
  that promotes it to `pinned`.
- `ac243_no_dispatch_across_open_gate` — a replay assertion over the event log: **no `dispatch` event carries a
  timestamp between `gate_opened` and that gate's terminal event for the same item**, plus the P8 dispatch-count
  regression asserting **0** across an open gate. Red on a kernel that dispatches while a gate is open. Cross-listed
  with 7c; this is the kernel half and it needs no adapter.
- `ac279_one_prefix_rewrite` — a count assertion on prefix-rewrite events per `gate_resumed`, **≤ 1**. Red on a path
  that rewrites twice. Cross-listed with 7c; kernel half, no adapter needed.
- `ac335_green_suite_does_not_promote_a_survivor` — red on a ledger that upgrades a survivor to `pinned` on a green
  suite. Four adjudicated defects survived 1,890/1,890 and 1,907/1,907 [F31].
- `ac345_quarantine_changes_no_ledger_status` — and, in the same test, **that stage exit is still refused for that AC**.
- `ac346_quarantined_survivor_still_refuses_exit` — AC-346's own fixture: quarantine the survivor's finding, assert
  exit is refused, the unpinned AC id is **named in the refusal**, and the refusal is **an event, not only a console
  line**. This is what discriminates ADR-0104 decision 4; `ac334_flip_predicate` does not.
- `ac326_bootstrap_comparator` — four cases: the boundary, the inert-filter shape, the empty comparator seeding at
  `probe-unverified`, and the pin bump resetting it. Red each; the pin-bump case is the one a cache gets wrong.
- `ac329_total_test_count_source` — the field semantics and `total_test_count_source`. The per-adapter sweep is 7c's.
- `ac333_ac_evidence_ledger` — **three** instruments: the closed-enum schema fixture **with no wildcard**, the
  file-path-only row that must fail, and the completeness check that the ledger's AC id set **equals** the spec's. Red
  the completeness check by omitting one AC; an `unpinned` row is the correct output, not an omission.
- `ac337_mutation_feasibility_is_declared` — an absent declaration is a configuration error; a `feasible = false` pack's
  ACs must not reach `pinned` by the mutation path.
- `ac239_gate_eligible_list_is_data` — the gate-eligible adapter list is **data with exactly one entry in Phase 1**,
  which the register calls *"checkable today, with no Codex adapter present"*. Red by hard-coding the adapter name at
  the gate evaluator. The **Codex-present half is excluded** (PROVISIONAL on S-CODEX; `rk-adapter-codex` is Phase 2 per
  ADR-0094 decision 2).
- `ac455_policy_decides_not_the_type` — **two policy-table rows over one fixture spec**: a spec carrying an AC with
  `verification: None` under a pack policy that requires one is **refused with the AC id named, at zero model cost**,
  **and the same spec passes under a pack whose policy does not require it**. The second row is the must-pass control
  and is not decoration — without it the criterion is satisfied by a policy engine that refuses every spec, which is the
  same argument `s7b` §7b.7 makes for AC-520's positive companion.

### 7b-ii.12 The manifest, the harness and what this stage may not disturb

`rk-kernel`'s manifest, module tree and `[lints] workspace = true` are `s7b` §7b.4's and are not re-decided here. Two
things this sub-step adds to that crate:

- **One `trybuild` harness, named after its rule and not after the tool.** The house convention is measured:
  `crates/rk-git/tests/it/reconciled.rs`:24 is `reconcile_runs_at_every_entry`, `crates/rk-store/tests/it/tier_ui.rs`:9
  is `a_state_field_without_a_tier_is_a_compile_error`, and neither name contains the substring `trybuild` [measured
  2026-09-13]. `s7b` §7b.4 fixes the kernel's harness name as
  **`trybuild_pins_the_reason_each_kernel_fixture_fails`**, in `crates/rk-kernel/tests/it/ui.rs`, and this sub-step
  adds **two compile-fail cases** to it — AC-244's partial gate record and AC-294's block constructor — each
  committing a `.stderr`. **The checkpoint filters on the harness test's own name, never on `trybuild`.** It also
  adds **two files that are not cases**: `ac285_policy_reads_the_model_label.rs` and
  `ac286_classifier_reads_the_clock.rs`, the positive controls for Checkpoint 7b-ii's two module-purity greps. They
  live under `tests/fixtures/lint-controls/` and **not** under `tests/ui/`, because `tests/ui/` is the harness's glob
  root: a control must carry the banned token in an ordinary file, while a glob case must fail to compile and commit
  a `.stderr`. §7b-ii.10 carries the directory and the reason.
- **`tests/it/main.rs` gains a `mod` per case**, never one file per case (ADR-0083:74).
- **Where the two architecture lints themselves live is stated, because the bans depend on it.** AC-285's
  *"`finding.evidence_label` has no reader inside the policy module"* lint and AC-286's classifier-purity lint are
  **not** inside the modules they police — they must contain the very literals they search for, so a lint written
  inside `policy` or `classify` makes the ban fire on its own implementation (the self-referential shape
  `s7a`:1032-1039 records). They sit in `rk-kernel`'s lint module, outside both paths.

**Fourteen claimed ids do not land in `rk-kernel`, and Checkpoint 7b-ii reaches each crate the claimed set reaches
rather than asserting them all under `-p rk-kernel`** — its third loop filters eight of them in `rk-types`,
`rk-store` and `rk-ports` by name. *(This sentence asserted that before the block did it: until 2026-09-13 every
invocation in the block was `-p rk-kernel`, so six ids whose subject tag is another crate would have selected zero
tests, cargo would have exited 0 printing `0 passed`, and the by-name grep would have halted the block on a correct
crate.)* Read off the register's subject tags against
`references/2026-09-08-architecture-design.md` §9.1, the crate-name authority: **`rk-types`** — AC-280, AC-281,
AC-293, AC-304's type half, AC-319; **`rk-store`** (via `rk-store-artifacts`) — AC-305, AC-310, AC-329's schema half,
AC-333; **`rk-ports`** (via `rk-port-toolchain`) — AC-325, AC-328, AC-337, AC-351; **`rk-git`** — AC-291, AC-292 (see
§7b-ii.14, which raises them rather than claiming them). **The module location follows the design and is not decided
here.** But the consequence is: every one of those crates has a checkpoint already recorded as passed — `rk-types` at
Checkpoint 5, `rk-store` and `rk-git` at Checkpoint 6, `rk-ports` at Checkpoint 7a — so under `00-preamble.md`:232-237
any edit inside one makes that pass **provisional**: mark the row `passed <hash>, re-run pending 7b-ii <crate>` in
`PROGRESS.md` §1 and **re-run that checkpoint in the same commit as the 7b-ii work**. Nothing in this step may land a
schema in a closed crate without that row.

### 7b-ii.13 Checkpoint 7b-ii

**Counted mechanically off the block below on 2026-09-13, and after this file was written and not before it** —
comments and blanks dropped, backslash continuations joined. **The rule that produces the number is stated so it
reproduces:** a *statement that can stop the block* is one carrying its own `exit 1`. That is the unit `PROGRESS.md`
records and it is named here because the same number in no unit is the defect Checkpoint 7b carried twice
(`s7a`:734-737; Halt 7). **72 non-comment statements, of which 42 can stop the block and 30 cannot.** The 30 that
cannot are not unguarded assertions and the decomposition reproduces: **3** BEFORE readings (PIA-035 — prints for
comparison, not assertions); **19** `out=` / `raw=` / `hits=` / `c=` capture halves whose guard is the statement
immediately below them; **1** clippy line, `cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"`, whose
last statement is an `echo` and whose guard is the line below; **1** `set -- $p_t` splitting the third loop's
crate/test pair, whose guard is the two statements below it; and **3 + 3**, the `for` and `done` of the three loops.
3 + 19 + 1 + 1 + 6 = 30; 72 − 30 = 42.
**Re-count and re-write every number in this paragraph after any repair to this block** — a number written before the
block was final is the defect Checkpoints 4 and 7b each carried once (Halt 7: count it off the script, not off this
sentence).

**No line in this block is `[WILL EXIST]`, and the absence is deliberate.** A checkpoint runs after its step's work, so
every path the block names is this step's own output or already on the tree. The two things that would not exist —
`./target/debug/redkiln` and `.redkiln/facets.toml` — are **not in this block at all**: the binary is `rk-cli`'s
(Step 7d, `INDEX.md`) and the registry is Step 7e's (`s7e` §7e.1), and `halts.md`:89-91 makes a `[WILL EXIST]` line
that exits non-zero because its binary is absent **not a pass either**. What this stage owes instead is the gate
behaviour under `cargo test`, which every line below asserts by name over in-crate fixtures.

**Every absence-based ban below carries a positive control, or says in place why it cannot.** An architecture lint, a
module-purity lint and a construction-site ban are all pass-on-absence: the grep that proves the ban is satisfied by a
tree where the search reached nothing, and by prose that names the thing it governs. So each ban is paired with (a) a
`test -d` guard so a missing directory cannot read as a pass, (b) a **behavioural test asserted by name** — a grep a
comment could satisfy is never the whole assertion (`s6`:249-250's house note) — and (c) a **positive control**
asserting the same pattern **does** match inside a committed fixture, so a pattern that silently stops matching is
caught. **One ban has no control and the trade is stated where it lives rather than implied**: the ADR-0101 clause 5
false-approval guard, whose pattern would match its own FAIL message if it searched the file it is written in —
`s7a`:1032-1039 records that shape, and an exclusion list tuned to this file's own wording would be the route-around
golden constraint 9 names.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }

# ---- BEFORE readings (PIA-035): each PRINTS on the untouched tree and is silent after the step. They are
# ---- READINGS FOR COMPARISON, NOT ASSERTIONS — nothing below consumes them, and the count above puts them
# ---- among the statements that cannot stop the block. Run them first and record all three prints.
test -z "$(ls -d crates/rk-kernel/src/policy crates/rk-kernel/src/policy.rs 2>/dev/null)" && echo "BEFORE: no rk-kernel::policy module yet"
test ! -d crates/rk-kernel/tests/fixtures/gates && echo "BEFORE: no gate fixtures yet"
test ! -f crates/rk-kernel/tests/ui/ac294_block_constructor.rs && echo "BEFORE: no AC-294 block-constructor fixture yet"

# ---- the five CI gates, each guarded, workspace-scoped. Checkpoints 5, 6, 7a and 7b each run all five
# ---- (s5:378-390, s6:193-199, s7a:785-793). `cargo build -p rk-kernel` is NOT Halt 1's instrument from
# ---- Step 5 on (halts.md:5-6). `deny` bites on contact: this sub-step adds dev-dependencies and fixture
# ---- files, which moves Cargo.lock, and the licence set is re-derived when the lock moves (s6:189-192).
cargo build --workspace || { echo "FAIL: cargo build --workspace (Halt 1's instrument from Step 5 on)"; exit 1; }
cargo fmt --all --check || { echo "FAIL: cargo fmt --all --check"; exit 1; }
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red — AC-001-AC-004's instrument IS this invocation gated on its exit code; nursery WARNINGS are permitted (ADR-0102, PIA-030)"; exit 1; }   # → exit=0
out=$(cargo test --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }   # → a FLOOR ONLY: rk-types, rk-store, rk-git and rk-ports tests each satisfy it; the named lines below are what bear on THIS step
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses"; exit 1; }
# The crate-wide run, asserted separately: every other rk-kernel line below is FILTERED, so a unit test can
# fail under a name no filter names while every guarded line stays green.
out=$(cargo test -p rk-kernel 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p rk-kernel exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: rk-kernel compiled but ran 0 tests"; exit 1; }

# ---- the named tests, one per criterion, asserted BY NAME. cargo's `--` filter is a SUBSTRING match over
# ---- TEST names, so the aggregate `test result: ok. N passed` line is satisfied by a near-miss name and a
# ---- filter matching nothing exits 0 printing `0 passed` (s6:211-214, s7a:939, measured). Each loop body
# ---- asserts the process status first and then that a test of exactly that name ran and passed.
# ---- NOT SCOPED `--test it`: it selects ONLY the integration binary, and several of these are signature and
# ---- type tests that live beside the code they police (AC-283's `policy::classify` signature test is the
# ---- example — it must stop COMPILING when a `&str` parameter is added, which is a `#[cfg(test)]` unit test
# ---- in `src/`). MEASURED on a probe crate 2026-09-13: a `src/`-resident test is reported `0 passed;
# ---- 1 filtered out` under `--test it` and the name grep does not match — a FAIL for a target-selection
# ---- reason unrelated to the rule. Only the trybuild harness keeps `--test it`, because `s7b` §7b.4 FIXES
# ---- its file at `crates/rk-kernel/tests/it/ui.rs`.
for t in ac244_gate_record_requires_all_three ac245_write_tree_one_algorithm ac524_fresh_record_on_rereach \
         ac246_token_is_single_use ac249_stale_tree_hash_is_void ac250_no_reconciliation_path \
         ac270_gate_state_from_the_log_alone ac272_escalation_is_one_gate_mechanism \
         ac243_no_dispatch_across_open_gate ac279_one_prefix_rewrite \
         ac284_class_decision_table ac285_class_drift_event ac286_classifier_is_deterministic \
         ac283_no_free_text_reaches_policy ac282_structured_output_required \
         ac318_no_json_from_prose_in_the_review_path ; do
  out=$(cargo test -p rk-kernel -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $t exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: no test named $t ran and passed — the -- filter is a SUBSTRING match, so an aggregate 'test result: ok' line is satisfied by a near-miss name and a filter matching nothing exits 0"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
for t in ac294_block_constructor_is_private_outside_policy ac295_weight_change_adds_only_narrowed_pin \
         ac296_advisory_never_blocks ac298_unset_weight_is_lenient ac299_read_only_spawns_nothing \
         ac300_fix_lane_is_strict ac301_cheap_first_cascade_is_refused \
         ac302_review_tier_takes_no_model_value ac303_confirm_step_survives_escalation \
         ac287_no_record_no_block ac288_four_cell_truth_table ac289_equivalent_mutant_does_not_block \
         ac290_unverifiable_never_blocks ac304_quarantined_is_absent_but_persisted \
         ac306_quarantine_expiry_re_raises ac307_every_block_has_an_event \
         ac530_sink_leaves_only_by_recorded_disposition ac313_replay_k10 \
         ac314_corpus_sha_is_the_corpus_repos ac334_flip_predicate ac015_inversion_regression \
         ac335_green_suite_does_not_promote_a_survivor ac345_quarantine_changes_no_ledger_status \
         ac346_quarantined_survivor_still_refuses_exit ac326_bootstrap_comparator \
         ac239_gate_eligible_list_is_data ac455_policy_decides_not_the_type ; do
  out=$(cargo test -p rk-kernel -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $t exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: no test named $t ran and passed — the -- filter is a SUBSTRING match (s7a:939)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
# ---- THE SIX IDS WHOSE SUBJECT TAG IS NOT `rk-kernel`, each filtered IN THE CRATE THE DESIGN NAMES.
# ---- design §9.1 is the crate-name authority under ADR-0099 clause 5 and maps the register's subject tags:
# ---- `rk-store-artifacts` is a module of `rk-store`, `rk-port-toolchain` of `rk-ports`, and AC-319's tag is
# ---- `rk-types` outright. Filtering those six under `-p rk-kernel` selects ZERO tests, cargo exits 0
# ---- printing `0 passed`, the by-name grep fires and the block HALTS on a correct crate. §7b-ii.12 says the
# ---- module location is the design's and not this stage's to decide — which is why the filter follows the
# ---- design rather than forcing every test into the kernel. NOTE THE CONSEQUENCE, stated at §7b-ii.12:
# ---- each of these crates has a checkpoint already recorded as passed, so any edit inside one makes that
# ---- pass PROVISIONAL and the row is marked in the same commit (00-preamble.md:232-237).
for p_t in "rk-types ac319_quarantine_without_until_is_unconstructible" \
           "rk-types ac281_findings_ingest_rejects_an_incomplete_repro" \
           "rk-types ac293_review_outcome_carries_the_reviewed_tree_digest" \
           "rk-store ac305_finding_identity_is_stable" \
           "rk-store ac310_sink_accounting" \
           "rk-store ac329_total_test_count_source" \
           "rk-store ac333_ac_evidence_ledger" \
           "rk-ports ac337_mutation_feasibility_is_declared" ; do
  set -- $p_t
  out=$(cargo test -p "$1" -- "$2" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $2 exited $rc in $1"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${2}\b.* \.\.\. ok$" || { echo "FAIL: no test named $2 ran and passed in $1 — its register subject tag is not rk-kernel (design §9.1) and a filter matching nothing exits 0"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done

# ---- the trybuild harness, filtered on the HARNESS TEST'S OWN NAME and scoped with --test it, which is
# ---- correct HERE because `s7b` §7b.4 FIXES this harness's file at `crates/rk-kernel/tests/it/ui.rs`.
# ---- A filter spelled `trybuild` selects zero tests against the house naming convention in rk-git
# ---- (reconcile_runs_at_every_entry) and rk-store (a_state_field_without_a_tier_is_a_compile_error), where
# ---- neither name contains the substring — BUT NOT IN THIS CRATE, whose harness name does contain it:
# ---- measured 2026-09-13, `cargo test -p rk-ports --test it -- trybuild` selects
# ---- `ui::trybuild_pins_the_reason_each_port_fixture_fails` and reports `ok. 1 passed`. The reason to
# ---- filter on the full name here is not that `trybuild` matches nothing; it is that the full name is the
# ---- only spelling that cannot be satisfied by a harness some later stage adds. The two by-path
# ---- assertions below are what pin WHICH cases ran, which no name filter can see.
# ---- AND NEVER TWO FREE FILTERS AFTER A DOUBLED `--`: libtest OR-unions them, so `-- trybuild -- <name>`
# ---- passes whenever ANY test whose name contains `trybuild` passes. MEASURED on this tree 2026-09-13:
# ---- `cargo test -p rk-ports --test it -- trybuild -- no_such_test_xyz` reported `ok. 1 passed` at rc=0
# ---- against a test name that does not exist.
out=$(cargo test -p rk-kernel --test it -- trybuild_pins_the_reason_each_kernel_fixture_fails 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trybuild must-fail suite did not pass (rc=$rc) — a fixture COMPILED, or its .stderr drifted"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails\b.* \.\.\. ok$' || { echo "FAIL: no test of that name ran and passed — a filter matching nothing exits 0 (00-preamble.md:178)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# The two cases this sub-step adds, asserted BY PATH: a harness that runs proves nothing about a case that
# is absent, and a committed .stderr is what makes `compile_fail` mean "fails for THIS reason" (s7a §7a.8).
test -f crates/rk-kernel/tests/ui/ac294_block_constructor.rs || { echo "FAIL: AC-294's block-constructor must-fail fixture is absent — it is one of only two type-level bans this sub-step owns"; exit 1; }
test -s crates/rk-kernel/tests/ui/ac294_block_constructor.stderr || { echo "FAIL: AC-294's fixture commits no .stderr, so it is green whenever compilation fails for ANY reason"; exit 1; }
test -f crates/rk-kernel/tests/ui/ac244_gate_record_missing_a_field.rs || { echo "FAIL: AC-244's type-level fixture is absent — the type-level half is the discrimination (register AC-244, Verified by)"; exit 1; }
test -s crates/rk-kernel/tests/ui/ac244_gate_record_missing_a_field.stderr || { echo "FAIL: AC-244's fixture commits no .stderr"; exit 1; }

# ---- the three absence-based bans, each GUARDED, PAIRED and POSITIVELY CONTROLLED. Order matters: the
# ---- test -d runs first so a missing directory cannot read as a pass (Halt 8 shape (a)/(g)); the named
# ---- behavioural test above is the evidence and these greps are the floor; and the positive control
# ---- asserts the pattern still matches where it must, so a pattern that silently stops matching is caught
# ---- rather than reported as a clean tree.
test -d crates/rk-kernel/src || { echo "FAIL: crates/rk-kernel/src absent — every recursive grep below would read as a pass over a missing directory (Halt 8 shape (a))"; exit 1; }
test -d crates/rk-kernel/tests/ui || { echo "FAIL: crates/rk-kernel/tests/ui absent — AC-294's compile-fail case and its construction-site control cannot run, and an absence ban with no control is not an assertion"; exit 1; }
test -d crates/rk-kernel/tests/fixtures/lint-controls || { echo "FAIL: crates/rk-kernel/tests/fixtures/lint-controls absent — the AC-285 and AC-286 positive controls live there and NOT under tests/ui/, because a trybuild glob case must FAIL to compile while a grep control must not be a compile-fail case at all (§7b-ii.10)"; exit 1; }

# ---- SCOPED BY MODULE PATH, NEVER BY DIRECTORY. `s7b` §7b.4 says "the file layout is not this step's to
# ---- decide" (00-preamble.md:55) and names two MODULE paths, not two directories; §7b-ii.1 adds nothing to
# ---- that. `grep -rn … crates/rk-kernel/src/policy/` exits 2 on the idiomatic `src/policy.rs` layout and on
# ---- `src/review/classify.rs`, and the rc guard then FAILS the block for a LAYOUT reason rather than a rule
# ---- reason. So the search is over the whole `src/` tree and is FILTERED to the files whose path names the
# ---- module — which is what the criterion is about, and which admits `policy.rs`, `policy/mod.rs` and
# ---- `review/policy/*.rs` alike. §7b-ii.12 states where the AC-285 and AC-286 lints themselves live: NOT in
# ---- the module they police, so the ban cannot fire on its own implementation.
# AC-285: `finding.evidence_label` has no reader inside the policy module. Comment lines are excluded — the
# house pattern is a module header quoting the rule it governs (crates/rk-store/src/lib.rs:1-20).
raw=$(grep -rn 'evidence_label' crates/rk-kernel/src/ --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the evidence_label search could not run (grep rc=$rc; 2 = no such path) — a count of 0 here would be a fake 0; AC-285"; exit 1; }
raw=$(printf '%s\n' "$raw" | grep -E '^crates/rk-kernel/src/([^:]*/)?policy(\.rs|/)')
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
[ -z "$hits" ] || { echo "FAIL: finding.evidence_label is read inside rk-kernel::policy on a non-comment line — AC-285 says the model label is retained only on the persisted finding"; printf '%s\n' "$hits"; exit 1; }   # → silent
grep -q 'evidence_label' crates/rk-kernel/tests/fixtures/lint-controls/ac285_policy_reads_the_model_label.rs || { echo "FAIL: the AC-285 positive control does not contain the token this ban searches for — the ban above is vacuous and would stay silent over any tree"; exit 1; }

# AC-286: the classifier module reads no clock, no env and no file. Same three parts, same path filter.
raw=$(grep -rnE 'std::fs|std::time|std::env|SystemTime|Instant' crates/rk-kernel/src/ --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the classifier purity search could not run (grep rc=$rc; 2 = no such path); AC-286"; exit 1; }
raw=$(printf '%s\n' "$raw" | grep -E '^crates/rk-kernel/src/([^:]*/)?classify(\.rs|/)')
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
[ -z "$hits" ] || { echo "FAIL: the classifier module names a clock, an env read or a filesystem call on a non-comment line — AC-286's lint is on the module's own SYNTAX, because a crate-level dependency-edge check would not catch a transitive route"; printf '%s\n' "$hits"; exit 1; }   # → silent
grep -qE 'std::time|SystemTime' crates/rk-kernel/tests/fixtures/lint-controls/ac286_classifier_reads_the_clock.rs || { echo "FAIL: the AC-286 positive control carries none of the banned tokens — the ban above is vacuous"; exit 1; }

# AC-294: zero block-value CONSTRUCTION sites outside rk-kernel::policy. The trybuild case above is the
# evidence; this is the floor, and the positive control is the fixture that must carry a construction site.
# THE PATTERN ANCHORS ON CONSTRUCTION, NOT ON THE TOKEN. A bare `Block\s*\{` fires on `pub struct Block {`,
# `impl Block {`, `impl Debug for Block {`, `enum BlockSet {` and every `match … { Block { .. } => }`
# destructuring pattern — all correct constructs AC-294 does not forbid, and all of which would FAIL a
# correct implementation whose Block type lives outside src/policy. AC-294 bans construction of a block
# VALUE; the real mechanism is the private constructor, and this grep is only its floor.
# THE PIPE IS SPLIT: `$(cmd | grep -v …); rc=$?` captures the LAST stage's status, so a missing src/ tree
# gave rc=1 and read as a clean tree (halts.md:69-71 shape (c), :83-84 the rule). The first capture is
# asserted before the filter runs.
raw=$(grep -rnE '\bBlock(Set)?\s*::\s*new\b|=[[:space:]]*Block(Set)?[[:space:]]*\{|\([[:space:]]*Block(Set)?[[:space:]]*\{' crates/rk-kernel/src/ --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the block-construction search could not run (grep rc=$rc; 2 = no such path) — a count of 0 here would be a fake 0; AC-294"; exit 1; }
raw=$(printf '%s\n' "$raw" | grep -vE '^crates/rk-kernel/src/([^:]*/)?policy(\.rs|/)')
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)')
[ -z "$hits" ] || { echo "FAIL: a block value is constructed outside rk-kernel::policy on a non-comment line — AC-294 makes that a BUILD failure, and this grep is the floor beneath the trybuild case"; printf '%s\n' "$hits"; exit 1; }   # → silent
grep -qE '\bBlock(Set)?\s*::\s*new\b|=[[:space:]]*Block(Set)?[[:space:]]*\{|\([[:space:]]*Block(Set)?[[:space:]]*\{' crates/rk-kernel/tests/ui/ac294_block_constructor.rs || { echo "FAIL: the AC-294 fixture constructs no block value, so neither the trybuild case nor the grep above is asserting anything"; exit 1; }

# ---- the two lattice instruments, which are different criteria over the same catalog and must not be
# ---- credited to one lint. AC-303 is the kernel's (activity × rigor) confirm-step lint, asserted by name
# ---- above; AC-104's per-(family, rigor) totality check is lattice.py's, re-run here at its OWN output
# ---- strings. spikes/p9/lint.py is NOT run here and must not be: it is red BY DESIGN from Step 4a until
# ---- s7e §7e.2 disposes of the seven `auth` sites (errors=3 warnings=2, rc=1, measured 2026-09-13).
test -f spikes/p9/lattice.py || { echo "FAIL: spikes/p9/lattice.py absent — AC-104's instrument"; exit 1; }
out=$(python spikes/p9/lattice.py --validate 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: lattice.py --validate exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -q 'validate errors=0' || { echo "FAIL: lattice.py --validate did not print 'validate errors=0' — AC-104's per-(family, rigor) totality check is not green"; printf '%s\n' "$out" | tail -20; exit 1; }   # → validate errors=0 [measured 2026-09-13]
hits=$(printf '%s\n' "$out" | grep -E 'holes=[1-9]')
[ -z "$hits" ] || { echo "FAIL: lattice.py --validate reports a family hole — AC-104 (totality) and AC-105/AC-303 (monotonicity) are separate obligations on the same enumeration and both are errors, not warnings (ADR-0081 clause 2)"; printf '%s\n' "$hits"; exit 1; }   # → silent
# POSITIVE CONTROL for the line above, which passes on absence: the output must still CARRY `holes=` keys at all.
# If lattice.py renames the key the ban goes permanently silent and reads as a hole-free lattice.
c=$(printf '%s\n' "$out" | grep -c 'holes=' | tr -d '[:space:]')
[ "${c:-0}" -ge 1 ] || { echo "FAIL: lattice.py --validate printed no 'holes=' key at all (it printed 15 on 2026-09-13) — the hole ban above is searching for a string this tool no longer emits"; exit 1; }   # → 15
# ALIGNMENT-TOLERANT AND ANCHORED: bisect.toml:16 is column-aligned (`max_rigor    = "recommended"`, four
# spaces), so the single-space pattern returned rc=1 on a CORRECT and UNCHANGED tree [measured 2026-09-13] —
# the same defect PROGRESS.md:325 records against Checkpoint 6's gix-pin line. The guard's scope is "exactly
# one ceiling declaration"; alignment is incidental, so the pattern moves and the manifest is left alone.
grep -qE '^max_rigor[[:space:]]*=[[:space:]]*"recommended"' spikes/p9/catalog/bisect.toml || { echo "FAIL: bisect is no longer ceilinged — P7-CEIL (ADR-0093 decision 5; DECISIONS-2026-09-11 #1) is what makes the confirm-step and quality-block monotonicity lints green on the shipped catalog (F37)"; exit 1; }

# ---- the standing prohibition on what this stage's close-out may claim (ADR-0101 clause 5). The gates
# ---- exist and fail closed; S-SUBTLE-CB is registered and UNRUN, so no document may say more than that.
# ---- SCOPE IS THE CRATE ONLY, AND THIS FILE IS DELIBERATELY NOT SEARCHED. A grep whose pattern names the
# ---- phrase it forbids matches its own pattern line and its own FAIL message when it searches the file it
# ---- lives in — s7a:1032-1039 records that shape ('an assertion that its target's own failure message
# ---- satisfies') as a Halt 8 shape this programme met for the first time on 2026-09-13, and an exclusion
# ---- list tuned to this file's own wording would be the route-around golden constraint 9 names. The trade
# ---- is stated rather than implied: this line polices the CRATE, and what PROGRESS.md, a commit message
# ---- or this stage's close-out note says is policed by reading, not by a grep.
raw=$(grep -rniE 'guarded against false approval|calibrated against false approval' crates/rk-kernel/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the false-approval search could not run (grep rc=$rc; 2 = no such path)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a claim that the review gate is guarded against false approval appears in rk-kernel — ADR-0101 clause 5 forbids it until S-SUBTLE-CB runs, and AC-312 carries the same sentence"; printf '%s\n' "$raw"; exit 1; }   # → silent
```

### 7b-ii.14 Acceptance set

The claimed set is the register's three gate areas — **`human-gate` AC-240..AC-279 plus AC-524 (40 in force)**,
**`review-gate` AC-280..AC-319 plus AC-530 (40)**, **`verify-gate` AC-320..AC-359 (40)** — **plus AC-455**,
cross-listed from the migration range where `s7d` excludes it by name, and **AC-239's list-is-data half**, which the
register assigns to *"the runbook's 7b/7d acceptance sets (PIA-072)"*. It is written as explicit ranges minus the ids
excluded, each named with its reason (`00-preamble.md`:229-231; PIA-070, PIA-071).

**This section claims by id ONLY what a discriminating test at this stage can pin.** Everything else is **ARMED, NOT
DISCHARGED**, by id and reason, because golden constraint 10's form is that powerless is reported as powerless and
never as a pass, and golden constraint 12's *Discriminate* bullet requires a test that fails when the behaviour is
absent **at the stage**.

**At 7b-ii, `crates/` holds `rk-git`, `rk-ports`, `rk-store`, `rk-types` and the `rk-kernel` Step 7b wrote** [measured
2026-09-13]. **There is no `rk-adapter-*`, no `rk-cli` and no `doctor`; `.redkiln/` does not exist; no lane has run and
no corpus run is recorded.**

**DISCHARGED HERE, by a discriminating test at this stage** — each named in §7b-ii.11 and asserted by name in the
block above:

- **Human gate:** AC-244 (its type-level and deserialisation halves), AC-245, AC-246, AC-249, AC-250, AC-270,
  AC-272, AC-524 (built and tested; **PROVISIONAL on P8-B**, so not claimed as evidenced).
- **Review gate:** AC-281, AC-283, AC-282 (its `rk-kernel` half), AC-285, AC-286, AC-287, AC-288, AC-289 (bar the
  workflow-only fixture), AC-290 (bar its doctor rule), AC-293, AC-294, AC-296, AC-298, AC-299, AC-300 (its
  pack-validation half), AC-301 (its refusal; the lift is ARMED), AC-302, AC-303, AC-304, AC-305, AC-306 (bar its
  doctor rule), AC-307 (bar its corpus replay half), AC-310 (bar its doctor rule), AC-313, AC-314 (bar its doctor
  rule), AC-318, AC-319, AC-530 (bar its doctor rule), and AC-284 **built and tested, not claimed as evidenced**
  (PROVISIONAL on S-CLASS-REPLAY).
- **Verify gate:** AC-326, AC-329's field-semantics half, AC-333, AC-335, AC-337's fixture-pack half, AC-345, AC-346.

**CROSS-LISTED — claimed at both sites, kernel half claimed here.** Cross-listed means *claimed at both sites* and is
the opposite of subtracted, so these are **not** exclusions and are no longer filed as such. *(They sat under the
"Excluded, by id and reason" heading until 2026-09-13, which read as subtracting twenty-five ids the file was in fact
claiming.)*

- **AC-243, AC-267, AC-279** — `s7c`:195 cross-lists all three; **the kernel halves are claimed here** and neither
  needs an adapter. AC-243's instruments are *"a replay assertion over the event log — no `dispatch` event with a
  timestamp between `gate_opened` and the gate's terminal event for the same item"* and the P8 dispatch-count
  regression; AC-279's is *"a count assertion on prefix-rewrite events per `gate_resumed`, asserted ≤ 1"*. Both are
  named in §7b-ii.11 and asserted by name in the block. **AC-267 is PROVISIONAL and is BUILT, NOT CLAIMED, and it
  alone among the cross-listed ids has no checkpoint line here — deliberately, and the reason is stated.** Its
  instrument is *"a metric assertion on the `gate_resumed` event's `cache_creation_input_tokens`, tracked as SM-073
  and asserted as a regression threshold in CI **against the recorded baseline**"*, and its second subject tag is
  `rk-adapter-claude`, which does not exist until 7c. **There is no recorded baseline at this stage and no adapter
  to emit the metric**, so a line naming it would assert a threshold against nothing. The `gate_resumed` event's
  field is authored here; the threshold is 7c's. `s6`:351-352 governs it verbatim — *build it; do not claim it*.
- **AC-282's `rk-kernel` half** — the unit test over a recorded transcript with `structured_output` absent, plus the
  F20 golden transcript. `s7c`:195 claims the adapter half.
- **AC-455** — **both policy-table rows**, including the must-pass control. Its **`doctor` half is ARMED**, below.
- **AC-239's list-is-data half** — **and the criterion itself is PROVISIONAL** (S-CODEX not reported in the brief;
  the register's §7 State line counts it among its three provisional ids). Only the list-is-data half is claimed,
  per the register's 2026-09-11 phase note routing it to *"the runbook's 7b/7d acceptance sets (PIA-072)"*; the
  **Codex-present half is excluded** (`rk-adapter-codex` is Phase 2, ADR-0094 decision 2). `s7c`:193 also holds the
  id by a bare range sweep with no reason stated — raised below, not settled here.

**ARMED, NOT DISCHARGED — by id and by reason.** Checkpoint 7b-ii may not be read as discharging any of the following,
and each is recorded in `PROGRESS.md` §1 as carried forward:

| id | register subject and instrument | missing at 7b-ii | where it can run |
| --- | --- | --- | --- |
| **AC-317** | the per-AC mutation harness's **invocation test in the fix and story lanes**, plus the report-time refusal | a lane run | **Step 8**, which consumes the fix lane (exit check (1)). The harness itself is built here |
| **AC-334** | the flip predicate over a declared patch, a named suite re-run and a tree revert | a **real** toolchain adapter — at this stage the predicate is exercised against the **Toolchain fake** only | **7c** for the real rows; the predicate and its F25 regression row are built and red-first here |
| **AC-295** | a replay test over a **recorded corpus run** at both weights | a recorded corpus run | **Step 8** |
| **AC-307** *(replay half)* | `count(blocks) == count(block_events)` **over the corpus** | a recorded corpus run | **Step 8**; the unit and lint halves run here |
| **AC-354** | a replay-equivalence test over a **recorded corpus of verify runs** | a recorded corpus run | **Step 8** |
| **AC-240, AC-241, AC-248, AC-251, AC-256, AC-260, AC-264, AC-268, AC-277, AC-280, AC-290, AC-300, AC-306, AC-310, AC-314, AC-323, AC-327, AC-330, AC-337, AC-349** *(their `doctor`/CLI halves only)* | a `doctor` rule-table row or a CLI integration test | `rk-cli` / `doctor` | **7d**. Each of these ids has a second instrument that DOES run here and is listed as discharged above; reporting a partly-discharged id as unmeasured is the blur golden constraint 10 forbids, so both states are carried. **AC-323 is the exception and is wholly ARMED**: its register subject is `rk-cli` / `rk-store-artifacts` and no `rk-cli` exists at this stage, so it has no half that runs here *(it was swept into "built here" until 2026-09-13)* |
| **AC-455** *(the `doctor` rule)* | *"a `doctor` rule reporting the same at configuration time"* — its third instrument, beside the two policy-table rows claimed above | `rk-doctor` is `rk-cli`'s (design §9.1, ADR-0099 clause 5) | **7d** — **but see RAISED below: `s7d`'s acceptance set EXCLUDES AC-455 by name and cross-lists it here, so no stage claims this row** |
| **AC-265** | *"the P8 fixture promoted to a kernel regression test, running a real `taskkill /T /F` (not a simulated abort) and comparing digests"* | no `taskkill` boundary is crossed at this step, and **authoring a fixture is not a discriminating test** | **Step 8 check (3)**, which the register's own runbook-placement note says this criterion is what it cites. The fixture and the regression test are **authored** here; the id is **not** claimed *(it was listed as DISCHARGED on its authoring half until 2026-09-13)* |
| **AC-262, AC-325, AC-328, AC-329** *(adapter half)*, **AC-331, AC-350, AC-351** | a differential/adapter/child-process test over a **real** adapter | any `rk-adapter-*` | **7c** |
| **AC-312** | the release-regression corpus job over seeded, control and **prior-approval framing** fixtures | the framing fixtures are **authored at 7c** and S-SUBTLE-CB is unrun | **7c** authors them; the arm is registered and unrun. ADR-0101 clause 5 governs what may be said about it |
| **AC-266, AC-271, AC-308, AC-356** | PROVISIONAL on P8-B, P8, E3-SEV-BLIND and S-BIN respectively — all registered and unrun | a spike result | **built and tested here, not claimed as evidenced** (`s6`:351-352: *"Build them; do not claim them"*). AC-271 is *"not answerable from P8"* in its own text; `s7c` excludes AC-356 for the same reason |
| **AC-289** *(workflow-only fixture)* | the fixture the register says *"is added once S0.1-SCOPE runs"* | S0.1-SCOPE, which left 8 of 23 items unmeasured by construction | when S0.1-SCOPE runs |
| **AC-301** *(the lift)* | the refusal *"lifts only when S-CASCADE … reports the cascade's recall loss"* | S-CASCADE, registered and unrun | the refusal is built here; the lift is not this stage's |

**Excluded, by id and reason** — **and only genuinely excluded ids are here**; ids claimed at two sites are in the
CROSS-LISTED block above, and ids claimed and built here without an individual test are in the ledger below.

- **AC-255** and **AC-297** — **cut, never reuse** (register §0.4; the §8 and §9 area keys).
- **AC-273's human half, AC-275, AC-276, AC-309, AC-311, AC-357, AC-358** — **HUMAN-GATED and not machine-decidable**,
  in the register's own words (*"routed to a human gate; not machine-decidable"*). The three area State lines count
  them: §8 *3 human-gated (AC-273, AC-275, AC-276)*, §9 *2 human-gated (AC-309, AC-311)*, §10 *2 human-gated (AC-357,
  AC-358)*. **Nothing at this or any stage discharges them, and Checkpoint 7b-ii may not be read as reporting on
  them.** Their machine halves are claimed above and separately instrumented: AC-273's placement-table lint and its
  no-cost-model test; **AC-274** (the totality and allowlist lint) is AC-275's; **AC-260** (field presence) is
  AC-276's; AC-311's is *"every sink record reaches a terminal disposition or is surfaced as ageing"* (SM-089).
  **AC-357 is the human gate ADR-0104 decision 4 reserves as the only answer to an unpinned AC, and it is not built
  here.** **No CI job may pass or fail on AC-273, AC-275, AC-276, AC-309, AC-311, AC-357 or AC-358.**
- **AC-263** — `rk-cli`'s non-CLI-channel refusal; **7d's**.
- **AC-269** — its instruments are *"a card-validator test over every card in the catalog"* and *"a build-configuration
  test that the answer API is not linked into the **adapter's** tool registry"*: **adapter-side, 7c's**, cross-listed
  here and not claimed. Its kernel-side sibling — that no channel but `answer` closes a gate — is AC-239's and
  AC-263's.
- **AC-282's adapter half** — `s7c`:195 claims it; the `rk-kernel` unit test and the F20 golden transcript are here.

**CLAIMED AND BUILT HERE, NOT INDIVIDUALLY ENUMERATED ABOVE — the ledger of what is still owed, not a discharge.**
*(Until 2026-09-13 twenty-three of the 120 gate ids in force appeared **nowhere** in this file — not discharged, not
armed, not excluded. Thirteen of them were reached by no sentence at all, and the human-gate and review-gate
partitions had no sweep clause, while the verify-gate sweep sat under an "Excluded" heading. The pre-split `s7b`
claimed all 120 by one bare range sweep, which was weaker but total; the enumeration below restores totality without
restoring the sweep's vagueness. `grep -rl 'AC-242' runbook/` returned nothing before this paragraph.)*

The remainder of the three claimed ranges, by id:

- **Human gate:** **AC-242, AC-247, AC-252, AC-253, AC-254, AC-257, AC-258, AC-259, AC-261, AC-278** — all
  `rk-kernel`, all with ADR-0017 (AC-259 also ADR-0022/ADR-0038, AC-261 also ADR-0009) on their `decisions:` line.
- **Review gate:** **AC-315, AC-316** — both `rk-kernel`.
- **Verify gate:** **AC-320–AC-322, AC-324, AC-332, AC-336, AC-338–AC-344, AC-347–AC-353, AC-355, AC-359**.

Each is claimed and built here to its own *Verified by* line. **Three of them are load-bearing enough to name their
reason:** **AC-278** is AC-240's twin in the register's §0.3 (*"Opening a gate is a write and an exit |
**AC-240 + AC-278**"*) and its own *Why* says *"split from AC-240 because exit 0 does not imply it"* — crediting the
pair to AC-240's instrument is the twin failure §7b-ii.4 argues against for AC-303/AC-105; **AC-254 and AC-259** are
two of the four ids `PROGRESS.md`:77 records as PIA-050's remediation set; **AC-258**'s clock-independence half is
asserted inside `ac270_gate_state_from_the_log_alone`, which the register says *"doubles as AC-258's … instrument"*.

**None of these ids has a named test in §7b-ii.11 or a filter in the block above**, so each is reached only by the
crate-wide `cargo test -p rk-kernel` floor, which `00-preamble.md`:186-189 says is *"satisfied by an implementation
nothing pins"*. **Before this step is RUN, either author a named test per id and add it to the block, or move the id
to ARMED with its reason.** They are not moved to ARMED now because nothing external blocks them — the code and the
tests are this stage's, and what is missing is the lines.

**RAISED, NOT SETTLED — allocation questions this file may not decide** (`00-preamble.md`:55; `s7a`:1217):

- **AC-291 and AC-292 are ORPHANED.** Both are `rk-git`'s, with structural instruments — *"a structural assertion on
  the spawn configuration"* and a *"clone-shape assertion covering each of those four properties"*. Until 2026-09-13
  `s7b`:256 *(at `60f17c6`, the pin this historical anchor reads at; the working tree's :256 carries unrelated
  prose after that day's rewrite)* said *"AC-291/AC-292 are Step 6's"*; **Step 6's acceptance set does not contain them and Step 6 closed at
  `ea83eac`**, and `grep -rn 'AC-291' runbook/` returned that one line and nothing else [measured 2026-09-13].
  PIA-047's remediation assigned them to Step 6 (`references/2026-09-10-pre-implementation-audit.md`:3403) and
  `PROGRESS.md`:168 records PIA-047 as *done* with `s6-store-and-git` among the files it touched. **Owed:** either an
  amendment adding them to `s6`'s set with Step 6's row marked `passed ea83eac, re-run pending AC-291/AC-292`, or an
  erratum re-assigning them to **`s7c`**, whose review-dispatch work is where the reviewer is spawned and the clone is
  made — the better-aligned branch, because this stage's crate may hold no adapter edge. **Until one is written, no
  stage claims them and no checkpoint tests them.** They are not claimed here.
- **Step 8 keeps no acceptance set.** `grep -n -i 'acceptance' runbook/phase-1/s8-exit-criterion.md` returns nothing
  [measured 2026-09-13], against `00-preamble.md`:229-231, which gives every Phase-1 stage one. Six ids above are
  ARMED *to Step 8* — AC-317, AC-295, AC-307's replay half, AC-354, AC-265's kill, AC-289's corpus half — and nothing
  at Step 8 records their discharge. **An acceptance set for Step 8, or an erratum, is OWED**; `s8` is not edited in
  this pass.
- **PIA-047 prescribed a `redkiln answer` round-trip as one of six 7b-ii checkpoint items**
  (`references/2026-09-10-pre-implementation-audit.md`:3401-3403). `redkiln` is `rk-cli`'s and `rk-cli` is Step 7d, so
  the round-trip cannot run here and `ac524_fresh_record_on_rereach` does **not** replace it — it tests a different
  behaviour. The channel is instrumented one stage later: the register's §12.2, *"`answer` — the only Phase 1 gate
  channel"*, holds AC-405–AC-410, all `rk-cli`, all inside `s7d`'s claimed AC-400–AC-439. **The round-trip is DEFERRED
  to Checkpoint 7d** and recorded there; **an erratum against PIA-047 is OWED** for prescribing a `rk-cli` instrument
  to a pre-`rk-cli` stage.
- **Two `doctor` rules are claimed by no stage, and this file does not choose their owner.** **AC-530's** `doctor`
  rule-table row and **AC-455's** configuration-time `doctor` rule are both `rk-cli`'s instrument, and `s7d`'s
  acceptance set claims neither: it **excludes AC-455 by name** and cross-lists it here, and AC-530 appears nowhere
  in it [measured 2026-09-13]. Both are reported ARMED to **7d** in the table above, which is where they *should*
  run — but *"naming a destination is not routing it there"* (`s7a`:1217), and until `s7d`'s set carries them the
  rows are orphaned. **Owed: an amendment adding both to `s7d`'s acceptance set, or an erratum re-assigning them.**
  Recorded in `PROGRESS.md` §2 for adjudication; `s7d` is not edited here.
- **AC-282's stage placement and AC-318's split are owed records.** PIA-047's remediation left AC-282 in 7c
  (`references/2026-09-10-pre-implementation-audit.md`:3403) while `s7c`:198 excludes *"AC-240–AC-359's kernel halves
  (7b-ii's)"*, so AC-282's two `rk-kernel` instruments — the unit test over a recorded transcript and the F20 golden
  fixture — were claimed by nothing until this file; and **AC-318**, which the register itself split out of AC-282
  and which is pure `rk-kernel`, appeared in no runbook file at all. **Owed: an erratum against PIA-047's placement,
  and a note in `s7c` recording the split.** Neither is written here.
- **AC-239 is double-claimed and only one site says so.** It is claimed here on its list-is-data half, with its
  PROVISIONAL label; `s7c`:193 also holds it inside the bare range `AC-200–AC-239 minus AC-207` with no reason
  stated. `s7b` §7b.15 raises the same question and correctly does not settle it. **Routed to the operator as an
  open allocation: an erratum aligning the two sets, or a reason-bearing note in `s7c`, is OWED.**
- **The register reconciliation is OWED and is not performed here.** This file claims **122** ids — the three gate
  areas at 120 in force, plus AC-455 and AC-239's list-is-data half — and names its
  discharges and arms by id; what it does not do is reconcile the three area State lines' binding/provisional/
  human-gated counts against the per-id dispositions above. That is a register-side count, it moved during this
  session, and `00-preamble.md`:7-10 gives the register every criterion's state. **Record it in `PROGRESS.md` §2 against
  Step 7b-ii.**

**Checkpoint 7b-ii is the gate milestone M3 names**, and this file is where that is said. `INDEX.md`:80 reads
*"**M3 — the kernel decides** | **Checkpoint 7b-ii passed** (after Checkpoint 7b)"*, moved there on 2026-09-13 in the
same change as the split; `s7b` §7b.15's close-out reads the same way. **ADR-0101 clause 5 bounds what that row may
be read as**: until S-SUBTLE-CB runs, no document may claim the review gate is guarded against false approval — not
`PROGRESS.md`, not a commit message, not the milestone row itself. This section's gate-check banner carries the
prohibition; this paragraph carries the attribution, which was missing: `M3` appeared nowhere in this file until now.

**Spikes this step consumes:** none — F25, F29, F31, F18, F19, F20, F27 and F38 are read, not run. **Spikes it owes:**
S-CLASS-REPLAY (AC-284), S-SUBTLE-CB (AC-312), S-CASCADE (AC-301's lift), S0.1-SCOPE (AC-289's workflow fixture),
P8-B (AC-524, AC-266), E3-SEV-BLIND (AC-308) — all registered, none run, and none of them gates this stage's build.
