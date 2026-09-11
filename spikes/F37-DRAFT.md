## F37 — P7 RESULT: the encapsulated `adversary → policy` block costs **2.64×** a single dispatch and changes the root cause on **1 item in 10**; its policy check returned **false on 33 of 33** runs where an adversary ran, so the block's verdict channel is a constant function of the arm and carries no information about the hypothesis; the P4 deviation channel was offered on 71 dispatches and used **0** times. §4.4's first criterion fires — but at n=3 per cell every Wilson interval overlaps by construction, so what this run establishes is the criterion's **power**, not its verdict

**Established by:** 61 graph runs of `spikes/p9/catalog/root-cause.toml` (v4, `306b58ba7504`) over the E3 rig, 2026-09-08, 297 records, **0 void**. Arms `optional` and `recommended` at 10 items × 3 repeats each, plus 1 `mandatory` run (arm dropped — see §6). Investigator and adversary both Opus at effort `xhigh`; the catalog carries no `model` key, so the tier is the driver's and is recorded. Corpus pin `47faec9f`; answer key = each item's code/test fix files from `spikes/results/s03-fixlane-manifest.jsonl`. **$131.62** of a $200 cap, no throttle halt. `spikes/results/p7.jsonl` → `spikes/results/p7-analysis.txt`. Pre-registered in `METHOD-P.md` §4. Driver: `spikes/runner` subcommand `graph-probe` (`src/p7.rs`).

**"Root cause correct" is mechanical.** The investigator's `root_cause.files` intersect the answer key — the same construction METHOD-E3 uses for its primary metric, so no judgement enters the number (F9's lesson). Every rate below is per item with a Wilson 95% interval, never pooled first (METHOD-S §0 rule 1, F27).

**Instrument break, disclosed.** The `claude` CLI **upgraded mid-run**, 2.1.263 → 2.1.265, and asymmetrically: it caught exactly `recommended`'s **r3 draw on all ten items** (20 dispatches) while `optional` ran entirely on 2.1.263. That is a confound inside the primary comparison. §7 reports the per-repeat split so it can be inspected: `recommended` r3 scored 7/10, identical to r2 on the old build, and the single differentiating result (`issue-81`) came from r1 on 2.1.263. **No effect is detectable, and the comparison is nonetheless not instrument-matched.**

**Record defect, disclosed.** The `emit` record's `corpus_sha` field holds the item's **checkpoint sha**, not the corpus pin — ten distinct values where there should be one. The pin is `47faec9f` throughout; the key is misnamed in the driver and should be `checkpoint_sha`.

### Metrics against the kill criteria (METHOD-P §4.4)

| criterion | measured | verdict |
| --- | --- | --- |
| 1. `recommended` catches no more than `optional` — interval overlap on **every** item | overlap on **10/10**; identical rates on 9/10; `issue-81` 0/3 vs 1/3 | **KILL FIRES** on the pre-registered reading — but see §2, the test has almost no power at n=3 *[corrected 2026-09-11: zero power — no two Wilson intervals can be disjoint at n=3; fires while powerless — Correction (2026-09-11)]* |
| 2. `mandatory` costs > 3 × `recommended` with no additional catches | `mandatory` = **3.66 ×** `recommended` ($9.86 vs $2.69), 1/1 correct *[denominator: the arm mean, n=30 — Correction (2026-09-11)]* | **NOT DECIDABLE** — n=1. The arm was dropped mid-run (§6) |
| 3. deviation warranted fraction < 30%, or > 1 request per item; < 10% removes the channel | **0 requests** in 61 runs / 71 investigator dispatches | **NOT DECIDABLE** — empty denominator (§5) |
| 4. `optional` reaches the key on ≥ 9 of 10 items → fixture too easy, halt and report | **7/10** items (identically 7/10 on the all-three-draws reading) | **does not fire** — the fixture set is not too easy |

### 1. The primary table [measured]

| item | `optional` | `recommended` |
| --- | --- | --- |
| `issue-77-advance-commit-partial-stage` | 3/3 | 3/3 |
| `fix-advance-write-lock-gate-scope` | 3/3 | 3/3 |
| `issue-82-status-terminal-field-collision` | 3/3 | 3/3 |
| `issue-66-id-lock-ec001-rmdir` | 3/3 | 3/3 |
| `issue-146-147-boundary-window` | 3/3 | 3/3 |
| `issue-63-advance-gate-diagnostic-truncated` | 3/3 | 3/3 |
| `issue-70-plugin-root-unset` | 3/3 | 3/3 |
| `issue-79-plan-prds-verifier-false-pass` | 0/3 | 0/3 |
| `issue-74-plan-briefs-dead-plan-anchor` | 0/3 | 0/3 |
| `issue-81-plan-discover-spec-gate-mismatch` | **0/3** | **1/3** |

