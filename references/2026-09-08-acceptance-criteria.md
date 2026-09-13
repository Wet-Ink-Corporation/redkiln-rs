# Redkiln-RS — the acceptance-criteria register

**Date:** 2026-09-08 · **Status:** Register, rev. 1, **amended 2026-09-09** to absorb F39 (P1 arm D scored) and ADR-0089 — §0.1, §0.4, §1.1, §1.2, §3.7 (AC-069, AC-070, AC-071, AC-072, AC-076, AC-079 and the new AC-520), AC-113, AC-148, AC-302, AC-347 and §15; **further amended 2026-09-10** to absorb F40 (P2's *perturbation* half scored; its advisory half unrun) — §1.2 row 2, §3's scope note, AC-068, AC-113, AC-434, the selection area's residual-risk list and §15.1. **The 2026-09-10 amendment adds no criterion and moves no count**: F40 changes *why* AC-068 is provisional, not what it asserts. **Further amended 2026-09-11 to absorb the pre-implementation audit (PIA-001…PIA-110):** the twenty-three ADRs 0092–0114 and the eighteen-plus-six decisions of `spikes/DECISIONS-2026-09-11.md` are absorbed — eleven criteria added (AC-521..AC-531, every one outside an exhausted range, §0.4), four PROVISIONAL labels moved (AC-026 and AC-428 lifted on recorded evidence; AC-284 and AC-312 gained on S-CLASS-REPLAY and S-SUBTLE-CB), the `StepError` set, the memo key, the idempotency key, the schema rule, the containment model, the headroom contract (ticks), the review-gate vocabulary, the companion kind list and the facet `source` classes restated to the records that fix them, every `decisions: ADR-0004` re-pointed to ADR-0074 and the §15 tables regenerated from the records' own frontmatter, and the §0.3 table gained rows C4 and C5 and six twin rows. Under ADR-0092 this register is the in-tree owner of each criterion's GIVEN/WHEN/THEN, instrument, state label and id; the thirteen area files it was assembled from are history, not owners. Binding on the Phase 1 build runbook; it decides nothing new and supersedes no ADR. · **Companion:** the success-measure register (SM-001..SM-154 **plus SM-157**, the derived-facet precision measure the 2026-09-09 amendment added — the measure bodies live there, not here) and the ADR set at `references/adr/` (with `references/adr/ERRATA.md`, cited here by erratum id where a frozen number moved) ·
**Inputs:** the thirteen area files; the condensed evidence brief of 2026-09-08 — `references/2026-09-08-evidence-brief.md`, a working condensation of findings F1–F38 (F9 struck, F30 unassigned) that every area refuted against — **those findings are in two places**: `spikes/FINDINGS.md` holds **F1–F28**, and **F29 and F31–F40 exist only as `spikes/F<NN>-DRAFT.md`**, never folded back into the running log [measured, `ls spikes/F*-DRAFT.md`, 2026-09-09]; the spike files it condenses remain authoritative where they disagree with it; `references/2026-09-05-architecture-proposal.md` rev. 4; `references/2026-09-07-design-recommendation.md`; the process-catalog proposal; `references/adr/`; `spikes/results/` (including `s-mig-report.md` and `spikes/corpus/PINNED_SHA`); `references/2026-09-01-shipmates-lessons-for-redkiln-rs.md`. **Absorbed after the brief was condensed and therefore read from the corpus, not from the brief:** `spikes/F39-DRAFT.md`, `references/adr/0089-facet-provenance-declared-surfaces-and-abstention.md`, `references/2026-09-09-change-classification.md`, and — for the 2026-09-10 amendment — `spikes/F40-DRAFT.md` with the P2 instrument decisions it rests on in `spikes/DECISIONS-2026-09-09.md` (#15, #16, #19) and the pre-registration it reports against, `spikes/METHOD-P.md` §2.2–§2.4 ·
**Evidence discipline:** every criterion carries an evidence tag — **[measured]** = a number that is in the evidence brief, or (for the 2026-09-09 and 2026-09-10 amendments only) in `spikes/F39-DRAFT.md` and `spikes/F40-DRAFT.md`, both of which postdate the brief · **[documented]** = a platform, ADR or proposal contract with no measurement behind it · **[inferred]** = a design consequence, labelled as such. Two areas add a third tag where it earns its place: **[motivating, not evidential]** (git-reconcile) and **[surface unverified]** (harness). A finding may be tagged `[measured]` against a criterion only when the measurement is about the same quantity the criterion constrains. The refutation pass cut every citation that failed that test and **named the cut inside the criterion that carried it**, so the cut is auditable rather than invisible. Where a result is one draw, one repo or one operator, it says so. Unrun spikes are listed as unrun in §1, never assumed.

---

## 0. How to read this

| term | what it means **here** |
| --- | --- |
| **an acceptance criterion** | One GIVEN/WHEN/THEN naming **one** observable behaviour of the built system, with a named instrument that can fail. It is not a requirement, a design note or a measurement. A criterion that restates a measurement, or that a system doing nothing would also satisfy, was cut or repaired — five were cut outright and the cuts are recorded in §0.4. |
| **Verified by** | The instrument that decides the criterion. **Almost none of these instruments exists yet**: `crates/` is empty and the repository has no CI workflow, so 'Verified by' names work the build must do. A criterion whose instrument never reaches the build runbook is unverifiable in practice however well it reads here. |
| **PROVISIONAL** | The criterion's *warrant* rests on a spike that is unrun or partial. It names the spike. A PROVISIONAL criterion may be implemented, but it must not be counted as covered, and if its spike never runs it should be demoted to an open question rather than shipped as an assurance. |
| **CONDITIONAL** | Weaker than PROVISIONAL, and used in one area only (ports-dist): the criterion binds **only if a decision is taken that has not been taken**. Six distribution criteria are conditional on a distribution mechanism with an install-and-repair step being decided; proposal §4.9 decides no such mechanism. |
| **HUMAN GATE** | The question is not machine-decidable and is routed to a named human, with the reason stated. Each such criterion also names its machine-checkable residue — usually that the decision record exists and is attributed. Twenty criteria are gates; they contribute no machine assurance. |
| **a success measure** | A quantity with an instrument, a baseline (or an honest statement that there is none), a target and a **kill condition that can fire**. Measure bodies live in the companion register; this document cross-references them by id only. |
| **the `rk-*` label after an id** | A **subject tag**, not a crate name (ADR-0099 clause 5; PIA-010/PIA-038). Design §9.1 is the crate-name authority; the tags map to it as follows — `rk-select`, `rk-catalog`, `rk-packs`, `rk-policy`, `rk-kernel-verify` → modules of **`rk-kernel`**; `rk-store-artifacts` → **`rk-store`**; `rk-port-toolchain` → **`rk-ports`**; `rk-doctor` → **`rk-cli`**; `rk-adapter-claude`, `rk-adapter-sqlite`, `rk-adapter-toolchain-node`, `rk-adapter-toolchain-rust` (ADR-0094), `rk-git`, `rk-types`, `redkiln-migrate` are crates by their own names. A runbook checkpoint names `cargo test -p rk-kernel -- <module>` for a module tag. A crate is a later split, not a rename. |
| **the resolved (bound) card** | The dispatch card the binder materialises by injecting the **role's** tier (`model`, `effort`) into the authored card; the authored card file never carries `model` or `effort` (AC-118, AC-418), so a dispatch's tier is traceable only through its role. AC-212, AC-236 and AC-036 read "card" as this resolved card (PIA-015). |

### 0.1 The count, by area and by state

All thirteen area files were present and complete; **no area is missing or empty**, and nothing below is silently omitted.

| § | area | criteria | binding | provisional | human-gated | conditional | measures |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 2 | Kernel | 39 | 37 | 1 | 1 | 0 | SM-001..SM-010 |
| 3 | Selection engine | 41 | 39 | 1 | 1 | 0 | SM-013..SM-024, SM-157 |
| 4 | Process packs, technology packs, the activity catalog and its lint | 40 | 37 | 1 | 2 | 0 | SM-025..SM-036 |
| 5 | Typed artifacts and the store | 40 | 36 | 1 | 3 | 0 | SM-037..SM-046 |
| 6 | Git as truth | 41 | 39 | 1 | 1 | 0 | SM-049..SM-058 |
| 7 | Harness port and the `claude -p` adapter | 41 | 37 | 3 | 1 | 0 | SM-061..SM-070 |
| 8 | Human gate | 40 | 33 | 4 | 3 | 0 | SM-073..SM-082 |
| 9 | Review gate | 40 | 35 | 3 | 2 | 0 | SM-085..SM-096 |
| 10 | Verification gate | 40 | 37 | 1 | 2 | 0 | SM-097..SM-107 |
| 11 | Event log, cost attribution, telemetry and throttling | 40 | 38 | 1 | 1 | 0 | SM-109..SM-118 |
| 12 | CLI surface and `doctor` | 41 | 40 | 0 | 1 | 0 | SM-121..SM-130 |
| 13 | `redkiln-migrate` and the clean break | 42 | 40 | 1 | 1 | 0 | SM-133..SM-141 |
| 14 | Ports, fakes, plugin generation and distribution | 40 | 31 | 2 | 1 | 6 | SM-145..SM-149, SM-151, SM-153, SM-154 |
| | **total** | **525** | **479** | **20** | **20** | **6** | **135 rows** |

**525 criteria in force.** 479 binding, 20 provisional, 20 human-gated, 6 conditional. **135 measure rows** are cross-referenced (134 measures plus SM-098, which the verify-gate pass demoted to a descriptive report and excluded from its own count).

**The 2026-09-11 amendment adds eleven criteria and moves four labels.** Added, each outside an exhausted range (§0.4): AC-521 (§13, story-lane self-hosting twin of AC-473 — ADR-0094), AC-522 (§13, `redkiln-migrate` refuses a dirty target and writes into its own branch/worktree — PIA-002), AC-523 (§7, `RateLimited { resets_at }` — ADR-0095), AC-524 (§8, fresh gate record on loop-back re-reach — ADR-0105, PROVISIONAL on P8-B), AC-525 (§3, `drift_advisory` — ADR-0025), AC-526 (§7, both TTL variables — ADR-0078), AC-527 (§4, the forbidden empty-set quality-block predicate — ADR-0080), AC-528 (§14, no `async fn` in `rk-ports` and no async runtime in `rk-kernel`'s graph — ADR-0076), AC-529 (§6, the run-owned worktree refusal — ADR-0105/ADR-0018), AC-530 (§9, `advisory_retired` accounting — ADR-0082), AC-531 (§12, `init`'s `rk-` namespacing — the clause AC-437 cut). Labels moved: AC-026 and AC-428 binding (their fixtures exist on disk — PIA-068, PIA-075); AC-284 PROVISIONAL (S-CLASS-REPLAY — ADR-0096) and AC-312 PROVISIONAL (S-SUBTLE-CB — decision #17). Net: +11 criteria, +10 binding, +1 provisional. The measure count is unchanged; new measures (SM-158+) are the companion register's.

**The selection row gained one criterion and one measure on 2026-09-09**, and they are the only changes to this table since rev. 1: **AC-520**, the derived-facet sibling gate that F39 forced (§3.7), and **SM-157**, the derived-facet precision measure whose scored record AC-520 gates on. Both are counted in the selection area because that is the area they belong to, and both ids sit outside that area's ranges because AC-040..AC-079 and SM-013..SM-024 are exhausted — §0.4 records the criterion-side hazard, and the companion register's §1 numbering paragraph records the measure-side one. The measure total therefore moves 134 → 135, matching the companion register's own count.

**The 2026-09-10 F40 absorption changes no cell of this table.** P2's perturbation half is scored, AC-068 stays PROVISIONAL for a different and now-named reason, and **no criterion was added** — the reasoning for absorbing F40 into AC-068 rather than splitting a sibling out of it is stated in §3's 2026-09-10 scope note, and it is deliberately not the F39/AC-520 shape.

Read that total with four deductions in front of it.

1. **Every instrument is prospective.** No criterion in this register has ever been run. The number counts specified checks, not coverage.
2. **Twenty human gates and six conditionals contribute no machine assurance**, and a further set of criteria are negative guards a system doing nothing also satisfies (the harness area names four of its own; events-cost names six). The count of criteria that can actually catch a regression on day one is materially below 469, and no area was able to state that number for itself.
3. **The measures are thinner than 135 suggests.** Every area's refutation found measures that saturate at zero by construction, that carry no firing kill, or that have no baseline at all. Where an area labelled a measure a *discovery instrument*, a *tracker*, a *tripwire* or a *descriptive report*, the label is reproduced in the per-area measure table's `note` column and must travel with it.
4. **Twenty-four of the 112 decisions are cited by no criterion** (§15.3, recomputed 2026-09-11 from every `decisions:` field — 112 is the record count with ADR-0092..ADR-0114; on 2026-09-08 it was 33 of 89, including the whole ADR-0074..ADR-0088 block). Eleven of that block's fifteen are now cited by the criteria that carry their rules; the rest of the uncited set is one premise, three superseded or proposed records, two conventions and seventeen schema/grammar amendments. The behaviour is mostly covered; the citation layer is closer to the behaviour than it was.

### 0.2 What this register is not

It is not a design document and not a build plan. It preserves the criteria **as authored and as refuted** — the assembly fixed formatting and recorded duplication across areas (§0.3), and changed no GIVEN, WHEN, THEN or instrument. Where two areas assert the same behaviour, both texts are kept and the better one is named as owner; where two areas **contradict**, neither was altered and the contradiction is raised in §0.3 for a human to settle.

### 0.3 Cross-area duplication, and five contradictions

The areas were authored independently against one proposal, so the same behaviour is sometimes asserted in two or three places with different instruments. None of these is a defect on its own; all of them are a traceability hazard, because a build that satisfies the owner and skips the twin will read as green in both areas. **Owner** is the criterion whose GIVEN/WHEN/THEN and instrument are strongest; **twins** are kept for their area-local instrument and must be read as the same behaviour.

| behaviour | owner | twins | disposition |
| --- | --- | --- | --- |
| A gate's single-use token: a second answer is refused | **AC-246** | AC-177, AC-407 | AC-177 is explicitly the reconcile-side half; AC-407 is the CLI channel. Same behaviour, three instruments. |
| One idempotent run entry point; no `--resume` flag | **AC-400** | AC-186, AC-264 | AC-400 is strongest — a committed denylist plus the argument snapshot plus a stated limit. The other two use the snapshot alone. |
| Store deleted, next action unchanged | **AC-161** | AC-365 | Identical GIVEN/WHEN/THEN. AC-161 binds equality to the canonical serialiser and names its dependency on AC-188. |
| `doctor --rebuild` reconstructs tier-1 from git alone | **AC-184** | AC-366, AC-427 | AC-184 carries the adversarial-store variant and the 40-hex corpus-pin assertion; AC-427 adds a next-action comparison AC-184 does not make. |
| One store, resolved at the git common dir | **AC-178** | AC-370 | AC-178 asserts the recursive no-second-store scan; AC-370 adds the stray-store `doctor` rule. Both halves are needed. |
| Store-identity and schema-version open refusals | **AC-179, AC-180** | AC-369, AC-413 | The git-reconcile pair is split by failure mode; the other two bundle them. |
| FSM replay yields the recorded action sequence | **AC-006** | AC-188 | The same criterion under two ids in two areas. AC-188 is load-bearing for four git-reconcile criteria; AC-006 is the kernel instrument. |
| `RateLimited` writes a durable mark and exits; never sleeps | **AC-030** | AC-222, AC-379, AC-404 | Four statements of one behaviour. AC-030 carries the injected-clock assertion and the honest `resets_at` derivation note. |
| Configuration-class errors are never retried | **AC-027** | AC-234, AC-378 | AC-027 is table-driven over all four classes. |
| Schema failure re-prompts exactly once | **AC-031** | AC-235 | AC-031 also carries the never-parse-JSON-from-prose sub-clause; AC-235 carries the honest note that F31's 52/52 was measured where this retry never fires. |
| Circuit breaker: K failures, per family, durable | **AC-033, AC-034, AC-035** | AC-237 | AC-237 bundles all three kernel criteria into one id. |
| `Degraded {from, to, why}` is recorded | **AC-037** | AC-236 (record half), AC-363 | AC-236's gate half — the gate does not close on a degraded result — is **AC-038's** behaviour under a second id; AC-038 owns it and AC-236's gate clause is its twin (PIA-070: the earlier "is not duplicated" was wrong). |
| Three budget counters, never shared | **AC-020** | AC-390 | AC-020 adds the `trybuild` newtype separation. |
| `--bare` never appears in argv | **AC-511** | AC-216, AC-422 (half) | AC-511 is strongest: it asserts the auth mode from the child's own `system/init`, which survives R6's expected default flip. A source lint alone does not. |
| `ANTHROPIC_API_KEY` absent from the child environment | **AC-488** | AC-217, AC-422 (half) | AC-488 asserts a typed construction error and a dispatch count of 0. |
| Isolation decided from the child's own `system/init` | **AC-202** | AC-195 | AC-386 was listed as AC-202's twin and is not one: AC-202 decides isolation from the MCP-server list under `--strict-mcp-config`; AC-386 decides it from the card's `tools` allowlist. Both class the failure `NotIsolated` (decision #5 of 2026-09-11; row C4 below), and AC-386's companion AC-399 (quarantine the run's metrics) is distinct. |
| Tool-set inequality against the card's allowlist is `NotIsolated` | **AC-386** | AC-204 | AC-204 said `NotAdopted` until 2026-09-11; one class for one condition (row C4). |
| The `is_error` / `api_error_status` frame is never success | **AC-218** | AC-428 | AC-218 is the harness-area owner (the four-way exit-code × flag matrix); AC-428 is the cli-doctor split half. Both read the seven recorded `e3.jsonl` frames (PIA-075). Owner named 2026-09-11 (PIA-029, PIA-051). |
| Backoff base 2 s / cap 5 min — kernel policy constants, not pack values | **AC-029** | AC-233 | Both state the envelope; AC-029 carries the home-of-the-numbers note (PIA-055): base and cap are `rk-kernel::retry` constants, `K`/`N`/the appetite are pack values, and design §3.6/§3.10 are amended toward this. |
| A dangling role/slot/skill reference is a `doctor` error | **AC-092** + AC-083 | AC-420 | The packs-catalog pair split by reference kind; AC-420 bundles them in the cli-doctor rule table (PIA-051). |
| A dead role, skill or slot is a `doctor` error | **AC-093** + AC-094 | AC-421 | AC-093 owns the unfilled slot, AC-094 the dead definition; AC-421 bundles both. AC-094's GIVEN reads "referenced by no enabled pack" and does not repeat AC-093's slot clause (PIA-051). |
| `rk-kernel` has no dependency edge on any adapter crate | **AC-484** | AC-005 | AC-484 asserts the transitive graph via `cargo metadata`; AC-005 asserts the same via the committed `deny.toml` (ADR-0100). One list, two instruments (PIA-051). |
| A missing capability fails as `StepError::Unsupported(capability)` | **AC-494** | AC-238 | AC-494 owns the typed failure and its never-a-pass clause; AC-238 adds the no-adapter-identity-branch lint. Both name a variant ADR-0112 added to ADR-0095's table (row C5, resolved). |
| Self-hosting: a recorded lane run on Redkiln-RS itself | **AC-473** (fix lane) | AC-521 (story lane) | Two lanes, one release check each; ADR-0094 decision 3 makes the exit bar both (added 2026-09-11). |
| The per-class flip rule for a re-executed finding | **AC-288** + AC-289 | AC-344 | `rk-kernel::policy` owns the predicate; AC-344 applies it to the verification gate's own `reproduced`-class findings and adds the sink routing (PIA-063). |
| exit 0 with a schema void resolves to `StepError::Schema` | **AC-206** | AC-439 | Identical. AC-282 (review), AC-324 (verify) and AC-142 (store) are the same mechanism applied to different consumers and are **not** duplicates. |
| Every harness event carries the CLI version; a pin mismatch fires | **AC-226, AC-227** | AC-385, AC-423 | The harness pair splits recording from the mismatch outcome; the other two bundle them. |
| Reviewer clone shape; the F17 recovery must fail | **AC-292** | AC-191 | AC-292 names four checkable clone-shape properties and demotes the F17 replay to a probe. |
| The reviewed worktree's tree hash is unchanged | **AC-291** | AC-192 | AC-291 rests on a structural spawn assertion rather than on the model choosing not to write. |
| Only a schema-conforming `Findings` payload crosses back | **AC-281 + AC-318** | AC-199, AC-208 | AC-281 is the ingest check, AC-318 the architecture lint; the twins bundle them. |
| An unregistered facet can feed no row | **AC-072** | AC-108, AC-351, AC-425 | Four areas lint the same registry. AC-072 owns `facets.toml` under ADR-0030/0061, and under ADR-0089 for the `deriver` field and the set shape. **AC-520 is not a twin:** AC-072 requires the registry *entry*, AC-520 requires the scored *record* behind a `source = "derived"` entry, and collapsing them into one lint rule is the failure ADR-0089 names. |
| A boolean facet used as a tag is a lint error | **AC-109** | AC-073 | AC-109 adds the positive companion fixture that stops a reject-everything lint from passing. |
| Rank ties and hard cycles fail the build | **AC-047, AC-059, AC-078** | AC-103 | The selection trio splits load-time lint from the runtime graph assertion; AC-103 bundles ties and cycles into one CI check. |
| Family totality, and the escalation cap | **AC-104 + AC-116** | AC-048 | AC-048 keeps the fact-driven-family exemption the packs pair does not state. |
| A timeout must be declared; no implicit default | **AC-097** | AC-419 | AC-097 catches an unresolvable interpolation; AC-419 catches a non-numeric value. Both fixtures are worth keeping. |
| `doctor` runs at zero model dispatches | **AC-414** | AC-098, AC-377 (half), AC-490 (half) | AC-414 is the only one bound to a rejecting-fixture corpus, so it cannot be satisfied by a `doctor` that does nothing. |
| A card omitting `StructuredOutput` fails at configuration time | **AC-095** | AC-417, AC-205 (dispatch-time), AC-504 (generation-time) | AC-095 and AC-417 are the same `doctor` rule in two areas; AC-205 and AC-504 are genuinely different moments. |
| A produced-but-never-read artifact is flagged | **AC-426** | AC-147, AC-472 (second half) | AC-426 makes the window a required key with no default and adds a within-window fixture that must **not** flag; AC-147 has no window at all. |
| Opening a gate is a write and an exit | **AC-240 + AC-278** | AC-024, AC-403 | The human-gate pair separates exit 0 from no-surviving-descendant, which F22's one survivor shows is a real distinction. |
| A review can add findings, never clear an AC | **AC-315** | AC-342 | Mirror images across two areas: AC-315 from the review side, AC-342 from the ledger side. |
| Per-AC mutation evidence; a surviving mutant is not `pinned` | **AC-334** | AC-317, AC-458 (migrated rows), AC-140 (rendering) | AC-334 owns the flip predicate and the `mutation-survived` status; AC-317 is the harness invocation that calls it and no longer carries a second predicate (ADR-0104 decision 2; PIA-051). |
| No model-authored decision field reaches policy | **AC-280 + AC-283** | AC-343 | AC-283's type-signature test is the load-bearing instrument in all three. |

**Five contradictions.** Three (C1–C3) the 2026-09-08 assembly could not resolve and left for a human decision; C1 is now decidable by measurement (PIA-068). C4 was found by the audit and resolved by decision #5 of 2026-09-11; C5 was found while absorbing ADR-0095 on 2026-09-11 and is recorded, not resolved.

| # | the contradiction | criteria | why it matters |
| --- | --- | --- | --- |
| C1 | **Do harness-internal retries charge the activity's attempt budget?** AC-026 and AC-232 say each observed inner attempt is counted against the budget and the kernel adds no retry of its own. AC-391 explicitly **cut** that clause in refutation as unlicensed — ADR-0037 counts *steps per pass*, and §4.3's own row says observe and record, do not duplicate — on the ground that it would make the budget a function of platform-side behaviour the kernel cannot bound. | AC-026, AC-232 vs AC-391 | Both readings are implementable and they give different budgets. **Re-opened as decidable by measurement (2026-09-11, PIA-068):** the `system/api_retry` payload *is* captured — 139 rows in 12 result files; per row 1 attempt on 89, 2 on 21, 3 on 12, 4 on 4, 5 on 1, 6 on 6, 10 on 6 (the `max_retries = 10` ceiling) [measured, recount at AC-026]. Under AC-026's reading, six recorded dispatches would have spent ten attempts of an activity budget on platform-side `overloaded`/529 weather before the kernel acted once. The decision is the operator's; the register records the distribution and does not take it. If C1 resolves against AC-391, ADR-0037 is superseded, never edited. |
| C2 | **Is a hand-edit inside a rendered section drift, or is it silently overwritten?** AC-124, AC-411 and AC-159 say the edit is surfaced as typed drift with both digests, and AC-159 has `doctor` report an unrecorded artifact mutation by digest comparison. AC-478 says the typed values are returned unchanged, the markdown is regenerated over the edit, and — verbatim — *no comparison of stored-vs-regenerated rendering is performed, so no drift state exists to report*. | AC-124, AC-159, AC-411 vs AC-478 | AC-478 records this as the intended ADR-0005 trade and states the consequence honestly. It is nonetheless the opposite behaviour, on the one surface a human touches most. |
| C3 | **How is a per-item token appetite raised?** AC-023 refuses an in-run raise *unless it arrives as a typed budget-change event with an author*. AC-389 refuses it outright: *raising the appetite is possible only by returning the item to triage*. | AC-023 vs AC-389 | AC-396 routes the appetite **value** to a human gate, which suits either mechanism — but a build cannot implement both refusals. |
| C4 | **What class is a tool set unequal to the card's allowlist?** AC-204 said `StepError::NotAdopted`; AC-202, AC-386 and design §7.4 said `StepError::NotIsolated`; only `NotIsolated` triggers AC-399's quarantine, so the pick silently decided whether adoption failures quarantined a run's metrics. | AC-204 vs AC-386 (and AC-202, AC-399) | **Resolved 2026-09-11 — decision #5 of `spikes/DECISIONS-2026-09-11.md` (PIA-016): `NotIsolated` everywhere.** AC-204 amended; AC-386 and AC-399 unchanged; ADR-0095's `StepError` table reserves `NotAdopted` for the wrong agent answering; SM-061's instrument is a set digest. |
| C5 | **Which `StepError` variants exist?** ADR-0095 decision 2 fixes the closed set at eight — `Transport`, `Schema`, `Timeout`, `Cancelled`, `NonZero`, `RateLimited`, `NotAdopted`, `NotIsolated` — and forbids any variant not in the table. Nine criteria and design §3.6 name three others as variants: `Platform { status, retry_after }` (AC-029, AC-218, AC-233, AC-237, AC-428), `Unsupported(capability)` (AC-027, AC-234, AC-238, AC-328, AC-494, AC-500) and `Overage` (AC-027, AC-221, AC-234, AC-378). | AC-027, AC-029, AC-218, AC-221, AC-233, AC-234, AC-237, AC-238, AC-328, AC-378, AC-428, AC-494, AC-500 vs ADR-0095 | **Recorded and resolved 2026-09-11 by ADR-0112** (successor-in-part to ADR-0095): the table gains `Platform { status, retry_after }` (retry with backoff honouring `retry_after`; `spoke()` true; `step_failed { class: platform }`), `Unsupported { capability }` (never retried; `step_refused { class: unsupported }`) and `Overage { reason }` (refused before spawn; `step_refused { class: overage }`) — eleven variants; the thirteen criteria keep their names. |

Two further tensions are already flagged inside their own areas and are recorded here only so they are not lost: AC-087 (rank selectors by specificity) against AC-088 (no specificity tie-break in the mixed-toolchain case), which will collide if implemented in one code path; and AC-057 (dead activities are a **warning** that does not fail the build) against AC-102 (the build **fails** above the registered threshold), which are reconcilable only if the warning is understood to apply below the threshold.

### 0.4 Id hazards — retired ids and reused ids

Ids are fixed and other documents will cite them. Two classes of hazard survive from the refutation pass, and a third was added deliberately on 2026-09-09.

**Retired or unallocated — never reuse:** AC-075 (selection: cut as SM-015 restated as a criterion), AC-119 (packs-catalog: never allocated), AC-207 (harness: cut for fabricated evidence, retired deliberately so no dead anchor points at 'envelope unwrapping'), AC-255 (human-gate: cut as subsumed by AC-244), AC-297 (review-gate: cut as SM-085's baseline restated), AC-513 (ports-dist: cut and retired).

**Reused — a citation predating this register may resolve to the wrong criterion:**

| id | was | is now | area |
| --- | --- | --- | --- |
| AC-185 | 'CI runs the rebuild test' (cut as vacuous) | the output-preservation half split from AC-187 | git-reconcile |
| AC-199 | 'change summary computed in the clone' (cut: contradicted AC-191, and PROVISIONAL on the unrun S-BIN) | the return-channel half split from AC-192 | git-reconcile |
| AC-323 | 'a capture test pins `basis` semantics' (cut: its THEN was a statement about the test suite; the GATE-05 ground it cited was withdrawn in §15 — no such identifier exists in the brief) | the point-of-use rendering half split from AC-322 | verify-gate — the original capture-test idea is a candidate for re-issue under a new id, not under this one (PIA-070) |
| AC-428 | 'CI deletes the store, rebuilds, diffs the next action' (cut as a duplicate of AC-427) | the `is_error` half split from AC-439 | cli-doctor |
| AC-511 | 'a compiled-in payload wins over a payload-shaped cwd' (cut: contradicts §4.9) | the `--bare` half split from AC-488 | ports-dist |
| AC-512 | 'exactly one warning, count asserted at 1' (cut: nothing left to warn about) | the receipt-rewriting half split from AC-514 | ports-dist |

The git-reconcile pass recommends retiring AC-185 and AC-199 and using AC-200/AC-201 instead; that range is allocated to the harness area, so the recommendation cannot be taken without extending the ranges. **The register keeps the ids as the areas left them and states the hazard rather than renumbering**, because renumbering is the one change that breaks every citation already written.

**Allocated outside its area's range — deliberate, and the reason is recorded:**

| id | area it belongs to | that area's range | why it sits outside |
| --- | --- | --- | --- |
| **AC-520** | selection (§3.7) | AC-040..AC-079 — **exhausted** | AC-075 is retired and never reusable; AC-077..AC-079 are already splits. There is no free id in the range, and AC-520 is the next free id in the register as a whole. Added 2026-09-09 as the derived-facet sibling gate F39 forced. |
| **AC-521** | migration / self-hosting (§13, beside AC-473) | AC-440..AC-479 — exhausted | The story-lane twin of AC-473 (ADR-0094 decision 3). Added 2026-09-11. |
| **AC-522** | migration (§13, beside AC-450) | AC-440..AC-479 — exhausted | `redkiln-migrate`'s run contract: refuse a dirty target, write into a dedicated branch/worktree (PIA-002). Added 2026-09-11. |
| **AC-523** | harness (§7.6, beside AC-225) | AC-200..AC-239 — exhausted (AC-207 retired) | `RateLimited { resets_at }` classification and the argv-length guard (ADR-0095; decision #18). Added 2026-09-11. |
| **AC-524** | human gate (§8.6, beside AC-271) | AC-240..AC-279 — exhausted (AC-255 cut) | Fresh `(token, tree_hash, deadline)` on loop-back re-reach (ADR-0105 clause 10; PROVISIONAL on P8-B). Added 2026-09-11. |
| **AC-525** | selection (§3.6, beside AC-067) | AC-040..AC-079 — exhausted | ADR-0025's positive clause: exactly one `drift_advisory` per in-set upstream change, carried by the next gate (PIA-029). Added 2026-09-11. |
| **AC-526** | harness (§7.4, beside AC-210) | AC-200..AC-239 — exhausted | Both cache-TTL variables set together or the configuration is refused (ADR-0078; PIA-029). Added 2026-09-11. |
| **AC-527** | packs-catalog (§4.5, beside AC-112) | AC-080..AC-118 — AC-119 was never allocated and stays unallocated | The forbidden empty-set quality-block terminal predicate is a lint error (ADR-0080; PIA-029). Added 2026-09-11. |
| **AC-528** | ports-dist (§14.3, beside AC-500) | AC-480..AC-519 — exhausted (AC-513 retired) | No `async fn` in any `rk-ports` trait; `cargo tree -p rk-kernel` names no async runtime (ADR-0076; PIA-029). Added 2026-09-11. |
| **AC-529** | git-reconcile (§6.4, beside AC-183) | AC-160..AC-199 — exhausted | A tier-1 write from a worktree the store does not own for the run is refused (ADR-0105 clause 8; ADR-0018; PIA-029). Added 2026-09-11. |
| **AC-530** | review gate (§9.6, beside AC-310) | AC-280..AC-319 — exhausted (AC-297 cut) | `advisory_retired { reason: not_re_raised }` and `live_sink == created - retired - dismissed` (ADR-0082; PIA-029). Added 2026-09-11. |
| **AC-531** | cli-doctor (§12.9, beside AC-437) | AC-400..AC-439 — exhausted | `init`'s namespacing clause, cut from AC-437 on 2026-09-08 and re-added under the decided `rk-` prefix (decision #8; PIA-106). Added 2026-09-11 — **allocated by this amendment beyond the brief's AC-521..AC-530 block**, and reported as such. |

**The 2026-09-11 ids follow the brief's order for the six PIA-029 criteria** — `drift_advisory` (AC-525), TTL variables (AC-526), the rk-catalog empty-set lint (AC-527), no-async-in-ports (AC-528), the §6.4 ownership refusal (AC-529), `advisory_retired` accounting (AC-530) — after the four the brief allocated by name (AC-521 story lane, AC-522 migrate run contract, AC-523 `RateLimited`, AC-524 fresh gate record). Every one is counted in the area its §0.4 row names, and the area's `**Criteria:**` line says so. ADR-0092 clause 3 governs the six re-issued ids above: each conforms (no external citation at the cut, both sites note the other, this table lists both meanings) and they are not debt.

Two consequences a reader must carry. **AC-520 is a selection criterion**, counted in §0.1's selection row and in §3's area key, however its number reads — the ports-dist range it sits beside (AC-480..AC-519, with AC-513 retired) does not own it; the same holds for each of AC-521..AC-531 and the area its row names. And **an id's numeric neighbourhood is no longer a reliable guide to its area**, which was already only a convention and is now explicitly not a rule. Renumbering any area to make room was rejected on the same ground as above.

---

## 1. What is blocked

This section comes first because the rest of the document reads as decided, and parts of it are not. Fifteen things were blocked, unmeasured or undecided on 2026-09-08; on 2026-09-11 three are discharged in place (rows 7, 8 and 15) and one moved from "open" to "not blind" (row 11). For each: what is blocked, what unblocks it, and — where one exists — the executable precondition that decides it.

### 1.1 The selection engine — P1's kill fired 4 of 4

**P1 is the block that shapes this whole register.** The LLM-emitted context vector produced the identical modal required set on **5/20, 3/20, 2/20 and 2/20** items — two packs (`bug-standard`, `story`) × two runs (v1, and the §1.4 anchor-rewrite re-run v2), each across all 10 draws — against an 80% line; `external_dependency` collapsed to a constant (200/200 false); and the pre-registered repair did not move it [measured, F36; 400 dispatches, 0 void, $21.37]. Wilson 95% intervals, computed rather than measured: 11.2–46.9%, 5.2–36.0%, 2.8–30.1%, 2.8–30.1% — every upper bound sits far below the line.

**Arm D is now scored, and it does not unblock the engine.** P1 arm D was scored on 2026-09-09 against **140 truth cells, all of them scored** by the fixed `score-arm-d-panel.py` re-run (the 2026-09-09 run scored 136 of the 140 — the panel-file key `resolve-open-questions` → `resolve-open-question` left four cells without an engine record, and the fixed scorer fails closed on such a cell — ERRATA E-0089-2) at corpus pin `47faec9f` under METHOD-P amendment 9, at **$0** model cost, and is reported as **panel-adjudicated**, never as human-panel [measured, F39 as corrected]. The per-facet figures below are the 2026-09-09 scoring except where E-0089-2 restates them (`auth` **0/12** [0.00, 0.24] on the fixed re-run; 0.00 [0.00, 0.26] over 11 positives on the first). Per-facet precision on the union source over 20 items, with Wilson 95% intervals: `ui` **1.00** [0.65, 1.00], recall 0.78 (tp 7, fp 0, fn 2, tn 6, excl 4) — the point clears 0.95 and **the interval does not**; `toolchain` 0.73 [0.48, 0.89], recall 0.79 (tp 11, fp 4, fn 3, excl 0); `change` 0.72 [0.54, 0.85], recall 0.84 (tp 21, fp 8, fn 4, excl 0); `binary_change` 0.60 [0.23, 0.88], recall 1.00 (tp 3, fp 2, fn 0, tn 15, excl 0); `flag_exists` 0.00 [0.00, 0.66], recall n/a (tp 0, fp 2, fn 0, tn 13, excl 4); `auth` **0.00** [0.00, 0.26], recall n/a (tp 0, **fp 11**, fn 0, tn 4, excl 4); and `data_migration` is **not measurable** — the script emits 0 positives on 20 items across all three sources, so precision is undefined, a perfect method scores identically, and the criterion has **no denominator**. **No facet clears METHOD-P §1.4's 0.95 bar.** The sub-0.95 clause fires on five facets, and the ≥0.95 clause is **not reachable at n=20 for any facet however good** — `ui` is 7/7 and its lower bound is 0.65; **73** all-correct positives are needed for a Wilson lower bound above 0.95 at the registered two-sided z = 1.96 (92 with continuity correction; 72 Clopper–Pearson — "roughly 60" was wrong, E-0089-2/PIA-073). That is a result *about* the pre-registration, reported in F39's limits and not an edit to it, and it is the same class of defect as P1's alpha rule (F36) and P7's criterion 1 (F37): a pre-registered bar the registered sample size cannot decide.

**Why `auth` scores zero**, because it decides what the repair is. The rule is an unanchored, case-insensitive regex over paths **and** diff lines, and it marks `auth = true` on 14 of 20 items. `token` fired 44 times, 36 of them inside `brand-token-substrate` / `brand-tokens.test.ts` / `redkiln.tokens.json` / `tokens_out`; `auth` fired 43 times, 40 of them inside *authored* / `report-md-unauthored` / *author* / *authors* / *authoring* / `only-the-author-can-advance`; `oauth` fired 8 times, all 8 inside *coauthor* (the string `oauth` sits inside `c-oauth-or`); `session` fired 8 times, 6 of them a *work* session; `role` fired twice, both an ARIA `role` attribute [measured, F39 §2]. **The failure is correlated, not random:** the colliding vocabulary is the vocabulary of the instrument under test. Eleven positives, eleven false, upper bound 0.26 — the rule carries **no signal** on this corpus, which is a finding about the method and not a tuning parameter.

**What F39 discharges, and what still holds the block.** AC-071 has five clauses — identity ≥ 0.80, n ≥ 20 items, ≥ 10 draws per item, arm-D panel scored, corpus pinned — and F39 discharges **the fourth clause only**. The block stands on the **first**: F36 measured modal required-set identity at **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line. **ADR-0079 stays `proposed`**: F39 measures a *derived* context vector and ADR-0079 asks about an *LLM-emitted* one, so F39 narrows 0079's options without deciding among them [documented, ADR-0089]. Nothing in this register may now say a facet *leaves the model* — none does — and nothing here makes this design *final*: S-BIN, S-H1c and P8's unrun arms gate that word.

**Two further F39 results changed criteria in §3.7.** First, **four of the twenty items changed no code whatsoever** — every changed file is `.bklg/` bookkeeping or telemetry — the panel returned `unknown` on all sixteen of their semantic-facet cells, and the script emitted a confident boolean on every one [measured, F39 §4]. That is the `excl = 4` column, and the registry line `values = [true, false]` is what turned four empty diffs into sixteen confident booleans; AC-079's route is extended to the booleans in consequence. Second, AC-071 gates `source = "llm"` and **nothing gated `source = "derived"`**, so the semantic facets sat outside the block and ran **ungated while measurably wrong** — **AC-520** is the sibling gate that closes it, new in this revision and allocated outside the selection range (§0.4). ADR-0089, accepted 2026-09-09, records the decision behind both: provenance classes with a `deriver` companion field, a set-valued `surfaces` facet replacing the repo-wide `auth` boolean, and tri-valued booleans routed to AC-079's existing `gate_opened { reason: unknown_facet }`. **Source asymmetry is not decided here.** `commits` beats `docs` on every facet — `toolchain` 1.00 [0.70, 1.00] against 0.50 [0.22, 0.78]; `change` 0.80 [0.61, 0.91] against 0.50 [0.24, 0.76]; `binary_change` 0.60 at recall 1.00 against no positives at all from `docs`, 3 missed; `ui` 1.00 at recall 0.67 against 1.00 at recall 0.22, union 1.00 at recall 0.78 — and `union` is *worse than `commits` alone* on `toolchain` (0.73) and `change` (0.72), because unioning a precise source with an imprecise one imports the imprecision [measured, F39 §3]. Which source the engine trusts is registered as an arm of the named spike **S-FACET** (ADR-0012), whose named consumer is `facets.toml`'s `source`/`deriver` fields.

**How much weight F39 carries.** Of the 140 cells (this breakdown is F39's 2026-09-09 scoring, which reached 136 of them; the fixed re-run scores all 140 and the breakdown is not re-derived here), **60** were decided by a deterministic oracle over git metadata (independently implemented), **39** by a unanimous panel, **9** by a unanimous panel where the adversarial flag was mechanically moot, and **32 by human adjudication** — Ryan, with the panel's and the script's positions revealed only after he answered; the adjudication surface was a claude.ai Artifact page not in the repository, and the `blind_attestation:` line at `spikes/results/p1-arm-d-labels-ryan.md:27` is unfilled, so "blind" is his to attest (decision #13 of 2026-09-11; PIA-096). His verdicts were 8 true, 24 false, 0 unknown, and **15 of 32 differ from the script**. The panel labelled 80 semantic cells: 39 accepted unanimous (23 decided, 16 honest `unknown`) and 41 flagged [measured, F39 §5]. F39's own stated limits travel with every number above and are not dropped here: no facet has enough positives to clear 0.95 with confidence even in principle; **39 of the 140 cells rest on unanimous 3-seat model agreement with no human**, and a 9-judge panel was measured to supply ~2.18 effective independent votes with accuracy 8–22 points short of independent-vote predictions [preprint, arXiv:2605.29800], so small-panel unanimity fails in the correlated direction; the oracle is independent on `toolchain` and on `binary_change`'s Unreal shortcut and **not** on the rest, because for `change` git's marker is the ground truth for the binary component and oracle and script therefore agree there by construction; one repo, one operator, one pin — every number is over `47faec9f`; a scorer defect was found and fixed **before** this was reported (the oracle emits `binary_change` as a JSON bool and the first scorer compared it to the string `"true"`, scoring the facet 0.00 when it is 0.60); and `unknown` is **excluded** from scoring, never counted against the script — 16 semantic cells are excluded on that basis, and reporting them as agreement would have raised every semantic precision figure.

**What survives.** The engine's *semantics* are writable and are written: purity (AC-040, AC-041, AC-044), replay (AC-042), rank and tie-is-an-error (AC-046, AC-047), unsatisfiable-is-a-typed-gate (AC-054, AC-055, AC-056), deviation-never-edits-`R` (AC-061..AC-064), and the lint set. **What does not.** Any criterion presuming a stable LLM-emitted vector. Ten areas state explicitly that nothing in them depends on the vector; the two that touch it (packs-catalog AC-113, cli-doctor AC-434) are scoped to a *fixed* vector and to *display* respectively.

**AC-071 is the machine-checkable statement of the block**, and it is not itself provisional. Its executable precondition:

```sh
redkiln lint --facet-stability .redkiln/facets.toml \
             --stability-record spikes/results/p1/stability.json
# exit 0 only when, for EVERY facet declared source = "llm" in facets.toml:
#   rec.modal_required_set_identity >= 0.80          # THE CLAUSE THAT STILL HOLDS THE BLOCK.
#                                                    # the pre-registered P1 line; F36 measured 0.25/0.15/0.10/0.10
#   AND rec.n_items >= 20 AND rec.draws_per_item >= 10
#   AND rec.panel_verdict == "scored"                # arm D precision AND recall present, non-null.
#                                                    # SATISFIABLE, and SATISFIED for arm D: F39 scored it over
#                                                    # 140 truth cells at 47faec9f, all scored on the fixed re-run (E-0089-2), panel-adjudicated, $0.
#   AND rec.corpus_sha == <catalog corpus pin>       # F37's emit.corpus_sha was misnamed; this one is the corpus
# Until that command exits 0, no facet may be source = "llm"; every ordinal axis is source = "human".
```

**Which clause holds the block, now that one of the five is discharged.** The fourth clause was written when no arm-D record existed; F39 supplies one, so `panel_verdict == "scored"` is no longer an unsatisfiable stub and no longer the reason the command exits non-zero. The block rests on the **first** clause: modal required-set identity measured **0.25 / 0.15 / 0.10 / 0.10** against the pre-registered 0.80 line [measured, F36]. Discharging one clause of a five-clause gate is not a partial unblocking, and the arm-D numbers are not a warrant to relax any of the other four.

**The residual risk in one sentence:** if that lint is implemented loosely — reading `modal_required_set_identity` and skipping `panel_verdict` or `corpus_sha` — the entire provisional apparatus unblocks silently, and the five-clause fixture set named in AC-071 is the only defence.

### 1.2 Everything else that is blocked

| # | what is blocked | unblocked by | executable precondition / what would settle it | criteria and measures affected |
| --- | --- | --- | --- | --- |
| 1 | LLM-emitted facet values in selection (see §1.1) | **P1** re-run at n>=20 items x >=10 draws. The arm-D panel clause is **discharged** — F39 scored arm D over 140 truth cells at `47faec9f` (all scored on the fixed re-run — E-0089-2), panel-adjudicated, $0 — and the block now rests on the identity clause alone, measured **0.25 / 0.15 / 0.10 / 0.10** against 0.80 [measured, F36, F39] | `redkiln lint --facet-stability` exits 0 (above); one of its five clauses is now satisfied and four are not | AC-071 (the gate), AC-069, AC-070, AC-072, AC-076, AC-079, AC-113, AC-434, **AC-520** (the derived-facet sibling gate); SM-015, SM-016, **SM-157** (the derived-facet precision record — a different quantity from SM-015's stability, and not fused with it) |
| 2 | The short-circuit policy — which classes get `stale_policy = gate` | **P2's perturbation half is scored** — 2026-09-10 at pin `47faec9f`, **41 `(case, downstream)` rows** over 12 perturbation cases, $0 model cost, labels blind and **timestamp-verified** [measured, F40] — and its **advisory half (A01–A06) is unrun**, so §2.4's criterion 3 is untouched. "P2 is entirely unrun" is now false and "P2 is complete" would also be false; every sentence about it must say which half. What blocks the policy is now the corpus, not the absence of a run: **0 of 41 rows are "should re-run"**, so recall has no denominator | **The bar is now known** — it is METHOD-P §2.4, quoted rather than paraphrased: (1) coarse "recall < 0.9 or precision < 0.7 → fine scope becomes mandatory for `L`-class activities and `stale_policy = gate` is the default for them"; (2) "fine scope cannot reach recall ≥ 0.9 either → `stale_policy = gate` for all `L`/`XL` activities; **the memo key is trusted only at `S`/`M`**"; (3) "advisory useful fraction < 0.3 → PC3's advisory is demoted to a log line (no gate text)". **Two of the three cannot be evaluated on the registered corpus** — criterion 1's recall half and criterion 2 in full are **NOT ANSWERABLE** for want of a positive class, never 0.00 and never 1.00 — and the third is **unrun**. Criterion 1's *precision* half fires (**0.00** [0.00, 0.26] over 11 emitted positives), but that figure is a property of a label set with no positives, and **the remedy it prescribes is contraindicated by the same measurement**: fine specificity **0.55** [0.38, 0.70] against coarse **0.73** [0.58, 0.84], and on the 33 rows both scopes scored **10 flip `Present` → `Stale` coarse → fine while none flips back** [measured, F40]. So `redkiln lint --stale-policy --p2-record …` has one clause discharged and two this corpus cannot settle: `labels_timestamped == true` is **satisfied** (`labelled_at` 2026-09-10T15:58:08Z precedes the engine's earliest record at 16:01:12Z); `n_items >= 20` is **not met** (12 cases — eight of the twenty selected items produced no perturbation case at all); and the recall line has no denominator to clear or fail. **PROVISIONAL(P2) no longer means *unspecified*: it means the criteria that would settle it have no denominator on the registered corpus, and the one that fires prescribes a contraindicated remedy.** What would settle it is a **P2 re-run on a corpus carrying substantive upstream changes** — a named spike with a named consumer (ADR-0012), the consumer being AC-068 and SM-021 — not another score of this one. **Nothing here settles `stale_policy`; the design consequence is open and needs a decision, not a default.** | AC-068; SM-021 |
| 3 | `ChangeSummary`'s field set, and every criterion that renders or migrates one | **S-BIN**, registered and unrun; it gates **the `ChangeSummary` type design and the Toolchain port's `change_summary` body, not Phase-1 entry** (ADR-0093 decision 4 — "gates Phase 1" was the wider reading the design's own Enter/May-not columns contradicted, PIA-001) | S-BIN runs against the ADR-0008 Unreal fixture and the type is settled; its METHOD-S §7 rule is evaluated per pass, branch 1 needs both passes (PIA-084). Until then no fixture naming its fields can be written and `ChangeSummary` is an opaque newtype in `rk-ports` (ADR-0099). | AC-154 (unverifiable until then), AC-356 (its AND clause struck, PIA-036), AC-475, AC-497; AC-049 stands on F35 §1.3 [measured], not "unfounded" (PIA-075) |
| 4 | The card-delivery append route | **S-H1c**, registered and unrun; ADR-0016 is conditional on it | S-H1c reports whether `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt` keeps the **shared-prefix** head (per machine and per directory — not account-wide, PIA-103) **and** holds >=0.95 across commits; its pre-registration names the per-directory prefix as the null hypothesis. | AC-211 fixes only the default (per card, decision #19); `cache_profile` (AC-383) gains a third value only if S-H1c runs |
| 5 | The second harness | **S-CODEX**. It is in proposal §8's spike table (46 Codex dispatches) but **does not appear in the evidence brief at all — not even in its unrun list**; the Codex adapter is Phase 2 behind the CI decision ADR-0010 defers (ADR-0094 decision 2) | A recorded E3-shaped validation result for the candidate adapter exists in the pinned corpus repo — with a METHOD entry before dispatch whose primary metric is a non-saturated measure (S-SUBTLE-tier detection; CLEAN FP with its Wilson upper bound), since E3 §9 is saturated (PIA-084). | AC-239, AC-496; SM-154. AC-502's dialect-parity domain is undefined until then |
| 6 | Multi-step resume, and every duration claim about it | **P8**, PARTIAL: 3 of a pre-registered 27+3 runs, recorded state holding exactly **one** step; the 65-min arm, the 24-h arm and all three controls are unrun | P8's remaining arms run. Note its §5.4(3) second-arrival criterion is **circular by construction** and no completion fixes that. | AC-013, AC-187, AC-266, AC-267, AC-271, AC-401; SM-002, SM-052, SM-073, SM-130 |
| 7 | Detecting an overage | **Narrowed 2026-09-11 (PIA-065).** The envelope *is* recorded — `isUsingOverage: false`, `overageDisabledReason`, `overageStatus` on F1's own `rate_limit_event` payload (`spikes/FINDINGS.md` :42–48), and `is_using_overage: false` on all 20 E0-headroom and all 60 S-VAR rows the runner captured; the condensed brief omitted the keys. What was never recorded is a **`true`** value | One recorded real stream carrying `isUsingOverage: true`. Until then the adapter test runs against the recorded false-valued payload with the boolean flipped — a recorded envelope, not a fixture of our own construction; S-RETRY is the named consumer of the first true reading. | AC-378 (PROVISIONAL narrowed to "no true-valued instance"), AC-221; SM-112 — whose baseline is absent for the same reason |
| 8 | Inner/outer retry accounting | **Discharged 2026-09-11 (PIA-068).** The payload **was** captured: `api_retries` carries `{attempt, error, error_status, max_retries, retry_delay_ms}` on 139 rows in 12 result files (`e3.jsonl` 26 of them) [measured, recount 2026-09-11]; "never captured" was false. What remains open is the *accounting* question — whether inner attempts charge the activity budget (C1 in §0.3), now **decidable by measurement** over those rows rather than blocked on a capture | C1's decision; the per-row attempt distribution is recorded at AC-026 | AC-026, AC-232, AC-391; SM-003 (the "payload never captured" premise is withdrawn in its companion row) |
| 9 | The Advisory sink | Brief §C.8 records that **no advisory sink exists** — 7 adjudicated-true `narrowed-pin` findings have no destination | AC-310 is the requirement that builds it. AC-344 is a requirement **on** that sink, not an assumption one is there. | AC-310, AC-311, AC-344; SM-089, SM-022, SM-104's kill |
| 10 | The distribution mechanism | Proposal **§4.9 decides no installer** — a plain git clone plus a `cargo-dist` binary shim. Receipts, rename tables, ownership states and integrity scans are shipmates' architecture, not a Redkiln-RS decision | §4.9 is reopened and a mechanism with an install-and-repair step is decided, or the six criteria are cut and 'decide the distribution mechanism' becomes a gating open question. | AC-512, AC-514, AC-515, AC-516, AC-517, AC-519 (partly); SM-149 |
| 11 | The false-block rate's denominator | The E3 severity adjudication was **adjudicated 2026-09-04 and closed 2026-09-09, NOT BLIND** — facts settled 6/6; severity from a sheet filled by the model at Ryan's direction, one operator, κ forfeited (PIA-081). The rates are settled on it: arm (a) FP 1/23, MCC +0.957 (E-0074-3); arm (b) FP 0/23, MCC +1.000 — upper bounds against CLEAN | **E3-SEV-BLIND** — a blind annotation of the preserved `e3-severity-annotation-BLANK.md` by an independent rater (design §10.5, consumer SM-086) — is what buys κ back. AC-298 sets the **system default** off the same NOT-BLIND sheet. | AC-308 (PROVISIONAL because the sheet is not an independent rater, not because the adjudication is open), AC-309, AC-357, AC-358; SM-086 |
| 12 | The whole-suite p95 half of the timeout rule | No whole-suite p95 wall clock was ever recorded — the field did not exist | First production readings. Until then only AC-330's declaration half can fire, which the criterion states inside itself. | AC-330; SM-102 |
| 13 | Corpus residency and the pinned-corpus instruments | `redkiln-corpus` **does not exist**; `spikes/corpus/`, `spikes/results/*.jsonl` and `spikes/runner/` are all tracked in this repo at HEAD `ab58733`; the 1,312-file corpus and the 118-ref history live only in `D:/repos/redkiln`; no golden event log exists for AC-006/AC-188 (PIA-002) | The ADR-0013 split happens at runbook Step 2 — a `git bundle create --all` of the frozen clone (or a fetchable remote plus SHA in `CORPUS_PIN`) pinned by that repository's SHA — and golden-log generation lands at Steps 5/6. **AC-465 is violated today by construction** and no measure tracks the distance to compliance; every corpus-bound test is a local-only suite until then. | AC-465; AC-006, AC-184, AC-188, AC-427, AC-444, AC-464, AC-519's fixture pin; SM-124 |
| 14 | Dead-artifact detection | `doctor`'s window **N is undeclared**, so AC-472's AND clause has no truth value and SM-139's target cannot be missed | N is declared in the `doctor` rule table and AC-472's threshold fixtures (N-1 must not fire, N+1 must) are written. | AC-472, AC-426; SM-139 |
| 15 | The worktree cost cell | **Discharged 2026-09-11 (PIA-075).** The S-WT run record (`spikes/results/s-wt.jsonl`) names `--system-prompt`, so §C.1 was right: static × worktree is 1.000 steady (w1 0.807 cold, n = 9) and F16's worktree-each 0.685 (n = 9) is the **default** × worktree cell — two cells, not a contest. CONTESTED is struck here and at AC-387; S-WT-REC is DONE | Nothing; AC-387 prices the two measured cells from their records and returns `Unmeasured(cell)` for every other. What remains thin is n = 9 on one machine at one CLI pin. | AC-387; SM-053, SM-113, SM-114 |

**One thing that is unmeasurable rather than unmeasured.** Whether cache reads draw down the rate-limit window cannot be decided at the instrument's 0.01 resolution — E0's cache-cost gate was **STRUCK, not answered** (F14, F6). AC-381 therefore encodes the negative (no cache metric may feed a headroom figure) rather than a positive rule, and it is the criterion to revisit first if the platform ever exposes finer resolution.

---

## 2. Kernel — workflow/activity split, replay, memoisation, budgets, retry

**Area key:** `kernel` · **Criteria:** AC-001..AC-039 — **39 in force** · **Measures:** SM-001..SM-010 (SM-011, SM-012 unallocated)
**State:** 37 binding · 1 provisional (AC-013) · 1 human-gated (AC-039) — AC-026's PROVISIONAL lifted 2026-09-11 (PIA-068: the `api_retry` payload is captured on 139 rows)
**Sources:** proposal §4.2, §4.2.1, §4.3, §4.7; activity-graph schema §5, §9a, §9b; ADR-0074 (supersedes ADR-0004), 0006, 0009, 0010, 0012, 0013, 0014, 0015, 0016, 0017, 0020, 0021, 0022, 0035, 0036, 0037, 0043, 0053, 0055, 0064.

*Scope note.* Replay has never been measured on this system: F19's 174/174 is losslessness of the state *carrier*, and the brief records that the FSM was never replayed. AC-006..AC-009 are the instruments that would establish it, not a record of it. Nothing here asserts that an LLM-emitted context vector is stable — AC-009 journals whatever vector was emitted and claims nothing more.

### 2.1 Determinism of workflow code — what must become impossible inside it

**AC-001** — `rk-kernel` (workflow) · evidence: proposal §4.2 [documented], principle 1 §3.1 · decisions: ADR-0022, ADR-0083, ADR-0102
GIVEN any function on the workflow side of the split (stage selection, gate evaluation, retry decision, DAG order)
WHEN it reads the wall clock, a monotonic instant, or a timezone-dependent value directly
THEN the crate fails to build.
*Verified by:* a clippy `disallowed-methods` list in a committed `clippy.toml` (workspace root) applied to
`rk-kernel::workflow` under a module-level `#[deny(clippy::disallowed_methods)]` (ADR-0083's commented override; ADR-0102)
naming `std::time::SystemTime::now`, `Instant::now`, `chrono::Utc::now`; CI runs `cargo clippy --workspace --all-targets`
**gated on its exit code** (nursery warnings permitted, never `-D warnings`, which would promote `clippy::nursery` to the
group deny ADR-0083 forbids); plus a must-fail **fixture crate** that reintroduces one call and is checked under the same
`cargo clippy` invocation — a `trybuild` fixture cannot fail on a lint, only on a compile error, so it is not the
instrument here. Time reaches the workflow only as a journaled activity result. *(Amended 2026-09-11, PIA-030/PIA-031:
`-D warnings` and the `trybuild` half replaced; the clippy invocation and the `clippy.toml` home are now named.)*

**AC-002** — `rk-kernel` (workflow) · evidence: proposal §4.2 [documented] · decisions: ADR-0022, ADR-0083, ADR-0097
GIVEN any function on the workflow side of the split
WHEN it draws randomness or mints an id from a non-deterministic source (`rand`, `uuid::new_v4`, address hashing)
THEN the crate fails to build.
*Verified by:* the same `disallowed-methods` list plus a must-fail fixture crate under `cargo clippy` (AC-001's
instrument; the `trybuild` half is withdrawn, PIA-031). Ids the workflow needs are derived from the idempotency key
`(item, activity_id@version, attempt)` (ADR-0097) or from a content hash.

**AC-003** — `rk-kernel` (workflow) · evidence: proposal §4.2 [documented] · decisions: ADR-0022, ADR-0083
GIVEN any function on the workflow side of the split
WHEN it performs I/O — filesystem read/write, network, process spawn, or a blocking sleep
THEN the crate fails to build.
*Verified by:* `disallowed-methods`/`disallowed-types` in `clippy.toml` over `rk-kernel::workflow` (`std::fs`,
`std::process`, `std::net`, `tokio::time::sleep`, `thread::sleep`) plus a must-fail fixture crate under `cargo clippy`
(AC-001's instrument; PIA-031). All of it belongs to activities, which carry the retry policy, the timeout with a number,
and the idempotency key.

**AC-004** — `rk-kernel` (workflow) · evidence: proposal §4.2 [documented]; [inferred] from the replay requirement
· decisions: ADR-0022, ADR-0083
GIVEN a workflow decision that iterates a collection (candidate activities, pack rows, findings)
WHEN the collection is an unordered map or set
THEN the crate fails to build; ordered containers, or an explicit sort with a total key, are the only admissible forms.
*Verified by:* a clippy `disallowed-types` rule in `clippy.toml` banning `HashMap`/`HashSet` in workflow decision
signatures and locals, plus a must-fail fixture crate under `cargo clippy` (AC-001's instrument; PIA-031).

**AC-005** — `rk-kernel` · evidence: proposal §4.1 port rules, §6 crate layout [documented] · decisions: ADR-0022, ADR-0014, ADR-0100, ADR-0086 (the four banned durable-execution crates)
GIVEN the crate dependency graph
WHEN `rk-kernel`'s workflow module gains a direct dependency on any adapter crate (`rk-adapter-*`)
THEN the build fails.
*Verified by:* a `cargo-deny` `[bans].deny` entry for every `rk-adapter-*` crate in `rk-kernel`'s graph in the committed
`deny.toml` (`cargo install cargo-deny --locked` at runbook Step 5.2; `cargo deny check bans` is a CI gate beside
fmt/clippy/test/audit — ADR-0100 decision 3, so the instrument named here now exists), plus a CI `cargo tree -i`
assertion that returns empty. The workflow reaches the world only through `rk-ports` traits, so a fake per port can
drive every replay test at zero model cost.

**AC-006** — `rk-kernel::replay` · evidence: F19 [measured: 174/174 lossless suspension, **but the FSM was never
replayed**, so "same next action" is unproven — this criterion is the instrument that would prove it]
· decisions: ADR-0022, ADR-0012, ADR-0013
GIVEN a recorded event log for a completed or suspended run
WHEN the workflow is replayed against that log with every port bound to its fake
THEN the sequence of decisions and the next action are byte-identical to the recorded ones.
*Verified by:* a golden-log replay test in `rk-kernel` over **golden event logs recorded by `rk-kernel` itself on the
pinned corpus repository and committed to `redkiln-corpus` as fixtures [WILL EXIST after runbook Step 5 — no such log
exists today; golden-log generation is a Step 5/Step 6 checkpoint item]** (ADR-0013); run in CI on every commit, at zero
model cost, and asserting **zero `rk-git` writes** on replay (ADR-0105 decision 1: the `GitOps` journal is served back).
*(Amended 2026-09-11, PIA-002: the earlier wording read as if recorded runs existed; they do not.)*

**AC-007** — `rk-kernel::replay` · evidence: F19 [measured], principle 4 "absence of evidence is a block" [documented]
· decisions: ADR-0022
GIVEN a replay whose recomputed decision differs from the recorded one
WHEN the divergence is detected
THEN the run halts with a typed `ReplayDivergence { activity_id, recorded, recomputed }`, no new action is taken, and
no continuation on the recomputed value is reachable.
*Verified by:* a unit test that mutates one recorded decision and asserts the halt; an exhaustive `match` on the
divergence outcome with no wildcard arm, and a `trybuild` test proving the wildcard cannot be reintroduced.

**AC-008** — `rk-kernel::replay` · evidence: proposal §4.2 [documented] · decisions: ADR-0022
GIVEN a replay of a run that dispatched N activities
WHEN the log is replayed
THEN the replay reaches the recorded terminal state with all N results served from the journal, **zero** harness
dispatches, and zero `rk-git` writes (the `GitOps` journal is served back — ADR-0105 decision 1).
*Verified by:* a replay test with a scripted-responder Harness fake that fails the test on any `dispatch` call, and
that separately asserts the terminal state matches the record — so a regression that re-dispatches on replay is a
red test, not a bill, and a kernel that does nothing at all fails the terminal-state half.

**AC-009** — `rk-kernel` (workflow) · evidence: **F36** [measured: modal `R` identical on 5/20, 3/20, 2/20, 2/20 against
an 80% line; alpha drift −0.168 on byte-identical anchors] · decisions: ADR-0022, ADR-0012
GIVEN a stage whose selection consumes a model-emitted classification or context vector
WHEN the vector is produced
THEN it is recorded verbatim in the activity result that produced it, and every later read — replay, resume or a
second stage — resolves from that journal entry rather than a new elicitation.
*Verified by:* an event-schema test that the vector is byte-identical to the dispatch payload it came from; plus the
AC-008 fake asserting zero dispatches of the classifier activity on re-entry.
**Scope:** the selection engine this feeds is BLOCKED by P1 (F36) and its *semantics* criteria live in the selection
area. This is the journaling rule only; it does not claim the vector is stable and must not be read as licensing
selection on an unstable vector. (Not PROVISIONAL: its warrant is a machine-checked recording rule, not a spike.)

### 2.2 The activity idempotency key

**AC-010** — `rk-kernel::activity` · evidence: proposal §4.2 (shipmates L5.4/L5.6) [documented] · decisions: ADR-0022
GIVEN an activity about to be executed
WHEN a dispatch is constructed
THEN it cannot be constructed without an `(item, activity_id@version, attempt)` idempotency key, and a malformed key
(empty `item`, empty `activity_id`, missing `@version`) is refused before any process is spawned.
*Verified by:* a `trybuild` test that a `dispatch` call without the key does not compile (the port takes it by value,
not `Option`); plus a unit test on the key constructor rejecting empty components.
*Note:* the key is **not** required to be globally fresh — re-entry under AC-013 deliberately reuses `item`, and
uniqueness is asserted only over the triple (AC-011). **Correction (2026-09-11, audit PIA-008).** The key was written
`(run_id, activity_id, attempt)` here and four-part in the activity-graph schema; ADR-0097 decision 2 fixes it as the
three-part `(item, activity_id@version, attempt)` everywhere, with `attempt` incrementing and `activity_committed`
(AC-012) emitted **per step activation** under ADR-0022. The memo key (AC-014) is a different formula.

**AC-011** — `rk-kernel::activity` · evidence: proposal §4.2, §4.3 outer-retry row [documented] · decisions: ADR-0022, ADR-0037
GIVEN an activity that has failed with a retryable class
WHEN the kernel retries it
THEN the new dispatch carries `attempt + 1` and never reuses a previous attempt number.
*Verified by:* a unit test over the retry loop asserting strictly increasing `attempt`; an event-log assertion that
`(item, activity_id@version, attempt)` is unique across the whole log (ADR-0097; PIA-008).

**AC-012** — `rk-kernel::activity` · evidence: proposal §4.3 "a step that *committed* is memoised by its key and never
re-runs" [documented] · decisions: ADR-0022
GIVEN two attempts of the same `(item, activity_id@version)` step activation where the first has already committed its
result
WHEN the second attempt returns late
THEN the late result is discarded as a duplicate and the committed value is unchanged.
*Verified by:* a concurrency test in `rk-kernel` driving two attempts against the fake store; an assertion that at most
one `activity_committed` event exists per step activation of `(item, activity_id@version)` — the event is per step
activation, not per activity (ADR-0097 decision 2; PIA-008).

**AC-013** — `rk-kernel::replay` · evidence: **F38** [measured: 3 runs, 0 changed, 0 lost, digests identical across a
real `taskkill /T /F`]; F19 [measured: 174/174 at activity granularity] · decisions: ADR-0022 — **PROVISIONAL**
GIVEN a run whose graph committed N steps and whose process was killed mid-graph
WHEN `redkiln run <item>` re-enters (there is no `--resume` flag)
THEN no committed step re-executes and every committed digest is byte-identical to its pre-kill value.
*Verified by:* a kill-and-resume integration test in `rk-kernel` over an N-step fixture graph with the Harness fake,
asserting digest equality and zero re-dispatch.
**PROVISIONAL:** P8 is PARTIAL — 3 of a pre-registered 27+3 runs, and its recorded state held **exactly one step**;
the 65-min arm, the 24-h arm and all three controls are unrun. The kernel test generalises to N what the spike
measured at 1.

### 2.3 The content-addressed memo key (D21)

**AC-014** — `rk-kernel::memo` · evidence: proposal §4.2 [documented] · decisions: ADR-0021, ADR-0022
GIVEN the step memo key — `SHA-256` over a length-prefixed (`u32-length || bytes`), ordered tuple of exactly these
eight components (ADR-0097 decision 2): (1) `activity_id@version`; (2) the rigor variant selected; (3) the step id
within the activity graph; (4) the digest of each resolved step input in declaration order, each canonicalised by
`rk-store`'s canonical serialiser (sorted keys, fixed float format, LF); (5) the card digest, or for a `tool` step the
Toolchain adapter's detected version string plus the command; (6) the digests of the resolved KB atoms, sorted; (7) the
loop iteration counter (0 outside a loop); (8) the output type's `schema_version`
WHEN any one component changes and all others are held fixed
THEN the key changes and the memo lookup misses.
*Verified by:* a property test in `rk-kernel::memo` parameterised over each key component (**eight** cases, one per
component), each asserting a miss; and a companion test asserting a hit when all eight are identical.
**Correction (2026-09-11, audit PIA-008).** The key was written as a five-component `hash(...)` with three different
component lists across the register, the design and the schema and no hash framing; ADR-0097 fixes the list, the
order and the framing, and this criterion and AC-019 enumerate exactly it.

**AC-015** — `rk-kernel::memo` · evidence: proposal §4.2 ("makes a stale `.kb` atom invalidate stale work"), §4.8
[documented] · decisions: ADR-0021
GIVEN an activity whose declared inclusion rules injected KB atom `A` and whose result is memoised
WHEN `A` is edited or superseded
THEN the next execution of that activity misses the memo and re-runs.
*Verified by:* an integration test over the store fake that edits an injected atom and asserts a `memo_miss` event
followed by a re-execution.
*(A `doctor` rule listing memo entries whose injected atoms no longer resolve is a useful diagnostic but verifies
nothing in this THEN; it belongs to the doctor area, not here.)*

**AC-016** — `rk-kernel::memo` · evidence: **P9b finding A18** [documented — ADR-0021 records it as an observed,
named failure mode with **no n and no figure**; it is not a measurement] · decisions: **ADR-0021**, ADR-0022
GIVEN an activity realised as an encapsulated graph of steps
WHEN the memo key is computed
THEN a key exists per *step* (AC-014's tuple), and the activity-level key the stage sees is a **separately derived**
value — `SHA-256` over the digests of every card reachable in the subgraph (ADR-0022; ADR-0097 decision 2) — not the
step key and not a hash of it.
*Verified by:* a unit test asserting that changing one card deep in a subgraph changes the activity-level key;
a lint that rejects an activity-level key computed from a single card reference; a unit test that the two keys are
different formulas (the activity-level key is unchanged when a step's resolved inputs change).

**AC-017** — `rk-kernel::memo` · evidence: **P9b finding A18** [documented, no n] · decisions: **ADR-0021**,
ADR-0043, ADR-0055
GIVEN a step inside a `repeat` / `on_false` cycle whose resolved inputs are byte-identical across two iterations
WHEN iteration 2 executes
THEN it does not hit iteration 1's memo entry, because the loop iteration counter is part of the step key.
*Verified by:* a unit test over a two-iteration fixture loop asserting two distinct keys and two executions —
the direct regression test for "loops cannot learn" (the rejected iteration-blind key).

**AC-018** — `rk-kernel` pack lint · evidence: ADR-0021 "Negative"/"What this forbids" [documented];
P9b finding A18 [documented, no n] · decisions: **ADR-0059**, ADR-0097 (withdraws ADR-0021's per-step `memo = false`
clause), ADR-0064
GIVEN a `tool` step whose verb's registry entry carries `live = true`
WHEN the step is memoised, or declares `memo = true`
THEN pack validation fails with a lint error at the step's span — liveness is the verb registry's flag and nothing else,
and a step-level `memo = false` declaration is itself a pack validation error (there is no per-step memo field).
*Verified by:* a lint rule with a rule-table row and two must-fail fixture activities (a live verb memoised; a step
declaring `memo = false`); a positive fixture where a non-live verb memoises. ADR-0059 makes the verb's `live` flag
imply no memo; ADR-0097 decision 3 withdraws ADR-0021's forbids bullet on the per-step declaration.
**Correction (2026-09-11, audit PIA-008).** As authored this criterion demanded the per-step `memo = false` rule that
accepted ADR-0059 had withdrawn; its WHEN could never be true under ADR-0059's registry flag and the lint was vacuous.
Rewritten to ADR-0097 decision 3; §15.2's ADR-0059 row now lists it.

**AC-019** — `rk-kernel::memo` · evidence: proposal §4.7 "attribution is free" (L6.3) [documented]; §C.12 method finding
(measure mechanism, not success rates) · decisions: ADR-0021, ADR-0036
GIVEN a step whose execution is short-circuited by a memo hit
WHEN the run proceeds
THEN a `memo_hit` event carrying the key and its **eight** components (exactly AC-014's list, ADR-0097) is appended; a
skip is never silent.
*Verified by:* an event-schema test asserting the field set; a replay assertion that executed steps plus `memo_hit`
events equals the number of **step activations** in the recorded run — not the graph's static step count, which
`repeat`/`on_false` (AC-017) and `for_each` (AC-022) make unequal to it.

### 2.4 Budgets — three counters, never shared

**AC-020** — `rk-kernel::budget` · evidence: proposal §4.3 budgets row (shipmates L5.5) [documented] · decisions: ADR-0037
GIVEN the three budgets — attempts per activity, tokens per item, wall clock per run
WHEN one of them is consumed or exhausted
THEN neither of the other two changes.
*Verified by:* a unit test per pair (three cases) asserting independence; a `trybuild` separation test — the three are
distinct newtypes and no `From`/`Add` impl crosses them, so a shared counter does not compile.

**AC-021** — `rk-kernel::budget` · evidence: activity-graph schema §9a [documented] · decisions: **ADR-0037**
GIVEN an activity whose graph runs A agent/tool steps in one pass and then retries the pass
WHEN `attempts_total` is evaluated
THEN it counts the agent/tool steps of a pass, not the sum of retried steps across passes.
*Verified by:* a unit test over a two-pass fixture asserting `attempts_total == A`, not `2A`.

**AC-022** — `rk-kernel::budget` · evidence: activity-graph schema §9b amendment 23 [documented] · decisions: **ADR-0053**, ADR-0035
GIVEN a `for_each` step projecting over N items
WHEN the attempts budget is charged
THEN the step counts once, independent of N.
*Verified by:* a parameterised unit test at N = 1, 2, 50 asserting a constant charge.

**AC-023** — `rk-kernel::budget` · evidence: proposal §4.7 "per-item appetite fixed at triage" [documented] · decisions: ADR-0022
GIVEN an item whose token appetite was fixed at triage
WHEN any activity attempts to raise it mid-run
THEN the raise is refused unless it arrives as a typed budget-change event with an author.
*Verified by:* a unit test asserting refusal of an in-run raise from an activity result; an event-schema test that
every appetite change carries an author field.
*(The `doctor` rule listing items whose appetite differs from its triage value is a diagnostic, not an instrument
for this THEN.)*

**AC-024** — `rk-kernel::budget` · evidence: **F22** [measured: ~12 background kills vs one 12,000 s detached survivor];
principle 3 "nothing waits" [documented] · decisions: ADR-0022, ADR-0017
GIVEN a run that reaches its wall-clock budget
WHEN the budget expires
THEN the kernel writes a durable mark and exits 0; no thread sleeps waiting for the budget to be extended.
*Verified by:* a test asserting process exit and a written mark; the AC-003 lint (no `sleep` in the workflow) is the
structural half of the same guarantee.

**AC-025** — `rk-kernel::budget` · evidence: principle 4 (GATE-01, RK-B0013) [documented]; F31 via §C.7 [measured: four
adjudicated defects survived fully green suites of 1890/1890 and 1907/1907 — the corpus's warrant for never mapping
absent evidence to `pass`] · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a gate whose evidence-producing activity ran out of any budget
WHEN the gate is evaluated
THEN the result is `Err(BudgetExhausted)` and the gate does not close.
*Verified by:* a unit test per budget asserting the gate does not close; a negative fixture proving no code path maps
an exhausted budget to `pass`.

**AC-026** — `rk-kernel::budget` · evidence: proposal §4.3 inner-layer row [documented]; `system/api_retry` **payload
captured** — the runner's `api_retries: Vec<ApiRetry>` (`spikes/runner/src/runner.rs:61`) holds `{attempt, error,
error_status, max_retries, retry_delay_ms}` on **139 rows across 12 result files** (`e3.jsonl` 26, `p7.jsonl` 38,
`s-var.jsonl` 28, `s-typed.jsonl` 18, `e0-stability` 6, `p8` 5, `s-var-2400` 5, `h4` 4, `p1-arm-l-v2` 4,
`s-typed-oldschema` 3, `e3-pilot` 1, `e3h` 1) [measured, `spikes/results/*.jsonl`, recount 2026-09-11] · decisions:
ADR-0037
GIVEN a dispatch during which the harness emitted `system/api_retry` events carrying an attempt counter
WHEN the activity's attempt budget is evaluated
THEN those inner attempts are counted against the activity's budget and the kernel does not add a retry of its own for
the same failure.
*Verified by:* a unit test over the **captured** stream fixture — rows from `spikes/results/e3.jsonl` (and `p7`/`s-var`)
at the corpus pin, containing real `api_retry` events — asserting the counter increments and the outer retry count stays
at zero for that failure.
**Correction (2026-09-11, audit PIA-068).** The line "the payload was never captured" was false: 139 rows carry it. The
recorded per-row attempt distribution is 1 attempt on 89 rows, 2 on 21, 3 on 12, 4 on 4, 5 on 1, 6 on 6 and **10 on 6**
(the `max_retries = 10` ceiling, reached six times); 284 attempts in all — `overloaded`/529 on 67, `unknown`/null status
on 217; `max_retries` is 10 on every row [measured, same recount; interval not computed]. PROVISIONAL is lifted; the
payload-capture check is discharged against the existing record, and §0.3's C1 is decidable by measurement. S-RETRY
keeps only its throttle-margin arm and gains exhaustion-on-result (ADR-0095, AC-523); it no longer waits on a
payload capture. Whether inner attempts charge the budget (AC-391's cut) stays C1's question.

### 2.5 Layered retry — and what is never retried

**AC-027** — `rk-kernel::retry` · evidence: proposal §4.3 retry table [documented]; F7/F8/F20 [measured: adoption and
isolation defects are silent and configuration-caused — `--agents` inert without `--agent`; 56% of a default cache
write is unrelated MCP; a `tools` allowlist voids `--json-schema` at exit 0] · decisions: ADR-0016, ADR-0010
GIVEN a `StepError` of class `NotAdopted`, `NotIsolated`, `Unsupported(_)` or `Overage`
WHEN the outer retry policy is applied
THEN zero re-dispatches occur and the run fails with the configuration error surfaced.
*Verified by:* a table-driven unit test over the four classes asserting dispatch count 0 **and** a surfaced typed
error; these are configuration, not weather, and a retry against any of them is a defect (tripwire in SM-003).

**AC-028** — `rk-kernel::retry` · evidence: proposal §4.3 `StepError` enum [documented] · decisions: ADR-0022
GIVEN the `StepError` enum in `rk-types`
WHEN a new variant is added without a row in the retry policy table
THEN the kernel fails to compile.
*Verified by:* an exhaustive `match` over `StepError` in the policy function with no wildcard arm; a compile-fail test
(`trybuild`) proving the wildcard cannot be reintroduced.

**AC-029** — `rk-kernel::retry` · evidence: proposal §4.3 [documented]; AWS "Exponential backoff and jitter" [documented]
· decisions: ADR-0022
GIVEN a `Transport`, `Platform 5xx` or `Timeout` failure
WHEN the kernel retries
THEN the delay is exponential with full jitter, base 2 s, capped at 5 min, and the attempt count never exceeds the
pack's `N`.
*Verified by:* a unit test with an injected clock and seeded jitter asserting the delay envelope and the attempt cap;
a pack-validation test that `N` is declared, never defaulted in code.
*Note:* base 2 s and the 5 min cap are **chosen constants, not measured** — no spike sized them (gap 3). The criterion
verifies conformance to the declared envelope, not that the envelope is right.
*Home of the numbers (2026-09-11, PIA-055).* Base and cap are **kernel retry-policy constants** — the position of this
criterion, AC-233, the activity-graph schema ("a step never defines its own backoff") and design §3.6 :338; only `K`,
`N` and the appetite are pack values. Design §3.6 :345 and §3.10 :424 ("every one of these numbers is declared in the
pack and refused as a code default") are the sentences to amend, per ADR-0092's precedence (the register owns the
criterion). ADR-0095 leaves AC-029's parameters unchanged. §0.3's authority table carries the row. The AWS post is
cited for the backoff *shape* (full jitter) only; it recommends Full or Decorrelated Jitter and calls Equal Jitter "the
loser", and neither "base 2 s" nor "cap 5 min" appears in it [documented, fetched 2026-09-10; grounding
EXT-DURABLEE-009].

**AC-030** — `rk-kernel::retry` · evidence: **F22** [measured]; F1/F6 [measured: the top-level `rate_limit_event` is the
only headroom signal and `utilization` is quantised to 0.01]; principle 3 [documented] · decisions: ADR-0022, ADR-0017
GIVEN a `RateLimited { resets_at }` failure
WHEN the kernel handles it
THEN it writes a `retry_at(resets_at)` mark and exits 0; it never sleeps until the reset.
*Verified by:* a unit test asserting exit plus a written mark and zero elapsed sleep with the injected clock; the
next `redkiln run` resumes from the mark (AC-013's re-entry test).
*Note:* `resets_at` is observed in the headroom recordings (`spikes/results/e0-headroom.jsonl`, unix seconds) but it
arrives on `rate_limit_event`, **not** on a failure result — the adapter must derive `RateLimited { resets_at }` from
the stream, and if it cannot, the mark degrades to a bare `retry_after` with a pack-declared delay.

**AC-031** — `rk-kernel::retry` · evidence: **F20** [measured: a tool-restricted card silently voids `--json-schema` at
exit 0]; F31 [measured: schema conformance 52/52 = 100% on one card/schema pair] · decisions: ADR-0016, ADR-0006
GIVEN a dispatch whose returned value fails schema validation at the boundary
WHEN the kernel handles it
THEN it re-prompts exactly once carrying the validation error, and on a second failure returns `Schema { attempts: 2 }`.
*Verified by:* a unit test over a scripted responder returning two invalid values, asserting exactly two dispatches and
the typed error.
*Sub-clause (compound, split blocked — no free id in AC-001..AC-039):* the boundary never extracts structure from
prose. Verified separately by a grep-lint over the boundary module rejecting any markdown/JSON-from-prose extraction
path, with a must-fail fixture. This is a build-time ban of the AC-001..AC-005 family and should take its own id when
the range is next widened.

**AC-032** — `rk-kernel::retry` · evidence: **RK-B0013** [documented]; **F29** [measured: 19/19 rejections on-target,
and the verdict flips on 3 of 10 draws while the claim does not] · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN an activity that completed successfully and returned findings that block
WHEN the retry policy is consulted
THEN no retry occurs — a domain rejection is not an error class.
*Verified by:* a type test proving the success and failure channels are disjoint (`Result<Findings, StepError>`), so
"the reviewer said no" cannot be constructed as a `StepError`; plus a unit test asserting dispatch count 1.

### 2.6 Circuit breaker

**AC-033** — `rk-kernel::retry` · evidence: proposal §4.3 breaker row [documented] · decisions: ADR-0022
GIVEN K consecutive `Platform` failures for one model family
WHEN the next dispatch for that family is requested
THEN it is refused and a durable breaker mark for the family is written.
*Verified by:* a unit test driving K consecutive failures and asserting refusal plus the mark; `K` is a pack value,
asserted present by pack validation.
*Note:* no corpus figure sizes `K`, and no spike measured a `Platform` failure rate at all (gap 3).

**AC-034** — `rk-kernel::retry` · evidence: proposal §4.3 "per model family; other families continue" [documented]
· decisions: ADR-0022
GIVEN an open breaker on one model family
WHEN a dispatch is requested for a different family
THEN it proceeds; the breaker never trips globally.
*Verified by:* a two-family unit test asserting one refusal and one success; a lint banning module-level `static`/
`lazy_static`/`OnceLock` breaker state, so process-wide breaker state cannot be introduced.

**AC-035** — `rk-kernel::retry` · evidence: **F22** [measured: ~12 background kills vs one 12,000 s survivor — nothing
may depend on a process surviving, §C.6] · decisions: ADR-0022, ADR-0017
GIVEN an open breaker
WHEN the process dies and `redkiln run` is invoked again
THEN dispatches to that family are still refused until the mark is read and found expired; breaker state is never
held only in memory.
*Verified by:* a kill-and-re-enter test asserting the refusal survives the restart; a lint that breaker state is
constructed only from the store.
*Note:* the expiry comparison reads the current time as a **journaled activity result** (AC-001), never from the
workflow's own clock.

### 2.7 Degradation — a typed value, never silent

**AC-036** — `rk-kernel` policy · evidence: proposal §4.3 degradation row [documented]; F15 [measured: reasoning effort
is applied per dispatch — thinking tokens 0.0 / 4.8 / 221.0, complete separation, though low vs high did **not**
separate]; F7 [measured: the card's model + effort are accepted] · decisions: ADR-0014, ADR-0016
GIVEN an activity whose card names a model tier
WHEN the kernel would substitute a different model (Opus → Sonnet) and the pack does not permit substitution
THEN the substitution is refused and the activity fails rather than running at the lower tier.
*Verified by:* a unit test over a pack without the permission asserting refusal; a must-fail fixture pack.

**AC-037** — `rk-kernel` policy · evidence: proposal §4.3 [documented]; F28 [measured: the digest carries what the next
step uses — H4 Jaccard FULL 1.000 = CONTROL 1.000, effect +0.000] · decisions: ADR-0014, ADR-0006
GIVEN a permitted model substitution
WHEN the activity completes
THEN `Degraded { from, to, why }` appears in both the event log and the activity's digest.
*Verified by:* an event-schema test and a digest-schema test asserting the field set; a property test that a
substituted dispatch cannot be constructed without the record.

**AC-038** — `rk-kernel` policy · evidence: proposal §4.3 [documented]; **ADR-0015** (review tier selects execution-grade
or read-only) [documented]; F31 [measured: the typed contract's adjudication was **14/14 confirmed**; the arm cost
**+74%** — $112.25 total against the $64.61 prose comparator, a **spend** ratio, not a measured price of a review tier]
· decisions: ADR-0015, ADR-0020, ADR-0074 (supersedes ADR-0004)
GIVEN a gate whose policy demands a review tier
WHEN the review that would close it was produced under a `Degraded` substitution below that tier
THEN the gate does not close and the reason names the degradation.
*Verified by:* a unit test pairing a degraded review with a tier-demanding gate, asserting non-closure; a policy-table
row test covering each rigor level (ADR-0020's deontic enum).

**AC-039** — `rk-kernel::budget` / human gate · evidence: F18 [measured: **gate→verdict** n=174, median 10.5 min,
45% > 1 h; the max 226 h / 9.4 days belongs to **stage→verdict** n=191, not to gate→verdict; zero model cost, one
operator]; F38 [measured: mid-graph resume 0 changed / 0 lost / digests identical across `taskkill /T /F`, n=3, PARTIAL;
the three sub-unity ratios it reported (0.45 / 0.55 / 0.78) were output tokens over F28's cache-creation tokens and are withdrawn — on the like
counter the resumes created 84,960 / 49,468 / 46,961 cache tokens, and the cost ordering is UNMEASURED (E-0087-1,
ADR-0106)] · decisions: ADR-0017, ADR-0009
GIVEN a run halted by an exhausted wall-clock or token budget
WHEN the operator decides whether to extend the budget or abandon the item
THEN the decision arrives as a gate answer bound to `(token, tree_hash, deadline)` and the kernel takes no action until
it does.
*Verified by:* routed to a **human gate**; **not machine-decidable** — whether more spend on this item is worth it is a
value judgement about the item, and no measured signal in this corpus discriminates it. The *mechanism* is machine-checked
(a test asserting the halt writes a gate record, and that an answer whose tree hash no longer matches is void, not
reconciled), but the answer is not.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-001 | Memo effectiveness | no baseline |
| SM-002 | Re-entry cost | — |
| SM-003 | Retry composition by error class | saturating; no baseline |
| SM-004 | Which budget binds, and how often | — |
| SM-005 | Cost per activity and per run | — |
| SM-006 | Headroom consumed per run | no firing kill |
| SM-007 | Circuit-breaker trips | no firing kill; no baseline |
| SM-008 | Degradation frequency and its consequence | not a success measure; no baseline |
| SM-009 | Replay divergence | saturating; no baseline |
| SM-010 | Schema re-prompt rate | saturating |

#### Residual risk this area could not repair

- Nothing was cut whole. 39 of 39 surviving is also what an insufficiently adversarial pass looks like; the closest calls were AC-026 (kept only because `system/api_retry` was verified to exist in `spikes/results/e3.jsonl`) and AC-009 (largely subsumed by AC-008 once its zero-dispatch half is removed).
- AC-026 may be unimplementable as written: its whole THEN depends on `system/api_retry` carrying an attempt counter, and the corpus recorded only an event-type census. If the payload has no counter, inner/outer double-counting has no instrument at all.
- AC-031 is still compound — exactly-once re-prompt plus the never-parse-JSON-from-prose ban. The split was blocked because AC-001..AC-039 is exhausted; the prose-extraction ban will be under-tracked until the range widens.
- AC-016..AC-018 (the loop-aware memo key) rest on ONE named failure mode observed by ONE author in ONE spike, with no n. The criteria are machine-checkable; the warrant for the design spend is thinner than the structure implies.
- Every criterion in sections A, C, D, E and F is verified by an instrument that does not exist yet. SM-009 has no baseline because the FSM has never been replayed, so 'deterministic replay' is an aspiration with a test plan.
- Roughly a third of the `[documented]` tags are unaudited proposal-section pointers (§4.2, §4.3, §4.7, §9a, §9b, principles 1/3/4). A fabricated section number would have survived this pass.

---

## 3. Selection engine — the seven-step pure function (BLOCKED on P1)

**Area key:** `selection` · **Criteria:** AC-040..AC-079 **plus AC-520 and AC-525** (AC-075 cut; AC-077..AC-079 are splits; the range is exhausted, so AC-520 and AC-525 are allocated outside it — see §0.4) — **41 in force** · **Measures:** SM-013..SM-024 **plus SM-157** (the SM-013..SM-024 allocation is likewise exhausted and SM-155/SM-156 are unallocated and never reassigned, so SM-157 sits outside the range for the same reason AC-520 does — see the companion register's §1 numbering paragraph; PIA-070)
**State:** 39 binding (AC-525 added 2026-09-11) · 1 provisional (AC-068) · 1 human-gated (AC-076)
**Sources:** architecture proposal §4.2; process-catalog proposal §2.5, §2.6; ADR-0020, 0023, 0024, 0025, 0030, 0046, 0056, 0057, 0058, 0059, 0060, 0061, 0069, 0073, **0089**, **0093** (kernel/declared source classes; AC-520's scope), **0103** (predicate grammar and null); evidence brief F19, F27, F35, F36, F37, F38, §C3, §C9, §C12; **F39** and `references/2026-09-09-change-classification.md` §4, both read from the corpus rather than from the brief, which predates them; `spikes/DECISIONS-2026-09-11.md` #4 (`stale_policy`).

*Scope note.* **This area is blocked — read section 1 first.** The engine's *semantics* are writable and are written here: purity, rank, tie-is-an-error, unsatisfiable-is-a-gate, deviation-never-edits-`R`, replay, lint. What is not writable is any criterion presuming a stable LLM-emitted context vector. AC-071 is the machine-checkable statement of the block and is not itself provisional.

*Scope note, added 2026-09-09.* Arm D is now scored [measured, F39], and the §3.7 criteria absorb it: AC-071's arm-D clause is discharged and its other four are not, AC-070's discard rule is bounded and its facet names corrected, AC-072 gains the `deriver` field and the set shape, AC-079's abstention route is extended to the boolean facets, and **AC-520** closes the gap where `source = "derived"` was gated by nothing. None of that unblocks the area, and none of it touches ADR-0079, which stays `proposed`: F39 measures a *derived* context vector, and ADR-0079 asks about an *LLM-emitted* one.

*Scope note, added 2026-09-10.* **P2's perturbation half is now scored** [measured, F40] and its **advisory half (A01–A06) is unrun**; §3.6 absorbs it in **AC-068 alone**. The area's counts do not move and **no criterion was added**, which is a deliberate departure from the F39 shape: F39 exposed a *behaviour* nothing in the register gated (`source = "derived"` with no scored record) and therefore forced AC-520; F40 exposes no ungated behaviour. It changes the *warrant* under AC-068 — from "P2 is unrun" to "the criteria that would settle it have no denominator on the registered corpus, and the one that fires prescribes a remedy the measurement contraindicates" — and the residue it leaves, the `stale_policy` default for `L`-class, is an **open decision this register may not take**. Writing that residue as a criterion would be this document deciding it. §1.2 row 2 carries the rewritten executable precondition.

### 3.1 Purity and replay (the reproducibility contract)

**AC-040** — `rk-select` · evidence: catalog §2.5 reproducibility contract [documented]; F35 for the corpus size only
(102 activities — the denominator of the 6/102 dead-activity reading — over 12 packs) · decisions: ADR-0024
GIVEN a fixed `(pack digest, catalog digest, context digest, artifact digests)` tuple
WHEN `select()` is called any number of times, in any order
THEN every call returns a **byte-identical** serialised DAG and an identical `inputs_digest`.
*Verified by:* a property test over the enumerated context lattice (the P6 corpus, 102 activities, 12 packs) asserting
digest equality across 100 shuffled repetitions; runs at zero model cost (P6 lattice work cost $0, brief §D).
*Scope limit:* the test asserts in-process, single-machine identity. Cross-machine identity is a design contract, not a
measured property — F35 is a single enumeration on one machine and licenses no cross-machine claim.

**AC-041** — `rk-select` · evidence: catalog §2.5 ("any run that cannot be replayed has a hidden input"); F8 (ambient
environment silently entered a dispatch: 12 MCP servers; MCP = 4,631 tokens = 56% of a default cache write) ·
decisions: ADR-0024
GIVEN the selection crate
WHEN it is compiled
THEN it cannot read wall-clock time, environment variables, the filesystem, the network, or a random source — the four
digests and their expanded values are its only inputs
AND a call that would need any of them fails to compile rather than reading them at runtime.
*Verified by:* an architecture lint (`cargo deny` / a workspace `forbid` test) on the selection crate's dependency and
import set — `std::env`, `std::fs`, `std::time::SystemTime`, `rand`, any socket type — plus a `trybuild` compile-fail
fixture. NEGATIVE criterion: the hidden input must become impossible, not merely absent.
*Instrument limit:* the lint decides an import/dependency set, not semantics; it cannot catch an ambient read reached
through a permitted dependency. That residual is carried by AC-042's replay assertion, not by this lint.

**AC-042** — `redkiln replay` · evidence: catalog §2.5; F19 limit ("**the FSM was never replayed**, so 'same next
action' is unproven"; 174/174 suspensions lossless) · decisions: ADR-0024
GIVEN a completed run whose event log contains its `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped }`
WHEN `redkiln replay --run <id>` re-derives the DAG from the logged inputs alone
THEN the re-derived DAG is identical to the logged one, node for node and edge for edge
AND a mismatch is reported as `HiddenInput { run_id, first_divergent_node }`, never as a warning.
*Verified by:* a replay assertion in the Phase 1 acceptance suite, run on every recorded run and failing the suite on
first divergence. **This is a first measurement: replay identity has never been measured on this system (F19's stated
limit).**

**AC-043** — `rk-select` + memo · evidence: proposal §4.2 memo key; catalog §2.5 ("the memo key is unsound") ·
decisions: ADR-0024
GIVEN a run whose replay assertion (AC-042) failed
WHEN any later run would hit a memo entry minted by that run
THEN the entry is refused and re-materialised, and the refusal is logged as `memo_evicted { reason: unsound_replay }`
AND no memo hit minted by a non-replayable run is ever served.
*Verified by:* unit test on the memo store with a poisoned run id; an event-log assertion that `memo_hit` and
`memo_evicted` never name the same `(run_id, activity_id)`. NEGATIVE criterion.

**AC-044** — `rk-select` · evidence: catalog §2.7 ("the LLM never chooses an activity"); F36 (the model's emitted
classification is not stable enough to drive selection) · decisions: ADR-0024, ADR-0030, ADR-0100 (`cargo deny` is the instrument)
GIVEN the selection crate's dependency graph
WHEN the workspace is built
THEN it does not contain the `Harness` port or any adapter that can dispatch a model
AND selecting a producer, resolving a rank, or evaluating a predicate cannot issue a model call.
*Verified by:* a workspace dependency test asserting the selection crate's transitive deps exclude the harness crate;
cross-checked at runtime by SM-019's $0-spend assertion. NEGATIVE criterion.

**AC-045** — `rk-select` · evidence: proposal §4.2 memo key `hash(activity id, resolved inputs, card digest, injected KB
atoms, toolchain version)`; catalog §2.5 · decisions: ADR-0024
GIVEN two selection calls differing in exactly one of pack digest, catalog digest, context digest or any artifact digest
WHEN a memo lookup is performed for a node common to both DAGs
THEN the second call misses the memo
AND the key type admits no construction that omits any of the four.
*Verified by:* a table-driven test perturbing one digest at a time (4 cases × N nodes), plus a struct-level test that the
key type has no `Default` and no optional digest field. NEGATIVE criterion.

### 3.2 Rank, ties and families (step 4)

**AC-046** — `rk-select` · evidence: catalog §2.5 step 4 (SHOP2 ordered methods); F35 (rank ties **0** across the
enumerated lattice) · decisions: ADR-0056, ADR-0024
GIVEN two or more admissible producers of the same artifact kind in one context — admissible meaning `min_rigor ≤ r ≤
max_rigor` at the stage's rigor `r`, an absent `max_rigor` being no upper bound (ADR-0056's ceiling; added to the GIVEN
2026-09-11, PIA-054)
WHEN the engine picks a producer
THEN the producer with the lowest `method_rank` is selected, and the selection record names the rank of every candidate
it beat.
*Verified by:* unit tests on fixture catalogs with 2, 3 and 5 candidates; the `dag_assembled` explanation row must list
the losing ranks (checked by schema conformance, SM-023).

**AC-047** — `redkiln lint` · evidence: catalog §2.5 step 4 and §2.6 ("no two producers tie at equal rank under any
context"); F35 (measured ties **0**; lattice runtime 27 s) · decisions: ADR-0056, ADR-0058
GIVEN a catalog in which two producers of one kind — or two members of one declared family — hold the same
`method_rank` and are simultaneously admissible in any enumerated context
WHEN the pack is loaded, or `redkiln lint` runs in CI
THEN load fails with `RankTie { kind, context, activities }` before any activity is dispatched.
*Verified by:* the lint rule plus a fixture catalog that must fail load; the shipped catalog's tie count must stay 0
(golden assertion, SM-014). NEGATIVE criterion.
*Cut from this criterion:* the clause "the engine exposes no runtime tie-break at all — no code path, no specificity
rule, no declaration order fallback". It is an absence claim over a whole crate that no named instrument decides; the
proposed return-type test is satisfied by any function returning a single producer and would pass on a system that
tie-breaks by declaration order. The design intent is recorded in ADR-0056; only the load-time failure is a criterion.

**AC-048** — `redkiln lint` · evidence: catalog §2.6; F35 for the post-decision residual only (unsatisfiable
`(stage, ctx)` pairs **0.3%** after decisions 1–10) · decisions: ADR-0056, ADR-0058, ADR-0020
GIVEN a rigor-driven family (spec form, test design, review, learning) whose lower members carry a `max_rigor` ceiling
WHEN the lattice lint enumerates contexts
THEN load fails unless, for every rigor level at or above the family's lowest `min_rigor`, some member remains
admissible in every context the lowest member's tags cover — fact-driven families (mitigation, deploy pattern) are out
of scope and are checked only on `applicable_if`.
*Verified by:* the family-totality lint rule; a hand-built fixture family whose ceilings leave one rigor level uncovered,
which must fail load.
*Struck:* the claimed "**30.1%** unsatisfiable against a prior 11.4%" configuration and the regression fixture pinned to
it. **No 30.1% reading exists in the evidence brief** (F35 records 23.5% → 11.4% → 0.3% and nothing else); its only
source is `spikes/p9/DECISIONS-2026-09-08.md` line 5, a scratch measurement with no stamped output (ERRATA E-0056-1).
*Ownership (2026-09-11, PIA-054):* this criterion carries the **ceiling** (`max_rigor`, ADR-0056); the **totality** check
is AC-104's under ADR-0058's declared family field, and AC-116 is a diagnostic on AC-104's failing report — the §0.3
row "Family totality, and the escalation cap" reads that way now.

**AC-049** — `rk-select` · evidence: **F35 §1.3 [measured]** — on a `node+unreal` item both change summaries are
admissible at rank 10, specificity picks the UE one, and the text summary is never produced; satisfiable and wrong
(`spikes/F35-DRAFT.md` §1.3; the condensed brief's F35 row omitted it, §15.1 restored it) · decisions: ADR-0073, ADR-0057
GIVEN an item whose derived `toolchain` set has more than one element and a required kind declared
`keyed_by = "toolchain"` (`ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`)
WHEN the required set is expanded
THEN one instance of the kind is required **per toolchain in the item's set**, each with its own producer, and the
generic producer's condition (e.g. `'text' in change`) is evaluated per instance rather than once against the item
AND a mixed item cannot be satisfied by a single instance of a keyed kind.
*Verified by:* a fixture mixed Node+UE item asserting two `ChangeSummary` nodes in `dag_assembled`; a lint rule that
rejects a keyed kind whose producer set cannot cover every configured toolchain. NEGATIVE criterion.
*Struck on 2026-09-08, restored 2026-09-11 (PIA-075):* the citation "**F35 §1.3 measured defect** — a mixed Node+UE item
received the UE `ChangeSummary` only, silently" was struck because the condensed brief's F35 entry carries no §1.3. The
primary record does: `spikes/F35-DRAFT.md` §1.3 is real and exact, and the fixture reproduces it. The strike was against
the brief, not the spike; the tag is **[measured, F35 §1.3]** and "must be re-grounded" is withdrawn (design §4.3 and
4-C carry the same restoration; AC-103 cites it). S-BIN — which gates the `ChangeSummary` type design only (ADR-0093
decision 4) — is registered and unrun.

### 3.3 Rigor and stage scope (steps 1 and 4)

**AC-050** — `rk-select` · evidence: catalog §2.3 (rank reversal in weighted sums); F27 (**verdict 2/4 items flaky**
over repeated draws — a judgement input to an arithmetic rubric is unstable) · decisions: ADR-0023, ADR-0020
GIVEN a `Rigor` value (`forbidden < discouraged < optional < recommended < mandatory`)
WHEN pack or engine code attempts to add, average, or otherwise arithmetically combine rigor values
THEN the code does not compile — `Rigor` is ordinal and admits comparison only
AND an activity with `min_rigor = recommended` is inadmissible at a stage running `optional`, and the skip is a logged
row, not an arithmetic result
AND `discouraged` is **reserved in Phase 1**: the variant exists in the enum, and a pack that sets any stage or
`min_rigor` to `discouraged` fails validation naming the site (decision #20 of 2026-09-11 — its semantics are defined
in Phase 2, not defaulted here).
*Verified by:* a `trybuild` compile-fail test (no `Add`/`Sum`/`Into<f64>` on `Rigor`) plus an admissibility unit table
across all five levels; a pack fixture declaring `discouraged` that must fail validation. NEGATIVE criterion.

**AC-051** — `redkiln lint` · evidence: proposal §4 (`forbidden` is a logged skip); catalog §3 (incidents forbid
discovery/spec/design) · decisions: ADR-0057, ADR-0020
GIVEN a stage set to `forbidden` for the item's class
WHEN any producer elsewhere in the DAG declares an unconditional `requires` naming an artifact whose only producers live
in that stage
THEN pack load fails, naming the requiring activity and the forbidden stage.
*Verified by:* the requires-resolution lint; a fixture incident pack in which a spec-stage artifact is reachable only via
`requires` and which must fail load. NEGATIVE criterion. *(Runtime half split to AC-077.)*

**AC-077** — `rk-select` · evidence: as AC-051 · decisions: ADR-0057, ADR-0020
GIVEN a run whose item class sets a stage to `forbidden`
WHEN the DAG executes
THEN no node whose activity declares that stage is dispatched, including one reached indirectly through another
activity's `requires`
AND every such exclusion appears as a logged skip row naming the forbidden stage.
*Verified by:* a runtime test on a pack that reaches a forbidden-stage activity transitively, asserting zero dispatches
of that activity and the presence of the skip row in `dag_assembled.skipped`. NEGATIVE criterion.
*(Split from AC-051: load-time failure and runtime non-dispatch are different behaviours with different instruments,
and a pack that never loads cannot demonstrate the runtime one.)*

**AC-052** — `rk-select` · evidence: catalog §2.5 step 4 [documented] · decisions: ADR-0057
GIVEN a stage requiring an artifact kind at rigor `r`
WHEN candidate producers are gathered
THEN only activities whose `stage` equals the requiring stage and whose `min_rigor ≤ r ≤ max_rigor` (absent ceiling = no
upper bound, ADR-0056 — PIA-054) are candidates
AND an activity from another stage, or one above its ceiling, is never selected as a producer, whatever its rank.
*Verified by:* unit test with an identical, lower-ranked producer registered in a neighbouring stage, which must not be
chosen. NEGATIVE criterion.
*Struck:* the gloss "F35 (candidacy enumerated per stage in the lattice run)" — the brief's F35 entry records counts and
runtime, not the enumeration's internal structure.

**AC-053** — `redkiln lint` · evidence: catalog §2.1 (human-supplied artifacts are source nodes with no producer);
worked example "initiative with an existing competitor analysis" · decisions: ADR-0057, ADR-0025
GIVEN a catalog and its `artifacts.toml` registry
WHEN the pack is loaded
THEN every unconditional `requires` kind resolves either to some activity's `provides` in an unforbidden stage or to a
kind registered `source = "human"`
AND an unresolvable `requires` fails load rather than surfacing as an unsatisfiable gate on a live item.
*Verified by:* the requires-resolution lint rule; a fixture with a dangling `requires` that must fail load.

### 3.4 Unsatisfiability is a typed gate, never a guess (steps 4 and 5)

**AC-054** — `rk-select` · evidence: catalog §2.5 step 4; F35 (post-decision unsatisfiability **0.3%**, down from 23.5%
and 11.4% — a residual that must surface, not be papered over) · decisions: ADR-0024, ADR-0056
GIVEN a required artifact kind with no admissible producer in the item's context
WHEN selection reaches step 4 for that kind
THEN selection returns `Unsatisfiable { kind, failed_predicates }` and opens a gate
AND the engine performs **no** fallback: no next-best rank, no relaxation of `min_rigor`, no substitute kind, no
dropping the artifact from `R`.
*Verified by:* unit test asserting the error variant and asserting the candidate list is untouched; an exhaustiveness
test that the producer-resolution function has no default/`_ =>` arm. NEGATIVE criterion.

**AC-055** — gate record schema · evidence: catalog §2.5 step 4; F31 (typed contract: **schema conformance 52/52 =
100%**, `repro.command` fill **147/147** — typed payloads get filled, prose ones do not) · decisions: ADR-0024, ADR-0030
GIVEN an `Unsatisfiable` gate
WHEN the gate record is written
THEN it names the artifact kind, every predicate that failed, and the **evaluated value of each facet** that predicate
read, plus the rigor in force
AND a gate record missing any of those fields is rejected by the store's single write path.
*Verified by:* schema conformance test on the gate record type (the S-TYPED instrument); a store test that an incomplete
record is refused.

**AC-056** — `redkiln lint --lattice` · evidence: **F35** (unsatisfiable `(stage, ctx)` pairs 23.5% → 11.4% → **0.3%**
after decisions 1–10 — the **8 pre-registered packs, pack-default rigor** series; the all-12-pack series beside it is
31.2% → 12.2% [measured, F35 metrics table; series labelled 2026-09-11, PIA-076]; pre-registered P6 kill >5%; lattice
runtime 27 s at run3 — 101 activities, 16 bands, 12 packs; post-decision 81-band runs unmeasured, under 2 min) ·
decisions: ADR-0056, ADR-0024
GIVEN the enumerated context lattice for the configured packs
WHEN `redkiln lint --lattice` runs at pack load and in CI
THEN every unsatisfiable `(stage, context)` pair is either absent or declared in the pack with a written reason
AND an **undeclared** unsatisfiable pair fails the build, and the run prints the pair count and rate.
*Verified by:* the lattice lint; a regression fixture pinned at the F35 pre-decision catalog which must fail (23.5%), and
at the post-decision catalog which must pass at 0.3% with every residual declared.

**AC-057** — `redkiln lint` · evidence: **F35** (dead activities 15/101 → **6/102**; the pre-registered >20% kill fired
by 0.8 points under the 8 pre-registered packs (20.8%), cleared under 12 (14.9%) and post-decision (5.9%)) ·
decisions: ADR-0024, ADR-0072
GIVEN an activity whose `provides` is required by no `require` row in any enabled pack
WHEN lint runs
THEN it is reported as a **warning** with a count and a list, and does not fail the build
AND the count, together with the enabled pack count, is written to the lint report artifact keyed by catalog version.
*Verified by:* the dead-content lint rule and its golden-file count assertion against the shipped catalog (expected
**6/102** at 12 packs); a test that the report artifact carries both numbers. The pack count is part of the assertion
because the reading moves 20.8% → 14.9% on pack set alone.

**AC-058** — `rk-select` · evidence: catalog §2.5 step 5 (`Σ cost_class` within the item's appetite) · decisions:
ADR-0024, ADR-0020
GIVEN an assembled DAG whose summed `cost_class` exceeds the item's declared appetite
WHEN step 5 validates the DAG
THEN a gate opens carrying the DAG, the appetite and the overage
AND the engine never trims, defers, or drops a member of `R` to fit the appetite.
*Verified by:* unit test asserting `R` is byte-identical before and after the appetite check on an over-budget fixture.
NEGATIVE criterion.

**AC-059** — `redkiln lint` · evidence: catalog §2.6 (La Rosa contradiction/cycle check); F35 (hard cycles **0** across
the enumerated lattice) · decisions: ADR-0024
GIVEN a catalog whose `requires` relation contains a cycle
WHEN lint runs at load
THEN load fails with the cycle path named.
*Verified by:* the cycle lint; a fixture catalog with a 3-node cycle that must fail load. *(Step-5 half split to
AC-078.)*

**AC-078** — `rk-select` · evidence: catalog §2.5 step 5 · decisions: ADR-0024
GIVEN an assembled DAG that is not acyclic
WHEN step 5 validates it
THEN validation fails before any node is dispatched, naming the cycle
AND no partially-ordered execution is attempted on a cyclic graph.
*Verified by:* a step-5 assertion test driven by a hand-assembled cyclic DAG (constructed inside the crate, since the
catalog lint of AC-059 prevents one arising from a pack), asserting zero dispatches. NEGATIVE criterion.
*(Split from AC-059: a load-time catalog lint and a runtime graph assertion are different instruments, and folding them
let the runtime half inherit the lint's evidence.)*

**AC-060** — `redkiln lint` · evidence: catalog §2.6 ("`excludes` are symmetric") · decisions: ADR-0024
GIVEN activity A declaring `excludes = [B]` while B does not declare `excludes = [A]`
WHEN lint runs
THEN load fails with `AsymmetricExcludes { a, b }`.
*Verified by:* the symmetry lint rule and a fixture pack that must fail load.

### 3.5 Deviation is typed and never edits `R` (step 7)

**AC-061** — `rk-select` · evidence: catalog §2.5 step 7 and §2.7 ("the agent never edits `R`"); **F37** (deviation was
offered **71 times and used 0** across 61 runs — the channel must stay closed by construction, not by disuse) ·
decisions: ADR-0046, ADR-0024
GIVEN any activity's typed output schema
WHEN the schema is validated at configuration time
THEN no activity output type carries a required-artifact set, a rigor override, or a producer choice — the only
selection-affecting variant an activity may return is `RequestDeviation { target, kind, claimed_predicate, why }`
AND `RequiredSet` is constructible only inside the selection crate and implements no deserialisation, so no
agent-returned value can reach it.
*Verified by:* a schema test over every shipped activity's output type (the `redkiln doctor` rule table); a type-level
test on `RequiredSet`'s visibility and its absence of a `Deserialize` impl. NEGATIVE criterion.

**AC-062** — `rk-select` · evidence: catalog §2.2 (`deviation_ok` is per-activity data); §5 risk row "deviation channel
becomes control flow by the back door" · decisions: ADR-0046
GIVEN an activity returning `RequestDeviation` with a `kind` not listed in its own `deviation_ok`
WHEN the engine receives the exit
THEN the request is refused as a configuration error and logged; it does not reach predicate evaluation and does not open
a deviation gate.
*Verified by:* unit test per `kind` (`skip`/`downgrade`/`add`) against a fixture activity declaring `deviation_ok =
["skip"]`. NEGATIVE criterion.

**AC-063** — `rk-select` · evidence: catalog §2.5 step 7; F37 (**empty denominator** — P7 §4.4(3) is recorded as not
answerable at 0 requests in 71 offers, so the branch must be exercised by fixtures, never by field data) ·
decisions: ADR-0046, ADR-0024
GIVEN a `RequestDeviation` whose `claimed_predicate` evaluates **false** against the item's evidence
WHEN the engine handles the exit
THEN a gate opens with the request, the predicate and its evaluated operands attached
AND the request is neither silently dropped nor auto-accepted, and `R` is unchanged at the moment the gate opens.
*Verified by:* unit test asserting gate emission plus `R` equality across the call; gate-record schema conformance.

**AC-064** — `rk-select` · evidence: catalog §2.5 step 7 ("re-run 2–5 and log the delta") · decisions: ADR-0046,
ADR-0024
GIVEN a `RequestDeviation` whose `claimed_predicate` evaluates **true**
WHEN the engine handles the exit
THEN steps 2–5 re-run and a `dag_amended { deviation_id, added, removed, rows_refired }` event is written naming every
change to `R`
AND the deviation record is part of the replay input set, so an amended DAG is not a hidden input.
*Verified by:* unit test on the delta event (fields present, `added`/`removed` match the fixture's expected delta); the
replay identity of an amended DAG is AC-042's assertion extended over deviation-amended runs and is measured there, not
here.

**AC-065** — `redkiln lint` · evidence: catalog §5 risk row ("human-judged criteria disguised as predicates"); F27
(**verdict 2/4 items flaky** over repeated draws; detection 0/7 flaky over 21 draws — judgement is the unstable half) ·
decisions: ADR-0033, ADR-0030
GIVEN a `claimed_predicate`, an `applicable_if`, or a `require` row `when` that does not resolve to an artifact
presence, a digest comparison, a registered facet value, or a typed verdict
WHEN lint runs
THEN load fails with `JudgementPredicate { location, text }` and the authoring guidance points at a gate node instead
AND no free-text or model-interpreted rule can enter selection.
*Verified by:* the closed-vocabulary predicate lint (grammar-level); fixture rows such as "stakeholders in agreement"
that must fail load. NEGATIVE criterion.

### 3.6 Short-circuit and staleness (step 3)

**AC-066** — `rk-select` · evidence: catalog §2.5 step 3; §2.1 (`derived_from` recorded at materialisation) ·
decisions: ADR-0024, ADR-0025
GIVEN a required artifact that exists on disk but one of whose `derived_from` digests does not match the current input
WHEN step 3 classifies it
THEN it is classified `Stale`, never `Present`
AND an artifact with no recorded `derived_from` entry for a declared input is `Stale`, not assumed fresh.
*Verified by:* unit table over (exists × digests match × digest recorded) — 8 cases — asserting the classification.
NEGATIVE criterion.

**AC-067** — `rk-select` · evidence: worked example "initiative with an existing competitor analysis"; F19 for the rate
of the **world** moving (drift 12/174 = 7%; RR 3.6 with CI [1.0, 12.9] on gates over 1 h — directional, intervals
overlap) — *not* for drift of a vouched
artifact, which the brief does not measure · decisions: ADR-0025, ADR-0057
GIVEN a human-supplied artifact recorded as a vouched root (`derived_from = []`, `source = "human"`)
WHEN an upstream artifact **of a kind listed in the registered producer's `requires` for the root's kind, in the same
stage (ADR-0025), changes digest** — that list is the edge set; a same-stage artifact outside it is not an upstream
(PIA-044)
THEN it stays `Present`, its producer is not dispatched, and any drift signal is written as an **advisory** that does not
block the stage
AND no advisory can promote itself to `Stale` or to a blocking gate.
*Verified by:* unit test on the vouched-root path; an event-log assertion that `drift_advisory` never co-occurs with a
`Stale` classification for the same artifact; a fixture where a same-stage artifact **outside** the edge set changes
and no advisory is emitted. NEGATIVE criterion. The positive half — exactly one advisory per in-set change, carried by
the next gate — is AC-525.

**AC-525** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-select` + `rk-kernel`
· evidence: ADR-0025 ("the *next* gate the item reaches carries an advisory note") [documented]; ADR-0025's own
"[unmeasured] The advisory's actual noise rate is untested" stands — SM-022 is the measure and it has no baseline ·
decisions: ADR-0025, ADR-0024
GIVEN a vouched root and a same-stage upstream artifact in the root's edge set (AC-067) whose digest changes
WHEN step 3 next classifies the root
THEN exactly one `drift_advisory { root, upstream, old_digest, new_digest }` event is appended for that
`(root, upstream, new_digest)` — never zero, never one per re-entry
AND the next `gate_opened` record the item reaches carries that advisory.
*Verified by:* a unit test over a fixture root with one in-set upstream change asserting exactly one `drift_advisory`
with all four fields; a re-entry test asserting no second advisory for the same digest; a gate test asserting the next
`gate_opened` record carries it; the AC-067 out-of-set fixture as the negative companion. *(PIA-029 named ADR-0025's
positive clause as one no criterion covered; AC-067 was the negative half only.)*

**AC-068** — `rk-select` · **PROVISIONAL (P2 — the criteria that would settle it have no denominator on the registered
corpus, and the remedy the fired criterion prescribes is contraindicated by the measurement)** · evidence: **F40** (P2's
**perturbation** half scored 2026-09-10 at pin `47faec9f` over **41 `(case, downstream)` rows** / 12 perturbation cases,
$0 model cost, labels blind and **timestamp-verified**: coarse specificity **0.73** [0.58, 0.84], fine **0.55**
[0.38, 0.70]; precision **0.00** [0.00, 0.26] coarse and 0.00 [0.00, 0.20] fine over 11 and 15 emitted positives;
**recall not computable** on either scope; fine abstains on **8 of 41 = 19.5%** [0.10, 0.34]; the **advisory** half
A01–A06 is **unrun**, and will be read over **six** cases against METHOD-P §2.2's registered **ten** — DECISIONS #15's remap, not the run) · decisions: ADR-0025, ADR-0024
GIVEN an artifact classified `Stale` whose producer's cost class — `L` here: the `cost_class` of the artifact kind's
registered producer in the requiring stage, resolved **before** step 3 classifies — is declared `stale_policy = "gate"` at
the TOML site `policy.<cost_class>.stale_policy` (enum `rematerialise | gate`; **decision #4 of 2026-09-11 sets
`rematerialise` for every class in Phase 1, coarse `derived_from` scope, fine scope available and never mandatory**, so
this branch is reached only by a pack that declares `gate` with a recorded warrant)
WHEN step 3 handles it
THEN a gate opens showing the digest diff, and the producer is not re-dispatched until the gate resolves.
PROVISIONAL because the *policy setting* — which classes get `stale_policy = gate` — was P2's output and is now a
**recorded pack value with a warrant** (decision #4), which a P2 re-run on a corpus with a positive class could reverse;
the mechanism is binding, the value is provisional. **The reason has changed, and the distinction is the whole point of
this amendment.** It is no longer "P2 is unrun": P2's perturbation half ran and is scored [measured, F40]. It is that
**the criteria that would settle the setting have no denominator on the registered corpus, and the one criterion that
does fire prescribes a remedy the same measurement contraindicates.** Unblocking precondition, executable — **inverted
2026-09-11 to the decision rule** (decision #4; PIA-059): the lint no longer asks P2's record to clear the pre-registered
lines, it asks that no pack's value was *defaulted* from P2:
```sh
redkiln lint --stale-policy .redkiln/packs --p2-record spikes/results/p2/shortcircuit.json \
  --decision spikes/DECISIONS-2026-09-11.md#4
# exit 0 only when, for EVERY cost class in every enabled pack:
#   policy.<class>.stale_policy is DECLARED (never a code default) AND names its warrant (decision #4, or a later
#   dated decision), AND the declared value matches what the warrant prescribes
#   (decision #4: rematerialise for every class; coarse derived_from scope);
#   AND rec.labels_timestamped == true
#   AND — for the value to be read as MEASURED rather than decided — rec.n_cases >= 20 with a positive class present
#       AND rec.recall/rec.precision clear METHOD-P §2.4's PRE-REGISTERED lines; until then the value is [decided],
#       and the lint reports it so.
# The record an instrument writes: score-p2.py --out spikes/results/p2/shortcircuit.json [WILL EXIST — the scorer
#   has no --out today]; AC-071's stability.json likewise [WILL EXIST].
# Historical reading of the P2 record, unchanged:
# exit 0 (old form) only when rec.labels_timestamped == true AND rec.n_items >= 20
#   AND rec.recall/rec.precision clear P2's PRE-REGISTERED lines.
# labels_timestamped: SATISFIED [measured, F40]. labelled_at 2026-09-10T15:58:08Z precedes the engine's
#   earliest record at 16:01:12Z, and every engine record carries the labels' own stamp, which the scorer
#   refuses to score against any other.
# n_items >= 20: NOT MET. 12 perturbation cases over 41 (case, downstream) rows; eight of the twenty
#   selected items produced no perturbation case at all (no artifact with two revisions).
# recall/precision: THE PRE-REGISTERED LINES ARE NOW CITABLE — METHOD-P §2.4, quoted below — but RECALL
#   HAS NO DENOMINATOR HERE: 0 of 41 rows are labelled "should re-run", so the recall line can be neither
#   cleared nor failed, and it is NOT ANSWERABLE, never 0.00 and never 1.00. Coarse precision is 0.00
#   [0.00, 0.26], which with no positive labels is a property of the label set and not a reading of the
#   engine.
# CONSEQUENCE: this command cannot exit 0 against the record that exists, and no re-score of THIS corpus
#   changes that. It needs a P2 re-run on a corpus carrying substantive upstream changes.
```
*The bar, quoted rather than paraphrased, because "pending a known bar" is now the honest description.* *(Criteria 2 and 3 were quoted here without their trailing clauses until 2026-09-10; both are restored, because "quoted rather than paraphrased" is a promise this register makes to itself. **The memo-key clause is the one with an unrecorded consequence:** criterion 2 being NOT ANSWERABLE leaves the trust level of the memo key at `S`/`M` unsettled as well as `stale_policy`, and **P2-B** is what would settle both.)* METHOD-P §2.4:
(1) coarse `derived_from` — "**recall < 0.9** or **precision < 0.7** → fine scope becomes mandatory for `L`-class
activities and `stale_policy = gate` is the default for them"; (2) "fine scope cannot reach recall ≥ 0.9 either →
`stale_policy = gate` for all `L`/`XL` activities; **the memo key is trusted only at `S`/`M`**"; (3) "advisory useful fraction **< 0.3** → PC3's advisory is demoted
to a log line (no gate text)". **Criterion 1's recall half and criterion 2 in full are NOT ANSWERABLE** on this corpus — the same
missing-denominator shape as `data_migration` in F39, cited as the same shape and with its numbers never mixed with
these. **Criterion 3 is STILL UNRUN**: the advisory half is labelled *after* the engine in its own file (§2.3) and
DECISIONS-2026-09-09 **#15**'s `human_authored` remap must be settled first. So ADR-0025's own
"[unmeasured] The advisory's actual noise rate is untested" **stands unchanged and true** — that ADR is accepted and is
not edited here — and nothing in this criterion may be read as having tested it.
*Why the corpus has no positive class, which is checkable and is not a labelling failure.* **0 of 41 rows are "should
re-run"; all 41 are "need not."** Classifying every hunk against each artifact's own YAML frontmatter boundary at the
pin, **11 of the 12 perturbation cases change nothing but frontmatter** — the keys `stage`, `status`, `updated` and
`gate_open`, which the redkiln CLI rewrites on every stage advance — and the twelfth (P01) changes one body line— a single rewritten line, which the unified diff shows as a `-`/`+` pair of
wording, *"The next stage (spec) has a clear starting point"* → *"The next stage has a clear starting point"*. The
labeller wrote *"only frontmatter properties (process/stage status), no new/updated facts of the system"* against **36
of the 41 rows** (35 the phrase exactly, one a shorter variant of it; four rows carry no reason at all and one is P01's —
"39 of 41" was 41 minus P01's two, computed rather than counted; corrected 2026-09-11, PIA-075), before any of that was
computed [measured, F40 §1]. The mechanism is a recorded decision:
DECISIONS-2026-09-09 **#19** doubled the case count from 6 to 12 by adding the **card** as a perturbation candidate
("11/20 cards have ≥2 revisions vs 6/20 for any downstream artifact"), and cards are exactly what the CLI churns — the
11 frontmatter-only cases are the 11 card (`story.md`) perturbations, and P01, the only non-card perturbation, is the
only one with a body change.
*The fired criterion's remedy is contraindicated, and that is recorded here rather than acted on.* Criterion 1's
precision half fires. Its prescribed remedy makes **fine scope mandatory for `L`-class**, and **the fine scope is the
worse of the two on this corpus**. On the aggregate the two specificity intervals overlap, so the difference is **not
separated at this n**; what separates them is the paired comparison on the **33 rows both scopes actually scored** —
coarse marks **5** `Stale`, fine marks **15**, and **10 rows flip `Present` → `Stale` going coarse to fine while none
flips the other way** [measured, F40 §3]. The mechanism is in the verdict breakdown: coarse's declared map carries
`spec.md ← <card>`, so a card write invalidates the spec and nothing else — 11 false alarms from 11 emissions; the fine
reconstruction *also* fires on `implementation-report.md` and `report.md`, which **the declared map does not connect to
the card at all**, because "which upstream changed before this artifact was last written" (§2.2) picks the card up as a
derivation edge when the card is rewritten so often that it is nearly always the most recent prior write. A document
that recorded criterion 1 as firing without recording that its own remedy is contraindicated would have absorbed half
the finding. **Nothing here settles `stale_policy`**: the design consequence is **open and needs a decision, not a
default**, and the honest route is the named spike **P2** re-run on a corpus carrying substantive upstream changes
(ADR-0012), whose named consumers are this criterion and SM-021.
*Blindness, stated precisely because it is not arm D's.* P2 is unaffected by METHOD-P amendment 8, so its chronology
proxy has not expired: the labels are **timestamp-verified**, not attested-blind, and **P2 is the only arm in this
programme of which that is true** [measured, F40]. Arm D (F39) is **panel-adjudicated**, and its amendment-8 attestation
route was never invoked. The two must not be blurred into "blind".
*The same failure class as F39, one level up.* F39's `auth` regex collided with the vocabulary of a tool whose documents
are *authored*; F40's `derived_from` reconstruction collides with the writes of a tool that stamps `stage` and `updated`
on every advance — in both, an inference over a trace **the instrument itself writes into**, and in both the refinement
meant to help imports the noise (F39 §3 measured `union` worse than `commits` alone on `toolchain` and `change`; this is
that shape at the artifact level). And P2 §2.4's criteria 1 and 2 are the **fourth** pre-registration in this programme
that its own registered sample cannot decide, after P1's α rule (F36), P7's criterion 1 (F37) and METHOD-P §1.4's 0.95
bar (F39): four is a pattern in how this programme writes pre-registrations, each having fixed a threshold without
checking that the registered corpus could produce the denominator the threshold needs. That is a result **about** the
pre-registration, carried in F40's limits — not an edit to METHOD-P, which this register may not make.
*Verified by:* unit test on the gate branch (mechanism, available now, and untouched by F40 — F40 constrains the
*setting*, not the behaviour); a schema test that `policy.<cost_class>.stale_policy` accepts exactly `rematerialise |
gate` and that a pack omitting it fails load (no code default — golden constraint 11); the inverted lint above with a
fixture pack whose value carries no warrant, which must fail. The threshold half is **not** verified by the P2 record
that now exists: that record answers **one half of one** of §2.4's three criteria, answers it in a direction whose
prescribed remedy it also contraindicates, cannot answer criterion 1's other half or criterion 2 at all, and leaves
criterion 3 unrun. The fixture that would decide this criterion is a P2 re-run whose corpus carries at least one
substantive upstream change, so that a positive class exists. *(Amended 2026-09-11 — PIA-044: the `stale_policy` schema
and the `L`-class resolution rule; decision #4: the value; PIA-059: the lint's exit condition and the [WILL EXIST]
records; PIA-075: the 36-of-41 count.)*

### 3.7 The context vector: facet hygiene and the P1 block (step 2)

**AC-069** — `redkiln lint` · evidence: **F36 + implication C9** — `unknown` is treated as `high` by position, so an axis
oscillating unknown↔low moves the required set the full band width; measured `unknown` rates novelty 46.0%, size 39.5%,
blast_radius 31.0%, criticality 27.0% · decisions: ADR-0069, ADR-0103 (the one narrowing pattern), ADR-0030, ADR-0023
GIVEN a `require` row or `applicable_if` containing an ordering comparison (`>=`, `<`, …) on an ordinal axis whose domain
includes `unknown`
WHEN lint runs
THEN load fails unless the row's predicate handles `unknown` explicitly (equality only), so `unknown` is never ordered
against `low`/`med`/`high` — the **one** narrowing pattern the lint recognises is `x != null and <ordering on x>` at the
same conjunction level, with no `or` and no `not` (ADR-0103 decision 4); any other ordering against a nullable axis is
the load-time error.
*Verified by:* the null/unknown-ordering lint (ADR-0069's rule extended to `unknown`, with ADR-0103's pattern); a fixture
row `novelty >= high` over an unknown-capable axis that must fail load; a fixture `novelty != unknown and novelty >=
high` that must load; fixtures `not (novelty == unknown) and novelty >= high` and `novelty == unknown or novelty >= high`
that must fail (the pattern is syntactic, not semantic — PIA-052). NEGATIVE criterion — the silent unknown→high promotion
must become impossible. *(Runtime half split to AC-079.)* *Amended 2026-09-11 (PIA-048/PIA-052): this criterion is
binding and in runbook Step 7b's acceptance set — it was excluded there as "P1-blocked", which it is not.*
*Scope note (2026-09-09).* This rule is **ordinal-only as written**: its GIVEN names an ordering comparison on an
ordinal axis, and the `unknown` rates it forecasts are the four ordinals'. Booleans now enter its domain, because
ADR-0089 makes a derived boolean tri-valued and `null` reaches a predicate the same way `unknown` does. The
*ordering* half does not extend — ADR-0069 already makes ordering against `null` a lint error, and a boolean has no
band to order against — so the extension that was needed is the **routing** half, and it is made in AC-079 rather
than here. Nothing in AC-069 changes; the scope is stated so that a reader does not infer boolean coverage from it.

**AC-079** — `rk-select` · evidence: as AC-069 for the ordinal half; the `unknown` rates above are this criterion's load
forecast (SM-016). Boolean half: **F39 §4** — 4 of 20 items changed no code whatsoever (every changed file is `.bklg/`
bookkeeping or telemetry), the panel returned `unknown` on all **16** of their semantic-facet cells, and the script
emitted a confident boolean on every one; that is the `excl = 4` column [measured, F39] · decisions: ADR-0069,
ADR-0103 (null semantics and the order of outcomes), ADR-0030, ADR-0089
GIVEN a context in which an ordinal axis holds `unknown`, **or a boolean facet holds `null` because no evidence for it
was computable**, and that facet is read by a row that fires
WHEN step 2 resolves the context vector and a top-level `applicable_if` or `require.when` evaluates to `null` under
ADR-0103's three-valued semantics (`null == v` is `null`; `all`/`none` over `null` are true, `any` is false)
THEN the item routes to the triage human gate with `gate_opened { reason: unknown_facet, axis }` — **counted once per
read facet, never per abstaining facet** (ADR-0103 decision 5) — for an ordinal axis no band is assigned for that axis,
and for a boolean facet **no value is assigned: the absence of evidence never resolves to `false`**
AND the abstention is recorded as an abstention, separately from an error, and is never counted as a false negative in
any scoring of the facet's deriver
AND `Unsatisfiable(kind, failed_predicates)` (AC-054) is reserved for step 4 over predicates that evaluated to `false`,
never raised for a `null` read — the order is lint at load (AC-069), gate at step 2 (here), `Unsatisfiable` at step 4.
*Verified by:* a runtime test asserting gate emission and asserting no `require` row resolved against a banded value for
that axis on that item; for the boolean half, a fixture item whose changeset is empty of code, asserting one
`gate_opened` **per facet a firing row read** (an abstaining facet no row reads opens nothing) and asserting that **no**
`require` row resolved against `false` for it; a must-fail fixture where `any` over absent evidence would pass an
activity, which must not; the negative companion — a boolean whose evidence *is* computable and resolves to `false` —
must not open a gate, or the rule is satisfied by a system that gates everything. *(Amended 2026-09-11, PIA-052/PIA-048:
gates counted per read facet; the null-outcome order; binding and in Step 7b's acceptance set.)*
*(Split from AC-069: a load-time grammar lint and a runtime routing decision are different behaviours with different
instruments; bundled, the runtime half was carried by the lint's fixture and never separately asserted.)*
*Extended 2026-09-09 to the boolean facets, under ADR-0089.* As written the criterion was **ordinal-shaped on both
sides** — "an axis holds `unknown`" and "no band is assigned for that axis" — and could not fire for a boolean at all,
because `facets.toml` declares the boolean facets `values = [true, false]`, so a derived boolean that cannot be computed
has nowhere to go but `false`. **That single registry line is what turned four empty diffs into sixteen confident
booleans** [measured, F39 §4]. ADR-0067's `recent_deploy` already emits `null` under rule
`R-recent-deploy-feed-absent` because the `toolchain:deploy_log` feed does not exist at this pin, so the pattern is
generalised here, not invented. The `axis` field carries the facet name in the boolean case; it is **not** renamed here,
and whether the payload key should change is a registry-schema question this criterion does not take.

**AC-070** — `redkiln lint` + `rk-select` · evidence: **F36** (`external_dependency` became a constant, **200/200
false** — an LLM-emitted value that carried no information); **F39** (`auth` precision **0.00**, Wilson 95%
[0.00, 0.26], over **11 positives of which 11 are false**, union source, 20 items at pin `47faec9f` — a *derived* value
carrying no signal; `data_migration` emits **0 positives on 20 items across all three sources** and is **not
measurable**) · decisions: ADR-0030, ADR-0061, ADR-0059, ADR-0089
GIVEN a facet declared `source = "derived"` in `facets.toml` — every facet the registry declares derived: **six** after
ADR-0093 (`toolchain`, `change`, `ui`, `flag_exists`, `data_migration`, `recent_deploy`; eleven at pin `47faec9f` before
the kernel/declared re-class; decision #20 of 2026-09-11) — or `source = "declared"` (`surfaces`)
WHEN a triage record arrives carrying a value for that facet
THEN the **model-emitted** value is discarded and the kernel-computed value is used, and the discrepancy is logged
AND no model-emitted value can reach a `require` row for a derived facet
AND the discard **yields to a human-confirmed value**: where a human has confirmed the fact at the triage gate, the
confirmed value wins over the derived one, the precedence is logged, and the confirmation is stored as AC-074's
decision-memory row and replayed on a byte-identical context digest rather than re-asked
AND a boolean facet with **no positive instance on the scored corpus may not key a `require` row** — with a zero
denominator precision is undefined, a perfect method and a broken one score identically, and the facet is reported as
*not measurable*, never as a pass or a failure.
*Verified by:* unit test feeding a triage record with a conflicting derived facet; an event assertion on
`derived_facet_override`; a second unit test feeding a **human-confirmed** value against a conflicting derived one,
asserting the human value wins and that the AC-074 row is written and replayed; a lint fixture declaring a `require` row
keyed on a boolean facet whose scored record carries 0 positives, which must fail load, with a positive companion (a
facet with positives) that must load. NEGATIVE criterion, except the human-precedence clause, which is positive and
carries its own passing fixture.
*Corrected 2026-09-09, and recorded rather than made silently.* The GIVEN named `toolchains`, `change_kinds` and
`touches_auth`. **None of the three is a registered facet name:** `facets.toml` declares `toolchain`, `change` and
`auth` [documented, `facets.toml`]. As written, this criterion's own GIVEN was live drift of exactly the kind AC-072
forbids — an unregistered name feeding a row — inside the criterion that governs derived facets. The names are
corrected to the registry's. `auth` is corrected, **not endorsed**: ADR-0089 withdraws the repo-wide `auth` boolean in
favour of a set-valued `surfaces` facet sourced from a checked-in `surfaces.toml`, so `auth` stood in this list only
until that registry change landed — the GIVEN now names the six derived facets ADR-0093 decision 2 leaves and the
`declared` class it creates (PIA-046, 2026-09-11) — and under ADR-0089/ADR-0093 `surfaces` may key no `require` row
until S-FACET scores a declared boundary map.
*Why the discard rule needed a bound.* "The emitted value is discarded and the kernel-computed value is used" was the
right rule when the threat was a model overwriting a sound computation. At `auth` precision **0.00** it prefers a value
carrying **no signal** over a human's [measured, F39]. ADR-0089's precedence order — DECLARED > DERIVED > PROPOSED >
ASKED, a higher class overriding a lower one for the same fact, and the override logged — and
`references/2026-09-09-change-classification.md` §4.1 settle it in one direction only: **the discard survives against a
model-emitted value and yields to a human-confirmed one.** AC-074's decision-memory row is the durable, replayable home
that already exists for the confirmation, so this clause builds no new mechanism.
*The degenerate-distribution clause now has two named instances, and they are not the same shape.* `external_dep` at
200/200 `false` [measured, F36] was the first — a constant, but a constant **with** a denominator. `data_migration` at
**0 positives on 20 items** [measured, F39 §4] is the second, and it has **no denominator at all**, which is what forced
the positive-class requirement above to be stated explicitly rather than left implicit in "degenerate". That is a
property of the corpus and not proof the rule is broken: no item at pin `47faec9f` carries a migration, and any claim
about the migration facet needs a second repository that contains one.
*Struck:* `binary_change` from the derived list. The brief's contested section records `binary_change` as **measured but
unregistered in `facets.toml`, so it can feed no require row** — it cannot simultaneously be a registered derived facet
here and an unregistered name in AC-072. F39 scores it 0.60 [0.23, 0.88] at recall 1.00 (tp 3, fp 2, fn 0, tn 15), which
measures the deriver and does **not** register the facet; the strike stands.

**AC-071** — `redkiln lint --facet-stability` · **THIS IS THE UNBLOCKING GATE** · evidence: **F36** (modal required set
identical on 5/20, 3/20, 2/20, 2/20 against an 80% line — **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line; the
anchor-rewrite repair did not fix it); **F39** (arm D **is now scored** — **140** truth cells at pin `47faec9f`, all of
them on the fixed `score-arm-d-panel.py` re-run (the 2026-09-09 run reached 136; the panel-file key fix
`resolve-open-questions` → `resolve-open-question` — ERRATA E-0089-2) under
METHOD-P amendment 9, panel-adjudicated and never human-panel, $0 model cost; no facet clears 0.95, and the ≥0.95
clause is not reachable at n=20 for any facet however good) · decisions: ADR-0030, ADR-0023, ADR-0093 (the `source`
set is `human | llm | derived | kernel | declared`; this gate reads `llm` only)
GIVEN a `facets.toml` in which any facet is declared `source = "llm"`
WHEN a pack is loaded, or CI runs
THEN load fails unless a scored P1 stability record exists for that facet meeting every clause of the executable
precondition at the head of this file (identity ≥ 0.80, n ≥ 20 items, ≥ 10 draws, arm-D panel scored, corpus pinned)
AND until such a record exists, every ordinal axis is `source = "human"` — supplied or confirmed at the triage gate — and
so are the three boolean facets the registry declared `llm` (`has_repro`, `external_dep`, `market_facing`; decision #2
of 2026-09-11, carried into ADR-0090 at runbook Step 4.4, so Checkpoint 4a asserts `grep -c 'source = "llm"'
spikes/p9/facets.toml` → 0 and every one of the eight booleans is tri-valued), and the engine refuses an LLM-emitted
value for any of them (ADR-0079, `proposed`, is the record of this block; ADR-0090 supersedes it at
runbook Step 4.4 and carries ERRATA E-0079-1's citation corrections).
*Verified by:* the `--facet-stability` lint rule; a fixture `facets.toml` declaring `source = "llm"` with no record,
which must fail load; a second fixture with a record that fails one clause at a time (5 cases). This criterion is
**not** provisional: it is the machine-checkable statement of the block.
*One clause of five is discharged, 2026-09-09; the gate is unchanged.* F39 supplies a scored arm-D record, so the
**arm-D-panel-scored** clause is satisfiable and, for arm D, satisfied. The other four are untouched and the gate still
fails: the **identity** clause measured 0.25 / 0.15 / 0.10 / 0.10 against 0.80 [measured, F36]. A discharged clause is
not a partial unblocking, and the five-case fixture set stays at five.
*Deliberately not widened.* This criterion's GIVEN stays `source = "llm"`. The sibling gap — `source = "derived"` with
no scored precision record — is **AC-520**, authored as its own criterion rather than bolted on here, because this
criterion's own residual-risk sentence (§1.1) names a loosely-implemented multi-clause gate as precisely the failure
mode: a sixth clause with a different subject would be the first thing an implementer collapsed.

**AC-520** — `redkiln lint --facet-provenance` · **THE DERIVED-FACET SIBLING GATE** · **id allocated outside this
area's range — see §0.4** · measure: **SM-157** (derived-facet precision, per facet and per source — the scored record
this gate reads; SM-157's body names AC-520 in return, so the back-link is bidirectional) · evidence: **F39** (arm D
scored over **140** truth cells at pin `47faec9f`, all scored on the fixed re-run (136 on 2026-09-09 — the panel-file
key fix, ERRATA E-0089-2) — METHOD-P amendment 9, panel-adjudicated, $0: `auth` **0/12** [0.00, 0.24] on the fixed
re-run (0.00 [0.00, 0.26] over 11 positives on the first); `flag_exists` 0.00 [0.00, 0.66]; `binary_change` 0.60 [0.23, 0.88]; `change` 0.72 [0.54, 0.85]; `toolchain`
0.73 [0.48, 0.89]; `ui` 1.00 [0.65, 1.00]; `data_migration` **not measurable**, 0 positives on 20 items;
`recent_deploy` **not measurable**, `toolchain:deploy_log` absent at the pin (E-0067-1) — the six facets ADR-0093
decision 2 leaves `source = "derived"` all carry a record, four scored and two NOT MEASURABLE; `binary_change` is scored
here but registered nowhere, which AC-070's strike and AC-072 already handle) · decisions: ADR-0089, **ADR-0093**
(supersedes ADR-0089 in part: the `kernel` and `declared` source classes this gate does not read), ADR-0030, ADR-0061,
ADR-0012
GIVEN a `facets.toml` in which any facet is declared `source = "derived"` — and only that class: `kernel` (`stage`,
`rigor`, `role`, `cost` — projections of the kernel's own state, no `deriver`, no record) and `declared` (`surfaces`, read
from `surfaces.toml`, unscored, may key no `require` row until S-FACET scores a declared boundary map) are outside this
gate by construction (ADR-0093 decision 1)
WHEN a pack is loaded, or CI runs
THEN load fails unless that facet declares a named `deriver`, a **`deriver_class`** of `lexical` or `structural`,
and a **`deriver_record`** naming the scored precision record — which must exist for that deriver over a pinned
corpus, carrying a Wilson 95% interval and a positive-instance count — a record that says **NOT
MEASURABLE** (0 positives; or a feed absent at the pin, as `recent_deploy`'s `R-recent-deploy-feed-absent`) **exists, is
pinned and is read**, and satisfies this clause
AND a facet whose scored record shows **no positive instance**, or is NOT MEASURABLE, is reported so — neither passed
nor failed — and may key no `require` row (`data_migration`, `recent_deploy`)
AND a facet whose only computation is a lexical scan of free text, file paths and diff lines included, does not satisfy
PC8 rule 2 for `require`-row purposes whatever its record says — and it declares that **by name**, as
`deriver_class = "lexical"`. Absence is never inferred: a `derived` facet with no `deriver_class` fails load, exactly as
one with no `deriver` does. The lexical bar and the no-positive-instance bar are **separate clauses doing separate
work**, and a facet may be barred by either or both: `ui` and `flag_exists` are barred as lexical while carrying
positives, `recent_deploy` is `structural` and barred for having none, and `data_migration` is barred twice over.
*Verified by:* the `--facet-provenance` lint rule; six must-fail fixture `facets.toml` files, one clause at a time — a
`derived` facet with no `deriver` field; a named `deriver` with no scored record; a record carrying 0 positives whose
facet keys a `require` row; a `deriver` declared lexical (`deriver_class = "lexical"`) whose facet keys a `require` row; a `derived` facet with no
`deriver_class`, and one whose `deriver_class` is neither of the two tokens; a `derived` facet with no
`deriver_record`, and one whose `deriver_record` names a path absent from the pinned corpus; a `kernel`-class facet
carrying a `deriver`, a `deriver_class`, a `deriver_record` or claiming a record (ADR-0093 forbids); a `require` row
keyed on a `declared` facet — and a **positive companion** that must load (a
`derived` facet with a named `deriver`, a scored record and positives; a `kernel` facet with neither; a `declared`
facet keying no row), without which the rule is satisfied by a lint that rejects everything. Runbook Checkpoint 4a
asserts **six** derived facets and six records, reconciled by name — not eleven (the earlier assertion that both counts are eleven is withdrawn). This
criterion is **not** provisional: like AC-071 it is a machine-checkable statement of a gap; unlike AC-071, as of
ADR-0093 it **passes** on the registry at the pin, because every facet it gates has a record.
*(Amended 2026-09-13. This criterion required two properties it gave no on-disk form for, and depends on a third
artifact that does not exist. **(1) Lexicality.** The clause above and this criterion's own must-fail fixture list both
turn on a deriver being "declared lexical", and no record — not ADR-0089, not ADR-0093, not AC-072's key list — said
how. It is now `deriver_class`, declared **by name rather than by omission**, which is AC-072's own rule applied one
field over and exactly the shape DECISIONS-2026-09-12 E1 chose for `nullable`. The distinction is load-bearing and
measured: of the six `derived` facets at the pin, `ui`, `flag_exists` and `data_migration` are scans over paths and
diff lines while `toolchain` and `change` are git metadata only — which is the whole of F39's finding, and it was
undeclarable. **(2) The record's location.** This criterion requires a scored record to "exist, be pinned and be
read", but `redkiln lint --facet-provenance` takes only `.redkiln/facets.toml` at every call site, so nothing said
where the lint looks. It is now `deriver_record`, corpus-relative, resolved through `CORPUS_PIN` and never through a
working tree (AC-464). **(3) And the record does not exist.** Measured 2026-09-13:
`spikes/p1/score-arm-d-panel.py` reads its five inputs and prints to stdout — it has no `--out` and persists nothing,
so the Wilson intervals and positive counts are terminal output only. The figures are measured and are quoted per
facet in `facets.toml`; the machine-readable artifact is **`[WILL EXIST]`** and is **Step 7b's to produce**, with the
lint that reads it. AC-071 is in the same state by construction: its `--stability-record
spikes/results/p1/stability.json` is `[WILL EXIST]` and now names a directory this repo no longer has, Step 2 having
moved `spikes/results/` into the corpus. Nothing is withdrawn by this amendment.)*

*The domain this GIVEN reaches — as written 2026-09-09, and as resolved 2026-09-11.* At corpus pin `47faec9f`,
`spikes/p9/facets.toml` declared **eleven** facets `source = "derived"` — `stage`, `rigor`, `toolchain`, `change`,
`role`, `cost`, `auth`, `data_migration`, `ui`, `flag_exists` and `recent_deploy` [measured, `spikes/p9/facets.toml` at
pin, 2026-09-09]. F39 scored **six** of them (`ui`, `toolchain`, `change`, `flag_exists`, `auth`, `data_migration`),
plus `binary_change`, which is scored but registered nowhere. **Five carried no scored record and no F39 figure of any
kind**: `stage`; `rigor`, which is a lookup over `policy.<class>.stages` rather than a computation over a changeset;
`role`; `cost`; and `recent_deploy`, which is underivable at this pin — ADR-0067 already has it emitting `null` under
`R-recent-deploy-feed-absent`. **So this criterion failed load on all eleven, not on the six its evidence line
enumerated**, and no runbook step or registered spike supplied a passing route [measured, PIA-001]. The register named
the honest route — "a registry change that moves a deterministic projection out of `source = "derived"` altogether,
never a quiet narrowing of this GIVEN" — and **ADR-0093 took it**: `stage`, `rigor`, `role`, `cost` are `source =
"kernel"`; `auth` is withdrawn for `surfaces` (`source = "declared"`); `recent_deploy` stays `derived` with a NOT
MEASURABLE record that this gate reads as existing. Six facets remain `derived`, every one with a record; the GIVEN is
unchanged and un-narrowed. **Which branch passes at Step 8 (6, second lint):** the registry re-class, recorded in
runbook Step 4a with anchor assertions — not S-FACET, which gates the `require` rows keyed on `ui`, `flag_exists`,
`data_migration` and `surfaces`, not Phase-1 entry (ADR-0093 decision 3).
*Why this exists.* AC-071 fails pack load when a facet declares `source = "llm"` without a scored stability record.
**There has never been an equivalent gate for `source = "derived"` without a scored precision record.** The semantic
facets are all `source = "derived"` — therefore outside AC-071's block, therefore **ungated while measurably wrong**.
That is the sharpest thing F39 carries for this register: the gate the whole document is shaped around gates the half of
the registry that was not the defect, and the half that was ran unchallenged at `auth` 0.00 over 11 positives.
*Why it is a criterion and not a sixth clause on AC-071.* AC-071's own residual-risk sentence (§1.1) names a
loosely-implemented multi-clause gate as precisely the failure mode — *"if that lint is implemented loosely … the
entire provisional apparatus unblocks silently"*. A single gate carrying two different subjects (`llm` stability and
`derived` precision), two different records and two different failure meanings is the shape that invites that collapse.
Two criteria, two lint rules, two fixture sets, two things that can fail separately.
*Id hazard, taken deliberately.* The selection area's range **AC-040..AC-079 is exhausted** — AC-075 is retired and
never reusable, and AC-077..AC-079 are already splits — so this criterion is allocated at **AC-520**, outside its own
area's range. §0.4 records the hazard. Renumbering the area was rejected on this register's standing ground: it is the
one change that breaks every citation already written.
*Limits carried, not discounted.* The records this gate demands exist for the six derived facets at the pin (four
scored, two NOT MEASURABLE), and F39's own limits travel with them: 39 of its 140 cells (the 60/39/9/32 breakdown is F39's 2026-09-09 scoring, which reached 136
of the 140; the fixed re-run scores all 140 and the breakdown is not re-derived here) rest on unanimous 3-seat model
agreement
with no human, and small-panel unanimity fails in the correlated direction [documented, preprint, arXiv:2605.29800];
the oracle is independent on `toolchain` and on `binary_change`'s Unreal shortcut and **not** on the rest; one repo, one
operator, one pin. And **METHOD-P §1.4's 0.95 bar is not reachable at n=20 for any facet however good** — `ui` is 7/7
with a lower bound of 0.65, and **73** all-correct positives are needed for a Wilson lower bound above 0.95 at the
registered two-sided z = 1.96 (92 with continuity correction; 72 Clopper–Pearson; "roughly 60" was wrong — E-0089-2,
PIA-073) — so this criterion requires that a scored record **exist, be pinned and be read**, and deliberately re-states
**no threshold** the registered sample size cannot decide. Which deriver eventually clears which bar, and whether the
engine should trust `commits` over `docs` or `union`, is the named spike **S-FACET** (ADR-0012), whose named consumer is
`facets.toml`'s `source`/`deriver` fields — re-registered (decision #12 of 2026-09-11) on a criterion 20 items can
decide: arm 3 vs arm 1 false-positive count, paired per item, with the Wilson lower bound as estimator; `auth` NOT
ANSWERABLE and `surfaces` NOT MEASURABLE on this corpus; the ≥ 0.95 bar recorded as powerless, never re-thresholded.

**AC-072** — `redkiln lint` · evidence: brief contested list (`binary_change` **measured but unregistered in
facets.toml so it can feed no require row**; `dynamism` **pre-registered with no registry entry**; `recent_deploy`
**underivable at this pin**); **ADR-0089** [documented, resting on F39] for the `deriver` companion field and for the
set shape as a third registry shape · decisions: ADR-0061, ADR-0059, ADR-0030, ADR-0089, ADR-0093 (the five `source` tokens), ADR-0103 (facet-name lexis)
GIVEN a facet name appearing in any `applicable_if`, `require.when`, or `claimed_predicate`
WHEN lint runs
THEN load fails unless that name has a live registry entry in `facets.toml` with a declared domain, source, anchor,
a declared **`nullable = true`** where the facet may abstain, and —
where `source = "derived"` — a named **`deriver`** identifying the instrument that computes it, a
**`deriver_class`** of `lexical` or `structural`, and a **`deriver_record`** naming its scored record (all three
required together; AC-520 owns what the class and the record MEAN, this criterion owns that the ENTRY carries them
— amended 2026-09-13, and the two must not be collapsed into one lint rule)
AND `source` is one of the **five** closed tokens the loader recognises by name — `human | llm | derived | kernel |
declared` (ADR-0093 decision 1): `kernel` for `stage`, `rigor`, `role`, `cost` (no `deriver`, no record — a `kernel`
entry carrying either fails load); `declared` for `surfaces` (read from `surfaces.toml`; may key no `require` row until
S-FACET scores it — a row keyed on it fails load)
AND the declared domain is one of **three** shapes the loader recognises **explicitly, by name rather than by
omission**: boolean, ordinal, or **set** (`set = true`) — the shape `toolchain` and `change` already carry and the shape
the `surfaces` facet takes, whose members disjoin under ADR-0050 while distinct facets conjoin
AND a facet whose value may be **absent** declares **`nullable = true`**; absence is never inferred from the domain.
`null` is *no evidence computed* and is **not** a member of the declared domain — the three shapes above are unchanged
and a fourth still fails load. A `require` row or `applicable_if` that reads a `nullable` facet and does not handle
`null` is a lint error at pack load (ADR-0103 decision 5(i)); a `null` read at selection step 2 emits
`gate_opened { reason: unknown_facet, axis }` counted once per read facet (AC-079), and **never**
`Unsatisfiable`, which is step 4's and is reserved for predicates that evaluated to `false` (ADR-0103 decision
5(iii)).
*(Amended 2026-09-12, DECISIONS-2026-09-12 E1: ADR-0089 decision 3 makes the boolean facets tri-valued at the
context-vector level and says AC-079's route "cannot reach [them] because their declared domain is `[true, false]`",
but names no on-disk form — and TOML has no `null` literal, so `values = [true, false, null]` is inexpressible while
deleting `values` fails this criterion's own declared-domain clause. Declaring nullability **by name rather than by
omission** is this criterion's rule, applied to nullability.)*
AND facet names match `[a-z_][a-z0-9_]*`, ADR-0030's hyphenated spellings reading as their underscore forms (ADR-0103
decision 2)
AND an unregistered facet can feed no row, so `binary_change` and `dynamism` cannot silently become selection inputs
(`recent_deploy` is registered, `derived`, NOT MEASURABLE at the pin, and may key no row for that reason — AC-520).
*Verified by:* the facet-registry lint; fixture predicates naming `dynamism` and `binary_change` that must fail load
against the current registry; a fixture entry declaring `source = "derived"` with no `deriver` field, which must fail
load; a fixture entry whose `source` is none of the five tokens, a `kernel` entry with a `deriver`, and a `require` row
keyed on a `declared` facet, each of which must fail load (ADR-0093; PIA-046); a fixture entry whose declared shape is
none of the three, which must fail load; a fixture entry that abstains without `nullable = true`, which must fail
load, and a positive `nullable = true` fixture that must load and whose `null` read emits exactly one `gate_opened`
per read facet and never an `Unsatisfiable`; the verbatim `[facet.surfaces]` block (`source = "declared"`, no `deriver`,
`set = true`, `values`) as a positive fixture that must load; and a positive `set = true` fixture that must load and
whose member predicate resolves, so the shape is recognised rather than merely tolerated. NEGATIVE criterion apart from
the positive fixtures, which are what stop a reject-everything lint passing.
*Amended 2026-09-09 under ADR-0089, in two places only.* (1) The registry entry requirement gains the **`deriver`**
companion to `source`: `source` says what class of thing computed the fact and `deriver` names the instrument, which is
what makes the requirement checkable at configuration time rather than a label. (2) The set shape is now stated. It was
previously admitted by omission — `toolchain` and `change` carry `set = true` and no rule mentioned it — and ADR-0073
keeps boolean and ordinal registries separate, so a third shape entering by omission is how a loader silently accepts a
member predicate it has no semantics for. This criterion requires the **field**; the **scored record** behind it is
AC-520's clause, and the two must not be collapsed into one lint rule.

**AC-073** — `redkiln lint` · evidence: catalog §2.4 (booleans vs ordinals) [documented]; F36 shows the failure mode a
degenerate boolean produces (`external_dependency` 200/200 false) but does not evidence this rule ·
decisions: ADR-0060, ADR-0050, ADR-0030
GIVEN a boolean facet used as an activity tag rather than in `applicable_if`
WHEN lint runs
THEN load fails with `BooleanFacetTagged { activity, facet }`.
*Verified by:* the boolean-facet lint and a fixture activity that must fail load.

**AC-074** — `rk-select` · evidence: catalog §2.4 (override stored as a decision-memory row and replayed on identical
context, Xu & Ramesh); F18 (a human gate costs operator attention: **gate→verdict median 10.5 min, n=174**; the
stage→verdict series, n=191, runs to **p90 ~9 h and max 226 h**) · decisions: ADR-0030, ADR-0025
GIVEN a human override of a facet value recorded as `(context, field, value, why)`
WHEN a later item presents a byte-identical context digest
THEN the override is replayed automatically, no gate is opened for that field, and the replay is logged
AND the replayed selection is identical to the one the operator approved.
*Verified by:* unit test on the decision-memory replay path; the AC-042 replay assertion must hold across an
override-replayed run.

**AC-076** — triage classification accuracy · **HUMAN GATE NODE — not machine-decidable** · evidence: **F39** (arm D
scored 2026-09-09 over **140 truth cells** at pin `47faec9f` — all 140 scored on the fixed `score-arm-d-panel.py`
re-run; the 2026-09-09 run reached 136 (the panel-file key fix, E-0089-2) — under METHOD-P amendment 9,
**panel-adjudicated and never human-panel**, $0 model cost: 60 cells from
a deterministic oracle over git metadata, 39 unanimous panel, 9 unanimous panel with the adversarial flag mechanically
moot, and **32 human adjudication** — Ryan, the panel's and the script's positions revealed only after he answered,
verdicts 8 true / 24 false / 0 unknown, **15 of 32 differing from the script**; the adjudication surface was a claude.ai
Artifact page that is not in the repository, and the `blind_attestation:` line at
`spikes/results/p1-arm-d-labels-ryan.md:27` was written by Ryan on 2026-09-11 (decision #13 of 2026-09-11;
PIA-096), so the 32 cells are *attested-blind* under METHOD-P amendment 8 — never timestamp-verified); **F36** for the block this gate
sits inside · decisions: ADR-0030, ADR-0089, ADR-0093
GIVEN the arm-D classifications over the corpus
WHEN a blind human panel labels each item
THEN precision and recall per axis are computed and recorded.
Recast as a gate node, with the reason: **no machine can build the answer key**, so no test may assert this and no CI
job may pass or fail on it; the machine-checkable residue is that the record exists, is pinned and is attributed. **The
gate is filled for arm D at pin `47faec9f`** — which discharges AC-071's fourth clause and no other. `source = "llm"`
stays blocked on AC-071's identity clause, 0.25 / 0.15 / 0.10 / 0.10 against 0.80 [measured, F36]. The gate node stays
open for any other corpus: one repo, one operator, one pin, and 39 of the 140 cells rest on unanimous 3-seat model
agreement with no human, which is a weaker correctness signal than its arithmetic implies and fails in the
correlated direction [documented, preprint, arXiv:2605.29800].
**Still no threshold is stated here, and the reason has changed.** It used to be that arm D was uncomputed and no
pre-registered precision/recall line was citable. Arm D is now computed and a pre-registered line does exist — METHOD-P
§1.4's 0.95 — but it **cannot be decided at this n**: `ui` is 7/7 with a Wilson lower bound of 0.65, **73** all-correct
positives are needed for a lower bound above 0.95 (z = 1.96; 92 with continuity correction — E-0089-2; "roughly 60" was
wrong), and no facet on a 20-item corpus can clear the bar however good the method [measured, F39, limit 1 as
corrected]. Applying §1.4 as this gate's threshold would report a **powerless criterion as a
failure**. The results are therefore recorded as measured — five facets fire the sub-0.95 clause, `ui` clears on the
point and not on the interval, and `data_migration` is **not measurable** for want of a denominator — and the bar is
recorded as unreachable at this sample size. That is a result *about* the pre-registration, carried in F39's limits; it
is not an amendment to METHOD-P, which this register may not make.
*Struck:* the precedent citation to "F9". The brief states plainly that **there is no F9 entry** — it is listed as cited
but unsupported and struck — so it cannot be used as evidence for anything.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-013 | DAG replay identity | no baseline |
| SM-014 | Catalog lint health: rank ties, unsatisfiable pairs, dead activities | — |
| SM-015 | Context-vector stability (the block's own metric) | — |
| SM-016 | `unknown` rate per axis (gate load transferred to the operator) | — |
| SM-017 | Deviation channel volume and warranted fraction | no firing kill |
| SM-018 | Unsatisfiable-gate rate on live items | no baseline |
| SM-019 | Selection cost | no baseline |
| SM-020 | Memo hits and unsound evictions | saturating; no baseline |
| SM-021 | Short-circuit specificity, precision and recall | **partial baseline** — the perturbation half only [measured, F40]: specificity coarse **0.73** [0.58, 0.84], fine **0.55** [0.38, 0.70], abstention **19.5%** [0.10, 0.34]. Precision is **0.00** by construction of a label set with no positive class, and recall is **not answerable**; both await **P2-B** |
| SM-022 | Advisory drift noise on vouched roots | — |
| SM-023 | Selection explanation completeness | saturating tripwire, not a discriminating measure; saturating; no firing kill |
| SM-024 | Human-supplied facet load | no baseline |
| SM-157 | Derived-facet precision, per facet and per source | discriminator, and it did discriminate — but **no facet clears 0.95 and the ≥0.95 half of its criterion is powerless at n=20**; `data_migration` is reported **not measurable**, never as a zero; id allocated outside the area range (above) |

#### Residual risk this area could not repair

- AC-049 (`keyed_by = toolchain` expansion) was recorded here as having no evidence-brief support after its citation was struck; the strike was against the condensed brief, and the primary record `spikes/F35-DRAFT.md` §1.3 carries the finding exactly (restored 2026-09-11, PIA-075). It rests on one lattice enumeration, and S-BIN — which gates the `ChangeSummary` type design it depends on — is unrun.
- Most criteria are verified by lint rules and tests that do not yet exist. 'Verified by: the family-totality lint rule' specifies an instrument, not an instrument.
- The negative criteria (AC-041 'cannot read the environment', AC-054 'no fallback', AC-077 'no forbidden-stage dispatch') are checked by import lints and fixtures that sample the space, not by proofs. AC-047 lost its absence clause for exactly this reason.
- AC-068 and SM-021 are still structurally incomplete, but **not in the way this line said when it was written**. The mechanism is testable now, as it was; the threshold is **no longer uncitable** — it is METHOD-P §2.4, quoted inside AC-068 — so 'PROVISIONAL (P2)' now means *the criteria that would settle it have no denominator on the registered corpus*, not *unspecified*. Two of the three cannot be evaluated at all (**0 of 41 rows are "should re-run"**), the third is **unrun**, and the one that fires prescribes a remedy the same measurement contraindicates [measured, F40]. SM-021 has **no numeric target and a partial baseline** — specificity only, from the perturbation half (coarse 0.73 [0.58, 0.84], fine 0.55 [0.38, 0.70]; SM §5.1) — and precision and recall remain without a baseline because the label set has no positive class: a precision of 0.00 over such a set is not a baseline (PIA-070).
- AC-071 is the single load-bearing gate for the whole block. If its lint is implemented loosely — reading only `modal_required_set_identity` and skipping `panel_verdict` or `corpus_sha` — the entire PROVISIONAL apparatus silently unblocks. One of its five clauses is now discharged by F39, which makes loose implementation *cheaper* to rationalise, not less costly: four clauses still have to fail.
- **AC-520 now passes at the pin, and that is the risk.** As written on 2026-09-09 it failed load on all eleven `source = "derived"` facets and nothing scheduled a passing route [measured, PIA-001]; ADR-0093's re-class (`kernel`, `declared`) leaves six derived facets, each with a record, so the gate is green by construction. The temptation moved: not to weaken the lint, but to move a facet into `kernel` to escape it. ADR-0093 forbids a `kernel` facet carrying a `deriver` or a record, and AC-072's fixtures enforce that; the four kernel projections are the closed list. S-FACET (re-registered, decision #12) gates the `require` rows on `ui`, `flag_exists`, `data_migration` and `surfaces`, not Phase-1 entry.
- **The area's own facet names were drifted and nobody caught it until F39.** AC-070's GIVEN named `toolchains`, `change_kinds` and `touches_auth`; the registry declares `toolchain`, `change` and `auth`. Three unregistered names sat inside the criterion set that forbids unregistered names, through authoring and a full adversarial pass. Nothing in this register cross-checks a criterion's own facet names against `facets.toml`, and that check does not exist as an instrument anywhere.
- AC-052, AC-053, AC-058, AC-060, AC-062, AC-066 rest on catalog/proposal sections the evidence brief does not cover; only their instrument shape was audited, not their content.

---

## 4. Process packs, technology packs, the activity catalog and its lint

**Area key:** `packs-catalog` · **Criteria:** AC-080..AC-118 **plus AC-527** (AC-119 unallocated and never reused; AC-116..AC-118 are splits; AC-527 is allocated outside the range — see §0.4) — **40 in force** · **Measures:** SM-025..SM-036
**State:** 37 binding (AC-527 added 2026-09-11) · 1 provisional (AC-113) · 2 human-gated (AC-114, AC-115)
**Sources:** proposal §4.2, §4.2.1; ADR-0014, 0022, 0024, 0027, 0028, 0029, 0030, 0050, 0051, 0056, 0057, 0058, 0059, 0060, 0061, 0065, 0067, 0072, 0073.

*Scope note.* Everything in the criteria set is configuration-time and spends no dispatch. SM-028 and SM-036 are the exceptions — they read runtime telemetry. F20 is the canonical defect a configuration-time check must catch *because it is invisible at run time*: the voided dispatch returns exit 0, `subtype: success`, `structured_output` absent.

### 4.1 The hard line: technology packs bind and contribute, never restructure

**AC-080** — `rk-packs` (loader) · evidence: proposal §4.2.1 "The hard line" [documented] · decisions: ADR-0014, ADR-0029
GIVEN an enabled technology pack whose TOML declares a `[stage.*]` table (a new stage, or a new activity list on an
existing stage)
WHEN `redkiln doctor` or any plan-time load of the pack set runs
THEN the load is refused as a configuration error naming the offending file and span, and no plan is produced.
*Verified by:* a loader unit test over a fixture technology pack that declares `[stage.cook]`; a `doctor` rule-table row;
the fixture must fail.

**AC-081** — `rk-packs` · evidence: proposal §4.2.1 layer table ("never owns: edits to process activities") [documented] · decisions: ADR-0014, ADR-0024
GIVEN a technology pack that names an activity `id` already declared by an enabled process pack
WHEN the pack set is loaded
THEN the load is refused as a configuration error, and the process pack's activity is never silently overridden or merged.
*Verified by:* loader unit test with a colliding activity id; a `doctor` rule-table row; a replay assertion that no run
record ever carries an activity whose `source_pack` is a technology pack outside a slot.

**AC-082** — `rk-packs` · evidence: proposal §4.2.1; ADR-0029 rationale [documented] · decisions: ADR-0029, ADR-0024, ADR-0028
GIVEN a technology pack that declares a rubric row, an acceptance requirement, or a `requires` row on a process activity
WHEN the pack set is loaded
THEN the load is refused as a configuration error — a technology pack may contribute facts, producers and artifact kinds
only.
*Verified by:* loader unit test per contribution kind (allowed: fact, producer, artifact kind; refused: rubric row,
requirement); a `doctor` rule-table row.

**AC-083** — `rk-packs` (binder) · evidence: proposal §4.2.1 binder rule 3 [documented] · decisions: ADR-0014
GIVEN a `[[slot_filler]]` whose `slot` names a slot not declared by any enabled process pack
WHEN the binder runs at plan time
THEN the plan is refused as a configuration error naming the undeclared slot, and no filler is inserted anywhere else.
*Verified by:* binder unit test on a fixture naming `verify.pre-cook`; a `doctor` rule-table row.

**AC-084** — `rk-packs` (binder) · evidence: proposal §4.2.1 binder rule 3 [documented] · decisions: ADR-0014
GIVEN two enabled technology packs each contributing a filler to the same declared slot
WHEN the binder runs twice over a byte-identical pack set and config
THEN **both fillers are present, in declaration order** — cross-pack order is the enabled-pack list in config, then
within-pack declaration order (design §4.3 rule 3; PIA-046; decision #20 of 2026-09-11) — and the two resolution records
are byte-identical.
*Verified by:* a determinism unit test (two binder runs, digest-compared records) that also asserts both filler ids
appear in the record **in that order**, so a binder that inserts nothing or orders by load order cannot pass it; a
fixture that reverses the enabled-pack list and asserts the fillers reverse; a replay assertion over the event log.

**AC-085** — `rk-packs` · evidence: proposal §4.2.1 "The hard line" (`story-ue` variant) [documented] · decisions: ADR-0014
GIVEN a run planned against a process-pack **variant** (the escape hatch used when a repo needs a shape no slot expresses)
WHEN the run record is written
THEN it names the selected variant and the source of that selection (triage decision or config key), both non-null, so
the shape change is attributable to one named choice rather than to two packs merging.
*Verified by:* a run-record schema field `process_pack_variant.{id,selected_by}` asserted non-null on a variant fixture
and absent on a non-variant fixture; a replay assertion.
*Scope, stated honestly:* this is a **schema-presence** criterion. It proves the choice is recorded; it does not prove
the choice was the right one, and "a shape a slot cannot express" is a human judgement made upstream of it.

### 4.2 The binder is deterministic and a tie is an error

**AC-086** — `rk-packs` (binder) · evidence: proposal §4.2.1 doctor rule 1 [documented] · decisions: ADR-0014
GIVEN a role declared by an enabled process pack and a configured toolchain for which no card's `fulfils` and `selector`
match
WHEN `redkiln doctor` runs
THEN doctor reports an unresolved role for that (role, toolchain) pair and exits non-zero, before any run is planned.
*Verified by:* a `doctor` rule-table row; a fixture with `toolchains = ["node","dotnet"]` and a card selector covering
only `node`, which must fail.

**AC-087** — `rk-packs` (binder) · evidence: proposal §4.2.1 binder rule 1 ("never a guess") [documented] · decisions: ADR-0014
GIVEN two candidate cards fulfilling the same role whose selectors are of **equal specificity** for the item at hand —
specificity is **subsumption over the closed selector key set** (design §4.3 rule 1; PIA-046; decision #20 of 2026-09-11):
selector A is more specific than B iff every key B constrains A also constrains with an equal or narrower value and A
constrains at least one more; two selectors neither of which subsumes the other are **incomparable**, and an
incomparable pair is `AmbiguousRole` — a pack validation error, exactly as an equal pair is
WHEN the binder resolves that role
THEN resolution fails as a configuration error naming both cards; the binder never orders them by file order, load
order, name or any other implicit tiebreak.
*Verified by:* binder unit test asserting `Err(AmbiguousRole)` and asserting both card names appear, over both the
equal-keys case and the incomparable case; a positive fixture where one selector subsumes the other and must resolve; a
`doctor` rule-table row; a property test that shuffling pack load order never changes an ambiguous case into a resolved
one.

**AC-088** — `rk-packs` (binder) · evidence: ADR-0073 [documented] · decisions: ADR-0073 (supersedes ADR-0029 on the tie-break)
GIVEN a work item touching two configured toolchains (a Node front-end and a .NET API)
WHEN the binder resolves the artifact a role must produce
THEN it resolves through **toolchain-keyed artifact kinds** and yields one card per toolchain-keyed kind, and no
specificity tie-break is consulted for the mixed case.
*Verified by:* binder unit test on a mixed-toolchain fixture asserting one card per toolchain-keyed kind; a lint rule
rejecting a re-introduced `specificity_tiebreak` path on the mixed case.
*Note:* AC-087 still ranks selectors by specificity for the **single-toolchain** case; the two rules must not be
implemented as one code path, and the lint above must be scoped to the mixed case or it will break AC-087.

**AC-089** — `rk-packs` (binder) · evidence: proposal §4.2.1 binder rule 4 [documented] · decisions: ADR-0014, ADR-0022
GIVEN a pack set, a config and a work item
WHEN the binder runs at plan time in two independent processes on two different machines
THEN both produce the same resolution (card, attached skills, fillers, tier) and the same resolution digest.
*Verified by:* a CI determinism matrix that runs the fixture corpus on **two runners of different OS** and compares
resolution digests as build artifacts (a same-process double-run cannot detect path-order, locale or hash-seed
dependence, so it is not the instrument); a replay assertion that re-planning from the event log reproduces the
recorded resolution digest. **Note:** this is determinism given a **fixed facet vector**; the stability of an
LLM-emitted vector is a separate, blocked question (see AC-113, F36).

**AC-090** — `rk-packs` · evidence: proposal §4.2.1 binder rule 4 [documented] · decisions: ADR-0014
GIVEN any completed dispatch
WHEN the operator asks "why did this run with that card"
THEN the run record answers it from recorded fields alone — card name, the selector that matched, the attached skills,
the inserted fillers and the tier.
*Verified by:* a run-record schema test asserting all five fields present and non-empty on every dispatch record in a
fixture run; a CLI test that runs `redkiln explain <activity>` **with the pack directory renamed away**, asserting all
five fields are still reported and the command exits zero — this is what makes "no TOML read required" falsifiable.

**AC-091** — `rk-packs` · evidence: F16 [inferred rationale only] · decisions: ADR-0014, ADR-0016
GIVEN a card whose attached skill set changes (a skill added, removed, or its text edited)
WHEN the activity cache key is computed
THEN the card digest changes and the memo entry for every activity bound to that card is invalidated.
*Verified by:* unit test on the digest function (skill edit ⇒ digest change); a memo test asserting that a second run
after the edit **recomputes** rather than serving the memo, read from the memo hit/miss counter.
*Rationale, correctly attributed:* F16 measured cache-read share (no-commit 1.000 · commit-each **0.689** ·
`--system-prompt` 1.000) [measured, F16]. It says nothing about card digests or memoisation; the step from "a stable
prefix only pays once" to "the card must be in the digest" is mine [inferred]. F8 was previously cited here and does not
bear on this criterion at all.

### 4.3 The `doctor` rule table (proposal §4.2.1, one criterion per row)

**AC-092** — `redkiln doctor` · evidence: proposal §4.2.1 doctor rule 2 [documented] · decisions: ADR-0014
GIVEN a card whose `fulfils` names an undeclared role, or a skill whose `attach.roles` names an undeclared role
WHEN `doctor` runs
THEN it reports a dangling reference naming the file, the span and the missing role, and exits non-zero.
*Verified by:* a `doctor` rule-table row; two fixtures (dangling `fulfils`, dangling `attach.roles`) that must fail.

**AC-093** — `redkiln doctor` · evidence: proposal §4.2.1 doctor rule 3 [documented] · decisions: ADR-0014
GIVEN a slot declared by an enabled process pack that no enabled pack fills and that is not marked optional
WHEN `doctor` runs
THEN it reports a dead slot and exits non-zero.
*Verified by:* a `doctor` rule-table row; a fixture declaring `slots = ["pre-test","post-test"]` with only `pre-test`
filled and `post-test` unmarked, which must fail.

**AC-094** — `redkiln doctor` · evidence: proposal §4.2.1 doctor rule 4 (L5.11 no dead schema) [documented] · decisions: ADR-0014, ADR-0027
GIVEN a role, skill or slot defined in an enabled pack and referenced by no enabled pack
WHEN `doctor` runs
THEN it reports a dead definition and exits non-zero.
*Verified by:* a `doctor` rule-table row; three fixtures (dead role, dead skill, dead slot).
*Rationale, with the citation corrected:* the only measured dead-reference figure in the corpus is for **dead
activities** — 15/101 → 6/102 [measured, F35] — which is AC-102's metric, not this one. **No measurement exists for
dead roles, skills or slots**, and the earlier claim that the counts moved "only once a machine looked for them" was a
causal story the brief does not support. This row is ADR-0014 given a checkable form [documented], carried by analogy.

**AC-095** — `redkiln doctor` · evidence: F20 · decisions: **ADR-0108** (ADR-0016, ADR-0014 related)
GIVEN a card bound to a role at least one of whose activities declares a JSON output schema, and whose `tools` list
omits `StructuredOutput` — or has no `tools:` key at all (a configuration error, never "inherit everything": ADR-0108
decision 2; AC-493 carries the fail-open evidence)
WHEN `doctor` runs
THEN it reports a silent-schema-void risk for that (card, role) pair and exits non-zero, at configuration time.
*Verified by:* a `doctor` rule-table row; a fixture card that must fail; a fixture card with no `tools:` key that must
fail. Distinct from the harness area's dispatch-time refusal (**AC-205** — the cross-reference read "AC-014", which is the
memo-key criterion; corrected 2026-09-11, PIA-070): this row must fire even when no dispatch is attempted, because F20
measured the failure as `exit 0`, `subtype: success`, `structured_output` absent — invisible at run time [measured,
F20].

**AC-096** — `redkiln doctor` · evidence: proposal §4.2.1 doctor rule 6 (L4.2/L4.6) [documented] · decisions: ADR-0014
GIVEN any enabled pack set containing a declared role that omits `tier`
WHEN `doctor` runs
THEN it reports the untiered role and exits non-zero.
*Verified by:* a `doctor` rule-table row; a fixture role without `tier`.
*(Split from the original compound row; the `model` half is AC-118. The author's note that "they are the same invariant"
is an authoring convenience, not a behaviour: they have distinct fixtures, distinct messages and can regress
independently.)*

**AC-097** — `redkiln doctor` · evidence: timeout history; §D per-unit figures · decisions: ADR-0014, ADR-0022
GIVEN a slot-filler activity, a process activity or a card with no `timeout_secs`, or with a `timeout_secs` that is not
a resolved number at plan time
WHEN `doctor` runs
THEN it reports an unbounded activity and exits non-zero; there is no inherited or implicit default timeout.
*Verified by:* a `doctor` rule-table row; a fixture with a missing timeout and one with an unresolvable
`${toolchain.review_timeout}`. Rationale measured: the 900 s default was "a default written for the fast condition" and
cost a CLEAN record; the ladder ran 900 → 2,400 → 3,600 with one 5,400 s retry [measured, timeout history], and the
longest E3 CLEAN review in the corpus ran **2,055 s** [measured, evidence brief §D "E3 CLEAN $1.06–4.89 (317–2,055 s)";
the brief's F21 row does not itself carry this figure, so it is cited to §D, not to F21].

**AC-098** — `redkiln doctor` · evidence: proposal §4.2.1 ("all configuration-time") [documented] · decisions: ADR-0014
GIVEN any pack set, valid or invalid
WHEN `doctor` runs to completion
THEN zero calls reach the harness port.
*Verified by:* an integration test over the **whole doctor fixture corpus including fixtures that must produce at least
one rule failure** (so a `doctor` that does nothing cannot pass it) with a spy adapter that panics on dispatch; an
assertion that `doctor` emits no `dispatch_started` event.
*Cut from this criterion:* "and zero tokens are billed". Billing is not instrumentable at this resolution — the
headroom instrument is quantised to **0.01** and per-run cost was proven unmeasurable [measured, F6]. Zero dispatches
is the checkable claim; zero tokens is its consequence, not a second assertion.

**AC-099** — `redkiln doctor` · evidence: proposal §4.2 ("validated in Rust with spans") [documented] · decisions: ADR-0014
GIVEN any `doctor` rule failure
WHEN the failure is reported
THEN the report names the file path and the TOML span of the offending declaration, and the process exits non-zero.
*Verified by:* a snapshot test over every fixture in the doctor fixture corpus asserting a `path:line:col` in each
message and a non-zero exit code.

### 4.4 Catalog-level completeness is CI, not per-file lint

**AC-100** — `rk-catalog` CI · evidence: F35 · decisions: ADR-0027, ADR-0022, ADR-0057
GIVEN a catalog in which every activity file passes per-file lint
WHEN CI runs the whole-catalog lattice over the enabled pack set
THEN a lattice failure still fails the build — a green per-file lint is never accepted as evidence of catalog
completeness.
*Verified by:* a CI job that runs the lattice independently of the per-file lint; a regression fixture (every file
individually clean, one require row unsatisfiable across files) that must fail CI and must pass the per-file lint.
Rationale: the unsatisfiable cells arise from interaction between activities in **different files**, and were found by
a 27-second whole-catalog run [measured, F35: unsatisfiability 23.5% → 11.4% → 0.3%; lattice runtime 27 s]. The area
brief states three quarters of gaps were invisible to a per-file lint [documented, area brief; the F35 ledger row
records the aggregate rates, not that split — do not quote the fraction as measured].

**AC-101** — `rk-catalog` CI · evidence: F35; P6 §3.4 kill criterion · decisions: ADR-0027, ADR-0024
GIVEN a catalog change that raises whole-catalog unsatisfiability above the registered threshold (pre-registered kill
line: >5%)
WHEN the lattice CI job runs
THEN the build fails and the report names each unsatisfiable (facet-vector × rigor) cell with the require row that has
no producer.
*Verified by:* the lattice CI job with a thresholded exit; a fixture catalog above the line. The rate is pinned to the
**pack-default rigor reading over the enabled pack set**, with item-level rigor overrides enumerated and reported but not
thresholded, and the pack count and rigor mode recorded with every quoted rate (SM-026/SM-027; PIA-046; decision #20
of 2026-09-11 — the METHOD-P keep/strike decision on the override mode stays with its registered owner). Baseline measured, **8 pre-registered
packs, pack-default rigor**: 23.5% (pre-registered reading, fires the kill), 11.4%, and 0.3% after decisions 1–10; the
all-12-pack series is 31.2% → 12.2% [measured, F35; series labelled 2026-09-11, PIA-076]. **The baseline is re-taken at
runbook Step 7e** after `bisect.toml` gains `max_rigor = "recommended"` (P7-CEIL's ceiling branch, ADR-0093 decision
5) and the lattice is re-run; that reading replaces these as the AC-101/AC-102/SM-026/SM-027 baseline.

**AC-102** — `rk-catalog` CI · evidence: F35; P6 dead-activity kill criterion · decisions: ADR-0027
GIVEN a catalog change that raises the dead-activity fraction above the registered threshold (pre-registered: >20%)
WHEN the lattice CI job runs
THEN the build fails and the report lists every activity reachable from no enabled pack at any rigor.
*Verified by:* the lattice CI job; a fixture. Baseline measured: 20.8% under the 8 pre-registered packs (fires by 0.8
points), 14.9% under 12 packs, 5.9% post-decision — 6 of 102 activities [measured, F35]. The threshold is only
meaningful with the pack count attached (see SM-027). Re-taken at runbook Step 7e with the `bisect` ceiling (ADR-0093
decision 5), as AC-101.

**AC-103** — `rk-catalog` CI · evidence: F35 (rank ties 0, hard cycles 0); F35 §1.3 [measured] for the mixed-toolchain
shape the `keyed_by = toolchain` rule (AC-049, ADR-0073) exists to catch — a tie-free, cycle-free catalog that is
satisfiable and wrong (citation reinstated 2026-09-11, PIA-075) · decisions: ADR-0056, ADR-0058, ADR-0073
GIVEN a catalog containing any rank tie between two activities in the same family, or any hard cycle in the
producer/consumer graph
WHEN the lattice CI job runs
THEN the build fails — the tolerance is zero, not a threshold.
*Verified by:* the lattice CI job; two fixtures (an injected rank tie, an injected cycle). Baseline measured: rank ties
**0** and hard cycles **0** on the v0 catalog [measured, F35], so any non-zero value is a regression, not a level.

**AC-104** — `rk-catalog` lint + CI · evidence: ADR-0056, ADR-0058 [documented] · decisions: ADR-0056, ADR-0058
GIVEN a declared family and a rigor level at which at least one of its members is active
WHEN the totality check runs
THEN the check fails unless the family is **total** at that rigor — every facet vector admitted at that rigor selects at
least one member.
*Verified by:* a lattice CI check per (family, rigor); a fixture family with a hole at `mandatory`.
*Citation corrected:* F35 was cited here and measures unsatisfiability, dead activities, rank ties, cycles and runtime —
**not family totality**. There is no measured baseline for totality; this is a design rule given a checkable form.
*Attribution (2026-09-11, PIA-054):* the totality rule is ADR-0056's (rule 26); the declared family field and its three
lint checks are ADR-0058's. This criterion owns totality; AC-048 owns the escalation cap; AC-116 is a diagnostic on
this criterion's failing report.

**AC-116** — `rk-catalog` lint + CI · evidence: ADR-0056 [documented] · decisions: ADR-0056, ADR-0058
*(split out of AC-104: a second, independently regressible behaviour.)*
GIVEN a family whose totality (AC-104) holds up to rigor *r* and fails at *r+1*
WHEN the totality check fails the build
THEN the failure report names the highest total rigor *r* and the member and facet vector that leave the hole at *r+1*
— a **diagnostic on AC-104's failing report**, not a runtime cap: the engine never computes a per-family ceiling and never
loads a catalog with a hole (ADR-0058 forbids it), so the only ceiling that exists is the authored `max_rigor` (AC-048).
*Verified by:* a lattice CI check asserting the failing report names *r* and the hole on a fixture family with a hole at
`mandatory`; an assertion that the fixture does **not** load. **Correction (2026-09-11, audit PIA-054).** As authored this
"capped escalation at *r* and reported the cap" — reading B, an engine that loads catalogs with holes, which ADR-0058
forbids and AC-104 fails; under reading A the fixture could never be loaded to observe a cap. Reworded as the
diagnostic.

**AC-105** — `rk-catalog` CI · evidence: F37; implication 10 · decisions: ADR-0081 (the ladder must be monotone), ADR-0093 decision 5 (the `bisect` ceiling), ADR-0056, ADR-0028, ADR-0020/0023
GIVEN any facet vector and any two adjacent rigor levels *r* and *r+1*
WHEN the monotonicity check runs
THEN the check fails if the activity graph selected at *r+1* omits a quality-apparatus step (adversary, policy, or
declared quality block) that the graph selected at *r* contains — escalating rigor must never DROP the quality block.
*Verified by:* a lattice CI monotonicity check across the full facet × rigor cross-product; a regression fixture
reproducing the measured defect. Measured defect: escalating to `mandatory` with `has_repro` selects `bisect`, which has
**no adversary step and no policy step** [measured mechanism, F37 / implication 10; note F37's primary criterion was
powerless at n=3, so this is cited as a **mechanism**, never as a rate].

### 4.5 Per-file lint, authoring and catalog hygiene

**AC-106** — `rk-catalog` lint · evidence: F32, F33, F34 · decisions: ADR-0027, ADR-0022
GIVEN an activity file whose count of **non-blank, non-comment lines** exceeds 150 (the count F32/F33/F34 measured; SM-031 carries the same wording — PIA-076)
WHEN lint runs
THEN lint fails naming the file and its line count.
*Verified by:* a lint rule with an over-length fixture. Baselines measured across three **authoring passes by one
operator** (not three independent authors): median 56.5 (F32), 73 (F33, +29%), 53 with sum 5,059 over 90 activities
(F34); the shipped corpus clears the kill line by ~2.8× [measured, F32/F33/F34, n=3 passes, one operator].

**AC-107** — `rk-catalog` lint · evidence: F33 · decisions: ADR-0027, ADR-0072
GIVEN a reference in an activity file that lint cannot resolve unambiguously (a step id, an output path, a facet value,
a family name)
WHEN lint runs
THEN lint reports the ambiguity and fails; it never resolves the reference by guessing a most-likely target.
*Verified by:* a lint unit test per ambiguity class asserting `Err(Ambiguous)` rather than a resolved binding; a
regression fixture drawn from F33's silent acceptances. Measured: the P9b lint **accepted 5 guesses silently** across 27
ambiguities, 20 of them new [measured, F33] — this criterion exists to make that class impossible.

**AC-108** — `rk-catalog` lint · evidence: contested list (`binary_change`, `dynamism`, `recent_deploy`) · decisions: ADR-0030, ADR-0061, ADR-0067
GIVEN an `applicable_if` predicate, a tag or a `requires` row naming a facet with no entry in `facets.toml`
WHEN lint runs
THEN lint fails naming the facet and the file — an unregistered facet can feed no require row.
*Verified by:* a lint rule; three fixtures from the recorded contested set — `binary_change` (measured but unregistered
in `facets.toml`), `dynamism` (pre-registered, no registry entry), `recent_deploy` (underivable at the pin until
ADR-0067's derived boolean lands) [documented, contested list in the evidence brief §E].

**AC-109** — `rk-catalog` lint · evidence: ADR-0060, ADR-0050 [documented] · decisions: ADR-0060, ADR-0050, ADR-0030
GIVEN a boolean facet used as a tag on an activity
WHEN lint runs
THEN lint fails — boolean facets are `applicable_if`-only.
*Verified by:* a lint rule (ADR-0060) with a fixture tagging a boolean facet; a companion test asserting the accepted
tag semantics (same facet disjoins, distinct facets conjoin, ADR-0050) on a passing fixture, so the rule cannot be
satisfied by a lint that rejects every tag.

**AC-110** — `rk-catalog` loader · evidence: ADR-0027 rationale (EPF rot from unexecuted content) [documented] · decisions: ADR-0027, ADR-0114 (the extended→shipped promotion route needs a ledger entry; an enabled-pack reference to an unpromoted activity is still the load-time error below)
GIVEN an activity that lives only in the unshipped extended catalog file
WHEN any enabled pack references it, or a run attempts to dispatch it
THEN the reference fails as a configuration error at load time and the activity is never dispatchable.
*Verified by:* a loader unit test over an extended-file reference; a `doctor` rule-table row; an integration test
asserting the extended file contributes zero entries to the resolved catalog.

**AC-111** — `rk-catalog` CI · evidence: — (no measurement) · decisions: ADR-0072
GIVEN a shipped activity file whose content digest changes in a commit without a `version` bump
WHEN CI runs
THEN the build fails naming the file and its old and new content digests.
*Verified by:* a CI job diffing shipped activity digests against the recorded versions; a fixture commit that must fail.
Note: no measurement supports a *rate* here — this is ADR-0072 given a checkable form, not an evidence-derived
threshold.

**AC-117** — `rk-catalog` CI · evidence: — (no measurement) · decisions: ADR-0072
*(split out of AC-111: the changelog check fails independently of the version check and needs its own fixture.)*
GIVEN a shipped activity file whose `version` is bumped in a commit with no corresponding changelog entry
WHEN CI runs
THEN the build fails naming the file, the new version and the missing changelog entry.
*Verified by:* a CI job joining shipped versions to changelog entries; a fixture commit that bumps a version and omits
the entry.

**AC-118** — `redkiln doctor` · evidence: F7 · decisions: ADR-0014
*(split out of AC-096.)*
GIVEN any enabled pack set in which an **authored** card file or a slot filler sets `model` (or `effort`) — the tier is a
role property the binder materialises into the resolved (bound) dispatch card; the authored card never carries it (§0
glossary; PIA-015)
WHEN `doctor` runs
THEN it reports the leaked model and exits non-zero — the tier is owned by the role.
*Verified by:* a `doctor` rule-table row; a fixture card with `model`. F7 measured that a card's model+effort **is**
accepted by the harness [measured, F7]; that nothing downstream will therefore reject a leaked model, and that this row
is "the only line of defence", is my inference from that measurement [inferred].

**AC-112** — `rk-catalog` · evidence: F34; ADR-0028 · decisions: ADR-0028, ADR-0022
GIVEN an activity that declares the delivered-artifact flag (`output.delivered = true`)
WHEN lint runs
THEN lint fails unless the activity declares a quality block or carries a registered exemption id.
*Verified by:* a lint rule with an exemption registry; a fixture with the flag and neither a block nor an exemption.
*Two honest limits:* (1) the delivered-artifact set is **authored, not derived** — a mis-flagged activity is invisible
to this rule; (2) the "starts red" claim rests on 42 of 90 *authored* activities carrying a quality block
[measured, F34], which is a **different, broader denominator** than the delivered-artifact subset (SM-032). The true
starting coverage against this rule's denominator is unknown until that subset is computed. Whether a given block is
*adequate* is AC-115.

**AC-527** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-catalog` lint ·
evidence: ADR-0080 decision 2 ("the predicate `findings.none(evidence in [Reproduced, MutationSurvived])` is forbidden as
a quality block's terminal condition") [documented]; F29 — a reproduced regression was approved 3 of 10 times, so the
empty set is not evidence [measured] · decisions: ADR-0080, ADR-0028, ADR-0103 (the quantifier semantics: `none` over an
empty list is true, which is exactly why the form cannot terminate a block)
GIVEN an activity whose quality block's terminal condition is `findings.none(evidence in [Reproduced,
MutationSurvived])`, or any predicate whose truth over an **empty** findings set is `true`
WHEN lint runs
THEN lint fails naming the activity and the predicate — a quality block terminates on a positive evidence record (a
`GreenAt(sha)` verify result, a reproduced-and-flipped finding set, a human gate answer), never on the absence of
findings.
*Verified by:* a lint rule with a red fixture activity carrying that predicate verbatim, which must fail; a red fixture
with `findings.all(...)` over an empty set, which must also fail; a positive fixture whose terminal condition names a
verify result, which must load. *(PIA-029 named ADR-0080's forbids clause as one no criterion covered.)*

### 4.6 What the selection block does and does not license

**AC-113** — `rk-catalog` selection · evidence: F36; **F39** (arm D scored) · decisions: ADR-0030, ADR-0024 —
**PROVISIONAL**
GIVEN a **fixed** facet vector that the catalog admits, and a rigor level
WHEN the catalog selection runs twice
THEN it returns the identical **and non-empty** activity set, and the selection is replayable from the recorded vector
alone.
*Verified by:* a property test over the facet × rigor cross-product asserting both equality across runs and
non-emptiness (equality alone is satisfied by a selector that always returns ∅); a replay assertion.
**PROVISIONAL / BLOCKED:** this criterion is written for a *given* vector only. Any criterion asserting catalog
completeness or coverage **over LLM-emitted vectors** is not writable: P1's kill fired 4 of 4 — the modal
required-artifact set `R` was identical on only 5/20, 3/20, 2/20 and 2/20 draws against an 80% line, with alpha drift
−0.168 on byte-identical anchors and `external_dep` constant at 200/200 false [measured, F36, 400 dispatches, 0 void,
$21.37]. **Arm D precision/recall is now computed and does not lift this block:** no facet clears METHOD-P §1.4's 0.95
bar, `auth` scores **0/12** [0.00, 0.24] (0.00 [0.00, 0.26] over 11 positives before the panel-file key fix — E-0089-2),
every positive false, and `data_migration` is **not measurable** at 0 positives on 20 items [measured, F39, 140 truth
cells at pin `47faec9f`, all scored on the fixed re-run (E-0089-2), panel-adjudicated, $0]. That
discharges AC-071's arm-D clause only; the identity clause (0.25 / 0.15 / 0.10 / 0.10 against 0.80) still holds. **P2's
perturbation half is now scored and its advisory half is unrun** [measured, F40]; it answers nothing this criterion
asks, and two of its own three criteria have **no denominator on its registered corpus**, so it stays a blocking spike
for the short-circuit policy rather than for the vector. Blocking spikes: **P1** — the identity clause, not arm
D, which has now run — **P2**, and **S-FACET**, which decides what computes the derived facts and which source the
engine trusts; since ADR-0093 (2026-09-11) S-FACET gates the `require` rows keyed on `ui`, `flag_exists`,
`data_migration` and `surfaces`, not Phase-1 entry, and AC-520 gates the six `source = "derived"` facets, each of
which has a record at the pin.

**AC-114** — `rk-catalog` curation · evidence: ADR-0027 [documented] · decisions: ADR-0027, ADR-0114 (a promotion decision cites the run id of the activity's ledger entry) — **HUMAN GATE**
GIVEN a proposed addition to the shipped v0 catalog
WHEN the addition is reviewed
THEN a named human records the ship/extend decision against the curated-catalog ceiling, and the decision id is stored
with the activity.
*Routed to a human gate; not machine-decidable.* Reason: "is this activity worth shipping" is a judgement about future
use, and the only evidence the corpus offers is negative (catalogues rot from unexecuted content, ADR-0027
[documented]).
*Cross-reference corrected:* the machine-checkable residue is that a decision id is present and non-empty. **No
criterion in this area checked that until 2026-09-11** — AC-110 governs the extended file, not decision ids, and SM-036
tracks size and usage, not provenance; it was recorded as gap 9. **Residue, authored (2026-09-11, PIA-029):** GIVEN a
shipped activity file in the v0 catalog / WHEN lint and `doctor` run / THEN the file carries a non-empty `curation.decision`
id resolving to a record in the catalog's decision log, and an activity with no such id or a dangling one fails lint
naming the file. *Verified by:* a lint rule; a fixture activity with no decision id and one with a dangling id, both of
which must fail; a positive fixture resolving to a logged decision. The judgement stays the human's; only the record's
presence is machine-checked.

**AC-115** — `rk-catalog` quality blocks · evidence: F29; F31 · decisions: ADR-0028 — **HUMAN GATE**
GIVEN an activity that declares a quality block
WHEN the block is assessed for adequacy
THEN a named human records whether the block actually discriminates the failure it claims to catch.
*Routed to a human gate; not machine-decidable.* Reason: the corpus shows structural presence is not sufficiency — a
reproduced regression was approved 3 of 10 times by a review gate [measured, F29] and four adjudicated defects survived
fully green suites of 1,890/1,890 and 1,907/1,907 [measured, F31]. A boolean "has a quality block" (AC-112) is
checkable; "the block works" is not.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-025 | Whole-catalog lattice CI runtime | — |
| SM-026 | Catalog unsatisfiability rate | — |
| SM-027 | Dead activities | — |
| SM-028 | Configuration defects caught at configuration time vs at dispatch time | saturating; no baseline |
| SM-029 | Lint precision and silent guesses | — |
| SM-030 | First-run lint errors per activity file (authoring ergonomics) | — |
| SM-031 | Activity verbosity | — |
| SM-032 | Quality-block coverage | — |
| SM-033 | Binder ambiguity and unresolved-role rate | no baseline |
| SM-034 | Schema amendment rate per authoring pass | TRACKER, not a gate: no target line and no kill condition; no firing kill |
| SM-035 | Rank-monotonicity violations | — |
| SM-036 | Shipped catalog size and per-activity usage | no baseline |

#### Residual risk this area could not repair

- ADR and proposal citations are structurally unverifiable from the evidence brief, which carries spike findings only. Roughly half the criteria rest on 'proposal §4.2.1 doctor rule N' or an ADR number. A misquoted rule-table row would pass this refutation untouched — the single largest residual risk in the area.
- AC-094 (dead roles/skills/slots) has no evidence at all after the misapplied dead-*activity* figure was cut. It is machine-checkable and must not be presented downstream as evidence-derived.
- AC-112's denominator is authored, not derived: an activity that produces a delivered artifact and omits `output.delivered = true` is silently exempt. SM-032 cannot establish the true coverage until the subset is computed.
- AC-088 and AC-087 are in latent tension — toolchain-keyed kinds and specificity ranking implemented in one code path will break one of them. A note exists; no criterion enforces the separation.
- The shipped catalog's `bisect` rung is repaired by a ceiling (`max_rigor = "recommended"`, ADR-0093 decision 5), so a `mandatory`-rigor item with a repro gets a full activity and no bisection in Phase 1; adding adversary and policy steps to `bisect` is the Phase-2 repair. AC-101/AC-102's baselines are re-taken at Step 7e and the figures above are the pre-ceiling readings until then.
- AC-100's regression fixture ('every file individually clean, one require row unsatisfiable across files') is the whole criterion, and nothing here specifies its construction. Built weakly, the criterion becomes a tautology.
- SM-026, SM-031 and AC-101/AC-102 sit far inside their thresholds post-decision (0.3% vs 5%; median 53–73 vs 150). Three of twelve measures cannot fire against today's catalog.

---

## 5. Typed artifacts and the store — layout C, rendering, drift

**Area key:** `artifacts-store` · **Criteria:** AC-120..AC-159 (AC-157..AC-159 are splits) — **40 in force** · **Measures:** SM-037..SM-046 (SM-047, SM-048 unallocated)
**State:** 36 binding · 1 provisional (AC-154) · 3 human-gated (AC-139, AC-155, AC-156)
**Sources:** proposal §4.4 (with §4.5 single-write-path and §9 D5/D6 rev. 4), §4.6; S-MIG census report `spikes/results/s-mig-report.md`; ADR-0074 (supersedes ADR-0004), 0005, 0006, 0008, 0009, 0012, 0017, 0018, 0019, 0025, 0030, 0072.

*Scope note.* S-MIG denominators are stated at every point of use: 12% = 19/153 specs · 25% = 39/156 reports · 83% = 19/23 reviews · 110/110 `plan.md` · 19 of 45 companion frontmatter records on the five-kind row (five of eleven companion kinds carry one type — E-0075-2) · 142/142 lifted TDD tables without the mutation column (the "0/156" is 0/142 plus 14 unassessed — s-mig-report Correction §4). S-MIG's own instrument disclosure travels with them — the lifters were iterated four times against this corpus before the recorded run, over one repo at one frozen SHA (`47faec9f`), so these are shares under a locator tuned to the corpus, not a blind parse. `crates/` is empty and the repository has no CI workflow: every 'Verified by' line specifies an instrument the build must create.

### 5.1 Rendering direction — markdown is an output, never an input

**AC-120** — `rk-store` · evidence: S-MIG free-form gate results 39/156 reports and 19/23 reviews [measured, one corpus];
old redkiln `template_sig`/`rendered_sig` drift [documented] · decisions: ADR-0006, ADR-0005, ADR-0100 (`cargo deny` is the instrument; no YAML crate outside `redkiln-migrate`)
GIVEN the `rk-store` crate and its full dependency graph
WHEN the workspace architecture lint runs in CI
THEN no markdown-parsing library is reachable from any store code path other than the single section-locator module
AND `rk-store` exports no item that takes `&str` markdown and returns a typed artifact field.
*Verified by:* a module-boundary architecture test asserting the markdown crate appears only under `store::locator`; a
`cargo deny`/`cargo tree --invert` CI rule (the committed `deny.toml`, ADR-0100 decision 3 — the same instrument also asserts no YAML
crate is reachable from `rk-store`: `yaml_serde` lives in `redkiln-migrate` alone, decision 1); an exported-items test over the crate's
public surface.
*Cut from this criterion:* the previous "does not exist in the public **or crate-internal** API", verified by a single
`trybuild` fixture for a `parse_spec_md` shape. A compile-fail fixture proves one named function is absent; it cannot
witness a universal non-existence claim over crate-internal code. The dependency-reachability rule above is what actually
bounds the internal case.

**AC-121** — `rk-store` render pipeline · evidence: §4.4 digest-stamped structured sections [documented] · decisions:
ADR-0006
GIVEN a typed artifact value and a pinned template set
WHEN the same value is rendered twice, on two operating systems and two working directories
THEN the two renderings are byte-identical, line endings included
AND the section digest computed over each rendering is identical.
*Verified by:* a golden-file test run on both CI targets; a property test over arbitrary values of every artifact type
(`proptest` + `Arbitrary`) asserting `render(v) == render(v)`; a CRLF/LF fixture.
*Precondition:* this is only checkable if CI runs **two** OS targets. If the build ships a single target the cross-OS
half is unverified and must be marked so, not assumed.
*Cut from this criterion:* the citation "F31 schema conformance 52/52". F31 measures conformance of *harness output to a
schema*; it says nothing about whether a renderer is deterministic. The criterion stands on §4.4 as a design rule with a
real instrument, not on that figure.

**AC-122** — `rk-store` · evidence: §4.5 rule 4, the single-write-path rule — proposal §4.4 line 94 [documented]; the
crate-level compile-time boundary is this design's own rule, not the proposal's [inferred; retagged 2026-09-11,
PIA-043] · decisions: ADR-0006, ADR-0018
GIVEN any crate in the workspace other than `rk-store`
WHEN it attempts to write a byte to a path under a managed artifact directory
THEN the write does not compile — no filesystem-write handle to the artifact tree is exported outside `rk-store`.
*Verified by:* an architecture test on exported items (no `File`/`fs::write` re-export from `rk-store`); a `trybuild`
compile-fail case for a direct write from a sibling crate.
*Cut from this criterion:* the git-branchless issue #1334 citation. #1334 is real and is the cited ground for §4.5's
**no-hooks** rule; ADR-0018's own evidence note says it "is about hooks, not worktrees". It does not license a
compile-time export boundary, so it is not evidence here. Split: the runtime half is now AC-159.

**AC-123** — `rk-store` render pipeline · evidence: §4.4 "never parses markdown to recover structure"
[documented] · decisions: ADR-0006
GIVEN a rendered structured section whose typed source value has changed
WHEN the store re-renders the artifact
THEN the whole structured section is replaced from the typed value
AND the section's bytes are a function of the typed value and template version alone: rendering the same value over two
different prior contents produces identical section bytes.
*Verified by:* a unit test that renders over a section containing a sentinel string absent from the typed value and
asserts the sentinel is gone; a differential test rendering one value over two prior contents; a mutation test that makes
the renderer read prior section text and must fail the suite.
*Repaired:* the old AND read "no byte of the previous rendering is read, merged or preserved" — a statement about the
implementation's reads, which no test observes. The differential form above is the same guarantee, observable.

### 5.2 Drift — an edit inside a rendered section is typed drift, surfaced not applied

**AC-124** — `rk-store` reconcile · evidence: §4.4 "an edit inside one is typed drift, surfaced not applied"
[documented] · decisions: ADR-0005, ADR-0006
GIVEN a human has edited text inside a digest-stamped structured section of `spec.md`
WHEN `reconcile()` runs at the next entry point
THEN the store emits a typed drift record naming the artifact, the section anchor and both digests
AND the item's structured value is unchanged by the edit.
*Verified by:* an integration test that mutates a rendered section on disk and asserts (a) a `structured_drift` event
exists and (b) the typed value round-trips unchanged; a replay assertion that no write to the typed value is recorded.
*Repaired:* the F19 "12/174 = 7%" citation is removed from this criterion. F19 measures **world** drift under a human
gate (files moving while a gate is open), not humans editing rendered sections. It is the nearest measured neighbour, a
different quantity, and it is cited as such in SM-037 only. No measurement of this criterion's quantity exists.

**AC-125** — `rk-store` reconcile · evidence: §4.4 layout-B rejection ("the import path is a place structure can rot
back in") [documented] · decisions: ADR-0006
GIVEN a structured-section drift record exists
WHEN any subsequent store operation runs, including a re-render or an LLM activity
THEN a re-render produces bytes identical to rendering the unchanged typed value — no part of the edited text survives
into any typed field
AND resolving the drift requires a typed write through the CLI, web or a gate answer.
*Verified by:* a re-render equality test after a drift record exists; an integration test asserting the drift-resolution
API accepts only a typed value; an architecture test that no code path passes drift-record text into a deserializer.

**AC-126** — `rk-store` reconcile · evidence: F19 drift is RR 3.6 (CI [1.0, 12.9], Fisher p = 0.039 — directional,
marginal at n = 174) more likely on gates over 1 h (9/79 = 11% vs 3/95 = 3%) [measured, one operator, one repo] and 45–48% of gates exceed 1 h (F18) [measured]; brief §C3 "forces `reconcile()` on
every entry and forbids an optimisation that skips it" · decisions: ADR-0005, ADR-0017
GIVEN an artifact whose rendered sections were last stamped at tree hash T
WHEN the store is entered at any tree hash other than T
THEN `reconcile()` runs before any read of the artifact is served
AND there is no configuration, cache or fast path that skips it.
*Verified by:* a kernel test that removes the reconcile call and must fail; a replay assertion that every
`artifact.read` event is preceded by a reconcile record for the same entry; absence of a skip flag asserted by a CLI
surface test.

**AC-127** — `rk-store` · evidence: ADR-0025 human-supplied artifacts are vouched roots with advisory drift
[documented] · decisions: ADR-0025, ADR-0006
GIVEN a human-supplied (vouched-root) artifact whose content has changed since it was vouched
WHEN the store reconciles it
THEN the change is recorded as advisory drift and does not block the stage
AND the same change on a store-rendered structured section does block until resolved.
*Verified by:* a table-driven test over {vouched root, rendered section} × {changed, unchanged} asserting the four
outcomes; a `doctor` rule-table row distinguishing the two drift kinds.

**AC-128** — `rk-store` · evidence: §4.4 "a schema change is a migration, and the old redkiln's
`template_sig`/`rendered_sig` drift is the failure to avoid" [documented] · decisions: ADR-0006
GIVEN a template set at version V and artifacts stamped under version V
WHEN the templates are changed to V+1 without re-rendering
THEN every affected artifact is reported as template drift with old and new template versions
AND no artifact is silently re-stamped, and no template drift is reported as human drift.
*Verified by:* a test that bumps the template version and asserts the drift kind is `template`, not `human`; a `doctor`
rule row; a CI check that the template version is part of the section digest input.
*Cut from this criterion:* the ADR-0072 citation. ADR-0072 is "shipped **activity** edits bump version" — a catalog rule
about process activities, not about artifact templates or `schema_version`. Reusing it here reads as decision cover the
decision does not give.

### 5.3 Schema design rules

**AC-129** — `rk-types` schema export · evidence: §4.4 design rule "`additionalProperties: false`" [documented],
corroborated by Anthropic's structured-output schema limitations, which require `additionalProperties: false` on all
objects and reject any other value [documented, vendor doc] · decisions: ADR-0006
GIVEN the `schemars`-derived JSON Schema of any shipped artifact type
WHEN the schema lint runs in CI
THEN every object node in the schema carries `additionalProperties: false`
AND a type that omits it fails the build rather than shipping.
*Verified by:* a CI schema lint over the exported schema set; a fixture type with an open object that must fail.
*Repaired:* F20 is demoted from evidence to motivation. F20 is a *tool-allowlist* defect (`structured_output` absent at
exit 0) — a reason to distrust silent schema voids in general, not evidence about `additionalProperties`.

**AC-130** — `rk-types` schema export · evidence: §4.4 "every enum closed" [documented]; F36 — `unknown` is treated as
`high` by position, so an axis oscillating unknown↔low moves the band the full width (brief §C9) [measured] · decisions:
ADR-0006, ADR-0030
GIVEN any artifact-type field whose domain is a category
WHEN the schema lint runs
THEN the field is a closed enum with an explicit variant for the absent/unknown case
AND no artifact-type field is a bare `String` standing in for a category.
*Verified by:* a schema lint rule (enumerated-domain registry vs exported schema); a fixture with a free `String`
category that must fail.

**AC-131** — `rk-types` schema export · evidence: §4.4 "no `oneOf` nesting the model handles badly" [documented];
the vendor's structured-outputs page lists `anyOf`/`allOf` as supported and does not list `oneOf`; schemars 1.2.2 renders
`#[serde(tag = "kind")]` as `oneOf` when variants are unique [documented, platform.claude.com and docs.rs, fetched
2026-09-10; measured against the pinned schemars] · decisions: ADR-0006, **ADR-0095** (supersedes ADR-0084's schema clause)
GIVEN any schema `rk-types` exports across the closed-output boundary
WHEN the schema lint runs over the **exported** shape (after `rk-types`' post-derive `schemars::transform::Transform`,
which rewrites a `oneOf` whose branches are objects discriminated by `kind` into an equivalent `anyOf`)
THEN the schema contains **no `oneOf` at any node**
AND no union is nested inside a union — no `anyOf` (or type array) has a descendant `anyOf` or type array
AND `additionalProperties: false` is set at every object level, with `required` ordered first.
*Verified by:* a schema lint rule walking the exported schema tree inside the golden-snapshot test; a nested-union
fixture type that must fail; a tagged-enum fixture whose exported schema must contain `anyOf` and no `oneOf`; runbook
Checkpoint 5's `grep -rnE 'oneOf|anyOf'` over the snapshot directory expecting `oneOf` → 0 (the nesting check lives in
the test, not the grep). **One rule, stated identically here, in AC-157, in design §5.3 and in the runbook's Step 5.3**
(ADR-0095 decision 1).
*Split:* the union-typed-parameter count is AC-157. *(Amended 2026-09-11, PIA-005: the nested-only rule and ADR-0084's
"no `oneOf` anywhere" could not both hold with schemars' output; ADR-0095 fixes the exported-shape rule above.)*

**AC-132** — `rk-types` schema export · evidence: the vendor's "Schema complexity limits" table — **24 optional
parameters, per request across all strict tool schemas plus the JSON output schema** (with 16 union-typed parameters,
20 strict tools per request, a 180 s compile timeout) [documented, platform.claude.com/docs/en/build-with-claude/
structured-outputs, fetched 2026-09-10; also in a Wayback capture of 2026-09-07] · decisions: ADR-0006, **ADR-0095**
GIVEN any schema `rk-types` exports across the closed-output boundary, under the precondition that **one output schema
is sent per dispatch and no strict user-defined tool is on the adapter's command line** (so per-schema equals the vendor's
per-request scope; the adapter's command builder lints that precondition on its argv — ADR-0099, ADR-0108)
WHEN the schema lint runs
THEN the schema declares at most 24 optional parameters.
*Verified by:* a schema lint rule counting optional properties on the exported schema inside the golden-snapshot test; a
fixture type with 25 optionals that must fail; the argv lint on the adapter's command builder, with a fixture that adds a
strict tool and must re-open the budget explicitly.
**Correction (2026-09-11, audit PIA-005; ERRATA E-0084-2).** The earlier "house rule, not a vendor cap" and "the vendor's
published limitations contain no such cap" were a truncation artefact of the 2026-09-08 fetch: the limits table is on
the page. 24/16 are the vendor's limits with the per-request scope stated above, not this project's own budget, and the
earlier sentence forbidding their citation as a platform constraint is withdrawn wherever it appears. The failure on breach is silent
at exit 0 (F20); **S-SCHEMA-CAP** (ADR-0095 decision 3; consumer: this test, AC-157 and AC-129's fail-open guard) submits a
25-optional and a 17-union-parameter schema to the pinned CLI to record the observed failure shape.
*Split:* property ordering is now AC-158.

**AC-133** — `rk-types` · evidence: §4.4 "a free-text `rationale` beside every enum that carries a judgement"
[documented]; F29 19/19 rejections on-target while class means wobble (Reproduced 0.70 > MutationSurvived 0.37 >
NarrowedPin 0.10) [measured] · decisions: ADR-0006, ADR-0074 (supersedes ADR-0004)
GIVEN any enum on a shipped artifact type
WHEN the schema lint runs
THEN the enum appears in the judgement-enum registry with an explicit judgement-bearing yes/no
AND every enum marked yes has a sibling free-text `rationale` field on the same object.
*Verified by:* a registry-completeness test (every shipped enum is registered, or the lint fails); a schema lint rule for
the rationale sibling; a fixture judgement enum with no rationale sibling that must fail.
*Repaired:* as written before, the criterion fired only on enums someone had already registered — a system with an empty
registry passed it. The completeness half above closes that. **Limit:** the lint still cannot tell whether the *value* of
the yes/no is right; that classification is the human gate AC-156.

**AC-134** — `rk-types` deserialization · evidence: §4.4 "enum values compared case-insensitively" [documented]; the
brief carries **no** casing measurement — F31's 52/52 conformance was not stratified by casing [inferred] · decisions:
ADR-0006
GIVEN a harness response whose enum value differs from the declared variant only in letter case
WHEN the kernel validates the value at the boundary
THEN the value is accepted and normalised to the declared variant
AND the artifact stores the normalised form, so a rendered artifact never shows two casings of one variant.
*Verified by:* a unit test over every shipped enum with upper, lower and mixed casing; a store test asserting the
persisted form equals the declared variant.
*Repaired:* the old evidence line read "F31 conformance 52/52 measured under one casing only [measured]". Nothing in the
brief or F31 states the casing condition; that was an assumption wearing a `[measured]` tag.

**AC-135** — `rk-types` · evidence: §4.4 "a `schema_version` per type" and "a schema change is a migration"
[documented] · decisions: ADR-0006
GIVEN a shipped artifact type whose field set or field semantics change
WHEN CI runs
THEN the change fails unless the type's `schema_version` is incremented in the same commit
AND the schema snapshot for the previous version remains in the repository.
*Verified by:* a schema-snapshot test (committed schema fixtures diffed against the export) that fails on an
unversioned change; a CI rule requiring an added snapshot file for each bump.
*Note:* "field semantics change" is not detectable by a snapshot diff — the instrument catches field-set changes only. A
semantics-only change that leaves the schema byte-identical passes this criterion. Say so in the runbook rather than
claiming coverage.
*Cut:* the ADR-0072 citation, for the reason given at AC-128.

**AC-136** — `rk-store` read path · evidence: §4.4 "a schema change is a migration"; old redkiln sig drift
[documented] · decisions: ADR-0006
GIVEN a stored artifact whose `schema_version` is older than the current type's
WHEN the store reads it
THEN the read fails with a named missing-migration error identifying type and both versions, unless a registered
migration for that exact version pair exists
AND the store never deserializes an unmigrated artifact on a best-effort basis.
*Verified by:* a unit test per version pair over the migration registry; a fixture artifact at an unregistered version
that must produce the error; a `doctor` rule row listing unmigratable artifacts in the repository.

**AC-157** — `rk-types` schema export · evidence: the vendor's "Schema complexity limits" table — **16 parameters with
union types (`anyOf` or type arrays), per request** [documented, platform.claude.com structured-outputs page, fetched
2026-09-10] · decisions: ADR-0006, **ADR-0095** *(split from AC-131)*
GIVEN any schema `rk-types` exports across the closed-output boundary, under AC-132's precondition (one output schema per
dispatch, no strict user-defined tool on the argv)
WHEN the schema lint runs
THEN the number of **union-typed parameters** — any property whose schema uses `anyOf` or a type array, a nullable
`Option<T>` included — is ≤ 16, counted per exported schema.
*Verified by:* a schema lint rule counting union-typed properties on the exported schema inside the golden-snapshot test;
the retained 17-parameter fixture type (17 union-typed properties) that must fail; a fixture with 17 `Option<T>` fields
and no explicit union that must also fail.
**Correction (2026-09-11, audit PIA-005).** "Union branches at any node" measured a different quantity from the vendor's:
a nullable `Option<T>` is one union-typed parameter under the vendor's count and zero under a branch count, so a type
with 24 `Option` fields could breach 16 with every lint green and fail silently (F20). Counting branches per node is
forbidden as this criterion's quantity (ADR-0095). The count is per exported schema, which is stricter than per request
only while AC-132's precondition holds.

**AC-158** — `rk-types` schema export · evidence: §4.4 "required-first" [documented] · decisions: ADR-0006 *(split from
AC-132)*
GIVEN any shipped artifact type's schema
WHEN the schema lint runs
THEN all required properties precede all optional properties in the emitted property order.
*Verified by:* a schema lint rule on the exported schema's property order; a fixture type with an optional-before-required
order that must fail.
*Precondition:* checkable only if the export preserves insertion order. **Settled (2026-09-11, PIA-031):** runbook Step
5.3 specifies `schemars` and `serde_json` with `features = ["preserve_order"]` and the snapshot test asserts property
order, so the order is observable and this criterion is verifiable; the "unverifiable" clause is withdrawn here and in
this area's residual-risk list.

### 5.4 The AC type, verification and the blocking policy

**AC-137** — `rk-types::AcceptanceCriterion` · evidence: S-MIG — **19/153 shipped specs (12%)** write criteria without
GIVEN/WHEN/THEN or without a verification note — **76 loss lines for a missing GWT and 87 for a missing verification
(75/85 distinct ids; 131 ACs with a loss; 29 lacking both; of 208 ACs in those 19 specs)** [measured, uncapped re-run of
`census-s-mig.py` at the pin; the earlier 45 and 46 were cap-truncated at `loss[:6]` — ERRATA E-0075-1] — all in the fix
lane and the late archive [measured, one corpus] · decisions: ADR-0006, ADR-0005, ADR-0075 (E-0075-2)
GIVEN a criterion with a `criterion` text, no `gwt` and no `verification`
WHEN the kernel validates it at the boundary
THEN the value is accepted and stored
AND the rendered `spec.md` shows the criterion with its GWT and verification slots visibly empty, not fabricated
AND `gwt` (`Gwt { given: Text, when: Text, then: Text, and: Vec<Text> }` — decision #21 of 2026-09-11) and
`verification` are **authored fields from the first CLI write** — migration never populates either from criterion
text (ADR-0075 amendment note as corrected by E-0075-2; AC-454 is the migration-side twin).
*Verified by:* a round-trip unit test on the minimal AC value; a golden render fixture for the empty-slot case; a
corpus test replaying the S-MIG specs that previously failed a strict grammar; a migration test over all 153 specs at
the pin asserting no `gwt` or `verification` is ever populated.

**AC-138** — `rk-policy` · evidence: §4.4 "the type accepts them while the policy decides what a missing verification
blocks" [documented]; F31 policy relaxation was free — lenient 9/23 blocked, verified false positives 0/23, detection
unchanged [measured] · decisions: ADR-0006, ADR-0074 (supersedes ADR-0004)
GIVEN a stage whose policy table row says a missing `verification` blocks
WHEN an item reaches that stage carrying an AC with no verification
THEN the stage is blocked with a finding naming that AC id
AND changing the same policy row to advisory changes the outcome to a non-blocking advisory with no code change.
*Verified by:* a table-driven policy test over {blocks, advises} × {verification present, absent}; a replay assertion
that the block decision is computed from the policy table row, not from a constant in the type.
*Note on the borrowed figure:* F31's "free" relaxation is measured against CLEAN, whose ground truth the brief records
as wrong for ≥13 of 23 (57%) — every false-positive rate against it is an **upper bound**. The relaxation being free is
still the brief's own §C8 conclusion; it just is not a point estimate.

**AC-139** — `rk-policy` default for a missing verification · evidence: F37 — P7's primary criterion is powerless at n=3,
every Wilson interval overlapping by construction [measured]; F31's severity call is Ryan's, not measured
[documented] · decisions: ADR-0074 (supersedes ADR-0004), ADR-0019
GIVEN no spike has measured the cost or recall of blocking on a missing verification
WHEN the default policy row is chosen for Phase 1
THEN the choice is recorded as a human decision with its rationale, and the shipped default is whatever that decision
names.
*Verified by:* **routed to a human gate; not machine-decidable** — no instrument discriminates the two defaults, and F37
shows the nearest spike is powerless at its n. What IS machine-checked is AC-138: the value is a policy-table row, so the
human decision is a data change.

**AC-140** — `rk-store` · evidence: F25 — the AC-015 probe inverted the ordering and the suite stayed **36/36 GREEN**,
so "reviewed and green" is not evidence of pinning [measured] · decisions: ADR-0019, ADR-0074 (supersedes ADR-0004)
GIVEN an AC that has no per-AC discriminating test recorded against it
WHEN the artifact renders its criteria section
THEN the AC is rendered as unpinned, with the absence stated in the rendering
AND the absence is never rendered as, or defaulted to, a pass.
*Verified by:* a golden render fixture for an AC with an empty evidence set; a test asserting the rendered token for
"no discriminating test" is distinct from every pass token; a `doctor` rule row counting unpinned ACs per item.

### 5.5 Gate results — emitted, rendered, never parsed

**AC-141** — `rk-store` · evidence: S-MIG — gate results free-form in **39/156 implementation reports (25%)** and
**19/23 reviews (83%)** [measured, one corpus; note the 23-file denominator on the second figure] · decisions: ADR-0006,
ADR-0074 (supersedes ADR-0004)
GIVEN a gate result produced by the toolchain as a typed record
WHEN the artifact is rendered
THEN the gate section is rendered from that typed record
AND no code path derives a gate outcome from any markdown or prose field of any artifact.
*Verified by:* an architecture test that the gate-outcome constructor is reachable only from the toolchain adapter; a
fixture report whose prose says "all gates passed" while the typed record is absent, asserting the render shows no
passing gate.

**AC-142** — `rk-store` · evidence: F20 — an agent card's `tools` allowlist silently voids `--json-schema`:
`structured_output` absent, exit 0, `subtype: success` [measured]; proposal §4.6 "an absent digest is `Err`"
[documented] · decisions: ADR-0006, ADR-0074 (supersedes ADR-0004)
GIVEN an activity that should have emitted a gate result but emitted none
WHEN the store persists that activity's output
THEN the absence is a typed error recorded against the activity
AND the artifact renders no gate outcome for it — absent is never rendered as pass, fail or unknown-treated-as-pass.
*Verified by:* a unit test on the empty-output case; a replay assertion over a fixture run with a stripped structured
output; a `doctor` rule row for activities with a missing expected emission.

### 5.6 Sidecars, companions and `plan.md`

**AC-143** — `rk-store` · evidence: §4.4 "each companion is a sidecar with its own schema" [documented]; S-MIG —
companion frontmatter lifted as one type on **five of eleven companion kinds** (the earlier "19 of 45 (42%)" figure was over a
five-kind subset — ERRATA E-0075-2) and is not one type [measured, `s-mig.jsonl` at 47faec9f: closure 30 · decomposition
30 · design 26 · design-review 22 · grounding 26 · implementation 4 · integration 26 · plan 4 · retrospective 4 · slices
26 · storymap 30, plus `ledger` 220] · decisions: ADR-0006, ADR-0075 cl. 3 (the kind list — `spikes/DECISIONS-2026-09-11.md`
#3)
GIVEN a Phase-1 companion artifact kind — exactly **six**: this criterion's five (`_integration`, `_grounding`,
`_decomposition`, `_review`, `_ledger`) plus the census's `ledger` kind (220 records at the pin), per decision #3 of
2026-09-11 ("AC-143's five plus `ledger`")
WHEN the store writes it
THEN it is written as JSON against that companion's own registered schema
AND a companion kind with no registered schema cannot be written at all
AND the other eight census kinds (`closure`, `design`, `design-review`, `slices`, `storymap`, `implementation`, `plan`,
`retrospective`) are Phase-2 kinds that `redkiln-migrate` lifts as opaque prose per ADR-0075 cl. 6, and a write of one of
them against the Phase-1 registry is refused, never accepted as untyped.
*Verified by:* a registry-completeness test enumerating **exactly six** companion kinds against registered schemas
(five plus `ledger`; AC-333 needs `ledger`); a fixture companion with no schema whose write must fail; a fixture for
each of the eight Phase-2 kinds asserting refusal; a schema lint pass over each companion schema (AC-129..AC-133,
AC-157, AC-158 apply).
*Note (amended 2026-09-11, PIA-006):* the three-source disagreement (ADR-0075 cl. 3's four kinds, this criterion's
five, the S-MIG census's eleven plus `ledger`) is settled by the operator's decision #3 of 2026-09-11, recorded against
ADR-0075 cl. 3 in the runbook's Step 5.3 (s5 §5.3). Until that decision lands in the runbook's acceptance lines for
Steps 5 and 6 this criterion is **unverifiable until then** and must not be counted as covered there; the residual-risk
list carries it. Adding a kind later is a schema, not a supersession.

**AC-144** — `rk-store` · evidence: §4.4 machine-only artifacts are JSON sidecars rendered to `.md` on demand
[documented] · decisions: ADR-0006
GIVEN a machine-only artifact whose `.md` rendering exists on disk
WHEN that artifact is read by any activity or command
THEN the JSON sidecar is the source read
AND deleting the `.md` rendering changes no behaviour other than the absence of the file.
*Verified by:* an integration test that deletes every rendered `.md` for machine-only artifacts and asserts the suite
still passes; an architecture test that no reader opens a machine-only `.md`.

**AC-145** — `rk-store` · evidence: S-MIG — **110/110 `plan.md` files were the unfilled template**, the empty `1.`
scaffold with template prose; planning happened in `_plan.md`, `_storymap.md` and `spec.md` [measured, one corpus] ·
decisions: ADR-0006
GIVEN any process pack, activity or command in the shipped configuration
WHEN the catalog lint and `doctor` run
THEN no producer declares `plan.md` as an output and no template renders it
AND a `plan.md` found in a managed item directory is reported as an unmanaged file, never read.
*Verified by:* a catalog lint rule over declared artifact kinds; a `doctor` rule-table row; a fixture item directory
containing a `plan.md` that must be reported and must not be parsed.
*Note:* §4.4's layout-C paragraph still lists `plan.md` among the human-facing artifacts while its rev. 4 text removes
it. This criterion follows rev. 4; the stale sentence should be struck in the proposal, not worked around here.

### 5.7 `artifact.read` and unread stages

**AC-146** — `rk-kernel` · evidence: §4.4 "every activity input emits an `artifact.read` event" [documented]
· decisions: ADR-0006
GIVEN an activity that consumes an artifact as an input
WHEN the activity runs
THEN an `artifact.read` event is emitted for that input, carrying artifact kind, item id, `schema_version` and the tree
hash the read was served at
AND an input consumed without such an event is a kernel error, not a silent read.
*Verified by:* a replay assertion over a fixture run comparing declared inputs to emitted `artifact.read` events; a
kernel unit test on the read path; the event schema's own conformance test.
*Repaired:* "exactly one event per input" → "an event". §4.4 licenses emission, not uniqueness; an activity that reads an
input twice would fail a uniqueness assertion for no stated reason. S-MIG's 110/110 is demoted to motivation — it is the
pathology this event class makes visible, not evidence that the event carries these four fields.

**AC-147** — `redkiln doctor` · evidence: §4.4 "a stage whose artifact is never read is flagged by `doctor`"
[documented] · decisions: ADR-0006
GIVEN an item whose stage produced an artifact for which no `artifact.read` event exists in the whole item history
WHEN `doctor` runs against that repository
THEN that stage and artifact are listed as produced-but-never-read.
*Verified by:* a `doctor` rule-table row; a fixture event log containing one produced-unread artifact, asserted to be
listed, and a second fixture with a matching read event, asserted not to be listed.
*Cut from this criterion:* "AND the check costs zero model dispatches", verified by "a cost assertion that the rule
issues no harness dispatch". `doctor` is a local rule engine over the event log; no implementation of this rule could
dispatch a model. The clause passes on any system, including one that does nothing, and it made the criterion look
better tested than it is.

**AC-148** — `redkiln doctor` · evidence: **F36 — the selection engine is BLOCKED**: modal `R` identical on only 5/20,
3/20, 2/20, 2/20 draws against an 80% line (the kill fires 4/4), alpha drift **−0.168 on byte-identical anchors**, and
the brief's own headline states "the prescribed remedy does not repair it" [measured; arm D precision/recall is now
computed — F39 scored it at pin `47faec9f` and no facet clears 0.95, which does not lift F36's block] · decisions:
ADR-0006, ADR-0012
GIVEN `doctor` has flagged an artifact as produced-but-never-read
WHEN the next run selects the required artifact set for a comparable item
THEN the flagged artifact is still produced if the pack requires it — the flag never removes an artifact from a
required set automatically
AND removing it requires an edit to the pack, recorded as such.
*Verified by:* an integration test asserting the required set is unchanged after a produced-unread flag; an
architecture test that the doctor rule has no write path into pack or catalog data.
**PROVISIONAL is NOT claimed for this criterion** — it is written precisely because the selection engine is blocked;
any criterion that instead assumed a stable LLM-emitted context vector could prune the set would be unwritable today.
*Repaired:* the previous evidence line named "the pre-registered **anchor-rewrite** repair". The brief names the remedy
only as "the prescribed remedy", with the anchor detail attaching to the alpha-drift measurement. The specific repair's
name is not in the brief and has been removed.

**AC-159** — `redkiln doctor` · evidence: §4.5 rule 4 single-write-path [documented]; §4.5 rule 2, rows are keyed by
SHA/tree hash, never path [documented] · decisions: ADR-0006, ADR-0018 *(split from AC-122)*
GIVEN a managed artifact whose **structured-section or frontmatter digest** differs from the store's last recorded
rendering (a prose-section edit accepted by `reconcile()` under AC-152 is not a mutation; the reconcile that accepted it
counts as the write record — ADR-0105 clause 9)
WHEN `doctor` runs and no store write record accounts for the difference
THEN the file is reported as an unrecorded artifact mutation, naming artifact, both digests and the last recorded write.
*Verified by:* a `doctor` rule-table row; a fixture repository in which one managed file is edited out-of-band, asserted
to be reported; a control fixture whose file is rewritten *through* the store, asserted not to be reported.
*Repaired:* the original instrument was "any managed artifact file whose **mtime** advanced without a corresponding
store write record". mtime is not an instrument here — a clone, checkout or format-on-save advances it with identical
content (false positive), and a mtime-preserving write hides a real mutation (false negative). It also contradicts §4.5
rule 2, which keys everything on content/tree hash. Digest comparison is the design's own mechanism.
*Amended 2026-09-11 (PIA-043):* the GIVEN was "on-disk content digest" — a whole-file hash false-positives on every
prose edit (AC-152) while a structured-only hash added nothing over AC-124; the structured-section/frontmatter scope
and the reconcile-as-write-record clause are ADR-0105 clause 9's. Evidence retag: the single-write-path rule is proposal
§4.4 line 94 [documented]; the crate-level boundary is the design's own rule (AC-122 carries the same retag).

### 5.8 The section locator — a locator, not a grammar

**AC-149** — `rk-store::locator` · evidence: §4.4 "a heading-anchored slice, not a grammar, and it is tested as such"
[documented] · decisions: ADR-0005, ADR-0006
GIVEN a `spec.md` and a designated opaque prose section anchor — **an anchor is a heading at the template's declared
level for that section; the slice runs from the line after the anchor to the next heading at that level or shallower,
or to end of file; deeper headings inside the slice are prose bytes and not anchors** (the end rule, added 2026-09-11,
PIA-033; design §5.1 row 4 carries the same sentence)
WHEN the locator resolves that anchor
THEN the section's content reaches the rest of the store only as an uninterpreted byte slice
AND no field of any typed artifact is populated from anything derived from that content.
*Verified by:* a signature test that the locator's return carries a byte range and no content-derived field; an
architecture test that no caller inspects the sliced bytes beyond passing them to the opaque prose field; a differential
test that changing the markup *inside* a section changes no typed field; a byte-range fixture per end-rule case (next
heading at the same level, next shallower heading, end of file, a deeper heading inside the slice).
*Repaired:* the old THEN named a return type (`Range<usize>`, "no parsed nodes"). The type is how it is done; the above
is what is observable.

**AC-150** — `rk-store::locator` · evidence: §4.4 layout-C trade-off "requires a section grammar, but only for
locating" [documented] · decisions: ADR-0005
GIVEN a document in which the target heading text appears inside a fenced code block, an indented block, an HTML
comment, a blockquote, or **as a deeper heading inside another section's slice**, as well as once as a real heading at
the template's declared level
WHEN the locator resolves the anchor
THEN the returned range is the real heading's section
AND the decoy occurrences are not selected.
*Verified by:* an adversarial fixture corpus (one file per decoy form, the deeper-heading decoy included — PIA-033) with
expected byte ranges; a property test that inserting a decoy occurrence into any fixture does not change the resolved
range.

**AC-151** — `rk-store::locator` · evidence: §4.4 "nothing parses structure out of prose" [documented]; F33 — the lint
"accepted 5 guesses silently" as the failure mode to avoid [measured] · decisions: ADR-0005, ADR-0006
GIVEN a document in which the target anchor occurs zero times, or more than once as a genuine heading at the
template's declared level
WHEN the locator resolves it
THEN it returns a named error identifying the anchor and the occurrence count
AND it never returns a first-match, nearest-match or fuzzy-match range
AND a same-level duplicate of the anchor text that sits **inside another section's prose** (deeper than that section's
level) is prose, not a second occurrence — AC-149's end rule.
*Verified by:* unit tests for the zero-match and duplicate-match cases; a same-level-duplicate-in-prose fixture that must
resolve (PIA-033); a mutation test that changes the error path to first-match and must fail the suite.

**AC-152** — `rk-store` · evidence: §4.4 "opaque prose fields"; S-MIG — everything designated opaque-by-design (3,100+
sections) lifted as opaque with no exceptions [measured, and stated by S-MIG itself to be the trivial half of that
result] · decisions: ADR-0005, ADR-0006
GIVEN an opaque prose section whose content a human has rewritten entirely
WHEN `reconcile()` runs
THEN the edit is accepted as the field's new value with no drift record and no gate
AND no store code path inspects, validates, lints or summarises the prose content.
*Verified by:* an integration test writing arbitrary bytes (including markdown headings) into a prose section and
asserting acceptance; an architecture test that the prose field is stored as an opaque string with no content
predicate.

**AC-153** — `rk-store` · evidence: §4.4 structured fields "change only through CLI, web or gate answers" [documented];
ADR-0009 the CLI is the only gate channel in Phase 1 [documented] · decisions: ADR-0005, ADR-0006, ADR-0009
GIVEN a human has edited a structured field in the typed frontmatter of `spec.md` in an editor or a PR
WHEN `reconcile()` runs
THEN the edit is surfaced as typed drift and the field's stored value is unchanged
AND the only accepted mutation channels for that field are the CLI, the web view and a gate answer — in Phase 1, the
CLI alone.
*Verified by:* an integration test per structured frontmatter field; a CLI surface test enumerating the mutation
commands; a `doctor` rule row for frontmatter drift.

### 5.9 Binary and non-text artifacts

**AC-154** — `rk-types::ChangeSummary` · **PROVISIONAL — rests on S-BIN, registered and unrun; S-BIN gates Phase 1**
· evidence: brief §E "Unrun: S-BIN (gates Phase 1 for `ChangeSummary` type design)" [documented]; ADR-0008 — a real
Unreal project is the S-BIN fixture, and ADR-0008 itself records that S-BIN "had not been run", so the type design it
authorises is provisional [documented] · decisions: ADR-0008, ADR-0006
GIVEN a change set containing binary assets that no diff renders
WHEN the store renders a `ChangeSummary` into a human-facing artifact
THEN the binary entries are rendered from typed fields (path, kind, size delta, toolchain-emitted facts)
AND no binary content is embedded, parsed or diffed as text.
*Verified by:* a golden render fixture over a `ChangeSummary` containing binary entries. **Blocked:** the field set is
not decidable until S-BIN runs against the ADR-0008 fixture, so the fixture this criterion names cannot be written yet.
This criterion pins the rendering direction only; the type's shape must not be frozen before S-BIN, and the criterion is
**unverifiable until then** — it must not be counted as covered in the runbook.

### 5.10 Human-judged, written as gates

**AC-155** — artifact template set · evidence: §4.4 layout C "matches how Hugo/Obsidian/Backlog.md treat frontmatter +
body" [documented] · decisions: ADR-0005, ADR-0006
GIVEN the rendered `spec.md` and `story.md` for a real item
WHEN a human reads them in an editor and in a GitHub diff
THEN the boundary between read-only structured sections and editable prose is unambiguous to that reader without
consulting documentation.
*Verified by:* **routed to a human gate; not machine-decidable** — legibility of the read-only boundary is a perceptual
judgement, and every mechanical proxy (a marker string, a digest comment) is satisfied by an illegible rendering. The
machine-checkable half is AC-124: an edit inside the boundary is caught whether or not it was legible.

**AC-156** — artifact type catalog · evidence: §4.4 designated prose fields [documented]; F32/F33/F34 authoring spread
(median 56.5 / 73 / 53 non-blank lines; the second author +29% with 27 ambiguities, 20 new) [measured, n=3 authors, one
author per point] · decisions: ADR-0005, ADR-0006
GIVEN a proposed new artifact type or a new field on a shipped one
WHEN the field is classified as structured or as opaque prose
THEN the classification is recorded with its rationale before the type ships.
*Verified by:* **routed to a human gate; not machine-decidable** — whether a field carries decision-bearing structure is
a design judgement, and F33 shows a second author diverges from the first by 29% with 20 new ambiguities. The
machine-checkable consequence is that every field carries one of the two classifications, which AC-129..AC-133's lint
enforces on the structured side and AC-133's registry-completeness test enforces for enums.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-037 | Structured-section drift rate | no baseline |
| SM-038 | Produced-but-never-read artifacts | — |
| SM-039 | Schema conformance of harness output | saturating; no firing kill |
| SM-040 | Cost of the typed contract | — |
| SM-041 | Schema and template authoring size | — |
| SM-042 | First-run lint error rate on authored schemas | — |
| SM-043 | Section-locator resolution failures | no baseline |
| SM-044 | AC completeness of shipped specs | — |
| SM-045 | Free-form gate outcomes reaching an artifact | — |
| SM-046 | Unmigratable stored artifacts | no baseline |

#### Residual risk this area could not repair

- Every instrument in this area is prospective. The surviving count is optimistic unless the build runbook carries each lint, architecture test, mutation test and doctor rule by name.
- Two criteria are unverifiable today and must not be counted as covered: AC-154 (its golden fixture cannot be written until S-BIN runs — and S-BIN's gate is scoped to the `ChangeSummary` type, ADR-0093 decision 4), and **AC-143 — unverifiable until the companion kind list (decision #3 of 2026-09-11: five kinds plus `ledger`) lands in the runbook's Step 5 and Step 6 acceptance lines** (PIA-006). AC-121's cross-OS half and AC-158 were on this list until 2026-09-11: the runbook's Step 5.2 names the `[ubuntu-latest, windows-latest]` matrix and Step 5.3 specifies `preserve_order`, so both are verifiable (PIA-031).
- AC-133, AC-143 and AC-130 depend on registries whose CONTENTS are human decisions. The completeness tests check that entries exist, never that their values are right — a wrong classification passes every machine check here.
- The S-MIG rates are one repo at one frozen SHA under a tuned locator. Anything downstream treating 12% or 83% as a rate for the new system is overreading them.
- AC-135's schema-snapshot instrument cannot detect a field-*semantics* change that leaves the schema byte-identical — precisely the old-redkiln failure mode ADR-0006 exists to prevent. Flagged, not repaired: no machine instrument distinguishes it.
- AC-143's companion-kind list was unreconciled until 2026-09-11 (three sources: four, five, eleven-plus-`ledger` kinds); decision #3 fixes six for Phase 1 and eight Phase-2 kinds lifted opaque. The residue is that the six registered schemas are human-authored contents the completeness test cannot judge.
- AC-132/AC-157's caps (24 optional parameters, 16 union-typed parameters) are the vendor's per-request Schema complexity limits [documented, fetched 2026-09-10 — ERRATA E-0084-2], not house rules; what remains unwarranted is the failure shape on breach, which F20 says is silent, until S-SCHEMA-CAP (ADR-0095) observes it.
- AC-131's "no `oneOf`" rests on a post-derive transform in the export path that can drift from schemars' output between versions; the golden snapshot is the only thing that catches it (ADR-0095's stated negative consequence).

---

## 6. Git as truth — trailers, reconcile(), worktrees, disposable clones, merge queue

**Area key:** `git-reconcile` · **Criteria:** AC-160..AC-199 **plus AC-529** (AC-185 and AC-199 are reused ids; AC-529 is allocated outside the range — see §0.4) — **41 in force** · **Measures:** SM-049..SM-058 (SM-059, SM-060 unallocated)
**State:** 39 binding (AC-529 added 2026-09-11) · 1 provisional (AC-187) · 1 human-gated (AC-171)
**Sources:** proposal §4.5 (durable state), §4.7 (cost and parallelism); ADR-0003, ADR-0013, ADR-0017 (boundary only), ADR-0018.

*Scope note.* A second evidence tag is in force here: **[motivating, not evidential]** marks a finding that explains why the design is shaped this way but does not test the criterion. A finding may be tagged `[measured]` against a criterion only when the measurement is about the same quantity the criterion constrains.

### 6.1 The tier-1 test — what may not live only in the database

**AC-160** — `rk-types` · evidence: §4.5 tier split, ADR-0018 [documented]; nothing in the programme measured a tier
annotation, so no F-number supports this · decisions: **ADR-0109** (the tier split), ADR-0086 (the memo table is built here, tier 2) (ADR-0018 related)
GIVEN any field of a state type in `rk-types` (applying this needs no judgement about which fields "participate in
selection" — it applies to every field of those types)
WHEN the crate is compiled
THEN a field carrying no durability-tier annotation (`tier1` written to git, `tier2` EventStore-only) is a compile
error, not a silent default.
*Verified by:* a `trybuild` compile-fail fixture holding an unannotated field, which must fail to compile with the
tier-missing diagnostic; the fixture is the instrument and it starts failing the moment the derive defaults.
*Limit:* **which** tier a field belongs in is a design judgement no instrument here settles. This criterion forces the
choice to be explicit and reviewable; SM-056 is what catches a wrong choice. *Amended 2026-09-11 (PIA-008, ADR-0097):*
the step memo is **tier 2** — rebuildable by re-execution, never by rebuild — and the fixture for this criterion drops
the memo from its rebuild assertion; design §5.4's tier table carries the row.

**AC-161** — `rk-store` + `rk-git` · evidence: F38 (0 changed, 0 lost, digests identical across a real `taskkill /T /F`)
[measured, n=3, PARTIAL] · decisions: **ADR-0109**, ADR-0097, ADR-0013 (ADR-0018 related)
GIVEN a repository whose EventStore database file has been deleted
WHEN `redkiln run <item>` is invoked
THEN the **stage-level next action** — the FSM's next stage transition and the gate records the trailers carry — is
equal, under the canonical serialiser, to the one selected before the deletion
AND the run proceeds with no manual repair step; step outputs and memo hits are **recomputed, not restored** (the memo is
tier 2 — ADR-0097 decision 1).
*Verified by:* the delete-the-database replay test in `rk-kernel` over the pinned corpus repo (ADR-0013), comparing the
canonical serialisation of the next stage transition and its gate records before and after deletion; any inequality
fails the test. The comparison is at stage grain; a step-grain comparison would assert what tier 2 cannot promise.
**Correction (2026-09-11, audit PIA-008).** As authored this compared the selected `ActivitySpec` and its resolved
inputs — a step-grain promise the SQLite memo tier cannot keep once the file is gone. ADR-0097 decision 1 narrows AC-161
and AC-164 to the stage-level next action and scopes AC-187 to a run whose EventStore survives.
*Inherited limit:* F19's 174/174 is losslessness of **suspension**, not of database deletion, and F19 explicitly never
replayed the FSM — so the evidence does not yet license "same next action". AC-188 is the criterion that would
establish it, and this one is only as strong as that.

**AC-162** — `rk-git` · evidence: §4.5 rule 4 (single write path) [documented]; F12 (360 trailer commits, 74 slices —
the corpus this is exercised against) [measured] · decisions: **ADR-0109**, ADR-0105, ADR-0003 (ADR-0018 related)
GIVEN a tier-1 transition (stage change, gate opened/answered, verdict, slice seal)
WHEN the transition is applied
THEN the trailer commit exists in git before any store row for that transition is readable
AND if the git write fails the store contains no row for that transition
AND the commit runs with `-c core.hooksPath=<empty directory>`, so the target repository's hooks never run on a Redkiln
commit (ADR-0105 decision 2).
*Verified by:* a fault-injection test with a git backend that fails the commit, asserting zero store rows afterwards;
a second fault-injection fixture installing a hook that rewrites the commit message, which must have no effect on the
trailer commit (ADR-0105); plus a reachability lint that the tier-1 store writer is callable only from the commit path
(the lint is a defence-in-depth check on structure, not the criterion — the fault-injection assertions are).

**AC-163** — `rk-git` · evidence: §4.5 tier-1 list [documented] · decisions: **ADR-0109**, ADR-0105 (ADR-0018 related)
GIVEN a tier-1 transition with no file changes to check point
WHEN the transition is applied
THEN an empty commit carrying the trailer is created (`git commit --allow-empty --trailer "<Key>: <value>"` through the
wrapper, with `-c trailer.ifexists=add` so a repeated identical trailer is appended, never dropped — ADR-0105 decision 3)
AND the transition is never recorded in the store alone.
*Verified by:* unit test asserting `HEAD` advanced and the trailer parses; a corpus replay asserting one commit per
tier-1 transition, which fails on a store row with no matching commit.

**AC-164** — `rk-store` · evidence: §4.5 tier-2 list [documented]; F6 (`utilization` quantised to 0.01, so per-run cost
is not measurable) [measured — motivating: it is why cost belongs in tier 2] · decisions: **ADR-0109**, ADR-0097,
ADR-0003 (ADR-0018 related)
GIVEN the tier-2 tables (usage, cost, headroom, wall clock, drift observations, inner retries, **the step memo** —
ADR-0097) are truncated
WHEN `redkiln run <item>` is invoked
THEN the **stage-level next action** is unchanged under the canonical serialiser (step outputs and memo hits are
recomputed — AC-161's narrowing, ADR-0097 decision 1)
AND `doctor` classifies the loss as telemetry-only, never as corruption.
*Verified by:* the AC-161 replay test parameterised on tier-2 truncation; an assertion on `doctor`'s emitted
classification code (a `corruption` classification for a tier-2 truncation fails the test).

**AC-165** — `rk-git` · evidence: F12 (360 trailer commits — the corpus supplying the fixtures) [measured] · decisions:
**ADR-0109** (three trailer axes), ADR-0105 cl. 3 (the grammar table in design §5.4) (ADR-0018 related)
GIVEN a commit whose `Story:` / `Slice-Verdict:` / `Gate:` trailer does not parse against the declared trailer grammar
(ADR-0105 clause 3's table: key, value ABNF, column anchoring, the verdict vocabulary, the unknown-key rule)
WHEN `reconcile()` reads the history
THEN the commit is surfaced as drift with the offending trailer text attached
AND it is neither silently skipped nor coerced into a state transition.
*Verified by:* fixture commits with malformed trailers in the corpus repo; assertion that each yields exactly one
`Drift::UnparsableTrailer` and zero state transitions.

### 6.2 Keying: SHA and tree hash only

**AC-166** — `rk-store` · evidence: §4.5 rule 2 (redkiln's verify cache keys on `(cmd, base, tree)`) [documented] ·
decisions: ADR-0018
GIVEN a store table that references the repository
WHEN the schema is validated
THEN every key column referencing the repository declares a key domain of `sha` or `tree`
AND a repo-referencing key column declaring any other domain — or none — is a schema-lint error.
*Verified by:* a schema lint over the migration files (`rk-store` build test) reading the declared domain, with a
fixture migration carrying a `branch TEXT` key that must fail the build. (Repaired: the original wording asked a lint to
recognise "a branch name" by inspection, which is not decidable from a column type. The declared domain makes it so.)

**AC-167** — `rk-git` · evidence: §4.5 rule 2 [documented] · decisions: ADR-0018
GIVEN rows anchored to commits that a rebase, reset, amend or force-push has made unreachable
WHEN `reconcile()` runs at the next entry
THEN those rows are reported as orphaned, with their anchor SHAs
AND no row is deleted or re-anchored as a side effect of the report.
*Verified by:* a corpus test that rewrites a branch and asserts the orphan set equals the expected anchors, plus
row-count equality before and after the report.

**AC-168** — `rk-store` · evidence: §4.5 rule 2, ADR-0018 ("surfaced, never deleted") [documented] · decisions: ADR-0018
GIVEN any code path in `rk-store`
WHEN the crate is built
THEN no `DELETE` against a tier-1 table exists outside the explicit `doctor --rebuild` path
AND adding one fails the build.
*Verified by:* a source lint over the SQL surface; a fixture patch introducing a delete that must fail CI.

**AC-169** — `rk-git` · evidence: ADR-0018 (rewrite events) [documented] · decisions: ADR-0018
GIVEN an orphaned anchor whose successor commit is identified by a recorded mechanism (patch-id equality, `ORIG_HEAD`,
or a reflog entry the operation itself wrote)
WHEN `reconcile()` runs
THEN a `rewrite(old_sha -> new_sha, mechanism)` event is emitted and the affected rows are re-anchored to `new_sha`
AND the `old_sha` anchor is retained in the event so the history is auditable.
*Verified by:* unit tests per mechanism; an assertion that every re-anchored row is preceded in the log by a `rewrite`
event naming both SHAs — a re-anchor with no preceding event fails.

**AC-170** — `rk-git` · evidence: ADR-0018 [documented] · decisions: ADR-0018
GIVEN an orphaned anchor for which no mechanism identifies a successor
WHEN `reconcile()` runs
THEN the row stays orphaned and drift-visible
AND no successor is inferred from branch name, file path, commit message or timestamp proximity.
*Verified by:* a negative test whose rewritten commit deliberately has the same message and path set but a different
patch-id, asserting zero `rewrite` events and one surviving orphan.

**AC-171** — **human gate** · evidence: none — no finding in the brief measures this decision · decisions: ADR-0018
GIVEN an orphan that survives AC-170 and blocks the next action for an item
WHEN the kernel needs a decision about whether the rewritten history carries the same work
THEN the item routes to a human gate carrying the orphan set, both SHAs and the diff between them.
*Verified by:* **routed to a human gate; not machine-decidable.** A rebase that also edits content produces no patch-id
match, and "is this the same work" is a judgement about intent, which no instrument in this programme measures.
The machine-checkable part is only that the gate is *opened*: asserted by a test that an unresolvable orphan yields a
`gate_opened` event and never a silent continuation.

### 6.3 `reconcile()` at every entry, git → store

**AC-172** — `rk-cli` + `rk-kernel` · evidence: F19 (drift 12/174 = 7%; 3/95 ≤1 h vs 9/79 >1 h, 3.6×) [measured, n=174,
**one operator, one repository**; the 3.6× is RR 3.6 with CI [1.0, 12.9], Fisher p = 0.039 — marginal at n = 174,
directional only (PIA-073)]; F18 (45–48% of gates exceed 1 h) [measured, one operator] · decisions: **ADR-0109**
(ADR-0018 related)
GIVEN any CLI entry point that can cause an action (`run`, `answer`, `doctor`; `status` is read-only and does **not**
reconcile — ADR-0105 clause 8)
WHEN it reads state that drives a decision
THEN that state is only obtainable as the output of `reconcile()` for this invocation
AND a new command that reads state without reconciling fails to compile.
*Verified by:* the planner accepting only a `Reconciled<State>` value (type-level, so the compile failure is the
instrument); an entry-point enumeration test asserting exactly one `reconcile_started` event per invocation, which fails
if an entry point is added without one.

**AC-173** — `rk-git` · evidence: F19 (drift 7%; the world moves rarely and predictably — which forbids an optimisation
that skips reconcile, brief §C.3) [measured] · decisions: **ADR-0109** (its forbids block: no cache, TTL or flag may skip
`reconcile()`) (ADR-0018 related)
GIVEN a second invocation moments after a first, with no intervening git activity
WHEN `reconcile()` runs
THEN it runs in full and no cache, TTL or "unchanged since" flag skips it
AND no configuration or CLI flag exists that disables it.
*Verified by:* a **differential** instrument (amended 2026-09-11, PIA-059 — the earlier two tests passed on a
`reconcile()` that consulted a cache one call deeper): `reconcile()` is run twice through the `rk-git` read layer with the
read set recorded, and the set of git objects read on the second invocation is asserted equal to `rev-list --all` (or to
the first invocation's set); plus a fixture that rewrites a trailer commit via `git replace` without moving `HEAD`
between the two invocations and asserts the rewrite is reflected in the second result. The CLI surface snapshot (no skip
flag can appear without updating it) and the `reconcile_started`-on-every-entry assertion are kept as defence in depth.
*Cut instrument:* "a source lint for early-return branches in `reconcile`" — early returns are legitimate control flow
and the lint would fire on correct code while missing a TTL cache one call deeper. It is not an instrument.
*Cut claim:* "drift is cheap to catch" — **no measurement of `reconcile()` cost exists anywhere in the brief.**

**AC-174** — `rk-git` · evidence: §4.5 rule 3 (git → store, never the reverse) [documented] · decisions: **ADR-0109**,
ADR-0077 (gix owns the read path), ADR-0003 (ADR-0018 related)
GIVEN a repository whose git directory is mounted read-only
WHEN `reconcile()` runs
THEN it completes and reports state and drift
AND it creates no commit, ref update, checkout, index write or stash.
*Verified by:* an integration test against a read-only git dir, which fails on any write error or mutation; plus a
before/after comparison of the git dir's file digests.

**AC-175** — `rk-git` · evidence: §4.5 rule 3 [documented] · decisions: **ADR-0109** (ADR-0018 related)
GIVEN `reconcile()` has run once
WHEN it runs again with the world unchanged
THEN the resulting state is equal to the first result
AND zero additional drift observations are recorded.
*Verified by:* a property test over corpus histories asserting `reconcile(reconcile(w)) == reconcile(w)` and an empty
second drift set.

**AC-176** — `rk-git` · evidence: F19 (drift 12/174 = 7%, concentrated on gaps over 1 h) [measured]; F18 (45–48% of
gates exceed 1 h, so the case is common) [measured] · decisions: ADR-0018, ADR-0017
GIVEN an item whose world changed while a human gate was open
WHEN `reconcile()` runs at the next entry
THEN the gate answer bound to a tree hash that no longer matches is classified `void`
AND the gate re-opens with the drift attached, rather than the answer being adjusted to fit the new tree.
*Verified by:* a unit test on the gate record `(token, tree_hash, deadline)` (ADR-0017) asserting the `void`
classification and a second `gate_opened`; plus a lint that no code path assigns to a recorded `tree_hash`.
**Boundary note:** this criterion is the reconcile-side half only; the gate record's own semantics belong to the gates
area.

**AC-177** — `rk-git` · evidence: **none available.** P8 §5.4(3) is circular by construction (arrivals were forced and
the mark was minted by the driver implementing the semantics), so nothing in the brief measures second arrivals ·
decisions: ADR-0017, ADR-0018
GIVEN a gate whose single-use token has already been consumed
WHEN a second answer carrying the same token arrives through any channel
THEN the second answer is refused and recorded as a duplicate
AND the first answer's effect is unchanged.
*Verified by:* a unit test on token consumption with two arrivals, asserting one refusal event and an unchanged first
effect. **The criterion is checkable; the empirical question it answers is not.** How often real second arrivals occur
is unmeasured and this criterion must not be cited as evidence that they are rare or common.

### 6.4 One store per repository; identity; no hooks

**AC-178** — `rk-store` · evidence: §4.5 rule 1 (locks already live at the common dir) [documented] · decisions: ADR-0018
GIVEN a linked worktree of a repository
WHEN the store is opened from inside it
THEN the resolved store path equals `git rev-parse --git-common-dir`/redkiln/ of the main working tree
AND no second store file is created anywhere under the linked worktree.
*Verified by:* an integration test creating N worktrees and asserting one store path and exactly one store file on disk
(a recursive scan of every worktree for store files must find none).

**AC-179** — `rk-store` · evidence: §4.5 rule 5 (Backlog.md #558) [documented — external reference, not in the evidence
brief] · decisions: ADR-0018
GIVEN a store whose identity row records a root commit and common dir
WHEN it is opened against a repository with a different root commit
THEN the open is refused with a typed error naming both identities
AND no row is read or written.
*Verified by:* a fixture store plus a second repository; assertion of the error variant and of zero writes (file digest
unchanged after the refused open).

**AC-180** — `rk-store` · evidence: §4.5 rule 5 (schema version, harness versions, boot-id) [documented] · decisions:
ADR-0018
GIVEN a store written at an earlier schema version
WHEN it is opened by a newer binary
THEN the open is refused and names the migration required
AND no implicit in-place migration occurs.
*Verified by:* a checked-in fixture store at version N-1; an assertion that the file's bytes are unchanged after the
refused open.

**AC-181** — `rk-store` · evidence: §4.7 (parallel across antichains, one worktree each) [documented] · decisions:
ADR-0018, ADR-0003
GIVEN two redkiln processes running in two worktrees of the same repository, **of two different items** — each process
owns its item's branch and run-owned worktree (ADR-0105 clause 8; the ownership refusal is AC-529)
WHEN both write to the store concurrently
THEN both writes are present after both processes exit
AND neither process fails with a lock error, and no second store file exists.
*Verified by:* a two-process integration test asserting both rows present, zero lock errors and a single store file
(one writer connection per process behind a mutex, `busy_timeout` 5000 ms — ADR-0105 clause 7; R-SQLITE confirms the
rusqlite default).
(Repaired: "serialised by the single store (WAL + `busy_timeout`)" named the implementation; the observable is both rows
landing without a lock error. F22 was cut as evidence — it measures background-process death, not store contention.)

**AC-182** — `rk-git` · evidence: §4.5 (no hooks; git-branchless `reference-transaction` and its issue #1334)
[documented — external reference, not in the evidence brief] · decisions: ADR-0018, ADR-0003
GIVEN any redkiln invocation
WHEN it completes
THEN nothing has been written under the repository's `hooks/` directory
AND the product functions identically in a repository whose hooks directory is absent or read-only.
*Verified by:* a source lint forbidding writes under `hooks/`; an integration run against a repo with the hooks
directory removed, asserting identical output to the same run with it present.

**AC-183** — `rk-git` · evidence: F19 (drift 12/174 = 7% caught at entry) [measured] · decisions: **ADR-0109** (ADR-0018
related)
GIVEN a git operation performed out of band by a human in a plain shell (commit, rebase, force-push)
WHEN the next redkiln entry occurs
THEN that operation is reflected in state by `reconcile()` alone
AND no notification, daemon or hook was required for it to be seen.
*Verified by:* an integration test that shells out to `git` between two invocations, asserting the second invocation's
state includes the change; the run executes with the hooks directory removed and asserts no child process outlives the
first invocation.
*Cut claim:* "none missed in 174" — F19 reports 12 drift events in 174 suspensions and 174/174 lossless **suspension**.
Neither figure is a measurement of drift the system failed to notice; no such oracle exists in the brief.

**AC-529** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-store` + `rk-git` ·
evidence: ADR-0018 (the kernel "refuses to write when the current worktree's HEAD is not the run's branch") [documented];
ADR-0105 clause 8 (narrows "current worktree" to "the worktree the store owns for the run") [documented]; no spike
measured a foreign-worktree write · decisions: ADR-0105, ADR-0018
GIVEN a tier-1 write for a run whose store records a run-owned worktree and branch
WHEN the write is attempted from a process whose target worktree is not the run-owned one, or whose `HEAD` is not the
run's branch — regardless of the process's cwd
THEN the write is refused with a typed `rk-git` error naming the target path and branch and the run-owned path and
branch
AND no commit, ref update or store row results; the run-owned worktree is resolved **from the store**, never from cwd.
*Verified by:* an integration test that opens a second worktree of the same repository on another branch, attempts a
tier-1 write for the run from inside it, and asserts the typed refusal, an unchanged `HEAD` on both branches and zero new
store rows; a positive control from the run-owned worktree that succeeds; an assertion that the resolution reads the
store's worktree row and not `std::env::current_dir`. *(PIA-029 named the missing criterion for ADR-0018's forbids clause;
ADR-0105 clause 8 is the rule it verifies.)*

### 6.5 Rebuild from a clean clone

**AC-184** — `redkiln doctor --rebuild` · evidence: F12 (360 trailer commits over 74 slices, **across all refs of the
frozen clone** — 118 refs, of which 46 are reachable from `47faec9f`; the corpus the rebuild is run against) [measured;
the ref-set clause added 2026-09-11, PIA-002] · decisions: **ADR-0109**, ADR-0105 cl. 9, ADR-0013 (ADR-0018 related)
GIVEN a clean clone of a repository, with no store present
WHEN `redkiln doctor --rebuild` runs
THEN the resulting tier-1 state is byte-identical under the canonical serialiser to the **tier-1 projection** of the
original store — the tier-1 rows whose anchor is reachable from the clone's refs; orphan rows and rewrite events are
tier-2-derived (recomputed by `reconcile()` at every entry) and sit **outside** this differential (ADR-0105 clause 9)
AND the rebuild consumes only the clone (a store file placed at the target path is not read).
*Verified by:* the rebuild differential test, run in CI on every commit against the F12 substrate **materialised as a
pinnable artefact in `redkiln-corpus` — a `git bundle create --all` of the frozen predecessor clone carrying all 118 refs,
pinned by that repository's SHA, with a ref-set assertion beside the 40-hex one [WILL EXIST after runbook Step 2]**
(ADR-0013), including an adversarial-store variant that must produce identical output and a **rebased-commit fixture**
whose expected output (the orphaned row reported, the projection unchanged) is pinned; the CI job additionally asserts
the corpus pin is a 40-hex SHA and not a branch, and that the bundle's ref set equals the pinned list, and fails if
either does not. Until Step 2 lands the artefact this test is a **local-only suite with a named runner**, not a CI job.
*Cut claim, corrected 2026-09-11 (PIA-036, PIA-075):* "`reconcile.rs` rewritten and validated on 360 trailer commits / 74
slices". F12 is S0.2, a trailer-corpus study reporting 15/74 ever-rejected and 21.6% sealed more than once. A
`reconcile.rs` **was built and run in S0.2** (`spikes/runner/src/reconcile.rs`, a re-implementation of the old tree's
`src/store/verify.ts` — PIA-078) and **exercised** over the 360 commits / 74 slices; it was **not validated against an
oracle or against engine decisions**. "Validated" was the fabrication; "exercised" is the record. After Step 2 the file
lives in `redkiln-corpus` at the pin, and the runbook's Step 6 names that location.

*(AC-185 as originally written — "CI runs the rebuild test" — was cut; see Refutation notes. Its only non-vacuous
content, the SHA pin, is folded into AC-184 above. The id is reassigned below.)*

### 6.6 Resume: one idempotent entry point

**AC-186** — `rk-cli` · evidence: §4.5 (L2.3, no `--resume` flag) [documented]; F38 (mid-graph resume is lossless across
a real kill: 0 changed, 0 lost, digests identical) [measured, n=3, PARTIAL — motivating: it is why one entry point can
serve both cases] · decisions: **ADR-0107** (ADR-0018 related)
GIVEN the shipped CLI surface
WHEN it is enumerated
THEN no `--resume`, `--continue` or equivalent flag exists on any command
AND `redkiln run <item>` is the sole entry for both a first run and a resumed run.
*Verified by:* a `clap` surface snapshot test that must be updated deliberately to add any flag.

**AC-187** — `rk-kernel` · evidence: F38 (3/3 across a real `taskkill /T /F`) [measured, n=3, PARTIAL — the 65-min and
24-h arms and all three controls are unrun] · decisions: ADR-0107 (one idempotent entry), ADR-0097 (ADR-0018 related)
GIVEN a run killed mid-graph after a step has committed, **whose EventStore survives the kill** (a run whose store is
lost replays from its trailer commits at stage grain under AC-161 — ADR-0097 decision 1; amended 2026-09-11, PIA-008)
WHEN `redkiln run <item>` is invoked again
THEN the committed step is not re-dispatched — its memo key resolves — and the next action equals, under the canonical
serialiser, the action taken at the same point by an uninterrupted control run of the same fixture.
*Verified by:* the kill-and-resume integration test asserting a memo hit, zero duplicate dispatch events, and equality
against a recorded uninterrupted control trace for the same fixture.
**PROVISIONAL at long durations:** P8's 65-minute and 24-hour arms are unrun, so equality is demonstrated only at short
durations and n=3. **Note also that P8's three controls are unrun** — the control trace this criterion compares against
is a product-test artefact, not something the spike measured.

**AC-185** *(id reassigned — split from AC-187)* — `rk-kernel` · evidence: F38 (0 changed, 0 lost, digests identical)
[measured, n=3, PARTIAL] · decisions: ADR-0018
GIVEN a run killed mid-graph after a step has produced output
WHEN `redkiln run <item>` is invoked again
THEN every output produced before the kill is present afterwards with an unchanged digest
AND no such output is rewritten by the resumed run.
*Verified by:* the kill-and-resume test capturing a digest per output artefact before the kill and asserting equality
after; any missing or changed digest fails.

**AC-188** — `rk-kernel` · evidence: F19 (stated limit: *the FSM was never replayed*, so "same next action" is unproven)
[measured] · decisions: ADR-0018
GIVEN a recorded event log and the git history it was written against
WHEN the deterministic workflow is replayed from the log
THEN the sequence of selected actions equals the sequence originally taken
AND a non-deterministic construct in the workflow layer (clock, RNG, filesystem read outside an activity) fails replay.
*Verified by:* the replay test in `rk-kernel` over **golden event logs recorded by `rk-kernel` on the pinned corpus
repository and committed to `redkiln-corpus` as fixtures [WILL EXIST after runbook Step 5 — none exists today; their
generation is a Step 5/Step 6 checkpoint item, PIA-002]**, asserting sequence equality; a determinism lint denying
clock/RNG/filesystem symbols in the workflow module, with a fixture using one that must fail. **This criterion exists
specifically to close F19's stated limit, and AC-161 depends on it.**

### 6.7 Worktrees, disposable clones, merge queue

**AC-189** — `rk-git` · evidence: §4.7 (strictly sequential within a tree; parallel across project-DAG antichains, one
worktree each) [documented] · decisions: ADR-0018, ADR-0003
GIVEN a project DAG with an antichain of width N
WHEN the scheduler dispatches
THEN at most one activity is in flight per worktree, and each concurrent item has its own worktree.
*Verified by:* a scheduler property test over generated DAGs asserting the per-worktree in-flight invariant over the
whole dispatch trace. (The trailing "AND two activities never share a working tree" was a restatement of the first
clause and was trimmed, not a second behaviour.)

**AC-190** — `rk-git` · evidence: §4.5 rule 2 (rows keyed by SHA, not by path) [documented] · decisions: ADR-0018
GIVEN a worktree destroyed while its item is mid-run
WHEN `reconcile()` runs at the next entry
THEN no row is orphaned by the destruction
AND the item resumes in a freshly created worktree without operator repair.
*Verified by:* an integration test that `rm -rf`s a worktree and `git worktree prune`s, asserting zero new orphans and a
successful resume. (F22 was cut as evidence — a background process dying is a different failure than a directory being
removed, and its numbers say nothing about row anchoring.)

**AC-191** — `rk-git` (review clone) · evidence: F17 (the reviewer restored the fix from the object store; 42 turns,
22,853 output tokens, returned `approved` — recorded as an instrument defect) [measured, n=1] · decisions: ADR-0018,
ADR-0003, ADR-0098 (the review clone is a retained gix write), ADR-0077, ADR-0105 clause 5
GIVEN a review dispatch for a candidate at SHA X
WHEN the reviewer's environment is prepared
THEN the prepared clone contains exactly one commit and no other branch, tag, note, stash or reflog entry
AND the F17 recovery procedure — reading a pre-fix blob for a changed file out of that clone's object store — fails.
*Verified by:* assertions on `git rev-list --count --all == 1` and on the reachable object set in the prepared clone; an
F17-shaped regression fixture that replays the historical recovery attempt and must now fail. (Repaired: "the pre-fix
content is not recoverable" is an unbounded universal negative no test can discharge. The named procedure can be.)
*Mechanism (2026-09-11, ADR-0098; grounding EXT-GITOXIDE-005):* the clone is a **gix write operation retained on
purpose** — `PrepareFetch::with_revision` + `with_shallow` at gix 0.87.1 (PR #2824, merged 2026-08-05), not gitoxide
discussion #2309, which documents a panic; **S-GIX/c** (consumer: this path; cost one fixture) checks the resulting HEAD
tree equals `git clone --depth 1 --revision <sha>`'s. The clone is **bare** unless the review activity declares it needs a
working tree (ADR-0105 clause 5).

**AC-192** — `rk-git` (review clone) · evidence: F26 (E3-H: the reviewer keeps `Bash`; 20/20 across two harder tiers)
[measured]; brief §E (the P8 fixture failed *because* the agent has Read+Bash) [documented] · decisions: ADR-0018
GIVEN a review dispatch with `Bash` and write tools enabled
WHEN the review completes or fails
THEN the source worktree's tree hash is unchanged from before the dispatch.
*Verified by:* a tree-hash equality assertion around every review dispatch, and a test in which the reviewer writes a
file inside the clone, asserting the source worktree tree hash is unchanged and that the file appears in no subsequent
commit.
*Cut citation:* "F17 (GATE-02's write-tool rule)" — **there is no GATE-02 in the evidence brief.**

**AC-199** *(id reassigned — split from AC-192)* — `rk-git` (review clone) · evidence: F31 (S-TYPED schema conformance
52/52 = 100%; `repro.command` fill 147/147) [measured] · decisions: ADR-0018, ADR-0003
GIVEN a completed review inside a disposable clone
WHEN its result crosses back to the kernel
THEN the only value accepted is a `Findings` payload conforming to the declared schema
AND a result that does not conform is refused rather than parsed out of prose.
*Verified by:* a schema-conformance assertion per review dispatch; a fixture returning prose containing a
JSON-shaped fragment, which must be refused. (F20's mechanism — a tool-restricted card silently voids `--json-schema`
at exit 0 — is why the refusal must be explicit rather than inferred from a successful exit.)

**AC-193** — `rk-git` (review clone) · evidence: F22 (nothing may depend on process survival; ~12 background kills vs
one 12,000 s detached survivor) [measured, operational] · decisions: ADR-0018
GIVEN a review dispatch that crashes, times out or is killed
WHEN the next entry occurs
THEN the clone directory no longer exists
AND a stale clone found at entry is **reported as drift by `reconcile()` and removed by the entry step's action** —
`reconcile()` mutates nothing (AC-174); removal is the entry's write (ADR-0105 clause 8) — rather than reused.
*Verified by:* a kill test asserting the path is absent after the next invocation; a fixture that plants a stale clone
and asserts it is reported, removed by the entry step and never read from; an assertion that `reconcile()` itself
performed no filesystem removal.

**AC-194** — `rk-git` (review clone) · evidence: F31 (`repro.command` fill 147/147) [measured]; §4.6 verification gate
(`GreenAt(sha)`, `basis`) [documented] · decisions: ADR-0018
GIVEN a `Findings` value returned from a review clone
WHEN the deterministic gate re-executes its repro records
THEN every repro is executed at the reviewed SHA X and the verdict is recorded as `GreenAt(X)`
AND a finding whose repro ran against any other SHA is refused rather than scored.
*Verified by:* a SHA-capture assertion on each repro execution; a negative fixture whose repro names a different SHA and
must be refused.
*Cut citations:* "F17 defect 2 (adoption and isolation verified from the child's own report)" — **the brief records no
numbered defects under F17**, and the child's-own-report principle is F8/§C.11, cited correctly in AC-195. "(B6)" —
**no such identifier exists in the brief.**

**AC-195** — `rk-git` (review clone) · evidence: F8 (MCP = 4,631 tokens = **56%** of a default cache write; an ignored
flag looks identical to an honoured one, so it must be verified from the child's own `system/init`) [measured] ·
decisions: ADR-0018, ADR-0003
GIVEN a review dispatch inside the disposable clone
WHEN the child process reports its own `system/init`
THEN the reported working directory is the clone path and the reported MCP/tool set matches the declared card
AND a mismatch aborts the review as a configuration error before findings are scored.
*Verified by:* an assertion on the child's `system/init` record per review dispatch; a fixture with an ignored isolation
flag that must abort.

**AC-196** — `rk-git` (merge queue) · evidence: §4.7 (serialised at a merge queue) [documented] · decisions: ADR-0018
GIVEN two worktree branches ready to integrate
WHEN the merge queue processes them
THEN they merge strictly one at a time
AND a second merge never begins before the first has produced a commit and its trailer.
*Verified by:* a concurrency test asserting a single-writer invariant on the integration ref and that no `merge_started`
event precedes the preceding merge's trailer commit.

**AC-197** — `rk-git` (merge queue) · evidence: §4.6 verification gate (`GreenAt(sha)`, `basis`) [documented]; F31 (four
adjudicated defects survived fully green suites of 1,890/1,890 and 1,907/1,907) [measured — **motivating, not
evidential**: it shows a green suite does not clear an AC, not that a verdict goes stale across a merge] · decisions:
ADR-0018
GIVEN a branch admitted to the merge queue with a verification verdict
WHEN the merge produces a tree different from the tree the verdict was issued against
THEN the verdict is invalidated and verification re-runs against the merged tree hash before the merge is sealed
AND no verdict is carried forward across a changed tree.
*Verified by:* an integration test with a conflicting concurrent merge, asserting a second `verify` event keyed to the
merged tree hash and refusal of the stale verdict.

**AC-198** — `rk-git` (merge queue) · evidence: F22 (~12 background kills vs one 12,000 s detached survivor; `issue-81`
killed 3× as background, 116 s in foreground) [measured, operational] · decisions: ADR-0018
GIVEN the merge queue process is killed mid-merge
WHEN the next entry occurs
THEN the queue state is recovered from git and the store, with no merge lost and no merge applied twice
AND recovery requires no live process to have survived.
*Verified by:* a kill-during-merge integration test asserting exactly-once integration per branch across the kill, run
with no surviving child process.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-049 | Drift rate at `reconcile()` | — |
| SM-050 | Orphan and rewrite volume | no baseline |
| SM-051 | Rebuild fidelity and cost | no baseline |
| SM-052 | Resume cost | — |
| SM-053 | Worktree cache cost | — |
| SM-054 | Review-clone isolation | no baseline |
| SM-055 | Merge-queue re-verification rate | no baseline |
| SM-056 | Tier-1 recoverability in production | — |
| SM-057 | Store-open refusals | saturating; no baseline |
| SM-058 | Gap duration preceding an entry | — |

#### Residual risk this area could not repair

- **ID REASSIGNMENT HAZARD.** AC-185 and AC-199 were cut and their numbers reused for split halves. Anything citing the old AC-185 ('CI runs the rebuild test') or the old AC-199 ('change summary in the clone') will resolve to a different criterion. See §0.4.
- Every §4.5/§4.6/§4.7 citation is `[documented]` against the proposal, which is not in the evidence brief; §4.5's rule numbering is already internally inconsistent (rule 4 was cited both as 'single write path' and as the hooks rule). About a third of the surviving criteria rest on citations this pass had no instrument to check.
- AC-188 (FSM replay) is load-bearing for AC-161, AC-185, AC-187 and SM-056 and has NO supporting evidence — it exists precisely because F19 never replayed the FSM. Four criteria and a measure are stacked on an unproven capability.
- The empirical spine is one operator, one repository. F19 (n=174) and F18 (n=174/191) both carry that caveat, and SM-049/SM-058 inherit it. The 3.6× drift ratio is RR 3.6 with CI [1.0, 12.9], Fisher p = 0.039 — marginal at n = 174, directional only; its conditional Wilson intervals OVERLAP and it is never to be quoted as an established ratio (PIA-073, PIA-107).
- The change-summary-in-a-depth-1-clone question is now UNANSWERED, not resolved: cutting the old AC-199 removed a contradiction with AC-191 but decided nothing. It belongs on the open-questions list gated on S-BIN.
- SM-053's target rests on S-WT numbers in the F30 slot, and **F30 was never written as a dated entry** — but the run exists: `spikes/results/s-wt.jsonl` records the `--system-prompt` × worktree combination (w2–w10 cache-read share 1.000, n = 9; w1 0.807 cold), so S-WT-REC is DONE and the "unrun"/CONTESTED reading is withdrawn (PIA-075; AC-387 carries the cells). What remains weak is that it is one run, one machine.
- AC-174 (read-only git dir) and AC-181 (two-process contention) are environment-dependent tests; on Windows, the primary platform, a flaky instrument marked allow-fail is a criterion that has quietly died.
- The F12 substrate the rebuild and replay tests name (AC-184, AC-165, AC-444, AC-464) and the golden event logs AC-006/AC-188 replay do not exist at HEAD: the 1,312-file corpus and the 118-ref history live only in `D:/repos/redkiln`, and no golden log has been recorded [measured, PIA-002]. Both are [WILL EXIST] items on runbook Steps 2 and 5/6; until then every corpus-bound test here is a local-only suite, and a CI run that reports them green is reporting their absence.

---

## 7. Harness port and the `claude -p` adapter

**Area key:** `harness` · **Criteria:** AC-200..AC-239 **plus AC-523 and AC-526** (AC-207 retired, not reused; AC-523 and AC-526 are allocated outside the range — see §0.4) — **41 in force** · **Measures:** SM-061..SM-070 (SM-071, SM-072 unallocated)
**State:** 37 binding (AC-523 and AC-526 added 2026-09-11) · 3 provisional (AC-211, AC-228, AC-239) · 1 human-gated (AC-213)
**Sources:** proposal §4.3 (with §4.1 port rules, §3 principles 3/4/5/6, §4.7 fan-out); ADR-0003, 0004, 0007, 0010, 0016.

*Scope note.* **Unverified CLI surface — largely discharged 2026-09-11.** Several criteria named flags, environment variables or stream fields the evidence brief does not record — `CLAUDE_CODE_PROMPT_CACHE_TTL`, `--permission-prompts`, `--bare`, `isUsingOverage`, `system/api_retry` and its fields. The brief records only `rate_limit_event.unifiedWindows.{five_hour,seven_day}.utilization` (F1), `system/thinking_tokens` (F4), `system/init` tool and server counts (F8), `structured_output`/`subtype` (F20) and `usage.cache_*` (F3/F16). Since then each name has been checked outside the brief: `CLAUDE_CODE_PROMPT_CACHE_TTL` (values `5m`/`1h`, v2.1.242+) and `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` are on code.claude.com/docs/en/prompt-caching; `--permission-prompts` shipped at CLI 2.1.259; `--bare` is still API-key-only; the current CLI is 2.1.268 [documented, fetched 2026-09-10]. `isUsingOverage` is recorded (`false`) in F1's own payload (`spikes/FINDINGS.md` :42–48) and, as the runner's `is_using_overage`, on all 20 E0-headroom and all 60 S-VAR rows (plus two rows of `instrument-checks/e1-smoke.jsonl`); `system/api_retry`'s payload is captured on 139 rows in 12 result files [measured, recount 2026-09-11 — AC-026]. The **[surface unverified]** tag now survives only where a *value* has never been observed (a true `isUsingOverage`, AC-221/AC-378) or where the pinned CLI must be at or above a named version (AC-215). A green synthetic test on an unverified surface is still an unproven criterion; the surface is no longer unverified.

### 7.1 Stream and event retention

**AC-200** — `rk-adapter-claude` · evidence: F1, F4 [measured] · decisions: ADR-0003
GIVEN a `claude -p` dispatch whose stream contains an event type the adapter's schema does not know
(F4: `system/thinking_tokens` at 4–6 per dispatch; F1: a top-level `rate_limit_event`, both undocumented)
WHEN the dispatch completes
THEN the unknown event is present verbatim in the event log for that run, with its raw JSON preserved.
*Verified by:* a replay assertion over a recorded fixture stream with three synthetic unknown event types injected —
the log must contain all three byte-for-byte; plus a property test that log round-trip preserves unrecognised fields.

**AC-201** — `rk-adapter-claude` · evidence: F1, F4 [measured: two undocumented event shapes already observed],
R8 [documented: no published stability policy] · decisions: ADR-0003
GIVEN the same stream containing an unknown event type or an unknown field on a known event
WHEN the adapter parses it
THEN the dispatch still resolves to its normal outcome and no `StepError` is raised by the unknown shape alone.
*Verified by:* unit test on the stream decoder with `deny_unknown_fields` explicitly absent; a fixture stream carrying a
future-shaped event that must still yield `Ok`.

### 7.2 Isolation and adoption

**AC-202** — `rk-adapter-claude` · evidence: F8 [measured: 12 MCP servers, ~280 tools; cache write 8,243 → 3,611 isolated;
MCP = 4,631 tokens = 56% of the write; tools 280 → 34] · decisions: ADR-0003, ADR-0095 (the `StepError` table)
GIVEN a dispatch that passes `--strict-mcp-config`
WHEN the child emits `system/init`
THEN isolation is decided from the child's own reported tool/server list, and a list that does not match the declared
configuration fails the dispatch as `StepError::NotIsolated { surface: tools | mcp_servers | env, diff_digest }` — the
presence of the flag in argv is never accepted as evidence of isolation, because an ignored flag looks identical to an
honoured one (brief §C.11). SM-061's instrument is a **set digest** of the reported list against the declared one, not a
count (PIA-016).
*Verified by:* unit test with a scripted responder whose `system/init` reports 280 tools while argv carries the flag —
the dispatch must fail; a lint that no isolation assertion reads the adapter's own argv.

**AC-203** — `rk-adapter-claude` · evidence: F7 [measured: `--agents` alone → 4; both → BANANA] · decisions: ADR-0003
GIVEN a dispatch specification carrying `--agents` with no `--agent <name>`
WHEN the adapter builds the command line
THEN the dispatch is refused as a configuration error before any process is spawned, because `--agents` is inert alone.
*Verified by:* unit test on the command builder; a fixture spec that must fail; a `doctor` rule-table row at zero model cost.

**AC-204** — `rk-adapter-claude` · evidence: F8 [measured: a default child inherits the operator's whole environment —
12 MCP servers, ~280 tools], F7 [measured: the card's model+effort are accepted, so the card is the declaration]
· decisions: ADR-0003, ADR-0016, ADR-0095
GIVEN a dispatch that names a card via `--agent`
WHEN `system/init` reports the child's effective tool set
THEN adoption holds only if that set is **equal** to the card's declared tool set, and any difference — extra or missing —
fails the dispatch as `StepError::NotIsolated { surface: tools, .. }`; a superset is not adoption. `NotAdopted` is
reserved for the wrong agent answering (`{ expected_agent, observed }`), never for tool-set inequality.
*Verified by:* unit tests for the three cases (equal / subset / superset) against a scripted `system/init`; the superset
case must fail with `NotIsolated`; a fourth case where `system/init` names a different agent must fail with
`NotAdopted`. The superset rule is [inferred] from F8's inheritance result — the brief records no measured instance of a
child retaining parent capabilities *under* `--agent`, so this case is a guard, not a reproduction of an observed defect.
**Correction (2026-09-11, audit PIA-016; decision #5 of `spikes/DECISIONS-2026-09-11.md`).** As authored this classified
tool-set inequality `NotAdopted` while AC-202, AC-386 and design §7.4 classified the same condition `NotIsolated`, and only
`NotIsolated` triggers AC-399's quarantine. One class for one condition: `NotIsolated` everywhere; AC-386 and AC-399 are
unchanged; §0.3 row C4 records the contradiction and its resolution.

### 7.3 Structured output

**AC-205** — `rk-adapter-claude` · evidence: F20 [measured: `structured_output` absent, exit 0, `subtype: success`] · decisions: **ADR-0108** (ADR-0016 related)
GIVEN a dispatch that carries a JSON schema and whose effective tool set — **the card's declared `tools:` list and nothing
else** (ADR-0108 decision 2; a card with no `tools:` key is a configuration error, AC-493's doctor row) — does not include
`StructuredOutput`
WHEN the adapter prepares the dispatch
THEN it is refused as a configuration error before any process is spawned.
*Verified by:* unit test on the dispatch builder; a fixture card lacking `StructuredOutput` that must fail; a `doctor`
rule-table row reporting the same card at configuration time at zero model cost (AC-095, AC-417). The check is on the
argv the adapter assembles — the same command builder that lints AC-132's per-request budget precondition (ADR-0108
decision 4).

**AC-206** — `rk-adapter-claude` · evidence: F20 [measured], brief §C.5 · decisions: ADR-0074 (supersedes ADR-0004), ADR-0016
GIVEN a dispatch that passed a schema and returns exit 0 with `subtype: success` but no `structured_output` field
WHEN the adapter classifies the result
THEN the result is `Err(StepError::Schema { .. })` and never `Ok` — the fail-open path that F20 records is unrepresentable.
*Verified by:* unit test over a fixture stream reproducing F20's recorded shape (exit 0, `subtype: success`, no
`structured_output`); a type-level guarantee that the schema-bearing dispatch path has no constructor for `Ok` without a
validated value.

*(AC-207 retired — see Refutation notes. The id is not reused.)*

**AC-208** — `rk-adapter-claude` · evidence: F20 [measured], brief §C.5 · decisions: ADR-0074 (supersedes ADR-0004), ADR-0016
GIVEN a dispatch result whose typed value is absent or invalid
WHEN the adapter attempts recovery
THEN no structured value is recovered by reading the assistant's prose, and the failure surfaces as `StepError::Schema`.
*Verified by:* a lint/grep rule that the adapter crate contains no JSON-from-text extraction (no regex or fenced-block
scraper) on the result path; a fixture whose prose contains valid JSON that must still fail.

### 7.4 Prompt, cache and card delivery

**AC-209** — `rk-adapter-claude` · evidence: F16 [measured: `--system-prompt` share 1.000 across commits, 50 dispatches,
within-arm range ≤0.001], S-WT [measured: w1 0.807, w2–w10 1.000] · decisions: ADR-0016
GIVEN two dispatches of the same card version with different volatile inputs
WHEN the adapter assembles the command
THEN the `--system-prompt` bytes are byte-identical between them (a volatile input that reached the system prompt would
change them).
*Verified by:* a golden test hashing the assembled system prompt across a matrix of volatile inputs (all hashes equal);
a replay assertion that the recorded prefix digest on both events is the same.

**AC-210** — `rk-adapter-claude` · evidence: F10 [measured, n=1 per gap: full hit at 45.1 min, 0.690 at 61.6 min; the
boundary was bracketed (45.1, 61.6) and never located], L6.4 [documented]; code.claude.com/docs/en/prompt-caching
(`CLAUDE_CODE_PROMPT_CACHE_TTL` takes `5m`/`1h`, v2.1.242+) [documented, fetched 2026-09-10] · decisions: ADR-0016,
**ADR-0078** (both TTLs set together)
GIVEN any dispatch
WHEN the child environment is constructed
THEN `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and** `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` are set explicitly and both
values are recorded on the dispatch event (ADR-0078 forbids setting the first without the second; AC-526 is the
refusal criterion).
*Verified by:* unit test on the environment builder asserting both variables; a replay assertion that no dispatch event
lacks either TTL field. *(Amended 2026-09-11, PIA-029: the subagent variable added; the `[surface unverified]` tag is
withdrawn — the variable is documented.)*
This criterion asserts only that the variables are set and recorded — F10 measured a ~1 h TTL, not that this variable
controls it, and the criterion must not be read as evidence that the TTL was extended.

**AC-526** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-adapter-claude` /
`redkiln doctor` · evidence: ADR-0078 ("Forbids setting `CLAUDE_CODE_PROMPT_CACHE_TTL` without also setting
`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`") [documented]; the two variables on code.claude.com/docs/en/prompt-caching
[documented, fetched 2026-09-10] · decisions: ADR-0078, ADR-0113, ADR-0016
GIVEN a configuration, pack value or environment builder path that would set one of `CLAUDE_CODE_PROMPT_CACHE_TTL` and
`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` without the other, or with two different values
WHEN the adapter is constructed or `redkiln doctor` runs
THEN the configuration is refused as a typed configuration error naming both variables, before any dispatch
AND a constructed child environment never carries exactly one of the two.
*Verified by:* a unit test on the environment builder over the four cases (neither / first only / second only / both
equal) asserting refusal on the two single cases and on a mismatched pair; a `doctor` rule-table row with a red
fixture; an assertion over the suite's recorded dispatch events that every event carries both fields with one value.
*(PIA-029 named ADR-0078's forbids clause as one no criterion covered.)*

**AC-211** — PROVISIONAL (S-H1c unrun) — `rk-adapter-claude` · evidence: S-H1c registered and unrun (brief §E);
F28 correction and S-H1b [measured: 29,619–29,680 created at ~66 min, share 0.000, n=1 per gap] · decisions: ADR-0016
GIVEN a card whose `cache_profile` is `static` (AC-383) and no explicit selection of the append route for it
WHEN the adapter delivers the card
THEN the static `--system-prompt` route is used and `--append-system-prompt` /
`--exclude-dynamic-system-prompt-sections` are absent from argv; under `default` the adapter omits `--system-prompt` and
prepends the card body to the user message (AC-383's mechanism).
*Verified by:* unit test on the command builder; a golden argv test per profile; a card fixture asserting the `static`
default. **Block:** D16 makes the append route conditional on S-H1c, which has not run; the criterion fixes only the
default, and the append route's own acceptance criteria may not be written until S-H1c reports. *(Amended 2026-09-11,
PIA-012, decision #19: the GIVEN was "a repository configuration", which made the static route repository-wide and
contradicted AC-383's per-card profile; the route choice is per card.)* The subsidy the static route forfeits is a **shared-prefix
subsidy** — the docs scope the Claude Code cache to one machine and one directory, not to the account (PIA-103; grounding
EXT-CLAUDECO-025 [documented, fetched 2026-09-10]).

### 7.5 Model, effort, permissions, auth

**AC-212** — `rk-adapter-claude` · evidence: F7 [measured: the card's model+effort are accepted], F15 [measured: thinking
tokens low 0.0 / high 4.8 / xhigh 221.0, complete separation; **low vs high did not separate**] · decisions: ADR-0003
GIVEN a dispatch whose `model` or `--effort` value cannot be traced to the **resolved (bound) card** — the dispatch card
the binder materialises by injecting the role's tier into the authored card; the authored card file itself may not carry
`model` or `effort` (AC-118, AC-418), so the values are traceable only through the role (see the glossary entry in §0)
WHEN the adapter prepares the dispatch
THEN it is refused as a configuration error before spawn — including when an environment variable or the parent process
would supply the value, which is never read.
*Verified by:* unit tests for (a) a bound card whose role declares no `effort`, (b) a bound card whose role declares no
`model`, (c) an env var set while the bound card is complete (the env value must not appear in argv); a `doctor`
rule-table row. *(Amended 2026-09-11, PIA-015: as authored, fixtures (a)/(b) required the authored card to carry the
tier, which AC-118/AC-418 reject — both rows rejected every card.)*
F15 licenses only that effort reaches the child and that the low/high pair is indistinguishable; it does not license any
claim about which effort a card should choose.

**AC-213** — HUMAN GATE — `rk-adapter-claude` / technology pack · evidence: timeout history [measured: 900 s default,
written for the fast condition, cost a CLEAN record → 2,400 → 3,600 → one 5,400 s retry], F21 [measured: E3 CLEAN
317–2,055 s] · decisions: ADR-0003
GIVEN a new activity class with no recorded wall-clock distribution
WHEN its timeout is first configured in a technology pack
THEN the number is set through a human gate that names the slow condition it is sized for, and the pack may not inherit a
global default.
*Verified by:* routed to a human gate; not machine-decidable — "the slow condition" for an unseen activity class has no
measured distribution to check against, and the one time it was guessed the default was written for the fast condition and
cost a record. The machine half (a class that *does* have history) is AC-223. The machine-checkable residue of this
criterion is narrow and is asserted separately: a pack that declares a timeout for a class with no recorded distribution
and no gate record is a lint error.

**AC-214** — `rk-adapter-claude` · evidence: platform docs [documented], principle 5 · decisions: ADR-0003
GIVEN any dispatch
WHEN the command line is assembled
THEN `--permission-mode` is present with an explicit value taken from the card or pack; there is no implicit default.
*Verified by:* unit test on the command builder; a lint that the flag is unconditional on the dispatch path.

**AC-215** — `rk-adapter-claude` · evidence: platform docs (`--permission-prompts none`) [documented], principle 5
· decisions: ADR-0003 · **[surface unverified]**
GIVEN a dispatch against the pinned CLI version, which must be at or above the version that provides `--permission-prompts`
WHEN the child would otherwise prompt for a permission decision
THEN the child cannot wait on a prompt: `--permission-prompts none` is passed and the child's stdin is closed, so a
prompt-shaped stall is impossible rather than merely unlikely.
*Verified by:* unit test asserting the flag and a null/closed stdin handle; an integration test with a fixture that
requests a disallowed tool, which must terminate rather than hang under a wall-clock bound.
The minimum version is documented, not measured here — `--permission-prompts` shipped at CLI **2.1.259**; the current
release is 2.1.268 [documented, fetched 2026-09-10] — and must be recorded in the pin alongside `TESTED_CLI_VERSION`
before this criterion is claimed met. F22's stall history is *not* cited: F22 is about detached processes, not prompts.

**AC-216** — `rk-adapter-claude` · evidence: charter §6, platform docs (`--bare` is still API-key-only at CLI 2.1.268)
[documented, fetched 2026-09-10] · decisions: ADR-0010
GIVEN any dispatch on a subscription-authenticated account
WHEN the command line is assembled
THEN `--bare` never appears in argv, under any card, pack or configuration.
*Verified by:* a property test asserting the token is absent from every argv the builder can produce across the
configuration matrix (the matrix, not a grep, is the instrument — a plain lint over today's source passes vacuously);
plus a lint as a cheap second line. This is a standing guard, not a reproduction of an observed defect.

**AC-217** — `rk-adapter-claude` · evidence: charter §6 [documented]; ADR-0083's `set_var`/`remove_var` ban (the earlier
Verified-by read as a forbidden `set_var`) [documented] · decisions: ADR-0010, ADR-0099 (the captured environment is a
value in `Config`)
GIVEN any dispatch, where the child environment is **the captured parent environment minus a committed denylist**
(`rk-adapter-claude/src/env_denylist.rs`: `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_BASE_URL`, the
`CLAUDE_CODE_USE_*` vendor-routing variables and every other name the file lists — decision #16 of 2026-09-11, inherit
minus denylist rather than an allowlist, because Claude Code reads many undocumented variables: proxy, locale, `TERM`,
editor)
WHEN the child environment is constructed
THEN the adapter asserts no denylisted name is present in the captured environment, and a denylisted name found there
fails the dispatch as a typed configuration error rather than being stripped and passed through
AND the adapter never reads the process environment itself — it receives the captured map as a value in `Config`.
*Verified by:* a unit test constructing a `Config` whose captured environment carries the key, asserting the typed error
and a dispatch count of 0 (no `std::env::set_var` anywhere — ADR-0083); an assertion on the constructed child env map
that no denylisted name is present; the `doctor` row (AC-422) and this test read the **same committed list**.
*(Amended 2026-09-11, PIA-060.)*

### 7.6 Errors, headroom and cancellation

**AC-218** — `rk-adapter-claude` · evidence: proposal §4.3 error contract [documented]; F21 and `spikes/results/e3.jsonl`
— seven recorded `is_error` frames (slugs `fix-doctor-git-config-repair`, `issue-122` §2, `issue-130`, `issue-131`,
`issue-72` for 529; `issue-82` for 500) [measured, PIA-075] · decisions: ADR-0003, ADR-0095, ADR-0112
GIVEN a result stream carrying `is_error: true` or an `api_error_status`
WHEN the process exits **with any code**
THEN the dispatch resolves to `StepError::Platform { status, retry_after }` — the exit code is not consulted to overturn
it. (ADR-0095's table carried no `Platform` row — nor `Unsupported` (AC-238) nor `Overage` (AC-221); §0.3 row C5
recorded the gap and **ADR-0112** closes it: all three are variants, eleven in all.)
*Verified by:* unit test on the four-way matrix (exit 0/non-zero × error flag set/unset); the exit-0-plus-`is_error` cell
must be an error, never `Ok`. The exit-0-with-failure *pattern* is measured (F20: exit 0, `subtype: success`, no output),
and the `is_error` / `api_error_status` frames are the seven recorded in `e3.jsonl` at the corpus pin — the must-fail
fixture for the matrix and for the runbook's `spoke()` rule — not a fixture of our own construction. *(Amended
2026-09-11, PIA-075: "may not exist" struck; the GIVEN's "exits 0" widened to any exit code.)*

**AC-219** — `rk-adapter-claude` · evidence: F1 [measured: headroom readable via top-level `rate_limit_event`;
`result.rate_limit` is null; no Opus/Sonnet family sub-limit key] · decisions: ADR-0003
GIVEN a completed dispatch
WHEN the adapter records headroom
THEN the values come from the top-level `rate_limit_event`
(`unifiedWindows.{five_hour,seven_day}.utilization`) and `result.rate_limit` is never treated as a source.
*Verified by:* unit test on a recorded stream where `result.rate_limit` is null and the event carries values — the event
must be recorded; a lint that `result.rate_limit` is not read.

**AC-220** — `rk-adapter-claude` · evidence: F6 [measured: `utilization` quantised to 0.01; five_hour and seven_day
proven independent], F1 [measured: no family sub-limit key], charter §6 · decisions: ADR-0003
GIVEN the last recorded `five_hour` utilisation places the account within the configured margin of the limit
WHEN a new dispatch is requested
THEN the dispatch is refused with `StepError::RateLimited { resets_at }` before spawn, and the margin is at least two
instrument ticks (≥0.02 at the measured 0.01 resolution).
*Verified by:* unit test over a margin table including the 0.02 boundary; a config lint that rejects any margin below two
ticks; a `doctor` row showing the current margin and last reading.
The two-tick floor is [inferred] from F6's measured 0.01 resolution, not measured. Per F1 the binding limit is invisible,
so `five_hour` is a proxy: this criterion bounds the *margin rule*, and does not claim overage is thereby prevented
(see SM-065's kill).

**AC-221** — `rk-adapter-claude` · evidence: F1 [measured: the `rate_limit_event` exists and is the only headroom source;
its recorded payload carries `isUsingOverage: false`, `overageDisabledReason`, `overageStatus` — `spikes/FINDINGS.md`
:42–48 — and the runner records the same field as `is_using_overage` on every row it captured: `false` on all 20
E0-headroom rows (`spikes/results/e0-headroom.jsonl`) and all 60 S-VAR rows (`spikes/results/s-var.jsonl`), plus the
two camelCase rows of `instrument-checks/e1-smoke.jsonl` [measured, grep 2026-09-11]]; charter §6 · decisions: ADR-0003, ADR-0112,
ADR-0110 (AC-220's two-tick margin rule is the companion SM-006's kill now reads with this criterion) · **[surface
unverified — no true-valued `isUsingOverage` instance in the recorded runs]**
GIVEN a `rate_limit_event` whose `rate_limit_info.isUsingOverage` is `true`
WHEN a dispatch is requested
THEN it is refused as `StepError::Overage` before spawn.
*Verified by:* unit test on the refusal against the **recorded** false-valued payload with the boolean flipped to `true`
(the only construction the record licenses). **Correction (2026-09-11, audit PIA-065).** The field name *is* recorded —
the 2026-09-08 refutation struck it as fabricated because the condensed brief omitted the keys F1's own payload carries.
What has never been observed is a `true` value — the runner's `is_using_overage` is `false` on all 20 E0-headroom rows
and all 60 S-VAR rows it captured [measured, grep 2026-09-11] — so the tag narrows to "no true-valued instance in the
recorded runs"; the fixture is the recorded envelope, not one of our own construction. (An earlier draft of this note
said the two files do not carry the key: they carry it in the runner's snake-case spelling, and that sentence is
withdrawn.) The companion rule that `Overage` is never retried is AC-234, not here. SM-006's kill re-points here (with
AC-220's margin rule); S-RETRY is the named consumer of the first true-valued reading.

**AC-222** — `rk-kernel` / `rk-adapter-claude` · evidence: F22 [measured, operational: ~12 kills vs one 12,000 s survivor],
brief §C.6, principle 3 · decisions: ADR-0003
GIVEN a dispatch refused or failed with `StepError::RateLimited { resets_at }`
WHEN the kernel applies its retry policy
THEN a durable `retry_at(resets_at)` mark is written and the process exits; no sleep, timer or live wait spans the reset.
*Verified by:* unit test asserting process exit and a written mark; a lint that the retry path contains no sleep for the
`RateLimited` class; a replay assertion that the next `run` resumes from the mark.

**AC-223** — technology pack / `rk-adapter-claude` · evidence: timeout history [measured: 900 → 2,400 → 3,600 → 5,400 s],
F31/S-TYPED [measured: CLEAN review median 1,071 s, slowest 4,561 s], F21/E3 [measured: CLEAN 317–2,055 s] · decisions: ADR-0003
GIVEN an activity class with a recorded wall-clock distribution in the event log
WHEN its pack declares a timeout
THEN the declared timeout is at least the recorded slow-condition figure for that class (its recorded maximum, not its
median), and a smaller value is a lint error at configuration time.
*Verified by:* a pack lint reading the recorded distribution from the event store; a fixture pack with a 900 s timeout
against the S-TYPED class that must fail.
The recorded maximum is a moving floor at small n (S-TYPED's 4,561 s is one draw of 51): the lint enforces the rule, it
does not certify the number is the true tail.

**AC-224** — `rk-adapter-claude` · evidence: F22 [measured, operational; one machine, one operator, cause unestablished],
brief §C.6 · decisions: ADR-0003, **ADR-0099** (containment per OS; the `CancelToken`)
GIVEN a dispatch that is cancelled by the `CancelToken` the composition root owns (the CLI's signal path or the
per-activity timeout — the kernel never calls cancel)
WHEN the adapter terminates the child
THEN the whole containment group is killed and no descendant of the dispatch survives the call — on **Windows** the
group is a Job Object created before spawn with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, the child assigned before it can
fork and the handle held for the child's lifetime (the `unsafe` FFI in one module of `rk-adapter-claude` under a
commented `#![allow(unsafe_code)]`, ADR-0102); on **POSIX** it is the process group from `CommandExt::process_group(0)`,
killed with `killpg`.
*Verified by:* an integration test (`cargo test -p rk-adapter-claude -- cancel_reaps_process_tree`) spawning a fixture
that forks a grandchild **and a detached/breakaway grandchild** (so a tree walk from the child's PID would miss it),
cancelling, then enumerating the group via the job (Windows) or the pgid (POSIX) and asserting no live PID; run on
both CI targets as a regression against F22's 12,000 s survivor. *(Amended 2026-09-11, PIA-010/PIA-059: "process group"
never held on Windows — the lifted runner does `taskkill /T /F` — and a tree-walk fixture passed on the behaviour it
forbids.)*

**AC-225** — `rk-adapter-claude` · evidence: platform docs (SIGTERM → 143, no result message — a POSIX observation)
[documented] · decisions: ADR-0003, **ADR-0099**, ADR-0095
GIVEN a dispatch terminated by the adapter's own cancel (the `CancelToken` fired)
WHEN the child exits with no result message — exit 143 on POSIX; the adapter's own `TerminateProcess` code on Windows,
where 143 never obtains
THEN the outcome is `StepError::Cancelled { by: signal | timeout | operator }` and never `NonZero(..)`, `Timeout` or
`Schema` — the classifier is keyed on the **cancel token having fired**, not on the exit code, and the exit code is
recorded per platform.
*Verified by:* unit test on the classifier for the cancel path on each platform (token fired + platform exit code →
`Cancelled`); a negative test that an *externally* killed child (token not fired, same exit code) does not classify as
`Cancelled`. *(Amended 2026-09-11, PIA-010: "exits 143" was the WHEN, and it is unobservable on the primary platform.)*

**AC-523** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-adapter-claude` ·
evidence: F1 [measured: `rate_limit_event` carries `resetsAt` per window and `status`; `resets_at` arrives on the event,
never on a failure result]; the charter's "no documented exit code or structured error subtype for usage-limit
exhaustion; `resets_at` from `rate_limits` is the practical retry-after" [documented]; the lifted runner's
`detect_usage_limit(stderr, result)` (`spikes/runner/src/runner.rs:486`) as the prior art for the text fallback
[measured] · decisions: **ADR-0095** decision 2 (the `RateLimited` row), ADR-0110
GIVEN a child that terminates with a usage-limit result — the structured `subtype` first; a text marker on stderr or the
result only as a **disclosed fallback**
WHEN the adapter classifies the result
THEN the dispatch resolves to `StepError::RateLimited { resets_at: Option<Timestamp> }`, with `resets_at` taken from the
last `rate_limit_event` in the same stream (the window whose `status` is not `allowed`), else `None` so the kernel falls
back to the pack-declared delay (AC-030's degradation)
AND the detection surface is recorded on the `rate_limited` event (`detected_by: subtype | text_marker`), and a text-marker
detection is never silent
AND the retry class is "wait until `resets_at`, else the pack delay; never backoff" — no attempt of the activity budget
and no exponential delay is spent on it (an adapter that maps a usage-limit result to backoff is forbidden by ADR-0095).
*Verified by:* a unit test over a recorded stream fixture (a `rate_limit_event` with a non-`allowed` status followed by a
usage-limit result) asserting the variant, the `resets_at` value and `detected_by: subtype`; a second fixture with the
structured subtype absent and the text marker present, asserting `detected_by: text_marker` on the event; a table-driven
test on the retry classifier asserting zero backoff attempts for `RateLimited`; an assertion that the argv-length guard
(the command builder refuses before spawn with a typed configuration error at the cap — decision #18 of 2026-09-11;
fixture at the cap) is on the same builder path. **S-RETRY's registered question extends to exhaustion-on-result**, not
only the throttle margin, with `rk-adapter-claude` as consumer. `--system-prompt-file` for cards over the argv cap is
**S-SPF** (registered, unused in Phase 1: whether the file route shares the inline route's cache prefix at the pinned CLI
is unmeasured, and AC-384/SM-113's 18,603-token flat-prefix promise rests on the inline route).

### 7.7 Version pinning and fan-out

**AC-226** — `rk-adapter-claude` · evidence: F1, F4 [measured: two events we depend on are undocumented],
R8 [documented: no published stability policy] · decisions: ADR-0003
GIVEN a child whose `system/init` reports a CLI version different from `TESTED_CLI_VERSION`
WHEN the adapter validates the child
THEN the configured outcome for a mismatch fires — either refusal, or a recorded `Degraded`-style allowance that the
configuration explicitly opted into — and the test asserts the *configured* one, not merely that some typed outcome
occurred.
*Verified by:* unit test with a scripted `system/init` at a higher and a lower version, run under both configurations,
each asserting its own outcome; a `doctor` row showing pinned vs detected version at zero model cost.

**AC-227** — `rk-adapter-claude` · evidence: R8 [documented], F1/F4 [measured: undocumented event shapes already in use]
· decisions: ADR-0003
GIVEN any harness event written to the event store
WHEN it is persisted
THEN it carries the CLI version the dispatch actually ran against.
*Verified by:* a replay assertion over a full run that no harness event lacks the version field; a schema requirement
(non-optional field) on the event type.

**AC-228** — PROVISIONAL (no fan-out spike ran) — `rk-adapter-claude` · evidence: F13 [measured: prefix ~17,439 shared +
~7,827 per-directory; 19/19 fresh repos identical]; code.claude.com/docs/en/workflows and /prompt-caching — the
vendor's mechanism is **hold all but the first sibling until the first response begins, capped at
`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS` (default 5000 ms), then release the held siblings together**; platform.claude.com
says only "wait for the first response before sending subsequent requests", with no number [documented, fetched
2026-09-10; grounding EXT-ANTHROPI-005 — the earlier "platform docs" citation was wrong]; R7 [documented] · decisions:
ADR-0003
GIVEN two or more sibling dispatches resolved in the same fan-out that share a stable prefix digest
WHEN they are launched
THEN the first is dispatched alone and the rest are held until its first response begins or the configured cap (~5 s)
elapses, then released together; dispatches with differing digests are not held.
*Verified by:* unit test on the scheduler asserting the hold-until-first-response-or-cap release for equal prefix
digests, and no hold for differing digests. **Block:** the ~5 s cap is documented, not measured here — no fan-out spike
ran; the *cap* is provisional, the *ordering property* is not. That the hold causes exactly one sibling to pay the cold
write is [inferred] from F13 and is **not** what the test checks. *(Amended 2026-09-11, PIA-057: the per-sibling
interval chain was not the vendor's mechanism; AC-388 carries the configurable shape SM-114's A/B needs.)*

### 7.8 Typed errors and the reviewer distinction

**AC-229** — `rk-kernel` · evidence: proposal §4.3 error contract, RK-B0013 [documented]; the variant set is the table
in **ADR-0095 decision 2** — `Transport { kind, detail }`, `Schema { attempts, last_error }`, `Timeout { after_secs }`,
`Cancelled { by }`, `NonZero { code, stderr_digest }`, `RateLimited { resets_at }`, `NotAdopted { expected_agent,
observed }`, `NotIsolated { surface, diff_digest }` — each with its retry class, its `spoke()` value and its AC-360 event
arm; step.rs's six-variant set and the superseded proposal's eleven are both withdrawn [documented] · decisions:
ADR-0003, **ADR-0095**
GIVEN any failure path in the harness adapter
WHEN it surfaces to the kernel
THEN it is exactly one of the eight closed `StepError` variants of ADR-0095's table and no stringly-typed or catch-all
error crosses the port boundary; any further variant is a successor to that record.
*Verified by:* an exhaustive `match` over `StepError` in the kernel with no wildcard arm (compile-time); a lint that the
port trait's error type is the enum and not a boxed dynamic error; a snapshot of the variant list against the table.
*(Amended 2026-09-11, PIA-005: `StepError` had never been enumerated in a governing document.)*

**AC-230** — `rk-kernel` · evidence: F20 [measured: the mechanism behind a consumer that fails open], brief §C.7,
GATE-01, RK-B0013, principle 4 [documented] · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a review dispatch that returned `Err(StepError::…)` — the reviewer never spoke
WHEN the gate policy evaluates the activity
THEN no findings are synthesised, the gate does not close, and the outcome is the error — absence of evidence is a block.
*Verified by:* unit test on the gate evaluator for each `StepError` variant, all of which must block; a type-level check
that the findings consumer takes `Findings`, never `Option<Findings>` or a defaulted empty value.

**AC-231** — `rk-kernel` · evidence: F29 [measured: 60 draws, 19/19 rejections on-target, class means 0.70/0.37/0.10/0.00
— no defect class is stable-reject], brief §C.7 · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a review dispatch that returned `Ok` with an empty findings list — the reviewer spoke and found nothing
WHEN the gate policy evaluates the activity
THEN that outcome is distinguishable in the record from AC-230's error case, and only this one may satisfy a policy that
requires the reviewer to have *run*.
*Verified by:* unit test asserting the two cases produce different recorded outcomes and different gate decisions; a
replay assertion that the event log discriminates them without reading prose.
Per §C.7 and F29, "the reviewer ran and found nothing" does **not** license closing an AC: the gate is high-precision,
low-recall and is used to ADD findings, never to CLEAR one. This criterion establishes the distinction only.

### 7.9 Layered retry, budgets and degradation

**AC-232** — `rk-adapter-claude` · evidence: proposal §4.3 (`system/api_retry` with `attempt`, `max_retries`,
`retry_delay_ms`, `error`) [documented]; the same fields **captured on 139 rows in 12 result files** (`api_retries` in
`spikes/results/e3.jsonl` and eleven others) [measured, recount 2026-09-11 — see AC-026] · decisions: ADR-0003
GIVEN a stream containing `system/api_retry` events
WHEN the adapter processes them
THEN each is recorded and counted against the activity's attempt budget, and the adapter issues no retry of its own for
those attempts.
*Verified by:* unit test on a **recorded** fixture stream (an `e3.jsonl` row with two inner retries, at the corpus pin)
asserting budget decremented by two and exactly one child spawn. *(Amended 2026-09-11, PIA-068: the `[surface unverified]`
tag and the "not met until confirmed against one recorded real stream" clause are withdrawn — the shape is confirmed
against 139 recorded rows; the fixture is recorded, not synthetic.)*

**AC-233** — `rk-kernel` · evidence: proposal §4.3 outer policy [documented] · decisions: ADR-0003
GIVEN a `Transport`, `Platform` 5xx or `Timeout` error within the pack's attempt budget
WHEN the kernel retries
THEN the delay is exponential with full jitter (base 2 s, cap 5 min) and the attempt count never exceeds the pack's N.
*Verified by:* unit test with a seeded RNG asserting the delay envelope and the attempt ceiling; a property test that no
delay exceeds the cap. Base, cap and N are design parameters from §4.3, not measured values.

**AC-234** — `rk-kernel` · evidence: proposal §4.3 [documented], F7/F8/F20 as the measured configuration-class failures
[measured] · decisions: ADR-0003, ADR-0095 (retry classes per variant; note that `Unsupported` and `Overage` are named
here and in AC-027/AC-221/AC-238 but are absent from ADR-0095's eight-row table — §0.3 row C5)
GIVEN a `NotAdopted`, `NotIsolated`, `Unsupported` or `Overage` error
WHEN the kernel applies its retry policy
THEN it is never retried — these are configuration, not weather — and the activity fails immediately.
*Verified by:* an exhaustive test of the retry classifier over every `StepError` variant, asserting no-retry for these
four; the test must fail to compile if a variant is added without a classification.

**AC-235** — `rk-kernel` · evidence: F20 [measured: the failure this policy responds to], proposal §4.3 [documented:
the retry count itself] · decisions: ADR-0074 (supersedes ADR-0004), ADR-0016
GIVEN a `Schema` failure on a dispatch
WHEN the kernel retries
THEN it re-prompts exactly once carrying the validation error, and a second failure ends the activity as
`Schema { attempts: 2 }`.
*Verified by:* unit test asserting exactly two spawns and the recorded attempt count; a negative test that no third
spawn occurs. "Exactly once" is a design parameter: the brief contains no measurement of schema-retry success, and
F31's 52/52 conformance was measured with `StructuredOutput` present, i.e. on the path where this retry never fires.

**AC-236** — `rk-kernel` · evidence: F15 [measured: effort is applied per dispatch and separates at xhigh],
proposal §4.3 degradation row [documented] · decisions: ADR-0003, ADR-0074 (supersedes ADR-0004)
GIVEN a dispatch that was degraded to a different model than the **resolved (bound) card** declared (the tier is the
role's, materialised into the dispatch card by the binder — §0 glossary; PIA-015)
WHEN a gate whose policy demands the original tier evaluates the result
THEN **both** of the following hold: the gate does not close, **and** the event and digest carry `Degraded { from, to, why }`.
*Verified by:* unit test on the gate evaluator with a degraded result (the gate half); a replay assertion that every
degraded dispatch has the triple recorded (the record half); a lint that degradation is only reachable when the pack
declares it. A pass requires all three — a green gate test alone does not satisfy this criterion.
No claim is made here that a lower tier produces worse review output: the brief measures no model-tier contrast
(F15 measures *effort*; F31's +74% and F29's class means are contract-format and defect-class effects, not tier effects).

**AC-237** — `rk-kernel` · evidence: proposal §4.3 circuit-breaker row [documented], brief §C.6 [measured: nothing may
depend on a process surviving] · decisions: ADR-0003
GIVEN K consecutive `Platform` failures on one model family
WHEN a further dispatch for that family is requested
THEN it is refused behind a durable mark while dispatches for other families continue unaffected.
*Verified by:* unit test driving K failures on family A and asserting refusal for A and success for B; a test that the
mark survives process exit (the breaker is not in-memory state). K is a design parameter — the brief records no
measured `Platform` failure run to derive it from.

### 7.10 Capabilities and the second harness

**AC-238** — `rk-kernel` / harness port · evidence: F7, F8 [measured: capability differences are observable from the
child's own `system/init`], L4.3/L5.7 [documented] · decisions: ADR-0003, ADR-0007
GIVEN an activity requiring a capability the selected adapter's registry does not record as verified
WHEN the kernel resolves the dispatch
THEN it fails with `StepError::Unsupported(capability)` derived from the registry entry
(`{capability, verified_on, probe}`), and the kernel contains no adapter-identity branch.
*Verified by:* unit test with a registry missing the capability; a lint over the kernel crate for adapter-name comparisons
(`if adapter == …`), which must find none.

**AC-239** — PROVISIONAL (S-CODEX not reported in the brief) — `rk-adapter-codex` · evidence: E3 is Claude-only
(proposal §2, "quality on a second harness"); ADR-0007's evidence list names S-CODEX · decisions: ADR-0007, ADR-0074 (supersedes ADR-0004)
GIVEN the gate-eligible adapter list in Phase 1
WHEN a gate evaluates any result
THEN the gate may close only on a result from an adapter on that list, which is data and has exactly one entry in
Phase 1; a Codex-sourced result therefore cannot close a gate — the second harness is validation-only until an E3-rig
run on it exists.
*Verified by:* a lint/unit test that the gate-eligible adapter list is data with exactly one entry in Phase 1 (checkable
today, with no Codex adapter present); plus, **once `rk-adapter-codex` exists**, a unit test on the gate evaluator with a
Codex-sourced result. **Block:** S-CODEX has no reported result in the evidence brief — it is not even in the brief's
unrun list — so the criterion encodes the *restriction* only; criteria for Codex's own contract mapping, and the
`codex exec --json --output-schema <file> -o` invocation shape, wait on that run and are unverified here. *Phase
(2026-09-11, ADR-0094 decision 2):* the second Harness adapter is **Phase 2**, behind the CI decision ADR-0010 defers, as
a named open item with `rk-adapter-codex` as consumer; the list-is-data half of this criterion is checkable in Phase 1
and belongs in the runbook's 7b/7d acceptance sets (PIA-072), with only the Codex-present half excluded.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-061 | Child-verified isolation | saturating |
| SM-062 | Structured-output fail-open count | — |
| SM-063 | Dispatch outcome class mix and void rate | — |
| SM-064 | Static-prefix cache stability | — |
| SM-065 | Headroom draw per dispatch and margin refusals | — |
| SM-066 | Wall-clock against configured timeout | — |
| SM-067 | Cost per dispatch by role | — |
| SM-068 | Cancellation cleanliness | — |
| SM-069 | CLI version drift | no baseline |
| SM-070 | Pre-spawn refusals | no baseline |

#### Residual risk this area could not repair

- Five criteria (AC-210, AC-215, AC-216, AC-221, AC-232) rested on CLI surface names that appear nowhere in the evidence brief. As of 2026-09-11 every name is documented or recorded (scope note above): AC-210's variables are on the vendor's prompt-caching page, AC-215's flag shipped at 2.1.259, AC-216's `--bare` is still API-key-only, AC-232's payload is captured on 139 rows, and AC-221's field is in F1's own payload. What remains is that no `true`-valued `isUsingOverage` has ever been observed (AC-221, AC-378), so the overage refusal is still verified against a flipped recording rather than a recorded overage.
- The `StepError` set is now ADR-0095's eight-row table; the adapter's Windows containment (AC-224) is one `unsafe` FFI module, and a regression there is caught only by the detached-grandchild fixture on the Windows CI target — a fixture that must actually run on `windows-latest`, not be skipped as environment-dependent.
- AC-236 remains compound (the gate must not close AND the `Degraded {from,to,why}` triple must be recorded) — the range is fully allocated and reusing the retired AC-207 would plant a dead anchor. A build satisfying the gate half and skipping the record half can be reported as green by a careless reader.
- AC-214, AC-216, AC-227 and AC-239 are negative or schema-shaped guards a system doing nothing also satisfies. A runbook counting them toward coverage overstates assurance by four.
- The evidence base is thin in a way no wording repair fixes: F10 is n=1 per gap with the TTL boundary bracketed (45.1, 61.6) and never located; F28/S-H1b's 66-min readings are n=1 per cell and the 24 h static read was never taken; F22 is one machine, one operator, cause unestablished.
- AC-234's compile-fail-on-new-variant clause is the strongest single instrument here and AC-221 and AC-229 now lean on it. If `StepError`'s exhaustiveness test is weakened, four criteria degrade silently at once.
- AC-223's floor is the recorded maximum from an event store that starts empty, so on a fresh class the constraint is satisfied by the first draw — exactly the window in which the 900 s defect occurred.
- AC-239's block is worse than it reads: S-CODEX does not appear in the brief's unrun list either, so there is no evidence the spike is even registered.

---

## 8. Human gate — a durable mark, not a waiting process

**Area key:** `human-gate` · **Criteria:** AC-240..AC-279 **plus AC-524** (AC-255 cut; AC-277..AC-279 are splits; AC-524 is allocated outside the range — see §0.4) — **40 in force** · **Measures:** SM-073..SM-082 (SM-083, SM-084 unallocated)
**State:** 33 binding · 4 provisional (AC-266, AC-267, AC-271, AC-524 — added 2026-09-11, PROVISIONAL on P8-B) · 3 human-gated (AC-273, AC-275, AC-276)
**Sources:** proposal §4.6 (human gate), §4.5 (resume entry point), §9 D9/D17/D22; ADR-0009, ADR-0017, ADR-0022, ADR-0038; evidence brief F10, F18, F19, F20, F22, F28, F29, F36, F37, F38 and §C.1–C.6, §C.12.

*Scope note.* **Expiry was unobservable in the corpus, not observed-absent.** The gates in F18/F19 had no deadline mechanism, so no gate *could* expire; the longest was answered at 226 h. Every expiry criterion rests on design and prior art, and the corpus supplies no expiry base rate in either direction.

### 8.1 The mark itself — opening a gate is a write and an exit

**AC-240** — `rk-kernel` · evidence: F22 · decisions: ADR-0017
GIVEN an activity or stage reaches a human gate
WHEN the kernel opens the gate
THEN the opening invocation terminates with exit code 0 and control returns to the caller without waiting for an answer.
*Verified by:* a CLI integration test that opens a gate and asserts the invocation exits 0 within a bounded wall-clock
budget; a `gate_opened` event written before that exit.
*Why:* F22 [measured] — ~12 background jobs died against one 12,000 s `Start-Process` survivor, and `issue-81` was killed
three times as a background job but completed in 116 s in the foreground; cause never established. Nothing may depend on a
process surviving (brief §C.6).

**AC-278** — `rk-kernel` · evidence: F22 · decisions: ADR-0017
GIVEN the kernel has opened a gate and the opening invocation has exited (AC-240)
WHEN the process table is examined after that exit
THEN no child, background, detached or daemon process belonging to that run remains alive.
*Verified by:* an integration test that enumerates the descendants of the run's job object / process group after exit and
asserts the set is empty; an architecture check that the gate path contains no detached-spawn call.
*Why:* split from AC-240 because exit 0 does not imply it — F22's one *survivor* [measured] was exactly a detached
`Start-Process` that outlived its parent by 12,000 s. A clean exit that leaks a detached child is the specific failure
this criterion catches and AC-240 cannot.

**AC-241** — `rk-kernel`, `rk-git`, `rk-store` · evidence: F38 [motivating, not confirming] · decisions: ADR-0017
GIVEN a gate is being opened, which writes both a `Gate:` trailer and a `gate_opened` event
WHEN the process is killed at any point during that open
THEN a later `redkiln run <item>` observes either both the trailer and the event, or neither — never a trailer with no
event or an event with no trailer.
*Verified by:* a crash-injection test that kills the process at each enumerated write boundary (a fault-point
enumeration, not a sleep race), asserting the two-sided invariant at every point; a `doctor` rule that reports any
trailer/event orphan pair.
*Why:* F38 [measured, n=3] showed mid-graph state surviving `taskkill /T /F` with 0 lost, 0 changed and identical
digests. That is the *motivation* only: P8 killed at three arbitrary instants and never enumerated write boundaries, so
the two-sided invariant here is [inferred] from the design and is established by this test, not by P8.

**AC-242** — `rk-kernel` · evidence: F22, F28 · decisions: ADR-0017
GIVEN the kernel's gate module and its direct helpers
WHEN the crate is built
THEN the build fails if that code contains a call to `thread::sleep`, a blocking channel receive, or a timer/scheduler
spawn on the gate-lifecycle path.
*Verified by:* a `cargo`-level deny-list / clippy lint over the named modules, failing the build rather than a test run;
a fixture module containing each forbidden construct that must fail the lint.
*Why:* negative criterion. F22 [measured] shows a waiting process is not a durable thing; F28 [measured] shows a gate
costs a flat ~18.6k tokens however long it lasts, and a waiting process would reintroduce the cost the mark design
removed. Scope limit: this is a **syntactic** check over named modules — it cannot prove no code path anywhere blocks,
only that these modules do not do it directly. The behavioural half is AC-240/AC-278.

**AC-243** — `rk-kernel`, `rk-adapter-claude` · evidence: F28, F10, F18 · decisions: ADR-0017
GIVEN a gate is open
WHEN any amount of wall-clock time passes, including past the ~1 h prompt-cache TTL
THEN zero harness dispatches are issued for that item until the gate reaches a terminal state.
*Verified by:* a replay assertion over the event log — no `dispatch` event with a timestamp between `gate_opened` and the
gate's terminal event for the same item; a regression test on the P8 fixture asserting the dispatch count across an open
gate is 0.
*Why:* negative criterion, and the obvious future regression. F10 [measured, n=1 per gap] puts the TTL at ~1 h (full hit
at 45.1 min, 0.690 share at 61.6 min; the boundary is bracketed, never located); F18 [measured] puts 45–48% of gates over
1 h. A "keep the cache warm" optimisation is the tempting wrong answer, and F28 [measured] proves it buys nothing:
18,603 tokens at 65.2 min and at 49.7 h alike.

### 8.2 The record — (token, tree_hash, deadline)

**AC-244** — `rk-kernel`, `rk-types` · evidence: — · decisions: ADR-0017
GIVEN a request to open a gate whose record lacks a token, a tree hash, or a deadline
WHEN the kernel processes it
THEN the gate is not opened and the run fails with a configuration error, leaving no trailer and no event.
*Verified by:* a type-level test (the gate record is unconstructible without all three fields) plus a runtime test on the
deserialisation path for records read back from the store; a `doctor` rule over existing open gates.
*Why:* ADR-0017 [documented] makes the triple the record. Its rejected alternative "item-keyed answer" is exactly what an
optional token degrades into. No spike bears on this; the criterion is decision-derived.

**AC-245** — `rk-kernel` · evidence: F19 · decisions: ADR-0017
GIVEN a gate is opened and a tree is rendered into the gate prompt/attachment the human reads
WHEN `gate_opened` is written
THEN the recorded `tree_hash` is the hash of that same tree object, not of HEAD's commit, not of the index at a later
instant — derived by **one algorithm, written once, used at gate open and at answer** (ADR-0105 clause 4): when a
checkpoint commit is due, the trailer commit's tree; otherwise `git write-tree` on a temporary index built from the
current index plus tracked modifications, **untracked files excluded**, through the wrapper (the op is on its write-side
list).
*Verified by:* a unit test asserting the rendered tree and the recorded hash resolve to one object id; a property test
over dirty-worktree fixtures, including one with an untracked file (whose presence must not change the hash) and one
with a tracked modification (whose presence must). *(Amended 2026-09-11, PIA-050: the hash of an empty commit over a
dirty worktree was undefined.)*
*Why:* ADR-0017's decision is "the tree hash the human saw". A hash of anything else makes the void rule (AC-249) check a
claim nobody made. F19 [measured] found drift on 12/174 gates, so the difference is reachable in normal operation.

**AC-246** — `rk-kernel` · evidence: — · decisions: ADR-0017
GIVEN a gate whose token has already been consumed by an applied answer
WHEN a second answer presents the same token
THEN the second answer is refused as spent, records a distinct event, and changes no item state.
*Verified by:* a unit test on the answer path; a replay assertion that no item's state advances twice on one token.
*Why:* ADR-0017 [documented] — "a single-use token". Evidence line is deliberately empty: F38's second-arrival result is
**not** evidence here, because P8 §5.4(3) is circular by construction (arrivals forced, the mark minted by the driver
that implements the semantics under test).

**AC-247** — `rk-kernel` · evidence: — · decisions: ADR-0017
GIVEN two open gates A and B on the same or different items
WHEN an answer carrying A's token is submitted against B
THEN it is refused and B remains open.
*Verified by:* a unit test over a two-gate fixture; a fuzz test over token/gate pairings.
*Why:* separate behaviour from AC-246 (replay of a spent token vs. use of a live token against the wrong gate). No
evidence cites this directly; it follows from ADR-0017's single-use token being the identity of one gate instance — if it
is not scoped, "single-use" is not a property of the gate.

**AC-248** — `rk-kernel`, `rk-cli` · evidence: — · decisions: ADR-0017
GIVEN an answer that names only an item (no token)
WHEN `reconcile()` processes it
THEN it is refused, and no open gate on that item is matched by item identity.
*Verified by:* a unit test on the answer path; a CLI test that `redkiln answer <item> approve` without a resolvable live
token exits non-zero with a named outcome.
*Why:* negative criterion. ADR-0017 explicitly rejects the item-keyed answer: item identity alone does not detect drift.

### 8.3 Void — an answer against a tree that moved

**AC-249** — `rk-kernel` · evidence: F19 · decisions: ADR-0017
GIVEN an open gate recorded with `tree_hash` T
WHEN an answer arrives and the item's current tree hash is not T
THEN the answer resolves **void**: a `gate_answer_void` event is written and no approve/changes-requested decision is
applied to the item.
*Verified by:* a unit test over a fixture that commits between open and answer; a replay assertion that void events never
precede a state transition for that gate.
*Why:* ADR-0017 [documented] — void, not reconciled. F19 [measured] — drift 12/174 = 6.9% overall (Wilson 4.0–11.7%),
9/79 = 11.4% on gates over 1 h. So this path is **reachable in normal operation at roughly 1 gate in 14**, not a rare
corner — but note the brief characterises the same corpus as "the world moves rarely and predictably", so do not read
this as a hot path either.

**AC-250** — `rk-kernel` · evidence: — · decisions: ADR-0017
GIVEN generated interleavings of (commit, answer, resume, expire) over a gated item
WHEN the event log is replayed
THEN in no generated interleaving does an answer whose tree hash mismatched at arrival cause the item to advance
AND the codebase contains no merge, re-application or deferred-reconciliation path that consumes a `gate_answer_void`
event as an input to a state transition.
*Verified by:* a property/fuzz test over generated interleavings asserting the invariant on the terminal state (not a
single example test); plus an architecture test that no consumer of `gate_answer_void` writes a state transition.
*Why:* negative criterion, and the strongest one in this area. ADR-0017's rejected option is literally "reconcile stale
answers". Scope limit made explicit in the THEN: a fuzz test cannot establish "there exists no sequence of inputs" — the
universal half of the claim is carried by the architecture test over the code, not by the generator.

**AC-251** — `rk-kernel` telemetry · evidence: F29, F36, F37 · decisions: ADR-0017
GIVEN a set of gate answers containing voids
WHEN approve / changes-requested rates are computed
THEN voids are excluded from the numerator and denominator of those rates and reported in their own denominator.
*Verified by:* a unit test on the metric roll-up over a fixture containing voids; the same assertion in the `doctor`
report renderer.
*Why:* house rule — a void is never a miss. Stated **by analogy, not by measurement**: every spike that reported a rate
reported voids separately — S-VAR 60 draws 0 void (F29), P1 arm L 400 dispatches 0 void (F36), P7 61 runs / 297 records
0 void (F37) [all measured] — but a spike "void" is a *discarded draw*, a different quantity from a gate answer voided by
tree drift. What transfers is the reporting discipline, not the rate. The production metric must not be the first place
that discipline lapses.

**AC-252** — `rk-kernel` · evidence: F19, F18 · decisions: ADR-0017
GIVEN an answer that resolved void
WHEN the kernel next acts on the item
THEN a new gate is opened whose record differs from the voided gate's in all three fields — a fresh token, the current
tree hash, and a fresh deadline.
*Verified by:* an integration test asserting the re-opened gate's record differs in all three fields; a replay assertion
that a void is always followed by either a re-open or a terminal state, never by nothing.
*Why:* proposal §4.6 [documented] — "a drifted item re-opens the gate with the drift attached". F18/F19 [measured] make
this the path for roughly 1 gate in 9 among gates open over an hour (9/79, Wilson 6.1–20.3%).

**AC-277** — `rk-kernel` · evidence: F19 · decisions: ADR-0017
GIVEN a gate re-opened after a void (AC-252)
WHEN its record is written
THEN a non-empty drift attachment is present on it, referencing both the voided gate's recorded tree hash and the current
one.
*Verified by:* an integration test asserting the attachment is present and non-empty on the re-opened gate; a `doctor`
rule reporting any post-void re-open with no attachment.
*Why:* split from AC-252 — a re-open with three fresh fields and no drift attached satisfies AC-252 completely while
losing the only thing that tells the human why they are being asked twice. Content and determinism of the attachment are
AC-254.

**AC-253** — `rk-kernel` · evidence: — · decisions: ADR-0017
GIVEN two states of the same item
WHEN the kernel decides whether the item drifted
THEN the decision is a comparison of tree hashes only — never file mtimes, wall-clock elapsed time, commit counts, or
branch tip identity.
*Verified by:* a unit test over a fixture that rewrites history to an identical tree (drift must be **false**) and one
that touches files back to identical content (drift must be **false**); an architecture test that the drift predicate
reads no clock.
*Why:* negative criterion. ADR-0017 [documented] treats tree-hash binding as strictly stronger than a commit-SHA pin; a
commit-identity or timestamp test would be weaker in one direction and noisier in the other.

**AC-254** — `rk-kernel` · evidence: F19 · decisions: ADR-0017
GIVEN a re-opened gate carrying drift
WHEN the drift attachment is rendered
THEN it names the **commits since the trailer commit, and the paths differing between the two trees** (the recorded
`tree_hash` and the current one, both by AC-245's algorithm — ADR-0105 clause 4), and is byte-identical across two
renders of the same event log.
*Verified by:* a golden-file replay assertion (render twice from the log, compare); a unit test over a multi-commit drift
fixture.
*Why:* determinism is the checkable property and it is unconditional. **The size of the payload is not evidenced**: F19's
median in-gap commit count of 0 is a statistic over *all* gates, 93% of which did not drift at all, so it licenses no
claim about the commit count on the drifted subset — that number was never measured. The attachment must therefore be
exact rather than approximate on principle, not because it is known to be small.

### 8.4 Deadline and expiry — terminal, never a hang

**AC-256** — `rk-kernel`, `redkiln doctor` · evidence: F18 · decisions: ADR-0017
GIVEN any gate whose deadline has passed
WHEN `reconcile()` runs
THEN the gate is in exactly one of `fail`, `escalate` or `auto-decide` — the set of gates that are simultaneously
past-deadline and open after reconcile is empty.
*Verified by:* an exhaustive state-machine test over (expiry policy × arrival ordering) asserting the state after
reconcile is terminal; a `doctor` rule whose expected result set is empty and whose non-empty result is an error.
*Why:* negative criterion — "never a hang" (ADR-0017 [documented]). F18 [measured] shows the tail is real: stage→verdict
p90 ~9 h and max 226 h (9.4 days) over n=191. Nothing in that corpus expired **because nothing could** — there was no
deadline mechanism — so this criterion is the reason the "no hang" property will hold, not a repair of an observed
failure.

**AC-257** — `rk-kernel` · evidence: F22 · decisions: ADR-0017
GIVEN a gate whose deadline passes while no redkiln process is running
WHEN the next `redkiln run <item>` or `redkiln answer` executes
THEN the expiry is observed and resolved at that read, with no timer, cron entry, daemon or scheduled job required to
have been alive at the deadline instant.
*Verified by:* an integration test that opens a gate with a past deadline in a fresh process with no prior process ever
having run; the architecture test of AC-242 (no timer spawn on the gate path).
*Why:* F22 [measured] — a process that must be alive at a moment in the future is not a mechanism this system has.

**AC-258** — `rk-kernel` · evidence: F19 · decisions: ADR-0017
GIVEN an event log containing a gate open, an answer and a deadline whose orderings are fixed by recorded timestamps
WHEN the log is replayed twice under two different injected clocks and two different working directories
THEN the gate's terminal state and the item's next action are identical.
*Verified by:* a replay assertion in the kernel crate parameterised over an injected clock and an injected filesystem
root, asserting the replay reads no ambient host state.
*Why:* this is the instrument F19 explicitly lacks — H2/H3 proved the state carrier lossless at 174/174 [measured] but
**the FSM was never replayed**, so "same next action" is unproven (brief §C.4). The criterion is writable; the evidence
for it does not yet exist and this test is what creates it. Scope note: injected-clock and injected-root replay is what
the instrument covers; genuine cross-machine reproduction is not asserted because nothing here tests it.

**AC-259** — `rk-kernel` · evidence: — · decisions: ADR-0022, ADR-0038, ADR-0017
GIVEN a gate whose expiry policy is `escalate`
WHEN the deadline passes and reconcile resolves it
THEN the escalation is itself a kernel-owned `Gate:` mark with its own (token, tree_hash, deadline) — not an in-activity
wait, not a notification with no record
AND the escalation gate's class is fixed (`escalation`), whose expiry action may not itself be `escalate` — every chain
terminates in `fail` or an auto-decide within one hop (ADR-0105 clause 10; AC-260 lints it).
*Verified by:* an integration test asserting the escalation produces a second `gate_opened` event conforming to the same
record schema and carrying class `escalation`; a schema test that the escalation payload carries only a question id
(ADR-0038); an expiry test on the escalation gate asserting it resolves to `fail` or auto-decide and never to a third
mark. *(Amended 2026-09-11, PIA-050: escalate chains were unbounded.)*
*Why:* ADR-0022 [documented] — "an activity-internal HITL escalation is still a kernel-owned durable `Gate:` mark (§4.6);
the activity *declares* escalation points, the kernel *executes* them."

**AC-260** — `rk-kernel` policy table · evidence: — · decisions: ADR-0017
GIVEN a gate class in the policy table with no expiry action declared, or the `escalation` class with expiry action
`escalate`
WHEN the table is linted
THEN the lint fails — there is no implicit default expiry action, and no escalate-on-escalate (ADR-0105 clause 10).
*Verified by:* a lint rule over the policy table with a fixture table that must fail on the missing action and a second
must-fail fixture row `escalation → escalate`; a `doctor` rule-table row.
*Why:* ADR-0017 [documented] requires a terminal path per gate but does not pick one; picking a silent default in code
would make the riskiest of the three (`auto-decide`) reachable by omission. See **AC-276** — the choice itself is a human
decision.

### 8.5 Reconcile and resume

**AC-261** — `rk-kernel` · evidence: F19, F18 · decisions: ADR-0017, ADR-0009
GIVEN any entry into `redkiln run <item>` for an item with an open or recently-closed gate
WHEN the kernel decides the next action
THEN it re-derives the item's state from the world (tree, trailers, event log) before acting, with no cached-state fast
path that skips that derivation.
*Verified by:* an architecture test asserting the single entry point calls the derivation; a mutation-style test —
deleting the re-derivation call must fail at least one test (per F25's lesson [measured] that a green suite does not pin
a claim: inverting the AC-015 ordering left 36/36 green).
*Why:* brief §C.3 [documented implication over measured inputs] — drift is 7% overall and **directionally** more likely
past 1 h (3.6× point estimate, F19; the ≤1 h and >1 h Wilson intervals overlap, so the ratio is not established — see
SM-075), while 45–48% of gates exceed 1 h (F18 [measured]). "Forces `reconcile()` on every entry and forbids an
optimisation that skips it."

**AC-262** — `rk-kernel` · evidence: — · decisions: ADR-0009, ADR-0017
GIVEN the same answer submitted through the CLI adapter and through a second (stub) channel adapter
WHEN each is applied
THEN the two produce byte-identical event sequences and identical resulting item state, and no channel can produce a gate
state transition that is absent from the reconcile event vocabulary.
*Verified by:* a differential test over the two adapters comparing event sequences byte-for-byte; a secondary
architecture test that the answer-application entry point is not `pub` beyond its single caller-facing wrapper.
*Why:* ADR-0009 [documented] — "all channels — present and future — write into the same `reconcile()` path, so this is a
phasing decision about which channels exist yet, not about the gate's internal shape." Recast from naming
`reconcile()` (an implementation) to the observable consequence of there being one: channel-independent event sequences.

**AC-263** — `rk-cli`, `rk-ports` · evidence: — · decisions: ADR-0009
GIVEN Phase 1, and an answer presented through a stub tracker-comment or web-write channel
WHEN it is submitted
THEN it is refused with a named "channel not enabled in this phase" outcome, no gate state changes, and no answer event
is written.
*Verified by:* a port-level test driving a stub non-CLI channel against the answer API and asserting the named refusal
and the unchanged gate; a build-configuration test that the tracker answer route does not compile in.
*Why:* negative criterion, recast to be non-vacuous — as originally written ("no tracker path is wired") it passed
trivially on a system with no tracker code at all. ADR-0009's forbids clause [documented]: no tracker comment grammar or
web-view write may be treated as a valid gate answer during Phase 1.

**AC-264** — `rk-cli` · evidence: — · decisions: ADR-0017
GIVEN an item whose gate has been answered
WHEN the operator resumes it
THEN `redkiln run <item>` is sufficient and `--resume` is not an accepted flag on any command.
*Verified by:* a CLI snapshot test of the accepted flag set; a test asserting `redkiln run --resume` exits as an unknown
argument.
*Why:* proposal §4.5 [documented] — "Resume is one idempotent entry point — `redkiln run <item>` — with no `--resume`
flag (L2.3)." A separate resume verb is a second code path that would not get AC-261's re-derivation.

**AC-265** — `rk-kernel` · evidence: F38 · decisions: ADR-0017, ADR-0022
GIVEN an activity suspended at a mid-graph gate step, whose process was then killed
WHEN the item is resumed after the gate is answered
THEN execution continues at that same step with the answer bound as its input, and the artifact digests produced from
that point are identical to the un-interrupted run's.
*Verified by:* the P8 fixture promoted to a kernel regression test, running a real `taskkill /T /F` (not a simulated
abort) and comparing digests.
*Why:* F38 [measured, n=3 of a pre-registered 27+3] — 0 changed, 0 lost, digests identical across `taskkill /T /F`;
P8 §5.4(1)'s lost-output kill did not fire. Limit: P8's state holds exactly **one** step (brief §C.4), so this criterion
is asserted at the granularity that was actually measured and no deeper. *Runbook placement (2026-09-11, PIA-047):* the
gates' kernel work — this record, the review classifier and policy table, the verify harness and ledger — has no build
sub-step of its own in the runbook as written; it belongs in a 7b-ii sub-step with a runnable checkpoint, and this
criterion is what Step 8 check (3) cites. That is the runbook's edit; this note is the register's.

**AC-266** — PROVISIONAL — `rk-kernel` · evidence: F38 · decisions: ADR-0017, ADR-0106 (cost ordering unmeasured), ADR-0087
GIVEN the same item gated mid-graph and gated at the stage boundary
WHEN each is resumed
THEN the mid-graph resume's `cache_creation_input_tokens` does not exceed **1.5×** the stage resume's.
*Verified by:* a cost-regression test on the P8 fixture with a recorded token budget and a fail threshold at 1.5×.
*PROVISIONAL:* rests on **P8, which is partial** — 3 draws of a pre-registered 27+3, $3.01 of a $40 cap. **Correction
(2026-09-11, audit PIA-080; ERRATA E-0087-1; ADR-0106).** The three sub-unity ratios F38 reported (0.45 / 0.55 / 0.78) divided the resumed
step's **`output_tokens`** (8,408 / 10,314 / 14,579) by F28's **`cache_creation_input_tokens`** (~18,603) — unlike
counters, so no ratio. On the like counter the resumed dispatches created **84,960 / 49,468 / 46,961** cache tokens
(4.57× / 2.66× / 2.52× of 18,603) [measured, F38 recount], on which field the 1.5× kill **fires** against F28, subject to
the cross-fixture caveat. **The cost ordering between a mid-graph gate and a stage gate is UNMEASURED** (ADR-0106
decision 1); the comparator unit for P8-B is `cache_creation_input_tokens` on the first post-resume dispatch, summed
across its turns, beside `total_cost_usd` (decision 2), and SM-052/SM-073 re-baseline to no baseline. **P8's 65-min arm,
24-h arm and all three controls are unrun**, so nothing about the ordering holds at any duration.
*Threshold note:* the bar is 1.5× (P8's own registered kill, which did not fire at n=3) and **not** "at or below the
stage resume", which was the original wording — a test with a 1.5× threshold does not enforce a ≤1× claim, and asserting
≤1× at n=3 would over-read three short-duration draws. The tracked distribution and the ≤1× aspiration live in SM-073;
this criterion is the regression guard only.

**AC-267** — PROVISIONAL — `rk-kernel`, `rk-adapter-claude` · evidence: F28, F30/S-H1b · decisions: ADR-0017
GIVEN a gate held open for any duration
WHEN the item resumes
THEN the resume's `cache_creation_input_tokens` does not increase with the gate's duration.
*Verified by:* a metric assertion on the `gate_resumed` event's `cache_creation_input_tokens`, tracked as SM-073 and
asserted as a regression threshold in CI against the recorded baseline.
*Why:* F28 as corrected [measured] — 18,603 tokens at 65.2 min and 18,603 at 49.7 h under the default prompt. Each of
those is **one measured point (n=1 per duration)**; no repeat-draw count is claimed, because the brief records none.
*PROVISIONAL for the static-card-prompt configuration:* S-H1b measured 29,619 created at 66.1 min with share 0.000, and
**the 24 h read was not taken** (F30 is unassigned/never written; the numbers exist only in the design recommendation and
the proposal; the S-H1b rows and `spikes/results/s-wt.jsonl` are the records). Under `--system-prompt` the shared-prefix
subsidy (per machine and per directory — not account-wide; PIA-103) is forfeited across time (brief §C.1), so duration-
invariance is established for the default prompt and **assumed, not measured**, for the card prompt.

**AC-279** — `rk-kernel`, `rk-adapter-claude` · evidence: F28 · decisions: ADR-0017
GIVEN a gate held open for any duration
WHEN the item resumes
THEN the resume performs at most one prompt-prefix rewrite.
*Verified by:* a count assertion on prefix-rewrite events per `gate_resumed`, asserted ≤ 1.
*Why:* split from AC-267 — rewrite *count* and duration-*invariance of token volume* are separately falsifiable (a resume
could rewrite once and still grow with duration, or rewrite twice within a flat budget). Brief §D [measured]: "gate
resume 18,603 default, 29,619–29,680 static — at most one prefix rewrite, flat in duration."

### 8.6 Refusals, audit and the things nothing may do

**AC-268** — `rk-kernel`, `rk-cli` · evidence: F20 · decisions: ADR-0017
GIVEN an answer submitted against a gate that is already answered, already void, already terminal by expiry, or that
never existed
WHEN `reconcile()` processes it
THEN each of those four cases produces a distinct, named outcome and none of them silently succeeds or exits 0 as if
applied.
*Verified by:* a table-driven CLI test with one row per case asserting distinct exit codes and distinct events.
*Why:* negative criterion, and the F20 lesson generalised — a silent success at exit 0 is how a schema void went
unnoticed [measured, F20: `structured_output` absent, exit 0, `subtype: success`]. A gate answer that appears to land but
did not is the same defect class.

**AC-269** — `rk-kernel`, `rk-adapter-claude` · evidence: F20 · decisions: ADR-0009, ADR-0022
GIVEN any agent dispatch, including one whose card carries broad tools
WHEN its card is validated and its tool surface enumerated
THEN no card exposes the answer, mint or resolve path — the answer API is absent from every card's tool list and from the
adapter's exposed surface.
*Verified by:* a card-validator test over every card in the catalog asserting the answer path is absent, plus a
build-configuration test that the answer API is not linked into the adapter's tool registry.
*Why:* negative criterion. ADR-0009 [documented] enumerates the channels and a model is not one; ADR-0022 [documented]
keeps escalation kernel-owned. F20 [measured] is the precedent for a capability surface silently permitting the wrong
thing at exit 0. **Instrument note:** the original also named "an integration test in which an agent instructed to answer
the gate fails to". That is a model-behaviour observation, not an instrument — one compliant sample proves nothing about
reachability, and F27 [measured] puts verdict flakiness at 2/4 items. Keep it as an optional smoke test; the static
checks above are the criterion.

**AC-270** — `rk-kernel` · evidence: F19, F38 · decisions: ADR-0017
GIVEN only the event log for an item
WHEN it is replayed
THEN every gate open, answer, void, expiry, escalation and re-open is present and the item's terminal gate state is
reconstructible without reading the working tree.
*Verified by:* a replay assertion that reconstructs state from the log alone and compares it to the live derivation; the
same test doubles as AC-258's clock-independence instrument.
*Why:* F19's stated limit [measured] — the FSM was never replayed. F38 [measured, n=3] showed the state carrier survives
a kill; nothing yet shows the log alone is sufficient.

**AC-271** — PROVISIONAL — `rk-kernel` · evidence: — · decisions: ADR-0017
GIVEN one open gate and two answers arriving with valid tokens and matching tree hashes
WHEN both are processed
THEN the first applies, the second is refused as spent (AC-246), and **no second mark is minted** for the same gate.
*Verified by:* a concurrency test submitting two answers against one token from two processes; the semantics under test
are declared, not derived.
*PROVISIONAL:* this is a design assertion with **no evidence**, and the evidence line is empty for that reason. F38
states plainly that whether a second arrival mints a new mark is **not answerable from P8**: §5.4(3) is circular by
construction — arrivals were forced, and the mark was minted by the same driver that implements the semantics under test.
The **arrival rate is entirely unmeasured** (see SM-078). The criterion is written so the chosen semantics are at least
pinned by a test, not so the choice is claimed to be evidenced. *Scope (2026-09-11, PIA-050):* this criterion and SM-078
are **duplicate-answer** instruments — two answers to one open mark. Whether a gate **re-reached by loop-back** mints a
fresh mark is a different question, AC-524's; the discovery instrument for it is a `gate_opened`-per-(item, gate step)
counter.

**AC-524** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — **PROVISIONAL (P8-B)** —
`rk-kernel` · evidence: activity-graph schema :262 ("Decided pending P8: a gate re-reached via loop-back is a new mark
and a new `HumanAnswer`") [documented]; ADR-0087 clause 4 [documented]; F38 — second-arrival semantics not answerable
from P8, §5.4(3) circular by construction [measured, n=3] · decisions: ADR-0105 clause 10, ADR-0087
GIVEN a gate step that an `on_false` loop-back (ADR-0103's `{ to = <earlier step>, max = N }` form) re-reaches after its
first mark was answered
WHEN the kernel reaches the gate step again
THEN it mints a **fresh** `(token, tree_hash, deadline)` record and a new `gate_opened` event — the earlier answer is not
replayed, the earlier token stays spent (AC-246), and the new mark's `tree_hash` is derived by AC-245's algorithm at the
re-reach
AND a `gate_opened` counter per `(item, gate step)` is kept as the discovery instrument, so the re-reach rate is a
measured quantity from the first run — SM-078 and SM-082 cite this criterion for the fresh-mark-on-re-reach meaning;
AC-271/SM-078's duplicate-answer instrument is the other question.
*Verified by:* a kernel test over a fixture graph whose gate sits inside a two-iteration loop, asserting two distinct
tokens, two `gate_opened` events, the first token refused on the second reach, and the per-(item, step) counter at 2;
a replay assertion that the second mark's tree hash was taken at the re-reach.
*PROVISIONAL:* the fresh-mark choice is declared, not evidenced — the schema recorded it "pending P8" and P8-B has not
run; ADR-0105 clause 10 fixes it provisionally until P8-B answers. The alternative (replay the earlier answer when the
tree is unchanged) is the reading this criterion's test would fail, deliberately, so that the choice is visible.

**AC-272** — `rk-kernel` · evidence: — · decisions: ADR-0022, ADR-0038
GIVEN an escalation raised from inside an activity's internal graph
WHEN it is observed in the event log and the trailer
THEN it is indistinguishable in mechanism from a stage-level gate: the same record shape, the same reconcile path, the
same terminal-expiry rule.
*Verified by:* a test asserting the two producers emit structurally identical `gate_opened` records; a schema test that
one gate type exists, not two.
*Why:* ADR-0022 [documented] — encapsulation must not hide human-in-the-loop moments from the event log. A second,
weaker gate mechanism inside activities would be the way that promise gets broken.

### 8.7 Not machine-decidable — routed to human gates

**AC-273** — placement · evidence: F28, F38 · decisions: ADR-0017
GIVEN the cost evidence that gate **duration** is free (18,603 tokens at both 65.2 min and 49.7 h, F28 [measured]) and
gate **frequency** has an **UNMEASURED** cost ordering (F38's "0.45–0.78×" divided output tokens by F28's
cache-creation tokens; on the like counter the resumes created 84,960 / 49,468 / 46,961 cache tokens — E-0087-1,
ADR-0106; n=3, partial) — placement is a value decision, never derived from a cost model in either direction
WHEN gate placement is chosen for a stage or activity
THEN a placement decision record exists for that stage or activity, naming an owner and a date, and no placement is
derived from a cost model.
*Machine half — verified by:* a lint over the placement table asserting every stage/activity present in the catalog has a
placement record with a non-empty owner; a test that no code path computes placement from a token or cost figure.
*Human half — routed to a human gate; not machine-decidable.* Reason: the evidence removed the cost argument entirely
(brief §C.2 — "Placement is a value decision"), so there is no measurable quantity left for a machine to optimise. The
machine can check that a decision exists and is attributed; it cannot check that it is right.

**AC-274** — autonomy table (machine half) · evidence: — · decisions: ADR-0017, ADR-0009
GIVEN the autonomy policy table of (stage × classification) → L1 report / L2 gated / L3 unattended-allowlisted
WHEN it is linted
THEN the table is total — every (stage, classification) pair present in the catalog has a row — and no row is L3 without
a non-empty allowlist.
*Verified by:* a lint rule over the policy table plus a fixture table with a missing pair and an allowlist-free L3 row,
both of which must fail.
*Why:* the totality and allowlist properties are machine-checkable even though the level itself is not (AC-275).

**AC-275** — autonomy level (human half) · evidence: — · decisions: ADR-0017
GIVEN a (stage × classification) pair
WHEN its autonomy level L1/L2/L3 is chosen
THEN the choice is made and signed by a human.
*Verified by:* **routed to a human gate; not machine-decidable.** Reason: proposal §4.6 — "do not ship a level you will
retract" is a judgement about consequences the kernel cannot observe. No spike measured autonomy-level regret; the
corpus contains no retraction event to learn from.

**AC-276** — expiry action per gate class (human half) · evidence: — · decisions: ADR-0017
GIVEN a gate class whose expiry action must be `fail`, `escalate` or `auto-decide`
WHEN the action is chosen
THEN the choice is made and signed by a human, per class.
*Verified by:* **routed to a human gate; not machine-decidable.** Reason: ADR-0017 fixes the *set* of terminal paths but
explicitly does not choose among them; the choice trades an unmeasured risk (auto-decide on an unreviewed tree) against
an unmeasured cost (a failed run). The F18 corpus of 174 gates had **no expiry mechanism, so expiry was unobservable**
[inferred] — that is an absence of instrumentation, not an observed absence of expiries, and it supplies no baseline in
either direction. The machine's half is AC-260 (the field must be present).

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-073 | Mid-graph gate resume cost | — |
| SM-074 | Gate open duration distribution | DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE; no firing kill |
| SM-075 | Void rate at answer (drift-at-answer) | — |
| SM-076 | Gate hang count (expired-and-open) | saturating; no baseline |
| SM-077 | Model spend while a gate is open | saturating |
| SM-078 | Answer arrivals per open gate (second-arrival rate) | DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE; no firing kill; no baseline |
| SM-079 | Expiry terminal-path mix | no baseline |
| SM-080 | Resume losslessness | saturating |
| SM-081 | Void rework cost | DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE; no firing kill; no baseline |
| SM-082 | Gate frequency per item | DISCOVERY INSTRUMENT, NOT A SUCCESS MEASURE; no firing kill; no baseline |

#### Residual risk this area could not repair

- No arithmetic error existed anywhere in the original — every ratio, percentage and Wilson interval recomputes exactly against the brief. The defects were entirely in citations and instruments. Downstream: the numbers can be trusted, the attributions could not.
- AC-266 and AC-267 remain PROVISIONAL on P8, which is 3 draws of a pre-registered 27+3 at SHORT DURATION ONLY. AC-267's whole point is duration-invariance and the duration arms are exactly what is missing. If P8 is never completed these two should be demoted to open questions rather than shipped as assurances.
- AC-267's card-prompt half is ASSUMED, not measured — S-H1b's 24 h read was never taken and F30 is an unassigned ledger slot. §C.1 says the two cache mitigations point in OPPOSITE directions, so under `--system-prompt` duration-invariance may simply be false. This is the criterion most likely to be falsified in production.
- AC-271 (a second answer mints no new mark) is a declared design choice with no evidence, and SM-078 confirms no spike can settle it (P8 §5.4(3) is circular by construction). It should carry a named human owner as AC-273/275/276 do.
- About a third of the surviving criteria rest on ADR text alone with an empty evidence line. ADR-0017 is load-bearing for roughly half the area and its quotations were not verifiable in this pass.
- AC-278's descendant-process check is written against a job object / process group. On Windows — the platform F22 was measured on — job-object containment is the reliable mechanism; if the implementation uses process groups the test may pass while a detached child survives.
- No criterion covers what the human actually SEES at the gate beyond the tree render (AC-245) and the drift attachment (AC-254/AC-277). Nothing requires that a gate carry a question at all.

---

## 9. Review gate — reviewer proposes, machine confirms

**Area key:** `review-gate` · **Criteria:** AC-280..AC-319 **plus AC-530** (AC-297 cut; AC-318, AC-319 are splits; AC-530 is allocated outside the range — see §0.4) — **40 in force** · **Measures:** SM-085..SM-096
**State:** 35 binding (AC-530 added 2026-09-11) · 3 provisional (AC-308; AC-284 until S-CLASS-REPLAY; AC-312 until S-SUBTLE-CB — both labels added 2026-09-11) · 2 human-gated (AC-309, AC-311)
**Sources:** proposal §4.6 (rev. 4); ADR-0074 (supersedes ADR-0004 in full — every `decisions: ADR-0004` in this section now reads ADR-0074), 0013, 0015, 0019, 0020, 0023, 0025, 0026, 0036, **0082** (advisory sink), **0096** (typed repro; discriminating mutant), **0101** (prior-approval framing arm), **0104** (author-declared mutants). Ids outside that list were removed in refutation.

*Vocabulary, fixed once (2026-09-11, PIA-032; decision #21 of `spikes/DECISIONS-2026-09-11.md`).* **`EvidenceClass`** is the closed four computed from the repro record — `reproduced | mutation-survived | narrowed-pin | advisory` (AC-284). **`ReExecution`** is a separate outcome carried on the finding — `Flipped | Unverifiable { both_fail | cannot_run | timeout }` (AC-287, AC-290); `Unverifiable` is a re-execution outcome, never a fifth class, and SM-088's instrument reads `re_execution = Unverifiable`. **`ReviewWeight`** is the explicit projection of the stage's deontic rigor (ADR-0020) — `optional → ReadOnly`, `recommended → Lenient`, `mandatory → Strict` — and is what the policy table keys on; the numeric `0.2`/`1.0` are ADR-0020's retired spellings and appear nowhere in a criterion. The **ledger** (§10) is one closed enum per AC — `pinned | mutation-survived | unpinned` — with `unpinned` reserved for "no evidence" (AC-333, AC-335). **`basis_reason`** variants are defined at AC-352. Fixture citations to "AC-015" mean the corpus item **issue-146-147 AC-015 (F25)**, never this register's AC-015.

*Scope note.* Neither gate is sufficient alone: a reproduced regression was *approved* on 3 of 10 draws (F29, 60 draws, 0 void), while four adjudicated defects survived fully green suites of 1,890/1,890 and 1,907/1,907 (F31). So the model may only *add* findings; the block is computed in Rust from re-executed evidence, and a green suite may not clear what the reviewer reproduced. CLEAN's ground truth is wrong for **≥13 of 23 (57%)**, so every false-positive number quoted against CLEAN is an upper bound and every criterion replaying CLEAN pins counts, not correctness.

### 9.1 The contract: findings, no verdict

**AC-280** — `rk-types` · evidence: F31 (schema conformance 52/52 = 100%) · decisions: ADR-0074 (supersedes ADR-0004;
its forbids: "any `pass`, `verdict` or `approve` field on the review card's output schema")
GIVEN the review card's declared output schema
WHEN the schema is loaded at configuration time
THEN a schema whose top-level `Findings` object declares any field named in the decision-field denylist
(`pass`, `verdict`, `approve`, `approved`, `approval`, `ok`, `status`, `decision`) is refused as a configuration error,
before any dispatch.
*Verified by:* a `rk-types` schema-shape unit test over the `Findings` derivation; a `redkiln doctor` rule-table row on
card output schemas; fixture cards carrying a `pass` field and an `approve` field, each of which must fail. *(Amended
2026-09-11, PIA-100: ADR-0074 names `approve`, which the list lacked; the three spellings are enumerated.)*
*(Repaired: the original said "or equivalently-typed boolean/enum decision field", which is a judgement no test can
make. The denylist is enumerated so the check can fail deterministically; it is a name check, not a semantic one, and a
verdict field under an unlisted name still gets through — see Refutation notes.)*

**AC-281** — `rk-types` · evidence: F31 (`repro.command` fill 147/147; conformance 52/52 — conformance to the
**pre-0096** repro shape) · decisions: ADR-0074, ADR-0019, **ADR-0096** (the typed repro)
GIVEN a `Findings` payload returned by a review dispatch
WHEN it is deserialised
THEN any item lacking a complete typed `repro { kind: run | mutate, command, mutation: Option<{ patch, suite }>,
expected: { exit } | { assertion: Locator }, observed, expected_when_met }` is rejected at ingest with the item identified,
and never reaches the classifier — including an item whose `command` does not begin with an executable token (a
prose-only "command"), and an item with `kind = mutate` and no `mutation` (ADR-0096 decision 1: malformed, not
`Unverifiable`, not a finding).
*Verified by:* `serde` round-trip + `jsonschema` validation tests; five negative fixtures (missing command, missing
observed, missing expected_when_met, a prose-only command such as "run the tests and see", `kind = mutate` without
`mutation`); a corpus replay asserting 100% of accepted items carry a command string beginning with an executable token
(presence and shape are checkable at ingest; runnability is not — that is AC-288/AC-290's job). *(Amended 2026-09-11,
PIA-007: ADR-0074's repro had no field for the mutant or suite AC-289/AC-305 re-execute; the snapshot at runbook Step
5.3 is taken on this shape.)*

**AC-282** — `rk-adapter-claude` + `rk-kernel` · evidence: F20 (tools allowlist silently voids `--json-schema`, exit 0,
`subtype: success`) · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a review dispatch that exits 0 with no `structured_output` block
WHEN the kernel consumes the result
THEN the step fails with a missing-structured-output error and no finding set — empty or otherwise — is produced.
*Verified by:* a `rk-kernel` unit test over a recorded transcript with `structured_output` absent (asserting the step is
an error, not a success with zero findings); a golden transcript fixture from F20.

**AC-318** — `rk-kernel` · evidence: F20; brief §C5 ("forbids parsing JSON out of prose") · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN the review path in the compiled workspace
WHEN the architecture lint runs
THEN no code path in the review path extracts findings from assistant prose (no JSON-from-text parse, no regex over the
transcript body).
*Verified by:* an architecture lint over the review path's call graph asserting zero calls into any text-to-JSON
extraction helper; a compile-fail/lint-fail fixture that adds one.
*(Split out of AC-282: a runtime behaviour on an empty result and a static property of the workspace are two
behaviours with two instruments.)*

**AC-283** — `rk-kernel` (policy) · evidence: F29 (a reproduced regression was approved on 3 of 10 draws; 19/19
rejections on-target — the prose verdict moves while the finding does not) · decisions: ADR-0074, ADR-0096
GIVEN two `Findings` payloads identical except that one's `rationale` free text asserts "approved" and the other's
asserts "blocking" — `claim` and `rationale` are the **only** free-text fields on a finding (ADR-0096 decision 1); every
other field is typed
WHEN the classifier (the pure function over the repro record) computes the class and `policy::classify` (the
policy-table lookup) computes the block set — **two functions**, neither of which takes a free-text field
THEN the two block sets are byte-identical.
*Verified by:* a signature/type test that no free-text field is an input type to either the classifier or
`policy::classify` — this is the load-bearing instrument; the paired property test over rationale as a free variable is
a corollary of the type and cannot fail while the signature holds.
*(Repaired: the F23 McNemar p = 0.625 citation was cut — that figure is a test on the E3 arms' **CLEAN false-positive**
discordant pairs (3 vs 1; detection 23/23 vs 23/23 has no discordant pairs and admits no test — E-0001-1, PIA-073) and
says nothing about prose entering policy.)*

### 9.2 The evidence class is computed in Rust

**AC-284** — `rk-kernel` (classifier) · **PROVISIONAL (S-CLASS-REPLAY)** · evidence: F31 (adjudication **14/14
confirmed** — human adjudication, not mechanical re-execution; and the block counts and sink seeds were produced by
`policy.blocks(evidence)` on the **model-emitted** label, `spikes/runner/src/e3.rs:254-256` — so F31's 9/23 and 0/23
are measured under the model-label policy and the Rust-computed class is **unmeasured** [measured, e3.rs; ERRATA
E-0074-2, PIA-097]), F25 (mechanism: an AC is a claim about one assertion) · decisions: ADR-0074, ADR-0019, ADR-0096
GIVEN a finding's typed repro record (ADR-0096) and the record of what actually ran
WHEN the class is computed
THEN it is the total decision table and nothing else — `reproduced` iff `kind = run`, the command ran and `observed ≠
expected_when_met`; `mutation-survived` iff `kind = mutate`, the patch was applied, the named suite stayed green **and**
the mutant discriminates (a reverted-fix patch, or an `expected.assertion` the gate observed flip — ADR-0096 decision
2); `narrowed-pin` iff only the discriminating assertion flips (or a surviving mutant has an assertion that did not
flip); `advisory` in every remaining case, including a surviving mutant with no assertion and every unrecognised
input.
*Verified by:* a table-driven unit test with one row per branch plus an `otherwise` row; an exhaustive `match` over the
closed four-variant `EvidenceClass` enum (no `_ =>` fallthrough to a blocking class); a fuzz input that produces
`advisory`, never a panic. **S-CLASS-REPLAY** ($0; consumers this criterion and SM-091): run this decision table over
the 253 recorded S-TYPED findings (118/29/29/77 by model label) and report per-class agreement with the model label.
Until it runs, the 9/23 and 0/23 are not evidence for the Rust classifier and this criterion is PROVISIONAL.

**AC-285** — `rk-kernel` · evidence: F27 (detection is stable, the verdict is not: detection 0/7 items flaky over 21
draws, verdict 2/4 items flaky), proposal §4.6 item 2 · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a finding whose model-supplied `evidence` label disagrees with the computed class
WHEN policy runs
THEN the computed class is used, the model label is retained only on the persisted finding, and a
`review_class_drift` event is emitted carrying both labels.
*Verified by:* a unit test on a disagreeing fixture asserting class, event emission and block set; an architecture lint
that `finding.evidence_label` has no reader inside the policy module.
*(Repaired: the original cited F31 for "class labels wobble at boundaries while claims do not". No such observation is
in the evidence brief's F31 row. F27 is the measured statement of the same shape and is cited instead.)*

**AC-286** — `rk-kernel` · evidence: F27 (verdict flaky on 2/4 items over 21 draws; detection 0/7 flaky) ·
decisions: ADR-0074 (supersedes ADR-0004)
GIVEN the same repro record and re-execution record
WHEN the class is computed 1,000 times in one process and again in a fresh process
THEN every result is identical
AND the classifier reads no clock, no environment variable and no file.
*Verified by:* a `proptest` determinism property; a source-level lint over the classifier module forbidding `std::fs`,
`std::time`, `std::env` and `SystemTime`/`Instant` call sites (a crate-level dependency-edge check alone would not
catch a transitive route, so the lint is on the module's own syntax).

### 9.3 Re-execution: a finding blocks only when it flips

**AC-287** — `rk-kernel` · evidence: F31, F29 · decisions: ADR-0074 (supersedes ADR-0004), ADR-0015 (superseded by ADR-0020)
GIVEN a finding classified `reproduced` or `mutation-survived`
WHEN the deterministic gate has produced no re-execution record for it at the reviewed SHA
THEN the finding does not block, and its re-execution outcome is `Unverifiable { cannot_run }` — an outcome carried on
the finding beside its class, not a change of class (§9 vocabulary).
*Verified by:* a unit test with an empty re-execution record asserting an empty block set; a replay assertion over the
corpus that `blocks ⊆ re-executed`.

**AC-288** — `rk-kernel` · evidence: F31 (14/14 adjudicated confirmed) · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a `reproduced` finding re-executed by the deterministic gate
WHEN the command fails on the candidate **and** passes on the reference
THEN and only then is the finding a block.
*Verified by:* a four-cell truth-table test (candidate × reference); a seeded corpus fixture whose command flips must
block, and a paired fixture that fails on both must not.
*(Repaired: the "Agentless 213/300 reproductions, 94 flipped" comparator was cut — the evidence brief's only Agentless
figures are $0.70/instance at 32.00%. See Refutation notes.)*

**AC-289** — `rk-kernel` · evidence: F11 (S0.1: 15 mutants, 9 of 10 test-reachable killed, **1 survived**; test-reachable
90.0%, Wilson 95% [59.6%, 98.2%]; **S0.1-SCOPE: 8 of 23 items unmeasured by construction** — their fixes touch only
`workflows/`, outside the affected-area gate; PIA-076), F31 · decisions: ADR-0074, ADR-0019, **ADR-0096** (a surviving
mutant blocks only when it discriminates)
GIVEN a `mutation-survived` finding whose repro is `kind = mutate` with `mutation { patch, suite }`
WHEN `mutation.patch` is re-applied at the reviewed SHA, `mutation.suite` is re-run and the tree is reverted
THEN it blocks only if the suite is still green with the mutant applied **and** the mutant discriminates — the patch is a
reverted fix (the change under review removed and the named test still green — the E3 measurement of a vacuous test,
F25), or `expected.assertion` names an assertion the gate observed flip; an equivalent mutant (green, no discriminating
assertion) collapses to `narrowed-pin` or `advisory` and cannot block (ADR-0096 decision 2; PIA-056).
*Verified by:* a mutation-harness integration test using the S0.1 fixture set (the 1 surviving mutant must block; the 9
killed ones must not); an equivalent-mutant fixture (a no-op patch, green suite, no assertion) that must not block; the
suite name and mutant patch digest are recorded on the block event; a workflow-only fixture is added once S0.1-SCOPE
runs.

**AC-290** — `rk-kernel` · evidence: F31, timeout history (a 900 s default written for the fast condition cost a CLEAN
record) · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a re-execution that fails on both candidate and reference, or that cannot run at all (missing command, harness
error, timeout)
WHEN the outcome is computed
THEN the finding's re-execution outcome is `Unverifiable { both_fail | cannot_run | timeout }` and it is treated as
advisory — it never blocks and it is never silently recorded as passing
AND the reason is on the persisted finding beside its (unchanged) `EvidenceClass` — `ReExecution = Flipped |
Unverifiable { .. }` is the outcome enum; the class enum has four variants and no `Unverifiable` (§9 vocabulary).
*Verified by:* three negative fixtures, one per reason; a replay assertion that no `Unverifiable` finding appears in any
block set; a doctor rule that **every finding classed `reproduced` or `mutation-survived` carries either a re-execution
record or an `Unverifiable` reason**.
*(Repaired: the original doctor rule — "a review with 0 executed and 0 `Unverifiable` findings is impossible" — is
false. A review that produces no findings at all has exactly those counts and is legal; the rule would have fired on
every clean review.)*

**AC-291** — `rk-git` · evidence: F17 (the E3 reviewer restored the fix from the object store — 42 turns, 22,853 output,
526 s, returned `approved`; an instrument defect) · decisions: ADR-0074 (supersedes ADR-0004), ADR-0026
GIVEN a review or re-execution in progress
WHEN it completes, however it completes
THEN the candidate worktree's tree hash and index are unchanged, and the store has no row written under the reviewer's
identity.
*Verified by:* a **structural** assertion on the spawn configuration — the reviewer's process has no write path to the
candidate worktree or the store (this is what actually holds the invariant); plus an integration **probe** capturing
`git rev-parse HEAD^{tree}` and `git status --porcelain` before and after a review whose card is instructed to modify
files, and the single-write-path rule asserted by an architecture lint.
*(Repaired: a probe that depends on the model choosing to attempt a write cannot establish a negative — F17 took the
reviewer 42 turns to find its route. The structural assertion is the instrument; the probe is corroboration.)*

**AC-292** — `rk-git` · evidence: F17 · decisions: ADR-0074 (supersedes ADR-0004), ADR-0026
GIVEN the reviewer's disposable shallow clone at the reviewed SHA
WHEN the clone is inspected
THEN no object that is absent from the reviewed SHA's own history is present in it: `--depth 1`, no
`objects/info/alternates`, no remote pointing at the parent repo, and `git cat-file -e <pre-change blob>` fails.
*Verified by:* a clone-shape assertion covering each of those four properties; the F17 defeat scenario replayed as a
**probe** (a review of a fix whose original implementation is reachable in the parent repo must not reproduce it) —
recorded as corroboration, not as the proof, since a passing probe may only mean the model did not try.

**AC-293** — `rk-types` · evidence: F17 (the E3 defeat is exactly a review whose basis was not the reviewed tree) ·
decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a completed review
WHEN its outcome is materialised
THEN the outcome record carries the tree digest of the SHA that was reviewed, and a record without that digest fails to
construct — no default, no empty string, no `None`.
*Verified by:* a type-level test that the digest field is non-optional; a serialisation round-trip test that captures
`basis`/`sha`; a negative fixture with the digest removed that must fail to construct.
*(Repaired twice: the citation "shipmates L2.5, B6 (`gateResults()` dropped the field…)" does not appear anywhere in the
evidence brief and was cut; and the original THEN named the type `GreenAt(sha)`, which is both implementation naming and
wrong — a review that produced blocks is not green.)*

### 9.4 The policy table is the only place a class becomes a block

**AC-294** — `rk-kernel` · evidence: proposal §4.6 item 4 · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN the whole workspace
WHEN the review path is compiled
THEN the build fails if a block value is constructed anywhere outside `rk-kernel::policy`, and every block value that
does exist originated in a `(class, stage_weight)` lookup.
*Verified by:* a private constructor plus a module-boundary architecture lint asserting zero construction sites outside
`rk-kernel::policy`; a compile-fail test (`trybuild`) that adds one.

**AC-295** — `rk-kernel` · evidence: F31 (CLEAN blocks strict 12/23, lenient 9/23), F29 (class means Reproduced 0.70 >
MutationSurvived 0.37 > NarrowedPin 0.10 > control 0.00) · decisions: ADR-0015, ADR-0020, ADR-0023
GIVEN one fixed `Findings` payload and re-execution record
WHEN the stage's `ReviewWeight` is changed from `Lenient` (`recommended`) to `Strict` (`mandatory`)
THEN the block set changes only by the addition of `narrowed-pin` findings, with no re-dispatch and no change to the
classifier's output.
*Verified by:* a replay test over a recorded corpus run at both weights, asserting the set difference is exactly the
`narrowed-pin` items; the classifier's output is byte-compared across the two runs.

**AC-296** — `rk-kernel` · evidence: **design invariant of the policy table (ADR-0004, carried by ADR-0074); no measurement supports it** ·
decisions: ADR-0074 (supersedes ADR-0004)
GIVEN any stage rigor weight in the enum
WHEN policy computes the block set
THEN no `advisory` finding is in it.
*Verified by:* a `proptest` quantified over every weight × every class asserting the negative for `advisory`.
*(Repaired twice: the citation "F31 (advisory never blocked in 52/52 conforming records)" is a fabricated gloss —
52/52 is F31's *schema conformance* figure and carries no blocking information; and the `Unverifiable` half was
dropped because AC-290 already asserts it with the same replay instrument.)*

**AC-298** — `rk-kernel` · evidence: F31 (lenient: 9/23 blocked, **0/23 verified false positives**, detection
unchanged; brief §C8) · decisions: ADR-0015, ADR-0020
GIVEN a stage that does not explicitly set a review weight
WHEN policy resolves the weight
THEN the resolved `ReviewWeight` is `Lenient` (the `recommended` projection) — `narrowed-pin` advises, it does not block.
*Verified by:* a default-resolution unit test plus a config golden file. **Note:** the *severity* call behind this
default is Ryan's judgement, not a measurement [F31, brief §A]; the measurement is only that the relaxation cost no
detection, and the 0/23 is measured against a ground truth known wrong for ≥13/23, so it is an upper bound.

**AC-299** — `rk-kernel` · evidence: proposal §4.6 (tier definition); **no measurement — the read-only tier's cost and
recall are both unmeasured here (gap 2)** · decisions: ADR-0015, ADR-0020
GIVEN a stage whose review weight is `ReadOnly` — the `optional` projection (ADR-0020: `0.2 → optional`; the numeric
spelling is retired — PIA-103)
WHEN the review runs
THEN no finding from that stage can enter a block set, and no re-execution is dispatched.
*Verified by:* a unit test asserting the empty block set at `ReadOnly` regardless of class; a process-accounting test
that the read-only tier spawns zero re-execution processes.
*(Repaired: the "+40% to +101% by condition" range is not in the evidence brief — the brief carries only the +74%
overall figure — and was cut; the Cloudflare $1.68 comparator was cut from this criterion because a cost figure cannot
support a blocking-behaviour claim. Both belong to SM-090, where the cost figure survives.)*

**AC-300** — `rk-kernel` (packs) · evidence: F31 (the reviewer produced blocking-grade findings on **12 of 23** shipped,
human-approved CLEAN fixes at the strict weight); brief §Contested (CLEAN's ground truth is wrong for **≥13/23 = 57%**)
· decisions: ADR-0015, ADR-0020
GIVEN the `bug-standard` / fix-lane stage definition
WHEN packs are loaded
THEN its review weight is execution-grade — `Strict`, the `mandatory` projection (ADR-0020: `1.0 → mandatory`; PIA-103)
— and a pack that lowers it fails validation.
*Verified by:* a pack-validation rule and a negative pack fixture; `redkiln doctor` rule-table row.
*(Repaired: the original wrote "≥52% of the corpus was mislabelled", conflating the 12/23 block *rate* with the
separately measured ≥13/23 mislabel figure. They are different quantities from different sources.)*

**AC-301** — `rk-kernel` · evidence: proposal §4.6 ("No cheap-first cascade until its recall loss is measured");
**S-CASCADE** — registered, unrun, does not gate Phase 1 (design §10.5; the earlier "no cascade spike is registered" is
withdrawn, PIA-088) · decisions: ADR-0015, ADR-0012
GIVEN a pack that configures a cheap-first review cascade (a read-only pass gating whether execution-grade runs)
WHEN packs are validated
THEN the configuration is refused with the reason "cascade recall loss unmeasured (S-CASCADE unrun)".
*Verified by:* a pack-validation rule and a fixture pack that must fail. The refusal lifts only when S-CASCADE, the named
spike with this criterion as consumer, reports the cascade's recall loss.
*(Repaired: ADR-0012 is outside this area's declared ADR source set and was cut.)*

**AC-302** — `rk-kernel` · evidence: **F36** (modal `R` identical on only 5/20, 3/20, 2/20, 2/20 against an 80% line;
alpha drift −0.168 on byte-identical anchors; the prescribed remedy does not repair it; 400 dispatches, 0 void) ·
decisions: ADR-0015, ADR-0020
GIVEN the review tier resolution for a stage
WHEN the tier is computed
THEN it derives only from the authored stage rigor weight and authored facets, and no LLM-emitted context vector is an
input.
*Verified by:* a signature/type test that `resolve_review_tier` takes no model-produced value; an architecture lint on
the call graph. This is the standing negative for the **BLOCKED selection engine** (P1's kill fired 4/4, F36): review
tier may not be made to depend on it. Arm D precision/recall is now computed [measured, F39], but it measures a
*derived* vector, not the LLM-emitted one this criterion excludes, so it licenses nothing about the input barred here.
*(Repaired: ADR-0030 is outside the declared ADR source set and was cut.)*

**AC-303** — `rk-kernel` · evidence: **F37 / brief §C10** (the rigor ladder is not monotone in internal quality:
escalating to `mandatory` with `has_repro` selects `bisect`, which has no adversary and no policy step), F37 (the policy
check was false on 33 of 33) · decisions: ADR-0081, ADR-0020, ADR-0023, ADR-0106, ADR-0111 (S-P7B's re-run changes the criterion)
GIVEN any pair of rigor levels (lower, higher) for a stage that has a review-confirm step
WHEN the lattice is linted
THEN no escalation removes the review-confirm step from the resolved graph.
*Verified by:* a lattice lint over every (activity × rigor) pair asserting step-set monotonicity for the confirm step;
the F37 `mandatory`+`has_repro` case as a named regression row. **Note:** F37's cost ratio is reported as 3.66× (F37)
and 4.4× (decisions log) — **reconciled by denominator identification** (ADR-0106 decision 3): 4.4× = $9.857 / $2.222,
the issue-77 `recommended` r1 single run; 3.66× = $9.857 / $2.693, the `recommended` arm mean at n = 30; the
`mandatory` figure is n = 1 and stays so until S-P7B (ADR-0111). This criterion does not depend on either number. The
shipped catalog's `bisect` rung is repaired by ceiling (`max_rigor = "recommended"`, ADR-0093 decision 5), so the
regression row now asserts the ceiling holds.

### 9.5 Quarantine, override, and the events

**AC-304** — `rk-types` + `rk-kernel` · evidence: proposal §4.6 item 5 · decisions: ADR-0074 (supersedes ADR-0004), ADR-0025
GIVEN a finding in state `Quarantined { reason, by, until }`
WHEN policy computes the block set
THEN the finding is absent from it, and it is still persisted and rendered with its quarantine reason.
*Verified by:* a unit test on the quarantined branch; a rendering golden file showing the finding is visible, not
deleted; a negative fixture in which a quarantine is requested for an **advisory** record, which must be refused —
advisory and `Quarantined` are disjoint (ADR-0082 clause 4; PIA-063).
*(Repaired: the comparators "GitLab quarantine" and Cloudflare "won't fix" appear nowhere in the evidence brief — its
only Cloudflare figures are read-only $0.98/$1.68 and break-glass 288/48,095 — and were cut. Prior art for the state
itself is convergent, not original: SARIF 2.1.0 §3.35 `suppression`, Snyk `--reason`/`--expiry`, GitHub dismissal,
SonarQube, Semgrep triage — ERRATA E-0074-1; D4-SARIF asks whether `Quarantined` maps onto `suppressions[].status`.)*

**AC-305** — `rk-store` · evidence: proposal §4.6 item 5; **designed, not measured (gap 4)** · decisions: ADR-0074 (supersedes ADR-0004),
ADR-0025
GIVEN a finding quarantined at review *n*
WHEN review *n+1* returns a finding with the same stable identity — the tuple (AC id, `repro.kind`, repro command digest,
and for `kind = mutate` the digest of `mutation.patch` plus `mutation.suite` — ADR-0096's typed fields, PIA-007)
THEN it is not re-raised as new and does not block.
*Verified by:* a finding-identity unit test (identity stable under rationale rewording and file-order change, unstable
under a changed repro command or a changed mutation patch); a two-review integration test.
*(Repaired: identity originally included a "normalised claim". Deciding that two differently worded claims are the same
claim is a judgement; the tuple above is computable. The cost is that a re-review restating the same defect with a
different command will re-raise — acknowledged in Refutation notes.)*

**AC-306** — `rk-kernel` · evidence: F18 (gate→verdict n=174, median **10.5 min**, 45% >1 h; stage→verdict n=191, p90
~9 h, max 226 h — two distinct series), F19 (drift 12/174 = 7% overall; 9/79 = **11%** on gates >1 h) ·
decisions: ADR-0025
GIVEN a quarantine whose `until` has passed
WHEN the next review runs
THEN the finding is re-raised and a `quarantine_expired` event is emitted, and the re-raised finding is subject to the
prevailing weight exactly as a newly raised finding of its class — it blocks iff its class blocks at that weight.
*Verified by:* a clock-injected unit test at `until ± 1s` asserting re-raise, event, and the class-appropriate block
decision at both lenient and strict; a doctor rule listing live quarantines and their ages.
*(Repaired: the original said the re-raised finding "blocks under the prevailing weight" unconditionally, which is
false for an `advisory` or `Unverifiable` finding and contradicts AC-290/AC-296. The two F18 series were also being
quoted as one.)*

**AC-319** — `rk-types` · evidence: proposal §4.6 item 5; F18 (a gate can outlive its own assumptions — max 226 h =
9.4 days) · decisions: ADR-0025
GIVEN a quarantine being constructed
WHEN it carries no `until`
THEN construction is refused.
*Verified by:* a type test that `until` is non-optional; a negative fixture that must fail to construct.
*(Split out of AC-306: an expiry behaviour and a construction-time refusal are two behaviours with two instruments.)*

**AC-307** — `rk-kernel` (events) · evidence: proposal §4.6 item 6; F31 (14/14 adjudicated confirmed) ·
decisions: ADR-0074 (supersedes ADR-0004), ADR-0036
GIVEN any block and any override (quarantine grant or removal)
WHEN it occurs
THEN an event is appended carrying `{ finding_id, ac, computed_class, model_label, stage_weight, sha, re_execution_id,
actor, adjudication: null }`, and no block or override exists without one.
*Verified by:* an event-schema test; a replay invariant `count(blocks) == count(block_events)` over the corpus; an
architecture lint that the block path and the event append are the same code path.
*(Repaired: the gloss "F29/F31 (all blocking decisions came from re-executed evidence)" is not a finding recorded in
the brief for either spike — it is this design's intent, which is AC-287's job to enforce, not evidence for it.)*

**AC-308** — `rk-kernel` · evidence: F24 (all six E3 rejections factually TRUE, 6/6, zero model cost; CLEAN ground truth
wrong for ≥5/23, raised to **≥13/23** in brief §Contested), F31 (0/23 verified false positives) · decisions: ADR-0074 (supersedes ADR-0004) ·
**PROVISIONAL**
GIVEN a set of block events
WHEN `false_block_rate` is computed
THEN only blocks carrying an adjudication record are in the denominator; historical approval (an item having been
merged or human-approved) is never usable as ground truth, and unadjudicated blocks are reported as a separate
`pending_adjudication` count.
*Verified by:* a metric unit test with a merged-but-defective fixture asserting it is excluded, not counted as a false
block; a replay assertion that the metric never reads a merge or approval field.
**PROVISIONAL** — amended 2026-09-11 (PIA-081): the E3 severity adjudication was **adjudicated 2026-09-04 and closed
2026-09-09** (facts settled 6/6; severity from the 2026-09-04 sheet, filled by the model at Ryan's direction, **NOT
BLIND**, one operator, κ forfeited). The rates are settled on that sheet — arm (a) FP 1/23, MCC **+0.957** (tp 23 / fn 0
/ fp 1 / tn 22; the earlier value 0.978 was a computation error, E-0074-3); arm (b) FP 0/23, MCC +1.000 — and remain upper bounds
against CLEAN. This criterion stays PROVISIONAL because the sheet is **not an independent rater** (pending E3-SEV-BLIND's
blind annotation of `e3-severity-annotation-BLANK.md`), not because the adjudication is open; design §6.4.4's "0 of 14
adjudicated" is SM-086's compliant baseline.

**AC-309** — human gate · evidence: F24 (adjudication was zero model cost and reversed the corpus's own labels), F31
(14/14 adjudicated confirmed) · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a block event awaiting adjudication
WHEN a human decides whether the block was true or false
THEN the decision is recorded as an adjudication record bound to the block event id and the reviewed SHA.
*Verified by:* **routed to a human gate — not machine-decidable.** Reason: the only machine-available label
(historical approval) is wrong for ≥13/23 of the corpus, so no automatic oracle exists. The machine-checkable half is
only that the record exists and is bound (AC-307, AC-308).

### 9.6 The Advisory sink

**AC-310** — `rk-store` · evidence: F31 / brief §C8 (**seven adjudicated-true `narrowed-pin` findings have no
destination; no Advisory sink exists**) · decisions: ADR-0074 (supersedes ADR-0004), ADR-0025
GIVEN a review that produced findings which do not block (`advisory`, `Unverifiable`, or a class the weight does not
block)
WHEN the review completes
THEN every one of them is persisted to the item's advisory sink with its class, repro record and reason for not
blocking, and none is dropped
AND `redkiln doctor` errors when a completed review's non-blocking finding count exceeds its sink's record count.
*Verified by:* a replay assertion `findings_in == blocks + sink_records` and, over the sink's lifetime, `live_sink ==
created - retired - dismissed` (AC-530's `advisory_retired` accounting); a doctor rule-table row; the seven F31
`narrowed-pin` findings replayed as the seed fixture, which must all land in the sink (seeded under the model-label
policy — E-0074-2; the Rust-classified seed set is S-CLASS-REPLAY's).

**AC-530** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-store` + `rk-kernel` ·
evidence: ADR-0082 (the sink is a first-class destination; a record leaves it only by a recorded disposition)
[documented]; F31's seven `narrowed-pin` findings as the seed [measured, model-label policy] · decisions: ADR-0082,
ADR-0074
GIVEN an advisory record in the sink whose finding is not re-raised by the next review of the same item (its stable
identity, AC-305, is absent from review *n+1*'s findings)
WHEN review *n+1* completes
THEN an `advisory_retired { finding_id, reason: not_re_raised, review_id }` event is appended and the record leaves the
live sink — never silently, never by deletion
AND at every point the replay invariant `live_sink == created - retired - dismissed` holds, where `dismissed` counts
AC-311's human dispositions.
*Verified by:* a two-review integration test asserting the event and the count; a replay assertion of the invariant over
a fixture run with created, retired and dismissed records; a doctor rule-table row that errors when the live count
disagrees with the invariant. *(PIA-029 named ADR-0082's accounting as uncovered; AC-310's `findings_in` invariant
covered intake, not exit.)*

**AC-311** — human gate · evidence: F31 (the seven have no destination), F18 (gate answering is cheap: zero model cost,
gate→verdict median 10.5 min, n=174, one operator) · decisions: ADR-0025, **ADR-0082** (advisory and `Quarantined` are
disjoint)
GIVEN an advisory record in the sink
WHEN a human triages it into a backlog item or a dismissal (with reason) — never a quarantine, which ADR-0082 clause 4
reserves for a blocking finding (PIA-063)
THEN the disposition is recorded against the advisory record.
*Verified by:* **routed to a human gate — not machine-decidable.** Reason: whether an advisory finding is worth a
backlog item is a value call. Machine-checkable half: every sink record reaches a terminal disposition or is surfaced
as ageing (SM-089).

### 9.7 The reviewer's own regression test

**AC-312** — corpus · **PROVISIONAL (S-SUBTLE-CB — the prior-approval framing arm, unrun)** · evidence: F29 (S-VAR
control **0/10** rejections), F26 (S-SUBTLE 5/5 sighted and 5/5 blind, twice); arXiv 2603.18740 v2 — strong bug-free
framing cuts detection by 16–93 percentage points on 5 of 6 models (Opus 4.5 −4.9 pp, n.s.) and false positives
collapse (+46 to +95 pp TN) [documented, fetched 2026-09-10] · decisions: ADR-0074, ADR-0013, **ADR-0101**
GIVEN a release regression corpus containing both negative-control fixtures (no seeded defect), seeded `reproduced`
fixtures, and the **prior-approval framing variants** (known-defective items whose card asserts the change was reviewed
and approved — authored at runbook Step 7c, unrun until S-SUBTLE-CB draws)
WHEN the review gate runs the release regression
THEN **every seeded `reproduced` fixture produces a block and every control produces zero.**
*Verified by:* a corpus regression job asserting both halves in the same run.
**PROVISIONAL (decision #17 of 2026-09-11; PIA-071):** this criterion ships in Phase 1 with the framing-variant fixtures
authored and unrun. Until S-SUBTLE-CB runs, **no document may claim the review gate is guarded against false approval**
(ADR-0101 clause 5), and the control half above is a floor, not calibration; the arm reports missed seeded defects and
the CLEAN-pair approval rate separately (clause 3). The symmetric "presented as already rejected" arm is S-SUBTLE-SYM
(clause 7), registered and not designed.
*(Repaired: as originally written — "zero blocks on the controls" — this passed on a gate that never blocks anything,
and its stated rescue ("only has meaning paired with AC-313") is wrong, because AC-313's determinism check is also
satisfied by a permanently empty block set. The conjunction is deliberate: neither half is a criterion alone.)*

**AC-313** — `rk-kernel` · evidence: **F29** (a reproduced regression was approved on 3 of 10 draws — the model half is
a weighted coin: class means 0.70 / 0.37 / 0.10 / 0.00 over 60 draws) · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN one recorded `Findings` payload containing a `reproduced` finding, **its recorded re-execution record**, and the
reviewed SHA (re-execution determinism itself is AC-288/AC-289's job; this criterion holds the record fixed — PIA-032)
WHEN the confirm half is run 10 times
THEN the block set is identical on all 10 runs **and equals the expected non-empty set for that fixture**.
*Verified by:* a replay job (recorded findings, no model dispatch, $0) asserting both set equality across k=10 and
equality to the pinned expected set. This is the criterion that converts F29's 3/10 verdict flip from a correctness
problem into a recall problem: the machine half must not flip at all.
*(Repaired: without the non-empty clause, a gate that blocks nothing scores 10/10.)*

**AC-314** — corpus · evidence: F37 / brief §Contested (`emit.corpus_sha` holds a per-item checkpoint sha, so the key is
misnamed) · decisions: ADR-0013
GIVEN a review regression run
WHEN its record is written
THEN it carries the corpus repository's commit sha, and a per-item checkpoint sha is stored under a differently named
field.
*Verified by:* a field-semantics test asserting the recorded sha resolves in the corpus repo's history; a doctor rule
that refuses a regression record whose corpus sha is unknown to the pinned corpus remote.
*(Repaired: the F13 citation was cut — F13 is the shared-prefix segmentation finding and has no bearing on sha field
semantics.)*

### 9.8 Neither gate is sufficient alone

**AC-315** — `rk-kernel` · evidence: F31 / brief §C7 (four adjudicated defects survived fully green suites of
1,890/1,890 and 1,907/1,907 — two through green suites, issue-70 and issue-83; two (issue-131) against an already-red
54-failure baseline, E-0082-2), F23 (detection 23/23 vs 23/23 — no discordant pairs, no test, **saturated, no
discriminating power**; the McNemar p = 0.625 is on the CLEAN FP 3-vs-1 discordant pairs — E-0001-1) · decisions:
ADR-0074, ADR-0019
GIVEN a review that produced no blocking findings
WHEN the item's AC verification state is computed
THEN no AC's state changes as a result — a review can only add findings, never mark an AC verified.
*Verified by:* a state-transition unit test (review outcome is not an input to AC verification); an architecture lint on
the call graph; a fixture where a clean review leaves all ACs unverified.

**AC-316** — `rk-kernel` · evidence: F29 (approved 3/10), F31 (14/14 adjudicated confirmed) · decisions: ADR-0074 (supersedes ADR-0004),
ADR-0019
GIVEN a finding that the deterministic gate has re-executed and confirmed as flipping
WHEN the item's full suite is green
THEN the block stands; suite greenness is not a dismissal path, and only a quarantine (AC-304) or a fix that makes the
finding stop flipping removes it.
*Verified by:* an integration test with a green suite plus a flipping finding asserting the block survives; a replay
assertion that no block was ever cleared by a `verify` result.

**AC-317** — `rk-kernel` · evidence: **F25** (issue-146-147 AC-015 probe: remove step 7 → RED, but invert the ordering →
**36/36 GREEN**), proposal §4.6 item 8 (per-AC mutation columns absent from historical reports — the *0/156* count is the
2026-09-05 proposal's and is tagged **[documented, proposal-only; not re-verified]** per ERRATA E-0019-1; the figure to
cite is `spikes/results/s-mig-report.md` Correction §4: **142 of the 142 lifted TDD tables lack the F25 mutation column
[measured, S-MIG]** — out of 156 impl-reports, 7 lifted with loss and 7 failed to lift and were not assessed, so "0/156"
is 0/142 plus 14 unassessed, not a count over 156; this criterion rests on F25, not on either count) ·
decisions: ADR-0019, **ADR-0104** (author-declared mutants; AC-334 owns the predicate)
GIVEN the fix or story lane at report time and every AC in the spec carrying a `mutation: { patch, suite }` field
(ADR-0104 decision 1; `mutation: not-feasible(reason)` per AC-337 is skipped and reported)
WHEN the per-AC mutation harness is invoked
THEN it applies each AC's declared patch at the candidate SHA, runs the named suite, reverts, and records AC-334's
outcome per AC (`pinned` iff the named assertion flips; `mutation-survived` iff the suite stays green), and an AC whose
mutation survives is refused at report time.
*Verified by:* the harness-invocation test in the fix and story lanes asserting one outcome per declared mutation and
the report-time refusal; the F25 issue-146-147 AC-015 ordering inversion as the canonical regression row, which must
yield `mutation-survived`. **Correction (2026-09-11, audit PIA-051; ADR-0104 decision 2).** As authored this gave the
harness a second predicate ("removed, and inverted" — a derivation of its own) beside AC-334's flip predicate; it is now
the invocation criterion that calls AC-334's, and "removed, and inverted" is dropped. The *remediation package* behind
this was proposed, not adopted, in F25; ADR-0019 adopts it and ADR-0104 fixes the derivation.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-085 | Block rate per review, by computed class | — |
| SM-086 | Verified false-block rate, against adjudication only | — |
| SM-087 | Confirm-half determinism | no baseline |
| SM-088 | Unverifiable share of findings | — |
| SM-089 | Advisory sink throughput and age | — |
| SM-090 | Review cost per item, by tier | — |
| SM-091 | Model label vs computed class drift | no baseline |
| SM-092 | Negative-control block rate | saturating |
| SM-093 | Seeded-defect detection recall | saturating; no firing kill |
| SM-094 | Per-AC discriminating-test coverage | no baseline |
| SM-095 | Override (quarantine) rate and live-quarantine age | no baseline |
| SM-096 | Review wall clock and re-execution timeout pressure | — |

#### Residual risk this area could not repair

- AC-280's denylist is a NAME check. A card shipping a top-level boolean called `blocking` or `should_merge` passes configuration validation and reintroduces the verdict field ADR-0074 forbids. The real defence is that policy never reads it (AC-283) — treat AC-280 as hygiene, not a guarantee.
- AC-291 and AC-292 rest on structural assertions written against code that does not exist. F17 is the proof that the previous structural assumption was wrong after 42 turns of adversarial effort; both should be re-adversarialised against the actual spawn and clone code.
- AC-296, AC-299 and AC-305 carry no measurement at all — they are design invariants sitting alongside 35 criteria that cite spikes. They are not the same kind of claim and should not be built with the same confidence.
- Every CLEAN-derived number (12/23, 9/23, 0/23, 14/14) comes from ONE corpus, ONE operator, and a ground truth known wrong for at least 57% of items. AC-298's lenient default and AC-300's execution-grade pin are both decisions taken on that single corpus.
- The severity call behind AC-298 and AC-308 is Ryan's judgement on a sheet filled by the model at his direction — adjudicated 2026-09-04, closed 2026-09-09, NOT BLIND, one operator, κ forfeited (PIA-081). AC-308 is PROVISIONAL because the sheet is not an independent rater (E3-SEV-BLIND is the named route), and AC-298 sets the SYSTEM DEFAULT off the same sheet.
- Every F31 block count and sink seed in this section (12/23, 9/23, 0/23, the seven `narrowed-pin`) was produced under the **model-label** policy the gate forbids (E-0074-2); the Rust-computed class is unmeasured until S-CLASS-REPLAY, and AC-284 is PROVISIONAL on it.
- AC-312's guarding claim is PROVISIONAL on S-SUBTLE-CB: the paper the arm rests on (arXiv 2603.18740) predicts detection falls and false positives collapse under prior-approval framing, so the control half's 0/10 is a floor measured without the framing, not calibration (ADR-0101).
- AC-317's per-AC mutation harness is the most expensive criterion here and rests on a single F25 probe (n=1 AC). SM-094's own kill (harness wall clock above the 1,071 s review median) is very likely to fire given S0.1's 323 s median per mutant.
- Nothing here constrains what happens when the review dispatch itself is void or rate-limited. AC-282 covers exit-0-with-no-output, but there is no criterion for 'the review never returned', and a missing review must not be indistinguishable from a clean one.

---

## 10. Verification gate — affected-area selection and per-AC mutation evidence

**Area key:** `verify-gate` · **Criteria:** AC-320..AC-359 (AC-323 is a reused id — see §0.4) — **40 in force** · **Measures:** SM-097..SM-107 (SM-098 demoted to a descriptive report; SM-108 unallocated)
**State:** 37 binding · 1 provisional (AC-356) · 2 human-gated (AC-357, AC-358)
**Sources:** proposal §4.6, §4.2/§4.2.1, §3 (principles 1, 4, 5, 6, 11), §4.1; ADR-0074 (supersedes ADR-0004), 0008, 0014, 0017, 0019, 0029, 0030/0061, 0073.

*Scope note.* Two mechanisms this area exists to defend. (1) Red→Green is evidenced at TEST granularity while an acceptance criterion is a claim about ONE ASSERTION: on the AC-015 probe, removing step 7 made the suite RED while inverting the ordering left **36 of 36 GREEN** (F25, n=1 AC, one operator). (2) Affected-area selection fails silently: a filter that did not survive package-manager forwarding turned a targeted invocation into a whole-suite run for **26 of 68 invocations**, a ~60× difference that ran undetected (brief §D).

### 10.1 Inputs, basis, and the record at the point of use

**AC-320** — `rk-kernel-verify` · evidence: §3 principle 4 (proposal) · decisions: ADR-0019
GIVEN a `verify` activity dispatched without a resolved changed-file set and a resolved toolchain selection policy
WHEN the kernel prepares the activity
THEN the activity fails with `StepError` before any toolchain process is spawned
AND no gate record is written for that attempt.
*Verified by:* unit test on the verify activity's input contract; a fixture run with an unset changed-file set that must
fail; an event-log assertion that zero `process_spawned` events precede the error.

**AC-321** — `rk-kernel-verify` · evidence: F19 (limit: the FSM was never replayed), brief §C.4 · decisions: ADR-0019
GIVEN a **completed** verify run over a named git range
WHEN its record is replayed for audit (not resumed — see AC-353)
THEN the range endpoints, the resolved path list and the toolchain version are read back from the run event verbatim
AND the replay reproduces the identical path list without re-reading the working tree.
*Verified by:* a replay test that rebuilds the path list from the event alone and asserts byte equality; a property test
over 100 synthetic ranges. Standing limit: the FSM has never been replayed on the corpus [F19], so this is a criterion
for new code, not a reproduction of a measured result.

**AC-322** — `rk-kernel-verify` · evidence: brief §D (26 of 68) · decisions: ADR-0019
GIVEN a verify run whose executed selection did not narrow the suite
WHEN the gate record is written
THEN the recorded `basis` is `whole-suite` and never `affected`.
*Verified by:* a fixture toolchain whose filter is deliberately inert; an assertion that the recorded basis is
`whole-suite` for that run.

**AC-323** — `rk-cli` / `rk-store-artifacts` · evidence: brief §D, §4.6 (B3, "the point of use says what ran") ·
decisions: ADR-0019
GIVEN a verify result rendered anywhere a human reads an outcome (CLI status line, gate record, event)
WHEN the outcome is shown
THEN the basis and its reason are shown in the same rendering as the result, so a reader cannot see "green" without
seeing what ran
AND a rendering that omits the basis fails.
*Verified by:* a snapshot test per rendering surface that fails if the basis token is absent; a negative fixture whose
renderer drops the basis, which must fail the snapshot.

**AC-324** — `rk-kernel-verify` · evidence: F20 (a tool-restricted card silently voids `--json-schema` at exit 0 — the
mechanism behind a consumer that fails open), brief §C.5 · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN a verify result whose `basis`, suite digest, or executed-command record is absent
WHEN the kernel evaluates the verification gate
THEN the gate resolves to `Err`, never to a pass
AND the item does not advance.
*Verified by:* `Result<VerifyReport, StepError>` at the boundary with no `Default` impl; three fixtures (missing basis,
missing digest, missing argv) that must each produce `Err`; a lint forbidding `unwrap_or_default` on the verify result.

### 10.2 The 60× selection defect — proving that selection actually selected

**AC-325** — `rk-port-toolchain` · evidence: brief §D (26 of 68 whole-suite invocations) · decisions: ADR-0029
(tie-break superseded by ADR-0073)
GIVEN a toolchain selection command that passes through a wrapper (a package manager, a task runner, a shell)
WHEN the test process is executed
THEN the argv actually handed to the OS process API is recorded on the run event as an argument vector, not the
templated command string
AND re-executing the recorded argv verbatim reproduces the same executed-test count.
*Verified by:* an adapter test asserting the recorded argv differs from the template when a wrapper rewrites it;
a fixture wrapper that drops its trailing arguments, which must be visible in the record; a re-execution test on the
recorded vector asserting the same test count.

**AC-326** — `rk-kernel-verify` · evidence: brief §D · decisions: ADR-0019
GIVEN a verify run that claims `basis: affected`
WHEN the toolchain's test report shows an executed test count greater than or equal to the most recent recorded
whole-suite count for that toolchain at the current version pin
THEN the run is reclassified `basis: whole-suite` with reason `selection-ineffective`
AND a `selection_ineffective` event is emitted
AND — the **bootstrap** clause (ADR-0104 decision 3; PIA-049) — when **no** whole-suite count is recorded for that
toolchain at the current version pin, the `affected` claim is reclassified `basis: whole-suite` with reason
`probe-unverified` and its executed count seeds the comparator; a pin bump resets the comparator.
*Verified by:* unit test on the reclassifier over (selected, total) pairs including the boundary; an integration fixture
reproducing the historical `pnpm test -- --run <file>` shape, which must reclassify rather than report `affected`; a
bootstrap fixture with an empty comparator asserting `probe-unverified` and the seeded count; a pin-bump fixture
asserting the comparator resets.
*Known gap (not repaired here):* this detects only a filter that is fully inert. A filter that drops most of its
arguments but not all — selecting 200 of 236 files — passes this check. AC-327's probe and SM-097's narrowing
distribution are the compensating instruments; neither is a substitute.

**AC-327** — `rk-doctor` · evidence: brief §D (15.6 s targeted vs ~250–900+ s whole suite) · decisions: ADR-0029,
ADR-0014
GIVEN a toolchain pack that declares affected-test selection support
WHEN `redkiln doctor` runs
THEN doctor executes the declared selection command against a single-file probe fixture and fails the pack if the
executed test count is not strictly less than the fixture's whole-suite count.
*Verified by:* a doctor rule-table row; a fixture pack whose filter does not survive forwarding and must fail the rule;
an assertion that the doctor run emits zero harness dispatch events (the rule is deterministic and costs no model spend).

**AC-328** — `rk-port-toolchain` · evidence: §4.1 port rules (`{capability, verified_on, probe}`) · decisions: ADR-0029
GIVEN a toolchain adapter whose `affected-selection` capability has no probe result recorded at the current version pin
WHEN the kernel resolves a verify activity for that toolchain
THEN selection is refused with `StepError::Unsupported("affected-selection")` and the pack routes to an explicit
whole-suite policy
AND no `basis: affected` record can be produced for that toolchain while the probe entry is absent or stale.
*Verified by:* capability-registry unit test; a fixture adapter with an expired `verified_on` that must produce
`Unsupported`; an assertion that no run under that fixture emits a record with `basis: affected`.

**AC-329** — `rk-store-artifacts` · evidence: §3 principle 6 (typed in, typed out), F31 (schema conformance
**52/52 = 100%** [measured]), brief §C.5 · decisions: ADR-0019, ADR-0074 (supersedes ADR-0004)
GIVEN a `VerifyReport` produced by any toolchain adapter
WHEN it is validated at the boundary
THEN it carries `basis`, `basis_reason`, `argv`, `selected_test_count`, `total_test_count`, `wall_secs`,
`toolchain_version` and `suite_sha`, and validation fails if any is absent — on an `affected` run `total_test_count` is
the most recent recorded whole-suite count at that pin and the report names the run it came from
(`total_test_count_source`); on the bootstrap run (AC-326) it is the run's own executed count (ADR-0104 decision 3)
AND no field of the report is obtained by parsing prose or markdown (brief §C.5 forbids parsing JSON out of prose).
*Verified by:* schema conformance test over every adapter's fixture reports; a negative fixture missing each field in
turn; a lint forbidding markdown parsing in the toolchain adapters.

**AC-330** — `rk-doctor` · evidence: brief §D timeout history (900 → 2,400 → 3,600 → one 5,400 s retry; the 900 s default
was written for the fast condition and cost a CLEAN record [documented]) · decisions: ADR-0029
GIVEN a verify activity or slot filler whose policy permits a whole-suite fallback
WHEN doctor checks its timeout
THEN doctor fails if no numeric `timeout_secs` is declared (an inherited default is a failure), and additionally fails
if a whole-suite p95 wall has been recorded for that toolchain and the declared timeout is below it.
*Verified by:* a doctor rule-table row; a lint for absent `timeout_secs`; a fixture pack with a 900 s timeout over a
toolchain whose recorded whole-suite p95 exceeds it, which must fail. Note the bootstrap condition: **no whole-suite p95
exists today** — the field was never recorded [brief §D] — so at first run only the declaration half of this rule can
fire. That is stated as a property of the rule, not hidden in it.

**AC-331** — `rk-kernel-verify` · evidence: F22 (background jobs die; `issue-81` killed 3× as background, 116 s in
foreground [measured, operational; cause unestablished]) · decisions: ADR-0019
GIVEN a verify run killed by its timeout or by the OS
WHEN the gate is evaluated
THEN the result is `Unverifiable` and the gate does not advance the item
AND the outcome is never recorded as a pass or as a test failure attributable to the candidate.
*Verified by:* an integration test that kills the child process mid-run and asserts the recorded outcome class; an
assertion that no `GreenAt(sha)` is minted for a killed run.

**AC-332** — `rk-kernel-verify` · evidence: brief §D (the 26/68 defect was invisible because no instrument distinguished
a filter that matched nothing from one that matched everything — this gloss is reasoning from §D, not a quotation) ·
decisions: ADR-0019
GIVEN a non-empty changed-file set
WHEN the selection resolves to zero tests
THEN the run does not pass: it either escalates to `basis: whole-suite` with reason `selection-empty` or fails,
according to the declared policy
AND "zero tests executed" is never rendered as green.
*Verified by:* unit test over (changed set non-empty, selected = 0) under each declared policy; a fixture whose glob
matches no test file, which must not produce a pass.

### 10.3 Per-AC evidence — one assertion, not one file

**AC-333** — `rk-store-artifacts` · evidence: F25 (an AC is a claim about ONE ASSERTION) · decisions: ADR-0019
GIVEN an acceptance criterion recorded against an item
WHEN verify writes the AC evidence ledger
THEN each AC row carries an assertion locator — file, test identifier and the assertion within that test — and a row
naming only a test file fails validation
AND every AC id in the spec is materialised in the ledger, with an AC that has no evidence materialised as `unpinned`
rather than omitted — the ledger status is **one closed enum per AC**, `pinned | mutation-survived | unpinned |
not-feasible | human-answered`, with `unpinned` reserved for "no evidence" (§9 vocabulary; PIA-032)
AND the ledger includes the six Phase-1 companion kinds' `ledger` record (AC-143).
*Verified by:* schema validation on the AC evidence ledger, including a schema fixture enumerating the closed status
enum with no wildcard; a negative fixture whose row names a file path only, which must fail; a completeness check
asserting the ledger's AC id set equals the spec's.

**AC-334** — `rk-kernel-verify` · evidence: F25 (remove step 7 → RED; invert the ordering → **36/36 GREEN**
[measured, n=1 AC, one operator]) · decisions: ADR-0019
GIVEN an AC whose named assertion is claimed to pin it and whose row carries an **author-declared** `mutation: { patch,
suite }` (ADR-0104 decision 1 — declared with the AC at the spec stage, a reverted-fix patch or one carrying a
discriminating assertion locator per ADR-0096; never derived by a model, which AC-347 forbids; `not-feasible(reason)`
per AC-337)
WHEN the declared patch is applied to the candidate tree at the candidate SHA, the named suite is re-run and the tree is
reverted — deterministic, zero dispatches
THEN the AC is `pinned` only if the named assertion flips from pass to fail
AND if the suite stays green the AC is recorded `mutation-survived`.
*Verified by:* the issue-146-147 AC-015 (F25) ordering-inversion fixture reproduced as a regression test — it must yield
`mutation-survived`, not `pinned`; a unit test on the flip predicate. **This criterion owns the predicate** (ADR-0104
decision 2); AC-317 is the harness invocation that calls it, and SM-094's instrument is this flip predicate.
**Correction (2026-09-11, audit PIA-049).** "The mutation derived from that AC's THEN clause" left the load-bearing
mechanism unspecified — a model would collide with AC-347. The derivation is the author's declaration, recorded as
runbook decision 6.

**AC-335** — `rk-kernel-verify` · evidence: F31 (four adjudicated defects survived fully green suites of 1,890/1,890 and
1,907/1,907 [measured]), brief §C.7 · decisions: ADR-0019, ADR-0074 (supersedes ADR-0004)
GIVEN a suite result of `GreenAt(sha)` for an item whose AC-334 mutation survived
WHEN the AC ledger is computed
THEN that AC **remains `mutation-survived`, never `pinned`**, and the green suite changes nothing about its status
(`unpinned` is reserved for "no evidence" — §9 vocabulary, PIA-032).
*Verified by:* a fixture with a green suite and a surviving ordering mutation, which must leave the AC
`mutation-survived`; a static check that the AC-status function does not take the suite verdict as an argument. (The
consequence for stage exit is AC-346; it is not restated here.)

**AC-336** — `rk-kernel-verify` · evidence: ADR-0097's step memo key (the eight-component tuple AC-014 enumerates);
`GreenAt(sha)` (§4.6) · decisions: ADR-0019, ADR-0097
GIVEN AC evidence recorded at sha N
WHEN verify runs at sha N+1
THEN evidence from sha N does not count toward `pinned` at N+1 unless the **verify step's memo key** — AC-014's tuple,
whose components 4 and 5 carry the resolved inputs (the changed-file set and suite sha among them) and the Toolchain
adapter's detected version string plus the command — is identical; this is the one memo key, not a distinct "evidence
key" sharing the word (PIA-008)
AND a stale-evidence reuse is recorded as a `memo_hit` with its key (AC-019), never as a fresh run.
*Verified by:* memo-key unit test including a toolchain-version bump; an integration test asserting the AC ledger at
N+1 recomputes rather than inherits.

**AC-337** — `rk-port-toolchain` · evidence: §4.2 ecosystem fit ("per-AC mutation evidence is required only where the
toolchain declares mutation feasible"), ADR-0008 · decisions: ADR-0029, ADR-0014
GIVEN a toolchain pack
WHEN it is loaded
THEN it declares mutation feasibility explicitly and an absent declaration is a configuration error
AND where it declares mutation infeasible, an AC records `mutation: not-feasible(reason)` which is not `pinned`.
*Verified by:* a doctor rule-table row for the missing declaration; a fixture pack declaring `feasible = false` whose
ACs must not reach `pinned` by the mutation path.

**AC-338** — `rk-kernel-verify` · evidence: F17 (the E3 reviewer restored the fix from the object store; 42 turns,
526 s, returned `approved` — measured as an instrument defect) · decisions: ADR-0019
GIVEN a mutation applied to produce per-AC evidence
WHEN the suite completes
THEN the harness re-reads the mutated region and records that the mutation was still present at completion
AND a run in which the mutation was reverted or restored before the suite finished is `Unverifiable`, never
`mutation-survived` and never `pinned`.
*Verified by:* the F17 fixture — an agent-restorable mutation — asserted to produce `Unverifiable`; a post-run content
hash of the mutated file compared against the applied mutation.

**AC-339** — `rk-kernel-verify` · evidence: F17, §4.6 (disposable shallow clone at the reviewed SHA) · decisions:
ADR-0019
GIVEN a mutation run for AC evidence
WHEN it executes
THEN it runs in a disposable clone or worktree and the item's own worktree tree hash is identical before and after
AND no mutation artifact is left in the item's git index or working tree.
*Verified by:* tree-hash equality assertion around the mutation harness; `git status --porcelain` empty after the run.

**AC-340** — `rk-kernel-verify` · evidence: F11 (S0.1: 15 mutants, 9 of 10 test-reachable killed, **1 survived**;
test-reachable 90.0%, headline 93.3% [measured; the interval on 1/10 is wide]; **S0.1-SCOPE: 8 of 23 items unmeasured by
construction**, PIA-076) · decisions: ADR-0019, ADR-0074
GIVEN an AC recorded `mutation-survived`
WHEN the verification gate resolves
THEN a finding of class `mutation-survived` is written as an event, and under the pack's default policy weight that
finding blocks
AND the survivor is never dropped because the rest of the suite is green.
*Verified by:* an S0.1-shaped surviving-mutant fixture asserted to emit the finding and to block under the default
weight; a second fixture under an explicitly relaxed weight asserting the finding is still emitted while not blocking.
*Note:* F31's severity call ("narrowed-pin advises") is Ryan's, not a measured result [brief F31], so the default weight
for `mutation-survived` is a decision this criterion enforces, not one the evidence sets.

### 10.4 Independence — verify, CI, and the review gate

**AC-341** — `rk-kernel-verify` · evidence: brief §C.7 ("neither gate is sufficient alone… use the review gate to ADD
findings, never to CLEAR an AC"), §4.6 · decisions: ADR-0019
GIVEN a CI status check reported through the Tracker port
WHEN the verification gate is evaluated
THEN a green CI status never satisfies verify: with CI green and verify `Err`, the item does not advance
AND no verify result writes to the Tracker port's status-check state.
*Verified by:* a static check that the verify gate's input type contains no tracker or CI field; an integration test in
which CI is green and verify is `Err`, which must not advance; a port-call assertion that a verify run issues zero
status-check writes.

**AC-342** — `rk-kernel-verify` · evidence: F29 (a reproduced regression was approved on **3 of 10** draws — S-VAR kill
branch 3 [measured]), F31 (verified false positives **0/23**; adjudication **14/14** confirmed) · decisions: ADR-0074 (supersedes ADR-0004),
ADR-0019
GIVEN a review gate that returns `Findings`
WHEN the AC ledger is computed
THEN a review finding can only add blocking state; no finding can move an AC from `unpinned` to `pinned`
AND a review approval is not an input to any AC's status.
*Verified by:* type-level separation (the AC-status function does not accept `Findings` as an argument); a fixture where
the reviewer reports "all criteria met" over an `unpinned` AC, which must still block.

**AC-343** — `rk-kernel-verify` · evidence: brief §C.7 ("blocking is computed in code from re-executed evidence, never
from the model's `pass`"), F20 (a tool-restricted card silently voids `--json-schema` at exit 0) · decisions: ADR-0074 (supersedes ADR-0004)
GIVEN any artifact reaching the verification gate
WHEN it is validated
THEN a model-authored `pass`, `verdict` or `approved` field is a schema violation
AND the blocking decision is computed in Rust from re-executed evidence only.
*Verified by:* schema test rejecting a `pass` field; a fixture model response carrying `"pass": true` that must fail
validation rather than be ignored.

**AC-344** — `rk-kernel-verify` · evidence: §4.6 point 3 (proposal), brief §C.7 · decisions: ADR-0074, ADR-0019,
ADR-0082
GIVEN a **`reproduced`-class** finding (`kind = run`) raised by the verification gate whose repro record names a command
— the per-class flip rule is **AC-288's (reproduced) and AC-289's (mutation-survived)** in `rk-kernel`, which owns the
block predicate; this criterion applies it to the verification gate's own findings and adds the sink routing (PIA-063;
§0.3 authority row)
WHEN the deterministic gate re-executes it
THEN it blocks only if it fails on the candidate and passes on the reference
AND both-fail or cannot-run resolves to `Unverifiable { both_fail | cannot_run }` and is routed to the advisory sink,
never to a block and never to a clear.
*Verified by:* unit test over the four (candidate, reference) outcome pairs; a cannot-run fixture (missing binary) that
must yield `Unverifiable`; an assertion that the advisory record is durably written and retrievable; an architecture
assertion that the predicate is `rk-kernel::policy`'s, not a second copy in the verify module.
*Dependency:* brief §C.8 records that **no Advisory sink exists** — the 7 adjudicated-true `narrowed-pin` findings still
have no destination. This criterion is unmeetable until that sink is built (AC-310, ADR-0082); it is a requirement on
the sink, not an assumption that one is there.

**AC-345** — `rk-kernel-verify` · evidence: §4.6 point 5 (quarantine mechanism, proposal only) · decisions: ADR-0074 (supersedes ADR-0004),
ADR-0019
GIVEN a `Quarantined { reason, by, until }` finding
WHEN the AC ledger is computed
THEN quarantine stops the finding from blocking **review** and from being re-raised until `until` — it is a
**review-gate-only** state (ADR-0104 decision 4; SM-095 likewise) — and does not change the AC's evidence class or
ledger status
AND the quarantine is an event carrying who and until when.
*Verified by:* a fixture quarantining a `mutation-survived` finding, asserting the AC remains `mutation-survived` while
the review gate does not block; an event-log assertion for `by` and `until`; an assertion that stage exit (AC-346) is
still refused for that AC.

**AC-346** — `rk-kernel-verify` · evidence: §3 principle 4 (proposal); F25 (the mechanism: green at test granularity is
not evidence at assertion granularity) · decisions: ADR-0019
GIVEN an AC that is neither pinned by a flip-verified assertion nor answered at a human gate (AC-357) explicitly for
**that AC** — a quarantine of the survivor's finding (AC-345) is not an answer and does not satisfy this criterion
(ADR-0104 decision 4; PIA-049)
WHEN the stage attempts to exit
THEN the exit is refused and the unpinned AC ids are named in the refusal
AND the refusal is an event, not only a console line.
*Verified by:* an integration test over a two-AC fixture with one unpinned AC; a fixture where the survivor's finding is
quarantined and exit must still be refused; an assertion that the refusal event names the AC id.

### 10.5 Selection semantics — determinism, composition, and no model in the path

**AC-347** — `rk-kernel-verify` · evidence: F36 (the LLM-emitted context vector produced the same modal required
artifact set on only **5/20, 3/20, 2/20 and 2/20** against an 80% line; the pre-registered anchor-rewrite repair did not
fix it — kill fired **4 of 4** [measured, 400 dispatches, 0 void]) · decisions: ADR-0019, ADR-0030/0061
GIVEN a verify activity
WHEN affected-area selection is computed
THEN no harness dispatch participates: the selection is a pure function of the changed-file set, the toolchain config
and `facets.toml`
AND a verify run that emits any harness dispatch fails the activity.
*Verified by:* an event-log assertion that a verify run emits zero `dispatch` events; a lint that the verify activity's
step kinds contain no agent step; a fixture pack attempting an agent step at `verify.pre-test` that must fail lint (the
positive fixture is what keeps this from passing vacuously).
*Scope note:* F36's arm D precision/recall was not computed at F36; **F39 computes it** [measured, 140 truth cells at
pin `47faec9f`, all scored on the fixed re-run — E-0089-2]. It measures the *derived* vector, so the licensed claim about
the **LLM-emitted** vector is still that it is unstable, not that it is inaccurate. What F39 licenses is the separate
claim that the *derived* vector is inaccurate — `auth` 0/12 [0.00, 0.24], all positives false — which is AC-070's and
AC-520's subject, not this criterion's.

**AC-348** — `rk-kernel-verify` · evidence: brief §C.9 / F36 (per-facet alpha looks healthy on 4 of 8 axes while `R`
almost never repeats) · decisions: ADR-0019
GIVEN identical inputs — the same changed-file set, toolchain version, selection policy version and suite sha
WHEN the selection is computed twice
THEN the two selections are byte-identical.
*Verified by:* a property test running the selector twice over 200 synthetic change sets asserting equality; a replay
test that recomputes selection from the event log.

**AC-349** — `rk-kernel-verify` · evidence: §4.2.1 binder rule 1 ("two candidates at equal specificity are a
configuration error, never a guess") · decisions: ADR-0073 (superseding ADR-0029 on the tie-break), ADR-0014
GIVEN two selection rules or two toolchain claims matching the same changed path at equal specificity
WHEN the binder resolves selection
THEN it is a configuration error reported at plan time, never a silent choice
AND the error names both rules and the path.
*Verified by:* binder unit test with two equal-specificity fixtures; a doctor rule-table row; an assertion that the
error message contains both rule ids.

**AC-350** — `rk-kernel-verify` · evidence: §4.2 (a repo may compose several toolchains — a Node front-end and a .NET
API) · decisions: ADR-0014, ADR-0073
GIVEN a changed-file set spanning more than one configured toolchain
WHEN verify runs
THEN every changed path is accounted for: one verify result is produced per claiming toolchain, each with its own basis,
the gate is their conjunction, and a path claimed by no configured toolchain fails the activity as a configuration error
rather than being silently excluded.
*Verified by:* an integration fixture with Node and .NET paths asserting two results and one conjunction; a fixture with
an unclaimed path (e.g. `infra/*.tf`) that must error; an assertion that the union of the results' claimed paths equals
the changed-file set.

**AC-351** — `rk-port-toolchain` · evidence: ADR-0061 frontmatter (facets.toml is the normative source for the toolchain
and change facets); brief §E contested (`binary_change` is measured but unregistered in `facets.toml`, so it can feed no
require row; `dynamism` is pre-registered with no registry entry) · decisions: ADR-0061, ADR-0030
GIVEN a selection rule or require row referencing a change facet
WHEN packs are linted
THEN a reference to a facet with no live registry entry in `facets.toml` is a lint error naming the facet.
*Verified by:* the pack lint over a fixture referencing `binary_change` — unregistered at this pin [brief §E], so the
fixture must fail today and must be updated deliberately if it is ever registered; a registry-completeness test
asserting every referenced facet has a live table row.

**AC-352** — `rk-kernel-verify` · evidence: brief §D, §4.6 (B3) · decisions: ADR-0019
GIVEN a whole-suite fallback
WHEN the gate record is written
THEN `basis` and `basis_reason` are both closed enums — reasons being `no-selection-support` (the pack declares no
affected-selection support), `probe-unverified` (AC-328's registry entry is absent or stale at the pin, **or** AC-326's
bootstrap: no whole-suite count recorded yet — AC-328 emits this variant), `selection-empty` (AC-332: a non-empty
change set selected zero tests), `selection-ineffective` (AC-326: the filter was inert), `policy-forced` (the pack's
policy demanded whole-suite regardless), `toolchain-unsupported` (the claiming toolchain has no verify adapter) —
matched exhaustively with no wildcard arm, so a new variant is a compile error (definitions added 2026-09-11, PIA-032)
AND a fallback with no reason fails validation.
*Verified by:* exhaustive `match` over both enums (no wildcard arm); a negative fixture with basis `whole-suite` and no
reason, which must fail; a capture test recording the pair for one run of each basis variant, which fails if a kernel
change moves a run from one variant to another.

**AC-353** — `rk-kernel-verify` · evidence: F22 (~12 kills vs one 12,000 s survivor under `Start-Process`; `issue-81`
killed 3× as background, 116 s in foreground [measured, operational; cause unestablished]), F38 (mid-graph resume
0 changed, 0 lost, digests identical across `taskkill /T /F` [measured, n=3, **PARTIAL** — 65-min and 24-h arms and all
3 controls unrun]), brief §C.3 (`reconcile()` on every entry) · decisions: ADR-0017, ADR-0019
GIVEN a verify run **in progress**
WHEN the orchestrating process is killed
THEN the run is resumable from a durable mark and nothing depends on the toolchain process having survived
AND resumption re-derives the changed-file set from the world rather than trusting the recorded set (this is resume, not
the audit replay of AC-321, which must not touch the working tree).
*Verified by:* a kill-and-resume integration test (`taskkill /T /F` shaped) asserting the resumed run reaches the same
gate decision; an assertion that `reconcile()` runs on every verify entry; an assertion that resume reads the working
tree while replay does not.

**AC-354** — `rk-kernel-verify` · evidence: F19 (174/174 lossless at activity granularity, drift 12/174 = 7%, RR 3.6 with
CI [1.0, 12.9] on gates over 1 h — directional, intervals overlap; **limit: the FSM was never replayed** [measured]) ·
decisions: ADR-0019
GIVEN a completed verify run's event stream
WHEN it is replayed
THEN the replay yields the same gate decision, the same basis and the same AC ledger
AND a replay divergence fails the build.
*Verified by:* a replay-equivalence test over a recorded corpus of verify runs. This asserts a property F19 does not
demonstrate — the FSM has never been replayed — so the test is the first evidence, not a reproduction.

**AC-355** — `rk-kernel-verify` · evidence: §4.6 point 7 (negative controls stay in the corpus, proposal) · decisions:
ADR-0019
GIVEN a change whose correct basis is `whole-suite` (a lockfile or shared-config change) and a change whose correct
basis is `affected` over a single file
WHEN the selector classifies each
THEN both classify correctly on basis and on selected-test count
AND a selection-policy edit that misclassifies either fails the build.
*Verified by:* the two-fixture selection regression suite held in CI, asserted on basis and on selected-test count. Its
purpose is anti-vacuity: a selector that always answers `whole-suite` fails the second fixture, and one that always
answers `affected` fails the first.

### 10.6 Provisional and human-gated

**AC-356** — PROVISIONAL (gated on **S-BIN**, registered and unrun [brief §B, §E]; S-BIN gates the `ChangeSummary` type
design and the Toolchain port's `change_summary` body, not Phase-1 entry — ADR-0093 decision 4) — `rk-port-toolchain` ·
evidence: §4.2 ecosystem fit (binary assets in LFS represented by manifest + captures, not diff), ADR-0008 · decisions:
ADR-0008, ADR-0029, ADR-0019
GIVEN a changed-file set containing paths the toolchain classifies `Binary`
WHEN the `ChangeSummary` is built for verify and for the reviewer
THEN binary paths are represented by manifest plus capture references and never by text diff.
*Verified by:* a `classify(path)` unit test over a fixture with `.uasset`/`.umap` paths; a `ChangeSummary` schema test.
*Struck 2026-09-11 (PIA-036):* the AND clause "per-AC mutation evidence is not required for a `binary`-class AC; its
evidence is 'asset touched + test/capture result'" — a design sketch under the S-BIN block, not a criterion: it exempted a
class from AC-334 with no instrument, and the design had refused that exemption. Whether a binary-class AC gets a
mutation path is AC-337's `not-feasible(reason)` route until S-BIN reports.
**PROVISIONAL because** the `ChangeSummary` type design is blocked on S-BIN, which is registered and has not run; the
field names above must not be frozen before it does (`ChangeSummary` is an opaque, non-finalised newtype in `rk-ports`
until then — ADR-0099 clause 5). Do not promote this criterion to binding on the strength of this document.

**AC-357** — HUMAN GATE — `rk-kernel-verify` · evidence: F24 (6/6 rejections confirmed factually true at zero model
cost; **severity settled 2026-09-09 on a NOT-BLIND, model-filled sheet at Ryan's direction, κ forfeited** — PIA-081),
F31 (adjudication 14/14 confirmed; the "narrowed-pin advises" severity call is also Ryan's) · decisions: ADR-0019,
ADR-0104
GIVEN an AC whose toolchain declares mutation infeasible (AC-337), **or an AC recorded `mutation-survived` whose finding
was quarantined** — the quarantine's `by`/`until` is not the answer; the human answers *this AC* explicitly here or the
AC stays unpinned (ADR-0104 decision 4; PIA-049)
WHEN the gate needs to know whether the named assertion actually discriminates that AC
THEN the question is routed to a human gate node carrying the AC text, the assertion source and the suite result
AND the gate's answer is recorded as the AC's evidence with `by` and `at`.
*Human-gated because* deciding whether an assertion discriminates a criterion, with no executable mutation available, is
a judgement about meaning; F24 and F31 both show severity judgements in this corpus remained Ryan's even where the facts
were mechanically settled. *Machine-checkable part:* that the gate node is created with those three payload fields, and
that no AC reaches `pinned` down this path without a recorded answer carrying `by` and `at`.

**AC-358** — HUMAN GATE (adjudication) — `rk-kernel-verify` · evidence: §4.6 point 6 ("false-block is measured against
adjudication, never against historical approval"); brief §E contested (**CLEAN's ground truth is wrong for ≥13 of 23 =
57%**, so every false-positive rate measured against it is an UPPER BOUND) · decisions: ADR-0074 (supersedes ADR-0004), ADR-0019
GIVEN a verify block that a human disputes
WHEN the false-block telemetry is computed
THEN the denominator is adjudicated blocks only, and a block never counts as false because the item was historically
approved
AND an unadjudicated block is excluded from the rate rather than counted either way.
*Human-gated because* the adjudication itself is a judgement. *Machine-checkable part:* a unit test on the rate
computation asserting unadjudicated blocks are excluded from both numerator and denominator, and that no
historical-approval field is an input to it.

**AC-359** — `rk-kernel-verify` · evidence: F18 (gate→verdict median 10.5 min, 45% > 1 h, n=174; stage→verdict median
51.6 min, p90 ~9 h, max 226 h, n=191 [measured, **one operator**]), F19 (drift 12/174 = 7%, 11% on gates over 1 h),
F28 (a resume costs a flat 18,603 tokens at both 65.2 min and 49.7 h under the default prompt [measured; the brief's
correction attributes the flatness to shared-prefix subsidy, not durability]) · decisions: ADR-0017, ADR-0019
GIVEN a human gate opened by the verification gate (AC-357 or AC-358)
WHEN the answer arrives
THEN it is void unless its tree hash still matches the tree the human saw, and a drifted item re-opens the gate with the
drift attached
AND the verify evidence recorded before the drift is not carried across as `pinned`.
*Verified by:* the gate-record tree-hash test; a drift fixture (a commit landing between open and answer) that must void
the answer and re-open; an assertion that the AC ledger is recomputed after re-open.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-097 | Selection narrowing ratio | — |
| SM-098 | Declared whole-suite fallback rate, by reason | DESCRIPTIVE REPORT, NOT A SUCCESS MEASURE; no firing kill; no baseline |
| SM-099 | Silent-selection-failure detections | no baseline |
| SM-100 | Per-AC pinned coverage | no baseline |
| SM-101 | Mutation-survived rate | — |
| SM-102 | Verify wall clock and timeout kills | no baseline |
| SM-103 | Model dispatches inside verify | saturating |
| SM-104 | False-block rate against adjudication | — |
| SM-105 | Selection-probe coverage and freshness | no baseline |
| SM-106 | Human-gate load created by the verification gate | — |
| SM-107 | Selection determinism | no baseline |

#### Residual risk this area could not repair

- AC-326 detects only a FULLY inert filter. A wrapper selecting 200 of 236 files is recorded as `affected` and passes every criterion in section B. The compensating instruments are detection-after-the-fact, not gates. This is the largest hole left in the 60× defect defence.
- Three criteria depend on data or components that do not exist: AC-330's p95 half cannot fire (no whole-suite wall was ever recorded — AC-326's bootstrap seeds the count comparator, not the wall), AC-344 routes to an advisory sink §C.8 says does not exist, and AC-356 is PROVISIONAL on S-BIN (scoped to the `ChangeSummary` type, ADR-0093).
- Roughly a third of the surviving criteria rest on proposal sections (§4.1, §4.2, §4.2.1, §4.6, principle 4) that could not be verified — only the evidence brief was in scope for the refutation pass.
- SM-100's ≥0.90 target and SM-104's ≤1% target have NO baseline. SM-104 is worse than it looks: the honest Wilson bound on 0/23 is [0%, 14.3%], so the evidence does not exclude a false-block rate fourteen times the target.
- AC-355 is a statement about the fixture corpus rather than about the system. It survives only because it is the area's sole anti-vacuity control — dropped as 'a test, not a criterion', AC-322, AC-326 and AC-332 become vacuous.
- AC-334's mutation-derivation step was left unspecified until 2026-09-11; ADR-0104 fixes it as the AC author's declared `mutation:` field (runbook decision 6). The residue moved: a declared mutant is only as discriminating as its author made it, and ADR-0096's reverted-fix / discriminating-assertion rule is what stops an equivalent mutant blocking — nothing here measures how often authors declare a vacuous one.
- AC-327, AC-330 and AC-350 were folded rather than split and still carry two testable properties in one id; traceability from a failing test back to a criterion will be ambiguous for those three.

---

## 11. Event log, cost attribution, telemetry and throttling

**Area key:** `events-cost` · **Criteria:** AC-360..AC-399 (all 40 ids consumed; AC-397..AC-399 are splits) — **40 in force** · **Measures:** SM-109..SM-118 (SM-119, SM-120 unallocated)
**State:** 38 binding · 1 provisional (AC-378) · 1 human-gated (AC-396)
**Sources:** proposal §4.5 (tier 1 / tier 2), §4.7 (cost and parallelism); §4.1 and §4.3 for the `EventStore` port contract and the adapter's rate-limit / cache rules; ADR-0003, 0016, 0017, 0018, 0022, 0036, 0037.

*Scope note.* Citation convention corrected in refutation: `C.n` means item *n* of the brief's section C. 'Principle 11' had been used five times for the method finding, which is **C.12**; C.11 is isolation-as-measurement-precondition, and C.6 is process survival (C.3 is drift). Citations to `L*.*` ('shipmates'), `R7`/`R8` and `Backlog.md #558` were removed throughout: they appear nowhere in the evidence brief and could not be checked.

### 11.1 Attribution: one child, one event, one cost row

**AC-360** — `rk-kernel` / `rk-adapter-claude` · evidence: F1, F3, F6 [measured]; §4.7 [documented] · decisions: ADR-0003, ADR-0022
GIVEN an activity that performs one harness dispatch
WHEN the dispatch terminates for any reason — success, `StepError`, cancellation or timeout
THEN exactly one dispatch event is appended carrying every token counter the harness reports verbatim, `total_cost_usd`,
the `five_hour` and `seven_day` headroom readings taken before and after, and the pinned harness/CLI version
AND no dispatch path exists that can terminate without appending that event.
*Verified by:* a kernel test that drives every `StepError` variant of ADR-0095's table through the dispatch path and
asserts one event each — the event arms are the table's (`step_failed { class }`, `step_cancelled`, `rate_limited`,
`step_refused { class }`; ADR-0105's event-kind table carries them); an exhaustive-match test over `StepError` so a new
variant fails to compile without an event arm; the `EventStore` fake (ADR-0003) as the assertion surface. The
`DispatchRecord` half of ADR-0099's `DispatchOutcome` (argv digest, env digest, instrument version, usage, timings,
raw retention per ADR-0105 clause 7 — argv and environment as digests only, the child environment never) is what this
event carries.
*Repaired:* the original said "all four token counters"; the brief enumerates no such count (F3 establishes only that the
naming is plain and carries no TTL split), so the criterion now binds to what the harness reports rather than to a number.

**AC-361** — `rk-kernel` · evidence: §4.7 [documented: "attribution is free — every activity is its own child"] ·
decisions: ADR-0022
GIVEN an item whose run dispatched N activities
WHEN the item's cost is rolled up
THEN the rollup is the sum of the N child events and nothing else
AND no apportionment, allocation or estimated-share field exists on any parent record.
*Verified by:* a unit test asserting rollup == sum over a fixture run; a lint/grep rule in CI forbidding the identifiers
`apportion`, `allocated_cost`, `estimated_cost` in `rk-kernel`; a type-level check that the parent record has no cost
field of its own.

**AC-362** — `rk-adapter-claude` · evidence: F3 [measured: `ephemeral_1h_*` / `ephemeral_5m_*` absent from `usage`;
charter E1's TTL-split metric is not obtainable] · decisions: ADR-0003
GIVEN a `usage` block from the harness that does not report a field the event schema knows about (e.g. any cache-TTL split)
WHEN the event is constructed
THEN the field is recorded as absent, and the type that carries it is an `Option` with no `Default` impl and no
`unwrap_or(0)` at any call site, so a consumer cannot read absent as zero without a visible match.
*Verified by:* a fixture replay of a recorded `usage` block with the TTL fields missing, asserting `None` not `Some(0)`;
a serialiser test that absent and zero round-trip distinguishably; a CI grep rule forbidding `unwrap_or(0)` /
`unwrap_or_default()` on the usage struct's fields **and a `#[derive(Default)]` on `Usage` or `CacheCreation` — the
grep's scope explicitly includes `rk-adapter-claude`'s lifted `event.rs`, whose spike form carries `Default` and
`unwrap_or(0)` (`spikes/runner/src/event.rs:124`, `:163` [measured]); `ttl_split()`, `cache_read_share()` and
`total_input()` return `None` / propagate absence when a field is absent, rather than being deleted (PIA-062)**.
*Repaired:* the original's "no consumer may treat absent as zero" named no instrument that could catch a consumer doing
it; the obligation is now discharged by the type and a grep rule that can actually fail.

**AC-363** — `rk-kernel` · evidence: §4.3 degradation row [documented — no spike measured model substitution] ·
decisions: ADR-0003
GIVEN a pack that permits a model substitution (e.g. Opus → Sonnet)
WHEN an activity dispatches under the substituted model
THEN the event and the activity digest both record the substitution with its origin, target and reason
AND a dispatch that substituted without such a record is refused at digest-seal time.
*Verified by:* a kernel test on the substitution path; a digest-seal assertion that model-on-event equals model-on-card
unless a substitution record is present.

**AC-364** — `rk-store` · evidence: §4.5 rule 2 [documented] · decisions: ADR-0003, ADR-0018
GIVEN an appended event
WHEN any later operation attempts to modify or delete it
THEN the store refuses the write
AND a correction is expressible only as a new appended event that references the corrected one.
*Verified by:* a store trait test asserting no `update`/`delete` method exists on the `EventStore` port; an integration
test that a correction produces two rows.
*Cut:* the F12 citation ("`reconcile.rs` validated on 360 trailer commits / 74 slices"). F12's 360 commits / 74 slices are
real, but F12 measures how often a slice passed through a rejected state — it validates no reconciler, and its own
registered primary was uncomputable at n=1. The append-only rule stands on §4.5 alone and needs no measurement.

### 11.2 Tier 2 is rebuildable-or-disposable

**AC-365** — `rk-kernel` · evidence: §4.5 tier test [documented]; F19 [measured: 174/174 lossless, drift 12/174 = 7%] ·
decisions: ADR-0003
GIVEN a repository whose event store file has been deleted
WHEN `redkiln run <item>` is invoked
THEN the next action it takes is identical to the action taken with the store intact
AND the store is repopulated from git without operator intervention.
*Verified by:* a replay test in the kernel crate that runs a fixture corpus twice — once with the store, once with the
store deleted — and asserts equal next-action sequences. **Limit:** F19 establishes losslessness of the *state carrier*,
not of the FSM; the FSM was never replayed in the spike, so this criterion is the first place "same next action" is
actually proven. The criterion itself is a requirement, not a finding — F19 licenses the expectation, not the result.

**AC-366** — `rk-store` / CI · evidence: §4.5 rule 3 [documented] · decisions: ADR-0003
GIVEN a clean clone of a repository at a given SHA
WHEN `redkiln doctor --rebuild` runs
THEN every tier-1 fact (stage transitions, gate opened/answered, verdicts, slice seals) is reconstructed from git and
diffs equal against the pre-delete tier-1 projection.
*Verified by:* a CI job that clones, rebuilds and diffs tier-1 projections against the pre-delete snapshot, failing on any
difference.

**AC-397** — `rk-store` / `redkiln doctor` · evidence: §4.5 rule 3 [documented] · decisions: ADR-0003
*(Split from AC-366: reconstruction fidelity and the classification of the missing tier-2 rows are independent behaviours
with independent failure modes — a rebuild can be faithful and still report its own expected emptiness as an error.)*
GIVEN a rebuilt store from which tier-2 telemetry (usage, cost, headroom, wall clock, drift observations, inner retries)
is necessarily absent
WHEN `redkiln doctor` evaluates the rebuilt store
THEN the tier-2 emptiness is reported with status `ok` and an explicit "expected after rebuild" reason
AND no rule row reports it as an error or a warning.
*Verified by:* a doctor rule-table test over a freshly rebuilt fixture store asserting the row's status and reason string.

**AC-367** — `rk-kernel` · evidence: §4.5 tier split [documented] · decisions: ADR-0003
GIVEN the event enum
WHEN the crate is compiled
THEN every variant carries an explicit tier tag, and every tier-1 variant is matched by the git trailer writer
AND adding a variant without a tier tag or without a trailer arm fails to compile.
*Verified by:* an exhaustive `match` over the event enum in the trailer writer (non-exhaustive match is a compile error);
a unit test enumerating tier-1 variants against the trailer grammar.
*Cut:* the "principle 11 / F28 [measured]" citation. F28 measures human-gate cache cost and the H4 digest result; it
licenses nothing about tier tagging, and the method finding it was standing in for is C.12. A compile-time rule needs no
empirical support and was weakened by pretending to have some.

**AC-368** — `rk-store` · evidence: §4.5 tier 2 "disposable" [documented] · decisions: ADR-0003
GIVEN a tier-2 append that fails (disk full, lock contention, corrupt store)
WHEN the failure is observed
THEN the activity continues and completes, and the failure is surfaced as a warning with a counter
AND no tier-2 append failure is ever mapped to a `StepError` that fails the activity.
*Verified by:* a fault-injecting `EventStore` fake (ADR-0003 makes the fake a hard requirement) that fails every append,
asserting the fixture run still completes; a test asserting the tier-1 path does the opposite.

**AC-369** — `rk-store` · evidence: §4.5 rule 5 [documented] · decisions: ADR-0003
GIVEN a store whose identity rows (root commit + common-dir, schema version, harness versions) do not match the
repository it is opened against
WHEN the store is opened
THEN the open is refused with a typed error and nothing is written
AND no "adopt this store" fallback path exists.
*Verified by:* a unit test opening a store against a second fixture repo; a `doctor` rule-table row.
*Cut:* the "Backlog.md #558, shipmates L5.3" citation (not in the evidence brief, uncheckable) and "boot-id" from the
identity-row list (no source in the brief or the cited proposal sections; the remaining four rows are §4.5's own).

**AC-370** — `rk-store` · evidence: §4.5 rule 1 [documented] · decisions: ADR-0018, ADR-0003
GIVEN a repository with one or more linked worktrees
WHEN any worktree opens the store
THEN it opens exactly one store located at `git rev-parse --git-common-dir`/redkiln/
AND a store file discovered under a linked worktree's own `.git` directory is reported by `doctor` as a configuration
error, not silently used.
*Verified by:* an integration test creating three worktrees and asserting one store path and one set of rows; a `doctor`
rule row for the stray-store case.
*Cut:* the F13 citation. F13's ~17,439 shared + ~7,827 per-directory segmentation and 19/19 fresh repos are real numbers
about the *prompt prefix cache*; they say nothing about where a store file lives, and the gloss "worktrees are
first-class in this design" is a design intention, not F13's claim.

**AC-371** — `rk-store` · evidence: §4.5 rule 2 [documented]; F12 [measured: sealed >once 21.6% of 74 slices — re-seals
are common] · decisions: ADR-0003, ADR-0018
GIVEN a repo-referencing event row
WHEN it is written
THEN it is keyed by SHA or tree hash and by no branch name or path.
*Verified by:* a schema test asserting no branch/path column is part of any key.
*Repaired:* the original gloss read "rebases and re-seals are the normal case". F12 measures re-seals (21.6%) and
ever-rejected slices (15/74 = 20.3%); it measured no rebases. The claim is narrowed to what was counted.

**AC-398** — `rk-store` / drift report · evidence: §4.5 rule 2 [documented]; F19 [measured: drift 12/174 = 7% overall,
RR 3.6 (CI [1.0, 12.9], directional) on gates over 1 h] · decisions: ADR-0003, ADR-0018
*(Split from AC-371: key shape and orphan handling are independent — a correctly keyed row can still be garbage-collected
when its object disappears, which is the failure this half exists to forbid.)*
GIVEN a repo-referencing event row whose referenced object is orphaned by a rebase, reset, amend or force-push
WHEN the drift report is produced
THEN the row is surfaced as drift with its original values intact
AND it is never deleted, rewritten or silently dropped from the report.
*Verified by:* an integration test that rewrites history and asserts the orphaned rows appear in the drift report with the
original count intact, and that the store's row count is unchanged before and after.

### 11.3 Headroom is quantised: per-run cost is NOT measurable

**AC-372** — `rk-adapter-claude` · evidence: F6 [measured: `utilization` quantised to 0.01; `five_hour` and `seven_day`
proven independent, not mirrors]; F14 [measured: 40 dispatches / **~1.0M tokens** (854k cache-read, 157k
cache-creation; "~1.9M" was a miscount — PIA-073) moved `seven_day` 0.100 → 0.100 — the gate was struck, not answered];
F29 [measured: the per-window split as the reporting shape] · decisions: **ADR-0110** (ADR-0003 related)
GIVEN a headroom delta computed between two `rate_limit_event` readings that **share one `resetsAt`** — a batch that
crosses a window reset is split per window and the partial window is reported with its own `n`, as F29 did — over an
**explicitly named dispatch set** that is the account's complete dispatch sequence between the two readings, with `n` on
it; `utilization` is parsed from the JSON decimal into an **integer tick count** (hundredths, `u16` — AC-374), and the
delta is `delta_ticks`
WHEN `delta_ticks <= 1` (the absolute change is at most one instrument tick)
THEN the emitted reading carries `resolution_limited: true`
AND a set with `n < 10` is reported `low_sample` and no delta is quoted (SM-111's floor, mirrored from AC-382; also on
AC-395)
AND `redkiln cost <item>` prints a per-item delta only when the item's dispatches were the only traffic in the set, and
otherwise prints `not attributable` with the interleaving count
AND the `redkiln cost` and `redkiln status` renderers print the flags alongside any reading that carries them.
*Verified by:* a unit test over the delta calculator in ticks at 10/10 (flagged), 10/11 (flagged — a change of exactly one
tick is not distinguishable from quantisation), 10/13 (not flagged), and at the decimal bases **0.09/0.10, 0.10/0.12,
0.20/0.21** — the cases a naive `f64` subtraction mis-flags (`0.10 - 0.09` is not `0.01` in binary floating point; 80 of
100 one-tick deltas mis-flag under `delta < 0.02` — PIA-017); a reset-crossing fixture asserting two per-window deltas
each with its own `n`; an interleaved-traffic fixture asserting `not attributable` and the count; an `n = 9` fixture
asserting `low_sample` and no delta; snapshot tests of the two named renderers asserting the flags appear in the output
string; a doctor rule that a flagged reading is never rendered bare. `throttle.rs`'s `batch_draw` is **rewritten, not
lifted** (its `delta < 0.02` on `f64` is the defect). *(Amended 2026-09-11, PIA-017/PIA-061 — one edit: ticks, the
`resetsAt` window rule, the named set and the `low_sample` floor, per ADR-0110.)*
*Repaired, twice (2026-09-08):* (a) the original paired a "≤ 0.01" rule with 0.100/0.110 as its *negative* test case — that
pair is exactly one tick and the rule flags it, so the named instrument contradicted the criterion it was meant to verify;
(b) "every rendering surface" was unbounded and unverifiable by the one snapshot test named, so the obligation is bound to
the two surfaces that exist. *And a third time (2026-09-11):* the decimal-fraction rule certified a defective `f64`
implementation on every named vector.

**AC-373** — `rk-kernel` · evidence: F6 [measured]; F29 [measured: per-draw `five_hour` **0.0098** as a mean over 60 draws,
0 void; F21's 0.077 is superseded, 7.8× too high] · decisions: **ADR-0110** (ADR-0003 related)
GIVEN the cost and telemetry API
WHEN a caller asks for the headroom drawn by a single run or a single dispatch
THEN no such accessor exists — the only headroom API is a batch delta over an explicitly named set of dispatches with
n on it
AND a per-run headroom field on any event or projection is a compile error.
*Verified by:* the absence of the method on the trait, asserted by a compile-fail (`trybuild`) test; a CI grep rule
forbidding `headroom_for_run` / `run_headroom_cost` identifiers. This is the negative F21 taught: a per-draw figure read
off a small sample was wrong by 7.8× [measured].

**AC-374** — `rk-adapter-claude` · evidence: F1 [measured: readable via the undocumented top-level `rate_limit_event`];
F6 [measured]; the Agent SDK TypeScript reference now types a top-level `rate_limit_event` with a `rate_limit_info`
object (`status`/`resetsAt`/`utilization`) — but not `unifiedWindows`, which stays undocumented for the fields this
instrument reads [documented, code.claude.com/docs/en/agent-sdk/typescript, fetched 2026-09-10; grounding
EXT-CLAUDECO-024] · decisions: **ADR-0110** (ADR-0003 related)
GIVEN a `rate_limit_event` payload
WHEN its `unifiedWindows.{five_hour,seven_day}.utilization` values are stored
THEN they are parsed from the JSON **decimal digits** into an integer tick count (hundredths, `u16`) and stored so —
"verbatim" means the decimal digits as reported, never an `f64` — at the resolution reported
AND no interpolation, smoothing, rescaling or extrapolation to finer resolution is performed anywhere in the pipeline.
*Verified by:* a round-trip test from a recorded payload fixture asserting the stored ticks re-render to the recorded
decimal digits byte-for-byte; a type test that the stored field is an integer tick type and no `f64` utilisation field
exists; a CI grep rule forbidding `interpolate`/`smooth` in the headroom module. *(Amended 2026-09-11, PIA-017.)*

### 11.4 Throttling on the visible window — the family sub-limit is invisible

**AC-375** — `rk-adapter-claude` · evidence: F1 [measured: **no Opus/Sonnet family sub-limit key** exists in
`unifiedWindows`; the binding limit is invisible]; F6 [measured] · decisions: **ADR-0110** (ADR-0003 related)
GIVEN the throttle decision path
WHEN it computes whether a dispatch may proceed
THEN it consults only `unifiedWindows.five_hour` and `unifiedWindows.seven_day`
AND no code path reads, derives, estimates or synthesises a per-model-family headroom value.
*Verified by:* a CI grep/lint rule over `rk-adapter-claude` forbidding a family-keyed headroom identifier; a unit test on
the throttle input struct asserting it has exactly two window fields. **This is the criterion that must become
impossible to violate:** the binding limit is the one the instrument cannot see, so an invented family figure would be a
confident fiction [measured, F1].

**AC-376** — `rk-adapter-claude` · evidence: F1, F6 [measured]; §4.3 rate-limit row [documented] · decisions: **ADR-0110**,
ADR-0095 (ADR-0003 related)
GIVEN a configured margin `margin_ticks` on a visible window
WHEN the last-read utilisation for that window, in ticks, satisfies `ticks >= 100 - margin_ticks`
THEN the adapter refuses to dispatch and returns `StepError::RateLimited { resets_at }` without spawning a process
AND the refusal is appended as a `rate_limited` event.
*Verified by:* a unit test on the throttle in ticks with a scripted `rate_limit_event` at the boundary and one tick below
it, and at the decimal margins 0.07 and 0.18 against bases 0.09/0.10/0.20 (the `f64` mis-flag cases — PIA-017);
an assertion that no child process was spawned in the refusing case; an assertion that the refusal row is present in the
`EventStore` fake (a refusal never reaches AC-360's dispatch path, so it needs its own assertion).

**AC-377** — `rk-adapter-claude` / `redkiln doctor` · evidence: F6 [measured: resolution 0.01, so one tick = 0.01];
§4.3 "margins ≥2 ticks" [documented] · decisions: **ADR-0110** (ADR-0003 related)
GIVEN a configuration whose window margin, parsed to ticks, is `margin_ticks < 2` — and the margin comes from the pack,
never from a code default: `Margins` implements no `Default` (the lifted `throttle.rs:35` carried `max_five_hour: 0.90` as
a code margin — PIA-062)
WHEN the configuration is loaded
THEN it is refused as a configuration error before any dispatch
AND `redkiln doctor` reaches the same verdict without dispatching a model call.
*Verified by:* a config-validator unit test with margins 0.005 / 0.01 / 0.02 (1 / 1 / 2 ticks — the decimal parse must not
round 0.01 to 2 or 0.02 to 1) and at 0.07 / 0.18; a fixture config that must fail; a fixture config with no margin that
must fail (no default); a compile-fail or grep rule that `Margins` implements no `Default`; a `doctor` integration test
with the harness adapter replaced by a fake that panics on dispatch. *(Amended 2026-09-11, PIA-017/PIA-062.)*
*Repaired:* "at zero model cost" was asserted with only a doctor rule-table row as its instrument, which cannot observe
cost. The panicking-adapter fake makes the zero-dispatch claim actually falsifiable.

**AC-378** — `rk-adapter-claude` · evidence: §4.3 [documented: `Overage` is never retried — configuration, not weather];
F1 [measured: `isUsingOverage: false`, `overageDisabledReason`, `overageStatus` on the recorded `rate_limit_event` —
`spikes/FINDINGS.md` :42–48; the runner's `is_using_overage` is `false` on all 20 E0-headroom rows and all 60 S-VAR rows]
· decisions: ADR-0003 · **PROVISIONAL — no true-valued `isUsingOverage` instance in the recorded runs (`false` on 20
E0-headroom + 60 S-VAR rows, F1 :42–48)**
GIVEN a dispatch refused or failed as overage
WHEN the retry policy is consulted
THEN `StepError::Overage` maps to zero attempts, so no backoff, jitter or attempt is spent on it
AND no process is spawned for a dispatch already classified as overage.
*Verified by:* a unit test on the retry-policy table asserting `Overage` maps to zero attempts; an adapter test with the
**recorded** false-valued payload with `isUsingOverage` flipped to `true` asserting no process spawn.
**Correction (2026-09-11, audit PIA-065).** The 2026-09-08 refutation struck "F1 [measured: `isUsingOverage` present on the
top-level event]" as fabricated. It was not: the condensed brief's F1 row omitted the keys, but F1's own recorded payload
in `spikes/FINDINGS.md` carries them, and the code the runbook lifts already parses them. The cut is withdrawn; the
detection field is named; what is still unrecorded is a `true` value — `is_using_overage` is `false` on every one of the
80 recorded E0-headroom and S-VAR rows [measured, grep 2026-09-11] — so PROVISIONAL narrows to that (SM-112's overage
baseline is absent for the same reason, and S-RETRY is the named consumer of the first true-valued reading). SM-006's
kill is re-pointed to AC-221 with AC-220's margin rule.

**AC-379** — `rk-kernel` · evidence: F22 [measured: ~12 background kills vs one 12,000 s detached survivor under
`Start-Process`; `issue-81` killed 3× as background, 116 s in foreground]; C.6 [documented: nothing may depend on a
process surviving] · decisions: **ADR-0110**, ADR-0095, ADR-0017 (ADR-0003 related)
GIVEN a `StepError::RateLimited { resets_at }`
WHEN the kernel handles it
THEN it writes a durable `retry_at(resets_at)` mark and the process exits 0
AND no sleep, poll loop or live wait on the reset is executed.
*Verified by:* a kernel test asserting process exit and a mark row, with a clock fake that fails the test if advanced;
a CI grep rule forbidding `sleep`/`thread::sleep` in the rate-limit handler.
*Repaired:* cited "principle 3", which is the brief's *drift* implication; the process-survival implication is C.6.

**AC-380** — `rk-adapter-claude` · evidence: F1 [measured: `result.rate_limit` is **null**; headroom is readable only via
the undocumented top-level `rate_limit_event`] · decisions: **ADR-0110** (ADR-0003 related)
GIVEN a harness result envelope
WHEN headroom is read
THEN it is read from the top-level `rate_limit_event` stream event
AND `result.rate_limit` is never read, and a null there is never interpreted as "no limit in force".
*Verified by:* a fixture replay of a recorded stream in which `result.rate_limit` is null and `rate_limit_event` carries
values, asserting the values are picked up; a grep rule on `result.rate_limit`.

### 11.5 The cache is a latency and cost lever, NOT a headroom lever

**AC-381** — `rk-kernel` · evidence: F6 [measured: at 0.01 resolution the question "do cache reads draw down the window?"
is unanswerable]; F14 [measured: the E0 cache-cost gate was **STRUCK at 0.01 resolution**, not answered];
F5 [measured: cache-read share 0.584/0.698/1.000/0.722 on an identical prompt at n=2 — explicitly NOT a result] ·
decisions: **ADR-0110** decision 4, ADR-0016 (ADR-0003 related)
GIVEN any cache metric (`cache_read_input_tokens`, `cache_creation_input_tokens`, or a derived share)
WHEN a headroom figure or a headroom projection is computed
THEN the cache metric is not an input to it
AND a cache-derived "headroom saved" field does not exist on any event, projection or report.
*Verified by:* a type-level separation — the headroom calculator's input struct cannot name a cache field — asserted by a
compile-fail test; a CI grep rule for `headroom_saved` / `cache_headroom`; a positive test that the headroom calculator
still produces a batch delta from the two window readings alone, so the criterion is not satisfied by removing the
calculator.

**AC-382** — `rk-kernel` · evidence: F5 [measured: n=2, four readings spanning 0.584–1.000 on an identical prompt];
C.12 [measured: the discriminating results came from cost, stability and mechanism] · decisions: ADR-0016
GIVEN a reported cache-read share
WHEN it is emitted to any surface
THEN it carries the sample size it was computed from
AND a share computed from fewer than the configured minimum samples (SM-113 sets it at n=10) is emitted with a
`low_sample` flag and is refused as an input to any automatic decision.
*Verified by:* a unit test on the share reporter at n=1, n=2 and n≥minimum; an assertion that the pack-selection path
rejects a `low_sample` share.

**AC-383** — `rk-kernel` / pack loader · evidence: F16 [measured, 50 dispatches, within-arm range ≤0.001: no-commit
1.000 · commit-each 0.689 · +`--exclude-dynamic` 0.846 · +`--system-prompt` **1.000** · worktree-each 0.685];
F28 as corrected / S-H1b [measured: default prompt **18,603** at both 65.2 min and 49.7 h; static prompt
**29,619–29,680** created at ~66 min with share **0.000**] · decisions: ADR-0016
GIVEN an agent card
WHEN it is loaded
THEN it declares a `cache_profile` naming which axis it is optimised against — `static` (commit and worktree churn) or
`default` (time and human gates)
AND a card that declares neither is refused as a configuration error before any dispatch
AND the profile selects the adapter's delivery mechanism: under `static` the card body is delivered as `--system-prompt`
(AC-211); under `default` the adapter **omits `--system-prompt` and prepends the card body to the user message** — F28's
measured arm (18,603 cache-creation flat at both 65.2 min and 49.7 h) — so `default` is a delivery route, not a
lint-only label.
*Verified by:* a card-validator unit test; a `doctor` rule-table row; a fixture card with no profile that must fail; a
**golden argv test per profile** (`static`: `--system-prompt` present; `default`: absent, card body at the head of the
user message). *(Amended 2026-09-11, PIA-012; decision #19 of `spikes/DECISIONS-2026-09-11.md`: `default` named a
profile with no delivery mechanism; the 18,603-token flat-prefix promise of AC-384/SM-113 is scoped to `static`.)*
**Rationale, stated because it is counter-intuitive:** the two mitigations point in opposite directions, and the
inference "held across commits ⇒ helps across time" is explicitly falsified [measured, F28 correction + S-H1b].
**Provenance caveat on S-H1b and S-WT:** F30 was reserved for them and never written as a dated entry; their records
are `spikes/results/s-wt.jsonl` (S-WT, `--system-prompt` × worktree, w1 0.807 cold, w2–w10 1.000, n = 9) and the S-H1b
rows the brief's §C.1/§D quote. They are measurements without a finding write-up, not measurements without a record
(PIA-075).

**AC-384** — pack lint · evidence: F28 as corrected / S-H1b [measured]; F18 [measured: gate→verdict n=174 median 10.5 min,
**45% > 1 h**; stage→verdict n=191 median 51.6 min, p90 ~9 h, max 226 h, 48% > 1 h — one operator]; F10 [measured:
TTL ~1 h; full hit at 45.1 min, 61.6 min → share 0.690; the boundary is bracketed (45.1, 61.6) and never located] ·
decisions: ADR-0016, ADR-0017
GIVEN a card declaring `cache_profile = "static"`
WHEN the process pack places any of that card's activities immediately after a human gate
THEN the pack lint reports an error naming the falsified inference and the measured cost of the wrong choice
(a ~30k full prefix rewrite once the ~1 h TTL passes, against 18,603 flat under the default prompt)
AND the pack does not load until the profile or the placement is changed.
*Verified by:* a pack lint rule with a fixture pack that must fail and one that must pass, the failing fixture asserting
the message contains both figures. **Limit:** the static prompt's 24 h read was never taken (S-H1b), and F10's TTL
boundary is n=1 per gap and date-stamped platform behaviour — so the lint fires on the *bracketed* 1 h boundary, not on a
located one, and F18's gate durations come from one operator.

**AC-385** — `rk-adapter-claude` · evidence: §4.3 [documented: pin `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and the CLI
version]; code.claude.com/docs/en/prompt-caching — the Claude Code cache is "effectively scoped to one machine and
directory"; the API cache is isolated per org (per workspace on the Claude API); a CLI upgrade invalidates **this
machine's** next conversation's cache — **not** "org-scoped" and not "anywhere on the account" [documented, fetched
2026-09-10; grounding EXT-CLAUDECO-025, PIA-103] · decisions: ADR-0016, ADR-0003
GIVEN a dispatch
WHEN the child reports its own `system/init`
THEN the observed CLI version is recorded on the event, and a mismatch against `TESTED_CLI_VERSION` is surfaced as a
typed warning attached to every cache metric taken in that run
AND `CLAUDE_CODE_PROMPT_CACHE_TTL` is passed explicitly, never inherited.
*Verified by:* a fixture replay asserting the version lands on the event; an adapter test asserting the env var is on the
spawned command line; a report assertion that cache metrics under a version mismatch carry the warning.
*Cut:* two citations. F13 (prefix segmentation, 19/19 fresh repos) does not establish org-scoping — identical prefixes
across fresh repos on one account are consistent with org-scoping but do not measure it. "R8/F20 [documented: record the
version on every event]" misattributes F20, which is the finding that a card's `tools` allowlist silently disables
`--json-schema`; R8 is not in the brief. *Corrected 2026-09-11:* the "org-scoped / anywhere on the account" mechanism is
withdrawn — the docs describe per-machine, per-directory prefixes, so the subsidy is a **shared-prefix subsidy** and the
upgrade-invalidation inference reads "a new CLI version rebuilds this machine's prefix on next launch" — which is still
precisely why the version is recorded on every event rather than assumed stable.

**AC-386** — `rk-adapter-claude` · evidence: F8 [measured: default `-p` inherits 12 MCP servers / ~280 tools;
cache_creation 8,243 → 3,611 under `--strict-mcp-config`; MCP = 4,631 tokens = **56%** of the write; tools 280 → 34];
C.11 [measured: the flag must be verified from the child's own `system/init`, because an ignored flag looks identical to
an honoured one]; F20 [measured: a card's `tools` allowlist silently voids a flag at exit 0] · decisions: ADR-0003,
ADR-0095 (`NotIsolated` for tool-set inequality everywhere — decision #5 of 2026-09-11; AC-204 now agrees)
GIVEN a dispatch that passed `--strict-mcp-config` and a card `tools` allowlist
WHEN the child's own `system/init` reports a tool set that is not equal to the card's effective allowlist
THEN the dispatch fails with `StepError::NotIsolated` and is never retried.
*Verified by:* an adapter test with a scripted `system/init` reporting extra tools, asserting the typed error and zero
retry attempts. **This is a measurement precondition, not a nicety:** an ignored flag looks identical to an honoured one
from the parent's side [measured, F8].
*Cut:* the F7 and F17 citations. F7 is `--agents` being inert without `--agent`; F17 is the E3 fixture being defeated by
an object-store restore. Neither states or implies "adoption must be verified by set equality" — that is C.11, now cited
directly.

**AC-399** — `rk-store` / `redkiln cost` · evidence: F8 [measured]; C.11 [measured] · decisions: ADR-0003
*(Split from AC-386: failing the dispatch and quarantining the run's already-recorded metrics are independent behaviours —
the second is what protects every downstream cost comparison, and it can silently not happen while the first works.)*
GIVEN a run in which any dispatch failed `StepError::NotIsolated`
WHEN cost, token or cache metrics from that run are read
THEN every such row is marked non-comparable in the store
AND no non-comparable row is included in any cost aggregate, distribution or comparison.
*Verified by:* a store test asserting the mark is set on all of that run's rows; a report assertion that non-comparable
rows are excluded from every cost aggregate, with a fixture in which inclusion would change the reported figure.

**AC-387** — `rk-kernel` cost model · evidence: F16 [measured, 50 dispatches, within-arm range ≤0.001: worktree-each
(arm d) 0.685, n = 9, under the **default** prompt; +`--system-prompt` 1.000 as a separate, commit-churn arm]; S-WT
[measured, `spikes/results/s-wt.jsonl` — the run record names `--system-prompt`: **static × worktree** w1 0.807 (cold),
w2–w10 1.000, n = 9] · decisions: ADR-0016
GIVEN the cost model used to price a parallel fan-out
WHEN it prices any (cache profile × churn axis) cell
THEN it returns either a figure carried with its source spike and n, or an explicit `Unmeasured(cell)` refusal naming the
cell, and never a number for a cell no spike measured — the two measured worktree cells are **static × worktree = S-WT
(1.000 steady, 0.807 cold, n = 9)** and **default × worktree = F16 arm (d) (0.685, n = 9)**; every other cell is
`Unmeasured(cell)` until a spike runs it
AND it never interpolates a cell from two arms that were measured separately.
*Verified by:* a unit test asserting a two-worktree fan-out under a `static` card prices from S-WT (source and n carried)
and under a `default` card from F16 arm (d); a unit test asserting a cell no spike ran (for example `static × time` beyond
the TTL, or any cell at a different CLI pin) returns `Unmeasured` and not a number; a table-completeness test asserting
every priced cell names a source spike and its n.
**Correction (2026-09-11, audit PIA-075).** The 2026-09-08 "CONTESTED EVIDENCE" label rested on the brief not saying which
prompt configuration S-WT ran; the run record does — `s-wt.jsonl` carries `--system-prompt` — so §C.1 was right, the
two readings are two cells (static × worktree ≈ 0 after the first worktree; default × worktree ≈ 31%), not two
readings of one, and the earlier unit test asserted a refusal the record did not warrant. S-WT-REC is DONE on that
record; CONTESTED is struck here and is to be struck at design §0.1 row 49, §7.7, §10.4 and SM-053 (handoffs).

**AC-388** — `rk-kernel` scheduler · evidence: code.claude.com/docs/en/workflows and /prompt-caching — the vendor's
mechanism: hold all but the first sibling until the first response **begins**, capped at
`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS` (default 5000 ms), then release the held siblings together; platform.claude.com
says only "wait for the first response before sending subsequent requests", with no number [documented, fetched
2026-09-10; grounding EXT-ANTHROPI-005 — the §4.3 row's "platform docs" attribution and its per-sibling ~5 s chain were
both drift]; F13 [measured: 69% of the prefix is shared across directories] · decisions: ADR-0016
GIVEN a fan-out of two or more sibling dispatches sharing a prefix
WHEN they are scheduled under a stagger shape `s`
THEN the scheduler implements all three shapes SM-114's A/B needs — **serial chain** (each sibling at least `i` after the
previous), **simultaneous** (`i` = 0), and the **vendor's shape** (hold until the first response begins, capped at `c`,
release together) — and the shape and its parameter are configurable per fan-out
AND under the vendor's shape the first is dispatched alone and every held sibling's dispatch timestamp is at
`min(first_response_began, first_dispatch + c)`.
*Verified by:* a scheduler unit test over recorded dispatch timestamps with a clock fake for each shape: chain at `i` = 5 s,
simultaneous at `i` = 0, and the vendor's shape at `c` = 5 s with a scripted first-response event both before and after
the cap.
**Evidence grade: this rule is `[documented]` (the workflow-runtime docs), not measured by this programme** — the stagger's
benefit has no local measurement, so SM-114 tracks whether it pays across the three arms (a null at n < 50 sibling pairs
is reported **powerless**, not "no difference"), and the shape is configurable precisely so that the measure has its
comparison arms. *Cut:* the "R7" citation (not in the brief). *Repaired:* the original's second clause ("a scheduler that
launches siblings simultaneously fails its test") asserted a property of the test rather than of the system. *(Amended
2026-09-11, PIA-057: the per-sibling interval was not the vendor's mechanism; the third arm added.)*

### 11.6 Per-item token appetite is the real ceiling

**AC-389** — `rk-kernel` · evidence: §4.7 [documented: "per-item appetite fixed at triage"]; F6 [measured: headroom
cannot serve as the per-item ceiling because it is not measurable per run] · decisions: ADR-0003
GIVEN an item whose token appetite was fixed at triage
WHEN any later stage, activity or agent attempts to raise it
THEN the raise is refused and the attempt is appended as an event
AND raising the appetite is possible only by returning the item to triage.
*Verified by:* a kernel test attempting a mid-run raise from a stage and from an activity; a type-level check that the
appetite field is not mutable outside the triage transition.

**AC-390** — `rk-kernel` · evidence: §4.3 budgets row [documented] · decisions: ADR-0037
GIVEN the three budget counters — attempts per activity, tokens per item, wall clock per run
WHEN one is exhausted
THEN only that counter's failure path fires, and the other two are unchanged
AND no counter can be replenished, borrowed against or converted into another.
*Verified by:* three unit tests, one per counter, each asserting the other two counters' values are untouched at the
failure point.
*Cut:* the "shipmates L5.5" citation (not in the evidence brief, uncheckable); §4.3 and ADR-0037 carry the rule.

**AC-391** — `rk-kernel` / `rk-adapter-claude` · evidence: §4.3 inner/outer table [documented: Claude Code emits
`system/api_retry` with `attempt`, `max_retries`, `retry_delay_ms`, `error`; observe, record, do not duplicate];
F4 [measured: further undocumented `-p` stream events exist — `system/thinking_tokens`, 4–6 per dispatch] ·
decisions: ADR-0037
GIVEN a dispatch during which the harness performs its own inner retries
WHEN those retries are observed on the stream
THEN each observed inner attempt is recorded as a tier-2 event carrying the harness's own attempt fields
AND the kernel issues no retry of its own for an error class the harness already retried.
*Verified by:* a fixture stream replay containing `system/api_retry` events, asserting one tier-2 row per observed attempt
and no outer retry issued.
**Limit, corrected 2026-09-11 (PIA-068):** a recording of `system/api_retry` **does** exist — the payload is captured on
139 rows in 12 result files (AC-026 carries the recount and the attempt distribution) — so the fixture is a recorded
row, not synthetic, and this criterion now verifies the event's shape and field names as well as the kernel's handling.
F4 establishes only that further undocumented stream events exist, and the one it names is `system/thinking_tokens`.
*Cut:* "counted toward the activity's budget for that pass". ADR-0037 counts *steps* per pass and the §4.3 gloss says
observe and record, do not duplicate; nothing cited licensed folding harness-internal retries into the attempts budget,
and doing so would make the budget a function of platform-side behaviour the kernel cannot see or bound.

**AC-392** — `rk-kernel` · evidence: §4.7 [documented]; C.6 / F22 [measured: nothing may depend on a process surviving] ·
decisions: ADR-0003, ADR-0017
GIVEN an item whose accumulated tokens across its dispatch events reach its appetite
WHEN the next activity is about to dispatch
THEN the run halts with a typed budget error and writes a durable mark from which `redkiln run` resumes
AND it does not silently continue, truncate inputs, or downgrade the model to fit.
*Verified by:* a kernel test driving an item past its appetite, asserting the typed error, the mark row and the absence
of a spawned process; a test that the resume path reads the mark.

**AC-393** — `rk-kernel` · evidence: F6 [measured: `utilization` quantised to 0.01 — a per-item draw is below the
instrument]; F29 [measured: **0.0098** `five_hour` per CLEAN draw as a batch-derived mean over 60 draws — a per-draw
figure, not a per-draw measurement] · decisions: ADR-0003
GIVEN the budget enforcement path
WHEN it decides whether an item may continue
THEN it reads only the measured token counters on that item's own dispatch events
AND it never reads a headroom `utilization` value, because a per-item draw is not resolvable at the instrument's
resolution.
*Verified by:* a type-level check that the budget evaluator's input does not include the headroom reading, asserted by a
compile-fail test; a CI grep rule over the budget module; a positive test that the evaluator still halts an over-appetite
item from token counters alone.
*Repaired:* the original gloss read "0.0098 five_hour per CLEAN draw, i.e. one draw is one tick", which reads as though a
single draw's cost had been measured at one tick — the opposite of what this criterion and AC-373 assert. 0.0098 is
60 draws' total divided by 60; no single draw was ever resolved.

### 11.7 Cost telemetry from commit one

**AC-394** — `rk-kernel` · evidence: §4.7 [documented]; C.12 [measured: every discriminating result the programme produced
came from cost, stability or mechanism — never from success rates] · decisions: ADR-0003
GIVEN the dispatch entry point
WHEN it is called
THEN it cannot be called without an `EventStore` — the port is a required parameter, not an optional collaborator
AND a build in which an activity can dispatch with telemetry disabled does not compile.
*Verified by:* the function signature itself, asserted by a compile-fail (`trybuild`) test that omits the store; the
in-memory `Vec` fake (ADR-0003) as the always-available default in tests, so there is never a reason to make it optional.
C.12 is cited as motivation, not as proof: it is why cost telemetry is mandatory from commit one, not evidence that a
required parameter works.

**AC-395** — `redkiln cost` · evidence: §4.7 [documented]; F24, F25, F34 and the F31 adjudication [measured: these
analyses cost **$0** — reading the record is free] · decisions: ADR-0003
GIVEN a completed or in-flight item
WHEN `redkiln cost <item>` is invoked
THEN it reports the per-activity breakdown — token counters, `total_cost_usd`, wall clock, cache counters, and the batch
headroom delta with its `n`, its `resolution_limited` flag and, at `n < 10`, `low_sample` with no delta quoted (AC-372;
SM-111's floor — ADR-0110), or `not attributable` with the interleaving count when the item's dispatches were not the
only traffic — from the event store alone
AND it dispatches no model call and produces the same output offline.
*Verified by:* an integration test running the command against a fixture store with the harness adapter replaced by a
fake that panics on dispatch; snapshot tests of the rendered output including the `resolution_limited` flag, the
`low_sample` case and the `not attributable` case.

### 11.8 Human gate (not machine-decidable)

**AC-396** — triage · evidence: C.12 / F28 [measured: five of this programme's metrics saturated; success rates were
never the discriminating signal]; F37 [measured: P7's primary criterion is **powerless at n=3** — every Wilson interval
overlaps by construction, so no measured default is available]; §4.7 [documented: appetite is fixed at triage] ·
decisions: ADR-0017
GIVEN an item entering triage
WHEN its per-item token appetite is set
THEN the value is recorded as a human gate answer bound to the item, with the tree hash and a deadline
AND no automatic rule computes the value from the item's classification alone.
*Verified by:* routed to a human gate; **not machine-decidable** — the appetite trades expected quality against cost,
and this programme has no measurement that discriminates on quality (its success-rate metrics saturated, C.12/F28; P7's
rigor-default experiment was powerless at n=3, F37). The *mechanics* of the gate are machine-checked (ADR-0017: a
single-use token, the tree hash the human saw, a deadline with a terminal path), asserted by a gate-record test; the
*value* is not.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-109 | Attribution completeness | — |
| SM-110 | Cost per completed item, by lane | — |
| SM-111 | Headroom draw per batch | saturating |
| SM-112 | Throttle refusals, overage and rate-limit marks | — |
| SM-113 | Prefix cache-read share, by `cache_profile` | — |
| SM-114 | Cache write tokens and isolation share | no baseline |
| SM-115 | Per-item token appetite consumption | no baseline |
| SM-116 | Tier-2 disposability in practice | no baseline |
| SM-117 | Wall clock, the actual binding constraint | no firing kill |
| SM-118 | Saturated-metric register | saturating; no firing kill |

#### Residual risk this area could not repair

- AC-387's CONTESTED reading was resolved 2026-09-11 by the S-WT run record (`s-wt.jsonl` names `--system-prompt`): static × worktree and default × worktree are two measured cells, not two readings of one. What remains is that both are n = 9 on one machine at one CLI pin, and every other cell is `Unmeasured`.
- AC-378 is verified against the recorded `rate_limit_event` payload with `isUsingOverage` flipped: the field is recorded (`false` on all 20 E0-headroom and 60 S-VAR rows), a `true` value never has been, so the overage refusal has still not been exercised against a real overage.
- AC-391's `system/api_retry` fixture is a recorded row (139 captured, AC-026); the open question is C1's accounting, not the event's shape.
- Headroom is now an integer-tick contract (AC-372/374/376/377, ADR-0110); the lifted `throttle.rs` is rewritten, not lifted, and a re-lift that reintroduces `f64` subtraction passes every decimal-named vector while mis-flagging 80 of 100 one-tick deltas — the tick-typed field and the 0.09/0.10, 0.10/0.12, 0.20/0.21 vectors are the only guards.
- Six criteria are negative constraints (AC-361's second half, AC-373, AC-374, AC-375, AC-381, AC-393) that a system dispatching nothing satisfies trivially. Their real enforcement is CI grep and compile-fail tests — and a grep rule is defeated by renaming an identifier.
- About a third of the criteria rest on `[documented]` proposal sections (§4.3, §4.5, §4.7) that could not be read. AC-363, AC-369, AC-390 and the 'margins ≥2 ticks' rule behind AC-377 stand entirely on unverified proposal text.
- SM-112's overage baseline is genuinely absent and its kill fires on a value nobody has seen — the event's shape is recorded (F1), a `true` `isUsingOverage` is not — so the kill may be unable to trigger until a real overage is recorded; S-RETRY is the named consumer of that first reading.
- All 40 ids are allocated: there is no headroom for a future split without renumbering or extending the range.

---

## 12. CLI surface and `doctor`

**Area key:** `cli-doctor` · **Criteria:** AC-400..AC-439 **plus AC-531** (AC-428 is a reused id; AC-531 is allocated outside the range — see §0.4) — **41 in force** · **Measures:** SM-121..SM-130 (SM-131, SM-132 unallocated)
**State:** 40 binding (AC-428's PROVISIONAL lifted 2026-09-11; AC-531 added) · 0 provisional · 1 human-gated (AC-435) — 41 in force
**Sources:** proposal §4.2.1, §4.3, §4.4, §4.5, §4.6, §4.9, §6, §7; process-catalog proposal PC2/PC5/PC8; ADR-0003, 0004, 0005, 0006, 0009, 0010, 0011, 0012, 0013, 0014, 0016, 0017, 0024, 0027, 0030, 0032, 0033, 0036, 0038, 0042, 0045, 0047, 0061, 0063, 0067.

*Scope note.* Citations tagged **[outside the brief]** come from the proposal, an ADR or an external source and could not be verified against the evidence brief; they are design rationale, never measurement. This area touches the blocked selection engine only at its *rendering* surface (`explain`): AC-434 is scoped to display, never to selection.

### 12.1 One idempotent run entry point

**AC-400** — `rk-cli` · evidence: none measured; this is a design decision · decisions: **ADR-0107** (the CLI shape —
one idempotent `run` entry, the committed denylist, `init` idempotence; ADR-0011 related; PIA-082)
GIVEN the shipped `clap` router for Phase 1, whose committed argument snapshot contains `run` with its full flag set
WHEN any invocation passes a flag or verb whose name appears on the committed resume-shaped denylist (`resume`,
`continue`, `stay`, `--stay`, `--resume`, `--continue`, `--from`, `--pick-up`, `--retry-step`, `--from-step`, `--again`
— ADR-0107 decision 3's list plus this register's earlier three)
THEN the invocation is refused as an unknown argument, and no denylisted name appears anywhere in the snapshot.
*Verified by:* a committed snapshot test of the full `clap` argument surface (`trycmd`/`assert_cmd` over `--help` of every
verb, subcommands × long flags), which fails on any diff, plus a router lint over the parser definition that fails the
build on a name matching the committed denylist. The runbook's Phase-1 exit check (3) — `taskkill /T /F` mid-graph,
then re-invoke `redkiln run --lane fix --item <item>` and assert digest equality via `redkiln explain <item>` — is the
behavioural check that `--resume` is refused (ADR-0107 decision 4; PIA-030: the runbook's `redkiln run --resume
<run-id>` line was the flag this criterion denies).
**Limit, stated:** the denylist is name-based. A resume-shaped flag under an unlisted name is caught only by human review
of the snapshot diff, not by the lint. F38 shows a re-invocation was lossless at n=3; it does not license the stronger
claim that no resume entry point is ever needed.

**AC-401** — `rk-cli`, `rk-kernel` · evidence: **adjacent only** — F38 (3/3 across a real `taskkill /T /F`: 0 changed,
0 lost, digests identical) [measured] · decisions: proposal §4.3 idempotency, §4.5
GIVEN an item whose last activity committed a step and whose world has not changed
WHEN `redkiln run <item>` is invoked a second time
THEN no dispatch is issued for the memoised committed step and its recorded output digest is unchanged.
*Verified by:* a kernel test over a fake `HarnessPort` that counts dispatches and fails on a second dispatch for a memo
key already present.
**Limit, stated:** F38 measured *resume losslessness*, not memoisation. The brief records P8 §5.4(3) — the second-arrival
question — as **circular by construction** ("arrivals forced, mark minted by the driver that implements the semantics"),
and P8's state holds exactly one step at n=3. This criterion therefore rests on the kernel test alone; no spike has
measured it. SM-130 carries the same correction.

**AC-402** — `rk-cli`, `rk-git` · evidence: F19 (drift 12/174 = 7%; 3/95 at ≤1 h vs 9/79 at >1 h) [measured, one
operator, one machine], F18 (45% of gate→verdict intervals, n=174, and 48% of stage→verdict intervals, n=191, exceed
1 h) [measured] · decisions: proposal §4.5 rule 3, principle 2
GIVEN any entry point that **can cause an action** (`run`, `answer`, `doctor`) — `status` and `explain` are
read-only and do **not** reconcile (ADR-0105 clause 8; AC-172)
WHEN it begins
THEN `reconcile(recorded, world)` has run before any decision is taken, and no CLI flag can skip it.
*Verified by:* a kernel test enumerating the **three** entry paths and asserting `reconcile` is called on each, **and
a negative case asserting `status` and `explain` do not call it**; the
argument-surface snapshot (AC-400) additionally fails the build on any `--no-reconcile` / `--skip-reconcile` / `--fast`
flag.
*(Amended 2026-09-12, DECISIONS-2026-09-12 E11: this criterion enumerated **four** paths — including `status`, which
ADR-0105 clause 8 excludes in terms, and omitting `doctor`, which can act via `--fix` and `--rebuild`. AC-172 had
already been amended to the governing set and cited clause 8 by name; this row and design §8.1's verb table were the
un-updated copies. ADR-0109 decision 1's scope — "every CLI entry that **can cause an action**" — is the governing
form.)*
**Cut on refutation:** the original also claimed no *configuration key* could skip reconcile. No instrument checked
configuration keys, so the claim is withdrawn rather than left uninstrumented.

**AC-403** — `rk-cli`, `rk-kernel` · evidence: F22 (~12 background kills vs one 12,000 s detached survivor) [measured,
operational; cause unestablished], F18 (gate→verdict median 10.5 min, n=174; the p90 ~9 h and max 226 h figures belong to
the **stage→verdict** distribution, n=191) [measured] · decisions: proposal §4.6, principle 3, ADR-0017
GIVEN an activity that reaches a human gate
WHEN `redkiln run <item>` opens that gate
THEN the `Gate:` trailer and `gate_opened` event are durably written and the process exits 0 without waiting on any
channel, prompt, socket or sleep.
*Verified by:* an integration test that runs `run` to a gate under a fake clock, asserting the trailer and event are on
disk and that the process exits with no live handle held; plus a lint forbidding blocking stdin reads in `rk-cli`
outside `init`.

**AC-404** — `rk-cli`, `rk-adapter-claude` · evidence: F22 / brief §C.6 ("nothing may depend on a process surviving;
every wait is a durable mark plus a later read") [measured] · decisions: proposal §4.3 outer-retry policy, principle 3
GIVEN a dispatch the adapter classifies as rate-limited
WHEN the kernel handles it
THEN a retry mark carrying the earliest admissible retry time is durably written and the process exits; no sleep, timer
or in-process wait is entered.
*Verified by:* a unit test on the retry policy asserting the rate-limited branch performs zero sleeps, under a fake clock
that fails the test if the retry path advances it.
**Limit, stated:** the brief evidences no `resets_at` field. F1 exposes `unifiedWindows.{five_hour,seven_day}.utilization`
and no family sub-limit key; F6 shows `utilization` quantised to 0.01. Where the retry time comes from is **[outside the
brief]** and must be established before this criterion can name a source field.

---

### 12.2 `answer` — the only Phase 1 gate channel

**AC-405** — `rk-cli`, `rk-store` · evidence: none measured; this is a scope decision · decisions: **ADR-0009**, ADR-0011
GIVEN a Phase 1 build
WHEN the Phase 1 dependency closure and the call graph are inspected
THEN it contains no tracker or web crate, and the gate-answer store method has exactly one call site, in `rk-cli`'s
`answer` command.
*Verified by:* a crate-graph test asserting no GitHub or web crate in the Phase 1 dependency closure, plus a lint
asserting exactly one call site of the gate-answer store method.
**Cut on refutation:** the original cited F18 and F28 (gate duration and gate cost), neither of which bears on who may
write a gate answer, and asserted "it does not compile" — a claim neither instrument makes. The THEN is now the observable
the instruments actually check.

**AC-406** — `rk-cli`, `rk-git` · evidence: F19 (drift 12/174 = 7%; 3/95 ≤1 h vs 9/79 >1 h — the 3.6× ratio is a point
estimate whose Wilson intervals **overlap**, see SM-125) [measured] · decisions: **ADR-0017**
GIVEN an open gate whose record binds `(token, tree_hash, deadline)`
WHEN `redkiln answer` is invoked with a valid token but the tree hash no longer matches the recorded one
THEN the answer is recorded as **void**, the gate stays open with the observed and recorded tree hashes attached, and no
stage transition occurs — the answer is never reconciled into a verdict.
*Verified by:* an integration test that answers after a commit lands, asserting a `gate_answer_void` event carrying both
tree hashes, `gate.state == open`, and no stage transition.

**AC-407** — `rk-cli`, `rk-store` · evidence: none measured; this is a design decision · decisions: ADR-0017
GIVEN a gate answered once with its single-use token
WHEN the same token is presented again
THEN the second answer is refused and no second `gate_answered` event is written.
*Verified by:* a store test replaying the same token twice and asserting exactly one `gate_answered` event.
**Cut on refutation:** F19's 174/174 losslessness was cited here and says nothing about token replay.

**AC-408** — `rk-cli`, `rk-kernel` · evidence: F18 (45% of gate→verdict intervals exceed 1 h, n=174; the 226 h / 9.4-day
maximum is stage→verdict, n=191) [measured], F28 (gate cost flat in duration: 18,603 tokens at both 65.2 min and 49.7 h)
[measured, **with the brief's published correction: the flatness is shared-prefix subsidy, not durability**] ·
decisions: ADR-0017
GIVEN a gate whose deadline has passed
WHEN the next `redkiln run` reaches it
THEN the declared terminal path (fail, escalate or auto-decide) is taken and recorded; the gate never remains undecided
past its deadline and no path hangs.
*Verified by:* a kernel test per terminal-path variant under a fake clock; an exhaustive-match test so a new deadline
variant with no terminal path fails to compile.

**AC-409** — `rk-cli`, `rk-types` · evidence: none measured; this is a graph-shape decision · decisions: **ADR-0045**,
ADR-0038
GIVEN a graph in which a downstream step binds to a gate step's `outputs`
WHEN the configuration is loaded (and when `doctor` runs over it)
THEN it is rejected as a configuration error: a gate answer populates the gate step's `answer` and can bind nothing
downstream.
*Verified by:* a lint/`doctor` rule row rejecting any graph reference that resolves to `outputs` of a gate step, with a
red fixture graph that must fail; plus a compile-time test that the gate step type exposes no `outputs`.
**Cut on refutation:** F32 (P9 authoring) and F33 (P9b ambiguities) were cited here and bear on schema authoring
verbosity, not on gate output bindings. The THEN was also recast from a type shape to a rejection outcome.

**AC-410** — `rk-cli`, `rk-types` · evidence: F33 (the lint accepted 5 guesses silently) [measured — cited as the
**failure shape** this rule guards against, not as measurement of this rule] · decisions: **ADR-0063**, ADR-0038
GIVEN a gate question whose registry entry declares its options and each option's admissible attachment types
WHEN `redkiln answer` is passed an attachment that no declared option admits
THEN the answer is refused at the CLI boundary and nothing is written.
*Verified by:* a unit test on the attachment validator; a red fixture carrying an unbound attachment that must fail; a
store assertion that the refused path wrote no event.

---

### 12.3 The CLI is the only writer of structured fields

**AC-411** — `rk-store` · evidence: **[outside the brief]** — the S-MIG percentages formerly quoted here (free-form gate
results, specs without GIVEN/WHEN/THEN) appear nowhere in the evidence brief and are cut; the brief records S-MIG only as
a $0 analysis · decisions: **ADR-0005**, **ADR-0006**
GIVEN a human-facing artifact rendered as frontmatter + digest-stamped structured sections + opaque prose sections
WHEN a human edits text inside a rendered structured section and `reconcile` runs
THEN the edit is surfaced as typed drift and is never parsed back into the value; the next render restores the value
from the type.
*Verified by:* a store test that mutates a structured section on disk and asserts a `structured_drift` event with an
unchanged canonical value and a restored render; a lint forbidding any markdown-to-structure parser in `rk-store` outside
the opaque-section locator.

**AC-412** — `rk-cli`, `rk-git` · evidence: none measured for this rule · decisions: proposal §4.5 rule 4 **[outside the
brief — the git-branchless `reference-transaction` atomicity issue is not in the evidence brief]**
GIVEN a repository initialised by `redkiln init`
WHEN the installed git hooks are inspected
THEN no redkiln-owned hook is present in `.git/hooks`.
*Verified by:* a `doctor` rule row that fails on any redkiln-authored hook in `.git/hooks`; an `init` test asserting zero
hooks installed.
**Cut on refutation:** the second clause ("the kernel's single write path remains the only writer of tier-1 rows") had no
instrument here and is covered, for the surface that is actually checkable, by AC-405. The F12 citation was also cut: F12
measured slice rejection states over 360 trailer commits, and does not validate `reconcile`.

**AC-413** — `rk-store` · evidence: **[outside the brief]** — the cited Backlog.md issue and shipmates note are not in the
evidence brief · decisions: proposal §4.5 rule 5, ADR-0003
GIVEN a store carrying repo identity (root commit + common-dir), schema version and boot-id
WHEN any command opens that store from a different repository
THEN the open is refused as a configuration error and no row is read or written.
*Verified by:* a store test opening a store against a second fixture repo, asserting the refusal and asserting zero reads
and writes against the store handle; a red fixture in the `doctor` corpus.

---

### 12.4 `doctor` — cost and rule table

**AC-414** — `rk-cli`, `rk-kernel` · evidence: every configuration-time analysis in the programme measured **$0** — F24
(the six-rejection adjudication), F25 (the AC-015 probe), F32 (P9, zero model dispatches), F34 (P6 authoring), F35 (the
P6 lattice) [measured, brief §D] · decisions: **ADR-0014**, ADR-0027
GIVEN any configuration, however large, and a rule table each of whose ids has a rejecting fixture (AC-415)
WHEN `redkiln doctor` runs to completion over that fixture corpus
THEN every fixture's expected rule fires **and** zero harness dispatches are issued and zero model tokens are spent.
*Verified by:* the `doctor` integration test injects a `HarnessPort` fake that fails the test on any dispatch, over the
AC-415 fixture corpus so that a doctor which does nothing fails the same test; plus a CI assertion that the `doctor` code
path does not link a live adapter.
**Repaired:** the original ("zero dispatches") passed trivially on a `doctor` that performs no checks. It is now bound to
the AC-415 corpus so the zero-cost claim can only be satisfied by a doctor that is actually checking.

**AC-415** — `rk-cli` · evidence: F32 (the lint's first run caught **13 real errors against 34 false alarms**)
[measured — the sub-splits "4 design defects" and "4 prototype bugs" are not in the brief and are cut], F20 (a check that
silently stopped checking is how the schema void survived) [measured] · decisions: ADR-0014, proposal §4.2.1
GIVEN the `doctor` rule table, which is data with a stable id per rule
WHEN CI enumerates the rule ids
THEN every id has at least one committed fixture configuration that the rule actually rejects, and a rule with no
rejecting fixture fails the build.
*Verified by:* a CI test joining rule ids to fixture directories, asserting a non-empty fixture set per id and asserting
each fixture is genuinely rejected by that id.

---

### 12.5 `doctor` — pack and card rules (all configuration-time, zero model cost)

**AC-416** — `rk-cli`, `rk-kernel` · evidence: none measured; proposal §4.2.1 binder rule 1 and PC7's equal-specificity
tie rule **[outside the brief]** · decisions: ADR-0014, ADR-0024
GIVEN an enabled process pack declaring roles and one or more technology packs contributing cards
WHEN a role resolves to zero cards, or to two cards at equal selector specificity, for any configured toolchain
THEN the configuration is rejected — `doctor` reports a configuration error naming the role and the candidate cards, and
no card is bound.
*Verified by:* a binder unit test per case (unresolved, ambiguous) asserting an error return and no binding; two red
fixtures; the corresponding rule row.

**AC-417** — `rk-cli`, `rk-adapter-claude` · evidence: **F20** (a card `tools` allowlist silently disables
`--json-schema`: `structured_output` absent, exit 0, `subtype: success`; adding `StructuredOutput` fixes it) [measured] ·
decisions: **ADR-0108** (ADR-0016, ADR-0014 related; PIA-082)
GIVEN a card whose `tools` list omits `StructuredOutput` — or a card with **no `tools:` key** (a configuration error;
Claude Code inherits every tool when `tools:` is absent, shipmates L4.4, and ADR-0108 decision 2 makes the effective set
the declared list and nothing else) — and whose role has at least one activity demanding a JSON schema
WHEN `redkiln doctor` runs
THEN that card is reported as a configuration error at configuration time, before any dispatch.
*Verified by:* a `doctor` rule row; a red fixture card that must fail; a red fixture card with no `tools:` key that must
fail (the new doctor row, PIA-060); a card-validator unit test. Zero-cost is asserted by AC-414's fake, not restated
here.

**AC-418** — `rk-cli` · evidence: F15 (reasoning effort IS applied per dispatch — thinking tokens low **0.0** / high
**4.8** / xhigh **221.0**, complete separation; **the brief also records that low vs high did NOT separate**) [measured];
proposal §4.3 ("`--effort` and `model` from the card, never inherited; a missing value is a config error") **[outside the
brief]** · decisions: ADR-0014
GIVEN tier declared on a role
WHEN any card or slot filler also sets a model or effort
THEN `doctor` reports tier leakage as a configuration error.
*Verified by:* a `doctor` rule row plus red fixtures in which a card sets `model` and in which a slot filler sets
`effort`.
**Cut on refutation:** "a card can never downgrade its role's tier" was a second, uninstrumented claim about tier ordering;
the instrumented claim is that any card-level model/effort is an error at all.

**AC-419** — `rk-cli` · evidence: the timeout history [measured, brief §D: a 900 s default written for the fast condition
cost a CLEAN record; escalated 2,400 → 3,600 → one 5,400 s retry; S-TYPED's slowest CLEAN review 4,561 s] ·
decisions: ADR-0014 (L5.6), PC7 (per-activity `timeout_secs` override)
GIVEN any card or slot-filler activity in an enabled pack
WHEN it declares no `timeout_secs`, or declares a non-numeric one
THEN `doctor` reports an unbounded activity as a configuration error.
*Verified by:* a `doctor` rule row; a red fixture with a missing timeout; a schema test rejecting a non-numeric value.

**AC-420** — `rk-cli` · evidence: F33 (the lint accepted 5 guesses silently) [measured — cited as the failure shape;
"feedback refs were lint folklore" is not in the brief and is cut] · decisions: ADR-0014, ADR-0042
GIVEN a card `fulfils`, a skill `attach.roles`, or a slot filler naming a slot
WHEN any of those names a role or slot that no enabled pack declares
THEN `doctor` reports a dangling reference as a configuration error.
*Verified by:* one `doctor` rule row per reference kind; three red fixtures.

**AC-421** — `rk-cli` · evidence: F35 (dead activities **15/101 under 12 packs = 14.9%**, falling to **6/102 = 5.9%**
after decisions 1–10; **the pre-registered reading over 8 packs measured 20.8% and fired the >20% kill by 0.8 points**)
[measured] · decisions: ADR-0014 (L5.11), **ADR-0027**
GIVEN roles, skills and declared slots in the enabled packs
WHEN a role, skill or slot is referenced by no enabled pack, or a declared slot is neither filled nor marked optional
THEN `doctor` reports a dead definition.
*Verified by:* a `doctor` rule row; a red fixture with an unreferenced skill; a fixture with an unfilled, unmarked slot.

---

### 12.6 `doctor` — environment, pins and facet vocabulary

**AC-422** — `rk-cli`, `rk-adapter-claude` · evidence: **[outside the brief]** — `--bare`, OAuth and `ANTHROPIC_API_KEY`
appear nowhere in the evidence brief; charter §6 and platform docs, quoted at proposal §4.3 · decisions: **ADR-0010**
GIVEN a subscription-authenticated environment
WHEN `redkiln doctor` runs, or any dispatch is composed
THEN the presence of any name on `rk-adapter-claude`'s committed environment denylist
(`rk-adapter-claude/src/env_denylist.rs` — `ANTHROPIC_API_KEY` and its siblings; the same list AC-217 and AC-488 read,
decision #16 of 2026-09-11) in the captured environment is reported as a configuration error, and `--bare` appears in no
composed argument vector.
*Verified by:* a `doctor` rule row reading the committed list, with a red fixture environment carrying the key; an
assertion that the `doctor` row and AC-217's construction test read one list; a unit test over the argument builder
asserting `--bare` is emitted on no input.
**Left compound, disclosed:** this bundles a `doctor`-time rule and an argument-composition rule. Both halves carry their
own instrument, so neither is uninstrumented, but no free id in the AC-400..AC-439 range remained to split them. Split
this before it reaches the runbook.

**AC-423** — `rk-cli`, `rk-adapter-claude` · evidence: F13 (prefix ~17,439 shared + ~7,827 per-directory; 19/19 fresh
repos identical) [measured], F16 (steady-share arms, 50 dispatches, within-arm range ≤0.001) [measured], the static-prompt
cold write **29,619** (S-H1b at 66.1 min) and **29,680** (F28 at 66.8 min) [measured, brief §C.1 and §D] ·
decisions: **ADR-0016**
GIVEN a pinned `TESTED_CLI_VERSION` and pinned `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and**
`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` (both, ADR-0078; PIA-029)
WHEN the installed CLI version or either TTL variable differs from the pin, or one TTL variable is set without the other
THEN `doctor` reports the mismatch as a configuration error, naming both the observed and the pinned value.
*Verified by:* a `doctor` rule row driven by a stubbed version probe; a red fixture with a mismatched version; a red
fixture with a mismatched TTL; a red fixture with exactly one TTL variable set (AC-526's rule, read by the same row).
**Cut on refutation:** the claim that the cache is org-scoped and that a CLI upgrade anywhere on the account invalidates
the shared tool block, and the cited platform issue showing a server-side TTL flip, are **not in the brief** and are cut.
What the brief supports is that the prefix subsidy is a **shared-prefix subsidy** — the docs scope the Claude Code cache
to one machine and one directory, so a new CLI version rebuilds *this machine's* prefix on next launch, never "anywhere
on the account" [documented, code.claude.com/docs/en/prompt-caching, fetched 2026-09-10; PIA-103] — and that TTL
behaviour is date-stamped, n=1 per gap (F10) — which is itself the reason to pin.

**AC-424** — `rk-cli`, `rk-kernel` · evidence: F35 (after decisions 1–10: unsatisfiability **0.3%**, rank ties **0**, hard
cycles **0**; **the pre-registered reading fired the >5% kill twice, at 23.5% and 11.4%**, and the cleared figure is
post-amendment) [measured] · decisions: **ADR-0030** (rule 1), **ADR-0061**
GIVEN `facets.toml` as the normative closed vocabulary
WHEN an activity or pack names a facet that does not exist, a value outside that facet's enumeration, or a technology
pack adds a new facet rather than extending an existing facet's values
THEN `doctor` rejects the configuration.
*Verified by:* three `doctor` rule rows with a red fixture each; a facet-resolver unit test.

**AC-425** — `rk-cli`, `rk-kernel` · evidence: **F36** (`external_dep` became constant, 200/200 false) [measured];
brief §E contested list (`recent_deploy` underivable at this pin; `binary_change` measured but unregistered in
`facets.toml`, so it can feed no `require` row; `dynamism` pre-registered with no registry entry) [measured/contested] ·
decisions: **ADR-0030** (rule 2), ADR-0061, ADR-0067
GIVEN every facet in `facets.toml`
WHEN a facet has no registered computed fact in the context vector, or a computed fact is used in a `require` row with no
registry entry
THEN `doctor` reports a configuration error and the facet may be referenced by no admissibility or requirement row.
*Verified by:* a `doctor` rule row joining `facets.toml` to the fact registry; red fixtures reproducing the three named
contested cases.

**AC-426** — `rk-cli`, `rk-store` · evidence: **none measured** — the S-MIG figures formerly cited here (unfilled `plan.md`
scaffolds, missing mutation-evidence columns) appear nowhere in the evidence brief and are cut. The nearest measured
analogue is F35's dead-activity count (6/102 after decisions 1–10) [measured], which is about *definitions*, not
artifacts · decisions: ADR-0006, ADR-0027, proposal §4.4
GIVEN an `artifact.read` event emitted for every activity input, and a configured window `unread_window_runs` (a required
key, no default)
WHEN a stage's produced artifact accumulates zero downstream reads over that window
THEN `doctor` flags that artifact as unread at warning severity, and nothing is deleted or rewritten.
*Verified by:* a `doctor` rule row over a synthetic event log containing a stage whose artifact is never read; a second
fixture within the window that must **not** flag; an assertion that the run's write set is unchanged after the rule fires.
**Limit, stated:** no measurement supports the window value or the hazard rate. This rule is a design intent with a
working instrument, not an evidenced requirement.

---

### 12.7 `doctor --rebuild`

**AC-427** — `rk-cli`, `rk-git`, `rk-store` · evidence: F12 (360 trailer commits / 74 slices of trailer data;
ever-rejected 15/74 = 20.3% — a *slice-state* result, **not** a validation of `reconcile`) [measured], F19 (174/174
lossless state carriage) [measured] · decisions: proposal §4.5 rule 3, ADR-0003, ADR-0013
GIVEN a repository whose tier-1 facts live in git trailers, with the store deleted or absent
WHEN `redkiln doctor --rebuild` runs
THEN a store is reconstructed from git alone, and the computed next action for every item in the pinned corpus is
identical to the recorded one; any divergence fails the build.
*Verified by:* the delete-and-rebuild CI job over the `redkiln-corpus` pin, comparing next-action output before and after
rebuild, whose divergence report artifact must be empty.
**Limit, stated:** F19 proves state *carriage*, not "same next action" — the FSM was never replayed [brief §C.4]. The
next-action half has **no prior measurement**; SM-124 establishes it.

**AC-428** — *(cut as a duplicate of AC-427; the id is re-issued below to the split half of AC-439 — see Refutation
notes)*

**AC-429** — `rk-git` · evidence: none measured for this rule; proposal §4.5 rules 3 and 4 **[outside the brief — the
cited git-branchless issue is not in the evidence brief]** · decisions: proposal §4.5 rules 3 and 4
GIVEN `doctor --rebuild` running against a repository
WHEN it completes
THEN no git object, ref or working-tree file has been written: reconciliation is git → store, never the reverse.
*Verified by:* a test comparing `HEAD`, the ref list and the tree hash before and after rebuild, against a read-only git
fixture that fails the test on any write attempt.

---

### 12.8 `explain` — both row families, printed from the record

**AC-430** — `rk-cli`, `rk-kernel` · evidence: F35 (the lattice's admissibility rows are what make the 0.3%
unsatisfiability and 0 rank-tie counts computable) [measured] · decisions: **ADR-0024**, ADR-0047, ADR-0014
GIVEN an item whose DAG has been selected
WHEN `redkiln explain <item>` runs
THEN the output prints, for every included activity, BOTH its admissibility row from the activity (`provides`,
`requires`, `applicable_if`, `method_rank`, `min_rigor`) AND the requirement rows from the process pack (`baseline` and
each matching `require` row) that put it in the DAG.
*Verified by:* a snapshot test on a fixture item asserting both row families for every included node, and failing if
either family is empty while the other is populated.
**Cut on refutation:** "F32/F34 — the two row families are authored in two separate homes" was cut. F32 and F34 measure
authoring verbosity and lint error rates; the two-home structure is ADR-0024's decision, not their finding.

**AC-431** — `rk-cli`, `rk-kernel` · evidence: F7 (`--agents` is inert without `--agent`; canary both → BANANA,
`--agents` alone → 4; the card's model+effort were accepted — so the resolution is not inspectable from outside)
[measured], F20 [measured] · decisions: ADR-0014 (§4.2.1 binder rule 4), ADR-0016
GIVEN a completed or planned activity
WHEN `redkiln explain <item>` runs
THEN the printed resolution names the card, the attached skills, the inserted slot fillers and the effective tier for
that activity.
*Verified by:* a snapshot test asserting all four resolution fields per activity, and failing when a resolution field
present in the run record is absent from `explain` output.

**AC-432** — `rk-cli` · evidence: **F36** (recomputing selection returned the same modal `R` on only **5/20, 3/20, 2/20
and 2/20** draws against an 80% line — a recomputation at explain time would print something that never ran) [measured] ·
decisions: proposal §4.5 (git is truth, the record is a cache), ADR-0036
GIVEN a run whose resolution was written to the run record
WHEN `redkiln explain` renders it
THEN the output is derived from the recorded resolution only; `explain` performs no selection, and an item with no
recorded resolution is an error rather than a recomputation.
*Verified by:* a replay assertion — `explain` over a replayed event log is byte-identical to `explain` at run time — plus
a kernel test asserting the selection engine is not reachable from the `explain` code path.

**AC-433** — `rk-cli`, `rk-kernel` · evidence: none measured for exclusion frequency; ISO/IEC/IEEE 12207:2017 Annex A on
recording tailoring decisions with rationale **[outside the brief]** · decisions: ADR-0024, ADR-0033
GIVEN an activity that is admissible in the catalog but excluded from this item's DAG
WHEN `redkiln explain <item>` runs
THEN the output names the specific clause that excluded it — the false `applicable_if` sub-predicate, the unmet
`min_rigor`, or the absent `require` row — rather than reporting "not selected".
*Verified by:* a snapshot test over a fixture carrying one exclusion of each kind, asserting the named clause and failing
on a bare "not selected" string.
**Cut on refutation:** F35's 6/102 dead activities was cited as evidence that "exclusions are common". Dead activities are
catalog definitions referenced by nothing; they are not per-item DAG exclusions. The citation is withdrawn.

**AC-434** — `rk-cli` · evidence: **F36** (`unknown` rates: novelty **46.0%**, size **39.5%**, blast_radius **31.0%**,
criticality **27.0%**; brief §C.9 — `unknown` is treated as `high` by position, so an axis oscillating unknown↔low moves
`R` the full band width) [measured] · decisions: ADR-0030, ADR-0032
GIVEN a context vector in which one or more facets are `unknown`
WHEN `redkiln explain` prints the admissibility rows that consumed them
THEN each `unknown` facet is rendered as `unknown` with its position-derived treatment stated, and is never rendered as a
measured band value.
*Verified by:* a snapshot test on a fixture vector carrying `unknown` on four axes, failing if any renders as a band value.
**Scope note (not PROVISIONAL).** F36 ran; P1's modal-`R` kill fired **4 of 4** and the selection engine is BLOCKED. This
criterion constrains *display* only and is implementable as written. Any criterion asserting that the displayed vector is
*stable* is not writable until P1's kill is retired. **P2's labels are now written and timestamp-verified** [measured,
F40], so that half of this sentence is discharged — and it never carried a stability claim anyway: P2 measures
short-circuit classification against a human label, not vector determinism, and its perturbation half is scored while
its advisory half is unrun. Do not extend this to a determinism claim.

**AC-435** — human gate · evidence: ADR-0024's own consequence note ("understanding why an activity ran requires reading
two artifacts; `redkiln explain` exists specifically to paper over this cost") **[outside the brief]** ·
decisions: **ADR-0024**, **ADR-0012**
GIVEN `explain` output that satisfies AC-430, AC-431, AC-433 and AC-434
WHEN a reviewer at a human gate is asked whether it answered "why did this run" without opening a TOML file
THEN the answer (yes/no, plus whether a TOML was opened) is recorded as a required field on the gate answer.
*Verified by:* **routed to a human gate; not machine-decidable.** Reason: sufficiency-for-a-reader is a legibility
judgement — that the fields are printed is checkable (AC-430/431/433/434), that they suffice is not. The machine-checkable
half is that the gate answer cannot be submitted with the field absent. SM-126 carries the rate.
**Cut on refutation:** "P3 registered the adjacent measure and is UNRUN" was cut. **No spike named P3 appears anywhere in
the evidence brief** — not in the ledger, the kill-criteria table, or the unrun list.

---

### 12.9 `catalog search` and `init`

**AC-436** — `rk-cli`, `rk-kernel` · evidence: **F36** (an LLM-shaped "which activities?" question is precisely what
proved unstable: modal `R` identical on 5/20, 3/20, 2/20, 2/20) [measured]; description-matching and label-sprawl
precedents **[outside the brief]** · decisions: **ADR-0030 rule 5**, ADR-0027
GIVEN an activity's engine-ignored `keywords` field, which `redkiln catalog search` searches
WHEN the `keywords` of every activity in the catalog are mutated arbitrarily and selection is re-run over a fixed context
vector
THEN the selected activity set is byte-identical.
*Verified by:* a property test that permutes `keywords` across the shipped catalog and asserts an unchanged selection,
plus a lint asserting `keywords` is unreachable from the admissibility evaluator.

**AC-437** — `rk-cli` · evidence: none measured; namespacing-retrofit precedent **[outside the brief]** ·
decisions: **ADR-0107** decision 2 (`init` is idempotent; ADR-0011 related; PIA-082), proposal §4.9
GIVEN a repository
WHEN `redkiln init` is run twice in succession
THEN the second run makes no change — identical tree, identical store identity rows — and exits 0.
*Verified by:* an integration test asserting an empty `git status --porcelain` diff, unchanged store identity rows and
exit 0 after the second `init`.
**Cut on refutation, re-added 2026-09-11:** the namespacing half ("every artifact it created is namespaced under the
single owned prefix") was cut rather than split — its only support was an out-of-brief anecdote and its instrument was
an unspecified "naming lint". The prefix is now decided (`rk-`, decision #8 of 2026-09-11) and the clause is its own
criterion, **AC-531**, with the lint regex AC-506 carries.

**AC-531** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-cli` (`init`) ·
evidence: shipmates L3.1 (twenty generically-named skills prefixed at a cost of one release, three regressions and six
follow-ups — "free at t=0; ~9 issues at t=30d") [documented]; decision #8 of 2026-09-11 (the prefix is `rk-`) ·
decisions: ADR-0107, proposal §4.9 ("every installed artifact is namespaced from day one (L3.1)")
GIVEN a repository on which `redkiln init` has run
WHEN the files, directories, git refs and hook-free entries it created are enumerated
THEN every name it created outside the repository's own `.redkiln/` directory matches the committed lint regex
`^rk-[a-z0-9][a-z0-9-]*$` (AC-506's regex), and `init` refuses to create a name that does not
AND a pre-existing name matching the prefix that `init` did not create is reported, never overwritten.
*Verified by:* a naming lint over `init`'s manifest of created names with a red fixture (an unprefixed name) that must
fail; an integration test enumerating the tree diff after `init` and asserting every created name matches the regex; a
fixture with a pre-existing `rk-` name asserting the report and no overwrite. *(The clause AC-437 cut on 2026-09-08 and
promised to re-add "when the prefix is decided" — PIA-106.)*

---

### 12.10 Exit codes, and a platform error that returns success

**AC-438** — `rk-cli`, `rk-types` · evidence: none measured; proposal §4.3 `StepError`, and SIGTERM → 143 with no result
**[outside the brief]** · decisions: proposal §4.3 `StepError`, ADR-0074 (supersedes ADR-0004)
GIVEN the closed `StepError` enum
WHEN `redkiln` terminates for any reason
THEN its exit code comes from a closed, snapshot-pinned table in which success, configuration error (including any
failing error-severity `doctor` rule), gate-open, findings-block and each runtime error class are distinct values; a new
`StepError` variant with no mapping fails to compile.
*Verified by:* an exhaustive-match test over `StepError` → exit code; a committed snapshot of the exit-code table; a
table-driven CLI test over fixtures × expected code, including the `doctor` fixtures of AC-415.
**Cut on refutation:** the F21 citation ("`is_error`/`api_error_status` observed alongside exit codes that do not match")
was cut. **F21 in the brief is the E3 arm (b) result** (S-SILENT 23/23, CLEAN FP 2/23, MCC +0.917, $56.47) and contains no
error-frame observation; the strings `is_error` and `api_error_status` do not appear in the brief at all.

**AC-439** — `rk-adapter-claude`, `rk-kernel` · evidence: **F20** (exit 0, `subtype: success`, `structured_output` absent
under a passed schema; adding `StructuredOutput` fixes it) [measured] · decisions: proposal §4.3, **ADR-0074 (supersedes ADR-0004)**
(principle 4: absence of evidence is a block), ADR-0016
GIVEN a dispatch whose process exits 0 and to which a JSON schema was passed
WHEN the stream carries no `structured_output`
THEN the step resolves to `StepError::Schema` and is never recorded as success — the exit code is not evidence.
*Verified by:* an adapter unit test over the recorded F20 exit-0 schema-void fixture; a replay assertion across the
recorded stream corpus that no schema-void frame maps to success.

**AC-428** *(id re-issued; split from AC-439)* — `rk-adapter-claude`, `rk-kernel` · evidence: F21 and
`spikes/results/e3.jsonl` — **seven recorded `is_error` frames** (slugs `fix-doctor-git-config-repair`, `issue-122` §2,
`issue-130`, `issue-131`, `issue-72` for 529; `issue-82` for 500) [measured, PIA-075]; the condensed brief's F21 row
omitted the platform-error shape, which is why the 2026-09-08 pass found "none in the brief" · decisions: proposal §4.3,
**ADR-0074** (principle 4), ADR-0095
GIVEN a dispatch whose process exits with any code
WHEN the stream carries an `is_error` or `api_error_status` frame
THEN the step resolves to `StepError::Platform` and is never recorded as success (see AC-218 and §0.3 row C5 on
`Platform`'s absence from ADR-0095's table).
*Verified by:* an adapter unit test over the recorded `e3.jsonl` frames at the corpus pin; a replay assertion across the
recorded stream corpus. **Correction (2026-09-11, audit PIA-075).** PROVISIONAL is lifted: the fixture exists — seven
frames on disk now, at the corpus pin after runbook Step 2 — and the "delete if none is found" clause is withdrawn.
AC-218 is the harness-area owner of the same behaviour; §0.3 carries the twin row.

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-121 | `doctor` model cost | saturating |
| SM-122 | `doctor` wall clock at catalog scale | no baseline |
| SM-123 | `doctor` rule yield: real catches vs false alarms, per rule | — |
| SM-124 | `doctor --rebuild` next-action fidelity | no baseline |
| SM-125 | Void answer rate at the gate | — |
| SM-126 | `explain` sufficiency at a gate | no baseline |
| SM-127 | Platform and schema errors that returned success | — |
| SM-128 | Structured-field write-path violations | no baseline |
| SM-129 | CLI-version pin drift | no baseline |
| SM-130 | Re-dispatch of a memoised committed step | no baseline |

#### Residual risk this area could not repair

- **AC-428's id is REUSED** — the original was cut as a duplicate of AC-427 and the number re-issued to the split half of AC-439. A runbook indexing by id across document versions will collide. See §0.4.
- AC-422 remains COMPOUND (an `ANTHROPIC_API_KEY` doctor rule plus a '`--bare` in no composed argument vector' rule). Both halves carry an instrument, but no free id remained; split it before it reaches the build runbook.
- Every ADR number, proposal section and PC reference in this area is unverifiable against the evidence brief. About a third of the criteria (AC-405, AC-407, AC-409, AC-412, AC-413, AC-416, AC-422, AC-429, AC-437, AC-438) rest on decisions alone with no measured support.
- AC-426 is design intent with an instrument but ZERO evidence. AC-428 was in the same state until 2026-09-11: its unit test needs a recorded stream fixture carrying an `is_error` frame, and seven exist in `e3.jsonl` (F21) — the brief's F21 row simply omitted them (PIA-075).
- Several instruments are asserted but are really work items: the `redkiln-corpus` pin (AC-427, SM-124), the crate-graph test (AC-405), the `keywords`-unreachability lint (AC-436), the markdown-parser ban (AC-411). A reader may take 'Verified by' for 'verified'.
- AC-400's denylist is name-based by necessity: a resume-shaped flag under an unlisted name passes both the lint and, silently, the snapshot unless a human reads the diff.
- Section 1's idempotency story rests on a kernel test over a fake `HarnessPort`. No spike has measured memoisation: F38 measured resume losslessness at n=3 with a driver that minted its own mark, and the brief calls the second-arrival question circular.

---

## 13. `redkiln-migrate` and the clean break

**Area key:** `migration` · **Criteria:** AC-440..AC-479 **plus AC-521 and AC-522** (AC-477..AC-479 are splits; AC-521 and AC-522 are allocated outside the range — see §0.4) — **42 in force** · **Measures:** SM-133..SM-141 (SM-142..SM-144 unallocated)
**State:** 40 binding (AC-521 and AC-522 added 2026-09-11) · 1 provisional (AC-475) · 1 human-gated (AC-476) — 42 in force
**Sources:** proposal §5 (clean break and migration), §4.4 rev. 4, §6, §7; evidence brief §A/§C/§D/§E; ADR-0002, 0005, 0006, 0008, 0009, 0011, 0012, 0013, 0017, 0019 — all ten verified `status: accepted` in `references/adr/` at HEAD `ab58733`; **ADR-0100** (the YAML reader: `yaml_serde = "=0.10.7"` in `redkiln-migrate` alone), **ADR-0094** (self-hosting means both lanes; AC-521).

*Phase note (2026-09-11, PIA-002; **amended 2026-09-12, DECISIONS-2026-09-12 E13**).* The criteria bound to the frozen corpus (AC-444, AC-451–AC-454, AC-456–AC-462, AC-464, AC-467, AC-469 and every S-MIG-figured fixture) are a **Phase-2 acceptance set**: the 1,312-file corpus and the 118-ref history live only in `D:/repos/redkiln`, `redkiln-corpus` does not exist at HEAD, and no CI job fetches either [measured, PIA-002]. Runbook Step 2 materialises the substrate (a `git bundle create --all` of the frozen clone, or a fetchable remote plus SHA in `CORPUS_PIN`); until it lands these tests are a local-only suite with a named runner. What Phase 1's Step 7d owes is a **v0** — the binary exists (AC-441); the structural assertions AC-441/AC-466/AC-474; the report schema AC-448/AC-468; the run contract AC-447/AC-450/AC-522; and a dry run over a small fixture — with a checkpoint line that invokes `redkiln-migrate`.

*(Amended 2026-09-12, DECISIONS-2026-09-12 E13 — **two ids this note listed on both sides**, which it could not. **AC-468** was inside the corpus-bound range `AC-467–AC-469` **and** named in what Phase 1 owes ("the report schema AC-448/AC-468"). The range now reads AC-467, AC-469, and AC-468 splits: its **schema conformance** of the differential report is **Phase 1's** — asserted against the v0 dry run's own report, needing no corpus — while its **corpus-scale** assertion over the 1,312-file run is Phase 2's. **AC-475** is removed from the corpus-bound list and splits the same way: its `Verified by:` has two halves, and the second — "a schema assertion of zero `ChangeSummary` values in migrated output" — needs no corpus, runs on Step 7d's own dry-run fixture, and is a **negative** criterion, so asserting it before the code that could violate it exists is the cheap direction. The **loss-marker migration fixture** half stays Phase 2's. Runbook Step 7d claims AC-468 and AC-475 on exactly those readings and excludes the rest of the corpus-bound set by id.)*

*Scope note.* **S-MIG has no F-number.** The brief's ledger (F1–F38, F9 struck, F30 unassigned) contains no S-MIG entry; §D records only that S-MIG cost $0. Every S-MIG figure here is cited `[measured, S-MIG]` with its own n and was re-derived from `spikes/results/s-mig-report.md` and `spikes/corpus/PINNED_SHA` (`REDKILN_SHA=47faec9f…`). The instrument was iterated four times against this corpus before the recorded run.

**AC-440** — `rk-store` · evidence: proposal §5 (row 1: "not as — a runtime compatibility path") · decisions: ADR-0002
GIVEN a repository whose `.bklg`/`.kb` tree is in the old redkiln layout — detected by **one predicate**: the new
layout's manifest is absent **and** a `.bklg/` directory is present (PIA-033); that detector is the one legacy-format
site AC-442's allowlist admits outside `redkiln-migrate`
WHEN any `redkiln` command that opens the store is invoked against it
THEN the store refuses with a typed unsupported-layout error naming `redkiln-migrate`
AND the fixture tree's content digest is unchanged after the invocation.
*Verified by:* an integration test over an old-layout fixture directory taken from the pinned corpus, mounted read-only,
asserting (a) the typed variant and its `redkiln-migrate` message, and (b) tree-digest equality before/after; the error
variant is enumerated in `StepError`, so a silent-success path is a compile-visible missing arm.
**Instrument limit:** a read-only mount plus a digest check falsifies *writing*, not *reading* — "no file was parsed as
content" has no instrument, so the original wording asserted more than the test decides and has been narrowed to the
observable (unchanged tree).

**AC-441** — `rk-cli` · evidence: proposal §6 (`redkiln-migrate`: "separate binary"), §5 · decisions: ADR-0002
GIVEN the shipped `redkiln` CLI router
WHEN its full subcommand table is enumerated
THEN no subcommand performs or triggers a migration
AND `redkiln-migrate` is reachable only as its own binary.
*Verified by:* a snapshot test of the `clap` command tree asserting the absence of a migrate verb; a build-graph
assertion that no `rk-*` crate depends on the `redkiln-migrate` crate.

**AC-442** — workspace lint · evidence: proposal §5 table, rows 2–3 (process packs cross as *content for the new TOML
process packs*, not as code; 101 CLI-behaviour test files cross as *black-box specifications*, not as ported tests —
paraphrase, not quotation) · decisions: ADR-0002
GIVEN the redkiln-rs workspace
WHEN the legacy-format provenance lint runs in CI
THEN every site that reads or writes an old redkiln file format (`.redkiln/processes/*.yaml`, old `.bklg` frontmatter,
`template_sig`/`rendered_sig`) is located inside the `redkiln-migrate` crate
AND a legacy-format handling site anywhere else fails the build.
*Verified by:* a CI lint over the workspace with an explicit allowlist of one crate **plus one named site** —
`rk-store`'s old-layout detector (AC-440's predicate: manifest absent and `.bklg/` present), which reads a path, never a
format; a fixture PR that adds a legacy reader to `rk-store` and must fail; a fixture that widens the detector to parse
frontmatter and must fail.
**Instrument limit:** the lint is syntactic — it matches the enumerated legacy path globs and the literals
`template_sig`/`rendered_sig`. A legacy reader that constructs its paths or field names dynamically passes it. The lint
bounds accidental drift, not a determined one; the enumerated pattern set is the criterion's real scope.

**AC-443** — `rk-git` · evidence: proposal §5 (row 5: "the one code import"), §6 · decisions: ADR-0002
GIVEN the file-provenance manifest that records every file imported from the old redkiln
WHEN CI validates the manifest against the workspace
THEN the manifest contains exactly one entry, `reconcile.rs` — a re-implementation whose origin is the old tree's
`src/store/verify.ts` at pin `47faec9f` (the manifest records that path and its hash at the pin; the entry cannot
resolve to a hash of `reconcile.rs` itself, which was rewritten in the spike runner — PIA-078); after runbook Step 2
`rk-git` lifts it from `redkiln-corpus` at the pin, and the runbook's Step 6 names that location
AND a workspace file whose content hash matches any file in the old tree at its pin, and is not that entry, fails CI.
*Verified by:* a content-hash manifest test (`provenance.toml` vs. hashes of the old tree at pin `47faec9f`); the
failure mode is a hash match on any other path.
**Instrument limit — this is the weak point of the clean-break bound.** A content hash catches a *verbatim* copy only.
`reconcile.rs` itself crosses "rewritten in the spike" (proposal §5, row 5), so the sanctioned import would not have
been detected by this instrument either. A second import that is likewise rewritten is invisible to CI and is caught
only by review. The manifest is therefore a *declared*-import register with a verbatim-copy tripwire, not a detector of
imports; the original "a second imported file fails CI" overstated it. The clause "until ADR-0002 is amended" was cut:
CI can check the manifest, not whether an ADR was amended — that obligation is process, not a check.

**AC-444** — `rk-git` · evidence: F12 (360 trailer commits, 74 slices; ever-rejected 15/74 = 20.3%; sealed >once 21.6%)
· decisions: ADR-0002
GIVEN the rewritten `reconcile()` and the F12 fixture — 360 trailer commits over 74 slices **across all refs of the
frozen predecessor clone (118 refs, 46 of them reachable from `47faec9f`), materialised as a `git bundle create --all`
committed to `redkiln-corpus` and pinned by that repository's SHA, with the ref set asserted [WILL EXIST after runbook
Step 2 — PIA-002]**
WHEN the reconcile replay test runs
THEN it reproduces 15 of 74 slices as ever-rejected and 74/74 slice states without manual annotation, and the trailer
drift counts it reports are pinned to ADR-0105 clause 3's grammar table
AND a divergence on any slice fails the test.
*Verified by:* a replay test in `rk-git` against the pinned bundle, with a ref-set assertion beside the 40-hex one;
local-only until Step 2 lands the artefact. **Limit:** F12's registered primary was uncomputable (1 of 101 runs recorded
the field), so this replays the re-scoped measure, not the original one; F12's "at the pinned SHA" reads "across all
refs of the frozen clone".

**AC-445** — test provenance · evidence: proposal §5 (row 3: 101 CLI-behaviour test files cross "as black-box
specifications … not ported tests") · decisions: ADR-0002
GIVEN the **237 test files at the corpus pin** (the proposal's "101 CLI-behaviour test files" is a classification the
census never recorded; the lint hashes every old test file, and the 101's classification list is a `redkiln-corpus`
item — PIA-078, decision #11)
WHEN the provenance lint hashes every test file in the redkiln-rs workspace
THEN no workspace test file matches an old test file by content hash, or by body hash after a declared normalisation
(whitespace collapsed, comments and import lines stripped)
AND a file matching either way fails CI.
*Verified by:* the same content-hash manifest instrument as AC-443, run over test paths, with the normalisation
function specified in the lint and covered by its own fixture pair (one verbatim copy, one reformatted copy).
**Instrument limit, and a vacuity note.** The normalised-body hash catches reformatting; it does not catch a test
re-expressed in Rust idiom, which is the realistic way a behaviour gets ported. And this is a pure absence check: it
passes on a workspace with no tests at all. Its positive counterpart — that the behaviours actually crossed — is
AC-446, which has a non-empty denominator; neither criterion is meaningful without the other.

**AC-446** — regression corpus · evidence: proposal §5 (row 4: lock/id collisions, `--stay`, RK-B0013, `basis` flip,
F25's per-AC gap), F25 · decisions: ADR-0002
GIVEN the table of recorded old-redkiln bugs that cross as test cases
WHEN the bug-provenance test runs
THEN each listed bug id resolves to at least one named test in a new crate that fails under a mutation that
reintroduces the recorded bug's mechanism
AND a listed bug with no such test fails CI.
*Verified by:* a table-driven test binding bug id → test path; the discriminating mutation is applied by the same
per-AC instrument ADR-0019 requires, not by inspection.
**Why the mutation, not guard deletion.** The original wording required only that the test "fails when its guard is
removed". F25 measured exactly that distinction on AC-015: deleting the step turns the suite RED, but *inverting the
ordering* — a mutation the AC's claim should distinguish — leaves it **36/36 GREEN** [measured, F25]. Deletion-only
probes are the form F25 showed is insufficient, so the criterion must name the bug's own mechanism as the mutant.
**Scope note:** the source table (proposal §5, row 4) lists five entries, one of which — "F25's per-AC gap" — is a
method finding about this programme's own evidence, not a defect in the old redkiln that a regression test can pin.
It is excluded from the table this criterion binds.

**AC-447** — `redkiln-migrate` · evidence: proposal §5 ("in one reviewed commit"; "the migration commit normalises
everything once") · decisions: ADR-0002
GIVEN a migration run over a pinned corpus
WHEN the run completes successfully
THEN exactly one commit is created on the target branch — the dedicated migration branch/worktree AC-522 requires
AND a run that would produce more than one commit aborts before writing, and a run that would produce zero commits
aborts before writing **unless it is AC-450's no-op case** (tree-digest equality with the prior migration commit), which
exits 0 with its report on stdout and no commit (PIA-022).
*Verified by:* an end-to-end test asserting `git rev-list --count` of the migration range equals 1; an abort test that
injects a second write phase; the AC-450 no-op fixture asserting exit 0, zero commits and no abort.

**AC-448** — `redkiln-migrate` · evidence: proposal §5 ("with a differential report"), S-MIG (the report is the
artifact that made the type decisions) · decisions: ADR-0002, ADR-0012
GIVEN a migration run
WHEN the new layout is written
THEN a differential report is emitted in the same commit, carrying per-(kind, slot) counts of lifted / lifted-with-loss
/ failed and the corpus SHA
AND a run that writes the new layout without emitting a report is refused.
*Verified by:* a schema-validated report artifact (`jsonschema`) plus an end-to-end test that deletes the report step
and must fail.

**AC-449** — `redkiln-migrate` · evidence: proposal §5 ("the new store owns its serialiser"; R3 "dissolved rather than
mitigated") · decisions: ADR-0002, ADR-0006
GIVEN the same corpus SHA and the same `redkiln-migrate` version
WHEN the tool is run twice into two empty targets
THEN the two output trees are byte-identical and the two report digests are equal
AND the identity holds across the swept nuisance parameters: `TZ`, directory-read order, and hash-map seed.
*Verified by:* a determinism test comparing tree digests across two runs, the second under a shuffled directory-read
order, a different `TZ`, and a different `RUST_HASH_SEED`-equivalent.
**Narrowed:** the original AND clause said "no output depends on … the operator's environment", which is unbounded and
untestable. The criterion now asserts only what the sweep decides; an environment variable outside the swept set is
not covered.

**AC-450** — `redkiln-migrate` · evidence: proposal §5 ("normalises everything once") · decisions: ADR-0002, ADR-0006
GIVEN an already-migrated tree produced by `redkiln-migrate` at version V from corpus SHA C, and the same input (SHA C)
WHEN `redkiln-migrate` at version V is run against that target again
THEN the run is a **no-op, defined as tree-digest equality with the prior migration commit's tree** — the re-run's
report is written to stdout (or a non-committed artifact), not to the tree, and is not required to show zero failed or
zero with-loss records, since the input's failures are the input's (PIA-022)
AND the tool never emits a second normalisation commit, and AC-447's zero-commit abort does not fire on this case
AND a run **killed between write and commit** leaves the target branch untouched (AC-522's worktree) and the next run
re-reaches the same tree digest (PIA-002).
*Verified by:* an idempotence test (migrate → migrate → `git diff --exit-code` and tree-digest equality against the first
commit); a kill test (`taskkill /T /F` between the write phase and the commit) asserting the target branch's `HEAD` is
unchanged and the re-run's tree digest equals the uninterrupted run's.

**AC-522** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `redkiln-migrate` run
contract · evidence: `spikes/corpus/PINNED_SHA` (the corpus checkout was dirty at pin time) [documented]; F38 (a real
`taskkill /T /F` mid-run is the measured interruption shape) [measured, n=3] · decisions: ADR-0002, ADR-0013, ADR-0105
(the wrapper's write set: `worktree add`/`remove`, `commit`)
GIVEN a target repository for a migration run
WHEN `redkiln-migrate` starts
THEN it refuses, before any write, a target whose `git status --porcelain` is non-empty, naming the dirty paths
AND it writes only into a **dedicated migration branch checked out in its own worktree** (created through the wrapper),
so an interrupted run — killed between write and commit — leaves the target's original branch and working tree
untouched and the partial worktree removable
AND the one migration commit (AC-447) lands on that branch; integration into the target branch is a separate, reviewed
step (AC-476).
*Verified by:* a dirty-target fixture (one untracked file; one modified file) asserting refusal and zero writes; a
kill-between-write-and-commit test asserting the original branch's `HEAD` and tree are unchanged and the migration
worktree is left in a state the next run refuses to reuse (re-created, never reused — ADR-0105 clause 6); a positive
run asserting the commit is on the migration branch only. *(PIA-002 named the missing run-contract criterion.)*

**AC-451** — `redkiln-migrate` · evidence: S-MIG (1,312 files / 6,877 (artifact, section) records; 18 files unmapped)
· decisions: ADR-0002, ADR-0006
GIVEN a corpus whose section census counts N (artifact, section) records
WHEN a migration run completes
THEN the report accounts for exactly N records over the **closed outcome vocabulary** — `lifted`, `lifted-with-loss`,
`failed`, `opaque`, `unmapped-section`, `missing-expected`, `empty-template`, `dropped-by-decision` — mapped from the
census's outcomes `Lifted` / `LiftedWithLoss` / `Failed` / `Opaque` / `Unmapped` / `Missing` / `EmptyTemplate`, with the
plan companions and `plan.md` (AC-459/AC-479) as `dropped-by-decision`
AND a run whose accounting does not balance aborts rather than committing.
*Verified by:* a conservation assertion inside the tool plus a report-schema rule enumerating the eight buckets; the
pinned-corpus case asserts N = 6,877 with the per-bucket counts at the pin — `Opaque` 3,513 · `Lifted` 3,011 ·
`EmptyTemplate` 112 · `Unmapped` 81 · `LiftedWithLoss` 73 · `Failed` **54** · `Missing` 33 (one record carries no outcome)
[measured, `spikes/results/s-mig.jsonl`, recount 2026-09-11; PIA-033, PIA-078 — "53" read 54].

**AC-452** — `redkiln-migrate` · evidence: S-MIG limit 3 ("18 files are unmapped — evidence attachments and readings
under two archived projects … they migrate as opaque attachments; no type is proposed") · decisions: ADR-0006
GIVEN a file in the corpus that no lifter maps to a typed slot
WHEN the migration runs
THEN the file is carried across as an opaque attachment with its original bytes and a recorded content digest
AND no unmapped file is dropped, truncated, or reformatted.
*Verified by:* a test asserting digest equality for the 18 unmapped files at the pin and a nonzero-count assertion so
"zero unmapped" (which would mean the census stopped finding them) also fails.

**AC-453** — `redkiln-migrate` · evidence: S-MIG decision 2 ("the migration fills `unknown` for these 40 sections
rather than guessing"); free-form gates in 25% of 156 reports and 83% of 23 reviews · decisions: ADR-0006, ADR-0005
GIVEN a section that fails to lift into its typed slot
WHEN the migration writes that artifact
THEN the section is carried as opaque prose flagged with a migration-loss marker and its slot value is `unknown`
AND no typed value is inferred, guessed, or model-generated for it.
*Verified by:* a fixture set of failed sections at the pin asserting `unknown` + marker, drawn from the three slots with
the largest failure counts — impl-report gates 8 failed of 156, review gate table 12 failed of 23, companions 20 failed
of 45 [each measured, S-MIG]; AC-466 independently forbids the dispatch that a guess would need.
**Citation correction:** S-MIG decision 2 says the migration "fills `unknown` for these 40 sections", and it says it of
*gate* sections. The three counts above sum to 40 only by including the 20 companion failures, which are not gate
results, and the census's total failed count across all slots is **53**, not 40. The identity 8+12+20=40 was an
inference, not a recorded decomposition, and the claim that these fixtures *are* decision 2's 40 has been withdrawn;
the three counts stand individually.

**AC-454** — `rk-types` · evidence: S-MIG (spec/acceptance criteria 153 records: 134 lifted, 19 with loss, 0 failed =
**88%**, Wilson 95% on 134/153 [0.81, 0.92]; **76 loss lines for a missing GWT and 87 for a missing verification** — the
earlier 45 and 46 were cap-truncated at `loss[:6]`, ERRATA E-0075-1; prediction "ACs lift ≥95%" **falsified as stated**) ·
decisions: ADR-0006, ADR-0005, ADR-0075 (E-0075-2: migration never populates `gwt` or `verification`)
GIVEN any acceptance criterion in the corpus — **all 153 specs at the pin** — including one written as a plain sentence
with no GIVEN/WHEN/THEN and no verification note
WHEN it is migrated
THEN every AC round-trips **byte-identical with both `gwt` and `verification` absent** — migration never populates
either from criterion text; both are authored fields from the first CLI write (AC-137, decision #21) — and
schema-validates so.
*Verified by:* a property test over **all 153 specs** at the pin asserting migrate → read → validate with both fields
absent and criterion text byte-identical; a `jsonschema` conformance test asserting `gwt` and `verification` are
optional; a negative fixture in which the lifter is made to populate `gwt` from a "Given…" sentence, which must fail.
*(Amended 2026-09-11, PIA-033: "the 19 affected specs" → all 153; the never-populate rule made explicit.)*
**Repaired:** the THEN named the Rust type `AC { id, criterion, gwt: None, verification: None }` rather than an
observable, and its AND clause ("a type or schema that rejects it is a defect") restated the THEN instead of adding a
condition — a tautology no instrument can fail. Both replaced with the round-trip outcome.

**AC-455** — `rk-kernel` policy · evidence: S-MIG decision 1 ("the *policy* (not the type) decides whether a missing
verification blocks a stage") · decisions: ADR-0006, ADR-0019
GIVEN a spec containing an AC with `verification: None` and a process pack whose policy requires a verification at this
stage
WHEN the stage's admissibility is evaluated
THEN the stage is refused with the AC id named, at zero model cost
AND the same spec passes under a pack whose policy does not require it.
*Verified by:* two policy-table rows over one fixture spec; a `doctor` rule reporting the same at configuration time.

**AC-456** — `rk-store` / toolchain adapters · evidence: S-MIG decision 2 (gate results free-form in 25% of 156
implementation reports and 83% of 23 reviews; impl-report gates lift **75%**, review gate table **17%**) ·
decisions: ADR-0006, ADR-0074 (supersedes ADR-0004)
GIVEN a gate result recorded anywhere in the system
WHEN the workspace is linted
THEN no code path outside `redkiln-migrate` parses a gate result out of prose, bullets, or fenced output
AND a fixture PR adding such a parser to a non-migrate crate fails the build.
*Verified by:* the AC-442 provenance lint extended with a "no gate parser outside `redkiln-migrate`" rule, carrying the
same syntactic limit recorded there.
**Split from the original AC-456**, which bundled the provenance rule (this criterion) with the render round-trip
(AC-477); they have different instruments and one can pass while the other fails.

**AC-477** — `rk-store` render path · evidence: S-MIG decision 2 (gate results free-form in 25% of 156 implementation
reports and 83% of 23 reviews); proposal §4.4 rev. 4 ("gate results are **emitted by the toolchain and rendered**,
never parsed") · decisions: ADR-0006, ADR-0074 (supersedes ADR-0004)
GIVEN a typed gate result held in the store
WHEN the artifact carrying it is rendered to markdown and the typed value is read back from the store
THEN the value read back equals the value written, and the markdown is derived from it on every read
AND no production code path reconstructs the typed value from the rendered markdown.
*Verified by:* a render round-trip test (typed → markdown, then typed re-read from the store, not from the markdown)
asserting equality; the "never reads the rendered form back" half is the AC-456 lint, not this test.

**AC-457** — `redkiln-migrate` · evidence: S-MIG decision 3 ("the TDD evidence table migrates with `mutation: unknown`
on **142 of 156** rows; the F25 columns exist in **0** reports"), F25 · decisions: ADR-0019, ADR-0006
GIVEN a historical implementation report whose TDD evidence table has no Assertion/Mutation/Test-run/Red? columns
WHEN it is migrated
THEN every migrated row carries `mutation: unknown`
AND the tool never synthesises, infers, or back-fills a mutation outcome.
*Verified by:* a fixture assertion over the **142 of 156** reports whose TDD table lifted [measured, S-MIG decision 3],
every row `unknown`; a negative test that any non-null mutation value anywhere in migrated output fails.
**Grain note:** 142 is the count of reports whose table *lifted*, not a count of rows; the remaining 14 (7 with-loss,
7 failed) are covered by AC-453's `unknown` + loss-marker rule, not by this fixture. The universal THEN is what the
negative test enforces — the 142 figure is the positive fixture's size, and the two must not be conflated.

**AC-458** — `rk-kernel` verification gate · evidence: F25 (AC-015 probe: remove step 7 → RED; invert the ordering →
**36/36 GREEN**), F31 (four adjudicated defects survived 1890/1890 and 1907/1907 green), S-MIG decision 3 ·
decisions: ADR-0019
GIVEN an AC whose only evidence is a migrated TDD row with `mutation: unknown`
WHEN the verification gate computes whether that AC is pinned
THEN the AC is reported as unpinned and the gate does not treat it as satisfied
AND a green suite alone cannot flip it to pinned.
*Verified by:* a gate unit test over a migrated fixture report; the F25 probe fixture (ordering inversion) as the
discriminating case. This is the criterion that keeps the migration from importing the corpus's own 0/156 gap as
apparent evidence.

**AC-459** — `rk-store` schema / `redkiln-migrate` · evidence: S-MIG decision 4 ("**110 of 110** `plan.md` files are
the unfilled template"; planning lived in `_plan.md`, `_storymap.md`, `spec.md`) · decisions: ADR-0006
GIVEN the 110 `plan.md` scaffolds in the corpus
WHEN the migration runs
THEN no `plan` artifact is emitted for any of them, and the store's artifact-kind enumeration contains no `plan` kind
AND a schema or fixture that introduces one fails the enumeration test.
*Verified by:* an enumeration test over the store's published artifact-kind list; a migration test asserting 0 emitted
plan artifacts over the 110 scaffolds at the pin.
**Split from the original AC-459**, which bundled non-emission with the dropped-by-decision accounting (AC-479).
Non-emission can hold while the bytes vanish unrecorded — which is precisely the failure the second half guards.

**AC-479** — differential report accounting · evidence: S-MIG decision 4 ("**110 of 110** `plan.md` files are the
unfilled template") · decisions: ADR-0006, ADR-0012
GIVEN a corpus file that the migration emits no artifact for, by an explicit type decision
WHEN the differential report is written
THEN that file appears as a `dropped_by_decision` record carrying its path, byte count, content digest and the ADR id
of the decision
AND a file for which no artifact is emitted and no `dropped_by_decision` record exists leaves the AC-451 conservation
identity unbalanced, which aborts the run.
*Verified by:* a migration test asserting 110 `dropped_by_decision` records for the `plan.md` scaffolds at the pin,
each resolving to an Accepted ADR id; a negative test deleting one such record, which must abort on AC-451's balance
check rather than commit. **Grain note:** AC-451 balances *(artifact, section) records*; this criterion is stated over
*files*. Both terms must appear in the report at both grains or the negative test above does not fire — that is a
requirement on the report schema (AC-448), recorded here so the dependency is not assumed.

**AC-460** — `rk-store` sidecars · evidence: S-MIG (companions / frontmatter: 45 records, 19 lifted, 6 with loss, 20
failed = **42%**, RECLASSIFY; `_grounding`/`_decomposition` have no frontmatter in 10 of 28 cases) · decisions: ADR-0006
GIVEN a companion artifact of a kind counted in the census companion row (`_integration`, `_grounding`,
`_decomposition`, `_implementation`)
WHEN it is migrated
THEN it is written as a JSON sidecar validated against that companion kind's own schema
AND a companion with no frontmatter yields an empty-but-valid sidecar of its own kind, never a stage-schema-shaped guess.
*Verified by:* one `jsonschema` per companion kind plus a migration test over the census companion records at the pin,
including the 10 of 28 `_grounding`/`_decomposition` cases with no frontmatter [measured, S-MIG decision 5].
**Fixture correction.** The original GIVEN listed `_ledger` and omitted `_plan`, but the 45-record companion row it
cites is *(integration, grounding, decomposition, implementation, **plan**)* — `_ledger` is a separate census row
(fenced YAML, n=110, 108/0/2) governed by AC-461, and the plan companions are dropped by AC-459/AC-479. So the
criterion's kind list and its named fixture did not describe the same records. The list is now the census row minus
plan. **The split is now recorded (2026-09-11, PIA-002):** the two `companion.plan` frontmatter records (1 lifted, 1
failed) are AC-459/AC-479's, and this criterion's fixture is **n = 43 — 18 lifted / 6 with loss / 19 failed**
(`integration` 13: 0/6/7 · `grounding` 13: 7/0/6 · `decomposition` 15: 11/0/4 · `implementation` 2: 0/0/2) [measured,
`spikes/results/s-mig.jsonl` frontmatter slots, recount 2026-09-11]. Under decision #3 of 2026-09-11 only `_integration`,
`_grounding` and `_decomposition` of these are Phase-1 kinds; `_implementation` is a Phase-2 kind lifted opaque.

**AC-461** — `redkiln-migrate` · evidence: S-MIG decision 6 ("**two** `_ledger.md` files are unreadable YAML —
unescaped quotes inside quoted evidence strings … real content, hand-fixed in migration") · decisions: ADR-0002
GIVEN an input file whose frontmatter or fenced block does not parse (read with `yaml_serde = "=0.10.7"`, the one YAML
reader in the workspace, in this crate alone — ADR-0100)
WHEN the migration runs
THEN the run fails loudly naming file and line, and completes only after the fix is recorded as an enumerated manual
intervention in the **committed carrier `interventions.toml`** in the `redkiln-migrate` crate — one entry per fix
carrying `path`, `line`, `reason`, the replacement bytes and the digest of the original (PIA-033) — which the
differential report enumerates
AND the file is never dropped, and its content is never repaired by heuristic
AND `interventions.toml` is an input to AC-449's determinism sweep and part of AC-467's versioned lifter set.
*Verified by:* a fixture pair (the two ledger files at the pin) asserting a hard failure without an intervention record
and a report entry with it; a digest-mismatch fixture (an intervention whose original digest no longer matches) that
must fail.

**AC-462** — differential report gate · evidence: S-MIG §4.3 decision rule as exercised (≥95% stands; 80–95% REVIEW —
ACs at 88%; <80% RECLASSIFY — impl-report gates **75%**, review gate table **17%**, companions **42%**) ·
decisions: ADR-0012, ADR-0006
GIVEN a differential report in which some (kind, slot) has a lifted share below 80%
WHEN the report gate evaluates it
THEN that slot is labelled RECLASSIFY and the report is not green until an ADR id recording the type decision is
attached to that slot
AND an attached id that does not resolve to an Accepted ADR fails the gate.
*Verified by:* a report-gate rule over the report schema; the three known RECLASSIFY slots at the pin as its fixture,
each resolving to the type decision S-MIG settled.

**AC-463** — Phase 2 exit · evidence: proposal §7 Phase 2 exit ("migration differential green on the frozen corpus"),
ADR-0013's consequences · decisions: ADR-0013, ADR-0012
GIVEN the Phase 2 exit check
WHEN it evaluates "migration differential green on the frozen corpus"
THEN green is decided mechanically as: section accounting balances (AC-451), zero unrecorded manual interventions
(AC-461), and every sub-80% slot carrying an Accepted ADR id (AC-462)
AND no other reading of "green" is accepted by the check.
*Verified by:* a single CI check consuming the report artifact; the definition lives in the check, not in prose.

**AC-464** — corpus pin · evidence: ADR-0013 consequences ("depend on an exact, pinned corpus SHA rather than a moving
target"); `spikes/corpus/PINNED_SHA`, which records the corpus checkout as *dirty at pin time* (a modified telemetry
partition, untracked references) and states "corpus extraction reads committed state at REDKILN_SHA, not the working
tree" · decisions: ADR-0013
GIVEN a migration or differential run
WHEN it reads corpus bytes
THEN it reads them from the pinned SHA in `redkiln-corpus` via the object store — the **predecessor's committed state at
`47faec9f`, vendored into `redkiln-corpus` as a bundle (or declared as a fetchable remote plus SHA in `CORPUS_PIN`) and
pinned by that repository's own SHA** [WILL EXIST after runbook Step 2; PIA-002] — never from a working tree
AND a run against an unpinned reference, or a corpus checkout with uncommitted changes, is refused before any read.
*Verified by:* a runner test with a dirtied checkout that must be refused; the read path takes an object-store handle
and has no filesystem-walk alternative (a type-level constraint, not a convention).
**Citation corrected:** the draft cited brief §E's "Working tree not clean as of 2026-09-09", which is about *this*
repo at HEAD `ab58733`, not about the corpus checkout, and so did not support the criterion it was attached to. The
pin file does support it, and does so directly — the corpus was already dirty when it was pinned.

**AC-465** — repo hygiene · evidence: proposal §5 (row 6: the corpus, E3 fixtures, JSONL results and spike runner move
out; "not … a permanent resident of the product repo") · decisions: ADR-0013
GIVEN the redkiln-rs product repository
WHEN the corpus-residency lint runs in CI
THEN no corpus artifact (frozen `.bklg`/`.kb` trees, E3 fixtures, spike JSONL) is tracked in the product repo
AND the only reference to them is the pinned SHA.
*Verified by:* a path/extension lint with a pin-manifest allowlist; a fixture commit re-adding a JSONL results file
must fail.
**Baseline state, recorded:** this criterion is **currently violated by construction** — `spikes/corpus/`,
`spikes/results/*.jsonl` (including the 1.3 MB `s-mig.jsonl` every share figure here derives from) and
`spikes/runner/` are all tracked in this repo at HEAD `ab58733`. The lint cannot be switched on until the
`redkiln-corpus` split (ADR-0013) actually happens, and that repo does not yet exist. This is a future-state criterion,
not a maintained invariant, and no measure tracks the gap. **The E3 fixtures are deliberately local, unversioned and
regenerated** by the runner's `build_fixture_at` in `redkiln-corpus` (PIA-022) — they are not copied or bundled, and
the lint's allowlist names the regeneration script, not the fixtures.

**AC-466** — `redkiln-migrate` · evidence: S-MIG ("zero model cost", 1,312 files); brief §C.12 (every discriminating
result came from cost, stability or mechanism, never from success rates) · decisions: ADR-0002
GIVEN any `redkiln-migrate` invocation
WHEN it runs to completion or to an abort
THEN zero harness dispatches are made and the binary links no harness adapter
AND an attempt to construct a harness port inside the migration path fails at build time.
*Verified by:* a build-graph assertion that `redkiln-migrate` does not depend on `rk-adapter-claude`/`-codex`; a
run-level assertion of zero `dispatch` events in the event log.

**AC-467** — differential report provenance · evidence: S-MIG limit 1 ("the lifters were iterated four times against
the corpus before the recorded run … it is not a blind test of a parser") · decisions: ADR-0012
GIVEN a differential report
WHEN it is validated
THEN it carries both the corpus SHA and the lifter/tool version, and a report missing either is rejected
AND a change to any lifter invalidates prior reports until a full re-census at the same pin is recorded.
*Verified by:* report-schema required fields; a CI rule comparing the lifter-source digest in the report against the
built binary's, failing on mismatch.

**AC-468** — differential report shape · evidence: S-MIG limit 2 ("'lifted' means the section parsed into the slot's
shape, **not** that the migration is lossless") · decisions: ADR-0006
GIVEN a differential report
WHEN any consumer (CI gate, human review, dashboard) reads a slot's outcome
THEN lifted, lifted-with-loss and failed are three distinct counts
AND a report or consumer that presents only a combined total, or folds with-loss into lifted, fails validation.
*Verified by:* the report `jsonschema` (three required integer fields, no `total`-only form); a consumer test asserting
the gate reads all three.

**AC-469** — `redkiln-migrate` fixtures · evidence: S-MIG limit 4 ("the fix-lane generation (42 specs) and the archived
generation (110) differ in template; both are in the count") · decisions: ADR-0002
GIVEN the two template generations present in the corpus
WHEN the migration test suite runs
THEN both generations are covered by named fixtures and both migrate under the same lifter set
AND each generation's fixture fails when that generation's recognition branch is removed from the lifter.
*Verified by:* generation-tagged fixture sets (fix-lane **n = 43** — the 42 plus `issue-146-147-boundary-window/spec.md`,
assigned here by decision #11 of 2026-09-11 (PIA-078); archived n=110) in the migration test suite; the AND clause is
checked by the ADR-0019 instrument removing each branch in turn, not by inspection — otherwise "both are covered" is
satisfied by a fixture set in which one generation's cases happen to pass under the other's lifter.
**Limit:** one frozen SHA — a single draw; no unseen corpus has been migrated.

**AC-470** — `rk-store` canonical serialiser · evidence: proposal §5 ("the YAML byte-parity risk (R3) is dissolved
rather than mitigated: the new store owns its serialiser and the migration commit normalises everything once") ·
decisions: ADR-0006
GIVEN every artifact written by the migration commit
WHEN each is read back and re-serialised by the canonical serialiser
THEN the bytes are identical for 100% of migrated artifacts
AND no byte-parity with the old YAML serialiser is attempted or asserted anywhere.
*Verified by:* a round-trip test over the whole migrated tree; the absence assertion is the AC-442 provenance lint (no
legacy emitter outside the migrate crate).

**AC-471** — `rk-types` · evidence: proposal §4.4 rev. 4 ("A schema change is a migration, and the old redkiln's
`template_sig`/`rendered_sig` drift is the failure to avoid") · decisions: ADR-0006, ADR-0005
GIVEN any typed artifact in the new store
WHEN the derived schema set is enumerated
THEN every artifact schema declares a `schema_version`, and no schema declares a template- or rendered-signature field
AND adding a `*_sig` field to any type fails the enumeration test.
*Verified by:* a schema-enumeration test over the derived schemas, asserting the `schema_version` field is present on
each and that no field name matches the signature pattern; a fixture type carrying a `*_sig` field that must fail.
**Split from the original AC-471**, whose second clause ("a rendering mismatch is unrepresentable as drift: the
rendered form is derived, never stored as truth") is a separate behaviour with a separate instrument — now AC-478.
Absence of a `*_sig` field does not by itself establish that the rendered form is never stored as truth.

**AC-478** — `rk-store` render path · evidence: proposal §4.4 rev. 4 ("A schema change is a migration, and the old
redkiln's `template_sig`/`rendered_sig` drift is the failure to avoid") · decisions: ADR-0006, ADR-0005
GIVEN a typed artifact whose rendered markdown on disk has been edited to disagree with the typed values
WHEN the artifact is next read
THEN the typed values are returned unchanged and the markdown is regenerated from them, overwriting the edit
AND no comparison of stored-vs-regenerated rendering is performed, so no drift state exists to report.
*Verified by:* a mutation test that edits the rendered markdown of a stored artifact and asserts the next read returns
the original typed values and rewrites the file; a schema assertion that no artifact carries a drift or
signature-comparison field. **Note:** this makes hand-edits to rendered markdown silently lossy, which is the
intended trade (ADR-0005 puts human edits in prose fields, not in rendered structure) and is stated so it is a
recorded consequence rather than a surprise.

**AC-472** — `rk-store` / artifact instrumentation · evidence: S-MIG decision 4 (`plan.md` dead in 110/110, found by
census long after it shipped); proposal §4.4 rev. 4 ("every activity input emits an `artifact.read` event so a stage
whose artifact is never read is flagged by `doctor`") · decisions: ADR-0006
GIVEN an activity that consumes an artifact as input
WHEN the activity runs
THEN an `artifact.read` event is emitted naming the artifact kind and id
AND `doctor` flags any artifact kind with zero reads across the last N runs, where N is a declared constant in the
rule table.
*Verified by:* an event-log assertion per activity input in the kernel test suite; a `doctor` rule-table row with a
fixture of N+1 runs in which one artifact kind is never read and must be flagged, and a paired fixture at N−1 runs in
which it must not be — so the threshold itself is exercised.
**Gap:** N is not yet chosen. Until it is, the AND clause has no truth value and this criterion is only half live; the
`artifact.read` emission (the first clause) is checkable today. SM-139 inherits the same gap.

**AC-473** — self-hosting switch · evidence: proposal §5 ("until Redkiln-RS is self-hosting … that milestone, not a
phased cutover, is the switch"), §7 Phase 1 exit · decisions: ADR-0002, ADR-0011
GIVEN a claim that Redkiln-RS is self-hosting
WHEN the release check evaluates it
THEN it resolves to at least one recorded fix-lane run under Redkiln-RS **on `D:/repos/redkiln-rs` itself — a Cargo
workspace, so the toolchain stage ran through `rk-adapter-toolchain-rust` behind the `Toolchain` port (ADR-0094
decision 1)** — whose event log contains every lane stage from intake to a landed commit, with no stage performed by
the old redkiln
AND the claim is refused if any stage is missing from the log, or if the toolchain stage was performed by anything but
the Rust adapter through the port
AND — self-hosting means **both lanes** (ADR-0094 decision 3; ADR-0011:47's wording is the exit bar) — the story-lane
half is AC-521, checked in the same release.
*Verified by:* a release check over the event log (stage-completeness assertion; an adapter-identity assertion on the
toolchain stage's dispatch record); the run id is the evidence, not a statement.

**AC-521** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — self-hosting switch, story
lane · evidence: ADR-0011:47 ("runs its own fix lane end to end (self-hosting) **and Phase 2's stories are authored under
it**") [documented]; ADR-0094 decision 3 [documented]; nothing measured bears on a story-lane run, which is why this is a
release check and not a threshold · decisions: ADR-0094, ADR-0011, ADR-0002
GIVEN a claim that Phase 1 is complete
WHEN the release check evaluates it
THEN it resolves to at least one recorded `story`-lane run under Redkiln-RS on `D:/repos/redkiln-rs` whose event log
contains every story-lane stage and whose output is a Phase-2 story authored into this repository's own backlog by that
run
AND the claim is refused if no such run exists, if any stage is missing from its log, or if the authored story is not
reachable from the run's event log (runbook Step 8 check (1b); design §10.2's Phase-1 exit).
*Verified by:* a release check over the event log (stage-completeness assertion per story-lane stage; an assertion that
the story artifact's digest appears in the run's `artifact.write` events); the run id and the story id are the
evidence. Twin of AC-473 (§0.3).

**AC-474** — clean break, negative · evidence: proposal §5 ("The old redkiln keeps running its own development until
… self-hosting"; "not a phased cutover") · decisions: ADR-0002
GIVEN the shipped Redkiln-RS binaries and their configuration surface
WHEN the workspace is linted and the config surface is enumerated
THEN no emitter of an old redkiln file format exists outside `redkiln-migrate`
AND no configuration key, flag or environment variable enables a dual-write or compatibility-write mode.
*Verified by:* the AC-442 provenance lint (no legacy emitter, with that lint's syntactic limit) plus a config-surface
snapshot test over the full key list, asserting no dual-write key exists.
**Clause cut.** The draft's THEN began "it never writes into a tree the old redkiln manages" — a runtime property with
no instrument: neither named check can tell whose tree a path belongs to, and nothing here detects the old redkiln's
presence. That clause was unverifiable and has been removed rather than left as an assurance. What remains is exactly
what the two instruments decide. **Note:** this constrains the new side only; nothing in this repo can verify the old
redkiln's behaviour, and no criterion here should pretend otherwise.

**AC-475** — `redkiln-migrate` / `rk-types` — **PROVISIONAL** · evidence: brief §E (S-BIN registered and **unrun**;
gates Phase 1 for `ChangeSummary` type design); proposal §7 Phase 1 (`ChangeSummary` designed against the UE fixture) ·
decisions: ADR-0008, ADR-0012
GIVEN historical change/diff material in the corpus and no settled `ChangeSummary` type
WHEN the migration encounters it
THEN it is carried as opaque content with a migration-loss marker
AND no `ChangeSummary` value is synthesised.
*Verified by:* a migration fixture asserting opaque carry with a loss marker, and a schema assertion of zero
`ChangeSummary` values in migrated output.
**PROVISIONAL — blocked on S-BIN (registered, unrun; brief §E).** When S-BIN settles the type, this criterion is
replaced by a lift criterion with its own measured share; until then any criterion asserting a `ChangeSummary` lift
rate would be invented. **Vacuity disclosure:** the second conjunct ("zero `ChangeSummary` values") is trivially true
while the type does not exist — it becomes a real constraint only once S-BIN lands, and it is kept as a tripwire
against a type appearing in the migration path before the spike that is supposed to design it. The load-bearing half
today is the first conjunct: opaque carry with a marker.

**AC-476** — migration commit review — **HUMAN GATE** · evidence: proposal §5 ("emits the new typed layout in **one
reviewed commit**, with a differential report") · decisions: ADR-0002, ADR-0009
GIVEN a completed migration commit and its differential report
WHEN the commit is proposed for landing
THEN a human answers a gate whose question is whether the recorded losses (with-loss and failed sections, dropped-by-
decision scaffolds, manual interventions) are acceptable
AND the commit does not land on a machine verdict alone.
*Verified by:* routed to a human gate; **not machine-decidable** — the mechanical checks (AC-451, AC-461, AC-462,
AC-463) establish that the losses are *enumerated*, but whether a given loss is acceptable is a judgement about
content. The gate record is a token + tree hash + deadline (ADR-0017 area), and the gate's cost is bounded and flat in
duration [measured, F28/F18: 18,603 tokens at both 65.2 min and 49.7 h; gate→verdict median 10.5 min, 45% > 1 h, one
operator].

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-133 | Migration lift share per (kind, slot) | — |
| SM-134 | Section conservation (unaccounted records) | saturating; no baseline |
| SM-135 | `unknown` density in migrated artifacts | — |
| SM-136 | Migration determinism and idempotence | saturating; no baseline |
| SM-137 | Manual interventions per migration | — |
| SM-138 | Migration cost | — |
| SM-139 | Dead-artifact detection latency | — |
| SM-140 | Legacy surface count | no baseline |
| SM-141 | Self-hosting: recorded end-to-end fix-lane runs | saturating |

#### Residual risk this area could not repair

- **THE CLEAN-BREAK BOUND IS WEAKER THAN IT READS.** AC-443/AC-445/SM-140 rest on content hashing, which detects only verbatim copies. The one sanctioned import, `reconcile.rs`, crosses *rewritten* — so the instrument would not have caught the very file it exists to license. A second rewritten import, or a test re-expressed in Rust idiom, passes CI silently.
- Every share figure is an upper bound from one draw and the criteria read as thresholds. n = 1 corpus, 1 pin, 1 operator; the census instrument was tuned to this corpus, and its per-AC loss list was cap-truncated at `loss[:6]` until the 2026-09-11 re-run (E-0075-1). **Two** Wilson intervals exist on S-MIG figures — 134/153 (AC-454's falsification, [0.81, 0.92]) and 142/156 (SM-135, 85.5–94.6%) — not "exactly one" (PIA-107; design §8.7 said the same and is amended the same way); every other share carries no interval.
- AC-460's fixture size was unknown until 2026-09-11; the split is now recorded from `s-mig.jsonl` (n = 43, two plan records to AC-459/AC-479). The residue is that three of its four kinds are Phase-1 kinds and one (`_implementation`) is Phase-2 opaque under decision #3.
- The census overall is **96.0%** typed-slot (3,011 / 3,138 — Lifted / (Lifted + LiftedWithLoss + Failed) over typed slots); the "98%" some documents quote is the spec-section slot only (E-0075-1). No single percentage stands for the census (SM-133).
- SM-133's target compares two different programs — a read-only census (`census-s-mig.py`) against a migration tool that does not exist. Until `redkiln-migrate` reproduces the census figures on the same pin, a slot falling below baseline cannot distinguish a regression from instrument disagreement.
- AC-465 is VIOLATED TODAY: `spikes/corpus/`, `spikes/results/*.jsonl` and `spikes/runner/` are all tracked at HEAD `ab58733`, and `redkiln-corpus` does not exist. Nothing measures the distance to compliance.
- AC-472/SM-139's doctor rule is the only instrument for dead-artifact detection and its N is undeclared, so the AND clause has no truth value. Its kill depends on someone running a census to notice the rule did not fire — an instrument whose failure is only detectable by the manual method it replaces.
- NOTHING HERE HAS BEEN EXECUTED. `redkiln-migrate` does not exist; the differential numbers come from a read-only census. Forty criteria against an unbuilt tool is a specification, not evidence.

---

## 14. Ports, fakes, plugin generation and distribution

**Area key:** `ports-dist` · **Criteria:** AC-480..AC-519 **plus AC-528** (AC-513 retired; AC-511, AC-512 are reused ids; AC-528 is allocated outside the range — see §0.4) — **40 in force** · **Measures:** SM-145..SM-149, SM-151, SM-153, SM-154 (SM-150 and SM-152 cut; SM-155, SM-156 unallocated)
**State:** 31 binding (AC-528 added 2026-09-11) · 2 provisional (AC-496, AC-497) · 1 human-gated (AC-482) · 6 conditional (AC-512, AC-514, AC-515, AC-516, AC-517, AC-519 (partly)) — 40 in force
**Sources:** proposal §2, §4.1, §4.8, §4.9, §6, §7, §8 (with §3 principles 5/9/10/11/13 and §9 D3/D7/D8/D10/D13/D16); ADR-0003, 0007, 0008, 0010, 0013, 0014, 0016, 0017, 0018; **ADR-0094** (four Phase-1 real adapters — SQLite, Claude, Node, Rust — and the phase clause on AC-499), **ADR-0099** (ports are `Arc<dyn Port + Send + Sync>`; the Harness signature; `rk-*` crate names), **ADR-0076** (ports stay sync; AC-528); `references/2026-09-01-shipmates-lessons-for-redkiln-rs.md` L1.2, L3.1–L3.8, L4.1–L4.4, L5.3, L5.4, L5.7, L5.8.

*Phase-1 subset (2026-09-11, PIA-071/PIA-072).* Runbook Step 7a's acceptance set is the Phase-1 subset of this area, not the whole range: **AC-480–AC-495, AC-497–AC-500, AC-509, AC-511, AC-518, AC-528**, with AC-499 instantiated only for ports that have a Phase-1 real adapter (ADR-0094). Out of Phase-1 scope, with their status: AC-501–AC-506 (Phase 2, `rk-plugin-gen`), AC-507/AC-508/AC-510 (Phase 3, GitHub and mdBook), AC-512/AC-514–AC-517/AC-519's receipt and rename halves (CONDITIONAL, unbuildable until a distribution mechanism is decided), AC-496 (PROVISIONAL on S-CODEX; the Codex adapter is **Phase 2** per ADR-0094 decision 2, behind the CI decision ADR-0010 defers — the residual-risk list below said Phase 4), AC-482 (human-gated), AC-513 (retired). AC-497 ships its port shape with `ChangeSummary` as an opaque, non-finalised newtype in `rk-ports` until S-BIN (ADR-0099 clause 5), which is how it coexists with Checkpoint 5's grep.

*Scope note.* **Two standing caveats.** (1) *Borrowed evidence*: every distribution finding (L3.1–L3.8, L5.3) is `[documented]` from shipmates' issue history — one project, one maintainer, one release series; Redkiln-RS has shipped no binary and has no distribution baseline of its own. (2) *Borrowed architecture, and it is not the decided one*: proposal §4.9 decides a plain git clone containing generated `commands/`, `agents/`, `skills/` plus a JS shim that fetches the binary from GitHub Releases. It decides **no installer, no `build.rs`-embedded payload, no receipts, no rename table, no ownership states and no integrity scan**. The six CONDITIONAL criteria bind only if a distribution mechanism with an install-and-repair step is decided, and none of them may be read as licensed by §4.9.

### 14.1 Port shape and adapter selection

**AC-480** — `rk-kernel`, `rk-ports` · evidence: proposal §4.1 port rules ("one trait per port, no generic parameters leaking into the kernel"); §3 principle 9 · decisions: ADR-0003
GIVEN a port trait in `rk-ports`
WHEN `rk-kernel` is compiled
THEN no function, struct or trait in `rk-kernel`'s public or internal API carries a generic type parameter bound by a port trait
AND every port is held as a single object-safe trait object — `Arc<dyn Port + Send + Sync>` (the **EventStore** port is `Arc<Mutex<dyn EventStore>>`, because `dispatch` takes `&mut dyn EventStore` — **ADR-0115**, narrowing ADR-0099 clause 4), with `Send + Sync` as supertraits on all
five traits, a bare trait object and not a newtype (ADR-0099 decision 4; the divergence from M-DI-HIERARCHY's ranking is stated there).
*Verified by:* an API-surface lint over `rk-kernel` (rejects `<P: Harness>`-shaped bounds and any `impl Trait` return naming a port);
a compile-fail test fixture that introduces one such parameter and must not build.

**AC-481** — `rk-ports` · evidence: proposal §4.1 ("a fake per port is a hard requirement of adding the port"); the §4.1 port table names all five fakes (in-memory Vec, scripted responder, in-memory board, no-op recorder, fixture-driven); shipmates L5.8 (count-as-assertion) · decisions: ADR-0003
GIVEN the set of port traits declared in `rk-ports` — **derived from source** (every `pub trait` in `rk-ports` carrying the `#[port]`
registration attribute; every fake carrying `#[fake_for(Trait)]`), never from a hand-maintained inventory list
WHEN the workspace test suite runs
THEN the count of fake implementations equals the count of port traits, and the equality is asserted as a number, not as a spot check
AND a port trait added without a fake fails the suite naming the missing fake, by name, from the same source parse.
*Verified by:* a `rk-ports` inventory test asserting `traits.len() == fakes.len()` over the source-derived sets; a fixture branch adding a
sixth trait with no fake **and editing no inventory** that must fail naming it. *(Amended 2026-09-11, PIA-059: an inventory the fixture
also had to edit passed on the behaviour it forbids.)* The fake count is per port and unchanged by ADR-0094's second Toolchain adapter.

**AC-482** — architecture review · evidence: proposal §3 principle 9 ("a port exists where a substitute has been named, or where the fake alone pays for it") · decisions: ADR-0003, ADR-0017
GIVEN a proposal to add a sixth port
WHEN the change is reviewed
THEN it is admitted only if a concrete substitute adapter is named, or the fake alone is judged to pay for the abstraction.
*Verified by:* routed to a human gate; not machine-decidable — "is this a named variation axis?" is a judgement about the world outside
the repo (which alternatives are real), and the failure mode ADR-0003 guards against is exactly a plausible-sounding port with no substitute.
The gate record is `(token, tree_hash, deadline)` per ADR-0017. The machine-checkable half is AC-481 (the fake must exist).

**AC-483** — `rk-git`, `rk-ports` · evidence: proposal §4.1 ("**Git is deliberately not a port.** Trailers, worktrees, shallow clones for review, and `reconcile()` are load-bearing and have measured behaviour (F12, F19). No alternative VCS is named."); §5 (`reconcile.rs` is the one code import, rewritten in the spike runner and **exercised** over 360 commits / 74 slices, F12 [measured] — not validated against an oracle; PIA-075) · decisions: ADR-0003, ADR-0105 (`GitOps` is a struct inside `rk-git`, not a port — decision 1)
GIVEN the workspace
WHEN the port inventory lint runs
THEN no trait in `rk-ports` abstracts git, and `rk-kernel` depends on `rk-git` as a concrete crate
AND a trait whose name or methods match the git surface (trailers, worktrees, clone, `reconcile`) is rejected by the lint.
*Verified by:* the port inventory lint (denylist on the git surface); a compile-fail fixture declaring `trait GitPort`.
Note: F12's ever-rejected rate (15/74) and F19's drift rate (12/174) are cited in this area elsewhere and do **not** bear on this criterion.
What F12/F19 license here is only what the proposal claims from them — that the git surface has *measured behaviour*, so it is a concrete
dependency rather than a variation axis. The rates themselves are struck from this row as decoration.

**AC-484** — `rk-kernel` · evidence: proposal §4.1, §6 crate layout · decisions: ADR-0003
GIVEN the workspace dependency graph
WHEN CI runs the dependency check
THEN `rk-kernel` has no dependency edge — direct or transitive — on any `rk-adapter-*` crate
AND an added edge fails CI naming the adapter crate.
*Verified by:* a `cargo metadata` dependency-graph assertion in CI; a fixture commit adding `rk-adapter-sqlite` to `rk-kernel` that must fail;
the same `deny.toml` `[bans].deny` list AC-005 names (ADR-0100 decision 3), so the two instruments read one committed list.

**AC-485** — `rk-cli`, `rk-kernel` · evidence: proposal §4.1 ("adapters selected once at startup from `.redkiln/config.toml`") · decisions: ADR-0003
GIVEN a run whose adapter set was resolved at startup
WHEN `.redkiln/config.toml` is edited mid-run to name a different adapter
THEN the running run's adapter set does not change, and the adapter set recorded on the run-start event is the one used for every
subsequent dispatch in that run.
*Verified by:* an integration test that mutates the config file between two activities and asserts adapter identity on both dispatch
events; a replay assertion that the adapter set is a run-scoped constant in the event log.

**AC-486** — `rk-cli` (`doctor`), `rk-kernel` · evidence: proposal §3 principle 5 ("explicit over discovered"; adoption and isolation verified from the child's own report), §4.6 (`redkiln doctor` rules are "all configuration-time, zero model cost") · decisions: ADR-0003
GIVEN `.redkiln/config.toml` names an adapter that is not compiled in, or names one whose required configuration is absent
WHEN the run starts
THEN the run is refused as a configuration error before any process is spawned and before any store write
AND `redkiln doctor` reports the same defect at configuration time at zero model cost.
*Verified by:* a startup-resolution unit test; a `doctor` rule-table row; a fixture config naming `rk-adapter-linear` that must fail.
Note on evidence: F7 (`--agents` inert without `--agent` [measured]) and F8 (a default `claude -p` inherits 12 MCP servers / ~280 tools
[measured]) are the *motivation* for principle 5 — they are why "explicit over discovered" is a principle — but neither measures anything
about startup refusal. The generalisation "an unhonoured setting looks identical to an honoured one" is `[inferred]` from them, not measured.

**AC-487** — `rk-kernel` · evidence: proposal §4.1 ("capability differences are data … never `if adapter == …` in the kernel"); shipmates L4.3 · decisions: ADR-0003
GIVEN kernel code that must behave differently depending on what an adapter can do
WHEN the kernel lint runs
THEN `rk-kernel` contains no comparison against an adapter identity — no adapter name string, no `Any::downcast`, no
adapter-discriminant enum
AND an introduced identity check fails the lint naming the file and line.
*Verified by:* a kernel lint (denylist: adapter crate names, `downcast_ref`, `downcast_mut` on port objects); a compile-fail fixture.
Note: the lint proves the *absence* of identity checks; it cannot prove the *presence* of a capability lookup, so the earlier clause
"and the difference is expressed as a capability lookup" is struck as unverifiable by the named instrument. That the lookup exists and is
correct is carried by AC-494 and AC-495, which fail if it does not.

**AC-488** — `rk-adapter-claude` · evidence: proposal §9 D10 ("subscription only, never `--bare`; adapter asserts `ANTHROPIC_API_KEY` absent"), §6 crate table · decisions: ADR-0010, ADR-0003
GIVEN `ANTHROPIC_API_KEY` is present in the environment the Claude adapter would dispatch from
WHEN the adapter is constructed at startup
THEN construction fails with a typed configuration error naming the variable, and no dispatch is attempted.
*Verified by:* a unit test with the variable set asserting the typed error and a dispatch count of 0; a `doctor` rule-table row.
ADR-0010 §Consequences makes this an assertion, not a warning: "an API-key-mode fallback is not an allowed degradation path."
(Split from the former compound AC-488; the `--bare` half is now AC-511.)

**AC-511** — `rk-adapter-claude` · evidence: proposal §9 D10 ("never `--bare`"); §2 risk R6 (`--bare` "will become the default for `-p`" — platform surface fetched 2026-09-05 [documented]); §3 principle 5 and §2 R6 mitigation ("asserts from `system/init`"); F7 (an inert flag is indistinguishable from an honoured one on the parent side [measured]) · decisions: ADR-0010, ADR-0003
GIVEN any argument vector the Claude adapter builds for a dispatch
WHEN the adapter lint runs and the test suite's recorded dispatch events are read
THEN `--bare` appears in no argument vector, in source or in any recorded dispatch
AND the session's auth mode is asserted from the child's own `system/init` report to be the subscription session, not inferred from the
argument vector alone — because R6 records that `--bare` is expected to become the `-p` default, at which point "we never pass it" stops
being sufficient.
*Verified by:* a source lint over argument-vector construction; an event-log assertion over the suite's recorded dispatches; a fixture code
path that adds the flag, which must fail the lint naming the call site.
(Split from the former compound AC-488. Uses the id freed by the cut of the old AC-511; see Refutation notes. **Id settled (2026-09-11,
PIA-036, ADR-0092 clause 3):** this register's §0.4 is the owner — AC-511 is the `--bare` half, in the ports-dist area; the design's
"unresolved" notes at :1585/:1969 and its AC-511 placement at :362 are amended toward this register, not the reverse.) *Auth-mode
conjunct (PIA-059):* the `system/init` field that names the session's auth mode is **unnamed in the corpus** — F8 records tool and
server counts only — so the second AND is **[surface unverified]** until the one-dispatch probe in runbook Step 7c's preamble retains a
raw `system/init` payload and names it; if none exists the conjunct is dropped and a named open item with `rk-adapter-claude` as consumer
takes its place.

### 14.2 Capability registry — dated data with a probe

**AC-489** — `rk-adapter-*` capability registry · evidence: proposal §4.1 (the registry shape is `{capability, verified_on, probe}`); shipmates L4.3 (dated `notes` carrying the evidence), L5.7 ("harness capability research has a shelf life of weeks"; Gemini→Antigravity, Windsurf→Devin Desktop [documented]) · decisions: ADR-0003
GIVEN a capability row in an adapter's registry
WHEN the workspace is built
THEN a row missing `capability`, `verified_on` or `probe` fails the build naming the row
AND no capability is readable by the kernel except through a row carrying all three.
*Verified by:* a registry schema test at build time (`serde` deny-unknown + required fields); a fixture registry with a bare
`{capability}` row that must fail the build.

**AC-490** — `rk-cli` (`doctor`) · evidence: shipmates L5.7 (shelf life measured in weeks; #204's table invalidated by its own follow-up comment [documented]) · decisions: ADR-0003
GIVEN a capability row whose `verified_on` is older than the registry's declared maximum claim age
WHEN `redkiln doctor` runs
THEN the row is reported as stale, naming the capability, the date and the probe that would refresh it
AND the report is produced at zero model cost.
*Verified by:* a `doctor` rule-table row; a unit test over a registry fixture with a back-dated row and a frozen clock.
Note: the maximum age is registry-declared config, not a measured constant — "weeks" is `[documented]`, never `[measured]` here.

**AC-491** — `rk-cli` (`doctor --probe`) · evidence: shipmates L5.7 ("a capability claim with no probe behind it is a bug"); F7 (`--agents` inert alone: the claim and the behaviour diverged silently [measured]) · decisions: ADR-0003
GIVEN a capability row asserting a capability is present
WHEN its probe is executed and the probe reports the capability absent
THEN `doctor --probe` exits non-zero naming the capability, the claim date and the observed probe output
AND the row is not silently refreshed to the probe's answer.
*Verified by:* `doctor --probe` integration test against a scripted responder that denies the capability; a replay assertion that no
registry file was rewritten by the probe run.
Note: the proposal names `doctor` and `doctor --rebuild`; the `--probe` subcommand is `[inferred]`, a design consequence of L5.7, not a
decision recorded anywhere yet.

**AC-492** — `rk-adapter-*`, build · evidence: shipmates L4.3 ("unknown scope → bail; unknown capability → bail; nothing degrades quietly") · decisions: ADR-0003
GIVEN an adapter registry naming a capability token or a scope token the canonical vocabulary does not define
WHEN the workspace is built
THEN the build fails naming the unknown token
AND no code path maps an unknown token to a default, an empty set or a no-op.
*Verified by:* a build-time vocabulary test; a fixture registry with `capability = "reads_minds"` that must fail; a lint forbidding a
catch-all match arm in the token mapper.

**AC-493** — `rk-adapter-claude`, `rk-plugin-gen` · evidence: shipmates L4.4 ("Claude Code **inherits every tool when `tools:` is absent**, which makes an empty mapping fail *open*. Their Copilot adapter refuses that case with an error; their Claude Code adapter does not. Ours must." [documented]); F20 (a card's `tools` allowlist that omits `StructuredOutput` silently voids `--json-schema`: `structured_output` absent, exit 0, `subtype: success` [measured]) · decisions: ADR-0003
GIVEN a capability whose adapter mapping resolves to an empty or absent tool list, or a card with no `tools:` key at all
WHEN that mapping or card is loaded
THEN it is rejected as an error naming the capability (or the card) — the pre-spawn effective tool set is the card's declared list and
nothing else, never "inherit everything" (ADR-0108 decision 2; the `doctor` row for a `tools:`-less card, PIA-060)
AND it is never passed on as an empty or absent allowlist.
*Verified by:* a unit test on the tool mapper; a fixture mapping with `tools = []` and a second with `tools` absent, both of which must fail;
a `doctor` rule-table row.
Note on evidence, corrected: the two failure modes point in **opposite** directions and the earlier draft conflated them. L4.4 `[documented]`
is that an *empty or absent* mapping fails **open** — the harness inherits every tool. F20 `[measured]` is that a *populated but incomplete*
allowlist fails **closed and silent** — it voids `--json-schema` at exit 0. F20 did not measure an empty allowlist. This criterion rests on
L4.4; the incomplete-allowlist case is AC-504.

**AC-494** — `rk-kernel`, `rk-types` · evidence: proposal §4.1 (`StepError::Unsupported(capability)`), §4.4 retry table ("`NotAdopted`/`NotIsolated`/`Unsupported`/`Overage`: never retried — configuration, not weather") · decisions: ADR-0003, ADR-0112
GIVEN an activity that needs a capability the selected adapter's registry does not claim
WHEN the kernel resolves the activity
THEN the activity fails with `StepError::Unsupported(capability)` carrying the capability token
AND the failure is not a generic error, not a retry, and never a `pass`.
*Verified by:* a unit test on the resolver with a fake adapter whose registry omits the capability; an exhaustiveness test that
`StepError::Unsupported` carries the token; a negative test asserting the retry policy does not fire on `Unsupported`.

**AC-495** — `rk-kernel` pack validation · evidence: proposal §4.1 ("the pack can route around it"), §4.2 two pack axes · decisions: ADR-0003, ADR-0014
GIVEN a process pack whose only route through a stage requires a capability no configured adapter claims
WHEN the pack is validated at startup
THEN validation fails with a span pointing at the requiring row, before any activity runs
AND a pack that declares an alternative route for that capability validates instead of failing.
*Verified by:* a pack-validation test with spans over two fixture packs (one unroutable, one routed), asserting the span location on the
failing one and clean validation on the routed one.
Note: the earlier draft's second clause also asserted that the alternative branch is *taken at runtime*. That is a different behaviour with a
different instrument, and it is already AC-500's subject; asserting it here made one row cover both startup validation and dispatch-time
routing. Trimmed to the validation-time twin.

**AC-496** — `rk-kernel`, config · evidence: proposal §2 risk table ("Quality on a second harness | E3 is Claude-only"); §8 S-CODEX (registered, budgeted at 46 Codex dispatches, gates Phase 1 = **no**, **unrun**); §9 D7 · decisions: ADR-0007, ADR-0003
**PROVISIONAL** — the *pass bar* rests on S-CODEX, which is registered and unrun; the refusal itself is decided by D7/ADR-0007 ("no
non-Claude adapter in a gate until it passes E3-shaped checks") and is implementable today — the config-time refusal and its `doctor`
row belong in runbook 7b/7d's acceptance sets (PIA-072); only the Codex-present half waits. The Codex adapter is Phase 2 (ADR-0094
decision 2), behind the CI decision ADR-0010 defers, as a named open item with `rk-adapter-codex` as consumer; S-CODEX's METHOD entry
must carry a primary metric on a non-saturated measure before dispatch (PIA-084).
GIVEN a configuration binding a non-Claude harness adapter to an activity that feeds a gate
WHEN the run starts
THEN the run is refused at configuration time naming the adapter and the missing validation record
AND the binding becomes legal only when a recorded E3-shaped validation result for that adapter exists in the pinned corpus repo.
*Verified by:* a startup-validation unit test; a `doctor` rule-table row; a fixture config binding `rk-adapter-codex` to a review
activity that must fail. Corpus pinning per ADR-0013.

**AC-497** — `rk-ports` Toolchain, `rk-types` · evidence: proposal §4.1 (`classify(path) -> Text/Binary/Generated`, `change_summary(range) -> ChangeSummary`); §8 S-BIN (**unrun**, "gates Phase 1: yes (type design)") · decisions: ADR-0008, ADR-0003
**PROVISIONAL** — `ChangeSummary`'s field set is designed against the S-BIN Unreal fixture, which has not run; only the port shape
below is asserted, never the type's contents. Until S-BIN reports, `ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`**
(ADR-0099 clause 5) and Checkpoint 5's grep for its absence covers `crates/` minus that one site; both Phase-1 Toolchain adapters
(`rk-adapter-toolchain-node`, `rk-adapter-toolchain-rust` — ADR-0094) implement `classify` and leave `change_summary`'s body to S-BIN
(ADR-0093 decision 4). S-BIN's METHOD-S §7 amendment (PIA-084): the rule is evaluated per pass, branch 1 needs both passes, a pass
missing a seed routes to branch 2/3 on that pass's prose, both passes reported.
GIVEN a change whose paths `classify` returns as `Binary`
WHEN the review activity's inputs are assembled
THEN the reviewer receives a typed `ChangeSummary` value
AND no textual diff of the binary path is assembled or dispatched.
*Verified by:* a Toolchain fake driven by a binary fixture; a unit test on input assembly asserting the diff assembler is not called
for a `Binary` classification. The contents of `ChangeSummary` are out of scope until S-BIN reports.

### 14.3 Fakes

**AC-498** — kernel test suite · evidence: proposal §4.1 (fake per port; the port table names all five fakes); F8 (a default `claude -p` inherits 12 MCP servers / ~280 tools; MCP was 4,631 tokens = 56% of the cache write [measured]) · decisions: ADR-0003
GIVEN the `rk-kernel` and pack-validation test suites
WHEN they run in CI
THEN they complete with zero model dispatches and zero outbound network calls, using fakes for all five ports
AND a test that reaches a real adapter fails the suite.
*Verified by:* a CI job with network egress denied and no adapter crates linked; a dispatch counter asserted at 0.
Note: the earlier draft added "the F8-style isolation assertion read from the child's own report where any child is spawned at all". With
the dispatch counter asserted at 0 and every port faked, no child is spawned, so that clause was inert here. The child-report assertion is
load-bearing in AC-509 and AC-511, where a child does exist.

**AC-499** — `rk-ports`, `rk-adapter-*` · evidence: proposal §4.1 (the EventStore port exists "for the fake"); shipmates L5.8 (parity, not syntax, is the gap [documented]) · decisions: ADR-0003
GIVEN a port with a fake and a real adapter **in the phase design §10.2 assigns** — Phase 1 instantiates `{fake, real}` for `EventStore`
(SQLite), `Harness` (Claude) and `Toolchain` (Node **and** Rust, two reals); `Tracker` and `DocsSite` have a fake only until Phase 3, and
this criterion does not fail on their absence (ADR-0094 decision 2; PIA-004/PIA-072)
WHEN the shared per-port conformance suite runs
THEN the same assertions execute against both implementations and both pass
AND an assertion that passes on the fake but fails on the real adapter is reported as a conformance divergence naming the assertion.
*Verified by:* one parameterised conformance suite per port, instantiated over `{fake, real}` for each real adapter the phase ships (four
rows in Phase 1: SQLite, Claude, Node, Rust); a CI matrix row per instantiation. The Harness real row runs a real dispatch and therefore
needs the CI decision ADR-0010 defers — until it is taken, that row is a **local-only, named-runner** row and the §14 State line carries the
dependency (PIA-072).

**AC-500** — `rk-ports` Harness fake · evidence: proposal §4.1 (`StepError::Unsupported`, "the pack can route around it"; the Harness fake is the "scripted responder") · decisions: ADR-0003
GIVEN the scripted-responder Harness fake
WHEN a test scripts a capability as absent
THEN the fake produces `StepError::Unsupported(capability)` and the pack's route-around path executes under test
AND the route-around path is covered by at least one test that cannot pass without it.
*Verified by:* an integration test on the fake; a mutation-style check that deleting the route-around branch turns the test red
(the issue-146-147 AC-015 probe pattern from F25: remove step 7 → RED, invert the ordering → 36/36 GREEN; a test that stays green when the
behaviour is removed is not evidence [measured]).

**AC-528** *(added 2026-09-11 — allocated outside this area's exhausted range; see §0.4)* — `rk-ports`, `rk-kernel` · evidence: ADR-0076
("port traits stay sync; async confined to adapters"; `async_fn_in_dyn_trait` is nightly-only through rustc 1.98.1, the pin — R-ASYNC
discharged negative, decision #7 of 2026-09-11) [documented, Rust release notes fetched 2026-09-10]; ADR-0099 decision 1 (the blocking
`dispatch` signature) · decisions: ADR-0076, ADR-0099, ADR-0102
GIVEN the five port traits in `rk-ports` and `rk-kernel`'s normal dependency graph
WHEN the workspace is built and CI runs
THEN no port trait declares an `async fn` or a method returning a `Future` (`impl Future`, `Pin<Box<dyn Future>>`, `BoxFuture`), and a
trait that does fails to compile against the trait-object bound `rk-kernel` holds
AND `cargo tree -p rk-kernel -e normal` names none of `tokio`, `async-std`, `futures`, `async-trait` — directly or transitively.
*Verified by:* a source lint over `rk-ports` (denylist: `async fn`, `Future` in a trait method's return type) with a compile-fail fixture
adding one; a CI assertion on `cargo tree -p rk-kernel -e normal` with a fixture commit adding `tokio` to `rk-ports` that must fail.
*(PIA-029 named ADR-0076's forbids clause as one no criterion covered; S-PORTS runs on 1.98.1.)*

### 14.4 Plugin generation

**AC-501** — `rk-plugin-gen` · evidence: proposal §4.9 ("the generator's output count is the assertion (L5.8)"); shipmates L5.8 ("make the *count* the assertion"; roughly one commit in fourteen exists only to re-sync derived artifacts [documented]) · decisions: proposal §4.9
GIVEN the pack and card sources
WHEN the generator runs
THEN the number of emitted artifacts equals the number computed from the sources, asserted as a number
AND a source entry that emits nothing fails the generator naming the entry.
*Verified by:* a generator count assertion in CI; a fixture source set with one entry the emitter skips, which must fail.
Note: `rk-plugin-gen` is a **Phase 2** deliverable (§7); nothing in this sub-section is checkable before then.

**AC-502** — `rk-plugin-gen` · evidence: shipmates L5.8 (a tool shipped with correct syntax everywhere and silently dropped another tool's card from the homepage; "the gap is convention + parity, not syntax" [documented]); proposal §4.9 (Codex/Pi equivalents from the same source) · decisions: proposal §4.9
GIVEN a source entry that must be emitted into every supported harness dialect
WHEN the generator runs
THEN the per-dialect artifact counts are equal and each source entry is present in every dialect
AND an entry present in one dialect and absent in another fails CI naming the entry and the dialect.
*Verified by:* a per-dialect parity test (set difference asserted empty in both directions), not a total-count check alone.

**AC-503** — `rk-plugin-gen`, CI · evidence: shipmates L5.8 (derived artifacts as a permanent tax [documented]); proposal §4.9 (the plugin "contains **generated** `commands/`, `agents/`, `skills/`") · decisions: proposal §4.9
GIVEN the checked-in generated `commands/`, `agents/`, `skills/` trees
WHEN CI regenerates them from source
THEN the regenerated output is byte-identical to what is checked in
AND any difference fails CI with the diff, before release.
*Verified by:* a CI regeneration-and-diff job; a fixture commit hand-editing one generated file that must fail.

**AC-504** — `rk-plugin-gen` · evidence: F20 (`tools` omitting `StructuredOutput` silently voids `--json-schema`: `structured_output` absent, exit 0, `subtype: success`; adding `StructuredOutput` fixes it [measured]); brief §C5 ("forces `StructuredOutput` in every card whose role demands a schema, checked at configuration time") · decisions: ADR-0016, proposal §4.9
GIVEN a card source whose role declares an output schema
WHEN the generator emits that card
THEN emission fails unless the card's `tools` list contains `StructuredOutput`
AND the failure names the card and the schema.
*Verified by:* a generator unit test; a fixture card with a schema and a `tools` list omitting `StructuredOutput` that must fail
generation. This is the generation-time twin of the kernel's configuration-time refusal of the same defect (owned by the kernel-area
criteria, not by this file); F20 shows the runtime signal is exit 0 with `subtype: success`, so neither check can be inferred from a run.
Note: the earlier draft anchored that twin as "(AC-014)", an id outside this area's range that cannot be resolved from this file — a dead
cross-reference, now described rather than numbered.

**AC-505** — plugin `PreToolUse` hook · evidence: shipmates L1.2 ("a hook can enforce a predicate on its own input, never a position in a sequence"; L1.1: nine days and 6,928 lines proved the alternative [documented]); F2 (settings-based hooks do fire in `-p`, 2 hook events per dispatch on a bare fixture [measured]) · decisions: proposal §4.9 ("the `PreToolUse` frontmatter deny survives as a stateless predicate (L1.2)")
GIVEN the generated `PreToolUse` deny hook
WHEN the hook lint runs
THEN the hook reads only the tool input passed to it, and references no stage, run, session or item state
AND a hook body referencing stage state fails the lint.
*Verified by:* a hook lint over the generated hook source (denylist of stage/run/session/item state accessors); a fixture hook reading a
stage variable that must fail; a differential test that the same tool input yields the same verdict when run at two different stages of a
fixture run. Note: the earlier draft claimed "a unit test that the deny predicate is a pure function of its input" — a unit test cannot
establish purity. The differential test above is what the instrument can actually show; the lint carries the rest.

**AC-506** — `rk-plugin-gen` · evidence: shipmates L3.1 (20+ generically-named skills, including one called `gh`, prefixed in 0.1.15 at a cost of one release, three regressions in the next and six open follow-ups — "free at t=0; ~9 issues at t=30d" [documented]) · decisions: proposal §4.9 ("every installed artifact is namespaced from day one (L3.1)")
GIVEN any artifact the generator emits into a shared user-level or project-level directory
WHEN the generator runs
THEN every emitted name carries the redkiln namespace prefix **`rk-`** — the decided literal (decision #8 of 2026-09-11; the crate prefix
and this register's label prefix) — matching the committed lint regex `^rk-[a-z0-9][a-z0-9-]*$`
AND an unprefixed name fails generation, so an unnamespaced artifact can never be installed even once.
*Verified by:* a name lint in the generator (the regex above on every emitted path and every `name:` field); a fixture source named
`polish` that must fail generation; a fixture named `redkiln-polish` that must also fail (one prefix, not two). *(Amended 2026-09-11,
PIA-106: the prefix was undecided in the design and DECIDED here; `rk-plugin-gen` is Phase 2, and AC-531 carries the same regex for
`init`.)*

### 14.5 Projections

**AC-507** — `rk-adapter-github` · evidence: proposal §4.8 (`rk:stage/*`, `rk:gate/*` orchestrator-only; `rk:kind/*` is the human-input namespace); F19 (drift 12/174 = 7%; ≤1 h 3/95 = 3% vs >1 h 9/79 = 11%, RR 3.6 with CI [1.0, 12.9], Fisher p = 0.039 — directional, marginal at n=174 [measured]) · decisions: ADR-0003
GIVEN a human changes an orchestrator-only label (`rk:stage/*` or `rk:gate/*`) on a tracked item
WHEN the next `reconcile()` runs
THEN a typed `Drift` record is produced and surfaced, and no state transition occurs from the label change
AND item state remains what the event log says it is.
*Verified by:* an integration test against the in-memory board fake with a mutated label; a replay assertion that no transition event
followed the drift event.
Note on the limit of F19: the brief records that in H2/H3 "the FSM was never replayed, so 'same next action' is unproven". F19 licenses the
drift *rate* and the claim that drift is rare and cheap to catch; it does not license any claim about replay equivalence, and none is made here.

**AC-508** — `rk-adapter-github` · evidence: proposal §4.8 ("idempotency key on every write"), §4.4 (activities carry an idempotency key `(run_id, activity_id, attempt)`); shipmates L5.4 (issue #267's thread holds the same "Ship Issue — Done" report posted three times by their own orchestrator [documented]) · decisions: ADR-0003
GIVEN a projection write that has already been applied
WHEN the same projection is replayed
THEN no second external write is issued, and the idempotency key on the recorded event matches
AND a write path with no idempotency key fails the adapter's conformance suite.
*Verified by:* the shared Tracker conformance suite (AC-499) run against the in-memory board fake: replay produces a write count of
exactly 1; a lint asserting every write helper takes a key argument.

**AC-509** — `rk-adapter-claude`, dispatch · evidence: F8 (a default `claude -p` inherits the operator's whole environment: 12 MCP servers, ~280 tools; MCP was 4,631 tokens = 56% of the cache write [measured]); brief §C11 (isolation must be verified from the child's own `system/init` because an ignored flag looks identical to an honoured one); proposal §4.8 ("agents hold **no** tracker token") · decisions: ADR-0003
GIVEN the orchestrator holds a tracker credential
WHEN a child process is spawned for any activity
THEN the child's environment contains no tracker credential
AND the assertion is made from the child's own reported environment, not from the parent's spawn arguments.
*Verified by:* an integration test spawning a fixture child that echoes its environment; a `doctor` rule-table row.

**AC-510** — `rk-kernel`, `rk-adapter-mdbook` · evidence: proposal §4.8 ("generator-agnostic invariants (build, links, drift-on-source-change) in the kernel, generator checks (`mdbook test`) in the adapter"); §4.1 port table (the DocsSite fake is the "no-op recorder") · decisions: ADR-0003
GIVEN the three DocsSite invariants (build succeeds, links resolve, drift is raised when a source changes)
WHEN the kernel test suite runs against the no-op recorder fake
THEN each of the three invariants is exercised and passes without any generator being installed
AND a recorder scripted to fail each invariant in turn turns exactly the corresponding kernel assertion red.
*Verified by:* the kernel DocsSite suite bound to the fake, with a negative case per invariant; a CI job with no mdBook binary present that
must still pass. Note: the negative cases are what make this non-vacuous — a recorder that answers yes to everything satisfies three
"invariants pass" assertions on a kernel that checks nothing. The earlier draft's closing clause ("an invariant that cannot be expressed
against the fake is reported as belonging to the adapter") is struck: "cannot be expressed" is a judgement with no instrument, and the
kernel/adapter split it gestures at is settled by §4.8, not discovered by a test.

### 14.6 Distribution and lifecycle

> **CONDITIONAL — read standing caveat 2 first.** Proposal §4.9 decides a plain git clone plus a `cargo-dist` binary shim. It decides no
> installer, no embedded payload, no receipts, no rename table, no ownership states and no integrity scan. Every criterion in this
> sub-section is conditional on a distribution mechanism with an install-and-repair step being decided; until then their `decisions:` field
> names the shipmates lesson only, never §4.9, and none is a commitment the design has made. Three further criteria that *contradicted*
> §4.9 (embed-over-cwd precedence, its warning count, and the `build.rs` embed allowlist) are cut — see Refutation notes.

**AC-514** — installer, `doctor --fix` · **CONDITIONAL** · evidence: shipmates L3.3 (#373: "a rename that only changes `name:` and hopes install is enough will leave two skills on disk"; a committed old→new table applied on both `install` and `doctor --fix`, before the new payload is written, ordered so the new path is valid on disk before the old is removed [documented]) · decisions: none — §4.9 decides no rename mechanism
GIVEN a checked-in old→new rename table and an installed tree carrying the old names
WHEN `install` or `doctor --fix` runs
THEN the table is applied before the new payload is written, and the new path is valid on disk before the old one is removed
AND an interruption between the two steps leaves the new path present rather than neither.
*Verified by:* an upgrade e2e test over a fixture rename asserting write-then-delete ordering; a fault-injection test killing the process
between write and delete, asserting the new path exists. (Split from the former compound AC-514; receipt rewriting is now AC-512.)

**AC-512** — installer receipts · **CONDITIONAL** · evidence: shipmates L3.3 ("rewriting every receipt that listed the old path *including sibling targets sharing a directory*" [documented]) · decisions: none — §4.9 decides no receipt mechanism
GIVEN a rename applied from the checked-in table
WHEN the installed tree's receipts are read afterwards
THEN no receipt references the old path, and every receipt that listed the old path — including receipts for sibling targets sharing a
directory — lists the new path
AND a fixture whose sibling receipt is left un-rewritten fails the test naming the receipt.
*Verified by:* a receipt-content assertion in the upgrade e2e over a fixture rename with a sibling in the same directory.
(Split from the former compound AC-514. Uses the id freed by the cut of the old AC-512; see Refutation notes.)

**AC-515** — installer · **CONDITIONAL** · evidence: shipmates L3.3 ("**no live stubs** — a stub skill is still a live name in the picker, which is the collision you were trying to kill"; "the CLI break is a tree that still has polish and shipmates-polish. The second one is worse." [documented]) · decisions: none — §4.9 decides no rename mechanism
GIVEN a completed rename over a tree that carried the old names
WHEN the installed tree is inventoried
THEN every new name is present on disk, and zero artifacts carrying the old name remain — including stubs, aliases and forwarding entries
AND the old-name count is asserted at 0, not sampled.
*Verified by:* a post-upgrade inventory assertion in the upgrade e2e, asserting both the new-name presence and the old-name zero; a fixture
that ships a stub under the old name and must fail. Note: the new-name presence clause is not decoration — without it the criterion passes
on an empty tree, and on a system that installs nothing at all.

**AC-516** — installer receipts, `doctor` · **CONDITIONAL** · evidence: shipmates L3.2 (#386 — "doctor --fix cannot claim an unmanaged flagship (report-bug left stale on 0.1.15 upgrade)"; three ownership states own / adopt / refuse, because failing closed on "ours but unclaimed" strands users on stale content forever [documented]) · decisions: none — §4.9 decides no receipt or ownership mechanism
GIVEN a file at a payload path that no receipt claims
WHEN `doctor --fix` runs
THEN the file is backed up, rewritten from the payload and claimed by a receipt carrying its SHA-256
AND a file the product does not own is refused instead, naming the flag that would force it, and third-party artifacts beside ours
are left untouched.
*Verified by:* three e2e cases (owned / ours-but-unclaimed / foreign) asserting the resulting receipt state and on-disk content;
a per-file SHA-256 assertion in the receipt schema test.

**AC-517** — installer integrity scan · **CONDITIONAL** · evidence: shipmates L3.5 (0.1.16 bounded the unmanaged-file scan to the payload's own subtrees, skips symlinks outright and never walks `node_modules`; "install --harness opencode aborts on node_modules/.bin symlinks (v0.1.15 upgrade stuck)" [documented]) · decisions: none — §4.9 decides no integrity scan
GIVEN a target directory containing a large symlink farm outside the payload subtrees
WHEN the integrity scan runs
THEN the scan visits only paths under the payload's own subtrees, follows no symlink, and completes
AND a scan that walks outside the claimed subtrees fails the test with the escaping path.
*Verified by:* an e2e test against a fixture tree with `node_modules/.bin` symlinks and a symlink loop, asserting completion and a
bounded visited-path set.

**AC-518** — `rk-kernel`, `rk-git` · evidence: shipmates L5.3 (0.1.18 committed into the user's own checkout; 0.1.19 pinned the write to a dedicated worktree — "the orchestrator never writes to the user's checkout. Ever." [documented]); F19 (drift 12/174 = 7% [measured]) · decisions: ADR-0018, ADR-0003
GIVEN a run whose working directory is the user's checkout
WHEN any orchestrator write is attempted — commit, file write, index change
THEN the write is refused unless its target is a run-owned worktree or the store under the resolved common dir
AND the refusal names the path and the run's branch.
*Verified by:* an integration test asserting the user checkout is unchanged across a full lane run — `git status --porcelain` empty,
`git diff-index --quiet HEAD` clean, and HEAD unmoved — plus a write-path lint asserting every mutating call resolves through the worktree
resolver. Note: the earlier draft's "index mtime unchanged" is struck as an unsound instrument — git refreshes the index stat cache during
ordinary read operations, so the mtime moves without any orchestrator write and the assertion would be flaky in the false-positive direction.
Note on evidence: F19's 7% is `[measured]`; the inference that a low drift rate makes an unexpected working-tree write *expensive to detect*
is `[inferred]`, and the earlier draft tagged it `[measured]`.

**AC-519** — release pipeline · **partly CONDITIONAL** · evidence: shipmates L3.8 (a CLI e2e job added at 0.1.5 after **five blind releases**; every escape since — #384, #386 — was an upgrade-path escape found by the owner upgrading, not by a test; "the upgrade path is simultaneously the least-tested and most-broken surface in this class of product" [documented]) · decisions: proposal §4.9 for the namespacing and generated-artifact-count assertions only; the receipt and rename-table assertions are conditional per standing caveat 2 · ADR-0013 for fixture pinning
GIVEN a release candidate
WHEN the release pipeline runs
THEN an end-to-end job installs the previous published version, upgrades to the candidate, and asserts namespacing and generated-artifact
counts on the upgraded tree — plus receipts and rename-table application if and only if those mechanisms are adopted
AND the release is refused if that job is absent, skipped or red.
*Verified by:* a required CI check wired to the release gate; a fixture run against a deliberately broken input that must block the release.
Fixtures pinned by SHA from the corpus repo (ADR-0013).

---

#### Measures — cross-reference only (bodies live in the companion register)

| id | measure | note carried from the area file |
| --- | --- | --- |
| SM-145 | Derived-artifact re-sync tax | no baseline |
| SM-146 | Upgrade-path escapes | no baseline |
| SM-147 | Capability-claim age at the moment of use | no baseline |
| SM-148 | `Unsupported` incidence and route-around share | saturating; no baseline |
| SM-149 | Install repair incidence | CONDITIONAL on a receipt mechanism being adopted (standing caveat 2); no baseline; conditional |
| SM-151 | Fake/real conformance divergence | saturating; no baseline |
| SM-153 | Probe cost and wall time | no baseline |
| SM-154 | Second-harness validation spend | saturating |

#### Residual risk this area could not repair

- The whole surviving distribution block (AC-512, AC-514..AC-517, AC-519's receipt/rename halves, SM-149) is UNBUILDABLE until §4.9 is reopened and a distribution mechanism is actually decided. A reader who skims past the CONDITIONAL banner will read them as requirements. The cleaner outcome would have been to cut all six and raise 'decide the distribution mechanism' as a gating open question.
- Every 'Verified by' in this area names an instrument that does not exist — no crate in the workspace beyond `spikes/`. Only AC-500 adopts F25's mutation probe, so 38 of 39 rows are formally exposed to the defect the programme's own root-cause finding identified.
- All plugin/distribution evidence is single-source, single-project, single-maintainer, unreplicated — and it is a SECONDARY retelling. Every quotation was verified against the lessons document, but the shipmates issue numbers (#385, #373, #386, #231, #384, #204, #267) could not be checked against any primary issue.
- AC-499's conformance matrix runs the shared suite against the REAL Claude adapter, implying model dispatches in CI at unbudgeted cost, and sits in tension with AC-498's network-denied isolation job. Nothing bounds that cost and no measure tracks it; since 2026-09-11 the Harness real row is a local-only, named-runner row until the CI decision ADR-0010 defers is taken.
- ADR-0094's Rust Toolchain adapter has no behaviour evidence of any kind — the fake-conformance row and AC-328/AC-329 are what will produce it — and the self-hosting run (AC-473/AC-521) exercises ADR-0073's mixed-toolchain expansion on this repository's own Python under `spikes/`.
- The area has essentially no discriminating measure before Phase 2: SM-145/146/147/149/151 have no baseline, SM-148 and SM-151 are explicitly saturated, and SM-153/154 are the only two with measured priors.
- AC-502's parity assertion has an undefined domain — the set of 'supported harness dialects' is nowhere enumerated; it survives on L5.8's `[documented]` parity failure, not on a decision.
- Phasing means roughly half the area is unexercisable during Phase 1: `rk-plugin-gen` (AC-501..AC-506) is Phase 2, the GitHub and mdBook adapters (AC-507, AC-508, AC-510) are Phase 3, the Codex adapter (AC-496) is Phase 2 behind the CI decision ADR-0010 defers (ADR-0094 decision 2 — this line said Phase 4 until 2026-09-11). The Phase-1 subset is stated in this section's phase note.

---

## 15. Traceability

These two tables are the point of the document. They are built **mechanically from what the criteria actually cite**, not from what ought to be there.

Read them with one caveat. The refutation pass cut many citations and **named each cut inside the criterion that carried it**. A cut citation therefore still appears as text in that criterion. The F-table below separates the two: the *rests on* column is taken from the criterion's `evidence:` line, which is the live attribution; the *named in notes* column is taken from the body, where the name usually appears **because the citation was cut, corrected or scoped**. A criterion in the second column is not evidence for that finding — go and read what the criterion says about it.

### 15.1 Evidence to criteria

| finding | what it measured | rests on it (evidence line) | named in notes (usually a cut or a scope limit) |
| --- | --- | --- | --- |
| **F1** | headroom readable only via the top-level `rate_limit_event`; `result.rate_limit` null; **no family sub-limit key** | AC-030, AC-200, AC-201, AC-219, AC-220, AC-221, AC-226, AC-227, AC-360, AC-374, AC-375, AC-376, AC-380 | AC-378, AC-404 |
| **F2** | settings-based hooks do fire under `-p` (2 hook events per dispatch) | AC-505 | — |
| **F3** | per-unit cost floor ($0.021–0.027 trivial dispatch); `usage` carries no cache-TTL split | AC-360, AC-362 | — |
| **F4** | `system/thinking_tokens`, 4–6 per dispatch — a second undocumented event shape | AC-200, AC-201, AC-226, AC-227, AC-391 | — |
| **F5** | cache-read share 0.584 / 0.698 / 1.000 / 0.722 on an identical prompt at n=2 — explicitly not a result | AC-381, AC-382 | — |
| **F6** | `utilization` quantised to **0.01**; five_hour and seven_day proven independent; per-run cost unmeasurable | AC-030, AC-164, AC-220, AC-360, AC-372, AC-373, AC-374, AC-375, AC-376, AC-377, AC-381, AC-389, AC-393 | AC-098, AC-404 |
| **F7** | `--agents` is inert without `--agent`; the card's model + effort are accepted | AC-027, AC-036, AC-118, AC-203, AC-204, AC-212, AC-234, AC-238, AC-431, AC-491, AC-511 | AC-386, AC-486 |
| **F8** | a default `claude -p` inherits 12 MCP servers / ~280 tools; MCP = 4,631 tokens = 56% of the cache write | AC-027, AC-041, AC-195, AC-202, AC-204, AC-234, AC-238, AC-386, AC-399, AC-498, AC-509 | AC-091, AC-194, AC-486 |
| **F9** | **no F9 entry exists** — struck from the ledger as cited-but-unsupported | — *(no criterion rests on it)* | AC-076 |
| **F10** | prompt-cache TTL ~1 h; full hit at 45.1 min, 0.690 at 61.6 min; boundary bracketed, never located (n=1 per gap) | AC-210, AC-243, AC-384 | AC-423 |
| **F11** | S0.1 mutants: 9 of 10 test-reachable killed, 1 survived (90.0%, Wilson 95% [59.6%, 98.2%]); median mutant run 323 s; **S0.1-SCOPE: 8 of 23 items unmeasured by construction** — their fixes touch only `workflows/`, outside the affected-area gate (PIA-076) | AC-289, AC-340 | — |
| **F12** | S0.2 trailer corpus: 360 commits / 74 slices; ever-rejected 15/74 = 20.3%; sealed more than once 21.6% | AC-162, AC-165, AC-184, AC-371, AC-427, AC-444, AC-483 | AC-364, AC-412 |
| **F13** | prefix ~17,439 shared + ~7,827 per-directory; identical across 19/19 fresh repos | AC-228, AC-388, AC-423 | AC-314, AC-370, AC-385 |
| **F14** | 40 dispatches / ~1.0M tokens (854k cache-read, 157k cache-creation; "~1.9M" was a miscount — PIA-073) moved `seven_day` 0.100 → 0.100 — the E0 cache-cost gate was STRUCK, not answered | AC-372, AC-381 | — |
| **F15** | reasoning effort is applied per dispatch (thinking tokens 0.0 / 4.8 / 221.0); **low vs high did NOT separate** | AC-036, AC-212, AC-236, AC-418 | — |
| **F16** | cache-read share: no-commit 1.000, commit-each 0.689, +`--exclude-dynamic` 0.846, +`--system-prompt` 1.000, worktree-each 0.685 | AC-091, AC-209, AC-383, AC-387, AC-423 | — |
| **F17** | the E3 reviewer restored the fix from the object store (42 turns, 22,853 output, 526 s, returned `approved`) — an instrument defect | AC-191, AC-291, AC-292, AC-293, AC-338, AC-339 | AC-192, AC-194, AC-386 |
| **F18** | gate→verdict n=174, median 10.5 min, 45% >1 h; stage→verdict n=191, median 51.6 min, p90 ~9 h, max 226 h (9.4 days), 48% >1 h; one operator | AC-039, AC-074, AC-126, AC-172, AC-176, AC-243, AC-252, AC-256, AC-261, AC-306, AC-311, AC-319, AC-359, AC-384, AC-402, AC-403, AC-408 | AC-276, AC-405, AC-476 |
| **F19** | suspension lossless 174/174; drift 12/174 = 7% (3/95 ≤1 h vs 9/79 >1 h); **the FSM was never replayed** | AC-006, AC-007, AC-013, AC-042, AC-067, AC-126, AC-172, AC-173, AC-176, AC-183, AC-188, AC-245, AC-249, AC-252, AC-254, AC-258, AC-261, AC-270, AC-277, AC-306, AC-321, AC-354, AC-359, AC-365, AC-398, AC-402, AC-406, AC-427, AC-483, AC-507, AC-518 | AC-124, AC-161, AC-407 |
| **F20** | a card `tools` allowlist omitting `StructuredOutput` silently voids `--json-schema`: `structured_output` absent, exit 0, `subtype: success` | AC-027, AC-031, AC-095, AC-142, AC-205, AC-206, AC-208, AC-230, AC-234, AC-235, AC-268, AC-269, AC-282, AC-318, AC-324, AC-343, AC-386, AC-415, AC-417, AC-431, AC-439, AC-493, AC-504 | AC-129, AC-199, AC-218, AC-385 |
| **F21** | E3 arm (b): S-SILENT 23/23, CLEAN FP 2/23, MCC +0.917, $56.47 | AC-213, AC-223, AC-373 | AC-097, AC-438 |
| **F22** | ~12 background kills against one 12,000 s detached survivor; `issue-81` killed 3× as background, 116 s in foreground; cause unestablished | AC-024, AC-030, AC-035, AC-193, AC-198, AC-222, AC-224, AC-240, AC-242, AC-257, AC-278, AC-331, AC-353, AC-379, AC-392, AC-403, AC-404 | AC-181, AC-190, AC-215 |
| **F23** | E3 detection 23/23 vs 23/23, McNemar p=0.625 — **saturated, no discriminating power** | AC-315 | AC-283 |
| **F24** | adjudication at zero model cost: 6/6 E3 rejections factually true; CLEAN ground truth wrong for ≥5/23 | AC-308, AC-309, AC-357, AC-395, AC-414 | — |
| **F25** | AC-015 probe: remove step 7 gives RED; **invert the ordering gives 36/36 GREEN** | AC-140, AC-284, AC-317, AC-333, AC-334, AC-346, AC-395, AC-414, AC-446, AC-457, AC-458 | AC-261, AC-500 |
| **F26** | S-SUBTLE 5/5 sighted and 5/5 blind, twice; E3-H 20/20 across two harder tiers | AC-192, AC-312 | — |
| **F27** | verdict flaky on 2/4 items over 21 draws; detection flaky on 0/7 | AC-050, AC-065, AC-285, AC-286 | AC-269 |
| **F28** | H1 correction: 18,603 created at both 65.2 min and 49.7 h under the default prompt (shared-prefix subsidy, not durability); `--system-prompt` 29,680 at 66.8 min; H4 Jaccard FULL 1.000 = CONTROL 1.000 | AC-037, AC-211, AC-242, AC-243, AC-267, AC-273, AC-279, AC-359, AC-383, AC-384, AC-396, AC-408, AC-423 | AC-367, AC-405, AC-476 |
| **F29** | S-VAR, 60 draws, 0 void, $176.96: a reproduced regression approved on 3 of 10 draws; class means 0.70 / 0.37 / 0.10 / 0.00; 19/19 rejections on-target; 0.0098 five_hour per draw | AC-032, AC-115, AC-133, AC-231, AC-251, AC-283, AC-287, AC-295, AC-312, AC-313, AC-316, AC-342, AC-373, AC-393 | AC-236, AC-307 |
| **F30** | **unassigned ledger slot** — S-H1b and S-WT were never written up; their numbers exist only in the design recommendation and the proposal | AC-267 | AC-383 |
| **F31** | S-TYPED: schema conformance 52/52, `repro.command` 147/147, CLEAN blocks 12/23 strict / 9/23 lenient, 0/23 verified FP, adjudication 14/14, $112.25 vs $64.61 = +74%, median $3.03 / 1,071 s, slowest 4,561 s; four defects survived 1,890/1,890 and 1,907/1,907 green | AC-025, AC-031, AC-038, AC-055, AC-115, AC-134, AC-138, AC-139, AC-194, AC-197, AC-199, AC-223, AC-280, AC-281, AC-284, AC-287, AC-288, AC-289, AC-290, AC-295, AC-298, AC-300, AC-307, AC-308, AC-309, AC-310, AC-311, AC-315, AC-316, AC-329, AC-335, AC-342, AC-357, AC-395, AC-458 | AC-121, AC-235, AC-236, AC-285, AC-296, AC-340 |
| **F32** | P9 authoring: lint 13 real / 34 false; median 56.5 non-blank lines; 4.7 first-run errors per file | AC-106, AC-156, AC-414, AC-415 | AC-409, AC-430 |
| **F33** | P9b: the lint accepted 5 guesses silently across 27 ambiguities (20 new); median 73 lines (+29%); 1.3 errors/file | AC-106, AC-107, AC-151, AC-156, AC-410, AC-420 | AC-409 |
| **F34** | P6 authoring: 90 activities, median 53 lines, sum 5,059; 42/90 carried a quality block; 0.11 errors/file (lint read first) | AC-106, AC-112, AC-156, AC-395, AC-414 | AC-430 |
| **F35** | P6 lattice, 102 activities / 12 packs, 27 s: unsatisfiability 23.5% → 11.4% → 0.3%; dead activities 20.8% (8 packs) → 14.9% (12) → 5.9% = 6/102; rank ties 0; hard cycles 0 | AC-040, AC-046, AC-047, AC-048, AC-054, AC-056, AC-057, AC-059, AC-100, AC-101, AC-102, AC-103, AC-414, AC-421, AC-424, AC-426, AC-430 | AC-049, AC-052, AC-094, AC-104, AC-433 |
| **F36** | P1 arm L, 400 dispatches, 0 void, $21.37: modal `R` identical on 5/20, 3/20, 2/20, 2/20 against an 80% line — 0.25/0.15/0.10/0.10; alpha drift −0.168 on byte-identical anchors; `external_dependency` 200/200 false; arm D precision/recall not computed **at F36** (scored later by F39) | AC-009, AC-044, AC-069, AC-070, AC-071, AC-073, AC-076, AC-113, AC-130, AC-148, AC-251, AC-302, AC-347, AC-348, AC-425, AC-432, AC-434, AC-436 | AC-089 |
| **F37** | P7, 61 runs / 297 records / 0 void / $131.62: deviation offered 71 times, used 0; graph cost $1.02 / $2.69 / $9.86 (n=1); primary criterion powerless at n=3 | AC-061, AC-063, AC-105, AC-139, AC-251, AC-303, AC-314, AC-396 | — |
| **F38** | P8 PARTIAL, 3 of a pre-registered 27+3: mid-graph resume across a real `taskkill /T /F` — 0 changed, 0 lost, digests identical; the resumed steps' output tokens were 8,408 / 10,314 / 14,579 and their `cache_creation_input_tokens` 84,960 / 49,468 / 46,961 — the three sub-unity ratios (0.45 / 0.55 / 0.78) divided the former by F28's cache-creation ~18,603 and is withdrawn; cost ordering UNMEASURED (E-0087-1, ADR-0106) | AC-013, AC-039, AC-161, AC-185, AC-186, AC-187, AC-241, AC-265, AC-266, AC-270, AC-273, AC-353, AC-401 | AC-246, AC-271, AC-400 |
| **F39** | P1 arm D scored 2026-09-09 over 140 truth cells at pin `47faec9f` (136 reached on the first run; all 140 on the fixed `score-arm-d-panel.py` re-run, E-0089-2), METHOD-P amendment 9, panel-adjudicated (never human-panel), $0: union-source precision `ui` 1.00 [0.65, 1.00] / `toolchain` 0.73 [0.48, 0.89] / `change` 0.72 [0.54, 0.85] / `binary_change` 0.60 [0.23, 0.88] / `flag_exists` 0.00 [0.00, 0.66] / `auth` 0.00 [0.00, 0.26] over 11 positives all false (0/12 [0.00, 0.24] on the fixed re-run) / `data_migration` **not measurable** (0 positives on 20 items); **no facet clears 0.95 and the bar is unreachable at n=20**; 4 items changed no code and produced 16 confident booleans over 16 panel `unknown`s; 32 of 140 cells human-adjudicated, 15 of 32 differ from the script | AC-070, AC-071, AC-072, AC-076, AC-079, AC-113, AC-520 | AC-069, AC-148, AC-302, AC-347 |
| **F40** | P2's **perturbation** half scored 2026-09-10 at pin `47faec9f` over 41 `(case, downstream)` rows / 12 cases, labels blind and **timestamp-verified** (`labelled_at` 15:58:08Z precedes the engine's earliest record at 16:01:12Z), $0: **0 of 41 rows are "should re-run"**, so recall is **not computable** and METHOD-P §2.4's criterion 1-recall and criterion 2 are **NOT ANSWERABLE**; coarse specificity 0.73 [0.58, 0.84] against fine 0.55 [0.38, 0.70]; precision 0.00 [0.00, 0.26] coarse over 11 emitted positives — a property of the label set, not a reading of the engine; fine abstains on 8 of 41 = 19.5% [0.10, 0.34], never counted as a miss; 10 of the 33 paired rows flip `Present` → `Stale` coarse → fine and none flips back; 11 of 12 cases change YAML frontmatter only (`stage`, `status`, `updated`, `gate_open`); criterion 1's precision half fires and its prescribed remedy is **contraindicated**; the **advisory half (A01–A06) is unrun** | AC-068 | AC-113, AC-434 |

**Five readings of that table worth carrying forward.**

- **F31 (S-TYPED) and F19 carry the most weight** — 35 and 31 criteria respectively rest on their evidence lines. Both are one corpus, one operator; F19's own recorded limit is that *the FSM was never replayed*, and F31's CLEAN ground truth is wrong for at least 57% of items. Every rate quoted against either is an upper bound.
- **F9 appears only as a struck citation** (in AC-076, which had cited it as precedent). There is no F9 entry; that row is the table working as intended.
- **F30 is an unassigned ledger slot.** S-H1b and S-WT were never written up, so AC-267 and AC-383 rest on numbers that live only in the design recommendation and the proposal. That is also the root of the contested worktree cost cell (§1.2 row 15).
- **F39 is the first row here read from the corpus rather than from the condensed brief**, which predates it and cannot carry it. That is the discipline §16's closing note argues for: a pass that checks only against a condensation cuts real citations as fabrications. Anything F39 touches — AC-070, AC-071, AC-072, AC-076, AC-079, AC-113, AC-520 — must be re-refuted against `spikes/F39-DRAFT.md` and ADR-0089, not against the brief.
- **F40 is the second such row, and what it adds is a reading about this programme's own pre-registrations.** P2 §2.4's criteria 1 and 2 join P1's α rule (F36), P7's criterion 1 (F37) and METHOD-P §1.4's 0.95 bar (F39) as **four registered thresholds their own registered samples cannot decide** — each fixed a bar without checking that the registered corpus could produce the denominator the bar needs. Read the four together before writing a fifth. AC-068 is the only criterion resting on F40's evidence line, and it must be re-refuted against `spikes/F40-DRAFT.md`, not against the brief, which predates it.

### 15.2 Decision to criteria

| decision | title (frontmatter `title`, verbatim) | criteria that implement it | n | note (the 2026-09-08 gloss, kept for continuity) |
| --- | --- | --- | ---: | --- |
| **ADR-0002** | Clean break from old redkiln; self-hosting is the cutover switch | AC-440, AC-441, AC-442, AC-443, AC-444, AC-445, AC-446, AC-447, AC-448, AC-449, AC-450, AC-451, AC-461, AC-466, AC-469, AC-473, AC-474, AC-476, AC-521, AC-522 | 20 | Clean break; self-hosting is the switch |
| **ADR-0003** | Five named ports with fakes; git excluded; no Postgres adapter | AC-162, AC-164, AC-174, AC-181, AC-182, AC-189, AC-191, AC-195, AC-199, AC-200, AC-201, AC-202, AC-203, AC-204, AC-212, AC-213, AC-214, AC-215, AC-218, AC-219, AC-220, AC-221, AC-222, AC-223, AC-224, AC-225, AC-226, AC-227, AC-228, AC-229, AC-232, AC-233, AC-234, AC-236, AC-237, AC-238, AC-360, AC-362, AC-363, AC-364, AC-365, AC-366, AC-367, AC-368, AC-369, AC-370, AC-371, AC-378, AC-385, AC-386, AC-389, AC-392, AC-393, AC-394, AC-395, AC-397, AC-398, AC-399, AC-413, AC-427, AC-480, AC-481, AC-482, AC-483, AC-484, AC-485, AC-486, AC-487, AC-488, AC-489, AC-490, AC-491, AC-492, AC-493, AC-494, AC-495, AC-496, AC-497, AC-498, AC-499, AC-500, AC-507, AC-508, AC-509, AC-510, AC-511, AC-518 | 87 | Five named ports with fakes; git excluded; no Postgres adapter |
| **ADR-0005** | Human edits are prose-free and structure-gated | AC-120, AC-124, AC-126, AC-149, AC-150, AC-151, AC-152, AC-153, AC-155, AC-156, AC-411, AC-453, AC-454, AC-471, AC-478 | 15 | Human edits live in prose fields, never in rendered structure |
| **ADR-0006** | Artifact layout C — split fields, opaque prose, rendered structure | AC-031, AC-037, AC-120, AC-121, AC-122, AC-123, AC-124, AC-125, AC-127, AC-128, AC-129, AC-130, AC-131, AC-132, AC-133, AC-134, AC-135, AC-136, AC-138, AC-141, AC-142, AC-143, AC-144, AC-145, AC-146, AC-147, AC-148, AC-149, AC-151, AC-152, AC-153, AC-154, AC-155, AC-156, AC-157, AC-158, AC-159, AC-411, AC-426, AC-449, AC-450, AC-451, AC-452, AC-453, AC-454, AC-455, AC-456, AC-457, AC-459, AC-460, AC-462, AC-468, AC-470, AC-471, AC-472, AC-477, AC-478, AC-479 | 58 | Typed artifacts, layout C; a schema change is a migration |
| **ADR-0007** | Codex CLI is a second harness for validation only, not a gate-eligible adapter | AC-238, AC-239, AC-496 | 3 | Codex is a validation-only second harness |
| **ADR-0008** | A real Unreal project is the S-BIN fixture in Phase 0; the toolchain adapter ships in Phase 2 | AC-154, AC-356, AC-475, AC-497 | 4 | A real UE project is the S-BIN fixture; Unreal adapter in Phase 2 |
| **ADR-0009** | Phase 1 human gates answer from the CLI only; tracker and web channels wait for Phase 3 | AC-039, AC-153, AC-261, AC-262, AC-263, AC-269, AC-274, AC-405, AC-476 | 9 | Phase 1 human gates answer from the CLI only |
| **ADR-0010** | The Claude adapter authenticates by subscription only, never --bare, and asserts no API key | AC-027, AC-216, AC-217, AC-422, AC-488, AC-496, AC-511 | 7 | Subscription only; never `--bare`; no API-key fallback |
| **ADR-0011** | Phase 1 ships fix and story lanes only | AC-405, AC-473, AC-521 | 3 | CLI shape and the Phase 1 exit criterion |
| **ADR-0012** | Open questions become named, gating spikes, not prose | AC-006, AC-009, AC-148, AC-301, AC-435, AC-448, AC-462, AC-463, AC-467, AC-475, AC-479, AC-520 | 12 | Unknowns are carried as named spikes |
| **ADR-0013** | Corpus lives in its own pinned repo, not under spikes/ | AC-006, AC-161, AC-184, AC-312, AC-314, AC-427, AC-463, AC-464, AC-465, AC-519, AC-522 | 11 | The corpus lives in its own pinned repo |
| **ADR-0014** | Process packs own structure; technology packs bind and contribute at slots | AC-005, AC-036, AC-037, AC-080, AC-081, AC-083, AC-084, AC-085, AC-086, AC-087, AC-089, AC-090, AC-091, AC-092, AC-093, AC-094, AC-096, AC-097, AC-098, AC-099, AC-118, AC-327, AC-337, AC-349, AC-350, AC-414, AC-415, AC-416, AC-418, AC-419, AC-420, AC-421, AC-430, AC-431, AC-495 | 35 | Process packs own structure; technology packs bind |
| **ADR-0015** | Review tier selects execution-grade or read-only, not a uniform policy | AC-038, AC-287, AC-295, AC-298, AC-299, AC-300, AC-301, AC-302 | 8 | Review tiering - the stage weight selects the tier |
| **ADR-0016** | Static system prompt carries the card for Phase 1 | AC-027, AC-031, AC-036, AC-091, AC-204, AC-206, AC-208, AC-209, AC-210, AC-211, AC-235, AC-381, AC-382, AC-383, AC-384, AC-385, AC-387, AC-388, AC-423, AC-431, AC-439, AC-504, AC-526 | 23 | A static `--system-prompt` carries the card for Phase 1 |
| **ADR-0017** | Gate record binds a single-use token to the tree hash, with a deadline and terminal path | AC-024, AC-030, AC-035, AC-039, AC-126, AC-176, AC-177, AC-240, AC-241, AC-242, AC-243, AC-244, AC-245, AC-246, AC-247, AC-248, AC-249, AC-250, AC-251, AC-252, AC-253, AC-254, AC-256, AC-257, AC-258, AC-259, AC-260, AC-261, AC-262, AC-264, AC-265, AC-266, AC-267, AC-268, AC-270, AC-271, AC-273, AC-274, AC-275, AC-276, AC-277, AC-278, AC-279, AC-353, AC-359, AC-379, AC-384, AC-392, AC-396, AC-403, AC-406, AC-407, AC-408, AC-482 | 54 | The gate record is (token, tree hash, deadline) |
| **ADR-0018** | Rewrite events and common-dir worktree ownership | AC-122, AC-159, AC-166, AC-167, AC-168, AC-169, AC-170, AC-171, AC-176, AC-177, AC-178, AC-179, AC-180, AC-181, AC-182, AC-185, AC-188, AC-189, AC-190, AC-191, AC-192, AC-193, AC-194, AC-195, AC-196, AC-197, AC-198, AC-199, AC-364, AC-370, AC-371, AC-398, AC-518, AC-529 | 34 | Rewrite events and common-dir worktree ownership |
| **ADR-0019** | Per-AC discriminating test is the norm | AC-139, AC-140, AC-281, AC-284, AC-289, AC-315, AC-316, AC-317, AC-320, AC-321, AC-322, AC-323, AC-326, AC-329, AC-331, AC-332, AC-333, AC-334, AC-335, AC-336, AC-338, AC-339, AC-340, AC-341, AC-342, AC-344, AC-345, AC-346, AC-347, AC-348, AC-352, AC-353, AC-354, AC-355, AC-356, AC-357, AC-358, AC-359, AC-455, AC-457, AC-458 | 41 | A per-AC discriminating test is the product norm |
| **ADR-0020** | Stage rigor is a deontic enum | AC-038, AC-048, AC-050, AC-051, AC-058, AC-077, AC-105, AC-295, AC-298, AC-299, AC-300, AC-302, AC-303 | 13 | Rigor is an ordinal deontic enum |
| **ADR-0021** | Loop-aware memo key inside activity graphs | AC-014, AC-015, AC-016, AC-017, AC-018, AC-019 | 6 | The content-addressed, loop-aware memo key |
| **ADR-0022** | Activities are encapsulated graphs; technology packs contribute producers, not stages | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-007, AC-008, AC-009, AC-010, AC-011, AC-012, AC-013, AC-014, AC-016, AC-023, AC-024, AC-028, AC-029, AC-030, AC-033, AC-034, AC-035, AC-089, AC-097, AC-100, AC-106, AC-112, AC-259, AC-265, AC-269, AC-272, AC-360, AC-361 | 34 | Activities are encapsulated graphs; the workflow is deterministic |
| **ADR-0023** | Rigor is a deontic enum, not a summed numeric weight | AC-050, AC-069, AC-071, AC-295, AC-303 | 5 | Rigor admits comparison, never arithmetic |
| **ADR-0024** | Admissibility lives in the activity, requirement lives in the pack | AC-040, AC-041, AC-042, AC-043, AC-044, AC-045, AC-046, AC-054, AC-055, AC-056, AC-057, AC-058, AC-059, AC-060, AC-061, AC-063, AC-064, AC-066, AC-068, AC-078, AC-081, AC-082, AC-101, AC-113, AC-416, AC-430, AC-433, AC-435, AC-525 | 29 | Admissibility lives in the activity, requirement in the pack |
| **ADR-0025** | Human-supplied artifacts are vouched roots with advisory drift | AC-053, AC-066, AC-067, AC-068, AC-074, AC-127, AC-304, AC-305, AC-306, AC-310, AC-311, AC-319, AC-525 | 13 | Human-supplied artifacts are vouched roots with advisory drift |
| **ADR-0026** | Review is three encapsulated levels, not a stage-visible reviewer | AC-291, AC-292 | 2 | Review is three encapsulated levels, not a stage-visible reviewer |
| **ADR-0027** | Curated ~95-activity catalog at v0, with unshipped extended file | AC-094, AC-100, AC-101, AC-102, AC-106, AC-107, AC-110, AC-114, AC-414, AC-421, AC-426, AC-436 | 12 | A curated v0 catalog with an unshipped extended file |
| **ADR-0028** | Each activity graph owns the quality of its own output | AC-082, AC-105, AC-112, AC-115, AC-527 | 5 | Each activity graph owns its own output quality |
| **ADR-0029** | Technology packs add facts, producers and artifact kinds, never rubric rows | AC-080, AC-082, AC-325, AC-327, AC-328, AC-330, AC-337, AC-349, AC-356 | 9 | Technology packs add facts, producers and artifact kinds, never rubric rows |
| **ADR-0030** | Activity classification is closed facets compiling to applicable_if, not free tags | AC-044, AC-055, AC-065, AC-069, AC-070, AC-071, AC-072, AC-073, AC-074, AC-076, AC-079, AC-108, AC-109, AC-113, AC-130, AC-347, AC-351, AC-424, AC-425, AC-434, AC-436, AC-520 | 22 | `facets.toml` is the normative closed vocabulary — **partially superseded by ADR-0089** (PC8 rule 2's silence on what kind of computation counts, and the `auth` boolean); stays `accepted`, and rules 1, 3, 4, 5 and rule 2's prohibition itself are unchanged |
| **ADR-0032** | Absent optional inputs are null, with vacuous-true quantifiers and a lint error elsewhere | AC-434 | 1 | Absent optional inputs are null, with vacuous-true quantifiers and a lint error elsewhere |
| **ADR-0033** | Predicate grammar is fixed and closed, with no user-defined functions | AC-065, AC-433 | 2 | Predicate grammar is fixed and closed, with no user-defined functions |
| **ADR-0035** | for_each sources and fan-out output projection are fixed syntactic forms | AC-022 | 1 | `for_each` projection semantics |
| **ADR-0036** | Each step kind mints a fixed, closed set of evidence variants | AC-019, AC-307, AC-432 | 3 | Each step kind mints a fixed, closed set of evidence variants |
| **ADR-0037** | Budget attempts_total counts agent/tool steps per pass, not summed retries | AC-011, AC-020, AC-021, AC-026, AC-390, AC-391 | 6 | Budget `attempts_total` counts steps per pass |
| **ADR-0038** | Escalate carries only a question id; options live in the question registry | AC-259, AC-272, AC-409, AC-410 | 4 | Escalation carries only a question id |
| **ADR-0042** | Feedback refs are normative, not lint folklore | AC-420 | 1 | Feedback refs are normative, not lint folklore |
| **ADR-0043** | Loops must be able to learn — iteration enters the memo key | AC-017 | 1 | Loops must be able to learn — iteration enters the memo key |
| **ADR-0045** | A gate step's output is answer, never outputs | AC-409 | 1 | A gate step's answer binds nothing downstream |
| **ADR-0046** | Deviation is authorable through on_false, not just declarable | AC-061, AC-062, AC-063, AC-064 | 4 | Deviation is typed and never edits `R` |
| **ADR-0047** | Spawning a follow-on initiative is a pack rule, not an activity behaviour | AC-430 | 1 | Spawning a follow-on initiative is a pack rule, not an activity behaviour |
| **ADR-0050** | Tag semantics — same facet disjoins, distinct facets conjoin | AC-073, AC-109 | 2 | Tag semantics - the same facet disjoins, distinct facets conjoin |
| **ADR-0053** | A for_each step counts once against the attempts budget | AC-022 | 1 | `for_each` charges the attempts budget once |
| **ADR-0055** | Self-reference inside a loop binds the previous iteration's output | AC-017 | 1 | Self-reference inside a loop binds the previous iteration's output |
| **ADR-0056** | Family totality caps rank escalation by rigor | AC-046, AC-047, AC-048, AC-054, AC-056, AC-103, AC-104, AC-105, AC-116 | 9 | Family totality caps rank escalation by rigor |
| **ADR-0057** | Producer candidacy is stage-scoped but requires resolve from any unforbidden stage or a human source | AC-049, AC-051, AC-052, AC-053, AC-067, AC-077, AC-100 | 7 | Stage scope and forbidden stages |
| **ADR-0058** | Families are a declared catalog field, lint-checked for sharing and totality | AC-047, AC-048, AC-103, AC-104, AC-116 | 5 | Families are a declared catalog field, lint-checked for sharing and totality |
| **ADR-0059** | Registries are one table per value; a verb's live flag implies memo = false | AC-018, AC-070, AC-072 | 3 | Registries are one table per value; a verb's live flag implies memo = false |
| **ADR-0060** | Boolean facets are applicable_if-only; tagging one is a lint error | AC-073, AC-109 | 2 | Boolean facets are `applicable_if`-only |
| **ADR-0061** | facets.toml is the normative source for the toolchain and change facets | AC-070, AC-072, AC-108, AC-351, AC-424, AC-425, AC-520 | 7 | `facets.toml` is the normative source for the toolchain and change facets |
| **ADR-0063** | Gate output carries typed attachments bound to question options | AC-410 | 1 | Gate questions declare their options and attachment types |
| **ADR-0064** | Tool steps are typed by their verb; emit may compose from several refs | AC-018 | 1 | The typed verb set for tool steps |
| **ADR-0067** | recent_deploy joins the facet registry as a derived boolean | AC-108, AC-425 | 2 | Derived booleans and the contested facet list |
| **ADR-0069** | Ordering operators against null are lint errors; equality is defined | AC-069, AC-079 | 2 | `unknown` is never ordered against a band |
| **ADR-0072** | Coherence edits to shipped activities bump version and enter the changelog | AC-057, AC-107, AC-111, AC-117 | 4 | Shipped activity edits bump version |
| **ADR-0073** | Toolchain-keyed artifact kinds replace the specificity tie-break for mixed-toolchain items | AC-049, AC-088, AC-103, AC-349, AC-350 | 5 | Toolchain-keyed artifact kinds replace the specificity tie-break |
| **ADR-0074** | Review gate rebuilt — reviewer proposes, machine confirms | AC-025, AC-032, AC-038, AC-133, AC-138, AC-139, AC-140, AC-141, AC-142, AC-206, AC-208, AC-230, AC-231, AC-235, AC-236, AC-239, AC-280, AC-281, AC-282, AC-283, AC-284, AC-285, AC-286, AC-287, AC-288, AC-289, AC-290, AC-291, AC-292, AC-293, AC-294, AC-296, AC-304, AC-305, AC-307, AC-308, AC-309, AC-310, AC-312, AC-313, AC-315, AC-316, AC-318, AC-324, AC-329, AC-335, AC-340, AC-342, AC-343, AC-344, AC-345, AC-358, AC-428, AC-438, AC-439, AC-456, AC-477, AC-530 | 58 | — |
| **ADR-0075** | Typed artifacts, revision-4 refinements — permissive AC type, emitted gates, sidecar per companion, no plan.md | AC-143, AC-454 | 2 | — |
| **ADR-0076** | Port traits stay sync; async is confined to adapters behind an explicit cancel | AC-528 | 1 | — |
| **ADR-0077** | gix is restricted to the read path; the git write path shells out pending spike S-GIX | AC-174, AC-191 | 2 | — |
| **ADR-0078** | The cache rule is a split rule; both TTL variables are set and verified per run; the scope is machine and directory | AC-210, AC-526 | 2 | — |
| **ADR-0080** | The encapsulated quality block's policy predicate is not a usable terminal condition, and the quality shorthand's default stays unset | AC-527 | 1 | — |
| **ADR-0081** | The rigor ladder must be monotone in internal quality, and the lattice lint must enforce it | AC-105, AC-303 | 2 | — |
| **ADR-0082** | The Advisory sink is a first-class destination, not a discard | AC-311, AC-344, AC-530 | 3 | — |
| **ADR-0083** | Rust workspace, toolchain and lint policy | AC-001, AC-002, AC-003, AC-004 | 4 | — |
| **ADR-0086** | No durable-execution framework; the memo table is built here, re-argued on facts that hold | AC-005, AC-160 | 2 | — |
| **ADR-0087** | A mid-graph gate resumes into the same step, and costs less than a stage gate | AC-266, AC-524 | 2 | — |
| **ADR-0089** | A facet's computed fact must name its provenance; auth becomes a declared surfaces set, and absence of evidence is not false | AC-070, AC-072, AC-076, AC-079, AC-520 | 5 | A facet's computed fact must name its provenance; `auth` becomes a declared `surfaces` set; absence of evidence is not `false` (accepted 2026-09-09; partially supersedes ADR-0030) |
| **ADR-0093** | Kernel projections and the declared surfaces map are their own facet source classes; AC-520 gates only what a spike can score | AC-071, AC-072, AC-076, AC-105, AC-497, AC-520 | 6 | — |
| **ADR-0094** | Phase 1 ships a Rust toolchain adapter beside Node, and self-hosting means the fix lane and one story-lane run on Redkiln-RS itself | AC-496, AC-497, AC-521 | 3 | — |
| **ADR-0095** | One closed-output schema rule — no oneOf, anyOf only by transform, union-typed parameters counted per exported schema — and StepError enumerated | AC-131, AC-132, AC-157, AC-202, AC-204, AC-218, AC-225, AC-229, AC-234, AC-376, AC-379, AC-386, AC-428, AC-523 | 14 | — |
| **ADR-0096** | A finding's repro is typed — run or mutate, with the mutation and suite named — and a surviving mutant blocks only when it discriminates | AC-281, AC-283, AC-284, AC-289 | 4 | — |
| **ADR-0097** | The step memo key is SHA-256 over one length-prefixed ordered tuple, the memo stays tier 2, and 0021's per-step memo = false clause is withdrawn with 0043's | AC-002, AC-018, AC-161, AC-164, AC-187, AC-336 | 6 | — |
| **ADR-0098** | S-GIX/a is a read-path check whose no-go falls back to the wrapper; the review clone is a gix write op retained on purpose | AC-191 | 1 | — |
| **ADR-0099** | Harness dispatch blocks and returns the record; cancel is a token the composition root owns; containment is a Job Object on Windows and a process group on POSIX; ports are Arc<dyn Port + Send + Sync> | AC-217, AC-224, AC-225, AC-497, AC-528 | 5 | — |
| **ADR-0100** | yaml_serde is the YAML reader for redkiln-migrate only; 0.x crates are exact-pinned by name with named exemptions; cargo-deny enforces the ban list | AC-005, AC-044, AC-120 | 3 | — |
| **ADR-0101** | Negative-control fixtures gain a prior-approval framing arm, cited to the paper that measured it and reported on both halves of the classifier | AC-312 | 1 | — |
| **ADR-0102** | rust-version equals the pin, a member manifest's [lints] is workspace = true and nothing else, the only in-source override is #[expect], and the nursery and error-handling evidence lines are restated | AC-001, AC-528 | 2 | — |
| **ADR-0103** | The predicate grammar gets precedence and associativity, a literal-versus-field rule, quoting, null semantics per quantifier and connective, and a closed on_false form set | AC-069, AC-072, AC-079, AC-527 | 4 | — |
| **ADR-0104** | Per-AC mutants are declared by the AC's author, the flip predicate is AC-334's, the whole-suite comparator bootstraps from the first affected run, and a quarantined survivor still blocks exit | AC-317, AC-357 | 2 | — |
| **ADR-0105** | rk-git and rk-store contracts as rules — the GitOps seam, hooks isolation, trailer plumbing, the tree_hash algorithm, the wrapper's write set, one writer with a bounded busy_timeout, an event-kind table, and retention | AC-162, AC-163, AC-165, AC-184, AC-191, AC-483, AC-522, AC-524, AC-529 | 9 | — |
| **ADR-0106** | A mid-graph gate resumes into the same step; whether it costs less than a stage gate is unmeasured, and placement is not derived from a cost model | AC-266, AC-303 | 2 | — |
| **ADR-0107** | The CLI has one idempotent run entry point, no resume flag or verb, an idempotent init, and a committed argument-surface snapshot with a name denylist | AC-186, AC-187, AC-400, AC-437, AC-531 | 5 | — |
| **ADR-0108** | StructuredOutput is a configuration-time check — a schema-carrying dispatch whose tool set lacks it is refused before spawn, and nothing parses a schema out of prose | AC-095, AC-205, AC-417 | 3 | — |
| **ADR-0109** | reconcile() runs at every entry and nothing skips it; state is two tiers split by the deleted-database test; trailers carry three axes | AC-160, AC-161, AC-162, AC-163, AC-164, AC-165, AC-172, AC-173, AC-174, AC-175, AC-183, AC-184 | 12 | — |
| **ADR-0110** | Headroom is a batch delta over a named dispatch set in integer ticks, throttled on the visible unified windows with a two-tick margin, never synthesised per family and never credited from a cache metric | AC-372, AC-373, AC-374, AC-375, AC-376, AC-377, AC-379, AC-380, AC-381, AC-523 | 10 | — |
| **ADR-0111** | The S-P7B re-run must change the criterion, not only n — P7's criterion 1 had zero power at n = 3 and ten draws per cell are still powerless for the effect it registers | AC-303 | 1 | — |
| **ADR-0112** | StepError gains Platform, Unsupported and Overage — the three variants thirteen criteria and design §3.6 already named — making the closed set eleven | AC-218, AC-221, AC-494 | 3 | — |
| **ADR-0113** | TTL verification is read from the nested cache_creation object — a -p run with ephemeral_1h > 0 and ephemeral_5m = 0 is verified 1 h — and S-TTL1 narrows to the subagent bucket and the billing-mode confound | AC-526 | 1 | — |
| **ADR-0114** | The executed-once gate moves to extended→shipped promotion; the v0 pack tags on P6 lint, the F42 parse and a per-activity executed-once ledger the doctor reports | AC-110, AC-114 | 2 | — |

**How to read the spread.** ADR-0003 (87), ADR-0006 (58), ADR-0074 (58), ADR-0017 (54) and ADR-0019 (41) carry most of the register between them — the five ports, the review gate (as ADR-0074, which supersedes ADR-0004 in full), the artifact layout, the gate record and the per-AC discriminating test. That concentration is a risk as much as a fact: **the ADR text itself was outside the refutation pass's evidence set in every area**, so a misquoted or superseded clause in one of those five would propagate into dozens of criteria and nothing in this document would have caught it. Verifying those five ADRs against the criteria that cite them is the second-highest-value audit left; the first is §15.3.

**Regenerated 2026-09-11 (PIA-082, PIA-037).** This table is now computed, not maintained: the `title` column is every record's frontmatter `title` verbatim (ADR-0085 rule 2 — the glosses the 2026-09-09 pass corrected by hand moved to the note column), and the criteria column is recomputed from every criterion's own `decisions:` field at this revision [measured, parsed from the 109 frontmatters and the 527 criterion headers, 2026-09-11]. A parenthetical of the form `(supersedes ADR-NNNN)`, `(ADR-NNNN related)` or `(superseded by ADR-NNNN)` inside a `decisions:` field is not a citation of the record it names. 85 decisions are cited; 85 + 24 = 109. The 2026-09-11 amendment re-pointed `decisions:` fields to the twenty new records where they carry the rule — ADR-0074 for ADR-0004 register-wide (PIA-037), ADR-0107 for the CLI shape (AC-400, AC-437, AC-186), ADR-0108 for the StructuredOutput configuration-time check (AC-095, AC-205, AC-417), ADR-0109 for reconcile-at-every-entry and the tiers (AC-160–AC-165, AC-172–AC-175, AC-183), ADR-0110 for headroom (AC-372–AC-381) — and the earlier ADR-0018/ADR-0003/ADR-0011/ADR-0016 citations survive only where the port, the phase or the caching rule is the actual decision. **What is still not discharged** is the citation direction the 2026-09-09 pass named: `ADR-0059` is cited by **AC-070** and **AC-072** for the facet registry while its text is the *verb* registry record (AC-018 now cites it for exactly that — the `live` flag); AC-430's `ADR-0047` and AC-434's `ADR-0032` are in the same class. That is a register audit with a named consumer (this table and the §15.3 count), recorded per ADR-0012 rather than resolved by silent deletion.

Decisions cited by exactly one criterion — ADR-0032 (AC-434), ADR-0035 (AC-022), ADR-0042 (AC-420), ADR-0043 (AC-017), ADR-0045 (AC-409), ADR-0047 (AC-430), ADR-0053 (AC-022), ADR-0055 (AC-017), ADR-0063 (AC-410), ADR-0064 (AC-018), ADR-0076 (AC-528), ADR-0080 (AC-527), ADR-0098 (AC-191), ADR-0101 (AC-312), ADR-0111 (AC-303) — are not necessarily under-covered; most are narrow schema amendments whose whole content is one rule, and the new records (ADR-0101 → AC-312, ADR-0106 → AC-266, ADR-0111 → AC-303) are cited where their one consequence lands. They are listed so a reader can tell the difference between a decision with one criterion and a decision with none.

### 15.3 Decisions no criterion cites

`references/adr/` holds **112 decisions** — ADR-0001..ADR-0089 plus the twenty-three records of 2026-09-11, ADR-0092..ADR-0114 (ADR-0090 and ADR-0091 are allocated to runbook Steps 4.4 and 6 and are unwritten). The criteria cite **88**. The remaining **24** are listed below, because a decision no criterion cites is a decision nothing will test — and this was the largest single defect the 2026-09-08 assembly found. [measured, recomputed 2026-09-11]

| decision | title (frontmatter, verbatim) | status |
| --- | --- | --- |
| ADR-0001 | Adopt the inversion architecture (Bet 1) on the spike evidence | `accepted` |
| ADR-0004 | Review gate returns findings, not a verdict | `superseded` |
| ADR-0031 | on_false targets only earlier-declared steps; forward branching stays in when | `accepted` |
| ADR-0034 | Refs support a coalesce operator over a conditional step's output | `accepted` |
| ADR-0039 | After=<step> shorthand deferred pending recurrence at scale | `accepted` |
| ADR-0040 | Normative closed grammar for step input sources | `accepted` |
| ADR-0041 | Alternative outcomes materialise through a final tool/check step, exit set stays three | `superseded` |
| ADR-0044 | on_false is a bounded table, never a bare scalar target | `accepted` |
| ADR-0048 | Predicate grammar gains parentheses and quoted facet-value literals | `accepted` |
| ADR-0049 | Quantifier bodies admit full comparisons, not membership only | `accepted` |
| ADR-0051 | Provided artifacts are unconditional at every rigor at or above min_rigor | `accepted` |
| ADR-0052 | Lint rule 3 resolves paths against the step's output schema, not just step ids | `accepted` |
| ADR-0054 | Optional artifact refs are legal; alternation inside requires is not | `accepted` |
| ADR-0062 | Shared question ids are legal; a mark is identified by the triple | `accepted` |
| ADR-0065 | Lint R4b requires an on_false target reachable at every rigor the check is active | `accepted` |
| ADR-0066 | rigor is a legal root in applicable_if predicates | `accepted` |
| ADR-0068 | A gate step inside a for_each is a lint error | `accepted` |
| ADR-0070 | Quantifier bodies admit len() and nested calls | `accepted` |
| ADR-0071 | Empty evidence is legal only for activities with no agent step | `accepted` |
| ADR-0079 | The selection engine's context-vector source is undecided; Phase 1 pack authoring is blocked on P1's human panel | `proposed` |
| ADR-0084 | Dependency policy — exact-pinned gix, audited CI, a golden schema snapshot, one SQLite writer | `accepted` |
| ADR-0085 | Documentation and traceability conventions for the design phase | `accepted` |
| ADR-0088 | Negative-control fixtures gain a confirmation-bias arm | `superseded` |
| ADR-0092 | Errata to a frozen record live in ERRATA.md; the four authority documents have one precedence order; a cut id may be re-issued under a hazard table | `accepted` |

**The ADR-0074..ADR-0088 block, which the 2026-09-08 assembly found wholly uncited, is discharged for 11 of its fifteen records** — ADR-0074, ADR-0075, ADR-0076, ADR-0077, ADR-0078, ADR-0080, ADR-0081, ADR-0082, ADR-0083, ADR-0086, ADR-0087 are now cited by the criteria that carry their rules (the pointers the 2026-09-08 table inferred from filenames were checked against the record text on 2026-09-11 and the `decisions:` fields re-pointed; the one that changed on reading is ADR-0074, whose citations span §2, §5, §7, §8, §9, §10, §12, §13 — 58 criteria — not "the whole of §9" as the earlier row had it, PIA-037). **Still uncited from that block:** ADR-0079, ADR-0084, ADR-0085, ADR-0088 — ADR-0084 is superseded in part by ADR-0095 and ADR-0100, whose criteria carry its surviving clauses (AC-005, AC-044, AC-120, AC-131, AC-132, AC-157, AC-461); ADR-0085 (with ADR-0092) is the convention this register is written under and has no criterion by nature; ADR-0088 is superseded in full by ADR-0101 (AC-312).

**Of the twenty-three 2026-09-11 records**, ADR-0092 (errata, precedence, cut-id re-issue — a convention, cited in §0 and §0.4, no criterion) is uncited by nature; ADR-0098 (S-GIX/a's shell fallback, the retained review clone and S-GIX/c) is cited by AC-191; ADR-0100 (the dependency policy) by AC-005, AC-044, AC-120, AC-461, AC-484; ADR-0106 and ADR-0111 by AC-266 and AC-303; ADR-0112 (the three added `StepError` variants) by AC-218, AC-221 and AC-494; ADR-0113 (the per-run TTL read) by AC-526; ADR-0114 (the executed-once ledger and promotion route) by AC-110 and AC-114. A reader should check the table above before assuming a gap.

**The other uncited records** are ADR-0001 (the inversion bet — a premise, not a checkable behaviour), ADR-0004 (superseded by ADR-0074, whose citations replace it), ADR-0079 (`proposed`; the §1.1 block and AC-071 describe it and ADR-0090 supersedes it at runbook Step 4.4) and the activity-graph schema and predicate-grammar amendments: ADR-0031, ADR-0034, ADR-0039, ADR-0040, ADR-0041, ADR-0044, ADR-0048, ADR-0049, ADR-0051, ADR-0052, ADR-0054, ADR-0062, ADR-0065, ADR-0066, ADR-0068, ADR-0070, ADR-0071. ADR-0103 (2026-09-11) consolidates the grammar's precedence, literal, quoting, null and `on_false` rules and is cited by AC-069, AC-072, AC-079 and AC-527, which covers the part of ADR-0048/ADR-0069's territory a Phase-1 lint reaches; ERRATA E-0012-1 records that ADR-0054 extends ADR-0040 rather than superseding it. Several of the rest are plausibly narrower than a criterion needs to be — but **that has not been audited**, and at least three read as lint rules that ought to have one (ADR-0065's `on_false` target-rigor reachability lint, ADR-0068's gate-inside-a-loop rule, ADR-0066's slot-filler ordering). Recorded per ADR-0012 as an audit with a named consumer — this section's count — not discharged as prose.

---

## 16. Sources

**Area files (the direct inputs — session scratch, never committed).** Under ADR-0092 clause 2 **this register is the owner of every criterion's wording**; the thirteen area files were the 2026-09-08 session's working inputs, exist in no commit or branch of this repository [measured, `git log --all`, PIA-029/PIA-036], and every sentence elsewhere that says "the area file owns the wording" reads "the register owns the wording". Thirteen files, each authored against the proposal and the evidence brief and then adversarially refuted against the brief line by line: `ac-kernel.md`, `ac-selection.md`, `ac-packs-catalog.md`, `ac-artifacts-store.md`, `ac-git-reconcile.md`, `ac-harness.md`, `ac-human-gate.md`, `ac-review-gate.md`, `ac-verify-gate.md`, `ac-events-cost.md`, `ac-cli-doctor.md`, `ac-migration.md`, `ac-ports-dist.md`. All thirteen were present and complete; each area's own refutation notes record what was cut, what was repaired and what could not be repaired, and the per-area *Residual risk* blocks above are drawn from them.

**Repository.** Findings F1–F38 — F9 struck as cited-but-unsupported, F30 unassigned (S-H1b and S-WT never written up) — **which live in two places, and a re-runner must go to both**: `spikes/FINDINGS.md` holds **F1–F28** only, and **F29 plus F31–F40 exist solely as `spikes/F<NN>-DRAFT.md`**, none of them folded back into the running log [measured, `ls spikes/F*-DRAFT.md` and a heading scan of `FINDINGS.md`, 2026-09-09] — plus **F39** (`spikes/F39-DRAFT.md`, 2026-09-09, P1 arm D scored at pin `47faec9f` under METHOD-P amendment 9), which postdates the brief and was read from the corpus directly — read through the condensed evidence brief of 2026-09-08, which is the artifact each area was refuted against and which lives outside the repository: **anyone re-running a refutation must go back to `spikes/FINDINGS.md`, the `spikes/F*-DRAFT.md` set and `spikes/results/`, not to the brief**; `spikes/METHOD-S.md` and its §7 amendments; `spikes/results/` — `s-var.jsonl`, `s-var-citations.py`, `s-var-citation-targets.json`, `s-var-2400.jsonl` (quarantined), `s-typed.jsonl`, `s-typed-adjudication.jsonl`, `s-h1b-1m.jsonl`, `s-h1b-65m.jsonl`, `s-h1b-24h.jsonl` (mark only — the read was never taken), `s-wt.jsonl`, `s-mig.jsonl`, `census-s-mig.py`, `s-mig-report.md`, `e0-headroom.jsonl`, `e3.jsonl`; `spikes/corpus/PINNED_SHA` (`REDKILN_SHA=47faec9f…`, recorded dirty at pin time).

**References.** `references/2026-09-05-architecture-proposal.md` rev. 4 (§2, §3, §4.1–§4.9, §5, §6, §7, §8, §9); `references/2026-09-07-design-recommendation.md`; `references/2026-09-07-process-catalog-proposal.md` (§2.1–§2.7, §3, §5) and its PC rows; `references/2026-09-07-activity-graph-schema.md` (§5, §9a, §9b); `references/2026-09-01-shipmates-lessons-for-redkiln-rs.md` (L1.2, L3.1–L3.8, L4.1–L4.4, L5.3, L5.4, L5.7, L5.8); `references/2026-09-09-change-classification.md` (§0, §1, §4.1–§4.5, §5) — the recommendation behind ADR-0089, which reached its conclusion from six research axes without reference to F39's numbers; `references/adr/` — 89 decisions, of which 56 are cited by a criterion and 33 are not (§15.3). Every ADR id cited in §15.2 was checked against the directory and exists; **their text was not read by the refutation pass in any area**, and that limit is stated in §15.2.

**External comparators, quoted for order of magnitude only and never as a threshold.** Cloudflare, *Orchestrating AI Code Review at scale* (read-only median $0.98, full tier $1.68, break-glass 288 of 48,095 MRs = 0.6%) [1P]; Anthropic, *Code Review for Claude Code* ($15–25, ~20 min) [1P]; Agentless ($0.70/instance at 32.00%) and CodeR ($3.34 at 28.33%) [peer]; Anthropic prompt-caching, structured-outputs and Claude Code CLI documentation [1P]; git-branchless `RewriteEvent` and issue #1334, Jujutsu, Beads, Backlog.md #558 [1P]. Comparators that could not be found in the evidence set — GitLab quarantine, Cloudflare "won't fix", Google's ICSE 2021 one-live-mutant-per-changelist figure, SDK issue #571, Agentless' 213/300 reproduction count — were **cut from the criteria that cited them** and are named here only so nobody restores them from memory.

**Identifiers the refutation pass struck as fabrications — and the correction.** The thirteen area refuters were
instructed to check every citation against the condensed evidence brief and to cut what they could not find there. That
instruction was too narrow: the brief is a **subset** of the corpus, so a real citation absent from it looked fabricated.
A verification pass on 2026-09-08 re-checked every struck identifier against the full corpus. **All of the following are
real and are restored as citable**, with the source that carries them:

| struck as "does not exist" | actually | source |
| --- | --- | --- |
| spike **P3** | real — **cut**, on documented grounds | process-catalog proposal §6.1 |
| spike **P4** | real — **absorbed into P7** | `spikes/METHOD-P.md` §4 |
| spike **P5** | real — **reclassified** to a Phase 1 acceptance test; its replay-identity kill is live | process-catalog proposal §6 row P5, §6.1 |
| **GATE-02**, **GATE-05** | real conformance-rubric rules, both scored PARTIAL in Run 3 | `references/conformance-2026-09-04-scorecard.md`; proposal §1, §4.6 |
| **SDK issue #571** | real — the stray `{"output": …}` envelope the adapter must unwrap | proposal §4.3, §10 |
| **shipmates L2.5** | real — "bind a gate to a SHA, not a branch" | `references/2026-09-01-shipmates-lessons-for-redkiln-rs.md` |
| **`F35 §1.3`** | real, and exact — on a `node+unreal` item both change summaries are admissible at rank 10, specificity picks the UE one, and the text summary is **never produced**; satisfiable and wrong | `spikes/F35-DRAFT.md` §1.3 |
| **30.1% unsatisfiable** | real — ceilings without the family-totality rule took unsatisfiability from 11.4% to 30.1% | activity-graph schema §9c.26 |
| **+40% to +101%** | real — execution-grade review over the prose reviewer | proposal §4.6 |
| **0/156 historical reports** | real — the per-AC mutation column exists in none of them | proposal §4.6 item 8 |
| **≥12 of 23 shipped fixes carry an AC no test pins** | real (52%), after S-TYPED's mechanical adjudication | proposal §1 |

**F9 is the one striking that stands**, and for a different reason: F9 is *cited* across the corpus but has **no ledger
entry** — it is the finding whose absence forced pre-registration amendment 1. Citing it as evidence reproduces the defect
it was struck for, so it stays struck.

The lesson is recorded rather than hidden: **an adversarial pass is only as good as the corpus it is allowed to check
against.** Refuting against a condensation produces false positives that read exactly like true ones, and the cost here was
eleven real citations cut from criteria that needed them. Any criterion whose citation was cut on these grounds should be
read as evidenced, not evidence-thin.
