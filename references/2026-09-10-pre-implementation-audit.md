# Redkiln-RS — pre-implementation audit of the planning corpus

**Date:** 2026-09-10 (external fetches) / 2026-09-11 (synthesis) · **Status:** Review. It decides nothing, supersedes nothing and edits nothing; every remediation is a proposal with a named owner, and
every concern is a claim about the record at HEAD `009a4ec` with its anchors, not an instruction to change it. · **Scope:** the authoritative set, audited as the primary read set — `RUNBOOK.md`;
`references/2026-09-08-architecture-design.md`; `references/2026-09-08-acceptance-criteria.md`; `references/2026-09-08-success-measures.md`; `references/2026-09-08-evidence-brief.md`;
`references/2026-09-08-evidence-augmentation.md`; `references/adr/` (89 records); `spikes/METHOD*.md` and `spikes/F##-DRAFT.md` — plus `references/2026-09-09-change-classification.md` and, in part,
the spike runner sources RUNBOOK Step 7c lifts (`runner.rs`, `step.rs`; the rest is in §8). Superseded documents (`references/2026-09-05-architecture-proposal.md` rev. 4, the RFC and its amendment,
the process-catalog proposal, the research briefs) were checked as sources only, never as authority. Which parts of the primary set were reached by citation rather than read whole is stated in §8, not
assumed away here. · **Inputs:** four workflows — a census (10 agents; nine script-computed artefacts, all nine re-run reproducible); an internal audit (34 finders — 21 area slices and 13 corpus-wide
lenses — each challenged by one of 34 refuters); an external grounding (32 verifiers and 20 second opinions over every non-confirmed verdict); and this synthesis (11 cluster agents, 1 ranker, 2
critics, 6 writers, 1 assembler-editor). Headline counts: 357 candidate findings → 338 survivors (P1 24, P2 213, P3 101; 19 killed) → 151 bucketed concerns → **110 concerns** after merge (P1 17, P2
82, P3 11); 860 external claims re-verified live on 2026-09-10 — **697 confirmed, 105 drifted, 29 contradicted, 16 unverifiable, 13 stale-by-date**, 28 verdicts flipped by the second opinion (25
drifted→confirmed, 2 contradicted→drifted, 1 drifted→contradicted); four best-practice lanes, 77 rows, unrefuted. · **Evidence discipline:** every load-bearing sentence carries an anchor (`file:line`)
with a verbatim quote and one of three tags — **[measured]** = computed or read by this audit at the pin; **[documented]** = a live source says it, with URL and fetch date; **[inferred]** = reasoned,
and labelled as such. A rate carries its interval or says none was computed; saturated, powerless, not-measurable and not-answerable are four different faults and are named as such. Priority is P0
(Step 5 would build the wrong thing, or a step depends on something unrun or absent that nothing fences), P1 (blocks a specific Phase-1 workstream), P2 (correctness of the record: a wrong number, a
drifted citation, a contradiction with no build consequence), P3 (hygiene and convention). Every concern is **NEW** (the corpus does not record it) or **SHARPENS-KNOWN** (the corpus records the area,
not this defect; the recording row is cited). This corpus audits itself: it carries 1,381 self-recorded issue rows across 106 files (`census/known-issues.json`: 332 scope limits, 278 deliberate
exceptions, 214 missing artefacts, 89 known contradictions, 81 unrun spikes, 79 unverified claims, 74 not-answerable, 49 blocked decisions, 49 instrument confounds, 32 owed by a human, 30 stale
documents, 26 saturated metrics, 24 n = 1, 19 not-measurable, 5 powerless criteria), and nothing in those rows is presented here as a discovery.

## 0. How to read this

**Priority.** P0 blocks pen-to-paper: Step 5 would build the wrong thing, or a step depends on something unrun or absent that no checkpoint, legend line or ADR fences. P1 blocks a specific Phase-1
workstream (a Step, a crate, a gate) and must be settled by a decision on record before that workstream starts. P2 is a fault in the record with no build consequence — a wrong number, a drifted
citation, two documents that disagree — and batches into correction passes. P3 is hygiene and convention. Within a priority the order is the build stage an implementer reaches first (Steps 0–4a and 2,
then 5, 6, 7a, 7b, 7c, 7d/8, then record-only), then breadth (member findings, descending), then bucket order.

**Classification.** NEW: no row of the corpus's own 1,381 known-issue rows records the defect. SHARPENS-KNOWN: a row records the area or an adjacent symptom but not this defect; the row id and anchor
are cited in the concern so the reader can see what the corpus already knew. Three ALREADY-RECORDED survivors of the internal audit were dropped at merge; none appears below. NEW is an upper bound —
the classification rests on a keyword index whose recall was not measured (§8).

**Ids.** Concerns are `PIA-001` … `PIA-110` (pre-implementation audit), a prefix chosen to collide with none of the corpus's namespaces (F, AC, SM, ADR, S, P, R, D, PC, E, H, L, A, GATE, SCHEMA).
Review limits carry no PIA- id and no NEW/SHARPENS-KNOWN tag; they are limits of this review, not concerns about the corpus. Each concern names the internal-audit findings it merged (`member_ids`) so
a reader can trace any sentence back to the finder that raised it and the refuter that challenged it.

**Anchors.** `file:line` against HEAD `009a4ec68119896e63a3fb3e405ce1b6616665c3` of `D:/repos/redkiln-rs`, the pin every workflow measured [measured: `census/constants.json` `meta.corpus_git_head`; 17
untracked and 0 modified paths in the working tree, none of them the file of any finding anchor]. Quotes are verbatim; where a quoted line is long the quote is a substring, never a paraphrase.
External sources carry the URL and the fetch date 2026-09-10; a verdict on such a source is STALE-BY-DATE by construction from the next publish onward.

**What the corpus got right** [measured, `census/reconcile.json` cross-checks X1a–X12a, 43 of 44 pass; the one failure is the audit's own seed of ~48 arXiv ids against a measured 99]: 89 ADRs, 86
accepted / 2 superseded / 1 proposed, every mandatory section present, numbering gap-free, and all 9 supersession links reciprocal · 514 AC and 135 SM definitions agreeing across three independently
computed artefacts, with the six dangling AC ids and the two undefined SM range ends the register itself declares · the 39-row design tally 26 DECIDED / 3 PROVISIONAL / 10 BLOCKED reproduced from the
table, the constants and the known-issues rows alike, with a BLOCKED row recorded for every BLOCKED area at the same line · 697 of 860 external claims confirmed live, 101 of the 144 load-bearing ones
among them · and of three preprints whose abstracts read as drift candidates, two dissolved on full-text reading (arXiv 2603.18740: 250 pairs is right; arXiv 2604.22411: all four values are in Table
1) and one held (arXiv 2603.04417, joint-summary sentence only).

## 1. The verdict

The corpus is ready to build from at Step 5, on one reading: no concern is P0. Nothing found makes Step 5 write the wrong crate; every dependency on something unrun or absent is fenced by a
checkpoint, a `[WILL EXIST]` legend line or an ADR reading resolvable on the page. What held is substantial [measured]: the census reproduced the id namespaces, ADR graph, area tallies and most
restated numbers; 697 of 860 external claims confirmed live (105 drifted, 29 contradicted, 16 unverifiable, 13 stale by date); every §9.2 crate pin clear of every advisory checked; 1,381 self-recorded
issues absorbing 69 of these 110 concerns as sharpenings, not discoveries.

Seventeen P1s must be settled before or at Step 5, by record, never by edit. Sequencing: the three LLM boolean facets have no disposition between Step 4's pre-registered rule and Checkpoint 4a
(PIA-003); AC-520 has no passing route and Phase-0 exit (c) is read three ways (PIA-001); the corpus Steps 6–7 test against is neither moved by Step 2 nor reachable from the pinned SHA (PIA-002).
Contracts Step 5.3 snapshots: the closed-output schema rules cannot all hold and `StepError` is unenumerated (PIA-005); `Findings.repro` has no mutant field (PIA-007); the companion kind list is
unsettled (PIA-006); the self-hosting exit has no Rust toolchain adapter (PIA-004). Contracts later steps build to: memo tier and keys (PIA-008), common-dir on gix (PIA-009), the Harness port's
dispatch and cancel (PIA-010), pack authoring's missing step (PIA-011), six adapter-level decisions (PIA-012 to PIA-017).

The 82 P2s carry no build consequence; four passes clear them: Step-5 manifest and checkpoint fixes (PIA-023 to PIA-039); one erratum record per affected accepted ADR (0077, 0083/0085, 0084,
0086–0089); a dated-correction pass over statistics and baselines (PIA-073 to PIA-078); a closeout pass over ranges, ids and un-propagated corrections (PIA-070, PIA-075). The 11 P3s are hygiene.

"No P0" is a finding about fences, not a clean bill. Eighteen rk-git behaviours are left to the implementer (PIA-040); 41 of 110 concerns are new to the corpus; five sizing figures are powerless at
their own n (PIA-073); the two modalities that could move what Steps 5, 7a and 7b build — compiling the port traits, parsing the predicate grammar — were not run (§8). PIA-003, the closest P0
candidate, stays P1: Checkpoint 4a's block is a contradiction resolvable on the page, not a dependency on something absent — one ranker's reading, adjudicated by no human.

### Top ten

| # | id | priority | title | owner |
|---|---|---|---|---|
| 1 | PIA-001 | P1 | AC-520 has no passing route; the rigor-monotonicity lint rejects the shipped catalog; Phase-0 exit (c) is read three ways | Ryan (operator) for the provenance re-class and P7-CEIL; RUNBOOK author for 3c/4a/7b/8 |
| 2 | PIA-002 | P1 | The corpus Steps 6–7 test against is neither moved by Step 2 nor reachable from the pinned SHA; golden logs and v0's contract unstated | Ryan (RUNBOOK Step 2, AC register §13, dependency policy); Step 7d implementer for the v0 definition and run-contract AC |
| 3 | PIA-003 | P1 | The three LLM boolean facets have no disposition between Step 4's rule and Checkpoint 4a; the checkpoint asserts a `surfaces.toml` that exists nowhere | Ryan (operator) via ADR-0090; RUNBOOK author for 4.1 and Checkpoint 4/4a |
| 4 | PIA-004 | P1 | Phase-1 exit (1) self-hosts on a Rust repo with only a Node toolchain adapter; ADR-0011's story-lane half is checked by nothing | Ryan (design + RUNBOOK author; ADR successor) |
| 5 | PIA-005 | P1 | rk-types' closed-output schema rules cannot all hold; `StepError` is never enumerated | ADR-0084 author (Ryan) for the supersession; rk-types author for the StepError table |
| 6 | PIA-006 | P1 | The companion kind list is delegated to a runbook that never mentions it; AC-143 sits unfenced in Steps 5 and 6 | Ryan (operator) — a recorded decision against ADR-0075 cl. 3; RUNBOOK author for 5.3/6.3 and the acceptance lines |
| 7 | PIA-007 | P1 | `Findings.repro` as fixed by ADR-0074 and snapshotted at Step 5.3 has no field for the mutant or suite AC-289/AC-305 re-execute | Claude (ADR/AC author); Ryan Britton (accepts the schema change) |
| 8 | PIA-008 | P1 | The memo record is specified three ways and its keys are unpinned; the per-step `memo = false` rule ADR-0059 withdrew is still demanded | rk-kernel / rk-store owner (Steps 6, 7b); ADR maintainer (Ryan) |
| 9 | PIA-009 | P1 | ADR-0077 keeps linked-worktree common-dir resolution on gix with no written fallback for an S-GIX/a no-go | Ryan (ADR author); Step 6 implementer |
| 10 | PIA-010 | P1 | The Harness port's `dispatch`/`cancel(handle)` contract is unimplementable as written; the POSIX process-group cancel never holds on Windows | rk-ports / rk-adapter-claude author |

Titles here are abridged; the full title, statement, anchors and remediation of each are in §3. Owners are as the concern records name them.

## 2. The concerns, by priority

One hundred and ten concerns follow, merged from 151 bucketed records that were themselves consolidated from the 338 refuted survivors (30 merges, 9 priority reconciliations, every one recorded in
`concerns/MASTER.json`). Each carries: priority; NEW or SHARPENS-KNOWN with the known-issue row it sharpens; the build stage it reaches first; a statement; anchors with verbatim quotes; why it matters
before Step 5; a remediation that respects the house rules — accepted ADRs are superseded, never edited; METHOD files take a dated amendment; an unknown discharges as a named spike with a named
consumer (ADR-0012); a criterion whose n cannot decide it is reported as powerless, never re-thresholded — and an owner. Seventy-eight carry the master list's per-concern tag [measured], 29
[documented], 3 [inferred]; in the text below, a sentence that reads the corpus at the pin is tagged [measured] even where the concern's master tag is [documented] (PIA-009, PIA-063, PIA-106, PIA-110
rest on corpus reads alone). The ordering rule is stated in §0; §3 to §5 present P1, then P2 by theme, then P3.

| priority | NEW | SHARPENS-KNOWN | total |
|---|---|---|---|
| P0 | 0 | 0 | **0** |
| P1 | 6 | 11 | **17** |
| P2 | 30 | 52 | **82** |
| P3 | 5 | 6 | **11** |
| **total** | **41** | **69** | **110** |

By build stage reached first [measured, `concerns/MASTER.json` `build_stage`]: Steps 0–4a and 2 (before Step 5) 8 · Step 5 22 · Step 6 7 · Step 7a 2 · Step 7b 14 · Step 7c 17 · Step 7d/8 3 ·
record-only 37.

## 3. P0 and P1 — settle before Step 5

No concern met the P0 bar: every P1 was re-read against the P0 definition [measured]. The closest was PIA-003 (Checkpoint 4a cannot pass from Step 4a's instructions and Halt 8 says a failed checkpoint
is a stop); it is held at P1 because the zero-count line passes under ADR-0089 Decision 3's unqualified reading and the `surfaces.toml` line is fenced by the [WILL EXIST] legend — a contradiction
resolvable on the page, not a dependency on something absent. Nothing Step 5 builds is the wrong thing under any of the seventeen concerns below; each blocks a named Phase-1 workstream, and seven of
them (PIA-001 to PIA-007) land at or before Step 5 by the build stage an implementer reaches first [measured, build_stage]; the rest land at Steps 6–7c but the record each needs (a successor ADR, a
RUNBOOK amendment, a register row) is cheapest written before the crate it governs exists. Ordered by the build stage an implementer reaches first, then by breadth. Concern headings sit under five
stage sub-headings.

### 3.1 Before Step 5 — the facet registry, the corpus substrate and the Phase-0 exit

### PIA-001 — AC-520 fails pack load on `stage`, `rigor`, `role`, `cost` and `recent_deploy` with no RUNBOOK step or registered spike supplying a passing route; the rigor-monotonicity lint rejects the shipped catalog; and Phase-0 exit (c) is read three ways (Step 3c checks S-BIN alone while design 10.2/10.5 mark S-FACET 'gates Phase 1', registered and unrun)

**Priority:** P1 · **Classification:** SHARPENS-KNOWN ([B01-C02] AC register 1157 (AC-520 body: 'fails load on all eleven'); ARCH-8.10-003/ARCH-10.5-002 (S-FACET registered, unrun); ARCH-10.5-013,
ARCH-1.2-015, ARCH-10.4-005 (broken bisect rung, P7-CEIL awaiting); design 1896 ('S-BIN, P1-D and S-FACET are the only three rows that stop Phase-1 work') | [B02-C04]
references/2026-09-08-acceptance-criteria.md:1287 (residual-risk bullet: 'the gate's honest effect is to fail load for the derived facets') and :1162 (names the registry-change route); neither is
indexed in known-issues.json | [B04-C02] ARCH-10.4-002 (design:1817), H10-LOOP-T-003 (handoff:52), ARCH-1.2-038, ARCH-10.5-003, H10-LOOP-P-001, RUNBOOK:865-880 (S-BIN and S-FACET unrun, 'gates Phase
1'). None records that Step 3c omits S-FACET or that section 10.2's Enter and May-not columns contradict each other.) · **Category:** blocked-leakage · **Confidence:** high · **Owner:** Ryan
(operator) for the provenance re-class and P7-CEIL; RUNBOOK author for 3c/4a/7b/8

AC-520, built at Step 7b, fails pack load unless every `source = "derived"` facet carries a scored precision record; Checkpoint 4a holds the derived count at eleven ('both are 11'), `stage`, `rigor`,
`role`, `cost` and `recent_deploy` have no record, `surfaces` is unscored, and S-FACET's registered consumer covers only `auth`/`surfaces`, `data_migration`, `ui` and `flag_exists`, so five of the six
can be scored by no registered spike [measured against spikes/p9/facets.toml]. Step 8 (6, second lint) nevertheless expects `--facet-provenance` to pass, `doctor` refuses an unloadable pack, and every
`require` row keys on `stage`, so once 7b builds the lint as written no Phase-1 pack loads on any path the RUNBOOK provides — unlike AC-071, whose passing branch is 'zero facets declared llm', AC-520
has no passing branch any Phase-1 step reaches [measured]. In the same window ADR-0081 makes a non-monotone rigor family an error, RUNBOOK 7b says the lint 'ships in Phase 1 either way and fails on
the violation', bisect.toml:12 still keys `applicable_if = "has_repro"` with `min_rigor = "optional"`, and the removing decision P7-CEIL is marked 'No' for gating Phase 1, 'Awaiting Ryan' [measured].
Step 3c checks Phase-0 exit (c) — universal over §10.5 rows marked 'gates Phase 1' — against S-BIN alone ('one genuine shortfall remains') although design 1789 and 1861 mark S-FACET 'gates Phase 1',
unrun, with no F-number and no ADR re-scope, and §10.2's Enter column requires exit (a)–(d) while its own May-not column and 10.5:1896 presuppose Phase-1 commits with S-BIN unrun [measured]. The
corpus records each mechanism separately; it does not record that nothing schedules the decisions between 7b and a loadable pack or catalog, that Step 3c omits S-FACET, or that §10.2's two columns
contradict each other [measured, known-issues].

**Anchors.**
- `RUNBOOK.md:2193` — "# exit 0 only when EVERY facet declared source = "derived" names a `deriver` AND a scored precision"
- `references/2026-09-08-acceptance-criteria.md:1157` — "**So this criterion fails load on all eleven, not on the six its evidence line"
- `references/2026-09-08-architecture-design.md:1789` — "where **S-FACET** is a new register row marked "gates Phase 1""
- `references/2026-09-08-acceptance-criteria.md:1162` — "registry change that moves a deterministic projection out of `source = "derived"` altogether, never a quiet narrowing"
- `references/2026-09-08-architecture-design.md:1790` — "| **1 — kernel, store, fix and story lanes on a fresh repo** | Phase 0 exit (a)–(d) |"
- `references/2026-09-08-architecture-design.md:1861` — "`facets.toml`'s `source`/`deriver` fields for `auth`/`surfaces`, `data_migration`, `ui` and `flag_exists` (the selection crate, `rk-kernel`)"

**Why it matters before Step 5.** Steps 5–6 are unaffected and the dependency lands at 7b/7d/8, which is why this is P1 for the pack-rows-over-derived-facets workstream and not P0 [inferred]. Both
clearing decisions are $0 and must precede 7b, or the implementer loosens AC-520 (the collapse AC-071's residual-risk sentence warns of) or takes an unrecorded registry decision; and Step 5's banner
requires Step 3c 'answered', which — answered against S-BIN alone — passes the Phase-1 entry check with a gating spike neither run nor re-scoped. Deciding now whether S-FACET runs in Phase 0, or
whether AC-520 exempts the policy-table derivers, changes what 4a.1 writes into facets.toml and what 7b's lint fixtures assert.

**Remediation.** Before Step 7b, and preferably before 4a: (a) a successor record to ADR-0089 (never an edit; 0090/0091 are claimed, so 0092 or later) moving `stage`, `rigor`, `role` and `cost` out of
`source = "derived"` into a new closed `source` value (e.g. `"kernel"`) that AC-072 recognises by name and AC-520 does not gate; keep `recent_deploy` derived with
`deriver = R-recent-deploy-feed-absent` and a record that says NOT MEASURABLE; `surfaces` is either re-classed or S-FACET is scheduled as a named Phase-0 step with AC-520 and the pack rows keyed on it
as consumer (ADR-0012). (b) Amend RUNBOOK 4a.1/Checkpoint 4a so the derived count after the edit equals the scored count, not eleven ('both are 11' becomes the reconciled-by-name count); amend
AC-520's domain note and Step 8 (6, second lint) to state which branch passes and which facets remain `derived`, the way (6) does. (c) Take P7-CEIL before 7b as a recorded catalog decision, or state
in 7b which of the two repairs the Phase-1 catalog ships with pending it; correct §10.5's gating cell to 'YES for the catalog as shipped'. (d) In the same record, or a further successor (never edit
ADR-0008 or ADR-0089), scope S-BIN's gate to the ChangeSummary type exactly as ADR-0008's '(type design)' wording has it; add an S-FACET row to Step 3c's exit-(c) table stating which branch it takes
(ADR-0089 as the explicit re-scope, also said in design §10.2's Phase-0 row; or a second $0 shortfall run in Phase 0) and a Checkpoint 3c grep for S-FACET beside the S-BIN one at 906; amend §10.2's
Phase-1 Enter column to '(a), (b), (d) and the non-S-BIN half of (c)' or the equivalent.

**Folds.** L03-01, L03-11, L03-07, G05-06, slice-G01-02, L08-02, slice-G03-01

### PIA-002 — The corpus Steps 6-7 test against is neither moved by Step 2 nor reachable from the pinned SHA: the 1,312-file corpus and 360-commit history live only in D:/repos/redkiln, F12's 74 slices need 118 refs (46 reachable from 47faec9f), no CI job fetches either, the rk-kernel golden logs AC-006/AC-188 replay do not exist, and v0's obligation, kill contract and YAML reader are unstated

**Priority:** P1 · **Classification:** SHARPENS-KNOWN ([B01-C06] AC-1.2-013 (register:193: redkiln-corpus does not exist; AC-464 affected); RB-GAP-003; AC-460:5745 and §13 residual risk:5966 (fixture
n unknown); ARCH-0.1-037 (the §0.1 row, not the absent Phase-1 obligation). The CI-provisioning gap, the v0 definition, the kill contract and the YAML row are recorded nowhere | [B08-C01] none for the
ref-reachability defect (known-issues rows ARCH-0.1-037, SM-5.1-019, SM-NOBASE-028/071, ADR-0077-NEG-93 restate 360/74 only). The AC-006/AC-188 half SHARPENS-KNOWN: SM-5.1-006
(success-measures.md:1326 'Nothing in the programme has ever replayed the FSM') and ARCH-3.10-004 (design:421) record that replay has no substrate yet, not that the two criteria name the pinned corpus
as if it held replayable logs.) · **Category:** sequencing · **Confidence:** high · **Owner:** Ryan (RUNBOOK Step 2, AC register §13, dependency policy); Step 7d implementer for the v0 definition and
run-contract AC

Two things are called 'the corpus': design §8.7 (1454) and ADR-0013 place 'the frozen corpus @ 47faec9f' inside redkiln-corpus and AC-464/AC-444 read 'from the pinned SHA in `redkiln-corpus`', but
RUNBOOK Step 2 copies only PINNED_SHA, results, runner, schemas and prompts, while the 1,312-file corpus redkiln-migrate reads and the 360-commit history AC-444 replays live in the predecessor repo at
D:/repos/redkiln, named in CORPUS_PIN by machine-local path (496); no RUNBOOK step, CI job or ADR fetches it, and 5.2 runs `cargo test --workspace` on a hosted runner, so every pinned-corpus fixture
test in §13 (AC-440, 444, 451, 452, 459, 460, 461, 469, 479) and AC-440's rk-store test at Step 6 has no input in CI and nothing marks them a local-only suite [measured]. F12's 360-commit / 74-slice
result was computed by `git log --all` over the local clone's 118 refs (42 heads, 41 remote-tracking, 35 tags); from 47faec9f only 46 of 74 slices (8 ever-rejected, not 15) and 231/348 trailer commits
are reachable, and F12 discloses 'across all refs' while AC-444, AC-464 and design §5.7 restate the fixture as a read from the pinned SHA via the object store — jointly unsatisfiable as written
[measured]. AC-006 and AC-188 verify kernel replay 'over corpus logs' from a pinned corpus that today holds only the old kernel's 124 workflow_run telemetry events (1 of 101 recording the decision
field), which a redkiln-rs kernel cannot replay; the golden logs do not exist and are not marked [WILL EXIST] [measured]. Design §0.1 says 'Yes — `redkiln-migrate` v0 is in Phase 1', yet Checkpoint 7d
(2102-2113) never builds or invokes the tool while 7d's acceptance set lists AC-440–AC-479 whole; a kill between writing the new layout and the one commit leaves a target neither AC-440 nor AC-450
describes (0 hits for dirty/porcelain/interrupt/partial across AC 5470-5760); no YAML reader is named in §9.2's fourteen rows, ADR-0084 or Step 7d though the tool must parse YAML frontmatter, fenced
ledgers and `.redkiln/processes/*.yaml`; and AC-460's 'unknown' fixture n is recoverable from s-mig.jsonl as 43 (18/6/19), its plan-companion sentence conflating `plan.md` scaffolds with `_plan.md`
and leaving two `_plan.md` records under no criterion [measured]. The corpus records that redkiln-corpus does not exist and that replay has no substrate; it does not record the ref-reachability gap,
the CI-provisioning gap, or that two criteria name the pinned corpus as if it held replayable logs [measured, known-issues].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:5784` — "THEN it reads them from the pinned SHA in `redkiln-corpus` via the object store, never from a working tree"
- `references/2026-09-08-acceptance-criteria.md:5530` — "GIVEN the rewritten `reconcile()` and the F12 fixture (360 trailer commits over 74 slices, pinned in `redkiln-corpus`)"
- `RUNBOOK.md:469` — "cp -r D:/repos/redkiln-rs/spikes/corpus   ./corpus"
- `RUNBOOK.md:496` — "REDKILN_REPO=D:/repos/redkiln"
- `spikes/METHOD.md:88` — "**Inputs.** 232 `Story:` and 104 `Slice-Verdict:` trailers across all refs at the"
- `RUNBOOK.md:2118` — "**Acceptance set: AC-400–AC-439** (CLI and `doctor`), **AC-440–AC-479** (`redkiln-migrate` and the clean break),"

**Why it matters before Step 5.** Step 2 (corpus extraction) runs before Step 5 and is the last point at which the 118-ref substrate can be captured cheaply [inferred]. Step 5.2 makes
`cargo test --workspace` CI from the first commit, and from Step 6 onward — AC-440's rk-store fixture, the rk-git replay/rebuild tests (AC-444, AC-165, AC-184, SM-051) and the kernel golden-log replay
(AC-006, AC-188, on which AC-161 and SM-056 are 'only as strong as') — every corpus-bound test is red or silently absent on the hosted runner and nothing fences it. No Step-5 artefact is built wrong,
so P1 not P0; an implementer following AC-464 literally reproduces 8/46 and reads the divergence as a reconcile bug, and one following AC-188 literally builds a CI test with no valid input.

**Remediation.** Amend RUNBOOK Step 2 (not an ADR) to materialise the F12 substrate as a pinnable artefact: a `git bundle create --all` of D:/repos/redkiln carrying all 118 refs, committed into
redkiln-corpus and pinned by that repo's SHA (or a fetchable remote + SHA declared in CORPUS_PIN), plus a 5.2 CI step that fetches it — or mark the corpus-bound tests an explicit local-only suite with
a named runner. Register edits (the register decides nothing new, so no ADR changes): reword AC-444/AC-464/AC-184 to name that artefact and its ref set; re-record F12's 'at the pinned SHA' as 'across
all refs of the frozen clone'; add a ref-set assertion beside design §5.7's 40-hex assertion; reword AC-006/AC-188's Verified-by to 'golden event logs recorded by rk-kernel on the pinned corpus
repository, committed to redkiln-corpus as fixtures [WILL EXIST after Step 5]' and add golden-log generation to the Step 5/Step 6 checkpoints; record AC-460's n=43 (18/6/19) [measured, s-mig.jsonl],
correct its plan-companion sentence and assign the two `_plan.md` records to a criterion. Add to Step 7d a v0 definition (binary exists; AC-441/AC-466/AC-474 structural assertions; report schema
AC-448/AC-468; a dry run over a small fixture), a checkpoint line that invokes `redkiln-migrate`, and move the corpus-bound criteria to an explicit Phase-2 acceptance set. Add one AC to the migrate
area: refuse a target with non-empty `git status --porcelain` before any write and write into a dedicated branch/worktree so an interrupted run leaves the target branch untouched; extend AC-450's
idempotence test with a kill between write and commit. Author a successor to ADR-0084 (never an edit) naming the YAML reader for `redkiln-migrate` only (RFC 209-213 refreshed: `serde_yaml` archived,
`serde_yml` RUSTSEC-2025-0068, `yaml_serde` FINE), with the 0.x exact-pin rule and `cargo audit --deny unmaintained` (or an explicit waiver) decided in the same record; add the row to §9.2 and a
`grep -c` on the pin to Checkpoint 7d.

**Folds.** slice-G20-01, slice-G20-07, L13-operational-07, L13-operational-01, slice-G20-08, slice-G14-01, slice-G14-07

### PIA-003 — The three LLM boolean facets have no disposition between Step 4's pre-registered rule and Checkpoint 4a: ADR-0090 as drafted leaves them `source = "llm"` (AC-071 fails closed), and Checkpoint 4a asserts a zero `[true, false]` count, a `surfaces.toml` that exists nowhere and a Halt-8 reading that Step 4a's own instructions cannot satisfy

**Priority:** P1 · **Classification:** NEW ([B01-C03] none — ARCH-1.2-012/015 and ARCH-10.4-005 concern `has_repro` on the bisect rung; RB-GC-001 records the require-row ban; no row records that the
Step 4.1 rule omits the booleans' source disposition | [B04-C03] RB-WILLEXIST-006 records the two [WILL EXIST] lines and AC-1.1-005 the `[true, false]` mechanism; RB-HALT-008 / RB-WILLEXIST-007 record
Halt 8 and the deferral; census anomaly_pool OH-4 (binary unbuilt) and OH-6 (lint in the same change) are neighbouring reasons the checkpoint cannot pass in canonical order. The count, the
surfaces.toml test and the Halt-8 collision are unrecorded.) · **Category:** blocked-leakage · **Confidence:** high · **Owner:** Ryan (operator) via ADR-0090; RUNBOOK author for 4.1 and Checkpoint
4/4a

AC-071's precondition forbids any `source = "llm"` facet until the lint exits 0, and RUNBOOK 4.1 (950-951) and 4.5 (1394-1395) pre-register the ADR-0090 decision as 'every ordinal axis is
`source = "human"`' — five axes; spikes/p9/facets.toml carries seven `source = "llm"` facets, three of them booleans (`has_repro` 76-78, `external_dep`, `market_facing`), nineteen catalog/pack rows
key on them, 7b's rigor-monotonicity paragraph (1971) presumes `has_repro` remains a live selection input, and ADR-0079:158-165 records a preference for their disposition, not a decision [measured].
Whoever writes ADR-0090 therefore has no rule for the three, Checkpoint 4 (1418-1432) has no registry-count line, and AC-071 fails closed on them at pack load [measured]. Checkpoint 4a then asserts
zero `values = [true, false]` lines where following 4a.1–4a.3 leaves three (eight exist: five derived, three `llm`; ADR-0089 Decision 3 says 'Boolean facets become tri-valued' without a qualifier
while its forbids block at line 215 says 'a derived boolean facet'), tests for a checked-in `surfaces.toml` that exists nowhere in the tree (ls and git ls-files: 0) and that ADR-0089 makes a
per-target-repo artefact deferred to S-FACET, and RUNBOOK:1550's 'Six are commands against files that exist today' counts that line among them [measured]. Halt 8 ('a checkpoint you could not run is a
failed checkpoint', applies at every step, 'none is discretionary') carries no carve-out for [WILL EXIST] lines although the legend (line 14), Checkpoint 4a's header (1550-1551) and 1569 instruct the
deferral, and 1570's expectation 'exit != 0 TODAY' is met by a missing binary — the 'green anyway' shape Halt 8 was written to catch [measured]. No known-issues row records the booleans' missing
source disposition, the count, the surfaces.toml test or the Halt-8 collision [measured].

**Anchors.**
- `RUNBOOK.md:950` — "- **If clause A fails** — which, on the records that already exist, it does, 4 times out of 4 — then **every"
- `RUNBOOK.md:1449` — "of them are the booleans this record re-shapes (`auth`, `data_migration`, `ui`, `flag_exists`, `recent_deploy`;"
- `RUNBOOK.md:951` — "  ordinal axis is `source = "human"` for Phase 1**, supplied or confirmed at the triage gate. That is not a"
- `RUNBOOK.md:1565` — "grep -c 'values = \[true, false\]' "$F"   # → 0: no boolean still declares a two-valued domain (4a.3)"
- `references/2026-09-08-acceptance-criteria.md:170` — "# Until that command exits 0, no facet may be source = "llm"; every ordinal axis is source = "human"."
- `RUNBOOK.md:1559` — "test -f surfaces.toml || test -f .redkiln/surfaces.toml               # the declared boundary map is checked in"

**Why it matters before Step 5.** ADR-0090 is the Step-5 gate token and is written at Step 4.4, before Step 5: written to 4.1's rule alone it leaves three `llm` facets AC-071 fails closed on, or its
author improvises an undocumented disposition; fixed in the pre-registered rule it costs one clause, fixed after ADR-0090 is accepted it needs a successor record [inferred]. Step 4a runs before Step 5
and a failed checkpoint is a stop, so an implementer will either silently re-shape the three LLM booleans (an edit ADR-0089's forbids block scopes to derived booleans) or seed an empty surfaces.toml
in the wrong repository to turn the checkpoint green — the defect golden constraint 7 exists to prevent. The only concern in this section with a pre-Step-5 deadline; not P0 because the fix is a
count/scope correction, not a wrong product.

**Remediation.** Extend RUNBOOK 4.1's pre-registered decision rule with a clause for `has_repro`, `external_dep`, `market_facing` (human-supplied at triage, or dropped per ADR-0079's 'honest
destination' option), carry that clause into ADR-0090 when it is written, and add a Checkpoint 4/4a line `grep -c 'source = "llm"' spikes/p9/facets.toml` -> 0 after the edit. Either extend 4a.3
explicitly to all eight booleans (stating that ADR-0089's tri-valued rule reaches the LLM booleans, or that they are re-sourced to human under Step 4's decision) and keep the zero assertion, or change
it to 3 with the three names. Replace the `test -f surfaces.toml` line with an assertion on the corpus/target path S-FACET will seed, or drop it from Checkpoint 4a and move it to S-FACET's own
checklist; correct 'Six are commands against files that exist today'. Scope Halt 8 by RUNBOOK amendment: 'lines marked [WILL EXIST] are deferred, not failed; record the deferral and re-run them at the
step that builds the path'; move 1570-1571 into Checkpoint 7b as named re-runs (7b has facet_provenance_fixtures at 1991 but not the lint invocation) with the expectation 'exit != 0 AND stderr names
the failing clause' so a missing binary does not pass.

**Folds.** L03-04, slice-G01-03, L08-07

### 3.2 Step 5 — rk-types and the self-hosting target

### PIA-004 — Phase-1 exit (1) self-hosts on a Rust repo with only a Node toolchain adapter; ADR-0011's story-lane half is checked by nothing

**Priority:** P1 · **Classification:** NEW (none for the adapter gap (ADR-0002-FORB-52 / ADR-0011-FORB-46 hold the definitions; SM-NOBASE-091 the three-adapter set). The story-lane half sharpens
ADR-0011-FORB-46 and ADR-0002-FORB-52: the divergence between them and the AC-473-only exit is recorded nowhere) · **Category:** sequencing · **Confidence:** high · **Owner:** Ryan (design + RUNBOOK
author; ADR successor)

AC-473 and RUNBOOK Step 8 (1) require a recorded fix-lane run on D:/repos/redkiln-rs — a Cargo workspace — with every lane stage present (2133-2134: `cd D:/repos/redkiln-rs` then
`run --lane fix --item <a real defect in this repo>`), and the verify and review stages reach the toolchain only through the `Toolchain` port (AC-328/AC-329 make the toolchain adapter the sole
producer of `VerifyReport`; AC-497 routes `classify`/`change_summary` through it); yet design §10.2 scopes Phase 1 to 'SQLite, Claude Code and Node' adapters, Rust appears only under 'Named
alternatives' in ADR-0003:42 and §3.3, the §9.1 crate list has no `rk-adapter-toolchain-rust`, Phases 2–4 add .NET/Python/Unreal, GitHub/mdBook, Codex/Linear/Astro and never Rust, and no technology
pack is mentioned [measured: design, AC, SM, RUNBOOK, 90 ADRs, proposal]. ADR-0011 (accepted) further forbids declaring Phase 1 complete on anything but 'runs its own fix lane end to end
(self-hosting) and Phase 2's stories are authored under it', while design §10.2's exit, Step 8's seven checks and Phase 2's 'Enter when' encode only the fix-lane half (AC-473), ADR-0011's four ACs
(400, 405, 437, 473) contain no story-lane criterion, and `story` appears in the RUNBOOK only in its scope line [measured]. ADR-0002:53 ('solely') is the source of the narrower reading and is in
tension with ADR-0011:47 [measured]. Known-issues holds the definitions and the three-adapter set; the adapter gap and the divergence between the two accepted exit bars are recorded nowhere
[measured].

**Anchors.**
- `RUNBOOK.md:2134` — "./target/debug/redkiln run --lane fix --item <a real defect in this repo>"
- `references/2026-09-08-architecture-design.md:246` — "Phase 1 ships exactly one adapter behind each port it uses — SQLite, Claude Code and Node — and **no tracker and no docs-site adapter at all**"
- `references/adr/0003-five-named-ports-no-postgres.md:42` — "| `Toolchain` | Node (vitest/pnpm) | .NET, Python, Unreal, Rust | fixture-driven |"
- `references/2026-09-08-acceptance-criteria.md:5896` — "THEN it resolves to at least one recorded fix-lane run under Redkiln-RS whose event log contains every lane stage from"
- `references/adr/0011-phase-1-ships-fix-and-story-lanes-only.md:47` — "the exit bar is specifically "runs its own fix lane end to end (self-hosting) and Phase 2's stories are authored under it" (§7)."
- `references/adr/0002-clean-break-self-hosting-switch.md:53` — "it is gated solely on Redkiln-RS running its own fix lane end to end (self-hosting)."

**Why it matters before Step 5.** Step 5 writes `members = ["crates/*"]`, a toolchain pin and CI, none of which changes under either remedy, and Steps 6, 7a, 7b are untouched — so P1, not P0
[inferred]. But the decision changes 7c's adapter set (a fourth adapter plus fake-conformance row and technology pack) and whether Step 8 (1) can be passed at all; ADR-0003 names the Phase-1 adapters,
so silently adding one in code would violate an accepted record. Taking it before Step 5 makes the workspace member list right first time.

**Remediation.** Decide by record, not in code: (a) add `rk-adapter-toolchain-rust` (cargo build/test or nextest, `classify`, `change_summary`) and a Rust technology pack to design §10.2's Phase-1
row, the §9.1 crate table and RUNBOOK 7c/Checkpoint 7c, via a successor to ADR-0003 (AC-499's matrix row; AC-481 unchanged — the fake count is per port); or (b) supersede ADR-0002/ADR-0011's
self-hosting definition with a new ADR naming the target repo of the Phase-1 run. In the same record settle 'solely the fix lane' (ADR-0002:53) against 'fix lane and stories' (ADR-0011:47); if the
latter, add an AC (a release check over the event log, twin of AC-473) for at least one recorded `story`-lane run authoring a Phase-2 story under Redkiln-RS, as RUNBOOK Step 8 check (1b) and in design
§10.2's Phase-1 exit.

**Folds.** slice-G12-01, G05-04

### PIA-005 — rk-types' closed-output schema rules cannot all hold: 'no `oneOf` anywhere' (ADR-0084:64, RUNBOOK 5.3) vs nested-only (AC-131, design 5.3), the prescribed `#[serde(tag = "kind")]` remedy emits `oneOf` in schemars 1.2.2, the 24/16 caps are documented per-request vendor limits counting union-typed parameters (not house rules, not per-type, not branches-at-a-node), and `StepError` is never enumerated (six variants in step.rs, eleven in the superseded proposal)

**Priority:** P1 · **Classification:** SHARPENS-KNOWN ([B06-C02] none -- ADR-0084-FORB-97 (adr/0084:98) records the adjacent principle (do not rely on schemars to enforce closed output); AC-0.3-017
records StepError only on the Schema twin; no row doubts the tag=kind claim or records the missing variant table | [B05-C03] none (ARCH-5.11-010 @ design:791 records 24/16 as house rules; RB-GAP-006
records the grep exit-code shape; no row records the strength conflict, the 'fields' regression at RUNBOOK:1742, or the `anyOf` gap) | [B09a-C01] known-issues EA-5-020 (evidence-augmentation.md:290:
'no limits table on the platform page, which partially truncated on fetch ... re-fetch platform.claude.com structured-outputs if the number must be load-bearing'); ARCH-5.11-010
(architecture-design.md:791)) · **Category:** inaccuracy · **Confidence:** high · **Owner:** ADR-0084 author (Ryan) for the supersession; rk-types author for the StepError table

Step 5.3 writes `rk-types` and its golden-schema snapshot under rules that cannot all hold: ADR-0084:64 asserts the exported schema contains 'no `oneOf` anywhere', :69 prescribes
`#[serde(tag = "kind")]` for data-carrying enums 'so they render as a single discriminated object', design:1742 repeats this tagged [documented, 1P] and Checkpoint 5 greps for zero `oneOf`; but in
schemars_derive 1.2.2 `expr_for_internal_tagged_enum` (schema_exprs.rs:347-374) builds one object per variant and calls `variant_subschemas` whose keyword is `if unique { "oneOf" } else { "anyOf" }`
(:484), so a `tag = "kind"` enum yields oneOf=1, an untagged enum anyOf=1, an externally tagged enum oneOf=1, and the prescribed shape fails the ADR's own assertion and the checkpoint [measured,
schemas generated with schemars =1.2.2; docs.rs fetched 2026-09-10]. AC-131 and design §5.3 row 3 (:665) forbid only a `oneOf`/`anyOf` nested inside another, AC-157 caps 'union branches at any node'
while RUNBOOK:1742 and ADR-0084 keep '≤ 16 union fields per type', and Checkpoint 5 greps `oneOf` only, which `#[serde(untagged)]`'s `anyOf` passes while violating the no-union intent — so under the
stronger rule AC-131's nested-union must-fail fixture and AC-157's 17-branch fixture fail for the wrong reason and the two lints are vacuous [measured]. The 24/16 caps the corpus states in four live
places are its own budget that 'must not be presented as documented vendor limits' are in fact the vendor's 'Schema complexity limits' table — on the page today and in a Wayback capture of 2026-09-07,
so the 'no limits table' sentence was a truncation artefact — and they are totals per REQUEST across all strict tool schemas plus the JSON output schema (20 strict tools per request; 180 s compile
timeout), with the 16 counting 'parameters that use anyOf or type arrays', not union branches at a node [documented, platform.claude.com fetched 2026-09-10]; a schemars Option<T> renders as a type
array, so a type passing AC-132 with 24 optional fields could carry 24 union-typed parameters and breach the vendor's 16 with every corpus lint green, and F20 records that closed-output violations
fail open at exit 0, so the breach would be silent [inferred]. `StepError`, the flagship such type, is asserted closed (AC-229) with one event arm per variant (AC-360) but is never enumerated in the
governing document: design:1649 says its shape is 'carried from step.rs' (six variants: Transport, Schema, Timeout, Cancelled, NonZero{code,stderr}, UsageLimit) while §3.8/§7 (:339, :395) name the
eleven-variant set of the superseded proposal §4.3 (Platform, RateLimited, NotAdopted, NotIsolated, Unsupported, Overage, Schema{attempts}, NonZero(143)), which AC-229:2936 cites [measured].
Known-issues records the adjacent principle (ADR-0084-FORB-97), the 24/16 as house rules (ARCH-5.11-010) and the truncated fetch (EA-5-020); no row records the strength conflict, the `oneOf` emission,
the per-request scope or the missing variant table [measured].

**Anchors.**
- `references/adr/0084-dependency-policy.md:69` — "Data-carrying enums that must cross the boundary are written `#[serde(tag = "kind")]` so they render as a single discriminated object."
- `references/2026-09-08-acceptance-criteria.md:1843` — "THEN no `oneOf`/`anyOf` node contains another `oneOf`/`anyOf` in any descendant."
- `RUNBOOK.md:1742` — "level; `required` ordered first; ≤ 24 optional fields and ≤ 16 union fields per type. **Those two caps are this project's own authoring budget, not a vendor constant** — they trace to an SDK issue's illustrative arithmetic and the vendor's published limitations list no such cap. Do not cite them as "
- `references/adr/0084-dependency-policy.md:64` — "- contains **no `oneOf`** anywhere;"
- `references/2026-09-08-architecture-design.md:666` — "| ≤ 24 optional fields per type; ≤ 16 union branches at any node; required properties before optional ones | **house rules, not vendor caps** | ... the vendor's published limitations contain **no such caps and no ordering requirement** [documented]. AC-157 also discloses that proposal §4.4 says "uni"
- `RUNBOOK.md:1762` — "  && { grep -rn 'oneOf' crates/rk-types/tests/snapshots/ | wc -l; }   # → 0, and the dir MUST exist"

**Why it matters before Step 5.** Step 5.3 builds rk-types and the snapshot test (RUNBOOK:1700 cites AC-129–133/157/158 as what the snapshot enforces): any data-carrying enum written to ADR-0084's
remedy fails Checkpoint 5 with no corpus guidance on which rule gives; implementer A ships AC-131's nesting lint plus a per-node branch count and allows a top-level `oneOf` while implementer B ships
the RUNBOOK grep and a per-type union-field count, and the set of legal Rust enum shapes differs between them; and the lint as written counts branches-at-a-node per type where the vendor counts
union-typed parameters per request, so the rk-types authoring budget and its fixtures (25-optional, 17-branch) would be committed against the wrong constraint with a vendor breach failing open (F20).
The kernel (7b) and adapter (7c) will match exhaustively against whichever StepError set the 5.3 author picks — six copied from step.rs or eleven reconstructed from a superseded document. P1 not P0:
the types Step 5 ships do not change, only the lint's dimension, scope and citation, and RUNBOOK 5.3 treats a changed snapshot as a reviewed schema change.

**Remediation.** One record before Step 5.3 — a successor to ADR-0084's schema clause and evidence line (:64, :69, :117; never an edit): (a) state one rule identically in AC-131, AC-157, design §5.3
and RUNBOOK 5.3 — either adopt AC-131's nesting rule plus the branch cap and change the Checkpoint 5 grep to detect nesting, or keep 'no oneOf anywhere' and state the true constraint (no multi-variant
data-carrying enum in any exported type; flatten to a struct with a unit `kind` enum plus optional payload fields, which spends the ≤24 optional budget); restore 'branches at any node' at RUNBOOK:1742
and extend Checkpoint 5 to `grep -rnE 'oneOf|anyOf'` with the nesting check in the test itself. (b) In the same record or an amendment to the harness area file, one table listing every `StepError`
variant, its payload, its retry class (AC-029/AC-234), whether `spoke()` is true, and its event arm; correct design:1649 to say the `spoke()` predicate and raw-retention pattern carry from step.rs and
the variant set is redesigned; drop design:1742's [documented, 1P] tag for the single-object claim. (c) By dated amendment to RUNBOOK §5.3 (:1740-1744) and design :666/:2009, cite the vendor's 'Schema
complexity limits' table as the source of 24/16, state the per-request scope (all strict tool schemas plus the JSON output schema; 20 strict tools; 180 s compile), and delete 'Do not cite them as a
platform constraint'. (d) Re-specify AC-157 and the snapshot assertion to count union-typed parameters — any property whose schema uses anyOf or a type array, nullable Option<T> included — per
exported schema, retaining the 17-parameter fixture and keeping AC-132 as is; state the precondition under which per-type equals per-request (one output schema per dispatch, no strict user-defined
tools on the command line) as a GIVEN in AC-132/AC-157 and as a lint on the adapter's argv, so that adding a strict tool later re-opens the budget. (e) Register S-SCHEMA-CAP (consumer: the rk-types
snapshot test, AC-132/AC-157, AC-129's fail-open guard): submit a 17-union-parameter schema and a 25-optional schema to the pinned CLI and record the observed failure shape, because F20 says the
failure is silent and nothing in the corpus has observed the vendor's rejection.

**Sources.** https://docs.rs/schemars/latest/schemars/ (fetched 2026-09-10) — front-page enum example renders "oneOf": [ {"type":"object", ...} ]; derive page: 'generate the schema for the internally tagged ... representation' -- no 'single object' wording; 1.2.2 is latest. · https://platform.claude.com/docs/en/build-with-claude/structured-outputs (fetched 2026-09-10) — | Optional parameters | 24 | Total optional parameters across all strict tool schemas and JSON output schemas. | Parameters with union types | 16 | Total parameters that use anyOf or type arrays ... | · https://platform.claude.com/docs/en/build-with-claude/structured-outputs.md (fetched 2026-09-10) — Second-opinion re-fetch of the .md form and a Wayback capture of 2026-09-07 both carry the limits table and the 4x6=24 note; anyOf/allOf supported, oneOf absent; additionalProperties must be false.

**Folds.** G21-02, L06-buildability-06, G19-05, G21-03, EXT-ANTHROPI-002

### PIA-006 — The companion kind list is delegated to a runbook that never mentions it; AC-143 sits unfenced in Steps 5 and 6

**Priority:** P1 · **Classification:** SHARPENS-KNOWN (ARCH-5.11-005 (design:786: 'a runbook decision recorded against ADR-0075 cl. 3 ... until then the criterion cannot be verified' — assumes the
runbook fixes it)) · **Category:** blocked-leakage · **Confidence:** high · **Owner:** Ryan (operator) — a recorded decision against ADR-0075 cl. 3; RUNBOOK author for 5.3/6.3 and the acceptance lines

Design §5.2 row 3, §5.11 row 5 and AC-143's note all say the runbook settles which companion kinds get a sidecar schema; `grep -i companion RUNBOOK.md` returns only the AC-071 positive-companion
fixture (1543-1547), 'kind list', sidecar, `_integration`, `_grounding`, `_decomposition`, AC-143 and ADR-0075 occur nowhere in it, and the RUNBOOK does not count the list among its four decisions
[measured]. The candidate lists disagree three ways: AC-143 names five kinds, ADR-0075 clause 3 four, and the S-MIG census holds eleven `companion.*` kinds over 114 files plus `ledger`; the cited
'19/45 (42%)' denominator is a five-kind subset [measured, s-mig.jsonl]. Step 5 (1774) and Step 6 (1863-1865) claim AC-120–AC-159 wholesale, exempting only AC-154, AC-121's cross-OS half and AC-158,
and the register's residual-risk list (2202) names the same three [measured]. The corpus knows the list is unsettled; it does not know that the place it delegated the decision to is empty.

**Anchors.**
- `references/2026-09-08-architecture-design.md:648` — "**Open:** proposal §4.4, its machine-only list and the S-MIG census name **different companion sets** — the runbook settles the list, and until it does AC-143 cannot be verified against it"
- `references/2026-09-08-architecture-design.md:1898` — "The companion **kind list** — proposal §4.4, its machine-only list and the S-MIG census name different sets — is a runbook decision recorded against ADR-0075 clause 3, and **AC-143** cannot be verified until it is taken (§5.11)."
- `references/2026-09-08-acceptance-criteria.md:2009` — "counted `implementation` and `plan`. Reconcile the kind list in the runbook — this criterion cannot be verified against a"
- `references/2026-09-08-acceptance-criteria.md:2010` — "list that three sources disagree on."
- `RUNBOOK.md:1774` — "**Acceptance set:** typed-artifact and store criteria **AC-120–AC-159**, of which the schema-design rules are"
- `references/2026-09-08-acceptance-criteria.md:2202` — "- Three criteria are unverifiable today and must not be counted as covered: AC-154 (its golden"

**Why it matters before Step 5.** Step 5.3 builds rk-types ('the crate ships every other artifact type') and Step 6.3 the registry AC-143 tests ('a companion kind with no registered schema cannot be
written'); two implementers reading AC-143, ADR-0075 and S-MIG ship different registries, and AC-143's completeness test asserts an undocumented choice [inferred]. rk-types would build an incomplete
thing, not the wrong one — P1, not P0 — but the decision is owed inside the first product step.

**Remediation.** Add a numbered decision to RUNBOOK Step 5.3 (or a DECISIONS line) fixing the Phase-1 companion kind list, with the S-MIG eleven-kind census as the enumerated candidate set and an
explicit keep/drop per kind, recorded against ADR-0075 clause 3 as design 1898 instructs; cite it from AC-143's note; add AC-143 to the 'unverifiable until' list in the register's residual-risk block
(2202) and in the Step 5 and Step 6 acceptance lines until it lands; correct the '19/45' citation in AC-143 and ADR-0075's amendment note to 'five of eleven companion kinds'.

**Folds.** G19-04, L03-10

### PIA-007 — `Findings.repro` as fixed by ADR-0074 and snapshotted at Step 5.3 has no field for the mutant or suite that AC-289/AC-305 re-execute

**Priority:** P1 · **Classification:** NEW (none (queries repro/mutation/suite/mutant/AC-283/AC-281 over known-issues.json return only F31-LIM-005, AC-0.3-033, AC-0.3-021, none on the shape)) ·
**Category:** buildability · **Confidence:** high · **Owner:** Claude (ADR/AC author); Ryan Britton (accepts the schema change)

ADR-0074 clause 1 fixes the finding shape as `{ ac, claim, evidence, rationale, files[], repro { command, observed, expected_when_met } }` and RUNBOOK 5.3 pins `rk-types::Findings` with an insta
golden snapshot before any classifier exists; the shape has no field for the mutant AC-289 must 're-apply', the 'named suite' it re-runs, or the 'mutant id' in AC-305's identity tuple [measured]. The
S-TYPED schema's `command` is described as 'command or code change', and in the measured payloads roughly 24 of 29 `mutation-survived` commands describe the mutation in prose and 111 of 176
blocking-grade `command` values do not begin with an executable [measured, refuter recount]; under AC-290 these become `cannot_run` -> Unverifiable -> advisory, a gate that fails open. Two
implementers will build different things: one extends the schema with a typed mutation object (invalidating the 52/52 conformance evidence and the Step 5.3 snapshot), the other executes
`repro.command` verbatim [inferred]. The finder's further claim that AC-283 bars AC-284's inputs is weakened: AC-283's GIVEN is 'policy computes the block set', so a two-function reading (classifier
separate from `policy::classify`) has no conflict, though no AC states that split [measured].

**Anchors.**
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:56` — "Each item is `{ ac, claim, evidence (advisory label), rationale, files[], repro { command, observed, expected_when_met } }`. Every finding is an executable claim; a finding that cannot be stated as one is not a finding."
- `references/2026-09-08-acceptance-criteria.md:3657` — "WHEN the named mutant is re-applied at the reviewed SHA and the named suite is re-run"
- `spikes/runner/src/e3.rs:175` — ""command": { "type": "string", "description": "The exact command or code change you ran to establish the evidence. Empty string if none." },"
- `references/2026-09-08-acceptance-criteria.md:3596` — "*Verified by:* a signature/type test that no free-text field is an input type to `policy::classify` — this is the"
- `references/2026-09-08-acceptance-criteria.md:3608` — "THEN it is the total decision table and nothing else — `reproduced` iff a command ran and `observed ≠"
- `RUNBOOK.md:1710` — "`rk-types` owns ids; the typed artifacts (`Spec`, `ImplReport`, `Findings`, `DocSet`, digests); schema derivation;"

**Why it matters before Step 5.** Step 5.3 writes `rk-types::Findings` and snapshots it before Step 7b's classifier exists; snapshotted as ADR-0074 clause 1 states it, the shape cannot support
AC-289/AC-305 and the snapshot must be broken. Not P0: RUNBOOK 5.3 treats a changed snapshot as a reviewed schema change and already holds one type (`ChangeSummary`) open where its field set cannot be
fixed, so the rk-kernel re-execution workstream is what is blocked, not the phase.

**Remediation.** Supersede (never edit) ADR-0074 clause 1 with a record that types the repro: `command` as a shell string with an explicit `kind: run | mutate`; a `mutation { diff | patch, suite }`
object required when `kind = mutate`; a machine-readable `expected` (exit code / assertion locator) beside the prose `expected_when_met`. Reconcile AC-283 by naming which fields are free text (claim,
rationale) and which are typed classifier inputs, and state whether the classifier and `policy::classify` are one function or two. Add a negative fixture to AC-281 that a prose-only command is
rejected at ingest. Do this before RUNBOOK 5.3's snapshot is taken.

**Folds.** G15-02

### 3.3 Step 6 — the memo record and the git read path

### PIA-008 — The memo record is specified three ways and its keys are unpinned: ADR-0086's SQLite tier vs AC-161/AC-187; a five- vs seven-component memo key with three component lists and no hash framing; an idempotency key three-part everywhere but the schema; and the per-step `memo = false` rule accepted ADR-0059 withdrew, still demanded by ADR-0021's forbids block, AC-018 and design section 3.5

**Priority:** P1 · **Classification:** SHARPENS-KNOWN ([B04-C01] AC-160 Limit (acceptance-criteria.md:2230-2231), ARCH-5.11-003, AC-0.3-003 (tier judgement unsettled); ADR-0021-FORB-44/45,
F33-LIM-018, AC-15.3-013 (iteration counter, per-0021 key); ADR-0059-NEG-54 (clause withdrawn). None records the AC-161/AC-187 collision, the five-vs-seven mismatch, or the missing frontmatter link. |
[B05-C01] known-issues rows ADR-0059-FORB-60, ADR-0059-NEG-54, ADR-0021-FORB-45 (the supersession at the ADR level); references/adr/0021-loop-aware-memo-key.md:52 (partial-supersession note). No row
records that AC-018 or design:325 still carry the withdrawn rule. | [B03-C11] none (no known-issues row on 'idempotency'; memo rows ADR-0021-FORB-44/45, ADR-0028-NEG-50, SM-5.1-001/009/047,
AC-15.3-013 cover loop-awareness, subgraph hashing and unmeasured hit rates, not key framing or the component count)) · **Category:** contradiction · **Confidence:** high · **Owner:** rk-kernel /
rk-store owner (Steps 6, 7b); ADR maintainer (Ryan)

ADR-0086 places the memoisation table in the EventStore (SQLite, the tier-2 substrate) [measured], while AC-161 requires the next action to survive deleting that file and AC-187 requires a committed
mid-graph step to resolve by memo key rather than re-dispatch; the design's tier-1 list names no step result or memo entry, ADR-0087:46 serialises bound step outputs 'to disk' with no substrate, and
F38's mid-graph state lived under spikes/results/p8-state/, a directory the design never maps, so the tier annotation AC-160 demands at compile time has no answer [measured]. The memo key is five
components at design:322/AC-014/AC-019, a per-step key with an iteration counter at AC-017/ADR-0021:26, a different list again at schema:87, a four-tuple at design:1014 and a three-tuple at AC-336, so
AC-019's 'five components' field set cannot carry what AC-017 requires; no document names the hash framing (length-prefixed vs concatenated), the canonicalisation of `resolved inputs`, what 'toolchain
version' denotes, or the output `schema_version`, so a schema migration (AC-135) hits the memo and fails AC-136's read instead of missing, and `memo_hit`'s journalled field set is undefined
[measured]. AC-010/011/012, AC-508 and design:306/320 key each dispatch by `(run_id, activity_id, attempt)` while activity-graph-schema.md:10 keys it by `(run_id, activity_id, step_id, attempt)`
citing a proposal that states the three-part key; under ADR-0022 each step is its own dispatch with its own `retry = { max = N }`, so AC-011's log-wide uniqueness and AC-012's
one-`activity_committed`-per-`(run_id, activity_id)` cannot both hold for a multi-step activity, and no text says at which granularity `attempt` increments [measured]. ADR-0059 (accepted) moves
liveness to `live = true` on the verb's registry entry and its forbids block bars a step from declaring `memo = false`, yet ADR-0021's forbids block, AC-018 (in Step 7b's AC-001–AC-039 set,
RUNBOOK:2006) and design §3.5:325 still prescribe exactly that per-step declaration — the design records the withdrawal at :324, one row above the row that ignores it — with the partial supersession
living only in 0021's Evidence prose (`superseded_by: []`, 0059 superseding 0043 alone, 0059 listed as `related`) and README:159 counting nine links where its own rule gives ten [measured]. The corpus
records the tier judgement as unsettled and the clause's withdrawal; it does not record that AC-161 and AC-187 cannot both hold under ADR-0086's placement, the key-count or framing defects, or that
AC-018 builds against an accepted ADR's forbids block [measured, known-issues].

**Anchors.**
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:53` — "The memoisation table and replayable log are built in this repo against the `EventStore` port of 0003 (SQLite, WAL, `busy_timeout`, under `<git-common-dir>/redkiln/`), keyed per 0021's content-addressed, loop-aware step key."
- `references/2026-09-08-acceptance-criteria.md:364` — "GIVEN a `tool` step whose verb reads live state (telemetry, flags, clocks)"
- `references/2026-09-08-acceptance-criteria.md:306` — "`(run_id, activity_id, attempt)` is unique across the whole log."
- `references/2026-09-08-acceptance-criteria.md:2235` — "GIVEN a repository whose EventStore database file has been deleted"
- `references/2026-09-08-acceptance-criteria.md:365` — "WHEN the step does not declare `memo = false`"
- `references/2026-09-07-activity-graph-schema.md:10` — "| Every non-deterministic step is its own process with an idempotency key `(run_id, activity_id, step_id, attempt)` and a timeout with a number | proposal §4.2, L5.4/L5.6, H4 |"

**Why it matters before Step 5.** Not Step 5 itself: AC-160 sits in Step 6's acceptance set (RUNBOOK:1863) and the memo lives in rk-kernel at 7b, but the tier decision determines what rk-store's typed
artifacts hold and whether a committed step needs its own checkpoint commit, which shapes the write path from Step 6 onward [inferred]. `Harness::dispatch`'s tuple is the kernel's first type, and
AC-010's trybuild test, AC-019's schema test and SM-001's three-bucket split pin whichever key is chosen first; an unpinned memo key silently changes hit rates between builds, which SM-001 would
misread as a behaviour change. An implementer reading AC-018 builds a per-step `memo` field and a lint that fires when a live verb's step omits it — violating ADR-0059's forbids block — while one
reading ADR-0059 builds a registry flag under which AC-018's WHEN can never be true and the lint is vacuous.

**Remediation.** Decide and record (a DECISIONS line, then a successor to ADR-0086 — never an edit) one of: (i) a committed step result is tier 1, rendered into the store's typed artifacts and
checkpointed (one commit per committed step, or a per-run sidecar committed at gate/exit) with the SQLite memo index a rebuildable projection, so AC-161 holds; or (ii) AC-161/AC-164 narrow to the
stage-level next action and AC-187 scopes to a run whose EventStore survives; add the chosen substrate to §5.4's tier table and AC-160's fixture. An amendment record to ADR-0021 (not an edit): SHA-256
over a length-prefixed ordered tuple; `resolved inputs` canonicalised by rk-store's canonical serialiser (sorted keys, fixed float format); `toolchain version` = the Toolchain adapter's detected
version string plus `TESTED_CLI_VERSION`; output `schema_version`, step id and the iteration counter as named components; AC-014/AC-019 enumerate that list (activity id@version, rigor variant, step
id, resolved input digests, card digest or toolchain version+command, resolved KB atoms, iteration counter) with one property case per component and `memo_hit` journals exactly it, the activity-level
key (AC-016) a separately derived value, and AC-336's evidence key either named as a distinct key or reconciled to the one list; let design §3.5 say the activity-level key and the step key are
different formulas. Amend AC-010..AC-012 and design §3.5 to the four-part key (or define `activity_id` as a step-activation id and say so) and state that `attempt` increments and `activity_committed`
(AC-012) is emitted per step activation under ADR-0022. Supersede, never edit: a new record adding 0021's `memo = false` clause to 0059's supersession scope, `superseded_by` set on 0021, `supersedes`
extended on 0059, 0021/0043 added to README's two-homes list and the supersession table recounted to ten. Amend AC-018 to GIVEN a `tool` step whose verb's registry entry carries `live = true` / WHEN
the step is memoised (or declares `memo = true`) / THEN pack validation fails; add ADR-0059 to its decisions line; correct design §3.5:325 to match :324; add AC-018 to ADR-0059's row in the register's
ADR table (AC:6485).

**Folds.** L06-buildability-03, L04-06, L05-overlap-01, G11-01, G11-05, L06-buildability-07, G11-06

### PIA-009 — ADR-0077 keeps linked-worktree common-dir resolution on gix -- the interplay it says gix lacks and the operation S-GIX/a tests -- with no written fallback for a no-go; 'a no-go costs nothing' is false for arm a, and the retained review clone is a gix-worktree-state checkout on the advisory surface the hybrid claims to avoid

**Priority:** P1 · **Classification:** SHARPENS-KNOWN ([B04-C05] ARCH-0.1-008 (design:26 'Git write path - the common-dir interplay'), ARCH-1.3-004, ARCH-3.10-002, ARCH-5.11-002, ARCH-10.4-012,
ARCH-10.6-005 (design:1912) record the common-dir gap and assume the shell-out is its fallback; ARCH-10.3-006 records the shallow clone. None records that the op the arm tests sits on gix's
un-fallen-back read path, or that the clone is a checkout on the RUSTSEC-2025-0001 surface. | [B05-C06] ARCH-0.1-008, ARCH-1.3-004, ARCH-5.11-002, ARCH-10.5-006 (frame the gap as a write-path block;
none records that the hybrid keeps the read operation the gap concerns on gix)) · **Category:** contradiction · **Confidence:** high · **Owner:** Ryan (ADR author); Step 6 implementer

ADR-0077 rejects 'gix for everything' because the `GIT_COMMON_DIR`/`GIT_WORK_TREE` interplay 'isn't implemented' and is 'precisely the mechanism the store design leans on' (:21, :30), then assigns to
gix 'resolving the repository common dir that 0018 requires at startup' (:60) — the read operation that IS that interplay — and design §5.9/6.2 (:743) and RUNBOOK 6.2 repeat the assignment, from which
the hybrid does not fall back [measured]. S-GIX/a asks exactly that question, yet ADR-0077:78, design 5.9 and RUNBOOK 6.1 say the arm 'gates only the narrowing' and 'a no-go costs nothing'; the store
is SQLite, so the arm's registered question ('can a store … be opened and written correctly from a linked worktree', design:752) names an operation gix never performs, while the operation gix does
perform for the shipped hybrid is what a no-go would break, and no fallback for common-dir resolution exists in the forbids block, 6.2 or AC-178 [measured]. Design 5.9's Read row says 'Nothing
contradicts the read-side bet' while its Write row cites the common-dir gap as its basis, and the arm never exercises the ops it could narrow (gix worktree add/remove), so a go licenses nothing the
forbids block names [measured]. Separately, the retained shallow clone by SHA produces a working tree the reviewer runs `repro.command` inside (AC-194) — a checkout by gix-worktree-state, the
sub-crate whose advisory (RUSTSEC-2025-0001) the ADR cites as the surface the hybrid 'happens to avoid' — so consequence line 88 is over-stated for the clone and neither S-GIX arm exercises clone; the
pin is clean at 2026-09-08 (design:750), so the clone half is rationale, not a build defect [measured]. AC-178 (:2403) tests the observable path, not the library, so either mechanism passes;
known-issues frames the gap as a write-path block and assumes the shell-out is its fallback, and no row records that the read op sits on gix's un-fallen-back path [measured].

**Anchors.**
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:60` — "- `rev-parse`, including resolving the repository common dir that 0018 requires at startup;"
- `references/2026-09-08-architecture-design.md:743` — "| **Read** | `gix`, at the exact pin `=0.87.1` | `rev-parse` (including resolving the common dir AC-178 requires at startup), repository and worktree status, **shallow clone by SHA** for review checkouts, **trailer parsing** |"
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:78` — "does the `GIT_COMMON_DIR`/`GIT_WORK_TREE` interplay behave, or is `crate-status.md` still accurate? | `rk-git` worktree lifecycle | **No** — the shell-out fallback is decided. It gates only the *narrowing*. |"
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:21` — "**The common-dir gap.** The store lives at `<git-common-dir>/redkiln/`, one store shared by every worktree (0003, 0018). gitoxide's own `crate-status.md` states that "the delicate interplay between `GIT_COMMON_DIR` and `GIT_WORK_TREE` isn't implemented""
- `RUNBOOK.md:1813` — "A **no-go** costs nothing, because"
- `RUNBOOK.md:1820` — "| **Read** | `gix`, exact pin `= "=0.87.1"` | `rev-parse` (including resolving the common dir AC-178 needs at startup)"

**Why it matters before Step 5.** Not Step 5. Step 6 runs S-GIX/a first (test name `s_gix_a_common_dir_from_linked_worktree`) and then builds rk-git 'to the hybrid, whatever S-GIX says'; on a no-go
the implementer following the RUNBOOK ships gix common-dir resolution from linked worktrees — the AC-178 startup path every worktree's store access depends on — with the spike result in hand saying it
does not work and no written fallback to reach for. Two implementers reading this text build different rk-git startup paths (gix per the §5.9/6.2 table; `git rev-parse --git-common-dir` through the
wrapper per AC-178's and ADR-0018's literal text) and AC-178 accepts both, so nothing is blocked but the no-go branch is not free.

**Remediation.** Successor record to ADR-0077 (never an edit; the RUNBOOK's ADR-0091 slot for the S-GIX result, or the next free number) that: restates S-GIX/a as a READ-path check whose no-go moves
common-dir resolution (`git rev-parse --git-common-dir`) into the exec wrapper and whose go confirms gix for that op; rewords S-GIX/a in §10.5 to the operation gix actually performs ('does gix
discovery from a linked worktree return the main repository's common dir at the pin'); defines separately what, if anything, would narrow 'no gix for worktree add/remove' (gix has no worktree-add API
to test); reclassifies the review clone as a gix WRITE op retained deliberately (typed shallow-by-SHA clone is the benefit), strikes or qualifies the 'narrows away from checkout' consequence, and adds
clone to the ops ADR-0084's audit job and the Windows CI target specifically cover. Amend RUNBOOK 6.1/6.2 to replace 'a no-go costs nothing' with the two outcomes, and change the register row's 'Gates
Phase 1?' for arm a from 'No' to 'Yes for the read-side owner of rev-parse; the shell fallback is decided'.

**Folds.** slice-G06-01, slice-G06-05, G13-01

### 3.4 Steps 7a–7b — the Harness port, pack authoring and the cache profile

### PIA-010 — The Harness port's `dispatch`/`cancel(handle)` contract is unimplementable as written: blocking dispatch, a handle no caller can hold, no slot for the AC-360 record, no `Send`/`Sync` bounds or trait-object shape, a POSIX process-group cancel that never holds on Windows (the lifted runner does `taskkill /T /F`; AC-225's 'exits 143' never obtains), and an S-BIN-blocked `ChangeSummary` placeholder with no stated home

**Priority:** P1 · **Classification:** SHARPENS-KNOWN ([B02-C01] none (ARCH-10.4-030 / ARCH-10.5-031 / ARCH-10.x-T-021 record the Windows handle leak; ARCH-0.1-018 the sync-trait tension; none records
the dispatch/cancel/record shape gap) | [B03-C07] cancel/bounds/crate names: none (ARCH-10.4-030, ARCH-10.x-T-021, ARCH-10.5-031, ADR-0076-FORB-78 concern the Windows handle leak and dropped-future
semantics, not the call shape or caller). ChangeSummary half: SHARPENS AC-1.2-003, ARCH-3.10-001 (design:418) and AC-PROV-037 (AC:6149), which record that S-BIN blocks the field set and AC-497 asserts
only the port shape -- not that the placeholder's home and Phase-1 return type are unstated while Checkpoint 5's grep is scoped to rk-types only | [B02-C02] ADR-0076-FORB-78; ARCH-10.4-030 (handle
leak / explicit-kill rule); references/2026-09-08-acceptance-criteria.md:3537 and :3097 (job-object containment recorded for AC-278/SM-068 only)) · **Category:** buildability · **Confidence:** medium
· **Owner:** rk-ports / rk-adapter-claude author

The design sketches the harness port as a sync `dispatch(ActivitySpec) -> Result<Typed, StepError>` plus `cancel(handle)`, and ADR-0076 requires the handle to be 'minted when the operation started'
while its Decision has the sync method block on the adapter's runtime until the child exits; no caller can therefore hold the handle before `dispatch` returns, no text names who calls cancel (a Ctrl-C
handler, a wall-clock watchdog, a second thread) or from which thread, nothing states whether the five traits carry `Send + Sync` or whether the kernel holds `Box<dyn Port>` or
`Arc<dyn Port + Send + Sync>` though ADR-0076:56 sends fan-out to threads, and `Result<Typed, StepError>` has no slot for the usage/headroom/effective-config record AC-360 requires on every
termination path, whereas the runner the adapter lifts returns `(StepRecord, Result<..>)` [measured: grep of design, AC register, RUNBOOK and ADR-0076]. `ActivitySpec`, `Typed` and the handle type are
never given fields, `Typed` cannot be a per-call generic on an object-safe trait under AC-480 [inferred], the spike runner carries only the timeout kill (runner.rs:356-359) and its `Cancelled` variant
is `dead_code` (step.rs:190) [measured]. AC-224, ADR-0076's Decision and design §3.8.2 specify cancel as a POSIX 'process-group kill'; the corpus knows Windows needs a Job Object (SM-068:528; AC-278
:3097; :3537) but says so only for AC-278/SM-068, and the runner §9.1 says 'survives unchanged' does neither — on Windows `taskkill /PID <pid> /T /F` (a parent-tree walk that misses re-parented
grandchildren, the F22 survivor shape), elsewhere `child.kill()` reaching only the direct child (runner.rs:469-481) — while AC-225's 'exits 143' is SIGTERM semantics that never obtains on Windows,
where a `taskkill /F` victim exits 1 [measured, this audit, Windows 11]; a Job Object with KILL_ON_JOB_CLOSE needs FFI under the workspace-wide `unsafe_code = "deny"`, and ADR-0083:68 licenses the one
commented override but nothing names it for this crate. Design:269 gives the Toolchain port `change_summary(range) -> ChangeSummary` and AC-497, inside 7a's AC-480..AC-519 set, asserts 'a typed
`ChangeSummary` value', while Checkpoint 5 greps only `crates/rk-types/src` for the S-BIN-blocked name, Checkpoint 8 forbids a finalised type and RUNBOOK 7a never mentions the method, so the
placeholder's home and Phase-1 return type are unstated and both fences are satisfiable by a placeholder anywhere else; Checkpoint 7c builds `rk-adapter-node`, a name occurring once in the corpus
against §9.1's `rk-adapter-toolchain-node`, and the AC register's owner labels include eight names that are not §9.1 crate families with no sentence saying the label is a subject tag [measured].
Known-issues records the Windows handle leak and the sync-trait tension; no row records the dispatch/cancel/record shape gap, the caller, the bounds, or the placeholder's home [measured].

**Anchors.**
- `references/2026-09-08-architecture-design.md:266` — "`dispatch(ActivitySpec) -> Result<Typed, StepError>`; capability registry with dated probes; cost/usage; **`cancel(handle)`**"
- `references/2026-09-08-acceptance-criteria.md:2892` — "THEN the whole process group is killed and no descendant of the dispatch survives the call."
- `references/adr/0076-port-traits-stay-sync-async-confined-to-adapters.md:54` — "Any port whose operations can be long-running exposes a cancel path that takes a handle minted when the operation started, and the adapter implements it as a real kill"
- `references/2026-09-08-success-measures.md:528` — "- **Instrument:** descendant PID count in the dispatch's process group (on Windows, its job object), sampled after cancel."
- `spikes/runner/src/runner.rs:206` — "pub async fn run(&self, spec: &StepSpec) -> (StepRecord, Result<serde_json::Value, StepError>) {"
- `references/2026-09-08-acceptance-criteria.md:5987` — "AND every port is held as a single object-safe trait object."

**Why it matters before Step 5.** Step 7a (`rk-ports`, RUNBOOK:1875) declares this trait and fixes the signatures every fake, adapter, AC-224's fixture and the composition root are written against;
two engineers would write incompatible traits (start/wait vs blocking dispatch; record-in-Ok vs adapter-appends-to-EventStore), and retrofitting bounds or a return type later ripples through fakes and
adapters — the cost ADR-0076 cites against generic ports [inferred]. Step 5 is touched only through the `StepError`/`ActivitySpec` field sets and the workspace member list, so P1 not P0; 7c's sign-off
wording is blocked because the integration test that 'asserts no live PID in the group' is unwritable until the containment primitive per OS is named.

**Remediation.** Before Step 7a, one successor record to ADR-0076 (never an edit) stating: the trait as
`dispatch(&ActivitySpec, &mut dyn EventStore) -> DispatchOutcome { record: DispatchRecord, result: Result<Typed, StepError> }` or a `start() -> Handle` / `wait(Handle)` pair; a `CancelToken` cloned
from the adapter before the call, owned by the composition root's signal/Ctrl-C handler and by the per-activity timeout; `Typed` as an erased validated value (`serde_json::Value` plus schema id) the
kernel deserialises; the field sets of `ActivitySpec` (from `StepSpec`) and the record (from `StepRecord`); and per-OS containment — Windows = Job Object created before spawn with
JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, child assigned before it can fork, handle held by the adapter so parent death reaps the tree; POSIX = `CommandExt::process_group(0)` plus `killpg` — naming the
commented `unsafe` override (or a safe wrapper crate) for `rk-adapter-claude` per ADR-0083:68. Carry the AC-224/AC-225/AC-360 instruments across: rewrite AC-225's WHEN as 'terminated by the adapter's
own cancel' with the classifier keyed on the cancel token, not the exit code; enumerate the group via the job/pgid in AC-224's test; correct §9.1's 'survives unchanged' for runner.rs's kill path.
Dated amendment to design §3.3 adding port rule P10: `dispatch` blocks to completion; the adapter's declared timeout (AC-223) is the in-band cancel; `cancel` is called by the composition root or the
CLI signal path, never the kernel; which ports carry it (Harness only, or Harness+Toolchain+DocsSite with an AC each); ports held as `Arc<dyn Port + Send + Sync>` with the supertraits declared on the
five traits — mirrored in RUNBOOK 7a with 'the Toolchain port's `ChangeSummary` is an opaque, non-finalised newtype in rk-ports until S-BIN', widening Checkpoint 5's grep to `crates/` minus that one
site. Make §9.1 the crate-name authority, change RUNBOOK 2056 to `rk-adapter-toolchain-node`, and add one sentence to the AC register's section 0 stating that the `rk-*` label after each id is a
subject tag, with the mapping to §9.1 crates.

**Folds.** L06-buildability-01, slice-G12-03, slice-G12-11, slice-G12-10, L03-12, slice-G03-05, L06-buildability-02

### PIA-011 — Pack authoring and `.redkiln/` have no owning step; Steps 4a and 8 cite a 'Step 7b pack authoring' that does not exist

**Priority:** P1 · **Classification:** SHARPENS-KNOWN (ADR-0079-NEG-178 (adr/0079:179, pack authoring 'stalled on a human act'); RB-WILLEXIST-009/010 (`.redkiln/` absent); ARCH-0.1-003 (stale_policy
BLOCKED, no ADR); ARCH-0.1-002 (the ADR-0089 gate, not its effect on the v0 files)) · **Category:** blocked-leakage · **Confidence:** high · **Owner:** RUNBOOK author (Claude); Ryan (operator) for the
`stale_policy` and row-disposition decisions

Design §10.2 lists `bug-standard` and `story` packs as Phase-1 deliverables, Step 8 exit (1) runs the fix lane against one and (6) lints `.redkiln/facets.toml`, and RUNBOOK 1444 and 2170 attribute
that file to 'Step 7b's pack authoring' — but Step 7b (1905-2008) carries no instruction to author a pack, create `.redkiln/` or derive the registry, Checkpoint 7b (1980-1997) has no pack line, and
phase1-deps records `.redkiln/facets.toml` as will_exist_unwritten [measured]. The unowned work is substantive: 72 `when` rows across 12 packs keyed on LLM-emitted facets that ADR-0079/AC-071 block;
12 `when` rows plus 3 `applicable_if` keyed on `auth`, which Step 4a.2 withdraws so those files fail AC-108 at load; the retry-envelope constants design §3.10 assigns to 'pack authoring'; and a
per-class `stale_policy` the RUNBOOK says 'needs a decision, not a default' (138) yet omits from its four-decision table (17-25) and from Steps 5–8 [measured, grep over spikes/p9; the word occurs
nowhere in 1590-2242]. Nothing in RUNBOOK 4a/7b, ADR-0089, DECISIONS-2026-09-09, HANDOFF or design §4.9 plans the ADR-0089 rewrite or a lattice re-run, so the F35 readings (0.3%, 6/102) behind row
53's DECIDED cell and the AC-101/AC-102/SM-026/SM-027 baselines describe a pack set that cannot load after Step 4a [measured]. The corpus records pack authoring as blocked and on the critical path; it
does not record that the RUNBOOK schedules no step for it.

**Anchors.**
- `RUNBOOK.md:1444` — "The registry that exists today is `spikes/p9/facets.toml`; Step 7b's pack authoring derives `.redkiln/facets.toml`"
- `RUNBOOK.md:2170` — "# pack authoring derives .redkiln/facets.toml from. Both the binary and the file are Phase-1 output."
- `references/adr/0079-context-vector-source-undecided-phase-1-blocked-on-p1.md:74` — "**1. Phase 1 pack authoring over the five ordinal axes and the three boolean risk facets is BLOCKED.** No `bug-standard`"
- `spikes/p9/packs/bug-standard.toml:68` — "when = "auth""
- `RUNBOOK.md:138` — "contraindicated has absorbed half the finding. **The design consequence is OPEN, and it needs a decision, not a"
- `references/2026-09-08-architecture-design.md:1790` — "`bug-standard` and `story` process packs only (ADR-0011)"

**Why it matters before Step 5.** Step 5 builds the right thing regardless, but the runbook cannot reach its own exit as written: Step 8 (1) and (6) consume work no step owns, and the implementer
reaching 7b finds a derivation two other steps cite and none specifies [inferred]. Left as is, pack content and a `stale_policy` value get set silently — the default design §0.1 (row A03) forbids. A
step and a decision-table row cost one edit now and a successor record later.

**Remediation.** Add a RUNBOOK step (7b.2 or 7e, 'Pack authoring') that: creates `.redkiln/`; derives `.redkiln/facets.toml` from spikes/p9/facets.toml after the 4a edits; carries a row-by-row
disposition table for the 72 LLM-keyed and 15 `auth`-keyed rows and every `ui`/`flag_exists`/`data_migration` predicate (surfaces may be authored, never key a `require` row); re-runs lattice.py and
records the new readings as the AC-101/AC-102/SM-026/SM-027 baselines with the pack count, noting in design §4.6/§4.9 that 0.3% and 6/102 predate ADR-0089; declares the retry envelope in the pack; and
takes `stale_policy` per class as a fifth header-table decision recorded as a DECISIONS line (or ADR), never as a code default. Checkpoint: `test -f .redkiln/facets.toml`;
`grep -c '^source = "llm"' .redkiln/facets.toml` -> 0; pack load exit 0 under AC-071/AC-520/AC-072; and a Checkpoint 8 bullet 'no `stale_policy` value was defaulted from P2; each class's setting is
recorded with its warrant'. Attribute RB-WILLEXIST-009/010 to that step.

**Folds.** L03-03, L08-01, slice-G18-03, L03-06

### PIA-012 — cache_profile = default names a profile with no delivery mechanism; AC-211 makes the static --system-prompt route repository-wide

**Priority:** P1 · **Classification:** NEW (none (16 known-issue rows on cache_profile/AC-211/AC-383 concern the BLOCKED append route S-H1c or the two-legal-values rule, not the default profile's
delivery)) · **Category:** contract-ambiguity · **Confidence:** medium · **Owner:** Claude (AC register + design section 7.1); Ryan chooses the reading

AC-383 requires every card to declare `cache_profile` as `static` or `default`, and AC-384's lint and SM-113's kill move post-gate activities to `default` on the promise of F28's '18,603 flat under
the default prompt' — the arm that ran with no static card [measured]. AC-211 makes the static `--system-prompt` route a repository-level default with no per-card exception, and no AC, design or
RUNBOOK sentence states the adapter's argv for a `default`-profile card [measured: grep of the AC register, RUNBOOK and design finds no delivery sentence]; design:1069 names the time-axis mitigation
as 'The default prompt (no static card)', which implies the card is not delivered by `--system-prompt`. An implementer building the command builder to AC-211 literally ships every card static and
makes `cache_profile` a lint-only label; one building to design §7.1 omits `--system-prompt` for `default` cards; the two readings produce different argv and different measured cost for every
post-gate activity, and AC-384's promised 18,603 never materialises under the first [inferred]. Known-issues' sixteen rows on cache_profile/AC-211/AC-383 concern the BLOCKED append route S-H1c or the
two-legal-values rule, not the default profile's delivery [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4746` — "THEN it declares a `cache_profile` naming which axis it is optimised against — `static` (commit and worktree churn) or"
- `references/2026-09-08-acceptance-criteria.md:4747` — "`default` (time and human gates)"
- `references/2026-09-08-acceptance-criteria.md:2760` — "GIVEN a repository configuration that does not explicitly select the append route"
- `references/2026-09-08-acceptance-criteria.md:2762` — "THEN the static `--system-prompt` route is used and `--append-system-prompt` /"
- `references/2026-09-08-architecture-design.md:1069` — "| **Time** — idle gaps, human gates, resumes | The default prompt (no static card) |"
- `references/2026-09-08-success-measures.md:847` — "- **Kill:** a `static`-profile card whose share falls below 0.5 across commits falsifies ADR-0016 for that card and moves it to the `default` profile."

**Why it matters before Step 5.** Not Step 5. It blocks Step 7c's command builder and the 7b pack lint: neither can be written to a contract that names a profile without a mechanism, and AC-211 and
AC-383 currently contradict each other on the scope (repository vs card) of the route choice.

**Remediation.** One sentence in AC-383 and a matching dated paragraph in design §7.1: under `default` the adapter omits `--system-prompt` and prepends the card body to the user message (F28's
measured arm), with a golden argv test per profile; amend AC-211's GIVEN from 'a repository configuration' to 'a card whose profile is `static`'. If the intended reading is the lint-only label, say so
in the same place and strike the '18,603 flat' promise from AC-384's lint message and SM-113's kill. Either way the choice is written before 7c; ADR-0016 and ADR-0078 are untouched because neither
states the default-profile argv.

**Folds.** slice-G07-04

### 3.5 Step 7c — the Claude adapter and the review gate

### PIA-013 — ADR-0088 misnames its source paper and inverts its mechanism: under bug-free framing false positives collapse, they do not 'barely move'

**Priority:** P1 · **Classification:** SHARPENS-KNOWN (references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:88 'the per-condition breakdown was not read this pass';
references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:78 'an external preprint that this pass did not fetch in full') · **Category:** misread-source / decision rationale contradicted ·
**Confidence:** high · **Owner:** Ryan (ADR-0088 supersession; S-SUBTLE prior-approval arm pre-registration)

ADR-0088 adds a 'confirmation-bias arm' to S-SUBTLE on the authority of arXiv 2603.18740, which it cites as 'Measuring and Exploiting Confirmation Bias in LLM-Assisted Security Code Review' and
characterises as finding that bug-free framing cuts detection 'by 16-93%' with an effect 'strongly asymmetric: false negatives rise sharply while false positives barely move'; the corpus records that
the full paper was not read. Read in full on 2026-09-10, the title is 'Contextual Bias' and the phrase 'confirmation bias' never appears; the 16-93 band is percentage points, under the STRONG bug-free
condition only, on 5 of 6 models, with the omitted sixth (Claude Opus 4.5) dropping 4.9 pp, not significant [documented]. The asymmetry runs the other way: under strong bug-free framing the 'Fixed'
rate on patched code rises +46 to +95 pp, so false positives collapse alongside detection, and it is bug-PRESENT framing that barely moves anything because false-positive rates are already above 88%
at neutral — a ceiling [documented]. The decision (add a prior-approval arm, report FN separately) survives because detection does drop, but the symmetric 'presented as already rejected' arm was
rejected partly on 'the source finds false positives barely move under framing manipulation', which the source contradicts, and the arm's name, the ADR title and the design's §10.4 register all
inherit the misnomer [documented].

**Anchors.**
- `references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:19` — "framing a change as "bug-free" before review reduces the vulnerability-detection rate by 16–93%**, and that the effect is **strongly asymmetric: false negatives rise sharply while false positives barely move**."
- `references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:46` — "Rejected on the measured asymmetry and on cost: the source finds false positives barely move under framing manipulation, the over-rejection direction is already covered by Jin & Chen and answered by the re-execution rule"
- `references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:88` — "*Measuring and Exploiting Confirmation Bias in LLM-Assisted Security Code Review* (~Mar 2026, arXiv 2603.18740): a controlled experiment over 250 CVE vulnerability–patch pairs finds that framing a change as "bug-free" before review reduces the vulnerability-detection rate by **16–93%**"
- `references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:3` — "title: Negative-control fixtures gain a confirmation-bias arm"
- `references/2026-09-08-architecture-design.md:1832` — "framing a change as already reviewed and approved reduces vulnerability detection by **16–93%**, strongly asymmetric toward false negatives [preprint, arXiv 2603.18740]"

**Why it matters before Step 5.** The arm is registered and unrun with a Phase-1 consumer (D4's negative-control fixture design and the review card's framing rules, ADR-0088 :60). A pre-registration
written from the ADR as it stands would predict 'FN up, FP flat' and collect FN only; the source predicts FN up AND FP down, so the arm would be registered against a wrong prediction and would not
collect the datum (CLEAN-pair approval rate under prior-approval framing) that distinguishes the two. Step 5 itself is unaffected; the review-gate workstream (Step 7) and the arm's pre-registration
are.

**Remediation.** Supersede ADR-0088 (never edit) with a record that: (1) cites the paper by its real title and renames the arm the 'prior-approval framing arm' (or 'contextual-bias arm'); (2) quotes
the effect as '16-93 pp under strong bug-free framing on 5 of 6 models; Opus 4.5 -4.9 pp n.s.; weak framing 0.8-24.7 pp'; (3) states the mechanism as a whole-classifier shift toward clean (FN up, FP
down) and requires the arm to report the CLEAN-pair approval rate alongside detection; (4) re-argues the rejected symmetric arm on cost alone, dropping the 'FP barely move' ground, or registers it as
a named follow-on spike per ADR-0012. Correct the §10.4 register row in the design's errata. Predictions go into the arm's pre-registration before any draw, per METHOD. Owner: review-gate workstream.

**Sources.** https://arxiv.org/abs/2603.18740 (fetched 2026-09-10) — Title: 'Measuring and Exploiting Contextual Bias in LLM-Assisted Security Code Review' (v1 19 Mar 2026; v2 23 Apr 2026). · https://arxiv.org/html/2603.18740 (fetched 2026-09-10) — Table 2, Strong Bug-free, Vuln./Fixed: GPT-4o-mini 3.6% (-93.5) / 98.0% (+94.8); Haiku 3.5 53.8% (-42.9) / 50.8% (+46.0); Opus -4.9pp n.s.; 'very high false positive rates (>88%) even under neutral'.

**Folds.** BP-EVAL-05, BP-EVAL-06, BP-EVAL-07

### PIA-014 — rk-adapter-claude: subscription exhaustion mid-dispatch has no error class and backs off; the assembled argv is unbounded

**Priority:** P1 · **Classification:** SHARPENS-KNOWN (ARCH-10.5-023 and ARCH-10.4-035 (design:1850: error taxonomy under load unobserved, S-RETRY unrun) — not that exhaustion-on-result has no
classification rule. The argv bound is NEW (ARCH-3.10-003, ARCH-10.3-004, ARCH-10.4-007, AC-0.3-014 concern the append route and `--bare`)) · **Category:** blocked-leakage · **Confidence:** high ·
**Owner:** Step 7c implementer (rk-adapter-claude); S-RETRY owner; AC register author for the new criterion

The design handles headroom before spawn (AC-220) and derives `RateLimited { resets_at }` from `rate_limit_event` (AC-030/AC-222), but a window that runs out while an 18-minute review is in flight
surfaces as a `result` carrying `is_error`/text, which AC-218 (2833) classifies as `Platform` and §3.6 (338) retries with 2 s-to-5 min backoff up to the pack's N — against a reset hours away
[measured]. The charter (96) records 'no documented exit code or structured error subtype' for exhaustion; the spike runner's `detect_usage_limit` (runner.rs:486, four text markers, non-retryable at
step.rs:271) exists for that reason and 7c lifts runner.rs 'unchanged'; yet no design `StepError` variant, AC or S-RETRY arm names the exhaustion-on-result surface, and AC-229's closed enum has no arm
for the failure ADR-0010's subscription-only rule makes primary [measured: 0 hits for usage limit / plan limit / UsageLimit across design, AC and ADR sets]. In the same adapter, the Phase-1 default
puts the card body on argv (`--system-prompt`, design 273 / AC-211) and the lifted runner also passes the whole agent card (`--agents`, runner.rs:172) and the output schema (`--json-schema`,
runner.rs:184) on argv — all sharing the ~32 KB command-line cap that §9.6 (1755) gives as the reason the prompt goes on stdin (the review card alone is 6.6 KB); no criterion, lint or builder
assertion bounds the assembled argv, and `--system-prompt-file` appears only in a research note [measured]. No measurement shows a current combination near the cap; this is a missing guard, not an
observed failure.

**Anchors.**
- `references/2026-08-31-rust-inversion-spike-charter.md:96` — "There is **no documented exit code or structured error subtype** for usage-limit exhaustion; `resets_at` from `rate_limits` is the practical retry-after."
- `spikes/runner/src/runner.rs:486` — "fn detect_usage_limit(stderr: &str, result: Option<&serde_json::Value>) -> Option<Option<String>> {"
- `references/2026-09-08-acceptance-criteria.md:2833` — "THEN the dispatch resolves to `StepError::Platform { status, retry_after }` — the exit code is not consulted to overturn it."
- `references/2026-09-08-architecture-design.md:338` — "`Transport` / `Platform` 5xx / `Timeout`: exponential backoff with full jitter, base 2 s, cap 5 min, ≤ the pack's `N`. `RateLimited { resets_at }`: **no sleep** — write a `retry_at(resets_at)` mark and exit 0."
- `references/2026-09-08-architecture-design.md:1755` — "| **Prompt on stdin, never argv** | A Windows command line caps near 32 KB; the `redkiln-review` card alone is 6.6 KB and the fix-lane prompt bodies are larger. Argv works in testing and fails on a real review | `rk-adapter-claude`"
- `spikes/runner/src/runner.rs:172` — "            args.push(card.to_agents_json());"

**Why it matters before Step 5.** Lands at 7c, not Step 5 [inferred]. rk-adapter-claude cannot be finished to spec without an unrecorded decision: either the variant is decided before the adapter is
built, or the attempts budget is silently spent on immediate re-failures and the text detector the design's no-prose-parsing principles do not licence ships by default. The argv guard is a builder
assertion whose absence fails at the first real review — the failure mode the stdin rule was written to prevent.

**Remediation.** Add an AC in the harness area (AC-200..239 has free ids): a child terminating with a usage-limit result resolves to `RateLimited { resets_at }`, `resets_at` taken from the last
`rate_limit_event` in the same stream, else the pack delay; name the detection surface explicitly (structured `subtype` first, text markers as a disclosed fallback recorded on the event); carry the
variant into the `StepError` table PIA-005 calls for. Extend S-RETRY's registered question to exhaustion-on-result, not only the throttle margin, with rk-adapter-claude as consumer (ADR-0012). Add an
argv-length assertion in the command builder (refuse before spawn with a typed configuration error; fixture at the cap) and decide before 7c whether large cards use `--system-prompt-file`, noting its
cache-share consequence is unmeasured — an S-H1c-shaped question, to be registered as a spike if it is to be answered.

**Folds.** L13-operational-04, L06-buildability-10

### PIA-015 — AC-212/236/036 require the card to carry model+effort; AC-118/AC-418 make a card that sets either a doctor error: both rows reject every card

**Priority:** P1 · **Classification:** NEW (none - known-issues has 0 hits for AC-212, AC-118, AC-418, 'tier leakage' or 'card property'; anomaly_pool: nothing.) · **Category:** contradiction ·
**Confidence:** high · **Owner:** Claude (register and design edits); Ryan to confirm the binder reading

Two accepted contracts assign model/effort to opposite owners. AC-212 (adapter, ADR-0003) refuses a dispatch whose model/--effort 'cannot be traced to the resolved card' and its red fixtures are 'a
card missing effort' and 'a card missing model'; AC-236 speaks of 'the model than the card declared', AC-036 of 'an activity whose card names a model tier', and design 0.1:72 says 'the pin is a card
property' [measured]. AC-118 and AC-418 (doctor, ADR-0014) make any card or slot filler that sets `model` or `effort` a 'tier leakage' configuration error because 'the tier is owned by the role', and
design:478 lists 'model choice' under what a card never owns [measured]. 'Resolved card' is defined nowhere (0 hits for 'resolved card'/'bound card' in the register, design and RUNBOOK), nothing says
how a role's tier reaches the child's `--agents` JSON, which F7 and the P7 records show carrying model/effort, and line 72 against line 478 has no saving reading; both the adapter (RUNBOOK 7c, builds
to AC-212) and doctor (7d, builds to AC-118/418) are Phase-1 deliverables in phase1-deps [measured]. Known-issues has 0 hits for AC-212, AC-118, AC-418, 'tier leakage' or 'card property' [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:2776` — "*Verified by:* unit tests for (a) a card missing `effort`, (b) a card missing `model`, (c) an env var set while the card"
- `references/2026-09-08-acceptance-criteria.md:5186` — "WHEN any card or slot filler also sets a model or effort"
- `references/2026-09-08-acceptance-criteria.md:5187` — "THEN `doctor` reports tier leakage as a configuration error."
- `references/2026-09-08-acceptance-criteria.md:1609` — "THEN it reports the leaked model and exits non-zero — the tier is owned by the role."
- `references/2026-09-08-architecture-design.md:72` — "Per-activity model/effort declared on the card and refused at configuration time when incoherent (AC-203, AC-212, AC-236, ADR-0003)"
- `references/2026-09-08-architecture-design.md:478` — "orchestration; model choice — the tier lives on the role | AC-118 (a card setting `model` fails `doctor`); ADR-0014 |"

**Why it matters before Step 5.** Not Step 5. Two engineers building the card schema, the binder and the doctor table from this text build incompatible things: one makes model/effort required card
fields, the other forbids them. The likely reconciliation (the authored card never carries tier; the binder injects the role's tier into the dispatch card the adapter emits) is stated nowhere
[inferred].

**Remediation.** Amend AC-212/AC-236/AC-036 to say 'the resolved (bound) card' and define that term once in the AC glossary: the binder materialises the role's tier into the dispatch card; the
authored card file may not carry `model`/`effort` (AC-118/AC-418). Re-word AC-212's fixtures (a)/(b) as 'a bound card whose role declares no tier'. Amend design line 72 to 'the pin is a role property,
materialised per dispatch' so it agrees with line 478 and ADR-0014. No ADR is touched — ADR-0003 and ADR-0014 are compatible once 'resolved card' is defined.

**Folds.** slice-G07-02

### PIA-016 — Tool set unequal to card allowlist is NotAdopted in AC-204 but NotIsolated in AC-202/AC-386/design 7.4; only NotIsolated triggers AC-399

**Priority:** P1 · **Classification:** SHARPENS-KNOWN (AC-0.3-016 (acceptance-criteria.md:79) records the AC-202/AC-386 twin row only; nothing records NotAdopted, NotIsolated, AC-204 or the variant
conflict; anomaly_pool: nothing.) · **Category:** contradiction · **Confidence:** high · **Owner:** Ryan (operator) for the decision; Claude for the register edits

The child's reported `system/init` tool set being unequal to the card's declared allowlist resolves to `StepError::NotAdopted` in AC-204 and design 3.6.1 ('adoption by set equality'), but to
`StepError::NotIsolated` in AC-202 (whose predicate is the 'tool/server list', not servers only), AC-386 and design 7.4:1124 [measured]. AC-399 quarantines a run's metrics only on `NotIsolated`, so a
card-not-applied run (F17's 'generic Claude') is quarantined under one reading and merely failed under the other [measured]. Section 0.3:79 files AC-202/AC-386 as twins and never names AC-204, so the
variant conflict was never raised as a contradiction; AC-386's own Cut note (4796) calls its predicate 'adoption'; no text supplies a discriminator (an MCP-tool vs built-in-tool delta, say) under
which AC-204's superset test and AC-386's extra-tools test both pass; and SM-061 targets set equality but instruments a tool count [measured]. The corpus's own rule (0.3) is that such contradictions
need a human decision before the build runbook runs.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:2706` — "THEN adoption holds only if that set is **equal** to the card's declared tool set, and any difference — extra or missing —"
- `references/2026-09-08-acceptance-criteria.md:2707` — "fails the dispatch as `StepError::NotAdopted`; a superset is not adoption."
- `references/2026-09-08-acceptance-criteria.md:4791` — "THEN the dispatch fails with `StepError::NotIsolated` and is never retried."
- `references/2026-09-08-architecture-design.md:1124` — "`StepError::NotIsolated` on a `system/init` tool set unequal to the card's allowlist, never retried; every metric row from that run marked non-comparable and excluded from every aggregate"
- `references/2026-09-08-acceptance-criteria.md:4802` — "GIVEN a run in which any dispatch failed `StepError::NotIsolated`"
- `references/2026-09-08-acceptance-criteria.md:79` — "| Isolation decided from the child's own `system/init` | **AC-202** | AC-386 | AC-386's companion AC-399 (quarantine the run's metrics) is distinct and is not duplicated. |"

**Why it matters before Step 5.** Not Step 5. The 7c adapter's error classifier cannot be written to spec without a decision: AC-204's and AC-386's unit tests cannot both pass on the same scripted
system/init, and the pick silently decides whether AC-399's quarantine covers adoption failures — that blocks one workstream's acceptance set.

**Remediation.** Add a fourth row (C4) to section 0.3's contradiction table naming AC-204 vs AC-386/7.4 and move AC-386 out of the AC-202 twin row. Decide by a DECISIONS line: either (a) AC-386's
predicate is re-read as the MCP-server list (making it a true AC-202 twin) and AC-399 gains `NotAdopted` as a second trigger, or (b) tool-set inequality is `NotIsolated` everywhere and AC-204 is
amended. Fix SM-061's instrument to a set digest, not a count. No ADR is touched; the ACs are register rows.

**Folds.** slice-G04-01

### PIA-017 — AC-372/376/377 state tick rules in decimal fractions: a naive f64 implementation passes every named vector yet mis-flags 80 of 100 one-tick deltas

**Priority:** P1 · **Classification:** NEW (none (no known-issues row or anomaly-pool entry mentions floating-point representation of utilization)) · **Category:** testability · **Confidence:** high ·
**Owner:** AC register author (harness area); Step 7c implementer

`(i+1)/100 - i/100 <= 0.01` is false for 80 of 100 one-tick bases (0.09 to 0.10 gives 0.010000000000000009), so an implementer who follows AC-372's 'at most one instrument tick (0.01)' on f64 fails to
flag most one-tick deltas; the spike runner's `delta < 0.02` (the F6 formulation, which Step 7c lifts verbatim from throttle.rs) wrongly flags 28 of 99 two-tick deltas (0.10 to 0.12 =
0.01999999999999999); and AC-376's `>= 1.0 - m` misses the exact boundary for 7 of 58 margins (m = 0.18 gives 0.8200000000000001) [measured, IEEE-754 doubles]. Every test vector the criteria name
(0.100/0.110, 0.100/0.130, m = 0.02) sits where f64 happens to work, and no document in the AC register, design, SM register or RUNBOOK mentions f64, fixed point or integer ticks [measured]. The
defect is silent: an unflagged one-tick delta renders as a measurement, and AC-372's 'never rendered bare' doctor rule cannot see a flag that is absent rather than bare; Phase-1 exit criterion (7)
rests on that flag [inferred]. No known-issues row or anomaly-pool entry mentions floating-point representation of utilization [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4616` — "WHEN the absolute change in `utilization` is at most one instrument tick (0.01)"
- `references/2026-09-08-acceptance-criteria.md:4642` — "THEN they are stored verbatim at the resolution reported"
- `spikes/runner/src/throttle.rs:159` — "resolution_limited: delta < 0.02,"
- `RUNBOOK.md:2013` — "call**. `rk-adapter-claude` lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs` from the spike"

**Why it matters before Step 5.** Mis-specified rather than under-specified: the criteria's own instruments certify a defective rk-adapter-claude, including the one Step 7c lifts from the spike
runner. Nothing is blocked, so P1 at the boundary, not P0.

**Remediation.** Amend AC-374 to state that utilization is parsed from the JSON decimal into an integer tick count (hundredths, u16) and that 'verbatim' means the decimal digits, not an f64; restate
AC-372 ('delta_ticks <= 1'), AC-376 ('ticks >= 100 - margin_ticks') and AC-377 ('margin_ticks >= 2') in ticks; add test vectors at bases 0.09/0.10, 0.10/0.12, 0.20/0.21 and margins 0.07/0.18; note in
RUNBOOK Step 7c that throttle.rs's batch_draw is rewritten, not lifted.

**Folds.** slice-G17-03

## 4. P2 — correctness of the record

Eighty-two concerns. P2 is the record being wrong with no build consequence of its own: a number that does not reproduce, a citation whose live source says something else, two corpus lines that
contradict each other, a term a criterion uses and nothing defines. Each concern carries its member findings' anchors and quotes verbatim from the finding records; statements merged from several
findings say so and carry each member statement verbatim. Order within a theme follows the master ordering rule (build stage an implementer reaches first, then breadth).

### 4.1 Contradictions (18)

### PIA-022 — Migrate tool and corpus split: AC-450's no-op re-run cannot report zero failed and AC-447 aborts it; Step 2 omits the E3 fixtures §8.7/AC-465 say move

**Priority:** P2 · **Classification:** NEW — none (SM-5.1-049 / SM-NOBASE-081 record only that no migrated output tree exists; ARCH-1.2-029 records F17's fixture defeat; no row for AC-450/AC-447
idempotence or fixture residency) · **Category:** contradiction · **Confidence:** high · **Owner:** AC register §13 author; RUNBOOK Step 2 author (Claude); Ryan sign-off

(a) AC-450 requires a re-run over an already-migrated tree to be 'a no-op: an empty diff and a report with zero failed and zero with-loss records'. AC-448 defines the report as per-slot lift counts of
the corpus and AC-464 fixes the corpus at the pinned SHA read via the object store, so a re-lift reports the same 54 failed / 73 with-loss [measured, s-mig.jsonl], not zero; 'zero' is reachable only
if the no-op report is a change-diff, which no criterion defines. A no-op yields zero commits, which AC-447 'aborts before writing', and AC-448 places the report inside the commit, so where AC-450's
report lands is unstated. Two implementers build different idempotence semantics for SM-136's hard gate. (b) Design §8.7:1454, ADR-0013 and AC-465's evidence line say the E3 fixtures move to
redkiln-corpus; RUNBOOK Step 2 (:441) moves only corpus/, results/ and runner/. spikes/fixtures/ holds 75 entries, 2.4 GB, 0 tracked files; .gitignore:22–23 calls them throwaway [measured]. They are
derived — the runner's build_fixture_at (e3.rs:289) rebuilds each from the predecessor repo at a named commit and the runner moves — so nothing is lost, but no document records that they are
deliberately regenerated rather than moved, and the design row is inconsistent with the step that executes it.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:5602` — "THEN the run is a no-op: an empty diff and a report with zero failed and zero with-loss records"
- `references/2026-09-08-acceptance-criteria.md:5573` — "AND a run that would produce zero or more than one commit aborts before writing."
- `references/2026-09-08-acceptance-criteria.md:5784` — "THEN it reads them from the pinned SHA in `redkiln-corpus` via the object store, never from a working tree"
- `references/2026-09-08-architecture-design.md:1454` — "| The frozen corpus @ `47faec9f`, the E3 fixtures, the JSONL results, the spike runner | a separate `redkiln-corpus` rep"
- `RUNBOOK.md:441` — "> only `corpus/`, `results/` and `runner/` go to `redkiln-corpus` — but AC-520's scored-record clause points at a"

**Why it matters before Step 5.** (b) affects what redkiln-corpus contains when Step 2 runs (before Step 5); (a) is a contradiction between three criteria of an unbuilt tool (Step 7 §13) with a
one-paragraph fix. No Phase-1 workstream is blocked.

**Remediation.** (a) Amend AC-450 to state the re-run input, define 'no-op' as tree-digest equality with the prior migration commit rather than a zero-failed report, exempt the no-op case from
AC-447's zero-commit abort, and say where a no-op report lands (stdout or a non-committed artifact). (b) Record in RUNBOOK Step 2 and AC-465's evidence line that E3 fixtures are deliberately local,
unversioned and regenerated by the runner's build_fixture_at, and reconcile design §8.7:1454's row to say 'regenerated in redkiln-corpus by the runner', or copy/bundle them if that is preferred.

**Folds.** slice-G20-03, slice-G20-10

### PIA-027 — Omitting `package.rust-version` leaves the edition-2024 MSRV-aware resolver inert; the ADR rejected substitution, not co-existence

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:45-47 records the option '`package.rust-version` floor instead of a toolchain
pin' as rejected; the co-existence effect is not recorded) · **Category:** newer first-party mechanism undermines a stated rationale · **Confidence:** high · **Owner:** Ryan (ADR-0083 supersession;
RUNBOOK Step 5.1)

ADR-0083 pins an exact rustc in rust-toolchain.toml and says 'package.rust-version is not used as a substitute' because 'a floor tells a consumer the minimum version ... Nobody consumes these crates'.
Cargo treats the two as complementary, not alternatives: edition 2024 selects resolver 3, which changes `resolver.incompatible-rust-versions` from allow to fallback, and that fallback 'will prefer
packages with a Rust version that is less than or equal to your own Rust version' only when `rust-version` is set; `cargo add` likewise selects a dependency version compatible with the declared
rust-version. With the pin at 1.97.1 and no rust-version, a dependency whose latest release requires a newer rustc is resolved anyway and breaks the pinned build, which is the CI outage the pin exists
to prevent. Microsoft M-MSRV says an MSRV should be set when crates are first created. The ADR's rejection was aimed at substitution and is correct; the floor set equal to the pin is what makes the
edition-2024 resolver do its job. [documented]

**Anchors.**
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:47` — "Rejected: a floor tells a *consumer* the minimum version that will compile the crate. Nobody consumes these crates. The question here is "which rustc built this binary", and only the pin answers it."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:61` — "`package.rust-version` is not used as a substitute."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:118` — "The "pin for a binary you control" framing is the blog's, corroborated by RFC 3537's separate treatment of the floor — it is not a first-party recommendation for this project shape."
- `RUNBOOK.md:1660` — "A pin and a floor answer different questions and are not substitutes. `package.rust-version` is not a substitute"

**Why it matters before Step 5.** Step 5.1 writes `[workspace.package]` with `edition = "2024"` and no `rust-version`; adding `rust-version = "1.97.1"` (equal to the pin, bumped with it) is one line
now and a resolver-behaviour change later. Not a wrong-thing blocker.

**Remediation.** In the ADR-0083 superseding record (shared with B10-C07 and B10-C09), add `workspace.package.rust-version` set equal to the pinned channel, inherited by every member, and state that
it exists to arm resolver 3's MSRV fallback and `cargo add`, not as a consumer floor; make the pin-bump PR bump both. Add the line to RUNBOOK 5.1's `[workspace.package]` block. Owner: Step 5 workspace
author.

**Sources.**
- https://doc.rust-lang.org/cargo/reference/resolver.html (fetched 2026-09-10) — resolver "3" (edition 2024 default): 'Change the default for resolver.incompatible-rust-versions from allow to fallback ... prefer packages with a Rust version that is less than or equal to your own'
- https://doc.rust-lang.org/cargo/reference/rust-version.html (fetched 2026-09-10) — cargo add auto-selects the dependency's version requirement to be the latest version compatible with your rust-version
- https://microsoft.github.io/rust-guidelines/print.html (fetched 2026-09-10) — M-MSRV: an MSRV should be set when crates are first created

**Folds.** BP-RUST-12

### PIA-028 — Four 0.x crates sit on caret ranges in the §9.2 table while ADR-0084 says 0.x dependencies inherit the exact-pin rule

**Priority:** P2 · **Classification:** NEW — none (references/adr/0084-dependency-policy.md:104 pre-declares the version list as a dated snapshot, which covers currency, not the range rule) ·
**Category:** internal contradiction in a Step-5 input · **Confidence:** high · **Owner:** Ryan (ADR-0084 supersession; RUNBOOK Step 5.1)

ADR-0084 pins gix exactly and states 'other 0.x dependencies adopted later inherit the same rule. Stable 1.x dependencies use ordinary caret ranges', on the driver that '0.x crates have no semver
contract on breakage'. The design's §9.2 dependency table lists jsonschema 0.55.1, rusqlite 0.40.2, chrono 0.4.45 and sha2 0.11.0, all 0.x, all on 'caret'. The grounding row raised jsonschema (now
0.56.0, so a caret range would move it on the first `cargo update`); reading the table shows the same shape for the other three [measured]. Either the table or the ADR is what Step 5 copies into the
workspace dependency table, and they disagree. jsonschema's version drift itself is not a defect: ADR-0084 :104 forbids citing the list as current after 2026-09-08. [documented]

**Anchors.**
- `references/adr/0084-dependency-policy.md:58` — "Exact pinning is required for `gix`; other 0.x dependencies adopted later inherit the same rule. Stable 1.x dependencies use ordinary caret ranges."
- `references/adr/0084-dependency-policy.md:31` — "- 0.x crates have no semver contract on breakage, and gix's worktree and trailer semantics are load-bearing for the Phase-1 exit gate (0077). A caret range can move them silently mid-phase."
- `references/2026-09-08-architecture-design.md:1680` — "| `jsonschema` | runner has **0.52.1**; current is **0.55.1** | caret, `default-features = false` | boundary validation |"
- `references/2026-09-08-architecture-design.md:1682` — "| `rusqlite` (`bundled`) | 0.40.2 (2026-08-08, MIT) | caret | `rk-adapter-sqlite` only | **new** |"
- `references/2026-09-08-architecture-design.md:1679` — "| `chrono` (serde) | 0.4.45 | caret | event log, journal headers | **proven** |"
- `references/2026-09-08-architecture-design.md:1678` — "| `sha2` | 0.11.0 | caret | digests, memo keys, provenance | **proven** |"

**Why it matters before Step 5.** Step 5.1 writes `[workspace.dependencies]`. If it follows the table, four 0.x crates float on caret in contradiction of ADR-0084; if it follows the ADR, the table is
wrong. Decide which before the first Cargo.toml so the pin-bump review (which cargo audit and S-GIX hang off) has a defined scope. Not a wrong-thing blocker: either reading builds.

**Remediation.** Supersede ADR-0084's range clause with an explicit list: which 0.x crates are exact-pinned (at minimum jsonschema, whose 0.x cadence is weekly, and rusqlite, which carries the
one-writer semantics) and which are exempted with a reason (chrono 0.4 and sha2 0.11 are de-facto stable; if exempted, say so). Correct the §9.2 table's range column to match and refresh jsonschema to
0.56.0 with a fetch date. Owner: Step 5 workspace author.

**Sources.**
- https://crates.io/api/v1/crates/jsonschema (fetched 2026-09-10) — max_version 0.56.0 (2026-09-10); 0.55.1 (2026-09-08). tokio 1.53.1, thiserror 2.0.20, anyhow 1.0.104, schemars 1.2.2, insta 1.48.0, rusqlite 0.40.2, gix 0.87.1 still current

**Folds.** BP-RUST-20

### PIA-030 — Three RUNBOOK commands are wrong as written: Phase-1 exit check (3) invokes `redkiln run --resume <run-id>`, the flag AC-013/AC-186/AC-264/AC-400 and the design deny; `cargo clippy -D warnings` misparses and promotes nursery to the group deny ADR-0083 forbids; Checkpoint 6's gix-pin grep false-fails on the ADR-0084-conformant `gix.workspace = true`

**Priority:** P2 · **Classification:** NEW — [B05-C04] none (AC-0.3-002 records only the AC-400/186/264 overlap; ADR-0083-FORB-105 is the forbids clause itself; nothing records the RUNBOOK commands'
conflict) | [B04-C07] AC-0.3-002 (acceptance-criteria.md:65) records the AC-400 vs AC-186/AC-264 overlap, i.e. the ban; nothing records RUNBOOK:2155 or the `<run-id>` addressing; anomaly_pool has no
`--resume` row. · **Category:** contradiction · **Confidence:** high · **Owner:** RUNBOOK author + AC register author (Claude); Ryan sign-off on the clippy policy

Merged from 2 concerns; each member statement is carried verbatim. (i) (1) Phase-1 exit check (3) is `redkiln run --resume <run-id>` (RUNBOOK:2155, the only `--resume` in the runbook). Design
:321/:1267/:1278, AC-013, AC-186 ('no `--resume`, `--continue` or equivalent flag exists on any command'), AC-264 and AC-400 all deny the flag; AC-264's Verified-by asserts that exact invocation exits
as an unknown argument and AC-400's router lint fails the build on the name. The same block uses `run --lane fix --item` and `explain --run <run-id> --stages`, shapes found nowhere else in the corpus
[measured]. (2) AC-001's Verified-by and exit check (4) (RUNBOOK:1952, :2160) run clippy with `-D warnings`, which promotes every nursery lint to an error — the group deny ADR-0083:106 forbids and
whose CI-outage rationale RUNBOOK:1638–1640 states; measured on the RUNBOOK manifest, `missing_const_for_fn` became 'error … implied by -D warnings'. As written, `cargo clippy --workspace -D warnings`
is rejected by cargo ('unexpected argument -D'), so exit check (4) cannot run. (3) ADR-0084:58 pins gix 'in the workspace dependency table'; Checkpoint 6 (RUNBOOK:1850) greps
`crates/rk-git/Cargo.toml` for the literal `gix = "=0.87.1"`, which returns 0 for the ADR-conformant `gix.workspace = true` [measured]. The build is fenced three ways against (1), and (2)/(3) are
one-line command fixes; no workstream is blocked. (ii) RUNBOOK Step 8's exit criterion (3), the third of 'seven runnable checks', is written as `./target/debug/redkiln run --resume <run-id>` - the
only `--resume` invocation in the corpus [measured, grep] - while AC-400 (in Step 7d's own acceptance set) places `--resume` on the committed denylist with a build-failing router lint, AC-186 and
AC-264 say no such flag exists, AC-013 gives `redkiln run <item>` as the re-entry, and design lines 321/731/773/1267 say 'no --resume, --continue or equivalent flag exists on any command'. The command
also addresses the run by `<run-id>` where every other anchor takes an item, and RUNBOOK:2134 itself invokes `run --lane fix --item`; lines 2149-2157 carry no qualifier. As written, exit check (3)
cannot pass on a binary that satisfies AC-400: an operator following it literally records a false failure or adds the forbidden flag and breaks AC-400's snapshot test. Three finders reported it
independently; the refuters corrected it P1 to P2 because AC-400's lint fences the flag and the fix is one line - the highest-value P2 in this bucket.

**Anchors.**
- `RUNBOOK.md:2155` — "./target/debug/redkiln run --resume <run-id>"
- `references/2026-09-08-acceptance-criteria.md:319` — "WHEN `redkiln run <item>` re-enters (there is no `--resume` flag)"
- `references/2026-09-08-acceptance-criteria.md:4982` — "WHEN any invocation passes a flag whose name appears on the committed resume-shaped denylist (`--resume`, `--continue`,"
- `references/2026-09-08-architecture-design.md:1278` — "`redkiln run` refuses a resume-shaped flag by a **committed name denylist** (`--resume`, `--continue`, `--retry-step`,"
- `references/2026-09-08-acceptance-criteria.md:4984` — "THEN the invocation is refused as an unknown argument, and no denylisted name appears anywhere in the snapshot."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:106` — "- Forbids denying `clippy::pedantic` or `clippy::nursery` as groups."

**Why it matters before Step 5.** (i) (2) is chosen at Step 5: the CI author must pick nursery-at-warn with an exit-code gate (ADR-0083) or `-D warnings` (AC-001), which silently converts nursery into
the forbidden group deny. (3) is decided by the root manifest written at Step 5. (1) sits on Phase 1's last gate and, left as is, cannot be executed by a conforming binary. (ii) Not Step 5. Step 8 is
the bar every Phase-1 step is built toward, and 7d's CLI author reads Step 8 for the command surface the exit needs; the correct form must be written before the CLI snapshot is committed in 7d.

**Remediation.** (i) (1) Rewrite RUNBOOK:2155 as `taskkill /T /F …` then `./target/debug/redkiln run <item>` (same item, run_id read back from the event log per AC-013), asserting digest equality via
`explain <item>`; align checks (1)–(2) of the same block with the §8 verb table or extend the table and AC-400's snapshot deliberately. (2) Amend AC-001's Verified-by and RUNBOOK:1952/:2160 to
`cargo clippy --workspace --all-targets` gated on exit code, and reword Checkpoint 5 (:1755) to 'exit 0; nursery warnings permitted'; if warnings-as-errors is wanted, set nursery to allow by
superseding ADR-0083. (3) Change Checkpoint 6 to `grep -c 'gix = "=0.87.1"' Cargo.toml` → 1 and `grep -c 'gix.workspace = true' crates/rk-git/Cargo.toml` → 1. (ii) Amend RUNBOOK:2155 to
`./target/debug/redkiln run --lane fix --item <item>` re-invoked after the kill (the idempotent re-entry, AC-013), keeping the digest-equality assertion unchanged and taking any run id for the
`explain` comparison from `redkiln explain --run` as criterion (1) does; add a one-line note that `--resume` being refused is itself AC-400's/AC-264's behavioural check. No ADR change.

**Folds.** G11-03, slice-G12-02, G21-04, G21-10, L07-testability-01, G05-03, G08-02

### PIA-036 — The design diverges from the binding register it defers to: AC-511 is 'unresolved' in five design lines the register settles (and the register re-issued six cut ids against ADR-0085 rule 7); AC-356's withdrawn AND clause is still criterion text; the design says `schema_version` where AC-111/AC-117 and ADR-0072 say `version`; thirteen never-committed `ac-*.md` area files are named as owners; and reconcile.rs sits in opposite states on each side

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B04-C12] ARCH-0.1-037 (design reconcile.rs row); ARCH-10.x-T-010 (design:1939, AC-356 withdrawal) and AC-PROV-027/028; ARCH-10.x-L-003
(design:1969) and AC-0.4-T-005 (AC:122). None records the opposing state on the other side of the boundary. The `schema_version`/`version` swap (design:534, :1333) is unrecorded and NEW within this
concern. | [B07-C03] ARCH-10.x-L-003 (references/2026-09-08-architecture-design.md:1969) records the design's 'still unresolved' stance; AC-0.4-T-005 / AC-0.4-P-003 (acceptance-criteria.md:114, :122)
record the reuse as a hazard. No row records that the two artefacts now disagree or that §0.4 contravenes ADR-0085 rule 7 as written. | [B07-C13] none — known-issues has no row on the area files'
absence (ARCH-10.x-L-002 at design:1968 records 'area files own the criteria' as a deliberate exception, not that the owners do not exist) and zero rows for 'schema_version', 'AC-111' or 'AC-117'.) ·
**Category:** contradiction · **Confidence:** high · **Owner:** Claude (design and register edits); Ryan (Step 6 source-location line)

Merged from 3 concerns; each member statement is carried verbatim. (i) reconcile.rs: design 0.1 row 55, 1.3:184, 5:252, 9.1:1653 and ADR-0002:52 say it was 'rewritten in the spike and validated on 360
commits / 74 slices'; the AC register (:2468-2471, :4514) and SM-051 cut that as 'fabricated' with 'no spike in the brief that built or validated a reconcile.rs'; ground truth is
spikes/runner/src/reconcile.rs (420 lines, tracked, 330d46c 'S0.2 RESULT'), which produced F12, whose limits say it reconstructed state over the corpus but was NOT validated against agent decisions
(n=1) - so the design overclaims 'validated', the register overcorrects to 'not built', RUNBOOK Step 2 deletes spikes/runner before Step 6, and Step 6 never says where rk-git's sanctioned import
(AC-443) is lifted from [measured]. AC-356: design:1025 withdraws the binary-class mutation exemption ('a design sketch under a block, not a relaxation') and RUNBOOK:1723 keeps the per-AC norm in
force, but the register's AND clause (:4381) still states it as criterion text, its PROVISIONAL note covering only field names, and as worded it makes 'asset touched + test/capture result' sufficient
evidence, which ADR-0019's forbids block rules out [measured]. AC-511: the register's 0.4, 0.3 and ports-dist area key settle it as the `--bare` half split from AC-488 (AC-513 retired), while the
design's 8.8 row and 10.x note say the conflict is 'carried rather than resolved' and 'the sources do not settle which criterion holds the id', and design:362 places it in cli-doctor where the
register places it in ports-dist [measured]. `version`: design:534 and :1333 attribute to AC-111/AC-117 and ADR-0072 a rule that a shipped activity's coherence edit bumps `schema_version`, but the
cited rows (AC:1590, :1599), ADR-0072 (:43, :48, :54) and schema S9c.42 name the activity's `version` field; `schema_version` is the per-artifact-type field (design:669, AC-135, AC:5858), which the
register itself separates at :1812-1813 [measured]. (ii) [measured] The register's §0.4 (:122) and AC-511's own body (:6069, 'Uses the id freed by the cut of the old AC-511') settle AC-511 as the
`--bare` half split from AC-488, so the design's §8.4 and §2 citations of it are correct; but design :357, :362, :1345, :1585 and :1969 still say 'the sources do not settle which criterion holds the
id' and tell the builder to re-read the area file before touching either statement — for a question the binding register already answers. What no record licenses: the register re-issued AC-511, AC-512
and four other ids after cutting them, which ADR-0085 rule 7 ('Ids are never reused within their scope') and its forbids block ('forbids reusing a retired number') prohibit; §0.4 distinguishes 'cut
and freed' from 'retired' on its own authority. If cut ids may be re-issued, every pre-refutation citation of one of the six is suspect — the hazard §0.4 itself warns of. (iii) [measured]
`git ls-files` and `git log --all --name-only` return no `ac-*.md` path: the thirteen area files the register says it was 'assembled from' (AC :6532) never entered the repository. The design
nonetheless names an `ac-<area>.md` file on seven lines (:511, :682, :1178, :1333, :1847, :1900, :1961) with section labels (§1c, §1.2, §1.7) that resolve nowhere (the register homes are §4.3, §11.2,
§6.7), states at :349 and :1968 that 'the area file owns the wording', and at :362 instructs that AC-218 and AC-428 'must be reconciled in the area files before either is built'; no sentence discloses
the files as uncommitted scratch. In the 60-id seeded mirror diff the one divergence between the design's rendering and the register's definition: design :534 and :1333 render AC-111/AC-117 as bumping
`schema_version`, while the register's GIVEN clauses (:1590, :1599), ADR-0072 :48 and the memo key (ADR-0043 :17, `activity id@version`) say `version`; `schema_version` elsewhere in both documents
(design :650/:669, AC-135 :1890) is the per-artifact-type field under ADR-0006 — a different field on a different entity (rk-types, not rk-catalog). The register and ADR are the build sources and are
consistent, so no wrong code results; the design sends the builder to files that do not exist and to the wrong join field for AC-111/AC-117's CI job.

**Anchors.**
- `references/2026-09-08-architecture-design.md:1653` — "`reconcile.rs` validated on 360 commits / 74 slices [measured, F12]"
- `references/2026-09-08-architecture-design.md:1585` — "Both statements are left standing. The sources do not settle which criterion holds the id, and ids are fixed and are never renumbered (ADR-0085)"
- `references/2026-09-08-architecture-design.md:1333` — "`ac-packs-catalog.md` §1c carries a **second** doctor rule table — **AC-092–AC-099** — whose rows are configuration-time and zero-dispatch at the *pack* level"
- `references/2026-09-08-acceptance-criteria.md:2470` — "spike in the brief that built or validated a `reconcile.rs`. The corpus is real; the validation of an implementation"
- `references/2026-09-08-acceptance-criteria.md:122` — "| AC-511 | 'a compiled-in payload wins over a payload-shaped cwd' (cut: contradicts §4.9) | the `--bare` half split from AC-488 | ports-dist |"
- `references/2026-09-08-architecture-design.md:349` — "It is still **cited rather than restated** — the area file owns the wording"

**Why it matters before Step 5.** (i) Not Step 5. A Step 6 implementer must know reconcile.rs exists, where it lives after Step 2 and that 'validated' means 'ran over the corpus'; a verify-gate
implementer reading AC-356 alone builds a class exemption the design refused; an rk-catalog CI check written from design 4.5 targets the wrong field; a builder reading the design is told AC-511 is
contested when the binding register has settled it. (ii) AC-511 as defined is the criterion RUNBOOK 7c means, so the build is unaffected; the cost is operator time on five stale 're-read before
changing' notes and an accepted-ADR/register contradiction on the convention every citation in the corpus relies on. (iii) A builder following the design's ownership rule is sent to files that do not
exist, and the pre-build reconciliation at :362 has no place to happen; an implementer of AC-111/AC-117's changelog join working from the design would join on the wrong field.

**Remediation.** (i) Amend the AC register's cut note and SM-051 to 'built and run in S0.2 (spikes/runner/src/reconcile.rs), not validated against engine decisions', the design rows to 'run over'
rather than 'validated on', and add to RUNBOOK Step 6 the source location of reconcile.rs in redkiln-corpus at the pin (the same hazard phase1-deps OH-2 records for 7c's five files). Amend AC-356 in
place (PROVISIONAL, not an ADR): strike the AND clause or relabel it 'design sketch under S-BIN block; not a criterion', and add ADR-0019 to its decisions line. Under the id-precedence rule the
register governs: amend design 8.8 (line 1585) and 10.x (line 1969) to cite register 0.4 as the settlement, delete the 'unresolved' editorial note, and move AC-511 to ports-dist at design:362. Amend
design lines 534 and 1333 to `version` and add one clause in section 5 distinguishing the activity `version` from the artifact-type `schema_version`. (ii) Amend the design (dated amendment) to
withdraw the five 'unresolved' notes, citing AC §0.4 :122. Either partially supersede ADR-0085 with a record that licenses re-issue of a cut id under a §0.4-style hazard table, or record the six
reuses as tracked convention debt in ADR-0085's consumer/backfill note. Do not renumber and do not edit ADR-0085. (iii) Amend the design in place (it is a design, not an ADR): replace each
`ac-<area>.md §n` pointer with the register section (`acceptance-criteria.md §4.3` etc.), reword :349/:1968 to 'the register owns the wording', and re-home the :362 reconciliation to the register; add
one sentence to AC §16 stating the area files were session scratch and were not committed. Correct design :534 and :1333 to `version` and add a note distinguishing the activity `version` (rk-catalog,
ADR-0072) from the artifact-type `schema_version` (rk-types, AC-135).

**Folds.** L02-03, slice-G06-06, slice-G03-04, L12-authority-03, slice-G10-03, L02-01, L05-overlap-08

### PIA-045 — First-party Rust guidance does address the fake-vs-real dispatch question and ranks dyn last; the design's 'no sourced guidance' framing is half wrong

**Priority:** P2 · **Classification:** NEW — none (references/2026-09-08-evidence-augmentation.md:152 records the absence of a ports-and-adapters page, which holds; it does not record M-DI-HIERARCHY)
· **Category:** best-practice disagreement with a design commitment · **Confidence:** medium · **Owner:** Ryan (ADR-0076 supersession; rk-ports)

The evidence register says hexagonal/ports-and-adapters has no dedicated page in the Rust API Guidelines or Microsoft's Pragmatic Rust Guidelines and that 'one fake per port, dyn dispatch' is
practitioner synthesis, not sourced guidance. The first half holds. The second does not: Microsoft's M-DI-HIERARCHY (generated 2026-08-19) ranks the exact choice the design makes: 'Prefer types over
generics, generics over dyn traits', with the test-double case sent to M-MOCKABLE-SYSCALLS ('implement your type as an enum'), and dyn allowed only 'once generics become a nesting problem' and then
behind a custom wrapper rather than a visible `Arc<dyn Trait>`. ADR-0076 commits to one object-safe trait object per port, with the dyn boundary 'load-bearing for the fakes', and forbids a generic
kernel. The sync-trait choice already removes the object-safety objection Microsoft raises, and the ADR's reasons (five swappable fakes at one composition root) are real, so the decision can stand;
but it stands against, not in the absence of, a first-party ranking, and the record should say so and answer it. [documented]

**Anchors.**
- `references/2026-09-08-evidence-augmentation.md:152` — "Hexagonal/ports-and-adapters has **no** dedicated page in the API Guidelines or the Pragmatic Rust Guidelines; the pattern-level advice is community practice"
- `references/adr/0076-port-traits-stay-sync-async-confined-to-adapters.md:29` — "The dyn boundary is load-bearing for the fakes: one fake per port, swapped at the composition root, is how the kernel is tested at all."
- `references/adr/0076-port-traits-stay-sync-async-confined-to-adapters.md:78` — "Forbids making the kernel generic over its ports to obtain native async-fn-in-trait; 0003's dyn boundary stands."

**Why it matters before Step 5.** Step 7 writes rk-ports and the fakes; Step 5 writes rk-types only. The port-trait shape (bare `dyn Port` versus a newtype wrapper such as
`struct DynHarness(Arc<dyn Harness>)`, or an enum {Real, Fake} per port) is cheapest to decide before rk-ports exists. It does not make Step 5 build the wrong thing.

**Remediation.** Supersede ADR-0076's evidence block to cite M-DI-HIERARCHY / M-MOCKABLE-SYSCALLS / M-AVOID-WRAPPERS and state the divergence and the reason it is accepted (sync traits, five ports,
one composition root, fakes are the test strategy). Decide, in the same record, whether the boundary is a bare trait object or a newtype-wrapped one per M-AVOID-WRAPPERS; do not adopt enum dispatch
without a reason, since the ADR's fakes are scripted responders, not sans-io mocks. Correct evidence-augmentation row 152 to 'no ports-and-adapters page; the DI ranking exists and is answered in
0076'. Owner: rk-ports workstream.

**Sources.**
- https://microsoft.github.io/rust-guidelines/print.html (fetched 2026-09-10) — M-DI-HIERARCHY: 'Prefer types over generics, generics over dyn traits ... for testing, implement your type as an enum ... Once generics become a nesting problem, dyn Trait can be considered'

**Folds.** BP-RUST-02

### PIA-048 — Step 7b lint/loader: the acceptance set drops ADR-0089's binding criteria; applicable_if scope, `any` over null and the on_false form set differ

**Priority:** P2 · **Classification:** NEW — ADR-0089-FORB-216 (auth-only applicable_if ban) and ARCH-8.10-003 (require-row ban) each record one scope; AC-15.3-P-003 (AC:6526) lists ADR-0031/0044
among unaudited grammar records and ADR-0044-FORB-42/44 record the forbids. Nothing records 7b's acceptance line, that 0.1 states a wider scope than the ADR it cites, ADR-0032's `any` inversion, or
the deviate/max collision. · **Category:** contradiction · **Confidence:** high · **Owner:** RUNBOOK author (Claude) for 7b; Ryan (ADR successors); lint/loader workstream (7b)

7b's acceptance line excludes AC-069, AC-070, AC-072 and AC-079 as 'P1-blocked rows', although the register's 3 state line marks only AC-068 provisional and AC-076 human-gated, its 2026-09-09 note
makes AC-072 (registry lint with `deriver` and the set shape) and AC-079 (the null abstention route) the Phase-1 mechanisms of ADR-0089, and RUNBOOK:1494 and Checkpoint 8 (:2229) demand both - the
exclusion list is 1.2 row 1's 'criteria affected' column, which is not a do-not-build list, and it keeps AC-068, the only row actually PROVISIONAL [measured]. The lexical-deriver ban is stated with
three scopes: require row or applicable_if on all four facets (design 0.1 row 20), require rows only (design 8.10:1627, AC-520), require-row purposes for all four plus applicable_if for auth alone
(ADR-0089:212-217); Steps 5-8 never mention applicable_if, and eight catalog activities key it on ui, flag_exists or data_migration, so their Phase-1 admissibility depends on which document is read
[measured]. ADR-0032 makes quantifiers over null vacuously true, which under standard semantics inverts `any` (an absent evidence ref satisfies `evidence.any(GreenAt)`), and design:541 restates the
rule over an empty set - a present object - so an empty evidence list satisfies both the `any` and `none` stage-exit checks in the schema's own example against principle 6; `any` has zero catalog uses
today, so the consequence is latent, and AC-434 (ADR-0032's sole citer) tests display of `unknown`, not null semantics [measured]. No accepted record states the closed `on_false` form set: ADR-0044's
forbids block scopes the scalar ban to `on_false = "<step>"`, so ADR-0031's `continue`/`fail` (28 catalog uses: 15 and 13) survive, but ADR-0044:44's 'a table missing max remains illegal' collides
with ADR-0046's `{ to = "deviate" }`, for which no max rule exists, and only spikes/p9/lint.py:92 resolves both [measured].

**Anchors.**
- `RUNBOOK.md:2006` — "**Acceptance set: AC-001–AC-039** (kernel), **AC-040–AC-079** (selection, minus the P1-blocked rows AC-069,"
- `references/2026-09-08-acceptance-criteria.md:599` — "**State:** 38 binding · 1 provisional (AC-068) · 1 human-gated (AC-076)"
- `references/2026-09-08-architecture-design.md:20` — "**YES** — no `require` row or `applicable_if` may be keyed on `auth`, `ui`, `flag_exists` or `data_migration` as computed by the lexical deriver."
- `references/2026-09-08-architecture-design.md:1627` — "**No `require` row keyed on `auth`, `ui`, `flag_exists` or `data_migration` from the lexical deriver may be authored** (ADR-0089)"
- `references/adr/0032-absent-inputs-are-null.md:35` — ""Absent `?` inputs are `null`; quantifiers over `null` are vacuously true; `null == null`; any other operation on `null` is a lint error.""
- `references/2026-09-08-architecture-design.md:541` — "absent optional inputs are `null` and quantifiers over an empty set are vacuously true (**ADR-0032**)"

**Why it matters before Step 5.** Not Step 5; all four sit in 7b. An implementer building to 7b's acceptance set omits the set-shape check and the null-to-gate_opened route that Step 4a and Checkpoint
8 then assert; the AC-520 lint will accept what 0.1 forbids; two implementers diverge on `any` and on whether an empty list is null; the loader's on_false grammar is recorded only in a spike script.

**Remediation.** Rewrite 7b's acceptance line: build AC-069, AC-070, AC-072, AC-079 (binding); mark AC-076 human-gated and AC-068 PROVISIONAL (P2, mechanism only); cite 3's state line, not 1.2's
'affected' column. Decide the applicable_if scope once in a successor note to ADR-0089 (never an edit) or in AC-520's amendment, align design 0.1 row 20 and 8.10 row 1627, and add the clause (or its
explicit exclusion) to Checkpoint 8. Supersede ADR-0032 with per-quantifier semantics (`all`/`none` over null or empty = true; `any` = false), stating whether an empty present list and an absent ref
share semantics, plus an AC with a must-fail fixture (`any` over absent evidence must not pass); correct design:541. An amending ADR stating the closed on_false form set -
`{ to = <earlier step>, max = N }` | "continue" | "fail" | `{ to = "deviate", ... }` with max required only for loop-back - citing lint.py's on_false_of as the reference behaviour.

**Folds.** L03-05, L03-09, L04-02, L04-07

### PIA-054 — Design §4 misstates ADR-0056/0058: §4.4 omits the max_rigor ceiling, §4.6 inverts the ADRs' roles, AC-116 caps-and-reports a hole AC-048/104 fail

**Priority:** P2 · **Classification:** NEW — none (DEC-D-007, ADR-0056-FORB-69, ADR-0066-NEG-39 record the ceiling's existence; AC-0.3-025 @ acceptance-criteria.md:88 lists AC-104+AC-116 vs AC-048 as
a coherent overlap; ARCH-4.9-006 records the missing totality baseline; none records the omission, the misattribution or the tension) · **Category:** contradiction · **Confidence:** high · **Owner:**
Claude (design pass); Ryan to confirm

(a) Design §4.4 step 4 (:509) states producer admissibility as `min_rigor ≤ stage rigor` and `max_rigor` occurs nowhere in the design [measured, grep 0 hits], while ADR-0056's Decision (:37), AC-048's
GIVEN, schema §9c.26 and lattice.py:137 all make a member inadmissible above its ceiling; the post-decision lattice numbers in §0.1 row 53, the row-52 'bisect at mandatory' mechanism (F37) and
AC-105/AC-116 exist only because of it. An omission in the authoritative summary, not a contradiction. (b) AC-116's THEN describes a catalog whose family has a hole at r+1 and an engine that caps
escalation at r 'rather than escalating into' the hole, on the same fixture ('a family with a hole at mandatory') that AC-048 says must fail load and AC-104 says must fail the check; ADR-0058's
forbids block says such a family may not ship. Neither ADR contains a cap-and-report behaviour: ADR-0056's Decision is the totality lint rule itself (rule 26) and ADR-0058 defers totality to it, so
design §4.6:561 ('ADR-0056 supplies the escalation cap, not the totality check') inverts the record [measured]. AC-048 carries the ceiling and reading A (the check fails and the failure report names
the highest total rigor r) satisfies AC-104, AC-116's Verified-by and ADR-0058 simultaneously, so the rk-catalog lint workstream (7b) is not blocked.

**Anchors.**
- `references/2026-09-08-architecture-design.md:509` — "Candidates = activities with the kind in `provides`, `applicable_if` true, `min_rigor ≤ stage rigor`, `stage` equal to the requiring stage; **lowest `method_rank` wins**; recurse on the producer's `requires`"
- `references/adr/0056-family-totality-caps-rank-escalation.md:37` — "with rigor by putting a `max_rigor` on its lower members; above its ceiling, that member is inadmissible."
- `spikes/F37-DRAFT.md:86` — "`root-cause.toml` carries `max_rigor = "recommended"`, so `lattice.py:137` makes the activity inadmissible at `rigor = mandatory` and the engine escalates within the `localisation` family instead."
- `references/2026-09-08-acceptance-criteria.md:1532` — "THEN escalation is capped at *r* and the build reports the cap, rather than escalating into a rigor at which the family"
- `references/2026-09-08-acceptance-criteria.md:1522` — "THEN the check fails unless the family is **total** at that rigor — every"
- `references/2026-09-08-architecture-design.md:561` — "ADR-0056 supplies the escalation cap, not the totality check, and this document previously credited it with both"

**Why it matters before Step 5.** Step 7b, not 5. A selector built from §4.4 as written never escalates (root-cause at rank 10 stays admissible at mandatory), making the F35 baselines AC-101/AC-102
pin, F37's mechanism (AC-105's fixture) and AC-116's cap unreachable. Reading B of AC-116 (the engine computes a per-family ceiling and loads catalogs with holes) ships something ADR-0058 forbids;
reading A leaves AC-116's fixture unrunnable as a load. The AC register is binding and AC-048 carries the ceiling, so no workstream builds from §4.4 alone.

**Remediation.** Amend design §4.4 step 4 and §4.7 to state admissibility as `min_rigor ≤ r ≤ max_rigor` (absent ceiling = no upper bound), cite ADR-0056 in the row, and add the ceiling to
AC-046/AC-052's GIVEN clauses. Reword AC-116 as a diagnostic on the failing report ('the failure names the highest total rigor r') or fold it into AC-104; correct design §4.6:561's attribution
(ADR-0056 = totality rule 26; ADR-0058 = the declared family field and its three lint checks); amend the AC §0.3 overlap row. Do not edit either ADR.

**Folds.** slice-G18-01, slice-G18-02

### PIA-055 — Backoff base 2 s / cap 5 min has two homes: AC-029 and schema say kernel policy, design §3.6/§3.10 say pack-declared and code defaults refused

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ARCH-3.10-007 @ references/2026-09-08-architecture-design.md:424 (records the numbers as unmeasured and 'declared in the pack'; not the
disagreement over where they live)) · **Category:** contradiction · **Confidence:** high · **Owner:** design author / AC register author (Claude)

AC-029's THEN hard-codes base 2 s / cap 5 min, its Note calls them 'chosen constants', and its Verified-by tests only that `N` is pack-declared; the activity-graph schema (:83) says a step never
defines its own backoff and selects the kernel's per-error-class policy. Design :345 and :424 say the same numbers are 'declared in the pack and refused as a code default (AC-029 …)', while design
:338 places them in 'kernel policy per error class' — the design disagrees with itself seven lines apart [measured]. Either shape passes AC-029's stated tests, so nothing fences the choice.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:461` — "THEN the delay is exponential with full jitter, base 2 s, capped at 5 min, and the attempt count never exceeds the"
- `references/2026-09-08-acceptance-criteria.md:462` — "pack's `N`."
- `references/2026-09-08-architecture-design.md:345` — "Every one of these numbers is declared in the pack and refused as a code default (AC-029, AC-033, AC-023); the criteria verify conformance to"
- `references/2026-09-07-activity-graph-schema.md:83` — "`retry = { max = N }` selecting the kernel's per-error-class policy (§4.3) — a step never defines its own backoff."

**Why it matters before Step 5.** An implementer following the AC writes constants into `rk-kernel::retry` (7b); one following design :345 adds pack fields and a validation rule refusing defaults,
which changes the pack schema authored at Step 4a/7b. Both pass the AC's tests; no crate is blocked. Record correctness.

**Remediation.** Decide once, before 7b: either amend AC-029 to 'base and cap are pack-declared; a code default is refused' with a pack-validation test, or amend design §3.6:345 and §3.10:424 to say
only K, N and the appetite are pack values and base/cap are kernel policy constants (the schema's and :338's position). Record which in the §0.3 authority table.

**Folds.** G11-07

### PIA-056 — An equivalent mutant survives re-execution, so ADR-0074's `mutation-survived` class can block on a no-op mutation the machine cannot filter

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (spikes/PRIOR-ART.md:132 and :234 record equivalent-mutant exclusion for the measurement denominator; ADR-0074's class computation does not carry
it) · **Category:** design gap against documented mutation-testing semantics · **Confidence:** medium · **Owner:** Ryan (ADR-0074 supersession; review-gate policy table)

ADR-0074 computes the evidence class in Rust: '`mutation-survived` iff a mutation was applied and the named suite stayed green', and the policy table makes `reproduced` and `mutation-survived` block.
Standard mutation-testing semantics, which PRIOR-ART s5 already adopts for the measurement side ('exclude equivalent mutants from the denominator; prefer revert-the-fix-keep-the-test'), include the
equivalent mutant: a mutation that does not change behaviour survives every suite. Re-execution therefore confirms the finding's premise (the mutant survived) without confirming its claim (the suite
has a gap), and the machine-confirms rule that ADR-0074 relies on to strip the reviewer's false blocks does not filter this case. cargo-mutants (27.1.0, June 2026) is live, and equivalent-mutant
detection remains an open research line (arXiv 2607.00511). Neither the ADR's clause 2 nor its policy table names equivalence triage or restricts the class to reverted-fix mutants. [inferred from
documented semantics]

**Anchors.**
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:57` — "`mutation-survived` iff a mutation was applied and the named suite stayed green"
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:60` — "`reproduced` and `mutation-survived` block; `narrowed-pin` blocks only under a strict weight; `advisory` and `Unverifiable` never block."
- `spikes/PRIOR-ART.md:234` — "| 3 | S0.1 adopts mutation-testing methodology: exclude equivalent mutants from the denominator; prefer revert-the-fix-keep-the-test as the mutation source; use existing tooling | §5 |"

**Why it matters before Step 5.** The review gate and its policy table are Step 7 work; Step 5 does not build them. It belongs in the record before the class computation is written, because the fix is
a change to what counts as a confirming repro record (a reverted-fix mutant, or a mutant plus a discriminating assertion), not a threshold.

**Remediation.** Supersede ADR-0074 clause 2 so that `mutation-survived` requires either a reverted-fix mutant (the change under review is reverted and the named test stays green) or a mutation paired
with an assertion that distinguishes mutant from original (collapsing the equivalent case into `narrowed-pin` or `advisory`); carry PRIOR-ART s5's equivalence rule into the ADR's evidence block. If
the false-block rate from equivalent mutants is to be measured rather than designed out, register it as a named spike with the review-gate policy table as consumer (ADR-0012). Owner: review-gate
workstream.

**Sources.**
- https://crates.io/api/v1/crates/cargo-mutants (fetched 2026-09-10) — max_version 27.1.0; newest created_at 2026-06-02; repository github.com/sourcefrog/cargo-mutants; no yanked versions
- https://arxiv.org/pdf/2607.00511 (fetched 2026-09-10) — Shu et al., Jul 2026: LLM-based equivalent-mutant detection on 3,302 Java + 1,088 C pairs; equivalence remains an active research line

**Folds.** BP-EVAL-17

### PIA-058 — The prompt-and-cache ADR pair (0016/0078) disagrees with its own evidence and its instruments at five points, all landing in the Step 7c adapter

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (AC-15.3-005 (AC:6514, ADR-0078 coverage as an 'unverified pointer'); ARCH-0.1-010 and EA-1-001 (two-bucket rule); ARCH-10.5-024, ARCH-10.4-023,
AC-1.2-015, SM-5.2-006 (S-WT-REC and the CONTESTED cell); ADR-0078-FORB-82 (the worktree clause as a deliberate exception). Unrecorded and NEW within this concern: the bucket mis-attribution
(design:1105), the forbids-vs-evidence contradiction inside ADR-0078, and the 7c grep vs ADR-0016:35.) · **Category:** contradiction · **Confidence:** high · **Owner:** Ryan (ADR successor,
append-route decision); Claude (register and design edits)

(1) AC-210, AC-385 and AC-423 pin only `CLAUDE_CODE_PROMPT_CACHE_TTL=1h`, the one-variable configuration ADR-0078:79 forbids; the register has zero occurrences of the subagent variable, 'billing mode'
or 'TTL-unverified', AC-385's evidence line still calls the cache org-scoped (forbidden at :82), and only RUNBOOK:2042 carries the two-variable rule, so an env builder that sets one variable passes
every instrument green [measured]. (2) Design 7.3:1105 says 'every dispatch this design generates is bucket two' while the 1P table five lines above places `-p` runs in bucket one; every activity is a
`claude -p` child (design:266, :1117), subagent_stats.spawned is 0 on all 104 P7 step records, and ADR-0078:28's 'dominant traffic' driver and the research pass's severity 'high' describe the legacy
plugin's in-session subagents, so S-TTL1 as a single `-p` dispatch cannot observe the bucket line 1109 charges it with [measured]. (3) ADR-0078's forbids block bans any budget that assumes two
worktrees share a prefix, and design:1089 calls the cold start 'structural', while the ADR's own evidence rows (:19, :89) cite S-WT w2-w10 at 1.000 with 0 tokens created in distinct worktree cwds
under the static prompt - the Phase-1 default - and F16 arm (d) shows default-prompt worktrees sharing about 68 percent, not none [measured]. (4) S-WT-REC ('which prompt configuration did S-WT run
under', $0, Not started) and AC-387's CONTESTED cell rest on 'the brief does not record', yet METHOD-S 4b.2 pre-registered `--system-prompt`, all ten s-wt.jsonl records carry it, ADR-0078:89 already
says so, and F16's 0.685 (default prompt) and S-WT's 1.000 (static) are different cells, not one cell read twice [measured]. (5) Checkpoint 7c asserts the literal `append-system-prompt` occurs zero
times in crates/*.rs while ADR-0016:35 retains the append route 'as a config option, pending S-H1c' and RUNBOOK:2214 permits it as an unmeasured option [measured].

**Anchors.**
- `references/adr/0078-cache-rule-is-split-both-ttls-set-scope-corrected.md:79` — "- Forbids setting `CLAUDE_CODE_PROMPT_CACHE_TTL` without also setting `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`."
- `references/2026-09-08-acceptance-criteria.md:2753` — "THEN `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` is set explicitly and its value is recorded on the dispatch event."
- `references/adr/0078-cache-rule-is-split-both-ttls-set-scope-corrected.md:83` — "- Forbids treating a cold first turn in a fresh worktree as a rig defect, and forbids any budget that assumes two worktrees of one repo share a prefix."
- `spikes/results/s-wt.jsonl:6` — ""arm":"wt-sysprompt-w2" … "cache_read_input_tokens":18752 … "cache_creation_input_tokens":0 … "cache_read_share":1.0"
- `RUNBOOK.md:2061` — "grep -rn  'append-system-prompt'    crates/ --include=*.rs | wc -l   # → 0 (AC-211, S-H1c unrun)"
- `references/adr/0016-static-system-prompt-carries-the-card.md:35` — "The append route (`--exclude-dynamic-system-prompt-sections` + `--append-system-prompt`) is retained as a config option, pending S-H1c."

**Why it matters before Step 5.** Not Step 5; all five land in the 7c adapter and the Phase-2 fan-out budget. Setting both variables is free and RUNBOOK 7c already says so, so the build is right; what
is wrong is that the AC instruments cannot fail a one-variable builder, AC-387's 'Unmeasured' refusal would lock a measured cell out of the kernel, an implementer following ADR-0016 fails Checkpoint
7c, and the cost model's stated TTL exposure is mis-attributed.

**Remediation.** One successor record to ADR-0078 (never edit): scope the worktree forbids-clause to the default prompt, citing S-WT's static-prompt cell with its n and caveats (n=9 steady-state
worktrees, haiku, --strict-mcp-config, an 18-byte fixture tree, one draw per worktree); correct the traffic shape to 'bucket one, bucket two reachable only if a card grants subagent tooling', and
register any bucket-two observation as a card that spawns a subagent rather than S-TTL1's single -p dispatch. Decide the append route once: (a) keep the grep and supersede ADR-0016:35 with 'not
implemented in Phase 1; S-H1c runs in the spike runner', amending 3.10/7.10 and RUNBOOK:2214, or (b) drop the grep from Checkpoint 7c for AC-211's own instrument, a config fixture asserting the flags
are absent from argv under the default. Close S-WT-REC in 10.5 as answered by METHOD-S 4b.2 and s-wt.jsonl; amend AC-387 so the static-card x worktree cell carries S-WT as source and
`Unmeasured(cell)` covers only cells no spike ran; strike 'the brief does not record which configuration' at design :49, :73, :177, :1174, :1237, :1245, :1838 and AC :195, :4809-4825, :4960, and
qualify design :1089. Amend AC-210/AC-423 to require both variables with a doctor red fixture for one-set; add an AC (or fold into AC-360) for billing-mode recording and the TTL-unverified report
shape; strike 'org-scoped' from AC-385 and 'account-wide' from AC-423; rewrite 15.3's ADR-0078 row from 'covers' to 'contradicted, amended'.

**Folds.** slice-G07-03, slice-G07-07, L10-register-integrity-01, L10-register-integrity-02, slice-G04-03

### PIA-063 — AC-311 offers a 'quarantine' ADR-0082 makes an error; AC-344 restates AC-288's block predicate in a second crate with no class scope

**Priority:** P2 · **Classification:** NEW — none (AC-1.2-009, AC-15.3-009, AC-PROV-029 map ADR-0082 to AC-310/311/344 and record only the advisory-sink dependency; ADR-0082-FORB-78 carries the
forbids; AC-288 has no row; no §0.3 authority row pairs AC-288 with AC-344) · **Category:** contradiction · **Confidence:** high · **Owner:** AC register author (Claude)

(a) AC-311 (human gate, in force) offers 'a backlog item, a quarantine, or a dismissal' as triage dispositions for an advisory sink record; ADR-0082 clause 4 ('Advisory and `Quarantined` are
disjoint') and its forbids block (:79, 'Forbids applying `Quarantined` to a finding that was not computed as blocking') make quarantining a non-blocking finding an error, not a no-op, because the
override telemetry would record a suppression that never happened. Design §6.4.2 restates the ADR rule (:942) in the row directly below its AC-311 enforcement row (:941) without reconciling them;
AC-311 cites only ADR-0025 [measured]. (b) AC-344 (rk-kernel-verify) duplicates AC-288's (rk-kernel) fail-candidate/pass-reference truth table and four-cell test in a second crate with no
cross-reference, and its GIVEN ('a finding whose repro record names a command') is not explicitly scoped to `reproduced`-class findings; AC-284's decision table implies that scope, so the apparent
contradiction with AC-289 for `mutation-survived` findings (which block when the suite stays green, i.e. pass-on-both) is a plausible misreading rather than a stated one. Which crate owns the
re-execution predicate is unstated [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:3894` — "WHEN a human triages it into a backlog item, a quarantine, or a dismissal"
- `references/adr/0082-advisory-sink-is-a-first-class-destination.md:59` — "4. **Advisory and `Quarantined` are disjoint.** `Quarantined { reason, by, until }` applies **only** to a finding the"
- `references/2026-09-08-acceptance-criteria.md:4242` — "**AC-344** — `rk-kernel-verify` · evidence: §4.6 point 3 (proposal), brief §C.7 · decisions: ADR-0004, ADR-0019"
- `references/2026-09-08-acceptance-criteria.md:4245` — "THEN it blocks only if it fails on the candidate and passes on the reference"
- `references/2026-09-08-acceptance-criteria.md:3654` — "**AC-289** — `rk-kernel` · evidence: F11 (S0.1: 15 mutants, 9 of 10 test-reachable killed, **1 survived**; test-reachable"
- `references/2026-09-08-acceptance-criteria.md:3658` — "THEN it blocks only if the suite is still green with the mutant applied."

**Why it matters before Step 5.** Step 7c, not 5. The human-gate answer schema for advisory triage (AC-311, bound to ADR-0063 attachments) is a Phase-1 type; offering 'quarantine' builds the state
ADR-0082 forbids. An engineer implementing rk-kernel-verify to AC-344 literally, without AC-284 in view, ships a verification gate whose own finding class (AC-340) never blocks. One-line AC amendments
plus a negative fixture; nothing fenced.

**Remediation.** Amend AC-311's WHEN to 'a backlog item or a dismissal (with reason)', add ADR-0082 to its decisions, and add the AC-304 negative fixture 'quarantine of an advisory record is refused'
that ADR-0082 clause 4 calls for. Scope AC-344's GIVEN to `reproduced`-class findings and cross-reference AC-288/AC-289 as the authority for the per-class flip rule (add a §0.3 authority row), or
delete AC-344's THEN and keep only its Unverifiable-to-advisory-sink clause; state in both criteria which crate owns the block predicate.

**Folds.** G15-12, G16-03

### PIA-072 — Phase-1 ports scope drawn three ways: ADR-0003 names GitHub/mdBook Phase-1, AC-499 needs the CI decision ADR-0010 punts, 7a drops checkable AC-239/496

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (SM-NOBASE-091 @ success-measures.md:1070 (SM-148's draft correction); ADR-0010-NEG-40 / ADR-0010-FORB-45 (the CI gap); ARCH-3.10-009,
ARCH-8.10-008, AC-PROV-017/036 (the 'checkable today' halves on the design/register side). No row records that ADR-0003 still carries the Phase-1 label, that AC-499 depends on the undecided ADR, or
that 7a's exclusion contradicts the register.) · **Category:** contradiction · **Confidence:** high · **Owner:** Ryan (ADR-0003 supersession); AC register maintainer (ports-dist area); RUNBOOK author

(a) ADR-0003 (accepted, superseded_by []) decides 'each with one real adapter named for Phase 1' with GitHub and mdBook in its 'Real adapter (Phase 1)' column; design §3.3 ('none in Phase 1'), §10.2
and SM-148 put both in Phase 3, the design says 'corrected against proposal §7', and no record amends or supersedes ADR-0003; the §3.1 diagram asterisks `mdbook*` but not `github`, and RUNBOOK 7a and
design §1/§3.9 repeat 'one real adapter and one fake each' [measured]. (b) AC-499's instrument is 'a CI matrix row per instantiation' over `{fake, real}`, which for the Harness port requires the
subscription-only Claude adapter to run under CI — the question ADR-0010:32/:46 declines to decide before Phase 3+ and whose Negative consequence says no path exists; AC:6369 records the cost and the
AC-498 tension but not this dependency [measured]. Tracker/DocsSite real rows are vacuously absent under AC-499's own GIVEN, not a defect. (c) RUNBOOK 7a (:1902) drops AC-239/AC-496 wholesale as
'S-CODEX is unrun', while AC-239's list-is-data test is 'checkable today, with no Codex adapter present' and AC-496's config-time refusal 'is implementable today' with a doctor row — the design's
stated hedge for S-CODEX being unrun (design:426). Only the Codex-present halves wait on Phase 4. RUNBOOK 7c builds exactly three adapter crates, so (a) is fenced; (b) yields a Harness row that is
either unbuildable or quietly reported green on fakes (the vacuity SM-151 discloses); (c) drops a config-time guard that is a small kernel/doctor addition.

**Anchors.**
- `references/adr/0003-five-named-ports-no-postgres.md:34` — "The kernel exposes exactly five ports, each with one real adapter named for Phase 1 and one fake, per §4.1's table:"
- `references/2026-09-08-architecture-design.md:246` — "GitHub and mdBook are Phase 3 (SM-148, corrected against proposal §7)"
- `references/2026-09-08-acceptance-criteria.md:6175` — "*Verified by:* one parameterised conformance suite per port, instantiated over `{fake, real}`; a CI matrix row per instantiation."
- `references/adr/0010-claude-adapter-subscription-only-auth.md:46` — "- Decisions about CI execution of the Claude adapter may not be made under this ADR; it is explicitly punted to Phase 3 or later."
- `RUNBOOK.md:1902` — "**Acceptance set: AC-480–AC-519** (ports, fakes; AC-239/AC-496/AC-502 are out of scope — S-CODEX is unrun and …"
  Phase 4 is the earliest a second harness ships)."
- `references/2026-09-08-acceptance-criteria.md:3038` — "*Verified by:* a lint/unit test that the gate-eligible adapter list is data with exactly one entry in Phase 1 (checkable …"
  today, with no Codex adapter present)"

**Why it matters before Step 5.** None of the three touches Step 5. A builder taking ADR-0003 as authority (ADRs bind) would put GitHub and mdBook adapters into 7c; an implementer of AC-499 either
wires a CI job that needs a Claude subscription session or drops the real row; following 7a as written ships Phase 1 without the one guard that makes 'a Codex-sourced result may not close a gate'
enforceable, and adding it after the argument surface and doctor rule table are snapshotted (7d) is a late change.

**Remediation.** (a) Record a short supersede-in-part ADR restating ADR-0003's column as 'named real adapter (phase per design §10.2)' with the Phase-1 set = {SQLite, Claude, Node}; asterisk `github`
in the §3.1 diagram; reword RUNBOOK 7a and design §3.9 to 'one fake each; three real adapters in Phase 1'. (b) Amend AC-499 with a phase clause: Phase 1 instantiates `{fake, real}` for EventStore
(SQLite) and Toolchain (Node); the Harness real row is deferred to the CI decision ADR-0010 punts, registered as a named open item with a consumer per ADR-0012; note the dependency in the §14 State
line. (c) Split 7a's exclusion: keep AC-239's list-is-data test and AC-496's config-time refusal plus its doctor row in the 7b/7d acceptance sets; exclude only the Codex-present halves and AC-502.

**Folds.** slice-G12-05, slice-G10-02, slice-G12-06

### PIA-075 — Corrections and restorations recorded as applied left copies uncorrected: F35 section 1.3 still 'must be re-grounded'/'fabricated' at AC-049, AC:1282 and design:496/589 after AC:6554 restored it; the +40..+101% band still 'fabrication' in SM-040; S-WT's prompt configuration still CONTESTED though s-wt.jsonl records `--system-prompt`; the is_error fixture 'may not exist' though e3.jsonl holds seven; AC-068's '39 of 41' and section 10.5's 'two-line' after the F40 fix; METHOD-S's S-BIN text_diff qualification propagated nowhere

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B05-C11] references/2026-09-08-success-measures.md:1437 and references/2026-09-08-acceptance-criteria.md:6554–6556 (the concession, for F35 §1.3
and +40%..+101%); SM-NOBASE-071 @ success-measures.md:916 and ARCH-0.1-037 (reconcile.rs 'validated' half). No row records that the struck bodies were not updated, nor that the reconcile.rs
'fabricated' strike is itself wrong. | [B07-C10] F35: acceptance-criteria.md:6554 and success-measures.md:1437 record the over-strike and its correction (no known-issues row), not the three stale
sites. S-WT: ARCH-10.4-023 registers S-WT-REC as 'recovering the existing run record' and marks the cell CONTESTED. is_error: SM-5.3-003 (success-measures.md:1399), AC-PROV-032 (:5439), AC-PROV-033
(:5465) record the 'may not exist' premise. None records that the evidence exists. | [B06-C13] H10-TRAP-001 / HANDOFF-2026-09-10:93-94 record the F40 miscounts as found and fixed; AC:6540-6554
(section 16 restoration table) restores 'F35 section 1.3' as real; MS-AMEND-010 records the METHOD-S amendment -- none records the missed copies) · **Category:** contradiction · **Confidence:** high ·
**Owner:** Claude (AC/SM/design authoring pass); Ryan (success-measures owner) sign-off

Merged from 3 concerns; each member statement is carried verbatim. (i) (a) F35-DRAFT.md §1.3 contains the mixed Node+UE silent-drop finding ('the text summary is never produced') [measured, read];
AC:6554 records it 'real, and exact' and SM:1437 records the pass 'over-struck' it against the condensed brief; design §4.3:496 and decision 4-C:589 still flag ADR-0073/AC-088 'evidence-thin … must be
re-grounded' and AC-103's body (:716) keeps the strike. (b) SM-040's body (:343) calls the +40%..+101% cost band 'the most damaging fabrication found in the area' and the §5.2 row (:1138) says it
'does not exist'; F31-DRAFT.md:166 prediction 5 reports +101% (S-SILENT), +40% (S-SUBTLE), +71% (CLEAN) beneath +74% overall; SM:1437 and AC:6556 concede the band is real but point the true source at
proposal §4.6 rather than F31. The cut removed the only measured spread SM-040 has, so its 'first repeat measurement sets the band' target is written as if none exists, and the +101% per-condition
ceiling that is the natural kill line is disowned. (c) AC-184's cut note (:2470), AC-443 (:4514) and SM-051 (:414) call 'reconcile.rs rewritten in the spike and validated on 360 commits / 74 slices'
fabricated; AC-483 (:6006), design §0.1 row 55, §11:1653, ADR-0002 and ADR-0003:64 carry it as [measured]. spikes/runner/src/reconcile.rs exists (420 lines, wired at main.rs:1177–1183) and produced
F12 (:370 'deterministic reconcile over all resume trailers'), so 'built and exercised' is true and 'fabricated' false; F12:437 says it was 'not validated against the agent's decisions', so
'validated' is unsupported — both sides are wrong. The brief has two 'reconcil' hits, neither about S0.2; SM:1437's named list does not include it [measured]. ADR-0002's 'rewritten and revalidated'
exception has no defined revalidation instrument. (ii) [measured] Three sites hold a criterion, a row or a spike open on the premise that evidence is missing which the repository already contains. (i)
Register §16 (:6554) restored 'F35 §1.3' as 'real, and exact' (spikes/F35-DRAFT.md :27) and SM :1437 records the over-strike, yet AC-049's refutation note (:716-718) still says the behaviour 'must be
re-grounded', and design :496 and :589 still say it 'is not in the evidence brief' / [documented] / evidence-thin while design :183 and :214 tag the same rule [measured, F35 §1.3]. (ii) Design §0.1
row 49 says 'CONTESTED: the brief does not say which prompt configuration S-WT ran'; METHOD-S :508-513 pre-registered `wt-sysprompt-w1..w10` under `--system-prompt`, all ten dispatches in
spikes/results/s-wt.jsonl carry that flag at CLI 2.1.263 (w1 read 15,129 / created 3,623 = 0.807; w2-w10 read 18,752 / created 0 = 1.000), the brief :102 itself says so, and ADR-0016 :17 and ADR-0078
state it; F16 arm (d) 0.685 is the default-prompt × worktree cell, a different cell. AC-387's Verified-by nonetheless requires the cost model to return `Unmeasured` for a cell with nine steady-state
draws, SM :431 calls the combination UNRUN, and S-WT is 'carried without a finding number'. (iii) AC-218 :2836, AC-428 :5431-5441 and SM-127 :1399 say the `is_error`/`api_error_status` frames have 'no
evidence anywhere in the brief' and the fixture 'may not exist' — true of the brief's one-row F21 entry, false of the tree: F21 (:1092-1097), references/2026-09-04-findings-for-product.md :81-86 and
seven records in spikes/results/e3.jsonl (six 529, one 500; `subtype: "success"`, `is_error: true`, `step.exit_code` 1) hold them. Two sharpenings: the measured platform-error shape is exit 1 +
is_error, not the exit-0 cell AC-218/AC-428 are written around; and F21's count (7 + 1 = 8) disagrees with the file (7), with the 500 record's `void_reason` still reading 'schema'. (iii) Four
corrections the corpus records as applied left a copy uncorrected. [measured] (1) HANDOFF-2026-09-10:93-94 records the F40 miscounts as found and fixed ('"39 of 41 rows" ... It is 36'; 'two body lines
... it changed one'), and F40 section 1, design section 0.1, SM-021 and RUNBOOK all say 36 and one line; AC-068:970-971 still says the labeller's reason appears against '39 of the 41 rows' (counting
the label file: 35 exact, one variant, four blank, one P01's; the 39 wraps a line break so the census seed p2.rows_36_41 reported AGREE without the register in its file list) and design section
10.5:1864 still says 'one two-line wording change' (the P01 diff is one `-`/`+` pair). (2) The AC document's restoration table (:6554) restores 'F35 section 1.3' as 'real, and exact'
(spikes/F35-DRAFT.md:27 carries the silent-drop sub-finding, cited by ADR-0073:33 and design :214/:589), but AC-049's strike note at :716-718 still stands, :1282 still says 'its only citation was
found fabricated' and :183 calls it 'unfounded on the same ground' -- AC-049 is 'the weakest survivor' on a premise the same document withdrew. (3) METHOD-S:960 says the register rows describing S-BIN
as 'no diff' 'are qualified accordingly' (text_diff scoped to source and test paths; four of five items carry bookkeeping text changes); a grep for the qualification over references/, RUNBOOK.md and
both handoffs finds nothing, and design :22, :130, :1862 and ADR-0008:53 still say 'no diff' unqualified.

**Anchors.**
- `references/2026-09-08-architecture-design.md:496` — "cited there as "F35 §1.3" — **is not in the evidence brief**: F35's ledger row records unsatisfiability, dead activities, ties, cycles and runtime, and has no such sub-finding. The rule stands on ADR-0073's authority as [documented] and is flagged evidence-thin; it must be re-grounded"
- `references/2026-09-08-acceptance-criteria.md:718` — "§1.3.** The behaviour is retained on ADR authority alone and must be re-grounded before it reaches the design document;"
- `references/2026-09-08-acceptance-criteria.md:970` — "labeller wrote *"only frontmatter properties (process/stage status), no new/updated facts of the system"* against **39"
- `references/2026-09-08-acceptance-criteria.md:6554` — "| **`F35 §1.3`** | real, and exact — on a `node+unreal` item both change summaries are admissible at rank 10, specificity picks the UE one, and the text summary is **never produced**; satisfiable and wrong | `spikes/F35-DRAFT.md` §1.3 |"
- `spikes/plans/HANDOFF-2026-09-10-design-session.md:93` — ""39 of 41 rows" — 41 minus P01's two, computed rather than counted. It is 36."
- `references/2026-09-08-success-measures.md:343` — "neither 40 nor 101 appears anywhere in it. This was the most damaging fabrication found in the area — a kill threshold set against a measurement that does not exist.)*"

**Why it matters before Step 5.** (i) No build blocker. An implementer reading the design alone treats AC-088's rule as unmeasured and blocked on S-BIN; Step 6's rk-git carries reconcile.rs as 'the
one code import' while the AC register tells the implementer the artefact does not exist and the design tells them it is validated. (b) affects how the SM-040 kill ('materially above +74%') will be
judged. (ii) AC-387's unit test as written asserts a refusal the record does not warrant; AC-428/SM-127 are PROVISIONAL-with-a-deletion-clause on a false premise, and the seven recorded frames are the
must-fail fixture for AC-218's four-way matrix and the RUNBOOK's `spoke()` rule — on disk now, at the corpus pin after Step 2. (iii) None for the build: AC-068 is PROVISIONAL, AC-049 is rk-select
(post-Phase-1), S-BIN is unrun. These are stale numbers inside a criterion that promises verbatim fidelity to F40, a record that says a measured sub-finding does not exist, and a METHOD claim that is
false as of this tree.

**Remediation.** (i) One re-grounding pass against the primary spike records, never the condensed brief: (a) restore the [measured, F35 §1.3] tag in design §4.3 and 4-C, delete 'must be re-grounded',
reinstate AC-103's citation with a pointer to §15, re-tag §4.6's 'three quarters' as [documented, F35 §4]. (b) Rewrite SM-040's baseline to '+74% overall ($112.25 vs $64.61); by condition +40%
(S-SUBTLE, n=5), +71% (CLEAN, n=23), +101% (S-SILENT, n=23) [measured, F31 prediction 5] — one draw per item per arm, so the spread is across conditions, not a sampling interval'; delete the
'fabrication' sentence, fix SM:1138, point AC:6556's true source at F31:166. (c) Amend the AC/SM cut notes to 'built in the spike runner and exercised over 360 commits / 74 slices (F12); not validated
against an oracle'; downgrade AC-483, §0.1 row 55 and §11 from 'validated' to 'exercised'; define 'revalidated' for ADR-0002's exception (AC-184's rebuild differential is the natural instrument). Do
not edit ADR-0002/0003. (ii) Re-ground AC-049 (:707/:717) and design :496/:589 to 'spikes/F35-DRAFT.md §1.3 [measured]' and delete the 're-grounded' clauses. Amend AC-387 to price static × worktree
from S-WT (w2-w10 = 1.000, n = 9; w1 = 0.807 cold) and default × worktree from F16 arm (d) (0.685, n = 9), keeping `Unmeasured(cell)` for cells no spike ran; mark S-WT-REC DONE citing s-wt.jsonl;
strike CONTESTED from §0.1 row 49, §7.7, §10.4 and SM-053; write the reserved F30 as a dated entry. Amend the brief's F21 row to carry the platform-error shape; strike 'may not exist' from AC-428 and
SM-127 citing F21 and e3.jsonl (slugs fix-doctor-git-config-repair, issue-122 §2, issue-130, issue-131, issue-72 for 529; issue-82 for 500); reconcile F21's 8 vs 7 by dated amendment; extend AC-218's
GIVEN to 'exits with any code'. (iii) Dated amendments: AC-068 -> '36 of the 41 rows (35 the phrase exactly, one a shorter variant of it; four rows carry no reason at all and one is P01's)' matching
F40 section 1, and add the AC register to the census seed's file list; design:1864 -> 'plus one single-line wording change (P01)'; AC-049's strike note -> cite spikes/F35-DRAFT.md section 1.3 and drop
'fabricated' at :1282 and 'unfounded' at :183 (ADR-0073 untouched); add the one-clause text_diff qualification to design row 1862's status cell, where the other METHOD-S section 7 facts were
propagated.

**Folds.** slice-G18-07, G19-03, G13-03, L12-authority-05, slice-G07-05, slice-G04-05, slice-G02-01, slice-G02-10, slice-G03-03, slice-G03-08

### PIA-081 — E3 severity adjudication closed 2026-09-09 NOT BLIND: design 1.1 row 1, AC-357 and six AC/SM sites re-issued after closure still carry it open/PROVISIONAL with the stale 4/23 figures, and design 6.4.4:956 alone calls it 'blind'

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B04-C08] ARCH-0.1-013 (design:31 RESOLVED) and ARCH-6.6-012 (design:1049 NOT BLIND) record the settled side; AC-1.2-011 (AC:191), AC-PROV-026
(AC:3989) and SM-5.2-007 (SM:1387) record 'open'/PROVISIONAL as current. No row records that they contradict, anchors design:956, or the word 'blind' as a claim. The 1.1 row (design:71) is unrecorded.
| [B05-C08] ARCH-6.6-012, ARCH-1.2-031, ARCH-10.4-004 (closure, design side) and SM-5.2-007, AC-1.2-011, AC-PROV-025/026 (the 'open' rows) held as separate owed_by_human rows; no row records the
disagreement, the re-issue past closure, or design:956's 'blind') · **Category:** contradiction · **Confidence:** high · **Owner:** design author (Claude); AC and SM register author

Merged from 2 concerns; each member statement is carried verbatim. (i) Design 0.1 row 31 and 1.2:123 record the E3 sheet filled (4 BLOCKING / 2 ADVISORY), FP 1/23 and 0/23, MCC +0.978 / +1.000,
DECIDED 2026-09-09, and row 31 itself says 'earlier rows called the sheet returned unfilled; that was stale' [measured]. That correction did not reach three places: design 1.1 row 1 - the row
ADR-0001/0002 rest on - still carries 4/23, 2/23, +0.839, +0.917 with 'PROVISIONAL ... severity is unadjudicated' (the only remaining '4/23' in the design; AC-357:4388 carries the same stale phrase);
the AC register (:191, :3858, :3866, :3989) and the SM register (:642, :1161, :1387), both amended 2026-09-09 and 2026-09-10, still say 'the E3 severity adjudication is open' and hold AC-308 and
SM-086 PROVISIONAL on that ground, the AC register declaring itself binding on the Phase-1 runbook [measured]. In the opposite direction, design:956 - the SM-086/AC-308 row of the telemetry table that
defines the false-block measure - says 'severity is Ryan's, one operator, blind', the only site in the corpus making that claim, while the sheet's own header, 0.1 row 31, 1.2:123, 6.6:1049, 10.5:1873
and RUNBOOK:881 say NOT BLIND (filled by the model at Ryan's direction, kappa forfeited) [measured]. The decision direction is unchanged under either figure set, and AC-308's rule is identical under
either status; ADR-0088 clause 5 forbids presenting CLEAN-derived rates as calibration evidence, so a 'blind' label on the provenance row strengthens exactly the claim the corpus withdraws. (ii)
Design :123, :153, :179, :956 and :1049 record the E3 severity adjudication RESOLVED/CLOSED 2026-09-09 (4 BLOCKING, 2 ADVISORY, NOT BLIND, κ forfeited) and §10.5 marks it DONE. The AC register (§1.2
row 11 at :191, AC-308 at :3866, §9.8 residual at :3989) and SM register (SM-086 at :642, :1161, :1387) still say it is open and keep AC-308/SM-086 PROVISIONAL for that retired reason; the SM header
records amendments on 2026-09-09 and 2026-09-10 with 'No id is renumbered', so the stale condition survived two re-issues [measured]. The design's surviving reason for provisionality (the sheet is not
an independent rater) and its '0 of 14 adjudicated' compliant denominator appear in neither register. Separately, design :956 characterises the severity annotation as 'Ryan's, one operator, blind'
while the artefact it rests on (spikes/results/e3-severity-annotation-ryan.md:3) says 'Filled by the model at Ryan's direction … NOT BLIND' and three other sections of the same document (:123, :1049,
:1873) carry the correct characterisation — overstating the independence of the only false-block denominator SM-086 has [measured].

**Anchors.**
- `references/2026-09-08-architecture-design.md:71` — "CLEAN false positives 4/23 vs 2/23 and MCC +0.839 vs +0.917 are **PROVISIONAL and do not belong in a settled column** (§0.1): severity is unadjudicated"
- `references/2026-09-08-success-measures.md:642` — "- **PROVISIONAL:** the adjudication this baseline rests on has settled *facts* (6/6) but **not severity** — the E3 severity adjudication is open [brief §E] — so the 0/23 is provisional in the same way AC-308 is."
- `references/2026-09-08-architecture-design.md:31` — "| False-positive rates and MCCs from E3 | **DECIDED** (was PROVISIONAL; resolved 2026-09-09) | **RESOLVED 2026-09-09.**"
- `references/2026-09-08-success-measures.md:3` — "**Status:** Register, **amended 2026-09-09** to absorb F39 (P1 arm D scored) and ADR-0089"
- `references/2026-09-08-architecture-design.md:956` — "The E3 severity adjudication **closed 2026-09-09** (facts settled 6/6; severity is Ryan's, one operator, blind), so the two FP rates and two MCCs are settled"
- `references/2026-09-08-architecture-design.md:1049` — "| **E3 severity adjudication** — **CLOSED 2026-09-09** | Facts settled 6/6"

**Why it matters before Step 5.** (i) No build impact; the record of the false-block measure's provenance and the binding register's status for AC-308 are what is wrong. (ii) No build impact. AC-308
is the false-block metric Phase 1 must implement; whether it is PROVISIONAL, why, and whether its baseline is 0/14 adjudicated or 0/23 differ between the two registers Phase 1 is bound to. A reader
deciding whether AC-308/AC-298's default weight may now be promoted gets two answers.

**Remediation.** (i) Update design line 71 to the settled figures with the 'still upper bounds; sheet NOT BLIND' caveat matching line 123, and re-tag AC-357's 'severity unadjudicated' (AC:4388) the
same way. Edit design:956 to '(facts settled 6/6; severity from the 2026-09-04 sheet, filled by the model at Ryan's direction, NOT BLIND, kappa forfeited)'. Amend AC-308's PROVISIONAL note, AC 1.2 row
11, AC 9's residual-risk bullet, SM-086's PROVISIONAL bullet and SM:1161/1387 to 'severity settled 2026-09-09 on a NOT-BLIND, model-filled sheet (kappa forfeited); the rates remain upper bounds
against CLEAN', and either lift PROVISIONAL from AC-308/SM-086 (updating AC 9's state line) or keep it with the new ground stated as the missing blind annotation, not an open adjudication. (ii) Amend
SM-086 (three sites) and AC §1.2 row 11 / AC-308 / §9.8 residual to 'adjudicated 2026-09-04, closed 2026-09-09, NOT BLIND, one operator; still PROVISIONAL because the sheet is not an independent rater
(pending E3-SEV's blind annotation), not because it is open', and carry §6.4.4's '0 of 14 adjudicated' as SM-086's compliant baseline. Amend design §6.4.4:956 to 'severity filled by the model at
Ryan's direction, NOT BLIND, κ forfeited (§6.6)'.

**Folds.** G05-05, G09-04, G09-06, G15-07, G15-08, G16-09

### PIA-088 — §10.5/open-item rows vs ADR-0027/0012: pack 'taggable' with 101/102 unrun; §8.10 preamble false for four rows; S-CASCADE registration not propagated

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ADR-0027-NEG-51 (executed-once as a promotion gate); ARCH-8.10-006 (the distribution row's content); ARCH-6.6-014 @ design:1051 and ARCH-10.5-026
@ design:1885 (stale row / registration). None records that HANDOFF/P6-PACKS declare the pack taggable without the gate, that no AC checks it, that §8.10's preamble is false for its own table, or that
§6.5.4 and AC-301 still carry 'none is registered'.) · **Category:** contradiction · **Confidence:** high · **Owner:** Ryan (ADR-0027 decision); Claude (design/AC authors)

(a) ADR-0027:17 decides every shipped activity must pass P6 lint 'and be executed at least once before the pack is tagged' and :56 forbids tagging otherwise; HANDOFF:237 declares 'the pack is
taggable' on the §3.4 lattice criteria alone and design §10.5:1891 prices tagging at '$0 — a decision plus a lattice re-run'. Only root-cause has been executed through the rig (F37, 61 runs); grep of
the AC register, design, SM and RUNBOOK for 'executed at least once' returns 0 hits, so no criterion carries the gate and SM-036's usage half cannot fire for a quarter [measured]. (b) §8.10's preamble
(design:1621) claims 'Every row resolves to a named spike with a named consumer in §10 (ADR-0012); none is discharged as prose'; four of its eleven rows resolve to 'a distribution ADR of its own —
none exists', 'a declared constant … not a spike', '—', and 'ADR-0013's unexecuted consequence' [measured]. The distribution question is tracked as an open item (AC:190; design:1630) but not as a §10
spike with a consumer, which ADR-0085 rule 9 and ADR-0012:43 require. (c) §10.5 registers S-CASCADE with AC-301 as consumer and :1885 says §6.6 'previously read "No spike registered"', yet the §6.6
row (:1051), §6.5.4 (:1029) and AC-301 (:3770, :3775) still say none is registered; 'previously' is inaccurate because the row was not updated [measured].

**Anchors.**
- `references/adr/0027-curated-catalog-with-unshipped-extended-file.md:56` — "- It forbids tagging the pack with any shipped activity that has not passed P6 lint (reachable from some context) or has never been executed — an activity that is authored but dead-on-arrival cannot ship."
- `spikes/HANDOFF.md:237` — "Ryan reviewed the 59 bumps, the four planning stages and R3/R7 (DECISIONS §Review) — the pack is taggable. Remaining: (1) tag the pack;"
- `references/2026-09-08-architecture-design.md:1891` — "tag the v0 pack; keep or strike the `optional`-override mode | SM-026, SM-027; ADR-0027; every 12-pack reading in §4.6 | **$0** — a decision plus a lattice re-run at **27 s** [measured, F35]"
- `references/2026-09-08-architecture-design.md:1621` — "Every row resolves to a named spike with a named consumer in §10 (ADR-0012); none is discharged as prose."
- `references/2026-09-08-architecture-design.md:1051` — "| **Cascade recall loss** | No spike registered | AC-301 |"
- `references/2026-09-08-acceptance-criteria.md:3775` — "measures the cascade's recall loss; none is registered today."

**Why it matters before Step 5.** None of the three gates Phase 1: the pack tag is a release gate, the plain clone is decided, and AC-301's refusal ships regardless. (a) is a DECIDED plan item
contradicting an accepted forbids block; (b) is the sentence a reader relies on to confirm nothing was discharged as prose, at exactly the place four items were; (c) leaves three sentences ADR-0012
forbids in force alongside the fix.

**Remediation.** (a) Either supersede that clause of ADR-0027 with a record narrowing the executed-once gate to the extended-to-shipped promotion path, or add the gate to P6-PACKS and an AC
(executed-once ledger per activity keyed by run id) and stop calling the pack taggable in HANDOFF and §10.5. (b) Amend §8.10's preamble to 'seven rows resolve to a named spike; four resolve to a
decision, a constant or a cut, as stated', and either register the distribution question as a zero-cost decision spike with the CONDITIONAL rows as consumer or write the distribution ADR §8.8 says is
missing. (c) Update the §6.6 row, §6.5.4 and AC-301's evidence line to name S-CASCADE (registered, unrun, does not gate Phase 1).

**Folds.** slice-G18-10, slice-G10-07, G16-10

### PIA-090 — The narrowed novelty claim 'in an AI-coding-specific backlog or spec tool' is false: spec-kit, Archon and Bernstein ship persisted human gates

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (references/2026-09-08-evidence-augmentation.md:38 already narrowed claim 2 once ('Sharpen claim 2 ... to
"...in an AI-coding-specific backlog or spec tool"'); references/2026-09-08-architecture-design.md:854 carries that narrowing) · **Category:** positioning claim contradicted by live sources ·
**Confidence:** high · **Owner:** Ryan (architecture-design and evidence-augmentation errata)

Evidence-augmentation claim 1 says that none of spec-kit/OpenSpec/BMAD/task-master 'ships an item-kind taxonomy, a stage-gate state machine, or durable approval on a work item', and the design narrows
its differentiation claim to 'in an AI-coding-specific backlog or spec tool'. Both were already false at the 2026-09-08 snapshot: spec-kit 1.0.x ships a workflow engine with a `gate` step
(approve/reject, on_reject abort/skip/retry), persisted run state under .specify/workflows/runs/<run_id>/ and `specify workflow resume <run_id>` (PR #2158 merged 2026-04-14); Archon has an `approval`
node type with `archon workflow approve/resume/reject` and runs persisted in its database (v0.4.0, 2026-05-28); and Bernstein (1,158 stars, created 2026-03-22, absent from the corpus) is a
deterministic non-LLM scheduler with task kinds, policy-declared approval and merge gates, a replay journal, worktree-per-task and checkpointed resumable runs, the closest unlisted analogue to the
kernel pitch. What survives is narrower than either wording: the gate is attached to a per-invocation run, not to a work item with a kind and a stage machine, and no surveyed system binds the answer
to a tree hash. The process-catalog proposal's 'nobody combines an item-kind taxonomy, an executable activity catalog, and deterministic selection' needs a Bernstein row before it can be repeated.
[documented]

**Anchors.**
- `references/2026-09-08-architecture-design.md:854` — "The claim that survives is the narrower one: **in an AI-coding-specific backlog or spec tool**, and on the *binding* rather than the gate — redkiln binds the answer to a **tree hash**, which no surveyed system does, Jira included."
- `references/2026-09-08-evidence-augmentation.md:123` — "None ships an item-kind taxonomy, a stage-gate state machine, or durable approval on a work item. The gap widened"
- `references/2026-09-08-evidence-augmentation.md:124` — "Claim 2: no AI coding tool ships stage-gated, mid-run, resumable approval on a durable item"
- `references/2026-09-07-process-catalog-proposal.md:46` — "Competitors: nobody combines an item-kind taxonomy, an executable activity catalog, and deterministic selection."

**Why it matters before Step 5.** No code consequence: the kernel design does not change because competitors have gates. It is a positioning statement in the live design and the evidence register that
a reader with spec-kit open can falsify in a minute, and it should be reworded before it is repeated in a README or a charter.

**Remediation.** Reword design line 854 and evidence-augmentation claims 1-2 to the claim that holds: 'no surveyed tool attaches a persisted human gate to a durable work item that carries a kind and a
per-kind stage machine, and none binds the answer to a tree hash'. Add rows for spec-kit's gate step, Archon's approval node and Bernstein (plus Untrivial-ai/agent-orchestrator and
microsoft/conductor) to the competitor watchlist in evidence-augmentation §5, each with url and fetch date, and add a Bernstein row under the process-catalog proposal's claim 3. Whether Bernstein's
approval gates are human or automated is not settled by its README and goes on the next-pass check list, not into prose. Owner: design author.

**Sources.**
- https://github.github.io/spec-kit/reference/workflows.html (fetched 2026-09-10) — 'gate | Pause for human approval before continuing ... Run states: created ... paused, failed, aborted ... Resumes a paused or failed workflow run from the exact step where it stopped'
- https://github.com/coleam00/Archon (fetched 2026-09-10) — approval node type; 'archon workflow approve/resume/reject'; release v0.4.0 2026-05-28 'Web DAG builder recognizes loop and approval node types'; runs persisted in Workflow Runs table; 23,427 stars
- https://raw.githubusercontent.com/sipyourdrink-ltd/bernstein/main/README.md (fetched 2026-09-10) — 'A deterministic scheduler - no model in the coordination loop - runs agents in parallel, gates what they produce ... Run state checkpoints to .sdd/runs/<run_id>/ on every node ... resume <run_id>'

**Folds.** BP-SDD-01, BP-SDD-02, BP-SDD-19

### PIA-094 — Spike-register gate verdicts drift from the governing table: P2 marked YES against 0.1's No; P7 left the 'design final' gate set unrecorded

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (MP-AMEND-001 (METHOD-P:240) and ARCH-0.1-007 (design:25) each hold one side of the design-final gate set. ARCH-10.4-003, ARCH-10.5-004 and
ARCH-0.1-003 are census extractions of the P2 rows themselves; the P2 verdict disagreement is unrecorded (NEW within this concern).) · **Category:** contradiction · **Confidence:** high · **Owner:**
Ryan Britton (METHOD-P amendment, design 0.1); design author (Claude) for 10.4/10.5

Design 10.4:1818 and 10.5:1863 give P2 a 'YES' gates-Phase-1 verdict scoped to the llm-facet surface - the P1-D column's wording - although P2 measures derived_from staleness and no facet; 0.1 row 21
says 'No - the step-3 mechanism ships', 10.5's own P2-B row says No, and RUNBOOK 4.1 says P2 'neither touches the selection path'; line 1812's claim that 'Gates Phase 1?' reproduces 0.1 exactly is
false for this row, and `git log -S` dates the YES wording to 4519edc, when P2's consumer was AC-076, since moved [measured]. For the 'design final' label, METHOD-P 6 registers P6, P7 and P8 as its
gates and HANDOFF:214-215 treats P6 and P7 as discharged because they 'now have results'; the design's header, 0.1 row 25 and 10.6:1918 name S-BIN, S-H1c and P8's unrun arms instead, and P7's result
is the one 0.1 row 24 calls powerless at n=3 with S-P7B 'Not designed' (design:1870), so HANDOFF's discharge rests on a result the design refuses and no dated amendment to METHOD-P records P7's exit
or S-BIN/S-H1c's entry [measured]. RUNBOOK Step 4 and Checkpoint 4 item 4 fence the P2 build reading, and the label gates nothing in the build; what is wrong is the register's fidelity claim and the
frozen pre-registration's stricter discharge condition.

**Anchors.**
- `references/2026-09-08-architecture-design.md:1863` — "the perturbation half cost nothing at all [measured, F40] | **YES** for the `llm`-facet surface | **PARTLY DONE 2026-09-10.**"
- `references/2026-09-08-architecture-design.md:1812` — "Every row names a consumer. "Gates Phase 1?" reproduces §0.1 exactly; no row is softened here."
- `references/2026-09-08-architecture-design.md:21` — "| **No** — the step-3 **mechanism** ships and is unit-tested (AC-066, AC-067; ADR-0025)"
- `RUNBOOK.md:1012` — "**neither touches the selection path**. So P2 **sits outside the blocking gate**."
- `spikes/METHOD-P.md:240` — "**P7, P8 and P6 gate "design final"**; P1 and P2 set parameters."
- `references/2026-09-08-architecture-design.md:3` — "**It is not "design final".** Three spikes gate that label — **S-BIN**, **S-H1c** and **P8's unrun arms** —"

**Why it matters before Step 5.** No build impact. A reader of 10.5 alone would treat P2 as a Phase-1 gate on the facet surface and could wait on P2-B before building; a reader of METHOD-P holds the
design-final label blocked on S-P7B while a reader of the design releases it after P8-B, S-BIN and S-H1c.

**Remediation.** Amend design 10.4:1818 and 10.5:1863 to carry 0.1's verdict verbatim ('No - gates the stale_policy setting, not the phase; the step-3 mechanism ships unit-tested'), dropping the
llm-facet clause that belongs to P1-D. Add a dated amendment to METHOD-P 6 (never edit the registered sentence) stating that P7's design-final gate is carried by S-P7B and either remains a gate on the
label or is released, and mirror that sentence in design 0.1 row 25's 'What unblocks it' cell and 10.6:1918.

**Folds.** slice-G02-02, G05-01

### 4.2 Inaccuracies (14)

### PIA-019 — The corpus's account of its own tree and instrument is stale: RUNBOOK 0.1/1.3 describe HEAD ce04058 and hard-code 'claude 2.1.266' (the instrument is 2.1.268) in a commit message said to quote a file, Checkpoint 1 accepts any 2.1.N, design:135 keeps a corrected residual, and F30 'never written' exists untracked in a linked worktree

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B06-C12] RUNBOOK:212's own hedge ('this enumeration has already been wrong once') and the census's runbook_claim_verification STALE rows on
168-171/206-216 cover the counts; H10-TRAP-006 covers spikes/HANDOFF.md; ARCH-1.2-043 (design:135) records HANDOFF as stale; ARCH-10.4-008 and SM-5.2-004 record 'F30 never written' as fact; reconcile
anomaly_pool records F30 as dangling-but-documented -- none records the verbatim Step 1.3 outcome, the self-contradicting cell, or the draft's existence | [B08-C04] RB-HALT-002 (RUNBOOK.md:2349) and
RB-GC-008 (RUNBOOK.md:106) record the lineage rule and the 2.1.263 -> 2.1.266 moves; the census anomaly pool records 2.1.268 as a fourth version. Neither records that line 290's 'quotes' claim is
false against 402's literal, nor that Checkpoint 1's regex cannot catch the mismatch.) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Ryan (RUNBOOK author; worktree owner); Claude
(design/brief/SM restatements)

Merged from 2 concerns; each member statement is carried verbatim. (i) The corpus's account of its own tree is older than the commit that carries it. [measured] RUNBOOK section 0.1 says 'HEAD is
`ce04058`' with 'eleven ` M` lines' and 'a single ` M` on `spikes/results/p2-labels-ryan.md`'; today HEAD is 009a4ec (eight commits on), ` M` count is 0 and `??` count 17, the F39 amendment landed at
a431c62 and the labels at a627547, and HANDOFF-2026-09-10:24 -- committed in the same commit as the RUNBOOK's last edit -- says 'Everything else is committed'. Run verbatim, Step 1.3's `git add` list
stages only the new instrument file under the message 'ADR-0089: facet provenance, declared surfaces, abstention -- F39 applied' plus a hard-coded instrument, and Checkpoint 1 still passes; Step 9's
salvage rationale (:2252) and checklist :2456 inherit it. Design:135 records the e3b0bac correction and then ends 'Residual: nothing from the AFK session was committed'; :1789's exit (a) carries the
uncorrected sentence; :904 says 'nine blocked rows' where section 0.1 counts ten; RUNBOOK:913 says 'F29-F39 live as drafts' when F30 is absent and F40 present. The brief (:12, :45), design (:49,
:1823), SM (:1384, :1421) and AC (:4, :6534) say F30 was 'never written' and its numbers 'exist only in the design-recommendation and the proposal'; a 202-line untracked `spikes/FINDINGS-DRAFT-F30.md`
(2026-09-06, house style, S-WT table matching s-wt.jsonl) sits in the linked worktree D:/repos/redkiln-rs.swt at 8b64201, an ancestor of main; main's tracked run prompt (run-s-h1b-24h-read.md:19, :98)
records it and instructed it be left uncommitted for Ryan's review; the raw records are tracked in main at spikes/results/s-wt.jsonl and s-h1b-{1m,65m,24h}.jsonl. The draft is unprotected (deleting
the directory or `git worktree remove --force` loses it), though not one `prune` away, and its prefix-decomposition section is by its own words a spike specification, not a correction. (ii)
`claude --version` returns 2.1.268 (Claude Code) on 2026-09-10 [measured]; the RUNBOOK's last measurement is 2.1.266 (247) and HANDOFF pins 2.1.263, so the programme has now straddled four versions.
Halt 2 covers the move as a halt (record, do not silently continue) and Checkpoint 0 writes the true value to spikes/results/instrument-checks/RUNBOOK-instrument.txt. But line 290 claims Step 1.3's
commit message 'quotes' that file, whereas 402 is the literal `claude 2.1.266`, and Checkpoint 1 (419) greps `claude 2\.1\.[0-9]+`, which any pasted 2.1.N satisfies: an operator who pastes the block
records the wrong instrument in the first branch commit and the checkpoint goes green. Golden constraint 8 names this exact failure class.

**Anchors.**
- `RUNBOOK.md:174` — "`spikes/p9/COMMIT-2026-09-09.sh` **has been run**, and so has much else since. **HEAD is `ce04058`, not `e3b0bac`**"
- `RUNBOOK.md:402` — "Instrument at time of commit: claude 2.1.266 (HANDOFF pins 2.1.263; P7 straddled"
- `RUNBOOK.md:388` — "git commit -m "ADR-0089: facet provenance, declared surfaces, abstention — F39 applied"
- `RUNBOOK.md:419` — "git log -1 --format=%B | grep -qE 'claude 2\.1\.[0-9]+' && echo "instrument named in the commit message""
- `spikes/plans/HANDOFF-2026-09-10-design-session.md:24` — "driver logs, `spikes/results/p8-state/`, `spikes/runner/target-{dev,p8}/`). Everything else is committed."
- `RUNBOOK.md:247` — "claude --version   # measured 2026-09-09: 2.1.266 (Claude Code)"

**Why it matters before Step 5.** (i) Not a build blocker; Checkpoint 1 passes either way. A wrong first-commit message on the build branch is the record defect golden constraint 8 is about, and the
only written-up S-H1b/S-WT finding -- the text that would close S-WT-REC and settle findings 01/02 -- is one directory deletion from loss while four live documents say it never existed. (ii) Step 1
precedes Step 5; the first branch commit would misstate the instrument the whole lineage rule keys on. Record defect, cheap to fix, no build consequence.

**Remediation.** (i) Rewrite section 0.1 to the measured state (HEAD 009a4ec, 0 ` M`, 17 `??`); reduce Step 1.3 to 'commit the instrument file' with a message that quotes the file
(`$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)`); update :2456 and Step 9's :2252 comment; delete the 'Residual' clause at design:135, correct :1789's exit (a) to the corrected
state, 'nine' -> 'ten' at :904, 'F29 and F31-F40' at RUNBOOK:913. Copy the F30 draft into the main checkout as `spikes/F30-DRAFT.md` (the drafts' naming convention), commit it, reword 'never written'
to 'drafted 2026-09-06, promoted <date>' at brief :12/:45, design :49/:73/:177/:1823, SM :49/:844/:1384, HANDOFF :210, and replace 'exist only in the design-recommendation and the proposal' with the
jsonl paths. (ii) Replace the literal at 402 with `$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)` inside a double-quoted heredoc; make Checkpoint 1 line 419
`grep -qF "$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)"`; add '2.1.268 measured 2026-09-10' to section 0.3 and to Halt 2's lineage list.

**Folds.** L08-05, G05-09, L10-register-integrity-03, L08-06

### PIA-021 — Step 2's pin script leaves the arm-L blind labels in one unpushed copy, stamps a literal PINNED_AT and re-admits the p8-state noise Step 1.2 excluded

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (RUNBOOK.md:2285-2287 (Step 9 s2b discloses single custody and orders copy-back); no known-issues.json row on PINNED_AT or on p8-state travelling)
· **Category:** operational-posture · **Confidence:** high · **Owner:** RUNBOOK author (Claude); Ryan Britton executes

Custody trace [measured]: Step 3b creates spikes/results/p1-arm-l-labels-ryan.md by `cp`; no `git add`/`git commit` exists between Step 1.3 (RUNBOOK:388) and Step 2 (:502); Step 2 copies results/ into
a freshly `git init`ed D:/repos/redkiln-corpus with no remote, then `rm -rf`s the redkiln-rs copy and gitignores the path. p1-arm-d-labels-ryan.md and p2-labels-ryan.md are tracked on main and survive
in history; the arm-L file, the one Step 2 calls the artefact 'no re-run can reproduce blind', is never tracked in redkiln-rs, so 'NOTHING ELSE HOLDS THEM' (:2285) is literally true for it alone, Step
9's 'they were tracked' (:2272) is false for it, and Step 9 then offers `rm -rf D:/repos/redkiln-corpus` (:2291). Step 9 s2b does disclose single custody and copies back before deletion; the residue
is the arm-L-only qualification, the missing remote, and that no line commits the file anywhere with an origin. In the same block, `PINNED_AT=2026-09-09` (:493) is a literal inside a quoted heredoc
although Step 2 runs sixth after a labelling session of unknown date, and Step 2's `cp -r` (:470) plus `git add -A` (:479) in a repo with no .gitignore commit spikes/results/p8-state/ (6 files, 19 KB,
gitignored at :346 as 'meaningless without the run that made it') into the pinned corpus [measured].

**Anchors.**
- `RUNBOOK.md:819` — "test -f spikes/results/p1-arm-l-labels-ryan.md || echo "MISSING: p1-arm-l-labels-ryan.md""
- `RUNBOOK.md:500` — "rm -rf spikes/corpus spikes/results spikes/runner"
- `RUNBOOK.md:2285` — "#     They are Step 3b's output, they travelled into redkiln-corpus with results/, and NOTHING ELSE"
- `RUNBOOK.md:2272` — "#    branch, `git checkout main` restores all three directories from main's tree — they were tracked."
- `RUNBOOK.md:493` — "PINNED_AT=2026-09-09"
- `RUNBOOK.md:479` — "git add -A && git commit -m "redkiln-corpus v0: pinned corpus, spike results, spike runner (ADR-0013)""

**Why it matters before Step 5.** Step 2 is on the critical path to Step 5; the moment it runs, an unreproducible human artefact has one copy on one disk with no remote. Per L08-10 the arm-L file has
no Phase-1 consumer before a re-run, so its loss blocks nothing; the pin is the AC-464 authority, so a wrong date in it is a record defect.

**Remediation.** Add to Checkpoint 3b (or the head of Step 2, before `git rm -r --cached`):
`git add spikes/results/p1-arm-l-labels-ryan.md && git commit -m "P1 arm L: blind labels, labelled_at <stamp>, instrument <version>"` so redkiln-rs history holds it; add a `git remote add` or
`git bundle` line to Step 2; correct :2272 to say arm L is restored only by s2b. Replace the literal with `PINNED_AT=$(date -u +%F)`; add `rm -rf ./results/p8-state` beside :472.

**Folds.** L08-03, L08-13

### PIA-024 — RUNBOOK pins rustc 1.97.1 and greps for '1.97' while stable is 1.98.1; R-ASYNC is discharged negative through 1.98.1 but reads 'Not started'

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ARCH-10.5-017 (architecture-design:1876, R-ASYNC 'Not started'); EA-5-001 (evidence-augmentation:271, 'read the Rust 1.97 release notes
directly'); ADR-0083-NEG-96 (a pinned toolchain lags stable by design)) · **Category:** toolchain/version-currency · **Confidence:** high · **Owner:** Step 5 implementer; RUNBOOK owner

RUNBOOK:235-236 expects 1.97.1 'or later', RUNBOOK:1656 writes the exact pin `channel = "1.97.1"`, and the Step-5 gate at RUNBOOK:1758 greps for the literal `channel = "1.97`. Rust 1.98.0 shipped
2026-08-20 and 1.98.1 on 2026-09-03, so the pin recorded on 2026-09-08 was already one minor behind stable when measured; it records the installed toolchain, not current stable [documented]. The three
lines are internally inconsistent: an implementer who honours 'or later' and pins 1.98.1 fails the gate at :1758, and one who copies :1656 pins a version two releases old on day one. Separately, the
corpus's own deferred check is now done: the release notes for 1.97.0, 1.97.1, 1.98.0 and 1.98.1 plus the tracking issue were read in full and none stabilises `async_fn_in_dyn_trait` [documented], so
ADR-0076's working assumption holds through 1.98.1 and R-ASYNC (architecture-design:1876) can be recorded as discharged rather than 'Not started'. The corpus was not wrong, only unverified; the stale
numbers are the pin string and the gate grep.

**Anchors.**
- `RUNBOOK.md:235` — "rustc --version   # expect 1.97.1 (8bab26f4f 2026-07-14) or later"
- `RUNBOOK.md:1656` — "channel = "1.97.1""
- `RUNBOOK.md:1758` — "test -f rust-toolchain.toml && grep -q 'channel = "1.97' rust-toolchain.toml"
- `references/2026-09-08-architecture-design.md:1876` — "| **R-ASYNC** | Does Rust 1.97 ship any dyn-async-trait stabilisation? | ADR-0076; AC-480, AC-481 | Reading | No | Not started |"
- `references/2026-09-08-architecture-design.md:387` — "as of 2026-09-08 a dyn-compatible native async fn in trait does not exist on stable Rust, and the evidence is **search-only** — nobody read the Rust 1.97 release notes [evidence-augmentation §5 row 1]."
- `references/2026-09-08-architecture-design.md:425` — "| Rust 1.97 dyn-async-trait stabilisation unread | ADR-0076's working assumption is **search-only** | The decision is correct under either answer; the check is a release-notes plus tracking-issue read at every toolchain-pin bump | toolchain-pin bump review · consumer: ADR-0083's pin | No |"

**Why it matters before Step 5.** Step 5 writes rust-toolchain.toml and runs the :1758 gate; the pin must be chosen once, explicitly, and the gate must match it, or Step 5 either fails its own check
or starts on a toolchain the project will bump within its first weeks.

**Remediation.** At Step 5, decide the pin as a RUNBOOK line (the RUNBOOK is operator-facing and editable): either keep 1.97.1 as the measured baseline and change :235-236 from 'or later' to
'exactly', or pin 1.98.1 and change :1656, :283, :2455 and the :1758 grep to `channel = "1.98.1"`. Whichever, the grep at :1758 must test the full pinned string, not a prefix. Record R-ASYNC as
discharged in the design's §10 register with the four release-note URLs and the tracking issue as the evidence, dated 2026-09-10, consumer unchanged (the toolchain-pin bump review, ADR-0083); the
assumption 'still requires `async-trait` for dyn ports' stands and ADR-0076 is untouched.

**Sources.**
- https://blog.rust-lang.org/releases/ (fetched 2026-09-10) — Rust 1.98.1 - September 3, 2026; Rust 1.98.0 - August 20, 2026; Rust 1.97.1 - July 16, 2026; Rust 1.97.0 - July 9, 2026
- https://static.rust-lang.org/dist/channel-rust-stable.toml (fetched 2026-09-10) — stable channel manifest: 1.98.1 (48a229cea 2026-09-01)
- https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/ (fetched 2026-09-10) — release notes read in full: no mention of async fn in dyn trait / async_fn_in_dyn_trait
- https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/ (fetched 2026-09-10) — release notes read in full: no dyn-async-trait stabilisation
- https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md (fetched 2026-09-10) — 1.97.1 and 1.98.1 point releases: no dyn-async-trait stabilisation; async_fn_in_dyn_trait remains unstable

**Folds.** EXT-TOOLCHAI-001, EXT-TOOLCHAI-005

### PIA-025 — RUNBOOK 5.1's verbatim root Cargo.toml fails `cargo clippy` on the pinned toolchain (lint_groups_priority); Checkpoint 5 cannot exit 0

**Priority:** P2 · **Classification:** NEW — none (ADR-0083-FORB-105 is the group-deny forbids clause only; no row mentions lint priority) · **Category:** buildability · **Confidence:** high ·
**Owner:** RUNBOOK author (Step 5)

The root manifest RUNBOOK 5.1 gives as copy-in text (lines 1620-1646) puts the group `nursery = "warn"` at implicit priority 0 alongside the enumerated deny-level lints. Reproduced on cargo/clippy
1.97.1, the exact pin: `cargo build` exits 0; `cargo clippy --workspace --all-targets` exits 101 with `error: lint group `nursery` has the same priority (0) as a lint` [measured]. The lint is a
deny-by-default correctness lint, not one promoted by `all = "deny"`. Adding `priority = -1` to `all` and `nursery` makes clippy exit 0 [measured]. Only Checkpoint 5's clippy line (RUNBOOK:1755, 'exit
0, zero warnings') fails; no qualifying sentence in RUNBOOK 1617-1649 discloses the priority rule. Deterministic but self-diagnosing (clippy prints the fix), so a wrong verbatim text in the record
rather than a blocked workstream.

**Anchors.**
- `RUNBOOK.md:1637` — "all = "deny""
- `RUNBOOK.md:1646` — "nursery = "warn""
- `RUNBOOK.md:1755` — "cargo clippy --workspace --all-targets                   # exit 0, zero warnings"

**Why it matters before Step 5.** Checkpoint 5 cannot be discharged with the manifest as written; the fix is one line and clippy names it, but the RUNBOOK presents the manifest as copy-in text and the
checkpoint as exit 0.

**Remediation.** In RUNBOOK 5.1 write `all = { level = "deny", priority = -1 }` and `nursery = { level = "warn", priority = -1 }` (Cargo's documented group/lint priority rule) and add the same note to
design 9.3 'Lints - content'. No ADR change: ADR-0083 states levels, not table syntax.

**Folds.** G21-01

### PIA-026 — ADR-0083:68 prescribes a per-crate manifest `[lints]` override that Cargo rejects beside `workspace = true`; :70 names the working route

**Priority:** P2 · **Classification:** NEW · **Category:** toolchain/lint-policy-mechanism · **Confidence:** high · **Owner:** ADR-0083 successor author; Step 5 implementer

ADR-0083:63 requires every member to carry `[lints] workspace = true` 'and nothing else'; ADR-0083:68 then says a crate that needs `unsafe_code` 'overrides in its own manifest's `[lints]`'. Cargo does
not allow both: RFC 3389 makes any other key beside `workspace = true` in `[lints]` a hard error, and rust-lang/cargo #13157 ('cannot override workspace.lints in lints'), which the ADR cites as
support, is the open issue asking for exactly the override :68 prescribes [documented]. The route that works is the one :70 already permits, a commented `#![allow(unsafe_code)]` source attribute 'for
something the manifest cannot express', and RUNBOOK:1649-1650 is consistent with it. The evidence ledger also cites the wrong Cargo Book page: /cargo/reference/lints.html documents Cargo's own lints
and shows only `[workspace.lints.cargo]`; the `[workspace.lints.rust]` / `[lints] workspace = true` mechanism is on the Workspaces and Manifest pages [documented]. The lint policy itself is
unaffected.

**Anchors.**
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:63` — "**Lints.** `[workspace.lints.rust]` and `[workspace.lints.clippy]` live in the root manifest; every member crate carries `[lints] workspace = true` and nothing else. Within that table:"
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:68` — "- `unsafe_code = "deny"` at the workspace level. A crate that genuinely needs it overrides in its own manifest's `[lints]` with a comment naming why — the point of the workspace table is that the exception is then the only thing that looks different."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:70` — "A per-crate `#![allow]`/`#![deny]` in source is permitted only as a visible, commented override for something the manifest cannot express, and never as a way to restate workspace policy."
- `references/2026-09-08-evidence-augmentation.md:335` — "Cargo Book, Lints — https://doc.rust-lang.org/cargo/reference/lints.html (with rust-lang/cargo #13157) ·"
- `RUNBOOK.md:1649` — "Every member carries `[lints] workspace = true` **and nothing else**. No `#![deny]` / `#![allow]` in source as a"

**Why it matters before Step 5.** Step 5 writes the workspace lint table; the first adapter crate that needs `unsafe` (the design already anticipates one, evidence-augmentation:148) will follow :68
and get a cargo manifest error, then have to discover :70 on its own.

**Remediation.** ADR-0083 is accepted and immutable: record a superseding clause (a short ADR, or a clause in the next toolchain ADR) stating that the only per-crate `unsafe_code` exception is a
commented `#![allow(unsafe_code)]` source attribute, that `[lints]` in a member manifest carries `workspace = true` and nothing else because Cargo rejects any mix (RFC 3389; cargo #13157 open), and
re-citing the Cargo Book Workspaces page. Add the corresponding line to RUNBOOK §5.1 beside :1649 so the Step 5 implementer never reads :68 unqualified.

**Sources.**
- https://doc.rust-lang.org/cargo/reference/workspaces.html (fetched 2026-09-10) — The workspace.lints table is where you define lint configuration to be inherited by members ... [workspace.lints.rust] unsafe_code = "forbid" ... [lints] workspace = true ... MSRV 1.74
- https://doc.rust-lang.org/cargo/reference/lints.html (fetched 2026-09-10) — documents Cargo's own lints; shows only [workspace.lints.cargo], never the rust/clippy tables
- https://github.com/rust-lang/cargo/issues/13157 (fetched 2026-09-10) — open: 'cannot override workspace.lints in lints' - packages cannot override inherited lints; mixing workspace = true with other keys is an error (RFC 3389)

**Folds.** EXT-TOOLCHAI-003

### PIA-034 — Checkpoint 5's snapshot guard fails for the ADR-0083 test layout, and no RUNBOOK line accepts the initial insta snapshots

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (RB-GAP-006 (RUNBOOK.md:1766, the false-pass reshaping of the same line); ADR-0084-NEG-89 (adr/0084:90, cargo insta accept as rubber-stamp) --
neither records the false-fail for the conformant layout or the missing initial accept) · **Category:** runbook-step5 · **Confidence:** high · **Owner:** RUNBOOK Step 5 author

insta stores snapshots in a `snapshots` directory beside the file containing the assertion. Under ADR-0083's mandated `tests/it/main.rs` layout they land at `crates/rk-types/tests/it/snapshots/`, so
Checkpoint 5's `test -d crates/rk-types/tests/snapshots` guard -- added to stop a false pass -- fails deterministically for the conformant implementer and passes only for the one-file-per-case layout
the ADR rejects [measured: reproduced with insta 1.48.0; first `cargo test` exits 101 with 'stored new snapshot ...tests/it/snapshots/it__schema__schema_snapshot.snap.new']. The RUNBOOK installs
cargo-insta (1688) and names `cargo insta accept` only as a rubber-stamp hazard (1745); on a first run insta writes `.snap.new` and the assertion fails, so `cargo test --workspace # exit 0` cannot
hold until an accept that no line instructs. Both halves are Checkpoint 5 lines: the first checkpoint of the first product crate fails twice for an implementer who follows the ADR.

**Anchors.**
- `RUNBOOK.md:1761` — "test -d crates/rk-types/tests/snapshots \"
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:74` — "at most one integration-test binary per crate under test, structured as `tests/it/main.rs` with `mod` declarations for the cases — never one file per case."
- `RUNBOOK.md:1688` — "cargo install cargo-insta --locked"

**Why it matters before Step 5.** Step 5 itself. Nothing is built wrong, but the conformant implementer hits two unexplained deterministic failures at the first checkpoint, and the non-conformant one
passes.

**Remediation.** RUNBOOK Step 5 (a dated RUNBOOK edit; no ADR is touched): change the guard at 1761 to `crates/rk-types/tests/it/snapshots`, or pin the path in the test with
`insta::Settings::set_snapshot_path` to the path the checkpoint names; insert `cargo insta test -p rk-types --accept` with a one-line review note ('inspect the .snap before accepting') before
Checkpoint 5, keeping the rubber-stamp warning at 1745.

**Folds.** G21-07

### PIA-041 — reconcile.rs: 'validated on 360 commits / 74 slices' survives [measured] in ADR-0002/0003 and five design lines after F12 disclaimed validation and the register cut it as 'fabricated'; the file exists (spikes/runner/src/reconcile.rs, 420 lines), has no RUNBOOK source path after Step 2 deletes spikes/runner, AC-443 describes it as an import from a tree holding zero .rs files, AC-444's oracle is the spike's own output, and F12/F19 are cited for worktree and clone behaviour they never measured

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B02-C08] none (ARCH-0.1-037, ADR-0002-FORB-51, ARCH-1.2-043 record the import and the corpus pin; census OH-2 covers Step 7c's five files only;
Step 6 is not in the anomaly pool) | [B07-C07] SM-NOBASE-071 (references/2026-09-08-success-measures.md:916) records the F12 limit at SM-081; FND-LIM-007/010 (spikes/FINDINGS.md:437) record the
disclaimer; ARCH-0.1-037 (design:55) harvests the claim as a scope_limit with the 'validated' wording uncorrected; ARCH-0.1-019 records row 37. No row records that two accepted ADRs and five further
design lines still assert 'validated' after the AC-184 cut, that AC-444's oracle is the spike's own output, or the F12/F19 misattribution.) · **Category:** buildability · **Confidence:** high ·
**Owner:** Ryan Britton (RUNBOOK operator); Claude (AC-443 amendment)

Merged from 2 concerns; each member statement is carried verbatim. (i) RUNBOOK Step 6 gives no source location for reconcile.rs, the one sanctioned code import (design section 11:1653; ADR-0002),
though Step 2 (canonical position 6, RUNBOOK:500) deletes spikes/runner before Step 6 (position 10) and the RUNBOOK contains 'reconcile.rs' zero times [measured]; after Step 2 the file survives only
in redkiln-corpus at CORPUS_PIN. `git ls-tree -r 47faec9f` on the old redkiln lists 2,444 paths and zero `.rs` files, so AC-443's manifest of files 'imported from the old redkiln' describes a file
that is this repo's spike artefact, not an import from that tree [measured]. AC-443's own instrument-limit note (AC:5521-5523) concedes the sanctioned import would not be detected by its hash
tripwire, so the criterion's THEN is not broken; what is missing is the lift instruction and a truthful provenance origin. (ii) [measured] F12's Limits (:437) say 'the reconcile was not validated
against the agent's decisions' (n = 1 recorded decision); the register cut AC-184's 'validated on 360 / 74' claim as unsupported by the brief (:2468), SM-081 (:916) records it as 'a slice-state
result, not a validation of reconcile', and design §5.7 :725 disclaims it. Yet design :55/:184/:215/:252/:1653 still say 'validated', :1456 tags a not-yet-built replay's outcome ('reproducing 15 of 74
... 74/74 slice states') [measured, F12], and the accepted ADR-0002 (:22, :57) and ADR-0003 (:64) assert it under a [measured] tag. Design :293, §0.1 row 37, ADR-0003 :22/:44/:64 and AC-483 :6006
further attribute 'measured behaviour' for trailers, worktrees, disposable clones and reconcile() to F12 and F19; both findings contain zero occurrences of 'worktree' or 'clone' — worktree behaviour
is measured in F13/F16 (arm d, :586), and 'clone' occurs nowhere in spikes/FINDINGS.md, so disposable clones have no measuring finding at all. AC-444's replay test compares the product's reconcile
against s02-reconcile.jsonl, which is the spike's own reconcile.rs output: a self-consistency regression, not a validation, and the AC's 'without manual annotation' wording hides that. The AC-184 cut
over-corrects the other way — a Rust reconcile was built (spikes/runner/src/reconcile.rs produced F12); what was never done is validation. The decision (import reconcile.rs) stands either way.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:5515` — "GIVEN the file-provenance manifest that records every file imported from the old redkiln"
- `references/2026-09-08-architecture-design.md:55` — "`reconcile.rs` is the one code import, rewritten in the spike and validated on 360 commits / 74 slices [F12]"
- `references/2026-09-08-acceptance-criteria.md:5519` — "*Verified by:* a content-hash manifest test (`provenance.toml` vs. hashes of the old tree at pin `47faec9f`); the"
- `spikes/FINDINGS.md:437` — "- **The reconcile was not validated against the agent's decisions**, because …"
    those decisions were almost never recorded (n=1)."
- `RUNBOOK.md:500` — "rm -rf spikes/corpus spikes/results spikes/runner"
- `references/adr/0002-clean-break-self-hosting-switch.md:57` — "- [measured] `reconcile.rs` was rewritten in the spike and validated on 360 commits / 74 slices (F12)."

**Why it matters before Step 5.** (i) Nothing at Step 5. Step 6's implementer has no source path for the import and cannot write the provenance.toml entry as specified; recoverable from the corpus
pin, so not blocking. (ii) No Step-5 impact on what is built; it determines what rk-git's replay test (AC-444) is allowed to claim and whether two accepted ADRs carry a [measured] tag for behaviour no
finding measures.

**Remediation.** (i) In RUNBOOK Step 6.3 add the lift instruction: copy spikes/runner/src/reconcile.rs from D:/repos/redkiln-corpus at CORPUS_PIN into crates/rk-git/src/reconcile.rs and record its
origin SHA in provenance.toml. Amend AC-443 (dated amendment to the register) so the manifest records provenance from both the old tree and the spike runner, two hash sources in one entry, and drop
'imported from the old redkiln' for this file. Record the same hazard class alongside census OH-2 so Steps 6 and 7c are fenced together. (ii) Edit design :55/:184/:215/:252/:1456/:1653 to 'exercised
over 360 commits / 74 slices; not validated against recorded decisions (F12)'; re-cite trailers and reconcile() → F12/F19, worktrees → F13/F16, and drop or retag [documented] the disposable-clone item
in §0.1 row 37, §3.3 and AC-483; mark AC-444 as a regression oracle against the spike's own output; correct the AC-184/SM-051 cut to 'built, not validated'. For ADR-0002/0003 add an erratum in the ADR
README's open-items block ('validated' → 'exercised over; not validated, F12 Limits'); never edit either record.

**Folds.** G13-04, slice-G14-03, slice-G20-06, G13-10

### PIA-062 — RUNBOOK 7c lifts event.rs and throttle.rs 'unchanged' though they carry `Default`/`unwrap_or(0)` on usage fields (AC-362) and a 0.90 code margin

**Priority:** P2 · **Classification:** NEW — none for the AC-362 conflict; ARCH-3.10-007 (design:424, the 2-tick margin 'Declared in the pack, refused as code defaults') records the margin rule but
not that the lifted file violates it; census OH-2 records only the Step-2-deletes/7c-lifts ordering · **Category:** buildability · **Confidence:** high · **Owner:** Claude (RUNBOOK 7c, AC-362/AC-377
instruments)

RUNBOOK 7c lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs` from the spike and says the async body 'survives unchanged', then in the same bullet list requires AC-362's 'Option
with no Default impl and no unwrap_or(0) at any call site' plus a CI grep rule, and names 'no Default impl' only for usage fields [measured]. The lifted event.rs derives `Default` on both `Usage`
(:95) and `CacheCreation` (:124) and calls `unwrap_or(0)` on usage fields at :145, :162, :163 and :181 (three times): two derives and five call sites inside AC-362's grep scope [measured].
throttle.rs:30-38 hard-codes `Margins::default()` at 0.90/0.90 under a comment that says 'Two ticks' (0.90 is ten ticks of the 0.01 resolution), against the design's rule that the margin is declared
in the pack and refused as a code default; AC-377 validates only the lower bound (< 0.02 refused), so a lifted 0.90 default passes it unchallenged [measured]. Step 7c's first CI run on the lifted
adapter is red on AC-362's grep rule unless 'unchanged' is amended, and the margin default ships silently if it is not.

**Anchors.**
- `RUNBOOK.md:2013` — "call**. `rk-adapter-claude` lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs` from the spike"
- `RUNBOOK.md:2029` — "- **`Option` over `0`** on usage fields, with **no `Default` impl**, plus the CI grep rule (AC-362). F3 measured"
- `spikes/runner/src/event.rs:124` — "#[derive(Debug, Clone, Default, Serialize, Deserialize)]"
- `spikes/runner/src/event.rs:163` — "            c.ephemeral_1h_input_tokens.unwrap_or(0),"
- `references/2026-09-08-acceptance-criteria.md:4490` — "THEN the field is recorded as absent, and the type that carries it is an `Option` with no `Default` impl and no"
- `spikes/runner/src/throttle.rs:35` — "max_five_hour: 0.90,"

**Why it matters before Step 5.** Nothing at Step 5. At Step 7c a trivial fix, but the RUNBOOK's 'survives unchanged' is false and the fix must preserve `ttl_split`'s absent-vs-zero distinction rather
than delete the method.

**Remediation.** Amend RUNBOOK 7c: 'lifted with edits: drop `Default` from `Usage` and `CacheCreation`, make `ttl_split()`, `cache_read_share()` and `total_input()` return `None`/propagate absence
when a field is absent; `Margins` has no `Default` and `batch_draw` takes the margin from the pack (AC-377)'; correct the 'Two ticks' comment. Add the adapter's usage struct to AC-362's grep scope
explicitly, and add to AC-377's verifier a compile-fail or grep rule that `Margins` implements no `Default`.

**Folds.** slice-G07-09, slice-G17-08

### PIA-065 — Recorded evidence disowned by the registers: `isUsingOverage` struck as fabricated; ADR-0019's 0/156 called unverified and its >=12/23 left [measured]

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (AC-1.2-007 (AC:187) and SM-5.3-002 (SM:1398) record the overage area, even that FINDINGS.md carries the field; SM-3.3-011 (SM:1257) records the
12/23 cut; SM-5.2-010 (SM:1390) repeats the stale 'not corroborated' disclaimer; the section 16 restoration table (AC:6540-6570) records the brief-subset trap but omits isUsingOverage; none names
ADR-0019 or the un-restored cut) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Claude (AC/SM registers, brief); Ryan to accept the ADR-0019 erratum route

Three [measured] facts are disowned by the registers against sources the corpus itself declares authoritative. (1) AC-378:4688-4689 labels the citation 'F1 [measured: `isUsingOverage` present]'
fabricated and says F1 records 'exactly two readable keys'; AC-221:2865 repeats it, SM-112 calls the classifying field unknown, and design:1249 says 'No payload ever recorded'. [measured] FINDINGS.md
F1:47 -- which the brief (:3) says wins over the brief -- prints `isUsingOverage`, `overageStatus` and `overageDisabledReason`; all 40 E0 and 60 S-VAR records carry
`is_using_overage:false, overage_status:"rejected"`; throttle.rs:97 already halts on `Some(true)`; and SM-006's target and kill are keyed on `isUsingOverage` inside the register that calls the field
unknown. Only a true-valued instance is unrecorded. Section 16 (AC:6540-6544) names this trap -- 'the brief is a subset of the corpus, so a real citation absent from it looked fabricated' -- and
restores eleven identifiers, not this one. (2) ADR-0019:51's '[measured] 0/156 historical reports carry the discriminating-mutation columns' is called 'proposal-only ... treat as unverified' by
AC-317:3955 and SM-094:702 while the same registers cite it '[measured, S-MIG]' (SM-135:988; AC:1706) and s-mig-report.md:69 measures it (run 2026-09-05) -- the disclaimer is stale, not the number
('not corroborated by the evidence brief' is literally true; the brief never carried it). (3) ADR-0019:52's '[measured] >=12 of 23 ... 52%' is the figure SM-100:749 'cut as fabricated' as a
mislabelled block count -- here the accepted ADR is stale and cannot be edited. The slice's governing ADR and its registers disagree on both of its evidence lines in opposite directions, with no
pointer either way.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4688` — "*Cut — fabricated evidence:* the original cited "F1 [measured: `isUsingOverage` present on the top-level event]"."
- `spikes/FINDINGS.md:47` — ""isUsingOverage": false,"
- `references/2026-09-08-success-measures.md:79` — "- **Target:** the margin check refuses to dispatch inside ≥2 ticks of the ceiling and whenever `isUsingOverage` is set."
- `references/adr/0019-per-ac-discriminating-test-is-the-norm.md:51` — "- [measured] 0/156 historical reports carry the discriminating-mutation columns (§4.6 item 8); n=156, the full cited"
- `references/2026-09-08-success-measures.md:702` — "[proposal §4.6 item 8 — **not corroborated by the evidence brief; treat as unverified**]"
- `references/2026-09-08-success-measures.md:988` — "- **Baseline [measured, S-MIG]:** TDD `mutation: unknown` on **142 of 156** reports = **91.0%** (**95% Wilson 85.5–94.6%**); per-AC mutation columns exist in **0 of 156** historical reports"

**Why it matters before Step 5.** Nothing is blocked: 7c lifts code that already detects the overage flag and the fixture test yields the same build; ADR-0019's decision survives on F25/F31. But Step
7c would build overage detection against 'a fixture of our own construction' while a recorded envelope shape exists and is parsed by the code being lifted, and an engineer checking ADR-0019's evidence
finds one line the registers wrongly disown and one they rightly cut.

**Remediation.** Amend AC-378 and AC-221: withdraw the 'fabricated evidence' cut, cite FINDINGS.md F1 lines 42-48 and spikes/results/e0-headroom.jsonl / s-var.jsonl as the recorded envelope, narrow
PROVISIONAL to 'no true-valued instance recorded', and make the adapter fixture the recorded false-valued payload with the boolean flipped; add the omitted keys to the brief's F1 row (section A line
16); reconcile SM-006 with SM-112; add `isUsingOverage` to the section 16 restoration table; name S-RETRY (or a first-measurement row in section 10) as the section 7.10 overage row's consumer. Amend
AC-317/SM-094 to cite spikes/results/s-mig-report.md for 0/156 as [measured, S-MIG, draft F-entry]. Record ADR-0019's >=12/23 mislabel in an evidence erratum (README errata or a superseding note),
never by editing the record.

**Folds.** slice-G17-01, G16-07

### PIA-066 — ADR-0016/design 7.1 say S-WT measured a `--system-prompt` 'carrying the card body'; it was one cardless sentence, never combined with `--agent`

**Priority:** P2 · **Classification:** NEW — none -- ARCH-0.1-005, ARCH-1.2-039, ARCH-3.10-003, ARCH-7.10-002, ARCH-10.4-007 all concern the unrun append route (S-H1c); none records that the static
route's evidence was cardless · **Category:** inaccuracy · **Confidence:** high · **Owner:** Claude (ADR supersession + probe registration); Ryan to run the probe

ADR-0016:17 and design:1068 characterise S-WT/F16 as measuring a static `--system-prompt` 'carrying the card body', and AC-209/AC-387 cite F16/S-WT as [measured] for that configuration. [measured] The
S-WT/S-H1b prompt was the one sentence at run-s-h1b-and-s-wt.md:9 ('You are a careful software reviewer. Answer only what is asked.'), F16's own Limits (FINDINGS.md:649) say so, the E1 probe ran with
`agent: None` (main.rs:1888), and a scan of 43 result files (9,409 records) finds 27 records with `--system-prompt` (all cardless) and 793 with `--agent` -- zero combine them. Phase 1 dispatches both
at once (design:173 'Every dispatch passes card, system prompt, tools, schema, MCP config'; AC-204 names a card via `--agent`; AC-209/AC-211 deliver the card body as `--system-prompt`; design:356).
Which prompt the child honours when an adopted agent's own prompt and `--system-prompt` coexist, and whether the 1.000 share survives a 6.6 KB card, is unmeasured; two argv readings remain open (agent
JSON prompt = card body plus a duplicate `--system-prompt`, or agent JSON with a stub prompt) and each changes the argv builder, the adoption check and AC-209's golden test.

**Anchors.**
- `references/adr/0016-static-system-prompt-carries-the-card.md:17` — "S-WT measured prompt-cache behaviour for a static `--system-prompt` carrying the card body"
- `references/2026-09-08-architecture-design.md:1068` — "| **Space** — commits, worktrees, directories | A static `--system-prompt` carrying the card body |"
- `spikes/prompts/run-s-h1b-and-s-wt.md:9` — ""You are a careful software reviewer. Answer only what is asked.""
- `spikes/FINDINGS.md:649` — "- `--system-prompt` was a one-line static prompt. A real agent card is larger,"
- `spikes/runner/src/main.rs:1888` — "agent: None,"
- `references/2026-09-08-architecture-design.md:356` — "Prompt on stdin never argv; the card body delivered as the static `--system-prompt` (the append route BLOCKED on S-H1c)"

**Why it matters before Step 5.** This is the argv the adapter is built around at Step 7c. No Phase-1 crate is blocked -- RUNBOOK:2064's 'one real dispatch before any set' partly fences a prompt
collision and SM-063 would show a share regression -- but an accepted ADR carries a [measured] tag for a configuration nothing measured, and the gap is not registered as a spike with a consumer
(ADR-0012).

**Remediation.** Supersede ADR-0016's context paragraph (never edit): state that F16/S-WT measured a cardless one-line `--system-prompt` and that the card+`--system-prompt` combination is unmeasured.
Register in section 10.5 a haiku-scale probe (five dispatches: agent adopted via `--agent` with the card body as `--system-prompt`; a canary sentence to show which prompt governs; steady-state cache
share) with consumer AC-209/AC-211. Name in section 3.6.1 which argv reading Phase 1 builds. Correct design:1068's 'carrying the card body' to 'a one-line static prompt, no card'.

**Folds.** slice-G04-02

### PIA-067 — 'F3 measured the ephemeral_1h/5m fields absent' is false: the nested object is on 954/954 recorded runs and parsed by the runner 7c lifts

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ARCH-0.1-010 (design:28), ARCH-7.10-001, ADR-0078-MARK-97 (adr/0078:98), ADR-0078-FORB-83 -- all carry 'F3 measured absent; contradiction open;
S-TTL1 to settle'; none records that the corpus's own result files carry the fields) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Claude (design/RUNBOOK); Ryan (ADR-0078
supersession)

Section 0.1 row 28, design section 7.3, ADR-0078 clause (iv)/line 98 and RUNBOOK 7c rest on 'F3 measured the `ephemeral_1h_*`/`ephemeral_5m_*` fields absent', an 'open contradiction' with first-party
documentation that S-TTL1 is registered to settle. [measured] F3 (2026-09-02, CLI 2.1.258) reports only that the flat naming was not seen; the nested
`usage.cache_creation.{ephemeral_5m_input_tokens, ephemeral_1h_input_tokens}` object is present on 954/954 runner-typed step records across CLI 2.1.258-2.1.265 (622 with 1h>0, 55 of them at 2.1.258; 0
with 5m>0), is stated present by F10 and METHOD.md:247, is parsed by the runner's own `event.rs::ttl_split()` with a unit test named `ttl_split_reads_the_nested_cache_creation_object`, and at F3's own
date and pin `e1-ttl.jsonl` line 12 (arm ttl-90m, 2026-09-02T18:50, CLI 2.1.258) reads `ephemeral_1h_input_tokens: 7822` -- which refutes ADR-0078:98's [inferred] CLI-version-skew reconciliation.
F10's '{0, 0} on every dispatch' is true only of zero-creation hits. ADR-0078's 'no run has produced the field evidence clause (ii) requires' is false of every recorded run: the S-H1b 66.1 min cold
write carries 29,619 tokens in the 1h bucket. The 'direct contradiction between measurement and first-party documentation' does not exist and S-TTL1's question is answered by the record.

**Anchors.**
- `references/2026-09-08-architecture-design.md:28` — "were **MEASURED ABSENT**: `usage` uses plain cache naming and exposes no TTL split, which made charter E1's split metric untestable. [measured, F3]"
- `spikes/METHOD.md:247` — "`usage.cache_creation` reads `{ephemeral_1h: 0, ephemeral_5m: 0}` on every"
- `spikes/runner/src/event.rs:159` — "    pub fn ttl_split(&self) -> Option<(u64, u64)> {"
- `spikes/results/s-h1b-65m.jsonl:4` — ""cache_creation":{"ephemeral_5m_input_tokens":0,"ephemeral_1h_input_tokens":29619}"
- `references/adr/0078-cache-rule-is-split-both-ttls-set-scope-corrected.md:98` — "**F3 measured them absent** from the `usage` block on this instrument"
- `RUNBOOK.md:2030` — "`ephemeral_1h_*` / `ephemeral_5m_*` **absent**; a `Some(0)` there would have made a TTL claim look verified."

**Why it matters before Step 5.** Row 28 says 'Gates Phase 1? No' and RUNBOOK 7c's rule (set both TTLs, report TTL-unverified) builds conservative code, so no crate is blocked. But 7c tells the
implementer both to lift event.rs (which parses the fields) and that the fields are absent; the adapter's `TTL-unverified` reporting path, SM-113's time axis and section 10.6's 'every cache figure in
section 7 becomes a point estimate at an unknown TTL' are designed around an absence the corpus's own data refutes, and a registered spike would answer an already-answered question. Top of P2.

**Remediation.** Do not edit ADR-0078; supersede clause (iv) and the 'open contradiction' paragraph with a record that (a) cites F10/METHOD.md and the result files as the presence measurement, (b)
re-reads F3 as 'flat naming absent, nested object present', (c) retires S-TTL1 or narrows it to the billing-mode confound and a bucket-two (5m>0) observation, and (d) discharges row 28's gate ('no run
may be reported as 1 h without per-run ephemeral_* evidence') by pointing at the `ttl_split` field the lifted event.rs already emits. Amend section 0.1 row 28 from BLOCKED to DECIDED-with-evidence;
correct RUNBOOK 7c's F3 sentence (:2030); add a FINDINGS.md erratum on F3 as a new dated entry, not an edit.

**Folds.** slice-G07-01

### PIA-068 — `system/api_retry` payload the corpus says was never captured is captured on 139 rows in 12 result files, e3.jsonl included

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (AC-1.2-008 (acceptance-criteria.md:188); AC-PROV-004 (:428); AC-PROV-005 (:435); ARCH-3.10-006 (design:423); SM-5.3-001 (SM:1397); ARCH-10.5-023
(S-RETRY unrun) -- the gap is ALREADY-RECORDED; the recording itself is false, which is why it is carried) · **Category:** inaccuracy · **Confidence:** high · **Owner:** AC register author (Claude);
spike programme (Ryan) for the S-RETRY re-scope

The register, design and SM say the `system/api_retry` payload 'was never captured', mark AC-026/AC-232 PROVISIONAL on that ground, route the inner/outer retry accounting to the unrun S-RETRY spike
and declare section 0.3 C1 unsettleable by measurement today. [measured] `spikes/results/e3.jsonl` carries `step.api_retries` arrays with full payloads (`attempt`, `max_retries`, `retry_delay_ms`,
`error`, `error_status`) on 26 of 101 rows (102 events; lines 16-19, 21 and 26 each hold ten consecutive 529 `overloaded` attempts reaching `max_retries: 10`); the same field is populated on 139 rows
across 12 result files (p7 38, s-var 28, s-typed 18, e0-stability 6, p8 5, h4 4, p1-arm-l-v2 4, ...); and the spike runner has parsed it (`runner.rs:61 pub api_retries: Vec<ApiRetry>`,
`event.rs:278 api_retry_from`) since 2026-09-02/03, before E3 ran. The '1-2 per run' census figure is also wrong: per-row counts in e3.jsonl are 1 (11 rows), 2 (2), 3-6 (7), 10 (6). The corpus records
this as a gap in six known-issue rows; the recording is false, so it is presented as a wrong record, not a discovery.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:428` — "**the payload was never captured**, so `attempt`/`max_retries` are an assumption] · decisions: ADR-0037 — **PROVISIONAL**"
- `references/2026-09-08-acceptance-criteria.md:188` — "| 8 | Inner/outer retry accounting | `system/api_retry` exists in `spikes/results/e3.jsonl` as an **event-type census only** (1–2 per run); the payload was never captured |"
- `references/2026-09-08-architecture-design.md:423` — "| `system/api_retry` payload never captured | AC-026 / AC-232 count inner attempts against the activity budget; the event type is a census entry only (1–2 per run in `spikes/results/e3.jsonl`), its"
- `spikes/results/e3.jsonl:14` — ""api_retries":[{"attempt":1,"error":"overloaded","error_status":529,"max_retries":10,"retry_delay_ms":552},{"attempt":2,"error":"overloaded","error_status":529,"max_retries":10,"retry_delay_ms":1094}"
- `spikes/runner/src/runner.rs:61` — "pub api_retries: Vec<ApiRetry>,"

**Why it matters before Step 5.** AC-026/AC-232 sit in Step 7b's AC-001-AC-039 set (RUNBOOK:2006) yet are PROVISIONAL on a capture the corpus says nobody has; an implementer either skips them or
writes a synthetic fixture while a real captured stream exists, and these rows are the only in-corpus sample of consecutive `Platform` failures (design section 3.6: 'No spike sized a `Platform`
failure rate at all') that could size K and the backoff envelope from data rather than declaration. Not P0: nothing in Step 5 depends on it.

**Remediation.** Discharge the payload-capture check against the existing record: amend AC-026/AC-232 to cite e3.jsonl (and p7/s-var) as the captured fixture and drop PROVISIONAL; correct section 1.2
row 8, SM-003, design section 3.10 and the S-RETRY register row (S-RETRY keeps only its throttle-margin arm); re-open section 0.3 C1 as decidable by measurement and record the per-row attempt
distribution beside it. Do not edit ADR-0037; if C1 resolves against AC-391, supersede.

**Folds.** G11-02

### PIA-070 — Summary rows, acceptance-set ranges, cross-reference ids and restated figures overstate, mis-point or were never re-derived: Step 5/7b's 'AC-080-AC-119' names an unallocated id and the 7a/7b/7c ranges sweep in all six dangling ids; design row 32, section 5.11's 'every row resolves', section 0.3's missing AC-038/AC-236 twin, stale F40 ranges and tallies, 'nine' blocked rows where 0.1 counts ten, AC-095's and SM-006's wrong cross-reference ids, a stale S-BIN row; and ten settled figures carried in two states (12/23 cut and restored, SM-021, SM-015's >=5 vs >=10, GATE-05, SM-155/156, the C-range, 'three filled panels')

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B06-C11] SM-NOBASE-084 (SM:1044) and AC:6372 record that half the ports area is unexercisable in Phase 1; ARCH-8.10-006 (design:1630) records
CONDITIONAL rows are not runbook requirements; AC-0.4-P-002 (AC:112) records AC-119 as never allocated; ARCH-0.1-014 (design:32) records row 32 as unverified; AC-0.3-012 (AC:75) records the row;
ARCH-5.11-005/009 and design:1898 record the rows -- none records that the summaries contradict them | [B07-C14] H10-TRAP-005 (HANDOFF-2026-09-10:105) records the C.14 append; AC-0.4-P-002
(acceptance-criteria.md:112) records the retirements; AC-0.3-028 (:91) places AC-205 as dispatch-time; ARCH-0.1-004 is the extraction of row 22 itself; ARCH-4.9-005/ARCH-5.11-001/ARCH-10.5-003 carry
the unqualified 'no diff' wording. None records the stale ranges, the RUNBOOK sweep, the wrong ids or the METHOD-S mismatch (the F40-range, blocked-count, RUNBOOK-range and SM-006 legs are NEW). |
[B04-C14] SM-3.3-011 / SM-NOBASE-106 (12/23 'cut' side only); SM-015: ALREADY-RECORDED at success-measures.md:136 itself ('flagged here rather than silently resolved'), which the census missed;
H10-TRAP-005 (C runs to C.14); AC-0.4-T-003 (AC:120 row); ARCH-1.2-043 (DIRTY-TREE row). Unrecorded: the :1437/AC:6558 restoration conflict, AC:1285 vs AC:1274, design:904 'nine', RUNBOOK:913 F29-F39,
SM-155/156 wording, the GATE-05 ground vs 15, the residual sentence, 'three filled'.) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Ryan (RUNBOOK owner); Claude (design and AC
register)

Merged from 3 concerns; each member statement is carried verbatim. (i) Summary rows and acceptance-set ranges overstate what their detailed sections say. [measured] (1) RUNBOOK:1902 gives Step 7a's
acceptance set as AC-480-AC-519 minus AC-239/AC-496/AC-502 only; the range still contains AC-501/AC-503-AC-506 (rk-plugin-gen, Phase 2 per AC:6372), AC-507/AC-508/AC-510 (GitHub and mdBook adapters,
Phase 3), retired AC-513 (AC:112) and the six CONDITIONAL criteria AC-512/AC-514-AC-517/AC-519 that design:1630 says 'are not runbook requirements' -- up to 16 criteria with no Phase-1 instrument. (2)
RUNBOOK:2007's Step 5 set spans 'AC-080-AC-119' though AC-119 was never allocated (AC:112, :1295 -- the only one of the RUNBOOK's 77 cited AC ids with no definition), and the same sentence's
'AC-040-AC-079' subtracts the P1-blocked rows but not retired AC-075. (3) Design section 0.1 row 32 says 'AC-480..AC-519 are borrowed-failure criteria and every measure in the area starts with no
baseline'; section 8.8:1563 narrows the first to the distribution half (AC-480/481/483/484 are Phase-1 CI assertions, RUNBOOK:1775) and SM-154 carries a measured $56.47 baseline (SM:1044 correctly
says seven of eight). (4) AC section 0.3:75 says 'AC-236's gate half is distinct and is not duplicated' while AC-038:550-552 asserts the same gate-half behaviour, section 0.3 has no AC-038/AC-236 row,
and design:342 lists AC-036/037/038/236 together; the clause is at best ambiguous ('not duplicated by AC-037') and the twin is missing either way. (5) Design section 5.11:778 claims 'Every row
resolves to a spike with a named consumer in section 10'; of its ten rows only three name a spike (S-BIN, S-GIX, P8's arms), and section 10:1898 itself says the rest resolve to a recorded decision.
(ii) [measured] Nine small pointer defects, all one-line fixes. F40-DRAFT.md landed 2026-09-10 (009a4ec) and is cited at 22/23/26 anchors in the design/SM/RUNBOOK, but the design's Inputs line (:3)
and sources row (:1986) still enumerate the drafts as F29–F38 (+F39), one row below a brief row that says F1–F40; SM :5 attributes 'F1–F38' to spikes/FINDINGS.md, which holds F1–F28; design :1985 says
'§C.1–C.13 ... C.13 is F39's, appended' in the same row whose ledger clause records 'F40 added 2026-09-10' (the brief has 14 items, design :218 maps C.1–C.14); design :904 says 'nine blocked rows of
§0.1' where :15 counts ten (the `stale_policy` row was added in a627547). RUNBOOK 7b's acceptance set 'AC-080–AC-119' includes AC-119, which the register marks unallocated, and the same range pattern
in 7a/7b/7c sweeps in all six ids ids.json lists as dangling (AC-075, AC-119, AC-207, AC-255, AC-297, AC-513), so a coverage script keyed on the ranges reports six phantoms. AC-095 :1438
cross-references AC-014 (the memo-key miss property test) for the dispatch-time StructuredOutput refusal and flags the reference as unverified — the intended criterion is AC-205 (§0.3 :91); SM-006's
only firing condition (:81) files an overage admission against AC-027 (zero re-dispatch after a StepError) when the pre-spawn refusal is AC-221 (:2863), so a defect routed to AC-027 would be closed as
not-a-retry. METHOD-S §7 (2026-09-09, :951-960) registers four of five S-BIN items as carrying a source/test `text_diff` and states the register rows 'are qualified accordingly', and :1017-1023 names
two candidate captures producers with the manual /ue-verify route (17 PNGs at the pin) the one in use; design :22 still names only the never-run automation comparison, and :607/:1862 still read 'no
diff' unqualified — no file in references/ or RUNBOOK contains 'not diff-free' or 'four of five'. (iii) Each item is one document carrying a figure or status in two states because a correction landed
in one place and the restatement was read off the sentence rather than re-derived, the drift class design 0.1's header (:15) already had to correct once [measured unless tagged]. SM register :749,
:988 and :1257 say the '>=12 of 23 (52%)' figure was cut as fabricated (with an arithmetically wrong rationale: 5/23 is F24's 21.7%, a different count), while :1437 and AC 15 (:6558) restore it as
real, sourced to proposal 1 and design-recommendation :38, so whether SM-100 has a baseline (at most 11/23 pinned) depends on which row is read; AC:1285 says SM-021 'still has no target and no
baseline' eleven lines after AC:1274 and the SM register's same-day amendment (:176, :1112, :1125, :1313, :1381, :1414) give it a partial specificity baseline and recount 59 to 58; SM:133 sets
SM-015's target at >=5 draws where AC-071's precondition (:165, :1112) and the design's corrected sites read >=10, SM:136 deferring to 'the area owner' - ac-selection.md - which does not exist
[measured]. Design:904 counts 'nine blocked rows of 0.1' after :15 raised the table to ten on 2026-09-10; design:1985 says the brief's C runs to C.13 and :3's Inputs omit F40-DRAFT, which the body
cites 22 times, against :218 and brief:143 (C.14 is F40's); RUNBOOK:913-914 says 'F29-F39 live as drafts' on a 2026-09-09 `ls F3*` glob that cannot match F40, which :9 and 26 other lines cite; the SM
id range is 'SM-001..SM-154 plus SM-157' (AC:3), 'SM-001-SM-156 plus SM-157' (design:3, :1990), and SM-155/156 are 'reserved to ports-dist' (AC:598) or 'belong to no area at all' (SM:31, AC:5975);
AC:120 still cuts AC-323's original on 'its evidence, GATE-05, does not exist' after 15 (:6551) restored GATE-05 as real (scorecard:141), the second cut ground standing but the original never
re-asked; design:135 records the AFK output committed at e3b0bac then closes 'Residual: nothing from the AFK session was committed', citing brief E, which post-correction says the opposite;
RUNBOOK:446-447, :548 and :2464 say 'three filled panels' while :723, :805-807 and :2458 retain arm D unfilled at `labeller: FILL-ME` by amendment 9. None has a build consequence; each is what a
builder reads first.

**Anchors.**
- `RUNBOOK.md:1902` — "**Acceptance set: AC-480–AC-519** (ports, fakes; AC-239/AC-496/AC-502 are out of scope — S-CODEX is unrun and"
- `references/2026-09-08-architecture-design.md:1986` — "`spikes/FINDINGS.md` · `spikes/F29-DRAFT.md`–`spikes/F38-DRAFT.md`"
- `references/2026-09-08-success-measures.md:1257` — "| **AC-level pinning rate, historical** | **No AC-level pinning rate has ever been measured**; the "≥12 of 23 = 52%" figure was a mis-labelled block count and was cut |"
- `RUNBOOK.md:2007` — "AC-070, AC-072, AC-076, AC-079), **AC-080–AC-119** (packs and catalog lint), and **AC-520** — the derived-facet"
- `references/2026-09-08-architecture-design.md:904` — "P8's shortfall is one of the nine blocked rows of §0.1 and it gates the **"design final" label**"
- `references/2026-09-08-success-measures.md:1437` — ""0/156 historical reports" and "≥12 of 23" are all **real**, and the acceptance-criteria register carries the table of their true sources."

**Why it matters before Step 5.** (i) Exit conditions and detailed sections agree, so no wrong build. But a checkpoint that counts the 7a range toward coverage overstates Phase-1 assurance by up to 16
criteria, an operator who tries to satisfy them builds Phase-2 surface out of order, a reader of section 0.1 alone files the four Phase-1 CI assertions under 'borrowed, no baseline, not Phase 1', and
a builder trusting section 0.3 instruments AC-038 and AC-236's gate clause twice. (ii) None for the build. S-BIN is BLOCKED and unrun either way; a reader of §10.5 alone would build its fixture as
diff-free. (iii) None for the build. Together they are the closeout pass the corpus owes before Step 5 reads these documents as settled; each fix is a line edit, and SM-015's needs the precedence rule
the register says it lacks.

**Remediation.** (i) Rewrite Step 7a's set as AC-480-AC-495, AC-497-AC-500, AC-509, AC-511, AC-518 and state that the six CONDITIONAL rows are dormant until the distribution decision; change
RUNBOOK:2007 to 'AC-080-AC-118' and subtract AC-075 from the selection range; amend section 0.1 row 32 and section 8.8:1563-1564 to 'AC-501..AC-519's distribution half are borrowed-failure criteria
and seven of the area's eight measures start with no baseline (SM-154 carries E3's $56.47)'; correct the section 0.3 row to owner AC-038 with AC-236's gate clause as twin (or the reverse) and drop 'is
not duplicated'; reword section 5.11:778 to 'Every row names what discharges it -- a spike, a runbook decision or a first measurement -- and none is discharged as prose (ADR-0012)'. (ii) One hygiene
pass across design, SM and RUNBOOK: update design :3/:1986 and RUNBOOK :913 to 'F29–F40 (F30 reserved)' and SM :5 to 'spikes/FINDINGS.md F1–F28 plus spikes/F29..F40-DRAFT.md'; design :1985 to
'§C.1–C.14 (C.13 is F39's and C.14 is F40's, both appended)'; design :904 'nine' → 'ten'. Write each RUNBOOK step's acceptance set as an explicit exclusion list naming all six dangling ids (AC-075 at
7b; AC-119 → 'AC-080–AC-118'; AC-207, AC-255, AC-297 at 7c; AC-513 at 7a). AC-095's parenthetical → AC-205, dropping the 'could not verify' hedge; SM-006's kill → AC-221 (with AC-220's margin rule).
Add to design :1862 'four of five items carry a source/test `text_diff` scoped per METHOD-S §7 (2026-09-09); only item 2 is diff-free', mirror at :607, and reword row 22 to 'two candidate producers —
the manual /ue-verify route (17 PNGs at the pin, in use) and UE's automation comparison (never run)'. (iii) One dated closeout pass, re-deriving each figure from its table or source rather than the
sentence: SM :749, :988, :1257 to 'struck by the area pass, restored by the 2026-09-08 corpus verification (:1437; AC 15): S-TYPED's confirmed-blocker count, a lower bound on items with an unpinned
criterion, giving SM-100 a baseline of at most 11/23 pinned; not a per-AC rate'; AC:1285 to 'no numeric target and a partial baseline (specificity only, perturbation half; SM 5.1); precision and
recall remain without a baseline because the label set has no positive class'; SM-015's Target to '>=10 draws' citing AC-071 as the owner of the number, and a precedence line that a measure restating
an AC precondition takes the AC's figure; design:904 to 'one of the blocked rows of 0.1' (no count); design:1985 to 'C.1-C.14 (C.13 is F39's, C.14 is F40's, both appended)' and :3 Inputs to add
spikes/F40-DRAFT.md; RUNBOOK:913-914 to 'F29 and F31-F40 [measured, ls spikes/F*-DRAFT.md, 2026-09-10]'; AC:598 to 'SM-155/SM-156 are unallocated and never reassigned' and design :3/:1990 to
'SM-001-SM-154 plus SM-157 (SM-155, SM-156 unallocated)'; AC:120 to '(cut: its THEN was a statement about the test suite; the GATE-05 ground was withdrawn in 15)' with a 15 note that the original
AC-323 is a candidate for re-issue under a new id; design:135's residual to '17 untracked items (driver logs, p8-state, two build directories); everything the figures rest on is tracked';
RUNBOOK:446-447, :548, :2464 to 'the three label files of record (two filled, arm D retained unfilled by amendment 9)'.

**Folds.** L07-testability-07, L12-authority-08, slice-G10-10, L05-overlap-07, G13-11, L02-07, L10-register-integrity-08, G08-07, L07-testability-08, slice-G18-14, slice-G04-09, G11-08, L09-07,
slice-G03-09, L01-contradiction-02, slice-G02-03, L12-authority-04, L01-contradiction-04, L01-contradiction-03, L01-contradiction-05, L01-contradiction-06, L01-contradiction-08, L02-04, L08-12

### PIA-076 — Register baselines carry the wrong condition label or qualifier: F35's 8-pack series called '12 packs'; SM-029 baselined on the buggy lint; AC-106's line count; SM-148's phase; design:72's F15 licence stated without tier or n; F11's 9/10 without the S0.1-SCOPE clause; SM-025's 27 s from the 16-band lattice; SM-104 borrowing SM-086's baseline

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B06-C10] SM-NOBASE-013 (SM:224) records SM-029 'already in its kill state' as fact; SM-5.1-053 (SM:1373) and SM-NOBASE-091 (SM:1070) record
SM-148's Phase-4 claim as the corpus's own statement; ARCH-0.1-030 restates row 48 without tier/n; ARCH-10.4-031 repeats the '12-pack readings' gloss; the F35 mislabel and the AC-106 instrument
mismatch are NEW | [B07-C11] ARCH-10.5-027 (references/2026-09-08-architecture-design.md:1886) registers S0.1-SCOPE with consumers 'AC-289, SM-101; ADR-0019; every F11 citation';
ARCH-1.2-004/ARCH-10.x-T-014 record the 8-of-23 scope; SM-NOBASE-010 (success-measures.md:162) records the 27 s as n=1; SM-3.3-010 and SM-5.2-010 record SM-104's rate as an upper bound and the missing
harness. None records the registers' non-compliance, the 16-band provenance of 27 s, or SM-104's borrowed baseline.) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Claude (design pass;
SM/AC register maintainer)

Merged from 2 concerns; each member statement is carried verbatim. (i) Five register rows carry a baseline or licence under the wrong condition label. [measured] (1) Design section 4.6:553 puts 11.4%
and 0.3% unsatisfiable `(stage, ctx)` pairs under a '12 packs' column and :563 calls 0.3% a '12-pack figure'; F35:9-10 and :37-47 give 23.5% -> 11.4% -> 0.3% for the 8 pre-registered packs at
pack-default rigor (12-pack: 31.2% -> 12.2% -> 0.2%), so the table that cites SM-027's rule 'two readings at different pack counts must never be compared' breaks it; the dead-activity row's headings
fit, the unsatisfiability row's do not; SM-014, SM-026 and AC-101 inherit the gloss and 12.2% appears nowhere outside F35. (2) SM-029:259 baselines '13 real / 34 false = 27.7% precision ... n=2
passes' and concludes the measure 'starts in the kill state'; F32:32 says the 34 false alarms came from four lint bugs fixed the same day, after which the same catalog gave exactly the 13 real errors,
and F33/F34 gave 2/4 and 9/10 on the unmodified v2 lint -- the baseline is an artefact of a superseded instrument and 'n=2' omits F34; design section 4.5 repeats it. (3) AC-106:1550 and SM-031:272
threshold 'non-blank line count'; the F32/F34 baselines they cite count non-blank, non-comment lines (F34: 5,059 against 5,683 non-comment), and 92 of 102 v0 files carry full-line comments -- a small
instrument mismatch (largest file 106 vs 104 against a 150 line). (4) SM-148:1070 and :1373 say a second adapter 'of any port' is 'not scheduled before Phase 4'; proposal section 7:367 ships the .NET,
Python and Unreal Toolchain adapters in Phase 2 and ADR-0008:49 concurs, so the one signal SM-148 is tracked for ('the first non-zero value') is dated two phases late; only a second Harness adapter is
Phase 4. (5) Design:72's Forbids on 'treating low->high as a quality dial: only xhigh is an observable step' generalises F15 -- 15 sonnet dispatches, one prompt, n=5 per cell, whose Limits say opus
and harder tasks are 'untested and was not asked' -- without stating tier or n (AC-212 carries the correct narrow licence). (ii) [measured] (i) Design :45 binds a scope clause ('8 of 23 corpus items
sit outside S0.1 entirely') to every F11 citation and §10.5 :1886 names AC-289 and SM-101 as S0.1-SCOPE's consumers; the design carries the clause on 8 of its 14 F11 lines, the AC register on 0 of 3
(AC-289 :3654, AC-340 :4199, the §15 F11 row :6396) and the SM register on 0 of 7 — a whole-file grep of both registers for '8 of 23', '8/23', 'workflows/' or 'S0.1-SCOPE' returns nothing, so a reader
of AC-289/AC-340/SM-101 takes 90% (Wilson [0.596, 0.982]) as the gate's kill rate over the corpus rather than over the src/-touching two-thirds. (ii) F35 :19 states 'Runtime 27 s' closing the
paragraph that enumerates the run3-era lattice (2^4 = 16 bands, 26,432 pre-registered pairs, 101 activities, 12 packs); the post-decision runs enumerate 3^4 = 81 bands and 155,520 pairs (5.9×) over
102 activities and F35 :51 reports their speed only as 'under two minutes'; no results file records a runtime. Design §4.6 :557 places 27 s in the 'After decisions 1–10' column, §0.1 row 53 says '27 s
lattice runtime after decisions 1–10', and SM-025 :228 baselines the CI budget on '27 s over the 102-activity v0 catalog' while caveating the pack count — which F35 does state — instead of the band
count, so its baseline is off by an unknown factor of up to ~4× with the wrong uncertainty attached. (iii) SM-104 measures verification-gate `block_emitted` events (AC-358), which no mutation harness
has produced (SM-5.2-010); its Baseline is 'S-TYPED measured 0 of 23 verified false positives' — the review-gate figure SM-086 already owns — while §1.9 :724 lists four measures with no baseline
omitting SM-104, and design :1019 and the AC residual :4447 say it has none. The same 0/23 is baseline for two gates in one register, and the register disagrees with itself on whether it is one.

**Anchors.**
- `references/2026-09-08-architecture-design.md:553` — "| Unsatisfiable `(stage, ctx)` pairs | **23.5%** | **11.4%** | **0.3%** | >5% — **fires twice**, clears post-decision |"
- `references/2026-09-08-architecture-design.md:1886` — "| **S0.1-SCOPE** | Extend the affected-area gate to the 8 of 23 corpus items whose fixes touch only `workflows/`, and re-measure — or record the exclusion as permanent | AC-289, SM-101; ADR-0019; every F11 citation |"
- `spikes/F35-DRAFT.md:10` — "| same, all 12 packs | 31.2% | 12.2% | | |"
- `references/2026-09-08-acceptance-criteria.md:3654` — "**AC-289** — `rk-kernel` · evidence: F11 (S0.1: 15 mutants, 9 of 10 test-reachable killed, **1 survived**; test-reachable"
- `references/2026-09-08-success-measures.md:259` — "- **Baseline:** **13 real / 34 false = 27.7% precision** on one authoring pass [measured, F32, 10 activities]"
- `references/2026-09-08-success-measures.md:756` — "F11's S0.1: **9 of 10 test-reachable mutants killed, 1 survived — 90.0%, Wilson 95% [59.6%, 98.2%]**"

**Why it matters before Step 5.** (i) No Step-5 code decision. SM-029's kill response is 'rules are removed, not tuned', so a buggy-instrument baseline invites removing rules that scored 90% on the
latest pass (SM-029 does instruct a re-measure first); AC-101's baseline sentence inherits the mislabel; pack authors reading design:72 are forbidden `high` on opus review cards on evidence that never
measured opus; SM-148's first discriminating value would arrive two phases before the register expects it. (ii) No build decision changes; SM-025's CI-budget baseline, SM-101's kill-rate baseline and
SM-104's calibration baseline are what the shipped kernel will first be measured against, and each is mis-qualified.

**Remediation.** (i) Relabel section 4.6's unsatisfiability row as the 8-pre-registered-pack, pack-default series with the 12-pack series beside it (fix :553, :563, section 4.9, SM-014, SM-026,
AC-101). Re-baseline SM-029 as 13/13 (F32 post-fix), 2/4 (F33), 9/10 (F34) with the instrument version per row, state that 27.7% was the buggy-lint reading, and remove 'starts in the kill state' from
SM section 1.3's preamble and design section 4.5. Write 'non-blank, non-comment' in AC-106 and SM-031. Amend SM-148 (:1070 and :1373): 'a second adapter of any port first exists in Phase 2 (the
.NET/Python/Unreal Toolchain adapters, ADR-0008); a second Harness adapter not before Phase 4'. Reword design:72 to 'no measured step between low and high on sonnet at n=5, one prompt; xhigh is the
only measured step' and drop the Forbids, or register the opus/harder-task contrast as a spike with a consumer before forbidding. (ii) Append the scope clause '(S0.1-SCOPE: 8 of 23 items unmeasured by
construction)' to AC-289, AC-340, the §15 F11 row, SM-101/SM-102 and the SM §3/§5 tables — or drop the 'every citation' clause from §0.1 :45 and keep it at the register row; add a workflow-only
fixture to AC-289's set once S0.1-SCOPE runs. Restate the runtime as '27 s at run3 (101 activities, 16 bands, 12 packs); post-decision 81-band runs unmeasured, under 2 min', move 27 s out of §4.6's
post-decision column and §0.1 row 53, replace SM-025's pack-count caveat with the band-count one, and take a stamped timing on the next lattice run. Reword SM-104's Baseline to 'none for this
instrument; nearest comparator is SM-086's review-gate 0/23 (upper bound)' and add SM-104 to §1.9's no-baseline list (five, not four).

**Folds.** slice-G18-05, slice-G18-08, slice-G18-13, slice-G10-06, slice-G07-12, L07-testability-06, G16-11, slice-G18-06, G16-14

### 4.3 Citation drift (20)

### PIA-020 — ADR-0079's external corroboration overstates three preprints: the schema-induced mechanism, P1-BOUND's boundary premise, Essence Coach

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ADR-0079:231 ('it motivates the diagnostic, it does not predict its outcome here') and :236-238 hedge the papers correctly;
architecture-design:143/:457/:1839 and evidence-augmentation:99/:202 do not, and P1-BOUND (architecture-design:1884) is registered on the unhedged reading) · **Category:**
citation-fidelity/design-and-ADR · **Confidence:** high · **Owner:** ADR-0090 author (Step 4.4); P1-BOUND owner

Three external corroborations of F36's instability finding say more than their sources. (1) architecture-design:143's joint summary, 'corroborated across five 2026 frontier models, with
Claude-Sonnet-4 measured at background temperature 0 - the instability is schema-induced, not decoding noise [preprint, arXiv 2603.04417; arXiv 2604.22411]', attaches a mechanism neither paper states:
2603.04417 tests Claude-Sonnet-4.5 (not Sonnet-4, which is 2604.22411's), finds Anthropic models 'often show minimal reductions and, in some cases, increased variability at temperature 0', and
attributes this to 'deeper architectural or decoding-level differences' [documented]. The per-source rows (:2030, evidence-augmentation:200, ADR-0079:222) are exact; the mechanism clause is ADR-0079's
own inference presented as external. (2) 'reports classification instability correlating with proximity to the decision boundary' (architecture-design:457, :1839, :2032; evidence-augmentation:202;
ADR-0079:231) credits arXiv 2511.01176 with a finding it does not measure: the boundary sentence is an unmeasured premise in its section 5.2, and its measured result is consistency over five identical
submissions with temperature having 'a small influence' [documented]. P1-BOUND, the registered diagnostic, rests on that premise. (3) evidence-augmentation:99 and ADR-0079:140 credit arXiv 2508.16445
('Essence Coach') with flagging that 'Essence assessment requires precise criterion matching probabilistic models struggle to guarantee'; three full-text passes over v1 and v2 find no such statement,
the paper's stated limitations being dataset size, no comparison and manual validation; v2 (2026-09-01) is retitled 'Essence Coach: A Bot for Software Practice Adoption' [documented]. The decisions
survive on other grounds: the temperature-arm prohibition is licensed by 04417's own T=0 result for Anthropic models and by 2511.01176's small temperature effect; P1-BOUND is desk work over existing
draws and costs nothing. What changes is the tag and the wording.

**Anchors.**
- `references/2026-09-08-architecture-design.md:143` — "Externally corroborated across five 2026 frontier models, with Claude-Sonnet-4 measured at **background temperature 0** — the instability is **schema-induced, not decoding noise** [preprint, arXiv 2603.04417; arXiv 2604.22411]"
- `references/2026-09-08-architecture-design.md:2030` — "| **arXiv 2603.04417**, *Same Input, Different Scores* (Feb 2026) — GPT-4o, GPT-4o-mini, Gemini-2.5-Flash, Claude-Haiku-4.5, Claude-Sonnet-4.5 | "Despite expectations of stability at temperature=0, substantial variability persisted across models""
- `references/2026-09-08-architecture-design.md:457` — "| arXiv 2511.01176 (code-clone classification) | Instability **correlates with proximity to the decision boundary**; far-from-boundary inputs are typically stable | [preprint] |"
- `references/adr/0079-context-vector-source-undecided-phase-1-blocked-on-p1.md:231` — "(arXiv 2511.01176), which reports classification instability correlating with proximity to the decision boundary. It is"
- `references/adr/0079-context-vector-source-undecided-phase-1-blocked-on-p1.md:237` — "measures **Claude-Sonnet-4** in **its own** harness, and P1 arm L ran haiku 4.5 and sonnet 5 at default temperature"
- `references/2026-09-08-evidence-augmentation.md:99` — "**New corroboration:** a 2025 applied paper builds an LLM tool ("Essence Coach") directly on Essence alphas/activity-spaces/criteria and independently flags that Essence assessment requires precise criterion matching probabilistic models struggle to guarantee"

**Why it matters before Step 5.** Step 4.4 writes ADR-0090, superseding ADR-0079 in full, before Step 5; it will inherit these three citations. The P1-BOUND write-up will interpret its buckets against
a premise its source never measured.

**Remediation.** ADR-0079 is `proposed` and RUNBOOK:1581 forbids touching it before Step 4.4; carry the corrections into ADR-0090: retag the mechanism clause [inferred, F36 + ADR-0079], name
Sonnet-4.5 for 2603.04417 and Sonnet-4 for 2604.22411, cite 2511.01176 for its measured temperature result and describe the boundary relation as the paper's premise, and either re-source the Essence
Coach determinism claim to a page and line or downgrade it to [inferred] with the v2 title. Amend the P1-BOUND register row (architecture-design:1884) by dated erratum so that its interpretation rule
states the premise is unmeasured in the source; the spike itself stands. Dated erratum to architecture-design:143, :457, :1839, :2032 and evidence-augmentation:99, :202.

**Sources.**
- https://arxiv.org/pdf/2603.04417 (fetched 2026-09-10) — five models incl. Claude-Sonnet-4.5; 'Anthropic models often show minimal reductions and, in some cases, increased variability at temperature 0 ... deeper architectural or decoding-level differences'
- https://arxiv.org/pdf/2511.01176 (fetched 2026-09-10) — boundary sentence is an unmeasured premise (s5.2); measured: >90% consistent for GPT-4o, 'temperature ... has a small influence on ... response consistency'
- https://arxiv.org/pdf/2508.16445v2 (fetched 2026-09-10) — retitled 'Essence Coach: A Bot for Software Practice Adoption' (2026-09-01); limitations: small dataset, no comparison, manual validation; no determinism-vs-LLM claim

**Folds.** EXT-PREPRINT-018, EXT-PREPRINT-022, EXT-PREPRINT-024

### PIA-023 — ADR-0083's and ADR-0085's evidence lines misquote or misattribute their sources: the nursery clause ('change or be removed without notice') is in no Clippy text and `nursery = "warn"` as a group is what the Lints book advises against; 'pin for a binary you control' is RFC 3537's framing, not swatinem's; MADR's minimal cut, its 'updated over time' clause and Log4brains are misdescribed; in-source overrides should be `#[expect]`

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B09b-C04] none (evidence-augmentation:401 records the swatinem URL as 'not returned' but ADR-0083:118 still attributes the framing to it) |
[B10-C07] references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:106 'Forbids denying `clippy::pedantic` or `clippy::nursery` as groups' records the group-deny hazard; it does not record the
misquote or the warn-group problem | [B09a-C11] evidence-augmentation.md:401 and :414 self-flag the swatinem and Log4brains URLs as 'not returned'; known-issues EA-1-011 records MADR immutability as
non-uniform; EA-5-025 records the MADR fetch failure. The specific misattributions are unrecorded.) · **Category:** citation-fidelity/live-ADR · **Confidence:** high · **Owner:** ADR-0083 successor
author

Merged from 3 concerns; each member statement is carried verbatim. (i) Two [documented]/[1P] attributions in ADR-0083 and its design and evidence-augmentation restatements are not in the cited
sources. (1) 'nursery lints can change or be removed without notice' (ADR-0083:29, :67, :117; architecture-design:1720; evidence-augmentation:149) appears in no Clippy text: the Lints book says the
nursery group 'contains lints which are buggy or need more work' and 'It is not recommended to enable the whole group', the README says 'new lints that are still under development'; and
'false-positive-prone' is the book's description of pedantic, not nursery [documented]. (2) 'the "pin for a binary you control" framing is the blog's' (ADR-0083:118; architecture-design:1718;
evidence-augmentation:150) attributes to swatinem.de a framing the post does not contain: it never mentions `package.rust-version`, floors, libraries or binaries, and the corpus never read it
(evidence-augmentation:401 'URL not returned'); the application framing is RFC 3537's ('an application with a vetted toolchain') [documented]. Neither decision changes: nursery at warn is licensed by
'not recommended to enable the whole group', and the pin by RFC 3537 alone. The `clippy::correctness`-only-deny and `clippy::cargo`-is-manifest halves of :117 are exact. (ii) ADR-0083 and RUNBOOK 5.1
justify `clippy::nursery = "warn"` as a whole group 'on the documented grounds that nursery lints can change or be removed without notice' and tag the sentence [1P] Clippy Lints book. The book
contains no such sentence. Its nursery text is 'lints which are buggy or need more work. It is **not** recommended to enable the whole group, but rather cherry-pick lints', so enabling the group at
warn is the thing the source advises against, and the ADR's n=1 first-party citation supports a claim the source does not make. The ADR also reserves per-crate `#![allow]`/`#![deny]` 'as a visible,
commented override'; `#[expect(lint, reason = "...")]` has been stable since 1.81 and is the self-cleaning form (a stale expectation is itself a lint), which Microsoft's M-STATIC-VERIFICATION uses.
The enumerated-pedantic-subset choice is a legitimate stricter variant of Microsoft's warn-group-with-opt-outs and needs no change. [documented] (iii) First verification of four self-flagged sources
shows the evidence sections of two accepted convention ADRs inverted or overstated what their sources say [documented]. ADR-0083 :118: 'The "pin for a binary you control" framing is the blog's,
corroborated by RFC 3537' - swatinem.de recommends pinning as 'yet another dependency' and never mentions rust-version, floors, libraries or binaries (zero grep hits); the application-vs-library,
pin-vs-floor framing is RFC 3537's own text, so the attribution is backwards. ADR-0085 :108: 'MADR's own bare/minimal cut keeps title, Context and Problem Statement, Decision Outcome and status/date
only' - the minimal template keeps Considered Options (mandatory per the 4.0.0 CHANGELOG), optional Consequences, and has no frontmatter at all. ADR-0085 :29/:109 and adr/README.md :13: MADR 'permits
Consequences/Confirmation to be updated over time' - not in MADR's template, CHANGELOG or docs; inference tagged [documented]. ADR-0085 :108 and evidence-augmentation :231: 'adr-tools and Log4brains
regenerate a README/index table from frontmatter on each new record' - Log4brains rejects NNNN numbering by design (YYYYMMDD-slug.md), infers metadata from raw text and git log, and adr-tools
regenerates a TOC only on an explicit `adr generate toc`. Every decision (exact pin; immutability as this repo's chosen convention; generate-don't-hand-maintain) survives.

**Anchors.**
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:29` — "`pedantic` and `nursery` are documented as false-positive-prone and, for nursery, unstable — nursery lints can change or be removed without notice."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:118` — "[documented] `rust-toolchain.toml` is a pin and `package.rust-version` a floor; they answer different questions. n = 2 ([blog] swatinem.de; [1P] RFC 3537), §3. The "pin for a binary you control" framing is the blog's, corroborated by RFC 3537's separate treatment of the floor"
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:117` — "[documented] `clippy::correctness` is the only deny-by-default group; `pedantic` and `nursery` are documented as false-positive-prone, and nursery lints can change or be removed without notice; `clippy::cargo` checks metadata, not code. n = 1 first-party ([1P] Clippy Lints book), §3."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:67` — "- `clippy::nursery = "warn"` only, on the documented grounds that nursery lints can change or be removed without notice."
- `references/adr/0085-documentation-and-traceability-conventions.md:108` — "MADR's own bare/minimal cut keeps title, Context and Problem Statement, Decision Outcome and status/date only. The supersession convention as a bidirectional pointer, stamped by `adr-tools -s`, is confirmed. Flat `NNNN-slug.md` with a generated index is community consensus at 100+ records"
- `references/adr/0085-documentation-and-traceability-conventions.md:109` — "[documented] Immutability is **not** an inherited standard. MADR permits an accepted decision's status to move to `deprecated` in place and permits Consequences/Confirmation to be updated over time."

**Why it matters before Step 5.** (i) No build consequence; the ledger's n=1 first-party count for the nursery clause and the blog attribution are what a later reader will re-verify and fail. (ii)
Step 5.1 writes `[workspace.lints.clippy]` verbatim from this ADR into the first Cargo.toml. Changing the lint table after twelve crates exist is a workspace-wide diff; changing it before Step 5 is
one line. Not a wrong-thing blocker, but the cheapest moment is now. (iii) None for the build. Step 5.1 writes rust-toolchain.toml exactly as ADR-0083 decides; the citation under it is wrong, the
decision is not.

**Remediation.** (i) In the same superseding clause as B09-C03, retag the nursery clause [inferred] with the book's actual wording ('buggy or need more work'; 'not recommended to enable the whole
group') as the documented ground, move 'false-positive-prone' to pedantic, and cite RFC 3537 alone for the pin-versus-floor distinction, dropping swatinem.de or citing it only for 'pin, and make
updates intentional'. Mirror the two corrections in the design's §9.3 table and §11 source table by dated erratum. (ii) Supersede ADR-0083's lint clause: drop `nursery = "warn"` as a group (either
omit nursery or enumerate the two or three nursery lints wanted, as a list like the pedantic subset); replace the misquoted rationale with the book's own wording and re-tag it; permit
`#[expect(lint, reason = "...")]` as the only in-source override form and forbid bare `#[allow]`. Carry the same three lines into RUNBOOK 5.1 before Checkpoint 5. Owner: Step 5 workspace author. (iii)
One successor erratum record covering ADR-0083 :28/:118 and ADR-0085 :29/:108/:109 (both accepted, immutable): cite RFC 3537 as primary for pin-vs-floor and application-vs-library and swatinem only
for pin-as-dependency; state MADR's minimal template verbatim (Context and Problem Statement, Considered Options, Decision Outcome, optional Consequences; no frontmatter); re-tag the 'updated over
time' clause [inferred]; cite adr-tools (and MADR's own pattern) for NNNN-slug and Log4brains only as a generated-index counter-example. Amend evidence-augmentation :39/:229/:231 and adr/README.md :13
by dated amendment.

**Sources.**
- https://doc.rust-lang.org/stable/clippy/lints.html (fetched 2026-09-10) — The clippy::nursery group contains lints which are buggy or need more work. It is not recommended to enable the whole group, but rather cherry-pick lints
- https://raw.githubusercontent.com/rust-lang/rust-clippy/master/README.md (fetched 2026-09-10) — clippy::cargo | lints for the cargo manifest | allow ... clippy::nursery | new lints that are still under development | allow
- https://swatinem.de/blog/rust-toolchain/ (fetched 2026-09-10) — argues 'pin the Rust toolchain ... to make updates intentional and avoid surprises'; never mentions rust-version, floors, libraries or binaries
- https://rust-lang.github.io/rfcs/3537-msrv-resolver.html (fetched 2026-09-10) — 'controlling their MSRV via rust-toolchain.toml ... (e.g. an application with a vetted toolchain)' - the application framing is the RFC's
- https://raw.githubusercontent.com/rust-lang/rust-clippy/master/book/src/lints.md (fetched 2026-09-10) — 'The clippy::nursery group contains lints which are buggy or need more work. It is **not** recommended to enable the whole group, but rather cherry-pick lints.' Pedantic: '#[allow] ... generously'
- https://microsoft.github.io/rust-guidelines/print.html (fetched 2026-09-10) — M-STATIC-VERIFICATION: pedantic = { level = "warn", priority = -1 } ... nursery optional; overrides via #[expect(lint, reason = ...)]
- https://swatinem.de/blog/rust-toolchain/ (fetched 2026-09-10) — I would recommend to pin the Rust toolchain to a specific version using a rust-toolchain.toml file ... think of the toolchain as yet another dependency. Zero hits for rust-version, librar, binary, app
- https://raw.githubusercontent.com/adr/madr/main/template/adr-template-minimal.md (fetched 2026-09-10) — # {short title} / ## Context and Problem Statement / ## Considered Options / ## Decision Outcome / <!-- optional --> ### Consequences. No frontmatter.
- https://github.com/thomvaill/log4brains (fetched 2026-09-10) — No required file numbering schema (i.e., adr-0001.md, adr-0002.md...): avoids git merge issues ... 20200101-your-first-adr.md ... ADR metadata automatically guessed from its raw text and git logs
- https://raw.githubusercontent.com/npryce/adr-tools/master/src/adr-new (fetched 2026-09-10) — adr new -s stamps 'Superceded by' / 'Supercedes' links and strips Accepted; TOC is generated by a separate `adr generate toc` (src/_adr_generate_toc).

**Folds.** EXT-TOOLCHAI-010, EXT-TOOLCHAI-012, EXT-TOOLCHAI-018, BP-RUST-14, EXT-NOURL-002, EXT-DECISION-004, EXT-DECISION-006, EXT-DECISION-008, EXT-NOURL-011

### PIA-037 — Fully superseded ADR-0004 is the governing decision on 57 criteria in seven sections; ADR-0074 cited by none; §15.3 scopes the repair to §9

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (AC-15.3-001 (references/2026-09-08-acceptance-criteria.md:6510) records the §9 pointer only; AC-15.3-P-002 (:6506) records the generic
propagation risk; ARCH-6.6-005 records the supersession. None records the seven-section spread, the empty ADR-0074 trace, or the 0080/0082/0088 `related` edges.) · **Category:** citation-drift ·
**Confidence:** high · **Owner:** register maintainer (Ryan); ADR README steward

[measured] 57 criteria across seven sections (§2: 3; §5: 6; §7: 7; §9: 27; §10: 9; §12: 3; §13: 2 — the §15.2 list; a body count reaches 58 with AC-317) carry ADR-0004 in their `decisions:` field.
ADR-0004's frontmatter is `status: superseded`, `superseded_by: [0074]`, and the design (:908) says it is 'citable as history only'; no criterion cites ADR-0074, no criterion notes the supersession
(AC-287's parenthetical concerns ADR-0015 → ADR-0020), and §15.2 hangs all 57 on ADR-0004 with no ADR-0074 row. The register's own repair note (§15.3 :6510) scopes the misdirection to 'the whole of
§9' — 27 of the 57 — so 30 criteria in the type, kernel and adapter sections (seven of them harness ACs: AC-206, AC-208, AC-230, AC-231, AC-235, AC-236, AC-239) sit outside the repair it records. The
same-day records ADR-0080, ADR-0082 and ADR-0088 carry `related: ["0004"]` while their bodies rely on ADR-0074's definitions (`Quarantined`, `Unverifiable`, the policy table) and cite 0074 only by
legacy key ('D4's rev. 4 rebuild'); only ADR-0075 relates to 0074. The rule ('findings, not verdicts') survives into ADR-0074, so a builder following `superseded_by` lands on the right decision; the
defect is a wrong pointer in 57 places of a register that is binding on the Phase-1 runbook.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:6510` — "| ADR-0074 review gate: reviewer proposes, machine confirms | the whole of §9 (AC-280..AC-319), which cites ADR-0004 instead |"
- `references/2026-09-08-acceptance-criteria.md:6441` — "| **ADR-0004** | Findings, not verdicts; absence of evidence is a block | AC-025, AC-032, AC-038, AC-133, AC-138, AC-139, AC-140, AC-141, AC-142, AC-206, AC-208, AC-230, AC-231, AC-235, AC-236, AC-239, AC-280,"
- `references/2026-09-08-architecture-design.md:908` — "ADR-0074 **fully supersedes ADR-0004**; ADR-0004 is citable as history only (it chose *findings, not verdicts* "pending S-VAR" and named no mechanism)."
- `references/2026-09-08-acceptance-criteria.md:2721` — "**AC-206** — `rk-adapter-claude` · evidence: F20 [measured], brief §C.5 · decisions: ADR-0004, ADR-0016"
- `references/adr/0004-review-gate-findings-not-verdicts.md:4` — "status: superseded"
- `references/adr/0082-advisory-sink-is-a-first-class-destination.md:10` — "related: ["0004", "0015", "0020", "0063"]"

**Why it matters before Step 5.** A Step-5 builder resolving AC-025's, AC-133's or AC-206's authority lands on a record whose text 'named no mechanism'; ADR-0074's reviewer-proposes/machine-confirms
mechanism is what those criteria need. Reachable through the supersession link, so not a blocker; but the sweep is cheapest before the register is first read as a build input.

**Remediation.** One register amendment (the register is amendable in place): rewrite each `decisions: ADR-0004` to `ADR-0074 (supersedes ADR-0004)` or, where only the findings-not-verdicts principle
is meant, `ADR-0004 (superseded by ADR-0074; principle only)`; add an ADR-0074 row to §15.2; correct §15.3 :6510 from 'the whole of §9' to the seven sections and the count. Add one line to
references/adr/README.md's open items that the 0080/0082/0088 `related` edges to 0004 are to be read as edges to 0074, and that new records citing the review gate cite 0074. Do not edit ADR-0004,
ADR-0074 or the three revision-block records.

**Folds.** L01-contradiction-07, L02-05, slice-G04-10, L05-overlap-03

### PIA-042 — ADR-0077's gitoxide citations point at sources that say the opposite: 'shallow clone by SHA, confirmed in gitoxide #2309' cites a thread showing a panic (the API is `PrepareFetch::with_revision`, PR #2824) and crate-status.md lists 'parse trailers' as an unchecked box (docs.rs gix-object 0.64.1 `BodyRef::trailers()` exists); S-GIX has no clone arm and its /b read half is discharged, leaving the executed test

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B09a-C05] ADR-0077:112 ('One discussion thread, not a docs guarantee') and architecture-design:743 hedge the source; neither records that the
thread reports the opposite. | [B10-C03] references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:111 records the trailer-write absence as 'UNVERIFIED' and the missing docs.rs read;
references/2026-09-08-architecture-design.md:1865 names S-GIX/b as 'A docs.rs API read plus a five-line unit test') · **Category:** external-grounding/gitoxide · **Confidence:** high · **Owner:**
rk-git implementer (S-GIX/c); design owner for the re-citation

Merged from 2 concerns; each member statement is carried verbatim. (i) ADR-0077 :48 and architecture-design :743/:2014, RUNBOOK :1820 and evidence-augmentation :161 rest the hybrid's read-side bet -
'shallow clone by SHA for review checkouts' - on gitoxide discussion #2309. That thread (2025-12-28) reports 'I get a panic from inside gix ... we map by name only and have no object-id in refspec',
the maintainer's reply that `with_ref_name()` 'really only wants a reference right now', and its fix PR #2310 closed unmerged [documented, first verification of a self-flagged row]. The capability
does exist at the pin, through a different API: docs.rs gix 0.87.1 `PrepareFetch::with_revision` ('Fetch only revision and check it out with a detached HEAD ... or a full object ID'), landed by PR
#2824 on 2026-08-05 and combinable with `with_shallow` - weeks old at the pin. Trailer parsing is CONFIRMED at source (gix-object 0.64.1 `BodyRef::trailers()`), so only the clone citation drifts.
RUNBOOK §6.1 registers S-GIX/a (common dir) and S-GIX/b (trailer writing) and nothing exercises the clone path AC-339's disposable checkout depends on. (ii) ADR-0077, the design's git-layer table and
evidence-augmentation row 8 all cite gitoxide's crate-status.md as the source that 'documents trailer parsing'. crate-status.md line 259 lists 'parse trailers' as an UNCHECKED box under gix-object
commit decode; it does not document the capability. The capability nonetheless exists: docs.rs for gix-object 0.64.1 (the sub-crate behind the pinned gix 0.87.1) exposes BodyRef::trailers(),
without_trailer() and message_blocks(), so the read-side bet is right and its citation is wrong. The same docs.rs read finds no trailer-writing method on MessageRef or BodyRef, and gix-actor's write
helpers are for signatures, not trailers, so the absence the corpus flagged UNVERIFIED ('the highest-value single check in the list') is now verified against a fetched listing; the executed-test arm
of S-GIX/b remains owed. A second first-party reason for the write-path split that the ADR does not cite is the unchecked 'create, move, remove, and repair' worktree box at crate-status.md line 188.
[documented]

**Anchors.**
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:48` — "shallow clone by SHA (confirmed supported, gitoxide discussion #2309), and structured trailer parsing rather than a regex over a commit message."
- `references/2026-09-08-architecture-design.md:744` — "Separately, `crate-status.md` documents trailer **parsing**, and no first-class trailer-**writing** API was found — **but the pass could not fetch a definitive docs.rs listing either way, so the absence is UNVERIFIED, not proven**"
- `references/2026-09-08-architecture-design.md:743` — "| `gix`, at the exact pin `=0.87.1` | `rev-parse` ..., **shallow clone by SHA** for review checkouts, **trailer parsing** | Shallow clone at a specific commit is supported [documented, 1P gitoxide discussion #2309 — one thread, not a docs guarantee]."
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:23` — "**The trailer-write gap.** `crate-status.md` documents trailer *parsing*. No mature first-class trailer-*writing* API comparable to `git interpret-trailers --in-place` was found, and the agent could not fetch a definitive docs.rs API listing either way — so the absence is itself **UNVERIFIED**"
- `RUNBOOK.md:1820` — "| **Read** | `gix`, exact pin `= "=0.87.1"` | `rev-parse` (including resolving the common dir AC-178 needs at startup), repository and worktree status, shallow clone by SHA for review checkouts, trailer **parsing** |"
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:111` — "[documented, absence UNVERIFIED] `crate-status.md` documents trailer parsing; no first-class trailer-writing API was found, **and the pass could not fetch a definitive docs.rs API listing either way**"

**Why it matters before Step 5.** (i) Not Step 5. It is Step 6.1's concern: RUNBOOK says 'Run these before the worktree code, not after', and the arm that would test the clone path is missing, so
rk-kernel-verify's review checkout would be the first place with_revision meets the pin. (ii) Step 6 runs S-GIX FIRST and Step 5 does not touch rk-git, so this does not block pen-to-paper. It should
be recorded before Step 6 so S-GIX/b's pre-registration cites the API that exists (BodyRef::trailers) and states that the docs.rs read half is already discharged, leaving only the executed test.

**Remediation.** (i) Re-cite by dated amendment (architecture-design :743/:2014, RUNBOOK :1820, evidence-augmentation :161/:343) to docs.rs gix 0.87.1 `PrepareFetch::with_revision` and PR #2824;
ADR-0077 :48/:112 by successor note, not edit. Add S-GIX/c to RUNBOOK §6.1 as a third `#[ignore]`d test: `with_revision(<sha>)` + `with_shallow` against the pin, tree byte-compared to
`git clone --depth 1` followed by `git checkout <sha>`; consumer rk-kernel-verify's AC-339 disposable clone. A no-go falls back to the shell-out path ADR-0077 already permits for writes. (ii)
Supersede ADR-0077's evidence block (do not edit it): cite docs.rs gix-object 0.64.1 commit::message::body::BodyRef::trailers for trailer parsing, drop crate-status.md as the parsing source (note it
lists the box unchecked), re-tag the write-side absence from 'UNVERIFIED' to '[documented] no write API at the pin, docs.rs read 2026-09-10; executed-test arm of S-GIX/b still owed', and add
crate-status.md line 188 (worktree create/move/remove/repair unchecked) as a second reason for the shell-out. Amend S-GIX/b's registration in the design §10.5 to record the read half as discharged by
this audit and keep the five-line test as the go/no-go instrument. Owner: rk-git workstream (Step 6).

**Sources.**
- https://github.com/GitoxideLabs/gitoxide/discussions/2309 (fetched 2026-09-10) — 'Is it actually possible to do a shallow clone of a specific commit?' - 'I get a panic from inside gix ... we map by name only and have no object-id in refspec'; Byron: with_ref_name 'really only want
- https://docs.rs/gix/0.87.1/gix/clone/struct.PrepareFetch.html (fetched 2026-09-10) — pub fn with_revision(...): Fetch only revision and check it out with a detached HEAD ... or a full object ID (PR #2824, merged 2026-08-05); combinable with with_shallow.
- https://raw.githubusercontent.com/GitoxideLabs/gitoxide/main/crate-status.md (fetched 2026-09-10) — line 259 (gix-object, decode commit): '* [ ] parse [trailers](...)' UNCHECKED; line 188: '[ ] create, move, remove, and repair' worktrees
- https://docs.rs/gix-object/latest/gix_object/commit/message/struct.BodyRef.html (fetched 2026-09-10) — BodyRef (gix-object 0.64.1): trailers - Return an iterator over the trailers parsed from the last paragraph of the body; without_trailer - Returns the body with the trailers stripped. No write method.

**Folds.** EXT-GITOXIDE-005, BP-DUR-19, BP-RUST-07, BP-RUST-06

### PIA-057 — AC-388's per-sibling stagger chain is not the vendor's mechanism (hold until first response begins, cap 5 s, release together); SM-114 tests it wrong

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (known-issues ARCH-7.10-006 (architecture-design.md:1248: 'UNMEASURED locally - [documented] from platform docs only'); ARCH-10.4-018 and
ARCH-10.5-016 (S-FANOUT, 'Not registered before now'). The mechanism mismatch is unrecorded.) · **Category:** external-grounding/anthropic-platform · **Confidence:** high · **Owner:** rk-kernel
scheduler owner; SM register owner

AC-388 requires that in a fan-out 'the first is dispatched alone and each remaining sibling's dispatch timestamp is at least `i` after the previous one', citing 'platform docs' for 'stagger
identical-prefix siblings ~5 s so the first pays the write'. platform.claude.com says only 'wait for the first response before sending subsequent requests', with no number; the 5 s is Claude Code's
workflow-runtime default: 'holds all but the first until the first agent's response begins, then releases the held agents together ... caps the hold at CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS
milliseconds, 5000 by default' [documented; Wayback 2026-09-02 identical]. That is a capped wait-for-first-token with simultaneous release, not a serial (n-1) x i chain. SM-114's decision rule (i = 5
s vs i = 0, n >= 50 sibling pairs, 'AC-388's stagger requirement is dropped if no difference') therefore never tests the vendor's scheduler, and a null result would drop a rule the vendor's mechanism
might have supported. Since rk-kernel spawns `claude -p` itself, the CLI's workflow stagger does not apply to redkiln's fan-out at all; the kernel must implement whichever shape it chooses.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4827` — "**AC-388** — `rk-kernel` scheduler · evidence: §4.3 fan-out row [documented: stagger identical-prefix siblings ~5 s so the first pays the write; platform docs]; F13 [measured: 69% of the prefix is shared across directories]"
- `references/2026-09-08-acceptance-criteria.md:4832` — "THEN the first is dispatched alone and each remaining sibling's dispatch timestamp is at least `i` after the previous one"
- `references/2026-09-08-architecture-design.md:1172` — "**The stagger's benefit has no local measurement.** The rule is `[documented]` from platform documentation and nothing in this programme measured it. SM-114 makes it a decision rule ... staggered (`i` = 5 s) against simultaneous (`i` = 0), A/B over **n ≥ 50 sibling pairs**"
- `references/2026-09-08-architecture-design.md:1248` — "| Does the 5-second fan-out stagger pay? | **UNMEASURED locally** — `[documented]` from platform docs only | SM-114's A/B at **n ≥ 50 sibling pairs** → AC-388 |"

**Why it matters before Step 5.** Not Step 5; it is the rk-kernel scheduler (Step 7b) and SM-114's design. Left as is, the A/B is powerless to say anything about the vendor's mechanism.

**Remediation.** Re-cite AC-388 and architecture-design :1172/:1248 to code.claude.com/docs/en/workflows (and prompt-caching), by dated amendment. Add a third arm to SM-114 / S-FANOUT -
hold-until-first-response-begins capped at 5 s, release together - so the A/B compares serial chain, simultaneous, and the vendor's shape; consumer rk-kernel scheduler AC-388. Keep the 'dropped if no
difference' rule but apply it per arm; report a null at n < 50 as powerless, not as no difference.

**Sources.**
- https://code.claude.com/docs/en/workflows (fetched 2026-09-10) — Claude Code holds all but the first until the first agent's response begins, then releases the held agents together ... caps the hold at CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS milliseconds, 5000 by de
- https://code.claude.com/docs/en/workflows.md (fetched 2026-09-10) — Second-opinion re-fetch identical; Wayback 2026-09-02 (workflows) and 2026-09-08 (prompt-caching) carry the same text - not stale.

**Folds.** EXT-ANTHROPI-005

### PIA-064 — Claude Code `defer` (`tool_deferred`) shipped in 2.1.89 on 2026-03-31 and is documented at code.claude.com/docs/en/hooks; evidence-augmentation's 'allow/deny only, unverified' and the competitor row assert the opposite, the section 6.6 survey omits the harness's own durable per-tool-call pause, and no record priced tool-grain suspend-and-resume against ADR-0087

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B09a-C02] known-issues EA-5-013 (evidence-augmentation.md:283: 'Did Claude Code's April-2026 tool_deferred / defer permission decision ever
ship? ... Changelogs were not checked.') | [B09b-C01] EA-5-013 (references/2026-09-08-evidence-augmentation.md:283) records the question as not_answerable, 'Changelogs were not checked'; ADR-0087
records the suspend/resume decision at step grain) · **Category:** external-grounding/claude-code · **Confidence:** high · **Owner:** Design owner (evidence-augmentation and architecture-design
amendments); spike registrar for S-DEFER

Merged from 2 concerns; each member statement is carried verbatim. (i) evidence-augmentation Claim 2 asserts that Claude Code hook permission decisions are 'allow/deny only' and that the April-2026
`tool_deferred` mechanism 'could not be corroborated in first-party docs'. The first-party hooks page documents four PreToolUse outcomes (allow, deny, ask, defer) with precedence deny > defer > ask >
allow, a 'Defer a tool call for later' section, `stop_reason: "tool_deferred"`, no timeout, and the session persisted on disk until `claude -p --resume <session-id>`; the CHANGELOG files it under
2.1.89, npm-published 2026-03-31 [documented]. The corpus self-flagged this UNVERIFIED and its first verification fails on both halves. No AC or ADR anchors Claim 2, and architecture-design :854 has
already narrowed the novelty claim to the tree-hash binding, so no decision falls. Two things do: the §6.6 durable-systems survey (:841-:848) omits the one durable, resumable, per-tool-call pause in
the harness the product drives, and it carries no built-in deadline, which is the Restate shape; and ADR-0087's rejected 'hold the process open' option has a vendor-provided third shape - exit with
the pending call preserved, resume by session id across a fresh process - which the design never priced against F38's 0.45-0.78x resume cost. AC-186/AC-400 forbid `--resume` on redkiln's own CLI, not
on the child. (ii) The corpus self-flagged the `tool_deferred` / `defer` question as unverified and left the competitor row on the negative reading. The first verification fails it: the live
first-party hooks reference documents `permissionDecision (allow/deny/ask/defer)` with '"defer" exits gracefully so the tool can be resumed later', and the official CHANGELOG records it shipping in
2.1.89 ('headless sessions can pause at a tool call and resume with -p --resume to have the hook re-evaluate') [documented]. A Wayback capture of 2026-09-07 already carries it, so
evidence-augmentation:283's 'the current first-party hooks doc shows no trace' was false on the corpus's own date, and :124's two predicates ('hook permission decisions are allow/deny only';
'`/resume` resumes a whole session, not a paused tool call') are false today [documented]. The design's differentiation claim survives because it was already narrowed to the tree-hash binding inside
an AI-coding-specific backlog tool (architecture-design:854) and ADR-0087's suspension is at step grain across a kernel-owned process boundary; but the Claude adapter now has a harness-native,
headless, tool-grain suspend-and-resume primitive that no record evaluated, and the design's `PreToolUse` rule (architecture-design:1580) was written on the assumption that a hook can only allow or
deny.

**Anchors.**
- `references/2026-09-08-evidence-augmentation.md:124` — "Claude Code hook permission decisions are allow/deny only (`PermissionDenied` can set `retry:true` for the *model* to retry, not for a human to intervene) ... **Several 2026 blogs claiming an April-2026 `tool_deferred` mechanism could not be corroborated in first-party docs** — treat as unverified"
- `references/2026-09-08-evidence-augmentation.md:283` — "Did Claude Code's April-2026 `tool_deferred` / defer permission decision ever ship? | Several SEO blogs describe it; the current first-party hooks doc shows no trace. Changelogs were not checked. | **Yes** — Anthropic's changelog/release notes."
- `references/2026-09-08-architecture-design.md:854` — "This design's novelty claim is **not** "nobody ships stage-gated, mid-run, resumable human approval on a durable work item" ... The claim that survives is the narrower one: **in an AI-coding-specific backlog or spec tool**, and on the *binding* rather than the gate"
- `references/adr/0087-mid-graph-gate-resumes-into-the-same-step.md:38-40` — "### Option — hold the process open across the gate ... Rejected on prior evidence, not on P8 ... Nothing may depend on a process surviving; every wait is a durable mark plus a later read."
- `references/2026-09-08-architecture-design.md:1580` — "The `PreToolUse` deny survives as a **stateless predicate** — it reads only its own tool input and references no stage, run, session or item state | DECIDED by proposal §4.9 | AC-505"
- `references/adr/0087-mid-graph-gate-resumes-into-the-same-step.md:47` — "2. Resume must cross a real process boundary. A reload that reads state back in-process is not a valid implementation of this decision and is not evidence for it, because the risk being managed is that a *fresh* process reconstructs something different."

**Why it matters before Step 5.** (i) Not a Step-5 blocker: rk-types is unaffected. It reaches Step 7c (rk-adapter-claude) and the gate-placement decisions ADR-0017/ADR-0087, whose survey of durable
pause primitives is now known to be incomplete for the harness actually pinned. (ii) Step 5 builds the kernel, not the Claude adapter, so nothing is built wrong; but the adapter's dispatch contract
and ADR-0087's successor will be written against a competitor table that is factually wrong about the harness the product dispatches to, and an option (harness-level defer at a tool call, resumed with
`-p --resume`) would be silently absent from the option set.

**Remediation.** (i) (1) evidence-augmentation :124 and :283 by dated amendment: 'allow, deny, ask, defer; defer shipped 2.1.89 (2026-03-31), documented at code.claude.com/docs/en/hooks; session
persists on disk, no timeout'. (2) architecture-design §6.6 table (:841-:848) gains a row 'Claude Code hook defer' - no token object (session id + pending tool call in the transcript), no built-in
deadline, resumed by `claude -p --resume`; the sentence 'every platform this pass could actually read mints a single-use signal and guarantees a terminal event' is narrowed accordingly. (3) Discharge
the placement question as a named spike, not prose: S-DEFER (consumer: ADR-0087 gate placement and rk-adapter-claude's AC-215 stall guard) - drive a PreToolUse hook returning defer under the pinned
CLI, kill the parent, resume in a fresh process, and record survival, drift and resume cost against F38; a go narrows ADR-0087 by successor record only. (4) No supersession of ADR-0017/0087 until
S-DEFER reports. (ii) Append a dated erratum to evidence-augmentation §1 row 'Claim 2' and §5 row 283 (the pass is a dated record; do not rewrite the rows) stating: defer shipped in 2.1.89, is
documented at code.claude.com/docs/en/hooks#defer-a-tool-call-for-later, and was present on 2026-09-07. Register a reading-plus-fixture spike (name it R-DEFER; consumer: the Claude adapter's dispatch
contract under ADR-0010's successor and ADR-0087's successor) that answers two questions on the pinned `claude` version: whether a `defer` from a settings-based `PreToolUse` hook fires under `-p` (F2
measured 2 hook events per dispatch, so the fixture exists), and whether a deferred tool call resumed with `-p --resume` preserves the tree-hash binding the design requires. Until it runs, the
differentiation sentence at architecture-design:854 stands unchanged and ADR-0087 stands unchanged; the erratum names R-DEFER as the discharge, per ADR-0012.

**Sources.**
- https://code.claude.com/docs/en/hooks (fetched 2026-09-10) — '"defer" is for integrations that run claude -p as a subprocess ... The tool doesn't execute. The process exits with stop_reason: "tool_deferred" and the pending tool call preserved in the transcript.
- https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md (fetched 2026-09-10) — ## 2.1.89 (npm 2026-03-31): 'Added "defer" permission decision' - first-party changelog entry the corpus said was not checked.
- https://code.claude.com/docs/en/hooks (fetched 2026-09-10) — permissionDecision (allow/deny/ask/defer); 'defer' exits gracefully so the tool can be resumed later; anchor #defer-a-tool-call-for-later (25 occurrences)
- https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md (fetched 2026-09-10) — 2.1.89: Added 'defer' permission decision to PreToolUse hooks - headless sessions can pause at a tool call and resume with -p --resume to have the hook re-evaluate
- https://web.archive.org/web/20260907124852id_/https://code.claude.com/docs/en/hooks (fetched 2026-09-10) — 2026-09-07 capture already documents defer, one day before the corpus's 'shows no trace'
- https://github.com/anthropics/claude-code/issues/41791 (fetched 2026-09-10) — docs-omission report for defer, since remedied

**Folds.** EXT-CLAUDECO-030, EXT-STATEDAB-027

### PIA-069 — `rate_limit_event` is now documented in both Agent SDK references with a shape F1 did not measure, and the SDK lists seven_day_opus/sonnet keys

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (known-issues ARCH-0.1-033 (architecture-design.md:51: 'undocumented top-level rate_limit_event ... no Opus/Sonnet family sub-limit key');
acceptance-criteria.md:2663 tags the surface '[surface unverified]'.) · **Category:** external-grounding/claude-code · **Confidence:** high · **Owner:** rk-adapter-claude implementer; design owner for
the amendment

The design's throttle decision rests on F1: headroom 'is readable only via an undocumented top-level `rate_limit_event`' carrying `unifiedWindows.{five_hour,seven_day}.utilization`, and 'there is **no
Opus/Sonnet family sub-limit key**'. Today the TypeScript Agent SDK reference types
`SDKRateLimitEvent = { type: "rate_limit_event"; rate_limit_info: { status: "allowed" | "allowed_warning" | "rejected"; resetsAt?; utilization?; ... } }` and the Python reference types the same event,
with `RateLimitType` listing `seven_day_opus` and `seven_day_sonnet` [documented]. The headless page still has zero mentions, and neither reference documents `unifiedWindows` or `isUsingOverage`, so
F1's fields remain undocumented - but 'the docs do not mention' the event is no longer true, the documented shape differs from the measured one, and 'no family sub-limit key' is an absence in F1's
runs, not a fact about the CLI. The adapter (AC-030) must derive `RateLimited { resets_at }` from this event; which shape the pinned 2.1.266/2.1.268 emits is unmeasured.

**Anchors.**
- `references/2026-09-08-architecture-design.md:51` — "Plan headroom is readable only via an undocumented top-level `rate_limit_event` (`result.rate_limit` is null), and there is **no Opus/Sonnet family sub-limit key** — the binding limit cannot be observed, so throttling is on `five_hour` with margins of ≥2 ticks"
- `references/2026-09-08-acceptance-criteria.md:468` — "**AC-030** — `rk-kernel::retry` · evidence: **F22** [measured]; F1/F6 [measured: the top-level `rate_limit_event` is the"
- `references/2026-09-08-acceptance-criteria.md:476` — "arrives on `rate_limit_event`, **not** on a failure result — the adapter must derive `RateLimited { resets_at }` from"
- `references/2026-09-08-acceptance-criteria.md:2663` — "The brief records only `rate_limit_event.unifiedWindows.{five_hour,seven_day}.utilization` (F1) ... Each such criterion is tagged **[surface unverified]**: a green synthetic test on an unverified surface is an unproven criterion."
- `references/2026-09-04-findings-for-product.md:67` — "headroom is a top-level `rate_limit_event` the docs do not mention, carrying"

**Why it matters before Step 5.** Step 7c (rk-adapter-claude) parses this event. Two shapes now exist - measured and documented - and the pinned CLI is a third version from F1's instrument; the
adapter's parser and AC-030's fixture must be captured from the pin, not from F1.

**Remediation.** Amend architecture-design :51 by dated amendment: 'documented in the Agent SDK references as rate_limit_info {status, resetsAt, utilization}; the unifiedWindows shape F1 read is not
documented; no family sub-limit key was observed in F1's runs (the SDK lists seven_day_opus/seven_day_sonnet)'. Cite code.claude.com/docs/en/agent-sdk/typescript and /python. Add to AC-030's
verification a captured `rate_limit_event` fixture from the pinned CLI version recorded beside TESTED_CLI_VERSION, and make the parser tolerant of both shapes with a named error on neither.

**Sources.**
- https://code.claude.com/docs/en/agent-sdk/typescript (fetched 2026-09-10) — ### SDKRateLimitEvent - type SDKRateLimitEvent = { type: "rate_limit_event"; rate_limit_info: { status: "allowed" | "allowed_warning" | "rejected"; resetsAt?: number; utilization?: number; ... } }
- https://code.claude.com/docs/en/agent-sdk/python.md (fetched 2026-09-10) — Python reference types the same event; RateLimitType lists seven_day_opus / seven_day_sonnet; grep for unifiedWindows and isUsingOverage: 0 hits in both references.

**Folds.** EXT-CLAUDECO-024

### PIA-074 — ADR-0086's crate facts were wrong on its own 2026-09-08 read date (temporalio-sdk 1.0.0 stable on sdk-core 0.9.0; graph-flow 0.8.0 with postgres enabled by default, 0.6 hard-required sqlx; dbos 0.5.0 first-party; iopsystems/durable has tags), its 'Rust second-class everywhere' driver is false, the time-skipping test-server item is answered, and :55 propagates the wrong versions

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B09a-C03] known-issues EA-1-005 (evidence-augmentation.md:33) and ARCH-0.1-017 (architecture-design.md:35) record the re-argument;
ADR-0086-FORB-78/80/84 record the exclusion's limits. None records that the versions were stale on 2026-09-08. | [B10-C01]
references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:61 and :85 register the time-skipping test-server question as unchecked; :90 dates the crates.io read 2026-09-08; :99 says
'three do, and one is a maintained first-party SDK') · **Category:** external-grounding/crate-versions · **Confidence:** high · **Owner:** ADR steward (successor record); design owner for the
amendments

Merged from 2 concerns; each member statement is carried verbatim. (i) ADR-0086 :90 claims a same-day crates.io read on 2026-09-08 that 'strictly dominates' the charter. On that date `temporalio-sdk`
was 1.0.0 (published 2026-09-04, release note 'the Rust SDK is now considered stable'; every 0.x tag marked pre-release) on `temporalio-sdk-core` 0.9.0, not 'v0.5.0, Public Preview, updated June
2026'; and `graph-flow` was 0.8.0 (2026-09-07), not 0.6 [documented]. The mandatory-dependency list ADR-0086 :20/:91 attributes to v0.6 is exactly 0.8.0's: at 0.6.0 sqlx ^0.8.6 was REQUIRED, so the
charter's 'hard-depends on sqlx+postgres' was true when written and became false only on 2026-09-07; and at 0.8.0 `postgres` is a DEFAULT feature, so 'no required database' holds only with
default-features = false. :55 orders every document to 'correct to the facts above, naming the live crates and versions', which propagates the wrong versions. Riders: the charter's 'dbos crate is 12
lines with 39 downloads' is stale (dbos 0.5.0, 2026-09-09, sqlx+postgres required); 'iopsystems/durable ... no version or release tags visible' is false on the tags endpoint (v0.6.2, durable-v0.6.0) -
it has tags but no GitHub Releases. The exclusion (server mandate; in-memory not durable; Postgres + WASM) does not turn on any of these and is, if anything, strengthened by postgres-by-default. (ii)
ADR-0086 re-argues the durable-execution exclusion 'on facts that hold' and dates every fact to a live read on 2026-09-08, yet the facts it names were already stale that day: temporalio-sdk was at
1.0.0 on temporalio-sdk-core 0.9.0 (published 2026-09-04, release notes declare the SDK stable), not 'v0.5.0, June 2026, Public Preview'; graph-flow was at 0.7.0 (0.8.0 on 2026-09-07), not v0.6, and
its postgres feature is enabled by default, the exact shape the ADR's feature-flag clause guards against; iopsystems/durable has no Releases but does carry version tags (v0.6.2, durable-v0.6.0). The
decision driver 'Rust remains second-class across every first-party durable-execution SDK checked' is false today: Temporal's Rust SDK is GA and DBOS Inc. published a first-party `dbos` crate (0.5.0,
2026-09-09) that is a library with no server, so 'the server is the arbiter' does not cover it either. The registered open item (a time-skipping local test server) is now answered:
`WorkflowEnvironment::start_local` exists in the SDK's testing feature. None of this reopens the exclusion, which survives on ADR-0003 (dbos hard-requires sqlx/postgres; Temporal still needs a server)
and on the ADR's own ban on re-litigating maturity, but a record whose stated purpose is citability now carries four stale currency facts, one false driver and one missing option row. [documented]

**Anchors.**
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:19` — ""`temporal-sdk-core` on crates.io is a 2021 alpha placeholder" — **withdrawn**. Temporal shipped a native Rust SDK in Public Preview: `temporalio-sdk` on `temporalio-sdk-core` v0.5.0, updated June 2026, actively maintained."
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:32` — "Rust remains second-class across every first-party durable-execution SDK checked, which is a standing reason to expect to build rather than adopt."
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:20` — ""`graph-flow` hard-depends on `sqlx`+postgres" — **withdrawn**. graph-flow v0.6 (docs July 2026) makes postgres an **optional** Cargo feature and ships `InMemorySessionStorage` with no required database."
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:90` — "[documented, 1P] `temporalio-sdk` on `temporalio-sdk-core` **v0.5.0**, updated June 2026, Public Preview, actively maintained — crates.io and `github.com/temporalio/sdk-rust`, read 2026-09-08."
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:55` — "**The retired facts are struck.** No redkiln document may argue this exclusion from "`temporal-sdk-core` is a 2021 alpha placeholder" or "`graph-flow` hard-depends on sqlx+postgres". Where those sentences appear they are corrected to the facts above, naming the live crates and versions."
- `references/adr/0086-no-durable-execution-framework-memo-table-built-here.md:43` — "Maturity is additionally unresolved: no version or release tags visible and roughly 58 stars (§5)."

**Why it matters before Step 5.** (i) No build consequence: ADR-0086 forbids all four crates regardless. It is a correctness-of-record defect in an accepted ADR whose :55 clause propagates the wrong
versions into every document that cites it. (ii) Not a Step-5 blocker: nothing built in Step 5 depends on ADR-0086's option table, and the exclusion stands on ADR-0003. It is a correctness-of-record
defect in an ADR whose first decision driver is 'the record must be citable', and it should be fixed before the memo table (Step 7) cites 0086 as its authority.

**Remediation.** (i) Issue a successor erratum record to ADR-0086 (accepted, immutable) stating: temporalio-sdk 1.0.0 (2026-09-04, 'considered stable') on temporalio-sdk-core 0.9.0 (crates.io, fetched
2026-09-10); graph-flow 0.8.0 (2026-09-07) with `default = ["postgres"]` and sqlx optional only under default-features = false; graph-flow 0.6.0 hard-required sqlx, so the charter's sentence is
'superseded by 0.8.0', not 'false'; dbos 0.5.0 (2026-09-09, sqlx+postgres required); iopsystems/durable carries tags (v0.6.2) and no GitHub Releases. Replace the :55 instruction with 'cite the
successor record's versions'. Amend architecture-design :35/:368/:2017 and evidence-augmentation :33 by dated amendment, and do not write 'v1.0.0' against sdk-core. The :61 open item (time-skipping
test server) stays open; this bucket did not check it. (ii) Supersede ADR-0086 (accepted records are immutable) with a record that: (1) replaces the temporalio-sdk fact with '1.0.0 on sdk-core 0.9.0,
2026-09-04, stable (GA), rust-version 1.92' and the graph-flow fact with '0.8.0; postgres feature optional but enabled by default, so `default-features = false` is required to exclude it'; (2) rewords
the durable maturity sentence to 'no GitHub Releases; version tags through v0.6.2'; (3) strikes the 'Rust remains second-class' driver and adds an Option row for `dbos` 0.5.0 (library, no server,
database arbitrates, hard-requires sqlx/postgres, 'Under construction'), rejected on ADR-0003 alone; (4) closes the registered test-server item as answered (start_local, test-only, does not reopen the
exclusion) and lifts the 'Forbids claiming ... no local or time-skipping test mode' clause accordingly. Owner: Ryan, as ADR-0086's author.

**Sources.**
- https://crates.io/api/v1/crates/temporalio-sdk (fetched 2026-09-10) — "max_version":"1.0.0"; versions: 1.0.0 2026-09-04, 0.8.0 2026-09-02, 0.7.0 2026-08-17, 0.6.0 2026-08-04, 0.5.0 2026-06-29; GitHub v1.0.0: 'the Rust SDK is now considered stable'.
- https://crates.io/api/v1/crates/temporalio-sdk-core (fetched 2026-09-10) — "max_version":"0.9.0" (2026-09-04); 0.5.0 exists (2026-06-29) and was superseded four times before 2026-09-08.
- https://crates.io/api/v1/crates/graph-flow/0.6.0/dependencies (fetched 2026-09-10) — graph-flow 0.6.0 (2026-07-19): sqlx ^0.8.6 REQUIRED; features {default:[], rig}. 0.8.0 (2026-09-07): default=[postgres], postgres=[dep:sqlx]; sqlx optional.
- https://api.github.com/repos/iopsystems/durable/tags (fetched 2026-09-10) — tags: v0.6.2, durable-worker-v0.5.0, durable-v0.6.0 ...; releases: []; stars 58; README requires PostgreSQL and WASI components.
- https://crates.io/api/v1/crates/temporalio-sdk (fetched 2026-09-10) — temporalio-sdk max_version 1.0.0 (2026-09-04); 0.5.0 was 2026-06-29; GitHub release v1.0.0: 'With this 1.0.0 release, the Rust SDK is now considered stable.'
- https://crates.io/api/v1/crates/graph-flow (fetched 2026-09-10) — max_version 0.8.0 (2026-09-07); README: '`postgres` (enabled by default) ... set `default-features = false` to exclude it.'
- https://api.github.com/repos/iopsystems/durable/tags (fetched 2026-09-10) — tags v0.6.2, durable-v0.6.0, durable-v0.5.10; releases: 0; 58 stars; pushed 2026-09-10
- https://crates.io/api/v1/crates/dbos (fetched 2026-09-10) — dbos 0.5.0 updated 2026-09-09, repo dbos-inc/dbos-transact-rust, sqlx ^0.9 REQUIRED; README: 'no separate service or orchestrator or any external dependencies except Postgres.'
- https://raw.githubusercontent.com/temporalio/sdk-rust/main/crates/sdk/README.md (fetched 2026-09-10) — testing feature: 'start an isolated Temporal CLI dev server for workflow tests ... WorkflowEnvironment::start_local(...)'

**Folds.** EXT-CRATEVER-004, EXT-CRATEVER-005, EXT-CRATEVER-006, EXT-DURABLEE-002, EXT-DURABLEE-012, EXT-DURABLEE-013, BP-DUR-01, BP-DUR-04, BP-DUR-05, BP-DUR-08, BP-DUR-03, BP-RUST-19

### PIA-079 — 'No first-party source' for the thiserror/anyhow split is false in six live places: the crate author's own anyhow README and thiserror docs document it, and Microsoft M-APP-ERROR exists with a named divergence on in-repo crates; the stricter 'never in a library's public API' stays a house rule

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B09b-C05] ADR-0083:121 records the convention as '[documented, weak] ... on explicitly thin evidence' (the area); it does not record that the
statement 'no first-party source' is false | [B10-C09] references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:121 tags the split '[documented, weak] ... two 2026 practitioner blogs and no
first-party source') · **Category:** citation-fidelity/live-ADR · **Confidence:** high · **Owner:** ADR-0083 successor author

Merged from 2 concerns; each member statement is carried verbatim. (i) ADR-0083:72 and :121, architecture-design:1673, :1721 and :2051, and evidence-augmentation:55 all state that the
thiserror-in-libraries / anyhow-in-application split rests on 'two 2026 practitioner blogs and no first-party source'. The canonical statement of the split is the crate author's own: dtolnay's anyhow
README, 'Comparison to thiserror' ('Use Anyhow if you don't care what error type your functions return ... This is common in application code. Use thiserror if you are a library'), and thiserror's
docs.rs page ('See also the anyhow library for a convenient single error type to use in application code') [documented]. The evidence is first-party and strengthens the ADR. What remains house
convention is the stricter clause: the README recommends 'application code', it does not forbid `anyhow::Error` in a library's public API, so ADR-0083:72's 'never' stays a rule the project sets, not
one the source documents. anyhow 1.0.104 (architecture-design:1673) is current on crates.io. (ii) ADR-0083 and the design's dependency table adopt 'thiserror in library crates, anyhow confined to
binary crates' as a house convention 'corroborated by two 2026 practitioner blogs and by no first-party source'. A vendor guideline now exists (Microsoft, generated 2026-08-19) and largely agrees,
with two named divergences: M-APP-ERROR lets 'crates in your own repository exclusively used from your application' use anyhow/eyre, where the ADR forbids anyhow in every non-binary crate; and
M-ERRORS-CANONICAL-STRUCTS recommends situation-specific error structs with Backtrace rather than enums for library errors. The ADR's stricter rule is defensible for adapter crates a second binary may
share (it already notes the cost at :99), but the evidence tag 'thin, no first-party source' is now wrong and should read 'vendor-corroborated with a named divergence'. [documented]

**Anchors.**
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:72` — "This split is corroborated by two 2026 practitioner blogs and by **no first-party source**; it is adopted as a house convention on that basis, not as a documented standard."
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:121` — "[documented, weak] `thiserror` for libraries and `anyhow` for the application layer is restated as current best practice by **two 2026 practitioner blogs and no first-party source** ([blog] OneUptime 2026-01-25; Pi Stack 2026-06-22), §2. Adopted as a house convention on explicitly thin evidence."
- `references/2026-09-08-architecture-design.md:1673` — "| `anyhow` | 1.0.104 | caret | **binary crates only** | **proven** | ADR-0083 — the split rests on **two 2026 practitioner blogs and no first-party source** (see §9.3) |"
- `references/2026-09-08-architecture-design.md:1721` — "Restated by **two 2026 practitioner blogs and by no first-party source** [blog, OneUptime 2026-01-25; Pi Stack 2026-06-22]. Adopted as a house convention **on that basis** — it is not a documented standard, and this document does not present it as one"
- `references/2026-09-08-evidence-augmentation.md:55` — "[blog] OneUptime 2026-01-25; Pi Stack 2026-06-22 — two blogs, no first-party source"

**Why it matters before Step 5.** (i) No build consequence; the convention is unchanged and better grounded than recorded. (ii) No build consequence; the rule Step 5 applies to rk-types (thiserror) is
unchanged. Record hygiene only.

**Remediation.** (i) In the ADR-0083 superseding clause (B09-C03), replace 'no first-party source' with a citation of the anyhow README 'Comparison to thiserror' section and docs.rs/thiserror, upgrade
the grade from [documented, weak] to [documented, 1P crate author], and state explicitly that 'a library's public API never returns `anyhow::Error`' is the project's rule beyond the source's
recommendation. Dated erratum to the design's §9.3 crate table, §9.3 conventions table and §11 source table, and to evidence-augmentation §2. (ii) In the ADR-0083 superseding record, cite M-APP-ERROR
and M-ERRORS-CANONICAL-STRUCTS, keep the stricter binary-only rule with its reason (adapter crates are shared libraries), and re-tag the evidence 'vendor-corroborated, one named divergence'. Update
design table row 1673's rationale cell. Owner: ADR-0083 author.

**Sources.**
- https://raw.githubusercontent.com/dtolnay/anyhow/master/README.md (fetched 2026-09-10) — Comparison to thiserror: 'Use Anyhow if you don't care what error type your functions return ... common in application code. Use thiserror if you are a library'
- https://docs.rs/thiserror/latest/thiserror/ (fetched 2026-09-10) — thiserror 2.0.20: 'See also the anyhow library for a convenient single error type to use in application code.'
- https://docs.rs/anyhow/latest/anyhow/ (fetched 2026-09-10) — anyhow 1.0.104 live; same comparison text
- https://microsoft.github.io/rust-guidelines/print.html (fetched 2026-09-10) — M-APP-ERROR: 'Applications, and crates in your own repository exclusively used from your application, may use ohno::AppError, anyhow, eyre or similar ... Libraries ... M-ERRORS-CANONICAL-STRUCTS.'

**Folds.** EXT-STATEDAB-013, EXT-STATEDAB-014, EXT-STATEDAB-021, EXT-STATEDAB-024, EXT-STATEDAB-050, EXT-STATEDAB-052, BP-RUST-15

### PIA-082 — Five DECIDED Phase-1 rules cite ADRs whose bodies do not contain them and no ADR does: the CLI shape (0011), the StructuredOutput configuration-time check (0016), reconcile-at-every-entry, the Slice-Verdict axis and the tier split (0018), and the batch-delta/2-tick headroom rules (0003); the forbids machinery protects none of them

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B07-C02] ARCH-0.1-028 (references/2026-09-08-architecture-design.md:46) records the StructuredOutput row as a missing artifact and AC-0.3-028
(:91) its four homes; acceptance-criteria.md:6498 records the ADR-table title audit as discharged. The ADR-0011 and ADR-0018 legs are NEW (adr-graph: consumers [] for 0011; ARCH-0.1-020/021 record the
reconcile rows as n_equals_1 only). | [B08-C12] AC:6496 records that 'the ADR text itself was outside the refutation pass's evidence set in every area' with ADR-0003's 96 citations concentrating the
risk; AC-15.3-P-002 (AC:6506) records criteria citing an older ADR for behaviour they cover; ARCH-0.1-003 and ARCH-0.1-014 record 'no ADR carries it' for stale_policy and distribution; AC-UNVER-011
records the >= 2-tick rule standing on unverified proposal text. None names the tier-split or throttle rows; the ADR-0018 miscitation is unrecorded (NEW half, refuter-confirmed).) · **Category:**
citation-drift · **Confidence:** high · **Owner:** Ryan (ADR author); register maintainer (Claude)

Merged from 2 concerns; each member statement is carried verbatim. (i) [measured] Three rules the design's §0.1 marks DECIDED cite a decision record whose body does not contain them, and grep over
references/adr/ finds no record that does. (i) ADR-0011's body decides the two-pack set and the Phase-1 exit bar only — no 'resume', 'init' or CLI text, consumers [] — yet AC-400 (resume-flag
denylist), AC-437 (`init` idempotence), design rows 1267/1273/1296/1609 (single `run` entry, no resume flag) and SM-130 :963 ('deterministic resume across kill') cite it, and the §15 table re-titles
it 'CLI shape and the Phase 1 exit criterion'. (ii) ADR-0016 decides card delivery and a TTL pin (:35) and is itself partially superseded by ADR-0078, yet AC-095, AC-205, AC-417 and AC-504 carry
`decisions: ADR-0016` for the configuration-time StructuredOutput check that §0.1 row 46 marks DECIDED and gates-Phase-1 'Yes'; only ADR-0084 mentions StructuredOutput, as evidence. (iii) ADR-0018
decides rewrite events and the worktree-ownership refusal (:38), yet §0.1 rows 38/39, §5.6 and AC-172..175/AC-183 cite it for 'reconcile() at every entry, and no optimisation that skips it' and the
Slice-Verdict trailer axis; proposal §4.5 rule 3 was a consistency rule, never a D-row. After the :6498 title audit ('all ten are corrected') the §15 table also still glosses ADR-0069 as '`unknown` is
never ordered against a band', a word absent from that record. Nothing is built wrong — AC-400, F20 with Checkpoint 7c, and AC-172/173 carry the rules into code — but ADR-0085 rule 8 makes the AC→ADR
trace mandatory, and the immutability/forbids machinery protects none of the three: an implementer adding a reconcile cache or a `--resume` flag violates no accepted ADR, which is where RUNBOOK Step 8
has already drifted (G05-03). (ii) ADR README:3 rules that 'A decision is cited from here, not from the prose that first argued it.' ADR-0018's Decision (line 38) is rewrite events plus the
common-dir/HEAD refusal; its body never mentions tiers as a decision, trailers, `Story:` or `Slice-Verdict:`, and no file under references/adr/ contains 'Slice-Verdict' [measured]. Yet section 0.1 row
38 (a Phase-1-gating DECIDED row) and section 5.4 line 684 say 'A `Slice-Verdict:` axis is therefore required (ADR-0018)', and AC-160 to AC-165 each list 'decisions: ADR-0018'; the tier-1 test and the
two-axis rule are decided only in the superseded proposal section 4.5 and RFC section 3.4, restated in design section 5.4. Likewise grep of ADR-0003 for headroom|throttl|rate_limit|utilization returns
0 lines and its forbids block covers ports, git and Postgres only [measured], while section 7.9 (design:1233-1235) attributes three decisions (batch-delta-only headroom, >= 2-tick margins on the
visible windows, no cache metric in a headroom figure) to ADR-0003 and AC-372 to AC-381 each carry 'decisions: ADR-0003' (78 such citations register-wide); the only ADR mentioning headroom at all is
ADR-0088, incidentally at line 72. Check C (ADR agreement) cannot be performed for either area, and a future supersession of the tier or throttle rule has no record to supersede.

**Anchors.**
- `references/2026-09-08-architecture-design.md:1296` — "| One entry point, no resume flag | any "pick up where you left off" verb or flag | argument-surface snapshot + denylist lint (AC-400) | design decision; ADR-0011 [documented] |"
- `references/2026-09-08-architecture-design.md:38` — "so a `Slice-Verdict:` axis is required. [measured, F12; the registered primary was uncomputable at n=1 and that is disclosed] ADR-0018 | — | Yes |"
- `references/2026-09-08-acceptance-criteria.md:6448` — "| **ADR-0011** | CLI shape and the Phase 1 exit criterion | AC-400, AC-405, AC-437, AC-473 | 4 |"
- `references/2026-09-08-architecture-design.md:684` — "A `Slice-Verdict:` axis is therefore required (ADR-0018)."
- `references/2026-09-08-acceptance-criteria.md:6210` — "brief §C5 ("forces `StructuredOutput` in every card whose role demands a schema, checked at configuration time") · decisions: ADR-0016, proposal §4.9"
- `references/adr/0018-rewrite-events-and-common-dir-worktree-ownership.md:38` — "On rebase or amend, the kernel emits a `rewrite(old_sha → new_sha)` event rather than silently re-keying or dropping the row. At startup, the kernel resolves the repository's common dir (`git rev-parse --git-common-dir`) and refuses to write when the current worktree's HEAD is not the run's branch."

**Why it matters before Step 5.** (i) Step 5 builds rk-types/rk-kernel against `decisions:` fields; a reader following AC-504 finds a record about prompt caching and AC-173 a record about rewrite
events. No wrong code results, but the forbids machinery that is supposed to stop a later 'optimisation' is absent for three rules the kernel and CLI are built on. (ii) The decision content is
unambiguous in design sections 5.4 and 7.9, so Steps 5-7 build the right thing; the defect is the authority trail on a Phase-1-gating DECIDED row and a Phase-1 adapter area.

**Remediation.** (i) Cut three new ADRs (next free numbers after the RUNBOOK's 0090/0091 allocations): the CLI shape (one idempotent `run <item>` entry, committed denylist, `init` idempotence;
consumers AC-400, AC-437, design §8); the StructuredOutput configuration-time rule (evidence F20; consumers AC-095, AC-205, AC-417, AC-504 and the doctor/kernel rule tables); reconcile-at-every-entry
plus the two-axis trailer rule with a forbids block ('no cache, TTL or flag may skip reconcile(); no single-axis trailer resume'; evidence F12/F19). Re-point the `decisions:` fields, the design rows,
the §15 consumer rows and SM-130 (cite design §10.2 exit (3) for the kill line) at them. Regenerate the §15 title column from frontmatter verbatim (ADR-0085 rule 2) and move glosses to a note column.
Leave ADR-0011, ADR-0016, ADR-0018 and ADR-0069 untouched. (ii) Author new ADRs (next free numbers after Step 6's 0091; do not edit ADR-0018 or ADR-0003): one recording the tier-1 test, the tier lists
and the three trailer axes with F12 as evidence (`supersedes: []`, `related: ["0018"]`), and one consolidating section 7.9's three rows ('headroom is batch-delta only; throttle on the visible unified
windows with >= 2-tick margins; no family value synthesised') with a forbids block mirroring AC-373/375/381. Re-point section 0.1 row 38, section 5.4 line 684, section 7.9 and the 'decisions:' fields
of AC-160..AC-165 and AC-372..AC-381 to them, leaving ADR-0003 only where the port/fake is the actual decision.

**Folds.** G05-07, slice-G10-04, G13-02, L04-12, slice-G14-04, slice-G17-06

### PIA-083 — ADR-0084's advisory evidence misattributes three gix advisories, 'no advisory affects bundled' is false, and gix has one crates.io owner

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (known-issues EA-5-004 (evidence-augmentation.md:274: 'rustsec.org 404'd on direct fetch; advisory data was reconstructed per-advisory ... absence
of evidence, not a clean bill'); ARCH-0.1-038 (architecture-design.md:56). The wrong attributions themselves are unrecorded.) · **Category:** external-grounding/advisories · **Confidence:** high ·
**Owner:** ADR steward (successor record); design owner for amendments

The advisory table at architecture-design :1697-:1700, repeated in ADR-0084 :19 and evidence-augmentation :160, was reconstructed under a rustsec.org 404 and carries four attributions the advisories
do not support [documented, first verification of self-flagged rows]: RUSTSEC-2025-0021 is patched at gix-features >= 0.41.0 (0.71.0 is the gix facade's fixed version, and the adjacent row uses the
sub-crate scale, so the column is inconsistent); RUSTSEC-2024-0351 is against gix-ref (>= 0.44.0) and RUSTSEC-2024-0353 against gix-worktree (>= 0.34.0), not `gix`; RUSTSEC-2024-0349's surface is
'crafted tree or blob names' inside a cloned repository, not 'crafted repo names'. ADR-0084 :115's 'no advisory affects `bundled`' is false as an absolute: RUSTSEC-2022-0090 (CVE-2022-35737) is filed
against libsqlite3-sys < 0.25.1 precisely because the bundled SQLite carried the CVE. ADR-0084 :19's 'multi-maintainer ... so not a single-author risk' is not what its sources show: crates.io lists
one owner (Byron) and no team. Every pin is clear on every item - gix 0.87.1 pulls gix-features ^0.49.1, gix-ref ^0.67.1, gix-worktree ^0.56.0; rusqlite 0.40.2 pulls libsqlite3-sys ^0.38.2 - so this
is the record, not the build.

**Anchors.**
- `references/2026-09-08-architecture-design.md:1697` — "| RUSTSEC-2025-0021 / CVE-2025-31130 | `gix-features` | missing SHA-1 collision detection | ≥ 0.71.0 |"
- `references/2026-09-08-architecture-design.md:1700` — "| RUSTSEC-2024-0351 / 0353 | `gix` | Windows reserved device names | before 0.87.1 |"
- `references/adr/0084-dependency-policy.md:19` — "It is multi-maintainer (GitoxideLabs, Sebastian Thiel leading), so not a single-author risk, but squarely a 0.x risk. Its RUSTSEC history is dense: RUSTSEC-2025-0021/CVE-2025-31130 (`gix-features`, missing SHA-1 collision detection) ... RUSTSEC-2024-0349 (`gix-worktree` path traversal / arbitrary fi"
- `references/adr/0084-dependency-policy.md:115` — "[documented] rusqlite's two historical advisories (RUSTSEC-2020-0014; RUSTSEC-2021-0128 ...) are far behind 0.40.2, and no advisory affects `bundled`. [1P] RustSec advisory-db, §3."
- `references/2026-09-08-evidence-augmentation.md:160` — "Dense RUSTSEC history against `gix-*`: RUSTSEC-2025-0021/CVE-2025-31130 (gix-features, missing SHA-1 collision detection, fixed ≥0.71.0); ... RUSTSEC-2024-0351/0353 (Windows reserved device names)"
- `references/2026-09-08-architecture-design.md:2015` — "rustsec.org **404'd on direct fetch**; the `gix-*` list was reconstructed per-advisory and **is not guaranteed complete** | The advisory cluster in checkout and path handling — all fixed at or before 0.87.1, so **the pin is CLEAN as of 2026-09-08**"

**Why it matters before Step 5.** Step 5.2 wires `cargo audit` into CI and Checkpoint 5 requires it green; that check, not this table, is authoritative and it will pass. The defect is a security note
in an accepted ADR that a future contributor is told they 'need not re-derive'.

**Remediation.** Successor erratum record to ADR-0084 (accepted, immutable) correcting :19 and :115: per-crate fixed versions (gix-features >= 0.41.0; gix-ref >= 0.44.0; gix-worktree >= 0.34.0;
gix-worktree-state >= 0.17.0), 'crafted tree or blob names', the omitted RUSTSEC-2024-0352 (gix-index) under the existing 'not complete' flag, 'three advisories in the rusqlite family, one
(RUSTSEC-2022-0090, libsqlite3-sys < 0.25.1) against bundled SQLite', and 'multi-contributor, single crates.io owner'. Amend architecture-design :1697-:1700/:2015 and evidence-augmentation :160/:162
by dated amendment. Keep `cargo audit` at Step 5.2 as the authoritative check and add `cargo deny` advisories so libsqlite3-sys is covered by lockfile, not by prose.

**Sources.**
- https://rustsec.org/advisories/RUSTSEC-2025-0021.html (fetched 2026-09-10) — Package gix-features ... Patched >=0.41.0; GHSA-2frx-2596-x5r6 lists gix range < 0.71.0 patched 0.71.0 (facade), gix-features < 0.41.0.
- https://rustsec.org/advisories/RUSTSEC-2024-0351.html (fetched 2026-09-10) — RUSTSEC-2024-0351: gix-ref ... Patched >=0.44.0 Affected OSes windows. RUSTSEC-2024-0353: gix-worktree ... Patched >=0.34.0.
- https://rustsec.org/advisories/RUSTSEC-2024-0349.html (fetched 2026-09-10) — gix-worktree: Traversal outside working tree ... crafted tree or blob names ... A tree named .git ... facilitates arbitrary code execution because hooks can be installed. Patched >=0.34.0
- https://rustsec.org/advisories/RUSTSEC-2022-0090.html (fetched 2026-09-10) — libsqlite3-sys via C SQLite CVE-2022-35737 ... As libsqlite3-sys bundles SQLite, it is susceptible ... affected < 0.25.1.
- https://crates.io/api/v1/crates/gix/owners (fetched 2026-09-10) — owner_user: ['Byron']; owner_team: []. GitHub org GitoxideLabs public members: Byron, EliahKagan; gix 0.87.1 still latest (2026-08-24).

**Folds.** EXT-ADVISORI-002, EXT-ADVISORI-005, EXT-ADVISORI-006, EXT-ADVISORI-007, EXT-SQLITE-005, EXT-GITOXIDE-002

### PIA-085 — Design section 6.6's durable-systems survey carries wrong cells: Step Functions' 'max activity TTL 6 h' is in no AWS source (the cap is the one-year execution quota), DBOS expires to a sentinel not an error, and Azure Durable mints no token

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B09a-C08] known-issues EA-5-009 (evidence-augmentation.md:279: Azure 'Searches returned only tangential pages'); architecture-design.md:848
carries the Azure row 'Attempted and not settled'. The '6 h' figure and the DBOS style are unrecorded. | [B10-C02] none) · **Category:** external-grounding/durable-execution · **Confidence:** high ·
**Owner:** Design owner (architecture-design §6.6 amendment)

Merged from 2 concerns; each member statement is carried verbatim. (i) Three cells of the cross-system table that ADR-0017's 'single-use plus a deadline is table stakes' conclusion rests on do not
match their sources [documented]. (1) architecture-design :841 gives Step Functions 'Server-enforced TTL, max activity TTL 6 h'; neither the SendTaskHeartbeat API page nor the quotas page states 6 h -
the developer guide says a waiting task can pause 'until the workflow execution reaches the one year service quota', with HeartbeatSeconds/TimeoutSeconds as opt-in bounds - and the corpus's own
grounding-s-results :232 says 'up to the 1-year cap'; 6 h is plausibly the GitHub Actions per-job limit cross-contaminated [inferred]. (2) design-recommendation :86 lists Azure Durable among systems
that 'mint a single-use token' with a [1P] tag; first direct verification of the self-flagged row shows Azure external events are addressed by instance ID + event name, mint no token, 'wait
indefinitely' unless raced against a durable timer, and are delivered at-least-once. The design :848 row honestly says 'unread'; the recommendation does not. (3) evidence-augmentation :183 files DBOS
under 'exception (AWS/DBOS)'; `DBOS.get_event` returns None on timeout - the Inngest sentinel style - and only read_stream raises. The tree-hash-binding conclusion and D17/ADR-0017 survive every
correction. (ii) The design's cross-system table (architecture-design §6.3.2) and the evidence-augmentation row it derives from state that AWS Step Functions task tokens carry a 'server-enforced TTL,
max activity TTL 6 h'. Neither Step Functions page fetched contains a 6-hour figure: a callback task waits up to the one-year execution quota, and the only tighter deadline is an author-set
HeartbeatSeconds/TimeoutSeconds. The same table and its expiry-style paragraph classify DBOS as 'raises an error' on expiry; DBOS's Python and TypeScript docs both say get_event/recv return None/null
on timeout, which is the sentinel style the design attributes only to Inngest. The conclusions drawn from the table (single-use tokens, every platform yields a terminal event, nothing binds to a tree
hash) are unaffected; the two cells and the 'exception (AWS/DBOS) or sentinel (Inngest)' recommendation are wrong as written. [documented]

**Anchors.**
- `references/2026-09-08-architecture-design.md:841` — "| AWS Step Functions | Task token, single-use | Server-enforced TTL, max activity TTL 6 h | No |"
- `references/2026-09-08-architecture-design.md:848` — "| Azure Durable Functions (external events) | **Attempted and not settled** — token expiry and single-use semantics could not be read this pass; the row is carried empty rather than dropped, and the six conclusions below rest on the other rows"
- `references/2026-09-08-evidence-augmentation.md:84` — "Step Functions task tokens are single-use with a server-enforced TTL (max activity TTL 6h)."
- `references/2026-09-07-design-recommendation.md:86` — "Every durable system surveyed — Temporal, Restate, Step Functions, Azure Durable, GitHub environments, Trigger.dev [1P] — mints a single-use token and most carry a deadline; only GitHub pins to a SHA."
- `references/2026-09-08-architecture-design.md:880` — "Step Functions raises `States.Timeout` and invalidates the token, Trigger.dev resolves to an incomplete status the run branches on, Inngest resolves `waitForEvent()` to `null`, DBOS raises an error"
- `references/2026-09-08-evidence-augmentation.md:183` — "Expiry-with-terminal-path is uniform in guarantee, split in style: Step Functions → `States.Timeout` error ...; Inngest → `waitForEvent()` resolves to `null`; DBOS → error. ... exception (AWS/DBOS) or sentinel the caller branches on (Inngest)"

**Why it matters before Step 5.** (i) No build consequence; the gate record shape (ADR-0017) does not depend on any of the three cells. Record correctness in the design's evidence table. (ii) No build
consequence: the gate-expiry design (AC-260, AC-276, ADR-0017) chose the exception style on its own grounds. Fix belongs in the design's errata before the table is cited as the comparator for SM-079's
expiry mix.

**Remediation.** (i) Dated amendment to architecture-design §6.6: Step Functions row 'no server cap short of the one-year execution quota; HeartbeatSeconds/TimeoutSeconds opt-in; fresh token on retry'
citing the AWS quotas and SendTaskHeartbeat pages; fill the Azure row 'no token; instanceId + eventName; no built-in deadline (race a durable timer); at-least-once' citing learn.microsoft.com
durable-task external-events (ms.date 2026-05-04) and lift the 'unread' caveat; evidence-augmentation :183 'exception (AWS) or sentinel (Inngest/DBOS)'. design-recommendation :86 is a research
document: mark the Azure attribution corrected by dated amendment rather than editing the sentence. No ADR changes. (ii) In the design document's next revision and in evidence-augmentation §3, correct
row 841 to 'optional HeartbeatSeconds/TimeoutSeconds set by the author; hard ceiling is the 1-year execution quota' and move DBOS from the exception column to the sentinel column in line 880 and row
183 ('error (Step Functions) or sentinel (Inngest, DBOS, Trigger.dev)'). Re-tag both cells [documented, 1P, fetched 2026-09-10]. Owner: design author.

**Sources.**
- https://docs.aws.amazon.com/step-functions/latest/dg/service-quotas.html (fetched 2026-09-10) — DG: you can pause Step Functions until the workflow execution reaches the one year service quota ... fails with a States.Timeout error name. No 6 h figure on the quotas or SendTaskHeartbeat pages.
- https://learn.microsoft.com/en-us/azure/durable-task/common/durable-task-external-events (fetched 2026-09-10) — A raised event includes an instanceID, an eventName, and eventData ... The 'wait-for-external-event' API waits indefinitely ... at-least-once delivery guarantee ... race a durable timer against the ex
- https://docs.dbos.dev/python/tutorials/workflow-communication (fetched 2026-09-10) — DBOS.get_event(workflow_id, key, timeout_seconds=60) ... returning None if the wait times out.
- https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html (fetched 2026-09-10) — 'A task that is waiting for a task token will wait until the execution reaches the one year service quota ... you can configure a heartbeat timeout'; on timeout 'a new random token is generated'
- https://docs.dbos.dev/python/tutorials/workflow-communication (fetched 2026-09-10) — get_event(workflow_id, key, timeout_seconds=60) ... 'returning `None` if the wait times out.'; TypeScript: 'returning `null` if the wait times out.'

**Folds.** EXT-DURABLEE-006, EXT-DURABLEE-011, EXT-DURABLEE-001, BP-DUR-09, BP-DUR-12

### PIA-086 — ADR-0085 rules 4 and 6 claim 'no external norm at all'; NASA SE Handbook App C/D and ISO/IEC/IEEE 29148 cover both

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (EA-5-017 (evidence-augmentation:287, 'Nothing first-party or peer-reviewed found ... No, currently') and evidence-augmentation:288 ('a genuine
gap in the external literature') record the questions; the corpus does not record that the answers are wrong) · **Category:** external-grounding/stated-absence · **Confidence:** high · **Owner:**
ADR-0085 successor author; design author

ADR-0085:30 states that two conventions 'have no external norm behind them at all', :112 transcribes the research pass verbatim for rules 4 (an AC names its verifying instrument) and 6 (phrasing a
deliberately undecided or blocked item), and architecture-design:153 and :1963 call both 'original process design'. Both have documented first-party antecedents that were live on 2026-09-08. Rule 6 is
squarely covered: NASA SE Handbook Appendix C prescribes marking an undecided value 'To Be Resolved' (TBR) 'with the rationale along with what should be done to eliminate the TBR, who is responsible
for its elimination, and by when' [documented]; the PROVISIONAL / BLOCKED-AC phrasing extends that convention with an owner gate and a machine-readable state. Rule 4 is covered at requirement grain:
Appendix D's Requirements Verification Matrix records, per uniquely identified requirement, the verification method and the evidence document, and ISO/IEC/IEEE 29148-framed templates carry a
per-requirement verification method [documented]. The residue that survives is narrower than the corpus's sentence: the norms name a method class (test, demonstration, inspection, analysis), not a
specific test ID, so 'no norm requires naming the instrument itself' may stand [inferred]. ADR-0085's discipline ('absence of a contradicting source is not validation') is intact; only the provenance
is wrong.

**Anchors.**
- `references/adr/0085-documentation-and-traceability-conventions.md:30` — "- Two of the conventions below have **no external norm behind them at all**, and the record must say so rather than borrow authority it does not have (evidence-augmentation §5)."
- `references/adr/0085-documentation-and-traceability-conventions.md:112` — "On the adjacent question behind rule 6: "Is there a convention for phrasing an AC for a deliberately undecided or blocked item so it does not read as an omission? — Nothing found. **No** — a genuine gap in the external literature. Any redkiln practice here is original.""
- `references/2026-09-08-evidence-augmentation.md:287` — "| Is there a documented norm that an acceptance criterion should name its own verifying instrument (test ID, lint rule, metric)? | Nothing first-party or peer-reviewed found; only informal practitioner blog content. |"
- `references/2026-09-08-evidence-augmentation.md:288` — "| Is there a convention for phrasing an AC for a deliberately undecided or blocked item so it does not read as an omission? | Nothing found. | **No** — a genuine gap in the external literature. Any redkiln practice here is original. |"
- `references/2026-09-08-architecture-design.md:153` — "there is **no documented norm that an acceptance criterion should name its own verifying instrument**, so the AC-### + `depends_on` / `tracesTo` convention is **ahead of any external norm rather than confirming one**; and there is **no convention for phrasing an AC"

**Why it matters before Step 5.** No build consequence; the conventions Step 5 follows are unchanged. The record claims originality it does not have, which the house rule at ADR-0085:30 itself
forbids.

**Remediation.** ADR-0085 is accepted: supersede the provenance bullets with a short ADR clause that cites NASA SP-2016-6105 Appendix C (TBD/TBR) as the antecedent rule 6 extends and Appendix D plus
ISO/IEC/IEEE 29148 verification-method attributes as the antecedent rule 4 specialises, rewording 'original process design' to 'specialises documented requirements-engineering practice; no
agile-AC-specific norm found'. Dated erratum to architecture-design §1.4 (:153) and §11 (:1963), and to evidence-augmentation §5 rows 287-288.

**Sources.**
- https://www.nasa.gov/reference/appendix-c-how-to-write-a-good-requirement/ (fetched 2026-09-10) — TBD values should be minimized; mark 'To Be Resolved' (TBR) with the rationale, what must be done to eliminate it, who is responsible, and by when
- https://www.nasa.gov/reference/appendix-d-requirements-verification-matrix/ (fetched 2026-09-10) — 'identify an approach for verifying the requirements'; 'The matrix should identify each shall by unique identifier and be definitive as to the source'
- https://ntrs.nasa.gov/api/citations/20170001761/downloads/20170001761.pdf (fetched 2026-09-10) — SP-2016-6105 Rev 2: Table D-1 (p.241) and Table 4.2-2 (p.70) name verification method and evidence per requirement
- https://ops.fhwa.dot.gov/publications/fhwahop18080/ch3.htm (fetched 2026-09-10) — 29148-framed: 'identify one of the following methods of verification for each requirement' (Demonstration, Test, Analysis, Inspection)

**Folds.** EXT-STATEDAB-007, EXT-STATEDAB-017, EXT-STATEDAB-029, EXT-STATEDAB-030, EXT-STATEDAB-054

### PIA-087 — `Quarantined` 'no prior art beyond GitLab and Cloudflare' (ADR-0074/0082) is contradicted by SARIF suppressions, Snyk ignore and GitHub dismissal

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (EA-5-022 (evidence-augmentation:292) records the question and the answer 'a real gap, not a short-search artifact'; ADR-0074:100 and
ADR-0082:29/:90 transcribe it) · **Category:** external-grounding/stated-absence · **Confidence:** high · **Owner:** ADR-0074 / ADR-0082 successor author

Three accepted ADRs and the research pass state that D4's `Quarantined { reason, by, until }` and the Advisory sink have no published prior art beyond two comparators. Four are published and were live
on 2026-09-08. OASIS SARIF 2.1.0 section 3.35 standardises a suppression object whose `status` is 'accepted', 'underReview' or 'rejected' - 'the disposition of a suppressed result within a result
management workflow', i.e. a quarantine state for a disputed finding with two outcomes [documented]. Snyk's `.snyk` ignore carries `--reason` and `--expiry` (default 30 days), field-for-field
`Quarantined`'s {reason, until} [documented]. GitHub code-scanning dismissal records a reason and is reopenable; SonarQube has False Positive / Won't Fix / Accepted resolutions [documented].
ADR-0074:100's 'original process design' and ADR-0082:90's 'no part of it should be presented as adopted practice' overstate; 'convergent with SARIF suppression and Snyk ignore-with-expiry' is the
honest framing. The Advisory sink's routing of a non-blocking-but-true finding may still be original; that is a different sentence from the one under test [inferred]. No decision changes; an interop
option opens.

**Anchors.**
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:100` — "- [documented] Clause 6 has **no external prior art beyond GitLab flaky-quarantine and Cloudflare "won't fix"**; the research pass looked and found none (evidence-augmentation §5). `Quarantined` is original process design and should be stated as such."
- `references/adr/0082-advisory-sink-is-a-first-class-destination.md:29` — "- No richer prior art exists to copy. GitLab flaky-quarantine and Cloudflare "won't fix" are the only comparators found, and the external research pass records this as a real gap rather than a short-search artefact (evidence-augmentation §5)."
- `references/adr/0082-advisory-sink-is-a-first-class-destination.md:90` — "D4's `Quarantined` state has no richer prior art to draw on", recorded as a real gap rather than a search artefact. The Advisory sink is therefore **original design**, and no part of it should be presented as adopted practice."
- `references/2026-09-08-evidence-augmentation.md:292` — "| Are there published quarantine/override patterns for disputed findings beyond GitLab flaky-quarantine and Cloudflare "won't fix"? | Nothing new found; this appears to be a real gap, not a short-search artifact. | **No** — D4's `Quarantined` state has no richer prior art to draw on. |"

**Why it matters before Step 5.** No build consequence for Step 5 (the review gate is a later lane); the record of three accepted ADRs claims a gap that a one-query search closes, and the D4 schema
will be written without the SARIF suppression shape in view.

**Remediation.** Supersede the three provenance bullets with one clause citing SARIF 2.1.0 section 3.35, Snyk ignore, GitHub alert dismissal and SonarQube resolutions as convergent prior art and
rewording 'original' to 'convergent'. Register a small design spike (name it D4-SARIF; consumer: the review-gate schema under ADR-0074's successor) to decide whether `Quarantined` maps onto
`suppressions[].status` / `justification` for tool interop; until it runs the D4 shape stands. Dated erratum to evidence-augmentation §5 row 292.

**Sources.**
- https://docs.oasis-open.org/sarif/sarif/v2.1.0/os/sarif-v2.1.0-os.html (fetched 2026-09-10) — 3.35 suppression object: kind inSource|external; status accepted|underReview|rejected - disposition of a suppressed result within a result management workflow
- https://docs.snyk.io/developer-tools/snyk-cli/commands/ignore.md (fetched 2026-09-10) — --reason: Human-readable <REASON> to ignore this issue ... --expiry: Expiry date in YYYY-MM-DD format. Defaults to 30 days
- https://docs.github.com/en/code-security/code-scanning/managing-code-scanning-alerts/resolving-code-scanning-alerts (fetched 2026-09-10) — The alert is moved to the 'Closed' list ... you can reopen it, if required. The reason why you closed the alert is recorded.
- https://docs.semgrep.dev/semgrep-code/triage-remediation (fetched 2026-09-10) — triage states To fix / Reviewing / Ignored with ignore reasons

**Folds.** EXT-STATEDAB-032, EXT-STATEDAB-041, EXT-STATEDAB-047, EXT-STATEDAB-048

### PIA-091 — The AWS backoff post does not 'nudge toward Equal Jitter for throttling'; it calls Equal Jitter 'the loser' -- a false [1P] attribution at design:345/:2012 and evidence-augmentation:85 (the 'base 2 s / cap 5 min is absent' half holds)

**Priority:** P2 · **Classification:** NEW — [B09a-C09] none | [B09b-C12] none · **Category:** external-grounding/durable-execution · **Confidence:** high · **Owner:** Design owner

Merged from 2 concerns; each member statement is carried verbatim. (i) architecture-design :345 and evidence-augmentation :85 state that the AWS Architecture Blog post 'nudges toward Equal Jitter
(`base/2 + random(0, base/2)`) specifically for throttling errors', tagged [documented, 1P]. The post says 'Of the jittered approaches, "Equal Jitter" is the loser. It does slightly more work than
"Full Jitter", and takes much longer', prefers Full or Decorrelated Jitter, and the string 'throttl' does not occur in its body [documented; raw-HTML keyword count on re-fetch]. The other two
sub-claims - 'base 2 s, cap 5 min' is absent, and the May 2023 currency note is verbatim - hold. The Equal-Jitter-for-throttling rule lives in AWS SDK retry-mode implementations, not this post
[inferred]. AC-029's full-jitter default is unaffected. (ii) architecture-design:345 and the §11 source row at :2012 cite the AWS Architecture Blog post to show that 'base 2 s, cap 5 min' does not
appear in it, which is correct, and add that 'it nudges toward Equal Jitter specifically for throttling errors'. The flattened live page contains no occurrence of 'throttl', and its verdict on Equal
Jitter is 'Of the jittered approaches, Equal Jitter is the loser', favouring Full or Decorrelated Jitter [documented]. The Equal-Jitter-for-throttling advice belongs to AWS SDK retry documentation,
not this post [inferred]. The design's retry table at :338 uses full jitter, which the post favours, so no design change follows.

**Anchors.**
- `references/2026-09-08-architecture-design.md:345` — "the AWS backoff post uses generic base/cap variables — "base 2 s, cap 5 min" does not appear in it, and it nudges toward Equal Jitter specifically for throttling errors [documented, 1P aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter, currency-noted May 2023, fetched 2026-09-08]"
- `references/2026-09-08-evidence-augmentation.md:85` — "**But "base 2s, cap 5 min" does not appear in that post**, which uses generic base/cap variables; and it nudges toward Equal Jitter (`base/2 + random(0, base/2)`) specifically for throttling errors | [1P] aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/"
- `references/2026-09-08-architecture-design.md:2012` — "Generic base/cap variables and a nudge toward Equal Jitter for throttling — cited to show that **"base 2 s, cap 5 min" does not appear in it**"
- `references/2026-09-08-acceptance-criteria.md:457` — "**AC-029** — `rk-kernel::retry` · evidence: proposal §4.3 [documented]; AWS "Exponential backoff and jitter" [documented] · decisions: ADR-0022"
- `references/2026-09-08-architecture-design.md:338` — "`Transport` / `Platform` 5xx / `Timeout`: exponential backoff with full jitter, base 2 s, cap 5 min, ≤ the pack's `N`."

**Why it matters before Step 5.** (i) None; rk-kernel::retry is Step 7b and its declared envelope is unchanged. A false [1P] attribution in the live design. (ii) None; the retry policy Step 5
implements (full jitter, declared base/cap in the pack) is what the source recommends.

**Remediation.** (i) Strike the Equal-Jitter sentence at architecture-design :345 and evidence-augmentation :85 by dated amendment, or re-source it to the AWS SDK retry-mode documentation if the
throttling distinction is wanted; leave AC-029 as is. (ii) Dated erratum to architecture-design:345 and :2012: keep 'generic base/cap variables' and the absence claim, delete the Equal Jitter clause
or re-source it to the SDK retry documentation that actually says it.

**Sources.**
- https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/ (fetched 2026-09-10) — Update (May 2023): After 8 years, this solution continues to serve as a pillar ... Of the jittered approaches, 'Equal Jitter' is the loser. It does slightly more work than 'Full Jitter', and takes muc
- https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/ (fetched 2026-09-10) — 'Of the jittered approaches, Equal Jitter is the loser'; zero occurrences of 'throttl', '2 s', '5 min'; base/cap only as formula variables

**Folds.** EXT-DURABLEE-009, EXT-STATEDAB-009, EXT-STATEDAB-018

### PIA-092 — ADR-0081's 'no source frames activity selection as a research question at all' contradicts SME, arXiv 2603.22386 and ADR-0020

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (EA-5-019 (evidence-augmentation:289) records the question with the 'LLM-driven' qualifier; ADR-0081:190 drops the qualifier and writes 'at all')
· **Category:** external-grounding/stated-absence · **Confidence:** high · **Owner:** ADR-0081 successor author; design author

Evidence-augmentation:289 asked for a subfield covering 'LLM-driven' tailoring; ADR-0081:190 restates it as 'no external source frames activity selection as a research question at all', and
architecture-design:2026 records 'no comparable published system exists to check this framing against' as a positive result. The unqualified sentence is false three ways. Situational Method
Engineering (Henderson-Sellers, Ralyte, Agerfalk and Rossi, Springer 2014) is a book-length field whose central question is which method fragments to select for a situation, using the OPF deontic
matrix [documented]; the corpus's own research/2026-09-07-method-engineering.md §3 documents it and ADR-0020:23 cites the OPF deontic matrix as 'prior art for a deontic ordering', so ADR-0081
contradicts an accepted sibling. For the LLM case, arXiv 2603.22386 (2026-03-23) surveys 'workflow optimization for LLM agents', explicitly covering 'dynamic methods, which select, generate, or revise
the workflow for a particular run', with AFlow, G-Designer, FlowReasoner and Workflow-R1 as instances [documented]. What survives, narrowed, is 'no published system selects SDLC activities per run
from internal quality signals': arXiv 2509.13942 instantiates fixed Waterfall/V/Agile scaffolds and does not select activities, and the verifier's searches for SDLC-specific tailoring returned nothing
[documented]. Restore the qualifier and cite the adjacent field.

**Anchors.**
- `references/adr/0081-rigor-ladder-must-be-monotone-in-internal-quality.md:190` — "that no external source frames activity selection as a research question at all, and names that as a positive gap — no"
- `references/2026-09-08-architecture-design.md:2026` — "The research pass looked for a named research subfield covering **LLM-driven process or workflow tailoring** and found none [documented, research pass §5 unresolved]. That is worth recording as a result rather than an absence: **no comparable published system exists to check this framing against**"
- `references/2026-09-08-evidence-augmentation.md:289` — "Search surfaced multi-agent SDLC coordination and lifecycle-aware code generation, but nothing framing activity selection as its own research question."
- `references/adr/0020-stage-rigor-is-a-deontic-enum.md:23` — "- Prior art for a deontic ordering exists and is cited directly: the OPF deontic matrix and the V-Modell XT mandatory-modules scheme (V-Modell XT's tailoring is documented as two decades in production)."

**Why it matters before Step 5.** No build consequence; the rigor-ladder decision rests on P7's measurement, not on the absence claim. The record asserts a gap the corpus's own research folder and an
accepted ADR contradict.

**Remediation.** Supersede ADR-0081's 'Corpus versus external evidence' bullet with a clause that restores the 'LLM-driven, per-run, from internal quality signals' qualifier, cites SME (Springer 2014)
and arXiv 2603.22386 as the adjacent fields, and cross-references ADR-0020:23. Dated erratum to architecture-design §11 preamble (:2026) and evidence-augmentation §5 row 289, replacing 'no named
subfield' with 'nearest fields: SME; agentic workflow optimisation (2603.22386); no SDLC-tailoring system found'.

**Sources.**
- https://arxiv.org/abs/2603.22386 (fetched 2026-09-10) — From Static Templates to Dynamic Runtime Graphs: A Survey of Workflow Optimization for LLM Agents - 'dynamic methods, which select, generate, or revise the workflow for a particular run'
- https://api.semanticscholar.org/graph/v1/paper/DOI:10.1007/978-3-642-41467-1 (fetched 2026-09-10) — Situational Method Engineering (Springer 2014): constructing methodologies by selecting and assembling method fragments; deontic matrices guide selection
- https://arxiv.org/abs/2509.13942 (fetched 2026-09-10) — instantiates Waterfall/V-Model/Agile as fixed coordination scaffolds for LLM software generation; does not select activities

**Folds.** EXT-STATEDAB-020, EXT-STATEDAB-031, EXT-STATEDAB-046

### PIA-093 — ADR-0089's 'Semgrep 0.57%-23.8% recall' floor is from a mis-cited arXiv ID (RealVuln: recall 17.5%); two more change-classification rows drift

**Priority:** P2 · **Classification:** NEW · **Category:** citation-fidelity/live-ADR-number · **Confidence:** high · **Owner:** ADR-0089 successor author; change-classification author

change-classification:424 cites arXiv 2604.13764 for 'Dataflow comparator on BigVul/Devign: Semgrep 2.43% F1 / 0.57% recall', :64 repeats the figures, and :9 and accepted ADR-0089:75 and :164 carry
the range 'from 0.57% to 23.8% recall' as measured evidence against buying an analysis platform. arXiv 2604.13764 is RealVuln (Pellew and Raza, 2026-04-15), a benchmark over 26 intentionally
vulnerable Python repositories in which Semgrep scores F1 18.9, precision 0.205, recall 0.175; 'BigVul', 'Devign', 'dataflow', '2.43' and '0.57' appear nowhere in its abstract, tables, references or
PDF, and a web search for the exact figures found no source [documented]. The ID is most likely mis-pasted [inferred]. With RealVuln substituted the measured floor becomes 17.5% recall and the range
17.5%-23.8%, which weakens but does not reverse ADR-0089's argument (CodeQL still publishes no classifier precision or recall; the vocabulary problem still relocates). A second Semgrep figure on the
same page drifts: :422's '59.9% IDOR recall' is not in the cited 2025 post but in Semgrep's 27 Aug 2026 Multimodal benchmark post, a different product and study, which the corpus never cites
[documented]. Third, :125's 'Nothing found, positive or negative. The 0/20 result has no literature to compare against' for data-migration classification is answered by the schema/code co-evolution
literature: Qiu, Li and Su (ESEC/FSE 2013) detect schema-changing revisions across 160K revisions in ten applications by the same schema-file detection the corpus's DECLARED migrations directory uses,
then validate by hand, giving a base rate of roughly 1.7% of revisions touching schema (about 0.9% real) [documented]; at that rate 0 positives in 20 items is the expected outcome, which is the number
the design's NOT MEASURABLE row (architecture-design:19) lacks to say how many items a decidable `data_migration` criterion needs [inferred].

**Anchors.**
- `references/2026-09-09-change-classification.md:424` — "- Dataflow comparator on BigVul/Devign: Semgrep 2.43% F1 / 0.57% recall [preprint] · https://arxiv.org/pdf/2604.13764"
- `references/2026-09-09-change-classification.md:64` — "**2.43% F1** on BigVul, **0.57% recall** on Devign vs 87.80% F1 for a dataflow comparator; **23.8% recall / F1 0.385** on a 42-case agent-security benchmark vs 95.24% for a purpose-built tool; **59.9% recall** on IDOR for Semgrep's"
- `references/adr/0089-facet-provenance-declared-surfaces-and-abstention.md:75` — "from 0.57% to 23.8% recall; each relocates the vocabulary problem into a different DSL rather than solving it."
- `references/adr/0089-facet-provenance-declared-surfaces-and-abstention.md:164` — "classifier of anything**; plain Semgrep's measured numbers on adjacent tasks run 0.57%–23.8% recall; a call graph"
- `references/2026-09-09-change-classification.md:422` — "- Semgrep AI-augmented detection (59.9% IDOR recall; 61% precision case study) [vendor] · https://semgrep.dev/blog/2025/ai-powered-detection-with-semgrep/"
- `references/2026-09-09-change-classification.md:125` — "| Anything at all classifies a **data-migration** facet | **Nothing found, positive or negative.** The 0/20 result has no literature to compare against |"

**Why it matters before Step 5.** No build consequence: ADR-0089's DECLARED/DERIVED/PROPOSED/ASKED decision does not turn on the floor of the range. An accepted ADR carries a number no source
contains.

**Remediation.** Dated erratum to change-classification:125 replacing 'nothing found' with 'nearest literature: schema co-evolution base rates (Qiu et al. 2013)' and carrying the ~1.7% base rate into
the design's data_migration NOT MEASURABLE row as the n-sizing input. Supersede ADR-0089's two sentences with a clause giving the corrected range (17.5%-23.8% recall; RealVuln arXiv 2604.13764 and
arXiv 2603.22853) and stating that the 0.57%/2.43% BigVul/Devign figures are withdrawn as unsourced; dated erratum to change-classification:9, :64 and :424, and add the Semgrep Multimodal post URL at
:422 with the product named. If the original BigVul/Devign source is later found, re-cite it by ID and restore the figures under a dated note.

**Sources.**
- https://arxiv.org/html/2604.13764v1 (fetched 2026-09-10) — RealVuln (Pellew, Raza, 2026-04-15): 26 Python repos; Table 4 Semgrep F1 18.9, precision 0.205, recall 0.175; no BigVul, Devign, 2.43 or 0.57
- https://semgrep.dev/blog/2025/ai-powered-detection-with-semgrep/ (fetched 2026-09-10) — 61% precision customer case study present; '59.9' absent
- https://semgrep.dev/blog/2026/idor-detection-benchmark-semgrep-multimodal/ (fetched 2026-09-10) — 27 Aug 2026: Semgrep Multimodal achieved 59.9% recall (precision 57.5%) vs 13.9% for Claude Security with Mythos
- https://web.cs.ucdavis.edu/~su/publications/fse13-db-study.pdf (fetched 2026-09-10) — Qiu, Li, Su ESEC/FSE 2013: schema/code co-evolution over 160K revisions in ten applications; schema-changing revisions ~1.7% (about 0.9% after manual validation)

**Folds.** EXT-PREPRINT-032, EXT-PROCESSL-010, EXT-STATEDAB-039

### PIA-095 — Review-gate ADRs mis-state F24/F31: 0082/0088 credit F24 with '≥13 of 23' (F24 said ≥5/23; the 13 is F31's); 'four survived fully green' is two

**Priority:** P2 · **Classification:** NEW — none — ARCH-1.2-030 and anomaly_pool clean.13of23 concern the value of the 13/23 figure, not its attribution to F24; F31-LIM-007 (spikes/F31-DRAFT.md:243)
records the 54/62-failure fixtures inside F31 itself; ARCH-0.1-025 records the design's restatement as a deliberate_exception without noticing the overstatement. · **Category:** citation-drift ·
**Confidence:** high · **Owner:** Ryan (ADR steward); Claude (design/register restatements)

[measured] F24 (FINDINGS.md :1350) established that CLEAN's ground truth is wrong for 'at least five of 23 items (22%)'; the raise to ≥13/23 first appears in F31-DRAFT :91, and design :122 attributes
it correctly. ADR-0088 :21/:59/:94 (the last tagged [measured]) and ADR-0082 :93 credit F24 with '≥13 of 23 (57%)' — a finding cited for a number it does not contain, in two accepted records that list
F31-DRAFT among their sources. In the same records, F31-DRAFT :75-76 reports four escapes from the deterministic gate of which exactly two (issue-70 AC-001 at 1,890/1,890; issue-83 AC-002 at
1,907/1,907) survived a fully green suite; issue-131's two survived 'with zero new failures against a 54-failure baseline', a suite already red for other reasons. Every restatement outside F31 — 22
lines across design :43/:814, AC-315/AC-335, SM-100, the brief, RUNBOOK 7c :2075, ADR-0074, ADR-0082 and ADR-0088 — says 'four ... survived fully green suites'; none says two, none carries the
54-failure baseline. AC-335's mechanism ('a fully green suite is not evidence') holds on two cases as well as four, so nothing is built wrong; it is the slice's most-restated evidence sentence, wrong
by a factor of two, in three immutable ADRs.

**Anchors.**
- `references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:94` — "- [measured] F24: CLEAN's ground truth is wrong for **at least 13 of 23 items (57%)** — every item shipped and was approved, and the defects"
- `references/adr/0082-advisory-sink-is-a-first-class-destination.md:93` — "F24 established CLEAN's ground truth is wrong for at least 13 of 23 items (57%), so every rate measured against it is provisional."
- `spikes/FINDINGS.md:1350` — "to be wrong for at least five of 23 items (22%).** Any false-positive rate"
- `spikes/F31-DRAFT.md:75` — "**fully green** suite (1890/1890 and 1907/1907); `issue-131`'s two survive with"
- `references/2026-09-08-architecture-design.md:814` — "**Four adjudicated defects survived fully green suites of 1,890/1,890 and 1,907/1,907** [measured, F31]"
- `RUNBOOK.md:2075` — "defects survived fully green suites of 1,890/1,890 and 1,907/1,907); blocking is computed in Rust from re-executed"

**Why it matters before Step 5.** None for the build; ADR evidence sections are the immutable record, and RUNBOOK 7c carries the overstated sentence into the checkpoint the builder reads.

**Remediation.** One erratum in references/adr/README.md covering ADR-0074/0082/0088: 'F24' at 0082:93 and 0088:21/59/94 reads 'F24 (≥5/23) raised to ≥13/23 by F31'; 'four adjudicated defects survived
fully green suites' reads 'four adjudicated defects escaped the deterministic gate, two through fully green suites (1,890/1,890 and 1,907/1,907) and two with zero new failures against an already-red
54-failure baseline'. Reword the design, AC, SM, brief and RUNBOOK 7c restatements to the same sentence. Never edit the records; if a superseding record is ever cut, carry the correct attribution.

**Folds.** G09-05, G16-01

### PIA-098 — The rank-reversal sentence behind 'rigor is ordinal, never summed' is pinned to a TOPSIS-only paper; it originates in arXiv 2312.03006

**Priority:** P2 · **Classification:** NEW · **Category:** external-grounding/peer-and-standards · **Confidence:** medium · **Owner:** Design owner

Five live places (architecture-design :52, :180, :569, :595, :2039; evidence-augmentation :205) justify the ordinal deontic rigor enum with 'rank reversal occurs for virtually every weighted MCDM
method, TOPSIS and SAW included [peer, ScienceDirect S0895717711007850]'. Crossref binds that PII to Garcia-Cascales & Lamata, 'On rank reversal and TOPSIS method', Math. Comput. Modelling 56 (2012) -
a TOPSIS-specific study. The quoted sentence appears verbatim in arXiv 2312.03006, where it is cited to Aires & Ferreira 2018 (Pesquisa Operacional 38) and Wang & Luo 2009 (S0895717708002860, covering
SAW and TOPSIS) [documented]. ScienceDirect returns 403, so the PII's full text is unread and containing the sentence cannot be excluded outright; on title, scope and reference list it does not
[inferred]. The general result stands and ADR-0020/0023's decision does not move; the pin is to the wrong paper.

**Anchors.**
- `references/2026-09-08-architecture-design.md:52` — "Rigor is `forbidden < discouraged < optional < recommended < mandatory` — ordinal, never summed, because **rank reversal occurs for virtually every weighted MCDM method**, TOPSIS and SAW included [peer, ScienceDirect S0895717711007850]."
- `references/2026-09-08-architecture-design.md:2039` — "| **ScienceDirect S0895717711007850** [peer] | **Rank reversal occurs for virtually every weighted MCDM method**, TOPSIS and SAW included — the general result, so "just use TOPSIS" is pre-answered. Why rigor is an ordinal deontic enum and is never summed | §0.1, §2 (principle 14), §4.7, §4.8 |"
- `references/2026-09-08-evidence-augmentation.md:205` — ""rank reversal of one or another type occurs for virtually every MCDM method", including TOPSIS and Simple Additive Weighting ... One comparative study found TOPSIS had the *least* rank reversal among AHP/TOPSIS/ELECTRE — the best-behaved weighted method still exhibits it | ... | [peer] ScienceDirect S0895717711007850 |"

**Why it matters before Step 5.** None; AC-050's trybuild compile-fail test does not depend on the citation. Record correctness of a load-bearing [peer] pin.

**Remediation.** Re-pin by dated amendment at all six sites to Wang & Luo 2009 (S0895717708002860) or Aires & Ferreira 2018 as the primary, with arXiv 2312.03006 as the quoting source and
Garcia-Cascales & Lamata 2012 retained only for the TOPSIS-specific comparison; mark the PII's own text unread.

**Sources.**
- https://arxiv.org/html/2312.03006 (fetched 2026-09-10) — 'Rank reversal of one or another type occur for virtually every MCDM method, and its implications are still highly debated.'[1][21] - [1] Aires & Ferreira 2018; [21] Wang & Luo 2009; 'compare [7] for 
- https://api.crossref.org/works?filter=alternative-id:S0895717711007850&rows=3 (fetched 2026-09-10) — PII resolves to Garcia-Cascales & Lamata, 'On rank reversal and TOPSIS method', Mathematical and Computer Modelling 56 (2012) 123-132.

**Folds.** EXT-PEERANDS-024

### PIA-099 — Principle 8 was changed on a vendor compaction quote ('nuanced understanding ...') that is on no live first-party page

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (EA-1-006 (evidence-augmentation:34) records the contradiction row and architecture-design:204 records the clause withdrawal; neither records that
the quotation is unsupported) · **Category:** citation-fidelity/design-principle · **Confidence:** high · **Owner:** design author

Evidence-augmentation:34 quotes the vendor's compaction guidance as saying native summarisation can discard 'nuanced understanding that took an entire session to build', cited to code.claude.com
context-window and prompt-caching pages; architecture-design:174 marks principle 8 'changed' on it, :204 records the 'adds nothing' clause withdrawn because 'the vendor's own compaction guidance
argues the opposite', and :2007 lists the page as a source. The phrase appears on none of the 191 pages listed in code.claude.com/docs/llms.txt, nor on the platform compaction or prompt-caching pages,
nor in two Anthropic posts, and an exact-phrase web search returns nothing; the live best-practices text says compaction 'preserves important code and decisions while freeing space' [documented]. The
absence half of the row ('no first-party or peer-reviewed source supports the strong form') holds, and the principle-8 change stands independently on F28's H4 result being scoped to file selection
(:204). The quotation must be re-sourced to a page and date or lose its quotation marks.

**Anchors.**
- `references/2026-09-08-evidence-augmentation.md:34` — "native summarisation can discard "nuanced understanding that took an entire session to build", which is why decisions belong in a durable file."
- `references/2026-09-08-architecture-design.md:174` — "and (**changed**) asserting that a structured digest loses nothing — a judgment call that matters downstream is carried as an explicit typed field or it is not carried."
- `references/2026-09-08-architecture-design.md:204` — ""The reviewer's prose adds nothing" is unsupported and the vendor's own compaction guidance argues the opposite (research pass §1 row 6). F28's H4 result is scoped to file selection."
- `references/2026-09-08-architecture-design.md:2007` — "| **code.claude.com** — context window and compaction guidance | Declines the strong form of "prose adds nothing": the vendor's own guidance argues the opposite for judgment-bearing content | §1.1, §2 (principle 8) |"

**Why it matters before Step 5.** No build consequence; principle 8's typed `notes`/`caveats` field is right on the measured ground. A design principle cites a quotation nobody can find.

**Remediation.** Dated erratum to evidence-augmentation §1 row 6 and architecture-design:204 and :2007: drop the quotation marks and cite code.claude.com/docs/en/how-claude-code-works (compaction
'preserves important code and decisions') as the nearest live first-party text, or, if the sentence is found in an archived page, cite the Wayback URL and capture date. Principle 8 is unchanged.

**Sources.**
- https://code.claude.com/docs/llms.txt (fetched 2026-09-10) — 191 listed pages fetched in .md form: zero hits for 'nuanced understanding'
- https://code.claude.com/docs/en/best-practices (fetched 2026-09-10) — compaction 'preserves important code and decisions while freeing space'
- https://platform.claude.com/docs/en/build-with-claude/compaction (fetched 2026-09-10) — platform compaction page: phrase absent

**Folds.** EXT-STATEDAB-023

### 4.4 Statistics (8)

### PIA-073 — Statistics tagged [measured] or settled do not reproduce under the registered two-sided Wilson interval: MCC +0.978 is +0.957 (ten sites); '~60 positives' is 73; F27's '~30%' is 56-66%; F37's 3/3 vs 0/3 overlap, so section 4.4(1) had zero power at n=3 and S-P7B's n=10 is powerless too; McNemar p=0.625 is bound to the detection tie at nine sites; F19's 3.6x is mis-inferred from CI overlap; SM-086's 'n >= 500' is 637

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B06-C07] F39-LIM-001/F39-LIMP-001 (F39:92), RB-GC-010 (RUNBOOK:119), RB-HALT-009, EB-E-003 carry '~60' as fact; F37-LIM-001/F37-LIMP-001
(F37:131), ARCH-1.2-011, ARCH-0.1-006 carry 'close to unfalsifiable'/'overlaps by construction'; MCC (ARCH-0.1-013, ARCH-1.2-031, ARCH-6.6-012, ARCH-10.4-004 quote +0.978 as settled), F27 ~30% and the
F19 inference: no row questions them -- those three are NEW | [B07-C04] F39-LIM-001 (spikes/F39-DRAFT.md:92), RB-GC-010 (RUNBOOK.md:119), RB-HALT-009 (RUNBOOK.md:2394), EB-E-003 carry 'roughly 60' as
the recorded limit; ARCH-1.2-011 and RB-GC-010 record 'powerless at n=3, every interval overlaps by construction'. None records that 60/60 fails the registered bound, that F37/ADR-0080/HANDOFF carry
the contradictory 3/3-vs-0/3 arithmetic, that S-P7B's n=10 is powerless for the effect it names, or SM-086's target n (the SM-086 leg is NEW: SM-5.2-007 and AC-1.2-011 concern its baseline only). |
[B07-C05] ARCH-0.1-015, ARCH-1.2-001, SM-3.1-001 and EB-B-002 record the saturation and repeat the mis-bound phrase 'detection 23/23 vs 23/23, McNemar p=0.625'; HST-UNRES-010 (spikes/HANDOFF.md:152)
binds it correctly to the co-primary FPR. No row records the misattribution. The MCC leg is NEW: ARCH-0.1-013, ARCH-1.2-031, ARCH-6.6-012, ARCH-10.4-004, ARCH-10.5-014 quote '+0.978' as settled and
constants.json f24.1of23 is AGREE across five files.) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Register/design owner (Claude) for restatements; F27/F37/F39 authors for the spike
amendments; Ryan for ADR notes

Merged from 3 concerns; each member statement is carried verbatim. (i) Five statistical figures tagged [measured] do not reproduce under the corpus's own conventions (Wilson 95%, z=1.96, METHOD-P:17).
(a) MCC +0.978 for E3 arm (a) at FP 1/23: the matrix tp=23 fn=0 fp=1 tn=22 gives +0.957; the same formula reproduces the corpus's +0.839 (fp=4) and +0.917 (fp=2), an exhaustive search finds no 23-item
matrix at 0.978, and the figure stands in ten places (annotation sheet :131; design :31, :123, :179, :956, :1049, :1819, :1873; RUNBOOK :881; rfc-amendment :223) with no matrix beside it. (b) 'Roughly
60 positives are needed for a lower bound above 0.95' (F39:92; RUNBOOK :123/:969/:2396 tagged [measured]; ~15 restatements): the first n with Wilson(k=n) lower > 0.95 is 73 (n=60 gives 0.940; 92 with
continuity correction; Clopper-Pearson 72). (c) F27's 'stable at n=2-3 tolerates a true flip rate up to ~30%' (FINDINGS:1686; design :79, brief :42, SM :18, ADR-0074 :96, ADR-0088 :91): Wilson 95%
upper on 0/3 is 0.56 and on 0/2 is 0.66; a 30% rate leaves a 34% chance of no flip in 3 draws, so ~30% corresponds to no conventional level and its derivation is undisclosed. (d) F37:39 says 3/3
[0.44, 1.00] against 0/3 [0.00, 0.56] is the only non-overlapping pair -- they share [0.44, 0.56], so section 4.4(1) is strictly unfalsifiable at n=3, the stronger claim F37:1 and design :24/:103
already make; section 2 and Limits 1 contradict them. (e) F19's 3.6x (9/79 vs 3/95): the ratio is right (3.61) but 'overlapping intervals, not separated' is the CI-overlap fallacy -- log-RR 95% CI
[1.01, 12.9], Fisher p=0.039 -- a conservative-direction mis-inference used as SM section 0 rule 3's paradigm of small-n discipline; SM :923/:925/:1182 also print the upper bound as 20.2% where 9/79
gives 20.3%. (a), (c), (e) are NEW; (b), (d) sharpen recorded limits that carry the wrong number as fact. (ii) [measured] Recomputed with the programme's registered interval — two-sided Wilson at z =
1.96, the `wilson()` in spikes/p1/score-arm-d-panel.py that reproduces F39's 0.65 for 7/7 and 0.26 for 0/11 — three sizing statements the corpus treats as settled are wrong. (i) 'Roughly 60 positives
are needed for a lower bound above 0.95' (F39 Limits 1; restated in the design, both registers, RUNBOOK :123/:969 tagged [measured], the brief, ADR-0089, the change-classification and two handoffs):
for k = n the lower bound is n/(n + 3.8416), so 60/60 gives 0.940 and the scorer's own verdict line (:120) would print STAYS; the bound first clears 0.95 at 73 all-correct positives (72 under
Clopper-Pearson); 'roughly 60' holds only for a one-sided z = 1.645 that nothing registers. (ii) F37 :39 and ADR-0080 :52 say the only n = 3 pair that fails to overlap is 3/3 against 0/3, 'the maximum
possible separation'; 3/3 = [0.439, 1.000] and 0/3 = [0.000, 0.561] overlap on [0.439, 0.561], so no n = 3 pair can separate under METHOD-P §4.4(1) — the criterion had zero power, not 'almost none'
(the §0.1/RUNBOOK phrase 'overlaps by construction' is the correct one; HANDOFF:223 repeats the error). ADR-0080 :104's claim that at n = 10 per cell a 1-in-10 effect is resolvable is false under the
same criterion: 0/10 = [0, 0.278] overlaps 1/10 = [0.018, 0.404] and 3/10 = [0.108, 0.603]; 6/10 is the first cell to separate, so the registered S-P7B design (design :1870, 'n = 10 per cell') is
powerless for the effect it names and would spend a multiple of $131.62 to reproduce an unfalsifiable comparison. (iii) SM-086's target reads 'once n allows an interval that excludes it (n ≥500
blocks)': 0/500 has upper 0.762%, which does not exclude 0.6%; the smallest n at which 0/n excludes it is 637 (0/636 → 0.6004%), and any observed false block pushes it higher (1/700 → 0.805%); the
register's rule 3 (every quoted interval computed here) was not applied to the n. No Phase-1 code restates any of the three, but S-FACET's corpus, S-P7B's re-run and SM-086's calibration claim would
each be sized from a number that is powerless at its own n — the defect class RB-GC-010 already counts four instances of. (iii) [measured] F23 (:1172-1179) states that detection was 'identical and at
ceiling ... zero discordant pairs' and that McNemar's test was run on the CLEAN verdict discordant pairs, 3 vs 1 — exact two-sided p = 2·5/16 = 0.625, reproduced from e3.jsonl; on detection 23/23 vs
23/23 the test has no value. The measurement doctrine that opens the SM register (§0 :12), design :33/:71/:960/:1554, AC :3937/:6408, ADR-0001 :56, ADR-0080 :192, ADR-0088 :42 and the brief :74 all
write 'detection 23/23 vs 23/23 at McNemar p=0.625', and AC-283's repair note (:3599) asserts that 'that figure is a detection comparison between E3 arms'; only HANDOFF:152 binds it to the co-primary
FPR. Separately, the DECIDED §0.1 row 'False-positive rates and MCCs from E3' settles arm (a) at FP 1/23 / MCC +0.978; on the corpus's own construction (46-item matrix, spikes/results/audit-e3.py
:122, which reproduces F21's +0.917 and F23's +0.839) tp=23 fn=0 fp=1 tn=22 gives +0.957, and +0.978 reproduces only as sqrt(0.957) or from a doubled matrix; no corpus document contains 0.957. The
+0.978 is restated at design :31/:123/:179/:956/:1049/:1819/:1873, RUNBOOK :881 and rfc-amendment :223 — census agreement on the wrong number. Neither error changes the gate's design (design :1049),
but the register's construction rule is introduced with a statistic attached to the wrong comparison, one repair note codifies it, and a DECIDED row cites a miscomputed calibration figure.

**Anchors.**
- `spikes/results/e3-severity-annotation-ryan.md:131` — "| MCC | +0.978 | **+1.000** |"
- `spikes/F39-DRAFT.md:92` — "`ui` is 7/7 and its lower bound is 0.65; roughly 60 positives are needed for a lower bound above 0.95."
- `spikes/FINDINGS.md:1178` — "so the test is McNemar's on the discordant pairs: 3 items where (a) rejected and"
- `references/2026-09-08-architecture-design.md:31` — "It settles to arm (a) FP **1/23**, MCC **+0.978**; arm (b) FP **0/23**, MCC **+1.000**."
- `RUNBOOK.md:123` — "roughly 60 positives are needed for a lower bound above 0.95 [measured, F39 Limits 1]"
- `references/2026-09-08-success-measures.md:12` — "**Five metrics saturated** — E3 detection 23/23 vs 23/23 at McNemar p=0.625, which the brief records as "primary metric saturated — no discriminating power" (F23)"

**Why it matters before Step 5.** (i) No build step is gated on any of the five and the conclusions stand (E3 MCC feeds no AC; n=20 is powerless either way; P7 section 4.4(1) stays BLOCKED; eager
reconcile() rests on the 7% overall rate). The exposure is sizing and doctrine: an S-FACET or P1 re-run planned to '~60 positives' on the strength of the sentence could not clear METHOD-P section 1.4
even with zero errors (RUNBOOK:1499 currently refuses to write a number, so nothing is mis-sized today), and ADR-0074/ADR-0088 lean on a stability label that constrains ~2x less than stated. (ii)
AC-520 restates no threshold and S-P7B gates nothing, so Step 5 builds nothing from these; but every future sizing decision (S-FACET's corpus, the P1 re-run, S-P7B's cell count, SM-086's first
reading) is planned from a number that cannot meet its own bar, and each is cheaper to correct before a spike budget is committed to it. (iii) No build consequence — saturation of detection is real
and correctly reported and the gate does not depend on the MCC — but a reader auditing the doctrine's own worked example (SM §0) finds it wrong on first inspection, and the E3 row is the DECIDED
calibration evidence the review gate cites.

**Remediation.** (i) Dated corrections, never silent edits: +0.978 -> +0.957 with the matrix (tp 23 / fn 0 / fp 1 / tn 22) stated beside it at every site, as F21 does for +0.917; F39 Limits 1 and its
restatements -> '73 all-correct positives (Wilson, z=1.96; 92 with continuity correction)', keeping [measured] only on the computed number; F27 Limits 1 and restatements -> 'compatible with a true
flip rate up to ~56% (n=3) / ~66% (n=2), Wilson 95%' (ADR-0074/ADR-0088 by correction note or supersession); F37 section 2 and Limits 1 -> 'at n=3 no two Wilson intervals can be disjoint; the
criterion cannot fail to fire at this n'; SM :18/:923 and design :712 -> report RR 3.6 with CI [1.0, 12.9] and 'marginal at n=174, one operator', and 20.2% -> 20.3% at SM :923/:925/:1182. (ii) F39:
append a dated correction to Limits 1 ('at least 73 all-correct positives at the registered two-sided z = 1.96; 72 under Clopper-Pearson'), propagate to every restatement, retag RUNBOOK :123/:969
[inferred], and carry the minimum n into METHOD-P's next dated amendment. F37/ADR-0080: dated F37 erratum; supersede ADR-0080 decision 5 (never edit) to state that n = 3 had zero power and that the
re-run must change the criterion, not only n — a pre-registered paired per-item comparison or an interval on the pooled difference of proportions with a stated minimum detectable effect and the n that
reaches it (0 vs 0.33 needs roughly n = 20 per cell under interval overlap); carry the corrected sizing into S-P7B's §10.5 row. SM-086: amend the target to 'n ≥ 640 adjudicated blocks at zero false
blocks (Wilson upper < 0.6%, computed here); more if any false block is observed', or state it as a bound rather than an exclusion. (iii) Amend SM §0 :12 and every restatement to 'detection 23/23 vs
23/23 (no discordant pairs, no test); CLEAN FP 4/23 vs 2/23, McNemar exact p = 0.625 on the 3-vs-1 discordant pairs'; correct AC-283's repair note. Append a dated correction to
spikes/results/e3-severity-annotation-ryan.md ('arm (a) MCC +0.957, tp=23 fn=0 fp=1 tn=22; +0.978 was a computation error'; Ryan to countersign — it is a decision record, not silently edited) and
update the nine restatements, keeping the qualitative conclusion (one-item gap, McNemar cannot separate). For ADR-0001/0080/0088 record the correction in the ADR README errata or a superseding note;
never edit an ADR.

**Folds.** L11-statistics-01, L11-statistics-03, L11-statistics-04, L11-statistics-05, L11-statistics-06, L07-testability-05, slice-G01-06, slice-G07-06, G15-09, L11-statistics-02, G15-06, G09-03

### PIA-077 — Spike-draft headline figures do not reproduce from their result files: F36 $9.70, F14 ~1.9M tokens, F39 140 cells, F29 n=56, F35 7/102

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ADR-0079:211 records the F36 residual as 'not reconciled in the source'; DEC-SEC-004 repeats $9.70; EB-E-006, ARCH-10.4-011, ARCH-10.5-012 and
anomaly_pool p7.cost_ratio record the 3.66x/4.4x dispute; F14's 1.9M (ARCH-0.1-032, SM-3.2-002, HST-UNRES-001), F39's join (RB-HALT-011 counts three defects), F29's 56/60 (ARCH-1.2-034, SM-3.2-001)
and F35's headline: no row questions them -- those four are NEW) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Ryan (F29/F39 author; scorer fix); Claude (F14/F35/F36 corrections;
register propagation)

The spike drafts are declared authoritative over the brief, and their headline figures were not reproduced from their records before promotion. [measured] F36:3/:57 and DECISIONS:227 give the arm-L v2
spend as $9.70; `p1-arm-l-v2.jsonl` sums to $10.71 (haiku 4.91 + sonnet 5.79; the draft lifted the sonnet driver's last 'spend in this file: $9.70' line while the haiku log ends '$10.71'), which is
what makes the design's $21.37 = $10.66 + $10.71 exact -- ADR-0079:211 records the residual as unreconciled and this resolves it against the draft. F14's '~1.9M tokens' (FINDINGS:497; 16 restatements
in 9 files, census-marked AGREE): the 40 E0 records sum to 1,012,638 tokens and F14's own table rows to ~1.01M; 1.9M is plausibly total_input plus cache_read counted twice [inferred]; the '0.100 ->
0.100' is the warm batch only (cold: 0.110 -> 0.110). F39 scores 136 of its '140 truth cells': the panel file keys one item `.../resolve-open-questions` where the engine output, oracle and worksheet
have `.../resolve-open-question`, score-arm-d-panel.py:91 silently drops the four cells, every semantic row of F39 section 1 sums to 19 items, and the corrected union figure is `auth` 0/12 (Wilson
[0.00, 0.24], fp 12, tn 4), not 0/11 -- a fourth shape-assumption defect beyond the three F39 Limits 5 counts, undetected because the scorer prints 'truth cells: 140' from the label side. F29:169
labels its pooled row n=56 but 0.0098 is 0.59/60; the three full windows it names pool to 0.0102 (7.5x, not 7.8x); downstream 'over 60 draws' is consistent with the value, so the defect is confined to
the table. F35:1's headline '7 of 102 after decisions 1-10' is the run8 reading while its verdict (:51) and every live citation give run9's 6/102 and attribute it to 'decisions 1-10' alone. Conversely
the P7 3.66x/4.4x dispute (R-P7RATIO, 'Not started') reconciles from p7.jsonl at $0: $9.86 over the arm mean $2.69 vs over the same item's first draw $2.22 [inferred denominator; decision 33 names
none].

**Anchors.**
- `spikes/F36-DRAFT.md:57` — "200 dispatches, **0 void**, $9.70,"
- `references/adr/0079-context-vector-source-undecided-phase-1-blocked-on-p1.md:211` — "$21.37 (F36 reports $10.66 + $9.70; the ledger's arm-L total is quoted as $21.37 and the small residual is not"
- `spikes/FINDINGS.md:497` — "## F14 — Headroom: 40 dispatches and ~1.9M tokens stayed below the instrument"
- `spikes/F39-DRAFT.md:25` — "| **`auth`** | **0.00** | **[0.00, 0.26]** | n/a | **0** | **11** | 0 | 4 | 4 | stays |"
- `spikes/F29-DRAFT.md:169` — "| **pooled** | **56** | — | **0.0098** |"
- `spikes/F35-DRAFT.md:1` — "15 of 101 activities are dead, 14 of them by the rank rule, not by absence — and 0.3% / 7 of 102 after decisions 1–10 (§1.6)"

**Why it matters before Step 5.** No Phase-1 sizing uses these numbers (F29 supersedes F14 for sizing; $1.01 on a $40 cap; 4% on a batch-only rule), so build impact is nil. But Checkpoint 4 asserts
'F39's figures reproduce', the F39 labels are registered for reuse by S-FACET arms 2-4, and F14 is the corpus's headline 'upper bound for sizing' -- ~2x too generous in tokens.

**Remediation.** One erratum pass over the drafts (dated correction sections, never silent edits): F36 :3/:57 and DECISIONS :227 -> $10.71 citing the two log lines, with ADR-0079:211's parenthetical
marked reconciled by note; F14 -> '~1.0M tokens (854k cache-read, 157k cache-creation)' quoting both batches' seven_day readings, propagated to section 0.1 row 50, :114, section 7.6, AC-372, SM-111,
SM section 3; fix the panel-file key, make score-arm-d-panel.py fail closed on a truth cell with no engine record, re-run and append the corrected F39 table (auth 0/12 [0.00, 0.24]; tn counts) with a
dated footnote on the 'eleven positives, eleven false' restatements; F29's table -> four rows pooled 60 / 0.0098 or three rows pooled 56 / 0.0102, stating which the downstream figure uses; F35's title
-> '6 of 102 after decisions 1-10 and Ryan's R3/R7 (section 1.6 run9)'; mark R-P7RATIO done and amend section 0.1 row 24, SM-067, SM-110 and (by supersession note) ADR-0080's forbids line with the
reconciliation, keeping the mandatory figure n=1 (ADR-0081).

**Folds.** L02-02, L10-register-integrity-04, slice-G17-04, slice-G01-07, slice-G17-07, L02-06, slice-G07-11

### PIA-078 — Corpus-pin figures restated as [measured] do not reproduce: 45/46 ACs cap-truncated, 'census overall 98%' unreported, `redkiln-migrate` never ran

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ARCH-10.x-T-002 (design:1931) and ARCH-10.x-T-003 record the harmonisation and the section 5.2 conflation repair; SM-5.2-008 (SM:1388) records
that redkiln-migrate does not exist; SM-NOBASE-083/096 record SM-140's baseline as documented-not-measured; ARCH-0.1-037 and ADR-0002-FORB-51 restate 'rewritten in the spike'; none records that the
numbers are wrong or irreproducible) · **Category:** inaccuracy · **Confidence:** high · **Owner:** Ryan (S-MIG instrument owner; ADR-0075 steward); Claude (design/AC/SM restatements)

Figures and provenance derived from the old corpus at pin 47faec9f are restated as [measured] but do not reproduce from s-mig.jsonl or the pin. [measured] (1) The '45 ACs without GWT / 46 without a
verification note' sub-counts are floors: census-s-mig.py:170 caps loss lines at `loss[:6]` and the report sums the capped lists (12 of 19 specs hit the cap); the committed script with the cap
removed, run against the same pin, gives 76/87 loss lines (75/85 distinct ids; 131 ACs with a loss, 29 lacking both, of 208 in the 19 specs), and the harmonisation pass propagated the truncated
numbers to eight sites (design 54, 174, 642, 1517, 1931; AC 1932, 5640; s-mig-report 52). 19/153 and 88% are unaffected. (2) 'The census overall of 98%' (design 651, 1496, 1616; SM 975, 1266) is a
figure S-MIG does not report and section 8.7, cited as its source, has no overall row; the typed-slot overall under the script's own convention is 3011/3138 = 96.0% (98.3% only by counting
LiftedWithLoss as lifted, a convention used nowhere else); ADR-0075:19/:97 keeps the spec-section conflation the design corrected, and :19 attributes the census to `redkiln-migrate`, a tool that does
not exist (instrument: census-s-mig.py, read-only). (3) AC-443:5517, SM-140:1027 and ADR-0002:57 describe `reconcile.rs` as the one file imported from the old redkiln; the pin holds zero .rs files --
it is the trailer-reconcile logic of `src/store/verify.ts` (named only at FINDINGS.md:443), re-implemented in this repo's spike at 330d46c on 2026-09-02; AC-443 discloses the entry has no hash
counterpart, but no planning document names the TS origin and no RUNBOOK line says rk-git lifts it from redkiln-corpus after Step 2 deletes spikes/runner. (4) AC-445:5539's GIVEN and SM-140's baseline
count '101 old CLI-behaviour test files'; the pin has 237 `*.test.ts` under a flat test/, the RFC's own R10 partition 101+61+80 = 242 contradicts its Method line's 237, and no classification rule or
list exists. (5) Total Failed over s-mig.jsonl is 54, not the 53 at AC:5635/design:1531 (the 54th is an untabled report/gates failure); fix-lane 42 + archived 110 = 152 against 153 specs, with
.bklg/support/issue-146-147-story-gate/issue-146-147-boundary-window/spec.md assigned to no generation.

**Anchors.**
- `spikes/results/census-s-mig.py:170` — "return "LiftedWithLoss", {"n": len(entries), "what": loss[:6]}"
- `references/2026-09-08-architecture-design.md:54` — "**19 of 153** shipped specs (12%) write criteria with **no GWT (45 ACs) or no verification note (46 ACs)**"
- `references/2026-09-08-architecture-design.md:651` — "against the census **overall** of 98%. ("Spec sections" is its own S-MIG slot, also at 98%"
- `references/adr/0075-typed-artifacts-revision-4-refinements.md:19` — "S-MIG has since run the census those records lacked: `redkiln-migrate` over the frozen corpus, **1,312 files / 6,877 sections, zero model cost**."
- `references/2026-09-08-acceptance-criteria.md:5517` — "THEN the manifest contains exactly one entry, `reconcile.rs`"
- `references/2026-09-08-acceptance-criteria.md:5539` — "GIVEN the 101 old CLI-behaviour test files at the corpus pin"

**Why it matters before Step 5.** No build impact: the permissive-AC and 80% reclassify decisions rest on 19/153 and per-slot shares, both correct; AC-443 already discloses its entry cannot resolve to
a hash; AC-445's lint hashes every old test file; AC-451's balance surfaces the 54th on the first run. These are [measured] numbers a reader cannot re-derive, in an area whose own rule (SM-133) is
'never as one percentage', and an accepted ADR names an instrument that never ran.

**Remediation.** Amend s-mig-report.md (a DRAFT) with the uncapped counts and note the cap; correct the eight restatements to 76/87 or drop the per-AC sub-counts and keep 19/153; optionally lift
`[:6]` in the script for future runs and record it in METHOD-S by dated amendment. Replace 'the census overall of 98%' with the record-weighted 96% (denominator stated: Lifted / (Lifted +
LiftedWithLoss + Failed) over typed slots) or drop the overall figure as SM-133 advises. Record against ADR-0075 (amendment/supersession note or the design's section 10.x table, never an edit): the
census instrument was census-s-mig.py and the 98% is the spec-section slot only. Correct AC-443/SM-140/section 0.1 to name `src/store/verify.ts` as origin (old tree, hash at pin) and reconcile.rs as
its re-implementation, with a RUNBOOK line saying where rk-git lifts it from after Step 2 (ADR-0002 by amendment note); restate AC-445/SM-140 over 'the 237 test files at the pin' or record the 101's
classification list in redkiln-corpus; 53 -> 54 (or 'across the fifteen tabled slots'); 42 -> 43 with the boundary-window path in AC-469 and the design.

**Folds.** G19-01, G19-02, G19-09, slice-G20-05, slice-G20-14, slice-G20-11, slice-G20-13

### PIA-080 — F38's 0.45x/0.55x/0.78x divides P8 output tokens by F28's cache-creation tokens; ADR-0087's 'costs less' (title, clause 3, forbids) rests on it against the design's own n=3 rule and misattributes METHOD-P 5.4's kill; SM-002/SM-052/SM-073 name one instrument and quote the other; and the 3.66x/4.4x 'contradiction' holding R-P7RATIO is two denominators

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B06-C03] ARCH-1.2-040 (design:132) and ADR-0087:53/:91 record 'no within-experiment baseline, compared only to F28'; nothing records that F28's
figure is a different token class, nor the 'buys nothing' over-read | [B07-C06] none — ARCH-0.1-011, ARCH-3.10-005 and SM-NOBASE-079 record n=3 / PARTIAL / inside-TTL caveats only; no row or
anomaly_pool entry records the unit mismatch, and the runner bakes it in (spikes/runner/src/main.rs:3348-3349, `f28_stage_gate_tokens: 18600`). | [B04-C13] ADR-0087-FORB-75 (forbids clause as
deliberate exception) and ARCH-1.2-040 (the design's 'not without n=3' rule); ARCH-1.2-035, ARCH-10.4-011, ARCH-10.5-011/012, EB-E-006, ADR-0080-FORB-174 and anomaly_pool #27 (the ratio UNRECONCILED,
R-P7RATIO Not started). None records that the ADR violates the design's rule, that ADR-0087:17 misattributes METHOD-P 5.4's kill consequence, or that the two ratios have different denominators.) ·
**Category:** inaccuracy · **Confidence:** high · **Owner:** Ryan (ADR-0087 successor; METHOD-P amendment); Claude (F38/design/SM/AC corrections)

Merged from 3 concerns; each member statement is carried verbatim. (i) ADR-0087 clause 3 and its forbids line ('measured, it costs less'), SM-052/SM-073's baselines and AC-266's provisional note rest
on F38's 0.45x/0.55x/0.78x. [measured] The numerator is P8's `resume_tokens` = summed `output_tokens` of the resumed dispatch (main.rs:3288; 8,408/10,314/14,579) and the denominator is F28's 18,603
`cache_creation_input_tokens` on a post-TTL re-read whose own output was 348-933 tokens (main.rs:3349 divides by a hard-coded 18600.0; FINDINGS:1714's column header is 'cache creation on resume'). On
the field SM-052/SM-073/AC-266 name as their instrument the same P8 dispatches recorded cache_creation of 84,960/49,468/46,961 = 4.57x/2.66x/2.52x of 18,603 -- above the pre-registered >1.5x kill,
with the cross-fixture caveat that P8's prompt is larger than F28's ~36k review prefix. Design:735 mislabels the three figures as `cache_creation` tokens, ADR-0087:17 mislabels F28's as 'output
tokens', and the ratio is restated roughly 27-35 times across seven to nine files (the '88 restatements' one finder claimed did not reproduce). A second over-read of F28 stands at design:828, AC:3136
and SM:588 -- 'F28 proves a keep-warm dispatch buys nothing' -- though F28 ran no keep-alive arm and its own correction (FINDINGS:1800-1805) credits the programme's constant unrelated dispatching for
the 48% that stayed warm; what F28 bounds is the rewrite (~18.6k-29.7k per gate). METHOD-P section 5.4 pre-registered 'resume cost in tokens' without fixing the token type, so P8-B inherits the driver
metric. (ii) [measured] F38's 8,408 / 10,314 / 14,579 are `usage.output_tokens` of the first post-resume dispatch (the runner sums `output_tokens`; p8.jsonl rows for issue-81/74/79 copy them into
`resume_tokens`), as F38's own table header says; F28's 18,603 is `cache_creation_input_tokens` ('cache creation on resume', FINDINGS.md :1714-1718; every 18603 in spikes/results is a cache_creation
field). The ratio quoted in §0.1 row 29, §3.5, AC-013, AC-039, SM-002 :49 and ADR-0087 therefore divides unlike counters, and ADR-0087 :88 mischaracterises F28's figure as 'output tokens'. SM-002 :48
names `cache_creation_input_tokens` as its instrument; on that instrument the same three dispatches read 49,468 (issue-81) / 84,960 (issue-74) / 46,961 (issue-79) — 2.5×–4.6× of 18,603, summed over
agentic turns so not a clean comparator either — the side of METHOD-P §5.4's >1.5× line the pre-registration anticipated. SM-002 as written cannot be evaluated against its own baseline, and ADR-0087's
provisional mid-graph placement rests on a 'does not fire' verdict for a criterion whose units were never fixed. (iii) Design 6.3.6:900, 0.1 row 29 and 7.10 rule that 'mid-graph gates are cheaper than
stage gates' may not be written without n=3 and the missing baseline in the same sentence, and design:894 takes the decision 'because it is safe under either answer'; the accepted ADR governing the
row states the forbidden sentence in its title (restated wherever the ADR is listed by title), in Decision clause 3 ('Measured, it is cheaper') and in its forbids block with no qualifier, and its
'what would change this decision' names a >1.5x cost result - so the decision is not safe under either answer per the ADR - while ADR-0087:17 attaches activity-terminal escalation to that kill where
METHOD-P 5.4:229-230 attaches it to the lost-output kill and only a pricing change to the cost kill [measured]. Separately, design 1.2:127 calls F37's 3.66x and the decisions log's 4.4x 'two numbers,
one quantity', refuses to price the top rigor tier until reconciled, and schedules R-P7RATIO as a 297-record recount; recomputed from spikes/results/p7.jsonl, the numerator is one mandatory run at
$9.857 and the denominators differ: the 30-run recommended arm mean $2.693 gives 3.66x (F37) and the same-item recommended run issue-77#r1 at $2.222 gives 4.44x (Decision #33), no running prefix mean
of the arm yielding 4.4 [measured]. Neither record states its denominator, which is the whole of the 'contradiction'; ADR-0080's n=1 forbiddance still holds after reconciliation.

**Anchors.**
- `spikes/F38-DRAFT.md:33` — "| item | resume output tokens | ÷ F28's ~18.6k |"
- `references/2026-09-08-success-measures.md:48` — "- **Instrument:** `cache_creation_input_tokens` on the first dispatch after a re-entry; the `run` entry event."
- `references/adr/0087-mid-graph-gate-resumes-into-the-same-step.md:3` — "title: A mid-graph gate resumes into the same step, and costs less than a stage gate"
- `spikes/FINDINGS.md:1714` — "| gap | cache-read share | cache creation on resume |"
- `references/2026-09-08-success-measures.md:49` — "- **Baseline:** mid-graph resume **8,408 / 10,314 / 14,579** tokens = **0.45× / 0.55× / 0.78×** of a stage gate's ~18,603 [measured, F38, **n=3, PARTIAL**"
- `references/adr/0087-mid-graph-gate-resumes-into-the-same-step.md:48` — "3. Gate placement is therefore a value decision, not a cost decision: a mid-graph gate is not to be avoided on the grounds that it is dearer than a stage gate. Measured, it is cheaper."

**Why it matters before Step 5.** (i) Step 5 is untouched and no crate is blocked: AC-266's test compares both arms on one field, and gate placement is 'a value decision' either way (row 29). But
SM-073, one of three discriminating human-gate measures, would compare its first production reading against a baseline of a different quantity; an accepted ADR forbids a design choice on a comparison
never made; and P8-B inherits the driver metric -- fix before P8-B runs. Top of P2. (ii) No kernel code is built from the ratio, but SM-002 is the first cost measure the shipped kernel will report
against, and its baseline and instrument are in different units; fix the record before the first SM-002 reading is taken. (iii) No build step depends on either sentence. Pack authors and the AC-273
placement-record lint take their rule from ADR-0087's forbids block, which today forbids a placement argument on evidence the design says may not be stated that way; SM-090 and the section 7
review-tier price stay blocked on a $0 spike that need not run.

**Remediation.** (i) Supersede ADR-0087 partially (never edit): withdraw clause 3's 'measured, it costs less' and the matching forbids line, restating F38's result as 'output tokens of the resumed
step, not comparable to F28's cache-creation figure; cost ordering UNMEASURED'. Amend F38 by a dated correction section restating section 2 on cache_creation_input_tokens (84,960/49,468/46,961) beside
output tokens, noting that on that field the >1.5x kill fires against F28 subject to the cross-fixture caveat. Amend METHOD-P section 5.4 by dated note fixing the token type for P8-B
(cache_creation_input_tokens and total_cost_usd, per SM-073). Re-baseline SM-052/SM-073 as 'no baseline', correct design:735 and AC-266's provisional note to state the class and the corrected numbers,
and reword the three 'buys nothing' sites to 'F28 bounds what a keep-warm could buy at one prefix rewrite (18.6k-29.7k) per gate; F22 forbids the process anyway'. (ii) State the comparator in one
unit: either re-express F38's resume cost as `cache_creation_input_tokens` on the first post-resume dispatch and re-evaluate METHOD-P §5.4's 1.5× criterion (noting the multi-turn summation), or
restate SM-002's instrument as output tokens and F28's comparator accordingly. Amend F38 by dated entry (never edit METHOD-P); annotate or supersede ADR-0087's evidence line :88; correct SM-002 :49
and the §0.1 / §3.5 / AC-013 / AC-039 restatements. (iii) Supersede ADR-0087 (never edit): keep clauses 1-2 and 4-6, drop 'costs less' from the title and clause 3, rewrite the forbids line as 'forbids
deriving placement from a cost model', and scope 'what would change this decision' so a >1.5x result changes the pricing guidance per METHOD-P 5.4 rather than the suspend/resume shape; set
superseded_by on 0087. For the ratio, amend (never edit) F37-DRAFT by dated note and add a DECISIONS-2026-09-09 row: '4.4x = $9.857 / $2.222 (issue-77 recommended r1, same-item single run); 3.66x =
$9.857 / $2.693 (recommended arm mean, n=30)'; update design 1.2 row, 10.4 and the R-P7RATIO register row to 'reconciled by denominator identification; the emit.corpus_sha re-keying remains'; leave
ADR-0080 as is.

**Folds.** G08-01, G05-02, G08-08, G11-04, G08-06, L01-contradiction-01

### PIA-084 — Four registered-unrun spikes cannot decide their own criterion: S-FACET (auth 0 positives, no surfaces labels), S-CODEX (saturated), S-BIN, P2-B

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (RB-GC-010 (RUNBOOK.md:119, 'four measured instances' of a threshold fixed without checking the corpus can produce the denominator), ARCH-10.5-002
(design:1861, S-FACET's kill 'legitimate either way'), F39-LIM-001 and ARCH-1.2-017 (design:109, n=20 unreachability, auth all-false), ADR-0007:55 and ARCH-10.5-033 (S-CODEX has no pass criterion),
ARCH-10.5-005 (design:1864, P2-B as the cure for the missing denominator), MS-AMEND-010 (the S-BIN section 7 amendment). None records that the registered criteria of these unrun spikes inherit the
fault; the S-BIN combination-rule gap is unrecorded (NEW half).) · **Category:** prereg-integrity · **Confidence:** high · **Owner:** pre-registration author (Ryan Britton); Claude for the METHOD-S
amendment text

S-FACET, the only registered unblock for the derived-facet BLOCKED row and the named source of SM-024's baseline, registers a kill on 'precision >= 0.95 on auth/surfaces and on ui' over the reused
140-cell arm-D label set. Reconstructed from the four label files, that set holds 7 facets x 20 items with no `surfaces` column at all; `auth` has 0 true / 14-16 false / 4 unknown, so its precision is
NOT ANSWERABLE (the fault the document names for data_migration only); `ui` has 9 positives, so 9/9 gives a Wilson 95% lower bound of 0.70 and the bar is POWERLESS [measured]. The same document states
27 lines earlier that the 0.95 clause 'is not reachable at n = 20 for any facet however good'; the kill was committed at e642a5b (2026-09-09 16:05) and that result absorbed at a431c62 (22:22) with the
kill untouched [measured, git]. The criterion names no estimator (point vs lower bound) and no abstention number, and S-FACET has no METHOD file. S-CODEX (design:1892, 46 dispatches) is shaped on E3
arm (b) over the same 23 fixtures, a comparator the corpus lists as saturated (23/23 vs 23/23, p = 0.625, 'Did not fire'; arm (b)'s FP is also at floor, 0/23), ADR-0007:55 records no pass criterion,
and no METHOD file registers it: the only decidable outcome is a regression. S-BIN's METHOD-S section 3.4 rule is single-pass ('2/2 detection and <= 1/3 false positive') while section 3.6 sizes two
passes 'for a stability read' with no combination rule (grep for 'both passes|two passes|disagree' returns only line 372) [measured]; the 2026-09-09 amendment re-affirms both 'exactly as registered',
and F29 established that verdicts on identical inputs are a weighted coin, so a 2/2-then-1/2 split is the expected case. P2-B exists only as a section 10.5 allocation row (design:1864) naming
question, consumers and reason but no criterion form, no n and no positive count; deciding recall >= 0.9 on a Wilson lower bound needs roughly 36 all-positive rows and F40's corpus had 0 of 41
[inferred]. Each is the class golden constraint 10 counts at four, repeated in the registered-unrun set.

**Anchors.**
- `references/2026-09-09-change-classification.md:297` — "**Kill criterion.** If arm 3 (or 3+4) does not reach precision ≥ 0.95 on `auth`/`surfaces` and on `ui` at an abstention rate the triage gate can absorb, t"
- `references/2026-09-09-change-classification.md:270` — "No facet clears METHOD-P §1.4's 0.95 bar, and the ≥ 0.95 clause is not reachable at n = 20 for any facet however good."
- `references/2026-09-08-architecture-design.md:1892` — "| **S-CODEX** | E3 arm (b) through `codex exec --json --output-schema` on the same fixtures: detection, FP, cost, missing capabilities | AC-239, SM-154; ADR-0007 | 46 Codex dispatches [documented, proposal §8] | No | Registered, unrun |"
- `references/2026-09-08-evidence-brief.md:74` — "**Did not fire.** E3 §9 (23/23 vs 23/23, p=0.625). E3-H."
- `spikes/METHOD-S.md:372` — "5 items × 1 arm = 5 dispatches, repeated twice for a stability read = **10"
- `references/2026-09-08-architecture-design.md:1864` — "Registered because a threshold with no denominator is what F36, F37, F39 and now F40 each hit | AC-068, SM-021; §0.1's `stale_policy` BLOCKED row; §4.4 step 3 | **Not computed.** P2's own perturbation half cost **$0** model cost [measured, F40]"

**Why it matters before Step 5.** None of the four is a scheduled RUNBOOK step (phase1-deps.json has zero S-FACET references; S-CODEX is Phase 4; P2-B gates only the stale_policy setting; S-BIN
unblocks a slice Step 5.3 already fences), so no Phase-1 block. Each must be fixed before dispatch, because a post-data re-registration is the fault the house rules forbid; S-BIN is the nearest to
dispatch.

**Remediation.** Dated pre-dispatch amendments, registered wording left verbatim (ADR-0012: a named spike with a named consumer, never prose). S-FACET: a METHOD-S2 file or a dated amendment block
under change-classification section 6 stating the estimator (Wilson 95% lower bound vs point), the positive count per facet in the reused labels (auth 0, ui 9, surfaces none), declaring `auth` NOT
ANSWERABLE and `surfaces` NOT MEASURABLE on this corpus unless a labelled surfaces column is built, fixing the abstention number, and either re-sizing (second corpus, ~60-73 positives per facet) or
re-registering the kill on a criterion the 20 items can decide (arm 3 vs arm 1 false-positive count, paired per item). S-CODEX: a METHOD entry before dispatch with a primary metric on a non-saturated
measure (S-SUBTLE-tier detection; CLEAN false-positive rate with its Wilson upper bound, noting arm (b)'s 0/23 floor), a stated pass rule and predictions, citing E3 section 9's saturation. S-BIN: a
dated METHOD-S section 7 amendment: rule evaluated per pass, branch 1 requires both passes, a pass missing a seed routes to branch 2/3 on that pass's prose, both passes reported. P2-B: a dated
METHOD-P amendment (or METHOD-P2B) stating criterion form, the minimum number of blind-labelled 'should re-run' rows, their construction and the cost once sized.

**Folds.** L09-01, slice-G01-05, L09-09, L09-08, slice-G03-07

### PIA-089 — Pre-data claims fail against git: METHOD-H 'before any H4 dispatch' post-dates three arm (a) records; METHOD-P's baseline post-dates P6; ledger short

**Priority:** P2 · **Classification:** NEW — ARCH-1.2-025 (design:117) records the METHOD-H amendment as 'Replaced pre-data, so no result is contaminated' -- the row this concern shows to be false;
HST-UNRES-004 is a planning row. The METHOD-P half SHARPENS-KNOWN: MP-AMEND-002 (spikes/METHOD-P.md:242) and HANDOFF.md:273-276 disclose amendments 6-7 as post-P6-first-run, not that the baseline
commit itself post-dates the P6 data. The ledger miscount is unrecorded (HANDOFF staleness is recorded generically; the 'this said seven' self-correction at HANDOFF:253 does not cover the residual). ·
**Category:** prereg-integrity · **Confidence:** high · **Owner:** pre-registration author (Ryan Britton); spikes/HANDOFF.md owner

[measured, git] spikes/results/h4.jsonl lines 1-4 carry H4 / arm a-contiguous records (the ORIGINAL section 6 design's contiguous arm) started 2026-09-03T04:37Z, first committed in 7a796ce (2026-09-02
21:39 -0700) whose body reads 'H4 arm (a), contiguous, n=3: all three answered "13"'. The section 6 amendment landed in 3eb93d5 (2026-09-04 23:23) headed 'before any H4 dispatch'; its motivating claim
(the registered A/B 'would return no divergence') was written after three arm (a) draws had returned identical answers; F28:1700-1701 and design:117 repeat 'replaced before data', and neither
METHOD-H, F28 nor the design discloses the three records. METHOD-H:3-5's 'approved and running from 2026-09-04' also post-dates h1.jsonl's earliest record (2026-09-03T04:36Z). METHOD-P declares
'Baseline: this document's commit', but `git log --diff-filter=A -- spikes/METHOD-P.md` returns exactly one commit, ab58733 (2026-09-07 21:20 -0700), which adds METHOD-P.md together with F34-DRAFT.md,
F35-DRAFT.md and roughly fifteen P6 result files under spikes/p9/, and that first form already carries amendments 6 and 7, dated 2026-09-08 and described as 'made after P6's first run': the
pre-amendment section 3.2 wording exists only as the amendments' own paraphrase, and 'No P data has been seen' is unverifiable from git for P6 (for P1/P2/P7/P8 the commit does precede the data). Every
dated amendment is disclosed in its own file, but the ledger a reader would use undercounts: HANDOFF:252 says 'Nine amendments' over METHOD.md, METHOD-E3, METHOD-H and METHOD-P, which carry 11 dated
in-file entries (omitting METHOD.md:127, METHOD-E3:132, METHOD-H:147 while counting METHOD-E3's creation as item 1); METHOD-E3:4 says 'three, all on 2026-09-03' but the file carries four, the fourth
(:57) committed 2026-09-02; METHOD-P:242 attributes 'Amendments 1-5' to 'this document' when they belong to METHOD.md/METHOD-E3 [measured].

**Anchors.**
- `spikes/METHOD-H.md:147` — "### Amended 2026-09-04, before any H4 dispatch — the registered comparison is vacuous"
- `spikes/results/h4.jsonl:2` — "{"experiment":"H4","arm":"a-contiguous","identity":"probe#1:1:haiku","label":"probe","phase":"a-contiguous","occurrence":1,"attempt":1,"model":"haiku","effort":"low","agent":null,"started_at":"2026-09-03T04:37:22.921489200Z""
- `references/2026-09-08-architecture-design.md:117` — "| METHOD-H §6 **†** | **VACUOUS — replaced before data** | The criterion could not fail as written | [documented, brief §E] | Replaced pre-data, so no result is contaminated |"
- `spikes/METHOD-P.md:4` — "**Baseline:** this document's commit. **Corpus pin:** redkiln @ `47faec9f` (S-MIG's frozen `.bklg`)."
- `spikes/METHOD-P.md:3` — "**Written before any P1/P2/P6/P7/P8 dispatch or analysis. No P data has been seen.**"
- `spikes/HANDOFF.md:252` — "the P-series and owns amendments 6–9. **Nine** amendments, each disclosed in the"

**Why it matters before Step 5.** No build consequence: H4 is 'not a kill' and its amended result is saturated (FULL 1.000 = CONTROL 1.000); F35's two-band reading stays primary. It is the corpus's
own audit ledger being wrong about pre-data status, and the house rule (followed by every METHOD-S entry) is that a post-data amendment names the records it post-dates.

**Remediation.** Dated correction entries, registered wording untouched (METHOD files are amended, never edited). METHOD-H section 6: state that the amendment post-dates three arm (a) records
(h4.jsonl lines 1-4, 2026-09-03T04:37Z, commit 7a796ce), that they contribute to no F28 figure, and whether they were seen before the amendment was written; correct design:117's 'replaced before data'
and F28:1701 to 'amended after three arm (a) draws, disclosed'; reconcile METHOD-H:4's 'running from 2026-09-04' with the 2026-09-02 dispatches. METHOD-P section 6: a dated note that the file's first
commit is ab58733, that it post-dates P6 runs 0-4 and already carries amendments 6-7, that the pre-amendment section 3.2 text is recoverable only from the amendment paraphrases, and that the
P1/P2/P7/P8 registrations do precede their data; reword :242 to attribute amendments 1-5 to METHOD.md/METHOD-E3. HANDOFF:248-252: replace 'Nine' with a per-file table (file, count, last date)
generated by grep; fix METHOD-E3:4 to 'four, 2026-09-02/03'.

**Folds.** L09-02, L09-03, L09-10

### PIA-096 — F39's panel-adjudicated auth 0.00 (load-bearing for ADR-0089) rests on an uncommitted adjudication surface, no attestation; 18 citations unlabelled

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (RUNBOOK.md:881 and HST-AMEND-008/009 (spikes/HANDOFF.md:277,282) record that amendment 8's attestation route never attached and blindness rests
on amendment 9's construction; AC-1.1-002 and SM-3.3-006 record the 'panel-adjudicated' label where it IS applied. Neither the uncommitted instrument, the browser-memory recovery, nor the 18
unlabelled citations is recorded ('browser memory', 'blind_attestation', 'wherever' return zero known-issue rows).) · **Category:** prereg-integrity · **Confidence:** high · **Owner:**
pre-registration author (Ryan Britton); design document owner for the citations

METHOD-P section 0 rule 1 requires that any substitution for a blind panel carry a `blind_attestation:` line and an explicit `--attested-blind` flag 'so the substitution can never happen silently'.
RUNBOOK:881 records that this route was never invoked and that arm D's blindness rests on amendment 9's construction ('revealed only after he answered'). [measured] The surface that implemented that
ordering was a claude.ai Artifact page, identified from the 2026-09-09 handoff's account of its `db.doc()` / `collection().get()` defects and of 32 adjudications 'recovered from browser memory' after
a save failure [inferred from HANDOFF-2026-09-09:79-82; adjudication stamps 19:24-20:30Z fit a live page]; `git log --all --name-only` shows no page, script or export of it (spikes/p1 holds only
triage-arm-d.py and score-arm-d-panel.py; the only committed labelling page is F40's p2-labels-page.html); the input p1-arm-d-contested.json carries the panel's per-seat positions beside every
contested cell, and p1-arm-d-labels-ryan.md:27 still reads `blind_attestation: FILL-ME`. So for the 32 cells that produce F39's headline (auth 0.00 over eleven positives; 15 of 32 human verdicts
differing from the script) the reveal-after-answer ordering is attested by prose alone, with less on record than amendment 8's own minimum and no committed instrument to audit. Amendment 9 also
requires results 'reported as panel-adjudicated, never as human-panel, wherever the number is cited'; a scripted scan over the design, AC and SM registers, RUNBOOK, evidence brief and
change-classification finds 60 lines citing F39 with one of its figures, of which 18-19 (the refuter reproduced 19, adding SM:211) carry no 'adjudicat' within six lines, including the design's section
0 summary (line 9) and the section 10.6 falsifier row (1909) [measured]. No claim is made that the labels are wrong.

**Anchors.**
- `spikes/METHOD-P.md:32` — "`blind_attestation:` line naming who attests, what they attest they did not open, and when — and the scorer must be"
- `spikes/METHOD-P.md:33` — "invoked with an explicit `--attested-blind` flag, so the substitution can never happen silently."
- `spikes/plans/HANDOFF-2026-09-09-design-session.md:81` — "an array; the oracle emits `binary_change` as a JSON bool, not a string. The first cost Ryan 32 adjudications"
- `spikes/results/p1-arm-d-labels-ryan.md:27` — "blind_attestation: FILL-ME (e.g. "Ryan Britton attests on 2026-09-09 that he had not opened spikes/results/p1-arm-d.jsonl before completing these labels")"
- `spikes/METHOD-P.md:95` — "human-panel, wherever the number is cited."
- `references/2026-09-08-architecture-design.md:9` — "no facet clears METHOD-P §1.4's 0.95 bar, `auth` scores **0.00** [0.00, 0.26] over eleven positives every one of which is false, and the bar itself is unreachable at n=20 (F39; ADR-0089)"

**Why it matters before Step 5.** No build impact. F39's auth 0.00 is the evidence ADR-0089 stands on and the corpus's own rule is that a weaker blindness check 'can never happen silently'; here the
check is weaker than either registered form and the instrument is unrecoverable.

**Remediation.** A dated note to METHOD-P section 0 rule 1 (registered wording untouched) and to F39 section 5 naming the adjudication surface, stating that it is not in the repository and that the
answers were recovered from browser memory, and adding the amendment-8-style attestation line for the 32 cells (who, what was not opened, when); if the Artifact page still exists, export its source
into spikes/p1/ so the reveal-after-answer logic is inspectable. For the label: either add '(panel-adjudicated)' at the 18-19 citations, or a dated scoping amendment to amendment 9 limiting the
requirement to result tables and first citation per document -- the second is cheaper and honest.

**Folds.** L09-04, L09-05

### PIA-097 — F31's block counts, sink seeds and typed-arm scoring came from the model-emitted evidence label ADR-0074 forbids; the Rust class is unmeasured

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ADR-0074:42 records 'model-chosen evidence class' as a rejected option and ARCH-6.6-005 (design:1039) records the area; AC-284's evidence line
partially discloses 'human adjudication, not mechanical re-execution'. No row records that the cited counts were produced under the rejected routing.) · **Category:** statistics · **Confidence:** high
· **Owner:** Claude (design/ADR author); Ryan to accept the spike registration

[measured] The registered S-TYPED instrument's `policy_verdict` (spikes/runner/src/e3.rs :254-256) reads each finding's model-emitted `evidence` string and calls `policy.blocks(evidence)`; the 12/23
strict and 9/23 lenient block counts, the seven narrowed-pin sink seeds and the `blocking_files` detection scoring that §0.1 rows 42/44, §6.4.2, ADR-0074 :95 and ADR-0082 :86 cite as [measured, F31]
were therefore produced by exactly the model-label routing ADR-0074 clause 2 / forbids :83 rejects. No measurement exists of a class computed in Rust from a re-execution record; the s-typed.jsonl
record (253 findings, 118/29/29/77 by label) has never been replayed through the clause-2 decision table. The review gate's quantitative justification (block counts, the lenient default, the Advisory
sink's seven seeds, AC-310's fixture expectation) therefore attaches to a mechanism Phase 1 will not build, and the mechanism Phase 1 will build has no measured discriminating power. Not a block: the
classifier is fully specified by AC-284/288/289 and the lenient default is disclosed as Ryan's judgement (AC-298 note, ARCH-1.2-032).

**Anchors.**
- `references/2026-09-08-architecture-design.md:42` — "The evidence class is therefore computed in Rust from the repro record and a finding blocks only after the deterministic gate re-executes it and it flips. ADR-0074 (full supersession of ADR-0004)"
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:83` — "Forbids taking the evidence class from the model. The model's label may be recorded; it may not be routed into the policy table."
- `spikes/runner/src/e3.rs:254` — "let evidence = f.get("evidence").and_then(|e| e.as_str()).unwrap_or("");"
- `spikes/runner/src/e3.rs:256` — "if policy.blocks(evidence) {"
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:57` — "The evidence class is computed in Rust from the repro record**, never taken from the model: `reproduced` iff a command ran and `observed ≠ expected_when_met`; `mutation-survived` iff a mutation was applied and the named suite stayed green; `narrowed-pin` iff only the discriminating assertion flips"

**Why it matters before Step 5.** Whoever writes AC-284's classifier has no data saying it reproduces the label split the policy table was tuned on; a $0 replay over the 253 recorded findings answers
that before the classifier is written, and is worthless after the fixtures are re-cut around it.

**Remediation.** Re-tag the F31 citations in §0.1 rows 42/44, §6.4.1 clause 2, §6.4.2 and (via a superseding note, never an edit) ADR-0074/0082 as 'measured under the model-label policy; the
Rust-computed class is unmeasured'. Register a $0 replay spike in §10.5 with AC-284 and SM-091 as named consumers (ADR-0012): run the clause-2 decision table over the 253 recorded S-TYPED findings and
report per-class agreement with the model label; mark AC-284 PROVISIONAL until it runs; do not cite the 9/23 or 0/23 as evidence for the Rust classifier until then.

**Folds.** G15-01

### 4.5 Register integrity (10)

### PIA-018 — Three undisclosed ordering hazards: 7c lifts deleted files; no commit carries ADR-0090/4a edits; Step 5 waits on a disputed arm-L act

**Priority:** P2 · **Classification:** NEW — none in the corpus for the first two (the census's phase1-deps OH-2 flags the 7c path; the cherry-pick gap is a seventh hazard outside its six). The arm-L
item sharpens H10-OWED-002 (handoff:57 'feeds nothing') and RB-GAP-010 / OH-5, which record the ordering but not the disagreement or that it is the sole human blocker before Step 5 · **Category:**
sequencing · **Confidence:** high · **Owner:** RUNBOOK author (Claude); Ryan (operator) for the arm-L sequencing choice

In the canonical order 0 -> 1 -> 3 -> 4 -> 4a -> 2 -> 5 -> 6 -> 7 -> 8 (437), Step 2 copies spikes/runner to redkiln-corpus (471) and then `rm -rf`s it (500); Step 7c, thirteenth, lifts `runner.rs`,
`event.rs`, `step.rs`, `throttle.rs`, `journal.rs` 'from the spike runner' (2013) and design §9.1 says 'from `spikes/runner`', with no path to the corpus pin; the ORDER banner names only Steps 3b/4/4a
as readers of the moved directories [measured]. Every `git add`/`git commit` in the file was enumerated [measured]: Step 1.3 (371-388) and Step 2 (479, 502, 532) only, none in Steps 3, 4 or 4a; Step
2's redkiln-rs commit stages only CORPUS_PIN and .gitignore, so ADR-0090, the 0079 status flip, README's anchors, the 4a facets.toml edits and surfaces.toml are all uncommitted when Step 5 writes 'CI
from the first commit'; Checkpoint 2 asserts a clean corpus checkout but not a clean redkiln-rs tree; Step 9's rollback cherry-picks 'the Step 4 commit' and 'the Step 4a commit' (2256), which no step
creates. Step 5 waits on Step 2 landed (1599); Checkpoint 2 lists p1-arm-l-labels-ryan.md (543-545), which does not exist and only Ryan's blind session creates; RUNBOOK 1249-1251 says its absence
blocks no decision and to fill it now and hold it for a re-run, while the 2026-09-10 handoff (58) says filling it 'currently feeds nothing' and to write the scorer first — the scorer
(spikes/p1/score-arm-l.py) does not exist [measured]. The only undelivered human act on the critical path to pen-to-paper is thus one the corpus's two most recent documents sequence differently.

**Anchors.**
- `RUNBOOK.md:500` — "rm -rf spikes/corpus spikes/results spikes/runner"
- `RUNBOOK.md:2013` — "call**. `rk-adapter-claude` lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs` from the spike"
- `RUNBOOK.md:502` — "git add CORPUS_PIN .gitignore && git commit -m "Extract redkiln-corpus at a pinned SHA (ADR-0013); AC-465 becomes enforceable""
- `RUNBOOK.md:2256` — "git cherry-pick <the Step 1 commit> <the Step 4 commit> <the Step 4a commit>"
- `RUNBOOK.md:545` — "   D:/repos/redkiln-corpus/results/p2-labels-ryan.md         # → all three; Step 3b's work travelled"
- `spikes/plans/HANDOFF-2026-09-10-design-session.md:58` — "   `spikes/HANDOFF.md`: **arm L's labels have no scorer**, so filling them currently feeds nothing. Write the"

**Why it matters before Step 5.** [measured] The second and third sit on the path to Step 5 itself: ADR-0090 — the Step-5 gate token, read from disk at 1914 so the build proceeds — would enter history
inside a Phase-1 code commit or sit uncommitted, and Step 9's rollback as written cannot run; and Step 5 is gated through Checkpoint 2 on an act with a disputed consumer. The 7c path is a dead pointer
with a one-sentence fix.

**Remediation.** In 7c write 'after Step 2 these live at `$CORPUS_REPO/runner/src/` at the SHA in CORPUS_PIN; copy from that commit, not a working tree, and name the SHA in the adapter's module
header', and add 7c to the ORDER banner's list of steps Step 2 affects. Add a commit block to Checkpoint 4 ('ADR-0090: ... supersedes 0079', staging 0090, 0079, README) and to Checkpoint 4a ('Apply
ADR-0089: deriver, surfaces, tri-valued booleans', staging spikes/p9/facets.toml and surfaces.toml); add `git status --porcelain` -> EMPTY to Checkpoint 2. For arm L, pick one and record it as a
DECISIONS line: (a) write `spikes/p1/score-arm-l.py` first so the act has a consumer and stays in Phase 0; or (b) let Checkpoint 2 accept arm L as outstanding, to be labelled in
redkiln-corpus/results/ and committed at a new pin.

**Folds.** L08-04, slice-G04-07, slice-G12-09, L08-09, L08-10

### PIA-029 — Register self-description fails verification: seven ADR clauses have no criterion, cargo-deny is unprovisioned, the area files do not exist

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (AC-15.3-003/005/007/009/013 (register 6506-6522: pointers 'inferred from the ADR's filename only ... must be checked before it is relied on' for
ADR-0076/0078/0080/0082/0086); AC-15.3-P-002 and ARCH-10.x-L-002/003 (area files treated as live sources); ADR-0025-FORB-52 and ADR-0018's forbids rows hold the clauses. None records the pointers
checked false, the ADR-0025/0018 clauses unenforced, cargo-deny unprovisioned, or the area files absent) · **Category:** coverage · **Confidence:** high · **Owner:** AC register author (Claude); Ryan
(operator) for the cargo-deny decision (ADR-0084 successor) and the ownership statement

§15.3 admits its ADR pointers were 'inferred from the ADR's filename only' and 'must be checked before it is relied on'; checked, seven fail [measured, grep over the register]. ADR-0025's mandatory
advisory: AC-067 is a NEGATIVE criterion whose THEN is conditional on 'any drift signal' and whose instrument asserts only non-co-occurrence with `Stale`; none of the twelve consumers asserts an
upstream change emits `drift_advisory` or that the next gate carries the note. ADR-0078's both-TTLs clause: `SUBAGENT` occurs zero times in the register; AC-210/AC-423 name one variable; RUNBOOK 7c
(2042) states the rule correctly, so the build will probably comply, but the acceptance set cannot detect the omission. ADR-0080's empty-finding-set predicate lint: AC-112 tests presence of a quality
block and AC-115 is a human gate; `findings.none` and 'policy predicate' occur zero times; 7b states the prohibition as prose. ADR-0076's sync-trait rule: AC-480 tests generic bounds only; the sole
enforcement is two RUNBOOK greps, the second over direct dependencies only, where 'depending on' is undefined between direct and transitive. ADR-0018's ownership refusal (HEAD not the run's branch):
only AC-518 — a different rule — mentions 'run's branch'; AC-181's two-worktree test passes with or without it. ADR-0082's automatic retirement path (a): no criterion implements it or names its event;
AC-305's identity tuple is scoped to quarantine. ADR-0086's crate ban: temporalio / graph-flow / iopsystems occur zero times in the AC, SM and RUNBOOK; AC-005's `bans.deny` names only `rk-adapter-*`.
AC-005, AC-044 and AC-120 verify by `cargo-deny`, yet ADR-0083/0084's CI decision (fmt, clippy, test, audit), §9.2's tool rows, §9.3's assertion list and RUNBOOK 5.2's install list (1687-1688:
cargo-audit only) never mention cargo-deny or a deny.toml; AC-484 covers the kernel-to-adapter edge with a `cargo metadata` assertion, so one invariant has two instruments, one unprovisioned. And
design 349/1968 and register 6532 assign ownership of every criterion's wording to thirteen `ac-*.md` area files that exist nowhere in the tree or `git log --all` [measured]; the design uses the
phrase both for those files and, at 3/1990, for the assembled registers (crediting them with AC-520 and SM-157, which exist only there). Four deferrals route to the literal files — the AC-218/AC-428
reconciliation 'before either is built' (362), the AC-511 id conflict (1585), the owed AC-114 criterion (1847), SM-015's target — and are dischargeable only if the register is declared the owner,
which nothing states; the brief (12) records that no area file was refuted against F39/F40.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:6506` — "**Fifteen of the 33 name behaviour this register demonstrably does cover, and simply cite an older ADR for it.**"
- `references/adr/0078-cache-rule-is-split-both-ttls-set-scope-corrected.md:79` — "- Forbids setting `CLAUDE_CODE_PROMPT_CACHE_TTL` without also setting `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`."
- `references/2026-09-08-acceptance-criteria.md:2753` — "THEN `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` is set explicitly and its value is recorded on the dispatch event."
- `references/2026-09-08-acceptance-criteria.md:246` — "*Verified by:* a `cargo-deny` `bans.deny` entry for every `rk-adapter-*` crate in `rk-kernel`'s graph, plus a CI"
- `RUNBOOK.md:1687` — "cargo install cargo-audit --locked"
- `references/2026-09-08-acceptance-criteria.md:6532` — "**Area files (the direct inputs).** Thirteen files, each authored against the proposal and the evidence brief and then adversarially refuted against the brief line by line: `ac-kernel.md`, `ac-selection.md`, `ac-packs-catalog.md`"

**Why it matters before Step 5.** [inferred] Step 5.2 writes the CI and install list — the cargo-deny/deny.toml choice, including ADR-0086's four bans, lands there and is a four-line fix now. The rest
are coverage gaps the RUNBOOK's one-off greps fence once and nothing tests afterward: an engine that never emits the advisory, sets one TTL, loads the forbidden predicate, or writes from a foreign
worktree passes every Phase-1 AC while violating an accepted forbids clause. The dead ownership pointer leaves four deferrals no builder can discharge as written; the RUNBOOK gates no step on them, so
the record, not the build, is wrong.

**Remediation.** At 5.2 either `cargo install cargo-deny --locked` plus a deny.toml (ADR-0086's four crates in `bans.deny`, workspace-wide including dev-dependencies, with a must-fail fixture; decide
whether its advisories check replaces cargo-audit so the audit job is one tool) or re-point AC-005/044/120's Verified-by at AC-484's `cargo metadata` assertion — recorded in a successor to ADR-0084.
Add criteria, using ids outside exhausted ranges as AC-520 did: a positive clause for ADR-0025 (GIVEN a vouched root and a same-stage upstream changing digest THEN exactly one
`drift_advisory { root, upstream, old_digest, new_digest }` is appended and the next `gate_opened` record carries it); both TTL variables in AC-210's THEN/Verified-by and AC-423's GIVEN, ADR-0078
added to AC-210's decisions; an rk-catalog lint criterion with a red fixture carrying `findings.none(evidence in [Reproduced, MutationSurvived])` (ADR-0080); 'no `async fn` and no future-returning
method in any `rk-ports` trait' plus `cargo tree -p rk-kernel -e normal` naming none of tokio/async-std/futures/async-trait (ADR-0076, transitive); a refusal criterion in §6.4 for a worktree whose
HEAD is not the run's branch, with AC-181's GIVEN stating each process owns its branch (ADR-0018); an `advisory_retired { reason: not_re_raised }` event and
`live_sink == created - retired - dismissed` on AC-310's replay invariant (ADR-0082). Correct §15.3's rows to the new ids. Either commit the thirteen area files under references/areas/ marked 'history
— superseded by the assembled register on 2026-09-08', or amend design §3.6.1 (349), §10.x (1968) and register §16 to state the assembled register is the owner and the area files its non-tracked
sources; then discharge the four deferrals in the register (an AC-218/AC-428 row in §0.3 naming the owner; the AC-511 note; the AC-114 criterion authored; SM-015's target amended).

**Folds.** slice-G02-07, slice-G04-04, slice-G07-08, slice-G12-07, slice-G14-09, G15-13, G11-09, G21-06, L12-authority-02

### PIA-031 — Step 5 owns the lint/CI policy yet leaves AC-001-004 with no clippy.toml, a trybuild fixture that cannot fail on a lint, and two settleable unknowns

**Priority:** P2 · **Classification:** NEW — The clippy.toml / trybuild incompatibility is unrecorded (AC-15.3-010 at AC:6519 lists ADR-0083 only as an 'unverified pointer' for AC-001..005's
instruments). The AC-158 / AC-121 half SHARPENS-KNOWN: ARCH-5.11-009 (design:790) and AC-UNVER-001 (AC:1732) record both as unverified and call it 'a runbook decision', not that the runbook declined
the decision it was delegated or that a schemars feature flag settles AC-158. · **Category:** testability · **Confidence:** high · **Owner:** RUNBOOK Step 5 author; AC register author (kernel area)

trybuild 1.0.121 (local registry, src/cargo.rs:97/130/153) invokes `cargo build` or `cargo check`, never clippy, so the RUNBOOK 2161 must-fail fixture 'reintroducing a clock read' compiles cleanly
(SystemTime::now() is not a rustc error) and 'failing to compile is the pass' cannot fire as specified [measured]. The only real instrument is the clippy `disallowed_methods` / `disallowed_types` list
AC-001-AC-004 place 'in `clippy.toml`', which ADR-0083:122 records as absent and no RUNBOOK step (5.1, 5.2 or 7b) authors (grep for clippy.toml in RUNBOOK.md: zero hits) [measured]. clippy.toml is
resolved per crate manifest, so AC-001's module scope `rk-kernel::workflow` is expressible only through a module-level `#[deny(clippy::disallowed_*)]` override that ADR-0083:70 permits; the missing
file and its owning step are the gap. Separately, RUNBOOK 1777 marks AC-158's property order and AC-121's two-OS half 'unverifiable as specified' though 5.3 chooses schemars, whose 1.2.2 manifest
exposes `preserve_order = ["serde_json/preserve_order"]` [measured], and 5.2 writes the CI matrix, which names only 'a Windows runner' (1671) where ADR-0084:60 says Windows is not optional, not
Windows only. AC-158 itself says 'settle that in the runbook'.

**Anchors.**
- `RUNBOOK.md:2161` — "must-fail fixture reintroducing a clock read (AC-001–AC-004). The fixture **failing to compile** is the pass."
- `references/2026-09-08-acceptance-criteria.md:215` — "*Verified by:* a clippy `disallowed_methods` list in `clippy.toml` applied to `rk-kernel::workflow` naming"
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:122` — "no `rust-toolchain.toml`, no `clippy.toml`, no `tests/` dir (17 files use inline `#[cfg(test)] mod tests`), and no `.github/workflows` directory exists in the repository at all."
- `RUNBOOK.md:1777` — "half and `AC-158`'s property ordering are **unverifiable as specified** until CI fixes two OS targets and an"
- `references/2026-09-08-acceptance-criteria.md:1927` — "property order is not observable in the exported schema and this criterion is unverifiable — settle that in the runbook."
- `RUNBOOK.md:1671` — "pull_request]`, a matrix with a Windows runner, `dtolnay/rust-toolchain` reading `rust-toolchain.toml`, and"

**Why it matters before Step 5.** Step 5.1 lays down the lint policy files and 5.2 the CI matrix; adding clippy.toml and the two settings then is free, and retrofitting after the kernel exists is the
churn ADR-0083 warns about. Phase-1 exit criterion (4)'s instrument is mis-specified but discovered at Step 7a with no wrong artefact built, so P2.

**Remediation.** In RUNBOOK 5.1 author `clippy.toml` (workspace root, or crates/rk-kernel if the ban is kept crate-scoped) with the `disallowed-methods` / `disallowed-types` lists and name the
module-level `#[deny(clippy::disallowed_methods)]` on rk-kernel::workflow as ADR-0083:70's commented override; replace the trybuild half of AC-001-004's Verified-by with a fixture crate checked under
`cargo clippy` (or a compile_fail that rests on a type-level ban). In 5.3 specify schemars and serde_json with `features = ["preserve_order"]` and assert property order in the snapshot test; in 5.2
name the matrix `[ubuntu-latest, windows-latest]`; then strike the 'unverifiable' clause at 1777 for AC-158 and AC-121's cross-OS half. ADR-0083 stays untouched (its Decision omits the file; the
RUNBOOK step, not the record, owns authoring it).

**Folds.** G21-05, L06-buildability-12, G21-08

### PIA-035 — Checkpoints 3c, 5, 6 and 7b/8 carry lines green on the untouched tree or on a violation; the checklist ticks deliverables no line asserts

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (RB-GC-009 (RUNBOOK.md:111) and RB-HALT-008 (RUNBOOK.md:2387) record the 'green when you have done nothing' class and the RUNBOOK corrects the E3
BLANK instance at 895-899; RB-GAP-006 (RUNBOOK.md:1766) records that Checkpoint 5's false-pass audit covered the oneOf and ChangeSummary lines only; RB-GC-008 (RUNBOOK.md:106) and ADR-0077-FORB-101
record the rules, not the checks. None records these instances.) · **Category:** testability · **Confidence:** high · **Owner:** RUNBOOK author (Claude); Ryan Britton for 5.2's CI table

[measured] Run read-only on the current tree, Checkpoint 3c's S-BIN grep (906-907) returns 13 lines: 9 from ADR-0008 (2026-09-05), 1 from ADR-0012, 2 from ADR-0029, 1 from adr/README.md and 0 from
spikes/DECISIONS-2026-09-09.md, so 'Silence on the first is a failed checkpoint' (919) can never trigger; Step 5's 'Step 3c answered' prerequisite (1599) rests on a command green before 3c does
anything, and the actual design plan (METHOD-S section 7, 862) sits in a file the grep does not read. Checkpoint 7b/8's 'golden constraint 1, as a command' (1994-1995) requires `require` and the facet
name on one line, but the pack format puts `[[stage.X.require]]` and `when =` on separate lines: over spikes/p9 it returns 3 catalog-comment hits and 0 of the 72 pack rows keyed on the seven LLM
facets, and its `crates/` scope names a directory no document places packs in (the only named path is `.redkiln/packs`, once, at AC:939). Checkpoint 6's wrapper grep (1851) returns 0 on
`use std::process::{Command, Stdio}; Command::new("git")` and on `tokio::process::Command`, its S-GIX verdict grep (1855) returns 1 on ADR-0077 itself (a forbids line with no verdict), it is broader
than ADR-0077's rule (every std::process::Command, not git invocations), it never re-runs after Step 6 when the adapters appear, and no AC or CI assertion carries the wrapper rule (AC section 15.3
lists ADR-0077 as uncited). Checkpoint 5's `grep -L 'workspace = true'` (1760) passes a member with `edition.workspace = true` and no [lints] table, and `channel = "1.97` (1758) accepts the floating
channel ADR-0083:61 forbids; the RUNBOOK runs `cargo new` (1613) before the root manifest, so cargo emits a literal edition and no inherited lints. Checkpoint 5 has no line for
`exclude = ["spikes/runner"]`, the Windows target or AC-362/480/481/483/484 although the one-glance checklist (2465) ticks them (three cannot exist before 7a/7b); no checkpoint asserts the AC-465
residency lint exists or runs; Checkpoint 6 omits the git minimum version (2467, 1829) that 7d does grep for; Checkpoint 7b omits the rigor-monotonicity lint 1970 says 'ships in Phase 1 either way'.

**Anchors.**
- `RUNBOOK.md:907` — "  | grep -viE 'registered,? unrun'                                  # → the design plan, the re-scope, or the deferral"
- `RUNBOOK.md:919` — "Silence on the first is a failed checkpoint. Design §10.2 also names the price of getting this wrong: *"A Phase-1"
- `RUNBOOK.md:1995` — "     crates/ --include=*.toml --include=*.rs | wc -l                   # → 0. Golden constraint 1, as a command."
- `RUNBOOK.md:1851` — "grep -rn 'std::process::Command' crates/ --include=*.rs \"
- `RUNBOOK.md:1760` — "grep -L 'workspace = true' crates/*/Cargo.toml           # → empty: every member inherits lints"
- `RUNBOOK.md:2465` — "structural assertions AC-362/480/481/483/484 wired"

**Why it matters before Step 5.** Step 5's gate names 'Step 3c answered' and Checkpoint 5 is the first Phase-1 gate; as written the checklist will tick items no line verifies and the
golden-constraint-1 guard reports 0 on a tree containing 72 forbidden rows. Nothing is built wrong (5.3 and Checkpoint 5 independently fence ChangeSummary; the AC-071 lint is the real gate), so P2.

**Remediation.** One RUNBOOK revision (the RUNBOOK is a living document; the checkpoints are not ADRs). 3c: point the check at artefacts an answer creates
(`grep -c 'before any S-BIN dispatch' spikes/METHOD-S.md`, `ls references/adr/009*-s-bin*`, `grep -n '^## F.*S-BIN' spikes/FINDINGS.md spikes/F*-DRAFT.md`) and exclude ADR-0008/0012/0029. 7b/8:
replace the single-line grep with the AC-071 pack-loader lint (a `require.when` / `applicable_if` predicate naming a facet whose registry `source` is `llm` fails load) or at minimum a multi-line
`when =` scan over `.redkiln/packs` and `crates/`, and note that the p9 packs contain 72 such rows today. 6: a clippy `disallowed_types` entry for `std::process::Command` and `tokio::process::Command`
outside rk-git's wrapper module, a CI grep on `Command::new("git")` added to 5.2's assertion table with an AC under section 6.4, and `grep -cE '^S-GIX/[ab]: (go|no-go)$'` on two explicit verdict
lines; state in 7c whether adapters must use tokio::process. 5: `grep -L '^\[lints\]' crates/*/Cargo.toml`, `grep -q 'channel = "1.97.1"'`, write the root manifest before `cargo new`, and add lines
for the exclude entry, the Windows target and the residency lint's own invocation; move AC-480/481/483/484 from 2465 to 7a/7b. 6:
`grep -cE 'git (>=|version) [0-9]+\.[0-9]+' references/adr/0091-*.md -> 1`. 7b: `cargo test -p rk-kernel -- rigor_monotonicity`.

**Folds.** L08-08, slice-G03-02, L03-02, slice-G06-07, G13-09, G21-09, L08-11

### PIA-038 — About 95 criteria are assigned to eight crate labels that exist in neither design section 9.1 nor the RUNBOOK workspace

**Priority:** P2 · **Classification:** NEW — none (the labels occur in known-issues.json only inside quoted AC headers, AC-PROV-006 and AC-PROV-027, never as a recorded mapping issue) · **Category:**
traceability · **Confidence:** high · **Owner:** AC register owner (Ryan Britton)

The register's criterion headers name `rk-kernel-verify`, `rk-select`, `rk-catalog`, `rk-packs`, `rk-port-toolchain`, `rk-store-artifacts`, `rk-policy` and `rk-doctor`: a strict header regex counts
28/22/18/12/4/2/2/2 = 90 criteria, and including PROVISIONAL/HUMAN GATE-prefixed headers reaches about 95 [measured]. None of the eight names occurs in RUNBOOK.md or the design (grep count 0 for each)
[measured]; section 9.1 lists ten crate families and folds packs, policy and gates into `rk-kernel`; the register carries no note that its crate labels are area-file names [measured: no hit for 'crate
label', 'module of', 'is not a crate']. Instruments such as 'a lattice CI monotonicity check' or an `rk-store` build test therefore point at a crate a builder cannot find, and no checkpoint can name
`cargo test -p rk-select`.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4005` — "**AC-320** — `rk-kernel-verify` · evidence: §3 principle 4 (proposal) · decisions: ADR-0019"
- `references/2026-09-08-acceptance-criteria.md:610` — "**AC-040** — `rk-select` · evidence: catalog §2.5 reproducibility contract [documented]; F35 for the corpus size only"
- `references/2026-09-08-architecture-design.md:1651` — "| `rk-kernel` | packs (process + technology), stages/activities, policy, memo, gates, budgets, retry, replay | new"

**Why it matters before Step 5.** Step 5.1 creates the workspace skeleton; whether these are modules of `rk-kernel` or their own crates decides where ~95 instruments live and what `cargo test -p` a
checkpoint can name. The design governs (RUNBOOK:4) so a builder can place them in `rk-kernel`, but traceability from a failing test to its criterion breaks silently while the mapping stays implicit.

**Remediation.** Add a crate-label to section-9.1-crate mapping table to the register's section 0 (e.g. `rk-select`, `rk-catalog`, `rk-packs`, `rk-policy`, `rk-kernel-verify` are modules of
`rk-kernel`; `rk-store-artifacts` of `rk-store`; `rk-port-toolchain` of `rk-ports`; `rk-doctor` of `rk-cli`) or, if they are meant to be crates, add them to section 9.1 and to Step 5.1's workspace
members. Either way, make the checkpoints' `cargo test -p` targets nameable.

**Folds.** L07-testability-04

### PIA-039 — No precedence rule ranks the design, AC register, SM register and RUNBOOK; the only ownership statement points at thirteen area files not in the tree

**Priority:** P2 · **Classification:** NEW — none ('precedence' in known-issues hits only ARCH-0.1-010, cache TTL; the anomaly pool has nothing). SM:136 records one instance of the gap biting, not the
rule gap. · **Category:** authority · **Confidence:** high · **Owner:** Ryan Britton (design and register author)

Four documents each assert binding force -- the design is 'the authoritative architecture design' (over the proposal), the AC register is 'Binding on the Phase 1 build runbook' and 'decides nothing
new', the SM register is 'Binding on Phase 1 telemetry', the ADR folder is 'the canonical location' -- and the RUNBOOK defers only to the design 'where the two disagree'. No document ranks the
design's rendering of a criterion against the register's definition, or the AC register against the SM register: the design names neither register by path and neither register names the design (0
occurrences each) [measured]; ADR-0085 rules 1-9 fix formats and ids but rank no document against another. The design's only ownership statement assigns the criteria wording to 'the area files'
(design:349, :1968), thirteen files that are not in the repository. The SM register's SM-015 note shows the gap biting: it declines to correct a target because doing so 'would make this document the
authority on a figure AC-071 owns'. Where design and register renderings differ (the AC-511, AC-218/AC-428 and AC-173 cases elsewhere in this audit), a builder has no rule to pick, and the corpus has
left three such differences standing rather than resolve them.

**Anchors.**
- `references/2026-09-08-architecture-design.md:3` — "**Status:** **The authoritative architecture design.** It supersedes `2026-09-05-architecture-proposal.md` at revision 4 in full; where the two disagree, this document governs and the proposal is history."
- `references/2026-09-08-acceptance-criteria.md:3` — "Binding on the Phase 1 build runbook; it decides nothing new and supersedes no ADR."
- `references/2026-09-08-success-measures.md:3` — "Binding on Phase 1 telemetry; every id below is stable and must not be renumbered."
- `RUNBOOK.md:4` — "`references/2026-09-08-architecture-design.md` §0.1 and §10.2; where the two disagree the design governs and this"
- `references/2026-09-08-architecture-design.md:1968` — "§3.6's rule stands: the area files own the criteria and this document cites rather than restates them."
- `references/2026-09-08-success-measures.md:136` — "choosing one of the two numbers in this pass would make this document the authority on a figure AC-071 owns. What is certain either way: the **stricter** number (≥ 10) is the one a re-run must satisfy"

**Why it matters before Step 5.** Step 5 builds against criteria that exist in two renderings and against a RUNBOOK with two masters. No Step-5 threshold differs between design and register today and
the RUNBOOK cites neither AC-511 nor AC-218/AC-428, so P2: correctness of the record with real remediation value, not a workstream blocker.

**Remediation.** Record one precedence rule as a new ADR (documentation conventions are ADR-0085's subject; add a record, never edit it) and cite it from each of the four headers: (i) accepted ADRs
govern every decision and change only by supersession; (ii) references/2026-09-08-acceptance-criteria.md is the in-tree owner of each criterion's GIVEN/WHEN/THEN, instrument, state label and id -- it
decides nothing new, so where the design's rendering differs the design is amended, never the register's THEN; (iii) references/2026-09-08-success-measures.md owns targets, baselines and kills, except
that a measure restating an AC's executable precondition takes the AC's number; (iv) spike files govern the brief (already stated at AC:4) and the rule is applied uniformly; (v) the RUNBOOK defers to
the design for phase order and to the AC register for thresholds and instruments. Replace 'the area files own the wording' (design:349, :1968) with the register, or check the area files in and declare
them history.

**Folds.** L12-authority-01

### PIA-047 — No step builds or checkpoints the three gates (AC-240–AC-359), filed under 7c; the cancel path and $0 spikes are unscheduled

**Priority:** P2 · **Classification:** NEW — none for the gate placement or the missing gate checkpoint [measured: AC-240/AC-320/AC-333/'checkpoint 7' searches return only AC-0.3-030]; AC-15.3-014
lists AC-265 only as ADR-0087's consumer. The spike part sharpens ARCH-10.5-019 (R-SQLITE 'Not started'), ARCH-10.5-031 (S-CANCEL-WIN) and ARCH-10.4-030/RB-HALT-006, none of which records the RUNBOOK
omission · **Category:** sequencing · **Confidence:** high · **Owner:** RUNBOOK author (Claude)

Step 7c's build text and its six checkpoint commands (2052-2061) cover `rk-adapter-claude/sqlite/node` only, yet its acceptance line (2070-2071) carries the human gate, review gate and verification
gate. The register assigns those to the kernel — 33 of 39 human-gate ACs and 25 of 39 review-gate ACs to `rk-kernel` [measured] — built in 7b, whose scope line names 'gates' in one word and whose
acceptance set (2006-2008) is AC-001–AC-119 plus AC-520. The human gate has a `--help` checkpoint in 7d; the review gate reaches the RUNBOOK as `Findings` in the rk-types list, a restated prohibition,
and 7c's `structured_output_required` (AC-282); the verification gate's instruments — the AC-317/AC-334 mutation harness and the AC-333 evidence ledger — appear in no build text anywhere [measured:
`mutat*` at 1723, 1977, 2215 only], and no checkpoint in 7a-7d exercises any gate behaviour. AC-265 (mid-graph resume across a real `taskkill`) is cited only inside the register although Step 8 (3)
performs exactly that test without naming it [measured, ids.json]. Likewise checkpoint 7c contains no cancel test though AC-224/AC-225 fall in its range, and R-SQLITE, S-RETRY and S-CANCEL-WIN — each
with a Phase-1 consumer in Steps 6, 7c or 7d, and each $0 or a reading — occur zero times in RUNBOOK.md, against S-GIX (scheduled explicitly at Step 6.1) 14 times [measured].

**Anchors.**
- `RUNBOOK.md:2070` — "**AC-360–AC-399** (event log, cost attribution, throttling), **AC-240–AC-279** (human gate), **AC-280–AC-319**"
- `RUNBOOK.md:2071` — "(review gate), **AC-320–AC-359** (verification gate)."
- `RUNBOOK.md:2006` — "**Acceptance set: AC-001–AC-039** (kernel), **AC-040–AC-079** (selection, minus the P1-blocked rows AC-069,"
- `references/2026-09-08-acceptance-criteria.md:3367` — "**AC-265** — `rk-kernel` · evidence: F38 · decisions: ADR-0017, ADR-0022"
- `RUNBOOK.md:2052` — "**Checkpoint 7c.** All six:"
- `references/2026-09-08-architecture-design.md:1878` — "| **R-SQLITE** | rusqlite's first-party multi-process WAL and `busy_timeout` guidance, plus the two-process contention test §9.4 asks for | ADR-0084; SM-057 | Reading | No | Not started |"

**Why it matters before Step 5.** [inferred] Step 5 is unaffected and nothing is blocked — the register's Verified-by instruments exist and 7c's line names the 120 criteria. But Step 8 (1) requires
every lane stage, so a Phase 1 following the RUNBOOK can reach Step 8 with `Findings` typed and nothing that classifies, re-executes, blocks or sinks, and discover it at self-hosting; ADR-0019's
evidence norm has no place in the execution order. A gate sub-step with a runnable checkpoint is cheapest before 7b starts.

**Remediation.** Add a sub-step (7b-ii) whose build text enumerates the gates' kernel work — the human-gate `(token, tree_hash, deadline)` record; review classifier AC-284..286, policy table
AC-294..298, re-execution AC-287..290, quarantine AC-304..307, sink AC-310, replay AC-313; verify harness AC-317/AC-334 and ledger AC-333 — and whose checkpoint runs at least the AC-334 flip-predicate
test, the AC-015 inversion regression (must yield `mutation-survived`), the AC-313 k=10 replay digest, the AC-310 `findings_in == blocks + sink_records` replay, the AC-294 trybuild block-constructor
test, and a `redkiln answer` round-trip. Move AC-240–AC-359 into that sub-step's acceptance set, leaving AC-282 in 7c, AC-291/292 in Step 6, AC-263 in 7d, and the four adapter-touching human-gate ACs
(243/267/269/279) cross-listed in 7c; cite AC-265 at Step 8 (3). Add to Checkpoint 7c `cargo test -p rk-adapter-claude -- cancel_reaps_process_tree` (AC-224) and an S-CANCEL-WIN fixture-cancel-rm run
on Windows recorded in the register row; add a one-line 'spikes owed by this step' note to Steps 6, 7c and 7d naming R-SQLITE, S-CANCEL-WIN and S-RETRY with their cost, mirroring Step 6.1's S-GIX
treatment, so each is run or explicitly deferred in the checkpoint commit.

**Folds.** L07-testability-02, G15-03, G08-09, slice-G12-12, L13-operational-08

### PIA-051 — One behaviour, two criteria: six twin pairs missing from section 0.3, AC-317/AC-334 give the mutation harness two predicates, AC-094 vs AC-093

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (design:362 (AC-218/AC-428 'must be reconciled') and design:1333 (two doctor tables, 'not reconciled') record two of the six pairs in the design
only; AC-0.3-032 (AC:95) records the AC-317/AC-334 split as resolved by delegation and the section 10 residual (AC:4450) records AC-334's derivation as unspecified. None records the six pairs in
section 0.3, that the two mutation predicates differ, or the AC-093/AC-094 contradiction (unrecorded, NEW half).) · **Category:** overlap · **Confidence:** high · **Owner:** AC register maintainer
(Claude); SM register owner for SM-094

Section 0.3 is, by its own words (AC:60), the list a builder uses to know which criterion owns a behaviour, because 'a build that satisfies the owner and skips the twin will read as green in both
areas'. Its 35 rows name none of six cross-area pairs that each assert one behaviour with agreeing exit conditions and differing instruments [measured, all six read in full]: AC-029/AC-233 (retry
envelope, the only pair in the register with 3-gram Jaccard above 0.6), AC-092/AC-420 and AC-093+AC-094/AC-421 (doctor dangling-reference and dead-definition rules), AC-005/AC-484 (kernel-to-adapter
ban, AC-484 the transitive form; design:289 calls it 'enforced twice'), AC-238/AC-494 (`StepError::Unsupported(capability)`) and AC-218/AC-428 (exit-0 `is_error` to `StepError::Platform`); the design
itself demands two of them be reconciled before either is built. AC-317 (section 9.8) requires the named *test* to fail under two mutations ('removed, and inverted' -- undefined for a step that is
neither an ordering nor a boolean) and refuses at report time, with SM-094 targeting every AC; AC-334 (section 10.3) requires the named *assertion* to flip under one THEN-derived mutation at stage
exit, with SM-100 targeting >= 0.90 and a human-gate residual; the design's headline (section 6.5.3) is assertion granularity, which AC-317 does not require, so the harness AC-0.3-032 says 'produces'
AC-334's class is specified with a different predicate, count and moment, and the wall-clock kill SM-094 predicts 'is very likely to fire' doubles under AC-317's two mutations. AC-093 exempts a slot
'marked optional'; AC-094's GIVEN ('a role, skill or slot defined in an enabled pack and referenced by no enabled pack') carries no exemption and its fixtures include a 'dead slot', so an optional
unfilled slot passes AC-093 and fails AC-094, making the marker inert.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:60` — "all of them are a traceability hazard, because a build that satisfies the owner and skips the twin will read as green in both areas."
- `references/2026-09-08-acceptance-criteria.md:3959` — "THEN the named test fails under both mutations, and an AC whose test survives either mutation is refused at report time."
- `references/2026-09-08-acceptance-criteria.md:4149` — "AND if the suite stays green the AC is recorded `mutation-survived`."
- `references/2026-09-08-architecture-design.md:1004` — "**An AC is a claim about ONE ASSERTION, and Red→Green is evidenced at TEST granularity.**"
- `references/2026-09-08-acceptance-criteria.md:1418` — "THEN it reports a dead slot and exits non-zero."
- `references/2026-09-08-acceptance-criteria.md:1425` — "THEN it reports a dead definition and exits non-zero."

**Why it matters before Step 5.** No wrong build: the six pairs agree on exit conditions, a mutation harness can satisfy both texts by running a superset, and the doctor clause is one line. The record
specifies one mechanism twice with incompatible parameters and omits from its traceability list the two reconciliations the design says must precede the build.

**Remediation.** AC register amendments. Section 0.3: add the six rows naming an owner per pair (suggested AC-029, AC-420, AC-421, AC-484, AC-494, AC-218) and cross-reference design:362 and
design:1333. Mutation: make AC-334 authoritative for the predicate (assertion flips under the THEN-derived mutation), rewrite AC-317 as the harness-invocation criterion that calls it, dropping
'removed, and inverted' or defining the derivation once, and align SM-094's instrument with AC-334. Doctor: drop 'slot' from AC-094 (AC-093 owns slots) or add 'and not marked optional' to its GIVEN.

**Folds.** L05-overlap-06, G16-06, slice-G18-12

### PIA-059 — Five Phase-1 criteria's instruments pass on the behaviour they forbid or read inputs nothing writes: AC-173, AC-224/225, AC-481, AC-511, AC-068

**Priority:** P2 · **Classification:** NEW — AC-481, AC-511 and AC-068 are unrecorded (AC-0.3-014 at AC:77 praises AC-511 as 'strongest' on the very conjunct that names no field; ARCH-10.5-017 cites
AC-481 only; AC-PROV-008 records AC-068's status change). The AC-173 half SHARPENS-KNOWN: the register's own cut note at AC:2355-2356 concedes the lint 'would miss a TTL cache one call deeper' but not
that the remaining instruments miss it too. The AC-224/225 half SHARPENS-KNOWN: AC:3537 records the job-object-vs-process-group hazard for the sibling AC-278 and never carries it across. ·
**Category:** testability · **Confidence:** medium · **Owner:** AC register author (git-reconcile, harness, ports-dist, adapter and policy areas); Step 7c implementer for the probe

AC-173 (Step 6): a reconcile() that emits `reconcile_started`, compares HEAD/reflog to a stored digest and returns memoised state passes every instrument named for AC-173, AC-174, AC-175 and AC-183
while being precisely the 'unchanged since' flag the THEN clause forbids; the cut note concedes the lint misses it and names no replacement. AC-224/AC-225 (Step 7c): the corpus runs on Windows (every
RUNBOOK command is `cd D:/...`; the measured kill is `taskkill /T /F`), where there is no POSIX process group, no `fork` and no exit 143; a parent-PID tree walk passes AC-224's fixture as worded while
missing the `Start-Process`-detached descendant F22 recorded as the 12,000 s survivor, and only a Job Object with KILL_ON_JOB_CLOSE reaps it; AC-225's 143 branch has no Windows instance. AC-481 (Step
7a): Rust cannot enumerate traits at runtime, so `traits.len()` comes from a hand-maintained inventory or a source-level count; under the former the sixth-trait fixture fails only if the author also
edits the inventory -- the same act that would prompt the fake -- and Checkpoint 7a greps only for the literal expression (design:1575's AC-501 already requires a count 'computed from the sources').
AC-511 (Step 7c): asserts the auth mode 'from the child's own system/init' but names no field; the spike runner's EffectiveConfig reads session_id, model, agent, cwd, permission_mode, tools,
mcp_servers and slash-command count and no auth field, results retain only a digest, and no file in the repository names an auth-mode key [measured, grep] -- two implementers diverge, one silently
degrading to the argv/env checks AC-511 exists to strengthen. AC-068: its executable precondition exits 0 only when recall/precision 'clear' METHOD-P section 2.4's lines, the case in which section 2.4
prescribes no stale_policy, and exits non-zero when the criteria fire and prescribe one: the pass condition is the negation of the decision rule; the record it reads
(spikes/results/p2/shortcircuit.json) is written by nothing (score-p2.py has only --engine/--labels), a convention shared with AC-071's spikes/results/p1/stability.json (AC:161), and neither is marked
[WILL EXIST] [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:2351` — "THEN it runs in full and no cache, TTL or "unchanged since" flag skips it"
- `references/2026-09-08-acceptance-criteria.md:2356` — "and the lint would fire on correct code while missing a TTL cache one call deeper. It is not an instrument."
- `references/2026-09-08-acceptance-criteria.md:2893` — "*Verified by:* an integration test spawning a fixture that forks a grandchild, cancelling, then asserting no live PID in …"
  the group; run on CI as a regression against F22's 12,000 s survivor."
- `references/2026-09-08-acceptance-criteria.md:5994` — "THEN the count of fake implementations equals the count of port traits, and the equality is asserted as a number, not as a spot check …"
  AND a port trait added without a fake fails the suite naming the missing fake."
- `references/2026-09-08-acceptance-criteria.md:6064` — "AND the session's auth mode is asserted from the child's own `system/init` report to be the subscription session, not inferred from the"
- `references/2026-09-08-acceptance-criteria.md:941` — "#   AND rec.recall/rec.precision clear P2's PRE-REGISTERED lines."

**Why it matters before Step 5.** Nothing is blocked: AC-488's env assertion holds for the adapter, either AC-481 mechanism builds, nobody builds a cache first, and no RUNBOOK step schedules
`redkiln lint --stale-policy`. Each criterion will nevertheless report green over the regression it exists to catch, and the fixes are Verified-by sentences that cost nothing before the tests are
written.

**Remediation.** AC register amendments (it decides nothing new; no ADR changes). AC-173: a differential instrument -- run reconcile() through the rk-git read layer and assert the set of objects read
on the second invocation equals `rev-list --all` (or the first invocation's set), or a fixture that rewrites a trailer via `git replace` without moving HEAD and asserts it is reflected. AC-224/225:
name the Windows mechanism (Job Object, KILL_ON_JOB_CLOSE, no breakaway) and the POSIX one (setpgid + killpg), make the fixture's grandchild a detached/breakaway process so a tree walk fails, and
restate AC-225 per platform (Windows: the adapter's own TerminateProcess exit code). AC-481: specify that the trait count is derived from source (or a `#[port]` / `#[fake_for(Trait)]` registration
macro), that the fixture adds the trait without editing any inventory, and that the test names the missing fake by parsing the same source. AC-511: a one-dispatch probe in the Step 7c preamble (golden
constraint 4 already requires one real dispatch) that retains the raw system/init payload and names the auth-mode field, then amend AC-511 to assert on it by name; if none exists, drop the conjunct
and record a named open item with rk-adapter-claude as consumer. AC-068: exit 0 only when rec.labels_timestamped, rec.n_cases >= 20 with a positive class present, AND every pack's declared
stale_policy per class matches what section 2.4's fired/unfired criteria prescribe; name a record an instrument writes (extend score-p2.py with --out, or p2-engine.jsonl), mark it and AC-071's
stability.json [WILL EXIST].

**Folds.** G13-05, slice-G12-04, slice-G12-08, slice-G10-05, slice-G02-04

### PIA-071 — Steps 7a/7c acceptance sets sweep in out-of-phase, CONDITIONAL, retired, S-BIN-blocked and spike-gated criteria: AC-480-AC-519 whole (fourteen Phase-2/3, CONDITIONAL-unbuildable or retired rows), AC-497's `ChangeSummary` against Checkpoint 5's grep, AC-356 without 7b's 'minus the P1-blocked rows' form, and AC-312 against ADR-0074:89's forbids on the unrun S-SUBTLE-CB arm

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B01-C10] ARCH-8.10-006 (CONDITIONAL rows are non-requirements) and SM-NOBASE-084/086 (Phase-2 phasing); AC-1.2-003 (register:183 lists AC-497
among S-BIN-gated criteria) and ARCH-3.10-001; AC-PROV-029 and RUNBOOK:1723 (AC-356 blocked); ADR-0074-FORB-88, ARCH-10.5-015, AC-15.3-015 (S-SUBTLE-CB unrun, pointer unverified). None records that
the Step 7 acceptance lines sweep them in | [B02-C06] references/2026-09-08-acceptance-criteria.md:6373 ('roughly half the area is unexercisable during Phase 1') and :6366 (the skim-past-banner
hazard); unindexed in known-issues.json) · **Category:** blocked-leakage · **Confidence:** high · **Owner:** RUNBOOK author (Claude); Ryan (operator) for the AC-312 scoping decision

Merged from 2 concerns; each member statement is carried verbatim. (i) 7a names AC-480–AC-519 with only AC-239/496/502 excluded (1902-1903). The register's own area note (6372) phases AC-501..506 to
Phase 2 and AC-507/508/510 to Phase 3; AC-512, 514-517 carry CONDITIONAL and AC-519 'partly CONDITIONAL' in their headers, and design 1587/1630 says those rows 'bind on nothing and are not runbook
requirements'; AC-513 is retired [measured]. AC-497 — in range, PROVISIONAL, 'may be implemented' — asserts the Toolchain port returns a typed `ChangeSummary` value while Step 5.3's checkpoint (1763)
greps rk-types for the identifier and demands zero, and no document says where a shape-only type may live; 5.3 names AC-154 as not evaluable and never AC-497. 7c states 'AC-320–AC-359' flat (2071)
without the 'minus the P1-blocked rows' form 7b uses (2006), although AC-356's Verified-by is a `ChangeSummary` schema test §6.5.4 forbids until S-BIN reports. 7c's AC-280–AC-319 includes AC-312 (a
release regression corpus with negative-control and seeded fixtures) while ADR-0074:89 forbids 'shipping the fixture family without the presented-as-already-approved arm'; that arm (S-SUBTLE-CB) is
registered, unrun and unsized, and no RUNBOOK step names AC-312, the corpus regression job or the arm's framing fixtures [measured]. Whether 'with the arm' means its paired presentations are authored
(ADR-0088 clause 1) or S-SUBTLE-CB has run is unstated. (ii) Step 7a's acceptance set (RUNBOOK:1902) names AC-480 to AC-519 whole, unlike its five sibling steps (RUNBOOK:1774, 1863, 2006, 2118), which
carve out non-evaluable rows; the RUNBOOK never mentions CONDITIONAL, `rk-plugin-gen` or distribution [measured]. It therefore binds a Phase-1 sign-off to AC-501/503-506 (`rk-plugin-gen`, Phase 2,
'nothing in this sub-section is checkable before then'), AC-507/508/510 (Phase 3), the CONDITIONAL AC-512/514-517 and part of AC-519 that the register calls UNBUILDABLE, and retired AC-513: fourteen
of thirty-seven named ids [measured]. The design says reading the CONDITIONAL rows as runbook requirements is 'the exact failure this subsection exists to prevent'; the RUNBOOK does exactly that. The
finder's two further claims fall: the AC-502 exclusion on S-CODEX grounds is correct (AC:185), and line 1877's 'one real adapter' restates the port rule as design:1852 does, imprecise in a Phase-1
step but not a contradiction of design:246. Checkpoint 7a's commands are unaffected: wrong record, not wrong build.

**Anchors.**
- `RUNBOOK.md:1902` — "**Acceptance set: AC-480–AC-519** (ports, fakes; AC-239/AC-496/AC-502 are out of scope — S-CODEX is unrun and"
- `references/2026-09-08-acceptance-criteria.md:6372` — "- Phasing means roughly half the area is unexercisable during Phase 1: `rk-plugin-gen` (AC-501..AC-506) is Phase 2, the GitHub and mdBook adapters (AC-507, AC-508, AC-510) are Phase 3, the Codex adapter (AC-496) is Phase 4."
- `references/2026-09-08-architecture-design.md:1589` — "them as settled requirements on a build runbook is the exact failure this subsection exists to prevent."
- `RUNBOOK.md:1763` — "grep -rn 'ChangeSummary' crates/rk-types/src/ | wc -l    # → 0. Blocked on S-BIN — must not exist yet"
- `references/2026-09-08-acceptance-criteria.md:6366` — "- The whole surviving distribution block (AC-512, AC-514..AC-517, AC-519's receipt/rename halves, SM-149) is UNBUILDABLE until §4.9 is reopened and a distribution mechanism is actually decided. A reader who skims past the CONDITIONAL banner will read them as requirements."
- `references/2026-09-08-acceptance-criteria.md:6153` — "THEN the reviewer receives a typed `ChangeSummary` value"

**Why it matters before Step 5.** (i) [inferred] No checkpoint tests any of them, so the exposure is scope creep by a literal reader — rk-plugin-gen and installer-receipt work against an undecided
distribution mechanism, an opaque `ChangeSummary` placeholder to satisfy a grep, a forbidden fixture, or a regression corpus shipped against an accepted forbids clause — rather than a blocked crate.
7b already shows the convention; the fix is restating four lines before Step 7 is executed. (ii) Nothing at Step 5. As written Step 7a can never be signed off against its own acceptance set; the
register's banners and RUNBOOK:5 (Phases 2-4 excluded) fence the implementer, so no crate is blocked.

**Remediation.** (i) Restate 7a's acceptance set as AC-480–AC-500, AC-509, AC-511, adding AC-497 and AC-154 to the explicit exclusions (or stating that the Toolchain trait ships without
`change_summary` until S-BIN and where a shape-only type may live), and list AC-501–AC-508, AC-510, AC-512–AC-519 as out of Phase-1 scope with their phase or CONDITIONAL status and AC-513 as retired.
Restate 7c's set as 'AC-320–AC-359 minus AC-356 (PROVISIONAL, S-BIN) and the human-gated halves of AC-357/AC-358'. Add one sentence to AC-312 and to 7c's acceptance paragraph stating which reading
holds: (a) AC-312 is PROVISIONAL on S-SUBTLE-CB and ships in Phase 1 with the arm's framing-variant fixtures authored but unrun — then add the authoring task to a step and price it in §10.5; or (b)
ADR-0074's forbids applies at the first release shipping the family and AC-312 is deferred to it — then re-label AC-312 in §9's state line ('36 binding · 1 provisional'). (ii) Amend RUNBOOK Step 7a's
acceptance set to the Phase-1 subset: AC-480..AC-495, AC-497..AC-500, AC-509, AC-511, AC-518, and AC-499 only for ports with a Phase-1 real adapter; list AC-501..AC-506 as Phase 2, AC-507/508/510 as
Phase 3, AC-512/514..517/519(part) as CONDITIONAL-unbuildable, AC-496 as PROVISIONAL, AC-482 as human-gated, AC-513 as retired. Leave the AC-502 exclusion reason as it stands. Optionally reword line
1877 to 'each a sync trait with a fake, and a real adapter where Phase 1 ships one (SQLite, Claude, Node)'.

**Folds.** L03-08, L04-08, G16-13, G09-02, slice-G10-01

### 4.6 Ambiguity (12)

### PIA-032 — Review/verify registers name enum values they never define: Unverifiable class-or-outcome, weight vocabularies, ledger shape, basis_reason, AC-015

**Priority:** P2 · **Classification:** NEW — none (ADR-0074-FORB-84 and SM-5.1-032 record other facts about Unverifiable; AC-15.3-001 records that section 9 cites ADR-0004 instead of ADR-0074, the
route by which the settling clauses are missed; ARCH-0.1-026, ARCH-1.2-032, ARCH-6.6-006, ADR-0015-NEG-43 record the lenient default and 0.2/1.0 cost, not the vocabulary mismatch; AC-0.3-031/032
record the AC-315/342 mirror and AC-334/317 authority split, not the status collision; ARCH-0.1-027, ARCH-6.6-007, HST-UNRES-008 do not record the AC-015 id collision) · **Category:**
vocabulary-undefined · **Confidence:** high · **Owner:** Claude (AC/SM register author); design author for sections 6.4.2 and 6.5.1

AC-284 closes `EvidenceClass` over four variants with an exhaustive `match`, yet SM-088 instruments `class = Unverifiable`, design:932 lists `Unverifiable` under 'Computed class', AC-286 names the
re-execution record as classifier input where ADR-0074 clause 2 computes from the repro record alone, and AC-287/AC-290/SM-104 treat it as a re-execution outcome; ADR-0074 clauses 2-5 settle the shape
(four-valued class, `Unverifiable` an outcome that never blocks), so the residue is wording -- but SM-088's instrument cannot be satisfied by a four-member enum, and AC-313 never says in words whether
its k=10 replay re-executes `repro.command` [measured]. The review weight is lenient/strict (AC-295/296, ADR-0074 clause 5), a five-level deontic enum (AC-303, ADR-0020) and numeric `0.2`/`1.0`
(AC-299/300, binding) -- the form ADR-0020:42 says it supersedes -- with no stated projection, so AC-296's proptest 'over every weight x every class' and AC-307's `stage_weight` field have no
determined cardinality [measured: 'lenient' appears in no ADR mapped to the deontic ladder]. AC-334 records a survived mutation as `mutation-survived` ('not `pinned`') and AC-335 records the same
green-suite-plus-surviving-mutation fixture as `unpinned`; AC-337 and AC-345 speak of an evidence class and a `pinned` status as two fields; SM-100 sums four names as one denominator -- no ledger type
is defined in section 6.5.3, ADR-0019 or the register [measured]. Four of AC-352's six `basis_reason` variants (`no-selection-support`, `probe-unverified`, `policy-forced`, `toolchain-unsupported`)
appear only in AC-352 and design:989 undefined, and AC-328's refusal does not say which it emits. The register's own AC-015 is the memo-key criterion, yet 46 citations of 'AC-015' across design, AC
and SM registers -- including AC-334's only named regression fixture -- mean issue-146-147's AC-015 from F25, and ids.json resolves every one to the memo criterion [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:3613` — "closed `EvidenceClass` enum (no `_ =>` fallthrough to a blocking class); a fuzz input that produces `advisory`, never a panic."
- `references/2026-09-08-success-measures.md:656` — "- **Instrument:** `finding` records with `class = Unverifiable`, split by reason (`both_fail` | `cannot_run` | `timeout`)."
- `references/2026-09-08-acceptance-criteria.md:3750` — "GIVEN a stage whose review weight is `0.2` (read-only advisory)"
- `references/adr/0020-stage-rigor-is-a-deontic-enum.md:42` — "Stage rigor is a **deontic enum**: `forbidden < discouraged < optional < recommended < mandatory`, ordinal, never summed. Activities declare `min_rigor`. This supersedes the numeric weights in §4.2 and in D15 (review tiering)."
- `references/2026-09-08-acceptance-criteria.md:4149` — "AND if the suite stays green the AC is recorded `mutation-survived`."
- `references/2026-09-08-acceptance-criteria.md:4157` — "THEN that AC is recorded `unpinned` and the green suite changes nothing about its status."

**Why it matters before Step 5.** `EvidenceClass`, the review weight and the ledger status land in rk-types at Step 5.3 and the classifier, policy lookup and ledger in Steps 7b/7c. Each reading builds
and the binding ADR text gives the four-variant enum, so no crate is blocked; but AC-296's domain, SM-088's query, one of AC-334/AC-335's two named regression fixtures and AC-352's capture test freeze
whichever the builder chooses, and an ADR-0085 traceability reader following 'AC-015' lands on the wrong criterion.

**Remediation.** One vocabulary pass over AC sections 9-10 and SM sections 1.9-1.11 (AC/SM edits; ADR-0074 and ADR-0020 untouched): state that `EvidenceClass` is the closed four computed from the
repro record and `ReExecution = Flipped | Unverifiable { both_fail | cannot_run | timeout }` is a separate outcome carried on the finding, rename SM-088's instrument to `re_execution = Unverifiable`,
relabel design:932's row, and add 'its recorded re-execution record' to AC-313's GIVEN with a note that re-execution determinism is AC-288/AC-289's job; define `ReviewWeight` once (either
`{ReadOnly, Lenient, Strict}` or the explicit projection optional->ReadOnly, recommended->Lenient, mandatory->Strict) and rewrite AC-299/AC-300 in it, noting in ADR-0020's successor whether 'weight'
is distinct from stage rigor; state the ledger shape once (a single closed enum with `unpinned` reserved for 'no evidence' and AC-335 reworded to 'remains `mutation-survived`, never `pinned`', or an
explicit two-field record with SM-100/SM-101 rewritten) and add it to AC-333's Verified-by as a schema fixture; add a one-line definition per `basis_reason` variant in AC-352 or section 6.5.1 and name
the variant AC-328 emits; qualify every fixture citation as 'issue-146-147 AC-015 (F25)' and name AC-334's fixture by the F25 row, not the AC id.

**Folds.** G09-01, G15-05, G15-04, G15-10, G16-02, G16-08, G16-12

### PIA-033 — rk-store and redkiln-migrate contracts: locator end rule, Gwt shape, conservation vocabulary, and no locator/carrier/predicate for three criteria

**Priority:** P2 · **Classification:** NEW — locator, Gwt, interventions, layout: none (ADR-0005-FORB-49 forbids the locator becoming a grammar; ARCH-0.1-036, ARCH-10.x-T-002, ADR-0075-FORB-78 record
the permissive AC type; no row on AC-461/SM-137/'intervention' or AC-440/AC-442's predicate). Conservation: SHARPENS SM-NOBASE-080 (success-measures.md:980, the census 'did not enforce a conservation
identity') and SM-5.1-048 -- the seven-vs-five-vs-four vocabulary is unrecorded. AC-475: SHARPENS AC-PROV-034/035 (PROVISIONAL on S-BIN; second-conjunct vacuity) -- the first conjunct's missing
locator and S-MIG's existing opaque routing are unrecorded · **Category:** contract-ambiguity · **Confidence:** high · **Owner:** Ryan Britton (design section 5 and AC register author); AC register
section 13 for the migrate criteria

The locator is 'a heading-anchored slice' and nothing in the proposal, ADR-0005/0006, design section 5.1 or AC-149..AC-151 says where a slice ends or what a 'genuine heading' is, so AC-152 (accept
headings inside prose) and AC-151 (error on a duplicate genuine heading) collide and AC-150's decoys cover neither a deeper heading inside a slice nor a same-level duplicate in prose [measured: 'next
heading', 'same level', 'sentinel', 'end of section' occur 0 times]. `Gwt` is `Option<Gwt>` in six documents and defined in none; S-MIG detected GWT by substring inside the `criterion` cell
(census-s-mig.py:165), no document says whether migration populates `gwt`/`verification` (0 hits), and AC-454's 'both fields absent' property over the 19 affected specs is either false (they hold 208
ACs, 77 with both fields, 29 lacking both) or trivially true [measured, uncapped rerun]. AC-451 names five conservation buckets, SM-134 four, design:1467 five, and the census whose N=6,877 they must
reproduce partitions into seven (Opaque 3,513 / Lifted 3,011 / EmptyTemplate 112 / Unmapped 81 / LiftedWithLoss 73 / Failed 54 / Missing 33) with no mapping for the 33 Missing, 63 section-level
Unmapped or 2 non-plan EmptyTemplate records, so one implementer folds Missing into failed (contradicting AC-453's per-slot counts) and another never balances N [measured, s-mig.jsonl]. AC-475's
'change/diff material' has no locator -- at the pin one .bklg file carries diff hunks and 156 '## Changes' sections that S-MIG's settled opaque list already routes to opaque with no marker -- and its
loss marker presupposes a typed `ChangeSummary` slot S-BIN has not designed; AC-461's 'fix recorded as an enumerated manual intervention' names no carrier for the fixed bytes (a corpus SHA bump is
closed by ADR-0013, AC-464 and the frozen pin, so the carrier must live in the migrate crate) and AC-449/AC-467 do not list it among the determinism inputs; AC-440's typed unsupported-layout error
names no recognition predicate and AC-442's one-crate allowlist no detector [measured].

**Anchors.**
- `references/2026-09-05-architecture-proposal.md:268` — "The one piece of markdown logic that survives is a **section locator** for the opaque prose fields — a heading-anchored slice, not a grammar — and it is tested as such."
- `references/2026-09-08-acceptance-criteria.md:2116` — "GIVEN a document in which the target anchor occurs zero times, or more than once as a genuine heading"
- `references/2026-09-08-architecture-design.md:642` — "| 1 | The AC type is `{ id, criterion: text, gwt: Option<Gwt>, verification: Option<text> }` — permissive by construction |"
- `references/2026-09-08-acceptance-criteria.md:5647` — "write → read → validate with both fields absent; a `jsonschema` conformance test asserting `gwt` and `verification`"
- `references/2026-09-08-acceptance-criteria.md:5610` — "THEN the report accounts for exactly N records as lifted + lifted-with-loss + failed + opaque + dropped-by-decision"
- `references/2026-09-08-acceptance-criteria.md:5919` — "GIVEN historical change/diff material in the corpus and no settled `ChangeSummary` type"

**Why it matters before Step 5.** Step 5 ships `rk-types::AcceptanceCriterion` (RUNBOOK 5.3 lists Spec among its types) and names AC-475 in its acceptance set (RUNBOOK:1717) -- defining `Gwt` is a
small decision the implementer can take and record, so P2 not P1. Step 6's rk-store locator is the one piece of markdown logic in the system and the thing SM-043 measures from first use with no
baseline; the migrate crate's report schema (AC-448) and conservation assert are its first definitions. Each reading builds, but AC-150's byte ranges, AC-454's property, AC-451's identity and AC-475's
fixture cannot be authored identically by two implementers.

**Remediation.** Design section 5.1 row 4 and AC-149 (dated amendment): 'an anchor is a heading at the template's declared level for that section; the slice runs from the line after the anchor to the
next heading at that level or shallower, or end of file; deeper headings inside the slice are prose bytes and not anchors', with an AC-150 deeper-heading decoy and an AC-151
same-level-duplicate-in-prose fixture. Design section 5.2 row 1: define `Gwt { given: Text, when: Text, then: Text, and: Vec<Text> }` (or state that gwt is a presence flag), add to ADR-0075's
amendment note and AC-137 that migration never populates `gwt` or `verification` from criterion text -- both are authored fields from the first CLI write -- and rewrite AC-454 as 'over all 153 specs
at the pin, every AC round-trips byte-identical with both fields absent'. AC-451 and SM-134: enumerate the report's outcome vocabulary (lifted, with-loss, failed, opaque, unmapped-section,
missing-expected, empty-template, dropped-by-decision) with the mapping from census outcomes and per-bucket N at the pin from s-mig.jsonl. AC-475: name the locator (which sections and files count as
change material at the pin, with the measured counts) and its SM-134 bucket (opaque vs with_loss), keeping the tripwire conjunct. AC-461: name the carrier (`interventions.toml` in the migrate crate:
path, line, reason, replacement bytes, digest of the original) and add it to AC-449's determinism inputs and AC-467's versioned lifter set. AC-440: state the predicate (absence of the new-layout
manifest plus presence of `.bklg/`) and allowlist that one detector explicitly in AC-442.

**Folds.** G19-07, G19-08, slice-G20-02, slice-G20-04, slice-G20-09, slice-G03-06

### PIA-040 — rk-git's and rk-store's Step-6 contracts are lists, not rules: unowned git ops (patch-id, merge, common-dir fallback), git-under-replay, target-repo hooks, trailer-affecting config and an undeclared trailer grammar, the answer record, `tree_hash` derivation, ADR-0018 vs AC-518 ownership, F20/2's extended-length path form, `busy_timeout` and transaction bounds, the event vocabulary, raw-payload retention and `Send`/`Sync` -- eighteen behaviours left to the implementer

**Priority:** P2 · **Classification:** SHARPENS-KNOWN ([B01-C11] none for git-under-replay, hooks, the answer record, `tree_hash`, the event vocabulary, retention or `Send`/`Sync` (SM-5.1-046 presumes
an event schema exists). Sharpens ARCH-9.4-002 / ADR-0084-NEG-90 (single writer UNVERIFIED, R-SQLITE owed — no timeout value or transaction bound), ARCH-10.5-031 / ARCH-10.4-030 (handle leak — no
product behaviour for a failed `worktree remove`), RB-GC-004 / HST-TRAP-006 (F20/2 as a rig lesson — no rk-git rule), and the anomaly-pool's dangling ADR-0091 id | [B03-C08] ARCH-5.11-002,
ARCH-3.10-006, ARCH-10.5-006 (S-GIX and the hybrid); ADR-0077-NEG-91 (ADR-0077:92, gix-vs-git config disagreement); FND-LIM-008 (column-anchored, case-sensitive parsing); ARCH-1.2-026 (1 of 101 runs
recorded). Unrecorded: the unowned patch-id and merge operations, the config-dependence of the write command and the S-GIX/b oracle, the undeclared trailer value grammar, the ADR-0018/AC-518 predicate
conflict, and AC-193's side effect on a pure reconcile()) · **Category:** ambiguity · **Confidence:** medium · **Owner:** Step 6 implementer (rk-git, rk-store); Ryan (operator) for the retention and
replay decisions; rk-kernel owner for the activity-side rule

Merged from 2 concerns; each member statement is carried verbatim. (i) Eleven behaviours the Step 6 crates must exhibit are stated nowhere, each verified by grep over the design, register and ADR sets
[measured]. rk-git: replay binds 'every port to its fake' (AC-006/AC-008) but git is a concrete crate, not a port, and nothing says whether tier-1 writes (trailers, empty commits, worktree add/remove,
merge landing) execute, are suppressed or are journalled under replay; `git commit --trailer` runs the target repository's own hooks (a commit-msg hook rejecting a trailer-only message fails every
transition and AC-162 then leaves no store row; a prepare-commit-msg hook can drop the trailer) while the corpus's 'no hooks' rule covers only redkiln-authored hooks (AC-182/AC-412); 'gate answered'
is tier-1 git-first (design 681) yet `Gate:` is written only at open (684), AC-244..264 record the answer as an event, and no trailer key, ref or write mechanics exist for it; a killed run's
still-present worktree has no reuse-or-recreate rule and a failed `worktree remove` no typed error or `doctor` row (`git worktree prune` appears only inside AC-190's fixture); F20/2's extended-length
path defect — `canonicalize()` yields `\\?\D:\...`, git parses it as a URL, 'no code path reached the fetch successfully, ever' — binds no rk-git AC, its fix (`e3::git_remote_path`) is in a file 7c
does not lift, and e3.jsonl already records `review_cwd` in the two forms a byte-equality reading of AC-195 would abort on; AC-245 demands `tree_hash` be 'that same tree object' with a dirty-worktree
property test but no document says whether it is `write-tree` over a temporary index, whether untracked files enter it, or whether `HEAD^{tree}` is used when clean; and Checkpoint 6's hedge (1860)
contradicts header row 23, 1343, 1830 and 2467, which make ADR-0091 unconditional. rk-store: ADR-0084 permits 'a single writer connection or a queued write path' with `busy_timeout` named fourteen
times and never valued, no bound on the longest write transaction another process must wait out for AC-181's 'no lock error', and a queue that lets an F22-class kill drop an event AC-360 says no
dispatch path may fail to append; the event log has no enumerated kind vocabulary, envelope or owning crate (18-20 backticked kinds across four documents; RUNBOOK 5.3's rk-types list omits an Event
type); raw payloads are retained by construction (assistant text, `StepError::Schema.raw`, unrecognised events, argv carrying the 7,655-char card) with no sentence on what is verbatim, digested or
never persisted; and port trait objects' `Send`/`Sync` bounds, which 7a must fix, are unstated. (ii) ADR-0077 assigns four read operations to gix and two write families to the exec wrapper, and its
forbids block covers only worktree add/remove and trailer writes: AC-169's patch-id mechanism and the merge-queue merge (AC-196/197) have no owner, common-dir resolution sits on gix's read side while
S-GIX/a tests exactly that interplay (so RUNBOOK:1813's 'a no-go costs nothing' is wrong for arm /a), and 'shallow clone by SHA for review checkouts' is a listed read while the same paragraph moves
gix away from `gix-worktree-state` checkout [measured]. Both named trailer-write commands honour `trailer.ifexists`, `trailer.where`, `trailer.separators`, `commit.cleanup` and `core.commentChar`;
nothing in ADR-0077, section 5.9, the ACs or RUNBOOK 6.2 pins them, and S-GIX/b's byte-identity oracle is the same config-honouring command over no named fixture [measured: the config keys occur only
inside spikes/fixtures], so a repo with `trailer.ifexists=replace` silently collapses a second `Story:` trailer on tier-1 state. AC-165's 'declared trailer schema' is declared nowhere: the corpus
holds 232 `Story:` values in three forms (201 project/slug, 22 bare RK-S ids, 9 free text), 15 prose-valued `Gate:` trailers and 49 `Fix:`/`Baseline-Repair:`/`Slice-Repair:` keys the design never
names, and the only parser is spikes/runner/src/reconcile.rs, so AC-444's expected drift set is unpinned [measured over D:/repos/redkiln --all]. ADR-0018:38 and its forbids block key write refusal on
the current worktree's HEAD; AC-518, the form that will be tested, keys it on the write's target and permits a run whose cwd is the user's checkout, so `redkiln answer` from a non-run worktree is
refused by one implementer and landed as a trailer commit (AC-162/163) by the other, nothing narrows the ADR, and AC-181's two-process GIVEN does not say the runs are different items. AC-193
attributes stale-clone removal to `reconcile()`, which section 5.6 and AC-174 define as a git-to-store read; AC-427's THEN would be unambiguous if it named the pre-deletion store.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:254` — "WHEN the workflow is replayed against that log with every port bound to its fake"
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:58` — "**gix owns the read path**, at the exact pin recorded in 0084:"
- `references/2026-09-08-acceptance-criteria.md:2440` — "THEN nothing has been written under the repository's `hooks/` directory"
- `RUNBOOK.md:1813` — "taken by the facet-provenance amendment on 2026-09-09 and `0090` by Step 4.4. A **no-go** costs nothing, because"
- `references/2026-09-08-architecture-design.md:684` — "The third axis, `Gate:`, is written when a mark is opened;"
- `references/adr/0077-gix-read-path-only-git-write-path-shells-out.md:68` — "- `git interpret-trailers --in-place` and/or `git commit --trailer` for every trailer write."

**Why it matters before Step 5.** (i) [inferred] None blocks Step 5 and any consistent choice satisfies the criteria as written; each is decided by whoever writes Step 6 first. Several shape public
API a later record cannot cheaply reverse — whether rk-git needs a record/replay seam, the crate that owns the event enum (rk-types, Step 5, if anywhere), the `Send`/`Sync` bounds on `dyn` ports at 7a
— and one (the path form) reproduces on the build platform a defect the corpus already paid for. Cheapest as one record before Step 6. (ii) Not Step 5. Step 6 builds rk-git to 'whatever S-GIX says'
and Checkpoint 6 grep-asserts one shell-out file; without the operation set, the pinned config, the grammar and the ownership predicate, the single wrapper's scope, whether the review clone is bare,
and the parser's AC-165/AC-444 fixtures are guesswork -- on the surface ADR-0077 itself calls 'a state-corruption class, not a bug class'. RUNBOOK 6.1 already plans the successor record where all of
this belongs.

**Remediation.** (i) One record before Step 6 — a successor or companion to ADR-0077/0018/0084 as each rule requires, never an edit — settling: git operations sit on the activity side of the split,
each `reconcile()` result and write outcome journalled and served under replay through a `GitOps` seam (a struct, so AC-483 stands), with AC-006/AC-008 gaining 'and zero rk-git writes'; trailer
commits run with `-c core.hooksPath=<empty dir>` (or `--no-verify` with the prepare-commit-msg residue documented) and AC-162's fault-injection fixture gains a rejecting commit-msg hook and a
rewriting prepare-commit-msg hook; the answer's trailer key and value, the ref it lands on, and the plumbing (`commit-tree` + `update-ref`) used when no run-owned worktree exists, added to §5.4/§6.3.2
and AC §6.1; at entry a worktree registered to the item whose HEAD matches the run's branch is reused, otherwise removed and re-created, and a failed `worktree remove` is a typed error marking the
item needs-operator and a `doctor` row listing orphans, with S-CANCEL-WIN's fixture as its test; every path handed to `git` or as a child `cwd` is the non-verbatim drive-letter form, verified by a
Windows unit test, `git_remote_path` ported into the wrapper under Step 6.2 constraint 4 citing F20/2; the `tree_hash` algorithm (temporary index from worktree state including
untracked-but-not-ignored files, `git write-tree`, same at open and answer) in design 6.3.2 and AC-245's Verified-by; ADR-0091 as 'S-GIX result and git-version floor, supersedes 0077 in part only on a
go', deleting or qualifying the 1860 hedge. For rk-store, with R-SQLITE: the `busy_timeout` value and the rule that every write transaction is shorter than it (rebuild in a separate file then rename),
one writer connection per process behind a mutex and no queue so append is synchronous with dispatch, a long-transaction fixture on AC-181; an event-kind table (kind, tier, payload fields, producing
step) in design §5 or a new ADR and the owning crate named in RUNBOOK 5.3; per dispatch record which fields are verbatim and which digest-only, the child environment never persisted, a `doctor`/lint
row asserting no credential-shaped field in the event schema — or an explicit 'redaction out of scope'; and port trait objects declared `Send` but not required `Sync`, with concurrent items stated
once as separate processes. (ii) In the S-GIX successor record RUNBOOK 6.1 plans (a new ADR; ADR-0077 and ADR-0018 untouched): state the split as a rule -- git executes every mutation of `.git` and
every operation gix does not expose at the pin -- with the current lists as examples; enumerate the wrapper's write set (worktree add/remove, `git add`/`commit --trailer`, empty commits,
`interpret-trailers`, patch-id, merge/fast-forward on the integration ref, clone removal, `write-tree`); state whether the review clone is bare and by which implementation; make S-GIX/a's no-go fall
back to `git rev-parse --git-common-dir` through the wrapper; run every trailer write with trailer-affecting config pinned via `-c`, and run S-GIX/b's oracle under the same pinned config over a named
fixture matrix (existing trailer block, no trailing newline, `---` divider, CRLF, duplicate token) with an AC asserting gix-parsed == git-written per fixture. Add a trailer grammar table to design
section 5.4 or AC section 6.1 (key, value ABNF, column anchoring, verdict vocabulary, unknown-key rule, and 'a `Gate:` value that is not the (token, tree_hash, deadline) triple is legacy drift,
counted but not a transition') and pin AC-444's expected drift counts from it. Add an AC under section 6.4 stating the target-based predicate once (a tier-1 write resolves the run-owned worktree from
the store and commits there regardless of cwd; a write whose target is not run-owned is refused naming path and branch), record in the same successor ADR that ADR-0018's 'current worktree' wording is
narrowed to it, and add 'of two different items' to AC-181's GIVEN. Reword AC-193 so removal is the entry step's clone-lifecycle action and `reconcile()` only reports the stale clone as drift; state
in AC-172 whether `status` reconciles; name the pre-deletion store in AC-427's THEN with a Limit sentence 'not compared to the old redkiln's recorded outcomes -- F12: 1 of 101 recorded'.

**Folds.** L06-buildability-04, slice-G06-03, slice-G14-06, L13-operational-06, L13-operational-02, L04-09, slice-G06-08, L06-buildability-08, L04-11, L13-operational-03, L06-buildability-13,
L06-buildability-05, slice-G06-04, slice-G06-02, slice-G14-05, G13-06, G13-07, slice-G14-08

### PIA-043 — Two Step-6 doctor rules rest on undefined terms: 'tier-1 projection' never places orphans/rewrites (AC-184); AC-159's 'store write record' vs prose

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (AC-0.3-035 @ acceptance-criteria.md:103 (C2: hand-edits inside rendered structured sections; the prose case is not recorded); ARCH-10.6-001,
AC-0.3-004, SM-5.1-019, SM-NOBASE-029 (rebuild is unmeasured; none records the undefined projection)) · **Category:** ambiguity · **Confidence:** medium · **Owner:** AC register author (Claude);
design §5.4/§5.5; Ryan sign-off

(a) AC-184's byte-identical rebuild, SM-051's kill and SM-056's scheduled differential compare a rebuilt store to 'the tier-1 projection of the original store', a term defined nowhere (grep over
design/AC/SM/ADRs: design:725 and AC-184 only). Orphan rows (AC-167/AC-170, 'no row is deleted') and rewrite events (AC-169) are neither tiered nor named as in or out of the projection; under the
reading that includes them, a repository that has ever been rebased diverges by construction because a clean clone cannot contain unreachable anchors, and SM-051's kill fires spuriously. On the pinned
corpus original and rebuilt stores derive from the same clone, so AC-184's CI differential cannot distinguish the readings; the ambiguity surfaces on live repositories (SM-056) [measured]. (b) AC-159
keys on a managed file whose digest differs 'and no store write record accounts for the difference'; ADR-0005 sanctions editor edits of prose sections and AC-152 accepts them 'with no drift record'.
Read with proposal §4.5 rule 3 (reconcile at every entry) the rule is consistent; read without it, `doctor` flags every sanctioned prose edit. AC-159 does not say which, and its out-of-band fixture
does not say which section kind is edited. Its '§4.5 rule 4' evidence tag names the single-write-path rule in its tier-1-commit form (proposal:290); the artifact-level statement is proposal:94
[measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:2462` — "THEN the resulting tier-1 state is byte-identical under the canonical serialiser to the tier-1 projection of the …"
  original store"
- `references/2026-09-08-success-measures.md:417` — "- **Kill:** a non-zero divergence means tier-1 state is not fully in git — the tier-1 test has been violated and the offending field must move or be re-tiered."
- `references/2026-09-08-acceptance-criteria.md:2297` — "THEN those rows are reported as orphaned, with their anchor SHAs …"
  AND no row is deleted or re-anchored as a side effect of the report."
- `references/2026-09-08-acceptance-criteria.md:2081` — "WHEN `doctor` runs and no store write record accounts for the difference"
- `references/2026-09-08-acceptance-criteria.md:2128` — "THEN the edit is accepted as the field's new value with no drift record and no gate"
- `references/adr/0005-human-edits-prose-free-structure-gated.md:37` — ""Humans edit *designated prose sections* of the `.md` directly in an editor or PR; every structured field (ACs, boundary, status, findings) changes only through CLI, web or gate answers and is re-rendered.""

**Why it matters before Step 5.** Step 6, not 5. AC-160 forces a per-field tier annotation and AC-184's CI oracle is in Step 6's acceptance set; two implementers pick opposite answers on orphans
(exclude them and lose AC-171's gate on rebuild, or keep them and fail the rebuild test on the first rebase). For AC-159, implementer A hashes the whole file (every prose edit false-positives in use);
B hashes structured sections only (AC-159 adds nothing over AC-124). Either reading of each passes the stated tests, so nothing is blocked.

**Remediation.** Amend the AC register (not ADR-0018/ADR-0005): define 'tier-1 projection' as tier-1 rows whose anchor is reachable from the clone's refs; state that orphan rows and rewrite events are
tier-2-derived (recomputed by reconcile() at every entry) or excluded from the AC-184/SM-051/SM-056 differential; add an AC-184 fixture with one rebased commit so the expected output is pinned.
Rewrite AC-159's GIVEN to 'a managed artifact whose structured-section or frontmatter digest differs …' or add 'AND a reconcile that accepted a prose-section edit counts as the write record'; retag
the single-write-path evidence at design §5.1 row 3, AC-122 and AC-159 to 'proposal §4.4 line 94 [documented]; the crate-level boundary is this design's own rule'.

**Folds.** slice-G14-02, G19-06

### PIA-044 — Step-3 drift contract: `stale_policy` has no schema and AC-067's vouched-root trigger names no edge set

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (ARCH-0.1-003 (design:21: stale_policy 'BLOCKED (no ADR carries it ...)') records that the setting has no home record, not that its site, domain
and keying class are stated three ways while AC-068 sits in the Phase-1 acceptance set. The AC-067 edge-set half is unrecorded (ADR-0025's NEG/FORB rows and SM-5.1-010 record the unmeasured noise rate
only)) · **Category:** contract-ambiguity · **Confidence:** medium · **Owner:** architecture design (section 4.4) and AC register author

`stale_policy` is 'per cost class' (METHOD-P:10), 'per activity class' (design:606) and 'a pack declaring' (AC-068:930), beside a section 4.4 step-1 policy table keyed on the item class
(`[policy.bug-standard]`); the only value ever named is `gate`, the non-gate value has no name, and no TOML site, enum or schema line exists [measured: grep over references/, RUNBOOK and METHOD-P;
activity-graph-schema.md:189 types cost_class, never stale_policy]. AC-068 is in Step 7b's Phase-1 acceptance set (RUNBOOK:2006), so the gate branch is built, and 'an L-class activity's artifact'
needs a producer `cost_class` that step 4 resolves after step 3 and no artifact record stores -- the two readings change the step order. AC-067's WHEN, 'an upstream artifact it would conceptually
depend on', has no edge set for a vouched root whose `derived_from` is empty: ADR-0025:39 bounds it to the same stage and its Consequences say the human 'is not asked to pre-declare its dependencies',
AC-053 makes human-supplied kinds source nodes with no producer, and AC-067's Verified-by is a negative co-occurrence assertion that cannot discriminate among the three buildable readings; F40:58
measured the most-recent-prior-write reconstruction as the noisier scope.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:930` — "GIVEN an L-class activity's artifact classified `Stale` under a pack declaring `stale_policy = gate`"
- `spikes/METHOD-P.md:10` — "`derived_from` scope; `stale_policy` per cost class"
- `references/2026-09-08-architecture-design.md:606` — "| `stale_policy` per activity class (step 3) |"
- `references/2026-09-08-acceptance-criteria.md:916` — "WHEN an upstream artifact it would conceptually depend on changes"
- `references/adr/0025-vouched-root-advisory-drift.md:39` — "When an upstream artifact in the same stage subsequently changes, the *next* gate the item reaches carries an advisory note"
- `spikes/F40-DRAFT.md:58` — "The reconstruction — "which upstream file changed before this artifact was last written" (§2.2) — is picking the card up as a derivation edge because the card is rewritten so often that it is nearly always the most recent prior write."

**Why it matters before Step 5.** Not Step 5; nothing in rk-types is fenced on it. The step-3 mechanism ships unit-tested in Phase 1 (design:21) and the shape of the setting is what rk-types/rk-kernel
type at Steps 5-7; the edge set decides SM-022's advisory volume and whether the advisory becomes the noise ADR-0025 records as unmeasured.

**Remediation.** Dated amendment to design section 4.4 step 3: the TOML site (`policy.<cost_class>.stale_policy`), the enum (`rematerialise | gate`, default `rematerialise` pending the blocked
decision), and that 'L-class' is the `cost_class` of the artifact kind's registered producer in the requiring stage, resolved before step 3 classifies; mirror in AC-068's GIVEN. Amend AC-067's WHEN to
name the edge set -- 'an artifact of a kind listed in the registered producer's `requires` for the root's kind, in the same stage (ADR-0025), changes digest' -- and add a fixture where a same-stage
artifact outside that set changes and no advisory is emitted. The blocked value stays blocked; only the type and the edge set are fixed, so ADR-0025 is untouched.

**Folds.** slice-G02-05, slice-G02-06

### PIA-046 — Selection-area terms never defined: surfaces' source token, AC-070's scope, selector specificity, cross-pack order, lattice rigor mode, discouraged

**Priority:** P2 · **Classification:** NEW — surfaces/specificity/discouraged: none (ARCH-8.10-003, ARCH-10.3-002, EB-E-005 record `surfaces` as authorable-but-unscored; ADR-0014-NEG-47,
ADR-0022-NEG-48/FORB-53, ADR-0029-NEG-63 restate equal-specificity-is-an-error; ADR-0020:52 notes only that `discouraged` has no numeric counterpart). AC-070's parenthetical: SHARPENS AC:1079-1084 and
AC:1288 (the drift was corrected once; the corrected list still names four of eleven). Lattice mode: SHARPENS ARCH-4.9-009 (design:611, optional-override 'not a real reading') and ARCH-10.5-032 ·
**Category:** contract-ambiguity · **Confidence:** high · **Owner:** Ryan Britton (selection area owner; METHOD-P mode decision); Claude for the register and RUNBOOK edits

Five terms the selection area's Phase-1 code keys on are named and never defined. The `[facet.surfaces]` block's `source` value is written nowhere: Checkpoint 4a's 'both are 11' presupposes
`source = "derived"`, under which AC-520 fails load until S-FACET scores it, contradicting design:1802's 'may be authored and linted'; the DECLARED class change-classification puts it in has no
registry token [measured]. AC-070's GIVEN, corrected once already against the registry, still parenthesises four of the eleven derived facets. 'Equal specificity' (AC-087) has no definition beyond
'toolchain match beats generic' -- key-count and subsumption give opposite verdicts on `{toolchain=node, stage=verify}` against `{kind=bug}` -- and AC-084's 'declaration order' across two packs needs
a pack order nothing defines, load order being the one AC-087's property test forbids. SM-026/AC-101 enumerate rigor as an axis of the lattice instrument without pinning the mode: run9 reads 0.3%
(432/155,520) at pack-default and 5.7% (35,676/622,080) summed over the four rigor modes -- opposite verdicts against the 5% kill on the same TOML [measured, results-p6-run9.txt:53-56]; F35 pins the
headline reading, the criteria do not. `discouraged` has no stated intent anywhere; under the stated admissibility rule (min_rigor <= stage rigor, catalog minimum `optional`) nothing is admissible
there, so a stage run at that level with a non-empty baseline gates as Unsatisfiable [measured: no pack uses it, the lattice enumerates three live levels].

**Anchors.**
- `RUNBOOK.md:1561` — "# → the two numbers are EQUAL, and both are 11. Every derived facet"
- `references/2026-09-08-architecture-design.md:1802` — "which may be authored and linted and may **not** key a `require` row until **S-FACET** scores it"
- `references/2026-09-08-acceptance-criteria.md:1063` — "GIVEN a facet declared `source = "derived"` in `facets.toml` (`toolchain`, `change`, `auth`, `data_migration`)"
- `references/2026-09-08-acceptance-criteria.md:1360` — "GIVEN two candidate cards fulfilling the same role whose selectors are of **equal specificity** for the item at hand"
- `references/2026-09-08-success-measures.md:234` — "- **Instrument:** lattice output, `unsatisfiable_cells / total_cells` over the facet-vector × rigor cross-product."
- `spikes/p9/results-p6-run9.txt:53` — "TOTAL prereg-8 optional      unsat=33084/155520 (21.3%)   all-12 unsat=36540/235872 (15.5%)"

**Why it matters before Step 5.** Step 5 ships `Rigor` with five variants (AC-050) but the semantics do not change its shape. The registry line is a Step 4a authoring choice -- two implementers
produce two registries, one failing Checkpoint 4a and the other unable to load a pack -- and the binder and lattice CI job are Step 7b: two binders disagree on which configurations are errors, and the
lattice job either fails on its first run or sits 16-25x inside its line depending on a choice the criteria do not make.

**Remediation.** One dated amendment to design sections 4.3/4.4 with matching AC edits: write the `[facet.surfaces]` block verbatim in RUNBOOK 4a.2 (`source`, `deriver`, `set = true`, `values`),
register its `source` token in AC-072's shape list, state whether AC-520 gates it, then fix the Checkpoint 4a count; replace AC-070's parenthetical with 'every facet the registry declares derived,
eleven at pin 47faec9f'; define specificity in section 4.3 rule 1 (recommend subsumption over the closed selector key set, incomparable = AmbiguousRole) and cross-pack order in rule 3 (enabled-pack
list in config, then within-pack declaration order), reflected in AC-084/AC-087's Verified-by; pin AC-101 and SM-026 to the pack-default reading over the enabled pack set, with item-level overrides
enumerated and reported but not thresholded, and record pack count and rigor mode with every quoted rate as SM-027 already requires for pack count; define `discouraged` in section 4.4 step 1 (baseline
runs, `require` rows fire, run record carries an advisory) or mark it reserved with a lint that rejects it in packs until defined. The METHOD-P keep/strike decision on the override mode stays with its
registered owner.

**Folds.** slice-G01-04, slice-G01-08, slice-G18-11, slice-G18-04, slice-G18-09

### PIA-049 — Verification gate underspecified: AC-334's mutation derivation, the AC-326/329 comparator bootstrap, quarantined survivors' exit route

**Priority:** P2 · **Classification:** NEW — The derivation gap is ALREADY-RECORDED at register:4449 (verify-gate residual list, 'entirely unspecified') — not in known-issues.json, which captured only
line 4445 as AC-PROV-029 (a census gap); it is folded here for its mis-prioritisation, not as a discovery. The bootstrap and exit-route gaps are NEW (ARCH-6.6-016 records the analogous p95 bootstrap
for AC-330, which the corpus does state; SM-NOBASE-055 records SM-100's routing to AC-357 without the mismatch) · **Category:** ambiguity · **Confidence:** high · **Owner:** Ryan (derivation and
override policy — design decisions); AC register author (Claude) for the criteria

The register records at 4449 that how a mutation is derived from an AC's THEN clause is 'entirely unspecified' and that a model answer collides with AC-347's zero-dispatch rule, yet ADR-0019 makes
that mechanism the product's evidence norm, AC-317/AC-334/AC-340/AC-446/AC-469 name it as their instrument, and the RUNBOOK asserts the norm 'stays in force for every class' (1723) while listing four
decisions (corpus pin, ADR-0077 hybrid, gix version, doctor's window N) of which it is none [measured] — a norm-level mechanism recorded as a residual-risk bullet, not a §0.1 BLOCKED row or a runbook
decision. AC-317 supplies two operators ('removed, and inverted'); the open part is identifying the mutation from a THEN clause. AC-326 reclassifies an `affected` run whose count meets 'the most
recent recorded whole-suite count for that toolchain'; AC-329 makes `total_test_count` required ('validation fails if any is absent'); AC-327's probe yields a fixture's count, not the repo's; a
filtered run does not report the whole-suite total, so on the first run per toolchain (or after a version bump) no comparator exists. AC-330 states its bootstrap explicitly; AC-326/AC-329 do not
[measured], and one natural reading — missing comparator means no reclassification — is the fail-open shape AC-322 exists to forbid. AC-345 says quarantine 'stops the finding from blocking' while the
AC stays `unpinned`; AC-346 refuses exit for an AC 'neither pinned by a flip-verified assertion nor answered at a human gate'; AC-357's human gate is GIVEN only 'an AC whose toolchain declares
mutation infeasible'. For a feasible-mutation AC whose survivor is quarantined every path is closed, and SM-100 routes 'the residual' to AC-357's gate, which its GIVEN does not admit [measured].

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4449` — "AC-334's mutation-derivation step ('the mutation derived from that AC's THEN clause') is the load-bearing mechanism of section C and is left entirely unspecified. If the answer is a model, it collides with AC-347's zero-dispatch rule."
- `references/2026-09-08-acceptance-criteria.md:4147` — "WHEN the mutation derived from that AC's THEN clause is applied to the candidate tree and the named suite is re-run"
- `references/2026-09-08-acceptance-criteria.md:4064` — "whole-suite count for that toolchain"
- `references/2026-09-08-acceptance-criteria.md:4095` — "THEN it carries `basis`, `basis_reason`, `argv`, `selected_test_count`, `total_test_count`, `wall_secs`,"
- `references/2026-09-08-acceptance-criteria.md:4258` — "THEN quarantine stops the finding from blocking and does not change the AC's evidence class or `pinned` status"
- `references/2026-09-08-acceptance-criteria.md:4267` — "THEN the exit is refused and the unpinned AC ids are named in the refusal"

**Why it matters before Step 5.** [inferred] Not Step 5. A builder reaching the verify gate must invent the derivation (author-supplied mutant, an operator set, or a model — the last forbidden by
AC-347); the first affected run on every toolchain is undefined and one reading fails open on the 26-of-68 defect §10.2 defends against; and whether a human can override a surviving mutant at all is a
product decision the text takes both ways, so two implementers build the override differently. None blocks a crate; each is a decision Ryan owns and the register should carry.

**Remediation.** Promote the derivation to a recorded decision: a new ADR (not an amendment to ADR-0019) fixing it as author-declared per-AC mutants — a `mutation:` field on the AC row,
`not-feasible(reason)` per AC-337 — so the instrument is deterministic and zero-dispatch, listed as a fifth runbook decision. Add a bootstrap clause to AC-326 (or a sibling criterion): with no
recorded whole-suite count at the current version pin, an `affected` claim is reclassified `whole-suite / probe-unverified` (or the run is policy-forced whole-suite) and the resulting count seeds the
comparator; state in AC-329 where `total_test_count` comes from on an `affected` run. Either extend AC-357's GIVEN to 'an AC recorded mutation-survived whose finding is quarantined', so the
quarantine's `by`/`until` doubles as the human-gate answer AC-346 accepts, or state in AC-346 that a quarantined survivor still blocks exit and mark quarantine review-gate-only in AC-345 and SM-095.

**Folds.** L07-testability-03, G16-04, G16-05

### PIA-050 — Gate semantics: loop-back re-reach has no AC, escalate chains are unbounded, the tree hash of an empty commit over a dirty worktree is undefined

**Priority:** P2 · **Classification:** NEW — escalation class and dirty-worktree tree: none (ARCH-6.6-003, ADR-0017-FORB-50, ADR-0087-FORB-72, F33-LIM-019 record the terminal-path rule; ARCH-1.2-043
is corpus provenance). Re-reach half: SHARPENS ARCH-6.6-009 (design:1046), which records not-answerable status but not that AC-271/SM-078 measure a different question from ADR-0087 clause 4's, nor
that loop-back re-reach has no AC · **Category:** contract-ambiguity · **Confidence:** medium · **Owner:** Claude (AC/design/SM edits); Ryan Britton for the provisional re-reach choice and AC-276's
escalation terminal action

ADR-0087 clause 4 and METHOD-P section 5.4(3) leave open whether a gate step re-reached by graph loop-back mints a new mark; design:1046 says AC-271 pins 'the chosen semantics' and SM-078 is 'the only
route to the answer', but AC-271's GIVEN is two answers to one open gate and SM-078 counts answer submissions per `gate_opened` -- neither observes a re-reached step -- while schema:262 decides 'a new
mark' and design:901's 'no second mark minted' reads the other way [measured: grep for re-reach/loop-back/rearm in the AC register returns nothing]. AC-259 makes an expired `escalate` gate open a
second kernel-owned `Gate:` mark with its own deadline and (AC-260/AC-276) its own per-class expiry action; no text gives the escalation gate a class or bounds the chain, so AC-256's 'no gate both
past-deadline and open' holds at every step of an escalate-to-escalate chain -- ADR-0017's forbidden hang in slow motion -- and AC-276's pending human choice is per existing class, not for a class
that is undefined. Opening a gate on an empty commit over a dirty worktree needs a tree object that AC-245 says is neither HEAD's tree nor a later index, and no document says how it is derived
(untracked in or out; `write-tree` on a temporary index is absent from design:744's write list; fingerprint.rs only digests git status), while AC-254's 'commits ... between the recorded tree hash and
the current one' is undefined when the recorded tree is no commit's tree [measured].

**Anchors.**
- `references/2026-09-07-activity-graph-schema.md:262` — "Decided pending P8: a gate re-reached via loop-back is a new mark and a new `HumanAnswer`."
- `references/2026-09-08-architecture-design.md:1046` — "**Second-arrival semantics** (does a re-reached gate mint a new mark?) | Not answerable by any spike run — P8 §5.4(3) circular by construction | ADR-0087 clause 4; SM-078"
- `references/2026-09-08-acceptance-criteria.md:3305` — "THEN the escalation is itself a kernel-owned `Gate:` mark with its own (token, tree_hash, deadline) — not an in-activity"
- `references/adr/0017-gate-record-is-token-tree-hash-deadline.md:51` — "- Forbids a gate with no terminal path — an expiry must resolve to fail, escalate, or auto-decide, never leave the gate hanging."
- `references/2026-09-08-acceptance-criteria.md:3153` — "THEN the recorded `tree_hash` is the hash of that same tree object, not of HEAD's commit, not of the index at a later"
- `references/2026-09-08-acceptance-criteria.md:3259` — "THEN it names the commits and the changed paths between the recorded tree hash and the current one, and is byte-identical"

**Why it matters before Step 5.** Phase-1 kernel behaviour, not Step 5: P9 catalog graphs loop into gates, the expiry state machine is AC-256's exhaustive test, and the drift predicate (AC-249/AC-253)
decides the void rate SM-075 tracks -- two readings give different void rates on the same repository. Re-reach has a schema-declared answer an implementer can follow, so nothing is blocked; the other
two are one-sentence definitions.

**Remediation.** Add a new AC outside AC-240..279 (exhausted) pinning loop-back re-reach as a declared PROVISIONAL choice consistent with schema:262 (fresh (token, tree_hash, deadline) on re-reach,
per ADR-0087 clause 4 until P8-B answers), correct design sections 6.3.6/6.6 and SM-078's 'why tracked' to describe AC-271/SM-078 as duplicate-answer instruments, and add a discovery instrument
counting `gate_opened` per (item, gate step). Add one sentence to design section 6.3.4 and a clause to AC-259: an escalation gate's class is fixed (`escalation`) and the policy-table lint (AC-260)
refuses `escalate` as that class's expiry action, so every chain terminates in fail or auto-decide within one hop, with a must-fail fixture row. Add to design section 6.3.2 and AC-245's THEN: the
recorded tree is the trailer commit's tree when a checkpoint is due, otherwise a tree written from the current index plus tracked modifications via `git write-tree` on a temporary index through the
wrapper (state whether untracked files are included; add the operation to the write-side list at 744); restate AC-254 as 'commits since the trailer commit, and paths differing between the two trees'.

**Folds.** G08-03, G08-04, G08-05

### PIA-052 — `null` in `require` predicates: guard narrowing undefined, `null == <non-null>` undefined, and three outcomes asserted for one row without an order

**Priority:** P2 · **Classification:** NEW — none (ADR-0032-MARK-46 records the 0032/0069 refinement lineage; ARCH-8.10-003 and ARCH-10.3-002 record that null routes to AC-079; no row records the
guard question, the equality value or the lint/Unsatisfiable/unknown_facet ordering) · **Category:** grammar-semantics · **Confidence:** high · **Owner:** predicate/lint workstream (rk-kernel lint,
rk-select); Ryan Britton as selection-area owner for the superseding record

ADR-0069's Negative clause tells authors to guard ordering with an equality/`!= null` check first, while its Forbids block rejects any ordering on an operand that can be null, AC-069 says 'equality
only', and the P9 lint (spikes/p9/lint.py:328) rejects every ordering next to a `?` ref, guard or not -- no text says whether a guard narrows [measured]. ADR-0069:48-49 declines to state the value of
`null == <non-null>` while ADR-0069:55's 'identity meaning everywhere' implies `false`, under which `when auth == true` silently skips on null -- the outcome ADR-0089's forbids block (215-216) exists
to prevent. Design:507, ADR-0089:123-126 and AC-079 assert three outcomes for a `require` row reading a null facet -- lint error for not 'handling null explicitly', runtime `Unsatisfiable`,
`gate_opened{unknown_facet}` -- in adjacent sentences without ordering them, and RUNBOOK:1510 names the AC-054 step-4 outcome where AC-079:1036 routes at step 2. The catalog carries 198 ordering
comparisons on the four unknown-capable ordinals and no guards [measured].

**Anchors.**
- `references/adr/0069-null-ordering-operators-are-lint-errors.md:62` — "set") must explicitly guard with an equality/`!= null` check first; the grammar gives no shorthand for "ordering that"
- `references/adr/0069-null-ordering-operators-are-lint-errors.md:66` — "- Forbids any predicate that applies `<`, `<=`, `>`, or `>=` to an operand that can be `null` — lint must reject it,"
- `references/2026-09-08-acceptance-criteria.md:1017` — "THEN load fails unless the row's predicate handles `unknown` explicitly (equality only), so `unknown` is never ordered"
- `references/adr/0069-null-ordering-operators-are-lint-errors.md:48` — "The only equality value the sources fix is §9a.2's `null == null`; neither §9a.2 nor amendment 39 states the result of"
- `references/2026-09-08-architecture-design.md:507` — "so a `require` row whose predicate cannot be evaluated over a `null` yields `Unsatisfiable` and escalates to the human gate rather than resolving silently. Every `require` row over a boolean facet must now handle `null` explicitly, and one that does not is a l"
- `RUNBOOK.md:1510` — "a `require` row whose predicate cannot be evaluated over a `null` yields `Unsatisfiable(kind, failed_predicates)`"

**Why it matters before Step 5.** Not Step 5, and no workstream is fenced: AC-069 and AC-079 are P1-blocked and excluded from Step 7b's acceptance set (RUNBOOK:2006), and Phase 1 forbids every
boolean-keyed `require` row, so only the AC-079 fixture exercises the rule. The semantics must be settled by record before rk-kernel's predicate evaluator is written; two implementers reading only
this text build one that gates every item on every abstaining facet and one that resolves absence of evidence to not-auth.

**Remediation.** One superseding record for ADR-0069 (never an edit) that (a) defines `null == <non-null>` (recommend: evaluates to `null`, and a top-level `null` predicate is the abstention case),
(b) states the narrowing rule as a syntactic pattern the lint recognises (`x != null and <ordering on x>` at the same conjunction level, no `or`, no `not`) or states that no guard narrows, and (c)
orders the three outcomes: lint at load for an unguarded ordering, `gate_opened{unknown_facet, axis}` at step 2 for a null read, `Unsatisfiable` reserved for step 4. Amend AC-069's and AC-079's
Verified-by to name the pattern and count gates per read facet, not per abstaining facet; reconcile RUNBOOK 4a.3 (1510) to AC-079's reason. Cite ADR-0089's forbids block as the fixed outcome (gate,
never skip).

**Folds.** L04-01, slice-G01-01

### PIA-053 — Predicate grammar for the 7b parser has no precedence/associativity, no mixed-unparenthesised rule and no literal-versus-field rule

**Priority:** P2 · **Classification:** SHARPENS-KNOWN (F33-LIM-013 (spikes/F33-DRAFT.md:62: parentheses missing) and F33-LIM-014; AC-15.3-P-003 (register:6526: the seventeen grammar ADRs 'have not
been audited'). The absence of a precedence rule and of a literal-vs-field rule is not recorded) · **Category:** ambiguity · **Confidence:** high · **Owner:** predicate/lint workstream (rk-catalog,
Step 7b)

Across ADR-0033/0048/0049/0070 and schema 9a.3/9b.18 no precedence, associativity or binding strength is given for `not`, `and`, `or`, comparison and `in` [measured:
precedence/associativ/binds-tighter match only ADR-0048:21, which mentions precedence only to say parentheses avoid 'inventing precedence rules by convention']; no record declares an unparenthesised
`a and b or c` legal or a lint error. The P9 lint is a vocabulary checker, not a parser (lint.py:327-340), so no de facto answer exists; the v0 catalog contains no unparenthesised mixed connective
[measured]. Nor does any grammar record state how a bare identifier in value position (`rigor >= recommended`, `kind == bug`) is told from a field (`owner != null`); the corpus resolves it by
vocabulary (AC-065's 'registered facet value' clause and lint.py's registry lookup), which is total only while facet names and values stay disjoint — they do at this pin. ADR-0033:35 writes the
quantifier receiver-less (`none/all/any(<field> in [...])`) while every worked example uses `<list>.quant(...)` (ADR-0049:31).

**Anchors.**
- `references/adr/0048-predicate-grammar-gains-parens-and-quoted-facet-values.md:21` — "grouping is needed for readable `and`/`or` compositions without inventing precedence rules by convention."
- `references/adr/0033-normative-predicate-grammar.md:35` — "operators `== != < <= > >= in`; boolean connectives `and or not`; literals are string/number/bool/`null`. No user functions."
- `references/adr/0049-quantifier-bodies-admit-full-comparisons.md:31` — "Quantifier bodies take full comparisons: `all(owner != null and due != null)`, not membership only."

**Why it matters before Step 5.** [inferred] The parser is a Step-7b deliverable (ADR-0070 replaced the regex with a parser); a grammar with no precedence cannot be implemented identically twice, and
the catalog gives two implementers nothing to disagree about today, so the choice is silent once made. Nothing waits on it; a lint-error-on-mixed-connectives choice is cheap and reversible.

**Remediation.** One amending ADR (a successor, not an edit to 0033/0048) fixing precedence (`not` > comparison/`in` > `and` > `or`, left-associative) or declaring mixed unparenthesised connectives a
lint error; stating 'an unquoted identifier in value position (RHS of a comparison or member of an `in` list) is a value literal; a field or root is only ever in operand position; field-to-field
comparison requires `${...}` refs'; stating the quantifier as `<list-expr>.quant(<body>)`; and fixing the string-literal quote set and escaping in the same record.

**Folds.** L04-03, L04-05

### PIA-060 — Claude adapter contract: no sibling denylist, no inherit-vs-allowlist rule, a Verified-by reading as forbidden set_var, no rule for a tools:-less card

**Priority:** P2 · **Classification:** NEW — none (AC-0.3-015 at AC:78 records only the AC-217/AC-488/AC-422 overlap; ADR-0010-FORB-44 restates the single-key assertion; ADR-0083-FORB-107 records the
set_var ban; no row records the unenumerated siblings, the inherit/allowlist question, the instrument ambiguity or the no-`tools:` card) · **Category:** contract-ambiguity · **Confidence:** medium ·
**Owner:** Step 7c implementer (rk-adapter-claude); AC register maintainer

AC-217 asserts 'no API key is present in the child environment (`ANTHROPIC_API_KEY` and siblings unset)' but the corpus names exactly one key; every other 'sibling' is AC-520's derived-facet gate,
AC-133's rationale sibling, AC-514's receipt siblings or fan-out stagger [measured]. Design:1726 and RUNBOOK:2044 require the child environment to be 'constructed explicitly' without saying
inherited-minus-denylist or allowlist; the lifted runner spawns with the parent environment untouched (runner.rs:267-273, no `env_clear`/`env_remove`) and gates only on the parent's own variable
(hygiene.rs:52). AC-217's and AC-488's 'Verified by' ('spawns with a key set in the parent'; 'a unit test with the variable set') read naturally as `std::env::set_var`, which ADR-0083:76/:108 forbids
in tests and its lint catches; neither AC says whether the adapter reads the process environment or receives a captured `Config` map, so the instrument is under-specified rather than contradictory.
Separately, no criterion or doctor row requires a card to declare `tools:` -- AC-493 rejects an empty or absent list only per capability mapping -- and sections 3.6.1/8.4 do not say that AC-205's
pre-spawn 'effective tool set' is the resolved declaration, the only thing available before spawn.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:2823` — "THEN the adapter asserts no API key is present in the child environment (`ANTHROPIC_API_KEY` and siblings unset), and a"
- `references/2026-09-08-architecture-design.md:1726` — "Constructing the *child's* environment explicitly — including `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` in the runner env (ADR-0078) — is the opposite behaviour and is required"
- `spikes/runner/src/runner.rs:267` — "let spawned = tokio::process::Command::new(&self.claude_bin)"
- `references/2026-09-08-acceptance-criteria.md:2825` — "*Verified by:* unit test that spawns with a key set in the parent and asserts refusal; an assertion on the constructed"
- `references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:108` — "- Forbids `std::env::set_var` and `std::env::remove_var` anywhere in the workspace, including tests."
- `references/2026-09-08-architecture-design.md:1352` — "| A capability mapping resolving to an empty or absent tool list | error | AC-493 | **fails open** — Claude Code inherits every tool when `tools:` is absent [documented, shipmates L4.4]."

**Why it matters before Step 5.** Step 7c (rk-adapter-claude) and Step 5.2 CI (the set_var lint), not Step 5.1. The subscription session is resolved by the `claude` binary from the operator's profile,
so a fully clean environment cannot authenticate and a fully inherited one passes every base-URL/auth-token/vendor-routing variable through: the denylist is the whole content of AC-217's assertion and
it is unwritten, and the first test written to the AC's letter fails the workspace lint.

**Remediation.** Amend AC-217 and AC-488 (AC edits; ADR-0010 and ADR-0083 untouched): enumerate the denied names as a committed list in `rk-adapter-claude`; decide inherit-minus-denylist versus
allowlist in the same sentence; rewrite both 'Verified by' lines to 'a unit test constructing a `Config` whose captured environment carries the key, asserting the typed error and dispatch count 0',
with 'the adapter never reads the process environment; it receives the captured map as a value'; have the `doctor` row (AC-422) and the construction test read the same list. Add a doctor rule row that
a card with no `tools:` key is a configuration error, citing shipmates L4.4 as the documented fail-open direction, and state in section 3.6.1 that the pre-spawn effective tool set is the card's
declared list and nothing else.

**Folds.** L13-operational-05, G05-08, slice-G04-06

### PIA-061 — Headroom contract: batch delta undefined across a window reset and shared-account fan-out, SM-111's n>=10 floor in no AC, criterion (7) cites SM-109

**Priority:** P2 · **Classification:** NEW — none (ARCH-10.4-035 records resets_at semantics at the throttle margin for S-RETRY; HST-UNRES-001 the shared account; neither records the delta's reset,
attribution or n-floor gaps; no row or anomaly_pool entry records the SM-109/SM-111 mis-citation on criterion (7)) · **Category:** contract-ambiguity · **Confidence:** high · **Owner:** Claude (AC
register, RUNBOOK Step 8, design section 7)

AC-372/373/395 define headroom as a batch delta between two `rate_limit_event` readings over an explicitly named set and require `redkiln cost <item>` to print one per item, but no text says what the
delta is when `five_hour` resets between the readings: F29 silently split by `resetsAt` (three full windows of 20/21/15 plus a partial of 4 draws [measured, s-var.jsonl]) while the lifted `batch_draw`
signs last minus first and flags `delta < 0.02` as resolution_limited, so a reset-crossing batch yields a negative delta silently flagged [measured, throttle.rs:150,159]. Under section 7.7's K
parallel worktrees on one shared subscription the reading pair measures the account's whole draw, so an item-level delta attributes other items' and the human's traffic to that item; SM-111's
>=10-dispatch reporting floor appears in no AC (only AC-382's `low_sample` for cache share) [measured]. Phase-1 exit criterion (7) at RUNBOOK:2204-2205 and design:1790 attaches `resolution_limited` to
'per-activity cost' -- a per-activity headroom field AC-373 makes a compile error -- and cites SM-109, whose body never mentions the flag; SM-111 owns it, design:114 cites both, and phase1-deps.json
verified the dependency by `grep -c 'SM-109'`, so the mis-citation propagated into the dependency census [measured]. AC-395 shows the intended reading (cost rows per activity plus one batch delta with
the flag), so the criterion is decidable but its wording invites the forbidden reading.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:4615` — "GIVEN a headroom delta computed between two `rate_limit_event` readings"
- `spikes/runner/src/throttle.rs:150` — "let delta = last - first;"
- `references/2026-09-08-success-measures.md:829` — "- **Sampling:** one reading per dispatch, aggregated and reported only in batches of **≥10 dispatches**"
- `RUNBOOK.md:2204` — "**(7) Per-activity cost is recorded and carries `resolution_limited`** wherever the 0.01 quantisation binds"
- `RUNBOOK.md:2205` — "(SM-109). `utilization` is quantised to 0.01, so per-run cost is **not measurable**; batch-delta is the only honest"
- `references/2026-09-08-acceptance-criteria.md:4633` — "AND a per-run headroom field on any event or projection is a compile error."

**Why it matters before Step 5.** Step 7c (position 13) and the Step 8 exit gate, not Step 5. The naive instrument (sign-agnostic, reset-blind, item-scoped) reports wrong headroom under exactly the
fan-out and 16-hour batches the design prescribes -- the confident fiction AC-375 exists to prevent -- and an exit criterion two reviewers read differently cannot gate the checkpoint.

**Remediation.** Amend AC-372 to define the delta over readings sharing one `resetsAt` (a batch that crosses a reset is split per window and the partial window reported with its own n, as F29 did);
state that the named set is the account's complete dispatch sequence between the two readings, so `redkiln cost <item>` prints a per-item headroom delta only when the item's dispatches were the only
traffic and otherwise prints 'not attributable' with the interleaving count; lift SM-111's n>=10 floor into AC-372/AC-395 as a `low_sample` refusal mirroring AC-382. Rewrite criterion (7): 'every
dispatch event carries a cost row (AC-394); headroom is reported only as a batch delta with n and `resolution_limited` (AC-372/373/395; SM-111), and `redkiln cost` renders the flag'; replace SM-109
with SM-111 at RUNBOOK:2205 and design:1790 and re-run the phase1-deps check against SM-111.

**Folds.** slice-G17-02, slice-G17-05

## 5. P3 — hygiene and convention

Eleven concerns. None blocks pen-to-paper or a Phase-1 workstream; each is a defect in the record that a reader following it reaches before an implementer does. Every quote below is carried verbatim
from the finding records; every anchor was re-read by the internal audit and, where the source is external, re-fetched live on 2026-09-10.

### PIA-100 — Names the governing record cannot lex or does not forbid: bare-capitalised predicate vs ADR-0048, ADR-0030 hyphenated facets, AC-280 `approved`

**Priority:** P3 · **Classification:** SHARPENS-KNOWN (F33-LIM-004 (brace vs bracket) and F33-LIM-014 (F33:63, 'facet values bare everywhere, step-output strings quoted') record the grammar split;
AC:3985's residual and AC-15.3-001 record that AC-280 is a name check citing ADR-0004; the ADR-0030 rows concern facets-not-tags -- none records the design/ADR-0080 spelling, the defining ADR's
hyphens, or the approve/approved gap) · **Category:** inaccuracy · **Confidence:** high · **Owner:** ADR author (Ryan); design maintainer and AC register author (Claude)

Names written in one document do not match the name the governing record fixes. [measured] The design's most-cited predicate, `findings.none(evidence in [Reproduced, MutationSurvived])`, appears
bare-capitalised ten times (design x7, RUNBOOK x2, ADR-0080:80's decision), single-quoted once (ADR-0080:33's evidence), with braces in the schema, and other literals double-quoted (RUNBOOK:1486,
ADR-0089 x2); `Reproduced` fails ADR-0048:32's bare pattern `[a-z_][a-z0-9_]*`, ADR-0048 speaks only of facet values, and which quote character is legal for step-output enum values is unstated -- so
ADR-0080 contradicts itself between evidence and decision, though the catalog the lint loads spells it quoted and the design disclaims grammar ownership, so nothing built from normative sources
rejects anything. ADR-0030:38 (the record that fixes the initial facet set) and ADR-0060 spell three facet names `data-migration`, `external-dep`, `market-facing` -- the PC8-era hyphenation the
identifier rule postdates -- where facets.toml and 58/40/12 live occurrences use underscores and a hyphen cannot lex as a bare identifier. AC-280:3556's denylist spells `approved` where ADR-0074:82
forbids `approve`; AC-280 is 'a name check, not a semantic one', so a card declaring `approve` passes at configuration time while violating the in-force ADR (AC-280 cites ADR-0004, which has no
denylist).

**Anchors.**
- `references/adr/0048-predicate-grammar-gains-parens-and-quoted-facet-values.md:32` — "Facet values are written bare when they match `[a-z_][a-z0-9_]*`, and quoted otherwise (e.g. `'security-vuln'`)."
- `references/adr/0080-quality-block-policy-predicate-is-not-a-terminal-condition.md:80` — "**2. The predicate `findings.none(evidence in [Reproduced, MutationSurvived])` is forbidden as a quality block's terminal"
- `references/adr/0030-facets-not-tags.md:38` — "the boolean risk facets `auth`, `data-migration`, `external-dep`, `ui`, `market-facing`"
- `spikes/p9/facets.toml:64` — "[facet.data_migration]"
- `references/2026-09-08-acceptance-criteria.md:3556` — "(`pass`, `verdict`, `approved`, `ok`, `status`, `decision`) is refused as a configuration error, before any dispatch."
- `references/adr/0074-review-gate-reviewer-proposes-machine-confirms.md:82` — "- Forbids any `pass`, `verdict` or `approve` field on the review card's output schema, and forbids the policy reading one if a card emits it anyway."

**Why it matters before Step 5.** Hygiene: the registry and catalog are unambiguous and no Step-5 parser is built from the design's examples. AC-280 is a Step 5.3 rk-types test, so its list and
fixture are the one edit with build value.

**Remediation.** In the next amending ADR (never edit 0030/0048/0074/0080): extend the literal rule to all string values, fix one quote character and an escape, and state that facet names are
`[a-z_][a-z0-9_]*` with 0030's hyphenated spellings reading as the underscore names; correct the ten bare-capitalised occurrences in the design and RUNBOOK and note ADR-0080's internal drift in the
amending record; amend AC-280's denylist to include `approve`, `approved` and `approval`, cite ADR-0074, and make the fixture card carry `approve`.

**Folds.** L04-04, L04-14, G15-14

### PIA-101 — Research-folder citation hygiene: dead links, statuses already false at authoring, a mislabelled GitHub rate limit, six numbers absent from their cited sources, three superseded pins, four paraphrases in quotation marks and twenty tag/venue/characterisation drifts -- none re-cited by a live document for the drifted claim

**Priority:** P3 · **Classification:** NEW · **Category:** external-grounding/competitors · **Confidence:** high · **Owner:** Research dossier owner

Merged from 3 concerns; each member statement is carried verbatim. (i) None of these changes a live decision; each is a record defect in a document the live set cites as history or evidence
[documented]. (a) evidence-augmentation :34 quotes the vendor's compaction guidance as 'nuanced understanding that took an entire session to build' under a [1P] tag; the phrase is on no first-party
page (six raw pages grepped) and only in third-party blogs; the substantive point at architecture-design :76 is carried by the page's 'intermediate reasoning are gone'. (b) github-platform :73 and
verification :9 call 2,000 points/min the GraphQL 'primary limit'; it is a secondary per-endpoint limit and the primary is 5,000 points/hour per user - a 24x error for anyone capacity-planning a
GitHub adapter. (c) competitive-workflows :42 'custom types still not GA as of the docs read' was false on the read date: GA in GitLab 19.1 (2026-06-18). (d) github-platform :43 'GitHub doesn't
publish one number' is false (50,000 items) and ~1,200 has been obsolete since Feb 2024. (e) Dead at write time: grounding-s-results :296 (beads docs/ADVANCED.md, removed 2026-07-12) and
architecture-proposal :425 (beads docs/WORKTREES.md, moved 2026-07-11); github-platform :46's blog slug 404s. (f) Statuses already false when written: 'Open issue #4214' (closed 2026-08-03),
Backlog.md 'Known issue #558' (closed 2026-06-17), and the 'GitHub Action variant' there is Nulab Backlog, not Backlog.md. (g) github-platform :13 names the `issues` webhook for sub-issue actions;
they arrive on `sub_issues`. (h) Misattributions: the 41-spec-files anecdote (workflow-model :44) is TrueFoundry's, on neither cited page; 'Firesmith & Henderson-Sellers' links a sole-author
Henderson-Sellers 2002 article; Archon #901 predates the rewrite and contains no complaint; OpenAI's moderation post does describe an optional distillation step; wshobson/agents' Gemini CLI path was
replaced by Antigravity CLI on 2026-08-18 and Aeon claims nine harnesses, not six. (ii) Research-only documents, none re-cited for these figures by a live document [measured: grep of RUNBOOK, design,
AC/SM registers and ADRs], carry numbers their sources do not contain. grounding-s-results:262 credits arXiv 2402.11048 with '~93% say most docs are incomplete or outdated' [P]; the paper is an
Ericsson defect analysis of 101 bug reports whose only shares are 30%, 17% and 59%, and the 93% most plausibly belongs to Aghajani et al.'s ICSE 2019 survey it cites [documented/inferred].
workflow-model:44's 'documented case of 41 spec files across 9 repos ... nine AGENTS.md variants' is in neither cited page; it is a TrueFoundry post that presents the figures as an illustrative
narrative about 'an engineering manager named Bastien', so 'documented case' is contradicted by the source's own words [documented]. grounding-s-results:293 points 'Martian benchmark' at arXiv
2603.26130, which is Foundry AI's SWE-PRBench (350 PRs, 15-31% recall), not Martian's Code Review Bench (200k-300k PRs, ~56% precision) described at :252 [documented]. competitive-workflows:148's
'another lists 2,300+' is on no version of the kissmyskills page (figures present: 1,000+, 2,000+, 42,816+) [documented]. method-engineering:22's 'Method Composer 7.6.x (GA Dec 2020)' is dated 20 Mar
2020 (7.6) / Jul 2020 (7.6.1) by the cited IBM page [documented]. workflow-model:15's '~10/25/55/10' RUP effort curve and OpenUP 'practice' concept are on neither the cited GeeksforGeeks page (five
phases, no percentages) nor any cited source [documented]. Separately, the self-flagged UNVERIFIED item at shipmates-status:8 ('Cargo.toml on main still reads 0.1.12') is now first-verified: the
commit that set 0.1.19 is dated 2026-09-01T20:48Z, before the 2026-09-03 pass, so the observation was a crawl artefact and the 'Avoid (4): Cargo.toml lagging release tags' lesson at :39 rests on
nothing [documented]. (iii) Thirty-one drifts confined to research-only or superseded documents, none of which a live document re-cites for the drifted claim [measured] (ADR-0028 cites Prifti as prior
art by name only; ADR-0031 cites workflow-model §2, not the drifted rows). Dead or withdrawn: change-classification:495's JMLR PDF path 404s (live path /papers/volume11/el-yaniv10a/el-yaniv10a.pdf);
:505's Greptile post is 410 Gone (Wayback 2025-08-04 confirms the claim); workflow-model:17's faros.ai summary 301s to the blog index; workflow-model:7's iso.org/standard/63712 is the withdrawn 2017
edition, revised by ISO/IEC/IEEE 12207:2026 (April 2026, before the pass) [documented]. Superseded pins: arXiv 2606.20615v2 (v3 2026-08-16, three authors, adds a SWE-bench section), arXiv 2411.13768v2
(v3 retitled 'Evaluation-Driven Development and Operations of LLM Agents'), and the Azure DevOps cmmi-process page rewritten 2026-06-22 so that competitive-workflows:28's quotes now live on the
sibling cmmi-process-workflow page [documented]. Paraphrase in quotation marks: method-engineering:82 'limited guidance on how teams integrate' (HELENA, in neither PDF), selection-and-planning:255 'a
working agreement, not a strict set of rules' (page says 'Regard the criteria as guidelines, not strict rules'), method-engineering:13 'descriptors' (EPF/RMC vocabulary, not SPEM 2.0's 'Method Content
Use'), :63 'producers perform work units, work units maintain work products' (not in the SEI PDF) [documented]. Tag, venue and characterisation: [blog] for a theory preprint
(change-classification:418), 'Yih' for 'Yu' (:484), 'image classification only' for a paper with a tabular regression task (:496), 'FSE-Industry 2024' for AIware '24 (grounding-s-results:147),
[preprint] for an EASE 2026 paper (:326), ChatDev phases attributed to a survey that does not list them (method-engineering:120), 2511.09794 credited with 2509.13942's 132-run numbers (:122),
'preferred minimal conflict sets' in neither cited ASP paper (selection-and-planning:105), 'more understandable' for a size/complexity-versus-modularity trade-off (method-engineering:106), a 2024
caveat paper cited as support for 'easy' SAT (selection-and-planning:111), Kubernetes 'named triage owners' where ownership is by SIG (change-classification:469), Dynamic Workflows on 'Pro' at a
launch whose sources list Max/Team/Enterprise/API (verification:10), Jira sprawl 'per type per project' and 'validators' where the source says per project and conditions (competitive-workflows:18),
BMAD token-cost and v4-to-v6 churn not in the cited sources (:80), LogRocket cited for complaints its tutorial does not contain (:90), two of three Kiro complaints unsupported (:99), Dev Genius's
'missing link' being spec-to-code traceability, not a cross-harness selector (:193), and the superseded architecture-proposal:423 writing 'RunTests' and attributing -NullRHI to Epic pages that do not
carry it [documented]. Substance survives in every case; the fixes are wording, tags and URLs.

**Anchors.**
- `references/2026-09-08-evidence-augmentation.md:34` — "native summarisation can discard "nuanced understanding that took an entire session to build", which is why decisions belong in a durable file. ... | [1P] code.claude.com/docs/en/context-window; prompt-caching (compaction section)"
- `references/research/2026-09-06-grounding-s-results.md:262` — "**Documentation debt** [P]: Ericsson study (arXiv 2402.11048) ~93% say most docs are incomplete or outdated."
- `references/2026-09-09-change-classification.md:495` — "- El-Yaniv & Wiener, *On the Foundations of Noise-free Selective Classification*, JMLR 2010 — formal risk-coverage framework [peer] · https://www.jmlr.org/papers/volume11/el-yaniv10a.pdf"
- `references/research/2026-09-03-github-platform.md:73` — "GraphQL: **primary limit ~2,000 points/min** (query cost based), secondary limits include **100 concurrent requests**, **90s CPU/60s wall** (60s of which may be GraphQL), **~80 content-creating requests/min, 500/hr**."
- `references/research/2026-09-03-workflow-model.md:44` — "a documented case of **41 spec files across 9 repos, four describing the same service differently, and nine AGENTS.md variants** ([hidden costs](https://hiddedesmet.com/the-hidden-costs-of-spec-driven-development)"
- `references/research/2026-09-03-workflow-model.md:7` — "([ISO 12207](https://en.wikipedia.org/wiki/ISO/IEC_12207), [iso.org/standard/63712](https://www.iso.org/standard/63712.html)). That is nearly a definition of redkiln."

**Why it matters before Step 5.** (i) None. Phase 1 has no GitHub or GitLab adapter and no competitive claim gates a step. (ii) None; no live document re-cites these figures. They are the corpus's own
research record and will be re-read by whoever writes the next research pass. (iii) None. These are research-record hygiene items; a reader following any of them reaches a source that does not say
what the row says, or no page at all.

**Remediation.** (i) One dated erratum appended to each research dossier (research files are never edited in place either): replace the EA:34 quote with the verbatim 'full tool outputs and
intermediate reasoning are gone'; relabel the GraphQL figures (primary 5,000 points/hour; 2,000/min secondary) in github-platform :73 and verification :9; 'GA in 19.1 (Premium/Ultimate, up to 40 types
per top-level group)'; '50,000 items (docs)'; re-point the two Beads links to gastownhall/beads/blob/main/docs/reference/{advanced,worktrees}.md or a pre-2026-07-11 SHA; 'since-fixed' for #4214 and
#558 and delete the Nulab Action sentence; `sub_issues` event; add the TrueFoundry URL; relabel the Springer link 'Henderson-Sellers 2002 (Annals SE 14)'; soften Archon #901; 'distillation mentioned
only as an optional step'; Antigravity CLI / 183 skills / nine harnesses. Date every research read going forward: half of these were stale at authoring, which the corpus cannot see because the
dossiers carry no fetch dates. (ii) Dated erratum block at the foot of each research file (the files are dated records; do not rewrite rows): re-cite the 93% to Aghajani et al. ICSE 2019 or drop it;
reword workflow-model:44 to 'an illustrative scenario (TrueFoundry)' with the TrueFoundry URL; split grounding-s-results:293 into Martian's own benchmark page for :252's numbers and Foundry AI's
SWE-PRBench (arXiv 2603.26130) if wanted; replace 2,300+ with 1,000+ or 2,000+ (Agensi) per the page; correct Method Composer to Mar 2020 (7.6) / Jul 2020 (7.6.1); cite Kruchten or the Eclipse OpenUP
documentation for the RUP curve and the practice concept; retire shipmates-status:8's sentence with the 2026-09-01 commit as the resolution and strike 'Avoid (4)' at :39. (iii) One dated erratum block
per research file listing the corrected URL, tag or wording per row, in the order above; the ADR-0028 Prifti citation and ADR-0031 workflow-model citation are unaffected and are not touched. Do not
re-run the research pass for these; the corrections are all in hand from the 2026-09-10 fetches.

**Sources.**
- https://code.claude.com/docs/en/context-window (fetched 2026-09-10) — It replaces the verbatim conversation: full tool outputs and intermediate reasoning are gone. (The quoted 'nuanced understanding' phrase does not appear on any first-party page fetched.)
- https://docs.github.com/en/enterprise-cloud@latest/graphql/overview/rate-limits-and-node-limits-for-the-graphql-api (fetched 2026-09-10) — For users: 5,000 points per hour per user ... Secondary rate limits ... no more than 2,000 points per minute are allowed for the GraphQL API endpoint
- https://docs.gitlab.com/user/work_items/configurable_work_item_types/ (fetched 2026-09-10) — Introduced in GitLab 19.0 with a feature flag ... Generally available in GitLab 19.1. Feature flag removed. Tier: Premium, Ultimate
- https://docs.github.com/en/issues/planning-and-tracking-with-projects/managing-items-in-your-project/archiving-items-from-your-project (fetched 2026-09-10) — Note: A project can contain a maximum of 50,000 items across both active views and the archive page.
- https://api.github.com/repos/gastownhall/beads/contents/docs/ADVANCED.md (fetched 2026-09-10) — 'Not Found'; last commit touching the path 5c2fe0d4 2026-07-12 removed it; docs/WORKTREES.md moved 2026-07-11 (a86a2c5d).
- https://docs.github.com/en/webhooks/webhook-events-and-payloads#sub_issues (fetched 2026-09-10) — sub_issue_added, parent_issue_added, sub_issue_removed, parent_issue_removed are actions of the sub_issues event; the cited 2025-03-06 changelog does not mention webhooks.
- https://arxiv.org/html/2402.11048 (fetched 2026-09-10) — Silva, TechDebt@ICSE 2023, Ericsson defect analysis; no 93% figure; shares quoted 30%, 17%, 59%
- https://www.truefoundry.com/blog/spec-driven-development-ai-agents (fetched 2026-09-10) — 'Forty-one specs and nine agent files aren't a tidiness problem' - narrative about 'an engineering manager named Bastien'
- https://arxiv.org/abs/2603.26130 (fetched 2026-09-10) — SWE-PRBench (Foundry AI, 27 Mar 2026): 350 PRs, frontier models detect 15-31% of human-flagged issues; no Martian affiliation
- https://kissmyskills.com/blogs/news/best-claude-skills-marketplaces-2026 (fetched 2026-09-10) — page dated 29 Jun 2026; regex for 2,?300 gives zero hits; figures 1,000+, 501, 2,000+, 42,816+
- https://www.ibm.com/support/pages/ibm-engineering-lifecycle-optimization-method-composer-76 (fetched 2026-09-10) — Method Composer 7.6 generally available ... 20 Mar 2020; modified 29 April 2020
- https://api.github.com/repos/saman-mb/shipmates/commits/d1a2e46a (fetched 2026-09-10) — commit setting Cargo.toml version 0.1.19 dated 2026-09-01T20:48:22Z, before the 2026-09-03 pass
- https://www.jmlr.org/papers/v11/el-yaniv10a.html (fetched 2026-09-10) — live abstract page; PDF at /papers/volume11/el-yaniv10a/el-yaniv10a.pdf (200); cited path 404
- https://www.greptile.com/blog/ai-reviews-need-context (fetched 2026-09-10) — HTTP 410 Gone; Wayback 2025-08-04 snapshot carries the dependency-graph claim
- https://www.iso.org/standard/63712.html (fetched 2026-09-10) — Withdrawn / Revised by ISO/IEC/IEEE 12207:2026 (iso.org/standard/90219), published April 2026
- https://arxiv.org/abs/2606.20615 (fetched 2026-09-10) — [v1] 24 May 2026; [v2] 24 June 2026; [v3] 16 August 2026 (current)
- https://learn.microsoft.com/en-us/azure/devops/boards/work-items/guidance/cmmi-process-workflow?view=azure-devops (fetched 2026-09-10) — carries the four quoted CMMI WIT definitions; the cited cmmi-process page was rewritten (ms.date 2026-06-22)
- https://aclanthology.org/2023.emnlp-main.398/ (fetched 2026-09-10) — Gao, Yen, Yu, Chen - no author surnamed Yih

**Folds.** EXT-CLAUDECO-029, EXT-COMPETIT-097, EXT-COMPETIT-152, EXT-COMPETIT-088, EXT-COMPETIT-139, EXT-COMPETIT-025, EXT-COMPETIT-077, EXT-COMPETIT-102, EXT-COMPETIT-103, EXT-COMPETIT-110,
EXT-COMPETIT-129, EXT-COMPETIT-068, EXT-COMPETIT-057, EXT-COMPETIT-090, EXT-PEERANDS-011, EXT-PEERANDS-012, EXT-PREPRINT-063, EXT-PREPRINT-064, EXT-PROCESSL-098, EXT-PROCESSL-162, EXT-PROCESSL-170,
EXT-PROCESSL-084, EXT-STATEDAB-058, EXT-PROCESSL-072, EXT-PROCESSL-044, EXT-PROCESSL-046, EXT-PROCESSL-086, EXT-PROCESSL-078, EXT-PROCESSL-145, EXT-PREPRINT-015, EXT-PREPRINT-075, EXT-PROCESSL-172,
EXT-PROCESSL-195, EXT-PEERANDS-057, EXT-PEERANDS-067, EXT-PREPRINT-030, EXT-PREPRINT-046, EXT-PREPRINT-054, EXT-PREPRINT-061, EXT-PREPRINT-068, EXT-PREPRINT-072, EXT-PREPRINT-074, EXT-PREPRINT-078,
EXT-PEERANDS-092, EXT-PEERANDS-098, EXT-PROCESSL-039, EXT-PROCESSL-073, EXT-PROCESSL-074, EXT-PROCESSL-144, EXT-PROCESSL-149, EXT-PROCESSL-151, EXT-PROCESSL-152, EXT-PROCESSL-164, EXT-PROCESSL-002,
EXT-PROCESSL-003

### PIA-102 — Six relations the ADR README and section 10.5 leave undeclared, so a supersession check or an ADR-0012 discharge-by-id grep cannot resolve them

**Priority:** P3 · **Classification:** SHARPENS-KNOWN (ARCH-10.x-L-001 and ADR-0054-NEG-38/FORB-41 record scope limits and 'not audited for supersession' generally; ADR-0074-FORB-88, ARCH-6.6-011 and
ARCH-1.2-033 record the S-SUBTLE arm as ADR-0088's unrun spike with 0074's fixture design as consumer; ARCH-10.5-014 and ARCH-10.5-015 record the E3-SEV and S-SUBTLE-CB rows verbatim;
RUNBOOK.md:913-917, AC:4/:6534 and SM:1421 record the F1-F28 / F29-F40 split and defer the fix. None records the missing 0040/0054 edge, the SM-022 kill's conflict with ADR-0025's forbids (both
unrecorded, NEW halves), the two-valued E3-SEV status, the unnamed spike id in row 30, or that FINDINGS.md carries no pointer.) · **Category:** register-integrity · **Confidence:** high · **Owner:**
ADR maintainer; design author; SM register owner; Ryan Britton for FINDINGS.md

[measured] ADR-0040 closes the input-source grammar ('may not use any source form outside this closed set', `?` on `toolchain:` only) and ADR-0054 later legalises `artifact:Kind?`, yet both carry
`related: []` and README:18-20's rule that a later decision changing a record must name it is unmet, unlike the 0033 -> 0048/0049/0052/0070 edges; a grammar implementer working from 0040's forbids
block rejects a form 0054 permits (the schema document carries both). ADR-0074 clause 7 and ADR-0088 both adopt the S-SUBTLE 'presented as already approved' arm from evidence-augmentation section 4
rec 13; 0088 names 0074 only by legacy key ('D4's rev. 4 rebuild'), its `related` lists the superseded 0004, and README's deliberately-not-supersessions list (175-177) omits the pair as it carries
0023/0020. SM-022's kill offers 'removed or promoted to a gate', an outcome ADR-0025:52 forbids and AC-067 pins ('no advisory can promote itself to a blocking gate'), without saying that firing it
requires a superseding ADR. Design:1873 defines E3-SEV as 'what a blind annotation of the preserved BLANK would still buy' and in the same row marks it DONE on the non-blind sheet, while design:31
says 'E3-SEV stays named as the open route': one id, two statuses, and RUNBOOK 3c's `grep E3-SEV -> DONE` passes whichever is meant. S-SUBTLE-CB's section 10.5 Cost cell holds sizing prose with
neither a figure nor the register's 'Unpriced' token (the disclosed consequence of ADR-0088 clauses 5-6), and section 0.1 row 30 describes the arm without naming the spike id, so a discharge grep by
id on that row misses. FINDINGS.md (last edited 2026-09-05) contains zero occurrences of 'DRAFT', 'F29' or any F3x/F40 token, while design section 10.2(c), ADR-0012:50 and ADR-0013:52 still name it as
where findings live; F32-DRAFT:3 and F34-DRAFT:3 ask for a promotion that has happened for none of the eleven drafts (ids.json also_in_FINDINGS = []).

**Anchors.**
- `references/adr/0040-normative-step-input-source-grammar.md:34` — "Input sources are normative and closed to: `artifact:Kind[.field]`, `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]`, `${…}`, and literals."
- `references/adr/0054-optional-artifact-refs-legal-requires-alternation-not.md:31` — "`artifact:Kind?` is legal as an optional input reference, resolving to `null` under §9a.2 semantics when absent."
- `references/adr/0088-negative-controls-gain-a-confirmation-bias-arm.md:10` — "related: ["0004", "0019", "0082"]"
- `references/2026-09-08-success-measures.md:192` — "- **Kill:** an advisory channel whose acted-upon fraction falls to zero over 50 advisories is either removed or promoted to a gate"
- `references/2026-09-08-architecture-design.md:1873` — "the register row is what a *blind* annotation of the preserved `e3-severity-annotation-BLANK.md` would still buy | SM-086; ADR-0074, ADR-0082 | **$0** — the F24 adjudication itself cost **$0** [measured, brief §D] | No | **DONE**"
- `references/2026-09-08-architecture-design.md:31` — "is what would buy κ back, which is why E3-SEV stays named as the open route."

**Why it matters before Step 5.** None; no number is wrong and no build consequence. Each item is a one-line entry whose value is that ADR-0012's discharge-by-id and the README's supersession
machinery resolve on the first try.

**Remediation.** ADR README (accepted records untouched): add '0088 elaborates 0074 clause 7 and adds the unrun/sizing obligations' and '0054 extends 0040's closed grammar' to the
deliberately-not-supersessions / open-items list. SM-022: append 'promotion to a gate requires an ADR superseding 0025 (which forbids it); removal does not'. Design section 10.5: mark E3-SEV DONE
(non-blind, 2026-09-04 sheet) and register the blind re-annotation under a new id (E3-SEV-BLIND, consumer SM-086, cost $0 plus Ryan's time, 'Registered, not taken'), pointing section 0.1 row 31 at the
new id; put a figure or 'Unpriced' in S-SUBTLE-CB's Cost cell (first-order: 2 framings x 5 items x k draws at S-TYPED's median $3.03/review) and name S-SUBTLE-CB in row 30; correct the 2026-09-10
handoff's 'with cost' claim. FINDINGS.md: append a dated 'Continued in' index listing F29-DRAFT..F40-DRAFT and s-mig-report.md so the file (c) names resolves every F-number by pointer, then either
fold the drafts or amend design section 10.2(c) to 'in FINDINGS.md or a spikes/F<NN>-DRAFT.md it indexes'.

**Folds.** L05-overlap-05, L05-overlap-02, slice-G02-09, G09-07, G09-08, L10-register-integrity-05

### PIA-103 — Retired vocabulary survives: numeric review weights 0.2/1.0 (AC-299/AC-300, superseded by ADR-0020) and 'account-wide subsidy' (four SM sites, two AC sites, ADR-0078:19, design-recommendation:80 tagged [1P]), which the prompt-caching docs contradict (per-machine, per-directory prefixes)

**Priority:** P3 · **Classification:** SHARPENS-KNOWN ([B07-C12] ARCH-10.x-T-012 records the 'account-wide' retirement inside architecture-design.md only ('at all three sites'); the register/AC/ADR
survivals are not recorded. The AC-299/AC-300 numeric-weight leg is NEW (24 rows match the terms; none concerns it). | [B09a-C12] S-H1c is registered and unrun for exactly this question
(acceptance-criteria.md:184: 'S-H1c reports whether --exclude-dynamic-system-prompt-sections + --append-system-prompt keeps the subsidised head'); architecture-design.md:273.) · **Category:**
citation-drift · **Confidence:** medium · **Owner:** Claude (register maintainer)

Merged from 2 concerns; each member statement is carried verbatim. (i) [measured] AC-299 (:3750, :3753 'the empty block set at weight 0.2') and AC-300 (:3764 '`1.0`') phrase the review tier in the
proposal's numeric vocabulary while citing ADR-0020, whose :42 maps `0.2 → optional`, `1.0 → mandatory`; the mapping occurs zero times in the register, and AC-299's Verified-by asserts a value the
deontic enum has no constructor for (both also cite ADR-0015, superseded by 0020, where the numerics survive from). Separately, the design retired 'account-wide subsidy' (:49, '§7.2 records that there
is no account-wide shared head'), but the phrase survives at SM :431/:499/:536/:953 and AC :3401/:5243, and ADR-0078's own context (:19) keeps 'forfeits the account-wide subsidy' beside its :82
forbids; SM-069 (:536) still carries the [inferred] kill mechanism 'a CLI upgrade anywhere on the account invalidates it' — the mitigation ADR-0078's forbids block names — so a cost anomaly would be
mis-attributed to account-wide upgrades. (ii) design-recommendation :80 states, with a [1P] tag, that 'the cache is **org-scoped**, a read refreshes the entry it hits, and the default prompt's static
head is refreshed by any session on the account'. The prompt-caching page says the opposite for Claude Code's default prompt: 'the cache is effectively scoped to one machine and directory ... two
sessions in different directories build different prefixes and miss each other's cache', with cross-machine sharing available only via `--exclude-dynamic-system-prompt-sections`; the platform page
adds per-workspace isolation [documented]. The operational rule (pin CLAUDE_CODE_PROMPT_CACHE_TTL=1h and the CLI version; a CLI upgrade invalidates the next conversation's cache) is doc-supported and
stands, and the live design (:273, AC-184/AC-211) treats the append route as an unmeasured option gated on S-H1c, so nothing acts on the over-attribution before the spike. P3 rather than P2 because
S-H1c already fences the mechanism; the drift is an attribution in a research document that AC-184 re-cites as 'the subsidised head'.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:3750` — "GIVEN a stage whose review weight is `0.2` (read-only advisory)"
- `references/2026-09-07-design-recommendation.md:80` — "The docs explain the subsidy: the cache is **org-scoped**, a read refreshes the entry it hits, and the default prompt's static head is refreshed by any session on the account [1P]."
- `references/adr/0020-stage-rigor-is-a-deontic-enum.md:42` — "Anything already written numerically maps as: `0 → forbidden`, `0.2 → optional`, `0.5 → recommended`, `1.0 → mandatory`"
- `references/2026-09-08-acceptance-criteria.md:184` — "| 4 | The card-delivery append route | **S-H1c**, registered and unrun; ADR-0016 is conditional on it | S-H1c reports whether `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt` keeps the subsidised head **and** holds >=0.95 across commits."
- `references/2026-09-08-success-measures.md:536` — "that this segment is account-wide, and therefore that a CLI upgrade anywhere on the account invalidates it, is [inferred]** from F13 plus §C.1's account"
- `references/2026-09-08-architecture-design.md:273` — "The append route **may not be enabled as anything but an unmeasured config option until S-H1c runs**."

**Why it matters before Step 5.** (i) An implementer resolves the numerics via ADR-0020 in one hop; SM-069's inferred kill mechanism would send a Phase-1 cost anomaly to a forbidden explanation.
Hygiene. (ii) None; S-H1c gates the route and Step 5 does not touch the adapter.

**Remediation.** (i) Amend AC-299 and AC-300 to `optional`/`mandatory` (or add the mapping once in the register's §0). Replace 'account-wide' with 'shared-prefix subsidy' at the four SM sites and two
AC sites; SM-069's inference becomes 'a new CLI version rebuilds this machine's prefix on next launch'. ADR-0078 :19 is corrected only through a superseding record, never an edit. (ii) Dated amendment
to design-recommendation :80 replacing the mechanism with the docs' wording and the [1P] tag with [inferred, S-H1b]; S-H1c's pre-registration should name the per-directory prefix as the null
hypothesis, so a 'subsidy' result is measured rather than assumed. No ADR change.

**Sources.** https://code.claude.com/docs/en/prompt-caching (fetched 2026-09-10) — In Claude Code, the cache is effectively scoped to one machine and directory ... two sessions in different directories build different prefixes and miss each other's cache. ... Caches are isolated bet

**Folds.** L04-15, slice-G07-10, EXT-CLAUDECO-025

### PIA-104 — Record hygiene against the corpus's own rules: three SM rates with no interval, a scratch 30.1% tagged [measured], a scripted edit leaving '****'

**Priority:** P3 · **Classification:** SHARPENS-KNOWN (SM-NOBASE-027/028 (SM:405/:407) record SM-050 as no-baseline and SM-5.1-032 records SM-088's no-direct-baseline, not a rule-3 breach and nothing
for SM-044; AC:704 struck the 30.1% figure for a different reason ('not in the brief') and AC:6555/SM:1437 reinstated it as 'real'. The markup breakage is unrecorded (NEW half; '****' and 'one body
line' return nothing in known-issues or the anomaly pool).) · **Category:** register-integrity · **Confidence:** high · **Owner:** SM register owner; design author (Claude)

[measured] SM rule 3 (SM:6, :18) requires every quoted rate to carry a Wilson 95% interval or the words 'not computed'; SM-044 ('19 of 153 shipped specs (12%)', Wilson [8.1%, 18.6%]), SM-050 ('15/74 =
20.3%' [12.7%, 30.8%] and '21.6%' [13.8%, 32.3%]) and SM-088 ('188 findings, 78%' [71.8%, 83.5%]) carry neither within their rows, and the F12 rates carry no interval anywhere in the live set (design
:38, :78, :118, :184, :252, :684, :700, :768); SM-050's 'not a bound on either' is a different-quantity disclaimer, not the declaration rule 3 requires. The 30.1% unsatisfiability figure occurs in no
F-draft and no results-p6-*.txt; its only sources are DECISIONS-2026-09-08:5 ('scratch measurement') and the schema note's 'F35 scratch run', with none of the catalog/facets/packs digests METHOD-P
section 0 rule 2 requires ('reproducible from committed inputs or it is void'), yet ADR-0056 says 'This is what the corpus measured directly' and tags it [measured], and AC section 15 (AC:6555)
records the citation as 'real'. The 'two body lines' -> 'one body line' correction left the identical broken run '****one body line-- a single rewritten line, which the unified diff shows as a `-`/`+`
pair**' at design:21, SM:178, RUNBOOK:1202 and evidence-brief:55, and a fifth casualty at design:1818 ('and the twelfth one body line--' with no verb and no bold) -- the golden-constraint-7 hazard (a
scripted edit with no anchor assertion) in the section 0.1 row Step 4 readers are told to read first.

**Anchors.**
- `references/2026-09-08-success-measures.md:6` — "Where a rate is quoted, a Wilson 95% interval is quoted with it or the interval is declared **not computed**"
- `references/2026-09-08-success-measures.md:373` — "- **Baseline:** **19 of 153 shipped specs (12%)** write criteria without GIVEN/WHEN/THEN or a verification note [measured, S-MIG census, one corpus, one frozen SHA]"
- `references/2026-09-08-success-measures.md:407` — "360 trailer commits over 74 slices, with **15/74 = 20.3% ever-rejected** and **21.6% sealed more than once** [measured, F12]."
- `references/2026-09-08-acceptance-criteria.md:6555` — "| **30.1% unsatisfiable** | real — ceilings without the family-totality rule took unsatisfiability from 11.4% to 30.1% | activity-graph schema §9c.26 |"
- `spikes/p9/DECISIONS-2026-09-08.md:5` — "scratch measurement: naive ceilings alone → 30.1% unsatisfiable, so the totality lint is mandatory"
- `references/2026-09-08-architecture-design.md:21` — "and the twelfth (P01) changes ****one body line— a single rewritten line, which the unified diff shows as a `-`/`+` pair** of wording;"

**Why it matters before Step 5.** None; none of the three SM rows gates anything (SM-050 is 'no baseline', SM-088's target is 'chosen not derived'), the totality rule stands on the run-validate hole
F35 section 1.6 did record, and the markup is rendering only.

**Remediation.** Add the three intervals marked 'computed here', or 'interval not computed', to SM-044, SM-050 and SM-088. Re-tag 30.1% as [documented, DECISIONS-2026-09-08 scratch, no stamped output]
in AC section 15 and the schema note; leave ADR-0056 untouched and note the over-tag in the ADR README's known-issues list. Replace the broken run with 'changes **one body line** -- a single rewritten
line, which the unified diff shows as a `-`/`+` pair of wording' in all four files and repair design:1818.

**Folds.** L11-statistics-07, slice-G18-15, slice-G02-11

### PIA-105 — Three landscape citations in evidence-augmentation are mis-scoped or paraphrased as verbatim (Jira plan scope, OpenSpec wording, Factory path)

**Priority:** P3 · **Classification:** NEW · **Category:** citation hygiene in the live evidence register · **Confidence:** high · **Owner:** Ryan (evidence-augmentation errata)

Three rows in the live evidence register carry small citation faults. (1) 'Agents in Jira (GA all Cloud plans, May 2026)' is cited for the native workflow Approval step; GA-on-all-plans applies to
Agents, and Atlassian's HITL guide restricts native approvals to Premium and Enterprise plans, while the community post cited does not mention approvals at all. (2) OpenSpec's concepts doc is quoted
as 'fluid not rigid - no phase gates, work on any artifact at any time'; the source reads 'work on what makes sense', and the corpus renders a paraphrase as a verbatim quote. (3) The Factory citation
paths docs.factory.ai/changelog and /cli/changelog return 404; the live page is /changelog/release-notes. The substance of all three rows (approval step exists; OpenSpec's anti-gate stance unchanged
through v1.13.0; Factory's droid lineup retired) is confirmed. [documented]

**Anchors.**
- `references/2026-09-08-evidence-augmentation.md:38` — ""Agents in Jira" (GA all Cloud plans, May 2026) layers agent assignment and a native workflow **Approval step** — holding a transition until a named approver signs off — onto the existing generic workflow/state-category machinery."
- `references/2026-09-08-evidence-augmentation.md:220` — "Jira's native Agents (GA May 2026) ships a workflow **Approval step** usable as a durable, named-approver gate an agent's change must pass before a status transition"
- `references/2026-09-08-evidence-augmentation.md:128` — "OpenSpec's own concepts doc lists "fluid not rigid — no phase gates, work on any artifact at any time"."
- `references/2026-09-08-evidence-augmentation.md:125` — "Claim 4: nobody ships an incident-specific catalog; Factory retired its Reliability Droid | Confirmed: Factory's departmental droid lineup has given way to generic primitives (Spec Mode, Missions, custom droids, skills, plugins, native worktrees, `droid exec`); CLI v0.209.0 as of 2026-09-01."

**Why it matters before Step 5.** None; register hygiene.

**Remediation.** In evidence-augmentation's errata: cite the Atlassian HITL guide (not the community post) for the approval step and add 'native approvals: Premium and Enterprise plans only'; replace
the OpenSpec quotation with the verbatim 'fluid not rigid - no phase gates, work on what makes sense'; repoint the Factory citation to docs.factory.ai/changelog/release-notes. Owner: evidence-register
maintainer.

**Sources.**
- https://www.atlassian.com/software/jira/guides/agentic-engineering/human-in-the-loop (fetched 2026-09-10) — 'A native approval step ties the transition to two outcomes, approved or declined, and records who decided and when on the work item. Native approvals are available on Premium and Enterprise plans.'
- https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md (fetched 2026-09-10) — 'fluid not rigid - no phase gates, work on what makes sense ... Dependencies are enablers, not gates.'
- https://docs.factory.ai/changelog/release-notes (fetched 2026-09-10) — live changelog path; /changelog and /cli/changelog return 404

**Folds.** BP-SDD-10, BP-SDD-11, BP-SDD-15

### PIA-106 — Two register-filing contradictions: AC-506 binds a prefix the design marks DECIDED and undecided; brief section B files P7 4.4(1) against its own rule

**Priority:** P3 · **Classification:** NEW · **Category:** record-hygiene · **Confidence:** high · **Owner:** AC register maintainer (ports-dist and cli-doctor areas); brief maintainer

AC-506 is binding on 'the redkiln namespace prefix' with no literal (`redkiln-`, `rk-`, `redkiln:` and `rk:` all satisfy the text) and a 'regex' with no pattern; proposal section 4.9 decides
namespacing but names no prefix, design:1578 marks AC-506 DECIDED while design:1634 says the prefix is undecided, and AC-437's twin clause was cut for the same missing instrument [measured]. Brief
section B lists P1's alpha rule under *Fired* 'because it fires while being undecidable' (brief:91) and names P7 4.4(1) in the same group of four criteria the registered sample size cannot decide, yet
files P7 4.4(1) -- which F37:15 records as 'KILL FIRES on the pre-registered reading' -- only under *Not answerable / powerless* [measured]; a reader counting what fired gets a different count from
each rule.

**Anchors.**
- `references/2026-09-08-acceptance-criteria.md:6234` — "THEN every emitted name carries the redkiln namespace prefix"
- `references/2026-09-08-architecture-design.md:1634` — "| `init`'s namespacing clause | AC-437 | Cut rather than left as false assurance; returns as its own criterion when the prefix is decided | — |"
- `spikes/F37-DRAFT.md:15` — "| 1. `recommended` catches no more than `optional` — interval overlap on **every** item | overlap on **10/10**; identical rates on 9/10; `issue-81` 0/3 vs 1/3 | **KILL FIRES** on the pre-registered reading — but see §2, the test has almost no power at n=3"
- `references/2026-09-08-evidence-brief.md:59` — "**Fired.** P1 §1.4 modal-R (<80% identical) — **fires 4/4**, the one to act on. P1 §1.4 alpha<0.6 — fires twice but"
- `references/2026-09-08-evidence-brief.md:79` — "P7 §4.4(1) — establishes the criterion's **power, not its verdict**, at n=3. P7 §4.4(2) — n=1, arm dropped."
- `references/2026-09-08-evidence-brief.md:91` — "registered sample size cannot decide** — P1's alpha rule (F36; listed under *Fired* above, because it fires while being"

**Why it matters before Step 5.** None for Phase 1: rk-plugin-gen is Phase 2, and the brief is a ledger. Recorded because a binding criterion says decided what the design says is not, and the document
every area was refuted against breaks the filing rule it states.

**Remediation.** Decide the prefix in a one-line decision record, then amend AC-506 to name it literally with the lint regex and re-add AC-437's clause as its own criterion as design:1634 promises --
or mark AC-506 PROVISIONAL on that decision, matching AC-437's treatment. In the brief's section B preamble state one filing rule and apply it: either list P7 4.4(1) under *Fired* with the same 'fires
on the pre-registered reading while powerless' note, or move P1's alpha rule out of *Fired*.

**Folds.** slice-G10-08, L10-register-integrity-06

### PIA-107 — Statistical restatement hygiene: '3.6× more likely' quoted bare against §6.2's rule; two 'exactly one Wilson interval' claims name different figures

**Priority:** P3 · **Classification:** SHARPENS-KNOWN (SM-5.1-024 @ success-measures.md:1344 ('the 3.6× duration split is directional only'); ARCH-0.1-021. No row concerns S-MIG's interval bookkeeping
(14 Wilson rows, none on this).) · **Category:** hygiene · **Confidence:** high · **Owner:** Claude — design/RUNBOOK/AC closeout

(a) Design §6.2:866 rules the 3.6× duration effect 'directional only — the intervals overlap. It is never to be stated as an established ratio'. RUNBOOK 6.3 (:1842) and ADR-0077 §Evidence (:115)
restate '3.6× more likely' with neither the 9/79 vs 3/95 counts nor the qualifier; §0.1:39, §1:85/170 and AC-172:2337 carry the counts but not the qualifier. The ratio is exact (recomputed 3.61)
[measured]. The decision rests on the 7% and on reconcile cost being unmeasured, not on the ratio. (b) Design §8.7:1494 says the only Wilson interval computed on the S-MIG figures is the falsification
interval 134/153 (81.4–91.9%); the AC §13 residual-risk block (:5964) says 'exactly one figure (142/156, 85.5–94.6%)', an interval the AC body never computes (grep over 5470–5990: the residual bullet
only) and which belongs to SM-135's unknown-density baseline (:988). Both intervals are numerically correct [measured: 81.4–91.9 and 85.5–94.6 recomputed]; the two 'exactly one' claims name different
figures. SM-133 (:971, the fifteen lift slots) and SM-135 (unknown density) are not in conflict.

**Anchors.**
- `references/2026-09-08-architecture-design.md:866` — "The 3.6× duration effect is **directional only — the intervals overlap**. It is never to be stated as an established ratio (SM-075)"
- `RUNBOOK.md:1842` — "`reconcile()` runs at **every** entry — drift is 7% overall and **3.6× more likely** on gates over 1 h, while"
- `references/2026-09-08-architecture-design.md:1494` — "**Wilson intervals are not computed on any of these shares except the one that carries a falsification** — the acceptance-criteria row abo"
- `references/2026-09-08-acceptance-criteria.md:5964` — "Wilson intervals are computed for exactly one figure (142/156, 85.5–94.6%)."
- `references/2026-09-08-success-measures.md:988` — "142 of 156** reports = **91.0%** (**95% Wilson 85.5–94.6%**)"

**Why it matters before Step 5.** None. Record consistency only.

**Remediation.** Append '(directional; intervals overlap)' to RUNBOOK:1842, or drop the ratio there and cite the two rates; ADR-0077 is accepted and stays. State in one place (AC §13 residual risk and
design §8.7) that two intervals exist — 134/153 in the design/AC-454 falsification and 142/156 in SM-135 — and remove both 'exactly one' claims.

**Folds.** G13-08, slice-G20-12

### PIA-108 — PRIOR-ART research-lineage hygiene: the '62.5% agreement / kappa 0.37' LLM-judge ceiling is unsourced and could not be attributed live (UNVERIFIABLE, not wrong); section 7's OSF registered-report 'template to borrow' is a gap analysis that ships no template

**Priority:** P3 · **Classification:** NEW · **Category:** unsourced load-bearing number in a research document · **Confidence:** medium · **Owner:** Ryan (PRIOR-ART amendment)

Merged from 2 concerns; each member statement is carried verbatim. (i) The spike charter states as 'the known ceiling ... stated in every write-up' that 'the best fully-automatic LLM-judge
configuration measured 62.5% agreement / kappa=0.37, still ~9pp and 0.17kappa below a single average human annotator', and PRIOR-ART s4 uses the kappa 0.37 to argue the ceiling sits below its 0.60
rubric floor. No URL is attached to the figure anywhere in the corpus; a targeted search on 2026-09-10 did not locate a source stating it, and the nearest code-review judge studies report human-LLM
kappa 0.43-0.71. No live document (design, AC/SM registers, ADRs, METHOD files) re-cites the figure [measured, grep], so the fault is confined to the charter and PRIOR-ART; it is reported as
UNVERIFIABLE, not as wrong. [inferred] (ii) PRIOR-ART s7 says 'There is an OSF-based registered report template for SE controlled experiments whose structure we can borrow'. The cited paper (Bett et
al., arXiv 2602.09292, 10 Feb 2026) is titled 'Towards an OSF-based Registered Report Template', reports that no existing OSF registration type comprehensively satisfies the SE guidelines, and
presents 'initial results toward establishing an RR template'; it ships no template. Pre-registration in this programme is done in-repo under METHOD, so the design is unaffected; the sentence is wrong
as written. [documented]

**Anchors.**
- `references/2026-08-31-rust-inversion-spike-charter.md:221` — "The known ceiling is stated in every write-up: the best fully-automatic LLM-judge configuration measured 62.5% agreement / κ=0.37, still ~9pp and 0.17κ below a single average human annotator."
- `spikes/PRIOR-ART.md:175` — "readers judge how severely a claim was tested. There is an OSF-based registered"
- `spikes/PRIOR-ART.md:22` — "| LLM-judge ceiling ~62.5% agreement / κ=0.37 | Plausible and conservative. But the charter's *reporting rule* is wrong — see §4 |"
- `spikes/PRIOR-ART.md:176` — "report template for SE controlled experiments whose structure we can borrow."
- `spikes/PRIOR-ART.md:120` — "means the rubric needs work. The charter's own cited ceiling (κ = 0.37) is"

**Why it matters before Step 5.** (i) No build or register consequence; research-lineage hygiene. (ii) None; research-lineage hygiene.

**Remediation.** (i) Either attach the source (the charter's author should know which write-up it came from) or amend PRIOR-ART by dated amendment to tag the figure [inferred, unsourced] and rest s4's
argument on the sourced kappa bands (Landis-Koch; Judge's Verdict 2510.09738) instead. The charter is a dated document and is left as is with the amendment pointing at it. Owner: PRIOR-ART maintainer.
(ii) Amend PRIOR-ART by dated amendment: 'a proposed direction for an OSF-based template (Bett et al. 2026), no finished template exists'. Owner: PRIOR-ART maintainer.

**Sources.**
- https://arxiv.org/pdf/2509.01494 (fetched 2026-09-10) — nearest on-topic hit, not the source: code-review judge studies report human-LLM kappa 0.43-0.71 and human-human 0.61-0.76; no source states '62.5% agreement / kappa = 0.37'
- https://arxiv.org/abs/2602.09292 (fetched 2026-09-10) — 'Towards an OSF-based Registered Report Template for Software Engineering Controlled Experiments ... none comprehensively satisfied all guidelines ... initial results toward ... an RR template'

**Folds.** BP-EVAL-11, BP-EVAL-13

### PIA-109 — S-H1b-24H's cost cell omits that the 2026-09-06 mark is spent and METHOD-S's correction needs a fresh mark plus a 24-hour no-dispatch window

**Priority:** P3 · **Classification:** SHARPENS-KNOWN (ARCH-10.5-009 (design:1868), ARCH-10.4-008, ARCH-7.10-008, SM-5.2-004 record the read as not taken; none records the spent mark or the
quiet-window constraint) · **Category:** register-integrity · **Confidence:** high · **Owner:** design maintainer

The design prices S-H1b-24H as '3 haiku reads, plumbed already' [measured]. `s-h1b-24h.jsonl` holds exactly two lines, the header and the warm mark of 2026-09-06T18:55:48Z, and no read [measured]; a
read against that mark now measures a ~96-hour gap, not 24 hours. METHOD-S's 2026-09-06 correction (:720-742) requires each long arm's mark to be 'the last dispatch of any kind' before its read
because the ~15.1k tool block is shared across system-prompt texts [measured], and P1/P7/P8 have dispatched since. The arm therefore needs a new mark, a >= 24-hour window with no dispatch on the
machine, and a read: a scheduling constraint the register does not state. The proposal (:417) already judges the read low-value; the cell is incomplete rather than false.

**Anchors.**
- `references/2026-09-08-architecture-design.md:1868` — "| **S-H1b-24H** | Does the static prefix hold, or forfeit, at a 24-hour gap? | SM-064; §7 | 3 haiku reads, plumbed already [documented, proposal §8] | No | Read not taken |"
- `spikes/results/s-h1b-24h.jsonl:1` — "{"record":"header","experiment":"S","arm":"h1b-24h","started_at":"2026-09-06T18:55:48.682265600Z""
- `spikes/METHOD-S.md:738` — "their waits, and each long arm's mark stays the last dispatch of any kind"

**Why it matters before Step 5.** None for Step 5. Whoever schedules the unrun rows reads a five-minute chore that is actually a one-day quiet window.

**Remediation.** Reword the section 10.5 cost cell: 'mark + read, >= 24 h apart, with no other dispatch on the machine between them (METHOD-S s7, 2026-09-06 correction); the 2026-09-06 mark is spent'.
Cross-reference the F30 draft's Limit 4 once promoted. No METHOD edit is needed; the correction already stands as a dated amendment.

**Folds.** L10-register-integrity-07

### PIA-110 — Design section 0.1 row 24 names the S-P7B re-run in prose, the only one of ten BLOCKED rows whose unblock cell cites no register id

**Priority:** P3 · **Classification:** NEW · **Category:** convention · **Confidence:** high · **Owner:** Claude (design maintainer)

ADR-0012 requires open questions to be carried as named register rows with consumers. S-P7B exists (design:1870, consumers SM-090; ADR-0080) and R-P7RATIO beside it (:1871), but the BLOCKED row they
unblock (design:24) cites neither; the 'What unblocks it' cell names a register id in 9 of the 10 section 0.1 BLOCKED rows (P1, S-FACET, P2-B, S-BIN, S-H1c, P8/S-BIN, S-GIX, S-GIX, S-TTL1) and row 24
alone does not [measured]. A reader of section 0.1 cannot reach the spike, its cost note or its 'Not designed' status without a text search.

**Anchors.**
- `references/2026-09-08-architecture-design.md:24` — "A re-run at **n = 10 per cell** plus a settled severity vocabulary. The mandatory arm's cost ratio is additionally unreconciled"
- `references/2026-09-08-architecture-design.md:1870` — "| **S-P7B** | The quality shorthand at **n = 10 per cell**, with a settled severity vocabulary | SM-090; ADR-0080 |"

**Why it matters before Step 5.** Hygiene only.

**Remediation.** Insert '**S-P7B** ' at the head of row 24's 'What unblocks it' cell and 'R-P7RATIO' before the ratio sentence.

**Folds.** slice-G07-13

## 6. External grounding — what moved since 2026-09-08

Every external claim the census extracted from the corpus — 860 rows across fifteen families — was re-fetched live on 2026-09-10 against its primary (crates.io API, rustsec.org and osv.dev,
platform.claude.com and code.claude.com, the Claude Code CHANGELOG, GitoxideLabs source and docs.rs, arXiv full text, Crossref DOIs, the GitHub REST API), and every verdict that was not CONFIRMED was
re-fetched a second time by a separate pass before it was allowed to stand. The tally [measured]: **CONFIRMED 697 / DRIFTED 105 / CONTRADICTED 29 / UNVERIFIABLE 16 / STALE-BY-DATE 13**. The second
opinion flipped 28 verdicts: 25 DRIFTED softened to CONFIRMED (a paraphrase the first pass over-read), 2 CONTRADICTED softened to DRIFTED (EXT-COMPETIT-025, -038), and 1 DRIFTED hardened to
CONTRADICTED (EXT-CRATEVER-006: graph-flow's `postgres` feature is optional *and on by default*, so the corpus's "no required database" holds only under `default-features = false`). Of the 163
non-confirmed rows, impact splits [measured] changes-a-citation 76 / cosmetic 49 / changes-a-number 23 / none 14 / blocks-decision 1 — the one is EXT-CLAUDECO-030 (`defer` shipped; PIA-064). Of the
144 rows marked load-bearing (an ADR, AC or RUNBOOK step rests on them), 101 held and 43 did not; the 43 are all carried by a PIA concern or listed below. No load-bearing row is UNVERIFIABLE.

The number that matters most is not the confirmation rate. The corpus marks its own weak spots — `UNVERIFIED`, "URL not returned", "not re-verified this pass", "search-only",
"changelogs were not checked" — and 101 of the 860 rows carry such a marker. Those 101 now have a **first verification** [measured]: **CONFIRMED 70 / DRIFTED 20 / CONTRADICTED 6 / STALE-BY-DATE 3 /
UNVERIFIABLE 2**. Seventy of the corpus's confessed gaps closed clean, including the five that gate Phase-1 work (no trailer-write API in gix; `async_fn_in_dyn_trait` still nightly through 1.98.1;
`ephemeral_1h/5m` fields still documented; `--permission-prompts none` real at 2.1.259; `--bare` still API-key-only). The 26 that failed are exactly where the corpus said to look — the honesty markers
were correctly placed — and they are listed in 6.4 by name so the markers can be discharged rather than left standing. A CONFIRMED verdict here means the live source says what the corpus says it says
on 2026-09-10; it is not a claim about the source's truth, and 637 of the 697 confirmations carry impact `none` because the row was never load-bearing.

### 6.1 Per-family tally

All fetches 2026-09-10 [measured]. "LB" = load-bearing rows; "LB not confirmed" = load-bearing rows whose verdict is anything other than CONFIRMED.

| family | claims | confirmed | drifted | contradicted | unverifiable | stale | LB | LB not confirmed |
|---|---|---|---|---|---|---|---|---|
| advisories | 13 | 9 | 4 | 0 | 0 | 0 | 11 | 4 |
| gitoxide | 8 | 6 | 2 | 0 | 0 | 0 | 7 | 2 |
| sqlite | 5 | 4 | 1 | 0 | 0 | 0 | 5 | 1 |
| crate_versions | 28 | 22 | 2 | 1 | 0 | 3 | 15 | 5 |
| toolchain | 21 | 15 | 4 | 0 | 0 | 2 | 17 | 5 |
| anthropic_platform | 15 | 12 | 2 | 1 | 0 | 0 | 7 | 2 |
| claude_code | 43 | 34 | 5 | 1 | 3 | 0 | 19 | 3 |
| durable_execution | 28 | 21 | 6 | 1 | 0 | 0 | 13 | 6 |
| decision_records | 10 | 7 | 3 | 0 | 0 | 0 | 5 | 2 |
| stated_absences | 64 | 34 | 5 | 19 | 6 | 0 | 17 | 8 |
| no_url | 15 | 12 | 3 | 0 | 0 | 0 | 4 | 2 |
| preprints | 99 | 83 | 12 | 2 | 0 | 2 | 6 | 1 |
| peer_and_standards | 113 | 102 | 10 | 0 | 1 | 0 | 9 | 1 |
| process_literature | 200 | 176 | 21 | 0 | 1 | 2 | 0 | 0 |
| competitors | 198 | 160 | 25 | 4 | 5 | 4 | 9 | 1 |
| **total** | **860** | **697** | **105** | **29** | **16** | **13** | **144** | **43** |

Two families are structurally different from the rest. `stated_absences` (64 rows) is the corpus saying "nothing found"; 19 of the 29 CONTRADICTED verdicts across the whole census live here, because
an absence is refuted by one counter-example and the counter-examples were findable. Six of its UNVERIFIABLE rows are not absences at all — the extractor cut internal spike text
("none found by any arm (control item)", spikes/METHOD-S.md:68) in half; they are not defects and are excluded from every count of concern below. `process_literature` (200 rows, 0 load-bearing) is
research-folder background; its 24 non-confirmed rows change no decision and are carried, where material, by PIA-101.

### 6.2 What moved, by family

Each line: verdict; corpus → source; URL; fetch date; PIA id where a concern carries it. Rows with no PIA id fell below the synthesis materiality bar and are listed so the record is complete.

**advisories** (13; 4 DRIFTED, all load-bearing, all first verifications of self-flagged rows; PIA-083)
- DRIFTED, changes-a-number — RUSTSEC-2025-0021 "Fixed >= 0.71.0" → 0.71.0 is the `gix` facade's fixed version; `gix-features` is patched at >= 0.41.0 and gix 0.87.1 pulls `gix-features ^0.49.1`, so the pin is clear either way. https://rustsec.org/advisories/RUSTSEC-2025-0021.html (2026-09-10). PIA-083.
- DRIFTED, cosmetic — RUSTSEC-2024-0351 and -0353 attributed to sub-crate `gix` → -0351 is against `gix-ref` (patched >= 0.44.0), -0353 against `gix-worktree` (patched >= 0.34.0); both Windows-only. https://rustsec.org/advisories/RUSTSEC-2024-0351.html, https://rustsec.org/advisories/RUSTSEC-2024-0353.html (2026-09-10). PIA-083.
- DRIFTED, cosmetic — RUSTSEC-2024-0349 "crafted repo names" → crafted tree or blob names inside a crafted repository; hooks-to-ACE characterisation holds. https://rustsec.org/advisories/RUSTSEC-2024-0349.html (2026-09-10). PIA-083.
- CONFIRMED but worth carrying — two gix-family advisories absent from ADR-0084's list (GHSA-2vh6-hw4j-32ww `gix-packetline` <= 0.21.4; GHSA-7rhf-42qf-vrvc / CVE-2025-24890 `gix-sec` <= 0.13.2, patched 2026-09-02) are both already patched in the sub-crate ranges gix 0.87.1 requires, so "the pin is clean" survives with two more rows [inferred from dependency ranges]. https://crates.io/api/v1/crates/gix (2026-09-10). PIA-083.

**gitoxide** (8; 2 DRIFTED, both load-bearing)
- DRIFTED, cosmetic — "multi-maintainer ... not a single-author risk" → crates.io publish authority is a single owner (`owner_user: ['Byron']`, no team); the org lists two public members. ADR-0084's bus-factor note should say single crates.io owner. https://crates.io/api/v1/crates/gix (2026-09-10). PIA-083.
- DRIFTED, changes-a-citation — "shallow clone by SHA ... confirmed in gitoxide discussion #2309" → #2309 documents a *panic* via `with_ref_name` and a closed PR; the capability exists in 0.87.1 through `PrepareFetch::with_revision` (PR #2824, merged 2026-08-05), combinable with `with_shallow`. Trailer *parsing* exists (`BodyRef::trailers()` in gix-object 0.64.1) despite crate-status.md's unchecked box. https://github.com/GitoxideLabs/gitoxide/discussions/2309 (2026-09-10). PIA-042. [inferred] `with_revision` is weeks old; S-GIX should exercise it.

**sqlite** (5; 1 DRIFTED, load-bearing)
- DRIFTED, changes-a-citation — ADR-0084:115 "no advisory affects `bundled`" → RUSTSEC-2022-0090 against `libsqlite3-sys` exists *because* bundled SQLite carried CVE-2022-35737 ("As `libsqlite3-sys` bundles SQLite, it is susceptible to the vulnerability"); fixed at libsqlite3-sys 0.25.1, so 0.40.2 (`^0.38.2`) is unaffected. Three advisories in the family, not two. https://rustsec.org/advisories/RUSTSEC-2022-0090.html (2026-09-10). PIA-083.

**crate_versions** (28; 2 DRIFTED, 1 CONTRADICTED, 3 STALE-BY-DATE)
- DRIFTED ×2 / CONTRADICTED, changes-a-number, load-bearing — ADR-0086 "`temporalio-sdk` on `temporalio-sdk-core` v0.5.0 ... Public Preview, updated June 2026, read 2026-09-08" → on 2026-09-08 the current release was already 1.0.0 (2026-09-04, "considered stable") on sdk-core 0.9.0; four releases stale on the corpus's own read date. graph-flow "v0.6 makes postgres optional" → optional only from 0.8.0 (2026-09-07) and *on by default* (`default = ["postgres"]`); 0.6.0 hard-requires sqlx, so the charter the ADR withdrew was right for 0.6. The exclusion (server requirement; in-memory not durable) is untouched; the currency facts are wrong. https://crates.io/api/v1/crates/temporalio-sdk, https://crates.io/api/v1/crates/temporalio-sdk-core, https://raw.githubusercontent.com/a-agmon/rs-graph-llm/main/graph-flow/Cargo.toml (2026-09-10). PIA-074.
- STALE-BY-DATE, load-bearing — jsonschema 0.55.1 → 0.56.0 published 2026-09-10 (five releases in eleven days; belongs beside the bus-factor note); format validation is opt-in at runtime ("requires explicitly calling .should_validate_formats(true)"), which the design must call if it relies on format checks [inferred]. https://docs.rs/jsonschema/latest/jsonschema/ (2026-09-10). No PIA id.
- STALE-BY-DATE — toml 1.1.5 → 1.1.6 (2026-09-10), caret absorbs it; release-plz 0.3.163 → 0.3.165 (two patch releases in two days), not linked. https://crates.io/api/v1/crates/toml, https://crates.io/api/v1/crates/release-plz (2026-09-10). No PIA id; the design's self-expiry of the version list is doing its job.

**toolchain** (21; 4 DRIFTED, 2 STALE-BY-DATE)
- STALE-BY-DATE, load-bearing, first verification — RUNBOOK:1758 `grep -q 'channel = "1.97'` and the deferred release-notes read → 1.97.0 and 1.98.0 notes both read in full, neither stabilises `async_fn_in_dyn_trait`; current stable is 1.98.1 (2026-09-03), so a fresh exact pin is 1.98.x. R-ASYNC is discharged negative through 1.98.1 but still reads "Not started". https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/, https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/ (2026-09-10). PIA-024.
- DRIFTED, changes-a-citation, load-bearing — `[workspace.lints]` cited to Cargo Book `/cargo/reference/lints.html` → that page covers Cargo's *own* lints; the mechanism is on the Workspaces page. cargo #13157 is open (packages cannot override inherited lints), which *supports* ADR-0083's "workspace = true and nothing else" and rejects the per-crate override ADR-0083:68 prescribes. https://doc.rust-lang.org/cargo/reference/workspaces.html (2026-09-10). PIA-026, PIA-023.
- DRIFTED, cosmetic ×2, load-bearing — nursery lints "can change or be removed without notice" (ADR-0083:29/:67/:117, tagged documented) → in no Clippy text; the book says "buggy or need more work" and "not recommended to enable the whole group", which still licenses warn-never-deny. All five enumerated pedantic lints verified present and grouped `pedantic` in clippy master. https://doc.rust-lang.org/stable/clippy/lints.html, https://raw.githubusercontent.com/rust-lang/rust-clippy/master/README.md (2026-09-10). PIA-023.
- DRIFTED, changes-a-citation, load-bearing — "pin for a binary you control" framing attributed to swatinem.de → the post never mentions `rust-version`, MSRV floors or binaries vs libraries (grep: zero hits); the framing is RFC 3537's. https://swatinem.de/blog/rust-toolchain/ (2026-09-10). PIA-023.

**anthropic_platform** (15; 2 DRIFTED, 1 CONTRADICTED)
- CONTRADICTED, changes-a-number, load-bearing, first verification — "no limits table was found ... the <=24 optional / <=16 union caps are house rules, not vendor caps" → an explicit "Schema complexity limits" table exists: "Total optional parameters across all strict tool schemas and JSON output schemas | 24", union-typed parameters 16, strict tools per request 20, 180 s compile timeout. The caps are vendor constants scoped *per request*, so several strict tools can breach 24/16 while each type passes AC-132/157. https://platform.claude.com/docs/en/build-with-claude/structured-outputs (2026-09-10). PIA-005.
- DRIFTED, changes-a-citation, load-bearing — AC-388's ~5 s per-sibling stagger "per platform docs" → platform docs give no number ("wait for the first response before sending subsequent requests"); the 5 s is a Claude Code workflow-runtime default that holds all-but-the-first until the first response *begins*, capped at 5000 ms, then releases together — not a per-sibling interval. SM-114 A/B tests a different mechanism from the vendor's. https://code.claude.com/docs/en/workflows (2026-09-10). PIA-057.
- DRIFTED, changes-a-citation — "plugin marketplaces became stable April 2026 (docs)" → the cited page carries no stability date; CHANGELOG has no GA entry after "Plugin System Released" under 2.0.12. https://code.claude.com/docs/en/plugin-marketplaces (2026-09-10). No PIA id.

**claude_code** (43; 5 DRIFTED, 1 CONTRADICTED, 3 UNVERIFIABLE)
- CONTRADICTED, blocks-decision, first verification — evidence-augmentation:124 "Claude Code hook permission decisions are allow/deny only ... Several 2026 blogs claiming an April-2026 `tool_deferred` mechanism could not be corroborated in first-party docs — treat as unverified" → the hooks reference documents "Defer a tool call for later": `permissionDecision: "defer"`, "The process exits with `stop_reason: "tool_deferred"` and the pending tool call preserved in the transcript", resumed by `claude -p --resume <session-id>`; CHANGELOG 2.1.89 (npm 2026-03-31) records it shipping; precedence deny > defer > ask > allow. Claim 2 ("no AI coding tool ships ... resumable approval") must be rewritten. https://code.claude.com/docs/en/hooks, https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md (2026-09-10). PIA-064.
- DRIFTED, changes-a-citation, load-bearing — "rate_limit_event ... the docs do not mention" → the Agent SDK TypeScript reference now types `SDKRateLimitEvent` with `rate_limit_info: { status, resetsAt?, utilization? }`; the measured `unifiedWindows.{five_hour,seven_day}` shape remains undocumented (grep of typescript.md and python.md: 0 hits), so F1's characterisation survives for the fields the instrument reads. https://code.claude.com/docs/en/agent-sdk/typescript (2026-09-10). PIA-069.
- DRIFTED, changes-a-citation, load-bearing — "static prompt forfeits an account-wide subsidy that keeps the default prompt's head warm" (design-recommendation:80, tagged [1P]) → docs say "the cache is effectively scoped to one machine and directory ... two sessions in different directories build different prefixes and miss each other's cache"; the platform page adds per-workspace isolation. The operational rule (pin TTL and CLI) survives; the mechanism attributed to the docs does not. https://code.claude.com/docs/en/prompt-caching (2026-09-10). PIA-103.
- DRIFTED, changes-a-citation — compaction quote "nuanced understanding that took an entire session to build" (evidence-augmentation:34) → on no first-party page (191 code.claude.com URLs from llms.txt plus three platform pages grepped: zero hits); live text is "full tool outputs and intermediate reasoning are gone", which supports the substantive point. https://code.claude.com/docs/en/context-window (2026-09-10). PIA-099.
- DRIFTED, cosmetic, load-bearing — the 2026-08-31 RFC/charter sketch `claude -p ... --cwd <worktree>` → no top-level `--cwd` flag exists in cli-reference or `claude --help` 2.1.268 (only `claude agents --cwd`); the later design sets the working directory from the spawner and does not rely on it. https://code.claude.com/docs/en/cli-reference (2026-09-10). No PIA id (superseded documents only).
- DRIFTED, changes-a-number — subagent cap "v2.1.219+" → 2.1.217 ("Added a cap on concurrently-running subagents (default 20 ...)"); 2.1.219 is nested-subagent `parent_tool_use_id` forwarding. Harmless to the >= 2.1.251 floor. https://code.claude.com/docs/en/sub-agents (2026-09-10). No PIA id.
- UNVERIFIABLE ×3 — the 98 % / 132k→2.3k compaction figure (corpus already refuses to cite it; no primary exists); the "stable April 2026" marketplace date (page does not carry it); HANDOFF.md:303's
  private artifact link (not retrievable; content duplicated in findings-for-product). Not defects; the corpus's own do-not-cite stance stands. (2026-09-10.)

**durable_execution** (28; 6 DRIFTED, 1 CONTRADICTED)
- DRIFTED, changes-a-number, load-bearing — Step Functions "max activity TTL 6 h" (architecture-design:841) → in no AWS page; a callback task waits "until the workflow execution reaches the one year service quota", with `HeartbeatSeconds`/`TimeoutSeconds` as the opt-in bound; conflicts with the corpus's own grounding-s-results:232 ("up to the 1-year cap"). https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html (2026-09-10). PIA-085.
- CONTRADICTED, changes-a-citation, first verification — "every durable system surveyed — incl. Azure Durable — mints a single-use token" (Azure row "attempted and not settled") → Azure external events mint no token: addressed by `instanceID` + `eventName`, "waits indefinitely", "at-least-once delivery guarantee", deadline only by racing a durable timer. The cross-system conclusion (tree-hash binding is stronger than all) survives; the Azure column at architecture-design:848 should be filled in. https://learn.microsoft.com/en-us/azure/durable-task/common/durable-task-external-events (2026-09-10). PIA-085.
- DRIFTED, cosmetic, load-bearing, first verification — DBOS "on expiry → error" → `get_event`/`recv` return `None` on timeout ("returning None if the wait times out"); only `read_stream` raises. Sentinel, like Inngest. https://docs.dbos.dev/python/tutorials/workflow-communication (2026-09-10). PIA-085.
- DRIFTED, changes-a-citation, load-bearing — AWS backoff post "nudges toward Equal Jitter specifically for throttling" (architecture-design:345, evidence-augmentation:85, tagged [1P]) → the post calls Equal Jitter "the loser" and never uses the word "throttl"; the recommendation lives in AWS SDK retry-mode docs. The "base 2 s / cap 5 min is absent" half holds; the design's full jitter is what the post favours. https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/ (2026-09-10). PIA-091.
- DRIFTED ×3 (temporalio 0.5.0 → 1.0.0; graph-flow 0.6 → 0.8.0 with sqlx required at 0.6; iopsystems/durable "no version or release tags" → tags exist, only GitHub Releases are absent, and `durable` 0.1.0 is on crates.io). https://crates.io/api/v1/crates/graph-flow, https://api.github.com/repos/iopsystems/durable (2026-09-10). PIA-074.
- CONFIRMED and closes an open item — ADR-0086:61 "nobody checked whether temporalio-sdk ships a time-skipping local test server" → it does: `ephemeral_server::TestServerConfig` downloads `temporal-test-server` (`has_test_service: true`), and SDK 1.0.0 adds a `testing` feature. Testing-strategy only; production still needs a server. https://github.com/temporalio/sdk-rust (2026-09-10). PIA-074 may record the answer.

**decision_records** (10; 3 DRIFTED; PIA-023)
- DRIFTED, changes-a-citation, load-bearing, first verification — "adr-tools AND Log4brains regenerate a README/index table from frontmatter; flat NNNN-slug.md is common practice" → Log4brains rejects NNNN ("No required file numbering schema ... avoids git merge issues"; uses `YYYYMMDD-slug.md`) and infers metadata "from its raw text and git logs", not frontmatter; adr-tools regenerates the TOC only on `adr generate toc`. ADR-0085:108 should cite adr-tools alone. https://github.com/thomvaill/log4brains (2026-09-10). PIA-023.
- DRIFTED, changes-a-citation — "MADR minimal template keeps title, Context, Decision Outcome and status/date frontmatter" → the minimal template has *no* frontmatter and *keeps* Considered Options; only Decision Drivers, Confirmation, Pros/Cons and More Information are cut. https://raw.githubusercontent.com/adr/madr/main/template/adr-template-minimal.md (2026-09-10). PIA-023.
- DRIFTED, cosmetic, load-bearing — "MADR ... allows Consequences/Confirmation to be updated over time" → not in the template, CHANGELOG or docs; at best inferred from the last-updated date field. https://raw.githubusercontent.com/npryce/adr-tools/master/src/adr-new (2026-09-10). PIA-023.

**stated_absences** (64; 5 DRIFTED, 19 CONTRADICTED, 6 UNVERIFIABLE-as-misclassified)
- CONTRADICTED ×4, changes-a-citation, three load-bearing — ADR-0074:100 "Clause 6 has **no external prior art beyond GitLab flaky-quarantine and Cloudflare "won't fix"** ... `Quarantined` is original process design"; ADR-0082:29/:90 same → Snyk `ignore` is a published first-party pattern with exactly `{reason, expiry, policy-file attribution}` ("--reason: Human-readable <REASON> ... --expiry: Expiry date in YYYY-MM-DD format. Defaults to 30 days"); GitHub code-scanning dismissal records a reason and is reopenable; SonarQube has False Positive / Won't Fix / Accepted; OASIS SARIF 2.1.0 standardises a `suppression` object whose status `underReview` is a quarantine state. Design unchanged; "original" overstates — "convergent with Snyk ignore-with-expiry" is the honest framing. https://docs.snyk.io/developer-tools/snyk-cli/commands/ignore.md, https://docs.github.com/en/code-security/code-scanning/managing-code-scanning-alerts/resolving-code-scanning-alerts, https://docs.oasis-open.org/sarif/sarif/v2.1.0/os/sarif-v2.1.0-os.html (2026-09-10). PIA-087.
- CONTRADICTED ×5, changes-a-citation, two load-bearing — ADR-0085:30 "Rules 4 ... and 6 ... have no external norm behind them at all" → NASA SE Handbook App C/D ("When developing requirements, it is important to identify an approach for verifying the requirements ... The matrix should identify each shall by unique identifier") and ISO/IEC/IEEE 29148's per-requirement verification method cover rule 4; the TBD/TBR marker convention with owner, rationale and closure date covers rule 6. Requirement-grain, not agile-AC-grain, so "no agile-AC-specific norm" may survive; "none at all" does not. https://www.nasa.gov/reference/appendix-d-requirements-verification-matrix/, https://www.nasa.gov/reference/appendix-c-how-to-write-a-good-requirement/ (2026-09-10). PIA-086.
- CONTRADICTED ×6, changes-a-citation, two load-bearing — ADR-0083:72/:121 "thiserror-for-libraries / anyhow-for-binaries ... restated by two 2026 practitioner blogs and by no first-party source" → the author of both crates documents the split in the anyhow README "Comparison to thiserror" and thiserror's docs.rs page ("See also the anyhow library for a convenient single error type to use in application code"). It says "application code", not "binaries only"; the stricter wording remains house convention. The "[documented, weak]" grade can be raised. https://github.com/dtolnay/anyhow, https://docs.rs/thiserror/latest/thiserror/ (2026-09-10). PIA-079.
- CONTRADICTED / DRIFTED ×3, changes-a-citation, one load-bearing — ADR-0081:190 "no external source frames activity selection as a research question at all" → Situational Method Engineering is a book-length field whose central question is which process fragments to select for a situation (using the very OPF deontic matrix the corpus cites); arXiv 2603.22386 surveys agentic-workflow generation. The narrower claim (no published system selects activities per run from internal quality signals) was not refuted. https://link.springer.com/book/10.1007/978-3-642-41467-1, https://arxiv.org/abs/2603.22386 (2026-09-10). PIA-092.
- CONTRADICTED, changes-a-citation — change-classification:125 "data-migration facet: Nothing found, positive or negative" → Qiu, Li & Su (ESEC/FSE 2013) detect schema-changing revisions across 160K revisions and give base rates the 0/20 result can be compared against; adjacent (schema-file diffing), so "nearest literature: schema co-evolution" is the right row. https://web.cs.ucdavis.edu/~su/publications/fse13-db-study.pdf (2026-09-10). PIA-093.
- CONTRADICTED, changes-a-number, first verification — shipmates "Cargo.toml on main still reads 0.1.12, behind the tags; UNVERIFIED whether a real gap or a crawl-timing artefact" → `version = "0.1.19"`, set 2026-09-01T20:48Z, two days before the research pass; a crawl artefact. The "Avoid (4): Cargo.toml lagging release tags" lesson rests on it and should be struck. https://raw.githubusercontent.com/saman-mb/shipmates/main/Cargo.toml (2026-09-10). PIA-101.
- CONFIRMED absences worth naming: `async_fn_in_dyn_trait` nightly-only with no stabilisation announcement (tracking issue #133119 open, "Exploration" status); no authoritative guide for a ~12-crate one-binary per-adapter workspace (searched three ways; practitioner guides only); no reachability vendor frames a call graph for first-party domain tagging. https://github.com/rust-lang/rust/issues/133119, https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html, https://docs.snyk.io/manage-risk/prioritize-issues-for-fixing/reachability-analysis (2026-09-10). Absences stand; they are absences, not clean bills.

**no_url** (15; 3 DRIFTED; all 15 rows self-flagged "URL not returned", 12 now located and confirmed)
- DRIFTED, changes-a-citation, load-bearing ×2 — swatinem.de and Log4brains, as above (the corpus could not fetch either; both now primary-located and both found to support less than attributed).
  PIA-023.
- DRIFTED, changes-a-number — "multi-provider orchestration cuts spend 40-60%" → cloudnuro.ai has the practice description "almost verbatim" but none of the figures; mintmcp.com says 40-70 % from model routing; the 603B-token / $1.3M example traces to Steinberger's OpenClaw post. The corpus's "do not quote without a primary" hedge stands and should be treated as binding. https://www.cloudnuro.ai/blog/llm-cost-management-2026-enterprise-buyers-guide (2026-09-10). No PIA id.
- CONFIRMED, first verification — Pi Stack (2026-06-22) and OneUptime (2026-01-25) both say exactly what the corpus says; Rust internals thread #20507 confirms workspace-wide feature unification under resolver 2 and that `resolver.feature-unification = package` (RFC 3692) is still nightly-only. https://www.pistack.xyz/posts/2026-06-22-rust-error-handling-anyhow-thiserror-eyre-guide/, https://internals.rust-lang.org/t/feature-unification-opt-out-in-workspace-crate/20507 (2026-09-10).

**preprints** (99; 12 DRIFTED, 2 CONTRADICTED, 2 STALE-BY-DATE)
- DRIFTED, changes-a-citation, load-bearing — architecture-design:19/:143 "corroborated across five 2026 frontier models, with Claude-Sonnet-4 at background temperature 0 — the instability is schema-induced, not decoding noise [2603.04417 / 2604.22411]" → 2603.04417 tests Sonnet-4.5 (not Sonnet-4, which belongs to 2604.22411) and offers a decoding-level mechanism; "schema-induced" is stated by neither paper. Per-source rows (:2030, ADR-0079:222) are accurate; the joint summary sentence presents ADR-0079's inference as external. https://arxiv.org/pdf/2603.04417 (2026-09-10). PIA-020.
- DRIFTED, changes-a-citation — 2511.01176 "reports LLM classification instability correlates with proximity to the decision boundary" (the basis for P1-BOUND) → the sentence is an unmeasured premise in Section 5.2, no boundary-distance data; its measured result (temperature "has a small influence on ... response consistency") better supports "no temperature spike". ADR-0079:231 hedges; architecture-design:457 and EA:202 do not. https://arxiv.org/pdf/2511.01176 (2026-09-10). PIA-020.
- CONTRADICTED, changes-a-citation — 2508.16445 "Essence Coach ... flags that Essence assessment requires precise criterion matching probabilistic models struggle to guarantee" (EA:99, ADR-0079:140) → three full-text passes (v1 HTML, v1 PDF, v2 HTML) find no such statement; the paper's stated limitations are dataset size, no comparison, manual validation. v2 (2026-09-01) retitled. https://arxiv.org/pdf/2508.16445v1 (2026-09-10). PIA-020.
- CONTRADICTED, changes-a-citation — change-classification:424 "Semgrep 2.43% F1 / 0.57% recall on BigVul/Devign, citing arXiv 2604.13764" → 2604.13764 is RealVuln (Python-only): Semgrep F1 18.9, recall 17.5 %; BigVul, Devign, 2.43 and 0.57 appear nowhere in it. Likely a mis-pasted ID [inferred]. https://arxiv.org/html/2604.13764v1 (2026-09-10). PIA-093.
- CONFIRMED after a flip (DRIFTED → CONFIRMED), load-bearing — 2603.18740 (ADR-0088's source): "250 CVE/patch pairs" is correct (the 17-vs-250 doubt dissolves); "16-93 %" is real but is percentage-point declines *across models* under one condition (Strong Bug-free: -16.2, -16.7, -42.9, -59.9, -93.5 pp; Opus 4.5 -4.9 pp n.s.), not across conditions; v2 (2026-04-23) is titled "Contextual Bias", not "Confirmation Bias". The mechanism inversion ("FPs barely move" vs the paper's collapse of false positives) is carried by the best-practice lane, BP-EVAL-07, into PIA-013. https://arxiv.org/pdf/2603.18740 (2026-09-10). PIA-013.
- The remaining nine are research-folder drift (wrong author surname "Yih" → "Yu" on ALCE; "image classification only" for SelectiveNet, which also runs a UCI regression set; AutoCommenter venue AIware '24 not FSE-Industry; Ericsson 2402.11048 carries no "~93 %" figure, which is Aghajani et al.'s; 2603.26130 is Foundry AI's SWE-PRBench, not the Martian benchmark; 2604.23667 is accepted at EASE 2026, so [peer] not [preprint]; 2604.26275 does not enumerate ChatDev's phases; 2511.09794 is Waterfall-only and carries none of the 132-run numbers; 2109.08304 does not itself establish preferred minimal conflict sets). All https://arxiv.org/abs/… (2026-09-10). PIA-101.

**peer_and_standards** (113; 10 DRIFTED, 1 UNVERIFIABLE)
- DRIFTED, changes-a-citation, load-bearing — "rank reversal occurs for virtually every weighted MCDM method, TOPSIS and SAW included" pinned to ScienceDirect S0895717711007850 → that PII is García-Cascales & Lamata 2012, TOPSIS-specific; the sentence is verbatim in arXiv 2312.03006 ("Rank reversal of one or another type occur for virtually every MCDM method"), cited there to Aires & Ferreira 2018 and Wang & Luo 2009 (which covers SAW and TOPSIS). ScienceDirect returned 403, so the PII's full text could not be excluded outright [inferred on title and scope]. The result behind "rigor is ordinal, never summed" stands; the pin should move. https://arxiv.org/html/2312.03006 (2026-09-10). PIA-098.
- DRIFTED, changes-a-number, first verification — "the DA goal poster PDF lists the 24 goals only" → the poster lists 21 (lacks Intake Work, Organize Metrics, Measure Outcomes); the 24-goal list and the per-goal decision points are on the process-goals web page, which is reachable today. https://www.pmi.org/-/media/pmi/microsites/disciplined-agile/posters/daprocessgoalsposter85x11.pdf (2026-09-10). No PIA id.
- DRIFTED ×2, changes-a-citation, first verification — 10.1007/s10664-021-10051-z described as an "MSR practice survey" carrying 97.6 % → it is Wen et al., quick-remedy commits, EMSE 2022, an empirical study that *quotes* the 97.6 % precision from Tufano et al. 2019. Number confirmed, descriptor and provenance drift. https://pmc.ncbi.nlm.nih.gov/articles/PMC8553712/ (2026-09-10). No PIA id.
- DRIFTED ×2, changes-a-citation — DOI 10.1023/A:1020570027891 labelled "Firesmith & Henderson-Sellers" (the OPF book) → sole-author Henderson-Sellers, Annals of Software Engineering 14 (2002). https://api.crossref.org/works/10.1023/A:1020570027891 (2026-09-10). PIA-101.
- CONFIRMED, first verification, closes R-MUT — "median one live mutant per changelist at review start (ICSE 2021); provenance uncertain, possibly 2018 SEIP" → both figures ("The median is 1 for Review and 0 for Submit"; "Mutants are coupled with 70% of high-priority bugs") are in Petrović, Ivanković, Fraser, Just, ICSE 2021, as ADR-0019 cites; the 2018 candidate need not be chased. https://arxiv.org/pdf/2103.07189 (2026-09-10).
- CONFIRMED, first verification — the five OPF deontic values {mandatory, recommended, optional, discouraged, forbidden}, which the corpus marked [inferred], are stated verbatim in Fahmideh, Sharifi & Jamshidi (SoSyM author copy), citing the OPF book. https://arxiv.org/pdf/2004.10136 (2026-09-10). The [inferred] tag can be raised to [documented, secondary].

**process_literature** (200; 21 DRIFTED, 1 UNVERIFIABLE, 2 STALE-BY-DATE; 0 load-bearing; PIA-101 where material)
- DRIFTED, changes-a-citation — Semgrep "59.9 % IDOR recall; 61 % precision" both cited to the 2025 post → 61 % is there; 59.9 % is on the 2026-08-27 Semgrep Multimodal benchmark post, a different product and study. https://semgrep.dev/blog/2025/ai-powered-detection-with-semgrep/ (2026-09-10). PIA-093.
- DRIFTED ×2, changes-a-citation — the "41 spec files across 9 repos ... nine AGENTS.md variants" case attributed to hiddedesmet.com and dev.to → on neither; it is a TrueFoundry post written as a persona narrative ("Bastien"), so "a documented case" overstates [inferred]. https://hiddedesmet.com/the-hidden-costs-of-spec-driven-development (2026-09-10). PIA-101.
- Dead or moved links at authoring time — ISO 12207 cited at iso.org/standard/63712 (the 2017 edition; 12207:2026 published April 2026, standard 90219); Azure DevOps CMMI quotes at the cmmi-process
  page (rewritten 2026-06-22; quotes live on cmmi-process-workflow); Greptile post 410 Gone (Wayback confirms content); Faros DORA summary 301s to the blog index; JMLR El-Yaniv & Wiener PDF path 404s
  (paper and claim correct). (2026-09-10.) PIA-101.
- STALE-BY-DATE / DRIFTED — Dynamic Workflows "Pro/Max/Team/Enterprise" → neither InfoQ nor testingcatalog lists Pro; InfoQ *does* quote the token warning the corpus could not pin ("can consume substantially more tokens than a typical Claude Code session"). https://www.infoq.com/news/2026/06/dynamic-workflows-claude-code/ (2026-09-10). PIA-101.
- Paraphrases in quotation marks (DoR "a working agreement, not a strict set of rules"; HELENA "limited guidance on how teams integrate"; SPEM "descriptors"/"Capability Pattern", which are EPF
  Composer vocabulary absent from the 236-page OMG PDF). (2026-09-10.) PIA-101.

**competitors** (198; 25 DRIFTED, 4 CONTRADICTED, 5 UNVERIFIABLE, 4 STALE-BY-DATE; 9 load-bearing, 1 not confirmed)
- CONTRADICTED, changes-a-citation — competitive-workflows:42 "GitLab custom types still not GA as of the docs read (2026-09-07)" → "Generally available in GitLab 19.1" (released 2026-06-18), Premium/Ultimate, up to 40 per top-level group; already GA before the corpus read. #393605 half confirmed (open importer issue). https://docs.gitlab.com/user/work_items/configurable_work_item_types/ (2026-09-10). PIA-101.
- CONTRADICTED, changes-a-number, first verification — github-platform:43 "Community reports ~1,200 items as a practical ceiling (#9678); exact hard cap UNVERIFIED, GitHub doesn't publish one number" → "A project can contain a maximum of 50,000 items across both active views and the archive page"; the 1,200 figure is a January-2022 limit the same thread reports lifted in February 2024. https://docs.github.com/en/issues/planning-and-tracking-with-projects/managing-items-in-your-project/archiving-items-from-your-project (2026-09-10). PIA-101.
- STALE-BY-DATE, load-bearing, first verification — Beads 26,983 stars flagged as conflicting with "~15.9K" → no conflict: 26,983 is Beads (27,041 today) and ~15.9K is Gas Town, a different repo (18,006 today, 13 % above the written figure, so already low at authoring). The batch note's conflict is a category error. https://api.github.com/repos/gastownhall/beads, https://api.github.com/repos/gastownhall/gastown (2026-09-10). No PIA id.
- DRIFTED, changes-a-citation, first verification — Archon #901 "reflects users asking about the lost task-management capability; tone/volume UNVERIFIED" → #901 is December 2025, pre-rewrite, two comments, asks whether Archon beats Jira; #957 has one comment ("Wow well done"). Volume of complaint in the cited threads is effectively zero. https://api.github.com/repos/coleam00/Archon/issues/901 (2026-09-10). PIA-101.
- DRIFTED, first verification ×2 — sudoall.com and mintmcp.com, cited UNVERIFIED for LangGraph PostgresSaver-per-node and the 603B-token figures → neither page carries the attributed text or numbers today. The corpus's own hedge was right. https://sudoall.com/multi-agent-coordination-2026-playbook/, https://www.mintmcp.com/blog/llm-cost-optimization-agent-teams (2026-09-10). No PIA id.
- Dead links at authoring — Beads WORKTREES.md and ADVANCED.md (moved/removed 2026-07-11/12, before the 2026-09-05/06 documents that cite them); Copilot coding-agent blog slug 404s (correct slug
  found); Codex non-interactive docs 308 to learn.chatgpt.com. Issues cited as "open"/"known" that were closed before authoring: beads #4214 (closed 2026-08-03), Backlog.md #558 (closed 2026-06-17),
  toml #781/#1052. Wrong GraphQL label: 2,000 points/min is a *secondary* limit; the primary is 5,000 points/hour. (2026-09-10.) PIA-101.
- CONFIRMED and load-bearing — every number in Cloudflare's post ("131,246 review runs", "48,095 merge requests", "The average review costs $1.19", "0.6% of merge requests"); Agent OS 3.0 CHANGELOG "Implementation/orchestration phases retired—frontier models handle this well on their own now" verbatim; Dagster RFC #22811 quotes verbatim (source spells "course-grained"); Linear as Symphony's state machine; GitLab quarantine mechanism (first verification). https://blog.cloudflare.com/ai-code-review/, https://raw.githubusercontent.com/buildermethods/agent-os/main/CHANGELOG.md, https://github.com/dagster-io/dagster/discussions/22811, https://handbook.gitlab.com/handbook/engineering/testing/quarantine-process/ (2026-09-10).

### 6.3 The four best-practice lanes (77 rows, separate from the 860)

Verdicts [measured]: Rust architecture 20 (13 confirmed, 5 drifted, 2 stale); durable workflow state 20 (13 / 6 / 1 contradicted); LLM evaluation 18 (13 / 3 / 1 contradicted / 1 unverifiable); SDD
landscape 19 (11 / 4 / 2 contradicted / 2 stale). The ones that change a decision are already concerns: BP-EVAL-07 (2603.18740's asymmetry runs the other way — under strong bug-free framing false
positives collapse, +46 to +95 pp true-negative rate; ADR-0088 inverts it) → PIA-013; BP-SDD-01/-02/-19 (spec-kit 1.0.x ships a `gate` step with persisted paused runs and
`specify workflow resume <run_id>`, PR #2158 merged 2026-04-14; Archon v0.4.0 has `approval` nodes and `archon workflow approve/resume/reject`; Bernstein, 1,158 stars, is absent from the corpus) →
PIA-090, so the narrowed novelty claim "in an AI-coding-specific backlog or spec tool" is false. BP-DUR-08 (DBOS Inc. published a first-party `dbos` crate 0.5.0 on 2026-09-09, a library with no
server, Postgres-backed) → PIA-074: it does not reopen ADR-0086 (Postgres crosses ADR-0003) but the ADR's "Rust second-tier" survey now names it wrongly. BP-RUST-02/-15 (Microsoft Pragmatic Rust
Guidelines, 2026-08-19, *do* rank the fake-vs-real dispatch choice and *do* address anyhow in in-repo crates) → PIA-045, PIA-079. BP-RUST-12 (edition-2024 resolver 3 is MSRV-aware only when
`package.rust-version` is set) → PIA-027. URLs and fetch dates: https://arxiv.org/html/2603.18740, https://github.github.io/spec-kit/reference/workflows.html, https://github.com/coleam00/Archon,
https://raw.githubusercontent.com/sipyourdrink-ltd/bernstein/main/README.md, https://crates.io/api/v1/crates/dbos, https://microsoft.github.io/rust-guidelines/print.html,
https://doc.rust-lang.org/cargo/reference/resolver.html (all 2026-09-10).

### 6.4 First verification of self-flagged claims that FAILED

Twenty-six of the 101 self-flagged rows came back DRIFTED (20) or CONTRADICTED (6). This is the corpus's own honesty paying off: every one of these sat under a marker the corpus wrote about itself
("UNVERIFIED", "URL not returned", "could not be corroborated", "not checked"), so a reader who trusted the markers was never misled, and each marker can now be discharged with a fact instead of
standing indefinitely. A marker that is never revisited is a debt; these are the ones that came due. Fetch date 2026-09-10 throughout.

CONTRADICTED (6):
- EXT-CLAUDECO-030 — `defer`/`tool_deferred` "could not be corroborated in first-party docs" → shipped 2.1.89 (2026-03-31), documented. https://code.claude.com/docs/en/hooks. PIA-064. (blocks-decision)
- EXT-STATEDAB-027 — same mechanism, "Changelogs were not checked" → CHANGELOG records it; issue #41791 was a docs-omission report, since remedied. https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md. PIA-064.
- EXT-ANTHROPI-002 — "no limits table was found ... house rules, not vendor caps" → the Schema complexity limits table exists; 24/16 are per-request vendor constants. https://platform.claude.com/docs/en/build-with-claude/structured-outputs. PIA-005. (load-bearing)
- EXT-STATEDAB-058 — shipmates Cargo.toml "0.1.12 ... UNVERIFIED whether a real gap or a crawl-timing artefact" → 0.1.19 since 2026-09-01; artefact. https://raw.githubusercontent.com/saman-mb/shipmates/main/Cargo.toml. PIA-101.
- EXT-COMPETIT-088 — GitHub Projects cap "UNVERIFIED, GitHub doesn't publish one number" → 50,000 published. https://docs.github.com/en/issues/planning-and-tracking-with-projects/managing-items-in-your-project/archiving-items-from-your-project. PIA-101.
- EXT-DURABLEE-011 — Azure Durable "attempted and not settled" → no token, no deadline, at-least-once. https://learn.microsoft.com/en-us/azure/durable-task/common/durable-task-external-events. PIA-085.

DRIFTED (20):
- EXT-ADVISORI-002, -005, -006, -007 — four gix advisory rows: fixed version attached to the wrong crate; sub-crate column wrong twice; "repo names" for tree/blob names. Pin clear in every case. https://rustsec.org/advisories/RUSTSEC-2025-0021.html and siblings. PIA-083. (all load-bearing)
- EXT-GITOXIDE-005 — clone-by-SHA "confirmed in #2309" → #2309 is a panic; cite `PrepareFetch::with_revision`. https://github.com/GitoxideLabs/gitoxide/discussions/2309. PIA-042. (load-bearing)
- EXT-DURABLEE-001, -012, -013 — DBOS sentinel-not-error; temporalio 0.5.0 → 1.0.0; iopsystems/durable has tags. https://docs.dbos.dev/python/tutorials/workflow-communication, https://crates.io/api/v1/crates/temporalio-sdk, https://api.github.com/repos/iopsystems/durable. PIA-085, PIA-074. (all load-bearing)
- EXT-DECISION-008, EXT-NOURL-011 — Log4brains "URL not returned" → located; it contradicts the NNNN convention it is cited for. https://github.com/thomvaill/log4brains. PIA-023. (load-bearing)
- EXT-NOURL-002 — swatinem.de "URL not returned" → located; supports "pin the toolchain" only. https://swatinem.de/blog/rust-toolchain/. PIA-023. (load-bearing)
- EXT-NOURL-009, EXT-COMPETIT-036, -038 — cloudnuro / sudoall / mintmcp secondaries → pages live, attributed figures and phrases absent. https://www.cloudnuro.ai/blog/llm-cost-management-2026-enterprise-buyers-guide, https://sudoall.com/multi-agent-coordination-2026-playbook/, https://www.mintmcp.com/blog/llm-cost-optimization-agent-teams. No PIA id; the corpus's do-not-quote hedge is binding.
- EXT-COMPETIT-018 — Gas Town "~15.9K stars, UNVERIFIED" → 18,006; release dates, org move and Kilo GA confirm. https://api.github.com/repos/gastownhall/gastown. No PIA id.
- EXT-COMPETIT-068 — Archon #901 "tone/volume UNVERIFIED" → pre-rewrite, no complaints. https://api.github.com/repos/coleam00/Archon/issues/901. PIA-101.
- EXT-COMPETIT-110 — Aeon "six harnesses (UNVERIFIED)" → real, nine harnesses today; wshobson's Gemini CLI row replaced by Antigravity CLI on 2026-08-18. https://github.com/wshobson/agents/blob/main/AGENTS.md. PIA-101.
- EXT-PEERANDS-030, -031 — the 97.6 % row "unverified against the primary PDF" → number confirmed, paper mischaracterised (quick-remedy study quoting Tufano 2019). https://pmc.ncbi.nlm.nih.gov/articles/PMC8553712/. No PIA id.
- EXT-PEERANDS-056 — DA poster "24 goals" → 21 on the poster; 24 on the web page. https://www.pmi.org/-/media/pmi/microsites/disciplined-agile/posters/daprocessgoalsposter85x11.pdf. No PIA id.

The remaining five self-flagged rows that did not confirm are STALE-BY-DATE (EXT-TOOLCHAI-005 rustc channel 1.97 → 1.98.1, PIA-024; EXT-COMPETIT-013 Beads stars; EXT-PROCESSL-072 shipmates) or
UNVERIFIABLE (EXT-CLAUDECO-031 the 98 % compaction figure, no primary exists; EXT-COMPETIT-039 cloudnuro, no URL to fetch) — the corpus's own label was already the correct one.

### 6.5 Confirmed and load-bearing: what was checked and stood

101 of the 144 load-bearing rows held on 2026-09-10. A reader needs the list, because "verified" only means something if the verification is named. Grouped; each line is a live-source reading with URL
and fetch date. Where the corpus's marker was UNVERIFIED, this is the first verification and says so.

Toolchain and crates
- gix 0.87.1 is still the latest (`"max_version":"0.87.1"`, created 2026-08-24, not yanked); no newer release. The exact pin `=0.87.1` is current and clean of every advisory ADR-0084 lists plus the two it omits. https://crates.io/api/v1/crates/gix (2026-09-10).
- No trailer-writing API in gix — **verified absence**, first verification: every public item of gix 0.87.1 (1,533 items), gix-object 0.64.1 (142) and gix-actor 0.42.0 (58) enumerated; "Trailer items with write/add/append/insert/set/interpret in name: none". Only read-side `Trailers`/`TrailerRef`/`BodyRef::trailers()` exist. ADR-0077's shell-out for `git interpret-trailers` is strengthened. https://docs.rs/gix/0.87.1/gix/all.html (2026-09-10).
- crate-status.md still carries "the delicate interplay between GIT_COMMON_DIR and GIT_WORK_TREE isn't implemented" (last change 2026-08-21) and an unchecked "create, move, remove, and repair" worktree entry, independently supporting the worktree shell-out — first verification. https://raw.githubusercontent.com/GitoxideLabs/gitoxide/main/crate-status.md (2026-09-10).
- git-branchless #1334: reference-transaction hooks cannot be atomic and force full-log replays; issue open, verbatim. https://github.com/arxanas/git-branchless/issues/1334 (2026-09-10).
- `async_fn_in_dyn_trait` is "Status: Unstable (nightly only)", tracking issue #133119 open with the stabilisation step unchecked; the 1.97.0 and 1.98.1 RELEASES.md language sections contain no entry for it; async-trait 0.1.92 is live. R-ASYNC's working assumption holds on current stable — first verification of the deferred release-notes read. https://doc.rust-lang.org/nightly/unstable-book/language-features/async-fn-in-dyn-trait.html, https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md (2026-09-10).
- rusqlite 0.40.2 (2026-08-08, MIT) still latest; docs.rs/rusqlite fetches in full (the R-SQLITE "page titles only" gap is dischargeable now); `busy_timeout` defaults to 5000 ms on new connections; sqlite.org WAL page states verbatim "there can only be one writer at a time" — first verification of all four. https://crates.io/api/v1/crates/rusqlite, https://docs.rs/rusqlite/0.40.2/rusqlite/struct.Connection.html#method.busy_timeout, https://www.sqlite.org/wal.html (2026-09-10).
- schemars 1.2.2 latest; enums derive to `oneOf` by default; `#[serde(tag)]` renders a discriminated object; `deny_unknown_fields` sets `additionalProperties: false`. docs.rs adds "The exact structure of generated schemas ... may change between versions of schemars — this is not considered a breaking change", which strengthens the golden-snapshot rationale. https://docs.rs/schemars/latest/schemars/ (2026-09-10).
- insta 1.48.0, trycmd 1.2.1, assert_cmd 2.2.2, cargo-semver-checks 0.50.0, git-cliff 2.14.1, cargo-dist 0.32.0 (rebranded `dist` at v0.24.0, crate name unchanged, `dist` 0.0.0 is a 2016 stub) — all still `max_version`. https://crates.io/api/v1/crates/{insta,trycmd,assert_cmd,cargo-semver-checks,git-cliff,cargo-dist} (2026-09-10).
- Edition 2024 stable since 1.85.0; `set_var`/`remove_var` newly unsafe (PR #124636 merged); `clippy::correctness` the only deny-by-default group; all five enumerated pedantic lints exist and are grouped `pedantic` in clippy master (a typo would silently no-op in `[workspace.lints.clippy]`); resolver-2 unification within a target confirmed against the Cargo Book; matklad's 3×/5× figures exact; Rust API Guidelines and Microsoft Pragmatic Rust Guidelines both fetch directly (first direct fetch of the former). https://doc.rust-lang.org/edition-guide/rust-2024/, https://raw.githubusercontent.com/rust-lang/rust-clippy/master/README.md, https://doc.rust-lang.org/cargo/reference/features.html#feature-resolver-version-2, https://rust-lang.github.io/api-guidelines/, https://microsoft.github.io/rust-guidelines/ (2026-09-10).

Anthropic platform and Claude Code
- `usage.cache_creation.ephemeral_1h_input_tokens` / `ephemeral_5m_input_tokens` are documented today on platform.claude.com, code.claude.com prompt-caching and tool-use-with-prompt-caching ("Claude Code reports one-hour cache writes under ephemeral_1h_input_tokens ..."), so **S-TTL1 stands as a live contradiction** with F3's measured absence — first verification of the documented half. https://platform.claude.com/docs/en/build-with-claude/prompt-caching (2026-09-10).
- Subagents "get five minutes even on a subscription until you choose a longer one" — the docs still say what the corpus says, so the opposite-signed in-session observation is still unreconciled and S-TTL1 remains legitimately open. Cache scope: workspace-level on API/AWS/Foundry, org-level on Bedrock/GCP; "effectively scoped to one machine and directory" in Claude Code; the six-step TTL precedence chain matches verbatim, including `experimental.cacheTtl` (2.1.248). https://code.claude.com/docs/en/prompt-caching (2026-09-10).
- `--permission-prompts none` "Requires Claude Code v2.1.259 or later", CHANGELOG 2.1.259 (npm 2026-09-02) agrees — first verification of a [surface unverified] AC surface. https://code.claude.com/docs/en/cli-reference (2026-09-10).
- `--bare` "never reads OAuth credentials or the system keychain" and "will become the default for `-p` in a future release"; subagents "receive only this system prompt plus basic environment details" — first verification of the `--bare` surface behind ADR-0010. https://code.claude.com/docs/en/sub-agents (2026-09-10).
- "In non-interactive mode (`-p`), the key is always used when present" — ADR-0010's refuse-on-API-key premise is exactly the documented behaviour. https://code.claude.com/docs/en/authentication (2026-09-10).
- SIGTERM → exit 143, no result, process tree killed, only SessionEnd hooks run (AC-225); `--strict-mcp-config` isolates but "doesn't filter servers you pass inline via `--agents`"; `--agents` JSON validated at startup (2.1.242+); per-card `effort` accepted in `--agents`; `--json-schema` and the `StructuredOutput` tool name confirmed (CHANGELOG 2.1.187); F20's tool-restricted inertness is the corpus's own measurement, which the docs neither confirm nor deny. https://code.claude.com/docs/en/headless, https://code.claude.com/docs/en/cli-reference (2026-09-10).
- 2.1.263 and 2.1.265 both exist; 2.1.265 carries subagent prompt-cache-prefix fixes, which is the surface P7 measures, so "arms not instrument-matched" is well founded. Latest is 2.1.268. https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md (2026-09-10).

Durable execution and decision records
- Temporal Rust SDK is a gRPC client of a server; the only server-less path is an ephemeral downloaded dev/test server — the exclusion in ADR-0086 stands (first verification), and the time-skipping test-server open item is now answered yes. Restate awakeables one-shot with no built-in deadline; Trigger.dev 10 m default / 1 h idempotency TTL; Inngest `timeout` required, `null` on expiry; AWS Durable Execution determinism ban list verbatim; GitHub Environments 30-day auto-fail and 43,200-minute cap — all first verifications of URLs the corpus recorded as "not returned". https://github.com/temporalio/sdk-rust, https://docs.restate.dev/develop/ts/external-events, https://trigger.dev/docs/wait-for-token, https://www.inngest.com/docs/reference/functions/step-wait-for-event, https://docs.aws.amazon.com/durable-execution/patterns/best-practices/determinism/, https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/review-deployments (2026-09-10).
- MADR's status field is `{proposed | rejected | accepted | deprecated | ... | superseded by ADR-0123}` on a single record and `date` is "when the decision was last updated"; nothing requires immutability — redkiln's immutability rule is its own, as ADR-0085 says. MADR 4.0.0 released 2024-09-17 (first verification of a search-snippet date). https://raw.githubusercontent.com/adr/madr/main/template/adr-template.md, https://api.github.com/repos/adr/madr/releases (2026-09-10).

Literature behind ADRs
- Petrović et al. ICSE 2021: "The median is 1 for Review and 0 for Submit"; "Mutants are coupled with 70% of high-priority bugs" — R-MUT closed. https://arxiv.org/pdf/2103.07189 (2026-09-10).
- Agentless 32.00 % at $0.70 vs CodeR 28.33 % at $3.34; 213/300 reproduced, 94 flip; FSE 2025 via ACM DOI. https://arxiv.org/html/2407.01489v2 (2026-09-10).
- Nine Judges (2605.29800): n_eff 2.18 [2.07, 2.31], Condorcet gap 8-22 pp, "9.1% error rate on unanimous items". SWE-Gate (2609.04167): 303 instances, 75 repos, 221/644 = 34.3 %. Background Temperature (2604.22411): Table 1 values exact, Claude-Sonnet-4 T_bg = 0 on Bedrock (30 prompts × 100 draws × <=32 tokens — a narrow measurement, carried as such). 2603.00539 is one paper, not two; every number verifies. https://arxiv.org/pdf/2605.29800, https://arxiv.org/pdf/2609.04167, https://arxiv.org/pdf/2604.22411, https://arxiv.org/pdf/2603.00539 (2026-09-10).
- Essence 1.2 (formal/18-10-02) activity spaces with Input / Entry / Completion criteria verbatim; 2.0 beta 2 (2026-03) exists but 1.2 remains formal. ISO/IEC/IEEE 12207:2017 Annex A normative tailoring (verified from the iTeh sample; iso.org 403). EARS RE'09 provenance (first verification). SHOP2 JAIR 2003. https://www.omg.org/spec/Essence/1.2/PDF, https://cdn.standards.iteh.ai/samples/63712/0bc6d9107eeb46d88dbc7b628b12cf4b/ISO-IEC-IEEE-12207-2017.pdf, https://research.manchester.ac.uk/en/publications/easy-approach-to-requirements-syntax-ears (2026-09-10).

What this list does not say: a CONFIRMED verdict is a reading of a page on one day. Thirteen rows went STALE-BY-DATE inside the 48 hours between the corpus's snapshot and this pass (three crate
versions, two toolchain, two preprint versions, six research-folder facts), which is the measured half-life of a currency fact in this corpus and the reason ADR-0086 forbids citing one by name. The
pins above are current on 2026-09-10; nothing here promises they are current on the day Step 5 runs `cargo build`.

## 7. Against the state of the art

Four lanes re-read the design's external commitments against the live first-party source on 2026-09-10: Rust architecture (20 rows), durable workflow state (20), LLM evaluation (18) and the
SDD/agent-harness landscape (19). Of 77 rows the grounding pass recorded CONFIRMED 50, DRIFTED 18, CONTRADICTED 4, STALE-BY-DATE 4, UNVERIFIABLE 1 [measured]. One of the 50 confirmations does not
survive this synthesis: the SDD lane's BP-SDD-04 said the Claude Code hooks reference documents no `defer` mechanism, and a direct read of `code.claude.com/docs/en/hooks.md` on 2026-09-11 finds it in
25 places (section "Defer a tool call for later", `stop_reason: "tool_deferred"`), as the grounding lanes had already found on 2026-09-10 (EXT-CLAUDECO-030, EXT-STATEDAB-027, both with a
second-opinion re-fetch). Corrected, the lane stands at CONFIRMED 49 / CONTRADICTED 5, and the closing sub-section is about that row. No rate below carries an interval: each lane is a census of its
own rows, not a sample. Where a row says "held", it means the cited page still says what the corpus says it says; it does not mean the design is right.

Fourteen concerns are cited from this section: PIA-013, PIA-023, PIA-027, PIA-028, PIA-042, PIA-045, PIA-056, PIA-064, PIA-074, PIA-079, PIA-085, PIA-090, PIA-105, PIA-108. Thirteen are sourced from
this bucket; PIA-064 is sourced from the grounding lanes and is the one this section turns on. Riders that reached no concern are named as riders, with their source, so that the absence of a concern
is not read as a clean bill.

### 7.1 Rust architecture

What was checked: the twenty external claims behind ADR-0076 (sync ports, async in adapters), ADR-0077 (gix read path, git write path), ADR-0083 (toolchain and lints), ADR-0084 (dependency policy) and
ADR-0086's crate facts, against rustc release notes, the Cargo Book, the Clippy Lints book, the Microsoft Pragmatic Rust Guidelines (page generated 2026-08-19), tokio and rusqlite docs.rs, sqlite.org,
crates.io, RUSTSEC/OSV and the gitoxide crate-status file.

| Design commitment | Current guidance | Verdict | Source (fetched 2026-09-10) |
| --- | --- | --- | --- |
| BP-RUST-01 ADR-0076:19,50: no dyn-compatible native async fn in trait on stable as of 2026-09-08; 1.97 notes unread | Tracking issue #133119 still open and experimental; 1.97.0 and 1.98.0 (stable 1.98.1) stabilised nothing async/dyn | CONFIRMED (first read of the 1.97 line) | github.com/rust-lang/rust/issues/133119; releases.rs 1.97.0, 1.98.0 |
| BP-RUST-02 ADR-0076:24: every port one object-safe trait object; "no guidance exists" for hexagonal in Rust | No ports-and-adapters page, but M-DI-HIERARCHY ranks enum > generics > dyn for the fake-vs-real case and M-AVOID-WRAPPERS wants dyn behind a newtype | DRIFTED | microsoft.github.io/rust-guidelines |
| BP-RUST-03 ADR-0076:52-53: adapter owns a runtime handle built at the composition root, block_on inside sync methods | Tokio's bridging guide recommends exactly this; Handle::block_on panics inside an async context | CONFIRMED | tokio.rs/tokio/topics/bridging; docs.rs tokio 1.53.1 |
| BP-RUST-04 ADR-0076:57: dropping a boxed future does not reap a process tree; cancel is an explicit kill [inferred] | Dropped Child does not kill by default; kill_on_drop kills that child only; no process-group primitive | CONFIRMED (direct child now [documented]) | docs.rs/tokio/latest/tokio/process/struct.Child.html |
| BP-RUST-05 ADR-0084:60: gix = "=0.87.1" | 0.87.1 is max_version, not yanked | CONFIRMED | crates.io/api/v1/crates/gix |
| BP-RUST-06 ADR-0077:21,96: GIT_COMMON_DIR/GIT_WORK_TREE interplay not implemented; worktree add/remove shells out | Sentence present verbatim (line 197); worktree create/move/remove/repair is an unchecked box (line 188) | CONFIRMED | raw.githubusercontent.com/GitoxideLabs/gitoxide/main/crate-status.md |
| BP-RUST-07 ADR-0077:23,97: crate-status documents trailer parsing; no write API found; docs.rs unfetched | crate-status lists "parse trailers" UNCHECKED (line 259); docs.rs gix-object 0.64.1 exposes BodyRef::trailers(); no write method exists | DRIFTED | docs.rs/gix-object/latest/.../BodyRef.html |
| BP-RUST-08 ADR-0084:23,90: gix-* advisories fixed at or before 0.87.1; rustsec.org 404'd | rustsec.org resolves; OSV: zero advisories for gix 0.87.1; latest gix-* advisory 2025-04-04 | CONFIRMED | rustsec.org; api.osv.dev/v1/query |
| BP-RUST-09 ADR-0084:27,93: WAL, readers do not block writers, still one writer (UNVERIFIED, R-SQLITE) | One writer at a time under WAL; same host only, no network filesystem; readers can see SQLITE_BUSY in obscure cases | CONFIRMED (first first-party read) | sqlite.org/wal.html |
| BP-RUST-10 ADR-0084:27,63: rusqlite 0.40.2, WAL + busy_timeout, one writer connection | 0.40.2 current; busy_timeout defaults to 5000 ms; Connection is Send, !Sync | CONFIRMED | docs.rs/rusqlite/latest/rusqlite/struct.Connection.html |
| BP-RUST-11 ADR-0083:47: edition = "2024" inherited from the workspace | Edition 2024 stable since 1.85.0; M-LATEST-EDITION says use it; resolver field unnecessary | CONFIRMED | microsoft.github.io/rust-guidelines; blog.rust-lang.org 1.85.0 |
| BP-RUST-12 ADR-0083:49: rust-toolchain.toml pins; package.rust-version "is not used as a substitute" | Resolver 3 (edition 2024) is MSRV-aware only when package.rust-version is set; M-MSRV: set it at crate creation | DRIFTED | doc.rust-lang.org/cargo/reference/resolver.html |
| BP-RUST-13 ADR-0083:53: [workspace.lints] with [lints] workspace = true in every member | Cargo Book documents the mechanism; M-CARGO-WORKSPACE recommends it plus default-features = false | CONFIRMED | doc.rust-lang.org/cargo/reference/workspaces.html |
| BP-RUST-14 ADR-0083:55-58: clippy::nursery = "warn" as a group, "nursery lints can change or be removed without notice"; overrides as commented #![allow] | Book: nursery is "buggy or need more work ... not recommended to enable the whole group"; the quoted phrase is not in the book; #[expect] stable since 1.81 | DRIFTED | raw.githubusercontent.com/rust-lang/rust-clippy/master/book/src/lints.md |
| BP-RUST-15 ADR-0083:60,109: thiserror in libraries, anyhow in binaries; "no first-party source" | M-APP-ERROR permits anyhow in in-repo application-only crates; M-ERRORS-CANONICAL-STRUCTS for libraries | DRIFTED | microsoft.github.io/rust-guidelines |
| BP-RUST-16 ADR-0083:29,63: set_var/remove_var unsafe in edition 2024; banned workspace-wide | Edition Guide confirms the promotion and the multithreading rationale | CONFIRMED | doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html |
| BP-RUST-17 ADR-0083:31,62: one tests/it/main.rs per crate; matklad 3x / 5x | Numbers and layout verbatim; M-INTEGRATION-TESTS prefers integration over inline unit tests when either serves | CONFIRMED | matklad.github.io/2021/02/27/delete-cargo-integration-tests.html |
| BP-RUST-18 ADR-0083:77 (S-FEAT): --workspace unifies features; isolation via -p or identical sets | Confirmed; the fix is resolver.feature-unification = "package" (RFC 3692, cargo #14774), unstable | CONFIRMED | doc.rust-lang.org/cargo/reference/unstable.html |
| BP-RUST-19 ADR-0086:20,89: temporalio-sdk on sdk-core v0.5.0, Public Preview; graph-flow v0.6; test server unchecked | temporalio-sdk 1.0.0 (2026-09-04), graph-flow 0.8.0; `testing` feature starts a local CLI dev server (a child process, not embedded) | STALE-BY-DATE | crates.io; raw.githubusercontent.com/temporalio/sdk-rust/main/crates/sdk/README.md |
| BP-RUST-20 ADR-0084:66 / design 9.2 table: jsonschema 0.55.1, tokio 1.53.1, thiserror 2.0.20, anyhow 1.0.104, schemars 1.2.2, insta 1.48.0 | jsonschema 0.56.0 published 2026-09-10; every other version still max_version | STALE-BY-DATE | crates.io/api/v1/crates/jsonschema |

What drifted, and what it means. Five drifts and two stale-by-date rows; none moves a decision, four move a Step-5 manifest line. The lint clause is the one with build consequence: ADR-0083 enables
`clippy::nursery = "warn"` as a group "on the documented grounds that nursery lints can change or be removed without notice" (references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:67), and
the book it cites says the opposite thing in different words: "It is **not** recommended to enable the whole group, but rather cherry-pick lints" [documented,
raw.githubusercontent.com/rust-lang/rust-clippy/master/book/src/lints.md, 2026-09-10]. The quoted rationale is in no Clippy text. That is PIA-023 (P2, SHARPENS-KNOWN: ADR-0083:106 records the
group-deny hazard, not the misquote or the warn-group problem); the remediation is a superseding lint clause that drops the nursery group, enumerates any nursery lints wanted, and permits
`#[expect(lint, reason = "...")]` as the only in-source override. The pin-versus-floor row is the second: ADR-0083 rejects `package.rust-version` because
"Nobody consumes these crates ... only the pin answers it" (references/adr/0083-rust-workspace-toolchain-and-lint-policy.md:47), which is right as far as it goes, but the Cargo Book makes the two
complementary: resolver 3 "Change[s] the default for resolver.incompatible-rust-versions from allow to fallback", and the fallback has nothing to act on unless rust-version is set [documented,
doc.rust-lang.org/cargo/reference/resolver.html, 2026-09-10]. PIA-027 (P2, SHARPENS-KNOWN) adds `workspace.package.rust-version` equal to the pinned channel, stated as arming the resolver rather than
as a consumer floor. The third with a manifest consequence is PIA-028 (P2, NEW): ADR-0084:58 says "other 0.x dependencies adopted later inherit the same rule" as gix's exact pin, and the design's
section 9.2 table puts jsonschema (0.55.1, now 0.56.0 [documented, crates.io, 2026-09-10]), rusqlite 0.40.2, chrono 0.4.45 and sha2 0.11.0 on caret ranges; the rule and the table cannot both be built
from. The remaining drifts are citation truth. ADR-0083:72 adopts the thiserror/anyhow split "corroborated by two 2026 practitioner blogs and by **no first-party source**", and Microsoft's M-APP-ERROR
/ M-ERRORS-CANONICAL-STRUCTS is a first-party vendor source that largely agrees and names one divergence (in-repo application-only crates may use anyhow) [documented,
microsoft.github.io/rust-guidelines/print.html, generated 2026-08-19, fetched 2026-09-10]; PIA-079 (P2, SHARPENS-KNOWN) re-tags the evidence and keeps the stricter house rule with its reason.
ADR-0076's "no sourced guidance" framing for the dyn port boundary is half wrong in the same way: there is no ports-and-adapters page, but M-DI-HIERARCHY
("Prefer types over generics, generics over dyn traits") addresses exactly the fake-vs-real dispatch question and ranks dyn last, behind a newtype [documented, same page]; PIA-045 (P2, NEW) supersedes
ADR-0076's evidence block to cite the ranking and state why the design departs from it (sync traits, five ports, one composition root, scripted fakes). And ADR-0077:23 rests the read-side bet on
"`crate-status.md` documents trailer *parsing*" when that file lists the box unchecked at line 259 while the API exists on docs.rs (gix-object 0.64.1, `BodyRef::trailers`) [documented, 2026-09-10];
PIA-042 (P2, SHARPENS-KNOWN) re-cites to docs.rs and discharges S-GIX/b's read half, leaving its executed-test half owed. The two stale-by-date rows are pre-declared currency (ADR-0084:104 forbids
citing the snapshot after its date); the temporal row is absorbed into PIA-074 below.

Riders that reached no concern, carried here so they are not lost [documented, all 2026-09-10]: with a `current_thread` runtime, spawned background tasks freeze between `block_on` calls, so an adapter
that spawns a stream reader needs `multi_thread` or the dedicated-thread pattern (tokio.rs/tokio/topics/bridging); rusqlite's `Connection` is `!Sync`, so "one writer connection" must be thread-owned
or behind a mutex/channel and plural readers are one-per-thread (docs.rs/rusqlite); WAL "does not work over a network filesystem", so a repo whose common dir sits on a share breaks the store and
`doctor` should check the mount, and readers can see `SQLITE_BUSY`, so `busy_timeout` belongs on reader connections too (sqlite.org/wal.html); the async pin target should now read 1.98.x, not 1.97
(releases.rs); and S-FEAT's future closure has a name, RFC 3692 / cargo #14774, to be checked at each pin-bump review (doc.rust-lang.org/cargo/reference/unstable.html).

What held: 13 of 20, including every pin's currency (gix 0.87.1, rusqlite 0.40.2, tokio 1.53.1), the advisory clearance (OSV: zero for gix 0.87.1), the bridging shape, the WAL one-writer rule (first
read from sqlite.org rather than a search result), the set_var ban, the single test binary, and the GIT_COMMON_DIR sentence verbatim.

### 7.2 Durable workflow state

What was checked: ADR-0086's exclusion of durable-execution frameworks and every fact it names (Temporal, graph-flow, iopsystems/durable, AWS Durable Execution, DBOS); the design's section 6.6 survey
of gate-token semantics across Step Functions, Trigger.dev, Inngest, DBOS, Restate and GitHub Environments; the determinism-split idiom; and the git and gitoxide invariants the store placement and
trailer write path lean on, against vendor docs, crates.io, the GitHub API and the Azure event-sourcing pattern page.

| Design commitment | Current guidance | Verdict | Source (fetched 2026-09-10) |
| --- | --- | --- | --- |
| BP-DUR-01 ADR-0086:90: temporalio-sdk on sdk-core v0.5.0, June 2026, Public Preview, "read 2026-09-08" | 1.0.0 on sdk-core 0.9.0, published 2026-09-04, "the Rust SDK is now considered stable"; 0.6, 0.7, 0.8, 1.0 all predate the read | DRIFTED | crates.io/api/v1/crates/temporalio-sdk; GitHub release v1.0.0 |
| BP-DUR-02 ADR-0086 surviving fact: Temporal Rust SDK requires a server or Cloud over gRPC, no embedded mode | Still true at 1.0.0; the only local option is the `testing` feature, which downloads and manages a Temporal CLI dev server | CONFIRMED | raw.githubusercontent.com/temporalio/sdk-rust/main/crates/sdk/README.md |
| BP-DUR-03 ADR-0086 open item: time-skipping local test server unchecked | Exists since 0.7.0: WorkflowEnvironment::start_local, ActivityEnvironment, core TestServerConfig; managed child processes | CONFIRMED (first verification of the registered item) | api.github.com/repos/temporalio/sdk-rust/releases |
| BP-DUR-04 ADR-0086:20: graph-flow v0.6 makes postgres optional, InMemorySessionStorage, no required database | 0.8.0 current; postgres optional but ENABLED BY DEFAULT ("set default-features = false to exclude it"); in-memory does not survive restarts | DRIFTED | crates.io/api/v1/crates/graph-flow |
| BP-DUR-05 ADR-0086:43: iopsystems/durable "no version or release tags visible", ~58 stars | Zero Releases, but tags v0.6.2, durable-v0.6.0, durable-v0.5.10; 58 stars; Postgres + WASM/WASI confirmed | DRIFTED | api.github.com/repos/iopsystems/durable/tags |
| BP-DUR-06 ADR-0086: AWS Durable Execution is TS/Python/Java/C#, not Rust; ban list Date.now/UUID/HTTP/random/branching | Identical list and language set | CONFIRMED | docs.aws.amazon.com/durable-execution/patterns/best-practices/determinism/ |
| BP-DUR-07 ADR-0086 Decision: determinism split adopted as a convergent idiom, not a Temporal-ism | Temporal's workflow-definition page states the same constraints and remedy | CONFIRMED | docs.temporal.io/workflow-definition |
| BP-DUR-08 ADR-0086:32: "Rust remains second-class across every first-party durable-execution SDK checked"; "the server is the arbiter" | Temporal Rust SDK GA; DBOS Inc. published first-party `dbos` 0.5.0 (2026-09-09): a library, no server, database arbitrates, hard-requires sqlx/postgres | CONTRADICTED | crates.io/api/v1/crates/dbos; docs.rs; dbos.dev |
| BP-DUR-09 design :841: Step Functions "Server-enforced TTL, max activity TTL 6 h" | No 6-hour ceiling; a callback task waits up to the 1-year execution quota; deadline is author-set HeartbeatSeconds/TimeoutSeconds | DRIFTED | docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html |
| BP-DUR-10 design :842: Trigger.dev default 10 m, 1 h idempotency-key TTL, branches on timeout | All three match | CONFIRMED | trigger.dev/docs/wait-for-token |
| BP-DUR-11 design :843: Inngest explicit timeout required, resolves to null | timeout Required; Promise<null or EventPayload> | CONFIRMED | inngest.com/docs/reference/functions/step-wait-for-event |
| BP-DUR-12 design :844/:880: DBOS keyed by (workflowID, key), "DBOS raises an error" on expiry | Keying confirmed; on expiry get_event returns None/null, a sentinel, not an error | DRIFTED | docs.dbos.dev/python/tutorials/workflow-communication |
| BP-DUR-13 design :845: Restate awakeable one-shot, no built-in deadline, pair with a durable timer; mandates a server | Exact match; Restate Server still required; SDK 0.12.0 adds no embedded mode | CONFIRMED | docs.restate.dev/develop/ts/external-events |
| BP-DUR-14 design :846: GitHub Environments approval by a real identity, deadline yes, SHA/ref only | Required reviewer identity; 30-day gate approval cap; bound to run sha and ref | CONFIRMED | docs.github.com/en/rest/actions/workflow-runs |
| BP-DUR-15 design :855: no surveyed system voids a stale-but-unexpired token; only GitHub binds to repo state | Across the six re-fetched platforms nothing binds to a tree hash; only GitHub binds to sha/ref | CONFIRMED (six rows; Azure still unread) | as above |
| BP-DUR-16 design :744: writes via git interpret-trailers --in-place and/or git commit --trailer | Both exist in git 2.54 docs with placement control | CONFIRMED | git-scm.com/docs/git-interpret-trailers |
| BP-DUR-17 ADR-0018 / design 5.7: one store at `git rev-parse --git-common-dir`/redkiln/, shared across worktrees | One common dir shared by all linked worktrees; refs shared except refs/bisect, refs/worktree, refs/rewritten | CONFIRMED | git-scm.com/docs/git-worktree |
| BP-DUR-18 design :744 / ADR-0077: GIT_COMMON_DIR/GIT_WORK_TREE sentence; gix =0.87.1 | Sentence present verbatim; pin current | CONFIRMED | gitoxide crate-status.md |
| BP-DUR-19 design :744 / evidence-augmentation row 8: "crate-status.md documents trailer parsing" | The box is unchecked; the capability exists in gix-object source; no write API found | DRIFTED | gitoxide crate-status.md; gix-object body.rs |
| BP-DUR-20 ADR-0018 / design 5.4-5.8: append-only event log, store as regenerable projection, reconcile() idempotent | Azure pattern: immutable append-only events, idempotent consumers tracking last-processed sequence, snapshots as optimisations | CONFIRMED | learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing |

What drifted and what was contradicted. The exclusion in ADR-0086 stands, and stands on the same three grounds it names (a server mandate; in-memory state that is not durable; Postgres plus a WASM
toolchain), but the record it stands on was wrong on the day it was written. ADR-0086:90 dates its crates.io read "2026-09-08" and reports
"`temporalio-sdk` on `temporalio-sdk-core` **v0.5.0**, updated June 2026, Public Preview"; on that date the crate was four releases further on, at 1.0.0 with the release note
"With this 1.0.0 release, the Rust SDK is now considered stable" [documented, crates.io and github.com/temporalio/sdk-rust releases, 2026-09-10]. The same record withdraws
"`graph-flow` hard-depends on `sqlx`+postgres" (ADR-0086:20) on the strength of v0.6 making postgres optional, and at the current 0.8.0 the feature is optional but default-on, which is the precise
shape ADR-0086's own feature-flag clause guards against [documented, crates.io graph-flow README, 2026-09-10]. The decision driver at ADR-0086:32,
"Rust remains second-class across every first-party durable-execution SDK checked", is contradicted twice over: by Temporal's GA and by DBOS Inc.'s first-party `dbos` 0.5.0 (2026-09-09), a library
with "no separate service or orchestrator or any external dependencies except Postgres", so "the server is the arbiter" no longer covers the field either [documented, crates.io dbos, docs.rs,
2026-09-10]. `dbos` is excluded by ADR-0003 alone (it hard-requires sqlx/postgres), so nothing reopens; PIA-074 (P2, SHARPENS-KNOWN: known-issues EA-1-005 and ARCH-0.1-017 record the re-argument, not
that its facts were stale on its own date) supersedes ADR-0086 with the live versions, strikes the driver, adds a `dbos` option row rejected on ADR-0003, closes the registered test-server item citing
the v0.7.0 release note, and replaces the :55 instruction to propagate "the facts above" with an instruction to cite the successor. The section 6.6 survey is the second cluster. Design :841 gives Step
Functions a "Server-enforced TTL, max activity TTL 6 h"; the figure is on neither AWS page fetched, the platform ceiling is the one-year execution quota and the deadline is the author's optional
`HeartbeatSeconds`/`TimeoutSeconds` [documented, docs.aws.amazon.com step-functions connect-to-resource, 2026-09-10]. Design :880 says "DBOS raises an error" on expiry; both the Python and TypeScript
docs return `None`/`null`, the Inngest style [documented, docs.dbos.dev, 2026-09-10]. PIA-085 (P2, SHARPENS-KNOWN: EA-5-009 records the Azure row as unread; the 6 h figure and the DBOS style are
unrecorded) amends the table by dated amendment; the six conclusions the design draws from the table are unaffected, because the one that matters,
"No surveyed system invalidates a stale-but-unexpired token automatically" (design :855), re-verified across all six rows. The trailer-parsing citation (BP-DUR-19) is the same defect as BP-RUST-07 and
is carried in PIA-042.

One rider with implementation consequence reached no concern and is recorded here [documented, git-scm.com/docs/git-interpret-trailers, 2026-09-10]: `git interpret-trailers` defaults `ifExists` to
`addIfDifferentNeighbor`, so a repeated identical `Gate:` or `Slice-Verdict:` trailer is silently not appended unless `--if-exists=add` is passed. The corpus does not mention this, and the append-only
intent of the tier-1 axes depends on it; it belongs in rk-git's write contract before Step 6 writes a trailer. A second rider: the three ref namespaces git does not share across worktrees
(refs/bisect, refs/worktree, refs/rewritten) are worth one line in rk-git, though the design keys on SHA and tree hash and is unaffected.

What held: 13 of 20, including the load-bearing server mandate at Temporal 1.0.0, the convergent determinism idiom from two vendors, five of six survey rows, the differentiator (nothing fetched binds
to a tree hash), both git invariants the store placement and write path rest on, and the event-sourcing shape with its one deliberate inversion (git trailers, not the event store, are the system of
record).

### 7.3 LLM evaluation

What was checked: the literature ADR-0074 (reviewer proposes, machine confirms), ADR-0088 (negative-control arm), ADR-0089/F39 (judge-panel discount), METHOD-P and PRIOR-ART rest on, against the arXiv
full texts (not abstracts), the Cloudflare review post, the Claude Agent SDK structured-output page and crates.io for cargo-mutants.

| Design commitment | Current guidance | Verdict | Source (fetched 2026-09-10) |
| --- | --- | --- | --- |
| BP-EVAL-01 ADR-0074 clauses 1-3: card returns Findings with no pass field; a finding blocks only after the deterministic gate re-executes it | Jin & Chen (2603.00539) and Gajjar (2604.10800: removing execution-grounded validation raises unnecessary repairs +131.7%) make execution the arbiter | CONFIRMED | arxiv.org/html/2603.00539; arxiv.org/abs/2604.10800 |
| BP-EVAL-02 ADR-0074:50: Agentless 213 of 300 reproductions, 94 flipped under the true fix | 213/300 reproduce on the original repository; 94 pass after the gold patch; "failed on the base commit" is loose (213 succeeded at reproducing) | CONFIRMED | arxiv.org/html/2407.01489 s5.1.3 |
| BP-EVAL-03 ADR-0074:36,91: SWE-Gate 221 of 644 functionally-passing repairs fail review constraints; 303 instances, 75 repos | Verbatim (abstract, submitted 2026-09-03) | CONFIRMED | arxiv.org/abs/2609.04167 |
| BP-EVAL-04 ADR-0074:46: high-flip items need ~15 trials for 90% consensus fidelity; 86.6% single trial | Full text s5.9/Table 6: 15 for FR>=10%; 3 for 90% and 11 for 95% overall; 86.6% single trial; judges are GPT-4o-mini/4.1-mini | CONFIRMED | arxiv.org/html/2606.13685 |
| BP-EVAL-05 ADR-0088:3,88: "Measuring and Exploiting Confirmation Bias in LLM-Assisted Security Code Review" | Title is "Contextual Bias"; the phrase "confirmation bias" never appears; v2 2026-04-23 | DRIFTED | arxiv.org/abs/2603.18740 |
| BP-EVAL-06 ADR-0088:19,88: bug-free framing reduces detection "by 16-93%" | 16.2-93.5 percentage points, strong condition only, 5 of 6 models; Opus 4.5 -4.9 pp n.s.; weak framing 0.8-24.7 pp | DRIFTED | arxiv.org/html/2603.18740 |
| BP-EVAL-07 ADR-0088:19,40: "strongly asymmetric: false negatives rise sharply while false positives barely move" | Under strong bug-free framing the Fixed rate on patched code rises +46 to +95 pp: FP collapse. It is bug-PRESENT framing that barely moves, because FP is already >88% at neutral | CONTRADICTED | arxiv.org/html/2603.18740 Table 2 |
| BP-EVAL-08 ADR-0088:89 / ADR-0074:89: one paper carries both the overcorrection result and the fix-guided filter | All numbers re-verify; authors Jin and Chen; venue attribution in evidence-augmentation :307 unverified on the arXiv record | CONFIRMED | arxiv.org/html/2603.00539 |
| BP-EVAL-09 F39 / ADR-0089: 9-judge panel n_eff 2.18, 8-22 points short, unanimity 90.9% | n_eff 2.18 [2.07, 2.31]; unanimous-item accuracy 90.9% vs Condorcet 99.99% | CONFIRMED | arxiv.org/html/2605.29800 |
| BP-EVAL-10 PRIOR-ART s4: kappa (alpha for >2 raters) headline; 0.60/0.80 bands; METHOD-P kill alpha<0.60 | Norman et al.: kappa deflation vs exact match universal at 33-41 pp; Landis-Koch bands agree; band names differ | CONFIRMED | arxiv.org/abs/2606.19544 |
| BP-EVAL-11 charter :221 / PRIOR-ART :22,:120: best automatic LLM-judge 62.5% agreement / kappa 0.37 | No URL in the corpus; targeted search finds no source; nearest studies report kappa 0.43-0.71 | UNVERIFIABLE | (none; search 2026-09-10) |
| BP-EVAL-12 PRIOR-ART s7: registered reports entered SE at MSR 2020; three conferences, two journals | Ernst & Baldassarre, EMSE 2023: exact | CONFIRMED | arxiv.org/abs/2302.03649 |
| BP-EVAL-13 PRIOR-ART :176-177: "an OSF-based registered report template ... whose structure we can borrow" | Bett et al. 2026 present initial results TOWARDS a template; none is shipped | DRIFTED | arxiv.org/abs/2602.09292 |
| BP-EVAL-14 ADR-0084:111 (F20): tools allowlist silently disabled --json-schema; subtype success, structured_output absent | Docs now document that failure shape and say treat it as failure; the allowlist trigger remains undocumented | CONFIRMED | code.claude.com/docs/en/agent-sdk/structured-outputs |
| BP-EVAL-15 ADR-0074:46 rejects majority-of-draws; F27: nothing inferred from a single draw | Coin Flip: 11-15 draws for majority vote; Norman: test-retest >0.95 coexists with severe position bias | CONFIRMED | arxiv.org/abs/2606.13685; 2606.19544 |
| BP-EVAL-16 METHOD-P amendment 9: pointwise labelling, N labellers + adversarial seat, unanimity | Position bias small (<=0.04); style bias (markdown vs prose) dominates at 0.10-0.76 | CONFIRMED (gap: formatting not held constant) | arxiv.org/abs/2604.23178 |
| BP-EVAL-17 ADR-0074 clause 2: mutation-survived iff a mutation applied and the suite stayed green; PRIOR-ART s5 excludes equivalent mutants | cargo-mutants 27.1.0 live; equivalent-mutant detection still an active line (2607.00511) | CONFIRMED (gap: equivalence not in the ADR) | crates.io/api/v1/crates/cargo-mutants |
| BP-EVAL-18 ADR-0074:93: Cloudflare 288 of 48,095 MRs = 0.6%; $1.68 mean / $1.47 median vs $0.98 | All figures verbatim | CONFIRMED | blog.cloudflare.com/ai-code-review/ |

What drifted and what was contradicted. One concern in this lane is P1 and it is the only P1 this section carries: ADR-0088. The record adds a "confirmation-bias arm" to S-SUBTLE citing a paper it did
not read in full (ADR-0088:88, "the per-condition breakdown was not read this pass"), and every one of its three characterisations fails on the full text. The title is
"Measuring and Exploiting Contextual Bias in LLM-Assisted Security Code Review" and the phrase "confirmation bias" never appears [documented, arxiv.org/abs/2603.18740, 2026-09-10]. The headline
"reduces the vulnerability-detection rate by **16-93%**" (ADR-0088:19) is percentage points, under the strong bug-free condition only, on five of six models; the sixth, Claude Opus 4.5, drops 4.9 pp
and not significantly, and the design's section 10.4 register inherits the band (design :1832). And the mechanism is inverted: the ADR says the effect is
"strongly asymmetric: false negatives rise sharply while false positives barely move" (ADR-0088:19) and rejects the symmetric already-rejected arm partly because
"the source finds false positives barely move under framing manipulation" (ADR-0088:46), whereas Table 2 shows the Fixed rate on patched code rising +46 to +95 pp under strong bug-free framing, so
false positives collapse with detection, and it is bug-present framing that barely moves anything, because "All models exhibit very high false positive rates (>88%) even under neutral conditions"
[documented, arxiv.org/html/2603.18740, 2026-09-10]. The decision survives, because detection does drop; the arm's name, the ADR's title, the effect size and one rejected-option ground do not. PIA-013
(P1, SHARPENS-KNOWN; blocks Step 7c and the S-SUBTLE pre-registration) supersedes ADR-0088 with the real title, a renamed arm, the per-condition effect, the whole-classifier-shift mechanism with a
CLEAN-pair approval rate reported alongside detection, and the symmetric arm re-argued on cost alone or registered as a named spike. The two remaining drifts are lineage hygiene in PRIOR-ART: the
"62.5% agreement / kappa 0.37" ceiling the charter states "in every write-up" (references/2026-08-31-rust-inversion-spike-charter.md:221) has no source in the corpus and could not be attributed live,
which is UNVERIFIABLE, not wrong, but it is load-bearing for s4's claim that the automatic ceiling sits below the 0.60 floor; and the
"OSF-based registered report template for SE controlled experiments whose structure we can borrow" (spikes/PRIOR-ART.md:175-176) is a gap analysis that ships no template [documented,
arxiv.org/abs/2602.09292, 2026-09-10]. Both are PIA-108 (P3, NEW), discharged by dated amendment. Two gaps surfaced under CONFIRMED rows and one was promoted to a concern because it has build
consequence: an equivalent mutant survives re-execution exactly as a real one does, so ADR-0074's `mutation-survived` class, "iff a mutation was applied and the named suite stayed green"
(ADR-0074:57), can block on a no-op mutation the machine cannot filter, while PRIOR-ART s5 already carries the equivalence rule for the measurement denominator (spikes/PRIOR-ART.md:234) and the ADR
does not; PIA-056 (P2, SHARPENS-KNOWN) requires a reverted-fix mutant or a distinguishing assertion. The style-bias gap (BP-EVAL-16: neither METHOD-P's panel nor the review card holds markdown/prose
framing constant, and style is the largest measured judge bias in 2026 at 0.10-0.76) reached no concern and is carried as a rider for S-SUBTLE's pair construction [documented,
arxiv.org/abs/2604.23178, 2026-09-10]. A second rider: evidence-augmentation :307's venue attribution for Jin & Chen ("Automated Software Engineering (Springer) 2026 / ASE 2026") is on no arXiv record
fetched and should carry [preprint] only.

What held: 13 of 18, and the ones that carry the design's weight all held on the full text rather than the abstract: execution-as-arbiter (two independent 2026 lines), the Agentless and SWE-Gate
counts, the coin-flip trial counts, the n_eff 2.18 panel discount, the kappa bands, the majority-vote rejection, the Cloudflare comparator verbatim, and F20's failure shape now in vendor docs.

### 7.4 SDD and agent-harness landscape

What was checked: evidence-augmentation's seven landscape claims and its competitor watchlist (spec-kit, OpenSpec, BMAD, task-master, Archon, Beads, superpowers, Vibe Kanban, Factory, Symphony,
Cursor, Jira), the Anthropic surfaces the design frames itself against (hooks, Agent SDK permissions, Dynamic Workflows, Routines, Managed Agents, the CLI pin), and GitHub's agentic-workflow preview,
against first-party docs, READMEs and the GitHub API.

| Design commitment | Current guidance | Verdict | Source (fetched 2026-09-10) |
| --- | --- | --- | --- |
| BP-SDD-01 evidence-augmentation :123: spec-kit/OpenSpec/BMAD/task-master, "None ships an item-kind taxonomy, a stage-gate state machine, or durable approval on a work item" | spec-kit 1.0.x ships a workflow engine with a `gate` step (approve/reject), persisted run state under .specify/workflows/runs/<run_id>/ and `specify workflow resume`; PR #2158 merged 2026-04-14 | CONTRADICTED (already false on 2026-09-08) | github.github.io/spec-kit/reference/workflows.html |
| BP-SDD-02 evidence-augmentation :124: "no AI coding tool ships stage-gated, mid-run, resumable approval on a durable item" | Archon has an `approval` node type, `archon workflow approve/resume/reject`, runs persisted in its DB (v0.4.0, 2026-05-28) | CONTRADICTED | github.com/coleam00/Archon |
| BP-SDD-03 evidence-augmentation :123: star counts and the task-master stall (2026-04-28) | 135,293 / 67,918 / 52,875 / 28,064; stall confirmed at 4.5 months; no gate added in OpenSpec 1.13.0 or BMAD 6.12.0 | STALE-BY-DATE | api.github.com/repos/github/spec-kit et al. |
| BP-SDD-04 evidence-augmentation :124: hook decisions "allow/deny only"; no first-party `tool_deferred` | Lane recorded "no defer/pause/tool_deferred mechanism documented". OVERTURNED: hooks.md documents `permissionDecision` (allow/deny/ask/defer), "Defer a tool call for later", `stop_reason: "tool_deferred"`; CHANGELOG 2.1.89 | CONTRADICTED (lane said CONFIRMED) | code.claude.com/docs/en/hooks.md (re-read 2026-09-11); raw CHANGELOG.md |
| BP-SDD-05 evidence-augmentation :124: Agent SDK canUseTool and Codex approval modes are session-scoped | canUseTool is an in-process callback in a six-step evaluation; the permissions page documents no persisted pending permission | CONFIRMED (narrowed by BP-SDD-04: the hooks page does) | code.claude.com/docs/en/agent-sdk/permissions.md |
| BP-SDD-06 architecture-proposal :423 / brainstorm :22: Dynamic Workflows, "No mid-run user input ... run each stage as its own workflow"; resume in-session | Unchanged; workflows saveable to .claude/workflows/ and distributable in plugins; 1,000 agents/run | CONFIRMED | code.claude.com/docs/en/workflows |
| BP-SDD-07 design :9,226: no Anthropic surface owns a stage-gated, durable, item-scoped loop | Routines (research preview): unattended, "no approval prompts during a run", fresh session per event | CONFIRMED (first check; corpus does not analyse routines) | code.claude.com/docs/en/routines.md |
| BP-SDD-08 same umbrella claim | Managed Agents (beta): server-persisted session that "resume[s] cleanly after pauses", steer/interrupt events; no item, stage or approval concept | CONFIRMED (absent from the corpus) | platform.claude.com/docs/en/managed-agents/overview |
| BP-SDD-09 design :359,1346: TESTED_CLI_VERSION 2.1.263 | Latest 2.1.268 (2026-09-10); no new loop/gate primitive in .266-.268; 2.1.268 gates TaskCreate/TodoWrite to older models unless CLAUDE_CODE_ENABLE_TODO_TOOLS=1 | STALE-BY-DATE | api.github.com/repos/anthropics/claude-code/releases |
| BP-SDD-10 evidence-augmentation :38,220: Agents in Jira "GA all Cloud plans", native Approval step | GA May 2026 confirmed; native approvals are Premium and Enterprise only; the community post cited does not mention approvals | DRIFTED | atlassian.com/software/jira/guides/agentic-engineering/human-in-the-loop |
| BP-SDD-11 evidence-augmentation :128: OpenSpec "fluid not rigid - no phase gates, work on any artifact at any time"; Agent OS v3 retired orchestration phases | Wording is "work on what makes sense"; anti-gate stance unchanged at 1.13.0; Agent OS 3.0 changelog confirms | DRIFTED (paraphrase quoted as verbatim) | raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md |
| BP-SDD-12 evidence-augmentation :126: Vibe Kanban commercially dead, technically alive | Shutdown post confirmed; zero pushes since 2026-04-24 | CONFIRMED | vibekanban.com/blog/shutdown |
| BP-SDD-13 evidence-augmentation :221: Beads, Dolt-backed, bd ready/claim/close, not a gate system | Unchanged; the command is `bd update <id> --claim` | CONFIRMED | raw.githubusercontent.com/gastownhall/beads/main/README.md |
| BP-SDD-14 evidence-augmentation :222: superpowers 283,457 stars, no control flow | 284,843; no release since v6.3.0; no state ownership | CONFIRMED | api.github.com/repos/obra/superpowers |
| BP-SDD-15 evidence-augmentation :125: Factory retired Reliability Droid; CLI v0.209.0 | v0.209.0 (2026-09-01) still latest; cited changelog paths 404, live path is /changelog/release-notes | CONFIRMED | docs.factory.ai/changelog/release-notes |
| BP-SDD-16 design :1614,2050: Symphony as the anti-pattern (tracker-as-state-machine) | SPEC.md confirms; its "Human Review" handoff is a durable human gate on a work item, implemented by the tracker | CONFIRMED | raw.githubusercontent.com/openai/symphony/main/SPEC.md |
| BP-SDD-17 research 2026-09-03 :53,94: gh-aw and Agent HQ preview-stage; build on the GA set | gh-aw now "public preview" (v0.89.3); third-party agents still public preview; "Copilot code review can now approve pull requests" (2026-09-01) | DRIFTED (label moved) | docs.github.com/en/actions/tutorials/develop-agentic-workflows-in-github-actions |
| BP-SDD-18 evidence-augmentation :124: Cursor headless has no HITL approval | Propose-or-force; no mid-run approval | CONFIRMED | cursor.com/docs/cli/headless |
| BP-SDD-19 process-catalog proposal :46: "nobody combines an item-kind taxonomy, an executable activity catalog, and deterministic selection" | Bernstein (created 2026-03-22, 1,158 stars) absent from the corpus: deterministic non-LLM scheduler, task kinds, policy-declared approval and merge gates, replay journal, worktree-per-task, checkpointed resumable runs | DRIFTED (unlisted analogue) | raw.githubusercontent.com/sipyourdrink-ltd/bernstein/main/README.md |

What drifted and what was contradicted. The competitor claims were narrowed once already, and the narrowing did not go far enough. Evidence-augmentation :123 says of spec-kit, OpenSpec, BMAD and
task-master that "None ships an item-kind taxonomy, a stage-gate state machine, or durable approval on a work item", and :124 that
"no AI coding tool ships stage-gated, mid-run, resumable approval on a durable item"; design :854 already retreats to
"**in an AI-coding-specific backlog or spec tool**, and on the *binding* rather than the gate". All three were false at the 2026-09-08 snapshot, not by drift: spec-kit's workflow engine has a `gate`
step ("Pause for human approval before continuing"), persisted paused runs and `specify workflow resume <run_id>`, merged 2026-04-14 [documented, github.github.io/spec-kit/reference/workflows.html,
2026-09-10]; Archon has an `approval` node with `archon workflow approve/resume/reject` and DB-persisted runs since v0.4.0 (2026-05-28) [documented, github.com/coleam00/Archon, 2026-09-10]; and
Bernstein, absent from the watchlist, is a deterministic non-LLM scheduler with task kinds, "role fences and approval gates [that] hold no matter what the agent does inside the task", a replay
journal, worktree-per-task and `bernstein workflow resume <run_id>` [documented, its README, 2026-09-10], which is the closest thing to the kernel pitch anyone has shipped. What survives is narrower
than any wording the corpus has used: no surveyed tool attaches a persisted human gate to a durable work item that carries a kind and a per-kind stage machine, and none binds the answer to a tree
hash. PIA-090 (P2, SHARPENS-KNOWN: design :854 and evidence-augmentation :38 record the first narrowing) rewords to that claim, adds spec-kit, Archon and Bernstein rows (plus
Untrivial-ai/agent-orchestrator and microsoft/conductor) to the watchlist with fetch dates, and puts Bernstein's human-or-automated question on the next-pass list rather than into prose. PIA-105 (P3,
NEW) collects the three citation defects with no consequence: the Jira approval step is Premium/Enterprise, not "all Cloud plans", and the community post cited does not mention it; OpenSpec's wording
is "work on what makes sense", not "any artifact at any time"; Factory's changelog path has moved. The Anthropic-surface rows all held in the direction the design wants, with one omission and one
drift: Managed Agents (beta), the nearest Anthropic thing to a durable harness
("sessions are long-running, resume cleanly after pauses, and store conversation history, sandbox state, and outputs server-side" [documented, platform.claude.com/docs/en/managed-agents/overview,
2026-09-10]), is nowhere in the corpus and should be named as an alternative dispatch target rather than left unmentioned; this is a rider, not a concern, because it changes no decision. The CLI moved
2.1.263 to 2.1.268, expected under an exact pin, but 2.1.268 offers TaskCreate/Get/Update/List and TodoWrite "only on Claude 3.x, Opus 4.0-4.7, Sonnet 4.0-4.6, Haiku 4.5" unless
`CLAUDE_CODE_ENABLE_TODO_TOOLS=1` [documented, raw CHANGELOG.md, 2026-09-10], which matters for any activity card that leans on those tools or on TaskCreated/TaskCompleted hooks under a newer model
[inferred]; carried as a rider against the pin-bump review.

What held: 11 of 19 as the lane recorded it, 10 after the BP-SDD-04 correction: Dynamic Workflows' no-mid-run-input rule, Routines and Managed Agents as gate-free, Cursor headless as propose-or-force,
Symphony's characterisation, the Vibe Kanban shutdown, and the Beads, superpowers and Factory watchlist rows.

### 7.5 The item that changes the framing: Claude Code `defer`

Evidence-augmentation :124 states that "Claude Code hook permission decisions are allow/deny only" and that
"**Several 2026 blogs claiming an April-2026 `tool_deferred` mechanism could not be corroborated in first-party docs** - treat as unverified", and :283 records
"the current first-party hooks doc shows no trace. Changelogs were not checked." The corpus self-flagged this; the first verification fails it on both halves, and this synthesis re-read the page on
2026-09-11 to settle the disagreement between its own lanes. `PreToolUse` accepts `permissionDecision` (allow/deny/ask/defer) with precedence "`deny` > `defer` > `ask` > `allow`";
"`"defer"` is for integrations that run `claude -p` as a subprocess and read its JSON output";
"The tool doesn't execute. The process exits with `stop_reason: "tool_deferred"` and the pending tool call preserved in the transcript"; the caller resumes with "`claude -p --resume <session-id>`" and
"The same tool call fires `PreToolUse` again";
"There is no timeout or retry limit. The session remains on disk until you resume it, subject to the `cleanupPeriodDays` retention sweep, which deletes session files after 30 days by default";
"`"defer"` only works when Claude makes a single tool call in the turn"; a missing tool on resume exits with `stop_reason: "tool_deferred_unavailable"`; and a `-p` resume
"doesn't restore any other stored permission mode" [documented, code.claude.com/docs/en/hooks.md, 2026-09-11]. The CHANGELOG files it under 2.1.89:
"Added `"defer"` permission decision to `PreToolUse` hooks - headless sessions can pause at a tool call and resume with `-p --resume` to have the hook re-evaluate" [documented,
raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md, 2026-09-11; npm-published 2026-03-31 per EXT-CLAUDECO-030]. A Wayback capture of 2026-09-07 already carries the section
(EXT-STATEDAB-027), so the corpus's "shows no trace" was false on its own date, and the pinned `TESTED_CLI_VERSION` 2.1.263 (design :359) is 174 releases past 2.1.89: the harness the product
dispatches to has carried this primitive throughout the spike programme.

What it overlaps with ADR-0017's mark, stated precisely. ADR-0017:35 defines
"The gate record is `(token, tree_hash, deadline)` - a single-use token, the tree hash the human saw, and an expiry with a terminal path (fail / escalate / auto-decide, never a hang); an answer whose tree hash no longer matches is void, not reconciled."
Against the three components: token, none. Defer mints no token; the identity of the pause is the session id plus the pending `deferred_tool_use.id` in the transcript, and it is not single-use,
because "the hook can return `"defer"` again" without limit. Tree hash, none. Nothing binds the pending call to repository state; `tool_deferred_unavailable` fires when the tool's MCP server is gone,
not when the worktree moved, so a stale answer is applied, not voided. Deadline, the opposite shape. "There is no timeout" and no terminal path; the only expiry is the 30-day session sweep, which
deletes the record rather than reaching fail / escalate / auto-decide, which is the hang ADR-0017 rules out, silently resolved by garbage collection. On the three named components the overlap is nil,
and design :854's claim that "redkiln binds the answer to a **tree hash**, which no surveyed system does" survives with defer added to the survey. What defer does overlap is the property ADR-0087
priced only two ways: the pause is process-independent. ADR-0087:38-40 rejects "hold the process open across the gate" and rules
"Nothing may depend on a process surviving; every wait is a durable mark plus a later read", and :47 requires that "Resume must cross a real process boundary". Defer satisfies both: the `claude`
process exits, the mark is durable (on disk, in Claude's session store under `~/.claude`, outside the repo's common dir and outside rk-store), and the resume is a fresh process keyed by session id. It
is a durable mark at tool-call grain inside one harness session, not at step grain on a work item, and it is confined to `-p` runs with a single tool call in the turn. So the record it bears on is not
ADR-0017 but ADR-0087 and design :1580, where "The `PreToolUse` deny survives as a **stateless predicate**" was written on the premise that a hook can only allow or deny; and the section 6.6 survey,
which now omits the one durable per-call pause in the harness actually pinned, with no token, no binding and no deadline, the Restate shape minus the timer. That is PIA-064 (P2, SHARPENS-KNOWN:
EA-5-013 records the question as not_answerable). Its remediation is the house one: a dated erratum to evidence-augmentation :124 and :283; a "Claude Code hook defer" row in the 6.6 table; and a named
spike, S-DEFER, with ADR-0087's successor and rk-adapter-claude's AC-215 stall guard as consumers, that drives a settings-based `PreToolUse` defer under the pinned CLI, kills the parent, resumes in a
fresh process, and records survival, drift and resume cost against F38's 0.45-0.78x figure. Until S-DEFER reports, ADR-0017 and ADR-0087 stand unchanged, and the competitive framing stands as narrowed
by PIA-090: the differentiator is the binding and the per-kind stage machine on a durable item, not the pause, and after this lane the corpus can no longer say that the pause itself is something the
harness cannot do. Nothing else the SDD lane found moved since 2026-09-08 touches the gate layer: three CLI releases with no loop or gate primitive, three SDD releases with no gate, one GitHub change
(Copilot approving PRs) in the reverse direction of the design's gate, and Bernstein, which was there before the snapshot and unread.

## 8. What this audit did not do

Two critics audited the audit: one for coverage (what was never read; `critic-coverage.json`, thirteen limits) and one for method (which modalities never ran; `critic-method.json`, fourteen limits).
Their rows are merged and deduplicated below. Every row is a limit on this review's confidence, not a concern about the corpus; none carries a PIA- id. Where a row touches an area the corpus already
records as absent, the corpus row is cited so that nothing here reads as a discovery. Absence of a finding in any area below is absence of evidence, never a clean bill. Rows are ordered by what the
limit can move: first what Step 5, 7a and 7b build; then the priority ordering and the NEW tag; then the read set; then specific concern classes; then the record of the audit itself.

| limit | consequence | how to close |
|---|---|---|
| **Nothing was compiled** (M-01). The five sync port traits held as dyn objects, the explicit cancel handle and the rk-kernel/rk-ports/rk-types layering were judged by reading design, ADRs and the spike runner's source, with rustc 1.97.1 on the box and `crates/` empty [measured]. The design's own row 279 names "a compile-fail" as the instrument. | Every realisability verdict (14 findings, 3 P1 among them: PIA-001, PIA-007 and PIA-010, the last carrying the Harness trait shape) and every "no defect found" on trait shape — including PIA-005's `StepError` census — is a reading with no compiler behind it. Whether the Harness port can be written as ADR-0076 states it is **not-answerable** from the corpus as written; the one runtime measurement in the class is n=1 on one machine (`taskkill /T /F` victim exit code 1). | A named spike (ADR-0012 discharge; consumer RUNBOOK Step 7a): one throwaway crate of 50–150 lines under scratch, never `crates/`, declaring the five port traits as ADR-0076 and design §3 write them, one fake, a `CancelToken`, and a `compile_fail` doctest for the generics-leak rule; `cargo check` on 1.97.1. One to two hours; zero corpus edits; a failure yields a successor to ADR-0076, never an edit. |
| **No parser was written for the predicate grammar** (M-02). Eight grammar ADRs (0033, 0034, 0035, 0040, 0048, 0049, 0069, 0070) were read for ambiguity; the only executable check in the corpus is the P9 lint's R9, a regex vocabulary scan; precedence is undeclared by design (ADR-0048:21 "grouping is needed for readable `and`/`or` compositions without inventing precedence rules by convention") [measured]. | The grammar findings (PIA-052, PIA-053) and their absences are reading-only. Whether `not a and b or c`, a nested `any(... in ...)` inside `len()`, or a string literal containing `]` parses to one tree is **not-answerable**; Step 7b's lint author and the pack authors will otherwise infer precedence independently. The corpus records the coalesce-chaining and `null == non-null` holes itself (ALREADY-RECORDED at merge); it does not record the missing precedence rule. | A named spike (consumer: Step 7b lint, and a successor to ADR-0033 if a precedence rule is needed): EBNF for ADR-0033+0048+0049+0070, a 150–250 line recursive-descent parser (throwaway, any language), run over every `applicable_if` / `require.when` / `on_false` string in the `spikes/p9` packs and catalog; report the count that fail or parse two ways, with its denominator. Two to four hours. |
| **Refutation was a single vote, and truth and materiality were judged by the same agent in one pass** (M-05, M-14). One refuter per finder, after the planned two-agent split was cut for budget [documented: orchestrator brief]; truth WEAKENED 102/338, materiality WEAKENED 115/338, crosstab UU 181 / UW 55 / WU 42 / WW 60; 10 of the 24 P1 survivors carry a WEAKENED vote from the only refuter that saw them [measured]. No record was refuted twice, so no inter-rater figure exists. | The P1/P2/P3 split rests on one judgement per record. The killed rate 19/357 (5.3%) carries no interval — one refuter, one pass, so an interval would describe nothing. Whether a second refuter would move any of the ten weakened P1s down, or ten P2s up, is **not-measurable**. The corpus applies a stronger standard to its own spike labels (a panel with a recorded disagreement count, F39: "15 of 32 differ") than this audit applied to its labels. The 55 UW records may sit a priority too high; the 42 WU records may carry a weakened quote. | A second, independent refuter over the 24 P1 records and the 72 P2 records carrying any WEAKENED vote (96 records), blind to the first refuter's reasons; report agreement on priority and on NEW/SHARPENS-KNOWN as k/n with its Wilson interval; a third vote only on the disagreements. Three to four agent-hours. |
| **No human adjudicated anything** (M-06). The corpus labels human reads separately from model reads (`spikes/results/p1-arm-d-labels-ryan.md`, `p2-labels-ryan.md`, `e3-severity-annotation-ryan.md` exist); this audit's 338 survivors and 860 grounding verdicts are model-labelled only, and the `confidence` field (high 278 / medium 60) is self-report by the finder that raised the finding [measured]. | The P0/P1 line — "Step 5 would build the wrong thing" — is a judgement the corpus would ordinarily put in front of its operator before acting on it. Nothing in this review has been. Confidence carries no calibration. | A blank label file in the corpus's own shape (id, agree/disagree/unsure, note) over the 17 P1 concerns and a seeded random 30 of the 82 P2, filled by the operator before Step 5; record the disagreement count beside the deliverable. Two to three human hours. Do not re-threshold priorities after the labels are in; report the disagreements. |
| **NEW / SHARPENS-KNOWN rests on a keyword index whose recall was never measured** (M-08; C-11 novelty half). The 1,381 known-issue rows were queried through a `search_index` of about 15 keywords per row, by finder then refuter; the rows' `source_anchor` (file:line) was not used for proximity matching; no known-issue row was itself re-verified against the corpus [measured]. | NEW (41 of 110) is an upper bound and the error is one-way: a missed row can only convert NEW to SHARPENS-KNOWN, never the reverse. The house rule "never present a known item as a discovery" is satisfied only to the recall of that index. The 3 ALREADY-RECORDED survivors show the refuters caught some; how many they missed is **not-measurable**. | Script: for each of the 338 survivors list every known-issue row whose `source_anchor` sits in the same file within ±5 lines of any finding anchor (1,420 anchors × 1,381 rows); hand-read the pairs for the 41 NEW concerns. Thirty minutes to script, one to two hours to read; report the reclassification count. |
| **AC section 12 (CLI surface and `doctor`; AC-400..AC-439, SM-121..SM-130) had no owning slice** (C-01). The census mapped the 39 design §0.1 rows to 21 groups and no §0.1 row owns the CLI, so no group's span covers AC register lines 4970–5471 [measured]. 27 of its 40 criteria and 6 of its 10 measures were never named by any finder or anchor; design §8.1–8.6 was touched at three lines. | Step 7d builds the composition root (`rk-cli`, `doctor`, `answer`, `explain`) against 40 criteria no finder tested for decidability, ambiguity, ADR agreement, powerless thresholds or two-implementer divergence. Silence on this area is the one Phase-1 AC area with no evidence either way; survivors that graze its edges (`run --resume` vs AC-400) show defects exist there. | One slice finder and one refuter with the standard checks over AC 4970–5471, SM §1.11, design 1259–1437 and RUNBOOK 2079–2125 (about 1,050 lines); add `cli-doctor` as a fortieth row to the census mapping so the gap cannot recur. Needed before Step 7d, not before Step 5. |
| **AC section 3.1–3.6 (selection engine steps 1 and 3–7) was named or anchored by no finder** (C-02). G01 read §3.7 plus §1.1; G02 read three criteria (AC-066/067/068); 21 of 40 criteria — purity and replay, rank and ties, rigor and stage scope, the unsat gate, typed deviation, short-circuit — are untouched, and no finder claimed a whole-section read [measured]. | The BLOCKED label on the two selection rows drew both finders to the facet-source block (PIA-001, PIA-003) and away from the pure function the design says is "decided and buildable now" (design:19). The seven-step engine's replay contract and its unsat/deviation typing enter Step 7b with only the census's mechanical checks behind them. | One slice finder and refuter over AC 608–1008 and design §4.4–4.6, with the two-implementer test as the primary lens: purity/replay and rank/ties are where two implementers diverge silently. |
| **Four sections were cleared on a finder's whole-section read claim while fewer than 60% of their criteria were individually named** (C-03): §4 (22 of 39 named), §7 (22 of 39), §8 (19 of 39), §13 (27 of 40); §11.1–11.2 partial (AC-361/363/364/365/370/397/398 untouched) [measured]. | "No defect found" over 86 criteria in §§4, 7, 8, 11 and 13 is a finder's word; the review cannot distinguish "read and clean" from "skimmed" for them, and says so wherever a section is reported clean. | Draw 10 criteria at random (seeded) from each untouched list; run the decidability and two-implementer tests with a fresh agent; if any of the 50 fails, re-run that section's slice in full. One agent, under an hour. |
| **54 of the 166 non-ADR markdown documents (908,670 bytes) were never named by any finder or anchor; four more were touched at one line** (C-06). Among them: 11 of 12 research briefs; the 2026-09-05 proposal; `DECISIONS-2026-09-09.md`; `spikes/HANDOFF.md`; the P1 classify prompts; `METHOD-P9.md`; `PRIOR-ART.md`; 17 of 20 p1-evidence dossiers. 23.6% of corpus bytes (1,024,260 of 4,341,346, ADRs included) were never named [measured]. | This is an audit of the four live documents, the RUNBOOK, the METHOD/F-draft chain and 59 ADRs, with the rest reached only through citations from them. A brief that reasoned badly from a true source would have been caught only if the design misquoted it or the source contradicted it. A decision recorded in DECISIONS or HANDOFF and never propagated to an ADR was not seen. The P1 prompts — the instrument whose failure blocks Phase 1 — were not read. | Three targeted reads: (1) DECISIONS-2026-09-09.md and HANDOFF.md whole, diffed against the ADR set and the RUNBOOK's decisions table (about 94 KB); (2) the proposal's §4.2–4.6 against AC §12, folded into the C-01 slice; (3) METHOD-P9.md and `spikes/prompts/p1-classify*.md` against F36/F39 and ADR-0079/0089, since Step 4's replacement for P1 is authored from them. The research briefs and fixture dossiers may stay unread if the review says it did not vouch for their reasoning — which it does not. |
| **30 of 89 ADRs had no body read** (C-07): 15 never named (0026, 0035, 0039, 0042, 0047, 0051, 0052, 0055, 0061, 0062, 0064, 0065, 0066, 0068, 0071) and 15 named only in a forbids list or pair check (0007, 0015, 0036, 0037, 0041, 0043, 0045, 0050, 0053, 0054, 0060, 0063, 0067, 0072, 0085) [measured]. ADR-0067 is a Phase-1 prerequisite per `census/phase1-deps.json`; ADR-0061 is cited 10 times in the AC register; ADR-0085 sets the documentation conventions this audit enforces. | The design-and-register-against-ADR check ran against forbids blocks for these 30, not their Decision clauses. A Decision clause contradicted by an AC in those areas was not found. This review enforces id-namespace and immutability rules from a record (ADR-0085) it did not read whole. | Read the 30 in full against the AC lines `census/ids.json` lists as their citation sites (about 128 KB), in the order 0067, 0061, 0064, 0072, 0085, 0007, 0015, 0037, 0036, 0043, then the DSL family; ADR-0041 is superseded and may be skipped once 0064 is read. |
| **No non-markdown artefact carried a verified claim** (C-09). The external census scoped itself to `*.md`; `spikes/runner/Cargo.lock` (141 packages) was never scanned against RUSTSEC/OSV and `cargo-audit` is absent on the box; `journal.rs`, `gate.rs`, `human_gate.rs`, `corpus.rs`, `p1.rs`, `p7.rs` were unread; the 12 pack and 102 catalog TOMLs were counted, not read [measured: `census/external.json` `_meta.scope`]. | What Steps 7b and 7c lift verbatim into product crates (RUNBOOK:2013 "lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs`") is unreviewed code and configuration. A lint error today's packs would raise, or an advisory against a lockfile dependency that survives the lift, is outside what this review can exclude. The verdict's "every §9.2 crate pin is clear" does not extend to the lockfile. | Install `cargo-audit` and run it on `spikes/runner/Cargo.lock` (minutes; it is RUNBOOK Checkpoint 5's own instrument). Read `journal.rs` against AC §11.1 and the §7 event criteria. Run `spikes/p9/lint.py` over the 12 packs and the catalog and read its output. |
| **Wilson intervals were recomputed for the paired set only** (M-03). One lens paired "every stated interval I could pair with a k/n" and matched to 2 dp; three slices recomputed independently for F36/F39, F40, F29/F31; plain Wilson without continuity correction was assumed everywhere; the unpaired residue was neither recomputed nor counted [measured]. | "No defect" on intervals holds for the paired set; the unpaired residue has no number — **not-measurable**, not zero. An interval the corpus computed by Clopper–Pearson or with correction would show as a mismatch only where the difference exceeds rounding, which at n ≤ 15 it often does not. The E3 exact-McNemar and MCC checks ran once, by one script. | Script: regex every `[a, b]` or `a–b%` bracket within 40 characters of a `k/n` in the four authority documents, FINDINGS.md and the F-drafts; recompute with Wilson, Wilson-cc and Clopper–Pearson; emit paired/unpaired/matched/mismatched counts. One hour; the unpaired count is the deliverable. |
| **The 481-id design/register mirror diff ran over four keyword classes only** (M-04): thresholds with units (141 rows hand-read), comparator phrasings (12), state words (27), Instrument vs Verified-by (79 pairs, 12 differences); the GIVEN/WHEN/THEN body was not diffed [measured]. | "No defect found" on the mirror is a no-defect on four keyword classes at full coverage, not on the criteria's semantics. A design row that narrows or widens a WHEN clause without changing a number, a state word or the instrument column is outside what was checked; the population rate of wording drift over 481 ids is **not-measurable** from this audit. | Script: for each of the 481 ids, normalise the design sentence(s) carrying the id and the register block's GIVEN/WHEN/THEN, compute a token-set similarity, hand-read the lowest 50. Two hours; report the distribution, not a verdict. |
| **Spike scorers were re-run for five results only** (M-07): P1 arm-D panel, P2, E3 from `e3.jsonl`, F12 from the old repo's git log, the S-MIG census. P7, P8, H1–H4, E0–E2, E3H, S-H1B/S-WT, P1 arm-L and the P9 lint/lattice were not re-run; F29/F31/F35/F37 figures were checked as restatements against the F-files, not reproduced [measured]. | For the unrun scorers the audit checked that the corpus restates itself consistently (PIA-073 to PIA-078), not that the restated number is what the data gives; a scorer bug or a jsonl that drifted after the F-file was written is invisible. F29/F31/F37 are runner-emitted with no scorer and are **not reproducible by script** from the repo at all; consistency of restatement is the only check that exists for them. | Run each existing scorer at the pin into scratch and diff against its F-file (P7, P1 arm-L, audit-e3, audit-s, lint/lattice `--validate`): one to two hours. For the jsonl-only results a 20-line recount of the headline k/n per file: one hour. For F29/F31/F37 record "runner-emitted, no scorer" as a known limit rather than reconstruct. |
| **Preprint verification was abstract-level for most of the 99 preprint claims** (M-09). `arxiv_abs` is the verify method on 101 claims; full text was fetched only where the first verdict was not CONFIRMED (the 10-row `preprints-deep` batch; 4 of the 16 bucketed preprint rows cite a full-text pass) [measured]. | A CONFIRMED verdict on a preprint attests that the id resolves, title and authors match, and the abstract is consistent with the corpus's gloss — not that a table value or limitation sentence the corpus attributes to the body is there. EXT-PREPRINT-024 (arXiv 2508.16445, PIA-020) is the failure mode: abstract consistent, body without the attributed statement. The 83 preprints that passed at abstract level carry that residual risk at a rate that is **not-measurable** here. | For every preprint claim tagged load-bearing, fetch the arXiv HTML or PDF and grep for the corpus's quoted figure or phrase; report found/not-found as k/n. Two to three hours for the load-bearing subset; a day for all 99. |
| **The four best-practice lanes were single agents with no refuter** (M-10). 77 rows carry `verdict` and `impact` but no `truth_verdict` or `material_verdict`; 4 rows are tagged blocks-decision and 7 changes-a-number on one agent's say-so; each lane's scope was its own choice [measured]. | Best-practice rows (PIA-045, PIA-074, PIA-079, PIA-085 among their sources) enter the deliverable at the same standing as refuted internal findings without an adversarial read. A practice a lane did not think to look for is absent, and absent is all that can be said. | One refuter over the 11 rows with impact blocks-decision or changes-a-number, with the same truth/materiality fields as the internal buckets. One hour. |
| **The testability lens (L07) delivered no coverage note** (C-04, M-11): 0 characters in `coverage.json`, no `coverage` key in its findings file; its 8 findings anchor five files [measured]. | The lens has positive findings and no negative space: the review cannot say any criterion is testable on L07's authority, only that it found eight that are not. Testability is the class Step 5's acceptance set depends on most directly; of 19 testability findings, 8 come from a lens whose scope cannot be stated. | Re-run L07 with the coverage note mandatory (its inputs are the AC Verified-by cells and the RUNBOOK acceptance sets; one agent-hour), or accept the eight and record the lens as unscoped — which this table does. |
| **The coverage record is a capped self-report** (C-05). `coverage.json` truncates every note to exactly 1,500 characters (33 of 34 entries are 1,500 long; one is 0); the untruncated notes run 1,625–3,383 characters for read spans of 331–2,096 lines; no "read in full" claim was checked against what the agent opened [measured]. | Every count in this section is a floor on the unread set and a ceiling on the read set. "No finder names X" does not mean X was not read, and "a finder names X" does not mean X was audited; the rows above inherit both caveats. | Not closable retrospectively. For any re-run, have the harness log file opens per agent so coverage is measured rather than reported. |
| **Refutation covered findings only, and every lens ran once** (C-11 recall half, M-12). None of the "checked and found no defect" statements in the 34 coverage notes was challenged; reproducibility was checked for the census scripts (9/9), never for a reading agent; no finder appears twice [measured]. | Precision of the 338 survivors is defended; recall is not. A finder that missed a defect and wrote "no defect" left no trace a refuter would test. The finding count per lens (13 down to 7) is a single draw with unknown overlap against a second draw; the audit cannot say what fraction of any class it captured — **not-measurable** — only what it found. Every absence-of-finding statement in this review is bounded by this row. | Re-run two lenses blind (L04-ambiguity and L08-sequencing, the two with the largest Phase-1 exposure) and report the Jaccard overlap of finding sets after anchor-matching; if overlap is below one half, downgrade the per-class "absence" statements to "not sampled". A second independent finder on the C-01 and C-02 sections gives a recall estimate for the rest. Two to three agent-hours. |
| **Census artefacts no lens consumed, and an anomaly pool that is a reconstruction** (C-08, M-13). `doc-map.json` was named by zero finders — its 126 orphans (27 strict), 37 stale-risk files (12 cited as current by a newer document) and `read_plan_problems` were adjudicated by nobody; `slices.json` by one. Eight of nine census artefacts hold no structured anomaly list, so 69 of the 82 agent-reported anomalies exist only in prose returns and the 115-row pool the lenses queried was rebuilt from structured stand-ins; `census/reconcile.json` records this at severity material [measured]. | The currency signal — a 2026-09-02 FINDINGS.md cited as current by three newer documents, and eleven similar pairs — is computed and unreviewed; the 27 strict orphans may be dead or may be authority the live set forgot to cite. An anomaly a census agent saw and described only in prose may have seeded no finding; the downstream effect is **not-measurable**. | One short lens over `doc-map` stale_risk (12 rows) and strict_orphans (27 paths): is the citing document right to treat it as current, and is the orphan dead or load-bearing? Half a day. If the census phase's prose returns are retained, a 30-minute pass lists the 69 prose anomalies against the 115-row pool. |
| **16 external claims ended UNVERIFIABLE** (C-10), six of them extractor misclassifications with nothing external to verify (EXT-STATEDAB-004, -060 to -064: the "none found" pattern caught internal text) and ten genuine, none load-bearing under the census rule [measured]. | The review can say the corpus does not depend on these ten; it cannot say they are true. The six misclassifications inflate the stated-absences family's 64-row denominator, which the grounding tally does not correct. | Re-cite or drop EXT-CLAUDECO-042 and EXT-PROCESSL-154; mark `spikes/HANDOFF.md:303` a convenience pointer or re-share the artifact; mark EXT-COMPETIT-026 "retired" as [inferred]; re-fetch openreview from an interactive browser; correct the stated-absences extractor so the family count is honest. |
| **Five of the 39 design areas have no AC section by the census's own mapping** (C-12): "Design final" as a whole, architecture shape, Phase-1 scope, the Rust workspace, the quality shorthand; several more map to an empty AC list with an ABSENCE note (structured output; gix common-dir — "gix occurs 0 times in the AC register body") [measured: `census/slices.json` `areas_with_no_ac_section: 5`]. | For those areas the decidability and instrument-coverage checks were vacuous by construction; the finders' "self-recorded absence" rows say so. This review does not report those areas as AC-clean. | None needed for the audit; the corpus records the absences itself (AC register 6519, design 1661). The remediation belongs to the corpus, not the review. |
| **Snapshot boundary** (C-13). Every finder measured HEAD `009a4ec` with 0 modified and 17 untracked paths; the untracked arm-L, P7 and P8 logs, `spikes/results/p8-state/` and the two runner target directories were not audited beyond one arm-L custody finding (PIA-021); anything committed after the census (2026-09-10 23:05) or the last synthesis write (2026-09-11 06:16) is outside this review [measured]. | The review describes a tree that will not be the tree Step 5 starts from once the untracked files are committed or discarded. | Re-run the nine census scripts after the commit that closes Step 3c (all nine reproduce; `doc-map` takes about three minutes) and diff the counts. |

In the corpus's own idiom: every count in this review is a single pass by one model family on one day over one pin. The 338 findings and 860 grounding verdicts are model-labelled, with no human label
file and no second draw; the only reproducibility figure is 9/9 on the census scripts, which measures the scripts, not the readers. The rates quoted about the audit itself — 5.3% killed, 30.2%
truth-WEAKENED, 28 of 860 verdicts flipped — carry no interval because the process that produced them was run once, and an interval over one run describes nothing. The census's cross-checks pass 43 of
44, and the one failure is the audit's own seed (~48 arXiv ids expected, 99 measured). External verdicts are dated 2026-09-10 and are STALE-BY-DATE by construction from the next crates.io publish or
documentation change onward. Of the four fault names: nothing about the audit's own rates is saturated except the 9/9, a script property; the killed rate and every per-lens count are powerless at n =
1 run; the recall of any lens, the recall of the known-issues index, the wording-drift rate over 481 ids and the residual body-versus-abstract risk over 83 preprints are not-measurable; and whether
the port traits compile and whether the predicate grammar is unambiguous are not-answerable, because the corpus as written does not contain the answer and this audit did not build the instrument that
would.

## 9. Method

1. **Census — machine-checked ground truth** (10 agents; nine scripts under `scratchpad/audit/scripts/`, nine artefacts under `scratchpad/audit/census/`). Computed from the 194 markdown documents in
   scope (4,341,346 bytes; 42,993 lines) at HEAD `009a4ec`: every id namespace and its definition and citation sites (`ids.json`: 514 AC, 135 SM, 89 ADR, 38 F, the spike ids, the legacy
   D/PC/SCHEMA/L/A ids); every restated constant with its agreement status (`constants.json`: 142 AGREE, 13 VARIANT, 11 RANGE-STATEMENT); every section reference and heading (`anchors.json`: 4,078
   refs, 4,023 resolved, 7 dangling); the ADR graph (`adr-graph.json`: frontmatter, sections, supersession, related edges, forbids blocks, README claims); the 39 design areas mapped to
   AC/SM/design/RUNBOOK spans (`slices.json`); every external claim with a verify method (`external.json`: 860 claims in 15 families); the corpus's own self-recorded issues (`known-issues.json`: 1,381
   rows with a keyword index); the RUNBOOK's Phase-1 dependency structure (`phase1-deps.json`); and a document map with orphans and stale-risk pairs (`doc-map.json`). A reconciliation step re-ran all
   nine scripts with only their output paths substituted and compared byte-for-byte after stripping volatile metadata: **9 of 9 reproducible** (`reconcile.json`, `reproducible_count: 9`), the repo's
   porcelain unchanged before and after (17 lines both times), 43 of 44 cross-artefact checks passing.
2. **Internal audit — finder gated by refuter** (34 + 34 agents). Twenty-one area slices (G01–G21, one per census group, each with a brief listing its AC/SM/design/RUNBOOK/ADR spans and the standard
   checks: contradiction, decidability, ADR agreement, two-implementer divergence, instrument coverage, blocked-leakage) and thirteen corpus-wide lenses (L01 contradiction, L02 citation fidelity, L03
   blocked leakage, L04 ambiguity, L05 overlap, L06 buildability, L07 testability, L08 sequencing, L09 pre-registration integrity, L10 register integrity, L11 statistics, L12 authority, L13
   operational) produced 357 candidate findings, 1,420 anchors, none without a quote. Each finder's output went to one adversarial refuter that re-read every anchor, checked every quote against the
   file (by script where the verdict file says so, e.g. G01: "All 37 anchors across the nine findings verified verbatim at file:line by script"), and voted on truth and on materiality-plus-novelty
   against `known-issues.json`: 19 killed, 338 survivors (P1 24, P2 213, P3 101; NEW 174, SHARPENS-KNOWN 161, ALREADY-RECORDED 3). No finding entered synthesis without a refuter verdict on the record
   (`findings/verdict-*.json`).
3. **External grounding — verifier gated by second opinion** (32 + 20 agents). The 860 claims were batched by family (competitors 198, process literature 200, peer and standards 113, preprints 99,
   stated absences 64, Claude Code 43, crate versions 28, durable execution 28, toolchain 21, Anthropic platform 15, advisories 13, gitoxide 8, SQLite 5, decision records 10, no-url 15) and
   re-verified live on 2026-09-10 by method — crates.io API, RUSTSEC/OSV, docs.rs, the GitHub API, arXiv abstracts, DOI resolution, vendor documentation, web search — each row recording URL, fetch
   date, source quote, what the corpus says, what the source says, impact. Every non-CONFIRMED first verdict went to a second-opinion re-fetch, which flipped 28 (25 DRIFTED→CONFIRMED, 2
   CONTRADICTED→DRIFTED, 1 DRIFTED→CONTRADICTED); three abstract-level preprint drift candidates were re-read in full text and two dissolved. Final tally: 697 confirmed, 105 drifted, 29 contradicted,
   16 unverifiable, 13 stale-by-date; 144 claims load-bearing, of which 101 confirmed. Four best-practice lanes (Rust architecture 20, durable workflow state 20, SDD landscape 19, LLM evaluation 18)
   ran without a refuter and are marked as such in §8.
4. **Synthesis — cluster, rank, criticise, write, assemble** (11 + 1 + 2 + 6 + 1 agents). Eleven cluster agents partitioned the 338 survivors and the grounding and best-practice rows into eleven
   buckets (readiness, realisability, ambiguity, contradiction ×2, inaccuracy, citation and statistics, method integrity, grounding ×2, best practice), consolidating to 151 bucket concerns with
   anchors carried forward verbatim from the finding records. One ranking agent merged across buckets (30 merges, 9 priority reconciliations), re-read every P1 against the P0 definition, and fixed the
   ordering rule and the verdict draft (`concerns/MASTER.json`). Two critics audited the audit — coverage (what was never read) and method (what never ran) — before any prose was written, and their
   limits are §8. Six writers produced this document from the master list under a brief that forbids adding a finding, changing a priority or paraphrasing a quote; anchors are carried forward verbatim
   from the finding records. One assembler-editor concatenated the six section drafts, renumbered the sections, applied the house-style pass and ran the consistency checks (every PIA id present
   exactly once, the priority table against the master list, the top-ten ids); it added no finding and changed no priority, and re-verified the 603 anchor bullets against the pin by script: 601 hold
   at the cited line, and two were re-lined after a read (AC-388 THEN clause, 4831 to 4832; ADR-0087 rejected option, 42 to 38-40).

Gating, in one line: no candidate reached a bucket without a refuter's verdict; no non-confirmed external verdict reached a bucket without a second fetch; no concern reached this document without a
ranker's re-read against P0 and the two critics' limits standing beside it.

## 10. Sources

### 10.1 Repo

`D:/repos/redkiln-rs` at HEAD `009a4ec68119896e63a3fb3e405ce1b6616665c3` (branch `main`, 0 modified, 17 untracked paths), read-only throughout; predecessor repo `D:/repos/redkiln` at
`47faec9f46b8d983c2c75f0794164e68415ae706` for the S-MIG and F12 re-counts. Audit scripts and artefacts live under the session scratchpad
`C:/Users/ryanm/AppData/Local/Temp/claude/D--repos-redkiln-rs/953a92ec-7709-4a24-b01d-8a5e2c5b86c8/scratchpad/audit/` and are **ephemeral** — they are not in the repo and will not survive the session
directory; anything below that the operator wants to keep must be copied out.

- `scripts/census_ids.py` → `census/ids.json` (1,106,371 bytes) · `scripts/census_constants.py` → `census/constants.json` (4,411,185) · `scripts/census_anchors.py` → `census/anchors.json` (6,521,586) · `scripts/census_adr.py` → `census/adr-graph.json` (439,800) · `scripts/census_slices.py` → `census/slices.json` (315,485) · `scripts/census_external.py` → `census/external.json` (988,010) · `scripts/census_known.py` → `census/known-issues.json` (1,227,062) · `scripts/census_deps.py` → `census/phase1-deps.json` (215,313) · `scripts/census_docmap.py` → `census/doc-map.json` (599,075).
- `scripts/reconcile_step1_inventory.py`, `reconcile_step2_rerun.py`, `reconcile_probe_arxiv.py`, `reconcile_step3_crosscheck.py`, `reconcile_step4_report.py` → `census/reconcile.json`.
- `scripts/make_briefs.py` → `findings/briefs/`; finders `findings/slice-G01..G21.json`, `findings/L01..L13-*.json`; refuters `findings/verdict-*.json`; `scripts/merge_internal.py` → `findings/INTERNAL-MERGED.json`.
- `scripts/make_batches.py` → `grounding/batches/*.json` (28 claim batches and an index); `scripts/merge_grounding.py` → `grounding/GROUNDING-MERGED.json` (860 rows plus 77 best-practice rows); fetched full texts and page captures alongside (`grounding/*.txt`, `*.pdf`, `*.html`).
- `scripts/make_buckets.py` → `synthesis/buckets/`; cluster outputs `synthesis/concerns/B01..B10-*.json`; `synthesis/build_master.py` → `synthesis/concerns/MASTER.json`; `synthesis/coverage.json`; `synthesis/critic-coverage.json`; `synthesis/critic-method.json`; this review under `synthesis/review/`.

### 10.2 Platform and vendor documentation

All fetched 2026-09-10 by the grounding lane. The lane used 757 distinct URLs across the 860 rows; 597 of them sit on 286 hosts once the arXiv, doi.org, Crossref, Springer, Semantic Scholar and PMC
hosts are set aside [measured, `GROUNDING-MERGED.json` `url` fields, split on `;` and whitespace, deduplicated]. The 80 most-cited of those 597 URLs follow, grouped by host and ordered by citation
count; each carries the number of claim-rows that cite it and their final verdicts. The remaining 517 URLs are in the merged file, one per row.

**code.claude.com**
- <https://code.claude.com/docs/en/cli-reference> — 11 claim-rows (10 confirmed, 1 drifted)
- <https://code.claude.com/docs/en/prompt-caching> — 7 claim-rows (6 confirmed, 1 drifted)
- <https://code.claude.com/docs/en/sub-agents> — 5 claim-rows (4 confirmed, 1 drifted)
- <https://code.claude.com/docs/en/context-window> — 4 claim-rows (2 drifted, 2 confirmed)
- <https://code.claude.com/docs/en/agent-sdk/structured-outputs> — 2 claim-rows (2 confirmed)
- <https://code.claude.com/docs/en/code-review> — 2 claim-rows (2 confirmed)
- <https://code.claude.com/docs/en/hooks> — 2 claim-rows (2 contradicted)
- <https://code.claude.com/docs/en/plugin-marketplaces> — 2 claim-rows (1 drifted, 1 unverifiable)
- <https://code.claude.com/docs/en/workflows> — 2 claim-rows (1 drifted, 1 confirmed)

**doc.rust-lang.org**
- <https://doc.rust-lang.org/nightly/unstable-book/language-features/async-fn-in-dyn-trait.html> — 9 claim-rows (9 confirmed)
- <https://doc.rust-lang.org/stable/releases.html> — 8 claim-rows (8 confirmed)
- <https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html> — 4 claim-rows (4 confirmed)
- <https://doc.rust-lang.org/stable/clippy/lints.html> — 3 claim-rows (2 confirmed, 1 drifted)

**github.com**
- <https://github.com/rust-lang/rust/issues/133119> — 11 claim-rows (11 confirmed)
- <https://github.com/MrLesk/Backlog.md/issues/558> — 2 claim-rows (1 confirmed, 1 drifted)
- <https://github.com/axodotdev/cargo-dist/releases/tag/v0.24.0> — 2 claim-rows (2 confirmed)
- <https://github.com/thomvaill/log4brains> — 2 claim-rows (2 drifted)

**raw.githubusercontent.com**
- <https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md> — 5 claim-rows (4 confirmed, 1 contradicted)
- <https://raw.githubusercontent.com/dtolnay/anyhow/master/README.md> — 4 claim-rows (4 contradicted)
- <https://raw.githubusercontent.com/GitoxideLabs/gitoxide/main/crate-status.md> — 2 claim-rows (2 confirmed)
- <https://raw.githubusercontent.com/rust-lang/rust-clippy/master/README.md> — 2 claim-rows (1 confirmed, 1 drifted)
- <https://raw.githubusercontent.com/saman-mb/shipmates/main/Cargo.toml> — 2 claim-rows (1 stale-by-date, 1 contradicted)

**api.github.com**
- <https://api.github.com/repos/Dicklesworthstone/beads_rust> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/RooCodeInc/Roo-Code> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/Tinder/bazel-diff> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/adr/madr/releases> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/adr/yadr/commits?per_page=10> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/anthropics/claude-agent-sdk-python/issues/571> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/anthropics/claude-code/issues/45756> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/anthropics/claude-code/issues/46829> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/bazel-contrib/target-determinator> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/bmad-code-org/BMAD-METHOD> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/coleam00/Archon/branches/archive/v1-task-management-rag> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/coleam00/Archon/issues/901> — 1 claim-row (1 drifted)
- <https://api.github.com/repos/coleam00/Archon/issues/952> — 1 claim-row (1 confirmed)
- <https://api.github.com/repos/coleam00/Archon/issues/957> — 1 claim-row (1 confirmed)

**blog.rust-lang.org**
- <https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/> — 9 claim-rows (8 confirmed, 1 stale-by-date)

**crates.io**
- <https://crates.io/api/v1/crates/temporalio-sdk> — 3 claim-rows (2 drifted, 1 confirmed)
- <https://crates.io/api/v1/crates/gix> — 2 claim-rows (1 drifted, 1 confirmed)
- <https://crates.io/api/v1/crates/rusqlite> — 2 claim-rows (2 confirmed)

**docs.rs**
- <https://docs.rs/thiserror/latest/thiserror/> — 5 claim-rows (5 contradicted)
- <https://docs.rs/jsonschema/latest/jsonschema/> — 2 claim-rows (1 stale-by-date, 1 confirmed)

**platform.claude.com**
- <https://platform.claude.com/docs/en/build-with-claude/prompt-caching> — 4 claim-rows (4 confirmed)
- <https://platform.claude.com/docs/en/build-with-claude/compaction> — 2 claim-rows (1 confirmed, 1 drifted)

**www.nasa.gov**
- <https://www.nasa.gov/reference/appendix-d-requirements-verification-matrix/> — 3 claim-rows (3 contradicted)
- <https://www.nasa.gov/reference/appendix-c-how-to-write-a-good-requirement/> — 2 claim-rows (2 contradicted)

**adr.github.io**
- <https://adr.github.io/madr/> — 3 claim-rows (3 confirmed)
- <https://adr.github.io/> — 1 claim-row (1 confirmed)

**docs.snyk.io**
- <https://docs.snyk.io/developer-tools/snyk-cli/commands/ignore.md> — 2 claim-rows (2 contradicted)
- <https://docs.snyk.io/manage-risk/prioritize-issues-for-fixing/reachability-analysis> — 2 claim-rows (2 confirmed)

**www.mintmcp.com**
- <https://www.mintmcp.com/blog/llm-cost-optimization-agent-teams> — 4 claim-rows (3 unverifiable, 1 drifted)

**aclanthology.org**
- <https://aclanthology.org/2021.findings-emnlp.354/> — 1 claim-row (1 confirmed)
- <https://aclanthology.org/2024.emnlp-industry.91.pdf> — 1 claim-row (1 confirmed)
- <https://aclanthology.org/2025.findings-emnlp.1361/> — 1 claim-row (1 confirmed)

**aws.amazon.com**
- <https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/> — 3 claim-rows (3 drifted)

**ops.fhwa.dot.gov**
- <https://ops.fhwa.dot.gov/publications/fhwahop18080/ch3.htm> — 3 claim-rows (3 contradicted)

**systima.ai**
- <https://systima.ai/blog/subagent-tax> — 3 claim-rows (3 confirmed)

**theinfinity.dev**
- <https://theinfinity.dev/articles/subagent-cost-measured> — 3 claim-rows (3 confirmed)

**www.digitalapplied.com**
- <https://www.digitalapplied.com/blog/headless-agent-cost-per-task-benchmark> — 3 claim-rows (3 confirmed)

**beam.ai**
- <https://beam.ai/agentic-insights/multi-agent-orchestration-patterns-production> — 2 claim-rows (2 confirmed)

**blog.cloudflare.com**
- <https://blog.cloudflare.com/ai-code-review/> — 2 claim-rows (2 confirmed)

**docs.github.com**
- <https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments> — 2 claim-rows (2 confirmed)

**docs.gitlab.com**
- <https://docs.gitlab.com/development/work_items/> — 2 claim-rows (2 drifted)

**linear.app**
- <https://linear.app/developers/graphql> — 2 claim-rows (2 confirmed)

**research.google**
- <https://research.google/blog/towards-a-science-of-scaling-agent-systems-when-and-why-agent-systems-work/> — 2 claim-rows (2 confirmed)

**research.manchester.ac.uk**
- <https://research.manchester.ac.uk/en/publications/easy-approach-to-requirements-syntax-ears> — 2 claim-rows (2 confirmed)

**shipmates.vc**
- <https://shipmates.vc> — 2 claim-rows (2 confirmed)

**swatinem.de**
- <https://swatinem.de/blog/rust-toolchain/> — 2 claim-rows (2 drifted)

**trigger.dev**
- <https://trigger.dev/docs/wait-for-token> — 2 claim-rows (2 confirmed)

**www.cs.ubc.ca**
- <https://www.cs.ubc.ca/~rtholmes/papers/icse_2014_inozemtseva.pdf> — 2 claim-rows (2 confirmed)

**www.helpnetsecurity.com**
- <https://www.helpnetsecurity.com/2026/04/28/openai-symphony-codex-orchestration-linear/> — 2 claim-rows (2 confirmed)

**www.inngest.com**
- <https://www.inngest.com/docs/reference/functions/step-wait-for-event> — 2 claim-rows (2 confirmed)

**www.istqb.org**
- <https://www.istqb.org/certifications/certified-tester-foundation-level> — 2 claim-rows (2 confirmed)

**www.jucs.org**
- <https://www.jucs.org/jucs_16_3/situational_method_engineering_state.html> — 2 claim-rows (2 confirmed)

**www.omg.org**
- <https://www.omg.org/spec/Essence/1.2/PDF> — 2 claim-rows (2 confirmed)

**www.pistack.xyz**
- <https://www.pistack.xyz/posts/2026-06-22-rust-error-handling-anyhow-thiserror-eyre-guide/> — 2 claim-rows (2 confirmed)

**about.codecov.io**
- <https://about.codecov.io/blog/mutation-testing-how-to-ensure-code-coverage-isnt-a-vanity-metric/> — 1 claim-row (1 confirmed)

**abseil.io**
- <https://abseil.io/resources/swe-book/html/ch22.html> — 1 claim-row (1 confirmed)

**ai-eval.org**
- <https://ai-eval.org/post/anthropic-demystifying-evals-for-ai-agents> — 1 claim-row (1 confirmed)

**aireviewzones.com**
- <https://aireviewzones.com/openhands-review/> — 1 claim-row (1 confirmed)

### 10.3 Literature

Every arXiv id and DOI a concern cites, with the grounding row that verified it. Titles are as the fetched page states them; where the audit did not capture a title, the paper is described by what the
concern relies on and no title is supplied.

**Preprints (arXiv)**

- 2312.03006 — rank reversal across MCDM methods; the source of the sentence "Rank reversal of one or another type occur for virtually every MCDM method" (cites Aires & Ferreira 2018 and Wang & Luo 2009). Cited by PIA-098. Grounding EXT-PEERANDS-007 confirmed, EXT-PEERANDS-024 drifted. <https://arxiv.org/html/2312.03006>
- 2402.11048 — Silva, "Towards identifying and minimizing customer-facing documentation debt" (TechDebt@ICSE 2023; an Ericsson defect analysis of 101 bug reports). Cited by PIA-101. EXT-PREPRINT-063 drifted. <https://arxiv.org/html/2402.11048>
- 2411.13768 — "Evaluation-Driven Development and Operations of LLM Agents: A Process Model and Reference Architecture" (v3, 17 Nov 2025; the corpus pins v2). Cited by PIA-101. EXT-PREPRINT-075 stale-by-date. <https://arxiv.org/abs/2411.13768>
- 2508.16445 — the "Essence Coach" preprint (v1 and v2 read in full; the attributed "precise criterion matching" statement is in neither). Cited by PIA-020. EXT-PREPRINT-024 contradicted. <https://arxiv.org/pdf/2508.16445v1>
- 2509.01494 — code-review LLM-judge agreement study (human–LLM κ 0.43–0.71, human–human 0.61–0.76); the nearest on-topic hit for PRIOR-ART's "62.5% agreement / κ 0.37", which no source states. Cited by PIA-108. <https://arxiv.org/pdf/2509.01494>
- 2509.13942 — eleven projects under Waterfall, V-Model and Agile scaffolds with four GPT variants, 132 runs. Cited by PIA-092 and PIA-101. EXT-PREPRINT-073 confirmed; EXT-STATEDAB-020/-031 drifted. <https://arxiv.org/abs/2509.13942>
- 2510.09738 — "Judge's Verdict: A Comprehensive Analysis of LLM Judge Capability Through Human Agreement". Cited by PIA-108. EXT-PREPRINT-095 confirmed. <https://arxiv.org/abs/2510.09738>
- 2511.01176 — decision-boundary sensitivity of LLM outputs (the P1-BOUND premise; the boundary sentence is background, not a measurement). Cited by PIA-020. EXT-PREPRINT-022 drifted. <https://arxiv.org/pdf/2511.01176>
- 2511.09794 — a simulated Waterfall cycle on 100 ClassEval tasks with three LLMs (credited in the corpus with 2509.13942's 132-run numbers). Cited by PIA-101. EXT-PREPRINT-074 drifted. <https://arxiv.org/abs/2511.09794>
- 2602.09292 — Bett et al., "Towards an OSF-based Registered Report Template for Software Engineering Controlled Experiments" (10 Feb 2026). Cited by PIA-108. <https://arxiv.org/abs/2602.09292>
- 2603.04417 — temperature-0 output variability across GPT-4o, GPT-4o-mini, Gemini-2.5-Flash, Claude-Haiku-4.5 and Claude-Sonnet-4.5. Cited by PIA-020. EXT-PREPRINT-018 drifted (joint-summary sentence only). <https://arxiv.org/pdf/2603.04417>
- 2603.18740 — "Measuring and Exploiting Contextual Bias in LLM-Assisted Security Code Review" (v2 title; 250 CVE/patch pairs, six LLMs, five framings). Cited by PIA-013. EXT-PREPRINT-019 confirmed after full-text read. <https://arxiv.org/pdf/2603.18740>
- 2603.22386 — "From Static Templates to Dynamic Runtime Graphs: A Survey of Workflow Optimization for LLM Agents" (23 Mar 2026). Cited by PIA-092. EXT-STATEDAB-020/-031 drifted. <https://arxiv.org/abs/2603.22386>
- 2603.22853 — Agent Audit on the 42-vulnerability AVB benchmark (Semgrep 23.8% recall). Cited by PIA-093. EXT-PREPRINT-031 confirmed. <https://arxiv.org/html/2603.22853>
- 2603.26130 — "SWE-PRBench: Benchmarking AI Code Review Quality Against Pull Request Feedback" (350 PRs; 15–31% recall). Cited by PIA-101. EXT-PREPRINT-064 drifted. <https://arxiv.org/abs/2603.26130>
- 2604.13764 — "RealVuln: Benchmarking Rule-Based, General-Purpose LLM, and Security-Specialized Scanners on Real-World Code" (Semgrep recall 17.5%). Cited by PIA-093. EXT-PREPRINT-032 contradicted. <https://arxiv.org/html/2604.13764v1>
- 2604.22411 — temperature-0 answer stability, Table 1 (claude-sonnet-4 0/0/0 on 30 prompts × 100 draws). Cited by PIA-020. EXT-PREPRINT-021 confirmed after full-text read. <https://arxiv.org/pdf/2604.22411>
- 2606.20615 — v3 of 16 Aug 2026 (three authors, adds a SWE-bench section; the corpus pins v2). Cited by PIA-101. EXT-PREPRINT-015 stale-by-date. <https://arxiv.org/abs/2606.20615>
- 2607.00511 — "Large Language Models for Multi-Lingual Equivalent Mutant Detection: An Extended Empirical Study" (3,302 Java and 1,088 C mutant pairs). Cited by PIA-056. EXT-PREPRINT-097 confirmed. <https://arxiv.org/abs/2607.00511>

**DOIs and publisher identifiers**

- 10.1007/978-3-642-41467-1 — Henderson-Sellers, Ralyté, Ågerfalk, Rossi, *Situational Method Engineering* (Springer, 2014); chapter 10.1007/978-3-642-41467-1_3 "Method Engineering as a Social Practice" (pp. 53–68). Cited by PIA-092. EXT-PEERANDS-063 to -066 confirmed; EXT-STATEDAB-046 contradicted. <https://link.springer.com/book/10.1007/978-3-642-41467-1>
- PII S0895717711007850 — García-Cascales & Lamata, "On rank reversal and TOPSIS method", *Mathematical and Computer Modelling* 56 (2012) 123–132, resolved through Crossref; the ScienceDirect full text returned 403 and is unread. PII S0895717708002860 — Wang & Luo 2009, the SAW-and-TOPSIS study the arXiv survey cites for the general sentence. Both cited by PIA-098. <https://api.crossref.org/works?filter=alternative-id:S0895717711007850&rows=3>

**Standards and handbooks** (not DOI-bearing; cited by PIA-086 and PIA-101)

- NASA SP-2016-6105 Rev 2, *NASA Systems Engineering Handbook*, Appendix C (TBD/TBR) and Appendix D (requirements verification matrix). <https://www.nasa.gov/reference/appendix-c-how-to-write-a-good-requirement/>, <https://www.nasa.gov/reference/appendix-d-requirements-verification-matrix/>, <https://ntrs.nasa.gov/api/citations/20170001761/downloads/20170001761.pdf>
- ISO/IEC/IEEE 29148 (requirements engineering; per-requirement verification method), as framed at <https://ops.fhwa.dot.gov/publications/fhwahop18080/ch3.htm>.
- ISO/IEC/IEEE 12207:2017 (withdrawn) at <https://www.iso.org/standard/63712.html>, revised by ISO/IEC/IEEE 12207:2026 (April 2026, iso.org/standard/90219).

---

Generated 2026-09-10 by a four-workflow audit (Claude Opus 5); ~151 agents; every anchor verified by script — see section 9.
