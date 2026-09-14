---
id: "0121"
title: Kleene absorption is commutative in both operand positions — `null and false` is `false` and `null or true` is `true`, and ADR-0103 decision 4's connective sentence is superseded in part on measured pack evidence
status: accepted
date: 2026-09-13
legacy_id: none
source: references/adr/0103-predicate-grammar-precedence-literals-null-and-on-false-forms.md decision 4 (:69-76, the connective sentence at :70-71) and its `**What this forbids**` block (:112-118, which carries no connective bullet); references/adr/0092-errata-precedence-and-cut-id-reissue.md decision 1 :64-68 (the erratum/supersession line, which forces a record rather than an erratum here); references/adr/0069-null-ordering-operators-are-lint-errors.md (the ordering-is-a-lint-error clause the short-circuit residual rests on); the live pack predicates, **quoted by path and line and verified against the working tree at `182b640`**: spikes/p9/catalog-b/feature-flag-kill.toml:43 (step `authorized`, :39-44, `on_false = "fail"`), spikes/p9/catalog/api-contract-spec.toml:10, spikes/p9/catalog/integration-tests.toml:10, spikes/p9/catalog/canary-release.toml:73; spikes/p9/facets.toml:131-134 (`external_dep`, `nullable = true`, `source = "human"`) and :124-129 (`data_migration`, `nullable = true`, `source = "derived"`); crates/rk-kernel/src/predicate/eval.rs (the `Expr::And` and `Expr::Or` arms, and the `or3`/`and3` helpers that serve `in`-membership and the quantifiers) and crates/rk-kernel/tests/it/evaluator.rs at `182b640`; runbook/PROGRESS.md §2 row **L4** (the residual raised at Step 7b Slice 1 by a review lens, confirmed by measurement and falsified by mutation on 2026-09-13); references/2026-09-08-acceptance-criteria.md AC-079, AC-054, AC-055 **by id**, the register having been under concurrent amendment on 2026-09-13; references/2026-09-08-architecture-design.md §4.4 steps 2 and 4 (:510, :512); the operator's decision of 2026-09-13 taken at Step 7b Slice 3 — there is no `spikes/DECISIONS-2026-09-13.md` and this record is that decision's primary record
supersedes: ["0103"]
superseded_by: []
related: ["0032", "0069", "0089", "0090", "0092", "0093"]
evidence: ["F42", "S-GRAMMAR", "F39"]
consumers: []
---

## Context and problem

**ADR-0103 decision 4, read as written, makes `and` and `or` non-commutative, and `rk-kernel` ships them
commutative.** The sentence, quoted whole from `0103`:70-71:

> `and`/`or` are three-valued: `false and null` = false, `true or null` = true, otherwise `null`.

**The literal reading is not merely available — the record forces it**, on three measured grounds:

1. ***"otherwise `null`"* is a total catch-all.** Every operand pair the sentence does not name falls into it. It
   does not name `null and false`, and it does not name `null or true`. There is no gap for a reader to fill with a
   convention: those two cells are assigned, and they are assigned `null`.
2. **The same decision spells out BOTH orders, two clauses earlier, when it means commutativity.** `0103`:69-70 —
   *"`null == v` **and** `v == null` for non-null `v` are **`null`**"*. An author who writes both orders for `==`
   and exactly one order for each connective in the next sentence has distinguished the two cases, not abbreviated
   one of them.
3. **Nothing elsewhere in the record reads back on it.** `0103`'s `**What this forbids**` block (:112-118) carries
   five bullets — precedence, value-position identifiers, `any` over `null` or empty, `Unsatisfiable` on a `null`
   predicate with the `gate_opened` counting rule, and the closed `on_false` set — and **not one mentions the
   connectives** [measured, 2026-09-13]. There is no second sentence to read the first against.

So under ADR-0103 as accepted, `false and null` is `false` while `null and false` is `null`; `true or null` is
`true` while `null or true` is `null`. `a and b` differs from `b and a`.

