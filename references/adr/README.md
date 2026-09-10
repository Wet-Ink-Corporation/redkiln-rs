# Architecture decision records

**Purpose:** the durable, citable record of every architecture decision taken for Redkiln-RS during the design phase — **89 records**: `0001`–`0073` retrofitted from the three proposal documents that originally carried them as inline decision rows (`D1`–`D22`, `PC1`–`PC8`, `SCHEMA-1`–`SCHEMA-43`), and `0074`–`0089` written directly against the 2026-09-08 evidence brief and research pass, and against F39 (`legacy_id: none`, except `0074` and `0075`, which carry revision keys — see **Open items**). A decision is cited from here, not from the prose that first argued it.
**Format:** MADR-lite — YAML frontmatter with a fixed key list, six mandatory sections, and a required **What this forbids** block inside Consequences. One decision per file, one file per number.
**Lifecycle:** `proposed → accepted → (rejected | deprecated | superseded)` (`research/2026-09-03-workflow-model.md` §4). The 73 retrofitted records were entered at the status their source gave them; `0074`–`0088` were decided on 2026-09-08 and `0089` on 2026-09-09. The set now holds 86 `accepted`, 2 `superseded` (0004, 0041) and 1 `proposed` (0079, deliberately undecided — see **Open items**).
**Immutability rule:** an accepted ADR is **never edited**. It is superseded by a new record, and **both sides carry the link** — the successor's `supersedes: ["NNNN"]` and the predecessor's `superseded_by: ["NNNN"]`. See `research/2026-09-03-workflow-model.md` §4 ("Accepted ADRs are immutable — supersede, never edit") and `2026-09-05-architecture-proposal.md` §4.8 (".kb: … ADRs immutable (supersede)").
**Numbering:** monotonic 4-digit, zero-padded, never reused. `0001`–`0022` D-series · `0023`–`0030` PC-series · `0031`–`0073` schema amendments · `0074`+ new decisions.

---

## The immutability rule, stated plainly

**This is redkiln-rs's chosen convention, not an inherited standard.** It matches the `adr-tools -s` / Nygard lineage and the `.kb` `validate --kb` accepted-decision-immutability rule this repo already enforces, but MADR itself does *not* require it — the MADR template permits an accepted record's status to move to `deprecated` in place and allows *Consequences* and *Confirmation* to be updated over time. The stricter rule is adopted here deliberately, because these records are the traceability spine for a build that has not started and a reader must be able to trust that a record says what it said when it was cited. Where a downstream document presents immutability as an industry norm, that is an overclaim and should be corrected to this framing. [documented — MADR template, `github.com/adr/madr`; checked 2026-09-08, `references/2026-09-08-evidence-augmentation.md` §1 row 11]

Once a record reaches `accepted`, its body is frozen. A later decision that changes it does not edit it — it is written as a **new record** that names the old one, and the relation is recorded **on both files**:

- the new record carries `supersedes: ["<old>"]`;
- the old record carries `superseded_by: ["<new>"]`.

A one-sided link is a defect, not a shorthand. The relation is recorded at **record granularity**, which means a supersession that in substance withdraws only one clause is still written as a link between two whole records; the two bodies then say in prose which clause moved and which survives.

That distinction is what separates the two kinds:

- **Full supersession** — the predecessor's normative content is entirely replaced. Its status moves to `superseded`. No `accepted` record is ever the target of a full supersession.
- **Partial supersession** — one clause is withdrawn and the rest stands. The predecessor **stays `accepted`** and still carries the `superseded_by` link; both bodies record the scope of the withdrawal. Partial supersessions are legitimate and are not a defect to be "cleaned up" into a `related` link.

`related` is a separate, weaker relation: two records that bear on each other but where neither withdraws anything from the other. It is directional by convention in this set (a record names what it needed, and reciprocity is not required), and a pair joined by a supersession is **not** also listed as `related`.

The rule is inherited, not local to this folder: it is the contract these records will carry into `.kb/decisions/` at self-hosting.

## Format

MADR-lite. Frontmatter first, then the body.

**Frontmatter key list** — all eleven keys, in this order, on every record:

