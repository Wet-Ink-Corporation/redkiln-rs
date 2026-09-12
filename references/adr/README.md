# Architecture decision records

**Purpose:** the durable, citable record of every architecture decision taken for Redkiln-RS during the design phase — **114 records**: `0001`–`0073` retrofitted from the three proposal documents that originally carried them as inline decision rows (`D1`–`D22`, `PC1`–`PC8`, `SCHEMA-1`–`SCHEMA-43`); `0074`–`0089` written directly against the 2026-09-08 evidence brief and research pass, and against F39; and `0092`–`0116` written on 2026-09-11 against the pre-implementation audit, and on 2026-09-12 against the runbook alignment review (`0115`, `0116`), (`references/2026-09-10-pre-implementation-audit.md`) — `legacy_id: none` on all of them except `0074` and `0075`, which carry revision keys (see **Open items**). `0090` and `0091` are **reserved, unwritten** numbers the runbook allocates to Step 4.4 (the context-vector record superseding `0079`) and Step 6.1 (the S-GIX result and git-version floor). Corrections to a frozen record's evidence lines live in [`ERRATA.md`](ERRATA.md) (ADR-0092), never in the record. A decision is cited from here, not from the prose that first argued it.
**Format:** MADR-lite — YAML frontmatter with a fixed key list, six mandatory sections, and a required **What this forbids** block inside Consequences. One decision per file, one file per number.
**Lifecycle:** `proposed → accepted → (rejected | deprecated | superseded)` (`research/2026-09-03-workflow-model.md` §4). The 73 retrofitted records were entered at the status their source gave them; `0074`–`0088` were decided on 2026-09-08, `0089` on 2026-09-09 and `0092`–`0114` on 2026-09-11. The set now holds **110 `accepted`, 3 `superseded`** (0004, 0041, 0088) **and 1 `proposed`** (0079, deliberately undecided — see **Open items**).
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

**`What this forbids` is required.** Every record carries a `**What this forbids**` block inside `## Consequences`, stating what the decision makes illegal. It is the half of a decision that is actually enforceable, and a record without it is incomplete. It is present on all 114.

## Numbering

| Block | Range | Contents |
| --- | --- | --- |
| D-series | `0001`–`0022` | Architecture decisions D1–D22, from `2026-09-05-architecture-proposal.md` |
| PC-series | `0023`–`0030` | Process-catalog decisions PC1–PC8, from `2026-09-07-process-catalog-proposal.md` §7 |
| Schema amendments | `0031`–`0073` | Activity-graph schema amendments 1–43, from `2026-09-07-activity-graph-schema.md` §9a/§9b/§9c |
| New | `0074`+ | Every decision taken from now on. `0074`–`0089` and `0092`–`0116` are allocated; `0090` and `0091` are reserved for the runbook's Step 4.4 and Step 6.1 records; the next free number is **`0117`**. |

Numbers are assigned once and never reused, including for records that are later superseded or rejected.

## Legacy-id map

This table is how `D14` and `PC7` stay citable from the documents written before the retrofit. Rows are in legacy order.

