# `three-exclusions/` — AC-433's fixture

Everything `two-activities/` carries, plus **three catalog activities that are in the catalog and
NOT in `dag_assembled.nodes`**, one per exclusion kind AC-433's THEN names:

| activity | excluded by |
| --- | --- |
| `excluded-predicate` | a false `applicable_if` sub-predicate — `market_facing` is `false` |
| `excluded-rigor` | `min_rigor = "mandatory"` against the stage's `recommended` |
| `excluded-norow` | its `provides` kind is named by no `baseline` entry and no fired `require` row |

## 🛑 The excluding clause is NOT on the record, and that is why AC-433 is PROVISIONAL

`dag_assembled.skipped[].reason` is only ever `forbidden_stage` or `forbidden_stage_via_requires`
(`SkipReason` has exactly two stage variants, measured), so **`skipped` does not carry these three**
and is empty here. The three clause kinds are printed by re-reading recorded inputs — the recorded
`applicable_if` through `rk_kernel::predicate::{parse, evaluate}` over the recorded `context.json`,
the recorded `min_rigor` against the recorded pack's rigor, the recorded `require` rows — and that
re-reading is **disclosed as a re-reading, not as a recorded exclusion**. The id is discharged whole
only once a kernel-recorded `excluded` row lands.

`predicate::on_false` is **not** the sub-predicate namer: it is ADR-0103 clause 6's closed form set
(`OnFalse{Continue,Fail,LoopBack,Deviate}`) plus `classify_on_false`. §7d.7's prose is wrong there
and is RAISED.

Every digest here is a sha256 over a label naming what it stands for. **No corpus byte.**
