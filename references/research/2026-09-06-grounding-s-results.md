# Grounding the S-results against prior art — 2026-09-06

**Purpose.** S-VAR, S-H1b, S-WT and S-MIG are in (S-H1b's 24 h read pending). Before
S-TYPED runs and before the proposal moves to rev. 4, check each result against
published research and first-party documentation: does the wild agree, and what does
the design still lack that prior art already solved? Three research passes, one per
result family; their full briefs are appended as Parts A–C with sources. This section
is the synthesis. Evidence tags: **[peer]**, **[preprint]**, **[1P]** first-party
docs, **[blog]**.

---

## 1. Where the wild agrees with us

| our result | prior art | status |
| --- | --- | --- |
| S-VAR: detection stable, verdict flips (7/10 on a reproduced regression; 2–6/10 on test-quality defects) | *The Coin Flip Judge* (arXiv 2606.13685) [preprint] names it the **pairwise–pointwise paradox**: the binarisation step carries the noise; coding sits in the high-flip stratum (23.6%). *Rating Roulette* (EMNLP 2025) [peer]: intra-rater reliability depends on the **criterion**, not the model. *Are LLMs Reliable Code Reviewers?* (ASE 2026) [peer]: rationale-heavy prompts **raise** false rejection and produce rationale/verdict contradictions | **agrees, and it has a name** |
| Our response — findings with an evidence class, no verdict, machine decides | Jin & Chen's Fix-guided Verification Filter [peer] executes the model's own artefact and overrides deterministically (FNR 55–69% → 16–29%); Agentless [peer] and Meta ACH [peer] accept a claim only when its executable check runs; no production reviewer lets a model verdict block a merge directly | **agrees — the shape is the field's shape** |
| S-TYPED prediction: no detection loss under a closed schema | *Capacity, Not Format* (arXiv 2606.09410) [preprint]: frontier models absorb JSON when reasoning precedes the constrained message; Anthropic docs [1P]: grammar does not apply to thinking or tool calls | **agrees, with a caveat** (§3) |
| S-H1b: static `--system-prompt` forfeits a "subsidy"; prefix = shared tool block + text-dependent tail | Platform docs [1P]: prefix order is tools → system → messages; a cache entry is a hash at a breakpoint; a read refreshes the entry it hits; the cache is **org-scoped**, so the subsidy can come from any session on the account, not just this machine | **explains it** |
| S-WT: worktrees cache-free under a static prompt | Claude Code docs [1P]: the per-directory segment is the dynamic system-prompt section; a custom prompt omits it | **consistent** |
| Mark-and-resume human gate | Temporal signals, Restate awakeables, Step Functions task tokens, Azure external events, GitHub environments, Trigger.dev waitpoints [1P] — every durable system except in-process interrupts (LangGraph, Claude SDK `canUseTool`) | **same pattern** |
| Git trailers as state, SQLite as rebuildable cache | Gerrit NoteDb, git-native-issue (trailers), Forgejo's `refs/issues` proposal with SQL as a query index [1P/blog] | **same pattern** |
| `plan.md` dead in 110/110 | Scott Logic on Spec Kit [blog]: 2,577 lines of markdown for 689 LOC; Agent OS v3 retired phases; no system measures downstream reads | **agrees; nobody instruments it** |

## 2. Where it disagrees or corrects us

1. **n=10 is the floor, not a result.** Coin Flip's reliability curve: 1 trial matches the 50-trial majority 86.6% of the time; hard items need 15 trials for 90%. Our 7/10 vs 6/10 is inside ±30 pp. The class *ordering* (0.70 > 0.37 > 0.10 > 0.00) is usable; the *levels* are not, and nothing in rev. 4 may depend on a one-item difference.
2. **Stability is not validity** (*Reliability without Validity*, arXiv 2606.19544): the control's 10/10 approval is consistent with a lenient judge. S-VAR has no negative control with a defect the rubric does not name; E3-H's S-SUBTLE seeds are that fixture and belong in S-TYPED.
3. **"Fails on base" is not "reproduces the defect."** Agentless: 213 of 300 generated tests reproduced, only 94 also passed under the true fix. A `reproduced` finding must **flip** — fail on base, pass on a reference — or it is a note, not a block. Our design says "re-execute"; it does not yet say "flip".
4. **The static-prompt rule has a cost we mis-priced twice.** Rev. 3 said gate resume "should cost ~0" under a card prompt; S-H1b says it costs the *whole* prefix (29.6k). The docs explain why and offer an alternative we never priced: **default prompt + `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt`** keeps the account-subsidised static head and drops only the per-directory git section, leaving just the appended tail as ours to keep warm.
5. **The subsidy is account-wide, so it is also a hazard**: any Claude Code upgrade on the account changes tool definitions and invalidates the shared head for every session; TTL defaults have flipped server-side before (#46829). Pin the CLI version and `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` explicitly.
6. **No comparable false-block metric exists anywhere.** Cloudflare's reviewer blocks merges and humans break glass 0.6% of the time (288 / 48,095 MRs); no other vendor publishes one. Redkiln-RS has to log every block and every override from day one or it will be in the same position.

## 3. What the design still lacks (prior art already solved it)

Each row is a concrete delta for proposal rev. 4 or for a spike amendment.

| gap | precedent | delta |
| --- | --- | --- |
| A gate answer is bound to nothing but the item | Every durable system mints a **single-use token**; GitHub environments pin approval to a **SHA**; all but LangGraph carry a **deadline** with an explicit expiry path | Gate = `(token, tree_hash, deadline)`. A stale answer (tree hash differs) is **void, not reconciled**; expiry has a terminal path (fail / escalate / auto-decide), never a hang |
| `Reproduced` blocks on re-execution | Agentless flip criterion; Jin & Chen's four deterministic cases (equivalent / **both fail** / over-repair / success) | `repro` is `{command, expect_on_base: fail, expect_on_ref: pass}`; both-fail → `Unverifiable` → advisory; equivalent-mutant guard for `MutationSurvived` (ACH needed a detector at precision 0.79 raw) |
| Disputed findings are either blocks or nothing | GitLab flaky-test quarantine (allowed-to-fail lane + fix deadline); Cloudflare "won't fix" resolves on re-review | A `Quarantined` finding state: a human override records a reason, the finding stops blocking, and re-review does not re-raise it |
| No measure of the gate's own quality | Cloudflare's 0.6% break-glass; Anthropic's pass^k guidance | Event log records every block and override; `false_block_rate` and per-card `pass^k` are first-class telemetry (principle 11) |
| Dead artifacts found by census, not by the product | nobody instruments downstream reads; Agent OS v3 pruned by intuition | Emit `artifact.read` per activity input; a stage whose artifact has zero reads over N runs is flagged for pruning — the product version of the S-MIG finding |
| SHA-keyed rows orphan on rebase/amend | git-branchless `RewriteEvent(old→new)`; jj op-log conflict-as-value | Record `rewrite(old_sha → new_sha)` events (from `reflog`/`rebase` reconcile) so trailers re-anchor; represent "two runs advanced the same gate" as a conflict value, not an error |
| Worktree confusion is the commonest failure in the wild | Beads redirect file; Backlog.md #558 (MCP wrote to the main repo) | Resolve the common dir at startup; refuse to write when the worktree's HEAD is not the branch the run claims |
| Schema may squeeze reasoning into the final message | *Capacity, Not Format*: keep reasoning before the schema; Anthropic limits on optional/union fields | Every `Findings` item gains a free-text `rationale`; enums stay closed; no unions; reasoning happens in thinking/tool turns before the `StructuredOutput` call |
| Cache strategy has one option | Platform "pre-warm" pattern (`max_tokens: 0` + breakpoint, API only); `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` in the TS SDK | Price the `--exclude-dynamic` + `--append-system-prompt` route (S-H1c, §4); do **not** build a heartbeat on a subscription until quota accounting for a keep-alive turn is measured |

## 4. Amendments to make before S-TYPED dispatches (all pre-data)

1. **Schema:** add `rationale: string` per finding; `repro` becomes an object `{command, observed_on_candidate, expected_on_base}`; enums unchanged. Disclosed as an instrument change in METHOD-S §7.
2. **Third condition:** the five E3-H `s-subtle` seeds, typed arm only — the negative control S-VAR lacked (a defect the checklist does not name). +5 dispatches.
3. **Registered secondary — contradiction rate:** on CLEAN, count records whose findings name the F24 file with evidence ∈ {reproduced, mutation-survived} — mechanical, and the typed analogue of the exploratory keyword read on S-VAR (§5). This is the claim D4 rests on: that the finding is *reported* when the verdict would have been approve.
4. **Report pass^k with Wilson intervals** for any item S-TYPED touches more than once; do not compare arms on a one-item difference.
5. **S-H1c** (new, haiku, ~free): H1's three gaps under default prompt + `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt "<card>"`. Decision rule: if the 65-min read creates ≤ ~15k (the tail only), the card rule is re-specified as *append*, not *replace*, and worktree/commit invariance is re-checked under it (S-WT variant, 10 dispatches).

## 5. A note on the S-VAR keyword read

The observation that approving draws still name the defect (issue-63: 10/10 mention the narrowed pin, 1/10 block) is a post-hoc keyword match on rubric rationale text, not a registered metric. Jin & Chen's `NO_but_positive` contradiction class is the same phenomenon measured properly. It is carried here as the hypothesis S-TYPED §4.3 tests, not as a finding.

---

# Part A — Reviewer verdict instability and closed-schema output

(Research pass 1, verbatim.)

Date: 2026-09-06. Source classes: **[peer]** peer-reviewed/accepted venue, **[preprint]** arXiv only, **[vendor]** first-party docs, **[blog]** practitioner post. Numbers are quoted from the source; where I infer, I say so.

## Finding A, Q1: verdict instability vs detection stability in the LLM-as-judge literature

**Rating Roulette (Findings of EMNLP 2025; arXiv 2510.27106, Oct 2025) [peer].** Measures *intra-rater* reliability (same judge, same prompt, repeated runs) with Krippendorff's alpha. On MTBench, Qwen-3 gave the same pairwise judgment on all 3 runs for only 61.3% of items; alpha 0.27 (Llama 3.1) to 0.56 (Qwen 3), all far below the 0.8 threshold. Reliability varies by *criterion* (high on coherence/consistency, very low on fluency). Running 5 instead of 3 repeats did not change the estimate. Majority vote over runs improves human agreement; temperature 0 removes variance but hurts accuracy.
*vs our finding: agrees.* Per-criterion reliability differences mirror our "regression 7/10 rejected vs test-quality 2-6/10": threshold stability depends on how crisp the criterion is, not on the model's ability to see the defect.

**The Coin Flip Judge? (arXiv 2606.13685, Jun 2026) [preprint].** 50 trials per item, two GPT-4-class judges, 29 items. Mean pairwise flip rate about 14%, but bimodal: 14 "easy" items at 2.9%, 15 "hard" items (coding, writing, reasoning) at 23.6%. Key result for us: the **pairwise-pointwise paradox** — items with tiny mean score gaps still produce unstable forced-choice verdicts, i.e. the binarisation step, not the underlying assessment, is where the noise lives. 44.7% of pointwise score variance is within-item noise (ICC 0.58-0.77). Temperature 0 cut flip rate 79% for one model but only 43% for the other; residual flips persist at t=0 and are attributed to API-level nondeterminism.
*vs our finding: agrees and extends.* The paradox is the formal version of "detection stable, threshold unstable". Coding is explicitly in the high-flip stratum. Their evidence that t=0 is "necessary but not sufficient" tells us not to expect a decoding fix.

**Reliability without Validity (arXiv 2606.19544, Jun 2026) [preprint].** 21 judges, ~541k judgments. Exact-match agreement overstates judge quality by 33-41 pp vs kappa; test-retest >0.95 coexists with severe position bias (a "consistency-bias paradox").
*vs our finding: extends.* A stable verdict is not a valid one; our 10/10 clean-item approval is consistent with a lenient-but-stable judge.

**Judging the Judges (Thakur et al., arXiv 2406.12624, Jun 2024) [preprint, widely cited].** Judges are lenient on under-specified answers; GPT-4-class human alignment is well below human-human.
*vs our finding: agrees (weakly).* Leniency on under-specified items maps onto our test-quality defects, where "vacuous" is a judgment call and rejection fell to 2/10.

Code-review-specific: **Are LLMs Reliable Code Reviewers? (Automated Software Engineering 2026; arXiv 2603.00539) [peer]** and its predecessor **Uncovering Systematic Failures... (arXiv 2508.12358, Aug 2025) [preprint]**. Five models judge code against NL specs. False-rejection of *correct* code is high (GPT-4o FNR 26-36% with a bare prompt) and **rises when the prompt asks for rationale and fixes** (GPT-4o: 58-88%; Claude 4.5 Sonnet: 26% to 36% on HumanEval, 58% to 62% on MBPP). Richer prompting redistributes error from FPR to FNR rather than reducing it. They also audit rationale/verdict contradictions (a "NO_but_positive" class).
*vs our finding: agrees and extends.* Our xhigh-effort, rubric-rationale reviewer is exactly the "Full" prompting regime in which they see the decision boundary drift conservative. Their contradiction class is the same phenomenon as our rationale-names-the-defect-but-verdict-flips.

Also: Stureborg et al. (arXiv 2405.01724, May 2024) [preprint] on low inter-sample agreement and temperature/attribute-order sensitivity; RAND's Judge Reliability Harness (ICLR 2026 workshop) [peer] on binary verdicts flipping under paraphrase/formatting perturbations.

**Not found:** any public study of Google Critique/AutoCommenter, Meta, CodeRabbit or Copilot review measuring *verdict* flip rates. AutoCommenter (FSE-Industry 2024) [peer] reports comment usefulness (54-66%) and ~40% resolution; it emits no pass/fail.

## Finding A, Q2: precedent for "model emits evidence, deterministic system decides"

**Fix-guided Verification Filter (Jin & Chen, ASE journal 2026) [peer].** Closest precedent. When the judge says NO and proposes a fix, the pipeline *executes* both original and fix against reference tests plus spec-constrained generated tests and overrides the verdict via four deterministic cases (equivalent / both fail / over-repair / repair success). Averaged across models FNR fell 54.8% to 16.3% (HumanEval), 69.0% to 28.9% (MBPP), 51.0% to 24.0% (QuixBugs); GPT-4o on MBPP 88.7% to 40.0%.
*vs our finding: agrees.* This is "reproduce then decide" with the model's own artefact as the counterfactual. Difference: they still let the model emit a verdict and then correct it; we drop the verdict entirely. Their Case 2 (both fail: fall back, re-judge up to K=2, then keep original verdict) is a gap our design should handle explicitly (see implications).

**Agentless (Xia et al., arXiv 2407.01489, Jul 2024; FSE 2025) [peer].** Generates reproduction tests, executes them on the unpatched repo to confirm they reproduce, then selects among candidate patches by reproduction + regression execution and majority vote. 213/300 generated tests reproduced the issue; only 94 also passed on the ground-truth patch.
*vs our finding: agrees.* Same shape: LLM proposes an executable artefact, the machine decides by running it. Caution for our "reproduced" class: failing on base is not the same as reproducing *the* defect.

**Meta ACH (FSE 2025 Industry; arXiv 2501.12862) [peer].** Mutation-guided: LLM generates targeted mutants, a test is accepted only if it kills the mutant when executed; 9,095 mutants and 571 hardening tests across 10,795 classes. An LLM equivalent-mutant detector reached precision 0.79/recall 0.47 (0.95/0.96 with preprocessing).
*vs our finding: agrees.* Direct precedent for the "mutation-survived" evidence class: the kill is verified by execution, not by the model's say-so. Their equivalent-mutant problem is our false-block risk on a mutant that is semantically a no-op.

**SWE-bench Verified (OpenAI, Aug 2024) [vendor]** separates human judgment of task validity from the deterministic FAIL_TO_PASS / PASS_TO_PASS grader. **Anthropic "Demystifying evals for AI agents" (Jan 2026) [vendor]** prefers outcome/state checks over model grading and calls model graders non-deterministic. AutoCommenter ships comments, never a gate.
*vs our finding: agrees; not comparable on flip rates.* The industry pattern is "AI proposes, human or CI confirms"; I found no production tool that lets a model verdict block a merge directly.

## Finding A, Q3: repeat counts and reported flip rates

- Coin Flip Judge: 50 trials/item (most prior work <=5). One trial matches the 50-trial majority 86.6% of the time; 3 trials reach 90%, 11 reach 95%; hard items need 15 for 90% and >50 for 95%. Recommends 10-20 trials with majority vote.
- Rating Roulette: 3 runs, checked up to 5, no change in alpha; 61.3% same-verdict-across-3 on MTBench.
- Reliability without Validity: 118 runs, 21 judges; test-retest >0.95 for production judges on pairwise tasks.
- "Same Input, Different Scores" (Mar 2026, 11 judges) [preprint, via secondary]: same-verdict >95% at t=0, as low as 70% at t=1.
- Anthropic evals guidance [vendor]: report pass^k (consistency), not only pass@k.

*vs our finding: agrees; extends.* n=10 is the bottom of the recommended band and gives roughly +/-30 pp CI on 7/10, so 7/10 vs 6/10 is unresolvable while 2/10 vs 10/10 is. Our defect-item flip rates (30-60%) sit at or above Coin Flip's hard stratum (23.6%) and inside their 40-50% "danger zone".

## Finding B, Q4: does a strict schema degrade reasoning or detection?

**Let Me Speak Freely? (Tam et al., arXiv 2408.02442, Aug 2024; EMNLP 2024 Industry) [peer].** Reports large reasoning drops under format restriction (JSON-mode worst; e.g. ~27 pp on GSM8K in some settings), with near-zero parse failures, so the loss is in reasoning, not parsing.
**Say What You Mean (dottxt, Nov 2024) [blog, vendor-of-Outlines].** Replicates with Llama-3-8B: structured >= unstructured (GSM8K 0.78 vs 0.77, Last Letter 0.77 vs 0.73). Attributes the paper's losses to prompts that did not leave room for reasoning and to conflating JSON-mode with constrained generation.
**JSONSchemaBench (arXiv 2501.10868, Jan 2025) [preprint].** Benchmark and studies of constrained decoding; finds constraint engines differ in coverage and that well-implemented constraints do not systematically reduce downstream task quality.
**Capacity, Not Format (arXiv 2606.09410, Jun 2026) [preprint].** Information-matched controls, 4 models, 5 benchmarks, 0% parse failures. Strong models absorb JSON without loss (Sonnet 88.7% JSON vs 89.3% CoT on MATH-Hard); weak or budget-limited models drop 28-36 pp; penalty scales with schema complexity. Frontier is not immune: Opus 4.7 on AIME 96.2% to 91.0% under JSON. **Delayed structure (reason freely, then format) recovers most of the loss.**
**Anthropic structured-outputs docs (2026) [vendor].** Grammar applies only to the final output, not to thinking blocks or tool calls: "Grammar state resets between sections, allowing Claude to think freely while still producing structured output". Warns enum casing is not guaranteed; optional and union fields blow up grammar size (limits: 24 optional params, 16 union params).

*vs our prediction (no loss): agrees with caveats.* For an Opus-class, xhigh-effort, tool-using reviewer whose reasoning happens in thinking and tool calls *before* the constrained final message, the literature predicts negligible loss; this is the "think first, format later" remedy the Capacity paper validates. Caveats: (1) the 5 pp AIME drop shows frontier models still pay when the *final message* must carry reasoning, so keep a free-text `rationale` field alongside the enums; (2) **no source measures detection recall on multi-turn agentic review**; all are single-shot QA/math, so S-TYPED is a contribution, not a replication; (3) closed enum vs free text for classification: an enum removes label-paraphrase drift (dottxt, Anthropic docs), but I found no controlled recall comparison; inferred.

## Design implications we may be missing

- **Handle the "both fail" case explicitly.** Jin & Chen fall back to re-judging when the repro fails on both base and candidate; our deterministic gate needs a defined outcome (probably "evidence unverifiable, advisory only") rather than defaulting to block or pass.
- **Verify the repro reproduces *this* defect, not just "fails on base".** Agentless: 213 tests reproduced, only 94 passed the true fix. A repro command should be run on base (must fail) *and* on a known-good reference where one exists (must pass) before it can block.
- **Guard the mutation class against equivalent mutants.** ACH needed a dedicated detector (precision 0.79 raw). A "mutation-survived" block should require the mutant to be behaviourally distinguishable, or the finding degrades to advisory.
- **Stability is not validity.** A 10/10 approval on the clean item is consistent with a lenient judge; add a negative-control fixture with a subtle defect the rubric does *not* name, and audit rationale/verdict contradictions per Jin & Chen's NO_but_positive class.
- **Richer prompting shifts the boundary conservative.** Our xhigh-effort, rationale-required rubric is the regime with the highest false-rejection in the code-review studies. Since the verdict is gone, this mostly manifests as inflated *advisory* findings; budget for that noise in the UI rather than the gate.
- **Report pass^k with CIs, and stratify.** n=10 cannot distinguish 6/10 from 7/10; if any claim in the proposal depends on that difference, either raise n to 20-50 for the borderline fixtures or drop the claim. Expect coding-domain fixtures to sit in the high-flip stratum by default.

## Sources

- Rating Roulette: Self-Inconsistency in LLM-as-a-Judge Frameworks. Findings of EMNLP 2025. https://aclanthology.org/2025.findings-emnlp.1361/ ; https://arxiv.org/html/2510.27106v1
- The Coin Flip Judge? Reliability and Bias in LLM-as-a-Judge Evaluation. arXiv 2606.13685, Jun 2026. https://arxiv.org/html/2606.13685
- Reliability without Validity. arXiv 2606.19544, 17 Jun 2026. https://arxiv.org/abs/2606.19544
- Judging the Judges (Thakur et al.). arXiv 2406.12624, Jun 2024 (v3 Oct 2024). https://arxiv.org/html/2406.12624v3
- Large Language Models are Inconsistent and Biased Evaluators (Stureborg et al.). arXiv 2405.01724, May 2024. https://arxiv.org/abs/2405.01724
- Judge Reliability Harness (RAND). ICLR 2026 workshop; arXiv 2603.05399, Mar 2026. https://arxiv.org/abs/2603.05399
- Are LLMs Reliable Code Reviewers? Systematic Overcorrection in Requirement Conformance Judgement. Automated Software Engineering, 2026; arXiv 2603.00539. https://link.springer.com/article/10.1007/s10515-026-00638-5 ; https://arxiv.org/html/2603.00539v1
- Uncovering Systematic Failures of LLMs in Verifying Code Against NL Specifications. arXiv 2508.12358, Aug 2025. https://arxiv.org/abs/2508.12358
- Agentless: Demystifying LLM-based Software Engineering Agents. arXiv 2407.01489, Jul 2024; FSE 2025. https://arxiv.org/abs/2407.01489
- Mutation-Guided LLM-based Test Generation at Meta (ACH). FSE 2025 Industry; arXiv 2501.12862, Jan 2025. https://arxiv.org/pdf/2501.12862
- AI-Assisted Assessment of Coding Practices in Modern Code Review (Google AutoCommenter). FSE-Industry 2024; arXiv 2405.13565. https://arxiv.org/abs/2405.13565
- Introducing SWE-bench Verified. OpenAI, Aug 2024. https://openai.com/index/introducing-swe-bench-verified/
- Demystifying evals for AI agents. Anthropic Engineering, Jan 2026 (via secondary summaries). https://ai-eval.org/post/anthropic-demystifying-evals-for-ai-agents
- Let Me Speak Freely? (Tam et al.). arXiv 2408.02442, Aug 2024. https://arxiv.org/abs/2408.02442
- Say What You Mean: A Response to "Let Me Speak Freely". dottxt blog, Nov 2024. https://blog.dottxt.ai/say-what-you-mean.html
- JSONSchemaBench / Generating Structured Outputs from Language Models. arXiv 2501.10868, Jan 2025. https://arxiv.org/html/2501.10868v1
- Capacity, Not Format: Rethinking Structured Reasoning Failures. arXiv 2606.09410, 8 Jun 2026. https://arxiv.org/abs/2606.09410
- Structured outputs. Claude Platform Docs, accessed 2026-09-06. https://platform.claude.com/docs/en/build-with-claude/structured-outputs

---

# Part B — Prompt-cache anatomy under `claude -p`

(Research pass 2, verbatim.)

Evidence tags: **[doc]** first-party documentation, **[pub]** public issue/paper/repo, **[inf]** inferred.

## Q1. Prefix order, breakpoints, independence of tools vs system

**Sources.** Platform prompt-caching doc [doc]: "Cache prefixes are created in the following order: `tools`, `system`, then `messages`"; "Cache hits require 100% identical prompt segments … up to and including the block marked with cache control"; invalidation table: tool-definition change ✘ tools/system/messages; system-prompt change leaves tools ✓ and invalidates system+messages. Mechanism ("How automatic prefix checking works"): a `cache_control` marker writes **one entry, a hash of the prefix ending at that block**; reads walk back ≤20 positions looking for entries prior requests wrote. Tool-use caching doc [doc]: `cache_control` on the last tool "caches the entire tool-definitions prefix". Claude Code caching doc [doc]: layers are system prompt (core instructions + tool definitions + output style) → project context → conversation; "a change to the system prompt invalidates everything". LLM-gateway protocol doc [doc]: "Claude Code attaches `cache_control` markers to `system` blocks and to `messages` entries". Claude Code blog, 30 Apr 2026 [doc]: order is "Static system prompt & Tools (globally cached) → CLAUDE.md (cached within a project) → Session context (cached within a session) → Conversation". No first-party doc states the exact number of breakpoints Claude Code sets (API cap: 4).

**vs our measurement: consistent / explains it.** Tools precede system, and a tools-terminated entry is exactly what would produce a ~15.1k segment shared across any two `--system-prompt` values; the ~14.5k tail is the system-block entry whose hash includes the prompt text. Entries are prefix hashes at breakpoint positions, so tool and system caches are independent *only in the sense that the shorter entry survives*. [inf] Whether the 15.1k boundary is the last-tool marker or a static first system block cannot be distinguished from usage counts; the API's cache-diagnostics beta reports the exact divergence position.

## Q2. TTL, bucket selection, and what a read refreshes

**Sources.** Platform doc [doc]: default 5 min; "refreshed for no additional cost each time the cached content is used"; lifetime measured from the *start* of the request; 1h writes at 2× base input. Claude Code doc [doc]: on a subscription *within plan usage* the main conversation (including `-p` runs) gets 1h; subagents, compaction, titles 5m; once usage credits engage, main drops to 5m. Override precedence: `FORCE_PROMPT_CACHING_5M` → `CLAUDE_CODE_PROMPT_CACHE_TTL`/`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` → `promptCacheTtl`/`subagentPromptCacheTtl` → subagent `experimental.cacheTtl` → `ENABLE_PROMPT_CACHING_1H` → bucket default (v2.1.242+). Boris Cherny, #45756, 12 Apr 2026 [pub]: "Claude Code uses a 1 hour prompt cache window for the main agent." #46829 (closed not planned) [pub]: JSONL evidence of 1h-only Feb 1–Mar 5, then 5m-dominant from Mar 6–8 — the default has changed before without notice.

**Refresh scope.** No doc says "a read refreshes only up to the hit position" in those words, but it follows from the entry model: a request that diverges before the system breakpoint never looks up, hence never refreshes, the deeper entry. [inf, strongly implied]

**vs our measurement: explains it.** ~1h effective TTL matches the documented 1h main-conversation TTL with refresh-on-read. The "subsidy" is real under the entry model: any request in the same org/workspace with the same model, tool set and static system head refreshes the 15.1k entry, whereas a custom-prompt tail entry is refreshed only by requests carrying that exact prompt. The cache is org/workspace-scoped, not machine-scoped.

## Q3. Cross-directory / worktree sharing, git snapshot

**Sources.** Claude Code doc, "Cache scope" [doc]: the cache "is effectively scoped to one machine and directory. The system prompt embeds the working directory, platform, shell, OS version, and auto memory paths … That includes worktrees … Sequential sessions share the prefix only when the git status snapshot at startup matches". CLI reference [doc]: `--exclude-dynamic-system-prompt-sections` moves working directory, environment info, memory paths and git-repo flag into the first user message; "ignored when `--system-prompt` or `--system-prompt-file` is set." Agent SDK doc [doc]: TypeScript-only `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` splits a custom prompt into two blocks "each with its own cache breakpoint".

**vs our measurement: consistent.** Default prompt: ~7.8k per-directory segment and ~17.4k shared is exactly the documented split. Static `--system-prompt`: share 1.000 across commits and 10 worktrees is expected because the custom prompt omits the dynamic sections entirely. Caveat: the documented shared-across-machines path is the *default* prompt plus the exclude flag; our custom-prompt route achieves the same effect but forfeits the subsidised static head.

## Q4. Rate-limit accounting and TTL-change issues

**Sources.** API rate-limits doc [doc]: "only uncached input tokens count toward your ITPM" — the *API* limiter. Claude Code costs doc [doc]: with caching "Claude Code re-reads that history at the cached token rate, so a one-line question in a session that has been open all day still draws usage for the whole conversation" (reads *do* draw plan usage, at a discounted rate); cache misses flagged in `/usage` when ≥10% of recent usage. No first-party doc publishes the subscription coefficient for cache reads; #45756 (open) requests it.

**vs our measurement: consistent / partially unknown.** Whether a keep-warm read costs quota proportionally to tokens or per request is undocumented.

## Q5. Public measurements and keep-alive practice

**Sources.** TraceLab (arXiv 2606.30560, June 2026) [pub]: ~4,300 Claude Code/Codex sessions; prefix-cache hit rate 84.4% on user-initiated turns, 97.5% on tool-result turns. Platform doc, "Pre-warm the cache" [doc]: official heartbeat pattern is `max_tokens: 0` with an explicit breakpoint on the last shared block, at least every 5 minutes. `yujiachen-y/claude-code-cache-keepalive` [pub]: Stop-hook keepalive for 5m TTL; ~12 keepalives per avoided rewrite; author warns it is counter-productive on Pro/Max because each keepalive is a real turn against quota. Claude Code workflow fan-out doc [doc]: same-prefix agents held ≤5 s so followers read the leader's write.

**vs our measurement: consistent.** Our single-shot dispatches are a degenerate TraceLab case; the only sources of miss are TTL expiry and prefix drift.

## Design implications we may be missing

- A heartbeat only needs to refresh the entry we care about, and can come from anywhere in the account — but quota treatment of a keep-alive turn on a subscription is undocumented; measure before building.
- **Pin the TTL explicitly** (`CLAUDE_CODE_PROMPT_CACHE_TTL=1h`, v2.1.242+): the default silently drops to 5m once usage credits engage, and defaults have flipped server-side before.
- **Pin the Claude Code version for the dispatcher.** Any upgrade changes tool definitions and invalidates the shared head for every session in the account.
- **Move per-review content out of `--system-prompt` into the user message** so the tail is identical across reviews.
- **Consider the default prompt + `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt`** as an alternative: keeps the subsidised static head, drops the per-directory git snapshot, and only the appended tail is ours to keep warm.
- Don't change model or effort between dispatches (each has its own cache); use the API cache-diagnostics beta to locate the 15.1k/14.5k boundary exactly.

## Sources (Part B)

- https://platform.claude.com/docs/en/build-with-claude/prompt-caching (fetched 2026-09-06)
- https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-use-with-prompt-caching
- https://platform.claude.com/docs/en/api/rate-limits
- https://code.claude.com/docs/en/prompt-caching
- https://code.claude.com/docs/en/costs
- https://code.claude.com/docs/en/cli-reference
- https://code.claude.com/docs/en/agent-sdk/modifying-system-prompts
- https://code.claude.com/docs/en/llm-gateway-protocol
- https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything (2026-04-30)
- https://github.com/anthropics/claude-code/issues/46829 ; https://github.com/anthropics/claude-code/issues/45756
- https://arxiv.org/abs/2606.30560 ; https://syfi.cs.washington.edu/blog/2026-06-25-tracelab/
- https://github.com/yujiachen-y/claude-code-cache-keepalive

---

# Part C — Human gates, verified review, dead artifacts, state in git

(Research pass 3, verbatim.) Sources tagged [1P] first-party docs, [P] paper, [Pr] practitioner post. Retrieved 2026-09-06.

## Q1. Human-gate precedent

**Temporal** [1P]. `await workflow.wait_condition()` / signal handler; worker returns the task to the server and goes idle, state persisted as event history and replayed on resume. Answer delivered by Signal or Update (server-deduped by Update ID). Signals "may be duplicated" so handlers check status ("ignore if already decided"); durable timers escalate to a backup approver on SLA expiry. The answer is bound to a *workflow run*, not to a snapshot of the world.

**Restate** [1P]. `ctx.awakeable()` returns an ID + promise; invocation suspends; resolved via `POST /restate/awakeables/<id>/resolve`; timeout via `promise.orTimeout`, persisted. The awakeable ID is an unguessable single-use capability token.

**AWS Step Functions `.waitForTaskToken`** [1P]. A unique token is minted, the execution pauses, `SendTaskSuccess/Failure(token)` resumes; up to the 1-year cap; `HeartbeatSeconds` so abandoned waits fail rather than hang; token single-use.

**Azure Durable Functions** [1P]. `WaitForExternalEvent` + `CreateTimer` raced with `Task.WhenAny`.

**GitHub Actions environments** [1P]. Required reviewers pause the *job*; approval is a forge-side record; auto-fail after 30 days; wait-timer cap 43,200 min. The job re-runs *at the original SHA* — staleness handled by binding approval to a commit.

**Argo Workflows `suspend`** [1P]; **Prefect `pause_flow_run(wait_for_input=Model)`** [1P]; **Dagster**: HITL still an open request (#14681).

**Agent frameworks**: LangGraph `interrupt()` + `Command(resume=…)` — node re-runs from the top on resume, no built-in timeout; OpenAI Agents SDK `RunState` serialises approvals; Trigger.dev waitpoints `wait.createToken({timeout, idempotencyKey})`; Claude Agent SDK `canUseTool` is a **live** in-process prompt.

**Vs our design.** *Same pattern*: mark-and-resume with a single-use token is the dominant model. *Different*: every system binds the answer to an **identity** (run ID, token) and most add a **deadline** with an explicit expiry path. *Gap*: none bind approval to a *content hash*; GitHub pins to a SHA. We reconcile against git, which is stronger — but we need the token/deadline halves too.

## Q2. Verification-gated review

**Meta ACH** [P, FSE 2025]. Tests kept only if they *kill* a mutant that survives the existing suite; an LLM equivalent-mutant detector at precision 0.79/recall 0.47 raw.

**Agentless** [P]. Reproduction tests run on the unpatched repo; 213/300 reproduced but only 94 flipped under the ground-truth patch. **SWE-bench Verified** exists because FAIL_TO_PASS tests needed human validation.

**Cloudflare AI code review** [Pr, 2026-04-20]. Multi-model reviewer that *blocks merges* on critical findings; the coordinator reads source to verify; ~1.2 findings/review; 131,246 runs / 48,095 MRs in 30 days; humans "break glass" 288 times (0.6%). Findings marked "won't fix" are treated as resolved on re-review. Verification is read-based, not executed.

**Martian benchmark** [Pr/P, Feb 2026, 300k PRs]. Precision as "share of suggestions matched by post-review developer changes"; ~56% for the best tools; no vendor publishes a false-block rate.

**Google AutoCommenter** [P, 2024]. ~50% of comments absent from the submitted snapshot; explicit feedback on ~10%.

**Mutation in CI** [1P]: StrykerJS `--incremental`, cargo-mutants `--in-diff`. **Flaky quarantine** [1P GitLab]: allowed-to-fail lane with a fix deadline, never deleted.

**Vs our design.** *Same pattern*: ACH/Agentless are the precedent for "a finding must come with an executable check that fails now." *Different*: production AI reviewers verify by reading, not running; none require a reproducible check to block. *Gap*: the Agentless failure mode — a check that fails before but doesn't pass after — needs the "flips" criterion.

## Q3. Dead artifacts / ceremony

**Scott Logic on Spec Kit** [Pr, 2025-11-26]. 689 LOC alongside 2,577 lines of markdown; 3.5 h review vs 15 min; plan step alone 2,067 lines. **OpenSpec** [1P]: "no phase gates". **Agent OS v3** [1P]: implementation/orchestration phases retired. **Documentation debt** [P]: Ericsson study (arXiv 2402.11048) ~93% say most docs are incomplete or outdated. **No measured downstream-read data found**; Dagster has asset observation events but nothing that flags unread assets.

**Vs our design.** Our 110/110 unfilled `plan.md` is the empirical version of the critique; the field deleted phases rather than instrumenting them. *Gap*: record `read`/`cite` events per artifact and prune on evidence.

## Q4. State in git vs DB

**Gerrit NoteDb** [1P]: change metadata at `refs/changes/NN/NNNN/meta`; caches rebuilt by `RebuildNoteDb`. **Nesbitt, "Issues in the Repo"** [Pr, 2026-08-20]: files-in-tree fork issue state across branches; orphan branches get pruned; git notes orphan on rebase because the SHA changes; git-bug uses op-log commits under `refs/bugs/<id>` with Lamport clocks; git-native-issue carries state in **commit-message trailers**; Forgejo proposal: `refs/issues/*` canonical with SQL rebuilt as an index. **Jujutsu op log** [1P]: lock-free; divergent heads 3-way merged; conflicts stored *in the view*. **git-branchless** [1P]: `RewriteEvent(old→new)` for every amend/rebase. **Beads** [1P/issues]: `.beads/issues.jsonl` + SQLite cache; worktree redirect; rebase conflicts on JSONL (#4074); a `ClearDirtyIssues` race. **Backlog.md #558**: MCP wrote to the main repo instead of the worktree.

**Vs our design.** *Same pattern*: trailers-as-state + rebuildable cache. *Different*: git-bug/jj store *conflict* as a first-class value. *Gap*: trailers survive rebase (they travel in the message) but the SHA our log references does not; a branchless-style `old→new` rewrite map is needed.

## Design implications we may be missing (Part C)

1. Gate = (token, content-hash, deadline); expire with an explicit terminal path.
2. Reject stale answers, don't reconcile them.
3. Reviewer findings must "flip", not just "fail".
4. Quarantine, don't delete, disputed checks.
5. Publish false-block rate; log every block and override.
6. Instrument artifact reads; retire stages with zero reads over N runs.
7. Record commit rewrites so the log can re-anchor after history rewrite.
8. Worktree ownership must be explicit at startup.

## Sources (Part C)

- Temporal: https://temporal.io/blog/human-in-the-loop-approvals ; https://docs.temporal.io/guides/reliable-document-approvals ; https://docs.temporal.io/handling-messages
- Restate: https://docs.restate.dev/ai/patterns/human-in-the-loop
- Step Functions: https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html
- Azure Durable: https://learn.microsoft.com/en-us/azure/azure-functions/durable/durable-functions-phone-verification
- GitHub environments: https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments
- Argo: https://argo-workflows.readthedocs.io/en/latest/walk-through/suspending/ ; Prefect: https://docs.prefect.io/v3/advanced/interactive ; Dagster #14681
- LangGraph HITL; OpenAI Agents SDK: https://openai.github.io/openai-agents-js/guides/human-in-the-loop/ ; Trigger.dev: https://trigger.dev/docs/wait-for-token ; Claude Agent SDK permissions: https://platform.claude.com/docs/en/agent-sdk/permissions
- Meta ACH: https://arxiv.org/pdf/2501.12862 ; Agentless: https://arxiv.org/pdf/2407.01489 ; SWE-bench Verified: https://openai.com/index/introducing-swe-bench-verified/
- Cloudflare AI code review: https://blog.cloudflare.com/ai-code-review/ ; Martian benchmark: https://arxiv.org/pdf/2603.26130 ; AutoCommenter: https://arxiv.org/pdf/2405.13565
- Stryker incremental: https://stryker-mutator.io/docs/stryker-js/incremental/ ; cargo-mutants: https://mutants.rs/pr-diff.html ; GitLab quarantine: https://docs.gitlab.com/development/testing_guide/quarantining_tests/
- Scott Logic on Spec Kit: https://blog.scottlogic.com/2025/11/26/putting-spec-kit-through-its-paces-radical-idea-or-reinvented-waterfall.html ; OpenSpec: https://github.com/Fission-AI/openspec ; Agent OS v3: https://github.com/buildermethods/agent-os/discussions/310 ; Ericsson doc-debt: https://arxiv.org/html/2402.11048
- Nesbitt: https://nesbitt.io/2026/08/20/issues-in-the-repo.html ; Gerrit NoteDb: https://gerrit-review.googlesource.com/Documentation/note-db.html ; jj: https://docs.jj-vcs.dev/latest/technical/concurrency/ ; git-branchless: https://github.com/arxanas/git-branchless/wiki/Architecture ; Beads: https://github.com/steveyegge/beads/blob/main/docs/ADVANCED.md , https://github.com/gastownhall/beads/issues/4074 ; Backlog.md #558

---

# Part D — S-TYPED results against prior art (added 2026-09-07)

(Research pass 4, verbatim. Tags: [peer] · [preprint] · [1P] · [blog].)

## Q1 (R2): Is "≥52% of merged, reviewed fixes carry an unpinned criterion" plausible?

1. **Petrović, Ivanković, Fraser, Just — ICSE 2021** [peer]. ~15M mutants at Google, mutation integrated into code review: median **1 live mutant per changelist at review start, 0 at submit**; 70% of high-priority real bugs were coupled with a mutant that would have been reported during review. The review-start baseline is that a typical change has ≥1 surviving productive mutant even with aggressive filtering.
2. **Petrović et al. — TSE 2021** [peer]. Surviving mutants surfaced in review for 12.6% of changelists after heavy suppression; 82% rated productive. A floor, not a comparable rate.
3. **Inozemtseva & Holmes, ICSE 2014** [peer]. Coverage–effectiveness correlation drops to low/moderate once suite size is controlled. A green, well-covered suite says little about discrimination.
4. **LLM-test weakness (2025–26)**: Zhao, Zhou, Cohen ISSTA 2026 (arXiv 2607.22880) [peer]; misguidance-effect companion (buggy code cuts bug-revealing tests 8.5% → 3.0%); *Beyond Test Presence* (arXiv 2607.12068) [preprint]: 11.6% ambiguous assertions in agent tests vs 1.5% human; Meta ACH [peer].

**vs our result: extends / partially comparable.** No study reports our exact metric; Google's review-start median of one live mutant per changelist is directionally consistent with ~50% of changes carrying a surviving productive mutant. Ours is criterion-directed, so expect it higher than operator-based survival. Treat 52% as plausible-but-local.

## Q2 (R3): Cost/benefit of execution-based vs read-only review; routing

1. **Cloudflare, 2026** [1P]. 131,246 runs / 48,095 MRs / 30 days; mean $1.19, median $0.98, P99 $4.45, median 3 m 39 s; read-only; tiers trivial $0.20 / lite $0.67 / full $1.68.
2. **Anthropic Code Review, 2026-03** [1P]. Multi-agent with a verification step against actual code behaviour; **$15–25 per review, ~20 min**; no approve/block verdict.
3. **Google AutoCommenter, FSE-Industry 2024** [peer]. Read-only; usefulness plateaued at 54–60% until suppression lists.
4. **Are LLMs Reliable Code Reviewers? (arXiv 2603.00539)** [preprint/peer]. 87% of false rejections are unverifiable claims; executing the reviewer's own proposed change is the adjudicator.
5. **OpenCodeReview (arXiv 2608.09290)** [preprint]. Deterministic dispatch + bounded tools: 1.3–2.2× SEM-F1 at 5–15× fewer tokens than free-form agentic review.
6. Execution-grounded validation (arXiv 2604.10800) [preprint]: removing execution validation raised unnecessary repairs 131.7%.

**vs our result: agrees / extends.** $3.03 / ~18 min sits between Cloudflare's full tier and Anthropic's verification-heavy review; 147/147 runnable repros is stronger than anything published. Routing by size/risk is well precedented; a cheap-first cascade's recall loss is not measured anywhere.

## Q3 (R4): Label-boundary fuzziness with stable underlying detection

1. **Rating Roulette, EMNLP 2025** [peer]; 2. **C-SSRS 7-level severity, arXiv 2505.13480** [preprint] — misclassifications predominantly between adjacent levels; 3. **When Policies Change Probabilities, arXiv 2608.02677** [preprint] — asking the reviewer to estimate *and* decide shifts reported probability 13.6–16.9 pp; a modular pipeline (model reports evidence, code applies the rule) cuts loss; 4. *Automated Classification of Code Review Comments*, arXiv 2604.23667 [preprint] — evidence-sensitive labels fail without verification signals.

**vs our result: agrees.** Derive the class deterministically from the repro record rather than asking the model to choose.

## Q4 (R1): Do schemas preserve agentic performance?

1. **Anthropic structured-outputs docs** [1P]: grammars apply only to the final output, not tool calls or thinking; ≤24 optional / ≤16 union params; prefer required, flat, small enums; enum casing not guaranteed.
2. **Constraint Tax, arXiv 2606.25605** [preprint]: on open-weight models under grammar-constrained decoding, tools + JSON schema together → 0% tool invocation; two-pass execution restores it. Hosted Claude unaffected.
3. *Let Me Speak Freely?* EMNLP-Industry 2024 [peer]; OpenCodeReview and Cloudflare use closed schemas in production without reported loss.

**vs our result: agrees for hosted Claude; not comparable for open-weight** — a real risk if the reviewer is ever ported to a self-hosted stack.

## Design implications (Part D)

- Compute the evidence class in code from the repro record; keep the model's label advisory.
- Route by risk; measure a cascade's recall loss before adopting it.
- Retire "reviewed and green" as evidence of pinning; require a discriminating test per AC.
- Keep the schema flat, required-first, small enums; compare enum values case-insensitively.
- Two-pass (tools, then schema) if ever self-hosting open-weight models.
- Report cost against the Cloudflare ($1.68) and Anthropic ($15–25) anchors.

## Sources (Part D)

- Petrović et al., ICSE 2021: https://arxiv.org/abs/2103.07189 ; TSE 2021: https://arxiv.org/pdf/2102.11378
- Inozemtseva & Holmes, ICSE 2014: https://cs.uwaterloo.ca/~rtholmes/papers/icse_2014_inozemtseva.pdf
- Zhao, Zhou, Cohen, ISSTA 2026: https://arxiv.org/abs/2607.22880 ; misguidance paper: https://www.eecg.toronto.edu/~shuruiz/forcolab/paper/issta26-b.pdf
- Beyond Test Presence: https://arxiv.org/html/2607.12068v1 ; Meta ACH: https://arxiv.org/abs/2501.12862
- Cloudflare: https://blog.cloudflare.com/ai-code-review/ ; Anthropic Code Review: https://claude.com/blog/code-review ; https://code.claude.com/docs/en/code-review
- AutoCommenter: https://doi.org/10.1145/3664646.3665664 ; Are LLMs Reliable Code Reviewers?: https://arxiv.org/html/2603.00539v1
- OpenCodeReview: https://arxiv.org/html/2608.09290 ; execution-grounded validation: https://www.arxiv.org/pdf/2604.10800
- Rating Roulette: https://arxiv.org/abs/2510.27106 ; C-SSRS: https://arxiv.org/abs/2505.13480 ; When Policies Change Probabilities: https://arxiv.org/html/2608.02677v1 ; code-review comment classification: https://arxiv.org/html/2604.23667v1
- Anthropic structured outputs: https://platform.claude.com/docs/en/build-with-claude/structured-outputs ; Constraint Tax: https://arxiv.org/abs/2606.25605 ; Let Me Speak Freely?: https://aclanthology.org/2024.emnlp-industry.91.pdf
