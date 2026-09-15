# LANDED · The register and design work owed by the 2026-09-14 decisions

*(Retitled 2026-09-15. This was filed on 2026-09-14 as "**DRAFT · The register and errata work owed by the
2026-09-14 decisions**", and **both halves of that title are now false**: it is not a draft, and under
`spikes/DECISIONS-2026-09-15.md` **D15-2** none of this work is errata — every item lands as a dated in-place
amendment of the AC register or the design, and `references/adr/ERRATA.md` is untouched.)*

**Status 2026-09-15: NOT A DRAFT ANY MORE — SIX OF THESE SEVEN SECTIONS HAVE LANDED IN FULL IN THE REGISTER OR
THE DESIGN, AND THE SEVENTH — §C-4 — HAS LANDED IN PART.** Checked section by section against the tree on
2026-09-15: **C-1, C-2, C-3, C-5, C-6 and C-7 are landed in full**; **§C-4** carries the five lettered items
ADR-0122 decision 5 names, and of those **(a), (b) and (d) landed, (e) owed no register edit, and (c) is STILL
OWED** as residual **R-2**. Each section's Status block quotes the clause that carries its verdict.
*(This read "**Status: DRAFT. Nothing minted.**", which was true when it was written on 2026-09-14 and false from
the moment the amendments landed on 2026-09-15 — see the corrected measurement below.)* Two residuals survive and
are named where they belong: **R-2** (ADR-0122 decision 5(c), inside §C-4) and **R-3** (§4's scope note, inside
§C-1). Every item below edits a record **above** the runbook in ADR-0092's chain — the AC register or the design —
and was therefore the operator's act; **`references/adr/ERRATA.md` is NOT among them**, because
`DECISIONS-2026-09-15` **D15-2** retired the erratum as the instrument for these corrections and that file is
untouched by this pass. Each carries the measurement that grounds it so it can be accepted or refused on evidence
rather than on my say-so. 🛑 **Read each section's Status block before the prose under it: the prose is kept as
history and several of its load-bearing claims are refuted.**

Seven items, ordered by what they unblock.

**Revised 2026-09-15.** Each section below now carries a **Status 2026-09-15** block saying what landed today and
what changed from the draft text beside it. Read the status block before the draft text under it: three of this
file's load-bearing claims were **refuted** against the primary records, and the draft prose is left standing only
so the refutation has something to point at.

🛑 **CORRECTED 2026-09-15 — the measurement that stood here was FALSE, and HOW it got that way matters more
than the number.** It read: *"`grep -c '2026-09-15' references/2026-09-08-acceptance-criteria.md` returns **0** and
`grep -c 'AC-532'` returns **0** — no register amendment in this file has landed."*

**Re-verified on this tree by QUOTATION, and the replacement counts are struck rather than refreshed:** the
register's own **§0 header** carries *"Further amended 2026-09-15"* … *"to land the five record items"*, its **§0
amendment note** carries *"The 2026-09-15 amendment adds one criterion, moves no label"*, and its **§0.1
allocation table** carries AC-532's row, *"packs-catalog (§4.6, beside AC-114"*. **The register amendments
LANDED**, on 2026-09-15, written by a sibling agent working in parallel in the same session. 🛑 **The counts are
NOT replaced with fresh ones:** this paragraph first reported **0**, a correcting pass wrote **61**, and **61 was
69 within the hour**. A count and a line number are the same kind of evidence — true only of the tree that existed
while it was being typed — so every clause here is cited by a fragment `grep -cF` will still find tomorrow.

⚠️ **This is the house's own recorded failure mode in a new dress** — *"a probe in another language is a proxy"* and
*"verify the operation, not a proxy for it"*. The new instance is narrower and worth naming on its own: **a real
command, correctly typed, run against the right file — at the wrong moment.** Both greps were taken before or during
the writes they claimed to report on, and the answer was then committed to this page as a **standing fact** with a
🛑 on it. A measurement is a fact only for as long as nothing else is writing, and in a parallel pass that window
can be shorter than the time it takes to type the sentence around it. **Re-measure at the moment you publish, and
record when you measured, not only what.**

