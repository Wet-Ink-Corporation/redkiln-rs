---
id: "0177"
title: "**Two review verb rows and the question registry** (`s8d`'s record R5): `shallow_clone` and `verdict_record` are rowed in `.redkiln/registry.toml` on two routes this record adds to `0170` decision 1's closed set, `review_clone` (ADR-0098 decision 2's retained gix clone, performed by the composition root and handed to the kernel as a `Workspace`) and `review_confirm` (`rk_kernel::review::confirm` re-executing in that clone); both are `side_effecting`, so each commits under R1's commit key and is served on re-entry, and neither is a `git_op` row or an ADR-0105 clause-5 wrapper write. `WorkingTree::Declared` now checks out the fetched revision's tree. The pack's questions live in `.redkiln/processes/questions.toml`, loaded by one non-recursive loader into the kernel's `QuestionRegistry` for `run` (an undeclared catalog gate kind refuses by name) and `answer` (never the built-in default). Supersedes nothing"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R5's row), §8d.6, §8d.10 (the `processes::` and `verb_rows::` names; mutant M16) and §8d.12 (9); spikes/DECISIONS-2026-09-21.md D21-41 (d) (*"Route `shallow_clone`/`verdict_record` through verb-registry rows, populate `.redkiln/processes/`"*), D21-43 (*"The loader and `.redkiln/processes/` are **s8d**'s."*), D21-47 (M7), D21-64 ruling 10 (this record's flip condition); references/adr/0063, 0098 (decision 2), 0105 (clause 5), 0129 (decision 3), 0145, 0156 (blocker 2), 0162, 0170 (decision 1), 0174 (R1's `side_effecting` column); spikes/p9/questions.toml and spikes/p9/registry.toml at `021c75a`; crates/rk-kernel/src/workflow/memo.rs (`Route`), crates/rk-kernel/src/runtime/step.rs (`ReviewRoutes`, `ToolRouting::with_review_routes`, the `tool` arm, `review_confirm`, `Runtime::execute_step_output`), crates/rk-kernel/src/runtime/reference.rs (`Scope::output_of_verb`), crates/rk-kernel/src/workflow/question.rs, crates/rk-git/src/clone.rs, crates/rk-cli/src/processes.rs, crates/rk-cli/src/review_pass.rs (`ReviewCloneRoutes`), crates/rk-cli/src/verbs/run.rs (`review_routes`, `bind_nodes`) and crates/rk-cli/src/verbs/answer.rs at `s8d`'s build (package P7)
supersedes: []
superseded_by: []
related: ["0063", "0098", "0105", "0129", "0145", "0156", "0162", "0170", "0174"]
evidence: []
consumers: []
---

## Context and problem

`adversarial-review` runs two `tool` steps whose verbs had no row: `clone` (`run = "toolchain:shallow_clone"`,
`inputs = { sha = "artifact:ChangeSummary.sha" }`, `outputs = "value:Workspace"`) and `record` (`run =
"toolchain:verdict_record"`, `inputs = { findings = "${review.outputs}", override = "${override.answer?}" }`,
`outputs = "value:Verdict"`). A verb with no row refuses by its bare name (ADR-0135 clause 2), so no walk of the review
row could pass either step. ADR-0170 decision 1 closed the route set at four and left the door open by record:
*"A later record may add a route (`s8d`'s, for `verdict_record`). It does so by record, not by editing the match."*

