# Prior art — what the field already knows

**Date:** 2026-09-02. Platform claims are date-stamped; they describe behaviour
we do not control.

Purpose: ground the programme before spending anything. Three questions —
do the charter's citations hold up, is there published work we should copy
rather than reinvent, and does any of it change our design?

Short answer to the third: **yes, in four places**, and one of them threatens
the RFC's headline cost argument.

---

## 1. The charter's citations, checked

| Charter claim | Status |
| --- | --- |
| Agentless: three fixed phases, 32.00% on SWE-bench Lite at $0.70/instance vs CodeR 28.33% at $3.34 | **Holds.** Phases confirmed as localization → repair → patch validation. $0.70 is the figure for the 32.00% configuration; the original July-2024 release reported $0.34. The charter's use is fair |
| "Kim et al. — 260 configurations, 5 architectures, 6 benchmarks, compute-matched … effects from +80.8% to −70.0%" | **Materially wrong, and the correction is more useful than the claim.** See §2 |
| Reflexion / self-correction depends on test quality; Huang et al. — intrinsic self-correction without an oracle degrades accuracy | Consistent with the wider literature; not separately re-verified here |
| LLM-judge ceiling ~62.5% agreement / κ=0.37 | Plausible and conservative. But the charter's *reporting rule* is wrong — see §4. *[Amended 2026-09-11: the figure is [inferred, unsourced] — §11 A1]* |
| Temperature 0 is not deterministic (1,000 completions → 80 unique outputs) | Consistent with the consensus that agentic evaluation is stochastic end-to-end |
| "No published benchmark of in-session Task dispatch vs `claude -p` per agent" | **Still true**, but no longer the closest evidence available — see §3 |

---

## 2. The multi-agent scaling result is sharper than the charter says — and it
   points directly at our fix lane

The charter cites this as "no architecture dominates, effects +80.8% to −70.0%".
The actual study (Google/MIT) reports **180 configurations, 5 architectures,
4 benchmarks, 3 model families** — and the effect range is not a general spread.
It is *structured by task type*:

| Finding | Number |
| --- | --- |
| Sequential tasks — multi-agent **degrades** | **−39% to −70%** |
| Parallelizable tasks — centralized coordination **improves** | **+80.9%** |
| Error amplification, independent agents | **17.2×** |
| Error amplification, centralized | **4.4×** |
| Predicting the best architecture from tool count + decomposability | **87%** accurate |

A separate line of work adds: diminishing returns are strong in *homogeneous*
settings, while **heterogeneity** (different models, prompts, tools per role)
keeps yielding gains; and there is a capability-saturation effect where adding
agents stops helping once a single-agent baseline is strong enough.

**Why this matters more than the charter's version.** `forge-implement.js`
carries the annotation *"STRICTLY SEQUENTIAL over slices — DO NOT CONVERT TO
pipeline()/parallel()"*, and `forge-fix.js` is the most sequential lane we have.
The literature says sequential is exactly where multi-agent orchestration costs
39–70%. **E4's null arm now has a strong, specific prior in its favour** — not a
vague "orchestration may not help", but "this structural property is the one
associated with the largest measured losses."

And the mirror image resolves an apparent tension in our own documents.
Shipmates L6.1 says executing the initiative DAG's antichains in parallel is
"the largest single capability gain available"; the literature says
parallelizable tasks are where centralized coordination gains 80.9%. **The two
halves are consistent:** trim orchestration where we are sequential (the fix
lane), add it where we are genuinely parallel (the project DAG). Neither the RFC
nor the charter reached that synthesis.

One caveat kept deliberately: at least one compute-matched study finds
multi-agent pipelines dominating the Pareto front on reasoning benchmarks. The
evidence is not one-sided, and E4 should be read as testing *our* lane, not
settling the general question.

Also relevant to F8: the tool-coordination trade-off is described as a tax that
grows disproportionately past ~16 tools. Our isolated configuration reports 34;
the un-isolated one reported ~280.

---

## 3. TraceLab is the closest published analogue to our core claim

The RFC is right that nobody has benchmarked in-session subagent dispatch
against `claude -p` per agent. But TraceLab characterises real coding-agent
workloads and reports the number our argument turns on:

