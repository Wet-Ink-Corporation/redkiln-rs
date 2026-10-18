---
id: "0200"
title: "**The two human roots are supplied through two pack-declared questions, each bound to the step-4 gate on its kind** (`s8j`'s human-root record; D21-41 f; D21-43's integration call; ADR-0129 decisions 3 and 4; ADR-0177 decision 7): `root.bug-report` and `root.intake-record` are entries in `.redkiln/processes/questions.toml`, each with one option `attach` whose `attaches` names its kind (`BugReport`, `IntakeRecord`); selection's step-4 `Unsatisfiable { reason: NoAdmissibleProducer }` on a `source = \"human\"` kind opens a gate of kind `human_root:<Kind>`, which each question answers through the loader's `binds`; each class carries a fail-closed `[[gate_expiry]]` row in both packs and is a `&str` constant on a code line; a root file a run attaches is committed at `.redkiln/intake/<item-id>.<Kind>.md`, a directory no reader scans. Supersedes nothing"
status: proposed
date: 2026-10-18
legacy_id: none
source: runbook/phase-1/s8j-run-preparation.md §8j.2 (the record's three decisions) and §8j.10 Checkpoint 8j (3); spikes/plans/s8j-build-plan-2026-10-16.md Round B (S8J-B1-ROOT-QUESTIONS) and Round C (S8J-C2-ROOT-QUESTIONS); spikes/DECISIONS-2026-09-21.md D21-41 (f), D21-42 21a, D21-43's integration calls, D21-110 ruling 2; references/adr/0063, 0116 decision 1, 0129 decisions 3 and 4, 0145, 0173, 0177 decisions 7 and 8, 0189 decisions 8 and 9, 0198 decisions 6 and 7; references/2026-09-08-acceptance-criteria.md AC-054, AC-276, AC-410; references/2026-09-08-architecture-design.md §5's `artifact.write` row and its R-48 note (RT-59)
supersedes: []
superseded_by: []
related: ["0063", "0116", "0129", "0145", "0173", "0177", "0183", "0189", "0198", "0201"]
evidence: []
consumers: []
---

## Context and problem

**Each lane's first activity needs a root no activity produces.** `.redkiln/artifacts.toml` registers `BugReport`
and `IntakeRecord` as `source = "human"` [measured 2026-10-18]. The fix lane's analysis activities `minimal-repro`
and `exploitability-analysis` both carry `requires = ["BugReport"]`, and three of the five Spec providers
(`bdd-scenarios`, `use-case-spec`, `user-story-ac`) require `IntakeRecord`; `bdd-scenarios` is the one the story
pack's mandatory specification stage selects (`s8j` §8j.2, measured there). D21-41 (f) decided the route once for
both lanes: *"IntakeRecord and BugReport stay `source = "human"` (D12 #15), supplied through a pack-declared gate
question whose option attaches the kind (ADR-0129 d3/d4); this is decided once for both lanes."*

**What exists, measured at the stage's entry commit `3fa69db`:**

- The loader and its file are `s8d`'s, under ADR-0177 decision 7: `.redkiln/processes/questions.toml`, each
  `[question."<id>"]` declaring `text`, a non-empty `options` list (an id, or `{ id, attaches = <Kind> }`) and an
  optional `binds`, *"further gate kinds it answers"*. The shipped file holds eleven entries; none attaches anything.
- Selection's step 4 reports a required kind with no admissible producer as `Unsatisfiable { kind,
  failed_predicates, rigor, reason }`, `reason` one of `no_admissible_producer` and `forbidden_stage`
  (`crates/rk-kernel/src/select/step4.rs`), and `Selection::journal` announces it as `gate_opened { reason:
  "unsatisfiable", unsatisfiable: {…} }` (`select.rs`). **No durable gate is opened for it**: `fsm::next_action`'s
  step 2 mints a `GateRecord` only for `selection.gates()`, the step-2 `unknown_facet` abstentions, under the kind
  `"<reason>:<axis>"` (`workflow/fsm.rs`, the module header's first reading). So AC-054's *"opens a gate"* is today
  an announcement only, and an item whose root is absent can be neither attached nor run.
- `answer` stores an admitted attachment as a human-supplied root, one `artifact.write` row each, carrying
  `attached_path` (ADR-0189 decision 8; `verbs/answer.rs`'s `store_attachments`), and the kernel refuses an option
  that declares `attaches` and is answered without it (ADR-0129 decision 4).
- `run`'s load-time lint refuses a run whose pack lacks a `[[gate_expiry]]` row for a reachable class, and refuses a
  row whose class is unreachable (ADR-0198 decision 7; `lint_gate_expiry` in `verbs/run.rs`). Its reachable set is
  the catalog's `gate`-step questions, `unknown_facet:<axis>`, `escalation` and the constant table
  `MINTED_GATE_KINDS`.

**Three things are this record's** (`s8j` §8j.2, *"The record … decides three things"*): the two question ids and
their options in the spelling ADR-0177 fixes; the gate-kind name the step-4 gate binds under; and the path convention
for the committed root files.

## Decision drivers

- **One loader, one file** (D21-43: *"The loader and `.redkiln/processes/` are **s8d**'s. The pack-declared human-root
  questions are **s8j**'s."*). This record adds entries in ADR-0177 decision 7's schema and no key.
- **ADR-0129 decision 4 both ways:** an attachment the option admits is stored as a root, and an option that declares
  `attaches` cannot be closed without it. A question that admitted the wrong kind would store the wrong root.
- **Fail closed on expiry** (ADR-0198 decision 6; D21-42 21a): every class the system can open has a row, signed row
  by row by the operator (AC-276).
- **A minted spelling is a `&str` constant on a code line** (`s8h` §8h.0 item 7), so Checkpoint 8h's
  `MINTED_CLASSES` reads it where it is minted.
- **The record decides only what C2 builds and its three `processes::` tests read** (the plan's Round C row), and
  nothing the §8j.4 proofs are there to measure.

## Considered options

### Option — two questions, one per kind, each bound to a `human_root:<Kind>` gate through `binds` (taken)

Everything under `## Decision`. A gate kind binds exactly one question (ADR-0177 decision 7 refuses a kind bound
twice), so one question per kind gives each gate an option set that can admit only its own kind.

