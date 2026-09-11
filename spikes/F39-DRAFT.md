## F39 — P1 arm D RESULT: **no derived facet clears §1.4's 0.95 bar, and `auth` scores 0.00 over 11 positives** — every positive the rule emits is a false positive, because `auth` is a substring of *authored*, `oauth` of *coauthor*, and `token` matches a design token; `data_migration` emits no positive at all and is **not measurable** on this corpus; the one facet at point-precision 1.00 (`ui`, 7/7) cannot clear the bar because seven positives buy a 95% lower bound of 0.65

**Date:** 2026-09-09 · **Corpus pin:** redkiln @ `47faec9f` · **Oracle:** panel-adjudicated (METHOD-P amendment 9), **never** human-panel · **Model cost of the scoring itself:** $0 · **Instrument:** `spikes/p1/derive-facets.py` (unchanged), scored by `spikes/p1/score-arm-d-panel.py`
**Evidence discipline:** every figure below is a count over 140 truth cells at the pin *[corrected 2026-09-11: the tables below were computed over 136 of those 140; see Correction (2026-09-11)]*. `unknown` is excluded from scoring and never counted against the script (§1.4). Rates carry a Wilson 95% interval. Where a facet has no positives, it is reported as *not measurable*, not as a pass or a failure.

### Metrics against the kill criteria (METHOD-P §1.4)

| criterion | threshold | measured | fires? |
| --- | --- | --- | --- |
| A derived facet at **precision ≥ 0.95** is removed from the classification record | ≥ 0.95 | **0 of 6 measurable facets clear it.** Best point estimate `ui` 1.00, 95% CI [0.65, 1.00] | **no facet is removed** |
| A derived facet at **precision < 0.95** stays LLM-emitted | < 0.95 | `auth` 0.00 · `flag_exists` 0.00 · `binary_change` 0.60 · `change` 0.72 · `toolchain` 0.73 | **fires on all five** |
| *(unstated in §1.4)* a facet with no positive instance | — | `data_migration`: the script emits **0 positives on 20 items**; nothing to be precise about | **the criterion has no denominator** |

### 1. The primary table [measured]

Union source, 20 items, 140 truth cells *[136 scored — Correction (2026-09-11)]*. `excl` counts cells the oracle returned `unknown` for.

| facet | precision | 95% CI | recall | tp | fp | fn | tn | excl | verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `ui` | **1.00** | [0.65, 1.00] | 0.78 | 7 | 0 | 2 | 6 | 4 | point clears, **interval does not** |
| `toolchain` | 0.73 | [0.48, 0.89] | 0.79 | 11 | 4 | 3 | — | 0 | stays |
| `change` | 0.72 | [0.54, 0.85] | 0.84 | 21 | 8 | 4 | — | 0 | stays |
| `binary_change` | 0.60 | [0.23, 0.88] | 1.00 | 3 | 2 | 0 | 15 | 0 | stays |
| `flag_exists` | 0.00 | [0.00, 0.66] | n/a | 0 | 2 | 0 | 13 | 4 | stays |
| **`auth`** | **0.00** | **[0.00, 0.26]** | n/a | **0** | **11** | 0 | 4 | 4 | stays *[0/12, [0.00, 0.24] on the corrected run — Correction (2026-09-11)]* |
| `data_migration` | — | — | — | 0 | 0 | 0 | 15 | 4 | **not measurable** |

`auth` is not a threshold miss. Eleven positives, eleven false, and an upper bound of 0.26 *[twelve, twelve, 0.24 on the corrected run — Correction (2026-09-11)]*: the rule carries **no signal** on this corpus. That is a result about the *method*, not about a tuning parameter.

### 2. Why `auth` scores zero — the collision is with the tool's own vocabulary [measured]

The rule is an unanchored, case-insensitive regex over paths **and diff lines**:

```
auth|login|logout|session|token|jwt|oauth|permission|role|acl|password|credential
```

It marks `auth = true` on **14 of 20 items**. Counting which alternative fired, and the word it actually fired *inside*:

| alternative | hits | the word it matched inside | count |
| --- | --- | --- | --- |
| `token` | 44 | `brand-token-substrate` · `brand-tokens.test.ts` · `redkiln.tokens.json` · `tokens_out` | 36 |
| `auth` | 43 | `authored` · `report-md-unauthored` · `author` · `authors` · `authoring` · `only-the-author-can-advance` | 40 |
| `oauth` | 8 | `coauthor` — the string `oauth` sits inside `c-oauth-or` | 8 |
| `session` | 8 | a *work* session ("a concurrent session holds the id") | 6 |
| `role` | 2 | an ARIA `role` attribute | 2 |

