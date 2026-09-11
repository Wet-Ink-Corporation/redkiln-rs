---
id: "0087"
title: A mid-graph gate resumes into the same step, and costs less than a stage gate
status: accepted
date: 2026-09-08
legacy_id: none
source: spikes/F38-DRAFT.md (P8, PARTIAL); spikes/METHOD-P.md §5.2/§5.4; spikes/DECISIONS-2026-09-09.md #24, #28, #35, #36
supersedes: []
superseded_by: ["0106"]
related: ["0012", "0017", "0038", "0045", "0062"]
evidence: ["F38", "F19", "F28", "F22", "F10"]
consumers: []
---

## Context and problem

An activity graph may reach a human gate part-way through, with step outputs already bound. Two questions had to be settled before the design could commit to that shape. First, **is it safe** — when the process holding those bound outputs dies and a fresh process reloads from disk, does it reconstruct the same state, or is a mid-graph suspension lossy in a way an activity-granularity suspension is not? F19 established lossless resume at *activity* granularity (174/174, drift 7%), but explicitly did not cover partial step outputs bound mid-graph. Second, **what does it cost** — the pre-registration assumed a mid-graph gate would be priced "like a stage gate plus the graph's replay", i.e. above F28's measured ~18.6k output tokens, and registered a kill at >1.5× that: if it fired, escalation would stop being a suspension and become **activity-terminal**, discarding the partial work and re-entering the activity from the top on the human's answer.

P8 ran to answer both. It is a **partial** spike — 3 runs of a pre-registered 27+3 — and it gates "design final", so its shortfall has to be recorded with the same weight as its result.

## Decision drivers

- F19's lossless result was at activity granularity and its own limit statement excludes the mid-graph case, so the safety question could not be inherited.
- The cost question is a value decision about gate *placement*: if a mid-graph gate were dearer than a stage gate, the design would be pushed toward coarse, stage-boundary-only escalation regardless of where the human judgement actually belongs.
- Nothing may depend on a process surviving (F22): the resume path had to be measured across a genuinely destructive kill in a fresh process, not by reading state back in-process, which would prove nothing.
- Activity-terminal escalation discards work already paid for and re-runs it, which is the expensive fallback the kill criterion exists to force only if suspension is unsafe.

## Considered options

### Option — escalation is activity-terminal

On reaching a gate, terminate the activity, and re-enter it from the top when the human answers, binding the answer as an input. This is what METHOD-P §5.4's first criterion would have forced if any step output were lost or changed across the kill. It loses on measurement: **0 changed, 0 lost across 3 of 3 runs**, so the safety argument for it does not exist, and it would discard bound step outputs and pay to recompute them for no gain.

### Option — resume into the same step, but price it as a stage gate plus replay

