# Decisions taken while landing the owed record work, 2026-09-15 (D15-1…D15-4)

Same shape as `DECISIONS-2026-09-14.md`. Every entry was taken **by the operator** (Ryan Britton), one at a time,
with the grounding and the enumerated options in front of him, **before** the record that depends on it was
written. Grounding is a document clause or a measured number, never taste.

**Numbering.** These carry **`D15`-ids**: a different pass with its own id space, distinct from the `#1`–`#27` of
`DECISIONS-2026-09-11.md`, the `E`-ids of `DECISIONS-2026-09-12.md` and the `D`-ids of `DECISIONS-2026-09-14.md`.
Cite them as `DECISIONS-2026-09-15 D15-<n>`.

**Where they came from.** The session that landed the five record items `DECISIONS-2026-09-14.md` left owed
(D-1, D-3, D-12, D-13, D-14) verified each draft against its primary record before authoring, and **four free
residuals surfaced that none of the five drafts had named**. Each blocked an edit that would otherwise have been
written on a default. `00-preamble.md`:55 — *"Where a stage is tempted to decide something else, it stops."*

**What the verification cost, stated once.** Fourteen agents read the primary records rather than the drafts.
Three of the drafts' load-bearing claims were **refuted**, and each would have landed a defect:

- Draft **A**'s `corpus_sha` value (`27d97e9b…`) is not transcribed from F36 at all — F36's own engine rows carry
  `47faec9f…` — and the record is **never regenerated**, so the wrong value would have been permanent. → **D15-1**.
- Draft **C-2**'s AC-023 amendment is prose *about* a cut and **cuts nothing**; applied as written it leaves the
  `unless` clause and its second instrument standing verbatim inside the criterion C3 quotes as the contradiction.
- Draft **C-3**'s *"no criterion states it"* is false — **AC-446**'s THEN reaches the recorded `--stay` bug id —
  and the withdrawal it proposes would have **renumbered** exit criteria (6) and (7). **ADR-0110** (accepted) cites
  *"Phase-1 exit criterion **(7)**"* by number twice, at :31 and :58; the **design** cites *"§10.2 exit **(6)**"* at
  :19 and :2000. Renumbering would therefore force a supersession of an accepted ADR to fix two citations, so **(5)
  is retired in place and nothing renumbers**. *(Corrected 2026-09-15: this sentence read "exit criteria (6) and
  (7), which **ADR-0110** (accepted) cites by number twice", which attributes both numerals to ADR-0110. ADR-0110
  cites only (7); (6)'s two citations are the design's. The conclusion is unchanged and rests on either citation
  alone.)*

---

## D15-1 · What `corpus_sha` denotes in a **transcribed** record

**The question.** AC-071 clause 5 asserts `rec.corpus_sha == <catalog corpus pin>`. AC-071's stability record is a
**transcription of F36** that `s7b` §7b.2 forbids regenerating. Three values were live and all three different.

**The choice: `47faec9f46b8d983c2c75f0794164e68415ae706` — the pin the measurement was taken at.**

**Grounding.** It is the value F36's own engine rows carry (`$CORPUS_REPO/results/p1-arm-l.jsonl`:
`"corpus_sha":"47faec9f46b8d983c2c75f0794164e68415ae706"`), the pin `F36-DRAFT.md`:3 names, and the value already
hard-coded in this tree at `crates/rk-kernel/tests/it/facet_stability.rs`:37 —
`const CATALOG_CORPUS_PIN: &str = "47faec9f46b8d983c2c75f0794164e68415ae706";` — whose own doc at :33-36 says in
terms *"which of the two shas AC-071's clause means is part of the amendment that criterion is OWED … Raised, not
settled, and the fixtures are authored against the pin the measurement names."* The two refused options each fail
on the record: `27d97e9b…` (`CORPUS_PIN`:5) is the **current** pin, which §7b.11's own move changes, so a record
carrying it fails clause 5 at the moment the act that commits it completes; and the post-move sha does not exist
when the transcription is made, which makes clause 5 self-referentially unsatisfiable for this record.

**What it costs, stated rather than buried.** Clause 5 **does not clear** under this reading, because
`47faec9f…` is not today's catalog pin. That is **fail-closed and correct**: AC-071 already fails on clause 1
(identity **0.10** against **0.80**), the gate is unchanged, and a record that made clause 5 pass while clause 1
failed would be the *"a lint that reads identity and skips the panel clause"* failure one level over.

**What it owes.** One clause in AC-071's dated amendment saying what `corpus_sha` denotes for a transcribed
record — **the pin the measurement was taken at, not the catalog pin at read time**.

---

## D15-2 · Criterion corrections take a dated in-place register amendment, not an erratum

