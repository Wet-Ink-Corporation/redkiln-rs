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
~30% *(corrected 2026-09-11 — up to ~56% at n=3, ~66% at n=2; see §7)*; an "unstable" one says nothing about *how* unstable. D4 moves the blocking
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

> **Amended 2026-09-06, before any S-TYPED dispatch** — see §7: `rationale`
> added per finding; `repro` is now a structured object; the schema below is
> the original registration and the amendment records the delta.

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

> **Amended 2026-09-06, before any S-TYPED dispatch** — a third condition,
> S-SUBTLE (the five E3-H seeds, typed arm only, comparator E3-H arm b sighted),
> is added as the negative control S-VAR lacked. See §7.

| condition | n | typed arm | prose comparator |
| --- | --- | --- | --- |
| S-SILENT | 23 | new dispatches, `b-typed-s-silent` | E3's `b-s-silent` records, repeat 1 (F23) |
| CLEAN | 23 | new dispatches, `b-typed-clean` | E3's `b-clean` records, repeat 1 (F23) |

The prose arm is **not re-run**. Detection on S-SILENT was stable across 21/21
repeat draws (F27), so reusing the recorded prose arm loses nothing there; on
CLEAN the recorded arm is a single draw of a flaky process, and the comparison
is stated with that limit rather than paid for twice.

### 2.5 Metrics

> **Amended 2026-09-06, before any S-TYPED dispatch** — a registered secondary,
> the *contradiction rate*, is added; see §7.

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

> **Amended 2026-09-09, before any S-BIN dispatch** — see §7: the project is
> named and pinned, the five items are drawn from its real history rather than
> constructed, and the two seeds are produced by a single `git checkout` of the
> pre-fix revision rather than by hand-authoring an asset. The registration
> below is unchanged and the amendment records the delta. **The greenness
> verification §3.2 requires is still outstanding and is the remaining
> precondition to dispatch.**

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

> **Amended 2026-09-09, before any S-BIN dispatch** — see §7. The draft below
> is the original registration and is unchanged. Reading the pinned fixture's
> own instruments found three places where it does not fit what the project
> actually emits: `text_diff` cannot be decided by "did git produce a diff"
> (under LFS it always does, and the diff is a two-line pointer), `tests`
> has no field for `succeededWithWarnings`, which is **180 of 1029** on the
> project's own last recorded run, and `captures` does have a producer after
> all. Those are inputs to the spike, not edits to the type: **`ChangeSummary`
> stays BLOCKED and its field set is not fixed here.**

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

## 4a. S-H1b — does a static system prompt remove the gate's resume cost?

**Added 2026-09-05, before any S-H1b dispatch; S-VAR was running, no S-H1b
data existed.** Registered in the proposal §8; not previously in this document.

### 4a.1 Why

F28 (H1): resume after a gate costs 0 below the TTL and a flat 18,603 tokens
above it — the volatile per-directory segment (F13). F16 showed
`--system-prompt` deletes the volatile git-status section and holds cache-read
share at 1.000 across *commits*. Whether it also holds across *time* is
untested (F28 says so explicitly). If it does, the gate is free rather than
nearly free, and the design's "one static system prompt per card" rule buys the
human gate as well as the checkpoint commits.

### 4a.2 Procedure — haiku, ~free, runs alongside S-VAR

Same instrument as H1 (`ttl-mark` / `ttl-read`, the 37 KB review-scale
`fixtures/h1/payload.txt`), with `--system-prompt <static card text>` on both
the mark and the read. Gaps: **1 min** (warm control), **65 min**
(TTL-crossing, the H1 decay point), **~24 h** (long tail). Each gap gets its
own mark; a mark is durable, so nothing must survive the wait (F22).

**Each arm runs in its own copy of the fixture directory** (`fixtures/h1-a`,
`h1-b`, `h1-c`), because the per-directory cache segment is the quantity under
test: a read in one arm would otherwise refresh the prefix the next arm's read
is supposed to find aged. H1 was not confounded this way only because both of
its long reads had already expired the segment; S-H1b's hypothesis is precisely
the case where a read would *not* expire it. No other dispatch may run in an
arm's directory between its mark and its read.

### 4a.3 Metric and decision rule

**Primary**: `cache_creation` on the read, by gap, against H1's 0 / 18,603 /
18,603.

- 65-min read ≈ 0 creation: the static system prompt blunts time decay as it
  does commit churn; the design records "gate cost 0 under the card rule" and
  D-rule "gate placement is not a cost decision" gains a second leg.
- 65-min read ≈ 18.6k: the volatile segment is not the system prompt's
  git section alone; F28's flat cost stands and nothing changes.
- Something in between: report the number; the segment is partly
  system-prompt-derived.

**Prediction, before data**: the 65-min read shows creation **between 4k and
10k** — the git snapshot is deleted (F16) but the per-directory segment also
carries tool definitions and settings that age independently.

**Sizing**: 3 marks + 3 reads = 6 haiku dispatches. Below instrument resolution
on headroom (F14). The 24 h read may be taken any time after 24 h.

## 4b. S-WT — does the per-worktree cache cost vanish under a static prompt?

**Added 2026-09-05, before any S-WT dispatch; S-VAR was running, no S-WT data
existed.**

### 4b.1 Why

F16 arm (d): one worktree per dispatch steady-states at 0.685 cache-read share
— the same ~31% loss as a commit — and the amendment says the combination with
`--system-prompt` "should cost nothing … though that combination was not run".
Phase 2's project-DAG parallelism (one worktree per story) is priced on that
untested sentence.

### 4b.2 Procedure — haiku, ~free

E1 arm (d)'s topology exactly: ten worktrees of `fixtures/e1-d-wt` (or fresh
ones under a new fixture dir if those are gone), one `probe --repeats 1` per
worktree, in sequence, with `--flag --system-prompt --flag "<static text>"` —
the same static text as E1 arm (e). Arm label `wt-sysprompt-w1..w10`. The
first dispatch pays the one-time cold write for the new system prompt and is
excluded from steady state, as in F16.

### 4b.3 Metric and decision rule

**Primary**: steady-state cache-read share across w2..w10, against arm (d)'s
0.685 and arm (e)'s 1.000.

- ≥ 0.95: worktree fan-out is cache-free under the card rule; Phase 2's DAG
  parallelism carries no cache penalty and the proposal's "~0 under
  `--system-prompt`" is measured rather than inferred.
- ≈ 0.69: the per-directory segment is not the system prompt; a worktree costs
  31% regardless, and the DAG pricing keeps that number.
- In between: report it; the segment is split.

**Prediction, before data**: **≥ 0.95**. F13's per-directory segment matched
F16's commit-invalidated segment token-for-token (7,827 vs 7,853), and
`--system-prompt` recovered all of the latter.

**Sizing**: 10 haiku dispatches, below headroom resolution.

## 4c. S-H1c — does *appending* to the default prompt keep the subsidised head?

**Added 2026-09-06, before any S-H1c dispatch.** Motivated by S-H1b's result
(the 65-min read re-created the whole 29.6k prefix under a static
`--system-prompt`) and by the platform docs (`references/research/2026-09-06-grounding-s-results.md`
Part B): the prefix is `tools → system → messages` with one cache entry per
breakpoint; the default prompt's static head is refreshed by any session on the
account, while a replaced prompt's tail is refreshed only by itself.

### 4c.1 Question

If the card is delivered as `--append-system-prompt` on top of the **default**
system prompt with `--exclude-dynamic-system-prompt-sections`, does a resume
after the TTL pay only for the appended tail (≈ the card, a few k tokens) rather
than the whole prefix — and is commit/worktree invariance preserved?

### 4c.2 Procedure — haiku, ~free

Two parts, same instruments as §4a/§4b:

1. **Time:** `ttl-mark`/`ttl-read` in three fresh directory copies (`h1-d/e/f`),
   gaps 1 min / 65 min / 24 h, flags `--exclude-dynamic-system-prompt-sections
   --append-system-prompt "<static card text>"` (the S-H1b text). No dispatch of
   any kind in an arm's directory between mark and read; the 65-min and 24 h
   marks are laid *after* any other haiku dispatch that day.
2. **Worktrees:** `probe --repeats 1` in ten worktrees with the same two flags
   (arm `wt-append-w1..w10`), and `probe --repeats 10 --commit-between` in one
   directory (arm `commit-append`).

### 4c.3 Metrics and decision rule

- **Primary:** `cache_creation` on the 65-min read. ≤ ~5k (the appended tail
  only) → the card rule in the proposal is re-specified as *append*, not
  *replace*, and gate resume is cheap in the way rev. 3 assumed. ≈ 18.6k →
  same as H1's default prompt; the head survived, the dynamic section did not,
  and append buys nothing over the default. ≈ 29.6k → the flag does not keep
  the head warm either; S-H1b's cost stands under both routes.
- **Co-primary:** steady-state cache-read share across `wt-append-w2..w10` and
  across `commit-append` draws 2–10, against S-WT's 1.000 and F16 arm c's
  0.846. < 0.95 on either → append forfeits the invariance the static prompt
  bought, and the trade-off is cost-per-resume versus cost-per-dispatch, to be
  priced from the two numbers.

**Prediction, before the data:** the 65-min read creates **4k–8k** (the appended
card plus the first user message carrying the dynamic sections); worktree share
**≥ 0.95**; commit share **0.85–0.95** (the dynamic sections moved into the user
message still change with the commit, but sit after every breakpoint that
matters).

**Sizing:** 6 + 10 + 10 = 26 haiku dispatches, below headroom resolution.

## 5. Sequencing

S-MIG first (free, and it settles D6's section list before any type is
written). S-TYPED and S-VAR next, interleaved across five-hour windows; they
share fixtures. **S-H1b and S-WT run alongside S-VAR** — haiku, below headroom
resolution, and S-H1b mostly waits. S-BIN when the UE project is pinned and its
seeds verified; the seed list is added to this document as a dated amendment
before dispatch.

*Actual order, recorded:* S-VAR started first (2026-09-05) before S-MIG,
because the census tool did not exist and the Opus window was free. S-MIG
remains a Phase 1 gate regardless of order.

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

- **2026-09-05, before any S-H1b dispatch:** §4a.2 gains one directory copy
  per arm, so a read cannot refresh the prefix a later arm measures.
- **2026-09-05, before any S-H1b or S-WT dispatch:** §4a and §4b added
  (both listed in the proposal §8, neither previously registered here). S-VAR
  was in progress; no S-H1b/S-WT data existed.

- **2026-09-05, after 10 dispatches, all of which are DISCARDED:** §1.3's
  dispatch ceiling raised from `--timeout-secs 2400` to `3600`, and the ten
  records taken at 2400 s quarantined to `results/s-var-2400.jsonl` and excluded
  from every S-VAR metric. The set is re-run from r1 under one ceiling.

  **What forced it.** `issues-135-129-declared-boundary-parse` r2 hit 2401 s and
  voided on `error_kind: timeout` (`api_error_status: null` — not a platform
  fault). It is not an outlier: 4 of the 20 arm-b CLEAN draws of these six items
  on record exceed 2000 s, and two items reach the ceiling —
  `issue-146-147-boundary-window` (2388 s in F27, 2055 s in E3, 2240 s here) and
  `issues-135-129` (2150 s, then the timeout). With 50 draws outstanding, ~17 of
  them belong to those two items.

  **Why it is a metric threat and not housekeeping.** A timeout voids, and a void
  is excluded from `n`. That is safe only if wall clock is independent of
  verdict, and it is not: on `issue-146-147` the rejections ran 1872–2240 s while
  F27's approval ran 893 s. A 2400 s ceiling therefore censors slow draws
  preferentially, and if slow draws reject more often the measured flip rate is
  biased **downward on exactly the two `MutationSurvived` items** whose band the
  §1.5 decision rule turns on. Dropping the voids does not correct that bias; it
  is the mechanism of it.

  **Why the records are discarded rather than pooled.** Draws at 2400 s and at
  3600 s are not one instrument. Keeping both would put a censoring ceiling on
  10 of 60 draws and leave a confound in the primary metric to save ~$26.

  **This note post-dates ten records and no others.** They are
  `issue-62` r1, `fix-advance` r1–r2, `issue-146-147` r1–r2, `issues-135-129`
  r1–r2 (r2 void), `issue-74` r1–r2 and `issue-63` r1 — every S-VAR record then
  in existence. All are retained on disk, unedited, in
  `results/s-var-2400.jsonl`; none contributes to any reported figure. No item
  had a readable band at n≤2, so no result was seen that this change could be
  fitted to. 3600 s is not a new number: HANDOFF already prescribes it for arm
  (a) CLEAN.