Keep the suspension but assume the resume replays the graph up to the gate, and budget accordingly (the pre-registered assumption behind §5.4's 1.5× kill). It loses because the replay term does not exist in this design: the resumed step is a single `attempt` dispatch carrying the gate answer, and prior step outputs are **read from disk, not recomputed**. Measured resume cost is 0.45×/0.55×/0.78× of a stage gate, not >1.5×.

### Option — hold the process open across the gate

Avoid the reload question by keeping the blocking process alive until the answer arrives. Rejected on prior evidence, not on P8: F22 measured ~12 killed background jobs against one 12,000 s survivor, and F18 measured 45–48% of human gates exceeding an hour with a maximum of 9.4 days. Nothing may depend on a process surviving; every wait is a durable mark plus a later read.

## Decision

**Escalation to a human gate suspends the activity graph and resumes into the same step with the gate answer bound. It is not activity-terminal.**

1. On reaching a gate the graph serialises its bound step outputs to disk and the process may die. On the answer, a **fresh process** reloads that state and continues at the step the gate suspended, with the answer bound as that step's gate input. Prior step outputs are read from disk; nothing is replayed and nothing is recomputed.
2. Resume must cross a real process boundary. A reload that reads state back in-process is not a valid implementation of this decision and is not evidence for it, because the risk being managed is that a *fresh* process reconstructs something different.
3. Gate placement is therefore a value decision, not a cost decision: a mid-graph gate is not to be avoided on the grounds that it is dearer than a stage gate. Measured, it is cheaper.

**What P8 did not settle, at equal prominence.** This decision rests on a partial spike, and three of its questions are open in ways that matter:

4. **The second-arrival question is not answerable from this spike, and is not to be reported as "does not fire".** METHOD-P §5.4's third criterion asks whether a re-reached gate mints a new mark (P9b's assumption A8). Both corrected runs report `second_arrival_reached: true` and `minted: true`, and **neither is evidence**: the gate's own predicate was false on every second arrival (`post_resume_attempt_status` was `reproduced`, so the `when` did not hold) and the arrival happened only because `--force-gate` was in effect; and the mark was minted by `repro-probe`, the driver that implements A8 by construction. Measuring A8 against an implementation that assumes A8 is circular. Answering it needs a gate that fires on its own condition and a kernel that decides the mark independently of the driver. Until then A8 stands unverified and no `rearm` flag is added or ruled out.
5. **Nothing about delay is established.** All three runs sat inside F10's ~1 h cache TTL — measured `gap_minutes` of **0.9, 28.4 and 29.1**, notwithstanding the `0-delay` arm label, because two cycles were driven manually after the orchestrator stalled. The arm label is not the measurement; read `gap_minutes`. The 65-minute and 24-hour arms — the ones that straddle the TTL and test the cache-cold resume path — and all three no-kill controls were **not run**, so the resume cost figures also have no within-experiment baseline and are compared only to F28.
6. **The fixture design in §5.2 failed, and that is P8's other real finding.** "Remove the repro steps so the gate fires" does not work against an agent holding `Read` and `Bash` in a real repository: across three redactions of increasing severity and two selection rules the gate **never fired** (reproduced on attempt 1 at 754 s, 650 s and 480 s). The sharpest form: `issue-74-plan-briefs-dead-plan-anchor` was localised **0 times in 6** Opus attempts in P7 and **reproduced on the first haiku attempt** in P8. **Reproducing a symptom and localising its cause are different tasks with different difficulty**, and §5.2 assumed the harder one gates the easier. Every gate arrival in this spike was therefore forced, and the *arrival rate* — how often a triage gate opens in practice — is entirely unmeasured.

**What would change this decision.** Any of: a lost or changed step output on a state holding more than one step; a resume cost above 1.5× F28's ~18.6k on the unrun 65-minute or 24-hour arms; or a second-arrival result from a gate that fires on its own predicate against a kernel that mints the mark independently. Each is a registered spike arm, not a hypothetical, and each is named here per 0012.

## Consequences

**Positive**
- Bound step outputs survive a destructive process kill and a fresh-process reload, so a human gate placed mid-graph costs no recomputation of work already done.
- Gate placement is freed to follow where the judgement belongs rather than where the cost model pushes it: at 0.45×–0.78× of a stage gate, a mid-graph gate is the cheaper of the two.
- Extends F19 rather than repeating it — the open question was mid-graph partial state, and it is answered on this evidence at that granularity.
- Removes the "plus the graph's replay" term from the cost model, which the pre-registration had assumed into existence and which does not exist in a design that reads prior outputs from disk.

**Negative**
- The decision rests on n=3, one arm, one item family, one model (haiku), one corpus pin, on Windows with `taskkill /T /F`. It is a thin base for a decision that gates "design final".
- The suspended state in every measured run held **exactly one step** (`attempt#1`) — the thinnest possible instance of "partial step outputs bound mid-graph". `minimal-repro` gates immediately after `attempt`, so this is inherent to the graph rather than a choice, but a richer state would be a stronger test and has not been run.
- The cache-cold resume path is unmeasured, and it is the expensive one: a resume after a gate longer than the TTL is the pattern the design institutionalises, and P8 did not reach it.
- The design still has no measurement of how often a triage gate opens on its own, so the frequency term in "duration is free, frequency is marginally costly" is unpriced for this gate kind.

**What this forbids**
- Forbids making escalation activity-terminal, or discarding bound step outputs on reaching a gate.
- Forbids an implementation whose resume reads state back in-process rather than across a real process boundary.
- Forbids keeping a process alive across a human gate, or making correctness depend on one surviving.
- Forbids avoiding a mid-graph gate on the stated grounds that it costs more than a stage gate — measured, it costs less.
- Forbids citing P8 as evidence about **delay**, cache-cold resume, or the no-kill baseline: those arms were not run.
- Forbids reporting §5.4's third criterion as "does not fire", and forbids citing `second_arrival_reached` / `minted` from these runs as support for P9b's A8 — the arrivals were forced and the mark was minted by the driver under test.
- Forbids reading a P8 record's arm label as its delay; only `gap_minutes` is the measurement.
- Forbids reusing the §5.2 fixture construction ("remove the repro steps so the gate fires") against an agent with repository read and execution access.

