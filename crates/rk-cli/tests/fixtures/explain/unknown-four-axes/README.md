# `unknown-four-axes/` — AC-434's fixture

`inputs/context.json` carries `unknown` on the four ordinal risk axes — `criticality`, `novelty`,
`size`, `blast_radius` — each as the **ordinal domain member**, never JSON `null`:

```json
"criticality": {"domain": ["low", "med", "high", "unknown"], "index": 3}
```

## 🛑 Why not `null`

`FacetValue` is `{Null, Bool, Str, Int, Ordinal{domain,index}, Set}` and **`Null` has no position**,
so AC-434's *"rendered as `unknown` with its position-derived treatment stated"* would be
unsatisfiable against it — there would be nothing to state. A `null` read also mints
`gate_opened { reason: unknown_facet, axis }` at selection step 2, so the axis would open an
abstention gate instead of appearing in an admissibility row at all. The declared domain is recorded
**beside** the index precisely because "position-derived treatment" is the index's meaning, and
`facets.toml` declares all four `ordered = true` over the same domain.

## 🛑 One included node's predicate reads all four

`emit-report`'s `applicable_if` is a conjunction over all four axes. AC-434's GIVEN is *"the
admissibility rows that consumed them"*, and `Outcome.read_facets` carries exactly the context paths
an evaluation actually read — so an axis no included predicate reads is one the test could not
assert, and the assertion on it would be vacuous.

Every digest here is a sha256 over a label naming what it stands for. **No corpus byte.**
