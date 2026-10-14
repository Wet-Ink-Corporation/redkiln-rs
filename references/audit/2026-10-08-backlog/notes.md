# Ledger manifest notes (draft, 2026-10-08, HEAD 71fc926)

## Rationale
- This follows the approved plan (`please-do-a-full-clever-mist.md`). The runbook owns s8i, s8j and Step 8. GitHub gets a pointer Initiative for M5 plus the audit findings, decisions nobody owns yet, post-M5 hardening and Phase 2-4 placeholders.
- Epic splits apply only to the plan's gap-2 list (#8 #9 #10 #13 #14 #15 #25 #27 #31), and only where the verifiers found more than one PR-sized part. The original keeps its number as the Epic. In other issues, extra parts were folded or moved, and no epic was made: #20's records note went to #23, #26's disposition table went to #27, and #33's code half waits for the Phase 2 breakdown.
- **Priority mapping.** §14 High or Medium-High maps to p1, Medium to p2, Low-Medium to p3. §15 items 1-3 map to p1, items 4-6 to p2, item 7 to p3. Where both apply, the higher wins; with neither, prio is omitted. No p0 was assigned: that is left for a human.
- **status/ready** is set only where the acceptance criteria exist (for existing issues, in the reconciliation comment), there is no `needs/*` label and no blocked-by link. That gives 20 ready issues.
- **needs/human on record edits.** #23 and #29 carry `needs/human` because they edit `runbook/PROGRESS.md`, which the live s8i session holds uncommitted edits to (`git status` at review: `PROGRESS.md`, `s8i-s-bin.md`, `DECISIONS-2026-09-21.md`, `METHOD-S.md`; s8j is not modified). The #25 recovery child carries it because the operator must approve runbook text before the 8j freeze.
- **Namespaces.** The audit's O-1 to O-11 (appendix §13) are not PROGRESS.md's O-1…O-11 or O-12…O-16 rows (`PROGRESS.md:175`, `:322`). Every body says "audit O-n" to keep them apart.
- **Sources.** Docs-derived items (R-37, R-64, temper r1, the follow-ups epic) are labelled `source/agent` with the marker `src=docs`.
- Every ID used was grepped in the repo: AC ids in the register, ADR files, D21 headings, the R and RT rows in PROGRESS.