| Key | Content |
| --- | --- |
| `id` | quoted 4-digit string, equal to the filename's number (`id: "0043"`) |
| `title` | the decision as a sentence |
| `status` | `proposed` \| `accepted` \| `rejected` \| `deprecated` \| `superseded` |
| `date` | ISO date the decision was taken |
| `legacy_id` | the pre-retrofit citation key (`D14`, `PC7`, `SCHEMA-29`) — exactly one per record, unique across the set |
| `source` | the document and section the decision was retrofitted from |
| `supersedes` | list of ADR ids this record supersedes, `[]` if none |
| `superseded_by` | list of ADR ids that supersede this record, `[]` if none |
| `related` | list of ADR ids bearing on this one, `[]` if none |
| `evidence` | finding tags / spike ids backing the decision, `[]` if none |
| `consumers` | reserved for the self-hosting projection; **`[]` on every record in this set** |

Every id appearing in `supersedes`, `superseded_by` or `related` must resolve to a file that exists.

**Mandatory sections** — all six on every record:

1. `## Context and problem`
2. `## Decision drivers`
3. `## Considered options`
4. `## Decision`
5. `## Consequences`
6. `## Evidence and confidence`

`Decision` may be placed before `Considered options` where the options only make sense once the decision is stated; both orders are in use and both are valid. No section may be omitted.

**`What this forbids` is required.** Every record carries a `**What this forbids**` block inside `## Consequences`, stating what the decision makes illegal. It is the half of a decision that is actually enforceable, and a record without it is incomplete. It is present on all 89.

## Numbering

| Block | Range | Contents |
| --- | --- | --- |
| D-series | `0001`–`0022` | Architecture decisions D1–D22, from `2026-09-05-architecture-proposal.md` |
| PC-series | `0023`–`0030` | Process-catalog decisions PC1–PC8, from `2026-09-07-process-catalog-proposal.md` §7 |
| Schema amendments | `0031`–`0073` | Activity-graph schema amendments 1–43, from `2026-09-07-activity-graph-schema.md` §9a/§9b/§9c |
| New | `0074`+ | Every decision taken from now on. `0074`–`0089` are allocated; the next free number is `0090`. |

Numbers are assigned once and never reused, including for records that are later superseded or rejected.

## Legacy-id map

This table is how `D14` and `PC7` stay citable from the documents written before the retrofit. Rows are in legacy order.

