# Step 4 — THE BLOCKING GATE

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s4` | Phase 0 | Checkpoint 3c | Checkpoint 4 (ADR-0090 exists) | `s3` | `s4a` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


**Nothing in the kernel's selection path proceeds until this step's checkpoint passes.** That is ADR-0079 and
AC-071, not a review convention.

### 4.1 State the decision rule BEFORE you look at any number

Write this into the ADR draft first, so the decision is not made post-hoc. Three clauses, each already fixed by a
pre-registration or by AC-071; nothing here is invented at scoring time.

**This section now carries results as well as rules, and the ordering is deliberate.** Clause B's rule was fixed by
METHOD-P §1.4 before any arm-D number existed, was applied unchanged on 2026-09-09, and returned an answer. The
rule below is **the rule as pre-registered**; the measured outcome is recorded **beside** it, tagged and sourced,
**not substituted for it**. Nothing in §4.1 was re-worded to fit a number, and the one thing no command can check
(Checkpoint 4's last line) is exactly that.

| Clause | Source | The line | Where it is measured |
| --- | --- | --- | --- |
| **A. Modal-`R` identity** | METHOD-P §1.4 | ≥ **80%** of items show an identical modal required set `R(stage, ctx)` across the draws | `p1-arm-l-analysis.txt` / `p1-arm-l-v2-analysis.txt`. **Already measured: 5/20, 3/20, 2/20, 2/20 (0.25/0.15/0.10/0.10). Wilson 95% upper bounds 46.9 / 36.0 / 30.1 / 30.1 — every one sits far below the line.** |
| **B. Arm-D per-facet precision** | METHOD-P §1.4 / the scorer's own printed verdict | precision ≥ **0.95** → the facet is removed from the classification record; below → it stays LLM-emitted, and therefore blocked. **This rule was pre-registered, was applied on 2026-09-09, and returned the result in the block below. It is not rewritten here to fit its own answer.** | `score-arm-d.py`; **scored** by `spikes/p1/score-arm-d-panel.py` against 140 truth cells at pin `47faec9f` [measured, F39] |
| **C. The AC-071 five clauses, in full** | AC-071 | identity ≥ 0.80 **AND** n ≥ 20 items **AND** ≥ 10 draws/item **AND** arm-D panel `scored` **AND** `corpus_sha` equals the catalog corpus pin | the `redkiln lint --facet-stability` check, Phase 1 |

**The decision the rule produces, stated in advance so it is not chosen after the fact:**

- **If clause A fails** — which, on the records that already exist, it does, 4 times out of 4 — then **every
  ordinal axis is `source = "human"` for Phase 1**, supplied or confirmed at the triage gate. That is not a
  workaround; it is the hedge design §10.3 already ships, and it is what SM-024 measures the operator cost of.
- **And the three LLM booleans go the same way (pre-registered 2026-09-11, DECISIONS-2026-09-11 #2; PIA-003).**
  `spikes/p9/facets.toml` carries **seven** `source = "llm"` facets, not five: the four ordinals above, plus
  `has_repro`, `external_dep` and `market_facing`, which are booleans. Nineteen catalog and pack rows key on the
  three, and 7b's rigor-monotonicity paragraph presumes `has_repro` is a live selection input. Under clause A's
  failure they are **`source = "human"`, supplied or confirmed at the triage gate** — ADR-0079's "honest
  destination" option — so that after Step 4a **no facet is `source = "llm"`** and AC-071 has nothing to fail
  closed on. ADR-0090 carries this clause; Checkpoint 4a asserts the zero count.
- **Clause B was applied on 2026-09-09 and NO FACET CLEARS THE BAR.** The rule above was written before the
  numbers were read, was run unchanged, and returned this. Union source, 20 items, 140 truth cells (the 2026-09-09 run scored 136 of them; the 2026-09-11 panel-file key fix scores all 140 and moves no verdict — F39-DRAFT's Correction), Wilson 95%
  intervals [measured, F39 §1]:

  | facet | precision | 95% CI | recall | tp | fp | fn | tn | excl | verdict |
  | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
  | `ui` | **1.00** | [0.65, 1.00] | 0.78 | 7 | 0 | 2 | 6 | 4 | point clears, **interval does not** |
  | `toolchain` | 0.73 | [0.48, 0.89] | 0.79 | 11 | 4 | 3 | — | 0 | stays |
  | `change` | 0.72 | [0.54, 0.85] | 0.84 | 21 | 8 | 4 | — | 0 | stays |
  | `binary_change` | 0.60 | [0.23, 0.88] | 1.00 | 3 | 2 | 0 | 15 | 0 | stays |
  | `flag_exists` | 0.00 | [0.00, 0.66] | n/a | 0 | 2 | 0 | 13 | 4 | stays |
  | **`auth`** | **0.00** | **[0.00, 0.26]** | n/a | **0** | **11** | 0 | 4 | 4 | stays |
  | `data_migration` | — | — | — | 0 | 0 | 0 | 15 | 4 | **not measurable** |

  **The sub-0.95 clause fires on five facets. The ≥ 0.95 clause fires on none, and cannot.** No facet has enough
  positives to clear 0.95 with confidence *even in principle at this n*: `ui` is 7/7 and its lower bound is 0.65,
  and **73** all-correct positives are needed for a lower bound above 0.95 at the registered two-sided
  z = 1.96 (92 with continuity correction; 72 under Clopper–Pearson) [ERRATA E-0089-2, which withdrew the
  "roughly 60" this line carried until 2026-09-12], so **METHOD-P §1.4's threshold is not
  reachable at n=20 for any facet however good the method** — the same class of defect as P1's α rule (F36) and
  P7's criterion 1 (F37), and the reason golden constraint 10 exists [measured, F39 Limits 1]. `data_migration`
  emits **0 positives on 20 items across all three sources**: precision is undefined, a perfect method scores
  identically, and **the criterion has no denominator** — it is reported as *not measurable*, never as a pass or a
  failure [measured, F39 §4].

- **`auth` is not a threshold miss, and this is the load-bearing result.** Eleven positives, eleven false, upper
  bound **0.26**: the rule carries **no signal** on this corpus. It is an unanchored case-insensitive regex over
  paths **and diff lines** and it marks `auth = true` on **14 of 20 items**. Which alternative fired, and the word
  it fired *inside* [measured, F39 §2]: `token` 44 hits — **36** inside `brand-token-substrate` /
  `brand-tokens.test.ts` / `redkiln.tokens.json` / `tokens_out`; `auth` 43 hits — **40** inside `authored` /
  `report-md-unauthored` / `author` / `authors` / `authoring` / `only-the-author-can-advance`; `oauth` 8 hits — **8**
  inside `coauthor` (the string `oauth` sits inside `c-oauth-or`); `session` 8 hits — **6** are a *work* session;
  `role` 2 hits — **2** are an ARIA `role` attribute. **The failure is correlated, not random**: the colliding
  vocabulary is the vocabulary of the instrument under test. Adding word boundaries removes the `coauthor` and
  ARIA hits and leaves the *authored* and design-`token` hits untouched.

- **Nothing leaves the model. Do not write, and do not read into this table, any sentence saying a facet does.**
  The previous revision of this bullet promised that a derived facet clearing 0.95 "leaves the model on its own
  warrant"; **that promotion cannot happen at this n for any facet**, and a document that still offers it is
  offering a route the measurement closed. Step 4a is what the result actually forces, and ADR-0089 is where it is
  already recorded.

- **Clause B does not decide ADR-0079, and Step 4.4's supersession does not rest on it.** The previous revision
  assumed arm D's scoring would decide the context-vector question. It does not: **F39 measures a *derived* vector;
  ADR-0079 asks about an *LLM-emitted* one.** F39 narrows 0079's options without deciding among them. **Clause A —
  F36's modal-`R`, failing 4 times out of 4 — is what decides it, and clause A is unchanged.** So Step 4's decision
  is still due, on exactly the warrant it always had.

- **Clause C is the only thing that unblocks LLM-sourced ordinals**, and it needs a full P1 re-run at n ≥ 20 ×
  ≥ 10 draws in addition to the panel. That re-run is **not** part of Phase 1's critical path. Commission it or
  don't; Phase 1 proceeds either way, on human-sourced axes. **F39 discharges the fourth clause of AC-071 —
  "arm-D panel `scored`" — and only that one.** The block stands on the first: identity 0.25 / 0.15 / 0.10 / 0.10
  against the 0.80 line [measured, F36].
- **A temperature spike is explicitly not the answer.** The instability is schema-induced, corroborated across five
  2026 frontier models with Claude-Sonnet-4 measured at background temperature 0.

- **P2 IS NOT A FOURTH CLAUSE OF THIS GATE, and its result does not belong in the clause structure above. Say
  this plainly rather than leaving a reader to infer it from where the numbers sit on the page.** Clauses A, B and
  C are all about the *context vector* — what may key a `require` row over an LLM-emitted or derived facet
  (ADR-0079, AC-071). **P2 measures the `derived_from` reconstruction and gates `stale_policy`**: METHOD-P §2.4's
  own remedies are *"fine scope mandatory for `L`-class"* and *"`stale_policy = gate` for all `L`/`XL`"*, and
  **neither touches the selection path**. So P2 **sits outside the blocking gate**. It is run at §4.2 and checked
  at Checkpoint 4 because that is where the scorers live, not because it gates anything Step 4 gates, and it
  **neither discharges nor tightens** the block on LLM-emitted facets. Equally: **nothing in P2's result settles
  `stale_policy`** — criterion 1's precision half fires, its prescribed remedy is contraindicated by the same run,
  and criterion 2 is not answerable, so the design consequence is **OPEN and needs a decision, not a default**
  [measured, F40 §4]. Whoever takes it takes it in its own record; it is not this step's to take and it is not
  ADR-0090's subject.

**Source asymmetry is a SEPARATE, UNTAKEN decision. Do not decide it here.** `commits` beats `docs` on every facet
and `union` is *worse than `commits` alone* on two of them, because unioning a precise source with an imprecise one
imports the imprecision [measured, F39 §3]:

| facet | `docs` precision | `commits` precision | `union` |
| --- | --- | --- | --- |
| `toolchain` | 0.50 [0.22, 0.78] | **1.00** [0.70, 1.00] | 0.73 |
| `change` | 0.50 [0.24, 0.76] | **0.80** [0.61, 0.91] | 0.72 |
| `binary_change` | no positives (recall 0.00, 3 missed) | **0.60**, recall 1.00 | 0.60 |
| `ui` | 1.00, recall 0.22 | 1.00, recall 0.67 | 1.00, recall **0.78** |

Which source the engine trusts is registered as an arm of **S-FACET** (ADR-0012, ADR-0089), whose named consumer is
`facets.toml`'s `source`/`deriver` fields. **ADR-0089 explicitly does not decide it and neither does this step.**

**Carry F39's own limits; they do not travel separately from its numbers.** (1) The 0.95 bar is unreachable at
n=20 — above. (2) **39 of the 140 cells rest on unanimous 3-seat model agreement with no human**; a 9-judge panel
was measured to supply ~2.18 effective independent votes with accuracy 8–22 points short of independent-vote
predictions [documented, preprint, arXiv:2605.29800], and small-panel unanimity fails in the correlated direction.
(3) **The oracle is independent on `toolchain` and on `binary_change`'s Unreal shortcut, and not on the rest** —
for `change`, git's marker *is* the ground truth for the binary component, so oracle and script agree there by
construction. (4) **One repo, one operator, one pin**; every number is over `47faec9f`. (5) **A scorer defect was
found and fixed before this was reported**: the oracle emits `binary_change` as a JSON bool and the first scorer
compared it to the string `"true"`, scoring the facet 0.00 when it is 0.60. (6) **`unknown` is excluded, never
counted against the script** — 16 semantic cells are excluded on that basis, and reporting them as agreement would
have raised every semantic precision figure.

**How the truth set was built, because "panel-adjudicated" is not "human-panel" and the difference is the result.**
140 cells: **60** deterministic oracle (`spikes/p1/oracle-mechanical.py` — git metadata and a canonical
extension/manifest table, independently implemented); **39** unanimous panel; **9** unanimous panel where the
adversarial flag was mechanically moot; **32 human adjudication** — Ryan, blind, with the panel's and the script's
positions revealed only after he answered. His verdicts: **8 true, 24 false, 0 unknown, and 15 of 32 differ from
the script**. The panel labelled 80 semantic cells: 39 accepted unanimous (23 decided, **16 honest `unknown`**), 41
flagged [measured, F39 §5]. **Report this as PANEL-ADJUDICATED, never as human-panel.**

**Absence of evidence, which is what Step 4a exists for.** **4 of the 20 items changed no code whatsoever** —
every changed file is `.bklg/` bookkeeping or telemetry. The panel returned `unknown` on all **16** of their
semantic-facet cells; **the script emitted a confident boolean on every one.** That is the `excl = 4` column
[measured, F39 §4].

### 4.2 Run the scorers

**Precondition — confirm the three method decisions first.** HANDOFF's "What Ryan does next" opens with this, and
the previous revision of this runbook skipped it entirely. `spikes/DECISIONS-2026-09-09.md` leads with a table
titled *"Decisions Ryan should reverse or confirm first"*, and three of its rows change what the scorers below
**measure**:

| # | What it changes | Why it must be settled before you score |
| --- | --- | --- |
| **15** | `human_authored` remapped from a commit-authorship heuristic to the `artifacts.toml` registry | **STILL UNSETTLED, and now correctly scoped: it gates the ADVISORY HALF ONLY.** It **changes what METHOD-P §2.3's advisory fraction measures** — the old heuristic yields 0 advisory cases, the new one yields 6 — so **criterion 3 cannot be run until it is settled**, and criterion 3 is UNRUN [measured, F40]. It does **not** gate the perturbation half, which was **scored on 2026-09-10 without it** and whose figures do not move when #15 is decided. The file's own note still says *"confirm the mapping before scoring"*; that instruction now binds **A01–A06**, not P01–P12 |
| **18** | `_intake-brief.md` and `discover.md` added to both P2 scripts' artifact lists; `_ledger.md` deliberately not | Reconstruct records go 72 → 77 and the advisory candidates become reachable at all |
| **27** | METHOD-P §1.4's anchor-rewrite re-run was taken as arm `L2` | If you would rather the axes went straight to human-supplied, **v1 stands alone as the pre-registered reading and `L2` can be discarded without touching it** — which is a different clause-A record set |

Confirm or reverse each in writing before the scorers run. A confirmation after the numbers are read is the same
defect §4.1 exists to prevent, one level down.

**#15 is still unsettled, and P2's perturbation half was scored on 2026-09-10 regardless. That is correct, not a
breach, and the reason is the scoping in the row above:** #15 changes the *advisory* fraction's denominator and
nothing the perturbation cases measure, so it is a precondition for **criterion 3** and for the A01–A06 labelling,
not for P01–P12 [measured, F40]. **It remains a live Step-4 precondition** — do not read "the P2 engine has run" as
"#15 is discharged", and do not label the advisory half until it is written down either way.

```bash
cd D:/repos/redkiln-rs
# Stay in redkiln-rs. Step 2 runs AFTER this step (see its ORDER banner), so spikes/results/ is still here
# and spikes/p1/, spikes/p2/ never move — only corpus/, results/ and runner/ go to redkiln-corpus.
# If you have taken Step 2 out of order, the scripts are still at spikes/p1/ and spikes/p2/ in THIS repo and
# only the --engine/--labels/--cases/--out paths move to D:/repos/redkiln-corpus/results/.

