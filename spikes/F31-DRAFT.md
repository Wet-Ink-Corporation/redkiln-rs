## F31 — S-TYPED RESULT: the typed contract costs no detection, conforms perfectly, and its 12/23 "false positives" contain **zero verified false positives** — every blocking finding replicates

**Established by:** 52 typed dispatches, 2026-09-06/07, arm (b), Opus/`xhigh`,
corpus pin `47faec9f`, `claude` **2.1.263** on all 52 records.
`spikes/results/s-typed.jsonl`. **51 valid, 1 void** (a timeout, retried
successfully — Limit 6). Pre-registered in `METHOD-S.md` §2 as amended
2026-09-06 (`86059c9`), with the ceiling and retry amendments of 2026-09-06/07.
Comparators **not re-run**: E3 `b-s-silent`/`b-clean`, E3-H `b-s-subtle`.
$112.25, 12.7 h of dispatch, no throttle halt.
**Adjudication:** `spikes/results/s-typed-adjudication.jsonl`, 14 mechanical
replications, **zero model cost**, 2026-09-07.

### The three detection conditions: ties at ceiling

| condition | typed / strict policy | prose comparator | discordant |
| --- | --- | --- | --- |
| S-SILENT strong detection | **23/23** | 23/23 (F23) | none |
| S-SUBTLE strong detection (negative control) | **5/5** | 5/5 (F26) | none |

**Schema conformance: 52/52 = 100%.** Not one digest failed the closed enum; the
single void is a timeout, not a schema failure. §2.6's last branch — "< 90% means
the enum shape is wrong for the model" — is nowhere near firing.

**`repro.command` fill on blocking-grade findings: 147/147 = 100%.** Every
blocking finding carries a re-runnable command with an `observed` and an
`expected_when_met`. This is not a cosmetic win: **the adjudication below was
executed entirely from those fields.**

**The risk §2.1 named did not materialise.** The worry was that a closed schema
with no `pass` would change *what the reviewer looks for* — stopping once
something is classified, or retreating into `advisory`. It did the opposite: on
S-SILENT it reported **106 `reproduced`** against 21 `advisory`.

### CLEAN: 12/23 under strict, and what adjudication did to that number

| policy | typed | prose comparator (F23) |
| --- | --- | --- |
| **strict** | **12/23** | 2/23 |
| **lenient** | **9/23** | — |

At face value a 6× false-positive rate. §2.6's third branch, however, requires
that `mutation-survived` excess be **"adjudicated by mutation as in F24 before it
counts."** Five of the twelve blocking items were already adjudicated by F24. The
remaining eight were adjudicated here.

### The adjudication: 14 findings, 14 CONFIRMED, 0 refuted

Method, following F24: apply the reviewer's **own cited `repro.command`** in the
frozen CLEAN fixture, re-run the named suite, compare against its stated
`observed`. Each replication was a single sequential run with the anchor asserted
before mutation and reverted after.

| item | AC | class | verdict | targeted result |
| --- | --- | --- | --- | --- |
| `issue-70-plugin-root-unset` | AC-001 | mutation-survived | **CONFIRMED** | 5/5 green; **full suite 1890/1890 green** |
| `issue-83-plan-prds-write-timeout` | AC-002 | mutation-survived | **CONFIRMED** | 31/31 green; **full suite 1907/1907 green** |
| `issue-131-kb-ingest-doctor-scope` | AC-005 | mutation-survived | **CONFIRMED** | 10/10; full suite = baseline, **0 new failures** |
| `issue-131-kb-ingest-doctor-scope` | AC-002 | mutation-survived | **CONFIRMED** | 10/10; full suite = baseline, **0 new failures** |
| `issue-130-slice-review-story-gate` | AC-007 | mutation-survived | **CONFIRMED** | 73/73 green **and** `check:workflows` exit 0 |
| `issue-130-slice-review-story-gate` | AC-005 | mutation-survived | **CONFIRMED** | 37/37 green |
| `issue-81-plan-discover-spec-gate-mismatch` | AC-001 | mutation-survived | **CONFIRMED** | 34/34 green |
| `issue-130-slice-review-story-gate` | AC-005 | narrowed-pin | **CONFIRMED** | 1 passed / 36 skipped |
| `issue-81-plan-discover-spec-gate-mismatch` | AC-002 | narrowed-pin | **CONFIRMED** | 34/34 green |
| `issue-82-status-terminal-field-collision` | AC-003 | narrowed-pin | **CONFIRMED** | 59/59 green |
| `issue-83-plan-prds-write-timeout` | AC-001 | narrowed-pin | **CONFIRMED** | 21/21 green |
| `fix-doctor-git-config-repair` | AC-002 | narrowed-pin | **CONFIRMED** | 15/15 green |
| `fix-doctor-git-config-repair` | AC-004 | narrowed-pin | **CONFIRMED** | 15/15 green |
| `memoize-repo-root` | AC-003 | narrowed-pin | **CONFIRMED** | AC-003 tests green — **but 3 other tests red** |