**`crates/rk-kernel` implements the other reading.** `predicate/eval.rs`'s `Expr::And` and `Expr::Or` arms absorb
from either side — `(_, Tri::False) => Tri::False` and `(_, Tri::True) => Tri::True` — and
`tests/it/evaluator.rs` asserts `Tri::False` for `null and false` and `Tri::True` for `null or true`. **The shipped
code therefore contradicts an accepted `## Decision` sentence.** PROGRESS §2 row **L4** raised this at Slice 1,
recorded it as *taken provisionally in code, clarification owed*, and falsified it: mutating the live `Expr::And`
arm to the literal reading turns the named test red with the right message, and restoring it turns it green
[measured, 2026-09-13].

**An erratum is not available, and the crate's own header proposed one.** `eval.rs`:31-39 names what it thought was
owed — *"an erratum against ADR-0103 decision 4 quoting the frozen sentence"*. ADR-0092 decision 1 :64-68 closes
that route: an erratum *"leaves every sentence of `## Decision` … standing as written. A correction that would move
either is a supersession and takes a new record."* This correction moves the sentence at :70-71. It is a
supersession.

**And it is a departure on the merits, not a clarification of what ADR-0103 already meant.** That distinction is
ADR-0092's own test and it is applied here against this record's own interest: the three grounds above say the
record means the literal reading, and this record overrides it anyway, because of what the literal reading does to
predicates that are already written.

### The measured evidence — four live predicates the literal reading breaks

All four verified by opening the file at `182b640`. `external_dep` is `nullable = true, source = "human"`
(`spikes/p9/facets.toml`:131-134); `data_migration` is `nullable = true, source = "derived"` (:124-129); `kind` is
**not** nullable (:24-26), so `kind == project` is always a definite `true` or `false`.

**1. A check whose author's intent becomes unreachable.** `spikes/p9/catalog-b/feature-flag-kill.toml`, step
`authorized` (:39-44):

```toml
[[step]]
id      = "authorized"
kind    = "check"
inputs  = { answer = "${authorize.answer?}" }
expr    = "answer == null or answer == 'proceed'"     # :43
on_false = "fail"
```