| Legacy id | ADR | Title | Status |
| --- | --- | --- | --- |
| D1 | [0001](0001-inversion-architecture-bet.md) | Adopt the inversion architecture (Bet 1) on the spike evidence | accepted |
| D2 | [0002](0002-clean-break-self-hosting-switch.md) | Clean break from old redkiln; self-hosting is the cutover switch | accepted |
| D3 | [0003](0003-five-named-ports-no-postgres.md) | Five named ports with fakes; git excluded; no Postgres adapter | accepted |
| D4 | [0004](0004-review-gate-findings-not-verdicts.md) | Review gate returns findings, not a verdict | superseded (by 0074) |
| D5 | [0005](0005-human-edits-prose-free-structure-gated.md) | Human edits are prose-free and structure-gated | accepted (partially superseded by 0075) |
| D6 | [0006](0006-artifact-layout-c-split-fields.md) | Artifact layout C — split fields, opaque prose, rendered structure | accepted (partially superseded by 0075) |
| D7 | [0007](0007-codex-validation-only-second-harness.md) | Codex CLI is a second harness for validation only, not a gate-eligible adapter | accepted |
| D8 | [0008](0008-real-unreal-project-is-s-bin-fixture.md) | A real Unreal project is the S-BIN fixture in Phase 0; the toolchain adapter ships in Phase 2 | accepted |
| D9 | [0009](0009-cli-only-gate-channel-in-phase-1.md) | Phase 1 human gates answer from the CLI only; tracker and web channels wait for Phase 3 | accepted |
| D10 | [0010](0010-claude-adapter-subscription-only-auth.md) | The Claude adapter authenticates by subscription only, never --bare, and asserts no API key | accepted |
| D11 | [0011](0011-phase-1-ships-fix-and-story-lanes-only.md) | Phase 1 ships fix and story lanes only | accepted |
| D12 | [0012](0012-open-questions-become-named-gating-spikes.md) | Open questions become named, gating spikes, not prose | accepted |
| D13 | [0013](0013-corpus-lives-in-its-own-pinned-repo.md) | Corpus lives in its own pinned repo, not under spikes/ | accepted |
| D14 | [0014](0014-process-packs-own-structure-tech-packs-bind.md) | Process packs own structure; technology packs bind and contribute at slots | accepted |
| D15 | [0015](0015-review-tier-selects-execution-grade-or-read-only.md) | Review tier selects execution-grade or read-only, not a uniform policy | accepted (partially superseded by 0020) |
| D16 | [0016](0016-static-system-prompt-carries-the-card.md) | Static system prompt carries the card for Phase 1 | accepted (partially superseded by 0078) |
| D17 | [0017](0017-gate-record-is-token-tree-hash-deadline.md) | Gate record binds a single-use token to the tree hash, with a deadline and terminal path | accepted |
| D18 | [0018](0018-rewrite-events-and-common-dir-worktree-ownership.md) | Rewrite events and common-dir worktree ownership | accepted |
| D19 | [0019](0019-per-ac-discriminating-test-is-the-norm.md) | Per-AC discriminating test is the norm | accepted |
| D20 | [0020](0020-stage-rigor-is-a-deontic-enum.md) | Stage rigor is a deontic enum | accepted |
| D21 | [0021](0021-loop-aware-memo-key.md) | Loop-aware memo key inside activity graphs | accepted |
| D22 | [0022](0022-activities-are-encapsulated-graphs.md) | Activities are encapsulated graphs; technology packs contribute producers, not stages | accepted |
| PC1 | [0023](0023-rigor-as-deontic-enum.md) | Rigor is a deontic enum, not a summed numeric weight | accepted |
| PC2 | [0024](0024-admissibility-in-activity-requirement-in-pack.md) | Admissibility lives in the activity, requirement lives in the pack | accepted |
| PC3 | [0025](0025-vouched-root-advisory-drift.md) | Human-supplied artifacts are vouched roots with advisory drift | accepted |
| PC4 | [0026](0026-three-level-encapsulated-review.md) | Review is three encapsulated levels, not a stage-visible reviewer | accepted |
| PC5 | [0027](0027-curated-catalog-with-unshipped-extended-file.md) | Curated ~95-activity catalog at v0, with unshipped extended file | accepted |
| PC6 | [0028](0028-activity-graph-owns-output-quality.md) | Each activity graph owns the quality of its own output | accepted |
| PC7 | [0029](0029-technology-pack-contribution-contract.md) | Technology packs add facts, producers and artifact kinds, never rubric rows | accepted (partially superseded by 0073) |
| PC8 | [0030](0030-facets-not-tags.md) | Activity classification is closed facets compiling to applicable_if, not free tags | accepted (partially superseded by 0089) |
| SCHEMA-1 | [0031](0031-on-false-loop-back-only.md) | on_false targets only earlier-declared steps; forward branching stays in when | accepted |
| SCHEMA-2 | [0032](0032-absent-inputs-are-null.md) | Absent optional inputs are null, with vacuous-true quantifiers and a lint error elsewhere | accepted |
| SCHEMA-3 | [0033](0033-normative-predicate-grammar.md) | Predicate grammar is fixed and closed, with no user-defined functions | accepted |
| SCHEMA-4 | [0034](0034-coalesce-operator-in-refs.md) | Refs support a coalesce operator over a conditional step's output | accepted |
| SCHEMA-5 | [0035](0035-for-each-sources-and-projection.md) | for_each sources and fan-out output projection are fixed syntactic forms | accepted |
| SCHEMA-6 | [0036](0036-evidence-minting-is-normative.md) | Each step kind mints a fixed, closed set of evidence variants | accepted |
| SCHEMA-7 | [0037](0037-budget-attempts-total-counts-steps-per-pass.md) | Budget attempts_total counts agent/tool steps per pass, not summed retries | accepted |
| SCHEMA-8 | [0038](0038-escalate-carries-only-a-question-id.md) | Escalate carries only a question id; options live in the question registry | accepted |
| SCHEMA-9 | [0039](0039-after-step-shorthand-deferred.md) | `After=<step>` shorthand deferred pending recurrence at scale | accepted |
| SCHEMA-10 | [0040](0040-normative-step-input-source-grammar.md) | Normative closed grammar for step input sources | accepted |
| SCHEMA-11 | [0041](0041-alternative-outcomes-via-final-materialising-step.md) | Alternative outcomes materialise through a final tool/check step, exit set stays three | superseded |
| SCHEMA-12 | [0042](0042-feedback-refs-are-normative.md) | Feedback refs are normative, not lint folklore | accepted |
| SCHEMA-13 | [0043](0043-loops-must-learn.md) | Loops must be able to learn — iteration enters the memo key | accepted (partially superseded by 0059) |
| SCHEMA-14 | [0044](0044-on-false-table-form.md) | on_false is a bounded table, never a bare scalar target | accepted |
| SCHEMA-15 | [0045](0045-gate-output-is-answer-only.md) | A gate step's output is answer, never outputs | accepted |
| SCHEMA-16 | [0046](0046-deviation-is-authorable.md) | Deviation is authorable through on_false, not just declarable | accepted |
| SCHEMA-17 | [0047](0047-spawn-moves-to-the-pack.md) | Spawning a follow-on initiative is a pack rule, not an activity behaviour | accepted |
| SCHEMA-18 | [0048](0048-predicate-grammar-gains-parens-and-quoted-facet-values.md) | Predicate grammar gains parentheses and quoted facet-value literals | accepted |
| SCHEMA-19 | [0049](0049-quantifier-bodies-admit-full-comparisons.md) | Quantifier bodies admit full comparisons, not membership only | accepted |
| SCHEMA-20 | [0050](0050-tags-same-facet-disjoins-distinct-facets-conjoin.md) | Tag semantics — same facet disjoins, distinct facets conjoin | accepted |
| SCHEMA-21 | [0051](0051-provided-artifacts-unconditional-above-min-rigor.md) | Provided artifacts are unconditional at every rigor at or above min_rigor | accepted |
| SCHEMA-22 | [0052](0052-rule-3-resolves-paths-against-output-schema.md) | Lint rule 3 resolves paths against the step's output schema, not just step ids | accepted |
| SCHEMA-23 | [0053](0053-for-each-step-counts-once-against-budget.md) | A for_each step counts once against the attempts budget | accepted |
| SCHEMA-24 | [0054](0054-optional-artifact-refs-legal-requires-alternation-not.md) | Optional artifact refs are legal; alternation inside requires is not | accepted |
| SCHEMA-25 | [0055](0055-self-reference-binds-previous-loop-iteration.md) | Self-reference inside a loop binds the previous iteration's output | accepted |
| SCHEMA-26 | [0056](0056-family-totality-caps-rank-escalation.md) | Family totality caps rank escalation by rigor | accepted |
| SCHEMA-27 | [0057](0057-producer-scope-is-any-stage-or-human-source.md) | Producer candidacy is stage-scoped but requires resolve from any unforbidden stage or a human source | accepted |
| SCHEMA-28 | [0058](0058-families-are-a-declared-catalog-field.md) | Families are a declared catalog field, lint-checked for sharing and totality | accepted |
| SCHEMA-29 | [0059](0059-registries-are-one-table-per-value-with-live-flag.md) | Registries are one table per value; a verb's live flag implies memo = false | accepted |
| SCHEMA-30 | [0060](0060-boolean-facets-are-applicable-if-only.md) | Boolean facets are applicable_if-only; tagging one is a lint error | accepted |
| SCHEMA-31 | [0061](0061-facets-toml-normative-toolchain-and-change.md) | facets.toml is the normative source for the toolchain and change facets | accepted |
| SCHEMA-32 | [0062](0062-shared-question-ids-legal.md) | Shared question ids are legal; a mark is identified by the triple | accepted |
| SCHEMA-33 | [0063](0063-gate-attachments.md) | Gate output carries typed attachments bound to question options | accepted |
| SCHEMA-34 | [0064](0064-tool-steps-typed-by-verb-composing-emit.md) | Tool steps are typed by their verb; emit may compose from several refs | accepted |
| SCHEMA-35 | [0065](0065-lint-r4b-on-false-target-rigor-reachability.md) | Lint R4b requires an on_false target reachable at every rigor the check is active | accepted |
| SCHEMA-36 | [0066](0066-rigor-legal-root-in-applicable-if.md) | rigor is a legal root in applicable_if predicates | accepted |
| SCHEMA-37 | [0067](0067-recent-deploy-derived-facet.md) | recent_deploy joins the facet registry as a derived boolean | accepted |
| SCHEMA-38 | [0068](0068-gate-inside-for-each-is-lint-error.md) | A gate step inside a for_each is a lint error | accepted |
| SCHEMA-39 | [0069](0069-null-ordering-operators-are-lint-errors.md) | Ordering operators against null are lint errors; equality is defined | accepted |
| SCHEMA-40 | [0070](0070-quantifier-bodies-admit-len-and-nesting.md) | Quantifier bodies admit len() and nested calls | accepted |
| SCHEMA-41 | [0071](0071-empty-evidence-requires-no-agent-step.md) | Empty evidence is legal only for activities with no agent step | accepted |
| SCHEMA-42 | [0072](0072-shipped-activity-edits-bump-version.md) | Coherence edits to shipped activities bump version and enter the changelog | accepted |
| SCHEMA-43 | [0073](0073-toolchain-keyed-kinds-replace-specificity-tiebreak.md) | Toolchain-keyed artifact kinds replace the specificity tie-break for mixed-toolchain items | accepted |

