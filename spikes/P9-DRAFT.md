## P9 — RESULT: the activity-graph schema authors at a median 56 lines, the lint's first run caught 4 design defects the author had not seen, and the `adversary → policy → gate` fragment sits exactly at the kill threshold

**Established by:** authoring 10 activities + 3 registries under `spikes/p9/`, one lint prototype (`spikes/p9/lint.py`, 311 lines Python), two lint runs, 2026-09-07 20:32–20:39 UTC. Pre-registered in `METHOD-P9.md`. **Zero model dispatches.** Raw outputs: `spikes/p9/results-run1.txt` (first run, unfixed lint), `results-run1b-lintfixed.txt` (same catalog, lint bugs fixed), `results-run2.txt` (catalog fixed), `lines.txt`. F-number to be assigned on promotion to `FINDINGS.md`.

### Metrics against the kill criteria

| metric | value | kill | verdict |
| --- | --- | --- | --- |
| non-blank lines per activity | **median 56.5** (range 40–102; sum 616) | > 150 | **pass, 2.7× margin** |
| steps per activity | median 5.5 (3–11) | — | — |
| largest fragment repeat | **5 of 10** (`check↺ → gate?`); the semantic `adversary → policy (→ gate)` block is in **6 of 10** by intent, 3 of 10 by exact shape | > 5 | **at threshold — not a kill, but real** (see §3) |
| lint first-run findings | 47 errors, 11 warnings | — | 13 real catches, 34 false alarms (§2) |
| registry overhead | 151 lines (facets 59, questions 30, schemas 62) | — | amortises to ~15 lines/activity at n=10, ~2 at n=95 |
| authoring wall-clock | ~4 min for 10 files | — | the schema's author writing his own schema; disclosed in METHOD-P9 §6, not a usable number |

Projection: 95 activities at the observed mean of 62 lines ≈ 5.9k lines of TOML plus ~300 lines of registry. The ten were chosen for step-kind coverage, so they over-represent the complex end; the median for the full catalog is likely lower.

### 1. What the lines are spent on

Of the 616 lines, 170 (28%) are the `[activity]` header — 15–17 keys, identical shape in every file. Steps average 8 lines. The header could shed `derived_from` (defaults to `requires`), `deviation_ok` (defaults to `[]`), `keywords` and `version` (defaults to 1) without loss; that is ~4 lines per activity. Not worth optimising before P7 says which fields are actually consulted.

### 2. The lint: 13 real catches from 9 distinct mistakes, 4 of them design-level

The first run raised 47 errors. Adjudicated on inspection (every false alarm named, per METHOD-P9 §3):

| bucket | count | what |
| --- | --- | --- |
| **false alarms from lint bugs** | 34 | R9 flagged field names inside dotted paths and quantifier scopes (15); R1 treated `repeat.until`'s self-reference as a cycle (2); R1 ordered "feedback" refs by an arbitrary topological position instead of declaration order (13); R3/R5 treated every `gate` as conditional even without `when` (4) |
| **real: design defects the author did not see** | 4 | `canary-release`: `ReleaseRecord` bound to `deploy-full?` — unreachable on the rollback path, i.e. the activity had no artifact when it mattered most; `minimal-repro`: `ReproCase` bound to a `rigor >= recommended` step — no artifact at all at `optional`; `threat-model`: `model` consumed `${review.outputs?}` as feedback but `policy.on_false` jumped forward to the residual gate, so review findings were never fed back (dead input); `tdd`: `red-verify` read `tests_added` off a `for_each` step whose outputs are a list |
| **real: registry / syntax** | 9 | unknown output schema `Workspace`; budget `wall_clock_secs` given as a substitution; `for_each` on an artifact written as `${spec…}` (2 findings, 1 cause); evidence promises that did not match the graph — `SelfCheck` and `HumanAnswer` promised unconditionally but produced by conditional steps (2), and a redundant `when = "rigor >= optional"` that made `self-check` conditional and tripped three findings (3, 1 cause) |

After fixing the four lint bugs the same catalog produced exactly the 13 real errors and nothing else; after fixing the catalog, zero errors. The nine warnings that remain are all **schema findings**, not authoring mistakes (§4).

What the lint could *not* check, listed so "13 catches" is not over-read: output *types* (an `emit` binding a `ReproAttempt` where a `ReproCase` is promised passes), predicate semantics beyond vocabulary, that a tool verb exists in any tech pack, that `kb:`/`checklist:` sources exist, and anything about runtime behaviour.

### 3. The fragment question — at the threshold, and the answer is a shorthand, not an include mechanism

Exact-shape repeats (kind / role / run / conditional / loop / fan-out per step):

| fragment | activities |
| --- | --- |
| `check↺ → gate?` | 5: adversarial-review, canary-release, root-cause, threat-model, user-story-ac |
| `check↺ → gate? → emit` | 4 |
| `agent/reviewer? → check↺` | 3: root-cause, user-story-ac, vertical-slicing |
| `agent/reviewer → check↺` (unconditional reviewer) | 2: adversarial-review, threat-model |
| `agent/investigator↺ → gate? → tool/test` (loop-then-escalate) | 2: minimal-repro, root-cause |