**Consequently every status block below has been re-verified against the register itself and rewritten.** Each now
says **LANDED** or **STILL OWED** on a **quoted clause read out of the record itself**, not on a line number and
not on this header's false premise that nothing had landed. The four operator decisions at
`spikes/DECISIONS-2026-09-15.md` (`D15-1`…`D15-4`) are what unblocked them and fixed their text; what remained the
operator's act has, for six of these seven sections, since been performed **in full**, and for the seventh —
**§C-4** — **in part**, ADR-0122 decision 5(c) excepted.

---

## C-1 · Allocate **AC-532** — the executed-once ledger (D-12)

**Why.** `s7b` §7b.10 routes ADR-0114 decision 2's `executed: { first, last }` write *"through **AC-110**"*, but
AC-110's own GIVEN/WHEN/THEN is about an extended-file reference failing as a load-time configuration error and
says nothing about a ledger. Under ADR-0092 a runbook cannot add a THEN. The test
`ac110_executed_ledger_first_and_last` is built and green and measures **ADR-0114 decision 2**, so today it
measures a record rather than a criterion.

**Next free id: AC-532** — allocated ids run to AC-531 (the 2026-09-11 amendment added AC-521..AC-531, *"every one
outside an exhausted range, §0.4"*) [measured 2026-09-14].

> **Status 2026-09-15 — ✅ LANDED IN THE REGISTER. The id stands; the area and the THEN were corrected before it
> landed.** **Verified on this tree by quotation** [`grep -cF` over the register and the runbook; every fragment
> below returns ≥ 1]: AC-532 is allocated in `references/2026-09-08-acceptance-criteria.md` — in its **§0.1
> allocation table**, whose row reads *"packs-catalog (§4.6, beside AC-114"*; in **§4's range and state lines**,
> *"plus AC-527 and AC-532"* and *"AC-532 added 2026-09-15"*; in **§4's criterion body**, tagged `rk-catalog` and
> opening *"allocated outside this area's exhausted range"*, whose **Allocation** paragraph cites D15-3 and whose
> *"Both readings are stated because the build already took them"* paragraph cites D15-4; and in **§15**, whose
> ADR-0114 row now lists AC-532 and whose amendment note reads *"ADR-0114's row moves 2 → 3"*. The runbook claims
> it in `runbook/phase-1/s7b-kernel.md` **§7b.15** — *"AC-532 — CLAIMED HERE, added 2026-09-15"* — and re-points
> `ac110_executed_ledger_first_and_last` in **§7b.10**, *"The register routes this through AC-532"*, so
> the *Consequence* note at the foot of this section is discharged too. **The only thing this section still owes is
> residual R-3** — §4's scope note, named in D15-3 and deliberately not edited.
> **(i) Area.** This draft's `rk-kernel` tag decided nothing: the register removes that tie-break itself in its
> **§0 tag map**, whose row for *"the `rk-*` label after an id"* reads *"A **subject tag**, not a crate name"*.
> **`DECISIONS-2026-09-15 D15-3` allocates AC-532 in
> §4 packs-catalog**, beside AC-110 and AC-114 — ADR-0114's other criteria — because ADR-0114 decision 2 calls the
> field *the catalog record's* and the criterion follows its subject. §2 kernel was refused: it is an argument
> about where the code is, which that same tag-map row says is not how an area is chosen. **Residual R-3** rides
> with it — **§4's scope note**, *"Everything in the criteria set is configuration-time and spends no dispatch"*,
> is imprecise once a runtime-written projection lands
> in that area; it is named in D15-3 and **not edited**. 🛑 *That note is cited by its words and by no number, and
> the reason is on the record: D15-3 cited one line for it, a correcting pass in this file cited another, and both
> numbers had drifted before the correction was finished. The clause is §4's; the clause is what survives.*
> **(ii) THEN.** This draft's *"the catalog record's `executed.first`"* is **silent on both readings the build
> already took** and can be read as the shipped TOML — the reading `crates/rk-kernel/src/runtime/ledger.rs`
> refused under its module-header section *"THE LEDGER IS A PROJECTION OVER THE STORE, NEVER A FIELD IN THE SHIPPED
> TOML"*, because writing `executed` into the committed catalog TOML would trip **AC-111** on every run
> and a per-run `version` bump would invalidate every memo entry keyed on `activity_id@version`.
> **`DECISIONS-2026-09-15 D15-4` states both, as built:** the record is the **RESOLVED, IN-MEMORY** catalog record,
> never the shipped TOML; and `first`/`last` are **JOURNAL (APPEND) ORDER, not `at` order** — grounded in that same
> file's second module-header reading, *"ARE JOURNAL ORDER, NOT `at` ORDER"*, and in the discriminating case already
> committed in `crates/rk-kernel/tests/it/ledger.rs`'s `ac110_executed_ledger_first_and_last`, which commits
> `"run-late"` last and asserts it is `first` although that row's `at` is later.
> 🛑 **The amendment LANDED and it CARRIES BOTH CLAUSES.** Verified in the register's AC-532 body, whose THEN reads
> *"resolved, in-memory catalog record"* … *"never the shipped TOML"* and *"JOURNAL (APPEND) ORDER, not `at`
> order"*, so the committed discriminating test has a criterion behind it. *(This sentence closed "the amendment
> the operator lands **must** carry both clauses" — a future-tense obligation inside a block headed ✅ LANDED. The
> obligation was discharged before that block was written; the tense is corrected 2026-09-15.)* The block below is
> the draft's wording, kept as history.

