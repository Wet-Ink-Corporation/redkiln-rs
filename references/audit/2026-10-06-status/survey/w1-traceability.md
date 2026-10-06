# W1 — Traceability audit (AC register, SM register, evidence ledger)

Snapshot: worktree at commit `36dd8ac` (branch `build/phase-0-to-1`). Read-only. No cargo run; "evidenced" below means *a test or checkpoint exists in the tree that asserts the criterion*, established by reading the test, not by running it. Whether the tests pass today is inherited from the runbook's recorded checkpoint passes (PROGRESS.md §1) and is NOT re-verified here.

## 1. Verdict

The programme's acceptance criteria are well specified and, for the Phase-1 set, roughly three quarters are backed by a real, discriminating test in the tree. The remaining quarter is a mix of criteria whose owner stage was never assigned, criteria whose rest-of-behaviour is not built, and a pack/catalog area that is thinly tested by id. Two structural gaps matter more than the percentage: (a) the AC register has never been mechanically reconciled to tests — there is no AC-to-test coverage report or CI gate for the programme's own 526 criteria (the "AC evidence ledger" is a *product feature* for user specs, not applied to this register); (b) the success-measure (SM) register is almost entirely un-instrumented, which is expected pre-production but means "success" is not yet measurable. Estimated Phase-1 AC evidence: **~73%** (sample hit-rate 11/15 fully evidenced, 12/15 counting the two partials at half; see §5).

## 2. The registers — counts

Source: `references/2026-09-08-acceptance-criteria.md` (8531 lines) and `references/2026-09-08-success-measures.md` (1446 lines).

| quantity | value | citation |
| --- | ---: | --- |
| ACs in force | **526** unique ids (527 `**AC-nnn**` heading lines; AC-428 appears as a heading twice — it was cut and re-issued, §0.4) | acceptance-criteria.md:43, §0.1 table at :26-41; `grep -oE '^\*\*AC-[0-9]+\*\*'` = 527 lines / 526 unique |
| of which binding / provisional / human-gated / conditional | 480 / 20 / 20 / 6 | acceptance-criteria.md:26-41 (table sums to 526) |
| Human-gated | **20** — matches INDEX | acceptance-criteria.md:26-41, :56; runbook/INDEX.md:119 ("Twenty AC rows … human-gated") |
| Conditional (distribution mechanism undecided) | 6 (AC-512, 514, 515, 516, 517, 519) | acceptance-criteria.md:7779 (Phase-1 subset note); §14 row of the §0.1 table |
| Retired | AC-513, AC-075 (cut, never reuse); AC-119 unallocated | acceptance-criteria.md:7779; s7b-kernel.md:1960-1976 |
| SM measures | **135 ids** (134 + SM-098 descriptive) | success-measures.md §5.5 (:1420); acceptance-criteria.md:43 |
| SM with a baseline / without | 77 / 58 (and ≥21 of the 77 are baselines of a different quantity or an upper bound) | success-measures.md:1422 |
| SM that cannot fail or discriminate (tripwire, canary, tracker, descriptive…) | 48 of 135 | success-measures.md:1422 |
| SM ids referenced anywhere in `crates/**/*.rs` | 14 distinct (SM-121 16×, SM-126, SM-035, SM-013, SM-061 …) | `grep -rhoE 'SM-[0-9]{3}' crates --include=*.rs` (measured) |

**The register's own caveat, which this audit confirms:** the register says the count of criteria that can catch a regression on day one is "materially below" the headline (acceptance-criteria.md:56) and that every instrument was prospective when written (:52-54). It is a specification count, not a coverage count.

### 2.1 ACs in force for Phase 1 (derived by this audit; no register row states it)

No document states the Phase-1 count. I derived it as 526 minus the ids the register itself routes elsewhere:

