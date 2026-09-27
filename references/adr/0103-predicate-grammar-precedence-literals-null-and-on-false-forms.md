---
id: "0103"
title: The predicate grammar gets precedence and associativity, a literal-versus-field rule, quoting, null semantics per quantifier and connective, and a closed on_false form set
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-052, PIA-053 (§4.6), PIA-048 (§4.1), PIA-100 (§5); references/adr/0032, 0033, 0048, 0069; spikes/p9/lint.py on_false_of; the 248 `when` and every `applicable_if` string in spikes/p9 at pin 47faec9f
supersedes: ["0032", "0033", "0048", "0069"]
superseded_by: ["0121", "0172"]
related: ["0031", "0034", "0035", "0040", "0044", "0049", "0070", "0089", "0093"]
evidence: ["PIA-052", "PIA-053", "PIA-048", "PIA-100", "S-GRAMMAR"]
consumers: []
---

## Context and problem

Eight records define the predicate grammar Step 7b's parser and lint will implement (0033, 0034, 0035, 0040, 0048,
0049, 0069, 0070), and between them they leave the parser's first four questions unanswered [measured, PIA-053]:

- **Precedence and associativity.** ADR-0048 added parentheses "without inventing precedence rules by convention", and
  no later record invented them, so `not a and b or c` has no single parse. The p9 packs already carry mixed
  unparenthesised connectives (`external_dep or kind == project`; `size >= high and blast_radius < high`) [measured].
- **Literal versus field.** ADR-0048 says facet values are "written bare when they match `[a-z_][a-z0-9_]*`" — the same
  class as a facet name — so in `kind == project` nothing says `project` is a value and not a field; the packs rely on
  the reading that it is a value [measured].
- **Quoting.** ADR-0048 shows single quotes (`'security-vuln'`); nothing fixes the quote set or an escape.
- **`null`.** ADR-0032 makes absent `?` inputs `null` and "quantifiers over `null` vacuously true"; ADR-0069 defines
  `==`/`!=` against `null` but "does not state" the result of `null == <non-null>`; nothing says whether `x != null and
  x >= med` narrows `x` for the lint, or which of three outcomes (a load-time lint, a `gate_opened{unknown_facet}` at
  step 2, `Unsatisfiable` at step 4) fires first on a null read [measured, PIA-052]. And "vacuously true" is wrong for
  `any`: `any(s in surfaces, s == "auth")` over an absent `surfaces` must not fire the activity [measured, PIA-048].
- **`on_false`'s form set** is open in the records; the P9 lint's `on_false_of` implements `{ to, max } | 'continue' |
  'fail'` and ADR-0046's `deviate` form exists beside it [measured, PIA-048].
- ADR-0030's initial facet list names `data-migration`, `external-dep`, `market-facing` with hyphens, which its own
  bare-value rule cannot lex; the registry and 110 live occurrences use underscores [measured, PIA-100].

No parser has been written (audit §8 M-02); **S-GRAMMAR** — an EBNF and a throwaway recursive-descent parser run over
every `applicable_if`, `when` and `on_false` string in the p9 packs and catalog — is the check on this record, and its
count of strings that fail or parse two ways is a deliverable, with its denominator.

This record supersedes ADR-0032, 0033, 0048 and 0069 **in part** — each gains the clause it lacked; none of their
stated rules is withdrawn except 0032's "vacuously true" for `any` — and all four stay `accepted`.

## Decision drivers

- A grammar two implementers parse differently is not a grammar; the lint author and the pack authors would otherwise
  infer precedence independently (audit §8 M-02).
- The packs already depend on the conventional reading; declaring mixed connectives an error would rewrite 248 rows
  to encode what everyone already reads.
- `null` is the abstention value ADR-0089 made load-bearing; its arithmetic has to be total.

## Decision

**1. Precedence and associativity**, tightest first: `not` (unary) > comparison and membership (`== != < <= > >= in`)
> `and` > `or`. `and` and `or` are left-associative. Parentheses override. `not a and b or c` parses as
`((not a) and b) or c`. Comparison operators do not chain (`a < b < c` is a parse error).

