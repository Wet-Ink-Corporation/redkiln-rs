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

`severity: ADVISORY (corrected 2026-09-04 — see below)`

**Originally annotated BLOCKING on a claim that was wrong, and corrected here.**
The first annotation said the surviving mutation was "the exact regression #63
was filed to fix". It is not. The reviewer's mutation truncates `detail` to the
first TWO lines (`slice(0, 2)`); the actual #63 bug truncated to ONE
(`split("\n")[0]`). Verified: restore the real one-line bug and the suite goes
**RED** — the test does catch the regression it was written for.

What survives is a *near-miss variant*, so AC-001's regression IS pinned and only
the AC's stated generality ("2+ unticked … names every unticked item") is not,
because the fixture leaves one item unticked. That is "narrowed", which is
exactly what the historical reviewer wrote before docking ac-coverage to 2/3 —
and that judgement now looks defensible rather than negligent.

This is the one finding where I think the reviewer over-blocked, and the
correction is recorded rather than quietly folded in because it moves a rate:
arm (a) goes to **FP 1/23**, not 0/23.

*Why the other three test-vacuity findings are not affected: they target
behaviour the fix NEWLY introduced — a `Path`-header guard, a `=== true`
strictness check, an ordering instruction — so the mutation hits the AC's own
claim directly and there is no prior bug it might be pinning instead.*

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

**4 BLOCKING, 2 ADVISORY** (after the finding-4 correction).

| arm | rejections | blocking basis? | false-positive rate |
| --- | --- | --- | --- |
| (a) in-session | `fix-advance` (1) ✓, `issues-135-129` (5) ✓, `issue-74` (6) ✓, **`issue-63` (4) ✗** | 3 of 4 | **1/23** |
| (b) headless | `fix-advance` (1) ✓, `issue-146-147` (3) ✓ | 2 of 2 | **0/23** |

| | arm (a) in-session | arm (b) headless |
| --- | --- | --- |
| detection | 23/23 | 23/23 |
| false positives | **1/23** | **0/23** |
| MCC | +0.978 *[corrected 2026-09-11: +0.957 — see the dated correction at the foot]* | **+1.000** |

The direction is unchanged from the unadjudicated result — headless is not
worse — and the gap is still one item, which McNemar cannot separate from
chance. **Quality remains unable to discriminate the two arms.** The entire
reliable difference is the 1.72× cost.

## The consequence that outlives this adjudication

CLEAN's ground truth (METHOD-E3 §5) labels an item `approved` because it shipped
and a historical review passed it. **Five of 23 items (22%) carry defects that
their own tests do not pin, and were approved anyway** — by a process that was
supposed to apply this same card. Either the historical reviews did not apply
the card's stated standard, or the standard is stricter than the process
enforces. Investigating that is tracked separately; it is a finding about
redkiln, not about the inversion thesis.

## Correction (2026-09-11) — arm (a)'s MCC was a computation error

**Audit PIA-073; ERRATA E-0074-3, E-0080-1, E-0001-1.** The table above gives arm (a) **MCC +0.978**. From the
matrix the same table states — detection 23/23, false positives 1/23 — the confusion matrix is **tp 23 / fn 0 /
fp 1 / tn 22**, and MCC = (23·22 − 1·0) / √((23+1)(23+0)(22+1)(22+0)) = **+0.957** [measured, recomputed
2026-09-11 from the matrix above; F21 computes its own +0.917 the same way]. The earlier figure was a computation error, not a
different matrix; arm (b)'s +1.000 (tp 23 / fn 0 / fp 0 / tn 23) is unchanged. The one-item gap, its direction
(headless not worse) and "McNemar cannot separate it from chance" all stand.

**The McNemar p = 0.625 is bound to the right quantity.** Where restatements say "detection 23/23 vs 23/23,
McNemar p = 0.625", the test is not on detection — detection has **no discordant pairs and admits no test**. The
p = 0.625 is the exact two-sided McNemar on the **CLEAN false-positive discordant pairs as recorded before this
adjudication: 3 items (a) rejected / (b) approved against 1 the other way** (F23). After this adjudication the
recorded 4/23 vs 2/23 settles to 1/23 vs 0/23 — one discordant pair, still nothing a test can separate.

This is a decision record; the correction is dated and the original line is left in place with a marker.
**countersign: Ryan Britton, 2026-09-11** — the correction was written by Claude on 2026-09-11 and countersigned the same day (via AskUserQuestion; recorded at Ryan's direction).