- **2026-09-06, before any S-H1b or S-WT dispatch; no S-H1b or S-WT record
  exists.** Five instrument disclosures for §4a/§4b. None changes a metric, a
  threshold or a prediction; each is forced by the rig or by the cache
  mechanism, and all five were settled before the first dispatch.

  1. **One `--out` file per S-H1b arm.** `ttl-read` finds its mark with
     `journal::last_warm_mark` (`runner/src/journal.rs:151`), which returns the
     globally latest `label:"warm"`, `outcome:"ok"` line in the `--out` file —
     no filter on `arm`, none on `cwd`, and no resume key at all. Three arms
     sharing one file, with their marks laid down minutes apart as §4a.2's
     separate directories invite, would pair every read against whichever mark
     was written last. Each arm therefore writes to
     `results/s-h1b-{1m,65m,24h}.jsonl`, one mark per file; the three are
     concatenated into `results/s-h1b.jsonl` after the last read, and the
     per-arm files are kept as provenance.

  2. **`--flag=--system-prompt`, not `--flag --system-prompt`.** §4b.2 (and
     `prompts/run-s-h1b-and-s-wt.md`) prescribe the space form. `ProbeArgs.flags`
     is declared without `allow_hyphen_values` (`runner/src/main.rs:394`), so
     clap rejects the hyphenated value and the process exits 2 having dispatched
     nothing. The `=` form is what E1 arm (c) recorded, and is what S-WT uses.

  3. **S-WT keeps E1 arm (e)'s text; S-H1b uses the card text. They must not be
     the same string.** §4b.2 requires "the same static text as E1 arm (e)",
     which `results/e1-arms.jsonl` records as `You are a terse assistant. Answer
     in one word.`; §4a.2's "static card text" is `You are a careful software
     reviewer. Answer only what is asked.` The system prompt heads the prefix,
     so two dispatch streams sharing it share the ~69% cross-directory segment
     (F13): S-WT's ten dispatches would refresh the very segment S-H1b's aged
     reads exist to find expired, and §4a.2's per-arm directories guard only the
     per-directory tail. Distinct texts make the two spikes cache-disjoint, so
     S-WT may run during S-H1b's waits as §5 intends.

  4. **Within S-H1b, the long arms' marks go last.** For the same reason, a
     read's effective shared-segment age is (read − last dispatch of *any* arm),
     not its own gap. Dispatch order is therefore: `h1b-1m` mark, `h1b-1m` read,
     `h1b-24h` mark, `h1b-65m` mark, then the waits. This makes each long arm's
     mark the last same-prefix dispatch before its read, so `gap_minutes` is the
     honest age of the whole prefix and not just of its per-directory tail. H1
     was not run under this discipline — `fixtures/h1` carried other traffic —
     and the comparison to its 0.484 is qualified accordingly in the report.

  5. **The run is in a git worktree** at `D:\repos\redkiln-rs.swt`, so that
     `cargo build` cannot relink the binary S-VAR is executing and no fixture or
     results path can collide with it. Every fixture is a fresh copy taken from
     the main checkout: `fixtures/h1` → `h1-a`/`h1-b`/`h1-c` (payload verified
     byte-identical, sha256 `cff4af90…`), and `fixtures/e1-d` → ten detached
     worktrees under `fixtures/s-wt/`, reproducing arm (d)'s topology and its
     18-byte tree. The absolute paths differ from H1's and E1's, which is
     immaterial: §4a.2 already requires directories never dispatched in.

  Recorded for the report, not a change: `claude` is **2.1.263** here, against
  **2.1.261** for every H1 record.

- **2026-09-06, after the `h1b-1m` pair and before any other S-H1b or S-WT
  dispatch: the note above, item 3, is WRONG on its mechanism and the
  sequencing is corrected.** It claimed distinct system-prompt texts make S-H1b
  and S-WT cache-disjoint. The `h1b-1m` mark falsifies that. Four minutes after
  a smoke dispatch that used a third, unrelated system prompt, the mark read
  **15,129 tokens warm** and created only 14,490 of a 29,619-token prefix
  (`results/s-h1b-1m.jsonl`, `label:"warm"`).

  **What that measures.** The prefix has a block of ~15.1k tokens that sits
  *before* the system prompt — tool definitions and settings — and is shared by
  any two dispatches whatever their `--system-prompt`. Only the ~14.5k after it
  is text-dependent. So the split F16 attributed to the system prompt is
  itself two-part, and no choice of text can isolate one dispatch stream from
  another; it can only isolate the tail.

  **Consequence.** S-WT's ten dispatches would refresh 51% of the prefix that
  S-H1b's 65-minute and 24-hour reads exist to find expired, which is the
  contamination §4a.2 was written to prevent. S-WT therefore runs to
  completion **before** the `h1b-24h` and `h1b-65m` marks are laid, not during
  their waits, and each long arm's mark stays the last dispatch of any kind
  before its read. §5's "S-H1b and S-WT run alongside S-VAR" is unaffected:
  S-VAR is Opus and shares no prefix with either.

  This note post-dates exactly one record — the `h1b-1m` pair, which it does not
  touch: that arm was marked and read 1.5 minutes apart with nothing in between,
  and it returned share 1.000 / creation 0 regardless of what the tool block did.
  No `h1b-65m`, `h1b-24h` or `wt-sysprompt-*` record existed when this was
  written.

- **2026-09-06, before any S-TYPED dispatch; no S-TYPED record exists.** Four
  changes to §2 and one new registration, all motivated by the grounding pass
  (`references/research/2026-09-06-grounding-s-results.md`) and disclosed here
  before the first typed dispatch:

  1. **Schema (§2.2).** Each finding gains a required free-text `rationale`
     (*Capacity, Not Format*, arXiv 2606.09410: frontier models pay a small
     penalty when the constrained final message must also carry the reasoning),
     and `repro` becomes a required object `{command, observed,
     expected_when_met}` so a finding is an executable claim the machine can
     later re-run under the **flip** criterion (Agentless: 213/300 reproductions
     failed on base, only 94 passed under the true fix). Enums unchanged;
     `additionalProperties: false` throughout; no optional fields. The runner's
     `typed_findings_schema()` is the registered instrument and a unit test
     validates a sample digest against it.
  2. **Third condition (§2.4).** The five E3-H S-SUBTLE seeds, typed arm only
     (`b-typed-s-subtle`, sighted prompt, `--timeout-secs 3600`), comparator
     E3-H arm (b) sighted (5/5, F26). This is the negative control S-VAR lacked:
     a defect the checklist does not name (*Reliability without Validity*,
     arXiv 2606.19544 — stability is not validity). +5 dispatches.
  3. **Registered secondary (§2.5) — contradiction rate.** On CLEAN, for each of
     the five F24 items: does a finding cite the pre-registered F24 target file
     (`results/s-var-citation-targets.json`, fixed before S-VAR) with a
     blocking-grade evidence class (`reproduced` or `mutation-survived`)? This is
     the claim D4 rests on — that the finding is *reported* even where the prose
     reviewer would have approved — measured mechanically. It replaces the
     exploratory keyword read of S-VAR's rubric rationales, which is not a
     metric. Predicted: ≥ 4 of 5.
  4. **Reporting rule (§2.5).** Any item drawn more than once is reported as
     `k/n` with a Wilson 95% interval (pass^k), and no arm comparison rests on a
     one- or two-item difference (*The Coin Flip Judge*, arXiv 2606.13685: hard
     items need ~15 trials for 90% majority fidelity).
  5. **Timeout.** S-TYPED CLEAN and S-SUBTLE dispatches use `--timeout-secs
     3600`, per the S-VAR amendment above (a 2400 s ceiling censors slow draws
     preferentially).
  6. **§4c S-H1c registered** — the `--append-system-prompt` route, with
     predictions. Instrument: `ttl-mark`/`ttl-read` gain the `--flag`
     passthrough `probe` already had (no behaviour change for existing arms).

  Sizing becomes 23 + 23 + 5 = **51 typed dispatches** (~$90).

- **2026-09-06, before any S-TYPED dispatch under the amended schema; 14
  superseded records quarantined.** Three things, and the first is a correction.

  1. **Correction to the bullet above.** That note states "no S-TYPED record
     exists". At the time it was written **14 did**: a validation draw plus 13
     more `b-typed-s-silent` records, produced earlier the same day by a run
     started against the schema as registered on 2026-09-05 and stopped as soon
     as the schema edits appeared in the working tree. They are quarantined,
     unedited, in `results/s-typed-oldschema.jsonl` (log:
     `s-typed-oldschema.log`) and contribute to **no** S-TYPED metric. The
     substance of the preceding bullet stands — every change in it was motivated
     by the grounding pass and none was chosen after seeing a result under the
     amended schema — but the pre-registration must say what was on disk, so this
     records it.

  2. **Why they cannot simply be resumed into.** All 14 carry
     `arm: b-typed-s-silent`, the same label the amended schema emits, and
     resume is keyed `(arm, slug, repeat)`. Left in place they would be *skipped*
     as already-recorded and would silently pool a `repro`-as-string, no-
     `rationale` digest into the conformance rate and the detection count, with
     no field an audit keyed on `arm` could separate them by. Quarantine, not
     resume. (14/14 were schema-conformant under the *old* schema, which says
     nothing about conformance under the new one — the structured `repro` object
     is precisely the harder demand.)

  3. **Timeout, restated for all three conditions.** S-TYPED S-SILENT, S-SUBTLE
     and CLEAN all run at `--timeout-secs 3600`. The preceding bullet set 3600
     for CLEAN and S-SUBTLE; S-SILENT joins them so a single ceiling covers the
     spike. The reason is the one given for S-VAR: §2.4 measures the typed arm
     against E3's *recorded* prose arm, which was **uncensored** (`b-clean`
     median 682 s, max 2055 s, 0 of 23 at 2400 s), while the typed arm is slower
     — the quarantined validation draw took **466 s against the same item's
     131 s in prose, 3.6×**. A ceiling only censors where it binds; holding the
     *number* equal to the comparator's would make the *conditions* unequal, and
     F29 showed the resulting bias falls on the slow, rejecting draws.


- **2026-09-07, after 42 S-TYPED records; one item retried at a raised ceiling,
  and a correction to the ceiling rationale above.**

  1. **The retry.** `issues-135-129-declared-boundary-parse` CLEAN voided on
     `error_kind: timeout` at **3601 s** (`api_error_status: null` — not a
     platform fault, cost $0.00). It is re-dispatched **once, at
     `--timeout-secs 5400`**, as a disclosed per-item exception. The other 22
     CLEAN items stay at 3600 s, so exactly one record in the condition carries
     a different ceiling and it is named here. The void record is retained,
     unedited, alongside whatever the retry produces; resume is keyed
     `(arm, slug, repeat)` and a void does not satisfy it, so no record is
     replaced.

  2. **Correction to the 2026-09-06 ceiling rationale.** That note justified
     3600 s on the claim that the typed arm is systematically slower than its
     prose comparator and would therefore be *preferentially* truncated —
     extrapolated from S-SILENT, where typed ran **+152%** on wall clock, and
     from a single validation draw at 3.6× prose. **The CLEAN data does not
     support that claim.** Over the 13 completed typed CLEAN draws the median is
     **1071 s against prose's 682 s — 1.6×, not 2.8×** — and **no** completed
     draw reached even 2400 s, let alone 3600 s. On the two slowest shared items
     the arms are at parity (1998 s vs prose 2029 s; 1834 s vs 1949 s).

     So the timeout is **one pathological draw, not a censored tail**: prose
     completed this same item in 1176 s, and the voided typed draw ran >1.8× the
     next-slowest typed CLEAN draw. Raising the ceiling to 3600 s remains
     defensible on margin alone (prose's max was 2055 s, leaving 2400 s little
     room), but the *mechanism* asserted in that note — systematic asymmetric
     censoring of the typed arm — is not what the data shows, and F31 must say
     so rather than inherit the earlier reasoning.

  3. **What this does not license.** If the retry also times out at 5400 s, the
     item is reported as a void and the condition as n=22, with the censoring
     question left open rather than resolved by raising the ceiling again. No
     further ceiling change is registered here.

- **2026-09-09, before any S-BIN dispatch:** §3.2's fixture and §3.3's draft type are bound
  to a named project. Nothing in §3.1, §3.4, §3.5 or §3.6 changes: the question, the decision
  rule, the prediction and the sizing stand exactly as registered. **This entry was rewritten
  the same day after an adversarial pass found three defects in its first form; what the pass
  found is recorded in full below rather than quietly corrected, because two of the three would
  have invalidated the seeds.**

  **The project and the pin.** `D:/repos/BrittonGames`, `OurGame/OurGame.uproject`, **Unreal
  Engine 5.8**, project pin **`041ead2cb284afd3004d07c2826f31d05b2799fd`** (2026-09-05, "fix(E8):
  stop a routed copy carrying the pack's container into the stash") [measured, `git log -1`]. It
  satisfies D8. Its automation already runs headless and its assets are in Git LFS
  [measured, `_workspace/tooling/verify_headless.ps1`; `.gitattributes` lines 4–5].

  **The pin is provenance, NOT the review tree.** Each item is reviewed **at its own commit**.
  This is the first defect the adversarial pass found and it is the load-bearing one: a seed
  applied by reverting an asset *at the shared pin* does not restore one defect, because both
  seeded assets were touched again before the pin. `WBP_LoadoutScreen.uasset` changed in
  **`b0cff782d`** and **`bf9c499a1`** after `a71265925`, and `WBP_StashPanel.uasset` changed in
  **`4014db6c8`** after `bf9c499a1` [measured, `git log a71265925..041ead2c -- <path>` and
  `git log bf9c499a1..041ead2c -- <path>`]. So at the pin, seed 4 would have rewound across three
  commits — carrying seed 5's own magenta search box, destroying the independence of the two
  seeded items — and seed 5 would additionally have stripped **8.5.8's sort strip**, whose
  automation spec `Inventory_StashSortControl.spec.cpp` ships at the pin, so §3.2's "the suite
  stays green" would have been **false by construction** rather than merely unverified.

  The first form of this amendment claimed the reverts were verified. What was actually checked
  was `git diff --stat <seed>^ <seed>`, which reports what that **commit changed**, not what
  reverting to its parent **at the pin** restores. That is the same defect class this programme
  logged three times in its own tooling — assuming a value's shape instead of checking it — and
  the check that settles it is a blob-oid comparison, not a file count.

  **Corrected instrument.** Each item's review tree is that item's own commit. A seed is that
  commit's own asset reverted to its immediate parent, applied in a tree at that commit, so the
  revert undoes exactly one commit's change to exactly that asset and nothing else:

  ```bash
  # item 4, in a tree at a71265925
  git checkout a71265925^ -- OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset
  # item 5, in a tree at bf9c499a1
  git checkout bf9c499a1^ -- OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset \
                             OurGame/Content/UI/Inventory/WBP_StashPanel.uasset
  ```

  Parents verified: `a71265925^` = `cc0cf2d01`, `bf9c499a1^` = `b0cff782d` [measured,
  `git rev-parse`]. **Before dispatch, each seed is confirmed by blob oid** —
  `git rev-parse <seed>^:<path>` against the seeded tree's own `git rev-parse HEAD:<path>` — and
  not by a file count.

  **The five items, and their spec.md is CONSTRUCTED.** The first form of this amendment said
  "they do not need constructing". That was an overclaim and is withdrawn. §3.2 registers items
  "each with a `spec.md` of 3–6 ACs and an implementation report"; the project's backlog supplies
  **ACs, in this programme's own voice, with a tier column** — which is more than §3.2 assumed —
  but two rows fall below three clauses and one of those is unverifiable by a reviewer. Each
  item's `spec.md` is therefore **built from the story row plus the commit's own stated claim**,
  and the implementation report is the commit message rendered into the fix lane's shape. That
  transcription is part of the fixture and is registered here, not left to the builder.

  | # | shape | commit | story | ACs at the pin | the change | ground truth |
  | --- | --- | --- | --- | --- | --- | --- |
  | 1 | CLEAN | `cc0cf2d01` | 8.5.5d | 4 sub-clauses — in range | `WBP_StashRow.uasset` — an empty `CanvasPanel` gains a row face: legible rows, `INDEX_NONE` rendered blank not `0`, the equipped chip bound and correctly **absent**, row count matching the viewmodel | approved |
  | 2 | CLEAN | `fa1ea72f4` | — (see below) | **none — closes no story** | `FTEST_Inventory_Stash.umap` — **a single file, a single asset**, the only genuinely diff-free item in the set | approved |
  | 3 | CLEAN | `e83463288` | 8.3.9 | 5 clauses — in range | `BP_GamePlayerState.uasset` + `DA_Item_Sword.uasset` — a content-only fix that turns the suite green ("Suite 997 tests, 0 failed") | approved |
  | 4 | SEEDED | `a71265925` | 8.5.5b | **1 clause, and it is a designer's written verdict** | `WBP_LoadoutScreen.uasset` — the fix set `LoadoutFill`'s alpha to 0.970; the seed reverts to **0.097**, leaving the left column translucent | changes-requested |
  | 5 | SEEDED | `bf9c499a1` | 8.5.5g | 4 clauses — **but none covers the search box** | `WBP_LoadoutScreen.uasset` + `WBP_StashPanel.uasset` — the fix gave the search box a dark background and cream foreground; the seed reverts to white-on-MAGENTA, rendering "Search stash…" as mud | changes-requested |

  Item defect descriptions are [documented] — they are the project's own commit messages
  (`a71265925`, `bf9c499a1`) verbatim, not an independent read of the binary assets.

  **Three item-level defects the adversarial pass found, stated rather than papered over.**

  1. **Item 2 closes no story.** `fa1ea72f4` is a map-authoring chore; its own message says
     "Nothing references this map yet; 8.5.6 is the story that will", and 8.5.6 was closed later
     by `2b0814f78`. 8.5.6's two ACs are about walking the player in and out of reach, which this
     commit does not do. Its `spec.md` is therefore constructed **from the commit's own claim**
     (world settings, physics volume and nav data matching the existing Tier-C harnesses; one
     `BP_GameStash` at X=800 against `DT_Reachability`'s 500 MaxDistance), and the item is
     retained because it is **the only diff-free item in the set** and §3.1's question is exactly
     about that case. It is not paired to 8.5.6.
  2. **Seed 5 violates no registered AC of 8.5.5g.** That row's clauses cover tab affordance,
     uncut titles and the empty-state label, and it explicitly hands hue to 8.5.5b. So the
     "changes-requested" ground truth cannot rest on 8.5.5g as registered. It rests on the
     constructed `spec.md`, which carries the commit's own claim — "a dark specified background
     and a cream specified foreground on all three colour slots at 17pt" — as an AC. **That
     clause is what the seed violates, and it is named here before dispatch.**
  3. **Item 4's story row hands the reviewer the answer.** 8.5.5b's row states the defect in
     bold ("`LoadoutFill` alpha is 0.10 where 0.97 was intended"). Its `spec.md` must therefore be
     constructed from the **commit's claim** ("Now 0.970, verified on both the authoring tree AND
     the generated class archetype") and must not include the story row's diagnosis. Its single
     registered clause — a designer's written verdict — is not verifiable by a reviewer holding
     `Read` and `Bash` and is **not** the clause the seed violates.

  **`text_diff` is scoped per item, and four of five items are not diff-free.** Only `fa1ea72f4`
  is a single asset. The other four carry `CLAUDE.md`, `STATUS.md` and
  `game_foundation_backlog.json` changes, and **item 1 adds 55 lines to
  `UE58_Editor_And_MCP_Gotchas.md` describing the very change under review** [measured,
  `git show --numstat`]. Registered: **`ChangeSummary.text_diff` carries the item's source and
  test paths only.** Backlog bookkeeping and the gotchas file are excluded, because leaving them
  in would decide §3.5's prediction by fixture construction rather than by the reviewer. The
  register rows that describe S-BIN as "no diff" are qualified accordingly.

  **The author-chosen-seed threat is weakened, not removed** (§3.6, same threat as E3-H §8.2).
  The seeds are real historical defects, exactly reproducible from git, with ground truth in the
  project's own subsequent fix commit. What remains is **selection**: two were chosen from 29
  asset-only commits [measured] because their defects are perceptual, so a reviewer good at
  perceptual defects and bad at others scores well here on a non-representative sample of two.
  n = 5 is a type-design sample, not a rate.

  **Still outstanding, and it is a precondition.** §3.2 requires the seeds' greenness verified
  **before** the spike runs — the suite must still pass with the reverted asset in place, since a
  seed the suite catches measures nothing about review-without-a-diff. **Not run.** It needs
  `verify_headless.ps1` with the editor closed. The pass identified a live route for it to fail:
  `Inventory_StashLoop.cqtest.cpp` loads `WBP_StashPanel` by hard path, so seed 5 must be checked
  against that spec specifically. If a seed goes red it is replaced from the remaining 27
  asset-only commits and this amendment is extended, never quietly edited.

  **What the fixture's instruments say about §3.3's draft type.** Inputs to the spike;
  `ChangeSummary` is BLOCKED on S-BIN's result and its field set is **not** fixed here.

  1. **`text_diff` cannot be decided by asking git.** §3.1 says binary assets "cannot be diffed".
     Under LFS they can, and the result is worse than no diff: the tracked content is the pointer,
     so `--numstat` returns a **pointer-line count — `1 1` or `2 2`, the oid line always and the
     size line when the size changed — and never the `-`/`-` binary marker** [measured,
     `git show --numstat cc0cf2d01 -- …/WBP_StashRow.uasset` → `2 2`, oids `c6a805c0…` →
     `33800c41…`, size 23,950 → 110,727; and `… a71265925 -- …/WBP_LoadoutScreen.uasset` →
     `1 1`, size unchanged at 50,724]. So `assets[].size` must be read **from the pointer**, never
     inferred from whether the size line moved. **This reaches past S-BIN.**
     `spikes/p1/derive-facets.py` marks `binary_change` on git's `--numstat` marker **or** an
     Unreal-path match, so on an LFS repository the marker half is structurally dead for assets
     and the path shortcut carries it — **for Unreal paths only.** On an LFS repository tracking
     any other binary kind, and this project's own `.gitattributes` LFS-tracks `.png`, `.psd`,
     `.zip`, `.wav` and `.dll` among others, **both halves are dead**: `change` gains `text` for a
     binary file and `binary_change` is emitted as a confident `false` — the
     absence-of-evidence-as-`false` defect **ADR-0089** was accepted to close [measured,
     `derive-facets.py` lines 40, 115, 117, 120]. Arm D's corpus is not an LFS repo — `redkiln`
     has no `.gitattributes` — so F39's `binary_change` 0.60 [0.23, 0.88] could not have measured
     this. **It is an S-FACET input as much as an S-BIN one**, and it makes no claim about F39's
     figure, which stands as measured.
  2. **`tests: { passed, failed, skipped }` loses a state the instrument reports.**
     `verify_headless.ps1` runs `Automation RunTest` under `-unattended -nopause -nosplash
     -nullrhi -noensuredialog` with `-ReportExportPath`, exporting `index.json` and
     `headless.log` [measured, the script]. That report's top level is `succeeded`,
     **`succeededWithWarnings`**, `failed`, **`notRun`**, `inProcess`, `totalDuration`; on the
     project's last recorded run — 2026-09-05, 1029 tests — succeeded **849**,
     succeededWithWarnings **180**, failed **0**, notRun **0** [measured,
     `_workspace/tooling/TestResults/index.json`]. **17.5% of that suite is in a state the draft
     type cannot express**, and `notRun` is not `skipped`.
     **The mapping is registered here, before dispatch, because leaving it free would decide
     §3.4's second branch by hindsight:** `succeededWithWarnings` and `notRun` are **folded into
     `passed`**, and the reviewer sees the raw report through `report_ref`. §3.4's branch 2 then
     fires on the reviewer's own prose naming the missing field, which is what it registered.
     **Provenance caveat:** `TestResults/` is gitignored (`.gitignore:164`) and untracked, so
     this report is **not reachable from a clone at the pin**. Before dispatch it is copied into
     the spike's own results directory with its SHA-256 recorded. The BrittonGames working tree
     also carries two modified paths (`AnimationGym.umap`, `OurGame.uproject`); the fixture is
     built from a clean tree at each item's commit, not from that working tree.
  3. **`captures[]` has two candidate producers, and the draft named neither.** UE's automation
     screenshot comparison is in the report schema and **has never run** — `comparisonExported`
     `False`, `comparisonExportDirectory` `''` [measured] — so it is reachable only by
     [inferred]. The producer the project actually uses is the manual `/ue-verify` route: **17
     PNGs tracked under `_workspace/runbooks/_captures/` at the pin**, and captures are named in
     three of the five items' own ACs [measured, `git ls-tree -r 041ead2c`]. Which one S-BIN
     feeds the reviewer is a fixture decision this amendment does not take.

  **One more thing the fixture buys that §3 did not anticipate, with its limit.** `tests[]` is
  per-test, carrying `fullTestPath`, `state`, `errors`, `warnings` and `artifacts`, and the
  project's story ACs name automation specs by dotted path
  (`OurGame.Inventory.DimensionlessLoss`). So an AC **can** be checked from the report alone —
  but from a named test's `state` **plus its `warnings`/`errors` counts**, not `state` alone:
  `state` is uniformly `Success` across all 1029 tests including the 180 warned ones [measured],
  so it cannot see the degradation finding 2 is about. That is the mechanism ADR-0019's per-AC
  discriminating-test norm needs, and S-BIN can measure whether a reviewer uses it.

- **2026-09-11, before any S-BIN dispatch, after every S-VAR, S-TYPED, S-MIG, S-H1b and S-WT record on
  file — five entries absorbing the 2026-09-10 pre-implementation audit** (`references/audit/2026-09-10/MASTER.json`;
  `spikes/DECISIONS-2026-09-11.md`). Registered wording is left verbatim throughout; each entry says what it
  post-dates.

  1. **§1.1's "~30%" (PIA-073).** The flip-rate bound F27 Limits 1 stated and §1.1 restates was not computed. At the
     registered two-sided Wilson 95% interval a classification that did not flip is compatible with a true flip rate
     up to **~56% at n = 3** (0/3 → [0.000, 0.561]) and **~66% at n = 2** (0/2 → [0.000, 0.658]) [measured,
     recomputed 2026-09-11]. §1.1's argument for S-VAR is strengthened, not weakened; nothing in §1.4/§1.5 moves.
     Post-dates all 60 S-VAR records (F29).

  2. **S-BIN's combination rule (PIA-084; ADR-0093 decision 4 scopes what S-BIN gates).** §3.6 registers each item
     dispatched **twice** for a stability read, and §3.4's decision rule is written per item as if once. Registered
     here before any dispatch: **the rule is evaluated per pass.** Branch 1 ("2/2 detection and ≤ 1/3 false positive
     → ship the type as drafted") **requires both passes** to satisfy it. A pass in which a seed is missed routes to
     branch 2 or 3 **on that pass's own prose** (the field the reviewer names, or the fields it cites without the
     asset), independently of the other pass. **Both passes are reported**, each with its detection count, its false
     positives and its cited fields; no figure pools them, and a 2/2 in one pass beside a 1/2 in the other is
     reported as exactly that — a stability finding, not a rounded pass. Post-dates no S-BIN record (none exists).

  3. **The prior-approval framing arm — predictions entered before any draw (PIA-013; ADR-0101 clauses 1–6, which
     supersede ADR-0088).** The arm is the S-SUBTLE family's "prior-approval framing" condition (the name replaces
     the earlier "already approved" wording): the five E3-H S-SUBTLE seeds and their five CLEAN pairs, typed arm only
     (§2's schema and strict policy), the card asserting in the paper's **strong** form that the change was
     reviewed and approved (arXiv 2603.18740 v2, *Measuring and Exploiting Contextual Bias in LLM-Assisted Security
     Code Review*: strong bug-free framing lowered detection 16–93 percentage points on 5 of 6 models, Opus 4.5
     −4.9 pp n.s.; weak framing 0.8–24.7 pp; true negatives rose +46 to +95 pp). Comparator: the same ten items
     under §2.4's unframed typed arm. **Measured quantity: detection** (ADR-0101 clause 2). **Both halves are
     reported separately every time** — missed seeds (FN) and the **CLEAN-pair approval rate** — because the
     mechanism is a whole-classifier shift toward "clean", and a pooled error rate would read a falling FP rate as
     an improvement (clause 3). **Predictions, before any draw:** (i) **detection goes down** on the seeded five
     against the unframed arm's 5/5 (F31); (ii) the **CLEAN-pair approval rate goes up** against the unframed
     typed arm's 14/23 approvals (F31: 9/23 blocked under the resolved policy) — the false-positive half *falls*,
     it does not stay put, which was the ground ADR-0088 rested on; (iii) **Opus-class models may show a null effect** on detection (the paper's
     Opus 4.5 cell), in which case the arm reports a bound, not a detection drop. **Sizing:** at n = 5 with
     detection at ceiling only a large effect (≥ 2 of 5 missed) is observable; each item is drawn k ≥ 3 times and
     reported as k/n with its Wilson interval; the arm reports a bound where it cannot decide (clause 6).
     **Cost (first order; design §10.5's cell, [inferred]):** ≈ 2 framings × 5 items × k draws at S-TYPED's
     median $3.03/review — ≈ $91 at k = 3, ≈ $152 at k = 5 for the five seeded items; reporting the CLEAN-pair
     half as registered above doubles the item count, so the honest first-order figure for both halves is
     ≈ $182 at k = 3 / ≈ $303 at k = 5, and the design's cell is the seeded half only; unscheduled. Consumer: the negative-control
     fixture design under ADR-0096 and the review card's framing rules. Until it runs no document claims the review
     gate is guarded against false approval (clause 5).

     **S-SUBTLE-SYM — registered, not designed (DECISIONS-2026-09-11 #9; ADR-0101 clause 7).** The symmetric arm:
     the same items framed as **already rejected**, measuring the false-rejection direction. Consumer: the
     review-gate policy table. Cost ≈ 2 framings × 5 items × k draws at $3.03/review, unpriced beyond that
     estimate; unscheduled. No claim about the false-rejection direction is made until it runs; it is second, not
     absent, because the gate's re-execution guard already protects that direction. Post-dates no record of either
     arm (none exists).

  4. **S-MIG's per-AC loss counts were cap-truncated (PIA-078; DECISIONS-2026-09-11 #11; ERRATA E-0075-1).**
     `spikes/results/census-s-mig.py` recorded each `LiftedWithLoss` record's loss list as `loss[:6]`, so the
     2026-09-05 run's per-AC counts (45 ACs without GIVEN/WHEN/THEN, 46 without a verification note) were the
     capped sums over 12 truncated records. On 2026-09-11 the cap was lifted for future runs — every `[:6]` / `[:4]`
     list truncation in the script (AC losses, TDD rows, gate rows, ledger rows, unknown keys, dangling links) — and
     the script was re-run **read-only at the pin** (`python spikes/results/census-s-mig.py --redkiln
     D:/repos/redkiln --sha 47faec9f --out <scratch>`, 45 s; `git show` reads, no write to the corpus or to
     `s-mig.jsonl`). Every one of the 6,877 records' `(path, kind, slot, outcome)` is identical to the recorded run;
     only the `what` lists lengthen. Uncapped: **76** loss lines "no GIVEN/WHEN/THEN" and **87** "no verification"
     (75 and 85 distinct `(spec, id)` pairs), **131** ACs with at least one loss, **29** lacking both, of **208** ACs
     in the 19 specs — 19/153 and 88% unchanged [measured, 2026-09-11]. The 98% the report's heading
     carries is the spec-section slot; the record-weighted typed-slot overall is **96.0%** (3,011 / 3,138). The
     recorded `s-mig.jsonl` is not replaced; `s-mig-report.md` carries the dated correction.

  5. **F30's `h1b-24h` read, restated for the schedule (PIA-109, PIA-019).** The 2026-09-06 correction above
     (item 3 of the five disclosures, and its correction) means the `h1b-24h` mark laid on 2026-09-06 is **spent** —
     S-WT and later dispatches refreshed its shared segment — so the read requires a **fresh mark and a ≥ 24 h
     window with no other dispatch on the machine** before it. `F30-DRAFT.md` (drafted 2026-09-06, promoted into the
     main checkout 2026-09-11) records the arm as unread. No new instrument rule; a restatement so the schedule
     cannot miss it.

Any later change is disclosed here with the date and the records it post-dates.

- **s8i pre-dispatch amendment — 2026-10-02, before any S-BIN dispatch; no S-BIN record exists** (the corpus's
  `main` at `285e011` holds no `results/s-bin.jsonl` [measured, `git -C D:/repos/redkiln-corpus cat-file -e
  main:results/s-bin.jsonl` → rc 128]). This entry settles what the 2026-09-09 entry left open, as runbook stage `s8i`
  §8i.2 requires, and records the operator's decisions on it: `spikes/DECISIONS-2026-09-21.md` D21-43 N19–N22,
  D21-48 X3 and D21-140 (which amends D21-105 item 2 for S-BIN). **Nothing in §3 changes.** The question, the
  decision rule, the prediction and the sizing stand exactly as registered, and so does every earlier entry, verbatim.
  Each item below cites its ledger line and does not re-argue it. Values that can be known only after B0's
  zero-spend fixture work are **not** written here as placeholders. Each is added later by an appended extension
  paragraph, dated, citing this entry, and never by an edit to this entry (the list is at the end).

  1. **Two corrections to the 2026-09-09 entry, recorded as measured** (read-only on `D:/repos/BrittonGames`,
     2026-10-02).
     - **Under the registered scoping, every item's `text_diff` is empty.** Command: `git -C D:/repos/BrittonGames
       show --numstat --format= <item>` for each of `cc0cf2d01`, `fa1ea72f4`, `e83463288`, `a71265925` and
       `bf9c499a1`. Item 1 changes `CLAUDE.md`, `WBP_StashRow.uasset`, `_workspace/backlog/STATUS.md`,
       `_workspace/backlog/game_foundation_backlog.json` and `_workspace/references/UE58_Editor_And_MCP_Gotchas.md`
       (55 added). Item 2 changes `FTEST_Inventory_Stash.umap` only. Item 3 changes `CLAUDE.md`,
       `BP_GamePlayerState.uasset`, `DA_Item_Sword.uasset`, `STATUS.md` and the backlog JSON. Item 4 changes
       `WBP_LoadoutScreen.uasset`, `STATUS.md` and the backlog JSON. Item 5 changes `WBP_LoadoutScreen.uasset`,
       `WBP_StashPanel.uasset`, `STATUS.md` and the backlog JSON. No item touches a source or test path. The entry's
       *"four of five items are not diff-free"* is git's view before the scoping, and the scoping it registered
       (*"`ChangeSummary.text_diff` carries the item's source and test paths only"*) leaves all five empty. The
       sentence stands as written, and this records what it means. The design register row that goes further is
       corrected by its own dated note in `s8i` §8i.8, not here.
     - **The failure route the entry named for seed 5 does not exist at seed 5's review commit.** Command: `git -C
       D:/repos/BrittonGames merge-base --is-ancestor 2b0814f78 bf9c499a1` → rc 1. `2b0814f78` (2026-08-31,
       *"test(E8): close 8.5.6 by walking the player in and out of the stash's 500cm reach under real movement"*)
       adds `OurGame/Source/OurGameTests/Private/Tests/Inventory/Inventory_StashLoop.cqtest.cpp` [measured, `git
       show --name-only --format= 2b0814f78`], and it is not an ancestor of `bf9c499a1`. Since each item is reviewed
       at its own commit, seed 5's greenness check would have been the whole `OurGame.*` suite as it stood at
       `bf9c499a1`. The point is moot for the dispatch, because seed 5 is replaced (item 12), but it is recorded
       rather than dropped.

  2. **The captures producer and its procedure** (D21-43 N19: *"**The operator captures by hand** on all five review
     trees through `/ue-verify`, with one procedure applied identically to clean and seeded trees."*). `/ue-verify`
     is BrittonGames' Claude Code slash command (`.claude/commands/ue-verify.md`, present at all five item commits),
     driving the open editor through Monolith MCP. So each capture is an interactive session on the account, made in
     B1 under its go-ahead. The procedure, identical on all five trees, in items 1 to 5 order:
     1. **The tree the editor opens** is the item's capture copy (item 4's table), a byte copy of the stripped base
        repository (item 3). The editor opens `OurGame/OurGame.uproject` in that copy.
     2. **`$ARGUMENTS`, by one rule, never chosen per tree.** List the asset paths the item's own commit changed:
        `git diff-tree --no-commit-id --no-renames -r --name-only <item commit>`, keep the paths ending in `.uasset`
        or `.umap`, and keep git's order. For item 5 the item commit is the replacement commit R. Map each path to
        its object path: `OurGame/Content/<rel>.<ext>` → `/Game/<rel>`, and `OurGame/Plugins/<P>/Content/<rel>.<ext>`
        → `/<P>/<rel>`. `$ARGUMENTS` is those object paths, space-separated, followed by the literal text ` — write
        the capture_scene_preview of the k-th asset above to _s-bin/captures/capture-<k>.png`, with k counted
        from 1. **No map is added.** A map appears only when it is itself a changed asset (item 2), because adding
        one would mean choosing it per tree, and N19 forbids a per-tree choice. The seeded trees use their item
        commit's list, so a seed changes the bytes captured and never the arguments.
     3. **The capture step is `capture_scene_preview`** (the command's step 7). Its files land at
        `_s-bin/captures/capture-<k>.png` inside the capture copy, at the same relative paths on every tree. Each is
        copied to `results/s-bin/captures/item-<n>/capture-<k>.png` in the corpus and hashed with SHA-256.
     4. **`monolith_status()` must report the capture copy's own project** before anything is captured. Afterwards
        the session and the editor are closed, and no `UnrealEditor` process survives before the next tree (Halt 6).
        The capture copy is then deleted.
     5. **One procedure-log line per session**, in `results/s-bin/captures/procedure-log.txt`, with space-separated
        fields and no spaces inside a value: `item=<n> tree=<capture copy path> model=<id> cost_usd=<x.xx>
        sha256=<64 hex, one per capture file, comma-joined> shows_subject=<yes|no> utc=<YYYY-MM-DDTHH:MM:SSZ>`.
        `shows_subject` is the operator's judgement on framing, never on the defect. A `no` on any tree, clean or
        seeded, stops the stage, and nothing falls back to empty captures.

     Only the capture files reach the reviewer, never the session's own report. A seeded tree's capture shows the
     seed, which is exactly what a real `Binary` summary would carry.

  3. **What the review tree holds besides the change** (D21-43 N20: *"**Backlog and status prose stripped from all
     five trees identically**, and recorded."*). **One strip set**, applied to every tree before its one commit:
     - `_workspace/backlog/`, whole (18 files at each item commit [measured, `git ls-tree -r --name-only <item>
       _workspace/backlog | wc -l`]);
     - `_workspace/references/UE58_Editor_And_MCP_Gotchas.md` (present at all five item commits [measured, `git
       cat-file -e`]);
     - `CLAUDE.md`'s four status-table rows, *Focus*, *Tier*, *Verify* and *Progress*, removed by this **byte rule**.
       Read `CLAUDE.md` as bytes and split it into lines, each line keeping its terminator (a line ends after a
       `\n`, or at end of file). Strip one trailing `\n`, then one trailing `\r`, from a copy of the line. The line
       matches if that copy begins with exactly one of the four byte prefixes `| **Focus** |`, `| **Tier** |`,
       `| **Verify** |` or `| **Progress** |`. Matching lines are removed together with their terminators, and every
       other byte is left unchanged. The strip **asserts exactly 4 rows removed per tree, each prefix matched exactly
       once**, and refuses otherwise. The rule is terminator-agnostic: it matches with or without a CR, and it does
       not depend on the measured fact that `CLAUDE.md` is LF-only at every item commit today (0 CR; 20,923, 22,365,
       22,376, 20,923 and 20,923 bytes; the four rows at lines 23–26 and each prefix once [measured, byte count over
       `git show <item>:CLAUDE.md`]).

     `fixture.json` records the set per tree, and it must be one set across the five
     (`stripped_path_set_is_identical_across_all_five_trees`).

     **Leak tokens.** No non-asset file of any stripped tree, tracked or not, may contain a registered leak token.
     The fixture asserts zero hits over all five trees (`review_tree_carries_no_registered_leak_token`), and the
     driver repeats the scan on every dispatch copy (item 4). **Item 4's token is `LoadoutFill`**, the name in its
     story row's diagnosis. At all five item commits it occurs only inside the strip set: once in
     `_workspace/backlog/epics/E8_Inventory/E8_Backlog.md` and two or three times in the backlog JSON, and in no
     file outside it [measured, `git grep -I -c LoadoutFill <item> -- . ':(exclude)_workspace/backlog'
     ':(exclude)_workspace/references/UE58_Editor_And_MCP_Gotchas.md'` → rc 1 at each]. Item 4's set is that one
     token. Broader words in the diagnosis (`translucent`, `0.970`, `0.097`, `WBP_LoadoutScreen`) occur in unrelated
     files outside the strip set at every item commit, so a scan for them would refuse a clean tree. They are not
     registered. The replacement seed's tokens are the words its own diagnosis uses, registered by extension when it
     is chosen (item 12). The reason for the whole strip is the one the 2026-09-09 entry gave for `text_diff`:
     *"because leaving them in would decide §3.5's prediction by fixture construction rather than by the
     reviewer"*.

  4. **The fix is unreachable** (F17; §0's `fix_unreachable`). Each review tree is a **fresh single-commit
     repository** holding only that tree. Its one commit is on `main` with the message `review tree` and the
     identity `s-bin <s-bin@invalid>`. It has no remote, no other ref and no tag. Assets are committed through the
     tree's own `.gitattributes` and BrittonGames' system-scope LFS filter, so each asset's blob is its LFS pointer.
     That pointer is byte-identical to BrittonGames' at the commit it came from, so a seed is confirmed by blob oid.
     Neither the fixed pointer blob nor its LFS object is present (`git rev-list --all --count` is 1). A reviewer
     that reads or runs `git` against a path outside its review tree voids the record as `void: out-of-tree`, and a
     void is never a miss.

     **Five copy kinds, and no tree is reused.** A Claude Code session keys its transcripts and auto-memory by its
     working directory's path: every character outside `[A-Za-z0-9]` becomes `-`, so `D:\repos\BrittonGames`
     becomes `projects/D--repos-BrittonGames`. A reviewer with `Bash` can also change the tree it is given. All
     copies sit under one short **review root, registered here as `D:/sb`**. It was absent when this was written
     [measured, `ls -d /d/sb`]. BrittonGames' longest tracked path is 164 characters, and a UE build adds deeper
     `Intermediate/` paths, all under Windows' 260-character limit.

     | copy | path | made | holds | afterwards |
     | --- | --- | --- | --- | --- |
     | base repository | `D:/sb/base/item-<n>` | B0 | the stripped tree, with its seed on items 4 and 5, as one commit | frozen; nothing runs in it |
     | greenness copy | `D:/sb/green/item-<n>` | B0 | a byte copy of the base | the operator's UE run; its report is copied out; deleted |
     | capture copy | `D:/sb/cap/item-<n>` | B1 | a byte copy of the base | the capture session; only the capture files leave; deleted |
     | review repository | `D:/sb/rev/item-<n>` | after B1, before any rendering is hashed | a fresh single-commit repository of the base tree plus `_s-bin/captures/capture-<k>.png` and `_s-bin/report/{index.json,headless.log}` (item 6) | frozen; its commit is `items.<n>.review_commit` |
     | dispatch copy | `D:/sb/d/p<pass>i<n>a<attempt>` | by the driver, before each Opus dispatch | a byte copy of the review repository | the dispatch; deleted once its record is written and no UE process survives |

     A copy is a byte copy of the directory, `.git` included. It is never a clone, so it gains no remote. **Before
     each spawn, the driver checks the dispatch copy and refuses on any miss.** It writes each result into the
     record:
     - `precheck_clean`: `git status --porcelain --untracked-files=all --ignored` prints nothing.
     - `precheck_blob_oids`: `git rev-list --all --count` is 1, `HEAD` is `items.<n>.review_commit`, and every asset
       path's blob oid is the fixture's.
     - `precheck_leak_hits`: the registered leak tokens count 0 over every non-asset file, tracked or not.
     - `precheck_no_project_dir`: neither `~/.claude-personal/projects/<slug>` nor `~/.claude/projects/<slug>` exists
       for the copy's path.
     - `tree_path`: the copy's path, which no earlier record and no capture copy used.

     Before each copy, the free space on the root's volume must be at least `SBIN_NEED_BYTES`: the base tree's size,
     plus the growth one greenness run left behind, plus a tenth. The rule is registered here, and the figure is
     added by extension from B0's measurements.

     **The answer-bearing paths outside the tree, named** so that the out-of-tree rule is checked against them by
     name: `D:/repos/redkiln-rs` (this file states the seeds), `D:/repos/redkiln-corpus` (`fixture.json` and the
     rendered prompts), `D:/repos/BrittonGames` (the fix itself), both accounts' `projects/` directories
     (`~/.claude-personal/projects/` and `~/.claude/projects/`, which hold the capture sessions' transcripts), and
     every other copy under `D:/sb`. Claude Code's permission rules cannot fence `Bash`, so prevention is not
     claimed. The defence is detection: the void, and for BrittonGames' two uncommitted paths, the snapshot series
     taken before B2 and after every batch.

  5. **The instrument, named.** The driver is `runner/src/sbin.rs` in the corpus runner, behind the new subcommand
     **`sbin-probe`**. Its 25 named tests are listed in `s8i` §8i.3. The registered argv, run from the corpus root
     through the launcher's `wrap` mode:
     - B2/B3, one pass each: `runner/target/debug/redkiln-spike-runner sbin-probe --pass <1|2> --fixture
       results/s-bin/fixture.json --out results/s-bin.jsonl --model opus --effort xhigh --timeout-secs <ceiling>`.
       This runs items 1–5 in order, resuming per `(arm, item, pass)`. A retry is the same argv run again: a valid
       record satisfies resume, a void does not, and the void's `(pass, item)` is dispatched once more as attempt 2.
     - B1's flag probes: `sbin-probe --flag-probe --model haiku --out results/s-bin/probes.jsonl`, at most three
       runs, each on a throwaway tree that is never a review tree.
     - The zero-spend dry run: `redkiln-spike-runner --claude-bin <stub> sbin-probe --dry-run --pass 1 --only-item 1
       --fixture results/s-bin/fixture.json --out results/s-bin/dry-run.jsonl`. It is the one invocation that passes
       `--claude-bin`.

     **The arm label is `b-typed-sbin`**: arm b (headless, the card adopted), the typed schema, and the S-BIN summary
     rendering. Following §6, *"Arm labels carry every instrument variant"*, so a B4 re-run carries
     `b-typed-sbin-plus-<field>`, registered by its own amendment. Model and effort are E3's, `--model opus --effort
     xhigh`. The model id is **observed from each init line** and recorded as `observed_model`. The set must report
     one observed id, as it must one CLI version, and a record reporting a second id halts the set.

     The card is E3's `redkiln-review`, adopted with tools **`Read` and `Bash`** (§3.3: *"the reviewer has `Read` and
     `Bash` in a disposable clone with the engine available"*). The runner's `e3::CARD_TOOLS` adds the
     `StructuredOutput` channel the digest returns on (F20), applied identically and not new reach. The dispatch
     runs with `--strict-mcp-config` and no MCP server. §0's pre-flight fields are recorded on every record:
     - `card_adopted`: adoption verified by equality between the init tool set and the card's tools; a mismatch
       voids before scoring.
     - `api_error_status`: platform errors named as such, `null` otherwise; a platform error is a void and never a
       miss.
     - `fix_unreachable`: true only when the dispatch copy passes item 4's checks.
     - `node_modules_linked`: stated as **not applying** to a UE tree, and recorded as `null`.

     Digests use §2's typed schema as amended 2026-09-06 and are scored under the **strict** policy (§3.4: *"under
     the typed schema and strict policy from §2"*). A digest failing the schema is `void: schema` (§0 rule 2). An
     exit 0 without structured output is a void, not a success.

  6. **The rendering.** §3.3's draft fields are *"Rendered into the prompt in place of E3's 'files this change was
     scoped to touch' list"*. The constructed `spec.md` and implementation report are rendered as E3 renders its spec
     and report artifacts, built as the 2026-09-09 entry registers. The mappings:
     - `range`: the review repository's single commit, with the base rendered as *not available in this repository*.
       This is the same on all five trees, because a base commit would be an object the fix-unreachable rule keeps
       out.
     - `text_diff`: source and test paths only, the entry's scoping. A source or test path is one under a `Source/`
       directory segment (`OurGame/Source/…` or `OurGame/Plugins/<P>/Source/…`). It is empty, and rendered as
       none, for all five items (item 1).
     - `assets[]`: one entry per asset path in item 2's list, as `{path, kind, action, size, lfs_oid}`. `kind` is the
       extension. `action` is git's status for that path in the item commit. `size` and `lfs_oid` are read **from the
       LFS pointer in the review tree**, the seeded pointer on a seeded item, and the size is *"never inferred from
       whether the size line moved"*.
     - `build`: `{command, exit, log_ref}`, from the item's greenness run. The command is `pwsh -File
       _workspace/tooling/verify_headless.ps1`.
     - `tests`: `{command, report_ref, passed, failed, skipped}`, from the same run's `index.json`.
       `succeededWithWarnings` and `notRun` are *"folded into `passed`"* (`passed` = `succeeded` +
       `succeededWithWarnings` + `notRun`), and `skipped` is rendered as 0, because no state of the report maps to
       it.
     - `report_ref` and `log_ref` point **inside the review tree**, to `_s-bin/report/index.json` and
       `_s-bin/report/headless.log`. Those are the greenness copy's report, also kept in the corpus at
       `results/s-bin/greenness/item-<n>/`. The raw report must be readable without an out-of-tree read, which
       would void the record. That is why the review repository's holdings in item 4's table add those two files to
       the captures.
     - `captures[]`: one `{path, viewport, note}` per capture file. `path` is `_s-bin/captures/capture-<k>.png`,
       `viewport` is `capture_scene_preview`, and `note` is the k-th object path from item 2. Nothing else from the
       session goes in.

     Each item's rendered prompt is written to `results/s-bin/rendered/item-<n>.txt` and hashed **before** its first
     dispatch. The driver refuses to render an item without its capture references
     (`every_item_carries_a_capture_reference_or_rendering_refuses`).

  7. **The ceiling: the rule now, the number by extension.** Golden constraint 6: *"When two conditions differ in
     shape, size the ceiling against the slow one"*. `ceiling_secs` = 3600 + `W_max`, rounded up to the next multiple
     of 300 s. `W_max` is the slowest greenness run's measured build-plus-suite wall clock over the five trees (B0).
     3600 s is the uncensored ceiling S-VAR and S-TYPED settled for a review that runs no engine. A reviewer that
     builds and runs the suite once adds `W_max`. The value is never below 3600 s. It is the same for every dispatch
     and every retry, and it is added by extension before B2.

  8. **Order.** Pass 1 is items 1–5 in order, then pass 2 is items 1–5 in order, after pass 1's audit has been read.
     Dispatches are strictly sequential, with one UE process at a time (Halt 6). Monolith listens on one port, so
     only one editor runs at a time in B1 as well.

  9. **The outcomes the rule does not decide, registered before dispatch.** D21-43 N21: *"**Registered as undecided
     before dispatch.** If either occurs, the `Binary` variant takes the human-gate route."* D21-48 X3:
     *"**Registered before dispatch as a third undecided outcome, `undecided-partial`**, routed to the human gate, as
     D21-43 N21 routes the other two."* Under the 2026-09-11 combination rule, each pass takes exactly one of six
     values: `branch1`, `branch2-detected` (B4's re-run detected), `branch3` (including a branch 2 whose re-run
     missed), and these three:
     - `undecided-fp`: 2/2 detection, after any B4 re-run, with ≥ 2 of the 3 clean items blocked.
     - `undecided-silent`: a seed missed, with no missing field named and no summary field cited.
     - `undecided-partial`: exactly one seed missed, counted after any B4 re-run, with summary fields cited and no
       missing field named.

     Each undecided outcome is reported as *"not decided by the registered rule"* and routes the `Binary` variant to
     the human gate. **Branch 2's named field is a judgement, and it is named as one** (§0: *"reported with the
     judgement it needs named"*). After each pass's audit the operator reads each missed seed's digest and writes
     `results/s-bin/missing-fields.jsonl`, one line per missed seed, `{pass, item, missing_fields: [{field,
     quote}]}`. Each quote is verbatim from a `claim` or `repro` in that record's digest. The file exists, possibly
     empty, when no seed was missed, and a B4 re-run exists for a `(pass, item)` exactly when that missed seed has a
     named field. `results/audit-s.py bin <s-bin.jsonl> <missing-fields.jsonl>` computes each pass's outcome from
     the records and that file alone. It ends with `rule_outcome_pass1=<v>` and `rule_outcome_pass2=<v>`, and no
     figure pools the two passes.

  10. **The caps and the void rule.**
      - **Caps**, enforced by the driver before it spawns (`the_dispatch_cap_refuses_before_spawning`), counting
        voids:
        - **18 Opus dispatches**, from 5 + 5 + ≤ 4 retries + ≤ 4 B4 re-runs.
        - **3 haiku**.
        - **USD 90.00** API-equivalent, summed over every S-BIN record's `total_cost_usd` and every probe's.
      - **The USD cap is registered at 90.00, citing D21-140.** In D21-140 item 1 the operator answered *"Raise to $90
        (file's cap)"*, and the entry reads: *"The amendment's item 10 registers the driver's USD cap at 90.00, citing
        this entry"*. It amends D21-105 item 2's *"capped at **US$30 total**"* for S-BIN's dispatches only.
      - The capture sessions are not the driver's to count, because it does not spawn them. The operator adds their
        procedure-log `cost_usd` to the running total. Before each batch, and before each retry, the batch launches
        only if (90.00 − spent so far) exceeds that batch's bound in `s8i` §8i.5's spend table. A missing cost is
        refused, never read as zero. Reaching any cap halts the set, and raising one is a ledger line.
      - **Retries.** A void is retried **once** per `(pass, item)`, at the same ceiling and inside its batch's
        go-ahead, as attempt 2 in a fresh dispatch copy. The same item voiding twice in one pass is a fixture defect,
        and it stops the stage.
      - **Resume** is keyed per `(arm, item, pass)` (§0's key, with `item` for `slug` and `pass` for `repeat`). A void
        does not satisfy it, and no record is ever replaced or edited: `results/s-bin.jsonl` is append-only, voids
        included.

  11. **The CLI version: frozen.** D21-43 N22: *"The CLI is **frozen from S-BIN on**, and the set never spans a
      version."* The freeze has two parts. The launcher exports `DISABLE_AUTOUPDATER=1` to each `wrap` child, and the
      operator writes the same row into `~/.claude-personal/settings.json`'s `env` block before B1. The row was absent
      when this was written [measured, `grep -c DISABLE_AUTOUPDATER` → 0]. Every record carries `cli_version`, which
      must equal its launch record's `CLI_VERSION`. A version move between the set's first record and any later
      dispatch **halts the set** (`a_version_move_between_dispatches_halts_the_set`), and it is never reported per
      version. Halt 2 is recorded, not silently continued: `RUNBOOK-instrument.txt` reads 2.1.278, the CLI read
      2.1.281 on 2026-09-24, and it reads 2.1.288 on 2026-10-02. The set's version is the one B1's pre-check records
      after the settings row is written. The CLI freeze has no exception. The method freeze below has exactly one.

  12. **Seed 5 is replaced, by a selection rule registered here before any candidate is examined.** D21-43 N22:
      *"Seed 5 is **replaced by the registered procedure** before dispatch."* The procedure is the 2026-09-09
      entry's: *"If a seed goes red it is replaced from the remaining 27 asset-only commits and this amendment is
      extended, never quietly edited."* A candidate must meet **all five** conditions:
      1. It is an asset-only commit from the pool below, and not one of the five items' commits.
      2. Its own message states the defect it fixes. The seed reverts its own asset or assets to its immediate
         parent, in a tree at that commit, as seeds 4 and 5 did (`git checkout R^ -- <each asset R changed>`), so
         every asset it changed exists at its parent.
      3. It shares the registered seeds' selection basis, a perceptual defect. The 2026-09-09 entry's selection
         threat applies unchanged and is restated: a reviewer good at perceptual defects and bad at others scores
         well here on a non-representative sample.
      4. Its constructed `spec.md` has a clause that the seeded state violates, and that clause is named before
         dispatch. This rules out the defect the entry found in seed 5: *"Seed 5 violates no registered AC of
         8.5.5g"*.
      5. Its seeded tree is green at its own commit (`verify_headless.ps1`, `failed` = 0 and exit 0, in a greenness
         copy), and every LFS object it needs is present locally under BrittonGames' `.git/lfs/objects`.

      **The pool command.** An asset-only commit is a non-merge commit reachable from the pin
      `041ead2cb284afd3004d07c2826f31d05b2799fd`. It changes at least one `.uasset` or `.umap` path. Every path it
      changes is either such an asset or inside item 3's strip set (`CLAUDE.md`, `_workspace/backlog/`,
      `_workspace/references/UE58_Editor_And_MCP_Gotchas.md`), so its `text_diff` under item 6 is empty. The five
      items meet the definition. It is enumerated by: for each `c` in `git -C D:/repos/BrittonGames rev-list
      --no-merges 041ead2cb284afd3004d07c2826f31d05b2799fd`, take `git diff-tree --root --no-commit-id -r
      --name-only c` and apply the test above.

      **The pool does not count 29, and the difference is recorded.** The 2026-09-09 entry's 29 was recorded as
      *"[measured]"* without its command. Under the definition above, the pool counts **148** of 575 non-merge
      commits, all five items included, so 143 are open to selection. That was measured 2026-10-02 by a count-only
      script that printed no candidate's id or message, so no candidate was examined in taking it. The definition is
      not narrowed to reach 29, because any narrowing chosen now would be chosen against a count rather than
      registered before it. Conditions 2–5 do the selecting.

      **The operator picks** among the candidates that meet the rule. The extension lists every candidate examined,
      with the reason each was passed over, and the operator's pick. Item 5 is then that commit R, reviewed at R with
      its seed applied. `bf9c499a1` is recorded as replaced, and correction 5 (item 1) and the entry's own
      *"Seed 5 violates no registered AC of 8.5.5g"* are the reasons.

  **Extension, and the one exception after dispatch.** A seed that goes red at its greenness check, the replacement
  included, is replaced under item 12, and this entry is extended before B2. After the first Opus dispatch no seed,
  item, rendering or ceiling moves, with **one registered exception, B4**. §3.4's branch 2 says *"add the field,
  re-run that item"*, so a B4 amendment adds the named field to that item's rendering for its re-run only. It is
  committed before B4's go-ahead, and the pass's original records stand. Any other change after that point discards
  the set, as the 2026-09-05 entry did, and the set is re-registered.

  **The go-ahead lines** for B1–B4 are written by the operator into ledger entry D21-141, reserved for S-BIN, in the
  form `s8i` §8i.5 fixes. The B1 line names Checkpoint 8h's hash, the dry run's SHA-256 and this entry's commit.

  **Deferred extensions: exactly these are appended later, each as its own dated paragraph citing this entry, and none
  edits it.**
  1. **Seed 5's replacement R**: the candidates examined, each with the reason it was passed over; the operator's
     pick; R's full sha and parent; R's assets and their LFS objects shown present; R's constructed `spec.md` clause
     that the seed violates; R's leak tokens with their zero-hit scan over all five stripped trees; and R's
     `$ARGUMENTS` under item 2's rule. Also the result of applying item 3's `CLAUDE.md` rule at R: if R's `CLAUDE.md`
     does not carry the four rows exactly once each, the strip refuses, and that is recorded there and not worked
     around.
  2. **The ceiling**: `W_max` and each tree's build-plus-suite wall clock, with the `ceiling_secs` item 7's rule
     gives.
  3. **`SBIN_NEED_BYTES`**: the base tree's size, the growth one greenness run left behind, and the resulting figure
     under item 4's rule.
  4. **The greenness results**: each tree's `failed`, its exit and its report's SHA-256. Also the pin's 2026-09-05
     report copied with its SHA-256, which must be `c3dae802…6bc6`.
  5. **B4's amendment**, only if a pass fires branch 2: the named field, its arm label `b-typed-sbin-plus-<field>`
     and the re-run's rendering.

  `review_root` is fixed above as `D:/sb` and is not deferred. This entry post-dates no S-BIN record.

- **s8i pre-dispatch amendment, extension: seed 5's replacement R, D21-142's second `CLAUDE.md` rule, the five base
  repositories, and the measured half of deferred extensions 3 and 4 — 2026-10-03, before B1; no S-BIN record
  exists** (cites the *s8i pre-dispatch amendment* above, its items 3, 4, 6 and 12 and its deferred-extensions list;
  edits nothing in it). Written in the AFK build, package S8I-F1, from the operator's ruling in
  `spikes/DECISIONS-2026-09-21.md` D21-142 and measurements read-only on `D:/repos/BrittonGames` (HEAD `ee6e450`, its
  `git --no-optional-locks status --porcelain` sha256 `a6751b0c…69ed`, the same before and after every build).

  **1. Deferred extension 1, seed 5's replacement R.** The pool, enumerated by the registered command (for each `c`
  in `git rev-list --no-merges 041ead2cb284afd3004d07c2826f31d05b2799fd`, `git diff-tree --root --no-commit-id
  --no-renames -r -z --name-status c`, kept when it changes at least one `.uasset` or `.umap` and every other path
  lies in item 3's strip set), counts **148 of 575** non-merge commits, the five items included, so **143** were
  examined [measured twice: package S8I-P3, and an independent re-count by S8I-F1 → 148, 575, 5 items inside]. The
  2026-09-09 entry's 29 is not reproduced, and the difference of 119 is recorded, not narrowed away. Why each was
  passed over: 81 add an asset absent at their parent (condition 2). Of the other 62, 49 predate
  `_workspace/tooling/verify_headless.ps1` (condition 5), and none of the 49 states its defect in its own message. Of
  the 13 left, 10 fail condition 2's message half or condition 3: `89e3605b0` (names no defect it fixes; the defect
  it states is seed 4's `LoadoutFill`, and it changes seed 4's asset), `4bf388081` and `8a45c29c3` (resaves),
  `897c2dbe7` (its stated defect is one it says is not resolved), `1a5e40b00`, `3247588c9` and `a61e1f76a`
  (features, functional), `e2c7b7b74` (a functional defect, an empty EQS query), `ebc4e95e8` (deletes a non-compiling
  duplicate) and `352b68b45` (untracks mistaken assets). Three met conditions 1–4 and the LFS half of condition 5,
  all story 2.8.5, 2026-07-30: `9ca831a42` and `6a101bdcd` (`WBP_GameVitals.uasset`) and `f6269ae7e`
  (`BP_GamePlayerCharacter.uasset`). The per-candidate table with each condition's reading is
  `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-P3-seed5-candidates.md`, outside every repository, and goes into the
  corpus's `results/s-bin/` at the corpus commit.
  - **The operator's pick (D21-142 item 2):** *"6a101bdcd white-on-white"*. R =
    `6a101bdcd2954768c498ac19b4dd161a5bbd8de1`, parent `9ca831a420e38571a42a7101477c793cb69e499c`, *"fix(2.8.5): make
    the vitals styling actually persist to disk"*. It changes `OurGame/Content/UI/HUD/WBP_GameVitals.uasset` (M),
    `_workspace/backlog/STATUS.md`, `_workspace/backlog/game_foundation_backlog.json` and
    `_workspace/references/UE58_Editor_And_MCP_Gotchas.md`, so its one asset is `WBP_GameVitals.uasset` and every
    other path is in the strip set.
  - **R's asset and its LFS objects, present.** At R the pointer blob is `c84ea7acc32c9ff499b6ee6ad3444b3692e96f18`
    (oid `59aa3581fb001476ec5a3d157f6e80e452c2284cfb8f4a0c2536f0af01538510`, size 84,345). At R^ it is
    `23c79a6337215a7a208d55676081f0fc33fcbeb7` (oid `86123be78c4566f951f87f84f19c1adf7890dccf4fce1f725409014a62b0261b`,
    size 74,460). Both objects are in BrittonGames' `.git/lfs/objects`, and each file's SHA-256 and size equal its
    pointer's; an all-zero oid reads absent (the control).
  - **The constructed `spec.md` clause the seed violates** (as S8I-P3 recorded it): *"The vitals widget's styling
    persists in the saved asset: on a cold load both the authoring tree and the generated class carry the fill
    texture, fill colours, Grenze font and retainer material, and the HUD renders a dark panel with legible labels,
    not white-on-white."*
  - **R's leak tokens:** `white-on-white`, `VERIFIED COLD` and `entirely white`, the words its own diagnosis uses.
    With item 4's `LoadoutFill`, the four registered tokens count **0** over every non-asset file, tracked or not,
    of every one of the five built base trees [measured over the work trees, `.git` excluded: 3,643, 3,650, 3,666,
    3,643 and 3,243 files at items 1–5].
  - **`$ARGUMENTS` under item 2's rule:** `/Game/UI/HUD/WBP_GameVitals — write the capture_scene_preview of the k-th
    asset above to _s-bin/captures/capture-<k>.png`.
  - `bf9c499a1` is recorded as replaced, for correction 5 (item 1) and the 2026-09-09 entry's *"Seed 5 violates no
    registered AC of 8.5.5g"*. Greenness at R, with the seed applied, is the operator's UE run (deferred extension 4).

  **2. Item 3's `CLAUDE.md` rule at R refuses, as the deferred-extensions list provides, and D21-142 registers a
  second rule.** BrittonGames' status table carried the rows *Epic / Feature / Focus / Next* until `07fe80ab6`
  (2026-08-07). At R item 3's four prefixes count 1/0/0/0, so that strip refuses, and the refusal is recorded here
  as the list requires [measured: `git show 6a101bdcd:CLAUDE.md`, 19,328 bytes, 0 CR]. The operator answered
  *"Add old-table strip rule"* (D21-142 item 1). The **second byte rule** is item 3's rule word for word with
  other prefixes: read `CLAUDE.md` as bytes and split it into lines, each keeping its terminator; strip one trailing
  `\n`, then one trailing `\r`, from a copy of the line; the line matches if that copy begins with exactly one of
  the four byte prefixes `| **Epic** |`, `| **Feature** |`, `| **Focus** |` or `| **Next** |`. Matching lines are
  removed with their terminators and every other byte is left unchanged. It **asserts exactly 4 rows removed, each
  prefix matched exactly once**, and refuses otherwise. It applies **only** to a tree whose `CLAUDE.md` predates
  `07fe80ab6`, decided as `git merge-base --is-ancestor 07fe80ab6 <item commit>` → rc 1. That is item 5 alone: the
  check reads rc 0 at items 1–4, which keep item 3's rule unchanged, and rc 1 at R. At R's blob the four prefixes
  count 1/1/1/1. The work tree's `CLAUDE.md` is CRLF (111 CRs, BrittonGames' `*.md text eol=crlf`), 19,439 bytes
  before and 14,663 after, so the rule ran on CRLF bytes. Controls: it refuses 3 rows, a duplicated row and the new
  table, and strips LF, CRLF and mixed input alike. The strip set's entry is still `CLAUDE.md#status-rows`, so the
  set stays one set across the five. The rule changes fixture construction, not selection, and it stands on
  D21-142's operator line because it was registered after candidates were examined.

  **3. The five base repositories (item 4).** Each is the one commit of a fresh repository on `main` (`review tree`,
  `s-bin <s-bin@invalid>`), with no remote, no other ref and no tag, and `git rev-list --all --count` reads 1 on each
  [measured 2026-10-03]. Each was checked out of a disposable `git clone --no-checkout --shared` of BrittonGames,
  the LFS smudge skipped and `core.autocrlf=false`. Every LFS pointer was replaced by its object from BrittonGames'
  own store, and each materialised file's SHA-256 and size equal its pointer's. Each committed tree equals
  BrittonGames' item tree entry for entry, with the seed applied and the strip removed, and `git status --porcelain
  --untracked-files=all --ignored` prints nothing.

  | item | item commit | base commit | base tree | LFS objects | base bytes (`.git` included) |
  | --- | --- | --- | --- | --- | --- |
  | 1 | `cc0cf2d01` | `85d73729bc908b6dce10eb40df8fa0ac27816ca1` | `59b90c8635c537a4cb89f8231eb1e05b7f9052e4` | 30,014 | 21,540,846,625 |
  | 2 | `fa1ea72f4` | `78a5ed29a4ac93d3b7685e35c6d6b9cebe7112a8` | `ca9a46bb0795acafc7338e91d2677e3ead0a391e` | 30,015 | 21,541,178,231 |
  | 3 | `e83463288` | `536eb55f3c3529bf5943b6c8ecf06da873824c8e` | `81e0c57073926d884556fb0037a3686ab16c8619` | 30,022 | 21,560,069,565 |
  | 4 | `a71265925` | `fd692b75ac96c6d94bf1de084e64bb03c422aaba` | `59b90c8635c537a4cb89f8231eb1e05b7f9052e4` | 30,014 | 21,540,845,865 |
  | 5 | `6a101bdcd` | `292cabad7e073a977010e7e84261cd4ad204389e` | `143377a3878f9e40a5d937826da9e0ffe9e0d8f8` | 11,886 | 10,604,498,965 |

  - **The seeds, by blob oid.** Seed 4: `a71265925^:OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset` =
    `9415f0986275a66d01885cc74f58c33e1957e183` = base 4's `HEAD:` blob. Seed 5:
    `6a101bdcd^:OurGame/Content/UI/HUD/WBP_GameVitals.uasset` = `23c79a6337215a7a208d55676081f0fc33fcbeb7` = base 5's
    `HEAD:` blob.
  - **The fix, unreachable.** Base 4 holds neither the fixed pointer blob `70e2a2e3f05d5b4c27930888050e762c1b3aa37c`
    nor its LFS object `02b53079c9fe24bbafe2603b2f99185ad4a62cfa780515851257479b1b8397db`. Base 5 holds neither
    `c84ea7acc32c9ff499b6ee6ad3444b3692e96f18` nor `59aa3581fb001476ec5a3d157f6e80e452c2284cfb8f4a0c2536f0af01538510`.
  - **The strip.** One set on all five, `CLAUDE.md#status-rows`, `_workspace/backlog/` and
    `_workspace/references/UE58_Editor_And_MCP_Gotchas.md`. It removed 19 files from each of bases 1–4 and 8 from base
    5, because R's `_workspace/backlog/` holds 7 files, not the 18 item 3 measured at the item commits. The set is
    the registered one and is not narrowed.
  - **Measured, not chosen.** Bases 1 and 4 hold the same tree, because `a71265925^` is item 1's commit and
    `a71265925` changes only `WBP_LoadoutScreen.uasset` plus stripped files. So clean item 1's tree carries seed 4's
    defect. This is the registration's own consequence.
  - **Tracked-but-ignored files.** BrittonGames tracks files its own `.gitignore` matches (`git ls-files -ci
    --exclude-standard`): 11 at items 1–4 and 10 at R. A plain `git add -A` drops them, so each base was committed
    with `git add -A -f`, and the entry-for-entry comparison shows the commit is exactly the tracked set. The
    driver's `commit_single` now force-adds exactly the base tree's tracked set and refuses any other ignored file
    (package S8I-F1's driver patch).

  **4. Item 6's pointer, as the driver reads it.** *"`size` and `lfs_oid` are read from the LFS pointer in the
  review tree"*: the review tree's work file holds the materialised asset, so that pointer is the blob committed in
  the review repository, read as `git cat-file blob HEAD:<path>`. This is how item 6 reads, unchanged, and the driver
  was corrected to it (package S8I-F1; S8I-P8 found the work-file read).

  **5. The measured half of deferred extensions 3 and 4.** The largest base is item 3's, 21,560,069,565 bytes. The
  growth one greenness run leaves is not measured yet, so `SBIN_NEED_BYTES` is not written here. The pin's
  2026-09-05 report is staged for the corpus at `results/s-bin/pin-report-2026-09-05/index.json`, SHA-256
  `c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6`, as registered. It was taken from
  BrittonGames' `_workspace/_runs/8.3.3g-20260904-2334/final-suite.json`, byte-identical to the report measured at
  authoring, because the registered source `_workspace/tooling/TestResults/index.json` was overwritten on 2026-09-25
  and now hashes `65960f789fc2d6216ced3d84b387f3b1d4a78588fd47488d3d707c9553cc1a8a`. Accepting that source is the
  operator's at the corpus commit.

  **Not appended here, and owed before B1/B2 in their own dated paragraph:** deferred extension 2 (`W_max` and
  `ceiling_secs`), the growth and figure of extension 3, and extension 4's greenness results, all of which wait for
  the operator's five greenness runs. No placeholder is written for them.

  **The go-ahead entry's id.** The amendment's *"ledger entry D21-141, reserved for S-BIN"* reads **D21-143**.
  D21-141 was taken first by the Checkpoint 8i (8) ruling, and D21-141's own id note says the S-BIN entry *"takes the
  next free id when it is written"*. D21-142 was taken next, so the go-ahead lines are in D21-143.

- **s8i pre-dispatch amendment, extension: B4's re-run keyed on its arm, and how the driver reads items 5, 6, 7 and
  10 after `s8i` refuting round 1 — 2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch
  amendment* above, its items 3, 4, 5, 6, 7 and 10 and its *"Extension, and the one exception after dispatch"*
  paragraph; edits nothing in it or in the extension above). Written in the AFK build, package R1FIX-8i.

  **1. B4's re-run, keyed on its arm.** Item 4's table names the dispatch copy `D:/sb/d/p<pass>i<n>a<attempt>`, and
  item 5 names a B4 re-run's arm `b-typed-sbin-plus-<field>`. Under the pass's form a re-run would claim the path the
  pass's own record used, which item 4 refuses (*"which no earlier record and no capture copy used"*). So a B4
  re-run's copy is `D:/sb/d/p<pass>i<n>a<attempt>-plus-<field>`, its raw transcript is
  `results/s-bin/raw/p<pass>i<n>a<attempt>-plus-<field>.jsonl`, and its rendering is
  `results/s-bin/rendered/item-<n>-plus-<field>.txt`. The pass's own copy path, transcript and rendering keep item 4's
  and item 6's forms and never move. `<field>` is `[A-Za-z0-9_]+`. The re-run's rendering is the pass's summary with
  one key added, `<field>`, whose value `fixture.json` carries as `items.<n>.b4_fields.<field>`. A field the summary
  already renders refuses, after the paragraph's *"adds the named field"*. The driver dispatches a re-run only after
  the pass's valid record for that item with `detected` false, and resume keys it on its own arm (item 10). Deferred
  extension 5 still registers the named field, its arm label and the content of the re-run's rendering when branch 2
  fires. This paragraph registers only where they go.

  **2. How items 5, 6, 7 and 10 read in the driver, unchanged.**
  - Item 5, *"a record reporting a second id halts the set"*: every record that carries an `observed_model`, voids
    included, as Checkpoint 8i (4)'s `model` line counts them.
  - Item 6, the constructed `spec.md` and report *"rendered as E3 renders its spec and report artifacts"*, the summary
    *"in place of E3's 'files this change was scoped to touch' list"*: the brief is E3's own `build_prompt` text, its
    24,000- and 16,000-character truncation included, with only the file-list slot replaced, under the header *"This
    change's binary assets cannot be diffed. Its ChangeSummary:"*. The earlier driver had re-worded two of E3's phrases
    and dropped the truncation. No rendering had been hashed.
  - Item 7, *"the same for every dispatch and every retry"*: a real pass reads `fixture.json`'s `ceiling_secs` and
    refuses before any spawn when it is absent or differs from `--timeout-secs`. The dry run, which comes before
    extension 2, runs at 3600 s.
  - Item 10, the caps *"counting voids"*: unchanged, and now tested over voided records. Item 10's *"The same item
    voiding twice in one pass is a fixture defect, and it stops the stage"*: the run that writes the second void halts
    once it is written, so no later item in that run is spawned.

  **3. Recorded, not changed: the status rows are not stripped identically.** One registered set,
  `CLAUDE.md#status-rows`, is stripped from all five trees, by two byte rules. Items 1–4 lose the
  `Focus/Tier/Verify/Progress` rows and keep `| **Epic** |` and `| **Feature** |`. Item 5 loses `Epic/Feature/Focus/Next`
  under D21-142's rule [measured: `git -C D:/sb/base/item-<n> show HEAD:CLAUDE.md | grep -cF` → `| **Epic** |` and
  `| **Feature** |` 1 and 1 at items 1–4, 0 and 0 at item 5]. D21-43 N20's *"stripped from all five trees
  identically, and recorded"* holds for the set's name, not for the bytes removed, and this is its record. Whether to
  widen item 3's rule is the operator's.

