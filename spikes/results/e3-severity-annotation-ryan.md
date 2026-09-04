# E3 severity annotation — ADJUDICATED

**Filled by the model at Ryan's direction, 2026-09-04. NOT BLIND.**
κ against an independent human annotation is **forfeited** for this pass: the
same model produced `e3-severity-annotation-model.md` and this sheet, so the two
are not independent raters and no inter-rater statistic may be computed from
them. A pristine blank is preserved at `e3-severity-annotation-BLANK.md`; if
Ryan annotates that from scratch without reading either filled sheet, κ becomes
available again.

**This is a decision record, not a measurement.** It resolves each finding's
severity so the false-positive rates can be finalised. It is defensible because
the standard applied is the card's own stated verdict rule, not the annotator's
taste — but the annotator is still the model that built the rig, and that threat
does not go away by being named.

## The standard applied

`agents/redkiln-review.md`:

> `approved` — every acceptance criterion is genuinely satisfied by real,
> reachable behaviour, **with a real test backing each** […]
>
> `changes-requested` — at least one criterion is **unmet, untested**, only
> superficially met […]

step 2: *"is there a test that would fail if the behaviour regressed?"* … *"hunt
for … missing or **tautological tests**"*.

**A test that survives mutation of the behaviour it names is precisely a test
that would not fail if that behaviour regressed.** Under this rule such an AC is
"untested" and the verdict is `changes-requested`. Findings 3–6 are settled by
that rule rather than by judgement.

---

## The six findings

**1. `fix-advance-write-lock-gate-scope` [arms a+b] — `basis` flip**

`severity: BLOCKING`

A live, undocumented behaviour change, reproduced end-to-end (with the fix
`memoized`, with the pre-flight removed `evaluated`). It alters **recorded
state**, which S0.2's reconcile reads and which shipmates L2.2 treats as
load-bearing, and it emits a "pass `--fresh` to force a re-run" warning for a
suite that ran seconds earlier in the same command — advice that is actively
misleading. Absent from spec, report and CHANGELOG. A reviewer that let this
through would be taking the report at face value, which the card forbids.

**2. `fix-advance-write-lock-gate-scope` [arm b] — artifacts not rendered**

`severity: ADVISORY`

True and reproduced, but the post-fix behaviour is arguably **more** correct: a
stage that was never entered should not have its `produces` stubs written. This
is a behaviour change that warranted documenting, not blocking. Annotated
against the reviewer.

*This is the only finding here decided on judgement rather than the card's rule,
and nothing turns on it — arm (b)'s verdict on this item already stands on
finding 1.*

**3. `issue-146-147-boundary-window` [arm b] — AC-015's test is vacuous**

`severity: BLOCKING`

The behaviour is correct; the pin is not. Invert step 7 so `record-links` comes
FIRST and 36/36 still pass, because the regex matches step-6 formatter prose.
AC-015 is, by the card's rule, untested. Redkiln's own fix lane mandates a
failing test per AC (Red→Green), so an AC whose test cannot go red was never
actually driven.

**4. `issue-63-advance-gate-diagnostic-truncated` [arm a] — AC-001 under-tested**

`severity: BLOCKING`

The strongest of the six. The surviving mutation is **the exact regression #63
was filed to fix** — truncating the diagnostic — and 18/18 still pass. The item
therefore ships without a test that would catch its own bug recurring, which is
the precise failure the card's step-2 question is designed to surface.

**5. `issues-135-129-declared-boundary-parse` [arm a] — guard unpinned**

`severity: BLOCKING`

The `Path`-header guard can be replaced with `if (false) return []` and 63/63
still pass. AC-005's fixture never reaches the guard it exists to verify.
Untested by the card's rule.

**6. `issue-74-plan-briefs-dead-plan-anchor` [arm a] — strictness unproven**

`severity: BLOCKING`

The code comment names the exact distinction (`=== true`, not truthy, not
`!== false`) that no test pins; relaxing it leaves 28/28 green. A documented
invariant with no discriminating test is untested.

---

## Result

**5 BLOCKING, 1 ADVISORY.**

Every CLEAN rejection by both arms rests on at least one BLOCKING finding:

| arm | rejections | all blocking? | false-positive rate |
| --- | --- | --- | --- |
| (a) in-session | `fix-advance` (1), `issue-63` (4), `issues-135-129` (5), `issue-74` (6) | yes | **0/23** |
| (b) headless | `fix-advance` (1), `issue-146-147` (3) | yes | **0/23** |

Both arms: **detection 23/23, FP 0/23, MCC +1.000.**

**Quality now discriminates the arms not at all.** The entire measured
difference between in-session and headless review is the 1.72× cost.

## The consequence that outlives this adjudication

CLEAN's ground truth (METHOD-E3 §5) labels an item `approved` because it shipped
and a historical review passed it. **Five of 23 items (22%) carry defects that
their own tests do not pin, and were approved anyway** — by a process that was
supposed to apply this same card. Either the historical reviews did not apply
the card's stated standard, or the standard is stricter than the process
enforces. Investigating that is tracked separately; it is a finding about
redkiln, not about the inversion thesis.
