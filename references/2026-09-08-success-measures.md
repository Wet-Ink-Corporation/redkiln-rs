# Redkiln-RS — the success-measurement register

**Date:** 2026-09-08 · **Status:** Register. Binding on Phase 1 telemetry; every id below is stable and must not be renumbered.
**Companion:** `2026-09-08-acceptance-criteria.md` (AC-001..AC-519, thirteen areas). Each SM range sits beside its area's AC range: a measure is what the criteria could not settle at build time.
**Inputs:** the thirteen adversarially-refuted area files (kernel, selection, packs-catalog, artifacts-store, git-reconcile, harness, human-gate, review-gate, verify-gate, events-cost, cli-doctor, migration, ports-dist); `spikes/FINDINGS.md` F1–F38 as condensed in the evidence brief §A–§E; `spikes/results/s-mig-report.md` and `spikes/corpus/PINNED_SHA`; `references/2026-09-05-architecture-proposal.md`; `references/2026-09-07-design-recommendation.md`; `references/2026-09-07-process-catalog-proposal.md`; the Phase 1 ADRs.
**Evidence discipline:** every baseline carries its source, its n and one of three tags. `[measured]` = a number read out of a results file or a FINDINGS entry. `[documented]` = a platform, ADR, proposal or external-vendor contract. `[inferred]` = a design consequence with no direct measurement. A figure that an ADR or the brief itself records as `[documented]` with no n may not be re-tagged `[measured]` here. Where a rate is quoted, a Wilson 95% interval is quoted with it or the interval is declared **not computed** — and where an interval was computed for this register rather than read from a source, it says **computed here**. Where a figure is one draw, one repo, one machine or one operator, it says so. **No baseline is ever invented:** a measure whose area author had none reads *no baseline — first measurement establishes it*, and §5 names what would establish it.

---

## 0. The measurement doctrine

The programme that produced this register measured its own measurements and found them wanting. **Five metrics saturated** — E3 detection 23/23 vs 23/23 at McNemar p=0.625, which the brief records as "primary metric saturated — no discriminating power" (F23); E3-H 20/20 across two harder tiers (F26); S-TYPED schema conformance 52/52 = 100% (F31); H2 suspension 174/174 (F19); H4 Jaccard FULL 1.000 = CONTROL 1.000, effect +0.000 (F28). **Every discriminating result the programme produced came from cost, stability or mechanism — never from a success rate** (brief §C.12, attributed there to F28).

That is not a footnote. It is the construction rule for this register.