By intent the **adversary → policy → (gate)** quality block is in 6 of 10; it splits into two exact shapes only because `min_rigor` decides whether the reviewer step carries `when`. The kill criterion was "> 5"; the honest reading is that the block is the default quality mechanism of PC6 and will appear in most agent-producing activities. Two options were considered:

- **General `include = "fragment:…"` with parameters** (schema open Q1) — SPEM's overlay algebra. Rejected for now: it is what made EPF content unreadable, and the repeats here are one block, not many.
- **A `quality` shorthand on the activity** — `quality = { adversary = "reviewer", policy = "findings-default", gate = "mandatory" }` expands to the three steps at load time, is visible in `redkiln explain` as ordinary steps, and can be overridden by writing them out. **Recommended**, but its exact defaults depend on P7 (whether `adversary` at `recommended` earns its cost). Also worth a shorthand: the `loop-then-escalate` pair — `on_exhausted = "escalate"` should *imply* the gate step instead of requiring one to be declared with a `when` on the previous step's status (3 of 10 wrote that pair by hand).

So: **no kill; adopt the two shorthands after P7; do not build includes.**

### 4. Schema amendments the authoring surfaced (the real payload)

Numbered for the schema note; each was hit while writing, not theorised.

1. **Branching is unspecified.** `on_false` was used as a *forward branch* in 3 of 10 (`verdict → override`, `healthy → degraded`, `policy → residual-gate`) and the schema only defines loop-back. Amend: `on_false ∈ { <earlier step>, continue, fail }`; forward branching is expressed by `when` on the downstream step (canary's `degraded` already did this correctly), and the lint rejects forward targets.
2. **Absent values need semantics.** `policy` at `optional` rigor evaluates `findings.none(…)` over an absent `?` input (R4 warning). Define: an absent `?` input is `null`; quantifiers over `null` are vacuously true; `null == null`; anything else on `null` is a lint error.
3. **The predicate grammar must be written down.** Roots (inputs, facets, `rigor`, `item`), field access, `len()`, quantifiers `none/all/any(<field> in […])` with the element's fields in scope, comparisons, `and/or/not`, `in`. The 15 R9 false alarms were the lint guessing at this.
4. **Coalesce.** `minimal-repro` needed "the reduced case if `reduce` ran, else the raw attempt"; the fix was to make `reduce` unconditional, which defeats the rigor variant. Add `??` in refs: `${reduce.outputs?} ?? ${attempt.outputs}`.
5. **Fan-out plumbing.** `for_each` over an artifact field (`artifact:Spec.acceptance_criteria`) is needed and not in the schema; and a fan-out step's list outputs need a projection to feed a scalar-taking step (`${red.outputs[*].tests_added}`), otherwise every consumer tool must accept lists.
6. **Evidence minting must be normative.** The lint had to invent the table "tool `toolchain:test` mints `ToolReport` + `GreenAt`; agent with `Findings` output mints `Findings`; gate mints `HumanAnswer`; `MutationReport` mints `Mutation`". Put that table in schema §2; the promise list is otherwise unlintable.
7. **Rule 8 (budget ≥ worst-case attempts) is mis-specified.** It warned on 6 of 10 because worst-case retries always exceed a sensible appetite; retries are weather, not plan. Replace with `attempts_total ≥ number of agent/tool steps on one pass`.
8. **`escalate.options` should not be repeatable.** The registry owns options (schema example shows them inline; lint warns). Amend the example.
9. **Rigor-conditional chains repeat `when`.** `tdd` carries `when = "rigor >= recommended"` on three consecutive steps. Tolerable at 10; consider `after = "<step>"` implying the same `when` if it recurs at 95.
10. **Input-source vocabulary.** `artifact:Kind[.field]`, `kb:atom`, `checklist:id`, `toolchain:logs?`, `${…}`, literals — used, undefined. Define, and let the lint check `kb:`/`checklist:` against registries.
11. **Alternative outcomes.** A canary that rolls back still "completes"; the only way to say so was an extra `record` tool step. Whether that is the right pattern or activities need an `outcome` enum on `Materialized` is a P7-adjacent design question; the tool step works and keeps the exit types to three.

### 5. Verdict

The schema clears the authoring-cost bar by a wide margin, and the lint earned its keep on the first run — four of the nine mistakes were design defects that would have shipped as activities with no artifact on their most important path. The fragment threshold was met exactly, which argues for a `quality` shorthand rather than an include mechanism, gated on P7. Eleven schema amendments came out of writing ten files; that is the number to expect to fall, not rise, as the next batch is authored against the amended schema. **PC5 (curated ~95 at v0) stands.** Schema open Q1 (sub-graph reuse) resolves to "shorthands, not includes".

### Limits

1. The author of the schema authored the activities and adjudicated the false alarms. A second author is the only way to get an ergonomics number that means anything; the lines metric is a lower bound on cost for anyone else.
2. The lint is Python over a stand-in type registry. Rules 3 (types) and 7 (list-ness) will be stricter under `schemars`; the 13/34 split is a property of this prototype, not of the rules.
3. Nothing ran. Whether `root-cause` at `mandatory` is worth its steps is P7; whether a `gate` mid-graph resumes cleanly is P8.
4. The ten over-represent complex graphs by design; medians will move down, not up, with the full catalog.
5. Fragment detection is exact-shape over consecutive declared steps; the "6 of 10 by intent" figure is my reading, not the tool's.
