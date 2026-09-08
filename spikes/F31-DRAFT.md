## F31 — S-TYPED RESULT: the typed contract costs no detection, conforms perfectly, and under the resolved policy blocks 9/23 on CLEAN with **zero verified false positives**

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

### What that did to the co-primary, before the severity call

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

### The severity call, and the resolved result

**Decided by Ryan, 2026-09-07, after the adjudication and recorded in
`results/s-typed-adjudication.jsonl`: `narrowed-pin` ADVISES; it does not
block.** `reproduced` and `mutation-survived` block. That is exactly the
**lenient** policy as registered in §2.2 before any data existed — the choice was
between two pre-registered options, not a threshold fitted to the result.

This closes the 0/23–12/23 range the adjudication left open:

| under the resolved (lenient) policy | |
| --- | --- |
| CLEAN items blocking | **9/23** (strict was 12/23) |
| of those, resting on an adjudicated-CONFIRMED finding | **9 of 9** |
| **verified false positives** | **0/23** |
| findings reclassified to advisory | 7 (all adjudicated true, now non-blocking) |
| S-SILENT detection | **23/23** — unchanged |
| S-SUBTLE detection | **5/5** — unchanged |

| CLEAN item still blocking | blocking-grade classes | adjudicated |
| --- | --- | --- |
| `fix-advance-write-lock-gate-scope` | `reproduced` 2 | F24 CONFIRMED |
| `issue-146-147-boundary-window` | `mutation-survived` 3 | F24 CONFIRMED |
| `issues-135-129-declared-boundary-parse` | `mutation-survived` 3 | F24 CONFIRMED |
| `issue-63-advance-gate-diagnostic-truncated` | `mutation-survived` 1 | F24 CONFIRMED |
| `issue-130-slice-review-story-gate` | `mutation-survived` 2 | F31 CONFIRMED ×2 |
| `issue-131-kb-ingest-doctor-scope` | `mutation-survived` 2 | F31 CONFIRMED ×2 |
| `issue-70-plugin-root-unset` | `mutation-survived` 1 | F31 CONFIRMED |
| `issue-81-plan-discover-spec-gate-mismatch` | `mutation-survived` 1 | F31 CONFIRMED |
| `issue-83-plan-prds-write-timeout` | `mutation-survived` 1 | F31 CONFIRMED |

Dropping to advisory: `fix-doctor-git-config-repair`,
`issue-82-status-terminal-field-collision`, `memoize-repo-root` — the three items
whose only blocking-grade evidence was `narrowed-pin`. Note `memoize-repo-root`
is the one finding the whole suite already catches, so the call removes the
weakest item on independent grounds too.

**The decisive fact is that lenient costs no detection.** S-SILENT stays 23/23
and S-SUBTLE 5/5 — identical to strict. Every seeded defect is caught by a
`reproduced` or `mutation-survived` finding, so nothing in the detection case
ever depended on `narrowed-pin` blocking. The policy relaxation is free.

### §2.6 decision rule — resolved

- **Branch 3's remedy is adopted.** Strict FP ≥5/23 as measured (12/23), and the
  branch's own instruction — send `mutation-survived` excess to mutation
  adjudication first — returned **zero false positives**. Its remedy,
  **"lenient becomes the default stage weight"**, is what the severity call
  independently chose.
- **Branch 1's condition is then met and D4 and D6 stand as written.** Detection
  ≥22/23 (it is 23/23) and false positives ≤2/23 (they are **0/23** verified).
  **Phase 1 ships the typed schema, with lenient as the registered default.**
- **Branch 2 (detection ≤20/23) does NOT fire.** 23/23 and 5/5.
- **Branch 4 (conformance <90%) does NOT fire.** 100%.

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
7. **D4's default stage weight is set: lenient.** `narrowed-pin` advises,
   `reproduced` and `mutation-survived` block. Under it the typed schema blocks
   9/23 on CLEAN with **zero verified false positives** and loses **no**
   detection (23/23, 5/5). Both halves of §2.6's branch 1 are satisfied, so
   **D4 and D6 stand as written and Phase 1 ships the typed schema.**

### Limits

1. **Severity was decided by a human, after and separately from the mechanical
   adjudication.** F31 establishes factual truth only — that every blocking claim
   replicates. The choice that `narrowed-pin` advises is Ryan's, recorded in
   `results/s-typed-adjudication.jsonl`, and it selected between two policies
   §2.2 registered before any data. The model that built the rig did not make it.
   **A different severity call would give a different blocking rate** (12/23
   under strict) without changing a single measured fact.
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

**The measurement question is closed and the policy question with it.** Phase 1
ships the typed schema under the lenient stage weight. What that buys, stated
plainly:

- Nine real defects caught per 23 shipped-and-approved items, against the prose
  reviewer's two — on a corpus whose ground truth F24 already showed to be wrong.
- Zero verified false positives, and a control that never fired in 10 draws
  (F29).
- Every blocking finding carrying a re-runnable command, which is what made this
  adjudication free and is what Phase 1 can automate.

**Two things this does not settle.**

The seven `narrowed-pin` findings are true and now advisory — they need somewhere
to go. F29's branch 4 did not fire (the control never rejected), so an `Advisory`
sink is not needed to suppress noise; it is needed to **carry real findings that
do not block**. Discarding them silently would waste adjudicated-true signal.

And the pairing conclusion stands unchanged and is the most consequential thing
in either finding: **four adjudicated defects survive a fully green suite, and
F29 showed the review gate approves a reproduced regression in 3 of 10 draws.**
Neither gate is sufficient alone. Phase 1 needs both on every required AC.
