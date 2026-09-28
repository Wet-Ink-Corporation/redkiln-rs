---
id: "0172"
title: "**A comparison against the literal `null` is a definite presence test**: `x == null` is `true` when `x` is absent and `false` when it is present, and `x != null` the reverse. A comparison between a null field and a non-null value stays `null`, and `in` against `null` stands. Supersedes, each in part and quoted, `0103` decision 4's *\"`null == v` and `v == null` for non-null `v` are **`null`**, as are `!=` and `in` against `null`\"* and `0121` decision 3's first bullet, which re-affirms it, as to `==` and `!=` against the literal only (D21-67 ruling 2)"
status: accepted
date: 2026-09-26
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-67 ruling 2 (*"A new record, written `proposed` and carried as an **`s8d` entry prerequisite** that lands before `s8d`'s `check` evaluation, supersedes ADR-0103 decision 4 and ADR-0121 decision 3 in part: **a comparison against the literal `null` is a definite presence test** (`x == null` is true when x is absent, false when present; `x != null` the reverse). A comparison between a null field and a non-null value stays null. No catalog row is edited; `s8d` re-measures `SHIPPED_UNSAT`/`SHIPPED_DEAD` and fixture 3's comment is corrected. It flips at `s8d`'s close under D21-64 ruling 10."*) and D21-64 ruling 10 (this record's flip condition); runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R7) and §8d.3a (a); runbook/phase-1/s8b-catalog-quality.md §8b.10 (the eight selection rows, the `x != null` class, and the tri-valued booleans' null arm, D21-56's *"Raised, not decided"*); references/adr/0103 decision 4 and references/adr/0121 decision 3 and clause 5(b) (the sentences superseded in part, and the residual this settles); crates/rk-kernel/src/predicate/eval.rs (`compare`, and the module header's reading 1) and crates/rk-kernel/tests/it/evaluator.rs at `d46b2eb`
supersedes: ["0103", "0121"]
superseded_by: []
related: ["0032", "0069", "0080", "0089", "0092", "0136"]
evidence: []
consumers: []
---

## Context and problem

ADR-0103 decision 4 fixes the arithmetic of `null`: *"`null == null` is true; `null == v` and `v == null` for non-null
`v` are **`null`**, as are `!=` and `in` against `null`"*. ADR-0121 decision 3's first bullet re-affirms the same rule
when it supersedes decision 4's connective sentence (*"`null == v` and `v == null` for non-null `v` are `null`; `!=`
and `in` against `null` are `null`"*), and its clause 5(b) names `null != null` as a residual it does not decide.

