# Step 8h — Cards, models and the run environment (Phase 1, part 8, sub-stage h)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8h` | Phase 1 | Checkpoint 8g | Checkpoint 8h | `s8g` | `s8i` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (HEAD `a6349b6`). What changed:
- §8h.0's four questions are closed as D21-43 N17 took them. Q1 reuses `NotAdopted` and `NotIsolated { surface: Env }`,
  and nothing is superseded, so Record A no longer supersedes ADR-0095. Q2 is the four `Usage` counters,
  Q3 is `Exit::Overage` (20), and Q4 is three probe-2 dispatches.
- D21-44 settled §8h.6's `.git` risk: run worktrees move to `.redkiln/worktrees/<item>`, which is `s8d-ii`'s to build
  (D21-46 R1 moved it out of `s8e`).
  Probe 1 now makes its worktree there, and it passes only if an edit inside that worktree succeeds. D21-45's
  protected-segment limit (`<wt>/.git`, `.cargo`, `.claude`, `.husky`) is stated and mapped to the typed
  `permission_denials` failure, with one new test, one mutant and one probe arm.
- AC-133's registry is `s8g`'s. This stage enters its types' enums in it and builds no second registry. Three
  `rk-types` tests are dropped, and `s8g`'s two are selected as regressions.
- The launcher gains a `wrap` mode, so that `s8i`'s corpus runner and `s8j`'s whole Checkpoint 7c can run through
  it. It also refuses `CLAUDE_PID` and `CLAUDE_EFFORT`. The live probes run from a plain terminal.
- §8h.10 carries rows for the gate kinds that earlier sub-stages mint (`s8f`'s `landing` and D21-43 N11's drift
  gate).
- The whole-suite floor is re-derived from Checkpoint 8g's recorded count. Checkpoint 8h re-runs Checkpoint 8g and
  the four older checkpoints its crate edits make provisional. It says where 7c, 7f and 8a are re-run, and why 7e
  cannot be.
- `s8`'s three `redkiln run` sites are `s8j`'s amendment (D21-43). This stage states the constraint and edits
  nothing there.
- New findings for the lead: 10–15 at the end of the file.

*Revised 2026-09-24 for D21-46* (HEAD `c5d19d7`; D21-46 is the authority). What changed:
- **The launcher gains an `answer` mode.** D21-46's integration call: *"s8h's launcher gains an **`answer` mode**, so
  s8j's hold answer is not refused for want of a launch record"*. §8h.7 specifies it: `run`'s environment refusals,
  `run`'s pin comparison, argument refusals of its own, and the same launch record. The launch check's scope on
  `answer` is restated precisely. The draft's *"`answer` where it continues the lane"* named a condition that no code
  has, because `verbs/answer.rs` names no harness at `c5d19d7` [measured: `grep -n -i 'harness'
  crates/rk-cli/src/verbs/answer.rs` prints nothing]. Two `rk-cli` tests and three mutants are added.
- **`wrap`'s guarantee is stated once, which resolves the self-contradiction `s8j` §8j.6 found.** The draft called
  `wrap` the entry for a child that does not dispatch through `rk-adapter-claude`, and then named Checkpoint 7c, whose
  probe does, as a caller. `wrap` now guarantees the refusals, the launch record and before/after version equality,
  and it never compares to the adapter pin. A `wrap` child that dispatches through the adapter carries its own pin
  check, as 7c's block does.
- **§8h.10's table names every gate class the `s8` stages mint, one explicit row each.** That is `s8f`'s two,
  `s8d`'s `unknown_facet:change`, `s8j`'s two human-root question gates, and `s8j`'s hold gate. `MINTED_CLASSES` is
  still re-read at stage open from the code.
- **`RK_CP_NESTED=1` and the replay no-dispatch guard are adopted** in Checkpoint 8h (13), in `s8g`'s form. D21-46:
  *"One re-run convention"* and *"One no-dispatch guard for replays"*.
- The counts move to 41 new tests (was 39), 17 new in `rk-cli` (was 15), and twenty mutants (was seventeen). New
  findings for the lead are 16–19.

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **The run-worktree placement and lifecycle are `s8d-ii`'s** (D21-46 R1), and `s8e` keeps the `worktree_assigned`
  row. Every sentence that gave the placement to `s8e` now names `s8d-ii`.
- **§8h.0 item 4's count source** is `s8e` §8e.10's interim `item` key, carried beside `dispatch_completed`'s payload
  under D21-46 R3.
- **§8h.0 item 8's floor** is recomputed once: Checkpoint 8g's 1127 + this stage's 41 = **1168**, not the drafted
  1000 + 41. Its `63 (8g)` is `s8g`'s 64.
- **§8h.10 names `s8j`'s three gate classes by the spellings `s8j` drafts** (`s8j` §8j.2, §8j.3).
- **§8h.14's 7f route** reads 7f as `s8d-ii` §8d-ii.7 left it: the run arm is in a throwaway clone and the cleanup line
  is gone, so the legacy tree is not re-scoped there. The 8a row quotes `s8a`'s guard from its current bytes.
- **Findings 16 and 17 are settled**: `s8i` §8i.5 and `s8j` §8j.6 re-quote §8h.7's current `wrap` text, and `s8j`'s
  screen matches the `answer` mode. The trap in (13) asserts that it starts empty.

---

**Why this step exists.** D21-40 decision 1 names four structural reasons that criterion (1) cannot be satisfied.
The last of them is this stage's: *"the shipped registry cannot bind a dispatch (0 of 8 cards carry
`permission_mode`, all 8 grant only `StructuredOutput`, no `.redkiln/schemas/`, 0 `model` rows)"*. Each clause was
re-measured at `ca8f387` on 2026-09-24:

- **The cards.** All eight `.redkiln/cards/*.json` hold exactly the keys `cache_profile`, `description`, `prompt` and
  `tools`, and every `tools` is `["StructuredOutput"]` (read with `json.load`, not grep). None carries
  `permission_mode`, so `AuthoredCard::from_file` refuses every one with `PermissionModeAbsent`. The only check on the
  mode's value is `card.rs`'s `if permission_mode.trim().is_empty() {`, so a misspelt `"acceptEdit"` would bind.
- **The models.** Both packs' eight `[role.*]` tables each carry one key, `family = "claude"`. `RoleExtras.model` is
  `Option<rk_adapter_claude::card::Tier>`, and `Tier` is the closed lifted enum `Haiku | Sonnet | Opus`.
  `--model` receives the tier word. Adoption is read through `init.rs`'s
  `reported == tier || reported.split('-').any(|seg| seg == tier)`, which is a segment match: a reported
  `claude-sonnet-4-6` "belongs" to a requested `sonnet`. `init.rs` says so itself: *"A record settling the reading is
  STILL OWED"*. `DispatchRecord` has no model member.
- **The schemas.** `bind_nodes` reads `.join("schemas")` then `.join(format!("{kind}.json"))`, where
  `let kind = activity.provides.first().cloned().unwrap_or_default();`. That directory does not exist. The binder's
  own module doc says the schema comes *"from `rk_types::registry::EXPORTED` by name"*, and
  `pub const EXPORTED: &[Exporter] = &[("Findings", || {` holds exactly one entry.
- **Principle 7.** Design principle 7 forbids *"inferring from the parent side that a flag took effect — an ignored
  flag looks identical to an honoured one"*. Nothing in `rk-adapter-claude/src` reads `permissionMode`,
  `apiKeySource` or `permission_denials` back from the child. `command.rs` emits no `--setting-sources` and no
  `--settings`, so the child discovers the operator's user settings. Those settings carry `enabledPlugins` and
  `autoMode` [measured 2026-09-24: the key NAMES of `C:\Users\ryanm\.claude-personal\settings.json`; no value was
  read, and there is no `permissions` key].
- **The pin.** `pub const TESTED_CLI_VERSION: &str = "2.1.278";` and `RUNBOOK-instrument.txt` reads
  `2.1.278 (Claude Code)`, but `claude --version` prints `2.1.281 (Claude Code)`. **Halt 2 is live.**
  `~/.local/bin/` holds `claude.exe` (2026-09-23) beside `claude.exe.old.1790192683866` (2026-09-22). That is the
  native updater's rename, so the instrument moved under the tree between sessions.
- **The environment.** No launcher exists. The shell this file was authored from carries `CLAUDECODE` and seven
  `CLAUDE_CODE_*` names (read by name only). `CLAUDE_CONFIG_DIR` is inherited, not set.
- **The envelope and the scalars.** The whole appetite is `appetite = { points = 40 }` in `bug-standard` and
  `appetite = { points = 60 }` in `story`. `appetite.tokens` is read by `select/pack.rs` and set nowhere, and no
  per-item dispatch count exists anywhere. `grep -c 'NO RECORD'` returns 5 in each pack:
  - the `gate_deadline_secs` comment;
  - the banner;
  - the breaker comment `# value; NO RECORD names the number.`;
  - `consecutive_platform_failures = 3   # NO RECORD -- authored at Step 7e (RT-57)`;
  - `expiry_secs = 600                   # NO RECORD -- authored at Step 7e (RT-57)`.
- **The gate expiry.** The packs carry 0 `[[gate_expiry]]` rows. ADR-0146 records the consequence: *"Zero rows, for
  zero of the 45 question classes"*, and a step gate past its deadline makes the next run exit 3.

🛑 **THIS STAGE DISPATCHES NOTHING.** D21-40 decision 2 gates every real dispatch on the fixture proofs, and those
are `s8j`'s. The two live probes of D21-41 (g) are **authored** here (§8h.11) and **run at `s8j`**. The CLI pin
**procedure** is authored here (§8h.8) and **executed at `s8j`**. Record A of §8h.1 is therefore committed
`proposed`, and it is accepted only after both probes pass. Checkpoint 8h asserts that no `dispatch` or
`dispatch_completed` row was appended while it ran.

**What this stage owns.**

| decision | what it builds or writes | section |
| --- | --- | --- |
| D21-41 (g) | the eight cards; three principle-7 guards; schemas as `EXPORTED` Rust types, read by name | §8h.2, §8h.4, §8h.5 |
| D21-42 15a, 15b | `[models]` table; full id to `--model` and the card digest; typed `observed_model`; exact-id refusal | §8h.3 |
| D21-42 16 | the pin-move procedure and the auto-update freeze (executed at `s8j`) | §8h.8 |
| D21-42 17a, 17b | the committed launcher; `CLAUDE_CONFIG_DIR` set to `~/.claude-personal`; the `apiKeySource` check | §8h.7, §8h.5 |
| D21-42 19a, 19c | the per-item dispatch cap and token appetite; the RT-57 scalar record | §8h.9 |
| D21-42 21a | `gate_expiry` drafted fail-closed for every reachable class, signed row by row | §8h.10 |
| D21-42 21b, 21d | explicit minimal `--setting-sources`; the child's confinement settings; probe 1's HALT | §8h.6, §8h.11 |
| D21-42 21c | the doctor card-grant row and ADR-0146's load-time lint | §8h.10 |
| D21-43 N17 | Q1–Q4 as taken: the refusal classes, the token unit, the envelope's exit, probe 2's three dispatches | §8h.0, §8h.5, §8h.9, §8h.11 |
| D21-44; D21-45's standing limit | probe 1 inside `.redkiln/worktrees/<probe>`; the protected-segment limit named by the typed denial failure | §8h.5, §8h.6, §8h.11 |
| D21-46's integration calls | the launcher's `answer` mode; `wrap`'s guarantee; one explicit row per minted gate class; `RK_CP_NESTED` and the replay trap | §8h.7, §8h.10, §8h.15 |