python spikes/p1/score-arm-d.py \
    --engine spikes/results/p1-arm-d.jsonl \
    --labels spikes/results/p1-arm-d-labels-ryan.md
# prints, per source (docs | commits | union) per facet:
#   precision=… recall=… tp=… fp=… fn=… unlabelled=…   LEAVES THE MODEL (>=0.95) | stays LLM-emitted
#
# THAT LITERAL "LEAVES THE MODEL (>=0.95)" IS A STRING IN THE SCRIPT, NOT AN OUTCOME. It was measured on
# 2026-09-09 and NO FACET PRINTS IT: the >=0.95 branch is unreachable at n=20 for any facet (§4.1 clause B,
# F39 Limits 1). If you see it, read the label file and the source column before you believe it.
#
# The panel-adjudicated scoring is a SECOND script over the same engine records. It takes NO ARGUMENTS —
# its five input paths are constants inside the file, resolved from the repo root, so the `cd` above is
# load-bearing here in a way it is not for score-arm-d.py:
python spikes/p1/score-arm-d-panel.py       # 140 truth cells at pin 47faec9f, all scored after the 2026-09-11 key fix (the 2026-09-09 run scored 136), $0 model cost, F39's corrected figures
# reads spikes/results/p1-arm-d.jsonl + -oracle.jsonl + -panel-accepted.jsonl + -contested.json
#     + -adjudications.json. If any is missing this is a HALT (halt condition 8), not a skip.