**The failure is correlated, not random.** The colliding vocabulary is the vocabulary of the instrument under test: a spec-driven backlog tool whose documents are *authored*, by *coauthors*, in an initiative about brand identity built on design *tokens*. A different corpus would collide differently, but any corpus collides with something — which is the point.

Ryan identified the mechanism unprompted before this was computed, guessing `identity` (visual identity, not security identity). `identity` is not in the pattern; the class was exactly right and the word was different.

### 3. Source asymmetry: what a commit touched beats what a card names [measured]

`derive-facets.py` scores three sources. They are not equivalent, and the gap is large:

| facet | `docs` precision | `commits` precision | `union` |
| --- | --- | --- | --- |
| `toolchain` | 0.50 [0.22, 0.78] | **1.00** [0.70, 1.00] | 0.73 |
| `change` | 0.50 [0.24, 0.76] | **0.80** [0.61, 0.91] | 0.72 |
| `binary_change` | no positives (recall 0.00, 3 missed) | **0.60**, recall 1.00 | 0.60 |
| `ui` | 1.00, recall 0.22 | 1.00, recall 0.67 | 1.00, recall **0.78** |

Paths named in prose are materially weaker evidence than files a commit actually touched, and on `binary_change` the `docs` source finds **nothing** — a card names a path but carries no binary marker, so the facet is unreachable from prose by construction. `union` is *worse than `commits` alone* on `toolchain` and `change`, because unioning a precise source with an imprecise one imports the imprecision. **This is a finding about which source the engine should trust, and it is independent of the regex question.**

### 4. `data_migration` has no denominator [measured]

The script emits **zero positives across all 20 items and all three sources**. Precision is undefined; a perfect method scores identically. This is a property of the corpus, not of the rule — no item at this pin carries a migration — and it means §1.4's criterion cannot be evaluated for this facet at all. It is reported as unmeasurable rather than folded into either column.

Related: **4 of the 20 items changed no code whatsoever** (every changed file is `.bklg/` bookkeeping or telemetry). The panel returned `unknown` on all 16 of their semantic-facet cells; the script emitted a confident boolean on every one. That is the `excl = 4` column in §1, and it is the strongest evidence in this finding for giving the boolean facets the third value the ordinals already have.

### 5. How the truth set was built [measured]

140 cells, three provenances, under amendment 9 *[136 of them reached the scorer — Correction (2026-09-11)]*:

| provenance | cells | what decided it |
| --- | --- | --- |
| deterministic oracle | 60 | `spikes/p1/oracle-mechanical.py` — canonical extension/manifest table and git's own `--numstat` marker, independently implemented |
| panel, unanimous | 39 | three independent model seats agreeing, accepted with the evidence that decided each |
| panel, unanimous (adversarial flag mechanically moot) | 9 | flag was a prediction about the script that did not fire |
| **human adjudication** | **32** | Ryan, blind: the cell and its evidence shown, the panel's and the script's positions revealed only after he answered *[the surface is named in Correction (2026-09-11), PIA-096]* |

The panel labelled 80 semantic cells: 39 accepted unanimous (23 decided, 16 honest `unknown`), 41 flagged, of which 9 were mechanically moot, leaving **32 contested**. Ryan's verdicts: **8 true, 24 false, 0 unknown — and 15 of 32 differ from the script**.

### 6. What this settles, and what it does not

**Settles:** the lexical method is not a tuning problem. `auth` at 0.00 with an upper bound of 0.26 cannot be recovered by adding word boundaries to one alternative. It is the strongest available evidence for the DECLARED-over-INFERRED design in `references/2026-09-09-change-classification.md` §4, which six independent research axes reached without reference to this number.

**Does not settle:** ADR-0079. The blocked question is whether an *LLM-emitted* context vector can drive selection; arm D measures whether a *derived* one can. Arm D's answer — not as implemented — narrows the options without deciding among them, and 0079 stays `proposed` pending the arm-L panel.

### Limits

