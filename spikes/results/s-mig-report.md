# DRAFT — F-entry for S-MIG (for Ryan's review; not committed to FINDINGS.md)

## F3x — S-MIG RESULT: 98% of spec sections lift; the losses name exactly three type decisions

> *Corrected 2026-09-11:* 98% is the spec-section slot; the record-weighted typed-slot overall is **96.0%** (3,011 / 3,138). See "Correction (2026-09-11)" at the foot.

**Established by:** a deterministic census of every markdown file under `.bklg/` and
`.kb/` at corpus pin `47faec9f` — **1,312 files, 6,877 (artifact, section) records** —
read via `git show`, never the working tree. **Zero model cost.** Pre-registered in
`METHOD-S.md` §4; instrument `spikes/results/census-s-mig.py`; records
`spikes/results/s-mig.jsonl`. Run 2026-09-05.

**Instrument disclosure, up front.** The lifters were iterated four times against the
corpus before the recorded run — each round widened recognition of a *real* form the
first draft missed (bold `**AC-001**` table ids, `- **EC-001 — …**` bullets, `Test:` as
a verification marker, gate results as bullets or fenced blocks, a per-kind key census
in place of a hand-written frontmatter key list). This is a census of the corpus's
regularity, and the instrument was tuned to the corpus's forms; it is not a blind test
of a parser. The §4.4 predictions were written before the first run and are compared
below regardless.

### The headline — per typed slot, against the §4.3 decision rule

| kind / slot | n | Lifted | w/ loss | Failed | share | §4.3 verdict |
| --- | --- | --- | --- | --- | --- | --- |
| spec / frontmatter | 153 | 153 | 0 | 0 | 100% | stands |
| spec / **acceptance criteria** | 153 | 134 | 19 | 0 | **88%** | REVIEW (80–95) |
| spec / error conditions | 152 | 152 | 0 | 0 | 100% | stands |
| spec / non-functional | 152 | 152 | 0 | 0 | 100% | stands |
| spec / PR boundary | 136 | 135 | 0 | 1 | 99% | stands |
| spec / merge gate | 134 | 133 | 0 | 1 | 99% | stands |
| spec / anchors table | 152 | 152 | 0 | 0 | 100% | stands |
| impl-report / TDD evidence | 156 | 142 | 7 | 7 | 91% | REVIEW |
| impl-report / **gates** | 156 | 117 | 31 | 8 | **75%** | RECLASSIFY |
| review / frontmatter | 38 | 36 | 0 | 2 | 95% | stands |
| review / coverage map | 23 | 21 | 2 | 0 | 91% | REVIEW |
| review / rubric table | 38 | 38 | 0 | 0 | 100% | stands |
| review / **gate table** | 23 | 4 | 7 | 12 | **17%** | RECLASSIFY |
| ledger / fenced YAML | 110 | 108 | 0 | 2 | 98% | stands |
| discover / signals, gate checklist | 122 | 122 | 0 | 0 | 100% | stands |
| all item cards / frontmatter (story, project, initiative, incident, bug) | 209 | 209 | 0 | 0 | 100% | stands |
| kb atoms / frontmatter, links | 47 | 46 | 1 | 0 | 98% | stands |
| companions / frontmatter (integration, grounding, decomposition, implementation, plan) | 45 | 19 | 6 | 20 | 42% | RECLASSIFY |

Everything designated opaque-by-design (context pack, repro, behaviour, out of scope,
kb bodies, discovery/research material — 3,100+ sections) lifted as opaque with no
exceptions, which is the trivial half of the result and is stated so it is not counted
as evidence.

### Against the §4.4 predictions

| prediction | measured | verdict |
| --- | --- | --- |
| ACs lift ≥ 95% | 88% strict; 100% if GIVEN/WHEN/THEN and a verification are optional | **falsified as stated** — 19 of 153 specs carry ACs without GWT (45 ACs) or without a verification note (46 ACs) *[cap-truncated; uncapped 76 / 87 loss lines — Correction (2026-09-11)]*, all in the fix lane and the late archive |
| Anchors ≥ 90%, boundary ≥ 90% | 100% / 99% | holds |
| TDD table < 80% (columns changed over time) | 91% lift; every one of 142 lacks the F25 mutation column | **falsified in the good direction** — the table is regular; it is the *columns* that are missing, uniformly |
| kb bodies opaque, 100% | 100% | holds |

### What this settles for D5/D6 (the section list)

1. **The AC type cannot require GIVEN/WHEN/THEN or a verification field.** 12% of
   specs — every one of them shipped and reviewed — write criteria as a plain sentence,
   and the verification lives in prose (`Test:`) or nowhere. The type is
   `{ id, criterion: text, gwt: Option<Gwt>, verification: Option<text> }`, and the
   *policy* (not the type) decides whether a missing verification blocks a stage.
2. **Gate results are never parsed again.** 25% of implementation reports and 83% of
   reviews record gate results as bullets, fenced output or prose. In the new design
   the toolchain adapter *emits* a typed gate result and the report *renders* it; the
   migration fills `unknown` for these 40 sections rather than guessing.
3. **The TDD evidence table migrates with `mutation: unknown` on 142 of 156 rows.**
   The F25 columns (Assertion / Mutation / Test-run / Red?) exist in 0 reports. This is
   the 22% per-AC gap (F24) seen from the artifact side.
4. **`plan.md` is dead schema.** 110 of 110 `plan.md` files are the unfilled template —
   the empty `1.` scaffold with template prose. Planning happened in `_plan.md`,
   `_storymap.md` and `spec.md`; the stage artifact was never authored once. Under the
   Dagster rule in the proposal (an artifact nothing consumes is pruned) it does not
   exist in Redkiln-RS.