# THIS HAS BEEN RUN. 2026-09-10, at corpus pin 47faec9f, $0 model cost — it dispatches nothing.
# It wrote spikes/results/p2-engine.jsonl: 82 records = 41 (case, downstream) rows x 2 scopes
# (coarse, fine), every record carrying corpus_sha, at, labels_at, case, item, downstream, scope,
# verdict, upstream_changed, derived_from and digests. Earliest `at` is 2026-09-10T16:01:12Z,
# AFTER the labels' 15:58:08Z — which is the blindness check, not a coincidence [measured, F40].
# Re-run it only to reproduce; the labels are already stamped and must not be re-stamped.
python spikes/p2/reconstruct-derived-from.py \
    --redkiln D:/repos/redkiln \
    --sha 47faec9f46b8d983c2c75f0794164e68415ae706 \
    --items spikes/plans/p2-items.txt \
    --phase engine \
    --cases spikes/results/p2-cases.jsonl \
    --labels spikes/results/p2-labels-ryan.md \
    --out   spikes/results/p2-engine.jsonl
```

Pass `--cases` **explicitly**. The engine phase used to derive it from `--out` by string replacement, which
mangled any path containing `engine` and — if `--out` lacked the substring — re-read the file it was about to
truncate, silently producing an empty result (DECISIONS-2026-09-09 #17). The script now refuses when `--cases`
resolves to `--out`, but pass it anyway.

**`reconstruct-derived-from.py` emits verdicts and stops. The scorer is a SECOND script, and the previous revision
of this runbook did not know it existed.** METHOD-P **§2.3** registered a precision/recall scoring of `Stale`
against the human "should re-run" labels and **nothing in the repository computed it** until
`spikes/p2/score-p2.py` was written for F40 on 2026-09-10. It takes the two defaults below, so the `cd` is
load-bearing:

```bash
cd D:/repos/redkiln-rs
python spikes/p2/score-p2.py \
    --engine spikes/results/p2-engine.jsonl \
    --labels spikes/results/p2-labels-ryan.md