| removed from the Phase-1 universe | n | basis |
| --- | ---: | --- |
| human-gated (identified: AC-039, 076, 114, 115, 155, 156, 171, 213, 240, 275, 276, 309, 311, 357, 358, 396, 435, 476, 482 = 19 positively identified; the 20th is unidentified — see unverified) | ~20 | acceptance-criteria.md:26-41; ids by heading/GWT text |
| conditional | 6 | :7779 |
| Phase-2 corpus-bound (AC-444, 451–454, 456–462, 467, 469) | 14 | acceptance-criteria.md:7144 (Phase note) |
| Phase-2 `rk-plugin-gen` (AC-501–506) | 6 | :7779, :8268 |
| Phase-3 GitHub/mdBook adapters (AC-507, 508, 510) | 3 | :7779 |
| Codex adapter AC-496 (Phase 4 per ADR-0167) | 1 | :7779 |
| **Phase-1 universe** | **≈475** (±5) | of which 20 are PROVISIONAL (built-not-claimed) |

(AC-464 is Phase-1: s6-store-and-git.md:474-480 claims it.)

## 3. Where evidence lives — what exists

**There is no AC-to-test matrix.** `grep -il traceab` finds no coverage tool; `runbook/tools/` holds only `check-adr-options.py`, `check-bash-syntax.py`, `check-checkpoints.py`, `check-refs.py`, `fake-claude.rs`, `nodispatch.sh`; `.github/workflows/ci.yml` names individual AC ids in comments/step names (e.g. :24, :110-114, :137) but no step reconciles the register. Evidence is carried by three informal mechanisms:

1. **Test functions named `acNNN_…`** — 343 such `fn`s covering **234 distinct AC ids** (`grep -rhoE 'fn ac[0-9]{3}' crates`). Examples: `ac012_late_duplicate_is_discarded` (crates/rk-kernel/tests/it/runtime.rs:1025), `ac285_class_drift_event` (tests/it/review.rs:426), `ac298_unset_weight_is_lenient` (:912), `ac302_review_tier_takes_no_model_value` (:1073), `ac218_is_error_is_platform_regardless_of_exit` (crates/rk-adapter-claude/tests/it/classify.rs:122), `ac333_ac_evidence_ledger` (rk-store, per fixtures README).
2. **Tests named for behaviour, with the AC id only in a doc comment or in the stage file's checkpoint** (e.g. `rigor_discouraged_refused` for AC-050 at crates/rk-kernel/tests/it/selection.rs:107; `decoys_are_not_selected` for AC-150 at crates/rk-store/tests/it/locator.rs:77-). Stage checkpoints in `runbook/phase-1/s*.md` name literal test names (e.g. s7b-ii-gates.md:448, :660; s7b-kernel.md:1191).
3. **Compile-fail (trybuild) cases** under `crates/rk-kernel/tests/ui/` (`ac007_…`, `ac010_…`, `ac020_…`, `ac025_…`, `ac028_…`, `ac050_rigor_is_not_arithmetic.rs`), registered by `every_kernel_compile_fail_case_is_registered_and_commits_a_stderr` (tests/it/ui.rs:196).

**The "AC evidence ledger" (AC-333/335/337/342/345) is built as a kernel mechanism, not populated for this programme.** Computation: `crates/rk-kernel/src/verify/ledger.rs` (closed status per spec AC from four inputs and no fifth; signature enforces AC-335 — no suite-verdict parameter); types `crates/rk-types/src/artifact/ledger.rs`; store side `crates/rk-store/src/artifacts/ledger.rs`; tests `crates/rk-kernel/tests/it/ledger.rs`, `…/verify.rs`; fixtures `crates/rk-kernel/tests/fixtures/ledger/` (7 data files: closed-status enum, file-path-only negative, 4-id spec, complete ledger, two-AC spec, basis-pairs golden, ac455 spec). `git ls-files | grep -i ledger` finds **no ledger data file recording this repository's own AC register** — only fixtures and the doctor-fixture ledgers (`RK-D-ADVISORY-LEDGER`, `RK-D-EXECUTED-LEDGER`, crates/rk-cli/tests/fixtures/doctor/). The per-AC mutation-flip evidence ADR-0104 demands is therefore a Phase-1 *capability* proven on fixtures; it is not evidence the 526 ACs are pinned.

## 4. Quantified evidence over the Phase-1 universe (≈475)

Measured by grep over the snapshot tree (ids derived from the register headings; the tool outputs were held in the shell only):

