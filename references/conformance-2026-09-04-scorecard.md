# Conformance scorecard — Run 3, measured

**Scored:** 2026-09-04 · **Rubric:** `redkiln/references/conformance/RUBRIC.md`
· **Subject:** redkiln @ `47faec9f` (the inversion spike's frozen corpus) ·
**Run:** 3 · **Previous:** `2026-08-08-scorecard.md` (Run 1),
`2026-08-09-scorecard.md` (Run 2)

**Filed in the spike repo, not `redkiln/references/conformance/`,** because
redkiln is frozen for the duration of this programme. Move it there when the
freeze lifts.

---

## What makes this run different

Runs 1 and 2 scored the repository **from disk**: an auditor reads the code and
judges whether the machinery exists. Run 2 closed every gap and recorded
**25 PASS / 0 PARTIAL / 0 FAIL**.

**Run 3 scores a subset from measurement rather than from reading.** The
inversion spike ran 145 model dispatches against this corpus and produced
evidence no static audit could reach: what the review gate actually does when
run, how often it agrees with itself, and what the gate's own telemetry records.

**Six rules move, all downward, all HARD or the SOFT rule closest to them.** That
is not a regression in the code — nothing changed at `47faec9f`. It is what
happens when a rule that was scored on *presence of machinery* is re-scored on
*behaviour of machinery*.

**Nineteen rules are NOT re-audited.** Their Run 2 verdicts stand and are cited
as such. Per the rubric's own standing constraint — "a rule scored PASS without
an evidence path is a defect in the scorecard" — this scorecard asserts **no new
PASS**. A carried-forward row is labelled `carried`, not `PASS`.

## Summary

| | Run 1 | Run 2 | **Run 3** |
|---|---|---|---|
| PASS | 17 | 25 | — (0 asserted) |
| PARTIAL | 4 | 0 | **5** |
| FAIL | 4 | 0 | **1** |
| carried (not re-audited) | — | — | **19** |
| **Total** | **25** | **25** | **25** |

---

## The rows that move

### GATE-01 [HARD] — Every gate can fail, and blocks when it does → **PARTIAL**

**Evidence:** `workflows/forge-fix.js:981` —

```js
const review = await agent(…, { agentType: 'redkiln:redkiln-review', schema: REVIEW_DIGEST });
if (review && review.pass === false) { … }
```

**The consumer fails open.** If the dispatch returns no digest, `review` is
falsy, the branch is skipped, and the workflow proceeds exactly as it would on
`pass: true`. A gate that cannot distinguish "approved" from "produced no
verdict" does not block when it fails; it blocks only when it *speaks and says
no*.

**And there is a measured mechanism that produces exactly that state.** Spike
finding **F20**: an agent card's `tools:` is an allowlist, and structured output
is delivered by a tool named `StructuredOutput`. `agents/redkiln-review.md:13`
declares `tools: [Read, Bash]` — no `StructuredOutput` — and this dispatch pairs
that card with `schema: REVIEW_DIGEST`. On the CLI path this composition makes
the schema **silently inert**: exit 0, `subtype: "success"`, no warning, and a
prose answer in place of the digest.

**Honest limit, and it is the reason this is PARTIAL and not FAIL:** F20 was
measured on the `claude -p` CLI path (`--agents` + `--agent` + `--json-schema`).
redkiln dispatches through the Workflow runtime's `agent()` helper, and **whether
that path resolves the tool allowlist identically is NOT verified.** The
fail-open consumer is verified statically; the mechanism that would trigger it is
verified on a different path.

**Cheapest resolution:** one dispatch. Run `fix-review` and assert the result is
an object with a boolean `pass`. If it is prose, this is a live FAIL on a HARD
rule guarding the project's primary quality gate.

**Fix regardless of that outcome:** `if (!review || review.pass !== true)` —
treat an absent verdict as a block. Absence of evidence is not approval, which is
GATE-04's rule.

### GATE-02 [HARD] — The maker never grades itself → **PARTIAL**

The rule has two clauses. The first holds: review is a distinct dispatch in a
fresh context, and no code path hardcodes an approval.

**The second does not: "the reviewing agent holds no write tools."** The card
grants `Bash`, which is unrestricted write access to the working tree.

**This is not theoretical — it was observed.** Spike finding **F17**: given a
tree whose state it found suspicious, the reviewer ran `git checkout` to restore
a different commit, reviewed *that*, ran its own mutation experiment, and
restored the tree afterwards. Verbatim from its report: *"the `s01-wt` checkout
sitting at that SHA holds a reverted spike fixture, **which I restored after
use**"*.

The behaviour was competent and, in that instance, admirable. It is still a
reviewer with write access to the artefact under review, which is the precise
thing the rule forbids — and the reason it forbids it is that a capable agent
finds the cheapest path to a green signal.

**Tension worth stating:** `Bash` is also what makes this reviewer good. It runs
the suite, mutation-tests assertions, and caught defects the suite could not
(**B5**/F26). Removing `Bash` would likely cost more than the rule protects.
The defensible resolution is a **read-only execution surface** — run commands in
a container or a copy — not removing execution.

### GATE-04 [HARD] — Absence of evidence is never recorded as a pass → **FAIL**

The result vocabulary exists and is correct: `evaluated` / `memoized` / `warned`
/ absent are distinguished, which is what Run 2 verified.

**The measured behaviour defeats it at a level the vocabulary does not reach.**
Spike findings **F24**/**F25**: **5 of 23 fix-lane items (22%) shipped with an
acceptance criterion that no test pins**, each verified by mutating the
behaviour and observing the suite stay green — 36/36, 18/18, 63/63, 28/28.

Each of those criteria was recorded as verified, on evidence that **could not
have failed**. That is absence of evidence recorded as a pass, arriving through
a route the "skipped" state was never designed to catch.

**Root cause (F25), and it is structural rather than careless:** `forge-fix`
mandates Red→Green per AC and the report records it — but **a test is red if any
of its assertions fails, while an AC is a claim about one of them**. A new
capability's test is red before the fix because the capability is absent, so the
evidence is satisfied for free while the criterion goes unexercised. The
corpus's own report for `issue-146-147` records *"RED pre-fix, same static proof
(the ordering words did not exist) → GREEN"* — literally true, and it conflates
two assertions.

**Remediation exists and is validated**, staged at `spikes/remediation/`: per-AC
Assertion / Mutation / Test-run / Red? columns, with `GREEN` as a failing value.
Applied to the real failure, AC-015's row cannot be completed. Not applied here
because the corpus is frozen.

### GATE-05 [SOFT] — Gate outcomes are observable → **PARTIAL**

Both halves of the check passed in Run 2 and still do in form: `basis` is
recorded, and gates demonstrably fire (**F12**: 187 pass / 53 fail — a 22%
negative class).

**But the basis field silently stopped meaning what it records.** Spike finding
**F24**, reproduced end to end: after `fix-advance-write-lock-gate-scope`, every
real `--apply` through a memoized verify gate records
`basis: "memoized"` where it previously recorded `"evaluated"` —

| | recorded `basis` |
| --- | --- |
| with the fix | `memoized` |
| with the fix's pre-flight line removed | `evaluated` |

The check **did** run for real, in the unlocked pre-flight; the confirming pass
inside the lock hits the memo and reports the cache. The field that exists to
distinguish "we evaluated this" from "we reused a result" now reports the latter
for a suite that ran seconds earlier in the same command — and prints
`pass --fresh to force a re-run` as advice, which is actively misleading.

**Nothing caught it**, and the reason is worth recording: `gateResults()` in
`test/advance-lock-scope.test.ts:432` reduces each event to `{stage, pass}` and
**never captures `basis`**. The fix's own tests pin the *warning* the memo emits,
but not the basis it implies.

### SHAPE-01 [HARD] — Consequential dispatches return validated structure → **PARTIAL**

Every consequential dispatch declares a schema; statically this passes, as Run 2
found. **Declaring a schema and receiving validated structure are different
things** (F20, and see GATE-01). The composition redkiln uses — a tool-restricted
card plus a demanded schema — is the one measured to yield prose on the CLI path,
with no error.

This rule is the reason the rubric exists: "prose-parsed control flow → FAIL".
The spike's own rig hit exactly this and lost a dispatch to it before the cause
was found. Same limit as GATE-01: unverified on the SDK path, cheap to settle.

### MEAS-04 [SOFT] — Known measurement gaps are documented where the number is used → **PARTIAL**

Two gaps now measured and undocumented at the point of use:

1. **`verify.affected_gate` performs no affected-area selection.** It is
   `pnpm typecheck && pnpm lint && pnpm test` with no `{{changed}}`
   placeholder, so **every gate run is the whole 236-file suite** regardless of
   what changed. Nothing at the point of use says the "affected area" is the
   whole repository. Measured impact: reviewers running the gate spent 300s+
   inside dispatches that otherwise take 130s.
2. **The `basis` semantics change** above is undocumented in spec, report or
   CHANGELOG.

---

## Carried forward — not re-audited (19)

Run 2 verdicts stand; this run neither confirms nor contests them, and asserts
no new PASS for them. See `2026-08-09-scorecard.md` for evidence paths.

`PACK-01` · `PACK-02` · `PACK-03` · `PACK-04` · `DEPTH-01` · `DEPTH-02` ·
`DEPTH-03` · `DEPTH-04` · `GATE-03` · `SHAPE-02` · `SHAPE-03` · `SURF-01` ·
`SURF-02` · `SURF-03` · `SURF-04` · `SURF-05` · `MEAS-01` · `MEAS-02` ·
`MEAS-03`

**`GATE-03` deserves a note despite not being re-audited.** Its rule —
"completion means reachable, not test-green" — is adjacent to what GATE-04
failed on, but distinct: the spike measured whether tests *discriminate*, not
whether capabilities are *wired in*. The DoD machinery Run 2 verified was not
exercised here.

---

## What Run 3 says, in one paragraph

The machinery is all present — Run 2 established that and nothing here contests
it. What measurement adds is that **three of the gate rules are satisfied in
form and defeated in behaviour**: a gate whose consumer treats silence as
approval, a reviewer holding the write tool the rule forbids, and a
verification record that reports a cache hit for a check that genuinely ran.
None of these is visible from disk, which is the argument for scoring a rubric
by running the system rather than reading it.

**The counterweight, and it is not small.** The same measurements produced the
strongest evidence in the programme *for* this architecture: the reviewer caught
the single mutant redkiln's entire affected-area gate failed to kill, in four of
four attempts, twice without being told which file to look at (**B5**/F26). The
gates are imperfect in the specific ways above. The reviewer behind them is
better than the suite it is checking.

---

## Threats to this scorecard

1. **Six rules re-audited, nineteen not.** This is a partial scorecard and says
   so; it is not a Run 2 replacement.
2. **GATE-01 and SHAPE-01 rest on a mechanism measured on the CLI path**, while
   redkiln dispatches through the Workflow runtime. Both are marked PARTIAL for
   that reason, and both are settleable with one dispatch.
3. **The auditor built the instrument.** Every measurement cited was produced by
   the spike's own runner, whose defects (F20, F17, the abbreviated-SHA fetch)
   are themselves recorded in `FINDINGS.md`. Where a finding was replicated
   independently — the mutation survivals, the end-to-end `basis` reproduction —
   that is stated at the row.
4. **Corpus is a single frozen SHA.** Behaviour measured at `47faec9f` may not
   hold at HEAD.
