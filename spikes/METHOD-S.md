# Pre-registration #5 — S: the spikes that gate Phase 1 of Redkiln-RS

**Status:** new pre-registration, per charter §8.2. **Written before any S
dispatch. No S data has been seen.**
**Baseline:** this document's commit. **Corpus pin:** redkiln @ `47faec9f`.
**Instrument:** `spikes/runner` (E3 rig, extended in the same commit).
**Why it exists:** `references/2026-09-05-architecture-proposal.md` §8 names
four spikes as gating Phase 1. Each feeds a decision already taken (D4, D6, D8)
whose *parameters* are not yet known, or validates a type that Phase 1 would
otherwise design blind. They are registered together because they share the
rig, the corpus and the discipline; they run and report separately.

| spike | question | feeds | model cost |
| --- | --- | --- | --- |
| **S-VAR** | how unstable is the review verdict, and is the instability predictable from the defect's evidence class? | D4 policy thresholds | ~60 Opus dispatches |
| **S-TYPED** | does demanding typed `Findings` (closed evidence enum, no `pass`) cost detection or raise false positives against the prose digest? | D4, D6, §4.4 schema rules | 46 Opus dispatches |
| **S-BIN** | can the reviewer verify an item whose change is not a text diff? | D8 `ChangeSummary`, Toolchain port | ~10 Opus dispatches + UE automation runs |
| **S-MIG** | what fraction of the frozen corpus lifts into the proposed types under layout C without judgement? | D6, store schema | zero |

---

## 0. Rules inherited from METHOD, METHOD-E3 and METHOD-H

Isolation (`--strict-mcp-config`, adoption verified by tool-set equality),
pre-flight checks recorded on every record (`card_adopted`,
`node_modules_linked`, `fix_unreachable` where applicable), a void is never a
miss, resume keyed per `(arm, slug, repeat)`, results append-only outside the
fixture tree, platform errors named as such (`api_error_status`), throttle
margins honoured, and **nothing is inferred from a single draw** (F27). Every
metric below is either mechanical or is reported with the judgement it needs
named.

**Two new instrument rules, disclosed here:**

1. **Repeats of a stochastic process are reported per item as a rate with a
   Wilson 95% interval**, never pooled across items first. F27 showed the flip
   rate is an item property.
2. **A typed schema is validated at the boundary before scoring.** A record whose
   digest fails the schema is `void: schema`, not a miss — the same rule F20
   applied to an absent digest.

---

## 1. S-VAR — verdict stability at n=10

### 1.1 Why it is needed

F27 measured verdict stability on 4 CLEAN items at n=2–3: two flipped. A
"stable" classification at that n is compatible with a true flip rate up to
~30%; an "unstable" one says nothing about *how* unstable. D4 moves the blocking
decision into a Rust policy, but the policy's thresholds — which evidence
classes block — need a measured relationship between evidence class and
stability that F27 could only hypothesise: *strength of the underlying defect
predicts verdict stability* (n=4).

### 1.2 Items and their known evidence class

Six CLEAN items, every one already adjudicated in F24 so its evidence class is
**known in advance and is not a judgement made here**:

| item | known finding (F24) | evidence class | F27 draws |
| --- | --- | --- | --- |
| `fix-advance-write-lock-gate-scope` | `basis` flips `evaluated→memoized`; entry gate stops rendering `produces` — reproduced end to end | **Reproduced** | REJECT / REJECT |
| `issue-146-147-boundary-window` | AC-015 ordering assertion vacuous (36/36 green inverted) | **MutationSurvived** | REJECT / approve / REJECT |
| `issues-135-129-declared-boundary-parse` | `Path`-header guard unpinned (63/63 green disabled) | **MutationSurvived** | unmeasured |
| `issue-74-plan-briefs-dead-plan-anchor` | `=== true` strictness unpinned (28/28 green relaxed) | **MutationSurvived** | unmeasured |
| `issue-63-advance-gate-diagnostic-truncated` | AC-001 pin narrower than the AC | **NarrowedPin** | approve / REJECT |
| `issue-62-new-stale-pinned-template` | none found by any arm | **none** (control) | approve / approve |

### 1.3 Procedure