**Fourteen for fourteen.** With F24's six for six, **twenty blocking findings from
this reviewer have now been adjudicated across two independent passes and not one
was false.**

**Four are outright escapes.** `issue-70` and `issue-83` AC-002 each survive a
**fully green** suite (1890/1890 and 1907/1907); `issue-131`'s two survive with
**zero new failures** against a 54-failure baseline. Those are live regressions
the deterministic gate cannot see — which is the same shape as F29's finding
about the review gate, inverted: here it is the *suite* that is blind.

**One is materially weaker.** `memoize-repo-root` AC-003 is true as stated — its
two designated tests are blind to the mutation — but three other tests catch it,
so nothing would actually escape.

### What that does to the co-primary — and why it is now a severity question

Following F24's own construction exactly: **if every adjudicated finding is
accepted as blocking, the typed arm's strict false-positive rate is 0/23. If none
is, it is 12/23. The truth is in between and it is Ryan's call.** F24 said the
same of E3's 4/23 and 2/23, and for the same reason: **CLEAN's ground truth is
"this shipped and a review approved it", which is not "this is defect-free"** —
and it is now known wrong for at least 13 of 23 items rather than 5.

The severity split is clean and the numbers are:

- **7 `mutation-survived` / `reproduced` findings** — "I changed the behaviour the
  AC describes and the test stayed green". Four demonstrably escape the whole
  gate.
- **7 `narrowed-pin` findings** — "a test exists but pins a narrower claim than
  the AC". A reviewer may legitimately treat these as advisory. If they are,
  strict drops toward the `mutation-survived` set and **lenient's 9/23 becomes the
  operative number**.

### §2.6 decision rule — which branch fires depends on that call

- **Branch 2 (detection ≤20/23 → the typed contract costs detection): does NOT
  fire.** 23/23 and 5/5.
- **Branch 4 (conformance <90% → re-spike the enum): does NOT fire.** 100%.
- **Branch 1 (detection ≥22/23 AND strict FP ≤2/23 → D4 and D6 stand as written,
  Phase 1 ships the typed schema): FIRES if the adjudicated findings are accepted
  as blocking.** Detection qualifies outright; the FP condition is satisfied at
  the adjudicated end of the range.
- **Branch 3 (strict FP ≥5/23 → lenient becomes the default stage weight): fires
  only on the unadjudicated reading**, and its own text sends `mutation-survived`
  excess to adjudication first — which is now done and returned zero false
  positives.

**So S-TYPED does not fail. It hands one decision to a severity judgement the
programme has already reserved for Ryan.**

### The §2.7 predictions, line by line

| # | prediction | outcome |
| --- | --- | --- |
| 1 | detection stays at ceiling (23/23 or 22/23) | **CONFIRMED.** 23/23, plus 5/5 on the added negative control. |
| 2 | F24 findings re-found and classified as F24 classified them, ≥4/5 | **SPLIT.** The registered contradiction secondary (blocking-grade citation) is **4/5 — met**. On *exact class match* it is **3/5**: `issue-63` was re-found but classified `mutation-survived` where F24 said `narrowed-pin` (stronger, not weaker); `issue-74` only as `advisory`. |
| 3 | strict FP 3–5/23; lenient ≤ E3's 2/23 | **FALSIFIED as measured** (12/23 and 9/23). But the prediction was about *false* positives, and adjudication found none — so the metric it named was never actually observed. |
| 4 | schema conformance ≥95% | **CONFIRMED.** 100%. |
| 5 | cost within ±15% of the prose arm | **FALSIFIED.** +101% (S-SILENT), +40% (S-SUBTLE), +71% (CLEAN); **+74% overall** ($112.25 vs $64.61). |

### The registered contradiction secondary: 4/5