# Measured 2026-09-10 at corpus pin 47faec9f, exit 0, $0 model cost — it dispatches nothing, so a
# re-run is free and there is no instrument-version lineage question here (GC8 does not bite).
# What it prints, in order:
#   corpus pin 47faec9f… · labelled_at 2026-09-10T15:58:08Z · labeller Ryan Britton · 82 records, 41 labels
#   LABEL SET: 0 should re-run, 41 need not, over 41 rows
#   a specificity table (coarse 0.73 [0.58, 0.84]; fine 0.55 [0.38, 0.70]; precision 0.00 in both)
#   abstention (fine): 8/41 = 19.5% [0.10, 0.34] — reconstructed scope `unknown`, NEVER a miss
#   LIKE-FOR-LIKE on the 33 rows both scopes scored: coarse Stale 5, fine Stale 15, agree 23/33,
#     and it NAMES the 10 flips — all Present -> Stale, none the other way
#   the three METHOD-P §2.4 criteria, verbatim, with criterion 1 marked FIRES on its precision half
```

**What it refuses to print, which is why it exists in this form.** Three reporting rules are hard-coded and the
script will not bend them [measured, `spikes/p2/score-p2.py` at this pin]:

1. **Recall.** With no positive labels `tp + fn = 0`, so recall has **no denominator** and the script prints
   `NOT COMPUTABLE` in the table and `NOT ANSWERABLE — no positive labels, no denominator` under criterion 1. It
   **will not print `0.00`, `1.00` or `n/a`**, because each of those is a claim and none of them is true. Same
   shape as `data_migration` in F39 §4 — **the same shape, never the same numbers; do not mix them.**
2. **Abstention.** A reconstructed fine scope of `unknown` is an **abstention**, reported as its own rate with its
   own Wilson interval, and **never summed into false negatives** (DECISIONS #16; the rule F39 §6 states for
   `unknown`).
3. **Bare rates.** Every rate carries a Wilson 95% interval, and a rate over zero positives says so.

It also refuses outright, rather than scoring, on three input faults: `labels are not timestamped — refusing to
score` (a `FILL` still in `labelled_at`), `engine records span several corpus pins`, and `engine was run against
different labels: … vs …` — the last comparing each record's `labels_at` against the label file's stamp, so a
re-stamped label file cannot be silently scored against an older engine run.

**Read the result as specificity, not as precision, and do not lead with the precision figure.** With **0 of 41**
rows labelled "should re-run", every `Stale` is a false positive **by construction**, so `precision = 0.00` is a
property of the label set and not an independent reading of the engine. The quantity this corpus genuinely
measures is how often the engine leaves a correctly-unchanged artifact alone [measured, F40 §2]:

| scope | specificity | 95% CI | precision | recall | tp | fp | fn | tn | excl | scored |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **coarse** | **0.73** | [0.58, 0.84] | 0.00 [0.00, 0.26] | **not computable** | 0 | 11 | 0 | 30 | 0 | 41 of 41 |
| **fine** | **0.55** | [0.38, 0.70] | 0.00 [0.00, 0.20] | **not computable** | 0 | 15 | 0 | 18 | **8** | 33 of 41 |

**The fine scope is the worse of the two, and the aggregate hides it.** The two specificity intervals **overlap**,
so on the aggregate the difference is **not separated at this n** — say so, and then give the paired comparison,
which is what separates them. On the **33 rows both scopes actually scored**: coarse marks **5** `Stale`, fine
marks **15**, and **10 rows flip `Present` → `Stale` going coarse to fine while none flips the other way**
[measured, F40 §3].

| downstream | coarse | fine |
| --- | --- | --- |
| `spec.md` | **11 Stale** | 5 Stale, 6 excluded |
| `implementation-report.md` | 12 Present | **5 Stale**, 6 Present, 1 excluded |
| `report.md` | 12 Present | **5 Stale**, 6 Present, 1 excluded |
| `_review.md` | 6 Present | 6 Present |

**Why, and it is a mechanism this runbook has met before.** Coarse's declared map carries `spec.md ← <card>`, so a
card write invalidates the spec and nothing else — **11 false alarms from 11 emissions**. Fine *also* fires on
`implementation-report.md` and `report.md`, which **the declared map does not connect to the card at all**: the
reconstruction — *"which upstream changed before this artifact was last written"* (METHOD-P §2.2) — picks the card up as a
derivation edge because the card is rewritten so often that it is nearly always the most recent prior write.
**That is F39's failure class at the artifact level: an inference over a trace the instrument itself writes
into.** F39's `auth` regex collided with the vocabulary of a tool whose documents are *authored*; F40's
`derived_from` reconstruction collides with the writes of a tool that stamps `stage` and `updated` on every
advance. In both the refinement meant to help imports the noise — F39 §3 measured `union` **worse than `commits`
alone** on two facets, and this is the same shape one level up.

**And the corpus is why there are no positives to score against.** 11 of the 12 perturbation cases change nothing
but YAML frontmatter — `stage`, `status`, `updated`, `gate_open`, the keys the redkiln CLI rewrites on every stage
advance; the twelfth (**P01**, the only non-card perturbation, on `discover.md`) changes ****one body line— a single rewritten line, which the unified diff shows as a `-`/`+` pair** of
wording, *"The next stage (spec) has a clear starting point"* → *"The next stage has a clear starting point"*. The
labeller wrote *"only frontmatter properties (process/stage status), no new/updated facts of the system"* against
**36 of the 41 rows** (35 the phrase exactly, one a shorter variant of it; four rows carry no reason at all and one is P01's), before any of this was computed. The mechanism is a recorded decision:
**DECISIONS-2026-09-09 #19** doubled the case count from 6 to 12 by adding the **card** as a perturbation candidate
(*"11/20 cards have ≥2 revisions vs 6/20 for any downstream artifact"*), and cards are exactly what the CLI churns
— the 11 frontmatter-only cases **are** the 11 card (`story.md`) perturbations [measured, F40 §1]. **The
zero-positive label set is the corpus, not the labeller**, and it is mechanically checkable; a re-run on a corpus
carrying real content perturbations is what would evaluate detection at all.

**How the three criteria came out, and the sentence that must travel with the first one:**

| criterion (METHOD-P §2.4) | outcome |
| --- | --- |
| **1** — coarse: recall < 0.9 **or** precision < 0.7 → fine scope mandatory for `L`-class, `stale_policy = gate` | recall **NOT ANSWERABLE** (no denominator); precision **0.00** [0.00, 0.26] over 11 emitted positives — **the precision half FIRES; the recall half cannot be evaluated** |
| **2** — fine cannot reach recall ≥ 0.9 either → `stale_policy = gate` for all `L`/`XL` | **NOT ANSWERABLE** — same missing denominator; it **cannot fire either way** |
| **3** — advisory useful fraction < 0.3 → PC3's advisory demoted to a log line | **STILL UNRUN.** The advisory half (A01–A06) is labelled *after* the engine in its own file (METHOD-P §2.3), and **DECISIONS #15's `human_authored` remap must be settled first** |

**Criterion 1 fires and its own remedy is contraindicated. Record both halves or you have absorbed half the
finding** (golden constraint 11). The remedy METHOD-P §2.4 prescribes is *"fine scope becomes mandatory for `L`-class
activities"* — and **the fine scope is the worse of the two on this corpus**, by the paired comparison above. **So
nothing here settles `stale_policy`**: the design consequence is **OPEN**, it needs a decision taken in its own
record, and it must not be resolved by defaulting to the prescribed remedy. **Criteria 1 and 2 are the fourth
undecidable pre-registration in this programme**, after P1's α rule (F36), P7's criterion 1 (F37) and METHOD-P
§1.4's 0.95 bar (F39) — golden constraint 10, and a result *about* the pre-registration, **not** a licence to edit
METHOD-P.

Re-read the unlabelled half of arm L for clause A:

```bash
python spikes/p1/analyse-arm-l.py --records spikes/results/p1-arm-l.jsonl
python spikes/p1/analyse-arm-l.py --records spikes/results/p1-arm-l-v2.jsonl \
                                  --compare-to spikes/results/p1-arm-l.jsonl
