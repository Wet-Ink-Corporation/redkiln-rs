# Pre-registration #7 — P: the process-catalog spikes (P1, P2, P6, P7, P8)

**Status:** pre-registration, per charter §8.2 (amendments 6–7 applied 2026-09-08 by Claude on Ryan's authorization; post-data, additive — see §3.2 and the closing note. **Amendment 8 applied 2026-09-09**, also on Ryan's authorization: it changes how §0 rule 1 is *verified* for P1 arm D, not what it requires, and changes no kill criterion — see §0 rule 1). **Written before any P1/P2/P6/P7/P8 dispatch or analysis. No P data has been seen.** P9 (`METHOD-P9.md`) and its second-author follow-up P9b are already run and reported (`F32-DRAFT.md`, `F33-DRAFT.md`); their results shaped this document and are cited where they do.
**Baseline:** this document's commit. **Corpus pin:** redkiln @ `47faec9f` (S-MIG's frozen `.bklg`). **Instrument:** `spikes/runner` (E3 rig) for P7/P8; `spikes/p9/lint.py` v2 for P6; offline analysis for P1's derivability half and P2.
**Why it exists:** `references/2026-09-07-process-catalog-proposal.md` §6/§6.1 names these spikes; P7 and P8 can overturn PC6 and the schema's escalation model, P6 gates the v0 pack, P1 and P2 fix parameters the selection engine needs before Phase 1 can author `bug-standard` and `story` packs.

| spike | question | feeds | model cost |
| --- | --- | --- | --- |
| **P1** | is the LLM-emitted half of the context vector stable, and can the derived half leave the model entirely? | PC7/PC8 facet registry; which axes become human-supplied | ~300 haiku + ~300 sonnet classifications |
| **P2** | does artifact-level freshness prune rework without skipping needed work; how noisy is PC3's advisory drift? | `derived_from` scope; `stale_policy` per cost class | zero |
| **P6** | does the v0 catalog satisfy completeness over the reachable context lattice, including mixed toolchains? | PC5 pack release gate | zero |
| **P7** | does the encapsulated `adversary → policy → gate` block (PC6) catch more per token than a single dispatch; are typed deviations warranted? | PC6 default internal rigor; the post-P7 `quality` shorthand; catalog §2.5 step 7 | ~90 Opus dispatches |
| **P8** | is resume into a mid-graph `gate` step lossless, and what does it cost? | schema §3 escalation model; §9b amendment 15; P9b A8 | ~30 haiku dispatches |

## 0. Rules inherited

Everything in METHOD, METHOD-E3, METHOD-H and METHOD-S §0 applies: isolation with adoption verified by tool-set equality; pre-flight fields on every record; a void is never a miss; resume keyed per `(arm, slug, repeat)`; results append-only under `spikes/results/`; platform errors named; throttle margins honoured; **nothing inferred from a single draw** (F27); repeats reported per item as a rate with a Wilson 95% interval, never pooled first; a typed digest that fails its schema is `void: schema`.

Two rules specific to P:

1. **Human labels are produced before the engine's output is seen.** P1's agreement panel and P2's "should re-run" set are written from the item and the artifacts alone; the classification vectors / freshness verdicts are unblinded afterwards. The label files carry a timestamp that precedes the first engine run.

   *Amendment 8 (2026-09-09):* the rule is unchanged; its **verification mechanism** gains a second, weaker form.
   As written, the rule is checked by chronology — `score-arm-d.py` refuses unless `labelled_at` precedes the engine
   output's earliest `at`. For P1 arm D that check can no longer be satisfied honestly: the AFK run of 2026-09-08
   produced `spikes/results/p1-arm-d.jsonl` (earliest `at` 2026-09-08T04:54:07Z) *before* the label template was
   filled, because the run deliberately stopped at the label boundary (§0 rule 1, AFK plan lane A5). Any truthful
   `labelled_at` written now is later than the engine's, and the script refuses permanently.

   The property the rule protects is **blindness**, not chronology; the timestamp was only the mechanically-checkable
   proxy for it. Where the proxy has expired but blindness holds in fact, the label file may instead carry a
   `blind_attestation:` line naming who attests, what they attest they did not open, and when — and the scorer must be
   invoked with an explicit `--attested-blind` flag, so the substitution can never happen silently. The attestation is
   copied onto every output record.

   **This is strictly weaker evidence.** A timestamp is machine-checkable; an attestation is a claim. It does not make
   a label blind — it records that someone says it was, and names them. Any result scored this way must be reported as
   attested-blind, never as timestamp-verified, and a reader is entitled to discount it accordingly.

   **When it was made:** after the engine data existed, and — on Ryan's attestation of 2026-09-09 — before the data it
   would affect had been seen. That is a weaker guarantee than amendments 1–5 and a different one from 6–7, and it is
   recorded as such. **It changes no kill criterion**: §1.4's precision ≥ 0.95 rule and the modal-`R` line are untouched.

   *Rejected alternatives, one line each.* Backdate `labelled_at` — falsification, and the AFK charge that "a completed
   lane with a fabricated label is not a good result" forbids it. Silently re-run `derive-facets.py` to refresh the
   timestamps — it would pass the check, because arm D is a deterministic zero-cost derivation at a fixed pin, but it
   hides the issue instead of disclosing it. Recruit a labeller who provably never saw the output — genuinely stronger,
   but none is available, and F36's decisive modal-`R` criterion needs no panel at all. Abandon arm D — the design
   already ships the hedge (human-sourced facets, `AC-071` failing closed), but it would leave the one measurement that
   could *shrink* the LLM's unstable surface untaken.

   **P2 is not affected.** Its engine phase requires only that the labels be timestamped, and its verdicts do not yet
   exist, so labelling now is blind on the original terms.
2. **A lint or engine result is reproducible from committed inputs or it is void.** P5's replay-identity rule applied early: every P6 verdict and every P2 freshness verdict is a function of `(catalog digest, facets digest, item digest)` recorded on the result line.

---

## 1. P1 — classification stability and facet derivability

### 1.1 Why
The selection engine is deterministic only if its input is. F27 showed a model's threshold judgement flips on the same input; the context vector (catalog §2.4) has five ordinal axes that are exactly such judgements. PC7/PC8 moved some facets to "derived"; that claim is untested.

### 1.2 Design
- **Corpus:** 30 items from the frozen `.bklg`, stratified: 8 bug, 6 story, 6 project, 4 initiative, 3 incident-shaped, 3 spike. Selection by hash order of slug, listed in `spikes/plans/p1-items.txt` before any dispatch.
- **Arm L (LLM-emitted facets):** one classification dispatch per item × 5 repeats × 2 models (haiku 4.5, sonnet 5), default temperature, `--json-schema` = the closed classification record (kind; `criticality, novelty, size, blast_radius, dynamism` ∈ {low, med, high, unknown} with written anchors; booleans `market_facing, external_dep, has_repro`; `evidence[]`; free `notes`). The card is committed before the first dispatch and not edited during P1.
- **Arm D (derived facets):** `toolchains`, `change_kinds`, `binary_change`, `auth`, `data_migration`, `ui`, `flag_exists` computed by a script from each item's affected paths and diff against the corpus repo. No model.
- **Human panel:** two human labels per item per facet (Ryan + one other, or Ryan twice ≥24 h apart, disclosed), produced before unblinding (rule 1).

### 1.3 Metrics
- Arm L: per-facet Krippendorff's α across the 10 draws per item; per item, the fraction of draws whose *derived required-artifact set* `R(stage, ctx)` under the v0 `bug-standard`/`story` packs is identical to the modal one. Reported per item with Wilson intervals, then summarised.
- Arm D: per-facet precision/recall against the human panel.
- Both: `unknown` rate per axis (an `unknown` routes to a human; a high rate is a cost, not an error).

