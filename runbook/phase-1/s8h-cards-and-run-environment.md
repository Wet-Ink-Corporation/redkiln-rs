# Step 8h — Cards, models and the run environment (Phase 1, part 8, sub-stage h)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8h` | Phase 1 | Checkpoint 8g | Checkpoint 8h | `s8g` | `s8i` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

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

### 8h.0 Stage open: re-measure, then four questions the ledger leaves open

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
   `AcceptanceCriterion` (D21-42 10a). A type that exists is **registered, never redeclared**.
4. **The count source for §8h.9.** Does the kernel's pre-dispatch `dispatch` row name the item? If it does not, the
   count reads the `item` member that `s8f` (D21-42 12b) put on `dispatch_completed`. Either way the source is the
   store, never a per-process counter.
5. **Whether `budget_extension` has gained a producer.** At `ca8f387` `wall_clock_expiry_mark`'s write has *"no call
   site anywhere in `src/`"* (`budget.rs`). If an earlier sub-stage wired one, that class joins §8h.10's table.
6. `claude --version`, and the size and bytes of `RUNBOOK-instrument.txt`. Halt 2 is recorded, not worked around,
   and D21-42 16 moves the pin once, at `s8j`.

**Four questions are owed to the operator before Red starts.** Nothing in D21-40…D21-42 settles them, and each one
changes a test's expected value. Each carries a recommendation. None is taken by this file.

- **Q1 — the typed class of the four new refusals.** ADR-0112 closes `StepError` at eleven and *"Forbids a twelfth
  variant without a successor to this record"*. ADR-0095 decision 2 fixes `NotIsolated`'s payload as
  `{ surface: tools \| mcp_servers \| env, diff_digest }`. Recommended:
  - **(i)** an init-reported model unequal to the requested id → `NotAdopted { expected_agent, observed }`. This is
    the existing RT-27 reading, which already refuses a wrong-tier model as `NotAdopted`, so nothing is superseded.
  - **(ii)** `apiKeySource` absent or not `"none"` → `NotIsolated { surface: Env, .. }`. `IsolationSurface::Env`
    has no producer at `ca8f387`. This is a stated reading, and nothing is superseded.
  - **(iii)** an init-reported `permissionMode` unequal to the card's, and **(iv)** a non-empty
    `permission_denials` → two new `IsolationSurface` members, `PermissionMode` and `PermissionDenied`. This
    **supersedes ADR-0095 decision 2's payload cell in part**. The variant count stays eleven, so ADR-0112's bar
    is not crossed, and the retry class is `NotIsolated`'s `Never`.

  The alternative for (iii) and (iv) is a twelfth variant. That supersedes ADR-0112 in part, and it adds an event
  arm and an `Exit` row (`exit::ac438_step_error_mapping_is_exhaustive` moves).
- **Q2 — the unit `appetite.tokens` counts.** Nothing charges `Charge::Tokens` today. Recommended: the sum of the
  four counters `Usage` carries verbatim (`input_tokens`, `output_tokens`, `cache_creation_input_tokens`,
  `cache_read_input_tokens`). It is the only sum every row can compute without a price model. Excluding
  `cache_read_input_tokens` is the alternative, since F14 left it open whether cache reads draw the window. §8h.9
  gives the first value under both units.
- **Q3 — how an envelope refusal leaves the CLI.** Recommended: a kernel `EnvelopeError` variant per counter, leaving
  through the existing `Exit::Overage` (20). Its doc already reads *"A budget or billing state that does not improve
  by being repeated"*, so the exit snapshot does not move. The alternative is a new `Exit` row, which moves
  `exit::ac438_exit_table_snapshot` and needs a dated amendment to the register's exit table.
