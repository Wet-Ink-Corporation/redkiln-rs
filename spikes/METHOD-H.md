# Pre-registration #3 — H: the human gate

**Status:** H2/H3 reported (F19). **H1 and H4 approved and running from
2026-09-04.** Written before any H dispatch.
**Corpus pin:** redkiln @ `47faec9f`. **Instrument:** `spikes/runner`.
**Why it exists:** RFC §7 identifies the mid-run human approval gate as the
*unoccupied market gap* — the capability that would justify building
Redkiln-RS at all — and neither the RFC nor the charter tests it. It is the
largest untested assumption in the thesis.

---

## 1. The framing that makes this testable

The human gate is not a UX feature. **It is a suspend/resume across an
unbounded, human-scale gap, and everything hard about it is a property of that
gap.** Spiking "can a Rust process wait for input" would be ceremonial — it
obviously can. What is uncertain is what the *gap* does to cost, to correctness,
and to the world.

**We already know the gap's shape**, measured from 174–191 historical gate
pauses (F18):

| anchor | n | median | **> 1 h (the cache TTL)** |
| --- | --- | --- | --- |
| `gate_result` → `verdict` | 174 | 10.5 min | **45%** |
| `stage_enter` → `verdict` | 191 | 51.6 min | **48%** |

Longest observed: **226 hours** (9.4 days). The distribution is not "humans are
slow" — it is **humans are fast, or they are gone**. Roughly half of all gates
land the resume on a cold prefix (F10: TTL ~1 h, boundary 45.1–61.6 min).

That single fact drives the whole design below.

---

## 2. What is deliberately NOT spiked

- **The UX.** How a human is asked and answers (CLI, file, notification, web) is
  a product decision, not an uncertainty.
- **Whether a process can suspend.** Trivially yes.
- **Multi-machine resume.** `MULTI-HARNESS.md` is right that "in a multi-worker
  fleet, local disk is a lie", but that is a Bet-2-adjacent question and one
  harness is in scope.
- **Whether gates are *desirable*.** Charter §2 already marshals evidence both
  ways. This measures feasibility and cost, not worth.

---

## 3. H1 — What does one suspension cost?

**Question.** When a gate outlives the cache TTL — which happens ~half the time
— what does the resume cost at **review scale**?

**Why it is not already answered.** F10 and F16 measured a ~25k trivial prefix.
A real review carries a spec, an implementation report and a diff. The cold-write
cost scales with that, and nothing has measured it.

**Procedure.** Build a review-scale prefix from the E3 corpus (spec + report +
diff for a real item). Warm it, then read it back at gaps drawn from the observed
human distribution: **5 min (p25-ish), 65 min (median-crossing), 8 h (p90)**.
Reuse `ttl-mark` / `ttl-read` — the gap is two timestamps, so nothing must
survive it.

**Primary metric.** `cache_creation` tokens on resume, by gap.
**Secondary.** The same under `--system-prompt`, which held 1.000 across commits
(F16) — if it also blunts the *time* decay, the gate becomes near-free and that
is a design conclusion, not a detail.

**Decision rule.** A per-gate cost under ~25k tokens is noise; batch gates
freely. Materially more, and gate placement becomes a cost decision — favour
fewer, later gates, and prefer `--system-prompt`.

**Cost.** ~8 dispatches plus wall clock. Cheap.

---

## 4. H2 — Can resume be correct across a process boundary? ← the gate

**Question.** After suspending, killing the orchestrator, and resuming from a
cold process, does the run take **the same next action** it would have taken
contiguously?

**Why it decides something.** This is where the `--stay` class of bug lives.
RFC §3.3: omitting `--stay` walks a project irreversibly past its own review
gate into a state where "`validate` and `doctor` both report clean" and no CLI
verb moves it back. Today that is guarded by a paragraph of prose. **If resume
across a real process boundary is not deterministic, the human gate is unsafe at
any cost**, and the RFC's central enforcement claim — that inversion makes the
failure unrepresentable — is false.

**Procedure.** Zero model cost. For each of N recorded historical gate answers:
reconstruct the pre-gate state, suspend, terminate the process, resume in a
fresh one, and compare the computed next action against the contiguous run.
`reconcile()` from S0.2 supplies the state; the human answer is replayed from
the `verdict` event.

**Primary metric.** Divergences between contiguous and suspended next-action,
enumerated with cause. Target is zero; **any divergence is a finding**, and one
that would have been a silent corruption in the current system.

**Decision rule.** Non-zero divergence bounds how much of the gate can be
trusted to the orchestrator versus needing an external authority (shipmates
L1.3: keep CI and the deterministic gates independently authoritative).

---

## 5. H3 — Does the world drift across the gap?

**Question.** Between a gate opening and a human answering, does the repository
change underneath the paused run?

**Why.** shipmates L2.2 — "recorded state is a cache; git is truth" — was written
for machine-timescale gaps. At a **median 10.5 minutes but a p90 of ~9 hours and
a maximum of 9.4 days**, drift stops being hypothetical. A resume that trusts its
own recorded state will act on a world that has moved.