## Evidence and confidence

- [measured] **F38 §1, losslessness, n=3.** Three suspend/kill/answer/resume cycles over `spikes/p9/catalog/minimal-repro.toml`, 2026-09-08, haiku, corpus pin `47faec9f`, `claude` 2.1.263, 10 `attempt` dispatches, $3.01 of a $40 cap. **0 changed, 0 lost, 3/3 runs**; state digest identical before and after in every case (`3633092d…` → `3633092d…` and two identical); state *files* independently sha256'd immediately before and after the kill (`6931E4D5…`, `0E24AA82…`, `CB1A79D1…`) all unchanged. The kill was `taskkill /T /F` — the destructive form, terminating the tree; Windows has no SIGTERM. §5.4's first criterion **does not fire**, and it is the criterion that would have withdrawn "resume into the same step".
- [measured] **F38 §2, cost, n=3.** Resume output tokens **8,408 / 10,314 / 14,579** = **0.45× / 0.55× / 0.78×** of F28's ~18.6k. §5.4's second criterion (kill at >1.5×) **does not fire**. Wall clock from answer to first post-resume dispatch 6.8 s / 114.5 s / 118.2 s, the two long ones process start plus fixture rebuild rather than model latency.
- [measured, then reasoned] **F38 §3, second arrival — NOT ANSWERABLE.** Reported as not answerable, not as "does not fire". The gate predicate `when = "${attempt.outputs.status} != 'reproduced'"` was false on every second arrival; arrival was forced by `--force-gate`; and `repro-probe` writes the second `triage-gate` record because it implements A8 by construction. There is no kernel present to disagree with the driver — Layer 0 is spike apparatus, not the product.
- [measured] **F38 §4, fixture failure.** Three fixture constructions, gate never fired: repro section withheld → reproduced, attempt 1, 754 s; keep-list title + problem statement only → reproduced, attempt 1, 650 s; items reselected to P7's three hardest → reproduced, attempt 1, 480 s. `issue-74` localised 0/6 by Opus in P7, reproduced first-attempt by haiku in P8.
- [measured] **Prior evidence this rests on.** F19: 174/174 lossless suspensions at activity granularity, drift 12/174 = 7%, 3% under an hour vs 11% over (3.6×) — with its own limit that the FSM was never replayed, so "same next action" is unproven. F28: a human gate costs a bounded ~18.6k output tokens whether it lasts 65.2 minutes or 49.7 hours, identical to the token, with the published correction that the flatness is shared-prefix subsidy rather than durability. F22: background jobs die (~12 kills vs one 12,000 s survivor); cause still unestablished. F10: prompt-cache TTL ~1 h, boundary bracketed at (45.1, 61.6) minutes and never located, n=1 per gap, date-stamped platform behaviour.
- [documented] The pre-registration itself is recorded as broken in two places: METHOD-P §5.2's fixture design and §5.4(3)'s circularity are both listed among the programme's broken pre-registrations. This record acts on the broken finding, not on the registered intent.
- **Corpus vs external.** The external pass adds three P8-relevant items the spike did not control for (evidence-augmentation §4 rec 24, §3 context/caching): subagent and workflow traffic sits in a **5 m** TTL bucket by default regardless of tier unless `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` is also set; workflow fan-outs stagger identical-prefix agents by up to 5 seconds by default, a dispatch-timing confound; and two worktrees of one repository build different prefixes and never share a cache, so a per-worktree cold first turn is structural rather than a rig defect. **The external reading is acted on**: it reinforces clause 5 — the delay arms were not merely unrun, they would also have needed billing mode and the subagent TTL variable recorded as confounds to be interpretable. None of it disturbs clauses 1–3, which are token counts and digests, not cache-hit rates.
- **What this evidence does not license.** It does not license any claim about resume across the TTL boundary, resume against a no-kill control, or the arrival rate of a triage gate — those arms were not run. It does not license generalising losslessness beyond a one-step suspended state, beyond haiku, or beyond Windows `taskkill /T /F`. It does not license treating 0.45×–0.78× as a resume price for a richer graph: the ratio is against F28's stage-gate figure, not against a within-experiment baseline, and F28's own flatness was corrected to a shared-prefix subsidy. It does not license any statement about A8 or a `rearm` flag in either direction. n=3 throughout, one operator, one corpus pin.