`review-probe --condition clean --arm b --repeat r --only <slug>` for r = 1..10,
prose digest schema (E3's), same card, same model/effort (Opus/xhigh), same
fixture construction, into `results/s-var.jsonl`. F27's existing draws in
`e0-stability.jsonl` are **not** merged in; they were taken under a different
pre-registration and a merged n would be a post-hoc choice.

### 1.4 Metrics

**Primary — per-item verdict flip rate**: for each item, the fraction of draws
returning `changes-requested`, with its Wilson 95% interval. An item is
*stable-reject* if the lower bound exceeds 0.55, *stable-approve* if the upper
bound is below 0.45, *flaky* otherwise. (Calibrated to n=10 before any data:
10/10 → [0.72, 1.00] and 9/10 → [0.60, 0.98] are stable-reject; 8/10 →
[0.49, 0.94] is flaky; 0/10 → [0.00, 0.28] and 1/10 → [0.02, 0.40] are
stable-approve; 2/10 → [0.06, 0.51] is flaky. An earlier draft used 0.8/0.2,
which no n=10 result can reach — corrected on the smoke run of the audit
script, with no S data in existence.)

**Secondary — evidence class vs stability**: the ordering of flip rates by
class. Predicted (before data): Reproduced ≥ MutationSurvived > NarrowedPin,
with the control near 0.

**Recorded**: whether each rejection cites the F24 finding's file (mechanical,
as in E3 strong detection); cost and wall clock per draw.

### 1.5 Decision rule — what this sets in D4's policy

- If `Reproduced` and `MutationSurvived` items are stable-reject and
  `NarrowedPin` is flaky: the proposal's default policy stands
  (`Reproduced`/`MutationSurvived` block; `NarrowedPin` blocks only under a
  strict stage weight).
- If `MutationSurvived` items are flaky: the *finding* is reliable but the
  *reviewer's own threshold* is not, and the policy must block on the finding's
  presence regardless of the reviewer's `pass` — which is exactly what
  S-TYPED's schema does. This outcome strengthens D4.
- If `Reproduced` is flaky (the item with a live behavioural regression is
  approved in ≥2 of 10): the review gate cannot be trusted as a sole blocker for
  any class, and Phase 1 must pair it with the deterministic gate for every
  required AC. **This is the outcome that changes the architecture.**
- If the control rejects in ≥2 of 10: the false-positive floor is real and the
  policy needs an `Advisory` sink; reported, not a redesign.

### 1.6 Prediction, before the data

1. `fix-advance-write-lock-gate-scope` is stable-reject (10/10 or 9/10).
2. At least one `MutationSurvived` item is flaky; `issue-146-147` flips at
   0.4–0.8.
3. `issue-63` (NarrowedPin) is flaky, in the 0.3–0.7 band.
4. `issue-62` approves ≥9/10.

### 1.7 Sizing and threats

6 items × 10 draws = **60 dispatches**, CLEAN-shaped (~700–2000 s, $1–5 each):
budget ~$150 and three five-hour windows at the measured +0.077 per CLEAN
dispatch (F21). Resume keyed per repeat, so a killed window costs one dispatch.

Threats: one arm only (b), so nothing is claimed about arm (a); the six items
have each been reviewed by this model before (repetition confound, affects the
absolute rate, not the ordering); n=10 gives ±0.3 intervals at worst, enough to
separate the bands in §1.4 but not to estimate a rate finely.

---

## 2. S-TYPED — typed findings versus the prose digest

### 2.1 Why it is needed

D6 makes every activity output a typed value and D4 removes `pass` from the
reviewer's output entirely: the model reports **findings with an evidence
class**, and Rust computes the verdict. Neither has been measured. The risk is
specific: a closed schema with no verdict field may change *what the reviewer
looks for* — it could stop searching once it has classified something, or
classify a real defect as `Advisory` to avoid committing — and that would show
up as lower detection or a different false-positive profile than E3's.

### 2.2 The typed schema (the instrument)

```json
{
  "type": "object", "additionalProperties": false,
  "required": ["findings", "boundary_respected", "rubric"],
  "properties": {
    "findings": { "type": "array", "items": {
      "type": "object", "additionalProperties": false,
      "required": ["ac", "claim", "evidence", "files", "repro"],
      "properties": {
        "ac":       { "type": "string" },
        "claim":    { "type": "string" },
        "evidence": { "type": "string", "enum": ["reproduced", "mutation-survived", "narrowed-pin", "advisory"] },
        "files":    { "type": "array", "items": { "type": "string" } },
        "repro":    { "type": "string" }
      }}},
    "boundary_respected": { "type": "boolean" },
    "rubric": { …E3's five dimensions, unchanged… }
  }
}
```

