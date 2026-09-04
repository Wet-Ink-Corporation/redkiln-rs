# E3 severity annotation — Ryan's sheet (κ, METHOD-E3 §7)

**Annotate this BEFORE reading `e3-severity-annotation-model.md`.** κ is only
meaningful if your annotation is blind to the model's.

## What is already settled — you do not need to re-check these

All six claims were replicated mechanically (F24). Every one is **factually
true**; none is a hallucination. You are annotating **severity only**: should
this finding have blocked the item?

`BLOCKING` = the reviewer was right to return `changes-requested` on this.
`ADVISORY` = true, but should not have blocked.

## The standard, quoted so you are annotating against the same one

`agents/redkiln-review.md`, the card the reviewer was given:

> `approved` — every acceptance criterion is genuinely satisfied by real,
> reachable behaviour, **with a real test backing each** […]
>
> `changes-requested` — at least one criterion is **unmet, untested**, only
> superficially met […]

step 2: *"is there a test that would fail if the behaviour regressed?"* … *"hunt
for … missing or **tautological tests**"*.

You may of course annotate against your own standard instead — if you do, say
so, because a κ disagreement then measures the standard rather than the judgement.

---

## The six findings

**1. `fix-advance-write-lock-gate-scope` [arms a+b] — `basis` flip**
Post-fix, every real `--apply` through a memoized verify gate records
`gate_result.basis: "memoized"` where pre-fix it recorded `"evaluated"`, and
prints the loud "reusing the verified result — pass `--fresh`" warning for a
suite that ran seconds earlier in the same command.
Reproduced end-to-end: with the fix `memoized`, with the pre-flight line removed
`evaluated`. Not mentioned in the spec, the report, or CHANGELOG.

`severity: ______`

**2. `fix-advance-write-lock-gate-scope` [arm b] — artifacts not rendered**
Post-fix, when an ENTRY gate refuses, the target stage's `produces` artifacts
are no longer written. Reproduced: pre-fix the item dir gains
`implementation-report.md` after the throw; post-fix it does not.

`severity: ______`

**3. `issue-146-147-boundary-window` [arm b] — AC-015's test is vacuous**
`test/implement-contract.test.ts:449` asserts `/after[^.]{0,40}commit/` against
the COMMIT step. That regex is satisfied by step-6 prose
("…LAST (after any lint --fix); commit WITHOUT --no-verify"), independent of
step 7's ordering claim. Verified: invert step 7 to say `record-links` comes
FIRST and **36/36 still pass**. The behaviour is correct; the pin is not.

`severity: ______`

**4. `issue-63-advance-gate-diagnostic-truncated` [arm a] — AC-001 under-tested**
AC-001 concerns 2+ unticked checklist items; the test leaves only one unticked,
so `toContain("Record constraints")` cannot distinguish the full message from
headline-plus-one-line. Verified: truncate `detail` to the first 2 lines —
**the exact regression #63 was filed to fix** — and 18/18 still pass.

`severity: ______`

**5. `issues-135-129-declared-boundary-parse` [arm a] — guard unpinned**
AC-005's fixture has no backticked tokens, so `tablePatterns` returns via the
"no backticks" path and never exercises the `Path`-header guard. Verified:
replace the guard with `if (false) return []` and **63/63 still pass**.
(Second finding, not separately replicated: AC-013's ledger pin asserts only
`code === 0` and does not assert `#`-line pass-through.)

`severity: ______`

**6. `issue-74-plan-briefs-dead-plan-anchor` [arm a] — strictness unproven**
The code comment at `forge-plan-prds.js:1276-1279` insists on
`=== true, not truthy and not !== false: an absent field is a claim nobody
made`. Verified: relax it to `!== false` and **28/28 still pass**. The
distinction the code documents is pinned by nothing.

`severity: ______`

---

## After you annotate

κ is computed over the six. Note that if we both mark everything BLOCKING, κ is
undefined (no variance) — which is itself worth reporting, and is the reason
PRIOR-ART §4 warns against raw agreement.

Then the number that matters: **which rejections were true positives**, and
therefore what each arm's false-positive rate actually is.
