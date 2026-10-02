# Runbook — Redkiln-RS from here to the end of Phase 1 · preamble

**Status:** operator-facing build runbook, split into one file per stage under `runbook/` on 2026-09-11 after the
pre-implementation audit (`references/2026-09-10-pre-implementation-audit.md`, 110 concerns) was absorbed. This
file is the shared preamble; `INDEX.md` is the map and `PROGRESS.md` the tracker. The runbook is the execution
order for `references/2026-09-08-architecture-design.md` §0.1 and §10.2 and **decides nothing the record does
not**: precedence is fixed by **ADR-0092** — accepted ADRs govern every decision; the AC register owns every
criterion's wording, instrument and state; the SM register owns targets, baselines and kills; the spike files govern
every restated number; and this runbook defers to the design for phase order and to the AC register for every
threshold and instrument. Where a stage file disagrees with any of those, the stage file is wrong. · **Scope:**
Phase 0 (close the programme) and Phase 1 (kernel, store, fix and story lanes, four adapters) only. Phases 2–4 are
out of scope. · **Inputs:** `references/2026-09-08-architecture-design.md` (§0.1, §5.3, §5.9, §9, §10) ·
`references/2026-09-08-acceptance-criteria.md` (AC-001–AC-519, AC-520 — the derived-facet gate — and the ids the
2026-09-11 amendment allocated from AC-521 upward, and **its** §12.10) · `references/adr/` (**196 records**,
`0001`–`0196` contiguous *(corrected 2026-09-29 at `s8g`'s build (package R-D2119-8g), when `0196` — `s8g`'s Findings-key record (D21-119, TAKEN PROVISIONALLY): the review pass and the slice entry check key on the review verdict kind through one predicate, and the seal reads only a review activation's `Findings` row, superseding `0186` in part — was written at `proposed` *(dated 2026-10-02, `s8g` close: ADR-0188 accepted on D21-117 at `s8f`'s close, ADR-0189 on D21-118 at `s8f`'s close, ADR-0190 on D21-130, ADR-0191 on D21-131, ADR-0192 on D21-132, ADR-0193 on D21-133, ADR-0194 on D21-134, ADR-0195 on D21-135, ADR-0196 on D21-136)*: this read **195 records**, `0001`–`0195`)* *(corrected 2026-09-29 at `s8g`'s build (package S8G-P1-RECORDS), when `0191` — `s8g`'s verify record (R1): the verify pass is the one `TestReport` route and journals `verify_completed`, superseding `0162`'s patch clause in part —, `0192` — `s8g`'s schema record (R2): D17-3's three spellings fixed and `AcceptanceCriterion` exported —, `0193` — `s8g`'s write-set record (R3): `tag` and `apply`, superseding `0105` clause 5 in part —, `0194` — `s8g`'s producers record (R4) — and `0195` — `s8g`'s tag record (R5): `tag-release` enters by trial — were written at `proposed`: this read **190 records**, `0001`–`0190`)* *(corrected 2026-09-29 at `s8f`'s build (package R1FIX-8f-B-RECORDS, round-1 finding: Checkpoint 8a's census guard), when `0188` — `s8f`'s landing record: the landed commit is a kernel-made content checkpoint on `rk-run/<item>`, superseding `0105` clause 5 in part *(dated 2026-09-29, s8f build, package R5FIX-8f: and clause 4 in part, R4FIX-8f, D21-112)* —, `0189` — `s8f`'s render record: `rk-store` owns the render, write and read path and structured drift is surfaced, restored and gated, superseding ~~nothing~~ `0173` in part *(dated 2026-09-29, s8f build, package R5FIX-8f; D21-113)* — and `0190` — `s8f`'s telemetry-key record: `dispatch_completed` carries `activity_id_at_version` and `cost` breaks down by activity, superseding `0131`, `0138`, `0163` and `0174` in part — were written at `proposed`: this read **187 records**, `0001`–`0187`; `s8f`'s S8F-P1-RECORDS note moved only the README)* *(corrected 2026-09-28 at `s8e`'s build (package S8E-RULINGS), when `0187` — `s8e`'s record of D21-92: a routing `null` `until` or gate `when` is journaled when its route continues the walk, and a `null` routed to a refusal is not, superseding `0174` decision 7 in part (its null sentence only) — was written, at `proposed`: this read **186 records**, `0001`–`0186`)* *(corrected 2026-09-28 at `s8e`'s build (package S8E-P7), when `0186` — `s8e`'s slice record: the slice is declared on the item record and `Slice-Verdict:` is sealed on both review outcomes through the kernel's tier-1 boundary, refused at the mint by name, the seal scoped to the stage it was made in (D21-41 e; D21-93), superseding nothing — was written, at `proposed`: this read **185 records**, `0001`–`0185`)* *(corrected 2026-09-28 at `s8e`'s build (package S8E-P5), when `0185` — `s8e`'s three-kinds record: `worktree_assigned`, `stage_empty` and `git_op`, tier 2, minted by ADR-0116's route (D21-42 7, 8d; D21-43 N7), superseding nothing — was written, at `proposed`: this read **184 records**, `0001`–`0184`)* *(corrected 2026-09-28 at `s8e`'s build (package S8E-P4), when `0183` — `s8e`'s stage-location record: a stage-bearing `Story: <project>/<slug> <stage>` mark at every boundary, folded by `reconcile` to a latest stage per item, the legacy stage-less mark admitted read-only and `closed` reserved for the lane's end (D21-41 a; D21-42 8d; D21-43 N7, N9), superseding `0105`, `0155`, `0157` and `0161` in part — and `0184` — `s8e`'s `Gate:` record: the value carries kind, reach and state (with the answered option and the expiry action), the deadline is integer unix seconds, and `gate_answer_void` moves to tier 1 (D21-42 8b; D21-43 N6, N8; D21-48 X11), superseding `0105` in part — were written at `proposed`: this read **182 records**, `0001`–`0182`)* *(corrected 2026-09-28 at `s8e`'s build (package S8E-P3), when `0181` — `s8e`'s tier-1 writer record: `EventStore::append_tier1(event, &TierOneAnchor)`, the fifth method, written git-first by the kernel, over `rk-store`'s `tier_one` statements and the `0002_gate_transitions` migration (D21-41 e; D21-42 8a, 8c; D21-43 N7, N10; D21-46 R2; D21-48 X10), superseding `0155` and `0158` in part — and `0182` — `s8e`'s interim `item` key record: the key rides beside the event, outside `payload` and `canonical_events`, and `query` honours `ByItem` (D21-43 N5; D21-46 R3), superseding nothing — were written at `proposed`: this read **180 records**, `0001`–`0180`)* *(corrected 2026-09-27 at `s8d-ii`'s build (package P1-RECORD), when `0180` — `s8d-ii`'s record: the run-owned worktree at `.redkiln/worktrees/<item>` under the main worktree, reused on re-entry, never force-removed and guarded (D21-44; D21-45 W1, W2, W4 and W3's carry; D21-46; D21-47 V1; D21-48 X1, X2 and X14), superseding `0105`, `0107` and `0137` in part — was written at `proposed`: this read **179 records**, `0001`–`0179`)* *(corrected 2026-09-27 at `s8d`'s build (package F2-D2173), when `0179` — `s8d`'s record R8, selection by rank, one node per activity, `rigor` bound for `require` rows and a `toolchain` row's undeclared inputs supplied (D21-73 option (C)), superseding `0170` decision 9 in part — was written at `proposed`: this read **178 records**, `0001`–`0178`)* *(corrected 2026-09-26 at `s8d`'s build (package P7), when `0177` — `s8d`'s record R5, two review verb rows and the question registry (D21-41 (d), D21-43) — and `0178` — `s8d`'s record R6, the `change` deriver in `build_item` and `change` at triage (D21-43 N3) — were written at `proposed`: this read **176 records**, `0001`–`0176`)* *(corrected 2026-09-26 at `s8d`'s build (package P6), when `0176` — `s8d`'s record R4, the kernel writes the error row and the breaker refuses from it (D21-42 20) — was written at `proposed`: this read **175 records**, `0001`–`0175`)* *(corrected 2026-09-26 at `s8d`'s build (package P5), when `0175` — `s8d`'s record R3, `on_false` as iteration (D21-42 4d; D21-48 X7) — was written at `proposed`: this read **174 records**, `0001`–`0174`)* *(corrected 2026-09-26 at `s8d`'s build (package P3), when `0174` — `s8d`'s record R1, step-grain commits (D21-42 3a, 3b; D21-43 N15, N16) — was written at `proposed`: this read **173 records**, `0001`–`0173`)* *(corrected 2026-09-26 at `s8d`'s build (package P2), when `0173` — `s8d`'s record R2, the complete reference resolver (D21-48 X8; D21-11) — was written at `proposed`: this read **172 records**, `0001`–`0172`)* *(corrected 2026-09-26 at `s8d`'s build, when `0172` — `s8d`'s record R7, the null-literal presence record (D21-67 ruling 2) — was written at `proposed`: this read **171 records**, `0001`–`0171`)* *(corrected 2026-09-26 at `s8c`, when `0169`, `0170` and `0171` — `s8c`'s records T, V and B — were written at `proposed`: this read **168 records**, `0001`–`0168`)* *(corrected 2026-09-26 at `s8b`, when `0168` was written at `proposed`: this read **167 records**, `0001`–`0167`; caught by Checkpoint 8a's guard at Checkpoint 8b's formal run)* *(corrected 2026-09-24 at `s8a`, when `0165`, `0166` and `0167` were written at `proposed`: this read **164 records**, `0001`–`0164`; corrected again 2026-09-25 at `s8a` fix round 6, because `s8a` had moved only the next-free sentence below)* *(corrected 2026-09-24 when `0164` — every text file is LF in every checkout — was written at `proposed`: this read **163 records**)* *(corrected 2026-09-24 at `s8`'s re-open, D21-40: this read **130 records**, `0001`–`0130` — stale since ADR-0131 on 2026-09-21; the census is `references/adr/README.md`'s)*, with `0130` — the recorded `Config` written to the run-record layout, repairing residual **R-54** — written at Step 7d Slice 3's close on 2026-09-20 as `proposed` *(this read **129 records**, `0001`–`0129` until then)*, with `0128` and `0129` written at Step 7d's stage-open on 2026-09-18 as `proposed` and accepted 2026-09-19 *(this read **127 records**, `0001`–`0127` until then)*, with `0126` and `0127` written at Step 7c's stage-open on 2026-09-17 *(this read **125 records**, `0001`–`0125` until then)*, with `0123`, `0124` and `0125` written at Step 7b-ii on 2026-09-17 *(this read **122 records**, `0001`–`0122` until then, **123** until `0124`, and **124** until `0125`)*, `0090` written at Step 4.4 on 2026-09-12, `0117` on 2026-09-13, `0091` at Step 6.1 on 2026-09-13, `0118` for Step 7a on 2026-09-13, `0119`, `0120` and `0121` for Step 7b on 2026-09-13 and `0122` at Step 7b Slice 4 on 2026-09-14 — no reserved number remains; `references/adr/ERRATA.md` for corrections to
frozen records) *(Corrected 2026-09-15: this read "**121 records**, `0001`–`0121` contiguous" from before **ADR-0122** was committed at `71b20c1` on 2026-09-14. Re-measured on this tree 2026-09-15: `ls references/adr/*.md | wc -l` → **124** files, of which **122** are numbered records — **118** `accepted`, **4** `superseded`, **0** `proposed` — plus `README.md` and `ERRATA.md`. The count in the parenthetical above is the RECORD count, which is what this sentence has always meant.)* *(Corrected 2026-09-13 four times: this read "**120 records**, `0001`–`0120` contiguous" until ADR-0121 — commutative Kleene absorption, superseding ADR-0103 decision 4's connective sentence in part for Step 7b Slice 3 — was written, "**119 records**, `0001`–`0119` contiguous" until ADR-0120 — the `dag_amended` event kind AC-064's THEN requires, minted by record because ADR-0116 permits no other route — was written, "**118 records**, `0001`–`0118` contiguous" until ADR-0119 — the selection memo key, settling AC-045's fourth key for Step 7b — was written, and "**117 records**, `0001`–`0117` contiguous" until ADR-0118, the lock-poisoning halt closing the residual ADR-0115 assigned to Step 7a.)* · `spikes/DECISIONS-2026-09-11.md`, `spikes/DECISIONS-2026-09-12.md`, **`spikes/DECISIONS-2026-09-14.md`** and **`spikes/DECISIONS-2026-09-15.md`** (the operator decisions this runbook executes — the 2026-09-11 log runs to #27; the 2026-09-12 log carries E1–E13 from the alignment review and is cited as `DECISIONS-2026-09-12 E<n>`; the **2026-09-14** log carries **D-1…D-14**, the decisions Step 7b Slice 4 was not permitted to take, cited as `DECISIONS-2026-09-14 D-<n>` — seven executed in code, two build-scope, five landed 2026-09-15 as record work above the runbook *(corrected 2026-09-15: this read "five still owed as record work above the runbook", and a later cut left "five record work above the runbook" — a noun phrase with no verb. The five — **D-1**, **D-3**, **D-12**, **D-13**, **D-14** — were landed on **2026-09-15**, with two obligations carried forward rather than absorbed: **ADR-0122 decision 5(c)** as residual **R-2**, and **AC-071's stability record itself**, which lands in the corpus and `s7b` §7b.11 precondition 7 reserves to the operator)*; the **2026-09-15** log carries **D15-1…D15-4**, the four free residuals that surfaced while landing those five and that no draft had named, cited as `DECISIONS-2026-09-15 D15-<n>`, and it names six residuals **R-1…R-6** it does not settle; the **2026-09-17** log carries **D17-5…D17-13**, the routings RT-1…RT-27, the open items O-1…O-11 and the residuals R-7…R-24 for Step 7c, cited as `DECISIONS-2026-09-17 D17-<n>`; and the **2026-09-18** log carries **D18-1…D18-18**, RT-28…RT-59 (RT-40 withdrawn by a dated line), O-12…O-16 and R-25…R-36 for Step 7d, cited as `DECISIONS-2026-09-18 D18-<n>` *(added 2026-09-18 at Step 7d's stage-open: the 2026-09-17 log was not listed in this sentence when Step 7c wrote it)*) ·
`spikes/F39-DRAFT.md` · `spikes/F40-DRAFT.md` · `spikes/F41-DRAFT.md` (S-PORTS) · `spikes/F42-DRAFT.md` (S-GRAMMAR) ·
`spikes/HANDOFF.md` · `spikes/DECISIONS-2026-09-09.md` · `references/2026-09-08-evidence-brief.md` §E. ·
**Tree state:** every path and every existing command in the stage files was re-verified against the working tree
on **2026-09-11** at the remediation branch's head (the previous revision's verification dates, 2026-09-08 and
2026-09-09, stand for the paths it did not touch). · **Commands that do not exist yet** — because the code is not
written — are marked **`[WILL EXIST]`** and are written in the form they will take; a `[WILL EXIST]` checkpoint
line is **deferred, not failed** (Halt 8's carve-out), and each names the stage at which it is re-run. Nothing else
in this runbook is aspirational.

**What this runbook decides, stated honestly.** It is not decision-free. It takes **seven** decisions; five are ADRs
and two are numbered decisions recorded in `spikes/DECISIONS-2026-09-11.md` before the stage that executes them:

| # | Decision | Where | Record |
| --- | --- | --- | --- |
| 1 | The context-vector source | **Step 4** — the blocking gate | **ADR-0090**, superseding ADR-0079 in full; carries the three LLM booleans' disposition (DECISIONS-2026-09-11 #2) |
| 2 | Whether S-GIX narrows the git write path | Step 6.1 | **TAKEN 2026-09-13** — **ADR-0091**, accepted: `S-GIX/a: go`, `S-GIX/b: no-go`, `S-GIX/c: go`. It narrows **nothing**: arm b is the only arm whose go could have withdrawn a sentence of ADR-0077, and it is a no-go, so 0077's write path stands in full and 0091 carries `supersedes: []` |
| 3 | The minimum `git` executable version the wrapper asserts | Step 6.2 constraint 3 | **TAKEN 2026-09-12**, **final number recorded 2026-09-13** — `>= 2.20`; **DECISIONS-2026-09-12 E10** and **ADR-0091** decision 4. The conditional half resolved downward: `git clone --revision` (git 2.49) is reached only on an S-GIX/c no-go, and /c is a **go**, so gix performs the review clone and nothing raises the floor above 2.20 |
| 4 | `doctor`'s unread-artifact window `N` | Step 7d | **TAKEN 2026-09-12** — `unread_window_runs = 50`; **DECISIONS-2026-09-12 E9**. In the `doctor` rule table and `init`'s scaffold, cited by AC-472/AC-426 *(Amended 2026-09-18, DECISIONS-2026-09-18 D18-5 / D18-18, C-01 D-08: N's ONE home is the per-repository key `[doctor] unread_window_runs` that `init` writes — read by the `RK-D-UNREAD` row, **not** a constant in the rule table; AC-472's "declared constant in the rule table" took the same dated amendment)* |
| 5 | `stale_policy` per cost class | Step 7e — pack authoring | **DECISIONS-2026-09-11 #4** (`rematerialise`, coarse scope, every class); recorded per class with its warrant in the pack; Checkpoint 8 asserts no value was defaulted from P2 |
| 6 | Per-AC mutants are author-declared | Step 5.3 (the spec template's `mutation:` field) and 7b-ii | **ADR-0104** |
| 7 | The Phase-1 companion kind list | Step 5.3 | **DECISIONS-2026-09-11 #3** — `_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger`; recorded against ADR-0075 clause 3 |

**An eighth, added 2026-09-12.** The 2026-09-11 audit left the two must-fail fixture crates (AC-001–AC-004, AC-005) without a name, a path or a shape, and this table did not carry them as a decision. They are now decided — feature-gated workspace members at `tests/fixtures/rk-clock-read/` and `tests/fixtures/rk-banned-dep/`, **DECISIONS-2026-09-12 E4**, executed at s5 §5.1a. Five further decisions from the same review (**E1**, **E2**, **E3**, **E11**, **E12**) move a record at or above the runbook in ADR-0092's chain and are **owed, not executed here** — `DECISIONS-2026-09-12` names what each one must write.

**`0089` is taken, and so are `0092`–`0122` and now `0090`.** `references/adr/0089-facet-provenance-declared-surfaces-and-abstention.md`
was accepted on 2026-09-09 and partially supersedes ADR-0030 (PC8); ADR-0093 (2026-09-11) supersedes it in part
again — `stage`/`rigor`/`role`/`cost` are `source = "kernel"`, `surfaces` is `source = "declared"`, six facets stay
`derived` — and Step 4a executes both. Step 4.4's context-vector record is **0090** — **written 2026-09-12**, superseding ADR-0079 in full — and Step 6.1's
S-GIX successor is **0091**, **written on 2026-09-13**, which was the last number the ADR README carried as reserved. Step 7a's
lock-poisoning record is **0118**, **written on 2026-09-13** — a poisoned store lock halts the run; it closes the residual
ADR-0115:135-138 named and assigned to this stage, and it supersedes nothing. Step 7b's selection-memo-key record is
**0119**, **written on 2026-09-13** — AC-045's key is a fourth, selection-level key derived in `rk-kernel::select` and
looked up outside `select()`; ADR-0097's step memo key gains no ninth component, so it too supersedes nothing. Step 7b's
second record is **0120**, **written on 2026-09-13** — the closed `EventKind` table gains **`dag_amended`** (tier 2 ·
`deviation_id, added, removed, rows_refired` · selection step 7's re-run of steps 2–5), the kind AC-064's THEN names and
that nothing minted; ADR-0116 had already withdrawn the completeness reading of ADR-0105 clause 7, so it supersedes
nothing either. Step 7b's **third** record is **0121**, **written on 2026-09-13** at Slice 3 — `and` and `or` are
Kleene's strong three-valued connectives and **both** absorbing positions absorb (`null and false` = `false`,
`null or true` = `true`), which **supersedes ADR-0103 decision 4's connective sentence IN PART**: exactly two cells of
the two truth tables move, every other clause of decision 4 and decisions 1, 2, 3, 5 and 6 stand, and ADR-0103 stays
`accepted` carrying `superseded_by: ["0121"]` *(dated 2026-09-26, `s8d` build: now `["0121", "0172"]`, and ADR-0121's is `["0172"]` — ADR-0172, written `proposed`, supersedes both in part for `==`/`!=` against the literal `null`, D21-67 ruling 2)*. It is a departure **on the merits**, not a clarification — the record as
written forces the literal, left-sequential reading, and four live `spikes/p9` predicates break under it. Step 7b Slice 4's record is **0122**, **written on
2026-09-14** — register §0.3's **C1**, its oldest open contradiction, resolves for **AC-026 and AC-232**: a
harness-internal `system/api_retry` attempt **charges** the activity's attempt budget, one charge per recorded
event in the pass that dispatched it, and **AC-391's *Cut* does not stand**. It supersedes **nothing** —
ADR-0037's whole `## Decision` is a load-time lint rule on a **declared** field and nothing in it moves, so
`supersedes: []` and ADR-0037 keeps `superseded_by: []` — and the register amendments its decision 5 owes are
`DECISIONS-2026-09-14 D-1`'s, not this runbook's. Step 7b-ii's record is **0123**, **written on 2026-09-17** — the
closed `EventKind` table gains **`gate_answer_spent`** (AC-246's *"records a distinct event"*; AC-271) and
**`gate_resumed`** (the event AC-267, AC-279 and SM-073 assert on), both tier 2, on ADR-0120's exact precedent; it
supersedes **nothing**, and its decision 5 is what makes Checkpoint 5's `PROGRESS.md` row provisional. Step 7b-ii's
second record is **0124**, **written on 2026-09-17** — the table gains the review and verify gates' five kinds
(`review_block`, `quarantine_granted`, `quarantine_removed`, `quarantine_expired`, `selection_ineffective`), each
named by a criterion or a success measure, on the same route; it supersedes **nothing**. Its third record is
**0125**, **written on 2026-09-17** — the table gains `stage_exit_refused`, the verify gate's refusal to let a stage
exit over an unpinned AC (AC-346), on the same route after the operator was asked once more with the alternatives
named; it supersedes **nothing**. Step 7c's two records are **0126** and **0127**, **written on 2026-09-17** at the
stage-open commit — `0126` mints `dispatch_completed`, the usage-bearing row AC-360 requires on every termination
path (`EventKind` 37 → 38 at Slice 1; committed `proposed` and flipped to `accepted` once D17-8's confirmation was
written), and `0127` homes process containment in a sixth Phase-1 crate, `rk-containment`, superseding ADR-0099
decision 3 **in part** (its location clause only). Step 7d's two records are **0128** and **0129**, **written on 2026-09-18** at
the stage-open commit, both committed `proposed` and **flipped to `accepted` on 2026-09-19** on the operator's dated
confirmation line under `spikes/DECISIONS-2026-09-18.md` D18-17 (ADR-0126's route) — `0128` fixes the one `run`
grammar, `redkiln run <item> [--lane fix|story]`, superseding ADR-0107 decision 4's `--lane fix --item <item>` sentence **in
part** (every other clause of 0107 stands), and `0129` fixes the `answer` grammar and the ADR-0063 question registry,
superseding ADR-0009's literal `approve|changes-requested` sentence **in part** (its CLI-only channel clause stands). Both
were forced by the C-01 pass: each grammar sat in a Decision sentence, so ADR-0092 allowed neither an erratum nor a
runbook choice. The next free number is **`0197`**.
*(Corrected 2026-09-29 at `s8g`'s build (package R-D2119-8g): this read **`0196`** until `0196` — `s8g`'s Findings-key record (D21-119) — was written, at `proposed` *(dated 2026-10-02, `s8g` close: ADR-0196 accepted on D21-136)*.)*
*(Corrected 2026-09-29 at `s8g`'s build (package S8G-P1-RECORDS): this read **`0191`** until `0191`–`0195` — `s8g`'s five records R1–R5, the verify, schema, write-set, producers and tag records — were written, at `proposed` *(dated 2026-10-02, `s8g` close: ADR-0191 accepted on D21-131, ADR-0195 on D21-135)*.)*
*(Corrected 2026-09-29 at `s8f`'s build (package R1FIX-8f-B-RECORDS): this read **`0188`** until `0188` — `s8f`'s landing record, the landed commit a kernel-made content checkpoint on the run ref —, `0189` — `s8f`'s render record, `rk-store`'s render, write and read path — and `0190` — `s8f`'s telemetry-key record, `activity_id_at_version` on `dispatch_completed` — were written, at `proposed` *(dated 2026-10-02, `s8g` close: ADR-0188 accepted on D21-117 at `s8f`'s close, ADR-0189 on D21-118 at `s8f`'s close, ADR-0190 on D21-130)*; round 1 found the README moved and this sentence not.)*
*(Corrected 2026-09-28 at `s8e`'s build (package S8E-RULINGS): this read **`0187`** until `0187` — `s8e`'s record of D21-92, a routing `null` journaled when its route continues the walk — was written, at `proposed`.)*
*(Corrected 2026-09-28 at `s8e`'s build (package S8E-P7): this read **`0186`** until `0186` — `s8e`'s slice record, the declared slice sealed on both review outcomes — was written, at `proposed`.)*
*(Corrected 2026-09-28 at `s8e`'s build (package S8E-P5): this read **`0185`** until `0185` — `s8e`'s three-kinds record, `worktree_assigned`, `stage_empty` and `git_op` — was written, at `proposed`.)*
*(Corrected 2026-09-28 at `s8e`'s build (package S8E-P4): this read **`0183`** until `0183` — `s8e`'s stage-location record, a stage-bearing `Story:` mark at every boundary — and `0184` — `s8e`'s `Gate:` record, kind, reach and state with an integer deadline — were written the same day, at `proposed`.)*
*(Corrected 2026-09-28 at `s8e`'s build (package S8E-P3): this read **`0181`** until `0181` — `s8e`'s tier-1 writer record, `append_tier1` written git-first by the kernel — and `0182` — `s8e`'s interim `item` key record — were written the same day, at `proposed`.)*
*(Corrected 2026-09-27 at `s8d-ii`'s build (package P1-RECORD): this read **`0180`** until `0180` — `s8d-ii`'s record, the run worktree at `.redkiln/worktrees/<item>`, reused and guarded (D21-44, D21-45, D21-48 X1/X2) — was written the same day, at `proposed`.)*
*(Corrected 2026-09-27 at `s8d`'s build (package F2-D2173): this read **`0179`** until `0179` — `s8d`'s record R8, selection by rank, one node per activity, `rigor` bound and row inputs supplied (D21-73) — was written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8d`'s build (package P7): this read **`0177`** until `0177` — `s8d`'s record R5, two review verb rows and the question registry — and `0178` — `s8d`'s record R6, the `change` deriver in `build_item` and `change` at triage — were written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8d`'s build (package P6): this read **`0176`** until `0176` — `s8d`'s record R4, the kernel writes the error row and the breaker refuses from it — was written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8d`'s build (package P5): this read **`0175`** until `0175` — `s8d`'s record R3, `on_false` as iteration — was written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8d`'s build (package P3): this read **`0174`** until `0174` — `s8d`'s record R1, step-grain commits — was written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8d`'s build (package P2): this read **`0173`** until `0173` — `s8d`'s record R2, the complete reference resolver — was written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8d`'s build: this read **`0172`** until `0172` — `s8d`'s record R7, a comparison against the literal `null` is a definite presence test (D21-67 ruling 2) — was written the same day, at `proposed`.)*
*(Corrected 2026-09-26 at `s8c`'s build: this read **`0169`** until `0169`, `0170` and `0171` — `s8c`'s records T, V and B — were written the same day, all three at `proposed`.)*
*(Corrected 2026-09-26 at `s8b`: this read **`0168`** until `0168` — the S-SEV record, the severity vocabulary and T — was written the same day, at `proposed`; Checkpoint 8a's guard caught it at Checkpoint 8b's formal run.)*
*(Corrected 2026-09-24 at `s8a`: this read **`0165`** until `0165`, `0166` and `0167` were written the same day, all three at `proposed`.)*
*(Corrected 2026-09-24: this read **`0164`** until `0164` was written the same day.)*
*(Corrected 2026-09-24 at `s8`'s re-open, D21-40: this read **`0132`**, stale since ADR-0132 was written at `s7f`; `0131`–`0163` are all written.)*
*(Corrected 2026-09-21 at Step 7d Slice 4: this sentence gave the next free number as `0131` until ADR-0131 — the event store's tag-set selector, outside the canonical digest — was written on 2026-09-21, at `proposed`.)*
*(Corrected 2026-09-20 at ADR-0130's flip: this sentence gave the next free number as `0130` until ADR-0130 — the recorded `Config` written to the run-record layout, repairing residual **R-54** — was written at Step 7d Slice 3's repair commit on 2026-09-20 and accepted the same day on the operator's dated line under `spikes/DECISIONS-2026-09-20.md` D20-1. The sentence went stale at the authoring commit rather than at the flip, and `references/adr/README.md`'s own numbering row had already been moved to `0131`; a next-free-number that names an allocated record is how a number gets reused, which the numbering rule forbids.)*
*(Corrected 2026-09-18 at Step 7d's stage-open: this sentence gave the next free number as `0128` until ADR-0128 and ADR-0129 were written.)*
*(Corrected 2026-09-17 at Step 7c's stage-open: this sentence gave the next free number as `0126` until ADR-0126 and ADR-0127 were written.)*
*(Corrected 2026-09-17, three times: this sentence gave the next free number as `0123` until ADR-0123 was written at Step 7b-ii, as `0124` until ADR-0124 was, and as `0125` until ADR-0125 was.)*
*(Corrected 2026-09-15: this sentence gave the next free number as `0122` until ADR-0122 was written at Step 7b
Slice 4 on 2026-09-14 and committed at `71b20c1`; it is `accepted`, dated 2026-09-14.)* *(Corrected 2026-09-13 four times: this sentence gave the next free number as `0121` until ADR-0121 was
written, as `0120` until ADR-0120 was, as `0119` until ADR-0119 was, and as `0118` before ADR-0118 was.)* *(Corrected 2026-09-12: this sentence said the next free number was 0115, from before ADR-0115
and ADR-0116 landed.)* **Step 4a is
not a decision** — it is the execution order for the registry, schema and lint work ADR-0089 and ADR-0093 force.

Everything else this runbook does is execution order. **Where a stage is tempted to decide something else, it stops.**

**Invoking things.** All commands assume the repo root `D:/repos/redkiln-rs` unless the block opens with its own
`cd`; **every block that needs a working directory now states it**. The rig is
`cd D:/repos/redkiln-rs/spikes/runner && cargo build && ./target/debug/redkiln-spike-runner <cmd>`. Python tooling
runs from the repo root as `python spikes/<lane>/<script>.py`. The predecessor repo — the frozen corpus — is
`D:/repos/redkiln` at `47faec9f46b8d983c2c75f0794164e68415ae706`.

**Shell.** Every shell block in every stage file is **POSIX and is executed in Git Bash**, which is present on this box at
`/usr/bin/*`. This is not a stylistic preference: the file uses `rm -rf`, `cat <<'EOF'` heredocs, `sed -i`,
`printf`, `wc -l`, `grep -E/-c/-L`, `date -u`, `test -f/-s/-d` and leading `!` negation, **none of which work in
PowerShell** (`rm` there is `Remove-Item` and rejects `-rf`; there are no heredocs; `!` is `-not`). Open Git Bash,
`cd /d/repos/redkiln-rs`, and run them there. PowerShell equivalents are given only where a step has no POSIX
route. Drive paths (`D:/repos/...`) work unchanged in Git Bash.

**Tool preflight — checked 2026-09-11; all present.** Run this before Step 0 and install what it names:

```bash
for t in sed awk wc grep date python cargo rustc git claude; do   # awk: Checkpoint 4a's deriver-count line
  command -v "$t" >/dev/null || echo "MISSING: $t"
done
cargo audit --version  >/dev/null 2>&1 || echo "MISSING: cargo-audit  → cargo install cargo-audit --locked"
cargo insta --version  >/dev/null 2>&1 || echo "MISSING: cargo-insta  → cargo install cargo-insta --locked"
cargo deny  --version  >/dev/null 2>&1 || echo "MISSING: cargo-deny   → cargo install cargo-deny --locked"
rustup run 1.98.1 rustc --version >/dev/null 2>&1 || echo "MISSING: toolchain 1.98.1 → rustup toolchain install 1.98.1 --profile minimal -c rustfmt -c clippy"
# Measured on this box 2026-09-11: cargo-audit 0.22.2, cargo-insta 1.48.0, cargo-deny 0.20.2 and rustc 1.98.1
# are all present (the first two and the toolchain were installed during the 2026-09-11 remediation pass; the
# 2026-09-08 measurement had cargo-audit and cargo-insta MISSING). Step 5 requires all four; Checkpoint 5 cannot be
# run without them. `rustup` self-update fails on this box (os error 2) — pass `--no-self-update`.
```

---

## Preamble — the golden constraints

These hold for every step below. They are not advice; each one is a measured failure this programme already paid for.

1. **Nothing in the selection path over LLM-emitted facets starts before Step 4's gate clears.** ADR-0079 is
   `superseded` — it was `proposed`, and the only non-accepted record in the set other than two full
   supersessions, when this constraint was written, and it **forbade being amended into a decision**.
   *(Corrected 2026-09-15: this read "ADR-0079 is `proposed`, is the only non-accepted record in the set other
   than two full supersessions, and **forbids being amended into a decision**". **ADR-0090** superseded it in
   full at Step 4.4 on 2026-09-12, and 0079's frontmatter reads `status: superseded`, `superseded_by:
   ["0090"]` today. **THE BLOCK ITSELF IS UNCHANGED IN FORCE**; only the record's status
   label was stale, and the prohibition was honoured rather than lifted — 0079 was never amended into a
   decision, a successor was written instead. Under `DECISIONS-2026-09-15 D15-2` a stale status label in an
   amendable file takes a **dated in-place correction and not an erratum**, and **`E-0079-2` is withdrawn as an
   instrument, not as an obligation** — this paragraph is where that obligation lands.)* Until Step 4 closes, no `require` row may be keyed on an LLM-sourced `criticality`,
   `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro`. `kind` is out of scope of the
   block (human input, ADR-0030 rule 4). The engine's *semantics* are decided and may be built now; its *input* may
   not. [F36: modal `R` identical on 5/20, 3/20, 2/20, 2/20 against an 80% line; 400 dispatches, 0 void, $21.37]

   **F39 does not lift this and moves only one clause of it.** AC-071 has five clauses — identity ≥ 0.80, n ≥ 20
   items, ≥ 10 draws per item, arm-D panel **scored**, corpus pinned — and P1 arm D's scoring on 2026-09-09
   **discharges the fourth clause only** [measured, F39]. The block still stands on the **first**: modal
   required-set identity was measured at **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line [measured, F36]. F39
   measures a *derived* context vector; ADR-0079 asks about an *LLM-emitted* one, so F39 narrows 0079's options
   without deciding among them and **0079 stayed `proposed`** until Step 4.4 wrote its successor — which it did,
   as **ADR-0090**, on 2026-09-12. *(Corrected 2026-09-15 with the label above: the sentence was written before
   Step 4.4 ran and its tense was never moved. The clause it supports is unchanged.)*

2. **Nothing may depend on a process surviving.** Anything long-running is a durable mark plus a later read. F22
   measured ~12 background kills against one 12,000 s survivor under `Start-Process`; `issue-81` was killed three
   times as a background job and completed in **116 s** in the foreground. Waits are marks, not sleeps; the rig
   already splits `ttl-mark` / `ttl-read` for exactly this reason.

3. **`cargo test` and `cargo clippy` DO NOT rebuild the binary. Always `cargo build` first.** This cost the
   programme four wasted runs and two false conclusions that code was broken. The invocation is
   `cd spikes/runner && cargo build` — *then* run the binary from `target/debug/`.

4. **A rig that compiles and passes its unit tests has NOT been run.** Three E3 defects survived "built and
   validated": `--json-schema` silently inert under a tool-restricted card, a `\\?\` path handed to `git fetch`,
   and `git fetch <abbrev-sha>` failing on every path that did not happen to call `rev-parse` first. None was
   reachable from a unit test, because none touched the filesystem or the CLI. **Validate one item before every
   set**, and prefer a haiku probe to an Opus one when the question is "does this flag work at all".

5. **Every waiter needs a failure alternative.** A marker-only wait spins forever on a dead job — one `until grep`
   waiter spun 25 minutes on a job that had already failed. Every wait is `grep -qE "MARKER|exited with code"` or
   equivalent, never `grep -q "MARKER"`.

6. **A default written for the fast condition costs a record on the slow one.** The 900 s timeout fitted S-SILENT
   and lost a CLEAN record; the ladder ran 900 → 2,400 → 3,600 → one 5,400 s retry. When two conditions differ in
   shape, size the ceiling against the slow one.

7. **Scripted edits need an anchor assertion.** Two silent `str.replace` no-ops were read as behaviour changes.
   Every scripted edit asserts its anchor exists *before* it writes, and fails loudly if it does not.

8. **The instrument version is part of every record.** F20 and F37 §7: a version move mid-run splits a comparison.
   P7 straddled 2.1.263 → 2.1.265, catching exactly `recommended`'s r3 draw on all ten items while `optional` ran
   wholly on 2.1.263 — no effect is detectable, and **the arms are still not instrument-matched**. Record the
   version; never silently continue across a move.

9. **A guard scoped wrong blocks or voids valid work.** HANDOFF, "Traps that cost time here": *"Two guards I wrote
   to protect data were scoped wrong and blocked or voided valid work."* This is the constraint that cuts **both**
   ways, and it is the one this runbook most nearly lost. A guard too narrow lets the failure through
   (`score-arm-d.py`'s facet regex admits seven facets and silently ignores a `recent_deploy` label, s3 §3b); a guard
   too wide refuses a valid run (a whole-second `labelled_at` refused against a microsecond engine stamp in the same
   second, s4 §4.3 — deliberate and correct, but only because it was *chosen*). **When you meet a guard, establish its
   scope before you obey it and before you route around it.** The scope is the finding, not the verdict.

10. **A pre-registered bar whose registered n cannot decide it is powerless, and must be reported as powerless.**
    **Four** measured instances now, all in the same pre-registration: P1's α rule [F36], P7's criterion 1 —
    powerless at n=3, every Wilson interval overlapping by construction [F37] — METHOD-P §1.4's 0.95 precision bar,
    which **no facet can clear at n=20 however good the method**: `ui` is 7/7 and its 95% lower bound is 0.65, and
    **73** all-correct positives are needed for a lower bound above 0.95 at the registered two-sided z = 1.96 (92 with continuity correction; 72 under Clopper–Pearson) [measured, F39 Limits 1 as corrected 2026-09-11, PIA-073] — and now **P2 §2.4's
    criteria 1 and 2**, whose recall half has no denominator at all: **0 of 41 rows are "should re-run"**, so recall
    is **NOT ANSWERABLE** and criterion 2 cannot fire either way [measured, F40]. **Four is a pattern in how this
    programme writes pre-registrations, not four coincidences** — each fixed a threshold without checking that the
    registered corpus could produce the denominator the threshold needs. **Before you run a
    criterion, ask what n it needs; after you run one, report saturated as saturated, powerless as powerless,
    not-measurable as not measurable and not-answerable as not answerable — never as a pass or a failure.** **Not-measurable and not-answerable are different faults and both are live:** F39's `data_migration` was NOT MEASURABLE (the script emitted no positive, so there was nothing to be precise about); F40's P2 recall is NOT ANSWERABLE (the label set carries no positive, so the denominator does not exist). This is a result *about* the
    pre-registration, reported in the finding; it is **not** a licence to edit METHOD-P after the numbers are in.

11. **A criterion that fires may prescribe a remedy its own measurement contraindicates. Record both halves.**
    P2's criterion 1 fires on its precision half — coarse `derived_from` precision **0.00** [0.00, 0.26] over 11
    emitted positives — and the remedy it prescribes, *"fine scope becomes mandatory for `L`-class activities"*, is
    **contraindicated by the same run**: on the 33 rows both scopes scored, coarse marks **5** `Stale` and fine
    marks **15**, and **10 rows flip `Present` → `Stale` going coarse to fine while none flips the other way**
    [measured, F40 §3]. A document that records the criterion as firing without recording that its own remedy is
    contraindicated has absorbed half the finding. **The design consequence is OPEN, and it needs a decision, not a
    default.** *(Decided 2026-09-13 by **ADR-0117**: the value stays `rematerialise` at coarse scope, and the
    contraindicated remedy is not applied. The constraint's rule stands for any later criterion of this shape. Noted 2026-09-24 at
    `s8`'s hygiene pass; this sentence still read OPEN.)*

12. **Production code is written test-first, and every criterion is discharged by a test that discriminates.**
    This is not a new rule — **ADR-0019** makes it the product's *normal* evidence standard and this runbook simply
    executes it: *"'Reviewed and green' is not evidence that a criterion is pinned. The per-AC discriminating test —
    TDD's normal mutation evidence — is the product's normal evidence standard, not an exceptional one."* Its
    root-cause note is the rule in one line: **Red→Green evidenced per test; an AC is a claim about one assertion.**
    The measured failure it exists to stop: **0 of 156** historical reports carried per-AC mutation evidence, and
    **≥ 12 of 23** shipped, human-approved fixes carried an AC that no test pinned [ADR-0019 drivers, proposal
    §4.6 item 8; F24/F25]. From Step 5 on, every sub-step that writes Rust runs in this order:

    - **Red.** Author the named test, run it, and **watch it fail for the right reason** — the absent behaviour,
      not a compile error, a typo'd filter or a missing fixture file. A test never observed red is not evidence
      that it can fail, and a filter that matches nothing exits 0 (Halt 8's shape, one level up).
    - **Green,** by the smallest change that satisfies it.
    - **Discriminate.** Every AC in the stage's acceptance set needs a test that fails when the behaviour is
      absent. Where the criterion declares a mutant — the `mutation:` field ADR-0104 makes author-declared and the
      harness's input — the mutant is authored, run, and **must flip the test**; a survivor blocks even when
      quarantined. **ADR-0096**'s typed repro (`kind`, `mutation`, machine-readable `expected`) carries the evidence,
      and a prose-only `command` is rejected at ingest.

    **A checkpoint grep is not this.** Asserting that a test named `X` exists and exits 0 is satisfied by an
    implementation nothing pins — exactly the gap between "green" and "pinned" that ADR-0019 was written about. The
    checkpoint is the floor, not the evidence. Constraint 4 is the same rule one level up: a rig that compiles and
    passes its unit tests has **not** been run.

**Where each constraint actually bites.** They are restated at these steps because the preamble is not where an
operator meets them:

| # | Restated at |
| --- | --- |
| 1 — the LLM-facet block | Step 4 (the gate), **Step 5 banner**, **Step 6 banner**, **Step 7a/7b banners**, **Step 7b-ii's gate-check banner** (AC-302 is its standing negative), Step 7e, Step 8 |
| 2 — nothing depends on a process surviving | Step 3a (the dispatch), Step 7c, Step 8 (3), Halt 5 |
| 3 — `cargo build` first | Checkpoint 0, Step 5.3, Step 6.1, Halt 1 |
| 4 — a rig that compiles has not been run | Step 3a, Step 7c |
| 5 — every waiter needs a failure alternative | Step 3a (the dispatch), Halt 5 |
| 6 — a default written for the fast condition | Step 3a (the dispatch timeout) |
| 7 — scripted edits need an anchor assertion | Step 3a, **Step 4a** (the registry edits), **Step 4.4** (the two ADR edits), Step 1.2 |
| 8 — instrument version in every record | Step 0.3, Step 1.3, Step 3a, Step 4.1, Halt 2 |
| 9 — a guard scoped wrong | Step 3b, Step 4.3, Halt 7 |
| 10 — a bar its own n cannot decide | **Step 4.1 clause B**, **Step 4.2 (P2's criteria 1 and 2)**, Halt 9 |
| 11 — a fired criterion whose remedy is contraindicated | **Step 4.2 (P2's criterion 1)**, Step 4.5, **Halt 12** |
| 12 — test-first, and a discriminating test per AC | **Step 5.3**, **Step 6.3**, **Step 7a**, **Step 7b**, **Step 7b-ii**, **Step 7c**, **Step 7d** — every stage that writes Rust. Step 7e authors TOML and is out of its scope; Steps 0–4a write records, not code. |

---

---

## Legend and conventions shared by every stage file

- **`[WILL EXIST]`** — a command or path the code has not produced yet, written in the form it will take. In a
  checkpoint it is **deferred, not failed** (Halt 8's carve-out): record the deferral in `PROGRESS.md` and re-run
  the line at the stage the line names. A `[WILL EXIST]` line that goes green because the binary is missing is the
  Halt 8 shape and is never a pass.
- **`[measured]` / `[documented]` / `[inferred]`** — as in the design and the registers: computed from a record in
  this repository (named); read from a live source (with a fetch date); reasoned.
- **Fault names** — saturated, powerless, not-measurable, not-answerable are four different faults and are named as
  such; never `0.00`, `1.00` or `n/a`.
- **Cross-references** — `Step N`, `Checkpoint N` and `Halt N` are global names; a bare `§N.M` refers to the stage
  file it appears in; another stage's section is `sN §N.M`; other documents are prefixed (`design §10.5`,
  `METHOD-P §2.4`, `AC §12.10`).
- **Checkpoints** — every checkpoint line is `command   # → expected result`. A checkpoint carries a BEFORE
  assertion (a line that fails on the untouched tree) beside its AFTER assertions wherever the audit found one that
  was green before the step ran (PIA-035); a checkpoint you could not run is a failed checkpoint (Halt 8).
- **Acceptance sets** — each Phase-1 stage lists the criteria it claims as an explicit range **minus** the ids it
  excludes, naming each excluded id and why (retired, PROVISIONAL, CONDITIONAL, human-gated, Phase 2+, or blocked on
  a named spike).
- **A checkpoint that changed after it passed is provisional.** When a stage file's checkpoint lines are edited
  after that checkpoint was recorded as passed, the recorded pass attests the *old* text. This has already
  happened once: `95e0164` (2026-09-12) rewrote every line of Checkpoint 0 and every line of Checkpoint 1, both of
  which were recorded as passed at `bf5086a` and `66276bf` before that commit existed — and the pre-hardening forms
  are the ones `halts.md` now names as unable to fail. **Mark the row `passed <hash>, re-run pending <reason>` in
  `PROGRESS.md` §1 and re-run the hardened block**; a pass is evidence about a specific text, not about a stage.
- **A stage closes in two commits, not one.** `INDEX.md` says to record the result in `PROGRESS.md` "in the same
  commit as the work", and §1's legend wants `checkpoint passed <hash>` — which are jointly unsatisfiable, because
  the row would have to embed the hash of the commit that contains it. The executed pattern at all three closed
  stages was **(1)** the work and its checkpoint outputs, then **(2)** a tracker commit writing that commit's own
  hash into §1 (`a64f1f2`, `8a2b5a3`, `54b61d8`; Step 0 differed — `bf5086a` touches `PROGRESS.md` alone). A §1 row
  is hashless only between the two. Any cleanliness assertion in a checkpoint must tolerate the tracker row being
  unstaged at the moment it runs.
- **Execution order** — `0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7a → 7b → 7b-ii → 7c → 7d → 7e → 7f → 7g → 7h → 7i → 8a → 8b → 8c → 8d → 8d-ii → 8e → 8f → 8g → 8h → 8i → 8j → 8` *(the eleven `s8` sub-stages `8a`–`8j` were inserted before `8` on 2026-09-24, D21-40; INDEX.md)*; Step 2 runs sixth (its
  ORDER banner); Step 9 is rollback and may run at any point.
