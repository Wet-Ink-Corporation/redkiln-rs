---
id: "0129"
title: The gate-answer grammar is `redkiln answer <item> <option> [--attach <kind>=<path>] --explain-sufficed <yes|no> --toml-opened <yes|no>` — the token resolved by the CLI from the item's open gate record, the option validated against an ADR-0063 question registry whose type lands in `rk-kernel` with the default question `approve` / `changes-requested` built in — so ADR-0009's literal `approve|changes-requested` grammar sentence is superseded in part and its CLI-only channel clause stands
status: accepted
date: 2026-09-18
legacy_id: none
source: references/adr/0009 — its Decision sentence (*"the only channel through which a human gate may be answered is `redkiln answer <item> approve|changes-requested`"*) — one sentence carrying two decisions, the channel (kept) and the literal grammar (moved); references/adr/0063 — its Decision (*"A gate's output is `answer` plus `attachments: [ArtifactRef]`. A question option may declare `attaches = <Kind>`; the kernel refuses to close the mark unless that attachment is supplied. The attachment is stored as a human-supplied root"*); references/adr/0017 (the answer binds `(token, tree_hash, deadline)`; an item-keyed answer is rejected); references/adr/0092 (*an erratum may not touch a Decision sentence*); references/2026-09-08-acceptance-criteria.md — AC-248 (Verified-by: *"a CLI test that `redkiln answer <item> approve` without a resolvable live token exits non-zero with a named outcome"* — no token argument), AC-405 (THEN: *"the gate-answer store method has exactly one call site, in `rk-cli`'s `answer` command"*), AC-406 (WHEN: *"`redkiln answer` is invoked with a valid token but the tree hash no longer matches"*), AC-407 (one `gate_answered` per token), AC-410 (GIVEN: *"a gate question whose registry entry declares its options and each option's admissible attachment types"*; WHEN: *"`redkiln answer` is passed an attachment that no declared option admits"*; THEN: *"refused at the CLI boundary and nothing is written"*), AC-435 (THEN: *"the answer (yes/no, plus whether a TOML was opened) is recorded as a required field on the gate answer"*; Verified-by: *"the gate answer cannot be submitted with the field absent"*), **cited by id and named clause** (DECISIONS-2026-09-15 D15-2); references/2026-09-08-architecture-design.md §8.1 (the verb table's `answer <item> approve\|changes-requested` row) and §5 (the `gate_answered` payload row); crates/rk-kernel/src/workflow/gate.rs at `239e899` (`answer(state, gate, presented, tree_hash, verdict: &str)` — an untyped verdict, no attachment parameter, no registry) and crates/rk-kernel/src/catalog.rs (no question/option/attaches type); the C-01 adjudication of 2026-09-18 (item D-04, BLOCKING — *"AC-410's three instruments are all satisfied by a refuse-all validator with no positive control"*; the token half refuted by AC-248), recorded in runbook/PROGRESS.md §6; the operator's decision of 2026-09-18, taken before Step 7d's stage-open commit with the alternatives named (a two-literal `clap` enum with no attachment path; the registry-validated option with an attachment flag — the second was taken; the registry's stage — 7d for the type and the default question, 7e for pack-declared questions), recorded as `spikes/DECISIONS-2026-09-18.md` D18-17
supersedes: ["0009"]
superseded_by: []
related: ["0017", "0024", "0063", "0092", "0107", "0128"]
evidence: ["F33", "PIA-059"]
consumers: []
---

## Context and problem

**ADR-0009's one Decision sentence carries two decisions.** *"In Phase 1, the only channel through which a human
gate may be answered is `redkiln answer <item> approve|changes-requested`."* The first is the **channel** — CLI only;
tracker comment grammar and web view wait for Phase 3 — and it is the decision the record argues, the one its
drivers, options and forbids are about. The second is the **grammar** — two literal verdicts and nothing else —
which the record never argues and which its own closing clause disclaims (*"this is a phasing decision about which
channels exist yet, not about the gate's internal shape"*).

Taken as grammar, the literal is jointly unsatisfiable with three later records. ADR-0063's Decision makes a
question option a **registry-declared id** that *"may declare `attaches = <Kind>`"*, with the kernel refusing to
close the mark without the attachment. AC-410 has `redkiln answer` *"passed an attachment"* and refused *"at the
CLI boundary"* when no declared option admits it. AC-435's THEN records the explain-sufficiency answer *"as a
required field on the gate answer"* that *"cannot be submitted with the field absent"*. A two-literal `clap` enum
with no attachment argument and no required field can exercise none of AC-410's WHEN — its three instruments pass
on a validator that refuses everything (the PIA-059 shape) — and cannot carry AC-435's field at all.

**What is on the tree.** `rk_kernel::workflow::gate::answer` takes `verdict: &str`, untyped, and no attachment;
no question, option or `attaches` registry exists in `rk-kernel` or `rk-types` [measured 2026-09-18, `239e899`].
The token half of the C-01 finding was **refuted**: AC-248's CLI test is `redkiln answer <item> approve` with no
token argument, so the CLI resolves the live token from the item's open gate record and no `--token` flag exists in
any record.

**Why this is a record and not an erratum.** The literal sits in a `## Decision` sentence; ADR-0092 forbids an
erratum that touches one and forbids the runbook deciding it. The route is a supersession **in part**: the
grammar sentence moves, the channel clause stands verbatim.

## Decision drivers

- ADR-0063 is accepted and later; a grammar that cannot carry its `attaches` is a grammar that silently repeals it.
- AC-410's instruments must be able to fail. A refuse-all validator passes a unit test, a red fixture and a
  zero-events assertion; only a **positive control** — an admitted attachment accepted and stored — makes the
  refusal discriminating.
- AC-435's machine half is decided by the register THEN and has no carrier on a two-literal grammar.
- AC-248 already decides the token: resolved by the CLI, never typed by the human.
- ADR-0009's channel decision is right and untouched: every answer still lands in the same `reconcile()` from the
  CLI and from nowhere else in Phase 1.

## Considered options

### Option — a `clap` value enum of two literals, no attachment argument, no extra field
ADR-0009's sentence read as grammar. Rejected: AC-410's WHEN is unreachable and its instruments pass vacuously;
AC-435's required field has no carrier; ADR-0063's `attaches` is dead on arrival; a pack-declared question with a
third option (`proceed`) can never be answered.

### Option — the option validated against a registry, an attachment flag, the sufficiency field **(taken)**
`answer <item> <option> [--attach <kind>=<path>] --explain-sufficed <yes|no> --toml-opened <yes|no>`. The option is a registry id; the
default question's two options are `approve` and `changes-requested`, so every invocation ADR-0009's sentence
admitted is still admitted; the attachment path is exercisable in both directions; the field is required.

### Option — the same grammar with the registry deferred wholesale to 7e
Rejected: without a registry type at 7d the option is again an untyped string and the attachment validator has
nothing to validate against; AC-410's positive control cannot be written. The **type** and the default question
are 7d's; the pack-declared questions are 7e's.

## Decision

**1. The `answer` grammar is `redkiln answer <item> <option> [--attach <kind>=<path>] --explain-sufficed <yes|no> --toml-opened <yes|no>`.**
`<item>` and `<option>` are positional. `--attach` is optional and repeatable, `<kind>` an artifact kind name and
`<path>` a file in the working tree. `--explain-sufficed` and `--toml-opened` are both **required** — AC-435's machine
half: *"the gate answer cannot be submitted with the field absent"* — and each takes `yes` or `no`. They are AC-435's
**two data in one field**: the THEN records *"the answer (yes/no, plus whether a TOML was opened)"* as *a required
field*, and SM-126's pre-registered kill — *"more than 15 of the first 30 answers report that the reviewer opened a
TOML anyway"* — is the cell *sufficed = yes AND opened = yes*, which one boolean cannot carry. The `gate_answered`
payload gains **one** member, `explain_sufficiency { sufficed, toml_opened }` (design §5's row is amended toward the
register). The flags' **names** are the defaults the implementer may rename at Step 7d; the stage file names the ones
shipped, and the register's AC-435 Verified-by is amended toward those names. *(Corrected 2026-09-19 before acceptance:
as proposed this decision named one flag and read the second datum as its `no` arm — the operator's answer to the
question the stage-open commit carried is two data, one field.)*

**2. The token is resolved by the CLI, never typed.** `answer` reads the item's open gate record from the store,
takes its live token, and hands `(token, tree_hash, option)` to `rk_kernel::workflow::gate::answer` (AC-405's one
call site). There is no `--token` flag. An item with no resolvable live token exits non-zero with a named outcome
(AC-248); a spent token is refused with exactly one `gate_answered` on record (AC-407); a tree-hash mismatch is
recorded void and the gate stays open (AC-406). None of ADR-0017 moves.

**3. Options are validated against an ADR-0063 question registry.** A `pub` registry type lands in **`rk-kernel`
at Step 7d** — a question id, its option ids, and per option the artifact kinds it `attaches` — with the **default
question built in**: options `approve` and `changes-requested`, neither attaching anything. A gate step that
names no question is the default question. **Pack-declared questions** (a question in a process pack's TOML with
its own options and `attaches`) are Step 7e's, loaded into the same type. An option the gate's question does not
declare is refused at the CLI boundary as a usage error with zero events.

**4. Attachments in both directions.** An attachment whose `<kind>` no declared option of the gate's question
admits is **refused at the CLI boundary and nothing is written** (AC-410's THEN, the negative). An attachment an
option admits is **accepted and stored as a human-supplied root** — ADR-0063's *"stored as a human-supplied root
under PC3's `derived_from = []` model"*, readable downstream as `${<gate>.attachments.<Kind>}` — and this is
AC-410's **positive control**, added to its Verified-by by dated amendment so a refuse-all validator fails it. An
option that declares `attaches` and is answered without the attachment is refused (ADR-0063's *"refuses to close
the mark"*) — **by the kernel**, inside `gate::answer`, which is handed the registry and the attachments and calls the
same `validate` before it closes the mark (ADR-0063's sentence stands unmoved); the CLI performs the same validation
first, so the operator sees a usage error with zero events, and a caller that skips the CLI check cannot reach
`Applied`.

**5. What this record supersedes, exactly.** ADR-0009's Decision sentence **as a grammar** — the literal
`approve|changes-requested` read as the closed set of what `answer` accepts — and nothing else: the channel clause
(*"CLI only; tracker comment grammar and web view in Phase 3"*), the *"all channels write into the same
`reconcile()` path"* clause, every bullet of its forbids and its Phase-3 deferral stand verbatim; 0009 stays
`accepted` carrying `superseded_by: ["0129"]`.

**6. The records amended toward it** (ADR-0092 clause 2.2), each a dated in-place amendment, none an erratum:
AC-410's Verified-by (the positive control); AC-405's and AC-406's Verified-by (the flags named); AC-435's
Verified-by (the flag names the stage ships and the field); design §8.1's `answer` row (`<option> [--attach …] --explain-sufficed --toml-opened`)
and design §5's `gate_answered` payload row (the field; and either a source for the `answerer` it names or the
field dropped — 7d's stage file states which).

**What this forbids**
- Forbids any second answer channel in Phase 1 — unchanged from ADR-0009: no tracker comment grammar, no web-view
  write, no file drop is a gate answer; only a CLI `redkiln answer` invocation reconciles.
- Forbids a `--token` flag, an item-keyed answer, or any grammar in which the human types the token.
- Forbids an option the gate's question does not declare, and forbids the CLI accepting one on a question that is
  not in the registry.
- Forbids an attachment path that is refuse-only: the registry must be able to admit at least the default
  question's declared kinds, and the positive control must exist as a test.
- Forbids submitting an answer with the sufficiency field absent, and forbids a default value for it.
- Forbids free-text answer fields (PC8 rule 5, ADR-0063 — unchanged).

## Consequences

**Positive**
- ADR-0009's channel decision, ADR-0063's attachments and ADR-0017's token binding compose into one grammar
  instead of contradicting each other.
- AC-410 gains a positive control and its three instruments become discriminating; AC-435's machine half has a
  carrier; AC-248's *no token argument* is honoured by construction.
- The default question keeps every invocation ADR-0009 ever admitted valid.

**Negative**
- A supersession-in-part of a `D9`-era record; the register's AC-405/406/410/435 and design §5/§8.1 each take a
  dated amendment in the stage-open commit that carries this record.
- A `pub` type enters `rk-kernel` at 7d (a Step-7b crate edit; its checkpoint lines re-run), and the registry has
  exactly one question until 7e — the pack-declared arm is exercised by 7d fixtures only.
- `--explain-sufficed` and `--toml-opened` on every answer are a cost to the operator on every gate; SM-126 is the
  measure that says whether it was worth asking, and it can only be read because both are asked.
- Committed `proposed`: the operator's word on this shape is recorded in `DECISIONS-2026-09-18` D18-17; the flip to
  `accepted` is a follow-up commit once that line is dated (ADR-0126's precedent), `date` unchanged.

## Evidence and confidence

- [documented] ADR-0009's Decision sentence and its own *"not about the gate's internal shape"*; ADR-0063's
  Decision; AC-248, AC-405, AC-406, AC-407, AC-410, AC-435 by clause; design §5 and §8.1 rows.
- [measured 2026-09-18, `239e899`] `gate::answer(…, verdict: &str)` untyped, no attachment parameter; no
  question/option/`attaches` type anywhere under `crates/`.
- [measured, F33] a lint that accepted five guesses silently — cited by AC-410 as the failure shape a refuse-all
  validator reproduces from the other side (it refuses everything and is never observed accepting the one thing it
  should).
- [documented, PIA-059] an instrument satisfied by an implementation that does nothing — the shape AC-410's
  positive control exists to close.
- **Confidence:** the joint unsatisfiability is a reading of quoted clauses and is not in doubt; the grammar and the
  registry's stage split are the operator's choice among three named options. The flags' final names and the
  `answerer` source are **not decided here** and are named as Step 7d's to state.
- **What this evidence does not license.** It does not license claiming that `--explain-sufficed` / `--toml-opened` measure
  legibility — AC-435's Verified-by keeps that *"routed to a human gate; not machine-decidable"*; this record carries
  the field, not the judgement.