- **Q4 — probe 2's dispatch count.** §8h.11 authors probe 2 as **three** dispatches: one per distinct
  (permission mode, tool set, model id) triple that probe 1 does not cover. A single dispatch would verify one model
  id. The other two would then be first observed on a real run, where the init check refuses rather than proceeds,
  which is the safe direction.

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
retires the segment match; the schema route (§8h.4) and the AC-133 judgement classification per enum, **signed
by the operator per enum** (AC-133's own limit: *"that classification is the human gate AC-156"*); the three guards and Q1's classes (§8h.5); the
setting-sources and confinement settings (§8h.6); the launcher and the launch record as the only entry to a real run
(§8h.7); the freeze (§8h.8). **It supersedes in part** ADR-0126 decision 1 — its payload table gains
`observed_model` (a member added to a `## Decision` table is a supersession, ADR-0092 r1) — and, if Q1 is taken as
recommended, ADR-0095 decision 2's `surface` cell. It supersedes **nothing** of ADR-0084 (the schemas are Rust types
in `EXPORTED`, which is what its golden-snapshot clause binds: *"Every type that crosses the closed-output boundary has
its generated `schemars` output snapshotted with `insta`"*) and nothing of ADR-0156/0162, whose bars on
`.redkiln/schemas/` are stage-scoped and are honoured anyway (no such directory is created). It records that
**landing is not decided here** (D21-41 (b), `s8e`/`s8f`) and that the probes' spend is D21-40 decision 2's.

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
`.current_dir(&prepared.worktree)`). Probe 1 reads it back from `system/init`'s `cwd`.

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

**AC-133 has no instrument in code, and this stage builds it.** At `ca8f387` no judgement-enum registry exists.
`registry.rs`'s header cites AC-133's completeness half about `EXPORTED`, which is a different registry. AC-133 wants
*"the enum appears in the judgement-enum registry with an explicit judgement-bearing yes/no AND every enum marked yes
has a sibling free-text `rationale` field"*. So `rk-types` gains the registry: every enum reachable from an
`EXPORTED` type, a yes/no per enum, and a completeness test in both directions. Every "yes" is checked for its
`rationale` sibling. The yes/no **values** are the operator's (AC-156), signed in Record A. The completeness tests
check that entries exist, never that the values are right.

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
   The classes are Q1's.