1. **No facet has enough positives to clear 0.95 with confidence, even in principle.** `ui` is 7/7 and its lower bound is 0.65; roughly 60 positives are needed for a lower bound above 0.95 *[corrected 2026-09-11: 73 — see Correction (2026-09-11)]*. A 20-item corpus cannot pass this criterion for any facet, however good the method. **§1.4's threshold is not reachable at this n** — the same class of defect as P1's α rule (F36) and P7's criterion 1 (F37): a pre-registered bar that the registered sample size cannot decide.
2. **39 of the 140 cells rest on unanimous 3-seat model agreement with no human.** *Nine Judges, Two Effective Votes* measured a 9-judge panel supplying ~2.18 effective independent votes, with accuracy 8–22 points short of independent-vote predictions [preprint, arXiv:2605.29800]. Small-panel unanimity is a weaker correctness signal than its arithmetic implies, and it fails in the correlated direction. The 32 contested cells carry a human; these 39 do not.
3. **The oracle is independent on `toolchain` and on `binary_change`'s Unreal shortcut, and not on the rest.** For `change`, git's marker is the ground truth for the binary component, so oracle and script agree there by construction; only the Unreal-path shortcut is genuinely measured.
4. **One repo, one operator, one pin.** Every number is over `47faec9f`.
5. **A scorer defect was found and fixed before this was reported.** The oracle emits `binary_change` as a JSON bool and the first scorer compared it to the string `"true"`, reading every oracle positive as false and scoring the facet 0.00. Corrected to 0.60. The defect class — assuming a value's shape instead of checking it — occurred three times in this session's tooling and is the reason §5's provenance table is stated explicitly rather than trusted.
6. **`unknown` is excluded, not counted.** 16 semantic cells are excluded on that basis. Reporting them as agreement would have raised every semantic precision figure.

### Correction (2026-09-11)

Dated corrections from the 2026-09-10 pre-implementation audit (`references/audit/2026-09-10/MASTER.json` PIA-073,
PIA-077, PIA-096; ERRATA E-0089-2). The original figures above are left as written; each corrected figure is stated
beside its original and here with its recount.

