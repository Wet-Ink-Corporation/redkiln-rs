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


Any later change is disclosed here with the date and the records it post-dates.