| Legacy id | ADR | Title | Status |
| --- | --- | --- | --- |
| D1 | [0001](0001-inversion-architecture-bet.md) | Adopt the inversion architecture (Bet 1) on the spike evidence | accepted (errata) |
| D2 | [0002](0002-clean-break-self-hosting-switch.md) | Clean break from old redkiln; self-hosting is the cutover switch | accepted |
| D3 | [0003](0003-five-named-ports-no-postgres.md) | Five named ports with fakes; git excluded; no Postgres adapter | accepted (partially superseded by 0094) |
| D4 | [0004](0004-review-gate-findings-not-verdicts.md) | Review gate returns findings, not a verdict | superseded (by 0074) |
| D5 | [0005](0005-human-edits-prose-free-structure-gated.md) | Human edits are prose-free and structure-gated | accepted (partially superseded by 0075) |
| D6 | [0006](0006-artifact-layout-c-split-fields.md) | Artifact layout C — split fields, opaque prose, rendered structure | accepted (partially superseded by 0075) |
| D7 | [0007](0007-codex-validation-only-second-harness.md) | Codex CLI is a second harness for validation only, not a gate-eligible adapter | accepted |
| D8 | [0008](0008-real-unreal-project-is-s-bin-fixture.md) | A real Unreal project is the S-BIN fixture in Phase 0; the toolchain adapter ships in Phase 2 | accepted |
| D9 | [0009](0009-cli-only-gate-channel-in-phase-1.md) | Phase 1 human gates answer from the CLI only; tracker and web channels wait for Phase 3 | accepted |
| D10 | [0010](0010-claude-adapter-subscription-only-auth.md) | The Claude adapter authenticates by subscription only, never --bare, and asserts no API key | accepted |
| D11 | [0011](0011-phase-1-ships-fix-and-story-lanes-only.md) | Phase 1 ships fix and story lanes only | accepted |
| D12 | [0012](0012-open-questions-become-named-gating-spikes.md) | Open questions become named, gating spikes, not prose | accepted (errata) |
| D13 | [0013](0013-corpus-lives-in-its-own-pinned-repo.md) | Corpus lives in its own pinned repo, not under spikes/ | accepted |
| D14 | [0014](0014-process-packs-own-structure-tech-packs-bind.md) | Process packs own structure; technology packs bind and contribute at slots | accepted |
| D15 | [0015](0015-review-tier-selects-execution-grade-or-read-only.md) | Review tier selects execution-grade or read-only, not a uniform policy | accepted (partially superseded by 0020) |
| D16 | [0016](0016-static-system-prompt-carries-the-card.md) | Static system prompt carries the card for Phase 1 | accepted (partially superseded by 0078) |
| D17 | [0017](0017-gate-record-is-token-tree-hash-deadline.md) | Gate record binds a single-use token to the tree hash, with a deadline and terminal path | accepted |
| D18 | [0018](0018-rewrite-events-and-common-dir-worktree-ownership.md) | Rewrite events and common-dir worktree ownership | accepted |
| D19 | [0019](0019-per-ac-discriminating-test-is-the-norm.md) | Per-AC discriminating test is the norm | accepted (errata) |
| D20 | [0020](0020-stage-rigor-is-a-deontic-enum.md) | Stage rigor is a deontic enum | accepted |
| D21 | [0021](0021-loop-aware-memo-key.md) | Loop-aware memo key inside activity graphs | accepted (partially superseded by 0097) |
| D22 | [0022](0022-activities-are-encapsulated-graphs.md) | Activities are encapsulated graphs; technology packs contribute producers, not stages | accepted |
| PC1 | [0023](0023-rigor-as-deontic-enum.md) | Rigor is a deontic enum, not a summed numeric weight | accepted |
| PC2 | [0024](0024-admissibility-in-activity-requirement-in-pack.md) | Admissibility lives in the activity, requirement lives in the pack | accepted |
| PC3 | [0025](0025-vouched-root-advisory-drift.md) | Human-supplied artifacts are vouched roots with advisory drift | accepted |
| PC4 | [0026](0026-three-level-encapsulated-review.md) | Review is three encapsulated levels, not a stage-visible reviewer | accepted |
| PC5 | [0027](0027-curated-catalog-with-unshipped-extended-file.md) | Curated ~95-activity catalog at v0, with unshipped extended file | accepted (partially superseded by 0114) |
| PC6 | [0028](0028-activity-graph-owns-output-quality.md) | Each activity graph owns the quality of its own output | accepted |
| PC7 | [0029](0029-technology-pack-contribution-contract.md) | Technology packs add facts, producers and artifact kinds, never rubric rows | accepted (partially superseded by 0073) |
| PC8 | [0030](0030-facets-not-tags.md) | Activity classification is closed facets compiling to applicable_if, not free tags | accepted (partially superseded by 0089) |
| SCHEMA-1 | [0031](0031-on-false-loop-back-only.md) | on_false targets only earlier-declared steps; forward branching stays in when | accepted |
| SCHEMA-2 | [0032](0032-absent-inputs-are-null.md) | Absent optional inputs are null, with vacuous-true quantifiers and a lint error elsewhere | accepted (partially superseded by 0103) |
| SCHEMA-3 | [0033](0033-normative-predicate-grammar.md) | Predicate grammar is fixed and closed, with no user-defined functions | accepted (partially superseded by 0103) |
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
| SCHEMA-18 | [0048](0048-predicate-grammar-gains-parens-and-quoted-facet-values.md) | Predicate grammar gains parentheses and quoted facet-value literals | accepted (partially superseded by 0103) |
| SCHEMA-19 | [0049](0049-quantifier-bodies-admit-full-comparisons.md) | Quantifier bodies admit full comparisons, not membership only | accepted |
| SCHEMA-20 | [0050](0050-tags-same-facet-disjoins-distinct-facets-conjoin.md) | Tag semantics — same facet disjoins, distinct facets conjoin | accepted |
| SCHEMA-21 | [0051](0051-provided-artifacts-unconditional-above-min-rigor.md) | Provided artifacts are unconditional at every rigor at or above min_rigor | accepted |
| SCHEMA-22 | [0052](0052-rule-3-resolves-paths-against-output-schema.md) | Lint rule 3 resolves paths against the step's output schema, not just step ids | accepted |
| SCHEMA-23 | [0053](0053-for-each-step-counts-once-against-budget.md) | A for_each step counts once against the attempts budget | accepted |
| SCHEMA-24 | [0054](0054-optional-artifact-refs-legal-requires-alternation-not.md) | Optional artifact refs are legal; alternation inside requires is not | accepted |
| SCHEMA-25 | [0055](0055-self-reference-binds-previous-loop-iteration.md) | Self-reference inside a loop binds the previous iteration's output | accepted |
| SCHEMA-26 | [0056](0056-family-totality-caps-rank-escalation.md) | Family totality caps rank escalation by rigor | accepted (errata) |
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
| SCHEMA-37 | [0067](0067-recent-deploy-derived-facet.md) | recent_deploy joins the facet registry as a derived boolean | accepted (errata) |
| SCHEMA-38 | [0068](0068-gate-inside-for-each-is-lint-error.md) | A gate step inside a for_each is a lint error | accepted |
| SCHEMA-39 | [0069](0069-null-ordering-operators-are-lint-errors.md) | Ordering operators against null are lint errors; equality is defined | accepted (partially superseded by 0103) |
| SCHEMA-40 | [0070](0070-quantifier-bodies-admit-len-and-nesting.md) | Quantifier bodies admit len() and nested calls | accepted |
| SCHEMA-41 | [0071](0071-empty-evidence-requires-no-agent-step.md) | Empty evidence is legal only for activities with no agent step | accepted |
| SCHEMA-42 | [0072](0072-shipped-activity-edits-bump-version.md) | Coherence edits to shipped activities bump version and enter the changelog | accepted |
| SCHEMA-43 | [0073](0073-toolchain-keyed-kinds-replace-specificity-tiebreak.md) | Toolchain-keyed artifact kinds replace the specificity tie-break for mixed-toolchain items | accepted |