### 1.4 Kill / decision criteria
- Any axis that fires a `require` row with **α < 0.6** becomes human-supplied at the triage gate (its anchors are rewritten first and re-run once; if still < 0.6, human-supplied).
- **< 80 % of items** with an identical modal `R` across draws → the rubric rows over that axis are rewritten to coarser bands before Phase 1 authors packs.
- A derived facet with **precision < 0.95** stays LLM-emitted (it does not leave the model); ≥ 0.95 removes it from the classification record.
- Model disagreement (haiku vs sonnet modal value differs on > 20 % of items for an axis) is reported, not a kill; the cheaper model that meets α is the triage default.

### 1.5 Limits
Thirty items from one corpus. Anchors written by the schema's author. Human panel of one or two.

---

## 2. P2 — short-circuit precision/recall and advisory-drift noise

### 2.1 Why
Catalog §2.5 step 3 marks artifacts `Present / Stale / Missing` from `derived_from` digests; PC3 adds advisory drift for human-supplied artifacts. Too coarse re-runs everything; too fine accepts stale work. Neither has been measured.

### 2.2 Design
- **Corpus:** 20 completed items from the frozen `.bklg` with ≥ 3 artifacts each (spec, plan, report, review…). Reconstruct each artifact's `derived_from` from the git history (which upstream file changed before it was last written); where ambiguous, record `unknown` and exclude.
- **Perturbation:** for each item, one upstream artifact is replaced by its previous git revision (a real historical change, not a synthetic edit) — 20 cases; plus 10 cases where a human-authored artifact's upstream changes (advisory path).
- **Engine:** a script implementing §2.5 step 3 with coarse `derived_from` (= `requires`) and with the recorded fine scope.
- **Labels (rule 1):** for every downstream artifact in every case, a human label "should re-run / need not re-run", written from the diff alone before the engine runs.

### 2.3 Metrics
Precision and recall of `Stale` against "should re-run", coarse and fine; advisory count per case and the fraction a human judges useful (label after the fact, disclosed as such).

### 2.4 Kill / decision criteria
- Coarse `derived_from`: **recall < 0.9** or **precision < 0.7** → fine scope becomes mandatory for `L`-class activities and `stale_policy = gate` is the default for them.
- Fine scope cannot reach recall ≥ 0.9 either → `stale_policy = gate` for all `L`/`XL` activities; the memo key is trusted only at `S`/`M`.
- Advisory useful fraction **< 0.3** → PC3's advisory is demoted to a log line (no gate text).

### 2.5 Limits
`derived_from` is reconstructed, not recorded — the old redkiln never stored it. Twenty items.

---

## 3. P6 — catalog completeness over the context lattice

### 3.1 Why
Essence Level-3 completeness as CI (catalog §2.6): every stage-exit artifact must have an admissible producer for every reachable context, or the engine emits `Unsatisfiable` at runtime. PC5 commits to ~95 activities; PC7 adds toolchain combinations to the lattice.

### 3.2 Design
- **Precondition (disclosed):** `lint.py` v2 implementing §9a and §9b; the ~85 remaining v0 activities authored against the amended schema by whoever authors them (P9b says a third author will find more ambiguities — those are recorded, not suppressed). P6 does not start until the v0 catalog passes activity-level lint with zero errors.
- **Lattice:** kinds (10) × rigor (3 live levels as item overrides, plus each pack's own policy rigor as a fourth
  mode) × ordinal axes banded to {low, med, high} (4 registered axes → 81) × toolchain sets {node, unreal, node+unreal,
  dotnet} × boolean risk facets as a covering array (pairwise, not full product); plus a separate `unknown` sweep (one
  ordinal axis at `unknown`, the others at `low`, 4 rows) reported on its own line. Enumerated by script; size recorded.
  *Amendment 7 (2026-09-08):* original text banded to {low, high} over 5 axes. `dynamism` is not in `facets.toml`, so
  4 axes; `med` added because `canary-release` (keyed `>= med` between `< med` and `>= high` neighbours) was unreachable
  under two bands — a banding artefact, not a catalog gap (F35 §1.4). The {low, high} run is `results-p6-run3.txt` and
  stays the pre-registered reading; the three-band run will be `results-p6-run4.txt`.