## Supersessions

Nine supersession links exist in this set, from eight superseding records — `0075` supersedes two predecessors. Two links are full; seven are partial and leave their predecessor `accepted`.

| Superseded | By | Full or partial | What changed |
| --- | --- | --- | --- |
| [0015](0015-review-tier-selects-execution-grade-or-read-only.md) (D15) | [0020](0020-stage-rigor-is-a-deontic-enum.md) (D20) | Partial | The numeric `0.2`/`1.0` review weights are replaced by the deontic enum `forbidden < discouraged < optional < recommended < mandatory`, mapping `0.2 → optional` and `1.0 → mandatory`. D15's normative content — the *tiering rule* itself, that the weight selects execution-grade vs read-only and that `bug-standard` is execution-grade — survives the renumbering, so 0015 stays `accepted`. |
| [0029](0029-technology-pack-contribution-contract.md) (PC7) | [0073](0073-toolchain-keyed-kinds-replace-specificity-tiebreak.md) (SCHEMA-43) | Partial | PC7(ii)'s "toolchain match beats generic" specificity tie-break is **withdrawn** for the five toolchain-keyed artifact kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`). A required keyed kind now expands to one instance per element of the item's `toolchain` set, one producer per instance, and a generic producer's `'text' in change` becomes an instance-level predicate. Measured cause: a mixed Node+UE item silently received the UE change summary only (F35 §1.3). PC7(i), (iii), (iv), the fact/producer/artifact-kind contract, the prohibition on stages and `require` rows, and specificity as applied to non-keyed kinds all stand — so 0029 stays `accepted`. |
| [0041](0041-alternative-outcomes-via-final-materialising-step.md) (SCHEMA-11) | [0064](0064-tool-steps-typed-by-verb-composing-emit.md) (SCHEMA-34) | **Full** | §9a.11's "unconditional final `tool`/`check` step materialises the artifact" pattern is superseded by `emit` composition (`Kind = { field = "${ref}", … }`, each field checked against the target kind's schema). A `tool` step's `outputs` must now equal its verb's registered `outputs` or be omitted, and `toolchain:record` is removed as a verb. The source scopes its supersession to pure-composition cases; because this set records supersession at record granularity, 0041 is carried at status **`superseded`** pointing here, and 0064's body records the narrower scope. |
| [0043](0043-loops-must-learn.md) (SCHEMA-13) | [0059](0059-registries-are-one-table-per-value-with-live-flag.md) (SCHEMA-29) | Partial | Amendment 13's **second sentence only** is withdrawn: a step may no longer declare `memo = false` to assert liveness. Liveness moves to a `live = true` flag on the verb's own registry entry, which implies `memo = false` on every step using that verb. Amendment 13's first sentence — the loop iteration counter enters the memo key — and its lint rule remain in force unchanged, so 0043 stays `accepted`. |
| [0004](0004-review-gate-findings-not-verdicts.md) (D4) | [0074](0074-review-gate-reviewer-proposes-machine-confirms.md) | **Full** | The review gate is rebuilt as "reviewer proposes, machine confirms": the evidence class is computed from the repro record rather than taken from the model, a finding blocks only after the deterministic gate re-executes it and sees it flip, `Unverifiable`/`Quarantined` enter the state set, and the policy table is the single place a class becomes a block. D4's normative content — findings not verdicts, Rust computes the block — survives inside 0074, but every operative term is now defined differently and D4's "pending S-VAR" provisionality no longer describes the evidence, so 0004 moves to status **`superseded`**. |
| [0005](0005-human-edits-prose-free-structure-gated.md) (D5) | [0075](0075-typed-artifacts-revision-4-refinements.md) | Partial | Withdrawn: only the implication that a structured field's shape is fixed independently of what the corpus contains — S-MIG's census (1,312 files / 6,877 sections) forces the AC type permissive (`gwt` and `verification` both optional; 19 of 153 shipped specs carry neither). D5's prose-free/structure-gated rule, its CLI/web/gate-only write path and its locator-not-a-grammar rule all stand, so 0005 stays `accepted`. |
| [0006](0006-artifact-layout-c-split-fields.md) (D6) | [0075](0075-typed-artifacts-revision-4-refinements.md) | Partial | Withdrawn: `plan.md` leaves the human-facing artifact list (110/110 were the unfilled scaffold), and the companion sidecars are refined into one schema per companion kind. Layout C itself, digest-stamped read-only structured sections, typed drift surfaced not applied, and "nothing parses structure out of prose" all stand, so 0006 stays `accepted`. |
| [0016](0016-static-system-prompt-carries-the-card.md) (D16) | [0078](0078-cache-rule-is-split-both-ttls-set-scope-corrected.md) | Partial | Withdrawn: the single `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` pin as *the* TTL fix (`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` is required too, and no run may be reported as 1h without per-run `ephemeral_*` field evidence), and the "the cache is org-scoped" justification for pinning the CLI version. D16's delivery clause — `--system-prompt` carries the card body, static, for Phase 1, append route retained as a config option pending S-H1c — is untouched, so 0016 stays `accepted`. |
| [0030](0030-facets-not-tags.md) (PC8) | [0089](0089-facet-provenance-declared-surfaces-and-abstention.md) | Partial | Withdrawn: rule 2's silence on **what kind of computation counts** as "a computed fact", and the repo-wide `auth` boolean named in PC8's initial-facets sentence. A computed fact must now name its evidence source and provenance class, `facets.toml`'s `source` gains a `deriver` companion, `auth` becomes a set-valued `surfaces` facet over a declared path map, and a derived boolean's absence of evidence resolves to `null` rather than `false`. Measured cause: the lexical deriver scores **precision 0.00 over 11 positives** on `auth`, 95% CI [0.00, 0.26] — every positive false, because `auth` is a substring of *authored*, `oauth` of *coauthor* and `token` matches a design token — and `data_migration` emits **zero positives on 20 items** and is not measurable (F39). PC8 rules 1, 3, 4 and 5, rule 2's prohibition itself, and the tags-are-sugar status all stand — so 0030 stays `accepted`. |