**2. Literal versus field.** In **operand position** (the left of a comparison, the subject of `in`, the list in a
quantifier, a bare boolean facet) an unquoted identifier is a **field**: a root (`item`, `rigor`, a facet name) or a
`.field` path from one. In **value position** (the right of a comparison, an element of an `in` list) an unquoted
identifier matching `[a-z_][a-z0-9_]*` is a **value literal**. Field-to-field comparison requires the `${…}` reference
form (ADR-0040). Facet names match `[a-z_][a-z0-9_]*`; ADR-0030's hyphenated spellings read as their underscore forms.

**3. Quoting.** String literals are double-quoted `"…"` with `\"` and `\\` as the only escapes; single-quoted
`'…'` is accepted as an alias with `\'` and `\\`. A value that does not match the bare class must be quoted (`"security-vuln"`).
Quantifiers are written `<list-expr>.any(<body>)`, `.all(...)`, `.none(...)`; the prefix form `any(x in list, body)`
is accepted as an alias.

**4. `null`.** `null == null` is true; `null == v` and `v == null` for non-null `v` are **`null`**, as are `!=` and
`in` against `null`; ordering against `null` stays a lint error (ADR-0069). `not null` is `null`. `and`/`or` are
three-valued: `false and null` = false, `true or null` = true, otherwise `null`. Quantifiers: `all` and `none` over
`null` or an empty list are **true**; `any` over `null` or an empty list is **false** (ADR-0032's "vacuously true"
withdrawn for `any`). An empty present list and an absent reference share these semantics. A predicate whose
top-level value is `null` is the **abstention case**. The lint recognises exactly one narrowing pattern: `x != null
and <ordering on x>` at the same conjunction level, no `or`, no `not` — under it the ordering is legal; any other
ordering against a nullable facet is the load-time lint error.

**5. Order of outcomes on a null read.** (i) At **pack load**, the lint rejects an unguarded ordering against a
nullable facet. (ii) At **selection step 2**, a `null` top-level `applicable_if` or `require.when` emits
`gate_opened { reason: unknown_facet, axis }` (AC-079), counted once per read facet, never per abstaining facet.
(iii) **`Unsatisfiable(kind, failed_predicates)`** is reserved for step 4, over predicates that evaluated to `false`,
never for `null`. A must-fail fixture: `any` over absent evidence must not pass an activity.

**6. `on_false` is closed:** `{ to = <earlier step id>, max = N }` | `"continue"` | `"fail"` | `{ to = "deviate", … }`
(ADR-0046's form), with `max` required only for the loop-back form; `lint.py`'s `on_false_of` is the reference
behaviour. `applicable_if` scope is decided once: it is evaluated over the item's context vector at selection, never
re-evaluated inside the activity.

## Considered options

### Option — mixed unparenthesised connectives are a lint error
Safest to parse. Rejected: the p9 packs already mix them under the conventional reading; S-GRAMMAR would report the
count and every one would need parentheses added to say what it already says.

### Option — leave precedence to the parser author with S-GRAMMAR as input
Rejected: Step 7e's pack authors need the rule before the parser exists.

### Option — `null == v` is false
Two-valued and simple. Rejected: it makes an absent fact behave like a wrong fact — the conflation ADR-0089 decision 3
was written to end.

## Consequences

**Positive**
- One parse for every string; `null` arithmetic is total; the pack lint has a precise narrowing rule.

**Negative**
- Three-valued connectives are a small evaluator to get right; S-GRAMMAR's parser is the first implementation.
- `any` over `null` becoming false changes the reading of any existing row that relied on vacuous truth — S-GRAMMAR
  reports how many.

**What this forbids**
- Forbids a parser or lint implementing any precedence but clause 1.
- Forbids an unquoted identifier in value position being read as a field.
- Forbids `any` over `null` or an empty list evaluating to true.
- Forbids `Unsatisfiable` on a `null` predicate, and forbids `gate_opened{unknown_facet}` counted per abstaining facet.
- Forbids an `on_false` form outside the closed set.

## Evidence and confidence

- [measured, spikes/p9 at 47faec9f] 248 `when` strings and the `applicable_if` set, with mixed connectives and bare
  value literals in use.
- [measured, PIA-052, PIA-053, PIA-048, PIA-100] the four undefined questions, the `any`/`null` hole, the open
  `on_false` set, the hyphenated names.
- [documented, ADR-0048 :21] "without inventing precedence rules by convention" — the gap this closes by record.
- **What this evidence does not license.** It does not license claiming every existing predicate parses under clause
  1 — S-GRAMMAR measures that and reports the failing count with its denominator. It does not license any
  performance claim about three-valued evaluation.