- **Engine:** the §2.5 selection function over the v0 packs — one per kind in `facets.toml`
  (`bug-standard`, `bug-expedite`, `story`, `story-ue`, `incident`, `spike`, `project`, `initiative`, and
  `chore`, `tech-debt`, `security-vuln`, `docs`) — steps 1, 2, 4, 5 with all artifacts `Missing`.
  *Amendment 6 (2026-09-08):* the four packs after the semicolon were not named in the original; they were written
  before the first lattice run (file timestamps precede `results-p6-run0-engine-v1.txt`) but the list was amended after
  the run. Every P6 metric is reported for the eight named packs first and for all twelve second (F35).

### 3.3 Metrics
Unsatisfiable `(stage, ctx)` pairs, by artifact kind and by which predicate failed; dead activities (never selected in any context); rank ties; contexts whose Σ `cost_class` exceeds the pack's appetite.

### 3.4 Kill / decision criteria
- **> 5 % unsatisfiable pairs** → catalog gaps are filled before any dispatch; the pack is not tagged.
- Any unsatisfiable pair in the **mixed Node+UE** slice → PC7's per-item toolchain facts are wrong in shape (not just the catalog) — reopen PC7.
- Dead activities > 20 % of the catalog → PC5's "curated ~95" was too many; the dead ones move to `extended`.

---

## 4. P7 — encapsulated quality: cost vs catch (absorbs P4)

### 4.1 Why
PC6 makes every agent-producing activity own its quality with an internal `adversary → policy → (gate)` block. P9/P9b found that block in 6 of 10 and 2 of 3 activities. Whether it earns its cost at `recommended` — or only at `mandatory` — decides the `quality` shorthand's defaults and the pack's default rigor. F23/F26 give the single-dispatch baseline for review; F27 says the *verdict* is unstable, which is why the block ends in a policy check, not a model verdict.