Seven items are solved on every draw by both arms; two are missed on every draw by both; one separates. Mean citation precision is **0.63** (`optional`) and **0.64** (`recommended`) — the block does not narrow the file set either.

### 2. What §4.4's first criterion can and cannot say [measured, then inferred]

The criterion asks for interval overlap on every item, and gets it on 10/10. **That verdict is nearly unearnable at this sample size.** With n=3, a 3/3 cell has a Wilson interval of [0.44, 1.00] and a 0/3 cell has [0.00, 0.56]; the only way two such cells fail to overlap is a 3/3 against a 0/3, which is the maximum possible separation *[corrected 2026-09-11: even that pair overlaps — [0.439, 1.000] against [0.000, 0.561]; see Correction (2026-09-11)]*. So the criterion as written can only detect an effect that is total, and it would have returned "fires" for any block whose benefit is smaller than *always vs never*.

The one cell that does separate in point rate — `issue-81`, 0/3 vs 1/3 — is exactly the size of effect the test cannot resolve. Taken at face value the block changed the answer on 1 item in 10; taken statistically, nothing here excludes 0 and nothing excludes considerably more.

**So the honest reading is that P7 establishes the criterion's power, not its verdict** [inferred]. Deciding the `quality` shorthand's default on this basis would be deciding it on a test that could not have come out differently. What would resolve it is more draws per cell, not more items: at n=10 per cell a 1-in-10 effect becomes visible, at n=3 it cannot be *[corrected 2026-09-11: at n=10 per cell 6/10 is the first cell that separates from 0/10, and a 0 vs 0.33 effect needs ≈ n=20 per cell — the criterion, not only n, must change; ADR-0111, Correction (2026-09-11)]*.

### 3. The policy check is a constant function of the arm [measured] — the strongest finding here

| arm | `policy` evaluations | pass | false |
| --- | --- | --- | --- |
| `optional` | 30 | **30** | 0 |
| `recommended` | 30 | 0 | **30** |
| `mandatory` | 3 | 0 | **3** |

**33 of 33 adversary dispatches produced at least one finding in a blocking evidence class**, and none produced zero findings. The catalog's check is `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`, so it was false every single time an adversary ran, and vacuously true every time one did not.

The check therefore carries **no information about the hypothesis**. It is fully predicted by whether the arm has an adversary step at all. And in the `recommended` arm it is also **inert**: loop-back is a `mandatory`-only behaviour, so 30 `false` verdicts produced **0** loop-backs and the graph proceeded to `emit` regardless. Whatever the block cost in that arm, it bought a verdict nothing consumed.

This is a finding about PC6's design rather than about this reviewer. A policy expressed as "no blocking-class finding exists" is satisfiable only by a reviewer that finds nothing, and F23/F26/F31 already established that this reviewer finds things reliably. Either the predicate needs a severity or confidence threshold, or the block needs a different terminal condition.

### 4. The adversary works; its work does not reach the answer [measured]

33 dispatches, **188 findings**, mean **5.7** per dispatch, at mean **$1.68** and **676 s** each:

| evidence class | findings | blocks under the catalog's policy? |
| --- | --- | --- |
| `reproduced` | 118 | yes |
| `mutation-survived` | 29 | yes |
| `advisory` | 22 | no |
| `narrowed-pin` | 19 | no |

Seventy-eight per cent of findings are in the two classes that require the reviewer to have *run something* — it reproduced a behaviour, or mutated one and watched the suite stay green. The adversary is not speculating.

It is also **self-evidencing**: it holds `Read` and `Bash` and produced these classes at this rate even while `confirm` was broken (§8) and handing it "the test did not finish". That is worth keeping — it means the block's evidence does not depend on the graph's tool step working.

What none of it does is change the root cause: 22 of the 30 `recommended` runs had a correct root cause *and* a false policy verdict simultaneously.

### 5. The deviation channel was never used [measured]

`RequestDeviation { target, kind, claimed_predicate, why }` was optional on all **71** investigator dispatches and emitted **0** times. §4.4's three deviation thresholds are computed over emitted requests and therefore have an empty denominator; none is decidable.

That the channel was available throughout and never touched is the result, not a gap in the data. `spikes/results/p7-deviation-labels-RYAN-TODO.md` has no rows to adjudicate. Whether P4's channel is worth keeping cannot be answered by a run in which nobody wanted to deviate — it needs a graph the investigator has reason to object to.

### 6. The `mandatory` arm, and why it was dropped [measured]