Reference relations that are deliberately **not** supersessions:

- `0023` (PC1) and `0020` (D20) are the **same decision recorded in two homes** — the rigor enum as the architecture proposal states it and as the process-catalog proposal states it. They are `related`. Neither withdraws anything from the other.
- `0028` (PC6), `0029` (PC7) and `0022` (D22) are `related`: encapsulated activity graphs, the technology-pack contribution contract, and the D-series record of both. No withdrawal among them.
- `0089` **generalises** `0067` (SCHEMA-37) rather than superseding it. `0067` already emits `null` under rule `R-recent-deploy-feed-absent` when the `toolchain:deploy_log` feed is absent; `0089` extends that same abstention from one derived boolean to all of them, so there is one mechanism and not two. Nothing of `0067` is withdrawn — its facet, its 24 h anchor and its `wants_facet` prohibition all stand — so the relation is `related`, recorded one way on `0089` because `0067` is `accepted` and therefore frozen.

## Where these live eventually

At self-hosting these records migrate to `.kb/decisions/<NNNN>-<slug>.md` under `KbFrontmatter`, joining the `.kb` intake → adjudicate → integrate contract (`2026-09-05-architecture-proposal.md` §4.8; `research/2026-09-03-workflow-model.md` §4). The rule crosses with them unchanged: **ADRs are immutable — supersede, never edit.** `.kb` writes are agent-autonomous within that contract, and ADR **status transitions are the one human gate**. The `consumers` key, `[]` throughout this set, is the hook the projection fills in once atoms are injected into activities by declared inclusion rules.