## Existing issues
| # | Audit | Action | Type | Parent | Blocked by | Labels (excluding source) |
| --- | --- | --- | --- | --- | --- | --- |
| #5 | A1 | keep | Bug | new:run-path-fixes | - | area/cli, pre-m5, prio/p1, status/ready |
| #6 | A2 | keep | Bug | new:run-path-fixes | - | area/cli, pre-m5, prio/p1, status/ready |
| #7 | A3 | keep | Bug | new:run-path-fixes | - | area/kernel, pre-m5, prio/p1, status/ready |
| #8 | A4 | convert-to-epic | Epic | new:post-m5-hardening | - | area/kernel, prio/p3 |
| #9 | A5 | convert-to-epic | Epic | new:post-m5-hardening | - | area/kernel |
| #10 | A6 | convert-to-epic | Epic | new:post-m5-hardening | - | area/kernel, area/pack |
| #11 | A7 | keep | Bug | new:run-path-fixes | - | area/cli, pre-m5, prio/p1, status/ready |
| #12 | A8 | keep | Task | new:post-m5-hardening | - | area/cli, status/ready |
| #13 | A9 | convert-to-epic | Epic | new:post-m5-hardening | - | area/kernel, area/cli |
| #14 | A10 | convert-to-epic | Epic | new:post-m5-hardening | - | area/kernel, area/cli |
| #15 | B1 | convert-to-epic | Epic | new:post-m5-hardening | - | area/git, area/kernel, area/store, area/types, prio/p3 |
| #16 | B2 | keep | Bug | new:post-m5-hardening | - | area/git, area/cli, prio/p3, status/ready |
| #17 | B3 | keep | Bug | new:post-m5-hardening | - | area/store, status/ready |
| #18 | B4 | keep | Bug | new:post-m5-hardening | - | area/git, status/ready |
| #19 | B5 | keep | Bug | new:post-m5-hardening | - | area/types, prio/p3, status/ready |
| #20 | B6 | narrow | Bug | new:m5-phase1-exit | - | area/containment, pre-m5, prio/p1, status/ready |
| #21 | B7 | keep | Bug | new:post-m5-hardening | - | area/adapter-claude, area/cli, needs/decision |
| #22 | B9 | keep | Bug | new:post-m5-hardening | - | area/cli, area/store, status/ready |
| #23 | C1 | narrow | Task | new:m5-phase1-exit | - | area/records, area/containment, pre-m5, prio/p1, needs/human |
| #24 | C2 | keep | Task | new:post-m5-hardening | #23 | area/records, needs/decision |
| #25 | C3 | convert-to-epic | Epic | new:m5-phase1-exit | - | area/records, pre-m5, prio/p1 |
| #26 | C4 | narrow | Task | new:m5-phase1-exit | - | area/records, pre-m5, prio/p2, needs/decision |
| #27 | C5 | convert-to-epic | Epic | new:post-m5-hardening | - | area/ci, area/records, prio/p2 |
| #28 | C6 | keep | Task | new:m5-phase1-exit | - | area/records, pre-m5, prio/p2, needs/decision |
| #29 | C7 | retitle | Task | new:m5-phase1-exit | - | area/records, needs/human |
| #30 | C9 | keep | Task | new:m5-phase1-exit | - | area/records, pre-m5, prio/p1, needs/decision |
| #31 | C10 | convert-to-epic | Epic | new:post-m5-hardening | new:m5-phase1-exit | area/cli, area/git, area/kernel, area/records, prio/p2 |
| #32 | C11 | narrow | Task | new:m5-phase1-exit | - | area/records, pre-m5, prio/p2, needs/human |
| #33 | C12 | keep | Task | new:m5-phase1-exit | - | area/records, area/cli, pre-m5, prio/p1, needs/decision |