**1. Limits 1 — the minimum n for §1.4's 0.95 bar (PIA-073).** "Roughly 60 positives" was not computed. At the
registered two-sided Wilson interval (z = 1.96) a facet needs **at least 73 all-correct positives** for its lower
bound to exceed 0.95 (73/73 → lower bound 0.9503; 72/72 → 0.9496); **92** with continuity correction; **72** under
Clopper–Pearson [measured, recomputed 2026-09-11 from the interval formulas; the audit's figures agree]. `[measured]`
stays only on the computed number; the sentence's conclusion — §1.4's threshold is not reachable at this n — is
unchanged and is now reported by name: the 0.95 criterion is **powerless** on a 20-item corpus whose largest
positive count is 29 (`change`). METHOD-P carries the minimum n as amendment 11.

**2. The scorer scored 136 of the 140 cells the truth set holds, and the table changes in five cells (PIA-077).**
`spikes/results/p1-arm-d-panel-accepted.jsonl` keys four cells of one item under the slug
`…/kb-harvest-and-closeout/resolve-open-questions` (plural) while the engine, the oracle and the contested file key
it `…/resolve-open-question`; `score-arm-d-panel.py`'s `if source not in eng.get(...): continue` skipped them
without a word. The four cells are that item's `auth`, `data_migration`, `flag_exists` and `ui`, all panel
`false`. The scorer is corrected in place (the result files are records and are untouched): the known mis-key is
normalised on read, and a truth cell with no engine record now **aborts the run** naming the cell (fail closed).
Re-run 2026-09-11 from `D:/repos/redkiln-rs`, no arguments, $0, nothing dispatched; the output verbatim:

```
truth cells: 140   provenance: {'oracle': 60, 'panel': 39, 'panel(moot-flag)': 9, 'human': 32}
every truth cell has an engine record for every source (fail-closed check passed; key fix applied to 1 panel slug)

facet           source      prec          95% CI    rec   tp  fp  fn  tn  excl
------------------------------------------------------------------------------------
toolchain       docs        0.50     [0.22,0.78]   0.29    4   4  10   0    0
toolchain       commits     1.00     [0.70,1.00]   0.64    9   0   5   0    0
toolchain       union       0.73     [0.48,0.89]   0.79   11   4   3   0    0

change          docs        0.50     [0.24,0.76]   0.20    5   5  20   0    0
change          commits     0.80     [0.61,0.91]   0.80   20   5   5   0    0
change          union       0.72     [0.54,0.85]   0.84   21   8   4   0    0

binary_change   docs         n/a             n/a   0.00    0   0   3  17    0
binary_change   commits     0.60     [0.23,0.88]   1.00    3   2   0  15    0
binary_change   union       0.60     [0.23,0.88]   1.00    3   2   0  15    0

auth            docs        0.00     [0.00,0.66]    n/a    0   2   0  14    4
auth            commits     0.00     [0.00,0.24]    n/a    0  12   0   4    4
auth            union       0.00     [0.00,0.24]    n/a    0  12   0   4    4

data_migration  docs         n/a             n/a    n/a    0   0   0  16    4
data_migration  commits      n/a             n/a    n/a    0   0   0  16    4
data_migration  union        n/a             n/a    n/a    0   0   0  16    4

ui              docs        1.00     [0.34,1.00]   0.22    2   0   7   7    4
ui              commits     1.00     [0.61,1.00]   0.67    6   0   3   7    4
ui              union       1.00     [0.65,1.00]   0.78    7   0   2   7    4

flag_exists     docs         n/a             n/a    n/a    0   0   0  16    4
flag_exists     commits     0.00     [0.00,0.66]    n/a    0   2   0  14    4
flag_exists     union       0.00     [0.00,0.66]    n/a    0   2   0  14    4

====================================================================================
VERDICT on the union source — METHOD-P §1.4: precision >= 0.95 removes the facet from the model
====================================================================================
  toolchain        precision 0.73 over 15 positives, 95% CI [0.48,0.89] -> STAYS
  change           precision 0.72 over 29 positives, 95% CI [0.54,0.85] -> STAYS
  binary_change    precision 0.60 over 5 positives, 95% CI [0.23,0.88] -> STAYS
  auth             precision 0.00 over 12 positives, 95% CI [0.00,0.24] -> STAYS
  data_migration   NOT MEASURABLE — the script emitted no positive; nothing to be precise about (4 cells excluded as unknown)
  ui               precision 1.00 over 7 positives, 95% CI [0.65,1.00] -> point >= 0.95 but interval does not clear
  flag_exists      precision 0.00 over 2 positives, 95% CI [0.00,0.66] -> STAYS

Reported as PANEL-ADJUDICATED (METHOD-P amendment 9), never as human-panel.
```

What moved, against §1's table: `auth` **0/12, [0.00, 0.24]** (was 0/11, [0.00, 0.26]; the twelfth positive is
the same substring collision — the engine's `evidence` for the cell is `design.tokens` and a diff line containing `brand-authored-vs-derived`, and the panel's unanimous `false` cites a knowledge-base prose file with no identity, credential or access-control change [measured, `p1-arm-d.jsonl`, `p1-arm-d-panel-accepted.jsonl`]);
`data_migration` tn 16 (was 15); `ui` tn 7 (was 6); `flag_exists` tn 14 (was 13); the `docs` source of `auth`
fp 2 (was 1), [0.00, 0.66]. Every precision, every interval on the other five facets, every verdict and the
headline are unchanged: **no derived facet clears the 0.95 bar; `auth` carries no signal**. The audit's expected
count ("136 cells") is read here as the number of cells the 2026-09-09 run actually scored; the corrected run
scores all **140** — the two statements agree and the audit's figure is not forced onto the output. The
"eleven positives, eleven false" restatements in the design, the registers and `HANDOFF.md` read
**twelve positives, twelve false** [measured, this re-run].

**3. §5 — the adjudication surface (PIA-096; DECISIONS-2026-09-11 #13).** The 32 contested cells were adjudicated
on a **claude.ai Artifact page** built for the purpose (a data-entry page that showed the cell, the diff evidence
and the question, and revealed the script's and the panel's positions only after the answer was entered). **The
page is not in the repository**, and its answers were **recovered from browser memory** after a synchronous
`db.doc()` failure on a path segment (`spikes/plans/HANDOFF-2026-09-09-design-session.md` :79-82), so the
reveal-after-answer logic is not inspectable from this tree. The adjudication is therefore **attested-blind**
under METHOD-P amendment 8's own minimum, and the `blind_attestation:` line that carries it (who attests, what was
not opened, when) — `spikes/results/p1-arm-d-labels-ryan.md` :27 — is Ryan's to write and is still `FILL-ME`.
Until it is written, §5's "Ryan, blind" is a claim by the session that built the page, not a record. Whether the
Artifact page still exists and can be exported into `spikes/p1/` is a handoff, not done here.