**Owned elsewhere, cited rather than re-specified here.**
- **The run-worktree placement and its lifecycle belong to `s8d-ii`** (D21-46 R1: *"It carries D21-44's relocation and
  D21-45 W1/W2/W4"*), and the `worktree_assigned` row stays `s8e`'s. That covers the move to `.redkiln/worktrees/<item>`,
  its record, the `worktree_assigned` row, item-id validation, the symlink guards, `RK_BASE`, the lock, re-entry,
  and the filter-driver refusal. D21-44 wrote *"Consequences, owned by `s8e`"*, and D21-45 *"W1, W2 and W4's worktree
  half are **s8e**'s"*, before D21-46 R1 split them into `s8d-ii`.
- **The per-toolchain `setup` verb and `.worktreeinclude` reading belong to `s8c`** (D21-43 N25; D21-45 W3).
- **AC-133's judgement-enum registry belongs to `s8g`.** D21-43: *"AC-133's judgement-enum registry is built once,
  by **s8g**. s8h consumes it."*
- **Routing `s8`'s three `redkiln run` sites through the launcher belongs to `s8j`.** D21-43: *"s8's three
  `redkiln run` sites go through s8h's launcher (s8j amendment)."*
- **The pin move, both probes' execution and Checkpoint 7c's whole re-run belong to `s8j`.** See the plan's `s8j`
  row, and D21-43 N28: *"**s8j re-runs it whole**, from a plain terminal through the launcher, with the live
  probes."*

### 8h.0 Stage open: re-measure, and the four questions D21-43 N17 took

**The entry gate.** Checkpoint 8g must be recorded as passed in `PROGRESS.md` §1 with a hash, and its block is re-run
whole by Checkpoint 8h (§8h.15 (13)). Before any Red, re-measure the following against the tree `s8g` left, and
write each reading into the stage-open commit:

1. `cargo test -p <crate> --test it -- --list` for the four crates named in §8h.12. Every regression name that
   §8h.12 reuses must still exist. A rename by `s8a`–`s8g` is corrected here with a dated line, never silently.
2. **How `s8d`'s per-step binding names a step's output kind.** §8h.4 reads the schema by the **bound step's**
   output kind, the `value:<Kind>` of its `outputs`, never by `provides[0]`. If `s8d` left `bind_nodes` per node,
   the per-node `provides[0]` read is replaced by the step's kind here, and the gap is reported.
3. **Which lane kinds already exist as `rk-types` types.** At `ca8f387`, 0 of §8h.4's 24 kinds other than
   `Findings` exist as a type. `s8f`/`s8g` may have added `DebtRegisterEntry` (D21-42 6a), `Spec` or
   `AcceptanceCriterion` (D21-42 10a). A type that exists is **registered, never redeclared**. `s8g` exports
   `AcceptanceCriterion` and fixes a constraint on this stage's `Spec` (`s8g` §8g.3): *"`s8h`'s `Spec` must embed
   `AcceptanceCriterion` unchanged"*.
4. **The count source for §8h.9.** Does the kernel's pre-dispatch `dispatch` row name the item? If it does not, the
   count reads `dispatch_completed` by its interim `item` key, which `s8e` §8e.10 carries beside the payload
   (D21-43 N5; D21-46 R3), through `query(ByItem)`. Either way the source is the store, never a per-process counter.
5. **Whether `budget_extension` has gained a producer.** At `ca8f387` `wall_clock_expiry_mark`'s write has *"no call
   site anywhere in `src/`"* (`budget.rs`). If an earlier sub-stage wired one, that class joins §8h.10's table.
6. `claude --version`, and the size and bytes of `RUNBOOK-instrument.txt`. Halt 2 is recorded, not worked around,
   and D21-42 16 moves the pin once, at `s8j`.
7. **The gate classes `s8d`–`s8g` mint** (§8h.10's table lists every `s8` class, one row each). `s8f` mints two,
   drafted as `landing.refused-path` (`s8f` §8f.4) and `render.structured-drift` (`s8f` §8f.5). `s8f`'s landing and
   render records fix the spellings. `s8d`'s R6 opens `gate_opened { reason: unknown_facet, axis: "change" }` at
   triage, and the gate is answered with the `facet.change` question (`s8d` §8d.6, §8d.7), so its class is
   `unknown_facet:change`. Read each spelling from the code the minting stage landed, never from this file.
   - Set `MINTED_CLASSES` for Checkpoint 8h (8) to the kernel-minted classes only. At this stage that is `s8f`'s two.
   - Confirm that R6's gate carries `unknown_facet:change`. That class is an axis row, not a minted one, and (8)
     asserts it by name.
   - A class that a later stage mints brings its own row in the change that mints it, because §8h.10's load-time
     lint refuses a row whose class nothing mints. §8h.10 names `s8j`'s three.
8. **The whole-suite floor.** Read Checkpoint 8g's recorded passed count from `PROGRESS.md` §1 and write it over
   `BASE_8G` in Checkpoint 8h (12) by a dated edit. The expected value, **re-derived at stage open from Checkpoint
   8g's recorded count**, is BASE_8G = **1127**, so the floor is 1127 + 41 = **1168**. It is derived as follows
   (D21-46's one recomputation):
   - ADR-0164's baseline at `992d257`: 740 passed / 0 failed / 9 ignored over 44 results;
   - plus each sub-stage's new tests: 9 (8a) + 21 (8b) + 63 (8c) + 52 (8d) + 32 (8d-ii) + 67 (8e) + 79 (8f) +
     64 (8g) = 387, which gives 1127.

   Those drafts are being revised too, so the recorded count governs. A floor is never lowered to make a red run
   green.
9. **The run-worktree root.** Confirm that `s8d-ii` moved every derivation of the run-owned worktree to
   `.redkiln/worktrees/<item>` (D21-44), and name `recreate_run_worktree`'s successor. §8h.6's deny set and probe 1
   derive from the spec's worktree path and assume nothing more.
10. **AC-133's test names in `s8g`.** In the draft at `a6349b6`, Checkpoint 8g selects
    `judgement_registry::ac133_every_exported_enum_is_registered` and
    `judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails`. Checkpoint 8h (4) selects both.
    A rename is corrected there by a dated line.
11. **Every binary-level `answer` spawn** (D21-46). List the tests that run the `redkiln` binary's `answer` verb. That
    includes `rk-cli`'s shared helper in `tests/it/answer.rs` [measured at `c5d19d7`: `pub(crate) fn spawn_answer(`],
    `s8e`'s `lane_e2e` binary arm, and any other. §8h.7's launch check on `answer` refuses each of them unless it
    carries a fixture launch record. The record is supplied in the shared helper, so every such test exercises the
    check and none bypasses it. The list goes into the stage-open commit.

**Four questions were owed before Red. D21-43 N17 took all four on 2026-09-24**, and each is stated here as taken.
Each one changes a test's expected value.

- **Q1: the typed class of the four new refusals.** Taken: *"reuse `NotAdopted` and
  `NotIsolated { surface: Env }`; nothing superseded."* ADR-0112 closes `StepError` at eleven and *"Forbids a
  twelfth variant without a successor to this record"*. ADR-0095 decision 2 fixes `NotIsolated`'s payload as
  `{ surface: tools \| mcp_servers \| env, diff_digest }`. The mapping:
  - **(i)** An init-reported model unequal to the requested id → `NotAdopted { expected_agent, observed }`. This is
    the RT-27 reading: `init.rs`'s no-`agent`-key arm already refuses a model outside the bound tier as
    `NotAdopted`.
  - **(ii)** `apiKeySource` absent or not `"none"` → `NotIsolated { surface: Env, diff_digest }`. The digest is taken
    over the reported value's one-member set (empty when the key is absent) against `{"none"}`.
    `IsolationSurface::Env` has no producer at `a6349b6` [measured: `grep -rn "IsolationSurface::Env" crates/*/src`
    returns nothing].
  - **(iii)** An init-reported `permissionMode` that is unequal or absent → `NotIsolated { surface: Env, .. }`. The
    digest is over `{reported}` against `{the card's mode}`.
  - **(iv)** A non-empty `permission_denials` → `NotIsolated { surface: Env, .. }`. The digest is over the sorted
    denied `(tool_name, path)` entries against the empty set.

  The draft recommended two new `IsolationSurface` members for (iii) and (iv), which would have superseded ADR-0095
  decision 2's payload cell in part. N17's *"nothing superseded"* rules that out. So (iii) and (iv) take `Env`,
  the surface of the child's run environment, and Record A states that reading. The variant count stays eleven,
  and the retry class is `NotIsolated`'s `Never`. `exit::ac438_step_error_mapping_is_exhaustive` and
  `exit::ac438_exit_table_snapshot` do not move.
- **Q2: the unit `appetite.tokens` counts.** Taken: *"`appetite.tokens` is the sum of `Usage`'s four counters,
  about 24M per pass at first."* The four are `input_tokens`, `output_tokens`, `cache_creation_input_tokens` and
  `cache_read_input_tokens`, carried verbatim. Cache reads count. This does not re-open F14's question of whether
  they draw the window; the envelope only needs a sum that every row can compute without a price model. §8h.9 gives
  the first value.
- **Q3: how an envelope refusal leaves the CLI.** Taken: *"`Exit::Overage` (20)."* Its doc already reads *"A budget
  or billing state that does not improve by being repeated"*, so the exit snapshot does not move and the
  register's exit table takes no amendment. The draft's kernel class, one `EnvelopeError` variant per counter,
  stands as this file's construction. Finding 12 names the existing `StepError::Overage` that the implementer must
  weigh first.
- **Q4: probe 2's dispatch count.** Taken: *"probe 2 is three dispatches, one per model id."* §8h.11 authors it that
  way. A single dispatch would verify only one model id.

### 8h.1 The records this stage writes

Both records are new. Each takes **the next free id at writing** (no number is pre-assigned; `00-preamble.md` names
`0165` as the next free at this file's authoring, and other sub-stages allocate in writing order). Both use ADR-0164's
form — frontmatter, `## Context and problem`, `## Decision drivers`, `## Considered options` with `### Option — <name>`
headings (D21-42 24a), `## Decision`, `## Consequences` with a **What this forbids** block, `## Evidence and
confidence`. Checkpoint 8h reads the two ids from `REC_A` and `REC_B`.

**Record A — cards, models and the run environment** (`proposed`; **accepted only after both §8h.11 probes pass at
`s8j`**, on the operator's dated ledger line, D21-41 (g)). It decides, as `## Decision` sentences:
the grant table of §8h.2; the closed permission-mode set and the shipped pair; the `[models]` shape and the three
ids (§8h.3); **RT-27 settled** — adoption requires exact equality of the reported and requested model id, which
retires the segment match; the schema route (§8h.4) and the AC-133 judgement classification of each enum on the
types this stage adds, entered in `s8g`'s registry and **signed by the operator per enum** (AC-133's own limit:
*"that classification is the human gate AC-156"*); the three guards, N17's Q1 mapping and D21-45's protected-segment
limit named as a known limit (§8h.5); the setting-sources and confinement settings (§8h.6); the launcher and the
launch record as the only entry to a real run (§8h.7); the freeze (§8h.8). **It supersedes in part** ADR-0126
decision 1 — its payload table gains `observed_model` (a member added to a `## Decision` table is a supersession,
ADR-0092 r1) — and nothing else of the error set: N17 took Q1 as reuse, so ADR-0095 decision 2 and ADR-0112
stand. It supersedes **nothing** of ADR-0084 (the schemas are Rust types in `EXPORTED`, which is what its golden-snapshot clause binds: *"Every type that crosses the closed-output boundary has
its generated `schemars` output snapshotted with `insta`"*) and nothing of ADR-0156/0162, whose bars on
`.redkiln/schemas/` are stage-scoped and are honoured anyway (no such directory is created). It records that
**landing is not decided here** (D21-41 (b), `s8e`/`s8f`) and that the probes' spend is D21-40 decision 2's. It
also records that the run-worktree placement is not decided here either: that is D21-44's, written as `s8d-ii`'s record,
and Record A only cites it.

**Record B — RT-57's pack scalars, the per-item envelope, and gate expiry** (`proposed` at writing; the operator
signs **each value and each gate row**; it flips to `accepted` on the operator's dated ledger line **at this stage's
close, before `s8i`**, because D21-42 19c says *"written and signed before any spend"* and `s8i` spends). It carries
§8h.9's value table with a warrant per value, §8h.10's per-class table (AC-276: *"the choice is made and signed by a
human, per class"*), and fixes the key spellings RT-57 routed to *"7e's pack schema record"*. It supersedes nothing.

### 8h.2 The cards — least privilege, exactly

| card | `tools` | `permission_mode` | reached by (§8h.4's walk) |
| --- | --- | --- | --- |
| `implementer` | `Read`, `Glob`, `Grep`, `Edit`, `Write`, `StructuredOutput` | `acceptEdits` | both lanes |
| `investigator` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `reviewer` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `architect` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `spec-author` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `planner` | `StructuredOutput` | `default` | both |
| `releaser` | `StructuredOutput` | `default` | both |
| `operator` | `StructuredOutput` | `default` | **neither** — `operate` is `forbidden` in both packs |

No card gets `Bash`, a `Web*` tool or an `mcp__*` tool. Build and test stay on the `Toolchain` port. The card
record stays closed (`deny_unknown_fields`), and `model`/`effort` stay a tier leak (AC-118, AC-418). The `cwd` is the
run-owned worktree, already carried on the spec (RT-21's `WORKTREE_KEY`, `runner.rs`'s
`.current_dir(&prepared.worktree)`). Probe 1 reads it back from `system/init`'s `cwd`. After `s8d-ii` that path is
`.redkiln/worktrees/<item>` (D21-44), which is not under `.git`.

**The closed permission-mode set.** `lint_card` refuses a mode outside the CLI's own set:
`default`, `acceptEdits`, `bypassPermissions`, `plan`, `dontAsk`, `auto`. [measured 2026-09-24: the installed
2.1.281 binary's `--permission-mode` `.choices(...)` array, read by string search, never executed: the literal
`["default","acceptEdits","bypassPermissions","plan","dontAsk","auto"]`.] The adapter admits all six, so a misspelling
is refused and a valid mode is not. **Only the shipped pair `default` / `acceptEdits` is admitted in a shipped card**,
and that is the doctor row's job (§8h.10), not the adapter's. A test pack may legitimately use another mode.

### 8h.3 Models — the `[models]` table, full ids, and `observed_model` (D21-42 15a, 15b)

Each installed pack gains one table, so both packs stay independently complete, as their timeout rows already are:

```text
[models.haiku]    id = "claude-haiku-4-5-20251001"   accepts_effort = false
[models.sonnet]   id = "claude-sonnet-5"             accepts_effort = true
[models.opus]     id = "claude-opus-5-5"             accepts_effort = true
```

(Shown flattened. Each row is a TOML table with the two keys. Record A fixes the spellings.)
**`Tier` becomes the open set of `[models]` row keys.** `RoleExtras.model` becomes the row key (a string), and the
closed `card::Tier` enum leaves the binding path. `Tier::accepts_effort`'s lifted rule (*"haiku rejects the
parameter"*) becomes the row's `accepts_effort`, and its warrant is carried beside the haiku row.
`EffortUntraceable` / `EffortRefusedByTier` keep their meaning, keyed on the row. The `Effort` enum stays closed.
A role naming a key absent from `[models]` is refused at bind, naming the role and the key. The three ids appear as
string literals in the installed binary [measured 2026-09-24 by string search: `"claude-sonnet-5"` 54,
`"claude-opus-5-5"` 43, `"claude-haiku-4-5-20251001"` 15]. The 2.1.278 init reported `model` as the full
`claude-haiku-4-5-20251001` when the argv carried `haiku` (`PROGRESS.md`'s Claude Code instrument row, the 2026-09-21
probe). **Whether the child
reports exactly the requested id when `--model` carries a full id is UNMEASURED**, and probe 2 measures it.

**The role rows**, per D21-41 (g) as kept by D21-42 15b:

| role | `model` | `effort` |
| --- | --- | --- |
| `implementer`, `investigator` | `sonnet` | `high` |
| `spec-author` | `sonnet` | `medium` |
| `reviewer` | `opus` | `high` |
| `architect` | `opus` | `medium` |
| `planner`, `operator`, `releaser` | `haiku` | *(none)* |

`family = "claude"` stays. The breaker's partition key is D-7's, and it does not move.

**The full id travels everywhere the tier word did.** It goes into `--model`, into `AgentCard.model`, and so into
`to_agents_json()` and `AgentCard::digest()`. That digest is `ActivitySpec.card_digest`, so it enters the per-step
memo key D21-42 3a names. **Adoption becomes exact equality**: the init's `model` must equal the requested id, and
`model_belongs_to_tier` is deleted rather than left beside it. **`DispatchRecord` gains a typed `observed_model`.** It
is `Some` on every path that read a `system/init` carrying `model`, and `None` on every path that terminated before
one was read. It is never the requested id copied in. That would be Halt 11's shape, the same reasoning that gives
`cli_version` its `Observed`/`Pinned` arms. This is Record A's supersession of ADR-0126 decision 1 in part.

### 8h.4 Schemas — Rust types in `EXPORTED`, read by name (D21-41 (g); ADR-0084, ADR-0095, AC-133)

**The kinds the two lanes reach.** These are enumerated mechanically, with the rule stated so it reproduces:
- For each installed pack (`bug-standard` → the fix lane, `story` → the story lane), take every stage whose rigor
  is not `forbidden`. Collect its `baseline` and every `require` row's `artifacts`.
- Close that set over `requires` through every catalog activity tagged `kind:bug` or `kind:story` respectively
  that `provides` a member.
- Collect the `value:<Kind>` of every `kind = "agent"` step of those activities.

[measured 2026-09-24 at `ca8f387`: fix lane 18 kinds and story lane 17 over 38 candidate activities each. The union
is **24**. Every agent-step `outputs` in the set has the `value:` form.] This is a **candidate universe, not a
selection run.** Facets unknown at run time can route either way, so a selection-run set would miss kinds that a
different facet reading reaches. Every kind in the universe gets a type, and the binder refuses any kind outside
`EXPORTED`, so the direction is fail-closed.

| kind | lanes | role(s) | kind | lanes | role(s) |
| --- | --- | --- | --- | --- | --- |
| `ADR` | story | architect | `ImplReport` | both | implementer, investigator |
| `APIContract` | story | spec-author | `Postmortem` | fix | investigator |
| `ArchitectureDescription` | story | architect | `ReproAttempt` | fix | investigator |
| `CausalChain` | fix | investigator | `ReproCase` | fix | investigator |
| `ChangeRecord` | both | releaser | `Retrospective` | both | implementer, investigator |
| `DebtRegisterEntry` | both | implementer | `RolloutPlan` | both | releaser |
| `DesignDoc` | story | architect | `RootCause` | fix | investigator |
| `Docs` | story | implementer | `SelfCheck` | both | all but operator |
| `Estimate` | both | planner | `Spec` | both | spec-author |
| `Findings` | both | reviewer | `TestDesign` | both | implementer |
| `FixDesign` | fix | architect | `ThreatModel` | story | architect |
| `GherkinScenarios` | both | spec-author | `HypothesisLog` | fix | investigator |

**Field shapes.** `spikes/p9/schemas.toml` (*"Stand-in for schemars types: value schema name -> fields; list fields
flagged"*) carries a `[value.<Kind>]` row for **all 24** [measured]. Its field names and list flags are the
transcription source. The Rust types are the implementer's, under ADR-0095 decision 1's rule: no `oneOf`, no nested
union, `additionalProperties: false`, `required` first, ≤ 16 union-typed and ≤ 24 optional parameters per exported
schema. Each type is registered in `EXPORTED` under its kind name, and the golden snapshot (`schema_snapshot`) takes
it (ADR-0084). An `Option` is not a default: a field the stand-in lists is required unless Record A names why not.

**AC-133's registry is `s8g`'s, and this stage enters its types' enums in it.** D21-43's integration call: *"AC-133's
judgement-enum registry is built once, by **s8g**. s8h consumes it."* `s8g` builds that registry to AC-133's
*Verified by* (`s8g` §8g.3): a completeness test over every enum reachable from `EXPORTED`, the rationale-sibling
rule, and a must-fail fixture. AC-133 wants *"the enum appears in the judgement-enum registry with an explicit
judgement-bearing yes/no AND every enum marked yes has a sibling free-text `rationale` field"*.

Every type this stage adds to `EXPORTED` brings its enums with it, so `s8g`'s completeness test goes red until each
enum is entered with a yes/no and every "yes" has its `rationale` sibling. That red is this stage's instrument. No
second registry and no second completeness test are written. The yes/no **values** are the operator's (AC-156),
signed in Record A. The tests check that entries exist, never that the values are right.

**`Spec` embeds `AcceptanceCriterion` unchanged.** `s8g` §8g.3 sets the constraint: *"`s8h`'s `Spec` must embed
`AcceptanceCriterion` unchanged"*. `Spec`'s criteria member is `criteria: Vec<rk_types::AcceptanceCriterion>`, and
nothing in it is redeclared.

**The binder reads by name.** `bind_nodes` (or `s8d`'s per-step successor) resolves the schema as
`rk_types::registry::EXPORTED` entry named by the bound step's output kind. A kind that is absent is a bind-time
refusal naming the kind and the step (`Exit::ConfigError`), before any spawn. `.redkiln/schemas/` is not created,
and the file read is deleted.

### 8h.5 The three principle-7 guards, and the auth check (D21-41 (g); D21-42 15a, 17a)

All of these read the child's own report, never the argv (AC-202's rule, the `init|isolation` module filter).

1. **The closed-set lint** (§8h.2). It runs in `lint_card`, so it applies at load and again at bind.
2. **Init equality.** `init::decide` gains three comparisons beside the existing tool-set and MCP checks:
   - `permissionMode` must equal the card's mode, and an absent key is refused, not treated as equal.
   - `model` must equal the requested full id (§8h.3).
   - `apiKeySource` must equal `"none"`, and an absent key is refused.

   The last is ADR-0010 checked in-band: *"The adapter must refuse to proceed (assert, not warn)"*. The field is
   named by the 2.1.277 raw init (D17-11's amendment: *"AC-511's auth-mode field is **`apiKeySource`**, read as
   `"none"` on a subscription session"*), and the installed 2.1.281 init builder still emits `apiKeySource`,
   `permissionMode`, `model`, `tools` and `cwd` [measured by string search of the binary's `subtype:"init"` object].
   The classes are N17's Q1 (§8h.0). A model mismatch is `NotAdopted`, and a mode or `apiKeySource` mismatch is
   `NotIsolated { surface: Env }`.
3. **`permission_denials` → a typed failure.** The result frame carries `permission_denials: [{ tool_name,
   tool_use_id, tool_input }]` [measured: the binary's result schema; documented in `headless` as *"the final result
   message lists them in `permission_denials`"*]. A non-empty list fails the dispatch as
   `NotIsolated { surface: Env }` (Q1), whatever the exit code. It **outranks a valid `structured_output`**: a step
   whose child was denied an out-of-worktree edit has not done what it reported. It is placed in `classify.rs`'s
   order after the platform arms and before the `structured_output` validation. An empty list is not a failure.

**The standing limit, named (D21-45).** Claude Code's `checkPathSafetyForAutoEdit` treats a path as sensitive if ANY
case-folded segment is one of `[".git",".vscode",".idea",".claude",".husky",".cargo",".devcontainer",".yarn",".mvn"]`
or `.config/git` [D21-45, quoted from the 2.1.281 code]. D21-45 draws the consequence for a run worktree:
*"inside a run worktree, `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/` stay protected. An
implementer edit there is denied, and s8h's typed `permission_denials` failure names it as a known limit, not a
defect."*

- **The class does not change.** Such a denial reaches guard 3 as a non-empty `permission_denials` and fails as
  `NotIsolated { surface: Env }`, like any other denial. Nothing is retried.
- **The failure carries a typed note.** When a denied path resolves inside the spec's worktree, and one of its
  segments below the worktree root is on that list, classification carries a typed note beside the error:
  `ProtectedSegmentDenial { segment, tool_name }`. The adapter prints it as a warning naming D21-45's known limit,
  the way `record::mismatch_warning` names a version mismatch. The class and the digest stay the same.
- **The segment list is one `const`** in `rk-adapter-claude`, copied from D21-45 with that citation. A CLI release
  that changes the list is Halt 2's concern, through the pin, and not this guard's.

### 8h.6 Settings isolation and confinement (D21-42 21b, 21d)

**`--setting-sources ""`, always.** The 2.1.281 help reads *"Comma-separated list of setting sources to load (user,
project, local)."*, and the parser begins `if(e==="")return[];` [measured by string search]. The empty value loads
none of the three file sources. Policy settings and `--settings` flag settings are separate sources and still apply.
The element joins the golden argv in a fixed position that Record A states. `golden_argv_per_profile`'s snapshots
move, and the cap measure counts the quoted empty element. The docs site does not document the flag [a docs-reading
pass, 2026-09-24], so its effect is asserted from the init line (tools and mode equal; probe 2) and never inferred.

**The child's confinement settings.** They go in one `--settings <json>` element, assembled per dispatch from the
spec's worktree. [documented, `code.claude.com/docs/en/headless.md`, via the same pass: `--settings` *"can set any key
your user settings file can set"*.] It carries two things:

- **`permissions.blockReadsOutsideWorkingDirectories: true`** — [documented, `permissions.md`: *"Set
  `permissions.blockReadsOutsideWorkingDirectories` to make the file tools refuse the paths it fences in every
  permission mode"*; present in the 2.1.281 binary].
- **`permissions.deny` rules for `Read`, `Edit` and `Write` covering everything outside the worktree.** They are
  written as the **complement along the worktree's ancestor chain**: at each ancestor level, deny every sibling
  entry not on the path to the worktree, plus `//<drive>/**` for every drive letter other than the worktree's, plus
  `~/**`. **A carve-out is not used.** [documented, `permissions.md`: *"Claude Code reads a `!` pattern relative to
  the current directory even when `/`, `~/`, or `//` follows the `!`, so the pattern can't reach a rule anchored with
  one of those prefixes"*; and *"An allow rule can't carve an exception out of a deny rule."*] Paths take the
  documented Windows form (`//d/repos/...`). A sibling created after assembly is outside the set, and that
  residual is stated, not claimed away. `Read` rules govern `Glob`/`Grep` on a documented *"best-effort"* basis only,
  which is why the block setting is the primary read fence and the deny rules are the second.

**The `.git` risk this section once carried is measured and settled.** D21-44 ran D21-43 N1's probe from the
operator's own shell:
- an edit inside `.git/redkiln/worktrees/probe-n1-in` was refused, *"**Edit AND Write**, both refused"*;
- an edit inside `.redkiln/worktrees/probe-n1b` succeeded;
- both outside-the-repository controls succeeded.

Run-owned worktrees therefore move to `.redkiln/worktrees/<item>`, which is git-ignored (`.gitignore` carries
`.redkiln/worktrees/` at `a6349b6` [measured: `git check-ignore -v`]). The move, its record and the lifecycle
around it are `s8d-ii`'s. This stage assumes only that the spec's worktree path is under that root. Probe 1
re-measures the placement with the shipped card and settings (§8h.11). The standing in-worktree limit is handled by
§8h.5's named note. The one legacy tree, `rk-fix-doctor-test-count` at `73c087a`, stays under `.git` until `s8j`
re-points it (D21-44), and nothing here dispatches into it.

**The deny chain under the new root.** With the worktree at `D:/repos/redkiln-rs/.redkiln/worktrees/<item>`, the
chain's levels are:
- `D:/`
- `D:/repos`
- the repository root
- `.redkiln`
- `worktrees`

So the main checkout's own files are denied siblings. That includes its `.git` common directory, which a run
worktree's `.git` file points into. The child has no `Bash` and runs no git, so it has no need to read there. Every
other run worktree is a denied sibling too.

**Nothing is carried into the worktree for the child's settings.** D21-45 records the same choice from the other
side: `settings.local.json` is not copied, *"because children run with isolated settings (s8h)"*.

### 8h.7 The launcher — the only entry to a real run (D21-42 17a, 17b)

**`tools/live-run.sh`**, committed, LF (ADR-0164), POSIX, for Git Bash. Its modes are:
- `run <item> [--lane fix|story]`;
- `probe <test>`, where `<test>` is a module-qualified `live::` name;
- `wrap -- <argv…>`, below;
- `answer <item> <option> [<flag>…]`, below (D21-46);
- `--check [<mode> …]`, which performs every refusal and every capture for the named mode (`run` when none is
  named), spawns nothing and writes no launch record.

Every mode prints its captures one `KEY=value` per line: `MODE`, `CLI_VERSION`, `CLAUDE_CONFIG_DIR`,
`DISABLE_AUTOUPDATER`, and `LAUNCH_CHECK=ok` once every refusal has passed. Checkpoint 8h (10) reads those lines.

In order:

| # | step | refusal / capture |
| --- | --- | --- |
| 1 | `uname -s` is `MINGW*`/`MSYS*` | refuse otherwise |
| 2 | no `CLAUDECODE`, no `CLAUDE_CODE_*` name, and no `CLAUDE_PID` or `CLAUDE_EFFORT`, in its own environment | refuse naming the variable (s7c §7c.15's reason). The last two sit outside the prefix. A Claude Code session exports both [measured 2026-09-24, by name only, in the shell this revision ran in], and D21-44's probe script stripped both |
| 3 | no denylisted name, **case-insensitively** | the list is **read from `crates/rk-adapter-claude/src/env_denylist.rs`'s two `pub const` arrays by anchored extraction**, never copied. That file is *"the ONE list AC-217, AC-422, AC-488 and AC-509 read"*. The extraction asserts both anchors and a non-empty list (golden constraint 7). `Config::new` refuses the same names again, as the second line |
| 4 | `CLAUDE_CONFIG_DIR` is **set, not inherited**, to `$(cygpath -w "$HOME/.claude-personal")` (D21-42 17b) | refuse if the directory or its `.credentials.json` is absent. That is an existence test only, and no content is read |
| 5 | `DISABLE_AUTOUPDATER=1` is exported (§8h.8) | — |
| 6 | the resolved `claude` path; `claude --version`, printed as `CLI_VERSION=<v>`; the sha256 of the resolved file | `run`, `probe`, `answer`: refuse unless the version equals `TESTED_CLI_VERSION`, read by 7c's anchored form, **and** `RUNBOOK-instrument.txt`, naming Halt 2. `wrap`: record only, and never compare to the pin (below) |
| 7 | `run`: `cargo build --workspace` rc 0 before `./target/debug/redkiln` is run (Halt 1). `probe`: `cargo test -p rk-adapter-claude --features live --test it -- --ignored --nocapture <test> --exact`, then `test <test> ... ok` **and** `1 passed` are asserted, because a filter matching nothing is a green. `wrap`: the argv after `--`, exactly as given. `answer`: `cargo build --workspace` rc 0, then `./target/debug/redkiln answer <item> <option> [<flag>…]`, exactly as given | the child's rc is the launcher's |
| 8 | the **launch record** at `<git-common-dir>/redkiln/launches/<UTC>-<mode>.txt`: mode and argv, claude path, version and sha256 before; `CLAUDE_CONFIG_DIR`; `DISABLE_AUTOUPDATER`; the pin; HEAD; then version and sha256 after, and the rc | exported to the child as `REDKILN_LAUNCH_RECORD=<path>` |
| 9 | after the child exits: version and sha256 again | **a difference fails the launch non-zero**, naming Halt 2. The records then span a version boundary |

**What `wrap` guarantees, and what it does not** (restated 2026-09-24 for D21-46). The draft opened this paragraph
with *"`wrap` is the entry for a child that does not dispatch through `rk-adapter-claude`"* and then named Checkpoint
7c as a caller. 7c's `live_dispatch_probe` does dispatch through that adapter, and `s8j` §8j.6 step 2 found the
contradiction. The guarantee is stated here by what the launcher does, not by what the child is.

- **It guarantees** steps 1–5 unchanged: every environment refusal, the config directory and the freeze. It also
  guarantees step 8, the launch record with `REDKILN_LAUNCH_RECORD` exported to the child, and step 9. Before the
  child starts, the resolved `claude`'s version and sha256 are recorded. After it exits they are read again, and a
  difference fails the launch naming Halt 2. So a `wrap` child never spans a CLI version.
- **At step 6 it records and does not compare.** The version is not compared to `TESTED_CLI_VERSION` and not to
  `RUNBOOK-instrument.txt`. `wrap` does **not** guarantee that the child's CLI equals the adapter pin, because
  `TESTED_CLI_VERSION` binds the adapter, and the launcher does not know whether its child dispatches through it.
- **A `wrap` child that dispatches through `rk-adapter-claude` is pinned by its own means, never by the launcher.**
  Two such means exist. One is the adapter's `MismatchPolicy::Refuse` at the init line, which `real_harness_config`
  hard-codes. The other is the child's own pin guard.

Its two callers rely on exactly this:
- **`s8i`'s S-BIN batches through the corpus runner.** D21-46: *"s8i uses s8h's `wrap` mode for S-BIN's corpus
  runner"*. The runner makes no adapter dispatch. It relies on `wrap` **not** refusing the installed 2.1.281 against
  the adapter pin 2.1.278, which moves only at `s8j` (D21-42 16). What the set needs is that it does not span a
  version, and D21-43 N22 freezes the CLI *"from S-BIN on"*. Step 9 fails a launch whose binary moved, and `s8i`'s
  own set-level check reads the recorded version.
- **`s8j`'s whole re-run of Checkpoint 7c** (D21-43 N28: *"from a plain terminal through the launcher"*). Its
  `live_dispatch_probe` does dispatch through the adapter, and the block carries its own pin check ahead of that
  dispatch. It compares `claude --version` to `RUNBOOK-instrument.txt`, failing with *"FAIL: Halt 2 — claude
  --version is '$live' and the pinned instrument is '$pin'"*, and requires `config.rs` to declare
  `pub const TESTED_CLI_VERSION` equal to it (`s7c` §7c.21). `s8j` runs it after the pin move (`s8j` §8j.6 step 1),
  so the block's check, the adapter's policy and `wrap`'s before/after all see one version.

`wrap` is not a way around the pin for `run` or `probe`, nor for `answer`. `--check wrap` is asserted separately from
`--check run` and `--check answer`, including the control that it accepts a CLI unequal to the pin (Checkpoint 8h
(10); mutant M19).

**`answer` is the entry for a gate answer inside a real run** (D21-46). `s8j` §8j.3 answers its hold with
`redkiln answer <item> approve --explain-sufficed <yes|no> --toml-opened <yes|no>`, and `s8j` §8j.2 attaches its
human roots with `answer --attach`. Both must pass the launch check below, so both go through
`tools/live-run.sh answer`.

- **Its refusals.** First come its own argument refusals, before step 1. An absent `<item>`, an `<item>` beginning
  with `-`, and no `<option>` after the item each refuse, naming `answer` and the usage. The launcher does not
  validate the option or the flags, because `answer`'s own grammar does (ADR-0129 decision 1). Then steps 1–4 apply
  unchanged: the platform, the session names, the case-insensitive denylist, and the config directory.
- **Step 6 compares the version to the pin, as `run` does, naming Halt 2.** `answer` spawns no `claude`. But an answer
  is spent irrevocably (`gate_answer_spent`, ADR-0123), and it sits between two `run`s of one window. A pin mismatch
  means the next `run` refuses, so the answer is refused first rather than spent into a window that cannot continue.
- **What it records.** The launch record is `<UTC>-answer.txt`. It carries step 8's members, with the item and the
  full answer argv as the argv:
  - the mode;
  - the resolved `claude` path, and its version and sha256 before and after;
  - `CLAUDE_CONFIG_DIR` and `DISABLE_AUTOUPDATER`;
  - the pin and HEAD;
  - the rc.

  Step 9 fails the launch non-zero when the binary moved. The record's path is exported as `REDKILN_LAUNCH_RECORD`,
  so `answer::run` passes its launch check.
- **Its tests.** Checkpoint 8h (10) first drives `--check answer` in a clean environment, which is the control:
  `LAUNCH_CHECK=ok`, `MODE=answer`, and the pin as `CLI_VERSION`. It then drives four session or denylisted names, a
  CLI unequal to the pin, and three malformed argument lists. Each must refuse and name what it refused. §8h.12
  gives `run::`'s launch-check pair an `answer::` twin. Mutants M18 and M20 (§8h.13) must turn them red.

**Where the launcher runs: a plain terminal, never a Claude Code session.** There are two reasons:
- A session exports `CLAUDECODE` and `CLAUDE_CODE_*`, and step 2 **refuses** them. It does not strip them.
- The session's own permission layer refused to spawn a `claude` child. D21-43 N1: *"the session's permission
  layer refused to spawn the child from inside Claude Code"*.

D21-44's probe did run as the operator's own `! python …` line. That worked because the Python script stripped
the variables itself. The launcher does not strip, so a `!` line that invokes it from a session is refused at step
2. That is the design working. `s8j` §8j.6 says the same: *"never from a Claude Code session"*.

The launch record is **not** the kernel's run record. Writing the version into the `run_id` preimage would move every
`run_id`, which ADR-0130 decision 1's byte-stability forbids. The version already rides every `dispatch_completed`
row as `instrument_version` and `cli_version`. The launch record adds the binary's sha256 and the freeze state, which
no row carries.

**"The only entry" is made mechanical.** Two binary composition roots refuse at entry with `Exit::ConfigError`
unless the captured `Env` carries `REDKILN_LAUNCH_RECORD` naming an existing file:
- `run::run`, whose composition root has chosen the **real** `ClaudeHarness`;
- `answer::run`, the binary's `answer` (D21-46). It spawns no harness. It is subject because an answer to a real
  run's gate belongs to that run's launched window. Its in-process twin `answer::run_with` is not subject, just as a
  fake-harness path is not.

The refusal comes **before `reconcile_started` is appended and before any worktree operation**. For `answer` that
means before `run::through_step_seven`. The name is read through `env.rs`, the one read (ADR-0083). A fake-harness
path is not subject to it. Binary-level tests that drive the real harness at the fixture child, and every
binary-level `answer` test (§8h.0 item 11), supply a fixture launch record. So the check is exercised on every such
test and bypassed by none. The
three `live::` probe tests assert the same variable as their first statement. **This is also what re-scopes
Checkpoint 7f's `redkiln run`.** `s8a` (11) records that *"From s8h on that is false, and this line must be re-scoped
before it runs"*. Once the cards bind, a bare `redkiln run` refuses before any spawn. It is a guard against accident,
not a security boundary: the variable can be forged, and that is stated. §8h.14 says which old checkpoints this
makes provisional and how each is re-run.

**`s8`'s three `redkiln run` sites.** `s8-exit-criterion.md` invokes `./target/debug/redkiln run` at three sites.
After this stage, each one refuses without a launch record, so each must go through `tools/live-run.sh run`. That
edit is `s8j`'s (D21-43). This stage states the constraint and does not touch `s8-exit-criterion.md`. The same holds
for every `redkiln answer` the real runs need, `s8j` §8j.3's hold answer and §8j.2's attach included. Each goes
through `tools/live-run.sh answer`, and that edit is `s8j`'s too (D21-46).

### 8h.8 The CLI pin procedure and the auto-update freeze (D21-42 16) — authored here, executed at `s8j`

**The freeze mechanism** [measured 2026-09-24 by string search of the 2.1.281 binary, and not documented on the docs
site per the docs-reading pass]. The updater's gate reads `if(a.DISABLE_UPDATES)` and then
`if(Me(process.env.DISABLE_AUTOUPDATER))`. The legacy `autoUpdates: false` preference is *"migrated"* into the
`env` block of user `settings.json` as `DISABLE_AUTOUPDATER`. `DISABLE_UPDATES` also refuses a manual
`claude update`, and it is not chosen, because D21-42 16 freezes the **auto**-update. So:

- **The child** gets `DISABLE_AUTOUPDATER=1` from the launcher's environment. Under `--setting-sources ""` a user
  `env` block would not reach it.
- **The operator's own interactive sessions**, the process that moved the binary on 2026-09-22/23, are frozen by the
  operator adding `"env": { "DISABLE_AUTOUPDATER": "1" }` to `C:\Users\ryanm\.claude-personal\settings.json`, and to
  any other config directory in use, **before** step 1 below. It stays for the whole real-run window, through `s8`,
  and the operator removes it after. It is user configuration, so this stage writes the instruction, never the file.
- **The detector** is the launcher's before/after sha256 (§8h.7 step 9). The setting is not trusted; a move is caught.

**The procedure, in D17-11's form, run once at `s8j` before either probe.** Every step is recorded in the stage's
commit and in `PROGRESS.md` §1/§3/§6.

1. The freeze is in place, as above.
2. `v=$(claude --version); rc=$?`, with rc and the form asserted. `printf '%s\n' "$v"` then re-writes
   `spikes/results/instrument-checks/RUNBOOK-instrument.txt`, which is never `tee`'d (Halt 8 shape (h)). Assert
   `test -s` and the byte form (`2.1.278 (Claude Code)\n` is 22 bytes).
3. Move `TESTED_CLI_VERSION`, its dated prose comment, and `config::tested_cli_version_is_the_pin_as_a_config_value`'s
   literal. Move every scripted `system/init` fixture's `claude_code_version` and `rk-cli`'s `frames.jsonl` copies with
   them. The 2026-09-21 move found twelve and three: *"whose `claude_code_version` must equal the pin to reach the
   adoption check at all"*. Assert `PERMISSION_PROMPTS_FLOOR` is still below the pin.
4. `cargo build --workspace`, the adapter suite, then **`tools/live-run.sh probe live::live_dispatch_probe`**. That is
   one paid haiku dispatch, and it runs only on the operator's go-ahead.
5. Only then probes 1 and 2 (§8h.11). `MismatchPolicy::Refuse` stays hard-coded in `real_harness_config`.

### 8h.9 The per-item envelope, and the RT-57 scalar record (D21-42 19a, 19c)

**The cap and the appetite are kernel-enforced, per item, across processes.** Before each dispatch the kernel counts
the item's prior dispatches **from the store** (§8h.0 item 4) and refuses the next one once the count has reached
`appetite.dispatches`. It sums the item's charged tokens (Q2's unit) from its `dispatch_completed` rows and refuses
once the sum has reached `appetite.tokens`. The refusal leaves through `Exit::Overage` (20), N17's Q3. It happens
before any spawn, and it writes no tier-1 row. An absent `appetite.dispatches` is refused by pack validation, never
defaulted (`require_*`'s doctrine, `select/pack.rs`). A dispatch with no `usage` still counts against the dispatch cap. That is the reason the cap exists
beside the appetite. Both keys join the existing inline table, for example
`appetite = { points = 40, tokens = …, dispatches = 40 }` in `bug-standard`, whose `points` stays 40 (`story`'s stays
60). `points` is step 5's unrelated cost-class sum, and the three are different counters (AC-020). Integers are
written without `_` separators, for `toml_subset`.

**The first values and their warrants.**

| key | value | warrant |
| --- | --- | --- |
| `appetite.dispatches` | **40** | D21-42 19a's value. It is above the research pass's realistic 20–35 per fix-lane pass (the facts brief, not re-derived here) |
| `appetite.tokens` | **2 × E** (below) | D21-42 19a: *"about 2× the happy-path token estimate"* |
| `gate_deadline_secs` | 864000 (10 d) | The register's scope note says *"the longest was answered at 226 h"*. **AC-039's evidence line contradicts it**: *"the max 226 h / 9.4 days belongs to **stage→verdict** n=191, not to gate→verdict"* (gate→verdict: median 10.5 min, 45% > 1 h, max not stated). 240 h clears the larger series' maximum. That is the conservative direction for "do not expire an answered gate". Record B states both and the operator signs |
| `[breaker] consecutive_platform_failures` (K) | 3 | [measured 2026-09-24, `results/e3.jsonl` at `CORPUS_PIN`, file order]: 7 of 101 rows carry `is_error: true`, in runs of 1, 4, 1, 1. The one measured `Platform` episode reaches four consecutive. K = 3 trips inside it, the isolated singles never trip it, and K ≥ 5 would never have tripped. This is an episode **shape**, not a rate. AC-033's note (*"no spike measured a `Platform` failure rate at all"*) stands, and file order is taken as append order |
| `[breaker] expiry_secs` | 600 | [inferred] 2 × AC-029's 5 min backoff cap, so an open mark outlasts two capped backoffs. There is no measurement. It is signed as chosen |
| `[margins] five_hour` / `seven_day` | 10 / 10 ticks | prior art: the runner's `max_five_hour: 0.90` (`throttle.rs`'s header). The floor is `MIN_MARGIN_TICKS = 2` (AC-220). F14: two 20-dispatch haiku batches moved `seven_day` by 0 ticks. **These lines carry no `NO RECORD` marker today**, and the facts brief's claim that they do is false. They are in RT-57's floor and are recorded here anyway |

**E, the happy-path token estimate: E = N × M.**

- **N = 17**, the upper end of the research pass's *"about 14–17 dispatches on the happy path"* (the facts brief). It
  is re-derived by `s8j`'s FakeHarness walk, which counts the lane's dispatches exactly.
- **M = 681,083**, the pooled median of every per-dispatch four-counter total over the corpus's agentic dispatch rows
  (`p7`, `p8`, `e3`, `e3h`, `s-typed`, `s-typed-oldschema`, `s-var`, `s-var-2400`, `h4`: n = 384; p75 1,321,531)
  [measured 2026-09-24 at `CORPUS_PIN` `61f590e5…`, read-only].
- So **E ≈ 11.58 M** and the first appetite is **≈ 23.2 M**, rounded up to 24,000,000.
- N17 took Q2 as the four-counter sum, which is D21-43's *"about 24M per pass at first"*. The alternative unit,
  with cache reads excluded, was **not taken**. It would have given M = 62,725 and an appetite of about 2.2 M.
  Record B keeps it only as a considered option.

The warrant is weak, and Record B says so. The rows are mostly Opus reviews and investigations on another CLI
version, and they contain no agentic Sonnet row, so no tier-matched figure exists. D21-42 19a: *"Both are revised and
recorded from measurement"*. The first real run's `dispatch_completed` usage revises both values, on a dated line.

Every `NO RECORD` marker is replaced by a comment naming Record B as `ADR-<id>`, on the `gate_deadline_secs`, K,
`expiry_secs`, `[margins]` and `appetite` lines at least, and `grep -c 'NO RECORD'` reaches 0 in both packs.

### 8h.10 Gate expiry, fail-closed, and the two load-time guards (D21-42 21a, 21c)

**The reachable classes.**

- **The 45 question classes** [measured: `grep -h '^question' .redkiln/catalog/*.toml | sort -u | wc -l` returns 45,
  and the inline `escalate = { question = … }` spellings add none]:
  `adr.accept api-contract.accept appetite.overrun asvs.level atam.accept-risks bisect.no-culprit bluegreen.cutover
  canary.degraded canary.proceed change.authorize closure.signoff confirmation.not-fixed dependency.breaking
  discovery.go-no-go draft.accept hotfix.authorize incident.end inspection.meeting loop.exhausted migration.approve
  migration.execute mitigation.not-restored mitigation.proceed postmortem.review prototype.user-test prr.accept-gaps
  repro.cannot-reproduce review.blocking rollback.confirm rollout.approve rollout.ramp root-cause.accept
  root-cause.no-hypothesis scan.accept-findings seam.cutover slicing.approve spec.ambiguity
  spec.requirements-conflict spike.decision spike.timebox-exhausted sprint.decide sprint.test-results
  tdd.green-exhausted threat-model.accept-residual vuln.accept-sla`.
- **`unknown_facet:<axis>` for every axis a pack's `require` rows read.** `step2.rs` opens one per abstaining axis,
  `GateReason` has only `UnknownFacet`, and `Item::context` binds only `self.facets`, so even `rigor` can abstain.
  `bug-standard` reads `criticality`, `blast_radius`, `size`, `change` and `rigor`, which is 5. `story` reads those
  plus `external_dep` and `novelty`, which is 7.
- **`escalation`**, carried fail-closed so that any `escalate` exception the operator signs is lint-valid (AC-260:
  no `escalation → escalate`).
- **Every gate class an `s8` stage mints, one explicit row each** (§8h.0 item 7; enumerated 2026-09-24 for D21-46
  from `s8d`, `s8f` and `s8j` at `c5d19d7`). None is a catalog question, so without its own row each would be an
  unmatched class. D21-43 N6 makes every gate transition, `expired` included, a git-first tier-1 event, so an expiry
  row is what that transition reads. The spellings are the drafts', and the minting stage's code replaces them.

| class (drafted spelling) | minted by | the class guard admits it as | its rows land | in `MINTED_CLASSES` |
| --- | --- | --- | --- | --- |
| `landing.refused-path` | `s8f` §8f.4, the landing's refused-path gate | a kernel-minted class | here, in both packs | yes |
| `render.structured-drift` | `s8f` §8f.5, D21-43 N11's drift gate | a kernel-minted class | here, in both packs | yes |
| `unknown_facet:change` | `s8d` §8d.7's R6 triage gate, answered with the `facet.change` question (`s8d` §8d.6) | `unknown_facet:<registered facet>` | here, in both packs, as one of the axis rows (both packs read `change`) | no. (8) asserts it by name, because R6 opens it whatever the `require` rows read |
| `human_root:BugReport` (drafted by `s8j`) | `s8j` §8j.2, bound to the question `root.bug-report` | a kernel-minted class | at `s8j`, in the change that mints it, in both packs | yes, from `s8j` |
| `human_root:IntakeRecord` (drafted by `s8j`) | `s8j` §8j.2, bound to the question `root.intake-record` | a kernel-minted class | at `s8j`, in the change that mints it, in both packs | yes, from `s8j` |
| `hold:specification` (drafted by `s8j`), the analysis → specification hold | `s8j` §8j.3, item-level (`hold_before`) | a kernel-minted class | at `s8j`, in the change that mints it, in both packs, because a hold is item-level and either pack's item can carry one | yes, from `s8j` |

`facet.change`, `root.bug-report` and `root.intake-record` are **question ids, not gate classes**. A row keys on the
class the gate carries, and the question is how that gate is answered. `s8j`'s records fix the class that the step-4
gate binds under (`s8j` §8j.2: *"the gate-kind name the step-4 gate binds under"*) and the hold's gate cause (`s8j`
§8j.3). The spellings in the table are `s8j`'s drafts, and this file drafts none of its own.

Their rows cannot land here. The misspelt-class half of the lint below refuses a row whose class nothing mints yet,
so a row drafted ahead of its minting code would turn this stage's own checkpoint red. `s8j` drafts the three rows
fail-closed for the operator's row-by-row signature (`s8j` §8j.2 item 4). When it re-runs Checkpoint 8h, it adds
their spellings to `MINTED_CLASSES`.

Both packs carry all 45 questions, so each stays independently complete. That is **51 rows** in `bug-standard` and
**53** in `story`, plus one row per kernel-minted class in each. At this stage's close that means `s8f`'s two, for 53
and 55. After `s8j`'s three the totals are 56 and 58. `budget_extension` joins the table only if §8h.0 item 5 finds
a producer.

**The draft is `fail` for every row.** `auto-decide` is refused by name in `run.rs` because it has no verdict carrier
(RT-57), so it cannot be drafted. `escalate` is admissible only as a signed exception with a warrant. None is
drafted. The operator signs row by row (AC-276). A signed exception amends the table here and Checkpoint 8h's
exception list, by dated line.

**ADR-0146's load-time lint** (*"the honest options are to ship the rows or to make an unmatched step-gate class a
load-time refusal"*). **Both are built.** At pack load, `run` and `answer` refuse, before any dispatch, when a
reachable class of the enabled pack has no `[[gate_expiry]]` row. This file reads *"a gate class with no process
entry"* as no row in the enabled process pack, and Record B states that reading. They also refuse a row whose class
is neither a catalog question, nor `unknown_facet:<registered facet>`, nor `escalation`, nor a gate kind the kernel
mints, because a misspelt class leaves the real class uncovered.

**The doctor row, `RK-D-CARD-GRANT`.** It refuses any card tool outside the (g) union
`{Read, Glob, Grep, Edit, Write, StructuredOutput}`, naming `Bash`, `PowerShell`, `Web*` and `mcp__*` in its message.
It also refuses any shipped card mode outside `{default, acceptEdits}`. The installed CLI offers a `PowerShell` tool
[measured: `CLAUDE_CODE_USE_POWERSHELL_TOOL` in the binary], which is a command channel of the same class as `Bash`.
A union allowlist refuses it without naming a list that goes stale. `ac415_every_rule_id_has_a_rejecting_fixture`
requires the row's fixture.

### 8h.11 The two live probes — authored here, run at `s8j` after the fixture proofs

Both are `#[ignore]` tests in `crates/rk-adapter-claude/tests/it/live.rs` behind `--features live`. They run only as
`tools/live-run.sh probe live::<name>`, from a plain terminal (§8h.7), on the operator's go-ahead, and after §8h.8's
pin move. They use the **shipped** cards bound through the **shipped** role rows, because the thing measured is what
ships. Each prints its values one per line, as `live_dispatch_probe` does. The raw `system/init` and result frames are retained by O-9's route, a
commit in `D:/repos/redkiln-corpus` named by SHA, which never moves `CORPUS_PIN`.

**Probe 1 — `live::live_confinement_and_denial_probe`** (one Sonnet dispatch; AC-215's integration half; D21-42 21d).

- **Setup: the worktree sits where run worktrees now live.** The probe creates its own worktree at
  `<repo>/.redkiln/worktrees/rk-probe-<token>`, with `git worktree add --detach <path> HEAD` under `LC_ALL=C`
  (D21-45 W1). It asserts that the path's realpath is under `.redkiln/worktrees/`. That is D21-44's placement, and
  not a temp directory, because the protected-path check reads the absolute path's segments. The path goes on the
  spec, and it is the `cwd`. The probe writes `inside.txt` there.
- **Two sentinels sit outside it**, each with a fresh random token and a recorded sha256:
  - `outside-write.txt`, in a sibling temp directory outside the repository;
  - `outside-read.txt`, directly under `.redkiln/worktrees/`, which makes it a sibling of the probe's tree.
- **Cleanup.** After reading its results, the probe removes the tree it created and only that tree. It then asserts
  that `git worktree prune --dry-run` prints nothing. This mirrors Claude Code's own rule, which D21-45 quotes:
  *"It never removes worktrees it did not create."*
- **The dispatch.** The bound `implementer` card (`acceptEdits`, the six tools) with the §8h.6 settings. The user
  message instructs, in order:
  1. write a given token into `inside.txt`;
  2. write a given token into `<wt>/.cargo/rk-probe.txt`, a protected segment inside the worktree (D21-45's
     standing limit);
  3. write into `outside-write.txt` by absolute path;
  4. read `outside-read.txt` by absolute path and put its content in the structured output;
  5. report each outcome.

  The timeout is 300 s, s7c's bound and Halt 5's alternative.
- **What it prints and records.** It prints one value per line:
  - from the init: `INIT_PERMISSION_MODE`, `INIT_CWD`, `INIT_TOOLS`, `INIT_MODEL`, `INIT_API_KEY_SOURCE`;
  - from the result: `DENIALS` (the count and the `tool_name` list), `OUTCOME` (the typed result), `LIMIT_NOTE` (the
    §8h.5 note's segment, or `none`), `WALL_MS`;
  - from the files: `INSIDE_SHA256`, `CARGO_PROBE_EXISTS`, and `OUTSIDE_WRITE_SHA256` before and after.
- **Pass, all of:**
  1. The child terminated under the bound. A kill by the timer is a never-waits failure.
  2. `inside.txt` holds the token. So an edit inside `.redkiln/worktrees/<probe>` works under `acceptEdits`, which
     is D21-44's N1b arm re-measured with the shipped card and settings.
  3. `<wt>/.cargo/rk-probe.txt` does not exist, and the adapter's note names segment `.cargo`.
  4. `outside-write.txt`'s sha256 is unchanged.
  5. The read token appears **nowhere** in the retained stdout, including the structured output.
  6. `permission_denials` is non-empty and names all three: the outside write, the outside read **and** the
     `.cargo` write.
  7. The adapter's outcome is `NotIsolated { surface: Env }` (Q1), not `Ok`.
  8. The init reports all of these: `acceptEdits`; the six tools; the requested id; `apiKeySource` `"none"`; and a
     `cwd` equal to the worktree, under `.redkiln/worktrees/`.
- **HALT — the work stops and a containment finding is reported, never re-tuned (D21-42 21d; Halt 7).** Any of:
  1. the outside file changed;
  2. the read token leaked;
  3. a denial occurred but was not surfaced as `permission_denials`;
  4. the child hung;
  5. the inside edit was denied. D21-44 measured this placement writable, so a denial now means the placement or
     the CLI moved.
- **A `.cargo` write that succeeds is not a containment breach**, because it is inside the worktree. The probe
  still does not pass (pass condition 3), because the write contradicts D21-45's measured list. It is reported, and
  the note's list is re-read against the installed code. The list is never loosened in place.
- **Inconclusive, which is not a pass.** An instructed action has no denial and its target is untouched, meaning the
  model did not attempt it. The (g) record is not accepted, and a re-run is a new operator go-ahead.

**Probe 2 — `live::live_init_equality_probe`** (N17's Q4: three dispatches, one per model id).
The bound `planner` (haiku, `default`, `[StructuredOutput]`), `investigator` (sonnet/high, `default`, the read set) and
`reviewer` (opus/high, `default`, the read set) cards run with a trivial instruction that returns a fixed
structured value.

- **Pass, per dispatch:** the init's `tools` equal the card's set; `permissionMode` equals the card's; `model` equals
  the requested **full id** exactly; `apiKeySource` is `"none"`; `mcp_servers` is empty; `claude_code_version`
  equals the pin; and the dispatch completes `Ok`.
- **Recorded, not asserted:** the init's `plugins`, `skills` and `slash_commands` counts, which are the observable
  trace of the settings sources that `--setting-sources ""` withheld.
- **A reported model that is a different spelling of the same model** (say, a dated suffix) **is a failure.** Record A
  then needs a decision on the mapping. The equality is never loosened in place (Halt 7).

**What the (g) record's acceptance cites:** both probes' printed values, the corpus commit SHA and the frames'
sha256, and the launch records' paths and sha256.

### 8h.12 Red first — the named tests

Each test is authored and **run red for the named reason** before its code, per golden constraint 12. Names are
module-qualified. The new modules, `rk-cli`'s `cards` and `schemas`, are added to that crate's `tests/it/main.rs`
[measured at `a6349b6`: neither exists]. **41 new tests in all.** D21-46's `answer` mode added two to the draft's 39.

**`rk-adapter-claude` (17 new).** Red reasons and N17's classes are in brackets.
- `card::permission_mode_outside_the_cli_closed_set_is_refused` [`"acceptEdit"` binds today]
- `card::every_cli_permission_mode_is_admitted_by_the_closed_set` [the control against an over-narrow set]
- `isolation::permission_mode_reported_unequal_to_the_card_is_refused` [`NotIsolated { surface: Env }`]
- `isolation::permission_mode_absent_from_init_is_refused` [the same class]
- `isolation::model_reported_unequal_to_the_requested_id_is_refused` [requested `claude-sonnet-5`, reported
  `claude-sonnet-4-6`: the segment match adopts it today; the class is `NotAdopted`]
- `isolation::model_equal_to_the_requested_id_proceeds`
- `isolation::api_key_source_other_than_none_is_refused` [`NotIsolated { surface: Env }`]
- `isolation::api_key_source_absent_is_refused` [the same class]
- `classify::permission_denials_non_empty_is_a_typed_failure` [`NotIsolated { surface: Env }`]
- `classify::permission_denials_empty_is_not_a_failure`
- `classify::permission_denials_outrank_a_valid_structured_output`
- `classify::a_denial_under_a_protected_worktree_segment_is_named_the_known_limit` [a `<wt>/.cargo/x` denial has
  the same class plus a `ProtectedSegmentDenial` note naming `.cargo`. Control: a denial outside the worktree
  carries no note, and neither does a path whose only `.cargo` segment is above the worktree root]
- `command::setting_sources_is_explicit_and_empty`
- `command::child_settings_carry_the_worktree_confinement` [the deny set is the ancestor-chain complement, no `!`,
  plus the block setting]
- `binder::the_full_model_id_reaches_argv_and_the_card_digest` [two ids for one row key move the digest]
- `binder::a_tier_absent_from_the_models_table_is_refused`
- `dispatch::observed_model_is_recorded_on_every_path_that_read_init` [`None` before init, never the requested id]

**`rk-types` (1 new).**
- `dispatch_record::observed_model_is_a_typed_member_absent_before_init`

AC-133's registry and its completeness tests are `s8g`'s, and this stage adds none. Its new types' enums are entered
in that registry, and `s8g`'s two `judgement_registry::` tests (§8h.0 item 10) are selected in Checkpoint 8h (4) as
regressions. They go red until the entries exist.

**`rk-kernel` (6 new).**
- `envelope::dispatch_cap_is_declared_never_defaulted`
- `envelope::the_dispatch_at_the_cap_proceeds`
- `envelope::the_dispatch_past_the_cap_is_refused_with_its_class`
- `envelope::the_dispatch_count_survives_a_second_process` [two runtimes over one store; a one-hop fixture cannot
  judge a multi-hop guard]
- `envelope::token_appetite_past_the_appetite_refuses_the_next_dispatch`
- `envelope::a_dispatch_with_no_usage_still_counts_against_the_cap`

**`rk-cli` (17 new).**
- `cards::shipped_cards_are_the_least_privilege_table`
- `cards::shipped_role_rows_resolve_through_the_models_table`
- `cards::the_models_table_pins_the_three_full_ids`
- `cards::shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class`
- `cards::shipped_rt57_scalars_carry_no_no_record_marker`
- `schemas::every_agent_output_kind_the_two_lanes_reach_is_exported` [it asserts the universe equals §8h.4's 24 as a
  PREMISE, so a catalog change that reaches a new kind fails loudly]
- `schemas::bind_reads_the_schema_from_exported_by_name`
- `schemas::a_kind_absent_from_exported_refuses_at_bind`
- `doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set` [fixtures `Bash`, `PowerShell`,
  `WebFetch`, `WebSearch`, `mcp__x__y`; control: the shipped cards are clean]
- `doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair`
- `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load` [a counting fake sees 0 dispatches]
- `run::a_gate_expiry_row_naming_no_reachable_class_is_refused`
- `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` [the store's row count and the
  worktree set are unchanged]
- `run::a_real_harness_run_with_a_launch_record_passes_the_launch_check` [the control]
- `answer::a_binary_answer_without_a_launch_record_refuses_before_any_row` [D21-46: the binary's `answer`, driven
  through the shared spawn helper with its fixture record withheld. The store's row count is unchanged and the gate
  is still open]
- `answer::a_binary_answer_with_a_launch_record_passes_the_launch_check` [the control: the same gate is answered]
- `exit::an_envelope_refusal_leaves_through_overage` [N17's Q3: a dispatch refused at the cap exits 20, and
  `exit::ac438_exit_table_snapshot` does not move]

**Live (authored, compiled, listed, never run here).** `live::live_confinement_and_denial_probe` and
`live::live_init_equality_probe`, each `#[ignore = "…paid…; run by the operator at s8j through tools/live-run.sh"]`.

### 8h.13 Mutants — named, each run from Python in a throwaway worktree, restore verified

Each mutant is applied by a Python script with an anchor assertion (golden constraint 7) in a
`git worktree add --detach` copy at the stage's HEAD. Its named test is run and must go **red**, then the copy is
removed, and `git diff --quiet` in the main tree is asserted. A survivor blocks.

| id | mutation | must turn red |
| --- | --- | --- |
| M1 | the closed-set check replaced by the old non-empty test | `card::permission_mode_outside_the_cli_closed_set_is_refused` |
| M2 | the init mode comparison deleted | `isolation::permission_mode_reported_unequal_to_the_card_is_refused` |
| M3 | model equality relaxed to the pre-stage segment match over the row key | `isolation::model_reported_unequal_to_the_requested_id_is_refused` |
| M4 | the `apiKeySource` comparison deleted | `isolation::api_key_source_other_than_none_is_refused` |
| M5 | `permission_denials` ignored | `classify::permission_denials_non_empty_is_a_typed_failure` |
| M6 | the denial arm moved after the `structured_output` validation | `classify::permission_denials_outrank_a_valid_structured_output` |
| M7 | the `--setting-sources` push deleted | `command::setting_sources_is_explicit_and_empty` |
| M8 | `AgentCard.model` given the row key instead of the id | `binder::the_full_model_id_reaches_argv_and_the_card_digest` |
| M9 | the cap comparison off by one (`>` for `>=`) | one of `envelope::the_dispatch_at_the_cap_proceeds` / `…past_the_cap…` |
| M10 | the dispatch count taken from an in-process counter | `envelope::the_dispatch_count_survives_a_second_process` |
| M11 | the gate-expiry load lint skipped | `run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load` |
| M12 | the grant row admits the `mcp__` prefix | `doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set` |
| M13 | `bind_nodes` reads `.redkiln/schemas/<kind>.json` again | `schemas::bind_reads_the_schema_from_exported_by_name` |
| M14 | the launch-record check deleted | `run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn` |
| M15 | the launcher's `CLAUDE_CODE_` refusal deleted, in a copy of the script | Checkpoint 8h (10)'s `CLAUDE_CODE_ENTRYPOINT` control |
| M16 | the `ProtectedSegmentDenial` note never attached | `classify::a_denial_under_a_protected_worktree_segment_is_named_the_known_limit` |
| M17 | the envelope refusal mapped to `Exit::ConfigError` | `exit::an_envelope_refusal_leaves_through_overage` |
| M18 | `answer::run`'s launch-record check deleted | `answer::a_binary_answer_without_a_launch_record_refuses_before_any_row` |
| M19 | `wrap` given `run`'s pin comparison at step 6, in a copy of the script | Checkpoint 8h (10)'s `--check wrap` control over a CLI unequal to the pin |
| M20 | `answer`'s pin comparison deleted at step 6, in a copy of the script | Checkpoint 8h (10)'s `--check answer` Halt 2 control |

### 8h.14 What this stage may not do, what it makes provisional, and what it hands on

- It may not dispatch, run a `live::` test, move the pin, or edit the operator's user settings.
- It may not create `.redkiln/schemas/`, copy the denylist, or build a second AC-133 registry.
- It may not loosen an equality to make a probe pass.
- It may not move the run-worktree root, or touch the legacy `rk-fix-doctor-test-count` tree. Both are `s8d-ii`'s and
  `s8j`'s (D21-44).

**The checkpoints this stage makes provisional, and where each is re-run.** The rule is to re-run the entry gate
whole, plus the older checkpoints this stage's own edits make provisional. It does not re-run the whole chain.

| checkpoint | why provisional | re-run where |
| --- | --- | --- |
| 8g | the entry gate | Checkpoint 8h (13), whole |
| 5 | `rk-types`: `observed_model`, the new `EXPORTED` types | Checkpoint 8h (13), whole |
| 7b, 7b-ii | `rk-kernel`: the per-item envelope | Checkpoint 8h (13), whole |
| 7d | `rk-cli`: the launch check, `RK-D-CARD-GRANT`, the load lint, the envelope's exit | Checkpoint 8h (13), whole |
| 7c | `rk-adapter-claude`: argv, init equality, classification | **not here.** Its block carries the live probe. `s8j` re-runs it whole through `wrap`, from a plain terminal (D21-43 N28). Its non-live regressions are selected by name in (3) |
| 7e | `.redkiln/cards/` and both packs | **not re-runnable whole**: its block ends in `git add .redkiln spikes/p9 && git commit`. Its subject is asserted by (8), by `conformance::shipped_pack_loads` in (5), and by the load lint's tests |
| 7f | its `redkiln run` arm now refuses at the launch check. Before that check exists, the same arm would reach a real dispatch once the cards bind | `s8j`'s entry-chain run, as described below |
| 8a | its (11) guard, *"a card carries permission_mode (rc=$rc) — 7i's or 7f's run could reach a REAL dispatch; stop"*, fires on the eight cards this stage writes | `s8j`, with that guard re-scoped to the route below |

**How a later stage re-runs an old checkpoint that executes `redkiln run`.** Of 7f, 7g, 7h and 7i's run arm, only
7f's block executes `redkiln run` at `a6349b6` [measured by a grep over the four files' bash lines]. Every re-run of
such a block after this stage has to do four things:
- point `[adapters] harness_binary` at `target/debug/rk-fixture-child-cli` through a fixture config, never the
  shipped `.redkiln/config.toml` left edited;
- supply a fixture `REDKILN_LAUNCH_RECORD`;
- back up the store and restore it;
- assert that no real `claude` was spawned, for example by showing no new session transcript under either
  configuration directory.

7f's block no longer touches the legacy tree. `s8d-ii` §8d-ii.7 moved its run arm into a throwaway clone and deleted
its `git worktree remove --force .git/redkiln/worktrees/rk-fix-doctor-test-count` line, so the four things above apply
inside that clone. All of this is `s8j`'s. This stage states it and runs none of it.

**What it hands on:**
- **To `s8j`:**
  - `s8`'s three `redkiln run` sites go through `tools/live-run.sh run` (§8h.7; D21-43).
  - The 7f / 8a route above.
  - The pin move and both probes, run from a plain terminal (`s8j` §8j.6).
  - Checkpoint 7c's whole re-run through `wrap`. `wrap` does not compare to the pin, and 7c's block does (§8h.7).
  - `s8j`'s answers go through `tools/live-run.sh answer`, both §8j.3 step 5's hold answer and §8j.2's attach
    (D21-46). The mode exists here, and the edit to those lines is `s8j`'s.
  - The rows for its three gate classes, landed in the change that mints them (§8h.10). Their spellings join
    `MINTED_CLASSES` when it re-runs Checkpoint 8h.
- **To `s8i`:**
  - Record B must be accepted before S-BIN spends (D21-42 19c).
  - S-BIN launches through `wrap` (§8h.7). The adapter pin does not bind the corpus runner, and D21-43 N22 freezes
    the CLI from S-BIN on. **This withdraws the draft's finding 3**, which said an `s8i` dispatch would refuse at
    the init line. That holds only for a dispatch through `rk-adapter-claude`, and S-BIN makes none.
- **To `s8i` and `s8j`: their dispatch scans refuse this checkpoint as written** (finding 10).

### 8h.15 Checkpoint 8h

- The eight cards are §8h.2's table. The closed-set lint, the init equality (mode, model, auth) and the denial
  mapping each fail when absent, each through its own named test.
- `[models]` pins the three ids, the role rows resolve through it, and `observed_model` is recorded.
- Every one of the 24 lane-reached kinds is an `EXPORTED` Rust type, read by name. Their enums are entered in
  `s8g`'s AC-133 registry, and its two completeness tests pass.
- `--setting-sources ""` and the confinement settings are on every argv.
- The launcher refuses what it must and captures what it must, in each of `run`, `wrap` and `answer`. Only `wrap`
  skips the pin comparison. A real-harness run, or a binary `answer`, without a launch record refuses before any row.
- The per-item cap and appetite refuse across processes.
- No `NO RECORD` marker remains. Both packs carry fail-closed gate rows for every reachable class, including the
  minted gate kinds. An unmatched class refuses at load, and `RK-D-CARD-GRANT` refuses a grant outside the union.
- A protected-segment denial inside the worktree keeps its class and carries the named note (D21-45).
- Record A is `proposed`. Record B is written, and the operator's signatures are a stage-close gate (AC-276:
  *"routed to a human gate; not machine-decidable"*).
- The two probes compile and are ignored. **No dispatch happened.**
- Checkpoint 8g passes whole, and so do Checkpoints 5, 7b, 7b-ii and 7d, which this stage's crate edits make
  provisional (§8h.14). Checkpoint 7c's shell guards pass.
- No `REDKILN_LAUNCH_RECORD` is in the environment, so nothing re-run here can pass the launch check.
- The whole suite is at or above Checkpoint 8g's recorded count plus 41, with 0 failed over at least 44 results.
- Under `RK_CP_NESTED=1`, (13) replays nothing. Otherwise every replay runs with no launch record and with
  `CLAUDE_CONFIG_DIR` at an empty trap directory that must still be empty afterwards (D21-46).

**Run it from a plain terminal: a Git Bash that Claude Code did not spawn** (7c's guard, repeated here). Set these:
- `REC_A` and `REC_B`;
- `A_SUPERSEDES`, the ids Record A supersedes, space-separated. It is `0126` alone, since N17 took Q1 as reuse;
- `TOKENS_APPETITE`, Record B's signed value, which is 24000000 as drafted;
- `MINTED_CLASSES`, from §8h.0 item 7;
- whatever the re-run checkpoints' blocks require themselves.

Leave `REDKILN_LAUNCH_RECORD` unset. `BASE_8G` in (12) is edited at stage open (§8h.0 item 8). Budget hours: (13)
runs five checkpoints whole, unless `RK_CP_NESTED=1` is set, in which case it runs none (D21-46).

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`. It DISPATCHES NOTHING: it never
# passes `--ignored` to a test run without `--list`, never runs tools/live-run.sh outside `--check`, and (14) asserts
# the store gained no dispatch row. Every cargo reading is `out=$(…); rc=$?`; every selection is `--test it`,
# module-qualified and `--exact`, asserting each `test <name> ... ok` line AND an exact count.

# ---- (0) THIS BOX, THEN CHECKPOINT 7c's THREE SHELL GUARDS FROM s7c §7c.21 (the re-run the entry asks for) — verbatim
# ---- except that their messages' section refs are qualified `s7c §…`, as a bare ref would name this file's section.
case "$(uname -s)" in MINGW*|MSYS*) : ;; *) echo "FAIL: this block runs on the Windows box; the POSIX arm is ARMED to CI and is not this block's evidence"; exit 1;; esac
env | grep -qE '^(CLAUDECODE=|CLAUDE_CODE_)' && { echo "FAIL: run this block from a Git Bash outside Claude Code — a Claude-spawned shell puts CLAUDECODE/CLAUDE_CODE_* into the probe's captured map (s7c §7c.15)"; exit 1; }
[ -z "${ANTHROPIC_FOUNDRY_API_KEY:-}" ] || { echo "FAIL: ANTHROPIC_FOUNDRY_API_KEY is set in this shell — it is on the committed denylist (s7c §7c.6) and Config::new will refuse; run the block under 'env -u ANTHROPIC_FOUNDRY_API_KEY bash' (never set_var)"; exit 1; }
env | grep -qE '^(GITHUB_TOKEN|GH_TOKEN|GH_ENTERPRISE_TOKEN)=' && { echo "FAIL: a tracker credential (GITHUB_TOKEN / GH_TOKEN / GH_ENTERPRISE_TOKEN — RT-25, AC-509) is set in this shell — it is on the committed denylist and Config::new will refuse; unset it outside the process"; exit 1; }
# ---- (0b) THIS STAGE'S ADDITION: the two session names outside the CLAUDE_CODE_ prefix (§8h.7 step 2).
env | grep -qE '^(CLAUDE_PID|CLAUDE_EFFORT)=' && { echo "FAIL: CLAUDE_PID or CLAUDE_EFFORT is set — a Claude Code session exported it; run this block from a plain terminal (§8h.7)"; exit 1; }

# ---- INPUTS. The record ids are allocated at writing; this block takes them rather than guessing them.
test -n "${REC_A:-}" && test -n "${REC_B:-}" || { echo "FAIL: set REC_A and REC_B to the two records' four-digit ids"; exit 1; }
test -n "${A_SUPERSEDES:-}" || { echo "FAIL: set A_SUPERSEDES (at minimum 0126 — observed_model moves ADR-0126 d1's payload table)"; exit 1; }
test -n "${TOKENS_APPETITE:-}" || { echo "FAIL: set TOKENS_APPETITE to Record B's signed appetite.tokens"; exit 1; }
test -n "${MINTED_CLASSES:-}" || { echo "FAIL: set MINTED_CLASSES to the kernel-minted gate classes read at stage open, space-separated (§8h.0 item 7; §8h.10's table)"; exit 1; }
# No launch record may be in reach: every real-harness `run` in this block and in the checkpoints (13) re-runs then
# refuses before any spawn (§8h.7), and (14) reads the store for the dispatch rows that would say otherwise.
[ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is set in this shell — unset it; nothing this block runs may pass the launch check"; exit 1; }
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store"; exit 1; }
launches="$(git rev-parse --git-common-dir)/redkiln/launches"
probe='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(c.execute("SELECT COUNT(*) FROM t2_event WHERE kind IN (?,?)",("dispatch","dispatch_completed")).fetchone()[0])'
disp_before=$(python -c "$probe" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$disp_before" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }
nl_before=$(find "$launches" -type f 2>/dev/null | wc -l)

# ---- (1) BUILD-OR-FAIL, FIRST (Halt 1). Nothing below may read a binary this did not just build.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8h NOT RUN"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) THE TWO RECORDS: one file each, the ADR-0164 form, Record A proposed and not yet back-linked.
fa=$(ls references/adr/"$REC_A"-*.md 2>/dev/null); fb=$(ls references/adr/"$REC_B"-*.md 2>/dev/null)
[ "$(printf '%s\n' "$fa" | grep -c .)" -eq 1 ] && [ "$(printf '%s\n' "$fb" | grep -c .)" -eq 1 ] || { echo "FAIL: expected exactly one file per record id"; exit 1; }
grep -qxF 'status: proposed' "$fa" || { echo "FAIL: Record A must be proposed until both s8j probes pass (D21-41 (g))"; exit 1; }
grep -qxE 'status: (proposed|accepted)' "$fb" || { echo "FAIL: Record B carries no valid status"; exit 1; }
for f in "$fa" "$fb"; do
  [ "$(grep -c '^### Option — ' "$f")" -ge 2 ] || { echo "FAIL: $f lacks ADR-0085 option headings (D21-42 24a)"; exit 1; }
  grep -qF 'What this forbids' "$f" || { echo "FAIL: $f carries no forbids block"; exit 1; }
  grep -qxF '## Evidence and confidence' "$f" || { echo "FAIL: $f carries no evidence section"; exit 1; }
done
sup=$(grep -m1 '^supersedes:' "$fa"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Record A has no supersedes line"; exit 1; }
for s in $A_SUPERSEDES; do printf '%s\n' "$sup" | grep -qF "\"$s\"" || { echo "FAIL: Record A does not declare it supersedes $s"; exit 1; }; done
if grep -m1 '^superseded_by:' references/adr/0126-*.md | grep -qF "\"$REC_A\""; then echo "FAIL: ADR-0126 is back-linked to a PROPOSED record — the link moves in the flip commit (ADR-0092 cl.2.1)"; exit 1; fi

# ---- (3)-(6) THE NAMED TESTS, per crate: each `... ok` and an exact count (new + the regression names §8h.12 keeps).
sel() {  # sel <package> <expected count> <tests...>
  pkg=$1; want=$2; shift 2
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $pkg selection rc=$rc"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted or not selected"; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $pkg did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
sel rk-adapter-claude 23 card::permission_mode_outside_the_cli_closed_set_is_refused card::every_cli_permission_mode_is_admitted_by_the_closed_set isolation::permission_mode_reported_unequal_to_the_card_is_refused isolation::permission_mode_absent_from_init_is_refused isolation::model_reported_unequal_to_the_requested_id_is_refused isolation::model_equal_to_the_requested_id_proceeds isolation::api_key_source_other_than_none_is_refused isolation::api_key_source_absent_is_refused classify::permission_denials_non_empty_is_a_typed_failure classify::permission_denials_empty_is_not_a_failure classify::permission_denials_outrank_a_valid_structured_output classify::a_denial_under_a_protected_worktree_segment_is_named_the_known_limit command::setting_sources_is_explicit_and_empty command::child_settings_carry_the_worktree_confinement binder::the_full_model_id_reaches_argv_and_the_card_digest binder::a_tier_absent_from_the_models_table_is_refused dispatch::observed_model_is_recorded_on_every_path_that_read_init command::golden_argv_per_profile command::ac214_permission_mode_explicit command::permission_prompts_none_on_every_argv isolation::ac204_adoption_equal_subset_superset_wrong_agent isolation::ac202_isolation_from_system_init_not_argv config::tested_cli_version_is_the_pin_as_a_config_value
sel rk-types 7 dispatch_record::observed_model_is_a_typed_member_absent_before_init schema_snapshot::schema_snapshot schema_no_oneof::no_oneof_at_any_node schema_caps::union_and_optional_caps dispatch_record::ac360_dispatch_record_carries_the_usage_fields judgement_registry::ac133_every_exported_enum_is_registered judgement_registry::ac133_a_judgement_enum_without_a_rationale_sibling_fails
sel rk-kernel 8 envelope::dispatch_cap_is_declared_never_defaulted envelope::the_dispatch_at_the_cap_proceeds envelope::the_dispatch_past_the_cap_is_refused_with_its_class envelope::the_dispatch_count_survives_a_second_process envelope::token_appetite_past_the_appetite_refuses_the_next_dispatch envelope::a_dispatch_with_no_usage_still_counts_against_the_cap envelope::ac029_n_is_declared_never_defaulted conformance::shipped_pack_loads
sel rk-cli 21 cards::shipped_cards_are_the_least_privilege_table cards::shipped_role_rows_resolve_through_the_models_table cards::the_models_table_pins_the_three_full_ids cards::shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class cards::shipped_rt57_scalars_carry_no_no_record_marker schemas::every_agent_output_kind_the_two_lanes_reach_is_exported schemas::bind_reads_the_schema_from_exported_by_name schemas::a_kind_absent_from_exported_refuses_at_bind doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load run::a_gate_expiry_row_naming_no_reachable_class_is_refused run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn run::a_real_harness_run_with_a_launch_record_passes_the_launch_check answer::a_binary_answer_without_a_launch_record_refuses_before_any_row answer::a_binary_answer_with_a_launch_record_passes_the_launch_check exit::an_envelope_refusal_leaves_through_overage doctor::ac415_every_rule_id_has_a_rejecting_fixture exit::ac438_exit_table_snapshot exit::ac438_step_error_mapping_is_exhaustive surface::ac400_help_snapshot_every_verb

# ---- (7) THE TWO PROBES EXIST, COMPILE UNDER `live`, AND ARE IGNORED — LISTED, NEVER RUN (`--list` runs nothing).
# ---- Finding 10: s8i's and s8j's dispatch scans match this line's feature flag; the scans are theirs to scope.
out=$(cargo test -p rk-adapter-claude --features live --test it -- --list --ignored 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the live feature does not compile (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
for t in live::live_confinement_and_denial_probe live::live_init_equality_probe; do
  printf '%s\n' "$out" | grep -qxF "$t: test" || { echo "FAIL: $t is not an IGNORED test under --features live — it would run, or it does not exist"; exit 1; }
done

# ---- (8) THE SHIPPED DATA, read by parsers (json / tomllib), never grepped: cards, [models], role rows, gate rows,
# ---- the envelope keys, and no NO RECORD marker. The expected gate-class set is DERIVED from the catalog and the
# ---- packs' own `when` rows and compared to §8h.10's counts as a PREMISE. EXCEPTIONS is empty until the operator
# ---- signs one; a signed exception amends this list by dated line. The minted gate kinds are an input, and each must
# ---- sit as a string literal on a code line under crates/*/src. Every `.redkiln/` read is one level deep, so no run
# ---- worktree under `.redkiln/worktrees/` is ever walked (D21-44).
out=$(python - "$TOKENS_APPETITE" "$REC_B" "$MINTED_CLASSES" <<'PY'
import glob, json, os, re, sys, tomllib
bad = []
want_cards = {
    "implementer": ({"Read", "Glob", "Grep", "Edit", "Write", "StructuredOutput"}, "acceptEdits"),
    "investigator": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "reviewer": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "architect": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "spec-author": ({"Read", "Glob", "Grep", "StructuredOutput"}, "default"),
    "planner": ({"StructuredOutput"}, "default"),
    "releaser": ({"StructuredOutput"}, "default"),
    "operator": ({"StructuredOutput"}, "default"),
}
names = sorted(os.path.basename(p)[:-5] for p in glob.glob(".redkiln/cards/*.json"))
if names != sorted(want_cards):
    bad.append("card set %r" % names)
for n, (tools, mode) in want_cards.items():
    d = json.load(open(".redkiln/cards/%s.json" % n, encoding="utf-8"))
    t = d.get("tools") or []
    if set(t) != tools or len(t) != len(set(t)):
        bad.append("%s tools %r" % (n, t))
    if d.get("permission_mode") != mode:
        bad.append("%s mode %r" % (n, d.get("permission_mode")))
    if "model" in d or "effort" in d:
        bad.append("%s carries a tier (AC-118)" % n)
models = {"haiku": ("claude-haiku-4-5-20251001", False), "sonnet": ("claude-sonnet-5", True), "opus": ("claude-opus-5-5", True)}
roles = {"implementer": ("sonnet", "high"), "investigator": ("sonnet", "high"), "spec-author": ("sonnet", "medium"),
         "reviewer": ("opus", "high"), "architect": ("opus", "medium"), "planner": ("haiku", None),
         "operator": ("haiku", None), "releaser": ("haiku", None)}
questions = set()
def walk(v):
    if isinstance(v, dict):
        for k, x in v.items():
            if k == "question" and isinstance(x, str):
                questions.add(x)
            walk(x)
    elif isinstance(v, list):
        for x in v:
            walk(x)
for f in glob.glob(".redkiln/catalog/*.toml"):
    walk(tomllib.load(open(f, "rb")))
if len(questions) != 45:
    bad.append("PREMISE: %d question classes, expected 45" % len(questions))
minted = set(sys.argv[3].split())
if len(minted) < 2:
    bad.append("PREMISE: MINTED_CLASSES names %d classes; s8f mints two (§8h.10's table)" % len(minted))
for c in sorted(minted):
    if c in questions or c.startswith("unknown_facet:") or c == "escalation":
        bad.append("PREMISE: MINTED_CLASSES lists %r, which already has a row of another kind; it would be counted twice" % c)
code = []
for f in glob.glob("crates/*/src/**/*.rs", recursive=True):
    code.extend(ln for ln in open(f, encoding="utf-8") if not ln.lstrip().startswith("//"))
if not code:
    bad.append("PREMISE: no code line read under crates/*/src")
for c in sorted(minted):
    if not any('"%s"' % c in ln for ln in code):
        bad.append("PREMISE: minted gate kind %r is a string literal on no code line under crates/*/src" % c)
EXCEPTIONS = {}
words = {"and", "or", "not", "in", "high", "med", "low", "mandatory", "recommended", "optional", "forbidden"}
expect_axes = {"bug-standard": 5, "story": 7}
expect_rows = {"bug-standard": 51 + len(minted), "story": 53 + len(minted)}
expect_points = {"bug-standard": 40, "story": 60}
for pack in ("bug-standard", "story"):
    raw = open(".redkiln/packs/%s.toml" % pack, encoding="utf-8").read()
    if raw.count("NO RECORD") != 0:
        bad.append("%s still carries %d NO RECORD markers" % (pack, raw.count("NO RECORD")))
    if raw.count("ADR-" + sys.argv[2]) < 5:
        bad.append("%s names Record B fewer than five times (gate deadline, K, expiry, margins, appetite)" % pack)
    d = tomllib.loads(raw)
    m = d.get("models", {})
    if set(m) != set(models):
        bad.append("%s [models] keys %r" % (pack, sorted(m)))
    for k, (mid, eff) in models.items():
        row = m.get(k, {})
        if row.get("id") != mid or row.get("accepts_effort") is not eff:
            bad.append("%s [models.%s] %r" % (pack, k, row))
    for r, (mk, ef) in roles.items():
        row = d.get("role", {}).get(r, {})
        if row.get("model") != mk or row.get("effort") != ef or row.get("family") != "claude":
            bad.append("%s [role.%s] %r" % (pack, r, row))
    app = d.get("pack", {}).get("appetite", {})
    if app.get("dispatches") != 40 or app.get("tokens") != int(sys.argv[1]) or app.get("points") != expect_points[pack]:
        bad.append("%s appetite %r" % (pack, app))
    axes = set()
    for st in d.get("stage", {}).values():
        for rq in st.get("require", []):
            text = re.sub(r"'[^']*'", " ", rq.get("when", ""))
            axes.update(w for w in re.findall(r"[a-z_]+", text) if w not in words)
    if len(axes) != expect_axes[pack]:
        bad.append("PREMISE: %s reads %d axes %r" % (pack, len(axes), sorted(axes)))
    if "change" not in axes:
        bad.append("PREMISE: %s's require rows no longer read change; s8d's R6 still opens unknown_facet:change at triage, so the row count below is wrong" % pack)
    want = set(questions).union({"unknown_facet:" + a for a in axes}, {"unknown_facet:change", "escalation"}, minted)
    rows = d.get("gate_expiry", [])
    got = [r.get("class") for r in rows]
    if "unknown_facet:change" not in got:
        bad.append("%s carries no unknown_facet:change row (s8d's R6 triage gate, §8h.10)" % pack)
    if len(got) != len(set(got)) or set(got) != want or len(got) != expect_rows[pack]:
        bad.append("%s gate rows: %d rows, missing %r, extra %r" % (pack, len(got), sorted(want.difference(got)), sorted(set(got).difference(want))))
    for r in rows:
        if r.get("action") != EXCEPTIONS.get(r.get("class"), "fail"):
            bad.append("%s gate %s action %r is not the signed value" % (pack, r.get("class"), r.get("action")))
if os.path.exists(".redkiln/schemas"):
    bad.append(".redkiln/schemas exists")
print("SHIPPED_DATA_OK" if not bad else "BAD: " + " ;; ".join(bad))
PY
); rc=$?
[ "$rc" -eq 0 ] && [ "$out" = "SHIPPED_DATA_OK" ] || { echo "FAIL: the shipped cards/packs are not §8h.2/§8h.3/§8h.9/§8h.10's (rc=$rc)"; printf '%s\n' "$out" | head -20; exit 1; }

# ---- (9) CODE-LINE FLOORS beneath the named tests — never the assertion. Comment lines are dropped AFTER the search
# ---- status is asserted, and each filter is first proved on a synthetic pair: a token on a code line must be kept, the
# ---- same token on a `//` line must be dropped (a guard satisfied by its own prose is the failure this exists for).
codehits() {  # codehits <regex> <path...>
  pat=$1; shift
  raw=$(grep -rnE --include=*.rs -e "$pat" "$@"); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: grep could not run over $* (rc=$rc)"; exit 1; }
  printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep -c .
}
ctl=$(mktemp -d)
printf 'let a = "--setting-sources";\n// "--setting-sources" in prose\n' > "$ctl/c.rs"
[ "$(codehits '--setting-sources' "$ctl")" -eq 1 ] || { echo "FAIL: the code-line filter is broken — the synthetic pair did not give exactly one code hit"; exit 1; }
[ "$(codehits '--setting-sources' crates/rk-adapter-claude/src/command.rs)" -ge 1 ] || { echo "FAIL: command.rs pushes no --setting-sources on a code line"; exit 1; }
[ "$(codehits 'permission_denials' crates/rk-adapter-claude/src)" -ge 1 ] || { echo "FAIL: no adapter code line reads permission_denials"; exit 1; }
[ "$(codehits 'apiKeySource' crates/rk-adapter-claude/src/init.rs)" -ge 1 ] || { echo "FAIL: init.rs reads no apiKeySource on a code line"; exit 1; }
[ "$(codehits 'REDKILN_LAUNCH_RECORD' crates/rk-cli/src)" -ge 1 ] || { echo "FAIL: rk-cli names no REDKILN_LAUNCH_RECORD on a code line"; exit 1; }
[ "$(codehits 'model_belongs_to_tier' crates/rk-adapter-claude/src crates/rk-cli/src)" -eq 0 ] || { echo "FAIL: the segment-match adoption survives on a code line (RT-27 is settled by equality)"; exit 1; }
[ "$(codehits 'join\("schemas"\)' crates/rk-cli/src)" -eq 0 ] || { echo "FAIL: rk-cli still reads a schemas directory (§8h.4)"; exit 1; }
rm -rf "$ctl"

# ---- (10) THE LAUNCHER, BEHAVIOURALLY, `--check` ONLY. A fake `claude` stands first on PATH, so nothing real is
# ---- spawned; the clean environment is `env -i` plus what Git Bash needs. Every refusal names what it refused.
L=tools/live-run.sh
test -f "$L" || { echo "FAIL: $L is absent"; exit 1; }
if od -c "$L" | grep -qF '\r'; then echo "FAIL: $L carries a CR byte (ADR-0164)"; exit 1; fi
out=$(bash -n "$L" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: $L does not parse"; printf '%s\n' "$out"; exit 1; }
pin=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9.]+)";$/\1/p' crates/rk-adapter-claude/src/config.rs)
test -n "$pin" || { echo "FAIL: the pin's anchored form did not match"; exit 1; }
fake=$(mktemp -d); bad_fake=$(mktemp -d)
printf '#!/bin/sh\necho "%s (Claude Code)"\n' "$pin" > "$fake/claude"
printf '#!/bin/sh\necho "9.9.9 (Claude Code)"\n' > "$bad_fake/claude"
chmod +x "$fake/claude" "$bad_fake/claude"
clean=(env -i "HOME=$HOME" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "PATH=$fake:/usr/bin:/mingw64/bin")
out=$("${clean[@]}" bash "$L" --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the launcher refused a clean environment (rc=$rc)"; printf '%s\n' "$out" | tail -10; exit 1; }
cfg=$(cygpath -w "$HOME/.claude-personal")
for line in "LAUNCH_CHECK=ok" "DISABLE_AUTOUPDATER=1" "CLAUDE_CONFIG_DIR=$cfg" "CLI_VERSION=$pin"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
for v in CLAUDECODE=1 CLAUDE_CODE_ENTRYPOINT=cli CLAUDE_PID=1 CLAUDE_EFFORT=high ANTHROPIC_API_KEY=x anthropic_base_url=x GH_TOKEN=x CLAUDE_CODE_USE_BEDROCK=1; do
  out=$("${clean[@]}" "$v" bash "$L" --check 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: the launcher accepted $v"; exit 1; }
  printf '%s\n' "$out" | grep -qiF "${v%%=*}" || { echo "FAIL: the refusal of $v does not name the variable"; printf '%s\n' "$out"; exit 1; }
done
out=$(env -i "HOME=$HOME" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "PATH=$bad_fake:/usr/bin:/mingw64/bin" bash "$L" --check 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the launcher accepted a CLI version unequal to the pin"; exit 1; }
printf '%s\n' "$out" | grep -qF 'Halt 2' || { echo "FAIL: the version refusal does not name Halt 2"; printf '%s\n' "$out"; exit 1; }
# `wrap` (§8h.7): the same refusals, a recorded version, and NO comparison to the adapter pin.
out=$("${clean[@]}" bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: --check wrap refused a clean environment (rc=$rc)"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "LAUNCH_CHECK=ok" "MODE=wrap" "DISABLE_AUTOUPDATER=1" "CLAUDE_CONFIG_DIR=$cfg" "CLI_VERSION=$pin"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check wrap did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
out=$("${clean[@]}" CLAUDECODE=1 bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: --check wrap accepted CLAUDECODE — wrap skips the pin, never the environment refusals"; exit 1; }
out=$(env -i "HOME=$HOME" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "PATH=$bad_fake:/usr/bin:/mingw64/bin" bash "$L" --check wrap -- true 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: --check wrap refused a CLI unequal to the adapter pin — wrap records the version, it does not compare (§8h.7)"; printf '%s\n' "$out" | tail -10; exit 1; }
printf '%s\n' "$out" | grep -qxF "CLI_VERSION=9.9.9" || { echo "FAIL: --check wrap did not record the version it found"; printf '%s\n' "$out"; exit 1; }
# `answer` (§8h.7, D21-46): run's refusals, run's pin comparison, and its own argument refusals. The first call is the
# control: the same launcher, the same clean environment and a well-formed argv must pass.
out=$("${clean[@]}" bash "$L" --check answer rk-fix-check-only approve 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: --check answer refused a clean environment (rc=$rc)"; printf '%s\n' "$out" | tail -10; exit 1; }
for line in "LAUNCH_CHECK=ok" "MODE=answer" "DISABLE_AUTOUPDATER=1" "CLAUDE_CONFIG_DIR=$cfg" "CLI_VERSION=$pin"; do
  printf '%s\n' "$out" | grep -qxF "$line" || { echo "FAIL: --check answer did not print '$line'"; printf '%s\n' "$out"; exit 1; }
done
for v in CLAUDECODE=1 CLAUDE_CODE_ENTRYPOINT=cli CLAUDE_PID=1 ANTHROPIC_API_KEY=x; do
  out=$("${clean[@]}" "$v" bash "$L" --check answer rk-fix-check-only approve 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: --check answer accepted $v — answer takes run's environment refusals"; exit 1; }
  printf '%s\n' "$out" | grep -qiF "${v%%=*}" || { echo "FAIL: --check answer's refusal of $v does not name the variable"; printf '%s\n' "$out"; exit 1; }
done
out=$(env -i "HOME=$HOME" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "PATH=$bad_fake:/usr/bin:/mingw64/bin" bash "$L" --check answer rk-fix-check-only approve 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: --check answer accepted a CLI unequal to the pin — an answer is spent irrevocably, so it is pinned as run is (§8h.7)"; exit 1; }
printf '%s\n' "$out" | grep -qF 'Halt 2' || { echo "FAIL: --check answer's version refusal does not name Halt 2"; printf '%s\n' "$out"; exit 1; }
for argv in "" "-x approve" "rk-fix-check-only"; do
  out=$("${clean[@]}" bash "$L" --check answer $argv 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: --check answer accepted the argument list '$argv' — an item and an option are required"; exit 1; }
  printf '%s\n' "$out" | grep -qF 'answer' || { echo "FAIL: the refusal of '$argv' does not name answer's usage"; printf '%s\n' "$out"; exit 1; }
done
rm -rf "$fake" "$bad_fake"

# ---- (11) THE PIN, READ NOT MOVED: the constant equals the instrument file (7c's anchored form). The live binary is a
# ---- READING: at authoring it is 2.1.281 against 2.1.278 — Halt 2 LIVE, recorded, moved once at s8j (§8h.8).
inst=$(sed -nE 's/^([0-9.]+) \(Claude Code\)$/\1/p' spikes/results/instrument-checks/RUNBOOK-instrument.txt)
[ -n "$inst" ] && [ "$inst" = "$pin" ] || { echo "FAIL: TESTED_CLI_VERSION ($pin) and RUNBOOK-instrument.txt ($inst) disagree — a half-taken pin move"; exit 1; }
live_v=$(claude --version 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: claude --version rc=$rc"; exit 1; }
if [ "$live_v" != "$pin (Claude Code)" ]; then echo "READING: Halt 2 LIVE — installed '$live_v' vs pin $pin; recorded, moved at s8j (D21-42 16), not here"; fi

# ---- (12) FMT, CLIPPY (the workspace, and the live-gated code the workspace line does not compile), THE WHOLE SUITE.
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo clippy -p rk-adapter-claude --all-targets --features live 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy over the live probes rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the whole suite rc=$rc (a fresh checkout needs pnpm install for the node fixture first)"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
# The floor is Checkpoint 8g's recorded passed count plus this stage's 41 new tests (§8h.12). Expected value, re-derived
# at stage open from Checkpoint 8g's recorded count (§8h.0 item 8): BASE_8G = 1127, floor 1127 + 41 = 1168. Edit the
# next line by a dated edit before the first run.
BASE_8G=1127
FLOOR=$((BASE_8G + 41))
tally() { printf '%s\n' "$1" | awk -v k="$2" '/^test result: /{for(i=1;i<NF;i++) if($(i+1)==k";" || $(i+1)==k) s+=$i} END{print s+0}'; }
[ "$(tally 'test result: ok. 3 passed; 0 failed; 9 ignored; 0 measured; 0 filtered out' passed)" -eq 3 ] || { echo "FAIL: synthetic control — tally does not read a passed count"; exit 1; }
[ "$(tally 'test result: FAILED. 3 passed; 2 failed; 0 ignored' failed)" -eq 2 ] || { echo "FAIL: synthetic control — tally does not read a failed count"; exit 1; }
[ "$(tally 'running 0 tests' passed)" -lt "$FLOOR" ] || { echo "FAIL: synthetic control — the floor admits a run of zero tests"; exit 1; }
passed=$(tally "$out" passed); failed=$(tally "$out" failed)
nres=$(printf '%s\n' "$out" | grep -c '^test result: ')
[ "$failed" -eq 0 ] && [ "$nres" -ge 44 ] && [ "$passed" -ge "$FLOOR" ] || { echo "FAIL: the whole suite read passed=$passed failed=$failed over $nres results — the floor is $FLOOR passed, 0 failed, at least 44 results"; exit 1; }

# ---- (13) RE-RUN WHOLE, FROM THEIR OWN BYTES: Checkpoint 8g (the entry gate) and the four older checkpoints
# ---- this stage's crate edits make provisional (§8h.14's table): 5, 7b, 7b-ii, 7d. 7c, 7e, 7f and 8a are NOT
# ---- re-run here; the table says where. Each extracted block gets a sentinel, so "rc 0 and the sentinel" means it
# ---- ran to its end. REDKILN_LAUNCH_RECORD is unset (INPUTS), so a real-harness `run` in any of them refuses first.
# ---- BOUNDED (D21-46's one re-run convention): every replay runs with RK_CP_NESTED=1, and when THIS block runs under
# ---- RK_CP_NESTED=1, as a later checkpoint's entry gate, it replays nothing. NO DISPATCH (D21-46's one guard for
# ---- replays, s8g's form): every replay runs with REDKILN_LAUNCH_RECORD removed from its environment and with
# ---- CLAUDE_CONFIG_DIR at an empty trap directory, which must still be empty afterwards.
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8H (13) SKIPPED: this block is itself a replay (RK_CP_NESTED=1); its re-runs ran at Checkpoint 8h's own close"
else
mkdir -p target/s8h-whole || { echo "FAIL: cannot create target/s8h-whole"; exit 1; }
trap_dir="$PWD/target/s8h-whole/claude-config-trap"
rm -rf "$trap_dir" && mkdir -p "$trap_dir" || { echo "FAIL: could not create the CLAUDE_CONFIG_DIR trap"; exit 1; }
# Synthetic control: the emptiness reading sees one file, or its silence afterwards proves nothing.
: > "$trap_dir/control" || { echo "FAIL: could not write the trap's control file"; exit 1; }
n=$(find "$trap_dir" -mindepth 1 | grep -c .)
[ "$n" -eq 1 ] || { echo "FAIL: the trap's emptiness reading saw $n entries where one was written"; exit 1; }
rm -f "$trap_dir/control" || { echo "FAIL: could not clear the trap's control file"; exit 1; }
n=$(find "$trap_dir" -mindepth 1 | grep -c .)
[ "$n" -eq 0 ] || { echo "FAIL: the trap does not start empty ($n entries) — its emptiness afterwards would prove nothing"; exit 1; }
extract() {  # extract <file> <anchor|marker> <key> <out>
  python - "$1" "$2" "$3" "$4" <<'PY'
import sys
path, mode, key, out = sys.argv[1:5]
fence = chr(96) * 3
lines = open(path, encoding="utf-8").read().split("\n")
blocks, cur, start = [], None, 0
for i, ln in enumerate(lines):
    if cur is None and ln == fence + "bash":
        cur, start = [], i
    elif cur is not None and ln == fence:
        blocks.append((start, cur))
        cur = None
    elif cur is not None:
        cur.append(ln)
if mode == "marker":
    hit = [b for (s, b) in blocks if ('echo "%s"' % key) in b]
else:
    at = [i for i, ln in enumerate(lines) if ln.startswith(key)]
    hit = [b for (s, b) in blocks if len(at) == 1 and s > at[0]][:1]
if len(hit) != 1:
    sys.exit("FAIL: %d bash blocks in %s match %r, expected exactly 1" % (len(hit), path, key))
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(hit[0]) + "\necho CP8H-WHOLE-RUN-SENTINEL\n")
PY
}
cp "$store" target/s8h-whole/store.before.sqlite3 || { echo "FAIL: cannot back up the store"; exit 1; }
for spec in "8g|runbook/phase-1/s8g-verify-and-restored-demands.md|marker|CP8G-END-OF-BLOCK reached" "5|runbook/phase-1/s5-workspace-ci-rk-types.md|anchor|**Checkpoint 5.** All of:" "7b|runbook/phase-1/s7b-kernel.md|anchor|### 7b.14 Checkpoint 7b" "7b-ii|runbook/phase-1/s7b-ii-gates.md|anchor|### 7b-ii.13 Checkpoint 7b-ii" "7d|runbook/phase-1/s7d-cli-and-migrate.md|marker|CP7D-END-OF-BLOCK reached"; do
  IFS='|' read -r name file mode key <<< "$spec"
  test -f "$file" || { echo "FAIL: $file is absent — Checkpoint $name cannot be re-run (Halt 8)"; exit 1; }
  extract "$file" "$mode" "$key" "target/s8h-whole/$name.sh" || { echo "FAIL: could not extract Checkpoint $name"; exit 1; }
  out=$(env -u REDKILN_LAUNCH_RECORD RK_CP_NESTED=1 CLAUDE_CONFIG_DIR="$trap_dir" bash "target/s8h-whole/$name.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $name run whole rc=$rc — its row stays provisional"; printf '%s\n' "$out" | tail -25; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8H-WHOLE-RUN-SENTINEL' || { echo "FAIL: Checkpoint $name did not reach its last line"; exit 1; }
done
left=$(find "$trap_dir" -mindepth 1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trap directory could not be read after the replays (rc=$rc)"; exit 1; }
[ -z "$left" ] || { echo "FAIL: something wrote into the trap CLAUDE_CONFIG_DIR during the replays — a real claude was spawned:"; printf '%s\n' "$left" | head -10; exit 1; }
fi

# ---- (14) NOTHING DISPATCHED, NO LAUNCH RECORD WRITTEN, THE WORKTREE PATH SET UNCHANGED (compared before any prune).
disp_after=$(python -c "$probe" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$disp_after" ] || { echo "FAIL: the post-run store probe did not run (rc=$rc)"; exit 1; }
[ "$disp_after" = "$disp_before" ] || { echo "FAIL: dispatch rows moved $disp_before -> $disp_after — this block, or a checkpoint (13) re-ran, DISPATCHED"; exit 1; }
nl_after=$(find "$launches" -type f 2>/dev/null | wc -l)
[ "$nl_after" -eq "$nl_before" ] || { echo "FAIL: a launch record was written — --check must write none"; exit 1; }
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

echo "CP8H-END-OF-BLOCK reached"
```

**Closing.** The stage closes in two commits: first the work and its checkpoint output, then the tracker.

- **Before the second commit, three gates are passed, and none can be decided by a machine.**
  - The operator's dated ledger line signs Record B **value by value** and **gate row by row** (AC-276). It also
    signs Record A's AC-133 judgement table **enum by enum** (AC-156).
  - Record B flips to `accepted` in the tracker commit (ADR-0092 cl.2.1): the frontmatter, the README index cell
    and the census move, and the body does not.
  - Record A stays `proposed`. `PROGRESS.md` §1 names it as awaiting `s8j`'s two probes.
- **The §8h.13 mutants' outcomes** go into the checkpoint output: twenty, each red, each restore verified.
- **Adversarial refuting rounds** run over the diff until a round moves no code and no guard.

**Findings for the lead, recorded rather than absorbed.** Items 1–9 are the draft's, each with its status after
D21-43/44/45. Items 10–15 are new in that revision, and items 16–19 are new in the D21-46 revision.

1. **Settled by D21-44.** The draft's blocker was *"the run-owned worktree lives under `.git`"*. D21-44 measured it:
   an edit inside `.git/redkiln/worktrees/…` was refused. The run worktree moves to `.redkiln/worktrees/<item>`,
   which is `s8d-ii`'s (D21-46 R1). Probe 1 re-measures the placement at `s8j` (§8h.11), and D21-45's in-worktree limit is named
   (§8h.5).
2. **`!` negation cannot carve out of an absolute rule**, as documented. So D21-42 21d's deny rules cannot be written
   as "deny all, except the worktree". §8h.6 writes them as the ancestor-chain complement, now over the new root.
   That is a construction choice, and Record A states it.
3. **Withdrawn.** S-BIN does not dispatch through `rk-adapter-claude`, and it launches through `wrap` (§8h.7,
   §8h.14).
4. **Assigned.** `s8`'s three `redkiln run` sites are `s8j`'s amendment (D21-43).
5. **Owned by `s8g`.** AC-133's registry is built there, and `s8g` carries `s5`'s dated correction. This stage only
   enters its enums.
6. **The register contradicts itself on the 226 h figure.** The scope note, *"the longest was answered at 226 h"*,
   sits against AC-039's *"belongs to stage→verdict … not to gate→verdict"*. The pack's `gate_deadline_secs` warrant
   quotes the first. Record B states both.
7. **Taken by D21-43 N17.** Q1–Q4 are no longer free residuals, and nothing is superseded by Q1.
8. **Brief facts that are false or imprecise, kept corrected:**
   - The margin ticks carry no `NO RECORD` marker.
   - `model_belongs_to_tier` is a `-`-segment match, not a prefix match.
   - `story`'s appetite is `points = 60`, not 40 [re-measured at `a6349b6`].
   - The packet's *"one paid dispatch each"* for the two probes cannot verify three model ids with one dispatch.
     N17 took Q4 as three.
9. **`DISABLE_AUTOUPDATER`, `DISABLE_UPDATES` and `--setting-sources` are undocumented on the docs site.** They are
   measured from the installed binary's strings. `blockReadsOutsideWorkingDirectories`, `--permission-prompts none`,
   the `!`-scope rule and the protected-path list are documented. D21-45 also quotes the protected-path list from
   the 2.1.281 code. Record A quotes each against a fresh fetch.
10. **`s8i`'s and `s8j`'s dispatch scans refuse Checkpoint 8h as written** [measured at `a6349b6`]. `s8i` (11)'s
    `DISPATCH_RE` matches both `--features live` and `--ignored`, and `s8j` (10) matches `--features live`, on any
    non-comment line. `s8i`'s revision, which is in progress, may already have narrowed its own scan. Checkpoint
    8h carries `--features live` twice:
    - (7), a `-- --list --ignored` listing that runs nothing;
    - (12), `cargo clippy` over the live-gated code.

    Neither dispatches. Respelling them to dodge the pattern would satisfy a guard by its text, so they stay as they
    are. Each scan has to admit a `--list` line and a `cargo clippy` line, with a synthetic control. That is `s8i`'s
    and `s8j`'s work, and it is not resolved here.
11. **Checkpoint 8a (11) and Checkpoint 7f become provisional at this stage.** The first stops on *"a card carries
    permission_mode"*. The second's `redkiln run` refuses at the launch check. Its force-removing cleanup line is gone
    since `s8d-ii` §8d-ii.7, which also moved its run arm into a throwaway clone. §8h.14 gives the route, and running
    it is `s8j`'s.
12. **`StepError::Overage { reason }` already exists.** Its retry comment reads *"`Overage` is refused BEFORE spawn
    (AC-221, ADR-0010)"*, and it maps to `Exit::Overage`. N17 fixed only the exit. Before building a parallel
    kernel `EnvelopeError`, the implementer has to decide whether the envelope refusal is this variant with a
    `reason` naming the counter. If it is, Record B says so. If it is not, Record B says why not.
13. **A Claude Code session exports two names outside `CLAUDE_CODE_*`.** They are `CLAUDE_PID` and `CLAUDE_EFFORT`
    [measured by name]. 7c's guard and the committed denylist miss both, and D21-44's probe script stripped both.
    The launcher and (0b) now refuse them. Whether 7c's guard should too is `s8j`'s question, since `s8j` re-runs
    Checkpoint 7c.
14. **Minted gate kinds need rows before they can be reached.** `s8f`'s `landing` gate and N11's drift gate are
    covered here (§8h.10). Any gate kind a later stage mints, including `s8j`'s pack-declared human-root questions,
    brings its own `[[gate_expiry]]` row, or the load lint refuses the pack. *Updated 2026-09-24 for D21-46:* §8h.10
    now lists all six `s8` classes by row, and says why `s8j`'s three land at `s8j`.
15. **The drafted floor chain rests on sibling drafts that are being revised too** (§8h.0 item 8). Only Checkpoint
    8g's recorded count may replace `BASE_8G`.
16. **Two sibling files quote the `wrap` text this revision replaced.** `s8i` §8i.5 quotes *"Step 6 records and does
    not compare, because `TESTED_CLI_VERSION` binds the adapter, and neither caller is the adapter"* [read at
    `c5d19d7`]. `s8j` §8j.6 step 2 quotes *"`wrap` is the entry for a child that does not dispatch through
    `rk-adapter-claude`"*. Neither sentence is in this file any more. Both callers' substance still holds: `s8i` needs
    no pin comparison, and `s8j` needs 7c's own pin check. Each quotation is re-pointed at §8h.7's restated guarantee
    by its own stage's revision. The row-6 cell *"`wrap`: record only, and never compare to the pin (below)"* and the
    sentence *"`wrap` is not a way around the pin for `run` or `probe`"* are unchanged, so those quotations still
    resolve. *Settled in the consistency pass 2026-09-24:* both quotations are re-pointed at §8h.7's restated text.
17. **`s8j` §8j.3 step 5 and §8j.2's attach still spell a bare `redkiln answer`.** After this stage, a bare binary
    `answer` refuses without a launch record (§8h.7), so each goes through `tools/live-run.sh answer`. That edit is
    `s8j`'s. `s8j`'s dispatch screen `SCREEN='tools/live-run\.sh[[:space:]]+(run|probe)|…'` does not match the
    `answer` mode, and `answer` spawns no `claude`. Whether the screen should see it is `s8j`'s question. *Settled:*
    `s8j`'s revised screen matches the launcher's `answer` and `wrap` modes, and §8j.2 and §8j.3 route every answer
    through `tools/live-run.sh answer`.
18. **The launch check on `answer` reaches every binary-level `answer` test** (§8h.0 item 11). That includes the
    spawn helper in `rk-cli`'s `tests/it/answer.rs` and `s8e`'s `lane_e2e` binary arm. Each is given a fixture
    launch record in its shared helper, not an exemption. If a checkpoint between `s8e` and `s8g` spawns `answer` from
    a bash line rather than through a test, it refuses at its next whole run. No such line exists at `c5d19d7`
    [measured: a scan of every fenced `bash` block in `runbook/phase-1/*.md` for `redkiln`, `$RK` or `$BIN` followed
    by `answer` on a non-comment line finds none].
19. **`s8j`'s three gate classes have no fixed spelling anywhere yet.** `s8j` §8j.2 leaves *"the gate-kind name the
    step-4 gate binds under"* to its record, and §8j.3 leaves *"the gate cause"*. `GateReason` has one variant,
    `UnknownFacet`, at `c5d19d7` [measured: `crates/rk-kernel/src/select/step2.rs`]. §8h.10 lists them by what they
    are and leaves their rows to `s8j`, so that the misspelt-class lint cannot refuse a row drafted before its code.

---
