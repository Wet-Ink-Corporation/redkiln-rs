## F38 — P8 PARTIAL: a resume into a mid-graph gate is **lossless across a real process kill** — 3 of 3 runs, every step-output digest and the whole state digest identical after a `taskkill /T /F` destroyed a process holding its state in memory — and costs **0.45–0.78 × F28's stage-gate price** (8,408 / 10,314 / 14,579 output tokens against ~18.6k), so neither of §5.4's first two criteria fires. Its third is **not answerable**: the gate never fired on its own in any run, so every arrival was forced, and the "second arrival mints a new mark" question was tested against the driver that implements it

**Status: PARTIAL.** METHOD-P §5.2 pre-registers 3 items × 3 delays × 3 repeats = 27 runs plus 3 no-kill controls. **3 runs exist**, all in the 0-delay arm; the 65-minute and 24-hour arms and the controls were not run. Everything below rests on n=3.

**Established by:** 3 suspend/kill/answer/resume cycles over `spikes/p9/catalog/minimal-repro.toml` (`01e17f07f1a6`), 2026-09-08, haiku, corpus pin `47faec9f`, `claude` 2.1.263. 10 `attempt` dispatches, **$3.01** of a $40 cap. `spikes/results/p8.jsonl`, state under `spikes/results/p8-state/`. Pre-registered in `METHOD-P.md` §5. Driver: `spikes/runner` subcommand `repro-probe` (`src/p8.rs`).

