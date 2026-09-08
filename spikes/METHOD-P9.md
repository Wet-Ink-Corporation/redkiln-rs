# Pre-registration #6 — P9: authoring ergonomics of the activity-graph schema

**Status:** pre-registration, per charter §8.2. **Written before any activity is authored.**
**Baseline:** this document's commit. **Model cost:** zero — P9 is authoring plus a deterministic lint.
**Why it exists:** `references/2026-09-07-process-catalog-proposal.md` §6.1 names P9 as one of three spikes that can overturn a taken decision: PC5 (ship ~95 activities at v0) and the activity-graph schema's open question 1 (sub-graph reuse). The failure it guards against is EPF's: content whose authoring cost exceeded its consumption value rotted and was archived (method-engineering §1). If ten representative activities cannot be written compactly and linted usefully, ninety-five will not be.

## 1. Question

Is `references/2026-09-07-activity-graph-schema.md` cheap enough to author, and does its lint catch real authoring mistakes, at the scale PC5 commits to?

## 2. Protocol

1. Author a facet registry (`facets.toml`, PC8), a question registry (`questions.toml`, schema open Q2), a minimal output-schema registry (`schemas.toml` — field names and list-ness only, standing in for the `schemars` types) and **ten** activities, chosen to cover every stage family and every step kind:

   | activity | stage | why chosen |
   | --- | --- | --- |
   | `minimal-repro` | analysis | tool-heavy; `has_repro` facet; bug/vuln/incident |
   | `root-cause` | analysis | the schema's worked example; loop + escalate + gate |
   | `competitor-analysis` | discovery | simplest possible graph; human-source artifact (PC3) |
   | `user-story-ac` | specification | agent + self-check; ordered-method family |
   | `threat-model` | design | risk-facet gated (`auth`); mandatory-only steps |
   | `vertical-slicing` | planning | produces a list consumed by `for_each` downstream |
   | `tdd-red-green-refactor` | implement | Red→Green loop; tool steps; toolchain substitution (PC7) |
   | `mutation-testing` | verify | pure tool graph; per-AC fan-out |
   | `adversarial-review` | review | findings-only + policy check (§4.6); item-level, not activity-level |
   | `canary-release` | release | gate-heavy; escalation on metrics; rollback branch |

2. Author each activity **once, without looking at the lint** until all ten are written; then run the lint and record every error it raises on that first run. Fix, re-run, record again. The first-run count is the honest "catches" number; anything I fix silently before the first run does not count.
3. Build the lint (`lint.py`, Python prototype standing in for the Rust implementation) implementing schema §9 rules 1–9 plus a **fragment detector**: two step sequences in different activities are the same fragment if, after renaming step ids, their `kind / role / run / when / repeat / on_false` tuples match.
4. Measure.

## 3. Metrics (all mechanical)

| metric | how |
| --- | --- |
| lines per activity | `wc -l`, excluding blank lines and comment-only lines |
| steps per activity | count of `[[step]]` |
| lint catches on first run | errors raised, by rule number, before any fix |
| lint false alarms | first-run errors I judge wrong on inspection, named |
| fragment repeats | for each fragment of ≥2 steps, the number of activities containing it |
| registry overhead | lines in `facets.toml` + `questions.toml` + `schemas.toml` |
| authoring wall-clock | timestamps of first and last catalog file write |

## 4. Kill criteria (registered in the proposal §6.1)

- **Median > 150 non-blank lines per activity** → the schema is too verbose; revise before authoring the other 85.
- **Any fragment of ≥2 steps repeated in > 5 of the 10 activities** → add `include` fragments (schema open Q1) before the other 85.
- Secondary, not a kill: **lint first-run catches = 0** means either the schema is trivially easy or the lint is toothless; report which by listing what the lint *could not* check.

## 5. What P9 cannot tell us

Whether the graphs *run well* — that is P7/P8. Whether ten is representative of ninety-five — chosen for coverage of step kinds and stage families, not for typicality; the lines metric is therefore likely an over-estimate (the ten include the most complex graphs deliberately). Whether a Rust implementation of the lint has the same catches — the prototype is a specification of the rules, not the implementation.

## 6. Limits disclosed in advance

- The author of the schema is the author of the activities. Ergonomics measured on the schema's author are a lower bound on cost for anyone else.
- `schemas.toml` is a stand-in; real `schemars` types will make rule 7 (list-typing) exact and may add catches this run cannot.
- Predicates are checked for vocabulary (rule 9) and for `rigor` evaluation (rule 4), not for full semantics.