| evidence class | ids | % of ≈475 |
| --- | ---: | ---: |
| `fn acNNN_…` test exists (strongest by-name signal) | 231 | 49% |
| id cited somewhere in a `.rs` file | 388 | 82% (upper bound; includes doc-comment-only cites) |
| id cited in a file under `crates/*/tests/` or a `#[cfg(test)]` source file | 373 | 79% (still an upper bound — citation is not assertion) |
| id cited only in non-test source (docs/impl comments) | 15 | 3% |
| id never cited in any `.rs` | 87 | 18% |

The 87 never-cited Phase-1 ids (not all unevidenced — twins and doctor-row discharges exist) are: 056 073 081 082 083 084 085 086 089 090 091 092 093 094 095 096 099 100 106 107 109 112 113 116 117 130 134 135 136 140 141 142 145 146 148 153 154 169 170 177 182 185 188 190 192 194 195 197 199 222 228 230 231 235 237 241 251 265 266 268 273 274 277 291 292 312 317 323 330 350 351 354 356 361 367 379 384 387 392 393 394 398 399 408 440 479 521. By area (ids not cited in any test file): packs/catalog **25**, git **15**, artifacts **13**, events/cost **12**, human gate **9**, harness **8**, verify **7**, review **4**, selection **4**, CLI **2**, migrate **2**, kernel **0**. **The packs/catalog area (AC-080–118) is the weakest by id**; part is explained (AC-092–099, 110-doctor-half and 118 are discharged by `doctor` rows at Checkpoint 7e, s7b-kernel.md:1980-1983, :2041), part is not (see AC-084 below).

**Twins explain a few:** AC-188 = AC-006 (acceptance-criteria.md §0.3 table), AC-177/AC-407 → AC-246, AC-222/379/404 → AC-030, AC-235 → AC-031, AC-237 → AC-033/034/035: the owner is tested, the twin carries no id. **PROVISIONAL-on-P8** multi-step-resume ACs (AC-013, 187, 266, 267, 271, 401) are "built, not claimed" by design (s6-store-and-git.md:474-480).

**The record itself lists unowned/OWED ACs, and they overlap the uncited list:** AC-291/292 are *orphaned* — no stage claims them, "no checkpoint tests them" (s7b-ii-gates.md:978-990; s7c-adapters.md:3168-3169 "undecided; not claimed without the erratum"); AC-262/351, 189/196–198, 228/388, 357/358 machine halves, 361/367/387 each have "an owner record OWED" (s7c-adapters.md:3254-3257). Step 8 "keeps no acceptance set" (s7b-ii-gates.md:992-995) — an acceptance set for the exit stage is owed.

## 5. Random sample of 15 Phase-1 ACs (seeded draw)

Method: `random.seed(20261005)`, 15 ids from the ≈475 Phase-1 universe (ids computed excluding the §2.1 classes). For each I read the register's GIVEN/WHEN/THEN and *Verified by*, then located the asserting test and read its body. Grades: FULL = a test asserts every THEN clause; PARTIAL = some clauses asserted, some not; MISS = no asserting test found; OWED = blocked by a recorded, in-progress dependency.