Until then this folder is the canonical location and the legacy-id map above is the migration key.

## Open items

**No gap, duplicate or unresolvable defect remains.** The set is exactly 89 files, `0001`–`0089`, with no gaps and no duplicate numbers; every `legacy_id` in `D1`–`D22`, `PC1`–`PC8`, `SCHEMA-1`–`SCHEMA-43` appears exactly once; `0074` and `0075` carry the revision keys `D4 rev.4` and `D5/D6 rev.4` rather than `none`, because each is the revision-4 rewrite of the D-series decision it supersedes, and `0076`–`0089` carry `legacy_id: none`; every id reference resolves; `consumers` is `[]` everywhere; the supersession graph is acyclic; every record carries all six sections and a `**What this forbids**` block; and exactly three records are not at `accepted` — `0041` and `0004`, the two targets of a full supersession, at `superseded`, and `0079` at **`proposed`**, which is correct because the axis-source question it records is not decided by anything measured so far and 0079 decides nothing in its place.

**Open questions carried at `proposed` or explicitly unresolved inside an accepted record.** `0079` is the only `proposed` record: Phase 1 pack authoring over the LLM-emitted facets is blocked and the record forbids being amended into a decision. **Its blocking condition has moved but has not lifted.** AC-071's five clauses include an arm-D panel scored, and that clause is now discharged — arm D was scored on 2026-09-09 against 140 panel-adjudicated truth cells [measured, F39] — while the clause that actually holds the block is untouched: modal required-set identity was measured at 0.25 / 0.15 / 0.10 / 0.10 against an 0.80 line [measured, F36]. F39 measures a *derived* context vector; 0079 asks about an *LLM-emitted* one, so it contributes to that record without deciding it. Five accepted records carry a named open item rather than a resolution: `0078` registers spike **S-TTL1** against the F3-versus-documentation contradiction over the `ephemeral_1h`/`ephemeral_5m` fields; `0077` registers **S-GIX** against gix's undocumented trailer-write support; `0080` leaves the quality shorthand's default explicitly unset pending a re-run at n = 10 per cell and a settled severity vocabulary; `0088` registers an unrun confirmation-bias arm and forbids claiming the gate is guarded against false approval until it runs; and `0089` registers **S-FACET** against the unscored `surfaces` facet and leaves the source-asymmetry question (`commits` beats `docs`; `union` is worse than `commits` alone on two facets) explicitly undecided.