The evident intent is *permit if the gate was not asked, or if it was asked and answered `proceed`*. Once `answer`
**exists**, decision 4's own standing clause makes `answer == null` evaluate to **`null`** (`v == null` for non-null
`v` is `null`). The predicate is then `null or true` when the answer is `proceed` and `null or false` otherwise —
and under the literal reading **both are `null`**. The row **can never be true once `answer` exists**: the opposite
of what it says. `on_false = "fail"` fires on `false` (ADR-0044's closed table form), and `null` is neither, so the
step has no defined behaviour at all — decision 4 makes a top-level `null` the **abstention case**, which decision
5(ii) routes to `gate_opened { reason: unknown_facet, axis }` at **selection step 2**, a route with no meaning for
a step `expr` evaluated inside a running activity. Under commutative absorption the `proceed` case is `true` and
the row works as written.

**2 and 3. The same condition, operands swapped, in two shipped activities.**

```
spikes/p9/catalog/api-contract-spec.toml:10   applicable_if = "external_dep or kind == project"
spikes/p9/catalog/integration-tests.toml:10   applicable_if = "kind == project or external_dep"
```

Both rows are `stage`-different but semantically the same guard, and `integration-tests.toml`:1 even names it in
its own comment. On a `kind = project` item whose `external_dep` was not supplied at the triage gate — `null`, and
ADR-0090 makes that facet human-sourced, so an unsupplied value is the normal case rather than the exotic one —
the literal reading gives:

- `api-contract-spec`: `null or true` → **`null`** → abstention → `gate_opened { unknown_facet }` at step 2, the
  activity not selected;
- `integration-tests`: `true or null` → **`true`** → the activity selected.

**Two activities behave differently on operand order alone**, on a guard two authors wrote both ways meaning one
thing. Under commutative absorption both are `true`.

**4. The `and` side, already pinned in the crate.** `spikes/p9/catalog/canary-release.toml`:73:

```toml
when = "${rollback.outputs?} == null and (${proceed.answer?} == 'proceed' or rigor < mandatory)"
```

Its left operand is `null` whenever `rollback.outputs` is **present** (decision 4: `v == null` is `null`). Under
commutative absorption the conjunction can be `false`, which step 4 may report as `Unsatisfiable`; under the
literal reading it is `null`, which is the **abstention** case. **Abstain and unsatisfiable are not the same
answer** — design §4.4 separates them by step (:510 versus :512) and decision 5(iii) forbids conflating them.

**The exposure is not confined to these four.** `external_dep` occurs in **12** live predicate strings across
`spikes/p9/`, of which **7** combine it with another operand under `or` [measured, 2026-09-13,
`grep -rn external_dep spikes/p9 --include=*.toml | grep -E 'applicable_if|when|expr'`]. Five of those seven are
the row `"auth or data_migration or external_dep"` (`catalog/threat-model.toml`:9, `packs/initiative.toml`:42,
`packs/project.toml`:42, `packs/story.toml`:24, `packs/story-ue.toml`:24), which under left-associativity is
`(auth or data_migration) or external_dep` and whose first two operands are both nullable-or-absent — but `auth` is
the pre-ADR-0089 spelling the facet registry no longer carries, so those five are a load defect of a different kind
and are **counted as corroboration, not as clean instances**.

## Decision drivers

- **A rule that makes an already-written predicate mean the opposite of what its author wrote is the defect.** The
  packs were authored against the conventional reading, which is the same ground ADR-0103 itself used to reject
  making mixed unparenthesised connectives a lint error.
- **Operand order is not a carrier of meaning anywhere else in this grammar.** ADR-0103 decision 1 fixes precedence
  and associativity precisely so that two implementers parse one string one way; a non-commutative `and` reinstates
  exactly the class of surprise that decision exists to remove, at the evaluator instead of the parser.
- **The abstain-versus-`Unsatisfiable` boundary is about to become an emitted outcome.** Step 7b Slice 3 builds
  selection steps 2 and 4, where `null` becomes `gate_opened { unknown_facet }` and `false` becomes
  `Unsatisfiable`. Shipping the engine over an unresolved connective table would put the ambiguity into the event
  log, where it stops being a code question.
- **The alternative costs more than it saves.** Repairing the code to the literal reading is two match arms; it
  would also require rewriting live pack rows to encode an order they already encode, and would leave
  `feature-flag-kill.toml`:43 with no reachable true branch until someone noticed.
- **A stage may not settle a record's ambiguity in code** (`00-preamble.md`:55). Slice 1 correctly did not: it
  pinned the divergence with a named, falsifiable test and recorded L4. What was owed was this record.

## Decision

**1. `and` and `or` are Kleene's strong three-valued connectives, and both absorbing positions absorb.** In full,
over `{true, false, null}`:

| `and` | `true` | `false` | `null` |
| --- | --- | --- | --- |
| **`true`** | `true` | `false` | `null` |
| **`false`** | `false` | `false` | **`false`** |
| **`null`** | `null` | **`false`** | `null` |

| `or` | `true` | `false` | `null` |
| --- | --- | --- | --- |
| **`true`** | `true` | `true` | **`true`** |
| **`false`** | `true` | `false` | `null` |
| **`null`** | **`true`** | `null` | `null` |

Both operations are **commutative and associative**. `not` is unchanged: `not null` is `null` (ADR-0103 decision 4,
standing).

**2. Exactly two cells move, and the withdrawal is one sentence.** What is withdrawn from ADR-0103 decision 4 is
the sentence at :70-71 — *"`and`/`or` are three-valued: `false and null` = false, `true or null` = true, otherwise
`null`"* — and specifically its *"otherwise `null`"* catch-all **as it applies to `null and false` and
`null or true`**. Those two cells change from `null` to `false` and `true` respectively. The two cases the sentence
names keep the values it gives them; they are the same under both readings. The catch-all keeps its full force
everywhere else: `true and null`, `null and true`, `false or null`, `null or false`, `null and null` and
`null or null` are all `null`, unchanged. **Nothing else of ADR-0103 is withdrawn.**

**3. What stands, enumerated so it cannot be read wider.** All of these remain in force exactly as ADR-0103 states
them:

- **Decision 4's equality and membership rules** — `null == null` is `true`; `null == v` and `v == null` for
  non-null `v` are `null`; `!=` and `in` against `null` are `null`; ordering against `null` stays a load-time lint
  error (ADR-0069).
- **`not null` is `null`.**
- **The quantifier rules** — `all` and `none` over `null` or an empty list are `true`; `any` over `null` or an
  empty list is `false` (ADR-0032's *"vacuously true"* stays withdrawn for `any`); an empty present list and an
  absent reference share these semantics.
- **The abstention case** — a predicate whose top-level value is `null` is the abstention case.
- **The narrowing pattern** — the lint recognises exactly one, `x != null and <ordering on x>` at the same
  conjunction level, no `or`, no `not`; any other ordering against a nullable facet is the load-time lint error.
- **Decisions 1, 2, 3, 5 and 6 in their entirety** — precedence and associativity, literal-versus-field, quoting
  and the quantifier forms, the lint → `gate_opened` → `Unsatisfiable` order of outcomes, and the closed `on_false`
  form set.
- **Every bullet of ADR-0103's forbids block** (:112-118). None of the five concerns the
  connectives, and none moves.

Because the withdrawal is one sentence and the rest of the record stands, **ADR-0103 stays `accepted`** and carries
`superseded_by: ["0121"]`, per the README's partial-supersession rule.

**4. This is a supersession on the merits and is not to be presented as a clarification.** ADR-0103 means the
literal reading; this record overrides it. Any downstream document that describes `0121` as *clarifying what `0103`
already said* is wrong and is to be corrected to this framing.

**5. What this record does NOT decide**, named rather than left to be discovered.

**(a) Evaluation order and short-circuiting are a separate rule, and this record does not supply them.** The table
in clause 1 is a rule about **values**. It says what `false and E` is worth; it does not say whether `E` is
evaluated. That distinction is load-bearing here because **an ordering against `null` is not a value at all**:
ADR-0069's forbids is *"lint must reject it, not evaluate it with some default ordering"*, so an evaluator that
reaches `x >= med` with `x` absent must **error**, not abstain. Decision 4's blessed narrowing pattern
`x != null and <ordering on x>` is therefore safe **only because the ordering is never reached**, and commutative
absorption alone gives no such guarantee — a conforming implementation could evaluate both operands and combine
them, and the pattern the lint blesses would error at every abstaining row.

**So the implementation must still guarantee, and this record states rather than provides:**

- `and` returns `false` on a `false` **left** operand **without evaluating the right**;
- `or` returns `true` on a `true` **left** operand **without evaluating the right**;
- the narrowing pattern's safety rests on the first of those, not on clause 1's table;
- **AC-079's read-facet accounting rests on the same two.** Decision 5(ii) — standing — counts `gate_opened`
  *"once per read facet, never per abstaining facet"*, and a short-circuited operand's facets were **not read** and
  must not be counted.

Left-sequential evaluation and a commutative value table are consistent, and the reason is worth stating so the
next reader does not treat them as a contradiction: **the short-circuit never changes a defined value; it only
avoids an evaluation that has no value.** Where both operands evaluate, order is immaterial by clause 1. Where the
unevaluated operand would have errored, there was no value for commutativity to be about. The table is
commutative; the evaluator is left-to-right; a criterion pins the second, not this record.

**(b) `null != null` is not decided here.** The narrowing pattern's guard needs `x != null` to be **`false`** when
`x` is absent, or the short-circuit in (a) never fires. Decision 4 says `!=` *"against `null`"* is `null`, which
would leave the guard `null` and the guard unable to guard. `rk-kernel` reads `!=` as the negation of `==`, giving
`not (null == null)` = `not true` = `false`. **That reading is a second, separate residual against ADR-0103
decision 4**, it is named here so it is not silently absorbed into this supersession, and decision 4's `==`, `!=`
and `in` clauses stand untouched under clause 3.

## Considered options

### Option — repair `rk-kernel` to the literal reading and change no record

Two match arms in `predicate/eval.rs`; the named test in `evaluator.rs` inverts; ADR-0103 is left alone and the
ADR set does not grow. **Rejected on the measured consequences, which is the whole of this record's argument.**
`feature-flag-kill.toml`:43 loses its true branch entirely once `answer` exists, and `on_false = "fail"` gets a
value it cannot act on. `api-contract-spec.toml`:10 and `integration-tests.toml`:10 — the same guard with its
operands swapped — begin to select differently. None of that is a pack defect a lint could report, because both
strings parse and both are legal under every other clause of ADR-0103; the divergence appears only at evaluation,
and F42 records that **nothing has ever evaluated these predicates** — *"the probe parses; it does not evaluate"*.
Repairing the code would ship a semantics whose first real exercise is the Step 7b selection engine.

### Option — an erratum against ADR-0103 decision 4

What `eval.rs`:31-39 named as owed, and the lighter instrument. **Rejected on mechanism, not on merit:** ADR-0092
decision 1 :64-68 forbids an erratum that moves a `## Decision` sentence, and this moves the one at :70-71.
Recording it as an erratum would also put the change outside the supersession graph `census_adr.py` validates,
where a reader of the index would never find it — the same ground ADR-0115 rejected the same option on.

### Option — leave L4 open and carry the divergence into Slice 3

Keep the test, keep the PROGRESS row, build the selection engine, and resolve the connective table later.
**Rejected as a terminal state, not as a stage act** — the Slice-1 stop was correct and is why this record exists.
Slice 3 is precisely where `null` becomes `gate_opened { unknown_facet }` and `false` becomes `Unsatisfiable`
(design §4.4 :510, :512), so carrying the ambiguity forward writes it into the event log and into golden fixtures
that later have to be re-recorded.

### Option — keep the literal reading and parenthesise the packs to say what they mean

Declare `and`/`or` left-sequential, add parentheses or reorder operands across the live rows, and treat the four
predicates above as author errors. **Rejected:** it rewrites live rows to encode an order they already encode —
ADR-0103's own stated ground for rejecting a mixed-connective lint error — and the swapped pair is the refutation
in miniature. Two authors wrote `external_dep or kind == project` and `kind == project or external_dep` for one
condition, in the same catalog, and one of them commented that they were the same. A grammar in which they are not
is a grammar its own authors do not have.

## Consequences

**Positive**

- The shipped evaluator and the accepted record agree, and `rk-kernel`'s module header stops carrying a 🛑 banner
  over a divergence.
- Four live predicates — one `check` expression and three selection guards — evaluate to what their authors wrote,
  and operand order stops being a carrier of meaning the grammar never declared.
- Abstain-versus-`Unsatisfiable` is settled **before** Slice 3 emits either, so no golden log records the
  unresolved reading.
- PROGRESS §2 row **L4** closes with a record above the runbook, not with a code comment.

**Negative**

- **An accepted `## Decision` sentence is overridden on judgement, not on measurement.** The three grounds that the
  record forces the literal reading are measured; the choice to keep Kleene rather than repair the code is a
  judgement about consequences and is recorded as one.
- **The evidence is a lower bound, not a census.** Four clean instances and five corroborating rows, found by
  reading, in a corpus where nothing has ever evaluated a predicate. No instrument enumerates every row whose
  outcome class moves, and this record does not claim one does.
- One more record, and one more partial supersession of `0103` — which was already the set's largest superseding
  record at four predecessors.
- Every hard-coded ADR count in the runbook moves again. Checkpoint 1's `ls references/adr/*.md | wc -l` assertion
  moves for the **sixth** time in three days.

**What this forbids**

- Forbids an evaluator or lint in which `a and b` differs from `b and a`, or `a or b` from `b or a`, for any
  operands drawn from `{true, false, null}`.
- Forbids evaluating `null and false` as `null`, or `null or true` as `null`.
- Forbids an evaluator that reaches the right operand of an `and` whose left operand evaluated `false`, or of an
  `or` whose left operand evaluated `true` — the table is commutative, the evaluator is left-sequential, and
  decision 4's narrowing pattern depends on the second.
- Forbids counting a short-circuited operand's facets among the read facets AC-079's `gate_opened` count is taken
  over.
- Forbids reading this record as withdrawing anything of ADR-0103 beyond the sentence named in clause 2 — the
  `==`/`!=`/`in` null rules, `not null`, the quantifier rules, the abstention case, the narrowing pattern,
  decisions 1, 2, 3, 5 and 6, and every bullet of `0103`'s forbids block all stand.
- Forbids deriving the value of `null != null` from this record.
- Forbids presenting this record as a clarification of what ADR-0103 already meant, or as an erratum.

## Evidence and confidence

- [documented, `references/adr/0103`:70-71] the superseded sentence, quoted verbatim above; [measured, 2026-09-13]
  its forbids block at :112-118 carries five bullets and none names a connective.
- [measured, 2026-09-13, `182b640`] `spikes/p9/catalog-b/feature-flag-kill.toml`:43 —
  `expr = "answer == null or answer == 'proceed'"`, in step `authorized` (:39-44) with
  `inputs = { answer = "${authorize.answer?}" }` and `on_false = "fail"`.
- [measured, 2026-09-13, `182b640`] `spikes/p9/catalog/api-contract-spec.toml`:10 —
  `applicable_if = "external_dep or kind == project"` — and `spikes/p9/catalog/integration-tests.toml`:10 —
  `applicable_if = "kind == project or external_dep"`. The second file's :1 comment names the guard as a require
  row for the same reach.
- [measured, 2026-09-13, `182b640`] `spikes/p9/catalog/canary-release.toml`:73 —
  `when = "${rollback.outputs?} == null and (${proceed.answer?} == 'proceed' or rigor < mandatory)"`.
- [measured, 2026-09-13] `spikes/p9/facets.toml` — `external_dep` at :131-134 is `nullable = true`,
  `source = "human"`; `data_migration` at :124-129 is `nullable = true`, `source = "derived"`; `kind` at :24-26
  declares ten values and **no** `nullable` key.
- [measured, 2026-09-13] `external_dep` occurs in **12** live `applicable_if`/`when`/`expr` strings under
  `spikes/p9/`; **7** combine it with another operand under `or`; **5** of those 7 are the
  `"auth or data_migration or external_dep"` row, whose `auth` is a pre-ADR-0089 spelling absent from the facet
  registry, and are counted as corroboration only.
- [measured, F42 / S-GRAMMAR, 2026-09-11] *"the probe parses; it does not evaluate. ADR-0103's three-valued
  connectives and quantifier null rules are unexercised (0 quantifiers in the corpus). Step 7b's evaluator is the
  first implementation."* The grammar-probe census of 2026-09-13 counts `and` 23, `or` 25, `null` 2 over 287 live
  predicates — a census of **parses**, not of evaluations.
- [measured, F39] `data_migration` emits **zero positives on 20 items** and is not measurable, so it resolves
  `null` in practice rather than `false` — which is why the rows that disjoin it are reachable at all.
- [measured, 2026-09-13, PROGRESS §2 row L4] the divergence was raised at Slice 1, warranted at both
  implementation sites, pinned by a named test, and **falsified**: mutating the live `Expr::And` arm to the literal
  reading turns that test red with the right message; restoring it turns it green. The first falsification attempt
  mutated `or3` and the test stayed green — `or3`/`and3` serve `in`-membership and the quantifiers, not the
  connectives — which is recorded in PROGRESS as a lesson about the harness, not about the code.
- [documented, `references/adr/0092` decision 1 :64-68] the erratum/supersession line that forces a record here.
- [documented, `references/2026-09-08-architecture-design.md` :510, :512] selection steps 2 and 4, which is where
  `null` and `false` become different emitted outcomes.
- **Confidence.** That ADR-0103's text forces the literal reading is **measured** from the record itself, on three
  independent grounds. That four named live predicates break under it is **measured**, by opening each file. That
  keeping Kleene is the better of the two repairs is a **judgement** taken on those consequences, and it is not
  measured: no instrument has ever evaluated a predicate in this corpus, so nothing bounds how many further rows
  change outcome class.
- **What this evidence does not license.** It does not license claiming that the literal reading breaks most or all
  of the corpus — nine of the twelve `external_dep` rows are untested here and five are excluded on a separate
  defect. It does not license claiming Kleene's strong three-valued logic is *correct* in general; it licenses only
  that it is the reading these authors wrote against. And it does not license treating the short-circuit obligation
  of clause 5(a) as decided by clause 1 — that is exactly the inference this record forbids.