- **s8i pre-dispatch amendment, extension: how items 4, 5 and 11 and §3.4's *"Recorded"* read in the driver after
  `s8i` refuting round 2 — 2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch amendment*
  above, its items 4, 5 and 11, and §3.4; edits nothing in it or in the two extensions above). Written in the AFK
  build, package R2FIX-8i.

  **1. Item 11, *"its launch record's `CLI_VERSION`"*.** The launcher's record (s8h §8h.7 step 8) holds the version as
  `CLI_VERSION_BEFORE` and `CLI_VERSION_AFTER`. `CLI_VERSION=` is printed on the launcher's stdout only, from the same
  reading [measured: `tools/live-run.sh` writes `printf 'CLI_VERSION_BEFORE=%s\n' "$cver"` into the record and prints
  `printf 'CLI_VERSION=%s\n' "$cver"`]. So the item reads as that record's `CLI_VERSION_BEFORE`. The driver reads it
  from the record that `REDKILN_LAUNCH_RECORD` names, before each dispatch. It refuses before the spawn in two cases:
  a real dispatch with no launch record, since item 5's argv runs *"through the launcher's `wrap` mode"*; and a record
  whose version differs from the `<claude-bin> --version` read before that dispatch. Every record carries the two
  values as `launch_record` and `launch_cli_version`. The dry run is not a `wrap` child, and its record carries both
  as null.

  **2. Item 11's set-level check runs before each dispatch.** The version is read by `<claude-bin> --version` before
  each item's dispatch within one invocation, not once per invocation, and every halt stops the invocation before the
  next item.

  **3. Item 5's `fix_unreachable` is true by construction.** The driver checks each dispatch copy for every fix blob and
  LFS object, and for a single commit. Either miss halts before the spawn, and no record is written. So every record
  written carries `true`, which is the item's *"true only when the dispatch copy passes item 4's checks"*.

  **4. §3.4's *"whether the reviewer ran the automation itself"*.** `ran_automation` counts a `Bash` command that
  invokes the suite, not one that names it. A segment counts when its program is `pwsh` or `powershell` and it names
  `verify_headless`, when its program is the `verify_headless.ps1` script itself, or when its program is
  `UnrealEditor-Cmd` and it carries `Automation RunTests`. Reading the script with `cat`, `grep` or `ls` is not a run.

  **5. Not changed, and recorded: item 4's per-copy free-space check is not in the driver.** Item 4's *"Before each
  copy, the free space on the root's volume must be at least `SBIN_NEED_BYTES`"* stands as registered. The driver does
  not run it, and `s8i` §8i.5's batch pre-check reads the free space once per batch. Building the per-copy check, or
  registering that the batch check carries it, is owed before B2 (`s8i` §8i.5's owed item (f)). Choosing between them
  narrows or keeps a registered check, so it is not taken here.

- **s8i pre-dispatch amendment, extension: how item 4's out-of-tree rule and item 5's B4 argv read in the driver, and a
  column label corrected, after `s8i` refuting round 3 — 2026-10-03, before B1; no S-BIN record exists** (cites the
  *s8i pre-dispatch amendment* above, its items 4 and 5 and its deferred extension 5, and the extension of 2026-10-03
  that lists the five base repositories; edits nothing in it or in the three extensions above). Written in the AFK
  build, package R3FIX-8i.

  **1. Item 4's out-of-tree rule, as the driver applies it.** Item 4 reads *"A reviewer that reads or runs `git`
  against a path outside its review tree voids the record"*, and names *"every other copy under `D:/sb`"* among the
  answer-bearing paths. `s8i` §8i.3 records the driver's check. Each string is folded (`\` to `/`, lower case) and
  checked by name against the answer-bearing roots, and against *"any path under the review root … other than the
  dispatch copy's own"*. Before that test the driver now normalises each path [measured: refuting round 3 found that
  `<copy>/../../base/…`, `../../base/…` and a doubled-backslash `D:\\repos\\BrittonGames` each read as in-tree]:
  - Every run of `/` is collapsed to one, in the string, in the review root and in the copy path.
  - `.` and `..` segments are resolved, for each root spelling found in the string and for each absolute path.
  - A relative path is resolved against the child's working directory, which starts at the dispatch copy. A `cd` or
    `pushd` in a `Bash` command moves it, and the move persists into later `Bash` calls. A `cd` in any other string
    moves only that string's own reading.
  - A relative path voids if it resolves under the review root outside the copy, or under an answer-bearing root. It
    does not void if it stays in the copy or resolves anywhere else.

  This is how item 4's rule reads. It names no new answer-bearing path, and it voids nothing the rule does not cover.
  One residual is disclosed: a bare `..` token in any tool input, a finding's prose included, resolves out of the copy
  and voids.

  **2. Item 5's argv for a B4 re-run.** Item 5 registers B2/B3's argv and the B4 arm label, and deferred extension 5
  registers B4's field. A re-run covers one item. Its argv is B2/B3's with `--arm b-typed-sbin-plus-<field> --rerun
  --only-item <n>` added, and it runs once for each (pass, item) that fired branch 2. Before anything renders, the
  driver refuses a re-run with no `--only-item`, and any `--only-item` outside 1–5.

  **3. A column label corrected.** In the extension that lists the five base repositories, the column *"LFS objects"*
  (30,014 / 30,015 / 30,022 / 30,014 / 11,886) counts LFS pointer paths, each checked against its object. It does not
  count distinct objects. The distinct objects number 29,576 / 29,577 / 29,584 / 29,576 / 11,447 [measured
  2026-10-03, read-only: `find D:/sb/base/item-<n>/.git/lfs/objects -type f | wc -l`, which equals the count of
  distinct oids in `git lfs ls-files -l`; the pointer paths are `git lfs ls-files | wc -l`]. This file is
  append-only, so the table is not edited. Read its column as *"LFS pointer paths"*.