- Claude and Codex serve **~96%** of prompt tokens from prefix cache overall
  (95.7% token-weighted).
- **Tool-result continuations hit ~97.5%.**
- **Rounds starting from a new user message hit only ~84.4%** — because caches
  expire after a few minutes of inactivity and humans pause longer than that.
- Of 2.3 B append tokens, **only 19% is genuinely new context**; 81% is
  re-prefilling already-seen tokens.

**Read against inversion:** every `-p` dispatch is structurally a cold start,
much closer to the 84.4% case than the 97.5% one. That is the first external
quantification bearing on the RFC's cost case, and it is not flattering. It is
also not decisive — TraceLab measures within-session continuation versus new
message, not subprocess-per-agent — but it sets the expectation, and it says
cache misses are driven by *idle gaps*, which is precisely what CC4's
behavioural TTL probe measures.

Related methodology worth copying: published benchmarks warm on a first pass and
measure hit rate on a second pass over a fixed set of 50–100 representative
prompts, and they **report cache hit rate in the results table** so a reader can
tell whether a cost difference came from model price, token volume, or cache
reuse. Our probe already has that shape; the reporting rule is now explicit.

---

## 4. Our LLM-judge reporting rule is wrong, and the corpus is why

Charter §9.3 says the judge's **agreement rate** is reported alongside every
result it produces. The consensus guidance is that **raw agreement inflates
badly on imbalanced label distributions**: if 90% of items are "pass", a judge
that always says "pass" scores 90% agreement with κ ≈ 0.

Our corpus is *more* imbalanced than that example. FINDINGS records 215 pass /
2 fail on verdicts, 1,249 of 1,249 ledger rows satisfied, 35 of 36 reviews
scored 3. **Raw agreement is close to meaningless here.**

