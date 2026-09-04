# Remediation package — per-AC mutation evidence

**Status:** proposed, not applied. **Target:** the `redkiln` TypeScript repo,
which is the spike's **frozen corpus** (`spikes/corpus/PINNED_SHA`,
`47faec9f`). Nothing here has been applied to it: every E3 fixture fetches from
that repo and every result is pinned to that SHA, so the corpus stays frozen
until the programme closes. Apply on a branch when you are ready.

**Evidence:** FINDINGS **F25** (root cause), **F24** (the six replicated
findings), **F23** (the E3 result).

---

## The defect being fixed

`forge-fix` mandates Red → Green: a failing test per AC. The
`implementation-report.md` records it and the reviewer checks it. Both are
honest. Both are satisfied by a test that does not pin its AC — **because a test
is red if ANY of its assertions fails, and an AC is a claim about ONE of them.**

A new capability's test is red before the fix because the capability is absent.
That redness is free. `issue-146-147`'s AC-015 is the clean case: its test
asserts both "does `record-links` exist?" and "is it ordered after the commit?".
Removing the step makes it red on the first; **inverting the ordering leaves
36/36 green**. The report recorded "RED pre-fix, same static proof (the ordering
words did not exist) → GREEN" — literally true, and it conflates the two.

Five of 23 fix-lane items (22%) shipped with a criterion nothing pins.

## What is in this package

| file | replaces | change |
| --- | --- | --- |
| `implementation-report.md` | `templates/implementation-report.md` **and** `.redkiln/templates/implementation-report.md` (byte-identical at the pin — update both) | TDD Evidence table gains **Assertion**, **Mutation (post-fix)**, **Test run**, **Red?** |
| `redkiln-review.md` | `agents/redkiln-review.md` | reviewer RE-RUNS one mutation row rather than reading it; and a definition of "untested" that distinguishes it from a narrowed pin |

`.card-orig.md` and `.template-orig.md` are the pinned originals, kept so the
diff is checkable.

## Why these two and not the obvious third

**Rejected: "make the review card require mutation."** The card already asks
*"is there a test that would fail if the behaviour regressed?"*. The instruction
is not missing — answering it truthfully requires mutating, and the reviewer
does that at its discretion. Measured, on one item, three identical runs
(same fixture, prompt, model, effort):

| draw | verdict |
| --- | --- |
| 1 | `changes-requested` — found the vacuous AC-015 test |
| 2 | **`approved`** |
| 3 | `changes-requested` |

**2 of 3.** A probabilistic detector is not repaired by asking it more firmly,
and review is the most expensive step in the pipeline (CLEAN reviews ran
700–2400s). The fix has to move the check out of the reviewer's judgement and
into an artifact that must be written truthfully.

**Rejected: "bind the rubric to the verdict."** Proposed on the strength of
`issue-63`, whose reviewer found the gap, docked `ac-coverage` to 2/3 and
approved anyway. On replication that finding was **downgraded to advisory** —
the regression IS pinned; only the AC's generality is not — so the reviewer's
call was right and the motivating case dissolved. The corpus then kills the
general form outright:

- 8 of 23 items carry a docked dimension; **4 were E3-rejected, 4 were
  E3-approved.** "Any dimension < 3 blocks" false-blocks half of what it stops.
- Narrowed to `ac-coverage < 3`: `issue-63` (correctly approved) and
  `issue-130` (approved by both arms). **Zero benefit, two false blocks.**

What survives is the *definition*, which costs nothing and is included in the
card patch: "untested" means no assertion discriminates the claim (blocking); a
narrowed pin is a deduction, not a block.

## Validation — it catches the real failure

Filling AC-015's row as the new template demands, against the actual fixture:

```
Assertion : test/implement-contract.test.ts:449
Mutation  : workflows/forge-implement.js:1800 → ordering inverted to "FIRST, before the commit"
Test run  : npx vitest run test/implement-contract.test.ts
Red?      : GREEN — 36/36 passed
```

`GREEN` is a failing value under the new template, so the row cannot be
completed and the story is not done. **The historical failure is caught
deterministically**, by construction rather than by a reviewer noticing.

The same procedure reproduces the other three: the `Path`-header guard replaced
with `if (false)` (63/63 green), `=== true` relaxed to `!== false` (28/28
green), and — the one it correctly does NOT block — `issue-63`, whose real
one-line truncation turns the suite **red**, so its row completes and the AC is
pinned.

## Cost

One scoped test-file run per AC, not per story, and not the full suite:
`issue-63`'s file is 18 tests in ~30s against a ~300s full suite. For a
five-AC fix that is roughly 2–3 minutes of machine time, paid once at
implement, against a defect rate of 22% of shipped items.

## Applying it

```bash
cd /d/repos/redkiln
git switch -c remediation/per-ac-mutation-evidence
cp <spike>/spikes/remediation/implementation-report.md templates/implementation-report.md
cp <spike>/spikes/remediation/implementation-report.md .redkiln/templates/implementation-report.md
cp <spike>/spikes/remediation/redkiln-review.md agents/redkiln-review.md
```

Then check whether anything asserts on the template's shape —
`test/lockup-outliner.test.ts` and `workflows/forge-fix.js` /
`workflows/forge-implement.js` all mention "TDD Evidence" at the pin, and
`template_sig` / `rendered_sig` on existing reports are computed from the
template, so **existing items will show a signature drift that needs a
deliberate migration decision.** That is the one part of this package that is
not a drop-in, and it is unexamined here.
