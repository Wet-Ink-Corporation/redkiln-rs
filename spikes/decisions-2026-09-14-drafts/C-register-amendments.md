# DRAFT · The register and errata work owed by the 2026-09-14 decisions

**Status: DRAFT. Nothing minted.** Every item below edits a record **above** the runbook in ADR-0092's chain —
the AC register, `references/adr/ERRATA.md`, or the design — and is therefore the operator's act. Each carries the
measurement that grounds it so it can be accepted or refused on evidence rather than on my say-so.

Seven items, ordered by what they unblock.

---

## C-1 · Allocate **AC-532** — the executed-once ledger (D-12)

**Why.** `s7b` §7b.10 routes ADR-0114 decision 2's `executed: { first, last }` write *"through **AC-110**"*, but
AC-110's own GIVEN/WHEN/THEN is about an extended-file reference failing as a load-time configuration error and
says nothing about a ledger. Under ADR-0092 a runbook cannot add a THEN. The test
`ac110_executed_ledger_first_and_last` is built and green and measures **ADR-0114 decision 2**, so today it
measures a record rather than a criterion.

**Next free id: AC-532** — allocated ids run to AC-531 (the 2026-09-11 amendment added AC-521..AC-531, *"every one
outside an exhausted range, §0.4"*) [measured 2026-09-14].

> **AC-532** — `rk-kernel` · evidence: ADR-0114 decision 2 [documented] · decisions: **ADR-0114**, ADR-0022
> GIVEN an activity committing for the first time in a run
> WHEN the kernel appends its `activity_committed` event (ADR-0022's activation record)
> THEN the catalog record's `executed.first` is pinned to that `{ run_id, item, at }` and is never rewritten
> AND every subsequent `activity_committed` for that activity updates `executed.last` and leaves `first` unchanged.
> *Verified by:* a unit test committing one activity twice and asserting `first` is stable across both while
> `last` moves; a negative asserting an activity that never commits reads `executed: never`.
> *Note:* the kernel is the only writer — nothing else sees `activity_committed`, and Step 7e authors TOML only.
> `doctor`'s never-executed count is **7d's** and `lint` **warns, never fails**, past the pack's declared
> `rot_after` (a pack setting, not a kernel constant).

**Rejected, and why it matters:** amending AC-110 to carry the clause would bolt a second unrelated subject onto
one id, against the register's own definition of a criterion — *"One GIVEN/WHEN/THEN naming **one** observable
behaviour"* — and it is the collapse shape AC-071's residual-risk note warns about.

**Consequence:** `ac110_executed_ledger_first_and_last` keeps its name (Checkpoint 7b asserts that literal); what
changes is **what it claims**. A one-line note in §7b.15 should re-point it at AC-532.

---

## C-2 · Amend **AC-023** and resolve **§0.3's C3** (D-13)

**Why.** C3 is the register's second open contradiction and the same shape as C1: AC-023 refuses an in-run
per-item token appetite raise *"unless it arrives as a typed budget-change event with an author"*; AC-389 refuses
it outright. The register says *"a build cannot implement both refusals."* Slice 4 built only the refusal both
share. **The operator resolved it for AC-389 on 2026-09-14.**

**The escape hatch is not merely unbuilt — it is unbuildable without a record**: a typed budget-change event needs
an `EventKind`, the enum is **closed at 29**, and ADR-0116 permits minting only by record.

Proposed amendment to **AC-023**:

> ***Amended 2026-09-14 — the "unless" clause is CUT.*** The per-item token appetite is fixed at triage and an
> in-run raise is **refused outright**; raising it means returning the item to triage. The previous clause
> *"unless it arrives as a typed budget-change event with an author"* is withdrawn, and §0.3's **C3** is resolved
> for **AC-389**. Its second instrument — *"an event-schema test that every appetite change carries an author"* —
> is withdrawn **with** the clause rather than left ARMED against no stage, because with no in-run raise there is
> no appetite-change event to carry an author. **AC-396** continues to route the appetite **value** to a human
> gate, which suits this mechanism unchanged.

And a resolution line on **§0.3's C3 row**, in the form C4 and C5 already use:

> **Recorded and resolved 2026-09-14** by the operator, in favour of **AC-389**: an in-run appetite raise is
> refused outright. AC-023's *"unless"* clause is cut by the dated amendment above. No `EventKind` is minted;
> the closed 29 stand.

---

## C-3 · Fold the `--stay` class into **AC-294**, and withdraw Phase-1 exit criterion (5) (D-14)

**Why.** Phase-1 exit criterion (5) gates on the `--stay` compile-fail fixture and **no criterion states it** —
searching `--stay|unrepresentable` returns **four** hits and only **two** are on the subject — AC-446's corpus list (:6548) and ADR-0107's flag denylist (:5923); the other two use *unrepresentable* of a fail-open path (:3299) and of drift (:6906) [measured 2026-09-14]. §7b.15
names exactly two lawful routes; the operator took the fold-in. The fixture
`tests/ui/stay_class_state_is_unconstructible.rs` and its `.stderr` exist as of Slice 4e.

AC-294 already owns *"a private constructor plus a module-boundary architecture lint asserting zero construction
sites outside `rk-kernel::policy`; a compile-fail test (`trybuild`) that adds one"* — the identical instrument
family, in the same crate.

> ***Amended 2026-09-14 — the `--stay` resumed state folds in here.*** This criterion's THEN extends to the
> resumed/half-advanced run state the `--stay` class of corruption depends on: it is unconstructible outside the
> module that owns it, by the same private-constructor-plus-boundary-lint instrument, with
> `crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs` and its committed `.stderr` as the
> compile-fail case. **Phase-1 exit criterion (5) is withdrawn**: its only warrant was ADR-0001:42, which this
> register classes as *"a premise, not a checkable behaviour"*, and the obligation now has a criterion instead.

**Still owed separately:** the by-path assertion in `s8` check (5). Checkpoint 7b asserts the `.rs` and `.stderr`
by path; `s8` does not, so `cargo test -p rk-kernel -- trybuild` there is green whenever **any** case passes and
an absent `--stay` case reads exactly like a present one.

---

## C-4 · ADR-0122's three owed items

ADR-0122 decision 5 names these; none may be written by a build session.

**(a) Withdraw AC-391's *Cut*.** Its stated ground is refuted against its own cited source: proposal §4.3's Inner
cell reads *"**Observe, record, do not duplicate.** Inner attempts count toward the activity's budget"* and the
*Cut* quoted the bolded first sentence and stopped. Schema §4:83 (*"steps draw from it … **Inner harness retries
(`system/api_retry`) count.**"*) and design §3.6:338 say the same independently.

**(b) A resolution line on §0.3's C1 row**, in C4/C5's form — *and a correction to it*. The row says *"If C1
resolves against AC-391, ADR-0037 is superseded, never edited."* **It is not.** ADR-0037's entire `## Decision` is
one sentence — a load-time lint rule on a **declared** field — and nothing in it moves. ADR-0122 carries
`supersedes: []` and ADR-0037 keeps `superseded_by: []`. What overreached was four criteria **citing** ADR-0037 as
authority for a runtime rule it never states, and an interpretation cannot be superseded.

**(c) A dated clarification to design §3.6:337.** Its Budgets row states AC-021's rule *inside a cell about
runtime counters* — *"`attempts_total` counts the agent/tool steps of a **pass**, not the sum of retried steps
across passes"* — which is the row a later reader will use to attack ADR-0122 decision 3. The clarification should
say which quantity that sentence governs (the **declared** field, on ADR-0037's unit), so :337 and :338 cannot be
read as one.

---

## C-5 · Two errata

**E-0071-1 — AC-071's dead `--stability-record` path.** The criterion names
`--stability-record spikes/results/p1/stability.json`. Dead twice: nothing has ever written it, and
`spikes/results/` moved to the corpus under ADR-0013 / AC-465. Draft **A** proposes
`results/p1-arm-l-stability.json`, corpus-relative through `CORPUS_PIN`.

**A new erratum against AC-484's third instrument.** Its *Verified by* claims *"the same `deny.toml`
`[bans].deny` list AC-005 names (ADR-0100 decision 3), so the two instruments read one committed list."*
Measured 2026-09-14: that list names `temporalio-sdk`, `temporalio-sdk-core`, `graph-flow`, `dbos`, `durable` —
**five workflow engines, no adapter**. And it cannot become AC-484's: `cargo deny` bans are **workspace-wide**
while AC-484 is a **per-crate edge** property, so banning `rk-adapter-sqlite` workspace-wide would forbid it in
`rk-cli`, which legitimately depends on adapters at 7d. The clause does not survive contact with either file.
AC-484's other two instruments are built and green; this one is ARMED and the test's header says so.

**Also still owed, pre-existing:** **E-0079-2** — AC-071's THEN and `00-preamble.md`'s golden constraint 1 still
call ADR-0079 `proposed` where ADR-0090 superseded it at Step 4.4.

---

## C-6 · A note saying what `schema_version` formally is (D-10)

Design §3.5 / AC-014 name *"the output type's `schema_version`"* as memo-key component 8. **No such field exists**
— `grep -rn schema_version crates/rk-types/src crates/rk-ports/src` returns **0** [measured 2026-09-14]. The only
value carrying a version is `SchemaId(String)`, in a `Kind/vN` convention. D-10 takes the **whole id** as
component 8. A dated note should say so, and say why the alternative was refused: parsing the `vN` suffix invents
a convention no record states, has no defined behaviour on an id without one, and makes `RecordedResult/v1` and
`ChangeSummary/v1` **collide** in component 8 — weakening the property AC-014's nine-case test exists to pin.

---

## C-7 · A note fixing the exhaustion comparison per counter (D-6)

Neither criterion states its comparison. AC-024's GIVEN is *"a run that **reaches** its wall-clock budget"*;
AC-025's is *"a gate whose evidence-producing activity **ran out** of any budget"*. D-6 reads the verbs: `>=` for
the wall clock, `>` for attempts and tokens.

**The defect it fixes was live** [measured 2026-09-14]: **9 of 102** shipped activities declare exactly their
lawful ADR-0037 rule-8 minimum, so under a uniform `>=` each was exhausted by its own **successful** pass and
`evaluate_gate` could never close a gate on its evidence — `change-summary` 2/2, `change-summary-ue` 3/3,
`checklist-review` 2/2, `debt-logging` 2/2, `dependency-update` 6/6, `rolling-deploy` 4/4, `smoke-test` 2/2,
`t-shirt-sizing` 2/2, `unit-tests` 4/4.

A dated note against AC-024/AC-025 fixing the comparison per counter would close it. Until it lands, the reading
lives in `rk-kernel::workflow::budget`'s header with the measurement beside it.