| item | blocking-grade citations | prose arm |
| --- | --- | --- |
| `issues-135-129-declared-boundary-parse` | **3** | **approve** |
| `fix-advance-write-lock-gate-scope` | **2** | reject |
| `issue-146-147-boundary-window` | **1** | reject |
| `issue-63-advance-gate-diagnostic-truncated` | **1** | **approve** |
| `issue-74-plan-briefs-dead-plan-anchor` | 0 | approve |
| `issue-62` (control) | 0 | approve |

The two rows carrying D4's claim are `issues-135-129` and `issue-63`: the prose
arm **approved** both, F24 confirmed a real finding in both, and the typed arm
re-found it at blocking grade. The control is clean at 0 of any class.

### What this settles

1. **The typed contract does not cost detection.** 23/23 and 5/5, no discordant
   item. §2.1's central risk is closed — subject to Limit 2.
2. **The closed enum is not too hard for the model.** 100% conformance across 52
   digests including a required nested object with no optional fields. §4.4 gains
   no "closed enums need a free-text `other`" rule.
3. **Structured `repro` pays for itself immediately.** The entire 14-finding
   adjudication ran off `repro.command`, mechanically, at zero model cost. That
   is the flip criterion working end to end and it is the strongest practical
   argument in the amendment's favour.
4. **This reviewer's blocking findings are trustworthy.** 20 for 20 adjudicated
   true across F24 and F31. Combined with F29's 19/19 on-target citations, the
   evidence that it does **not** fabricate blocking findings is now substantial.
5. **D4's central move is validated.** Computing the verdict from findings rather
   than trusting `pass` is right — and F29 showed why it is necessary, since the
   prose reviewer approves a *reproduced* regression in 3 of 10 draws.
6. **The deterministic gate is not a sufficient backstop either.** Four
   adjudicated defects survive a green suite. F29 concluded the review gate cannot
   be a sole blocker; this shows the test gate cannot be one either. **Phase 1
   needs both, and neither is redundant.**

### Limits

1. **Severity is not adjudicated — only factual truth.** Exactly F24's scope. The
   0/23-to-12/23 range closes only on Ryan's judgement, and the model that built
   the rig must not make it.
2. **Both detection metrics were already saturated** (E3 23/23, E3-H 5/5). A tie
   at ceiling shows neither arm is stressed by these defects; it is not evidence
   of equivalence.
3. **The prose CLEAN comparator is a single draw of a flaky process.** F27 and F29
   measured per-item flip rates from 0.00 to 0.70. No comparison here that turns
   on a one- or two-item difference is safe.
4. **Schema and prompt changed together, by design** (§2.3). No effect is
   attributable to one rather than the other.
5. **Whole-suite escape status is not uniform in coverage.** Established for six
   findings (four escape, one is caught, one *not established*). The remaining
   eight were adjudicated at their named suite only. `issue-130` AC-005's
   whole-suite status was abandoned after four runs contaminated by the
   adjudicator's own process errors — a fixture reverted mid-run, and concurrent
   runs on one fixture. The targeted verdict is unaffected; the gap is recorded
   rather than papered over. This is the same error class F24 recorded in its own
   limit 4.
6. **One void, and the ceiling story is two-sided.**
   `issues-135-129-declared-boundary-parse` CLEAN timed out at 3601 s and was
   retried at 5400 s (§7), completing in **4562 s** with 3 `mutation-survived`
   findings on the F24 target files — a censored draw that *would have blocked*.
   **But the stated justification for the 3600 s ceiling was wrong in mechanism:**
   the typed arm is not systematically slower on CLEAN (median 1071 s vs prose
   682 s, **1.6×**; only 2 of 23 draws over 2400 s). One item has a long tail; the
   arm does not.
7. **Two fixtures are not green at their checkpoint** — `issue-131` (54 failures)
   and `issue-130` (62). Mutation results there are baseline-differenced, not
   read as absolute pass counts.
8. **One arm (b), one draw per item.** §2.5's `k/n` Wilson rule was not needed:
   no item has more than one valid draw.

### For the programme

The open item is no longer measurement, it is **a severity call on 14 adjudicated
findings** — specifically whether `narrowed-pin` ("the test is weaker than the
AC") blocks or advises. That single decision sets D4's default stage weight, and
both readings are now fully costed:

- **`narrowed-pin` blocks** → strict, 12/23 blocked on this corpus, 0 false.
- **`narrowed-pin` advises** → lenient, 9/23 blocked, and the 7 `narrowed-pin`
  findings become an advisory sink.

F29's branch-4 finding stands either way: the control never rejected in 10 draws,
so an `Advisory` sink is not needed to suppress noise — it would exist to carry
real but non-blocking findings.