5. **Companion frontmatter is not one type.** `_integration.md` carries
   `dod_green`/`reachability_ok`/`deferred_scenarios` and nothing of the stage
   schema; `_grounding.md` and `_decomposition.md` have no frontmatter in 10 of 28
   cases. These are JSON sidecars in layout C (D6), each with its own schema, and
   the migration tool needs one per companion kind.
6. **Two `_ledger.md` files are unreadable YAML** (unescaped quotes inside quoted
   evidence strings). Real content, hand-fixed in migration; the new sidecar is JSON.
7. **Frontmatter is regular everywhere.** 209/209 item cards, 153/153 specs, 47/48 kb
   atoms; the corpus's own key census (`archetype`, `slice` on 110 stories;
   `last_reviewed` on 49 atoms) is the type, and the hand-listed keys in the
   proposal were incomplete on both.

### The opaque list, settled (§9 "still open")

Prose sections in `spec.md`: one-line/executive summary, context pack, repro,
behaviour and interfaces, implementation notes, clarifications, dependencies, data
and migrations, risks and coupling, interaction quality, integration contract, out of
scope, amendments. In `implementation-report.md`: changes, notes, post-review
amendments (the 8 dated "Fix pass / amendment" headings that no slot fits are prose
appended after the fact and belong to an `amendments: Vec<{date, text}>` field, not a
section grammar). In cards: everything but the frontmatter and the AC list.

### Limits

1. **The instrument was tuned to the corpus** (disclosed above). The share figures
   say how regular the corpus is under a locator that knows its forms; a new corpus
   without the old validators would be less regular, which is the argument for
   rendering rather than parsing and is not tested here.
2. **"Lifted" means the section parsed into the slot's shape, not that the migration
   is lossless.** Only the criteria and ledger lifters check internal completeness;
   tables elsewhere are counted by row.
3. **18 files are unmapped** — evidence attachments and readings under two archived
   projects. They migrate as opaque attachments; no type is proposed for them.
4. **One frozen SHA.** The fix-lane generation (42 specs *[43 — Correction (2026-09-11)]*) and the archived generation
   (110) differ in template; both are in the count.

### Correction (2026-09-11)

Audit PIA-078; DECISIONS-2026-09-11 #11; ERRATA E-0075-1; METHOD-S §7 (2026-09-11, item 4). Original figures above
are left in place with markers.

**1. The per-AC loss counts were cap-truncated.** `census-s-mig.py` recorded each `LiftedWithLoss` AC section's loss
list as `loss[:6]`, and 12 of the 19 spec records hit the cap, so the "45 ACs without GWT / 46 without a
verification note" in the predictions table are sums over truncated lists. The cap was lifted for future runs on
2026-09-11 and the script re-run **read-only at the pin** (`python spikes/results/census-s-mig.py --redkiln
D:/repos/redkiln --sha 47faec9f --out <scratch file>`; 45 s; every one of the 6,877 records' `(path, kind, slot,
outcome)` identical to `s-mig.jsonl`, which is not replaced). Uncapped, over the same 19 specs:

| count | capped (recorded run) | uncapped |
| --- | --- | --- |
| loss lines "no GIVEN/WHEN/THEN" | 45 | **76** |
| loss lines "no verification" | 46 | **87** |
| distinct `(spec, id)` pairs — no GWT / no verification | — | **75 / 85** |
| ACs with at least one loss | — | **131** |
| ACs lacking both | — | **29** |
| ACs in the 19 specs | — | **208** |

[measured, uncapped re-run 2026-09-11]. The slot-level figures — 19 of 153 specs, 88% — are unaffected, because the
slot outcome was never capped; only the *within-record* enumeration was. The identical ids repeat across specs
(AC-001 … in every fix-lane spec), which is why 76 lines are 75 pairs and only 15 distinct id strings.

**2. The 98% is one slot, not the census.** The heading's 98% is the **spec-section** slot. Over every typed
slot the record-weighted overall is **96.0% = 3,011 Lifted / 3,138 (Lifted + LiftedWithLoss + Failed)** — 98.3%
only if LiftedWithLoss is counted as lifted [measured, `s-mig.jsonl` recount 2026-09-11]. Restatements that read
"the census overall of 98%" read 96.0% typed-slot with that denominator.

**3. Two counts the table under-reports by one.** The table's Failed column sums to **53**; the census holds **54**
— the one not tabled is a `report / gates` slot (1 Failed) outside the fifteen tabled slots [measured,
`s-mig.jsonl`]. And Limits 4's "fix-lane generation (42 specs)" is **43**: the `support/` lane holds 43 `spec.md`
at the pin, the one the count missed being
`.bklg/support/issue-146-147-story-gate/issue-146-147-boundary-window/spec.md` [measured, `s-mig.jsonl` paths].
AC-469 assigns that file; the design and the AC register carry the 43.

**4. Which instrument, and the 0/156.** The census instrument was `spikes/results/census-s-mig.py`; nothing
named `redkiln-migrate` ran (it does not exist). The "0/156 historical reports carry the discriminating-mutation
columns" figure (ADR-0019 :51; ERRATA E-0019-1) is the 2026-09-05 proposal's; what **this** report measures is
narrower and is the figure to cite: **142 of the 142 lifted TDD tables lack the F25 mutation column** (§ "What this
settles" item 3), out of 156 impl-reports of which 7 lifted with loss and 7 failed to lift and were not assessed
for the column [measured, `s-mig.jsonl`]. 0/156 is 0/142 plus 14 unassessed, not a count over 156.
