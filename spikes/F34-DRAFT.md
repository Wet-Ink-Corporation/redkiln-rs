## F34 — P6 AUTHORING RESULT: a third author wrote 90 activities at a median 53 lines, the lint's first runs caught 6 distinct mistakes and 1 false alarm across 11 batches (and missed 1 defect found by reading), 30 new ambiguities surfaced, and the `adversary → policy → gate` block is in 42 of 90

**Established by:** authoring 90 activities + registry additions under `spikes/p9/` against `references/2026-09-07-activity-graph-schema.md` §1–§9b (as amended, §9b.25 included), lint v2 unmodified, 2026-09-08T01:30Z–~04:30Z UTC. **Zero model dispatches.** Raw outputs: `spikes/p9/results-v2-batch-<stage>.txt` (first run per batch, unfixed), `results-v2-batch-<stage>-run2.txt` (after fix, only where the first run had errors), `results-v2-catalog-full.txt` (100 activities before the P6 lattice fixes), `results-v2-batch-p6fixes.txt` (101, after them), `lines-v0.txt` (lines/steps per activity). Companion: `F35-DRAFT.md` (P6 lattice). Assigned F34 on 2026-09-07; promote to `FINDINGS.md` with the S-series drafts.

Third author's disclosed bias: a model in the same session lineage as P9/P9b, and — unlike P9b — one that **read `lint.py` before writing a line**. That single fact explains most of §2 and is the first item of §5.

### Metrics

| metric | P9 (n=10) | P9b (n=3) | **P6 batch (n=90)** [measured] | kill |
| --- | --- | --- | --- | --- |
| non-blank, non-comment lines per activity | median 56.5 (40–102) | median 73 | **median 53, mean 56.2, range 29–103, sum 5,059** | > 150 |
| steps per activity | 5.5 | 8 | **5** (2–12) | — |
| lines per step, ex-header (~17) | ~8 | ~7.3 | **~7.9** | — |
| lint first-run findings | 47 err / 11 warn → 13 real / 34 false | 4 err → 2 real / 2 false | **10 err + 1 warn over 11 batches → 9 real (6 distinct mistakes) / 1 false alarm (1 lint bug)**; **+1 defect the lint cannot see, found by reading** | — |
| registry lines added | 151 total | +56 | **+525** (schemas +246, questions +151, registry.toml +18, facets +3 comment) → 728 total; ~7 lines/activity at n=101 | — |
| largest exact-shape fragment | `check↺ → gate?` 5/10 | — | **`check↺ → gate?` 35/100; `agent/reviewer? → check↺` 34/100** | > 5 |
| `adversary → policy (→ gate)` block by intent | 6/10 | 2/3 | **42/90 new (47/100 total)**; exact 3-step `reviewer → check↺ → gate?` in 26 | — |
| authoring wall-clock | ~4 min / 10 | 67 s / 3 | first file 01:31Z → last operate file ≈03:20Z (~110 min for 90 files incl. registries and per-batch lint); not comparable — see F32 §Limits 1 | — |

Full catalog after Part A: **100 activities** (10 P9 + 90), 101 after the P6 lattice's first fix round (`walking-skeleton`), 5,683 non-comment lines. Projection at F32's 95 was ≈5.9k lines; the measured 100 is 5.7k. **PC5's cost bar clears by 2.8×** on the median [measured].

Per stage (P6 batch only, median lines / median steps / n): discovery 59/6/7 · specification 65/7/7 · design 53/5/10 · planning 50/5/6 · closeout 50.5/5/8 · analysis 57/6/11 · implement 53/5/11 · verify 52/5/9 · review 50/5/8 · release 57.5/5.5/6 · operate 63/6/7. The toolchain-realised stages are not longer than the agnostic ones; the two outliers (`branch-by-abstraction` 103, `dependency-update` 88) are multi-loop graphs, not verbose ones.

### 0. What was written, and the selection rule

Proposal §4's table lists ~174 ids, not ~95 (11 triage, 13 discovery, 21 analysis, 17 spec, 19 design, 12 planning, 18 implement, 19 verify, 12 review, 16 release, 14 operate, 12 closeout). Ryan chose "curate ~85" and "skip triage" (triage produces the context vector the engine selects on; nothing can select it). Selection rule, applied before writing: **(a)** every ordered family in §4 complete so `method_rank` has data; **(b)** every kind × stage "yes" cell of the raw catalog's §4 has a producer for its named exit artifact; **(c)** everything the eight pre-registered packs' baselines need; **(d)** the mixed-toolchain slice populated as instructed — generic text producers plus one Unreal specialisation each for `ChangeSummary` (`change-summary-ue`), `TestReport` (`unit-tests-ue`), `ReproCase` (`minimal-repro-ue`). Result: 90, not 85 — families (a) cost the extra five.

