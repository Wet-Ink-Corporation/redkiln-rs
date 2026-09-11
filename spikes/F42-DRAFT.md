## F42 — S-GRAMMAR RESULT: under ADR-0103's grammar 285 of 287 live predicates parse to one tree, **zero** depend on precedence, the literal-versus-field rule is load-bearing on 191 sites, and the two failures plus three `on_false` values are lint errors in the packs — one of them in a shipped pack

**Status: COMPLETE.** Registered 2026-09-11 per ADR-0012 against audit §8 M-02 ("no parser was written"); consumer:
runbook Step 7b's lint and ADR-0103. Run the same day. Zero model cost.

**Established by:** `spikes/p9/grammar-probe.py` — an EBNF (in its docstring) and a ~250-line recursive-descent
parser for ADR-0033 + 0040 + 0048 + 0049 + 0070 + 0103, run over every `applicable_if`, `when` and `on_false` string
in `spikes/p9/{packs,catalog,catalog-b}/*.toml` at pin `47faec9f`: **386 strings** — 39 `applicable_if`, 248 `when`,
99 `on_false`. Output in `spikes/p9/results-s-grammar.json`. 2026-09-11, <1 s, $0.

### Result

| question | measured | reading |
| --- | --- | --- |
| Do the live predicates parse under ADR-0103's rules? | **285/287** parse; 2 fail | both failures are pack defects, not grammar gaps (below) |
| Do any depend on the precedence rule? | **0/287** carry a mixed unparenthesised `and`/`or` | the precedence ADR-0103 fixes has no live instance to disagree with; it settles future rows, not existing ones |
| Is the literal-versus-field rule load-bearing? | **191** bare identifiers in value position (`kind == project`, `criticality >= high`, `kind in [incident, …]`) | yes — every one is a literal only because ADR-0103 clause 2 says so |
| Set-membership form | **11** uses of `'text' in change` / `'unreal' in toolchain` (a string literal on the *left* of `in`, a set-valued facet on the right — ADR-0050's "same facet disjoins") | **a form ADR-0033 admits but neither it nor ADR-0103 spells out**: `in` has two shapes, `<field> in [literals]` (1 use) and `<string> in <set-facet>` (11 uses). The probe's grammar states both; the design's grammar section records both; ADR-0103 is not moved (its clauses hold for both) |
| `${ref}` uses | **62**, 11 with the `?` optional marker; step ids inside refs carry hyphens (`${green-verify.outputs.status}`) | the ref segment class is `[a-zA-Z_][a-zA-Z0-9_-]*`, wider than a facet name — recorded in the design; ADR-0103's facet-name class is unchanged |
| Quantifiers | **0** uses of `any`/`all`/`none` in any live row | ADR-0103's quantifier null rules have no live instance; they settle Phase-1 authoring |
| `len()` | **14**, all as `${…}.len()` method form | the method form is the one in use; the prefix form is an alias |
| `null` literal | **2** | |
| `on_false` in the closed set | **96/99** (33 keywords, 66 loop-backs `{ to, max }`) | the 3 outside it are bare step-name strings in `catalog-b` |

### The five defects, by file [measured]

| file | string | defect | shipped? |
| --- | --- | --- | --- |
| `packs/incident.toml` `stage.closeout.spawn[0].when` | `exists(ProblemRecord)` | a user function; ADR-0033 admits none. Rewrite as `${ProblemRecord?} != null` (ADR-0040 ref + ADR-0103 null) or add `exists` to the grammar by successor | **yes** — a Phase-2 pack (`incident` is not `bug-standard`/`story`), so outside Phase 1's load path but a lint error the moment the pack loads |
| `catalog-b/blameless-postmortem.toml` `applicable_if` | `kind in [incident, security-vuln] …` | bare hyphenated value; ADR-0048 requires quoting (`'security-vuln'`) | no — catalog-b is the unshipped extended file (ADR-0027) |
| `catalog-b/api-contract-spec.toml` step 5 `on_false` | `'assemble'` | a bare step name; the closed set is `{ to, max }` \| `"continue"` \| `"fail"` \| `{ to = "deviate", … }` | no |
| `catalog-b/blameless-postmortem.toml` steps 2 and 4 `on_false` | `'draft'` | same | no |

The P9 lint (`spikes/p9/lint.py`, run 2026-09-11) reports `errors=0 warnings=2` over the same files: its R9 is a
regex vocabulary scan and does not parse, which is why none of the five is caught today. Step 7b's lint, built to
ADR-0103, catches all five.

### 1. What this settles [measured]

- The grammar as fixed is **sufficient for every live predicate** once the set-membership `in` form and the
  hyphen-bearing ref segment are stated — two forms the packs already relied on and no record had written down.
  Both are additions of statement, not of semantics; ADR-0103's precedence, literal, quoting and null clauses are
  unaffected and no successor is needed for them.
- The precedence question the audit flagged as "not-answerable" (§8 M-02) has **no live instance**: 0 of 287 strings
  mix `and` and `or` without parentheses. The rule matters for Phase-1 authoring, not for the existing packs.
- The literal-versus-field rule is the load-bearing clause: 191 sites read one way only because of it.

### 2. What this does not settle

- **Evaluation.** The probe parses; it does not evaluate. ADR-0103's three-valued connectives and quantifier null
  rules are unexercised (0 quantifiers in the corpus). Step 7b's evaluator is the first implementation.
- **Ambiguity within a grammar with one parse.** A recursive-descent parser with fixed precedence yields one tree
  by construction; "parses two ways" was checked as "would parse differently under the other precedence", which is
  what the mixed-connective count measures.
- **The Phase-1 catalog after Step 7e.** Pack authoring rewrites rows (ADR-0093's facet re-class; `has_repro` to
  `human`); the probe is re-run at Checkpoint 7e as its regression instrument.

### Limits

1. One grammar reading, one author, one day. The EBNF in the probe's docstring is the reading; a second reader of
   ADR-0033/0048/0103 could write a different one — the probe pins this one.
2. The `?` optional marker inside `${…}` and the `.len()` method form are accepted because the packs use them
   (ADR-0040/0070); the probe did not test malformed refs.
3. `catalog-b` is unshipped and its three `on_false` defects were counted, not excluded; the shipped-catalog count
   is 1 defect (`packs/incident.toml`) out of 287 predicates and 96 on_false values.