> 🛑 *The `rk-kernel` tag on the next line is the **draft's**, and is not what landed.*
> *`DECISIONS-2026-09-15` **D15-3** allocated AC-532 in **§4 packs-catalog**. The register's tag map says the `rk-*`*
> *label is a **subject tag, not a crate name**, so the tag is not wrong in kind — but it must not be read as the*
> *area, which is what this draft used it for. The landed criterion carries `rk-catalog`.*
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

> 🛑 **Status 2026-09-15 — REFUTED as written: this amendment text CUTS NOTHING.** It is prose *about* a cut.
> Applied verbatim it leaves AC-023's *"unless it arrives as a typed budget-change event with an author"* clause
> and its second instrument **standing verbatim inside the criterion** that §0.3's C3 quotes as the contradiction —
> so C3 would be recorded resolved while the text that constitutes it is still there. The register's own precedent
> for a correction that cuts is **AC-204**, which **rewrote the THEN and the instrument in place** and
> *then* appended a dated Correction paragraph. **The landed amendment follows AC-204**: the `unless` clause is
> struck from AC-023's THEN in place, the second instrument is struck from its *Verified by* in place, and the
> dated run-in below is appended after them saying what was struck and why. A blockquote describing a cut that
> leaves the cut text standing is the defect, not the fix.
> **✅ LANDED 2026-09-15, in AC-204's form and not this draft's.** **Verified on this tree by quotation:** in
> `references/2026-09-08-acceptance-criteria.md`, **AC-023's THEN** now reads *"THEN the raise is refused."* with
> **no `unless` clause**; its *Verified by* carries **one** instrument, *"a unit test asserting refusal of an in-run
> raise from an activity result"*; and a dated **Correction (2026-09-15,
> `spikes/DECISIONS-2026-09-14.md` D-13 … the instrument form is `spikes/DECISIONS-2026-09-15.md` D15-2)** follows
> it, opening *"As authored the THEN read"* and saying what was struck and why. **§0.3's C3 row** carries the
> resolution line, *"AC-389 — refuse outright"*. **The prose
> below this block is the refuted draft, kept only so the refutation has something to point at — it is NOT what
> landed, and its blockquote still describes a cut instead of making one.**

Proposed amendment to **AC-023** — *the run-in only; see the status block above for what must be cut in place*:

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