| AC | criterion (short) | result | evidence (test fn / location) |
| --- | --- | --- | --- |
| AC-012 | a late duplicate attempt is discarded, committed value unchanged | **FULL** | `ac012_late_duplicate_is_discarded`, crates/rk-kernel/tests/it/runtime.rs:1025 (two threads, barrier, distinguishable results) |
| AC-048 | family totality: a hole at a rigor level fails load | **FULL** | `family_hole_at_mandatory_fails_totality`, crates/rk-kernel/tests/it/lattice.rs:925; ceiling half `rigor_stage_scope_and_ceiling`, tests/it/producer.rs:152 |
| AC-050 | `Rigor` not arithmetic; `discouraged` reserved | **FULL** | trybuild `tests/ui/ac050_rigor_is_not_arithmetic.rs`; `rigor_discouraged_refused`, tests/it/selection.rs:107 (names the site) |
| AC-059 | `requires` cycle (non-optional) fails load naming the path | **FULL** | `cycle_and_excludes_fail_load`, crates/rk-kernel/tests/it/dag.rs:353 (asserts the 4-node path) + `ac059_optional_only_requires_cycle_loads`, dag.rs:328 |
| AC-084 | two tech packs fill one slot: both fillers present, declaration order, byte-identical records | **MISS** | no test, no id citation anywhere in `crates/`; the only "fillers" surface is `explain.rs:545` printing `fillers: not recorded`. The multi-pack slot-filling binder behaviour appears unbuilt (design §4.3 rule 3, architecture-design.md:496) |
| AC-150 | locator selects the real heading, not five decoy forms | **FULL** | `decoys_are_not_selected`, crates/rk-store/tests/it/locator.rs:77-115 (fence, indent, HTML comment, blockquote, deeper-heading decoy) |
| AC-154 | `ChangeSummary` binary entries rendered from typed fields | **OWED** | PROVISIONAL on S-BIN; zero citations in `crates/`; plan says binding only if ADR accepted and both `render::ac154_*` tests pass (s8i-s-bin.md:2633). That is the step in progress |
| AC-158 | schema lint: required properties precede optional in emitted order | **FULL** | `optional_before_required_is_caught`, crates/rk-types/tests/it/schema_no_oneof.rs:72; `preserve_order_is_in_force`, tests/it/fixtures.rs:151; lint at crates/rk-types/src/schema/lint.rs:113 |
| AC-162 | tier-1: git commit before store row; failed commit leaves zero rows; hooks neutralised | **FULL** | `a_failed_commit_leaves_zero_store_rows` and `a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit` (with positive control), crates/rk-kernel/tests/it/tier_one.rs:293- |
| AC-218 | `is_error`/`api_error_status` resolves to `StepError::Platform` regardless of exit code | **FULL** | `ac218_is_error_is_platform_regardless_of_exit`, crates/rk-adapter-claude/tests/it/classify.rs:122 |
| AC-285 | computed class wins; model label retained; `review_class_drift` carries both; lint: policy never reads the label | **FULL** | `ac285_class_drift_event`, crates/rk-kernel/tests/it/review.rs:426; lint `policy_reads_no_model_label_rule`, crates/rk-kernel/src/module_boundary.rs:725 with positive control fixture |
| AC-292 | reviewer clone: depth 1, no alternates, no remote to parent, pre-change blob absent | **PARTIAL** | F17 recovery probe + one-commit shape asserted (crates/rk-git/tests/it/review_clone.rs:35-105: `the_prepared_clone_has_one_commit_and_no_other_ref`, `the_f17_recovery_procedure_fails_against_a_review_clone`, with control). No assertion of `objects/info/alternates` or remote-absence found. Also the id is recorded as ORPHANED (s7b-ii-gates.md:981) |
| AC-298 | unset review weight resolves `Lenient` | **FULL** | `ac298_unset_weight_is_lenient`, crates/rk-kernel/tests/it/review.rs:912 (golden over every rigor) |
| AC-302 | review tier derives only from authored rigor; no model value input | **FULL** | `ac302_review_tier_takes_no_model_value`, crates/rk-kernel/tests/it/review.rs:1073; call-graph lint crates/rk-kernel/src/module_boundary.rs:798 |
| AC-389 | raised appetite refused AND the attempt appended as an event | **PARTIAL** | refusal asserted: `appetite_raise_from_an_activity_result_is_refused`, crates/rk-kernel/tests/it/budget.rs:430. The *event* half is explicitly NOT BUILT: crates/rk-kernel/src/workflow/budget.rs:85-91 ("no `EventKind` for a refused appetite raise"), residual R-1, spikes/DECISIONS-2026-09-15.md:191; no appetite kind in rk-types events |

**Hit-rate:** 11 FULL, 2 PARTIAL, 1 MISS, 1 OWED of 15. Strict (FULL only): **11/15 = 73%**. Lenient (partials at 0.5): 12/15 = **80%**. 95% binomial interval on 11/15 is roughly 45%–92% (n=15 is small; read as "about three quarters, probably between a half and nine tenths").