Neither verb fits a route that exists. `s8c`'s `git` route runs *"through the journalled `GitOps` seam"*, and
`rk_git::clone::prepare_review_clone` does not: it calls gix's `prepare_clone_bare` / `prepare_clone` directly, the
one gix write ADR-0098 decision 2 retains (*"The review clone is a gix WRITE operation, retained deliberately"*). And
`verdict_record`'s meaning is ADR-0162's `review::confirm`, which classifies findings by re-execution; no port method
does that. `rk-git` is a concrete crate and not a port (`rk-kernel`'s manifest: *"`rk-ports` declares five ports and no
sixth"*), and `rk-kernel` opens no files, so something has to say which side performs the clone.

The clone also could not serve `confirm`. `prepare_review_clone` called `fetch_only` in **both** arms, so a clone
declared to need a working tree (`WorkingTree::Declared`, ADR-0105 clause 5: *"The **review clone is bare** unless the
review activity declares it needs a working tree"*) got a non-bare repository with nothing checked out, and `confirm`
re-executes each finding's test in a tree.

The question registry had the same shape of gap. ADR-0129 decision 3 built the type in `rk-kernel` and said
*"**Pack-declared questions** (a question in a process pack's TOML with its own options and `attaches`) are Step 7e's,
loaded into the same type."* No loader was built. ADR-0156's blocker 2 measured *"`.redkiln/processes/` is empty"*, and
production `answer` passed `&QuestionRegistry::default()`, so every gate was answerable only as `approve` /
`changes-requested` whatever its activity asked. D21-43 gave the loader and the directory to `s8d` alone.

## Decision drivers

- **A route is added by record** (ADR-0170 decision 1), and the executor's match stays exhaustive with no wildcard.
- **A writing verb is not re-executed on re-entry** (R1's `side_effecting` column): a second clone or a second verdict
  is a repeated write.
- **`rk-kernel` opens no files, and `rk-git` is not a port.** The kernel must be handed the clone's result, not asked to
  make it.
- **A gate is answered with the question its activity asks.** A default that silently stands in for an undeclared
  question is an answer to a different question.
- **One loader.** D21-43: *"The loader and `.redkiln/processes/` are **s8d**'s."* `s8j` and `s8g` load through it.

## Considered options

### Option — Two routes by record, the clone performed by the composition root through a kernel seam (taken)

`review_clone` and `review_confirm` join the closed set; the root binds a `ReviewRoutes` value on the run's
`ToolRouting`; the kernel's arms call it and commit what it returns.

### Option — Route `shallow_clone` on the `git` route

Rejected: the `git` route's contract is the journalled `GitOps` seam and `s8e`'s `git_op` row. A gix clone writes no
such row, so a replay would clone again, and the row would claim a journalling the write never had (`s8d` §8d.6, M7).

### Option — A sixth port for the review clone

Rejected: `rk-ports`' `git_port_is_not_a_port` fixture refuses a sixth port, and ADR-0003's test for a port fails on
the git axis (the manifest's AC-483 note). A seam on the kernel's routing value is enough and adds no port.

### Option — Keep the built-in default question when a gate kind is undeclared

Rejected: `approve` / `changes-requested` is not what `closure.signoff` (`close` / `hold`) or `facet.change` (`text`
/ `binary` / `generated`) asks. An undeclared catalog gate kind is a missing file, refused by name.

## Decision

**1. Two routes are added to ADR-0170 decision 1's closed set, by this record.** `review_clone` (`Route::ReviewClone`)
and `review_confirm` (`Route::ReviewConfirm`). The set is now `port | toolchain | store | git | review_clone |
review_confirm`, matched exhaustively with no wildcard. `.redkiln/registry.toml` rows the two verbs, with the spike's
`outputs` and `timeout` (`spikes/p9/registry.toml`: `shallow_clone = { outputs = "Workspace", live = false, timeout =
"build_timeout" }`, `verdict_record = { outputs = "Verdict", live = false, timeout = "build_timeout" }`):

- `shallow_clone = { outputs = "Workspace", live = false, timeout = "build_timeout", inputs = ["sha"], route =
  "review_clone", side_effecting = true }`;
- `verdict_record = { outputs = "Verdict", live = false, timeout = "build_timeout", inputs = ["findings",
  "override"], route = "review_confirm", side_effecting = true }`.

**2. The composition root performs both routes, through a kernel seam.** `rk_kernel::runtime::step::ReviewRoutes`
(`clone_at(sha)` and `reexecutor(workspace)`) is bound on `ToolRouting` with `with_review_routes`; `rk-cli`'s
`review_pass::ReviewCloneRoutes` is the production value, bound by `run` (`verbs::run::review_routes`). Unbound, both
routes refuse by name before any call, as does a row input the step does not declare. The seam rides `ToolRouting`
rather than a `Runtime` field so that every path that builds a `Runtime` from a routing value (`replay::resume_routed`
included) carries it.

**3. The clone is journalled as a side-effecting step's commit, not as a `git_op` row.** `clone_at` refuses a sha that
is not 40 hex digits, removes a leftover directory at the destination (a killed run's; its commit was never journalled,
or the step would have been served), clones the repository's common directory (a `file://` URL, whose object store the
run-owned branch writes into) to `<common-dir>/redkiln/clones/<item>/<sha>`, and returns the `Workspace` value `{ path,
sha, working_tree: "declared" }`. That value is the step's R1 commit (`side_effecting = true`), so on re-entry or replay
the step is SERVED and no clone is made. **The clone is not an ADR-0105 clause-5 wrapper write**: clause 5 enumerates
the wrapper's writes, and the review clone's only entry there is its *removal*; the clone itself is ADR-0098 decision
2's retained gix write. No supersession is taken, and D21-47's clause-5 rule (the wrapper's writes) is untouched.

**4. `WorkingTree::Declared` checks out the fetched revision's tree**, inside ADR-0098 decision 2's retained write:
`prepare_clone` + `with_revision` + `with_shallow`, then gix's `fetch_then_checkout` and `main_worktree`. The bare arm
(`NotNeeded`) is unchanged (`fetch_only`). The object set is AC-191's either way: one commit, no other branch, tag,
note, stash or reflog entry. The working tree's line terminators are whatever the operator's git configuration makes
a checkout's (a `core.autocrlf`, or a `.gitattributes` in the cloned tree), as `git clone`'s would be.

**5. `verdict_record` is `rk_kernel::review::confirm` over the resolved finding set.** The arm ingests
`inputs.findings` with `Findings::ingest` (an unreadable payload refuses by name, never an empty set, AC-282), resolves
the tier from the selection context's authored stage `rigor` with `review::policy::resolve_review_tier` (AC-302; no
rigor in the context refuses by name), finds the `Workspace` the activation's `review_clone`-routed step committed in
the walk's scope (`Scope::output_of_verb`; none refuses by name, never confirmed against another tree), asks the seam
for the re-executor rooted in that clone (candidate) and at the checkout (reference), and calls `confirm_with` at the
workspace's sha, `review_id` the step's id, no quarantines, and the epoch as `now` (as `review::confirm::confirm`
does). The committed `Verdict` is `{ sha, weight, blocking, advisories, override }`, each finding by its AC-305
identity, `ac` and computed class; `override` is the gate answer carried verbatim, applied by nothing here. The
`verdict` check's `record.blocking.len() == 0` reads it.

**6. The route appends none of `confirm`'s rows.** `review_block`, `advisory_created` and `review_class_drift` stay the
review pass's to write (ADR-0162 decision 6), so there is still one site that appends them.

**7. The question file's home is `.redkiln/processes/questions.toml`, and its schema is closed.** This reconciles D21-41
(d)'s `.redkiln/processes/` with design §4.5's *"`questions.toml` (question ids, closed option sets, `attaches`)"*. The
loader (`rk_cli::processes::load`) reads every `*.toml` directly under `.redkiln/processes/`, sorted, and does not
recurse; an absent directory is the built-in default question alone. Each `[question."<id>"]` declares `text`, a
non-empty `options` list (an id, or `{ id, attaches = <Kind> | [<Kind>…] }`) and optionally `binds`, further gate
kinds it answers. Any other key, a duplicate option, an id declared twice, or a gate kind bound twice is refused by
name. Each question binds its own id as a gate kind, because a catalog `gate` step's kind is its `question`
(ADR-0145). The kernel's `Question` carries the text (`with_text`, `text`), and `QuestionRegistry` gains `binds_kind`
and `question_ids`; it still reads no TOML.

**8. `run` and `answer` use the loaded registry.** `answer` validates against it and never
`QuestionRegistry::default()`; a file that does not load refuses the answer by name before any row. `run` loads it once
and, in the binding pass, refuses by name a selected activity's `gate` step whose question no file declares, before
any dispatch or gate row. The selection's `unknown_facet:*` gates keep the default question unless a file binds their
kind (`question.rs`: *"The default binding is the absence of one"*).

**9. The shipped file holds ten entries.** Nine are every `gate` and `escalate` question the two lanes' selected
activities declare (`s8d` §8d.6's table), with text and options verbatim from `spikes/p9/questions.toml`. The tenth is
N3's `facet.change`, bound to `unknown_facet:change`, options `text`, `binary`, `generated` in `facets.toml`'s order; its
text is the one line not from `spikes/p9`. `spec.ambiguity` is not an entry (N2). The total is not pinned: `s8j`
authors the human-root questions into the same directory, through this loader.

## Consequences

**Positive**

- The review row's `clone` and `record` steps have rows and routes; a lone `verdict_record` step's `Verdict` is
  `confirm`'s decision, and a lone `shallow_clone`'s `Workspace` is a checked-out tree at the sha.
- A re-entered or replayed review serves its clone and its verdict from the journal: no second clone, no second
  re-execution.
- Every gate a lane walks is answered with the question its activity asks, and a gate step whose question is missing is
  a named refusal at load rather than a default answer at the gate.

**Negative**

- **Nothing on a lane reaches these routes yet.** `clone`'s `sha = "artifact:ChangeSummary.sha"` resolves through the
  read seam, which is `UnwiredReadSeam` until `s8f`; so the review row still refuses before the clone.
- **Two confirms, once a walk reaches `record`.** The in-walk verdict (the clone, the checkout as reference) and the
  post-run review pass (the run-owned worktree, a reference worktree at `HEAD`) would both run `confirm` over the same
  finding set, on trees that may differ. Which one decides, and whether the pass defers to the committed `Verdict`, is
  owed to the stage that walks the review row, and this record does not take it.
- **The clone is not removed by the route.** `remove_review_clone` (clause 5's removal) has no caller on the run path;
  a clone stays under `<common-dir>/redkiln/clones/` until the entry step's reconcile (AC-193) or a later stage removes
  it.
- **The step's bound is resolved and not enforced.** The `review_clone` and `review_confirm` arms resolve the step's
  `timeout_secs` in ADR-0170 decision 3's order, and neither the gix clone nor `confirm` takes a bound.
- **The production clone fetches by sha.** A sha that is not a ref tip needs the source to serve it
  (`uploadpack.allowAnySHA1InWant`); the run branch's tip is a ref tip, and nothing else is fetched today.

**What this forbids**

- Forbids a `shallow_clone` on the `git` route, a `git_op` row for the review clone, and a review clone made outside
  ADR-0098 decision 2's gix write.
- Forbids re-executing a `shallow_clone` or a `verdict_record` step whose commit is in the journal.
- Forbids `verdict_record` confirming against a tree other than the clone its activation committed, and forbids a
  review tier from any value but the authored stage rigor.
- Forbids `answer` validating against `QuestionRegistry::default()`, a second question loader, and a recursive read of
  `.redkiln/`.
- Forbids answering a catalog gate step with the built-in default question when no file declares its question.

## Evidence and confidence

- [documented] D21-41 (d), D21-43, ADR-0098 decision 2, ADR-0105 clause 5, ADR-0129 decision 3, ADR-0156 blocker 2 and
  ADR-0170 decision 1, each quoted above or in `source`; `s8d` §8d.6.
- [measured 2026-09-26 at `s8d`'s build, through the no-dispatch helper] each test first shown RED, then GREEN:
  - `processes::the_lanes_nine_gate_questions_and_the_change_question_load_from_redkiln_processes` — RED *"`repro.cannot-reproduce`
    is not declared under .redkiln/processes/"*;
  - `processes::the_answer_verb_validates_against_the_loaded_registry_not_the_default` — RED at exit `Some(0)` for
    `approve` (the default registry applied it);
  - `processes::a_gate_kind_the_registry_does_not_declare_refuses_by_name` — RED at exit `Some(0)` (the run bound the
    undeclared gate step);
  - `verb_rows::shallow_clone_routes_to_the_retained_review_clone` — RED first at *"the shipped registry has a
    `shallow_clone` row"*, then, with the row, at *"no checked-out `f.txt` in the review clone"* (the `fetch_only` arm);
  - `verb_rows::verdict_record_routes_to_review_confirm` — RED at *"the shipped registry has a `verdict_record` row"*.
- [measured, same session] mutant M16 (`answer` handed `QuestionRegistry::default()`) is KILLED by the answer test;
  X1 (the `Declared` arm's checkout removed) by the clone test at the missing file; X2 (the `Verdict`'s block set
  dropped) by the verdict test at *"the reproduced finding BLOCKS at Strict"*. Each ran from a Python driver, each file
  restored and its sha256 verified.
- [measured] `crates/rk-git/src/clone.rs` changed, so Checkpoint 6 (which covers `rk-git`) is provisional until it
  re-runs; `rk-git`'s `review_clone::` tests (AC-191, AC-193, bare-unless-declared) stay green.
- **What this evidence does not license.** No real review ran: every re-execution here is a scripted toolchain's, and
  no lane walk reached `clone` or `record`. This record flips at `s8d`'s close under D21-64 ruling 10, on its own dated
  ledger line, only if the formal Checkpoint 8d passes whole and a refuting round finds no blocker or major.