Deferred to `extended` (still in the table, unwritten): analytics-review, user-story-mapping, rabbit-hole-scan, market-landscape-scan, stakeholder-map; slo-definition, definition-of-done, example-mapping, data-contract-spec, abuse-case-spec, privacy-requirements, accessibility-requirements, requirements-tracing; architecture-options, context-mapping, data-model-design, privacy-by-design, failure-mode-analysis, observability-design, walking-skeleton-plan; dependency-dag (`vertical-slicing` already provides `DependencyDAG`), scope-cutlines, pre-mortem, test-strategy, migration-sequencing; dora-metrics-review, outcome-vs-hypothesis, docs-finalization, flag-cleanup, action-item-tracking; change-correlation, log-analysis, root-cause-confirmation (`root-cause` has a confirm step), impact-assessment, blast-radius-analysis, dependency-impact-analysis, code-archaeology, feasibility-study, analysis-report; parallel-change, instrumentation, secure-coding-pass, runbook-authoring, self-review, implementation-report (`tdd` emits `ImplReport`); contract-tests, e2e-acceptance-tests, regression-suite, sast-scan, secret-scan, accessibility-audit, backward-compat-test, performance-load-test, test-completion-report, flaky-test-triage; api-review, docs-review, privacy-review; artifact-signing-provenance, sbom-generate, versioning, release-notes, migration-contract-execute, post-deploy-smoke, change-record-close, release-record (every deploy pattern emits `ReleaseRecord` directly, as `canary-release` does), dark-launch; alert-rule-authoring, incident-communication, workaround-publish, error-budget-review, vuln-monitoring, adoption-measurement, runbook-update; all 11 triage ids. `hypothesis-troubleshooting` is `root-cause` (same graph, same rank); not written twice.