## New records (`0074`+)

Records with `legacy_id: none` (and the two revision-keyed ones). `(errata)` marks a record with an entry in [`ERRATA.md`](ERRATA.md).

| Record | Title | Date | Status |
| --- | --- | --- | --- |
| [0074](0074-review-gate-reviewer-proposes-machine-confirms.md) | Review gate rebuilt — reviewer proposes, machine confirms | 2026-09-08 | accepted (partially superseded by 0096) (errata) |
| [0075](0075-typed-artifacts-revision-4-refinements.md) | Typed artifacts, revision-4 refinements — permissive AC type, emitted gates, sidecar per companion, no plan.md | 2026-09-08 | accepted (errata) |
| [0076](0076-port-traits-stay-sync-async-confined-to-adapters.md) | Port traits stay sync; async is confined to adapters behind an explicit cancel | 2026-09-08 | accepted (partially superseded by 0099) |
| [0077](0077-gix-read-path-only-git-write-path-shells-out.md) | gix is restricted to the read path; the git write path shells out pending spike S-GIX | 2026-09-08 | accepted (partially superseded by 0098) (errata) |
| [0078](0078-cache-rule-is-split-both-ttls-set-scope-corrected.md) | The cache rule is a split rule; both TTL variables are set and verified per run; the scope is machine and directory | 2026-09-08 | accepted (partially superseded by 0113) |
| [0079](0079-context-vector-source-undecided-phase-1-blocked-on-p1.md) | The selection engine's context-vector source is undecided; Phase 1 pack authoring is blocked on P1's human panel | 2026-09-08 | proposed (errata) |
| [0080](0080-quality-block-policy-predicate-is-not-a-terminal-condition.md) | The encapsulated quality block's policy predicate is not a usable terminal condition, and the quality shorthand's default stays unset | 2026-09-08 | accepted (partially superseded by 0111) (errata) |
| [0081](0081-rigor-ladder-must-be-monotone-in-internal-quality.md) | The rigor ladder must be monotone in internal quality, and the lattice lint must enforce it | 2026-09-08 | accepted (errata) |
| [0082](0082-advisory-sink-is-a-first-class-destination.md) | The Advisory sink is a first-class destination, not a discard | 2026-09-08 | accepted (errata) |
| [0083](0083-rust-workspace-toolchain-and-lint-policy.md) | Rust workspace, toolchain and lint policy | 2026-09-08 | accepted (partially superseded by 0102) (errata) |
| [0084](0084-dependency-policy.md) | Dependency policy — exact-pinned gix, audited CI, a golden schema snapshot, one SQLite writer | 2026-09-08 | accepted (partially superseded by 0095, 0100) (errata) |
| [0085](0085-documentation-and-traceability-conventions.md) | Documentation and traceability conventions for the design phase | 2026-09-08 | accepted (partially superseded by 0092) (errata) |
| [0086](0086-no-durable-execution-framework-memo-table-built-here.md) | No durable-execution framework; the memo table is built here, re-argued on facts that hold | 2026-09-08 | accepted (errata) |
| [0087](0087-mid-graph-gate-resumes-into-the-same-step.md) | A mid-graph gate resumes into the same step, and costs less than a stage gate | 2026-09-08 | accepted (partially superseded by 0106) (errata) |
| [0088](0088-negative-controls-gain-a-confirmation-bias-arm.md) | Negative-control fixtures gain a confirmation-bias arm | 2026-09-08 | superseded (by 0101) |
| [0089](0089-facet-provenance-declared-surfaces-and-abstention.md) | A facet's computed fact must name its provenance; auth becomes a declared surfaces set, and absence of evidence is not false | 2026-09-09 | accepted (partially superseded by 0093) (errata) |
| [0092](0092-errata-precedence-and-cut-id-reissue.md) | Errata to a frozen record live in ERRATA.md; the four authority documents have one precedence order; a cut id may be re-issued under a hazard table | 2026-09-11 | accepted |
| [0093](0093-kernel-and-declared-facet-source-classes.md) | Kernel projections and the declared surfaces map are their own facet source classes; AC-520 gates only what a spike can score | 2026-09-11 | accepted |
| [0094](0094-phase-1-ships-a-rust-toolchain-adapter-and-self-hosting-means-both-lanes.md) | Phase 1 ships a Rust toolchain adapter beside Node, and self-hosting means the fix lane and one story-lane run on Redkiln-RS itself | 2026-09-11 | accepted (errata) |
| [0095](0095-closed-output-schema-rule-and-the-steperror-table.md) | One closed-output schema rule — no oneOf, anyOf only by transform, union-typed parameters counted per exported schema — and StepError enumerated | 2026-09-11 | accepted (partially superseded by 0112) |
| [0096](0096-typed-repro-record-and-a-mutant-that-must-discriminate.md) | A finding's repro is typed — run or mutate, with the mutation and suite named — and a surviving mutant blocks only when it discriminates | 2026-09-11 | accepted |
| [0097](0097-the-memo-key-is-one-hashed-tuple-and-the-memo-stays-tier-2.md) | The step memo key is SHA-256 over one length-prefixed ordered tuple, the memo stays tier 2, and 0021's per-step memo = false clause is withdrawn with 0043's | 2026-09-11 | accepted |
| [0098](0098-s-gix-a-is-a-read-check-with-a-shell-fallback-and-the-review-clone-is-a-retained-gix-write.md) | S-GIX/a is a read-path check whose no-go falls back to the wrapper; the review clone is a gix write op retained on purpose | 2026-09-11 | accepted |
| [0099](0099-harness-dispatch-blocks-cancel-is-a-token-and-containment-is-per-os.md) | Harness dispatch blocks and returns the record; cancel is a token the composition root owns; containment is a Job Object on Windows and a process group on POSIX; ports are Arc<dyn Port + Send + Sync> | 2026-09-11 | accepted (partially superseded by 0115) |
| [0100](0100-dependency-policy-yaml-reader-0x-pin-list-and-cargo-deny.md) | yaml_serde is the YAML reader for redkiln-migrate only; 0.x crates are exact-pinned by name with named exemptions; cargo-deny enforces the ban list | 2026-09-11 | accepted |
| [0101](0101-negative-controls-gain-a-prior-approval-framing-arm.md) | Negative-control fixtures gain a prior-approval framing arm, cited to the paper that measured it and reported on both halves of the classifier | 2026-09-11 | accepted |
| [0102](0102-toolchain-and-lint-policy-corrected.md) | rust-version equals the pin, a member manifest's [lints] is workspace = true and nothing else, the only in-source override is #[expect], and the nursery and error-handling evidence lines are restated | 2026-09-11 | accepted |
| [0103](0103-predicate-grammar-precedence-literals-null-and-on-false-forms.md) | The predicate grammar gets precedence and associativity, a literal-versus-field rule, quoting, null semantics per quantifier and connective, and a closed on_false form set | 2026-09-11 | accepted |
| [0104](0104-per-ac-mutants-are-author-declared.md) | Per-AC mutants are declared by the AC's author, the flip predicate is AC-334's, the whole-suite comparator bootstraps from the first affected run, and a quarantined survivor still blocks exit | 2026-09-11 | accepted |
| [0105](0105-rk-git-and-rk-store-contracts-as-rules.md) | rk-git and rk-store contracts as rules — the GitOps seam, hooks isolation, trailer plumbing, the tree_hash algorithm, the wrapper's write set, one writer with a bounded busy_timeout, an event-kind table, and retention | 2026-09-11 | accepted (partially superseded by 0116) |
| [0106](0106-mid-graph-gate-cost-ordering-is-unmeasured.md) | A mid-graph gate resumes into the same step; whether it costs less than a stage gate is unmeasured, and placement is not derived from a cost model | 2026-09-11 | accepted |
| [0107](0107-one-idempotent-run-entry-and-no-resume-flag.md) | The CLI has one idempotent run entry point, no resume flag or verb, an idempotent init, and a committed argument-surface snapshot with a name denylist | 2026-09-11 | accepted |
| [0108](0108-structured-output-is-a-configuration-time-check.md) | StructuredOutput is a configuration-time check — a schema-carrying dispatch whose tool set lacks it is refused before spawn, and nothing parses a schema out of prose | 2026-09-11 | accepted |
| [0109](0109-reconcile-at-every-entry-two-tiers-and-three-trailer-axes.md) | reconcile() runs at every entry and nothing skips it; state is two tiers split by the deleted-database test; trailers carry three axes | 2026-09-11 | accepted |
| [0110](0110-headroom-is-a-batch-delta-in-ticks-on-visible-windows.md) | Headroom is a batch delta over a named dispatch set in integer ticks, throttled on the visible unified windows with a two-tick margin, never synthesised per family and never credited from a cache metric | 2026-09-11 | accepted |
| [0111](0111-s-p7b-re-run-must-change-the-criterion-not-only-n.md) | The S-P7B re-run must change the criterion, not only n — P7's criterion 1 had zero power at n = 3 and ten draws per cell are still powerless for the effect it registers | 2026-09-11 | accepted |
| [0112](0112-steperror-gains-platform-unsupported-and-overage.md) | StepError gains Platform, Unsupported and Overage — the three variants thirteen criteria and design §3.6 already named — making the closed set eleven | 2026-09-11 | accepted |
| [0113](0113-ttl-verification-is-read-from-the-nested-cache-creation-object-and-s-ttl1-narrows-to-the-subagent-bucket.md) | TTL verification is read from the nested cache_creation object — a -p run with ephemeral_1h > 0 and ephemeral_5m = 0 is verified 1 h — and S-TTL1 narrows to the subagent bucket and the billing-mode confound | 2026-09-11 | accepted |
| [0114](0114-the-executed-once-gate-moves-to-extended-to-shipped-promotion-and-the-v0-pack-tags-on-lint-parse-and-a-visible-ledger.md) | The executed-once gate moves to extended→shipped promotion; the v0 pack tags on P6 lint, the F42 parse and a per-activity executed-once ledger the doctor reports | 2026-09-11 | accepted |
| [0115](0115-the-event-store-port-is-held-behind-interior-mutability.md) | The EventStore port is held behind interior mutability; ADR-0099 clause 4 is narrowed to the four ports whose methods take &self | 2026-09-12 | accepted |
| [0116](0116-the-event-kind-table-gains-artifact-write.md) | The event-kind table gains artifact.write — the producing half of artifact.read, so a run's outputs are a query rather than a replay | 2026-09-12 | accepted |