```

**`[WILL EXIST]`** — there is **no arm-L panel scorer**. `analyse-arm-l.py` computes the unlabelled half only (its
arguments are `--records`, `--out`, `--compare-to`), and `score-arm-d.py`'s facet regex admits only the seven
derived facets. Scoring `p1-arm-l-labels-ryan.md` against the LLM-emitted ordinals needs a script that does not
exist:

```bash
# [WILL EXIST]
python spikes/p1/score-arm-l.py \
    --engine spikes/results/p1-arm-l.jsonl \
    --labels spikes/results/p1-arm-l-labels-ryan.md
```

Its absence does **not** block the decision. Clause A resolves from the modal-`R` figures already computed, and
F36's own conclusion is that modal-`R` "fires 4 times out of 4, needs no inter-rater statistic, and is what the
engine actually feels. Act on that one." Write the arm-L panel file, stamp it, and hold it for the re-run — it is
the ground truth that re-run will be scored against.

### 4.3 The refusal behaviour — and do not work around it

The scorers refuse on an unfilled or post-dated label file. The exact messages:

| Message | Cause |
| --- | --- |
| `<path>: labelled_at missing — labels must be timestamped before unblinding` | `FILL-ME` still present, or no `labelled_at:` line |
| `<path>: labelled_at <ts> is not before engine first output <first> — METHOD-P §0 rule 1` | the stamp is at or after the engine's earliest record |
| `--labels required for the engine phase (METHOD-P §0 rule 1)` | P2 engine phase run without `--labels` |
| `labels are not timestamped — do not run the engine` | P2 label file still carries `FILL` |
| `--cases resolves to --out; pass --cases explicitly (#17)` | the #17 guard firing |

**Two of those five are P2's, and on 2026-09-10 neither fired.** `--labels required for the engine phase` and
`labels are not timestamped — do not run the engine` both passed cleanly on the P2 engine run: `--labels` was
given, and `labelled_at: 2026-09-10T15:58:08Z` carried no `FILL` [measured, F40]. **A guard that does not fire is
evidence only that its precondition held** — it says the labels were stamped before the engine ran, which is
exactly the blindness claim P2 rests on, and it says nothing whatever about the verdicts. Record the non-firing;
do not read it as a pass on the result.

**Six more messages exist and this table did not carry them** — eleven refusals in total, five here and six there, because `spikes/p2/score-p2.py`
did not exist when it was written: `labels are not timestamped — refusing to score` (a `FILL` in `labelled_at`),
`no engine records` (the scorer-side guard for the truncated-file case Checkpoint 4 also covers),
`engine records span several corpus pins: {…}`, `engine was run against different labels: {…} vs {…}`,
`<stamp> is not a UTC ...Z stamp`, and **`BLINDNESS VIOLATED: labelled_at <t> does not precede the engine's
earliest record <t>`** — added 2026-09-10, because until then METHOD-P §0 rule 1's chronology proxy was
being checked by hand while the corpus said the scorer did it. **Count them off the script, not off this
sentence.** The
last is the one worth knowing — it compares every record's `labels_at` against the label file's own stamp, so a
re-stamped label file **cannot** be quietly scored against an engine run that predates the re-stamp. Same
instrument, one level further on. Do not disable it to get a number out.

**These refusals are the instrument.** Do not back-date `labelled_at`. Do not edit the `at` field in
`p1-arm-d.jsonl`. Do not re-run the engine to move `first` past your stamp. If a refusal fires because you labelled
after opening the output, the honest repair is to say so in the ADR and treat the panel as sighted — a sighted
panel is a weaker result, and a silently unblinded one is no result at all.

Note the guard is deliberately strict on ties: a whole-second label stamp compares *after* a microsecond engine
stamp in the same second, so it refuses. That is fail-closed, and it is correct.

### 4.4 Take the decision and record it as a new ADR

**The number is `0090`, and it is reserved for this step.** `0089` was taken on 2026-09-09 by the facet-provenance
amendment Step 4a executes; `0092`–`0114` were taken on 2026-09-11 by the audit remediation; `references/adr/README.md`
§Numbering now reads, verbatim: "`0074`–`0089` and `0092`–`0114` are allocated; `0090` and `0091` are reserved for
the runbook's Step 4.4 and Step 6.1 records; the next free number is **`0115`**." Write
`references/adr/0090-<slug>.md` carrying all eleven frontmatter keys in order, all six mandatory sections, and a
`**What this forbids**` block inside Consequences. **The record carries §4.1's booleans clause** — `has_repro`,
`external_dep` and `market_facing` are `source = "human"` — as a numbered decision beside the ordinal axes, and cites
DECISIONS-2026-09-11 #2 as the pre-registration. It also carries ERRATA E-0079-1's three preprint corrections
(PIA-020) in its Evidence section rather than repeating ADR-0079's overstatements.

**This record decides on clause A, not on clause B.** §4.1 measures a *derived* context vector; ADR-0079 asks about
an *LLM-emitted* one, so F39 narrows 0079's options without deciding among them [measured, F39 §6]. What decides it
is F36's modal-`R`, failing 4 times out of 4, which is unchanged. Write the record on that warrant and say in its
Evidence section that F39 discharges AC-071's fourth clause only.

The supersession is **full** — ADR-0079 decides nothing and forbids being amended, so a successor replaces it
whole. Both sides carry the link; a one-sided link is a defect, not a shorthand:

- `0090` frontmatter: `supersedes: ["0079"]`, `status: accepted`, `date: <the day you take it>`,
  `legacy_id: none`, `consumers: []`.
- `0079` gains `superseded_by: ["0090"]` and its `status` moves `proposed → superseded`. **This is the one edit
  permitted to 0079**, and it is permitted because 0079 is `proposed`, not `accepted` — the immutability rule binds
  accepted records. Change nothing else in its body. **Until this step runs, 0079 stays `proposed`**; ADR-0089 does
  not touch it and says so in terms.

The eleven keys, in order, verified against `references/adr/README.md` §Format: `id`, `title`, `status`, `date`,
`legacy_id`, `source`, `supersedes`, `superseded_by`, `related`, `evidence`, `consumers`. The six sections:
`## Context and problem`, `## Decision drivers`, `## Considered options`, `## Decision`, `## Consequences`,
`## Evidence and confidence` (`Decision` may precede `Considered options`; both orders are valid). `0079`'s
current frontmatter carries `status: proposed` and `superseded_by: []` in exactly the literal form the checkpoint
greps for — verified 2026-09-08 against `0004`, the existing `superseded_by: ["0074"]` precedent.

**These two edits are scripted edits, so they get anchor assertions (golden constraint 7).** Two silent
`str.replace` no-ops were already read as behaviour changes once in this programme:

```bash
cd D:/repos/redkiln-rs
A=$(ls references/adr/0079-*.md)
n=$(grep -c '^status: proposed$' "$A"); test "${n:-0}" -eq 1 || { echo "FAIL: ADR-0079 is not 'status: proposed' (got ${n:-<unreadable>}) — you are editing the wrong file or the wrong key"; exit 1; }   # → silent: the BEFORE anchor (golden constraint 7)
n=$(grep -c '^superseded_by: \[\]$' "$A"); test "${n:-0}" -eq 1 || { echo "FAIL: ADR-0079's superseded_by is not the empty list (got ${n:-<unreadable>}) — the BEFORE anchor is absent"; exit 1; }   # → silent: the BEFORE anchor
# …make the two edits…
n=$(grep -c '^status: superseded$' "$A"); test "${n:-0}" -eq 1 || { echo "FAIL: the status edit did not land — ADR-0079 is not 'status: superseded'"; exit 1; }   # → silent: AFTER
n=$(grep -c '^superseded_by: \["0090"\]$' "$A"); test "${n:-0}" -eq 1 || { echo "FAIL: the back-link edit did not land — ADR-0079 does not point at 0090"; exit 1; }   # → silent: AFTER
```

Then update `references/adr/README.md`, which needs **five** changes and each has its own anchor. **Every anchor
string changed on 2026-09-11 when ADR-0092–0114 landed**; two earlier revisions of this table each quoted anchors
that were stale within a day, so **copy them from the file, not from here, before you write** — the row below says
what each becomes, and the anchor column is a hint about where to look:

| # | Anchor now in README.md (as of 2026-09-11 — re-read it) | Becomes |
| --- | --- | --- |
| 1 | The Lifecycle sentence: `The set now holds **108 \`accepted\`, 3 \`superseded\`** (0004, 0041, 0088) **and 1 \`proposed\`** (0079, …)` | **109** `accepted`, **4** `superseded` (0004, 0041, 0088, **0079**) and **0** `proposed` |
| 2 | §Numbering: `` `0090` and `0091` are reserved for the runbook's Step 4.4 and Step 6.1 records; the next free number is **`0115`** `` | `0090` **written** (this step); `0091` reserved for Step 6.1; next free still `0115` |
| 3 | The **Supersessions** lead sentence: `Twenty-nine supersession links exist in this set, from twenty-five superseding records … Three links are full (0004→0074, 0041→0064, 0088→0101); twenty-six are partial` | **Thirty** links from **twenty-six** records; **four** full (add **0079→0090**); twenty-six partial. Add the `0079 → 0090` row to the table: **Full**, "0079 decided nothing and forbade amendment; 0090 decides the axis source on F36's clause A" |
| 4 | The **New records** table's `0079` row status `proposed` | `superseded (by 0090)`; add the `0090` row |
| 5 | §Open items: `` `0079` is the only `proposed` record `` … and `exactly four records are not at \`accepted\` — \`0041\`, \`0004\` and \`0088\`, the three targets of a full supersession, at \`superseded\`, and \`0079\` at **\`proposed\`**` | no `proposed` record remains; **exactly four** at `superseded` (0004, 0041, 0088, 0079); rewrite the 0079 paragraph to say what 0090 decided and that the block on LLM-sourced axes stands on AC-071's first clause |

**Anchor 3 is the one that moved most and is easiest to get wrong.** Twenty-six partial supersessions already sit
in that table and every predecessor of a partial **stays `accepted`** [documented, README §Supersessions]. Do not
"tidy" any of them while you are in the file.

Leave the accepted records that carry named open items (0077/S-GIX with ADR-0098's fallback, 0078/S-TTL1, 0080 as
narrowed by ADR-0111, 0089/S-FACET as narrowed by ADR-0093, 0101/S-SUBTLE) exactly as they are; this step settles one
row, not the section. The README's file count does **not** move (0090 fills a reserved number: 112 records → 113,
`ls references/adr/*.md | wc -l` → **115** with README.md and ERRATA.md).