**Edits to the ten P9 files** (disclosed; each a one-line catalog-coherence change, each commented in the file with the date): `tdd-red-green-refactor` rank 10→20 (loop family `test-after(10) < tdd(20) < bdd-outside-in(30)`); `adversarial-review` rank 20→30 (review family `checklist(10) < code-review(20) < adversarial(30) < formal-inspection(40)`); `user-story-ac` tags lose `kind:bug` (`fix-spec` provides `Spec` for bugs, per the raw catalog's §4 row). After the P6 lattice's first run four more (F35 §3): `user-story-ac` gains `kind:tech-debt, kind:docs` and requires `IntakeRecord` + `ProblemStatement?` (discovery is forbidden for stories, so the old `requires` was unsatisfiable by construction); `adversarial-review` gains `kind:project, kind:incident`. F33 A24 predicted exactly this: ranks and tags are relative to files an author cannot see.

Every activity carries a `# reach: (kind, stage, rigor)` comment naming the context expected to select it; F35 §2 checks them (23 of the 90 guesses were wrong in the direction of "the lattice never reaches it" — the whole of F35's dead list plus the banding artefact).

### 1. What the lines are spent on

Header ≈17 lines (same as P9). The long-form `adversary → policy → signoff` block is 17–19 lines each time; at 42 repeats that is ≈750 lines, **15% of the batch**, written out by hand because the `quality` shorthand is gated on P7 (schema §9a end). Two other repeated shapes cost as much: `tool/toolchain:test → check↺(green)` (11 activities, ≈12 lines each) and `gate? → tool/toolchain:record → emit` (11 activities). The FRAG output over the full 100 (`results-v2-catalog-full.txt`):

| fragment | activities | note |
| --- | --- | --- |
| `check↺ → gate?` | 35 | the policy check + optional sign-off |
| `agent/reviewer? → check↺` | 34 | adversary + policy, rigor-conditional reviewer |
| `gate? → emit` | 28 | |
| `check↺ → gate? → emit` | 23 | |
| `agent/reviewer? → check↺ → gate?` | 17 | the block, exact 3-step, conditional reviewer |
| `check↺ → emit` | 17 | policy without sign-off |
| `agent/reviewer → check↺` | 13 | unconditional reviewer (min_rigor ≥ recommended activities) |
| `check↺ → gate` | 12 | unconditional gate (human-heavy activities) |
| `tool/toolchain:test → check↺` | 11 | the green loop |
| `tool/toolchain:record → emit` | 11 | see B12 — `record` became an escape hatch |
| `agent/reviewer → check↺ → gate?` | 9 | |

The P9 threshold ("> 5 exact repeats") is now exceeded seven-fold. This does not change F32's answer (shorthand, not includes) — it sharpens it: **two** shorthands would remove most of it (`quality = {…}` and a `green = { run = toolchain:test, loop_to = <step>, max = N }`), and a third (`record = <schema>`) would remove the escape hatch by forcing the verb registry to type its output (B12).

### 2. The lint: 9 real catches (6 distinct mistakes), 1 false alarm, 1 miss — and eight batches with zero findings

First-run outputs per batch, adjudicated on inspection (every false alarm named):

| batch | first run | adjudication |
| --- | --- | --- |
| discovery (7) | 0 err | — |
| specification (7) | 0 err | — |
| design (10) | 0 err | — |
| planning (6) | **1 err** R1 | **real.** `appetite-check.size` read `${overrun.answer?}` (a later gate) but nothing loops back to `size` — a leftover input from an earlier draft of the graph. Fixed by removing the input; the `resize` step is the real consumer. |
| closeout (8) | 0 err | — |
| analysis (11) | 0 err, **1 warn** R5 | **real.** `technical-spike` mints `GreenAt` (a `toolchain:test` step) but promised only `ToolReport`. Promise fixed. |
| implement (11) | 0 err | — |
| verify (9) | 0 err | **1 miss.** `unit-tests.green` (unconditional check) had `on_false = { to = "fill" }` where `fill` is `when = "rigor >= recommended"` — at `optional` the loop-back target is inactive and a failing green has nowhere to go. No rule checks `on_false` targets against R4's per-rigor activity. Found by re-reading; fixed with an unconditional `base-green` check and a conditional `green`. A scan of all 100 files for the same shape found no other instance. Proposed R4b (amendment 35). |
| review (8) | 0 err | — |
| release (6) | **3 err** R1, R9, R5 | R1 **real, design-level**: `flag-progressive-rollout.ramp` (a `for_each` over rollout steps) read `${hold.answer?}` from the gate after it — I wanted a human per ramp step and wrote it as if a gate could sit inside the fan-out. It cannot (B21). Rewritten with one gate after all ramps, which is a weaker activity. R9 **false alarm (lint bug)**: `logs.all(breached.len() == 0)` — the quantifier regex `\.(none|all|any)\([^()]*\)` cannot see past the nested `len()` parens and reported `breached` as an unknown identifier. Worked around with `status == 'ok'`; grammar question in B23. R5 **real, slip**: `rollback` promised `ToolReport` unconditionally but the only unconditional tool (`record`) was not in the emit's evidence list. |
| operate (7) | **6 err** R3 ×5, R5 ×1 | R3 **real, 1 cause**: the mitigation template's `not-restored.when` read `${observe.outputs.status}` without `?` although `observe` is conditional; five files from one template, one mistake. R5 **real, slip**: `incident-command` promised `HumanAnswer` unconditionally from a conditional gate. |
| p6 fixes (+1, 7 edited) | 0 err | — |

Total: 10 errors + 1 warning on first runs, of which 9 real from 6 distinct mistakes (2 design-level: appetite-check's dead input, flag-rollout's gate-in-fan-out), 1 false alarm from 1 lint bug (nested parens in a quantifier body). The second runs were clean every time (`*-run2.txt`).

**Why eight of eleven batches were clean, and what it means.** F33 §2 already said the lint checks what its author knows and passes what a newcomer guesses. This author read the lint first; every rule was in mind while writing, so the first-run error rate collapsed from P9b's 1.3/file and P9's 4.7/file to **0.11/file** — and the one defect that mattered most (`unit-tests`) was in the class the lint has no rule for. Read the 0.11 as "an author who has internalised R1–R10 does not need them"; do **not** read it as "the schema is now unambiguous": §3 lists 30 points where the schema was silent and this author guessed, and F35 shows that the guesses that matter (rank semantics, stage scoping, roots) are catalog-level, invisible to a per-file lint by construction.

What the lint still cannot see, added to F32's list: `on_false` targets inactive at a rigor (found here); catalog-level coherence — rank ties, family membership, whether a kind is producible for the kinds an activity tags, whether any pack row will ever require the artifact (all of these are P6's job and P6 found 23 of them, F35); whether a `record`/`toolchain:*` verb can produce the schema it is declared to; whether a gate's option set is meaningful for the step that reads it.

### 3. Ambiguity list — every point where the schema was silent and I guessed

Recorded while authoring, numbered B1– to avoid colliding with F33's A1–A27. "§9a/§9b" = covered by an existing amendment; "F33" = named there but not adjudicated into the schema; "new" = neither. **Consequential** in bold.

| # | needed | schema said | did | status |
| --- | --- | --- | --- | --- |
| **B1** | an ordered family that *escalates* with rigor (`test-after < tdd < bdd`, `unit < equivalence < property < mutation`, `rollback < flag-kill < …`) | `method_rank`: "lower wins among providers"; `min_rigor`: a floor. Proposal §3 confirms: rank 10 wins whenever admissible | encoded escalation in the LOWER member's `applicable_if` (`unit-tests: criticality < high`, `code-review: criticality < high or rigor < recommended`, `rolling-deploy: blast_radius < med or …`); higher members are deviation-`add` only where I could not | **new — the single most consequential**; F35 §2 shows 14 of 15 dead activities are exactly these |
| **B2** | which activities are producer *candidates* for a stage's artifact — only those with `stage == requiring stage`, or any? | nothing (§2.5 step 4 says "activities with the kind in `provides`") | same stage only; a producer's own `requires` from any non-forbidden stage. Consequence: `rollback` (release) vs `rollback-mitigation` (operate) as two files; `hotfix(60)` in the operate family is unreachable from operate | **new**; F35 I1 |
| **B3** | ordered families whose members provide different kinds (discovery spine; `smoke-test`/`mutation-testing` in the verify family; `blameless-postmortem` vs the retro family) | "ordered methods (pick one by rank/rigor)" | assigned ranks and a family comment; rank is inert where `provides` differ | **new** |
| B4 | dated headers on registry additions | task rule 1 | TOML arrays cannot be re-opened under a later table; `registry.toml`/`facets.toml` additions are in-place with a dated comment; `schemas.toml`/`questions.toml` (one table per entry) take real headers | new (tooling) |
| B5 | a boolean facet as a tag (`auth:true`) | PC8: tags compile to a conjunction | impossible: lint compares `"true"` to `str(True)`; booleans live in `applicable_if` only | new (registry/lint) |
| B6 | facet names | proposal/task: `toolchains`, `change_kinds`; `facets.toml`: `toolchain`, `change` | registry names | new (doc drift) |
| B7 | a question id per gate | registry "owns options"; every P9 gate had its own id | shared `draft.accept` (23 uses) and `loop.exhausted` | new (convention) |
| **B8** | human-supplied root kinds (`IntakeRecord`, `BugReport`, `IncidentRecord`; `ProblemStatement` for stories) | PC3 (`derived_from = []`), `human_source_ok` on the *producer* — nothing says "this input has no producer anywhere" | assumed roots; F35 had to hard-code `ROOTS` | **new**; F35 I1 |
| B9 | quoting `kind == project` vs `kind == 'security-vuln'` | §9b.18 | as specified; reads inconsistently | §9b (minor) |
| B10 | what `human_source_ok` adds beyond PC3 | nothing | set it where a human plausibly pastes the artifact | new (minor) |
| **B11** | a gate that returns *data* (user-test results, the chosen sketch, attached session notes) | `answer: OptionId` only (§9b.15) | enum options like `results-attached`; the results themselves have no channel | **new** — discovery's human steps are theatre without it |
| **B12** | a tool step whose output schema is not implied by its verb (`toolchain:record` → `ClosureRecord`/`ChangeRecord`/`ConfirmationResult`/…) | verb registry lists names only | used `record` 11× and `build` → `TestReport`; the lint accepts any `outputs` on any verb, so R3's schema check is decorative for tools | **new** |
| B13 | does `wall_clock_secs` include gate waits (`design-sprint` 5 days, postmortem meeting) | F33 A10 open | assumed not; set 432000 anyway | F33 |
| B14 | `unknown` in ordinal comparisons | facets order `low < med < high < unknown` | `novelty >= high` is true for `unknown` by position — probably wrong | new |
| B15 | loop-learning rule satisfied by *any* later `?` ref | §9b.13 | added feedback refs; the rule cannot tell a real feedback edge from an unrelated one | §9b (weak) |
| B16 | an activity providing two kinds when a pack requires only one; a second producer of the by-product in the same stage | nothing | let it happen (`structured-nl-requirements` provides `Spec`+`RequirementsSet`; `equivalence-boundary` `TestReport`+`TestDesign`) | new |
| B17 | `when` mixing rigor and data (`rigor >= mandatory or breaking_changes.len() > 0`) | R4 evaluates only the rigor half and treats `or` as live | consumers use `?`; fine, undocumented | new (minor) |
| **B18** | a check whose `on_false` target is rigor-inactive | nothing; no lint rule | found by reading (`unit-tests`); proposed R4b | **new** (lint gap) |
| B19 | `rigor` as a root in an activity-level `applicable_if` | §9a.3 lists `rigor` among roots for step predicates; PC8 lists activity facets without it | used it (`code-review`, `rolling-deploy`) to keep kinds satisfiable at `optional` where the higher member has `min_rigor = recommended` | new |
| **B20** | the mitigation family's selector fact `last_deploy_age < 24h` (proposal §3) | not in `facets.toml`; PC8 rule 1 forbids a new facet from the catalog | `rollback-mitigation` unconditional; ranks 20–50 deviation-only | **new** — proposal §3's own example is unwritable |
| **B21** | a gate per element of a `for_each` (human per ramp step) | nothing | impossible; one gate after the fan-out | **new** |
| B22 | `${x.outputs.status?} != 'stable'` when `x` did not run | §9a.2: `null == null`; other ops on null are errors | used `!=` anyway; every "did the optional step reach X" test needs it | new (grammar) |
| B23 | `len()` inside a quantifier body | §9b.19 allows comparisons; lint regex rejects nested parens | worked around | new (grammar/lint) |
| B24 | an activity with no evidence (`lightweight-retro`, `evidence = []`) | PC6 "with evidence" | lint accepts | new |
| B25 | the `# reach:` triple — which pack row will require this artifact for this kind | PC2 puts rows in the pack the catalog cannot see | guessed; 23 wrong (F35) | new (structural) |
| B26 | id collision with `catalog-b/` (`blameless-postmortem`, `api-contract-spec`, `feature-flag-kill` now exist in both) | nothing on id namespaces | wrote the v0 ones in `catalog/`; catalog-b untouched | new (minor) |
| B27 | which tool verbs read live state (`memo = false`) | §9b.13: "tool steps that read live state declare memo = false" — per use | set per step (`metrics`, `smoke`, `bisect`, `migrate_dry_run`, `slo_query`, `trace_collect`) | §9b (should be a verb property) |
| B28 | `for_each` sourced from another step's list output, whose outputs are then consumed whole downstream | §9a.5 | did it (`branch-by-abstraction.migrate`); accepted | §9a |
| B29 | `on_false = "fail"` → which `Blocked{reason}` | §3 "a step failure with retries exhausted is Blocked" | assumed `Blocked{reason: PolicyFailed}` | new (minor) |
| **B30** | editing a shipped activity for catalog coherence (rank, tags, requires of P9 files) | nothing on ownership | made 7 one-line edits, all commented | **new** (process) |

Tally: 30; 4 covered by §9a/§9b (B9, B15, B27, B28), 1 named in F33 (B13), **25 new**, of which 9 consequential (B1, B2, B3, B8, B11, B12, B18, B20, B21, B30 — ten if B30 counts). F33's prediction that the count would not fall holds at n=90 in *kind* but not in *rate*: 25 new over 90 files (0.28/file) against P9b's 20 over 3 (6.7/file). The new ones are almost all catalog-level (selection, families, roots, packs), not step-level — the step-level grammar has largely converged.

### 4. Proposed amendments (numbered from 26; none applied to the schema note)

26. **Rank semantics stated.** `method_rank` selects the lowest admissible producer; a family escalates with rigor only through the *lower* members' `applicable_if`/`min_rigor` (a ceiling) or through deviation `add`. Either say so and give families a `max_rigor`, or invert: rank selects the *highest* admissible (SHOP2 orders methods most-preferred-first). (B1)
27. **Producer scope.** Candidates for a stage's artifact are activities with `stage == requiring stage`; a producer's `requires` resolve from any non-forbidden upstream stage or from a declared root kind. Add a `roots` list to the pack (or a `source = human` artifact registry). (B2, B8)
28. **Families are declared.** `family = "<name>"` on the activity; lint: members share `provides`, ranks distinct, one file per rank. (B3)
29. **Registries take one table per value** (`[verb.bisect] added = "2026-09-07" outputs = "BisectReport" live = true`) so additions can be dated and verbs typed. (B4, B12, B27)
30. **Boolean facets in tags** are legal (`auth:true`), or the schema says booleans are `applicable_if`-only. (B5)
31. **Names**: `facets.toml`'s `toolchain`/`change` are normative; proposal §2.4 and PC7 updated. (B6)
32. **Shared question ids are legal**; the mark is identified by (activity, step). (B7)
33. **Gate answers may carry attachments**: `answer` + `attachments: [ArtifactRef]`, minted as `HumanAnswer{…, attachments}`. (B11)
34. **Verb registry maps verb → output schema**; a tool step's `outputs` must match; `record` is removed. (B12)
35. **Lint R4b**: an `on_false.to` target must be active at every rigor at which the check is active. (B18)
36. **`rigor` is a legal root in `applicable_if`** (the stage rigor at selection). (B19)
37. **Facet requests.** A catalog may declare `wants_facet = ["last_deploy_age"]`; P1 arm D evaluates derivability before PC8 admits it. (B20)
38. **Gate inside `for_each`**: forbidden explicitly, or defined as one mark per element. (B21)
39. **Null**: `==` and `!=` are defined against null; ordering operators are errors. (B22)
40. **Quantifier bodies admit `len()` and nested calls**; the lint's regex becomes a parser. (B23)
41. **Empty evidence** is legal only for activities with no `agent` step, else an error. (B24)
42. **Catalog-coherence edits** to shipped activities bump `version` and are listed in the pack's changelog; P6 is the test. (B30)
43. **Lattice banding** must include the `med` band (F35: `canary-release` dead because `>= med` vs `>= high` collapse under {low, high}). (F35)

Shorthands already gated on P7, now with sizes: `quality = {…}` saves ≈17 lines × 42; a `green = { run, loop_to, max }` shorthand saves ≈12 × 11.

### 5. Verdict

The cost bar holds a third time, at scale: 90 activities at a median 53 lines, 5.7k lines for the full hundred, no stage above 65 [measured]. The lint's first-run yield fell to 0.11 errors/file because this author read the lint first — which is the cleanest demonstration yet of F33's point that the rules encode their author's knowledge; the one defect that would have shipped an activity with no exit at `optional` (`unit-tests`) was in a class with no rule. Thirty ambiguities, 25 new, but their character changed: P9b's were step-grammar, these are **catalog-level** — rank semantics (B1), producer scope (B2), roots (B8), the mitigation family's missing facet (B20) — and none of them is visible to a per-file lint. That is what P6 is for, and F35 shows it finds them. **PC5 stands on cost. The schema note needs amendments 26–43 before a fourth author, and the two most consequential (26, 27) before Phase 1 authors a selection engine, because F35's numbers depend on the interpretation chosen here.**

### Addendum (2026-09-08, later): decisions applied

Amendments 26–43 were decided with Ryan (`p9/DECISIONS-2026-09-08.md`) and applied: schema §9c, lint v3 (v2 frozen; first run over this catalog = exactly the 25 errors the new rules create, `results-v3-catalog-run1.txt`), `artifacts.toml`, typed verbs, 15 `family` keys with `max_rigor` on four families, 12 composing emits replacing `toolchain:record`, `implement-ue` (102 activities), 46 version bumps. The catalog-level checks now live in `lattice.py --validate`. See F35 §1.6 for what P6 says afterwards.

### Limits

1. Same session lineage as P9/P9b; a human third author is still the number that matters. This author read the lint source, which P9b did not; the two first-run rates are not comparable and neither is "the" rate.
2. Lines are non-blank non-comment; the `# reach:` and family comments (1–3 lines/file) are excluded and would add ≈2 to the median.
3. The 42/90 block count is by intent (reviewer step immediately followed by a check), computed by script over the step list, not the lint's FRAG output; FRAG's 34/100 is exact-shape with the conditional-reviewer variant only.
4. Nothing ran. Whether `record`-style tool steps can be typed, whether `draft.accept` is a usable mark, whether a 5-day `design-sprint` budget means anything — P7/P8.
5. The selection rule in §0 is mine; a different curator would have kept a different 90. The families and the pack baselines are the non-negotiable part; ~30 of the 90 are interchangeable with their deferred neighbours.