Read with the literal `null` on one side, that rule makes a presence test impossible to write. `x == null` over a
**present** `x` is `v == null` for a non-null `v`, so it is `null`, never `false`; and `x != null` over a present `x`
is `null`, never `true`. `rk-kernel`'s evaluator (`crates/rk-kernel/src/predicate/eval.rs`) implements exactly that:
`compare` sends `==` to `equality`, whose `(Value::Null, _) | (_, Value::Null) => Ok(Tri::Null)` arm answers every
comparison with one null side, and `!=` is its negation (the module header's reading 1).

`s8b` §8b.10 measured what that does to the shipped pack, and D21-67 ruling 2 took it (*"`x != null` can never be
true (the eight selection rows, eleven `check` expressions, AC-527 fixture 3; and the tri-valued booleans' null arm,
folded in)"*):

- **ADR-0103's own narrowing pattern abstains on the case it was written for.** Decision 4 blesses exactly one guard,
  *"`x != null and <ordering on x>`"*. With `x` absent the guard is `false` and the conjunction short-circuits, which
  is what makes the ordering safe against ADR-0069's error. With `x` present the guard is `null`, so the conjunction
  is `null` or `false` and never `true`: a row guarded exactly as ADR-0103 prescribes can never fire.
  `evaluator::predicate_null_arithmetic` asserted that residual as *"RAISED, NOT SETTLED"*.
- **The eight shipped selection rows of the form `x != null and x`** (`data-migration-plan`, `migration-dry-run`,
  `migration-expand-execute`, `schema-migration-script` and the `threat-model` disjunct on `data_migration`;
  `feature-flag-kill` and `flag-progressive-rollout` on `flag_exists`; `ux-flow-design` on `ui`) are the tri-valued
  boolean's null arm: bound to `null`, `true` or `false`, the author means `false`, `true`, `false`. Under decision 4
  they read `false`, `null`, `false`, so no binding makes them `true`.
- **Eleven shipped `check` expressions** carry an `x != null` conjunct over a step output (for example
  `record.rollback != null`, `draft.guard != null`, `timeline.detected_at != null`). Once `s8d`'s `check` evaluation
  runs them, each is `null` whenever the field is present, which ADR-0136 decision 1's `check` arm does not read as
  `true`.
- **AC-527's fixture 3** (`crates/rk-kernel/tests/fixtures/activities/terminal_condition_names_a_verify_result.toml`),
  the positive exemplar of a lawful terminal condition, reads `verify.status == 'passing' and verify.green_at != null`,
  and its own dated comment records that it *"can never be TRUE"*.

ADR-0092 decision 1 leaves every sentence of an accepted `## Decision` standing, so the correction is a supersession
and takes a record. This is that record. D21-67 ruling 2 carries it as an **entry prerequisite of `s8d`**: it lands,
with its evaluator change and its test, before `s8d` builds `check` evaluation, so that no `check` is ever evaluated
under the rule it replaces.

## Decision drivers

- **A guard the grammar blesses must be able to pass.** ADR-0103 wrote the narrowing pattern so that a nullable axis
  can be ordered once it is known to be present. A pattern that is `null` exactly when the axis is present blesses a
  predicate that never fires.
- **An absent fact must not behave like a wrong fact** (ADR-0089 decision 3; ADR-0103's own rejected option *"`null ==
  v` is false"*). That concern is about comparing a null **field** with a non-null **value**: the answer to *"is
  `novelty` `high`?"* is unknown when `novelty` is absent. It does not reach a comparison whose other side is the
  literal `null`, because *"is `novelty` absent?"* has a definite answer whether or not `novelty` is absent.
- **No catalog row is edited** (the ruling). The authors of the eight selection rows and the eleven `check`s wrote the
  presence test they meant; the rule, not the rows, is what made them unsatisfiable.
- **The change is the smallest that settles it.** It is keyed on the literal as written, so every comparison between
  two non-literal operands, every ordering and every `in` keeps its value.

## Considered options

### Option — The literal `null` is a presence test for `==` and `!=`; everything else stands (taken)

D21-67 ruling 2's option, taken as ruled. `x == null` answers whether `x` is absent, `x != null` whether it is
present; a null field against a non-null value stays `null`; `in` against `null` is not the ruling's and stays `null`.

### Option — Edit the catalog rows instead

Rewrite the eight selection rows and the eleven `check`s so that they never compare against `null`, for example by a
new `exists(x)` form. Rejected by the ruling (*"No catalog row is edited"*): it would bump nineteen activities under
ADR-0072 to re-encode what their authors already wrote, add a form ADR-0040's grammar does not have, and leave
ADR-0103's own blessed pattern unsatisfiable on a present axis.

### Option — Make every comparison with a null operand two-valued (`null == v` is `false`)

The option ADR-0103 itself rejected: it makes an absent fact behave like a wrong fact, which ADR-0089 decision 3 was
written to end. Rejected again for the same reason. The presence test needs only the literal case.

### Option — An erratum against ADR-0103 decision 4

What `evaluator::predicate_null_arithmetic`'s *"RAISED, NOT SETTLED"* comment named as owed. Rejected on mechanism:
ADR-0092 decision 1 forbids an erratum that moves a `## Decision` sentence, and this moves one in two records.

## Decision

**1. A comparison against the literal `null` is a definite presence test.** When either operand of `==` or `!=` is
the literal `null` as the predicate is written, the comparison answers presence and is never `null`:

| predicate | `x` absent (`null`) | `x` present (any non-null value, `false` included) |
| --- | --- | --- |
| `x == null` | `true` | `false` |
| `x != null` | `false` | `true` |

Operand order does not matter: `null == x` and `x == null` are the same test (an unquoted `null` in operand position
does not parse under ADR-0103 decision 2, so the reversed order arises only in a constructed expression; it is
answered the same way). `null == null` is `true`, as before. Presence is not truthiness: a present `false` is present.

**2. Everything else in decision 4's equality and membership rules stands.** A comparison between a null **field**
and a non-null **value** stays `null` (`novelty == high` over an absent `novelty` is `null`, and so is
`novelty != high`). `in` against `null` stays `null`. Ordering against `null` stays a load-time lint error and an
evaluation error (ADR-0069). `not null` is `null`. The quantifier rules, the abstention case, ADR-0121's connective
tables and its left-sequential evaluation obligation (clause 5(a)) all stand.

**3. "The literal" means the token, not a value that happens to be null.** The rule keys on the operand
**expression** being the literal `null` (`Expr::Lit(Literal::Null)` in `rk-kernel`), never on an operand's evaluated
value. A reference or field that resolves to `null` is a null field, not the literal: `${a?} == 'proceed'` over an
absent `a` is `null` by clause 2, not `false`. A resolver that binds an absent reference must therefore not bind it by
rewriting the expression into the literal `null`.

**4. What is superseded, each in part and quoted.**
- **ADR-0103 decision 4**: *"`null == v` and `v == null` for non-null `v` are **`null`**, as are `!=` and `in` against
  `null`"*, as to `==` and `!=` whose other operand is the literal `null`. The same sentence stands for a null field
  against a non-null value, and for `in`. ADR-0103 stays `accepted`, and its `superseded_by` gains `0172`.
- **ADR-0121 decision 3, first bullet**: *"`null == v` and `v == null` for non-null `v` are `null`; `!=` and `in`
  against `null` are `null`"*, in the same part. The rest of that bullet (`null == null` is `true`; ordering against
  `null` is a lint error) and every other bullet of decision 3 stand. ADR-0121 stays `accepted`, and its
  `superseded_by` gains `0172`.
- **ADR-0121 clause 5(b)**'s residual, *"`null != null` is not decided here"*, is settled by this record, not by
  ADR-0121: `x != null` over an absent `x` is `false`. That is the value `rk-kernel`'s reading 1 already gives, so the
  reading stands and is subsumed. ADR-0121's forbids bullet *"Forbids deriving the value of `null != null` from this
  record"* stands: the value is derived from this record.

**5. ADR-0103's narrowing pattern stands as a lint rule**, unchanged: *"`x != null and <ordering on x>` at the same
conjunction level, no `or`, no `not`"*. Under this record its guard is `false` on an absent `x` (the short-circuit that
keeps the ordering off ADR-0069's error, as before) and `true` on a present `x`, so the ordering now decides on a
present axis.

**6. It is built in `crates/rk-kernel/src/predicate/eval.rs`'s `compare`**, before `equality` is consulted: when
either operand expression is the literal `null`, `CmpOp::Eq` answers `Tri::from_bool(other.is_null())` and `CmpOp::Ne`
its negation. `equality` itself is unchanged. The module header's reading 1 takes a dated line naming this record.

## Consequences

**Positive**

- ADR-0103's blessed guard passes on a present axis, and the tri-valued booleans' guarded rows read `false`, `true`,
  `false` over `null`, `true`, `false` (never `null`).
- The eight shipped `x != null and x` selection rows, the eleven `check` `x != null` conjuncts and AC-527's fixture 3
  become satisfiable without a catalog edit.
- `s8d`'s `check` evaluation is built on a rule under which a presence conjunct can pass.

**Negative**

- A third record now carries decision 4's equality rules (`0103`, `0121`, `0172`), and a reader has to hold all three
  to know what `x == null` means.
- Selection outcomes can move: a row that abstained (`null`, which AC-079 routes to `gate_opened{unknown_facet}`) on a
  present facet now decides. `s8d` re-measures the shipped lattice (`SHIPPED_UNSAT`, `SHIPPED_DEAD`) after the change
  and records the reading either way.
- Clause 3 puts an obligation on a component that is not built yet: `s8d`'s resolver must bind an absent reference as
  a null value, not as the literal.

**What this forbids**

- Forbids evaluating `x == null` or `null == x` to `null`, and `x != null` or `null != x` to `null`, when the other
  operand is anything but the literal `null`: the answer is `true` or `false`.
- Forbids reading this record as moving a comparison between a null field and a non-null value, or `in` against
  `null`, or any ordering: those stand as ADR-0103 decision 4 and ADR-0069 state them.
- Forbids keying the presence test on an operand's evaluated value rather than on the literal as written, and forbids
  a resolver binding an absent reference by rewriting it into the literal `null`.
- Forbids a catalog edit made to work around the pre-record rule for the rows this record covers.

## Evidence and confidence

- [documented, D21-67 ruling 2] the ruling, quoted in `source`, and its receiver (*"`s8d` (1, 2, 3, and 4's lib.rs
  sentence)"*).
- [documented, `references/adr/0103` decision 4; `references/adr/0121` decision 3 and clause 5(b)] the sentences
  quoted in Decision clause 4.
- [measured 2026-09-26 at `s8c` (package D67), a probe: `s8d` §8d.3a (a)] the reading applied to `compare` and the
  whole workspace run `--no-fail-fast` through the no-dispatch wrapper: 872 passed, 1 failed, 9 ignored over 44 result
  lines, the one failure `evaluator::predicate_null_arithmetic` at *"v == null — left: False, right: Null"*;
  `lattice::p6_lattice_over_the_shipped_pack` green, so `SHIPPED_UNSAT` (1 296) and `SHIPPED_DEAD` (56) did not move.
- [measured 2026-09-26 at `s8d`'s build] `evaluator::a_comparison_against_the_literal_null_is_a_presence_test` was
  run red before the change (*"novelty == null over a present novelty reads Null, not False — left: Null, right:
  False"*) and green after it; `evaluator::predicate_null_arithmetic`, with its three assertions rewritten citing this
  record, likewise. After the change `lattice::p6_lattice_over_the_shipped_pack` prints `unsat=1296 dead=56/102`: the
  two constants did not move.
- **What this evidence does not license.** It does not license claiming that every shipped predicate now means what
  its author intended; it licenses only the presence reading for `==`/`!=` against the literal. It does not license
  any value for a resolved reference, which is `s8d`'s resolver's (clause 3 states only what it must not do).