- **s8i pre-dispatch amendment, extension: which tool inputs item 4's out-of-tree rule reads, after `s8i` refuting
  round 4 — 2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch amendment* above and its
  item 4, and the extension written after refuting round 3, its item 1; edits nothing in them or in the other
  extensions above). Written in the AFK build, package R4FIX-8i.

  **1. Two sentences withdrawn.** The round-3 extension's item 1 says the driver *"voids nothing the rule does not
  cover"*, and that *"One residual is disclosed: a bare `..` token in any tool input, a finding's prose included,
  resolves out of the copy and voids"*. Both sentences are withdrawn. That driver audited every string of every tool
  input. That included the digest, which arrives as a `StructuredOutput` tool use, and a `Bash` `description`. So a
  quoted `#include "../Public/…"`, a relative import in a repro, a quoted line from the tree's own
  `_s-bin/report/headless.log`, or the copy's own path followed by a full stop each voided. None of them is a read.
  Measured by a replay of the corpus's own records [measured 2026-10-03, read-only: the round-3 driver's
  `out_of_tree_reads` run from inside a test binary over every record in `D:/repos/redkiln-corpus/results/*.jsonl`
  that carries reviewer tool uses, with the digest as the `StructuredOutput` input. The copy was `D:/sb/d/p1i1a1` and
  the root `D:/sb`]: **59 of 754 records voided (7.8%), and every one of those voids came from a digest's prose.**

  **2. What is read.** Item 4 reads *"A reviewer that reads or runs `git` against a path outside its review tree
  voids the record"*. From this extension on, the driver audits only the inputs through which a tool reads. A
  mention is not audited.
  - **`Bash`: the `command` string only.** It is lexed as a shell command: quotes are grouped, `;`, `&`, `|` and a
    newline end a command, and `(` or `$(` opens a subshell.
    - Audited: each command's program; every operand, except as below; the file after `<` or `>`; and the absolute
      paths in a heredoc's body or a here-string.
    - Not audited: the operands of `echo` and `printf`, which are output text; and a grep's pattern, which is its
      first operand or each `-e` value, for `grep`, `egrep`, `fgrep`, `rg`, `ag`, `ack`, `zgrep`, `findstr` and `git
      grep`. A grep's files and its `-f` file are audited.
    - The cwd starts at the dispatch copy. A `cd` or `pushd` moves it, and the move persists into later `Bash` calls.
      That holds when the `cd` follows `then`, `do`, `else`, `{`, `!` or an assignment, and when it carries options
      such as `--`, `-P` or `-L`. `popd` and `cd -` move it back. A `cd` inside `( … )` or `$( … )` lasts until the
      `)`.
  - **Path fields:** `Read`'s `file_path`, `Glob`'s `pattern` and `path`, and `Grep`'s `path` and `glob`. For any
    other tool, its `file_path`, `notebook_path` and `path` are audited.
  - **Never audited:** the `StructuredOutput` digest, a `Bash` `description`, a `Task` or `Agent` prompt, a `Grep`
    tool's `pattern` (a regular expression over file contents, the same as a grep's pattern), and every other field.

  This narrows what the driver voids under item 4. It does not change the rule's words. It is written under the AFK run's
  standing request, and the operator confirms it before B1.

  Each audited word is folded and resolved as the round-3 extension's item 1 says. Before that, a trailing `.`, `,`,
  `:`, `;`, `!`, `?`, `)`, `]`, `}`, `*` or quote is dropped, so `<copy>.` names the copy. A `..` keeps its dots.

  **3. The child's own saved output is in-tree.** When a tool's output is large, the CLI saves it under
  `<account>/projects/<slug of the cwd>/<session>/tool-results/`, and the reviewer may read it there. A path that
  resolves under `<account root>/projects/<the dispatch copy's slug>/<one segment>/tool-results/` does not void. That
  holds for each account root the driver already knows: `default_account_roots`, i.e. `~/.claude-personal`,
  `~/.claude`, and `CLAUDE_CONFIG_DIR` when it names a third (`s8i` §8i.3). The copy's path is never used before
  (§8i.4), and `precheck_no_project_dir` proves its slug absent before the spawn. Every other slug still voids, a
  capture session's `D--sb-cap-item-<n>` included. So does the copy slug's project directory outside `tool-results`,
  and a path that climbs out of `tool-results` through `..`.

  **4. The greenness log and the base's own files.** A finding that quotes the review tree's
  `_s-bin/report/headless.log`, whose lines name the greenness copy `D:/sb/green/item-<n>`, does not void. Neither does
  a finding that quotes `CLAUDE.md`'s `rootFolder` line, which names `D:\repos\BrittonGames`. Both are mentions, and
  so are an `echo` of either and a grep whose pattern quotes either over in-copy files. A `Read`, `cat`, `ls` or `git
  -C` of either path is a read, and it voids.

  **5. The measured false-void rate under this scoping** [measured 2026-10-03, read-only; the same replays, run from
  inside the test binary under the no-dispatch wrapper]:
  - **Corpus records: 0 of 754** (the round-3 driver: 59 of 754).
  - **Reviewer transcripts.** The replay read every session transcript under
    `~/.claude-personal/projects/D--repos-redkiln-rs-spikes-fixtures-*` (the E3, P7 and P8 fixture sessions). Each
    session's fixture directory was mapped onto the copy, and its project slug onto the copy's slug. That is 838
    sessions and 13,147 tool uses, and 779 of the sessions use only the card's tools. The round-3 driver voided 218 of
    them. This driver voids 69, every one card-shaped, and each of the 69 is a real read outside the fixture:
    - 68 name a `D:/repos/redkiln-rs` path outside the session's own fixture in a read position. These are `ls` or
      `diff` of the sibling fixtures, `cd` and `git` in the repository, a `Read` of a mistyped
      `fixtures/e3-clean/issue-…` path, a `find /d`, and a script run from `spikes/scratch`.
    - The 69th runs `ls ..` and `cd ../.. && diff` against its sibling fixtures.

    The two sessions that read their own saved output under `tool-results` no longer void.

  **6. Residuals, disclosed.**
  - A relative path inside inline code is not resolved. That covers `python -c`, `node -e` and a heredoc's body:
    a relative import there is relative to its file. An absolute path in a heredoc's body is still read, even when the
    body is only text written to a file.
  - `cd` with no operand, which goes to `$HOME`, leaves the cwd unchanged.
  - A path inside a `git log --grep=<x>` value is audited as a path.
  - The cwd model assumes the `Bash` tool keeps a `cd` that leaves the copy. Whether the CLI resets the cwd to the
    project directory in that case is not measured here. A read-only probe of the CLI binary for it was refused by the
    session's permission classifier. A reset would make a later relative read resolve against the wrong directory.
  - The separator collapse in `norm_sep` is no longer observable: every audited word is resolved before the
    containment test, and `resolve_path` drops empty segments. Its mutant from the round-3 extension survives for that
    reason. A `Bash` command's own collapse (`sep_keep_case`) is observable through a heredoc's body, and its mutant
    flips.