**Citation correction.** The AFK brief says "SIGTERM (F21 semantics)". `FINDINGS.md:978` **F21 is the E3 arm (b) detection result**, not process semantics. The suspension and process-death work is **F19** (lossless resume at activity granularity), **F22** (background jobs die; a detached process does not) and `runner/src/human_gate.rs`. Windows has no SIGTERM: `taskkill /T /F` terminates the tree, `taskkill /T` posts WM_CLOSE. **All three runs used `/T /F`** — the destructive form (DECISIONS-2026-09-09 #24).

### Metrics against the kill criteria (METHOD-P §5.4)

| criterion | measured | verdict |
| --- | --- | --- |
| **Any lost or changed step output** → escalation becomes activity-terminal; §3's "resume into the same step" is withdrawn | **0 changed, 0 lost, 3/3 runs.** State digest identical before and after in every case | **does not fire** |
| **Resume cost > 1.5 × F28's ~18.6k** → a mid-graph gate is priced like a stage gate plus the graph's replay | **0.45× / 0.55× / 0.78×** (8,408 / 10,314 / 14,579 output tokens) | **does not fire** |
| **Second arrival does not mint a new mark** → §9b's A8 reversed, gate gains an explicit `rearm` flag | the gate's own predicate was **false on every second arrival**; arrivals were forced, and the mark was minted by the driver under test | **NOT ANSWERABLE** (§3) |

### 1. Resume is lossless across a real kill [measured]

The kill destroys a process that is **blocking with its state in memory**, and the resume is a **separate process invocation** that must rebuild every step output from disk. Reading state back in-process would prove nothing — the risk is that a *fresh* process reconstructs something different, which is why `human_gate::round_trip` crosses a genuine boundary (F19), and why this driver does too.

| item | gap | steps | changed | lost | state digest before → after |
| --- | --- | --- | --- | --- | --- |
| `issue-74-plan-briefs-dead-plan-anchor` | 0.9 min | 1 | — | — | `3633092d…` → `3633092d…` |
| `issue-79-plan-prds-verifier-false-pass` | 28.4 min | 1 | — | — | identical |
| `issue-81-plan-discover-spec-gate-mismatch` | 29.1 min | 1 | — | — | identical |

Independently, the state **files** were sha256'd immediately before and after the `taskkill`: `6931E4D5…`, `0E24AA82…`, `CB1A79D1…`, all unchanged. So the loss, had there been any, could not have been hidden in the file.

**This extends F19 rather than repeating it.** F19 proved lossless resume at *activity* granularity; §5.1's open question was whether that holds with **partial step outputs bound mid-graph**. It does, on this evidence — with the large caveat in §4 that the state being round-tripped holds exactly one step.

### 2. A mid-graph resume is cheaper than a stage gate, not dearer [measured]

| item | resume output tokens | ÷ F28's ~18.6k |
| --- | --- | --- |
| `issue-81` | 8,408 | **0.45×** |
| `issue-74` | 10,314 | **0.55×** |
| `issue-79` | 14,579 | **0.78×** |

§5.4's second criterion anticipated a mid-graph gate costing *more* — "like a stage gate plus the graph's replay". Measured, it costs **less than a stage gate**, because the resumed step is a single `attempt` dispatch carrying the gate answer, and nothing is replayed: the prior step outputs are read from disk, not recomputed. The replay term the criterion assumed is absent from this design.

Wall clock from the answer to the first post-resume dispatch, computed from the recorded `answered_at`: **6.8 s, 114.5 s, 118.2 s**. The two long ones are process start plus fixture rebuild, not model latency.

### 3. Why the second-arrival question is not answerable here [measured, then reasoned]

P9b's A8 assumes a re-reached gate mints a **new** mark. Both corrected runs report `second_arrival_reached: true` and `minted: true`. **That is not evidence for A8**, for two compounding reasons:

1. **The gate's own predicate was false.** `post_resume_attempt_status` is `reproduced` on both, so `when = "${attempt.outputs.status} != 'reproduced'"` did not hold. The second arrival happened because `--force-gate` was in effect, not because the graph reached the gate.
2. **The mark was minted by the driver under test.** `repro-probe` implements A8 by construction — it writes a second `triage-gate` record on re-arrival because that is how it was written. Measuring A8 against an implementation that assumes A8 is circular. There is no kernel here to disagree with it; Layer 0 is spike apparatus, not a product.

So §5.4's third criterion is reported as **not answerable**, not as "does not fire". Answering it needs a gate that fires on its own and a kernel that decides the mark independently of the driver.

### 4. The fixture design in §5.2 does not work [measured] — and this is P8's other real finding

§5.2's fixture is "3 E3 items with the repro steps deliberately removed **so the gate fires**". Across three redactions of increasing severity and two selection rules, the gate **never fired**:

| fixture construction | result |
| --- | --- |
| repro section withheld (control run) | **reproduced**, attempt 1, 754 s |
| keep-list — title + problem statement only (#28) | **reproduced**, attempt 1, 650 s |
| items reselected to P7's three hardest (#35) | **reproduced**, attempt 1, 480 s |

The sharpest form of it: `issue-74-plan-briefs-dead-plan-anchor` was localised **0 times in 6** Opus attempts in P7, and **reproduced on the first haiku attempt** in P8. An investigator holding `Read` and `Bash` in a real repository reconstructs a reproduction from the symptom alone; the steps in the report are not what it was using.

**Reproducing a symptom and localising its cause are different tasks with very different difficulty, and §5.2 assumed the harder one gates the easier.** Every gate arrival in this spike is therefore forced (#36), and the *arrival rate* — how often a real triage gate opens in practice — is entirely unmeasured.

### 5. What this settles

- **Settles (n=3):** serialise → kill → reload is lossless at mid-graph granularity, across a destructive process kill, in a fresh process. §5.4's first criterion does not fire, and it is the criterion that would have withdrawn §3's "resume into the same step".
- **Settles (n=3):** a mid-graph resume is priced **below** a stage gate, not above. The second criterion does not fire, and the "plus the graph's replay" term it assumed does not exist in this design.
- **Settles:** "remove the repro steps so the gate fires" is not a workable fixture construction against an agent with repository access (§4).
- **Does not settle:** A8 / the `rearm` question (§3).
- **Does not settle:** anything about delay. All three runs sat inside F10's ~1 h cache TTL (0.9–29.1 min). The arms that straddle it were not run.
- **Does not settle:** resume cost against a **no-kill control**. §5.2's 3 controls were not run, so the resume figures have no within-experiment baseline and are compared only to F28.

### Limits

1. **n=3, one arm.** The 65-minute and 24-hour arms and the 3 no-kill controls were not run. §5.2 pre-registers 27+3.
2. **The arm label is not the measurement.** Records are labelled `0-delay` but the measured `gap_minutes` are **0.9, 28.4 and 29.1** — the two long ones because the cycle was driven manually after the orchestrator stalled. Read `gap_minutes`, never the arm name. All three are still inside the ~1 h TTL, so none of them tests the cache-cold path the delay arms exist for.
3. **Every gate arrival was forced** (#36); no gate fired on its own condition in any run.
4. **The suspended state holds exactly one step** (`attempt#1`). §5.1's concern was "partial step outputs bound mid-graph", and a one-step state is the thinnest possible instance of that. `minimal-repro` gates immediately after `attempt`, so this is inherent to the graph, not a choice — but a richer state would be a stronger test and has not been run.
5. **The driver is the thing measured.** There is no kernel; `repro-probe` implements the suspend/resume semantics it is being used to check. That is legitimate for "is this approach lossless" and circular for "does the kernel behave as A8 assumes" (§3).
6. **Haiku only**, per §5.5.
7. **The first emit record was written by a defective instrument** and is superseded, not deleted (the results file is append-only). Its `second_arrival_minted_a_new_mark: false` conflated "gate not re-reached" with "re-reached and minted nothing", its `gate_forced` read the resume invocation's flag rather than the run's, and its `ms_from_answer_to_first_post_resume_dispatch` (686,454) timed the dispatch instead of the latency to its start. All three are fixed (#37); **that record's `lossless` and token figures are unaffected and stand**.
8. **`taskkill /T /F` only.** The polite WM_CLOSE form is a different experiment and was not run.
