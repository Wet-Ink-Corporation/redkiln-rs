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

- **s8i pre-dispatch amendment, extension: item 3's strip set gains the editor launcher, the five base repositories
  are rebuilt, and the driver refuses a dispatch copy whose executable file names an answer-bearing path, under
  D21-146 — 2026-10-03, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch amendment* above and its
  items 3 and 4, the extension of 2026-10-03 that lists the five base repositories, its item 3, and the extension
  written after refuting round 8, R8-RECORD-8i, its item 1. It edits nothing in them or in the other extensions
  above). Written in the AFK build, package D21146-8i, under `spikes/DECISIONS-2026-09-21.md` D21-146, the operator's
  ruling of 2026-10-03 (Weigh-In wi-938b68, *"Neutralise in all 5"*). Its item 1 reads *"The launcher joins §8i.2
  item 3's registered strip set, by an appended METHOD-S extension, identically in all five trees. It is either
  removed or rewritten to a tree-relative PROJECT, whichever keeps the stripped set identical across trees; the fix
  package chooses and states which."*

  **1. Item 3's strip set gains one entry: `_workspace/tooling/RunEditor_OurGame.bat`, removed.** Item 3 registers
  *"**One strip set**, applied to every tree before its one commit"*. From this extension on the set has four
  entries, the same in all five trees: `CLAUDE.md#status-rows`, `_workspace/backlog/`,
  `_workspace/references/UE58_Editor_And_MCP_Gotchas.md` and `_workspace/tooling/RunEditor_OurGame.bat`. The file is
  removed whole, like the gotchas reference. It is not rewritten. Three reasons [measured 2026-10-03, read-only on
  `D:/sb/base`]:
  - **Removing it keeps one rule.** The launcher's blob is not the same in every tree. It is
    `570e8add27de2e8ea446e02a6172d975791fd9b2` (1,432 bytes) in items 1 to 4 and
    `548aa4511088233c2d83755699757c73758f6fcb` (774 bytes) in item 5. A rewrite of `PROJECT` would need a byte
    rule per blob, each asserted, and a fifth registered rule to keep identical. Removal adds one path entry, and it
    is the same in all five trees.
  - **Removing it leaves no executable route.** A tree-relative `PROJECT` would still start an editor, on the copy's
    own project. Removal starts nothing.
  - **Nothing that runs in the tree calls it.** `verify_headless.ps1` names it in a comment only: *"# Engine root
    matches RunEditor_OurGame.bat. Override with $env:UE_5_8 if installed elsewhere."* `AGENTS.md`,
    `.claude/skills/unreal-editor-mcp/SKILL.md`, `_workspace/tooling/Rider_CoDev_Setup.md`, `UE58_MCP_Setup.md`,
    `UE_MCP_Operating_Procedures.md` and, in items 1 to 4, `_workspace/runbooks/Inventory_Gym_Review.md` mention it as
    text. Those mentions stay, and they name a file the tree no longer holds. Item 2's capture procedure opens
    `OurGame/OurGame.uproject` in the capture copy directly, never through the launcher.

  **2. The tooling class, measured (D21-146 item 4).** Every non-asset file of the five old bases, `.git` excluded,
  was read for an absolute path to an answer-bearing root: amendment item 4's (BrittonGames, `redkiln-rs`,
  `redkiln-corpus`, the account roots and their `projects/`, the review root) and the R5FIX-8i sweep's (`D:/r6c`,
  `D:/repos/redkiln-sev`, `~/.claude-personal`, `~/.claude-backup`) [measured 2026-10-03, read-only:
  `s8i_D21146-8i_measure.py`, output `D21146-8i-measure-old.json`; 3,643, 3,650, 3,666, 3,643 and 3,243 files].
  - **Executable code: one file, the launcher,** in all five trees, at its line 13.
  - **Docstring usage text, not stripped:** in items 1 to 4, the eight `_workspace/tooling/author_*.py`
    (`author_baseline_vessels`, `author_belt_container`, `author_equipment_slots`, `author_item_catalogue`,
    `author_nested_vessels`, `author_spatial_pack`, `author_starting_loadout`, `author_upgraded_vessels`) and five
    `synty/` scripts (`calibrate_deformation.py`, `ci_validate.py`, `strip_character_sources.py`,
    `diagnostics/diag_conform_axis.py`, `diagnostics/proto_orient_bones.py`); in item 5, `synty/ci_validate.py` and
    `synty/strip_character_sources.py`. Each path sits in the **module** docstring, as `-script="D:/repos/BrittonGames/…"`
    or, in `strip_character_sources.py`, `sys.path.insert(0, r"D:/repos/BrittonGames/…")` under *USAGE* [measured by
    `ast`: every hit lies inside the module docstring's line span]. D21-146 item 4: *"They are not stripped unless the
    precheck finds a path in executable code."* It finds none (item 4 below).
  - **Prose, not executable, not stripped:** in all five trees `AGENTS.md`, `CLAUDE.md`,
    `.cursor/rules/britton-ue58-conventions.mdc`, `OurGame/Plugins/Monolith/Docs/SPEC_CORE.md`,
    `OurGame/Plugins/Monolith/Tools/MonolithProxy/README.md`, `_workspace/references/Migration_Tooling_Remediation_Plan.md`,
    `_workspace/runbooks/E1_Synty_Pack_Migration_Runbook.md`, `_workspace/tooling/Rider_CoDev_Setup.md` and
    `_workspace/tooling/UE58_MCP_Setup.md`, and in items 1 to 4 also `_workspace/references/Locomotion_Framework_Strategy.md`.
  - The bare word `BrittonGames`, with no path, appears in a further 321, 328, 338, 321 and 125 files: C++ copyright
    headers, C# build files and Markdown. None is an executable script.

  **3. The five base repositories, rebuilt (D21-146 item 2).** Each was built exactly as the extension of 2026-10-03
  that lists them records, by `s8i_D21146-8i_build.py`, which `s8i_D21146-8i_mkbuild.py` derives from S8I-F1's script
  by nine anchored replacements: the launcher's removal, the strip set's fourth entry, and a new base root and
  record directory. A disposable `git clone --no-checkout --shared` of BrittonGames under `D:/sb/tmp-d21146` was used,
  then deleted. Each repository was built beside the old one, under `D:/sb/base-d21146`, and replaced it at
  `D:/sb/base/item-<n>` only after all five verified. BrittonGames' HEAD (`ee6e450`), status sha256 (`a6751b0c…69ed`),
  worktree-list sha256 (`ba14505a…e0ab`) and config sha256 were the same before and after.

  | item | item commit | base commit | base tree | LFS pointer paths | base bytes (`.git` included) |
  | --- | --- | --- | --- | --- | --- |
  | 1 | `cc0cf2d01` | `37f0f0e2f679d58316edbf4da7f23ccebca8dde2` | `7f1018de67a96c2c0b545d024838ea62b126c837` | 30,014 | 21,540,843,118 |
  | 2 | `fa1ea72f4` | `bdff598b83b4cbb5f1f27ca901697cf8aa615994` | `3354f08014e0a986735141625f55e17e713c0955` | 30,015 | 21,541,174,637 |
  | 3 | `e83463288` | `de3b4ca53758f014c321f1321ed9b6ce7c3997b9` | `a2e25cc92a4a11c16499f958af510406a4c3e503` | 30,022 | 21,560,066,805 |
  | 4 | `a71265925` | `1553eda41de0b0f5a25a38ed5cdbea9d6258af6c` | `7f1018de67a96c2c0b545d024838ea62b126c837` | 30,014 | 21,540,843,759 |
  | 5 | `6a101bdcd` | `5f90d80ef4fed42822f955b944b5804cabfc3da6` | `370b48cd11736139ee86039f7c1071fe749b4634` | 11,886 | 10,604,497,923 |

  - **Each new tree is the old tree less one entry.** Entry by entry, each new `HEAD` tree equals the old base's
    `HEAD` tree, which P7 and F1 verified, except for `_workspace/tooling/RunEditor_OurGame.bat`. That entry is in the
    old tree and not in the new, and no other entry is added or changed (33,570/33,569, 33,578/33,577,
    33,595/33,594, 33,570/33,569 and 15,066/15,065 entries). So the seeds, `CLAUDE.md`'s byte rules and the
    other three strip entries are byte for byte the old bases'. Base items 1 and 4 still hold one tree,
    `7f1018de67a96c2c0b545d024838ea62b126c837`.
  - **The strip.** The build removed the launcher from every tree, plus the 19 files of items 1 to 4 and the 8 of
    item 5 it removed before. `CLAUDE.md` lost its four rows under item 3's rule at items 1 to 4, and under D21-142's
    rule at item 5. Every build record's `stripped` is the one four-entry set.
  - **LFS.** For every LFS pointer in each `HEAD`, the work file's size and SHA-256 equal the pointer's, and the
    object is in the base's own `.git/lfs/objects` at that size: 30,014, 30,015, 30,022, 30,014 and 11,886 pointer
    paths, holding 29,576, 29,577, 29,584, 29,576 and 11,447 distinct objects, with 0 bad. The build had first proven
    each materialised file against its pointer.
  - **Leak tokens.** `LoadoutFill`, `white-on-white`, `VERIFIED COLD` and `entirely white` count 0 over every
    non-asset file of all five, before and after each commit.
  - **The seeds, by blob oid**, unchanged. Base 4's `HEAD:` blob at `WBP_LoadoutScreen.uasset` is
    `9415f0986275a66d01885cc74f58c33e1957e183` = `a71265925^`'s. Base 5's at `WBP_GameVitals.uasset` is
    `23c79a6337215a7a208d55676081f0fc33fcbeb7` = `6a101bdcd^`'s. The fixed blobs (`70e2a2e3…aa37c`, `c84ea7ac…6f18`)
    and their LFS objects are absent; the same probes fire on the seeds' own blob and object (the control).
  - **One commit each.** `main`, `review tree`, `s-bin <s-bin@invalid>`; `git rev-list --all --count` is 1,
    `refs/heads/main` is alone, and there is no remote and no tag. `git status --porcelain --untracked-files=all
    --ignored` prints nothing.
  - **Controls.** Verifying an old base against itself fails, as it must: no entry differs, and the launcher is
    still in the tree. The leak scan finds planted tokens and skips assets and `.git`. The executable-path rule
    finds a planted launcher and passes a planted module docstring. Outputs: `D21146-8i-verify-all.json` and
    `D21146-8i-verify-ctl-oldold.json`.

  This table supersedes, for B1 on, the base commits and trees in the extension that lists the five base repositories,
  its item 3. That extension is not edited. Its LFS figures and seeds stand.

  **4. The driver's pre-spawn check gains `precheck_exec_paths` (D21-146 item 3).** Item 4's checks on each dispatch
  copy before it spawns now include one more check. The driver reads every executable file in the copy, `.git` aside.
  Those are the files whose extension, matched without regard to case, is `.bat`, `.cmd`, `.ps1`, `.psm1`, `.sh`,
  `.bash` or `.py`. A batch file, a PowerShell script and a shell script are read whole. A Python file is read
  without its module docstring and its `#` comments. The module docstring is the first statement, a string literal
  standing alone on its line, and any other docstring is read as code. A file with a UTF-16 byte-order mark is
  decoded as UTF-16. Each text is folded (`\` to `/`, case folded, runs of `/` collapsed) and checked for amendment
  item 4's roots by name, on any drive or mount, and for the R5FIX-8i sweep's roots: a drive root in its `x:/`,
  `/x/` and `/mnt/x/` spellings, and a home root by its path under any home. If any file matches, the driver refuses
  before the spawn and names the files. A refusal writes no record (R2FIX-8i's extension, item 3), so `s8i` §8i.7's
  record keeps its 34 names.
  - **Measured over the real trees,** from inside the test binary over the shipped code, in a control copy that is
    never shipped (`D21146-8i-ctl`, one `#[ignore]`d test). Each old base yields
    `["_workspace/tooling/RunEditor_OurGame.bat"]`, and each rebuilt base yields `[]`. The module-docstring scripts
    pass in both.
  - **Residuals disclosed.** These are not caught:
    - a path spelled through a variable (`%REPOS%\BrittonGames`);
    - a binary or any other extension;
    - a Python 3.12 f-string that nests its own quote.
    A path in a function's docstring refuses, which is the stricter reading. The runtime void and the shadow audit
    are unchanged. R8-RECORD-8i item 1's class (a process the reviewer starts reads paths no tool input names) stands
    for any program this check does not read.

  **5. The dry run's review repository, rebuilt.** It is a byte copy of base item 1 (`s8i` §8i.3, package S8I-P8), so
  it held the launcher, and item 4's check refuses it. It is rebuilt from the rebuilt base item 1 by
  `s8i_D21146-8i_review.py`, which is S8I-P8's script with its two pins of base item 1 moved. It is one parentless commit, `48cd028e2b83b65e12d456c4ca2e2a258cbed593`, base item 1's tree plus exactly the two `_s-bin/report/` files, and it supersedes `378a183fa265f00b6af2a3be02ba2d8adbf87b36`. The dry run's fixture, re-derived unchanged by S8I-P8's own script, hashes `8f98bf430ddf6f461217eab9f89305b6d279bdec011f5282aac691e67f728ea1`. The check reads `[]` over it.

- **s8i pre-dispatch amendment, extension: item 3's strip writes one generated backlog stub back into all five
  trees, and the driver refuses a dispatch copy whose stub differs, under D21-147 — 2026-10-03, before B1; no S-BIN
  record exists** (cites the *s8i pre-dispatch amendment* above and its items 3 and 4, and the D21146-8i extension
  above, its items 1 and 4. It edits nothing in them or in the other extensions above). Written in the AFK build,
  package D21147-8i-A, under `spikes/DECISIONS-2026-09-21.md` D21-147, the operator's ruling of 2026-10-03. Its item 1
  reads *"After the existing whole-directory strip, one generated file is written at
  `_workspace/backlog/game_foundation_backlog.json`, byte-identical in all five trees"*, and *"No further story is
  admitted"*.

  **1. The rule.** Item 3's strip set is unchanged: the same four entries, the same in all five trees
  (`CLAUDE.md#status-rows`, `_workspace/backlog/`, `_workspace/references/UE58_Editor_And_MCP_Gotchas.md` and
  `_workspace/tooling/RunEditor_OurGame.bat`). `_workspace/backlog/` is still removed whole. Right after that removal,
  the strip writes exactly one file back, at `_workspace/backlog/game_foundation_backlog.json`. Its bytes are the ones
  in item 2, the same in all five trees. After the strip, `_workspace/backlog/` holds that one file and nothing else.
  The stub is written content, not a fifth strip entry, so every tree's `stripped` set stays the one four-entry set.

  **2. The bytes, registered.** UTF-8 (all ASCII), one line, LF-terminated, no byte-order mark, compact JSON with no
  whitespace. The stories are in the order `8.3.5a`, `8.3.5b`, `8.5.8`. Each story has `id` then `status`, and
  `status` is `todo`. There is no other field and no other story. These are the 109 bytes:

  ```
  {"stories":[{"id":"8.3.5a","status":"todo"},{"id":"8.3.5b","status":"todo"},{"id":"8.5.8","status":"todo"}]}
  ```

  followed by one LF. sha256 `211a4dcf7be9eb68ef73dfcb6e67a832b1713f1e3c9aa8f148fb52df189f8787`. A copy is kept
  at `D:/repos/redkiln-sev/afk-tools/sp/s8i/D21147-8i-A-backlog-stub.json`. The stub holds none of the registered
  leak tokens.
  - **The shape is the one the specs read** [measured 2026-10-03, read-only on `D:/sb/base`]. The spec
    `OurGame/Source/OurGameTests/Private/Tests/Architecture_NoInertCapability.spec.cpp` exists in bases 2 and 3 only
    (sha256 `81e90ac9…8628` and `af0f7643…f396`). Both read the file at test time in `TryGetStoryStatus`. Each
    deserialises the root, takes `Root->TryGetArrayField(TEXT("stories"), Stories)`, and keeps an entry only when
    `TryGetStringField(TEXT("id"), Id)` and `TryGetStringField(TEXT("status"), Status)` both hold. Item 2's waivers
    name `8.3.5a`, `8.3.5b` and `8.5.8`, and its anti-blindness story is `8.3.5a`. Item 3's one waiver names `8.3.5b`,
    and so does its anti-blindness story. In each of the five trees no other code reads the file: `OurGameTests.Build.cs`
    names it in a comment, `check_docmap.py` (items 1 to 4) names it only in an exemption set, and the rest are Markdown.
  - **Under the stub,** item 2's two failing assertions read as follows. *"Expected '8.3.5a exists in the backlog' to
    be true"* finds `8.3.5a` as `todo`. *"Expected 'the backlog JSON parsed and yielded stories' to be true"* finds
    three stories. Every waiver names a story that is not `done`. That is the prediction; the greenness runs are the
    operator's (D21-147 item 3).

  **3. Disclosed: the stub is fixture content, not the item's backlog.** Read-only at the five item commits
  [measured 2026-10-03, `git show <commit>:_workspace/backlog/game_foundation_backlog.json` piped to a reader that
  printed only these three ids' statuses; nothing was written]:
  - at `cc0cf2d01`, `fa1ea72f4` and `a71265925`, all three read `todo`, as in the stub;
  - at item 3's commit `e83463288`, `8.3.5a` and `8.5.8` read `done`, and `8.3.5b` reads `todo`. Item 3's spec reads
    only `8.3.5b`, so the two stories the stub states as `todo` there are never read by its tests;
  - at item 5's commit `6a101bdcd`, none of the three ids is in the file, which holds 189 stories. Item 5 has no such
    spec.

  No further story is admitted. `8.5.5b` tracks the seed. The stub states no title, epic or prose, so it carries
  nothing the reviewer could read as the item's status.

  **4. The driver writes it and checks it (D21-147 item 2).** In the driver patch that supersedes
  `S8I-D21146-8i-sbin-driver.patch`, `strip_tree` writes `BACKLOG_STUB` at `BACKLOG_STUB_PATH` right after it removes
  `_workspace/backlog/`. `stripped_path_set_is_identical_across_all_five_trees` now admits exactly that one file
  under `_workspace/backlog/` in each of the five trees, at the registered bytes. It also pins the sha256 and the
  shape above. Item 4's pre-spawn checks gain one more check, `precheck_backlog_stub`. It refuses before the spawn,
  naming the check, unless the copy's `_workspace/backlog/` holds exactly one entry, the stub file, at the registered
  bytes. A missing directory, a missing stub, a second file, CRLF, a byte-order mark, a missing LF, a further story
  or field, or a pretty-printed equivalent all refuse. A refusal writes no record (R2FIX-8i's extension, item 3), so
  `s8i` §8i.7's record keeps its 34 names. The cases sit in two existing named tests, so the driver's selection
  stays at 33 tests. The base trees and the dry run's review repository built before this extension hold no stub, so
  the check refuses each of them (`D21147-8i-A-ctl-old.out`). They are rebuilt under D21-147 item 3, not by this
  extension.

  **5. Residuals standing.** Every earlier extension's residuals stand. The stub changes no seed, item, rendering or
  ceiling, and no part of METHOD-S §3.

  **6. The bases and the dry run's review repository, rebuilt (D21-147 item 3).** Recorded in the AFK build, packages
  D21147-8i-B and D21147-8i-C. Each base was built as the D21146-8i extension's item 3 records, by
  `s8i_D21147-8i_build.py`, which `s8i_D21147-8i-B_mkbuild.py` derives from `s8i_D21146-8i_build.py` by eight anchored
  replacements: the stub's bytes and their write right after the backlog strip, the expected tree admitting exactly
  the stub's blob, a check of the committed stub, and new base, clone and record roots. Each was built beside the old
  one, under `D:/sb/base-d21147`, from a disposable clone under `D:/sb/tmp-d21147` (deleted), and replaced it at
  `D:/sb/base/item-<n>` only after all five verified (`ALL_OK`). BrittonGames' HEAD (`ee6e450`), status sha256
  (`a6751b0c…69ed`), worktree-list sha256 (`ba14505a…e0ab`) and config sha256 were the same before and after. This
  table supersedes the D21146-8i extension's item 3 table.

  | item | item commit | base commit | base tree | LFS pointer paths | base bytes (`.git` included) |
  | --- | --- | --- | --- | --- | --- |
  | 1 | `cc0cf2d01` | `58e9896335f2009a7772fb99d6df9cf123418dd4` | `6782b95bf399274efbc6e20ea28ab00a5976c958` | 30,014 | 21,540,844,470 |
  | 2 | `fa1ea72f4` | `e8954b617f4a8b30e7ed2b1dfd221655caf65186` | `72ff014f359c6ce4dbb4af37f53826084bbb09a5` | 30,015 | 21,541,173,139 |
  | 3 | `e83463288` | `efcc24297540af65905c699cb41a25464dd69082` | `86bc1c51b2375ac99f8ec4070f6e846bb55c0f9d` | 30,022 | 21,560,067,392 |
  | 4 | `a71265925` | `2878d79dda6ae72a1143a63937834ed3fb327eff` | `6782b95bf399274efbc6e20ea28ab00a5976c958` | 30,014 | 21,540,843,094 |
  | 5 | `6a101bdcd` | `e0620bfaac31a388fc2863a4f01f8b3f1c16a0a4` | `f1c3d6cff4e004ec05014e81a5028f651a596f51` | 11,886 | 10,604,498,743 |

  - **Each new tree is the D21146-8i tree plus the stub.** Entry for entry, each new `HEAD` tree equals the D21146-8i
    base's, except for one added entry, `_workspace/backlog/game_foundation_backlog.json`, blob
    `999761537faf3961b3424f1cb3a36bbcd2cec967` (the registered bytes, LF), with none removed or changed. The LFS,
    leak-token, seed blob-oid and single-commit checks were re-run and pass, and every build record's `stripped` is
    the one four-entry set (`s8i` §8i.4's D21147-8i note). Base items 1 and 4 still hold one tree.
  - **The dry run's review repository** is rebuilt from the new base item 1 by `s8i_D21147-8i-B_review.py`, S8I-P8's
    script with its two pins of base item 1 moved. It is one parentless commit,
    `a8d9478b54556d6617963cab622b385109928ab3` (tree `4bf6ff94d001720f2f550448cb3bcc9d42bfb7e5`), base item 1's tree plus
    exactly the two `_s-bin/report/` files, and it supersedes the D21146-8i extension item 5's
    `48cd028e2b83b65e12d456c4ca2e2a258cbed593`. The dry run's fixture, re-derived unchanged by S8I-P8's own script,
    hashes `62f6469c11e3bc5e656b565ba28c79709ad30526b954d88017ebd69024b6a8b1`, superseding
    `8f98bf430ddf6f461217eab9f89305b6d279bdec011f5282aac691e67f728ea1`. The driver's check reads
    `backlog_stub_ok=true` and `exec_path_hits=[]` over each new base and over it.
  - **The driver patch** item 4 describes is `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21147-8i-sbin-driver.patch`
    (sha256 `0bf69ab4a1a1e8a396f278c63d1cd4fb943ac64ffc1898392a3c6e8cce54da3c`), applied with `S8I-R5FIX-8i-audit-bin.patch` and no
    other. The dry-run script is unchanged (sha256 `5d112e09…2c2c`).

- **s8i pre-dispatch amendment, extension: tree 5's headless verify script is BrittonGames' later Monolith-disabling blob, identical in all five trees, under D21-148 — 2026-10-04, before B1; no S-BIN record exists** (cites the *s8i pre-dispatch amendment* above and its items 3 and 4, and the D21147-8i extension above, its items 1 and 6. It edits nothing in them or in the other extensions above). Written in the AFK build, package D21148-8i-A, under `spikes/DECISIONS-2026-09-21.md` D21-148, the operator's ruling of 2026-10-04. Its item 1 reads *"It is registered by an appended METHOD-S extension, which discloses that base 5's tooling file differs from R's."*

  **1. The rule.** In tree 5, after item 3's strip and the D21147-8i extension's stub, the work file and the committed blob at `_workspace/tooling/verify_headless.ps1` are BrittonGames blob `ebe7e58f3df5756ad016d9075512a27cd2417fae`: 10,057 bytes, LF, sha256 `4fa3509eb8c5f5dbeebf9c9ff18527fed499aca26806758f66d48d50a440d036`. Its provenance is BrittonGames `71e8f6c91` (2026-08-18, *"stop Monolith running inside the headless test editor, where it can fail any test"*), which passes `-DisablePlugins=Monolith`. Trees 1 to 4 already carry that blob from their own item commits, so all five trees carry one identical script, and the registered greenness command (`s8i` §8i.4 step 1) is unchanged. It is written content in tree 5, not a strip entry, so every tree's `stripped` set stays the one four-entry set.

  **2. Disclosed: base 5's tooling file differs from R's.** R = `6a101bdcd` carries blob `a2047f2d836d2d48e1f37c2145a2f1328091736f` (6,654 bytes, sha256 `fe5dc619…28d8`), which predates `71e8f6c91`; `6a101bdcd` is an ancestor of `71e8f6c91`. Between the two blobs [measured, read-only `git diff` of the blobs] the script gains the `-DisablePlugins=Monolith` flag and its comment, a `-KeepLast` prune of old run logs in `TestResults/`, a `-Tier A` sample remapped from `OurGame.Inventory` to `OurGame.SmokeTier`, and documentation text. The default run, every `OurGame.*` test, reads no path that R's tree lacks. One added comment names `UE58_Editor_And_MCP_Gotchas.md 6.104`, a file in the strip set; trees 1 to 4 carry the same comment, and it holds no registered leak token. A reviewer who runs the suite in its copy runs it with Monolith disabled, as in trees 1 to 4.

  **3. The superseded red run and its diagnosis.** The operator's greenness run of item 5 on base `e0620bfa` read 175 tests, 2 failed, exit 1 (`index.json` sha256 `3d325fc6…52af`, `headless.log` `381881d9…83a9`). Re-measured from its files: both failures are `OurGame.Combat.AttackRootMotion.BlendAttackRootMotionVelocity`, whose error events are `LogBlueprint` asset-load messages from `BP_AnimNotify_FoleyEvent` and `BP_NotifyState_MontageBlendOut` only; the log holds 1,383 `LogMonolithIndex` lines, 0 `-DisablePlugins=Monolith` and 0 `WBP_GameVitals`. Items 1 to 4's green runs each hold the flag twice and 0 indexer lines. Per D21-148 it is an instrument defect, not a red seed, so `s8i` §8i.4 step 2's red-seed replacement does not apply. The run is kept, moved and not deleted, at `D:/repos/redkiln-sev/afk-tools/sp/s8i/S8I-D21148-out/superseded/greenness/item-5/`, with the measurement beside it (`item-5-diagnosis.json`). Item 5's greenness is re-run once on the new base 5, and that run binds (D21-148 item 3).

  **4. Base 5, rebuilt (D21-148 item 2).** By `s8i_D21148-8i-A_build.py`, which `s8i_D21148-8i-A_mkbuild.py` derives from `s8i_D21147-8i_build.py` by eight anchored replacements, beside the old base at `D:/sb/base-d21148/item-5`, from a disposable clone under `D:/sb/tmp-d21148` (deleted). Base commit `c2f948ecb902d972f19c2b4b87491be22159f314`, tree `a8b469e1c7fae157c2a6876f10907d4c82160e6c`, 10,604,502,675 bytes (`.git` included), 11,886 LFS pointer paths. The tree equals the D21147-8i extension item 6's base 5 tree (`f1c3d6cff4e004ec05014e81a5028f651a596f51`) except that one entry, changed from `a2047f2d` to `ebe7e58f`. The leak-token, seed blob-oid, fixed-blob, LFS, strip, stub, exec-path and single-commit checks were re-run and pass (`ALL_OK`), and BrittonGames' HEAD, status, worktree-list and config sha256 were the same before and after. Bases 1 to 4 and their green reports stand. The driver needs no change: it pins no base commit or tree, and its exec-path check reads `[]` over the new base 5; `S8I-D21147-8i-sbin-driver.patch` stays the driver patch.

  **5. Not yet in place.** At this extension's writing, the replacement of `D:/sb/base/item-5` (still `e0620bfa`) by the rebuilt base was refused by the AFK session's permission classifier and is owed to the operator. Item 4's values are the base the re-run must use; a re-run on `e0620bfa` would be on R's script and would not be the binding run.

  **6. Residuals standing.** Every earlier extension's residuals stand. The script changes no seed, item, rendering or ceiling, and no part of METHOD-S §3.

*(dated 2026-10-04, s8i, operator sitting)* **The swap is done, by the operator.** From a plain Git Bash the operator renamed `D:/sb/base/item-5` (`e0620bfa`) aside, moved `D:/sb/base-d21148/item-5` into its place, re-read it there (HEAD `c2f948ecb902d972f19c2b4b87491be22159f314`, `status --ignored` empty, `verify_headless.ps1` blob `ebe7e58f`), replaced `S8I-P7-out/item-5.json` with the D21148 record (`base_path` `D:/sb/base/item-5`; the D21-147 record and the build-time record are kept under `S8I-D21148-out/superseded/`), then deleted the old base. `s8i_D21148-8i-A_prepcheck.py` reads `PREPARE_PRECONDITIONS_HOLD` (its first run failed only on its own `p7 == rec` condition, because the D21148 record still named the build-time path; the record's `base_path` was aligned). Item 5's single binding greenness run may start.

- **s8i pre-dispatch amendment, extension: the deferred extensions 2, 3 and 4 — the greenness results, `ceiling_secs`
  and `SBIN_NEED_BYTES` — 2026-10-04, before B1; no S-BIN record exists.** This paragraph cites the entry above and
  edits nothing in it. It is written from the operator's five greenness runs (D:/repos/redkiln-sev/afk-tools/sp/s8i/
  `s8i_S8I-P7_green.py`, `prepare`/`run`/`collect`/`cleanup`, from a plain Git Bash, the editor closed) and the
  staged `state.json` files. The draft at `S8I-P7-extension-draft.md` is superseded: its base paragraph named the
  P7-era bases, which the D21146-8i, D21147-8i and D21148-8i extensions above replaced; the bases are theirs.
  - **The greenness results (deferred extension 4).** Each tree ran `pwsh -File _workspace/tooling/verify_headless.ps1`
    in its own copy `D:/sb/green/item-<n>`, which was then deleted. All five carry the same script blob
    (`ebe7e58f3df5756ad016d9075512a27cd2417fae`, D21-148) and the D21-147 backlog stub.

    | item | commit | seeded | base | exit | failed | succeeded | succeededWithWarnings | notRun | tests | wall s | suite s | index.json SHA-256 | headless.log SHA-256 |
    | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
    | 1 | `cc0cf2d01` | no | `58e9896335f2009a7772fb99d6df9cf123418dd4` | 0 | 0 | 732 | 160 | 0 | 892 | 693.1 | 41.0 | `49e3b3f084d2c3bcbb7a00fd679567e81eec0cb62b4ab8ea94a6c6a496f41a0c` | `1a675cd0870be05b38560937794da23b0d57cc1b64fa63978eefe2bfe38aa27c` |
    | 2 | `fa1ea72f4` | no | `e8954b617f4a8b30e7ed2b1dfd221655caf65186` | 0 | 0 | 754 | 166 | 0 | 920 | 728.0 | 51.4 | `f511374108406e549174ccefc127dca78037d3fb51fbfa22b4821ecb9a830582` | `058e7c5bd8b378f516859f8d6d26e0b403a35bab8f995dc91bdb979d5356fa70` |
    | 3 | `e83463288` | no | `efcc24297540af65905c699cb41a25464dd69082` | 0 | 0 | 826 | 171 | 0 | 997 | 755.2 | 56.0 | `9176f1ff8ace344a5e993f5efb9c7fdbe74e058aa6e0e208bb3dbb6f545e32ed` | `f383279c4c0724684935200d0eda327e4f259f683e1152297bca516bbbfc7e45` |
    | 4 | `a71265925` | yes | `2878d79dda6ae72a1143a63937834ed3fb327eff` | 0 | 0 | 732 | 160 | 0 | 892 | 702.7 | 41.1 | `ea43a11b3839a40d0c679b67a9b9bd536b1b75ca7c3ab7dd1509a0b9248d9d9e` | `e3031a72ecb5e3b8f45584993afb80787cd1126c31f5471d505685f6b3df551c` |
    | 5 | `6a101bdcd` | yes | `c2f948ecb902d972f19c2b4b87491be22159f314` | 0 | 0 | 139 | 36 | 0 | 175 | 644.4 | 13.1 | `755517b4312585def291fc8c1fc28a2806b7f3dfca69bf7128b5f8bd452c1139` | `fabbcc89d8c8daab2c2fd11441c1406cef897eb62cdd61a6ff3e87acfb5fccfb` |

    Both seeded trees are green (`failed` = 0, exit 0), and so are the three clean trees. Superseded readings, kept
    with their diagnoses: items 1 and 2 on the D21-146 bases (item 1 green, item 2 red from the backlog strip, D21-147)
    and item 3's on its D21-146 base, under `S8I-D21147-out/superseded/greenness/`; item 5's red run on `e0620bfa`
    (Monolith's indexer, D21-148) under `S8I-D21148-out/superseded/greenness/item-5/`. Each survivor the helper
    reported was `UnrealTraceServer.exe` only, which D21-147 item 4 rules is not Halt 6.
  - **The pin report (owed item (d)), accepted by the operator on 2026-10-04.** It is copied to
    `results/s-bin/pin-report-2026-09-05/index.json`, SHA-256 `c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6`,
    the value measured at authoring. The registered source, BrittonGames' `_workspace/tooling/TestResults/index.json`,
    was overwritten on 2026-09-25 (it hashes `65960f78…1a8a`), so the copy was taken from
    `_workspace/_runs/8.3.3g-20260904-2334/final-suite.json` (2026-09-05, `reportCreatedOn` `2026.09.05-08.40.33`,
    849/180/0/0 over 1029 tests), which is byte-identical (re-hashed 2026-10-04).
  - **The ceiling (deferred extension 2).** Build-plus-suite wall clock per tree: item 1 693.1 s, item 2
    728.0 s, item 3 755.2 s, item 4 702.7 s, item 5 644.4 s.
    `W_max` = 755.2 s (item 3). Under item 7's rule, `ceiling_secs` = 3600 + `W_max` rounded up to the next multiple
    of 300 = **4500**, the same for every dispatch and every retry. `fixture.json`'s top-level `ceiling_secs` is
    4500.
  - **`SBIN_NEED_BYTES` (deferred extension 3).** The largest base (`.git` included) is 21,560,067,392 bytes (item 3);
    the largest growth one greenness run left behind is 9,550,135,442 bytes (item 3). Under item 4's rule,
    `SBIN_NEED_BYTES` = ceil((21,560,067,392 + 9,550,135,442) × 1.1) = **34221223118**.

- **s8i pre-dispatch amendment, extension: item 2 step 5's captures follow the tool's own rejection, a stalled
  capture session is logged and retried once, and item 4's per-copy free-space check is in the driver, under D21-149,
  D21-150 and D21-151 — 2026-10-06, after B1 and before B2; no S-BIN record exists** (cites the *s8i pre-dispatch
  amendment* above and its items 2, 4 and 6, the R2FIX-8i extension above, its item 5, and the extension of
  2026-10-04 for the deferred extensions 2, 3 and 4, its `SBIN_NEED_BYTES`. It edits nothing in them or in the other
  extensions above). Written in the AFK build, package D21149-8i-A, under `spikes/DECISIONS-2026-09-21.md` D21-149
  and D21-150 (the operator's rulings of 2026-10-05, taken at B1's stops) and D21-151 (the operator's ruling of
  2026-10-06). D21-149 item 2 reads *"Registered by an appended METHOD-S extension that amends item 2 step 5 and
  discloses the confound"*.

  **1. The capture step, as the tool decides (D21-149 item 1).** Every changed asset still gets the registered
  `capture_scene_preview` call, unchanged: item 2's `$ARGUMENTS` and its capture step as registered. When the tool
  rejects an asset's class, that asset carries no capture. Nothing is substituted, re-framed or chosen per tree. An item
  whose every asset is rejected carries `captures: []` in `fixture.json`, and its review tree holds no capture. A
  partly previewable item carries one capture for each accepted asset, at `_s-bin/captures/capture-<k>.png`, where
  `<k>` is that asset's 1-based index in the item's asset list (git's order, as step 2 counts it). Each `captures[]`
  entry in `fixture.json` names its asset by that explicit `asset_index`, never by its position among the captures.
  The tool's rejection decides, so one procedure still runs on all five trees (N19).

  **2. Item 2 step 5, amended.** An item whose every asset was rejected gets a procedure-log line
  `sha256=none shows_subject=n/a`, with the session's model and `cost_usd`. A partly previewable item's line carries
  one hash per capture file, comma-joined, as registered. Step 5's *"nothing falls back to empty captures"* stands
  for a framing failure. A `no` on any tree is still a stop. An empty list is the tool's own rejection, not a
  fallback. B1 measured it: items 2 (`FTEST_Inventory_Stash`, a World) and 3 (`BP_GamePlayerState`, a Blueprint
  class; `DA_Item_Sword`, a data asset) are `n/a`. Items 1 (`WBP_StashRow`), 4 (`WBP_LoadoutScreen`) and 5
  (`WBP_GameVitals`) are captured, `yes`. These lines are in `results/s-bin/captures/procedure-log.txt` at corpus
  `cc7f222`.

  **3. A stalled session (D21-150 items 1 and 2, the procedure-log rule).** A capture session that stalls on the
  instrument (no capture written, no rejection, no framing judgement) gets its own line,
  `sha256=none shows_subject=stalled`, with its model and `cost_usd`. The item then gets **one** retry on a **fresh**
  capture copy under the unchanged step, and the retry's line binds. A second stall is a stop for a ruling. So an
  item's procedure log is at most one `stalled` line, then its one binding line (`yes` or `n/a`), and the binding
  line is the item's last. Every line's `cost_usd`, a stalled session's included, counts in the cap. Item 5's first
  session stalled ($0.76). D21-150's dated paragraph measured the cause on the retry: a UE compile-error modal
  blocking the game thread at `start_pie`. The retry captured ($1.00). Checkpoint 8i's `caplog_ok` reads it so, by
  dated edit in `s8i` §8i.9.

  **4. The driver (D21-149 item 3).** The driver accepts an empty capture list and renders it neutrally: the
  ChangeSummary's `captures` is `[]`, and nothing else in the summary or the brief marks the absence. A capture entry's
  note is its own asset's object path, by `asset_index`. The dry run no longer injects a synthetic capture. Its item 1
  renders and records item 1's real B1 capture (`capture-1.png`, sha256
  `9ba295de7b614c61b8e17585a5888ab30b654440d0b5e92de86d1a3f756a98d0`), committed in the dry run's review
  repository. A record's `synthetic_capture` is always false. Item 6's `captures[]` shape (`path`, `viewport`,
  `note`) is unchanged.

  **5. Item 4's per-copy free-space check is in the driver (D21-151).** Item 4's *"Before each copy, the free space
  on the root's volume must be at least `SBIN_NEED_BYTES`"* is now run by the driver before every dispatch copy,
  retries and B4 re-runs included. It reads the review root's volume as the batch pre-check does
  (`df -B1 --output=avail`). It compares that against the registered `SBIN_NEED_BYTES` = 34,221,223,118 (the extension
  of 2026-10-04). It refuses before the copy on a smaller reading or an unreadable one. The R2FIX-8i extension's item 5
  recorded the check as not in the driver and owed. D21-151 settles it: the check is built, and the batch pre-check
  (`s8i` §8i.5) stays as a second layer. The check narrows nothing that was registered.

  **6. The confound, disclosed (D21-149 item 2).** Capture presence follows asset type. Both seeds are widgets
  (items 4 and 5), so both seeded items carry a capture. Of the three clean items, only item 1 (a widget) carries
  one. Items 2 and 3 carry `captures: []`. Over the five items, P(seeded | capture) = 2/3, and P(seeded | no capture)
  = 0. A reviewer could read a capture's presence as a signal. Two things temper that: each reviewer sees only its own
  item, never the five-item distribution, and clean item 1 carries a capture. Item 1's tree is item 4's base tree,
  which the R1FIX-8i note in `s8i` §8i.5 makes visible. F43 must state this confound beside the false-positive
  outcome. Before B2, item 2's and item 3's constructed specs are checked for an acceptance criterion that names a
  capture (D21-149 item 4). That check is `s8i`'s owed item (e)'s, not this extension's.

  **7. Residuals, standing.** Every earlier extension's residuals stand. From this build:
  - The driver's production free-space reader (`probe` wiring `df`) is not reached by a unit test, because `probe`
    reads the cwd's corpus and `current_exe`. Its first real exercise is the re-run dry run. Its mutant is recorded
    as unobservable.
  - The AFK build read the driver's selection with its test scratch under the sweep root `d:/repos/redkiln-sev`, so
    four audit tests read red there, and red the same way at `cc7f222`. The binding 34-test reading is the
    operator's B0 block over the new runner commit (`s8i` §8i.5).
  - Nothing here changes a seed, an item, a ceiling, the registered `$ARGUMENTS`, or any part of METHOD-S §3.

- **s8i pre-dispatch amendment, extension: the five constructed `spec.md` and implementation reports, as the
  2026-09-09 entry's transcription is built — 2026-10-06, after B1 and before B2; no S-BIN record exists** (cites the
  2026-09-09 entry's *"Each item's `spec.md` is therefore **built from the story row plus the commit's own stated
  claim**, and the implementation report is the commit message rendered into the fix lane's shape. That
  transcription is part of the fixture and is registered here, not left to the builder"*, its three item-level
  defects, the *s8i pre-dispatch amendment* above and its items 3 and 6, and the extension of 2026-10-03, item 1, for
  R's clause. It edits nothing in them). Written in the AFK build, package D21149-8i-B, as `s8i` §8i.5's owed item
  (e). **Proposed: the operator reads and accepts or rejects the five pairs before any rendering is hashed.** The
  2026-09-09 entry registers what each spec is built from but not its words, so the builder's words are registered
  here, as one rule over all five, and disclosed.

  **1. The rule.** The shape is E3's fix-lane `spec.md` and `implementation-report.md` (§4.2's sections), built by
  one script (`D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_D21149-8i-B_specs.py`) reading BrittonGames read-only.
  - The story row is the backlog row **at the pin `041ead2c`** (the entry's *"ACs at the pin"*), its `title` and
    `verify` transcribed verbatim and never its `notes`, which are the closing evidence. Item 2 closes no story
    (defect 1), so it has none.
  - The ACs are the row's `verify` clauses, then the commit's own stated claims, each in E3's
    `AC-###: GIVEN … WHEN … THEN …` form with a `_Verification:_` line. Items 2 and 4 take their ACs from the
    commit's claim only (defects 1 and 3). Item 5's AC-001 is R's registered clause, verbatim.
  - The report's per-AC evidence cells quote the commit message, and each quote is asserted present in it. The
    commit message (its body read by git, the `Co-Authored-By` trailer dropped) is reproduced in full under
    *Notes*. The *Frozen PR boundary* and *Changes* tables name the change's assets only: every other path the
    commit touched is in the registered strip set and is not in the review tree (amendment item 3; the entry's
    `text_diff` scoping).
  - Item 4's row is transcribed **without its bold diagnosis sentence** (defect 3: *"must not include the story row's
    diagnosis"*), removed exactly once by an asserted anchor and without a marker in the text.
  - The slug is the story id (or `chore`) and words from the commit's subject.

  **2. The five pairs**, at `results/s-bin/spec-item-<n>.md` and `results/s-bin/report-item-<n>.md` (fixture.json's
  `spec_path` and `report_path`), LF:

  | item | slug | ACs | spec sha256 | report sha256 |
  | --- | --- | --- | --- | --- |
  | 1 | `8.5.5d-stash-row-face` | 6 | `11f11168…` | `9ae0c522…` |
  | 2 | `chore-ftest-inventory-stash` | 5 | `66923901…` | `a916fa90…` |
  | 3 | `8.3.9-install-routing-configuration` | 6 | `c08d409b…` | `00696b45…` |
  | 4 | `8.5.5b-loadout-column-opaque` | 4 | `d4ebdb9e…` | `f0a9f60f…` |
  | 5 | `2.8.5-vitals-styling-persists` | 4 | `25da68e7…` | `8848e41c…` |

  `ac_ids` are each spec's own AC ids by the runner's parse rule (`corpus::ac_ids`), read back by the driver's control
  copy. Each item has 3–6 ACs, as §3.2 registers.

  **3. Disclosed, not decided here.**
  - **The reports of the two seeded items carry the commit's own diagnosis, with registered leak tokens.** The
    entry registers the report as *"the commit message rendered"*, and E3's reports describe the defect they fix. So
    item 4's report quotes *"LoadoutFill's ColorAndOpacity alpha was 0.097 where 0.97 was intended"* (`LoadoutFill`
    twice), and item 5's carries `white-on-white`, `VERIFIED COLD` and `entirely white` once each. Item 5's spec
    carries `white-on-white` once, inside R's registered clause. The leak-token rule (amendment item 3) is about the
    review tree, which holds none of them. A report that states the defect as **fixed** is the claim the reviewer is
    asked not to take at face value; the story row's diagnosis, which states it as **open**, stays out of item 4's
    spec. If the operator reads the entry otherwise, the reports are rebuilt from the same script with the diagnosis
    sentences dropped, before any rendering is hashed.
  - Item 5's row at R reads *"2.8.2/2.8.4"* where the pin's reads *"2.8.2"*; the pin's is used, as for every item.
  - Item 1's and item 4's transcribed `verify` name captures *"tracked under `OurGame/Saved/Screenshots/`"*, which the
    review tree does not hold (gitignored). They are the rows' own words.

  **4. D21-149 item 4's check.** No acceptance criterion of item 2 or item 3 names a capture: the words `captur`,
  `screenshot`, `ue-verify` and `png` occur nowhere in either item's spec or report. Item 1's AC-001 verification
  names a `/ue-verify` PIE pass; items 1, 4 and 5 carry a capture.

  Nothing here changes a seed, an item, a ceiling, `$ARGUMENTS`, the rendering's mappings, or any part of METHOD-S §3.

- **s8i pre-dispatch amendment, extension: the five constructed spec/report pairs accepted as registered, and an
  F43 disclosure and evidence-source tag (D21-152) — 2026-10-06, before B2; no S-BIN record exists, and no rendering
  is hashed.** This paragraph cites the D21149-8i-B extension above and edits nothing in it.
  1. **Accepted as built.** The operator accepts the five pairs exactly as the D21149-8i-B extension tables them,
     reports included with the commit message's diagnosis (that extension's item 3, first bullet, is settled as K).
  2. **Disclosed in F43.** The seeded items' reports carry the commit's own diagnosis (widget, pre-fix value or state,
     and for item 5 the cold-verification method); the review tree does not. A §3.4 branch-1 result reads as
     *"sufficient given the report"*, not *"ChangeSummary alone sufficient"*, and branch 2's signal is less likely.
  3. **Recorded-only tag.** When F43 is written, each blocking finding is tagged by its evidence source: read from the
     asset (editor, Python or binary read, or a capture), or only strings that appear in the spec or report. The tag is
     never used in scoring; §3.4's decision rule is unchanged.
  4. **Considered and not taken:** cutting all five reports at `## Notes` (one mechanical rule, identical across
     items), because it departs from E3's fix-lane report shape and would be chosen after the B1 captures were seen.

- **s8i pre-dispatch amendment, extension: item 3's trees gain a registered path rule, and the absolute spellings of the BrittonGames path join the leak tokens, under D21-153 item 2 (a) — 2026-10-07, after pass 1 was discarded (D21-153 item 1) and before B2's re-run; PROPOSED** (cites the *s8i pre-dispatch amendment* above and its items 3 and 4, and the D21146-8i, D21147-8i and D21148-8i extensions above. It edits nothing in them or in the other extensions above). Written in the AFK build, package D21153-8i-A, under `spikes/DECISIONS-2026-09-21.md` D21-153, the operator's ruling of 2026-10-07. Its item 2 reads *"By a registered mechanical rule applied identically to all five trees, every absolute spelling of the BrittonGames path is removed from the trees' text files, and those spellings join the registered leak tokens, so the pre-spawn leak check refuses a tree that still holds one. The rule is the build package's to state and measure; a rule that would change a file the build or suite reads returns to the operator."*

  **1. The rule.** In every non-asset file of a tree (every path not ending in `.uasset` or `.umap`, in any letter case; `.git` is not the tree, as for the leak scan), each match of the byte pattern below is replaced by the 14 bytes `<project-root>`. The pattern is an absolute spelling of the BrittonGames repository root: a drive (`D:`), a Git Bash mount (`/d`, not after a name character, `.`, `~`, `/`, `\` or `-`) or a WSL mount (`/mnt/d`, the same), then `repos`, then `BrittonGames`, each joined by `/`, `\` or an escaped `\\`, in any letter case, and not continued by a letter, digit, `_`, `.` or `-`. In Python's notation: `(?i)(?:[a-z]:|(?<![A-Za-z0-9_.~/\\-])/mnt/[a-z]|(?<![A-Za-z0-9_.~/\\-])/[a-z])(?:\\\\|[\\/])repos(?:\\\\|[\\/])brittongames(?![A-Za-z0-9_.-])`. Only the root is replaced. The path's tail after it, every other byte and every line terminator are kept, so the rule is terminator-agnostic and idempotent; it neutralises the path and removes no file. A match inside a file whose first 8 KiB hold a NUL byte refuses, because the rule edits text only. It is applied after item 3's strip, the D21147-8i stub and the D21148-8i tooling blob, and before each tree's one commit, and it applies to the review repository's added `_s-bin/` files as much as to the base's. The one definition is `D:/repos/redkiln-sev/afk-tools/sp/s8i/s8i_D21153-8i-A_rule.py`, which the build, the verifier and the controls import. Its synthetic controls (`s8i_D21153-8i-A_rulectl.py`, `RULECTL_OK`) cover each spelling the pattern names, CRLF and two hits on one line, a UE log line, the cases it must leave alone (below), the binary refusal, the census in UTF-8 and UTF-16 and a Python module docstring; 15 mutants of the module were each killed by them (`D21153-8i-A-mutants.out`).

  **2. The leak tokens.** The spellings measured in the trees join item 3's registered tokens, exactly as they occur: `D:/repos/BrittonGames` and `D:\repos\BrittonGames`. The registered set is six: `LoadoutFill`, `white-on-white`, `VERIFIED COLD`, `entirely white`, `D:/repos/BrittonGames`, `D:\repos\BrittonGames`. `fixture.json`'s `leak_tokens` carries the six, so the driver's pre-spawn `precheck_leak_hits` (every non-asset file of the dispatch copy, by exact bytes, tracked or not) counts them and refuses a copy that holds one. The driver reads the tokens from the fixture and needs no change for them.

  **3. What the rule changes, measured** (`D21153-8i-A-inventory-text.json`, `D21153-8i-A-readset.json`, read-only over `D:/sb/base/item-1..5` and `D:/sb/rev/item-1..5`). Every occurrence of the word `BrittonGames`, in any case, in UTF-8 and in either UTF-16 order, was listed with its context. Only the two spellings above are absolute spellings of the repository root. At items 1 to 4 the rule changes the same 21 files and 29 spellings: 7 Markdown files (`AGENTS.md` and `CLAUDE.md`, each the Rider `rootFolder` line; `_workspace/references/Locomotion_Framework_Strategy.md` 1, `Migration_Tooling_Remediation_Plan.md` 3; `_workspace/runbooks/E1_Synty_Pack_Migration_Runbook.md` 6; `_workspace/tooling/Rider_CoDev_Setup.md` 2, `UE58_MCP_Setup.md` 1), `.cursor/rules/britton-ue58-conventions.mdc` 1, and 13 Python files whose one spelling sits in the module docstring's usage text (the eight `author_*.py`, and `synty/calibrate_deformation.py`, `ci_validate.py`, `strip_character_sources.py`, `diagnostics/diag_conform_axis.py` and `diagnostics/proto_orient_bones.py`; D21-146 item 4's measured class). At item 5 it changes 9 of the same files and 17 spellings (6 Markdown, the `.mdc` and two `synty/` scripts). In each review repository it also changes `_s-bin/report/headless.log`: 4 spellings at items 1 to 3 and 2 at items 4 and 5, UE's `Found project "(ProjectName="D:/repos/BrittonGames/…` lines, which name the real project and a `.claude/worktrees/` project under it. No line terminator moves in any file.

  **4. Left alone, and why.** The rule takes only absolute spellings of the repository root (D21-153 item 2 (a)). These remain, identically on all five trees, and are disclosed. The word as a name: `// Copyright BrittonGames.` headers, `Category = "BrittonGames|…"`, and headings and prose (in 323, 330, 340, 323 and 126 files at items 1 to 5). Relative spellings: `BrittonGames/` in `README.md` (2), `Docs/FOLDER_STRUCTURE.md` (item 5) and `_workspace/tooling/cursor-unreal-workflow.md` (item 5), and `cd BrittonGames`. The remote URL `https://github.com/Ryan-Britton/BrittonGames.git` in `README.md` and `Docs/GIT_LFS_SETUP.md`: a remote, not a path on this machine; whether a reviewer could fetch it with the account's credentials is not measured here, and it is the operator's. `UE-LocalDataCachePath = D:\DDC\BrittonGames` in `_workspace/tooling/UE58_Memory_Runbook.md`: a derived-data cache path, not the repository, and absent on D: [measured, `ls`]. The assets: the word occurs in none of the 29,927 assets of base 1 or the 11,823 of base 5 [measured, every asset read, `D21153-8i-A-inventory-assets1.out`, `-assets5.out`].

  **5. No file the build or the suite reads changes, so greenness is not re-run (D21-153 item 7).** The changed paths are `.md`, `.mdc`, `.py` and, in the review repositories, the greenness log. None is a build input by type (`.uproject`, `.uplugin`, `.cs`, C/C++ sources and headers, `.ini`, shaders, `.json`) or by place (`OurGame/Config/`, `OurGame/Source/`, `OurGame/Content/`, a plugin's `Source/`, `Config/`, `Resources/` or `Shaders/`). None is the instrument: `verify_headless.ps1` is unchanged, names no Python, and writes `headless.log` without reading it. None is read by the suite: every file-reading call in the trees' tests (`LoadFileTo*`, `FindFiles*`, `IterateDirectory*`) globs `.h`, `.cpp`, `.cs`, `.Target.cs`, `.uasset` or `.umap`, or reads a named `.cpp` or `.Build.cs`, the D21147-8i stub, or Monolith's own C++ fixtures under its plugin `Source/` (Monolith's tests, which the headless run disables). Each Python change is in the module docstring only (the module's AST without its docstring is identical before and after), and none of the 13 files mentions `__doc__`; none is under `Content/Python`, and no `PythonScriptPlugin` startup script is configured. So the build's and the suite's inputs in each new base are byte-identical to the old base's, whose binding greenness reports (the hashes the extension of the deferred extensions 2, 3 and 4 records) stand. The rule does change the review repositories' copy of each report's `headless.log`, an output, so the in-tree log differs from the corpus copy at `results/s-bin/greenness/item-<n>/headless.log` in those spellings only; the in-tree log is what `build.log_ref` points at, and its new bytes are recorded where the review repositories are rebuilt.

  **6. The five bases, rebuilt (D21-153 item 7).** By `s8i_D21153-8i-A_build.py`, beside the current bases, at `D:/sb/base-d21153/item-<n>`, then swapped in. Each new base is derived from its current base, not re-cloned: rebuilding all five from BrittonGames beside the old would have taken about 97 GB with D: at 128 GB free and a 60 GB floor. Every unchanged work file and every LFS object is an NTFS hard link to the current base's file; each file the rule changes is written fresh; then a fresh `git init -b main`, `add -A -f` and one commit `review tree` by `s-bin <s-bin@invalid>`, as before. Every later copy (greenness, capture, review, dispatch) is a byte copy, never a link. D: free space moved from 127.89 GB to 127.70 GB.

  | item | base commit | base tree | changed files / spellings | base bytes (`.git` included) |
  | --- | --- | --- | --- | --- |
  | 1 | `6b5e4e3d1b9806237cd3682d528b1b1a477b2aef` | `46c36cc92a2d19f44b53fe200c4e324d064cbd6e` | 21 / 29 | 21,540,843,154 |
  | 2 | `a8c2883ba4f5cc6e005d76ffd6e1df0894acba5a` | `44ce922f883d19c877761d443a38051e2e488e65` | 21 / 29 | 21,541,172,757 |
  | 3 | `6d664ea8bf995ffb4e921aae51240fe64d51873a` | `482397522ad0bbe3941320288378937b3eda4611` | 21 / 29 | 21,560,066,890 |
  | 4 | `5bb530c141139a85ca93e736fa9e8f5f560004b3` | `46c36cc92a2d19f44b53fe200c4e324d064cbd6e` | 21 / 29 | 21,540,843,059 |
  | 5 | `76095e7c86b59fb74ce3a0827bf2ffca5ae8a16d` | `9178e476e2e606f70dffa16217eddddef102557c` | 9 / 17 | 10,604,502,489 |

  Verified by `s8i_D21153-8i-A_verify.py` (`ALL_OK`, `D21153-8i-A-verify-all.json`). Each tree equals its current base's tree entry for entry, except exactly the files the rule matches among the old tree's non-asset blobs (found independently from those blobs), each with its mode unchanged and its blob that of the rule over the old blob; each changed work file is the rule over the old work file. The rule's pattern, both path tokens and the four seed tokens count 0 over every non-asset file, in UTF-8 and in a UTF-16 reading. The checks D21-147 ran are re-run and pass: every LFS pointer's work file matches its pointer and its object is present (30,014, 30,015, 30,022, 30,014 and 11,886 pointer paths; 29,576, 29,577, 29,584, 29,576 and 11,447 distinct objects; 0 bad); the seeds by blob oid (`9415f098…` at item 4, `23c79a63…` at item 5), the fixed blobs and their LFS objects absent, the control probes present; one commit, `refs/heads/main` alone, no remote or tag, `status --porcelain --untracked-files=all --ignored` empty; the D21147-8i stub; the launcher absent; no executable file with an absolute answer-bearing path in code; `verify_headless.ps1` blob `ebe7e58f` on all five; items 2 and 3's `NoInertCapability` spec BrittonGames' blob; the stripped set the one four-entry set. Items 1 and 4 still hold one tree. Verifying a current base against itself reads `NOT_OK` (the control). BrittonGames' HEAD, status, worktree-list and config sha256 were the same before and after. The new bases were then swapped into `D:/sb/base/item-1..5` by renames, and re-verified there against the old bases, kept aside at `D:/sb/base-old-d21153/item-1..5` (`ALL_OK`, `D21153-8i-A-verify-inplace.out`). The old bases are not deleted; their unchanged files are the new bases' hard-link partners.

  **7. What moves downstream, and what does not.** The seeds, the items, the strip set, the stub, the verify script and every greenness report stand. The base commits and trees move, so the five review repositories, `fixture.json`'s base keys and `review_commit`s, its `leak_tokens` (six) and the dry run's review repository are rebuilt from the new bases (D21-153 item 7), with the rule applied to the review repositories' added `_s-bin/` files. The rule leaves the driver's selection and record names (34) unchanged.

  **8. Residuals standing.** Every earlier extension's residuals stand, and §4's are added. The rule changes no part of METHOD-S §3.

- **s8i pre-dispatch amendment, extension: the driver follows D21-153 items 2, 3, 5 and 6: the path tokens in the pre-spawn scan, the rendering's statement of the review's limits, the audit's cwd model, and pass 1's discarded set as a records file of its own — 2026-10-07, after pass 1 was discarded (D21-153 item 1) and before B2's re-run; PROPOSED** (cites the *s8i pre-dispatch amendment* above and its items 3 to 6 and 10, the R3FIX-8i, R5FIX-8i and R6FIX-8i extensions' audit rules and the D21153-8i-A extension above. It edits nothing in them). Written in the AFK build, package D21153-8i-B, under `spikes/DECISIONS-2026-09-21.md` D21-153, the operator's ruling of 2026-10-07, whose items read *"(d) The audit's cwd model follows the CLI, red first: a `cd` inside the copy (or into temp the session wrote) is kept, and a `cd` outside it is audited as the read it is and then reset, so later relative words resolve in the copy."*, *"(e) A new arm or records file, so that pass 1's `VoidedTwice` does not carry into the re-run."* and *"(b) The motive is answered in the rendering. It states plainly that the review has no runtime, no PIE, no built binaries and no base checkout, and that any read outside the review tree voids the record."* The driver is `runner/src/sbin.rs` as `S8I-D21153-8i-sbin-driver.patch` gives it against corpus `07bd6a2` (sha256 `2dc0a74b757bcaeb391f6c53818d6ec6884aac61d647e2395a6b8ee2cfc5ee0a`). *(dated 2026-10-08, s8i build, D21153-8i-E: superseded by `1d6a8882…5203`, the D21153-8i-E extension below, item 8)*

  **1. The cwd model (item 5).** The R3FIX-8i extension's item 1 resolved a relative path against *"the copy, as moved by `cd`"*, and a `cd` persisted across `Bash` calls wherever it went. Now, after each `Bash` call, the audit leaves the cwd the CLI leaves: a cwd inside the dispatch copy is kept; a cwd strictly below the temp directory and at or above a path the same session wrote (the shadow audit's written paths: a `>`/`>>` target, a `cp`/`mv` destination, the `Write` tool's path) is kept; any other cwd, the review root's other trees, the engine install and the temp directory itself included, is reset to the copy. The `cd` that left the copy is audited when it runs, as before, and inside its own command the `cd` holds (`cd X && grep -rn .` reads `X`). The reset is the cwd every later tool sees, a `Grep` or `Glob` tool's relative path included. Measured over pass 1's six transcripts (corpus `07bd6a2`, `results/s-bin/raw/`), replayed through both audits with B2's temp directory and account roots named (`D21153-8i-B-replay-old.out`, `-new.out`): the old audit reproduces the records exactly (item 1's voids 25 and 64; the shadow flags 26, 19, 17 and 6 at items 2 to 5), and the new one gives 5 and 3 runtime hits at item 1 (every one a read of BrittonGames or the `find /d /c/Users/ryanm` sweep), with items 2 to 5 unchanged. No record's void status moves.

  **2. The path tokens in the pre-spawn scan (item 2).** The D21153-8i-A extension's item 2 said the driver *"needs no change"* for the six tokens, because it reads them from `fixture.json`. The driver now also holds the two path tokens, `D:/repos/BrittonGames` and `D:\repos\BrittonGames` (`LEAK_TOKENS_PATH`), and counts them in `precheck_leak_hits` whatever the fixture lists, so a copy that still holds one refuses before the spawn even under a fixture that omits them. When a copy both holds a token in an executable file and fails `precheck_exec_paths`, the refusal names the executable-path finding first (D21-146's check stays observable); both refuse. The scan is by exact bytes, as for every token: other spellings (`/d/repos/…`, another case) are the rule's to remove at build (D21153-8i-A, verified 0 by its pattern), not the scan's to catch.

  **3. The rendering's statement (item 3).** The registered rendering gains one fixed paragraph, right after the ChangeSummary and before E3's spec, in the same bytes for every item and run (the pass's, an item with `captures: []`, and B4's): *"The limits of this review: it has no runtime, no PIE, no built binaries and no base checkout. Any read outside this review tree voids this review's record."* (sha256 `66ccdca02cffd52180eaab41b663db6f55b2ab3cfe2e58e357c09b150204b8b4`, `REVIEW_LIMITS`). It is stricter than the measured runtime void, which still voids only the registered roots (*"Paths outside all of these, such as the engine install, are not out-of-tree"*); a reviewer reading the engine install is not voided. Every item's rendering hash moves, so pass 1's `results/s-bin/rendered/item-<n>.txt` move aside with the set (§5), or `render_item` refuses.

  **4. A records file, not an arm (item 6).** The arm stays `b-typed-sbin` (METHOD-S §0, `audit-s.py bin` and Checkpoint 8i read it). Pass 1's records file is moved aside, byte for byte, to `results/s-bin/superseded/pass1-d21153/s-bin.jsonl` (sha256 `830b957b65253a8c174b0316e24efc1de1d4040ae6080c514d2668dc831e95ef`, blob `results/s-bin.jsonl` at `07bd6a2`: six records, $16.074148), and the re-run writes a fresh `results/s-bin.jsonl` under the same registered argv, so resume (`next_attempt`) and B4's missed-original lookup never read pass 1 and its `VoidedTwice` does not carry. The driver reads the discarded set (`SUPERSEDED_PASS1`, `read_superseded`) in every run and refuses before any copy when it is absent or at other bytes. It counts toward the caps (its six Opus dispatches and its spend, D21-153 item 8) and its copy paths are used paths; in a real run it is also the set's first record for Halt 2 and the model halt (the dry run's stub reports CLI `0.0.0` and model `sbin-stub-replay`, so the dry run is exempt from those two, as `check_launch` and `check_ceiling` already exempt it). Because the account keeps a `projects/` entry for each of pass 1's copy slugs (`D--sb-d-p1i1a1` and the rest, measured in the account's `projects/`), the re-run's dispatch copies sit under `<root>/d2/` (`DISPATCH_DIR`), a directory pass 1 never used: `D:/sb/d2/p<pass>i<n>a<attempt>`, slug `D--sb-d2-…`. The raw transcripts keep their stems (`results/s-bin/raw/p1i1a1.jsonl`), so pass 1's `raw/p1i*` move aside with the set.

  **5. What the operator's commit moves, for this to run.** Before B0's dry run, by `git mv`, under `results/s-bin/superseded/pass1-d21153/`: `results/s-bin.jsonl` (to `s-bin.jsonl`, the registered path and bytes), `results/s-bin/raw/p1i*.jsonl` and `p1i*.stderr.txt`, and `results/s-bin/rendered/item-1.txt` to `item-5.txt`. A copy directory `D:/sb/d2` must not exist. The dry-run script checks the first and the last.

  **6. What moves downstream, and what does not.** The driver's selection is 36 tests (two named tests added: `a_cd_outside_the_copy_is_audited_then_reset_as_the_cli_does`, `pass_ones_discarded_set_counts_but_never_resumes`); the record keeps its 34 names. Two earlier test expectations moved under D21-153 and are disclosed: R5FIX-8i's two-call *"cd to an ancestor, then grep -rn ."* case becomes the one-call form (the two-call form is pinned as reset), and D21146-8i's spawn controls naming `D:/repos/BrittonGames` in usage text, a comment and Markdown now refuse on the leak count alone, with `precheck_exec_paths` still empty, and spawn once the rule's `<project-root>` replaces the root. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the review repositories and `fixture.json` re-registered under D21-153 item 7 — 2026-10-08, after pass 1 was discarded (D21-153 item 1) and before B2's re-run; PROPOSED** (cites the *s8i pre-dispatch amendment* above and its items 3 and 4, and the D21149-8i-B, D21153-8i-A and D21153-8i-B extensions above. It edits nothing in them). Written in the AFK build, package D21153-8i-C, under `spikes/DECISIONS-2026-09-21.md` D21-153, whose item 7 reads *"Rebuilt at zero spend: the five bases and review trees, the fixture, a new runner commit, the B0 block, a new dry run, and refuting rounds."*

  **1. The review repositories.** Each is built as the D21149-8i-B extension registers it, by the driver's `commit_single`, from the D21153 base (D21153-8i-A). There are three differences, each named here. The D21153-8i-A rule applies to the added `_s-bin/report/headless.log`, as that extension's item 1 says it does. The work files and the LFS objects are NTFS hard links to the base's, each LFS object hashed to its oid before it is linked, because five byte copies beside the old five did not fit D:'s floor; every copy made from a review repository stays a byte copy. The commit's author and committer dates are pinned at `1791331200 +0000`, so a rebuild reproduces the commit. The commits are `46f1b0fc…`, `afa1bfa0…`, `65476210…`, `dab720e6…` and `98e79e85…` (`s8i` §8i.4's D21153-8i-C note). The corpus's greenness copies stay the binding reports (`tests.report_ref` and `build.log_ref` still name the tree's copies), and each tree's `headless.log` differs from its corpus copy in the rule's spellings only.

  **2. `fixture.json`.** It is re-registered from pass 1's `a93044cf`: `leak_tokens` becomes the four seed tokens and the two path tokens; each item's `base_commit`, `base_tree`, `base_bytes` and `review_commit` move; nothing else does. Staged sha256 `43bfe9c0c8c235480e56c977f48cba199452ca81c2f6f617e2e5c288f7b47091`. Pass 1's fixture is kept, moved beside pass 1's set to `results/s-bin/superseded/pass1-d21153/fixture.json`.

  **3. What the operator's move commit holds.** It is the D21153-8i-B extension's item 5 list, plus pass 1's `fixture.json` and the D21149 dry run (`ed280009`, moved to `results/s-bin/superseded/dry-run-ed280009/`), because the dry-run script writes only into empty slots. The new `fixture.json` is committed after the D21153 dry run, as one path.