**The question.** The owed erratum id **`E-0071-1`** collides with **ADR-0071**
(`0071-empty-evidence-requires-no-agent-step.md`, `status: accepted`, no errata section). `ERRATA.md`'s only
criterion-keyed precedent — `## AC-520` carrying `E-0520-1` (:271, :284) — could take that id **only because no
ADR-0520 exists or can exist**. The same collision is already owed a second time: `s7b-kernel.md`:1887 owes
**`E-0105-1`** against **AC-105**, and **ADR-0105 exists**. Nothing in ADR-0092, in `ERRATA.md`, in the register or
in the runbook says what happens when a criterion's number is already an ADR's number. It had to be answered as a
**rule**, not case by case.

**The choice: no erratum. The correction lands as a dated in-place amendment to the AC register.**

**Grounding.** `ERRATA.md`:273-276 concedes the route in its own words — *"the register is amendable in place, so
this section is the lighter instrument and **not the only one available**"* — and `s7b` §7b.11 **precondition 2
asks for exactly that and names no erratum at all**: *"its dated register amendment has landed naming its path and
its producer."* An erratum is ADR-0092 decision 1's instrument for a **frozen** record; a criterion in an amendable
register is not frozen, so the erratum was the heavier instrument for a correction the lighter one already covers.

**What it retires, by name.** **`E-0071-1`** (AC-071's dead `--stability-record` path), **`E-0079-2`** (the stale
*"ADR-0079 is `proposed`"* text) and **`E-0105-1`** are withdrawn **as instruments**, not as obligations: each
correction still lands, in place and dated. `E-0079-2` was the wrong instrument on its own terms — ADR-0079's
frontmatter already reads `status: superseded` and ADR-0090:108-110 says its body is deliberately untouched, so
**there is no frozen line for an erratum to correct**; the stale text lives in the register, the SM register, the
design and the runbook, each of which is amendable in place. The correction owed against **AC-484**'s third
instrument takes the same form.

**What it does NOT do.** It does not touch `E-0520-1` or any existing erratum, and it does not forbid a future
criterion erratum — it says the *default* instrument for a criterion correction is the register itself.
**`ERRATA.md`'s convention header is unchanged**; this decision is recorded here and cited from each amendment.

---

## D15-3 · AC-532 is allocated in **§4 packs-catalog**

**The question.** D-12 allocates AC-532 for ADR-0114 decision 2's executed-once ledger. **Which area section.**
The register removes the obvious tie-break itself at :19 — the `rk-*` label is *"a **subject tag**, not a crate
name"* — so draft C-1's `rk-kernel` decides nothing, and both candidate ranges are exhausted so §0.4 takes a row
either way.

**The choice: §4 packs-catalog**, beside AC-110 and AC-114 — ADR-0114's other criteria.

**Grounding.** ADR-0114 decision 2 calls the field **the catalog record's**: *"The catalog record gains
`executed: { first, last }`"*. The criterion follows its subject. The refused option (§2 kernel) is where the
writer, the test and Checkpoint 7b's claimed range live, which is an argument about **where the code is**, and
:19 is the clause that says that is not how an area is chosen.

**The tension, recorded rather than smoothed.** §4's own scope note (:1675) reads *"Everything in the criteria
set is configuration-time and spends no dispatch"*, and a ledger written at runtime on `activity_committed` is
not configuration-time. **The scope note is the thing that is now imprecise, not the allocation** — it was
written before ADR-0114 gave a catalog record a runtime-written projection. It is **named here and not edited**;
a dated amendment to §4's scope note is **OWED** and is the only residual this decision leaves.

---

## D15-4 · AC-532's THEN states both readings the build already took

**The question.** `crates/rk-kernel/src/runtime/ledger.rs` carries **two** readings with *"a record is OWED"*
written beside each. Minting AC-532 silent on either leaves a discriminating test with no criterion behind it.

**The choice: the THEN states both, as built.**

**(i) The RESOLVED, IN-MEMORY catalog record — never the shipped TOML.** Grounding is `ledger.rs`:27-34, which
refused the TOML reading on the record: writing `executed` into the committed catalog TOML *"would trip **AC-111**
— a shipped-activity content-digest change with no `version` bump — on **every run**, and a per-run `version` bump
would invalidate **every** memo entry keyed on `activity_id@version` (ADR-0097 component 1)."* Draft C-1's
*"the catalog record's `executed.first`"* is silent and can be read as the TOML — the reading the build refused.

**(ii) `first` and `last` are JOURNAL (APPEND) ORDER, not `at` order.** Grounding is `ledger.rs`:47-57 and the
**discriminating case already committed** at `tests/it/ledger.rs`:145-157, which asserts `first.run_id ==
"run-late"` **although that row's `at` is later**. ADR-0114 says *"on the first and every subsequent
`activity_committed`"* and nothing about timestamps, so the ADR does not forbid this reading — but it does not
state it either, which is why it needed a decision and not a default.