Three items are recorded as notes rather than defects:

- **Record-granularity supersession is coarser than the sources.** Both `0041`/`0064` and `0043`/`0059` supersede in substance only part of the predecessor, and `0041`'s source scopes its supersession to pure-composition cases. The link is nevertheless recorded whole-record, per the rule above, with the scope written into both bodies. Anyone acting on `0041` must read `0064`'s Decision section for what actually survives.
- **`related` is directional.** It is not maintained reciprocally across the set (114 of the 166 `related` edges point one way only), and this is convention, not an error — a record names what it drew on. Do not infer from a missing back-link that a relation was dropped.
- **One clause of `0067`'s Evidence section has been overtaken, and the correction lives here because the record is frozen.** `0067` says of the schema row's "P1 arm D measures it": "a forward reference only. No measurement result is given, and P1 arm D has not been read out (its blind panel is unfilled), so nothing here rests on a measured effect." Arm D **was** read out, on 2026-09-09 [measured, F39], so that clause no longer holds. Everything the sentence was doing still does. The parenthetical remains literally true — the pre-registered blind panel file was never filled; METHOD-P amendment 9 replaced that oracle with a deterministic one on the mechanical facets and an adjudicated model panel on the semantic ones, and the file is retained unfilled [documented, METHOD-P §0 amendment 9]. And `recent_deploy` does **not** appear in F39's precision table: the `toolchain:deploy_log` feed does not exist at the corpus pin, so arm D emitted the facet as `null` under `R-recent-deploy-feed-absent` instead of scoring it [measured, F39; `spikes/p1/derive-facets.py`]. The forward reference is therefore still unfulfilled, and `0067`'s Decision continues to rest on no measured effect — which is what the sentence was disclaiming, not using.

One cosmetic inconsistency was left as-is because fixing it would mean editing accepted records for no normative gain: the required forbids block is punctuated three ways across the set — `**What this forbids**` (75), `**What this forbids**:` (11), `**What this forbids:**` (3), which is all 89. The canonical form for new records is `**What this forbids**`.