1. **The register is built on cost, stability and mechanism.** The cost measures (SM-005, SM-040, SM-067, SM-090, SM-110, SM-153, SM-154), the stability measures (SM-015, SM-087, SM-107, SM-113, SM-136) and the mechanism alarms — those whose *nonzero* value falsifies an architectural claim rather than moving a dial — carry the load. Everything else is context for them.
2. **Every success-rate metric present must justify its place.** Each row is labelled: a **discriminator** can distinguish a good outcome from a bad one; a **tripwire / canary** saturates and its only information is the moment it moves; a **discovery instrument** has no target and cannot fail, and exists to establish a distribution; a **tracker** records data and gates nothing. A tripwire is not assurance. 134 measures are not 134 pieces of evidence, and §3 records the quantities this register refuses to track at all.
3. **A rate is quoted with a Wilson 95% interval, or the interval is declared not computed.** Point estimates at small n are this programme's own recorded failure mode: F19's 3.6× duration effect on drift has overlapping conditional intervals (1.1–8.9% vs 6.1–20.3%) and is directional only; F27's "stable" label at n=2–3 tolerates a true flip rate up to ~30%.
4. **A void is never a miss.** A void draw is reported in its own denominator alongside the outcomes, never folded into either. This holds for spike voids (P1 400/0, S-VAR 60/0, P7 61/0 — the brief reports *no* void count for P8, only 0 changed and 0 lost) and for production `gate_answer_void` (SM-075, SM-125) and `Unverifiable` (SM-088, SM-096, SM-104).
5. **A baseline carries its n.** "18,603 tokens" is n=1 per gap. "0 of 23 verified false positives" is n=23 items on one corpus and its Wilson interval reaches 14.3%. "P8" is 3 of a pre-registered 27+3, with the 65-min arm, the 24-h arm and all three controls unrun.
6. **A metric measured on one operator, one machine or one repo says so, at every point of use.** F18 (n=174 and n=191 gate latencies), F19 (n=174 drift), F22 (process survival, cause unestablished), S-MIG (1,312 files at one frozen SHA `47faec9f`, by an instrument iterated four times against that same corpus) and every authoring baseline (F32/F33/F34 — three authoring *passes*, not three independent people) carry that limit. None of their rates generalise.
7. **Upper bounds are not values.** CLEAN's ground truth is wrong for **≥13 of 23 = 57%** (brief §E, contested). Every false-positive rate measured against it — 2/23, 4/23, 0/23 — is an **upper bound only**, and §2's final column says so for each.
8. **A superseded figure is never quoted.** Per-draw `five_hour` headroom is **0.0098** (F29, 60 draws); F21's earlier ~0.077 is **7.8× too high** and appears nowhere below except as this correction. F28's flat-18,603 result stands as a *bound*, but its mechanism was corrected: the flatness was shared-prefix subsidy, not durability.
9. **A measure with no condition that can fire is not a measure.** Where an area author could state no firing kill, this register keeps the row and says so *in* the row, rather than shipping an unmissable target as assurance.
10. **Saturation is declared at the row, not discovered in a dashboard.** A metric that reaches ceiling or floor is demoted from decision input to regression tripwire within one review cycle, and the demotion is recorded (SM-118 is the register's own method control).

---

## 1. The register

**134 ids across thirteen areas.** Ranges are area-allocated. Unused ids inside a range — SM-011, SM-012, SM-047, SM-048, SM-059, SM-060, SM-071, SM-072, SM-083, SM-084, SM-108, SM-119, SM-120, SM-131, SM-132, SM-142, SM-143, SM-144, SM-155, SM-156 — are reserved and are never reassigned. **SM-150 and SM-152 were cut in refutation and their ids are retired**, not reused.


### 1.1 Kernel — workflow/activity split, replay, memoisation, budgets, retry (SM-001..SM-010)

Ten measures. Four can discriminate (SM-001, SM-002, SM-004, SM-005). Three carry no firing kill (SM-006 at instrument resolution, SM-007's rate, SM-008 as telemetry) and three saturate at zero by construction (SM-003, SM-009, SM-010).

**SM-001 — Memo effectiveness** · *discriminator*
- **Instrument:** `memo_hit` / `memo_miss` events carrying the five memo-key components (AC-019).
- **Baseline:** **no baseline — first measurement establishes it.** No spike measured memoisation; the memo table does not yet exist.
- **Target:** hit rate reported split three ways — (a) steps outside loops, (b) steps inside `repeat`/`on_false` cycles, (c) `memo = false` tool steps. **Only (a) is a meaningful rate:** (b) is a cache miss *by construction* under ADR-0021's loop-aware key and (c) opts out, so an undifferentiated hit rate is a meaningless number and must not be reported.
- **Sampling:** every step; a distribution over activities, never a single mean.
- **Kill:** a hit rate near zero in bucket (a) means the key is over-specified — probably a volatile input leaking into resolved inputs — and the memo layer is paying hashing cost for nothing.

**SM-002 — Re-entry cost** · *discriminator*
- **Instrument:** `cache_creation_input_tokens` on the first dispatch after a re-entry; the `run` entry event.
- **Baseline:** mid-graph resume **8,408 / 10,314 / 14,579** tokens = **0.45× / 0.55× / 0.78×** of a stage gate's ~18,603 [measured, F38, **n=3, PARTIAL** — 3 of a pre-registered 27+3, $3.01 of a $40 cap, short-duration draws only]. Stage-gate comparator: **18,603 under the default prompt at both 65.2 min and 49.7 h**, n=1 per point [measured, F28]. Static-card-prompt comparator: **29,680 at 66.8 min (F28) and 29,619 at 66.1 min (S-H1b)** — **n=2, both inside one hour**; the **S-H1b 24-h read was never taken**, and S-H1b has no F-row (F30 is unassigned; its numbers live only in the design recommendation and the proposal). No static-prompt figure exists at 49.7 h. Interval not computed (n=3).
- **Target:** at most one prefix rewrite per re-entry, invariant in the gap's duration. Duration-invariance is measured for the **default-prompt stage gate only**; for mid-graph resume it is a hypothesis, because P8's 65-min and 24-h arms are unrun.
- **Sampling:** every re-entry; report as a distribution.
- **Kill:** a re-entry cost that grows with gap duration falsifies the mark-and-read design and returns gate placement to a cost decision. One machine, one operator, three runs.

**SM-003 — Retry composition by error class** · *saturate-at-zero tripwire*
- **Instrument:** `activity_retry` events carrying the `StepError` class; `system/api_retry` for the inner layer — the event type is real and observed in `spikes/results/e3.jsonl` (1–2 per run), but **its payload was never captured**, so an attempt counter is an assumption (AC-026 is PROVISIONAL on a capture check).
- **Baseline:** **no baseline — first measurement establishes it** for the outer layer. Inner-layer retry *events* are observable in the stream today; whether they carry an attempt counter is not established.
- **Target:** **zero** retries recorded against `NotAdopted` / `NotIsolated` / `Unsupported` / `Overage` (AC-027). Expected to saturate at zero; kept as a tripwire, never as a discriminator.
- **Sampling:** every retry. Quote any nonzero rate with a Wilson interval; a void run is never counted as a miss.
- **Kill (fires on a single observation):** one retry against any of the four configuration classes is a defect report against AC-027 and blocks the release that introduced it.

**SM-004 — Which budget binds, and how often** · *discriminator*
- **Instrument:** `budget_exhausted` events tagged `attempts | tokens | wall_clock`.
- **Baseline:** **no production baseline.** Nearest corpus anchors on the wall-clock axis [measured]: S-TYPED CLEAN review median **1,071 s (~18 min)**, slowest **4,561 s**; E3 CLEAN 317–2,055 s; S0.1 mutant runs median **323 s**; the full gate has no affected-area selection, so every run is the whole 236-file suite (~250 s to >900 s over 1,890–1,927 tests). Timeout history [documented]: 900 s → 2,400 → 3,600 → one 5,400 s retry — a default written for the fast condition, which cost a CLEAN record.
- **Target:** wall-clock exhaustion is rare and attempts exhaustion is the common bind; the three shares are reported separately, because the budgets are never shared (AC-020).
- **Sampling:** every exhaustion, plus p50/p90/max of each budget's utilisation per run.
- **Kill:** if wall-clock is the routine binding budget, timeouts are again sized for the fast condition.

**SM-005 — Cost per activity and per run** · *discriminator*
- **Instrument:** `total_cost_usd` and `usage` on each activity's own child event (attribution is free because every activity is its own child).
- **Baseline [measured]:** trivial haiku dispatch **$0.021–0.027** (F3); E3 S-SILENT review **$0.45–0.64** (97–424 s); E3 CLEAN **$1.06–4.89** (317–2,055 s); S-TYPED CLEAN review **median $3.03** at median 1,071 s (F31, 51 valid). Separately, the S-TYPED **arm total** was **$112.25** against a **$64.61** prose comparator = **+74%** — an arm-level spend ratio, **not** a per-review delta; **the two must never be quoted as one figure.** P7 graph run **$1.02** optional / **$2.69** recommended / **$9.86** mandatory (n=1, an arm the decisions log flags as pricing an unreachable configuration, with an unreconciled ratio: 3.66× in F37, 4.4× in the decisions log).
- **Target:** a Phase-1 fix-lane run lands inside the **$0.70–$1.68** Agentless-to-Cloudflare-full-tier band per item (see §4 for what that band is and is not).
- **Sampling:** every activity. Per-activity distributions, never a run mean — a run mean hides the one $9.86 outlier.
- **Kill:** a per-item median above the S-TYPED CLEAN median (**$3.03**) means the fix lane costs more than a full adversarial review of the same change, and the lane's economics are refuted.
- **Note:** `usage` exposes no cache-TTL split (F3), so the charter's TTL-split metric is not computable from this instrument.

**SM-006 — Headroom consumed per run** · *batch telemetry; no kill available at instrument resolution*
- **Instrument:** the top-level `rate_limit_event`, `unifiedWindows.{five_hour,seven_day}.utilization`, read as a batch delta; `isUsingOverage` per dispatch.
- **Baseline [measured]:** **0.0098 `five_hour` per CLEAN draw** (F29, 60 draws, 0 void, $176.96, 16.2 h); **`seven_day` 0.0017/draw**. **F21's 0.077 is superseded — it was 7.8× too high.** Instrument resolution is **0.01** (F6), so a single run sits below the instrument and per-run cost is not measurable: report batch deltas with a `resolution_limited` flag. There is **no Opus/Sonnet family sub-limit key** (F1) — the binding limit is invisible to this instrument.
- **Target:** the margin check refuses to dispatch inside ≥2 ticks of the ceiling and whenever `isUsingOverage` is set. **The 2-tick margin is chosen, not measured.**
- **Sampling:** batch-level. Standing finding: on a review-scale set the binding constraint is **wall clock, not headroom** — 16.2 h for 60 draws (F29).
- **Kill:** **none available at this resolution.** A per-run headroom target cannot fire against a 0.01-quantised instrument, and this is descriptive batch telemetry until a finer signal exists. The one condition that can fire is a refusal that did not happen: a dispatch admitted while `isUsingOverage` was true is a defect against AC-027.

**SM-007 — Circuit-breaker trips** · *classification tripwire; the rate is untestable*
- **Instrument:** `breaker_opened` / `breaker_closed` events, per model family, with the K constituent failures attached.
- **Baseline:** **no baseline — first measurement establishes it.** The only public comparator for an exceptional-path rate is Cloudflare's break-glass at **288 of 48,095 MRs = 0.6%** [documented] — a different mechanism in a different organisation (§4).
- **Target / Kill:** "trips are rare" is **not a testable target and carries no kill** — no corpus figure sizes a `Platform` failure rate. The one testable condition is classification: a trip whose K constituent failures include a `Schema` or `NonZero` error misclassified as `Platform` is a classification defect and fires immediately, independent of rate.
- **Sampling:** every trip. Quote the rate with a Wilson interval or state that it is not computed.

**SM-008 — Degradation frequency and its consequence** · *descriptive telemetry, not a success measure*
- **Instrument:** `Degraded { from, to, why }` events; count of gates that failed to close because of AC-038.
- **Baseline:** **no baseline — first measurement establishes it.** Nearest corpus analogue: a typed policy escape hatch **offered 71 times and used 0** (F37, 61 runs / 297 records) — so the live question is whether `Degraded` is likewise a dead path or a routine one.
- **Status:** "either answer is informative" is the definition of a target that cannot be missed. Kept because the instrument is free and the answer sizes a real design question.
- **Sampling:** every substitution; report the `why` distribution.
- **Kill (the one condition that can fire):** a single dominant `why` combined with a nonzero count of AC-038 gate non-closures means the pack's tier choice, not the runtime, is wrong, and pack tiering is reopened.

**SM-009 — Replay divergence** · *saturate-at-zero tripwire*
- **Instrument:** `replay_divergence` events from the nightly replay of the last N runs (AC-006/AC-007), every port on its fake.
- **Baseline:** **no baseline — first measurement establishes it.** F19's 174/174 losslessness is about the state *carrier*; **the FSM was never replayed**, so replay determinism has never been measured at all.
- **Target:** **zero.** Expected to saturate; kept because a single divergence falsifies the workflow/activity split's central claim and blocks Phase 1's "deterministic resume across kill" exit criterion.
- **Sampling:** every recorded run, nightly, at zero model cost.
- **Kill:** a divergence that cannot be traced to a banned construct (AC-001..AC-005) means the ban list is incomplete.

**SM-010 — Schema re-prompt rate** · *saturated in the corpus; per-card kill only*
- **Instrument:** `schema_retry` events; `Schema { attempts }` errors.
- **Baseline [measured]:** schema conformance **52/52 = 100%** and `repro.command` fill **147/147** under the typed contract (F31) — i.e. the observed re-prompt rate on that card/schema pair was **0**. Interval not computed. **One card/schema pair.**
- **Target:** near zero. A card whose re-prompt rate is materially above zero has a schema the model handles badly (proposal §4.4: flat, no nested `oneOf`, closed enums, `additionalProperties: false`, required-first).
- **Sampling:** every dispatch carrying a schema; report per card, never pooled.
- **Kill (per card, not per programme):** a card whose re-prompt rate exceeds zero on repeated dispatches has its schema rewritten against the §4.4 rules before it ships. **This measure carries no programme-level kill.**
- **Note:** F20's silent-void mechanism (a card `tools` list omitting `StructuredOutput` → `structured_output` absent at exit 0, `subtype: success`) does **not** show up here; it is caught at configuration time in the adapter area, and this measure must not be read as covering it.

### 1.2 Selection engine — the seven-step pure function (SM-013..SM-024) · **BLOCKED on P1**

Twelve measures. The area's own gate is unmet: P1's kill fired **4 of 4** and every ordinal facet stays `source = "human"` until AC-071's stability gate exits 0. Five measures have no baseline (SM-013, SM-018, SM-020, SM-022, SM-024) and cannot fire on their first reading.

**SM-013 — DAG replay identity** · *saturate-at-zero tripwire*
- **Instrument:** `dag_assembled.inputs_digest`, `redkiln replay --run <id>`, `HiddenInput` events.
- **Baseline:** **no baseline — first measurement establishes it.** Nearest analogue is F19's explicit limit: 174/174 gate suspensions were lossless but **the FSM was never replayed**, so replay identity has never been measured on this system [measured for losslessness only, F19, n=174, one operator]. Mid-graph resume adds 3/3 digest-identical steps across a real `taskkill /T /F` [F38, PARTIAL, n=3 of 27+3]. **Correction (2026-09-08, post-refutation).** The refutation pass struck every reference to "spike P5" on the grounds that it appears nowhere in the evidence brief. That was an over-correction: P5 is real — `references/2026-09-07-process-catalog-proposal.md` §6 row P5 and §6.1: "P5 replay identity + conformance — is the event log sufficient to re-derive every DAG and to check invariants?", kill criterion "any non-reproducible DAG → hidden input → memo key unsound; fix before anything else", **reclassified** to a Phase 1 acceptance test. The brief simply did not carry it. P5's replay-identity rule is the soundness check for memoisation and is **restored here as this measure's registered criterion**: 100% replay identity, with violations attributed to pack versus kernel.
- **Target:** 100% replay identity across all recorded runs.
- **Sampling:** every run, continuously; report a *count* of non-identical runs, never a percentage rounded to 100%.
- **Kill:** **any** non-reproducible DAG means a hidden input, which makes the proposal §4.2 memo key unsound; a single failure stops memo serving (AC-043) and does not lower the target.

**SM-014 — Catalog lint health: rank ties, unsatisfiable pairs, dead activities** · *two live counts, two tautological zeroes*
- **Instrument:** `redkiln lint --lattice`, zero model cost.
- **Baseline [measured, F35, n=1 enumeration over 102 activities, one machine, lattice runtime 27 s]:** unsatisfiable `(stage, ctx)` pairs **23.5% → 11.4% → 0.3%** across the pre-registered, 12-pack and post-decision readings; dead activities **15/101 → 6/102** (**20.8% at 8 packs → 14.9% at 12 → 5.9% post-decision**, brief §B); hard cycles **0**; rank ties **0**.
- **Target:** ties 0 and cycles 0 permanently (both are build failures under AC-047/AC-059/AC-078); unsatisfiable ≤0.3% and non-increasing across catalog versions; dead ≤6/102 at 12 packs, and reported.
- **Sampling:** every CI run and every catalog version bump; report the three counts **and the enabled pack count**, never a composite score.
- **Kill:** unsatisfiable >5% is the pre-registered P6 kill — catalog gaps are fixed before any dispatch. The reading is sensitive to pack count: the dead-activity kill (>20%) fired by 0.8 points under 8 packs and cleared under 12, so **a rate reported without its pack set is not a reading.**
- **Honesty note:** ties 0 and cycles 0 are a single enumeration of one catalog, not a durable property, and once both are build failures they cannot recur without CI failing. The discriminating content of this measure is the unsatisfiable and dead counts, not the two zeroes. *(A claimed "30.1% unsatisfiable" was struck — no such figure exists in the brief.)*

**SM-015 — Context-vector stability (the block's own metric)** · *discriminator; already fired*
- **Instrument:** the P1 rig (`spikes/runner`, arm L), `context_digest`, modal required-set identity over draws.
- **Baseline [measured, F36, 400 dispatches, 0 void, $21.37, one corpus pin]:** modal required set identical on **5/20 = 25%**, **3/20 = 15%**, **2/20 = 10%** and **2/20 = 10%** — two packs (`bug-standard`, `story`) × two runs (v1, and the §1.4 anchor-rewrite re-run v2), each computed across all 10 draws — against an **80%** line. Wilson 95% intervals **computed here, not measured**: 11.2–46.9%, 5.2–36.0%, 2.8–30.1% and 2.8–30.1% — every upper bound sits far below the line. Alpha drift **−0.168 on byte-identical anchors**; **the pre-registered alpha rule is not decidable on this corpus**, so alpha is recorded and is not the decision variable. Draws-per-item is **10** — METHOD-P §1 dispatches 5 repeats × 2 models (haiku and sonnet) per item, and F36 and `spikes/HANDOFF.md` both state the modal set was identical "across ten draws". The 400 dispatches are 20 items × 5 repeats × 2 models × 2 runs (v1 and the §1.4 anchor-rewrite re-run v2). *(An earlier refutation pass struck "ten draws" as unsourced and substituted 5; that substitution was itself wrong and is reversed here against F36 and HANDOFF. AC-071's gate requires ≥ 10.)*
- **Target:** ≥80% modal identity at n ≥20 items × ≥5 draws, with the **Wilson lower bound above 0.80**.
- **Sampling:** a full re-run per triage-card or anchor change; never a partial re-run — the pre-registered anchor-rewrite repair already failed once at full n.
- **Kill:** **already fired, 4/4.** A re-run that does not clear the line keeps every ordinal axis `source = "human"` (AC-071). This is the programme's method finding in miniature: the discriminating signal is stability, not a success rate.

**SM-016 — `unknown` rate per axis (gate load transferred to the operator)** · *tracker; forecasts operator load*
- **Instrument:** the triage context record, per-field `unknown` count; `gate_opened { reason: unknown_facet }`.
- **Baseline [measured, F36, n=200 draws]:** novelty **46.0%**, size **39.5%**, blast_radius **31.0%**, criticality **27.0%**; `external_dependency` degenerate at **200/200 false**. Per-axis Wilson intervals **not computed** — the draws are not independent across items.
- **Target:** **no target is set from this data.** It is tracked because `unknown` routes to a human gate (AC-079), so the rate forecasts operator load directly — priced at F18's **gate→verdict median 10.5 min (n=174)**. The heavier tail (**p90 ~9 h, max 226 h = 9.4 days**) belongs to F18's **stage→verdict** series (n=191) and must be quoted as that series, never fused with the median above.
- **Sampling:** every triage record.
- **Kill:** an axis whose `unknown` rate makes its gate load exceed the operator's budget is made `source = "human"` outright — the axis stops pretending to be model-emitted rather than gating on every item.

**SM-017 — Deviation channel volume and warranted fraction** · *floor kill only; the fraction has an empty denominator*
- **Instrument:** `RequestDeviation` records, deviation gate outcomes, `dag_amended` events.
- **Baseline [measured, F37, 61 runs, 297 records, 0 void, $131.62]:** deviation was **offered 71 times and used 0**. The warranted fraction therefore has an **empty denominator** — P7 §4.4(3) is recorded in the brief as *not answerable*, not as 0%.
- **Target:** none set. Track offers, uses and warranted-on-review as **three separate counts**.
- **Sampling:** every activity exit that could carry a deviation.
- **Kill (the floor, which can actually fire):** if uses stay at 0 over the first 100 items, the channel is dead code and is removed rather than carried — a channel with an empty denominator cannot be evidence for itself. *(Two thresholds attributed to a "spike P4" were struck: there is no P4 in the brief and neither number appears in it.)*

**SM-018 — Unsatisfiable-gate rate on live items** · *no baseline; ordered against an external comparator*
- **Instrument:** `gate_opened { reason: unsatisfiable }` per stage entry.
- **Baseline:** **no baseline — first measurement establishes it.** F35's 0.3% is over *enumerated* contexts, not observed items, and does not transfer. Comparator [documented, brief §D]: Cloudflare's break-glass rate, **288 of 48,095 MRs = 0.6%**, is the external order of magnitude for "the process could not express this case".
- **Target:** within the same order as 0.6%; report with a Wilson interval once n ≥100 stage entries.
- **Sampling:** every stage entry; report the rate with its interval **plus the distinct `(stage, kind)` pairs behind it** — a rate concentrated on one kind is a catalog gap, not item novelty.
- **Kill:** a rate materially above the comparator with a concentrated `(stage, kind)` profile sends the catalog back to authoring (AC-056) rather than relaxing the engine. *(The comparator is a different organisation's different process: it bounds an order of magnitude, and no threshold derived from it is a pre-registration.)*

**SM-019 — Selection cost** · *mechanism alarm*
- **Instrument:** `dag_assembled` wall time; model spend attributable to selection, which must be $0.
- **Baseline [measured, F35 and brief §D, one machine, n=1 run]:** full lattice enumeration **27 s** over 102 activities and 12 packs; P6, P9, P9b and the lattice work cost **$0** in model dispatches. Per-item selection latency: no baseline.
- **Target:** **$0.00** model spend for selection, always; per-item selection wall time negligible against the cheapest dispatch (a trivial haiku dispatch is $0.021–0.027 [F3]; TTFB ~1,480 ms of ~3,400 ms [F8]).
- **Sampling:** every selection.
- **Kill:** any model spend attributed to selection falsifies purity (AC-044) and invalidates every replay claim in the area.

**SM-020 — Memo hits and unsound evictions** · *the hit rate saturates; the eviction count is the measure*
- **Instrument:** `memo_hit`, `memo_miss`, `memo_evicted { reason }` events.
- **Baseline:** **no baseline — first measurement establishes it.**
- **Target:** evictions for `unsound_replay` = **0**. The hit rate is expected to saturate near 1.0 on unchanged items and **is therefore not the discriminating signal** — it is retained only as the denominator for the eviction count, per the programme's method finding.
- **Sampling:** every memo lookup; evictions as an absolute count, hits as a distribution by activity.
- **Kill:** one served hit from a non-replayable run (AC-043) is a correctness defect, not a rate to manage.

**SM-021 — Short-circuit precision and recall** · *target not citable; P2 is unrun*
- **Instrument:** the P2 harness — Present/Stale/Missing against a human "should re-run" label over replayed items, one perturbed upstream artifact each.
- **Baseline:** **no baseline — P2 is entirely unrun.** Brief §B records all three of its criteria as unanswered and §E records it as awaiting Ryan, with the engine phase refusing until the labels are timestamped.
- **Target:** **not citable from the evidence brief.** The brief does not carry P2's registered precision/recall lines or its n; they must be read out of P2's pre-registration before this measure has a target. Until then this row records only that the measurement is owed. *(A previously stated "recall ≥0.90, precision ≥0.70 at n=20" appears nowhere in the brief and was struck.)*
- **Sampling:** one P2 run per change to the staleness scope rule.
- **Kill:** below P2's registered lines, `stale_policy = gate` becomes mandatory for L-class activities (AC-068's precondition) — a cost transfer to the operator priced by F18's 10.5 min gate→verdict median. **The kill's direction is registered; its numeric trigger is not yet citable.**

**SM-022 — Advisory drift noise on vouched roots** · *no direct baseline; measures a channel's decay*
- **Instrument:** `drift_advisory` events per human-supplied artifact; advisory-to-action ratio.
- **Baseline:** **no direct baseline.** The nearest measured proxy is world-drift at gate re-entry — **12/174 = 7%**, split **3/95 = 3%** for gates ≤1 h against **9/79 = 11%** for gates >1 h, a 3.6× difference, with 45–48% of gates exceeding 1 h [measured, F19 and F18, one operator, one repository]. A Wilson 95% interval of **4.0–11.7%** on the 7% is **computed, not measured**. **That is drift in the world, not drift of a vouched artifact, and must not be quoted as the latter.**
- **Target:** none set. The measure exists to detect an advisory channel that becomes noise and gets ignored — which is how "advisory" degrades into "absent", the same destination the brief flags for the 7 adjudicated-true `narrowed-pin` findings that have no sink (§C.8).
- **Sampling:** every entry to a stage requiring a vouched artifact.
- **Kill:** an advisory channel whose acted-upon fraction falls to zero over 50 advisories is either removed or promoted to a gate — a permanently ignored advisory is worse than none, because it looks like coverage.

**SM-023 — Selection explanation completeness** · *saturating tripwire, not a discriminating measure*
- **Instrument:** schema conformance on `dag_assembled` explanation rows (`rows_fired`, losing ranks, `skipped: [(kind, Present)]`).
- **Baseline [measured, F31, n=52]:** the comparable typed-contract conformance rate was **52/52 = 100%**, with `repro.command` filled 147/147 — a typed schema the kernel writes is filled by construction.
- **Target:** 100%. **This metric is known to saturate and does not discriminate.** It is retained purely as a regression tripwire, because the failure it catches — an explanation row silently dropped in a refactor — is exactly the class F20 showed can occur at exit 0 with `subtype: success`.
- **Sampling:** every `dag_assembled` write; alarm on the first non-conforming record, never on a trend.
- **Kill:** **none.** A saturating tripwire has no kill criterion; it is labelled as such precisely so it is never read as evidence that selection works.

**SM-024 — Human-supplied facet load** · *the price of the P1 block*
- **Instrument:** decision-memory rows `(context, field, value, why)`; override replays; triage-gate duration.
- **Baseline:** **no baseline — first measurement establishes it.** Its cost basis is measured: a human gate costs a bounded **18,603 tokens** at both 65.2 min and 49.7 h under the default prompt, and **29,619–29,680** under a static card prompt at ~66 min [measured, F28 as corrected and S-H1b / §C.1; three marks at 65–67 min plus one at 49.7 h, n=1 per point]; gate→verdict wall time is median **10.5 min** [F18, n=174, one operator], with the p90 ~9 h tail belonging to the **stage→verdict** series (n=191).
- **Target:** the replay rate — overrides served from decision memory rather than re-asked — rises over time; the count of first-time overrides per item falls.
- **Sampling:** every triage gate; a distribution of overrides per item, never a mean.
- **Kill:** if AC-071 keeps every ordinal axis `source = "human"` and the per-item override count does not fall, the closed context vector is being hand-authored on every item and the deterministic-selection premise is paying an operator cost the programme never priced. **That is a design reopening, not a metric miss.**


### 1.3 Process packs, technology packs, the activity catalog and its lint (SM-025..SM-036)

Twelve measures. Three have no baseline (SM-028, SM-033) or an invalid one (SM-032). Three sit so far inside their lines that they cannot fire against today's catalog (SM-026 at 16× slack, SM-031 never at risk in three passes, SM-029 already *in* its kill state at baseline).

**SM-025 — Whole-catalog lattice CI runtime** · *discriminator*
- **Instrument:** the lattice CI job's wall clock, emitted as `lattice_run_completed { duration_ms, activity_count, pack_count }`.
- **Baseline:** **27 s** over the 102-activity v0 catalog [measured, F35, n=1 reported run, one machine; **the brief does not record the pack count for that run**, so no pack-count attribution is made].
- **Target:** stays inside the per-commit CI budget (≤5 min) at present size. **The "at 2× catalog size" projection is untested** — nothing in the corpus establishes how the lattice scales — so it is a hypothesis this measure exists to check, not a target.
- **Sampling:** every CI run; report p50 and p95, never a mean alone.
- **Kill:** if the run exceeds the per-commit budget, catalog completeness moves to a nightly job and AC-100 weakens from "fails the build" to "fails the build within one day" — which must then be tracked as a staleness measure, because the whole value of F35 was that the check was cheap enough to run on every change.

**SM-026 — Catalog unsatisfiability rate** · *regression tripwire; 16× inside its line*
- **Instrument:** lattice output, `unsatisfiable_cells / total_cells` over the facet-vector × rigor cross-product.
- **Baseline:** **23.5% → 11.4% → 0.3%** across the pre-registered reading, the 12-pack reading and the state after decisions 1–10 [measured, F35, n=1 catalog].
- **Target:** ≤5% — the pre-registered P6 §3.4 kill line; trend to 0.
- **Sampling:** every catalog change. **Interval not computed:** this is a census of the cross-product, not a sample, so a Wilson interval does not apply.
- **Kill:** a rate that climbs back above 5% as packs are added falsifies "requirement lives in the pack" (ADR-0024) as composable, and forces requirement authorship back into the activity.
- **Slack, stated:** the current 0.3% sits **16× inside** the line, so this measure cannot discriminate until the pack set grows substantially. Until then it is a regression tripwire, not a live signal.

**SM-027 — Dead activities** · *discriminator, but only when read with its pack count*
- **Instrument:** lattice output — activities reachable from no enabled pack at any rigor.
- **Baseline:** **20.8%** under the 8 pre-registered packs (fires the >20% kill by 0.8 points), **14.9%** under 12 packs, **5.9% = 6 of 102** post-decision [measured, F35].
- **Target:** ≤20% hard, ≤10% desired.
- **Sampling:** every catalog change; **report the list, not only the count** — the list is the input to the ADR-0027 ship/extend decision (AC-114).
- **Kill:** above 20% at the declared pack set.
- **Note:** this metric is sensitive to the *number of enabled packs*, not only to the catalog — 8 packs vs 12 moved it 5.9 points with the catalog unchanged. **A reading without its pack count is uninterpretable**, and two readings at different pack counts must never be compared.

**SM-028 — Configuration defects caught at configuration time vs at dispatch time** · *saturating gap-detector*
- **Instrument:** `doctor_rule_failed{rule}` counts against `dispatch_refused{reason}` and post-hoc run-record audits.
- **Baseline:** **no baseline — first measurement establishes it.** The failure mode being prevented is measured: a tool-restricted card silently voids `--json-schema` at exit 0, `subtype: success`, with `structured_output` absent [measured, F20, n=1 configuration].
- **Target:** the dispatch-time and post-hoc counters stay at zero; every configuration defect is a `doctor_rule_failed`.
- **Saturation, declared:** if the rule table is complete the dispatch-time counter saturates at zero and carries no information. It is tracked anyway, because a single non-zero value is a **gap in the rule table** — the mechanism class the programme found discriminating.
- **Sampling:** continuous. **Unlike every criterion in this area, this measure reads runtime telemetry.**
- **Kill:** any configuration-class defect observed at dispatch time that a rule could have caught.

**SM-029 — Lint precision and silent guesses** · *starts in its own kill state*
- **Instrument:** lint run output adjudicated against the author's own corrections; two counters, `lint_finding{real|false}` and `lint_ambiguity_resolved_silently`.
- **Baseline:** **13 real / 34 false = 27.7% precision** on one authoring pass [measured, F32, 10 activities]; and **5 guesses accepted silently** across 27 ambiguities, 20 of them new, on a second pass [measured, F33]. **n=2 passes, one operator. Wilson interval not computed.**
- **Target:** silent guesses **0** (hard — AC-107); precision above 50%, i.e. real findings outnumber false ones.
- **Sampling:** every lint run against the author's corrections.
- **Kill:** precision that stays below ~30% as the catalog grows makes the lint author-hostile and rules are removed, not tuned. **The kill line is already met at baseline (27.7% < 30%):** this measure starts in the kill state, and the first honest action on it is to re-measure the current rule set, not to declare a regression.

**SM-030 — First-run lint errors per activity file (authoring ergonomics)** · *discriminator, confounded*
- **Instrument:** lint run on the author's first submission of each file, `errors / file`.
- **Baseline:** **4.7** (F32, P9), **1.3** (F33, P9b), **0.11** (F34, P6) [measured]. **n=3 passes, one operator, and confounded:** the P6 author read the lint before authoring, which F34 states explicitly. The 43× improvement is therefore **not attributable to the lint itself**.
- **Target:** ≤0.5 for a new author with the lint documented.
- **Sampling:** every authoring pass; report per author and **say whether the author had read the lint**.
- **Kill:** a first-time author (lint documented, lint not pre-read) above **4.7 errors/file** — the P9 level — means the documentation is not carrying the schema and the rules teach nothing; the response is to cut rules, not to add documentation. Below that the measure is uninformative at one operator.

**SM-031 — Activity verbosity** · *tripwire; never at risk in three passes*
- **Instrument:** non-blank line count per shipped activity file.
- **Baseline:** median **56.5** (F32) · **73** (F33, +29% over F32) · **53** with sum 5,059 over 90 activities (F34) [measured, n=3 authoring passes, one operator].
- **Target:** median ≤90; no file over 150 — the P9 §4 kill line, which did not fire in three attempts.
- **Sampling:** every catalog change; report the distribution and the max, never the mean.
- **Kill:** a median above 150 says the schema rules are not producing authorable content.
- **Note:** the +29% between two passes on the same schema is the honest variance floor — a 20% move between passes is noise, not a trend. With the highest measured median at 73 against a 90 target and a 150 kill, **the target has never been at risk**; this is a tripwire for a future author, not a live constraint.

**SM-032 — Quality-block coverage** · *no valid baseline*
- **Instrument:** lint — `activities_with_quality_block / delivered_artifact_activities` — plus the registered-exemption count.
- **Baseline:** **no valid baseline.** 42 of 90 = 46.7% of *authored* activities carried a quality block [measured, F34], but **that denominator is the authored set, not the delivered-artifact subset this measure names**, so it is not a baseline for this measure at all. True starting coverage is unknown until the subset is computed. Wilson interval not computed.
- **Target:** 100% of delivered-artifact activities carry a block or a registered exemption id.
- **Sampling:** every catalog change.
- **Kill:** an exemption registry that grows faster than the block count means the exemption is being used as the migration path's terminus rather than its start; at that point the rule enforces paperwork, not quality, and is cut.
- **Note:** coverage is **presence only**. Adequacy is a human gate (AC-115), because presence has been measured not to imply sufficiency — F29's 3/10 approvals of a reproduced regression, F31's four defects through 1,890/1,890 green.

**SM-033 — Binder ambiguity and unresolved-role rate** · *no baseline*
- **Instrument:** `doctor_rule_failed{rule="ambiguous_role"}` and `{rule="unresolved_role"}` per config change; count of distinct (role, toolchain) pairs.
- **Baseline:** **no baseline — first measurement establishes it.**
- **Target:** near-zero after a repo's first configuration; a repeat offender indicates a pack, not a user, error.
- **Sampling:** every `doctor` invocation, tagged by repo and pack-set digest.
- **Kill:** ambiguity that recurs as toolchains are added falsifies "most specific selector wins" as a sufficient resolution rule and forces ADR-0073's toolchain-keyed artifact kinds to extend from mixed-toolchain items to card resolution generally.

**SM-034 — Schema amendment rate per authoring pass** · **TRACKER: no target line, no kill condition**
- **Instrument:** the amendment ledger in the activity-graph schema; amendments opened per 100 activities authored.
- **Baseline:** the P6 pass opened amendments **26–43** while authoring 90 activities [measured, F34]. **Reading that span as 18 amendments = 20 per 100 is arithmetic on an id range the brief does not gloss [inferred]; if 26–43 is a count range rather than an id range the figure is wrong. n=1 pass** — an earlier "n=3" conflated this with F33's 27 ambiguities (20 new), a different metric.
- **Target:** **none.** The prediction that amendments would fall was **falsified at n=3** (F33 vs F32), so no trend is established.
- **Sampling:** every authoring pass.
- **Why kept:** a rate that does not fall by the third *independent* author would refute "the schema is stabilising", a load-bearing assumption of the v0 catalog. Until an independent author exists this row records data and gates nothing — **do not cite it as assurance.**

**SM-035 — Rank-monotonicity violations** · *mechanism alarm; a known violation exists*
- **Instrument:** the lattice CI monotonicity check (AC-105), `monotonicity_violation{family, facet_vector, rigor_pair}`.
- **Baseline:** **at least one violation exists today** — escalating to `mandatory` with `has_repro` selects `bisect`, which has no adversary step and no policy step [measured mechanism, F37 / brief §C.10]. The full cross-product has not been enumerated, so **the count is a lower bound of 1, not a rate.**
- **Target:** **0**, enforced as a build failure.
- **Sampling:** every catalog change; report each violating cell, not a count.
- **Kill:** violations that cannot be removed without collapsing the ladder mean rigor is not a single ordered axis and ADR-0020/0023's deontic enum needs a second dimension.

**SM-036 — Shipped catalog size and per-activity usage** · *count half measured; usage half has no baseline*
- **Instrument:** shipped activity count from the loader; `activity_dispatched{id}` counts over a rolling 90 days.
- **Baseline:** **102** activities in the lattice corpus and **90** authored in the P6 pass [measured, F35/F34], against a curated ~95-activity v0 target [documented, ADR-0027]. **The usage half has no baseline: no dispatch telemetry over a 90-day window exists, and it cannot fire until the system has run for a quarter.**
- **Target:** shipped count within the declared ceiling; every shipped activity dispatched at least once per 90 days.
- **Sampling:** continuous for dispatches; per release for the count.
- **Kill:** an activity never dispatched in 90 days moves to the unshipped extended file — ADR-0027's EPF-rot rationale given a measurable trigger.
- **Comparator warning:** no external comparator exists for catalog size. Cloudflare's 0.6% break-glass rate is a rarity threshold for an *escape hatch* and must not be borrowed as a catalog-size line.

### 1.4 Typed artifacts and the store — layout C, rendering, drift (SM-037..SM-046)

Ten measures. Three have no baseline (SM-037, SM-043, SM-046). Three have targets that essentially cannot be missed (SM-039 saturated, SM-041 at 2.8× slack, SM-045 unconstructible-by-design) — the area really has about six measures whose target can fail.

**SM-037 — Structured-section drift rate** · *no baseline; establishes the D6 boundary*
- **Instrument:** `structured_drift` event, per artifact kind and per stage.
- **Baseline:** **no baseline — first measurement establishes it.** The nearest measured neighbour is a *different quantity*: F19's **world** drift on human gates, 12/174 = **7%**, 3/95 = 3% at ≤1 h vs 9/79 = **11%** over 1 h [measured, one operator, one repo]. Intervals not computed here; **a void is not a miss.**
- **Target:** **no target in Phase 1** — the number is being established. A rate above F19's 7% would mean humans edit rendered sections more often than the world moves under a gate, which is the signal that layout C's prose/structure boundary sits in the wrong place.
- **Sampling:** every reconcile; a rate with its denominator (reconciles), broken out by artifact kind.
- **Kill:** a per-section structured-drift rate that **has not fallen after 20 items** (the rolling window SM-038 uses) — humans keep editing the same rendered section — falsifies the D6 boundary for that section and moves the field to opaque prose or to a CLI command. *(The kill previously read "over the first N items" with N unset; a kill with a free parameter cannot fire.)*

**SM-038 — Produced-but-never-read artifacts** · *the baseline is the pathology, not the metric*
- **Instrument:** `artifact.read` events against declared stage outputs; the `redkiln doctor` produced-unread rule.
- **Baseline:** **110/110 `plan.md` files were the unfilled scaffold** [measured, S-MIG census, `spikes/results/s-mig-report.md`; denominator stated; one repo, one frozen SHA]. That is the pathology this measure exists to make visible. **`plan.md` itself is removed by AC-145, so for the remaining kinds there is no baseline** — first measurement establishes it.
- **Target:** zero artifact kinds with a produced-unread rate at 1.00 over a rolling 20 items.
- **Sampling:** every item completion; per artifact kind, with its denominator.
- **Kill:** an artifact kind never read across 20 items is a candidate for removal from the pack — **but the removal is a human pack edit, never automatic (AC-148)**, because the selection engine is blocked by F36.

**SM-039 — Schema conformance of harness output** · *saturated canary; no kill*
- **Instrument:** kernel boundary validation result per dispatch; `schema_conformance` event.
- **Baseline:** **52/52 = 100%** under the typed contract [measured, F31]; `repro.command` fill 147/147 [measured]. **Wilson two-sided 95% lower limit ≈ 0.931** (computed here; the one-sided 95% lower limit is ≈0.951 — an earlier revision mislabelled the two-sided bound as one-sided).
- **Target:** 100%. **This metric is saturated and is tracked anyway.** It is not a discriminator; it is retained as the regression canary for the F20 class, where a tool-restricted card silently voids `--json-schema` at exit 0 with `subtype: success`, so a drop from 1.000 is the only visible symptom of a card or platform change.
- **Sampling:** every dispatch carrying a schema. **Fire condition: any single non-conformance**, not a rate change — that is the only thing this measure can miss.
- **Kill:** **none.** A saturated canary has no kill; if it ever costs anything to collect, it is dropped instead.

**SM-040 — Cost of the typed contract** · *discriminator; one paired point*
- **Instrument:** `total_cost_usd` per review dispatch, arm-tagged.
- **Baseline:** S-TYPED **$112.25** against a prose comparator **$64.61 = +74%** [measured, F31 — **one paired comparison, one draw per arm; there is no measured range**]; per-review medians **$3.03** and **1,071 s (~18 min)** on CLEAN, slowest 4,561 s [measured]. Nearest independent cost ratio in the programme: E3's a/b arms at **1.72×** (F23), a different comparison. *(A claimed "+40% to +101% band already measured" and a "measured ceiling (+101%)" were **cut**: the brief contains exactly one typed-vs-prose figure, +74%, and neither 40 nor 101 appears anywhere in it. This was the most damaging fabrication found in the area — a kill threshold set against a measurement that does not exist.)*
- **Target:** the premium stays at or near the single measured point (+74%). **The first repeat measurement sets the band, because one draw per arm is not a range.**
- **Sampling:** every execution-grade review; report the distribution, never a mean alone — F31's slowest draw is 4.3× its median.
- **Kill:** a premium materially above +74% with detection unchanged returns the render/parse choice to a cost decision — it does not by itself license parsing prose, which AC-120 forbids on other grounds.
- **Weakened, disclosed:** "a median review under Anthropic's $15–25 comparator" is a slack bound, not a target: the measured median is $3.03, about 5× under it, and cannot be missed without a fivefold regression.

**SM-041 — Schema and template authoring size** · *guardrail, not a discriminator*
- **Instrument:** non-blank line count per shipped type/template file, recorded at merge.
- **Baseline:** median **56.5** lines, author 1 (F32) · median **73**, author 2, **+29%** (F33) · median **53**, author 3 (F34, sum 5,059 over 90 activities) [all measured, zero model dispatches, three authoring passes by one operator]. Pre-registered kill line **>150**; it did not fire, three times, and F34 clears it by **2.8×**.
- **Target:** median under 150 non-blank lines per type — **a guardrail that cannot plausibly be missed**, detecting only a regime change. The second target — a second author's median within 30% of the first's — is **calibrated on the single observed pair (+29%)** and would be missed by any spread wider than the one already seen: a tripwire on n=2, not a validated tolerance.
- **Sampling:** every merged type or template; report median and max with n.
- **Kill:** a median above 150 says §4.4's schema rules (flat, closed, no nested unions) are not producing authorable types — **the design rules are wrong, not the authors.**

**SM-042 — First-run lint error rate on authored schemas** · *three observations, not a trend*
- **Instrument:** lint errors per file on the first run, before any fix.
- **Baseline:** **0.11 errors/file** when the author read the lint first (F34, 90 activities) · **1.3** (F33) · **4.7** (F32) [measured]. **n=3 authors, one author per point;** the spread is confounded with author *and* with whether the lint was read first.
- **Target:** under 1.0 errors/file for any author with the lint documented.
- **Sampling:** first CI run on each new type/template PR.
- **Kill:** a rate that does not fall as the lint documentation improves means the lint encodes rules authors cannot apply, and the rules move into the schema derive rather than the lint.
- **Caution from F33:** that lint "accepted 5 guesses silently" — a falling error rate is not evidence of correctness, only of conformance to what the lint checks.

**SM-043 — Section-locator resolution failures** · *no baseline; no spike measured the locator*
- **Instrument:** locator error events by kind (zero-match, duplicate-match, decoy-adjacent) per reconcile, **each carrying the document's byte length and heading count**.
- **Baseline:** **no baseline — first measurement establishes it.** No spike measured the locator; §4.4 asserts only that it is "tested as such".
- **Target:** zero fuzzy resolutions by construction (AC-151), and a duplicate/zero-match rate low enough that humans are not routinely blocked. **The acceptable rate is not set by evidence and must not be invented.**
- **Sampling:** every reconcile that resolves an anchor; report count and kind with the document that produced it.
- **Kill:** a duplicate-match rate that **rises with document length** means heading anchors are not unique enough as a locator key and the anchor gains an explicit id — it does **not** license promoting the locator to a grammar. *(Length and heading count were added to the instrument during refutation; without them this kill could never fire.)*

**SM-044 — AC completeness of shipped specs** · *discriminator on a policy question*
- **Instrument:** `AcceptanceCriterion` field presence at spec seal — `gwt` present, `verification` present, discriminating test recorded.
- **Baseline:** **19 of 153 shipped specs (12%)** write criteria without GIVEN/WHEN/THEN or a verification note [measured, S-MIG census, one corpus, one frozen SHA]; the F25 per-AC evidence columns (Assertion / Mutation / Test-run / Red?) exist in **0 of 156** implementation reports [measured, **S-MIG census item 3** — *not* F25 and *not* §4.6, as an earlier revision claimed].
- **Target:** the 0/156 figure moves off zero. **The 12% figure is not targeted to zero** — the type accepts these criteria deliberately (AC-137) and the *policy*, not the schema, decides what is blocked (AC-138).
- **Sampling:** every sealed spec; the three presence rates reported separately with denominators.
- **Kill:** if the missing-verification fraction stays at 12% **and** the missing-verification policy row is set to block, the policy is blocking one spec in eight on a condition the corpus shows is normal — **the policy row is wrong, not the specs.**

**SM-045 — Free-form gate outcomes reaching an artifact** · *construction check, not a discriminating measure*
- **Instrument:** count of gate sections rendered from a typed record, against any gate outcome present without one.
- **Baseline:** gate results were free-form in **39 of 156 implementation reports (25%)** and **19 of 23 reviews (83%)** in the migrated corpus [measured, S-MIG census; **the review figure rests on 23 files**, a share of a small denominator, and no interval is computed].
- **Target:** **zero** untyped gate outcomes after cutover — a hard-zero measure, since AC-141/AC-142 make the alternative unconstructible. The metric exists to prove the construction holds in production, not to trend.
- **Sampling:** every rendered artifact; alert on any single occurrence.
- **Kill:** a non-zero count means some path is still synthesising a gate outcome outside the toolchain adapter — an architecture violation that is fixed, not tuned.

**SM-046 — Unmigratable stored artifacts** · *no baseline; guards ADR-0006's premise*
- **Instrument:** `doctor` missing-migration rule count, by type and version pair.
- **Baseline:** **no baseline — first measurement establishes it.** The named prior failure is the old redkiln's `template_sig`/`rendered_sig` drift [documented, §4.4], which was never counted.
- **Target:** zero at every release tag; a non-zero count between tags is permitted only for the duration of a migration PR.
- **Sampling:** every `doctor` run and the release CI job.
- **Kill:** a repository that cannot be brought to zero without hand-editing stored artifacts means schema evolution is not in fact a migration and `schema_version` is decoration — the exact old-redkiln failure ADR-0006 exists to avoid.


### 1.5 Git as truth — trailers, `reconcile()`, worktrees, disposable clones, merge queue (SM-049..SM-058)

Ten measures. Five have no baseline (SM-050, SM-051, SM-054's clone cost, SM-055, SM-057). The whole area's empirical spine is **one operator, one repository**.

**SM-049 — Drift rate at `reconcile()`** · *discriminator on catch-rate, not on the world's rate*
- **Instrument:** `reconcile_started` / `drift_detected` events, drift class, gate-open duration.
- **Baseline:** **12/174 = 6.9%** overall; ≤1 h gaps **3/95 = 3.2%**; >1 h gaps **9/79 = 11.4%**, a 3.6× ratio [measured, F19, n=174, **one operator, one repository**]. Wilson 95%: overall **[4.0%, 11.7%]**; ≤1 h **[1.1%, 8.9%]**; >1 h **[6.1%, 20.3%]** — **the two conditional intervals overlap, so the 3.6× ratio is a point estimate, not a separated effect.** *(Intervals and ratio recomputed during refutation and confirmed exact.)*
- **Target:** **no target on the rate itself** — it is a property of the world, not of the product. The tracked target is that **100% of drift is caught at entry**, i.e. zero drift discovered after an action was taken.
- **Detection rule (added, because the target was otherwise unmissable):** drift is counted as *missed* when a later `reconcile()` records a drift observation whose causing commit is older than the timestamp of an action already taken. Without this rule there is no oracle for missed drift. **It relies on commit timestamps, which a rewrite can forge; it closes the hole, it is not tamper-proof.**
- **Sampling:** every entry; distribution by drift class and by preceding gap duration, never a pooled mean.
- **Kill:** any drift meeting the detection rule falsifies the git-first claim and returns `reconcile()` placement to a design decision.

**SM-050 — Orphan and rewrite volume** · *no baseline*
- **Instrument:** `rewrite(old->new, mechanism)` event; orphan-row count per entry.
- **Baseline:** **no baseline — first measurement establishes it.** The corpus offers only a different quantity: 360 trailer commits over 74 slices, with **15/74 = 20.3% ever-rejected** and **21.6% sealed more than once** [measured, F12]. **That is a rejection/reseal rate, not a history-rewrite rate, and is not a bound on either.**
- **Target:** every orphan resolves to a `rewrite` event or to an opened human gate (AC-171); the count of orphans that are neither is **0**.
- **Sampling:** every entry. Report orphan age (entries survived) as a distribution — a long-lived orphan is the signal.
- **Kill:** a steady-state orphan population growing monotonically over 10 consecutive entries means re-anchoring is under-powered and ADR-0018's "surfaced, never deleted" rule is producing an unusable backlog rather than an audit trail.

**SM-051 — Rebuild fidelity and cost** · *no baseline*
- **Instrument:** `doctor --rebuild` exit status, divergence count, wall clock.
- **Baseline:** **none. No prior measurement of rebuild exists.** The corpus (360 trailer commits, 74 slices, F12) is the substrate the first measurement will run against, and nothing more. *(A claim that `reconcile.rs` was "rewritten in the spike and validated on 360 trailer commits / 74 slices" was **cut as fabricated**: the corpus is real, the validation of an implementation against it is not.)*
- **Target:** divergence **0** on every CI run; wall clock reported and bounded once the first measurement exists.
- **Sampling:** every CI run on the pinned corpus, plus any operator-invoked rebuild.
- **Kill:** a non-zero divergence means tier-1 state is not fully in git — the tier-1 test has been violated and the offending field must move or be re-tiered.

**SM-052 — Resume cost** · *discriminator; duration-invariance currently unfalsifiable*
- **Instrument:** `cache_creation_input_tokens` on the first dispatch after a resume.
- **Baseline:** mid-graph resume **8,408 / 10,314 / 14,579** = **0.45× / 0.55× / 0.78×** of the 18,603-token stage-gate resume [measured, F38, **n=3, PARTIAL**: 3 of a pre-registered 27+3, $3.01; the 65-minute and 24-hour arms and all three controls are unrun]. Stage-gate comparator: 18,603 at both 65.2 min and 49.7 h under the default prompt; 29,619–29,680 under a static card prompt at ~66 min [measured, F28 as corrected, and the S-H1b numbers in the never-written F30 slot].
- **Target:** at most one prefix rewrite per resume, invariant in the length of the interruption.
- **Sampling:** every resume; a distribution, never a mean.
- **Kill:** pre-registered at **>1.5×** the stage-gate cost — **did not fire at n=3, and n=3 is too small for the absence to mean much.** A resume cost that **grows with the gap** would falsify the mark-and-read design; that cannot yet be asserted at all, because the long-duration arms are unrun. **The duration-invariance half of the target is unfalsifiable in this programme's data** and stays open until the 65-min and 24-h arms run.

**SM-053 — Worktree cache cost** · *the weakest quantitative target in the area*
- **Instrument:** cache-read share on the first dispatch in a newly created worktree.
- **Baseline:** worktree-each steady share **0.685** — 31.5% of the prefix re-written per worktree — across 50 dispatches, within-arm range ≤0.001 [measured, F16]; S-WT: **w1 0.807, w2–w10 all 1.000** [measured, **F30 slot — F30 was never written**; these numbers exist only in the design recommendation and the proposal]. Prefix structure: ~17,439 shared + ~7,827 per-directory, 19/19 fresh repos identical [measured, F13] — which is *why* the worktree penalty is ~31% and not 100%.
- **Target:** share **≥0.99** from the second worktree onward under a static `--system-prompt`.
- **Sampling:** every first-dispatch-in-a-new-worktree.
- **Note:** the `--system-prompt` **×** worktree combination is **UNRUN**, so the target is a prediction, not a measured expectation. Do not infer it from F16's "held across commits": F28's correction shows a static prompt **forfeits the account-wide subsidy across time** (29,619–29,680 created at ~66 min, share 0.000). **The two mitigations point in opposite directions** (brief §C.1). *(This cell is contested — see §3 and SM-113.)*
- **Kill:** a measured second-worktree share **below 0.685** — worse than taking no mitigation at all — means the static-prompt route costs more than it saves and the worktree-per-item plan must be repriced against §4.7's parallelism budget before Phase 1 exits.

**SM-054 — Review-clone isolation** · *saturated invariant; one occurrence kills*
- **Instrument:** prepared-clone `rev-list --count --all`, reachable-object count, clone wall time; plus the count of reviews whose findings traced to content outside the reviewed SHA.
- **Baseline:** **no baseline for clone cost.** The only prior datum is a defect: in E3 the reviewer restored the fix from the object store, spending 42 turns and 22,853 output tokens over 526 s and returning `approved` [measured, F17, n=1, recorded as an instrument defect]. Review cost comparators: S-TYPED CLEAN median **$3.03** at **1,071 s**, slowest 4,561 s [measured, F31]; Cloudflare read-only full tier $1.68; Anthropic Code Review $15–25 [documented].
- **Target:** the prepared clone holds exactly the depth-1 object set at the reviewed SHA on 100% of reviews; out-of-SHA content recoveries **0**.
- **Sampling:** every review dispatch.
- **Saturation label:** expected to sit at 0 / 100% permanently, so **this is not a quality score.** It is retained because F17 shows the failure is silent and looks like success (`approved`, exit 0); AC-191's negative control is the only thing that would surface it.
- **Kill:** **one occurrence.** A single review whose findings trace to content outside the reviewed SHA voids every review verdict issued under that clone configuration and reopens the isolation design. There is no acceptable non-zero rate.

**SM-055 — Merge-queue re-verification rate** · *no baseline*
- **Instrument:** `verify` events keyed by tree hash; merged-tree against verdict-tree comparison.
- **Baseline:** **no baseline — first measurement establishes it.** The nearest historical anchor is that 21.6% of corpus slices were sealed more than once [measured, F12] — a different quantity that must not be read as this one.
- **Target:** 100% of merges are sealed against a verdict at the merged tree hash. **The re-verification rate itself has no target;** only the count of merges sealed on a stale verdict, which must be 0.
- **Sampling:** every merge.
- **Kill:** any merge sealed on a verdict issued against a different tree hash falsifies AC-197 and reopens the merge-queue design.

**SM-056 — Tier-1 recoverability in production** · *saturated; reports byte-recoverability only*
- **Instrument:** a scheduled `doctor --rebuild` into a temporary path, diffed against the live store's tier-1 projection.
- **Baseline:** suspension is lossless **174/174** at activity granularity [measured, F19] and **3/3** at mid-graph step granularity across a real process kill — 0 changed, 0 lost, digests identical [measured, F38, n=3, PARTIAL]. **The stronger claim is not measured:** F19 notes explicitly that the FSM was never replayed, so "same next action" is unproven. AC-188 is the instrument that would establish it; **until it exists this measure reports byte-recoverability, not decision-equivalence.**
- **Target:** 0 divergences; every divergence triaged to a specific field that must move to tier 1.
- **Sampling:** scheduled — per CI run on the corpus, periodically in a live repository.
- **Kill:** a recurring divergence in the same field is a tier misclassification, not a bug in rebuild.

**SM-057 — Store-open refusals** · *saturated canary; the kill is the canary going quiet*
- **Instrument:** typed open errors (`WrongRepository`, `SchemaVersionMismatch`), counted.
- **Baseline:** **no baseline — first measurement establishes it.** The design rationale is external: Backlog.md #558 [documented, **not in the evidence brief**].
- **Target:** refusals in normal operation **0**; refusals in the fixture suite **exactly one per fixture**.
- **Saturation label:** expected to saturate at 0 in production and therefore carries no information as a production number. It is retained only as a canary with a mandatory failing fixture — an open that stops refusing is visible only because something is trying it.
- **Sampling:** continuous in production; every fixture run in CI.
- **Kill:** **the fixture suite recording zero refusals** — the canary has stopped firing and the identity check is dead code. (A production refusal is a diagnosis, not a kill: it means someone pointed a binary at the wrong repository.)

**SM-058 — Gap duration preceding an entry** · *covariate, not a target*
- **Instrument:** `gate_opened` → `answered`, and last-entry → this-entry deltas.
- **Baseline:** gate→verdict **n=174**, median **10.5 min**, 45% over 1 h; stage→verdict **n=191**, median **51.6 min**, p90 ~9 h, max **226 h (9.4 days)**, 48% over 1 h [measured, F18, zero model cost, **one operator**].
- **Target:** **none** — this is the covariate SM-049 and SM-052 are conditioned on, not a thing to optimise.
- **Sampling:** every entry; p50/p90/max, never a mean. **Within stage→verdict the max is ~260× its own median.** *(A previously stated "1,300× the median" divided the stage→verdict max by the gate→verdict median — a ratio between two different series — and is withdrawn.)*
- **Kill:** not a kill metric. If the production gap distribution collapses to minutes, SM-049's >1 h stratum loses its denominator and the drift-rate conditioning must be re-derived rather than carried over.

### 1.6 Harness port and the `claude -p` adapter (SM-061..SM-070)

Ten measures. Two have no baseline (SM-069, SM-070). SM-063's baseline is a borrowed placeholder from spike batches that measured a different system under a different driver.

**SM-061 — Child-verified isolation** · *saturated canary; its value is the date it breaks*
- **Instrument:** `system/init` tool and MCP-server count on the dispatch event; `usage.cache_creation_input_tokens`.
- **Baseline:** a default `claude -p` inherited **12 MCP servers and ~280 tools**; under `--strict-mcp-config` the cache write fell **8,243 → 3,611** and tools **280 → 34**, with MCP accounting for **4,631 tokens = 56%** of the write [measured, F8; **single operator environment, one fixture, n=1**].
- **Target:** reported effective tool set equals the card's declared set on 100% of dispatches.
- **Sampling:** every dispatch, read from the child's own `system/init` — because an ignored flag looks identical to an honoured one from the parent's side (brief §C.11).
- **Saturation:** expected to sit at 100% once AC-202/AC-204 hold. **This is a saturated metric and is not a result.** It is kept as the canary for a CLI upgrade or an environment change moving underneath the pin: its value is the *date it breaks*, not its level.
- **Kill:** a contamination share above F8's 56% recurring after the flag is honoured means isolation is not achievable from the CLI, and the cost model built on the isolated 3,611-token write is wrong.

**SM-062 — Structured-output fail-open count** · *mechanism alarm*
- **Instrument:** dispatches carrying a schema where `structured_output` is absent, over schema-bearing dispatches.
- **Baseline:** **100% failure for the one card configuration that omitted `StructuredOutput`** — exit 0, `subtype: success`, no `structured_output` [measured, F20, **n=1 configuration**]; 0 observed once the tool was added. **Wilson interval not computed: the denominator is one configuration, not a sample.**
- **Target:** **0.** Any non-zero value is a defect in AC-205/AC-206, not a rate to manage.
- **Sampling:** every schema-bearing dispatch; report as a count with the offending card ids, never as a percentage.
- **Kill:** a non-zero count *with AC-205 passing* means the pre-spawn check and the runtime assertion disagree — the card model does not describe effective tools, and adoption verification (AC-204) is unsound.

**SM-063 — Dispatch outcome class mix and void rate** · *borrowed placeholder baseline*
- **Instrument:** the `StepError` variant on every terminal harness event.
- **Baseline:** three spike batches explicitly report zero void — P1 arm L **400 dispatches, 0 void** [F36], S-VAR **60 draws, 0 void** [F29], P7 **61 runs, 0 void** [F37]. **These units are not commensurable:** a P7 "run" is a graph run spanning many dispatches (61 runs produced 297 records) and an S-VAR "draw" is a review, so the pooled denominator of 521 is heterogeneous and its pooled Wilson upper bound (≈0.7%) is **indicative, not a dispatch-level rate**. The dispatch-level figure the brief actually supports is **F36's 0/400, Wilson upper ≈0.9%**. **P8 is not counted:** F38 reports 0 changed and 0 lost across 3 runs and reports no void count at all.
- **Target:** zero void dispatches; `Platform` and `Transport` shares reported as a distribution per model family.
- **Sampling:** every dispatch. **Re-derive a true dispatch-level baseline from the first 400 production dispatches and replace this line** — the spike pool is a placeholder, not a measurement of this system.
- **Kill:** a void rate whose Wilson **lower** bound exceeds 0.9% falsifies the assumption that spike-grade reliability carries into production, and forces the retry budget to be re-derived from data rather than from the pack's N.

**SM-064 — Static-prefix cache stability** · *discriminator; the two mitigations conflict*
- **Instrument:** `usage.cache_read_input_tokens` share and `cache_creation_input_tokens` per card.
- **Baseline:** share **1.000** across commits (50 dispatches, within-arm range ≤0.001) [measured, F16]; across worktrees **w1 0.807, w2–w10 1.000** [measured, S-WT]; **across time the static prompt forfeits the account-wide subsidy** — share **0.000** with **29,619** created at 66.1 min [S-H1b] and **29,680** at 66.8 min [F28 correction], **n=1 per gap**; the default prompt was flat at **18,603** at 65.2 min (n=1) and again at 49.7 h (n=1) [measured, F28]. **The 24 h read for the static prompt was not taken.** §C.1 forbids inferring from "held across commits" to "helps across time" — that inference is explicitly falsified.
- **Target:** at most one full prefix rewrite per card per TTL window; share 1.000 for dispatches inside the window.
- **Sampling:** every dispatch, grouped by card digest and TTL window; report as a distribution.
- **Kill:** two rewrites inside one TTL window for a card whose prefix bytes did not change falsifies ADR-0016's static-prompt route and re-opens the append route (S-H1c, registered and unrun) as the default.

**SM-065 — Headroom draw per dispatch and margin refusals** · *batch-only; the binding limit is invisible*
- **Instrument:** `rate_limit_event.unifiedWindows.five_hour.utilization` delta; count of pre-spawn `RateLimited` refusals.
- **Baseline:** **0.0098 `five_hour` per CLEAN draw** and **0.0017 `seven_day`** [measured, F29, 60 draws]; **F21's earlier ~0.077 is superseded and was 7.8× too high.** Instrument resolution is **0.01**, so single-dispatch deltas are not measurable and every reading carries a `resolution_limited` flag [measured, F6]; batch deltas are the reportable form. There is **no Opus/Sonnet family sub-limit key**, so the binding limit is invisible [measured, F1] and `five_hour` is a **proxy** for it.
- **Target:** zero dispatches entering overage; refusals occur at the margin, not at the limit.
- **Sampling:** batch delta per run, never per dispatch.
- **Kill:** an overage entered despite a ≥2-tick margin means `five_hour` is not a usable proxy for the binding limit and throttling must move to the per-item token appetite alone.

**SM-066 — Wall clock against configured timeout** · *discriminator; p99 is not estimable*
- **Instrument:** dispatch duration against the activity's timeout; count of `Timeout` errors.
- **Baseline:** S-TYPED CLEAN review median **1,071 s (~18 min)**, slowest **4,561 s** (51 valid) [measured, F31]; E3 CLEAN 317–2,055 s and S-SILENT 97–424 s [measured, F21/F23]; timeout history 900 s — **a default written for the fast condition, which cost a CLEAN record** — → 2,400 → 3,600 → one 5,400 s retry [measured, operational].
- **Target:** **recorded maximum below 0.6 × the configured timeout** for each activity class, and zero records lost to a timeout. **A p99 target is not stated:** no class in the brief has an n at which p99 is estimable (the largest is 51 valid), and p99 becomes the reported statistic only once a class has n ≥100.
- **Sampling:** every dispatch; per class as a distribution with the max, never a mean alone.
- **Kill:** any record lost to a timeout repeats the 900 s defect and moves timeout selection out of pack configuration into a derived-from-history value.
- **Limit:** the floor is the recorded maximum from an event store that starts empty, so on a fresh class the constraint is satisfied by the first draw — near-zero protection in exactly the window in which the 900 s defect occurred.

**SM-067 — Cost per dispatch by role** · *discriminator*
- **Instrument:** `total_cost_usd` on the dispatch event, grouped by card.
- **Baseline [all measured]:** trivial haiku dispatch **$0.021–0.027** (F3); E3 S-SILENT **$0.45–0.64**, E3 CLEAN **$1.06–4.89** (F21/F23); S-TYPED CLEAN review median **$3.03**, the typed contract costing **+74%** over prose at the arm level ($112.25 vs $64.61, F31); P7 graph run optional **$1.02**, recommended **$2.69** (F37). **P7's mandatory arm is excluded from the baseline:** n=1 at $9.86, the arm was dropped, the decisions log flags it as pricing an unreachable configuration, and its cost ratio is unreconciled (3.66× in F37 vs 4.4× in the decisions log).
- **Comparators [documented]:** Cloudflare read-only median $0.98, full tier $1.68, break-glass 288 of 48,095 MRs = 0.6%; Agentless $0.70/instance at 32.00% vs CodeR $3.34 at 28.33%; Anthropic Code Review $15–25.
- **Target:** median review-grade dispatch at or below **$3.03** — the measured S-TYPED median, i.e. **a no-regression target, not an improvement target**.
- **Sampling:** every dispatch; a distribution per card, with the comparator line drawn on the chart.
- **Kill (two conditions, the first of which can fire near the target):** a review-lane median above **$6.06** (2× baseline) sustained over 20 dispatches forces the per-item appetite fixed at triage to be re-derived before more work is dispatched; and a median above the Anthropic Code Review floor (**$15**) removes the cost argument for self-hosting the review lane altogether.

**SM-068 — Cancellation cleanliness** · *incident record, not a rate*
- **Instrument:** descendant PID count in the dispatch's process group (on Windows, its job object), sampled after cancel.
- **Baseline:** **~12 kills against one 12,000 s survivor** under `Start-Process`, and `issue-81` killed 3× as a background job but completing in **116 s** in the foreground [measured, F22 — **operational, one machine, one operator, cause unestablished**]. **No production baseline exists; this is an incident record, not a rate.**
- **Target:** zero surviving descendants after cancel; zero dispatches whose result depends on a detached process.
- **Sampling:** every cancel, plus the CI regression test from AC-224.
- **Kill:** a survivor in production re-opens F22's unestablished cause and blocks any design that treats cancel as reclaiming the account's headroom.

**SM-069 — CLI version drift** · *no baseline*
- **Instrument:** `system/init` version against `TESTED_CLI_VERSION`, recorded per event.
- **Baseline:** **no baseline — first measurement establishes it.** The platform publishes no stability policy [documented], and F1/F4 measure two undocumented events we already depend on. F13 measured a shared prefix segment of **~17,439 tokens identical across 19/19 fresh repos** [measured]; **that this segment is account-wide, and therefore that a CLI upgrade anywhere on the account invalidates it, is [inferred]** from F13 plus §C.1's account-wide-subsidy language — not measured.
- **Target:** 100% of harness events carry a version; drift detected within one dispatch of an upgrade.
- **Sampling:** every dispatch.
- **Kill:** drift detected only after a cost anomaly, rather than at the assertion, means AC-226 is not firing and the pin is decorative.

**SM-070 — Pre-spawn refusals** · *no baseline*
- **Instrument:** count of configuration-class refusals (`NotAdopted`, `NotIsolated`, `Unsupported`, `Overage`, missing model/effort, missing `StructuredOutput`) raised before any child is spawned.
- **Baseline:** **no baseline — first measurement establishes it.** The precedent is that these failures were previously *invisible*: F20's card silently exited 0 with `subtype: success`, and F7's `--agents` was inert with no error at all [measured].
- **Target:** every configuration defect surfaces here at zero model cost rather than as a spent dispatch. Report as refusals per 100 dispatch attempts **alongside the same errors caught after spawn — the second number should be zero.**
- **Sampling:** every dispatch attempt; `doctor` reports the same rules statically.
- **Kill:** a configuration-class error observed after spawn that `doctor` could have caught means the rule table and the runtime assertions have diverged, and the "zero model cost" claim for configuration checking is false.


### 1.7 Human gate — a durable mark, not a waiting process (SM-073..SM-082)

Ten measures. **Four are discovery instruments, not success measures** (SM-074, SM-078, SM-081, SM-082): they carry no target and cannot fail, and must not be counted as assurance. Of the six that can fail, two saturate at zero by construction (SM-076, SM-077) and one is already saturated at both granularities (SM-080). **The genuinely discriminating count is three: SM-073, SM-075, SM-079.**

**SM-073 — Mid-graph gate resume cost** · *discriminator*
- **Instrument:** the `gate_resumed` event, `cache_creation_input_tokens`, `total_cost_usd`.
- **Baseline:** **8,408 / 10,314 / 14,579 tokens = 0.45× / 0.55× / 0.78×** of the ~18,603-token stage-gate resume [measured, F38, **n=3 of a pre-registered 27+3**, $3.01 of a $40 cap, **short-duration draws only**].
- **Target:** the ratio distribution stays at or below **1.0×** at the median. **1.0×–1.5× is a warning band with no automatic action** — three short-duration draws do not license a hard ≤1× bar.
- **Sampling:** every resume; a distribution over the ratio, never a mean (n=3 has no meaningful mean).
- **Kill:** a ratio above **1.5×** — P8's own registered kill, which did not fire at n=3, and the bar AC-266 enforces — falsifies "mid-graph gates are cheaper" and returns gate placement to a cost decision instead of a value decision.
- **Caveat:** P8's 65-min arm, 24-h arm and all three controls are **unrun**. Production is where the duration arms actually get measured.

**SM-074 — Gate open duration distribution** · **DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE**
- **Instrument:** `gate_opened` → terminal event timestamps.
- **Baseline:** gate→verdict median **10.5 min**, **45% over 1 h**, **n=174**; stage→verdict median **51.6 min**, p90 ~9 h, max **226 h (9.4 days)**, **48% over 1 h**, **n=191** [measured, F18; zero model cost; **one operator, one machine, historical corpus**].
- **Target:** **none, deliberately** — duration is free (F28), so a long gate is not a defect. **Kill: none; this measure cannot fail.**
- **Sampling:** every gate; report the full distribution and the fraction over 1 h (the cache-TTL boundary, F10).
- **Why tracked:** it is the *input* to SM-075 (drift scales with duration) and to the expiry-policy question (SM-079). **Do not set a target on it, do not let a dashboard turn it into one, and do not count it as a success measure.**

**SM-075 — Void rate at answer (drift-at-answer)** · *discriminator against a proxy*
- **Instrument:** `gate_answer_void` / (`gate_answered` + `gate_answer_void`).
- **Baseline:** **no direct baseline** — no gate in the corpus was tree-hash bound, so nothing was ever voided. Historical proxy: drift observed on **12 of 174** gates = 6.9% (Wilson 95% **4.0–11.7%**); split by duration ≤1 h **3/95 = 3.2%** (Wilson **1.1–8.9%**) against >1 h **9/79 = 11.4%** (Wilson **6.1–20.3%**) [measured, F19]. **The 3.6× duration effect is directional only — the two intervals overlap.** State it as such and never as an established ratio.
- **Target:** the production void rate tracks the duration-adjusted proxy, i.e. stays inside the proxy's **4.0–11.7%** envelope after adjusting for the observed fraction of gates over 1 h. **A void is never a miss (AC-251):** it is reported in its own denominator alongside approve / changes-requested, never folded into either.
- **Sampling:** every answer.
- **Kill:** a sustained void rate materially above the proxy's upper bound (**>20%**) means tree-hash binding is too coarse for this repo's commit rhythm, and reopens the question ADR-0017 leaves open — only the tree-hash-mismatch case is specified as void; other drift shapes are not enumerated. Path-scoped drift would then need pricing (see SM-081).
- **Known softness:** there is a wide band between the 11.7% envelope and the 20% kill in which the measure is off-target and nothing happens.

**SM-076 — Gate hang count (expired-and-open)** · *invariant alarm; saturates at zero by construction*
- **Instrument:** the `doctor` rule from AC-256; the expected result set is empty.
- **Baseline:** **no baseline — first measurement establishes it;** the design value is exactly zero.
- **Target:** 0, permanently. **Kill: any non-zero result, once.**
- **Sampling:** every `doctor` run and every reconcile.
- **Saturation:** saturates at zero by construction and is kept anyway as an invariant alarm rather than a rate — it is the one thing ADR-0017 forbids outright ("never leave the gate hanging"), and an alarm that only ever fires on a real defect is worth its cost. **Do not report it as a percentage.**

**SM-077 — Model spend while a gate is open** · *invariant alarm; saturates at zero*
- **Instrument:** `dispatch` events and `total_cost_usd` between `gate_opened` and the gate's terminal event.
- **Baseline:** **0 dispatches, $0.00 — [inferred], not measured.** No spike instrumented dispatch count across an open gate. The inference is from F28's mechanism [measured]: a 0.9 min gate showed share 1.000 with creation 0, and the flat 18,603 tokens at 65.2 min and at 49.7 h (one measured point each) are consistent with no work happening in between.
- **Target:** 0. **Kill: any dispatch inside an open gate's interval.**
- **Sampling:** every gate.
- **Saturation:** saturates at zero and is tracked anyway, because the tempting future regression is explicit and named — a keep-the-cache-warm dispatch, motivated by F10's ~1 h TTL against F18's 45–48% of gates over 1 h. **F28 proves such a dispatch buys nothing;** this metric is what would catch someone adding it.

**SM-078 — Answer arrivals per open gate (second-arrival rate)** · **DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE**
- **Instrument:** count of answer submissions carrying a valid token per `gate_opened`.
- **Baseline:** **no baseline — entirely unmeasured, and not measurable by any spike run.** P8 forced arrivals, and its second-arrival criterion is **circular by construction** (F38, P8 §5.4(3)): the mark was minted by the driver that implements the semantics under test. Nothing in F18's 174-gate corpus records arrival multiplicity.
- **Target:** none set. **Kill: none; this measure cannot fail.** First measurement establishes the distribution.
- **Sampling:** every gate; report the histogram of arrivals per gate, not a mean.
- **Why tracked:** it is the *only* route to answering whether a second arrival should mint a new mark (AC-271 is PROVISIONAL precisely because no run can settle it). If second arrivals are effectively never observed, the question closes by irrelevance; if they are common, the semantics need a decision backed by real frequencies.

**SM-079 — Expiry terminal-path mix** · *discriminator against an external comparator*
- **Instrument:** the gate's terminal event, field `outcome ∈ {fail, escalate, auto-decide}`.
- **Baseline:** **no baseline — the corpus had no expiry mechanism, so no gate could expire [inferred: absence of instrumentation, not observed absence];** the longest observed gate was answered at **226 h** [measured, F18]. External comparator: Cloudflare's break-glass path, **288 of 48,095 MRs = 0.6%** [documented], as the order of magnitude for a legitimately-used escape hatch.
- **Target:** `auto-decide` stays at or below the comparator's order of magnitude (~1%); `escalate` is expected to dominate. **The comparator is a different mechanism in a different organisation — it sets an order of magnitude, not a threshold.**
- **Sampling:** every expiry.
- **Kill:** `auto-decide` as the modal terminal path means the gate is not functioning as a gate — the human was routinely absent and the machine routinely decided — and the autonomy level for that stage (AC-275) should have been L3 in the first place. **Re-decide the level rather than tuning the deadline.**

**SM-080 — Resume losslessness** · *already saturated at both granularities; invariant alarm*
- **Instrument:** artifact digest comparison at `gate_resumed`; count of items with changed or lost output.
- **Baseline:** **174/174** lossless at activity granularity (**Wilson 95% lower bound 97.8%**) [measured, F19; one operator] and **3/3** at mid-graph step granularity across a real `taskkill /T /F`, with 0 changed, 0 lost and identical digests [measured, F38, PARTIAL].
- **Target:** 100%. **Kill: a single changed or lost output.**
- **Sampling:** every resume.
- **Saturation:** already saturated at both granularities and kept as an invariant alarm, not a rate. Its value is that a single failure is architecture-bearing — it would falsify "suspension is lossless", the premise the whole mark-not-process design rests on.
- **Standing limit:** F19's losslessness covers the *state carrier*; **the FSM was never replayed**, so "same next action" is pinned by AC-258's test, not by this metric.

**SM-081 — Void rework cost** · **DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE**
- **Instrument:** tokens and wall clock between a `gate_answer_void` and the subsequent answer on the re-opened gate; plus human answer latency from SM-074 restricted to re-opened gates.
- **Baseline:** **no baseline — never measured.** ADR-0017 names this as its own negative consequence: a human's work in answering a gate can be discarded and must be redone if the tree moved before it landed.
- **Target:** none set. **Kill: none; this measure cannot fail.** First measurement establishes it.
- **Sampling:** every void.
- **Why tracked:** it is the price of the void rule, and the only number that would justify revisiting ADR-0017's all-or-nothing binding in favour of a path-scoped drift test. Without it, "void, not reconciled" is a design preference with an unpriced cost. **Report machine cost and human latency separately — the human half is the expensive one** (SM-074's median 10.5 min, p90 ~9 h).

**SM-082 — Gate frequency per item** · **DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE**
- **Instrument:** count of `gate_opened` events per item, including re-opens and escalations.
- **Baseline:** **no baseline derivable from the brief** — F18 counts 174 gate→verdict and 191 stage→verdict marks, but **the item denominator is not recorded** [measured for the numerators only].
- **Target:** none set. **Kill: none; this measure cannot fail.**
- **Sampling:** every gate open; per item and per stage, as a distribution.
- **Why tracked:** brief §C.2 is explicit that **duration is free but frequency is marginally costly**, so frequency is the one dimension of gate placement that still has a price attached.
- **Note:** re-opens (AC-252/AC-277) and escalations (AC-259) count here, so a rising frequency may be a drift problem (SM-075) or an expiry problem (SM-079) rather than a placement problem — read it against those two before changing placement.

### 1.8 Review gate — reviewer proposes, machine confirms (SM-085..SM-096)

Twelve measures. Two are saturated (SM-092, SM-093) and one is explicitly diagnostic with no target (SM-091); **nine can fire a kill**, and five of those nine have no baseline at all. Every CLEAN-derived number here comes from **one corpus, one operator, and a ground truth known wrong for at least 57% of items.**

**SM-085 — Block rate per review, by computed class** · *shape discriminator, not a level*
- **Instrument:** `review_block` event, field `computed_class`; `stage_weight`.
- **Baseline:** on the pinned CLEAN corpus, **strict 12/23 = 52.2%** (Wilson 95% **33.0–70.8%**) and **lenient 9/23 = 39.1%** (Wilson 95% **22.2–59.2%**) [measured, F31, n=23 items, one corpus, one operator; **CLEAN's ground truth is wrong for ≥13/23, so these are counts, not accuracy**].
- **Target:** **no target rate.** Tracked as a distribution by class and stage weight; **a step change in the ratio between classes is the signal, not the level.**
- **Sampling:** every review; per class per weight, never a single pooled rate.
- **Kill:** if the `reproduced` share falls below the `advisory` share for three consecutive releases, the executable-claim contract is no longer producing executable claims and §4.6 item 1 is falsified.
- *(This measure carries the 12/23 and 9/23 counts that a cut criterion, AC-297, had been pinning as an acceptance criterion — a change-detector that fires as loudly on a legitimate classifier improvement as on a regression.)*

**SM-086 — Verified false-block rate, against adjudication only** · *discriminator; PROVISIONAL*
- **Instrument:** `review_block` events joined to adjudication records; the denominator **excludes** `pending_adjudication`.
- **Baseline:** **0 of 23** verified false positives under the typed contract (**Wilson 95% 0–14.3%**) [measured, F31]; under the prose reviewer CLEAN FP was 2/23 and 4/23 [measured, F21/F23], but those were measured against a ground truth now known wrong for ≥13/23, so they are **upper bounds only**.
- **PROVISIONAL:** the adjudication this baseline rests on has settled *facts* (6/6) but **not severity** — the E3 severity adjudication is open [brief §E] — so the 0/23 is provisional in the same way AC-308 is.
- **Comparator:** Cloudflare break-glass **288 of 48,095 MRs = 0.6%** [documented] — the only public rate for "the machine blocked and a human had to override".
- **Target:** sustained below the comparator once n allows an interval that excludes it (n ≈500 blocks).
- **Sampling:** every block; Wilson interval always quoted; **a void run is never a miss.**
- **Kill:** a verified false-block rate whose Wilson **lower** bound exceeds 0.6% means the policy table, not the reviewer, is wrong — relax the weight before touching the card.

**SM-087 — Confirm-half determinism** · *discriminator; no baseline*
- **Instrument:** the k=10 replay job (AC-313); `review_block` set digest.
- **Baseline:** **no baseline — first measurement establishes it.** The comparator is the *model* half: a reproduced regression was approved on **3 of 10** draws and class means were 0.70 / 0.37 / 0.10 / 0.00 across 60 draws [measured, F29]; verdicts were flaky on **2 of 4** items over 21 draws while detection was flaky on **0 of 7** [measured, F27, n=2–3 per item].
- **Target:** block-set digest identical on 10/10 replays **and equal to the pinned non-empty expected digest** — an all-empty block set scores 10/10 and must not count as a pass.
- **Sampling:** every release, on the pinned corpus; **$0** (recorded findings, no dispatch).
- **Kill:** any non-identical digest, **or a digest matching the empty set**, falsifies "machine confirms" and returns the gate to a sampling problem.

**SM-088 — Unverifiable share of findings** · *no direct baseline; target chosen not derived*
- **Instrument:** `finding` records with `class = Unverifiable`, split by reason (`both_fail` | `cannot_run` | `timeout`).
- **Baseline:** **no direct baseline for this pipeline.** Nearest measured: of **188 findings, 78% required running something** [measured, F37, 61 runs, 297 records]. *(A comparator claiming "Agentless reproduced 213 of 300 and 94 flipped" is **not in the evidence brief** and was cut.)*
- **Target:** `cannot_run` + `timeout` under 10% — **an unmeasured target, chosen not derived**; `both_fail` is information, not failure.
- **Sampling:** every review; the three reasons reported separately — pooling them hides a harness regression behind genuine both-fail cases.
- **Kill:** `cannot_run` above 50% means findings are not executable claims and AC-281's contract is not being honoured in practice.

**SM-089 — Advisory sink throughput and age** · *discriminator; the sink does not exist yet*
- **Instrument:** sink record count against non-blocking finding count; record age at disposition.
- **Baseline:** **7 adjudicated-true `narrowed-pin` findings with no destination at all** [measured, F31, from 23 CLEAN items]; sink record count today is **0 by construction** [brief §C.8].
- **Target:** routed = created (zero dropped, AC-310); median age at disposition below F18's **gate→verdict** median of **10.5 min** [n=174, one operator].
- **Sampling:** every review for the count invariant; weekly for the age distribution, reported p50/p90, never a mean.
- **Kill:** if the sink's ageing p90 exceeds F18's **stage→verdict** p90 (~9 h, n=191 — **a different series from the 10.5 min median above**) for a month, the sink is a landfill and advisory findings should be dropped at source rather than pretended into a backlog.

**SM-090 — Review cost per item, by tier** · *discriminator*
- **Instrument:** `total_cost_usd` on the review activity's child event; `stage_weight`.
- **Baseline:** execution-grade CLEAN review median **$3.03**, spend **$112.25** for 51 valid runs against a prose comparator's **$64.61 = +74%** [measured, F31]. **The read-only tier is not measured** (§5). *(A "+40% to +101% by condition" range is **not in the brief** and was cut.)*
- **Comparators:** Cloudflare read-only median $0.98 / full tier **$1.68**; Anthropic Code Review **$15–25**; Agentless **$0.70/instance** at 32.00% vs CodeR $3.34 at 28.33% [documented].
- **Target:** execution-grade stays inside the **$1.68–$15** band between the two public comparators.
- **Sampling:** every review; the distribution and the slowest decile — the median hides the 4,561 s tail.
- **Kill:** execution-grade above Anthropic's $15–25 band makes the tier a per-stage budget decision, not a default.

**SM-091 — Model label vs computed class drift** · *diagnostic, never gated*
- **Instrument:** the `review_class_drift` event (AC-285), carrying both labels.
- **Baseline:** **no baseline — first measurement establishes it.** The nearest measured shape is F27: the verdict is flaky (**2/4 items over 21 draws**) while detection is not (**0/7**) [measured, n=2–3]. *(A citation of F31 for "class labels wobble at boundaries while claims do not" is in neither F31's ledger row nor anywhere else in the brief, and was cut.)*
- **Target:** **none. Diagnostic, never gated** — it exists to tell us whether the Rust rule is doing work.
- **Sampling:** every finding.
- **Kill:** two-sided and both informative. Drift ≈0 for three releases means the field is redundant and the model label can be dropped from the schema; sustained high drift is direct evidence for keeping computation in Rust (ADR-0004) and **must never be "fixed" by trusting the label.**

**SM-092 — Negative-control block rate** · *saturated by design*
- **Instrument:** the corpus regression job (AC-312); blocks on control fixtures.
- **Baseline:** S-VAR control **0 of 10** rejections (**Wilson 95% 0–27.8%**) [measured, F29, 10 draws, one fixture family]. **Saturated by design and expected to stay at 0.**
- **Target:** 0 blocks, every release.
- **Sampling:** every release on the pinned corpus.
- **Kill:** one control block per release is tolerable and adjudicated; **two in a release** means the policy table has drifted.
- **Honest limit:** its stated partner SM-093 is *also* saturated, so the pair is a one-way canary in both directions and can discriminate between two configurations in neither. **The only thing that keeps this measure from being satisfied by a gate that never blocks is AC-312's seeded half, which is a criterion, not this measure.**

**SM-093 — Seeded-defect detection recall** · *saturated; canary with no kill*
- **Instrument:** the corpus regression job; findings on seeded fixtures.
- **Baseline:** S-SILENT **23/23**, S-SUBTLE **5/5 sighted and 5/5 blind, twice**; E3 arms **23/23 vs 23/23**, **McNemar p=0.625** [measured, F23/F26/F31].
- **Saturated — explicitly.** This metric had **no discriminating power** in E3, and the programme's method finding is that five metrics saturated and every discriminating result came from cost, stability or mechanism, never success rates [brief §C.12].
- **Target:** stays at ceiling. Kept as a one-way canary only: a drop below ceiling is a real regression signal; a stay at ceiling means nothing. **This is a canary, not a gate — it has no kill and cannot fail a release.**
- **Sampling:** every release. **Never used to compare two configurations.**
- **Kill:** **none.** If it ever falls below ceiling, escalate to a harder fixture tier rather than tuning against it.

**SM-094 — Per-AC discriminating-test coverage** · *no measured baseline; the most expensive measure here*
- **Instrument:** the per-AC mutation harness (AC-317); ratio of ACs whose named test fails under **both** removal and inversion.
- **Baseline:** **no measured baseline.** The proposal reports the mutation columns absent from historical reports (0 of 156) [proposal §4.6 item 8 — **not corroborated by the evidence brief; treat as unverified**]. The one *measured* datum is F25's single probe: inverting AC-015's ordering left the suite **36 of 36 GREEN** [measured, n=1 AC].
- **Target:** every AC in the fix and story lanes has a test that fails under both mutations.
- **Sampling:** every AC at report time; report coverage **and** the median mutant runtime (S0.1's median mutant run was **323 s** [measured, F11]).
- **Kill:** if the harness's added wall clock exceeds the review's own median (**1,071 s**, F31), per-AC mutation becomes a sampled check rather than the norm, and ADR-0019 needs revisiting. **This kill is very likely to fire** given S0.1's 323 s median per mutant.

**SM-095 — Override (quarantine) rate and live-quarantine age** · *no baseline*
- **Instrument:** `quarantine_granted` / `quarantine_expired` events; live quarantine ages from `redkiln doctor`.
- **Baseline:** **no baseline — first measurement establishes it.** Comparator: Cloudflare break-glass **288 of 48,095 MRs = 0.6%** [documented] — the only external rate for a human overriding a machine block.
- **Target:** an override rate of the same order as the comparator; **zero quarantines older than their `until`.**
- **Sampling:** every quarantine; rate with a Wilson interval, age as a distribution.
- **Kill:** an override rate an order of magnitude above 0.6% is evidence the *policy table* is mis-set — the response is to change the weight (a one-line, replayable change, AC-295), **never to weaken the classifier or the re-execution rule.**

**SM-096 — Review wall clock and re-execution timeout pressure** · *discriminator; target chosen not derived*
- **Instrument:** review activity duration; `Unverifiable{timeout}` share.
- **Baseline:** execution-grade CLEAN review median **1,071 s (~18 min)** against prose **682 s = 1.6×**, slowest **4,561 s** [measured, F31]; the deterministic gate it feeds has **no affected-area selection** — every run is the whole 236-file suite, ~250 s to >900 s over 1,890–1,927 tests [measured]. Timeout history: a 900 s default written for the fast condition **cost a CLEAN record**, then 2,400 → 3,600 → one 5,400 s retry [documented].
- **Target:** timeout-caused `Unverifiable` under **5%** — **an unmeasured target, chosen not derived**. *(The original second half, "review p90 inside the item's gate budget", is dropped: no gate budget is defined anywhere in the evidence, so it could neither pass nor fail.)*
- **Sampling:** every review and every re-execution; p50/p90/max — the max is the number that set the timeout history.
- **Kill:** if timeouts drive `Unverifiable` above 5%, the re-execution step needs affected-area selection before the review tier is tuned. The `confirm` bug is the precedent: `pnpm test -- --run <file>` ran the whole suite for **26 of 68** invocations where `npx vitest run <file>` is 1 file / 6 tests / **15.6 s** — a **~60×** difference [measured].


### 1.9 Verification gate — affected-area selection and per-AC mutation evidence (SM-097..SM-107)

Eleven ids. **SM-098 is a descriptive report, not a success measure**, and the area author excludes it from the measure count — it is kept here in full because its instrument is real and its histogram gives SM-097 and SM-099 their context. Four have no baseline (SM-099, SM-100, SM-105, SM-107).

**SM-097 — Selection narrowing ratio** · *discriminator; n=1 command pair*
- **Instrument:** the `verify_completed` event — `selected_test_count`, `total_test_count`, `wall_secs`, `basis` — reported per toolchain.
- **Baseline:** the one documented comparison on the Node corpus — `npx vitest run <file>` = 1 file / 6 tests / **15.6 s** against a whole suite of 236 files and 1,890–1,927 tests at ~250 s to >900 s, a **~60× wall ratio** [measured, brief §D; **n=1 command pair, one repo, one machine**]. **No distribution exists: the field was never recorded.**
- **Target:** for `basis: affected` runs, median `selected/total` ≤ **0.25** and median wall ≤ **0.5×** the same toolchain's whole-suite median.
- **Sampling:** every verify run; a distribution per toolchain, never a mean alone.
- **Kill:** if `basis: affected` runs do not separate from whole-suite runs on wall clock, affected-area selection is not paying for its complexity and the design should ship whole-suite-only with an honest basis.

**SM-098 — Declared whole-suite fallback rate, by reason** · **DESCRIPTIVE REPORT, NOT A SUCCESS MEASURE**
- **Instrument:** `basis` + `basis_reason` on every `verify_completed` event.
- **Baseline:** the only measured figure is the *silent* historical rate — a filter that did not survive package-manager forwarding produced a whole-suite run on **26 of 68 invocations = 38.2%**, **Wilson 95% [27.6%, 50.1%]** [measured, brief §D; **the interval is computed here, not quoted from the brief**]. **The declared rate has no baseline:** the field did not exist.
- **Why it is not a measure:** it has no kill condition that can fire and no target that can be missed independently. A high declared fallback rate is acceptable by design, and reason coverage is already a build failure under AC-352 — so this row would be a metric whose target cannot be missed.
- **Sampling:** every run; report the reason histogram alongside the rate.

**SM-099 — Silent-selection-failure detections** · *no baseline; first-occurrence alarm*
- **Instrument:** `selection_ineffective` events (AC-326) as a share of `basis: affected` attempts.
- **Baseline:** **no baseline — first measurement establishes it.** The historical rate is unmeasurable because the executed argv was never recorded; the 26/68 figure above is the only evidence that the failure mode is real, **and it was found by reading a command string, not by an instrument.**
- **Target:** any non-zero rate is a toolchain-pack bug to fix, not a tolerance.
- **Sampling:** every `affected` attempt; alert on first occurrence per toolchain rather than on a threshold.
- **Kill:** if this fires steadily above ~5% for a toolchain after its probe passes `doctor` (AC-327), the probe is not representative and the doctor rule needs a harder fixture.
- **Known hole:** AC-326 detects only a **fully** inert filter. A wrapper that drops some arguments — selecting 200 of 236 files — is recorded as `affected` and passes every criterion in the area; this measure is detection-after-the-fact, not a gate.

**SM-100 — Per-AC pinned coverage** · *no baseline; target is an aspiration*
- **Instrument:** the AC evidence ledger — `pinned` / (`pinned` + `unpinned` + `mutation-survived` + `not-feasible`) per item at stage exit.
- **Baseline:** **none. No AC-level pinning rate has ever been measured.** The nearest evidence is two points, neither of which is a rate: F25's single AC-015 probe, where inverting the ordering left **36 of 36 GREEN** [measured, n=1 AC, one operator]; and F31, where **four adjudicated defects survived fully green suites of 1,890/1,890 and 1,907/1,907** [measured]. The claim that no historical implementation report carried per-AC evidence columns is *proposal §4.6 point 8 — not in the evidence brief; no report-corpus denominator is asserted.* **Any rate quoted against CLEAN's ground truth would be an upper bound in any case: that ground truth is wrong for ≥13 of 23 = 57%** [brief §E, contested]. *(A claimed "≥12 of 23 = 52% of shipped items carried an AC no test pins, Wilson [32%, 72%]" was **cut as fabricated**: the 22% it was evidently derived from is F24's statement that CLEAN's ground truth is wrong for ≥5/23 — an entirely different quantity.)*
- **Target:** coverage **≥0.90** at stage exit, with the residual routed to AC-357's human gate rather than left silent. **This target is set with no baseline** — an aspiration to be revised on the first real distribution, not an inference from evidence.
- **Sampling:** every item at stage exit; per lane (fix vs story), as a distribution.
- **Kill:** if coverage cannot exceed ~0.5 because mutation is infeasible for most ACs in real repos, per-AC mutation evidence is not the *normal* TDD evidence, and ADR-0019's "norm" claim needs rewriting rather than enforcing.

**SM-101 — Mutation-survived rate** · *discriminator; two small measured points*
- **Instrument:** AC ledger class counts; the applied-mutation record (AC-338).
- **Baseline:** two measured points, both small. F25's AC-015 probe: removing step 7 → suite RED; **inverting the ordering → 36 of 36 GREEN** [measured, n=1 AC, one operator]. F11's S0.1: **9 of 10 test-reachable mutants killed, 1 survived — 90.0%, Wilson 95% [59.6%, 98.2%]** (computed here; the brief states only that the CI on 1/10 is wide), headline 93.3% over 15 mutants [measured].
- **Target:** a **falling** survived rate over time per repo; **no absolute target** — the rate is a property of the suite under test, not of redkiln.
- **Sampling:** every AC with a feasible mutation; a rate with its Wilson interval, per repo.
- **Kill:** if the survived rate is near zero from day one across repos, the mutations being generated are too easy and the evidence is decorative — **check against the AC-355 negative controls before believing it.**

**SM-102 — Verify wall clock and timeout kills** · *discriminator; the timeout-kill rate has no baseline*
- **Instrument:** `wall_secs` and `outcome = Unverifiable(timeout)` on `verify_completed`.
- **Baseline:** whole-suite **~250 s to >900 s** [measured, §D]; S0.1 mutant runs median **323 s** [measured, F11]; targeted single file **15.6 s** [measured, n=1]; the S-TYPED CLEAN review comparator's slow tail reached **4,561 s** [measured, §D]. Timeout history: a 900 s default written for the fast condition cost a CLEAN record, then 2,400 → 3,600 → one 5,400 s retry [documented]. **The timeout-kill rate itself has no baseline.**
- **Target:** timeout kills **<1%** of verify runs; p95 wall inside the declared timeout. **The "≥2× headroom" figure is an aspiration, not a floor** — AC-330's enforceable rule is only "timeout ≥ recorded p95, where a p95 exists", **and no whole-suite p95 has ever been recorded**, so that half cannot fire yet.
- **Sampling:** every run; p50/p95/max per toolchain per basis — never a mean, because the whole-suite tail is what sets the timeout.
- **Kill:** a p95 that grows with repo age faster than the timeout is raised means the whole-suite fallback is not viable as a fallback and selection must become mandatory rather than best-effort.

**SM-103 — Model dispatches inside verify** · *saturates at zero; invariant, not a comparison*
- **Instrument:** count of `dispatch` events attributed to a verify activity.
- **Baseline:** **0 by construction**, and **$0 measured in every deterministic spike** — S0.1, S0.2, F24, F25 and the F31 adjudication all cost $0 [measured, brief §D].
- **Target:** exactly 0, always.
- **Sampling:** every verify run.
- **Saturation:** expected to saturate at zero and tracked anyway. **A saturated *invariant* is not a saturated *comparison*:** the value here is the moment it stops being zero, which is the moment a model entered the selection path that F36 shows cannot carry it (modal `R` identical on 5/20, 3/20, 2/20, 2/20 against an 80% line, 400 dispatches, 0 void).
- **Kill:** a non-zero count is a defect, not a metric movement.

**SM-104 — False-block rate against adjudication** · *discriminator; the interval does not exclude 14%*
- **Instrument:** `block_emitted` and `block_adjudicated` events; **the denominator is adjudicated blocks only** (AC-358).
- **Baseline:** S-TYPED measured **0 of 23 verified false positives** under the typed contract, **Wilson 95% [0%, 14.3%]** [measured, F31; **interval computed here** — the upper bound is 14.3%, not the ~12% an earlier revision carried, and a 0/23 draw licenses no claim below it]. The earlier prose arm measured 2/23 and 4/23 against CLEAN's ground truth, **which is wrong for ≥13 of 23**, so every rate measured against it is an **upper bound**. External comparator: Cloudflare break-glass **288 of 48,095 MRs = 0.6%** [documented, §D].
- **Target:** adjudicated false-block ≤ **1%**, in the neighbourhood of the comparator. **Note the baseline's interval does not exclude 14%, so this target is not yet supported by evidence** — it is the bar to measure against.
- **Sampling:** every block; adjudicate a sample, never the whole population; report with the interval **and** the adjudicated denominator, **and state the unadjudicated count alongside. A void (unadjudicated) is never a miss.**
- **Kill:** an adjudicated false-block rate above ~5% makes the verification gate a tax rather than a filter and returns `mutation-survived` to advisory — **which presupposes the advisory sink of AC-344, which does not exist yet** [brief §C.8].

**SM-105 — Selection-probe coverage and freshness** · *no baseline*
- **Instrument:** `redkiln doctor` output; `{capability, verified_on, probe}` rows in the toolchain capability registry.
- **Baseline:** **no baseline — first measurement establishes it.** Zero toolchains carry such a probe today; the historical selection defect ran undetected for **26 of 68 invocations** precisely because no configuration-time probe existed [measured, brief §D].
- **Target:** 100% of configured toolchains carry a probe verified at the current version pin. The probe is deterministic and therefore costs no model spend by construction; **the $0 spikes are the precedent for deterministic work costing nothing, not a measurement of this rule.**
- **Sampling:** every `doctor` run and every CI run.
- **Kill:** if probe maintenance proves so costly that repos disable it, selection must default to whole-suite **with a declared basis** rather than to an unverified `affected`.

**SM-106 — Human-gate load created by the verification gate** · *discriminator on frequency, never on duration*
- **Instrument:** `gate_opened` events with `origin = verify`; gate→verdict latency; `cache_creation_input_tokens` on resume.
- **Baseline:** gate→verdict median **10.5 min**, 45% over 1 h, **n=174**; stage→verdict median 51.6 min, p90 ~9 h, max 226 h (9.4 days), 48% over 1 h, **n=191** [measured, F18; **one operator**, so this is one person's working rhythm, not a population]. Resume cost is flat in duration under the default prompt: **18,603 tokens** at both 65.2 min and 49.7 h [measured, F28 as corrected — **the flatness is shared-prefix subsidy, not durability**, and `--system-prompt` instead creates 29,619–29,680 at ~66 min]. Drift on gates over 1 h is 3.6× the rate on shorter gates: **9/79 = 11% vs 3/95 = 3%** [measured, F19; intervals overlap].
- **Target:** fewer than **1 verify-origin human gate per item** in the fix lane. **Duration is not a target, because gate duration is free and only frequency is costly** [brief §C.2].
- **Sampling:** every gate; report the latency distribution and the drift-on-reopen rate together, since AC-359 makes drift a re-open rather than a silent overwrite.
- **Kill:** if AC-357's mutation-infeasible route opens a gate on most ACs, per-AC mutation evidence has become a human workflow rather than an automated one, and the toolchain feasibility declaration (AC-337) is being set wrong.

**SM-107 — Selection determinism** · *no baseline; the alternative was already killed*
- **Instrument:** paired recomputation on identical inputs (AC-348) plus replay equivalence (AC-354), reported as a defect count.
- **Baseline:** **no baseline for the deterministic path — first measurement establishes it.** The comparator is the alternative that was tested and killed: an LLM-emitted context vector reproduced its modal required artifact set on only **5/20, 3/20, 2/20 and 2/20** against an 80% line, with alpha drift **−0.168** on byte-identical anchors [measured, F36; 400 dispatches, 0 void, **kill fired 4 of 4**]. That is why selection is code.
- **Target:** 100% identity; **any divergence is a defect with a named cause, not a rate.**
- **Sampling:** paired recomputation on every run in CI; replay equivalence nightly over the recorded corpus.
- **Kill:** a divergence whose cause cannot be named and removed kills the *pure-function* selection design as specified — and since the modelled alternative is already killed by F36, **the response is to re-specify selection (narrower inputs, pinned policy version), never to reintroduce a dispatch.**
- **Standing limit:** F19 established resume losslessness at activity granularity (174/174) but **the FSM was never replayed**, so this measure's first readings are new evidence, not confirmation.

### 1.10 Event log, cost attribution, telemetry and throttling (SM-109..SM-118)

Ten measures. Five have no baseline (SM-109, SM-110, SM-112, SM-115, SM-116). Two are tripwires that re-run guarantees other criteria already enforce (SM-109's compile-time half, SM-116's next-action half).

**SM-109 — Attribution completeness** · *only the reconciliation half can fail*
- **Instrument:** dispatch events, `total_cost_usd`, token counters; `redkiln cost <item>`; monthly reconciliation against the account's own billed spend.
- **Baseline:** **no product baseline — first measurement establishes it.** The programme's own per-unit spend, for scale: trivial haiku dispatch **$0.021–0.027**; E3 S-SILENT **$0.45–0.64** (97–424 s); E3 CLEAN **$1.06–4.89** (317–2,055 s); S-TYPED CLEAN review median **$3.03**; P7 adversary dispatch **$1.68 / 676 s** [measured].
- **Target:** 100% of dispatches carry a cost row; **the gap between summed `total_cost_usd` and the account's billed spend for the same period is ≤2%.**
- **Honest note on what can actually fail:** the in-product half is close to tautological — AC-394 makes a dispatch without a store uncompilable, so "100% carry a cost row" is enforced at compile time and this measure cannot discover a violation the compiler already prevents. **The reconciliation against billed spend is the half that can fail** (sub-agent dispatches, retried children, or a harness path the adapter does not own), so the ≤2% gap is the operative target and the compile-time half is a regression tripwire.
- **Sampling:** every dispatch; reconciled monthly.
- **Kill:** a reconciliation gap that does not close falsifies "attribution is free because every activity is its own child" (§4.7) and forces an explicit accounting layer the design deliberately does not have.

**SM-110 — Cost per completed item, by lane** · *discriminator; no product baseline*
- **Instrument:** sum of `total_cost_usd` over an item's dispatch events, grouped by lane (`fix`, `story`) and stage rigor.
- **Baseline:** **no product baseline — first measurement establishes it.** Nearest measured analogue is P7's graph-run cost: optional **$1.02**, recommended **$2.69**, mandatory **$9.86 (n=1)**, over 61 runs / 297 records / $131.62 total, 0 void [measured, F37]. **Contested figure:** the mandatory cost ratio is **3.66× in F37 and 4.4× in the decisions log, unreconciled** — do not build a target on it, and note the mandatory arm is n=1.
- **Target:** reported as a distribution (p50/p90), never a mean alone; the fix lane's p50 tracked against the execution-grade review's own median of **$3.03** [measured, F31].
- **Sampling:** every completed item.
- **Kill:** a fix-lane p50 above ~3× the execution-grade review cost (**above ~$9.09**) falsifies "the fix lane stays lean: one agent + tests + policy review" (§4.7) and reopens the lane's composition.
- **Saturation note:** none expected. Cost has been the discriminating signal throughout (§C.12).

**SM-111 — Headroom draw per batch** · *saturates at the low end; kept as the only view of the ceiling*
- **Instrument:** `rate_limit_event.unifiedWindows.{five_hour,seven_day}.utilization`, batch delta, with the `resolution_limited` flag.
- **Baseline:** **0.0098 `five_hour` per CLEAN draw** and **0.0017 `seven_day` per draw**, both **batch-derived means over 60 draws** [measured, F29: 0 void, $176.96, 16.2 h]. **F21's earlier 0.077 figure is superseded — 7.8× too high** [measured]. A 40-dispatch / ~1.9M-token batch moved `seven_day` 0.100 → 0.100, i.e. entirely below the instrument [measured, F14 — **that gate was struck, not answered**]. Instrument resolution: **0.01** [measured, F6].
- **Target:** report batch deltas only, always with n and the `resolution_limited` flag; **never a per-run figure.**
- **Sampling:** one reading per dispatch, aggregated and reported only in batches of **≥10 dispatches** — 10 × 0.0098 ≈ 0.098 of the window, about 10 ticks, which is comfortably resolvable; a single CLEAN draw at 0.0098 sits at roughly one tick and is not resolvable on its own. *(Repaired: the original required ≥100 and justified it as "100 × 0.0098 ≈ 1.0 ticks" — an arithmetic error of two orders of magnitude, which would have meant consuming nearly a whole five-hour allowance before a figure could be quoted.)*
- **Saturation, stated:** **this instrument saturates at the low end and is kept anyway.** It cannot resolve a single run and never will at 0.01 resolution; it is tracked because it is the only view of the platform ceiling that exists, and because F1 shows the *binding* family sub-limit has no key at all.
- **Kill:** two or more batches in a month that halt on a margin refusal (AC-376) **before** exhausting their wall-clock budget — headroom would then be the binding constraint and the parallelism design changes. Measured today, the binding constraint on a CLEAN set is **wall clock**: 16.2 h for 60 draws.

**SM-112 — Throttle refusals, overage and rate-limit marks** · *baseline unknown, not zero*
- **Instrument:** counts of `StepError::Overage`, `StepError::RateLimited` and margin refusals, per 1,000 dispatches.
- **Baseline:** **none, and for overage none is obtainable from the programme.** What *is* measured is void rate: P1 400 dispatches / 0 void; P7 61 runs / 297 records / 0 void; S-VAR 60 draws / 0 void; P8 3 runs of a pre-registered 27+3 [measured]. **A void is not a miss and is not counted as one.**
- **Overage was never instrumented — treat its baseline as unknown, not as a measured zero.** *(An earlier draft read "zero overage observed across the programme's large runs"; the brief reports no overage counts of any kind and no overage payload was ever recorded, which is also why AC-378's detection half is PROVISIONAL. Absence of a report is not a measurement.)*
- **Rate quoting:** no Wilson interval — for void the numerator is zero on every arm, and for overage there is no numerator at all.
- **Target:** `Overage` count 0; every margin refusal followed by a successful resumed dispatch after `resets_at`.
- **Sampling:** every dispatch.
- **Kill:** a single mid-run overage means the visible-window proxy is insufficient for the invisible family sub-limit (F1), and throttling must fall back to a conservative fixed dispatch rate rather than a headroom-derived one. **Note this kill may be unable to fire while its detection path (AC-378) is itself blocked.**

**SM-113 — Prefix cache-read share, by `cache_profile`** · *discriminator, per cell, against that cell's own baseline*
- **Instrument:** `cache_read_input_tokens` / (`cache_read_input_tokens` + `cache_creation_input_tokens`) per dispatch, grouped by card profile and by axis (commit churn / worktree churn / elapsed time).
- **Baseline, commit and worktree axes [measured, F16: 50 dispatches, within-arm range ≤0.001]:** no-commit **1.000**; commit-each **0.689**; +`--exclude-dynamic` **0.846**; +`--system-prompt` **1.000**; worktree-each **0.685**. Worktrees [measured, S-WT]: **w1 0.807, w2–w10 all 1.000** — see §3 for the unresolved question of which prompt configuration S-WT ran, and note **F30 was never written**. Time axis [measured, F28 as corrected + S-H1b, **n=1 per point**]: default prompt **18,603** created at 65.2 min *and* at 49.7 h; static prompt **29,619–29,680** created at ~66 min with read share **0.000**; **the 24 h read was not taken.** TTL [measured, F10, **n=1 per gap, date-stamped platform behaviour**]: full hit at 45.1 min; 61.6 min → share 0.690; boundary bracketed **(45.1, 61.6), never located**. Noise floor [measured, F5]: 0.584 / 0.698 / 1.000 / 0.722 on an identical prompt at n=2 — **explicitly not a result. Never quote a share at n<10.**
- **Target, per (profile × axis) cell against that cell's own measured baseline:** `static` across commits **≥0.95** (baseline 1.000); `static` across worktrees **≥0.95** (baseline w2–w10 1.000, subject to the contested cell); `default` across time **≥0.65** (the only measurement of that cell is F10's 0.690 at 61.6 min). **Cells with no measurement carry no target and are reported as unmeasured.** *(A blanket "each card holds share ≥0.85 on the axis its profile optimises for" is **withdrawn**: the only measurement of the default × time cell is 0.690, so 0.85 was falsified before it was set.)*
- **Sampling:** every dispatch; a distribution per profile per axis, never a mean alone.
- **Kill:** a `static`-profile card whose share falls below 0.5 across commits falsifies ADR-0016 for that card and moves it to the `default` profile.

**SM-114 — Cache write tokens and isolation share** · *mechanism alarm; the stagger half is a decision rule*
- **Instrument:** `cache_creation_input_tokens` per dispatch; child `system/init` tool count; fan-out stagger interval.
- **Baseline [measured, F8]:** default `-p` cache write **8,243 → 3,611** under `--strict-mcp-config`; MCP servers accounted for **4,631 tokens = 56%** of the default write; tools **280 → 34**; TTFB ~1,480 of ~3,400 ms. Prefix structure [measured, F13]: **~17,439 shared + ~7,827 per-directory**, 19/19 fresh repos identical — a directory change reprices at ~31%, not 100%. Review-scale prefix: **~36,160 default / ~29,680 static** [measured].
- **Target:** MCP share of the cache write = **0%** on 100% of dispatches, verified from the child's own `system/init`.
- **Sampling:** every dispatch.
- **Stagger sub-measure — no baseline, and a decision rule rather than a target.** The stagger rule is [documented] (platform docs), not locally measured, so there is no threshold to inherit. The decision rule is: first-write-pays share among siblings, staggered (i = 5 s) against simultaneous (i = 0, which AC-388 requires the scheduler to support), **A/B over n ≥50 sibling pairs**. Until that runs the stagger is tracked, not decisive, and **no claim about its benefit may be made.** If the A/B shows no difference at n ≥50, AC-388's stagger requirement is dropped.
- **Kill:** any dispatch whose `system/init` exceeds the card allowlist **invalidates every cost comparison in that run** — an ignored flag is indistinguishable from an honoured one from the parent's side (F8, §C.11).

**SM-115 — Per-item token appetite consumption** · *no baseline*
- **Instrument:** sum of input + output tokens per item against the appetite fixed at triage.
- **Baseline:** **no baseline — first measurement establishes it.** The only comparable per-item token magnitudes measured are **structural, not work**: warm trivial prefix ~25,260; review-scale prefix ~36,160 default / ~29,680 static; gate resume **18,603** default and 29,619–29,680 static; mid-graph resume 8,408 / 10,314 / 14,579 [measured, F13/F28/S-H1b/F38].
- **Target:** report the fraction of items halting on appetite as a distribution across lanes, with p50 and p90 consumption as a fraction of appetite; **the operative threshold is the kill below.** *(A "healthy distribution is right-skewed with a small tail" formulation was cut — a judgement with no threshold and no instrument.)*
- **Sampling:** every item at completion or halt.
- **Kill:** more than ~20% of items halting on appetite says the ceiling was sized from prefix cost rather than from work, and returns the triage gate (AC-396) to a different anchor.

**SM-116 — Tier-2 disposability in practice** · *regression tripwire; only the append-failure counter carries new information*
- **Instrument:** the tier-2 append-failure counter; the `redkiln doctor --rebuild` CI job; the replay test's next-action diff.
- **Baseline:** **no baseline — first measurement establishes it.** Adjacent measured evidence: suspension is lossless **174/174** at activity granularity and **3/3** across a real `taskkill /T /F` at mid-graph step granularity, digests identical [measured, F19 and F38 PARTIAL]. **Limit, stated:** the FSM was never replayed in either spike, and P8's state holds exactly one step — so "same next action" is unproven by the spikes and is proven only by AC-365's replay test.
- **Target:** rebuild reproduces 100% of tier-1 state from a clean clone on every CI run; tier-2 loss changes the next action **0 times**.
- **Sampling:** every CI run; tier-2 append failures counted continuously.
- **Status: regression tripwire, not a discovery instrument.** The next-action half re-runs AC-365's own test, so it can only catch a regression, not establish a new fact. **The tier-2 append-failure counter is the part that carries new information** — nothing in the programme measured how often a tier-2 append fails in practice.
- **Kill:** a rebuild producing a different next action means a fact currently tiered 2 is decision-bearing and must move to tier 1 and to a git trailer.

**SM-117 — Wall clock, the actual binding constraint** · *discriminator; its kill has already fired historically*
- **Instrument:** per-dispatch duration and per-run wall clock on the event; timeout-hit counter.
- **Baseline [measured]:** S-VAR 60 draws in **16.2 h**; E3 median wall S-SILENT 242 s (arm a) vs **128 s** (arm b), CLEAN 769 vs 682 s; S-TYPED CLEAN median **1,071 s (~18 min)**, slowest **4,561 s**; P7 adversary dispatch 676 s; S0.1 mutant runs median 323 s; the full gate has no affected-area selection — every run is the whole 236-file suite, ~250 s to >900 s over 1,890–1,927 tests. Human gate median 10.5 min, p90 ~9 h, max 9.4 days [measured, F18, **one operator**].
- **Target:** p50/p90 tracked per activity kind; **every activity kind's configured timeout sized above its measured p90.**
- **Sampling:** every dispatch.
- **Kill:** any activity kind whose measured p90 exceeds its configured timeout re-opens the timeout sizing. **This is a recorded defect, not a hypothetical:** a 900 s default written for the fast condition cost a CLEAN record before being raised to 2,400 → 3,600 with one 5,400 s retry. *(Repaired: the original stated "Kill: none — a tracked constraint, not a hypothesis". A measure with no condition that can fire is not a measure.)*

**SM-118 — Saturated-metric register** · *the register's own method control*
- **Instrument:** a `doctor` report rule listing every telemetry metric currently at ceiling or floor, with its n.
- **Baseline [measured]:** **five of this programme's metrics saturated** — E3 detection 23/23 vs 23/23 (McNemar p=0.625, no discriminating power, F23); E3-H 20/20 across two harder tiers (F26); S-TYPED schema conformance 52/52 = 100% (F31); H2 suspension 174/174 (F19); H4 Jaccard FULL 1.000 = CONTROL 1.000, effect +0.000 (F28). **Every discriminating result the programme produced came from cost, stability or mechanism** [measured, §C.12].
- **Target:** a metric that saturates is **demoted from decision input to regression tripwire within one review cycle, and the demotion is recorded** — not silently retained as if it still informed a choice.
- **Sampling:** monthly, over the whole telemetry catalogue.
- **Why saturated metrics are still tracked rather than dropped:** a saturated detection metric remains the negative control — S-SUBTLE-shaped fixtures stay in the corpus precisely because a stable approval is also consistent with a lenient judge [documented, §4.6 point 7].
- **Kill:** a metric still cited as a decision input in a shipped pack, ADR or gate **one full review cycle after** the register listed it at ceiling — the register is then decorative and the demotion step is not being executed. *(Repaired: the original stated "Kill: none — this is a method control on the telemetry itself". A method control that cannot fail controls nothing.)*


### 1.11 CLI surface and `doctor` (SM-121..SM-130)

Ten measures. Six have no baseline (SM-122, SM-124, SM-126, SM-128, SM-129, SM-130). SM-121 saturates at zero by design.

**SM-121 — `doctor` model cost** · *saturates at 0 by design; tripwire*
- **Instrument:** dispatch counter on the `HarnessPort` during `redkiln doctor`, and `total_cost_usd` summed over the run.
- **Baseline:** **0 dispatches, $0** by construction. Every comparable configuration-time analysis in the programme measured $0: F24 (the six-rejection adjudication), F25 (the AC-015 probe), F32/F33 (schema authoring and lint), F34 (90 activities), F35 (the lattice, 27 s over 102 activities) [measured, brief §D].
- **Target:** exactly 0 dispatches **over the AC-415 fixture corpus, so a `doctor` that does nothing cannot satisfy it.**
- **Sampling:** every `doctor` invocation, in CI and in the shipped binary.
- **Saturation, disclosed:** saturates at 0 by design and has no discriminating power. Kept as a tripwire — F20 is the measured precedent for a check that silently stopped checking, and a rule that quietly reaches for a model is that same failure shape.
- **Kill:** any non-zero dispatch count from `doctor` falsifies "every doctor rule is configuration-time at zero model cost" and returns that rule to the runtime path, where it costs money per item.

**SM-122 — `doctor` wall clock at catalog scale** · *no baseline; numeric target and kill*
- **Instrument:** `doctor` duration, reported per rule and in total.
- **Baseline:** **no baseline — first measurement establishes it.** Nearest measured comparators: the P6 lattice engine evaluated the v0 catalog (**102 activities**) in **27 s** [measured, F35, n=1 run]; the affected-area gate's mutant runs had a median wall of **323 s** [measured, F11, 15 mutants].
- **Target:** `doctor` over the shipped v0 catalog (102 activities) completes in **≤60 s**, so that running it is never a reason to skip it.
- **Sampling:** every CI run; a distribution across consumer repos, never a mean alone.
- **Kill:** a median above **120 s** on the v0 catalog. A `doctor` that costs minutes will be skipped, and a skipped configuration-time rule is a runtime failure deferred — at which point the rule table has to shrink or shard. *(Repaired: the original kill named a "~95-activity catalog", a figure in no source, and left a dead band between "tens of seconds" and "minutes" in which neither target nor kill fired.)*

**SM-123 — `doctor` rule yield: real catches vs false alarms, per rule** · *discriminator, per rule*
- **Instrument:** `doctor_rule_fired` event with `rule_id` and `severity`, joined to whether the resulting configuration change was kept or reverted.
- **Baseline:** the lint's first run caught **13 real errors against 34 false alarms** [measured, F32, 10 activities]; once hardened and read first, first-run errors fell to **0.11/file** for a third author against **1.3** (F33) and **4.7** (F32) [measured, F34, 90 files]. **Rests on three authoring passes in one session lineage — not three independent humans** — and F33 records that P9's "amendments will fall" prediction was **falsified at n=3**.
- **Target:** per rule, real catches exceed false alarms across a release; a rule with zero catches across **three consecutive releases** is deleted rather than kept (ADR-0027's EPF-rot argument applied to the rule table itself).
- **Sampling:** every `doctor` failure; per rule id, never table-wide — F32's split is exactly the information an aggregate destroys.
- **Kill:** a rule table whose false alarms dominate teaches operators to route around it, which is the behaviour the spike's 34 false alarms produced in miniature. *(Cut: "4 of them design defects" and "34 false alarms from 4 prototype bugs" are not in the brief.)*

**SM-124 — `doctor --rebuild` next-action fidelity** · *no baseline; report ids, not a rate*
- **Instrument:** the delete-and-rebuild CI job's divergence report — count and ids of corpus items whose next action differs after rebuild.
- **Baseline:** **no baseline — first measurement establishes it.** What is measured is adjacent, not this: F12 covered 360 trailer commits / 74 slices of trailer data (**a slice-state result, not a validation of `reconcile`**) and F19 measured suspension lossless 174/174. The brief states the limit plainly — **the FSM was never replayed, so "same next action" is unproven** [§C.4].
- **Target:** **0** divergent items on the pinned corpus, every CI run.
- **Sampling:** every CI run of the corpus job; **report the item ids, not a rate** — at 0 expected, a rate carries no information and a single id carries all of it.
- **Kill:** any divergence falsifies the tier-1/tier-2 split — a decision-bearing fact is living only in the database, and either that fact moves into a git trailer or the "store is a cache" claim is withdrawn.

**SM-125 — Void answer rate at the gate** · *discriminator against the measured drift band*
- **Instrument:** `gate_answer_void` events over (`gate_answered` + `gate_answer_void`).
- **Baseline:** drift measured at **12/174 = 6.9%** overall [**Wilson 95% 4.0–11.7%**], **3/95 = 3.2%** for gaps ≤1 h [**1.1–8.9%**], **9/79 = 11.4%** for gaps >1 h [**6.1–20.2%**] [measured, F19, **one operator, one machine**]. **The 3.6× ratio is a point estimate and those two intervals overlap — the ratio is not separated at this n.** 45% of gate→verdict intervals (n=174) and 48% of stage→verdict intervals (n=191) exceed 1 h [measured, F18].
- **External comparator:** Cloudflare break-glass **288 of 48,095 MRs = 0.6%** [documented] — **a different quantity** (human override, not answer invalidation), cited only for the order of magnitude at which an escape hatch stays exceptional.
- **Target:** within each duration band (≤1 h, >1 h), the void rate does not exceed the **upper Wilson bound of the measured drift rate for that band — 8.9% and 20.2%**; every void carries the drift that caused it.
- **Sampling:** every answer; a rate with its Wilson interval, split by gate duration ≤1 h / >1 h.
- **Kill:** a void rate above the drift band means the CLI is minting tokens against a tree the human never saw, and ADR-0017's tree-hash binding is not doing the work it was chosen for. *(Repaired: "tracks the drift rate" was a judgement with no threshold.)*

**SM-126 — `explain` sufficiency at a gate** · *no baseline; pre-registered kill*
- **Instrument:** the required field on the gate answer recording whether `explain` answered "why did this run" without the reviewer opening a TOML file (the human-gate half of AC-435).
- **Baseline:** **no baseline — first measurement establishes it.** Pre-registered line: **the first 30 gate answers set the baseline, reported as raw counts.**
- **Target:** stated by the kill below.
- **Sampling:** every human gate answer; raw counts below n=30, a rate with a Wilson interval above it.
- **Kill (pre-registered so it can fire):** **more than 15 of the first 30** answers report that the reviewer opened a TOML anyway. At that point the two-home split (ADR-0024) is paying its cost without buying its benefit, and PC2 is back open; the residual becomes a named spike, not a prose target.
- **Disclosure:** the >15-of-30 line is **this register's pre-registration, not anyone's measurement.** It makes the measure firable, which it was not, but the number is arbitrary and needs sign-off from whoever owns PC2 before it becomes a decision line. *(A citation of "spike P3" was cut: no P3 appears anywhere in the brief.)*

**SM-127 — Platform and schema errors that returned success** · *mechanism alarm; two counts, read together*
- **Instrument:** count of dispatches whose process exit code was 0 and whose recorded terminal `StepError` is `Platform` or `Schema`.
- **Baseline:** **one measured mechanism, n=1 fixture** — F20 (`structured_output` absent, exit 0, `subtype: success`) [measured]. **No production rate exists.** The `is_error` / `api_error_status` half of this measure has **no evidence in the brief** and may have no fixture at all. *(An F21 citation for those frames was cut: F21 is the E3 arm (b) result and carries no such observation.)*
- **Target:** dispatches *classified as success* while carrying a schema void = **0**. The count *detected and reclassified* is expected to be non-zero, and is the evidence that the check is alive.
- **Sampling:** every dispatch; **report the two counts separately — a detection count falling to zero is as suspicious as a misclassification count rising.**
- **Kill:** one dispatch recorded as success with `structured_output` absent under a passed schema means "absence of evidence is a block" is not being enforced at the boundary.

**SM-128 — Structured-field write-path violations** · *no baseline that survives checking*
- **Instrument:** `structured_drift` events (a human edit inside a rendered structured section), plus the count of structured writes not originating from `rk-cli`.
- **Baseline:** **no baseline, and no measured precedent that survives checking.** *(S-MIG percentages formerly quoted here — free-form gate results in reports and reviews, specs without GIVEN/WHEN/THEN — are **not in the evidence brief**, which records S-MIG only as a $0 analysis; they were cut from this row. The equivalent figures are carried in SM-044 and SM-045, sourced directly to `spikes/results/s-mig-report.md`.)* The old redkiln's `template_sig`/`rendered_sig` drift is the named failure to avoid [outside the brief].
- **Target:** structured writes from outside `rk-cli` = **0** (enforced by AC-405 and AC-411); `structured_drift` events surfaced 100%, applied 0%.
- **Sampling:** every reconcile; drift events per item per week, as a distribution.
- **Kill:** a single **applied** structured edit falsifies layout C (ADR-0006) — structure has rotted back in through prose, the exact path ADR-0005 was chosen to close.

**SM-129 — CLI-version pin drift** · *no baseline*
- **Instrument:** `TESTED_CLI_VERSION` compared with the observed CLI version, recorded on every event.
- **Baseline:** **no baseline — first measurement establishes it.** The hazard is measured: the prefix is **~17,439 shared + ~7,827 per-directory** tokens [F13], and a static card prompt creates **29,619** (S-H1b, 66.1 min) to **29,680** (F28, 66.8 min) on a cold write [measured]. Brief §C.1 records the subsidy as **account-wide**; **the stronger org-scoping and cache-invalidation-on-upgrade claims are outside the brief and were cut.**
- **Target:** dispatches issued under an unpinned or mismatched CLI version = 0; each mismatch is a configuration error (AC-423), never a warning.
- **Sampling:** every dispatch; mismatches reported as events carrying observed and pinned versions.
- **Kill:** silent version drift makes this programme's cache measurements unreproducible — F13, F16, F28 and S-WT are all conditioned on a fixed CLI, and F10 already date-stamps TTL behaviour as platform-dependent at n=1 per gap.

**SM-130 — Re-dispatch of a memoised committed step** · *no baseline; the adjacent result is circular*
- **Instrument:** dispatch count keyed by memo key, joined to `step_committed`.
- **Baseline:** **no baseline.** The adjacent measurement is F38: mid-graph resume across a real `taskkill /T /F` was lossless with **0 changed, 0 lost, digests identical** at n=3, costing 0.45× / 0.55× / 0.78× of the ~18.6k gate resume [measured, $3.01 of a $40 cap]. **Limits, stated:** (1) P8 is **PARTIAL** — 3 of a pre-registered 27+3; its 65-min and 24-h arms and all three controls are unrun, and its state holds exactly one step. (2) The brief records P8 §5.4(3), the second-arrival question, as **circular by construction** — arrivals were forced and the mark was minted by the driver that implements the semantics — **so F38 cannot be read as "0 re-dispatches measured".** *(Repaired: the original claimed a measured baseline of "0 re-dispatches at n=3", which is exactly that circular result.)*
- **Target:** 0 re-dispatches of a committed step, at any gap duration.
- **Sampling:** every `run`; counts with the gap duration attached, since the durations that matter (65 min, 24 h) have never been run.
- **Kill:** a re-dispatch after a long gap falsifies idempotence (AC-401) and means `run` is not safely re-invocable — which puts a `--resume` flag back on the table, and ADR-0011's Phase 1 exit criterion ("deterministic resume across kill") with it.

### 1.12 `redkiln-migrate` and the clean break (SM-133..SM-141)

Nine measures — deliberately fewer than criteria, because migration is a one-shot gated transformation whose requirements are mostly boolean invariants checked once per run. **Four are explicitly saturating invariants** (SM-134, SM-136, SM-140, SM-141) and **two have no live target** (SM-138's wall-clock half, SM-139's undeclared N). **The honest count of live measures here is seven, of which two are placeholders.** Nothing in this area has been executed: `redkiln-migrate` does not exist, and every share figure comes from a read-only census.

**SM-133 — Migration lift share per (kind, slot)** · *discriminator; upper bounds from one corpus*
- **Instrument:** the differential report artifact from `redkiln-migrate --report` — `lifted` / `with_loss` / `failed` per (kind, slot); census records in `spikes/results/s-mig.jsonl`.
- **Baseline [measured, S-MIG, corpus pin `47faec9f`, 1,312 files / 6,877 records, **n=1 corpus, zero model cost, instrument tuned to this corpus over four iterations**]:** spec sections **98%**; spec frontmatter 153/153 = 100%; **acceptance criteria 88%** (134 lifted / 19 with loss / 0 failed of 153); PR boundary 99%; merge gate 99%; anchors 100%; impl-report TDD evidence 91% (142/7/7 of 156); **impl-report gates 75%** (117/31/8 of 156 — RECLASSIFY); review rubric 100%; review coverage map 91%; **review gate table 17%** (4/7/12 of 23 — RECLASSIFY); ledger fenced YAML 98%; item-card frontmatter 209/209 = 100%; kb atoms 98%; **companion frontmatter 42%** (19/6/20 of 45 — RECLASSIFY). **Wilson intervals not computed on any of these shares;** the small-n slots (review gate table n=23, companions n=45) carry intervals wide enough that their point estimates must not be compared against each other.
- **Saturated slots, labelled:** spec frontmatter (153/153), anchors (152/152), review rubric (38/38) and item-card frontmatter (209/209) are already at 100% and carry no trend information — they can only fall, and are tracked as regression tripwires. **Four of the fifteen slots are in that state, so the *median* slot share is not a meaningful summary.**
- **Target:** at the Phase 2 differential, no slot below 80% without an Accepted ADR attached (AC-462), and no slot's share lower than its baseline here.
- **Cross-instrument caveat on that target:** the baseline was produced by `census-s-mig.py`, a read-only census; the target is evaluated against `redkiln-migrate`, a different program with its own lifters. **A slot falling below baseline does not distinguish "the migration lifts less" from "the two instruments disagree."** The comparison is only sound once the migration tool reproduces the census figures on the same pin — an unmet precondition.
- **Sampling:** every migration run and every lifter change; **per slot, never as a single overall percentage — the overall 98% is exactly the number that hides the 17%.**
- **Kill:** if a lifter change raises the overall share while any slot falls, the census is measuring the instrument rather than the corpus, and the report is no longer evidence for a type decision.

**SM-134 — Section conservation (unaccounted records)** · *conservation invariant; saturates at 0*
- **Instrument:** the differential report — `input_sections` against `lifted + with_loss + failed + opaque`.
- **Baseline:** **no baseline — first measurement establishes it.** The census counted **6,877 (artifact, section) records** [measured, S-MIG] but did not enforce a conservation identity; AC-451 introduces it.
- **Target:** **0** unaccounted records on every run.
- **Sampling:** every run, as a hard gate.
- **Saturation:** saturates at 0 by design and carries no trend information. It is tracked because it is the only measure whose *nonzero* value falsifies the "one reviewed commit, everything accounted" claim — a conservation invariant, not a performance signal.
- **Kill:** any nonzero value blocks the migration commit outright.

**SM-135 — `unknown` density in migrated artifacts** · *discriminator on the live-authored half only*
- **Instrument:** counts of `mutation: unknown`, gate `unknown` and empty companion sidecars in migrated output, and the same counts in **live-authored** artifacts thereafter.
- **Baseline [measured, S-MIG]:** TDD `mutation: unknown` on **142 of 156** reports = **91.0%** (**95% Wilson 85.5–94.6%**); per-AC mutation columns exist in **0 of 156** historical reports; gate results free-form in **25%** of 156 implementation reports and **83%** of 23 reviews (**Wilson not computed; n=23 is small**). *(A "related corpus figure" of "≥12 of 23 shipped fixes carried an AC no test pins (52%, F24/F31)" was **cut as fabricated** — F31's 12/23 is a count of items the strict policy *blocked*, not a per-AC test-coverage gap, and no measured per-AC test-gap rate exists at any n.)*
- **Target:** the migrated `unknown` density is a **fixed historical figure and never improves**; the tracked quantity is the `unknown` density in artifacts authored *under* Redkiln-RS, which must trend to 0 for `mutation` (ADR-0019 makes the discriminating test the norm).
- **Sampling:** every artifact write, **partitioned migrated vs live-authored — never pooled**, or the historical mass hides the live trend.
- **Comparator:** Google's review-start median of one live mutant per changelist (ICSE 2021) [documented, proposal §4.6 item 8]. **Contested provenance: the review-gate pass cut this same comparator on the ground that it is not in the evidence brief. Treat it as proposal-sourced and unverified against a primary — see §4.**
- **Kill:** live-authored reports still at **>50% `mutation: unknown`** after Phase 1's first 20 fix-lane runs means ADR-0019's norm did not take and the per-AC requirement is decoration rather than a gate.

**SM-136 — Migration determinism and idempotence** · *saturating invariant*
- **Instrument:** tree digest of the migrated output across repeated runs; `git diff --exit-code` on a re-run.
- **Baseline:** **no baseline — first measurement establishes it.** The S-MIG census was a read-only count, not a write, so **no output tree has ever been produced** [measured absence].
- **Target:** identical tree digest across runs at the same (corpus SHA, tool version); empty diff on re-run.
- **Sampling:** every CI run of the migration suite, **including one run under shuffled directory order and a different `TZ`.**
- **Saturation:** saturates at "identical" and is tracked only for its failures; the design claim it protects — the new store owns its serialiser and normalises everything once — is falsified by a single divergence.
- **Kill:** any nondeterminism returns the byte-parity risk from "dissolved" to "mitigated", which the proposal explicitly refuses.

**SM-137 — Manual interventions per migration** · *discriminator on one pin*
- **Instrument:** enumerated `manual_fix` records in the differential report (file, line, reason).
- **Baseline [measured, S-MIG]:** **2 of 1,312 files = 0.15%** — the two `_ledger.md` files with unescaped quotes inside quoted evidence strings. **Wilson interval not computed:** a 2-of-1,312 rate has an interval roughly an order of magnitude wide and must not be quoted as a point estimate.
- **Target:** every intervention enumerated with file, line and reason before the commit lands; on the pinned corpus, **≤2** — the two recorded `_ledger.md` files. Any third intervention at this pin is a new finding and goes to the human gate (AC-476) with the report.
- **Sampling:** every run.
- **Kill:** an intervention applied **without** an enumerated record (file, line, reason), which AC-461 makes a hard failure — or **more than 2 interventions on pin `47faec9f`** without a recorded explanation. *(Kill condition replaced: the draft's "interventions growing with corpus size" cannot fire — there is exactly one corpus at one pin, no second corpus is planned, and no size-scaling series exists or is scheduled.)*

**SM-138 — Migration cost** · *dispatch half live; wall-clock half is a placeholder*
- **Instrument:** harness dispatch count and wall clock of a full migrate run; the event log.
- **Baseline:** **zero model cost** over 1,312 files / 6,877 sections [measured, S-MIG]; **wall clock was not recorded — no baseline for run time.**
- **Target:** **0 dispatches, permanently** (AC-466); wall clock **reported, not gated** — there is no measured run time and therefore no CI budget to hold it to. **The dispatch half of this target can be missed; the wall-clock half cannot, until a first run sets a threshold.**
- **Sampling:** every run.
- **Comparator:** every decision this programme made cheaply came from a zero-cost instrument — F24's adjudication, F25's probe, P6, P9, P9b and S-MIG were all $0 [measured, §D], against $176.96 for S-VAR and $112.25 for S-TYPED. The external per-unit comparators (Agentless $0.70/instance, Cloudflare read-only median $0.98) price the model path this measure exists to keep out of the migration.
- **Kill:** a single dispatch inside `redkiln-migrate` makes the migration nondeterministic and unreplayable, contradicting SM-136 and AC-449.

**SM-139 — Dead-artifact detection latency** · *no live target: N is undeclared*
- **Instrument:** `artifact.read` events per activity input; the `doctor` zero-reads rule (AC-472).
- **Baseline [measured, S-MIG]:** **110 of 110 `plan.md` files were the unfilled scaffold** — one dead stage artifact, found by a census *after* the artifact had shipped for the whole life of the old product. **Detection latency therefore = "never, until a census."** No instrumented baseline exists, because nothing emitted a read event.
- **Target:** **not yet set — N is undeclared.** A stage artifact with zero downstream reads must be flagged by `doctor` within N runs, but no N has been chosen, **so as written this target cannot be missed.** It becomes a live measure when N is declared in the `doctor` rule table and AC-472's threshold fixtures (N−1 must not fire, N+1 must) exist.
- **Sampling:** continuous, from the event log.
- **Comparator:** Agent OS v3 retired its orchestration phases by intuition; Scott Logic's Spec Kit accounting is 2,577 lines of markdown for 689 LOC [documented, design recommendation; that source also states **no surveyed system instruments downstream reads**].
- **Kill:** if the rule never fires across Phase 1 and 2 while a census finds a dead artifact, the instrument is placed wrong — reads are being emitted where artifacts are *opened*, not where they are *consumed*. **Note the circularity: this kill depends on someone running the census the rule was meant to replace.**

**SM-140 — Legacy surface count** · *constant by construction; documented baseline*
- **Instrument:** the file-provenance manifest test (AC-443/AC-445) and the legacy-format lint (AC-442).
- **Baseline [documented, proposal §5/§6 — not a measured quantity]:** exactly **1** file imported from the old redkiln (`reconcile.rs`, whose corpus is 360 commits / 74 slices, F12); **101** old CLI-behaviour test files that must cross as black-box specifications and **0** as ports; legacy-format handling sites outside `redkiln-migrate`: **no baseline yet** — the first workspace build establishes it, and the target is 0.
- **Target:** 1 import, 0 ported tests, 0 legacy handlers outside one crate — invariant for the life of the product.
- **Sampling:** every CI run.
- **Saturation:** a constant by construction; tracked because its only informative value is the moment it moves, and a movement is a clean-break failure requiring an amendment to ADR-0002 rather than a code-review comment.
- **Kill:** a second import or a legacy handler outside the migrate crate falsifies ADR-0002's "shares no code, file format or runtime".
- **Instrument limit, disclosed:** the manifest is a **content hash**, so it detects only *verbatim* copies. The one sanctioned import, `reconcile.rs`, crosses **rewritten** — the instrument would not have caught the very file it exists to license. **A rewritten second import passes CI silently, and no instrument in this area detects a semantic port.**

**SM-141 — Self-hosting: recorded end-to-end fix-lane runs** · *a boolean switch, never a rate*
- **Instrument:** event log run ids with a complete lane (intake → landed commit); `redkiln status`.
- **Baseline:** **0** [documented — Phase 1 is unstarted; brief §E records the working tree unclean at HEAD `ab58733` with nothing from the AFK session committed].
- **Target:** **≥1 complete recorded run is the switch** (ADR-0002); thereafter the tracked quantities are per-run cost, resume behaviour and stage mechanism.
- **Sampling:** every run.
- **Saturation warning:** **do not track this as a success rate.** Five of this programme's metrics saturated and success rates were never the discriminating signal — cost, stability and mechanism were [F28 method finding; F23's 23/23 vs 23/23 at McNemar p=0.625 is the worked example]. A "self-hosting: yes" boolean plus per-run cost and resume distributions carries everything a pass rate would, and none of its false comfort.
- **Kill:** if self-hosting is declared **without** a complete lane in the event log, the milestone has been asserted rather than measured, and the switch — which is the whole migration plan — has no evidence behind it.

### 1.13 Ports, fakes, plugin generation and distribution (SM-145..SM-154)

Eight measures. **SM-150 and SM-152 were cut in refutation and their ids are retired** — SM-150 measured a payload-override mechanism the design does not have, and SM-152 was a CI gate re-labelled as a measure ("Kill: none", target "a permanent 0"). **Seven of the eight have no baseline**, and the area has essentially no discriminating measure before Phase 2: roughly half of it is unexercisable during Phase 1 (`rk-plugin-gen` is Phase 2; the GitHub and mdBook adapters are Phase 3; the Codex adapter is Phase 4).

**SM-145 — Derived-artifact re-sync tax** · *no baseline; external comparator only*
- **Instrument:** commits whose changed paths lie wholly inside generated trees, counted per 100 commits by a CI script.
- **Baseline:** **no baseline** — Redkiln-RS has generated nothing yet, and `rk-plugin-gen` is a Phase 2 deliverable; the first 100 commits after it lands establish the baseline.
- **External comparator:** shipmates — roughly **one commit in fourteen (~7%)** exists only to re-sync payload digests, generated pages or hard-coded counts [documented; **their issue history, one project, single maintainer, unreplicated, and a secondary retelling**].
- **Target:** below the comparator, with AC-503's regeneration gate making a re-sync commit unnecessary rather than merely rare.
- **Sampling:** every merge to main; a rolling 100-commit rate with a Wilson interval.
- **Kill:** a re-sync rate at or above 7% means generation is not actually reproducible from source and the generator should emit fewer artifacts.

**SM-146 — Upgrade-path escapes** · *no baseline*
- **Instrument:** defects filed against an installed release whose reproduction requires upgrading from the previous published version, tagged at triage.
- **Baseline:** **no baseline** — nothing has been published. Comparator: shipmates shipped **five blind releases** before any upgrade e2e existed, and **every** escape after it was still found by the owner upgrading rather than by the job [documented, one project].
- **Target:** zero upgrade escapes per release once AC-519's job gates the pipeline.
- **Sampling:** every release, indefinitely. **A release with zero escapes is not evidence until several have shipped — quote the count, and state that the interval is not computed below n=5 releases.**
- **Kill:** escapes continuing after the e2e job exists falsifies the fixture, not the users: the job exercises a synthetic tree and the real install surface is untested.

**SM-147 — Capability-claim age at the moment of use** · *no baseline*
- **Instrument:** a `capability_used` event carrying `capability`, `verified_on` and the dispatch timestamp.
- **Baseline:** **no baseline — first measurement establishes it.** The only prior is [documented]: shelf life "weeks", with two named renames inside one research cycle invalidating a table by its own follow-up comment.
- **Target:** p95 claim age below the registry's declared maximum; every capability used at least once per maximum-age window has a probe run.
- **Sampling:** every capability lookup; **a distribution by capability, never a mean** — a single stale high-traffic capability is the failure, and a mean over many fresh ones hides it.
- **Kill:** an `Unsupported` or a probe contradiction (AC-491) at a claim age well inside the window falsifies the shelf-life model and means the capability needs a probe on every use, not a dated claim.

**SM-148 — `Unsupported` incidence and route-around share** · *knowingly saturated at zero until Phase 4*
- **Instrument:** `StepError::Unsupported` events per 100 activities, split by "pack routed around" vs "run terminated".
- **Baseline:** **no baseline; expected 0 through Phase 1**, which ships exactly one adapter behind each port it uses — SQLite, Claude and Node — and ships **no tracker and no docs-site adapter at all** (those are Phase 3). **This metric is knowingly saturated at zero until a second adapter of any port exists**, which is not scheduled before Phase 4. *(Corrected: an earlier draft described Phase 1 as shipping "one harness, one tracker, one docs site and one toolchain adapter". The saturation is wider than that draft claimed, not narrower.)*
- **Why still tracked:** the first non-zero value is the signal — it is the moment the registry's data-not-identity design (AC-487) is either load-bearing or already bypassed, and it is the acceptance evidence for ADR-0007's second harness.
- **Target:** when non-zero, the routed-around share is the majority; **a terminated run on `Unsupported` means AC-495's startup validation missed an unroutable pack.**
- **Sampling:** every occurrence; **counts, not rates, while n is small.**
- **Kill:** `Unsupported` appearing with a *terminated* outcome means pack validation is not actually reachability-checking the capability set.

**SM-149 — Install repair incidence** · **CONDITIONAL: the distribution mechanism is undecided**
- **Instrument:** `doctor` outcomes by ownership state (own / adopt / refuse) per install and per upgrade.
- **Baseline:** **no baseline.** Comparator: shipmates' namespace retrofit cost **one release, three regressions in the next, and six open follow-ups** — "free at t=0, ~9 issues at t=30d" [documented, one project]; the adopt state exists because failing closed on "ours but unclaimed" strands users on stale content.
- **Target:** `refuse` outcomes are rare and always name a genuine third-party file; `adopt` trends to zero across successive releases, because a persistent adopt rate means the previous release's receipts were wrong.
- **Sampling:** every install and upgrade; **the three states reported separately** — a single "repairs" total conflates a healthy adopt with a broken refuse.
- **Kill:** an adopt rate that does not fall after a release means receipts are not being written for everything the installer writes.
- **Standing caveat:** the proposal's distribution section decides a **plain git clone plus a JS shim fetching the binary from GitHub Releases** — no installer, no receipts, no ownership states. This measure and its criteria are **unbuildable until that section is reopened and a distribution mechanism is actually decided.** Do not read it as a Phase 1 requirement.

**SM-151 — Fake/real conformance divergence** · *known to saturate; coverage growth is the real metric*
- **Instrument:** conformance-suite results per port from the AC-499 CI matrix.
- **Baseline:** **no baseline;** the suites do not exist yet. The first green matrix establishes it.
- **Target:** zero assertions that pass on the fake and fail on the real adapter, per release.
- **Sampling:** every CI run; **report per port, and report the assertion count alongside the divergence count — a suite with three assertions has a meaningless zero.**
- **Known to saturate.** A green matrix reads zero by construction, so a saturated zero here is informative only next to a growing assertion count: **the metric actually tracked is coverage growth, and the divergence count is its alarm.**
- **Kill:** a divergence found in production rather than in the matrix means the fake models the adapter's happy path only, and the "fake per port" rule (AC-481) is satisfied in letter and not in substance.

**SM-153 — Probe cost and wall time** · *no baseline for the suite*
- **Instrument:** a `doctor --probe` full-registry run — `total_cost_usd` summed across probes, and wall seconds.
- **Baseline:** **no baseline for the suite.** Per-probe prior: a trivial haiku dispatch costs **$0.021–0.027** [measured, F3, n small], and non-model probes cost **$0** (F24, F25, S-MIG and the F31 adjudication all ran at $0 [measured, §D]).
- **Target:** a full registry re-probe under a dollar and under a few minutes, so refreshing dated claims is never deferred on cost.
- **Sampling:** every `doctor --probe` invocation; cost and wall as a distribution.
- **Kill:** if a full re-probe approaches the cost of a review dispatch (**S-TYPED CLEAN review median $3.03 at median 1,071 s** [measured, brief §D cost table — *not* F31's ledger row, which carries S-TYPED's totals]), operators will not run it, dates will go stale in place, and the dated-registry design fails in practice rather than in principle.

**SM-154 — Second-harness validation spend** · *discriminator on cost; detection is a saturated precondition*
- **Instrument:** total `total_cost_usd` and wall clock for one E3-shaped validation set through a candidate harness adapter.
- **Baseline:** **$56.47** for E3 arm (b), median wall **128 s** [measured, F21 and §D]; the paired arm (a) cost **$97.39**, a **1.72×** ratio [measured, F23]. S-CODEX is budgeted at **46 Codex dispatches** [documented, proposal §8] and is **unrun**.
- **Target:** a candidate adapter's validation set completes within the same order as **$56.47**, so ADR-0007's bar is affordable to re-run whenever the harness's platform surface moves.
- **Sampling:** once per candidate adapter and once per re-validation after a pinned-version bump; **report cost, wall and detection together — never cost alone.**
- **Detection is a saturated precondition, not a result.** E3's detection was **23/23 vs 23/23** at **McNemar p=0.625**, and the brief labels the primary metric "saturated — no discriminating power". A candidate harness matching 23/23 is **a floor to clear, never evidence of parity**; the discriminating quantities here are cost and wall.
- **Kill:** a validation set costing more than **~$113 (2× the measured $56.47)** — **a chosen line, not a measured threshold** — means the Harness port's second adapter is untestable in practice, and ADR-0007's "validation only" becomes "never validated".


---

## 2. Baselines at a glance

Every measure in §1 that carries a baseline figure. **75 of 134.** Seventy-three are `[measured]`; two (SM-140, SM-141) carry a `[documented]` baseline and are marked as such. The other **59** measures have no baseline at all and are listed in §5. The last column is the one that matters most: **several of these figures are upper bounds, not values, because the corpus ground truth was wrong for a majority of items** (CLEAN: ≥13 of 23 = 57%), and several are baselines *of a different quantity* than the measure that carries them.

| measure | baseline | n | source | what it is a baseline **of** |
|---|---|---|---|---|
| SM-002 | 8,408 / 10,314 / 14,579 tok = 0.45× / 0.55× / 0.78× of 18,603 | 3 of 27+3 | F38, PARTIAL | mid-graph resume cost, short-duration draws only; **not** long-gap resume |
| SM-005 | $0.021–0.027 haiku · $0.45–0.64 S-SILENT · $1.06–4.89 CLEAN · median $3.03 · arm total $112.25 vs $64.61 = +74% | 51 valid (S-TYPED) | F3/F21/F23/F31 | per-dispatch and per-review spend; the +74% is an **arm-level** ratio, not a per-review delta |
| SM-006 | 0.0098 five_hour / draw; 0.0017 seven_day | 60 draws | F29 | headroom draw **as a batch mean**; supersedes F21's 0.077 (7.8× too high) |
| SM-010 | conformance 52/52 = 100%; repro fill 147/147; re-prompts 0 | 52 | F31 | one card/schema pair, **not** a programme-level rate |
| SM-014 | unsat 23.5→11.4→0.3%; dead 15/101→6/102; ties 0; cycles 0; 27 s | 1 enumeration | F35 | one catalog on one machine; ties/cycles are now build failures and cannot recur |
| SM-015 | modal identity 5/20, 3/20, 2/20, 2/20 vs an 80% line; alpha −0.168 | 400 dispatches = 20 items × 5 repeats × 2 models × 2 runs; 4 readings (2 packs × 2 runs), 10 draws each | F36 | LLM context-vector **instability**; the kill fired 4/4 |
| SM-016 | unknown: novelty 46.0%, size 39.5%, blast_radius 31.0%, criticality 27.0%; external_dep 200/200 false | 200 draws | F36 | per-axis unknown rate → **forecast operator gate load**, not model quality |
| SM-017 | deviation offered 71, used 0 | 61 runs / 297 records | F37 | offers and uses; **the warranted fraction has an empty denominator** |
| SM-019 | lattice 27 s; $0 model spend | 1 run | F35, §D | selection cost floor; per-item latency has no baseline |
| SM-023 | 52/52 = 100% conformance | 52 | F31 | a **proxy** — typed-contract conformance, not explanation-row completeness |
| SM-025 | 27 s over 102 activities | 1 run, 1 machine | F35 | lattice runtime; **the brief records no pack count for that run** |
| SM-026 | 23.5% → 11.4% → 0.3% | 1 catalog census | F35 | unsatisfiable cross-product cells; a census, so no Wilson interval applies |
| SM-027 | 20.8% @ 8 packs → 14.9% @ 12 → 5.9% = 6/102 | 1 catalog | F35, §B | dead activities; **uninterpretable without its pack count** |
| SM-029 | 13 real / 34 false = 27.7% precision; 5 silent guesses of 27 ambiguities | 2 passes, 1 operator | F32/F33 | lint precision; **already inside its own kill line at baseline** |
| SM-030 | 4.7 → 1.3 → 0.11 errors/file | 3 passes, 1 operator | F32/F33/F34 | first-run lint errors; **confounded** — the 0.11 author read the lint first |
| SM-031 | median 56.5 · 73 (+29%) · 53 (sum 5,059/90) | 3 passes, 1 operator | F32/F33/F34 | activity verbosity; the kill (>150) never fired in three attempts |
| SM-034 | amendments 26–43 while authoring 90 activities | 1 pass | F34 | amendment volume; **"18 = 20 per 100" is an inference on an id range the brief does not gloss** |
| SM-035 | ≥1 monotonicity violation exists (mandatory + has_repro → `bisect`) | mechanism, not enumerated | F37, §C.10 | a **lower bound of 1**, not a rate — the cross-product was never enumerated |
| SM-036 | 102 catalog activities; 90 authored | 1 catalog / 1 pass | F35/F34 | shipped catalog size; **the usage half has no baseline at all** |
| SM-038 | 110/110 `plan.md` were unfilled scaffolds | 110 | S-MIG census | the **pathology** this measure exists to see; `plan.md` is removed by AC-145, so remaining kinds have no baseline |
| SM-039 | 52/52 = 100%; Wilson two-sided 95% lower ≈0.931 (computed here) | 52 | F31 | schema conformance; **saturated** — a canary for the F20 class only |
| SM-040 | $112.25 vs $64.61 = +74%; median $3.03 / 1,071 s; slowest 4,561 s | one draw per arm | F31 | **a single paired point, not a range** — a "+40% to +101% band" does not exist |
| SM-041 | median 56.5 / 73 / 53 against a >150 kill | 3 passes, 1 operator | F32/F33/F34 | authoring size; F34 clears the kill by 2.8×, so the target cannot plausibly be missed |
| SM-042 | 0.11 / 1.3 / 4.7 errors/file | 1 author per point | F34/F33/F32 | first-run lint errors; three observations, **not a trend** |
| SM-044 | 19/153 = 12% specs lack GWT or verification; 0/156 reports carry per-AC evidence columns | 153 / 156 | S-MIG census item 3 | AC field presence in the **historical** corpus, one repo, one frozen SHA |
| SM-045 | gate results free-form in 39/156 = 25% of reports and 19/23 = 83% of reviews | 156 / 23 | S-MIG census | historical free-form gate share; **the 83% rests on 23 files** |
| SM-049 | 12/174 = 6.9% [4.0–11.7%]; ≤1 h 3/95 = 3.2% [1.1–8.9%]; >1 h 9/79 = 11.4% [6.1–20.3%] | 174 | F19 | **world** drift at gate re-entry, one operator, one repo; the 3.6× ratio's intervals **overlap** |
| SM-052 | 0.45× / 0.55× / 0.78× of an 18,603-token stage resume | 3 of 27+3 | F38, PARTIAL | resume cost at short duration; **the duration-invariance half is unfalsifiable today** |
| SM-053 | worktree-each 0.685; S-WT w1 0.807, w2–w10 1.000 | 50 dispatches / 10 worktrees | F16 / S-WT (F30 never written) | worktree cache share; **the `--system-prompt` × worktree cell is contested and unrun** |
| SM-054 | E3 reviewer restored the fix from the object store: 42 turns, 22,853 output, 526 s, returned `approved` | 1 | F17 | an **instrument defect**, not a clone-cost baseline — none exists |
| SM-056 | 174/174 lossless; 3/3 mid-graph digest-identical | 174 / 3 | F19 / F38 | **byte-recoverability of the state carrier only** — the FSM was never replayed |
| SM-058 | gate→verdict n=174 median 10.5 min, 45% >1 h; stage→verdict n=191 median 51.6 min, p90 ~9 h, max 226 h | 174 / 191 | F18 | **two distinct series**, one operator; never fuse them |
| SM-061 | 8,243 → 3,611 cache write; MCP 4,631 = 56%; tools 280 → 34 | 1 fixture, 1 environment | F8 | contamination of a default `-p` dispatch; **saturated once the flag is honoured** |
| SM-062 | 100% fail-open for the one card omitting `StructuredOutput`; 0 after adding it | 1 configuration | F20 | **a mechanism, not a rate** — the denominator is one configuration |
| SM-063 | P1 0/400, S-VAR 0/60, P7 0/61 (units not commensurable); dispatch-level 0/400, Wilson upper ≈0.9% | 400 dispatches | F36 (+F29/F37) | **a placeholder** from spike batches under a different driver; P8 reports no void count |
| SM-064 | commits 1.000; worktrees w1 0.807 / w2–w10 1.000; time: static 29,619–29,680 created at ~66 min, share 0.000; default flat 18,603 at 65.2 min and 49.7 h | 50 dispatches; n=1 per gap | F16 / S-WT / F28 / S-H1b | prefix cache share per axis; **the two mitigations point in opposite directions** |
| SM-065 | 0.0098 five_hour, 0.0017 seven_day per draw; resolution 0.01 | 60 draws | F29 / F6 | headroom per draw as a batch mean; the **binding** family sub-limit has no key (F1) |
| SM-066 | median 1,071 s, slowest 4,561 s; E3 CLEAN 317–2,055 s, S-SILENT 97–424 s | 51 valid | F31 / F21 / F23 | dispatch wall clock by class; **no class has an n at which p99 is estimable** |
| SM-067 | $0.021–0.027 · $0.45–0.64 · $1.06–4.89 · median $3.03 · P7 $1.02 / $2.69 | 51 valid; 61 runs | F3/F21/F23/F31/F37 | per-dispatch cost by role; **P7's $9.86 mandatory arm is excluded (n=1, unreachable configuration)** |
| SM-068 | ~12 background kills against one 12,000 s survivor; issue-81 116 s in foreground | incident record | F22 | **an incident, not a rate**; one machine, one operator, cause unestablished |
| SM-073 | 0.45× / 0.55× / 0.78× of ~18,603 | 3 of 27+3 | F38 | mid-graph gate resume ratio at short duration only |
| SM-074 | median 10.5 min, 45% >1 h (n=174); median 51.6 min, p90 ~9 h, max 226 h, 48% >1 h (n=191) | 174 / 191 | F18 | gate duration distribution; **a discovery instrument — no target, cannot fail** |
| SM-080 | 174/174 (Wilson lower 97.8%); 3/3 across a real kill | 174 / 3 | F19 / F38 | resume losslessness; **already saturated at both granularities** |
| SM-085 | strict 12/23 = 52.2% [33.0–70.8%]; lenient 9/23 = 39.1% [22.2–59.2%] | 23 items | F31 | **block counts on one corpus, not accuracy** — CLEAN's ground truth is wrong for ≥13/23 |
| SM-086 | 0/23 verified false positives, Wilson 95% [0–14.3%] | 23 | F31 | **an upper bound**, and PROVISIONAL — the E3 severity adjudication is open |
| SM-089 | 7 adjudicated-true `narrowed-pin` findings with no destination | 23 CLEAN items | F31 | the size of the orphaned-advisory problem; **the sink itself is 0 by construction** |
| SM-090 | median $3.03; $112.25 for 51 runs vs $64.61 = +74% | 51 valid | F31 | execution-grade review cost; **the read-only tier is not measured** |
| SM-092 | control 0/10 rejections, Wilson 95% [0–27.8%] | 10 draws, 1 fixture family | F29 | negative-control block rate; **saturated by design** |
| SM-093 | S-SILENT 23/23; S-SUBTLE 5/5 sighted and 5/5 blind, twice; E3 23/23 vs 23/23, McNemar p=0.625 | 23 / 5 / 23 | F23/F26/F31 | seeded-defect recall; **saturated, no discriminating power, no kill** |
| SM-096 | median 1,071 s vs prose 682 s = 1.6×; slowest 4,561 s; whole suite ~250 s to >900 s | 51 valid; 236 files | F31, §D | review wall clock and the suite it feeds; **the timeout-kill rate has no baseline** |
| SM-097 | `npx vitest run <file>` 15.6 s (1 file / 6 tests) against a 236-file suite at ~250 s to >900 s = ~60× | **1 command pair** | §D | the narrowing *opportunity*, not a distribution — the field was never recorded |
| SM-098 | silent whole-suite fallback on 26/68 = 38.2%, Wilson 95% [27.6%, 50.1%] (computed here) | 68 invocations | §D | the **silent** historical rate; the declared rate has no baseline. **Descriptive report, not a measure** |
| SM-101 | AC-015: remove step 7 → RED, invert ordering → 36/36 GREEN; S0.1 9/10 test-reachable killed = 90.0% [59.6%, 98.2%] (computed here) | 1 AC; 15 mutants | F25 / F11 | mutation survival; two small points, **not a rate for a population of repos** |
| SM-102 | whole-suite ~250 s to >900 s; mutant median 323 s; single file 15.6 s; slow tail 4,561 s | 15 mutants; n=1 pair | F11, §D | verify wall clock; **timeout-kill rate has no baseline, and no whole-suite p95 was ever recorded** |
| SM-103 | 0 dispatches, $0 across S0.1, S0.2, F24, F25 and the F31 adjudication | 5 deterministic exercises | §D | that deterministic work costs nothing; **a saturated invariant, not a comparison** |
| SM-104 | 0/23, Wilson 95% [0%, 14.3%] (computed here) | 23 | F31 | **an upper bound**; prose-arm 2/23 and 4/23 are upper bounds against a ground truth wrong for ≥13/23 |
| SM-106 | median 10.5 min / 45% >1 h (n=174); 18,603 tok flat at 65.2 min and 49.7 h; drift 9/79 = 11% vs 3/95 = 3% | 174 / 191; n=1 per gap | F18 / F28 / F19 | human-gate load and its price; **one operator's working rhythm, not a population** |
| SM-107 | the killed alternative: 5/20, 3/20, 2/20, 2/20 vs an 80% line; alpha −0.168 | 400 dispatches | F36 | **why selection is code** — there is no baseline for the deterministic path |
| SM-111 | 0.0098 five_hour, 0.0017 seven_day per draw (batch means); 40 dispatches / ~1.9M tok moved seven_day 0.100 → 0.100 | 60 draws; 40 dispatches | F29 / F14 / F6 | headroom per batch; **saturates at the low end and always will at 0.01 resolution** |
| SM-113 | no-commit 1.000 · commit 0.689 · +exclude-dynamic 0.846 · +system-prompt 1.000 · worktree 0.685; w1 0.807 / w2–w10 1.000; TTL full hit 45.1 min, 0.690 at 61.6 min; noise floor 0.584–1.000 at n=2 | 50 dispatches; n=1 per gap | F16 / S-WT / F10 / F5 | cache share **per cell**; unmeasured cells carry no target. **Never quote a share at n<10** |
| SM-114 | 8,243 → 3,611; MCP 4,631 = 56%; ~17,439 shared + ~7,827 per-directory, 19/19 fresh repos; review-scale ~36,160 / ~29,680 | 1 fixture; 19 repos | F8 / F13 | cache write composition; **the stagger half has no baseline and is a decision rule** |
| SM-117 | 16.2 h / 60 draws; E3 128 s vs 242 s; S-TYPED median 1,071 s, slowest 4,561 s; adversary 676 s; mutants 323 s; suite ~250 s to >900 s; gate max 9.4 days | 60 draws; 51 valid; 174/191 | F29/F23/F31/F37/F11/F18, §D | wall clock as the actual binding constraint; **its kill already fired historically at 900 s** |
| SM-118 | five saturated metrics: 23/23 vs 23/23 (p=0.625) · 20/20 · 52/52 · 174/174 · Jaccard +0.000 | the programme | F23/F26/F31/F19/F28, §C.12 | the register's own method control — **the count of metrics that stopped informing anything** |
| SM-121 | 0 dispatches, $0 across F24, F25, F32/F33, F34, F35 | 5 exercises | §D | that configuration-time analysis is free; **saturates at 0 by design** |
| SM-123 | 13 real / 34 false; 4.7 → 1.3 → 0.11 errors/file | 10 activities; 90 files | F32 / F34 | rule yield and authoring ergonomics; **three passes in one session lineage, not three humans** |
| SM-125 | 12/174 = 6.9% [4.0–11.7%]; ≤1 h 3.2% [1.1–8.9%]; >1 h 11.4% [6.1–20.2%]; 45% / 48% of gates >1 h | 174 / 191 | F19 / F18 | **world drift**, used as the band a void rate must stay inside; one operator, one machine |
| SM-127 | one mechanism: `structured_output` absent at exit 0, `subtype: success` | 1 fixture | F20 | the **mechanism** of a silent success; no production rate exists, and the `is_error` half has no evidence at all |
| SM-133 | spec sections 98% · frontmatter 153/153 · ACs 88% (134/19/0) · TDD 91% (142/7/7) · gates 75% (117/31/8) · review gate table 17% (4/7/12 of 23) · companions 42% (19/6/20 of 45) · anchors 152/152 · cards 209/209 | 1,312 files / 6,877 records, pin `47faec9f` | S-MIG | **upper bounds** on a fresh corpus: the census instrument was iterated four times against this same corpus. Four slots are saturated at 100% |
| SM-135 | `mutation: unknown` 142/156 = 91.0% [85.5–94.6%]; 0/156 per-AC columns; free-form gates 25% of 156 and 83% of 23 | 156 / 23 | S-MIG | the **historical** unknown density, a fixed figure that never improves; only the live-authored half can trend |
| SM-137 | 2 of 1,312 files = 0.15% (two `_ledger.md`, unescaped quotes) | 1,312 | S-MIG | manual interventions **at one pin**; the interval is ~an order of magnitude wide and is not computed |
| SM-138 | $0 over 1,312 files / 6,877 sections; wall clock **not recorded** | 1 census run | S-MIG | zero model cost; **the wall-clock half has no baseline and no live target** |
| SM-139 | 110/110 `plan.md` unfilled; detection latency = "never, until a census" | 110 | S-MIG | a dead artifact's **shelf life**, not a detection latency — nothing emitted a read event |
| SM-140 | 1 imported file (`reconcile.rs`); 101 old CLI test files to cross as specs; 0 ports | — | proposal §5/§6 **[documented]** | the sanctioned legacy surface. **Content hashing detects verbatim copies only — the one import crosses rewritten** |
| SM-141 | 0 complete recorded lanes | — | ADR-0002, brief §E **[documented]** | the self-hosting switch; **a boolean, never a rate** |
| SM-154 | $56.47 arm (b), median wall 128 s; arm (a) $97.39 = 1.72× | E3 arms | F21 / F23, §D | second-harness validation spend; **detection at 23/23 is a saturated precondition, not parity evidence** |

**Reading the last column.** Six kinds of figure appear above and only the first is a baseline in the ordinary sense.

1. **A baseline of the measure's own quantity** — SM-005, SM-030, SM-049, SM-058, SM-067, SM-117, SM-133, SM-137.
2. **A baseline of a different but adjacent quantity**, carried because nothing closer exists — SM-022's world drift for advisory noise, SM-038's `plan.md` for produced-unread, SM-050's reseal rate, SM-107's killed alternative, SM-139's shelf life.
3. **An upper bound** forced by a wrong ground truth — SM-085, SM-086, SM-104, and every CLEAN-derived false-positive rate. CLEAN's ground truth is wrong for **≥13 of 23 = 57%**.
4. **An upper bound forced by instrument tuning** — every S-MIG share in SM-133 and SM-135: the census was iterated four times against the corpus it then measured, at one frozen SHA.
5. **A mechanism, not a rate** — SM-035 (a lower bound of 1), SM-062 (one configuration), SM-068 (one incident), SM-127 (one fixture).
6. **A saturated ceiling** whose only information is the moment it moves — SM-039, SM-061, SM-080, SM-092, SM-093, SM-103, SM-121, and the four 100% slots inside SM-133.


---

## 3. Metrics deliberately not tracked, and why

Three reasons a quantity is refused: **the metric saturated**, **the instrument cannot resolve it**, or **the denominator does not exist**. Each refusal below is a decision, not an oversight, and each names what would have to change before it could be reopened.

### 3.1 Saturated — measured, at ceiling, and carrying no signal

| refused quantity | measurement that saturated it | why it is not tracked as a success measure |
|---|---|---|
| **Seeded-defect detection recall as a discriminator between two configurations** | E3 **23/23 vs 23/23**, **McNemar p=0.625**; E3-H **20/20** across two harder tiers; S-TYPED S-SILENT 23/23, S-SUBTLE 5/5 | The brief's own words: "primary metric saturated — no discriminating power". Retained only as SM-093, a **one-way canary with no kill**, and as the negative control S-SUBTLE keeps in the corpus. **Never used to compare two configurations, two harnesses (SM-154) or two tiers.** |
| **Schema conformance as a quality signal** | S-TYPED **52/52 = 100%**, `repro.command` fill 147/147 | Ceiling on one card/schema pair. Retained as SM-039 / SM-023, canaries for the F20 failure class only. |
| **Suspension losslessness as a rate** | H2 **174/174**; F38 **3/3** across a real kill | Already saturated at both granularities. Retained as SM-080 — an invariant alarm, not a rate. **And it covers the state carrier only: the FSM was never replayed.** |
| **Digest sufficiency (H4)** | Jaccard FULL 1.000 = CONTROL 1.000, **effect +0.000** | The experiment answered its question and the metric has nowhere left to move. Not carried into production telemetry at all. |
| **Any lane-level "pass rate"** | §C.12: five metrics saturated; every discriminating result came from cost, stability or mechanism | Refused outright. Self-hosting is a **boolean plus distributions** (SM-141), not a success rate. |

### 3.2 The instrument cannot resolve it

| refused quantity | the resolution limit | consequence |
|---|---|---|
| **Per-run headroom cost** | `utilization` is quantised to **0.01** (F6); a CLEAN draw is **0.0098**, roughly one tick | Only batch deltas are reportable (SM-006, SM-065, SM-111), always with a `resolution_limited` flag and n ≥10 dispatches. **SM-006 therefore carries no kill at all.** F21's 0.077 is superseded (7.8× too high). |
| **Whether cache reads draw down the window** | E0's cache-cost gate was **STRUCK, not answered**; 40 dispatches / ~1.9M tokens moved `seven_day` 0.100 → 0.100 (F14) | AC-381 encodes the **negative** — no cache metric may feed a headroom figure. Reopen only if the platform exposes finer resolution. |
| **The binding Opus/Sonnet family sub-limit** | **No family sub-limit key exists** (F1); only `unifiedWindows.{five_hour,seven_day}` | Every throttling measure runs on a **proxy**. AC-375 forbids inventing the missing value; SM-112's kill is what tells us the proxy failed — and that kill may itself be unable to fire, because no overage payload was ever recorded. |
| **Cache TTL split (1 h vs 5 m)** | `usage` uses plain cache naming and exposes **no TTL split**; `ephemeral_1h_*` / `ephemeral_5m_*` are absent (F3) | Charter E1's split metric is **not computable from this instrument** and is not carried. The TTL itself is bracketed **(45.1, 61.6 min) and never located** (F10, n=1 per gap, date-stamped platform behaviour). |
| **p99 for any dispatch class** | The largest class in the corpus is **51 valid** (F31) | SM-066 states a **recorded-maximum** target instead, and p99 becomes reportable only at n ≥100 per class. |
| **Whether a partially inert test filter is doing its job** | AC-326 detects only a **fully** inert filter; a wrapper selecting 200 of 236 files is recorded as `affected` | SM-099 is detection-after-the-fact, not a gate. **This is the largest open hole in the ~60× defect defence** and it is disclosed rather than fixed. |
| **A semantics-only schema change** | A schema snapshot diff cannot distinguish a field-semantics change from no change | No measure is invented for it; it is the old-redkiln failure ADR-0006 exists to prevent, and it stays visible as a known gap. |

### 3.3 The denominator does not exist

| refused quantity | the missing denominator | status |
|---|---|---|
| **Warranted-deviation fraction** | Deviation was **offered 71 times and used 0** (F37). P7 §4.4(3) is recorded as *not answerable*, **not as 0%** | SM-017 tracks offers, uses and warranted-on-review as three counts, with only a floor kill (0 uses over 100 items). No fraction is quoted. |
| **Second-arrival rate, from any spike** | P8 §5.4(3) is **circular by construction** — arrivals were forced and the mark was minted by the driver implementing the semantics under test | SM-078 is a **discovery instrument** with no target and no kill. AC-271's semantics are a declared design choice with no evidence, and no run can settle it. |
| **Gates per item** | F18 records **174 gate→verdict and 191 stage→verdict marks; the item denominator is not recorded** | SM-082 is a discovery instrument. Frequency is the costly dimension (§C.2), so it is tracked — but no rate can be derived from the corpus. |
| **Per-facet alpha as a decision variable** | P1's alpha rule fired twice but **the predicate is not decidable on this corpus** | Alpha is recorded (drift −0.168 on byte-identical anchors) and is **not** the decision variable. SM-015's decision variable is modal required-set identity. |
| **P1 arm D precision and recall** | **Not computed — awaits Ryan's blind panel** (brief §E) | AC-071's gate requires `panel_verdict == "scored"`. Until then no accuracy claim about the emitted vector exists, only an instability claim. |
| **P2's precision/recall lines** | **P2 is entirely unrun**; all three criteria unanswered, labels unwritten | SM-021 records that the measurement is owed. Any number quoted for it today would be fabricated — a previously stated "recall ≥0.90, precision ≥0.70 at n=20" was struck for exactly that reason. |
| **The blocker-recall answer key** | **F9 has no ledger entry**: all 15 `_review.md` Required-Changes sections read "None"; the key does not exist and cannot be synthesised | Struck from the programme; it forced pre-registration amendment 1. **No measure here asserts recall against a key**, and AC-076 routes the question to a human gate with that reason recorded. |
| **Review p90 against an item's gate budget** | **No gate budget is defined anywhere in the evidence** | Dropped from SM-096: a half-target that can neither pass nor fail is not a target. |
| **Accuracy against CLEAN's ground truth** | That ground truth is **wrong for ≥13 of 23 = 57%** (brief §E, contested) | Every FP rate against it is an **upper bound** (SM-085, SM-086, SM-104). No accuracy metric is derived from CLEAN; only counts, adjudicated blocks and cost. |
| **AC-level pinning rate, historical** | **No AC-level pinning rate has ever been measured**; the "≥12 of 23 = 52%" figure was a mis-labelled block count and was cut | SM-100's ≥0.90 target is declared an **aspiration set with no baseline**, not an inference from evidence. |
| **Read-only review tier cost and recall** | Only the execution-grade tier was measured (F31); the read-only price is borrowed from an external vendor | AC-301 refuses the cascade rather than pricing it. **"No cascade until its recall loss is measured" — a Phase 2 spike, not a Phase 1 default.** |
| **Dead roles, skills and slots** | The only dead-reference measurement in the corpus is for **activities** (F35) | AC-094 is machine-checkable but carries **no measured baseline**, and must not be presented downstream as evidence-derived. |
| **Semantic fidelity of a migrated artifact** | "Lifted" is not "lossless": only the criteria and ledger lifters check internal completeness; other tables are counted by row | No measure of semantic fidelity exists and **none is invented**. AC-468 forces the report to keep the distinction; SM-133 measures lift, not meaning. |
| **The old redkiln during parallel running** | Unverifiable from this repository | No criterion and no measure constrains it (AC-474 says so explicitly). |
| **Catalog size against an external anchor** | **No external comparator for catalog size exists** | Cloudflare's 0.6% is a rarity threshold for an *escape hatch* and **must not be borrowed** as a catalog-size line (SM-036). |

### 3.4 Refused shapes of reporting

- **Composite scores and single overall percentages.** SM-133's overall 98% is exactly the number that hides its 17% slot; SM-014 forbids a composite of ties, cycles and unsatisfiability. Report the components.
- **Means where a tail sets the decision.** SM-040, SM-066, SM-096, SM-102, SM-117: the slowest S-TYPED draw is 4.3× its median, and the timeout history was written by maxima, not medians.
- **Rates at n<10 for cache share.** F5 measured 0.584 / 0.698 / 1.000 / 0.722 on an identical prompt at n=2 and the brief labels it "explicitly NOT a result".
- **Pooling incommensurable units.** SM-063's three spike batches count dispatches, draws and graph runs; the pooled 521 denominator is heterogeneous and its interval is indicative only.
- **Fusing F18's two series.** Gate→verdict (n=174) and stage→verdict (n=191) are different distributions. Every previously fused figure in the area files — a "1,300× median", an "n=174/191", a "median 10.5 min with a max of 226 h" — was withdrawn.
- **A rate without its pack count.** SM-027's dead-activity share moved 5.9 points between 8 and 12 packs with the catalog unchanged.

---

## 4. External comparators

Five external anchors appear in this register. **None of them is a threshold**, because none measures our quantity on our corpus with our instrument. Each entry says what it *is* comparable to and what it is not.

**Cloudflare — break-glass rate: 288 of 48,095 MRs = 0.6%** [documented, 1P].
*Comparable to:* the order of magnitude at which a deliberate escape hatch stays exceptional. Used as an ordering anchor by SM-018 (unsatisfiable gates), SM-079 (`auto-decide` expiry paths), SM-086 and SM-104 (verified false blocks), SM-095 (quarantine overrides).
*Not comparable to:* any of those quantities directly. It is a human overriding a machine block in a different organisation's merge process, at a scale (48,095 MRs) this programme will not see for years. **It bounds an order of magnitude; no threshold derived from it is a pre-registration.** It is explicitly *not* a catalog-size line (SM-036) and *not* a payload-override rate — a measure built on that reading was cut with SM-150.

**Cloudflare — review cost: read-only median $0.98, full tier $1.68** [documented, 1P; 131,246 runs over 30 days].
*Comparable to:* the low end of a per-review cost band, and the strongest evidence that size-tiering cuts trivial-review cost by roughly 8×.
*Not comparable to:* our execution-grade review, which runs a repro for every finding — something Cloudflare does not publish. The $1.68 figure is **the borrowed price of a read-only tier we have not measured** (SM-090's gap), and AC-301 refuses the cascade rather than pricing it from this number.

**Anthropic Code Review — $15–25 and ~20 min per review** [documented, 1P].
*Comparable to:* the upper bound of the same band. SM-090 targets execution-grade review inside **$1.68–$15**; SM-067's second kill fires if a review-lane median reaches $15, at which point the cost argument for self-hosting the lane disappears.
*Not comparable to:* a quality comparison. Neither side publishes detection on the other's corpus, and our own detection metric is saturated.

**Agentless — $0.70/instance at 32.00%, against CodeR $3.34 at 28.33%** [documented, peer].
*Comparable to:* a per-item cost-and-accuracy *point* in the same rough regime, and the design's most-cited demonstration that a cheaper deterministic pipeline can outscore a more expensive agentic one. SM-005 uses $0.70 as the floor of its target band.
*Not comparable to:* our accuracy at all — the 32.00% is SWE-bench resolution, a different task with a different oracle. **The reproduction figures once attached to it in draft criteria ("213 of 300 reproductions, 94 flipped") are not in the evidence brief and were cut from three places.** Only the cost/accuracy point survives.

**Google — mutation testing at review time, a median of one live mutant per changelist (ICSE 2021)** [documented, peer, via the proposal].
*Comparable to:* the plausibility of our own per-AC mutation findings — it puts a 52% unpinned-criterion rate in range rather than out of it, alongside Inozemtseva & Holmes's result that the coverage–effectiveness correlation drops once suite size is controlled (which is what a 1,907-test green suite under a live mutation looks like).
*Not comparable to:* our metric. Ours is **criterion-directed** — the reviewer chooses the mutation to attack a stated AC — so it should run higher than blind operator mutation.
**Contested provenance, recorded:** the review-gate refutation **cut** this comparator on the ground that it does not appear in the evidence brief; the migration file **retains** it as proposal-sourced. This register keeps it in SM-135 with that disagreement stated, and **no threshold anywhere depends on it.**

**Single-project lessons (shipmates) — ~7% of commits are pure re-syncs; five blind releases before an upgrade e2e; a namespace retrofit costing ~9 issues at 30 days** [documented].
*Comparable to:* the shape of a failure mode in a comparable plugin-distribution project, and the only prior for SM-145, SM-146 and SM-149.
*Not comparable to:* a rate for this system. **One project, one maintainer, unreplicated, and reaching this register as a secondary retelling** — the underlying issue numbers could not be checked against any primary. SM-149 is additionally **conditional on a distribution mechanism that has not been decided.**

**Two precedents that carry no number.** GitLab's flaky-test quarantine and Cloudflare's "won't fix" state are the design precedents for `Quarantined { reason, by, until }` as a first-class state (design recommendation §2 rule 4). **Both were cut from the review-gate and verify-gate criteria as absent from the evidence brief**, and neither supplies a measurable comparator. They justify a *type*, not a *target*. The same applies to Agent OS v3's intuition-retired phases and Scott Logic's Spec Kit accounting (2,577 lines of markdown for 689 LOC), which SM-139 cites to establish that **no surveyed system instruments downstream reads** — an absence, not a rate.


---

## 5. What must be measured before a target can be set

**59 of 134 measures have no baseline.** None of them can fire a kill on its first reading; each establishes a level, and only the second reading can detect a regression. This section names, for each, the specific thing that establishes it — a first production window, a named unrun spike, an undeclared threshold, or a component that does not exist yet.

### 5.1 Waiting on a first production window

These need nothing but the system running and recording. Each names the window that makes the first reading meaningful.

| measure | what first establishes it |
|---|---|
| SM-001 Memo effectiveness | The first production window with a live memo table, reported in the three mandatory buckets (outside loops / in loops / `memo = false`). Bucket (a) alone is a meaningful rate. |
| SM-003 Retry composition | The first outer-layer `activity_retry` events. **The inner-layer half additionally needs a captured `system/api_retry` payload** — see §5.3. |
| SM-004 Which budget binds | The first window of `budget_exhausted` events. Corpus anchors exist for the wall-clock axis only (1,071 s / 4,561 s / 323 s / ~250–900 s); the attempts and tokens axes have nothing. |
| SM-007 Circuit-breaker trips | The first trip, with its K constituent failures attached. **The rate will never be testable;** only the classification check can fire. |
| SM-008 Degradation frequency | The first substitutions, reported as a `why` distribution. F37's "offered 71, used 0" is the shape to watch for. |
| SM-009 Replay divergence | The first nightly replay of recorded runs. **Nothing in the programme has ever replayed the FSM.** |
| SM-013 DAG replay identity | The first `redkiln replay --run <id>` over recorded runs. Same gap as SM-009. |
| SM-018 Unsatisfiable-gate rate | **n ≥100 live stage entries**, reported with a Wilson interval and the `(stage, kind)` profile behind it. |
| SM-020 Memo hits and unsound evictions | The first memo lookups. The hit rate will saturate; the eviction count is the reading that matters. |
| SM-022 Advisory drift on vouched roots | The first `drift_advisory` events, and **50 advisories** before the acted-upon-fraction kill can fire. |
| SM-024 Human-supplied facet load | The first triage gates under AC-071's all-human facet regime. The cost basis (18,603 / 29,619–29,680 tokens, 10.5 min median) is measured; the load is not. |
| SM-028 Config defects at configuration vs dispatch time | The first window with both counters live. Expected to saturate at zero; the reading is a **gap in the rule table**. |
| SM-032 Quality-block coverage | **The delivered-artifact subset must be computed first.** F34's 42 of 90 = 46.7% is over the *authored* set — a different denominator — so it is not a baseline for this measure at all. |
| SM-033 Binder ambiguity | The first `doctor` invocations across more than one repo and pack-set digest. |
| SM-037 Structured-section drift | The first 20 items, which is also the rolling window its kill uses. |
| SM-043 Section-locator failures | The first reconciles that resolve an anchor — **with document byte length and heading count on the event**, without which the kill cannot fire. **No spike measured the locator at all.** |
| SM-046 Unmigratable stored artifacts | The first schema-version migration and the first release tag after it. |
| SM-050 Orphan and rewrite volume | The first entries producing orphan rows; **10 consecutive entries** before the monotonic-growth kill can fire. |
| SM-051 Rebuild fidelity and cost | The first `doctor --rebuild` on the pinned corpus. **No prior measurement of rebuild exists** — F12's 360 commits / 74 slices are the substrate the first run will use, and nothing more. |
| SM-055 Merge-queue re-verification | The first merges keyed by tree hash. |
| SM-057 Store-open refusals | The fixture suite's first run (the production number will stay at zero; **the fixture is the instrument**). |
| SM-069 CLI version drift | The first CLI upgrade observed against a recorded pin. |
| SM-070 Pre-spawn refusals | The first window reporting refusals per 100 dispatch attempts **alongside the same errors caught after spawn**. |
| SM-075 Void rate at answer | The first answers against tree-hash-bound gates. **No gate in the corpus was tree-hash bound, so nothing was ever voided;** F19's 6.9% [4.0–11.7%] world-drift rate is the proxy the target is set against, and the 3.6× duration split is directional only. |
| SM-076 Gate hang count | The first `doctor` runs. Saturates at zero by construction; a single non-zero result is the entire signal. |
| SM-077 Model spend while a gate is open | The first gates with dispatch counting across the open interval. Currently `[inferred]` from F28's mechanism, **not measured**. |
| SM-078 Answer arrivals per open gate | The first gates in production. **No spike can establish it** — P8's second-arrival criterion is circular by construction. |
| SM-079 Expiry terminal-path mix | The first expiries. **The corpus had no expiry mechanism, so no gate could expire** — that is absence of instrumentation, not observed absence; the longest observed gate was answered at 226 h. |
| SM-081 Void rework cost | The first `gate_answer_void` followed by a re-answer, with machine cost and human latency reported separately. |
| SM-082 Gate frequency per item | The first items with a recorded gate count. **F18 supplies numerators only; the item denominator was never recorded.** |
| SM-087 Confirm-half determinism | The first k=10 replay job on the pinned corpus — **and a pinned non-empty expected digest**, without which an all-empty block set scores 10/10. |
| SM-088 Unverifiable share of findings | The first reviews reporting `both_fail` / `cannot_run` / `timeout` separately. **No direct baseline for this pipeline;** F37's "78% of 188 findings required running something" is the nearest measured shape, and the <10% target is chosen, not derived. |
| SM-091 Model label vs computed class drift | The first findings carrying both labels. Diagnostic; never gated. |
| SM-095 Quarantine rate and age | The first quarantines granted. |
| SM-099 Silent-selection-failure detections | The first `basis: affected` attempts with the executed argv recorded. **The historical rate is unmeasurable — the argv was never recorded.** |
| SM-105 Selection-probe coverage | The first toolchain to carry a `{capability, verified_on, probe}` row. **Zero carry one today.** |
| SM-109 Attribution completeness | The first monthly reconciliation against the account's billed spend. The compile-time half cannot fail. |
| SM-110 Cost per completed item | The first completed items per lane. P7's $1.02 / $2.69 are graph-run analogues, not lane costs. |
| SM-112 Throttle refusals and overage | The first margin refusals — **and, for overage, a recorded overage envelope, which has never existed** (§5.3). |
| SM-115 Per-item token appetite | The first items completing or halting against an appetite fixed at triage. |
| SM-116 Tier-2 disposability | The first tier-2 append failures. The next-action half only re-runs AC-365's own test. |
| SM-122 `doctor` wall clock | The first `doctor` run over the shipped 102-activity catalog. |
| SM-124 `doctor --rebuild` next-action fidelity | The first delete-and-rebuild CI job on the pinned corpus, reporting **item ids, not a rate**. |
| SM-126 `explain` sufficiency | **The first 30 gate answers**, reported as raw counts. The >15-of-30 kill is this register's pre-registration and **needs sign-off from the PC2 owner before it is a decision line**. |
| SM-128 Structured-field write-path violations | The first reconciles emitting `structured_drift`. **No measured precedent survived checking.** |
| SM-129 CLI-version pin drift | The first dispatch under a version-pinned event schema. |
| SM-130 Re-dispatch of a memoised committed step | The first runs with memo-keyed dispatch counting **at the gap durations that matter (65 min, 24 h), which have never been run**. |
| SM-134 Section conservation | The first `redkiln-migrate --report` run enforcing the AC-451 identity. |
| SM-136 Migration determinism | The first pair of migrate runs at the same (corpus SHA, tool version), including one under shuffled directory order and a different `TZ`. |
| SM-145 Derived-artifact re-sync tax | **The first 100 commits after `rk-plugin-gen` lands (Phase 2).** |
| SM-146 Upgrade-path escapes | **n ≥5 published releases** — a single clean release is not evidence, and the interval is not computed below that. |
| SM-147 Capability-claim age | The first `capability_used` events carrying `verified_on`. |
| SM-148 `Unsupported` incidence | **A second adapter behind any port — not scheduled before Phase 4.** Knowingly saturated at zero until then. |
| SM-151 Fake/real conformance divergence | The first green AC-499 CI matrix, reported **with its assertion count** — a suite with three assertions has a meaningless zero. |
| SM-153 Probe cost and wall time | The first full-registry `doctor --probe` run. |

### 5.2 Waiting on a named unrun spike or an open adjudication

| measure | the named blocker |
|---|---|
| SM-021 Short-circuit precision/recall | **P2 is entirely unrun** — all three criteria unanswered, labels unwritten, and the engine phase refuses until they are timestamped. **Its registered lines must be read out of P2's pre-registration before this row has a target;** the brief does not carry them. |
| SM-015 (target re-run) Context-vector stability | **P1 arm D precision/recall is not computed** — it awaits Ryan's blind panel. AC-071's gate requires `panel_verdict == "scored"`, so until the panel is filled the block cannot lift even if a re-run cleared the 80% line. |
| SM-052 / SM-073 / SM-130 duration-invariance | **P8's 65-min arm, 24-h arm and all three controls are unrun** (3 of a pre-registered 27+3). The duration-invariance half of each target is **unfalsifiable in this programme's data**. |
| SM-064 / SM-113 static-prompt time axis | **The S-H1b 24-h read was never taken**, and F30 was never written, so the static-prompt numbers exist only in the design recommendation and the proposal. No static-prompt figure exists at 49.7 h. |
| SM-064 append-route fallback | **S-H1c is registered and unrun**, and ADR-0016 is conditional on it. `cache_profile` gains a third value only if it runs. |
| SM-053 worktree × static prompt | **The `--system-prompt` × worktree combination is unrun**, and the evidence is *contested*: §C.1 states `--system-prompt` holds 1.000 across worktrees w2–w10 per S-WT, while F16's worktree-each arm measured 0.685 unmitigated. **Resolving this needs the S-WT run record — which prompt configuration it used — not another spike.** A cost model shipped with 0.685 hard-coded would misprice fan-out by ~31× if §C.1 is right. |
| SM-086 verified false-block rate | **The E3 severity adjudication is open** (facts settled 6/6, severity not). Both FP rates and both MCCs in the corpus stay provisional, and so does this baseline. |
| SM-133 / SM-135 migration shares | `redkiln-migrate` **does not exist**. The baseline came from a read-only census; **the target cannot be read as a regression until the migration tool reproduces the census figures on the same pin.** |
| SM-154 second-harness spend | **S-CODEX is unrun** (budgeted at 46 Codex dispatches in the proposal). |
| SM-094 / SM-100 per-AC mutation | The per-AC mutation harness does not exist. F25 is **n=1 AC**; the 0/156 report-corpus figure is proposal-sourced and **not corroborated by the evidence brief**. |
| SM-149 Install repair incidence | **The distribution mechanism is undecided.** The proposal decides a plain git clone plus a JS shim fetching the binary from GitHub Releases — no installer, no receipts, no ownership states. This measure and the criteria around it are **unbuildable until that section is reopened**, and must not be read as a Phase 1 requirement. |

### 5.3 Waiting on a payload or a field that has never been recorded

| measure | what is missing |
|---|---|
| SM-003 (inner layer) | `system/api_retry` is real as an **event-type census entry** in `spikes/results/e3.jsonl` (1–2 per run), but **its payload was never captured**. Whether it carries an attempt counter is unknown, and AC-026 may be unimplementable as written. |
| SM-112 overage detection | **No overage envelope was ever recorded.** The field that classifies a dispatch as overage is unknown; `isUsingOverage` appears in `spikes/FINDINGS.md` but not in the brief's F1 row, which records only the two `unifiedWindows` utilization keys plus the *absence* of a family sub-limit key. AC-378's detection half is PROVISIONAL. **SM-112's kill may be unable to fire until this is closed.** |
| SM-127 (`is_error` half) | The `is_error` / `api_error_status` frames have **no evidence anywhere in the brief**, and the unit test that would verify them needs a recorded stream fixture that may not exist. If none is ever found, that half should be deleted rather than shipped as provisional. |
| SM-129 / SM-069 CLI surface | `CLAUDE_CODE_PROMPT_CACHE_TTL`, `--permission-prompts`, `--bare` and `isUsingOverage` appear nowhere in the brief. **One recorded real stream and one recorded argv close this**; until then the tests are green against fixtures we wrote ourselves. |

### 5.4 Waiting on a threshold nobody has chosen

Three measures have an instrument, a baseline or both, and **no declared number**. Until the number is declared they are placeholders, and counting them as live measures is the exact false assurance this register exists to prevent.

| measure | the undeclared number |
|---|---|
| SM-138 (wall-clock half) | No migration run time was ever recorded, so there is no CI budget. **The dispatch half can be missed; the wall-clock half cannot.** |
| SM-139 | **N is undeclared** — "flagged by `doctor` within N runs" has no N, and AC-472's threshold fixtures (N−1 must not fire, N+1 must) do not exist. |
| SM-126 | The >15-of-30 kill line is **this register's own pre-registration**, not a measurement, and needs the PC2 owner's sign-off. |

### 5.5 The honest count

Of 134 ids: **75 carry a baseline figure and 59 do not.** Of the 75, at least twenty are a baseline of a different quantity, an upper bound, a mechanism, an incident or a saturated ceiling rather than a value (§2's closing note). And **48 of the 134 rows in §1 carry a label that is not "discriminator"** — tripwire, canary, discovery instrument, tracker, descriptive report, invariant alarm, guardrail, construction check, covariate, placeholder or an explicit *not a discriminating measure*. **Those 48 cannot fail in the sense that matters, and they are not assurance.** The number of measures that can both fire a kill and discriminate a good outcome from a bad one on evidence that exists today is **considerably smaller than 134**, and every area file says so in its own words. That is the register's most important property, and no summary of it should round it away.

---

## 6. Sources

**Primary evidence.**
- `spikes/FINDINGS.md` (F1–F38; **there is no F9 entry**, and **F30 was reserved for S-H1b/S-WT and never written**), as condensed in the session evidence brief §A (finding ledger), §B (kill criteria and outcomes), §C (architecture-bearing implications C.1–C.12), §D (cost, token and latency figures) and §E (open items, pre-registrations found broken, unrun spikes, contested figures).
- `spikes/F29-DRAFT.md` .. `spikes/F38-DRAFT.md`; `spikes/DECISIONS-2026-09-09.md`; `spikes/METHOD*.md`; `spikes/HANDOFF.md`.
- `spikes/results/s-mig-report.md` and `spikes/corpus/PINNED_SHA` (corpus pin `47faec9f`) — the source of every S-MIG denominator quoted in SM-038, SM-044, SM-045, SM-133, SM-135, SM-137, SM-138 and SM-139. The brief carries no S-MIG entry.
- `spikes/results/e3.jsonl` (the `system/api_retry` event-type census), `spikes/results/e0-headroom.jsonl` (`resets_at` on `rate_limit_event`), `spikes/results/p1/` (arm L stability records).

**Design and decision sources.**
- `references/2026-09-05-architecture-proposal.md` (rev. 4) — §4.2/§4.2.1 packs, binder and memo key; §4.3 harness contract; §4.4 typed artifacts and schema design rules; §4.5 git as truth; §4.6 the review gate; §4.7 events, cost and parallelism; §4.9 distribution; §5 the clean break; §7 phasing; §8 the spike table.
- `references/2026-09-07-design-recommendation.md` — the scorecard, the review-gate rebuild, the cost section and the prefix table.
- `references/2026-09-07-process-catalog-proposal.md` — §2.5 the seven steps, §2.6 the catalog lint.
- `references/2026-09-07-activity-graph-schema.md` — §5, §9a, §9b.
- `references/adr/` — the Phase 1 ADRs cited by the area files, including ADR-0002 (clean break), ADR-0004 (findings not verdicts), ADR-0005/0006 (prose-free structure, layout C), ADR-0007 (second harness), ADR-0011 (Phase 1 exit criteria), ADR-0016 (static prompt, conditional on S-H1c), ADR-0017 (the gate mark), ADR-0019 (per-AC mutation as the norm), ADR-0021 (the loop-aware memo key), ADR-0024 (the two-home split), ADR-0027 (catalog ship/extend), ADR-0037 (budgets), ADR-0056/0057/0073 (rank, families, toolchain-keyed kinds).
- `references/2026-09-01-shipmates-lessons-for-redkiln-rs.md` — the single-project distribution lessons behind SM-145, SM-146 and SM-149.

**External comparators** (§4): Cloudflare's published reviewer and merge-request figures [1P]; Anthropic Code Review pricing [1P]; Agentless and CodeR [peer]; Petrović et al. on mutation testing at review time [peer], via the proposal and **contested for provenance**; Inozemtseva & Holmes on coverage and effectiveness [peer]; GitLab flaky-test quarantine and Cloudflare "won't fix" [1P, precedent only, no number]; Agent OS v3 and Scott Logic's Spec Kit accounting [documented, via the design recommendation].

**A standing limit on the citation layer.** The thirteen area files' refutation passes verified every F-number, spike name and figure against the evidence brief, and struck or re-tagged what failed — that pass cut citations from more than forty places. **A subsequent verification against the full corpus found that pass over-struck.** The refuters were told to check against the condensed brief, which is a subset, so real citations absent from it looked fabricated: "+40% to +101%", "30.1% unsatisfiable", "spike P3", "spike P4", "spike P5", "F35 §1.3", "GATE-02", "GATE-05", "SDK issue #571", "shipmates L2.5", "0/156 historical reports" and "≥12 of 23" are all **real**, and the acceptance-criteria register carries the table of their true sources. Only F9 stays struck, because it is cited corpus-wide but has no ledger entry. **The brief contains no ADR text and no proposal text**, so every ADR number and every `§` reference reaching this register through an area file is `[documented]` and **unverified by that pass**. Roughly a third of the surviving criteria rest on such pointers. That is the largest residual risk carried into this register, and it is recorded here rather than left implied.