**Procedure.** Zero model cost, entirely from the frozen corpus. For each of the
174 measurable gate pauses, count commits landing on the relevant refs inside the
gap, and classify: none / unrelated / **touches the item's own files**.

**Primary metric.** Share of gate pauses with in-gap commits touching the paused
item's files.

**Decision rule.** Sets the required strength of `reconcile()` on resume. If
material, resume must re-derive completion from the world (shipmates L2.4) rather
than trusting the record — and that is a design requirement, not an optimisation.

---

## 6. H4 — Does the agent lose anything across the gap?

**Question.** Does a step after a suspension produce the same output as the same
step run contiguously?

**Why.** RFC R1 (context fidelity) at the gate boundary. Under inversion, all a
resumed step receives is the prior step's digest plus reconstructed state. If
that is insufficient, the gate is not free even when the cache is warm.

**Procedure.** A two-step sequence on identical inputs, run (a) contiguously and
(b) with a suspend/terminate/resume between the steps. Compare step-2 outputs.
Paired, n from E0's per-item stability once it exists.

**Decision rule.** Not a kill — a constraint on what a step's digest must carry.
A divergence tells us the digest schema is under-specified, which is fixable.

### Amended 2026-09-04, before any H4 dispatch — the registered comparison is vacuous

**The architecture has no in-process continuity to lose.** Every step under
inversion is its own `claude -p` process (`runner.rs:267` spawns one per
dispatch), so a step after a suspension and a step run "contiguously" receive
*exactly the same thing*: the prompt the orchestrator hands it. Running the
registered A/B would return "no divergence" for a reason that has nothing to do
with the gate, and reporting that as evidence the gate is free would be a
measurement artefact.

**The intent survives and is testable.** §6's stated why is digest sufficiency —
"all a resumed step receives is the prior step's digest plus reconstructed
state. If that is insufficient, the gate is not free even when the cache is
warm." That is the real question, and the suspend/resume mechanics were only ever
a proxy for it.

**Replacement procedure.** A second step consumes a *real* step-1 review output
from the E3 corpus, under two conditions:

| arm | what step 2 receives |
| --- | --- |
| **DIGEST** | the structured digest alone — `issues[]` with `issue` / `fix` / `files` |
| **FULL** | the same digest **plus** step 1's full prose response |
| **CONTROL** | DIGEST again, to measure baseline variance in the same batch |

**The CONTROL arm is required, not optional, and E0 is why.** F27 measured
verdict outputs flipping on 2 of 4 items across identical runs. Without a
same-batch baseline, any DIGEST-vs-FULL difference is unattributable — it could
be ordinary variance. The quantity of interest is whether DIGEST-vs-FULL
divergence **exceeds** DIGEST-vs-CONTROL divergence.

**Primary metric — mechanical.** File-set agreement between step-2 outputs
(Jaccard over cited repo-relative paths). Step 1's outputs already exist in
`results/e3.jsonl`, so step 1 is not re-run: the same recorded digest is
replayed into every arm, which removes step-1 variance from the comparison
entirely.

**Decision rule, unchanged in force.** If FULL agrees with itself no better than
DIGEST does, the digest is a sufficient carrier and the gate costs nothing in
context fidelity. If FULL systematically differs, the digest schema is
under-specified and §6's original conclusion stands.

**n.** 5 items × 3 arms = 15 dispatches.

---

## 7. Sequencing, and the honest tension

**H2 and H3 cost nothing in headroom.** They are deterministic, use instruments
already built (`reconcile`, the corpus parser), and can run alongside E3 without
competing for the Opus budget.

**H1 is cheap.** ~8 haiku-or-sonnet dispatches.

**H4 needs E0's variance figure to size**, so it waits.

**The tension worth naming:** E3 is the *kill gate* — if headless review degrades,
none of this matters. H is the *differentiator* — if the human gate is
infeasible, passing E3 buys a product with no moat. The RFC treats E3 as
decisive and the human gate as an implementation detail; the market evidence in
§7 says the opposite.

**Recommendation:** run **H2 and H3 now**, in parallel with E3, because they are
free and because a correctness failure in resume would change what E3's result
even means. Hold H1 and H4 until E3 reports.

---

## 8. Threats to validity

1. **The latency distribution is one operator's.** 174 gate pauses, one person,
   one project. A team's distribution would differ, and probably lengthen.
2. **`gate_result → verdict` is a proxy.** It brackets the human's answer but may
   include tail agent work; `stage_enter → verdict` is a looser upper bound. Both
   are reported. The >1 h share (45% vs 48%) is stable across both, which is the
   figure the design rests on.
3. **Replayed answers are not live answers.** H2 replays recorded verdicts. It
   tests the orchestrator's determinism, not a human's behaviour under a new UX.
4. **No multi-machine case**, per §2.
5. **The corpus was produced by the incumbent system**, whose gates a human
   answered through a live session. A headless gate may change how quickly people
   answer — plausibly for the worse, since nothing is waiting in front of them.