- **s8i pre-dispatch amendment, extension: the s8i verify panel's findings over D21-153's build — 2026-10-08, before B2's re-run; PROPOSED** (cites the D21153-8i-A, D21153-8i-B and D21153-8i-C extensions above. It edits nothing in them, except by the dated notes named below). Written in the AFK build, package D21153-8i-E, under `spikes/DECISIONS-2026-09-21.md` D21-153 and the operator's rulings of 2026-10-08 recorded there.

  **1. A drive-relative `cd` target.** The D21153-8i-B extension item 1 audits *"The `cd` that left the copy"*. A `cd` or `pushd` target that is a drive `x:` (any case) not followed by `/` or `\` is now read as Git Bash's `cd` reads it: `x:/`, then the rest. The panel measured `cd D:repos` giving `/d/repos`, and `cd D:` and `cd D:..` giving `/d`. The fold is applied before the target is audited and before the cwd moves. A drive-relative word that is not a `cd` target (`cat D:../x`) is not folded; it is a residual for the refuting rounds.

  **2. `flag_probe`'s caps.** B1's flag probes now count the discarded set's spend toward the USD cap, as `dispatch_one` does (D21-153 item 8). They refuse when the registered set is absent or at other bytes.

  **3. Owed: the temp clause (ruling (r6)).** The D21153-8i-B extension item 1 says *"a cwd strictly below the temp directory and at or above a path the same session wrote … is kept"*. That is not the CLI's rule under either reading: the CLI cannot see which paths the session wrote, so it either keeps a temp cwd or resets it. The clause stands unchanged until the operator's measurement on CLI 2.1.289 settles which, and the audit is then built to the measured rule before the runner commit. The cases are staged in `D21153-8i-E-cwd-redcases.txt`.

  **4. The path rule's look-ahead.** The D21153-8i-A extension item 1's *"not continued by a letter, digit, `_`, `.` or `-`"* becomes *"not continued by a letter, digit, `_` or `-`, or by a `.` followed by a letter, digit or `_`"*: a sentence-final `.` ends the root. In Python's notation the pattern's tail is `brittongames(?![A-Za-z0-9_-]|\.[A-Za-z0-9_])`. Over the five bases and the review trees the fixed rule matches 0 times, so it changes 0 bytes and nothing is rebuilt (`MEASURE_OK`).

  **5. Provenance of the bases.** The five bases were built under the pattern with the drive look-behind, `(?<![A-Za-z0-9_])[a-z]:`, which their records carry. The look-behind was removed before the module was registered. The registered rule, the pre-removal pattern and the fixed rule each reproduce every new blob from the old base's, byte for byte. The panel measured this, and D21153-8i-E re-measured it over all five items.

  **6. The token scan is case-exact, and the rule is the case-insensitive backstop.** The driver's `precheck_leak_hits` counts the six tokens by exact bytes. A spelling in another case or mount form, such as `d:/repos/brittongames` or `/d/repos/BrittonGames`, is the rule's to remove at build: the rule is case-insensitive and is verified 0 over every tree. The scan does not catch such a spelling.

  **7. One inode set.** `D:/sb/base`, `D:/sb/base-old-d21153` and `D:/sb/rev` share their unchanged work files and LFS objects as NTFS hard links (26,504 inodes in item 5, measured by the panel). None of the three is ever edited in place. The D21153-8i-A extension item 6 says *"Every later copy (greenness, capture, review, dispatch) is a byte copy, never a link"*. That holds for greenness, capture and dispatch, but not for review, as the D21153-8i-C extension item 1 discloses. Before B2 the packet re-verifies the bases and the review repositories (step 10b).

  **8. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `a2d1953d340fe81b38e07b68e519650b1e3118ceb9c8dd425dbac64c2f8c1e40`) gives against corpus `07bd6a2`: sha256 `1d6a8882345a904cabc2538c8a60aacd4b92ae1861834c1106695baa52205203`. This supersedes the D21153-8i-B extension's `2dc0a74b…`. The selection stays 36 tests, and the record keeps its 34 names. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the audit's cwd model to CLI 2.1.289's measured rule, and OLDPWD neutralised at the spawn — 2026-10-08, before B2's re-run; PROPOSED** (cites the D21153-8i-B and D21153-8i-E extensions above. It edits nothing in them). Written in the AFK build, package D21153-8i-G, under `spikes/DECISIONS-2026-09-21.md` D21-153: ruling (r6), the operator's rulings on the cwd probe ((4): *"the item's lead, 'The audit's cwd model follows the CLI', governs for temp"*), and the operator's ruling of 2026-10-08 on OLDPWD, relayed: *"cd - / OLDPWD: Neutralise + audit"*.

  **1. Temp (supersedes the D21153-8i-B extension item 1's temp clause, and settles the D21153-8i-E extension item 3).** Measured on CLI 2.1.289: a cwd under the temp directory, the directory itself included, written by the session or not, is reset silently to the cwd the call started in. The audit resets it there. Any other cwd outside the copy is reset to the copy, as before.

  **2. OLDPWD.** Measured: every call's OLDPWD is the value the child inherited, never the previous call's, and `cd -` follows it out of the copy. The driver now spawns every `claude` child with `OLDPWD` set to its copy and records it as `spawn_oldpwd`. The audit starts each call's OLDPWD at that value, follows it through each `cd` inside the call, and resolves `cd -`, `~-`, `$OLDPWD` and `${OLDPWD}` to it. One outside the copy, or unknown, voids: stricter than a `cd` target's read audit, as the backstop to the spawn.

  **3. The pushd stack** is empty in each call, as the audit already assumed.

  **4. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `b8667aa5828bee31b7e9cdc1a92a299b4d0c88a322dbd42e2ad7a3715a64f0f7`) gives against corpus `07bd6a2`: sha256 `d3015a8f310906f0b2255d1eb19ebfe4a68086632541db3d59e227b931480d71`. This supersedes the D21153-8i-E extension item 8's `1d6a8882…`. The selection stays 36 tests, and the record keeps its 34 names; records gain `spawn_oldpwd`. METHOD-S §3 is unchanged.

  **5. Residuals.** Whether 2.1.289 hands each Bash call the OLDPWD the driver sets is not measured live. A nested shell's OLDPWD starts at the spawn's, where bash would pass the current one. Every earlier extension's residuals stand.

- **s8i pre-dispatch amendment, extension: a bare `cd` is `cd ~`, OLDPWD kept in step with bash, and the directory changes the audit does not model void — 2026-10-09, before B2's re-run; PROPOSED** (cites the R4FIX-8i extension, *"which tool inputs item 4's out-of-tree rule reads"*, and the D21153-8i-G extension above. It edits nothing in them). Written in the AFK build, package D21153-8i-H, after `s8i` refuting round 12, under the operator's rulings (9) and (10). The operator gave them in chat on 2026-10-09, they were relayed by the orchestrator, and the operator writes them into D21-153 in their own words.

  **1. A bare `cd` (supersedes the R4FIX-8i extension item 6's residual line *"`cd` with no operand, which goes to `$HOME`, leaves the cwd unchanged."*).** A `cd` with no non-option operand (`cd`, `cd -P`, `cd -L`, `cd --`) is `cd ~`. It is audited as the read of the home directory that it is, it sets OLDPWD to the cwd it leaves, and it moves the cwd to the home directory. A later relative read in the same call then resolves there, and voids under the sweep's roots (`~/.claude-personal`, `~/appdata/local/jetbrains`, the temp `claude` root) and under the account roots' `projects/`. With the home directory unknown, it voids. The residual was disclosed before R5FIX-8i registered roots under the home directory. Round 12 measured it reaching them, with the shadow audit clear too.

  **2. OLDPWD in step with bash (extends the D21153-8i-G extension item 2).**
  - `popd` sets OLDPWD to the cwd it leaves.
  - An assignment to OLDPWD sets it to the expanded value, so the spawn's value no longer overrides it. This holds for a bare or prefix assignment, and for one through `export`, `declare`, `local`, `typeset` or `readonly`. An empty value is unknown.
  - A declaration builtin's value is tilde-expanded and audited, as a bare assignment's is.
  - `~-` after `=` or `:` is an OLDPWD word, so it voids when OLDPWD lies outside the copy or is unknown.

  **3. Not modelled, so voided.**
  - A bare `pushd`, which swaps the top two.
  - A `+N` or `-N` operand of `pushd` or `popd`, which rotates or removes an entry.
  - Any `cd` or `pushd` once the session has assigned or exported `CDPATH` (bare, prefix, declared, or `export CDPATH`). It holds for the rest of the session, as the audit's variables do.
  - An `eval`, or a `source`/`.` of a staged file, whose commands change directory. Each runs in the calling shell.

  `builtin cd` and `command cd` were already followed as `cd`.

  **4. Rulings (5) to (8).** These are in the operator's paragraph under D21-153, *"The operator's rulings after the cwd probe"*. The D21153-8i-G extension's temp rule (item 1) and OLDPWD rule (item 2) are now the operator's rulings (5) and (6), not proposals. SPAWN in step 0m's form is accepted (7), and the moved expectation in the cwd test is approved under H-12 (8).

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `21d3b3725db8d3c71f0b6fba5d60206c861705f5cf50809cf440880fc65f2bc4`) gives against corpus `07bd6a2`: sha256 `33e87bd08925ab6c264a850a1f12a0107f19f35fd5d34a34d74939c01f86f545`. This supersedes the D21153-8i-G extension item 4's `d3015a8f…`. The selection stays 36 tests, and the record keeps its 34 names. METHOD-S §3 is unchanged.

  **6. Residuals, disclosed.**
  - A `CDPATH` the child inherits at spawn is not seen; the driver neither sets nor clears it.
  - `pushd -n <dir>` and `popd -n` are modelled as moving the cwd.
  - A `source` of a file the session did not stage is not followed.
  - A nested shell's OLDPWD starts at the spawn's (the D21153-8i-G extension item 5).

  Every earlier extension's other residuals stand.

- **s8i pre-dispatch amendment, extension: the registered path rule extended to the greenness copy, and its two spellings registered leak tokens — 2026-10-09, before B2's re-run; PROPOSED** (cites the D21153-8i-A, D21153-8i-C and D21153-8i-E extensions above. It edits nothing in them). Written in the AFK build, package D21153-8i-I, after `s8i` refuting round 12, under the operator's ruling (11). The operator gave it in chat on 2026-10-09, it was relayed by the orchestrator, and the operator writes it into D21-153 in their own words.

  **1. The greenness-copy clause (extends the D21153-8i-A extension item 1).** In every non-asset file of each tree, each absolute spelling of a greenness copy's root `D:/sb/green/item-<n>` is replaced by the 16 bytes `<greenness-copy>`: a drive (`D:`), a Git Bash mount (`/d`) or a WSL mount (`/mnt/d`), then `sb`, then `green`, then `item-` and one or more digits, joined by `/`, `\` or an escaped `\\`, in any letter case, and ended as the BrittonGames root is (not continued by a letter, digit, `_` or `-`, or by a `.` followed by one). The tail, every other byte and every terminator are kept; a match in a file holding a NUL in its first 8 KiB refuses. The BrittonGames clause is unchanged. The module is `s8i_D21153-8i-A_rule.py`, sha256 `4bd3b61aa742c6603bb11e846f530a880962c0b859bf57de06d882b058d94b9c`.

  **2. The leak tokens (extends the D21153-8i-A extension item 2 and the D21153-8i-B extension item 2).** The spellings measured in the review trees' `_s-bin/report/headless.log` are `D:/sb/green/item-<n>` and `D:\sb\green\item-<n>`; their roots, `D:/sb/green` and `D:\sb\green`, join the registered tokens, which are eight. The driver compiles them in `LEAK_TOKENS_PATH` beside the BrittonGames root's, and `fixture.json` lists them. The scan stays case-exact; the rule is the case-insensitive backstop (the D21153-8i-E extension item 6).

  **3. What it changes.** Only `_s-bin/report/headless.log` in each review repository (124, 124, 126, 126 and 119 spellings); nothing in the bases or the dry run's review repository. The build and the suite do not read that file (`READSET_OK`), so greenness is not re-run and the five binding greenness reports stand. The five review repositories are rebuilt by `commit_single` with dates pinned at `1791504000 +0000`: `379033a5…7987`, `891e1de7…32c9`, `c39aeff2…587a`, `26c2f4d0…9162`, `e2a7be96…4e86`. `fixture.json` is re-staged at `82add83e5cf2ef8f3ccd8bb7e561afd40a14274496bedd49d9d0ea4b369397cb` (eight tokens, the five review commits); the dry run's fixture is `d9329924791e17751f141c09bf1f1003f72f7440bdb1545cf0932f05b97bbb43` (eight tokens); the dry run's review repository is unchanged (`434286de…`, reproduced).

  **4. What else remains.** After the rule, every file of the five bases, the five review repositories and the dry one, assets included, holds 0 matches of either clause, 0 of the eight tokens and 0 absolute `sb` or `repos` spellings in every non-asset file; and in the assets, which neither the rule nor the leak scan reads, no `D:/sb/` or `D:/repos/` spelling either: the broad scan finds only a third-party plugin's `G:/Repos/Black_Eye_Cameras/…` in 28 BlackEye editor-image assets (30 UTF-8 and 90 UTF-16 readings per tree), one binary byte run `/x/sb` in `SM_Prop_Market_Label_01.uasset` (no path), and `LoadoutFill` 3 times in assets of items 1 to 4 and the dry one (an asset's own name).

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `19043e585b10b6f278b661f56c82bae01701fe94295e371c54fdb8de1cd46653`) gives against corpus `07bd6a2`: sha256 `e32f054d8172b951315052ae49dfccf0352243d00226dc9d02f8cbadcf01cf75`. This supersedes the D21153-8i-H extension item 5's `33e87bd0…`. The selection stays 36 tests, and the record keeps its 34 names. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the out-of-tree audit is fail-closed — 2026-10-10, before B2's re-run; PROPOSED** (cites the R4FIX-8i, R5FIX-8i, D21153-8i-G and D21153-8i-H extensions above. It edits nothing in them). Written in the AFK build, package D21153-8i-J, after `s8i` refuting round 13 and the temper:rust-reviewer's findings, under the operator's rulings (12) and (13). The operator gave them in chat on 2026-10-09/10, they were relayed by the orchestrator, and the operator writes them into D21-153 in their own words.

  **1. The contract.** Every read the audit can resolve is checked against the roots and names; every read it cannot resolve voids. The void is the runtime void, not only the shadow flag. It applies to a `Bash` command's read-position words and `cd`/`pushd` targets; a tool's path field and a heredoc's data are read as before, except item 4's short name.

  **2. What the audit resolves.** Bash quoting, lexed on the raw command before any `\` to `/` fold: an unquoted backslash escape; single and double quotes; `$'…'` with its named, `\xHH`, `\NNN`, `\uHHHH`, `\UHHHHHHHH` and `\cX` escapes; `$"…"`. Then the tracked variables, `$PWD` (the cwd), `~-` and OLDPWD as before, `~` and `~<user>` for the account's own user (the home directory's last segment), and each command substitution whose output the audit can name (`pwd`, `cygpath`, `realpath`, `readlink`, `dirname`, `basename`, `git rev-parse --show-toplevel` in the copy, `mktemp`, a one-word `echo`, and a listing by `find`, `ls`, `grep -l`/`-L` or `git ls-files` through line filters). A `for` variable is each of its words, up to 64 combinations. An unquoted glob or brace expansion is the region below its literal prefix: inside the copy it is in-tree; anywhere else it voids. `git -C` chains, `tar`/`make -C` and `env -C` set the directory their operands resolve in. A `test` reads only a file operator's operands.

  **3. What voids because the audit cannot resolve it.** A word holding an escape it did not decode; a `$` left after expansion (an unset variable, a `${NAME…}` operation, an array subscript, a special parameter, an arithmetic expansion, a substitution whose output it cannot name; an assignment's value may hold one, and the name is unknown from there); a `~name` other than the account's own user and a dirs-stack tilde; `~` and a bare `cd` once HOME is reassigned in the session; a brace alternative that holds a separator, climbs or starts a tilde; a drive-relative word; a write to HOME, USERPROFILE, OLDPWD or CDPATH by `printf -v`, `read`, `mapfile`/`readarray`, `getopts`, `unset`, `+=`, a nameref or a `for` loop; `pushd -n` and `popd -n`; a `cd`, `pushd` or `popd` once a shell function is defined, or with an unresolved target; `source` or `.` of a file the session did not write; an `eval`, `source` or `.` that changes directory or writes a guarded name; xargs over input the audit cannot read. A child shell's or interpreter's unread script or stdin is flagged in shadow only (ruling (12) names `eval`, `source` and `.`, which run in this shell).

  **4. A Windows short name** outside the copy (an 8.3 segment such as `CLAUDE~3`, which on the home volume is `.claude-personal`) voids from any tool, because Windows resolves it for every program.

  **5. The spawn.** Every `claude` spawn the driver makes removes CDPATH, beside setting OLDPWD to the copy (the D21153-8i-G extension item 2). This closes the D21153-8i-H extension item 6's first residual; its second (`pushd -n`, `popd -n`), third (an unstaged `source`) and fourth (a nested shell's OLDPWD, which now starts at the running shell's) are closed by items 3 and 2.

  **6. Residuals, disclosed.** The audit is lexical: a junction, symlink or hard link that aliases a root is read under the name it is spelled by. A program's own directory option other than those of item 2 is not followed (its directory is audited as a read). An interpreter's inline code stays as R8-RECORD-8i disclosed. A prefix `HOME=` assignment voids every later `~` in the session, which over-voids.

  **7. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `52200ae5b29192101a3ca49fe07a08b81d51a381fa922c3eaa7b2a409c28110d`) gives against corpus `07bd6a2`: sha256 `324013c2c80c7db21dfb94fcf99925ae436e4a11a616dea3d18e334d9a6385a1`. This supersedes the D21153-8i-I extension item 5's `e32f054d…`. The selection stays 36 tests, and the record keeps its 34 names. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the out-of-tree audit's threat model, its bounds, its cheap closes and its residuals — 2026-10-10, before B2's re-run; PROPOSED** (cites the D21153-8i-J extension above. It edits nothing in it). Written in the AFK build, package D21153-8i-K, after `s8i` refuting round 14 and the temper:rust-reviewer's findings on D21153-8i-J, under the operator's ruling (17). The operator gave it in chat on 2026-10-10, it was relayed by the orchestrator, and the operator writes it into D21-153 in their own words.

  **1. The threat model.** The audit's adversary is the honest, curious reviewer: a model that wants context the review tree does not hold, and reaches for it with ordinary shell. Every out-of-copy read such a reviewer would plausibly make voids. Every real void so far was one: `git` in the main repository, `ls /d/sb`, `find /d`, a `cd` into BrittonGames. Ordinary shell the audit cannot resolve stays fail-closed, under the D21153-8i-J extension's contract. Honest in-copy work does not void. Deliberate obfuscation is outside the model: bash escape tricks, deep nesting, and Windows path-canonicalisation spellings. Where closing a class of it is cheap and fail-closed, it is closed (item 3). Otherwise it is a residual, disclosed in item 6.

  **2. The bounds (H-03, H-13).** The audit finishes on any input:
  - The lexer follows at most 64 substitutions nested inside quotes inside substitutions. A command it cannot lex (that, or an unclosed quote or substitution) voids.
  - A tracked value or an expansion past 4,096 bytes is unknown, and a read of it voids.
  - A command past 64 KiB voids unread.
  - A run past depth 8 voids, where it used to pass unread. A run that repeats one it is inside (the same text, cwd and variables) is not followed again.
  - Brace and glob masking is one linear pass.
  - Every void on something the audit cannot resolve also flags in shadow.

  Measured: on a 1 MiB stack in the debug build, 10,000 random and adversarial commands and 11 shaped ones finish unpanicked; the slowest random one takes 4 ms, and the slowest shaped one (10,000 nested `$(`) takes 1.07 s.

  **3. The cheap fail-closed closes.** These void in a command word and in a tool's path:
  - a UNC or device path (`//host/share`, `\\host\share`, `\\?\`, `\\.\`, `//?/`, `//./`);
  - a segment ending in `.` or a space (Win32 trims it; `.` and `..` excepted). In a tool's path it always voids; in a command word, only on a drive path once folded, or in a word that climbs;
  - a `:` after the drive designator, an alternate data stream: in a tool's path always, in a command word on a drive path;
  - any input holding the audit's own marks (U+0001, U+E000 to U+E01F), and a `$'…'` escape that decodes to a private-use mark;
  - `${NAME:=…}`, `${NAME=…}`, `${NAME:?…}` and `${NAME?…}`, which write a guarded name. Any other name written this way is unknown;
  - any write to PWD;
  - `find -files0-from`; find's start points are read after its leading options;
  - a `popd` after `dirs -c`, `+N` or `-N`;
  - a trap whose action (audited as `eval` would run it) reads out of the copy, moves the cwd or writes a guarded name;
  - `shopt -s cdable_vars` or `expand_aliases`, and any alias definition;
  - the command substitutions of an unquoted heredoc, audited as the commands they are.

  **4. What the audit now resolves (the false-void fixes).**
  - Revision, count and date tokens, which are never a path:
    - git's output inside the copy: `rev-parse <rev>`, `rev-list`, `merge-base`, `symbolic-ref`, and `log` (or `show -s`) with a hash-only format;
    - an arithmetic expansion;
    - `date` with a field-only format;
    - `seq` with numbers only.

    A token in an absolute or tilde path, or beside `..`, voids. Every operand of git is still a read, so `git diff` against an outside path voids as before.
  - `git rev-parse --git-dir` is the copy's `.git`.
  - git's `--name-only` and `--name-status` listings (not `--no-index`; `show` and `log` with an empty format).
  - `dirname` of a listing is the listing's region.
  - A `case` subject is not a read.
  - A `$NAME` followed by a quote or an escape (`"$d"Source`).
  - `read NAME` fed a listing or revisions.
  - Arrays of literal words.
  - A function's positional parameters, bound to every operand its calls pass (not after `shift` or `set`).
  - A substitution inside a substitution, within the run depth.

  Measured, before (D21153-8i-J) and after: the real transcripts (326 tool uses) have 0 false voids both times. Round 14's 460 honest commands void 37 times before and once after, the genuine `ls ../`.

  **5. What still voids (a disclosed false-void surface).** Each of these is an honest command the audit cannot resolve to a definite path, kept fail-closed:
  - a `cd` into a glob's or a listing's region (`cd "$(dirname "$f")"`; ruling (12)'s cwd move);
  - an unmodelled substitution in a `for` list (bash globs its output, a read of an unknown place);
  - a substitution the audit cannot name, in a read position;
  - two tokens joined by `..` (`$(…)..$(…)`), which might both print nothing;
  - an absolute `sed` or `awk` address holding a `:` on a drive path;
  - a command with an unclosed quote, which bash itself refuses;
  - a nested run past depth 8;
  - a value past 4,096 bytes;
  - an array or a positional parameter whose binding the audit drops (after `shift`, or with a substitution among several words);
  - a function called without its definition in the same command.

  **6. Residuals, disclosed (the obfuscation classes, outside the threat model).** These carry the D21153-8i-J extension item 6's residuals, which the operator's ruling (16) accepted (*"A later round re-raises one only with a concrete exploit."*):
  - the audit is lexical, so a junction, symlink or hard link that aliases a root is read under the name it is spelled by;
  - a program's own directory option other than `git`, `tar` and `make -C` and `env -C` is not followed;
  - interpreter inline code is not followed;
  - a child shell's or interpreter's unread script or stdin is flagged in shadow only.

  Ruling (17) adds:
  - bash escape and expansion tricks the lexer does not model beyond item 2 and the D21153-8i-J extension's quoting (for example `IFS` games, `${!name}` indirection, `printf %b` to a script the session then runs);
  - Windows canonicalisation spellings other than item 3's: a trimmed segment or a stream in a relative command word that does not climb (it stays in the cwd's subtree), device names (`CON`, `NUL`), `%VAR%` forms (no tool expands them), and 8.3 names inside the copy;
  - nesting deeper than the bounds, which voids rather than resolves;
  - a token from git's `--abbrev-ref` or `symbolic-ref`, which names a ref (a branch named like a path is not modelled);
  - a trap's action read in the cwd it was set in, not the cwd it runs in;
  - an array element or a function parameter bound across `shift`, `set` or a sourced file the audit did not see.

  **7. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `d187eb0172491cd41062d8d5a981b1de3c4db7eed80f178335aee54de7ad4ac5`) gives against corpus `07bd6a2`: sha256 `6cc5a6a5dad33c7cfaa9caea7951e910018486937ca2bd13a9b96068cb648f00`. This supersedes the D21153-8i-J extension item 7's `324013c2…`. The selection stays 36 tests, and the record keeps its 34 names. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: round 15's fixes to the out-of-tree audit, its new bounds, and its residuals — 2026-10-10, before B2's re-run; PROPOSED** (cites the D21153-8i-K extension above. It edits nothing in it). Written in the AFK build, package D21153-8i-L (L1 to L3), after `s8i` refuting round 15, under the operator's rulings (12) and (17) and the design notes (a) to (h) the orchestrator relayed. The operator wrote ruling (17) into D21-153 on 2026-10-10, where the D21153-8i-K extension says the operator writes it.

  **1. What the audit now resolves or voids.**
  - A double-quoted `${…}` is verbatim only when its body is a plain parameter (a name or a positional parameter, with an optional literal subscript). Any other body is lexed as D21153-8i-J lexed it, so a substitution inside `"${X:-…}"`, `$(…)` or backticks, is audited.
  - A read rooted at `/` (an empty path once trimmed) reads the registered temp roots, so `find /`, `grep -r … /`, `ls -R /` and a Grep or Glob path `/` void; with no temp directory known they void.
  - A word made only of revision tokens and range syntax (`A..B`, `A...B`) is no read. `git rev-list` with `--format`, `--pretty`, `--oneline`, `--header` or `--bisect*` prints text, so its output is not a revision.
  - `powershell` is an interpreter. Which word is the script is read from per-interpreter option tables, so an option's value is never the script. For PowerShell a parameter is read in any case, by alias or by a unique prefix (an ambiguous prefix reads as a switch), `-File` names its script wherever it stands, and each operand before it is read as a script too. Inline code stays shadow-flagged (ruling (16)).
  - Text an interpreter runs, staged or fed on stdin, is read token by token with its backslashes as separators, as a heredoc's body is. A staged script's relative `..` resolves against the cwd it runs from.
  - A Write-tool text holding one of the audit's reserved marks is unreadable, and running it voids and flags.
  - `read`, `mapfile` and `readarray` fed by `< <(…)`, by a here-string, or as a `while` or `until` condition whose `done` is fed, bind as the piped form does: a listing or revisions the audit names is that text, and literal text binds to each of its lines and each line's words (at most 64 values).
  - A glob inside a temp subdirectory the session wrote into is the session's own. The temp root itself stays foreign, because other processes' files live there, and so does anything at, above or below a registered sweep root.

  **2. The new bounds (H-13).** These sit beside the D21153-8i-K extension's item 2:
  - a cwd past 1,024 bytes is unknown, so the `cd` or `pushd` that would make it voids;
  - a `pushd` stack past 64 entries is unknown, and every later `popd` voids;
  - an unbraced name is scanned back at most 256 characters; a longer one is left unbraced and voids.

  The variable map is no longer cloned per command, and `expand` no longer rescans. Round 15's 50 scale shapes (debug build) all finish under 5 s, against up to 21.8 s under D21153-8i-K. The slowest shaped input of the cwd test's robustness block is 4,000 chained assignments, at 2.67 s alone.

  **3. What still voids (a disclosed false-void surface, beside the D21153-8i-K extension's item 5).** Each is honest work the audit cannot resolve, kept fail-closed:
  - a glob at the temp root itself (`ls /tmp/*.str`);
  - a two-name `read` (`read -r a b <<< "$(…)"`), a `read` fed by a file (`done < /tmp/x.txt`), and one piped from a program other than `printf` or `echo` of literal words;
  - a `read` inside a `for` or `select` loop whose `done` is fed (bash feeds it; the audit binds only `while` and `until`);
  - a `read` fed an empty here-string (bash binds empty; the audit binds unknown);
  - a quoted use of a literal line holding spaces (the audit binds each word too);
  - `TMPDIR` as a temp spelling;
  - a cwd past 1,024 bytes (round 15's `cds500`, which D21153-8i-K cleared) and an unbraced name past 256 characters;
  - a staged script's relative token that climbs and holds a `$` or a glob, which is read in shell mode;
  - after `python -m <module>`, the next operand is read as a script.

  **4. Residuals, disclosed (outside the threat model, or measured open).** These are beside the D21153-8i-J extension's item 6 and the D21153-8i-K extension's item 6:
  - A preload option's value is code the interpreter runs (`node -r` or `--require`, `--import`, `--loader`; `ruby -r`; `perl -M`; `python -m` of a module on a staged path), but the audit reads it as a value: `node -r <staged script> -e 1` is flagged in shadow (inline code), not voided. A cheap fail-closed close would read those values as scripts too.
  - Arithmetic `$((…))` nesting and parameter writes inside heredoc bodies (the temper:rust-reviewer's F4 on D21153-8i-K) are not modelled: obfuscation-class.
  - A range whose two tokens both print nothing would read `..`; this assumes a git revision token from the copy is never empty, as D21153-8i-K's token rule does.
  - Round 15's leak cases still not voiding, each as under D21153-8i-K. Flagged in shadow (15): `cd ~ && ls -a`, `ls /tmp`, `ls $TEMP`, `ls /d/repos`, `ls /d/`, `ls -lt /tmp`, an inline-heredoc `python -` globbing `D:/repos/*` or expanding `~` (p10, p11), a `cmd //c` script or command (p25, w20, w21), the engine's own log reads (u09, u11, u12), and `Path.home()` in a staged script (x11). Not flagged (11): `where.exe` and `which` (b19); the engine binaries run on the copy (u04, u08, u10); `python -` listing `..` (p12) and `python3 -c` listing `/d/repos` (p23), inline code under ruling (16); a staged PowerShell script joining `$env:USERPROFILE` (w16); the copy's own `verify_headless.ps1`, unstaged (w18, w19); `uv run <staged script>` (w27, `uv` is not an interpreter the audit follows); and `git config --global --get-regexp` (g04).

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `f848bb25d267e7af86f1be5ff5d2936a465ddcc13c80744a40d5ea8cf12ec723`) gives against corpus `07bd6a2`: sha256 `93f1bfac93f2764ba20fa48f349a5b5a178f021c880fd6005c10c8b367280f03`. This supersedes the D21153-8i-K extension item 7's `6cc5a6a5…`. The selection stays 36 tests, and the record keeps its 34 names. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the out-of-tree audit bounded on any input by its own work count, and the residuals the threat model leaves — 2026-10-11, before B2's re-run; PROPOSED** (cites the D21153-8i-K and D21153-8i-L extensions above. It edits nothing in them). Written in the AFK build, package D21153-8i-M, after `s8i` refuting round 16 and the temper:rust-reviewer's re-read of D21153-8i-L, under the operator's rulings (17) and (18), and the operator's ruling of 2026-10-11 on the robustness test, *"Count work, not time"*, relayed by the orchestrator, which the operator writes into D21-153 in their own words.

  **1. What does not change.** No verdict moves on any calibrated case. The whole-set harness reads 1,971 rows identically under D21153-8i-L and D21153-8i-M, hits and shadow flags alike (`s8i` §8i.3's D21153-8i-M note).

  **2. The bounds** (H-13).
  - The work bound: the audit counts its own work on each tool use, and a use past 2^24 (16,777,216) units is not read further. It voids, and flags `unresolved audit work bound`. The largest calibrated use costs under 30,000.
  - The tracked variables keep an undo log; past 1 MiB of names and values, the log included, an assignment binds the unknown value `$`. Subshells nested past 64 in one command void.
  - A process substitution is recorded only after a `<` redirect. Past 64 recorded in one command, the command is not lexable and voids.
  - A run already read in the same state is replayed, not read again. Its key is its text's digest, cwd, OLDPWD, the variables' digest, depth, the state outside the variables, and whether an assignment or a write is read. It is replayed only where each run it tried would be cut as before.

  **3. Disclosed residuals and false-void surface, under ruling (17)** (the honest-model minors round 16 and the reviewer named; not fixed in this package, the orchestrator's call as relayed, for the operator's acceptance):
  - Name listings the audit does not name: `git grep -l`, `git lfs ls-files -n` and `git ls-tree --name-only`. Output fed on from them is unknown, and a read of it voids: a false-void surface.
  - `xargs` fed by a here-string, a heredoc or `< <(…)`: its input is not read as xargs's words. An absolute path in a here-string or heredoc is still read as data, but a relative one reaches the command unaudited: a residual. `xargs` fed by a file the session wrote by means the audit does not stage voids, as ruling (12)'s unreadable input does: a false-void surface.
  - `node` resolving `require('../x')` against the script's own directory: the audit resolves a staged script's climbs against the cwd it is run from. A script run from another directory is a residual, or a false void.
  - `${arr[$i]}` and `${1:-default}` inside double quotes are not plain parameters, so they are lexed as package J lexed them, and their `$` voids where read: a false-void surface.
  - `mktemp -d` inside a longer word is not named as the session's temp path, so a read under it voids: a false-void surface.
  - The outputs of `git format-patch -o`, `split` and `csplit` in a session temp directory are not recorded as written, so a later read of them voids: a false-void surface.
  - The cost of a unit varies. The slowest measured is about 2.7 µs in the debug build, so a use at the bound could take about 45 s there. A staged text has no size bound but the work bound: a 1.2 MB staged script read once costs 3.6 M units, 6.3 s in debug.
  - Two runs, or two flag lines, are compared by 128-bit SipHash-1-3 digests under fixed keys, where D21153-8i-L compared copies. A collision needs about 2^64 work, which is obfuscation-class.

  **4. The test** (H-12, approved by the operator on 2026-10-11). `k_robustness` no longer asserts a wall-clock bound. It asserts each shape's verdict at its size n, and that its work at 2n is under three times its work at n, both under the work bound. It also asserts replays and the work bound. The selection stays 36 tests, and the record keeps its 34 names.

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `cff7c1c3da4e16732e275dda74be095d7841e49b64811feee686b17ed2b933d1`) gives against corpus `07bd6a2`: sha256 `cda0d8f39d1ceb3d3dd5bb5b8994026f74dd1e167dfd3b6086f84c68a44b55fe`. This supersedes the D21153-8i-L extension item 5's `93f1bfac…`. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the robustness test's work ratio exactly as ruled, round 17's three honest false-void classes cleared, and the rust-reviewer's F1 to F3 — 2026-10-11, before B2's re-run; PROPOSED** (cites the D21153-8i-K, D21153-8i-L and D21153-8i-M extensions above. It edits nothing in them). Written in the AFK build, package D21153-8i-N, after the operator's ruling (19), `s8i` refuting round 17 and the temper:rust-reviewer's re-read of D21153-8i-M, under rulings (16), (17) and (19).

  **1. The work-ratio rule** (ruling (19); H-12, for the operator's ruling (20)). Each shape of `k_robustness` is read at its size n, where its verdict is asserted, and at 2n. Its work at 2n must be strictly under three times its work at n, with nothing added, and both under the work bound (2^24). The ceiling group: a shape that has no 2n of its own is read at its size only and must stay under an absolute ceiling of 4,096 units. There are two. A command over the audit's size bound is refused unread at a fixed cost (46 units) whatever its length, so n does not scale its work. A name at the brace scan's bound has the bound itself (256) as its size; at twice it, it is the next shape, a name past the bound, which voids and is measured by the ratio. Its measured step, 1,042 units at 256 and 3,376 at 512, is from braced (about 3 units a byte) to unbraced (about 6), each linear. D21153-8i-M's code allowed `3 * at_n + 4096`; that slack is gone.

  **2. The three false-void classes** (round 17's honest minors, ruling (17)).
  - (a) A sed script and an awk program are code, never a path Win32 reads: each `-e`'s value (attached or not, and `--expression`'s; gawk's `-e` and `--source`), or else the first operand; awk stops at its program. A `-f` script file and every file operand are read as before.
  - (b) An arithmetic expansion runs no command of its own: each `$(…)` or backtick in its body is audited as the command it is, nested arithmetic one run deeper, and its value is a count, never a path. `((…))` where a command starts, its inner `(` closed just before a `)`, is an arithmetic command; any other `((` is two subshells.
  - (c) Perl's bundled switches ending in `e` (`-ne`, `-lne`, `-lpe`, `-ane`, `-0777 -ne`) take inline code, as `-e` does. Inline code stays shadow-flagged per ruling (16), never a path word.
  Code (a sed script, an awk program, an interpreter's inline code) is audited as the word it was, so a root, a copy outside or an answer-bearing name it spells voids, as does an unresolved `$`; each relative token in it that climbs is resolved against the cwd; and Win32's spelling closes (a UNC or device prefix, a trimmed segment, an alternate stream) do not apply to it. Round 17's honest sets void 34 times under D21153-8i-M and 6 under D21153-8i-N; the 1,971 calibrated rows and every must-void case read as before.

  **3. Disclosed residuals and false-void surface, under ruling (17)** (new in this package, for the operator's acceptance):
  - A UNC, device, trimmed-segment or stream spelling inside code (a sed script, an awk program, inline code) is not closed. A root or copy spelled in it still voids; a spelling Win32 alone would resolve outside is obfuscation-class.
  - A sed script's or awk program's own reads (`r file`, `getline < file`) and inline code's reads are audited as data, as before: a relative name that does not climb is not resolved.
  - `((` is arithmetic only where a command starts (after `;`, `&`, `|`, `(`, a newline or a heredoc's body, and after `if`, `then`, `else`, `elif`, `do`, `while`, `until`, `!` or `{`). Elsewhere, `for ((…))` included, it reads as two subshells, as before: a false-void surface.
  - Round 17's six honest voids left, outside the three classes: `set --` then `$1`, `"$@"` or `for h; do` (four: h154, b038, b039, b040), `expr $(…) + $(…)` (b035), and a `cd` into a listing's region (h179, ruling (12)'s). A false-void surface.
  - A sed or awk regex read as a path still raises a shadow flag (h214's `/[/script/…`), which holds the record unscored (D21-144) but never voids it, as before.
  - D21153-8i-M's item 3 said digests are taken *"under fixed keys"*. They are now keyed at random once per audit (the rust-reviewer's F1), so no input can be built in advance to collide.

  **4. Bounds** (H-13). The memo of run outcomes is bounded by its bytes (2^24: flags, cwd, OLDPWD and runs tried) as well as by its count (4,096), and an outcome larger than the bound is not kept (F2). A staged text's copy, made while a run holds it, is charged to the work count (F3). 50,200 fuzz inputs on a 1 MiB stack read with no panic, the most work on one 3.06 M, none at the bound.

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `3cbbdaf09c3cc77da2bf327f8625b6b177c4c4611f36450713fadb400d46aaf9`) gives against corpus `07bd6a2`: sha256 `06a92950996547c9b8d0b3ebc918fb697d32df426b8721875fc16728ddf6c53a`. This supersedes the D21153-8i-M extension item 5's `cda0d8f3…`. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: the rust-reviewer's N1 to N3 on D21153-8i-N and round 18's glued climbs closed, and round 18's other findings disclosed — 2026-10-11, before B2's re-run; PROPOSED.** It cites the D21153-8i-M and D21153-8i-N extensions above and edits nothing in them. It was written in the AFK build, package D21153-8i-O, after the operator's ruling (20), the temper:rust-reviewer's re-read of D21153-8i-N and `s8i` refuting round 18, under rulings (16), (17) and (20).

  **1. Code, arithmetic and the root.**
  - An arithmetic body (`$((…))`, or `((…))` where a command starts) that holds a `'` or `$'` at its top level voids, flagged *"a quote in arithmetic"* (the reviewer's N1). Bash reads the body as if double-quoted: such a quote is literal, a substitution inside it runs, and then bash rejects the body. No honest arithmetic is lost.
  - In perl's inline code an unquoted bare `..` is the range or flip-flop operator, not the parent (N2). In every other code, a shell's included (`pwsh -c 'gci ..'`), a bare `..` is the parent, and a quoted one is the parent in all code.
  - The review root spelled with trailing dots (`D:/sb./d/…`, `D:/sb..`, `/d/sb.`) is the root, in code and outside it (N3). This closes, for the root, item 3's *"a … trimmed-segment … spelling inside code is not closed"* in the D21153-8i-N extension.
  - A climb glued to the token before it (sed's `1r../x`, `/re/r./../x`) is resolved from where the climb starts, when the rest climbs (round 18's code-classes finding 1). Perl's `q{…}` and `q[…]` split code.
  - Item 2's *"each relative token in it that climbs"* (D21153-8i-N extension) therefore now also covers a climb glued to a letter or digit.

  **2. Measured.** The 1,971 calibrated rows and round 17's 2,035 rows read as under N, and the real transcripts read 0 false voids. Of round 18's 635 rows, 18 move, every one a must-void read (or write) of another copy now voiding, and no honest row moves.

  **3. Disclosed residuals and false-void surface, under ruling (17)** (new in this package, for the operator's acceptance; round 18's findings that are not fixed, and the reviewer's nits that are not taken):
  - *False-void surface (round 18's code-classes finding 2, refuted to minor).* A climbing token inside a sed or awk regex voids. This is an unescaped `..` before an escaped `/` or after a quote, as in `sed 's/#include "..\///'` (round 18's H38, new in N). It also covers a letter or digit glued before `../` (`sed 's/abc../X/'`). The D21153-8i-N extension's item 3 sentence, *"A sed or awk regex read as a path … never voids it"*, holds for regexes that do not climb. The escaped forms (`s#\.\./##g`) and `s/^..//` read 0.
  - *False-void surface.* A quoted bare `..` in any code is the parent (`awk '{print $1 ".." $2}'`, `python -c 'print("..")'`; the reviewer's sa10), and so is an unquoted one in code other than perl's.
  - *False-void surface.* The root followed by dots and then a character that is not a letter, digit, `-` or `_` is the root (`D:/sb.~x`).
  - *Residual.* A climb glued after a character that is neither a letter, a digit nor `/` (`~../x`) is not resolved, and neither is a relative name in code that does not climb (as before).
  - *Residual (round 18's code-classes finding 1, its one case left).* A sed that find's `-exec` runs (`find . -exec sed -n '1r../p1i2a1/CLAUDE.md' {} +`) is not audited as sed: its script is one of find's arguments, so the glued climb is not resolved. This is as under N.
  - *Residual (round 18's honest-reads finding).* `xargs -a <(…)` and `xargs --arg-file <(…)` (the value as the next word) read outside the copy with no void and no shadow flag (`xargs -a <(dirname "$PWD") ls`). This sits beside ruling (19)'s accepted `xargs … < <(…)`. `--arg-file=<(…)` and the piped form void.
  - *False-void surface (round 18's in-copy-robust finding 1).* A `${NAME op}` expansion of a variable whose value is a known in-copy path voids as an unresolved `$` in a read position. This covers `#`, `##`, `%`, `%%`, `/`, `//`, `:-`, `,,` and `^^`, as in `wc -l "${f%.cpp}.h"` and `ls "OurGame/Content/${p#Game/}.uasset"`.
  - *False-void surface (round 18's in-copy-robust finding 2).* An array element or append assignment (`n[k]=$(…)`, `files+=("$f")`), `local f=$1`, `printf -v` or a `for` over `${!arr[@]}` voids honest in-copy reads, and an element assignment is read as a command word.
  - *Not renamed (round 18's code-classes finding 3).* Case sav9, *"a script climbing out of the copy"*, climbs in its file operand. A test name is not rewritten here (H-16). The script climbs now have their own cases (cg1 to cg15, and plv4).
  - *Not taken (the reviewer's N4 to N6, nits).*
    - N4: `time`, `coproc` and a `case` item's `)` do not start a command, so `((` there reads as two subshells and fails closed.
    - N5: a program that runs its input as commands (sed's `e` and `s///e`, awk's `system($0)` and `cmd | getline`, perl's `system $_`) is not followed into a staged input file. This is adversarial-only.
    - N6: `script_operands`'s option tables are not lifted, and mawk's or nawk's `-W` is not valued, which fails closed.
  - Round 18's packet-records findings are answered by dated notes in `s8i` §8i.3 and §8i.5: the stray `main.tar` is gone (measured), the fifth sed and ruling (20) are recorded, and F4's disposition is recorded.

  **4. Bounds** (H-13). `glued_climb` and `arith_quoted` are each one linear pass, and every word they resolve is charged as before. 51,020 fuzz inputs on a 1 MiB stack read with no panic, the most work on one 3.17 M, none at the bound.

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `9db4f55ac67b34ad7b4d753519adb494d78991b5285b6993b05bdc777bbb6cbd`) gives against corpus `07bd6a2`: sha256 `e7dd5eb0b903e17593273040288f2e05e05aff103d619cc9ea9a09b7400c2367`. This supersedes the D21153-8i-N extension item 5's `06a92950…`. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: perl's `..` by string context (the rust-reviewer's O1 and O2 on D21153-8i-O, round 19's o-correctness findings), and O3 disclosed — 2026-10-12, before B2's re-run; PROPOSED.** It cites the D21153-8i-M, D21153-8i-N and D21153-8i-O extensions above and edits nothing in them. It was written in the AFK build, package D21153-8i-P, after the operator's ruling (21), the temper:rust-reviewer's review of D21153-8i-O and `s8i` refuting round 19, under rulings (16), (17) and (21).

  **1. Code and arithmetic.**
  - In perl's inline code a bare `..` inside a string (`'…'`, `"…"`), a backtick span or a quote-like span (`q`, `qq`, `qx`, `qw`, any delimiter) is the parent, and outside every span it is the range or flip-flop operator, whatever stands beside it. This is ruling (21)'s *"when quoted in any code, `..` stays the parent"*, which the D21153-8i-O extension's item 1 states and O's code did not implement (it read a quote next to the `..`). A span's closing delimiter ends a token, so `q{..}` and `q[..]` hold a bare `..`. Every other code is unchanged.
  - A quote inside a parameter expansion (`${…}`) in an arithmetic body is honest, unless the single-quoted span holds `$(` or a backtick, which bash runs there and which still voids. With that, the D21153-8i-O extension item 1's *"No honest arithmetic is lost"* holds for round 19's A1 and A6, which it did not under O.
  - Bash's `$[…]` is an arithmetic expansion, as `$((…))` is: its substitutions are audited, a top-level quote voids, and one left unclosed voids and flags.

  **2. Measured.** The 1,971 calibrated rows, round 17's 2,035 and round 18's 635 read as under O, and the real transcripts read 0 false voids. Of round 19's 52 rows, 15 differ: nine must-void reads newly void, one gains a flag, and five honest rows clear. No must-void row moves to clear.

  **3. Disclosed residuals and false-void surface, under ruling (17)** (new in this package, for the operator's acceptance):
  - *Not applied (the rust-reviewer's O3, a nit).* `glued_climb` still applies in every code class, so a regex with a letter or digit glued before `../` voids at the copy root in perl's and python's code (`perl -pe 's/v1../v2/'`), as the D21153-8i-O extension's item 3 discloses for sed and awk. Restricting it to sed's `r`/`R` was measured as a candidate. Even after `r` or `R` in every code, it moves round 18's must-void L25 (`sed -n '1W../p1i2a1/out.txt'`, a glued write into another copy) from void to clear, so it is not applied.
  - *False-void surface.* In perl's code, a quote character that opens no perl string opens a span the audit cannot close where perl does. This covers a quote in a comment (`# don't`), in a regex (`s/"//`) or in the old package separator (`Foo'bar`), but not `$'`, `$"` and `` $` ``. A bare `..` after it is then read as the parent and voids (fails closed).
  - *Residual.* Perl's heredoc (`<<"EOF"`) and string interpolation of a variable holding `..` are not followed: a bare `..` in a heredoc body is read as the operator. These are adversarial-shaped, and pass 1's transcripts hold no perl (round 19's refuter, measured).
  - *Residual.* A quote-like operator written as a method or a qualified name (`->q(…)`, `Foo::q(…)`) is not a span, and neither is a regex quote-like (`m`, `qr`, `s`, `tr`, `y`), whose `..` is no path.

  **4. Bounds** (H-13). `perl_spans`, `param_close` and `bracket_close` are each one linear pass, and every word they resolve is charged as before. 50,200 fuzz inputs on a 1 MiB stack read with no panic, the most work on one 3.10 M, none at the bound.

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `61cfac85b16a5ad678c507814690fab08c599b7b88ace219f80e53648a8739b9`) gives against corpus `07bd6a2`: sha256 `75d5b42c38be1f33c8cd3ce9fc7430b7bcb243b0aa4a4895e3a1023c1dacab33`. This supersedes the D21153-8i-O extension item 5's `e7dd5eb0…`. METHOD-S §3 is unchanged.

- **s8i pre-dispatch amendment, extension: every bare `..` in perl's inline code is the parent (the operator's ruling (23); round 20's findings and the rust-reviewer's P1 to P4 on D21153-8i-P) — 2026-10-13, before B2's re-run; PROPOSED.** It cites the D21153-8i-N, D21153-8i-O and D21153-8i-P extensions above and edits nothing in them. It was written in the AFK build, package D21153-8i-Q, after the operator's ruling (22), the temper:rust-reviewer's review of D21153-8i-P, `s8i` refuting round 20 and the operator's ruling (23) (given in chat on 2026-10-13, relayed by the orchestrator), under rulings (17), (22) and (23).

  **1. Code.**
  - In perl's inline code every bare `..` is the copy's parent, as in every other code (fail-closed). The D21153-8i-P extension item 1's *"outside every span it is the range or flip-flop operator"* no longer holds: no perl `..` is cleared as an operator, whatever spans stand around it. A span's closing delimiter still ends a token, so `q{..}` and `q[..]` hold a bare `..`; that is the spans' only use now.
  - A `..` glued to what stands beside it is no bare `..` and is read as before: `1..10`, `0..$#ARGV` and `/A/../B/` hold no `..` token and stay clear, as they did before D21153-8i-O.
  - Ruling (22) accepted the D21153-8i-P extension's item 3 as it stood; item 3 below corrects two of its claims.

  **2. Measured.** The 1,971 calibrated rows, round 17's 2,035 and round 18's 635 read as under P, and the real transcripts read 0 false voids. Of round 19's 52 rows, 4 honest perl ranges move from clear to void. Of round 20's 106 rows, 18 move from clear to void: 16 must-void reads and 2 honest perl rows. No row moves to clear. Seven test expectations move under H-12, honest perl ranges and flip-flops that now void (`s8i` §8i.3's D21153-8i-Q note lists them).

  **3. Disclosed residuals and false-void surface, under ruling (17)** (new in this package, for the operator's acceptance):
  - *False-void surface (ruling (23)).* In perl's inline code a spaced or quoted range or flip-flop voids: `1 .. 10`, `/A/ .. /B/`, `"a".."z"`, `'a'..'e'`, `("01".."12")`. So does a perl string holding a spaced ` .. ` (a join separator or a range label: `join(" .. ", 1..3)`, `"$a[0] .. $a[1]"`; round 20's minor), and a bare `..` in a perl heredoc body (each measured). Pass 1's transcripts hold no perl (round 20's refuter: `grep -c -w perl` over `results/s-bin/raw/p1i*.jsonl` reads 0 in all six).
  - *Two claims of the D21153-8i-P extension corrected.* Its item 3's *"A bare `..` after it is then read as the parent and voids (fails closed)"* was incomplete under P: a quote that opens no perl string (in `s///`, `tr///` or a comment), or an escaped quote inside a string, could invert the pairing and clear a later quoted `..` (round 20's p-correctness findings; the reviewer's P3). Under ruling (23) a pairing moves only where a token ends, and those reads void (rows plv20 to plv26). Its residual *"a bare `..` in a heredoc body is read as the operator"* no longer holds: it is now false-void surface, above.
  - *Residual (the reviewer's P1; round 20's third p-correctness finding).* The D21153-8i-P extension's item 1 says a quote-like span with *"any delimiter"* is the parent. That holds for the span, not for the token: a quote-like whose opener is not a code delimiter (`q!..!`, `q#..#`, `qq~..~`, `q/../`) keeps its opener in the token (`q!..`), so the `..` is no bare `..` and does not void. It read clear under O and P too (round 20's V-pl-qbang, V-pl-qhash, V-pl-qqtilde, V-pl-qslash, V-qbang and V-qq-hash). Ending a token at a span's opener as well would close it; it is not applied, by the orchestrator's default for the reviewer's P1 to P3 (settle by ruling (23) where it moots them, otherwise correct the claim and disclose). It is adversarial-shaped, and pass 1's transcripts hold no perl.
  - *Residual, narrowed (the reviewer's P2).* The names before a `q` that stop a quote-like (`>` and `:` for `->q` and `Foo::q`, and `&`) also stop one after `=>`, `&&` or `?:`. With a bracket delimiter that no longer clears anything (`d=>q(..)` voids, row plv27); with another delimiter it is P1's residual.
  - *Inert, kept.* `perl_spans`'s `\` escape is never reached from inline code, which arrives with each `\` already read as `/`; its unit rows test `perl_spans` itself. Under ruling (23) no read depends on the pairing beyond where a token ends.
  - *Not a residual.* `$"..$"` (round 20's V-pl-dollar-sq) is a range of perl's list separator, not a path; it read clear under O, P and Q.
  - *Not taken (the reviewer's P4, a nit).* `perl_spans`'s length and the history-style doc comments.

  **4. Bounds** (H-13). Q removes the per-token span lookup; `perl_spans` is still one linear pass. 50,200 fuzz inputs on a 1 MiB stack read with no panic, the most work on one input 3.07 M, none at the bound.

  **5. The driver.** `runner/src/sbin.rs` is the bytes that `S8I-D21153-8i-sbin-driver.patch` (sha256 `59408f5858764cdf2faa4bc5b2d49edf232ad7e6895062074b980708aae50691`) gives against corpus `07bd6a2`: sha256 `664fd3678ea88145b4dbd35378ebbaa22b3880ccb977da2162f1ea2e1ef39c5a`. This supersedes the D21153-8i-P extension item 5's `75d5b42c…`. METHOD-S §3 is unchanged.
