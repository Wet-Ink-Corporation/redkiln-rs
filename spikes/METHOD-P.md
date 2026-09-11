# Pre-registration #7 — P: the process-catalog spikes (P1, P2, P6, P7, P8)

**Status:** pre-registration, per charter §8.2 (amendments 6–7 applied 2026-09-08 by Claude on Ryan's authorization; post-data, additive — see §3.2 and the closing note. **Amendments 8 and 9 applied 2026-09-09**, also on Ryan's authorization. 8 changes how §0 rule 1 is *verified* for P1 arm D; 9 changes what arm D's precision is scored *against* — a deterministic oracle on the mechanical facets, an adversarial model panel on the semantic ones, Ryan adjudicating the contested set. Neither changes a kill criterion — see §0 rule 1. **Amendments 10–16 applied 2026-09-11** by Claude on Ryan's authorization, absorbing the 2026-09-10 pre-implementation audit — every one post-data for the spike it touches, each stating what it post-dates; none moves a threshold; two register the form of a re-run (P2-B, S-P7B) before any draw of it — see §6). **Written before any P1/P2/P6/P7/P8 dispatch or analysis. No P data has been seen.** P9 (`METHOD-P9.md`) and its second-author follow-up P9b are already run and reported (`F32-DRAFT.md`, `F33-DRAFT.md`); their results shaped this document and are cited where they do.
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

   *Amendment 9 (2026-09-09):* for **P1 arm D only**, the oracle §1.3 scores against changes from "the human panel"
   to **a deterministic oracle on the mechanical facets, an adversarial model panel on the semantic ones, and Ryan
   adjudicating every contested cell**. The rule that labels are produced blind is unchanged and now binds the panel:
   no panel member may read `spikes/results/p1-arm-d.jsonl`.

   **Why.** `derive-facets.py` is a regex over file paths and diff lines. That splits the seven scored facets in two.
   `toolchain`, `change` and `binary_change` are decided by git's own metadata — a human reading the same file list
   agrees with the regex by construction, so a human label there measures regex coverage, not semantics, and 60 of the
   140 cells were nearly tautological. `auth`, `data_migration`, `ui` and `flag_exists` ask whether a lexical match on
   a path or diff line approximates a semantic fact — `authorship.ts` matches an auth regex and is not auth; a session
   change may be auth and match nothing. Those four are the measurement.

   **What each oracle is.** Mechanical facets: an independently implemented derivation from git metadata, not sharing
   the script's regexes — `binary_change` in particular tests a real rule, because the script marks binary on git's
   `--numstat` marker **or** an Unreal-path match, so a text `.uproject` is a live disagreement. Semantic facets: N
   independent model labellers per cell, each returning a value, the file or line that decided it, and a confidence,
   plus one adversarial seat whose job is the opposite — find a real auth or UI change carrying no lexical marker,
   which is the failure a path regex cannot see. Unanimous panel agreeing with the script is accepted and logged with
   its evidence; every split, and every unanimous panel that contradicts the script, is escalated.

   **The human's role is unchanged in kind and smaller in volume.** Ryan adjudicates the contested set. To keep that
   judgement independent, the adjudication surface shows the cell, the diff evidence and the question, and reveals
   which side the script and the panel took only after he answers — the same blind-then-unblind discipline as the E3
   severity annotation (METHOD-E3 §7).

   **This is weaker than the pre-registered oracle, in one specific way.** A model panel reading a diff and a regex
   grepping the same diff can share a blind spot: a semantic change with no lexical marker anywhere may be missed by
   both, and would be scored as agreement. The adversarial seat is the mitigation and is not a cure. It is also
   weaker in kind — F27 and F36 both measured model judgement to be unstable — which is why unanimity is required
   before a cell is accepted without a human, and why every accepted cell carries the evidence that decided it so the
   claim is checkable after the fact.

   **It is stronger in two ways, and both are stated so the trade is visible.** The panel reads the diff *body* and
   surrounding code; the script reads paths and diff lines through a regex, so the panel is a richer reader rather
   than the same reader twice. And the human's scarce judgement is spent on the cells where something actually
   disagrees, with the evidence assembled, instead of on a majority that are obvious.

   **What it does not change.** §1.4's thresholds are untouched: precision ≥ 0.95 still removes a facet from the
   model, α < 0.6 and the modal-`R` line are unaffected, and `?` is still excluded from scoring rather than counted
   against the script. Results scored under this amendment are reported as **panel-adjudicated**, never as
   human-panel, wherever the number is cited.

   *Rejected alternatives.* Keep the pre-registered human oracle — stronger evidence and the honest default, rejected
   only because the labeller states his own labels would be best-effort rather than definitive, and 60 of the 140
   cells measure nothing. Let the panel be the final oracle — rejected: it inverts ADR-0074's "reviewer proposes,
   machine confirms", making a model the authority over whether a deterministic rule is correct. Score the mechanical
   facets against a human anyway — rejected as measuring the regex against itself.


   *Amendment 10 (2026-09-11):* two scoping notes on amendment 9, registered wording untouched (audit PIA-096;
   DECISIONS-2026-09-11 #13).

   **(a) Where the "panel-adjudicated" label is required.** Amendment 9's "wherever the number is cited" is scoped
   to **result tables and the first citation of an arm-D figure in each document**; a later restatement in the same
   document may cite the figure bare. The label's job is that no reader can take an arm-D number for a human-panel
   one, and a table plus a first citation does that; the audit found 18 restatements in the design and the
   registers without it, and labelling every one adds nothing a first citation did not.

   **(b) The adjudication surface, and what it does not prove.** The 32 contested cells were adjudicated on a
   **claude.ai Artifact page** built for the purpose, showing the cell, the diff evidence and the question and
   revealing the script's and the panel's positions only after the answer was entered. **The page is not in the
   repository**, and its answers were recovered from browser memory after a save failure
   (`spikes/plans/HANDOFF-2026-09-09-design-session.md` :79-82). The reveal-after-answer logic is therefore not
   inspectable from this tree, and the blindness of the 32 adjudications rests on amendment 8's weaker form: an
   attestation. The `blind_attestation:` line in `spikes/results/p1-arm-d-labels-ryan.md` (:27) is where it goes —
   who attests, what was not opened (`spikes/results/p1-arm-d.jsonl`, the panel files, and the page's reveal
   before each answer), and when — and it is **Ryan's to write**; it is `FILL-ME` as of this amendment. Until it is
   written, F39 §5's "Ryan, blind" is reported as **attested-blind, attestation pending**, never as verified.

   **When it was made:** after F39 was reported (2026-09-09) and after the audit read it (2026-09-10). It changes
   no kill criterion and no figure.

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


*Amendment 11 (2026-09-11):* the third criterion's registered wording ("precision ≥ 0.95 removes it from the
classification record") is untouched; this note records the **minimum n** it needs and reports it by name (audit
PIA-073, PIA-077; ERRATA E-0089-2; golden constraint 10).