**What the alternative would have cost, stated because it was a live option.** Choosing `at` order would make the
shipped code and its discriminating test **wrong** and red. That is a legitimate outcome and was offered as one;
it was refused because no clause favours `at` order and the append reading is the one the store's own append-only
shape produces.

---

## A fifth thing this pass settles, which is a correction and not a decision

**`DECISIONS-2026-09-14` D-9's closing clause is refuted by measurement.** Its landed column reads *"The separate
note that `HARNESS_ALREADY_RETRIED` is this crate's proposed class token **stands** and is still owed."* The build
session that was asked to update `crates/rk-kernel/src/workflow/retry.rs`:580-583 accordingly **refused and
reported**, correctly: it will not contradict a committed record on an instruction's say-so. So the refutation is
recorded here instead, and the code follows the record rather than the other way round.

**What is measured** [2026-09-15, code lines only, comment lines filtered]: `HARNESS_ALREADY_RETRIED` is a
`&'static str` on the kernel-internal `RetryDecision::Refuse`. It is **never serialised, never journaled, never
crosses a port**, and its only producer `decide_after_inner_retries` has **zero callers** under any `crates/*/src/`.

**Why the analogy that made it look owed does not transfer.** `retry.rs`:580-583 reasons from
`crates/rk-types/src/event_kind.rs`:16-21 — *"Three names are owed to design §5 … until it lands those three are
this crate's proposal rather than the design's word."* That holds for `EventKind` because `EventKind` **is** design
§5's own closed, serialised table. `RefusalClass` is a different animal: `crates/rk-types/src/event.rs`:24-29 is
closed at **four** and is one-to-one with `StepError`'s four refusal variants, and ADR-0112 forbids a twelfth
`StepError` variant without a successor record. `HARNESS_ALREADY_RETRIED` is **not a `RefusalClass` token and
cannot become one** — so there is no vocabulary above this crate for it to be owed to.

**The consequence:** nothing is owed above the runbook for D-9, the runbook may carry the token, and `retry.rs`'s
*"A record naming the refusal-class vocabulary is OWED"* is withdrawn as an overclaim. D-9's own decision — that
`JitterDraw::from_key` is **FORCED, sanctioned, no record owed for the source** — is untouched and stands.

---

## What these four decisions unblock