**Checkpoint 4.** All thirteen — recounted on disk 2026-09-11; the block said "twelve" and carries thirteen. This is the gate; treat a failure as a stop, not a note. **Two of the thirteen are P2's, and
P2 does not gate anything this step gates** (§4.1's last bullet) — they are here because this is where the scorers
are run, and a green P2 pair says nothing about the selection path.

```bash
cd D:/repos/redkiln-rs
out=$(python spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl \
       --labels spikes/results/p1-arm-d-labels-ryan.md) ; rc=$? ; printf '%s\n' "$out" ; echo "exit=$rc"
test "$rc" -eq 0 || { echo "FAIL: score-arm-d.py exited $rc (its refusal is on stderr, above)"; exit 1; }
printf '%s\n' "$out" | grep -q 'precision=' || { echo "FAIL: no per-facet row printed — exit 0 is not evidence it scored anything"; exit 1; }   # → exit=0, per-facet table printed
python spikes/p1/score-arm-d-panel.py ; rc=$? ; echo "exit=$rc" ; test "$rc" -eq 0 || { echo "FAIL: score-arm-d-panel.py exited $rc"; exit 1; }   # → exit=0, F39's figures reproduce
n=$(wc -l < spikes/results/p2-engine.jsonl) ; test "${n:-0}" -eq 82 || { echo "FAIL: p2-engine.jsonl has ${n:-0} lines, expected 82"; exit 1; }
                                                                   # → silent; 82 (41 rows x 2 scopes), NOT `test -s`:
                                                                   #   a truncated file is non-empty too
python spikes/p2/score-p2.py ; rc=$? ; echo "exit=$rc" ; test "$rc" -eq 0 || { echo "FAIL: score-p2.py exited $rc"; exit 1; }
                                                                   # → exit=0; "0 should re-run, 41 need not",
                                                                   #   coarse specificity 0.73, fine 0.55,
                                                                   #   recall NOT COMPUTABLE, criterion 1 FIRES
test "$(ls references/adr/0090-*.md 2>/dev/null | wc -l)" = 1 || { echo "FAIL: references/adr/0090-*.md is not exactly one file"; exit 1; }   # → silent; exactly one file
n=$(grep -c '^superseded_by: \["0090"\]$' references/adr/0079-*.md); test "${n:-0}" -eq 1 || { echo "FAIL: ADR-0079 carries ${n:-<unreadable>} back-links to 0090, expected 1"; exit 1; }   # → silent: back-link present
n=$(grep -c '^status: superseded$' references/adr/0079-*.md); test "${n:-0}" -eq 1 || { echo "FAIL: ADR-0079's status is not 'superseded' (got ${n:-<unreadable>})"; exit 1; }   # → silent: status moved
test "$(grep -c '0079. is the only .proposed. record' references/adr/README.md)" = 0 || { echo "FAIL: README still carries the stale '0079 is the only proposed record' claim"; exit 1; }   # → silent; the stale claim is GONE
test "$(grep -c '0090. and .0091. are reserved' references/adr/README.md)" = 0 || { echo "FAIL: the '0090 and 0091 are reserved' sentence is still in README (BOTH sites: Purpose and §Numbering)"; exit 1; }   # → silent; the PREVIOUS reservation sentence is gone
n=$(grep -c '0091. reserved for' references/adr/README.md); test "${n:-0}" -eq 1 || { echo "FAIL: README carries ${n:-<unreadable>} '0091 reserved for' sentences, expected exactly 1 (Step 6.1)"; exit 1; }   # → silent: 0091 alone stays reserved
test "$(grep -c 'source = "llm"' spikes/p9/facets.toml)" = 7 || { echo "FAIL: spikes/p9/facets.toml no longer carries exactly 7 source = \"llm\" facets — the BEFORE half of Checkpoint 4a's zero"; exit 1; }   # → silent; 7 BEFORE Step 4a (the registry is Step 4a's; this line is the BEFORE half of Checkpoint 4a's zero)
ls references/adr/0090-*.md >/dev/null 2>&1 || { echo "FAIL: references/adr/0090-*.md absent — grep -c would exit 2 and print nothing"; exit 1; }
n=$(grep -c 'has_repro\|external_dep\|market_facing' references/adr/0090-*.md); test "${n:-0}" -ge 1 || { echo "FAIL: ADR-0090 does not name the three booleans — the clause DECISIONS-2026-09-11 #2 pre-registered is missing (PIA-003)"; exit 1; }   # → silent: ≥ 1
git add references/adr/0090-*.md references/adr/0079-*.md references/adr/README.md \
  && git commit -m "ADR-0090: the context-vector source — supersedes 0079 in full; every LLM-sourced axis and boolean is human at triage (DECISIONS-2026-09-11 #2)" \
  || { echo "FAIL: git add/commit did not succeed"; exit 1; }
st=$(git status --porcelain) || { echo "FAIL: git status itself failed"; exit 1; }
if printf '%s\n' "$st" | grep -q '^ M'; then echo "FAIL: modified files remain after the commit"; printf '%s\n' "$st"; exit 1; fi   # → silent; the decision is in history BEFORE Step 4a and Step 5 (PIA-018)
```

The `next free number` pair replaced `grep -c 'proposed' references/adr/README.md`, which was **not a check at
all**: the word `proposed` appears in the lifecycle line, the status enum table and the format table, so the count
is nonzero whatever you do and the comment beside it ("the Open-items claim must be gone") was an intention wearing
a command's clothes. **The `0090`/`0091` pair is now given both ways, and that is the point**: the previous
revision asserted `→ 1` on `the next free number is 0090`, which was already true on this tree before Step 4.4 ran
a single edit — a checkpoint green when you have done nothing, the exact defect §3a names. Asserting `0091 → 1`
**and** `0090 → 0` cannot both be satisfied by the starting state — and the 2026-09-11 pair keeps that shape against the
README as it now stands (the reservation sentence is the BEFORE state, "0091 alone reserved" the AFTER). The commit
line exists because no earlier revision committed ADR-0090 anywhere, so Step 9's cherry-pick named a commit that did
not exist (PIA-018).

Plus one that no command can check and that you must not skip: **the decision rule in §4.1 was written down before
the numbers were read.** It was — clause B was pre-registered in METHOD-P §1.4, applied on 2026-09-09 and returned
"no facet clears the bar", and §4.1 records the result **beside** the unchanged rule rather than in place of it. If
that ever stops being true, the ADR says so.

### 4.5 What "the gate is discharged" means — and what it does not

Passing Checkpoint 4 does **not** unblock selection over LLM-emitted facets. On the records that already exist,
clause A fails 4 times out of 4, and the decision ADR-0090 records is therefore expected to be *"every ordinal
axis is `source = "human"` for Phase 1"*. The gate is **discharged by a recorded decision**, not cleared by a
passing measurement. Those are different states and the difference is the whole point of Step 4.

**Nor does ADR-0089 discharge it.** ADR-0089 is accepted, it is about facet *provenance* and it is not the gate
token: it partially supersedes ADR-0030 and explicitly does not touch ADR-0079 [documented, ADR-0089]. A reader
who sees `references/adr/0089-*.md` on disk and concludes the gate is discharged has read the wrong number — which
is why the token below moved to `0090` and why the Step 7b banner also greps `0079`'s status.

**Nor does P2's result touch it, in either direction.** P2 gates `stale_policy`, not selection (§4.1's last
bullet). Its criterion 1 fires on precision, its prescribed remedy is contraindicated by the same run, criterion 2
is not answerable and criterion 3 is unrun [measured, F40] — **so `stale_policy` is an OPEN design consequence
carried out of Step 4, not a thing Step 4 settles and not a thing ADR-0090 records.** A reader who sees
`spikes/results/p2-engine.jsonl` on disk and concludes either that the gate moved or that `stale_policy` is
decided has read a `derived_from` result as a context-vector one.

Concretely, after Checkpoint 4 the following are still true and are still enforced at Steps 5–8:

```bash
cd D:/repos/redkiln-rs
# The gate token every later step checks. It asserts a DECISION exists, not that the block lifted.
( ls references/adr/0090-*.md >/dev/null 2>&1 \
    || { echo "GATE OPEN — no ADR-0090, so no selection-path work may start. Go to Step 4."; exit 1; }
  echo "GATE DISCHARGED — ADR-0090 exists" )   # a subshell, so a pasted FAIL does not kill an interactive shell
```

Run that line at the head of Steps 5, 6, 7a and 7b. It is the cheapest possible check and it is the one thing
between a reader who skimmed the preamble and a `require` row keyed on an LLM-emitted ordinal.

---