> 🛑 **Status 2026-09-15 — two corrections, one of them a refutation.**
> **(i) *"No criterion states it"* is FALSE.** **AC-446 IS a criterion**, and its THEN reaches the recorded
> `--stay` bug id — the draft's own measurement names AC-446's corpus list as one of the four hits and then
> calls it not-on-the-subject. What **no criterion stated** is the **UNREPRESENTABILITY PROPERTY**: that the
> resumed/half-advanced state the `--stay` class depends on is unconstructible outside the module that owns it.
> That is the gap AC-294's fold-in closes, and it is the claim the amendment must make — not the wider one.
> **(ii) The withdrawal of exit criterion (5) is not a free edit.** Withdrawing it **renumbers** exit criteria (6)
> and (7), which are cited by number **four times across two documents**. *(Citation corrected 2026-09-15: this
> read "and **ADR-0110** (accepted) cites them **by number, twice**", which attributes both numerals to ADR-0110.)*
> **Verified on this tree:** ADR-0110 (accepted, and frozen by ADR-0092 decision 1 — which is why its line numbers
> are kept, as an accepted ADR's may be, while the design's below are not) cites *"Phase-1 exit criterion **(7)**"* twice —
> `references/adr/0110-headroom-is-a-batch-delta-in-ticks-on-visible-windows.md`:31 and :58 — and cites **(6)
> NOWHERE** (`grep -c '(6)'` over that file → **0**); **(6)**'s two citations are the **design's**, which is not
> frozen and is therefore cited by its words: `references/2026-09-08-architecture-design.md` carries *"§10.2 exit
> (6)"* in its blocked-row table, the row *"Selection engine — the context-vector source"*, and again in its
> drifted-figure row, *"AC-071's unblocking bar was written"*. An accepted ADR may not be edited (ADR-0092
> decision 1), so **(7)**'s two citations alone forbid the renumber and the design's two forbid it again — **the
> conclusion is unchanged and rests on either citation alone**: the withdrawal retires (5) **in place**, leaving
> the numbering intact, rather than removing the row. Named 2026-09-15; the wording is the operator's.

> **✅ LANDED 2026-09-15, both halves, with (i) and (ii) applied.** **Verified on this tree by quotation:** in
> `references/2026-09-08-acceptance-criteria.md`, **AC-294** carries
> *"Amendment (2026-09-15, `spikes/DECISIONS-2026-09-14.md` D-14)"*, which states the **unrepresentability
> property** as (i)
> requires and names **AC-446** as the criterion the draft below wrongly says does not exist; and Phase-1 exit
> criterion (5) is **withdrawn in place with its number retired** in
> `references/2026-09-08-architecture-design.md`'s §10.2 exit-criteria row, which now reads *"WITHDRAWN
> 2026-09-15"* against (5) and closes *"The sequence is (1) (2) (3) (4) (6) (7)."* The
> **separately owed `s8` by-path assertion also landed**: `runbook/phase-1/s8-exit-criterion.md` now names the case
> **by path**, carrying
> *"test -f crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs"* and
> *"test -s crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.stderr"* — the `test -f` on the `.rs` and
> the `test -s` on the committed `.stderr`. The draft
> prose below is what was corrected, not what landed.

**Why.** Phase-1 exit criterion (5) gates on the `--stay` compile-fail fixture and **no criterion states it** —
searching `--stay|unrepresentable` returns **four** hits and only **two** are on the subject — **AC-446**'s corpus list and **ADR-0107**'s flag denylist; the other two use *unrepresentable* of a fail-open path and of drift [measured 2026-09-14; cited by criterion and by record rather than by line, because the register has been amended since]. §7b.15
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

## C-4 · ADR-0122's **five** owed items

ADR-0122 decision 5 names these; none may be written by a build session.