- **s8i pre-dispatch amendment, extension: item 4's out-of-tree rule after `s8i` refuting round 5 — the runtime void's
  four additions, the roots a measured seed sweep names, and a shadow audit that holds a flagged record unscored —
  2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch amendment* above and its items 4 and 5,
  and the extension written after refuting round 4, R4FIX-8i, its items 2, 3, 5 and 6; edits nothing in them or in the
  other extensions above). Written in the AFK build, package R5FIX-8i, under `spikes/DECISIONS-2026-09-21.md` D21-144
  (TAKEN PROVISIONALLY under D21-105; panel 2-1). It is the operator's to confirm before B1, beside R4FIX-8i.

  **1. Why.** Refuting round 5 measured reads that R4FIX-8i's reads-only audit did not void:
  - a path carried by a shell assignment and read through `$VAR`, e.g. *"`BG='D:\repos\BrittonGames'; git -C "$BG"
    log`"*;
  - a recursive read of an ancestor of a registered root, e.g. `grep -rn LoadoutFill /d/repos`, or the `Grep` tool on
    `D:/repos`;
  - copies of seed-stating files outside the registered roots (`D:/r6c`, `D:/repos/redkiln-sev`);
  - the `/c/…` and `~/…` spellings of the session's own `tool-results`, which voided an honest read.

  **2. The runtime void: it voids the record, and §8i.6's second-void halt applies.** It is R4FIX-8i item 2's
  reads-only lexer, with four additions. None changes item 4's words.
  - **(a) Assignments.** A bare `X=val` and a prefix `X=val cmd` are tracked, and so are `export`, `local`,
    `declare`, `typeset` and `readonly`. A bare assignment persists into later `Bash` calls, as the cwd does. A prefix
    assignment is seen only by its own command's words. `$X`, `${X}` and `"$X"` in a read position are expanded
    before the path test. **The assignment's value is itself audited as a read-position word.** R4FIX-8i item 2
    exempts only `echo`/`printf` operands and a grep's pattern, and never exempted an assignment. A `$` inside single
    quotes is literal. A descriptor written against a redirect (`2>`, `0<`) is not a word [measured: in the replay's
    `cp dist/cli.js /tmp/d_cli.bak 2>/dev/null`, the `2` was read as `cp`'s destination]. A `$VAR` that no assignment
    set does not void
    at runtime; it goes to the shadow audit (item 5).
  - **(b) The ancestor rule, as a reading of item 4's existing words** (*"reads … a path outside its review tree"*).
    A recursive read rooted at a registered root, or at an ancestor of one, or at an ancestor of the review root,
    reads that root. The recursive readers are:
    - `grep`, `egrep`, `fgrep` and `zgrep` with `-r`, `-R` or `--recursive`; `rg`, `ag` and `ack`, which recurse by
      default;
    - `find`, `ls -R`, `tree` and `du`;
    - `git grep --no-index` (its paths), and `git -C <dir> grep` (`<dir>`);
    - the `Glob` tool (its `path`, or its pattern's literal directory) and the `Grep` tool (its `path`).
    A walk with no root operand walks the cwd. The registered roots for this test are the review root, each account
    root's `projects/`, `D:/repos/redkiln-rs`, `D:/repos/redkiln-corpus`, `D:/repos/BrittonGames`, and the sweep's
    roots (item 3).
  - **(c) One spelling before every containment test.** Each word is folded (`\` to `/`, lower case), then `~`,
    `$HOME`, `$USERPROFILE` (and their `${…}` forms) become the home directory; `$TEMP`, `$TMP` and `/tmp` become the
    temp directory (`/tmp` is `%TEMP%`: `cygpath -w /tmp` → `C:\Users\ryanm\AppData\Local\Temp`); and `/mnt/<x>/…` and
    `/<x>/…` become `<x>:/…`. A path beginning with `/` has its `.` and `..` resolved first, because Git Bash
    resolves them before it maps a mount: `/tmp/../c/x` is `C:/x`, not the temp directory's parent [measured: the
    transcript replay's session `eec3efa3` read its own task output that way]. R4FIX-8i item 3's `tool-results`
    exemption is tested after the fold, so the `/c/`,
    `/mnt/c/`, `~`, `$HOME` and `$USERPROFILE` spellings of the copy slug's `tool-results` are exempt, and no other
    slug's.
  - **(d) The sweep's roots, by name** (item 3), checked like item 4's roots, in the `x:/`, `/x/` and `/mnt/x/`
    spellings, under a name boundary (`D:/r6cx` is not `D:/r6c`).

  **3. The sweep, and the roots it names** [measured 2026-10-03, read-only: GNU `grep -r -l -F` (symlinks and
  junctions below a start point not followed; binary files searched) over 5,066 start points on C: and D: (each
  drive's top-level entries, opened one level into `D:/repos`, `D:/references`, `D:/workspaces`,
  `D:/repos/redkiln-sev`, `C:/Users`, the home directory, its two account roots, `AppData`, `AppData/Local`,
  `AppData/Roaming` and `AppData/Local/Temp`), for 15 patterns: the four registered leak tokens `LoadoutFill`,
  `white-on-white`, `VERIFIED COLD` and `entirely white`; the seed blobs `9415f098…`, `23c79a63…` and seed 5's LFS oid
  `86123be7…`; the fix blobs `70e2a2e3…`, `c84ea7ac…` and their LFS oids `02b53079…`, `59aa3581…`; the item commits
  `a71265925` and `6a101bdcd`, and the replaced `bf9c499a1`; and the fixture's seed statement *"the seed reverts to"*.
  The scripts and every output are under `D:/repos/redkiln-sev/afk-tools/sp/s8i/` (`s8i_R5FIX-8i_sweep2.py`,
  `R5FIX-8i-sweep-out/`)].
  - **Skipped, by an explicit list only:** the OS and toolchain directories (`C:/Windows`, `C:/Program Files`,
    `C:/Program Files (x86)`, `C:/ProgramData`, the recycle bins, `System Volume Information`, `Recovery`, `PerfLogs`,
    the driver and redistributable folders, the page, hibernate and swap files); the profile's legacy junctions,
    whose targets are searched under their own names; and the roots item 4 already registers.
  - **The two OneDrive roots were not grepped**, because reading an online-only file downloads it. By metadata:
    `~/OneDrive` holds 1,040,560 files, 210 modified on or after 2026-07-01, of which the 48 stored locally were
    searched with no hit; `~/OneDrive - The IQ Business Group, Inc` holds 158,718 files, 5,466 modified since
    2026-07-01, of which the 5,430 stored locally were searched. 162 and 36 recent online-only files were not opened.
  - **Partial reads, named:** 30 start points ended with a grep error (permission denied on some files: browser,
    Windows and app caches under `AppData`, `NTUSER.DAT` and its logs, one repository). The grep of `<temp>/claude`
    did not finish (it holds this run's own task outputs and scratch, which grew during the scan); a hit there is
    confirmed directly, and the directory is registered whole.

  **397 files hold a hit. They are bounded by eleven roots, registered here by name:**
  - `D:/repos/redkiln-sev` (314 files: runbook and METHOD-S copies, the `s8i` scratch, the seed-5 candidates);
  - `D:/r6c` (4: a checkout of this repository at `b8e7aa9`);
  - `~/.claude-personal`, the whole account root (15 outside `projects/`: `file-history` copies of edited files);
  - `~/.claude-backup` (10: a copy of an account root whose `projects/D--repos-BrittonGames` holds that repository's
    session transcripts, which item 4's two account names do not cover);
  - `~/AppData/Local/JetBrains` (7: Rider and RustRover caches and indexes of this repository's files);
  - `~/AppData/Local/Packages/Claude_pzs8sxrjxfjjc` (21: the desktop app's IndexedDB, agent-session and VM data);
  - under the temp directory: `claude` (the CLI's task outputs and scratch), `tmp.uDZZzvzEF4` (copies of METHOD-S,
    the `s8i` stage file and two audit files), and the files `h.md`, `l143` and `ms0`.

  **Hits not registered, a judgement named here.** The ruling reads *"Register every directory holding a hit"*. 17
  hits state no seed. 16 are mypy's `report.py`, where `entirely white` is a substring of *"entirely whitespace"*, in
  virtual environments and caches under `D:/repos/{grove, inksmith, inksmith-archive, iqbg-bid-archive, morphica,
  praecepta, rb, redkiln - Copy, torquem}`, `D:/references/monarch-mcp`, `D:/workspaces/_recovery`,
  `~/AppData/Local/uv`, `~/My Drive` and Google Drive's content cache. One is a work-OneDrive client document that uses
  *"white-on-white"* as plain prose. Registering them would void no seed read and could only add false voids. The
  operator confirms or reverses this at item 4's gate.

  The copies are bounded, so no escalation to an allowlist void follows (D21-144: *"If the sweep finds copies it
  cannot bound, stop and escalate to B"*).

  **4. The child's own background-task output (PROVISIONAL, and returned to the orchestrator as a blocker the ruling
  did not foresee).** The CLI keeps a background command's output at `<temp>/claude/<cwd
  slug>/<session>/tasks/<id>.output`,
  as it keeps a large output under `tool-results`. Registering `<temp>/claude` voided **64 of the 838 replayed
  sessions** that read their own task output, each an honest read [measured, the transcript replay of item 7, before
  this exemption]. So, the R4FIX-8i item 3 shape: a path that resolves under `<temp>/claude/<the dispatch copy's
  slug>/<one segment>/tasks/` does not void, in every spelling, and no other slug's does, nor the own slug outside
  `tasks/`. The copy's path is never used before (§8i.4), so nothing there predates the dispatch; no pre-spawn check
  proves that directory absent, as `precheck_no_project_dir` does for `projects/<slug>`. The shadow audit still flags
  these reads, as D21-144's allowlist reads (305 reads in 77 sessions of the replay). Whether this exemption stands,
  whether the pre-spawn check should cover the directory, and whether the shadow allowlist should carry it, are the
  orchestrator's panel's and the operator's.

  **5. The shadow audit. It never voids, never retries, and never counts toward a cap or a halt.** It runs over the
  kept raw transcript's tool uses, through the same lexer with assignments expanded, and never over the digest. Each
  record carries `oot_shadow`: `{verdict: clear | flagged, flagged_reads: [...], allowlist_version}`. A read-position
  path outside the dispatch copy is flagged unless the allowlist covers it. Allowlist version **`r5fix-8i-1`**:
  - the copy slug's `tool-results`, in every spelling (item 2 (c));
  - `C:/Windows`; `C:/Program Files*`, a prefix, which holds `C:/Program Files (x86)`, the Git install and the engine
    install `C:/Program Files/Epic Games/UE_5.8`; and Git Bash's `/usr`, `/bin`, `/etc`, `/dev/null` and `/mingw64`
    (the Git install);
  - `~/AppData/Local/UnrealEngine`, the engine's caches, **listed provisionally**. The engine paths are measured from
    B1's capture transcripts and replace this entry by a later appended extension;
  - a temp path, **only** where the same session wrote that path earlier: the `Write` tool's path, a `>` or `>>`
    target, a `cp` or `mv` destination, or a `NAME=$(mktemp …)` variable. Temp is not allowed wholesale.
  A read-position word still holding an unresolved `$VAR`, `${…}` or `$(…)` is flagged. A heredoc's body is data:
  only its absolute paths are read, and its `$` words are not flagged.

  **A flagged record is held unscored.** The driver only records the field: `void`, `detected` and every halt are
  unchanged, and resume treats the record as done, so it is never retried. `audit-s.py bin` holds it. It takes an
  optional third argument, `results/s-bin/oot-shadow-rulings.jsonl`, one line per ruling: `{"raw_ref": <the record's
  raw_ref>, "ruling": "clear" | "void"}`. A cleared record is scored. An operator void is `void: out-of-tree`, shown as
  `out-of-tree(operator)`, and is never a miss. A held record is shown as `HELD:oot-shadow`, and its pass's outcome
  is `BAD-held`, which no checkpoint accepts. Checkpoint 8i (4) applies the same rule to the records it scores. The
  operator rules at item 4's confirmation gate before B1, and for records after B1 before that batch is scored. An
  operator void gets no retry during the AFK run and does not count toward §8i.6's *"the same (pass, item) voiding
  twice"*: the driver halts only on its own runtime voids.

  **6. Item 5's named tests.** Item 5's *"Its 25 named tests are listed in `s8i` §8i.3"* reads **33** from this
  extension on. The eight added, each watched red for its stated reason before its code:
  `an_assignment_carried_path_voids`,
  `a_recursive_read_of_an_ancestor_voids`, `a_tool_glob_or_grep_on_an_ancestor_voids`, `a_sweep_measured_root_voids`,
  `tool_results_c_and_tilde_spellings_are_exempt`, `the_shadow_audit_flags_reads_outside_the_allowlist`,
  `a_temp_read_written_earlier_in_the_session_is_not_flagged` and
  `an_unresolved_var_in_a_read_position_is_flagged_not_voided`. §8i.7's record gains `oot_shadow` (34 names).
  `runner_commit` is the commit that last changed `runner/` (round 5's minor), so a results-only corpus commit during
  the set leaves it one value.

  **7. The measured rates** [measured 2026-10-03, read-only: both replays run from inside the test binary under the
  no-dispatch wrapper, in a replay copy never shipped, over this extension's driver (`sbin.rs` sha256
  `bcdd084840e5…`); outputs `R5FIX-8i-replay-last.out`, `R5FIX-8i-newvoids-last.txt` and
  `R5FIX-8i-shadowflags-last.txt` under `D:/repos/redkiln-sev/afk-tools/sp/s8i/`]:
  - **Corpus records: 0 of 754 void. This audits no read input.** The records carry only the digest, which is never
    audited (R4FIX-8i item 2), so the replay reads zero inputs (`read_inputs=0`) and the zero follows by
    construction. It shows only that no prose is read, and it is not evidence of correctness. R4FIX-8i item 5's
    *"Corpus records: 0 of 754"* reads the same way (refuting round 5's minor).
  - **Reviewer transcripts.** 838 sessions, 13,147 tool uses. 71 runtime voids, every one card-shaped: R4FIX-8i's 69,
    each still voiding, and 2 new. One is genuine (`find /d -name "*.js" -path "*/workflows/*"`, a walk of D:). One is
    a false void: a session read a mistyped spelling of its own task-output path under `<temp>/claude`, a directory
    that does not exist. Exempting it would open other slugs, so it is disclosed. Before the task-output exemption and
    the two replay fixes, 64 more sessions voided on their own task output, and one more on `/tmp/../c/...`. The
    shadow audit flags 312 of the 838 sessions (10,786 reads), all 71 voids among them, so 241 sessions are flagged
    and not voided: the allowlist's measured false-flag rate. By class of the flagged path: `D:/repos/redkiln-rs`
    4,109 reads in 67 sessions; temp not written first 2,420 in 58; Git Bash words outside the allowlist (most are
    regex or sed operands read as paths) 1,544 in 263; other D: paths 1,013 in 22; other words (escapes read as paths)
    584 in 156; unresolved `$VAR` or `$(...)` 364 in 116; the session's own task output 305 in 77; other C: paths 257
    in 10; the review root outside the copy 185 in 17; the home directory 5 in 2. The `/tmp/*.bak` write-then-restore
    pattern is clear; one `rm` of a temp path never written stays flagged.

  **8. Residuals, disclosed.**
  - `cp -r`, `tar`, `zip` and `rsync` are not recursive readers under (b). A read of an ancestor through them is
    flagged by the shadow audit, not voided.
  - A prefix assignment's value is visible to its own command's words, which bash would expand with the old value.
    That can only void more.
  - The variables persist across `Bash` calls on the same assumption as R4FIX-8i item 6's cwd model, which is not
    measured.
  - Under (b), `D:/repos/redkiln-rs`, `D:/repos/redkiln-corpus` and `D:/repos/BrittonGames` add no ancestor the sweep
    root `D:/repos/redkiln-sev` does not already add: every proper ancestor of one is an ancestor of it, and each is
    voided by name itself. Their mutant survives for that reason and is recorded rather than counted.
  - R4FIX-8i item 6's residuals stand.

- **s8i pre-dispatch amendment, extension: item 4's out-of-tree rule after `s8i` refuting round 6 — D21-145's ruling
  on the task-output exemption, the temp rule's directory destinations, output that reaches execution, and a
  correction to the sweep's counts — 2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch
  amendment* above and its item 4, the extension written after refuting round 4, R4FIX-8i, its items 2 and 6, and the
  extension written after refuting round 5, R5FIX-8i, its items 3, 4, 5, 7 and 8; edits nothing in them or in the other
  extensions above). Written in the AFK build, package R6FIX-8i, under `spikes/DECISIONS-2026-09-21.md` D21-144 and
  D21-145 (each TAKEN PROVISIONALLY under D21-105). It is the operator's to confirm before B1, beside R4FIX-8i and
  R5FIX-8i.

  **1. The task-output exemption is ruled (R5FIX-8i item 4).** Item 4 calls the exemption of the child's own
  background-task output *"PROVISIONAL, and returned to the orchestrator as a blocker the ruling did not foresee"*.
  D21-145 rules it: *"That own-slug `tasks/` path is exempt, in every spelling, exactly as R4FIX item 3 exempts the
  session's own tool-results. Other slugs under `<temp>/claude` stay answer-bearing."* The exemption stands as built,
  and item 4 is read with this ruling from here on. D21-145 decides the exemption only. Item 4's other two questions
  stay with the operator at item 4's gate: whether a pre-spawn check proves `<temp>/claude/<copy slug>/` absent, and
  whether the shadow allowlist carries the path. As built there is no such check, and the shadow audit flags those
  reads. *"In every spelling"* is now tested for the `~`, `$HOME`, `$USERPROFILE`, `/c/` and `/mnt/c/` spellings too,
  with a home the temp directory sits under, as on the operator's machine (`%TEMP%` is `<home>/AppData/Local/Temp`).
  Before this, only `/tmp`, `$TEMP`, the native form and `/tmp/../c/…` were tested.

  **2. The temp rule's written paths (R5FIX-8i item 5).** Item 5 allows a temp path *"**only** where the same session
  wrote that path earlier"* and says *"Temp is not allowed wholesale."* Refuting round 6 measured that `cp X /tmp/` and
  `cp X "$TEMP"` recorded the temp directory itself as written, so every later temp read was clear. From this
  extension on:
  - A `cp` or `mv` destination that is a directory receives `<dest>/<basename of each source>`, and the directory
    itself is not recorded. A destination is a directory when it ends in `/`, when its last segment is `.` or `..`,
    when it folds to the temp directory or an ancestor of it (`/tmp`, `$TEMP`, `$TMP`, `/tmp/..`), or when the command
    names several sources.
  - A written path covers a read only when it lies below the temp directory. The temp directory itself, or an
    ancestor of it, written by any route (for example `> $TEMP`), covers nothing.
  - The allowlist's version is now **`r6fix-8i-1`**. Its entries are item 5's.

  **3. Output that reaches execution (R4FIX-8i item 2's `echo`/`printf` exemption).** Item 2 leaves the operands of
  `echo` and `printf` unaudited, *"which are output text"*. Refuting round 6 measured that a path reaching execution
  through them escaped both audits, for example `echo D:/sb/base/item-4/CLAUDE.md | xargs cat`, `printf 'cat …' >
  s.sh; bash s.sh` and `echo "…open('D:/sb/…')…" > p.py && python p.py`, while the heredoc form of the same script
  voided. The exemption stands for output that goes to the terminal or to a file never run: that is a mention. It ends
  when the output reaches execution, and what runs is then audited as what it is.
  - **Piped.** A `|` now ends a command whose output feeds the next one (`||` does not). An `echo` or `printf`'s text
    in the pipe is known:
    - piped into `xargs`, its words become the operands of xargs's command (with `-I`'s replacement applied), and that
      command is audited as a simple command;
    - piped into a shell (`sh`, `bash`, `zsh`, `dash`, `ksh`) that reads its script from stdin, it is audited as a
      `Bash` command;
    - piped into an interpreter (`python`, `python3`, `py`, `node`, `perl`, `ruby`, `pwsh`) that names no script, it is
      audited as data: its absolute paths are read.
  - **Staged.** An `echo` or `printf` redirected to a file, and the `Write` tool's content, are kept by the file's
    path. When the session later runs that file, its text is audited the same way. That holds when the file is run as
    the program (`./f`), as a shell's script (`bash f`, `sh f`, `bash < f`), by `source f` or `. f`, as xargs's input
    (`xargs < f`, `xargs -a f`), or as an interpreter's script or stdin.
  - **Fed to a shell directly.** A shell's `-c` string, `eval`'s operands, and a heredoc's body or a here-string fed
    to a shell that reads stdin are audited as `Bash` commands. A heredoc's body is still read as data as well, as
    before.
  - Each run inside a run is audited in a cwd and variables of its own, as a child shell's are, to a depth of 8.
  - **The shadow audit's unresolved rule (R5FIX-8i item 5) extends to input that reaches execution and that the audit
    cannot read.** That is xargs's input from a pipe of any other program, or from a file the session did not stage,
    and a shell's or interpreter's stdin from such a pipe. Each is flagged, as `unresolved input of xargs` or
    `unresolved stdin of <program>`, and never voided, as an unresolved `$VAR` is.
  - The runtime void follows from the rules that already stand: a word read this way voids where it would void as a
    read anywhere else.

  **4. Measured** [measured 2026-10-03, read-only: both replays run from inside the test binary under the no-dispatch
  wrapper, in a replay copy never shipped (`S8I-R6FIX-8i-replay`), over R5FIX-8i's driver (`sbin.rs` sha256
  `bcdd084840e5…`) and this extension's (`6ebe56ffaadc…`), over the same 838 transcripts and 754 records; outputs
  `R6FIX-8i-replay-r5base.out`, `R6FIX-8i-replay-r6final.out` and `R6FIX-8i-shadowflags-new.txt` under
  `D:/repos/redkiln-sev/afk-tools/sp/s8i/`]:
  - **Runtime voids: 71 of 838 under both drivers**, the same 71 sessions with the same 632 evidence lines. Nothing
    new voids, so R5FIX-8i item 7's check, that every void is a genuine read, stands.
  - **Shadow: 312 of 838 sessions flagged under both.** The reads flagged go from 10,786 to 10,793. The 7 new flags
    fall in 6 sessions that were already flagged. Five are `unresolved input of xargs`, e.g. `find … | xargs grep …` and
    `… | head -1 | xargs -I {} tail …`. One is an `rm -rf /tmp/rkpkg` of a directory the session made with `mkdir`
    and filled with `cp -r … /tmp/rkpkg/`. One is the JavaScript string fragment `"/spec.md"`, read as an absolute path
    in a script that `printf` staged and `node` ran.
  - **Corpus records: 0 of 754**, auditing zero read inputs, as before.

  **5. A correction to the R5FIX-8i extension's item 3** [measured 2026-10-03, read-only: `s8i_R6FIX-8i_sweepcount.py`
  over `R5FIX-8i-sweep-out/` and `R5FIX-8i-sweep2-roots.txt`, output `R6FIX-8i-sweepcount.out`]. Item 3 reads *"397
  files hold a hit. They are bounded by eleven roots"* and *"17 hits state no seed. 16 are mypy's `report.py` … One is a
  work-OneDrive client document"*. The sweep's outputs hold 397 unique hit paths: 378 under the eleven roots, and 19
  outside them. The 19 are 17 paths to mypy's `report.py` and one Google DriveFS content-cache file, itself a cached
  copy of that `report.py` (its hit is the same *"entirely whitespace"*). The cache file is listed twice: under its own
  path and under the profile's legacy `AppData/Local/Application Data` junction. So that junction was not wholly
  skipped, contrary to item 3's skip list (*"the profile's legacy junctions, whose targets are searched under their
  own names"*). The OneDrive document came from the separate OneDrive pass, not from these 397. No registered root
  moves: every unregistered hit still states no seed.

  **6. Residuals, disclosed.**
  - Inline code is not lexed. A `python -c` that builds a path from parts (`os.path.join('D:/repos', 'BrittonGames',
    …)`) does not void, and a multi-word read-position word is still resolved as one relative path, so the shadow audit
    does not see an absolute fragment inside it. A known root's name inside the word still voids, as before.
  - A profile junction spelling (`/c/Documents and Settings/…`) is not folded to the home directory, so a named root
    reached that way does not void. The shadow audit flags it, inside a `bash -c` string too.
  - A heredoc written to a file and run later is not staged. Its absolute paths are read when it is written, but a
    relative path in it is not resolved when it runs. A file changed by the `Edit` tool and then run is not staged
    either.
  - Any program between an `echo` and its executor (`tee`, `tr`, `cat`) makes the text unknown, so the run is flagged,
    not audited.
  - A `cp` or `mv` into an existing temp subdirectory named without a trailing `/` (`cp X /tmp/sub`) records `sub`,
    which then covers `sub`'s subtree. The audit cannot know that `sub` existed. A `mkdir` is not a recorded write, so a
    read of a directory only `mkdir` made stays flagged.
  - R4FIX-8i item 6's and R5FIX-8i item 8's residuals stand.

- **s8i pre-dispatch amendment, extension: item 4's out-of-tree rule after `s8i` refuting round 7 — a text the audit
  runs is audited only as that run, precommands are peeled, item 9's audit invocation, and a quotation corrected —
  2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch amendment* above and its items 4 and 9,
  the extension written after refuting round 4, R4FIX-8i, its item 3, the extension written after refuting round 5,
  R5FIX-8i, its items 3 and 5, and the extension written after refuting round 6, R6FIX-8i, its items 3, 4 and 6; edits
  nothing in them or in the other extensions above). Written in the AFK build, package R7FIX-8i, under
  `spikes/DECISIONS-2026-09-21.md` D21-144 and D21-145 (each TAKEN PROVISIONALLY under D21-105). It is the operator's
  to confirm before B1, beside R4FIX-8i, R5FIX-8i and R6FIX-8i.

  **1. A text the audit runs is audited only as that run (R6FIX-8i item 3, "Fed to a shell directly").** Item 3
  audits a shell's `-c` string, `eval`'s operands, and a heredoc's body or a here-string fed to a shell as `Bash`
  commands, and says *"A heredoc's body is still read as data as well, as before."* Refuting round 7 measured that the
  `-c` string and `eval`'s operands were also still audited as one operand word of the shell or of `eval`. Such a word
  does not resolve to the path inside it, so the exemptions of the session's own `tool-results` (R4FIX-8i item 3) and
  of its own task output (D21-145: *"That own-slug `tasks/` path is exempt, in every spelling"*) were skipped, and the
  word voided by name. The 838-transcript replay holds a reviewer doing exactly this: session `d695a1ce` waited on its
  own background task with `timeout 300 bash -c 'while [ ! -f "C:\Users\…\Temp\claude\D--sb-d-p1i1a1\<session>\tasks\byujxk0ry.output" ]; do sleep 2; done; tail -200 "…"'`,
  and that command voided. From this extension on:
  - a shell's `-c` string, `eval`'s operands and xargs's command are audited only as the run they are, never also as
    operand words;
  - a heredoc's body or a here-string fed to a shell that reads its script from stdin is audited only as that run. It
    is no longer read as data as well, so item 3's sentence quoted above is withdrawn. A path in it voids exactly where
    it would void typed bare: a read voids, and a mention (an `echo` operand, a grep pattern) does not;
  - a heredoc's body or a here-string given to any other program is read as data, as before.

  This is item 3's own rule, applied once: *"a word read this way voids where it would void as a read anywhere
  else."* Another slug's `tool-results` or task output in the same positions still voids.

  **2. Precommands are peeled (item 3's run-following).** Refuting round 7 measured that a precommand in front of a
  shell hid its `-c` string, staged script, heredoc or piped stdin from item 3, so a relative climb escaped both the
  runtime void and the shadow flag, for example `timeout 60 bash -c 'cd ../../base/item-4 && git log -3 --stat'`,
  `nohup bash s.sh` after `printf 'ls ../..' > s.sh`, and `echo 'ls ../..' | timeout 30 bash`. From this extension on,
  these precommands are peeled, repeatedly, before the program is read:
  - `timeout`: `-k`, `-s`, `--kill-after` and `--signal` take a value; then its duration;
  - `nice`: `-n` and `--adjustment` take a value;
  - `nohup`;
  - `env`: `-u`, `--unset`, `-S`, `--split-string`, `-C` and `--chdir` take a value; `-i`, `-` and `NAME=value` words
    are its own; `-C <dir>` and `--chdir=<dir>` run the program in `<dir>`;
  - `stdbuf`: `-i`, `-o`, `-e` and their long forms take a value;
  - `time`: `-f`, `-o`, `--format` and `--output` take a value;
  - `command` (`-v` and `-V` look a name up and run nothing, so they are not peeled);
  - `exec`: `-a` takes a value.

  Every other word starting with `-` is a flag of the precommand. The precommand's own words are audited as operands,
  as an assignment's value is, and the program after them is followed exactly as if typed bare. `time`, `command`,
  `exec` and `env` were words the lexer skipped before a program; they are peeled now, with their options. xargs is
  followed as a precommand too: its command runs once even when its input is unknown or empty, as GNU xargs runs it, so
  that command is audited as a simple command, with its input's words when the audit knows them and flagged as before
  when it does not.

  **3. Measured** [measured 2026-10-03, read-only: both replays run from inside the test binary under the no-dispatch
  wrapper, in a replay copy never shipped (`S8I-R7FIX-8i-replay`), over R6FIX-8i's driver (`sbin.rs` sha256
  `6ebe56ffaadc…`) and this extension's (`0efe267f6872…`), over the same 838 transcripts and 754 records; outputs
  `R7FIX-8i-replay-r6base.out`, `R7FIX-8i-replay-r7.out` and `R7FIX-8i-replaydiff.out` under
  `D:/repos/redkiln-sev/afk-tools/sp/s8i/`]:
  - **Runtime voids: 71 of 838 under both drivers**, the same 71 sessions. The evidence lines go from 632 to 631. The
    one removed is session `d695a1ce`'s own-task wait quoted in item 1. That session still voids, on `find /d
    /repos/redkiln-rs -maxdepth 2 -type f -name "*.md"`, a walk of D:, and on `find
    "C:\Users\ryanm\AppData\Local\Temp\claude" -name "byujxk0ry.output"`, a walk of the registered temp root `claude`,
    which holds every slug's output. Peeling voids no further session: R5FIX-8i item 7's check, that every void is a
    genuine read, stands.
  - **Shadow: 312 of 838 sessions flagged under both.** The reads flagged go from 10,793 to 10,794, all in sessions
    already flagged. One flag goes: `xargs grep -l "id: $S"` had its pattern flagged as an unresolved word, and the
    pattern is now read as grep's pattern. Two are new: the same `d695a1ce` wait, whose own task output the shadow audit
    now sees, as it sees that output read bare (R6FIX-8i item 1: the shadow allowlist does not carry it); and `time`
    read as a word in a directory outside the copy (`cd /d/tmp/rk77.4fmi && … time ./node_modules/.bin/vitest …`).
  - **Corpus records: 0 of 754**, auditing zero read inputs, as before.

  **4. Item 9's audit invocation.** Item 9 reads *"`results/audit-s.py bin <s-bin.jsonl> <missing-fields.jsonl>`
  computes each pass's outcome from the records and that file alone."* R5FIX-8i item 5 gave `bin` an optional third
  argument, the operator's rulings, and Checkpoint 8i (4) re-runs the pinned audit with it and compares the output byte
  for byte. From this extension on, `results/s-bin/audit.txt` is written as `python results/audit-s.py bin
  results/s-bin.jsonl results/s-bin/missing-fields.jsonl results/s-bin/oot-shadow-rulings.jsonl`. The rulings file is
  committed with every batch's results, an empty file when no record is flagged, and is pinned (`s8i` §8i.7 and
  Checkpoint 8i (3)). Computed with two arguments, any flagged and ruled record would show as held, and the pinned
  `audit.txt` could not reproduce.

  **5. A quotation corrected (R5FIX-8i item 3).** Item 3 closes *"The copies are bounded, so no escalation to an
  allowlist void follows (D21-144: *"If the sweep finds copies it cannot bound, stop and escalate to B"*)."* The ledger
  entry D21-144 does not hold those words. It reads *"If the sweep finds copies it cannot bound, or the denylist still
  leaks, escalate to B as the runtime void by a separate proposed extension."* The quoted words are the ruling record's,
  `target/afk-wf/s8i-ruling-d21-144.json` (an untracked workflow output; sha256 `b500fb8aa4ff…`): *"If the sweep finds
  copies it cannot bound, stop and escalate to B as a separate proposed extension."* Item 3's conclusion holds under
  either wording, because the copies are bounded.

  **6. Residuals, disclosed.**
  - A program-and-args wrapper outside item 2's list (`winpty`, `watch`, `xvfb-run`, `strace`) is not peeled. The
    program after it is audited as its operand words, and a `-c` string after it as one word, as before this extension.
  - `env -S`'s string is audited as one operand word, not split into the command it names.
  - `timeout` before `eval` is followed, although `timeout` cannot run a builtin. The audit only voids more for it.
  - A heredoc whose shell is not the last command before its body's line (`bash <<EOF | cat`) is read as data only,
    as before.
  - R4FIX-8i item 6's, R5FIX-8i item 8's and R6FIX-8i item 6's residuals stand.

- **s8i pre-dispatch amendment, extension: item 4's out-of-tree rule after `s8i` refuting round 8. Two residuals are
  disclosed: a process the reviewer starts, and an interpreter's inline code. 2026-10-03, before B1; no S-BIN record
  exists** (cites the *s8i pre-dispatch amendment* above and its items 3 and 4; the extension written after refuting
  round 6, R6FIX-8i, its items 3 and 6; and the extension written after refuting round 7, R7FIX-8i, its item 1. It
  edits nothing in them or in the other extensions above). Written in the AFK build, package R8-RECORD-8i, under
  `spikes/DECISIONS-2026-09-21.md` D21-144 and D21-145 (each TAKEN PROVISIONALLY under D21-105). It changes no rule
  and no code: it discloses. Refuting round 8 is the last round under the AFK build's safety cap of 8, and it found no
  blocker. This extension is the operator's to confirm before B1, beside R4FIX-8i to R7FIX-8i. Its item 1 is a
  decision owed before B1 (`s8i` §8i.5, owed item (i)).

  **1. A process the reviewer starts may read paths that no tool input names.** Item 4's runtime void and the shadow
  audit both read what the reviewer's tool inputs say. Neither sees what a program the reviewer runs reads by itself.
  An in-tree script the session did not stage is audited only as the words of the command that runs it. That
  includes Python run inside an editor over port 9316. R6FIX-8i item 3 reads a script's text only when the session
  staged it.
  - **The measured instance.** Every base tree ships `_workspace/tooling/RunEditor_OurGame.bat`. Its line 13 reads
    `set PROJECT=D:\repos\BrittonGames\OurGame\OurGame.uproject`, it then starts the editor on that project, and
    the tree's `AGENTS.md` names it as the way to launch the editor [measured 2026-10-03, read-only on
    `D:/sb/base/item-1..5`]. The editor it starts loads the fixed project, and the tree's `CLAUDE.md` says
    *"`editor_query("run_python", ...)` is arbitrary editor Python"* and *"**Port 9316 has no auth.**"* Round 8's
    finder measured the launcher's run from inside the test binary, in a private copy (corpus `285e011` with both
    patches, under the tree `D:/sb/d/p1i4a1`): `VOID=false hits=[]  FLAG=false []`. The R8-RECORD-8i package did not
    re-run it.
  - **The class.** In items 1 to 4, 14 executable files under `_workspace/tooling/` name an absolute
    `D:/repos/BrittonGames` path. Item 5 has 3. In every `.py` among them the path is docstring usage text, which
    running the script does not read, and which voids by name when typed. The launcher is the one file whose code
    runs the path.
  - **Mitigations.** These are partial.
    - The card's MCP is isolated (`Read` and `Bash`, `--strict-mcp-config`), so reaching the editor needs JSON-RPC
      written by hand to `localhost:9316`.
    - Halt 6 fires if the editor survives the dispatch. It fires only after the record is written.
    - A shadow-held record waits for the operator's review.

  The record of a dispatch that took this route would be unsound and would be scored unheld. Not fixed here. The
  options are `s8i` §8i.5's owed item (i).

  **2. An interpreter's inline code loses the session's own exemptions.** R7FIX-8i item 1 audits a shell's `-c`
  string, `eval`'s operands, xargs's command and a heredoc or here-string fed to a shell as the run they are. Inside
  them, the session's own `tool-results` (R4FIX-8i item 3) and its own task output (D21-145: *"That own-slug `tasks/`
  path is exempt, in every spelling"*) stay exempt. An interpreter's inline code is different (`python -c`,
  `node -e`, `powershell -Command` and the rest). R6FIX-8i item 6 reads *"Inline code is not lexed"*, and the code
  string is still audited as one relative read-position word. So an absolute path to the session's own output inside
  it is not exempted, and the registered temp root's name in it voids the record.
  - **Measured by round 8's finder** from inside the test binary, in a private copy whose `sbin.rs` sha256 began
    `0efe267f`, the R7FIX-8i driver. The copy slug was `D--sb-d-p1i4a1`, and the copy was removed afterwards. A
    `python -c` reading the session's own `tool-results` file, a `node -e` reading its own task output, and a
    `powershell -NoProfile -Command "Get-Content '<own task output>' -Tail 50"` each gave `void=true`. The control,
    the same own path passed as a separate operand to `python -c`, gave `void=false`.
  - **Scope.** The finder also scanned 856 earlier reviewer transcripts. About 250 `Bash` reads of the session's own
    `tool-results` or task output were found, and none put the path inside inline code. The 5 `python -c` and
    `node -e` hits were sleeps beside a bare `tail` or `cat` of the path. The R8-RECORD-8i package did not re-run
    either measurement.
  - **Direction.** This is a false void: an honest record is lost, and no read is scored. Such a read voids, and a
    void is never a miss (METHOD-S §0). It is not fixed here. A fix would apply the own exemptions to each absolute
    path fragment of an interpreter's inline operand before the by-name check, and add the three cases above to the
    named tests, with another slug's path in the same place as a must-void control.

  **3. Residuals standing.** R4FIX-8i item 6's, R5FIX-8i item 8's, R6FIX-8i item 6's and R7FIX-8i item 6's residuals
  stand.
