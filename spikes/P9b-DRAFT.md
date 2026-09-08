## P9b — RESULT: a second author lands at a median 73 lines (+29% on P9, still 2× under the kill), the lint caught 1 real mistake in 3 files and silently accepted 5 guesses, and 20 of 27 authoring ambiguities were in neither §9a nor P9

**Established by:** a second author (not the schema's author) writing 3 activities under `spikes/p9/catalog-b/` against `references/2026-09-07-activity-graph-schema.md` §1–§9 as amended by §9a, without reading `P9-DRAFT.md` or `spikes/p9/catalog/` until after the second lint run. Lint unmodified (`spikes/p9/lint.py`, predates §9a). **Zero model dispatches.** Raw outputs: `spikes/p9/results-b-run1.txt`, `results-b-run2.txt`. Registry additions appended under a `P9b additions` header in `spikes/p9/schemas.toml` (16 value schemas, 44 lines) and `questions.toml` (4 questions, 12 lines); `facets.toml` untouched. Nothing under `spikes/p9/catalog/` or the schema note was edited.

Second author's disclosed bias: a model, reading the same proposal documents the first author wrote, in the same session lineage. This is a *different* author, not an *independent* one; a human second author remains the number that matters.

### Metrics

| metric | P9 (first author, n=10) | P9b (second author, n=3) | kill |
| --- | --- | --- | --- |
| non-blank, non-comment lines per activity [measured] | median 56.5 (40–102) | **median 73** — `blameless-postmortem` 65, `feature-flag-kill` 73, `api-contract-spec` 74 | > 150 |
| of which `[activity]` header [measured] | ~17 | 16 / 15 / 14 | — |
| steps per activity [measured] | median 5.5 (3–11) | **7 / 8 / 8** (median 8) | — |
| lines per step, ex-header [measured] | ~8 | 7.0 / 7.3 / 7.5 | — |
| lint first-run findings [measured] | 47 errors, 11 warnings → 13 real / 34 false | **4 errors, 0 warnings → 2 real (1 distinct mistake) / 2 false (1 lint bug) / 0 §9a disagreements** | — |
| guesses the lint could not see [measured, listed §2] | — | **5** | — |
| registry lines added [measured] | 151 total | +56 | — |
| authoring wall-clock [measured] | ~4 min / 10 files | first file write 2026-09-07T22:55:48Z → last registry write 22:56:55Z (**67 s** of writing; ≈3 min including graph design after the reads finished ≈22:54Z) | — |
| fragment repeats [measured] | `check↺ → gate?` 5/10 | `agent/reviewer? → check↺` 2/3 | > 5 of 10 |

The three activities were chosen so none shares a stage with its most similar P9 graph, and each ended up carrying a gate, a loop and an adversary — i.e. all three sit at the complex end where P9's 67–82-line graphs (`root-cause`, `canary-release`) live. Lines-per-step is the same as P9's; the higher median is step count, not verbosity [inferred].

### 1. First-run findings by rule, adjudicated [measured]

| # | activity | rule | finding | adjudication |
| --- | --- | --- | --- | --- |
| 1 | blameless-postmortem | R3 | `reviewed: .outputs on gate step 'review-meeting'` | **real.** Schema §3 writes `${gate.answer}`; I read "gate" as a placeholder for the step id but assumed the answer sat under `.outputs`. The schema never says a gate has no `.outputs`. Fixed to `${review-meeting.answer}`. |
| 2 | feature-flag-kill | R3 | `authorized: .outputs on gate step 'authorize'` | **real,** same mistake. Fixed to `${authorize.answer?}`. |
| 3 | blameless-postmortem | R9 | `applicable_if: identifier 'security' is not a facet…` | **false alarm (lint bug).** The identifier regex `[A-Za-z_][A-Za-z0-9_]*` splits the registered facet value `security-vuln` at the hyphen. The tag `kind:security-vuln` in the same file passed (tag check uses `partition`). Kept bare, per every facet-value example in the schema and proposal. |
| 4 | blameless-postmortem | R9 | `… identifier 'vuln' …` | same bug, second half of the token. |

Run 2 (`results-b-run2.txt`): the two R9 lint-bug errors remain, nothing else. No lint/§9a disagreements fired: I used no forward `on_false`, no `??`, no `[*]`, and my budgets clear both the old rule 8 (Σ 1+retry.max = 6 / 9 / 8 ≤ 8 / 10 / 12) and the §9a.7 form.

### 2. What the lint accepted that it should not have [measured]

Each of these was a guess at a point where the schema was silent; the lint passed all five, so none counts as a catch — and a second author is exactly who makes these.

1. `spawn = ["ProblemRecord"]` in `[activity]` — invented key, ignored.
2. `max = 2` on a `check` step to bound the `on_false` back-edge (rule 1 says "each with a finite `max`" but gives the field no home) — ignored.
3. `${observe.escalate.answer?}` — a ref path that resolves to nothing; `parse_ref` checks the step id, never the path. Rule 3 says "every `${ref}` resolves" and the lint does not check paths against the output schema.
4. `toolchain:flag_set`, `toolchain:metrics`, `toolchain:contract_lint` — invented verbs (P9 §2 already lists this as unlintable).
5. `checklist:postmortem-sections` — §9a.10 says lint checks `checklist:` against a registry; there is none.

### 3. Ambiguity list — every point where the schema was silent or I guessed

Recorded while authoring, before the lint ran. "§9a" = already covered by an amendment; "P9" = named in `P9-DRAFT.md` but not in §9a; "new" = neither.

| # | needed | schema said | did | status |
| --- | --- | --- | --- | --- |
| A1 | a `ProblemRecord` *spawn signal* (proposal §3 `spawn_followup`) | nothing — three exits, no spawn construct | emitted `ProblemRecord` as a third provided artifact and added an unregistered `spawn` key | **new** |
| A2 | a role for "postmortem author" / "operator" | role facet: implementer, reviewer, investigator, spec-author, architect, planner, releaser | `investigator` for the draft, `releaser` for the mitigation record | new (registry) |
| A3 | element fields of `Finding` for `findings.none(severity in […])` | §2 Rust has `Finding.claim/why`; `schemas.toml` has `Findings.items` only | guessed `severity`, value `blocking` | new (registry) |
| A4 | bracket form inside quantifiers | §7 uses `{…}`, §9a.3 uses `[…]` | `[…]` per §9a.3 | §9a (example not updated) |
| A5 | "every action item has an owner and a due date" | §9a.3 quantifiers are membership-only: `all(<field> in […])` | pushed the predicate into the output schema as `unassigned_items` and checked `len(...) == 0` | **new** — the grammar forces predicate design into the model's output type |
| A6 | where the finite `max` of an `on_false` back-edge lives | rule 1 requires one; §7 example has none; `check` has no such field | `max = N` on the check step (lint ignores; first author declares none) | **new** |
| A7 | how a downstream step reads a gate's answer | §3 `${gate.answer}` — literal `gate` or the step id? `.outputs`? | `${step.outputs.answer}` — wrong, lint caught it | new (§3 wording; §8 `StepKind::Gate` has no output type) |
| A8 | whether a gate re-reached via loop-back is a new `Gate:` mark | nothing | assumed new mark | new (P8) |
| A9 | `ProblemRecord` only at `mandatory` (proposal §3) | rule 3 "every `provides` bound" — conditional bound? | unconditional; lint enforces `provides` must be unconditional | P9 (lint message exists; schema text does not say it) |
| A10 | does `wall_clock_secs` include time waiting at a gate | nothing (kernel exits 0 at a gate, so presumably not) | assumed not | new (minor) |
| A11 | `tags = ["kind:incident", "kind:security-vuln"]` semantics | PC8: tags compile to a *conjunction*; §7 example has `kind:bug, kind:incident` which as a conjunction is unsatisfiable | assumed same-facet = disjunction | **new** |
| A12 | `applicable_if` as string or list | proposal §2.2 list; schema §7 canary string | string | new (minor) |
| A13 | parentheses in predicates | §9a.3 omits them | used them; lint accepts | new (grammar) |
| A14 | bare vs quoted values | facet values bare everywhere, step-output strings quoted; hyphenated bare value breaks the lint | bare | **new** (grammar) |
| A15 | `criticality >= high`, `blast_radius >= high` | facets.toml marks "ordered" in a comment only | assumed ordinal | new (registry: `ordered = true`) |
| A16 | returning `RequestDeviation` from inside the graph (gate answer `choose-other-mitigation`) | §3 lists the exit; no step construct produces it; `on_false = "fail"` is presumably `Blocked` | `on_false = "fail"` | **new** — the deviation exit is unauthorable |
| A17 | toolchain verb vocabulary | `test/build/change_summary` + "tech-pack verbs" | invented three | P9 (§2 limits) |
| A18 | `repeat` on a tool step with identical inputs (`observe` three windows) | §5 step key = inputs digest, no iteration; "a completed step is never re-run" | wrote the loop anyway; as specified, iteration 2 is a memo hit and never re-observes | **new** — also affects P9's `hypothesise` loop |
| A19 | where an `on_exhausted = "escalate"` answer is read | §3: resume "into the same step with `${gate.answer}` bound"; §9a says a shorthand *will* imply a gate step after P7 | `${observe.escalate.answer?}` (wrong; the first author declares an explicit gate step with a `when` on the loop's status) | P9 (§3) — the *current* required form is nowhere in the schema text |
| A20 | composing `MitigationRecord` from three step outputs | `emit` binds one ref per artifact | an agent `record` step (a model dispatch to fill a record) | §9a.11 (tool/check step) — but a `check` cannot produce and a "record" tool verb is invented per activity |
| A21 | "DesignDoc *or* Spec" as `requires` | `requires` is a conjunction; `?` shown only on `toolchain:` sources | `requires = ["Spec"]` + `artifact:DesignDoc?` | new (minor; first author did the same) |
| A22 | feeding review findings back into a looped-back step | §1 edges are implicit and rule 1 says acyclic; a ref from `assemble` to the later `adversary` looks like a cycle | did **not** feed back — so `on_false = "assemble"` re-enters a step with unchanged inputs (A18: memo no-op). The first author's catalog feeds back everywhere and the lint has a "feedback ref needs a back-edge" rule the schema never states | **new** — the most consequential gap |
| A23 | does a `for_each` step count once or N times in rule 8 | §9a.7 silent | once | new (minor) |
| A24 | expressing the ordered mitigation family `rollback < feature-flag-kill < …` | `method_rank` per activity, relative to files I cannot see | rank 20, comment naming neighbours | new (catalog-level) |
| A25 | `when` roots: rigor only (§1 wording) or facets/step outputs too | §1 "rigor >= … style"; §9a.3 roots include facets and inputs | `rigor >= recommended or blast_radius >= high` | §9a.3 (§1 wording narrower) |
| A26 | `${specify.outputs}` as the whole fan-out list vs `[*]` | §1 says list; §9a.5 gives projection | whole list | §9a |
| A27 | whether the reviewer's `Findings` on a draft should loop to the draft or fail | nothing normative; §7 loops | loop with `max` | new (convention, minor) |

Tally: 27 ambiguities; 4 already in §9a (A4, A20, A25, A26), 3 named in P9-DRAFT but absent from the schema text (A9, A17, A19), **20 new**, of which 7 are consequential (A1, A5, A6, A11, A16, A18, A22) [inferred].

### 4. Schema amendments proposed (numbered from 12; none applied)

12. **Feedback refs are legal only with a back-edge, and are `null` on the first pass.** A ref to a later-declared step is permitted iff that step, or one after it, has `on_false`/`repeat` targeting the referencing step; it evaluates to `null` until the loop-back. State it in §1 "Edge" and rule 1. The lint already implements this; the schema never says it (A22).
13. **Loop iterations enter the step key.** Add the `repeat`/`on_false` iteration counter to §5's step key, or make it a lint error for a loop-back target whose inputs contain no feedback ref (a loop that cannot learn anything). Without this `observe`-style telemetry loops and P9's `hypothesise` loop are memo no-ops (A18).
14. **`on_false` back-edges declare their bound.** `on_false = { to = "<step>", max = N }` (or `max` on the check step); rule 1 currently demands a finite `max` with no field to hold it (A6).
15. **Gate outputs are `answer`, nothing else.** §3: "downstream steps read `${<gate-step-id>.answer}`; `.outputs` on a gate is a lint error." Add `Gate { … } → answer: OptionId` to §8. Until the post-P7 shorthand lands, document the current pattern for `on_exhausted = "escalate"`: declare a `gate` step with `when` over the looped step's status and read its `.answer?` (A7, A19).
16. **Deviation is authorable.** `on_false ∈ { <earlier step>, continue, fail, deviate }` with `deviate = { kind = skip|downgrade|add, target = <ArtifactKind> }` — or a question option may map to an exit. Today `RequestDeviation` is in the exit table and unreachable from any step (A16).
17. **Spawn signal.** `[activity] spawns = { ProblemRecord = "initiative" }` (or on `emit`), consumed by the stage's `spawn_followup`; the closeout-incident path in proposal §3 depends on it (A1).
18. **Grammar completeness.** Add parentheses; state that facet values are bare identifiers when they match `[a-z_][a-z0-9_]*` and must be quoted otherwise (`'security-vuln'`); fix the lint's identifier regex either way; `facets.toml` gains `ordered = true` on ordinal facets so `>=` is checkable (A13–A15).
19. **Quantifier bodies take comparisons.** `all(owner != null and due != null)`, not membership only; otherwise authors move predicates into output schemas (A5).
20. **Tags: same facet disjoins, distinct facets conjoin.** One sentence in PC8/§7 (A11).
21. **Provided artifacts are unconditional at every rigor ≥ `min_rigor`.** Rule 3 text; the lint already enforces it (A9).
22. **Rule 3 resolves paths, not just step ids.** `${step.a.b}` is checked against the step's output schema (or `answer` for gates); `${observe.escalate.answer?}` must fail (§2 item 3).
23. **Rule 8 and fan-out.** A `for_each` step counts once (A23).
24. **`artifact:Kind?` is legal; `requires` alternation is not.** Say both (A21).

### 5. Verdict

The lines half of the P9 result survives a second author: 73 versus 56.5 is a step-count difference on three deliberately loop-and-gate-heavy graphs, lines-per-step is identical, and the kill line is still 2× away [measured]. The catch half does not survive in the form P9 stated it. The lint found one distinct mistake in three files (a gate-answer path), and let five guesses through untouched — an invented key, an invented bound, an unresolvable ref path, three invented verbs, an unregistered checklist — because its rules check what the schema's author already knew (step ids, `?` marks, rigor reachability) and not what a newcomer has to guess (paths, keys, verbs, registries). Twenty-seven ambiguities came out of three files against the amended schema, twenty of them new; P9's claim that the amendment count "should fall, not rise" for the next batch is falsified at n=3, and the two most consequential gaps — that feedback refs exist only as lint folklore (A22) and that loops over unchanged inputs are memo no-ops (A18) — sit in the first author's own ten files too [inferred]. PC5 still stands: the cost bar is cleared. But the schema note needs amendments 12–24, and rule 3 path resolution plus a verb/checklist registry check, before a third author is asked to write the other ninety.