### 4.2 Design
- **Graph:** `spikes/p9/catalog/root-cause.toml` (v2-lint clean) run through a thin driver over the E3 rig: `hypothesise` (investigator, Opus) → `confirm` (real test run on the fixture) → `adversary` (reviewer, Opus, when rigor ≥ recommended) → `policy` (Rust-equivalent check in the driver) → `signoff` (gate; auto-answered `accept` in P7 — gate behaviour is P8's question) → `emit`.
- **Items:** 10 E3 S-SILENT fixtures (seeded defects with known root causes; the answer key exists — F9's lesson) chosen by hash order, listed in `spikes/plans/p7-items.txt` before dispatch.
- **Arms:** rigor `optional` (no adversary), `recommended` (adversary + policy), `mandatory` (adversary + policy + loop-back on failure, ≤ 3). 3 repeats per item per arm = 90 graph runs; each `agent` step is one dispatch.
- **Deviation channel (P4):** the investigator's output schema includes an optional `RequestDeviation { target, kind, claimed_predicate, why }`. Every emitted request is labelled warranted/unwarranted by a human after the run (rule 1 does not apply — the label is of the request, which does not exist before the run — disclosed).

### 4.3 Metrics
Per item per arm: root cause correct (against the answer key) — rate with Wilson interval over 3 draws; total tokens and `total_cost_usd`; number of loop-backs; adversary findings by evidence class. Deviation: requests per item; warranted fraction; how many the engine would auto-accept (claimed predicate holds against evidence) vs gate.

### 4.4 Kill / decision criteria
- `recommended` catches **no more root causes than `optional`** on the pooled per-item rates (interval overlap on every item) → the adversary block's default is `mandatory`-only and the `quality` shorthand defaults to `{ adversary = none }`.
- `mandatory` costs **> 3 × `recommended`** with no additional catches → internal loop-back is reserved for `criticality = high` rows.
- Deviation **warranted fraction < 30 %** or **> 1 request per item** → `deviation_ok` is restricted to `skip`; **< 10 %** → the channel is removed from the schema.
- If `optional` already reaches the answer key on ≥ 9 of 10 items, the fixture set is too easy for the question; report and re-run on E3-H subtle fixtures before deciding.

### 4.5 Limits
One graph, one stage, ten seeded items. Auto-answered gate. Opus only. Cost ≈ $150 at F31's per-dispatch rate — recorded, and the run halts at $200.

---

## 5. P8 — gate inside a graph

### 5.1 Why
Schema §3 resumes *into the same step* with `${gate.answer}` bound; §9b.15 makes `answer` the gate's only output; P9b A8 assumes a re-reached gate is a new mark. F19 proved lossless resume at activity granularity; F28 priced a stage-level gate at a flat ~18.6k. None of that was measured with partial step outputs bound mid-graph.

### 5.2 Design
- **Graph:** `minimal-repro.toml`: `attempt` (haiku, loop ≤ 3) → `triage-gate` (when not reproduced) → `confirm` → `reduce` → `emit`. Fixture: 3 E3 items with the repro steps deliberately removed so the gate fires.
- **Procedure per run:** drive to the gate mark; kill the process (SIGTERM, F21 semantics); wait 0 / 65 min / 24 h (F18/F28's anchors); answer via CLI; resume; compare every step output digest before the kill with the digest after resume; then loop back to `attempt` via the answer and let the gate fire a second time.
- **Repeats:** 3 items × 3 delays × 3 repeats = 27 runs, plus 3 control runs with no kill.

### 5.3 Metrics
Step-output identity across the kill (any digest changed = lost); resume cost in tokens vs the no-kill control and vs F28's ~18.6k; whether the second gate arrival produces a second `Gate:` mark and `HumanAnswer`; wall-clock from answer to first post-resume dispatch.

### 5.4 Kill / decision criteria
- **Any lost or changed step output** → escalation becomes activity-terminal only (`Blocked` with the partial outputs attached); §3's "resume into the same step" is withdrawn.
- Resume cost **> 1.5 × F28** → a mid-graph gate is priced like a stage gate plus the graph's replay; pack authors are told so and the `quality` shorthand defaults its gate to `mandatory` only.
- Second arrival does **not** mint a new mark → §9b's A8 decision is reversed and the gate step gains an explicit `rearm` flag.

### 5.5 Limits
Haiku; three items; the 24 h delay gives one draw per item per repeat and cannot be hurried.

---

## 6. Order, and what each can overturn

P6 needs the catalog and lint v2 (no model spend) and gates the pack. P1's arm D and P2 are offline and run in parallel with the S-series. P1's arm L, then P7, then P8 take the rig in that order once the S-series releases it — P7 before P8 because P7's result decides whether the gate P8 tests is on by default. **P7, P8 and P6 gate "design final"**; P1 and P2 set parameters. P5 remains Phase 1's acceptance test and is not in this document.

Amendments to this document are disclosed here and in their commit. Amendments 1–5 (`HANDOFF.md` §Pre-registration
status) were made before the data they affect. Amendments 6 and 7 (§3.2) were made after P6's first run and change
what is *reported*, not the kill criteria; the pre-registered readings remain primary in F35. **Amendment 8** (§0 rule 1, 2026-09-09) was made after the
arm-D engine output existed and, on the labeller's own attestation, before it had been seen; it substitutes an
attestation for an expired chronology check on P1 arm D alone, is strictly weaker than the check it replaces, and
changes no kill criterion. Every result scored under it is reported as **attested-blind**, never as
timestamp-verified.
