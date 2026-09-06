# DRAFT — F-entry for S-MIG (for Ryan's review; not committed to FINDINGS.md)

## F3x — S-MIG RESULT: 98% of spec sections lift; the losses name exactly three type decisions

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
| ACs lift ≥ 95% | 88% strict; 100% if GIVEN/WHEN/THEN and a verification are optional | **falsified as stated** — 19 of 153 specs carry ACs without GWT (45 ACs) or without a verification note (46 ACs), all in the fix lane and the late archive |
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
4. **One frozen SHA.** The fix-lane generation (42 specs) and the archived generation
   (110) differ in template; both are in the count.