3. **`permission_denials` → a typed failure.** The result frame carries `permission_denials: [{ tool_name,
   tool_use_id, tool_input }]` [measured: the binary's result schema; documented in `headless` as *"the final result
   message lists them in `permission_denials`"*]. A non-empty list fails the dispatch with Q1's class, whatever the
   exit code. It **outranks a valid `structured_output`**: a step whose child was denied an out-of-worktree edit has
   not done what it reported. It is placed in `classify.rs`'s order after the platform arms and before the
   `structured_output` validation. An empty list is not a failure.

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

🛑 **A documented risk that can make the implementer unable to write at all.** `permission-modes.md` lists `.git` as a
**protected directory**, whose writes *"still prompt"* under `acceptEdits`. Its only worktree exception is
*"`.claude`, except for `.claude/worktrees`"* [documented, via the docs-reading pass]. The run-owned worktree is
`common_dir.join("redkiln").join("worktrees").join(item)`, which on this box is
`D:/repos/redkiln-rs/.git/redkiln/worktrees/<item>` [measured: `run.rs`, and `git worktree list`]. If the protection
keys on a `.git` ancestor, every implementer edit prompts, and under `--permission-prompts none` *"anything that
would prompt is denied automatically"* [measured: the 2.1.281 help text]. **Probe 1's inside-edit condition decides
this.** If it fails, the work HALTS (§8h.11). Moving the worktree root is a design/record decision (design §5's
`reconcile_started` note derives `<common-dir>/redkiln/worktrees/<item>/`) and is not taken here.

### 8h.7 The launcher — the only entry to a real run (D21-42 17a, 17b)

**`tools/live-run.sh`**, committed, LF (ADR-0164), POSIX, for Git Bash. Its modes are `run <item> [--lane fix|story]`,
`probe <test>` (a module-qualified `live::` name) and `--check`. `--check` performs every refusal and every capture,
spawns nothing and writes no launch record. In order:

| # | step | refusal / capture |
| --- | --- | --- |
| 1 | `uname -s` is `MINGW*`/`MSYS*` | refuse otherwise |
| 2 | no `CLAUDECODE`, no `CLAUDE_CODE_*` name in its own environment | refuse naming the variable (s7c §7c.15's reason) |
| 3 | no denylisted name, **case-insensitively** | the list is **read from `crates/rk-adapter-claude/src/env_denylist.rs`'s two `pub const` arrays by anchored extraction**, never copied. That file is *"the ONE list AC-217, AC-422, AC-488 and AC-509 read"*. The extraction asserts both anchors and a non-empty list (golden constraint 7). `Config::new` refuses the same names again, as the second line |
| 4 | `CLAUDE_CONFIG_DIR` is **set, not inherited**, to `$(cygpath -w "$HOME/.claude-personal")` (D21-42 17b) | refuse if the directory or its `.credentials.json` is absent. That is an existence test only, and no content is read |
| 5 | `DISABLE_AUTOUPDATER=1` is exported (§8h.8) | — |
| 6 | the resolved `claude` path; `claude --version`; the sha256 of the resolved file | refuse unless the version equals `TESTED_CLI_VERSION`, read by 7c's anchored form, **and** `RUNBOOK-instrument.txt`, naming Halt 2 |
| 7 | `run`: `cargo build --workspace` rc 0 before `./target/debug/redkiln` is run (Halt 1). `probe`: `cargo test -p rk-adapter-claude --features live --test it -- --ignored --nocapture <test> --exact`, then `test <test> ... ok` **and** `1 passed` are asserted, because a filter matching nothing is a green | the child's rc is the launcher's |
| 8 | the **launch record** at `<git-common-dir>/redkiln/launches/<UTC>-<mode>.txt`: mode and argv, claude path, version and sha256 before; `CLAUDE_CONFIG_DIR`; `DISABLE_AUTOUPDATER`; the pin; HEAD; then version and sha256 after, and the rc | exported to the child as `REDKILN_LAUNCH_RECORD=<path>` |
| 9 | after the child exits: version and sha256 again | **a difference fails the launch non-zero**, naming Halt 2. The records then span a version boundary |

The launch record is **not** the kernel's run record. Writing the version into the `run_id` preimage would move every
`run_id`, which ADR-0130 decision 1's byte-stability forbids. The version already rides every `dispatch_completed`
row as `instrument_version` and `cli_version`. The launch record adds the binary's sha256 and the freeze state, which
no row carries.

**"The only entry" is made mechanical.** Every verb whose composition root has chosen the **real**
`ClaudeHarness` (`run`, and `answer` where it continues the lane) refuses at entry with `Exit::ConfigError` unless
the captured `Env` carries `REDKILN_LAUNCH_RECORD` naming an existing file. The refusal comes **before
`reconcile_started` is appended and before any worktree operation**. The name is read through `env.rs`, the one
read (ADR-0083). A fake-harness path is not subject to it. Binary-level tests that drive the real harness at the
fixture child supply a fixture launch record, so the check is exercised on every such test and bypassed by none. The
three `live::` probe tests assert the same variable as their first statement. **This is also what re-scopes
Checkpoint 7f's `redkiln run`.** `s8a` (11) records that *"From s8h on that is false, and this line must be re-scoped
before it runs"*. Once the cards bind, a bare `redkiln run` refuses before any spawn. It is a guard against accident,
not a security boundary: the variable can be forged, and that is stated.

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
once the sum has reached `appetite.tokens`. The refusal is typed per Q3, happens before any spawn, and writes no
tier-1 row. An absent `appetite.dispatches` is refused by pack validation, never defaulted (`require_*`'s doctrine,
`select/pack.rs`). A dispatch with no `usage` still counts against the dispatch cap. That is the reason the cap exists
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
- Under Q2's alternative unit (cache reads excluded), M = 62,725, E ≈ 1.07 M, and the appetite is ≈ 2.13 M, rounded
  up to 2,200,000.

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

Both packs carry all 45 questions, so each stays independently complete. That is **51 rows** in `bug-standard` and
**53** in `story`. `budget_extension` joins the table only if §8h.0 item 5 finds a producer.

**The draft is `fail` for every row.** `auto-decide` is refused by name in `run.rs` because it has no verdict carrier
(RT-57), so it cannot be drafted. `escalate` is admissible only as a signed exception with a warrant. None is
drafted. The operator signs row by row (AC-276). A signed exception amends the table here and Checkpoint 8h's
exception list, by dated line.

**ADR-0146's load-time lint** (*"the honest options are to ship the rows or to make an unmatched step-gate class a
load-time refusal"*). **Both are built.** At pack load, `run` and `answer` refuse, before any dispatch, when a
reachable class of the enabled pack has no `[[gate_expiry]]` row. This file reads *"a gate class with no process
entry"* as no row in the enabled process pack, and Record B states that reading. They also refuse a row whose class
is neither a catalog question, nor `unknown_facet:<registered facet>`, nor `escalation`, because a misspelt class
leaves the real class uncovered.

**The doctor row, `RK-D-CARD-GRANT`.** It refuses any card tool outside the (g) union
`{Read, Glob, Grep, Edit, Write, StructuredOutput}`, naming `Bash`, `PowerShell`, `Web*` and `mcp__*` in its message.
It also refuses any shipped card mode outside `{default, acceptEdits}`. The installed CLI offers a `PowerShell` tool
[measured: `CLAUDE_CODE_USE_POWERSHELL_TOOL` in the binary], which is a command channel of the same class as `Bash`.
A union allowlist refuses it without naming a list that goes stale. `ac415_every_rule_id_has_a_rejecting_fixture`
requires the row's fixture.

### 8h.11 The two live probes — authored here, run at `s8j` after the fixture proofs

Both are `#[ignore]` tests in `crates/rk-adapter-claude/tests/it/live.rs` behind `--features live`, run only as
`tools/live-run.sh probe live::<name>` on the operator's go-ahead, after §8h.8's pin move. They use the **shipped**
cards bound through the **shipped** role rows, because the thing measured is what ships. Each prints its values one
per line, as `live_dispatch_probe` does. The raw `system/init` and result frames are retained by O-9's route, a
commit in `D:/repos/redkiln-corpus` named by SHA, which never moves `CORPUS_PIN`.

**Probe 1 — `live::live_confinement_and_denial_probe`** (one Sonnet dispatch; AC-215's integration half; D21-42 21d).

- **Setup.** A temp git repository stands in as the run-owned worktree: its path goes on the spec, and it is the
  `cwd`. It holds `inside.txt` and nothing else. **Two sentinels sit outside it**, each with a fresh random token and
  a recorded sha256: `outside-write.txt` in a sibling temp directory, and `outside-read.txt` under the worktree's
  parent.
- **The dispatch.** The bound `implementer` card (`acceptEdits`, the six tools) with the §8h.6 settings. The user
  message instructs, in order:
  1. write a given token into `inside.txt`;
  2. write into `outside-write.txt` by absolute path;
  3. read `outside-read.txt` by absolute path and put its content in the structured output;
  4. report each outcome.

  The timeout is 300 s, s7c's bound and Halt 5's alternative.
- **What it prints and records.** `INIT_PERMISSION_MODE`, `INIT_CWD`, `INIT_TOOLS`, `INIT_MODEL`,
  `INIT_API_KEY_SOURCE`, `DENIALS` (count and `tool_name` list), `OUTCOME` (the typed result), `WALL_MS`,
  `INSIDE_SHA256`, and `OUTSIDE_WRITE_SHA256` before and after.
- **Pass, all of:**
  1. The child terminated under the bound. A kill by the timer is a never-waits failure.
  2. `inside.txt` holds the token, so an inside edit works under `acceptEdits` and §8h.6's `.git` risk did not bite.
  3. `outside-write.txt`'s sha256 is unchanged.
  4. The read token appears **nowhere** in the retained stdout, including the structured output.
  5. `permission_denials` is non-empty and names the outside write **and** the outside read.
  6. The adapter's outcome is Q1's typed failure, not `Ok`.
  7. The init reports `acceptEdits`, the six tools, the requested id, `apiKeySource` `"none"`, and `cwd` equal to the
     worktree.
- **HALT — the work stops and a containment finding is reported, never re-tuned (D21-42 21d; Halt 7).** Any of:
  1. the outside file changed;
  2. the read token leaked;
  3. a denial occurred but was not surfaced as `permission_denials`;
  4. the child hung;
  5. the inside edit was denied.
- **Inconclusive, which is not a pass.** No denial occurred and both sentinels are untouched, meaning the model did
  not attempt the outside actions. The (g) record is not accepted, and a re-run is a new operator go-ahead.

**Probe 2 — `live::live_init_equality_probe`** (Q4: three dispatches, one per triple probe 1 does not cover).
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
module-qualified. New modules (`judgement`, `cards`, `schemas`) are added to their crate's `tests/it/main.rs`.

**`rk-adapter-claude` (16 new).** Red reasons are in brackets.
- `card::permission_mode_outside_the_cli_closed_set_is_refused` [`"acceptEdit"` binds today]
- `card::every_cli_permission_mode_is_admitted_by_the_closed_set` [the control against an over-narrow set]
- `isolation::permission_mode_reported_unequal_to_the_card_is_refused`
- `isolation::permission_mode_absent_from_init_is_refused`
- `isolation::model_reported_unequal_to_the_requested_id_is_refused` [requested `claude-sonnet-5`, reported
  `claude-sonnet-4-6`: the segment match adopts it today]
- `isolation::model_equal_to_the_requested_id_proceeds`
- `isolation::api_key_source_other_than_none_is_refused`
- `isolation::api_key_source_absent_is_refused`
- `classify::permission_denials_non_empty_is_a_typed_failure`
- `classify::permission_denials_empty_is_not_a_failure`
- `classify::permission_denials_outrank_a_valid_structured_output`
- `command::setting_sources_is_explicit_and_empty`
- `command::child_settings_carry_the_worktree_confinement` [the deny set is the ancestor-chain complement, no `!`,
  plus the block setting]
- `binder::the_full_model_id_reaches_argv_and_the_card_digest` [two ids for one row key move the digest]
- `binder::a_tier_absent_from_the_models_table_is_refused`
- `dispatch::observed_model_is_recorded_on_every_path_that_read_init` [`None` before init, never the requested id]

**`rk-types` (4 new).**
- `judgement::every_enum_on_an_exported_type_is_registered`
- `judgement::a_judgement_enum_without_a_rationale_sibling_fails`
- `judgement::the_registry_names_no_enum_that_is_not_exported`
- `dispatch_record::observed_model_is_a_typed_member_absent_before_init`

**`rk-kernel` (6 new).**
- `envelope::dispatch_cap_is_declared_never_defaulted`
- `envelope::the_dispatch_at_the_cap_proceeds`
- `envelope::the_dispatch_past_the_cap_is_refused_with_its_class`
- `envelope::the_dispatch_count_survives_a_second_process` [two runtimes over one store; a one-hop fixture cannot
  judge a multi-hop guard]
- `envelope::token_appetite_past_the_appetite_refuses_the_next_dispatch`
- `envelope::a_dispatch_with_no_usage_still_counts_against_the_cap`

**`rk-cli` (14 new).**
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

### 8h.14 What this stage may not do, and what it hands on

- It may not dispatch, run a `live::` test, move the pin, or edit the operator's user settings.
- It may not create `.redkiln/schemas/` or copy the denylist.
- It may not loosen an equality to make a probe pass.
- **To `s8j` (for its s8 instrument amendments):** `s8-exit-criterion.md` invokes `./target/debug/redkiln run` at
  three sites. After this stage each of those refuses without a launch record, so they must go through
  `tools/live-run.sh run`. This amendment is not in the plan's list of s8 amendments.
- **To `s8i`:** S-BIN's dispatches are real spend. Record B must be accepted first (19c). The launcher is the
  entry. **The pin is still 2.1.278 while the CLI reads 2.1.281, so an adapter dispatch at `s8i` refuses at the init
  line.** The plan puts the pin move at `s8j`, which is after `s8i`'s spend. This is reported (finding 3), not
  resolved here.

### 8h.15 Checkpoint 8h

- The eight cards are §8h.2's table. The closed-set lint, the init equality (mode, model, auth) and the denial
  mapping each fail when absent, each through its own named test.
- `[models]` pins the three ids, the role rows resolve through it, and `observed_model` is recorded.
- Every one of the 24 lane-reached kinds is an `EXPORTED` Rust type, read by name. The AC-133 registry exists and is
  complete in both directions.
- `--setting-sources ""` and the confinement settings are on every argv.
- The launcher refuses what it must and captures what it must. A real-harness run without a launch record refuses
  before any spawn.
- The per-item cap and appetite refuse across processes.
- No `NO RECORD` marker remains. Both packs carry fail-closed gate rows for every reachable class. An unmatched class
  refuses at load, and `RK-D-CARD-GRANT` refuses a grant outside the union.
- Record A is `proposed`. Record B is written, and the operator's signatures are a stage-close gate (AC-276:
  *"routed to a human gate; not machine-decidable"*).
- The two probes compile and are ignored. **No dispatch happened.** Checkpoint 8g passes whole, and Checkpoint 7c's
  shell guards pass.

**Run it from a Git Bash that Claude Code did not spawn** (7c's guard, repeated here). Set `REC_A`, `REC_B`,
`A_SUPERSEDES` (the ids Record A supersedes, space-separated; at minimum `0126`), `TOKENS_APPETITE` (Record B's signed
value) and whatever Checkpoint 8g's block itself requires.

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

# ---- INPUTS. The record ids are allocated at writing; this block takes them rather than guessing them.
test -n "${REC_A:-}" && test -n "${REC_B:-}" || { echo "FAIL: set REC_A and REC_B to the two records' four-digit ids"; exit 1; }
test -n "${A_SUPERSEDES:-}" || { echo "FAIL: set A_SUPERSEDES (at minimum 0126 — observed_model moves ADR-0126 d1's payload table)"; exit 1; }
test -n "${TOKENS_APPETITE:-}" || { echo "FAIL: set TOKENS_APPETITE to Record B's signed appetite.tokens"; exit 1; }
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
sel rk-adapter-claude 22 card::permission_mode_outside_the_cli_closed_set_is_refused card::every_cli_permission_mode_is_admitted_by_the_closed_set isolation::permission_mode_reported_unequal_to_the_card_is_refused isolation::permission_mode_absent_from_init_is_refused isolation::model_reported_unequal_to_the_requested_id_is_refused isolation::model_equal_to_the_requested_id_proceeds isolation::api_key_source_other_than_none_is_refused isolation::api_key_source_absent_is_refused classify::permission_denials_non_empty_is_a_typed_failure classify::permission_denials_empty_is_not_a_failure classify::permission_denials_outrank_a_valid_structured_output command::setting_sources_is_explicit_and_empty command::child_settings_carry_the_worktree_confinement binder::the_full_model_id_reaches_argv_and_the_card_digest binder::a_tier_absent_from_the_models_table_is_refused dispatch::observed_model_is_recorded_on_every_path_that_read_init command::golden_argv_per_profile command::ac214_permission_mode_explicit command::permission_prompts_none_on_every_argv isolation::ac204_adoption_equal_subset_superset_wrong_agent isolation::ac202_isolation_from_system_init_not_argv config::tested_cli_version_is_the_pin_as_a_config_value
sel rk-types 8 judgement::every_enum_on_an_exported_type_is_registered judgement::a_judgement_enum_without_a_rationale_sibling_fails judgement::the_registry_names_no_enum_that_is_not_exported dispatch_record::observed_model_is_a_typed_member_absent_before_init schema_snapshot::schema_snapshot schema_no_oneof::no_oneof_at_any_node schema_caps::union_and_optional_caps dispatch_record::ac360_dispatch_record_carries_the_usage_fields
sel rk-kernel 7 envelope::dispatch_cap_is_declared_never_defaulted envelope::the_dispatch_at_the_cap_proceeds envelope::the_dispatch_past_the_cap_is_refused_with_its_class envelope::the_dispatch_count_survives_a_second_process envelope::token_appetite_past_the_appetite_refuses_the_next_dispatch envelope::a_dispatch_with_no_usage_still_counts_against_the_cap envelope::ac029_n_is_declared_never_defaulted
sel rk-cli 18 cards::shipped_cards_are_the_least_privilege_table cards::shipped_role_rows_resolve_through_the_models_table cards::the_models_table_pins_the_three_full_ids cards::shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class cards::shipped_rt57_scalars_carry_no_no_record_marker schemas::every_agent_output_kind_the_two_lanes_reach_is_exported schemas::bind_reads_the_schema_from_exported_by_name schemas::a_kind_absent_from_exported_refuses_at_bind doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair run::a_reachable_gate_class_without_an_expiry_row_refuses_at_load run::a_gate_expiry_row_naming_no_reachable_class_is_refused run::a_real_harness_run_without_a_launch_record_refuses_before_any_spawn run::a_real_harness_run_with_a_launch_record_passes_the_launch_check doctor::ac415_every_rule_id_has_a_rejecting_fixture exit::ac438_exit_table_snapshot exit::ac438_step_error_mapping_is_exhaustive surface::ac400_help_snapshot_every_verb

# ---- (7) THE TWO PROBES EXIST, COMPILE UNDER `live`, AND ARE IGNORED — LISTED, NEVER RUN (`--list` runs nothing).
out=$(cargo test -p rk-adapter-claude --features live --test it -- --list --ignored 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the live feature does not compile (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
for t in live::live_confinement_and_denial_probe live::live_init_equality_probe live::live_dispatch_probe; do
  printf '%s\n' "$out" | grep -qxF "$t: test" || { echo "FAIL: $t is not an IGNORED test under --features live — it would run, or it does not exist"; exit 1; }
done

# ---- (8) THE SHIPPED DATA, read by parsers (json / tomllib), never grepped: cards, [models], role rows, gate rows,
# ---- the envelope keys, and no NO RECORD marker. The expected gate-class set is DERIVED from the catalog and the
# ---- packs' own `when` rows and compared to §8h.10's counts as a PREMISE. EXCEPTIONS is empty until the operator
# ---- signs one; a signed exception amends this list by dated line.
out=$(python - "$TOKENS_APPETITE" "$REC_B" <<'PY'
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
EXCEPTIONS = {}
words = {"and", "or", "not", "in", "high", "med", "low", "mandatory", "recommended", "optional", "forbidden"}
expect_axes = {"bug-standard": 5, "story": 7}
expect_rows = {"bug-standard": 51, "story": 53}
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
    want = set(questions).union({"unknown_facet:" + a for a in axes}, {"escalation"})
    rows = d.get("gate_expiry", [])
    got = [r.get("class") for r in rows]
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
for v in CLAUDECODE=1 CLAUDE_CODE_ENTRYPOINT=cli ANTHROPIC_API_KEY=x anthropic_base_url=x GH_TOKEN=x CLAUDE_CODE_USE_BEDROCK=1; do
  out=$("${clean[@]}" "$v" bash "$L" --check 2>&1); rc=$?
  [ "$rc" -ne 0 ] || { echo "FAIL: the launcher accepted $v"; exit 1; }
  printf '%s\n' "$out" | grep -qiF "${v%%=*}" || { echo "FAIL: the refusal of $v does not name the variable"; printf '%s\n' "$out"; exit 1; }
done
out=$(env -i "HOME=$HOME" "USERPROFILE=$USERPROFILE" "SYSTEMROOT=$SYSTEMROOT" "TEMP=$TEMP" "TMP=$TMP" "PATH=$bad_fake:/usr/bin:/mingw64/bin" bash "$L" --check 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the launcher accepted a CLI version unequal to the pin"; exit 1; }
printf '%s\n' "$out" | grep -qF 'Halt 2' || { echo "FAIL: the version refusal does not name Halt 2"; printf '%s\n' "$out"; exit 1; }
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
total=$(printf '%s\n' "$out" | awk '/^test result:/{for(i=1;i<NF;i++) if($(i+1) ~ /^passed;?$/) s+=$i} END{print s+0}')
# 740 measured at ca8f387 plus this stage's 40 new. Raise it at stage open to Checkpoint 8g's recorded count + 40.
[ "$total" -ge 780 ] || { echo "FAIL: the whole suite passed $total tests, below the floor of 780"; exit 1; }
printf '%s\n' "$out" | grep -E '^test result:' | grep -vqE ' 0 failed;' && { echo "FAIL: a result line reports failures"; exit 1; }

# ---- (13) CHECKPOINT 8g, WHOLE, FROM ITS OWN BYTES (s8a (11)'s helper shape): the ONE bash block carrying the anchor,
# ---- plus a sentinel, so "rc 0 and the sentinel" means it ran to its end. The store is copied first.
mkdir -p target/s8h-whole || { echo "FAIL: cannot create target/s8h-whole"; exit 1; }
extract() {
  python - "$1" "$2" "$3" <<'PY'
import re, sys
fence = chr(96) * 3
pat = re.compile(re.escape(fence) + "bash\n(.*?)" + re.escape(fence), re.S)
text = open(sys.argv[1], encoding="utf-8").read()
found = [m.group(1) for m in pat.finditer(text) if sys.argv[2] in m.group(1)]
if len(found) != 1:
    sys.exit("FAIL: %d bash blocks in %s carry the anchor, expected exactly 1" % (len(found), sys.argv[1]))
open(sys.argv[3], "w", encoding="utf-8", newline="\n").write(found[0] + "\necho CP8H-WHOLE-RUN-SENTINEL\n")
PY
}
cp "$store" target/s8h-whole/store.before.sqlite3 || { echo "FAIL: cannot back up the store"; exit 1; }
S8G=runbook/phase-1/s8g-verify-and-restored-demands.md
test -f "$S8G" || { echo "FAIL: $S8G is absent — the entry gate cannot be re-run (Halt 8)"; exit 1; }
extract "$S8G" 'CP8G-END-OF-BLOCK reached' target/s8h-whole/8g.sh || { echo "FAIL: could not extract Checkpoint 8g"; exit 1; }
out=$(bash target/s8h-whole/8g.sh 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8g run whole rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP8H-WHOLE-RUN-SENTINEL' || { echo "FAIL: Checkpoint 8g did not reach its last line"; exit 1; }

# ---- (14) NOTHING DISPATCHED, NO LAUNCH RECORD WRITTEN, THE WORKTREE PATH SET UNCHANGED (compared before any prune).
disp_after=$(python -c "$probe" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$disp_after" ] || { echo "FAIL: the post-run store probe did not run (rc=$rc)"; exit 1; }
[ "$disp_after" = "$disp_before" ] || { echo "FAIL: dispatch rows moved $disp_before -> $disp_after — this block, or Checkpoint 8g inside it, DISPATCHED"; exit 1; }
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
- **The §8h.13 mutants' outcomes** go into the checkpoint output: fifteen, each red, each restore verified.
- **Adversarial refuting rounds** run over the diff until a round moves no code and no guard.

**Findings for the lead, recorded rather than absorbed:**

1. 🛑 **The run-owned worktree lives under `.git`, which the CLI documents as a protected directory.** Under
   `acceptEdits` and `--permission-prompts none`, this may mean the implementer can never write (§8h.6). Probe 1
   decides it, and a failure HALTs. The remedy moves a design-derived path (`<common-dir>/redkiln/worktrees/<item>`).
   Consider whether the operator wants probe 1 earlier than `s8j`, since `s8e`–`s8g`'s landing path is built on that
   location.
2. **`!` negation cannot carve out of an absolute rule**, as documented. So D21-42 21d's deny rules cannot be written
   as "deny all, except the worktree". §8h.6 writes them as the ancestor-chain complement. That is a construction
   choice, and Record A states it.
3. **`s8i` spends before the pin moves.** The plan moves the pin at `s8j`, but `s8i`'s S-BIN dispatches are real
   spend. At 2.1.281 against a 2.1.278 pin the adapter refuses at init under `MismatchPolicy::Refuse`. Either S-BIN
   does not go through the adapter, or the pin move and freeze must come before `s8i`.
4. **`s8-exit-criterion.md` invokes `./target/debug/redkiln run` at three sites.** Each needs routing through the
   launcher. This is not in the plan's list of s8 amendments.
5. **AC-133's judgement-enum registry does not exist in code**, although `s5` claims AC-133 in its acceptance set.
   This stage builds it (§8h.4). The `s5` claim wants a dated correction.
6. **The register contradicts itself on the 226 h figure.** The scope note, *"the longest was answered at 226 h"*,
   sits against AC-039's *"belongs to stage→verdict … not to gate→verdict"*. The pack's `gate_deadline_secs` warrant
   quotes the first. Record B states both.
7. **Four questions (§8h.0 Q1–Q4) are free residuals.** Q1 in particular may supersede ADR-0095 decision 2 in part,
   or ADR-0112 in part.
8. **Brief facts that are false or imprecise:**
   - The margin ticks carry no `NO RECORD` marker.
   - `model_belongs_to_tier` is a `-`-segment match, not a prefix match.
   - The packet's *"one paid dispatch each"* for the two probes cannot verify three model ids with one dispatch
     (Q4).
9. **`DISABLE_AUTOUPDATER`, `DISABLE_UPDATES` and `--setting-sources` are undocumented on the docs site.** They are
   measured from the installed binary's strings. `blockReadsOutsideWorkingDirectories`, `--permission-prompts none`,
   the `!`-scope rule and the protected-path list are documented. Record A quotes each against a fresh fetch.

---