### Option — one question with two options, each attaching one kind

`[question."root"]` with options `bug-report` (attaches `BugReport`) and `intake-record` (attaches `IntakeRecord`),
bound to both gate kinds. **Not taken.** The `human_root:BugReport` gate would then admit the `intake-record` option
and store an `IntakeRecord` root against a gate that asked for a `BugReport`; ADR-0129 decision 4 validates the
attachment against the option, not against the gate's kind, so nothing would refuse it.

### Option — answer the root gate with the default question

**Not taken.** The default question's two options attach nothing (ADR-0129 decision 3), so no answer to it can supply
the root, and the gate could only be answered without the thing it exists to collect.

### Option — spell the gate kind from step 4's reason: `no_admissible_producer:<Kind>` or `unsatisfiable:<Kind>`

The pattern of `unknown_facet:<axis>`, where the prefix is the reason. **Not taken.** Step 4's reason covers every
kind without an admissible producer, activity-sourced ones included, and an answer cannot supply an activity-sourced
kind. A prefix that named the reason would make every such kind a reachable gate class owing an expiry row and a
question, for gates no answer can satisfy. `human_root` names the one case an answer resolves, and the draft spelling
`s8j` §8j.2 item 4 gave it stands.

### Option — read the root from a committed file at `run`, with no gate

`run` would read `.redkiln/intake/<item>.<Kind>.md` when present. **Not taken.** D21-41 (f) and ADR-0129 decision 4
route a human root through an answered gate and `answer`'s `artifact.write`; a file `run` reads on its own is a second
producer of a human kind and an unrecorded input to the run.

## Decision

**1. The two questions**, entries twelve and thirteen of `.redkiln/processes/questions.toml`, in ADR-0177 decision
7's schema, under a dated comment:

```toml
[question."root.bug-report"]
text = "No activity produces a BugReport: a person supplies it. Attach the bug report the work starts from."
options = [{ id = "attach", attaches = "BugReport" }]
binds = ["human_root:BugReport"]

[question."root.intake-record"]
text = "No activity produces an IntakeRecord: a person supplies it. Attach the intake record the work starts from."
options = [{ id = "attach", attaches = "IntakeRecord" }]
binds = ["human_root:IntakeRecord"]
```

- **The ids** are §8j.2's drafts, kept: `root.bug-report` and `root.intake-record`.
- **Each has one option, `attach`**, whose `attaches` names exactly its kind. The option is answered as `redkiln answer
  <item> attach --attach <Kind>=<path> --explain-sufficed <yes|no> --toml-opened <yes|no>` (ADR-0129 decision 1), and
  in `s8` through `tools/live-run.sh answer` with the same arguments.