**Cross-check against the grep tiers:** of the 12 sampled ids that are cited in a test file, 11 were FULL and 1 PARTIAL (citation precision ≈ 92–96%); the 3 not cited in a test file were MISS, OWED and PARTIAL. Applying ~92% precision to the 373 test-cited ids gives ≈343, plus a handful of ids covered with no id citation (AC-188/177-type twins, doctor-row discharges) ⇒ **≈350/475 ≈ 73%**, which agrees with the sample.

## 6. Findings that matter

1. **Evidence is real where it exists.** Of 11 FULL hits, the tests are discriminating: positive controls beside negatives (AC-162's hook control, AC-150's decoy set, AC-292's pre-fix-blob control), trybuild compile-fail for type-level claims, architecture lints with committed violating fixtures (`tests/fixtures/lint-controls/`). That is above the usual standard for a spec-driven build.
2. **The tracker overstates nothing but it also can't prove coverage.** PROGRESS.md shows Steps 5–8h with recorded passes/"re-run WHOLE and passed at d24d15a" (PROGRESS.md:19-60), but each checkpoint asserts named tests; no artifact asserts "every Phase-1 AC has a test". Two stage sets are known incomplete (Step 8 has no acceptance set; AC-291/292 orphaned; s7b-ii-gates.md:978-995).
3. **Pack/catalog slot-binding (AC-081–AC-086) looks unbuilt or unevidenced.** AC-084 MISS is representative: six consecutive ids (081–086) and AC-089–091 have no id citation, and `explain` reports fillers as "not recorded". Because the shipped pack is deliberately only two packs (`bug-standard`, `story`; PROGRESS.md 7e row), technology-pack slot filling may simply not be exercised in Phase 1 — but the register still counts these as binding Phase-1 criteria (s7e acceptance set "AC-080–AC-118", s7e-pack-authoring.md:228).
4. **Known partial/unbuilt halves are honestly recorded** (AC-389 event half R-1; AC-154 awaiting S-BIN; AC-356/497/475 same cluster per s8i-s-bin.md:2633,2659) — the records are candid; the risk is the register still reads them as "binding".
5. **SM register is measurement doctrine, not running instruments.** 14 of 135 SM ids are even referenced in code; 48 are by the register's own account non-discriminating; the success measures need a first production window (success-measures.md §5.1). The Phase-1 exit does not depend on them, but "are we succeeding" cannot yet be answered by the register.
6. **Registry hygiene:** AC-428 duplicated heading; the §0.1 deduction (3) says "materially below 469" while the table says 480 binding (stale number, acceptance-criteria.md:55 vs :26-41); the register records its own stale counts at §15.3 ("NOT recomputed" for six ADRs, :56).

## 7. Unverified / limits

- The exact twenty human-gated ids: the register carries the count (20) but no single id list; I positively identified 19 (listed in §2.1). The 20th is probably AC-139 or AC-133-adjacent; not confirmed.
- Phase-1 universe (~475) is derived, not stated by any record; a ±5 error is plausible (e.g. further S-FACET/S-CODEX-gated ids I did not individually classify).
- Test pass/fail was not run (no cargo). "Evidenced" = a test exists and asserts the clause when read; green status is inherited from PROGRESS.md.
- The 87 uncited ids were not each investigated; only sampled ones were traced. Some will be covered by tests that cite neither the id nor an `acNNN_` name (I confirmed this pattern for AC-050, AC-059, AC-150, AC-158, AC-162). The true uncovered count is therefore below 87 but cannot be stated.
- "Cited in a test file" is an upper bound on evidence (a comment can cite an id without asserting it); sample precision ≈92% is from n=12.
- I did not check `.github/workflows/ci.yml` end-to-end for a coverage gate beyond grepping AC mentions; I found none.
- Stage checkpoint (shell) assertions in `runbook/phase-1/*.md` were consulted for test names, not re-executed; ACs discharged *only* by shell checkpoints (e.g. Checkpoint 7e `doctor` rows for AC-092–099/118) were not individually traced.
- The AC-084 MISS is "no test or implementation found by id/behaviour search", not proof the behaviour is absent.