> 🛑 **Status 2026-09-15 — the heading was wrong and an item was lost to a silent renumbering.** ADR-0122
> decision 5 names **FIVE** lettered items, verified 2026-09-15 by reading the record:
> **(a)** :218, **(b)** :222, **(c)** :226, **(d)** :231, **(e)** :249 — *(line numbers survive in this file only
> where they point into an **accepted ADR** — here ADR-0122, and elsewhere ADR-0001, ADR-0090 and ADR-0110.
> ADR-0092 decision 1 freezes an accepted ADR's body, so those are the one kind of number that cannot go stale
> under us. Every citation into the register, the design, the runbook or the crate is by quotation instead.)*. An earlier draft of this file carried only
> (a), (b) and ADR-0122's **(d)**, and relabelled that (d) as its own **(c)** — which is how ADR-0122's real **(c)**
> disappeared. The lettering below is now ADR-0122's own. ADR-0122's **(c)** is **still OWED** and is carried as
> **residual R-2** in `DECISIONS-2026-09-15`. ADR-0122's **(e)** needs no register edit: its stage obligation is
> **discharged** by `crates/rk-kernel/tests/it/budget.rs`'s `for_each_charges_once_independent_of_n`, which
> asserts the charge is *"constant across N = 1, 2, 50"*.
> **✅ (a), (b) and (d) LANDED 2026-09-15; (c) is STILL OWED as R-2; (e) owes no edit.** **Verified on this tree by
> quotation:** **(a)** in `references/2026-09-08-acceptance-criteria.md`, **AC-391**'s *Cut* is withdrawn with its
> clause restored — the THEN carries *"(**restored 2026-09-15**; ADR-0122 decision 1, via decision 5(a))"*, the cut
> paragraph is headed *"Cut — WITHDRAWN 2026-09-15"*, and a dated **Correction** follows it, opening
> *"Correction (2026-09-15, ADR-0122 decision 5(a)"*.
> **(b)** **§0.3's C1 row** carries the resolution line naming ADR-0122 and the correction to the
> *"ADR-0037 is superseded"* sentence — *"Correction (2026-09-15, ADR-0122 decision 4, via decision 5(b))"*.
> **(d)** **design §3.6's Budgets row** now carries *"clarified 2026-09-15 per **ADR-0122** decision
> 5(d)"*. **(c) has NOT landed and is not claimed to have** —
> the register says so itself, in AC-391's *"What this amendment does not touch"* paragraph, reporting that clause
> *"armed, not discharged"* and pointing at **R-2**.

**(a) Withdraw AC-391's *Cut*.** Its stated ground is refuted against its own cited source: proposal §4.3's Inner
cell reads *"**Observe, record, do not duplicate.** Inner attempts count toward the activity's budget"* and the
*Cut* quoted the bolded first sentence and stopped. The activity-graph schema's §4 (*"steps draw from it … **Inner
harness retries (`system/api_retry`) count.**"*) and design §3.6's **Inner retry** row (*"observe, record, do not
duplicate; inner attempts count against the activity's budget"*) say the same independently.

**(b) A resolution line on §0.3's C1 row**, in C4/C5's form — *and a correction to it*. The row says *"If C1
resolves against AC-391, ADR-0037 is superseded, never edited."* **It is not.** ADR-0037's entire `## Decision` is
one sentence — a load-time lint rule on a **declared** field — and nothing in it moves. ADR-0122 carries
`supersedes: []` and ADR-0037 keeps `superseded_by: []`. What overreached was four criteria **citing** ADR-0037 as
authority for a runtime rule it never states, and an interpretation cannot be superseded.

**(c) A dated clarification of AC-391's *surviving* class-scoped clause — STILL OWED (residual R-2).** ADR-0122
:226 owes a dated clarification of AC-391's surviving second clause — *"an error class the harness already
retried"* — against **AC-026**'s *"the same failure"*, so the two do not ship as a **class-scoped** and a
**dispatch-scoped** rule under one behaviour. ADR-0122 decision 2 takes the **dispatch** scope for the build; the
register owns the wording. **No text is drafted for it here**, because the scope the register should state is a
question no record settles, and `runbook/00-preamble.md`'s closing golden clause — *"Where a stage is tempted to
decide something else, it stops."* — stops a stage that is tempted to decide it.

