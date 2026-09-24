# Step 8 decision packet — 2026-09-24

*Nine grounded agents drafted these packets and nine adversarial refuters corrected them against the records at HEAD `3db4f5a` (workflow `wf_49591b78-75f`); they were then put to the operator. Under each heading, the TAKEN line is the operator's decision (D21-41). Every option below is as the refuter returned it, and each **Why** opens with the refuter's defect list. Citations are by quotation. Where an option says `0164`, that is a placeholder: each decision takes the next free ADR id when it is written.*

## (a) Where does an item's current stage live across processes, so that a second `redkiln run` continues the lane instead of reporting Done having done nothing (ADR-0157 decision 4, OWED)?

**TAKEN 2026-09-24:** A — stage-bearing `Story:` trailer on rk-run/<item>, folded by reconcile.

**Settled by record already:** False

**Binding records found:**

- ADR-0157 (accepted; superseded in part by 0161 at clause 1 only). Decision 4: "The cross-process residual is RECORDED, not closed." Decision 5: "No document may describe a lane as resumable across processes". Consequences: "The route out is a decision about where an item's current stage lives — the record, the journal, or a reconciliation between them — and it is named here as OWED." Its What-this-forbids list includes "Writing an item's advanced stage back to `.redkiln/items/` (AC-518)" and "Describing a lane as resumable across processes". Under ADR-0092 decision 1 a change to a Decision sentence OR to a forbids bullet is a supersession.
- ADR-0161 decision 1: "Every other clause of ADR-0157 stands, including its named cross-process residual". The guard is `!state.stages_entered.iter().any(|s| s == to)`. Its considered options reject "Compute the successor from the fold rather than the record" a second time. It also records that no test calls the lane loop: "`grep -rln "verbs::run::run(" crates/*/tests` returns nothing".
- ADR-0092 decision 1: an erratum "leaves every sentence of `## Decision` and every bullet of `**What this forbids**` standing as written. A correction that would move either is a supersession and takes a new record."
- ADR-0109 (accepted, never superseded). Decision 1: decision-driving state only "as the output of `reconcile()`". Decision 2: Tier 1 "is written to git first — a trailer on a checkpoint commit or an empty commit carrying the trailer". Decision 3: "Resume from `Story:` alone is forbidden". Decision 4: "`doctor --rebuild` reproduces the stage-level next action from git alone". It forbids "a tier-1 transition recorded only in the store" and "resuming from a single trailer axis".
- ADR-0160 decision 1: `s7h` "does **not** deliver the `Slice-Verdict:` axis", which has 0 producers. So any stage resume from trailers today reads `Story:` plus, at most, `Gate:`.
- Design §5.4: "If yes, the state may not live only in the database." The event-kind row gives `stage_transition` as "1 (trailer)". The grammar row gives `Story:` as "`<project> "/" <slug>` — one item id", and it says "a second identical `Story:` for one slug is drift `duplicate-story-trailer`, counted, not a transition".
- AC-161 (narrowed by ADR-0097): after the EventStore file is deleted, "the **stage-level next action** … is equal, under the canonical serialiser, to the one selected before the deletion". AC-164 says the same after tier 2 is truncated.
- AC-518: a write "is refused unless its target is a run-owned worktree or the store under the resolved common dir". The criterion forbids writing the checkout. It does not forbid writing on rk-run/<item>. ADR-0157's forbids bullet is broader and names `.redkiln/items/` without that qualifier.
- ADR-0155 (accepted). Decision 1: "exactly **one** `Story:` trailer per item". Decision 2 fixes the value as `<project>/<item-id>`. Decision 5(a) records the N−1 residual; that text is a Decision clause. Forbids bullets: "A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes" and "A `Story:` value carrying a stage, a boundary, or any third token, until the grammar is widened by a record that supersedes `0105` clause 3". Option 3, "Widen the grammar to a three-token value", is "the honest long-term answer", rejected "**for this stage**, not on merit". Decision 6 amended s7h by erratum to read "*the first* stage boundary".
- ADR-0158 (accepted). Decision 4 routes "a tier-1 write method on `EventStore` … called from the same place the trailer commit is made". Its forbids list includes "Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis" and "Treating the `t2_event` `stage_transition` row as a tier-1 durability record".
- ADR-0105 clause 3 (accepted; 0105 is already superseded in part by 0116, at clause 7 only). Its own words fix the keys ("Keys are `Story:`, `Slice-Verdict:`, `Gate:`") and delegate the values: "values follow the trailer grammar table in design §5.4 … AC-444's drift counts are pinned to that table". Clause 6: a worktree is "re-created, never reused, per run".
- AC-444 is corpus-bound and belongs to the "Phase-2 acceptance set" (AC register phase note, amended by E13), so re-pinning its counts is not a Phase-1 gate.
- ADR-0162: "`resolved.stage` is compared to no string literal anywhere in `crates/rk-cli/src/`". Any new stage source inherits this.
- Ledger D21-33 (spikes/DECISIONS-2026-09-21.md): "across processes the lane is idempotent and **stalled**".
- runbook/phase-1/s8-exit-criterion.md (1): the PASS line needs "EVERY lane stage from intake to a landed commit", read from `git log "rk-run/$FIX_ITEM" … --grep='^(Story|Slice-Verdict|Gate):'`, and the comment beside it assumes "the trailer values are the pack's stage names". Under ADR-0155 a Story value carries no stage name, so the check as written cannot see stages 2..N.

**Measured state:**

I read this at 4c282d7 without running cargo or redkiln; the only commands were read-only git and grep.

**Where the stage comes from.** In crates/rk-cli/src/verbs/run.rs, `resolved.stage` comes from the checkout's item record, and the refusal there reads "the stage is read once, from the record, and is not defaulted". `base_inputs` gets the successor from `pack.successor(&resolved.stage)`. Step (9) calls `fsm::fold(&item_id, &resolved.stage, &prior)`, so the store journal does carry the advanced stage into the next process: the fold's `from` moves. What never moves is the successor, which is computed from the record. The `stages_entered` guard therefore finds nothing to do, and the lane stalls. That is ADR-0161's account, and I read it from the code rather than re-measuring it.

**`answer` shares the seam.** `answer` resolves through `run::through_step_seven` and folds with `&resolved.stage` in verbs/answer.rs, so it uses the same record seed. `answer` does not advance a lane. After a gate, only a second `run` can continue, and that second run meets the stall.

**Order of writes at a boundary.** `story_trailer` is called after `rk_kernel::replay::resume(...)` returns and after `*advanced` is set. The t2 `stage_transition` row is appended before the `Story:` commit exists, so at HEAD the order is store first, git second.

**reconcile.** crates/rk-git/src/reconcile.rs `read_commits` runs `log --all --reverse`, and `reconcile` folds oldest-first. `Slice-Verdict:` already uses "Latest wins". `Story:` keeps only `stories_committed: BTreeSet<String>`, a second identical value is `Drift::DuplicateStoryTrailer`, and no stage is kept per item.

**The run branch persists.** `recreate_run_worktree` removes and re-adds the worktree each run, but it re-attaches the existing `rk-run/<item>` branch (it tries `-b` first and falls back to `rk_git::worktree::add`). Trailers on that ref therefore survive to the next invocation, and `--all` reads them.

**Existing trailers.** Exactly one `rk-run/*` ref exists (`rk-run/rk-fix-doctor-test-count`). Exactly one real `Story:` trailer exists in the whole repository: `redkiln-rs/rk-fix-doctor-test-count`, with no stage token. Any widened grammar has to say what that legacy value means.

**doctor --rebuild.** crates/rk-cli/src/doctor/rebuild.rs takes `to` from "`.redkiln/items/<slug>.toml`'s `stage` key in the checkout", keeps a `last_mark` per `(project, slug)`, and writes one `stage_transition` per item. Rebuild therefore reports the record's stage, not the lane's.

**rk-git is not frozen by any record.** crates/rk-git has not been touched since Step 6 (`git log --all -- crates/rk-git` returns 3 commits, the last being 7aa02c7). I found no record or runbook text that forbids editing it. The packet's claim that "earlier stage floors froze" rk-git is unsupported. The "floor" hits in the runbook are test-count floors and `GIT_FLOOR`.

**Consequences for s8.**
- Criterion (1) (every stage from intake to landed) and (1b) both need more than one invocation whenever a gate ends a process. The s8 comment says "A run that STOPS AT A GATE exits 0".
- (3)'s operational arm re-invokes after a kill.
- All three meet the stall past the first boundary.
- (1)'s trailer read cannot list stages 2..N under ADR-0155's one-mark-per-item grammar.

**What the records settle, and what they do not.** They settle the substrate: tier 1, git first, read through reconcile(). They do not settle the carrier, and every carrier needs at least one supersession. Nothing already decides this.

**Options:**

### A. A stage-bearing trailer per boundary on rk-run/<item>, folded by reconcile (ADR-0155 option 3)

Each boundary mints a trailer commit on rk-run/<item>, for example `Story: <project>/<item> <stage-entered>`: one per boundary, latest in fold order wins, the same fold rule `Slice-Verdict:` already uses. `reconcile` gains a per-item stage. `through_step_seven` seeds `resolved.stage` from the reconciled stage, and uses the record only when the item has no stage-bearing mark. Because the seed is shared, this covers both `run` and `answer`. The fold then starts from the right seed, and the `stages_entered` guard (ADR-0161) keeps replays idempotent. `doctor --rebuild` takes `to` from the marks and no longer from the checkout record.

The new ADR must state what the one existing stage-less value means. The obvious rule: it means "entered the lane", and the stage falls back to the record. The stage then becomes derivable from git alone, which is what AC-161/AC-164 test. ADR-0158 forbids CLAIMING AC-161 is met, so ADR-0164 must either lift that bullet for AC-161 in part or make no such claim.

The mark is minted at today's seam, after `resume` returns. Minting it before the t2 row would need a kernel seam change and is not part of this option; AC-162 stays vacuous either way (ADR-0158).

Sub-variant: a fourth key, `Stage:`. It keeps `Story:` at one per item, but it rewrites ADR-0105 clause 3's own words ("Keys are `Story:`, `Slice-Verdict:`, `Gate:`") and ADR-0109 decision 3 ("three axes"), so it is the larger supersession.

- *Record form:* New ADR-0164. It supersedes the following, each in part:
- ADR-0105 clause 3, as ADR-0155's forbids bullet demands. The clause's own words need not change, because it delegates values to the design §5.4 table. The table is what gets amended.
- ADR-0155 decisions 1, 2 and 5(a). 5(a) is a Decision clause, so the packet's "erratum to the ADR-0155 residual 5(a) text" is the wrong form under ADR-0092 decision 1.
- ADR-0155's forbids bullets "A second `Story:` trailer for one `(project, slug)`…" and "A `Story:` value carrying a stage…".
- ADR-0157 decisions 4–5 and its forbids bullet "Describing a lane as resumable across processes…".
- ADR-0158's forbids bullet "Claiming AC-160, AC-161…", for AC-161 only. This one applies only if ADR-0164 or s8 is to claim AC-161.
- ADR-0109 decision 3's "Resume from `Story:` alone is forbidden" and its forbids bullet on "a single trailer axis". The ADR must either show that the resume also reads `Gate:`, or supersede these in part. `Slice-Verdict:` cannot be the second axis, because ADR-0160 records it has 0 producers.

ADR-0164 executes ADR-0109 decisions 1, 2 and 4 and does not change them. It also cures a live violation of 0109's forbids bullet "a tier-1 transition recorded only in the store" for stages 2..N.

Attendant edits:
- a dated amendment to the design §5.4 `Story:` grammar row and its duplicate rule
- a re-pin of AC-444's drift counts (Phase-2 set, local-only)
- a re-amendment of s7h's ADR-0155 erratum ("*the first* stage boundary")
- a ledger entry

ADR-0158's t1-row route stays separate and complementary: the t1 writer would take its `anchor_sha` from this same commit.
- *Work:* rk-git:
- grammar.rs: `classify`'s Story arm parses an optional stage token. `split_qualified` stays first-slash, and ADR-0161 already refuses `/` in a project name.
- reconcile.rs: `stories_committed` gains a per-item latest stage. A repeated identical `(project, slug, stage)` stays drift. Say explicitly that ADR-0161's `stages_entered` guard already forbids re-entry within a lane.

rk-cli:
- `through_step_seven` seeds the stage from `Reconciled<State>`, with the record as fallback. This reaches `answer` too.
- `story_trailer` mints at every boundary instead of only the first. Its idempotence read changes from "item already committed" to "this (item, stage) already committed".
- doctor/rebuild.rs writes one row per stage mark.

Tests:
- the first automated two-process lane test driving `verbs::run::run(` twice. ADR-0161 records that none exists.
- a run → gate → answer → run test
- a delete-database test after hop 2
- a legacy stage-less mark fixture
- a mutant that seeds from the record instead of the reconciled stage, which must go red

The pack is unchanged.
- *Size:* M–L. About 400–700 LOC across rk-git and rk-cli plus tests, and 1 ADR with about 4 attendant record edits.
- *Risks:* - It edits rk-git for the first time since Step 6. No record forbids that, but s8 should name it as in scope.
- Re-pinning AC-444 moves an accepted count set. The risk is lower than it sounds: AC-444 is Phase-2 and corpus-bound, and whether its counts actually move depends on the F12 corpus's `Story:` values, which is unmeasured.
- The legacy stage-less mark on rk-run/rk-fix-doctor-test-count must fold deterministically.
- The fold relies on `git log --all --reverse` order without `--topo-order`. That is sound for one linear run ref, but the ADR should state the assumption.
- The claim that the resume reads more than one axis rests on `Gate:`, not on `Slice-Verdict:` (0 producers). If the resume does not actually read `Gate:`, 0109 decision 3 must be superseded in part, not argued around.
- The mark still comes after the t2 row at today's seam. Nothing may describe it as git-first ordering, and AC-162 stays vacuous (ADR-0158).
- No document may claim AC-161 met while ADR-0158's forbids bullet stands.
- *Citations:* references/adr/0155-one-story-trailer-per-item-per-lane-and-the-ac162-residual-is-named.md: "Widen the grammar to a three-token value"; "the honest long-term answer"; forbids "A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes"; references/adr/0109-reconcile-at-every-entry-two-tiers-and-three-trailer-axes.md: "written to git first"; "only as the output of `reconcile()`"; "Resume from `Story:` alone is forbidden"; references/adr/0160-the-slice-verdict-axis-needs-a-slice-identity-nobody-declares.md: "`s7h` does **not** deliver the `Slice-Verdict:` axis"; references/adr/0158-the-tier-1-store-layer-has-no-writer-on-any-axis.md: "Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis"; references/adr/0092-errata-precedence-and-cut-id-reissue.md: "every bullet of `**What this forbids**` standing as written"; references/2026-09-08-architecture-design.md §5.4: "If yes, the state may not live only in the database."; references/adr/0157-a-stage-does-not-advance-into-itself-and-a-lane-completes-within-one-process.md: "named here as OWED"; crates/rk-git/src/reconcile.rs: `read_commits` "log", "--all", "--reverse"; "Latest wins: commits are folded oldest-first."; crates/rk-cli/src/doctor/rebuild.rs: "`.redkiln/items/<slug>.toml`'s `stage` key in the checkout"; crates/rk-cli/src/verbs/run.rs: `recreate_run_worktree` "`worktree add -b <branch> <path> HEAD` on a FIRST run; `rk_git::worktree::add` when the branch already exists"; runbook/phase-1/s8-exit-criterion.md: "the trailer values are the pack's stage names"

### B. A run-owned item record: commit the advanced stage into `.redkiln/items/<item>.toml` on rk-run/<item>

At each boundary the run writes the new `stage` into the item record inside the run-owned worktree (`<common-dir>/redkiln/worktrees/<item>/`) and commits it on rk-run/<item>. AC-518 permits this, because the target is a run-owned worktree. ADR-0157's forbids bullet does not permit it: "Writing an item's advanced stage back to `.redkiln/items/`" carries no worktree qualifier. When rk-run/<item> exists, the next invocation reads the record from the ref's tip, not from the checkout. The stage lives in git as file content, not as a trailer.

- *Record form:* New ADR-0164. It supersedes the following, each in part:
- ADR-0109 decision 2: the stage would be carried by a file commit, not "a trailer on a checkpoint commit or an empty commit carrying the trailer".
- ADR-0109 decision 1, unless reconcile is taught to read tree content.
- ADR-0157 decisions 4–5, AND its forbids bullets "Writing an item's advanced stage back to `.redkiln/items/` (AC-518)" and "Describing a lane as resumable across processes".

It also amends the design §5.4 tier-1 substrate cell, and records the ref-tip item record as authoritative over the checkout's while the ref exists.
- *Work:* rk-cli:
- `through_step_seven` resolves the record from `rk-run/<item>:.redkiln/items/<item>.toml` through the wrapper (a read, so outside ADR-0105 clause 5's write list), with the checkout as fallback.
- At each boundary, commit the record edit in the run-owned worktree. The `Story:` trailer rides on the first such commit.
- doctor/rebuild.rs reads the stage from the ref-tip record.

Tests: two-process lane, run → gate → answer → run, delete-database, and one confirming the checkout is unchanged (AC-518). The pack is unchanged. rk-git is untouched apart from perhaps a read op.
- *Size:* M. About 250–450 LOC and 1 ADR, including a supersession of the tier-substrate rule.
- *Risks:* - It creates two sources of an item's stage, the checkout record and the ref-tip record. That is the divergence ADR-0157 diagnosed, moved to a different place.
- It leaves the trailer substrate that the design, AC-162/AC-163 and ADR-0158's anchor route all assume.
- A user who edits the checkout record mid-lane is silently overridden, or hits a conflict at merge.
- Pruning rk-run/<item> sends the lane back to the record's stage with no drift reported.
- It needs three supersessions in two records (0109 decisions 1 and 2, 0157 decisions and forbids) where A's are narrower in kind.
- *Citations:* references/2026-09-08-acceptance-criteria.md AC-518: "unless its target is a run-owned worktree or the store under the resolved common dir"; references/adr/0157-a-stage-does-not-advance-into-itself-and-a-lane-completes-within-one-process.md: "Writing an item's advanced stage back to `.redkiln/items/` (AC-518)"; references/adr/0109-reconcile-at-every-entry-two-tiers-and-three-trailer-axes.md: "a trailer on a checkpoint commit or an empty commit carrying the trailer"; references/2026-09-08-architecture-design.md: "`<common-dir>/redkiln/worktrees/<item>/` on `rk-run/<item>`"; crates/rk-cli/src/verbs/run.rs: "the stage is read once, from the" / "record, and is not defaulted"

### C. Derive the current stage by folding the journal's own `stage_transition` rows, as a named interim

The shared resolution folds the prior store journal before building `RunInputs` and takes the effective stage from the last transition, falling back to the record stage when there is none. It computes the successor from that. This re-adopts an option rejected twice: as ADR-0157 option 2 and again in ADR-0161's considered options. The part of the rejection about the arm still advancing no longer applies, because the `stages_entered` guard now exists. It is the cheapest way to unblock multi-invocation lanes for s8. The stage then lives only in `t2_event`.

- *Record form:* New ADR-0164. It supersedes the following, each in part:
- ADR-0109 decision 1: a decision-driving stage read from the store journal, not from `reconcile()` output. The packet omitted this one.
- ADR-0109 decision 2, and its forbids bullet "a tier-1 transition recorded only in the store".
- ADR-0158's forbids bullet on treating the `t2_event` row as tier-1.
- ADR-0157 decisions 4–5 and the corresponding forbids bullet.

It also takes on an AC-161/AC-164 non-conformance. Following ADR-0163's precedent of a named interim that supersedes in part, it would be time-boxed, with the retirement condition "Option A lands".
- *Work:* rk-cli run.rs/`through_step_seven`: a pre-fold (read the store journal, run `fsm::fold`, then use `state.stage`). Tests: a two-process lane test, plus a test pinning that deleting the database DOES change the next action, so the non-conformance is recorded rather than hidden. No change to rk-git, rebuild or the grammar.
- *Size:* S. About 60–150 LOC and 1 ADR, whose supersession is large relative to the code.
- *Risks:* - It fails the design's deleted-database test by construction. After the store is deleted, `run` falls back to the record stage and walks the lane again, so AC-161 and AC-164 are violated outright, and s8 criterion (2)'s `stage_level_next_action` claim becomes false for any multi-hop item.
- `doctor --rebuild` still reads the checkout record, so it disagrees with the store after hop 1.
- s8's (1) trailer read still cannot see stages 2..N.
- The interim could ossify: s8's criteria (1)/(1b)/(3) would then pass on a substrate the design forbids.
- *Citations:* references/adr/0157-a-stage-does-not-advance-into-itself-and-a-lane-completes-within-one-process.md: "Compute the successor from the fold rather than from the record"; references/adr/0161-a-stage-the-lane-entered-is-not-entered-again-and-a-failed-stage-earns-no-successor.md: "*Compute the successor from the fold rather than the record* — inverts the order"; references/adr/0158-the-tier-1-store-layer-has-no-writer-on-any-axis.md: "Treating the `t2_event` `stage_transition` row as a tier-1 durability record"; references/adr/0109-reconcile-at-every-entry-two-tiers-and-three-trailer-axes.md: "Forbids a tier-1 transition recorded only in the store"; references/2026-09-08-acceptance-criteria.md AC-161: "GIVEN a repository whose EventStore database file has been deleted"

**Recommended:** A. Record it as ADR-0164: a stage-bearing `Story:` trailer on rk-run/<item> at every boundary, folded by reconcile, seeding the stage through the shared `through_step_seven` path. ADR-0164 supersedes, each in part:
- ADR-0105 clause 3
- ADR-0155 decisions 1, 2 and 5(a), plus its two forbids bullets on a second or stage-bearing `Story:`
- ADR-0157 decisions 4–5, plus its forbids bullet on cross-process resumability
- ADR-0109 decision 3's single-axis rule, unless the resume demonstrably reads `Gate:`
- ADR-0158's forbids bullet for AC-161 only, and only if AC-161 is to be claimed

It carries the design §5.4 grammar amendment, the AC-444 re-pin (Phase-2 set), the re-amendment of s7h's first-boundary erratum, and a stated rule for the one existing stage-less mark. None of the 0155 changes may be done as an erratum.

**Why:** DEFECTS FOUND IN THE PACKET:
(1) Option A's record_form proposes "an erratum to the ADR-0155 residual 5(a) text". 5(a) sits in `## Decision`, and ADR-0092 decision 1 says an erratum leaves "every sentence of `## Decision` and every bullet of `**What this forbids**` standing". It has to be a supersession in part.
(2) Option A misses ADR-0155's forbids bullet "A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes". Under ADR-0092 a forbids bullet also changes only by supersession.
(3) Option A misses ADR-0158's forbids bullet "Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis". The packet's "survives deleting the database (AC-161)" edges into exactly that claim.
(4) The packet argues ADR-0109 decision 3 still holds because "the rejection history still comes from Slice-Verdict". But ADR-0160 records that `Slice-Verdict:` has 0 producers, so in every real run the stage would be resumed from `Story:` alone. The argument can rest only on `Gate:`; failing that, 0109 decision 3 and its forbids bullet on "a single trailer axis" must be superseded in part.
(5) Option A says the trailer is minted "before its store row is written" and cites "ADR-0155 clause 4 ordering". MEASURED: `story_trailer` runs after `rk_kernel::replay::resume(...)` has appended the t2 row. ADR-0155 clause 4 asks only for "as close to the boundary as the seams allow". Git-first ordering would need a kernel seam change that the packet does not cost.
(6) The risk "rk-git, which earlier stage floors froze" is unsupported. No record or runbook text freezes rk-git; it is merely untouched since Step 6 (3 commits, last 7aa02c7).
(7) Option B misses ADR-0157's forbids bullet "Writing an item's advanced stage back to `.redkiln/items/` (AC-518)". That bullet is broader than AC-518 and carries no run-owned-worktree qualifier, so B needs it superseded too. B's claim that AC-518 "already allows this" is true of the criterion and false of the ADR.
(8) Option C's record_form omits ADR-0109 decision 1 (the stage would be read from the store, not from `reconcile()`) and its forbids bullet "a tier-1 transition recorded only in the store". It also omits that ADR-0161 rejected the same option a second time.
(9) The measured state says "Nothing carries the advanced stage into the next invocation". That is imprecise: the store journal does carry it, and the fold's `from` moves. The stall comes from the successor being computed from the record.
(10) The packet does not mention that `answer` shares the record seed (`run::through_step_seven`, then `fsm::fold(item, &resolved.stage, …)`), so the code work must name the shared seam and not run.rs alone.
(11) It misses that one real stage-less `Story:` trailer already exists (`redkiln-rs/rk-fix-doctor-test-count`), and a widened grammar must say how it folds.
(12) It misses that AC-444 is in the Phase-2 corpus-bound acceptance set, which lowers the cost of the re-pin.
(13) It misses that ADR-0105 clause 3 delegates `Story:` values to the design §5.4 table. The "in part" supersession exists because ADR-0155's forbids bullet demands it, not because clause 3's own words change; the Stage: sub-variant is the one that does change them.
(14) It misses that s8 (1)'s trailer read assumes "the trailer values are the pack's stage names", which only a stage-bearing mark satisfies. That is further support for A.

WHAT SURVIVES. Every quoted phrase I checked exists in the file it is attributed to, including the rebuild.rs, run.rs and design phrases. The shape of the recommendation holds. A is the only option that executes ADR-0109 rather than reversing it:
- The stage becomes a tier-1 trailer written to git and read through `reconcile()`.
- `read_commits` already reads `--all --reverse`.
- The run branch persists across invocations, because `recreate_run_worktree` re-attaches the existing `rk-run/<item>`.
- The stage-level next action becomes reproducible from git after the database is deleted.
- It cures an existing violation of 0109's forbids bullet "a tier-1 transition recorded only in the store" for stages 2..N.
- It makes `doctor --rebuild` report the lane's stage, not the record's.

C breaks AC-161/AC-164 by construction. B needs more supersessions of 0109 and 0157, and it recreates the two-sources divergence ADR-0157 diagnosed. ADR-0158's t1 row complements A and is not an alternative.

A is not the cheapest in code: C is. It is the least change to the design's rules. The honest no-supersession fallback is to keep ADR-0157's residual and descope s8 (1)/(1b)/(3). It does not meet s8 (1), because any gate ends the process and only a second `run` can continue the lane.

The operator decision owed is whether to accept ADR-0164 with the supersession list above: seven bullets and clauses across 0105, 0155, 0157, 0109 and 0158, not the packet's three records.

## (b) What does "a landed commit" mean for AC-473 ("every lane stage from intake to a landed commit")? Three parts: who stages the harness's work, where the work lands (rk-run/<item> only, or integrated into a ref the checkout can see), and whether the work survives when a run re-enters.

**TAKEN 2026-09-24:** A — kernel content checkpoint commit on rk-run/<item>; no integration in Phase 1.

**Settled by record already:** False

**Binding records found:**

- AC-473 (AC register, decisions ADR-0002, ADR-0011): a fix-lane run "whose event log contains every lane stage from intake to a landed commit". Verified by "a release check over the event log ... the run id is the evidence". It never defines "landed".
- AC-521 is AC-473's story-lane twin, but it does NOT use the word "landed". Its THEN reads "a Phase-2 story authored into this repository's own backlog by that run" and refuses the claim "if the authored story is not reachable from the run's event log". Verified by "an assertion that the story artifact's digest appears in the run's `artifact.write` events".
- ADR-0094 decision 3 (accepted): "carries every lane stage from intake to a landed commit". Its rejected Node-corpus option reads "a fix lane cannot land a commit on it", so "land" means committing into the target repository.
- ADR-0109 decision 2 (accepted; MISSED by the original packet): tier 1 "is written to git first — a trailer on a checkpoint commit or an empty commit carrying the trailer — and a failed git write leaves zero store rows".
- ADR-0105 (accepted; superseded in part only by 0116, for clause 7's event table). Clause 1: "Git operations on the activity side go through a journalled `GitOps` seam", with "zero" rk-git writes on replay. Clause 2: "Every commit the wrapper makes runs with `-c core.hooksPath=<empty directory>`". Clause 3: "The commit's ref is the run's branch". Clause 4: "When a checkpoint commit is due, the recorded tree is that commit's tree", otherwise a temporary index with "untracked files are excluded". Clause 5's write set includes "`add`" and "fast-forward/merge on the integration ref". Clause 6: "A worktree is **re-created**, never reused, per run". Clause 8: a tier-1 write resolves the run-owned worktree "from the store". What-this-forbids: "Forbids a second `tree_hash` derivation, and forbids including untracked files in it."
- AC-245 (MISSED): the gate's recorded `tree_hash` "is the hash of that same tree object" the human reads, and "when a checkpoint commit is due, the trailer commit's tree". Its Verified by is a property test "including one with an untracked file (whose presence must not change the hash)".
- Design section 7.7 (MISSED): "Serialised at a merge queue | The queue, not the agent, is the authority on what lands". AC-196 (decisions ADR-0018) asks for "a single-writer invariant on the integration ref" and says "no `merge_started` event precedes the preceding merge's trailer commit". AC-197 requires admission by `GreenAt(sha)`. Design section 10.2 puts "project DAG in parallel worktrees plus a merge queue" in Phase 2's scope, not Phase 1's.
- ADR-0077 (accepted; superseded in part by 0098, which leaves its write-path forbid standing): "A process-exec wrapper owns the write path". It forbids "any `std::process::Command` invocation of `git` outside `rk-git`'s single wrapper".
- ADR-0018 (accepted): the kernel "refuses to write when the current worktree's HEAD is not the run's branch". ADR-0105 clause 8 narrows it.
- ADR-0137 clause 3 (0139 and 0140 supersede other clauses, not this one): "**`project_root` is the run-owned worktree**", `<common-dir>/redkiln/worktrees/<item>`.
- AC-518: any orchestrator write ("commit, file write, index change") is refused "unless its target is a run-owned worktree or the store". Verified by: `git status --porcelain` empty and "HEAD unmoved" across a full lane run.
- Design section 5 tier-1 row: "trailers on a checkpoint commit, or an **empty commit** carrying the trailer when no checkpoint is due". No record defines "due".
- Design section 5.9 write-set row repeats ADR-0105 clause 5. No record defines "the integration ref"; AC-196 presumes one behind a merge queue.
- ADR-0156 clause 5(b) (0162 supersedes clause 3 only): the log is scoped to `<base>..rk-run/$FIX_ITEM` and the branch is "asserted **ahead** of the base".

**Measured state:**

Read at 4c282d7. Nothing was run: the build holds the lock.

- **Harness cwd.** `.current_dir(&prepared.worktree)` in rk-adapter-claude/src/runner.rs, which is the run-owned worktree on `rk-run/<item>`.
- **Three tier-1 commit sites, not two.** All three pass `allow_empty = true` and none is preceded by an add:
  - run.rs `story_trailer` ("story {item} entered the lane").
  - run.rs `terminal_read` ("Gate opened for {item_id}").
  - answer.rs, the answer's `Gate:` commit (`answer({}): {}`). The original packet missed this one.
  Runtime `GitOp::Add` callers are only rk-git/src/tree.rs (`read-tree HEAD` plus `add -u` on a temporary index) and redkiln-migrate/src/run_contract.rs (`add -A`, then `commit_with_trailers(..., false)`). That is an in-tree precedent for kernel-side staging, under AC-522.
- **The commits are not journalled.** All three call `rk_git::trailer::write::commit_with_trailers` over `GitExec` directly. `rk_git::gitops::GitOps` (the ADR-0105 clause 1 seam) has no caller in crates/*/src outside rk-kernel/src/replay.rs. So "the kernel's journalled wrapper" is not how commits are made today.
- **Integrate has no caller.** `GitOp::Integrate` exists only in rk-git/src/git_exec.rs.
- **Re-entry keeps commits.** `recreate_run_worktree` runs `worktree remove --force` plus `remove_dir_all`, then `worktree add -b rk-run/<item> <path> HEAD`, falling back to `rk_git::worktree::add` on the existing branch. Commits on `rk-run/<item>` survive re-entry. Uncommitted changes, which today means all harness work, are destroyed.
- **The gate binds the wrong tree (NEW, read not run).**
  - `prepare_inputs` takes `tree_hash` with `TreeBasis::WorkingState` over `resolved.worktree`, which is `repo.worktree`, the invoking checkout's `--show-toplevel`. It does this at stage start, before any dispatch.
  - That value is passed as `RuntimeContext::supplied(tree_hash, deadline)`, and `terminal_read` copies it (`field("tree_hash")`) into the `Gate:` trailer.
  - answer.rs hashes `owned.path()`, the run-owned worktree.
  - So at HEAD the gate never binds the tree the harness produced, which is against AC-245 and ADR-0105 clause 4. Checkpointing before `Gate:` does not by itself make "the tree the human saw a commit". `TreeBasis::CheckpointCommit` exists and only redkiln-migrate uses it.
- **Pack.** `bug-standard.toml` `stage_order` ends "implement", "verify", "review", "release", "operate", "closeout". Policy: release "recommended", operate "forbidden", closeout "recommended". `[stage.release]` goal = "The fix is in production reversibly", baseline `ReleaseRecord`. No activity or record gives release a git action.
- **s8 (1)** reads `git log "rk-run/$FIX_ITEM"`. Its comment: the tier-1 commits "live on rk-run/<item> ... never on the checkout's HEAD".
- **s8 (1b)** runs `git ls-files --error-unmatch "$AUTHORED_STORY"` after `cd D:/repos/redkiln-rs`, against the checkout's index. Under AC-518 a run cannot satisfy that without an operator merge. It is an instrument defect: the register's own instrument for AC-521 is the `artifact.write` digest.
- **Tracker row (stale).** PROGRESS row 8 reads "nothing stages the harness's work, so a LANDED run leaves zero commits". Since 7h a run does leave commits: the empty `Story:` commits. ADR-0156 names the empty `triage` plan as the mechanical cause of the original zero-commits premise. What is true at HEAD is zero CONTENT commits.

**Options:**

### A. Landed means a kernel-made content checkpoint commit on rk-run/<item>. No integration in Phase 1.

The kernel, not the harness, stages the run-owned worktree through the rk-git wrapper (`add -A`, which respects gitignore). It does this at the tier-1 boundaries: before each stage-boundary `Story:` commit, at lane end, and at a `Gate:` open.

A checkpoint is "due" exactly when the staged tree differs from `HEAD^{tree}`. The trailer then rides on that content commit, so the checkpoint and the trailer commit are one commit, as AC-245's "the trailer commit's tree" requires. Otherwise the commit stays empty (ADR-0109 decision 2, design section 5).

At a gate, the gate's `tree_hash` must be re-derived with `TreeBasis::CheckpointCommit` on the run-owned worktree after staging. Today it is the checkout's stage-start WorkingState, so without this the Gate: trailer would name a tree other than its own commit's.

"Landed commit" (AC-473) means the commit on `rk-run/<item>` that carries the lane's last-stage `Story:` trailer, with a non-empty `<base>..rk-run/<item>` diff.

Re-entry keeps ADR-0105 clause 6's re-create. Work survives because it is committed on the branch.

Moving the work into the checkout's line of history is NOT the run's job. AC-518 forbids it, and design section 7.7 makes "the queue, not the agent" the authority on what lands, a queue that section 10.2 schedules for Phase 2. The ADR must say that in Phase 1, "landed" is read short of the queue.

- *Record form:* New ADR-0164 as execution under ADR-0105 clauses 3-5 and ADR-0109 decision 2. It fills the free residual "a checkpoint is due" and the reading of "landed", and supersedes no `## Decision` sentence.

It must state its reading of ADR-0105's "forbids including untracked files in it": files are staged before the derivation, so the CheckpointCommit basis never sees an untracked file. It must cite design section 7.7 and AC-196 as the Phase-2 landing authority.

Plus dated in-place register amendments, not errata:
- AC-245's Verified by: scope the untracked-file property to the no-checkpoint-due basis.
- AC-473: an interpretive note on "landed commit".
- AC-521: a note on "authored into this repository's own backlog" (it has no "landed").

Plus a note on design section 5's tier-1 row.
- *Work:* rk-cli run.rs: a `checkpoint(exec, owned)` helper. It runs `add -A` on the owned worktree, compares `write-tree` with `HEAD^{tree}`, and passes `allow_empty = !due` to the existing trailer commit.

Call sites:
- `story_trailer`.
- Lane end.
- `terminal_read`, together with the gate-hash repair: `tree_hash` from `TreeBasis::CheckpointCommit` over `owned.path()`, not `resolved.worktree`. It is a `RunInputs` / run_id input, so the ADR-0152 run_id grain must be re-checked.

`answer` does not checkpoint; its WorkingState over a checkpointed tree equals HEAD's tree.

Tests:
- A FakeHarness that writes a tracked edit and an untracked file must produce a non-empty `<base>..rk-run/<item>` diff that survives a second `run`.
- A gate opened after that dispatch carries a tree_hash equal to its own commit's tree, and `answer` does not void it.
- AC-518 whole-run: checkout unchanged.
- Mutants: drop the add; use the stage-start hash. Each must go red.

Runbook:
- s8 (1) adds a non-empty `git diff --stat <base>..rk-run/$FIX_ITEM`. A rev-list count alone does not discriminate, because empty `Story:` commits already put the branch ahead.
- s8 (1b) replaces the checkout `ls-files` with `git cat-file -e rk-run/$STORY_ITEM:$AUTHORED_STORY`.

No pack change.
- *Size:* M: about 1.5-2 days. Roughly 200-300 LOC with the gate-hash repair, 4-5 integration tests and two mutants, one ADR, three dated register notes (AC-245, AC-473, AC-521), and two runbook line amendments.
- *Risks:* - **Junk.** `add -A` commits any non-ignored junk the agent leaves, so it needs an exclusion rule or a size/pattern refusal.
- **Gate-hash repair.** It changes a kernel input (tree_hash is in the run digest). A skipped gate-hash repair leaves Gate: trailers whose hash is not their commit's tree, against AC-245.
- **Journalling.** The new add/commit, like the existing three commits, goes through `GitExec`, not the journalled `GitOps`. ADR-0105 clause 1 names the "activity side", and a reviewer may count kernel boundary commits under it too. That is a pre-existing gap A widens.
- **Release says production.** The pack's `release` goal "The fix is in production reversibly" and its `ReleaseRecord` could claim production while the change sits on rk-run. That is a wording hazard for the release activities.
- **Phase 2 needs the stories.** Phase 2 needs the authored stories on the build branch, so an operator merge of rk-run is the hand-off and should be named in s8.
- **Killed dispatch.** A killed dispatch still loses its uncommitted edits, deliberately.
- **Line endings.** CRLF smudge on committed agent output (autocrlf) can make tree comparisons noisy.
- *Citations:* ADR-0109 decision 2: "a trailer on a checkpoint commit or an empty commit carrying the trailer"; ADR-0105 clause 3: "The commit's ref is the run's branch"; ADR-0105 clause 4: "When a checkpoint commit is due, the recorded tree is that commit's tree"; ADR-0105 clause 5: write set includes "`add`"; ADR-0105 clause 6: "re-created, never reused, per run"; ADR-0105 forbids: "forbids including untracked files in it"; AC-245: "when a checkpoint commit is due, the trailer commit's tree"; fixture "an untracked file (whose presence must not change the hash)"; design section 5 tier-1 row: "an **empty commit** carrying the trailer when no checkpoint is due"; design section 7.7: "The queue, not the agent, is the authority on what lands"; AC-518: "refused unless its target is a run-owned worktree"; s8-exit-criterion.md (1): "the tier-1 commits live on rk-run/<item>"; redkiln-migrate run_contract.rs: `exec.run(GitOp::Add, &worktree, ["add", "-A"])`; rk-cli run.rs prepare_inputs: `&resolved.worktree, ... TreeBasis::WorkingState`

### B. Option A, plus kernel-owned integration: the run fast-forwards a declared integration ref at the lane's terminal stage.

Everything in A, plus a call to the idle `GitOp::Integrate` at the terminal boundary: an ff-only `update-ref` of a DECLARED `[git] integration_ref`.

The run refuses if that ref is checked out in any worktree (AC-518's "HEAD unmoved"), or if the ref is not an ancestor of rk-run/<item>.

"Landed" then means reachable from the integration ref.

- *Record form:* This is NOT execution-only.

Design section 7.7 ("The queue, not the agent, is the authority on what lands") and AC-196/AC-197 (a serialised merge queue with a single writer on the integration ref, `merge_started` ordering and `GreenAt(sha)` admission) already bind the integration ref's writer. Section 10.2 puts the merge queue in Phase 2's scope.

A run that fast-forwards the ref directly is an agent-side landing past the queue. It therefore needs one of two things:
- a new ADR that builds the queue's single-writer and admission rules into Phase 1 (scope growth against design section 10.2's Phase-1 row), or
- a superseding record that relaxes section 7.7 and AC-196 for Phase 1.

Either way it also defines the integration ref and adds a config-schema row.
- *Work:* Everything in A, plus:
- An `rk_git::integrate` wrapper (ff-only via `merge-base --is-ancestor` plus `update-ref <ref> <new> <old>`), journalled.
- A `worktree list --porcelain` refusal.
- A config key.
- The terminal call and a divergence gate.
- AC-196's single-writer and `merge_started` events.
- AC-197's `GreenAt(sha)` admission.
- Tests for each refusal.
- An AC-518 whole-run test with integration on.
- A dedicated `rk-integrated` ref for this repo, since `build/phase-0-to-1` is checked out.

Runbook s8 (1) and (1b) then read that ref.
- *Size:* L: about 3-4 days. Roughly 500-800 LOC and tests with AC-196/197, an ADR that supersedes or extends Phase-1 scope, a config row, and runbook amendments.
- *Risks:* - Builds a Phase-2 mechanism (the merge queue) in Phase 1, or bypasses it.
- Adds a second "done" authority that reconcile, rewrite events (ADR-0018) and doctor's orphan rules do not model.
- Writing a ref other than the run's branch strains ADR-0018's "refuses to write when the current worktree's HEAD is not the run's branch".
- On this repo the checked-out build branch can never be the target, so the landing is still invisible from the checkout: the same visible outcome as A at 2-3x the cost.
- A concurrent-item race surface on one ref (Halt 6, AC-196).
- *Citations:* ADR-0105 clause 5: "fast-forward/merge on the integration ref"; design section 7.7: "The queue, not the agent, is the authority on what lands"; AC-196: "a single-writer invariant on the integration ref"; design section 10.2 Phase 2 scope: "project DAG in parallel worktrees plus a merge queue"; rk-git git_exec.rs: `Integrate` with no caller; AC-518 Verified by: "HEAD unmoved"; ADR-0018: "refuses to write when the current worktree's HEAD is not the run's branch"

### C. The pack stages: the implement stage's activity (a tool step or the agent) commits its own work.

The kernel keeps making only empty trailer commits. A `tool` step or the harness agent runs `git add`/`git commit` in the run-owned worktree, and "landed" means "the implement stage's activity committed".

- *Record form:* Supersedes ADR-0105 in part:
- clause 1: activity-side git ops would not go through the journalled `GitOps` seam, so replay cannot guarantee zero writes;
- clause 2: an agent's own `git commit` runs the repo's hooks.

It also narrows ADR-0077's "Forbids any `std::process::Command` invocation of `git` outside `rk-git`'s single wrapper" if a Rust tool step does it.

The gate tree-hash defect (AC-245) stays owed separately.

Pack authoring across `.redkiln/catalog` (implement activities: 14, per ADR-0156's census).
- *Work:* - A new git-commit tool capability or card-level instructions.
- Pack edits across implement activities.
- Replay/journal carve-outs.
- Hooks exposure accepted or re-mitigated.
- *Size:* L: 3-5 days plus two supersession records and pack re-authoring.
- *Risks:* - Commits made outside the wrapper are invisible to the journal, which breaks AC-006/AC-008's "zero rk-git writes".
- Target-repo hooks can rewrite messages (the AC-162 fault fixture).
- Nothing guarantees the agent commits at all (the RK-B0013 "never spoke" shape).
- It reverses two accepted rules to buy what A gets without reversing any.
- *Citations:* ADR-0105 clause 1: "Git operations on the activity side go through a journalled `GitOps` seam"; ADR-0105 clause 2: "the target repository's hooks never run on a Redkiln commit"; ADR-0077: "Forbids any `std::process::Command` invocation of `git` outside `rk-git`'s single wrapper"

**Recommended:** A, corrected. "Landed" means a kernel-made content checkpoint commit on rk-run/<item>, with no integration inside the run.

Record it as ADR-0164, as execution under ADR-0105 clauses 3-5 and ADR-0109 decision 2, superseding nothing. It defines when "a checkpoint is due", reads ADR-0105's untracked-file forbid as applying before staging, and names design section 7.7's Phase-2 merge queue as the authority for landing on mainline.

Add dated in-place register notes on AC-245 (scope the untracked-file fixture), AC-473 ("landed commit") and AC-521 ("authored into this repository's own backlog").

Bundle the gate tree-hash repair, which takes the hash from the checkpoint on the owned worktree instead of from the checkout at stage start, because work at a gate does not survive re-entry without it.

Repair s8 (1) and (1b) to read the run branch's diff and tree instead of the checkout's index.

**Why:** DEFECTS FOUND IN THE ORIGINAL PACKET:
1. **Missed ADR-0109 decision 2.** This accepted record is the tier-1 rule itself: "a trailer on a checkpoint commit or an empty commit carrying the trailer". A is execution under it as much as under ADR-0105.
2. **Missed AC-245.** It says "when a checkpoint commit is due, the trailer commit's tree", so the checkpoint and the trailer commit must be one commit. Its Verified-by fixture requires that "an untracked file (whose presence must not change the hash)". A's `add -A` due-rule makes an untracked-only change a due checkpoint, so that property fails whenever a checkpoint fires. ADR-0105's forbids bullet "forbids including untracked files in it" is implicated too. A cannot claim "no record touched" without a stated reading and a dated AC-245 amendment.
3. **Missed design section 7.7, AC-196/AC-197 and section 10.2.** Section 7.7: "The queue, not the agent, is the authority on what lands". AC-196 requires a single writer on the integration ref and `merge_started` ordering. AC-197 requires `GreenAt(sha)` admission. Section 10.2 puts the merge queue in Phase 2's scope. B's "execution under ADR-0105 clause 5, no supersession" is therefore wrong: a run fast-forwarding the ref bypasses a bound admission route. A must also cite section 7.7 to justify reading "landed" short of the queue in Phase 1.
4. **False measured-state (gate).** The claim was that the next run "wipes the tree the human reviewed" and that checkpointing before `Gate:` "makes the tree the human saw a commit". At HEAD (read, not run), the gate's `tree_hash` comes from `prepare_inputs` over `resolved.worktree`, the invoking checkout, with the WorkingState basis at stage start, before dispatch. `terminal_read` copies it into the trailer, while `answer` hashes `owned.path()`. So the gate never binds the harness's tree. A's gate checkpoint helps only together with a hash repair, which touches a RunInputs/run_id input, so the estimate rises from S-M to M.
5. **Omitted the third tier-1 commit site.** answer.rs's `Gate:` answer commit is also `allow_empty = true`. It was missing from the survey.
6. **False "journalled" claim.** The "why" said git writes go through "the kernel's journalled, hook-isolated wrapper". At HEAD, rk-cli's commits call `commit_with_trailers` over `GitExec`, and `GitOps` has no src caller outside rk-kernel's replay. A widens that gap, and it is now listed as a risk.
7. **Stale tracker row endorsed.** The quoted row reads "nothing stages the harness's work, so a LANDED run leaves zero commits". It was called consistent, but since 7h a run leaves empty `Story:` commits, and ADR-0156 attributes the zero-commits premise to the empty `triage` plan. What is true is zero CONTENT commits.
8. **Wrong wording for AC-521.** AC-521 was treated as carrying "landed". Its THEN reads "authored into this repository's own backlog", and its instrument is the `artifact.write` digest. The interpretive note must read that phrase instead.
9. **Non-discriminating check.** A's s8 (1) `rev-list --count > 0` passes on empty `Story:` commits alone (ADR-0156 5(b) already asserts "ahead"). Only the diff check discriminates.
10. **Missed pack wording.** The pack's `[stage.release]` goal is "The fix is in production reversibly", with a `ReleaseRecord` baseline. That is a wording hazard under A.
11. **Beyond least change.** A checkpointed on "every terminal exit (FindingsBlock or refusal)". Boundary checkpoints (Story:, Gate:, lane end) are enough.
12. **Minor.** The supersession state of ADR-0077 (0098), ADR-0137 (0139/0140) and ADR-0156 (0162) went unstated. Each cited clause was checked and stands.

SURVIVING REASONING:
- **Who stages.** ADR-0105 clauses 1-2 and ADR-0077 put git writes in rk-git's hook-isolated wrapper, and `add` is on clause 5's write set. That excludes C without two supersessions.
- **Survival.** Clause 6 forbids worktree reuse, and recreate_run_worktree already keeps branch commits. Committing on `rk-run/<item>` is therefore the only survival mechanism the records permit.
- **Where it lands.** AC-518 forbids writing to the checkout. Section 7.7 and AC-196 make a Phase-2 merge queue the authority on what lands. So in Phase 1, "landed" can only mean committed on the run branch in the target repository, which is consistent with ADR-0094's "land a commit on it". B costs 2-3x, needs a scope or supersession record, and still is not visible from the checkout.

What is genuinely free is (i) when a checkpoint is due and (ii) the reading of "landed" and of AC-521's "authored into this repository's own backlog". A settles both without superseding a `## Decision` sentence, but it owes dated register amendments (AC-245, AC-473, AC-521) and the gate-hash repair.

already_decided_by_record is false: the records fix who writes and how work survives, but not when a checkpoint is due or what "landed" means.

## (c) How does the fix lane's verify stage (bug-standard baseline TestReport, ConfirmationResult, SmokeResult) get performed through rk-adapter-toolchain-rust, and what record carries AC-473's "the toolchain stage was performed by the Rust adapter through the port" when a successful tool step journals nothing?

**TAKEN 2026-09-24:** A — rk-cli verify pass on ADR-0162's shape, selected by `TestReport`; AC-473 "through the port" register reading approved by the operator.

**Settled by record already:** False

**Binding records found:**

- ADR-0148 (accepted, supersedes 0135 in part): Decision clause 1 "`toolchain:test` refuses"; clause 3 "`catalog::Step` is not widened to parse `inputs`"; clause 4 "This is reversed by a resolver, not by a revert" (ADR-0136 clause 4). Forbids: "Calling `Toolchain::test` with an empty selection from the executor", "Passing a declared `selection` string through as a test selector without a resolver", "Widening `catalog::Step` to parse `inputs` without a record routing what may then read it"
- ADR-0135 (accepted; superseded_by 0148): clause 1 "The map is closed and holds exactly four entries"; clause 2 every other verb refused "by name"; clause 4 `change_summary` "must not be 'fixed' into a synthesised value"; clause 6 "The 29 unmapped verbs are recorded as an open gap, owned by no stage in this phase" (smoke is one of them); the NOT-decided paragraph "It adds no method to the `Toolchain` port: the count stays **six**". Its decision drivers open with "AC-473 needs a `tool` step to reach `rk-adapter-toolchain-rust` behind the port" (a driver, not a Decision sentence)
- ADR-0136 (accepted) clause 4: "The reference resolver is OWED ... no stage in this runbook currently claims it". Scope: this record governs `when` guards and says it "does not decide what a `check` step's `expr` does with a reference"
- ADR-0162 (accepted, D21-38; MISSED by the original packet): clause 5 "The review pass is selected by an activity's declared `provides`, never by a stage name" and "`resolved.stage` is compared to no string literal anywhere in `crates/rk-cli/src/`, and this record does not introduce the first one". Clause 4: the re-executor is `Toolchain`-backed and "applying a patch would need a seventh method on a port Checkpoint 7a counts at six". It is the built precedent (crates/rk-cli/src/review_pass.rs) for a composition-root pass that calls the port outside the catalog executor
- ADR-0137 (accepted; superseded in part by 0139 and 0140): clause 1 "`run.rs`'s `Ports` takes a real `Arc<dyn Toolchain>`"; clause 2 "The adapter is chosen by `Toolchain::detect()`"; clause 3 `project_root` is the run-owned worktree; clause 7 `program` comes from `[adapters] toolchain_programs`, so argv[0] can be configured
- ADR-0139 (accepted) clause 3: no `Toolchain` "on the `run` path may answer `Ok` to `build`, `test`, `lint`, `classify` or `change_summary` without" doing the work
- ADR-0140 (accepted, supersedes 0137 cl.5 in part; MISSED): "The construction-time `timeout` is THE bound on every child a toolchain adapter spawns. It is not a backstop". 3600 s stands
- ADR-0149 (accepted): detect chooses before a candidate adapter is validated. This applies to which adapter the runner holds
- ADR-0003 / design P6 / AC-487: "`rk-kernel` contains no comparison against an adapter identity — no adapter name string". Its scope is rk-kernel; rk-cli is not bound by it
- ADR-0094 decision 1: Rust `test` = `cargo test --workspace` "(or `cargo nextest run` where present, recorded on the report)"; a Rust pack binds `TestReport`
- ADR-0116 decision 1: `artifact.write` is "emitted by **every activity output**", with payload kind, item id, `schema_version` and content digest, and no adapter field. Decision 3: AC-521's check resolves against it, "and AC-473's twin does the same on the fix lane"
- ADR-0104 decision 3 (the bootstrap comparator seeds from the whole-suite count); decision 1 (the per-AC reverted-fix mutation harness; its patch application is not built and is refused on the port per ADR-0162 cl.4)
- ADR-0073: keyed kinds expand "one instance per element of the item's `toolchain` set". The key is the item's facet, not the adapter identity
- ADR-0156 clause 4 plus its enumerated blockers 0 to 5 (cards lack `permission_mode`; `.redkiln/schemas/` absent; step-output resolution OWED). These gate any shipped-pack lane before verify is reached
- ADR-0092 rule 1: a `## Decision` sentence moves by supersession. The AC register owns each criterion's GIVEN/WHEN/THEN (design header, per ADR-0092)
- AC-473 THEN (not only its Verified-by): "refused ... if the toolchain stage was performed by anything but the Rust adapter through the port"; Verified-by: "an adapter-identity assertion on the toolchain stage's dispatch record"
- AC-485 (MISSED): "asserts adapter identity on both dispatch events". s7d §7d.18 carries it PROVISIONAL because "the dispatch-event identity half has no `DispatchRecord` field to read and the carrier is routed, RT-41"
- AC-328: "selection is refused with `StepError::Unsupported(\"affected-selection\")` and the pack routes to an explicit whole-suite policy"
- AC-324: `Result<VerifyReport, StepError>` at the boundary; missing basis/digest/argv must be `Err`
- AC-331 (killed means `Unverifiable`) and AC-344 (cannot-run means `Unverifiable { cannot_run }`)
- AC-347: "a verify run that emits any harness dispatch fails the activity"; lint "the verify activity's step kinds contain no agent step"
- Runbook R-9 / RT-6 (s7c-adapters.md): "an inherent `fn verify_report(&self, selection)` on each toolchain adapter; port unchanged" and "the transport into `rk_kernel::verify` is 7d's composition root — a record is OWED (R-9)"
- Runbook s7d §7d.18 (MISSED in scope): the verify block AC-320, AC-338, AC-339, AC-341, AC-348, AC-349, AC-353, AC-355, AC-359 is "RAISED here too, not built — the verify activity runner is neither an adapter's nor this stage's set ... a record routing the block to a stage is OWED"
- .redkiln/artifacts.toml `[artifact.TestReport]`: `source = "activity"`, `keyed_by = "toolchain"`. `KindSource::Activity` is documented as "Some catalog activity `provides` it" (crates/rk-kernel/src/select/artifacts.rs)

**Measured state:**

Everything below was read at 4c282d7 (git rev-parse confirmed). Nothing was built or run.

(1) Executor, crates/rk-kernel/src/runtime/step.rs. `const MAPPED_VERBS: [&str; 4] = ["build", "change_summary", "lint", "test"]`. `fn run_tool_step(&self, step: &Step) -> Result<(), StepError>`. The `"test" =>` arm returns `Err(StepError::Unsupported {..})`, and verbs outside the map (smoke among them) are refused by name. The Tool arm is `Ok(()) => StepVerdict::Continue`, so a successful tool step journals nothing and its value is discarded. At the end of the walk, an activation with no agent step is refused: "walked its steps with no `agent` step producing a value ... `catalog::Step` parses neither `artifacts` nor `evidence`, so the binding needs the reference resolver ADR-0136 records as OWED". (ADR-0136 itself governs `when` guards only. The `check` expr and `emit` bindings are the wider step-output-resolution debt, ADR-0156 blocker 5.)

(2) The verify producers in .redkiln/catalog/, all `stage = "verify"`:
- confirmation-test (ConfirmationResult): tool `toolchain:test` with `selection = "artifact:ReproCase.command"`, then check, then gate `when = "${before.outputs.status} != 'passing'"`, then a composing emit over `${before.outputs.*}`. No agent step.
- smoke-test (SmokeResult): tool, tool, emit. No agent step.
- unit-tests (TestReport): tool `selection = "affected"`, check, and two agent steps (role implementer) under `when = "rigor >= recommended"`.
- equivalence-boundary and property-based-testing (TestReport): both carry agent steps. equivalence-boundary's are unguarded.
- Five files declare `provides` TestReport: those three plus unit-tests-ue and migration-dry-run.

`verify_activity_has_no_agent_step` (crates/rk-kernel/src/verify.rs) is called only from tests/it/verify.rs on the two ac347 fixtures and never on the shipped catalog. That is a latent AC-347 conflict.

(3) Port. `Toolchain::test(&[String]) -> Result<u32, StepError>`. The Rust adapter's port `test` is `self.verify_report(selection).map(|r| r.selected_test_count)`, so the port path already computes the full report and discards it. The inherent `RustToolchain::verify_report` always returns `basis: WholeSuite, basis_reason: Some(ProbeUnverified)`. On a red suite it returns `Err` (`expect_green`, "`NonZero` on a red suite (RT-22)"), not a report with failures. Timeout is `Timeout`; OS kill and spawn failure are `Transport`. Node has the same inherent method. `rk_kernel::verify::report_at_boundary(value: serde_json::Value) -> Result<VerifyReport, BoundaryError>` exists. `grep` finds no `src/` caller of `verify_report` or `report_at_boundary` outside the adapters themselves, so R-9 stands.

(4) Precedent. crates/rk-cli/src/review_pass.rs (ADR-0162) is a built composition-root pass: `ToolchainReExecutor` implements `ReExecutor` over the port, and it is selected by `provides`, not by a stage literal.

(5) Adapter identity. `real_toolchain` in run.rs holds the chosen adapter's `name` in the `detect()` loop but records it nowhere. `reconcile_started.adapter_set` is `config.toolchains()`, meaning every configured toolchain. argv[0] comes from `program_for(name, DEFAULT_PROGRAM)` and can be configured (ADR-0137 cl.7). The ADR-0116 `artifact.write` payload has no adapter field. AC-485's dispatch-event identity half is PROVISIONAL for the same missing carrier (RT-41).

(6) Upstream reachability. The fix lane cannot reach verify at HEAD whatever this decision says:
- ADR-0156 blocker 0: 8 of 8 cards lack `permission_mode`.
- Blocker 1: `ls .redkiln/schemas` fails.
- bug-standard's implement baseline is `["Changeset", "ImplReport", "ChangeSummary"]`, and ChangeSummary refuses by ADR-0135 clause 4.

(7) Pack baselines:
- bug-standard `[stage.verify] baseline = ["TestReport", "ConfirmationResult", "SmokeResult"]`, with goal "The repro passes at the fix sha and nothing regressed".
- story `[stage.verify] baseline = ["TestReport", "SmokeResult"]`.

(8) No DECISIONS-2026-09-2* entry (through D21-39) addresses R-9 or the §7d.18 verify block. s8-exit-criterion.md says "the verify stage's toolchain is rk-adapter-toolchain-rust".

**Options:**

### A. A composition-root verify pass (the ADR-0162 shape) closes R-9, and the verify baselines are re-routed to TestReport

Beside review_pass.rs, rk-cli gains a verify pass. It is selected by the required kind `TestReport` (declared `provides`/baseline), never by a `resolved.stage == "verify"` literal (ADR-0162 cl.5). It calls `verify_report` on the adapter `detect()` chose, which is whole-suite with `basis_reason: ProbeUnverified` (AC-328's explicit whole-suite policy). It validates the result through `report_at_boundary` (AC-324), feeds the ADR-0104 d3 comparator, and writes the keyed TestReport with one `artifact.write` row (ADR-0116 d3 already names this row as AC-473's fix-lane carrier). Adapter identity is the name `real_toolchain` chose, written by rk-cli. That is allowed because AC-487/P6 binds rk-kernel only. argv[0] is not the identity: it is configurable (ADR-0137 cl.7), and `rustc -V` names the compiler, not the adapter. The same carrier also discharges AC-485's PROVISIONAL identity half (RT-41).

The pass displaces the catalog TestReport producer for that node. That is a stated change, because TestReport is `source = "activity"` and the selection would otherwise walk unit-tests, whose agent steps also breach AC-347. SmokeResult leaves both the bug-standard and story baselines: `smoke` is unowned under ADR-0135 cl.6. ConfirmationResult leaves bug-standard's baseline and is recorded as an owed narrowing. It is NOT carried by ADR-0104's mutation harness, which is unbuilt and whose patch application ADR-0162 cl.4 refuses on the port. ADR-0135, 0136 and 0148 stand. The 28 catalog `toolchain:test` steps keep refusing.

- *Record form:* A new ADR-0164, executing under AC-328, AC-347, ADR-0104 d3, ADR-0073 and ADR-0116 d3 on ADR-0162's precedent, and superseding no Decision sentence. It discharges R-9/RT-6 and is the routing record s7d §7d.18 says is OWED for the verify block (AC-320, AC-338, AC-339, AC-341, AC-348, AC-349, AC-353, AC-355, AC-359), naming by id which it claims and which stay RAISED. The pack edits (bug-standard and story `[stage.verify]`) are recorded inside it, on the s7e disposition precedent.

One register item needs explicit operator approval. AC-473's THEN says "through the port", and `verify_report` is an inherent method (RT-6 "port unchanged"). Either the register's THEN/Verified-by are amended to read "by the port-implementing adapter `detect()` chose", with the carrier being the keyed TestReport artifact.write plus the chosen adapter's name, or that reading is refused and option C's port widening follows. The register owns GIVEN/WHEN/THEN (ADR-0092), so this is a register amendment ledgered in DECISIONS as a D-entry, not only a Verified-by touch. The s8-exit-criterion.md sentence "the verify stage's toolchain is rk-adapter-toolchain-rust" is updated to name the carrier.
- *Work:* rk-cli:
- `real_toolchain` also returns the chosen adapter name and a concrete handle, so `Ports` keeps `Arc<dyn Toolchain>` (ADR-0137 cl.1 unchanged).
- New `verify_pass.rs`: `verify_report` → `report_at_boundary` → comparator → store → append `artifact.write`, then re-read the journal (the ADR-0162 cl.6 idiom).
- Map `Timeout` and OS-kill to `Unverifiable` (AC-331), Spawn to `cannot_run` (AC-344), and red (`NonZero`) to a failed verify, never a report.
- Record the chosen adapter name on a CLI-written row.
- Node goes through its own `verify_report`.

Packs: bug-standard and story `[stage.verify]` baselines.

Tests:
- a `verbs::run::run` integration test over a fixture Cargo project, asserting the TestReport `artifact.write` row, `basis` whole-suite and the recorded adapter name;
- a negative control where a Node-detected fixture records the Node adapter;
- zero `dispatch_completed` rows in the pass (AC-347);
- a mutant that swaps the recorded name must go RED.
- *Size:* M: most parts exist and are tested (verify_report, report_at_boundary, validate_report, the comparator, the review_pass template). What is new is the pass, the identity row, the producer displacement and two pack edits. It does NOT make AC-473 reachable alone, because upstream blockers (cards, schemas, the ChangeSummary implement baseline) remain.
- *Risks:* - Two routes to a TestReport exist: the pass, and the refusing catalog steps. A later reader could restore the catalog route without reading ADR-0164.
- The "through the port" reading of AC-473 is contestable, and the operator may refuse it.
- Displacing a `source = "activity"` producer is new selection behaviour, and ADR-0116 d1's "every activity output" wording does not literally cover a pass-written artifact. ADR-0164 must say so.
- Dropping ConfirmationResult leaves bug-standard's own goal "The repro passes at the fix sha" without a baseline artifact.
- A whole-suite `cargo test --workspace` in a cold run-owned worktree runs against ADR-0140's 3600 s, which is THE bound, not a backstop.
- The shipped catalog's AC-347 conflict (unit-tests, equivalence-boundary, property-based-testing) is left alone, not resolved.
- *Citations:* runbook/phase-1/s7c-adapters.md: "the transport into `rk_kernel::verify` is 7d's composition root — a record is OWED (R-9)"; runbook/phase-1/s7d-cli-and-migrate.md: "the verify activity runner is neither an adapter's nor this stage's set (no §12 or §13 criterion names it); a record routing the block to a stage is OWED"; references/adr/0162-...md clause 5: "The review pass is selected by an activity's declared `provides`, never by a stage name"; references/adr/0116-...md decision 3: "AC-473's twin does the same on the fix lane"; references/2026-09-08-acceptance-criteria.md AC-473: "if the toolchain stage was performed by anything but the Rust adapter through the port"; references/2026-09-08-acceptance-criteria.md AC-328: "the pack routes to an explicit whole-suite policy"; references/2026-09-08-acceptance-criteria.md AC-487: "`rk-kernel` contains no comparison against an adapter identity"; references/adr/0137-...md clause 7: "`program` is `[adapters] toolchain_programs`' entry for the chosen adapter"; references/adr/0140-...md: "It is not a backstop, because there is nothing behind it"; crates/rk-adapter-toolchain-rust/src/report.rs: "`basis` is **always `WholeSuite`** with"; crates/rk-kernel/src/verify.rs: `pub fn report_at_boundary(value: serde_json::Value) -> Result<VerifyReport, BoundaryError>`; crates/rk-kernel/src/select/artifacts.rs: "Some catalog activity `provides` it"

### B. The general executor route: selection resolver, smoke mapping, tool-output binding and a `tool_step_completed` event

The catalog stays the verify route and every missing piece gets built:
- (i) A selection resolver: `"affected"` goes to AC-328's whole-suite policy, `artifact:ReproCase.command` to a selector, and `${…}` through step-output resolution (ADR-0136 cl.4 for guards, plus the check/emit resolution ADR-0156 blocker 5 records as OWED).
- (ii) `smoke` enters the map.
- (iii) Tool steps return values bound for `${step.outputs}`.
- (iv) Agent-free activities bind emit outputs to `provides`.
- (v) A new tier-2 event `tool_step_completed` carries adapter identity from data rk-cli supplies.

- *Record form:* - Supersede ADR-0148 whole (its clause 4 invites this).
- Supersede ADR-0135 cl.1 and cl.6 in part for `smoke`.
- A new ADR executes the owed resolver, including iteration semantics.
- A new ADR adds the event kind (the ADR-0116/0126 precedent).
- `smoke` needs either a seventh port method, contradicting ADR-0135's "the count stays **six**" and ADR-0162's "Forbids a seventh method on the `Toolchain` port", or a synthesised success, which ADR-0139 cl.3 forbids.
- *Work:* - step.rs: a value-returning `run_tool_step` and a resolver.
- `catalog::Step` parses `inputs` (ADR-0148 cl.3 and its forbids bullet).
- The emit binding, and removal of the no-agent refusal.
- A port change for smoke.
- Adapters, fakes and conformance.
- The new journal row and its fold arm.
- The AC-347 lint run over the shipped catalog, then rewrite the three activities it fails.
- *Size:* XL: several stages. The resolver is unowned by any runbook stage.
- *Risks:* - The largest surface, and the resolver has no owner.
- It re-opens the `cargo test affected` exits-0 trap ADR-0148 measured.
- `smoke` has no semantics on a CLI workspace.
- It breaks the six-method invariant asserted by ADR-0135, ADR-0162 and Checkpoint 7a.
- The AC-347 conflict surfaces immediately.
- Self-hosting waits on all of it.
- *Citations:* references/adr/0148-...md: "This is reversed by a resolver, not by a revert"; references/adr/0136-...md: "The reference resolver is OWED"; references/adr/0135-...md: "It adds no method to the `Toolchain` port: the count stays **six**"; references/adr/0162-...md: "Forbids a seventh method on the `Toolchain` port"; references/adr/0139-...md clause 3: "on the `run` path may answer `Ok`"; crates/rk-kernel/src/runtime/step.rs: "walked its steps with no `agent` step producing a value"; references/2026-09-08-acceptance-criteria.md AC-347: "a verify run that emits any harness dispatch fails the activity"

### C. A narrow executor route: one literal, one new agent-free activity, a widened port `test`

The verify stage stays inside the catalog walk and admits one shape:
- `toolchain:test` with `selection` exactly `"affected"` routes to the AC-328 whole-suite policy.
- A new `whole-suite-test` activity (`[tool, emit]`, no `${…}`) is the TestReport producer.
- The executor binds that tool value as `provides`.
- `Toolchain::test` returns `VerifyReport`, so the stage literally runs "through the port". This is the one option that satisfies AC-473's THEN without a register reading.
- `smoke` stays refused, and SmokeResult and ConfirmationResult leave the baselines.
- The carrier is the keyed TestReport artifact.write, plus the adapter name recorded by rk-cli.

- *Record form:* A new ADR-0164:
- supersedes ADR-0148 clause 1 in part (the `"affected"` literal only) and clause 3 in part (`inputs.selection` is parsed);
- records the narrow emit binding;
- reverses RT-6's inherent-method routing, which is a runbook routing and not a Decision sentence;
- carries the pack and catalog authoring;
- routes the §7d.18 verify block.
No AC-473 register reading is needed. The Verified-by line still needs a register amendment, because nothing is a "dispatch record".
- *Work:* - Widen the `Toolchain::test` signature to `VerifyReport` (the count stays six), rippling into both adapters, `FixtureToolchain`, `UndetectedToolchain` and conformance.
- `catalog::Step` gains `inputs.selection`.
- `run_tool_step` returns a value.
- A special-case emit binding.
- A new catalog TOML, and edits to bug-standard.toml and story.toml.
- *Size:* L: a port signature change across both adapters and the fakes, plus an executor special case.
- *Risks:* - This is the half-built state ADR-0148 cl.3 warns about.
- The `[tool, emit]` special case must later be absorbed by the resolver.
- It contradicts RT-6's recorded choice.
- Node's report-returning `test` needs its own verification.
- Check steps over tool outputs stay unresolved.
- *Citations:* references/adr/0148-...md clause 3: "`catalog::Step` is not widened to parse `inputs`"; references/adr/0148-...md: "`\"affected\"` × 17"; runbook/phase-1/s7c-adapters.md RT-6: "an inherent `fn verify_report(&self, selection)` on each toolchain adapter; port unchanged"; crates/rk-ports/src/toolchain.rs: `fn test(&self, selection: &[String]) -> Result<u32, StepError>;`; references/2026-09-08-acceptance-criteria.md AC-473: "through the port"

**Recommended:** A as corrected: a verify pass in rk-cli on ADR-0162's shape. It is selected by the required kind `TestReport`, never by a stage literal, and it discharges R-9 and routes s7d §7d.18's verify block. The TestReport is written keyed, with one `artifact.write`. The chosen adapter's name is recorded by rk-cli as AC-473's (and AC-485's) identity carrier. SmokeResult leaves both baselines and ConfirmationResult leaves bug-standard's as a named, owed narrowing. Record it as ADR-0164 plus an AC-473 register amendment that the operator approves explicitly: the "through the port" reading in THEN, and the "dispatch record" wording in Verified-by. If the operator refuses the "through the port" reading, C's port widening is the fallback.

**Why:** DEFECTS FOUND IN THE ORIGINAL PACKET:
1. It missed ADR-0162 (accepted, D21-38). Clause 5 says a pass is "selected by an activity's declared `provides`, never by a stage name", and `resolved.stage` is compared to no literal in rk-cli/src. Option A's "at the verify stage" would introduce the first such literal. ADR-0162 is also the direct built precedent (review_pass.rs) that A should mirror.
2. AC-473's THEN clause, not only its Verified-by, says "through the port". A routes through the inherent `verify_report`, which is not a port method (RT-6 "port unchanged"). So A needs an operator-approved register reading of the THEN clause, not just a Verified-by swap. The packet presented that as a Verified-by-only amendment.
3. The identity carrier was wrong. argv[0] comes from `[adapters] toolchain_programs` and can be configured (ADR-0137 cl.7), and `rustc -V` names the compiler, not the adapter. The ADR-0116 `artifact.write` payload has no adapter field. The honest carrier is the adapter name `real_toolchain` chose, written by rk-cli. AC-487/P6 binds rk-kernel only.
4. It missed AC-485, whose dispatch-event identity half is PROVISIONAL in s7d §7d.18 ("the carrier is routed, RT-41"). It is the same missing carrier and should be co-routed.
5. It missed s7d §7d.18's RAISED verify block (AC-320, AC-338, AC-339, AC-341, AC-348, AC-349, AC-353, AC-355, AC-359). There, "a record routing the block to a stage is OWED", and ADR-0164 is that record.
6. It cited a stale bound. It called the 3600 s an "ADR-0137 clause 5" backstop, but ADR-0140 superseded that sentence: "It is not a backstop, because there is nothing behind it".
7. It made a false carrier claim. "ConfirmationResult's intent is carried by ADR-0104's reverted-fix mutation": that harness is unbuilt, and ADR-0162 cl.4 refuses patch application on the port ("would need a seventh method"). Dropping ConfirmationResult is a real narrowing and leaves bug-standard's goal "The repro passes at the fix sha" without an artifact.
8. It left a mechanism unstated. TestReport is `source = "activity"` ("Some catalog activity `provides` it"), so the selection would still walk unit-tests. A must state that it displaces that producer. ADR-0116 d1's "every activity output" wording does not literally cover a pass-written artifact, and d3 already names `artifact.write` as the AC-473 carrier, which the packet did not cite.
9. Measured state omitted three facts:
   - The fix lane cannot reach verify at HEAD (ADR-0156 blocker 0 cards; `.redkiln/schemas` absent; the bug-standard implement baseline ChangeSummary refuses under ADR-0135 cl.4).
   - The Rust port `test` already computes the full report and discards it (`self.verify_report(selection).map(|r| r.selected_test_count)`).
   - `verify_report` returns `Err` on a red suite rather than a failing report.
10. It overreached on ADR-0136. It attributed check/emit resolution to ADR-0136 cl.4, but that record governs `when` guards and says it "does not decide what a `check` step's `expr` does". The check/emit debt is ADR-0156 blocker 5 / step-output resolution. (step.rs's own message cites ADR-0136, so the code shares the looseness.)
11. It cited "s8-exit-criterion.md line 49" by line number, which the cite-by-quotation rule forbids. It is replaced by the quote "the verify stage's toolchain is rk-adapter-toolchain-rust".
12. story.toml's verify baseline `["TestReport", "SmokeResult"]` also carries SmokeResult. That is now explicit rather than "check the same way".

WHAT SURVIVES: No accepted record settles the route. AC-328 fixes what runs, ADR-0148 fixes what the executor may not do, ADR-0135 cl.6 leaves `smoke` unowned, and R-9 names the transport as an OWED record. So a decision is owed.

A remains the least-change route:
- It supersedes no `## Decision` sentence.
- It reuses built, tested parts on an accepted precedent (ADR-0162).
- It satisfies AC-347 by construction.
- It keeps P6, because the identity is written by rk-cli, not the kernel.

B waits on the unowned resolver and forces either a seventh port method (forbidden by ADR-0135 and ADR-0162) or a synthesised success (ADR-0139 cl.3). C is the only option that literally satisfies "through the port" without a register reading, but it builds the parsed-and-partly-routed state ADR-0148 cl.3 exists to prevent and reverses RT-6. So it is the fallback if the operator refuses A's reading.

The operator should approve explicitly:
- (a) the AC-473 THEN/Verified-by reading;
- (b) dropping SmokeResult from both lanes and ConfirmationResult from bug-standard as named narrowings;
- (c) the pass displacing the selected TestReport producer.

Before building, measure `cargo test --workspace` wall time in a cold run-owned worktree against ADR-0140's 3600 s bound. Also note that A alone does not make AC-473 reachable, because the upstream blockers stand.

## (d) Step-executor scope for Phase 1: should the missing executor features be built (step-output resolution, check evaluation, per-step agent specs, test selection), should the fix lane be narrowed to activities that can walk, or a hybrid of the two? Which activities does bug-standard select per stage for rk-fix-doctor-test-count, and which of them could walk under each option?

**TAKEN 2026-09-24:** C — hybrid: complete resolver + check/emit/test selection; infrastructure-bound demands cut as Phase-2-owed; per-step agent specs only WITH ADR-0150 c4's step-grain key.

**Settled by record already:** False

**Binding records found:**

- ADR-0136 clause 4 (accepted), verified verbatim: "The reference resolver is OWED" ... "**It is not `s7g`'s**, and no stage in this runbook currently claims it." Clause 3: "`null_out_refs` is not called on the execution path". The resolver must supply an output-binding model, prior-step name resolution per ADR-0103, ADR-0055's previous-iteration rule, and `for_each` / `repeat.until` semantics.
- D21-11 (spikes/DECISIONS-2026-09-21.md, ADR-0136's acceptance), verified: "It does **not** license a partial resolver" and "**no stage in this runbook claims it**". No runbook stage file claims it at 4c282d7: grep of runbook/phase-1/s8-exit-criterion.md, halts.md, INDEX.md and 00-preamble.md for 'resolver' finds no claim.
- ADR-0142 clauses 3-4 (accepted), verified: "The other **46 keep ADR-0136's refusal**"; "`check` steps are NOT repaired by this record, and the `check` arm continues to refuse."
- ADR-0143 clauses 2 and 5 (accepted), verified: "A second *runnable* agent step refuses"; "Per-step specs are OWED ... It contradicts `bind_nodes`' current *one spec per node* contract and ADR-0099:61's field set, so it needs its own record".
- ADR-0150 clause 4 (accepted), verified: "The repair is an idempotency key with a STEP component, and no record currently routes it" ... "Until a record does, no partial mechanism is built." The clause states that ADR-0143 c5 would not close this residual. MISSED CONSEQUENCE: under ADR-0150's per-activation commit, a second agent step made runnable by per-step specs would run inside an activation that has already committed. On re-entry after a crash it would be skipped, so per-step specs enlarge this named residual.
- ADR-0144 (accepted, MISSED): "A step that fails **after** its activation's dispatch has committed **refuses by name**, and the commit **stands**" (`ReplayError::StepFailedAfterCommit`). This governs what a false `check` after the agent step does. It does not quietly end the walk as `Failed`.
- ADR-0148 clauses 1 and 4 (accepted), verified: "`toolchain:test` refuses"; "This is reversed by a resolver, not by a revert". Its title: all 28 selections are `affected` (17), `${…}` (6), `artifact:…` (4) or `e2e` (1).
- ADR-0135 clause 1/2 plus D21-10 (accepted), verified: "adds no port method — the count stays six, as Checkpoint 7a asserts". MISSED: D21-10 continues "That the pack carries no verb registry at all, against design §4.5's five-registry row, is recorded as a gap and deliberately not filled." The design's own route for unmapped verbs is a verb registry, not port methods.
- ADR-0156 (accepted): clause 4 says "`0 of 102` shipped activities completing a committing walk stands unchanged". Its Context lists SIX blockers (0-5), not two: (0) the cards have no `permission_mode` (exit 3, blocks EVERY agent dispatch); (1) `.redkiln/schemas/` does not exist (61 kinds); (2) "`.redkiln/processes/` is empty", so `question = "review.blocking"` resolves to nothing; (3) `verdict_record` is unmapped and "`adversarial-review`'s `toolchain:shallow_clone` is unmapped too"; (4) no item is at stage review; (5) "`${review.outputs}` does not resolve".
- ADR-0162 (accepted): builds the review call site over a fixture corpus. It declines "mapping `toolchain:verdict_record` under this stage's scope", so routing verdict_record takes a new record.
- ADR-0147 clause 3 (accepted), verified: "**0 of 102** shipped activities complete a committing walk". Option (d): "a carried finding that is not written down is a finding that is lost".
- ADR-0128 clause 3 (accepted), verified: "`fix` → the `bug-standard` pack".
- ADR-0114 clause 3 (accepted), verified: promotion "only with a ledger entry: it must have run at least once". The same clause adds "the promotion route is a one-run, one-activity trial the operator declares, not a pack reference" (AC-110 unchanged). Rejected option: "tag a Phase-1 slice of ~40 ... Rejected again".
- ADR-0024 (accepted, MISSED): "Admissibility lives in the activity, requirement lives in the pack". Narrowing bug-standard's baselines is the pack's own domain. bug-standard.toml already carries dated precedent: "Step 7e (2026-09-21) -- DROPPED: [[stage.release.require]] when = 'data_migration'".
- ADR-0093 decision 2 and AC-520 (MISSED as binding): `change` stays `source = "derived"` and has a record at pin 47faec9f (0.72 [0.54, 0.85], 29 positives). facets.toml declares `deriver = "R-change-extension-and-numstat-marker"`, `deriver_class = "structural"` and `deriver_record = "results/p1-arm-d-panel-scored.json"`. AC-520's load gate is therefore already satisfied for `change`; wiring the deriver needs a call site, not a new scored record.
- AC-473 (register), verified: "whose event log contains every lane stage from intake to a landed commit" and "the toolchain stage ran through `rk-adapter-toolchain-rust` behind the `Toolchain` port". s8-exit-criterion.md (1) reads it as "the verify stage's toolchain is rk-adapter-toolchain-rust".

**Measured state:**

MEASURED at 4c282d7 with read-only tools: ADR text, the D21 ledger, the AC register, s8-exit-criterion.md, .redkiln/packs/bug-standard.toml, .redkiln/facets.toml, .redkiln/items/rk-fix-doctor-test-count.toml, the tags of the 17 relevant catalog TOMLs, step4.rs `tags_match`, and rk-cli verbs/run.rs `build_item`. NOT measured: the per-stage selection and the walk verdicts. Those come from a hand application of step4 `judge` and a Python simulation (scratchpad walk_d.py, present on disk but not re-run here). They are claims until a `redkiln run` confirms them.

VERIFIED BY READING:
- bug-standard stage rigor is triage mandatory, discovery forbidden, analysis recommended, specification/design/planning optional, implement/verify/review mandatory, release recommended, operate forbidden, closeout recommended. Release and closeout are therefore RECOMMENDED, not mandatory.
- The verify baseline is `[\"TestReport\", \"ConfirmationResult\", \"SmokeResult\"]`, release is `[\"BuildArtifact\", \"ReleaseRecord\"]`, and closeout is `[\"DebtRegisterEntry\", \"ClosureRecord\"]`.
- The item carries `cos, has_repro, criticality, blast_radius, size`, all `source = \"human\"` in facets.toml. It does not carry `change` or `toolchain`, both `source = \"derived\"`.
- build_item nulls every non-kernel facet the record omits. It ACCEPTS a `[facets]` value for any non-kernel facet, derived ones included, because only `kind` and `source = kernel` are excluded. So hand-declaring `change = [\"text\"]` is mechanically supported today. It would, however, carry a human value on a derived-class facet (ADR-0089/0093 provenance), and I found no record that licenses that.
- `tags_match` returns false for a null axis (\"An axis the item does not bind cannot satisfy a constraint on it\").
- Tags confirm the packet's exclusions: minimal-repro, root-cause and tdd-red-green-refactor are tagged `change:text`, and sca-scan is also tagged `change:text`. None of the 15 is tagged `toolchain:*`, so a null `toolchain` does not exclude them.
- MAPPED_VERBS is build, change_summary, lint and test (ADR-0156 blocker 3).

SELECTION (claim, from the packet's hand application; tag facts verified). With the item as shipped (`change` null):
- triage: 0 activities (ADR-0156).
- analysis: ReproCase has no producer (Unsatisfiable); RootCause goes to differential-debugging.
- specification: fix-spec.
- implement: Changeset and ImplReport are Unsatisfiable; ChangeSummary is an abstention gate.
- verify: TestReport is an abstention gate; then confirmation-test and smoke-test; ScanReport abstains.
- review: adversarial-review.
- release: build-package and rolling-deploy.
- closeout: debt-logging, closure-record and lightweight-retro.
So no Changeset can be produced until `change` has a value.

With `change = [\"text\"]`, 15 distinct activities are selected: minimal-repro, root-cause, fix-spec, tdd-red-green-refactor, change-summary, equivalence-boundary, confirmation-test, smoke-test, sca-scan, adversarial-review, build-package, rolling-deploy, debt-logging, closure-record and lightweight-retro.

WALKABLE, by executor feature set (claim, from the simulation, which assumes ref-free guards hold):
- Today: 0 of 15. The packet's list named code-review, which is NOT selected; that was an error in the packet. The rest of today's breakdown stands: guard refs, second agent step, `test`, no agent step, check, unmapped verbs.
- Resolver + check + emit binding: 2 of 15 (change-summary, closure-record).
- Adding per-step agent specs: 4 of 15 (adds fix-spec, in the OPTIONAL specification stage, and lightweight-retro, in the RECOMMENDED closeout stage).
- Adding test selection: 9 of 15.
- The remaining 6 refuse on unmapped verbs: smoke, sca_scan, shallow_clone/verdict_record, package, deploy and item_create.

ALSO BLOCKING, UNDER EVERY OPTION (ADR-0156 blockers the packet under-counted):
- (0) The cards have no `permission_mode`, so exit 3 fires before any agent dispatch.
- (1) `.redkiln/schemas/` is absent.
- (2) `.redkiln/processes/` is empty, so adversarial-review's `review.blocking` question resolves to nothing. Routing shallow_clone and verdict_record alone does not make review walk.

PER-STAGE, once all executor features exist (claim):
- analysis, specification and implement: yes.
- verify (mandatory): no, because of smoke and sca_scan.
- review (mandatory): no, because of shallow_clone, verdict_record and the empty processes registry.
- release (recommended): no.
- closeout (recommended): no, because of item_create.

**Options:**

### A. Build the executor out in full, and map the verbs the fix lane needs

Build the whole reference resolver, which D21-11 requires to be complete rather than partial: output binding, prior-step name resolution, ADR-0055 previous-iteration binding, and `for_each`/`repeat.until`. On top of it:
- check evaluation;
- emit `artifacts`/`evidence` binding;
- per-step agent specs, TOGETHER with ADR-0150 c4's step-grain idempotency key, because per-step specs without it enlarge that residual and 0150 says "no partial mechanism is built";
- `inputs.selection` resolution for `toolchain:test`.
Then route the 7 remaining verbs (smoke, sca_scan, shallow_clone, verdict_record, package, deploy, item_create). Prefer the design §4.5 verb registry that D21-10 records as a gap over new port methods. Populate `.redkiln/processes/` for `review.blocking`, and wire the declared `change` deriver into build_item.

- *Record form:* Several new ADRs from ADR-0164, each a supersession where it touches a Decision sentence:
- resolver/output binding (discharges ADR-0136 c4 and ADR-0142 c4);
- test selection (supersedes ADR-0148, as its clause 4 foresees);
- per-step agent specs PLUS the step-grain commit key (discharges ADR-0143 c5 and ADR-0150 c4; supersedes ADR-0099's spec field set in part);
- the verb registry/map (supersedes ADR-0135 c1 in part; if it adds port methods it also reverses D21-10's "the count stays six" and amends Checkpoint 7a; a registry would not);
- the deriver call site.
A runbook stage insertion is also needed, since no stage claims the resolver (D21-11).
- *Work:* Code:
- rk-kernel runtime: resolver, iteration, check, emit binding, a per-step commit grain, and step-keyed idempotency in `is_committed`.
- rk-cli bind_nodes: one spec per agent step.
- A verb registry or port methods for 7 verbs.
- rk-adapter-toolchain-rust: package, smoke, sca_scan.
- rk-git: shallow_clone (ADR-0098's retained clone).
- verdict_record via review::confirm (ADR-0162), and item_create via the store.
- deploy has no honest target in a Cargo library repo.
Pack: `.redkiln/processes/` entries, cards `permission_mode`, `.redkiln/schemas/` (61 kinds), `model` rows.
- *Size:* XL: several stages' worth. ADR-0136's drivers call the resolver alone "genuinely larger than `s7g`", and the step-grain commit key adds to that.
- *Risks:* - Phase 1 exit moves far out.
- `deploy` still needs a scope answer, so A is not pure.
- Changing the commit grain collides with AC-013, ADR-0138, ADR-0143 and ADR-0144, and risks criterion (3)'s kill_and_resume_n_step.
- New port methods reopen Checkpoint 7a.
- The simulation assumes ref-free guards hold.
- *Citations:* ADR-0136 clause 4: "The reference resolver is OWED"; D21-11: "It does **not** license a partial resolver"; ADR-0143 clause 5: "Per-step specs are OWED"; ADR-0150 clause 4: "no record currently routes it. Until a record does, no partial mechanism is built."; ADR-0148 clause 4: "This is reversed by a resolver, not by a revert"; D21-10: "adds no port method — the count stays six" and "no verb registry at all, against design §4.5's five-registry row, is recorded as a gap"; ADR-0156 Context blocker (2): "`.redkiln/processes/` is empty"

### B. Narrow: author thin, single-agent bug-lane activities that walk on today's executor, and point bug-standard at them

Leave the executor as ADR-0147 closed it. For each demanded stage, author one activity shaped `[agent, (tool build|lint)?, emit]`: no check, no reference-bearing guard, one agent step, no unmapped verb. Rank these below the current kind:bug producers, or narrow bug-standard's baselines to the kinds they provide. Under ADR-0024 that narrowing is the pack's own domain. About 7 new activities walk; the 102 shipped activities stay at 0.

- *Record form:* A new ADR (ADR-0164) scoping the Phase-1 fix lane. If the thin activities live in a separate pack, it supersedes ADR-0128 c3 in part. The new activities enter by ADR-0114 c3's declared one-run, one-activity trial ("not a pack reference", so AC-110 is not tripped), each cited by `curation.decision`. The ADR must say why this is not ADR-0114's twice-rejected "Phase-1 slice".
- *Work:* No kernel work. Pack work:
- about 7 new extended-catalog TOMLs, then a trial run and promotion for each;
- bug-standard baseline edits after the `redkiln-pack-v0` tag;
- a rank-lint re-run;
- a value for the item's `change` facet (see the risks).
The registry fills are still required: cards `permission_mode`, `.redkiln/schemas/`, `model` rows. `.redkiln/processes/` is needed too if any thin review activity asks a process question.
- *Size:* M
- *Risks:* - Halt 8-shaped vacuity: criterion (1) passes over a lane that runs none of the shipped method. That is the case D21's "A fixture proves the wiring, not the catalog" and ADR-0147 c3 warn against.
- Verify cannot run `test` (ADR-0148), so AC-473's toolchain half rests on build/lint only.
- ADR-0136 c4, 0142 c4, 0143 c5 and 0150 c4 all stay OWED and unowned.
- A hand-declared `change` puts a human value on a `source = "derived"` facet. build_item accepts it, but no record licenses that provenance.
- The promotion trials must run BEFORE the pack references the activities. ADR-0114 c3 provides that route, so this is a sequencing cost, not a blocker.
- *Citations:* ADR-0114 clause 3: "the promotion route is a one-run, one-activity trial the operator declares, not a pack reference"; ADR-0114 rejected option: "tag a Phase-1 slice of ~40 ... Rejected again"; ADR-0024: "Admissibility lives in the activity, requirement lives in the pack"; ADR-0128 clause 3: "`fix` → the `bug-standard` pack"; ADR-0147 clause 3: "0 of 102 shipped activities complete a committing walk"; ADR-0148 clause 1: "`toolchain:test` refuses"; AC-473: "every lane stage from intake to a landed commit"

### C. Hybrid: build the executor core the records name as OWED; narrow only the demands whose producers need infrastructure this repo lacks

Build the complete resolver (D21-11): output binding, iteration, check evaluation, emit binding and test-selection resolution. On that core alone, 7 of the 15 activities walk in the simulation: minimal-repro, root-cause, tdd-red-green-refactor, change-summary, equivalence-boundary, confirmation-test and closure-record.

Per-step agent specs add only fix-spec (OPTIONAL specification) and lightweight-retro (RECOMMENDED closeout). They also must NOT land without ADR-0150 c4's step-grain idempotency key. So C builds them only if a run shows the item actually enters those stages at its rigor; otherwise the ADR records them as Phase-2-owed, by name.

For review (mandatory), route adversarial-review's `shallow_clone` and `verdict_record` onto existing mechanisms: ADR-0098's retained review clone and ADR-0162's review::confirm. Do this through the verb registry that D21-10 records as a design §4.5 gap, not through new port methods, so Checkpoint 7a's six stands. Also populate `.redkiln/processes/` for `review.blocking` (ADR-0156 blocker 2).

Narrow bug-standard under ADR-0024, following 7e's dated-DROPPED precedent. Drop verify's SmokeResult, the require row that demands ScanReport, release's BuildArtifact and ReleaseRecord, and closeout's DebtRegisterEntry. Name each drop and mark it Phase-2-owed.

Give `change` a value by wiring the declared deriver, whose AC-520 record already exists at 47faec9f, into build_item.

- *Record form:* - ADR-0164: the resolver, check, emit binding and test selection. It supersedes ADR-0148 and discharges ADR-0136 c4 and ADR-0142 c4.
- A second ADR: per-step agent specs AND the step-grain commit key together, discharging ADR-0143 c5 and ADR-0150 c4 and superseding ADR-0099's field set in part. Or, if not built, a clause in ADR-0164 that names them as Phase-2-owed. "Explicitly declining" 0150 c4 while building per-step specs is not available, because 0150 forbids a partial mechanism.
- A third ADR: the verb registry rows for shallow_clone and verdict_record, superseding ADR-0135 c1 in part and stating that the port count stays six.
- A fourth ADR, or a section of one: the bug-standard narrowing after the v0 tag, each dropped kind named.
- The deriver call-site record.
- A runbook stage inserted ahead of Checkpoint 8, since D21-11 says no stage claims the resolver.
- *Work:* Code:
- the resolver core (resolver, iteration, check, emit, test selection);
- two registry verb routes onto rk-git and review::confirm;
- deriver wiring in build_item;
- per-step specs plus the step-grain key ONLY if the specification or closeout stages are entered.
Pack:
- bug-standard baseline and require drops, dated;
- `.redkiln/processes/` entry or entries;
- the registry fills that ADR-0156 blockers 0 and 1 require under every option: cards `permission_mode`, `.redkiln/schemas/`, `model` rows.
No new port methods and no new catalog activities.
- *Size:* L. Smaller than A by the 5 infrastructure verbs, the port change and, if deferred, the commit-grain work. The resolver dominates.
- *Risks:* - Cutting pack scope after the v0 tag repeats ADR-0147 option (d)'s silent-scope pattern unless every dropped demand is named and marked OWED.
- If specification or closeout is entered at the item's rigor, the per-step-spec and step-grain work comes back, with its AC-013 and criterion (3) risk.
- A false `check` after the committed dispatch refuses under ADR-0144 (`StepFailedAfterCommit`), so tdd's `red-check` and equivalence-boundary's `green` may need on_false loop-back (ADR-0031/0044). Settle that in the resolver ADR.
- The simulation assumed ref-free guards hold. The 7-of-15 (or 9-of-15) count is a claim until a run confirms it.
- *Citations:* ADR-0136 clause 4: "The reference resolver is OWED"; D21-11: "It does **not** license a partial resolver"; ADR-0143 clause 5: "Per-step specs are OWED"; ADR-0150 clause 4: "Until a record does, no partial mechanism is built."; ADR-0144: "refuses by name**, and the commit **stands**"; ADR-0148 clause 4: "This is reversed by a resolver, not by a revert"; ADR-0156 Context blockers (2) "`.redkiln/processes/` is empty", (3) "`toolchain:verdict_record` is not a mapped verb", (5) "`${review.outputs}` does not resolve"; ADR-0162: declines "mapping `toolchain:verdict_record` under this stage's scope"; D21-10: "no verb registry at all, against design §4.5's five-registry row, is recorded as a gap"; ADR-0024: "requirement lives in the pack"; ADR-0093 decision 2 / AC-520: `change` record at 47faec9f; ADR-0147 option (d): "a carried finding that is not written down is a finding that is lost"

**Recommended:** C, the hybrid, with the corrections above:
- Build the complete resolver with check, emit and test selection.
- Route shallow_clone and verdict_record through verb-registry rows onto the existing review clone and review::confirm, and populate `.redkiln/processes/` for `review.blocking`.
- Wire the `change` deriver.
- Cut only the infrastructure-bound demands (SmokeResult, the ScanReport require row, BuildArtifact, ReleaseRecord, DebtRegisterEntry), each named and marked Phase-2-owed.
- Build per-step agent specs only together with ADR-0150 c4's step-grain key, and only if a run shows the item enters specification or closeout. Otherwise name them as owed.
- Whatever the option, land ADR-0156 blockers 0-1 first: cards `permission_mode` and `.redkiln/schemas/`.

**Why:** DEFECTS FOUND IN THE PACKET:
1. It missed ADR-0144, which is binding. A step failing after the committed dispatch refuses by name and the commit stands. The packet's C risk said a false check \"ends the walk as `Failed`\"; it actually refuses with `StepFailedAfterCommit`.
2. It misread ADR-0150 c4 as optional. C offered \"routing or explicitly declining\" it. But 0150 says \"no partial mechanism is built\", and per-step agent specs under the per-activation commit would make a second agent step run inside an already-committed activation, where a crash skips it on re-entry. Per-step specs must land WITH the step-grain key, or not at all.
3. It under-counted ADR-0156's blockers. It cited (3) and (5), but blockers (0) `permission_mode`, (1) `.redkiln/schemas/` and (2) \"`.redkiln/processes/` is empty\" bind under every option. Because of (2), routing shallow_clone and verdict_record alone does not make adversarial-review walk.
4. It dropped half of D21-10. The same paragraph records the missing verb registry \"against design §4.5's five-registry row\" as a gap. Routing verbs through that registry is the design's route and leaves Checkpoint 7a's six intact. A's \"new port methods\" and C's \"kernel/git steps\" both overlooked it.
5. It missed ADR-0024 (\"requirement lives in the pack\") and 7e's dated-DROPPED precedent in bug-standard.toml. Both support C's narrowing.
6. It missed ADR-0093 d2 and AC-520. The `change` deriver's scored record already exists at 47faec9f, and facets.toml names deriver, class and record. C's risk implied the record work was still owed; only a call site is.
7. The measured state was wrong about hand-declaration. build_item ACCEPTS a derived facet from the item's `[facets]` table; only kind and kernel facets are excluded. So hand-declaring `change` is mechanically possible today. The objection is provenance, not a missing path.
8. The recommendation and \"why\" claimed C runs the shipped method \"through every mandatory stage (triage, implement, verify, review)\". That is false on two counts: triage selects 0 activities (ADR-0156), and C cuts verify's SmokeResult. Release and closeout are also RECOMMENDED in bug-standard.toml, not mandatory, which the per-stage table did not say.
9. The \"Today\" walk list named code-review, which the packet's own selection excludes.
10. B's AC-110 \"sequencing trap\" is already answered by ADR-0114 c3's own text: the trial is \"not a pack reference\".
11. The recommendation was not the least-change route. Per-step specs unlock only fix-spec (optional stage) and lightweight-retro (recommended stage), yet they carry the largest risk (the commit grain, criterion (3)). Deferring them unless a run shows those stages are entered is smaller.

WHAT SURVIVES: every quoted phrase from ADR-0136, 0142, 0143, 0147, 0148, 0150, 0156, 0128, 0114, D21-10, D21-11 and AC-473 exists as quoted. The tag facts behind the selection claims are verified.

WHY C, AS CORRECTED: no accepted record settles this. ADR-0136 c4, ADR-0142 c4, ADR-0143 c5 and ADR-0150 c4 each say the work is OWED, and D21-11 says \"no stage in this runbook claims it\". So an operator decision is owed, and it takes new ADRs plus a runbook stage insertion, not an amendment.
- B certifies thin fixtures-as-method, which is the vacuity ADR-0147 c3 and D21's \"A fixture proves the wiring, not the catalog\" warn against. It also cannot execute `test`.
- A needs C's pack cut anyway, because deploy, smoke and friends have no honest target here, and it costs more.
- C discharges the resolver debt the 0-of-102 finding traces to, and keeps every cut named.

Independent of the option chosen, two things come first:
- ADR-0156 blockers 0-1 (cards `permission_mode`, `.redkiln/schemas/`). Every agent dispatch stops at exit 3 without them.
- A value for `change`. With it null, implement has no admissible Changeset producer.

CAVEAT: the selection and walk counts are a reading plus a simulation (walk_d.py), not a run. Confirm them with `redkiln run` once the build lock is free.

## (e) Does Phase-1 exit criterion (1) ("the event log contains EVERY lane stage from intake to a landed commit") need the Slice-Verdict: axis, and the tier-1 store writer, to be delivered? If so, what gives the slice its identity, and what record makes that decision?

**TAKEN 2026-09-24:** A — BOTH now: Slice-Verdict (declared `slice`, sealed on both outcomes) AND the tier-1 EventStore writer (port change under ADR-0003), against the packet's recommendation C.

**Settled by record already:** False

**Binding records found:**

- spikes/DECISIONS-2026-09-21.md D21-3 (operator): 'Phase 1 is finished as specified'. Rejected: 're-scoping Phase 1 to exclude lane execution; narrowing the exit to one stage; re-planning the phase'. MISSED BY THE PACKET: D21-3 treats 'TrailerKey::Story and ::SliceVerdict occur only in the parser' as a defect to build out. It also binds 'Three source lints … each discharged by the landing that makes it pass'. runbook/phase-1/s7f-conformance-and-records.md assigns the trailer lint to 's7h'. So the question of whether Slice-Verdict ships in Phase 1 is already settled by D21-3, ADR-0109 decision 3 and design §5. It does not rest on a reading of criterion (1).
- ADR-0160 (accepted, D21-36). Decision 4: 'No producer is written from an invented slice identity'. Decision 6: 'declare the slice on the item record (or decide that a fix lane has exactly one, and say so), then a `slice_sealed` producer at the review stage's boundary, then the trailer through the same `commit_with_trailers` path'. Evidence: 'option 2 is the route, not the decision'. Forbids: 'Emitting a `Slice-Verdict:` trailer whose `<slice>` is the item id, the project, the stage, the run id, or any value not declared as a slice'. It also forbids 'Removing … `#[ignore]` before a **reachable** `Slice-Verdict:` producer exists'.
- ADR-0162 DECISION 2 (a Decision sentence, missed by the packet, which cited only the forbids bullet): 'The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that. Checkpoint 7h guards (6) and (7) require the ARMED lint to stay ARMED … `0160` stands whole'. It is scoped to 0162's own work (§7h.3), so a later record does not supersede it. A new record must still cite it and must say that it re-states those guards. Forbids: 'Forbids writing a `slice_sealed` producer, declaring a `<slice>` … under this record's scope'. Consequences, the reachability precedent: 'Exit::FindingsBlock gains a producer a real `redkiln run` reaches', where that run is over a FIXTURE corpus.
- ADR-0161 decisions 1–2 (MISSED BY THE PACKET). The advance fires 'only when the successor names a stage the fold has not already entered'. 'A stage that failed has not earned a successor. The lane loop returns a non-`Exit::Success` status immediately, before reading `advanced`.' So a 'changes-requested' verdict (Exit::FindingsBlock, exit 4) crosses NO boundary.
- ADR-0158 (accepted, D21-34): the tier-1 tables have zero writers, and AC-160..AC-163 are 'vacuous on every axis' (ADR-0158's wording; see the defect on AC-160/AC-161). Decision 3: 'No document, record or checkpoint may state that AC-162 is met'. Decision 4 route: 'a tier-1 write method on `EventStore` (with its fake and conformance row), called from the same place the trailer commit is made'. Its forbids bullet 'Adding a tier-1 write method to `EventStore` under `s7h`' is scoped to s7h. The record sets no phase for the route.
- ADR-0109 decision 2: 'Tier 1 (stage transitions, gate opened/answered, verdicts, slice seals) is written to git first … and a failed git write leaves zero store rows (AC-160–AC-163)'. Decision 3: 'Trailers carry three axes: `Story:`, `Slice-Verdict:`, `Gate:`. Resume from `Story:` alone is forbidden'.
- ADR-0105 clause 9: AC-184's 'tier-1 projection = the tier-1 rows whose anchor is reachable from the clone's refs'. This is the definition that exit criterion (2) cites.
- ADR-0155 decision / Consequences (a): 'For a lane crossing N boundaries, N−1 stage changes carry no trailer commit at all'.
- ADR-0156 (superseded in part by ADR-0162; clause 4's six blockers stand): '`0 of 102` shipped activities completing a committing walk stands unchanged'.
- ADR-0159: '<project>' 'is declared, in a new `[project] name` key, and a run that would mint a trailer without one REFUSES by name'. This is the pattern for <slice>.
- ADR-0092 rule 1: 'An erratum corrects an evidence line … and leaves every sentence of `## Decision` and every bullet of `**What this forbids**` standing as written. A correction that would move either is a supersession'. Rule 2 fixes the register's state labels as 'binding / PROVISIONAL / CONDITIONAL / HUMAN GATE / retired'.
- ADR-0003: 'The kernel exposes exactly five ports'. Adding a method changes the surface of a port, not the count of ports.
- ADR-0002: 'gated solely on Redkiln-RS running its own fix lane end to end (self-hosting)'. It is read together with ADR-0094 decision 3 and AC-521 (both lanes).
- AC-473: 'whose event log contains every lane stage from intake to a landed commit … the claim is refused if any stage is missing from the log'. Verified by 'a release check over the event log (stage-completeness assertion …)'. AC-521 is the story-lane twin: 'contains every story-lane stage'.
- AC-162 (register): 'the trailer commit exists in git before any store row for that transition is readable'. It is verified by a fault-injection test plus 'a reachability lint that the tier-1 store writer is callable only from the commit path'. AC-160 is a trybuild tier-annotation fixture, and AC-161 is the delete-the-database next-action equality. Neither depends on t1_* rows.
- Design §5 event table: '`slice_sealed` / verdict | 1 (`Slice-Verdict:` trailer) | slice, verdict | the lane's review stage'. §5.4 grammar: '`<project> "/" <slice> SP <verdict>` … latest wins, commits folded oldest-first'.
- ERRATA E-0160-1: 'exactly one construction' is 'overstated as worded'. There is one `Event::new` site (rebuild.rs). The variant is also constructed in `pub const ALL` in src and at two test sites, and it is rehydrated on every query.
- runbook/00-preamble.md: 'A checkpoint that changed after it passed is provisional'. runbook/phase-1/s7f-conformance-and-records.md greps each armed #[ignore] reason for 'discharged by (s7g|s7h)', because 'the reason IS the debt record'.

**Measured state:**

Measured with read-only tools at 4c282d7 (grep/sed/git). None of this was taken from a tracker row.
(a) MEASURED. Outside tests/, `SliceSealed` occurs 9 times. There are 5 in rk-types/src/event_kind.rs (declaration, ALL, as_str, Tier::One, trailer_axis), 3 match arms (fsm.rs, breaker.rs, memo.rs), and one `Event::new` construction in crates/rk-cli/src/doctor/rebuild.rs, which is the rebuild path. No run path produces it. Per E-0160-1, the `ALL` literal is also a construction in src, but it is an enumeration, not an emission.
(b) MEASURED. Outside tests, `TrailerKey::SliceVerdict` occurs only in crates/rk-git/src/trailer/grammar.rs (the parser). Nothing emits it.
(c) MEASURED. `insert into t1_` (case-insensitive) outside tests returns 0, and no `t1_` occurrence appears in any crates/**/src .rs file. crates/rk-ports/src/event_store.rs declares only `fn append`, `fn query`, `fn snapshot` and `fn rebuild`.
(d) MEASURED. crates/rk-kernel/tests/it/conformance.rs still carries `#[ignore = "ARMED: Slice-Verdict has 0 producers; Gate and Story are both minted as of s7h.2; discharged by s7h"]`. The tail 'discharged by s7h' is stale against ADR-0160. It is a SOURCE string, not a record. Checkpoint 7f greps every armed reason for `discharged by (s7g|s7h)`, and Checkpoint 7h guard (6) greps 'ignored, ARMED: Slice-Verdict has 0 producers'. Rewording the string therefore moves two checkpoints. Discharging the lint deletes the string.
(e) MEASURED. `ItemRecord` (crates/rk-cli/src/verbs/run.rs) has `id`, `kind`, `stage` and `facets`, and no slice.
(f) MEASURED. Both shipped packs declare `review = "mandatory"` and include "review" in `stage_order` (.redkiln/packs/bug-standard.toml, story.toml). Design §5 names 'the lane's review stage' as the producer of `slice_sealed`.
(g) MEASURED plus INFERENCE. fsm.rs `RunState` holds one `sealed: Option<String>`, and `terminal()` documents '1. a `slice_sealed` → [`TerminalState::Sealed`]'. Reading this as 'the kernel treats one slice per item' is an inference. Design §5.4's 'latest wins, commits folded oldest-first' is equally consistent with it. Nothing DECLARES the cardinality.
(h) MEASURED. The criterion (1) instrument (runbook/phase-1/s8-exit-criterion.md) runs `redkiln explain` and a `git log rk-run/$FIX_ITEM --grep='^(Story|Slice-Verdict|Gate):'` and READS them: 'NO stage-line pattern is pinned'. Its comment says 'the stage sequence is read from the event log's tier-1 trailer commits'. But under ADR-0155 a lane carries ONE Story: trailer, so trailers can never enumerate every stage under any option. A Slice-Verdict evidences at most the review stage. 'Every lane stage' therefore has to be read from the journal (explain) whatever is chosen. Criterion (1)'s text alone does not force Slice-Verdict. D21-3, ADR-0109 decision 3 and design §5 do.
(i) MEASURED. No exit criterion's instrument reads t1_* tables. `rebuild_differential` (crates/rk-kernel/tests/it/projection.rs) projects by `e.kind.tier() == Tier::One` over events. However, criterion (2) cites ADR-0105 clause 9, whose projection is defined as 'the tier-1 rows whose anchor is reachable from the clone's refs'. Only anchored t1 rows satisfy that definition, so the current test is a kind-filter proxy for it. The tracker says criterion (2) passes. I did not re-run it.
(j) MEASURED. ADR-0161 decision 2 means Exit::FindingsBlock (a 'changes-requested' outcome) returns before `advanced` is read, so no stage boundary is crossed on the rejecting path.
(k) Whichever option is chosen, the shipped review stage stays unreachable (ADR-0156 clause 4's six blockers, carried by ADR-0162). ADR-0162 set the precedent that a real `redkiln run` over a fixture corpus counts as reaching a producer. It forbids any claim that the shipped review stage is reachable.
(l) Tracker (not re-measured): Step 7i CLOSED at 6907312, and s8 is next. No ADR-0164 file exists.

**Options:**

### A. Build all of it: a declared slice, a seal producer and the Slice-Verdict trailer, plus a tier-1 EventStore writer on all tier-1 kinds

Deliver ADR-0160's route and ADR-0158's route before Checkpoint 8. The slice is declared on the item record, following ADR-0159, as its own key (e.g. `slice`). It is refused by name when absent and is never defaulted from, or equal to, the item id. The fix lane declares exactly one slice per item, and the story lane declares its slice. review_pass seals on BOTH outcomes. 'approved' is written when the confirmed block set is empty, on the advance path. 'changes-requested' is written on Exit::FindingsBlock, on the no-advance path that ADR-0161 decision 2 returns from, where no boundary is crossed. The trailer is written first through commit_with_trailers, then the row. EventStore also gains a tier-1 method that takes the returned anchor_sha. It is called for stage_transition, Gate:, Story: and the seal, and it has a reachability lint limiting callers to the commit path (AC-162's third instrument).

- *Record form:* Two new ADRs. ADR-0164 declares the <slice> identity and places the seal producer on both review outcomes, as execution of ADR-0160 decision 6. It supersedes nothing ONLY IF the declared slice may not be the item id. If it permits slice == item id, it supersedes ADR-0160's first forbids bullet in part (ADR-0092 rule 1). It cites ADR-0162 decision 2 as scoped to §7h.3. ADR-0165 adds a tier-1 write method to the EventStore port, with its fake, a conformance row, the AC-162 fault-injection test and the reachability lint, as execution of ADR-0158 decision 4. It changes a port's surface, not ADR-0003's count. Also needed: ledger entries; dated amendments to Checkpoint 7h guards (6)/(7) and to Checkpoint 7f's armed-lint count and 'discharged by (s7g|s7h)' grep. Those checkpoints become provisional under the preamble's 'changed after it passed' rule. No erratum for the ignore reason is needed: it is source, not a record, and it disappears in the discharging commit.
- *Work:* rk-cli: `slice` on ItemRecord with a by-name refusal, added to the shipped items and the fixture items. A seal emitter on both review_pass outcomes in run.rs. rk-ports: a new EventStore method. rk-store: INSERTs into t1_stage_transition, t1_gate and t1_slice_seal. The rk-store fake gets the same method. Four call sites. A fault-injection test with a git backend that fails the commit, asserting zero t1 rows. The reachability lint. A behavioural test: a real `redkiln run` over ADR-0162's fixture corpus, with each verdict on its own fixture, must write a Slice-Verdict that git's trailer parser reads on rk-run/<item>. Remove the lint's #[ignore] in that same commit. Re-run Checkpoints 7f/7g/7h/7i.
- *Size:* Large: two stages (seal/slice, then tier-1 port). Roughly 1.5–2x s7h's scope, plus two ADRs and adversarial passes.
- *Risks:* This has the widest change surface: a late port-surface change, and the AC-162 ordering has never been exercised. It does, however, make criterion (2)'s ADR-0105 clause-9 projection computable from anchored rows rather than from a kind-filter proxy, which is a real Phase-1 benefit that the packet omitted. Checkpoint 7h guards (6)/(7) and 7f's armed-lint greps turn RED by design and must be re-stated. As in every option, the shipped review stage stays unreachable (ADR-0156 blockers), so the seal is proven over a fixture only. That meets ADR-0160's 'reachable' bar on ADR-0162's precedent. It does not license any shipped-reachability claim.
- *Citations:* references/adr/0160-the-slice-verdict-axis-needs-a-slice-identity-nobody-declares.md: 'declare the slice on the item record (or decide that a fix lane has exactly one, and say so)'; references/adr/0158-the-tier-1-store-layer-has-no-writer-on-any-axis.md: 'a tier-1 write method on `EventStore` (with its fake and conformance row), called from the same place the trailer commit is made'; references/adr/0161-…md: 'A stage that failed has not earned a successor'; references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md clause 9: 'the tier-1 rows whose anchor is reachable from the clone's refs'; references/2026-09-08-acceptance-criteria.md AC-162: 'a reachability lint that the tier-1 store writer is callable only from the commit path'; spikes/DECISIONS-2026-09-21.md D21-3: 'Phase 1 is finished as specified'

### B. Rule that Story:, Gate: and stage_transition cover 'every lane stage', and defer Slice-Verdict and the tier-1 writer to Phase 2

Read criterion (1) as satisfied by a stage_transition into each non-forbidden stage in the journal that explain reads, plus the one Story: trailer and any Gate: trailers. Slice-Verdict and the tier-1 writer move to Phase 2, and Phase 1 exits with resume from Story: alone.

- *Record form:* New ADR-0164 plus a NEW OPERATOR DECISION reversing D21-3 in part. D21-3 rejected 'narrowing the exit to one stage', listed SliceVerdict 'only in the parser' as a defect to build, and bound each armed lint to be 'discharged by the landing that makes it pass'. B also SUPERSEDES ADR-0109 decision 3 in part ('Resume from `Story:` alone is forbidden'), which is a Decision sentence under ADR-0092 rule 1. It needs a dated design §5 amendment, and a dated register note on AC-473/AC-521 defining 'stage-completeness'. The note is for AC-473/AC-521 only, not for AC-160/AC-161, which do not depend on the axis.
- *Work:* Almost none: records, a register note and a design note. The lint stays ARMED across Checkpoint 8, and its reason string must be re-worded, which moves the 7f/7h reason greps.
- *Size:* 
- *Risks:* It narrows the milestone that ADR-0002 gates 'solely' on, and the operator has already rejected that (D21-3). Review is mandatory in both packs, and design §5 defines its output as a seal. Phase-2 stories would be authored under a resume that loses F12's 20.3% rejection history. It does NOT unblock criterion (1): the review stage still has to be reached, and ADR-0156's blockers stand.
- *Citations:* spikes/DECISIONS-2026-09-21.md D21-3: 'Rejected: re-scoping Phase 1 to exclude lane execution; narrowing the exit to one stage; re-planning the phase'; references/adr/0002-clean-break-self-hosting-switch.md: 'gated solely on Redkiln-RS running its own fix lane end to end (self-hosting)'; references/adr/0109-reconcile-at-every-entry-two-tiers-and-three-trailer-axes.md: 'Resume from `Story:` alone is forbidden'; references/2026-09-08-acceptance-criteria.md AC-473: 'the claim is refused if any stage is missing from the log'; .redkiln/packs/bug-standard.toml: 'review = "mandatory"'

### C. Split: deliver Slice-Verdict in Phase 1 (a declared slice, one per fix-lane item, sealed on both review outcomes) and route the tier-1 store writer past Checkpoint 8 with a record

Slice-Verdict is owed in Phase 1 by D21-3, ADR-0109 decision 3 and design §5, not by a reading of criterion (1)'s text. The slice is declared on the item record as its own key, refused by name when absent. The record fixes its value as declared data that may NOT be the item id and is never derived in code. It says in terms that a fix-lane item carries exactly one slice (ADR-0160 decision 6's parenthesis). The story item names its slice. The seal is produced from review_pass's outcome on BOTH paths. 'approved' is written with the advance. 'changes-requested' is written on Exit::FindingsBlock, where ADR-0161 decision 2 returns before any boundary is crossed, so 'at the review stage's boundary' is read as 'at the review stage's exit outcome'. The trailer is written first through commit_with_trailers, then the slice_sealed row, which lands in t2_event: the same residual ADR-0158 names. The tier-1 store writer is recorded as OWED after Checkpoint 8. No exit criterion's instrument reads t1_*, and ADR-0158 bounds it and set no phase. AC-162 carries a dated register note pointing at ADR-0158. No label moves, and AC-160/AC-161 are untouched.

- *Record form:* New ADR-0164 declares the slice identity and places the seal producer on both review outcomes. It is execution of ADR-0160 decision 6 and supersedes nothing, PROVIDED it forbids the slice value equalling the item id (otherwise it supersedes 0160's first forbids bullet in part). It cites ADR-0162 decision 2 and ADR-0161 decision 2. The tier-1 phasing is a clause in 0164, as execution under ADR-0158 decision 4, and changes no Decision sentence. Also needed: a dated in-place register note on AC-162 (not a state-label move; 'vacuous' is not a register label); dated amendments to Checkpoint 7h guards (6)/(7) and to 7f's armed-lint count and reason grep, which become provisional until re-run; and a ledger entry that records the tier-1 deferral explicitly for the operator, since D21-3 says 'finished as specified'. NO erratum: the stale 'discharged by s7h' is a source string, and it is deleted in the commit that removes the #[ignore]. No reversal of D21-3 is needed, because the exit is not narrowed.
- *Work:* rk-cli: an ItemRecord `slice` key plus a by-name refusal (the ADR-0159 pattern), added to the shipped fix/story items and the fixture items. A seal emitter on both review_pass outcomes in run.rs, with the non-advance exit still propagated (ADR-0161 defect 2's lesson). A behavioural test: a real `redkiln run` over ADR-0162's fixture corpus must write a Slice-Verdict that git's own trailer parser reads on `<base>..rk-run/<item>`, with one fixture per verdict branch. Remove the lint's #[ignore] IN THAT SAME COMMIT. That run is ADR-0160's 'reachable' bar, on ADR-0162's precedent, and waiting for a shipped run would leave D21-3's discharge unmet at Checkpoint 8. No rk-ports or rk-store change.
- *Size:* Medium: one stage, about s7h.2's Story: work plus one ADR, and no port change.
- *Risks:* Deciding 'fix-lane item = one slice' is a real identity decision. It must be declared data, and it must not equal the item id, or the record takes a supersession. Deferring tier-1 leaves ADR-0109 decision 2's store half degenerate. It also leaves criterion (2) measured by a Tier::One kind filter rather than by ADR-0105 clause 9's 'tier-1 rows whose anchor is reachable'. A reviewer may hold that criterion (2) is then met only by proxy, and the ledger entry should put that to the operator. The seal is proven over a fixture only: no claim of shipped-review reachability (ADR-0162 forbids it). ADR-0155's N−1 Story: residual remains, and criterion (1)'s 'every lane stage' read still comes from explain's journal, not from trailers.
- *Citations:* references/adr/0160-the-slice-verdict-axis-needs-a-slice-identity-nobody-declares.md: 'or decide that a fix lane has exactly one, and say so'; references/adr/0160-…md: 'Emitting a `Slice-Verdict:` trailer whose `<slice>` is the item id, the project, the stage, the run id, or any value not declared as a slice'; references/adr/0160-…md: 'Removing `conformance_every_trailer_key_has_a_producer`'s `#[ignore]` before a **reachable** `Slice-Verdict:` producer exists'; references/adr/0162-…md decision 2: 'The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that'; references/adr/0161-…md: 'The lane loop returns a non-`Exit::Success` status immediately, **before** reading `advanced`'; references/adr/0158-…md: 'No document, record or checkpoint may state that AC-162 is met'; references/adr/0159-…md title: 'a run that would mint a trailer without one REFUSES by name'; spikes/DECISIONS-2026-09-21.md D21-3: 'each discharged by the landing that makes it pass'; runbook/phase-1/s8-exit-criterion.md: 'NO stage-line pattern is pinned'

**Recommended:** C, corrected: deliver Slice-Verdict in Phase 1. The slice is a declared `slice` key on the item record. It is refused by name when absent and may not equal the item id, and exactly one is declared per fix-lane item. The seal is minted on both review outcomes, including the non-advancing FindingsBlock path. The lint is un-ignored in the commit whose real fixture run reaches the producer. The tier-1 store writer is recorded as owed after Checkpoint 8, as execution under ADR-0158 decision 4, with a register note on AC-162 only. All of this is recorded as new ADR-0164, which supersedes nothing, together with a ledger entry and dated 7f/7h checkpoint amendments. No erratum is needed.

**Why:** DEFECTS FOUND IN THE PACKET
1. A missed binding operator record. D21-3 lists 'TrailerKey::Story and ::SliceVerdict occur only in the parser' as a defect to build out, and binds the three lints to be 'discharged by the landing that makes it pass'. s7f's table assigns the trailer lint to s7h. So Phase-1 delivery of Slice-Verdict is already settled by D21-3, ADR-0109 decision 3 and design §5. The packet derived it from criterion (1)'s text instead.
2. ADR-0162 DECISION 2 was missed ('The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that … `0160` stands whole'). The packet cited only 0162's forbids bullet. The decision is scoped to §7h.3, so it is not a supersession, but ADR-0164 must cite it.
3. ADR-0161 decisions 1–2 were missed. Exit::FindingsBlock returns before `advanced` is read, so a 'changes-requested' seal crosses no boundary. ADR-0160 decision 6's 'at the review stage's boundary' cannot produce the rejecting verdict that F12's 20.3% needs unless the record places the seal on the stage's exit outcome. Options A and C elided this.
4. The register amendment for 'AC-160..AC-163 state labels' is wrong. AC-160 is a trybuild tier-annotation fixture. AC-161 is the delete-the-database next-action equality that criterion (2) itself verifies. Neither depends on t1_* rows. 'Vacuous' and 'Phase 2' are also not register labels (ADR-0092 rule 2 lists binding / PROVISIONAL / CONDITIONAL / HUMAN GATE / retired). Only AC-162 needs a dated note.
5. An 'erratum for the ignore reason' is the wrong instrument. ERRATA.md corrects accepted records (ADR-0092 rule 1), and the `#[ignore = …]` string is source. Checkpoint 7f greps every armed reason for 'discharged by (s7g|s7h)' and 7h guard (6) greps its prefix, so rewording it moves two checkpoints. Under A and C the string is deleted in the discharging commit.
6. The packet attributed 'guards (6)/(7)' to Checkpoint 7f. They are 7h's. 7f's affected lines are the armed-lint count and the reason grep. PIA-035 was also miscited. The applicable rule is the preamble's 'A checkpoint that changed after it passed is provisional'.
7. The lint bar is inconsistent. A's risks and the 'why' require a SHIPPED or 'real' run before un-ignoring. ADR-0160's bar is a 'reachable' producer, and ADR-0162 counts a real `redkiln run` over a fixture as reaching a producer. Requiring a shipped run would leave the lint ARMED at Checkpoint 8, contrary to D21-3.
8. 'Supersedes nothing' was asserted while the packet left 'whether it may equal the id' open. ADR-0160's forbids bullet names 'the item id'. A record that permits equality moves a forbids bullet, which is a supersession in part under ADR-0092 rule 1. The least-change route forbids equality.
9. Measured (h) was one-sided. It is literally true that no instrument reads t1_*. But criterion (2) cites ADR-0105 clause 9's 'tier-1 rows whose anchor is reachable', and the test approximates that with a `Tier::One` kind filter. So deferring the writer is a risk to criterion (2)'s fidelity, not a free choice. It is also A's unstated benefit.
10. The packet did not state that criterion (1)'s instrument reads the stage sequence from trailer commits and explain, with 'NO stage-line pattern is pinned'. Under ADR-0155 only one Story: exists per lane, so trailers cannot enumerate every stage under any option.
11. The E-0160 paraphrase was inaccurate. The erratum also names the `ALL` literal in src and two test sites, not only 'rehydration and match arms'.
12. The claim that the FSM's `sealed: Option<String>` shows the kernel treats one slice per item was presented as measured. It is an inference: §5.4's 'latest wins' fits it equally well.

REASONING (the claims that survived, and the corrections)
- Why Slice-Verdict ships in Phase 1: it is owed by record (D21-3, ADR-0109 decision 3, design §5 'the lane's review stage'), not by criterion (1)'s wording. Review is 'mandatory' in both shipped packs.
- Why not B: B needs an operator reversal of D21-3 and a supersession of ADR-0109 decision 3. It also still leaves criterion (1) blocked on ADR-0156's blockers.
- The tier-1 writer: no exit criterion's instrument reads it, and ADR-0158 routes it without setting a phase. Deferring it therefore changes no Decision sentence. Its cost is criterion (2)'s proxy fidelity, so the ledger entry should put that trade to the operator explicitly. If the operator wants AC-162 real, or clause-9 fidelity, at exit, A is the upgrade: C plus ADR-0165's port method.
- The genuinely free residual is the slice identity, which ADR-0160 leaves open ('option 2 is the route, not the decision'). C settles it as declared data, ADR-0159-style, and forbids the item id.

PRECONDITIONS UNCHANGED UNDER EVERY OPTION
- The shipped review stage remains unreachable (ADR-0156 clause 4). The seal is proven over ADR-0162's fixture corpus and no shipped-reachability claim is made.
- Checkpoint 7h guards (6)/(7) and 7f's armed-lint lines are amended before the seal lands.
- ADR-0155's N−1 Story: residual stands.

## (f) Criterion (1b), AC-521 story lane: which Phase-2 story is the candidate, where IntakeRecord comes from, which stage the story item enters at, what the "authored story" artifact is and where it lands, and where Checkpoint 7i's empty cost case points once the story item starts owning readings.

**TAKEN 2026-09-24:** A — full lane, entered at triage; candidate: harden `spikes/p9/packs/spike.toml`; Codex item retired; 7i empty case re-pointed.

**Settled by record already:** False

**Binding records found:**

- ADR-0011 Decision (accepted): "Phase 1 ships exactly two process packs: `bug-standard` and `story`" (references/adr/0011-phase-1-ships-fix-and-story-lanes-only.md). The packet did not cite this sentence, and it binds the candidate choice. The packet's quoted phrase "`bug-standard` and `story` process packs only" comes from design §10.2's Phase-1 row, not from ADR-0011
- spikes/DECISIONS-2026-09-12.md E7 (operator): "A: exactly two — `bug-standard` and `story`. The other ten stay in `spikes/p9/` as Phase-2 work". The rejected alternative there is "Install twelve, enable two via `[packs] enabled`", which is the same shape as shipping a pack disabled
- ADR-0007 Decision (accepted, superseded_by []): "the Codex adapter ships in Phase 4 (\"reach\"), and only after S-CODEX passes". Its forbids: "No non-Claude adapter ... may be the harness behind a review or verification gate until it has passed E3-shaped checks". E12 weighed only ADR-0094 against design :1905 and never names this Decision sentence
- ADR-0094 d2: "The Harness port's second real adapter (Codex) is Phase 2 and stays behind the CI decision ADR-0010 defers". d3: "(b) at least one recorded `story`-lane run under Redkiln-RS authors a Phase-2 story". d1: "Phase 1 ships four real adapters"
- ERRATA E-0094-1: "`rk-adapter-codex` is out of Phase 1 under every reading". E12: "A: Phase 2 — ADR-0094 decision 2 stands as written. No supersession"
- ADR-0010: "CI is a Phase 3+ question". The subject is CI execution of the CLAUDE adapter ("no decision is taken here about how (or whether) the Claude adapter runs under CI"). It sets no phase for Codex
- design §9.1: "`rk-adapter-codex` | the second harness | ... S-CODEX is unrun; it is not in Phase 1". design: "Phase 1 ships no plugin; `rk-plugin-gen` is Phase 2"
- AC-521 (register): "whose event log contains every story-lane stage and whose output is a Phase-2 story authored into this repository's own backlog by that run". Verified-by: "the story artifact's digest appears in the run's `artifact.write` events"
- ADR-0092 rule 2.2: the register "is the in-tree owner of each criterion"; "the register's THEN is never edited to match the design". Rule 1: a correction that moves a Decision sentence "is a supersession and takes a new record"
- ADR-0116 d1: "emitted by **every activity output**". d3: "Step 8 (1b) asserts that, not tree membership". d6: the digest is computed "by the same function the typed-artifact digests use (`rk-types`)"
- ADR-0136 d4: "The reference resolver is OWED"; "It cannot be built without an output-binding model — which is the `emit` arm's subject". Forbids: "extending the executor with an ad-hoc partial resolver"
- ADR-0143 d2: "A second *runnable* agent step refuses"
- ADR-0129 d3/d4: options are validated against a question registry, and the default question has options "`approve` and `changes-requested`, neither attaching anything". An attachment no declared option admits "is refused at the CLI boundary and nothing is written"
- design §5.1 / AC-122: "One write path: only `rk-store` writes bytes under a managed artifact tree"
- ADR-0006 (partially superseded by ADR-0075; spec.md stays human-facing): layout C for "human-facing artifacts (`spec.md`, `plan.md`, `story.md`)"
- ADR-0025 / AC-067: a human-supplied artifact is a vouched root (`derived_from = []`, `source = "human"`). DECISIONS-2026-09-12 #15 CONFIRMED 2026-09-13 (operator): the registry `source` is `human` for "`BugReport`, `IntakeRecord`, `IncidentRecord`, `ImpactAssessment`, `SessionNotes`"
- ADR-0128 d2/d3: the lane comes from the item's kind; an item whose lane is "neither given nor derivable" is refused and "writes nothing". When `--lane` is given and the kind derives nothing, the flag stands
- ADR-0155 / ADR-0159: the trailer slug is the `.redkiln/items/` file stem, and "Nothing in this repository declares a project."
- ADR-0156 / ADR-0157: `triage` has zero catalog activities, and a real run printed "advanced triage -> analysis". ADR-0162 supersedes 0156 clause 3 in part (the review call site is now built over a fixture)
- ADR-0163 d6: "The shipped tree is left alone, renders only the empty case, and Checkpoint 7i asserts that the empty case is distinguishable there"

**Measured state:**

Everything below was read at 4c282d7 unless marked as a tracker claim.

(1) `.redkiln/items/rk-story-codex-adapter.toml` was added in 9d3c469, a commit about the requires-cycle lint. It has kind="story", stage="specification", external_dep=true, blast_radius="med", size="med", criticality="med". Its own header cites ADR-0094 d2 as its warrant. No ledger row decided it: rk-story, STORY_ITEM, AUTHORED_STORY and "story item" get 0 hits across spikes/DECISIONS-2026-09-2*.md and .redkiln/DISPOSITIONS-2026-09-21.md.

(2) Codex's phase is contested, not settled. ADR-0094 d2 and E12 say Phase 2. ADR-0007's Decision is accepted and not superseded (ADR-0094's frontmatter supersedes only 0003), and it says Codex "ships in Phase 4 ... only after S-CODEX passes". The design §10.2 Phase-4 row ("Codex adapter — only after S-CODEX passes"; evidence ADR-0007) matches that Decision sentence. So it is not simply a stale site. E12's options table names ADR-0007 only for its gate bar and never quotes its phase sentence. Either way E-0094-1 says `rk-adapter-codex` "is out of Phase 1 under every reading". Under ADR-0007 the Codex adapter is not even a Phase-2 story.

(3) The Codex adapter cannot be delivered in Phase 1. The warrants are E-0094-1, design §9.1 ("it is not in Phase 1") and ADR-0007. ADR-0010 is NOT a warrant for this: its "Phase 3+" concerns the Claude adapter's CI. The story pack sets implement, verify, review and release to "mandatory", so a full-lane run on this item would have to land Codex code.

(4) The item's facets inflate the selection by five kinds. external_dep requires APIContract (specification), ThreatModel (design) and IntegrationReport (verify). blast_radius>=med requires RolloutPlan (planning) and FlagRecord (implement). rollout-plan's `requires` also leads with IntakeRecord.

(5) `.redkiln/artifacts.toml` registers IntakeRecord as `source = "human"`, and no activity provides it. There are 5 Spec providers (user-story-ac, bdd-scenarios, use-case-spec, structured-nl-requirements, fix-spec), and 3 of them hard-require IntakeRecord. Only user-story-ac (rank 10) and bdd-scenarios (rank 30) are tagged kind:story, and both require it.
- CORRECTION: the ledger does mention it. DECISIONS-2026-09-09 #15 and DECISIONS-2026-09-12 (#15 CONFIRMED 2026-09-13, operator) name IntakeRecord as a registry `source = "human"` root, mapped to the corpus `_intake-brief.md`. Keeping it human is therefore record-backed.
- In code, step3 returns Missing for an unmaterialised kind. A human-sourced kind has no candidate, so step4 returns `Unsatisfiable { reason: NoAdmissibleProducer }` (AC-054: "opens a gate").
- Unmeasured: whether a real run raises that gate.

(6) The attach route is closed by design today. Under ADR-0129 d3/d4 an attachment is admitted only if the gate's question declares an option that `attaches` the kind. The default question attaches nothing, and `grep -rn attaches .redkiln/` returns 0 hits. So `answer --attach IntakeRecord=...` is refused at the CLI unless a pack-declared question admits IntakeRecord and the step-4 gate is bound to it. `answer.rs` also says its role as `artifact.write`'s producer is "ROUTED, not decided" (RT-59; the record is owed at §7d.18).

(7) The human-root problem is shared with criterion (1). The fix lane's analysis activities minimal-repro and exploitability-analysis require BugReport, which is also `source = "human"`. `grep BugReport` over runbook/PROGRESS.md, s8 and s7h returns 0 hits, so no route or tracker row covers it for either lane.

(8) CORRECTION: a step executor does exist. Step 7g is tracker-CLOSED at 18eb4ae. `Runtime::execute_step` in crates/rk-kernel/src/runtime/step.rs is called from the activity walk. Its emit arm is `StepKind::Emit => StepVerdict::Emitted`, which writes nothing. `artifact.write` has one producer, `crates/rk-cli/src/verbs/answer.rs` `store_attachments` ("nothing in `rk-kernel/src` writes `artifact.write` [measured]"). Three further blockers apply:
- user-story-ac's emit binds `artifacts = { Spec = "${draft.outputs}" }`. That is a `${…}` reference, and ADR-0136 d4 says the resolver it needs is owed and names no stage.
- user-story-ac has 3 agent steps (draft; self-check with no `when`; adversary). Under ADR-0143 d2 the walk refuses at self-check before it reaches emit. fix-spec also has 3 agent steps, so this is shared with (1).
- The file write must go through rk-store (AC-122).

(9) Neither pack has a `[stage.triage]` block, and the catalog has 0 triage activities. ADR-0157 measured the `triage -> analysis` advance on the fix item. An item entering at `specification` never shows `triage` in its log, which fails AC-521's "every story-lane stage".

(10) The current s8 (1b) block ends with `git ls-files --error-unmatch "$AUTHORED_STORY"` and "# PASS only when ... the authored story is in the tree". That is the tree-membership assertion ADR-0116 d3 rejects in terms.

(11) Checkpoint 7i (5) hard-codes rk-story-codex-adapter. It requires `.redkiln/items/rk-story-codex-adapter.toml` to exist (`test -f`), and its PRECONDITION fails if the item owns any `rate_limit_reading` row. Both a real (1b) dispatch on it and deleting its file break a re-run of 7i.

(12) Tracker claims, not re-measured here: no `.redkiln/schemas`, `.redkiln/processes/` is empty, the 8 cards lack `permission_mode`, there are no `model` rows, no Story/Slice-Verdict trailer writer exists (ADR-0158/0160), and the shipped review stage is unreachable (ADR-0156; ADR-0162 built the call site over a fixture only).

**Options:**

### A. A full-lane run on a Phase-2 story that ships nothing into Phase 1's installed surface, entered at triage; IntakeRecord stays human (record-backed); the authored story is the emitted Spec

Keep AC-521 exactly as written: every story-lane stage, plus the Spec digest in the run's `artifact.write` rows. Retire the Codex item: E-0094-1, §9.1 and ADR-0007 keep it out of Phase 1.

The candidate must be a real Phase-2 story whose landed change does not install Phase-2 scope. That rules out a third installed pack (ADR-0011 Decision; E7 rejected install-and-disable), a fifth real adapter (ADR-0094 d1), rk-plugin-gen ("Phase 1 ships no plugin") and corpus migration (ADR-0013 frozen corpus). The least-colliding default is a story whose change stays in `spikes/p9/`, which E7 names as where "the other ten stay ... as Phase-2 work". Example: harden `spikes/p9/packs/spike.toml` against the installed catalog's five spike activities. Give it low facets (external_dep=false, blast_radius/size/criticality=low).

Set stage=`triage`: it has 0 activities, ADR-0157 measured its advance, and it puts the mandatory stage in the log.

IntakeRecord stays `source="human"`, as D12 #15 already confirmed. It reaches the run through `answer --attach`, which needs a pack-declared question whose option `attaches = IntakeRecord` (ADR-0129 d3/d4), bound to the step-4 NoAdmissibleProducer gate. The same route serves BugReport for criterion (1).

The authored story is the Spec from user-story-ac's `emit`, rendered as layout-C `spec.md` (ADR-0006/0075) and written through rk-store (AC-122). The emit arm journals it as `artifact.write` (ADR-0116 d1). (1b) asserts that row's digest equals the rk-types digest of the landed bytes.

- *Record form:* The check and the producer are execution under accepted ADR-0094 d3 and ADR-0116 d1/d3/d6.

One new record, ADR-0164, for the free residuals:
- where an emitted human-facing artifact lands under rk-store;
- the human-root route: which question attaches IntakeRecord/BugReport and how the step-4 gate binds to it. This also settles RT-59's routed-not-decided producer;
- the definition of "every story-lane stage". Proposed: a stage_transition into every `stage_order` stage whose rigor is not `forbidden`.

ADR-0136 d4's owed reference resolver and output-binding model needs its own record, or ADR-0164 carries it. It must be the whole mechanism, because ADR-0136 forbids a partial resolver.

The second-agent-step refusal (ADR-0143 d2) is shared with (1) and is owed there, not here.

A ledger row D24-x records the candidate and the reading that landing in spikes/p9 does not ship a pack (ADR-0011/E7). No design §10.2 edit belongs in this packet: see the ADR-0007 note in 'why'.
- *Work:* - A new `.redkiln/items/<candidate>.toml` (kind=story, stage=triage, low facets).
- The Codex item is retired in the same change as the 7i re-point, because 7i `test -f`s it.
- A pack-declared question in `.redkiln/packs/story.toml` (and bug-standard's twin) whose option attaches the human root, and the step-4 gate bound to it.
- The reference resolver (ADR-0136 d4). The emit arm then resolves `${draft.outputs}`, renders layout C, writes through rk-store and appends `artifact.write`.
- Rewrite s8 (1b): drop `git ls-files`, and query the store for artifact.write kind=Spec item=$STORY_ITEM whose digest equals the landed bytes.
- Shared with (1): the ADR-0143 per-step-spec problem (user-story-ac and fix-spec each have 3 agent steps), schemas, processes, permission_mode, model rows, the trailer writer, review reachability and project declaration (ADR-0159).
- *Size:* Record work: ADR-0164, the resolver record (or ADR-0164 carries it), 1 ledger row and 1 runbook edit. Code: L. The resolver and emit binding are the largest items, then the question/attach wiring. The rest is shared with (1).
- *Risks:* - The candidate must be cleared on the record. Landing a spikes/p9 edit under a story lane is a reading of ADR-0011/E7, not something they expressly permit, so the D24 row must state it.
- Every Phase-2 story delivered end to end during Phase 1 is inherent to AC-521's full-lane reading, and that should be said once.
- The resolver is not small, and ADR-0136 names no stage that owns it.
- Unmeasured: whether step 4 raises the gate on a real run, and whether a re-invocation classifies the attached root as Present.
- (1b) cannot close before (1): the ADR-0143 refusal and the human-root route block both lanes.
- *Citations:* references/adr/0011-phase-1-ships-fix-and-story-lanes-only.md "Phase 1 ships exactly two process packs"; spikes/DECISIONS-2026-09-12.md E7 "The other ten stay in `spikes/p9/` as Phase-2 work"; references/adr/ERRATA.md E-0094-1 "`rk-adapter-codex` is out of Phase 1 under every reading"; references/adr/0007-codex-validation-only-second-harness.md "only after S-CODEX passes"; references/2026-09-08-architecture-design.md §9.1 "it is not in Phase 1"; references/adr/0116-the-event-kind-table-gains-artifact-write.md "not tree membership"; references/2026-09-08-acceptance-criteria.md AC-521 "every story-lane stage"; spikes/DECISIONS-2026-09-12.md #15 "`BugReport`, `IntakeRecord`, `IncidentRecord`"; references/adr/0129-answer-grammar-and-the-question-registry.md "neither attaching anything"; references/adr/0136-a-step-guard-with-an-unresolved-reference-refuses.md "The reference resolver is OWED"; references/adr/0143-a-second-agent-step-refuses-rather-than-overwriting-the-commit.md "A second *runnable* agent step refuses"; references/2026-09-08-architecture-design.md §5.1 "only `rk-store` writes bytes under a managed artifact tree"; crates/rk-kernel/src/runtime/step.rs `StepKind::Emit => StepVerdict::Emitted`; crates/rk-cli/src/verbs/answer.rs "RT-59: that `answer` is this row's producer is ROUTED, not decided"; .redkiln/packs/story.toml `triage = "mandatory"`; references/adr/0157-a-stage-does-not-advance-into-itself-and-a-lane-completes-within-one-process.md "advanced triage -> analysis"

### B. "Authored" means specified; the run stops after the Spec is journalled

Read ADR-0011's "Phase 2's stories are authored under it" and ADR-0094 d3's "authors a Phase-2 story" as writing the story, not delivering it. This does not rescue the Codex item. Under ADR-0007 Codex is Phase 4, so it is not a Phase-2 story, and nothing about a spec-only run settles that. The candidate is still chosen as in A, but with no constraint on what its implementation would ship.

Entry at `triage`. IntakeRecord and the Spec as in A. The run stops at an open human gate after specification or planning, which exits 0 under C-01 D-01. (1b) asserts the transitions triage..stop plus the Spec's `artifact.write` digest.

- *Record form:* New ADR-0164 driving a dated in-place amendment of AC-521's THEN, from "every story-lane stage" to the stages up to the authoring boundary. The direction (register toward an ADR) is allowed under ADR-0092 rule 2.2; the register is never edited toward the design. Design §10.2 (1b) and s8 (1b) are amended toward it. ADR-0094 d3's sentence stands, but it calls AC-521 the "twin of AC-473", which is a full-lane criterion, so the record must say why the twin is asymmetric. This changes the Phase-1 exit bar and needs an operator decision.
- *Work:* A stop mechanism no record provides. ADR-0128 forbids a second `run` grammar, so it cannot be a flag; it has to be a pack or item-level gate at the boundary. Beyond that, the same resolver, emit binding, question/attach route and ADR-0143 dependency as A, because user-story-ac's walk still refuses at its second agent step.
- *Size:* Record work: 1 ADR plus register, design and runbook amendments. Code: M for the stop mechanism, plus the A-shared resolver, emit and attach work.
- *Risks:* It weakens an exit criterion and breaks the symmetry with AC-473. ADR-0116's rejected "NOT RUN" option concerned the digest half, not stage-completeness, so B is not the same shape and should not be argued as if it were. The stop point depends on the selection and is unmeasured. It mints machinery that exists only to pass the check. The shared blockers (resolver, ADR-0143) are NOT avoided, so B's saving is limited to review and release reachability.
- *Citations:* references/adr/0011-phase-1-ships-fix-and-story-lanes-only.md "Phase 2's stories are authored under it"; references/adr/0094-phase-1-ships-a-rust-toolchain-adapter-and-self-hosting-means-both-lanes.md d3 "authors a Phase-2 story"; references/adr/0092-errata-precedence-and-cut-id-reissue.md "the register's THEN is never edited to match the design"; references/adr/0128-one-run-grammar-lane-optional.md "Forbids a second `run` grammar"; references/adr/0007-codex-validation-only-second-harness.md "ships in Phase 4"

### C. The authored story is a new backlog item record the run writes

Read "authored into this repository's own backlog" as the run producing a new `.redkiln/items/<phase-2-story>.toml` (kind=story), with its file digest in `artifact.write`. Entry stage and intake as in A.

- *Record form:* New ADR-0164 minting an artifact kind for an item record (ItemRecord is not an artifact kind today), a producing activity, and its lane placement. Register and runbook amendments.
- *Work:* A new artifacts.toml kind, a new catalog activity with its schema, and a writer into `.redkiln/items/`. `.redkiln/items/` is not an rk-store managed artifact tree, so AC-122's single write path must be extended or argued. No shipped activity provides anything item-shaped: vertical-slicing provides SliceSet/DependencyDAG and is tagged project/initiative only.
- *Size:* L: a new kind, activity, schema and writer, plus records.
- *Risks:* The largest option. The story pack's lane does not decompose, so this is a pack-shape change close to ADR-0011's line. Writing into `.redkiln/items/` crosses ADR-0155/0159: the file stem becomes a trailer slug, and "Nothing in this repository declares a project."
- *Citations:* references/2026-09-08-acceptance-criteria.md AC-521 "authored into this repository's own backlog"; .redkiln/catalog/vertical-slicing.toml `provides = ["SliceSet", "DependencyDAG"]`; references/adr/0159-the-project-component-of-a-trailer-value-is-declared-and-refused-when-absent.md "Nothing in this repository declares a project."

**Recommended:** Option A, corrected.

Execution owed now under accepted records, whichever option is chosen:
(i) Rewrite s8 (1b)'s last two lines to ADR-0116 d3's assertion (the Spec's digest in the run's `artifact.write` rows), not `git ls-files`. The line will stay unsatisfiable until an emit producer exists, and that is honest.

Removed from the execution list: the design §10.2 Phase-4 row correction. That row matches ADR-0007's accepted, unsuperseded Decision ("ships in Phase 4"), so editing it would move a Decision without a record (ADR-0092 rule 1). Raise it as a separate item: ADR-0094 d2 vs ADR-0007 on Codex's phase, owed a supersession-in-part of ADR-0007 or an operator ruling that E12 did not make.

Decisions A asks the operator to record:
- Candidate: retire rk-story-codex-adapter (E-0094-1, §9.1, ADR-0007). Name a real Phase-2 story whose landed change stays inside `spikes/p9/`, for example hardening `spikes/p9/packs/spike.toml`, with low facets. NOT the spike pack installed disabled, which is E7's rejected option and a supersession of ADR-0011's Decision.
- Entry stage: `triage`.
- Human roots: IntakeRecord and BugReport stay `source=\"human\"`, as D12 #15 already confirmed. They are supplied through a pack-declared question whose option attaches the kind (ADR-0129 d3/d4), bound to the step-4 gate. Decide this once for both lanes: criterion (1) needs it too.
- Authored story: user-story-ac's `Spec`, emitted through the owed reference resolver (ADR-0136 d4, the whole mechanism), rendered layout-C `spec.md`, written by rk-store (AC-122) at a path ADR-0164 fixes, and journalled as `artifact.write`.
- 7i re-point: point Checkpoint 7i (5)'s empty case at a dedicated item nobody dispatches, for example `.redkiln/items/rk-cost-empty-case.toml`, in the same change that retires the Codex item. Keep the PRECONDITION guard, since that guard is the protection. Do NOT claim the sentinel "can never run", and do not add a `run` step: `derive_lane`'s `(Some(given), None) => Ok(given)` runs a kind=docs item whenever `--lane` is passed. This is a runbook edit carrying out ADR-0163 d6.

**Why:** DEFECTS FOUND IN THE PACKET:

1. Missed binding: ADR-0011's Decision "Phase 1 ships exactly two process packs", and E7's operator ruling that rejected "Install twelve, enable two via `[packs] enabled`". The packet's default candidate (the spike pack shipped disabled) is that rejected option, so it would supersede ADR-0011 and reverse E7 without saying so. It also misstates the work: `spikes/p9/packs/spike.toml` already exists. And the quote "`bug-standard` and `story` process packs only" is design §10.2's wording, not ADR-0011's.

2. Missed binding: ADR-0007's Decision (accepted, superseded_by []) says Codex "ships in Phase 4 ... only after S-CODEX passes". The packet's claims that Codex = Phase 2 is "settled ... not contested" and that the design Phase-4 row is merely stale are both false. E12 never addressed ADR-0007's phase sentence. Recommendation (ii), the design correction, is therefore not execution: it moves a Decision (ADR-0092 rule 1).

3. Mischaracterised citation: ADR-0010's "CI is a Phase 3+ question" concerns the Claude adapter's CI. It does not place Codex behind a Phase 3+ decision. The direct warrant is E-0094-1: "out of Phase 1 under every reading".

4. False measured claim: "No ADR, design section or ledger mentions IntakeRecord (grep: 0 hits)". DECISIONS-2026-09-09 #15 and DECISIONS-2026-09-12 (#15 CONFIRMED 2026-09-13, operator) name it as a registry `source = "human"` root. Keeping it human is record-backed, not a free residual.

5. False measured claim: "no step executor exists". Step 7g closed at 18eb4ae, and `execute_step` walks the steps. Its emit arm returns `StepVerdict::Emitted` and writes nothing. The memory quote the packet relied on is stale.

6. Missed binding: ADR-0136 d4. The emit binding `Spec = "${draft.outputs}"` needs the reference resolver, which is OWED, and ADR-0136 forbids a partial one. That makes the emit work large, not the small unique cost the packet claims.

7. Missed binding: ADR-0143 d2. user-story-ac has 3 agent steps, and the walk refuses at `self-check` before `emit`. fix-spec has the same shape, so this is shared with (1).

8. Missed binding: ADR-0129 d3/d4. The default question attaches nothing, and `.redkiln/` declares zero `attaches`. `answer --attach IntakeRecord=...` is therefore refused by design. RT-59 also records `answer` as `artifact.write`'s producer "ROUTED, not decided".

9. Missed binding: AC-122 / design §5.1. Only rk-store writes under a managed artifact tree, so the Spec file cannot be written by a kernel "emit executor". ADR-0006 is also partially superseded by ADR-0075, which the packet did not cite.

10. Scope error: the human-root route is shared with (1). BugReport is `source = "human"` and is required by the fix lane's analysis activities. No tracker row covers either lane, so treating IntakeRecord as a (1b)-only residual would split one decision in two.

11. False claim about the 7i sentinel: a kind=docs item "can never run" is wrong. With `--lane` given, `derive_lane` lets the flag stand. The proposed "run refuses" step also adds a `run` call to a checkpoint that asserts the store is unchanged. It is dropped as not least-change.

12. Overstated risk in B: ADR-0116's rejected option was dropping the digest half ("record Step 8 (1b)'s digest half NOT RUN"), not narrowing stages. B's real obstacle is changing a register THEN and breaking the "twin of AC-473" symmetry. B also cannot keep Codex, because under ADR-0007 it is not a Phase-2 story.

WHAT SURVIVES:
- A still keeps AC-521 and ADR-0116 d3 exactly as written. B lowers the exit bar, and C invents an artifact kind and a pack shape.
- Retiring the Codex item stands, on better warrants: E-0094-1, §9.1 and ADR-0007.
- `triage` as the entry stage stands. It has 0 activities, ADR-0157 measured the advance, and entering at `specification` fails "every story-lane stage" on the log alone.
- The s8 (1b) rewrite is genuine execution under ADR-0116 d3.

The corrected candidate default keeps the landed change inside `spikes/p9/`. E7 names that as where the Phase-2 packs stay, so no third pack is installed. Every other §10.2 Phase-2 scope line collides with a record if delivered in Phase 1: ADR-0094 d1's four adapters, "Phase 1 ships no plugin", and ADR-0013's frozen corpus.

A's unique cost is no longer small. The resolver, the question/attach wiring and the rk-store write path are all real work, and the ADR-0143 refusal blocks both lanes, so (1b) cannot close before (1).

## (g) Card permissions and model tiers: what tool grants and permission_mode each of the 8 shipped cards carries, what model/effort each [role.*] row declares, how the closed-output schema for each reached kind is produced (and for which kinds), and which [[gate_expiry]] rows ship. The constraints are that a run must never wait on a permission prompt and that edits must stay inside the run-owned worktree.

**TAKEN 2026-09-24:** B — least privilege; tiers as the packet's table; schemas as rk-types EXPORTED; gate_expiry per-class table signed by the operator; accepted only after two live probes.

**Settled by record already:** False

**Binding records found:**

- ADR-0014 (accepted): process packs own "roles ... tier policy per role"; technology packs "bind (cards, skills)". It settles who owns the tier, but it names no value.
- AC-118 / AC-418: an authored card carrying model or effort is a leak ("the tier is owned by the role"). card.rs refuses it as ConfigError::TierLeak.
- AC-212: model and effort must trace to the role. binder.rs refuses with ModelUntraceable, EffortUntraceable (a tier that takes effort and has none: sonnet and opus REQUIRE an effort) or EffortRefusedByTier (haiku given one). AC-212's note: "it does not license any claim about which effort a card should choose".
- AC-214: "`--permission-mode` is present with an explicit value taken from the card or pack; there is no implicit default". The quote is AC-214's own. ADR-0003 is its decision link but has no permission-mode sentence.
- AC-215: "`--permission-prompts none` is passed and the child's stdin is closed, so a prompt-shaped stall is impossible rather than merely unlikely". Its instrument is "a fixture that requests a disallowed tool, which must terminate rather than hang under a wall-clock bound". It is tagged [surface unverified].
- Design principle 7 (§ principles table): "Every dispatch passes card, system prompt, tools, schema, MCP config and permission mode. *Forbids:* inferring from the parent side that a flag took effect". The packet MISSED this. It binds any containment that rests on a permission mode being honoured.
- ADR-0108 d2: "The effective tool set is the card's declared list and nothing else". AC-205 makes StructuredOutput mandatory on a schema-bearing dispatch.
- AC-204: the system/init tool set must be EQUAL to the card's ("a superset is not adoption"), or the dispatch fails as NotIsolated.
- AC-269: no tool name may spell answer, mint or resolve as a segment.
- AC-529 + ADR-0105 clause 8: the run-owned worktree is resolved "**from the store**, never from cwd". Scope: this governs TIER-1 (git) writes, not an agent's file edits.
- ADR-0127: rk-containment owns process containment (a Job Object with KILL_ON_JOB_CLOSE). Its Decision contains no filesystem clause.
- ADR-0010 + env_denylist.rs: subscription-only auth; a denylisted variable is a construction refusal. ADR-0010 is an AUTH record and carries no filesystem-containment posture.
- ADR-0084 (accepted, partially superseded by 0095 and 0100). The packet MISSED this. Its golden-snapshot clause stands: "Every type that crosses the closed-output boundary has its generated `schemars` output snapshotted with `insta`", and its Forbids reads "the rule binds the Rust types, and the snapshot test is what checks it". ADR-0095 supersedes only "the golden-snapshot clause's rule set and the :117 evidence line".
- ADR-0095 decision 1: the schema rule binds "Every schema `rk-types` exports across the closed-output boundary" (no oneOf, no nested unions, additionalProperties:false, <=16 union and <=24 optional parameters).
- AC-133 (the packet MISSED this): "GIVEN any enum on a shipped artifact type ... the enum appears in the judgement-enum registry", plus a rationale sibling and a completeness test. A hand-authored JSON schema escapes that registry.
- rk_types::registry doc: EXPORTED is "Every schema that crosses the closed-output boundary". The binder doc agrees: the composition root takes the schema "from `rk_types::registry::EXPORTED` by name".
- ADR-0017: "fail / escalate / auto-decide, never a hang". AC-260: "no implicit default expiry action, and no escalate-on-escalate".
- AC-276 (the packet MISSED this): "the choice is made and signed by a human, per class". The instrument is "routed to a human gate; not machine-decidable".
- ADR-0146: "Zero rows, for zero of the 45 question classes ... It is OWED, and the honest options are to ship the rows or to make an unmatched step-gate class a load-time refusal". ADR-0147 carried finding (22) says the same.
- ADR-0156 blockers (0) and (1). Its Forbids bar "Authoring `.redkiln/schemas/` ... under this stage's scope", and ADR-0162 bars the same "under this record's scope" and says "The route stays recorded and unowned". Both bars are stage-scoped.
- ADR-0163: "0 `model =` rows sit under the 16 `[role.*]` tables ... `ConfigError::ModelUntraceable`".
- RT-57 (DECISIONS-2026-09-18 routing table; s7d): the PackExtras key floor includes "the gate-class expiry action", and "a record fixing these keys in the pack schema is 7e's (RT-57; §7d.18 RAISED)". The auto-decide verdict carrier is absent: run.rs refuses "auto-decide" by name.
- Working tree only, NOT at HEAD. D21-40 in spikes/DECISIONS-2026-09-21.md is uncommitted (`git status` shows M); the last entry at 4c282d7 is D21-39. Decision 1 there: "nothing stages or lands the harness's work", and the owed decisions go to the operator "as a packet, and the new stages are authored after that". Decision 2: real runs may dispatch "once the sub-stages' fixture proofs pass, and not before".

**Measured state:**

Read directly at 4c282d7, read-only; nothing built or run. The working tree also has uncommitted edits to runbook/PROGRESS.md and spikes/DECISIONS-2026-09-21.md, the latter adding D21-40.

CARDS
- All 8 .redkiln/cards/*.json have exactly the keys {cache_profile:"static", description, prompt, tools:["StructuredOutput"]}.
- None has permission_mode, so AuthoredCard::from_file refuses each one. This is ADR-0156 blocker (0).
- card::lint_card checks only that permission_mode is non-empty (`permission_mode.trim().is_empty()`). There is NO closed-set check: a misspelled "acceptEdit" would pass bind.
- The implementer prompt says "Make the change the step describes, and report it", yet its only tool is StructuredOutput.

ROLE ROWS
- There are 16 [role.*] tables across bug-standard.toml and story.toml; `grep -c "model\s*="` returns 0 and 0.
- RoleExtras (run.rs) parses optional `model: Option<Tier>` and `effort: Option<Effort>`.
- Binder::bind behaves this way:
  - no model → ModelUntraceable;
  - a tier that accepts effort (sonnet or opus) with no effort → EffortUntraceable;
  - haiku with an effort → EffortRefusedByTier.
- Every sonnet or opus row therefore MUST carry an effort.
- The pack-min fixture shows the shape, and its implementer card carries "permission_mode": "acceptEdits".
- The packs already carry gate_deadline_secs = 864000 (10 days), [margins], [cache], and 102 [activity.*] timeout_secs rows each. So the timeout floor is not a blocker.

ARGV (command.rs assemble)
- The fixed order is: `-p --output-format stream-json --verbose --model <tier> [--effort <e>] --agents {...} --agent <role> --json-schema <schema> --permission-mode <card value> --strict-mcp-config --permission-prompts none [--system-prompt <body> under static]`.
- `--permission-prompts none` is unconditional (the AC-215 argv half).
- No `--allowed-tools` is emitted.
- No `--setting-sources` or `--settings` flag is emitted, so the child still discovers user and project settings. MEASURED: the operator's two user settings files currently hold no `permissions`/`additionalDirectories` rules.
- The argv cap is ARGV_CAP_BYTES = 32,767 (UTF-16 units).

CONTAINMENT (runner.rs)
- `.current_dir(&prepared.worktree)`, where the worktree comes from spec.inputs["worktree"] (RT-21).
- `.env_clear()`, then the captured environment.
- A Job Object for lifetime, and stdin written once and then closed.
- Nothing in rk-containment restricts paths.
- NOTHING in rk-adapter-claude/src reads the child's permission mode back from system/init. Outside command.rs, card.rs and binder.rs's copy there is no `permission` match.
- NOTHING handles `permission_denials` / `permission_denied`. Those appear only in recorded test fixtures, so a denied tool call ends in an ordinary completion.
- The run-owned worktree sits under .git/redkiln/worktrees/<item>; one exists for rk-fix-doctor-test-count.

LANDING (the packet MISSED this)
- Both `commit_with_trailers` call sites in verbs/run.rs pass `true` (allow_empty) with no `git add`.
- GitOp::Integrate has no caller outside git_exec.rs.
- So any file an agent edits in the worktree is never staged or committed. No card option makes criterion (1) pass without a landing path, which D21-40 (working tree) routes to a new sub-stage.

WHAT HAS BEEN MEASURED
- The Checkpoint 7c live probe (PROGRESS row 490: "`--permission-prompts` on the argv, `COMPLETED_ROWS=1`"; TESTED_CLI_VERSION "2.1.278") used probe.json with "permission_mode": "default" and StructuredOutput only.
- The earlier recorded card argv (F20) used bypassPermissions.
- UNMEASURED:
  - acceptEdits under prompts none, both for an edit inside the worktree and one outside it;
  - reads outside cwd under either mode;
  - system/init tool-set equality with Read/Glob/Grep/Edit/Write granted;
  - whether system/init reports the mode that was passed.

ROLE BINDING (bind_nodes, run.rs)
- The card is `.redkiln/cards/<role>.json`, taken from `activity.steps.iter().find_map(|s| s.role.clone())`: the FIRST STEP THAT HAS a role, not literally the first step.
- There is one spec per activity; activities with no role are skipped.
- The schema is read from `.redkiln/schemas/<provides[0]>.json`, and that directory does not exist.
- No schema lint runs at bind: `schema::lint` has no caller in rk-cli/src or rk-adapter-claude/src.

SCHEMAS
- rk_types::registry::EXPORTED has one entry, "Findings", doc-commented as "Every schema that crosses the closed-output boundary".
- rk_types::schema::lint::lint(&Value) exists.
- None of RootCause, ReproCase, FixDesign, Changeset, TestReport, ReviewVerdict, ChangeRecord or ClosureRecord exists as a Rust type anywhere in crates/*/src.
- So run.rs's file read contradicts both the binder doc and the registry's own contract.

KINDS REACHED (a PROXY by kind tag, not a selection run)
- About 20 kinds for the fix lane and about 31 across both lanes, against 61 catalog-wide.
- The items are rk-fix-doctor-test-count (kind bug, stage triage) and rk-story-codex-adapter (stage specification).

GATES
- `grep -h '^question' .redkiln/catalog/*.toml | sort -u | wc -l` → 45; 0 [[gate_expiry]] rows.
- expiry_pass fires only for an open gate whose deadline has passed, so a missing row bites only after the 10-day deadline, at the next read.
- Selection gates key their class as `unknown_facet:<axis>`.
- auto-decide is refused by name.

NEVER-WAITS
- The phrase "run NEVER WAITS" is not verbatim in 00-preamble.md.
- The recorded forms are AC-215's sentence and golden constraint 5, "Every waiter needs a failure alternative".

**Options:**

### A. Agents never write: every card read-only under "default"; the kernel applies a diff

CARDS
- All 8 cards get "permission_mode": "default", the one mode measured with the full vector at Checkpoint 7c.
- Operator, releaser and planner get StructuredOutput only.
- Investigator, reviewer, architect, spec-author and implementer get Read, Glob, Grep and StructuredOutput.

THE WRITE PATH
- The implementer's Changeset carries a unified diff.
- rk-cli/rk-git runs `git apply --check`, then applies the diff in the run-owned worktree after an AC-529-style ownership check.
- It then stages and commits the change, which is the landing path D21-40 says is missing.

Tiers are as in B. Schemas and gate_expiry are as in B.

- *Record form:* New ADR-0164, which decides:
- the grants and the mode;
- a diff-bearing Changeset Rust type (under ADR-0084/0095);
- the kernel-side apply-and-land step.

It must reconcile, in one sentence, with ADR-0162's "forbids applying a mutation patch from `rk-cli`". That Forbids covers ReproKind::Mutate, is scoped to that record, and is near enough to need saying.
- *Work:* - Edit 8 cards and 16 role rows.
- Add a Changeset Rust type with a patch field, within the ADR-0095 limits.
- Build the apply, stage and commit path in rk-git/rk-cli, which refuses on a HEAD or branch mismatch, plus its tests.
- Schemas and gate_expiry as in B.
- *Size:* L. A new apply path plus the landing path, which every option needs anyway, about 31 Rust types and 45+ signed expiry rows.
- *Risks:* - Strongest containment: no agent holds a write tool, and the only mode used is the measured one.
- Blind unified-diff authoring is a known fidelity failure (context drift and hunk offsets), and retries consume attempts_total.
- It diverges from how the spikes measured implementation work, with agents holding Read and Bash (design §5.2, P8).
- A large patch inflates the StructuredOutput payload, though not argv.
- Reads outside cwd are still unmeasured.
- *Citations:* crates/rk-adapter-claude/tests/fixtures/cards/catalog/probe.json: "permission_mode": "default"; runbook/PROGRESS.md Checkpoint 7c row: "`--permission-prompts` on the argv, `COMPLETED_ROWS=1`"; references/adr/0108: "The effective tool set is the card's declared list and nothing else"; references/adr/0162: "forbids applying a mutation patch from"; crates/rk-cli/src/verbs/run.rs: both `commit_with_trailers(` call sites pass `true` (allow_empty), with no `git add`

### B. Per-role least privilege: only the implementer writes, under acceptEdits confined to cwd = worktree; no Bash; schemas as rk-types exports

CARDS
- implementer: ["Read","Glob","Grep","Edit","Write","StructuredOutput"], "acceptEdits".
- investigator, reviewer, architect, spec-author: ["Read","Glob","Grep","StructuredOutput"], "default".
- planner, operator, releaser: ["StructuredOutput"], "default".
- No card gets Bash, Web* or mcp__*, and build and test stay on the Toolchain port.

PRINCIPLE-7 COMPLETIONS, owed before the mode is trusted:
- a closed-set lint on permission_mode (default|acceptEdits|plan|dontAsk|bypassPermissions|auto, with the shipped set restricted to default|acceptEdits);
- a system/init check that the reported permission mode equals the card's (the design's "inferring from the parent side" Forbids);
- turning a result's `permission_denials` into a typed dispatch failure, so a denied out-of-worktree edit fails the step instead of completing silently.

TIERS ([inferred]; every sonnet or opus row must carry an effort, or EffortUntraceable fires):
- implementer sonnet/high; investigator sonnet/high; spec-author sonnet/medium;
- reviewer opus/high; architect opus/medium;
- planner, operator, releaser haiku with no effort.

SCHEMAS
- For each kind the two lanes' selections actually reach (about 31 by proxy), add a Rust type in rk-types, register it in `EXPORTED`, and snapshot it. Its enums are registered under AC-133.
- run.rs takes the schema from `rk_types::registry::EXPORTED` by provides[0], as the binder doc already says, instead of reading .redkiln/schemas/.
- This complies with ADR-0084 and ADR-0095 as written, and needs no directory at all.

GATE_EXPIRY
- One row per class the two lanes can open: the 45 question classes plus the `unknown_facet:<axis>` classes the two items can raise.
- The recommended action is "fail", but each class is listed and SIGNED by the operator in the record, per AC-276.
- No escalate (it would need an escalation-class row, AC-260) and no auto-decide (it has no verdict carrier, RT-57).

- *Record form:* New ADR-0164. It executes ADR-0014, AC-212, AC-214, AC-215, ADR-0108, ADR-0084/0095, AC-133, and ADR-0146's OWED rows.

It carries as Decision sentences:
- RT-57's gate-class expiry values, as the AC-276 per-class signed table;
- the per-role tier table.

It supersedes nothing. The ADR-0156 and ADR-0162 bars are stage-scoped, and routing schemas through EXPORTED avoids an ADR-0084 supersession.

It also records two things:
- landing (stage and commit of the implementer's edits) is NOT decided here and is owed by the D21-40 sub-stage;
- the probes' spend is governed by D21-40 decision 2, once that entry is committed.
- *Work:* PACK EDITS
- 8 card JSONs.
- model/effort in the 16 role rows, directly under each [role.x]. Beware pack-min's "a bare key belongs to the LAST table opened" trap.

CODE
- The closed-set permission_mode lint.
- The system/init permission-mode equality check.
- The permission_denials → typed failure mapping.
- About 31 rk-types Rust types with the EXPORTED registration, snapshot updates and AC-133 registry entries.
- run.rs reading EXPORTED by name.

GATE ROWS
- 45+ signed [[gate_expiry]] rows per pack.

TWO LIVE PROBES, one paid dispatch each, after the fixture proofs (D21-40 decision 2):
1. An acceptEdits implementer edits a file inside the worktree and completes. An Edit aimed outside it is denied, surfaces as a failure and exits under the wall-clock bound (AC-215's "fixture that requests a disallowed tool"). The same probe records whether a Read outside cwd is permitted.
2. system/init's tool list equals the card list, and its permission mode equals the card's, under both modes (AC-204, principle 7).

OPTIONAL
- A doctor row refusing Bash, Web* or mcp__* in any shipped card.
- ADR-0146's load-time lint for an unmatched gate class.
- *Size:* M-L. Pack content, about 31 derived Rust types, three small adapter guards, and two live probes. No new write path in this packet; the landing path is owed separately by every option.
- *Risks:* - acceptEdits confinement under prompts none is DOCUMENTED CLI behaviour, not measured. If the CLI auto-approves an out-of-cwd edit, or stalls instead of denying, containment or never-waits breaks. Probe 1 is the gate, and a failure is a Halt, not a re-tune.
- Nothing pins the settings sources. A user or project `permissions.allow`/`additionalDirectories` rule would silently widen the grant. None exists today (measured), but it is discovered, not explicit. The system/init mode check and an optional pinned `--setting-sources` (which changes the golden vector and would need its own measurement) mitigate this.
- Whether reads are confined to cwd is UNMEASURED, and the packet asserted it without evidence. With no Bash, web or MCP tool, the only exfiltration channel is the StructuredOutput payload.
- B does NOT by itself make criterion (1) pass: an edit made in the worktree is never staged or committed at HEAD.
- Binding uses the first role-bearing step's card for the whole activity.
- The static profile's double prompt plus a large schema could approach the 32,767 cap. assemble refuses that at bind time.
- *Citations:* crates/rk-adapter-claude/src/command.rs: "argv.push(\"--permission-prompts\".to_owned());" (unconditional, AC-215); crates/rk-adapter-claude/src/runner.rs: ".current_dir(&prepared.worktree)"; crates/rk-adapter-claude/src/binder.rs: "the composition root takes it from `rk_types::registry::EXPORTED` by name"; crates/rk-types/src/registry.rs: "Every schema that crosses the closed-output boundary."; references/adr/0084: "Every type that crosses the closed-output boundary has its generated `schemars` output snapshotted with `insta`"; references/2026-09-08-architecture-design.md principle 7: "*Forbids:* inferring from the parent side that a flag took effect"; references/2026-09-08-acceptance-criteria.md AC-276: "the choice is made and signed by a human, per class"; references/2026-09-08-acceptance-criteria.md AC-215: "an integration test with a fixture that requests a disallowed tool, which must terminate rather than hang under a wall-clock bound"; references/2026-09-08-acceptance-criteria.md AC-204: "a superset is not adoption"; references/2026-09-08-acceptance-criteria.md AC-133: "the enum appears in the judgement-enum registry"; references/adr/0146: "the honest options are to ship the rows or to make an unmatched step-gate class a load-time refusal"; crates/rk-cli/tests/fixtures/pack-min/cards/implementer.json: "permission_mode": "acceptEdits"

### C. Broad grant: bypassPermissions on every card, with Bash for implementer and investigator

This follows F20's recorded argv (`--permission-mode bypassPermissions`) and the reviewer fixture.
- Every card gets "bypassPermissions".
- Implementer and investigator get Read, Glob, Grep, Edit, Write, Bash and StructuredOutput, as the P7/P8 spike agents had.
- The rest get read tools.
- No prompt can arise, because nothing is asked.

Tiers, schemas and gate_expiry are as in B.

- *Record form:* New ADR-0164. It must state that filesystem and command containment is cwd only.

ADR-0010 is an auth record and is not in conflict. Design principle 7's isolation is about MCP/tool inheritance as a measurement precondition, not filesystem scope, so a supersession is not strictly required. The record must still answer principle 7's "Every dispatch passes ... permission mode" with a system/init check.
- *Work:* - Edit 8 cards and 16 role rows.
- Schemas and gate_expiry as in B.
- One live probe for AC-204 tool-set equality with Bash present.
- *Size:* M. The schema types and signed rows dominate, and it needs the fewest probes.
- *Risks:* - No filesystem or command containment: a Bash-holding agent can write anywhere the user can, run git against other refs, and run builds and tests outside the Toolchain port's recorded verbs, which duplicates and unrecords the measurements.
- Bash can read user-scope files, even though the env denylist keeps credentials out of the environment.
- An agent could `git commit` in the worktree, creating a commit that does not come from the kernel's trailer writer.
- It is the most likely option to produce an unattributable change.
- *Citations:* crates/rk-adapter-claude/src/command.rs: "the one recorded card-dispatch argv is F20's"; crates/rk-adapter-claude/tests/fixtures/cards/catalog/reviewer.json: "permission_mode": "bypassPermissions"; references/2026-09-08-architecture-design.md: "Isolation is a **measurement precondition**, not hygiene"; references/2026-09-08-acceptance-criteria.md AC-212: "it does not license any claim about which effort a card should choose"; references/2026-09-08-architecture-design.md: "localised **0 times in 6** Opus attempts in P7 and **reproduced on the first haiku attempt** in P8"

**Recommended:** B, corrected, recorded as a new ADR-0164 that supersedes nothing.

CARDS
- Only the implementer writes (Read/Glob/Grep/Edit/Write/StructuredOutput under acceptEdits, cwd = the run-owned worktree).
- The four reading roles get Read/Glob/Grep/StructuredOutput under "default".
- Planner, operator and releaser get StructuredOutput under "default".
- No Bash, Web or MCP tool on any card.
- Add three principle-7 guards: a closed-set permission_mode lint, a system/init permission-mode equality check, and permission_denials mapped to a typed failure.

TIERS
- The [inferred] table: implementer and investigator sonnet/high; spec-author sonnet/medium; reviewer opus/high; architect opus/medium; planner, operator and releaser haiku with no effort.
- Every sonnet or opus row carries an effort.

SCHEMAS
- Rust types in rk-types for the kinds the two lanes actually reach, registered in EXPORTED, snapshotted and AC-133-registered.
- run.rs reads them from EXPORTED by name. This complies with ADR-0084 and ADR-0095, so .redkiln/schemas/ is not authored at all.
- Hand-authored JSON would need an explicit ADR-0084 supersession in part and would escape AC-133.

GATE_EXPIRY
- A per-class table covering the 45 question classes plus the reachable unknown_facet:<axis> classes.
- "fail" is proposed for each, SIGNED by the operator per class (AC-276).

ACCEPTANCE AND LANDING
- ADR-0164 is accepted only after two live probes pass:
  1. acceptEdits confinement plus a denial that surfaces and terminates under prompts none;
  2. init tool-set and permission-mode equality.
- Both probes are dispatched only after the fixture proofs D21-40 decision 2 requires.
- The record states that landing the implementer's edit is out of scope and owed by the D21-40 sub-stage.

**Why:** DEFECTS FOUND IN THE ORIGINAL PACKET:
(1) It missed ADR-0084 (accepted). Its golden-snapshot clause stands after ADR-0095, which supersedes only "the golden-snapshot clause's rule set and the :117 evidence line": "Every type that crosses the closed-output boundary has its generated `schemars` output snapshotted with `insta`", and "the rule binds the Rust types". The packet's hand-authored .redkiln/schemas/*.json route therefore breaks an accepted Decision sentence, and "supersedes nothing" was false. The compliant route is Rust types in EXPORTED, and the registry's own doc ("Every schema that crosses the closed-output boundary") and the binder doc already say so.
(2) It missed AC-133. Enums on shipped artifact types must be in the judgement-enum registry with a rationale sibling, and hand-written JSON escapes that.
(3) It missed AC-276: "the choice is made and signed by a human, per class". A blanket "fail for all 45" is not that. The record must carry a per-class signed table.
(4) It missed the landing gap. Both commit_with_trailers call sites in verbs/run.rs pass allow_empty = true with no `git add`, and GitOp::Integrate has no caller. So "B is the smallest grant that can still pass s8 criterion (1)" and "adds no new write path" overstate: under B an edit is never committed. D21-40 (working tree, uncommitted at 4c282d7) records this and routes it to a sub-stage.
(5) It missed design principle 7's Forbids: "inferring from the parent side that a flag took effect". B's confinement rests on acceptEdits being honoured, yet nothing in rk-adapter-claude/src reads the permission mode back from system/init. lint_card also checks only that permission_mode is non-empty, with no closed set.
(6) It missed the silent-denial risk. There is no permission_denials handling in adapter src, so a denied out-of-worktree Edit completes normally. AC-215's "terminate rather than hang" test would pass while the step silently failed.
(7) It missed settings discovery. No --setting-sources is emitted, so user or project permission rules could widen the grant. None exist today (measured).
(8) "Reads are NOT confined" was asserted without measurement. It is now labelled unmeasured and folded into probe 1.
(9) bind_nodes binds the first role-BEARING step (`find_map`), not the literal first step.
(10) Option C cited "ADR-0010's containment posture", but ADR-0010 is subscription-only auth and has no containment clause. C also said it "defeats AC-529's intent", but AC-529 governs tier-1 git writes, not agent file edits.
(11) It attributed AC-214's sentence to ADR-0003, which carries no permission-mode sentence.
(12) It omitted that sonnet and opus rows MUST carry an effort (EffortUntraceable), and that D21-40 decision 2 gates paid dispatches behind fixture proofs.
Quotes verified as existing and meaning what the packet said: ADR-0146, ADR-0147 (22), ADR-0156, ADR-0162, ADR-0163, ADR-0108, ADR-0095, AC-204, AC-212, AC-215, AC-260, AC-529, the design's "model choice — the tier lives on the role" and "0 times in 6", command.rs, runner.rs, the fixtures, the 45-class count and the Checkpoint 7c row.

WHY B, CORRECTED:
- No accepted record settles this. ADR-0014 and AC-212 fix only who owns the tier. AC-214 fixes only that a mode must be explicit. RT-57 routed the expiry values to a record, and AC-276 makes them a human per-class decision. ADR-0162 leaves the schema route "recorded and unowned". An operator decision is owed as a new ADR.
- Among the options, B is the smallest grant under which an agent can make a real edit.
- It keeps never-waits on the register's own mechanism: AC-215's unconditional `--permission-prompts none` plus closed stdin.
- It confines writes with the existing cwd = worktree plus the acceptEdits scope, now verified from the child's own report rather than inferred.
- Routing schemas through EXPORTED is the least-RECORD-change path: it satisfies ADR-0084, ADR-0095 and AC-133 as written and removes the run.rs/binder-doc contradiction, where hand-authored JSON would force a supersession.
- A adds a diff applicator on top of the landing path every option needs, and it carries diff-fidelity failure.
- C gives up filesystem and command containment, and lets agents bypass the Toolchain port's recorded verbs.
- B's load-bearing unknown is the acceptEdits and prompts-none interaction on CLI 2.1.278. It is exactly AC-215's prescribed integration test, so it is a probe with a Halt on failure.

FINDINGS TO CARRY INTO THE RECORD:
- Binding happens once per activity, from the first role-bearing step.
- Landing is owed separately.
- D21-40 must be committed before it is cited as authority.

## (h) ADR-0080 decision 2 / AC-527: an `expr` on a `kind = "check"` step that carries `on_false` and is spelled with quoted members, `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`. Is that the terminal condition ADR-0080 decision 2 forbids? If it is, what has to happen to the shipped `.redkiln/catalog`, to the AC-527 lint that never reads that catalog, and to the gap between AC-527's THEN and decision 2?

**TAKEN 2026-09-24:** A — execute ADR-0080 d2 as written, in any quoting; AC-527 brought back to d2 by dated amendment.

**Settled by record already:** True

**Binding records found:**

- ADR-0080 decision 2 (accepted). ADR-0111 supersedes it in part, decision 5 only: "Decisions 1–4 ... and every forbids bullet stand". ERRATA E-0080-1: "decisions 1–4 unchanged". Quote from d2: "The predicate `findings.none(evidence in [Reproduced, MutationSurvived])` is forbidden as a quality block's terminal condition"; "it must be one of: a **threshold** over severity or confidence — the terminal condition is 'no finding at or above threshold T survives re-execution' ... or a **different quantity entirely** — a bounded loop-back count, a re-execution outcome, or a delta"; "Lint rejects a policy predicate whose satisfaction requires an empty finding set"
- ADR-0080 What this forbids, quoted IN FULL (the prior packet elided the first clause): "Forbids `findings.none(evidence in [...])`, or any policy predicate satisfiable only by an empty finding set, as a quality block's terminal condition — in the catalog, in a technology pack, or in a shorthand expansion"
- ADR-0080 Context: "The catalog's policy check is `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`". This is the quoted spelling, on `spikes/p9/catalog/root-cause.toml` v4, the only graph F37 ran. The Evidence section says `root-cause` "is one of four `localisation` family members and the others were not run"
- ADR-0080 decision 3: "The threshold value itself is not set here". Decision 6 applies R4b only to "A policy check active at a rigor where its `on_false` target is not active"
- ADR-0103 decision 2: in value position an unquoted identifier is a literal only if it matches `[a-z_][a-z0-9_]*`. Decision 3: "A value that does not match the bare class must be quoted". Decision 4: "`all` and `none` over `null` or an empty list are **true**". Decision 6: `on_false` is closed over `{ to, max }` | "continue" | "fail" | deviate
- ADR-0031: "`on_false ∈ { <earlier-declared step>, continue, fail }` — loop-back only"
- ADR-0065 (R4b): "an `on_false.to` target must be active at every rigor at which the check that names it is active". It governs a TARGET, so it says nothing about `on_false = "continue"`
- ADR-0072 Decision: "Edits to shipped activities made for catalog coherence bump `version` and are listed in the pack changelog. The P6 lattice is the test that such an edit did not break reachability"
- ADR-0092 decision 1: "A correction that would move either is a supersession and takes a new record". An erratum corrects Context, Drivers, Options or Evidence and leaves every Decision sentence standing
- ADR-0092 decision 2, item 1: "A design row, a criterion or a runbook step that contradicts an accepted ADR is the defect; the ADR is not". Item 2: the register owns each THEN and Verified-by
- Design references/2026-09-08-architecture-design.md §6.6 row: "ADR-0080 forbids `findings.none(evidence in [\"Reproduced\", \"MutationSurvived\"])` as a terminal condition anywhere (quoted-literal form per ADR-0103)". Golden constraint 15: "may not terminate anything"
- references/2026-09-07-activity-graph-schema.md: "The verdict policy of §4.6 becomes a `check` step *inside* the review-bearing activities". Its worked example has `expr = "findings.none(evidence in {Reproduced, MutationSurvived})"` and `on_false = { to = "hypothesise", max = 2 }`
- AC register AC-527 THEN: "or any predicate whose truth over an **empty** findings set is `true`" ... "never on the absence of findings". Its Verified-by requires "a red fixture with `findings.all(...)` over an empty set, which must also fail"
- spikes/DECISIONS-2026-09-21.md D21-2: "whether an `expr` on a `kind = \"check\"` step carrying `on_false` is the *terminal condition* ADR-0080 decision 2 forbids is a reading of the record, RAISED and not taken"

**Measured state:**

Everything below was read at 4c282d7, read-only. Nothing was built or run.

1. Shipped rows. `grep -rnE "findings\.(none|all)"` returns 46 rows in `.redkiln/` and 46 in `spikes/p9/catalog`. Each catalog holds 102 files. The evidence-class `none` rows are these:
- `root-cause.toml`: `policy` check, v4, `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`, `on_false = { to = "hypothesise", max = 2 }`. The adversary is `when = "rigor >= recommended"`.
- `differential-debugging.toml`: the same expr, v2, `on_false = { to = "compare", max = 2 }`.
- `adversarial-review.toml`: `verdict` check, v3. `findings.none(evidence in ['Reproduced', 'MutationSurvived']) and (rigor < mandatory or findings.none(evidence in ['NarrowedPin']))` with `on_false = "continue"`. It has no adversary step, and the `override` gate and the `record` tool read `${review.outputs}`, not the verdict.
- `threat-model.toml`: `findings.none(evidence in ['Reproduced'])`, loop-back to `model`.
- `user-story-ac.toml` and `vertical-slicing.toml`: `check.failed == 0 and findings.none(evidence in ['Reproduced'])`, loop-back.
- `instrumented-trace-analysis.toml`: `cause.confidence == 'high' and findings.none(evidence in ['Reproduced'])`, loop-back.

`diff --strip-trailing-cr` against `spikes/p9/catalog` shows all seven identical except `threat-model.toml`, which differs only in `applicable_if`.

2. The standalone `findings.none(severity in ['blocking'])` rows are 7: checklist-review, code-review, design-review, design-review-lightweight, migration-dry-run, security-review, spike-decision-review. **6 of the 7 carry `on_false = "continue"`**; migration-dry-run carries `"fail"`. They are verdicts, not loop-backs.

3. The lint never reads the shipped catalog. `crates/rk-kernel/tests/it/quality_block.rs` holds `const SHIPPED_CATALOG: &str = "spikes/p9/catalog";`, `SHIPPED_VIOLATIONS: usize = 11` and `SHIPPED_INDETERMINATE: usize = 83`. `rigor.rs`, `review.rs` and `memo.rs` use the same constant. There is no `rk-catalog` crate, `grep quality_block crates/rk-cli` is empty, and the module's own table marks the CLI surface \"NO — ARMED\". Step 7e excludes AC-527 by name.

4. The 11 refused rows are INFERRED, and the count reconciles. There are 10 standalone `findings.none(...)` exprs: 7 severity, root-cause, differential-debugging and threat-model. Add adversarial-review, whose unbound-free conjunction evaluates to true, and the total is 11. The other 35 findings-reading rows read an unbound path, so they are indeterminate.

5. The existing AC-527 fixture 2, `terminal_condition_is_findings_all.toml`, is `expr = "findings.all(severity != 'blocking')"` and must FAIL. That is logically the same predicate as `findings.none(severity in ['blocking'])`.

6. Checkpoint 8 in `s8-exit-criterion.md` matches only the unquoted spelling and excludes only comments and `crates/rk-kernel/tests/it/evaluator.rs`. Its own comment records that an optional-quote pattern also hits `crates/rk-kernel/tests/fixtures/activities/terminal_condition_is_findings_none.toml` (a live `expr` line, the must-REFUSE fixture), as well as the three shipped rows.

7. `quality_block.rs` says reconciling AC-527 with d2 is owed: \"An erratum reconciling the two is OWED and is not written here\".

**Options:**

### A (corrected). Execute ADR-0080 d2 as written. The named predicate, in any quoting, on a loop-back check step is forbidden; AC-527's THEN and its Verified-by are brought back to d2; a d2-form assertion runs over .redkiln/catalog

Accepted records settle the core reading.

- The quoted spelling is the only well-formed way to write the predicate (ADR-0103 d2/d3). ADR-0080's Context calls that quoted string "The catalog's policy check", and design §6.6 says "(quoted-literal form per ADR-0103)".
- The block's policy step is a `kind = "check"` step with a bounded loop-back `on_false` (schema §5).
- So `root-cause` is forbidden outright: it is the very step F37 measured. `differential-debugging` is covered by the forbids bullet's "in the catalog", not by F37, which never ran it.

Under ADR-0092 d2 item 1, AC-527's THEN ("truth over an empty findings set is `true`") is the defect. It refuses what d2 permits: bullet 1's full form, "no finding at or above threshold T survives re-execution", is itself true over an empty set.

The amended rule refuses two things on a check step whose `on_false` is a loop-back or `fail`:
- `findings.none(evidence in [...])` in any quoting, alone or as a conjunct required for truth (the forbids bullet's own `[...]`);
- any predicate that a single-class witness set falsifies, where each witness finding carries only an `evidence` field in a blocking class (Reproduced, MutationSurvived).

Severity-only predicates read no `evidence`, so they stay lawful.

The four one-member `['Reproduced']` loop-back rows fall under the forbids bullet's `findings.none(evidence in [...])` and under the per-class witness, so they are repaired in the same pass.

`adversarial-review` is NOT settled by record. Its `on_false = "continue"` terminates and loops nothing, R4b (ADR-0065) governs only an `on_false.to` target, and golden constraint 15 forbids a predicate that terminates something. The operator confirms one of two dispositions in the D-entry:
- (i) rewrite its verdict anyway, so the forbidden spelling leaves the catalog and the widened guard can stay path-free;
- (ii) leave it, and exclude it from the widened guard by name, with this reason.

- *Record form:* Execution under accepted ADR-0080 d2 (with ADR-0103 d2/d3 and ADR-0092 d2 item 1). The record work:
- A dated in-place AC-register amendment to AC-527's THEN and its Verified-by, a register amendment rather than an erratum. Fixture 2 is recast so that it keeps its discriminator role on the named FORM with no shared text, e.g. `findings.all(not (evidence in ['Reproduced', 'MutationSurvived']))`, which must FAIL. A positive `findings.none(severity in ['blocking'])` is added, which must LOAD.
- A ledger D-entry that closes D21-2's RAISED reading and records the adversarial-review disposition.

There is no new ADR, because no Decision sentence or forbids bullet moves. There is no ADR erratum: a note glossing how the Decision's spelling "reads" would touch a Decision sentence, which ADR-0092 d1 bars, and ADR-0103 d3 plus design §6.6 already carry the reading.
- *Work:* (1) Rewrite the check `expr`s in `.redkiln/catalog` root-cause (v4→5), differential-debugging (v2→3), threat-model, user-story-ac, vertical-slicing and instrumented-trace-analysis, plus adversarial-review if disposition (i) is chosen. Each replacement is a d2 bullet-2 quantity (a re-execution outcome such as `check.failed == 0` over an existing tool step, or a loop-back count). None is a threshold, because d3 leaves T unset. Under ADR-0072, bump each version, write a changelog row and RE-RUN THE P6 LATTICE, which is ADR-0072's reachability test. Assert that the lattice loaded a non-zero pack count; a filter over zero packs is a green.

(2) Change `rk_kernel::quality_block`'s refusal as follows:
- Keep the named-form match.
- Replace 'true over empty' with 'false under some single-class blocking witness'.
- Restrict the subject to loop-back and `fail` checks, or to all checks if (i) is chosen.
- Re-pin 11/83 from inside the test binary.

(3) Add an assertion over `.redkiln/catalog` that must be GREEN. `spikes/p9/catalog` stays RED and pinned (§7b.12).

(4) Widen Checkpoint 8's pattern to optional quotes. Also exclude the must-REFUSE fixture path, and adversarial-review under (ii). Update `syn=` to the quoted form.

The `rk-catalog`/doctor CLI surface stays ARMED; say so, don't imply it is met. The `rigor_monotonicity` re-point is out of scope.
- *Size:* Medium: 6–7 catalog rows, each with a version bump, a changelog row and one lattice re-run; one lint change with a fixture recast; one register amendment; one D-entry; one checkpoint line. Roughly one slice.
- *Risks:* - The replacement predicates are authored while d3 leaves severity unsettled. Each must be a bullet-2 quantity and not a disguised threshold.
- Refusing the one-member `['Reproduced']` rows goes beyond F37's measurement: F37 measured "at least one finding in a blocking evidence class", not at least one Reproduced finding. The rows rest on the forbids bullet's `[...]`, which could also be read as shorthand for the two-member list. Disclose this in the D-entry.
- The witness change moves the pinned counts, so re-measure them; don't infer.
- Rewriting root-cause v4 detaches the shipped activity from F37's pinned artefact (`306b58ba7504`). Record that.
- Both adversarial-review dispositions carry a cost: (i) edits an inert verdict on a reading; (ii) keeps the forbidden spelling in the shipped catalog behind a named exclusion.
- *Citations:* references/adr/0080-quality-block-policy-predicate-is-not-a-terminal-condition.md: "Forbids `findings.none(evidence in [...])`, or any policy predicate satisfiable only by an empty finding set"; references/adr/0080-...md: "no finding at or above threshold T survives re-execution"; references/adr/0080-...md: "The catalog's policy check is `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`"; references/adr/0103-...md: "A value that does not match the bare class must be quoted"; references/adr/0065-lint-r4b-on-false-target-rigor-reachability.md: "an `on_false.to` target must be active at every rigor at which the check that names it is active"; references/adr/0072-shipped-activity-edits-bump-version.md: "The P6 lattice is the test that such an edit did not break reachability"; references/adr/0092-errata-precedence-and-cut-id-reissue.md: "a criterion or a runbook step that contradicts an accepted ADR is the defect; the ADR is not"; references/2026-09-08-architecture-design.md: "(quoted-literal form per ADR-0103)"; crates/rk-kernel/tests/fixtures/activities/terminal_condition_is_findings_all.toml: `expr     = "findings.all(severity != 'blocking')"`; crates/rk-kernel/tests/it/quality_block.rs: `const SHIPPED_CATALOG: &str = "spikes/p9/catalog";`

### B. Adopt AC-527's stricter rule on the record: a quality block terminates only on a positive evidence record

Keep AC-527's THEN as it stands ("never on the absence of findings") and make it the decision by superseding ADR-0080 d2 in part. Every `none`/`all` over findings that is true on an empty set would be forbidden as a terminal condition, severity thresholds included, and d2 bullet 1's threshold route would be withdrawn. The bounded-loop-back-count and delta shapes in bullet 2 would need re-examining too, since a delta against an empty previous iteration can also be true on empty.

- *Record form:* A new ADR-0164 superseding ADR-0080 decision 2 IN PART: bullet 1's permitted shape and the 'requires an empty finding set' sentence both move. ADR-0080 stays accepted, with `superseded_by` gaining 0164. Design §6.6 and golden constraint 15 are amended to match, and a D-entry is added.
- *Work:* The lint witness stays 'true over empty'. Rewrite all 11 refused shipped rows, 7 of them severity verdicts (6 of those under `continue`). Decide on or rewrite the 35 indeterminate findings-reading conjunctions. Up to ~46 rows, each with an ADR-0072 bump and a P6 lattice re-run. Re-point the lint at `.redkiln/catalog` and widen Checkpoint 8.
- *Size:* Large: a new ADR, design amendments and up to ~46 activity rewrites. Several slices, and it likely postpones the Phase 1 exit.
- *Risks:* It withdraws the only remedy shape ADR-0080 prescribed for reviews while d3's severity vocabulary is still unsettled, so it decides on no new evidence. F29 ("a reproduced regression was approved 3 of 10 times") is a reviewer-reliability result, not the constant-function mechanism. It also has a large blast radius on a pack Step 7e just closed.
- *Citations:* references/2026-09-08-acceptance-criteria.md AC-527: "never on the absence of findings"; references/adr/0080-...md: "Lint rejects a policy predicate whose satisfaction requires an empty finding set"; references/adr/0092-...md: "A correction that would move either is a supersession and takes a new record"; crates/rk-kernel/src/quality_block.rs: "A rule that refuses the record's own repair is worse than no rule"

### C. Execute only the named predicate now and leave the AC-527/d2 reconciliation raised

Take only what the records settle beyond reading: the two-member predicate in any quoting, on a loop-back check step, is forbidden.
- Repair root-cause and differential-debugging.
- Widen Checkpoint 8's guard to the quoted spelling, excluding the must-REFUSE fixture and, unless it is rewritten, adversarial-review.
- Add a named-predicate assertion over `.redkiln/catalog`.

The AC-527 form lint stays on `spikes/p9/catalog`, RED and pinned. AC-527 is reported as partially met at the Phase 1 exit.

- *Record form:* Execution under accepted ADR-0080 d2 plus a ledger D-entry. AC-527 is carried as a named, open register defect under ADR-0092 d2 item 1.
- *Work:* 2 or 3 catalog `expr` rewrites, each with an ADR-0072 bump and a P6 lattice re-run. Widen one checkpoint pattern (excluding the fixture path, and updating `syn=`). Add one named-predicate assertion over `.redkiln/catalog`.
- *Size:* Small: a few hours.
- *Risks:* AC-527's own instrument still never runs over the product artefact. The register keeps a criterion that contradicts its governing ADR, and it is already known that AC-527 refuses six `continue` severity verdicts d2 permits. The four one-member `['Reproduced']` loop-back rows stay unaddressed. A string-level guard does what the module warns against: "the rule is on the FORM, not the string".
- *Citations:* runbook/phase-1/s8-exit-criterion.md: "This pattern matches only the UNQUOTED spelling, and the shipped catalog writes the predicate with QUOTED members"; spikes/DECISIONS-2026-09-21.md: "RAISED and not taken"; runbook/phase-1/s7e-pack-authoring.md: "it is **excluded**, by name and with that reason"

**Recommended:** A (corrected)

**Why:** DEFECTS FOUND IN THE PRIOR PACKET:
(1) The forbids-bullet quote left out its first clause, "Forbids `findings.none(evidence in [...])`". That clause bears directly on the one-member `['Reproduced']` rows, which the packet called the only open residual.
(2) Option A contradicts itself. It keeps the three existing fixtures, but fixture 2 is `findings.all(severity != 'blocking')` and must FAIL. It then adds a positive `findings.none(severity in ['blocking'])` that must LOAD, which is the same predicate. AC-527's Verified-by fixture 2 has to be recast, not kept.
(3) The proposed witness, "every findings set holding a blocking-class finding falsifies it", does NOT refuse `findings.none(evidence in ['Reproduced'])`, because a {MutationSurvived} set satisfies it. That contradicts the packet's own step (5). A literal 'satisfaction requires an empty set' test would not refuse even the named predicate, because an {Advisory} set satisfies it. The witness has to be per-class, with each witness finding carrying `evidence` only.
(4) The packet called adversarial-review's `continue` verdict "the case ADR-0080 d6/R4b names". That is false. R4b (ADR-0065) governs an `on_false.to` TARGET, and `continue` has none. A `continue` check also terminates nothing, so whether it is a "terminal condition" is exactly the reading D21-2 left open. It is not settled by record, and 6 of the 7 standalone severity rows are `continue` verdicts too.
(5) The packet said root-cause and differential-debugging are "identical to the files F37 ran". F37 ran only root-cause. ADR-0080's Evidence says the other localisation members "were not run". differential-debugging is covered by the forbids bullet's "in the catalog".
(6) The packet called the severity rows "d2 bullet 1's own prescribed threshold shape". That overclaims: bullet 1 is "no finding at or above threshold T survives re-execution", and the rows have no re-execution clause. The AC-527 contradiction survives anyway, because bullet 1's full form is also true over an empty set.
(7) ADR-0072's Decision also requires "The P6 lattice is the test that such an edit did not break reachability". The packet cited only the title and left out the lattice re-run.
(8) The E-0080-2 'reads as' note would gloss a Decision sentence. Under ADR-0092 d1, errata correct Context, Drivers, Options and Evidence only. It is unnecessary as well, since ADR-0103 d3 and design §6.6 already carry the reading. Dropped.
(9) The widened Checkpoint 8 pattern would also hit the must-REFUSE fixture `terminal_condition_is_findings_none.toml`, which is a live `expr` in `crates/`. The s8 comment names it, and the packet omitted the exclusion.
(10) Re-pointing `rigor_monotonicity` (PIA-035) is outside this question and not the least-change route. Dropped.
(11) The packet never said that a kernel test over `.redkiln/catalog` leaves AC-527's `rk-catalog` CLI surface ARMED. It now says so.
(12) Minor: the packet's "ADR-0092 rule 1" is the supersession sentence of decision 1, and "rule 2(1)" is decision 2 item 1. Relabelled.

The citations that survived verification are exact at 4c282d7: ADR-0080 Context, d2 and d3; ADR-0103 d2, d3 and d4; ADR-0111's \"every forbids bullet stand\"; design §6.6 and golden constraint 15; schema §5; AC-527; D21-2; the Checkpoint 8 comment; the quality_block constants; and the 46/102/11 counts.

WHY A (corrected): The core reading is settled by accepted records, so no new decision is owed on it. The quoted spelling is the only well-formed form under ADR-0103, ADR-0080's own Context names that quoted string as \"The catalog's policy check\", and the check step with a bounded loop-back is the schema's policy step. The direction of the AC-527 fix is also fixed by ADR-0092 d2 item 1: the criterion that refuses d2's own permitted threshold form is the defect. Amending the register toward d2 is execution, not supersession. B would take a supersession on no new evidence, and C leaves a known register contradiction and the four one-member loop-back rows untouched.

Two residuals are genuinely open, and the operator should confirm them in the D-entry:
(a) adversarial-review's `continue` verdict. Rewriting it and excluding it by name are both lawful.
(b) Whether the forbids bullet's `[...]` reaches the one-member `['Reproduced']` rows, which extends F37's measurement. The recommendation is to refuse them and disclose that.

## (i) Before Phase-1 exit under ADR-0092, how should three earlier checkpoints that are red or cannot pass be repaired: (1) Checkpoint 5's ChangeSummary TYPE ban, (2) Checkpoint 7b's F42 parse pin, (3) Checkpoint 7f's ARMED-lint literal? For each one, is the route a supersession, an erratum, or a re-pin with a record?

**TAKEN 2026-09-24:** A — proportionate: ADR superseding 0099 and 0135 in part for CP5; 7b re-pinned with a record; 7f executes D21-9.

**Settled by record already:** False

**Binding records found:**

- ADR-0092 rule 1 (references/adr/0092-errata-precedence-and-cut-id-reissue.md): an erratum leaves "`## Decision` and every bullet of `**What this forbids**` standing as written. A correction that would move either is a supersession"
- ADR-0092 rule 2.4: "a number is quoted from the spike file" that computed it. Rule 2.5, the runbook: "It decides execution order and nothing else; where it must decide, it says so in its header table"
- ADR-0099 clause 5, a ## Decision sentence: "`ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`** until S-BIN reports; Checkpoint 5's grep for its absence covers `crates/` minus that one site."
- ADR-0099 What this forbids: "Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports." ADR-0099 is ALREADY superseded in part by 0115 and 0127 (frontmatter `superseded_by: ["0115", "0127"]`), so a new record appends a third back-link. ADR-0127 says explicitly that "every bullet of its forbids stand"
- ADR-0135 clause 5, a ## Decision sentence the packet quoted only as licence: "Checkpoint 5's grep bans the bare type outside `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths ... **Checkpoint 5's RED is untouched by this record.**" This present-tense description of the grep MOVES if Checkpoint 5's filter changes
- ADR-0135 What this forbids: "Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel` — in a signature, a binding, a generic argument or a return type." ADR-0135 is superseded_by 0148 only at clause 1's `test` row, so clause 5 and this bullet stand
- ADR-0094 decision 1 (a ## Decision sentence): "`classify` and `change_summary` over `cargo metadata` and the changeset — `change_summary` remaining an opaque non-finalised newtype until S-BIN (ADR-0093 decision 4)". It requires an implementor, and an implementor must write the return type
- ADR-0093 decision 4: "S-BIN's gate is scoped to the `ChangeSummary` type, exactly as ADR-0008 words it." Its forbids: "Forbids citing S-BIN as a block on any Phase-1 commit outside `ChangeSummary`'s type design and the Toolchain port's `change_summary` body"
- Ledger D17-9 (spikes/DECISIONS-2026-09-17.md), an operator decision the packet MISSED: "both toolchain adapters' `change_summary` return `Err(StepError::Unsupported { capability })` from a **dated capability-registry entry**" ... "`ChangeSummary` stays S-BIN's (ADR-0099 decision 5's forbids, unchanged; the type-scoped grep of D17-1)". The operator already chose to have implementors while declaring the forbids bullet unchanged. That choice is the collision
- Ledger D21-10 (spikes/DECISIONS-2026-09-21.md): "`change_summary` reaching the port and receiving `Unsupported` is the correct behaviour and is licensed here explicitly" ... "Checkpoint 5's RED is" [untouched]
- PROGRESS §2 D17-1 readings 2 and 3 (runbook/PROGRESS.md): runbook-only narrowings of the forbids bullet to "the name in CODE" (string literals, comments, and the `Capability::ChangeSummary` variant exempted). No ADR ratified either one
- ADR-0114 decision 1: "The two F42 parse failures and three `on_false` defects are repaired at Step 7e before the tag; nothing else stands between the v0 pack and its tag." 285/287 appears only under Evidence. No ERRATA.md section and no D2x ledger entry mentions 0114 (grep: 0 hits)
- s7b-kernel.md Checkpoint 7b comment: "THE THREE NUMBERS BELOW ARE VALID UNTIL s7e §7e.2 REPAIRS THE FIVE PACK DEFECTS ... s7e:117 expects 287/287 afterwards. If any of them moves, RE-DERIVE from F42 and record the new reading — do not widen the pattern"
- s7e-pack-authoring.md: "The `287/287` this section carried is unreachable" ... "the count settles at **278/279**" ... "**96/99 is the ceiling**"
- Ledger D21-9: Checkpoint 7f "requires the literal `test result: ok. 0 passed; 0 failed; 3 ignored` and three ARMED lints, so **any** discharge invalidates it" ... "Checkpoint 7f is `s7g`'s entry gate only; nothing downstream requires it green"
- 00-preamble.md: "a pass is evidence about a specific text, not about a stage"
- ADR-0160 and s7h-lane-completes.md: guards "assert the ARMED lint still names `Slice-Verdict` as the sole" unproduced key
- Design §9.1 crate row (references/2026-09-08-architecture-design.md): "`rk-ports` — `ChangeSummary` (an opaque, non-finalised newtype, the one site Checkpoint 5's absence grep excludes — ADR-0099 clause 5)". A supersession owes a dated design note here

**Measured state:**

Read at 4c282d7 with read-only git and grep. I ran no cargo, no redkiln and no probe. I did run Checkpoint 5's own filter chain and s8's regex as plain greps.

(1) CHECKPOINT 5.
- Checkpoint 5's filter chain returns SIX hits (MEASURED). They are the `use` line and the `fn change_summary(...) -> Result<ChangeSummary, StepError>` signature in each of rk-adapter-toolchain-node/src/lib.rs, rk-adapter-toolchain-rust/src/lib.rs and rk-cli/tests/it/review_pass.rs. The review_pass.rs pair postdates the tracker's count. PROGRESS row 5, the ledger (\"stays RED with four hits in the two toolchain adapters\") and s8's comment all still say four.
- CORRECTION: the `::ChangeSummary` loophole is not hypothetical. It has already been taken at TWO committed sites that the filter swallows:
  - crates/rk-cli/src/verbs/run.rs, production code: `impl rk_ports::toolchain::Toolchain for UndetectedToolchain` with `-> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError>`, landed at fbf3c4c.
  - crates/rk-kernel/tests/it/runtime.rs `CountingToolchain`, whose doc comment says: \"The return type is written `rk_ports::toolchain::ChangeSummary` and the name is NOT imported. Do not \\\"tidy\\\" this into a `use`\" ... \"would turn Checkpoint 5 red\".
- So eight code sites literally violate ADR-0099's forbids bullet, not six. There are five `impl Toolchain` outside rk-ports: node, rust, UndetectedToolchain, CountingToolchain and ScriptedToolchain. Every site is an import or an impl signature of the port's own method.
- The rk-kernel site also literally violates ADR-0135's forbids (\"anywhere in `rk-kernel` ... a return type\"). This is PLAUSIBLE: the bullet does not say whether tests/ is in scope. The test double cannot move into rk-ports, because Checkpoint 7a pins exactly 5 port impls there (runtime.rs doc: \"A sixth column-0 `impl Toolchain for` in that directory turns a DIFFERENT, already-closed checkpoint red\").
- s8's definition/construction regex is TRUE of the tree (MEASURED). Its only hits are in rk-ports: `pub struct ChangeSummary(pub(crate) ())`, fakes.rs `Ok(ChangeSummary(()))`, one doc line and one test string. No `type X = ...ChangeSummary` exists anywhere.
- CORRECTION: rk-ports' doc comment concedes the name only inside rk-ports (\"on `change_summary`'s return type and on the fake's impl too\", where the fake is rk-ports' own fakes.rs). It also still says \"no real adapter implements it at this stage\" and \"This is the one site in `crates/` the name may appear\", which is stale.
- The red is a conflict among accepted records. ADR-0094 d1 plus operator decision D17-9 require implementors. ADR-0099's forbids bullet bans the name. ADR-0135 clause 5 and its forbids describe and extend the ban. An erratum cannot fix it, because ADR-0092 rule 1 protects both Decision sentences and both forbids bullets. PROGRESS row 5 itself names this fork: \"clause 5 needs a fourth reading that admits an implementor of the port's own method\".

(2) CHECKPOINT 7b.
- s7b still pins `grep -qxF 'predicates parsed 285/287; failed 2/287'`, `'mixed unparenthesised and/or (precedence-dependent) 0/287'` and `^on_false forms 96/99 in the closed set`.
- spikes/p9/results-s-grammar.json was last written at dddcdbd (MEASURED). It reads: by_kind when 240, on_false 99, applicable_if 39; parsed 278; failed 1, which is catalog-b/blameless-postmortem.toml `-vuln`; mixed 0; on_false_ok 96, with three on_false failures, all catalog-b's.
- grammar-probe.py prints `predicates parsed {ok}/{n_pred}; failed {len(fails)}/{n_pred}`, so the new exact lines are 278/279, 1/279 and 0/279. The first two pins are red; 96/99 holds.
- The comment's \"s7e:117 expects 287/287 afterwards\" is stale: s7e now says \"The `287/287` this section carried is unreachable\". The FAIL message still names exists(ProblemRecord), which was repaired.
- dddcdbd touched 9 p9 catalog and pack files: 3 catalog and 6 pack (MEASURED).
- F42-DRAFT.md still reads \"**285/287** parse; 2 fail\", with no dated correction.
- ADR-0114 d1 is literally unmet: only 1 of its 5 named defects was repaired, and no record addresses it (MEASURED).
- Running the probe `json.dump`s into the tracked JSON.

(3) CHECKPOINT 7f.
- conformance.rs has two un-ignored conformance lints and one `#[ignore = \"ARMED: Slice-Verdict has 0 producers; Gate and Story are both minted as of s7h.2; discharged by s7h\"]` (MEASURED).
- The `2 passed; 0 failed; 1 ignored` figure is PROGRESS row 7f's 2026-09-22 reading. It was not re-measured here, and conformance.rs has changed since (6d3b6e0).
- PROGRESS row 7f already says \"in replay order — 7f before `s7g`'s work lands — it still passes, so the recorded pass stands as evidence about the text it attested\". Its status cell still reads \"RE-RUN PENDING\".
- D21-9 already decides the disposition.

**Options:**

### A. Proportionate: one supersession covering 0099 and 0135 in part for (1), a re-pin with record for (2), and execution of D21-9 for (3)

(1) ADR-0164 restates the ChangeSummary rule as the TYPE's home and opacity. ChangeSummary is defined only in rk-ports, stays opaque (`pub(crate)` field) and non-constructible, and no crate outside rk-ports defines, aliases, finalises, constructs or implements traits for it (`impl ... for ChangeSummary`, `impl ChangeSummary`). An implementor of `Toolchain::change_summary`, in production code or a test double in any crate including rk-kernel's tests/, may name it in that method's signature and import it. So may the port's own call through a method call. The record supersedes in part:
- ADR-0099 clause 5's second sentence and its forbids bullet;
- ADR-0135 clause 5's grep-describing sentence, and its forbids bullet as far as it reaches a test double's impl signature (rk-kernel production code stays banned from naming the type).
It also ratifies PROGRESS D17-1 readings 2 and 3 (string literals, comments and the `Capability::ChangeSummary` variant are not the type). Those readings were runbook-only narrowings that the next adversarial pass would otherwise find. Clause 5's first sentence and ADR-0093 d4 stand. (2) Re-pin Checkpoint 7b to `predicates parsed 278/279; failed 1/279` and `mixed unparenthesised and/or (precedence-dependent) 0/279`. Keep `on_false forms 96/99` and keep the exact -qxF form. Rewrite the stale "287/287 afterwards" comment and the FAIL message so they name catalog-b's single `-vuln` failure. Add a dated F42 correction under rule 2.4. Name ADR-0114 d1's literal non-fulfilment as a separately OWED record (a supersession-in-part or an operator reading), not folded into the re-pin. (3) Under D21-9, change PROGRESS row 7f's "RE-RUN PENDING" to a terminal disposition that quotes D21-9. There is no checkpoint edit. Optionally replay at 79e0ef4 in a throwaway worktree (check out, then overlay).

- *Record form:* (1) New ADR-0164 with `supersedes: ["0099", "0135"]` in part. Back-links: 0099 `superseded_by` becomes ["0115","0127","0164"], and 0135's becomes ["0148","0164"]. Precedents are ADR-0132 over 0073 at two sites and ADR-0148 over 0135. Also: a ledger entry for the operator's acceptance; a dated design note on §9.1's "the one site Checkpoint 5's absence grep excludes" row. (2) No ADR. An execution-order re-pin under ADR-0092 rule 2.5 and the checkpoint's own licence, a dated F42 correction, and a PROGRESS 7b entry. The ADR-0114 d1 residual is recorded as OWED. (3) Execution under the accepted D21-9 and ADR-0134, as a PROGRESS cell edit only.
- *Work:* (1) No behavioural code change. Checkpoint 5 gets reading 4:
- Replace the name-in-code ban with a definition, alias, construction and trait-impl regex over crates/ minus rk-ports: s8's `\b(struct|enum|type|trait)\s+ChangeSummary\b|ChangeSummary\s*[({]`, plus `type\s+\w+\s*=.*\bChangeSummary\b` and `(for|impl)\s+(\w+::)*ChangeSummary\b`.
- Drop the `::ChangeSummary` exclusion.
- Add an rk-ports pin that the field stays `pub(crate) ()`. The grep excludes rk-ports, so opacity there is otherwise unguarded.
- Add synthetic controls: an impl signature passes, both bare and path-qualified; a construction, an alias, a definition and an `impl X for ChangeSummary` each fire.

Then update the stale texts:
- rk-kernel/tests/it/runtime.rs's doc comment, which justifies its path-qualified spelling by the exclusion being removed;
- rk-ports' toolchain.rs doc ("the one site", "no real adapter implements it");
- s8's "RED with four hits" comment and PROGRESS row 5 (really eight literal sites, six through the filter);
- README index, lifecycle, next-free and census counts.

Re-run Checkpoint 5 WHOLE.

(2) Edit three s7b lines plus the comment above them. Add the F42 correction. Re-run Checkpoint 7b WHOLE, and confirm afterwards that results-s-grammar.json is unchanged. Compare the parsed numbers or raw bytes, not `git diff` alone, because autocrlf normalises line endings. (3) Edit one PROGRESS cell.
- *Size:* (1) M: an ADR of about 150-200 lines with two supersession targets and back-links on two ADRs, about 25 checkpoint lines with controls, 3 doc-comment edits, a design note, a README recount, a ledger entry and one whole run. (2) XS-S. (3) XS. In total one working session across 2-3 commits.
- *Risks:* (1) The ban is widened, so the synthetic controls carry the whole weight. The admitted class must be exactly "impl signature of the port's own method"; anything broader re-creates the vacuity. If ADR-0135 is not named, the next pass finds that a checkpoint no longer matches 0135 clause 5's Decision sentence, and that a rk-kernel file names the type in a return type. The README census and contiguity sentences have gone stale at every recent flip. (2) The probe rewrites a tracked JSON on every run, and on a CRLF-smudged checkout the bytes can change while `git diff` looks clean. Re-pinning while ADR-0114 d1 stays literally unmet leaves a record residual, so name it. (3) A terminal disposition can read as dropping the preamble's re-run obligation, so quote D21-9 in the cell.
- *Citations:* references/adr/0092-errata-precedence-and-cut-id-reissue.md: "A correction that would move either is a supersession"; references/adr/0099-...: "Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports."; references/adr/0135-...: "Checkpoint 5's grep bans the bare type outside `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths"; references/adr/0135-...: "Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel`"; references/adr/0094-...: "`classify` and `change_summary` over `cargo metadata` and the changeset"; spikes/DECISIONS-2026-09-17.md D17-9: "both toolchain adapters' `change_summary` return `Err(StepError::Unsupported { capability })`"; crates/rk-kernel/tests/it/runtime.rs: "The return type is written `rk_ports::toolchain::ChangeSummary` and the name is NOT imported"; crates/rk-cli/src/verbs/run.rs: "impl rk_ports::toolchain::Toolchain for UndetectedToolchain"; runbook/phase-1/s8-exit-criterion.md: "NARROWED TO THIS BULLET'S OWN WORDS"; runbook/PROGRESS.md row 5: "clause 5 needs a fourth reading that admits an implementor of the port's own method"; runbook/phase-1/s7b-kernel.md: "If any of them moves, RE-DERIVE from F42 and record the new reading — do not widen the pattern"; runbook/phase-1/s7e-pack-authoring.md: "the count settles at **278/279**"; spikes/p9/results-s-grammar.json: "parsed": 278; references/adr/0114-...: "The two F42 parse failures and three `on_false` defects are repaired at Step 7e before the tag"; spikes/DECISIONS-2026-09-21.md D21-9: "nothing downstream requires it green"

### B. No new ADR: conform the code to ADR-0099 literally, and re-scope 7f to a form that holds on any tree

(1) Add `pub type ChangeSummaryResult = Result<ChangeSummary, StepError>;` in rk-ports and make the trait return it. The five implementors outside rk-ports then name only the alias. The path-qualified sites in run.rs and runtime.rs would also have to switch, because the alias does not remove the existing `::ChangeSummary` spellings unless they are edited. (2) The same re-pin as A. (3) Rewrite s7f's block to a state-invariant form.

- *Record form:* (1) Presented as execution under ADR-0099. The record-layer failure is that it silently departs from D17-9's own statement ("the type-scoped grep of D17-1"). (2) As A. (3) A runbook amendment to a closed stage's checkpoint.
- *Work:* (1) Edit rk-ports toolchain.rs, which makes Checkpoint 7a provisional. Edit both adapters, which makes 7c provisional. Edit rk-cli run.rs and review_pass.rs, and rk-kernel runtime.rs, which reach 7g and 7h's subjects. Re-run 5, 7a, 7c, 7g and 7h whole. (3) Rewrite about 6 lines of s7f and re-run it.
- *Size:* M-L: the code is small, but four or five closed checkpoints reopen.
- *Risks:* (1) This games the grep's word boundary. `ChangeSummaryResult` still contains the name the forbids bullet bans "anywhere". It leaves the ADR-0094/D17-9 versus ADR-0099/0135 conflict unrecorded, and it leaves readings 2 and 3 unratified. (3) It rewrites what 79e0ef4 attested, and it collides with the trailer lint's "discharged by s7h" reason while s7h is CLOSED and ADR-0160 requires it to stay ARMED.
- *Citations:* references/adr/0099-...: "Forbids `ChangeSummary` appearing anywhere in `crates/`"; runbook/phase-1/s5-workspace-ci-rk-types.md: "grep -vE '::ChangeSummary\b'"; crates/rk-kernel/tests/it/conformance.rs: "discharged by s7h"; runbook/phase-1/s7h-lane-completes.md: "assert the ARMED lint still names `Slice-Verdict` as the sole"; runbook/00-preamble.md: "a pass is evidence about a specific text, not about a stage"

### C. No repair: carry all three into Step 8's exit record as residuals the operator accepts

Leave Checkpoints 5, 7b and 7f as they are, and write one ledger entry that waives them.

- *Record form:* A ledger D-entry as an operator waiver. No ADR and no erratum. The ADR-0094/D17-9 versus ADR-0099/0135 conflict stays unrecorded.
- *Work:* Correct the tracker counts only. PROGRESS, s8 and the ledger say four; the filter gives six, and eight sites violate the forbids bullet literally.
- *Size:* XS
- *Risks:* It contradicts the operator's stated aim. Checkpoint 5 stays red for the rest of the phase, with a documented evasion in runtime.rs, and 7b fails on a stale message. Two accepted ADRs and one ledger decision stay in literal conflict.
- *Citations:* runbook/PROGRESS.md row 5: "It stays RAISED and is not repaired here"; spikes/DECISIONS-2026-09-21.md: "It does not discharge **Checkpoint 5**, whose broader `ChangeSummary` TYPE ban stays"

**Recommended:** A (corrected): ADR-0164 superseding 0099 AND 0135 in part and ratifying D17-1 readings 2 and 3; a re-pin with record for 7b, with the ADR-0114 d1 residual named as owed; execution of D21-9 for 7f.

**Why:** DEFECTS FOUND IN THE PACKET:
(a) It missed a binding accepted ADR. ADR-0135 clause 5 is a ## Decision sentence that states what Checkpoint 5's grep does (\"excluding `::ChangeSummary` paths\") and ends \"Checkpoint 5's RED is untouched by this record\". Its forbids bans the type \"anywhere in `rk-kernel` ... a return type\". Option A changes the grep and admits test doubles, which moves both protected sites. Under ADR-0092 rule 1 that is a supersession of 0135 in part too, and the packet cited 0135 only as a licence.
(b) It missed operator decision D17-9. That decision chose adapter implementors of `change_summary` while declaring 0099's forbids \"unchanged\". This is the root of the conflict and the strongest warrant for A.
(c) A measured-state claim is false. The `::ChangeSummary` loophole was presented as a future risk (\"must not be taken\"), but it has already been taken at two committed sites. One is production code in rk-cli run.rs (`UndetectedToolchain`). The other is rk-kernel/tests/it/runtime.rs (`CountingToolchain`), whose doc comment deliberately avoids Checkpoint 5. So the literal forbids-bullet violations number eight, not six, and the rk-kernel site also plausibly violates ADR-0135's forbids.
(d) It omitted that ADR-0099 is already `superseded_by: [\"0115\", \"0127\"]`, and that 0127 affirmed \"every bullet of its forbids stand\". The back-link is an append, and ADR-0164 must say it overrides a forbids bullet that a later record explicitly kept.
(e) It overstated a quote. rk-ports' doc comment concedes the name only inside rk-ports (\"the fake's impl\" is rk-ports' own fakes.rs), not for outside implementors. The comment is also stale (\"no real adapter implements it at this stage\").
(f) It miscited a section. \"design §9.2\" does not hold the \"gates the type and the port's `change_summary` body, nothing else\" rule; that text sits in the migration section (around \"the port's `change_summary` body, nothing else in Phase 1\"). §9.2 is Dependencies. The design text that a supersession actually moves is §9.1's crate row, \"the one site Checkpoint 5's absence grep excludes\", which owes a dated note.
(g) Option A's regex had gaps:
- it did not catch `impl Trait for ChangeSummary` or `impl ChangeSummary` outside rk-ports (orphan rules permit the first);
- it left rk-ports' own opacity unguarded, because the grep excludes rk-ports;
- it did not ratify PROGRESS D17-1 readings 2 and 3, which already narrowed the forbids bullet (string literals, comments, the `Capability::ChangeSummary` variant) with no record.
(h) The 7b byte check was a proxy. `git diff --exit-code` under autocrlf is not byte equality, so compare numbers or raw bytes. The stale comment \"s7e:117 expects 287/287 afterwards\" was also left out of the edit list.
(i) A minor overclaim: the packet stated `2 passed; 0 failed; 1 ignored` as if measured at HEAD. It is a tracker reading from 2026-09-22, and conformance.rs changed afterwards (6d3b6e0).

WHAT SURVIVES:
- The six-hit filter count (MEASURED).
- s8's definition/construction regex holding true (MEASURED).
- The 7b numbers 278/279, 1/279, 0/279 and 96/99 (MEASURED from the JSON and the probe's print format).
- ADR-0114 d1 being literally unmet with no record (MEASURED).
- D21-9 deciding 7f.
- The 7f trailer-lint and ADR-0160 interaction.

RECOMMENDATION: A, corrected, remains the least-change lawful route.
(1) A supersession is required. Five implementors outside rk-ports are mandated by ADR-0094 d1, D17-9 and ADR-0139's refusing toolchain, and 7g's test doubles cannot move into rk-ports without turning Checkpoint 7a red. Meanwhile 0099's and 0135's protected sites ban the name. One ADR-0164 can restate the rule as the type's home and opacity and supersede 0099 and 0135 in part. B games the word boundary and reopens four or five closed checkpoints.
(2) No Decision sentence pins 285/287, and the checkpoint licenses its own re-derivation. Keep ADR-0114 d1's unmet sentence as a named, separately owed record.
(3) D21-9 already disposes of it; only the tracker cell remains.
already_decided_by_record stays false, because (1) needs a new record.