## New issues
| Key | Type | Title | Parent | Blocked by | Labels |
| --- | --- | --- | --- | --- | --- |
| `m5-phase1-exit` | Initiative | M5 — Phase-1 exit (pointer: the runbook owns s8i, s8j and Step 8) | - | - | source/human, area/records |
| `post-m5-hardening` | Initiative | Post-M5 hardening | - | - | source/audit-2026-10-06, area/records |
| `phase-2` | Initiative | Phase 2 — packs and parallelism (placeholder) | - | new:m5-phase1-exit | source/human, area/records |
| `phase-3` | Initiative | Phase 3 — surfaces (placeholder) | - | new:phase-2 | source/human, area/records |
| `phase-4` | Initiative | Phase 4 — reach (placeholder) | - | new:phase-3 | source/human, area/records |
| `run-path-fixes` | Epic | Run-path defects to fix before the first paid lane run (#5 #6 #7 #11) | new:m5-phase1-exit | - | source/audit-2026-10-06, area/cli, area/kernel, pre-m5, prio/p1 |
| `followups-spikes` | Task | Disposition the unrun audit follow-ups and the registered-but-unrun spikes | new:post-m5-hardening | - | source/agent, area/records, needs/decision |
| `a4-decide` | Task | Decide: wire budget exhaustion and degradation now, or defer them to Phase 2 by ADR | #8 | - | source/audit-2026-10-06, area/kernel, area/records, prio/p3, needs/decision |
| `a4-ledger-gate` | Task | Fold pass charges into the BudgetLedger and gate dispatch on exhaustion (AC-025) | #8 | new:a4-decide | source/audit-2026-10-06, area/kernel, prio/p3 |
| `a4-wallclock` | Task | Write the durable wall-clock expiry mark when a run's budget expires (AC-024) | #8 | new:a4-decide | source/audit-2026-10-06, area/kernel, prio/p3 |
| `a4-degrade` | Task | Route the degradation policy's model substitution through harness dispatch (AC-363) | #8 | new:a4-decide | source/audit-2026-10-06, area/kernel, prio/p3 |
| `a5-usage-fail-closed` | Bug | Fail closed on a malformed per-item usage charge | #9 | - | source/audit-2026-10-06, area/kernel, status/ready |
| `a5-atomic-admission` | Bug | Make per-item dispatch admission atomic across journal read, admit and append | #9 | - | source/audit-2026-10-06, area/kernel, status/ready |
| `a6-strict-header` | Bug | Load catalog headers strictly: typed errors, a closed key set, no silent min_rigor default | #10 | - | source/audit-2026-10-06, area/kernel, area/pack, status/ready |
| `a6-digest-fields` | Task | Add excludes and deviation_ok to the catalog digest (moves goldens and run ids) | #10 | - | source/audit-2026-10-06, area/kernel, area/pack, needs/decision |
| `a9-bind-claims` | Feature | Bind verify claims at plan time and refuse an equal-specificity tie by name (AC-349) | #13 | - | source/audit-2026-10-06, area/kernel, area/cli, status/ready |
| `a9-retire` | Task | Retire advisories not re-raised after each review pass | #13 | - | source/audit-2026-10-06, area/kernel, area/cli, status/ready |
| `a9-quarantine` | Task | Decide: a quarantine grant/remove verb, or an ADR deferring it to Phase 2 | #13 | - | source/audit-2026-10-06, area/kernel, area/cli, area/records, needs/decision |
| `a10-differential` | Task | Add a differential test: lattice and rigor cells against select over .redkiln/ | #14 | - | source/audit-2026-10-06, area/kernel, status/ready |
| `a10-routing` | Task | Call pack_capability_routing_rule from the run composition root | #14 | - | source/audit-2026-10-06, area/kernel, area/cli, status/ready |
| `a10-step7` | Task | Decide whether mid-walk deviation (selection step 7) is Phase 2 | #14 | - | source/audit-2026-10-06, area/kernel, area/records, needs/decision |
| `b1-decide` | Task | Decide: schedule the orphan/rewrite detector, or record a dated deferral | #15 | - | source/audit-2026-10-06, area/records, area/git, prio/p3, needs/decision |
| `b1-orphan-report` | Feature | reconcile() reports unreachable anchors as Drift::OrphanedAnchor without deleting rows (AC-167) | #15 | new:b1-decide | source/audit-2026-10-06, area/git, area/kernel, area/store, prio/p3 |
| `b1-rewrite-event` | Feature | Emit rewrite(old, new, mechanism) by patch-id before re-anchoring, with the AC-170 negative test | #15 | new:b1-decide, new:b1-orphan-report | source/audit-2026-10-06, area/git, area/kernel, area/types, prio/p3 |
| `b1-human-gate` | Feature | Route an unresolvable orphan that blocks an item to a human gate (AC-171) | #15 | new:b1-decide, new:b1-rewrite-event | source/audit-2026-10-06, area/kernel, prio/p3 |
| `c3-recovery-procedure` | Task | Write the failed-first-real-run recovery and re-freeze procedure (audit O-9, option a) | #25 | - | source/audit-2026-10-06, area/records, pre-m5, prio/p1, needs/human |
| `c3-pre-freeze-dispatch` | Task | Decide: one real redkiln run dispatch on the fixture item before the 8j freeze (audit option c) | #25 | - | source/audit-2026-10-06, area/records, pre-m5, prio/p1, needs/decision |
| `c5-reconcile-report` | Task | Add a register-wide AC-to-test reconciliation script and CI report | #27 | - | source/audit-2026-10-06, area/ci, area/records, prio/p2, status/ready |
| `c5-triage-gaps` | Task | Triage the sampled silent AC gaps and the orphaned AC-291 and AC-292 | #27 | - | source/audit-2026-10-06, area/records, area/git, area/kernel, area/store, prio/p2, needs/decision |
| `c5-register-amendments` | Task | Land or schedule the owed AC-043, AC-044, AC-045 and AC-056 register amendments | #27 | - | source/audit-2026-10-06, area/records, prio/p2, needs/decision |
| `c4-disposition-table` | Task | Build the disposition table for the unscheduled binding ACs and unspiked provisional ACs | #27 | #26, new:c5-reconcile-report | source/audit-2026-10-06, area/records, prio/p2 |
| `c10-proposed-sweep` | Task | Sweep stale 'proposed' ADR labels in source comments against ADR status | #31 | new:m5-phase1-exit | source/audit-2026-10-06, area/records, area/kernel, area/cli, prio/p2 |
| `c10-headers` | Task | Fix crate and module headers that contradict the code | #31 | new:m5-phase1-exit | source/audit-2026-10-06, area/cli, area/kernel, prio/p2 |
| `c10-provenance` | Task | Move dated provenance comments out of source into the ledgers | #31 | new:m5-phase1-exit | source/audit-2026-10-06, area/records, area/cli, area/kernel, prio/p2 |
| `c10-split-run` | Task | Split crates/rk-cli/src/verbs/run.rs behind the existing tests | #31 | new:m5-phase1-exit | source/audit-2026-10-06, area/cli, prio/p2 |
| `c10-split-worktree` | Task | Split crates/rk-git/src/worktree.rs behind the existing tests | #31 | new:m5-phase1-exit | source/audit-2026-10-06, area/git, prio/p2 |
| `c10-stage-cap` | Task | Decide a stage-file size cap for Phase 2 | #31 | new:m5-phase1-exit | source/audit-2026-10-06, area/records, prio/p2, needs/decision |
| `o4-record-a` | Task | Sign ADR-0197 (Record A) decision 7's table (audit O-4) | new:m5-phase1-exit | - | source/audit-2026-10-06, area/records, pre-m5, prio/p1, needs/decision |
| `o7-fold-fixes` | Task | Decide whether the run-path fixes join s8j's chain before the first paid launch (audit O-7) | new:run-path-fixes | - | source/audit-2026-10-06, area/records, pre-m5, prio/p1, needs/decision |
| `r37-spec-supplier` | Task | Settle R-37: the spec supplier seam cannot make a node dispatchable on its own | new:post-m5-hardening | - | source/agent, area/kernel, area/records, needs/decision |
| `r64-tag-rebuild` | Task | Measure R-64: can doctor --rebuild re-derive every event-store tag from git alone? | new:post-m5-hardening | - | source/agent, area/cli, area/store, area/adapter-sqlite, status/ready |
| `temper-r1` | Task | Make the temper gate green on the merged Rust (operator ruling r1, 2026-10-08) | new:post-m5-hardening | - | source/agent, area/ci, needs/repro |

## Labels
- 41 entries: 14 `area/*`, 2 `status/*`, 4 `needs/*`, 4 `source/*` (`source/audit-2026-10-06` is renamed from `audit-2026-10-06`), 4 `prio/*`, 3 `size/*` and `pre-m5`.
- All 9 GitHub defaults are marked `deleteAfterMigration`. GitHub's close reasons (completed, not planned, duplicate) cover duplicate, wontfix and invalid.
- There are no `rk:` labels; that namespace belongs to AC-507.
- No `size/*` label was assigned: no source gives sizes, so a human should set them.

## Deliberately skipped
- **D1** (human-root arm) is owned by s8j §8j.2 (`s8j-run-preparation.md:539`). The audit flags its conditional wording, but it is scheduled, so it is not duplicated.
- **D6** (adapter pin 2.1.278 to 2.1.289, probes) is owned by s8j §8j.6 (`:931`).
- **D9** (`hold_before`) is owned by s8j §8j.3 (`:658`).
- **Audit O-8** (pick the fix item) is owned by s8j §8j.5 (`:849`).
- **L3** is already repaired at s8e §8e.1 (`crates/rk-store/src/conn.rs:137-141`). It survives only as a stale tracker row in #29.
- **Audit O-2** (merge and the temper baseline): the merge is done (`88a8dbd`), and the baseline decision is in AGENTS.md's decisions log. Recording it is part of #23.
- **Audit O-11** (Codex placement) is moot: ADR-0167 (audit README; C8 not filed).
- **B8 and C8**: not filed by the audit, because a record owns each (RT-24; ADR-0167).
- **F35, F36 and F37** (F-drafts): no owed action is recorded. F35's verdict clears at `spikes/F35-DRAFT.md:59`. They are named as out of scope in the follow-ups epic.
- **The CORPUS_PIN-on-remote checkpoint line** (from #32) and **the checkpoint re-runs on the merged tree** (from #23) belong to Checkpoint 8i, 8j and s8j's chain, which the runbook owns.
- **The `ci.yml` ARMED-comment corrections and s8j §8j.7's premise** (from #23, audit C1 item 3) are s8j §8j.7's: *"The ARMED comments in `ci.yml` get dated, comment-only corrections once their job has run green"* (`s8j-run-preparation.md:1222`).
- **The D21-143 tracker row** (from #29, audit C7) is s8i's live go-ahead row and stays with the runbook.
- **#33's wiring PR** is not filed yet; it becomes a Phase 2 Feature when that phase is broken down.
- **The 50 unscheduled binding ACs and 14 provisional ACs** get no per-id issues. They are carried by `c4-disposition-table` under #27, which is the AC-backlog epic.
- **Gap 5** has no separate new issue: #29 already was that task, so it is retitled and widened (D17-3, RT-27 and R-40…R-62 added) instead of duplicated.

## Judgement calls for the reviewer
- #24 (landing step) sits under Post-M5 hardening (§16 lists landing there) and is blocked by #23. If the landing step must be written before M5, move it under M5.
- #29 sits under the M5 initiative per the plan's gap 5, but without `pre-m5` (the verifier: not an M5 gate).
- `c4-disposition-table` and #27 are post-M5. If #26 decides option (a), both move under M5 and become `pre-m5`.
- The Post-M5 hardening initiative is not blocked by M5; only #31 and its children are. The hardening bugs are independent and can land whenever a human schedules them.
- `temper-r1` cites the D21153-8i-E rulings paragraph of `spikes/DECISIONS-2026-09-21.md` by heading, not line: it is uncommitted at `71fc926` and had already moved from :7002 to :7017 in the working tree by review time. The GitHub temper and ci runs are green at `71fc926`, so the issue is `needs/repro`.

## Adversarial review (2026-10-08, fresh context)
Fixes applied to `ledger-manifest.json` (the pre-review copy is `ledger-manifest.pre-review.json`; `gen_manifest.py` was NOT updated and would regenerate the old version):
- #23: dropped the `ci.yml:216/:305/:580` corrections and the s8j §8j.7 premise note (s8j owns them, `s8j-run-preparation.md:1222`); dropped `area/ci`; AC now limits the diff to PROGRESS.md, rk-containment's lib.rs and ERRATA.md.
- #29: dropped D21-143 (s8i state); O-12…O-16 evidence corrected to `DECISIONS-2026-09-18.md:507-511` (answered 2026-09-19), not "the 7d-open flip (ADR-0128/0129)"; the optional second PR moved to out of scope (one PR per Task).
- `followups-spikes`: an Epic with zero children became a Task; `needs/triage` → `needs/decision` (its AC is an operator disposition).
- `a9-quarantine`: ADR-0162 (the mutate-repro refusal) replaced by ADR-0074 decision 6, AC-307 and ADR-0124, which govern quarantine.
- `c10-provenance`: "one crate per PR" made it multi-PR; scoped to one crate (`rk-cli`) with further crates filed separately.
- `temper-r1`: the uncommitted :7002 citation (already moved to :7017) replaced by the paragraph heading.
- Line fixes: #20 `ffi.rs:648-662` → `:650-664`; #33 `verify_pass.rs:574` is a doc line, not a setter; #19 wording; M5 initiative milestones `:94-100` → `:95-100`; `a10-routing` gains AC-495; `c5-register-amendments` gains the audit's `s7b-kernel.md:2035-2039`; hardening initiative says #31 itself is also blocked on M5.