**There is no `pass`.** The prompt tells the reviewer this and defines the four
classes (§2.3). Everything else — card, tools, model, effort, fixtures — is E3's.

**The policy (Rust, `e3::policy_verdict`)** — registered here so it cannot be
tuned to the data:

| evidence | strict policy | lenient policy |
| --- | --- | --- |
| `reproduced` | blocks | blocks |
| `mutation-survived` | blocks | blocks |
| `narrowed-pin` | blocks | advisory |
| `advisory` | never | never |

**Strict is primary.** Lenient is reported alongside because D4 makes the
choice a stage-weight parameter and both ends need a number.

### 2.3 Prompt deviation, disclosed

E3's prompt is used verbatim with one appended paragraph that (a) says the
digest carries no verdict and the verdict is computed from the findings, and
(b) defines the four evidence classes operationally:

- `reproduced` — you ran the behaviour and observed the AC violated;
- `mutation-survived` — you changed the behaviour the AC describes and the
  relevant test stayed green;
- `narrowed-pin` — a test exists but pins a narrower claim than the AC;
- `advisory` — a quality concern that does not show an AC unmet.

This is the operational form of the card's own question ("is there a test that
would fail if the behaviour regressed?", F25 remediation 3). It is a prompt
change and is confounded with the schema change by design: D4/D6 ship them
together, so the spike measures the pair.

### 2.4 Conditions and arms

| condition | n | typed arm | prose comparator |
| --- | --- | --- | --- |
| S-SILENT | 23 | new dispatches, `b-typed-s-silent` | E3's `b-s-silent` records, repeat 1 (F23) |
| CLEAN | 23 | new dispatches, `b-typed-clean` | E3's `b-clean` records, repeat 1 (F23) |

The prose arm is **not re-run**. Detection on S-SILENT was stable across 21/21
repeat draws (F27), so reusing the recorded prose arm loses nothing there; on
CLEAN the recorded arm is a single draw of a flaky process, and the comparison
is stated with that limit rather than paid for twice.

### 2.5 Metrics

**Primary — S-SILENT strong detection under the strict policy**: the policy
blocks **and** a blocking finding names a file the revert touched. Compared with
E3's 23/23.

**Co-primary — CLEAN false-positive rate under the strict policy**, and under
lenient, against E3's 2/23 (F23) and the 0/23 upper bound after adjudication
(F24).