**(d) A dated clarification to design §3.6's Budgets row.** That row states AC-021's rule *inside a cell about
runtime counters* — *"`attempts_total` counts the agent/tool steps of a **pass**, not the sum of retried steps
across passes"* — which is the row a later reader will use to attack ADR-0122 decision 3. The clarification should
say which quantity that sentence governs (the **declared** field, on ADR-0037's unit), so the **Budgets** row and
the **Inner retry** row beneath it cannot be read as one.

**(e) AC-022 under clause 1 — no register edit owed.** ADR-0122 :249 raises AC-022 (*"a `for_each` step projecting
over N items … the step counts once, independent of N"*) as put under pressure by clause 1, and reconciles it in
its own body: AC-022 governs the step's own charge, clause 1 governs the harness's attempts within each dispatch.
Its **stage** obligation is discharged in code — `crates/rk-kernel/tests/it/budget.rs`'s
`for_each_charges_once_independent_of_n` [verified 2026-09-15, by reading it]. Listed here for completeness so a
later reader does not count four items and go
looking for a fifth edit.

---

## C-5 · Two corrections — **no longer errata**

> 🛑 **Status 2026-09-15 — both instruments are WITHDRAWN, and the second finding is wider than the draft says.**
> **(i) Instrument.** `DECISIONS-2026-09-15 D15-2` rules that a **criterion** correction takes a **dated in-place
> register amendment, not an erratum**: an erratum is ADR-0092 decision 1's instrument for a **frozen** record, and
> a criterion in an amendable register is not frozen. `references/adr/ERRATA.md` concedes the route in its own words
> (*"the register is amendable in place, so this section is the lighter instrument and **not the only one
> available**"*) and §7b.11 precondition 2 asks for a register amendment and **names no erratum at all**. The rule
> was forced by an id collision: **`E-0071-1`** collides with the accepted **ADR-0071**, and the same collision was
> already owed a second time in `runbook/phase-1/s7b-kernel.md` **§7b.15** (**`E-0105-1`** against **AC-105**, and
> **ADR-0105 exists**).
> **`E-0071-1`, `E-0079-2` and `E-0105-1` are withdrawn as instruments, not as obligations** — each correction
> still lands, in place and dated. **Nothing is written to `references/adr/ERRATA.md` this pass and its convention
> header is unchanged.** `E-0079-2` was the wrong instrument on its own terms besides: ADR-0079's frontmatter
> already reads `status: superseded` and ADR-0090:108-110 says its body is deliberately untouched, so there is
> **no frozen line for an erratum to correct**.
> **(ii) The AC-484 finding applies EQUALLY to AC-005, which this draft never names.** **AC-005**'s *Verified by*
> asked for *"a `cargo-deny` `[bans].deny` entry for **every `rk-adapter-*` crate in `rk-kernel`'s
> graph** in the committed `deny.toml`"* [verified 2026-09-15, by reading it] — the same instrument, resting on the
> same list, failing for the same two reasons: the committed list names five workflow engines and **no adapter**,
> and `cargo deny` bans are **workspace-wide** while the property is a **per-crate edge**, so the ban cannot be
> written without forbidding `rk-adapter-sqlite` in `rk-cli`, which legitimately depends on adapters at 7d.
> AC-484 is where the draft found it; **AC-005 is where it originates**, and an in-place correction that touches
> only AC-484 leaves the same dead instrument standing one criterion over.
> **✅ (iii) BOTH CORRECTIONS LANDED 2026-09-15 — in place, dated, and neither as an erratum.** **Verified on this
> tree by quotation; every fragment below returns ≥ 1 under `grep -cF`.** In
> `references/2026-09-08-acceptance-criteria.md`: **§1.1**'s dead `--stability-record` path is corrected in place,
> *"Path corrected 2026-09-15"*; **AC-071** carries *"Amendment (2026-09-15) — the stability record's path and its
> producer"*; **AC-520** carries a dated note of the same date — *"Note added 2026-09-15"* … *"What still stands is AC-520's own half"*;
> **AC-484**'s third instrument is struck by a **Correction (2026-09-15, … D15-2)** opening *"A third instrument
> stood here"*; and **AC-005** — the criterion this draft never named — carries its own
> *"As authored this required"* correction, measured against `deny.toml`'s own `[bans].deny` list (`temporalio-sdk`,
> `temporalio-sdk-core`, `graph-flow`, `dbos`, `durable` — five entries and no adapter). The stale
> *"ADR-0079 is `proposed`"* text is corrected in place in the register (*"`status: superseded` since runbook Step
> 4.4 on 2026-09-12"*, and §15's ADR-0079 row, *"corrected in place 2026-09-15"*), in the design in three places
> (*"ADR-0079, status `superseded`"* beside *"corrected 2026-09-15"*), and in `runbook/00-preamble.md`'s golden
> constraint 1 (*"Nothing in the selection path over LLM-emitted facets starts before Step 4's gate clears."*).
> **`references/adr/ERRATA.md` is
> untouched**, as D15-2 requires. **Nothing in this section is still owed.**

**E-0071-1 — AC-071's dead `--stability-record` path** *(the obligation; the erratum id is withdrawn).* The criterion names
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

**Also still owed, pre-existing:** 🛑 **NEITHER HALF OF THIS SENTENCE SURVIVES — read it as history.**
**`E-0079-2` is WITHDRAWN as an instrument** by `DECISIONS-2026-09-15` **D15-2** (an erratum is ADR-0092 decision
1's instrument for a *frozen* record; ADR-0079's body is deliberately untouched under ADR-0090:108-110, so there is
no frozen line for an erratum to correct), **and the obligation it named has since LANDED** as dated in-place
corrections. **Verified on this tree by quotation:** the register (AC-071's THEN and §15's ADR-0079 row), the design
(its blocked-row table, its **4-J** row — *"No decision is taken on the LLM-emitted context-vector source."* — and
its *"LLM-emitted one this section renders"* note) and `runbook/00-preamble.md`'s **golden constraint 1** each now
record ADR-0079 as `superseded` **and** quote the `proposed` text they replaced, each carrying a dated
*"corrected 2026-09-15"* or *"corrected in place 2026-09-15"* beside it. What the draft sentence described was
real and was this: AC-071's THEN and `00-preamble.md`'s golden constraint 1 called ADR-0079 `proposed` where
ADR-0090 superseded it at Step 4.4. **Nothing is owed here, and nothing is written to `ERRATA.md`.**

---

## C-6 · A note saying what `schema_version` formally is (D-10)

> **Status 2026-09-15 — the note stands as drafted; one phrase must not be read as a defect report.**
> *"AC-014's **nine-case** test"* below is **correct and so is the codebase's**: the memo key has **EIGHT**
> components and its property test runs **NINE** cases. Nobody is being asked to change "nine-case test" to
> "eight-case test" — the two numbers count different things, and an edit reconciling them would introduce the
> error it looks like it is fixing.
> **✅ LANDED 2026-09-15.** **Verified on this tree by quotation:** `references/2026-09-08-acceptance-criteria.md`
> carries the dated note against **AC-014** — *"what component 8's `schema_version` formally is."* — and against
> **AC-019** — *"is **unchanged and the eight stands**"* —
> and each **re-measures** `grep -rn schema_version crates/rk-types/src crates/rk-ports/src`, carrying
> *"[re-measured 2026-09-15]"* rather than this draft's 2026-09-14 figure forward. The *"nine-case"* phrase below was
> left alone, as this block asks. **Nothing is owed here.**

Design §3.5 / AC-014 name *"the output type's `schema_version`"* as memo-key component 8. **No such field exists**
— `grep -rn schema_version crates/rk-types/src crates/rk-ports/src` returns **0** [measured 2026-09-14]. The only
value carrying a version is `SchemaId(String)`, in a `Kind/vN` convention. D-10 takes the **whole id** as
component 8. A dated note should say so, and say why the alternative was refused: parsing the `vN` suffix invents
a convention no record states, has no defined behaviour on an id without one, and makes `RecordedResult/v1` and
`ChangeSummary/v1` **collide** in component 8 — weakening the property AC-014's nine-case test exists to pin.

---

## C-7 · A note fixing the exhaustion comparison per counter (D-6)

> **Status 2026-09-15 — ✅ LANDED; NO LONGER OWED.** *(This block read "unchanged by today's decisions, and still
> OWED", which was written on the same stale premise as this file's header and is corrected here.)* No `D15`
> decision touches the reading itself, and the dated note against **AC-024 and AC-025** landed as an **in-place
> register amendment** (`DECISIONS-2026-09-15 D15-2`), not an erratum. **Verified on this tree by quotation:**
> `references/2026-09-08-acceptance-criteria.md` carries the dated *"Note (2026-09-15,
> `spikes/DECISIONS-2026-09-14.md` D-6) — against AC-024 and AC-025 both"*, which **re-measures** the defect
> against `spikes/p9/catalog/` — *"exactly 9 of 102"* — rather than carrying this draft's
> 2026-09-14 figure forward, and which adds *"What does and does not exercise this comparison at Step 7b"*, code
> lines only.
> The reading also still lives in `rk-kernel::workflow::budget`'s header, which is where D-6 put it.

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