**Change:** report **Cohen's κ** (or Krippendorff's α where there are more than
two raters, e.g. E6's panel arm) as the headline, with raw agreement only
alongside. Conventional bands: κ > 0.80 strong, 0.60–0.80 substantial, < 0.60
means the rubric needs work. The charter's own cited ceiling (κ = 0.37) is
already below that floor, which is an argument for leaning on deterministic
outcomes wherever they exist — as the charter says it prefers to. *[Amended
2026-09-11: κ = 0.37 is unsourced; the argument rests on the Landis–Koch bands and
Judge's Verdict — §11 A1]*

---

## 5. Our gate-FPR experiment is mutation testing, and it has a known bias

S0.1 — inject known-bad patches and count how many the gate passes — is
mutation testing under another name. Two things follow.

**Use the established denominator.** Mutation score is
`killed / (total − equivalent)`. **Equivalent mutants** are mutations that are
semantically identical to the original; they *cannot* be detected and are not
evidence of a weak suite. Counting them as gate failures would inflate our
false-positive estimate — and the FPR we compute is the oracle E4 and E5 both
rest on, so the bias would propagate into both.

**Practical consequence:** reverting a historical fix while keeping its test is
a good mutation source precisely because the accompanying test proves
non-equivalence. Prefer that construction over synthetic mutations, and where
synthetic mutants are used, budget for manual equivalence triage on survivors.

Mature tooling exists (`cargo-mutants`, Stryker for TS/JS, PIT for Java); the
redkiln `.gitignore` already anticipates `cargo mutants`. We should not
hand-roll mutation operators.

---

## 6. Variance: the field runs 3–5 repeats, and variance is not uniform

Published agentic benchmarks commonly run **3–5 repeats per configuration** to
estimate run-to-run variance — consistent with the charter's sizing, and with
E0's design.

The more useful finding is about *shape*. In one reported evaluation, of 32
points across five runs: **15 passed all five, 1 failed all five, and 16 were
flaky** — variance concentrated in a small unstable subset rather than spread
evenly.

**Change to E0:** a single pooled σ would describe none of our items well.
Report **per-item stability** (always-pass / always-fail / flaky) alongside σ,
and size later experiments against the flaky subset, which is where the power is
actually needed. This also gives E3 and E4 a cheap sanity check: an item that is
always-pass in every arm carries no signal and should not be counted as
agreement.

---

## 7. Pre-registration: we are aligned with accepted practice

Registered reports entered software engineering at MSR 2020 and now run at three
conferences and two journals including *Empirical Software Engineering*. The
claimed benefits match the charter's reasoning exactly — preventing questionable
research practices, forcing design feedback before data collection, and letting
readers judge how severely a claim was tested. There is an OSF-based registered
report template for SE controlled experiments whose structure we can borrow.
*[Amended 2026-09-11: a proposed direction, not a finished template — §11 A2]*

This does not change the plan; it confirms it, and it means the open item in
FINDINGS (nothing committed, so `preregistration_sha` currently points at a
commit containing no prompts or arms) is a real defect against an established
standard rather than a private preference.

---

## 8. The finding that threatens the cost case

**Anthropic's platform docs state the default prompt-cache TTL is 5 minutes**,
with 1 hour available only by explicitly requesting
`cache_control: {"type": "ephemeral", "ttl": "1h"}`. Pricing: 5m write 1.25×
base input, 1h write 2.0×, cache read 0.1×.

Separately, a widely-cited GitHub issue against `anthropics/claude-code`
(#46829) reports — with 119,866 API calls of telemetry across Jan–Apr 2026 —
that Claude Code's effective TTL **silently moved from 1h to 5m around 6 March
2026**, showing `ephemeral_1h > 0` and `ephemeral_5m = 0` for 33 consecutive
days beforehand, then 5m outnumbering 1h 5:1 by 8 March. It attributes ~17%
cost inflation to the change. **Status: user-reported, never confirmed by
Anthropic, and closed as not planned.** Treat it as a serious hypothesis, not a
fact.

**Why this is the most consequential item here.** RFC §3.1's headline asymmetry
is that in-session subagents get "5 minutes, always" while the main conversation
— *including `-p`* — gets 1 hour on a subscription within plan usage. If the
effective default is now 5m for both, **that asymmetry does not exist, and the
cache half of the inversion cost argument collapses to parity.** The transcript-
accumulation argument survives untouched; the headline does not.

Our own measurement is consistent with the pessimistic reading so far: the
`usage.cache_creation` split is emitted and reads
`{ephemeral_1h_input_tokens: 0, ephemeral_5m_input_tokens: 0}` on every dispatch
observed — present but uninformative, so it cannot settle the question either
way. **Behaviour is the only route left**, which is exactly CC4's TTL probe.
That probe has gone from a nice-to-have to the highest-value cheap experiment in
the programme.

**A second, opposite finding worth holding.** The docs say cache hits are **not
deducted against the rate limit** ("when you want to improve your rate limit
utilization, because cache hits are not deducted against your rate limit"). The
charter §6.1 asserts the opposite — that "cached re-reads still draw down the
allowance … at roughly a tenth of the input rate." If the docs are right for
subscription windows too, a warm prefix is nearly free *in the unit that
actually binds us*, and the cost case gets stronger rather than weaker. These
cannot both be true. It is now directly testable against
`seven_day.utilization`, and E0 should settle it before anything is sized.

---

## 9. What changes

| # | Change | Driver |
| --- | --- | --- |
| 1 | E0 reports **per-item stability**, not one pooled σ | §6 |
| 2 | Judge results report **Cohen's κ**, not agreement rate | §4 |
| 3 | S0.1 adopts mutation-testing methodology: exclude equivalent mutants from the denominator; prefer revert-the-fix-keep-the-test as the mutation source; use existing tooling | §5 |
| 4 | CC4's TTL probe promoted to **first cheap experiment after E0** | §8 |
| 5 | E0 must settle **whether cache reads draw down the subscription window** before any sizing | §8 |
| 6 | E4's write-up states the sequential-task prior explicitly, so a null-arm win is read as *expected* rather than as a surprise | §2 |
| 7 | Results tables report cache hit rate alongside cost, per published practice | §3 |
| 8 | The RFC amendment records the corrected multi-agent numbers (180/5/4, −39–70% sequential, +80.9% parallel) | §2 |

## 10. Sources

- [Agentless (arXiv 2407.01489)](https://arxiv.org/abs/2407.01489) · [repo](https://github.com/openautocoder/agentless) · [FSE 2025 version](https://lingming.cs.illinois.edu/publications/fse2025.pdf)
- [Towards a science of scaling agent systems — Google Research](https://research.google/blog/towards-a-science-of-scaling-agent-systems-when-and-why-agent-systems-work/) · [Agent scaling via diversity (arXiv 2602.03794)](https://arxiv.org/html/2602.03794v1) · [Multi-agent Pareto-optimal test-time scaling (arXiv 2605.01566)](https://arxiv.org/pdf/2605.01566)
- [TraceLab (arXiv 2606.30560)](https://arxiv.org/abs/2606.30560) · [SyFI Lab summary](https://syfi.cs.washington.edu/blog/2026-06-25-tracelab/)
- [Anthropic prompt caching docs](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) · [claude-code issue #46829](https://github.com/anthropics/claude-code/issues/46829)
- [Cohen's kappa for LLM judges (AWS samples)](https://github.com/aws-samples/sample-GEDD/blob/main/grounded-evals/docs/cohens-kappa-for-llm-judges.md) · [Calibrating an LLM judge with human annotations](https://galileo.ai/blog/calibrate-llm-judge-human-annotations) · [Judge's Verdict (arXiv 2510.09738)](https://arxiv.org/pdf/2510.09738)
- [Registered reports in software engineering (EMSE)](https://link.springer.com/article/10.1007/s10664-022-10277-5) · [Benefits of preregistration and Registered Reports](https://www.tandfonline.com/doi/full/10.1080/2833373X.2024.2376046) · [OSF-based RR template for SE (arXiv 2602.09292)](https://arxiv.org/pdf/2602.09292)
- Mutation testing: [Codecov overview](https://about.codecov.io/blog/mutation-testing-how-to-ensure-code-coverage-isnt-a-vanity-metric/) · [LLMs for equivalent mutant detection (arXiv 2607.00511)](https://arxiv.org/pdf/2607.00511)
- [AgentLens (arXiv 2607.06624)](https://arxiv.org/pdf/2607.06624) · [Claw-SWE-Bench (arXiv 2606.12344)](https://arxiv.org/pdf/2606.12344) · [Prompt cache hit rate as a production metric](https://tianpan.co/blog/2026-04-20-prompt-cache-hit-rate-production-metric)

---

## 11. Amendments (2026-09-11)

Dated amendments from the 2026-09-10 pre-implementation audit (PIA-101 research-lineage part, PIA-108;
DECISIONS-2026-09-11 #14). The original lines stand with a marker; nothing above is rewritten.

**A1 — :22 and :120, the "~62.5% agreement / κ = 0.37" LLM-judge ceiling is [inferred, unsourced].** The figure
is quoted from the charter (`references/2026-08-31-rust-inversion-spike-charter.md` :221), which cites no
write-up, and a targeted search on 2026-09-10 attributed it to nothing live: the nearest on-topic study
(arXiv 2509.01494, code-review LLM-judge agreement) reports human–LLM κ 0.43–0.71 and human–human 0.61–0.76, and
no source states 0.37 with 62.5% agreement [documented, audit PIA-108, fetched 2026-09-10]. It is retagged
**[inferred, unsourced]** here and in the charter's reading. §4's argument does not need it: the rule change
rests on the **Landis–Koch bands** (κ < 0.60 means the rubric needs work) and on *Judge's Verdict: A Comprehensive
Analysis of LLM Judge Capability Through Human Agreement* (arXiv 2510.09738, confirmed live 2026-09-10 as the
sourced treatment of LLM-judge agreement in κ terms). "Already below that floor" therefore reads "an LLM judge on
this corpus should be assumed below that floor until measured, which E6's κ would do". The charter is a dated
document and is left as is; this amendment points at it.

**A2 — :175-176, the OSF "template to borrow".** Bett et al. 2026 (arXiv 2602.09292) is a **gap analysis that
proposes a direction for an OSF-based registered-report template for SE controlled experiments; it ships no
template**. The sentence reads: "there is **a proposed direction for an OSF-based template (Bett et al. 2026,
arXiv 2602.09292); no finished template exists**". §7's conclusion ("this does not change the plan; it confirms
it") is unchanged — the practice this programme follows is METHOD's own pre-registration form, not a borrowed
template [documented, fetched 2026-09-10].