**Recorded**: schema conformance rate (a `void: schema` is an instrument
finding about the model's ability to emit closed enums); evidence-class
distribution on CLEAN (are the adjudicated F24 findings re-found *and*
classified as F24 classified them — mechanical against §1.2); cost and wall
clock versus the prose arm; whether findings on S-SILENT are classified
`mutation-survived`/`reproduced` (the reviewer did the work) or `advisory` (it
did not commit).

### 2.6 Decision rule

- Detection ≥ 22/23 and strict-policy FP ≤ 2/23: **D4 and D6 stand as
  written**; Phase 1 ships the typed schema.
- Detection ≤ 20/23 (three or more misses the prose arm did not make): the
  typed contract costs detection; Phase 1 keeps `pass` **as a recorded field
  that the policy ignores** (so the reviewer still commits), and the schema is
  re-spiked with that change before D6 is applied to review.
- Strict-policy FP ≥ 5/23 with the excess classified `advisory`-adjacent
  (`narrowed-pin`): the strict policy is too strict for a default; lenient
  becomes the default stage weight. FP excess classified `mutation-survived` is
  adjudicated by mutation as in F24 before it counts.
- Schema conformance < 90%: the enum shape is wrong for the model; §4.4's
  schema rules gain "no closed enum without a free-text `other`" and the spike
  is re-run.

### 2.7 Prediction, before the data

1. Detection stays at ceiling (23/23 or 22/23). The classes are the reviewer's
   own behaviour named.
2. CLEAN findings are re-found on the five F24 items and classified as F24
   did, ≥4 of 5.
3. Strict FP lands 3–5/23 because the reviewer, freed from committing to
   `pass`, reports more `narrowed-pin`; lenient lands at or below E3's 2/23.
4. Schema conformance ≥ 95%.
5. Cost is within ±15% of the prose arm.

### 2.8 Sizing and threats

**46 dispatches** (23 S-SILENT at ~$0.5, 23 CLEAN at ~$1–5): budget ~$80. The
prose comparator is a single earlier draw on CLEAN; a difference of one or two
items is inside F27's measured variance and is reported as such. The prompt and
schema change together, so the spike cannot attribute an effect to one of them.

---

## 3. S-BIN — review without a diff

### 3.1 Why it is needed

D8 designs `ChangeSummary` against a real Unreal project and every review
measurement in the programme assumed a text diff exists. Binary assets
(`.uasset`, `.umap`) cannot be diffed; the reviewer receives a *summary* — asset
manifest, build log, automation-test report, captures — and must verify ACs
from that plus whatever it can run. Whether an adversarial reviewer can catch a
seeded regression under those conditions is unmeasured, and the type Phase 1
ships depends on the answer.

### 3.2 Fixture

Ryan's UE project (D8), at a pinned commit. Five items are constructed, each
with a `spec.md` of 3–6 ACs and an implementation report, exactly as the fix
lane would produce them:

| # | shape | ground truth |
| --- | --- | --- |
| 1–3 | CLEAN: the change as made, automation tests green | approved |
| 4–5 | SEEDED: one asset or one line of Blueprint/C++ altered so an AC is unmet while the automation suite stays green (verified before dispatch, as E3-H seeds were) | changes-requested |

Seeds are chosen and their greenness verified **before** the spike runs; the
seed list is committed with this document's amendment once the project is
pinned, and no seed is changed after a dispatch.

### 3.3 The `ChangeSummary` under test

```
ChangeSummary {
  range: (base_sha, head_sha),
  text_diff: Option<Patch>,                       // C++/Blueprint text where it exists
  assets: [{ path, kind, action: Added|Modified|Deleted, size, lfs_oid }],
  build: { command, exit, log_ref },
  tests: { command, report_ref, passed, failed, skipped },   // -ReportExportPath JSON
  captures: [{ path, viewport, note }],
}
```

Rendered into the prompt in place of E3's "files this change was scoped to
touch" list and spec/report artifacts; the reviewer has `Read` and `Bash` in a
disposable clone with the engine available, so it may run
`UnrealEditor-Cmd … -ExecCmds="Automation RunTests …" -NullRHI` itself.

### 3.4 Metrics and decision rule

**Primary**: strong detection on the two seeded items (blocking finding names
the seeded asset or file) and approval on the three clean ones, under the typed
schema and strict policy from §2.

**Recorded**: which `ChangeSummary` fields the reviewer's findings cite
(mechanical — file paths and field names); wall clock (UE automation runs are
minutes each); whether the reviewer ran the automation suite itself.

**Decision rule** — a *type* decision, not a kill:

- 2/2 detection and ≤1/3 false positive: `ChangeSummary` as drafted is
  sufficient; ship it.
- A seed missed and its finding would have needed a field the summary lacks
  (named in the reviewer's own prose): add the field, re-run that item.
- Both seeds missed with the summary fields cited but not the asset: binary
  review needs a *rendered* representation (captures with diffs, asset
  property dumps), which is Toolchain-adapter work in Phase 2 and is recorded
  as a requirement.

### 3.5 Prediction, before the data

Detection 1/2 or 2/2; the reviewer runs the automation suite unprompted (it
ran vitest unprompted in every E3 CLEAN dispatch); the field it cites most is
`tests.report_ref`, not `assets`.

### 3.6 Sizing and threats

5 items × 1 arm = 5 dispatches, repeated twice for a stability read = **10
dispatches**, ~$30 plus UE automation wall clock. n=5 is a type-design sample,
not a rate; nothing here generalises beyond "the type carries what one reviewer
used". The seeds are author-chosen (same threat as E3-H §8.2).

---

## 4. S-MIG — the typed census of the frozen corpus

### 4.1 Why it is needed

D5/D6 commit to typed frontmatter + rendered structured sections + opaque prose,
with sidecars for machine-only artifacts. The proposal names the opaque
sections as "settled by S-MIG's census of what cannot be typed". Until that
census exists, the store schema and `redkiln-migrate` are guesses.

### 4.2 Procedure — zero model cost

A deterministic tool (`redkiln-migrate census`, or a script in `spikes/`)
walks every artifact under `.bklg/` and `.kb/` at `47faec9f` (1,251 markdown
files under `.bklg/` per RFC §3.5, plus `.kb/` and `.redkiln/processes/`) and,
per artifact kind, attempts to lift each section into the proposed type:

| artifact | proposed typed sections | proposed opaque sections |
| --- | --- | --- |
| `spec.md` | frontmatter; `AC-###` Given/When/Then + `_Verification:_`; `EC-###`; `NF-###`; Frozen PR boundary; Merge gate; Anchors table | summary; Context pack; Repro narrative; Behavior prose; Out of scope |
| `implementation-report.md` | TDD Evidence table (per-AC rows); files touched; gate result | narrative |
| `_review.md` | verdict; rubric; findings → sidecar entirely | — |
| `_ledger.md`, `_plan.md`, `_storymap.md`, `_integration.md` | sidecar entirely | — |
| `story.md`, `project.md`, `initiative.md` | frontmatter; status; links | body prose |
| `.kb` atoms | frontmatter (KbFrontmatter); links | body |

For each (artifact, section): `Lifted` (parsed into the type with no loss),
`LiftedWithLoss { what }` (parsed, but some content had nowhere to go),
`Opaque` (kept as prose by design), or `Failed { why }`.

### 4.3 Metrics and decision rule

**Primary**: per artifact kind, the share of sections `Lifted` or `Opaque`
by design, versus `LiftedWithLoss` or `Failed`. The census output (counts and
the list of `Failed` with reasons) is the deliverable.

**Decision rule**:

- ≥ 95% of structured sections `Lifted` across every kind: layout C and the
  proposed section split stand; the `Failed` list becomes the migration tool's
  fixture set.
- A section kind with < 80% `Lifted`: that section is reclassified as opaque
  in Phase 1 (the type is deferred, not forced), and the reason is recorded in
  the store schema's ADR.
- Any `LiftedWithLoss` whose lost content is decision-bearing (a verification
  note, a boundary path): the type gains a field; no loss of that class is
  accepted in migration.

### 4.4 Prediction, before the data

ACs lift ≥ 95% (the Given/When/Then grammar was enforced by the old validators);
the Anchors table and Frozen PR boundary lift ≥ 90%; the TDD Evidence table
lifts < 80% because its columns changed across the corpus's history (F25
remediation added columns); `.kb` bodies are opaque by design and lift 100% as
such.

### 4.5 Threats

The census measures the *old* corpus's regularity, which the old validators
enforced; a new corpus without those validators may be less regular, and that
is precisely why the new design renders rather than parses. It cannot measure
how humans will edit under D5.

---

## 5. Sequencing

S-MIG first (free, and it settles D6's section list before any type is
written). S-TYPED and S-VAR next, interleaved across five-hour windows; they
share fixtures. S-BIN when the UE project is pinned and its seeds verified;
the seed list is added to this document as a dated amendment before dispatch.

## 6. How to run

```
cd spikes/runner && cargo build          # cargo test does NOT rebuild the bin (HANDOFF)
# S-VAR — one item, ten draws, prose schema (E3's), into its own file
for r in 1..10: review-probe --condition clean --arm b --only <slug> --repeat $r --out ../results/s-var.jsonl
python results/audit-s.py var results/s-var.jsonl

# S-TYPED — typed schema, both conditions, prose comparator is e3.jsonl
review-probe --condition s-silent --arm b --schema typed --out ../results/s-typed.jsonl
review-probe --condition clean    --arm b --schema typed --out ../results/s-typed.jsonl
python results/audit-s.py typed results/s-typed.jsonl results/e3.jsonl
```

F22 applies: launch long sets detached (`Start-Process`), never as a harness
background job; resume is keyed per `(arm, slug, repeat)` so a kill costs one
dispatch. Arm labels carry every instrument variant (`b-typed-clean`,
`b-blind-typed-s-subtle`) so an audit keyed on `arm` can never pool them.

## 7. Amendments

- **2026-09-05, before any dispatch:** §1.4 stability bands changed from
  0.8/0.2 to 0.55/0.45 on Wilson bounds, because at n=10 no result could reach
  the original thresholds. Found by the audit script's smoke test on
  synthetic records; no S data existed.

Any later change is disclosed here with the date and the records it post-dates.