**Why.** Under §0's rule that a rate is reported with its Wilson 95% interval, the bar is met when the interval's
lower bound exceeds 0.95, and that needs **at least 73 all-correct positives** at the registered two-sided z = 1.96
(73/73 → 0.9503); **92** with continuity correction; **72** under Clopper–Pearson [measured, recomputed 2026-09-11].
F39's largest positive count on the 20-item corpus is 29 (`change`), so the criterion is **powerless** there for
every facet: no method, however good, could have cleared it. That is the third instance of the same
pre-registration defect (P1's α rule, F36; P7's criterion 1, F37), and it is **not re-thresholded** — a lower bar
chosen after the data would be exactly the move §0 forbids. What discharges it is a corpus with ≥ 73 positives per
facet of interest, or a different criterion registered before its data (S-FACET's re-registration under
change-classification §6, DECISIONS-2026-09-11 #12, is that criterion for the `require`-row question).

**And the count F39 reports over.** F39's tables were computed over **136 of the 140** truth cells: the
panel-accepted file keys four cells of one item under a mis-spelt slug and the scorer skipped them silently. The
scorer (`spikes/p1/score-arm-d-panel.py`) now normalises the known mis-key and fails closed on a truth cell with no
engine record; the re-run of 2026-09-11 scores all 140 and moves one figure — `auth` 0/12 [0.00, 0.24] (was 0/11
[0.00, 0.26]) — and no verdict (F39 "Correction (2026-09-11)"). **When it was made:** after F39's data; changes no
threshold.

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


*Amendment 12 (2026-09-11):* **P2-B is registered here, before any draw of it** (audit PIA-084; design §10.5's
P2-B row; F40 Limits 1). §2.4's registered criteria are untouched; F40 could not decide two of the three because 0
of 41 rows were "should re-run" (11 of 12 perturbations changed only YAML frontmatter), so the recall half of
criterion 1 and criterion 2 are **not answerable** on the frozen corpus — reported by that name, not as passed or
failed.

**Criterion form.** The same §2.4 criteria, evaluated on a label set that has a positive class: recall of `Stale`
against blind "should re-run" labels, coarse and fine, each with its Wilson 95% interval; a criterion is decided
when its interval excludes the threshold, and reported **powerless** when the interval straddles it.

**Minimum rows.** For recall ≥ 0.9 to be *demonstrated* (lower bound ≥ 0.90) the label set needs **at least 35
blind-labelled "should re-run" rows with every one recalled** (35/35 → 0.901); **53** if one is missed, **69** if
two [measured, recomputed 2026-09-11; the audit's "≈ 36" is the same sizing rounded]. For recall < 0.9 to *fire*
(upper bound < 0.90) the set needs roughly the same order of positives with a miss rate well above 10%. Fewer than
35 positives and the criterion is registered as powerless before the run and is not run.

**Construction.** Positives are **real content perturbations**: an upstream artifact replaced by an earlier git
revision whose diff touches the **body** (not only `stage` / `status` / `updated` / `gate_open`), classified
mechanically against the artifact's frontmatter boundary as F40 §1 does, **before** labelling. The frozen corpus at
`47faec9f` supplied one such case (P01) in twenty items, so P2-B needs a second corpus or a later pin of the same
project carrying real revisions; the case list is committed before any label is written. Labels follow §0 rule 1
on P2's original terms — written from the diff alone, `labelled_at` in UTC preceding the engine's earliest record,
**timestamp-verified, never attested** — and the engine is `reconstruct-derived-from.py --phase engine` scored by
`score-p2.py`, unchanged.

**Cost, once sized.** Model cost **$0** (the engine and scorer are offline, as F40's were). The cost is the
labeller's: P2's 41 rows were labelled in one sitting on 2026-09-10; at ≥ 35 positives plus their negatives the
set is ≥ 70 rows, so about two sittings, plus whatever it takes to identify or produce a corpus with ≥ 35
body-changing revisions — which is the unpriced part and is stated as such. **When it was made:** after F40
(2026-09-10) and before any P2-B case list, label or engine record exists.

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


*Amendment 13 (2026-09-11):* §4.4(1)'s registered wording stands; this note records its power and registers the
form of its re-run, **before any S-P7B draw** (audit PIA-073, PIA-080; ADR-0111, ADR-0106).

**Why.** At n = 3 per cell no two Wilson 95% intervals can be disjoint — 3/3 → [0.439, 1.000] against 0/3 →
[0.000, 0.561] — so "interval overlap on every item" held **by construction** and criterion 1 **could not fail to
fire** [measured, recomputed 2026-09-11]. It is reported as **fires while powerless** (golden constraint 10;
DECISIONS-2026-09-11 #10), never as a verdict on the block. F37 §2's remedy ("n = 10 per cell") is also
insufficient for the effect it names: at n = 10 the first cell that separates from 0/10 is 6/10, and 0 vs 0.33
needs ≈ n = 20 per cell under interval overlap.

**S-P7B's criterion, registered.** The re-run does not inherit §4.4(1). It uses **either** a paired per-item
comparison — each item run under both arms in the same batch, the paired difference in root-cause-correct tested
(sign or McNemar over items, draws per item ≥ 3) — **or** an interval on the pooled difference of proportions with
a stated minimum detectable effect and the n that reaches it (≈ 20 per cell for 0 vs 0.33). The choice, the MDE
and the n are written into a further dated amendment before the first S-P7B dispatch, with the severity vocabulary
ADR-0080 decision 3 waits on settled first. Running S-P7B under §4.4(1) at any n is forbidden (ADR-0111).

**The `mandatory` ratio.** §4.4(2)'s figure is n = 1 and stays NOT DECIDABLE. The two published ratios are two
denominators, not a contradiction: **3.66×** = $9.857 / $2.693 (the `recommended` arm mean, n = 30, F37 §6) and
**4.4×** = $9.857 / $2.222 (the `recommended` r1 draw of the same item, `issue-77`; DECISIONS-2026-09-09 #33).
Neither is quoted again without its denominator (ADR-0106 clause 3); R-P7RATIO is closed on that basis.

**When it was made:** after P7's 297 records (2026-09-08) and after F37; before any S-P7B record exists. It moves
no threshold in §4.4.

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


*Amendment 14 (2026-09-11):* the second criterion's registered wording stands; this note fixes the **token type**
it is evaluated on for P8-B and records what F38 measured against it (audit PIA-080; ADR-0106 decision 2; SM-073).

**Why.** F38 §2 divided the resumed dispatch's `output_tokens` (8,408 / 10,314 / 14,579) by F28's
`cache_creation_input_tokens` (~18.6k) — unlike counters. "Resume cost" in §5.3 and the "1.5 × F28" in §5.4 are
read, for every remaining P8 arm (P8-B: the 65-minute and 24-hour delays and the three no-kill controls), as
**`cache_creation_input_tokens` on the first post-resume dispatch, summed across that dispatch's turns**, reported
beside **`total_cost_usd`** — the same counter F28's 18,603 is. On that counter the three existing runs read
**84,960 / 49,468 / 46,961 = 4.57× / 2.66× / 2.52×** of 18,603 [measured, `spikes/results/p8.jsonl`], so the
> 1.5 × criterion **fires on all three**, subject to the cross-fixture caveat (F28's gate and F38's resume are
different fixtures) and to n = 3 with overlapping intervals; what that licenses is only that **the cost ordering
between a mid-graph gate and a stage gate is UNMEASURED** (ADR-0106 decision 1), not that a mid-graph gate is
dearer or cheaper. The no-kill controls §5.2 registers are the within-experiment comparator P8-B must run; F38's
figures are compared only to F28 until then. **When it was made:** after the 3 P8 records of 2026-09-08 and F38;
before any 65-minute, 24-hour or control record exists. The 1.5 × threshold is unchanged.

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
timestamp-verified. **Amendment 9** (§0 rule 1, 2026-09-09) replaces arm D's human-panel oracle with a deterministic oracle on the mechanical facets and an adversarially-verified model panel on the semantic ones, with Ryan adjudicating every contested cell. It is weaker than the pre-registered oracle — panel and regex can share a lexical blind spot — and richer in what the panel can read. Results are reported as **panel-adjudicated**, never as human-panel. §1.4's thresholds are untouched.

*Amendment 15 (2026-09-11):* the gate verdicts this section carries, made explicit because the design's register
had drifted from them (audit PIA-094). **P2 does not gate "design final"**: the sentence above — "P1 and P2 set
parameters" — is the reading this document holds, and the design's §0.1 verdict ("No — gates the `stale_policy`
setting, not the phase; the step-3 mechanism ships unit-tested") is the same reading; a §10.4/§10.5 row marking P2
YES was the drift. **P7's "design final" gate is carried by S-P7B and remains a gate on one label only** — the
quality shorthand's default (ADR-0080 decision 4 ships the shorthand with no default; ADR-0111 registers the
re-run that could set it) — and is **released for everything else P7 could overturn**: PC6's block is retained
and its terminal predicate is forbidden (ADR-0080 decisions 1–2, from F37 §3, a result that does not depend on
sample size), and the catalog's rigor family is repaired by ceiling (ADR-0093 decision 5, P7-CEIL). The design's
§0.1 row 25 and §10.6 mirror this sentence. **When it was made:** after F37 and F40; it changes no criterion.

*Amendment 16 (2026-09-11):* provenance of this document against git, recorded because the audit found the
header's "written before any … dispatch" is only checkable with it (audit PIA-089). **The file's first commit is
`ab58733` (2026-09-07 21:20:08 −0700)**, and that commit already carries amendments 6 and 7 and post-dates the P6
lattice runs it amends — `results-p6-run0-engine-v1.txt` through `results-p6-run9.txt` were written 18:54–21:15
−0700 the same day [measured, `git log --diff-filter=A`; file mtimes]. The pre-amendment §3.2 text (two bands over
five axes; eight packs) is therefore recoverable only from amendments 6–7's own paraphrases, not from any commit.
**The P1, P2, P7 and P8 registrations do precede their data**: the earliest P1 arm-D record is 2026-09-08T04:54Z,
arm L 05:18Z, P7 05:30Z, P8 13:08Z, P2's engine 2026-09-10T16:01Z — all after `ab58733` (2026-09-08T04:20Z)
[measured, the result files' timestamps]. And the sentence above attributing "amendments 1–5" to this document's
lineage is corrected: those five are **METHOD.md's and METHOD-E3's** (METHOD-E3's four of 2026-09-02/03 and
METHOD.md's struck E0 gate), listed in `HANDOFF.md` §Pre-registration status; this document's own amendments begin
at 6. **When it was made:** after every P record named above.