## Supersessions

Thirty-one supersession links exist in this set, from twenty-seven superseding records — `0075` supersedes two predecessors, `0103` four, `0084` is superseded in part by two, `0095` in turn by `0112`, `0078` and `0027` each by one later-the-same-day record, and `0099` and `0105` each in part by one record of 2026-09-12 (`0115`, `0116`). Three links are full (0004→0074, 0041→0064, 0088→0101); twenty-six are partial and leave their predecessor `accepted`.

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
| [0085](0085-documentation-and-traceability-conventions.md) | [0092](0092-errata-precedence-and-cut-id-reissue.md) | Partial | Withdrawn: rule 7's "never reused" in its unqualified form (a cut id may be re-issued under a hazard table) and the silence on how a frozen record's evidence lines are corrected. Added: the errata convention (ERRATA.md) and the precedence order accepted ADRs > AC register > SM register > spike files > runbook. Rules 1–6, 8 and 9 stand — 0085 stays `accepted`. |
| [0089](0089-facet-provenance-declared-surfaces-and-abstention.md) | [0093](0093-kernel-and-declared-facet-source-classes.md) | Partial | Withdrawn: decision 1's implicit claim that every non-human, non-LLM facet is `derived`. `source` gains `kernel` (`stage`, `rigor`, `role`, `cost`) and `declared` (`surfaces`); six facets remain `derived`, each with a record at the pin; S-BIN's gate is scoped to `ChangeSummary`; `bisect` is ceilinged. Decisions 1–3 otherwise stand — 0089 stays `accepted`. |
| [0003](0003-five-named-ports-no-postgres.md) (D3) | [0094](0094-phase-1-ships-a-rust-toolchain-adapter-and-self-hosting-means-both-lanes.md) | Partial | Withdrawn: the `Toolchain` row's Phase-1 column (Node only) and the unqualified "named real adapter" column. Phase 1 ships Node **and Rust**; the column is read per design §10.2's phase. Also settles 0002:53 against 0011:47: self-hosting is the fix lane **and** one story-lane run (AC-521). Five ports, no Postgres and git-not-a-port stand — 0003 stays `accepted`. |
| [0084](0084-dependency-policy.md) | [0095](0095-closed-output-schema-rule-and-the-steperror-table.md) | Partial | Withdrawn: the golden-snapshot clause's rule set ("no `oneOf` anywhere" beside a `tag = "kind"` that "renders a single object") and the :117 evidence line. One rule: no `oneOf`; `anyOf` by post-derive transform; no nested unions; ≤ 16 union-typed and ≤ 24 optional per exported schema (the vendor's per-request limits, scope stated); `StepError` enumerated. The `gix` pin, `cargo audit`, the rusqlite and `jsonschema` clauses stand — 0084 stays `accepted`. |
| [0084](0084-dependency-policy.md) | [0100](0100-dependency-policy-yaml-reader-0x-pin-list-and-cargo-deny.md) | Partial | Withdrawn: the implicit range clause. `yaml_serde` is named for `redkiln-migrate` only; 0.x crates are exact-pinned by name (`gix`, `jsonschema`, `rusqlite`, `yaml_serde`) with `chrono` and `sha2` exempted and said why; `cargo-deny` and a `deny.toml` enforce ADR-0086's bans; the audit job runs `--deny unmaintained`. |
| [0074](0074-review-gate-reviewer-proposes-machine-confirms.md) | [0096](0096-typed-repro-record-and-a-mutant-that-must-discriminate.md) | Partial | Withdrawn: clause 1's untyped `repro { command, observed, expected_when_met }` and clause 2's `mutation-survived` definition. The repro is typed (`kind: run` or `mutate`; `mutation { patch, suite }`; a machine-readable `expected`); prose-only commands are rejected at ingest; a surviving mutant blocks only if it discriminates. Clauses 3–8 stand — 0074 stays `accepted`. |
| [0021](0021-loop-aware-memo-key.md) (D21) | [0097](0097-the-memo-key-is-one-hashed-tuple-and-the-memo-stays-tier-2.md) | Partial | Withdrawn: the per-step `memo = false` sentence (the clause 0059 withdrew from 0043; liveness is the verb's `live = true`). The memo key is pinned as SHA-256 over a length-prefixed eight-component tuple; the memo stays tier 2 and AC-161/164/187 narrow. Per-step keys and the iteration counter stand — 0021 stays `accepted`. |
| [0077](0077-gix-read-path-only-git-write-path-shells-out.md) | [0098](0098-s-gix-a-is-a-read-check-with-a-shell-fallback-and-the-review-clone-is-a-retained-gix-write.md) | Partial | Withdrawn: S-GIX/a's row (a write question that "gates only the narrowing") and the review clone's placement under the read path. S-GIX/a is a read check whose no-go moves common-dir resolution to the wrapper; the clone is a retained gix write covered by the audit job; S-GIX/b's read half is discharged; S-GIX/c is added. The hybrid and the one-wrapper rule stand — 0077 stays `accepted`. |
| [0076](0076-port-traits-stay-sync-async-confined-to-adapters.md) | [0099](0099-harness-dispatch-blocks-cancel-is-a-token-and-containment-is-per-os.md) | Partial | Withdrawn: the cancel clause's "a handle minted when the operation started". `dispatch` blocks and returns `DispatchOutcome`; cancel is a `CancelToken` the composition root owns; containment is a Job Object on Windows and a process group on POSIX; ports are `Arc<dyn Port + Send + Sync>`. Sync traits and adapter-owned runtimes stand — 0076 stays `accepted`. |
| [0088](0088-negative-controls-gain-a-confirmation-bias-arm.md) | [0101](0101-negative-controls-gain-a-prior-approval-framing-arm.md) | **Full** | The arm survives, corrected: the paper is arXiv 2603.18740 ("Contextual Bias"); the effect is 16–93 percentage points with false positives collapsing (a whole-classifier shift), not "barely moving"; both halves are reported; the symmetric arm is registered as S-SUBTLE-SYM rather than rejected on a false premise. Because the mechanism sits in the decision's rationale, 0088 moves to **`superseded`**. |
| [0083](0083-rust-workspace-toolchain-and-lint-policy.md) | [0102](0102-toolchain-and-lint-policy-corrected.md) | Partial | Withdrawn: the per-crate `[lints]`-override sentence (Cargo rejects it), the `rust-version`-not-used sentence, `#[allow]` as the in-source override, and three misquoted evidence lines. `rust-version` equals the pin; `[lints] workspace = true` only; `#[expect]`; group entries carry `priority`. The pin rule, the pedantic subset and the `set_var` ban stand — 0083 stays `accepted`. |
| [0032](0032-absent-inputs-are-null.md) (SCHEMA-2) | [0103](0103-predicate-grammar-precedence-literals-null-and-on-false-forms.md) | Partial | Withdrawn: "quantifiers over `null` are vacuously true" for `any` (now false; `all` and `none` stay true). |
| [0033](0033-normative-predicate-grammar.md) (SCHEMA-3) | [0103](0103-predicate-grammar-precedence-literals-null-and-on-false-forms.md) | Partial | Added, not withdrawn: precedence (`not` > comparison > `and` > `or`, left-associative), the literal-versus-field rule, quoting and escapes, the quantifier forms. |
| [0048](0048-predicate-grammar-gains-parens-and-quoted-facet-values.md) (SCHEMA-18) | [0103](0103-predicate-grammar-precedence-literals-null-and-on-false-forms.md) | Partial | Withdrawn: the deliberate absence of precedence; the quote set is fixed; facet names are `[a-z_][a-z0-9_]*`. |
| [0069](0069-null-ordering-operators-are-lint-errors.md) (SCHEMA-39) | [0103](0103-predicate-grammar-precedence-literals-null-and-on-false-forms.md) | Partial | Added: `null == v` is `null`; three-valued connectives; the one narrowing pattern; the order lint → gate → `Unsatisfiable`. Ordering against `null` stays a lint error. |
| [0087](0087-mid-graph-gate-resumes-into-the-same-step.md) | [0106](0106-mid-graph-gate-cost-ordering-is-unmeasured.md) | Partial | Withdrawn: the title's and clause 3's "costs less than a stage gate — measured, it is cheaper" and the matching forbids bullet (F38 divided unlike counters). The cost ordering is UNMEASURED; placement is not derived from a cost model; P8-B's unit is fixed. Clauses 1–2 and the fresh-process rule stand — 0087 stays `accepted`. |
| [0080](0080-quality-block-policy-predicate-is-not-a-terminal-condition.md) | [0111](0111-s-p7b-re-run-must-change-the-criterion-not-only-n.md) | Partial | Withdrawn: decision 5 ("ten draws per cell settles the default" — powerless for a 0 vs 0.33 effect; n = 3 had zero power). S-P7B's re-run changes the criterion (paired, or pooled-difference with a stated MDE). Decisions 1–4 stand — 0080 stays `accepted`. |
| [0078](0078-cache-rule-is-split-both-ttls-set-scope-corrected.md) | [0113](0113-ttl-verification-is-read-from-the-nested-cache-creation-object-and-s-ttl1-narrows-to-the-subagent-bucket.md) | Partial | Withdrawn: clause (iv)'s premise that F3 measured the `ephemeral_*` fields absent, clause (ii)'s "no run can currently discharge" reading, and the forbids line "until S-TTL1 runs" — the nested `cache_creation` object is on 950/955 records and 621 creations were on the 1 h tier. The two-axis rule, both-variables rule, scope restatement and billing-mode confound stand — 0078 stays `accepted`. |
| [0027](0027-curated-catalog-with-unshipped-extended-file.md) | [0114](0114-the-executed-once-gate-moves-to-extended-to-shipped-promotion-and-the-v0-pack-tags-on-lint-parse-and-a-visible-ledger.md) | Partial | Withdrawn: the second tagging condition ("executed at least once before the pack is tagged") and its forbids line — unmeetable with 101 of 102 activities unrun before the kernel exists. The condition moves to extended→shipped promotion; the tag rests on P6 lint and the F42 parse; a per-activity executed-once ledger makes rot visible. The curated split, the P6-lint condition and AC-110/AC-114 stand — 0027 stays `accepted`. |
| [0099](0099-harness-dispatch-blocks-cancel-is-a-token-and-containment-is-per-os.md) | [0115](0115-the-event-store-port-is-held-behind-interior-mutability.md) | Partial | Withdrawn: clause 4's "held as `Arc<dyn Port + Send + Sync>` … a bare trait object" for the **EventStore port only**, which cannot produce the `&mut dyn EventStore` clause 1 takes. Clause 1, the other four ports and the newtype prohibition stand. |
| [0105](0105-rk-git-and-rk-store-contracts-as-rules.md) | [0116](0116-the-event-kind-table-gains-artifact-write.md) | Partial | Withdrawn: clause 7's event-kind table as a closed set without a write-side artifact kind. Every other clause stands; the table gains `artifact.write` and loses nothing. |
| [0095](0095-closed-output-schema-rule-and-the-steperror-table.md) | [0112](0112-steperror-gains-platform-unsupported-and-overage.md) | Partial | Withdrawn: decision 2's table at eight rows and its "any further variant is a successor" as applied to `Platform`, `Unsupported` and `Overage` — three variants thirteen criteria already named. The set is eleven; the schema rule and S-SCHEMA-CAP stand — 0095 stays `accepted`. |

Reference relations that are deliberately **not** supersessions:

- `0023` (PC1) and `0020` (D20) are the **same decision recorded in two homes** — the rigor enum as the architecture proposal states it and as the process-catalog proposal states it. They are `related`. Neither withdraws anything from the other.
- `0028` (PC6), `0029` (PC7) and `0022` (D22) are `related`: encapsulated activity graphs, the technology-pack contribution contract, and the D-series record of both. No withdrawal among them.
- `0054` (SCHEMA-24) **extends** `0040`'s closed input-source grammar; neither record names the other in `related` — a declared relation only here and in ERRATA E-0012-1.
- `0101` (formerly `0088`) **elaborates** `0074` clause 7 and adds the unrun/sizing obligations; `0101` names `0074` in `related`, and 0074 is frozen.
- `0105` and `0109` are **companions** to `0018` — rules it left open, recorded beside it; both name `0018` in `related`.
- `0089` **generalises** `0067` (SCHEMA-37) rather than superseding it. `0067` already emits `null` under rule `R-recent-deploy-feed-absent` when the `toolchain:deploy_log` feed is absent; `0089` extends that same abstention from one derived boolean to all of them, so there is one mechanism and not two. Nothing of `0067` is withdrawn — its facet, its 24 h anchor and its `wants_facet` prohibition all stand — so the relation is `related`, recorded one way on `0089` because `0067` is `accepted` and therefore frozen.

## Where these live eventually

At self-hosting these records migrate to `.kb/decisions/<NNNN>-<slug>.md` under `KbFrontmatter`, joining the `.kb` intake → adjudicate → integrate contract (`2026-09-05-architecture-proposal.md` §4.8; `research/2026-09-03-workflow-model.md` §4). The rule crosses with them unchanged: **ADRs are immutable — supersede, never edit.** `.kb` writes are agent-autonomous within that contract, and ADR **status transitions are the one human gate**. The `consumers` key, `[]` throughout this set, is the hook the projection fills in once atoms are injected into activities by declared inclusion rules.

Until then this folder is the canonical location and the legacy-id map above is the migration key.

## Open items

**Two deliberate gaps, no duplicates, no unresolvable defect.** The set is 112 files over `0001`–`0114`: `0090` and `0091` are reserved numbers the runbook's Step 4.4 and Step 6.1 will fill (a gap by allocation, not by loss), and there are no duplicate numbers; every `legacy_id` in `D1`–`D22`, `PC1`–`PC8`, `SCHEMA-1`–`SCHEMA-43` appears exactly once; `0074` and `0075` carry the revision keys `D4 rev.4` and `D5/D6 rev.4` rather than `none`, because each is the revision-4 rewrite of the D-series decision it supersedes, and `0076`–`0089` carry `legacy_id: none`; every id reference resolves; `consumers` is `[]` everywhere; the supersession graph is acyclic; every record carries all six sections and a `**What this forbids**` block; and exactly four records are not at `accepted` — `0041`, `0004` and `0088`, the three targets of a full supersession, at `superseded`, and `0079` at **`proposed`**, which is correct because the axis-source question it records is not decided by anything measured so far and 0079 decides nothing in its place.

**Open questions carried at `proposed` or explicitly unresolved inside an accepted record.** `0079` is the only `proposed` record: Phase 1 pack authoring over the LLM-emitted facets is blocked and the record forbids being amended into a decision. **Its blocking condition has moved but has not lifted.** AC-071's five clauses include an arm-D panel scored, and that clause is now discharged — arm D was scored on 2026-09-09 against 140 panel-adjudicated truth cells [measured, F39] — while the clause that actually holds the block is untouched: modal required-set identity was measured at 0.25 / 0.15 / 0.10 / 0.10 against an 0.80 line [measured, F36]. F39 measures a *derived* context vector; 0079 asks about an *LLM-emitted* one, so it contributes to that record without deciding it. Five accepted records carry a named open item rather than a resolution: `0078` registers spike **S-TTL1** against the F3-versus-documentation contradiction over the `ephemeral_1h`/`ephemeral_5m` fields; `0077` registers **S-GIX** against gix's undocumented trailer-write support; `0080` leaves the quality shorthand's default explicitly unset pending a re-run at n = 10 per cell and a settled severity vocabulary; `0088` registers an unrun confirmation-bias arm and forbids claiming the gate is guarded against false approval until it runs; and `0089` registers **S-FACET** against the unscored `surfaces` facet and leaves the source-asymmetry question (`commits` beats `docs`; `union` is worse than `commits` alone on two facets) explicitly undecided.

Three items are recorded as notes rather than defects:

- **Record-granularity supersession is coarser than the sources.** Both `0041`/`0064` and `0043`/`0059` supersede in substance only part of the predecessor, and `0041`'s source scopes its supersession to pure-composition cases. The link is nevertheless recorded whole-record, per the rule above, with the scope written into both bodies. Anyone acting on `0041` must read `0064`'s Decision section for what actually survives.
- **Three `related` edges point at a superseded record.** `0080`, `0082` and `0088` name `0004` in `related`; `0004` is superseded in full by `0074`, and the three bodies invoke it for the one rule that survives inside 0074 — findings are data, Rust computes the block (0080 :61, 0082 :61). Read those edges as edges to `0074` (PIA-037). The frontmatter is frozen, so the note lives here.
- **`related` is directional.** It is not maintained reciprocally across the set (227 of the 293 `related` edges point one way only), and this is convention, not an error — a record names what it drew on. Do not infer from a missing back-link that a relation was dropped.
- **Errata live in [`ERRATA.md`](ERRATA.md)** (ADR-0092): evidence-line corrections to 0001, 0012, 0019, 0056, 0067, 0074, 0075, 0077, 0079, 0080, 0081, 0082, 0083, 0084, 0085, 0086, 0087 and 0089, each dated, sourced and tagged, none moving a Decision. The 0067 note that used to live here is E-0067-1 there; the paragraph below is retained as the precedent it was.
- **One clause of `0067`'s Evidence section has been overtaken, and the correction lived here because the record was frozen** (now E-0067-1). `0067` says of the schema row's "P1 arm D measures it": "a forward reference only. No measurement result is given, and P1 arm D has not been read out (its blind panel is unfilled), so nothing here rests on a measured effect." Arm D **was** read out, on 2026-09-09 [measured, F39], so that clause no longer holds. Everything the sentence was doing still does. The parenthetical remains literally true — the pre-registered blind panel file was never filled; METHOD-P amendment 9 replaced that oracle with a deterministic one on the mechanical facets and an adjudicated model panel on the semantic ones, and the file is retained unfilled [documented, METHOD-P §0 amendment 9]. And `recent_deploy` does **not** appear in F39's precision table: the `toolchain:deploy_log` feed does not exist at the corpus pin, so arm D emitted the facet as `null` under `R-recent-deploy-feed-absent` instead of scoring it [measured, F39; `spikes/p1/derive-facets.py`]. The forward reference is therefore still unfulfilled, and `0067`'s Decision continues to rest on no measured effect — which is what the sentence was disclaiming, not using.

One cosmetic inconsistency was left as-is because fixing it would mean editing accepted records for no normative gain: the required forbids block is punctuated three ways across the set — `**What this forbids**` (100), `**What this forbids**:` (11), `**What this forbids:**` (3), which is all 114. The canonical form for new records is `**What this forbids**`.