One run: `issue-77`, correct, **$9.86**, 6 dispatches, 3 `hypothesise` passes, **2 loop-backs** — **3.66 ×** `recommended`. Dropped after that run, on measured grounds rather than budget.

`root-cause.toml` carries `max_rigor = "recommended"`, so `lattice.py:137` makes the activity inadmissible at `rigor = mandatory` and the engine escalates within the `localisation` family instead. Running the family through the engine:

| `has_repro` | rigor | selected | `adversary`+`policy` present? |
| --- | --- | --- | --- |
| false | recommended | `root-cause` (rank 10) | yes |
| false | mandatory | `instrumented-trace-analysis` (rank 40, cost L) | yes |
| true | recommended | `root-cause` (rank 10) | yes |
| true | **mandatory** | **`bisect`** (rank 30) | **NO** |

**For an item that has a reproduction, escalating from `recommended` to `mandatory` drops the block entirely.** `bisect`'s steps are `bisect → widen → explain → confirm → signoff → emit` — a gate, but no adversary and no policy check. Under "lowest admissible wins" it beats `instrumented-trace-analysis`, so the highest rigor level buys *less* internal quality than the one below it, on exactly the items most likely to be reproducible.

So the `mandatory` arm was pricing a block on an activity the engine would not select, in a context where the engine's actual choice has no block to price. **This is a candidate defect in F34 amendment 26's ceiling design, not in `root-cause`**: the family's rigor ladder is not monotone in internal quality. `lattice.py --validate` checks family *totality* over rigor (some member admissible at every level); on this evidence it should also check that escalation does not lose the block. Nothing was edited — the catalog is not touched on an unattended run.

### 7. Instrument lineage across the run [measured]

| arm | repeat | correct / items | claude |
| --- | --- | --- | --- |
| `optional` | r1 / r2 / r3 | 7/10 · 7/10 · 7/10 | 2.1.263 |
| `recommended` | r1 / r2 | 8/10 · 7/10 | 2.1.263 |
| `recommended` | **r3** | 7/10 | **2.1.265** |
| `mandatory` | r1 | 1/1 | 2.1.263 |

Reported because F20's rule is that the instrument is on every record. Had the version moved the metric it would show as a repeat departing from its siblings; it does not. The confound is real and its effect is not detectable at this resolution.

### 8. Instrument findings, and one that invalidates a step's data [measured]

**8.1 `confirm` ran the whole suite for the first 26 of 68 invocations.** It invoked `pnpm test -- --run <file> [-t <pattern>]`, but the package's `test` script is `vitest run`, so that expanded to `vitest run --run <file>` and the positional filter did not survive pnpm's argument forwarding. Caught by reading the step's own captured output: runs of `test/advance-lock-scope.test.ts` were emitting `AC-KB-TOLERANT` and `AC-RUN-SINCE-BAD` results, from unrelated files. Measured side by side in an idle fixture: the `pnpm` form never finished inside 900 s; `npx vitest run test/advance-git-commit.test.ts` reported **1 file, 6 tests, 15.57 s**. **All 26 pre-fix `confirm` records are full-suite runs and must not be read as discriminating-test results.** After the fix (42 invocations): median **4 s**, max 174 s, **0 timeouts**.

**8.2 Two earlier explanations for the same symptom were wrong, and are retired here.** Capping vitest's fan-out from outside (`VITEST_MAX_THREADS`, `--no-file-parallelism`) did not take — 83 node processes and a 24-minute step survived it. Bounding the step with a timeout stopped it stalling an arm but made it *inert*: 15 of 15 bounded runs timed out, because the thing being bounded was a 236-file suite. The bound is still correct — the catalog declares `timeout_secs` on this step — but it was not the cause, and contention was not the cause either.

**8.3 A model-written test selection is not a command line.** The investigator returns `test/x.test.ts -t "some name with spaces"`. The file and the `-t` pattern must be separate argv elements; passed as one string, vitest takes it as a nonsense positional.

**8.4 Parallel arms need separate fixture trees.** `build_silent_fixture` clears and rebuilds `e3-silent-<slug>`, so three arms sharing one fixture root clobbered each other (`os error 32`) and killed the `mandatory` arm on every item in 90 s. Each arm now runs with its own `--worktree`.

### 9. What this settles