- **The text** says what the attachment is for and nothing about any item (§8j.2 item 1). The texts above are this
  record's.
- **No other question attaches either kind**, and every `source = "human"` kind a selected activity of either lane
  requires without `?` is attached by exactly one question (the test
  `processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question` checks both directions).

**2. The gate kind the step-4 gate binds under is `human_root:<Kind>`**, where `<Kind>` is the kind's registry name
verbatim. For the two Phase-1 roots that is `human_root:BugReport` and `human_root:IntakeRecord`.

- **When it opens.** For every `Unsatisfiable` selection's step 4 reports with `reason = NoAdmissibleProducer` on a kind
  the artifact registry declares `source = "human"`, the kernel opens a durable gate of that kind (AC-054's *"opens a
  gate"*), through `fsm::next_action`'s step 2 beside the `unknown_facet` gates, its token derived by
  `GateRecord::derive_reach` as every gate's is, in the order `Selection::unsatisfiable` is sorted (by kind). The kind's
  source is read from the registry the selection already holds; nothing outside the kernel decides whether a gate opens.
- **Re-opening follows the one rule step 2 applies to every gate.** A kind not yet reached opens; an answered gate that
  is not void is not opened again; a void answer (AC-039, the tree moved) re-opens fresh (AC-252). An answered gate
  whose root the next invocation still does not classify `Present` is therefore not re-opened, and that outcome is
  what §8j.4's proofs measure (§8j.2 item 3); it is not decided here.
- **Not widened.** An `Unsatisfiable` on an activity-sourced kind, or with `reason = ForbiddenStage`, keeps today's
  announcement and opens no `human_root` gate.
- **The binding** is the loader's `binds` (ADR-0177 decision 7), which is `s8d`'s `bind` route; no code binds the kind
  by name. The question's own id is also bound as a kind by the loader, but no catalog step and no kernel path opens a
  gate of kind `root.bug-report` or `root.intake-record`, so neither id is a reachable gate class.
- **After the answer** the attachment is stored as a human-supplied root under ADR-0129 decision 4, one
  `artifact.write` row, by the code that exists (`store_attachments`). This record changes nothing in that path.

**3. Each class is fail-closed and minted as a constant.** `human_root:BugReport` and `human_root:IntakeRecord` are
each a `&str` constant on a code line under `crates/*/src` (or a member of a constant `&[&str]` table there), join
`rk-cli`'s `MINTED_GATE_KINDS` table so `run`'s load-time lint counts them reachable in every pack, and each takes one
`[[gate_expiry]]` row in **both** `.redkiln/packs/bug-standard.toml` and `.redkiln/packs/story.toml`, `action =
"fail"`, in the change that mints them. The rows are drafts until the operator signs them row by row (D21-42 21a;
AC-276); this record signs nothing. Their spellings join Checkpoint 8h's `MINTED_CLASSES` line by the dated edit
`s8h` §8h.10 names, and Checkpoint 8j (3)'s `S8J_GATE_CLASSES` already carries them.

**4. The path convention.** A human root a real run attaches is committed at

`.redkiln/intake/<item-id>.<Kind>.md`

where `<item-id>` is the item record's `id` and `<Kind>` the registry kind verbatim: in Phase 1,
`.redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md` and `.redkiln/intake/<fix-item>.BugReport.md` once
§8j.5's pick is made.

- **Nothing reads the directory.** No loader, lint or reader scans `.redkiln/intake/`; a file there reaches a run only
  as the path an operator names to `--attach`, and the stored root is the `artifact.write` row, not the file. The
  `attached_path` carrier records the path relative to the checkout (ADR-0189 decision 8).
- **The directory also holds §8j.5's repro pair** (`<fix-item>.repro.toml`, `<fix-item>.repro.rs`), which Checkpoint
  8j (4) reads and no run reads. They are not roots, and their names cannot collide with a root's, because no kind is
  named `repro`.
- Checkpoint 8j's whole-suite line is what proves the new directory disturbs nothing (§8j.2).

**5. What this record does not decide.**

- **RT-59 is not this record's.** It was settled by R-48 on 2026-09-20 by a dated note on design §5's `artifact.write`
  row: *"every activity output, **and `redkiln answer` for an admitted attachment**"*. The comment in
  `crates/rk-cli/src/verbs/answer.rs` that still reads *"RT-59: that `answer` is this row's producer is ROUTED, not
  decided"* is stale, and C2 corrects it by a dated comment-only edit.
- **The reader's and resolver's root arm** (ADR-0189 decision 9's last sentence; D21-110 ruling 2) is not decided
  here. A `requires` presence check is all this record's gate needs; whether a lane reads a root's bytes through an
  `artifact:` input is S8J-A1's measurement and the conditional record's.
- **Whether a re-invocation classifies the attached root `Present`** is measured by §8j.4, not decided.

**6. What this record supersedes: nothing.** It adds entries to the file ADR-0177 decision 7 fixes, in its schema, and
opens AC-054's gate on the kind D21-41 (f) names. Carrying out ADR-0129 decision 3's *"Pack-declared questions … are
Step 7e's"* at a later stage is execution order, not a moved Decision sentence.

## Consequences

**Positive**

- Each lane's root is supplied by the route D21-41 (f) chose, once for both lanes, through the one loader and the one
  `answer` path, and an option can admit only the kind its gate asked for.
- AC-054's gate becomes a durable, answerable gate for the one case an answer can satisfy, and every other
  `Unsatisfiable` is unchanged.
- Both classes are fail-closed from the change that mints them, and the load-time lint refuses a pack that lacks a row.

**Negative**

- **Two more gate classes need signed rows** before any run that reaches them; until the operator signs them, a lapsed
  root gate fails the item, which is the intended fail-closed cost.
- **`human_root:<Kind>` is parametric but only two spellings are minted.** A later `source = "human"` kind required by
  a selected activity opens a `human_root:<Kind>` gate with no question and no row: the lint refuses the run at load
  for the missing row, and the
  `processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question` test fails for the missing question.
  Both are loud, and the new kind's record adds the entry, the constant and the rows.
- **The root's arrival in a later classification is unmeasured here** (decision 5); if §8j.4 finds an attached root is
  not `Present` on re-invocation, an answered gate does not re-open, and the lane stops for a reason this record names
  in advance.

**What this forbids**

- Forbids a second question file, a second loader, or a key outside ADR-0177 decision 7's schema for these entries.
- Forbids a human-root question whose option attaches a kind other than the one its gate kind names, or more than one
  question attaching the same `source = "human"` kind.
- Forbids answering a `human_root:<Kind>` gate through the default question, or closing it without the attachment.
- Forbids opening a `human_root:` gate on an activity-sourced kind, or on a `ForbiddenStage` `Unsatisfiable`.
- Forbids minting a `human_root:` class without its `&str` constant and its fail-closed row in both packs in the same
  change.
- Forbids any reader, loader or lint scanning `.redkiln/intake/`, and forbids `run` reading a root from that directory
  without an answered gate.

## Evidence and confidence

- [measured 2026-10-18 at `3fa69db`] `artifacts.toml`'s `BugReport` and `IntakeRecord` rows (`source = "human"`); the
  eleven entries of `.redkiln/processes/questions.toml`; `UnsatReason` and `Unsatisfiable::to_json` in
  `crates/rk-kernel/src/select/step4.rs`; `Selection::journal`'s `{ reason: "unsatisfiable" }` announcement in
  `select.rs`; `next_action`'s step 2 iterating `selection.gates()` only, in `workflow/fsm.rs`; `MINTED_GATE_KINDS`,
  `lint_gate_expiry` and `catalog_questions` in `crates/rk-cli/src/verbs/run.rs`; the stale RT-59 comment in
  `verbs/answer.rs`.
- [documented] D21-41 (f), D21-43's integration call, ADR-0129 decisions 3 and 4, ADR-0177 decisions 7 and 8, ADR-0198
  decisions 6 and 7, AC-054, R-48's `artifact.write` note, all quoted above or in `s8j` §8j.2.
- [inferred] that no lane needs a root's bytes through an `artifact:` input, which S8J-A1 measures.
- **Confidence.** High in the spelling and the binding: they are ADR-0177's schema with two entries, and the loader
  already refuses a kind bound twice. Moderate in the gate's opening site: today nothing opens AC-054's gate, so the
  `next_action` step-2 arm is new code that C2's third test (`processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question`)
  and §8j.4's attach proof are the first to exercise.
