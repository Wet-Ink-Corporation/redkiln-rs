# Step 8g — The verify record and the demands restored (Phase 1, part 8, sub-stage g)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8g` | Phase 1 | Checkpoint 8f | Checkpoint 8g | `s8f` | `s8h` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (HEAD `a6349b6`, against `spikes/DECISIONS-2026-09-21.md`'s three entries).
What changed:

- **N23**: the displaced `TestReport` providers are re-homed to the implement stage as test-authoring activities, in
  R1, and the test-design escalation row moves with them. They are **four, measured**, not the five the draft named
  (§8g.2).
- **N24**: AC-139's default is recorded as block, in R2. Both shipped packs ship `verification_required = ["verify"]`
  (§8g.3).
- **Ownership** (D21-43's integration calls): `.redkiln/questions.toml` is dropped. `verify.ac-discriminates` goes
  into `s8d`'s `.redkiln/processes/` registry (§8g.6). AC-133's registry stays here, and `s8h` consumes it (§8g.3).
  R4 cites `s8c`'s record V for the three verb handlers, so the eight handler tests have left this file and its four
  handler mutants are replaced by activity-level ones (§8g.6, §8g.9).
- **The entry table** reads the seven-method count after record V. It gains N25's `setup` verb and widened `build`,
  `s8d`'s loader and `s8d-ii`'s run-worktree root (D21-44; D21-46 R1 moved it out of `s8e`).
- **N25**: the `sca-scan` edit (`unrated == 0`) is `s8b`'s catalog edit, and `setup_projects` is `s8c`'s row. Both
  are cited, not re-specified. The pass's whole suite meets a tree `setup` has prepared, including the Node
  fixture's dependencies, and a test asserts it (§8g.2, §8g.6).
- **Owners named**: the `release_tag` row name follows `s8c`'s registry (§8g.7). AC-114's `curation.decision` lint is
  `s8j`'s (§8g.7). AC-341's CI-green half gets a dated register note (§8g.5).
- **D21-44**: the run-owned worktree is `.redkiln/worktrees/<item>`, and the mutation tree never sits inside it
  (§8g.5).
- **The checkpoint**: the floor is re-derived from Checkpoint 8f's recorded count. The replays are bounded by
  `RK_CP_NESTED`, and none may reach a real `claude`. Checkpoint 8b joins the replays, because the re-homing moves its
  pinned lattice. The cold figure is `s8c`'s, cited, and it is the slowest child (§8g.2, §8g.10).
- **New findings**: `smoke-test` has no check, and R4 decides it (§8g.6). `integration-tests` and `migration-dry-run`
  keep agent steps at verify, and that residual is named (§8g.5).

*Revised again 2026-09-24 for D21-46* (HEAD `c5d19d7`, against that ledger's `## D21-46`). Its R5 and R6 are this
stage's. They are cited as **D21-46 R5** and **D21-46 R6** throughout, so they cannot be confused with this stage's
own records R1–R5 (§8g.8). The last bullet of the list above is superseded by both. What changed:

- **D21-46 R5, first half**: *"`integration-tests`' test-writing steps are re-homed to implement, as N23"*. The
  activity moves to the implement stage whole, as N23 moved the four, over the same test-authoring kind. It takes a
  version bump (2 → 3) and a changelog row, `story.toml`'s `external_dep` → `IntegrationReport` row moves with it,
  and the P6 lattice re-run covers it (§8g.2).
- **D21-46 R5, second half**: `migration-dry-run` is AC-347's **single** named residual. That no Phase-1 lane reaches
  it is verified by quotation, not asserted (§8g.5). The AC-347 lint's residual set is now that one activity, and a
  selection test asserts the unreachability.
- **D21-46 R6**: `smoke-test` gains a `passing` check, `smoke.status == 'passing'` with `on_false = "fail"`, as an
  ADR-0072 edit (1 → 2) with a changelog row. The option framing in §8g.6 is replaced by the decision. Its test is
  renamed `demands::a_failing_smoke_result_fails_the_stage` and proves both arms.
- **Counts**: **sixty-four** new tests (`rk-kernel` 9 → 11), and twenty-three mutants (M20–M23 added, §8g.9).
  Checkpoint 8g (2), (5), (7c) and (7g) read the new shape.
- **The floor**: the drafted `PREV` is **954**, `s8f`'s drafted Checkpoint 8f `FLOOR`, in place of 939. It is still
  re-derived at stage open from Checkpoint 8f's recorded count (§8g.10).

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The run worktree is `s8d-ii`'s** (D21-46 R1): its root, the `setup` call and the carry at creation. Every
  sentence that named `s8e` for them now names `s8d-ii`.
- **`verify_completed`'s `item` rides `s8e`'s interim key, beside the payload** (§8g.4). D21-46 R3 fixes N5's key
  *"outside the canonical digest"*, and `s8e`'s record supersedes nothing of ADR-0131 because the key is not in any
  payload. A payload `item` here would be the addressing member ADR-0131's first forbids bullet names. The two names
  `s8j` binds to are unchanged: the `item` key and the `adapter` payload member.
- **(12) adds `REDKILN_LAUNCH_RECORD` unset** to every replay (D21-46's one replay guard), and the trap asserts that it
  starts empty.
- **The floor chain is recomputed once**: Checkpoint 8f's 1063 + this stage's 64 = **1127**, not the drafted 954 + 64,
  and (9) gains the at-least-44-result-lines guard.

---

**Why this step exists.** The fix lane's verify stage demands three artifacts and its release stage two, and at
`ca8f387` none of the five has a producer a run can reach. `bug-standard.toml` reads *"`baseline = ["TestReport",
"ConfirmationResult", "SmokeResult"]`"* at verify and *"`baseline = ["BuildArtifact", "ReleaseRecord"]`"* at release,
and its verify stage adds *"`when = "'text' in change"`"* → `ScanReport`. D21-41 (c) and (d) narrowed four of those
out as owed; **D21-42 6a reverses the narrowings** and 6b gives a CLI its own release. Underneath the producers sits
the verify record itself, which three owed items have kept open since 7b-ii:

- **The transport.** `s7c` routed it as RT-6 / R-9 — *"the transport into `rk_kernel::verify` is 7d's composition
  root — a record is OWED (R-9)"* — and `s7d` §7d.18 raised the verify block (AC-320, AC-338, AC-339, AC-341, AC-348,
  AC-349, AC-353, AC-355, AC-359) because *"the verify activity runner is neither an adapter's nor this stage's set
  (no §12 or §13 criterion names it); a record routing the block to a stage is OWED"*.
- **D17-3**, OPEN since 2026-09-17: three key spellings no record fixes, and *"`rk_types::AcceptanceCriterion` (AC-137,
  ADR-0104's `mutation:` field), which the tree does NOT carry"*. The kernel reads a stand-in whose own header says
  so: *"a kernel-local shape, decided 2026-09-17 by the operator: the full `rk-types` spec type is the migration's,
  7d"* (`crates/rk-kernel/src/verify/ledger.rs`).
- **D17-4**, OPEN since the same day: AC-321's *"its record is replayed for audit"* needs a completed verify run's
  event, and `EventKind` has none.

**This stage takes records, and says so at the top.** Five are named in §8g.8, each at the next free id **at the
moment it is written** (no number is pre-assigned here: `s8a`–`s8f` write records too, and their order is not
fixed). Every one is committed `proposed` and flips only on the operator's dated ledger line. Three earlier stage
files said *"this step takes no decision"* and were wrong (`s7g`, `s7h`, `s7i`); this one does not repeat the
sentence.

**What it inherits and must find built, measured at its open (Halt 8 if any is absent):**

| from | what | how the open measures it |
| --- | --- | --- |
| `s8c` (D21-42 1a) | `Toolchain::test` returns an outcome-bearing report (the `VerifyReport` plus passed/failed counts); a red suite is `Ok` with counts; `Err` is cannot-run, timeout or killed only | read the trait in `crates/rk-ports/src/toolchain.rs`; the **seven**-method count after `s8c`'s record V (1b's generic method is the seventh) |
| `s8c` (D21-42 1b, 4a; record V) | the registry-backed seventh method, design §4.5's `registry.toml` verb table with its per-toolchain values table, and the Rust `smoke`, `sca_scan` and `package` handlers | the registry file, its Rust rows, and the three handlers' own tests in `s8c`'s Checkpoint 8c |
| `s8c` (D21-43 N25) | a per-toolchain `setup` verb, and `build` widened to an outcome report as 1a widened `test` | a `setup` row in `.redkiln/registry.toml`; `Toolchain::build`'s return type carries the outcome |
| `s8d` (D21-42 3a, 4c, 4d) | step-grain commits and the complete reference resolver: output binding, iteration, check evaluation, emit binding and test-selection resolution; `"affected"` is the package-level affected set, falling back to whole-suite with `basis_reason` | a fixture activity of shape `[tool, check, emit]` walks to an `artifact.write` |
| `s8d` (its R5) | the question loader, used by `run` and `answer`, over `.redkiln/processes/`, holding the lanes' ten gate questions, `confirmation.not-fixed` and `scan.accept-findings` among them | `answer` no longer passes `&QuestionRegistry::default()`; each of the two ids is declared once under `.redkiln/processes/`; whether `s8d`'s loader test pins the total at ten (§8g.6) |
| `s8e` (D21-42 24c) | the first FakeHarness end-to-end test of `verbs::run::run` | `grep -rn "verbs::run::run(" crates/*/tests` is non-empty |
| `s8d-ii` (D21-44; D21-45 W1–W3; D21-46 R1) | run-owned worktrees at `.redkiln/worktrees/<item>`, with `setup` run at creation | no code line under `crates/rk-cli/src` joins the common dir with `redkiln/worktrees` |
| `s8f` (D21-42 5a, 5b) | rk-store's render/write path, one `artifact.write` per write, under the item's managed artifact directory in the run-owned worktree | the one write path's call site |
| `s8b` (D21-42 11a) | the pack changelog at `.redkiln/CHANGELOG.md` | the file exists |
| `s8b` (D21-43 N25) | `sca-scan`'s check requires `unrated == 0` or routes to `scan.accept-findings` | the check's expression names `report.unrated` |

🛑 **If `s8c` or `s8d` delivered a different shape than this table names, this stage stops and reports rather than
re-deriving the shape** — the producers below are thin *because* those stages are thick, and a producer written
against a guessed port is the `s7g` shape (a lint green over code nothing calls).

### 8g.1 What this stage owns, and what it may not decide

It owns **D21-41 (c) as superseded by D21-42 1a, and D21-42 6a, 6b, 10a, 10b and 10c** (the plan's row). From D21-43
it owns **N23** (the displaced `TestReport` providers re-homed) and **N24** (AC-139's default recorded). From D21-46
it owns **R5** (`integration-tests` re-homed, and `migration-dry-run` pinned as AC-347's single residual) and **R6**
(`smoke-test`'s check). It builds **AC-133's judgement-enum registry** once, for every
later stage. D21-43's integration call reads *"AC-133's judgement-enum registry is built once, by **s8g**. s8h
consumes it."*

It does **not** own the following. It cites each owner rather than re-specifying the work:

- **DebtRegisterEntry.** 6a's fifth demand is `s8f`'s rk-store item write.
- **The Rust `smoke`, `sca_scan` and `package` handlers and their semantics.** These are `s8c`'s record V (*"The
  Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4)"*). So are `setup`, its
  `setup_projects` row and the widened `build` (D21-43 N25).
- **`sca-scan`'s `unrated == 0` edit.** N25's catalog edit is `s8b`'s, beside its other catalog edits. This stage's
  producer walk reads the edited activity and does not edit it again.
- **Question loading.** The loader and `.redkiln/processes/` are `s8d`'s R5. The pack-declared human-root questions
  are `s8j`'s.
- **The run-owned worktree.** Its root, `.redkiln/worktrees/<item>`, and the carry at creation are `s8d-ii`'s (D21-44;
  D21-45 W1–W3; D21-46 R1).
- **Card grants, the `Spec` schema and the AC-133 classification.** The card grants, the `Spec` type's schema and the
  per-enum AC-133 classification the operator signs are `s8h`'s (D21-41 g).
- **The story lane, the real trial and AC-114.** The story lane is `s8j`'s (D21-41 f). So are the tag-release real
  trial, AC-114's promotion and its `curation.decision` lint (§8g.7).
- **Any real dispatch.** **No line of this stage dispatches to a real account** (D21-40 decision 2). §8g.10 (12)
  asserts this of every checkpoint it replays.

**What this stage edits, and the checkpoints each edit makes provisional.** Each row is marked `passed <hash>, re-run
pending s8g <crate>` in `PROGRESS.md` §1, in the commit that lands the edit. The precedent is ADR-0124 decision 5:
*"The consequence for Checkpoint 5 is taken, not routed around"*.

| crate or tree | edit | provisional row | re-run where |
| --- | --- | --- | --- |
| `rk-types` | `EventKind::VerifyCompleted`; `AcceptanceCriterion` in `registry::EXPORTED`; the judgement-enum registry | Checkpoint 5 | Checkpoint 8g (12), whole |
| `rk-git` | `GitOp::Tag`, `GitOp::Apply` | Checkpoint 6 | Checkpoint 8g (12), whole |
| `rk-kernel` | the verify runner; `KindSource::Pass`; the ledger reads `rk_types::AcceptanceCriterion` | Checkpoints 7b and 7b-ii | Checkpoint 8g (12), whole |
| `rk-adapter-toolchain-rust` | **tests only**: `selection::ac348_…` and `selection::ac355_…`, both over `s8c`'s affected-set function. No source line changes, because the handlers are record V's | none | Checkpoint 7c's whole re-run, with its live probe, is `s8j`'s (D21-43 N28: *"s8j re-runs it whole, from a plain terminal through the launcher, with the live probes"*) |
| `rk-cli` | `verify_pass.rs`, the `release_tag` git-route handler, the release write, the trial route | Checkpoint 7d | Checkpoint 8g (12), whole |
| `.redkiln/` | the four re-homed activities and the moved escalation row (N23); `integration-tests` re-homed and `story.toml`'s `IntegrationReport` row moved with it (D21-46 R5); `smoke-test`'s check (D21-46 R6, recorded in R4); two `[policy]` rows (N24); the goal amendments; `tag-release` in `catalog-extended/`; `verify.ac-discriminates` in `s8d`'s registry; this repository's `package_binaries` and `claims` rows in `s8c`'s table | Checkpoint 8b. Its shipped-pack lattice *"pins the measured shape"*, and the re-homing moves that shape | Checkpoint 8g (12), whole |

**Checkpoint 8d is conditional, and it is measured at stage open, not assumed.** Its (9) asserts that each of its ten
ids is declared exactly once, and an eleventh id leaves that true. If `s8d`'s loader test pins the registry's total
at ten, the question this stage adds moves it. In that case the test takes a dated edit in the work commit, and
Checkpoint 8d joins (12)'s list by a dated edit.

**The re-runs are bounded**, because nested re-runs cost hours. (12) re-runs the entry gate, 8f, and the provisional
rows above, each whole, with `RK_CP_NESTED=1` and without `REDKILN_LAUNCH_RECORD` in its environment, under the
trap (D21-46). When this block is itself run with `RK_CP_NESTED=1`,
as `s8h` replays it, (12) says that it is skipped and replays nothing. A replayed block that does not honour the
variable still runs its own replays. That costs time, not correctness.

### 8g.2 The verify pass — one TestReport route, called through the widened port

**The route, stated once, because two silent routes is the risk the packet named.** D21-41 (c) chose *"an rk-cli
verify pass on ADR-0162's shape … selected by the required kind `TestReport` and never by a stage literal"*; D21-42
1a kept the pass and changed what it calls — `Toolchain::test` now returns the outcome, so the pass calls **the
port**, and 1a *"Supersedes D21-41(c)'s 'through the port' register reading, which is withdrawn as unneeded"*. So:

- **`TestReport` has exactly ONE producer on a lane: the verify pass.** It runs for a node whose required set
  contains `TestReport` — read off the resolved selection, never off `resolved.stage` (ADR-0162 decision 5: *"The
  review pass is selected by an activity's declared `provides`, never by a stage name"*, and the same record forbids
  *"matching `resolved.stage` against a string literal"*).
- **The catalog's four `TestReport` providers are DISPLACED, not deleted.** `artifacts.toml`'s header says *"source =
  "activity" → some catalog activity provides it"* and the selector's `KindSource::Activity` doc says *"Some catalog
  activity `provides` it"* — so as shipped, selection would walk `unit-tests`, whose `gaps` and `fill` steps are
  `kind = "agent"` under `when = "rigor >= recommended"`, and bug-standard's verify stage is `mandatory`. That walk
  breaches AC-347 in terms (*"a verify run that emits any harness dispatch fails the activity"*). The verify record
  (§8g.8, R1) therefore adds a third `KindSource`, **`pass`** — *produced by a composition-root pass the record
  names* — and sets `[artifact.TestReport] source = "pass"`. The selector selects no catalog producer for a `pass`
  kind; stage completeness still requires its `artifact.write`. **The rejected alternative is named**: keep
  `source = "activity"` and author an agent-free `whole-suite-test` activity ranked first — rejected because
  bug-standard's `[[stage.verify.require]] when = "criticality >= high" … rigor = "mandatory"` escalates the
  test-design family to `equivalence-boundary` and `property-based-testing`, both of which carry agent steps, so the
  catalog route re-enters AC-347's breach at the rigor the fix lane is most likely to reach.
- **ConfirmationResult, SmokeResult and ScanReport are catalog walks** (§8g.6) — `confirmation-test`, `smoke-test`
  and `sca-scan`, all agent-free, walking under `s8d`'s resolver and `s8c`'s port and registry method. Their
  `toolchain:test` calls are port calls like the pass's, and journal the same row (§8g.4).

**The displaced providers are re-homed (D21-43 N23), and there are FOUR of them, measured.** N23 reads *"The five
displaced TestReport activities | **Re-homed to the implement stage** as test-authoring activities under ADR-0072.
Verify only measures."* The draft this revision replaces listed five, and one entry was wrong. At `a6349b6`,
`grep -lE '^provides *=.*"TestReport"' .redkiln/catalog/*.toml` finds **four**: `unit-tests`, `unit-tests-ue`,
`equivalence-boundary` and `property-based-testing`. `migration-dry-run` declares *"`provides     =
["MigrationTestReport"]`"*, and a `pass` source for `TestReport` does not displace that kind. Its one require row was
also dropped at 7e (`story.toml`: *"DROPPED: [[stage.verify.require]] when = 'data_migration', artifacts =
["MigrationTestReport"]"*). R1 (§8g.8) executes N23 over the four:

- **Each activity moves to the implement stage and provides the test-authoring kind.** It takes `stage =
  "implement"`, and in `provides` it replaces `TestReport` with **one test-authoring kind**, listed first. R1 names
  that kind and gives it a `source = "activity"` row in `artifacts.toml`. Its emit binds that kind. Its own
  `toolchain:test` runs stay as step evidence, never as the `TestReport` artifact. R1's options are **(a)** mint a
  kind for the authored tests, or **(b)** reuse `TestDesign`. Option (b)'s value schema (`spikes/p9/schemas.toml`
  `[value.TestDesign]`) is a case design, and `equivalence-boundary` already emits it beside its report, so one kind
  would carry two meanings. Recommended: (a).
- **Each takes a version bump and a row in `.redkiln/CHANGELOG.md`.** Under ADR-0072, an edited activity is a new
  version.
- **The test-design escalation moves with them.** Both packs' verify stage carries *"`when = "criticality >= high"`"*
  → `TestReport` at `rigor = "mandatory"`, under the comment *"criticality raises the rigor of test design"*. That row
  moves to `[stage.implement]` over the new kind in both packs, because a rigor floor on a pass-sourced kind selects
  nothing. `selection::the_rehomed_test_authoring_activities_select_at_implement` asserts the outcome.
- **The P6 lattice is re-run, and Checkpoint 8b becomes provisional.** The lattice, ported to Rust at `s8b` (D21-42
  13b), is re-run with the new source and stage, and its dead count is reported **by id** against AC-102's registered
  20% line. `s8b`'s shipped-pack lattice test *"pins the measured shape"*, so its pin takes a dated edit in this
  stage's work commit, and Checkpoint 8b is re-run whole (§8g.1).

⚠️ **Two things N23 does not settle. The first is named in R1 and reported upward (§8g.11). D21-46 R5 settles the
second:**

- **`unit-tests-ue` authors nothing.** Its steps are two tool steps (`cook`, `automation`), a check and an emit, with
  no agent step. It is re-homed by N23's letter, and the record says so.
- **Two agent-bearing activities stay at verify.** `integration-tests` (`IntegrationReport`, one agent step, reachable
  in the story lane under `external_dep`) and `migration-dry-run` (one agent step, unreachable since 7e) remain at
  verify after N23, so its *"Verify only measures"* did not hold over the shipped catalog. D21-46 R5 reads
  *"**`integration-tests`' test-writing steps are re-homed to implement**, as N23. `migration-dry-run` stays a named
  residual, because no Phase-1 lane reaches it (`data_migration` is NOT MEASURABLE)."* The first half is executed
  below. The second is executed in §8g.5, where AC-347's shipped-catalog lint pins `migration-dry-run` alone.

**`integration-tests` is re-homed as N23 re-homed the four (D21-46 R5).** Its test-writing step is `write`
(*"`kind    = "agent"`"*, *"`role    = "implementer"`"*), and its `green` check loops back to it (*"`on_false = { to =
"write", max = 2 }`"*). A step cannot sit in a different stage from its activity, so R1 moves the activity whole, as
N23 moved the four:

- **`stage = "implement"`, and the test-authoring kind is listed first.** `provides` becomes `[<K>,
  "IntegrationReport"]`, where `<K>` is the one kind R1 names for the four. The emit binds `<K>` beside the
  `IntegrationReport` it already composes. `IntegrationReport` stays, because no other activity provides it and
  `story.toml` requires it.
- **The version moves 2 → 3, with a changelog row.** `.redkiln/CHANGELOG.md` takes `integration-tests@3` in `s8b`'s
  form (`s8b` §8b.5: *"`<id>@<new version>`, the date, and the ledger line"*, quoting the old and new text). The
  ledger line is D21-46 R5.
- **The require row moves with it, for N23's reason.** `story.toml`'s *"`when = "external_dep"`"* → *"`artifacts =
  ["IntegrationReport"]`"* row moves from `[stage.verify]` to `[stage.implement]`. A verify-stage row over a kind
  whose only producer sits at implement selects nothing. `bug-standard.toml` has no such row: measured at
  `c5d19d7`, `grep -rn IntegrationReport .redkiln/packs` finds `story.toml` alone.
- **Verify still measures the integration tests.** The tests that `write` authors are in the workspace, so the verify
  pass's whole suite (below) runs them. Only the authoring leaves verify.
- **The rejected alternative is named.** Splitting the activity into an implement-stage authoring activity and an
  agent-free verify measurement would mint a 103rd shipped activity. That moves `s8b`'s pinned count of 102 and the
  `curation.decision` backfill (§8g.7), and N23's precedent moves activities whole.
- **The lattice re-run above covers it**, and its dead count is reported by id with the four.
  `selection::integration_tests_selects_at_implement_under_external_dep` asserts the outcome: over `story.toml`, with
  `external_dep = true`, `integration-tests` is selected at implement and at no other stage.

**What the pass does, in order** (`crates/rk-cli/src/verify_pass.rs`, beside `review_pass.rs`, on its shape):

1. **Meet a set-up tree.** The pass runs in the run-owned worktree, which is at `.redkiln/worktrees/<item>` after
   `s8d-ii`'s D21-44 move. The whole suite it requests includes the Node fixture tests. `s8c`'s `setup` verb has run
   there at creation (the carry is `s8d-ii`'s). If it failed, creation refused by name (N25: *"refused by name on
   failure"*), so no pass reaches a tree that was not set up.
   `verify_pass::setup_precedes_the_whole_suite_in_a_fresh_run_worktree` checks this over a counting `Toolchain`
   double on a fresh run. It asserts that `setup` comes before the pass's one `test` call, and that a failed `setup`
   leaves zero `test` calls.
2. Refuse before any port call if the changed-file set or the selection policy is unresolved (AC-320) —
   `StepError`, no gate row, no port call (asserted by a counting `Toolchain` double: zero calls).
3. Call `rk_kernel::verify::runner` — **the kernel makes the port call** and journals `verify_completed` in the same
   function (§8g.4). The selection is AC-328's *"explicit whole-suite policy"*: the pass requests whole-suite, and the
   report's `basis_reason` is what the adapter says under AC-352's closed enum (`probe-unverified` while no
   affected-selection probe is recorded at the pin).
4. Map the outcome: `Ok` with `failed == 0` is green; `Ok` with `failed > 0` is a **failed verify, never a report of
   green**; `Timeout` and an OS kill are `Unverifiable` (AC-331); a spawn failure is `Unverifiable { cannot_run }`
   (AC-344, the note `s7d` §7d.12 owed).
5. Validate through `report_at_boundary` (AC-324), feed the ADR-0104 decision 3 comparator, and write the keyed
   `TestReport` through `s8f`'s rk-store write path — **one `artifact.write`**. The key is the item's `toolchain`
   facet element (ADR-0073: *"one instance per element of the item's `toolchain` set"*), **never the adapter name**.
6. Run the AC ledger and the stage-exit check (AC-346, ADR-0125) over the item's ACs as `rk_types::AcceptanceCriterion`
   values (§8g.3), including the mutation harness for every AC carrying a declared `mutation` (§8g.5, AC-338/AC-339).
7. Append through the store and re-read the journal (ADR-0162 decision 6's idiom).

🛑 **The pass is whole-suite, and a cold whole suite runs against ADR-0140's bound.** ADR-0140 decision 2 reads *"It
is not a backstop, because there is nothing behind it"*. For tool steps, `s8c`'s record B moves that sentence. D21-42
10c says *"A cold `cargo test --workspace` wall time is measured first"*, and `s8c` §8c.5 is that measurement: *"This
is that measurement, and `s8g` cites it rather than repeating it."*

**This stage cites the figure by quotation from `s8c`'s record and names the commit it was measured at. It does not
re-measure.** If `s8c`'s record carries no such figure, this stage stops (Halt 8) rather than measure a second time.

**The figure is the slowest child.** The Rust adapter's timeout bounds *each child it spawns*, not the call (ADR-0140
decision 2, as `s8c` §8c.5 reads it), and §8c.5 finds that *"the cold compile lands in the FIRST, the `--list` pass"*.
So the verify record carries one line of the form `cold cargo test --workspace slowest child: <N> s (source: <s8c's
record id>, at <sha>)`, and Checkpoint 8g (8) reads it.

**A figure at or above 3600 s is a halt.** Every cold verify pass would then be `Unverifiable` by construction (golden
constraint 6: size the ceiling against the slow condition).

### 8g.3 The spec/pack schema record — D17-3's three spellings, and `AcceptanceCriterion` exported (D21-42 10a)

D17-3 lists three spellings that are *"readings, not decisions"*. The schema record (§8g.8, R2) fixes all three **as
they are read today**, so nothing on the tree moves but its warrant:

| spelling | where it is read today | fixed as |
| --- | --- | --- |
| `[policy] verification_required = [<stages>]` | `select/pack.rs`: *"**AC-455's policy row**"* | the process pack's key, unchanged |
| `[mutation] feasible = <bool>, reason = <str>` | `rk_ports::toolchain::MutationFeasibility::declared`; the Rust fixture pack | the key, unchanged — **and its shipped home** (below) |
| `[[ac]] … [ac.mutation] not_feasible = "<reason>"` | `verify/ledger.rs`'s fixture reader | superseded by `AcceptanceCriterion`'s serde form (below) |

**Two absences the record must state, both measured at `ca8f387`:**

- **The shipped `.redkiln/` declares no `[mutation]` table anywhere.** Its two packs are process packs, and AC-337 says
  *"an absent declaration is a configuration error"*. The record gives the declaration a shipped home — the Rust row
  of `s8c`'s per-toolchain values table (D21-42 4a: *"Each value carries its measurement as its warrant"*), with
  `feasible = true` and ADR-0104 decision 1's warrant — so the ledger's feasibility input exists on this tree.
- **Neither shipped pack declares `verification_required`.** Re-measured at `a6349b6`: each pack's `[policy]` holds
  `stages` and `stage_order` only, so AC-455's refusal cannot fire on this tree. What the shipped default *is* belongs
  to **AC-139, a HUMAN GATE**: *"the choice is recorded as a human decision with its rationale, and the shipped
  default is whatever that decision names"*.
  - **The operator took that decision on 2026-09-24.** D21-43 N24 reads *"**Block (fail-closed)**, recorded as the
    human decision AC-139 requires"*. R2 records the decision and its rationale by quotation.
  - **Both shipped packs gain `verification_required = ["verify"]` in their `[policy]` table.** The row names the
    verify stage because that is where the pass runs the ledger and the admissibility check (§8g.2, step 6).
  - **AC-139's own *Verified by* makes a policy row the right form**: *"What IS machine-checked is AC-138: the value
    is a policy-table row, so the human decision is a data change"*. AC-139 takes a dated register note that names
    N24 and R2.
  - **A test proves the row decides.** `verify_pass::ac139_shipped_packs_block_a_missing_verification` loads each
    shipped pack over a fixture spec with `verification: None` and asserts that the stage is refused, naming the AC
    id. It then asserts that the same spec passes under a copy of the pack with the row removed. That is AC-138's
    *"computed from the policy table row, not from a constant in the type"*.

**`rk_types::AcceptanceCriterion`** (AC-137's GIVEN/THEN; ADR-0104 decision 1's field): `id`, `criterion: Text`,
`gwt: Option<Gwt>` (*"`Gwt { given: Text, when: Text, then: Text, and: Vec<Text> }`"*), `verification:
Option<Text>`, and `mutation: Option<MutationDeclaration>`, where `MutationDeclaration` is
`#[serde(tag = "kind")]` over `Declared { patch, suite }` and `NotFeasible { reason }` (ADR-0084's discriminated
form). A prose-only mutation is refused at ingest (ADR-0096 via golden constraint 12). It enters
`registry::EXPORTED` — whose doc says *"Every schema that crosses the closed-output boundary"* — and gains its
`exported__AcceptanceCriterion.snap`, accepted once, deliberately, and reviewed as a schema change.

**The kernel-local type is retired.** `SpecAc` and its TOML reader leave `rk-kernel`; the ledger, the exit check and
`admissibility::admissible` read `&[rk_types::AcceptanceCriterion]`. Two text fixtures carry the old name and move
with it (`crates/rk-kernel/tests/fixtures/lint-controls/ac335_status_reads_the_suite_verdict.rs` and a string in
`tests/it/verify.rs`) — the name occurs **17** times under `crates/` at `ca8f387`, and Checkpoint 8g (4) requires
zero code lines.

🛑 **AC-133's registry does not exist, and this stage builds it rather than registering into nothing.** The brief
asks for the new enum to be *AC-133-registered*. AC-133's THEN is *"the enum appears in the judgement-enum registry
with an explicit judgement-bearing yes/no AND every enum marked yes has a sibling free-text `rationale` field"*.
Measured at `ca8f387`: `grep -rni judgement crates/rk-types/tests crates/rk-types/src/schema` returns **nothing**,
and no rationale-sibling rule exists — yet `s5`'s acceptance set claims AC-133 and `PROGRESS.md` §1 row 5 says
*"Decisions 6 and 7 executed (ADR-0104's `mutation:` field on `AcceptanceCriterion`…)"*, which D17-3 already called
false. The registry is built here to AC-133's *Verified by* (a completeness test over every enum reachable from
`EXPORTED`, the rationale-sibling rule, and a must-fail fixture), `Findings`' enums registered with it; row 5 takes a
**dated correction in the tracker commit**, not an erratum (D15-2's form for an amendable file). A third test,
`judgement_registry::the_registry_names_no_enum_that_is_not_exported`, keeps the registry from outliving its types.

**The registry is built once, here, and `s8h` consumes it** (D21-43's integration call). This stage registers every
enum reachable from `EXPORTED` at its close, `Findings`' and `AcceptanceCriterion`'s included, each with a stated
yes/no. The types `s8h` adds to `EXPORTED` register into the same registry. The per-enum classification the operator
signs is `s8h`'s, because AC-133's own limit reads *"that classification is the human gate AC-156"*. **`s8h` does not
build a second registry.**

⚠️ **A cross-stage constraint on `s8h`.** The pass reads the item's ACs off the committed `Spec` artifact. The `Spec`
type's schema is `s8h`'s (D21-41 g: schemas are Rust types in `EXPORTED`). Until then this stage's fixtures carry a
`Spec` whose one typed member is `criteria: Vec<rk_types::AcceptanceCriterion>`, and **`s8h`'s `Spec` must embed
`AcceptanceCriterion` unchanged** — named here so the second author does not re-invent it.

### 8g.4 `verify_completed` — AC-321's replay event and the identity carrier (D21-42 10b)

The kind is minted by record on ADR-0116's route (its *Option — mint the kind in the runbook* was rejected *"on
mechanism"*), in the verify record (§8g.8, R1), with the design §5 table row and the `rk-types` variant **in the
same change** (ADR-0116 decision 5). Its row:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `verify_completed` | 2 | `report_digest`, `selection`, `basis`, `basis_reason`, `passed`, `failed`, `selected_test_count`, `adapter`, `range: { base, head }`, `paths: [changed paths]`, `toolchain_version`; and the interim `item` key **beside** the payload, on `s8e` §8e.10's carrier | the kernel, once per **completed** `Toolchain::test` port call, whatever activity or pass made it |

- **10b's four members plus AC-321's three.** D21-42 10b names *"report digest, selection with `basis_reason`, counts
  and adapter identity"*. AC-321's THEN adds *"the range endpoints, the resolved path list and the toolchain version
  are read back from the run event verbatim"* — so `range`, `paths` and `toolchain_version` are carried **as
  members**, not only inside the digest. That is AC-321's text, not a new decision.
- **`item`, because the store cannot scope a row any other way, carried beside the payload.** When this was drafted
  the store's selector was inert, and ADR-0163 decision 4 carried `item` as a payload member for that reason. D21-43
  N5 has since made an interim key the rule for every item-scoped kind (*"An interim `item` key on every item-scoped
  event kind"*), and D21-46 R3 fixes where it lives: *"**Carried outside the canonical digest**, as ADR-0131 places
  its tag set."* `s8e` §8e.10 carries it beside the event, in the `t2_event.item` column, and `query(ByItem)` honours
  it. So `verify_completed` takes the key on that carrier and not in its payload, which moves no ADR-0131 bullet, and
  `s8e`'s `fsm::every_item_scoped_kind_carries_the_interim_item_key` covers the new kind. `s8j`'s fixture proof reads
  the item's `verify_completed` rows by that key and asserts `adapter` on every one. The two names, the `item` key and
  the `adapter` member, are **fixed here** so `s8j`'s probe binds to them rather than to a guess (*restated
  2026-09-24, consistency pass*).
- **Tier 2**, by ADR-0109 decision 2's deleted-database test: the report is recomputable by re-running the suite, and
  the next action does not move without the row. No trailer.
- **Completed only.** A run that timed out, was killed or could not spawn writes **no** `verify_completed`; its
  refusal travels as the typed `StepError` row D21-42 20 has the kernel append. AC-321 says *"a **completed** verify
  run"*, and a row for a run that did not complete would be a replay of something that did not happen.
- **One site.** The executor's `toolchain:test` arm and the pass both reach the port through the one kernel function
  that appends the row, so a `confirmation-test` walk journals the same kind as the pass.
- **Adapter identity is data the kernel carries and never compares** (AC-487: *"`rk-kernel` contains no comparison
  against an adapter identity — no adapter name string"*). `rk-cli` supplies the name `real_toolchain`'s `detect()`
  loop chose, as an opaque value on the runtime context. argv[0] is not the identity (ADR-0137 clause 7 makes it
  configurable), and `rustc -V` names a compiler, not an adapter.

**The identity carrier moves, and the register moves with it.** AC-473's *Verified by* reads *"an adapter-identity
assertion on the toolchain stage's dispatch record"* — and no dispatch record carries a toolchain. The verify record
carries the dated register amendment D21-42 10b authorises: the assertion reads the item's `verify_completed` rows.
AC-485's PROVISIONAL identity half (`s7d` §7d.18: *"the dispatch-event identity half has no `DispatchRecord` field to
read and the carrier is routed, RT-41"*) is discharged **for the toolchain adapter** by the same row, and a dated note
on AC-485 says so; its harness half stays with RT-41's carrier. **`s8`'s own sentence** *"the verify stage's
toolchain is rk-adapter-toolchain-rust"* is re-pointed at `verify_completed` by `s8j` (the plan's instrument
amendments), not here. **D17-4 closes** in the tracker commit, naming the record.

### 8g.5 `s7d` §7d.18's verify block, claimed whole (D21-42 10c)

Each id has a named instrument, or a named reason in the verify record. **No id is reached only by the crate-wide
floor.**

| id | instrument (module-qualified, `--test it`) | built on | residue, named |
| --- | --- | --- | --- |
| **AC-320** | `verify_pass::ac320_unresolved_inputs_refuse_before_any_port_call` | a counting `Toolchain` double: zero calls, zero gate rows | its *Verified by* names *"zero `process_spawned` events"*, and **no such kind exists** (`grep -c process_spawned crates/rk-types/src/event_kind.rs` → **0** at `ca8f387`). The record carries a dated *Verified by* note substituting the port-call count — no spawn is possible without a port call — rather than minting a kind for one assertion |
| **AC-321** | `verify::ac321_replay_reads_the_path_list_from_verify_completed_alone`; `verify::ac321_replay_property_over_100_synthetic_ranges` | §8g.4's payload; the replay reads no working tree | — |
| **AC-338** | `verify_pass::ac338_restored_mutation_is_unverifiable` | the mutation harness below; a fixture suite that restores the mutated file mid-run (F17's shape); the post-run content hash of the mutated region compared to the applied mutation | — |
| **AC-339** | `verify_pass::ac339_mutation_runs_in_a_disposable_worktree` | an ephemeral worktree created and **removed on every path** (ADR-0162 decision 10's precedent). It sits beside the review pass's `<item>.review-ref` tree, at whatever root `s8d-ii`'s D21-44 move gives that tree, and **never inside the run-owned worktree at `.redkiln/worktrees/<item>`**. The run-owned worktree's `tree_hash` is equal before and after, and `git status --porcelain` is empty | the patch is applied by the kernel through `GitOp::Apply` and never by a `-p` child, so Claude Code's protected-segment rule (D21-45: *"The path is sensitive if ANY segment is one of"* …) does not bear on where the tree sits |
| **AC-341** | `verify_pass::ac341_the_pass_calls_no_tracker_method` (a counting `Tracker` in `Ports`, which `run.rs` already builds as `tracker: Arc::new(rk_ports::fakes::BoardTracker::default())`); the static half stays `verify_reads_no_suite_verdict_rule`'s, inside `ac335_…` | the port-call assertion | **the *"CI is green and verify is `Err`"* integration half cannot be constructed**: the `Tracker` port has `project` and `inbox` and no status surface, and its module says *"Fake-only in Phase 1; GitHub is Phase 3"*. Named in the record as Phase 3's (AC-508), not claimed. **R1 carries a dated register note on AC-341 that says so**, so the half is recorded as Phase 3's rather than dropped silently |
| **AC-348** | `selection::ac348_selection_is_byte_identical_over_200_change_sets` (Rust adapter); `verify_pass::ac348_selection_is_recomputed_from_verify_completed` | `s8c`'s affected-set function over a pinned `cargo metadata` fixture — **it must be a pure function of (change set, metadata, policy version)**; if `s8c` built it impure, stop (AC-347 requires *"a pure function"*) | — |
| **AC-349** | `verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both`; `doctor::ac349_equal_specificity_claim_is_a_doctor_row` | a claims binder over each configured toolchain's declared path claims — a `claims` column the verify record adds to `s8c`'s per-toolchain table; the doctor row `RK-D-TOOLCHAIN-CLAIM-TIE` | — |
| **AC-353** | `verify_pass::ac353_killed_verify_resumes_to_the_same_decision` | a real `redkiln` over a fixture whose `[adapters] toolchain_programs` names `rk-fixture-toolchain` rigged to block; the tree killed through `rk-containment`; re-entry reaches the same decision, `reconcile` ran on entry, resume read the tree and the AC-321 replay did not | — |
| **AC-355** | `selection::ac355_lockfile_change_is_whole_suite_and_one_file_is_affected` (Rust adapter) | two fixtures asserted on basis **and** selected-test count | *"held in CI"* is true once `ci.yml` runs green, which is `s8j`'s (D21-42 14a) |
| **AC-359** | `verify_pass::ac359_drift_voids_the_answer_and_recomputes_the_ledger` | a verify-opened AC-357 gate (question `verify.ac-discriminates`); a commit lands between open and answer; the answer is void, the gate re-opens with the drift attached, and the ledger is recomputed | AC-357 is a HUMAN GATE and is **not claimed**; its machine-checkable part (*"the gate node is created with those three payload fields"*) is exercised inside this test and named so |

**Also discharged by the pass, beside the block**: AC-331 (`verify_pass::ac331_timeout_and_kill_are_unverifiable`),
AC-344's `cannot_run` mapping (`verify_pass::ac344_missing_binary_is_cannot_run`), and AC-347's zero-dispatch half
(`verify_pass::ac347_the_pass_dispatches_nothing`).

**AC-347's lint runs over the shipped catalog** (`verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual`).
It walks every `.redkiln/catalog` activity with `stage = "verify"` and asserts a non-zero count walked, which is D21-42
13a's rule. It then asserts that the set carrying an agent step is **exactly** `{migration-dry-run}`, the single
residual D21-46 R5 names (§8g.2). A new agent step at verify turns the test red. So does `integration-tests` back at
verify, and so does removing the residual without updating the test. The test states the residual. It does not hide
it. **Still ARMED, unchanged**: AC-350 (a composite `Toolchain`, R-8), AC-317 (Step 8).

**Why no Phase-1 lane reaches `migration-dry-run`, by quotation** (D21-46 R5: *"no Phase-1 lane reaches it
(`data_migration` is NOT MEASURABLE)"*). Each line below was read at `c5d19d7`:

- **The facet may key no demand.** `.redkiln/facets.toml`'s `[facet.data_migration]` reads *"`deriver_class =
  "lexical"           # a scan over paths and diff lines — may key no require row (AC-520)`"*. Its `deriver_record`
  comment reads *"NOT MEASURABLE at 47faec9f: 0 positives on 20 items across all three sources"*.
  `.redkiln/corpus-records.toml` says the same: *"`# data_migration -- NOT MEASURABLE: 0 positives on 20 items across
  all three sources [F39 s4]`"*.
- **The one demand for its kind was dropped at 7e.** `story.toml` reads *"DROPPED: [[stage.verify.require]] when =
  'data_migration', artifacts = ["MigrationTestReport"]"*. `grep -rn MigrationTestReport .redkiln/packs` finds that
  comment line and no row.
- **Its one consumer is unreached as well.** `migration-expand-execute` `requires` `MigrationTestReport` and provides
  `MigrationLog`. Both packs dropped `MigrationLog`'s release row at 7e (*"DROPPED: [[stage.release.require]] when =
  'data_migration', artifacts = ["MigrationLog"]"*), and no pack row names either kind.

So `migration-dry-run`'s `applicable_if` (*"`data_migration != null and data_migration`"*) is never consulted,
because selection never looks for a `MigrationTestReport` producer.
`selection::migration_dry_run_is_selected_on_no_shipped_lane` asserts the outcome. It selects both shipped packs over
every lattice cell, with `data_migration = true` forced on the fixture item, and asserts that `migration-dry-run` is
selected in none. The AC-347 lint cannot see reachability, so this test is the one that fails if a later row makes
the residual reachable (M22). A reachable residual is no longer a named exception. It is AC-347's breach.

**The mutation harness AC-338 and AC-339 need is ADR-0104 decision 1's, built here** — *"The harness applies the
patch at the candidate SHA, runs the suite, reverts, and records the outcome. It is deterministic and dispatches
nothing."* Two records bar the obvious build and are moved by record, not worked around:

- ADR-0105 clause 5 enumerates the wrapper's writes and says *"Anything not listed is a successor"*; `git apply` is
  not listed, and `crates/rk-git/src/git_exec.rs` says *"Anything not here is a successor record, and adding a
  variant is that decision made visible"*. **R3 (§8g.8) supersedes clause 5 in part** to add `apply` (and `tag`,
  §8g.7). The patch is applied **only** in the pass's ephemeral worktree, never the run-owned one.
- ADR-0162's forbids bullet includes *"forbids applying a mutation patch from `rk-cli`; `ReproKind::Mutate` is refused
  by name until a record adds the capability"*. `s8c`'s 1b record supersedes that bullet's seventh-method half. The
  verify record states the disposition of the patch clause by quotation: **if `s8c`'s record did not reach it, R1
  supersedes it in part for the verify gate's declared AC mutations alone**; the review pass's `ReproKind::Mutate`
  arm stays refused by name either way, and the record says so.

### 8g.6 The four demands restored (D21-42 6a)

D21-42 6a reads *"Reversed for five demands … **Supersedes D21-41(c)/(d)'s narrowings.**"* The four demands this stage
owns walk shipped catalog activities under `s8d`'s resolver.

**What the three Rust verbs do is `s8c`'s record V. This stage cites it and does not decide it again.** V fixes the
`smoke`, `sca_scan` and `package` handlers, their typed outputs and the scan's semantics. `s8c` §8c.4 reads *"The
scan's semantics are the ones `s8g` §8g.6 reads, and record V fixes them so that the two files agree."*

**The producers record (§8g.8, R4) fixes what is left**: the activities and this repository's data. That is two
step-list edits, the smoke checklist, and the declaration rows `s8c`'s per-toolchain table admits. Each activity edit
is a version bump with a row in `.redkiln/CHANGELOG.md` (ADR-0072).

**SmokeResult — `smoke-test`.** The activity runs `toolchain:build`, then `toolchain:smoke` with *"`checklist =
"checklist:smoke"`"*. The handler runs *"the binaries the call's build produced, never an installed `redkiln`"*
(record V).

- **The checklist is this stage's**, at `.redkiln/checklists/smoke.toml`. Its id is already on the design's checklist
  registry: `spikes/p9/registry.toml`'s `[checklists]` carries `"smoke"`.
- **Each item takes the form record V reads**: `argv`, `cwd`, `expect_exit` and one stdout predicate. **No item
  dispatches**: no `run`, no network.
- **The first cut.** Each expectation is measured on the built binary before it is written down:
  - `redkiln --version`;
  - `redkiln-migrate --version`;
  - `redkiln --help`, naming the nine verbs (*"The nine verbs, and no others"*, `cli.rs`);
  - `redkiln -C <scratch> init`, then `status`;
  - `redkiln-migrate` with no arguments, exiting clap's `2` (RT-53).
- **The scratch repository sits outside the run-owned worktree.** Every item's `cwd` is `scratch`, a repository
  **outside `.redkiln/worktrees/<item>`** (D21-44). Inside it, D21-42 9's landing would stage the scratch files.
  Where the handler makes the scratch repository is record V's decision. This stage requires that it be outside the
  run tree, and `demands::smoke_result_is_produced_through_the_registry_verb` asserts that.
- **`smoke` is live.** The spike registry marks it *"`live = true`"*, and the design's rule is that a live verb
  *"implies `memo = false` on every step using it"*.

🛑 **A finding, decided by D21-46 R6: `smoke-test` has no check.** At `c5d19d7` its steps are `build`, `smoke` and
`emit`, and its version is 1. So a `SmokeResult` with `failed ≥ 1` is emitted and the stage completes on it. `s8c`'s
draft reads *"The catalog reads `smoke.status == 'passing'`"*, but no step does. D21-46 R6 reads *"**A check on
`status == 'passing'` with `on_false = "fail"`**, as an ADR-0072 edit in s8g."* R4 records it:

- **The step.** A check step with `id = "passing"` sits between `smoke` and `emit`. It has `kind = "check"`, `inputs =
  { smoke = "${smoke.outputs}" }`, `expr = "smoke.status == 'passing'"` and `on_false = "fail"`.
- **ADR-0072.** The version moves 1 → 2, and `.redkiln/CHANGELOG.md` takes a row `smoke-test@2` in `s8b`'s form: the
  date, the ledger line (D21-46 R6), and the old and new text quoted.
- **The rejected alternative stays named.** Keeping the activity emit-only, so that a later stage reads the status,
  was rejected. No stage-exit rule reads a `SmokeResult`'s status, and a measurement whose verdict nobody reads is
  not a gate.
- **The test asserts the outcome, on both arms.** `demands::a_failing_smoke_result_fails_the_stage` walks
  `smoke-test` twice over a `Toolchain` double. With `status = 'passing'`, it emits one `SmokeResult`
  `artifact.write`. With `failed = 1`, the activity fails, no `SmokeResult` is written, and the verify stage does not
  complete, because its baseline `SmokeResult` demand is unmet. M15 deletes the check and M23 inverts it, and each
  arm kills one of them.

**ConfirmationResult — `confirmation-test`.** `toolchain:test` with *"`selection = "artifact:ReproCase.command"`"* goes
through `selection_of` (ADR-0162) and the resolver. The check reads *"`result.status == 'passing'`"*, and a
non-passing result opens *"`question = "confirmation.not-fixed"`"*. The `ReproCase` comes from `minimal-repro` at
`s8d`'s step grain; it has two agent steps, `attempt` and `reduce`. The fixture proves both arms: at the fix sha a
passing result emits, and a result that does not pass opens the gate and emits nothing.

**ScanReport — `sca-scan`.** The activity runs `toolchain:sca_scan`, and its gate is `scan.accept-findings`.

- **The handler's semantics are record V's.** That covers both tools, the CVSS v3 bands, `unrated`, the advisory
  database's commit (or a refusal by name) and the pinned offline database. `s8c` also takes the first measurement of
  the tools' database member.
- **The activity edit is `s8b`'s, and this stage cites it.** D21-43 N25 reads *"**`sca-scan` requires `unrated ==
  0`** or goes to `scan.accept-findings`"*. As shipped, the check reads *"`report.critical == 0 and report.high ==
  0`"*, which passes with `unrated > 0` (`s8c`'s draft at `a1d35ac`: *"As shipped, that check passes with `unrated >
  0`"*). The edit is `s8b`'s catalog edit, beside its other ADR-0072 edits.
- **What this stage asserts of it.** The producer walk over the edited activity must open the gate on an unrated
  advisory: `demands::scan_unrated_advisory_opens_scan_accept_findings`. If `s8b`'s edit is absent at Checkpoint 8f's
  close, this stage stops (Halt 8) and does not make the edit itself.

**BuildArtifact — `build-package`.** The steps are `toolchain:build`, the `built` check, then `toolchain:package`.

- **The handler is record V's.** It runs a locked release build and takes the plain sha256 of each **declared**
  binary.
- **The declared set is this stage's data.** `package_binaries`, in the Rust row of `s8c`'s per-toolchain table, is
  `redkiln` and `redkiln-migrate`, with its warrant (AC-441: the migrator is *"a separate binary"*). A release build
  also links `rk-fixture-child-cli`, `rk-fixture-toolchain` and `rk-fixture-child-claude`, and none of them is the
  release.
- **The `built` check is now live.** N25 widens `build` to an outcome report. So the check *"`report.status ==
  'passing'`"*, which `s8c` §8c.4 found dead, now reads a red build.

**This repository's `setup` declaration is `s8c`'s `setup_projects` row** (D21-43 N25; D21-45 W3: *"Dependencies
come through N25's per-toolchain `setup` verb"*). This stage depends on it naming
`crates/rk-adapter-toolchain-node/tests/fixtures/project`. This repository's Rust suite includes the Node adapter's
tests, and a fresh run worktree without that fixture's `node_modules` fails ten of them at preflight (`s8c`'s draft
at `a1d35ac`: *"A fresh run-owned worktree runs this repository's suite RED"*). The verb and the row are `s8c`'s, and
running it at creation is `s8d-ii`'s. This stage asserts that the pass's whole suite meets a tree that has been set up
(§8g.2, step 1), and (7e) reads the row as an entry precondition.

**The gate questions load through `s8d`'s registry, and this stage writes no `.redkiln/questions.toml`.**

- **Two questions are already `s8d`'s.** `confirmation.not-fixed` and `scan.accept-findings` are two of the ten
  questions `s8d` transcribes from `spikes/p9/questions.toml` into `.redkiln/processes/`. Its default reading is
  `.redkiln/processes/questions.toml`, which its R5 fixes.
- **One question is new.** `verify.ac-discriminates` (§8g.5) is declared once under `.redkiln/processes/`, in the
  same form, in the file `s8d`'s R5 fixes. `s8d`'s loader loads it on both `run` and `answer`.
- **A test asserts it.** `verify_pass::verify_ac_discriminates_answers_through_the_loaded_processes_registry` answers
  an AC-359 gate through `answer`'s production path, not through `run_with`'s registry parameter.
- **Checkpoint 8d may become provisional** if `s8d`'s loader test pins the total (§8g.1).

### 8g.7 `tag-release` — a CLI's release, entered by ADR-0114's trial (D21-42 6b)

**What ReleaseRecord is.** 6b: *"a release tag on the landed commit, plus the BuildArtifact digest and the rollback
target (the previous release tag). The tag is local and never pushed by the run."* The tag record (§8g.8, R5) fixes:

- **The tag** — annotated, named `rk-release/<item>`, on the landed commit D21-41 (b) defines (*"A kernel-made content
  checkpoint commit on `rk-run/<item>`"*). Re-running for the same item at the same commit is idempotent; at a
  different commit it refuses by name.
- **The rollback target** — the nearest `rk-release/*` tag on an ancestor of the landed commit, read through the
  existing `GitOp::Log` (`--decorate-refs=refs/tags/rk-release/ --simplify-by-decoration`), so no new read op is
  owed. **A first release names its target explicitly as `none`**; it is never absent and never defaulted to the
  landed sha.
- **The write** — a tag is a git write, so it is **`GitOp::Tag`**, R3's second addition to ADR-0105 clause 5, and it
  goes through the one wrapper.
- **The verb row is `s8c`'s, and its name is `release_tag`.** `s8c`'s registry lands the row as `release_tag`, with
  route `git` and typed output `ReleaseTag`, *"Refused naming the route until `s8g`"*. The spike registry's
  `release_record` row (*"`outputs = "ReleaseRecord", live = false`"*) is not the shipped row. This stage builds the
  `git`-route handler. It runs **in the composition root, through `rk-git`**, and never in a toolchain adapter
  shelling git (ADR-0105: *"Forbids a wrapper write not on clause 5's list"*). The activity's emit composes the
  `ReleaseRecord` artifact from the `ReleaseTag` value and the BuildArtifact digest.
- **The target must be on the run-owned branch.** The tag target must be a commit on that branch
  (`authorise_tier1_write`'s ownership pair). Any other target refuses.
- **Never pushed** — `GitOp` has no push variant and R3 adds none; a test asserts the write set.

**The activity** is agent-free: `toolchain:release_tag` (inputs `artifact:BuildArtifact`, `artifact:Changeset.sha`),
a check that the record names a tag and a rollback target, and an emit. **Agent-free is the property that makes its
trial cheap**: it cannot dispatch, so its run needs no spend gate.

**How it enters: ADR-0114 decision 3, read whole.** *"An activity is promoted from the `extended` file to the shipped
catalog only with a ledger entry: it must have run at least once … in a run whose id the promotion decision (AC-114's
`curation.decision`) cites … the promotion route is a one-run, one-activity trial the operator declares, not a pack
reference."* And its Negative consequence: *"The extended-promotion route is a small new CLI/config surface (Phase 2,
with `rk-plugin-gen`)."* So:

- **`tag-release` is authored into `.redkiln/catalog-extended/`**, not `.redkiln/catalog/`. AC-110 is unchanged: an
  enabled pack referencing it, or selection reaching it, is a load-time configuration error, and the extended
  directory contributes **zero** entries to the resolved catalog.
- **The trial declaration is data on an item record, not a CLI flag** — `[trial] activity = "tag-release"` on a
  dedicated trial item; `run` admits exactly that one extended activity for exactly that item's one run, and the
  AC-400 help snapshot does not move. R5 records this as the Phase-1 form of the route ADR-0114 placed in Phase 2,
  with its limits stated.
- **The ledger entry exists already**: AC-532's executed ledger folds `activity_committed` rows into
  `executed: { first, last }` *"per `activity_id@version`, each stamp holding all three of `{ run_id, item, at }`"*.
- **At this stage the trial runs under the fixture harness only** —
  `tag_release::trial_runs_one_activity_with_zero_dispatches`, in a temporary repository, asserting one activation,
  zero `dispatch` and `dispatch_completed` rows, a ReleaseRecord `artifact.write`, and a local tag the fixture's bare
  "remote" never receives.
- **The trial's REAL run on this repository is DEFERRED to `s8j`, explicitly.** It needs a landed commit and a
  journalled BuildArtifact for its item, which only a lane run produces, and it creates a tag in the operator's
  repository — an operator moment, like a push. **The promotion that follows is AC-114's HUMAN GATE**, and it must
  precede any lane run whose release stage is to select `tag-release`.
- **AC-114's residue lint does not exist, and its owner is `s8j`.** The residue reads *"the file carries a non-empty
  `curation.decision` id resolving to a record in the catalog's decision log"*. Re-measured at `a6349b6`, `grep -rln
  curation .redkiln crates` finds nothing. It goes to `s8j` with the promotion it checks, for two reasons:
  - The lint's first non-vacuous input is the promotion decision that `s8j`'s real trial produces.
  - Built here, it would either fail all 102 shipped activities, none of which carries a `curation.decision` id, or
    ship with a grandfather rule that no record takes.

  That backfill is a decision owed upward (§8g.11).

**Why selection would pick it — a free residual, and R5 decides it rather than leaving it.** ReleaseRecord's four
shipped producers (`rolling-deploy`, `blue-green-deploy`, `canary-release`, `flag-progressive-rollout`) all run
`toolchain:deploy`, and no existing facet distinguishes a repository whose release is a tag. R5's options: **(a)** a
`declared` facet (the `surfaces` precedent: *"read from a checked-in declaration map"*) keyed in `tag-release`'s
`applicable_if` — `applicable_if` is not a `require` row, so a declared facet may key it — with `tag-release` first in
the `deploy-pattern` family, and **no shipped deploy activity edited**; **(b)** verb availability as a selection-time
admissibility condition, which is the long-term shape and is rejected **here** because it moves every stage's
selection on the lane `s8` runs. Recommended: (a). Checkpoint 8g asserts the **outcome**
(`tag_release::selection_picks_tag_release_over_the_deploy_patterns`), not the mechanism.

**The release goal is amended, not read.** bug-standard says *"The fix is in production reversibly"* and story says
*"In production, reversibly"*. A run that makes a local tag and pushes nothing puts nothing in production, so a
reading would leave a false sentence in the pack. R5 amends them to **`The fix is tagged for release, reversibly`** and
**`Tagged for release, reversibly`**. The packs carry no `version` key (measured), and ADR-0072 governs activity edits,
so no version moves; the edit is listed in the pack changelog `s8b` opened for D21-42 11a — `s8b` §8b.5 creates
`.redkiln/CHANGELOG.md` and says *"Nothing names the file, so the path is chosen here and said"*. **If that file is
absent at Checkpoint 8f's close, stop**: this stage does not choose a second path. The `tag-release` addition takes
its row there too.

### 8g.8 The records this stage writes

Each at the next free id **at writing**, `proposed`, in ADR-0085's form with `### Option — <name>` headings (D21-42
24a), a `## Consequences` carrying **What this forbids**, and a slug fixed here so the checkpoint can find it without a
number:

| | slug | decides | supersedes, amends |
| --- | --- | --- | --- |
| R1 | `the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed` | §8g.2, §8g.4, §8g.5: the pass and its composition, `setup` before the whole suite; `KindSource::Pass`; D21-43 N23's re-homing of the four displaced providers to a test-authoring kind at implement, with the escalation row; D21-46 R5's re-homing of `integration-tests` with `story.toml`'s `IntegrationReport` row, and `migration-dry-run` named as AC-347's single residual, its unreachability quoted; `verify_completed`; the verify block's routing, each id's instrument or reason; the cold slowest-child figure cited from `s8c`; the claims column; the mutation harness | ADR-0162's patch clause in part (§8g.5's condition); R-9 and RT-6 discharged; register: AC-473 *Verified by* (10b); dated notes on AC-320, AC-485, and AC-341 (its CI-green half is Phase 3's); design §5's table row; D17-4 closed |
| R2 | `the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion` | §8g.3, including AC-139's default: block, fail-closed, recorded as the human decision (D21-43 N24) and shipped as `verification_required = ["verify"]` in both packs; the AC-133 registry, which `s8h` consumes | D17-3 closed; AC-139 dated note; `PROGRESS.md` row 5 corrected in the tracker commit |
| R3 | `the-rk-git-write-set-gains-tag-and-apply` | `GitOp::Tag`, `GitOp::Apply`, each with its scope | **ADR-0105 clause 5 in part**; ADR-0105 keeps `status: accepted` and gains this id in `superseded_by` |
| R4 | `the-four-verify-and-release-demands-have-producers` | §8g.6: the activities (`smoke-test`'s `passing` check, as D21-46 R6 decided; the producer walks), the smoke checklist's home and shape, and this repository's `package_binaries` row. **The three verb handlers and their semantics are `s8c`'s record V, and `sca-scan`'s `unrated` edit is `s8b`'s; each is cited by id and not decided again** | executes D21-42 6a (which supersedes D21-41 (c)/(d)'s narrowings) |
| R5 | `tag-release-is-the-cli-release-and-enters-by-trial` | §8g.7: ReleaseRecord's shape, the tag name and rollback target, the `release_tag` git-route handler (`s8c`'s row), the extended directory, the item-record trial, the selection mechanism, the goal amendment, AC-114's lint named as `s8j`'s | executes D21-42 6b under ADR-0114 decision 3 |

🛑 **A change to a `## Decision` sentence or a forbids bullet is a supersession, never an erratum** (ADR-0092 rule 1).
R3 is the only record here that moves an accepted Decision sentence outright; R1's ADR-0162 disposition is conditional
and stated by quotation.

### 8g.9 Red first — the named tests and the mutants

Every test below is authored, run, and **watched fail for its own reason** before the code that makes it pass —
golden constraint 12 — and the red output is quoted in the commit that lands it. **Sixty-four new tests**, all under
`--test it`: `rk-types` 9, `rk-kernel` 11, `rk-git` 4, `rk-adapter-toolchain-rust` 2, `rk-cli` 38. Checkpoint 8g
asserts each by its module-qualified name and asserts the exact count per crate; the lists are there and are not
repeated here. The draft's eight handler tests (`registry::`, `sca::`, `package::`, `smoke::` in the Rust adapter)
left this list because the handlers are `s8c`'s record V, and `s8c`'s own `verbs::` tests pin them.

**The mutants**, each authored, run **from Python and never from the shell** (a backtick once swallowed a restore),
and each must turn its named test red. A survivor blocks the close.

| # | mutant | must turn red |
| --- | --- | --- |
| M1 | the pass records the configured list's first toolchain instead of the one `detect()` chose | `verify_pass::verify_completed_names_the_adapter_detect_chose` |
| M2 | `Ok` with `failed > 0` mapped to green | `verify_pass::red_suite_is_a_failed_verify_never_green` |
| M3 | `Timeout` mapped to a failed verify | `verify_pass::ac331_timeout_and_kill_are_unverifiable` |
| M4 | the pass keyed on `resolved.stage == "verify"` | `verify_pass::selected_by_required_kind_not_stage_name` (its fixture pack names the stage `check`) |
| M5 | `verify_completed` written for a killed run | `verify::no_verify_completed_for_a_run_that_did_not_complete` |
| M6 | `paths` dropped from the payload, recomputed from the tree on replay | `verify::ac321_replay_reads_the_path_list_from_verify_completed_alone` |
| M7 | `sca-scan`'s check restored to its v1 expression, without `report.unrated == 0` (a mutant over `s8b`'s edit, killed by this stage's walk) | `demands::scan_unrated_advisory_opens_scan_accept_findings` |
| M8 | `bug-standard.toml`'s `verification_required` row deleted | `verify_pass::ac139_shipped_packs_block_a_missing_verification` |
| M9 | `package_binaries` gains `rk-fixture-toolchain` | `demands::build_artifact_carries_release_digests` (it asserts the exact declared set) |
| M10 | the rollback target defaulted to the landed sha | `tag_release::rollback_target_is_the_previous_release_tag` |
| M11 | the post-run content hash compare removed | `verify_pass::ac338_restored_mutation_is_unverifiable` |
| M12 | the patch applied in the run-owned worktree | `verify_pass::ac339_mutation_runs_in_a_disposable_worktree` |
| M13 | an equal-specificity tie resolved to the first claim | `verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both` |
| M14 | the affected set built by iterating a `HashMap` | `selection::ac348_selection_is_byte_identical_over_200_change_sets` (its generator permutes input order) |
| M15 | `smoke-test`'s `passing` check deleted | `demands::a_failing_smoke_result_fails_the_stage` |
| M16 | `KindSource::Pass` read as `Activity` by the selector | `selection::a_pass_sourced_kind_selects_no_catalog_producer` |
| M17 | the pass calls `test` on a fresh run worktree without `setup` having run | `verify_pass::setup_precedes_the_whole_suite_in_a_fresh_run_worktree` |
| M18 | `unit-tests` restored to `stage = "verify"` | `selection::the_rehomed_test_authoring_activities_select_at_implement` |
| M19 | an agent step added to `smoke-test` | `verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual` |
| M20 | `integration-tests` restored to `stage = "verify"` | `verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual` |
| M21 | `story.toml`'s `external_dep` → `IntegrationReport` row moved back to `[stage.verify]` | `selection::integration_tests_selects_at_implement_under_external_dep` |
| M22 | a `[[stage.verify.require]]` row `when = "external_dep"`, `artifacts = ["MigrationTestReport"]` added to `story.toml` | `selection::migration_dry_run_is_selected_on_no_shipped_lane` |
| M23 | `smoke-test`'s `passing` expression inverted to `smoke.status != 'passing'` | `demands::a_failing_smoke_result_fails_the_stage` (its passing arm) |

### 8g.10 Checkpoint 8g

**What this checkpoint asserts:**

- **Five records exist, each once, in ADR-0085's form.**
  - R3's supersession of ADR-0105 is recorded in both directions.
  - R1 executes N23's re-homing and names the AC-347 residual.
  - R2 records AC-139's fail-closed decision.
- **`verify_completed` is a tier-2 kind** in `rk-types` and in design §5's table, and AC-473's *Verified by* names it.
  AC-341 and AC-139 carry their dated notes.
- **The schema.** `AcceptanceCriterion` is exported and snapshotted, AC-133's registry exists and covers it, and
  `SpecAc` is gone.
- **The verify pass is the one TestReport route.** It is reached by `verbs::run::run`, selected by kind, and runs
  whole-suite through the port over a tree `setup` has prepared. Every id of the verify block has its named test.
- **The re-homing and the policy rows.** The four displaced providers are re-homed to implement over one
  test-authoring kind, and the escalation row moved with them. `integration-tests` is re-homed over the same kind,
  and `story.toml`'s `IntegrationReport` row moved with it (D21-46 R5). `migration-dry-run` is AC-347's single
  residual, and no shipped lane selects it. Both packs ship `verification_required = ["verify"]`.
- **The four demands have producers**, and each writes one `artifact.write`. The producer walk over `s8b`'s `sca-scan` reads `unrated`. The gate
  questions load from `s8d`'s registry, and there is no `.redkiln/questions.toml`. `smoke-test` fails on a
  `SmokeResult` that is not passing (D21-46 R6).
- **`tag-release`** sits in the extended directory and trials under the fixture harness with zero dispatches. **No
  `rk-release/*` tag appears in this repository** as a side effect of the block.
- **The whole suite, fmt and clippy are green.** Checkpoints 5, 6, 7b, 7b-ii, 7d, 8b and 8f re-run whole from their
  stage files' own bytes. The re-runs are bounded, and none of them spawns a real `claude`.

**BEFORE** (PIA-035). Each of these fails on the untouched tree:

- (2) finds no record.
- (3) finds no `verify_completed` row, and no dated note on AC-341 or AC-139.
- (4) finds **17** `SpecAc` sites (re-measured at `a6349b6`).
- (7) finds four `TestReport` providers at `stage = "verify"` and no `verification_required` row.
- (7g) finds `integration-tests` at `stage = "verify"` and version 2, and `smoke-test` at version 1 with no `passing`
  step.

**Checkpoint 7c is not re-run here.** Its live probe is `s8j`'s (§8g.1).

**The floor is re-derived at stage open.** (9)'s `PREV` is Checkpoint 8f's recorded passed count. It is written in by
a dated edit before the block's first run and never lowered to make a red run green. The expected value,
**re-derived at stage open from Checkpoint 8f's recorded count**, is PREV = **1063**, `s8f`'s Checkpoint 8f `FLOOR`
(`s8f` §8f.2 item 9: *"which gives 984 at Checkpoint 8e and 984 + 79 = **1063** here"*), so (9)'s floor is 1063 + 64
= **1127**. The chain, recomputed once under D21-46's floor-chain call, is ADR-0164's 740 passed at `992d257`, plus
`s8a` 9, `s8b` 21, `s8c` 63, `s8d` 52, `s8d-ii` 32, `s8e` 67 and `s8f` 79. The recorded count governs.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`,
# MODULE-QUALIFIED and `--exact`, and asserts each test's own `... ok` line AND an exact count: a bare name
# under `--exact` runs zero tests and exits 0.

# ---- (0) THE WORKTREE PATH SET AND THE RELEASE-TAG SET, read FIRST, before any cargo run.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
tags_before=$(git tag -l 'rk-release/*'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git tag -l could not run (rc=$rc)"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: run 'pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project' first — ten node tests fail at preflight without it and the whole-suite line would be red for a reason unrelated to this stage"; exit 1; }

# ---- (1) BUILD-OR-FAIL, before anything reads target/ (Halt 1).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8g NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE FIVE RECORDS, EACH EXACTLY ONCE, EACH IN ADR-0085's FORM (D21-42 24a). Found by SLUG, because
# ---- this file pre-assigns no number. BEFORE: every glob matches nothing on the untouched tree.
for slug in the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion the-rk-git-write-set-gains-tag-and-apply the-four-verify-and-release-demands-have-producers tag-release-is-the-cli-release-and-enters-by-trial; do
  set -- references/adr/[0-9][0-9][0-9][0-9]-"$slug".md
  [ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: no single record carries the slug $slug"; exit 1; }
  num=${1#references/adr/}; num=${num%%-*}
  grep -qxF "id: \"$num\"" "$1" || { echo "FAIL: $1's frontmatter id is not $num"; exit 1; }
  grep -qxE 'status: (proposed|accepted)' "$1" || { echo "FAIL: $1 is neither proposed nor accepted"; exit 1; }
  n=$(grep -c '^### Option — ' "$1")
  [ "$n" -ge 2 ] || { echo "FAIL: $1 carries $n '### Option — ' headings; ADR-0085 and D21-42 24a need the considered options in that form"; exit 1; }
  grep -qF '**What this forbids**' "$1" || { echo "FAIL: $1 has no What this forbids block"; exit 1; }
done
set -- references/adr/[0-9][0-9][0-9][0-9]-the-rk-git-write-set-gains-tag-and-apply.md
wnum=${1#references/adr/}; wnum=${wnum%%-*}
grep -qE '^supersedes: \[.*"0105".*\]$' "$1" || { echo "FAIL: the write-set record does not list 0105 in supersedes — a new wrapper write without one is ADR-0105's forbids bullet broken"; exit 1; }
grep -qE "^superseded_by: \[.*\"$wnum\".*\]$" references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md || { echo "FAIL: ADR-0105's superseded_by does not carry $wnum — the supersession is recorded in one direction only"; exit 1; }
set -- references/adr/[0-9][0-9][0-9][0-9]-the-verify-pass-is-the-one-test-report-route-and-journals-verify-completed.md
vf="$1"
grep -qF 'applying a mutation patch from `rk-cli`' "$vf" || { echo "FAIL: the verify record does not quote ADR-0162's patch clause — its disposition is stated nowhere (s8g §8g.5)"; exit 1; }
grep -qF 'Re-homed to the implement stage' "$vf" || { echo "FAIL: the verify record does not quote D21-43 N23 — the displaced providers' re-homing is executed nowhere (s8g §8g.2)"; exit 1; }
for a in integration-tests migration-dry-run; do
  grep -qF "$a" "$vf" || { echo "FAIL: the verify record does not name $a — D21-46 R5 re-homes the one and names the other as AC-347's residual (s8g §8g.2, §8g.5)"; exit 1; }
done
grep -qF 'no Phase-1 lane reaches it' "$vf" || { echo "FAIL: the verify record does not quote D21-46 R5 — migration-dry-run's residual carries no reason (s8g §8g.5)"; exit 1; }
set -- references/adr/[0-9][0-9][0-9][0-9]-the-spec-and-pack-schema-fixes-three-spellings-and-exports-acceptance-criterion.md
grep -qF 'Block (fail-closed)' "$1" || { echo "FAIL: the schema record does not quote D21-43 N24 — AC-139's human decision is recorded nowhere (s8g §8g.3)"; exit 1; }

# ---- (3) verify_completed: THE DESIGN ROW, AND AC-473's VERIFIED-BY NAMES IT (ADR-0116 decision 5; D21-42 10b).
# ---- Then the two dated register notes: AC-341's CI-green half is Phase 3's, and AC-139 names N24. The marker and a
# ---- date must sit on ONE line of the AC's own block, so a date elsewhere in the block cannot satisfy it.
grep -qE '^\| `verify_completed` \| 2 \|' references/2026-09-08-architecture-design.md || { echo "FAIL: design §5's event table has no tier-2 verify_completed row — the variant and the row land in one change"; exit 1; }
blk=$(awk '/^\*\*AC-473\*\* /{p=1} p&&/^\*\*AC-521\*\*/{exit} p' references/2026-09-08-acceptance-criteria.md)
test -n "$blk" || { echo "FAIL: AC-473's block could not be read from the register"; exit 1; }
printf '%s\n' "$blk" | grep -qF 'verify_completed' || { echo "FAIL: AC-473 still asserts identity on a dispatch record no toolchain writes (D21-42 10b's register amendment is missing)"; exit 1; }
for spec in "341@Phase 3" "139@N24"; do
  ac=${spec%%@*}; want=${spec#*@}
  blk=$(awk -v s="**AC-$ac** " 'index($0,s)==1{p=1;print;next} p&&/^\*\*AC-[0-9]+\*\* /{exit} p' references/2026-09-08-acceptance-criteria.md)
  test -n "$blk" || { echo "FAIL: AC-$ac's block could not be read from the register"; exit 1; }
  hit=$(printf '%s\n' "$blk" | grep -F "$want")
  test -n "$hit" || { echo "FAIL: AC-$ac's block carries no line naming '$want' (s8g §8g.3, §8g.5)"; exit 1; }
  printf '%s\n' "$hit" | grep -qE '20[0-9]{2}-[0-9]{2}-[0-9]{2}' || { echo "FAIL: AC-$ac's '$want' line is undated — a register amendment carries its date"; exit 1; }
done

# ---- (4) THE SCHEMA: AcceptanceCriterion exported, snapshotted and tracked; AC-133's registry; SpecAc GONE.
snap=crates/rk-types/tests/it/snapshots/exported__AcceptanceCriterion.snap
test -f "$snap" || { echo "FAIL: no AcceptanceCriterion snapshot — the type is not in registry::EXPORTED"; exit 1; }
git ls-files --error-unmatch "$snap" >/dev/null 2>&1 || { echo "FAIL: the snapshot exists and is not tracked"; exit 1; }
pending=$(find crates/rk-types/tests/it/snapshots -name '*.snap.new'); rc=$?
[ "$rc" -eq 0 ] && [ -z "$pending" ] || { echo "FAIL: an unaccepted snapshot is pending — a schema change nobody reviewed:"; printf '%s\n' "$pending"; exit 1; }
# CODE LINES ONLY, anchored to grep's `path:LINE:` prefix. BEFORE: 17 sites at a6349b6.
raw=$(grep -rnE '\bSpecAc\b' crates --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the SpecAc sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: the kernel-local SpecAc survives on a code line (D21-42 10a retires it):"; printf '%s\n' "$code"; exit 1; }
# Synthetic control: the same filter KEEPS a code line and DROPS a comment line, or silence above proves nothing.
ctl=$(printf '%s\n%s\n' 'crates/x/src/a.rs:1:pub struct SpecAc {' 'crates/x/src/a.rs:2:/// SpecAc was here' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: the code-line filter's control kept $ctl of 2 lines, expected exactly the code line"; exit 1; }

# ---- (5) THE NAMED TESTS, per crate, each by its own `... ok` line and an EXACT count.
T_TYPES="event_kinds::verify_completed_is_tier_two_and_wired verify_types::verify_completed_payload_refuses_an_absent_member acceptance_criterion::ac137_criterion_without_gwt_or_verification_is_accepted acceptance_criterion::mutation_declaration_round_trips_declared_and_not_feasible acceptance_criterion::a_prose_only_mutation_is_refused_at_ingest acceptance_criterion::acceptance_criterion_is_in_the_exported_registry judgement_registry::ac133_every_exported_enum_is_registered judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails judgement_registry::the_registry_names_no_enum_that_is_not_exported"
T_KERNEL="verify::verify_completed_is_journalled_once_per_completed_test_port_call verify::no_verify_completed_for_a_run_that_did_not_complete verify::ac321_replay_reads_the_path_list_from_verify_completed_alone verify::ac321_replay_property_over_100_synthetic_ranges verify::ac349_equal_specificity_claims_are_a_configuration_error_naming_both verify::ac347_verify_stage_agent_steps_are_exactly_the_named_residual verify::the_ledger_reads_rk_types_acceptance_criterion selection::a_pass_sourced_kind_selects_no_catalog_producer selection::the_rehomed_test_authoring_activities_select_at_implement selection::integration_tests_selects_at_implement_under_external_dep selection::migration_dry_run_is_selected_on_no_shipped_lane"
T_GIT="gitops::tag_is_on_the_enumerated_write_set gitops::apply_is_on_the_enumerated_write_set gitops::the_write_set_has_no_push gitops::tag_refuses_a_target_off_the_run_owned_branch"
T_RUST="selection::ac348_selection_is_byte_identical_over_200_change_sets selection::ac355_lockfile_change_is_whole_suite_and_one_file_is_affected"
T_CLI="verify_pass::selected_by_required_kind_not_stage_name verify_pass::writes_one_keyed_test_report_with_one_artifact_write verify_pass::verify_completed_names_the_adapter_detect_chose verify_pass::node_detected_fixture_records_the_node_adapter verify_pass::red_suite_is_a_failed_verify_never_green verify_pass::ac331_timeout_and_kill_are_unverifiable verify_pass::ac344_missing_binary_is_cannot_run verify_pass::ac347_the_pass_dispatches_nothing verify_pass::ac320_unresolved_inputs_refuse_before_any_port_call verify_pass::ac338_restored_mutation_is_unverifiable verify_pass::ac339_mutation_runs_in_a_disposable_worktree verify_pass::ac341_the_pass_calls_no_tracker_method verify_pass::ac348_selection_is_recomputed_from_verify_completed verify_pass::ac353_killed_verify_resumes_to_the_same_decision verify_pass::ac359_drift_voids_the_answer_and_recomputes_the_ledger verify_pass::ac485_config_edit_mid_run_does_not_move_the_verify_adapter verify_pass::the_lane_reaches_the_pass_through_verbs_run_run verify_pass::setup_precedes_the_whole_suite_in_a_fresh_run_worktree verify_pass::ac139_shipped_packs_block_a_missing_verification verify_pass::verify_ac_discriminates_answers_through_the_loaded_processes_registry demands::smoke_result_is_produced_through_the_registry_verb demands::a_failing_smoke_result_fails_the_stage demands::confirmation_result_passes_at_the_fix_sha demands::confirmation_not_fixed_opens_its_gate demands::scan_high_finding_opens_scan_accept_findings demands::scan_unrated_advisory_opens_scan_accept_findings demands::scan_clean_emits_without_a_gate demands::build_artifact_carries_release_digests demands::every_restored_demand_writes_one_artifact_write tag_release::release_record_tags_the_landed_commit_locally tag_release::rollback_target_is_the_previous_release_tag tag_release::first_release_names_its_rollback_target_as_none tag_release::release_record_carries_the_build_artifact_digest tag_release::the_tag_is_never_pushed tag_release::trial_runs_one_activity_with_zero_dispatches tag_release::an_enabled_pack_referencing_the_extended_activity_is_refused tag_release::selection_picks_tag_release_over_the_deploy_patterns doctor::ac349_equal_specificity_claim_is_a_doctor_row"
NEW=0
for spec in "rk-types@9@$T_TYPES" "rk-kernel@11@$T_KERNEL" "rk-git@4@$T_GIT" "rk-adapter-toolchain-rust@2@$T_RUST" "rk-cli@38@$T_CLI"; do
  crate=${spec%%@*}; rest=${spec#*@}; want=${rest%%@*}; names=${rest#*@}
  got=$(printf '%s\n' $names | grep -c .)
  [ "$got" -eq "$want" ] || { echo "FAIL: $crate's list carries $got names, this file says $want — the list and the prose disagree"; exit 1; }
  out=$(cargo test -p "$crate" --test it -- $names --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $crate's named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in $names; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $crate: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $crate's selection did not run exactly $want tests — a filter that matches nothing is a green"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
  NEW=$((NEW + want))
done
[ "$NEW" -eq 64 ] || { echo "FAIL: $NEW named tests ran, §8g.9 says sixty-four"; exit 1; }

# ---- (6) NO STAGE-NAME LITERAL IN rk-cli/src (ADR-0162 decision 5) — a FLOOR over CODE lines beneath the
# ---- behavioural `selected_by_required_kind_not_stage_name` above, whose fixture names its verify stage `check`.
raw=$(grep -rnE 'stage[[:space:]]*(==|!=)[[:space:]]*"|"[[:space:]]*(==|!=)[[:space:]]*[A-Za-z_.]*stage\b' crates/rk-cli/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the stage-literal sweep could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: resolved.stage compared to a string literal on a CODE line (ADR-0162 decision 5):"; printf '%s\n' "$code"; exit 1; }
ctl=$(printf '%s\n' 'crates/rk-cli/src/verify_pass.rs:9:    if resolved.stage == "verify" {' | grep -E 'stage[[:space:]]*(==|!=)[[:space:]]*"' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: the stage-literal pattern does not catch its own synthetic control"; exit 1; }

# ---- (7) THE SHIPPED FILES THIS STAGE AUTHORS OR EDITS, where the records put them. Each is a FLOOR beneath a
# ---- named behavioural test in (5); none of them is the assertion.
test -f .redkiln/catalog-extended/tag-release.toml || { echo "FAIL: tag-release is not in the extended directory (ADR-0114 decision 3)"; exit 1; }
test ! -e .redkiln/catalog/tag-release.toml || { echo "FAIL: tag-release is in the SHIPPED catalog before a trial ran and a promotion decision cited it (ADR-0114 decision 3; AC-110)"; exit 1; }
test -f .redkiln/checklists/smoke.toml || { echo "FAIL: no smoke checklist — checklist:smoke resolves to nothing"; exit 1; }
# (7a) The questions: s8d's registry, and no second file (D21-43's integration call).
test ! -e .redkiln/questions.toml || { echo "FAIL: .redkiln/questions.toml exists — question loading is s8d's, under .redkiln/processes/ (D21-43)"; exit 1; }
set -- .redkiln/processes/*.toml
[ -f "$1" ] || { echo "FAIL: .redkiln/processes/ holds no .toml — s8d's registry is absent (Halt 8)"; exit 1; }
for q in confirmation.not-fixed scan.accept-findings verify.ac-discriminates; do
  n=$(grep -hxF "[question.\"$q\"]" .redkiln/processes/*.toml | grep -c .)
  [ "$n" -eq 1 ] || { echo "FAIL: question $q is declared $n times under .redkiln/processes/, not once — the gate that opens it could not be answered through the loaded registry"; exit 1; }
done
# (7b) The release goals (R5) and AC-139's policy rows (D21-43 N24; R2).
grep -qxF 'goal = "The fix is tagged for release, reversibly"' .redkiln/packs/bug-standard.toml || { echo "FAIL: bug-standard's release goal is not the amended wording (R5)"; exit 1; }
grep -qxF 'goal = "Tagged for release, reversibly"' .redkiln/packs/story.toml || { echo "FAIL: story's release goal is not the amended wording (R5)"; exit 1; }
for p in bug-standard story; do
  n=$(grep -cE '^goal = ".*[Pp]roduction' ".redkiln/packs/$p.toml")
  [ "$n" -eq 0 ] || { echo "FAIL: $p.toml still promises production, which a run that pushes nothing cannot reach"; exit 1; }
  pol=$(awk '/^\[policy\]$/{p=1;next} /^\[/{p=0} p' ".redkiln/packs/$p.toml")
  test -n "$pol" || { echo "FAIL: $p.toml has no [policy] table to read"; exit 1; }
  printf '%s\n' "$pol" | grep -qxF 'verification_required = ["verify"]' || { echo "FAIL: $p.toml's [policy] does not ship AC-139's fail-closed default (D21-43 N24; R2)"; exit 1; }
done
# (7c) The changelog (s8b's), with a row for every activity and pack edit this stage makes (ADR-0072).
test -f .redkiln/CHANGELOG.md || { echo "FAIL: s8b's pack changelog is absent — ADR-0072's row for this stage's pack and catalog edits has nowhere to go"; exit 1; }
for w in tag-release 'tagged for release' smoke-test unit-tests-ue equivalence-boundary property-based-testing verification_required; do
  grep -qiF "$w" .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md carries no row naming '$w' (ADR-0072; R1, R2, R4, R5)"; exit 1; }
done
for pair in integration-tests@3 smoke-test@2; do
  grep -qF "$pair" .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md carries no row naming $pair in s8b's <id>@<new version> form (ADR-0072; D21-46 R5, R6)"; exit 1; }
done
# (7d) N23: the four displaced providers, re-homed to implement over ONE test-authoring kind listed first.
kinds=""
for a in unit-tests unit-tests-ue equivalence-boundary property-based-testing; do
  f=.redkiln/catalog/$a.toml
  test -f "$f" || { echo "FAIL: $f is absent — N23 re-homes the activity, it does not delete it"; exit 1; }
  grep -qxE 'stage[[:space:]]*=[[:space:]]*"implement"([[:space:]]*#.*)?' "$f" || { echo "FAIL: $a is not re-homed to the implement stage (D21-43 N23)"; exit 1; }
  prov=$(grep -E '^provides[[:space:]]*=' "$f")
  test -n "$prov" || { echo "FAIL: $a declares no provides"; exit 1; }
  n=$(printf '%s\n' "$prov" | grep -cF '"TestReport"')
  [ "$n" -eq 0 ] || { echo "FAIL: $a still provides TestReport beside the pass — two routes (s8g §8g.2)"; exit 1; }
  ver=$(sed -n 's/^version[[:space:]]*=[[:space:]]*\([0-9][0-9]*\).*/\1/p' "$f")
  [ -n "$ver" ] && [ "$ver" -ge 3 ] || { echo "FAIL: $a's version is '$ver', not bumped past 2 (ADR-0072: an edited activity is a new version)"; exit 1; }
  k=$(printf '%s\n' "$prov" | sed -n 's/^provides[[:space:]]*=[[:space:]]*\["\([A-Za-z0-9]*\)".*/\1/p')
  test -n "$k" || { echo "FAIL: $a's first provided kind could not be read from: $prov"; exit 1; }
  kinds="$kinds $k"
done
n=$(printf '%s\n' $kinds | sort -u | grep -c .)
[ "$n" -eq 1 ] || { echo "FAIL: the four re-homed activities list $n distinct first kinds ($kinds), not one test-authoring kind"; exit 1; }
K=$(printf '%s\n' $kinds | sort -u)
row=$(awk -v h="[artifact.$K]" '$0==h{p=1;next} /^\[/{p=0} p' .redkiln/artifacts.toml)
printf '%s\n' "$row" | grep -qxF 'source = "activity"' || { echo "FAIL: the test-authoring kind $K has no source = \"activity\" row in artifacts.toml"; exit 1; }
for p in bug-standard story; do
  imp=$(awk '/^\[\[?stage\.implement(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' ".redkiln/packs/$p.toml")
  printf '%s\n' "$imp" | grep -qxF "artifacts = [\"$K\"]" || { echo "FAIL: $p.toml's implement stage carries no require row over $K — the test-design escalation did not move with its activities (N23)"; exit 1; }
  vrf=$(awk '/^\[\[?stage\.verify(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' ".redkiln/packs/$p.toml")
  test -n "$vrf" || { echo "FAIL: $p.toml's verify stage could not be read"; exit 1; }
  n=$(printf '%s\n' "$vrf" | grep -cxF 'artifacts = ["TestReport"]')
  [ "$n" -eq 0 ] || { echo "FAIL: $p.toml's verify stage still escalates TestReport's rigor — a floor on a pass-sourced kind selects nothing (N23)"; exit 1; }
done
# (7e) ENTRY PRECONDITIONS this stage reads and does not author: s8b's N25 edit to sca-scan, and s8c's setup_projects.
grep -qE '^expr[[:space:]]*=[[:space:]]*".*report\.unrated == 0' .redkiln/catalog/sca-scan.toml || { echo "FAIL: sca-scan's check does not require unrated == 0 — s8b's N25 edit is absent (Halt 8)"; exit 1; }
test -f .redkiln/registry.toml || { echo "FAIL: s8c's registry.toml is absent (Halt 8)"; exit 1; }
grep -qF 'crates/rk-adapter-toolchain-node/tests/fixtures/project' .redkiln/registry.toml || { echo "FAIL: s8c's setup_projects row does not name the Node fixture project — a run worktree's whole suite fails ten tests at preflight (D21-43 N25; s8c §8c.5)"; exit 1; }
# (7f) TestReport is pass-sourced.
grep -qE '^\[artifact\.TestReport\]' .redkiln/artifacts.toml || { echo "FAIL: artifacts.toml has no TestReport row"; exit 1; }
tr_row=$(awk '/^\[artifact\.TestReport\]/{p=1;next} /^\[/{p=0} p' .redkiln/artifacts.toml)
printf '%s\n' "$tr_row" | grep -qxF 'source = "pass"' || { echo "FAIL: TestReport is not pass-sourced — the catalog's providers would still be selected beside the pass (two routes)"; printf '%s\n' "$tr_row"; exit 1; }
# (7g) D21-46 R5: integration-tests re-homed to implement over the SAME test-authoring kind $K, IntegrationReport kept,
# and story.toml's IntegrationReport row moved with it. D21-46 R6: smoke-test's passing check, in step order.
f=.redkiln/catalog/integration-tests.toml
test -f "$f" || { echo "FAIL: $f is absent — D21-46 R5 re-homes the activity, it does not delete it"; exit 1; }
grep -qxE 'stage[[:space:]]*=[[:space:]]*"implement"([[:space:]]*#.*)?' "$f" || { echo "FAIL: integration-tests is not re-homed to the implement stage (D21-46 R5)"; exit 1; }
grep -qE '^version[[:space:]]*=[[:space:]]*3([[:space:]]|$)' "$f" || { echo "FAIL: integration-tests is not at version 3 (ADR-0072; D21-46 R5)"; exit 1; }
prov=$(grep -E '^provides[[:space:]]*=' "$f")
k=$(printf '%s\n' "$prov" | sed -n 's/^provides[[:space:]]*=[[:space:]]*\["\([A-Za-z0-9]*\)".*/\1/p')
[ "$k" = "$K" ] || { echo "FAIL: integration-tests lists '$k' first, not the test-authoring kind $K (D21-46 R5, as N23): $prov"; exit 1; }
printf '%s\n' "$prov" | grep -qF '"IntegrationReport"' || { echo "FAIL: integration-tests no longer provides IntegrationReport — story.toml's external_dep row would have no producer"; exit 1; }
imp=$(awk '/^\[\[?stage\.implement(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' .redkiln/packs/story.toml)
printf '%s\n' "$imp" | grep -qxF 'artifacts = ["IntegrationReport"]' || { echo "FAIL: story.toml's implement stage carries no IntegrationReport row — the require row did not move with its activity (D21-46 R5)"; exit 1; }
vrf=$(awk '/^\[\[?stage\.verify(\.require)?\]\]?$/{p=1;next} /^\[/{p=0} p' .redkiln/packs/story.toml)
test -n "$vrf" || { echo "FAIL: story.toml's verify stage could not be read"; exit 1; }
n=$(printf '%s\n' "$vrf" | grep -cxF 'artifacts = ["IntegrationReport"]')
[ "$n" -eq 0 ] || { echo "FAIL: story.toml's verify stage still requires IntegrationReport, whose only producer now sits at implement (D21-46 R5)"; exit 1; }
f=.redkiln/catalog/smoke-test.toml
grep -qE '^version[[:space:]]*=[[:space:]]*2([[:space:]]|$)' "$f" || { echo "FAIL: smoke-test is not at version 2 (ADR-0072; D21-46 R6)"; exit 1; }
order=$(sed -n 's/^id[[:space:]]*=[[:space:]]*"\([A-Za-z0-9_-]*\)".*/\1/p' "$f" | tr '\n' ' ')
[ "$order" = "smoke-test build smoke passing emit " ] || { echo "FAIL: smoke-test's ids read '$order', not the activity then build, smoke, passing, emit (D21-46 R6)"; exit 1; }
stp=$(awk '/^\[\[step\]\]$/{p=0} /^id[[:space:]]*=[[:space:]]*"passing"/{p=1} p' "$f")
test -n "$stp" || { echo "FAIL: smoke-test's passing step could not be read"; exit 1; }
printf '%s\n' "$stp" | grep -qE '^kind[[:space:]]*=[[:space:]]*"check"' || { echo "FAIL: smoke-test's passing step is not a check"; exit 1; }
printf '%s\n' "$stp" | grep -qF "\"smoke.status == 'passing'\"" || { echo "FAIL: smoke-test's passing check does not read smoke.status == 'passing' (D21-46 R6)"; exit 1; }
printf '%s\n' "$stp" | grep -qE '^on_false[[:space:]]*=[[:space:]]*"fail"' || { echo "FAIL: smoke-test's passing check does not fail on false (D21-46 R6)"; exit 1; }

# ---- (8) THE COLD SLOWEST-CHILD FIGURE IS IN THE VERIFY RECORD, cited from s8c, and under ADR-0140's bound.
secs=$(sed -n 's/.*cold cargo test --workspace slowest child: \([0-9][0-9]*\) s.*/\1/p' "$vf" | head -1)
test -n "$secs" || { echo "FAIL: the verify record carries no cold slowest-child line in the form §8g.2 fixes"; exit 1; }
[ "$secs" -lt 3600 ] || { echo "FAIL: a cold child takes $secs s, at or over ADR-0140's 3600 s — every cold verify pass would be Unverifiable (golden constraint 6; halt)"; exit 1; }

# ---- (9) THE WHOLE SUITE, --no-fail-fast, rc 0, every result line green, and a FLOOR: Checkpoint 8f's recorded
# ---- passed count plus this stage's 64. Expected value, re-derived at stage open from Checkpoint 8f's recorded count:
# ---- PREV = 1063, so the floor is 1063 + 64 = 1127. Edit PREV by a dated edit of the next line before the first run.
# ---- A lower bound, never an equality. At least 44 result lines, as at 992d257.
PREV=1063
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the workspace suite is red (rc=$rc)"; printf '%s\n' "$out" | grep -E '^test .* FAILED$|panicked' | head -20; exit 1; }
results=$(printf '%s\n' "$out" | grep -c '^test result:')
greens=$(printf '%s\n' "$out" | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed')
[ "$results" -ge 44 ] && [ "$results" -eq "$greens" ] || { echo "FAIL: $greens of $results result lines are green, over a floor of 44 result lines"; exit 1; }
passed=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9][0-9]*\) passed.*/\1/p' | awk '{s+=$1} END {print s+0}')
[ "$passed" -ge $((PREV + NEW)) ] || { echo "FAIL: $passed passed, under the floor of $((PREV + NEW)) (Checkpoint 8f's $PREV + this stage's $NEW)"; exit 1; }

# ---- (10) fmt AND clippy, each by its own exit code.
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --all --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^error' | head -20; exit 1; }

# ---- (11) NO RELEASE TAG WAS MADE IN THIS REPOSITORY BY THE BLOCK — the tag tests run in temporary repositories.
tags_after=$(git tag -l 'rk-release/*'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git tag -l could not run after the suite (rc=$rc)"; exit 1; }
[ "$tags_before" = "$tags_after" ] || { echo "FAIL: the block created or removed an rk-release tag in the operator's repository:"; diff <(printf '%s\n' "$tags_before") <(printf '%s\n' "$tags_after"); exit 1; }

# ---- (12) THE ENTRY GATE AND THE PROVISIONAL CHECKPOINTS, RE-RUN WHOLE FROM THEIR STAGE FILES' OWN BYTES (§8g.1):
# ---- 5 (rk-types), 6 (rk-git), 7b and 7b-ii (rk-kernel), 7d (rk-cli), 8b (.redkiln/), and 8f, the entry gate.
# ---- Extracted, never retyped. A block with no end marker gets one appended, so rc=0 can be told apart from a block
# ---- that stopped early. The extractor writes under target/ — Git Bash's /tmp is invisible to Windows Python. Store
# ---- handling around `doctor --rebuild` is each block's own (the s8 amendment), and nothing below reads the store.
# ---- BOUNDED: every replay runs with RK_CP_NESTED=1, and when THIS block runs under RK_CP_NESTED=1 it replays
# ---- nothing. NO DISPATCH: a replayed block that invokes `redkiln run` on a code line must name harness_binary, and
# ---- every replay runs with REDKILN_LAUNCH_RECORD unset and CLAUDE_CONFIG_DIR at an empty trap directory that must
# ---- still be empty afterwards (D21-46's one no-dispatch guard for replays).
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8G (12) SKIPPED: this block is itself a replay (RK_CP_NESTED=1); its re-runs ran at Checkpoint 8g's own close"
else
  mkdir -p target/cp8g || { echo "FAIL: could not create target/cp8g"; exit 1; }
  trap_dir="$PWD/target/cp8g/claude-config-trap"
  rm -rf "$trap_dir" && mkdir -p "$trap_dir" || { echo "FAIL: could not create the CLAUDE_CONFIG_DIR trap"; exit 1; }
  # Synthetic control: the emptiness reading sees one file, or its silence afterwards proves nothing.
  : > "$trap_dir/control" || { echo "FAIL: could not write the trap's control file"; exit 1; }
  n=$(find "$trap_dir" -mindepth 1 | grep -c .)
  [ "$n" -eq 1 ] || { echo "FAIL: the trap's emptiness reading saw $n entries where one was written"; exit 1; }
  rm -f "$trap_dir/control" || { echo "FAIL: could not clear the trap's control file"; exit 1; }
  n=$(find "$trap_dir" -mindepth 1 | grep -c .)
  [ "$n" -eq 0 ] || { echo "FAIL: the trap does not start empty ($n entries) — its emptiness afterwards would prove nothing"; exit 1; }
  RUNPAT='(\$RK|\$BIN|redkiln(\.exe)?)"?[[:space:]]+run([[:space:]]|$)'
  n=$(printf '%s\n' '"$RK" run "$ITEM" --lane fix' '# "$RK" run in a comment' | grep -vE '^[[:space:]]*#' | grep -cE "$RUNPAT")
  [ "$n" -eq 1 ] || { echo "FAIL: the run-invocation pattern's control matched $n of the one code line"; exit 1; }
  EXTRACT='import sys
F = chr(96) * 3
src, mode, key, out = sys.argv[1:5]
lines = open(src, encoding="utf-8").read().split("\n")
blocks = []
i = 0
while i < len(lines):
    if lines[i] == F + "bash":
        j = i + 1
        while j < len(lines) and lines[j] != F:
            j += 1
        blocks.append((i, lines[i + 1:j]))
        i = j + 1
    else:
        i += 1
if mode == "marker":
    hit = [b for (s, b) in blocks if any(key in l for l in b)]
else:
    at = [n for n, l in enumerate(lines) if key in l]
    hit = [b for (s, b) in blocks if at and s > at[0]][:1]
if len(hit) != 1:
    print("EXTRACT-FAIL", len(hit))
    sys.exit(3)
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(hit[0]) + "\n")
print("EXTRACTED", len(hit[0]))'
  for spec in "cp5@runbook/phase-1/s5-workspace-ci-rk-types.md@anchor@**Checkpoint 5.** All of:" "cp6@runbook/phase-1/s6-store-and-git.md@anchor@**Checkpoint 6.** All of:" "cp7b@runbook/phase-1/s7b-kernel.md@anchor@### 7b.14 Checkpoint 7b" "cp7bii@runbook/phase-1/s7b-ii-gates.md@anchor@### 7b-ii.13 Checkpoint 7b-ii" "cp7d@runbook/phase-1/s7d-cli-and-migrate.md@marker@CP7D-END-OF-BLOCK reached" "cp8b@runbook/phase-1/s8b-catalog-quality.md@marker@CP8B-END-OF-BLOCK reached" "cp8f@runbook/phase-1/s8f-landing-and-rendering.md@marker@CP8F-END-OF-BLOCK reached"; do
    name=${spec%%@*}; rest=${spec#*@}; file=${rest%%@*}; rest=${rest#*@}; mode=${rest%%@*}; key=${rest#*@}
    test -f "$file" || { echo "FAIL: $file does not exist — Checkpoint $name cannot be re-run (Halt 8)"; exit 1; }
    ex=$(python -c "$EXTRACT" "$file" "$mode" "$key" "target/cp8g/$name.sh" 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: could not extract exactly one block for $name: $ex"; exit 1; }
    runs=$(grep -vE '^[[:space:]]*#' "target/cp8g/$name.sh" | grep -E "$RUNPAT")
    [ -z "$runs" ] || grep -qF 'harness_binary' "target/cp8g/$name.sh" || { echo "FAIL: Checkpoint $name invokes redkiln run and names no harness_binary — its replay could dispatch to a real account:"; printf '%s\n' "$runs"; exit 1; }
    printf '\necho "CP8G-REPLAY-%s-END reached"\n' "$name" >> "target/cp8g/$name.sh" || { echo "FAIL: could not append $name's marker"; exit 1; }
    out=$(env -u REDKILN_LAUNCH_RECORD RK_CP_NESTED=1 CLAUDE_CONFIG_DIR="$trap_dir" bash "target/cp8g/$name.sh" 2>&1); rc=$?
    [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $name re-run whole exited $rc — its row stays provisional"; printf '%s\n' "$out" | grep -E 'FAIL' | head -10; printf '%s\n' "$out" | tail -15; exit 1; }
    printf '%s\n' "$out" | grep -qxF "CP8G-REPLAY-$name-END reached" || { echo "FAIL: Checkpoint $name exited 0 without reaching its last line"; exit 1; }
  done
  left=$(find "$trap_dir" -mindepth 1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the trap directory could not be read after the replays (rc=$rc)"; exit 1; }
  [ -z "$left" ] || { echo "FAIL: something wrote into the trap CLAUDE_CONFIG_DIR during the replays — a real claude was spawned:"; printf '%s\n' "$left" | head -10; exit 1; }
fi

# ---- (13) THE WORKTREE PATH SET IS UNCHANGED, compared BEFORE any prune; the prune is a dry run and finds nothing.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET (the pass's ephemeral trees must be removed on every path — ADR-0162 decision 10):"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }

echo "CP8G-END-OF-BLOCK reached"
```

⚠️ **What this block cannot see, stated rather than claimed away.**

- **(6) is a floor over code lines.** A stage literal assembled by a `const`, a `matches!` over a pattern, or a helper
  whose name hides the comparison passes it. That is why the behavioural test with a renamed stage is the assertion.
- **(7) reads the catalog and packs as text.** A kind listed second, a require row written as an inline table, or a
  `provides` spread over two lines reads wrong. Each of those lines is a floor beneath a named test in (5), and a
  shape it cannot read fails, which is the safe direction.
- **(9)'s floor is a lower bound.** It cannot tell a deleted older test from a new one. The named lists are what pin
  this stage's tests.
- **(12) replays with the environment the operator gives this block, plus the two variables it sets and the one it
  unsets.** If 8f needs a
  variable, its own guard names it and this block fails, which is the right direction.
- **(12)'s trap has limits.** It catches a real `claude` that inherits `CLAUDE_CONFIG_DIR`. A child launched with that
  variable set explicitly escapes it. From `s8h` on, that path is closed by `s8h`'s launch-record check: a real-harness run
  without a launch record refuses before it spawns anything (`s8h`'s launcher; the exact sentence is `s8h`'s to fix). The static `harness_binary` guard reads
  code lines only, so a run assembled from a variable passes it.
- **`setup` is not exercised end to end here.** It runs for real on a fresh run worktree only in `s8j`'s fixture
  proof of the lane, where a whole verify runs inside a real run worktree. Here, (5)'s `setup_precedes_…` asserts the
  order over a double, and (7e) asserts the declaration.

### 8g.11 Acceptance set

**Claimed here, each by the named test in §8g.10 (5):**

- **The verify block**: AC-320 (with its dated *Verified by* note), AC-321, AC-338, AC-339, AC-348, AC-349, AC-353
  and AC-359, which is **seven of the verify block whole**, and AC-355 on its fixtures (its *"held in CI"* clause is
  `s8j`'s 14a).
- **AC-341** on the port-call half. Its static half is cross-listed to 7b-ii.
- **Beside the block**: AC-331; AC-344's `cannot_run` mapping; AC-347's zero-dispatch half and its shipped-catalog
  lint, with the lint's single residual, `migration-dry-run`, named and its unreachability asserted (D21-46 R5).
- **AC-137.**
- **AC-133.** Its instrument is built here for the first time. `s5`'s claim is corrected, not re-claimed, and `s8h`
  registers into the same registry.
- **AC-485's toolchain identity half.**

**Named with a reason, not claimed:**

| id | reason |
| --- | --- |
| AC-341's CI-green integration half | the `Tracker` port has no status surface. It is Phase 3's (AC-508), recorded by a dated register note |
| AC-357 | HUMAN GATE. Its machine part is exercised inside `ac359_…` |
| AC-114 | HUMAN GATE. The promotion and its `curation.decision` lint are `s8j`'s |
| AC-139 | HUMAN GATE, **decided 2026-09-24** by D21-43 N24 (*"Block (fail-closed)"*). R2 records it, and both packs ship it as a `[policy]` row. A human decision is recorded, never claimed by a test |
| AC-350 and AC-317 | ARMED, as before |
| AC-473 and AC-521 | Step 8's release checks. This stage moves AC-473's *Verified by* and claims neither |
| AC-347 over `migration-dry-run` | the single named residual (D21-46 R5): one agent step at verify, on an activity no Phase-1 lane reaches (§8g.5). A row that makes it reachable turns `selection::migration_dry_run_is_selected_on_no_shipped_lane` red |

**Cross-listed:**

- AC-328's whole-suite policy. The adapter side is `s8c`'s.
- AC-455. It was discharged at 7b-ii, and is re-pointed here to `rk_types::AcceptanceCriterion` with its two policy
  rows unchanged.
- AC-138, re-proven over the shipped rows by `ac139_shipped_packs_block_a_missing_verification`'s row-removed arm.
- AC-110, unchanged. `tag_release::an_enabled_pack_referencing_the_extended_activity_is_refused` asserts it.

**Owed and reported upward, because no sub-stage owns them:**

- **The `curation.decision` backfill for the 102 shipped activities.** None of them carries a decision id, and
  AC-114's lint (`s8j`'s) will meet them on its first run (§8g.7).
- **N23's *"Verify only measures"* holds over every reachable verify activity, and not over the shipped catalog.**
  D21-46 R5 re-homed `integration-tests` and left `migration-dry-run` as the named residual, unreachable while
  `data_migration` is NOT MEASURABLE (§8g.5). Making that facet measurable, or adding any row that requires
  `MigrationTestReport`, reopens the question for whichever stage does it.
- **`unit-tests-ue` authors nothing.** It is re-homed as a test-authoring activity by N23's letter (§8g.2).
- **`s8h`'s `Spec` must embed `AcceptanceCriterion` unchanged** (§8g.3).

**Handed to `s8j`, by name:**

- the tag-release real trial, AC-114's promotion and its lint (§8g.7);
- Checkpoint 7c's whole re-run (D21-43 N28; §8g.1);
- `verify_completed`'s `item` and `adapter` member names, for its fixture proof (§8g.4).