D15-1 unblocks **D-3** (AC-071's amendment and the stability-record draft). D15-2 unblocks **every** correction
that was waiting on an erratum id — D-3's dead path, the AC-484 instrument and the pre-existing ADR-0079
staleness. D15-3 and D15-4 unblock **D-12** (AC-532's allocation and its THEN).

**Still owed after this pass, and named so nobody rediscovers them** — each is a residual these decisions
surfaced and did not settle:

| # | what is owed | raised by |
| --- | --- | --- |
| R-1 | **AC-389's second THEN half — *"the attempt is appended as an event"* — has no event kind to be appended as.** `EventKind` is closed at 29, ADR-0112's eleven `StepError` variants carry no member for a refused appetite raise, and design §5's table has no row. The crate says so at `tests/it/budget.rs`:391-392. Needs an ADR in ADR-0116's / ADR-0120's form, or a dated amendment withdrawing the append clause | D-13's resolution |
| R-2 | **ADR-0122 decision 5(c)** — a dated clarification of AC-391's **surviving** class-scoped clause (*"an error class the harness already retried"*) against AC-026's *"the same failure"*. Decision 5 carries **five** lettered items; draft C-4 and the D-1 row both carry three and drop this one. (5(e) needs no edit: its stage obligation is discharged at `tests/it/budget.rs`:319-372) | D-1's verification |
| R-3 | **§4's scope note (:1675)** — *"configuration-time and spends no dispatch"* — is imprecise once AC-532 lands in that area | D15-3 |
| R-4 | **AC-294's third instrument — PARTLY discharged 2026-09-15, in the same session that raised it, and STILL OPEN on its other half.** When raised, the tree had the private constructor and the `trybuild` case but no module-boundary lint. It was then **built test-first** under golden constraint 12: `crates/rk-kernel/src/module_boundary.rs` (the rule) and `crates/rk-kernel/tests/it/module_boundary.rs` (`ac294_run_state_construction_sites_are_module_bounded`), wired into `lib.rs` and `tests/it/main.rs`, green in the suite. **It scans code lines only and proves it rather than claiming it**: one part is a comment-only control carrying every banned spelling in a module doc, an item doc, a line comment *and* a block comment, which must produce **zero** violations; another asserts the naive comment-blind scan the rule replaces reports **more** hits over the same bytes, so the control cannot pass by the stripper doing nothing; and premise assertions fail if the fixture is vacuous. That closes, on its fourth occurrence, the failure this codebase has already paid for three times — **a guard satisfied by the comment explaining it**. 🛑 **What is NOT discharged:** the lint covers the `RunState` / `--stay` half only. AC-294's **block-value** half names construction sites outside `rk-kernel::policy`, and **this workspace has no `policy` module and no block-value type**, so that half is **armed, not discharged** and R-4 stays open on it. AC-294's own dated amendment in the register says the same and claims neither half as discharged — *(corrected 2026-09-15: this row first read "**DISCHARGED**" flatly, which overstated it against the register's own more careful wording)* | D-14's verification |
| R-5 | **A record routing `ActivitySpec` assembly.** D-11 settled the *build scope* (7c); draft B §4 records that a **record** routing spec assembly is still owed | draft B §4 |
| R-6 | **`references/adr/README.md` is stale against ADR-0122 in five places and indexes no row for it** (:3, :5, :73, :269, :281). Pre-existing; an index amendable in place, not an erratum | D-1's verification |

---

## A note on method, because it is the reason three defects were caught

Every claim in the three drafts was re-read against the file that contains the clause, and every proposed anchor
was independently re-read with `sed -n 'NNNp'` and counted with `grep -cF` before any edit was written. That pass
found **the drafts' own citations unreliable at eleven line numbers** while their substance held, and it found the
three refutations listed at the head of this file.

### 🛑 The finding this session paid for three times, and the rule that follows from it

**An inline `[measured: <command> → N]` line citation into a file that is still being edited is self-falsifying,
and writing one is a defect even when the command is right and the number was true when it ran.**

Measured, three times over, in this session's own passes. A pass corrected a batch of stale line cites and, in the
correction note, wrote fresh line numbers that were already wrong by the time it finished — because sibling agents
were still writing the cited files. One pass wrote *"`grep -c '2026-09-15' <register>` returns **0** — no register
amendment has landed"* as a bolded 🛑 standing fact; the register carried **61** occurrences within the hour,
because the amendments landed in parallel while the measurement was being typed. A later pass corrected that to
**61**; re-measured minutes afterwards it was **69**. The same file's `:1545`, cited as a stable anchor, moved
three times in one afternoon.

This is the house's own two atoms in a new dress — *a probe in another language is a proxy* and *verify the
operation, not a proxy for it* — with a third edge the earlier records did not carry: **a correct command run at
the wrong moment produces the same shape of output as a correct one, and a number is the one kind of evidence
that carries no timestamp inside itself.** The `grep` was right. The file was right. The reading was right. Only
*when* was wrong, and nothing in the recorded form shows it.

**The rule, for every record in this repository from here:** cite a **section, a symbol, or a quoted sentence** —
never a bare line number — for anything in a file that is not frozen. A quoted string is checkable with
`grep -cF` at any later moment and gives the same answer; a line number is checkable only against the tree that
existed when it was written. The AC register already reaches for this habit in places and it should be the
default. Where a line number is genuinely wanted, it is a **convenience beside the quotation**, and the quotation
is the citation that survives — which is exactly what `ERRATA.md`'s own `## AC-520` section says in terms:
*"The clause each number names is quoted beside it and is the citation that survives; the number is a convenience
and may go stale again."* That sentence was already in this repository, and this session rediscovered it the
expensive way.

It also settled a measurement this tree gets wrong in both directions. **`grep -c $'\r'` cannot answer the
line-ending question at all** — Git Bash strips the CR on read, so it returns **0 for a CRLF file and 0 for an LF
file alike**, and two agents in this session reported the tree's endings confidently and in *opposite* directions
off that one instrument. Count the bytes instead:
`total=$(wc -c <f); nocr=$(tr -d '\r' <f | wc -c); echo $((total-nocr))`.

**What the bytes say, and it is not a tidy rule:** every `.md` record file is **LF** — the AC register, the
design, `ERRATA.md` and every runbook file carry **0** CR bytes. The Rust tree is **MIXED**:
`crates/rk-kernel/src/workflow/budget.rs` is CRLF, and every other `.rs` file measured — `lib.rs`, `retry.rs`,
`ledger.rs`, `envelope.rs`, `module_boundary.rs`, `main.rs` and the new fixtures — is **LF**.
`core.autocrlf=true` with no `.gitattributes` means git normalises on write, so **a whole-file ending flip is
invisible to `git diff`**; one agent silently converted `runbook/PROGRESS.md` LF → CRLF in its entirety this
session and nothing in the diff showed it. **So there is no rule to carry — measure the specific file, every
time, before writing a multi-line anchor into it.**
*(Corrected 2026-09-15: this paragraph first read "the record files are LF and the **Rust sources are CRLF**",
generalising from `budget.rs` alone. Re-measured across every `.rs` file this session touched: `budget.rs` is the
exception, not the rule. The correction is the same defect this section is about — a true measurement of one
thing, written down as a fact about a class.)*