- **Settles:** the `adversary → policy` block, as PC6 specifies it and with this policy expression, produces a **verdict that is a constant function of the arm**. That does not depend on sample size and is not a power question (§3).
- **Settles:** the block costs **2.64 ×** a single dispatch — $2.69 against $1.02 per graph run, driven by one extra Opus dispatch at $1.68 and 676 s.
- **Settles:** the P4 deviation channel is unused on this graph (§5), and the fixture set is not too easy (§4.4 criterion 4, 7/10).
- **Does not settle:** whether the block catches more root causes. §4.4's first criterion fires, but the test could not have come out otherwise at n=3 (§2). **The `quality` shorthand's default should not be set from this run.**
- **Does not settle:** anything about `mandatory`, at n=1 — and the arm as specified is not a configuration the engine reaches (§6).
- **Does not settle:** gate behaviour. Both `no-hypothesis` and `signoff` were auto-answered here; that is P8's question.

### Limits

1. **n=3 per cell.** Every Wilson interval is wide enough that §4.4's first criterion is close to unfalsifiable (§2) *[corrected 2026-09-11: unfalsifiable, not close to it — Correction (2026-09-11)]*. This is the dominant limit.
2. **The arms are not instrument-matched:** `recommended`'s r3 ran on `claude` 2.1.265, everything else on 2.1.263 (§7).
3. **26 of 68 `confirm` records are full-suite runs**, not discriminating tests (§8.1). The primary metric never read them; the adversary was handed them.
4. **One graph, one stage, Opus only** (§4.5). `root-cause` is one of four `localisation` family members and the others were not run.
5. **Ten seeded S-SILENT items from one corpus** — whatever E3's fixture construction gets wrong, P7 inherits.
6. **The `mandatory` arm is n=1** and was dropped; §4.4's second criterion has no result.
7. **`no-hypothesis` never fired** — 0 arrivals in 61 runs. The gate the driver was built to auto-answer was never reached, so the auto-answer policy (#4) is untested.
8. **The `corpus_sha` field on `emit` records is misnamed** and holds a per-item checkpoint sha.

### Correction (2026-09-11)

Dated corrections from the 2026-09-10 pre-implementation audit (PIA-073, PIA-077, PIA-080; ADR-0111, ADR-0106).
The original figures and sentences above stand as written; the corrected statements sit beside them and here.

**1. §2 and Limits 1 — at n = 3 no two Wilson intervals can be disjoint, so criterion 1 had zero power (PIA-073;
ADR-0111 decision 3).** §2 says a 3/3 against a 0/3 is "the only way two such cells fail to overlap". They do
overlap: 3/3 → **[0.439, 1.000]** and 0/3 → **[0.000, 0.561]** at the registered two-sided z = 1.96 [measured,
recomputed 2026-09-11; the audit's figures agree]. There is no pair of cells at n = 3 whose intervals are disjoint,
so §4.4(1) **could not fail to fire** at this n, whatever the block did. Limits 1's "close to unfalsifiable" reads
**unfalsifiable at n = 3**. The criterion is reported, by name, as **fires while powerless** (golden constraint 10;
DECISIONS-2026-09-11 #10): it fires on its pre-registered reading and its registered n could not decide it. Nothing
here re-thresholds it.

§2's remedy — "more draws per cell … at n = 10 per cell a 1-in-10 effect becomes visible" — is also corrected. At
n = 10 per cell the first cell whose interval separates from 0/10 ([0.000, 0.278]) is **6/10** ([0.313, 0.832]);
an effect of 0 vs 0.33 needs roughly **n = 20 per cell** under interval overlap. S-P7B as registered (n = 10) is
therefore **powerless** for the effect ADR-0080 registered it to detect, and the re-run must change the
**criterion**, not only n — a paired per-item comparison, or an interval on the pooled difference of proportions
with a stated minimum detectable effect and the n that reaches it (ADR-0111 decision 1; METHOD-P amendment 13).

**2. R-P7RATIO — the 3.66× / 4.4× "contradiction" is two denominators (PIA-080; ADR-0106 clause 3).** Both figures
share the numerator, the single `mandatory` run on `issue-77` at **$9.857**. **3.66×** = $9.857 / **$2.693**, the
`recommended` arm's mean per graph run over n = 30 (§6 above; `p7-analysis.txt` §3). **4.4×** (DECISIONS-2026-09-09
#33) = $9.857 / **$2.222**, the `recommended` r1 draw of the same item, `issue-77`, a single run. Neither is wrong;
neither may be quoted without its denominator (ADR-0106 forbids). The `mandatory` figure stays **n = 1** (ADR-0081;
ADR-0111 decision 4) and §4.4(2) stays NOT DECIDABLE; R-P7RATIO is reconciled and closed by denominator
identification, and the `emit.corpus_sha` re-keying (§9 above, Limits 8) remains open. [documented, audit PIA-080
and ADR-0106 for the $2.222 draw; measured, `p7-analysis.txt` §3 for $2.69 and $9.86]
